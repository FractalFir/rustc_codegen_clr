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
pub fn fn0(mut _1: i8,mut _2: char) -> usize {
mir! {
type RET = usize;
let _3: [u8; 4];
let _4: isize;
let _5: (&'static [isize; 5],);
let _6: *mut [u8; 4];
let _7: ((i8, *const f64, i16, i32),);
let _8: &'static isize;
let _9: [i32; 7];
let _10: *mut (f32,);
let _11: [i64; 5];
let _12: ([u32; 1], Adt31, *const *mut Adt20, *const (f32,));
let _13: i8;
let _14: *mut *mut Adt31;
let _15: u8;
let _16: char;
let _17: Adt31;
let _18: i64;
let _19: isize;
let _20: Adt76;
let _21: f64;
let _22: bool;
let _23: [i128; 3];
let _24: f32;
let _25: Adt28;
let _26: f32;
let _27: u64;
let _28: isize;
let _29: i16;
let _30: u32;
let _31: f32;
let _32: ();
let _33: ();
{
_1 = 95_i8 + 76_i8;
RET = 0_usize - 1_usize;
RET = 2124346391_u32 as usize;
_1 = (-81_i8) ^ 72_i8;
RET = 0_usize;
_3[RET] = 129_u8 & 213_u8;
RET = !3_usize;
RET = (-91972737_i32) as usize;
_3 = [47_u8,201_u8,83_u8,179_u8];
_2 = '\u{472fa}';
_1 = RET as i8;
_3 = [177_u8,112_u8,109_u8,136_u8];
_4 = 20_isize ^ (-78_isize);
RET = !6_usize;
RET = 4424278647092598002_u64 as usize;
RET = 9980026832994743546_usize ^ 6601496408129994584_usize;
RET = 2_usize << _4;
Call(_2 = fn1(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [47_u8,45_u8,202_u8,53_u8];
_3 = [83_u8,71_u8,247_u8,227_u8];
_2 = '\u{8851b}';
_6 = core::ptr::addr_of_mut!(_3);
(*_6) = [241_u8,218_u8,15_u8,177_u8];
Goto(bb2)
}
bb2 = {
_4 = -9223372036854775807_isize;
_2 = '\u{adfd9}';
RET = 1_usize;
_4 = true as isize;
_6 = core::ptr::addr_of_mut!(_3);
(*_6) = [81_u8,76_u8,249_u8,128_u8];
(*_6) = [2_u8,138_u8,128_u8,82_u8];
_6 = core::ptr::addr_of_mut!((*_6));
_4 = 9223372036854775807_isize;
(*_6) = [169_u8,113_u8,156_u8,217_u8];
_1 = _2 as i8;
_4 = (-48_isize);
_4 = (-9223372036854775808_isize);
(*_6)[RET] = 2897833765_u32 as u8;
_2 = '\u{10a67d}';
_3 = [150_u8,41_u8,67_u8,239_u8];
(*_6) = [62_u8,62_u8,110_u8,200_u8];
_7.0.2 = 10179_i16;
_3[RET] = !179_u8;
_7.0.0 = _1 | _1;
(*_6)[RET] = 1197267146_i32 as u8;
_3 = [178_u8,172_u8,227_u8,168_u8];
_7.0.2 = 9603_i16;
RET = 9796572103543545943_usize * 1370991805147892934_usize;
_8 = &_4;
_3 = [243_u8,12_u8,250_u8,217_u8];
_2 = '\u{2121a}';
RET = 5_usize >> _1;
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb3 = {
_3 = [47_u8,45_u8,202_u8,53_u8];
_3 = [83_u8,71_u8,247_u8,227_u8];
_2 = '\u{8851b}';
_6 = core::ptr::addr_of_mut!(_3);
(*_6) = [241_u8,218_u8,15_u8,177_u8];
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
_1 = _7.0.0 << RET;
_7.0.3 = !662752272_i32;
_4 = (-165246209283763833815303671472047712274_i128) as isize;
_7.0.2 = 183_u8 as i16;
_7.0.0 = 228_u8 as i8;
RET = 4514104899178855808_usize * 10264860415585049170_usize;
RET = 105368883907776738888204423232235554392_i128 as usize;
RET = 1_usize;
_2 = '\u{10303a}';
_1 = !_7.0.0;
RET = 0_usize;
_8 = &_4;
RET = 10674283687072574211_usize >> _1;
_8 = &(*_8);
_7.0.3 = 1669369209_i32 * (-1132778549_i32);
_7.0.0 = _1 * _1;
(*_6) = [168_u8,200_u8,194_u8,180_u8];
(*_6) = [219_u8,89_u8,15_u8,177_u8];
_11 = [7756051637230786521_i64,(-8253390241450525893_i64),6322258498920482634_i64,(-621228383424774967_i64),(-788123889030048357_i64)];
(*_6) = [41_u8,201_u8,91_u8,219_u8];
_6 = core::ptr::addr_of_mut!((*_6));
_12.0 = [2705831326_u32];
Goto(bb9)
}
bb9 = {
_2 = '\u{48198}';
_8 = &(*_8);
RET = !2_usize;
_3 = [152_u8,36_u8,183_u8,211_u8];
_8 = &(*_8);
Goto(bb10)
}
bb10 = {
_8 = &(*_8);
RET = 6_usize >> _1;
(*_6) = [245_u8,175_u8,75_u8,237_u8];
(*_6) = [162_u8,3_u8,141_u8,112_u8];
_13 = _7.0.0 ^ _7.0.0;
_2 = '\u{33ac9}';
_8 = &_4;
RET = 16012805033952829465_usize - 7810266852264295958_usize;
_7.0.2 = (-27445_i16) >> _4;
_11 = [(-1644486413501989842_i64),1217065173482461556_i64,943308060110333785_i64,4107436473191448118_i64,468133088505139469_i64];
_8 = &(*_8);
_11 = [(-7823305864316998674_i64),8308635808440851872_i64,2740640888003378355_i64,(-7437204349324950468_i64),5958418107774151983_i64];
_7.0.0 = _1;
_8 = &(*_8);
_7.0.3 = !1011234386_i32;
_3 = [238_u8,85_u8,167_u8,236_u8];
_13 = _2 as i8;
_3 = [223_u8,127_u8,10_u8,35_u8];
_16 = _2;
Goto(bb11)
}
bb11 = {
_2 = _16;
RET = 2_usize - 13203549541068379590_usize;
_9 = [_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3];
_3 = [42_u8,134_u8,165_u8,20_u8];
(*_6) = [164_u8,110_u8,129_u8,46_u8];
_22 = _7.0.2 > _7.0.2;
_9 = [_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3];
_16 = _2;
_8 = &(*_8);
_9 = [_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3];
_12.0 = [713267494_u32];
_8 = &_4;
_18 = 1954075713_u32 as i64;
_8 = &_19;
_7.0.1 = core::ptr::addr_of!(_21);
_23 = [(-118861701641187005633665964270683625823_i128),(-167184128800869872123265262926185615877_i128),(-131051993875969768017485551478177223320_i128)];
_7.0.1 = core::ptr::addr_of!(_21);
_13 = 4267173973_u32 as i8;
_22 = !true;
_22 = true;
_3 = [34_u8,119_u8,183_u8,251_u8];
_21 = _7.0.2 as f64;
(*_6) = [150_u8,35_u8,95_u8,114_u8];
Goto(bb12)
}
bb12 = {
_8 = &(*_8);
_19 = _4;
_1 = _7.0.0 | _13;
_3 = [168_u8,251_u8,202_u8,158_u8];
_7.0.2 = 28053_i16;
(*_6) = [21_u8,235_u8,77_u8,220_u8];
_7.0.3 = (-1070655069_i32) >> _19;
_8 = &_4;
_9 = [_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3,_7.0.3];
_7.0.0 = _22 as i8;
_16 = _2;
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb13)
}
bb13 = {
(*_6) = [174_u8,106_u8,19_u8,123_u8];
_13 = 4476009414889745127_u64 as i8;
_19 = _22 as isize;
_6 = core::ptr::addr_of_mut!((*_6));
_13 = 7247464519932197645_u64 as i8;
_15 = !108_u8;
_4 = 32509_u16 as isize;
_21 = _19 as f64;
_7.0.2 = 9891_i16;
_27 = _1 as u64;
Goto(bb14)
}
bb14 = {
_11 = [_18,_18,_18,_18,_18];
RET = !4_usize;
_28 = -_19;
_8 = &_19;
_18 = 3464012807469974607_i64;
(*_6) = [_15,_15,_15,_15];
_3 = [_15,_15,_15,_15];
RET = 3_usize;
_4 = _22 as isize;
_2 = _16;
_6 = core::ptr::addr_of_mut!(_3);
_7.0.1 = core::ptr::addr_of!(_21);
_11 = [_18,_18,_18,_18,_18];
(*_6) = [_15,_15,_15,_15];
_24 = RET as f32;
_1 = _13 ^ _7.0.0;
_13 = _7.0.0;
_3[RET] = !_15;
_9[RET] = _7.0.3;
_8 = &_19;
_23 = [(-4104756131477615612591771530434479143_i128),(-38429469613221476931209271601837944923_i128),(-163564251774368698263829959141660497218_i128)];
_19 = _28;
_9[RET] = _7.0.3;
_4 = -_28;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(0_usize, 9_usize, Move(_9), 13_usize, Move(_13), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(0_usize, 3_usize, Move(_3), 28_usize, Move(_28), 22_usize, Move(_22), 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1() -> char {
mir! {
type RET = char;
let _1: [u8; 4];
let _2: (*mut Adt31,);
let _3: *const (f32,);
let _4: *const i16;
let _5: ([i16; 2],);
let _6: Adt20;
let _7: *mut (f32,);
let _8: u128;
let _9: usize;
let _10: &'static i16;
let _11: u32;
let _12: u16;
let _13: Adt20;
let _14: &'static [isize; 5];
let _15: u16;
let _16: [i32; 7];
let _17: u16;
let _18: [i32; 5];
let _19: i16;
let _20: f64;
let _21: bool;
let _22: f64;
let _23: isize;
let _24: Adt76;
let _25: [u16; 1];
let _26: f64;
let _27: [i32; 5];
let _28: u16;
let _29: char;
let _30: char;
let _31: ([i16; 2],);
let _32: *const f64;
let _33: *mut [u16; 3];
let _34: u64;
let _35: ();
let _36: ();
{
RET = '\u{d94f4}';
RET = '\u{63968}';
RET = '\u{983c0}';
_1 = [216_u8,97_u8,108_u8,205_u8];
_1 = [150_u8,54_u8,58_u8,192_u8];
RET = '\u{ce0a3}';
RET = '\u{fac44}';
_5.0 = [(-28110_i16),30188_i16];
_1 = [58_u8,30_u8,144_u8,90_u8];
RET = '\u{94ca}';
RET = '\u{7ba}';
RET = '\u{e1e8f}';
_5.0 = [1108_i16,27823_i16];
_5.0 = [29898_i16,22227_i16];
_6 = Adt20::Variant1 { fld0: 197_u8,fld1: RET };
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
place!(Field::<u8>(Variant(_6, 1), 0)) = Field::<char>(Variant(_6, 1), 1) as u8;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
RET = Field::<char>(Variant(_6, 1), 1);
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
_8 = 269742219101670285420288321454277166674_u128;
_1 = [Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0)];
_8 = 2197248238_u32 as u128;
_8 = 246799207788191965210360950553369895699_u128;
Call(_6 = fn2(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<f64>(Variant(_6, 0), 0)) = 24885_i16 as f64;
SetDiscriminant(_6, 0);
place!(Field::<usize>(Variant(_6, 0), 1)) = 4_usize * 2_usize;
_1 = [156_u8,19_u8,0_u8,174_u8];
place!(Field::<f64>(Variant(_6, 0), 0)) = Field::<usize>(Variant(_6, 0), 1) as f64;
place!(Field::<u16>(Variant(_6, 0), 3)) = 33322_u16 * 30207_u16;
RET = '\u{10acb9}';
place!(Field::<f64>(Variant(_6, 0), 0)) = 722544978117512574_u64 as f64;
place!(Field::<i128>(Variant(_6, 0), 2)) = false as i128;
RET = '\u{59f54}';
_5.0 = [(-8224_i16),(-25991_i16)];
_1 = [247_u8,37_u8,123_u8,102_u8];
_8 = !339304701386722237229222020062369443396_u128;
_1 = [178_u8,108_u8,29_u8,15_u8];
RET = '\u{15e84}';
_8 = 159_u8 as u128;
_6 = Adt20::Variant1 { fld0: 144_u8,fld1: RET };
_8 = true as u128;
_9 = 7_usize;
_9 = 9223372036854775807_isize as usize;
_6 = Adt20::Variant1 { fld0: 120_u8,fld1: RET };
RET = Field::<char>(Variant(_6, 1), 1);
_1 = [158_u8,163_u8,102_u8,138_u8];
RET = Field::<char>(Variant(_6, 1), 1);
place!(Field::<u8>(Variant(_6, 1), 0)) = 20_u8;
_6 = Adt20::Variant1 { fld0: 147_u8,fld1: RET };
_6 = Adt20::Variant1 { fld0: 181_u8,fld1: RET };
_8 = 39133463824230830451637159271314401284_u128;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
Goto(bb2)
}
bb2 = {
_5.0 = [(-29649_i16),(-24625_i16)];
_1 = [134_u8,58_u8,124_u8,237_u8];
_8 = 127086484984682807130273683574909489016_u128 & 226161039110755185237653527829988945295_u128;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
RET = Field::<char>(Variant(_6, 1), 1);
RET = Field::<char>(Variant(_6, 1), 1);
_9 = !7_usize;
RET = Field::<char>(Variant(_6, 1), 1);
Goto(bb3)
}
bb3 = {
_8 = 8466607419180624222756658510423122559_u128 >> _9;
_11 = 2808374383_u32 & 2006882357_u32;
_8 = 99470143838126380894428458174701669974_u128;
RET = Field::<char>(Variant(_6, 1), 1);
RET = Field::<char>(Variant(_6, 1), 1);
_6 = Adt20::Variant1 { fld0: 38_u8,fld1: RET };
_9 = !5_usize;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
RET = Field::<char>(Variant(_6, 1), 1);
_1 = [54_u8,13_u8,234_u8,189_u8];
place!(Field::<u8>(Variant(_6, 1), 0)) = !241_u8;
RET = Field::<char>(Variant(_6, 1), 1);
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
place!(Field::<u8>(Variant(_6, 1), 0)) = !26_u8;
_11 = 1357300970_u32 & 1521944705_u32;
_12 = !17772_u16;
_5.0 = [(-32690_i16),20781_i16];
RET = Field::<char>(Variant(_6, 1), 1);
Goto(bb4)
}
bb4 = {
_9 = !1_usize;
_13 = Adt20::Variant1 { fld0: Field::<u8>(Variant(_6, 1), 0),fld1: Field::<char>(Variant(_6, 1), 1) };
_1 = [Field::<u8>(Variant(_13, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_13, 1), 0),Field::<u8>(Variant(_6, 1), 0)];
_6 = _13;
_5.0 = [19803_i16,(-2213_i16)];
RET = Field::<char>(Variant(_6, 1), 1);
_1 = [Field::<u8>(Variant(_13, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_13, 1), 0),Field::<u8>(Variant(_13, 1), 0)];
_9 = 6_usize;
place!(Field::<char>(Variant(_13, 1), 1)) = RET;
SetDiscriminant(_13, 1);
place!(Field::<char>(Variant(_13, 1), 1)) = RET;
_13 = _6;
RET = Field::<char>(Variant(_13, 1), 1);
_8 = !201006864045466642454111901988379074908_u128;
SetDiscriminant(_6, 1);
_15 = _12;
_5.0 = [(-10881_i16),(-3415_i16)];
place!(Field::<u8>(Variant(_6, 1), 0)) = !Field::<u8>(Variant(_13, 1), 0);
place!(Field::<u8>(Variant(_6, 1), 0)) = Field::<u8>(Variant(_13, 1), 0) & Field::<u8>(Variant(_13, 1), 0);
_16[_9] = 697274185_i32;
_6 = _13;
match _16[_9] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
697274185 => bb10,
_ => bb9
}
}
bb5 = {
_8 = 8466607419180624222756658510423122559_u128 >> _9;
_11 = 2808374383_u32 & 2006882357_u32;
_8 = 99470143838126380894428458174701669974_u128;
RET = Field::<char>(Variant(_6, 1), 1);
RET = Field::<char>(Variant(_6, 1), 1);
_6 = Adt20::Variant1 { fld0: 38_u8,fld1: RET };
_9 = !5_usize;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
RET = Field::<char>(Variant(_6, 1), 1);
_1 = [54_u8,13_u8,234_u8,189_u8];
place!(Field::<u8>(Variant(_6, 1), 0)) = !241_u8;
RET = Field::<char>(Variant(_6, 1), 1);
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
place!(Field::<u8>(Variant(_6, 1), 0)) = !26_u8;
_11 = 1357300970_u32 & 1521944705_u32;
_12 = !17772_u16;
_5.0 = [(-32690_i16),20781_i16];
RET = Field::<char>(Variant(_6, 1), 1);
Goto(bb4)
}
bb6 = {
_5.0 = [(-29649_i16),(-24625_i16)];
_1 = [134_u8,58_u8,124_u8,237_u8];
_8 = 127086484984682807130273683574909489016_u128 & 226161039110755185237653527829988945295_u128;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
RET = Field::<char>(Variant(_6, 1), 1);
RET = Field::<char>(Variant(_6, 1), 1);
_9 = !7_usize;
RET = Field::<char>(Variant(_6, 1), 1);
Goto(bb3)
}
bb7 = {
place!(Field::<f64>(Variant(_6, 0), 0)) = 24885_i16 as f64;
SetDiscriminant(_6, 0);
place!(Field::<usize>(Variant(_6, 0), 1)) = 4_usize * 2_usize;
_1 = [156_u8,19_u8,0_u8,174_u8];
place!(Field::<f64>(Variant(_6, 0), 0)) = Field::<usize>(Variant(_6, 0), 1) as f64;
place!(Field::<u16>(Variant(_6, 0), 3)) = 33322_u16 * 30207_u16;
RET = '\u{10acb9}';
place!(Field::<f64>(Variant(_6, 0), 0)) = 722544978117512574_u64 as f64;
place!(Field::<i128>(Variant(_6, 0), 2)) = false as i128;
RET = '\u{59f54}';
_5.0 = [(-8224_i16),(-25991_i16)];
_1 = [247_u8,37_u8,123_u8,102_u8];
_8 = !339304701386722237229222020062369443396_u128;
_1 = [178_u8,108_u8,29_u8,15_u8];
RET = '\u{15e84}';
_8 = 159_u8 as u128;
_6 = Adt20::Variant1 { fld0: 144_u8,fld1: RET };
_8 = true as u128;
_9 = 7_usize;
_9 = 9223372036854775807_isize as usize;
_6 = Adt20::Variant1 { fld0: 120_u8,fld1: RET };
RET = Field::<char>(Variant(_6, 1), 1);
_1 = [158_u8,163_u8,102_u8,138_u8];
RET = Field::<char>(Variant(_6, 1), 1);
place!(Field::<u8>(Variant(_6, 1), 0)) = 20_u8;
_6 = Adt20::Variant1 { fld0: 147_u8,fld1: RET };
_6 = Adt20::Variant1 { fld0: 181_u8,fld1: RET };
_8 = 39133463824230830451637159271314401284_u128;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_17 = _15 >> _16[_9];
_16 = [(-668934918_i32),152595351_i32,51531491_i32,243660649_i32,(-1560165332_i32),(-314174403_i32),(-1484286737_i32)];
_9 = 17457456464917448954_usize - 3_usize;
_15 = !_17;
_11 = 1508402181_u32 + 1913170016_u32;
_8 = 219112165865415180799795581536159173214_u128;
_12 = !_15;
_4 = core::ptr::addr_of!(_19);
_18 = [(-2104894472_i32),380614185_i32,(-1322841982_i32),1905457622_i32,(-1376433358_i32)];
_15 = 736269876_i32 as u16;
_10 = &(*_4);
(*_4) = _8 as i16;
_9 = !3284447243905809060_usize;
place!(Field::<u8>(Variant(_13, 1), 0)) = Field::<u8>(Variant(_6, 1), 0) + Field::<u8>(Variant(_6, 1), 0);
_12 = _17 - _17;
_4 = core::ptr::addr_of!((*_4));
_15 = _12;
_6 = _13;
SetDiscriminant(_13, 1);
_19 = 12852_i16 ^ (-29919_i16);
_17 = _15;
_10 = &(*_4);
place!(Field::<char>(Variant(_13, 1), 1)) = RET;
Goto(bb11)
}
bb11 = {
_22 = _11 as f64;
RET = Field::<char>(Variant(_6, 1), 1);
_8 = 326711756169475004123590255811830936526_u128;
place!(Field::<char>(Variant(_6, 1), 1)) = RET;
Goto(bb12)
}
bb12 = {
_11 = 1610998705_u32 ^ 1203669767_u32;
RET = Field::<char>(Variant(_6, 1), 1);
_25 = [_17];
_26 = _22 * _22;
(*_4) = (-21202_i16) >> _12;
_17 = _11 as u16;
(*_4) = (-76_isize) as i16;
_9 = !7_usize;
_1 = [Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0),Field::<u8>(Variant(_6, 1), 0)];
place!(Field::<u8>(Variant(_13, 1), 0)) = Field::<u8>(Variant(_6, 1), 0) << _11;
_27 = [256881471_i32,1237934931_i32,1648594402_i32,1300257009_i32,552559398_i32];
(*_4) = (-559_i16) & 18825_i16;
SetDiscriminant(_6, 0);
(*_4) = _15 as i16;
_13 = Adt20::Variant0 { fld0: _22,fld1: _9,fld2: (-131758096009502370793764334823304867113_i128),fld3: _12 };
_19 = !(-1055_i16);
place!(Field::<i128>(Variant(_13, 0), 2)) = 68213414774795264991718349787253095411_i128;
_6 = Adt20::Variant0 { fld0: Field::<f64>(Variant(_13, 0), 0),fld1: _9,fld2: Field::<i128>(Variant(_13, 0), 2),fld3: _15 };
SetDiscriminant(_6, 0);
_19 = -8534_i16;
SetDiscriminant(_13, 1);
place!(Field::<usize>(Variant(_6, 0), 1)) = !_9;
match _8 {
0 => bb11,
326711756169475004123590255811830936526 => bb13,
_ => bb7
}
}
bb13 = {
_17 = _12 >> (*_4);
_9 = _8 as usize;
_9 = !Field::<usize>(Variant(_6, 0), 1);
_28 = !_12;
_1 = [34_u8,70_u8,12_u8,9_u8];
place!(Field::<i128>(Variant(_6, 0), 2)) = 103635741102563861973982276410520929617_i128;
_30 = RET;
place!(Field::<u16>(Variant(_6, 0), 3)) = !_12;
_23 = 102_isize - 9223372036854775807_isize;
place!(Field::<i128>(Variant(_6, 0), 2)) = !(-41425968276062054096876709509401730643_i128);
(*_4) = 22400_i16 >> _11;
_31 = (_5.0,);
_21 = false;
place!(Field::<usize>(Variant(_6, 0), 1)) = _9 * _9;
(*_4) = -13022_i16;
_20 = _22;
_27 = _18;
_17 = !_28;
_4 = core::ptr::addr_of!((*_4));
_26 = _20;
_1 = [200_u8,48_u8,108_u8,106_u8];
RET = _30;
place!(Field::<u16>(Variant(_6, 0), 3)) = !_12;
_29 = RET;
_5.0 = [(*_4),(*_4)];
_4 = core::ptr::addr_of!(_19);
Goto(bb14)
}
bb14 = {
_21 = Field::<u16>(Variant(_6, 0), 3) >= _28;
_9 = 118_u8 as usize;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(1_usize, 30_usize, Move(_30), 12_usize, Move(_12), 25_usize, Move(_25), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(1_usize, 5_usize, Move(_5), 16_usize, Move(_16), 29_usize, Move(_29), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(1_usize, 8_usize, Move(_8), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2() -> Adt20 {
mir! {
type RET = Adt20;
let _1: f64;
let _2: [i8; 3];
let _3: f32;
let _4: Adt76;
let _5: &'static usize;
let _6: ([i16; 2],);
let _7: (u32,);
let _8: usize;
let _9: char;
let _10: bool;
let _11: Adt20;
let _12: char;
let _13: isize;
let _14: u64;
let _15: Adt20;
let _16: &'static [isize; 5];
let _17: f64;
let _18: *const u32;
let _19: f32;
let _20: *mut Adt31;
let _21: &'static *mut Adt20;
let _22: ();
let _23: ();
{
_1 = (-62032050999504532023739257946821394221_i128) as f64;
Goto(bb1)
}
bb1 = {
_1 = 3249463502099416861_i64 as f64;
Call(RET = fn3(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = Adt20::Variant1 { fld0: 160_u8,fld1: '\u{1618d}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = !184_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 4541_u16 as u8;
RET = Adt20::Variant0 { fld0: _1,fld1: 12467802406886221236_usize,fld2: (-45517772744080548602871410141730463568_i128),fld3: 20439_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = -157019350321498853028533172004398067541_i128;
place!(Field::<u16>(Variant(RET, 0), 3)) = 26970_u16 - 5325_u16;
RET = Adt20::Variant1 { fld0: 14_u8,fld1: '\u{f7a4f}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 6_usize,fld2: (-44367581439581959717999715964699511178_i128),fld3: 59232_u16 };
place!(Field::<usize>(Variant(RET, 0), 1)) = 0_usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = !22009_u16;
place!(Field::<usize>(Variant(RET, 0), 1)) = Field::<u16>(Variant(RET, 0), 3) as usize;
RET = Adt20::Variant1 { fld0: 62_u8,fld1: '\u{2914e}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 1924016562565113651_usize,fld2: 132800922541585731458612003765578778461_i128,fld3: 62587_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = !(-18181730066854808654955346098122380123_i128);
RET = Adt20::Variant0 { fld0: _1,fld1: 7_usize,fld2: 52858095133334971583974107863734640700_i128,fld3: 37323_u16 };
place!(Field::<usize>(Variant(RET, 0), 1)) = 5257858406100916765_usize >> 148_u8;
place!(Field::<u16>(Variant(RET, 0), 3)) = !25929_u16;
Goto(bb3)
}
bb3 = {
place!(Field::<u16>(Variant(RET, 0), 3)) = Field::<usize>(Variant(RET, 0), 1) as u16;
RET = Adt20::Variant1 { fld0: 200_u8,fld1: '\u{1099e9}' };
_2 = [(-122_i8),(-105_i8),(-97_i8)];
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{f0224}';
RET = Adt20::Variant0 { fld0: _1,fld1: 5713684702446624657_usize,fld2: 37004957993214121613699793525387112357_i128,fld3: 17389_u16 };
_1 = Field::<f64>(Variant(RET, 0), 0) - Field::<f64>(Variant(RET, 0), 0);
place!(Field::<usize>(Variant(RET, 0), 1)) = 13007358303905294066_usize ^ 350729923927718477_usize;
place!(Field::<usize>(Variant(RET, 0), 1)) = !17487174587171961837_usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = 22848_u16 * 33262_u16;
_3 = Field::<u16>(Variant(RET, 0), 3) as f32;
_1 = Field::<f64>(Variant(RET, 0), 0) * Field::<f64>(Variant(RET, 0), 0);
RET = Adt20::Variant1 { fld0: 106_u8,fld1: '\u{2054a}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = 154_u8 >> (-4673174709765593198_i64);
RET = Adt20::Variant1 { fld0: 193_u8,fld1: '\u{10f21a}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = '\u{266bc}' as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{83d60}';
_1 = (-116816757577182085718825780863714408236_i128) as f64;
RET = Adt20::Variant1 { fld0: 97_u8,fld1: '\u{5b95}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 2676516908153824641_usize,fld2: 85708200302574797495032214491547885401_i128,fld3: 56900_u16 };
_6.0 = [(-18738_i16),11067_i16];
place!(Field::<i128>(Variant(RET, 0), 2)) = 39668437790635804026760939352399990635_i128 * 112905235852771393900806559568884125642_i128;
place!(Field::<f64>(Variant(RET, 0), 0)) = 1219122514544431164_i64 as f64;
RET = Adt20::Variant0 { fld0: _1,fld1: 5_usize,fld2: (-376258940015865783001875877691297218_i128),fld3: 20833_u16 };
Goto(bb4)
}
bb4 = {
_8 = 1468983851_u32 as usize;
_5 = &_8;
place!(Field::<usize>(Variant(RET, 0), 1)) = 332376633382291322484231991386724902994_u128 as usize;
_1 = 4012491103_u32 as f64;
_7 = (1954190041_u32,);
place!(Field::<u16>(Variant(RET, 0), 3)) = 6030279307686584182_u64 as u16;
place!(Field::<u16>(Variant(RET, 0), 3)) = 57216_u16 | 35275_u16;
place!(Field::<i128>(Variant(RET, 0), 2)) = (-40750079569539884288214690660954704026_i128) ^ 45381143847804176107555876424127434761_i128;
_10 = true | true;
_5 = &(*_5);
_8 = !Field::<usize>(Variant(RET, 0), 1);
Goto(bb5)
}
bb5 = {
place!(Field::<u16>(Variant(RET, 0), 3)) = 31025_u16 | 56850_u16;
_5 = &place!(Field::<usize>(Variant(RET, 0), 1));
match _7.0 {
1954190041 => bb6,
_ => bb1
}
}
bb6 = {
_7.0 = 1268751275_u32;
place!(Field::<usize>(Variant(RET, 0), 1)) = 9223372036854775807_isize as usize;
SetDiscriminant(RET, 1);
match _7.0 {
0 => bb1,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1268751275 => bb13,
_ => bb12
}
}
bb7 = {
place!(Field::<u16>(Variant(RET, 0), 3)) = 31025_u16 | 56850_u16;
_5 = &place!(Field::<usize>(Variant(RET, 0), 1));
match _7.0 {
1954190041 => bb6,
_ => bb1
}
}
bb8 = {
_8 = 1468983851_u32 as usize;
_5 = &_8;
place!(Field::<usize>(Variant(RET, 0), 1)) = 332376633382291322484231991386724902994_u128 as usize;
_1 = 4012491103_u32 as f64;
_7 = (1954190041_u32,);
place!(Field::<u16>(Variant(RET, 0), 3)) = 6030279307686584182_u64 as u16;
place!(Field::<u16>(Variant(RET, 0), 3)) = 57216_u16 | 35275_u16;
place!(Field::<i128>(Variant(RET, 0), 2)) = (-40750079569539884288214690660954704026_i128) ^ 45381143847804176107555876424127434761_i128;
_10 = true | true;
_5 = &(*_5);
_8 = !Field::<usize>(Variant(RET, 0), 1);
Goto(bb5)
}
bb9 = {
place!(Field::<u16>(Variant(RET, 0), 3)) = Field::<usize>(Variant(RET, 0), 1) as u16;
RET = Adt20::Variant1 { fld0: 200_u8,fld1: '\u{1099e9}' };
_2 = [(-122_i8),(-105_i8),(-97_i8)];
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{f0224}';
RET = Adt20::Variant0 { fld0: _1,fld1: 5713684702446624657_usize,fld2: 37004957993214121613699793525387112357_i128,fld3: 17389_u16 };
_1 = Field::<f64>(Variant(RET, 0), 0) - Field::<f64>(Variant(RET, 0), 0);
place!(Field::<usize>(Variant(RET, 0), 1)) = 13007358303905294066_usize ^ 350729923927718477_usize;
place!(Field::<usize>(Variant(RET, 0), 1)) = !17487174587171961837_usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = 22848_u16 * 33262_u16;
_3 = Field::<u16>(Variant(RET, 0), 3) as f32;
_1 = Field::<f64>(Variant(RET, 0), 0) * Field::<f64>(Variant(RET, 0), 0);
RET = Adt20::Variant1 { fld0: 106_u8,fld1: '\u{2054a}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = 154_u8 >> (-4673174709765593198_i64);
RET = Adt20::Variant1 { fld0: 193_u8,fld1: '\u{10f21a}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = '\u{266bc}' as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{83d60}';
_1 = (-116816757577182085718825780863714408236_i128) as f64;
RET = Adt20::Variant1 { fld0: 97_u8,fld1: '\u{5b95}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 2676516908153824641_usize,fld2: 85708200302574797495032214491547885401_i128,fld3: 56900_u16 };
_6.0 = [(-18738_i16),11067_i16];
place!(Field::<i128>(Variant(RET, 0), 2)) = 39668437790635804026760939352399990635_i128 * 112905235852771393900806559568884125642_i128;
place!(Field::<f64>(Variant(RET, 0), 0)) = 1219122514544431164_i64 as f64;
RET = Adt20::Variant0 { fld0: _1,fld1: 5_usize,fld2: (-376258940015865783001875877691297218_i128),fld3: 20833_u16 };
Goto(bb4)
}
bb10 = {
RET = Adt20::Variant1 { fld0: 160_u8,fld1: '\u{1618d}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = !184_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 4541_u16 as u8;
RET = Adt20::Variant0 { fld0: _1,fld1: 12467802406886221236_usize,fld2: (-45517772744080548602871410141730463568_i128),fld3: 20439_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = -157019350321498853028533172004398067541_i128;
place!(Field::<u16>(Variant(RET, 0), 3)) = 26970_u16 - 5325_u16;
RET = Adt20::Variant1 { fld0: 14_u8,fld1: '\u{f7a4f}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 6_usize,fld2: (-44367581439581959717999715964699511178_i128),fld3: 59232_u16 };
place!(Field::<usize>(Variant(RET, 0), 1)) = 0_usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = !22009_u16;
place!(Field::<usize>(Variant(RET, 0), 1)) = Field::<u16>(Variant(RET, 0), 3) as usize;
RET = Adt20::Variant1 { fld0: 62_u8,fld1: '\u{2914e}' };
RET = Adt20::Variant0 { fld0: _1,fld1: 1924016562565113651_usize,fld2: 132800922541585731458612003765578778461_i128,fld3: 62587_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = !(-18181730066854808654955346098122380123_i128);
RET = Adt20::Variant0 { fld0: _1,fld1: 7_usize,fld2: 52858095133334971583974107863734640700_i128,fld3: 37323_u16 };
place!(Field::<usize>(Variant(RET, 0), 1)) = 5257858406100916765_usize >> 148_u8;
place!(Field::<u16>(Variant(RET, 0), 3)) = !25929_u16;
Goto(bb3)
}
bb11 = {
_1 = 3249463502099416861_i64 as f64;
Call(RET = fn3(), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
RET = Adt20::Variant1 { fld0: 59_u8,fld1: '\u{a90a1}' };
_3 = 119836219092104510503041496024155114138_i128 as f32;
_8 = 10927552709383387678_usize & 7_usize;
_3 = 1763_u16 as f32;
_8 = 3_usize;
RET = Adt20::Variant1 { fld0: 179_u8,fld1: '\u{97536}' };
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{18908}';
RET = Adt20::Variant1 { fld0: 147_u8,fld1: '\u{42bef}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = 87_u8;
Goto(bb14)
}
bb14 = {
place!(Field::<u8>(Variant(RET, 1), 0)) = '\u{b8896}' as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{cfa0a}';
_14 = 9605175992866117572_u64 + 12167038615878155830_u64;
_5 = &_8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 7644_i16 as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{f8dd6}';
_3 = _7.0 as f32;
RET = Adt20::Variant0 { fld0: _1,fld1: _8,fld2: 76351165538774639901677710542586100990_i128,fld3: 33044_u16 };
_14 = !12176513397307278905_u64;
RET = Adt20::Variant0 { fld0: _1,fld1: (*_5),fld2: (-135774028539257023227714443405009729739_i128),fld3: 25268_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = (-31705005558350567257041365351923131114_i128) >> (*_5);
place!(Field::<usize>(Variant(RET, 0), 1)) = _14 as usize;
_15 = Adt20::Variant0 { fld0: _1,fld1: _8,fld2: Field::<i128>(Variant(RET, 0), 2),fld3: 58257_u16 };
RET = Adt20::Variant0 { fld0: Field::<f64>(Variant(_15, 0), 0),fld1: (*_5),fld2: Field::<i128>(Variant(_15, 0), 2),fld3: 96_u16 };
_11 = Adt20::Variant1 { fld0: 177_u8,fld1: '\u{45e0f}' };
RET = Adt20::Variant0 { fld0: _1,fld1: (*_5),fld2: Field::<i128>(Variant(_15, 0), 2),fld3: 13844_u16 };
RET = Adt20::Variant0 { fld0: Field::<f64>(Variant(_15, 0), 0),fld1: Field::<usize>(Variant(_15, 0), 1),fld2: Field::<i128>(Variant(_15, 0), 2),fld3: 61799_u16 };
_17 = _1;
_1 = Field::<f64>(Variant(RET, 0), 0);
place!(Field::<f64>(Variant(_15, 0), 0)) = -_1;
_7 = (4089129725_u32,);
_12 = '\u{af8d}';
match _8 {
0 => bb6,
1 => bb10,
2 => bb9,
3 => bb17,
_ => bb16
}
}
bb15 = {
_1 = 3249463502099416861_i64 as f64;
Call(RET = fn3(), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
place!(Field::<i128>(Variant(RET, 0), 2)) = _3 as i128;
RET = Adt20::Variant1 { fld0: 138_u8,fld1: _12 };
RET = Adt20::Variant0 { fld0: Field::<f64>(Variant(_15, 0), 0),fld1: (*_5),fld2: Field::<i128>(Variant(_15, 0), 2),fld3: 32353_u16 };
_8 = 9223372036854775807_isize as usize;
_18 = core::ptr::addr_of!(_7.0);
place!(Field::<f64>(Variant(RET, 0), 0)) = Field::<f64>(Variant(_15, 0), 0) * _1;
Goto(bb18)
}
bb18 = {
place!(Field::<f64>(Variant(RET, 0), 0)) = -_1;
place!(Field::<char>(Variant(_11, 1), 1)) = _12;
_6.0 = [(-20949_i16),25802_i16];
place!(Field::<i128>(Variant(_15, 0), 2)) = 185_u8 as i128;
RET = Adt20::Variant0 { fld0: _1,fld1: _8,fld2: Field::<i128>(Variant(_15, 0), 2),fld3: 44349_u16 };
_12 = Field::<char>(Variant(_11, 1), 1);
place!(Field::<usize>(Variant(_15, 0), 1)) = Field::<usize>(Variant(RET, 0), 1) * _8;
place!(Field::<f64>(Variant(_15, 0), 0)) = Field::<f64>(Variant(RET, 0), 0);
_8 = Field::<usize>(Variant(_15, 0), 1);
place!(Field::<u16>(Variant(_15, 0), 3)) = !23883_u16;
RET = Adt20::Variant0 { fld0: _1,fld1: Field::<usize>(Variant(_15, 0), 1),fld2: Field::<i128>(Variant(_15, 0), 2),fld3: Field::<u16>(Variant(_15, 0), 3) };
place!(Field::<u8>(Variant(_11, 1), 0)) = 108_u8;
place!(Field::<i128>(Variant(RET, 0), 2)) = !Field::<i128>(Variant(_15, 0), 2);
place!(Field::<usize>(Variant(_15, 0), 1)) = Field::<f64>(Variant(RET, 0), 0) as usize;
place!(Field::<i128>(Variant(RET, 0), 2)) = -Field::<i128>(Variant(_15, 0), 2);
(*_18) = 1430140997_u32;
_12 = Field::<char>(Variant(_11, 1), 1);
(*_18) = Field::<u16>(Variant(_15, 0), 3) as u32;
place!(Field::<usize>(Variant(RET, 0), 1)) = Field::<usize>(Variant(_15, 0), 1) >> (*_18);
Goto(bb19)
}
bb19 = {
Call(_22 = dump_var(2_usize, 8_usize, Move(_8), 2_usize, Move(_2), 10_usize, Move(_10), 23_usize, _23), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3() -> Adt20 {
mir! {
type RET = Adt20;
let _1: i64;
let _2: usize;
let _3: Adt67;
let _4: *const u32;
let _5: *mut *const *mut Adt20;
let _6: *mut isize;
let _7: ([i8; 3], char, i32, [u32; 1]);
let _8: f64;
let _9: char;
let _10: [u32; 1];
let _11: Adt20;
let _12: char;
let _13: u8;
let _14: [i64; 1];
let _15: f32;
let _16: (i16, f32, *mut u32, [i32; 6]);
let _17: i32;
let _18: &'static isize;
let _19: usize;
let _20: f32;
let _21: [u128; 8];
let _22: isize;
let _23: *mut isize;
let _24: [i32; 7];
let _25: [i64; 5];
let _26: char;
let _27: [isize; 5];
let _28: *const f64;
let _29: isize;
let _30: *mut *mut Adt31;
let _31: *mut u16;
let _32: (&'static [isize; 5],);
let _33: u8;
let _34: ();
let _35: ();
{
RET = Adt20::Variant1 { fld0: 134_u8,fld1: '\u{420b4}' };
place!(Field::<u8>(Variant(RET, 1), 0)) = false as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{cba76}';
_1 = 4258485490_u32 as i64;
place!(Field::<u8>(Variant(RET, 1), 0)) = 163_u8 | 211_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 339061402579219466144694799147231961843_u128 as u8;
_1 = Field::<char>(Variant(RET, 1), 1) as i64;
RET = Adt20::Variant1 { fld0: 63_u8,fld1: '\u{452ed}' };
_1 = -4502457354093750031_i64;
RET = Adt20::Variant1 { fld0: 134_u8,fld1: '\u{840d3}' };
RET = Adt20::Variant1 { fld0: 179_u8,fld1: '\u{c356d}' };
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{4107}';
_2 = 3_usize & 14820351301596989578_usize;
place!(Field::<u8>(Variant(RET, 1), 0)) = 304146974409598868091220366027964832879_u128 as u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{ea955}';
place!(Field::<u8>(Variant(RET, 1), 0)) = !229_u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{5816c}';
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{1cbf1}';
RET = Adt20::Variant1 { fld0: 104_u8,fld1: '\u{ec061}' };
_2 = 7_usize + 8386693848780000052_usize;
place!(Field::<u8>(Variant(RET, 1), 0)) = 146_u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{ce0ea}';
_2 = Field::<char>(Variant(RET, 1), 1) as usize;
_1 = 1153318073303376652_i64;
Call(RET = fn4(_1, _1, _2, _1, _1, _1, _1, _2, _2, _2, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<u8>(Variant(RET, 1), 0)) = _1 as u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 12_u8 - 151_u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{51a24}';
_1 = !(-8385565532329602182_i64);
_1 = (-739205228655411937_i64);
_2 = 13093508465791104430_usize & 5_usize;
place!(Field::<u8>(Variant(RET, 1), 0)) = !171_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = !203_u8;
RET = Adt20::Variant1 { fld0: 58_u8,fld1: '\u{42b8e}' };
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{def18}';
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{104ee1}';
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{10fa4c}';
place!(Field::<u8>(Variant(RET, 1), 0)) = 225_u8 ^ 212_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 242_u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 168_u8;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{2a818}';
_7.1 = Field::<char>(Variant(RET, 1), 1);
place!(Field::<u8>(Variant(RET, 1), 0)) = true as u8;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463462635402203112799519 => bb6,
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
_8 = (-22924_i16) as f64;
_7.0 = [85_i8,(-21_i8),(-37_i8)];
_7.1 = Field::<char>(Variant(RET, 1), 1);
_7.0 = [(-63_i8),122_i8,(-16_i8)];
_7.3 = [2025042755_u32];
_1 = (-147729275592072722704886806709559270200_i128) as i64;
RET = Adt20::Variant0 { fld0: _8,fld1: _2,fld2: (-9444881982488565147690106953199532818_i128),fld3: 17328_u16 };
place!(Field::<i128>(Variant(RET, 0), 2)) = -52786206319188632651855261496655918516_i128;
place!(Field::<u16>(Variant(RET, 0), 3)) = (-22_i8) as u16;
_7.2 = Field::<u16>(Variant(RET, 0), 3) as i32;
place!(Field::<usize>(Variant(RET, 0), 1)) = _2;
SetDiscriminant(RET, 0);
Goto(bb7)
}
bb7 = {
place!(Field::<u16>(Variant(RET, 0), 3)) = _7.1 as u16;
RET = Adt20::Variant1 { fld0: 181_u8,fld1: _7.1 };
_1 = -(-4793572384075750017_i64);
place!(Field::<u8>(Variant(RET, 1), 0)) = 209_u8;
_9 = Field::<char>(Variant(RET, 1), 1);
_7.1 = Field::<char>(Variant(RET, 1), 1);
Goto(bb8)
}
bb8 = {
_2 = 6_usize + 2184027486788579719_usize;
_2 = 101309223035264720945716636079979795734_i128 as usize;
_1 = (-13965593058976821511074705545978965613_i128) as i64;
_1 = 6095518105503869916_i64 >> Field::<u8>(Variant(RET, 1), 0);
_7.1 = Field::<char>(Variant(RET, 1), 1);
_10 = [1526360918_u32];
_7.3 = _10;
place!(Field::<char>(Variant(RET, 1), 1)) = _9;
place!(Field::<u8>(Variant(RET, 1), 0)) = 194_u8 & 113_u8;
_10 = [3123640313_u32];
_11 = Adt20::Variant0 { fld0: _8,fld1: _2,fld2: (-56995565383995427563852013309017777276_i128),fld3: 10042_u16 };
_7.0 = [43_i8,(-109_i8),(-41_i8)];
_1 = 1888944820482133955_i64 + 1845757746279051756_i64;
place!(Field::<u8>(Variant(RET, 1), 0)) = 17503_i16 as u8;
place!(Field::<f64>(Variant(_11, 0), 0)) = _8 - _8;
RET = Adt20::Variant1 { fld0: 172_u8,fld1: _7.1 };
place!(Field::<u16>(Variant(_11, 0), 3)) = 42626_u16 & 17942_u16;
Call(_2 = core::intrinsics::transmute(Field::<usize>(Variant(_11, 0), 1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<u8>(Variant(RET, 1), 0)) = Field::<u16>(Variant(_11, 0), 3) as u8;
place!(Field::<u8>(Variant(RET, 1), 0)) = 133_u8 ^ 97_u8;
_9 = _7.1;
place!(Field::<usize>(Variant(_11, 0), 1)) = _2 ^ _2;
place!(Field::<u16>(Variant(_11, 0), 3)) = Field::<char>(Variant(RET, 1), 1) as u16;
RET = Adt20::Variant1 { fld0: 54_u8,fld1: _9 };
place!(Field::<f64>(Variant(_11, 0), 0)) = -_8;
place!(Field::<u16>(Variant(_11, 0), 3)) = 48066_u16 * 23766_u16;
_13 = 123_u8 * 67_u8;
place!(Field::<f64>(Variant(_11, 0), 0)) = _8;
_7.1 = Field::<char>(Variant(RET, 1), 1);
_12 = _7.1;
_9 = Field::<char>(Variant(RET, 1), 1);
_10 = [233479932_u32];
place!(Field::<u8>(Variant(RET, 1), 0)) = _13;
place!(Field::<f64>(Variant(_11, 0), 0)) = _8;
_15 = _13 as f32;
place!(Field::<char>(Variant(RET, 1), 1)) = _7.1;
Goto(bb10)
}
bb10 = {
_10 = [1505543288_u32];
_7.1 = Field::<char>(Variant(RET, 1), 1);
_14 = [_1];
_10 = [2959111945_u32];
_16.1 = -_15;
SetDiscriminant(RET, 1);
place!(Field::<char>(Variant(RET, 1), 1)) = _7.1;
_7.3 = _10;
_7.2 = 1676359493337293218_u64 as i32;
_11 = Adt20::Variant1 { fld0: _13,fld1: _12 };
_16.0 = _8 as i16;
_7.3 = [252920742_u32];
_15 = -_16.1;
place!(Field::<u8>(Variant(_11, 1), 0)) = _13 << _1;
place!(Field::<u8>(Variant(RET, 1), 0)) = !_13;
_16.3 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_7.3 = _10;
place!(Field::<u8>(Variant(_11, 1), 0)) = !Field::<u8>(Variant(RET, 1), 0);
place!(Field::<u8>(Variant(RET, 1), 0)) = !_13;
_12 = Field::<char>(Variant(RET, 1), 1);
Goto(bb11)
}
bb11 = {
place!(Field::<u8>(Variant(_11, 1), 0)) = !Field::<u8>(Variant(RET, 1), 0);
SetDiscriminant(_11, 0);
SetDiscriminant(RET, 1);
place!(Field::<i128>(Variant(_11, 0), 2)) = (-147402175341316935267380243434395952441_i128);
_10 = [1721910894_u32];
place!(Field::<u16>(Variant(_11, 0), 3)) = !39024_u16;
_2 = 6_usize;
_19 = _2;
_7.1 = _9;
Goto(bb12)
}
bb12 = {
place!(Field::<char>(Variant(RET, 1), 1)) = _7.1;
_8 = Field::<i128>(Variant(_11, 0), 2) as f64;
_22 = -(-9223372036854775808_isize);
_20 = _7.2 as f32;
place!(Field::<f64>(Variant(_11, 0), 0)) = -_8;
place!(Field::<f64>(Variant(_11, 0), 0)) = 124317440_u32 as f64;
_21[_2] = _16.1 as u128;
_17 = -_7.2;
_18 = &_22;
RET = Adt20::Variant1 { fld0: _13,fld1: _9 };
_13 = !Field::<u8>(Variant(RET, 1), 0);
_2 = _15 as usize;
_26 = _12;
Goto(bb13)
}
bb13 = {
place!(Field::<u16>(Variant(_11, 0), 3)) = true as u16;
_10 = _7.3;
place!(Field::<u8>(Variant(RET, 1), 0)) = _13 & _13;
_24 = [_7.2,_17,_17,_17,_7.2,_17,_17];
place!(Field::<usize>(Variant(_11, 0), 1)) = _2;
_7.0 = [4_i8,(-91_i8),5_i8];
Call(place!(Field::<u8>(Variant(RET, 1), 0)) = core::intrinsics::bswap(_13), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<f64>(Variant(_11, 0), 0)) = -_8;
RET = _11;
_12 = _7.1;
place!(Field::<u16>(Variant(RET, 0), 3)) = Field::<u16>(Variant(_11, 0), 3);
_21[_19] = 186850720301479608489956251566543446312_u128;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(3_usize, 26_usize, Move(_26), 12_usize, Move(_12), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(3_usize, 19_usize, Move(_19), 2_usize, Move(_2), 35_usize, _35, 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i64,mut _2: i64,mut _3: usize,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: i64,mut _12: usize) -> Adt20 {
mir! {
type RET = Adt20;
let _13: f32;
let _14: f32;
let _15: &'static *mut Adt20;
let _16: &'static isize;
let _17: char;
let _18: u128;
let _19: u8;
let _20: (Adt48, Adt67, (i16, i16, [i16; 2]));
let _21: usize;
let _22: isize;
let _23: char;
let _24: f64;
let _25: u16;
let _26: (&'static [isize; 5],);
let _27: f64;
let _28: ();
let _29: ();
{
RET = Adt20::Variant1 { fld0: 147_u8,fld1: '\u{10c503}' };
_10 = _8 << _1;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{8ad67}';
_11 = _2 & _6;
place!(Field::<u8>(Variant(RET, 1), 0)) = !188_u8;
_12 = _8;
_12 = _8;
RET = Adt20::Variant1 { fld0: 48_u8,fld1: '\u{2f759}' };
_2 = '\u{f9da9}' as i64;
_8 = 2159_i16 as usize;
_12 = _11 as usize;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{40609}';
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{60944}';
_5 = 729_u16 as i64;
place!(Field::<u8>(Variant(RET, 1), 0)) = true as u8;
_14 = 5211338591245822145_u64 as f32;
place!(Field::<u8>(Variant(RET, 1), 0)) = 29312726_u32 as u8;
_10 = !_12;
_7 = 2036279875_i32 as i64;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1153318073303376652 => bb9,
_ => bb8
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
Return()
}
bb9 = {
_8 = _10 >> _11;
place!(Field::<u8>(Variant(RET, 1), 0)) = 198_u8;
SetDiscriminant(RET, 0);
place!(Field::<usize>(Variant(RET, 0), 1)) = (-123916258276818162736788120067679124056_i128) as usize;
Goto(bb10)
}
bb10 = {
place!(Field::<f64>(Variant(RET, 0), 0)) = 10660_i16 as f64;
place!(Field::<usize>(Variant(RET, 0), 1)) = _7 as usize;
place!(Field::<i128>(Variant(RET, 0), 2)) = (-44992329247971756381508682600810177241_i128) - (-61506074010804657708190955659696612665_i128);
_10 = _9;
_19 = !23_u8;
place!(Field::<usize>(Variant(RET, 0), 1)) = 703598469_i32 as usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = _8 as u16;
_10 = !_3;
_4 = _11;
_18 = _19 as u128;
_12 = !_3;
_11 = (-9223372036854775808_isize) as i64;
_22 = 9223372036854775807_isize * 9223372036854775807_isize;
_20.2.1 = 11001_i16 * 17352_i16;
place!(Field::<u16>(Variant(RET, 0), 3)) = !13290_u16;
Call(_17 = fn5(_1, _12, _14, _4, _22, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<usize>(Variant(RET, 0), 1)) = _17 as usize;
place!(Field::<usize>(Variant(RET, 0), 1)) = _8 + _8;
place!(Field::<i128>(Variant(RET, 0), 2)) = (-41354090435171083141928942514676451930_i128) | (-14956510766215067460252231681718657520_i128);
_8 = _22 as usize;
place!(Field::<i128>(Variant(RET, 0), 2)) = -56234867788318759628911494485526724749_i128;
_14 = _4 as f32;
_6 = !_1;
place!(Field::<u16>(Variant(RET, 0), 3)) = 6377_u16 << _10;
SetDiscriminant(RET, 1);
place!(Field::<char>(Variant(RET, 1), 1)) = _17;
_22 = 114_i8 as isize;
_14 = _22 as f32;
_8 = !_12;
_7 = _11;
_20.2.2 = [_20.2.1,_20.2.1];
_9 = (-1585409986_i32) as usize;
_20.2.2 = [_20.2.1,_20.2.1];
_14 = _11 as f32;
_14 = _19 as f32;
place!(Field::<u8>(Variant(RET, 1), 0)) = _19 << _5;
_20.2.2 = [_20.2.1,_20.2.1];
_16 = &_22;
_14 = _20.2.1 as f32;
Call(place!(Field::<u8>(Variant(RET, 1), 0)) = fn18(Move(_16)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_21 = !_3;
_19 = !Field::<u8>(Variant(RET, 1), 0);
match _1 {
1153318073303376652 => bb14,
_ => bb13
}
}
bb13 = {
place!(Field::<f64>(Variant(RET, 0), 0)) = 10660_i16 as f64;
place!(Field::<usize>(Variant(RET, 0), 1)) = _7 as usize;
place!(Field::<i128>(Variant(RET, 0), 2)) = (-44992329247971756381508682600810177241_i128) - (-61506074010804657708190955659696612665_i128);
_10 = _9;
_19 = !23_u8;
place!(Field::<usize>(Variant(RET, 0), 1)) = 703598469_i32 as usize;
place!(Field::<u16>(Variant(RET, 0), 3)) = _8 as u16;
_10 = !_3;
_4 = _11;
_18 = _19 as u128;
_12 = !_3;
_11 = (-9223372036854775808_isize) as i64;
_22 = 9223372036854775807_isize * 9223372036854775807_isize;
_20.2.1 = 11001_i16 * 17352_i16;
place!(Field::<u16>(Variant(RET, 0), 3)) = !13290_u16;
Call(_17 = fn5(_1, _12, _14, _4, _22, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb14 = {
_27 = _5 as f64;
_8 = _12 | _9;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(4_usize, 1_usize, Move(_1), 2_usize, Move(_2), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(4_usize, 11_usize, Move(_11), 7_usize, Move(_7), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i64,mut _2: usize,mut _3: f32,mut _4: i64,mut _5: isize,mut _6: i64) -> char {
mir! {
type RET = char;
let _7: [usize; 8];
let _8: i32;
let _9: i32;
let _10: [i128; 3];
let _11: char;
let _12: u16;
let _13: [u16; 3];
let _14: [i64; 5];
let _15: (u32,);
let _16: u32;
let _17: (*mut Adt31,);
let _18: [i8; 3];
let _19: &'static *mut Adt20;
let _20: u32;
let _21: i32;
let _22: (i16, i16, [i16; 2]);
let _23: i16;
let _24: [isize; 5];
let _25: [i128; 3];
let _26: f32;
let _27: isize;
let _28: [u8; 7];
let _29: u16;
let _30: *const (f32,);
let _31: u16;
let _32: u32;
let _33: [i128; 3];
let _34: isize;
let _35: u32;
let _36: &'static *mut Adt20;
let _37: i32;
let _38: *mut [u8; 4];
let _39: bool;
let _40: *mut Adt20;
let _41: isize;
let _42: bool;
let _43: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _44: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _45: ();
let _46: ();
{
_6 = _1;
_4 = _6;
_1 = _2 as i64;
_1 = !_4;
_4 = _1 - _1;
_2 = 3_usize;
_3 = (-564511881_i32) as f32;
RET = '\u{102ffe}';
_2 = 7_usize - 17265457428762774982_usize;
_1 = RET as i64;
_6 = _4 >> _2;
_2 = _5 as usize;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = '\u{7dca3}';
_3 = (-106_i8) as f32;
_8 = RET as i32;
_3 = 16380_u16 as f32;
_1 = !_4;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = '\u{104914}';
_5 = -9223372036854775807_isize;
RET = '\u{10e8a0}';
_9 = _8 >> _4;
_1 = _6;
_1 = _5 as i64;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
Call(_5 = fn6(_9, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = RET as isize;
_10 = [(-80609118887383355799542137305569828831_i128),109815926328618747060012319380685129101_i128,(-145423619557354053785595308750322502905_i128)];
_9 = 58948_u16 as i32;
_10 = [127556368387917008422645446103408510204_i128,146161453718747085307841035338155526262_i128,89442235411185367196108658093337587182_i128];
_6 = 123_u8 as i64;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = '\u{101ee2}';
_5 = 43_isize;
RET = '\u{bab2d}';
_1 = !_6;
_8 = !_9;
RET = '\u{64124}';
_3 = 13051233341174135699_u64 as f32;
_9 = _8;
_4 = !_1;
Goto(bb2)
}
bb2 = {
_3 = (-65_i8) as f32;
_8 = -_9;
_1 = _4;
_5 = 77_isize;
_5 = 819182656_u32 as isize;
_3 = _5 as f32;
_9 = _8 | _8;
Goto(bb3)
}
bb3 = {
RET = '\u{c8f37}';
_5 = 9223372036854775807_isize;
_2 = 6_usize;
_10 = [(-137388622827085948702926359093089208394_i128),29680021792410883108664762493617083686_i128,82267464103309379193999432795231546561_i128];
_6 = _4 ^ _1;
_13 = [13871_u16,44051_u16,6316_u16];
_11 = RET;
_5 = 9223372036854775807_isize >> _7[_2];
Call(_6 = core::intrinsics::bswap(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = RET;
_4 = _6;
RET = _11;
_5 = RET as isize;
_14 = [_6,_4,_6,_6,_1];
_7[_2] = _2 + _2;
_15 = (1768996075_u32,);
_8 = -_9;
RET = _11;
_7[_2] = _2;
_15 = (912505173_u32,);
_11 = RET;
_11 = RET;
RET = _11;
_13 = [24209_u16,41236_u16,61920_u16];
_10 = [(-63584376161295787749634825873535905690_i128),(-40263779385788460367248499790484832722_i128),(-9989305887005996242610747354781927094_i128)];
_2 = _7[_2];
_12 = 64765_u16 + 64330_u16;
_13 = [_12,_12,_12];
_16 = !_15.0;
_7[_2] = _2 >> _4;
_9 = _8 | _8;
_18 = [(-28_i8),(-102_i8),52_i8];
RET = _11;
_20 = _15.0;
match _15.0 {
0 => bb1,
1 => bb2,
912505173 => bb6,
_ => bb5
}
}
bb5 = {
RET = '\u{c8f37}';
_5 = 9223372036854775807_isize;
_2 = 6_usize;
_10 = [(-137388622827085948702926359093089208394_i128),29680021792410883108664762493617083686_i128,82267464103309379193999432795231546561_i128];
_6 = _4 ^ _1;
_13 = [13871_u16,44051_u16,6316_u16];
_11 = RET;
_5 = 9223372036854775807_isize >> _7[_2];
Call(_6 = core::intrinsics::bswap(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_8 = _7[_2] as i32;
_12 = 20833_u16 << _6;
_9 = _8 + _8;
RET = _11;
_13 = [_12,_12,_12];
_21 = _9 >> _9;
_21 = _8;
_9 = _21 ^ _8;
_11 = RET;
_18 = [115_i8,(-25_i8),92_i8];
_14 = [_1,_6,_4,_6,_4];
_4 = _6;
_15.0 = (-20762946813612887248676801383188838108_i128) as u32;
_22.2 = [1100_i16,(-19941_i16)];
_22.1 = 14702_i16;
_8 = -_21;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_18 = [59_i8,68_i8,(-65_i8)];
_10 = [(-52356223976518823731941139986861520693_i128),(-81763031446076376388853857030275133259_i128),110413029967301221238555336002937655923_i128];
_22.1 = (-22880_i16) + 27119_i16;
match _2 {
0 => bb7,
6 => bb9,
_ => bb8
}
}
bb7 = {
_5 = RET as isize;
_10 = [(-80609118887383355799542137305569828831_i128),109815926328618747060012319380685129101_i128,(-145423619557354053785595308750322502905_i128)];
_9 = 58948_u16 as i32;
_10 = [127556368387917008422645446103408510204_i128,146161453718747085307841035338155526262_i128,89442235411185367196108658093337587182_i128];
_6 = 123_u8 as i64;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = '\u{101ee2}';
_5 = 43_isize;
RET = '\u{bab2d}';
_1 = !_6;
_8 = !_9;
RET = '\u{64124}';
_3 = 13051233341174135699_u64 as f32;
_9 = _8;
_4 = !_1;
Goto(bb2)
}
bb8 = {
_3 = (-65_i8) as f32;
_8 = -_9;
_1 = _4;
_5 = 77_isize;
_5 = 819182656_u32 as isize;
_3 = _5 as f32;
_9 = _8 | _8;
Goto(bb3)
}
bb9 = {
_1 = _4;
_21 = !_9;
_4 = _6;
_7[_2] = _2 * _2;
_15 = (_20,);
_25 = [59887516051884517818391078552013747719_i128,149818974521609819398153775695206525639_i128,(-23655586463032277158468384198272803057_i128)];
_16 = !_15.0;
_26 = -_3;
_23 = 12033539942056820865_u64 as i16;
_7[_2] = _2;
_3 = -_26;
_22.0 = _23 + _23;
_9 = -_21;
_7[_2] = _2;
_18 = [(-97_i8),(-62_i8),(-72_i8)];
Goto(bb10)
}
bb10 = {
_11 = RET;
_26 = _3 - _3;
_11 = RET;
_2 = !_7[_2];
_22.0 = _23;
_16 = 25_i8 as u32;
_22.2 = [_22.0,_23];
_20 = !_15.0;
_22.1 = _23;
_5 = _4 as isize;
_16 = _22.0 as u32;
_1 = _4;
_15 = (_16,);
_23 = -_22.1;
_3 = -_26;
_5 = !9223372036854775807_isize;
_20 = _22.0 as u32;
_9 = _21;
_8 = _9;
_23 = _22.1;
_27 = _5;
_20 = !_15.0;
_6 = -_4;
_24 = [_5,_5,_5,_5,_27];
_15 = (_20,);
RET = _11;
RET = _11;
Goto(bb11)
}
bb11 = {
_11 = RET;
_21 = _9 >> _9;
Goto(bb12)
}
bb12 = {
_33 = [19834978715972284459507104445254074819_i128,55443640952837958778191922974390742086_i128,(-103539739662910686765399555752781395251_i128)];
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_8 = _21 & _21;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_11 = RET;
_4 = _1;
_6 = _1;
_9 = 266564283778295716876415606807833557717_u128 as i32;
_14 = [_4,_6,_4,_1,_4];
_11 = RET;
_34 = _1 as isize;
_3 = -_26;
_15.0 = _12 as u32;
_26 = _3 + _3;
_26 = _6 as f32;
_21 = -_8;
_24 = [_27,_34,_34,_27,_34];
Goto(bb14)
}
bb14 = {
_6 = _1;
_20 = _15.0;
_31 = _2 as u16;
_15 = (_16,);
_6 = true as i64;
_19 = &_40;
_26 = -_3;
_39 = true;
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_19 = &_40;
_11 = RET;
_4 = _1 - _6;
_28 = [99_u8,134_u8,21_u8,211_u8,224_u8,148_u8,113_u8];
_15 = (_20,);
_19 = &(*_19);
_41 = _5;
_43.2.2 = _39 as i32;
RET = _11;
_10 = [(-116309900282739002851314814787430854738_i128),(-12380943922893137467476086312469748050_i128),156357105380269285406982970432373815803_i128];
_43.1 = _12 as isize;
_3 = _8 as f32;
_19 = &_40;
_6 = _4 + _4;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(5_usize, 24_usize, Move(_24), 31_usize, Move(_31), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(5_usize, 1_usize, Move(_1), 23_usize, Move(_23), 16_usize, Move(_16), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(5_usize, 4_usize, Move(_4), 41_usize, Move(_41), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(5_usize, 27_usize, Move(_27), 33_usize, Move(_33), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i32,mut _2: [usize; 8]) -> isize {
mir! {
type RET = isize;
let _3: Adt48;
let _4: f64;
let _5: isize;
let _6: *mut [u16; 3];
let _7: u64;
let _8: isize;
let _9: *const ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _10: &'static usize;
let _11: (i8, *const f64, i16, i32);
let _12: char;
let _13: *mut [u8; 4];
let _14: [u16; 3];
let _15: *mut [u8; 4];
let _16: [i64; 7];
let _17: u128;
let _18: isize;
let _19: &'static [isize; 5];
let _20: bool;
let _21: *mut [i128; 3];
let _22: &'static &'static isize;
let _23: (&'static [isize; 5],);
let _24: isize;
let _25: &'static &'static *const *mut Adt20;
let _26: Adt74;
let _27: u32;
let _28: [u16; 1];
let _29: f32;
let _30: u32;
let _31: [u16; 3];
let _32: Adt20;
let _33: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32);
let _34: usize;
let _35: f32;
let _36: u16;
let _37: i64;
let _38: i64;
let _39: i8;
let _40: ();
let _41: ();
{
_2 = [3977071971249257834_usize,7_usize,17404499591502746038_usize,2_usize,2_usize,1674510715870801786_usize,2_usize,0_usize];
RET = 9223372036854775807_isize;
_4 = 8_i8 as f64;
RET = -9223372036854775807_isize;
_4 = (-8962374419552640923_i64) as f64;
RET = (-74_isize) ^ 9223372036854775807_isize;
_5 = RET;
RET = _5 ^ _5;
_4 = (-128_i8) as f64;
RET = (-27382_i16) as isize;
_7 = 1990573955259680706_u64 | 7012042502207952251_u64;
_4 = 117143703855205230779017255887183666814_i128 as f64;
_1 = 1152359268_i32;
_7 = 209281989867494968_u64 * 11054722031744575853_u64;
_7 = 7756446670865049363_u64 - 4845506454042434724_u64;
_4 = 170_u8 as f64;
_2 = [10053082767144277445_usize,13180521668314158025_usize,2_usize,15112588700317264865_usize,18141424621542511066_usize,12036039250637692062_usize,6_usize,1_usize];
RET = 154985356238534954613586502635260593765_i128 as isize;
_4 = 3395479393_u32 as f64;
_1 = (-81_i8) as i32;
RET = -_5;
Goto(bb1)
}
bb1 = {
_8 = _1 as isize;
RET = _8 >> _5;
_14 = [37935_u16,1339_u16,64176_u16];
_4 = 2_usize as f64;
_5 = 3523377902_u32 as isize;
_11.0 = (-27015_i16) as i8;
_6 = core::ptr::addr_of_mut!(_14);
_8 = 26_u8 as isize;
_11.2 = (-4363_i16) + 5200_i16;
RET = 2069663549_u32 as isize;
RET = _8 ^ _5;
_12 = '\u{2ca7d}';
_6 = core::ptr::addr_of_mut!((*_6));
_16 = [(-2241483233592347152_i64),(-2273054618465478812_i64),(-6281045566432946849_i64),5517361193052079407_i64,(-32674108090831170_i64),5460572761098319248_i64,(-7097897239021849669_i64)];
_14 = [1491_u16,52981_u16,61362_u16];
RET = _11.0 as isize;
_11.2 = 20355_i16;
_11.0 = !(-74_i8);
_17 = !326537277676118910485186769931175613337_u128;
match _11.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
20355 => bb10,
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
_20 = _11.2 == _11.2;
_17 = _1 as u128;
_16 = [2471409339406654138_i64,2125178484324544865_i64,(-5034377204397675584_i64),2224923340668551899_i64,(-555169306335043839_i64),(-8810452110372915583_i64),(-3683875391780968901_i64)];
(*_6) = [14515_u16,34392_u16,43611_u16];
_20 = !false;
_17 = _20 as u128;
Call(_11.2 = fn7(_16, (*_6), _2, _7, _7, _2, _12, Move(_6), _16, _12, _16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12 = '\u{554a2}';
_17 = 221483252755981828537961428220304038894_u128 + 126107056164234322095623350394287460594_u128;
_2 = [18301639235390093594_usize,3_usize,4991226038008908373_usize,7_usize,18066544195206397679_usize,6531059635464697645_usize,11258105050938039485_usize,5_usize];
_4 = _11.0 as f64;
_11.3 = _1 & _1;
_6 = core::ptr::addr_of_mut!(_14);
_11.2 = 24120_i16 + 15343_i16;
_24 = !RET;
_11.3 = !_1;
_18 = _24 + _8;
_24 = _18;
_11.0 = _11.3 as i8;
_20 = true ^ false;
_11.3 = -_1;
_5 = RET;
_4 = 23213_u16 as f64;
_11.0 = (-57_i8);
_18 = (-347153087727588370_i64) as isize;
RET = _24;
_24 = _18 * RET;
_11.2 = 11538_i16;
_20 = !false;
_11.1 = core::ptr::addr_of!(_4);
_27 = 693146461_u32 - 2584600173_u32;
RET = _24 >> _11.3;
_11.1 = core::ptr::addr_of!(_4);
_4 = _11.0 as f64;
_5 = _24;
Goto(bb12)
}
bb12 = {
(*_6) = [44136_u16,53833_u16,46966_u16];
_7 = 2182578709605678820_u64 + 12573533949809201342_u64;
_11.2 = 27956_i16;
_28 = [23929_u16];
_14 = [6430_u16,48489_u16,17505_u16];
_28 = [28823_u16];
_2 = [7_usize,2_usize,12350384870331362474_usize,5_usize,2_usize,10488116217395130058_usize,18387360112189825488_usize,1_usize];
_1 = _11.3;
_30 = _27 - _27;
_16 = [2880771745675856242_i64,(-2960521248296725122_i64),524752151601224348_i64,(-8392923545019013065_i64),(-2920864986879893697_i64),2726795749840966670_i64,(-4529786883774662516_i64)];
_24 = RET;
_11.3 = _1 ^ _1;
(*_6) = [4361_u16,50617_u16,35949_u16];
_7 = 6171041842074591866_u64 * 5799175386367589770_u64;
match _11.2 {
0 => bb1,
27956 => bb14,
_ => bb13
}
}
bb13 = {
_12 = '\u{554a2}';
_17 = 221483252755981828537961428220304038894_u128 + 126107056164234322095623350394287460594_u128;
_2 = [18301639235390093594_usize,3_usize,4991226038008908373_usize,7_usize,18066544195206397679_usize,6531059635464697645_usize,11258105050938039485_usize,5_usize];
_4 = _11.0 as f64;
_11.3 = _1 & _1;
_6 = core::ptr::addr_of_mut!(_14);
_11.2 = 24120_i16 + 15343_i16;
_24 = !RET;
_11.3 = !_1;
_18 = _24 + _8;
_24 = _18;
_11.0 = _11.3 as i8;
_20 = true ^ false;
_11.3 = -_1;
_5 = RET;
_4 = 23213_u16 as f64;
_11.0 = (-57_i8);
_18 = (-347153087727588370_i64) as isize;
RET = _24;
_24 = _18 * RET;
_11.2 = 11538_i16;
_20 = !false;
_11.1 = core::ptr::addr_of!(_4);
_27 = 693146461_u32 - 2584600173_u32;
RET = _24 >> _11.3;
_11.1 = core::ptr::addr_of!(_4);
_4 = _11.0 as f64;
_5 = _24;
Goto(bb12)
}
bb14 = {
_32 = Adt20::Variant1 { fld0: 111_u8,fld1: _12 };
_33.0.0 = _12 as i16;
_5 = _11.0 as isize;
_1 = RET as i32;
_6 = core::ptr::addr_of_mut!((*_6));
_28 = [5981_u16];
_16 = [(-5585794810491022389_i64),7781456159294585682_i64,416253404678885571_i64,8514289592828876904_i64,(-6113932970999036415_i64),(-1927780719395919167_i64),3049198785133038781_i64];
_12 = Field::<char>(Variant(_32, 1), 1);
_4 = _17 as f64;
_33.0.1 = 2467970008726869916_usize as f32;
_33.1 = [_24,_8,_24,_18,_5];
_29 = _17 as f32;
_20 = !false;
_32 = Adt20::Variant1 { fld0: 31_u8,fld1: _12 };
_23.0 = &_33.1;
_27 = _30;
_33.0.3 = [_1,_1,_11.3,_1,_1,_11.3];
_7 = 11370803272500259941_u64 ^ 6352115987274679295_u64;
_32 = Adt20::Variant0 { fld0: _4,fld1: 1_usize,fld2: (-92992801753450407474177266012789136901_i128),fld3: 8265_u16 };
_38 = (-6069836988025520120_i64);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(6_usize, 28_usize, Move(_28), 2_usize, Move(_2), 14_usize, Move(_14), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(6_usize, 17_usize, Move(_17), 20_usize, Move(_20), 18_usize, Move(_18), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [i64; 7],mut _2: [u16; 3],mut _3: [usize; 8],mut _4: u64,mut _5: u64,mut _6: [usize; 8],mut _7: char,mut _8: *mut [u16; 3],mut _9: [i64; 7],mut _10: char,mut _11: [i64; 7]) -> i16 {
mir! {
type RET = i16;
let _12: [u8; 7];
let _13: u32;
let _14: *const *mut Adt20;
let _15: (f32,);
let _16: char;
let _17: *mut u16;
let _18: (Adt48, Adt67, (i16, i16, [i16; 2]));
let _19: isize;
let _20: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _21: isize;
let _22: [u16; 1];
let _23: ([u32; 1], Adt31, *const *mut Adt20, *const (f32,));
let _24: Adt31;
let _25: char;
let _26: Adt35;
let _27: f32;
let _28: usize;
let _29: ([i16; 2], &'static *const u32);
let _30: char;
let _31: i32;
let _32: [i16; 2];
let _33: [i128; 3];
let _34: ();
let _35: ();
{
_1 = [6776419486929492449_i64,8748163608980200756_i64,4855629835963150732_i64,3252990381444510385_i64,(-1666785906429263056_i64),4736819411919329987_i64,(-2073451625685201138_i64)];
_1 = _11;
_7 = _10;
_2 = [2503_u16,12667_u16,43732_u16];
_9 = [255751286786820935_i64,3318118793338646433_i64,5027275670838532658_i64,8292578611035650032_i64,(-3187565345926536372_i64),8209270805049843058_i64,(-1232312905520949018_i64)];
RET = 26901_i16 ^ (-13143_i16);
_12 = [91_u8,85_u8,191_u8,192_u8,120_u8,26_u8,25_u8];
_3 = _6;
_5 = _4;
RET = (-6852_i16);
RET = true as i16;
_13 = !1816510174_u32;
_5 = !_4;
_10 = _7;
_9 = [7973658778603836441_i64,7587742730469506813_i64,8025983496354046576_i64,7240712684578940432_i64,(-1128751990054625141_i64),8836033357239687727_i64,4238768073835096176_i64];
RET = (-22197_i16) - (-3807_i16);
_8 = core::ptr::addr_of_mut!(_2);
_4 = !_5;
_13 = 1926830677_u32;
_8 = core::ptr::addr_of_mut!((*_8));
_7 = _10;
Goto(bb1)
}
bb1 = {
RET = !(-25536_i16);
_11 = [(-1763185088143465141_i64),(-6660351579606210290_i64),(-1945858459676482016_i64),(-5885248533940688650_i64),3920469329704621921_i64,7868889177794123201_i64,9083797992263777362_i64];
_13 = 2909059392_u32 >> RET;
Call(_12 = fn8(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [55297_u16,27350_u16,45549_u16];
_3 = _6;
_13 = 275402259_u32 | 1998759947_u32;
_2 = [65023_u16,31151_u16,28636_u16];
(*_8) = [59238_u16,41082_u16,28692_u16];
_10 = _7;
_12 = [91_u8,100_u8,166_u8,214_u8,177_u8,0_u8,40_u8];
_16 = _10;
_4 = _5 - _5;
_1 = _9;
_13 = !3490652533_u32;
_15.0 = 5536445348074600316_i64 as f32;
_1 = [9058238268044262343_i64,(-6194922820836725564_i64),(-6719780420421865269_i64),(-901132309383526285_i64),(-4568842120568592099_i64),4880462854078619639_i64,(-7177046074896081448_i64)];
_13 = 3207919863_u32 - 1485297390_u32;
RET = true as i16;
_11 = [(-7317066992129242517_i64),(-3666034210819035978_i64),(-4481803576983701560_i64),2924531197848620424_i64,(-8139873873958848985_i64),(-1304202267742991418_i64),4810404520662165633_i64];
_13 = (-26_i8) as u32;
(*_8) = [22836_u16,2316_u16,27545_u16];
_9 = [1193284773606438867_i64,(-2528258893995124632_i64),8758339634435988033_i64,3559318946001216739_i64,6163387228471863420_i64,7900852007288752736_i64,(-7685490940779523607_i64)];
(*_8) = [21156_u16,39980_u16,41631_u16];
_2 = [23105_u16,30259_u16,10549_u16];
_2 = [34554_u16,42263_u16,57330_u16];
_5 = _4 - _4;
RET = 9223372036854775807_isize as i16;
Call(_11 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = (-2672_i16) + (-27998_i16);
_2 = [59652_u16,18258_u16,50630_u16];
_3 = [16414476475451975375_usize,10875128043180760982_usize,6_usize,4_usize,4_usize,3_usize,3_usize,2_usize];
_15.0 = 63307_u16 as f32;
_18.2.2 = [RET,RET];
_7 = _10;
_8 = core::ptr::addr_of_mut!((*_8));
_15.0 = 8327867994236030859_usize as f32;
_12 = [28_u8,25_u8,6_u8,162_u8,6_u8,194_u8,159_u8];
_20.1 = 9223372036854775807_isize >> RET;
_16 = _10;
_20.2.2 = -1571954536_i32;
_19 = _20.1 >> _5;
_16 = _7;
_19 = _20.1;
Goto(bb4)
}
bb4 = {
_2 = [54156_u16,36114_u16,26032_u16];
_8 = core::ptr::addr_of_mut!((*_8));
_16 = _7;
_3 = [14720259806028697698_usize,17905036328102114390_usize,3_usize,1_usize,1_usize,1_usize,3_usize,10360498572101998427_usize];
_20.0 = [_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2];
_20.2.1 = _7;
_20.1 = _19;
_20.2.0 = [79_i8,86_i8,110_i8];
_19 = _5 as isize;
_20.2.0 = [(-60_i8),11_i8,59_i8];
_22 = [36923_u16];
_20.2.2 = !63016541_i32;
_9 = [7271323877280741521_i64,2039889853588737030_i64,993163367410638271_i64,7800822497415454561_i64,(-4844221203913729856_i64),(-639697746268282421_i64),(-557602024241552905_i64)];
Goto(bb5)
}
bb5 = {
_7 = _10;
_20.0 = [_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2];
_23.0 = [_13];
RET = _19 as i16;
(*_8) = [58455_u16,3326_u16,1506_u16];
_18.2.0 = RET;
_20.2.3 = _23.0;
_23.0 = [_13];
_23.3 = core::ptr::addr_of!(_15);
_15.0 = 294068861080354431757404100778511224676_u128 as f32;
_28 = !1_usize;
_5 = (-116037705854144910050793483914916081293_i128) as u64;
_9 = [(-1001929256266212321_i64),7841331052969353046_i64,4424542302460212032_i64,4541453517513042641_i64,(-2222564184009232597_i64),(-5840827018064994046_i64),(-7446134850976419983_i64)];
_26 = Adt35 { fld0: _4 };
_7 = _16;
_23.0 = [_13];
_20.2.1 = _16;
_19 = -_20.1;
_25 = _20.2.1;
Goto(bb6)
}
bb6 = {
_20.2.0 = [46_i8,119_i8,106_i8];
_20.0 = [_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2];
_21 = _28 as isize;
_25 = _7;
_20.0 = [_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2];
_16 = _7;
_4 = _26.fld0;
_18.2.2 = [RET,_18.2.0];
_23.3 = core::ptr::addr_of!(_15);
_30 = _16;
_31 = false as i32;
_8 = core::ptr::addr_of_mut!(_2);
_23.0 = [_13];
_26 = Adt35 { fld0: _4 };
_4 = !_26.fld0;
(*_8) = [11845_u16,61414_u16,11819_u16];
_4 = !_26.fld0;
_20.2.3 = [_13];
(*_8) = [57828_u16,41752_u16,26007_u16];
_26 = Adt35 { fld0: _4 };
(*_8) = [43002_u16,29843_u16,28295_u16];
_2 = [6168_u16,41266_u16,3735_u16];
_18.2.0 = RET | RET;
_22 = [24374_u16];
_20.2.2 = -_31;
_11 = [(-3057734017051772529_i64),955449361773196779_i64,8221565765991776210_i64,(-6429563812403923309_i64),(-6132914140107072932_i64),4959892686555307000_i64,4140417793970366160_i64];
RET = !_18.2.0;
Goto(bb7)
}
bb7 = {
Call(_34 = dump_var(7_usize, 30_usize, Move(_30), 31_usize, Move(_31), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_34 = dump_var(7_usize, 21_usize, Move(_21), 28_usize, Move(_28), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_34 = dump_var(7_usize, 9_usize, Move(_9), 22_usize, Move(_22), 35_usize, _35, 35_usize, _35), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i64; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _2: isize;
let _3: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32);
let _4: i128;
let _5: isize;
let _6: u32;
let _7: isize;
let _8: *mut [u16; 3];
let _9: isize;
let _10: ((i8, *const f64, i16, i32),);
let _11: isize;
let _12: isize;
let _13: *mut *mut Adt31;
let _14: f64;
let _15: char;
let _16: bool;
let _17: [u8; 7];
let _18: [u32; 1];
let _19: isize;
let _20: f64;
let _21: &'static *const u32;
let _22: &'static i16;
let _23: &'static *const *mut Adt20;
let _24: *const u32;
let _25: *mut Adt31;
let _26: u8;
let _27: [i64; 7];
let _28: char;
let _29: usize;
let _30: f64;
let _31: ();
let _32: ();
{
RET = [30_u8,69_u8,63_u8,218_u8,244_u8,122_u8,99_u8];
RET = [211_u8,237_u8,143_u8,120_u8,70_u8,47_u8,28_u8];
_1 = [3292173572362721804_i64,165863619852814009_i64,(-1195265030648194071_i64),(-5317007663847588347_i64),1656514497717918141_i64,(-3558390008974865369_i64),(-8869960951339760338_i64)];
RET = [150_u8,146_u8,145_u8,97_u8,85_u8,95_u8,167_u8];
RET = [190_u8,81_u8,86_u8,31_u8,190_u8,10_u8,1_u8];
RET = [201_u8,215_u8,9_u8,31_u8,182_u8,47_u8,222_u8];
RET = [68_u8,138_u8,78_u8,30_u8,57_u8,117_u8,30_u8];
RET = [87_u8,221_u8,12_u8,36_u8,252_u8,78_u8,86_u8];
RET = [75_u8,2_u8,166_u8,159_u8,98_u8,32_u8,160_u8];
RET = [228_u8,204_u8,98_u8,228_u8,172_u8,234_u8,80_u8];
RET = [252_u8,197_u8,246_u8,45_u8,56_u8,84_u8,206_u8];
_1 = [5083203408579732638_i64,1124135386322677181_i64,(-3153681696996266089_i64),3797046885522517920_i64,7573249995285908462_i64,3933165679110413641_i64,(-3100854177430011544_i64)];
RET = [63_u8,111_u8,181_u8,237_u8,39_u8,218_u8,81_u8];
RET = [196_u8,255_u8,133_u8,101_u8,140_u8,183_u8,47_u8];
RET = [109_u8,177_u8,163_u8,110_u8,214_u8,223_u8,154_u8];
RET = [235_u8,208_u8,55_u8,60_u8,134_u8,217_u8,158_u8];
_1 = [3051610519219309864_i64,(-6047444058077867176_i64),(-4395660740972493768_i64),6824829056544700768_i64,7495432749677429421_i64,4605632083381675952_i64,(-1589759526404261100_i64)];
RET = [99_u8,177_u8,82_u8,45_u8,224_u8,90_u8,29_u8];
RET = [138_u8,237_u8,97_u8,52_u8,168_u8,246_u8,61_u8];
RET = [91_u8,246_u8,241_u8,155_u8,139_u8,209_u8,149_u8];
_1 = [(-702204400806683830_i64),8597884841298942774_i64,2163685083634654576_i64,(-6454998269325942849_i64),(-4234526254151094282_i64),3086683731789987750_i64,(-7740534159207693955_i64)];
RET = [66_u8,61_u8,212_u8,210_u8,164_u8,151_u8,98_u8];
RET = [37_u8,79_u8,43_u8,224_u8,149_u8,10_u8,117_u8];
RET = [112_u8,116_u8,138_u8,247_u8,54_u8,99_u8,49_u8];
RET = [104_u8,134_u8,234_u8,230_u8,223_u8,25_u8,62_u8];
_1 = [2955298698175648134_i64,7203891218407159305_i64,(-2027957536198645098_i64),(-8134346820887260527_i64),(-4968977812833268685_i64),(-2275527782203329860_i64),2853755925699689517_i64];
RET = [138_u8,54_u8,167_u8,145_u8,106_u8,250_u8,182_u8];
_2 = 9223372036854775807_isize + 47_isize;
_1 = [(-1025343894148712642_i64),1378930745635581470_i64,(-413469616700811828_i64),(-3767646477531615943_i64),(-3716599937956125774_i64),(-2058455836249677591_i64),(-4689378348219113381_i64)];
RET = [29_u8,8_u8,30_u8,226_u8,242_u8,107_u8,120_u8];
_2 = 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_1 = [3635069447635330747_i64,(-2341708774440940072_i64),(-2597923906990340243_i64),(-1026793201766305173_i64),413870465449612078_i64,(-4110029901395737755_i64),(-183611215065053891_i64)];
_1 = [(-3226576499881439253_i64),6616703991156437443_i64,8992353099680797866_i64,(-5498638661477739690_i64),(-1084662180867928745_i64),(-3001312604856108979_i64),(-2484227532973875236_i64)];
_2 = 9223372036854775807_isize;
_2 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
RET = [7_u8,196_u8,152_u8,65_u8,111_u8,189_u8,211_u8];
_2 = '\u{b21e0}' as isize;
RET = [23_u8,150_u8,217_u8,185_u8,78_u8,178_u8,21_u8];
_2 = (-2_isize);
_1 = [(-3899760700645037681_i64),(-9099530387388072489_i64),7804706050205681660_i64,7921608163655520050_i64,(-6751822006469362870_i64),(-5313401416789182491_i64),(-7369813833854816330_i64)];
_3.0.0 = false as i16;
_3.0.1 = 78454199467601554209780147738530595803_u128 as f32;
_1 = [8346284788638813433_i64,(-5194832843112134080_i64),1685261018516668895_i64,(-252748019427647459_i64),2696207138472260097_i64,6007082390464888109_i64,(-1863754675879036112_i64)];
_3.1 = [_2,_2,_2,_2,_2];
_3.0.0 = (-20932_i16) + (-7165_i16);
_3.0.1 = (-4563932702910067267_i64) as f32;
RET = [160_u8,83_u8,200_u8,116_u8,175_u8,76_u8,118_u8];
_3.2 = _3.0.1;
_3.0.3 = [(-7018116_i32),1861321300_i32,2105214223_i32,1112378832_i32,(-1928371399_i32),(-1067548969_i32)];
_3.0.1 = -_3.2;
_3.0.3 = [1529224012_i32,(-920847585_i32),(-1942952463_i32),(-857254387_i32),304542999_i32,974780487_i32];
_3.0.3 = [(-1646930637_i32),(-584046603_i32),(-295669458_i32),1590928323_i32,512776003_i32,(-894431416_i32)];
_3.2 = _3.0.1;
_3.0.1 = -_3.2;
_4 = -(-133878211074726339922812449282121377761_i128);
_1 = [(-221364099157849035_i64),841861481631923867_i64,(-1690243461326601637_i64),(-1558887183340948453_i64),6922323049890921520_i64,8063212421685847262_i64,(-4425901711197626558_i64)];
RET = [90_u8,72_u8,84_u8,180_u8,60_u8,113_u8,102_u8];
Goto(bb2)
}
bb2 = {
_4 = 108294677750079248990824562865631969779_i128;
_3.0.3 = [796863604_i32,(-1081614649_i32),(-1975213032_i32),(-177552923_i32),1012629952_i32,795502669_i32];
_6 = 3309390052_u32;
_4 = !(-46196499082174343705704806023686005428_i128);
_3.0.0 = (-29844_i16);
_4 = 21421910423251468280904758509792947342_i128 * (-85912523316403756243653114524477120805_i128);
_5 = _2 * _2;
RET = [238_u8,31_u8,164_u8,33_u8,232_u8,244_u8,185_u8];
_3.0.2 = core::ptr::addr_of_mut!(_6);
_3.0.3 = [1394432655_i32,137478569_i32,1936157334_i32,1659777849_i32,(-1517443059_i32),1422439750_i32];
_5 = -_2;
Goto(bb3)
}
bb3 = {
_4 = _2 as i128;
RET = [154_u8,239_u8,109_u8,192_u8,34_u8,138_u8,252_u8];
Goto(bb4)
}
bb4 = {
_3.0.1 = _3.2 + _3.2;
_3.0.3 = [(-605210500_i32),(-1891688715_i32),1688137705_i32,(-18110373_i32),1589624393_i32,(-1512904160_i32)];
_3.1 = [_5,_5,_2,_5,_5];
_7 = (-7982897789389621914_i64) as isize;
_3.0.2 = core::ptr::addr_of_mut!(_6);
_1 = [5456409984299883471_i64,(-295194784818073061_i64),2963270621145338759_i64,(-5041378788176612286_i64),3141595965812761125_i64,(-7145601789203534605_i64),8573261218479197055_i64];
_6 = !312303791_u32;
_3.0.1 = _3.2;
RET = [243_u8,253_u8,153_u8,119_u8,3_u8,25_u8,121_u8];
_3.0.3 = [(-239964151_i32),266517466_i32,688352053_i32,144949037_i32,(-1971597196_i32),275084026_i32];
_5 = 286303366685539811600590151086900434795_u128 as isize;
_3.0.1 = 43238440758762003065369571015928439031_u128 as f32;
_7 = _5;
_5 = _2;
_10.0.3 = 10224345666047868037_u64 as i32;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211454 => bb6,
_ => bb5
}
}
bb5 = {
_4 = _2 as i128;
RET = [154_u8,239_u8,109_u8,192_u8,34_u8,138_u8,252_u8];
Goto(bb4)
}
bb6 = {
Call(_9 = core::intrinsics::transmute(_2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = [4126983394768297231_i64,(-4187807765382417041_i64),(-3936212085881169029_i64),1988783225905951319_i64,8991725683432326241_i64,3064350762105391332_i64,4130820408159611139_i64];
RET = [195_u8,227_u8,21_u8,85_u8,40_u8,167_u8,145_u8];
_10.0.2 = _3.0.0 >> _4;
_12 = _4 as isize;
_10.0.0 = 86_i8 + 112_i8;
_3.2 = _3.0.1 - _3.0.1;
_10.0.1 = core::ptr::addr_of!(_14);
RET = [141_u8,33_u8,131_u8,107_u8,196_u8,157_u8,204_u8];
_6 = 2634458944_u32;
_6 = 3011408011_u32;
_3.0.3 = [_10.0.3,_10.0.3,_10.0.3,_10.0.3,_10.0.3,_10.0.3];
_6 = 2097206912_u32 & 822591085_u32;
_10.0.1 = core::ptr::addr_of!(_14);
_1 = [7409109870747416134_i64,6914260298395304533_i64,181839527717021989_i64,4097158580141491117_i64,(-5919659307690116160_i64),(-8193520412666827959_i64),(-6175774313146522482_i64)];
_6 = 1947325617_u32 + 428980203_u32;
_3.0.2 = core::ptr::addr_of_mut!(_6);
_10.0.0 = (-87_i8) & (-109_i8);
_5 = _12 & _7;
_1 = [6155519215962111703_i64,2977924852285540791_i64,7020957392422959362_i64,7451001101548778087_i64,(-4344491281700496089_i64),(-7438096727427229339_i64),(-9214091114747167046_i64)];
_14 = _4 as f64;
RET = [155_u8,141_u8,42_u8,70_u8,230_u8,147_u8,119_u8];
match _2 {
0 => bb2,
1 => bb8,
340282366920938463463374607431768211454 => bb10,
_ => bb9
}
}
bb8 = {
_1 = [3635069447635330747_i64,(-2341708774440940072_i64),(-2597923906990340243_i64),(-1026793201766305173_i64),413870465449612078_i64,(-4110029901395737755_i64),(-183611215065053891_i64)];
_1 = [(-3226576499881439253_i64),6616703991156437443_i64,8992353099680797866_i64,(-5498638661477739690_i64),(-1084662180867928745_i64),(-3001312604856108979_i64),(-2484227532973875236_i64)];
_2 = 9223372036854775807_isize;
_2 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
RET = [7_u8,196_u8,152_u8,65_u8,111_u8,189_u8,211_u8];
_2 = '\u{b21e0}' as isize;
RET = [23_u8,150_u8,217_u8,185_u8,78_u8,178_u8,21_u8];
_2 = (-2_isize);
_1 = [(-3899760700645037681_i64),(-9099530387388072489_i64),7804706050205681660_i64,7921608163655520050_i64,(-6751822006469362870_i64),(-5313401416789182491_i64),(-7369813833854816330_i64)];
_3.0.0 = false as i16;
_3.0.1 = 78454199467601554209780147738530595803_u128 as f32;
_1 = [8346284788638813433_i64,(-5194832843112134080_i64),1685261018516668895_i64,(-252748019427647459_i64),2696207138472260097_i64,6007082390464888109_i64,(-1863754675879036112_i64)];
_3.1 = [_2,_2,_2,_2,_2];
_3.0.0 = (-20932_i16) + (-7165_i16);
_3.0.1 = (-4563932702910067267_i64) as f32;
RET = [160_u8,83_u8,200_u8,116_u8,175_u8,76_u8,118_u8];
_3.2 = _3.0.1;
_3.0.3 = [(-7018116_i32),1861321300_i32,2105214223_i32,1112378832_i32,(-1928371399_i32),(-1067548969_i32)];
_3.0.1 = -_3.2;
_3.0.3 = [1529224012_i32,(-920847585_i32),(-1942952463_i32),(-857254387_i32),304542999_i32,974780487_i32];
_3.0.3 = [(-1646930637_i32),(-584046603_i32),(-295669458_i32),1590928323_i32,512776003_i32,(-894431416_i32)];
_3.2 = _3.0.1;
_3.0.1 = -_3.2;
_4 = -(-133878211074726339922812449282121377761_i128);
_1 = [(-221364099157849035_i64),841861481631923867_i64,(-1690243461326601637_i64),(-1558887183340948453_i64),6922323049890921520_i64,8063212421685847262_i64,(-4425901711197626558_i64)];
RET = [90_u8,72_u8,84_u8,180_u8,60_u8,113_u8,102_u8];
Goto(bb2)
}
bb9 = {
_3.0.1 = _3.2 + _3.2;
_3.0.3 = [(-605210500_i32),(-1891688715_i32),1688137705_i32,(-18110373_i32),1589624393_i32,(-1512904160_i32)];
_3.1 = [_5,_5,_2,_5,_5];
_7 = (-7982897789389621914_i64) as isize;
_3.0.2 = core::ptr::addr_of_mut!(_6);
_1 = [5456409984299883471_i64,(-295194784818073061_i64),2963270621145338759_i64,(-5041378788176612286_i64),3141595965812761125_i64,(-7145601789203534605_i64),8573261218479197055_i64];
_6 = !312303791_u32;
_3.0.1 = _3.2;
RET = [243_u8,253_u8,153_u8,119_u8,3_u8,25_u8,121_u8];
_3.0.3 = [(-239964151_i32),266517466_i32,688352053_i32,144949037_i32,(-1971597196_i32),275084026_i32];
_5 = 286303366685539811600590151086900434795_u128 as isize;
_3.0.1 = 43238440758762003065369571015928439031_u128 as f32;
_7 = _5;
_5 = _2;
_10.0.3 = 10224345666047868037_u64 as i32;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211454 => bb6,
_ => bb5
}
}
bb10 = {
_2 = !_5;
_15 = '\u{1652}';
_10.0.1 = core::ptr::addr_of!(_14);
_10.0.1 = core::ptr::addr_of!(_14);
Call(_10.0.0 = fn9(Move(_3), _5, _1, _5, _1, _6, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = [120_u8,194_u8,59_u8,139_u8,32_u8,112_u8,142_u8];
_3.0.0 = !_10.0.2;
RET = [141_u8,93_u8,161_u8,237_u8,199_u8,44_u8,214_u8];
_10.0.1 = core::ptr::addr_of!(_14);
_9 = !_7;
_11 = !_7;
_6 = 11839908096816234512_usize as u32;
RET = [24_u8,252_u8,171_u8,6_u8,134_u8,227_u8,218_u8];
_10.0.0 = !(-101_i8);
_10.0.3 = 211_u8 as i32;
Goto(bb12)
}
bb12 = {
_18 = [_6];
_2 = !_5;
_3.1 = [_2,_5,_2,_12,_5];
_18 = [_6];
_3.0.2 = core::ptr::addr_of_mut!(_6);
_3.1 = [_9,_2,_2,_11,_5];
Goto(bb13)
}
bb13 = {
_3.0.3 = [_10.0.3,_10.0.3,_10.0.3,_10.0.3,_10.0.3,_10.0.3];
_10.0.3 = 167_u8 as i32;
_3.1 = [_7,_2,_5,_9,_5];
_18 = [_6];
_7 = _12;
_11 = _6 as isize;
_4 = (-102831068778156893571052201880612074564_i128);
_10.0.0 = _10.0.2 as i8;
_2 = _12;
_11 = _7 * _12;
_10.0.1 = core::ptr::addr_of!(_20);
RET = [2_u8,16_u8,56_u8,211_u8,146_u8,178_u8,119_u8];
RET = [182_u8,99_u8,121_u8,0_u8,165_u8,215_u8,14_u8];
_3.1 = [_11,_5,_5,_5,_9];
_18 = [_6];
_3.0.1 = _5 as f32;
_18 = [_6];
_3.0.0 = _10.0.2;
_17 = [227_u8,235_u8,48_u8,68_u8,247_u8,123_u8,228_u8];
_1 = [5890200207985650695_i64,(-4967221223892309301_i64),2638781569816738845_i64,(-6196002216930499979_i64),(-1865148934687467644_i64),(-8711417666733647108_i64),(-3110203872363661721_i64)];
_26 = 81_u8;
_13 = core::ptr::addr_of_mut!(_25);
_28 = _15;
Goto(bb14)
}
bb14 = {
_28 = _15;
_3.2 = _3.0.1;
_24 = core::ptr::addr_of!(_6);
(*_24) = 2757163028_u32 | 1860206212_u32;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(8_usize, 7_usize, Move(_7), 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(8_usize, 18_usize, Move(_18), 17_usize, Move(_17), 26_usize, Move(_26), 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32),mut _2: isize,mut _3: [i64; 7],mut _4: isize,mut _5: [i64; 7],mut _6: u32,mut _7: [u8; 7]) -> i8 {
mir! {
type RET = i8;
let _8: [i64; 7];
let _9: f64;
let _10: char;
let _11: *mut u32;
let _12: &'static &'static *const *mut Adt20;
let _13: f32;
let _14: f64;
let _15: [i64; 1];
let _16: i8;
let _17: Adt67;
let _18: [i64; 1];
let _19: i32;
let _20: [i64; 4];
let _21: [u8; 7];
let _22: Adt67;
let _23: i64;
let _24: ([u32; 1], Adt31, *const *mut Adt20, *const (f32,));
let _25: char;
let _26: f32;
let _27: f32;
let _28: (u32,);
let _29: u8;
let _30: [i16; 2];
let _31: (i16, f32, *mut u32, [i32; 6]);
let _32: char;
let _33: isize;
let _34: u64;
let _35: i8;
let _36: ((i8, *const f64, i16, i32),);
let _37: f32;
let _38: &'static ([i16; 2], &'static *const u32);
let _39: char;
let _40: f64;
let _41: [i16; 2];
let _42: &'static &'static *const *mut Adt20;
let _43: *const i16;
let _44: isize;
let _45: bool;
let _46: bool;
let _47: Adt48;
let _48: u16;
let _49: isize;
let _50: [i32; 6];
let _51: ();
let _52: ();
{
_1.0.3 = [1642618328_i32,(-1114725512_i32),(-806169668_i32),451408192_i32,(-483611578_i32),(-1653856922_i32)];
RET = 46_i8 ^ 54_i8;
_1.0.0 = 6132_i16 & (-1198_i16);
_8 = [(-1164377456414123694_i64),(-5643278516171980114_i64),7051416795651258207_i64,7522732307055412608_i64,7430736628198256112_i64,7852387182302851038_i64,4965938533452364851_i64];
_1.0.3 = [(-533197116_i32),1854234644_i32,(-1896692960_i32),1891517378_i32,1878955569_i32,(-1676658232_i32)];
_1.0.3 = [81885333_i32,503640102_i32,(-1991068365_i32),293718688_i32,(-732859392_i32),611406543_i32];
_6 = 156673242_u32 >> RET;
_5 = [2526616955809913236_i64,226459360930751685_i64,7649254654401869804_i64,1662412373877938485_i64,(-700197890569483195_i64),6923496059815332051_i64,(-4987638714808119258_i64)];
_1.1 = [_2,_2,_2,_2,_4];
_9 = 122303116505942309824180552997335680150_u128 as f64;
_2 = _4;
_10 = '\u{10dc5c}';
_1.0.2 = core::ptr::addr_of_mut!(_6);
_1.1 = [_2,_2,_4,_4,_2];
_1.0.1 = _1.2 * _1.2;
_1.0.3 = [6966858_i32,(-769422135_i32),1879901092_i32,1629897022_i32,1306522921_i32,(-1645587268_i32)];
_11 = core::ptr::addr_of_mut!(_6);
Goto(bb1)
}
bb1 = {
_10 = '\u{cdb50}';
_5 = [(-4127002212362316680_i64),(-4968033396297827343_i64),(-3549635802939478548_i64),4477665332175972495_i64,5266847391608154024_i64,3146113705638836591_i64,(-9006506067520880204_i64)];
RET = (-10_i8) - 59_i8;
_5 = _8;
_8 = [4933754294213205094_i64,(-1679786163468411498_i64),208808307794193394_i64,(-8514083978532291872_i64),3286685865016354297_i64,6681090212079252367_i64,2067729995877961437_i64];
_10 = '\u{e77d9}';
(*_11) = !3022110286_u32;
_1.1 = [_4,_2,_4,_4,_2];
_1.0.1 = _1.2 * _1.2;
_1.0.1 = _1.2 - _1.2;
_1.0.1 = 11903701596206509952_u64 as f32;
_2 = _4;
_13 = -_1.2;
_1.0.2 = core::ptr::addr_of_mut!(_6);
(*_11) = !499043029_u32;
_1.0.2 = Move(_11);
RET = !(-32_i8);
_1.0.3 = [(-1132505229_i32),825072466_i32,(-360734982_i32),(-2143361885_i32),136405624_i32,319118256_i32];
_13 = -_1.2;
_6 = !2762041525_u32;
Goto(bb2)
}
bb2 = {
_10 = '\u{bf56b}';
_1.0.2 = core::ptr::addr_of_mut!(_6);
_6 = 101_u8 as u32;
_4 = _2 << _1.0.0;
_14 = _9;
_6 = 604128282_u32 * 12011222_u32;
_14 = _9 * _9;
_1.2 = 259539727330231283854164562869633397407_u128 as f32;
_3 = _5;
_13 = _1.2 - _1.0.1;
_13 = _1.2;
Goto(bb3)
}
bb3 = {
_1.0.0 = (-32701_i16) * 8749_i16;
_2 = _4 + _4;
_4 = 1368795683_i32 as isize;
_11 = Move(_1.0.2);
_1.0.2 = core::ptr::addr_of_mut!(_6);
_1.0.0 = (-13994_i16) << _2;
Call(_15 = fn10(_2, _5, Move(_1.0), _5, _9, _13, _1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = [8236269343848607278_i64,(-7914310494463365569_i64),(-8362914930847471213_i64),2166924171747676469_i64,(-4423929572938063505_i64),(-6126631802005559171_i64),6528740995187258125_i64];
_1.2 = _13 + _13;
_1.2 = _13;
RET = (-107_i8) - 105_i8;
RET = 80_i8;
_1.0.2 = Move(_11);
Goto(bb5)
}
bb5 = {
_1.0.3 = [201126067_i32,1899624078_i32,(-1345414817_i32),1003623209_i32,346786246_i32,1728471866_i32];
_2 = _4 ^ _4;
_1.2 = -_13;
_1.0.1 = _1.2 * _1.2;
_19 = 2121658296_i32;
_9 = _14 - _14;
Goto(bb6)
}
bb6 = {
_2 = _4 - _4;
RET = -99_i8;
_2 = _13 as isize;
_10 = '\u{21bbc}';
_13 = _1.2;
_20 = [(-9000963976595786960_i64),668186370947309329_i64,3714191862665809027_i64,8148630656612596346_i64];
_14 = _19 as f64;
Goto(bb7)
}
bb7 = {
_20 = [(-8504108432550272045_i64),627855242938063531_i64,2001234225115396258_i64,374776725086550867_i64];
_1.1 = [_2,_2,_2,_4,_2];
_13 = -_1.2;
_10 = '\u{adc58}';
RET = 8273420462893079757_i64 as i8;
RET = -(-63_i8);
_7 = [180_u8,134_u8,114_u8,89_u8,134_u8,199_u8,45_u8];
_1.0.1 = _13 * _1.2;
_16 = RET | RET;
_11 = core::ptr::addr_of_mut!(_6);
_18 = [7136897951979050733_i64];
_20 = [7872150446896535041_i64,3242452495665959129_i64,7945621641026928608_i64,(-3524596753072514878_i64)];
_19 = (-849875240_i32) - 562928725_i32;
_1.0.0 = -(-28176_i16);
_21 = [108_u8,181_u8,120_u8,80_u8,225_u8,153_u8,196_u8];
_21 = _7;
_23 = _13 as i64;
_18 = [_23];
_20 = [_23,_23,_23,_23];
_3 = _5;
_2 = -_4;
_10 = '\u{55026}';
_10 = '\u{10b9ef}';
_23 = -1392100925606421536_i64;
Goto(bb8)
}
bb8 = {
_7 = [239_u8,179_u8,193_u8,168_u8,50_u8,202_u8,7_u8];
_19 = (-998953658_i32);
_24.0 = [_6];
_25 = _10;
_6 = !4191163466_u32;
_15 = _18;
RET = _16 >> _19;
_27 = _16 as f32;
_9 = _14;
RET = -_16;
_1.0.3 = [_19,_19,_19,_19,_19,_19];
_20 = [_23,_23,_23,_23];
_1.0.1 = _27 - _1.2;
RET = _4 as i8;
_26 = _1.0.1 * _1.0.1;
_7 = _21;
_26 = _1.0.1;
_7 = [246_u8,198_u8,127_u8,112_u8,211_u8,142_u8,160_u8];
_15 = [_23];
_1.0.1 = _27;
_8 = [_23,_23,_23,_23,_23,_23,_23];
_20 = [_23,_23,_23,_23];
_18 = [_23];
Goto(bb9)
}
bb9 = {
_5 = _3;
_21 = [203_u8,153_u8,118_u8,65_u8,183_u8,130_u8,72_u8];
_9 = _6 as f64;
_28.0 = (*_11);
_1.0.1 = -_13;
_16 = (*_11) as i8;
_29 = 140_u8 - 157_u8;
_6 = 246060154032509857844095399363174945689_u128 as u32;
_1.2 = _2 as f32;
(*_11) = _28.0 << RET;
_1.0.3 = [_19,_19,_19,_19,_19,_19];
_1.0.0 = 10064_i16 * (-28893_i16);
_31.3 = [_19,_19,_19,_19,_19,_19];
RET = -_16;
_1.0.2 = Move(_11);
_31.2 = Move(_1.0.2);
_15 = [_23];
_15 = [_23];
_1.1 = [_2,_4,_4,_4,_4];
_3 = _8;
_1.0.2 = core::ptr::addr_of_mut!(_6);
_23 = -8531449944170935637_i64;
_1.1 = [_2,_4,_2,_4,_4];
match _19 {
340282366920938463463374607430769257798 => bb11,
_ => bb10
}
}
bb10 = {
_10 = '\u{cdb50}';
_5 = [(-4127002212362316680_i64),(-4968033396297827343_i64),(-3549635802939478548_i64),4477665332175972495_i64,5266847391608154024_i64,3146113705638836591_i64,(-9006506067520880204_i64)];
RET = (-10_i8) - 59_i8;
_5 = _8;
_8 = [4933754294213205094_i64,(-1679786163468411498_i64),208808307794193394_i64,(-8514083978532291872_i64),3286685865016354297_i64,6681090212079252367_i64,2067729995877961437_i64];
_10 = '\u{e77d9}';
(*_11) = !3022110286_u32;
_1.1 = [_4,_2,_4,_4,_2];
_1.0.1 = _1.2 * _1.2;
_1.0.1 = _1.2 - _1.2;
_1.0.1 = 11903701596206509952_u64 as f32;
_2 = _4;
_13 = -_1.2;
_1.0.2 = core::ptr::addr_of_mut!(_6);
(*_11) = !499043029_u32;
_1.0.2 = Move(_11);
RET = !(-32_i8);
_1.0.3 = [(-1132505229_i32),825072466_i32,(-360734982_i32),(-2143361885_i32),136405624_i32,319118256_i32];
_13 = -_1.2;
_6 = !2762041525_u32;
Goto(bb2)
}
bb11 = {
_31.3 = [_19,_19,_19,_19,_19,_19];
_31.2 = core::ptr::addr_of_mut!(_6);
_1.0.0 = !20075_i16;
_31.0 = _16 as i16;
_32 = _25;
_8 = _3;
_10 = _25;
_3 = [_23,_23,_23,_23,_23,_23,_23];
_13 = _1.2;
_34 = RET as u64;
_3 = [_23,_23,_23,_23,_23,_23,_23];
_19 = -162947508_i32;
_16 = RET;
Goto(bb12)
}
bb12 = {
_37 = _19 as f32;
_28.0 = _2 as u32;
_1.0.0 = _26 as i16;
_33 = _27 as isize;
_18 = [_23];
_16 = -RET;
_28 = (_6,);
_1.2 = _27;
_16 = RET;
_20 = [_23,_23,_23,_23];
_36.0.1 = core::ptr::addr_of!(_14);
_35 = RET;
_36.0.3 = _16 as i32;
_23 = !(-7474660312288444497_i64);
_31 = Move(_1.0);
_31.1 = _37 + _26;
_19 = _36.0.3;
Goto(bb13)
}
bb13 = {
_6 = !_28.0;
_13 = _33 as f32;
_15 = [_23];
_41 = [_31.0,_31.0];
_16 = _19 as i8;
_36.0.0 = false as i8;
_7 = _21;
RET = 6556629806400979657_usize as i8;
_9 = -_14;
_36.0.0 = -RET;
_39 = _25;
_1.0.0 = _31.0;
_44 = _2 & _4;
_2 = _26 as isize;
_13 = -_26;
_40 = _9;
_9 = -_40;
Goto(bb14)
}
bb14 = {
_28 = (_6,);
_1.0.3 = _31.3;
_31.2 = core::ptr::addr_of_mut!(_28.0);
_31.3 = [_19,_19,_19,_36.0.3,_36.0.3,_19];
_8 = [_23,_23,_23,_23,_23,_23,_23];
_40 = -_14;
_8 = [_23,_23,_23,_23,_23,_23,_23];
_10 = _39;
_40 = -_9;
_25 = _39;
_30 = [_1.0.0,_1.0.0];
_31.1 = _31.0 as f32;
_31.0 = _1.0.0;
_14 = _40;
_31.0 = _39 as i16;
_34 = !2416424426364582295_u64;
_1.0 = Move(_31);
_31.2 = core::ptr::addr_of_mut!(_6);
_24.0 = [_28.0];
_34 = 12664400246386700383_u64;
_49 = !_4;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(9_usize, 23_usize, Move(_23), 49_usize, Move(_49), 39_usize, Move(_39), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(9_usize, 16_usize, Move(_16), 19_usize, Move(_19), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(9_usize, 8_usize, Move(_8), 25_usize, Move(_25), 33_usize, Move(_33), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(9_usize, 20_usize, Move(_20), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: [i64; 7],mut _3: (i16, f32, *mut u32, [i32; 6]),mut _4: [i64; 7],mut _5: f64,mut _6: f32,mut _7: [isize; 5]) -> [i64; 1] {
mir! {
type RET = [i64; 1];
let _8: *mut Adt31;
let _9: ([i16; 2],);
let _10: f32;
let _11: &'static isize;
let _12: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _13: f64;
let _14: Adt28;
let _15: i8;
let _16: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _17: (u32,);
let _18: bool;
let _19: i128;
let _20: [i32; 5];
let _21: u32;
let _22: bool;
let _23: &'static *const u32;
let _24: usize;
let _25: (i16, f32, *mut u32, [i32; 6]);
let _26: *mut [i16; 2];
let _27: &'static &'static isize;
let _28: (i16, i16, [i16; 2]);
let _29: char;
let _30: u64;
let _31: ();
let _32: ();
{
RET = [(-6097227541877758420_i64)];
_3.0 = !17268_i16;
_3.0 = !6382_i16;
_3.0 = (-26211_i16);
RET = [5693274527257632112_i64];
RET = [7867351441258224273_i64];
_3.3 = [1120381870_i32,(-1408007042_i32),(-1448019956_i32),(-349037239_i32),(-465216102_i32),1379507134_i32];
_2 = [519540929843698760_i64,5457636343379369154_i64,7436919351826095877_i64,(-9189781440882180460_i64),(-7421878662698421009_i64),2944015305552846766_i64,(-8024065886611420527_i64)];
_5 = _3.0 as f64;
_5 = 10561_u16 as f64;
_1 = 9223372036854775807_isize;
_3.0 = 16084_i16 ^ 23981_i16;
_1 = !9223372036854775807_isize;
_3.3 = [(-1720283513_i32),(-1373433480_i32),(-1301391588_i32),357574867_i32,1465368792_i32,687334152_i32];
_10 = -_3.1;
_12.1 = !_1;
_12.1 = (-84_i8) as isize;
_1 = 118_u8 as isize;
_10 = 17094094573698827084369665399163160055_i128 as f32;
RET = [2771124542054059744_i64];
_3.0 = -29458_i16;
_12.2.3 = [3768268678_u32];
_12.2.2 = 154303744_i32 + 1273899909_i32;
Call(_8 = fn11(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = [180732800863908937_i64,5780825777874986877_i64,3336035560893375165_i64,6836364217840054774_i64,2978095498276097628_i64,443555506513306599_i64,4626799394767946219_i64];
_12.2.1 = '\u{9a6e6}';
_12.2.0 = [(-10_i8),(-62_i8),102_i8];
_3.0 = 35083349751328593726771340970791534029_i128 as i16;
_16.0 = _3.3;
_16 = (_3.3, _12.1, _12.2);
_12.0 = _3.3;
_7 = [_1,_1,_1,_12.1,_16.1];
_12.0 = [_12.2.2,_16.2.2,_16.2.2,_12.2.2,_16.2.2,_12.2.2];
_18 = true ^ true;
_16.2.1 = _12.2.1;
_17 = (3000172154_u32,);
_2 = _4;
_17 = (361201570_u32,);
_12.2.0 = _16.2.0;
_12.2.3 = _16.2.3;
_17 = (3678050333_u32,);
_16.2.1 = _12.2.1;
RET = [2047054075746482810_i64];
_16.2.0 = [(-56_i8),14_i8,(-123_i8)];
_12.2.1 = _16.2.1;
Goto(bb2)
}
bb2 = {
_11 = &_12.1;
_9.0 = [_3.0,_3.0];
match _17.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
3678050333 => bb10,
_ => bb9
}
}
bb3 = {
_2 = [180732800863908937_i64,5780825777874986877_i64,3336035560893375165_i64,6836364217840054774_i64,2978095498276097628_i64,443555506513306599_i64,4626799394767946219_i64];
_12.2.1 = '\u{9a6e6}';
_12.2.0 = [(-10_i8),(-62_i8),102_i8];
_3.0 = 35083349751328593726771340970791534029_i128 as i16;
_16.0 = _3.3;
_16 = (_3.3, _12.1, _12.2);
_12.0 = _3.3;
_7 = [_1,_1,_1,_12.1,_16.1];
_12.0 = [_12.2.2,_16.2.2,_16.2.2,_12.2.2,_16.2.2,_12.2.2];
_18 = true ^ true;
_16.2.1 = _12.2.1;
_17 = (3000172154_u32,);
_2 = _4;
_17 = (361201570_u32,);
_12.2.0 = _16.2.0;
_12.2.3 = _16.2.3;
_17 = (3678050333_u32,);
_16.2.1 = _12.2.1;
RET = [2047054075746482810_i64];
_16.2.0 = [(-56_i8),14_i8,(-123_i8)];
_12.2.1 = _16.2.1;
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
_13 = _5 + _5;
_2 = [(-3675456977676745167_i64),(-2489415555605925055_i64),1005244102705322173_i64,(-4421621675093485561_i64),(-373535712361418970_i64),(-6993605192193687434_i64),5576248513965909170_i64];
_12.2 = (_16.2.0, _16.2.1, _16.2.2, _16.2.3);
_17.0 = !2961775306_u32;
_3.3 = _16.0;
_19 = -25232146315991290858384556833698401443_i128;
_16.2.3 = [_17.0];
_16.2.3 = [_17.0];
_17.0 = 1064779700_u32;
_16.2.3 = [_17.0];
_16.2.2 = -_12.2.2;
_12.2.0 = [(-78_i8),64_i8,74_i8];
_14 = Adt28::Variant1 { fld0: _12.2.3,fld1: _17.0,fld2: 140_u8,fld3: (-37_i8),fld4: _3.0 };
place!(Field::<i16>(Variant(_14, 1), 4)) = _3.0;
_2 = [(-8595282325408621927_i64),(-4363998060806965698_i64),(-1550423626317739764_i64),2752573075795988228_i64,(-712589525595149029_i64),(-1113813474409163660_i64),(-8844913415195367401_i64)];
_3.3 = _16.0;
_3.3 = [_16.2.2,_12.2.2,_12.2.2,_16.2.2,_12.2.2,_16.2.2];
_16.0 = [_16.2.2,_16.2.2,_12.2.2,_12.2.2,_12.2.2,_16.2.2];
place!(Field::<u8>(Variant(_14, 1), 2)) = 112_u8 | 133_u8;
place!(Field::<u8>(Variant(_14, 1), 2)) = !147_u8;
Goto(bb11)
}
bb11 = {
_17 = (Field::<u32>(Variant(_14, 1), 1),);
_4 = _2;
_6 = _16.2.2 as f32;
_12 = _16;
place!(Field::<[u32; 1]>(Variant(_14, 1), 0)) = _16.2.3;
Call(_14 = fn12(_16.2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = !110299385351821345739417571968086077846_i128;
_3.1 = _6 * _10;
_22 = !_18;
_12.2.0 = _16.2.0;
place!(Field::<u32>(Variant(_14, 1), 1)) = _17.0 >> _17.0;
RET = [4200436579291381200_i64];
_12.2.0 = [Field::<i8>(Variant(_14, 1), 3),Field::<i8>(Variant(_14, 1), 3),Field::<i8>(Variant(_14, 1), 3)];
_21 = _13 as u32;
_9.0 = [Field::<i16>(Variant(_14, 1), 4),Field::<i16>(Variant(_14, 1), 4)];
_26 = core::ptr::addr_of_mut!(_9.0);
place!(Field::<i8>(Variant(_14, 1), 3)) = 67_i8 + 42_i8;
_19 = 18451937828879922961004474534557922526_i128;
_16.2.2 = _12.2.2 | _12.2.2;
_25.1 = _3.1;
_10 = _3.1;
_7 = [_12.1,_1,_1,_1,_16.1];
_21 = _17.0;
_29 = _12.2.1;
match _17.0 {
0 => bb13,
1 => bb14,
1064779700 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_2 = [180732800863908937_i64,5780825777874986877_i64,3336035560893375165_i64,6836364217840054774_i64,2978095498276097628_i64,443555506513306599_i64,4626799394767946219_i64];
_12.2.1 = '\u{9a6e6}';
_12.2.0 = [(-10_i8),(-62_i8),102_i8];
_3.0 = 35083349751328593726771340970791534029_i128 as i16;
_16.0 = _3.3;
_16 = (_3.3, _12.1, _12.2);
_12.0 = _3.3;
_7 = [_1,_1,_1,_12.1,_16.1];
_12.0 = [_12.2.2,_16.2.2,_16.2.2,_12.2.2,_16.2.2,_12.2.2];
_18 = true ^ true;
_16.2.1 = _12.2.1;
_17 = (3000172154_u32,);
_2 = _4;
_17 = (361201570_u32,);
_12.2.0 = _16.2.0;
_12.2.3 = _16.2.3;
_17 = (3678050333_u32,);
_16.2.1 = _12.2.1;
RET = [2047054075746482810_i64];
_16.2.0 = [(-56_i8),14_i8,(-123_i8)];
_12.2.1 = _16.2.1;
Goto(bb2)
}
bb15 = {
_11 = &_12.1;
_9.0 = [_3.0,_3.0];
match _17.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
3678050333 => bb10,
_ => bb9
}
}
bb16 = {
_17.0 = !Field::<u32>(Variant(_14, 1), 1);
_28.0 = 2_usize as i16;
_11 = &_12.1;
_10 = Field::<i8>(Variant(_14, 1), 3) as f32;
_4 = _2;
_27 = &_11;
_10 = -_3.1;
place!(Field::<i8>(Variant(_14, 1), 3)) = _22 as i8;
_5 = _13 * _13;
_24 = 0_usize;
_3.0 = !Field::<i16>(Variant(_14, 1), 4);
RET = [_4[_24]];
place!(Field::<[u32; 1]>(Variant(_14, 1), 0))[_24] = Field::<u32>(Variant(_14, 1), 1);
_16.2 = (_12.2.0, _12.2.1, _3.3[_24], Field::<[u32; 1]>(Variant(_14, 1), 0));
_12.2.0[_24] = -_16.2.0[_24];
_7 = [(*_11),(*_11),_16.1,(*_11),_16.1];
_12.2.2 = _22 as i32;
_12.2 = (_16.2.0, _16.2.1, _3.3[_24], Field::<[u32; 1]>(Variant(_14, 1), 0));
_4[_24] = !RET[_24];
_3.1 = -_10;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(10_usize, 29_usize, Move(_29), 4_usize, Move(_4), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(10_usize, 9_usize, Move(_9), 1_usize, Move(_1), 17_usize, Move(_17), 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> *mut Adt31 {
mir! {
type RET = *mut Adt31;
let _1: char;
let _2: (&'static [isize; 5],);
let _3: (u32,);
let _4: usize;
let _5: ([u32; 1], Adt31, *const *mut Adt20, *const (f32,));
let _6: u128;
let _7: isize;
let _8: isize;
let _9: ([i16; 2], &'static *const u32);
let _10: bool;
let _11: (*mut Adt31,);
let _12: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32);
let _13: u64;
let _14: f32;
let _15: [i64; 1];
let _16: i16;
let _17: u64;
let _18: u64;
let _19: [i16; 2];
let _20: *mut *mut Adt31;
let _21: [u8; 4];
let _22: *const u32;
let _23: f64;
let _24: [u8; 4];
let _25: bool;
let _26: ();
let _27: ();
{
_1 = '\u{2f6fc}';
_1 = '\u{10ec36}';
_1 = '\u{e375e}';
_3.0 = !2966773821_u32;
_1 = '\u{92b42}';
_4 = !1231366857854765089_usize;
_3.0 = 865909066_u32;
_3 = (1334846870_u32,);
_3 = (1937883938_u32,);
_5.0 = [_3.0];
_5.0 = [_3.0];
_4 = 0_usize;
_3.0 = !_5.0[_4];
Goto(bb1)
}
bb1 = {
_1 = '\u{a5c2a}';
_3.0 = _5.0[_4] - _5.0[_4];
RET = core::ptr::addr_of_mut!(_5.1);
Goto(bb2)
}
bb2 = {
_1 = '\u{a32db}';
_5.0 = [_3.0];
_3.0 = 42_i8 as u32;
RET = core::ptr::addr_of_mut!(_5.1);
_3.0 = _5.0[_4];
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
_5.0[_4] = _3.0;
_5.0 = [_3.0];
_4 = 14_i8 as usize;
RET = core::ptr::addr_of_mut!(_5.1);
_4 = 7_usize;
_3 = (3330113259_u32,);
_1 = '\u{a50df}';
_3 = (2896299562_u32,);
RET = core::ptr::addr_of_mut!((*RET));
_6 = 286351073479971301034701172173841289199_u128 << _4;
_3 = (2016986994_u32,);
_8 = (-2626136708028757359_i64) as isize;
_5.0 = [_3.0];
RET = core::ptr::addr_of_mut!((*RET));
_6 = 169898187140638578851910597424542579721_u128;
_7 = _8;
RET = core::ptr::addr_of_mut!((*RET));
match _3.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2016986994 => bb9,
_ => bb8
}
}
bb3 = {
_1 = '\u{a5c2a}';
_3.0 = _5.0[_4] - _5.0[_4];
RET = core::ptr::addr_of_mut!(_5.1);
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
_1 = '\u{42bb2}';
_6 = 279935552411271716050624059106087724659_u128;
_7 = _8;
_3.0 = !1196653742_u32;
_3 = (1670979750_u32,);
_7 = _1 as isize;
RET = core::ptr::addr_of_mut!((*RET));
_5.0 = [_3.0];
_10 = false;
_9.0 = [(-8081_i16),(-31770_i16)];
_3.0 = !3229390683_u32;
RET = core::ptr::addr_of_mut!((*RET));
_3.0 = _6 as u32;
_11.0 = core::ptr::addr_of_mut!((*RET));
_11.0 = core::ptr::addr_of_mut!((*RET));
_9.0 = [(-1103_i16),(-22060_i16)];
_9.0 = [17332_i16,(-2786_i16)];
_7 = (-142235965563850311975218398349626695951_i128) as isize;
_3 = (2342726396_u32,);
_7 = _8;
_7 = -_8;
_12.1 = [_8,_8,_7,_7,_8];
match _6 {
279935552411271716050624059106087724659 => bb10,
_ => bb2
}
}
bb10 = {
_8 = 120_u16 as isize;
_12.2 = _3.0 as f32;
_13 = _1 as u64;
_2.0 = &_12.1;
_12.0.1 = -_12.2;
_4 = 13444573684141163358_usize;
_15 = [3300033051174750458_i64];
_15 = [1786783163430239669_i64];
_12.1 = [_8,_8,_7,_8,_7];
_2.0 = &_12.1;
_12.0.2 = core::ptr::addr_of_mut!(_3.0);
_14 = 94_i8 as f32;
_12.0.0 = -8970_i16;
_3.0 = !1440814709_u32;
_11.0 = core::ptr::addr_of_mut!((*RET));
_3 = (3447342140_u32,);
_9.0 = [_12.0.0,_12.0.0];
_7 = _8 & _8;
_6 = _1 as u128;
_9.1 = &_22;
_2.0 = &_12.1;
_22 = core::ptr::addr_of!(_3.0);
match (*_22) {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb11,
4 => bb12,
3447342140 => bb14,
_ => bb13
}
}
bb11 = {
_1 = '\u{a32db}';
_5.0 = [_3.0];
_3.0 = 42_i8 as u32;
RET = core::ptr::addr_of_mut!(_5.1);
_3.0 = _5.0[_4];
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
_5.0[_4] = _3.0;
_5.0 = [_3.0];
_4 = 14_i8 as usize;
RET = core::ptr::addr_of_mut!(_5.1);
_4 = 7_usize;
_3 = (3330113259_u32,);
_1 = '\u{a50df}';
_3 = (2896299562_u32,);
RET = core::ptr::addr_of_mut!((*RET));
_6 = 286351073479971301034701172173841289199_u128 << _4;
_3 = (2016986994_u32,);
_8 = (-2626136708028757359_i64) as isize;
_5.0 = [_3.0];
RET = core::ptr::addr_of_mut!((*RET));
_6 = 169898187140638578851910597424542579721_u128;
_7 = _8;
RET = core::ptr::addr_of_mut!((*RET));
match _3.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2016986994 => bb9,
_ => bb8
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_13 = 6938330395678265317_u64 | 17711211393887221173_u64;
_7 = 648690000_i32 as isize;
_12.0.3 = [1262568922_i32,(-1905734060_i32),(-1070706405_i32),(-818039522_i32),1716281890_i32,(-531040542_i32)];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(11_usize, 4_usize, Move(_4), 6_usize, Move(_6), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: ([i8; 3], char, i32, [u32; 1])) -> Adt28 {
mir! {
type RET = Adt28;
let _2: (*mut Adt31,);
let _3: Adt35;
let _4: i64;
let _5: f64;
let _6: isize;
let _7: bool;
let _8: *const (f32,);
let _9: u128;
let _10: i64;
let _11: i32;
let _12: *mut [u8; 4];
let _13: char;
let _14: &'static &'static isize;
let _15: f32;
let _16: Adt35;
let _17: bool;
let _18: bool;
let _19: [i32; 5];
let _20: isize;
let _21: [usize; 8];
let _22: &'static &'static isize;
let _23: &'static &'static isize;
let _24: [i64; 7];
let _25: i8;
let _26: u8;
let _27: char;
let _28: ();
let _29: ();
{
RET = Adt28::Variant1 { fld0: _1.3,fld1: 748511142_u32,fld2: 113_u8,fld3: (-79_i8),fld4: 17840_i16 };
place!(Field::<u32>(Variant(RET, 1), 1)) = 2143674923_u32;
place!(Field::<i8>(Variant(RET, 1), 3)) = !94_i8;
_1.3 = [Field::<u32>(Variant(RET, 1), 1)];
place!(Field::<u8>(Variant(RET, 1), 2)) = !35_u8;
place!(Field::<i16>(Variant(RET, 1), 4)) = (-16702_i16);
place!(Field::<i16>(Variant(RET, 1), 4)) = -(-5729_i16);
SetDiscriminant(RET, 0);
_3 = Adt35 { fld0: 16909578248122846490_u64 };
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).2 = 23942_i16;
place!(Field::<u16>(Variant(RET, 0), 2)) = 11491_u16;
place!(Field::<u16>(Variant(RET, 0), 2)) = 14467_u16;
_1.0 = [(-125_i8),(-28_i8),(-2_i8)];
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).3 = 4071756483_u32 as i32;
_1.2 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).2 as i32;
place!(Field::<u16>(Variant(RET, 0), 2)) = 7813_u16;
place!(Field::<u16>(Variant(RET, 0), 2)) = 39185_u16;
place!(Field::<[isize; 5]>(Variant(RET, 0), 1)) = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-33_isize),9223372036854775807_isize,(-28_isize)];
place!(Field::<usize>(Variant(RET, 0), 3)) = 10004023412036989111_usize;
_1.2 = !Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
place!(Field::<f32>(Variant(RET, 0), 6)) = _3.fld0 as f32;
place!(Field::<u16>(Variant(RET, 0), 2)) = !59978_u16;
_1.2 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3 >> Field::<u16>(Variant(RET, 0), 2);
_1.1 = '\u{f442c}';
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).3 = _1.2;
_5 = 14736447081239005246459357888400222332_i128 as f64;
Goto(bb1)
}
bb1 = {
_1.3 = [2207211418_u32];
_4 = -(-730814252842793648_i64);
_1.3 = [1003794919_u32];
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
_4 = 151383084714599894530213421483913994405_u128 as i64;
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).0 = (-110_i8) + (-126_i8);
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).2 = false as i16;
_1.2 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3 | Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
place!(Field::<bool>(Variant(RET, 0), 0)) = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3 <= _1.2;
_1.1 = '\u{263bc}';
_7 = _1.1 <= _1.1;
place!(Field::<usize>(Variant(RET, 0), 3)) = 5_usize;
place!(Field::<bool>(Variant(RET, 0), 0)) = _7;
place!(Field::<f32>(Variant(RET, 0), 6)) = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).2 as f32;
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
_3 = Adt35 { fld0: 7728050674655124124_u64 };
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
place!(Field::<u16>(Variant(RET, 0), 2)) = 57657_u16;
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
place!(Field::<[isize; 5]>(Variant(RET, 0), 1)) = [60_isize,9223372036854775807_isize,(-66_isize),(-9223372036854775808_isize),65_isize];
place!(Field::<usize>(Variant(RET, 0), 3)) = !0_usize;
_1.2 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3 - Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
_1.3 = [2246304724_u32];
Call(place!(Field::<[isize; 5]>(Variant(RET, 0), 1)) = fn13(Move(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).1), _3.fld0, Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).0, _1.1, _1.0, _1.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).3 = _1.2 ^ _1.2;
_9 = _1.2 as u128;
place!(Field::<[i32; 6]>(Variant(RET, 0), 5)) = [Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,_1.2];
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).2 = -(-26759_i16);
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).0 = (-54_i8);
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
place!(Field::<usize>(Variant(RET, 0), 3)) = !3_usize;
place!(Field::<[i32; 6]>(Variant(RET, 0), 5)) = [Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,_1.2,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,_1.2,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3,Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3];
_1.1 = '\u{8db29}';
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
_1.2 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
SetDiscriminant(RET, 0);
place!(Field::<u16>(Variant(RET, 0), 2)) = 41429_u16;
place!(Field::<f32>(Variant(RET, 0), 6)) = (-88_isize) as f32;
place!(Field::<[i32; 6]>(Variant(RET, 0), 5)) = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
place!(Field::<[i32; 6]>(Variant(RET, 0), 5)) = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).3 = _1.2;
place!(Field::<bool>(Variant(RET, 0), 0)) = _1.1 == _1.1;
Goto(bb3)
}
bb3 = {
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).1 = core::ptr::addr_of!(_5);
place!(Field::<usize>(Variant(RET, 0), 3)) = 4_usize * 2_usize;
_7 = Field::<bool>(Variant(RET, 0), 0);
place!(Field::<u16>(Variant(RET, 0), 2)) = 66_u8 as u16;
place!(Field::<f32>(Variant(RET, 0), 6)) = 24122769386423241982888712043508021698_i128 as f32;
place!(Field::<usize>(Variant(RET, 0), 3)) = 6_usize >> Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
_4 = Field::<f32>(Variant(RET, 0), 6) as i64;
place!(Field::<[isize; 5]>(Variant(RET, 0), 1)) = [(-18_isize),106_isize,(-9223372036854775808_isize),(-89_isize),(-9223372036854775808_isize)];
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).0 = !115_i8;
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).2 = (-3057_i16);
_1.1 = '\u{326db}';
_1.3 = [629084050_u32];
place!(Field::<f32>(Variant(RET, 0), 6)) = _4 as f32;
_9 = 42411490318861912216617068188775473062_u128;
_11 = Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4).3;
_4 = 8508992163491092559_i64 * 1300495830594562825_i64;
Goto(bb4)
}
bb4 = {
_5 = _4 as f64;
place!(Field::<(i8, *const f64, i16, i32)>(Variant(RET, 0), 4)).3 = _11 | _1.2;
RET = Adt28::Variant1 { fld0: _1.3,fld1: 2758221498_u32,fld2: 2_u8,fld3: 2_i8,fld4: (-3563_i16) };
_10 = (-127_i8) as i64;
place!(Field::<[u32; 1]>(Variant(RET, 1), 0)) = [4214433154_u32];
_15 = 47649_u16 as f32;
place!(Field::<u32>(Variant(RET, 1), 1)) = 17817_i16 as u32;
place!(Field::<u8>(Variant(RET, 1), 2)) = !17_u8;
_18 = _7;
_5 = 1_usize as f64;
_3 = Adt35 { fld0: 12068902383141738688_u64 };
RET = Adt28::Variant1 { fld0: _1.3,fld1: 3212784510_u32,fld2: 98_u8,fld3: 28_i8,fld4: 12075_i16 };
_7 = _18 & _18;
RET = Adt28::Variant1 { fld0: _1.3,fld1: 3076009711_u32,fld2: 44_u8,fld3: (-58_i8),fld4: 18840_i16 };
_17 = _18;
place!(Field::<i8>(Variant(RET, 1), 3)) = (-116_i8) - (-117_i8);
_1.1 = '\u{8ae26}';
place!(Field::<u32>(Variant(RET, 1), 1)) = !1283994508_u32;
_1.3 = [Field::<u32>(Variant(RET, 1), 1)];
Goto(bb5)
}
bb5 = {
_1.1 = '\u{9b59e}';
_16.fld0 = _3.fld0 | _3.fld0;
place!(Field::<i16>(Variant(RET, 1), 4)) = !(-14862_i16);
Call(place!(Field::<u8>(Variant(RET, 1), 2)) = fn17(_3.fld0, _11, _1.1, _1.2, _9, _18, _1.2, _1.2, _1, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<i8>(Variant(RET, 1), 3)) = _1.2 as i8;
_24 = [_4,_4,_10,_4,_10,_4,_10];
_6 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_1.0 = [Field::<i8>(Variant(RET, 1), 3),Field::<i8>(Variant(RET, 1), 3),Field::<i8>(Variant(RET, 1), 3)];
_6 = -9223372036854775807_isize;
_9 = 136595910381653614425513444746221748757_u128 << _11;
_20 = Field::<u32>(Variant(RET, 1), 1) as isize;
SetDiscriminant(RET, 1);
_9 = !298179366591444234142559777112331814039_u128;
place!(Field::<i16>(Variant(RET, 1), 4)) = -(-9573_i16);
Goto(bb7)
}
bb7 = {
_21 = [3_usize,1_usize,7_usize,7312327017299497084_usize,12832793728282477041_usize,16477641484518239113_usize,7_usize,12515699023972351162_usize];
_13 = _1.1;
_1.0 = [115_i8,(-77_i8),21_i8];
place!(Field::<[u32; 1]>(Variant(RET, 1), 0)) = [2906352046_u32];
place!(Field::<i16>(Variant(RET, 1), 4)) = -6512_i16;
_26 = _10 as u8;
place!(Field::<i16>(Variant(RET, 1), 4)) = 15925_i16 >> _20;
_13 = _1.1;
place!(Field::<u32>(Variant(RET, 1), 1)) = 1864554405_u32;
_3.fld0 = _16.fld0;
place!(Field::<i8>(Variant(RET, 1), 3)) = 10609_u16 as i8;
_3.fld0 = Field::<u32>(Variant(RET, 1), 1) as u64;
_7 = !_17;
_6 = _20 + _20;
_20 = _6 >> _1.2;
_24 = [_4,_4,_4,_4,_10,_10,_4];
_26 = 187_u8 ^ 111_u8;
place!(Field::<u8>(Variant(RET, 1), 2)) = _26 ^ _26;
_3 = Move(_16);
_10 = (-143916630970633273112085309903026102261_i128) as i64;
_27 = _13;
_9 = 213584760642678220562451445460682047857_u128;
place!(Field::<i8>(Variant(RET, 1), 3)) = (-1_i8) << _20;
_1.3 = Field::<[u32; 1]>(Variant(RET, 1), 0);
_15 = Field::<i16>(Variant(RET, 1), 4) as f32;
place!(Field::<i16>(Variant(RET, 1), 4)) = Field::<i8>(Variant(RET, 1), 3) as i16;
_16.fld0 = !_3.fld0;
Goto(bb8)
}
bb8 = {
Call(_28 = dump_var(12_usize, 1_usize, Move(_1), 24_usize, Move(_24), 6_usize, Move(_6), 27_usize, Move(_27)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_28 = dump_var(12_usize, 17_usize, Move(_17), 20_usize, Move(_20), 26_usize, Move(_26), 29_usize, _29), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const f64,mut _2: u64,mut _3: i8,mut _4: char,mut _5: [i8; 3],mut _6: i32) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _7: [i128; 3];
let _8: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _9: bool;
let _10: *const i16;
let _11: (&'static [isize; 5],);
let _12: &'static &'static isize;
let _13: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32);
let _14: [i8; 3];
let _15: Adt20;
let _16: [i128; 3];
let _17: &'static i16;
let _18: u32;
let _19: [i64; 7];
let _20: *mut (f32,);
let _21: isize;
let _22: isize;
let _23: [u32; 1];
let _24: i32;
let _25: i8;
let _26: ([i16; 2],);
let _27: char;
let _28: char;
let _29: *mut *const *mut Adt20;
let _30: isize;
let _31: f64;
let _32: i64;
let _33: ();
let _34: ();
{
_6 = -(-1802154992_i32);
Goto(bb1)
}
bb1 = {
_3 = 122_i8;
_3 = 60_i8;
RET = [(-94_isize),(-77_isize),127_isize,(-118_isize),(-9223372036854775808_isize)];
_5 = [_3,_3,_3];
_4 = '\u{98be5}';
_8.2.3 = [1409862546_u32];
_8.1 = 49_isize;
_8.2.2 = 1_usize as i32;
_8.2.0 = _5;
match _3 {
0 => bb2,
60 => bb4,
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
_8.1 = !(-9223372036854775808_isize);
_4 = '\u{1f46b}';
Goto(bb5)
}
bb5 = {
_8.2.3 = [1714291291_u32];
_6 = _8.2.2;
_8.0 = [_6,_8.2.2,_6,_8.2.2,_6,_8.2.2];
Goto(bb6)
}
bb6 = {
_2 = 13049742164130598240_u64 >> _6;
_8.2.0 = [_3,_3,_3];
Goto(bb7)
}
bb7 = {
_3 = !5_i8;
_2 = 11973892827842870178_u64 << _8.1;
_8.0 = [_8.2.2,_8.2.2,_6,_8.2.2,_8.2.2,_6];
Call(_8 = fn14(RET, RET, _5, RET), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7 = [(-77445020482054707328714367743146431760_i128),156535254769922277386972149783216519311_i128,72301842173119110328802662730408483175_i128];
_4 = _8.2.1;
_8.2.1 = _4;
_8.2.0 = _5;
_8.2.1 = _4;
_8.1 = (-9223372036854775808_isize);
_8.0 = [_8.2.2,_8.2.2,_6,_6,_8.2.2,_8.2.2];
_8.2.3 = [3757951626_u32];
_5 = [_3,_3,_3];
_7 = [(-38986980777326124975960925612754684855_i128),(-103524845312157373631064416285846927966_i128),(-25060847985136018521975734785198541831_i128)];
_9 = _8.2.2 < _6;
_9 = _3 <= _3;
_5 = _8.2.0;
_8.2.1 = _4;
_8.2.1 = _4;
_6 = 413_i16 as i32;
_5 = _8.2.0;
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
_6 = _8.2.2 ^ _8.2.2;
_8.2.0 = _5;
_7 = [(-167490044152933175396568176550229600786_i128),(-11176579803007601662377281559836833144_i128),(-66852090812704443755194384784850070377_i128)];
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
_4 = _8.2.1;
_13.2 = 5210_u16 as f32;
_13.0.0 = (-5990_i16);
Goto(bb9)
}
bb9 = {
_8.2.2 = _6;
_8.2.1 = _4;
_3 = 34_i8;
_13.1 = [_8.1,_8.1,_8.1,_8.1,_8.1];
RET = _13.1;
_13.2 = _8.1 as f32;
_8.1 = -(-9223372036854775808_isize);
_8.2.3 = [3082147069_u32];
_13.0.0 = (-30722_i16);
_5 = [_3,_3,_3];
_13.0.3 = [_6,_6,_8.2.2,_6,_6,_6];
_13.0.3 = _8.0;
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
_14 = _5;
_13.0.3 = [_6,_8.2.2,_6,_6,_8.2.2,_6];
_13.0.0 = -30417_i16;
_13.0.1 = _2 as f32;
Goto(bb10)
}
bb10 = {
_11.0 = &RET;
_8.2.2 = _6;
RET = _13.1;
_4 = _8.2.1;
_13.0.0 = 20376_i16;
_13.2 = _13.0.1;
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
RET = _13.1;
_2 = 4344866414067682923_u64;
_13.0.2 = core::ptr::addr_of_mut!(_18);
_13.0.0 = !14485_i16;
_23 = _8.2.3;
_6 = !_8.2.2;
_13.0.1 = _13.2;
_18 = _9 as u32;
_13.0.0 = _4 as i16;
_8.1 = 29_isize;
_10 = core::ptr::addr_of!(_13.0.0);
_16 = [(-31524814546655523201848143965931634406_i128),66961617198150044019783092574385371202_i128,156712411756450235479734105901845489380_i128];
_8.0 = [_8.2.2,_8.2.2,_6,_6,_8.2.2,_8.2.2];
_13.1 = [_8.1,_8.1,_8.1,_8.1,_8.1];
match _8.1 {
0 => bb11,
1 => bb12,
29 => bb14,
_ => bb13
}
}
bb11 = {
_3 = 122_i8;
_3 = 60_i8;
RET = [(-94_isize),(-77_isize),127_isize,(-118_isize),(-9223372036854775808_isize)];
_5 = [_3,_3,_3];
_4 = '\u{98be5}';
_8.2.3 = [1409862546_u32];
_8.1 = 49_isize;
_8.2.2 = 1_usize as i32;
_8.2.0 = _5;
match _3 {
0 => bb2,
60 => bb4,
_ => bb3
}
}
bb12 = {
_7 = [(-77445020482054707328714367743146431760_i128),156535254769922277386972149783216519311_i128,72301842173119110328802662730408483175_i128];
_4 = _8.2.1;
_8.2.1 = _4;
_8.2.0 = _5;
_8.2.1 = _4;
_8.1 = (-9223372036854775808_isize);
_8.0 = [_8.2.2,_8.2.2,_6,_6,_8.2.2,_8.2.2];
_8.2.3 = [3757951626_u32];
_5 = [_3,_3,_3];
_7 = [(-38986980777326124975960925612754684855_i128),(-103524845312157373631064416285846927966_i128),(-25060847985136018521975734785198541831_i128)];
_9 = _8.2.2 < _6;
_9 = _3 <= _3;
_5 = _8.2.0;
_8.2.1 = _4;
_8.2.1 = _4;
_6 = 413_i16 as i32;
_5 = _8.2.0;
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
_6 = _8.2.2 ^ _8.2.2;
_8.2.0 = _5;
_7 = [(-167490044152933175396568176550229600786_i128),(-11176579803007601662377281559836833144_i128),(-66852090812704443755194384784850070377_i128)];
RET = [_8.1,_8.1,_8.1,_8.1,_8.1];
_4 = _8.2.1;
_13.2 = 5210_u16 as f32;
_13.0.0 = (-5990_i16);
Goto(bb9)
}
bb13 = {
_8.1 = !(-9223372036854775808_isize);
_4 = '\u{1f46b}';
Goto(bb5)
}
bb14 = {
_13.0.0 = 30153_i16;
_10 = core::ptr::addr_of!(_13.0.0);
_24 = _8.2.2 | _8.2.2;
_17 = &_13.0.0;
_13.2 = _13.0.1;
_22 = _8.1 * _8.1;
_14 = [_3,_3,_3];
_11.0 = &_13.1;
_8.2.1 = _4;
_8.2.1 = _4;
_7 = [162225629147803092633853514617399933365_i128,(-9868308016649737693280960904977803298_i128),165426606069545888267037954523040340353_i128];
_8.0 = _13.0.3;
_26.0 = [_13.0.0,_13.0.0];
_16 = [164174353380446471459579970359270610273_i128,26239739173991702262091738425219942807_i128,(-149910799962788930576635976283512587123_i128)];
_8.1 = _22;
_18 = !2500822126_u32;
_6 = _18 as i32;
_25 = _3;
_13.0.3 = [_24,_24,_24,_24,_24,_6];
_28 = _8.2.1;
_22 = _8.1;
_11.0 = &RET;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(13_usize, 2_usize, Move(_2), 6_usize, Move(_6), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(13_usize, 26_usize, Move(_26), 7_usize, Move(_7), 18_usize, Move(_18), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [isize; 5],mut _2: [isize; 5],mut _3: [i8; 3],mut _4: [isize; 5]) -> ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1])) {
mir! {
type RET = ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _5: bool;
let _6: [i32; 7];
let _7: [u8; 4];
let _8: *const f64;
let _9: Adt31;
let _10: isize;
let _11: *mut [u16; 3];
let _12: *mut Adt20;
let _13: bool;
let _14: isize;
let _15: *mut (f32,);
let _16: ([i16; 2],);
let _17: ([i16; 2],);
let _18: ([i16; 2], &'static *const u32);
let _19: *mut (f32,);
let _20: &'static [isize; 5];
let _21: i32;
let _22: ([u32; 1], Adt31, *const *mut Adt20, *const (f32,));
let _23: (*mut Adt31,);
let _24: [u8; 4];
let _25: ((i8, *const f64, i16, i32),);
let _26: &'static *mut Adt20;
let _27: isize;
let _28: i32;
let _29: ([i8; 3], char, i32, [u32; 1]);
let _30: (i8, *const f64, i16, i32);
let _31: isize;
let _32: i32;
let _33: isize;
let _34: &'static &'static *const *mut Adt20;
let _35: *mut [u8; 4];
let _36: isize;
let _37: f32;
let _38: [i128; 3];
let _39: i64;
let _40: [i8; 3];
let _41: [i64; 4];
let _42: ();
let _43: ();
{
RET.2.2 = -1866711981_i32;
RET.2.1 = '\u{b8021}';
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_1 = [56_isize,(-9223372036854775808_isize),127_isize,9223372036854775807_isize,9223372036854775807_isize];
RET.2.3 = [832511555_u32];
_2 = _4;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.2.3 = [3653677737_u32];
RET.2.1 = '\u{f8503}';
RET.2.3 = [3381039717_u32];
RET.1 = 9223372036854775807_isize >> RET.2.2;
RET.2.2 = (-112_i8) as i32;
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_4 = _1;
RET.2.3 = [2484779414_u32];
_2 = _4;
Goto(bb1)
}
bb1 = {
RET.2.0 = _3;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.1 = -9223372036854775807_isize;
RET.1 = 9223372036854775807_isize;
RET.1 = 9223372036854775807_isize;
_5 = RET.1 <= RET.1;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_2 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_3 = [(-16_i8),124_i8,113_i8];
_6 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_10 = RET.2.2 as isize;
_6 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.2.0 = [(-77_i8),(-47_i8),79_i8];
RET.2.0 = _3;
RET.1 = -_10;
_5 = true ^ true;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_7 = [187_u8,176_u8,60_u8,156_u8];
_14 = -_10;
_2 = [_14,_10,_14,_10,_14];
_10 = RET.1 + RET.1;
_13 = !_5;
Call(_8 = fn15(_10, _1, _13, _14, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = RET.1;
_14 = _10;
RET.2.1 = '\u{3b9fc}';
_16.0 = [17515_i16,30336_i16];
RET.1 = -_10;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.2.1 = '\u{b2bfb}';
_16.0 = [(-26642_i16),30941_i16];
RET.1 = _14 & _14;
_14 = RET.1 & _10;
RET.1 = _5 as isize;
_5 = !_13;
_2 = [_14,_10,_14,_14,_10];
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_1 = [_14,_14,_14,_14,RET.1];
RET.2.0 = _3;
_17 = (_16.0,);
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_10 = RET.2.2 as isize;
RET.2.2 = (-805504343_i32) - 1917498541_i32;
RET.2.1 = '\u{dbe7e}';
_4 = _1;
_17 = _16;
_5 = _13 & _13;
Goto(bb3)
}
bb3 = {
_16 = (_17.0,);
_16 = (_17.0,);
RET.1 = 171073520_u32 as isize;
_16.0 = [(-32169_i16),4114_i16];
RET.2.0 = [61_i8,64_i8,105_i8];
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_22.0 = [2554653320_u32];
_18.0 = _16.0;
_16.0 = [32021_i16,(-30633_i16)];
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_14 = _10;
_3 = RET.2.0;
_2 = [_14,_10,_14,_10,_14];
_16.0 = [30235_i16,7906_i16];
RET.2 = (_3, '\u{af84f}', 1364453063_i32, _22.0);
RET.2.3 = _22.0;
match RET.2.2 {
0 => bb4,
1 => bb5,
2 => bb6,
1364453063 => bb8,
_ => bb7
}
}
bb4 = {
_10 = RET.1;
_14 = _10;
RET.2.1 = '\u{3b9fc}';
_16.0 = [17515_i16,30336_i16];
RET.1 = -_10;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.2.1 = '\u{b2bfb}';
_16.0 = [(-26642_i16),30941_i16];
RET.1 = _14 & _14;
_14 = RET.1 & _10;
RET.1 = _5 as isize;
_5 = !_13;
_2 = [_14,_10,_14,_14,_10];
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_1 = [_14,_14,_14,_14,RET.1];
RET.2.0 = _3;
_17 = (_16.0,);
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_10 = RET.2.2 as isize;
RET.2.2 = (-805504343_i32) - 1917498541_i32;
RET.2.1 = '\u{dbe7e}';
_4 = _1;
_17 = _16;
_5 = _13 & _13;
Goto(bb3)
}
bb5 = {
RET.2.0 = _3;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.1 = -9223372036854775807_isize;
RET.1 = 9223372036854775807_isize;
RET.1 = 9223372036854775807_isize;
_5 = RET.1 <= RET.1;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_2 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_3 = [(-16_i8),124_i8,113_i8];
_6 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_10 = RET.2.2 as isize;
_6 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
RET.2.0 = [(-77_i8),(-47_i8),79_i8];
RET.2.0 = _3;
RET.1 = -_10;
_5 = true ^ true;
RET.0 = [RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_7 = [187_u8,176_u8,60_u8,156_u8];
_14 = -_10;
_2 = [_14,_10,_14,_10,_14];
_10 = RET.1 + RET.1;
_13 = !_5;
Call(_8 = fn15(_10, _1, _13, _14, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
RET.1 = -_14;
match RET.2.2 {
1364453063 => bb9,
_ => bb3
}
}
bb9 = {
_25.0.1 = Move(_8);
_7 = [163_u8,176_u8,83_u8,104_u8];
_8 = Move(_25.0.1);
_23.0 = core::ptr::addr_of_mut!(_22.1);
_21 = RET.2.2;
_21 = (-86_i8) as i32;
_26 = &_12;
RET.1 = _10;
RET.2.0 = _3;
RET.0 = [_21,RET.2.2,RET.2.2,RET.2.2,RET.2.2,RET.2.2];
_20 = &_2;
_17.0 = [(-28821_i16),(-30747_i16)];
_18.0 = [(-18299_i16),(-18002_i16)];
_18.0 = [31054_i16,2079_i16];
_13 = !_5;
_25.0.3 = !RET.2.2;
_27 = _14;
_25.0.1 = Move(_8);
RET.2.3 = [3309088719_u32];
_7 = [157_u8,255_u8,218_u8,246_u8];
_10 = -RET.1;
RET.0 = [_25.0.3,_21,_25.0.3,RET.2.2,_25.0.3,RET.2.2];
_30.0 = (-97_i8) & (-70_i8);
Goto(bb10)
}
bb10 = {
RET.2.1 = '\u{e79f0}';
_18.0 = [(-16306_i16),18933_i16];
RET.2 = (_3, '\u{a4631}', _21, _22.0);
RET.2.0 = [_30.0,_30.0,_30.0];
_29.3 = [3592563458_u32];
_23.0 = core::ptr::addr_of_mut!(_9);
_30 = (48_i8, Move(_25.0.1), 6977_i16, RET.2.2);
_1 = _2;
_6 = [RET.2.2,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3];
_24 = _7;
_29.3 = RET.2.3;
_29.2 = 4238340110273058502_i64 as i32;
Goto(bb11)
}
bb11 = {
_13 = RET.2.1 != RET.2.1;
_5 = !_13;
RET.2.3 = [2336759961_u32];
_29.1 = RET.2.1;
_25.0.1 = Move(_30.1);
_25.0.2 = !_30.2;
_20 = &_4;
RET.2.3 = [3008182051_u32];
_26 = &_12;
Goto(bb12)
}
bb12 = {
RET.2.2 = _30.3;
_35 = core::ptr::addr_of_mut!(_7);
_5 = !_13;
RET.2 = (_3, _29.1, _30.3, _22.0);
RET.2 = (_3, _29.1, _25.0.3, _22.0);
_30.2 = _25.0.2;
_7 = [245_u8,70_u8,115_u8,67_u8];
_30 = (6_i8, Move(_25.0.1), _25.0.2, _29.2);
RET.0 = [_25.0.3,_30.3,_25.0.3,_30.3,_25.0.3,_25.0.3];
_5 = !_13;
_2 = _4;
Goto(bb13)
}
bb13 = {
_25 = (Move(_30),);
RET.1 = 14205199559630049943_u64 as isize;
_29.3 = [4097898908_u32];
_30.2 = 3414958929_u32 as i16;
_30.2 = _25.0.2;
RET.1 = _13 as isize;
_37 = 3977803167_u32 as f32;
_17.0 = [_30.2,_25.0.2];
_6 = [RET.2.2,RET.2.2,RET.2.2,_21,RET.2.2,RET.2.2,RET.2.2];
_37 = 2727477476_u32 as f32;
_26 = &_12;
_24 = _7;
RET.2 = (_3, _29.1, _29.2, _29.3);
_5 = _29.1 >= RET.2.1;
_16 = (_17.0,);
_8 = Move(_25.0.1);
_31 = (-140954833225199944309163924992084616555_i128) as isize;
RET.2.1 = _29.1;
RET.0 = [RET.2.2,_29.2,_25.0.3,RET.2.2,_25.0.3,_21];
_30.1 = Move(_8);
_20 = &_4;
_3 = [_25.0.0,_25.0.0,_25.0.0];
_6 = [_25.0.3,RET.2.2,_21,_29.2,_25.0.3,RET.2.2,_25.0.3];
Goto(bb14)
}
bb14 = {
Call(_42 = dump_var(14_usize, 16_usize, Move(_16), 10_usize, Move(_10), 21_usize, Move(_21), 17_usize, Move(_17)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_42 = dump_var(14_usize, 27_usize, Move(_27), 5_usize, Move(_5), 24_usize, Move(_24), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: [isize; 5],mut _3: bool,mut _4: isize,mut _5: bool) -> *const f64 {
mir! {
type RET = *const f64;
let _6: (f32,);
let _7: u16;
let _8: u16;
let _9: *mut *const *mut Adt20;
let _10: bool;
let _11: [i32; 5];
let _12: [i64; 7];
let _13: *mut [i128; 3];
let _14: f32;
let _15: char;
let _16: ([i8; 3], char, i32, [u32; 1]);
let _17: char;
let _18: *mut Adt20;
let _19: u16;
let _20: Adt67;
let _21: i8;
let _22: *mut (f32,);
let _23: [usize; 8];
let _24: usize;
let _25: f32;
let _26: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _27: [i16; 2];
let _28: u8;
let _29: *const (f32,);
let _30: u64;
let _31: u64;
let _32: i128;
let _33: isize;
let _34: [i128; 3];
let _35: i64;
let _36: u128;
let _37: bool;
let _38: isize;
let _39: u32;
let _40: (&'static [isize; 5],);
let _41: Adt20;
let _42: *mut [u16; 3];
let _43: f64;
let _44: &'static *const *mut Adt20;
let _45: [i32; 6];
let _46: i32;
let _47: char;
let _48: ();
let _49: ();
{
_1 = _4 & _4;
_1 = _4;
_1 = !_4;
_4 = _1;
_1 = 2252562091_u32 as isize;
_2 = [_1,_1,_4,_4,_4];
_5 = _3;
_4 = !_1;
_6.0 = 123_i8 as f32;
_3 = _5;
_4 = _1 - _1;
Goto(bb1)
}
bb1 = {
_1 = 1274292974_u32 as isize;
_1 = _4 - _4;
_5 = !_3;
_6.0 = (-346084825_i32) as f32;
_2 = [_4,_4,_1,_1,_1];
_4 = _1;
_1 = _4;
_6.0 = (-60_i8) as f32;
Goto(bb2)
}
bb2 = {
_3 = _1 < _1;
_3 = _5;
_5 = _3;
_1 = 1958917433769142254_u64 as isize;
_1 = -_4;
_1 = 60447425700257128_usize as isize;
_6.0 = 148972820479114792_usize as f32;
_4 = !_1;
_8 = 46812_u16 - 19145_u16;
Goto(bb3)
}
bb3 = {
_3 = !_5;
_4 = _1;
_3 = !_5;
_5 = _3 | _3;
_6.0 = 9070659270366587036_i64 as f32;
_4 = -_1;
_11 = [(-1197492087_i32),1686981036_i32,(-285983253_i32),1711259052_i32,882779359_i32];
_7 = _8 - _8;
_4 = -_1;
_7 = _8 * _8;
_5 = _3 & _3;
_1 = _6.0 as isize;
_6.0 = (-8958171106964608475_i64) as f32;
_10 = _5;
_1 = 10963406126003518698_usize as isize;
_11 = [797544138_i32,(-1974695686_i32),2057531559_i32,1223603223_i32,1876504337_i32];
_8 = !_7;
_12 = [1985030520284647187_i64,(-3989993722979476009_i64),8579259829674855098_i64,(-3592137778313071046_i64),(-5623921699450781029_i64),6662751644418052691_i64,6071556736755673757_i64];
_8 = !_7;
_3 = _10;
_10 = !_5;
_1 = (-7480561253504696312_i64) as isize;
_5 = _10;
Goto(bb4)
}
bb4 = {
_3 = _7 != _7;
_5 = !_3;
_11 = [(-1319030819_i32),(-634198026_i32),(-1927834370_i32),187307989_i32,1232889092_i32];
_11 = [1094854123_i32,(-210090765_i32),(-1071703119_i32),(-1102556_i32),671984630_i32];
_11 = [904668351_i32,839155900_i32,611098013_i32,2111302476_i32,592509776_i32];
Call(RET = fn16(_7, _12, _3, _4, _3, _8, _11, _7, _2, _12, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_15 = '\u{2c629}';
_16.2 = 1433863478_i32;
_4 = _1 + _1;
_16.0 = [(-51_i8),33_i8,76_i8];
_16.3 = [3671870222_u32];
_12 = [6520408612934581079_i64,8490946182615048308_i64,2345918080235761776_i64,7903283705637486832_i64,4798231981238898946_i64,(-6532447844490892978_i64),(-1457328230049071904_i64)];
_14 = _6.0;
_4 = !_1;
_3 = !_5;
_5 = _10 ^ _3;
_7 = 8_i8 as u16;
_15 = '\u{102002}';
_10 = _5;
_16.2 = !(-1488857555_i32);
_16.1 = _15;
_16.2 = -114357293_i32;
_15 = _16.1;
_6 = (_14,);
_16.0 = [97_i8,(-116_i8),(-108_i8)];
_16.3 = [1199516336_u32];
_8 = _7;
_16.3 = [4246763284_u32];
_6.0 = -_14;
_2 = [_1,_4,_1,_1,_1];
_11 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_8 = _7;
_5 = _10;
_11 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_6.0 = _14;
_17 = _15;
Goto(bb6)
}
bb6 = {
_14 = _6.0;
_10 = _5;
_17 = _15;
_19 = _16.2 as u16;
_16.0 = [(-58_i8),(-17_i8),80_i8];
_16.1 = _17;
_16.3 = [4086375717_u32];
Goto(bb7)
}
bb7 = {
_10 = _5;
_7 = _8;
_16.2 = 1685844507_i32 * (-1396240810_i32);
_3 = !_10;
_21 = 83592748776509468333003448093675711305_i128 as i8;
_3 = _10 & _10;
_24 = 64678826154229078679491617457364451831_i128 as usize;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
_14 = 36_u8 as f32;
_2 = [_4,_4,_4,_1,_1];
_17 = _16.1;
_17 = _15;
_8 = !_7;
_21 = (-61_i8) + 75_i8;
_11 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_7 = _8;
_2 = [_1,_1,_1,_4,_1];
_15 = _16.1;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
Goto(bb8)
}
bb8 = {
_7 = _4 as u16;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
_16.0 = [_21,_21,_21];
_16.1 = _17;
_16.1 = _17;
_26.1 = _4;
_16.0 = [_21,_21,_21];
_26.2.0 = _16.0;
_26.2.1 = _16.1;
_24 = 0_usize;
_26.0 = [_11[_24],_16.2,_11[_24],_16.2,_16.2,_16.2];
_6 = (_14,);
Goto(bb9)
}
bb9 = {
_26.2.2 = 52883499676856182157513056625908299766_i128 as i32;
_10 = !_5;
_5 = _3;
_17 = _15;
_26.2 = _16;
_26.0[_24] = _16.0[_24] as i32;
_19 = _24 as u16;
_26.2.3[_24] = !_16.3[_24];
_16.1 = _26.2.1;
_26.2 = _16;
_22 = core::ptr::addr_of_mut!(_6);
_26.2.3 = _16.3;
_27[_24] = !17743_i16;
_16.3[_24] = _26.0[_24] as u32;
_16.0 = _26.2.0;
_4 = _12[_24] as isize;
_21 = _23[_24] as i8;
_26.2.2 = _11[_24] * _11[_24];
_11[_24] = _26.0[_24] >> _26.0[_24];
_5 = !_3;
_11[_24] = _26.2.2;
_16.3[_24] = _16.0[_24] as u32;
_16.1 = _17;
_26.1 = _4;
_12 = [(-1383603693137566358_i64),(-8253262055609540390_i64),418076931762023142_i64,(-5984237707020033416_i64),(-5866528750004299939_i64),1962287371870089583_i64,(-7189386207412408808_i64)];
_22 = core::ptr::addr_of_mut!((*_22));
_12 = [382593575975279659_i64,5827613974359380799_i64,(-5721173777766220485_i64),(-2747222607018778634_i64),664989544329642854_i64,(-1054318473015119278_i64),2011371191585912164_i64];
_6.0 = _14;
_24 = _23[_24];
Call(_27 = core::intrinsics::transmute(_16.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_22 = core::ptr::addr_of_mut!((*_22));
_17 = _15;
_25 = _6.0;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
_28 = 16913720852630286431_u64 as u8;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
(*_22).0 = _19 as f32;
_26.0 = [_16.2,_26.2.2,_26.2.2,_26.2.2,_26.2.2,_26.2.2];
_6 = (_25,);
_27 = [23711_i16,(-15476_i16)];
(*_22).0 = -_25;
_14 = -(*_22).0;
_1 = -_26.1;
_26.2.0 = [_21,_21,_21];
(*_22).0 = -_25;
_16.0 = _26.2.0;
_12 = [(-2388271588286209206_i64),565905220587307416_i64,(-5800465148131478276_i64),7346151549423011548_i64,(-4804082449207606128_i64),(-4056017115731986051_i64),(-351403705638732881_i64)];
_26.2.3 = _16.3;
_10 = !_5;
_12 = [(-8213187242783297162_i64),5206781297409366993_i64,2156459573874528599_i64,7207160978513582640_i64,(-8278170655853407574_i64),(-6562832248490583423_i64),(-3291450978041318123_i64)];
(*_22).0 = _25 - _14;
_7 = _8;
_23 = [_24,_24,_24,_24,_24,_24,_24,_24];
(*_22) = (_14,);
_19 = _28 as u16;
_16.2 = _6.0 as i32;
_6.0 = _14;
_16 = _26.2;
_5 = !_10;
Goto(bb11)
}
bb11 = {
_16 = _26.2;
_31 = !8882372371516534902_u64;
_16.3 = _26.2.3;
_26.2.3 = [1590813689_u32];
_1 = _26.1 * _26.1;
_26.2.0 = [_21,_21,_21];
_33 = _1;
_32 = (-124656313850368593504696978793339961633_i128) >> _26.1;
_16.2 = _32 as i32;
_27 = [11435_i16,27473_i16];
_25 = _14 + (*_22).0;
Goto(bb12)
}
bb12 = {
_31 = !189027991002549791_u64;
_34 = [_32,_32,_32];
_21 = _31 as i8;
(*_22).0 = 423797344_u32 as f32;
_37 = _10 <= _3;
_35 = (-3902579978952555252_i64) & 3868419771644435252_i64;
_36 = _16.2 as u128;
_6.0 = -_14;
_26.2 = _16;
_14 = _25;
_13 = core::ptr::addr_of_mut!(_34);
_26.2.3 = _16.3;
_13 = core::ptr::addr_of_mut!((*_13));
_26.2.3 = [1070898198_u32];
(*_22) = (_25,);
_16.1 = _17;
_26.1 = -_1;
(*_22).0 = _14 * _14;
_36 = 281917131950127816944213196593967151897_u128 & 306082440331317250168077285601235137609_u128;
_26.2.0 = _16.0;
_30 = _31;
_18 = core::ptr::addr_of_mut!(_41);
_2 = [_33,_33,_4,_4,_1];
_8 = _19 ^ _7;
_32 = _36 as i128;
_43 = _35 as f64;
_32 = _24 as i128;
Goto(bb13)
}
bb13 = {
_4 = _26.1 | _1;
_34 = [_32,_32,_32];
_36 = _21 as u128;
(*_13) = [_32,_32,_32];
_26.1 = _1;
_7 = _19 * _8;
_16.0 = _26.2.0;
_35 = 6176263089921720020_i64 * 1202768065832290362_i64;
(*_18) = Adt20::Variant1 { fld0: _28,fld1: _17 };
RET = core::ptr::addr_of!(_43);
_6 = (_25,);
_2 = [_26.1,_1,_26.1,_33,_4];
place!(Field::<u8>(Variant((*_18), 1), 0)) = _28;
_27 = [6644_i16,4809_i16];
_6.0 = -_14;
(*_13) = [_32,_32,_32];
SetDiscriminant(_41, 1);
place!(Field::<u8>(Variant((*_18), 1), 0)) = _7 as u8;
Goto(bb14)
}
bb14 = {
_39 = _16.1 as u32;
_36 = 243978515229654170333317209057172018696_u128 & 269565420465716074244566213809372180216_u128;
_14 = _25 * (*_22).0;
(*_22).0 = _35 as f32;
_28 = Field::<u8>(Variant(_41, 1), 0) * Field::<u8>(Variant(_41, 1), 0);
_28 = !Field::<u8>(Variant(_41, 1), 0);
_38 = _4;
_35 = !(-6954884452939492120_i64);
_1 = _26.2.2 as isize;
_26.2.0 = [_21,_21,_21];
_41 = Adt20::Variant1 { fld0: _28,fld1: _16.1 };
(*_22).0 = _14 - _14;
place!(Field::<u8>(Variant(_41, 1), 0)) = _28 ^ _28;
_38 = _21 as isize;
_16 = (_26.2.0, _15, _26.2.2, _26.2.3);
place!(Field::<u8>(Variant(_41, 1), 0)) = _28;
(*_13) = [_32,_32,_32];
_33 = (-25561_i16) as isize;
(*RET) = _24 as f64;
_43 = _39 as f64;
_16.3 = [_39];
_31 = _30;
place!(Field::<char>(Variant((*_18), 1), 1)) = _17;
place!(Field::<u8>(Variant((*_18), 1), 0)) = !_28;
_3 = _37 ^ _5;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(15_usize, 3_usize, Move(_3), 30_usize, Move(_30), 32_usize, Move(_32), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(15_usize, 39_usize, Move(_39), 31_usize, Move(_31), 34_usize, Move(_34), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(15_usize, 10_usize, Move(_10), 4_usize, Move(_4), 2_usize, Move(_2), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(15_usize, 5_usize, Move(_5), 23_usize, Move(_23), 24_usize, Move(_24), 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u16,mut _2: [i64; 7],mut _3: bool,mut _4: isize,mut _5: bool,mut _6: u16,mut _7: [i32; 5],mut _8: u16,mut _9: [isize; 5],mut _10: [i64; 7],mut _11: [i32; 5]) -> *const f64 {
mir! {
type RET = *const f64;
let _12: isize;
let _13: [isize; 5];
let _14: char;
let _15: *mut [u8; 4];
let _16: f64;
let _17: *mut isize;
let _18: [i64; 4];
let _19: isize;
let _20: [i32; 7];
let _21: char;
let _22: f32;
let _23: ([i16; 2],);
let _24: (i16, i16, [i16; 2]);
let _25: bool;
let _26: [u128; 8];
let _27: [i128; 3];
let _28: *mut [i16; 2];
let _29: isize;
let _30: f64;
let _31: ([i8; 3], char, i32, [u32; 1]);
let _32: (i8, *const f64, i16, i32);
let _33: f32;
let _34: [i128; 3];
let _35: *mut [i128; 3];
let _36: &'static &'static isize;
let _37: u128;
let _38: char;
let _39: &'static &'static isize;
let _40: [u32; 1];
let _41: isize;
let _42: ();
let _43: ();
{
_11 = _7;
_10 = [(-5457668509590259297_i64),(-4778398730740865737_i64),(-2883876595420287039_i64),6406789167483429311_i64,1002734424273285560_i64,1175248596534959756_i64,3791891830639822637_i64];
_6 = _8;
_12 = _3 as isize;
_2 = _10;
_13 = _9;
_12 = _3 as isize;
_7 = _11;
_2 = _10;
_9 = _13;
_14 = '\u{40ce0}';
_4 = _12 >> _6;
_4 = !_12;
_2 = [(-665814096500185965_i64),919591525157964647_i64,(-2666744454664550493_i64),5176344705129350184_i64,(-455584751437636818_i64),(-4042135552585345234_i64),(-4925789561374266668_i64)];
_10 = [(-5495915200432586307_i64),8275520531584071783_i64,8754440654915573309_i64,1265552929454000067_i64,4741974884722548827_i64,5188000977382034492_i64,943618210340012878_i64];
_9 = [_4,_12,_4,_4,_12];
_3 = _5;
_6 = 55_u8 as u16;
_5 = _3;
_7 = [1509037408_i32,(-1046561422_i32),2081084624_i32,254541266_i32,(-770282324_i32)];
_2 = [7531925755979267079_i64,3445513626965108647_i64,(-8742014319071270256_i64),2082952808298218429_i64,5360672546033212754_i64,(-263314592654669924_i64),(-6423183165770662835_i64)];
_5 = _4 >= _4;
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of!(_16);
RET = core::ptr::addr_of!((*RET));
_2 = [(-8595638661995383118_i64),(-5519036578168066086_i64),1358662805619401391_i64,(-1726496319189770360_i64),5728631186965852603_i64,(-8195677638074056442_i64),(-6238283639531608984_i64)];
(*RET) = 10243761825936997214_u64 as f64;
_10 = [(-904430500025750156_i64),(-2260595732808995610_i64),8352520919182550754_i64,2564104514645843104_i64,(-4567910504577881551_i64),(-6437280286081455242_i64),7806358396797287009_i64];
_2 = _10;
_18 = [2500474482050798304_i64,(-5243570100521330572_i64),9058742375490879951_i64,(-6922985969621996472_i64)];
_8 = !_1;
Call(_4 = core::intrinsics::transmute(_12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [5274421159260807280_i64,(-6945634605969409291_i64),4703954728155313292_i64,697650432360970270_i64,(-5038760905505660061_i64),9214370331846280421_i64,(-9058525692565106641_i64)];
_3 = _4 > _4;
_7 = _11;
_1 = 1405580896_u32 as u16;
_10 = [(-8212870194803548737_i64),(-3818495348450903541_i64),4961522479607132900_i64,(-3943325197920728613_i64),(-1349570451804376464_i64),4181961598531894508_i64,(-91499375224909596_i64)];
(*RET) = 23921_i16 as f64;
RET = core::ptr::addr_of!((*RET));
_12 = _4;
_17 = core::ptr::addr_of_mut!(_4);
_21 = _14;
(*RET) = 88_i8 as f64;
_8 = _6 & _6;
(*RET) = 327412068758566894_u64 as f64;
_14 = _21;
Goto(bb3)
}
bb3 = {
_11 = [1871248735_i32,798920302_i32,546313338_i32,(-2043685376_i32),714282888_i32];
_22 = _8 as f32;
(*RET) = _4 as f64;
_10 = _2;
_18 = [(-1553072894264682723_i64),7456547755864628549_i64,8883070288990322360_i64,(-680653455409074939_i64)];
_23.0 = [(-31194_i16),(-1856_i16)];
_20 = [575557080_i32,(-290791737_i32),431264571_i32,(-2106758306_i32),(-734679947_i32),476910346_i32,(-437389127_i32)];
_6 = _1;
_17 = core::ptr::addr_of_mut!(_12);
(*RET) = 937925438_i32 as f64;
(*RET) = _8 as f64;
_18 = [(-164626141067260864_i64),(-6541449831616867220_i64),6008425202404486819_i64,8567078466086775236_i64];
_3 = !_5;
_24.1 = -22204_i16;
_24.2 = [_24.1,_24.1];
_24.1 = (-38225749607141964_i64) as i16;
_16 = _22 as f64;
(*RET) = 105_u8 as f64;
_23 = (_24.2,);
_3 = !_5;
_2 = _10;
Goto(bb4)
}
bb4 = {
_3 = _4 > _12;
_24.1 = (-22236_i16) * (-17922_i16);
_20 = [893381318_i32,(-351699761_i32),1090241519_i32,(-637184511_i32),1064016979_i32,(-1781789593_i32),(-392405411_i32)];
_24 = ((-3456_i16), (-6788_i16), _23.0);
_23.0 = [_24.0,_24.0];
_24 = ((-28162_i16), 7167_i16, _23.0);
Call(_13 = core::intrinsics::transmute(_9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_24.1 = -_24.0;
(*_17) = !_4;
_18 = [(-1858326634548578452_i64),3149357759214183166_i64,6548452325830482168_i64,(-3855492049161046437_i64)];
_21 = _14;
_25 = _5;
_27 = [(-140923723151152178761494337870054072544_i128),127278420650811102449765282628715810908_i128,138616961470678695734663479873620610609_i128];
_14 = _21;
(*_17) = _4 << _24.0;
_21 = _14;
_3 = _25 ^ _5;
RET = core::ptr::addr_of!((*RET));
_21 = _14;
_20 = [428084306_i32,1395559029_i32,1151373499_i32,745032978_i32,1139393642_i32,1326054892_i32,(-1645560933_i32)];
_17 = core::ptr::addr_of_mut!((*_17));
RET = core::ptr::addr_of!((*RET));
_6 = !_1;
_8 = _6;
_20 = [604546396_i32,1556817718_i32,984027962_i32,(-1939932155_i32),643084868_i32,741423563_i32,1339992911_i32];
_28 = core::ptr::addr_of_mut!(_24.2);
_21 = _14;
RET = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb6 = {
_24.2 = [_24.1,_24.1];
_29 = (*_17);
_29 = _12;
(*_17) = 16724802154142079715_u64 as isize;
_10 = _2;
_13 = _9;
_28 = core::ptr::addr_of_mut!(_24.2);
_5 = _24.0 > _24.0;
_31.0 = [(-52_i8),(-122_i8),21_i8];
_32.3 = 1530755346_i32 ^ 54659790_i32;
_25 = _5;
_32.0 = _32.3 as i8;
(*_17) = _29 & _4;
_21 = _14;
_24 = ((-5826_i16), 26523_i16, _23.0);
(*_17) = _4;
(*_28) = [_24.1,_24.0];
_9 = [(*_17),_12,_29,(*_17),(*_17)];
(*_17) = !_4;
_5 = _25 & _25;
_12 = _29;
_10 = [(-2870534758725892907_i64),3842293246652211833_i64,8993363885911850531_i64,7199185534504099237_i64,3817348339167168418_i64,7690139745143924388_i64,1306234410811165682_i64];
_33 = _22;
Goto(bb7)
}
bb7 = {
(*_28) = [_24.0,_24.1];
_20 = [_32.3,_32.3,_32.3,_32.3,_32.3,_32.3,_32.3];
match _24.1 {
26523 => bb9,
_ => bb8
}
}
bb8 = {
_24.1 = -_24.0;
(*_17) = !_4;
_18 = [(-1858326634548578452_i64),3149357759214183166_i64,6548452325830482168_i64,(-3855492049161046437_i64)];
_21 = _14;
_25 = _5;
_27 = [(-140923723151152178761494337870054072544_i128),127278420650811102449765282628715810908_i128,138616961470678695734663479873620610609_i128];
_14 = _21;
(*_17) = _4 << _24.0;
_21 = _14;
_3 = _25 ^ _5;
RET = core::ptr::addr_of!((*RET));
_21 = _14;
_20 = [428084306_i32,1395559029_i32,1151373499_i32,745032978_i32,1139393642_i32,1326054892_i32,(-1645560933_i32)];
_17 = core::ptr::addr_of_mut!((*_17));
RET = core::ptr::addr_of!((*RET));
_6 = !_1;
_8 = _6;
_20 = [604546396_i32,1556817718_i32,984027962_i32,(-1939932155_i32),643084868_i32,741423563_i32,1339992911_i32];
_28 = core::ptr::addr_of_mut!(_24.2);
_21 = _14;
RET = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb9 = {
_32.2 = _24.1;
_32.0 = 5_i8 | (-98_i8);
match _24.1 {
26523 => bb10,
_ => bb3
}
}
bb10 = {
_31.1 = _21;
_26 = [30142163575446049374983731915911092392_u128,242208185613414734776852188030951291405_u128,298658255403372420504933387295245479230_u128,230633591177397372561652899494292382576_u128,63958155866319605918026564930917323569_u128,113565148468660912738970144566498461220_u128,227754657532932778108202748936717942015_u128,72214870427986292645620444434980629150_u128];
_8 = _6;
_10 = [(-573403924654089642_i64),2434923088705034212_i64,(-5784010225531982583_i64),(-6144866485617638523_i64),(-6708046524143844876_i64),4843248622480273071_i64,1806185251485327024_i64];
_23.0 = [_24.0,_24.1];
_21 = _14;
Goto(bb11)
}
bb11 = {
(*RET) = (-10512939030868553502494057706449928803_i128) as f64;
_32 = ((-114_i8), Move(RET), _24.0, 1663384658_i32);
_27 = [27611680970411936832597332769117957842_i128,(-11050703436442226823313746583658181418_i128),16382096611900802543979000062732177230_i128];
_16 = 183_u8 as f64;
_20 = [_32.3,_32.3,_32.3,_32.3,_32.3,_32.3,_32.3];
RET = Move(_32.1);
_22 = _33 + _33;
_32.2 = _24.0;
(*_17) = _29 | _29;
_18 = [(-5788555568627522476_i64),(-8913090573776075308_i64),8191488555567524718_i64,1017905609893558744_i64];
match _32.3 {
0 => bb4,
1 => bb2,
2 => bb10,
3 => bb12,
1663384658 => bb14,
_ => bb13
}
}
bb12 = {
_31.1 = _21;
_26 = [30142163575446049374983731915911092392_u128,242208185613414734776852188030951291405_u128,298658255403372420504933387295245479230_u128,230633591177397372561652899494292382576_u128,63958155866319605918026564930917323569_u128,113565148468660912738970144566498461220_u128,227754657532932778108202748936717942015_u128,72214870427986292645620444434980629150_u128];
_8 = _6;
_10 = [(-573403924654089642_i64),2434923088705034212_i64,(-5784010225531982583_i64),(-6144866485617638523_i64),(-6708046524143844876_i64),4843248622480273071_i64,1806185251485327024_i64];
_23.0 = [_24.0,_24.1];
_21 = _14;
Goto(bb11)
}
bb13 = {
RET = core::ptr::addr_of!(_16);
RET = core::ptr::addr_of!((*RET));
_2 = [(-8595638661995383118_i64),(-5519036578168066086_i64),1358662805619401391_i64,(-1726496319189770360_i64),5728631186965852603_i64,(-8195677638074056442_i64),(-6238283639531608984_i64)];
(*RET) = 10243761825936997214_u64 as f64;
_10 = [(-904430500025750156_i64),(-2260595732808995610_i64),8352520919182550754_i64,2564104514645843104_i64,(-4567910504577881551_i64),(-6437280286081455242_i64),7806358396797287009_i64];
_2 = _10;
_18 = [2500474482050798304_i64,(-5243570100521330572_i64),9058742375490879951_i64,(-6922985969621996472_i64)];
_8 = !_1;
Call(_4 = core::intrinsics::transmute(_12), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_29 = (*_17) << _32.2;
_12 = _4 << _4;
(*_28) = [_32.2,_24.0];
RET = core::ptr::addr_of!(_16);
_24.0 = !_32.2;
_10 = _2;
_31.3 = [2286284308_u32];
_2 = [8235783949070175610_i64,(-8753999730861357265_i64),(-2497429088185579153_i64),(-1603261993629902526_i64),(-3692392612087745922_i64),(-4543550047946899004_i64),593139319941510012_i64];
_27 = [(-74326181138232788969439286502674487633_i128),(-64664836201891438686179676512571436339_i128),41667429122066916079087677747970584475_i128];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(16_usize, 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(16_usize, 23_usize, Move(_23), 20_usize, Move(_20), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(16_usize, 9_usize, Move(_9), 21_usize, Move(_21), 24_usize, Move(_24), 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u64,mut _2: i32,mut _3: char,mut _4: i32,mut _5: u128,mut _6: bool,mut _7: i32,mut _8: i32,mut _9: ([i8; 3], char, i32, [u32; 1]),mut _10: ([i8; 3], char, i32, [u32; 1])) -> u8 {
mir! {
type RET = u8;
let _11: isize;
let _12: *mut u32;
let _13: *const (f32,);
let _14: *mut u16;
let _15: *mut isize;
let _16: Adt35;
let _17: [i128; 3];
let _18: f32;
let _19: *const f64;
let _20: *mut (f32,);
let _21: *const (f32,);
let _22: *mut u32;
let _23: &'static &'static *const *mut Adt20;
let _24: u16;
let _25: bool;
let _26: *const f64;
let _27: [i64; 4];
let _28: f32;
let _29: &'static *const u32;
let _30: (i16, i16, [i16; 2]);
let _31: f64;
let _32: [i64; 1];
let _33: u16;
let _34: *const *mut Adt20;
let _35: u64;
let _36: &'static &'static isize;
let _37: *const (f32,);
let _38: (i8, *const f64, i16, i32);
let _39: bool;
let _40: [u8; 4];
let _41: ();
let _42: ();
{
_10.0 = [(-112_i8),(-34_i8),89_i8];
Goto(bb1)
}
bb1 = {
_10.1 = _3;
_9.3 = _10.3;
_9 = (_10.0, _3, _8, _10.3);
_9.2 = _2 & _2;
_4 = _10.2;
_9.0 = [82_i8,(-70_i8),(-61_i8)];
RET = 30_u8 | 52_u8;
RET = 221_u8;
_15 = core::ptr::addr_of_mut!(_11);
(*_15) = -9223372036854775807_isize;
_16 = Adt35 { fld0: _1 };
_16 = Adt35 { fld0: _1 };
_10.2 = _2;
(*_15) = (-9223372036854775808_isize);
RET = (-157094228896104931481396528643199533220_i128) as u8;
_2 = _7;
_8 = -_10.2;
_3 = _10.1;
_4 = _11 as i32;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
42411490318861912216617068188775473062 => bb6,
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
_9.1 = _3;
_10.0 = [(-75_i8),(-33_i8),(-127_i8)];
_1 = _16.fld0 + _16.fld0;
_10.0 = [(-98_i8),(-121_i8),(-76_i8)];
_10 = (_9.0, _3, _8, _9.3);
_10 = _9;
RET = 183_u8;
_9.3 = [1474144839_u32];
_7 = _11 as i32;
(*_15) = !(-9223372036854775808_isize);
_18 = _16.fld0 as f32;
_9 = (_10.0, _10.1, _2, _10.3);
_8 = !_9.2;
_17 = [(-73150176092150300177576194868298149891_i128),144919834067969969483814896204699403692_i128,(-25792042959353848756215489677465354213_i128)];
_15 = core::ptr::addr_of_mut!((*_15));
_5 = 13811445040443044819810148653977471013_u128 - 288546697185657601360127847226488399671_u128;
_2 = -_10.2;
_8 = _10.2;
_9.2 = 128187248762750807201386174946679554313_i128 as i32;
_15 = core::ptr::addr_of_mut!(_11);
Goto(bb7)
}
bb7 = {
_6 = !false;
_2 = 12131_u16 as i32;
_4 = _8;
_9.3 = [508923769_u32];
_9.2 = _4;
_9.0 = [115_i8,41_i8,14_i8];
_1 = !_16.fld0;
_8 = 50858_u16 as i32;
RET = 82_u8;
RET = 186_u8 & 188_u8;
_10.0 = _9.0;
_5 = 112883807087380632418723044090422453823_u128 & 31656788761250882850650998207659103937_u128;
_9.3 = [1087588246_u32];
_10.0 = [(-49_i8),52_i8,(-103_i8)];
_6 = _1 <= _16.fld0;
_11 = RET as isize;
_11 = 9223372036854775807_isize;
RET = !198_u8;
_9 = _10;
match _16.fld0 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb8,
4 => bb9,
12068902383141738688 => bb11,
_ => bb10
}
}
bb8 = {
_9.1 = _3;
_10.0 = [(-75_i8),(-33_i8),(-127_i8)];
_1 = _16.fld0 + _16.fld0;
_10.0 = [(-98_i8),(-121_i8),(-76_i8)];
_10 = (_9.0, _3, _8, _9.3);
_10 = _9;
RET = 183_u8;
_9.3 = [1474144839_u32];
_7 = _11 as i32;
(*_15) = !(-9223372036854775808_isize);
_18 = _16.fld0 as f32;
_9 = (_10.0, _10.1, _2, _10.3);
_8 = !_9.2;
_17 = [(-73150176092150300177576194868298149891_i128),144919834067969969483814896204699403692_i128,(-25792042959353848756215489677465354213_i128)];
_15 = core::ptr::addr_of_mut!((*_15));
_5 = 13811445040443044819810148653977471013_u128 - 288546697185657601360127847226488399671_u128;
_2 = -_10.2;
_8 = _10.2;
_9.2 = 128187248762750807201386174946679554313_i128 as i32;
_15 = core::ptr::addr_of_mut!(_11);
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_10.2 = _5 as i32;
_2 = _4;
(*_15) = (-9223372036854775808_isize) | 15_isize;
_1 = _6 as u64;
_10.0 = _9.0;
_24 = 4487_u16;
_10.2 = _2;
_10.1 = _9.1;
_11 = !(-77_isize);
_14 = core::ptr::addr_of_mut!(_24);
_9 = (_10.0, _3, _2, _10.3);
_6 = true | true;
_9.2 = _10.2;
Goto(bb12)
}
bb12 = {
_9 = (_10.0, _10.1, _10.2, _10.3);
_7 = _2;
_16.fld0 = !_1;
_15 = core::ptr::addr_of_mut!(_11);
_25 = !_6;
_11 = 118_isize * (-70_isize);
(*_15) = 9223372036854775807_isize + (-9223372036854775808_isize);
(*_15) = (-71_isize) & (-34_isize);
_24 = RET as u16;
RET = 229_u8 * 11_u8;
Goto(bb13)
}
bb13 = {
_17 = [169518795002978814978392954846052639260_i128,(-125580388891280406689051678294385826903_i128),133435182848930633555967602895401199314_i128];
(*_14) = 1412170946_u32 as u16;
_10.3 = [1617882466_u32];
_16 = Adt35 { fld0: _1 };
_10.2 = _9.2;
_15 = core::ptr::addr_of_mut!((*_15));
_4 = -_7;
_9.1 = _3;
_4 = _9.2 >> _1;
_9.0 = [42_i8,(-120_i8),(-50_i8)];
RET = _18 as u8;
_26 = core::ptr::addr_of!(_31);
_10.2 = _7 & _9.2;
_10.3 = [2232495907_u32];
_3 = _9.1;
_10.1 = _3;
(*_14) = !15859_u16;
_4 = _9.2;
_15 = core::ptr::addr_of_mut!((*_15));
Goto(bb14)
}
bb14 = {
_9 = (_10.0, _3, _7, _10.3);
_32 = [(-1833350154519448717_i64)];
(*_26) = _16.fld0 as f64;
_30.0 = 21258_i16 & (-30810_i16);
(*_15) = 9223372036854775807_isize << _9.2;
_35 = !_1;
_10.2 = _4 << (*_15);
(*_14) = 1988_u16 * 10473_u16;
_9 = (_10.0, _10.1, _10.2, _10.3);
_28 = -_18;
_2 = -_9.2;
_10 = _9;
_11 = 9223372036854775807_isize;
_38.0 = 115_i8 >> _7;
_30.1 = _30.0;
_38.1 = Move(_26);
RET = 165_u8 ^ 11_u8;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(17_usize, 11_usize, Move(_11), 9_usize, Move(_9), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(17_usize, 2_usize, Move(_2), 3_usize, Move(_3), 1_usize, Move(_1), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: &'static isize) -> u8 {
mir! {
type RET = u8;
let _2: char;
let _3: *mut Adt31;
let _4: *mut *const *mut Adt20;
let _5: u8;
let _6: [i64; 4];
let _7: u32;
let _8: i8;
let _9: i64;
let _10: isize;
let _11: isize;
let _12: [isize; 5];
let _13: [i32; 5];
let _14: Adt48;
let _15: char;
let _16: ((i16, f32, *mut u32, [i32; 6]), [isize; 5], f32);
let _17: i128;
let _18: *mut u16;
let _19: Adt48;
let _20: i16;
let _21: &'static usize;
let _22: ([i16; 2], &'static *const u32);
let _23: f64;
let _24: Adt74;
let _25: *mut [i16; 2];
let _26: u32;
let _27: u64;
let _28: bool;
let _29: f32;
let _30: *mut *mut Adt31;
let _31: f64;
let _32: *mut isize;
let _33: *const ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1]));
let _34: [u8; 4];
let _35: [i64; 4];
let _36: ();
let _37: ();
{
RET = !103_u8;
RET = 74_u8 << 41444_u16;
RET = 267397346941103494042239477622341438303_u128 as u8;
RET = 75_u8 >> 9128_u16;
RET = 20_u8;
RET = 222_u8;
RET = 4_u8 * 2_u8;
RET = 59_u8 << (-70712713780824861129801886140961699240_i128);
RET = !246_u8;
RET = !116_u8;
RET = '\u{f309e}' as u8;
_2 = '\u{98d5d}';
RET = 194266679799009325_i64 as u8;
RET = 91_u8 | 170_u8;
Goto(bb1)
}
bb1 = {
RET = 231_u8;
RET = 30461_i16 as u8;
RET = 198_u8 - 45_u8;
_2 = '\u{f511b}';
RET = 11911228396822747343_u64 as u8;
RET = (-3466971547502368262_i64) as u8;
RET = 201_u8 * 172_u8;
RET = !118_u8;
Goto(bb2)
}
bb2 = {
_2 = '\u{55ff4}';
_6 = [(-7948690129368104573_i64),(-1340823279610370593_i64),8969178084636022035_i64,(-5027545258019182200_i64)];
_8 = -84_i8;
_2 = '\u{b410b}';
_6 = [8435947697909020338_i64,5976373341066052914_i64,6425245761270760890_i64,(-4907700599535211636_i64)];
RET = 64_u8 - 124_u8;
_2 = '\u{cff63}';
_9 = (-321348472466441580_i64) >> RET;
Goto(bb3)
}
bb3 = {
_7 = 9223372036854775807_isize as u32;
_7 = 3043274275_u32 * 4135182079_u32;
_6 = [_9,_9,_9,_9];
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_8 = -(-23_i8);
_12 = [_10,_10,_10,_10,_10];
_6 = [_9,_9,_9,_9];
Goto(bb4)
}
bb4 = {
_11 = -_10;
_5 = true as u8;
RET = _10 as u8;
RET = 6_usize as u8;
_1 = &_11;
RET = !_5;
RET = !_5;
_11 = _7 as isize;
_6 = [_9,_9,_9,_9];
_2 = '\u{71234}';
_13 = [349243217_i32,(-306654126_i32),1873276257_i32,(-804287125_i32),(-700378651_i32)];
_1 = &_10;
_11 = -_10;
_11 = (*_1);
_13 = [(-1134809313_i32),283574200_i32,(-432056489_i32),(-1204203671_i32),(-1744560638_i32)];
_9 = 1771372754246745678_i64;
_1 = &_11;
_2 = '\u{45376}';
_12 = [_11,_10,_11,_10,_10];
RET = _10 as u8;
_5 = RET;
_2 = '\u{ecb3f}';
_12 = [(*_1),_11,(*_1),(*_1),_10];
_16.0.2 = core::ptr::addr_of_mut!(_7);
_16.0.3 = [(-385081917_i32),634037280_i32,(-1778456235_i32),1199969583_i32,16986462_i32,1700499594_i32];
_16.2 = _7 as f32;
Goto(bb5)
}
bb5 = {
_12 = [(*_1),_10,_11,_10,_11];
_16.0.1 = _16.2 - _16.2;
_17 = -(-153384543046284925074803786439731843272_i128);
_6 = [_9,_9,_9,_9];
_6 = [_9,_9,_9,_9];
match _9 {
1771372754246745678 => bb7,
_ => bb6
}
}
bb6 = {
_7 = 9223372036854775807_isize as u32;
_7 = 3043274275_u32 * 4135182079_u32;
_6 = [_9,_9,_9,_9];
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_8 = -(-23_i8);
_12 = [_10,_10,_10,_10,_10];
_6 = [_9,_9,_9,_9];
Goto(bb4)
}
bb7 = {
_7 = 1126896017_u32;
_8 = -118_i8;
_15 = _2;
RET = _5;
RET = _5 & _5;
_16.0.0 = (-32376_i16) & 21236_i16;
_16.0.3 = [2091273702_i32,(-842816548_i32),1972308085_i32,83010581_i32,(-935497159_i32),(-1260210431_i32)];
_17 = 61107937603889500013775532601192820358_i128;
_5 = true as u8;
_16.1 = _12;
_16.1 = [_11,_10,_11,(*_1),_11];
_1 = &_10;
_9 = -(-6153819741611778579_i64);
_2 = _15;
_20 = _16.0.0 - _16.0.0;
_1 = &(*_1);
_9 = (-4393733521789950470_i64) + (-7686331149411496187_i64);
_1 = &(*_1);
_11 = (*_1) * (*_1);
_8 = !(-12_i8);
RET = _5;
_16.0.1 = -_16.2;
match _7 {
0 => bb8,
1 => bb9,
2 => bb10,
1126896017 => bb12,
_ => bb11
}
}
bb8 = {
RET = 231_u8;
RET = 30461_i16 as u8;
RET = 198_u8 - 45_u8;
_2 = '\u{f511b}';
RET = 11911228396822747343_u64 as u8;
RET = (-3466971547502368262_i64) as u8;
RET = 201_u8 * 172_u8;
RET = !118_u8;
Goto(bb2)
}
bb9 = {
_12 = [(*_1),_10,_11,_10,_11];
_16.0.1 = _16.2 - _16.2;
_17 = -(-153384543046284925074803786439731843272_i128);
_6 = [_9,_9,_9,_9];
_6 = [_9,_9,_9,_9];
match _9 {
1771372754246745678 => bb7,
_ => bb6
}
}
bb10 = {
_2 = '\u{55ff4}';
_6 = [(-7948690129368104573_i64),(-1340823279610370593_i64),8969178084636022035_i64,(-5027545258019182200_i64)];
_8 = -84_i8;
_2 = '\u{b410b}';
_6 = [8435947697909020338_i64,5976373341066052914_i64,6425245761270760890_i64,(-4907700599535211636_i64)];
RET = 64_u8 - 124_u8;
_2 = '\u{cff63}';
_9 = (-321348472466441580_i64) >> RET;
Goto(bb3)
}
bb11 = {
_7 = 9223372036854775807_isize as u32;
_7 = 3043274275_u32 * 4135182079_u32;
_6 = [_9,_9,_9,_9];
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_8 = -(-23_i8);
_12 = [_10,_10,_10,_10,_10];
_6 = [_9,_9,_9,_9];
Goto(bb4)
}
bb12 = {
_16.0.3 = [(-1461151198_i32),(-1602607172_i32),(-1200870562_i32),1797991961_i32,299515_i32,(-311460299_i32)];
_9 = (-7942320877893837059_i64);
_20 = _16.0.0 << _17;
_2 = _15;
_16.1 = [_11,_11,_10,(*_1),(*_1)];
_17 = true as i128;
_23 = _11 as f64;
_16.2 = _16.0.1;
_16.2 = 16821718368451180927_usize as f32;
_22.0 = [_16.0.0,_20];
_16.0.3 = [390197119_i32,(-2120796020_i32),407271209_i32,(-1818507643_i32),873179326_i32,299504395_i32];
_16.2 = _16.0.1 + _16.0.1;
_10 = 961550844_i32 as isize;
_16.1 = [_11,_11,_11,_11,_11];
_16.0.2 = core::ptr::addr_of_mut!(_26);
_22.0 = [_20,_20];
_8 = true as i8;
_30 = core::ptr::addr_of_mut!(_3);
_16.0.2 = core::ptr::addr_of_mut!(_7);
_9 = !5904074846594931446_i64;
_26 = !_7;
_25 = core::ptr::addr_of_mut!(_22.0);
_1 = &_10;
Goto(bb13)
}
bb13 = {
(*_25) = [_20,_16.0.0];
(*_25) = [_16.0.0,_16.0.0];
_5 = !RET;
_31 = _23;
_17 = _15 as i128;
_16.0.0 = _8 as i16;
_15 = _2;
_29 = _16.2 - _16.2;
_16.2 = _29 * _29;
_27 = 4424319727654365003_u64 & 5097661985558304387_u64;
Goto(bb14)
}
bb14 = {
_32 = core::ptr::addr_of_mut!((*_1));
_11 = true as isize;
_1 = &_10;
_35 = _6;
_22.0 = [_20,_20];
_17 = 60184579460756383101995338466474106136_i128 & 3573004519114623896568327463714076141_i128;
_8 = (-19_i8);
_17 = !(-53027115088972238348819604825684215953_i128);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(18_usize, 35_usize, Move(_35), 26_usize, Move(_26), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(18_usize, 12_usize, Move(_12), 6_usize, Move(_6), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-78_i8)), std::hint::black_box('\u{efb4e}'));
                
            }
impl PrintFDebug for Adt20{
	unsafe fn printf_debug(&self){unsafe{printf("Adt20::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt20 {
Variant0{
fld0: f64,
fld1: usize,
fld2: i128,
fld3: u16,

},
Variant1{
fld0: u8,
fld1: char,

}}
impl PrintFDebug for Adt28{
	unsafe fn printf_debug(&self){unsafe{printf("Adt28::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt28 {
Variant0{
fld0: bool,
fld1: [isize; 5],
fld2: u16,
fld3: usize,
fld4: (i8, *const f64, i16, i32),
fld5: [i32; 6],
fld6: f32,

},
Variant1{
fld0: [u32; 1],
fld1: u32,
fld2: u8,
fld3: i8,
fld4: i16,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: bool,
fld1: u128,
fld2: [u16; 3],
fld3: i8,
fld4: [isize; 5],
fld5: Adt28,
fld6: u8,
fld7: Adt20,

},
Variant1{
fld0: bool,
fld1: usize,
fld2: [i8; 3],
fld3: f32,
fld4: [i32; 6],

},
Variant2{
fld0: *const *mut Adt20,
fld1: u16,
fld2: (i8, *const f64, i16, i32),

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt35{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt35 {
fld0: u64,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1])),
fld1: Adt20,
fld2: u128,
fld3: *mut Adt31,
fld4: Adt28,
fld5: *const *mut Adt20,
fld6: u64,
fld7: [i8; 3],

},
Variant1{
fld0: Adt28,
fld1: Adt31,
fld2: [isize; 5],
fld3: ([i32; 6], isize, ([i8; 3], char, i32, [u32; 1])),
fld4: [u8; 4],
fld5: *mut Adt31,
fld6: [u16; 3],

}}
impl PrintFDebug for Adt67{
	unsafe fn printf_debug(&self){unsafe{printf("Adt67::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt67 {
Variant0{
fld0: u64,
fld1: *const i16,
fld2: *mut *const *mut Adt20,
fld3: f32,
fld4: *mut *mut Adt31,

},
Variant1{
fld0: bool,
fld1: Adt28,
fld2: [i32; 6],
fld3: u32,
fld4: [usize; 8],
fld5: i32,
fld6: [u16; 3],
fld7: i128,

},
Variant2{
fld0: i8,
fld1: Adt48,
fld2: isize,

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){unsafe{printf("Adt74::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: [u32; 1],

},
Variant1{
fld0: *mut [i16; 2],
fld1: Adt20,
fld2: (i16, i16, [i16; 2]),
fld3: [u8; 7],

},
Variant2{
fld0: f64,
fld1: i32,

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: u128,
fld1: Adt28,
fld2: [i32; 6],
fld3: Adt74,
fld4: (f32,),

},
Variant1{
fld0: bool,
fld1: *const *mut Adt20,
fld2: [u16; 3],
fld3: usize,
fld4: i16,
fld5: ((i8, *const f64, i16, i32),),

}}

