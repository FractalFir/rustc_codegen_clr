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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: isize,mut _4: i8,mut _5: i128,mut _6: i32) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _7: isize;
let _8: (usize, f32, Adt18, &'static usize);
let _9: *const (bool, u16, [u32; 1], u16);
let _10: u8;
let _11: bool;
let _12: Adt44;
let _13: f64;
let _14: isize;
let _15: isize;
let _16: [u32; 1];
let _17: *const char;
let _18: bool;
let _19: &'static [i16; 6];
let _20: f64;
let _21: i8;
let _22: &'static i16;
let _23: f64;
let _24: isize;
let _25: i8;
let _26: [i16; 1];
let _27: (u8,);
let _28: ();
let _29: ();
{
_6 = !(-787621050_i32);
RET = [94_u8,122_u8,96_u8,194_u8,124_u8,181_u8,243_u8];
RET = [54_u8,213_u8,237_u8,252_u8,249_u8,170_u8,139_u8];
RET = [19_u8,212_u8,201_u8,104_u8,242_u8,136_u8,116_u8];
_8.1 = 15643716258588945406_u64 as f32;
RET = [87_u8,102_u8,159_u8,58_u8,208_u8,161_u8,41_u8];
RET = [209_u8,98_u8,43_u8,198_u8,67_u8,211_u8,238_u8];
_8.3 = &_8.0;
_4 = (-5804370606299176079_i64) as i8;
_8.3 = &_8.0;
RET = [91_u8,0_u8,140_u8,129_u8,206_u8,185_u8,235_u8];
Goto(bb1)
}
bb1 = {
_2 = !177_u8;
_6 = 1995380697_i32;
_4 = 8_i8 & (-33_i8);
_7 = (-9223372036854775808_isize) << _2;
_1 = true;
_8.3 = &_8.0;
_7 = (-9223372036854775808_isize);
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_8.0 = !5_usize;
_5 = (-10333002490668214908051814157853643821_i128) ^ 138602483927091063141664188882465794895_i128;
_11 = _1;
_1 = _8.1 < _8.1;
_3 = -_7;
_8.3 = &_8.0;
_7 = _8.0 as isize;
_8.3 = &_8.0;
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_10 = (-1605_i16) as u8;
_7 = _3;
_8.0 = (-5451797400316235635_i64) as usize;
_8.3 = &_8.0;
_2 = _10 - _10;
Goto(bb2)
}
bb2 = {
_11 = !_1;
_10 = _2 << _5;
_7 = _3;
Call(_9 = fn1(), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8.0 = 22858_u16 as usize;
_2 = _8.1 as u8;
RET = [_2,_10,_10,_10,_2,_10,_10];
_8.3 = &_8.0;
match _6 {
0 => bb4,
1995380697 => bb6,
_ => bb5
}
}
bb4 = {
_11 = !_1;
_10 = _2 << _5;
_7 = _3;
Call(_9 = fn1(), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = !177_u8;
_6 = 1995380697_i32;
_4 = 8_i8 & (-33_i8);
_7 = (-9223372036854775808_isize) << _2;
_1 = true;
_8.3 = &_8.0;
_7 = (-9223372036854775808_isize);
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_8.0 = !5_usize;
_5 = (-10333002490668214908051814157853643821_i128) ^ 138602483927091063141664188882465794895_i128;
_11 = _1;
_1 = _8.1 < _8.1;
_3 = -_7;
_8.3 = &_8.0;
_7 = _8.0 as isize;
_8.3 = &_8.0;
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_10 = (-1605_i16) as u8;
_7 = _3;
_8.0 = (-5451797400316235635_i64) as usize;
_8.3 = &_8.0;
_2 = _10 - _10;
Goto(bb2)
}
bb6 = {
_8.0 = _3 as usize;
_1 = _11;
_1 = !_11;
_6 = (-1825714490_i32) << _10;
_13 = 26238_u16 as f64;
_4 = -56_i8;
_6 = 2109002812_i32;
RET = [_10,_10,_10,_10,_10,_10,_2];
RET = [_10,_10,_10,_10,_10,_10,_10];
_7 = -_3;
_11 = _2 >= _10;
_7 = !_3;
RET = [_2,_10,_10,_2,_10,_10,_10];
_5 = _8.0 as i128;
_5 = _4 as i128;
_8.0 = 8274138783886427391_usize;
_8.2 = Adt18::Variant1 { fld0: _11,fld1: 3939566829_u32,fld2: _13,fld3: _5,fld4: _2,fld5: _6,fld6: 3259087088254405713_i64 };
place!(Field::<i64>(Variant(_8.2, 1), 6)) = 3269696790582553094_i64 + (-2186086589431503743_i64);
_15 = !_3;
_1 = _11 <= _11;
_8.0 = 2_usize;
_4 = (-4857_i16) as i8;
_14 = _15;
place!(Field::<u32>(Variant(_8.2, 1), 1)) = 2932432241_u32 * 2455089011_u32;
place!(Field::<bool>(Variant(_8.2, 1), 0)) = _1;
Call(_2 = fn16(_1, Field::<i32>(Variant(_8.2, 1), 5), Move(_9), _8.2, _8.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = _10 > _2;
Call(place!(Field::<u8>(Variant(_8.2, 1), 4)) = core::intrinsics::bswap(_2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = (-68_i8);
_20 = _2 as f64;
_18 = !_11;
_10 = !Field::<u8>(Variant(_8.2, 1), 4);
place!(Field::<i128>(Variant(_8.2, 1), 3)) = 5152129770487774507_u64 as i128;
place!(Field::<u8>(Variant(_8.2, 1), 4)) = 10024796339981099350_u64 as u8;
RET = [_2,_2,Field::<u8>(Variant(_8.2, 1), 4),_10,_2,_2,Field::<u8>(Variant(_8.2, 1), 4)];
_8.3 = &_8.0;
place!(Field::<i128>(Variant(_8.2, 1), 3)) = -_5;
SetDiscriminant(_8.2, 0);
_5 = (-16274790320131070847006198414639752745_i128) >> _6;
place!(Field::<i64>(Variant(_8.2, 0), 2)) = 7495380132604677814_i64 >> _6;
place!(Field::<(u8, isize)>(Variant(_8.2, 0), 0)).1 = !_14;
place!(Field::<(u8, isize)>(Variant(_8.2, 0), 0)).1 = _7;
place!(Field::<i8>(Variant(_8.2, 0), 3)) = Field::<i64>(Variant(_8.2, 0), 2) as i8;
place!(Field::<f32>(Variant(_8.2, 0), 4)) = _8.1;
match _4 {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
340282366920938463463374607431768211388 => bb15,
_ => bb14
}
}
bb9 = {
_1 = _10 > _2;
Call(place!(Field::<u8>(Variant(_8.2, 1), 4)) = core::intrinsics::bswap(_2), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_8.0 = _3 as usize;
_1 = _11;
_1 = !_11;
_6 = (-1825714490_i32) << _10;
_13 = 26238_u16 as f64;
_4 = -56_i8;
_6 = 2109002812_i32;
RET = [_10,_10,_10,_10,_10,_10,_2];
RET = [_10,_10,_10,_10,_10,_10,_10];
_7 = -_3;
_11 = _2 >= _10;
_7 = !_3;
RET = [_2,_10,_10,_2,_10,_10,_10];
_5 = _8.0 as i128;
_5 = _4 as i128;
_8.0 = 8274138783886427391_usize;
_8.2 = Adt18::Variant1 { fld0: _11,fld1: 3939566829_u32,fld2: _13,fld3: _5,fld4: _2,fld5: _6,fld6: 3259087088254405713_i64 };
place!(Field::<i64>(Variant(_8.2, 1), 6)) = 3269696790582553094_i64 + (-2186086589431503743_i64);
_15 = !_3;
_1 = _11 <= _11;
_8.0 = 2_usize;
_4 = (-4857_i16) as i8;
_14 = _15;
place!(Field::<u32>(Variant(_8.2, 1), 1)) = 2932432241_u32 * 2455089011_u32;
place!(Field::<bool>(Variant(_8.2, 1), 0)) = _1;
Call(_2 = fn16(_1, Field::<i32>(Variant(_8.2, 1), 5), Move(_9), _8.2, _8.2), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_2 = !177_u8;
_6 = 1995380697_i32;
_4 = 8_i8 & (-33_i8);
_7 = (-9223372036854775808_isize) << _2;
_1 = true;
_8.3 = &_8.0;
_7 = (-9223372036854775808_isize);
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_8.0 = !5_usize;
_5 = (-10333002490668214908051814157853643821_i128) ^ 138602483927091063141664188882465794895_i128;
_11 = _1;
_1 = _8.1 < _8.1;
_3 = -_7;
_8.3 = &_8.0;
_7 = _8.0 as isize;
_8.3 = &_8.0;
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_10 = (-1605_i16) as u8;
_7 = _3;
_8.0 = (-5451797400316235635_i64) as usize;
_8.3 = &_8.0;
_2 = _10 - _10;
Goto(bb2)
}
bb12 = {
_2 = !177_u8;
_6 = 1995380697_i32;
_4 = 8_i8 & (-33_i8);
_7 = (-9223372036854775808_isize) << _2;
_1 = true;
_8.3 = &_8.0;
_7 = (-9223372036854775808_isize);
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_8.0 = !5_usize;
_5 = (-10333002490668214908051814157853643821_i128) ^ 138602483927091063141664188882465794895_i128;
_11 = _1;
_1 = _8.1 < _8.1;
_3 = -_7;
_8.3 = &_8.0;
_7 = _8.0 as isize;
_8.3 = &_8.0;
_8.3 = &_8.0;
RET = [_2,_2,_2,_2,_2,_2,_2];
_10 = (-1605_i16) as u8;
_7 = _3;
_8.0 = (-5451797400316235635_i64) as usize;
_8.3 = &_8.0;
_2 = _10 - _10;
Goto(bb2)
}
bb13 = {
_8.0 = 22858_u16 as usize;
_2 = _8.1 as u8;
RET = [_2,_10,_10,_10,_2,_10,_10];
_8.3 = &_8.0;
match _6 {
0 => bb4,
1995380697 => bb6,
_ => bb5
}
}
bb14 = {
_11 = !_1;
_10 = _2 << _5;
_7 = _3;
Call(_9 = fn1(), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
place!(Field::<(u8, isize)>(Variant(_8.2, 0), 0)) = (_2, _14);
place!(Field::<char>(Variant(_8.2, 0), 1)) = '\u{2990a}';
_11 = _18;
place!(Field::<char>(Variant(_8.2, 0), 1)) = '\u{8b1c4}';
SetDiscriminant(_8.2, 1);
_16 = [905868414_u32];
_21 = _15 as i8;
_7 = _3;
_7 = _14 + _3;
_24 = !_15;
place!(Field::<i64>(Variant(_8.2, 1), 6)) = _8.0 as i64;
_25 = !_4;
_1 = _10 <= _2;
_1 = _2 < _2;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(0_usize, 14_usize, Move(_14), 7_usize, Move(_7), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(0_usize, 5_usize, Move(_5), 11_usize, Move(_11), 16_usize, Move(_16), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1() -> *const (bool, u16, [u32; 1], u16) {
mir! {
type RET = *const (bool, u16, [u32; 1], u16);
let _1: f64;
let _2: u32;
let _3: bool;
let _4: [bool; 8];
let _5: *mut i128;
let _6: u32;
let _7: [i16; 1];
let _8: i32;
let _9: &'static bool;
let _10: ((u8,), (usize, f32, Adt18, &'static usize), [i64; 4], (&'static i64,));
let _11: *const u128;
let _12: char;
let _13: f32;
let _14: &'static [i16; 7];
let _15: ((u8,), (usize, f32, Adt18, &'static usize), [i64; 4], (&'static i64,));
let _16: isize;
let _17: &'static [i16; 7];
let _18: isize;
let _19: f32;
let _20: isize;
let _21: ();
let _22: ();
{
_1 = 6658307311031302605_i64 as f64;
_2 = 3899033497_u32;
_2 = !958965488_u32;
_3 = _2 == _2;
_2 = 3318110056_u32;
_2 = _3 as u32;
_3 = !true;
_2 = 115423332085898867077811371900871795389_i128 as u32;
_1 = 5_usize as f64;
_1 = 121959171628459452936957054514465684985_i128 as f64;
_2 = !2771090609_u32;
Goto(bb1)
}
bb1 = {
_3 = true;
_2 = 2005435454_u32;
_2 = 229086521_u32 + 1831411970_u32;
_2 = 4193328132_u32 & 4093981286_u32;
_1 = 28823_u16 as f64;
_3 = false;
_1 = (-6555424007603698489_i64) as f64;
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = false ^ false;
Call(RET = fn2(_2, _2, _2, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = [(-17906_i16)];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7 = [15238_i16];
_6 = !_2;
_8 = (-1903798366_i32);
_8 = 415495384_i32;
_3 = _1 < _1;
_7 = [25255_i16];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = !_6;
_10.1.2 = Adt18::Variant1 { fld0: _3,fld1: _6,fld2: _1,fld3: 169991407770512632695268588840004204279_i128,fld4: 47_u8,fld5: _8,fld6: (-2181569489138993391_i64) };
_10.1.3 = &_10.1.0;
_10.0 = (192_u8,);
_4 = [Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,_3,Field::<bool>(Variant(_10.1.2, 1), 0)];
_10.0 = (91_u8,);
_10.1.1 = _6 as f32;
_5 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_10.1.2, 1), 3)));
_3 = Field::<bool>(Variant(_10.1.2, 1), 0);
_6 = (-25529_i16) as u32;
match _8 {
415495384 => bb3,
_ => bb1
}
}
bb3 = {
_5 = core::ptr::addr_of_mut!((*_5));
place!(Field::<bool>(Variant(_10.1.2, 1), 0)) = !_3;
place!(Field::<i128>(Variant(_10.1.2, 1), 3)) = _3 as i128;
_13 = _10.1.1;
_10.1.3 = &_10.1.0;
_5 = core::ptr::addr_of_mut!((*_5));
_10.1.3 = &_10.1.0;
_12 = '\u{b18bd}';
_10.1.3 = &_10.1.0;
place!(Field::<i128>(Variant(_10.1.2, 1), 3)) = 47814972299600840735671526976535078178_i128;
place!(Field::<u32>(Variant(_10.1.2, 1), 1)) = 29_isize as u32;
_12 = '\u{30e29}';
_10.0 = (45_u8,);
Goto(bb4)
}
bb4 = {
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
_4 = [Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3];
_6 = !_2;
place!(Field::<i64>(Variant(_10.1.2, 1), 6)) = (-9223372036854775808_isize) as i64;
_10.1.3 = &_10.1.0;
place!(Field::<f64>(Variant(_10.1.2, 1), 2)) = _1 + _1;
_10.2 = [Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6)];
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
match _8 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
415495384 => bb10,
_ => bb9
}
}
bb5 = {
_5 = core::ptr::addr_of_mut!((*_5));
place!(Field::<bool>(Variant(_10.1.2, 1), 0)) = !_3;
place!(Field::<i128>(Variant(_10.1.2, 1), 3)) = _3 as i128;
_13 = _10.1.1;
_10.1.3 = &_10.1.0;
_5 = core::ptr::addr_of_mut!((*_5));
_10.1.3 = &_10.1.0;
_12 = '\u{b18bd}';
_10.1.3 = &_10.1.0;
place!(Field::<i128>(Variant(_10.1.2, 1), 3)) = 47814972299600840735671526976535078178_i128;
place!(Field::<u32>(Variant(_10.1.2, 1), 1)) = 29_isize as u32;
_12 = '\u{30e29}';
_10.0 = (45_u8,);
Goto(bb4)
}
bb6 = {
_7 = [(-17906_i16)];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7 = [15238_i16];
_6 = !_2;
_8 = (-1903798366_i32);
_8 = 415495384_i32;
_3 = _1 < _1;
_7 = [25255_i16];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = !_6;
_10.1.2 = Adt18::Variant1 { fld0: _3,fld1: _6,fld2: _1,fld3: 169991407770512632695268588840004204279_i128,fld4: 47_u8,fld5: _8,fld6: (-2181569489138993391_i64) };
_10.1.3 = &_10.1.0;
_10.0 = (192_u8,);
_4 = [Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,_3,Field::<bool>(Variant(_10.1.2, 1), 0)];
_10.0 = (91_u8,);
_10.1.1 = _6 as f32;
_5 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_10.1.2, 1), 3)));
_3 = Field::<bool>(Variant(_10.1.2, 1), 0);
_6 = (-25529_i16) as u32;
match _8 {
415495384 => bb3,
_ => bb1
}
}
bb7 = {
_3 = true;
_2 = 2005435454_u32;
_2 = 229086521_u32 + 1831411970_u32;
_2 = 4193328132_u32 & 4093981286_u32;
_1 = 28823_u16 as f64;
_3 = false;
_1 = (-6555424007603698489_i64) as f64;
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = false ^ false;
Call(RET = fn2(_2, _2, _2, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
match Field::<i32>(Variant(_10.1.2, 1), 5) {
0 => bb7,
415495384 => bb12,
_ => bb11
}
}
bb11 = {
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
_4 = [Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3];
_6 = !_2;
place!(Field::<i64>(Variant(_10.1.2, 1), 6)) = (-9223372036854775808_isize) as i64;
_10.1.3 = &_10.1.0;
place!(Field::<f64>(Variant(_10.1.2, 1), 2)) = _1 + _1;
_10.2 = [Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6)];
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
match _8 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
415495384 => bb10,
_ => bb9
}
}
bb12 = {
_15.1.2 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_10.1.2, 1), 0),fld1: Field::<u32>(Variant(_10.1.2, 1), 1),fld2: Field::<f64>(Variant(_10.1.2, 1), 2),fld3: Field::<i128>(Variant(_10.1.2, 1), 3),fld4: _10.0.0,fld5: Field::<i32>(Variant(_10.1.2, 1), 5),fld6: Field::<i64>(Variant(_10.1.2, 1), 6) };
_5 = core::ptr::addr_of_mut!((*_5));
_2 = !_6;
place!(Field::<i64>(Variant(_10.1.2, 1), 6)) = Field::<i64>(Variant(_15.1.2, 1), 6);
_9 = &_3;
(*_5) = Field::<i128>(Variant(_15.1.2, 1), 3);
_15.0.0 = _10.0.0 << _2;
place!(Field::<u8>(Variant(_15.1.2, 1), 4)) = !_10.0.0;
_15.0 = _10.0;
_9 = &place!(Field::<bool>(Variant(_10.1.2, 1), 0));
_15.1.3 = &_15.1.0;
_10.1.3 = &_15.1.0;
place!(Field::<i32>(Variant(_15.1.2, 1), 5)) = -Field::<i32>(Variant(_10.1.2, 1), 5);
_15.3.0 = &place!(Field::<i64>(Variant(_15.1.2, 1), 6));
place!(Field::<bool>(Variant(_15.1.2, 1), 0)) = Field::<bool>(Variant(_10.1.2, 1), 0);
_16 = Field::<u32>(Variant(_15.1.2, 1), 1) as isize;
(*_5) = Field::<i128>(Variant(_15.1.2, 1), 3);
(*_5) = Field::<i128>(Variant(_15.1.2, 1), 3);
_15.1.0 = 0_usize;
place!(Field::<u32>(Variant(_15.1.2, 1), 1)) = _6 | Field::<u32>(Variant(_10.1.2, 1), 1);
_19 = _10.1.1 * _13;
_5 = core::ptr::addr_of_mut!((*_5));
_15.2 = [Field::<i64>(Variant(_15.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_15.1.2, 1), 6),Field::<i64>(Variant(_15.1.2, 1), 6)];
(*_5) = -Field::<i128>(Variant(_15.1.2, 1), 3);
place!(Field::<u8>(Variant(_10.1.2, 1), 4)) = Field::<bool>(Variant(_15.1.2, 1), 0) as u8;
match Field::<i32>(Variant(_10.1.2, 1), 5) {
0 => bb11,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb10,
415495384 => bb14,
_ => bb13
}
}
bb13 = {
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
_4 = [Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3,Field::<bool>(Variant(_10.1.2, 1), 0),_3];
_6 = !_2;
place!(Field::<i64>(Variant(_10.1.2, 1), 6)) = (-9223372036854775808_isize) as i64;
_10.1.3 = &_10.1.0;
place!(Field::<f64>(Variant(_10.1.2, 1), 2)) = _1 + _1;
_10.2 = [Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6)];
_10.3.0 = &place!(Field::<i64>(Variant(_10.1.2, 1), 6));
match _8 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
415495384 => bb10,
_ => bb9
}
}
bb14 = {
_10.3 = (Move(_15.3.0),);
_15.2 = [Field::<i64>(Variant(_15.1.2, 1), 6),Field::<i64>(Variant(_10.1.2, 1), 6),Field::<i64>(Variant(_15.1.2, 1), 6),Field::<i64>(Variant(_15.1.2, 1), 6)];
_20 = -_16;
_15.2 = _10.2;
_15.1.0 = !5_usize;
_9 = &_3;
place!(Field::<i128>(Variant(_15.1.2, 1), 3)) = (*_5) ^ (*_5);
place!(Field::<i64>(Variant(_10.1.2, 1), 6)) = -Field::<i64>(Variant(_15.1.2, 1), 6);
place!(Field::<i128>(Variant(_10.1.2, 1), 3)) = Field::<i128>(Variant(_15.1.2, 1), 3);
place!(Field::<i32>(Variant(_15.1.2, 1), 5)) = -Field::<i32>(Variant(_10.1.2, 1), 5);
place!(Field::<bool>(Variant(_10.1.2, 1), 0)) = Field::<u32>(Variant(_15.1.2, 1), 1) < _2;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(1_usize, 4_usize, Move(_4), 6_usize, Move(_6), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u32,mut _2: u32,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: u32) -> *const (bool, u16, [u32; 1], u16) {
mir! {
type RET = *const (bool, u16, [u32; 1], u16);
let _7: isize;
let _8: char;
let _9: &'static bool;
let _10: u32;
let _11: isize;
let _12: Adt48;
let _13: *mut &'static (u8, isize);
let _14: (Adt33, (u8,), (&'static (u8,),), u32);
let _15: Adt60;
let _16: u128;
let _17: f32;
let _18: [i64; 4];
let _19: Adt45;
let _20: (Adt33, i8, Adt48);
let _21: u8;
let _22: f64;
let _23: isize;
let _24: Adt45;
let _25: &'static (u8, isize);
let _26: *mut &'static (u8, isize);
let _27: [usize; 3];
let _28: ([i16; 6],);
let _29: &'static i64;
let _30: f64;
let _31: u16;
let _32: &'static &'static i128;
let _33: char;
let _34: ([i16; 6],);
let _35: bool;
let _36: [u16; 5];
let _37: [u8; 4];
let _38: [i16; 6];
let _39: [isize; 1];
let _40: *mut u32;
let _41: (&'static (usize, f32, Adt18, &'static usize), char, u32);
let _42: &'static u128;
let _43: [u16; 5];
let _44: (&'static (u8,),);
let _45: u128;
let _46: (u8, isize);
let _47: char;
let _48: &'static (u8,);
let _49: f64;
let _50: i8;
let _51: (Adt33, (u8,), (&'static (u8,),), u32);
let _52: f32;
let _53: Adt75;
let _54: (usize, f32, Adt18, &'static usize);
let _55: char;
let _56: [usize; 7];
let _57: u32;
let _58: (bool, u16, [u32; 1], u16);
let _59: u128;
let _60: char;
let _61: ();
let _62: ();
{
_2 = 80204574997871735303223972824686395027_u128 as u32;
_4 = _5 | _3;
_5 = !_6;
_6 = _1;
_2 = !_4;
_7 = 66693906508288001509952399312800720912_u128 as isize;
Call(_4 = fn3(_1, _7, _1, _5, _2, _6, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 10408481057949767008_u64 as u32;
_8 = '\u{45f74}';
_11 = (-980028355223347163_i64) as isize;
_8 = '\u{95f5d}';
_11 = _7;
_8 = '\u{da50a}';
_1 = _7 as u32;
_3 = _8 as u32;
_2 = !_6;
_6 = !_2;
_1 = !_2;
_1 = 35291_u16 as u32;
_8 = '\u{17ab0}';
_2 = _1 & _3;
_14.0.fld7 = 17171119012471334666_u64 >> _5;
_1 = _5 * _2;
Goto(bb2)
}
bb2 = {
_5 = 3_usize as u32;
_14.0.fld3.0 = 175_u8;
_14.0.fld6.0 = !_14.0.fld3.0;
_14.0.fld0 = true as u16;
_2 = 86617152363474904207386679177067090227_i128 as u32;
_17 = 206517324326222204876767980315276044406_u128 as f32;
_14.0.fld3.1 = _7;
_16 = 39899960673355712775103646836674443303_u128 - 172144981485882738277086164596776519737_u128;
Goto(bb3)
}
bb3 = {
_14.0.fld1 = _8;
_19.fld5 = Adt18::Variant0 { fld0: _14.0.fld3,fld1: _14.0.fld1,fld2: 3745251676959333224_i64,fld3: (-110_i8),fld4: _17 };
_14.2.0 = &_14.1;
_14.3 = _1;
_16 = !339190717882914078032750962055211957231_u128;
_14.0.fld3.0 = 7859648147937984886_i64 as u8;
_19.fld0 = [(-764702705_i32),(-1357117045_i32),1762926395_i32,(-1895416578_i32)];
_3 = (-24542_i16) as u32;
_14.1 = (Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).0,);
_19.fld4 = _14.0.fld0;
_20.0.fld6.0 = _14.0.fld3.0;
place!(Field::<f32>(Variant(_19.fld5, 0), 4)) = _17 + _17;
_14.0.fld6.1 = _14.0.fld0 as u8;
_20.0.fld4 = [_5];
_18 = [(-2835037802897396234_i64),(-3424582842499158719_i64),(-5708859973711366059_i64),(-8645636647827734369_i64)];
_2 = _14.3 ^ _6;
_14.0.fld3.0 = _20.0.fld6.0 << _1;
_6 = _1;
_14.0.fld7 = 140662219416586492073760654494772126399_i128 as u64;
_22 = 1065100272_i32 as f64;
_14.0.fld6.0 = Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).0 >> _6;
_18 = [4398250996973368384_i64,5452413284101549716_i64,1268798891822461630_i64,(-3624898474908015892_i64)];
_8 = _14.0.fld1;
Goto(bb4)
}
bb4 = {
_19.fld1 = (_14.1.0, _14.0.fld3.1);
_23 = _14.0.fld3.1 >> _19.fld1.1;
_20.0.fld3.1 = _14.0.fld3.1;
_24.fld6 = core::ptr::addr_of!(_20.1);
Call(_24.fld3 = core::intrinsics::transmute(_14.0.fld3.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = _24.fld3;
_24.fld1.1 = 546760805_i32 as isize;
_19.fld2 = _14.0.fld6.0;
_14.0.fld6.1 = _19.fld2 & _19.fld2;
_24.fld2 = 22825_i16 as u8;
_20.0.fld6.1 = _14.0.fld3.1 as u8;
_24.fld5 = Adt18::Variant1 { fld0: false,fld1: _3,fld2: _22,fld3: (-134505373255661371119708739450888977595_i128),fld4: _14.0.fld6.1,fld5: 1901429979_i32,fld6: 2712160374271889981_i64 };
_28.0 = [8330_i16,(-11887_i16),(-11302_i16),(-11882_i16),(-30273_i16),(-28871_i16)];
_24.fld5 = Adt18::Variant1 { fld0: false,fld1: _6,fld2: _22,fld3: 114804673647944828670253127049525721613_i128,fld4: _19.fld2,fld5: (-1585161742_i32),fld6: 6238265317543335385_i64 };
_1 = _6 | _14.3;
place!(Field::<u8>(Variant(_24.fld5, 1), 4)) = _19.fld2 & _14.0.fld3.0;
match _14.1.0 {
0 => bb1,
1 => bb3,
175 => bb7,
_ => bb6
}
}
bb6 = {
_5 = 3_usize as u32;
_14.0.fld3.0 = 175_u8;
_14.0.fld6.0 = !_14.0.fld3.0;
_14.0.fld0 = true as u16;
_2 = 86617152363474904207386679177067090227_i128 as u32;
_17 = 206517324326222204876767980315276044406_u128 as f32;
_14.0.fld3.1 = _7;
_16 = 39899960673355712775103646836674443303_u128 - 172144981485882738277086164596776519737_u128;
Goto(bb3)
}
bb7 = {
place!(Field::<i32>(Variant(_24.fld5, 1), 5)) = 278556835_i32;
_30 = _22 + Field::<f64>(Variant(_24.fld5, 1), 2);
_24.fld7 = Adt44::Variant1 { fld0: false };
_20.0.fld6.0 = Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).0;
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = _24.fld3 >> _3;
_14.1 = (_14.0.fld3.0,);
_20.0.fld3 = (_14.0.fld6.1, _24.fld1.1);
Call(_20.0.fld1 = fn4(_20.0.fld3, _20.0.fld3.0, _14.0.fld3.0, _20.0.fld3, Field::<u32>(Variant(_24.fld5, 1), 1), _14.0.fld3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_27 = [1892738311884289308_usize,1072757585412545924_usize,1_usize];
place!(Field::<u8>(Variant(_24.fld5, 1), 4)) = _14.0.fld6.1 - _19.fld2;
_20.0.fld6.0 = _14.0.fld0 as u8;
place!(Field::<i128>(Variant(_24.fld5, 1), 3)) = 59086127618860952616564632785763536608_i128 + 83986471491405463731244366849554711603_i128;
_21 = _19.fld1.0 % _19.fld1.0;
_17 = Field::<f32>(Variant(_19.fld5, 0), 4) * Field::<f32>(Variant(_19.fld5, 0), 4);
_14.0.fld3.0 = _20.0.fld3.0;
_24.fld3 = Field::<i8>(Variant(_19.fld5, 0), 3);
_10 = !_1;
_20.0.fld3 = _14.0.fld3;
_34.0 = _28.0;
match Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).0 {
0 => bb3,
175 => bb9,
_ => bb5
}
}
bb9 = {
_20.0.fld5 = [(-6262_i16),(-30795_i16),(-20419_i16),9849_i16,32729_i16,(-7764_i16),19442_i16];
_9 = &place!(Field::<bool>(Variant(_24.fld7, 1), 0));
_25 = &place!(Field::<(u8, isize)>(Variant(_19.fld5, 0), 0));
_7 = !_14.0.fld3.1;
_2 = _10 * _3;
_15 = Adt60::Variant3 { fld0: _30 };
_6 = !_1;
_17 = Field::<f32>(Variant(_19.fld5, 0), 4);
place!(Field::<i64>(Variant(_19.fld5, 0), 2)) = 9204301208257110435_i64 * 2351217553899880542_i64;
_20.0.fld2 = _20.0.fld6.0;
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = Field::<f64>(Variant(_15, 3), 0) as i8;
_14.0.fld7 = 11821302935746934200_u64;
_16 = _17 as u128;
_6 = _10;
_14.0.fld3.1 = (*_25).1;
_13 = core::ptr::addr_of_mut!(_25);
_14.0.fld3.1 = _16 as isize;
place!(Field::<u32>(Variant(_24.fld5, 1), 1)) = Field::<i32>(Variant(_24.fld5, 1), 5) as u32;
match (*_25).0 {
0 => bb6,
175 => bb10,
_ => bb3
}
}
bb10 = {
_20.0.fld3.1 = _24.fld1.1;
_20.0.fld7 = _14.0.fld7 | _14.0.fld7;
place!(Field::<bool>(Variant(_24.fld5, 1), 0)) = true ^ true;
_19.fld2 = _20.0.fld3.0;
_20.0.fld3 = (_14.0.fld6.1, Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).1);
_20.0.fld6.0 = Field::<u8>(Variant(_24.fld5, 1), 4) & _20.0.fld3.0;
_20.1 = Field::<i8>(Variant(_19.fld5, 0), 3) + _24.fld3;
Goto(bb11)
}
bb11 = {
_24.fld6 = core::ptr::addr_of!(_19.fld3);
_14.2.0 = &_14.1;
_20.0.fld0 = _14.0.fld0 * _19.fld4;
SetDiscriminant(_15, 0);
_20.0.fld6.1 = (-19846_i16) as u8;
place!(Field::<u32>(Variant(_24.fld5, 1), 1)) = !_6;
place!(Field::<f32>(Variant(_19.fld5, 0), 4)) = _2 as f32;
_18 = [Field::<i64>(Variant(_19.fld5, 0), 2),Field::<i64>(Variant(_19.fld5, 0), 2),Field::<i64>(Variant(_19.fld5, 0), 2),Field::<i64>(Variant(_19.fld5, 0), 2)];
_20.0.fld3.0 = _14.1.0;
_14.0.fld4 = _20.0.fld4;
_13 = core::ptr::addr_of_mut!(_25);
_35 = Field::<bool>(Variant(_24.fld5, 1), 0);
_31 = _24.fld1.1 as u16;
_46.0 = _14.0.fld6.1;
place!(Field::<(Adt18,)>(Variant(_15, 0), 2)).0 = Adt18::Variant0 { fld0: _14.0.fld3,fld1: _14.0.fld1,fld2: Field::<i64>(Variant(_19.fld5, 0), 2),fld3: Field::<i8>(Variant(_19.fld5, 0), 3),fld4: Field::<f32>(Variant(_19.fld5, 0), 4) };
_24.fld1.0 = !_14.0.fld6.1;
_45 = _16;
_41.1 = Field::<char>(Variant(Field::<(Adt18,)>(Variant(_15, 0), 2).0, 0), 1);
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = 4_usize as i8;
Goto(bb12)
}
bb12 = {
_33 = _14.0.fld1;
_24.fld4 = Field::<bool>(Variant(_24.fld5, 1), 0) as u16;
_24.fld3 = _20.1;
_14.0.fld2 = !Field::<u8>(Variant(_24.fld5, 1), 4);
_7 = Field::<(u8, isize)>(Variant(Field::<(Adt18,)>(Variant(_15, 0), 2).0, 0), 0).1;
place!(Field::<f64>(Variant(_24.fld5, 1), 2)) = -_30;
place!(Field::<*mut i128>(Variant(_15, 0), 4)) = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_24.fld5, 1), 3)));
Goto(bb13)
}
bb13 = {
_14.1.0 = _46.0;
place!(Field::<char>(Variant(place!(Field::<(Adt18,)>(Variant(_15, 0), 2)).0, 0), 1)) = _33;
_30 = Field::<f64>(Variant(_24.fld5, 1), 2);
_44.0 = &_14.1;
place!(Field::<Adt44>(Variant(_15, 0), 3)) = Adt44::Variant1 { fld0: _35 };
place!(Field::<(u8, isize)>(Variant(_19.fld5, 0), 0)).1 = Field::<bool>(Variant(_24.fld5, 1), 0) as isize;
place!(Field::<(u8, isize)>(Variant(_15, 0), 5)).1 = Field::<(u8, isize)>(Variant(Field::<(Adt18,)>(Variant(_15, 0), 2).0, 0), 0).1;
match _19.fld1.0 {
0 => bb6,
1 => bb12,
175 => bb14,
_ => bb5
}
}
bb14 = {
_19.fld6 = core::ptr::addr_of!(_20.1);
_15 = Adt60::Variant2 { fld0: _19.fld0,fld1: _18 };
match (*_25).0 {
0 => bb10,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
175 => bb20,
_ => bb19
}
}
bb15 = {
_5 = 3_usize as u32;
_14.0.fld3.0 = 175_u8;
_14.0.fld6.0 = !_14.0.fld3.0;
_14.0.fld0 = true as u16;
_2 = 86617152363474904207386679177067090227_i128 as u32;
_17 = 206517324326222204876767980315276044406_u128 as f32;
_14.0.fld3.1 = _7;
_16 = 39899960673355712775103646836674443303_u128 - 172144981485882738277086164596776519737_u128;
Goto(bb3)
}
bb16 = {
_33 = _14.0.fld1;
_24.fld4 = Field::<bool>(Variant(_24.fld5, 1), 0) as u16;
_24.fld3 = _20.1;
_14.0.fld2 = !Field::<u8>(Variant(_24.fld5, 1), 4);
_7 = Field::<(u8, isize)>(Variant(Field::<(Adt18,)>(Variant(_15, 0), 2).0, 0), 0).1;
place!(Field::<f64>(Variant(_24.fld5, 1), 2)) = -_30;
place!(Field::<*mut i128>(Variant(_15, 0), 4)) = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_24.fld5, 1), 3)));
Goto(bb13)
}
bb17 = {
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = _24.fld3;
_24.fld1.1 = 546760805_i32 as isize;
_19.fld2 = _14.0.fld6.0;
_14.0.fld6.1 = _19.fld2 & _19.fld2;
_24.fld2 = 22825_i16 as u8;
_20.0.fld6.1 = _14.0.fld3.1 as u8;
_24.fld5 = Adt18::Variant1 { fld0: false,fld1: _3,fld2: _22,fld3: (-134505373255661371119708739450888977595_i128),fld4: _14.0.fld6.1,fld5: 1901429979_i32,fld6: 2712160374271889981_i64 };
_28.0 = [8330_i16,(-11887_i16),(-11302_i16),(-11882_i16),(-30273_i16),(-28871_i16)];
_24.fld5 = Adt18::Variant1 { fld0: false,fld1: _6,fld2: _22,fld3: 114804673647944828670253127049525721613_i128,fld4: _19.fld2,fld5: (-1585161742_i32),fld6: 6238265317543335385_i64 };
_1 = _6 | _14.3;
place!(Field::<u8>(Variant(_24.fld5, 1), 4)) = _19.fld2 & _14.0.fld3.0;
match _14.1.0 {
0 => bb1,
1 => bb3,
175 => bb7,
_ => bb6
}
}
bb18 = {
_20.0.fld3.1 = _24.fld1.1;
_20.0.fld7 = _14.0.fld7 | _14.0.fld7;
place!(Field::<bool>(Variant(_24.fld5, 1), 0)) = true ^ true;
_19.fld2 = _20.0.fld3.0;
_20.0.fld3 = (_14.0.fld6.1, Field::<(u8, isize)>(Variant(_19.fld5, 0), 0).1);
_20.0.fld6.0 = Field::<u8>(Variant(_24.fld5, 1), 4) & _20.0.fld3.0;
_20.1 = Field::<i8>(Variant(_19.fld5, 0), 3) + _24.fld3;
Goto(bb11)
}
bb19 = {
_20.0.fld5 = [(-6262_i16),(-30795_i16),(-20419_i16),9849_i16,32729_i16,(-7764_i16),19442_i16];
_9 = &place!(Field::<bool>(Variant(_24.fld7, 1), 0));
_25 = &place!(Field::<(u8, isize)>(Variant(_19.fld5, 0), 0));
_7 = !_14.0.fld3.1;
_2 = _10 * _3;
_15 = Adt60::Variant3 { fld0: _30 };
_6 = !_1;
_17 = Field::<f32>(Variant(_19.fld5, 0), 4);
place!(Field::<i64>(Variant(_19.fld5, 0), 2)) = 9204301208257110435_i64 * 2351217553899880542_i64;
_20.0.fld2 = _20.0.fld6.0;
place!(Field::<i8>(Variant(_19.fld5, 0), 3)) = Field::<f64>(Variant(_15, 3), 0) as i8;
_14.0.fld7 = 11821302935746934200_u64;
_16 = _17 as u128;
_6 = _10;
_14.0.fld3.1 = (*_25).1;
_13 = core::ptr::addr_of_mut!(_25);
_14.0.fld3.1 = _16 as isize;
place!(Field::<u32>(Variant(_24.fld5, 1), 1)) = Field::<i32>(Variant(_24.fld5, 1), 5) as u32;
match (*_25).0 {
0 => bb6,
175 => bb10,
_ => bb3
}
}
bb20 = {
SetDiscriminant(_19.fld5, 1);
_4 = !_6;
_51.2.0 = Move(_44.0);
(*_13) = &_24.fld1;
Call(place!(Field::<f64>(Variant(_19.fld5, 1), 2)) = core::intrinsics::fmaf64(Field::<f64>(Variant(_24.fld5, 1), 2), Field::<f64>(Variant(_24.fld5, 1), 2), Field::<f64>(Variant(_24.fld5, 1), 2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_47 = _33;
_28.0 = [(-12948_i16),(-15857_i16),(-14745_i16),3112_i16,(-18484_i16),22894_i16];
_48 = &_14.1;
_24.fld1.1 = Field::<f64>(Variant(_19.fld5, 1), 2) as isize;
_20.1 = !_24.fld3;
_29 = &place!(Field::<i64>(Variant(_24.fld5, 1), 6));
_14.0.fld6 = (_20.0.fld3.0, (*_25).0);
_20.0.fld3.1 = Field::<i128>(Variant(_24.fld5, 1), 3) as isize;
_24.fld2 = _14.0.fld6.1 - Field::<u8>(Variant(_24.fld5, 1), 4);
_51.0 = _20.0;
_20.0.fld3.0 = _24.fld2 & _46.0;
_20.0.fld6.0 = Field::<i128>(Variant(_24.fld5, 1), 3) as u8;
_24.fld1 = (_20.0.fld2, _51.0.fld3.1);
_14.0 = Adt33 { fld0: _20.0.fld0,fld1: _41.1,fld2: _51.0.fld6.0,fld3: _20.0.fld3,fld4: _20.0.fld4,fld5: _51.0.fld5,fld6: _51.0.fld6,fld7: _51.0.fld7 };
_14.0.fld5 = _20.0.fld5;
_20.0.fld7 = _24.fld1.1 as u64;
_24.fld6 = Move(_19.fld6);
_19.fld7 = Adt44::Variant3 { fld0: Field::<f64>(Variant(_24.fld5, 1), 2),fld1: _20.0.fld3.0,fld2: _24.fld1.1,fld3: _10,fld4: 2611919478411030884_usize,fld5: _51.0.fld7,fld6: 1058647191172681545_i64 };
_49 = -Field::<f64>(Variant(_24.fld5, 1), 2);
_9 = &place!(Field::<bool>(Variant(_19.fld5, 1), 0));
_20.0.fld2 = Field::<f64>(Variant(_19.fld5, 1), 2) as u8;
match _19.fld1.0 {
175 => bb22,
_ => bb10
}
}
bb22 = {
RET = core::ptr::addr_of!(_58);
SetDiscriminant(_15, 0);
(*RET).1 = _14.0.fld0;
_54.1 = -_17;
place!(Field::<(u8, isize)>(Variant(_15, 0), 5)).1 = _20.0.fld3.1;
(*RET).2 = [_1];
_45 = 5_usize as u128;
(*RET).0 = _14.3 < _3;
_19.fld1.0 = _14.1.0 | _14.0.fld6.0;
_40 = core::ptr::addr_of_mut!(_4);
_19.fld3 = _20.1 | _20.1;
_51.0.fld1 = _20.0.fld1;
_24.fld5 = Adt18::Variant0 { fld0: _14.0.fld3,fld1: _14.0.fld1,fld2: (-7023326641516364664_i64),fld3: _20.1,fld4: _17 };
_20.0.fld0 = (*RET).1 ^ (*RET).1;
_54.0 = (*RET).0 as usize;
_20.0 = Adt33 { fld0: _58.1,fld1: _33,fld2: _24.fld2,fld3: _14.0.fld3,fld4: _58.2,fld5: _51.0.fld5,fld6: _14.0.fld6,fld7: _51.0.fld7 };
Goto(bb23)
}
bb23 = {
Call(_61 = dump_var(2_usize, 18_usize, Move(_18), 33_usize, Move(_33), 34_usize, Move(_34), 47_usize, Move(_47)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_61 = dump_var(2_usize, 1_usize, Move(_1), 45_usize, Move(_45), 2_usize, Move(_2), 27_usize, Move(_27)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_61 = dump_var(2_usize, 16_usize, Move(_16), 10_usize, Move(_10), 28_usize, Move(_28), 62_usize, _62), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u32,mut _2: isize,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: u32,mut _7: u32,mut _8: u32) -> u32 {
mir! {
type RET = u32;
let _9: [u16; 5];
let _10: [char; 8];
let _11: i64;
let _12: Adt75;
let _13: i32;
let _14: ();
let _15: ();
{
RET = _5 | _4;
_7 = RET;
RET = _1;
RET = _7;
RET = _7 >> _1;
_9 = [15022_u16,36091_u16,24433_u16,40932_u16,7311_u16];
_4 = !_7;
_5 = !_7;
_1 = 97273125164296945323371304669536616943_u128 as u32;
_6 = _7 << _4;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !_6;
_6 = '\u{4d7da}' as u32;
_9 = [44777_u16,65161_u16,25990_u16,37433_u16,38681_u16];
_3 = !_1;
_10 = ['\u{27d77}','\u{47051}','\u{5b2c8}','\u{d4386}','\u{25c40}','\u{67c7d}','\u{78b0e}','\u{73f1b}'];
_2 = 9223372036854775807_isize;
_8 = RET;
_8 = 27444_u16 as u32;
_5 = _7;
_2 = 22304025293546462094709708565813500972_i128 as isize;
_6 = false as u32;
RET = !_7;
_13 = (-2096652303_i32);
_4 = !RET;
_5 = _4;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(3_usize, 1_usize, Move(_1), 8_usize, Move(_8), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_14 = dump_var(3_usize, 6_usize, Move(_6), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (u8, isize),mut _2: u8,mut _3: u8,mut _4: (u8, isize),mut _5: u32,mut _6: (u8, isize)) -> char {
mir! {
type RET = char;
let _7: u64;
let _8: *mut [i16; 6];
let _9: *const char;
let _10: isize;
let _11: &'static [i16; 7];
let _12: f64;
let _13: [u8; 4];
let _14: u8;
let _15: Adt44;
let _16: i64;
let _17: i64;
let _18: [i16; 6];
let _19: u32;
let _20: f32;
let _21: i8;
let _22: Adt18;
let _23: *mut [i16; 6];
let _24: u64;
let _25: *mut [i16; 6];
let _26: [bool; 8];
let _27: [char; 8];
let _28: (bool, u16, [u32; 1], u16);
let _29: i64;
let _30: *const Adt18;
let _31: u64;
let _32: f32;
let _33: f64;
let _34: (u8,);
let _35: i128;
let _36: [bool; 8];
let _37: bool;
let _38: u8;
let _39: [i32; 4];
let _40: (bool, u16, [u32; 1], u16);
let _41: &'static Adt48;
let _42: ();
let _43: ();
{
_1.1 = !_4.1;
_6.0 = _3;
_4.1 = _6.1 * _1.1;
_7 = 1690217200525744712_u64;
_7 = !200864061983065279_u64;
RET = '\u{1cce2}';
_1.1 = 61161456_i32 as isize;
_4.1 = 8710026324713734802_i64 as isize;
_6.0 = _1.0;
_6.0 = !_2;
_5 = 436973251_i32 as u32;
_6.0 = _4.0 + _4.0;
_2 = (-9394_i16) as u8;
_6.1 = _7 as isize;
_1 = (_4.0, _6.1);
_10 = _4.1 * _1.1;
_3 = (-6347070702468256409_i64) as u8;
Call(_6.0 = fn5(_4.0, _4, _1, _7, _4.0, RET, _4, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = (_4.0, _6.1);
_4.1 = !_6.1;
_9 = core::ptr::addr_of!(RET);
Goto(bb2)
}
bb2 = {
_6 = (_4.0, _1.1);
_13 = [_4.0,_1.0,_4.0,_1.0];
_13 = [_4.0,_1.0,_1.0,_1.0];
_5 = 8703448297550073087_i64 as u32;
(*_9) = '\u{1d83e}';
_6.1 = !_4.1;
_5 = 3242205064_u32;
_4.0 = _6.0 >> _6.0;
_14 = !_4.0;
_7 = 1450163211399271788_u64 | 11259698863076079514_u64;
_17 = (-4932862540261060682_i64) & (-1044422925070274930_i64);
_12 = 1524377300_i32 as f64;
(*_9) = '\u{10731f}';
RET = '\u{73b23}';
_14 = 115808198125362583900367124935517747315_i128 as u8;
Call(_4.0 = core::intrinsics::transmute(_1.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = 41149_u16 as i64;
_4.1 = !_10;
_1 = (_4.0, _6.1);
_3 = _6.0 | _4.0;
RET = '\u{86d37}';
Goto(bb4)
}
bb4 = {
_5 = (*_9) as u32;
_1.1 = !_6.1;
_3 = _2;
_6 = _1;
_4.0 = _1.0;
_3 = !_4.0;
_8 = core::ptr::addr_of_mut!(_18);
_16 = _17;
_17 = _16 + _16;
(*_9) = '\u{1f41d}';
_3 = _14 * _6.0;
_4.1 = (*_9) as isize;
(*_9) = '\u{a6203}';
_17 = -_16;
_7 = 74730042325706433395069811798738756781_u128 as u64;
RET = '\u{2ae65}';
_8 = core::ptr::addr_of_mut!((*_8));
_14 = !_4.0;
_7 = !7130737803983388840_u64;
_7 = !8681281956748177791_u64;
_19 = _5 ^ _5;
(*_8) = [11248_i16,4434_i16,(-20_i16),23017_i16,1609_i16,6704_i16];
_10 = -_4.1;
_7 = 17571099591228093138_u64;
Goto(bb5)
}
bb5 = {
_5 = 298365185276500262835556080053419442699_u128 as u32;
(*_8) = [(-19726_i16),22052_i16,(-5715_i16),17531_i16,(-6015_i16),(-18709_i16)];
_8 = core::ptr::addr_of_mut!(_18);
_16 = !_17;
_6 = (_1.0, _10);
_4 = (_3, _6.1);
_9 = core::ptr::addr_of!((*_9));
_10 = _1.1 >> _3;
_1 = (_14, _10);
_1 = _4;
_2 = !_4.0;
_4.0 = 34496_u16 as u8;
_20 = _16 as f32;
(*_8) = [(-21285_i16),30719_i16,21321_i16,(-2158_i16),(-4475_i16),(-13338_i16)];
_7 = !5015312188599927104_u64;
(*_8) = [(-15467_i16),(-27392_i16),(-4406_i16),(-7305_i16),(-20626_i16),20513_i16];
_12 = _20 as f64;
_1 = (_2, _10);
(*_8) = [(-11934_i16),(-26220_i16),(-681_i16),3879_i16,(-6170_i16),(-608_i16)];
_9 = core::ptr::addr_of!((*_9));
_10 = !_1.1;
(*_9) = '\u{951e7}';
_7 = 155266351980308217052973508918943341788_u128 as u64;
_13 = [_1.0,_3,_4.0,_6.0];
_1 = (_3, _10);
_9 = core::ptr::addr_of!(RET);
_1 = _6;
_4.1 = -_1.1;
RET = '\u{ca6d7}';
_18 = [11176_i16,(-2354_i16),(-18367_i16),29481_i16,21219_i16,31119_i16];
Call(_1.0 = fn6(_10, Move(_8), _10, (*_8), _6.0, (*_9), _10, _13, _6, _6.0, (*_8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14 = !_3;
_4.1 = _10;
_21 = 47_i8 + (-121_i8);
_4 = (_6.0, _10);
_13 = [_1.0,_6.0,_6.0,_4.0];
RET = '\u{cad55}';
(*_9) = '\u{840ce}';
_6.1 = _4.1 ^ _4.1;
_8 = core::ptr::addr_of_mut!(_18);
_4 = _6;
_8 = core::ptr::addr_of_mut!(_18);
_10 = !_4.1;
_18 = [(-19320_i16),(-12812_i16),(-5542_i16),(-28787_i16),20396_i16,737_i16];
_2 = _1.0 | _1.0;
_14 = !_2;
_9 = core::ptr::addr_of!(RET);
_16 = _17;
_24 = _7;
_19 = _24 as u32;
_6.1 = _4.1 - _10;
_4 = (_2, _6.1);
(*_8) = [11436_i16,26660_i16,5241_i16,28813_i16,29115_i16,12585_i16];
_6.1 = _10 << _4.0;
RET = '\u{e0340}';
_25 = core::ptr::addr_of_mut!(_18);
_6.1 = _4.1;
_21 = -(-43_i8);
_16 = !_17;
Call(_14 = fn8(_6.1, (*_8), _4.1, _1.0, _2, _3, _3, _4.1, _4.1, _2, _2, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17 = !_16;
_12 = 24405_i16 as f64;
_12 = 458364972_i32 as f64;
_13 = [_4.0,_4.0,_1.0,_14];
_10 = _20 as isize;
_9 = core::ptr::addr_of!((*_9));
_20 = _17 as f32;
_24 = _7;
_22 = Adt18::Variant1 { fld0: false,fld1: _5,fld2: _12,fld3: (-99936809835726237894479894075599111185_i128),fld4: _4.0,fld5: 382007567_i32,fld6: _17 };
_18 = [26583_i16,7222_i16,(-23269_i16),(-17983_i16),19655_i16,(-16383_i16)];
place!(Field::<i128>(Variant(_22, 1), 3)) = 11912435528987749284680515484147761392_i128;
_23 = core::ptr::addr_of_mut!(_18);
(*_9) = '\u{a5e99}';
_6.1 = RET as isize;
_24 = false as u64;
_18 = [5970_i16,(-24462_i16),(-28043_i16),18976_i16,11072_i16,(-18121_i16)];
_27 = [(*_9),(*_9),(*_9),(*_9),RET,(*_9),(*_9),RET];
(*_8) = [8154_i16,(-8525_i16),21597_i16,(-7422_i16),(-9401_i16),(-5366_i16)];
(*_9) = '\u{f8b64}';
_1 = _4;
_28.1 = 7528_u16 - 33199_u16;
(*_8) = [16488_i16,17809_i16,32388_i16,32400_i16,14877_i16,29502_i16];
place!(Field::<f64>(Variant(_22, 1), 2)) = _12 - _12;
_1.1 = false as isize;
_17 = _16 ^ Field::<i64>(Variant(_22, 1), 6);
match Field::<i128>(Variant(_22, 1), 3) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
11912435528987749284680515484147761392 => bb8,
_ => bb5
}
}
bb8 = {
_30 = core::ptr::addr_of!(_22);
_26 = [true,true,true,true,true,true,false,false];
_20 = (-834173754_i32) as f32;
place!(Field::<u8>(Variant((*_30), 1), 4)) = _1.0;
place!(Field::<i32>(Variant(_22, 1), 5)) = (-26277_i16) as i32;
(*_25) = [(-25351_i16),26670_i16,28692_i16,(-13961_i16),6867_i16,(-27974_i16)];
place!(Field::<bool>(Variant(_22, 1), 0)) = true;
place!(Field::<bool>(Variant((*_30), 1), 0)) = _2 < Field::<u8>(Variant((*_30), 1), 4);
(*_30) = Adt18::Variant1 { fld0: false,fld1: _19,fld2: _12,fld3: 7135428798531159265214188182161999951_i128,fld4: _2,fld5: 1271538761_i32,fld6: _17 };
_8 = Move(_23);
place!(Field::<i128>(Variant(_22, 1), 3)) = !70675605621114990650591794797748956013_i128;
_19 = 3_usize as u32;
place!(Field::<f64>(Variant((*_30), 1), 2)) = 11577_i16 as f64;
_4.0 = _6.0 | _1.0;
place!(Field::<f64>(Variant(_22, 1), 2)) = _12;
_5 = _19 | Field::<u32>(Variant((*_30), 1), 1);
_23 = Move(_25);
_2 = _20 as u8;
place!(Field::<f64>(Variant((*_30), 1), 2)) = -_12;
place!(Field::<u32>(Variant(_22, 1), 1)) = 865296303_i32 as u32;
_15 = Adt44::Variant1 { fld0: false };
(*_9) = '\u{a1a98}';
(*_30) = Adt18::Variant0 { fld0: _4,fld1: RET,fld2: _16,fld3: _21,fld4: _20 };
place!(Field::<i8>(Variant((*_30), 0), 3)) = !_21;
place!(Field::<bool>(Variant(_15, 1), 0)) = !true;
Goto(bb9)
}
bb9 = {
_21 = Field::<i8>(Variant((*_30), 0), 3) ^ Field::<i8>(Variant(_22, 0), 3);
_23 = Move(_8);
_14 = !_4.0;
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)) = (_14, _4.1);
_24 = _7 >> Field::<(u8, isize)>(Variant((*_30), 0), 0).0;
_1.0 = 27368_i16 as u8;
Goto(bb10)
}
bb10 = {
place!(Field::<i8>(Variant((*_30), 0), 3)) = -_21;
_22 = Adt18::Variant0 { fld0: _6,fld1: (*_9),fld2: _16,fld3: _21,fld4: _20 };
_33 = _12;
place!(Field::<i8>(Variant(_22, 0), 3)) = 5744655557381518314_usize as i8;
place!(Field::<i64>(Variant((*_30), 0), 2)) = _17;
Call(_4.0 = fn13(_13, _13), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8 = core::ptr::addr_of_mut!(_18);
Goto(bb12)
}
bb12 = {
(*_8) = [(-3055_i16),5571_i16,(-17704_i16),16542_i16,5088_i16,29270_i16];
place!(Field::<(u8, isize)>(Variant((*_30), 0), 0)).0 = _14 + _14;
place!(Field::<(u8, isize)>(Variant((*_30), 0), 0)).1 = -_4.1;
place!(Field::<i64>(Variant((*_30), 0), 2)) = 22434_i16 as i64;
SetDiscriminant((*_30), 0);
_6.1 = Field::<bool>(Variant(_15, 1), 0) as isize;
_35 = 124619199619200309625112622126907382242_i128;
_22 = Adt18::Variant0 { fld0: _4,fld1: RET,fld2: _17,fld3: _21,fld4: _20 };
_4.1 = !Field::<(u8, isize)>(Variant((*_30), 0), 0).1;
place!(Field::<f32>(Variant((*_30), 0), 4)) = -_20;
_6.0 = !_14;
(*_9) = Field::<char>(Variant((*_30), 0), 1);
_14 = _6.0 | _6.0;
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).1 = _4.1 - _4.1;
Goto(bb13)
}
bb13 = {
_36 = [Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0),Field::<bool>(Variant(_15, 1), 0)];
_17 = _24 as i64;
_6.1 = -Field::<(u8, isize)>(Variant(_22, 0), 0).1;
SetDiscriminant(_15, 3);
place!(Field::<char>(Variant(_22, 0), 1)) = RET;
place!(Field::<i64>(Variant(_15, 3), 6)) = _17;
SetDiscriminant(_22, 1);
place!(Field::<u32>(Variant((*_30), 1), 1)) = _19 >> Field::<i64>(Variant(_15, 3), 6);
place!(Field::<i128>(Variant((*_30), 1), 3)) = -_35;
Call(_6.1 = core::intrinsics::bswap(_4.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<bool>(Variant((*_30), 1), 0)) = false;
_8 = Move(_23);
place!(Field::<f64>(Variant(_22, 1), 2)) = _12 - _33;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(4_usize, 24_usize, Move(_24), 4_usize, Move(_4), 27_usize, Move(_27), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(4_usize, 36_usize, Move(_36), 1_usize, Move(_1), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(4_usize, 10_usize, Move(_10), 21_usize, Move(_21), 43_usize, _43, 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u8,mut _2: (u8, isize),mut _3: (u8, isize),mut _4: u64,mut _5: u8,mut _6: char,mut _7: (u8, isize),mut _8: (u8, isize),mut _9: (u8, isize)) -> u8 {
mir! {
type RET = u8;
let _10: (&'static (usize, f32, Adt18, &'static usize), char, u32);
let _11: &'static i64;
let _12: u64;
let _13: [u32; 1];
let _14: ();
let _15: ();
{
_6 = '\u{d4765}';
_9.1 = 14723_i16 as isize;
_10.1 = _6;
RET = !_8.0;
_7.0 = !RET;
_7 = (_5, _8.1);
RET = _9.0;
_1 = _3.0 & _7.0;
_7.0 = _5 | _9.0;
_9.0 = _3.0 << _4;
_2 = (_7.0, _8.1);
_9.1 = _2.1;
_2.1 = (-1086705047_i32) as isize;
_2.1 = _9.1;
_6 = _10.1;
_7.0 = _2.0 - _9.0;
_10.1 = _6;
_2.0 = !_1;
_7.0 = !_2.0;
_9.1 = _2.1 - _2.1;
RET = _7.0;
_5 = 75_i8 as u8;
_8 = _7;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(5_usize, 3_usize, Move(_3), 5_usize, Move(_5), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: *mut [i16; 6],mut _3: isize,mut _4: [i16; 6],mut _5: u8,mut _6: char,mut _7: isize,mut _8: [u8; 4],mut _9: (u8, isize),mut _10: u8,mut _11: [i16; 6]) -> u8 {
mir! {
type RET = u8;
let _12: &'static (usize, f32, Adt18, &'static usize);
let _13: u16;
let _14: f32;
let _15: i128;
let _16: f64;
let _17: isize;
let _18: f64;
let _19: [usize; 3];
let _20: i128;
let _21: ();
let _22: ();
{
_8 = [_5,_10,_9.0,_10];
_9.0 = _10 & _5;
Goto(bb1)
}
bb1 = {
RET = (-10241_i16) as u8;
_9.1 = _7;
_9.0 = !_5;
Call(_5 = fn7(_7, _4, _3, _9, _7, _9.1, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [RET,_10,_9.0,_10];
_11 = [(-25259_i16),4283_i16,(-31187_i16),5806_i16,(-30796_i16),(-2353_i16)];
_8 = [_9.0,_10,_5,_9.0];
_9.1 = _7;
_9.0 = _5;
RET = !_5;
_8 = [_9.0,RET,_5,_9.0];
_6 = '\u{f48eb}';
_2 = core::ptr::addr_of_mut!(_11);
_13 = !32745_u16;
(*_2) = [17778_i16,4846_i16,25958_i16,2930_i16,15063_i16,14653_i16];
RET = 29_i8 as u8;
RET = _5 + _10;
_15 = !39601279036979946979473642428667545638_i128;
_9.1 = _3 | _3;
_16 = (-1322335711_i32) as f64;
_7 = _9.1;
_1 = (-6977161766747122581_i64) as isize;
Goto(bb3)
}
bb3 = {
Call(_21 = dump_var(6_usize, 9_usize, Move(_9), 11_usize, Move(_11), 7_usize, Move(_7), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_21 = dump_var(6_usize, 4_usize, Move(_4), 10_usize, Move(_10), 22_usize, _22, 22_usize, _22), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: [i16; 6],mut _3: isize,mut _4: (u8, isize),mut _5: isize,mut _6: isize,mut _7: isize,mut _8: (u8, isize)) -> u8 {
mir! {
type RET = u8;
let _9: (&'static i64,);
let _10: f64;
let _11: *const [u32; 1];
let _12: [u8; 4];
let _13: ();
let _14: ();
{
RET = _4.0 | _4.0;
_4 = (RET, _5);
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(7_usize, 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: [i16; 6],mut _3: isize,mut _4: u8,mut _5: u8,mut _6: u8,mut _7: u8,mut _8: isize,mut _9: isize,mut _10: u8,mut _11: u8,mut _12: (u8, isize)) -> u8 {
mir! {
type RET = u8;
let _13: usize;
let _14: &'static Adt48;
let _15: isize;
let _16: Adt18;
let _17: i128;
let _18: f64;
let _19: ();
let _20: ();
{
_13 = 7_usize;
RET = _5 | _11;
_3 = -_9;
RET = true as u8;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
7 => bb5,
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
_11 = _12.0 * _4;
_12.1 = !_3;
_9 = _1 | _1;
_11 = '\u{8cf31}' as u8;
RET = _10;
_2 = [17121_i16,(-7569_i16),(-18855_i16),22699_i16,25157_i16,5389_i16];
RET = 5975822198091715070_i64 as u8;
_5 = '\u{92154}' as u8;
_12 = (_6, _8);
_10 = _6;
_9 = _1;
_10 = _4 * _4;
_6 = _10 - _10;
_6 = !_7;
_1 = 3012354521_u32 as isize;
_7 = 14868901375125620648_u64 as u8;
_1 = _13 as isize;
_5 = _10 | _12.0;
_15 = -_3;
_10 = _6 ^ _5;
_4 = _13 as u8;
_10 = (-65_i8) as u8;
_15 = _8;
_5 = 56502_u16 as u8;
RET = 15762_i16 as u8;
Call(_2 = fn9(_8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.0 = _6 & _6;
_12.0 = _6;
RET = _12.0 - _6;
_6 = !RET;
_5 = (-1501845492_i32) as u8;
_2 = [14520_i16,(-12362_i16),29093_i16,(-1248_i16),7904_i16,(-21860_i16)];
_12.1 = _15 ^ _9;
_7 = !_12.0;
_17 = 156206609970232885592461487180448260030_i128;
_5 = _6;
_5 = _7 * _12.0;
_7 = 274430671277748501660145678368989307580_u128 as u8;
_15 = '\u{d7a3c}' as isize;
_12.0 = _5 - _5;
_12.0 = _17 as u8;
_3 = _9 - _12.1;
_6 = 14600616620120629610_u64 as u8;
_18 = 22755_u16 as f64;
_18 = 418210683_i32 as f64;
_10 = 2128586769_i32 as u8;
Goto(bb7)
}
bb7 = {
Call(_19 = dump_var(8_usize, 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_19 = dump_var(8_usize, 15_usize, Move(_15), 17_usize, Move(_17), 4_usize, Move(_4), 20_usize, _20), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize) -> [i16; 6] {
mir! {
type RET = [i16; 6];
let _2: u32;
let _3: f32;
let _4: (&'static i64,);
let _5: &'static [i16; 7];
let _6: [i16; 1];
let _7: f64;
let _8: f32;
let _9: Adt77;
let _10: i32;
let _11: *mut i128;
let _12: (&'static (u8,),);
let _13: Adt33;
let _14: isize;
let _15: f32;
let _16: u16;
let _17: u64;
let _18: isize;
let _19: u16;
let _20: char;
let _21: (bool, u16, [u32; 1], u16);
let _22: f64;
let _23: Adt48;
let _24: f32;
let _25: &'static u128;
let _26: *const char;
let _27: Adt44;
let _28: [usize; 3];
let _29: &'static (usize, f32, Adt18, &'static usize);
let _30: i64;
let _31: f64;
let _32: usize;
let _33: [char; 8];
let _34: isize;
let _35: ([i16; 6],);
let _36: u32;
let _37: &'static [i16; 6];
let _38: i64;
let _39: i8;
let _40: &'static (usize, f32, Adt18, &'static usize);
let _41: Adt45;
let _42: f64;
let _43: &'static Adt48;
let _44: ();
let _45: ();
{
_1 = (-9223372036854775808_isize) >> 11382475270820822982_usize;
_3 = 5_usize as f32;
_3 = 11695_i16 as f32;
RET = [(-21731_i16),(-21422_i16),(-80_i16),(-3356_i16),3589_i16,18049_i16];
_2 = !1073596226_u32;
_2 = _3 as u32;
RET = [32334_i16,7274_i16,5957_i16,11808_i16,20890_i16,(-9125_i16)];
_1 = (-9223372036854775808_isize);
_1 = 9223372036854775807_isize;
RET = [(-18639_i16),(-23694_i16),27824_i16,(-25587_i16),(-29878_i16),13031_i16];
_6 = [6141_i16];
_1 = (-9223372036854775808_isize);
_2 = (-544035288_i32) as u32;
_8 = _3;
_3 = 3831_i16 as f32;
_1 = -(-9223372036854775808_isize);
_10 = 202415285_i32;
Goto(bb1)
}
bb1 = {
_10 = 1227140261_i32;
RET = [25285_i16,31195_i16,(-32418_i16),32718_i16,(-14424_i16),(-17032_i16)];
_10 = 493413266_i32;
RET = [(-2248_i16),(-18395_i16),5732_i16,19299_i16,(-4288_i16),(-8916_i16)];
_7 = (-12547_i16) as f64;
RET = [(-22462_i16),30782_i16,19797_i16,30223_i16,6225_i16,(-12018_i16)];
_6 = [5446_i16];
_8 = _3 + _3;
_13.fld7 = 14706310582859651136_u64;
_5 = &_13.fld5;
_13.fld3 = (107_u8, _1);
_13.fld2 = _13.fld3.0 | _13.fld3.0;
_13.fld3.1 = _1;
_13.fld1 = '\u{94b95}';
_13.fld5 = [8236_i16,4295_i16,11715_i16,(-21026_i16),(-20468_i16),(-13316_i16),18279_i16];
_13.fld3 = (_13.fld2, _1);
_13.fld5 = [(-4639_i16),5181_i16,(-26252_i16),20368_i16,(-17303_i16),(-11265_i16),(-18409_i16)];
_13.fld5 = [2502_i16,16613_i16,26795_i16,22499_i16,31410_i16,6717_i16,(-17373_i16)];
_13.fld0 = 29525_i16 as u16;
_13.fld7 = 18043700884246438545_u64;
_2 = (-24_i8) as u32;
_13.fld4 = [_2];
Call(_8 = fn10(_13.fld3.1, _13.fld5, _13.fld3.1, _13.fld5, _7, _13.fld5, _13.fld3, _13.fld3, _13.fld3, _13.fld3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [(-9299_i16),(-26157_i16),(-17568_i16),31140_i16,(-25262_i16),(-18833_i16)];
RET = [(-8462_i16),(-27341_i16),12374_i16,25990_i16,(-8069_i16),(-7309_i16)];
RET = [(-22514_i16),(-1089_i16),18704_i16,(-815_i16),17941_i16,25271_i16];
_13.fld4 = [_2];
_13.fld6.1 = !_13.fld2;
_15 = _8;
_20 = _13.fld1;
_1 = -_13.fld3.1;
_6 = [1000_i16];
_14 = _1 * _13.fld3.1;
RET = [(-9701_i16),(-8802_i16),(-545_i16),32183_i16,(-13325_i16),(-23948_i16)];
_18 = _13.fld3.1 - _14;
_13.fld7 = 7665619934772958650_u64;
_13.fld3.1 = _1 >> _13.fld6.1;
_8 = (-60_i8) as f32;
_13.fld6.1 = _2 as u8;
_19 = (-43703754037828295697318753207002704002_i128) as u16;
RET = [(-26943_i16),(-14081_i16),(-23094_i16),(-8414_i16),9599_i16,4313_i16];
_16 = _19;
_13.fld7 = _14 as u64;
_13.fld1 = _20;
_22 = _7 + _7;
_13.fld5 = [6729_i16,(-16835_i16),(-27956_i16),(-6744_i16),31178_i16,29733_i16,(-21142_i16)];
_17 = false as u64;
_13.fld3 = (_13.fld2, _18);
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
493413266 => bb7,
_ => bb6
}
}
bb3 = {
_10 = 1227140261_i32;
RET = [25285_i16,31195_i16,(-32418_i16),32718_i16,(-14424_i16),(-17032_i16)];
_10 = 493413266_i32;
RET = [(-2248_i16),(-18395_i16),5732_i16,19299_i16,(-4288_i16),(-8916_i16)];
_7 = (-12547_i16) as f64;
RET = [(-22462_i16),30782_i16,19797_i16,30223_i16,6225_i16,(-12018_i16)];
_6 = [5446_i16];
_8 = _3 + _3;
_13.fld7 = 14706310582859651136_u64;
_5 = &_13.fld5;
_13.fld3 = (107_u8, _1);
_13.fld2 = _13.fld3.0 | _13.fld3.0;
_13.fld3.1 = _1;
_13.fld1 = '\u{94b95}';
_13.fld5 = [8236_i16,4295_i16,11715_i16,(-21026_i16),(-20468_i16),(-13316_i16),18279_i16];
_13.fld3 = (_13.fld2, _1);
_13.fld5 = [(-4639_i16),5181_i16,(-26252_i16),20368_i16,(-17303_i16),(-11265_i16),(-18409_i16)];
_13.fld5 = [2502_i16,16613_i16,26795_i16,22499_i16,31410_i16,6717_i16,(-17373_i16)];
_13.fld0 = 29525_i16 as u16;
_13.fld7 = 18043700884246438545_u64;
_2 = (-24_i8) as u32;
_13.fld4 = [_2];
Call(_8 = fn10(_13.fld3.1, _13.fld5, _13.fld3.1, _13.fld5, _7, _13.fld5, _13.fld3, _13.fld3, _13.fld3, _13.fld3.0), ReturnTo(bb2), UnwindUnreachable())
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
_21 = (true, _13.fld0, _13.fld4, _13.fld0);
_15 = _3 - _8;
match _10 {
0 => bb8,
1 => bb9,
493413266 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
RET = [(-9299_i16),(-26157_i16),(-17568_i16),31140_i16,(-25262_i16),(-18833_i16)];
RET = [(-8462_i16),(-27341_i16),12374_i16,25990_i16,(-8069_i16),(-7309_i16)];
RET = [(-22514_i16),(-1089_i16),18704_i16,(-815_i16),17941_i16,25271_i16];
_13.fld4 = [_2];
_13.fld6.1 = !_13.fld2;
_15 = _8;
_20 = _13.fld1;
_1 = -_13.fld3.1;
_6 = [1000_i16];
_14 = _1 * _13.fld3.1;
RET = [(-9701_i16),(-8802_i16),(-545_i16),32183_i16,(-13325_i16),(-23948_i16)];
_18 = _13.fld3.1 - _14;
_13.fld7 = 7665619934772958650_u64;
_13.fld3.1 = _1 >> _13.fld6.1;
_8 = (-60_i8) as f32;
_13.fld6.1 = _2 as u8;
_19 = (-43703754037828295697318753207002704002_i128) as u16;
RET = [(-26943_i16),(-14081_i16),(-23094_i16),(-8414_i16),9599_i16,4313_i16];
_16 = _19;
_13.fld7 = _14 as u64;
_13.fld1 = _20;
_22 = _7 + _7;
_13.fld5 = [6729_i16,(-16835_i16),(-27956_i16),(-6744_i16),31178_i16,29733_i16,(-21142_i16)];
_17 = false as u64;
_13.fld3 = (_13.fld2, _18);
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
493413266 => bb7,
_ => bb6
}
}
bb10 = {
Return()
}
bb11 = {
_13.fld6.1 = !_13.fld3.0;
_13.fld3.1 = -_14;
_21 = (true, _19, _13.fld4, _13.fld0);
_13.fld3.1 = -_18;
_13.fld6 = (_13.fld2, _13.fld2);
_26 = core::ptr::addr_of!(_20);
_22 = -_7;
_20 = _13.fld1;
_27 = Adt44::Variant3 { fld0: _7,fld1: _13.fld3.0,fld2: _14,fld3: _2,fld4: 7_usize,fld5: _13.fld7,fld6: 3818742942273283386_i64 };
_8 = 28132_i16 as f32;
_28 = [16503008707261486054_usize,13762047090167165303_usize,7_usize];
_26 = core::ptr::addr_of!((*_26));
_16 = _21.3 ^ _13.fld0;
_4.0 = &place!(Field::<i64>(Variant(_27, 3), 6));
_13.fld4 = [_2];
_2 = Field::<u32>(Variant(_27, 3), 3);
_13.fld3.1 = !_14;
_13.fld3.0 = !_13.fld6.1;
Goto(bb12)
}
bb12 = {
_1 = !_14;
_5 = &_13.fld5;
_13.fld5 = [(-25371_i16),11972_i16,(-30520_i16),(-24912_i16),23936_i16,(-8251_i16),30223_i16];
_26 = core::ptr::addr_of!((*_26));
RET = [(-21261_i16),10385_i16,24456_i16,7701_i16,17509_i16,1702_i16];
place!(Field::<usize>(Variant(_27, 3), 4)) = !2_usize;
_19 = (-2998249328032120886527019581195612832_i128) as u16;
_1 = 94116406593172799108435907708718173801_u128 as isize;
Call(_8 = core::intrinsics::transmute(_21.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_5 = &_13.fld5;
_27 = Adt44::Variant3 { fld0: _7,fld1: _13.fld6.0,fld2: _1,fld3: _2,fld4: 7262944055102251325_usize,fld5: _17,fld6: (-711130970101736658_i64) };
_19 = _13.fld0;
RET = [(-5505_i16),7815_i16,21509_i16,(-1085_i16),17444_i16,(-25153_i16)];
place!(Field::<isize>(Variant(_27, 3), 2)) = _13.fld6.1 as isize;
_4.0 = &_30;
_13.fld5 = [10570_i16,2896_i16,(-27353_i16),(-1083_i16),(-14000_i16),19768_i16,(-23883_i16)];
_13.fld5 = [(-14885_i16),(-29725_i16),(-24565_i16),10085_i16,(-1397_i16),639_i16,17552_i16];
_13.fld4 = [_2];
_31 = _7 + Field::<f64>(Variant(_27, 3), 0);
_14 = _2 as isize;
_16 = _13.fld0 ^ _19;
place!(Field::<isize>(Variant(_27, 3), 2)) = _18;
match _10 {
0 => bb9,
1 => bb2,
493413266 => bb15,
_ => bb14
}
}
bb14 = {
_21 = (true, _13.fld0, _13.fld4, _13.fld0);
_15 = _3 - _8;
match _10 {
0 => bb8,
1 => bb9,
493413266 => bb11,
_ => bb10
}
}
bb15 = {
_24 = 65733549571599398891332852053237485691_i128 as f32;
_22 = _7 - _31;
_15 = _8;
_18 = _13.fld3.0 as isize;
_37 = &RET;
_26 = core::ptr::addr_of!((*_26));
_26 = core::ptr::addr_of!(_13.fld1);
_15 = (-7265982609064207058_i64) as f32;
_17 = _13.fld7;
_30 = -(-7826905121847806449_i64);
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(9_usize, 19_usize, Move(_19), 21_usize, Move(_21), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(9_usize, 1_usize, Move(_1), 17_usize, Move(_17), 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: [i16; 7],mut _3: isize,mut _4: [i16; 7],mut _5: f64,mut _6: [i16; 7],mut _7: (u8, isize),mut _8: (u8, isize),mut _9: (u8, isize),mut _10: u8) -> f32 {
mir! {
type RET = f32;
let _11: Adt45;
let _12: bool;
let _13: i32;
let _14: &'static [i16; 6];
let _15: *const i8;
let _16: bool;
let _17: Adt75;
let _18: f64;
let _19: [u32; 1];
let _20: char;
let _21: f64;
let _22: isize;
let _23: [i32; 4];
let _24: *const Adt18;
let _25: i8;
let _26: [usize; 7];
let _27: (&'static (usize, f32, Adt18, &'static usize), char, u32);
let _28: &'static i64;
let _29: ([bool; 8],);
let _30: [u8; 4];
let _31: &'static i64;
let _32: bool;
let _33: u8;
let _34: char;
let _35: *mut [usize; 3];
let _36: bool;
let _37: f64;
let _38: u16;
let _39: Adt77;
let _40: &'static i16;
let _41: Adt75;
let _42: [u8; 4];
let _43: *const u128;
let _44: isize;
let _45: ();
let _46: ();
{
_9.0 = !_7.0;
_1 = !_7.1;
_7 = (_9.0, _3);
Goto(bb1)
}
bb1 = {
_8 = (_10, _1);
_8.1 = -_9.1;
_9.0 = _7.0 | _10;
_11.fld0 = [(-1207381619_i32),(-627168012_i32),339851537_i32,(-1919723390_i32)];
_6 = [(-19038_i16),12852_i16,30350_i16,22341_i16,1538_i16,12403_i16,26_i16];
_11.fld0 = [944930276_i32,(-275464424_i32),(-955199492_i32),1318928825_i32];
RET = 13383_i16 as f32;
_3 = _9.1 - _1;
_8.1 = -_7.1;
_11.fld1.1 = _8.1 >> _10;
_11.fld0 = [324183558_i32,1433652189_i32,1387678214_i32,890899275_i32];
_8.1 = (-165187320789817103662772667305996765540_i128) as isize;
_11.fld1.1 = _3 ^ _3;
_8.1 = -_11.fld1.1;
RET = _5 as f32;
_11.fld1.1 = RET as isize;
_8.1 = _3;
_11.fld7 = Adt44::Variant1 { fld0: false };
_11.fld7 = Adt44::Variant1 { fld0: true };
_8.0 = false as u8;
_7.1 = _3;
_7 = (_9.0, _8.1);
_7 = (_9.0, _1);
RET = (-1445706926171236175_i64) as f32;
_9.1 = _8.1 ^ _3;
RET = 2864442666_u32 as f32;
_11.fld2 = !_9.0;
_11.fld1.0 = _11.fld2;
place!(Field::<bool>(Variant(_11.fld7, 1), 0)) = true;
RET = 2_usize as f32;
_11.fld4 = '\u{9d6f}' as u16;
Goto(bb2)
}
bb2 = {
_11.fld5 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_11.fld7, 1), 0),fld1: 1267130843_u32,fld2: _5,fld3: 32660120093662297247875445888287913474_i128,fld4: _7.0,fld5: 1040597835_i32,fld6: (-6888167644218367997_i64) };
place!(Field::<u8>(Variant(_11.fld5, 1), 4)) = _7.0;
_4 = _6;
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = (-2971098499512907148_i64) + (-4776822797752038227_i64);
_11.fld6 = core::ptr::addr_of!(_11.fld3);
place!(Field::<i128>(Variant(_11.fld5, 1), 3)) = 14114317050468642419_usize as i128;
_6 = _4;
_11.fld4 = 32286_u16 * 4069_u16;
_5 = _11.fld1.0 as f64;
Goto(bb3)
}
bb3 = {
_3 = _8.1;
_11.fld1.0 = !Field::<u8>(Variant(_11.fld5, 1), 4);
SetDiscriminant(_11.fld7, 1);
RET = _9.1 as f32;
_11.fld5 = Adt18::Variant1 { fld0: true,fld1: 2595942739_u32,fld2: _5,fld3: 7791717969655160783879636685323545860_i128,fld4: _11.fld1.0,fld5: (-1473834330_i32),fld6: 882543252748006238_i64 };
_9 = _11.fld1;
_11.fld6 = core::ptr::addr_of!(_11.fld3);
_11.fld6 = core::ptr::addr_of!(_11.fld3);
_11.fld1.0 = _9.0 << _3;
_11.fld1.0 = Field::<u8>(Variant(_11.fld5, 1), 4) + _11.fld2;
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = 8607416699570929578_i64;
_11.fld1.1 = -_9.1;
_8 = (_11.fld2, _7.1);
place!(Field::<i32>(Variant(_11.fld5, 1), 5)) = 1262070018_i32;
RET = 2353459762_u32 as f32;
_1 = 233100523164221753889264533063365383012_u128 as isize;
_11.fld0 = [Field::<i32>(Variant(_11.fld5, 1), 5),Field::<i32>(Variant(_11.fld5, 1), 5),Field::<i32>(Variant(_11.fld5, 1), 5),Field::<i32>(Variant(_11.fld5, 1), 5)];
_10 = Field::<u8>(Variant(_11.fld5, 1), 4);
_11.fld5 = Adt18::Variant1 { fld0: true,fld1: 1423473881_u32,fld2: _5,fld3: 113447983374832227638923707988435061974_i128,fld4: _7.0,fld5: 208130387_i32,fld6: (-4209690572936372655_i64) };
_12 = false;
_1 = _12 as isize;
_2 = [22792_i16,(-10707_i16),(-970_i16),23817_i16,6413_i16,(-18717_i16),19348_i16];
_15 = core::ptr::addr_of!(_11.fld3);
_8 = (_11.fld1.0, _3);
_11.fld7 = Adt44::Variant1 { fld0: _12 };
Goto(bb4)
}
bb4 = {
_11.fld0 = [1147763178_i32,(-1857274864_i32),1637288069_i32,(-290317082_i32)];
SetDiscriminant(_11.fld7, 3);
_11.fld1.1 = _9.1;
(*_15) = 105_i8;
(*_15) = _5 as i8;
place!(Field::<f64>(Variant(_11.fld5, 1), 2)) = 32381_i16 as f64;
place!(Field::<f64>(Variant(_11.fld7, 3), 0)) = Field::<f64>(Variant(_11.fld5, 1), 2);
_16 = _12 ^ _12;
_13 = (-1455242250_i32);
Call(place!(Field::<u32>(Variant(_11.fld5, 1), 1)) = fn11(_9, _11.fld1, _8.0, (*_15), _11.fld1, _11.fld1, _5, _11.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_15 = Move(_11.fld6);
_7 = (_11.fld2, _9.1);
_1 = _8.1 << Field::<u32>(Variant(_11.fld5, 1), 1);
_11.fld5 = Adt18::Variant1 { fld0: _16,fld1: 1270122135_u32,fld2: _5,fld3: 97333540882918965642711549224965183237_i128,fld4: _11.fld2,fld5: _13,fld6: 474835270570125923_i64 };
_5 = Field::<f64>(Variant(_11.fld5, 1), 2);
_21 = Field::<i32>(Variant(_11.fld5, 1), 5) as f64;
place!(Field::<usize>(Variant(_11.fld7, 3), 4)) = 491651588_u32 as usize;
_11.fld6 = core::ptr::addr_of!(_11.fld3);
_13 = Field::<i32>(Variant(_11.fld5, 1), 5);
Goto(bb6)
}
bb6 = {
_18 = -_5;
RET = Field::<f64>(Variant(_11.fld5, 1), 2) as f32;
_11.fld3 = 1164458581_u32 as i8;
place!(Field::<isize>(Variant(_11.fld7, 3), 2)) = RET as isize;
place!(Field::<i64>(Variant(_11.fld7, 3), 6)) = (-7925314012164397550_i64) | (-9193879301990845607_i64);
RET = _11.fld3 as f32;
_11.fld1 = (_9.0, _1);
place!(Field::<i64>(Variant(_11.fld7, 3), 6)) = (-4373789273293353486_i64) * 8527961400287879512_i64;
_9 = (_11.fld2, _1);
_18 = -Field::<f64>(Variant(_11.fld5, 1), 2);
_7 = (_11.fld1.0, Field::<isize>(Variant(_11.fld7, 3), 2));
_11.fld5 = Adt18::Variant1 { fld0: _12,fld1: 1713118175_u32,fld2: _5,fld3: (-160544422270844145424890731822948367087_i128),fld4: _11.fld2,fld5: _13,fld6: Field::<i64>(Variant(_11.fld7, 3), 6) };
_18 = Field::<f64>(Variant(_11.fld5, 1), 2);
place!(Field::<f64>(Variant(_11.fld7, 3), 0)) = Field::<i64>(Variant(_11.fld7, 3), 6) as f64;
_3 = _11.fld1.1;
place!(Field::<u64>(Variant(_11.fld7, 3), 5)) = !2902826820082475088_u64;
_19 = [1139412699_u32];
_8.1 = '\u{e2312}' as isize;
place!(Field::<u32>(Variant(_11.fld7, 3), 3)) = !3492117052_u32;
place!(Field::<u32>(Variant(_11.fld7, 3), 3)) = 1363828498_u32;
_11.fld1.0 = 30389_i16 as u8;
place!(Field::<isize>(Variant(_11.fld7, 3), 2)) = 48156059810542693480481382488307486553_i128 as isize;
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = -Field::<i64>(Variant(_11.fld7, 3), 6);
Goto(bb7)
}
bb7 = {
_9.1 = _1;
place!(Field::<u32>(Variant(_11.fld5, 1), 1)) = !Field::<u32>(Variant(_11.fld7, 3), 3);
_3 = -_1;
_11.fld6 = Move(_15);
match Field::<u32>(Variant(_11.fld7, 3), 3) {
0 => bb1,
1 => bb2,
2 => bb5,
1363828498 => bb8,
_ => bb4
}
}
bb8 = {
_21 = Field::<usize>(Variant(_11.fld7, 3), 4) as f64;
_23 = [_13,Field::<i32>(Variant(_11.fld5, 1), 5),_13,Field::<i32>(Variant(_11.fld5, 1), 5)];
_12 = !_16;
_9 = _11.fld1;
_11.fld1.0 = _13 as u8;
_15 = Move(_11.fld6);
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = !Field::<i64>(Variant(_11.fld7, 3), 6);
Goto(bb9)
}
bb9 = {
_11.fld7 = Adt44::Variant3 { fld0: _5,fld1: _8.0,fld2: _9.1,fld3: Field::<u32>(Variant(_11.fld5, 1), 1),fld4: 7_usize,fld5: 16106773859223383655_u64,fld6: Field::<i64>(Variant(_11.fld5, 1), 6) };
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = Field::<i64>(Variant(_11.fld7, 3), 6);
_11.fld1 = _9;
_3 = _12 as isize;
_11.fld5 = Adt18::Variant1 { fld0: _12,fld1: Field::<u32>(Variant(_11.fld7, 3), 3),fld2: _18,fld3: 78911131135291977135634956296730034872_i128,fld4: _8.0,fld5: _13,fld6: Field::<i64>(Variant(_11.fld7, 3), 6) };
place!(Field::<u64>(Variant(_11.fld7, 3), 5)) = 4704172621415550877_u64;
_1 = -_11.fld1.1;
RET = (-101531752996877400096226159732393515052_i128) as f32;
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = Field::<i64>(Variant(_11.fld7, 3), 6);
_3 = -Field::<isize>(Variant(_11.fld7, 3), 2);
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = Field::<i64>(Variant(_11.fld7, 3), 6);
_24 = core::ptr::addr_of!(_11.fld5);
place!(Field::<f64>(Variant(_11.fld5, 1), 2)) = _18;
_22 = '\u{60b03}' as isize;
place!(Field::<i128>(Variant((*_24), 1), 3)) = (-43358418806412121208130826390614776310_i128);
_9 = (_7.0, Field::<isize>(Variant(_11.fld7, 3), 2));
_28 = &place!(Field::<i64>(Variant(_11.fld5, 1), 6));
(*_24) = Adt18::Variant0 { fld0: _9,fld1: '\u{4806e}',fld2: Field::<i64>(Variant(_11.fld7, 3), 6),fld3: _11.fld3,fld4: RET };
_10 = _13 as u8;
_11.fld5 = Adt18::Variant0 { fld0: _7,fld1: '\u{47261}',fld2: Field::<i64>(Variant(_11.fld7, 3), 6),fld3: _11.fld3,fld4: RET };
match Field::<u64>(Variant(_11.fld7, 3), 5) {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb11,
4704172621415550877 => bb13,
_ => bb12
}
}
bb10 = {
_15 = Move(_11.fld6);
_7 = (_11.fld2, _9.1);
_1 = _8.1 << Field::<u32>(Variant(_11.fld5, 1), 1);
_11.fld5 = Adt18::Variant1 { fld0: _16,fld1: 1270122135_u32,fld2: _5,fld3: 97333540882918965642711549224965183237_i128,fld4: _11.fld2,fld5: _13,fld6: 474835270570125923_i64 };
_5 = Field::<f64>(Variant(_11.fld5, 1), 2);
_21 = Field::<i32>(Variant(_11.fld5, 1), 5) as f64;
place!(Field::<usize>(Variant(_11.fld7, 3), 4)) = 491651588_u32 as usize;
_11.fld6 = core::ptr::addr_of!(_11.fld3);
_13 = Field::<i32>(Variant(_11.fld5, 1), 5);
Goto(bb6)
}
bb11 = {
_11.fld0 = [1147763178_i32,(-1857274864_i32),1637288069_i32,(-290317082_i32)];
SetDiscriminant(_11.fld7, 3);
_11.fld1.1 = _9.1;
(*_15) = 105_i8;
(*_15) = _5 as i8;
place!(Field::<f64>(Variant(_11.fld5, 1), 2)) = 32381_i16 as f64;
place!(Field::<f64>(Variant(_11.fld7, 3), 0)) = Field::<f64>(Variant(_11.fld5, 1), 2);
_16 = _12 ^ _12;
_13 = (-1455242250_i32);
Call(place!(Field::<u32>(Variant(_11.fld5, 1), 1)) = fn11(_9, _11.fld1, _8.0, (*_15), _11.fld1, _11.fld1, _5, _11.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb12 = {
_18 = -_5;
RET = Field::<f64>(Variant(_11.fld5, 1), 2) as f32;
_11.fld3 = 1164458581_u32 as i8;
place!(Field::<isize>(Variant(_11.fld7, 3), 2)) = RET as isize;
place!(Field::<i64>(Variant(_11.fld7, 3), 6)) = (-7925314012164397550_i64) | (-9193879301990845607_i64);
RET = _11.fld3 as f32;
_11.fld1 = (_9.0, _1);
place!(Field::<i64>(Variant(_11.fld7, 3), 6)) = (-4373789273293353486_i64) * 8527961400287879512_i64;
_9 = (_11.fld2, _1);
_18 = -Field::<f64>(Variant(_11.fld5, 1), 2);
_7 = (_11.fld1.0, Field::<isize>(Variant(_11.fld7, 3), 2));
_11.fld5 = Adt18::Variant1 { fld0: _12,fld1: 1713118175_u32,fld2: _5,fld3: (-160544422270844145424890731822948367087_i128),fld4: _11.fld2,fld5: _13,fld6: Field::<i64>(Variant(_11.fld7, 3), 6) };
_18 = Field::<f64>(Variant(_11.fld5, 1), 2);
place!(Field::<f64>(Variant(_11.fld7, 3), 0)) = Field::<i64>(Variant(_11.fld7, 3), 6) as f64;
_3 = _11.fld1.1;
place!(Field::<u64>(Variant(_11.fld7, 3), 5)) = !2902826820082475088_u64;
_19 = [1139412699_u32];
_8.1 = '\u{e2312}' as isize;
place!(Field::<u32>(Variant(_11.fld7, 3), 3)) = !3492117052_u32;
place!(Field::<u32>(Variant(_11.fld7, 3), 3)) = 1363828498_u32;
_11.fld1.0 = 30389_i16 as u8;
place!(Field::<isize>(Variant(_11.fld7, 3), 2)) = 48156059810542693480481382488307486553_i128 as isize;
place!(Field::<i64>(Variant(_11.fld5, 1), 6)) = -Field::<i64>(Variant(_11.fld7, 3), 6);
Goto(bb7)
}
bb13 = {
(*_24) = Adt18::Variant0 { fld0: _9,fld1: '\u{9bf02}',fld2: Field::<i64>(Variant(_11.fld7, 3), 6),fld3: _11.fld3,fld4: RET };
place!(Field::<i8>(Variant((*_24), 0), 3)) = _11.fld3 + _11.fld3;
_24 = core::ptr::addr_of!((*_24));
place!(Field::<i8>(Variant((*_24), 0), 3)) = !_11.fld3;
_4 = [(-22573_i16),(-3802_i16),7660_i16,23265_i16,(-16437_i16),(-23158_i16),29230_i16];
_24 = core::ptr::addr_of!((*_24));
place!(Field::<u8>(Variant(_11.fld7, 3), 1)) = _11.fld2 * Field::<(u8, isize)>(Variant(_11.fld5, 0), 0).0;
_11.fld6 = core::ptr::addr_of!(_25);
_9.0 = Field::<(u8, isize)>(Variant((*_24), 0), 0).0;
place!(Field::<u8>(Variant(_11.fld7, 3), 1)) = !Field::<(u8, isize)>(Variant((*_24), 0), 0).0;
place!(Field::<f32>(Variant(_11.fld5, 0), 4)) = RET - RET;
_10 = _8.0 + _9.0;
place!(Field::<f32>(Variant((*_24), 0), 4)) = RET - RET;
_3 = Field::<(u8, isize)>(Variant((*_24), 0), 0).1;
place!(Field::<usize>(Variant(_11.fld7, 3), 4)) = !0_usize;
_7.0 = Field::<(u8, isize)>(Variant((*_24), 0), 0).0 & _9.0;
_9 = _11.fld1;
place!(Field::<char>(Variant((*_24), 0), 1)) = '\u{dafd3}';
_23 = [_13,_13,_13,_13];
_9.1 = 13195_i16 as isize;
_27.2 = Field::<u32>(Variant(_11.fld7, 3), 3);
_34 = Field::<char>(Variant((*_24), 0), 1);
_20 = Field::<char>(Variant((*_24), 0), 1);
SetDiscriminant((*_24), 1);
_34 = _20;
place!(Field::<i64>(Variant((*_24), 1), 6)) = !Field::<i64>(Variant(_11.fld7, 3), 6);
_34 = _20;
match _13 {
0 => bb11,
1 => bb9,
2 => bb3,
3 => bb6,
340282366920938463463374607430312969206 => bb14,
_ => bb5
}
}
bb14 = {
_32 = !_16;
_23 = [_13,_13,_13,_13];
place!(Field::<usize>(Variant(_11.fld7, 3), 4)) = 5_usize;
_27.2 = _5 as u32;
_9.0 = _7.0;
(*_24) = Adt18::Variant0 { fld0: _7,fld1: _20,fld2: Field::<i64>(Variant(_11.fld7, 3), 6),fld3: _11.fld3,fld4: RET };
_11.fld1.0 = !Field::<(u8, isize)>(Variant((*_24), 0), 0).0;
_2 = [26862_i16,3888_i16,(-7083_i16),(-24460_i16),30447_i16,6564_i16,14084_i16];
_30 = [Field::<u8>(Variant(_11.fld7, 3), 1),Field::<(u8, isize)>(Variant(_11.fld5, 0), 0).0,Field::<(u8, isize)>(Variant(_11.fld5, 0), 0).0,Field::<(u8, isize)>(Variant((*_24), 0), 0).0];
_11.fld1.1 = 44188614670841149006512036122636162892_i128 as isize;
_33 = !_11.fld1.0;
_12 = !_32;
place!(Field::<i64>(Variant(_11.fld7, 3), 6)) = Field::<i64>(Variant((*_24), 0), 2);
_20 = Field::<char>(Variant((*_24), 0), 1);
SetDiscriminant(_11.fld5, 0);
SetDiscriminant(_11.fld7, 2);
place!(Field::<(u8, isize)>(Variant((*_24), 0), 0)) = (_8.0, _3);
_25 = 166277155090024439385234397203960883376_u128 as i8;
_21 = 3988823358572730982_u64 as f64;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(10_usize, 12_usize, Move(_12), 20_usize, Move(_20), 8_usize, Move(_8), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(10_usize, 33_usize, Move(_33), 3_usize, Move(_3), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(10_usize, 6_usize, Move(_6), 9_usize, Move(_9), 46_usize, _46, 46_usize, _46), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (u8, isize),mut _2: (u8, isize),mut _3: u8,mut _4: i8,mut _5: (u8, isize),mut _6: (u8, isize),mut _7: f64,mut _8: (u8, isize)) -> u32 {
mir! {
type RET = u32;
let _9: &'static Adt48;
let _10: u64;
let _11: isize;
let _12: Adt45;
let _13: [i32; 6];
let _14: *const Adt18;
let _15: u128;
let _16: ([i16; 6],);
let _17: i128;
let _18: [i32; 4];
let _19: u16;
let _20: isize;
let _21: u16;
let _22: f32;
let _23: [usize; 7];
let _24: *mut [usize; 3];
let _25: &'static (u8, isize);
let _26: isize;
let _27: f64;
let _28: i32;
let _29: *const char;
let _30: isize;
let _31: [i16; 7];
let _32: [i32; 4];
let _33: usize;
let _34: (&'static i64,);
let _35: [i64; 4];
let _36: &'static bool;
let _37: bool;
let _38: (&'static (u8,),);
let _39: usize;
let _40: &'static &'static i128;
let _41: [i16; 6];
let _42: char;
let _43: &'static [i16; 7];
let _44: u32;
let _45: ();
let _46: ();
{
_2.1 = !_8.1;
_2.1 = _1.1;
_7 = 2055228394491120993_i64 as f64;
_3 = _8.0 & _6.0;
_5.1 = -_2.1;
_2.1 = 28961_i16 as isize;
_5 = (_3, _2.1);
RET = 1604930420_u32 | 2948876237_u32;
RET = 4214079752_u32;
_2.1 = _1.1;
_5.1 = _6.1 | _1.1;
_5.1 = _2.1 * _8.1;
_2.1 = _1.1;
RET = 2068328136_i32 as u32;
_5.0 = _6.0 >> _6.0;
_8.0 = _2.0 ^ _6.0;
_8 = (_2.0, _2.1);
_8.0 = 14289278645755490553_u64 as u8;
_5.1 = !_8.1;
_10 = !1261333264077098638_u64;
Goto(bb1)
}
bb1 = {
_11 = -_5.1;
_8.1 = !_6.1;
RET = !805207786_u32;
_4 = _2.0 as i8;
_3 = !_2.0;
_1.0 = _3;
_12.fld1 = (_1.0, _11);
_8 = (_5.0, _5.1);
_6.1 = -_11;
_12.fld7 = Adt44::Variant1 { fld0: true };
_12.fld1.0 = !_8.0;
_2.0 = !_1.0;
_1.0 = _6.0 * _8.0;
_8.1 = _10 as isize;
_5.0 = _1.0;
RET = _7 as u32;
_6 = (_5.0, _1.1);
Call(place!(Field::<bool>(Variant(_12.fld7, 1), 0)) = fn12(_6, _6, _8.0, _6.0, _6, _6, _6, _5, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.fld2 = !_1.0;
Goto(bb3)
}
bb3 = {
_2 = (_12.fld2, _8.1);
_16.0 = [9391_i16,15412_i16,(-7551_i16),(-17573_i16),(-2283_i16),(-15920_i16)];
Goto(bb4)
}
bb4 = {
_12.fld0 = [(-369590001_i32),2048948997_i32,183335481_i32,(-1067252031_i32)];
_12.fld1.0 = _8.0 << _6.0;
_5.1 = _12.fld1.1 | _11;
_6.0 = _12.fld2;
_12.fld3 = _4 << _12.fld1.0;
_8.0 = !_5.0;
SetDiscriminant(_12.fld7, 2);
_11 = (-3831759591847230430_i64) as isize;
_8.1 = -_1.1;
_11 = _8.1 + _12.fld1.1;
_1.1 = !_6.1;
_8 = _6;
_13 = [72961302_i32,266304969_i32,782042686_i32,(-1168284826_i32),(-1928069282_i32),1015421050_i32];
Goto(bb5)
}
bb5 = {
_15 = !169000892904110760819923514890636319646_u128;
_2 = _8;
_13 = [(-1927589758_i32),1080315049_i32,998067601_i32,1207061399_i32,1757368815_i32,539385420_i32];
_8.1 = _12.fld3 as isize;
RET = 354300224_u32;
RET = 785994577_u32;
_14 = core::ptr::addr_of!(_12.fld5);
_17 = 114411414603603549197845426863134607326_i128;
_12.fld4 = 9137_u16 & 59899_u16;
(*_14) = Adt18::Variant1 { fld0: false,fld1: RET,fld2: _7,fld3: _17,fld4: _5.0,fld5: 1070486158_i32,fld6: 2224670318162006692_i64 };
place!(Field::<u8>(Variant((*_14), 1), 4)) = _8.0;
place!(Field::<bool>(Variant(_12.fld5, 1), 0)) = !true;
_18 = [1447841943_i32,(-1554033886_i32),483329804_i32,(-1659943945_i32)];
_20 = _8.1 + _8.1;
place!(Field::<*const u128>(Variant(_12.fld7, 2), 2)) = core::ptr::addr_of!(_15);
_12.fld1.1 = _20;
_3 = _12.fld4 as u8;
_17 = Field::<i128>(Variant((*_14), 1), 3);
_1 = (Field::<u8>(Variant((*_14), 1), 4), _20);
_8 = (_5.0, _20);
_8 = (_2.0, _20);
place!(Field::<i128>(Variant((*_14), 1), 3)) = _17 & _17;
_12.fld3 = _4 - _4;
_23 = [6603806605718189925_usize,0_usize,2_usize,813557442817806292_usize,7_usize,2177416771767988822_usize,2_usize];
_12.fld4 = _12.fld1.1 as u16;
Goto(bb6)
}
bb6 = {
place!(Field::<i64>(Variant(_12.fld5, 1), 6)) = !4970731225779811254_i64;
_1.1 = _8.1 | _12.fld1.1;
_12.fld3 = (-324235284_i32) as i8;
_16.0 = [23342_i16,(-14711_i16),(-24612_i16),29936_i16,(-19782_i16),(-18134_i16)];
_27 = Field::<f64>(Variant((*_14), 1), 2);
place!(Field::<i64>(Variant(_12.fld7, 2), 1)) = -Field::<i64>(Variant((*_14), 1), 6);
place!(Field::<i64>(Variant((*_14), 1), 6)) = Field::<i64>(Variant(_12.fld7, 2), 1) & Field::<i64>(Variant(_12.fld7, 2), 1);
place!(Field::<u32>(Variant((*_14), 1), 1)) = RET;
place!(Field::<i32>(Variant((*_14), 1), 5)) = -(-1200460134_i32);
_12.fld1.1 = _8.1;
_26 = (-1817_i16) as isize;
_2 = (Field::<u8>(Variant((*_14), 1), 4), _20);
place!(Field::<i32>(Variant((*_14), 1), 5)) = _12.fld1.1 as i32;
_11 = -_8.1;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
114411414603603549197845426863134607326 => bb7,
_ => bb5
}
}
bb7 = {
_12.fld4 = Field::<f64>(Variant((*_14), 1), 2) as u16;
_3 = _8.0;
place!(Field::<i64>(Variant(_12.fld5, 1), 6)) = -Field::<i64>(Variant(_12.fld7, 2), 1);
_1.1 = _20;
_8.1 = !_20;
SetDiscriminant(_12.fld5, 0);
place!(Field::<char>(Variant(_12.fld5, 0), 1)) = '\u{beed2}';
_5.0 = _12.fld4 as u8;
place!(Field::<(u8, isize)>(Variant((*_14), 0), 0)).1 = -_2.1;
_13 = [1329309701_i32,1560368246_i32,(-1598424193_i32),(-200216854_i32),(-1291452331_i32),(-2131757261_i32)];
place!(Field::<(u8, isize)>(Variant(_12.fld5, 0), 0)).0 = _2.0;
place!(Field::<char>(Variant((*_14), 0), 1)) = '\u{52d94}';
place!(Field::<f32>(Variant(_12.fld5, 0), 4)) = _27 as f32;
RET = !3931507422_u32;
_12.fld1.1 = Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).1 * _20;
place!(Field::<i64>(Variant(_12.fld5, 0), 2)) = Field::<i64>(Variant(_12.fld7, 2), 1);
Goto(bb8)
}
bb8 = {
_8 = (_12.fld2, Field::<(u8, isize)>(Variant((*_14), 0), 0).1);
place!(Field::<i16>(Variant(_12.fld7, 2), 0)) = !1820_i16;
place!(Field::<*const u128>(Variant(_12.fld7, 2), 2)) = core::ptr::addr_of!(_15);
_15 = _10 as u128;
_6.1 = Field::<(u8, isize)>(Variant((*_14), 0), 0).1 | Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).1;
place!(Field::<i64>(Variant((*_14), 0), 2)) = Field::<i64>(Variant(_12.fld7, 2), 1);
place!(Field::<(u8, isize)>(Variant((*_14), 0), 0)).1 = _5.1 - _8.1;
place!(Field::<i64>(Variant((*_14), 0), 2)) = Field::<i64>(Variant(_12.fld7, 2), 1);
(*_14) = Adt18::Variant1 { fld0: false,fld1: RET,fld2: _27,fld3: _17,fld4: _6.0,fld5: (-1417521779_i32),fld6: Field::<i64>(Variant(_12.fld7, 2), 1) };
place!(Field::<i128>(Variant(_12.fld5, 1), 3)) = -_17;
_12.fld1 = _1;
SetDiscriminant(_12.fld7, 3);
_25 = &_1;
_1 = (_6.0, _8.1);
_4 = _12.fld3 + _12.fld3;
place!(Field::<u64>(Variant(_12.fld7, 3), 5)) = _15 as u64;
place!(Field::<i32>(Variant((*_14), 1), 5)) = RET as i32;
_15 = 90722103574261721074539859548112432918_u128;
place!(Field::<f64>(Variant((*_14), 1), 2)) = _27;
Goto(bb9)
}
bb9 = {
_30 = Field::<i64>(Variant((*_14), 1), 6) as isize;
place!(Field::<i32>(Variant((*_14), 1), 5)) = (-1628762468_i32);
_8.0 = !_12.fld1.0;
_28 = Field::<i32>(Variant((*_14), 1), 5) | Field::<i32>(Variant((*_14), 1), 5);
place!(Field::<isize>(Variant(_12.fld7, 3), 2)) = _20 + _12.fld1.1;
_21 = '\u{6d3d2}' as u16;
(*_14) = Adt18::Variant1 { fld0: true,fld1: RET,fld2: _7,fld3: _17,fld4: _8.0,fld5: _28,fld6: (-2625896838943667752_i64) };
_12.fld1 = (Field::<u8>(Variant(_12.fld5, 1), 4), _8.1);
RET = Field::<u32>(Variant((*_14), 1), 1);
_12.fld6 = core::ptr::addr_of!(_12.fld3);
_5 = (_12.fld1.0, _1.1);
place!(Field::<u8>(Variant(_12.fld7, 3), 1)) = _8.0;
place!(Field::<i64>(Variant((*_14), 1), 6)) = '\u{ea49f}' as i64;
_16.0 = [(-6658_i16),29835_i16,(-13349_i16),27542_i16,(-7936_i16),13043_i16];
place!(Field::<f64>(Variant(_12.fld7, 3), 0)) = _2.1 as f64;
place!(Field::<u32>(Variant(_12.fld7, 3), 3)) = Field::<u32>(Variant((*_14), 1), 1);
_32 = [_28,Field::<i32>(Variant((*_14), 1), 5),Field::<i32>(Variant((*_14), 1), 5),_28];
_25 = &_2;
_12.fld3 = Field::<i64>(Variant((*_14), 1), 6) as i8;
Goto(bb10)
}
bb10 = {
_18 = [_28,Field::<i32>(Variant(_12.fld5, 1), 5),Field::<i32>(Variant(_12.fld5, 1), 5),Field::<i32>(Variant((*_14), 1), 5)];
place!(Field::<u8>(Variant(_12.fld7, 3), 1)) = (*_25).0 - _3;
_12.fld4 = _21 & _21;
match _17 {
0 => bb5,
1 => bb2,
114411414603603549197845426863134607326 => bb12,
_ => bb11
}
}
bb11 = {
_12.fld2 = !_1.0;
Goto(bb3)
}
bb12 = {
place!(Field::<u64>(Variant(_12.fld7, 3), 5)) = false as u64;
_27 = 20323_i16 as f64;
_34.0 = &place!(Field::<i64>(Variant((*_14), 1), 6));
place!(Field::<f64>(Variant(_12.fld5, 1), 2)) = _10 as f64;
place!(Field::<i32>(Variant((*_14), 1), 5)) = 2_usize as i32;
_29 = core::ptr::addr_of!(_42);
place!(Field::<isize>(Variant(_12.fld7, 3), 2)) = _11;
_42 = '\u{65a9e}';
_44 = !Field::<u32>(Variant(_12.fld5, 1), 1);
_18 = [_28,Field::<i32>(Variant((*_14), 1), 5),_28,Field::<i32>(Variant(_12.fld5, 1), 5)];
_32 = [_28,Field::<i32>(Variant((*_14), 1), 5),_28,Field::<i32>(Variant((*_14), 1), 5)];
_41 = [(-30906_i16),(-2101_i16),(-356_i16),15361_i16,(-25930_i16),13399_i16];
place!(Field::<i32>(Variant((*_14), 1), 5)) = _28 | _28;
place!(Field::<i128>(Variant((*_14), 1), 3)) = -_17;
_26 = -_5.1;
_5.0 = _2.0;
_8.1 = _12.fld4 as isize;
_39 = Field::<i128>(Variant((*_14), 1), 3) as usize;
_12.fld4 = !_21;
RET = _12.fld1.1 as u32;
_41 = [(-3749_i16),(-7449_i16),(-26178_i16),(-7491_i16),19223_i16,(-29107_i16)];
_19 = _21;
_2 = (_12.fld2, _12.fld1.1);
_15 = Field::<f64>(Variant(_12.fld7, 3), 0) as u128;
_37 = true & false;
_5 = (Field::<u8>(Variant(_12.fld7, 3), 1), _12.fld1.1);
_20 = -Field::<isize>(Variant(_12.fld7, 3), 2);
_2.0 = _37 as u8;
Goto(bb13)
}
bb13 = {
Call(_45 = dump_var(11_usize, 18_usize, Move(_18), 11_usize, Move(_11), 30_usize, Move(_30), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_45 = dump_var(11_usize, 17_usize, Move(_17), 13_usize, Move(_13), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_45 = dump_var(11_usize, 39_usize, Move(_39), 10_usize, Move(_10), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(11_usize, 28_usize, Move(_28), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (u8, isize),mut _2: (u8, isize),mut _3: u8,mut _4: u8,mut _5: (u8, isize),mut _6: (u8, isize),mut _7: (u8, isize),mut _8: (u8, isize),mut _9: (u8, isize)) -> bool {
mir! {
type RET = bool;
let _10: ();
let _11: ();
{
RET = true;
RET = !false;
_8.0 = !_6.0;
_6.0 = !_1.0;
_6.0 = _5.0 ^ _4;
_9.1 = _1.1 >> _9.0;
_6.0 = (-6747059150584055415964388006480887645_i128) as u8;
_2 = _5;
_1.0 = _8.0 >> _3;
RET = _4 != _2.0;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(12_usize, 8_usize, Move(_8), 6_usize, Move(_6), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [u8; 4],mut _2: [u8; 4]) -> u8 {
mir! {
type RET = u8;
let _3: bool;
let _4: u8;
let _5: i16;
let _6: f32;
let _7: u64;
let _8: f32;
let _9: (u8,);
let _10: f64;
let _11: char;
let _12: Adt45;
let _13: (Adt33, i8, Adt48);
let _14: u128;
let _15: char;
let _16: i32;
let _17: bool;
let _18: &'static (usize, f32, Adt18, &'static usize);
let _19: ();
let _20: ();
{
RET = 56_u8;
Call(RET = fn14(_1, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 1257690840_u32 as u8;
RET = false as u8;
_2 = [RET,RET,RET,RET];
_2 = [RET,RET,RET,RET];
_2 = _1;
_2 = _1;
_1 = _2;
_1 = [RET,RET,RET,RET];
_3 = false;
RET = 136_u8;
_1 = [RET,RET,RET,RET];
_3 = RET > RET;
_5 = 10122612981370975368_u64 as i16;
_4 = RET;
_2 = [_4,RET,RET,RET];
RET = _4;
RET = !_4;
Goto(bb2)
}
bb2 = {
_7 = (-4074338488475072022_i64) as u64;
_1 = _2;
_7 = 1241088294424661953_u64 * 10339958626543050915_u64;
_7 = !6280725827361581679_u64;
_4 = !RET;
_7 = !6975242384479348731_u64;
_5 = (-18761_i16);
_3 = true;
_2 = [RET,RET,_4,RET];
_4 = !RET;
_4 = RET - RET;
_5 = (-20620_i16);
RET = (-60120559522194556419916256388282209828_i128) as u8;
_8 = 26832650098050193646181472641312033922_i128 as f32;
_8 = 51601_u16 as f32;
match _5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768190836 => bb8,
_ => bb7
}
}
bb3 = {
RET = 1257690840_u32 as u8;
RET = false as u8;
_2 = [RET,RET,RET,RET];
_2 = [RET,RET,RET,RET];
_2 = _1;
_2 = _1;
_1 = _2;
_1 = [RET,RET,RET,RET];
_3 = false;
RET = 136_u8;
_1 = [RET,RET,RET,RET];
_3 = RET > RET;
_5 = 10122612981370975368_u64 as i16;
_4 = RET;
_2 = [_4,RET,RET,RET];
RET = _4;
RET = !_4;
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
_2 = _1;
_6 = _5 as f32;
_4 = RET | RET;
_5 = !(-28978_i16);
_8 = -_6;
_3 = true | false;
_3 = !true;
_4 = RET;
RET = !_4;
RET = (-81_i8) as u8;
_10 = 6_usize as f64;
RET = _4;
RET = !_4;
_5 = !13346_i16;
Goto(bb9)
}
bb9 = {
_12.fld3 = 86_i8;
_1 = _2;
_12.fld1.0 = !RET;
_8 = _6 * _6;
_12.fld5 = Adt18::Variant1 { fld0: _3,fld1: 552599757_u32,fld2: _10,fld3: 35149418994498095424757415634705492497_i128,fld4: _12.fld1.0,fld5: 435418222_i32,fld6: (-4777394081121171443_i64) };
_9.0 = !_12.fld1.0;
match _12.fld3 {
0 => bb1,
86 => bb10,
_ => bb7
}
}
bb10 = {
_12.fld3 = -81_i8;
_13.0.fld3 = (_9.0, (-9223372036854775808_isize));
_12.fld0 = [101053813_i32,1287031518_i32,(-1750500325_i32),(-396943093_i32)];
_13.1 = _12.fld3;
_13.0.fld3 = (_4, 9223372036854775807_isize);
place!(Field::<i128>(Variant(_12.fld5, 1), 3)) = 327159165446794738792490635601547193466_u128 as i128;
_12.fld6 = core::ptr::addr_of!(_12.fld3);
_13.0.fld4 = [3303004996_u32];
_13.0.fld3.0 = _12.fld1.0;
_3 = !Field::<bool>(Variant(_12.fld5, 1), 0);
_13.0.fld0 = 63975_u16 & 30632_u16;
_13.0.fld5 = [_5,_5,_5,_5,_5,_5,_5];
Call(_13.0.fld5 = fn15(Field::<i128>(Variant(_12.fld5, 1), 3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.fld3 = _13.1 + _13.1;
_12.fld5 = Adt18::Variant0 { fld0: _13.0.fld3,fld1: '\u{24306}',fld2: 2500067350854829568_i64,fld3: _12.fld3,fld4: _8 };
_12.fld3 = _13.0.fld3.1 as i8;
_13.0.fld3 = Field::<(u8, isize)>(Variant(_12.fld5, 0), 0);
_13.1 = -_12.fld3;
_12.fld0 = [1598536921_i32,701426976_i32,1211794005_i32,540383148_i32];
place!(Field::<char>(Variant(_12.fld5, 0), 1)) = '\u{862b3}';
_13.0.fld6 = (RET, RET);
_12.fld3 = _10 as i8;
_1 = [_4,_13.0.fld3.0,RET,_4];
_2 = _1;
_16 = 325228149254192159562388178745919289364_u128 as i32;
_12.fld1 = (_13.0.fld3.0, _13.0.fld3.1);
_1 = [_9.0,RET,RET,_13.0.fld3.0];
_12.fld1 = (_9.0, _13.0.fld3.1);
_2 = [RET,_4,Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).0,_12.fld1.0];
_11 = Field::<char>(Variant(_12.fld5, 0), 1);
_12.fld2 = _13.0.fld3.0;
_12.fld1.1 = !_13.0.fld3.1;
_12.fld5 = Adt18::Variant0 { fld0: _12.fld1,fld1: _11,fld2: 7796121244697959907_i64,fld3: _13.1,fld4: _6 };
place!(Field::<(u8, isize)>(Variant(_12.fld5, 0), 0)).1 = 8365534313857275612_i64 as isize;
_3 = !false;
_13.0.fld1 = _11;
_13.0.fld6 = (_9.0, _12.fld1.0);
RET = !_13.0.fld6.0;
_13.0.fld0 = 19038_u16 - 15923_u16;
_9.0 = _13.0.fld3.0 - _13.0.fld3.0;
match _13.0.fld3.1 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
9223372036854775807 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_13.1 = Field::<i8>(Variant(_12.fld5, 0), 3);
match _13.0.fld3.1 {
0 => bb8,
1 => bb2,
9223372036854775807 => bb15,
_ => bb14
}
}
bb14 = {
RET = 1257690840_u32 as u8;
RET = false as u8;
_2 = [RET,RET,RET,RET];
_2 = [RET,RET,RET,RET];
_2 = _1;
_2 = _1;
_1 = _2;
_1 = [RET,RET,RET,RET];
_3 = false;
RET = 136_u8;
_1 = [RET,RET,RET,RET];
_3 = RET > RET;
_5 = 10122612981370975368_u64 as i16;
_4 = RET;
_2 = [_4,RET,RET,RET];
RET = _4;
RET = !_4;
Goto(bb2)
}
bb15 = {
_13.0.fld7 = _16 as u64;
_13.0.fld2 = Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).0 >> Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).0;
_12.fld0 = [_16,_16,_16,_16];
_12.fld1 = Field::<(u8, isize)>(Variant(_12.fld5, 0), 0);
_17 = _13.0.fld6.0 <= _4;
_16 = (-109210975423298361889298092536683166373_i128) as i32;
RET = _13.0.fld6.1;
_12.fld5 = Adt18::Variant0 { fld0: _12.fld1,fld1: _13.0.fld1,fld2: (-7246449901465608084_i64),fld3: _13.1,fld4: _6 };
RET = (-78518012542086968740386420196237921414_i128) as u8;
place!(Field::<(u8, isize)>(Variant(_12.fld5, 0), 0)).1 = 457918457262443917_i64 as isize;
_13.1 = 291915780182054698411383742137604292972_u128 as i8;
_13.0.fld6 = (_9.0, _9.0);
_16 = _12.fld1.0 as i32;
_9 = (Field::<(u8, isize)>(Variant(_12.fld5, 0), 0).0,);
_13.0.fld0 = 299921099090818501495449705970190046804_u128 as u16;
_11 = Field::<char>(Variant(_12.fld5, 0), 1);
place!(Field::<char>(Variant(_12.fld5, 0), 1)) = _11;
_14 = 30716741182713601786592938062273997983_u128 + 68832295409232496159569866372151305516_u128;
_13.0.fld2 = _12.fld2 - _4;
_10 = _13.0.fld0 as f64;
_12.fld4 = _5 as u16;
place!(Field::<i8>(Variant(_12.fld5, 0), 3)) = 3947880639838798983_usize as i8;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(13_usize, 3_usize, Move(_3), 16_usize, Move(_16), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(13_usize, 9_usize, Move(_9), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [u8; 4],mut _2: [u8; 4],mut _3: [u8; 4],mut _4: [u8; 4]) -> u8 {
mir! {
type RET = u8;
let _5: isize;
let _6: bool;
let _7: isize;
let _8: (bool, u16, [u32; 1], u16);
let _9: &'static i128;
let _10: *mut [usize; 3];
let _11: f64;
let _12: bool;
let _13: i16;
let _14: [u16; 5];
let _15: Adt77;
let _16: *mut [i16; 6];
let _17: f32;
let _18: *const [u32; 1];
let _19: ();
let _20: ();
{
_4 = [27_u8,92_u8,65_u8,151_u8];
_1 = _3;
RET = 15520086800686575831_u64 as u8;
_5 = (-9223372036854775808_isize) | 2_isize;
RET = !52_u8;
_1 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
_2 = _4;
RET = !80_u8;
_1 = [RET,RET,RET,RET];
RET = 2_usize as u8;
_5 = (-13_isize) ^ (-45_isize);
_5 = 123_isize * (-9223372036854775808_isize);
_1 = _4;
_3 = _2;
_8.0 = _5 < _5;
Goto(bb1)
}
bb1 = {
_5 = !9223372036854775807_isize;
RET = 31949_i16 as u8;
_7 = '\u{ab716}' as isize;
_5 = _7 >> RET;
_8.2 = [259976495_u32];
_6 = _8.0 <= _8.0;
_8.1 = !53349_u16;
_4 = _2;
_2 = _4;
_5 = !_7;
_1 = [RET,RET,RET,RET];
RET = 4214241656248249226_usize as u8;
RET = 90_u8;
RET = 45_u8 * 189_u8;
_6 = !_8.0;
_6 = _8.0 <= _8.0;
RET = 2962913748_u32 as u8;
_8.2 = [1619715228_u32];
RET = 17_u8;
_2 = [RET,RET,RET,RET];
_8.1 = 41640_u16;
_5 = _7;
Goto(bb2)
}
bb2 = {
RET = !8_u8;
_8.3 = 330194650530066426268138008612810915572_u128 as u16;
_5 = -_7;
_3 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
Goto(bb3)
}
bb3 = {
RET = _8.1 as u8;
_8.3 = _8.1 - _8.1;
RET = 134_u8;
match _8.1 {
0 => bb1,
1 => bb4,
2 => bb5,
41640 => bb7,
_ => bb6
}
}
bb4 = {
RET = !8_u8;
_8.3 = 330194650530066426268138008612810915572_u128 as u16;
_5 = -_7;
_3 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
Goto(bb3)
}
bb5 = {
_5 = !9223372036854775807_isize;
RET = 31949_i16 as u8;
_7 = '\u{ab716}' as isize;
_5 = _7 >> RET;
_8.2 = [259976495_u32];
_6 = _8.0 <= _8.0;
_8.1 = !53349_u16;
_4 = _2;
_2 = _4;
_5 = !_7;
_1 = [RET,RET,RET,RET];
RET = 4214241656248249226_usize as u8;
RET = 90_u8;
RET = 45_u8 * 189_u8;
_6 = !_8.0;
_6 = _8.0 <= _8.0;
RET = 2962913748_u32 as u8;
_8.2 = [1619715228_u32];
RET = 17_u8;
_2 = [RET,RET,RET,RET];
_8.1 = 41640_u16;
_5 = _7;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_2 = [RET,RET,RET,RET];
_1 = [RET,RET,RET,RET];
_8.3 = _6 as u16;
_8.2 = [1983573082_u32];
_1 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
_8.0 = _6;
_11 = 13354907010773734589_u64 as f64;
_7 = !_5;
_1 = [RET,RET,RET,RET];
_8.3 = _8.1;
_3 = [RET,RET,RET,RET];
_6 = _8.0;
_4 = _2;
_8.1 = _8.3;
Goto(bb8)
}
bb8 = {
_5 = 16760_i16 as isize;
_4 = [RET,RET,RET,RET];
_6 = _8.1 >= _8.1;
_7 = _5;
_12 = !_8.0;
_7 = _5 + _5;
_1 = [RET,RET,RET,RET];
RET = 98_u8 >> _5;
_4 = [RET,RET,RET,RET];
_5 = _7;
RET = 101_u8 >> _7;
_13 = 28037_i16;
_8.3 = _8.1 | _8.1;
_13 = (-22155_i16);
_1 = _3;
_6 = _12 & _8.0;
RET = 11_u8;
_7 = _5 - _5;
_4 = _1;
_3 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
_8.3 = _8.1 << _5;
RET = !108_u8;
_8.0 = _12 ^ _6;
match _8.1 {
0 => bb5,
1 => bb7,
2 => bb3,
3 => bb9,
4 => bb10,
5 => bb11,
41640 => bb13,
_ => bb12
}
}
bb9 = {
RET = !8_u8;
_8.3 = 330194650530066426268138008612810915572_u128 as u16;
_5 = -_7;
_3 = [RET,RET,RET,RET];
_3 = [RET,RET,RET,RET];
Goto(bb3)
}
bb10 = {
Return()
}
bb11 = {
_5 = !9223372036854775807_isize;
RET = 31949_i16 as u8;
_7 = '\u{ab716}' as isize;
_5 = _7 >> RET;
_8.2 = [259976495_u32];
_6 = _8.0 <= _8.0;
_8.1 = !53349_u16;
_4 = _2;
_2 = _4;
_5 = !_7;
_1 = [RET,RET,RET,RET];
RET = 4214241656248249226_usize as u8;
RET = 90_u8;
RET = 45_u8 * 189_u8;
_6 = !_8.0;
_6 = _8.0 <= _8.0;
RET = 2962913748_u32 as u8;
_8.2 = [1619715228_u32];
RET = 17_u8;
_2 = [RET,RET,RET,RET];
_8.1 = 41640_u16;
_5 = _7;
Goto(bb2)
}
bb12 = {
RET = _8.1 as u8;
_8.3 = _8.1 - _8.1;
RET = 134_u8;
match _8.1 {
0 => bb1,
1 => bb4,
2 => bb5,
41640 => bb7,
_ => bb6
}
}
bb13 = {
RET = !130_u8;
_14 = [_8.1,_8.1,_8.3,_8.1,_8.1];
_4 = [RET,RET,RET,RET];
_6 = _8.0 & _12;
_14 = [_8.1,_8.1,_8.3,_8.3,_8.3];
RET = !12_u8;
_8.0 = _6 != _6;
_8.0 = !_6;
_7 = _5;
_13 = 22447_i16;
_8.3 = _8.1 % _8.1;
_11 = 10036055950896832130_u64 as f64;
_2 = _4;
RET = _11 as u8;
_4 = _3;
_8.0 = _6 & _6;
_13 = (-7198_i16);
_2 = [RET,RET,RET,RET];
Goto(bb14)
}
bb14 = {
_8.1 = _8.3;
_4 = [RET,RET,RET,RET];
_1 = _2;
_14 = [_8.1,_8.1,_8.3,_8.1,_8.3];
RET = 4195066803_u32 as u8;
_7 = 1106479128_u32 as isize;
_17 = _13 as f32;
_1 = _2;
_11 = _17 as f64;
_8.0 = _6 ^ _6;
_17 = RET as f32;
_14 = [_8.3,_8.1,_8.1,_8.3,_8.1];
_12 = _8.0 >= _6;
_8.0 = _12;
_1 = [RET,RET,RET,RET];
_8.1 = _8.3;
_14 = [_8.3,_8.1,_8.3,_8.3,_8.3];
_18 = core::ptr::addr_of!(_8.2);
_6 = _12;
(*_18) = [1769654667_u32];
_6 = _8.0 | _12;
_18 = core::ptr::addr_of!((*_18));
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(14_usize, 13_usize, Move(_13), 6_usize, Move(_6), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(14_usize, 7_usize, Move(_7), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i128) -> [i16; 7] {
mir! {
type RET = [i16; 7];
let _2: f64;
let _3: [usize; 7];
let _4: [u32; 1];
let _5: (&'static (u8,),);
let _6: &'static i64;
let _7: [usize; 3];
let _8: &'static &'static (u8,);
let _9: &'static i128;
let _10: u16;
let _11: i32;
let _12: [isize; 1];
let _13: (Adt33, (u8,), (&'static (u8,),), u32);
let _14: isize;
let _15: [u8; 7];
let _16: isize;
let _17: i8;
let _18: isize;
let _19: (bool, u16, [u32; 1], u16);
let _20: u128;
let _21: isize;
let _22: f64;
let _23: (u8, isize);
let _24: (bool, u16, [u32; 1], u16);
let _25: char;
let _26: Adt45;
let _27: &'static &'static (u8,);
let _28: [bool; 8];
let _29: ();
let _30: ();
{
RET = [10352_i16,(-16244_i16),(-1133_i16),(-3650_i16),(-22305_i16),8726_i16,(-16632_i16)];
_2 = 2106085607_i32 as f64;
RET = [(-16461_i16),8744_i16,(-21845_i16),31714_i16,23780_i16,2242_i16,17110_i16];
RET = [12516_i16,(-11047_i16),(-8999_i16),6126_i16,2391_i16,30039_i16,22037_i16];
_2 = (-1488763629_i32) as f64;
_1 = 70300275245193973308722669129996882392_i128 * 147796223018747276955297567405866074582_i128;
_1 = (-18961976648791792751064705718629469555_i128);
RET = [(-9398_i16),(-23166_i16),(-9388_i16),31900_i16,11731_i16,26058_i16,(-3186_i16)];
RET = [(-966_i16),4652_i16,(-29778_i16),10318_i16,19868_i16,17216_i16,(-21171_i16)];
_2 = 39580760741230345367783084648774640874_u128 as f64;
RET = [(-21754_i16),13588_i16,8856_i16,20424_i16,10889_i16,14564_i16,1473_i16];
_2 = 26243_i16 as f64;
RET = [(-12876_i16),5642_i16,(-13201_i16),(-8243_i16),303_i16,(-22024_i16),(-31784_i16)];
_3 = [4_usize,4_usize,4_usize,7_usize,204392017642352751_usize,6_usize,7_usize];
RET = [7167_i16,(-8055_i16),(-1386_i16),(-20979_i16),(-8500_i16),28032_i16,(-30390_i16)];
_3 = [11218429398096137082_usize,7_usize,3_usize,6040075872427582866_usize,0_usize,1418381457117324147_usize,8552695071095357860_usize];
_3 = [3126489205870558360_usize,0_usize,4_usize,7_usize,2_usize,11392099222149165010_usize,13622632485298791406_usize];
_4 = [4098754685_u32];
_3 = [6120278322350245930_usize,10168133166065705384_usize,5_usize,13447080369280592401_usize,1_usize,4_usize,81809943424380440_usize];
Goto(bb1)
}
bb1 = {
_2 = 11337_u16 as f64;
_3 = [3_usize,5_usize,13580997628325461676_usize,1456428918473067308_usize,2800325675148416487_usize,4_usize,4_usize];
_2 = 47_i8 as f64;
_3 = [16951421866432964133_usize,11374016798232127718_usize,6502953094286166598_usize,1_usize,14637084199349175496_usize,1205468439392244440_usize,0_usize];
_1 = !(-153465668803274393422964533236904807201_i128);
_4 = [2461297157_u32];
_1 = 108166177541219367678884333457491272100_i128;
RET = [(-7047_i16),(-14627_i16),(-22130_i16),21067_i16,(-13259_i16),6003_i16,(-309_i16)];
_3 = [2_usize,17474164439016748408_usize,3_usize,7_usize,6_usize,2_usize,7_usize];
RET = [11340_i16,16601_i16,7767_i16,(-10955_i16),(-15918_i16),8656_i16,(-25473_i16)];
_4 = [2741646972_u32];
Goto(bb2)
}
bb2 = {
_3 = [6_usize,4_usize,10383328281661915989_usize,1_usize,4_usize,4270314805590384849_usize,9225169518370953448_usize];
_3 = [5020865219554599600_usize,15650647596908631572_usize,2_usize,16804839170438054039_usize,1_usize,3_usize,6_usize];
_3 = [1469737223466622315_usize,1_usize,1613787670375354230_usize,9922928916000353448_usize,15079774145049774641_usize,9994824313703153355_usize,3_usize];
_2 = 126_i8 as f64;
RET = [(-14064_i16),(-25947_i16),17472_i16,(-13063_i16),7894_i16,19649_i16,13132_i16];
_4 = [2276316920_u32];
_2 = 74_isize as f64;
RET = [31068_i16,(-23159_i16),(-685_i16),19339_i16,5125_i16,17517_i16,16050_i16];
_3 = [1_usize,0_usize,5414016011243813192_usize,3533594506431342269_usize,2_usize,7453682680172320815_usize,11795786249831612485_usize];
_1 = 2026208936_i32 as i128;
_3 = [2914549743571113776_usize,7972533205815508834_usize,4326225499510674786_usize,4038890462101066308_usize,2843688694320583572_usize,9035309577660127552_usize,7797053674830083019_usize];
_1 = !127846945015507978115883011936560413942_i128;
RET = [(-4883_i16),(-29521_i16),(-19934_i16),11433_i16,(-11916_i16),(-2522_i16),13764_i16];
RET = [29961_i16,25778_i16,21319_i16,(-23560_i16),(-31862_i16),(-19133_i16),20773_i16];
_3 = [5_usize,5333444601521943755_usize,14547618925184065684_usize,3004720527075024738_usize,1_usize,3_usize,2_usize];
_3 = [7_usize,11663866036980456635_usize,16289180401986507109_usize,17475115818244972818_usize,7_usize,719595615682743464_usize,5600106495698532793_usize];
RET = [23896_i16,17438_i16,(-12863_i16),4630_i16,(-5803_i16),(-26867_i16),(-28858_i16)];
RET = [19287_i16,30727_i16,(-29807_i16),2791_i16,31209_i16,32113_i16,(-3824_i16)];
RET = [5685_i16,18013_i16,16204_i16,(-7126_i16),28962_i16,14083_i16,5543_i16];
_7 = [7_usize,3_usize,9974684493976715962_usize];
_7 = [4_usize,0_usize,5917913024096943893_usize];
_2 = 86_i8 as f64;
_2 = 118_i8 as f64;
_4 = [589122971_u32];
_9 = &_1;
Goto(bb3)
}
bb3 = {
_9 = &_1;
_9 = &(*_9);
_3 = [5992806753722777807_usize,2012842648317865420_usize,8534172431140911787_usize,3_usize,6796335277423315989_usize,14916269966872297443_usize,5_usize];
_10 = 2528_u16 << _1;
_4 = [3766346610_u32];
_9 = &(*_9);
_8 = &_5.0;
_10 = 36404_u16 - 45418_u16;
_11 = (-1678872623_i32);
_9 = &(*_9);
_8 = &(*_8);
_2 = 6503467446155507126_u64 as f64;
_4 = [549363443_u32];
_7 = [9097609224019506429_usize,7_usize,7_usize];
_3 = [8131183742299810088_usize,2766163909174630942_usize,14848312338204335333_usize,5184795126610799027_usize,1_usize,4_usize,5_usize];
_12 = [(-61_isize)];
_13.1 = (115_u8,);
match _13.1.0 {
115 => bb4,
_ => bb2
}
}
bb4 = {
_5.0 = &_13.1;
_12 = [(-9223372036854775808_isize)];
_7 = [1_usize,10012697718205867873_usize,10290788267736336313_usize];
_13.0.fld3.1 = -(-9223372036854775808_isize);
_8 = &_5.0;
_13.0.fld3.0 = 9964880948726324882_usize as u8;
_13.3 = 2549631982_u32;
Goto(bb5)
}
bb5 = {
_13.2 = Move(_5);
_5.0 = &_13.1;
_13.0.fld2 = _13.0.fld3.0;
_14 = 141910508750482380944153574082731148468_u128 as isize;
_13.0.fld1 = '\u{a1bb1}';
_13.0.fld3 = (_13.0.fld2, _14);
_8 = &_5.0;
_9 = &(*_9);
_9 = &(*_9);
_13.0.fld6.0 = _10 as u8;
_15 = [_13.0.fld3.0,_13.0.fld3.0,_13.0.fld3.0,_13.0.fld3.0,_13.0.fld3.0,_13.0.fld6.0,_13.1.0];
_13.0.fld4 = [_13.3];
_13.0.fld5 = [(-4096_i16),(-23938_i16),(-18337_i16),12522_i16,13827_i16,(-31517_i16),7428_i16];
RET = [(-31117_i16),(-934_i16),(-17500_i16),27497_i16,29274_i16,(-12298_i16),(-20074_i16)];
_14 = _13.0.fld3.1;
_2 = _13.0.fld3.1 as f64;
_9 = &_1;
_14 = _13.0.fld3.1;
_13.0.fld1 = '\u{3ad52}';
_13.0.fld6.1 = _11 as u8;
_12 = [_14];
_1 = _2 as i128;
_13.0.fld3.1 = -_14;
_2 = _1 as f64;
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607430089338833 => bb10,
_ => bb9
}
}
bb6 = {
_5.0 = &_13.1;
_12 = [(-9223372036854775808_isize)];
_7 = [1_usize,10012697718205867873_usize,10290788267736336313_usize];
_13.0.fld3.1 = -(-9223372036854775808_isize);
_8 = &_5.0;
_13.0.fld3.0 = 9964880948726324882_usize as u8;
_13.3 = 2549631982_u32;
Goto(bb5)
}
bb7 = {
_9 = &_1;
_9 = &(*_9);
_3 = [5992806753722777807_usize,2012842648317865420_usize,8534172431140911787_usize,3_usize,6796335277423315989_usize,14916269966872297443_usize,5_usize];
_10 = 2528_u16 << _1;
_4 = [3766346610_u32];
_9 = &(*_9);
_8 = &_5.0;
_10 = 36404_u16 - 45418_u16;
_11 = (-1678872623_i32);
_9 = &(*_9);
_8 = &(*_8);
_2 = 6503467446155507126_u64 as f64;
_4 = [549363443_u32];
_7 = [9097609224019506429_usize,7_usize,7_usize];
_3 = [8131183742299810088_usize,2766163909174630942_usize,14848312338204335333_usize,5184795126610799027_usize,1_usize,4_usize,5_usize];
_12 = [(-61_isize)];
_13.1 = (115_u8,);
match _13.1.0 {
115 => bb4,
_ => bb2
}
}
bb8 = {
_3 = [6_usize,4_usize,10383328281661915989_usize,1_usize,4_usize,4270314805590384849_usize,9225169518370953448_usize];
_3 = [5020865219554599600_usize,15650647596908631572_usize,2_usize,16804839170438054039_usize,1_usize,3_usize,6_usize];
_3 = [1469737223466622315_usize,1_usize,1613787670375354230_usize,9922928916000353448_usize,15079774145049774641_usize,9994824313703153355_usize,3_usize];
_2 = 126_i8 as f64;
RET = [(-14064_i16),(-25947_i16),17472_i16,(-13063_i16),7894_i16,19649_i16,13132_i16];
_4 = [2276316920_u32];
_2 = 74_isize as f64;
RET = [31068_i16,(-23159_i16),(-685_i16),19339_i16,5125_i16,17517_i16,16050_i16];
_3 = [1_usize,0_usize,5414016011243813192_usize,3533594506431342269_usize,2_usize,7453682680172320815_usize,11795786249831612485_usize];
_1 = 2026208936_i32 as i128;
_3 = [2914549743571113776_usize,7972533205815508834_usize,4326225499510674786_usize,4038890462101066308_usize,2843688694320583572_usize,9035309577660127552_usize,7797053674830083019_usize];
_1 = !127846945015507978115883011936560413942_i128;
RET = [(-4883_i16),(-29521_i16),(-19934_i16),11433_i16,(-11916_i16),(-2522_i16),13764_i16];
RET = [29961_i16,25778_i16,21319_i16,(-23560_i16),(-31862_i16),(-19133_i16),20773_i16];
_3 = [5_usize,5333444601521943755_usize,14547618925184065684_usize,3004720527075024738_usize,1_usize,3_usize,2_usize];
_3 = [7_usize,11663866036980456635_usize,16289180401986507109_usize,17475115818244972818_usize,7_usize,719595615682743464_usize,5600106495698532793_usize];
RET = [23896_i16,17438_i16,(-12863_i16),4630_i16,(-5803_i16),(-26867_i16),(-28858_i16)];
RET = [19287_i16,30727_i16,(-29807_i16),2791_i16,31209_i16,32113_i16,(-3824_i16)];
RET = [5685_i16,18013_i16,16204_i16,(-7126_i16),28962_i16,14083_i16,5543_i16];
_7 = [7_usize,3_usize,9974684493976715962_usize];
_7 = [4_usize,0_usize,5917913024096943893_usize];
_2 = 86_i8 as f64;
_2 = 118_i8 as f64;
_4 = [589122971_u32];
_9 = &_1;
Goto(bb3)
}
bb9 = {
_2 = 11337_u16 as f64;
_3 = [3_usize,5_usize,13580997628325461676_usize,1456428918473067308_usize,2800325675148416487_usize,4_usize,4_usize];
_2 = 47_i8 as f64;
_3 = [16951421866432964133_usize,11374016798232127718_usize,6502953094286166598_usize,1_usize,14637084199349175496_usize,1205468439392244440_usize,0_usize];
_1 = !(-153465668803274393422964533236904807201_i128);
_4 = [2461297157_u32];
_1 = 108166177541219367678884333457491272100_i128;
RET = [(-7047_i16),(-14627_i16),(-22130_i16),21067_i16,(-13259_i16),6003_i16,(-309_i16)];
_3 = [2_usize,17474164439016748408_usize,3_usize,7_usize,6_usize,2_usize,7_usize];
RET = [11340_i16,16601_i16,7767_i16,(-10955_i16),(-15918_i16),8656_i16,(-25473_i16)];
_4 = [2741646972_u32];
Goto(bb2)
}
bb10 = {
_13.1 = (_13.0.fld6.0,);
_19.1 = _10 ^ _10;
_17 = (-68_i8);
_13.0.fld4 = [_13.3];
RET = [29328_i16,(-10460_i16),19420_i16,(-16419_i16),(-9120_i16),(-7366_i16),(-30402_i16)];
_1 = _17 as i128;
_13.1 = (_13.0.fld6.0,);
_13.0.fld6.1 = !_13.0.fld2;
_13.3 = !3742521447_u32;
_13.0.fld7 = !16931141543012306503_u64;
_18 = _13.0.fld3.1;
match _17 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb8,
4 => bb5,
340282366920938463463374607431768211388 => bb11,
_ => bb6
}
}
bb11 = {
_19.2 = _13.0.fld4;
_23 = (_13.1.0, _14);
_4 = [_13.3];
_19 = (false, _10, _4, _10);
_16 = _18 + _13.0.fld3.1;
_12 = [_18];
_10 = _19.3 * _19.3;
_13.0.fld3.1 = _16 ^ _16;
_13.0.fld3.1 = _16;
_22 = _2;
_2 = _22 * _22;
_13.0.fld6.1 = !_13.0.fld3.0;
_21 = -_23.1;
_19.0 = _16 >= _13.0.fld3.1;
_22 = _17 as f64;
_13.2.0 = &_13.1;
_13.0.fld3.1 = _16;
_24.2 = [_13.3];
_19.2 = [_13.3];
_24.1 = !_10;
_17 = _2 as i8;
_24.3 = _10 >> _13.1.0;
_26.fld4 = !_19.3;
match _11 {
0 => bb8,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
340282366920938463463374607430089338833 => bb18,
_ => bb17
}
}
bb12 = {
_13.1 = (_13.0.fld6.0,);
_19.1 = _10 ^ _10;
_17 = (-68_i8);
_13.0.fld4 = [_13.3];
RET = [29328_i16,(-10460_i16),19420_i16,(-16419_i16),(-9120_i16),(-7366_i16),(-30402_i16)];
_1 = _17 as i128;
_13.1 = (_13.0.fld6.0,);
_13.0.fld6.1 = !_13.0.fld2;
_13.3 = !3742521447_u32;
_13.0.fld7 = !16931141543012306503_u64;
_18 = _13.0.fld3.1;
match _17 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb8,
4 => bb5,
340282366920938463463374607431768211388 => bb11,
_ => bb6
}
}
bb13 = {
_2 = 11337_u16 as f64;
_3 = [3_usize,5_usize,13580997628325461676_usize,1456428918473067308_usize,2800325675148416487_usize,4_usize,4_usize];
_2 = 47_i8 as f64;
_3 = [16951421866432964133_usize,11374016798232127718_usize,6502953094286166598_usize,1_usize,14637084199349175496_usize,1205468439392244440_usize,0_usize];
_1 = !(-153465668803274393422964533236904807201_i128);
_4 = [2461297157_u32];
_1 = 108166177541219367678884333457491272100_i128;
RET = [(-7047_i16),(-14627_i16),(-22130_i16),21067_i16,(-13259_i16),6003_i16,(-309_i16)];
_3 = [2_usize,17474164439016748408_usize,3_usize,7_usize,6_usize,2_usize,7_usize];
RET = [11340_i16,16601_i16,7767_i16,(-10955_i16),(-15918_i16),8656_i16,(-25473_i16)];
_4 = [2741646972_u32];
Goto(bb2)
}
bb14 = {
_9 = &_1;
_9 = &(*_9);
_3 = [5992806753722777807_usize,2012842648317865420_usize,8534172431140911787_usize,3_usize,6796335277423315989_usize,14916269966872297443_usize,5_usize];
_10 = 2528_u16 << _1;
_4 = [3766346610_u32];
_9 = &(*_9);
_8 = &_5.0;
_10 = 36404_u16 - 45418_u16;
_11 = (-1678872623_i32);
_9 = &(*_9);
_8 = &(*_8);
_2 = 6503467446155507126_u64 as f64;
_4 = [549363443_u32];
_7 = [9097609224019506429_usize,7_usize,7_usize];
_3 = [8131183742299810088_usize,2766163909174630942_usize,14848312338204335333_usize,5184795126610799027_usize,1_usize,4_usize,5_usize];
_12 = [(-61_isize)];
_13.1 = (115_u8,);
match _13.1.0 {
115 => bb4,
_ => bb2
}
}
bb15 = {
_9 = &_1;
_9 = &(*_9);
_3 = [5992806753722777807_usize,2012842648317865420_usize,8534172431140911787_usize,3_usize,6796335277423315989_usize,14916269966872297443_usize,5_usize];
_10 = 2528_u16 << _1;
_4 = [3766346610_u32];
_9 = &(*_9);
_8 = &_5.0;
_10 = 36404_u16 - 45418_u16;
_11 = (-1678872623_i32);
_9 = &(*_9);
_8 = &(*_8);
_2 = 6503467446155507126_u64 as f64;
_4 = [549363443_u32];
_7 = [9097609224019506429_usize,7_usize,7_usize];
_3 = [8131183742299810088_usize,2766163909174630942_usize,14848312338204335333_usize,5184795126610799027_usize,1_usize,4_usize,5_usize];
_12 = [(-61_isize)];
_13.1 = (115_u8,);
match _13.1.0 {
115 => bb4,
_ => bb2
}
}
bb16 = {
_5.0 = &_13.1;
_12 = [(-9223372036854775808_isize)];
_7 = [1_usize,10012697718205867873_usize,10290788267736336313_usize];
_13.0.fld3.1 = -(-9223372036854775808_isize);
_8 = &_5.0;
_13.0.fld3.0 = 9964880948726324882_usize as u8;
_13.3 = 2549631982_u32;
Goto(bb5)
}
bb17 = {
_2 = 11337_u16 as f64;
_3 = [3_usize,5_usize,13580997628325461676_usize,1456428918473067308_usize,2800325675148416487_usize,4_usize,4_usize];
_2 = 47_i8 as f64;
_3 = [16951421866432964133_usize,11374016798232127718_usize,6502953094286166598_usize,1_usize,14637084199349175496_usize,1205468439392244440_usize,0_usize];
_1 = !(-153465668803274393422964533236904807201_i128);
_4 = [2461297157_u32];
_1 = 108166177541219367678884333457491272100_i128;
RET = [(-7047_i16),(-14627_i16),(-22130_i16),21067_i16,(-13259_i16),6003_i16,(-309_i16)];
_3 = [2_usize,17474164439016748408_usize,3_usize,7_usize,6_usize,2_usize,7_usize];
RET = [11340_i16,16601_i16,7767_i16,(-10955_i16),(-15918_i16),8656_i16,(-25473_i16)];
_4 = [2741646972_u32];
Goto(bb2)
}
bb18 = {
_17 = _14 as i8;
_9 = &_1;
_26.fld0 = [_11,_11,_11,_11];
_26.fld1 = (_13.0.fld6.0, _21);
_7 = [11369124125748781072_usize,2_usize,0_usize];
_24 = _19;
_19.3 = !_24.1;
_21 = _24.0 as isize;
_26.fld5 = Adt18::Variant1 { fld0: _24.0,fld1: _13.3,fld2: _2,fld3: _1,fld4: _13.0.fld6.1,fld5: _11,fld6: (-2185117685672610067_i64) };
_6 = &place!(Field::<i64>(Variant(_26.fld5, 1), 6));
_24.1 = _26.fld4;
_13.1 = (_23.0,);
place!(Field::<f64>(Variant(_26.fld5, 1), 2)) = -_22;
Goto(bb19)
}
bb19 = {
Call(_29 = dump_var(15_usize, 21_usize, Move(_21), 7_usize, Move(_7), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_29 = dump_var(15_usize, 1_usize, Move(_1), 14_usize, Move(_14), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: i32,mut _3: *const (bool, u16, [u32; 1], u16),mut _4: Adt18,mut _5: Adt18) -> u8 {
mir! {
type RET = u8;
let _6: f32;
let _7: &'static ([bool; 8],);
let _8: &'static i128;
let _9: bool;
let _10: f32;
let _11: i64;
let _12: isize;
let _13: isize;
let _14: &'static *mut i64;
let _15: [i64; 4];
let _16: isize;
let _17: [i16; 7];
let _18: isize;
let _19: *const [u32; 1];
let _20: &'static [i16; 6];
let _21: char;
let _22: [u16; 5];
let _23: &'static i64;
let _24: usize;
let _25: char;
let _26: &'static i64;
let _27: u32;
let _28: (u8, isize);
let _29: bool;
let _30: Adt45;
let _31: (&'static (u8,),);
let _32: i32;
let _33: isize;
let _34: (u8,);
let _35: ();
let _36: ();
{
place!(Field::<i64>(Variant(_4, 1), 6)) = _1 as i64;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_4, 1), 1) ^ Field::<u32>(Variant(_4, 1), 1);
SetDiscriminant(_5, 0);
match Field::<i32>(Variant(_4, 1), 5) {
0 => bb1,
1 => bb2,
2109002812 => bb4,
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
RET = 4_usize as u8;
place!(Field::<f32>(Variant(_5, 0), 4)) = 28020_u16 as f32;
_5 = _4;
place!(Field::<i64>(Variant(_5, 1), 6)) = Field::<i64>(Variant(_4, 1), 6);
place!(Field::<u8>(Variant(_5, 1), 4)) = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<u8>(Variant(_4, 1), 4)) = Field::<i128>(Variant(_5, 1), 3) as u8;
_6 = _2 as f32;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<f64>(Variant(_4, 1), 2) as u32;
place!(Field::<bool>(Variant(_5, 1), 0)) = _1 < _1;
place!(Field::<i128>(Variant(_5, 1), 3)) = Field::<i128>(Variant(_4, 1), 3);
_2 = Field::<i32>(Variant(_4, 1), 5) * Field::<i32>(Variant(_4, 1), 5);
place!(Field::<i128>(Variant(_5, 1), 3)) = -Field::<i128>(Variant(_4, 1), 3);
place!(Field::<i32>(Variant(_5, 1), 5)) = Field::<i32>(Variant(_4, 1), 5);
place!(Field::<f64>(Variant(_4, 1), 2)) = -Field::<f64>(Variant(_5, 1), 2);
SetDiscriminant(_5, 1);
place!(Field::<i64>(Variant(_5, 1), 6)) = -Field::<i64>(Variant(_4, 1), 6);
place!(Field::<u8>(Variant(_4, 1), 4)) = !RET;
place!(Field::<u32>(Variant(_4, 1), 1)) = 3410569087_u32 & 2768931790_u32;
place!(Field::<i32>(Variant(_4, 1), 5)) = Field::<u32>(Variant(_4, 1), 1) as i32;
place!(Field::<u8>(Variant(_5, 1), 4)) = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<bool>(Variant(_5, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<f64>(Variant(_5, 1), 2)) = Field::<f64>(Variant(_4, 1), 2) - Field::<f64>(Variant(_4, 1), 2);
RET = Field::<u8>(Variant(_5, 1), 4) >> _2;
_1 = Field::<bool>(Variant(_5, 1), 0) ^ Field::<bool>(Variant(_4, 1), 0);
_2 = 7_usize as i32;
_1 = Field::<bool>(Variant(_4, 1), 0) | Field::<bool>(Variant(_5, 1), 0);
_2 = 4_usize as i32;
Call(_2 = core::intrinsics::transmute(Field::<i32>(Variant(_4, 1), 5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<i64>(Variant(_5, 1), 6)) = '\u{3e276}' as i64;
_4 = Adt18::Variant1 { fld0: _1,fld1: 442467191_u32,fld2: Field::<f64>(Variant(_5, 1), 2),fld3: (-33220516368762239331822487551148219338_i128),fld4: RET,fld5: _2,fld6: Field::<i64>(Variant(_5, 1), 6) };
place!(Field::<f64>(Variant(_4, 1), 2)) = Field::<f64>(Variant(_5, 1), 2);
_8 = &place!(Field::<i128>(Variant(_4, 1), 3));
_5 = Adt18::Variant1 { fld0: _1,fld1: 1304672403_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: 9646446979437609691191526399605855305_i128,fld4: Field::<u8>(Variant(_4, 1), 4),fld5: Field::<i32>(Variant(_4, 1), 5),fld6: Field::<i64>(Variant(_4, 1), 6) };
place!(Field::<u8>(Variant(_5, 1), 4)) = RET | RET;
place!(Field::<i64>(Variant(_4, 1), 6)) = !Field::<i64>(Variant(_5, 1), 6);
_2 = Field::<i32>(Variant(_4, 1), 5) ^ Field::<i32>(Variant(_4, 1), 5);
_5 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_4, 1), 0),fld1: 471889562_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: (-45107516295600294144593349163921851031_i128),fld4: Field::<u8>(Variant(_4, 1), 4),fld5: _2,fld6: Field::<i64>(Variant(_4, 1), 6) };
Call(place!(Field::<u32>(Variant(_5, 1), 1)) = fn17(Move(_8), Move(_3), Field::<bool>(Variant(_4, 1), 0), Field::<bool>(Variant(_4, 1), 0), _1, Field::<bool>(Variant(_4, 1), 0), Field::<bool>(Variant(_5, 1), 0)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<i64>(Variant(_4, 1), 6)) = !Field::<i64>(Variant(_5, 1), 6);
_5 = Adt18::Variant1 { fld0: _1,fld1: 3741978838_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: 48434895124443755580027068714482337746_i128,fld4: RET,fld5: _2,fld6: Field::<i64>(Variant(_4, 1), 6) };
place!(Field::<i128>(Variant(_4, 1), 3)) = (-3919287472327813911474830994093454950_i128);
_5 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_4, 1), 0),fld1: 1605041752_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: Field::<i128>(Variant(_4, 1), 3),fld4: Field::<u8>(Variant(_4, 1), 4),fld5: Field::<i32>(Variant(_4, 1), 5),fld6: Field::<i64>(Variant(_4, 1), 6) };
place!(Field::<bool>(Variant(_5, 1), 0)) = !Field::<bool>(Variant(_4, 1), 0);
place!(Field::<u8>(Variant(_4, 1), 4)) = !Field::<u8>(Variant(_5, 1), 4);
_1 = Field::<bool>(Variant(_5, 1), 0) >= Field::<bool>(Variant(_5, 1), 0);
_9 = _1 ^ _1;
place!(Field::<u32>(Variant(_5, 1), 1)) = !2649344360_u32;
_12 = 0_isize;
_15 = [Field::<i64>(Variant(_5, 1), 6),Field::<i64>(Variant(_4, 1), 6),Field::<i64>(Variant(_4, 1), 6),Field::<i64>(Variant(_4, 1), 6)];
SetDiscriminant(_5, 1);
RET = Field::<i64>(Variant(_4, 1), 6) as u8;
place!(Field::<i128>(Variant(_5, 1), 3)) = -Field::<i128>(Variant(_4, 1), 3);
match _12 {
0 => bb7,
_ => bb4
}
}
bb7 = {
place!(Field::<u8>(Variant(_4, 1), 4)) = RET * RET;
_13 = _12;
_8 = &place!(Field::<i128>(Variant(_5, 1), 3));
RET = Field::<u8>(Variant(_4, 1), 4) ^ Field::<u8>(Variant(_4, 1), 4);
place!(Field::<u8>(Variant(_5, 1), 4)) = RET >> Field::<i32>(Variant(_4, 1), 5);
place!(Field::<u8>(Variant(_4, 1), 4)) = Field::<u8>(Variant(_5, 1), 4);
_12 = !_13;
place!(Field::<u32>(Variant(_4, 1), 1)) = 354096732_u32;
_10 = _6 - _6;
_8 = &place!(Field::<i128>(Variant(_4, 1), 3));
SetDiscriminant(_4, 1);
place!(Field::<u8>(Variant(_4, 1), 4)) = !Field::<u8>(Variant(_5, 1), 4);
place!(Field::<i32>(Variant(_5, 1), 5)) = _2 - _2;
_11 = (-5501805808269243576_i64);
RET = !Field::<u8>(Variant(_4, 1), 4);
_11 = -1456544447562478155_i64;
match _13 {
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
0 => bb12,
_ => bb11
}
}
bb8 = {
RET = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<i64>(Variant(_4, 1), 6)) = !Field::<i64>(Variant(_5, 1), 6);
_5 = Adt18::Variant1 { fld0: _1,fld1: 3741978838_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: 48434895124443755580027068714482337746_i128,fld4: RET,fld5: _2,fld6: Field::<i64>(Variant(_4, 1), 6) };
place!(Field::<i128>(Variant(_4, 1), 3)) = (-3919287472327813911474830994093454950_i128);
_5 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_4, 1), 0),fld1: 1605041752_u32,fld2: Field::<f64>(Variant(_4, 1), 2),fld3: Field::<i128>(Variant(_4, 1), 3),fld4: Field::<u8>(Variant(_4, 1), 4),fld5: Field::<i32>(Variant(_4, 1), 5),fld6: Field::<i64>(Variant(_4, 1), 6) };
place!(Field::<bool>(Variant(_5, 1), 0)) = !Field::<bool>(Variant(_4, 1), 0);
place!(Field::<u8>(Variant(_4, 1), 4)) = !Field::<u8>(Variant(_5, 1), 4);
_1 = Field::<bool>(Variant(_5, 1), 0) >= Field::<bool>(Variant(_5, 1), 0);
_9 = _1 ^ _1;
place!(Field::<u32>(Variant(_5, 1), 1)) = !2649344360_u32;
_12 = 0_isize;
_15 = [Field::<i64>(Variant(_5, 1), 6),Field::<i64>(Variant(_4, 1), 6),Field::<i64>(Variant(_4, 1), 6),Field::<i64>(Variant(_4, 1), 6)];
SetDiscriminant(_5, 1);
RET = Field::<i64>(Variant(_4, 1), 6) as u8;
place!(Field::<i128>(Variant(_5, 1), 3)) = -Field::<i128>(Variant(_4, 1), 3);
match _12 {
0 => bb7,
_ => bb4
}
}
bb9 = {
Return()
}
bb10 = {
RET = 4_usize as u8;
place!(Field::<f32>(Variant(_5, 0), 4)) = 28020_u16 as f32;
_5 = _4;
place!(Field::<i64>(Variant(_5, 1), 6)) = Field::<i64>(Variant(_4, 1), 6);
place!(Field::<u8>(Variant(_5, 1), 4)) = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<u8>(Variant(_4, 1), 4)) = Field::<i128>(Variant(_5, 1), 3) as u8;
_6 = _2 as f32;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<f64>(Variant(_4, 1), 2) as u32;
place!(Field::<bool>(Variant(_5, 1), 0)) = _1 < _1;
place!(Field::<i128>(Variant(_5, 1), 3)) = Field::<i128>(Variant(_4, 1), 3);
_2 = Field::<i32>(Variant(_4, 1), 5) * Field::<i32>(Variant(_4, 1), 5);
place!(Field::<i128>(Variant(_5, 1), 3)) = -Field::<i128>(Variant(_4, 1), 3);
place!(Field::<i32>(Variant(_5, 1), 5)) = Field::<i32>(Variant(_4, 1), 5);
place!(Field::<f64>(Variant(_4, 1), 2)) = -Field::<f64>(Variant(_5, 1), 2);
SetDiscriminant(_5, 1);
place!(Field::<i64>(Variant(_5, 1), 6)) = -Field::<i64>(Variant(_4, 1), 6);
place!(Field::<u8>(Variant(_4, 1), 4)) = !RET;
place!(Field::<u32>(Variant(_4, 1), 1)) = 3410569087_u32 & 2768931790_u32;
place!(Field::<i32>(Variant(_4, 1), 5)) = Field::<u32>(Variant(_4, 1), 1) as i32;
place!(Field::<u8>(Variant(_5, 1), 4)) = Field::<u8>(Variant(_4, 1), 4);
place!(Field::<bool>(Variant(_5, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<f64>(Variant(_5, 1), 2)) = Field::<f64>(Variant(_4, 1), 2) - Field::<f64>(Variant(_4, 1), 2);
RET = Field::<u8>(Variant(_5, 1), 4) >> _2;
_1 = Field::<bool>(Variant(_5, 1), 0) ^ Field::<bool>(Variant(_4, 1), 0);
_2 = 7_usize as i32;
_1 = Field::<bool>(Variant(_4, 1), 0) | Field::<bool>(Variant(_5, 1), 0);
_2 = 4_usize as i32;
Call(_2 = core::intrinsics::transmute(Field::<i32>(Variant(_4, 1), 5)), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<u8>(Variant(_4, 1), 4)) = Field::<u8>(Variant(_5, 1), 4);
place!(Field::<i32>(Variant(_4, 1), 5)) = Field::<i32>(Variant(_5, 1), 5);
_21 = '\u{a7eda}';
_16 = _12 | _12;
place!(Field::<f64>(Variant(_5, 1), 2)) = 6285790362147005694_usize as f64;
_13 = _16;
_6 = _10 + _10;
place!(Field::<u32>(Variant(_4, 1), 1)) = 3276658773_u32;
_22 = [16115_u16,14581_u16,26096_u16,13707_u16,61940_u16];
_10 = _6 - _6;
_13 = 7368_u16 as isize;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_4, 1), 1);
place!(Field::<u32>(Variant(_4, 1), 1)) = Field::<u32>(Variant(_5, 1), 1) ^ Field::<u32>(Variant(_5, 1), 1);
_23 = &_11;
place!(Field::<i128>(Variant(_4, 1), 3)) = Field::<i128>(Variant(_5, 1), 3) & Field::<i128>(Variant(_5, 1), 3);
place!(Field::<u32>(Variant(_4, 1), 1)) = 242146958186256983_u64 as u32;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_4, 1), 1);
_21 = '\u{ca1b5}';
_9 = _1 != _1;
_8 = &place!(Field::<i128>(Variant(_4, 1), 3));
RET = !Field::<u8>(Variant(_4, 1), 4);
_21 = '\u{722dd}';
Goto(bb13)
}
bb13 = {
place!(Field::<u32>(Variant(_5, 1), 1)) = !Field::<u32>(Variant(_4, 1), 1);
_8 = &place!(Field::<i128>(Variant(_5, 1), 3));
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_4, 1), 1) ^ Field::<u32>(Variant(_4, 1), 1);
place!(Field::<i128>(Variant(_5, 1), 3)) = Field::<u32>(Variant(_4, 1), 1) as i128;
_13 = 51535_u16 as isize;
place!(Field::<bool>(Variant(_4, 1), 0)) = !_1;
_8 = &place!(Field::<i128>(Variant(_5, 1), 3));
_13 = _12;
place!(Field::<i128>(Variant(_4, 1), 3)) = 10540_i16 as i128;
_21 = '\u{93ee9}';
_17 = [(-23584_i16),8111_i16,(-13214_i16),(-17568_i16),6028_i16,(-15682_i16),(-27933_i16)];
_2 = -Field::<i32>(Variant(_5, 1), 5);
_12 = _16 | _16;
_2 = Field::<i32>(Variant(_4, 1), 5);
_18 = _12 | _12;
place!(Field::<i128>(Variant(_5, 1), 3)) = Field::<i128>(Variant(_4, 1), 3) >> _2;
_8 = &place!(Field::<i128>(Variant(_4, 1), 3));
place!(Field::<bool>(Variant(_5, 1), 0)) = !_9;
Goto(bb14)
}
bb14 = {
place!(Field::<f64>(Variant(_4, 1), 2)) = Field::<f64>(Variant(_5, 1), 2) - Field::<f64>(Variant(_5, 1), 2);
place!(Field::<bool>(Variant(_4, 1), 0)) = _1 == _1;
_12 = 5724323882540446546_u64 as isize;
_23 = &place!(Field::<i64>(Variant(_4, 1), 6));
_2 = Field::<i128>(Variant(_5, 1), 3) as i32;
_12 = 2478568067881618086_u64 as isize;
place!(Field::<i128>(Variant(_5, 1), 3)) = Field::<u32>(Variant(_4, 1), 1) as i128;
_5 = Adt18::Variant1 { fld0: _9,fld1: Field::<u32>(Variant(_4, 1), 1),fld2: Field::<f64>(Variant(_4, 1), 2),fld3: (*_8),fld4: RET,fld5: _2,fld6: _11 };
_29 = Field::<bool>(Variant(_5, 1), 0) >= Field::<bool>(Variant(_5, 1), 0);
place!(Field::<bool>(Variant(_4, 1), 0)) = !_29;
_29 = Field::<bool>(Variant(_5, 1), 0) < _1;
_2 = Field::<i32>(Variant(_5, 1), 5);
place!(Field::<u8>(Variant(_4, 1), 4)) = _21 as u8;
_30.fld2 = !RET;
_34 = (_30.fld2,);
place!(Field::<i128>(Variant(_5, 1), 3)) = (*_8);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(16_usize, 12_usize, Move(_12), 13_usize, Move(_13), 18_usize, Move(_18), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(16_usize, 15_usize, Move(_15), 16_usize, Move(_16), 2_usize, Move(_2), 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: &'static i128,mut _2: *const (bool, u16, [u32; 1], u16),mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> u32 {
mir! {
type RET = u32;
let _8: ([bool; 8],);
let _9: *mut i128;
let _10: usize;
let _11: isize;
let _12: isize;
let _13: &'static i128;
let _14: &'static [i16; 7];
let _15: bool;
let _16: *mut &'static (u8, isize);
let _17: *const char;
let _18: *mut &'static (u8, isize);
let _19: Adt44;
let _20: (&'static &'static (u8,), [i64; 4], ([usize; 3], ([i16; 6],)));
let _21: char;
let _22: [u8; 7];
let _23: &'static *mut i64;
let _24: bool;
let _25: u64;
let _26: bool;
let _27: *mut [i16; 6];
let _28: usize;
let _29: &'static &'static i128;
let _30: (u8, u8);
let _31: i8;
let _32: [u32; 1];
let _33: [char; 8];
let _34: Adt45;
let _35: f64;
let _36: &'static u128;
let _37: *mut i128;
let _38: ([i16; 6],);
let _39: Adt44;
let _40: ();
let _41: ();
{
_6 = _7;
_5 = _7 < _4;
_4 = !_7;
_7 = _4 < _6;
Goto(bb1)
}
bb1 = {
RET = 1692611110_u32 + 3700670295_u32;
_5 = _3 ^ _3;
_7 = _5;
_8.0 = [_6,_7,_3,_6,_7,_3,_5,_3];
_8.0 = [_7,_7,_3,_6,_7,_6,_5,_5];
RET = !3968080524_u32;
RET = (-343756428_i32) as u32;
_8.0 = [_6,_3,_3,_7,_7,_7,_7,_5];
RET = 17756_u16 as u32;
_8.0 = [_4,_3,_3,_7,_3,_5,_3,_7];
_10 = 13146723193596712523_usize;
_8.0 = [_5,_6,_4,_7,_5,_3,_7,_7];
_4 = _3;
_7 = !_5;
_11 = 9223372036854775807_isize - (-99_isize);
_11 = 15660933883525202750667823718840608659_u128 as isize;
RET = !2993122191_u32;
_6 = !_5;
_12 = 218_u8 as isize;
_5 = _7;
Goto(bb2)
}
bb2 = {
_4 = _5 < _5;
_6 = _5 == _4;
_8.0 = [_5,_7,_6,_7,_3,_6,_6,_7];
_8.0 = [_4,_7,_4,_6,_5,_6,_6,_6];
RET = 102237296_u32 * 397792689_u32;
_11 = _12 << RET;
_4 = _6 & _5;
_10 = 7_usize;
RET = !2168982758_u32;
RET = 1368900214_u32;
_5 = _3;
_12 = _11 ^ _11;
_8.0[_10] = !_4;
_7 = _4;
_11 = _12;
_7 = !_8.0[_10];
_12 = (-17235_i16) as isize;
_7 = _6;
RET = 1174599178_u32 << _11;
Call(RET = fn18(_4, _8.0[_10], _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = _6 <= _7;
RET = 3754495542_u32 + 1840024613_u32;
RET = 3626811331_u32 - 3410343466_u32;
_11 = _12 | _12;
_4 = !_6;
_3 = _6 >= _4;
_15 = _3 & _4;
_15 = _3;
_10 = 1_usize - 4169058768626639600_usize;
_3 = _4;
_7 = _15 == _3;
_7 = _5;
_20.1 = [1627268670409883035_i64,3430129451859294494_i64,(-6964667895518460995_i64),(-7174249467694689897_i64)];
Goto(bb4)
}
bb4 = {
RET = 2372602918_u32;
RET = 426163767_u32 << _10;
_15 = _4;
_15 = _7;
_11 = _12;
RET = !585950734_u32;
_10 = _5 as usize;
_20.2.0 = [_10,_10,_10];
Call(_10 = core::intrinsics::transmute(_8.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = !_4;
_11 = _12 | _12;
_17 = core::ptr::addr_of!(_21);
(*_17) = '\u{602f3}';
_4 = !_3;
_5 = !_15;
_7 = _4;
_7 = _15 < _4;
_24 = _7;
_17 = core::ptr::addr_of!(_21);
_15 = _24 < _5;
_5 = _6;
_12 = 171321317567038910933883310563927450092_u128 as isize;
_7 = _10 == _10;
_17 = core::ptr::addr_of!((*_17));
_15 = _24;
_8.0 = [_4,_5,_6,_3,_6,_7,_5,_6];
_17 = core::ptr::addr_of!(_21);
Goto(bb6)
}
bb6 = {
_5 = _15;
(*_17) = '\u{10b0c2}';
_20.2.0 = [_10,_10,_10];
Goto(bb7)
}
bb7 = {
_20.1 = [4996599869823304680_i64,(-5493447058409028266_i64),8202992354871492331_i64,(-5005431022030367061_i64)];
Goto(bb8)
}
bb8 = {
_5 = _15;
_20.2.1.0 = [22930_i16,19402_i16,15701_i16,(-18879_i16),3471_i16,(-2585_i16)];
_30.1 = _10 as u8;
_8.0 = [_4,_3,_24,_5,_5,_7,_7,_3];
_22 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_15 = _30.1 >= _30.1;
_20.1 = [2848590485262484320_i64,(-8814068882310907994_i64),9176182474491210347_i64,(-2728946758052126322_i64)];
_4 = !_7;
RET = 3479306085_u32 ^ 919408675_u32;
RET = 834906371_u32;
_26 = _5;
_28 = (-98241537445418504972949302994017657197_i128) as usize;
_22 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_30.0 = 21474_i16 as u8;
_8.0 = [_24,_3,_4,_15,_7,_3,_6,_24];
_7 = _6 < _26;
match RET {
0 => bb9,
1 => bb10,
2 => bb11,
834906371 => bb13,
_ => bb12
}
}
bb9 = {
_20.1 = [4996599869823304680_i64,(-5493447058409028266_i64),8202992354871492331_i64,(-5005431022030367061_i64)];
Goto(bb8)
}
bb10 = {
_5 = _6 <= _7;
RET = 3754495542_u32 + 1840024613_u32;
RET = 3626811331_u32 - 3410343466_u32;
_11 = _12 | _12;
_4 = !_6;
_3 = _6 >= _4;
_15 = _3 & _4;
_15 = _3;
_10 = 1_usize - 4169058768626639600_usize;
_3 = _4;
_7 = _15 == _3;
_7 = _5;
_20.1 = [1627268670409883035_i64,3430129451859294494_i64,(-6964667895518460995_i64),(-7174249467694689897_i64)];
Goto(bb4)
}
bb11 = {
_4 = _5 < _5;
_6 = _5 == _4;
_8.0 = [_5,_7,_6,_7,_3,_6,_6,_7];
_8.0 = [_4,_7,_4,_6,_5,_6,_6,_6];
RET = 102237296_u32 * 397792689_u32;
_11 = _12 << RET;
_4 = _6 & _5;
_10 = 7_usize;
RET = !2168982758_u32;
RET = 1368900214_u32;
_5 = _3;
_12 = _11 ^ _11;
_8.0[_10] = !_4;
_7 = _4;
_11 = _12;
_7 = !_8.0[_10];
_12 = (-17235_i16) as isize;
_7 = _6;
RET = 1174599178_u32 << _11;
Call(RET = fn18(_4, _8.0[_10], _4), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
RET = 1692611110_u32 + 3700670295_u32;
_5 = _3 ^ _3;
_7 = _5;
_8.0 = [_6,_7,_3,_6,_7,_3,_5,_3];
_8.0 = [_7,_7,_3,_6,_7,_6,_5,_5];
RET = !3968080524_u32;
RET = (-343756428_i32) as u32;
_8.0 = [_6,_3,_3,_7,_7,_7,_7,_5];
RET = 17756_u16 as u32;
_8.0 = [_4,_3,_3,_7,_3,_5,_3,_7];
_10 = 13146723193596712523_usize;
_8.0 = [_5,_6,_4,_7,_5,_3,_7,_7];
_4 = _3;
_7 = !_5;
_11 = 9223372036854775807_isize - (-99_isize);
_11 = 15660933883525202750667823718840608659_u128 as isize;
RET = !2993122191_u32;
_6 = !_5;
_12 = 218_u8 as isize;
_5 = _7;
Goto(bb2)
}
bb13 = {
_30.1 = _30.0 & _30.0;
_10 = !_28;
_5 = _6;
_17 = core::ptr::addr_of!((*_17));
_11 = _12;
_30.0 = !_30.1;
_7 = !_15;
RET = 3705453233_u32 | 418273590_u32;
_19 = Adt44::Variant1 { fld0: _24 };
(*_17) = '\u{13be5}';
_24 = _15;
_34.fld3 = (-53_i8);
_30.1 = _21 as u8;
_31 = _34.fld3 ^ _34.fld3;
Goto(bb14)
}
bb14 = {
_3 = !_26;
_34.fld4 = 47979_u16 + 12740_u16;
_34.fld0 = [(-156759031_i32),1498336517_i32,722358843_i32,1951081094_i32];
_34.fld3 = _31;
_21 = '\u{fffa6}';
_34.fld1.0 = !_30.1;
_21 = '\u{62b36}';
_10 = !_28;
_34.fld1 = (_30.0, _12);
_27 = core::ptr::addr_of_mut!(_20.2.1.0);
_20.2.1.0 = [24008_i16,8533_i16,(-3974_i16),(-1601_i16),18036_i16,265_i16];
_34.fld6 = core::ptr::addr_of!(_34.fld3);
_32 = [RET];
_21 = '\u{c3f79}';
_17 = core::ptr::addr_of!(_21);
_30.1 = _34.fld1.0 | _34.fld1.0;
_28 = !_10;
_21 = '\u{c04c}';
(*_17) = '\u{d2749}';
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(17_usize, 11_usize, Move(_11), 24_usize, Move(_24), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(17_usize, 5_usize, Move(_5), 26_usize, Move(_26), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(17_usize, 3_usize, Move(_3), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: bool,mut _3: bool) -> u32 {
mir! {
type RET = u32;
let _4: char;
let _5: isize;
let _6: char;
let _7: isize;
let _8: [i16; 1];
let _9: ((u8,), (usize, f32, Adt18, &'static usize), [i64; 4], (&'static i64,));
let _10: (usize, f32, Adt18, &'static usize);
let _11: bool;
let _12: i128;
let _13: (&'static &'static (u8,), [i64; 4], ([usize; 3], ([i16; 6],)));
let _14: *mut &'static (u8, isize);
let _15: [i64; 4];
let _16: isize;
let _17: &'static usize;
let _18: i128;
let _19: [i32; 4];
let _20: Adt18;
let _21: *const Adt18;
let _22: f32;
let _23: isize;
let _24: i32;
let _25: u8;
let _26: i16;
let _27: bool;
let _28: f64;
let _29: Adt18;
let _30: bool;
let _31: isize;
let _32: Adt75;
let _33: [u16; 5];
let _34: *const (bool, u16, [u32; 1], u16);
let _35: &'static usize;
let _36: ();
let _37: ();
{
RET = !2365544875_u32;
_1 = _3 >= _3;
RET = 237524462718563869800327515648510145892_u128 as u32;
RET = 1128486234_u32;
_1 = !_3;
_3 = _2 & _1;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
1128486234 => bb5,
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
_1 = !_3;
_2 = _3;
RET = !2319953869_u32;
_4 = '\u{a6c0b}';
_5 = (-122306840067411935519084785181061744587_i128) as isize;
_3 = !_1;
RET = 308509081827467375729030529718213621214_u128 as u32;
_6 = _4;
_5 = !9223372036854775807_isize;
RET = 729978374_u32;
_1 = !_2;
_6 = _4;
_3 = _2 | _1;
RET = 4126937463_u32 & 2035489321_u32;
Goto(bb6)
}
bb6 = {
_4 = _6;
_6 = _4;
_3 = _1;
_1 = _2 > _2;
_9.2 = [8570517551185190176_i64,(-3216626541798243672_i64),(-549151241882325701_i64),6959148033303223576_i64];
_9.1.1 = (-6833490256773395938_i64) as f32;
_3 = _1;
RET = 3613794901_u32 - 219654379_u32;
_9.0 = (255_u8,);
_10.3 = &_9.1.0;
_9.1.0 = 8114780597486515819_usize;
_10.1 = -_9.1.1;
_9.1.0 = !0_usize;
_9.0 = (1_u8,);
Call(_10.0 = core::intrinsics::bswap(_9.1.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = _4;
_8 = [(-18403_i16)];
RET = !3123716351_u32;
_7 = -_5;
_11 = _1 == _1;
_9.1.3 = &_10.0;
RET = 3981582843_u32;
_6 = _4;
_9.1.1 = -_10.1;
_9.0 = (151_u8,);
RET = 4147652745_u32;
_10.3 = &_9.1.0;
_11 = !_2;
_2 = _3;
_13.2.0 = [_9.1.0,_9.1.0,_9.1.0];
_13.1 = _9.2;
_12 = !(-110573990421629783806218672922025771193_i128);
_9.2 = [(-4925505395042224702_i64),(-7193131377655227849_i64),(-6964140744084823581_i64),1304885204191032439_i64];
_9.1.1 = _10.1 - _10.1;
_9.1.0 = 3_usize << _5;
match RET {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4147652745 => bb8,
_ => bb5
}
}
bb8 = {
_9.0.0 = (-30883_i16) as u8;
_9.2 = [1795628819869780681_i64,1960454675680346502_i64,1469276785792266036_i64,(-4026076513324147648_i64)];
_1 = _2 <= _2;
_10.0 = !_9.1.0;
_9.2 = [4772468599830598311_i64,(-3487304870583117164_i64),4731635731951846875_i64,(-4825343223918915912_i64)];
_10.3 = &_9.1.0;
_9.0 = (189_u8,);
_13.2.0 = [_9.1.0,_10.0,_9.1.0];
_11 = _3 | _3;
_1 = _2;
_10.0 = _9.1.0;
RET = 2671422697_u32;
_13.2.0 = [_9.1.0,_9.1.0,_9.1.0];
_9.1.3 = &_10.0;
_4 = _6;
_9.1.0 = 219518792666501306100488676628173805672_u128 as usize;
_9.0 = (188_u8,);
Goto(bb9)
}
bb9 = {
_5 = _4 as isize;
_10.0 = !_9.1.0;
_9.1.3 = &_9.1.0;
_2 = _11;
_10.1 = -_9.1.1;
_9.0.0 = 144_u8 << _12;
_13.2.1.0 = [12152_i16,13883_i16,3829_i16,(-9414_i16),(-4450_i16),32264_i16];
_13.1 = _9.2;
_4 = _6;
_13.2.0 = [_9.1.0,_9.1.0,_10.0];
_5 = _7;
_13.2.1.0 = [8112_i16,(-3737_i16),19168_i16,(-23447_i16),(-22119_i16),21842_i16];
_10.1 = -_9.1.1;
_13.2.0 = [_10.0,_9.1.0,_9.1.0];
_12 = !(-14463430235105080743863343050885529954_i128);
RET = 2160200492_u32 >> _10.0;
_6 = _4;
_11 = !_3;
_12 = -(-22060468021027895616359083874353363825_i128);
_7 = _5 | _5;
_11 = _1 & _2;
_9.2 = [2317500958721951462_i64,(-1404372289332371397_i64),840090781332725527_i64,1357201624953972283_i64];
_13.2.0 = [_10.0,_9.1.0,_9.1.0];
RET = 4090314591_u32;
_4 = _6;
_10.0 = _9.1.0;
Goto(bb10)
}
bb10 = {
_9.0.0 = 5_u8 | 197_u8;
_1 = !_2;
_6 = _4;
_9.0.0 = 91_u8 * 88_u8;
_9.0.0 = 106_u8;
_16 = _7;
_13.2.1.0 = [(-30405_i16),(-5024_i16),12804_i16,(-17928_i16),(-19812_i16),(-9415_i16)];
_18 = _12 & _12;
_13.2.0 = [_10.0,_10.0,_10.0];
_13.2.1.0 = [18118_i16,25679_i16,(-22420_i16),30710_i16,29440_i16,(-9428_i16)];
_3 = !_11;
_4 = _6;
_10.3 = &_9.1.0;
_3 = _11 | _2;
_13.2.0 = [_9.1.0,_9.1.0,_10.0];
_10.0 = _9.1.0;
_5 = _7 << _16;
_6 = _4;
_13.2.1.0 = [7551_i16,(-8099_i16),13023_i16,5524_i16,(-23336_i16),(-24900_i16)];
_4 = _6;
_2 = _11;
_13.2.1.0 = [(-30320_i16),6032_i16,(-24486_i16),(-8985_i16),(-14705_i16),9867_i16];
_9.0 = (173_u8,);
RET = 2403381344_u32;
_15 = _13.1;
_6 = _4;
_9.2 = [(-841428443529861585_i64),(-7509664470237862598_i64),(-2512864259568722396_i64),8334287210958034877_i64];
Goto(bb11)
}
bb11 = {
_9.0.0 = !5_u8;
_13.1 = _9.2;
_8 = [12027_i16];
_9.0 = (173_u8,);
_1 = !_2;
_17 = &_9.1.0;
_26 = (-13057_i16);
_24 = !(-299220533_i32);
_13.2.1.0 = [_26,_26,_26,_26,_26,_26];
_2 = _1 & _11;
_21 = core::ptr::addr_of!(_10.2);
_12 = -_18;
_15 = _13.1;
_28 = _26 as f64;
_9.1.3 = &(*_17);
_10.3 = &_10.0;
RET = 1729494525_u32 | 2270841797_u32;
_9.1.0 = !_10.0;
_4 = _6;
_13.2.0 = [_9.1.0,_9.1.0,_10.0];
_17 = &_10.0;
_26 = _1 as i16;
Goto(bb12)
}
bb12 = {
_9.2 = [(-2702666910952392817_i64),4670432227491538589_i64,(-1678126835870195274_i64),(-2375632702641495651_i64)];
_18 = !_12;
_9.0 = (145_u8,);
_10.1 = -_9.1.1;
_9.1.1 = 13476338484775477304_u64 as f32;
_23 = _5;
_6 = _4;
_1 = !_3;
_10.3 = Move(_17);
_16 = -_23;
_30 = !_3;
_27 = _3;
_22 = _9.1.1 - _10.1;
(*_21) = Adt18::Variant1 { fld0: _1,fld1: RET,fld2: _28,fld3: _12,fld4: _9.0.0,fld5: _24,fld6: (-7527101171090206635_i64) };
_9.1.2 = Adt18::Variant1 { fld0: _11,fld1: Field::<u32>(Variant((*_21), 1), 1),fld2: Field::<f64>(Variant(_10.2, 1), 2),fld3: _18,fld4: _9.0.0,fld5: Field::<i32>(Variant((*_21), 1), 5),fld6: 8647913769907952273_i64 };
match _9.0.0 {
145 => bb13,
_ => bb7
}
}
bb13 = {
place!(Field::<i64>(Variant(_10.2, 1), 6)) = (-1301098227225101902_i64) & (-8659938677304318374_i64);
place!(Field::<i64>(Variant(_9.1.2, 1), 6)) = Field::<u8>(Variant((*_21), 1), 4) as i64;
_31 = 106798469516330987736501227955237393639_u128 as isize;
_6 = _4;
_7 = Field::<u32>(Variant(_10.2, 1), 1) as isize;
place!(Field::<bool>(Variant(_9.1.2, 1), 0)) = !Field::<bool>(Variant((*_21), 1), 0);
Goto(bb14)
}
bb14 = {
place!(Field::<i64>(Variant((*_21), 1), 6)) = !Field::<i64>(Variant(_9.1.2, 1), 6);
(*_21) = _9.1.2;
RET = Field::<u32>(Variant((*_21), 1), 1);
_30 = _27;
_21 = core::ptr::addr_of!(_20);
(*_21) = _9.1.2;
_26 = Field::<i32>(Variant(_20, 1), 5) as i16;
place!(Field::<bool>(Variant(_9.1.2, 1), 0)) = !_11;
place!(Field::<bool>(Variant((*_21), 1), 0)) = !_30;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(18_usize, 3_usize, Move(_3), 4_usize, Move(_4), 7_usize, Move(_7), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(18_usize, 2_usize, Move(_2), 6_usize, Move(_6), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(18_usize, 18_usize, Move(_18), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(79_u8), std::hint::black_box((-108_isize)), std::hint::black_box(94_i8), std::hint::black_box(22202466539127348122305837339812106320_i128), std::hint::black_box((-406736964_i32)));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: (u8, isize),
fld1: char,
fld2: i64,
fld3: i8,
fld4: f32,

},
Variant1{
fld0: bool,
fld1: u32,
fld2: f64,
fld3: i128,
fld4: u8,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: u16,
fld1: char,
fld2: u8,
fld3: (u8, isize),
fld4: [u32; 1],
fld5: [i16; 7],
fld6: (u8, u8),
fld7: u64,
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: ([i16; 6],),
fld1: u128,
fld2: (u8, u8),
fld3: [i16; 6],
fld4: [i32; 4],
fld5: i32,
fld6: [u8; 4],
fld7: u16,

},
Variant1{
fld0: bool,

},
Variant2{
fld0: i16,
fld1: i64,
fld2: *const u128,

},
Variant3{
fld0: f64,
fld1: u8,
fld2: isize,
fld3: u32,
fld4: usize,
fld5: u64,
fld6: i64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: [i32; 4],
fld1: (u8, isize),
fld2: u8,
fld3: i8,
fld4: u16,
fld5: Adt18,
fld6: *const i8,
fld7: Adt44,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const u128,
fld1: Adt18,
fld2: u128,
fld3: i8,
fld4: i128,
fld5: Adt44,
fld6: *const i8,

},
Variant1{
fld0: (bool, u16, [u32; 1], u16),

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: bool,
fld1: [i64; 4],
fld2: (Adt18,),
fld3: Adt44,
fld4: *mut i128,
fld5: (u8, isize),

},
Variant1{
fld0: [u32; 1],
fld1: [bool; 8],
fld2: [i64; 4],
fld3: [usize; 3],
fld4: u32,
fld5: Adt44,
fld6: Adt48,
fld7: [isize; 1],

},
Variant2{
fld0: [i32; 4],
fld1: [i64; 4],

},
Variant3{
fld0: f64,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: [i64; 4],
fld1: (u8,),

},
Variant1{
fld0: *mut i64,
fld1: *const i8,
fld2: u32,
fld3: i8,
fld4: Adt48,
fld5: Adt18,
fld6: [char; 8],

},
Variant2{
fld0: Adt44,
fld1: Adt60,
fld2: *const [u32; 1],
fld3: [u8; 4],
fld4: [i32; 4],
fld5: ([bool; 8],),

},
Variant3{
fld0: u32,

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: ([i16; 6],),
fld1: Adt44,
fld2: (Adt33, i8, Adt48),
fld3: i8,
fld4: *mut [i16; 6],
fld5: i32,
fld6: (u8, isize),
fld7: usize,

},
Variant1{
fld0: (Adt33, i8, Adt48),
fld1: *mut i64,
fld2: Adt45,
fld3: (bool, u16, [u32; 1], u16),
fld4: f32,
fld5: [i16; 6],

},
Variant2{
fld0: *const (bool, u16, [u32; 1], u16),
fld1: [i16; 6],
fld2: [u32; 1],
fld3: u128,
fld4: i16,
fld5: [i32; 4],
fld6: Adt48,

},
Variant3{
fld0: Adt45,

}}

