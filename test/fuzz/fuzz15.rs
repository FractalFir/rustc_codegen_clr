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
pub fn fn0(mut _1: u16,mut _2: i128,mut _3: isize) -> bool {
mir! {
type RET = bool;
let _4: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _5: (f64, isize);
let _6: [i32; 3];
let _7: isize;
let _8: i64;
let _9: [u128; 6];
let _10: f64;
let _11: u16;
let _12: [i16; 3];
let _13: bool;
let _14: Adt46;
let _15: Adt47;
let _16: bool;
let _17: Adt46;
let _18: i32;
let _19: (f64, isize);
let _20: [isize; 6];
let _21: u128;
let _22: u16;
let _23: u32;
let _24: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _25: [u128; 6];
let _26: isize;
let _27: i16;
let _28: isize;
let _29: ();
let _30: ();
{
_2 = 2249841253_u32 as i128;
_2 = (-47029966822739217746451208906605960316_i128);
_2 = 81890633718407516502416316604296846603_i128 >> 1268781817_i32;
RET = _2 >= _2;
Call(_4.1 = fn1(_2, RET, _2, RET, RET, RET, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !false;
_4.2.1 = !9223372036854775807_isize;
_4.6.1 = -14408_i16;
_2 = (-27136735503784554293900971405175486081_i128);
_4.6.3 = 17470651299311921583_usize as f64;
_4.6.2 = '\u{cd1ac}' as i128;
_4.1 = 314834280654911466391250188838010446688_u128;
_4.2.0 = _4.6.3 * _4.6.3;
_4.2.1 = RET as isize;
_4.6 = (16887228562642489380_u64, 4069_i16, _2, _4.2.0, _2);
_3 = 2089113616_u32 as isize;
_5 = (_4.6.3, _4.2.1);
_6 = [(-944620100_i32),184772384_i32,1363622016_i32];
_4.6.4 = '\u{4bb2e}' as i128;
_4.6.4 = _4.6.2 * _4.6.2;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
RET = !true;
_1 = 38375_u16;
_4.6.0 = 2587967132006971109_u64;
RET = !true;
_5.1 = _3 ^ _4.2.1;
Call(_4.6.2 = core::intrinsics::transmute(_4.6.4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.1 = !64918143070773977782573868422369256508_u128;
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.4.0 = [_5.1,_4.2.1,_5.1,_4.2.1,_5.1,_4.2.1];
_4.2.0 = -_5.0;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.1 = 228061894220931389767650250743404844612_u128 - 140587601183748137463623536326618583609_u128;
_4.0 = [_5.1,_5.1];
_2 = _4.6.2 * _4.6.4;
_4.0 = [_4.2.1,_4.2.1];
_4.6.2 = -_4.6.4;
_5.1 = _4.2.1;
_4.2.0 = _4.6.3;
_10 = _5.0;
_5.1 = _3 & _4.2.1;
_4.6.3 = _4.2.0;
Call(_4.4 = fn2(_4.6.4, _4.6.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4.1 = 334639121540941479908359890830710955238_u128;
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_8 = 5332617567434766970_i64;
_4.2 = (_5.0, _5.1);
_8 = 3879428694570413005_i64 << _4.6.4;
_12 = [_4.6.1,_4.6.1,_4.6.1];
_7 = '\u{d110d}' as isize;
_4.1 = 62568333266119809668815902309356818770_u128 * 179753845678096626521461928936582770768_u128;
_1 = 12638_u16 & 8062_u16;
RET = _10 == _10;
RET = !false;
_13 = !RET;
_4.0 = [_7,_4.2.1];
_5 = (_4.2.0, _3);
_4.6.3 = _4.2.0;
_9 = _4.3;
_9 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.6.1 = 24810_i16;
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.6 = (17953753147180708597_u64, 25196_i16, _2, _4.2.0, _2);
_4.6 = (1340589820629111255_u64, 10384_i16, _2, _10, _2);
match _4.6.0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
1340589820629111255 => bb10,
_ => bb9
}
}
bb4 = {
_4.1 = !64918143070773977782573868422369256508_u128;
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.4.0 = [_5.1,_4.2.1,_5.1,_4.2.1,_5.1,_4.2.1];
_4.2.0 = -_5.0;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.1 = 228061894220931389767650250743404844612_u128 - 140587601183748137463623536326618583609_u128;
_4.0 = [_5.1,_5.1];
_2 = _4.6.2 * _4.6.4;
_4.0 = [_4.2.1,_4.2.1];
_4.6.2 = -_4.6.4;
_5.1 = _4.2.1;
_4.2.0 = _4.6.3;
_10 = _5.0;
_5.1 = _3 & _4.2.1;
_4.6.3 = _4.2.0;
Call(_4.4 = fn2(_4.6.4, _4.6.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = !false;
_4.2.1 = !9223372036854775807_isize;
_4.6.1 = -14408_i16;
_2 = (-27136735503784554293900971405175486081_i128);
_4.6.3 = 17470651299311921583_usize as f64;
_4.6.2 = '\u{cd1ac}' as i128;
_4.1 = 314834280654911466391250188838010446688_u128;
_4.2.0 = _4.6.3 * _4.6.3;
_4.2.1 = RET as isize;
_4.6 = (16887228562642489380_u64, 4069_i16, _2, _4.2.0, _2);
_3 = 2089113616_u32 as isize;
_5 = (_4.6.3, _4.2.1);
_6 = [(-944620100_i32),184772384_i32,1363622016_i32];
_4.6.4 = '\u{4bb2e}' as i128;
_4.6.4 = _4.6.2 * _4.6.2;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
RET = !true;
_1 = 38375_u16;
_4.6.0 = 2587967132006971109_u64;
RET = !true;
_5.1 = _3 ^ _4.2.1;
Call(_4.6.2 = core::intrinsics::transmute(_4.6.4), ReturnTo(bb2), UnwindUnreachable())
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
_3 = _4.2.1;
RET = _13;
_2 = _4.6.4 - _4.6.2;
_11 = !_1;
_1 = '\u{ee2e}' as u16;
_13 = !RET;
_9 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_7 = _3 >> _4.6.1;
_3 = !_4.2.1;
_8 = (-1087503826941127631_i64);
Goto(bb11)
}
bb11 = {
_4.6.2 = _2 * _2;
_4.2.0 = _5.0 + _5.0;
_10 = _4.6.3;
_4.6.3 = _4.2.0;
_15 = Adt47::Variant1 { fld0: 59_u8 };
_4.6.4 = !_2;
_5.1 = _7;
_16 = _13;
_5.0 = 15156477567755424938_usize as f64;
_13 = !_16;
_4.6.0 = 18081437029922139362_u64;
_4.6 = (2946947264321178755_u64, (-15430_i16), _2, _4.2.0, _2);
_4.2 = _5;
_4.6.0 = _4.1 as u64;
_4.0 = [_3,_4.2.1];
_18 = 557233921_i32;
_4.1 = 270584132905241966547644144685727382633_u128;
_8 = !8674725204502577430_i64;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
Goto(bb12)
}
bb12 = {
_4.3 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.2.1 = _18 as isize;
_4.1 = 57530608891549658155763316438727169818_u128;
_4.6.1 = !(-11747_i16);
_4.6.2 = -_2;
_13 = _16 & RET;
_5 = _4.2;
_18 = 1554324915_i32;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.2 = (_5.0, _7);
_8 = _1 as i64;
_20 = [_4.2.1,_3,_4.2.1,_5.1,_7,_7];
_21 = _11 as u128;
_4.2 = _5;
Goto(bb13)
}
bb13 = {
_4.3 = [_21,_4.1,_21,_21,_21,_4.1];
_4.6.2 = _2;
_4.6.2 = 1943557283_u32 as i128;
_4.6.4 = _21 as i128;
_19.1 = _4.2.1;
_4.6 = (17659483073790835867_u64, (-8419_i16), _2, _5.0, _2);
place!(Field::<u8>(Variant(_15, 1), 0)) = 3_u8;
_19 = _4.2;
RET = _16;
_4.6.4 = _2;
_1 = 467557201_u32 as u16;
_16 = RET;
_4.4 = (_20,);
_4.6.2 = -_4.6.4;
_4.4 = (_20,);
SetDiscriminant(_15, 2);
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_22 = _11;
_3 = _7;
_5.0 = 236_u8 as f64;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.1 = !_21;
_24.3 = [_21,_4.1,_21,_21,_21,_4.1];
_24.2 = (_4.2.0, _3);
Goto(bb14)
}
bb14 = {
_1 = _22 & _11;
_24.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.5 = _24.5;
_24.2.1 = _3;
_12 = [_4.6.1,_4.6.1,_4.6.1];
_3 = !_24.2.1;
_26 = '\u{2e24f}' as isize;
_24.6 = (_4.6.0, _4.6.1, _2, _4.2.0, _2);
_24.2.0 = _4.6.1 as f64;
_24.1 = _21;
_4.0 = [_24.2.1,_24.2.1];
_24.6.3 = 607149518_u32 as f64;
_24.6 = (_4.6.0, _4.6.1, _2, _24.2.0, _4.6.2);
_7 = _24.2.1 * _5.1;
_4.6.2 = -_24.6.4;
_4.6.0 = !_24.6.0;
_8 = (-5498336261958290054_i64) + 6248596469152471241_i64;
_3 = !_24.2.1;
_1 = _22 << _22;
_4.4 = (_20,);
_1 = _11 >> _24.6.2;
_24.1 = _4.1 ^ _4.1;
_24.6.1 = !_4.6.1;
_4.3 = [_21,_21,_4.1,_21,_4.1,_24.1];
_24.4 = (_20,);
place!(Field::<Adt42>(Variant(_15, 2), 1)) = Adt42::Variant1 { fld0: _12,fld1: _4.6.4,fld2: _4.0 };
_4.6.4 = _24.6.2;
_13 = !_16;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(0_usize, 7_usize, Move(_7), 21_usize, Move(_21), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(0_usize, 12_usize, Move(_12), 16_usize, Move(_16), 11_usize, Move(_11), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i128,mut _2: bool,mut _3: i128,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: i128,mut _8: i128) -> u128 {
mir! {
type RET = u128;
let _9: [usize; 8];
let _10: (i16, u128, [u128; 6], (i8,), u8);
let _11: Adt44;
let _12: [isize; 2];
let _13: u8;
let _14: [u128; 6];
let _15: isize;
let _16: Adt45;
let _17: bool;
let _18: f32;
let _19: [u128; 6];
let _20: i128;
let _21: [i16; 3];
let _22: i64;
let _23: char;
let _24: Adt53;
let _25: i64;
let _26: Adt42;
let _27: bool;
let _28: ();
let _29: ();
{
_7 = (-66_i8) as i128;
RET = 283167770145029171893053306178712014077_u128;
_8 = _3;
_7 = _1;
RET = !144794032497014611726180283287402914821_u128;
RET = 246412645886357347762283608996961794810_u128 - 206970391339788976688872109853963878716_u128;
_3 = -_8;
_8 = _1 << _1;
_8 = '\u{6d958}' as i128;
_1 = _3 ^ _7;
_5 = _2 <= _4;
_4 = _5;
RET = !110309908482263283549287688201017093877_u128;
_3 = -_1;
_7 = _1 | _8;
_10.2 = [RET,RET,RET,RET,RET,RET];
_10.0 = 7359_i16 << _3;
_10.3 = (32_i8,);
_10.2 = [RET,RET,RET,RET,RET,RET];
RET = 101895055795686413250624300696701889941_u128 >> _10.0;
Goto(bb1)
}
bb1 = {
RET = 131773150655569202384848626437619056175_u128;
_9 = [2_usize,280791977953460872_usize,1_usize,5_usize,5433670125027721446_usize,6746013179094060956_usize,14603697211313570272_usize,5722927549611237726_usize];
_12 = [33_isize,9223372036854775807_isize];
_4 = !_2;
_10.0 = (-24160_i16) - 56_i16;
_2 = !_5;
_10.3 = ((-87_i8),);
_10.3 = ((-24_i8),);
_10.4 = 153_u8;
_1 = -_3;
_4 = !_6;
_9 = [5728849478350148351_usize,9784001000338890565_usize,15455080831191262113_usize,3085001189393436708_usize,4454257351970056078_usize,2_usize,13324665660693654475_usize,69131045292937886_usize];
_10.1 = !RET;
_10.4 = !246_u8;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10.3.0 = -73_i8;
_9 = [1_usize,13935530550515506833_usize,4_usize,5_usize,7_usize,7332155875605425033_usize,13775876071528856488_usize,6_usize];
_2 = !_5;
_10.0 = !(-15313_i16);
_7 = _1 << _3;
_10.0 = !25449_i16;
_3 = !_1;
_10.1 = RET;
_6 = _2;
_10.2 = [RET,RET,RET,RET,RET,RET];
_3 = '\u{41f7e}' as i128;
_9 = [5_usize,2_usize,5372823564959951004_usize,4216907757583861317_usize,4948967335574837781_usize,16414386443259316791_usize,3734423458163220338_usize,13255673964357928920_usize];
Call(_14 = core::intrinsics::transmute(_10.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = -52_isize;
RET = _10.1 >> _7;
_6 = _2 | _5;
_6 = _2;
_3 = _5 as i128;
RET = _10.3.0 as u128;
_13 = _10.4 >> _7;
RET = !_10.1;
_9 = [16556277112875207857_usize,6974863595818090967_usize,9819036127636259644_usize,2_usize,6437228603834868954_usize,795451627099803901_usize,7_usize,7525887264080456924_usize];
_10.1 = RET;
_5 = _2;
RET = _10.1 ^ _10.1;
_10.1 = _7 as u128;
_10.3.0 = (-100_i8);
_10.3 = ((-12_i8),);
_2 = _5;
_4 = _10.3.0 != _10.3.0;
_3 = _7 * _8;
_10.3.0 = (-53_i8) >> _7;
_6 = _2 ^ _2;
Goto(bb3)
}
bb3 = {
_9 = [469532573686840491_usize,3_usize,6946142252668845812_usize,1435232354621910301_usize,6_usize,3_usize,6202133341901818623_usize,5937901705326370310_usize];
_20 = _3 | _1;
_14 = [_10.1,_10.1,RET,_10.1,RET,_10.1];
RET = _10.1 * _10.1;
_23 = '\u{bed07}';
_10.0 = -22775_i16;
_10.2 = _14;
_2 = _1 > _1;
_18 = (-936556698_i32) as f32;
Goto(bb4)
}
bb4 = {
Call(_28 = dump_var(1_usize, 9_usize, Move(_9), 20_usize, Move(_20), 6_usize, Move(_6), 23_usize, Move(_23)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_28 = dump_var(1_usize, 5_usize, Move(_5), 3_usize, Move(_3), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i128,mut _2: u64) -> ([isize; 6],) {
mir! {
type RET = ([isize; 6],);
let _3: Adt39;
let _4: [isize; 2];
let _5: i32;
let _6: u32;
let _7: i8;
let _8: Adt52;
let _9: Adt53;
let _10: Adt46;
let _11: isize;
let _12: Adt43;
let _13: char;
let _14: f64;
let _15: u8;
let _16: char;
let _17: [i16; 3];
let _18: bool;
let _19: u8;
let _20: u128;
let _21: isize;
let _22: [isize; 6];
let _23: usize;
let _24: Adt46;
let _25: ();
let _26: ();
{
_1 = (-75_i8) as i128;
_2 = 15659720575803317542_u64 | 18212572501317284509_u64;
RET.0 = [29_isize,9223372036854775807_isize,9223372036854775807_isize,(-41_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET.0 = [9223372036854775807_isize,72_isize,(-110_isize),(-9223372036854775808_isize),9223372036854775807_isize,34_isize];
RET.0 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-8_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = !(-30176570268032734455110544194877712045_i128);
_4 = [(-36_isize),9223372036854775807_isize];
_1 = (-88890291655948729424525071098401271258_i128);
Goto(bb1)
}
bb1 = {
_1 = -(-23609837863718497753657594931706526965_i128);
_5 = (-9223372036854775808_isize) as i32;
_5 = 2136973722_i32;
RET.0 = [(-123_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-126_isize)];
_5 = (-868268629_i32);
_4 = [9223372036854775807_isize,9223372036854775807_isize];
_2 = 7264766331152439842_u64;
_6 = 1718831607_u32 | 249280621_u32;
_7 = (-22_i8);
_1 = 117_u8 as i128;
_6 = 3171822436_u32;
_5 = 33150_u16 as i32;
_4 = [(-9223372036854775808_isize),(-61_isize)];
_7 = _1 as i8;
_1 = -(-23371908538669244307398805172680480415_i128);
_4 = [(-9_isize),(-56_isize)];
RET.0 = [(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),112_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = !1730189201_i32;
_5 = !(-2132847207_i32);
_8.fld2.fld0.2 = [_5,_5,_5];
_8.fld2.fld0.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),77_isize];
_8.fld1.0 = [9223372036854775807_isize,9223372036854775807_isize,44_isize,9223372036854775807_isize,9223372036854775807_isize,(-17_isize)];
_7 = !(-93_i8);
_8.fld1 = RET;
Call(_8.fld2.fld1 = fn3(RET.0, RET.0, _8.fld1.0, _8.fld2.fld0.0, _8.fld1.0, _8.fld1.0, _8.fld2.fld0.0, _8.fld1.0, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.fld2.fld0 = (RET.0, _8.fld1.0, _8.fld2.fld1);
_8.fld2.fld0.2 = _8.fld2.fld1;
_8.fld2.fld0.1 = [9223372036854775807_isize,9223372036854775807_isize,(-111_isize),9223372036854775807_isize,(-58_isize),9223372036854775807_isize];
_8.fld2.fld2 = -(-9223372036854775808_isize);
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_8.fld2.fld2 = -9223372036854775807_isize;
_8.fld2.fld0.2 = [_5,_5,_5];
_8.fld2.fld0.2 = [_5,_5,_5];
_8.fld1 = RET;
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_8.fld2.fld0.0 = _8.fld2.fld0.1;
_6 = '\u{e3cc7}' as u32;
_8.fld1.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_4 = [_8.fld2.fld2,_8.fld2.fld2];
_4 = [_8.fld2.fld2,_8.fld2.fld2];
_8.fld2.fld0.1 = RET.0;
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
7264766331152439842 => bb8,
_ => bb7
}
}
bb3 = {
_1 = -(-23609837863718497753657594931706526965_i128);
_5 = (-9223372036854775808_isize) as i32;
_5 = 2136973722_i32;
RET.0 = [(-123_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-126_isize)];
_5 = (-868268629_i32);
_4 = [9223372036854775807_isize,9223372036854775807_isize];
_2 = 7264766331152439842_u64;
_6 = 1718831607_u32 | 249280621_u32;
_7 = (-22_i8);
_1 = 117_u8 as i128;
_6 = 3171822436_u32;
_5 = 33150_u16 as i32;
_4 = [(-9223372036854775808_isize),(-61_isize)];
_7 = _1 as i8;
_1 = -(-23371908538669244307398805172680480415_i128);
_4 = [(-9_isize),(-56_isize)];
RET.0 = [(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),112_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = !1730189201_i32;
_5 = !(-2132847207_i32);
_8.fld2.fld0.2 = [_5,_5,_5];
_8.fld2.fld0.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),77_isize];
_8.fld1.0 = [9223372036854775807_isize,9223372036854775807_isize,44_isize,9223372036854775807_isize,9223372036854775807_isize,(-17_isize)];
_7 = !(-93_i8);
_8.fld1 = RET;
Call(_8.fld2.fld1 = fn3(RET.0, RET.0, _8.fld1.0, _8.fld2.fld0.0, _8.fld1.0, _8.fld1.0, _8.fld2.fld0.0, _8.fld1.0, _7), ReturnTo(bb2), UnwindUnreachable())
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
_8.fld4 = [188175206715122793230154510476534150701_u128,94459635854544348716908093201057761156_u128,289792068364122933060473678158791252256_u128,250949672762154226192131670439141479431_u128,151841154593109497770834824537597366649_u128,292032980518139402287853369697763427711_u128];
_8.fld2.fld0 = (RET.0, RET.0, _8.fld2.fld1);
RET = _8.fld1;
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_14 = 80_u8 as f64;
_7 = 106_i8 >> _2;
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_15 = 107_u8;
match _2 {
0 => bb1,
1 => bb2,
2 => bb6,
7264766331152439842 => bb9,
_ => bb5
}
}
bb9 = {
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_7 = false as i8;
_8.fld2.fld0.0 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_8.fld2.fld0 = (RET.0, _8.fld1.0, _8.fld2.fld1);
_4 = [_8.fld2.fld2,_8.fld2.fld2];
_6 = !4039515029_u32;
_8.fld5 = _1;
_2 = 28294_u16 as u64;
_7 = '\u{3686a}' as i8;
_8.fld2.fld0.1 = RET.0;
_1 = _8.fld5 & _8.fld5;
_8.fld5 = _1;
_1 = _8.fld5 * _8.fld5;
_6 = 1544090763_u32;
_8.fld2.fld0.0 = _8.fld2.fld0.1;
_8.fld1 = (_8.fld2.fld0.1,);
_8.fld0 = core::ptr::addr_of!(_17);
_16 = '\u{ecc8c}';
_1 = _8.fld5 & _8.fld5;
Goto(bb10)
}
bb10 = {
_13 = _16;
_1 = _8.fld5;
_8.fld2.fld0.0 = _8.fld2.fld0.1;
_11 = -_8.fld2.fld2;
RET = (_8.fld2.fld0.1,);
_19 = _15 - _15;
_8.fld0 = core::ptr::addr_of!(_17);
_1 = _8.fld5;
_8.fld2.fld0.2 = [_5,_5,_5];
_8.fld2.fld0 = (_8.fld1.0, RET.0, _8.fld2.fld1);
_14 = (-160054404464391310_i64) as f64;
_8.fld0 = core::ptr::addr_of!(_17);
_6 = !2416441768_u32;
_19 = _14 as u8;
RET.0 = [_11,_11,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_11];
_8.fld2.fld0.1 = [_11,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_11];
_18 = _5 <= _5;
_8.fld1.0 = _8.fld2.fld0.1;
_8.fld1.0 = [_11,_8.fld2.fld2,_11,_11,_8.fld2.fld2,_11];
_8.fld5 = _2 as i128;
_8.fld2.fld1 = [_5,_5,_5];
RET.0 = _8.fld1.0;
RET.0 = [_8.fld2.fld2,_8.fld2.fld2,_11,_8.fld2.fld2,_8.fld2.fld2,_11];
_1 = _16 as i128;
match _15 {
0 => bb3,
1 => bb8,
2 => bb11,
3 => bb12,
107 => bb14,
_ => bb13
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
RET = (_8.fld2.fld0.1,);
_1 = -_8.fld5;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(2_usize, 6_usize, Move(_6), 11_usize, Move(_11), 18_usize, Move(_18), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(2_usize, 19_usize, Move(_19), 5_usize, Move(_5), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [isize; 6],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 6],mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [isize; 6],mut _8: [isize; 6],mut _9: i8) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _10: Adt52;
let _11: ([isize; 6],);
let _12: i16;
let _13: i32;
let _14: Adt45;
let _15: Adt42;
let _16: Adt47;
let _17: char;
let _18: u16;
let _19: usize;
let _20: i128;
let _21: bool;
let _22: f32;
let _23: (bool,);
let _24: (i16, u128, [u128; 6], (i8,), u8);
let _25: [isize; 2];
let _26: Adt40;
let _27: Adt49;
let _28: char;
let _29: u32;
let _30: isize;
let _31: Adt50;
let _32: [i16; 3];
let _33: isize;
let _34: ();
let _35: ();
{
_5 = _8;
RET = [(-1125893799_i32),518497416_i32,(-790232344_i32)];
_3 = _4;
_5 = [47_isize,27_isize,9223372036854775807_isize,(-61_isize),31_isize,9223372036854775807_isize];
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,103_isize,(-9223372036854775808_isize)];
_6 = [(-104_isize),9223372036854775807_isize,9223372036854775807_isize,117_isize,(-126_isize),(-9223372036854775808_isize)];
_9 = 98_i8;
_4 = [(-56_isize),9223372036854775807_isize,101_isize,8_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = _8;
RET = [390892127_i32,872133932_i32,271643262_i32];
_7 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,48_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_9 = -(-13_i8);
_7 = [9223372036854775807_isize,9223372036854775807_isize,(-39_isize),9223372036854775807_isize,0_isize,(-33_isize)];
_3 = _4;
_5 = _1;
_3 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,94_isize,(-42_isize)];
_4 = _7;
_4 = [9223372036854775807_isize,(-80_isize),100_isize,(-9223372036854775808_isize),(-32_isize),(-9223372036854775808_isize)];
_1 = _2;
Call(_5 = fn4(_7, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _2;
_8 = _1;
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize),83_isize,(-9223372036854775808_isize),(-78_isize),9223372036854775807_isize];
_8 = [31_isize,(-9223372036854775808_isize),(-92_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_2 = [9223372036854775807_isize,106_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_7 = [6_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-30_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = [(-35_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _5;
_3 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,45_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [1245608394_i32,(-1198906356_i32),1317301268_i32];
_11.0 = [9223372036854775807_isize,9223372036854775807_isize,(-18_isize),36_isize,89_isize,(-9223372036854775808_isize)];
_10.fld2.fld0.2 = [(-227666618_i32),1380686296_i32,(-589056297_i32)];
_10.fld1 = _11;
_10.fld2.fld2 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_10.fld4 = [191727597533035290598697002371799109258_u128,118191872657612813127980905332774229648_u128,107445745909077310629059513274759798244_u128,280116084266970430933164827200172669679_u128,247491637666380862283112651577040682120_u128,177056918869473494844231180341937000604_u128];
_10.fld2.fld0 = (_11.0, _5, RET);
_10.fld2.fld0.1 = _8;
_10.fld2.fld0.1 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld2.fld1 = [(-1752510419_i32),(-577569043_i32),866899926_i32];
_12 = (-13847_i16);
_6 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld2.fld2 = -(-24_isize);
_5 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld1 = (_8,);
_12 = 5287_i16 & 10235_i16;
Goto(bb2)
}
bb2 = {
_10.fld2.fld1 = [(-13338551_i32),1905024339_i32,(-1136737097_i32)];
_3 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_5 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld5 = (-91042406681609418278177365441125511658_i128) - 133109439734266233397176319158570394052_i128;
_1 = _5;
_10.fld2.fld0.2 = [1792287037_i32,605076353_i32,1481776235_i32];
Goto(bb3)
}
bb3 = {
RET = _10.fld2.fld1;
RET = _10.fld2.fld0.2;
_10.fld1.0 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
Goto(bb4)
}
bb4 = {
_7 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_7 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_9 = false as i8;
_12 = 22732_i16 + (-18024_i16);
_2 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld2.fld0.1 = _11.0;
_10.fld2.fld1 = [987879203_i32,(-25750941_i32),784167745_i32];
Goto(bb5)
}
bb5 = {
_11 = _10.fld1;
Goto(bb6)
}
bb6 = {
_12 = 26290_i16 >> _9;
RET = [1505148369_i32,1246024267_i32,(-1012678859_i32)];
_12 = 0_u8 as i16;
_3 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_12 = 15345_i16;
_10.fld1.0 = _8;
_10.fld2.fld0.1 = _10.fld1.0;
_12 = 32057_i16;
_17 = '\u{f7cfd}';
_10.fld2.fld0 = (_6, _8, RET);
_7 = _10.fld2.fld0.1;
_11.0 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_13 = 979215027_i32;
_8 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_7 = _2;
_11 = _10.fld1;
_8 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_10.fld2.fld0 = (_10.fld1.0, _11.0, RET);
_16 = Adt47::Variant1 { fld0: 174_u8 };
Goto(bb7)
}
bb7 = {
_11.0 = _3;
_10.fld2.fld1 = [_13,_13,_13];
_18 = 29719_u16;
_21 = true;
_10.fld2.fld2 = !9223372036854775807_isize;
_12 = (-6197_i16);
_23 = (_21,);
_11.0 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_11 = _10.fld1;
_13 = 318440171_i32;
_24.2 = [73820874256318040694191544818240596042_u128,231341473506323326108367232241054546505_u128,287654755141781551887619013646462751512_u128,247960641299696661412759821469922377104_u128,134101590415464665908191940641123477506_u128,78567122260100020142571668397739824283_u128];
RET = [_13,_13,_13];
_24.1 = 195089680096183895228159081517557901455_u128 << _10.fld5;
_20 = !_10.fld5;
_24.0 = _17 as i16;
_9 = (-14_i8) ^ 110_i8;
Goto(bb8)
}
bb8 = {
_8 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
place!(Field::<u8>(Variant(_16, 1), 0)) = 4461094689016641092_i64 as u8;
_4 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_3 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_19 = 5_usize;
_10.fld1.0[_19] = _10.fld2.fld0.0[_19] >> _20;
_7 = [_10.fld2.fld0.1[_19],_10.fld2.fld0.1[_19],_10.fld2.fld0.0[_19],_11.0[_19],_2[_19],_10.fld1.0[_19]];
_10.fld2.fld0 = (_11.0, _3, RET);
place!(Field::<u8>(Variant(_16, 1), 0)) = 213_u8;
_8[_19] = _10.fld2.fld0.0[_19] & _10.fld2.fld0.0[_19];
_24.3 = (_9,);
_10.fld1.0[_19] = _24.3.0 as isize;
SetDiscriminant(_16, 2);
_10.fld4[_19] = _24.1;
_4[_19] = 1172329221_u32 as isize;
_19 = !4_usize;
_22 = _19 as f32;
Goto(bb9)
}
bb9 = {
_19 = 4_usize;
_8[_19] = _11.0[_19] * _7[_19];
_18 = 28936_u16 << _24.3.0;
_2[_19] = _19 as isize;
_11.0 = _8;
_21 = !_23.0;
_10.fld2.fld0 = (_11.0, _8, RET);
_24.0 = _12;
_10.fld2.fld0 = (_8, _7, RET);
_10.fld4 = [_24.1,_24.1,_24.2[_19],_24.1,_24.2[_19],_24.2[_19]];
_24.4 = 221_u8;
_8 = [_10.fld1.0[_19],_3[_19],_10.fld2.fld0.0[_19],_5[_19],_6[_19],_5[_19]];
Call(_10.fld2.fld0.1[_19] = core::intrinsics::bswap(_6[_19]), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3[_19] = _8[_19];
_12 = _24.0;
_4[_19] = _12 as isize;
_22 = _19 as f32;
_27.fld0.2 = _10.fld2.fld1;
RET = _10.fld2.fld1;
_4 = _10.fld2.fld0.0;
_10.fld2.fld0.1[_19] = _18 as isize;
_21 = _1[_19] == _4[_19];
_10.fld2.fld0.2 = RET;
match _24.2[_19] {
134101590415464665908191940641123477506 => bb12,
_ => bb11
}
}
bb11 = {
RET = _10.fld2.fld1;
RET = _10.fld2.fld0.2;
_10.fld1.0 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
Goto(bb4)
}
bb12 = {
_27.fld0 = (_3, _10.fld2.fld0.0, _10.fld2.fld0.2);
_8[_19] = _27.fld0.0[_19] + _1[_19];
_10.fld2.fld1 = [_13,_13,_13];
_1[_19] = 2578115677_u32 as isize;
_27.fld1 = _10.fld2.fld0.2;
_1[_19] = _8[_19] * _3[_19];
_8[_19] = !_27.fld0.0[_19];
_27.fld1 = [_13,_13,_13];
_9 = _24.3.0 - _24.3.0;
_23 = (_21,);
_24.2 = [_10.fld4[_19],_10.fld4[_19],_10.fld4[_19],_10.fld4[_19],_10.fld4[_19],_24.1];
_10.fld2.fld0.1 = [_1[_19],_5[_19],_3[_19],_8[_19],_27.fld0.1[_19],_10.fld2.fld0.0[_19]];
_2 = [_11.0[_19],_6[_19],_8[_19],_4[_19],_8[_19],_4[_19]];
_25 = [_4[_19],_27.fld0.1[_19]];
_24.2[_19] = _24.1 - _24.1;
_10.fld2.fld0.1 = [_1[_19],_2[_19],_6[_19],_6[_19],_7[_19],_4[_19]];
_6 = [_10.fld2.fld0.1[_19],_1[_19],_4[_19],_1[_19],_7[_19],_1[_19]];
_11 = _10.fld1;
_9 = -_24.3.0;
_18 = 8874_u16 | 20664_u16;
_1[_19] = !_27.fld0.1[_19];
_17 = '\u{88c03}';
_4[_19] = 2772599625_u32 as isize;
_10.fld2.fld0.0[_19] = _24.4 as isize;
_18 = 65151_u16;
_13 = (-989513491_i32);
match _10.fld4[_19] {
0 => bb6,
1 => bb9,
2 => bb3,
3 => bb4,
134101590415464665908191940641123477506 => bb14,
_ => bb13
}
}
bb13 = {
RET = _10.fld2.fld1;
RET = _10.fld2.fld0.2;
_10.fld1.0 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
Goto(bb4)
}
bb14 = {
_22 = _24.4 as f32;
_10.fld1.0 = [_27.fld0.0[_19],_8[_19],_3[_19],_1[_19],_3[_19],_3[_19]];
_4 = [_10.fld2.fld0.0[_19],_10.fld1.0[_19],_8[_19],_10.fld2.fld0.0[_19],_27.fld0.0[_19],_27.fld0.1[_19]];
_10.fld2.fld0.1 = _27.fld0.1;
_10.fld2.fld0.1[_19] = !_8[_19];
_24.1 = _22 as u128;
_1[_19] = _5[_19];
_27.fld0.1[_19] = _4[_19];
_28 = _17;
_27.fld0.0[_19] = _5[_19] + _27.fld0.1[_19];
_1 = [_2[_19],_27.fld0.1[_19],_6[_19],_10.fld2.fld0.1[_19],_27.fld0.1[_19],_5[_19]];
_22 = _19 as f32;
_28 = _17;
_19 = 3769900227_u32 as usize;
_24.1 = 200402446227898878553407415303299555752_u128;
_10.fld2.fld0.1 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_5 = _10.fld2.fld0.0;
_27.fld2 = -_10.fld2.fld2;
_17 = _28;
_1 = [_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2,_27.fld2];
_12 = _24.3.0 as i16;
_7 = _6;
_2 = _10.fld2.fld0.0;
_23 = (_21,);
_24.3 = (_9,);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(3_usize, 12_usize, Move(_12), 9_usize, Move(_9), 28_usize, Move(_28), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(3_usize, 4_usize, Move(_4), 24_usize, Move(_24), 25_usize, Move(_25), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(3_usize, 13_usize, Move(_13), 1_usize, Move(_1), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [isize; 6],mut _2: [isize; 6]) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _3: [i32; 3];
let _4: i8;
let _5: char;
let _6: u64;
let _7: *const ([i32; 3],);
let _8: f64;
let _9: Adt53;
let _10: f64;
let _11: i8;
let _12: f64;
let _13: [isize; 6];
let _14: [i16; 3];
let _15: (i8,);
let _16: i64;
let _17: Adt49;
let _18: (f64, isize);
let _19: i128;
let _20: Adt52;
let _21: i16;
let _22: u128;
let _23: ([i32; 3],);
let _24: Adt47;
let _25: f64;
let _26: [u64; 6];
let _27: [isize; 6];
let _28: Adt48;
let _29: (i16, u128, [u128; 6], (i8,), u8);
let _30: u128;
let _31: bool;
let _32: bool;
let _33: char;
let _34: Adt55;
let _35: (i16, u128, [u128; 6], (i8,), u8);
let _36: [u128; 6];
let _37: [u8; 2];
let _38: u128;
let _39: [usize; 8];
let _40: *mut bool;
let _41: u8;
let _42: f32;
let _43: Adt47;
let _44: i128;
let _45: [isize; 6];
let _46: i16;
let _47: Adt50;
let _48: ();
let _49: ();
{
RET = [(-106_isize),16_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),22_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),16_isize,41_isize,(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_1 = _2;
RET = _1;
RET = [(-9223372036854775808_isize),(-26_isize),(-9223372036854775808_isize),(-10_isize),(-74_isize),(-56_isize)];
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
Call(_1 = fn5(_2, RET, RET, _2, RET, RET, RET, _2, _2, RET, _2, _2, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _2;
_1 = [67_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,81_isize];
RET = [95_isize,9223372036854775807_isize,111_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [(-1296129169_i32),(-2060955783_i32),1480250343_i32];
_4 = 142043699742193632963106220791076633253_i128 as i8;
_3 = [(-849868873_i32),2045230993_i32,70417475_i32];
_3 = [1453533810_i32,1934632855_i32,(-1750610886_i32)];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),62_isize,(-26_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [64447486_i32,(-1797102168_i32),1440407537_i32];
_2 = _1;
_4 = -(-23_i8);
_4 = (-34_i8) | 6_i8;
Goto(bb2)
}
bb2 = {
_4 = 53_i8 * 43_i8;
_5 = '\u{d4478}';
RET = _1;
_4 = 112_u8 as i8;
RET = [(-79_isize),(-101_isize),87_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),47_isize,113_isize];
RET = _1;
RET = [81_isize,(-22_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,64_isize,(-68_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-78_isize)];
_4 = 65_i8;
_1 = [(-76_isize),(-71_isize),(-81_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _2;
RET = [(-9223372036854775808_isize),(-7_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 16580166223634381416_u64 & 14255446917481077103_u64;
_1 = _2;
_3 = [2086976645_i32,(-132348943_i32),1997887790_i32];
_2 = _1;
_10 = _6 as f64;
RET = [87_isize,71_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,29_isize];
_8 = _10 + _10;
_1 = [(-26_isize),9223372036854775807_isize,(-32_isize),9223372036854775807_isize,(-9223372036854775808_isize),109_isize];
_6 = !10204499251473880894_u64;
RET = [(-9223372036854775808_isize),(-17_isize),45_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),83_isize];
_8 = _10 * _10;
_8 = _10 * _10;
Goto(bb3)
}
bb3 = {
_10 = 4372_u16 as f64;
_6 = !13390134914342264866_u64;
RET = _2;
_10 = 56722_u16 as f64;
_8 = _10;
RET = _1;
_11 = !_4;
RET = _2;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,69_isize,73_isize,115_isize];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-91_isize),(-9223372036854775808_isize),113_isize,9223372036854775807_isize];
_11 = _8 as i8;
_5 = '\u{1b79e}';
_4 = !_11;
_3 = [(-1130804669_i32),(-171087075_i32),(-1170320854_i32)];
RET = [9223372036854775807_isize,36_isize,9223372036854775807_isize,(-69_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_12 = -_10;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),116_isize,(-107_isize),9223372036854775807_isize];
Goto(bb4)
}
bb4 = {
_3 = [(-1740161843_i32),(-1008906571_i32),968696831_i32];
_3 = [1217312077_i32,(-253663051_i32),1211096972_i32];
_5 = '\u{8daf1}';
_12 = _8 - _10;
RET = [(-9223372036854775808_isize),(-121_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-55_isize),(-9223372036854775808_isize)];
_2 = _1;
_5 = '\u{85062}';
_15.0 = -_11;
_2 = [48_isize,9223372036854775807_isize,79_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-83_isize),48_isize,(-9223372036854775808_isize)];
_1 = _13;
_6 = 10703170354778811134_u64 >> _4;
_11 = _4;
_14 = [2144_i16,14858_i16,16619_i16];
_4 = _11 >> _15.0;
Goto(bb5)
}
bb5 = {
_11 = -_4;
RET = _13;
_15 = (_4,);
_14 = [31867_i16,(-9472_i16),(-11023_i16)];
_16 = 2418564239950958251_i64;
_8 = _15.0 as f64;
_13 = [9223372036854775807_isize,(-103_isize),9223372036854775807_isize,(-112_isize),100_isize,9223372036854775807_isize];
_6 = 726668915833541362_u64 ^ 3396023067574623277_u64;
RET = [63_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-1_isize),(-9223372036854775808_isize),(-75_isize)];
_17.fld0.2 = [(-1091494488_i32),(-170415413_i32),(-556892202_i32)];
_2 = [(-121_isize),(-69_isize),18_isize,9223372036854775807_isize,9223372036854775807_isize,97_isize];
_17.fld0 = (RET, _13, _3);
_15 = (_4,);
_20.fld0 = core::ptr::addr_of!(_14);
_20.fld2 = Adt49 { fld0: _17.fld0,fld1: _3,fld2: 12_isize };
_17.fld2 = _20.fld2.fld2 << _20.fld2.fld2;
_4 = !_15.0;
_20.fld1.0 = [_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2,_17.fld2];
_20.fld4 = [33528107998948044231426723193165206070_u128,53812555814232973867630708660321978695_u128,47086042802082134322593569456640552796_u128,89601091495322998263912799550234502656_u128,189237746984159698931060144031588277015_u128,236400125944835633136007163565254042053_u128];
_23.0 = [(-1910152419_i32),(-423554076_i32),165509086_i32];
_13 = [_20.fld2.fld2,_17.fld2,_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2];
match _20.fld2.fld2 {
12 => bb6,
_ => bb2
}
}
bb6 = {
_16 = -(-488687373969378366_i64);
_20.fld2.fld0 = (_17.fld0.0, _17.fld0.0, _17.fld0.2);
_18.0 = -_10;
_18.0 = -_10;
_21 = -(-21620_i16);
_14 = [_21,_21,_21];
_17.fld0 = (_20.fld2.fld0.1, _2, _23.0);
_15 = (_4,);
_11 = 9386877002608560053_usize as i8;
_15 = (_11,);
_21 = (-27790_i16) - 723_i16;
_20.fld2.fld0.1 = _17.fld0.0;
_14 = [_21,_21,_21];
_18.1 = 19146414658468136258256366701556162988_u128 as isize;
_8 = _12 * _10;
_14 = [_21,_21,_21];
_17.fld2 = _20.fld2.fld2 | _20.fld2.fld2;
_20.fld2.fld0.2 = _3;
_17 = Adt49 { fld0: _20.fld2.fld0,fld1: _20.fld2.fld0.2,fld2: _20.fld2.fld2 };
_7 = core::ptr::addr_of!(_23);
_22 = 52837948341903390140364743875181945248_u128;
_23.0 = [1275609964_i32,(-198190149_i32),(-770649194_i32)];
_17.fld0.1 = _20.fld2.fld0.0;
_25 = _8;
Goto(bb7)
}
bb7 = {
_20.fld2.fld0 = _17.fld0;
_20.fld2.fld0 = (_20.fld1.0, _13, (*_7).0);
_27 = [_20.fld2.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2];
_27 = _1;
RET = [_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_20.fld2.fld2,_18.1,_17.fld2];
_20.fld0 = core::ptr::addr_of!(_14);
_10 = _22 as f64;
_30 = _22;
_3 = [50629055_i32,(-424050191_i32),(-180423420_i32)];
_8 = -_25;
_6 = !10172479684134270071_u64;
_17.fld0.0 = [_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2,_17.fld2];
_10 = _25 * _12;
_28.fld2 = [51_u8,68_u8];
(*_7).0 = _20.fld2.fld1;
_4 = -_11;
_8 = _10 + _25;
_30 = _22;
Goto(bb8)
}
bb8 = {
_3 = (*_7).0;
_20.fld2 = Adt49 { fld0: _17.fld0,fld1: _23.0,fld2: _17.fld2 };
_12 = _25 + _8;
_20.fld2.fld0.0 = [_20.fld2.fld2,_20.fld2.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_20.fld2.fld2];
_8 = -_12;
_4 = _11 >> _17.fld2;
_29.1 = _30 ^ _30;
_23.0 = [739393206_i32,(-2097208863_i32),(-1400290411_i32)];
_28.fld1 = !2387581547720455386_usize;
_29.1 = !_30;
_16 = 5139228162396076035_i64;
_35.2 = [_29.1,_22,_29.1,_30,_29.1,_30];
_17 = _20.fld2;
_28.fld0 = core::ptr::addr_of!((*_7));
(*_7) = (_17.fld1,);
_18.0 = _6 as f64;
_29.3.0 = _4 & _4;
_39 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_26 = [_6,_6,_6,_6,_6,_6];
_35.0 = _21 * _21;
_7 = _28.fld0;
_20.fld2 = Adt49 { fld0: _17.fld0,fld1: (*_7).0,fld2: _17.fld2 };
_23 = (_17.fld0.2,);
_18 = (_25, _17.fld2);
Goto(bb9)
}
bb9 = {
_33 = _5;
(*_7) = (_3,);
_29.3 = (_4,);
_36 = [_29.1,_29.1,_30,_29.1,_30,_29.1];
_39 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_29.3 = (_11,);
_40 = core::ptr::addr_of_mut!(_32);
_35.3 = (_4,);
_19 = (-166271553641474728631692767340722677524_i128);
_35.0 = _21;
_26 = [_6,_6,_6,_6,_6,_6];
match _18.1 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
12 => bb17,
_ => bb16
}
}
bb10 = {
RET = _2;
_1 = [67_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,81_isize];
RET = [95_isize,9223372036854775807_isize,111_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [(-1296129169_i32),(-2060955783_i32),1480250343_i32];
_4 = 142043699742193632963106220791076633253_i128 as i8;
_3 = [(-849868873_i32),2045230993_i32,70417475_i32];
_3 = [1453533810_i32,1934632855_i32,(-1750610886_i32)];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),62_isize,(-26_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [64447486_i32,(-1797102168_i32),1440407537_i32];
_2 = _1;
_4 = -(-23_i8);
_4 = (-34_i8) | 6_i8;
Goto(bb2)
}
bb11 = {
_20.fld2.fld0 = _17.fld0;
_20.fld2.fld0 = (_20.fld1.0, _13, (*_7).0);
_27 = [_20.fld2.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2];
_27 = _1;
RET = [_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_20.fld2.fld2,_18.1,_17.fld2];
_20.fld0 = core::ptr::addr_of!(_14);
_10 = _22 as f64;
_30 = _22;
_3 = [50629055_i32,(-424050191_i32),(-180423420_i32)];
_8 = -_25;
_6 = !10172479684134270071_u64;
_17.fld0.0 = [_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2,_17.fld2];
_10 = _25 * _12;
_28.fld2 = [51_u8,68_u8];
(*_7).0 = _20.fld2.fld1;
_4 = -_11;
_8 = _10 + _25;
_30 = _22;
Goto(bb8)
}
bb12 = {
_16 = -(-488687373969378366_i64);
_20.fld2.fld0 = (_17.fld0.0, _17.fld0.0, _17.fld0.2);
_18.0 = -_10;
_18.0 = -_10;
_21 = -(-21620_i16);
_14 = [_21,_21,_21];
_17.fld0 = (_20.fld2.fld0.1, _2, _23.0);
_15 = (_4,);
_11 = 9386877002608560053_usize as i8;
_15 = (_11,);
_21 = (-27790_i16) - 723_i16;
_20.fld2.fld0.1 = _17.fld0.0;
_14 = [_21,_21,_21];
_18.1 = 19146414658468136258256366701556162988_u128 as isize;
_8 = _12 * _10;
_14 = [_21,_21,_21];
_17.fld2 = _20.fld2.fld2 | _20.fld2.fld2;
_20.fld2.fld0.2 = _3;
_17 = Adt49 { fld0: _20.fld2.fld0,fld1: _20.fld2.fld0.2,fld2: _20.fld2.fld2 };
_7 = core::ptr::addr_of!(_23);
_22 = 52837948341903390140364743875181945248_u128;
_23.0 = [1275609964_i32,(-198190149_i32),(-770649194_i32)];
_17.fld0.1 = _20.fld2.fld0.0;
_25 = _8;
Goto(bb7)
}
bb13 = {
_11 = -_4;
RET = _13;
_15 = (_4,);
_14 = [31867_i16,(-9472_i16),(-11023_i16)];
_16 = 2418564239950958251_i64;
_8 = _15.0 as f64;
_13 = [9223372036854775807_isize,(-103_isize),9223372036854775807_isize,(-112_isize),100_isize,9223372036854775807_isize];
_6 = 726668915833541362_u64 ^ 3396023067574623277_u64;
RET = [63_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-1_isize),(-9223372036854775808_isize),(-75_isize)];
_17.fld0.2 = [(-1091494488_i32),(-170415413_i32),(-556892202_i32)];
_2 = [(-121_isize),(-69_isize),18_isize,9223372036854775807_isize,9223372036854775807_isize,97_isize];
_17.fld0 = (RET, _13, _3);
_15 = (_4,);
_20.fld0 = core::ptr::addr_of!(_14);
_20.fld2 = Adt49 { fld0: _17.fld0,fld1: _3,fld2: 12_isize };
_17.fld2 = _20.fld2.fld2 << _20.fld2.fld2;
_4 = !_15.0;
_20.fld1.0 = [_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2,_17.fld2,_17.fld2];
_20.fld4 = [33528107998948044231426723193165206070_u128,53812555814232973867630708660321978695_u128,47086042802082134322593569456640552796_u128,89601091495322998263912799550234502656_u128,189237746984159698931060144031588277015_u128,236400125944835633136007163565254042053_u128];
_23.0 = [(-1910152419_i32),(-423554076_i32),165509086_i32];
_13 = [_20.fld2.fld2,_17.fld2,_17.fld2,_17.fld2,_20.fld2.fld2,_20.fld2.fld2];
match _20.fld2.fld2 {
12 => bb6,
_ => bb2
}
}
bb14 = {
_3 = [(-1740161843_i32),(-1008906571_i32),968696831_i32];
_3 = [1217312077_i32,(-253663051_i32),1211096972_i32];
_5 = '\u{8daf1}';
_12 = _8 - _10;
RET = [(-9223372036854775808_isize),(-121_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-55_isize),(-9223372036854775808_isize)];
_2 = _1;
_5 = '\u{85062}';
_15.0 = -_11;
_2 = [48_isize,9223372036854775807_isize,79_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-83_isize),48_isize,(-9223372036854775808_isize)];
_1 = _13;
_6 = 10703170354778811134_u64 >> _4;
_11 = _4;
_14 = [2144_i16,14858_i16,16619_i16];
_4 = _11 >> _15.0;
Goto(bb5)
}
bb15 = {
_10 = 4372_u16 as f64;
_6 = !13390134914342264866_u64;
RET = _2;
_10 = 56722_u16 as f64;
_8 = _10;
RET = _1;
_11 = !_4;
RET = _2;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,69_isize,73_isize,115_isize];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-91_isize),(-9223372036854775808_isize),113_isize,9223372036854775807_isize];
_11 = _8 as i8;
_5 = '\u{1b79e}';
_4 = !_11;
_3 = [(-1130804669_i32),(-171087075_i32),(-1170320854_i32)];
RET = [9223372036854775807_isize,36_isize,9223372036854775807_isize,(-69_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_12 = -_10;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),116_isize,(-107_isize),9223372036854775807_isize];
Goto(bb4)
}
bb16 = {
_4 = 53_i8 * 43_i8;
_5 = '\u{d4478}';
RET = _1;
_4 = 112_u8 as i8;
RET = [(-79_isize),(-101_isize),87_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),47_isize,113_isize];
RET = _1;
RET = [81_isize,(-22_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,64_isize,(-68_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-78_isize)];
_4 = 65_i8;
_1 = [(-76_isize),(-71_isize),(-81_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _2;
RET = [(-9223372036854775808_isize),(-7_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 16580166223634381416_u64 & 14255446917481077103_u64;
_1 = _2;
_3 = [2086976645_i32,(-132348943_i32),1997887790_i32];
_2 = _1;
_10 = _6 as f64;
RET = [87_isize,71_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,29_isize];
_8 = _10 + _10;
_1 = [(-26_isize),9223372036854775807_isize,(-32_isize),9223372036854775807_isize,(-9223372036854775808_isize),109_isize];
_6 = !10204499251473880894_u64;
RET = [(-9223372036854775808_isize),(-17_isize),45_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),83_isize];
_8 = _10 * _10;
_8 = _10 * _10;
Goto(bb3)
}
bb17 = {
_20.fld2.fld0.2 = [(-1843631765_i32),157834864_i32,(-1253971659_i32)];
_35.4 = 241_u8 | 175_u8;
_17.fld0.2 = _17.fld1;
_7 = core::ptr::addr_of!((*_7));
_29.2 = [_30,_30,_30,_29.1,_30,_30];
_32 = !false;
_22 = !_29.1;
_12 = _6 as f64;
_29.3.0 = !_35.3.0;
_27 = _20.fld2.fld0.0;
_35.4 = 228_u8;
_32 = _33 != _5;
_20.fld2.fld1 = [2097595875_i32,(-642583857_i32),(-1394178611_i32)];
_35.4 = 791591323_u32 as u8;
_14 = [_35.0,_35.0,_21];
_20.fld5 = _19 & _19;
Goto(bb18)
}
bb18 = {
Call(_48 = dump_var(4_usize, 23_usize, Move(_23), 27_usize, Move(_27), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(4_usize, 5_usize, Move(_5), 2_usize, Move(_2), 26_usize, Move(_26), 32_usize, Move(_32)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(4_usize, 33_usize, Move(_33), 16_usize, Move(_16), 19_usize, Move(_19), 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [isize; 6],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 6],mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [isize; 6],mut _8: [isize; 6],mut _9: [isize; 6],mut _10: [isize; 6],mut _11: [isize; 6],mut _12: [isize; 6],mut _13: [isize; 6],mut _14: [isize; 6],mut _15: [isize; 6]) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _16: Adt51;
let _17: [usize; 8];
let _18: bool;
let _19: isize;
let _20: (u64, i16, i128, f64, i128);
let _21: isize;
let _22: char;
let _23: isize;
let _24: Adt39;
let _25: u32;
let _26: char;
let _27: [i32; 3];
let _28: [u8; 2];
let _29: Adt39;
let _30: [i32; 3];
let _31: Adt42;
let _32: u16;
let _33: u8;
let _34: Adt52;
let _35: f32;
let _36: [isize; 6];
let _37: *const [i16; 3];
let _38: Adt55;
let _39: (bool,);
let _40: [u64; 6];
let _41: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _42: [i32; 3];
let _43: i32;
let _44: ([isize; 6], [isize; 6], [i32; 3]);
let _45: char;
let _46: [u128; 6];
let _47: ([i32; 3],);
let _48: isize;
let _49: [isize; 6];
let _50: [i32; 3];
let _51: u128;
let _52: f64;
let _53: [u64; 6];
let _54: ([isize; 6],);
let _55: ();
let _56: ();
{
_15 = [(-21_isize),(-9223372036854775808_isize),(-60_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,(-93_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_13 = _15;
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-50_isize)];
_6 = _5;
_4 = _6;
Goto(bb1)
}
bb1 = {
_3 = _9;
_13 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),15_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-97_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = _2;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-3_isize),9223372036854775807_isize];
RET = _13;
_13 = [(-9223372036854775808_isize),84_isize,(-9223372036854775808_isize),9223372036854775807_isize,74_isize,(-18_isize)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,80_isize,55_isize,(-9223372036854775808_isize)];
_4 = RET;
_14 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-127_isize),125_isize];
_2 = [9223372036854775807_isize,35_isize,(-47_isize),9223372036854775807_isize,96_isize,9223372036854775807_isize];
_8 = _15;
_17 = [11057772501762766769_usize,4343903160096590197_usize,520438980272046532_usize,0_usize,5928227986584442409_usize,10007258884700038826_usize,16578656879306812931_usize,16612700822729852390_usize];
_7 = [20_isize,(-56_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-39_isize),98_isize,(-93_isize),(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = [(-13_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),87_isize,9223372036854775807_isize];
_14 = _5;
_14 = [9223372036854775807_isize,9223372036854775807_isize,69_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20.0 = 14641826093622437203_u64 ^ 12178871965881049455_u64;
_15 = [(-9223372036854775808_isize),37_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,83_isize];
_20.4 = (-70494583368822351175717184367705324163_i128);
RET = [(-92_isize),(-9223372036854775808_isize),83_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-64_isize)];
Goto(bb2)
}
bb2 = {
_9 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = _9;
_1 = [9223372036854775807_isize,88_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),100_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-67_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),22_isize];
_20.3 = _20.0 as f64;
_22 = '\u{6771f}';
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,15_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_19 = (-9223372036854775808_isize);
_21 = _19 & _19;
_25 = !3246533048_u32;
_20.3 = 100274638_i32 as f64;
_6 = [_19,_21,_19,_21,_21,_21];
_13 = [_21,_19,_19,_21,_21,_21];
_19 = _21;
_20.1 = -(-30094_i16);
_15 = [_19,_21,_21,_21,_19,_19];
_8 = _1;
_14 = [_19,_21,_21,_19,_19,_19];
_18 = !true;
_8 = [_21,_19,_21,_19,_19,_21];
_11 = _9;
Call(_25 = fn6(_17, RET, _2, _9, _10, _10, _5, _11, _3, _10, _2, _1, _20.4, _10, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_20.4 = (-398004238831031094332455101264220562_i128) | (-120187622804638154448748304800775210602_i128);
_14 = [_19,_21,_19,_19,_19,_19];
_20.0 = !17998648880209340667_u64;
_16 = Adt51::Variant0 { fld0: _20.0 };
_22 = '\u{29443}';
_17 = [5_usize,0_usize,3_usize,14271828454453637370_usize,3_usize,14874616841001607656_usize,6676528347331218536_usize,5_usize];
_22 = '\u{9b17f}';
_20.1 = (-388_i16);
_18 = true;
_10 = [_21,_19,_19,_21,_21,_19];
RET = _2;
place!(Field::<u64>(Variant(_16, 0), 0)) = (-517874841_i32) as u64;
SetDiscriminant(_16, 0);
_20.2 = _20.4;
place!(Field::<u64>(Variant(_16, 0), 0)) = _20.0;
_27 = [2095244553_i32,1406869945_i32,1477690122_i32];
_25 = 2138524201_u32 - 999085020_u32;
_21 = 1187681563020169846_usize as isize;
_12 = [_19,_21,_21,_19,_19,_19];
_6 = _5;
_14 = _4;
_14 = _1;
Goto(bb4)
}
bb4 = {
_3 = [_21,_19,_19,_21,_19,_19];
_22 = '\u{381c2}';
_20.0 = 33_i8 as u64;
RET = [_19,_19,_19,_19,_19,_19];
RET = [_21,_19,_19,_19,_21,_19];
RET = [_21,_21,_19,_19,_21,_19];
_20.3 = 7402350632662016884_usize as f64;
_25 = !3076567204_u32;
_8 = _11;
place!(Field::<u64>(Variant(_16, 0), 0)) = !_20.0;
_23 = 6_usize as isize;
_5 = _1;
_17 = [12528854810502360606_usize,7080815965810306557_usize,0_usize,6094548475948027314_usize,0_usize,5_usize,4_usize,17742298657078293862_usize];
_28 = [144_u8,74_u8];
RET = _8;
_10 = _1;
_28 = [197_u8,115_u8];
_12 = [_21,_19,_19,_19,_21,_19];
_20.4 = _20.2;
_13 = _5;
_27 = [794131126_i32,(-787203298_i32),(-1720010927_i32)];
Goto(bb5)
}
bb5 = {
_11 = [_19,_23,_21,_19,_21,_23];
_2 = [_23,_23,_19,_23,_21,_19];
_12 = [_23,_21,_21,_19,_21,_19];
Goto(bb6)
}
bb6 = {
_13 = [_21,_21,_19,_23,_23,_19];
_11 = [_21,_21,_19,_19,_19,_19];
SetDiscriminant(_16, 1);
_25 = 3490949009_u32 - 3219740417_u32;
_19 = _21;
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).2 = [207287818353704626948735591129234201777_u128,41402722915736614276761037626481835076_u128,101527638162189792231979225404511406155_u128,142911008356511798310827916119456383410_u128,136745552117935261746532287279255373582_u128,102602729460317418550531486864099545765_u128];
_32 = !49933_u16;
_25 = _20.1 as u32;
Goto(bb7)
}
bb7 = {
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).1 = 332525483543345083679618360617698154369_u128 << _25;
_20.4 = !_20.2;
place!(Field::<[u128; 6]>(Variant(_16, 1), 0)) = [Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1];
_3 = _1;
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).3.0 = !(-128_i8);
_4 = [_21,_21,_23,_21,_19,_23];
_20.4 = -_20.2;
_34.fld4 = [Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1];
_26 = _22;
_34.fld2.fld0 = (_5, _6, _27);
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).4 = !244_u8;
_34.fld1 = (_14,);
_30 = [(-1688759514_i32),(-879400130_i32),919349428_i32];
Goto(bb8)
}
bb8 = {
RET = [_23,_21,_23,_21,_19,_19];
place!(Field::<isize>(Variant(_16, 1), 2)) = _32 as isize;
_36 = [Field::<isize>(Variant(_16, 1), 2),_23,_23,_23,_19,_23];
_34.fld2.fld1 = [(-1884918002_i32),(-432561163_i32),(-302955364_i32)];
_34.fld2.fld2 = -_21;
_34.fld2.fld0.2 = _30;
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5)).3 = -_20.3;
_18 = !false;
_41.6.3 = Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).3 - Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).3;
_41.6.4 = _20.2 >> _34.fld2.fld2;
_41.3 = [Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1];
_34.fld2.fld2 = !_21;
place!(Field::<*mut bool>(Variant(_16, 1), 4)) = core::ptr::addr_of_mut!(_39.0);
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5)).1 = _20.4 as i16;
match _20.1 {
0 => bb1,
1 => bb3,
2 => bb9,
3 => bb10,
340282366920938463463374607431768211068 => bb12,
_ => bb11
}
}
bb9 = {
_3 = [_21,_19,_19,_21,_19,_19];
_22 = '\u{381c2}';
_20.0 = 33_i8 as u64;
RET = [_19,_19,_19,_19,_19,_19];
RET = [_21,_19,_19,_19,_21,_19];
RET = [_21,_21,_19,_19,_21,_19];
_20.3 = 7402350632662016884_usize as f64;
_25 = !3076567204_u32;
_8 = _11;
place!(Field::<u64>(Variant(_16, 0), 0)) = !_20.0;
_23 = 6_usize as isize;
_5 = _1;
_17 = [12528854810502360606_usize,7080815965810306557_usize,0_usize,6094548475948027314_usize,0_usize,5_usize,4_usize,17742298657078293862_usize];
_28 = [144_u8,74_u8];
RET = _8;
_10 = _1;
_28 = [197_u8,115_u8];
_12 = [_21,_19,_19,_19,_21,_19];
_20.4 = _20.2;
_13 = _5;
_27 = [794131126_i32,(-787203298_i32),(-1720010927_i32)];
Goto(bb5)
}
bb10 = {
_3 = _9;
_13 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),15_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-97_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = _2;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-3_isize),9223372036854775807_isize];
RET = _13;
_13 = [(-9223372036854775808_isize),84_isize,(-9223372036854775808_isize),9223372036854775807_isize,74_isize,(-18_isize)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,80_isize,55_isize,(-9223372036854775808_isize)];
_4 = RET;
_14 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-127_isize),125_isize];
_2 = [9223372036854775807_isize,35_isize,(-47_isize),9223372036854775807_isize,96_isize,9223372036854775807_isize];
_8 = _15;
_17 = [11057772501762766769_usize,4343903160096590197_usize,520438980272046532_usize,0_usize,5928227986584442409_usize,10007258884700038826_usize,16578656879306812931_usize,16612700822729852390_usize];
_7 = [20_isize,(-56_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-39_isize),98_isize,(-93_isize),(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = [(-13_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),87_isize,9223372036854775807_isize];
_14 = _5;
_14 = [9223372036854775807_isize,9223372036854775807_isize,69_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20.0 = 14641826093622437203_u64 ^ 12178871965881049455_u64;
_15 = [(-9223372036854775808_isize),37_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,83_isize];
_20.4 = (-70494583368822351175717184367705324163_i128);
RET = [(-92_isize),(-9223372036854775808_isize),83_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-64_isize)];
Goto(bb2)
}
bb11 = {
_11 = [_19,_23,_21,_19,_21,_23];
_2 = [_23,_23,_19,_23,_21,_19];
_12 = [_23,_21,_21,_19,_21,_19];
Goto(bb6)
}
bb12 = {
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5)) = _20;
_20.1 = !Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).1;
_41.6.4 = _18 as i128;
_20.0 = Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).4 as u64;
_14 = [Field::<isize>(Variant(_16, 1), 2),Field::<isize>(Variant(_16, 1), 2),_21,_23,_23,_23];
_5 = [Field::<isize>(Variant(_16, 1), 2),Field::<isize>(Variant(_16, 1), 2),_21,Field::<isize>(Variant(_16, 1), 2),_23,Field::<isize>(Variant(_16, 1), 2)];
_41.1 = Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1 + Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1;
_41.2.0 = _41.6.3 + _20.3;
_34.fld2.fld1 = [(-1409662560_i32),1692235376_i32,(-2009288874_i32)];
_39.0 = _18;
_43 = (-1265810000_i32) & 235205134_i32;
_34.fld1.0 = [_23,Field::<isize>(Variant(_16, 1), 2),Field::<isize>(Variant(_16, 1), 2),_23,_19,_23];
_12 = [Field::<isize>(Variant(_16, 1), 2),_34.fld2.fld2,_34.fld2.fld2,_21,Field::<isize>(Variant(_16, 1), 2),_34.fld2.fld2];
_41.4.0 = _7;
_28 = [Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).4,Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).4];
Call(place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).3.0 = core::intrinsics::transmute(_39.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.4 = -Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).4;
_35 = _20.2 as f32;
_7 = [_34.fld2.fld2,_23,_19,_19,_21,_19];
_23 = _19;
_41.6.4 = _26 as i128;
_47.0 = [_43,_43,_43];
_49 = [Field::<isize>(Variant(_16, 1), 2),_34.fld2.fld2,_19,_21,_19,Field::<isize>(Variant(_16, 1), 2)];
_29 = Adt39::Variant0 { fld0: _47,fld1: _34.fld1,fld2: _28,fld3: Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).3.0,fld4: _20.0,fld5: _17,fld6: _35 };
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5)).1 = _41.1 as i16;
SetDiscriminant(_29, 1);
_21 = _23;
RET = [_34.fld2.fld2,_19,_34.fld2.fld2,_23,Field::<isize>(Variant(_16, 1), 2),_19];
_41.4.0 = [_21,_21,Field::<isize>(Variant(_16, 1), 2),_21,_21,_23];
_34.fld3 = Adt46::Variant1 { fld0: _20.2,fld1: _26,fld2: _17,fld3: _35 };
_41.4 = _34.fld1;
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1)).2 = _41.3;
place!(Field::<u8>(Variant(_16, 1), 6)) = !Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).4;
place!(Field::<i8>(Variant(_16, 1), 3)) = -Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).3.0;
_44.2 = [_43,_43,_43];
SetDiscriminant(_34.fld3, 0);
_26 = _22;
_41.6.1 = _35 as i16;
_41.6.2 = _41.6.3 as i128;
Goto(bb14)
}
bb14 = {
_34.fld2.fld0.1 = _10;
place!(Field::<(i8,)>(Variant(_34.fld3, 0), 7)) = (Field::<i8>(Variant(_16, 1), 3),);
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_34.fld3, 0), 5)).1 = Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).1 ^ _41.1;
_45 = _22;
RET = [Field::<isize>(Variant(_16, 1), 2),_34.fld2.fld2,Field::<isize>(Variant(_16, 1), 2),_23,_21,_19];
_44.0 = [_23,Field::<isize>(Variant(_16, 1), 2),_19,_34.fld2.fld2,Field::<isize>(Variant(_16, 1), 2),_19];
RET = [_23,_19,_23,_34.fld2.fld2,_23,_21];
_1 = [_19,Field::<isize>(Variant(_16, 1), 2),_21,_19,Field::<isize>(Variant(_16, 1), 2),_23];
_34.fld2.fld2 = _41.2.0 as isize;
_20.2 = Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).4 << Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_34.fld3, 0), 5).1;
place!(Field::<([isize; 6],)>(Variant(_34.fld3, 0), 1)) = (_34.fld2.fld0.1,);
_47 = (_44.2,);
_49 = [_23,_21,_21,_19,_21,_34.fld2.fld2];
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_34.fld3, 0), 5)).3 = Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).3;
_41.2.0 = Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).3 * _20.3;
_41.5 = [Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).0,_20.0,Field::<(u64, i16, i128, f64, i128)>(Variant(_16, 1), 5).0,_20.0,_20.0,_20.0];
place!(Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_34.fld3, 0), 5)).0 = _32 as i16;
place!(Field::<i8>(Variant(_16, 1), 3)) = !Field::<(i16, u128, [u128; 6], (i8,), u8)>(Variant(_16, 1), 1).3.0;
_34.fld2.fld0.0 = _9;
_47.0 = [_43,_43,_43];
RET = [_34.fld2.fld2,_34.fld2.fld2,_21,_23,_34.fld2.fld2,_23];
_41.3 = Field::<[u128; 6]>(Variant(_16, 1), 0);
_47.0 = [_43,_43,_43];
Goto(bb15)
}
bb15 = {
Call(_55 = dump_var(5_usize, 28_usize, Move(_28), 14_usize, Move(_14), 47_usize, Move(_47), 49_usize, Move(_49)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(5_usize, 3_usize, Move(_3), 45_usize, Move(_45), 18_usize, Move(_18), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(5_usize, 36_usize, Move(_36), 8_usize, Move(_8), 1_usize, Move(_1), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_55 = dump_var(5_usize, 19_usize, Move(_19), 10_usize, Move(_10), 39_usize, Move(_39), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [usize; 8],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 6],mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [isize; 6],mut _8: [isize; 6],mut _9: [isize; 6],mut _10: [isize; 6],mut _11: [isize; 6],mut _12: [isize; 6],mut _13: i128,mut _14: [isize; 6],mut _15: [isize; 6]) -> u32 {
mir! {
type RET = u32;
let _16: bool;
let _17: ([i32; 3],);
let _18: [bool; 2];
let _19: isize;
let _20: bool;
let _21: isize;
let _22: char;
let _23: [u8; 2];
let _24: [i32; 3];
let _25: f64;
let _26: u32;
let _27: *mut bool;
let _28: i8;
let _29: u64;
let _30: [u64; 6];
let _31: ([isize; 6], [isize; 6], [i32; 3]);
let _32: u32;
let _33: Adt39;
let _34: i128;
let _35: Adt39;
let _36: i16;
let _37: i32;
let _38: Adt55;
let _39: u16;
let _40: u8;
let _41: ();
let _42: ();
{
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-77_isize),9223372036854775807_isize,48_isize];
_10 = _11;
_15 = [9223372036854775807_isize,9223372036854775807_isize,123_isize,113_isize,13_isize,(-9223372036854775808_isize)];
RET = 3637081423_u32;
_7 = _4;
_8 = _15;
_16 = !false;
_1 = [6_usize,3_usize,17243843847347594499_usize,2469989158964629666_usize,15137487672424984084_usize,6_usize,15851589590778400200_usize,7_usize];
_16 = RET > RET;
_13 = (-35087325655130206229554049996888000341_i128) << RET;
_5 = _12;
_5 = _14;
Goto(bb1)
}
bb1 = {
_17.0 = [(-2047491753_i32),(-1590588613_i32),1408617659_i32];
RET = 467043151_u32 ^ 684150053_u32;
RET = 3577102582_u32;
_13 = 168695019749784410208173608924836219723_i128;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_11 = [(-62_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-49_isize),122_isize,(-9223372036854775808_isize)];
_13 = 10594_i16 as i128;
_17.0 = [1878551624_i32,1928588349_i32,1781145433_i32];
_18 = [_16,_16];
_17.0 = [(-956060986_i32),985428378_i32,(-1471430348_i32)];
match RET {
0 => bb2,
3577102582 => bb4,
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
_12 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),86_isize,9223372036854775807_isize];
match RET {
0 => bb1,
1 => bb3,
3577102582 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_15 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-106_isize),(-26_isize)];
_16 = false;
_19 = 9223372036854775807_isize & 55_isize;
_11 = [_19,_19,_19,_19,_19,_19];
Call(_19 = fn7(_6, _14, _14, _10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = !1139208830_u32;
_6 = [_19,_19,_19,_19,_19,_19];
RET = 535762564_u32;
_14 = [_19,_19,_19,_19,_19,_19];
_16 = _19 != _19;
_2 = [_19,_19,_19,_19,_19,_19];
_4 = [_19,_19,_19,_19,_19,_19];
_6 = [_19,_19,_19,_19,_19,_19];
_19 = 9223372036854775807_isize;
Goto(bb8)
}
bb8 = {
_18 = [_16,_16];
_22 = '\u{a86c7}';
_21 = _13 as isize;
_24 = [1466891977_i32,(-1792533782_i32),1750074862_i32];
_17 = (_24,);
_10 = [_19,_21,_19,_19,_21,_19];
Call(_7 = fn10(_1, _2, _6, _2, _16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13 = 67359503003477598765901915143394406599_i128;
_17.0 = [2010816578_i32,(-1183357714_i32),(-968143053_i32)];
_23 = [232_u8,159_u8];
RET = 3627328236_u32 & 1787339956_u32;
_15 = [_21,_21,_21,_19,_21,_21];
_23 = [243_u8,223_u8];
_20 = _16 < _16;
_3 = [_21,_21,_19,_21,_19,_21];
_8 = [_19,_19,_19,_19,_19,_21];
RET = (-9267_i16) as u32;
_19 = !_21;
_8 = [_19,_19,_19,_19,_21,_21];
RET = 1959877692_u32 - 2284587610_u32;
_3 = [_19,_21,_21,_19,_19,_19];
_16 = _20 >= _20;
_6 = [_21,_21,_19,_21,_19,_19];
_26 = RET;
_26 = RET >> _21;
_28 = (-11063_i16) as i8;
_25 = 5794673422037394484_u64 as f64;
_23 = [192_u8,217_u8];
_25 = 146_u8 as f64;
_9 = [_19,_21,_21,_21,_19,_21];
Call(_10 = core::intrinsics::transmute(_14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_31.2 = [(-1147495127_i32),1960577475_i32,1943432722_i32];
_30 = [14756404681318150520_u64,2257260570806963243_u64,9342625975781120282_u64,7257138073214058456_u64,7891321290377196350_u64,3611370507089277263_u64];
_31.1 = [_19,_19,_21,_19,_19,_19];
_29 = _19 as u64;
_31 = (_10, _4, _17.0);
_7 = _4;
_11 = [_21,_19,_19,_21,_19,_19];
_22 = '\u{f28bf}';
_13 = -104376365754895912567003403723588924569_i128;
Call(_17 = fn12(_3, _16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _31.0;
_27 = core::ptr::addr_of_mut!(_20);
_17 = (_31.2,);
Goto(bb12)
}
bb12 = {
_8 = _31.0;
_25 = _21 as f64;
Goto(bb13)
}
bb13 = {
_26 = (-6037148353296264010_i64) as u32;
RET = _26;
_31 = (_6, _12, _24);
_10 = [_19,_21,_19,_21,_21,_21];
_3 = [_19,_21,_19,_19,_19,_21];
_22 = '\u{16194}';
_1 = [6629078644341594079_usize,4988307424151423084_usize,2_usize,1808736318364533755_usize,4_usize,14230465768239795393_usize,18360871912297945987_usize,5_usize];
_19 = 6_usize as isize;
_30 = [_29,_29,_29,_29,_29,_29];
_19 = -_21;
_31.0 = [_21,_21,_21,_21,_21,_21];
_29 = 4400937672382981451_u64;
_26 = RET ^ RET;
_36 = 82_u8 as i16;
_3 = _6;
_28 = (-116_i8);
_28 = 52_i8 << _21;
_30 = [_29,_29,_29,_29,_29,_29];
_11 = [_21,_19,_21,_21,_19,_21];
_20 = !_16;
_31.0 = [_21,_21,_19,_21,_21,_21];
_3 = [_19,_21,_19,_21,_19,_19];
(*_27) = _16 < _16;
_6 = [_21,_19,_19,_19,_21,_19];
match _29 {
0 => bb1,
1 => bb14,
2 => bb15,
3 => bb16,
4400937672382981451 => bb18,
_ => bb17
}
}
bb14 = {
_18 = [_16,_16];
_22 = '\u{a86c7}';
_21 = _13 as isize;
_24 = [1466891977_i32,(-1792533782_i32),1750074862_i32];
_17 = (_24,);
_10 = [_19,_21,_19,_19,_21,_19];
Call(_7 = fn10(_1, _2, _6, _2, _16), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_17.0 = [(-2047491753_i32),(-1590588613_i32),1408617659_i32];
RET = 467043151_u32 ^ 684150053_u32;
RET = 3577102582_u32;
_13 = 168695019749784410208173608924836219723_i128;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_11 = [(-62_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-49_isize),122_isize,(-9223372036854775808_isize)];
_13 = 10594_i16 as i128;
_17.0 = [1878551624_i32,1928588349_i32,1781145433_i32];
_18 = [_16,_16];
_17.0 = [(-956060986_i32),985428378_i32,(-1471430348_i32)];
match RET {
0 => bb2,
3577102582 => bb4,
_ => bb3
}
}
bb17 = {
_13 = 67359503003477598765901915143394406599_i128;
_17.0 = [2010816578_i32,(-1183357714_i32),(-968143053_i32)];
_23 = [232_u8,159_u8];
RET = 3627328236_u32 & 1787339956_u32;
_15 = [_21,_21,_21,_19,_21,_21];
_23 = [243_u8,223_u8];
_20 = _16 < _16;
_3 = [_21,_21,_19,_21,_19,_21];
_8 = [_19,_19,_19,_19,_19,_21];
RET = (-9267_i16) as u32;
_19 = !_21;
_8 = [_19,_19,_19,_19,_21,_21];
RET = 1959877692_u32 - 2284587610_u32;
_3 = [_19,_21,_21,_19,_19,_19];
_16 = _20 >= _20;
_6 = [_21,_21,_19,_21,_19,_19];
_26 = RET;
_26 = RET >> _21;
_28 = (-11063_i16) as i8;
_25 = 5794673422037394484_u64 as f64;
_23 = [192_u8,217_u8];
_25 = 146_u8 as f64;
_9 = [_19,_21,_21,_21,_19,_21];
Call(_10 = core::intrinsics::transmute(_14), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
_29 = 1689856390089761377_u64 * 11537283949542205131_u64;
_22 = '\u{174df}';
_21 = -_19;
_18 = [_16,(*_27)];
(*_27) = _16;
_34 = _13;
_18 = [(*_27),(*_27)];
_36 = (-3385804225770278358_i64) as i16;
_38 = Adt55::Variant1 { fld0: _25 };
_3 = [_21,_19,_21,_19,_19,_19];
place!(Field::<f64>(Variant(_38, 1), 0)) = -_25;
(*_27) = _16;
_25 = _29 as f64;
_11 = _8;
_32 = _26;
_4 = [_21,_21,_19,_19,_21,_21];
_12 = [_19,_21,_21,_19,_19,_21];
_26 = !RET;
_20 = !_16;
_11 = [_21,_19,_21,_21,_21,_21];
_19 = Field::<f64>(Variant(_38, 1), 0) as isize;
_6 = [_19,_21,_21,_21,_19,_21];
_30 = [_29,_29,_29,_29,_29,_29];
_40 = 107_u8;
Goto(bb19)
}
bb19 = {
Call(_41 = dump_var(6_usize, 11_usize, Move(_11), 36_usize, Move(_36), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_41 = dump_var(6_usize, 23_usize, Move(_23), 20_usize, Move(_20), 19_usize, Move(_19), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_41 = dump_var(6_usize, 17_usize, Move(_17), 1_usize, Move(_1), 29_usize, Move(_29), 24_usize, Move(_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(6_usize, 3_usize, Move(_3), 15_usize, Move(_15), 10_usize, Move(_10), 21_usize, Move(_21)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [isize; 6],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 6]) -> isize {
mir! {
type RET = isize;
let _5: bool;
let _6: [i16; 3];
let _7: isize;
let _8: isize;
let _9: [usize; 8];
let _10: ([i32; 3],);
let _11: ();
let _12: ();
{
RET = -(-9223372036854775808_isize);
_4 = [RET,RET,RET,RET,RET,RET];
RET = 9223372036854775807_isize;
_1 = _3;
Goto(bb1)
}
bb1 = {
_1 = [RET,RET,RET,RET,RET,RET];
RET = 9102_u16 as isize;
_6 = [(-918_i16),(-27417_i16),(-31799_i16)];
_7 = RET | RET;
_7 = RET;
_8 = RET;
_7 = (-83_i8) as isize;
_6 = [30172_i16,9657_i16,17197_i16];
_2 = _4;
Goto(bb2)
}
bb2 = {
_5 = false;
_5 = !false;
_8 = RET & _7;
_4 = [RET,_7,_8,_8,_7,_8];
_8 = !RET;
RET = _7;
RET = _8 ^ _8;
_8 = RET & _7;
RET = _8;
_6 = [(-2191_i16),1967_i16,(-22532_i16)];
Call(_7 = fn8(_3, _4, RET, _3, _3, _1, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = RET < _8;
RET = _7 * _7;
Goto(bb4)
}
bb4 = {
Call(_11 = dump_var(7_usize, 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [isize; 6],mut _2: [isize; 6],mut _3: isize,mut _4: [isize; 6],mut _5: [isize; 6],mut _6: [isize; 6],mut _7: isize) -> isize {
mir! {
type RET = isize;
let _8: Adt47;
let _9: bool;
let _10: [i32; 3];
let _11: char;
let _12: u128;
let _13: Adt49;
let _14: isize;
let _15: isize;
let _16: f64;
let _17: isize;
let _18: Adt50;
let _19: *mut bool;
let _20: (i8,);
let _21: ([isize; 6], [isize; 6], [i32; 3]);
let _22: i64;
let _23: isize;
let _24: Adt39;
let _25: isize;
let _26: Adt41;
let _27: [u8; 2];
let _28: i32;
let _29: isize;
let _30: ();
let _31: ();
{
_5 = [_7,_3,_7,_3,_3,_3];
_9 = true;
RET = _3;
_6 = [_7,_3,_3,RET,RET,RET];
_5 = _6;
_9 = false;
_1 = [RET,RET,_7,_7,RET,_3];
_10 = [(-1363313049_i32),169123454_i32,(-5055308_i32)];
RET = _7 & _7;
RET = _7 << _3;
_5 = [RET,_7,_3,_7,_7,RET];
_1 = _4;
_8 = Adt47::Variant1 { fld0: 64_u8 };
_10 = [(-107996935_i32),235694061_i32,1889257766_i32];
_3 = RET;
RET = _3 * _3;
RET = _3;
_11 = '\u{25656}';
place!(Field::<u8>(Variant(_8, 1), 0)) = 125_u8;
_4 = [RET,_3,_3,RET,RET,RET];
Goto(bb1)
}
bb1 = {
RET = -_7;
_3 = !_7;
_4 = _5;
_2 = [RET,RET,_7,_3,_7,RET];
_11 = '\u{310b2}';
_2 = _1;
place!(Field::<u8>(Variant(_8, 1), 0)) = !61_u8;
_1 = [_7,_3,_7,RET,_7,_3];
SetDiscriminant(_8, 1);
_8 = Adt47::Variant1 { fld0: 174_u8 };
RET = _3 | _7;
place!(Field::<u8>(Variant(_8, 1), 0)) = !81_u8;
_10 = [1075993210_i32,(-176388012_i32),(-1661238221_i32)];
_7 = !RET;
_10 = [2061354822_i32,2037537870_i32,867074252_i32];
_10 = [(-1619541870_i32),(-314934301_i32),(-1536434148_i32)];
_5 = [_7,_7,_7,RET,RET,_7];
_10 = [1703873092_i32,(-1958796446_i32),(-1888927296_i32)];
_9 = RET >= _7;
_13.fld1 = [2141984999_i32,1574026313_i32,159438766_i32];
_4 = [_7,_7,RET,RET,RET,RET];
_8 = Adt47::Variant1 { fld0: 207_u8 };
_9 = false;
_13.fld1 = [924619719_i32,(-1493770257_i32),1940230644_i32];
_12 = 2798807763_u32 as u128;
_2 = _4;
Call(_6 = fn9(RET, RET, _4, RET, _7, _5, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13.fld0.1 = [RET,_7,RET,RET,_7,RET];
_6 = [RET,_7,RET,_7,RET,_3];
_13.fld2 = -_7;
_12 = 47785298832056446582999249820237897174_u128;
_13.fld0 = (_6, _2, _13.fld1);
_13.fld0 = (_4, _5, _10);
_15 = !_3;
_16 = 85388467539008519113100226113424475251_i128 as f64;
RET = !_7;
_9 = true;
_10 = [563531984_i32,691206833_i32,641944175_i32];
_12 = 145742637387685162827690138968858375198_u128 ^ 60594615398616706647803517616293593878_u128;
_4 = [_7,_7,RET,_3,_13.fld2,_15];
_16 = (-85683539035080576081825405309690561928_i128) as f64;
place!(Field::<u8>(Variant(_8, 1), 0)) = !2_u8;
_11 = '\u{e6539}';
_15 = -_7;
_12 = 143082916660529542129377723685125651663_i128 as u128;
_1 = [_7,RET,_13.fld2,_15,RET,_15];
Goto(bb3)
}
bb3 = {
_1 = [_7,_15,RET,_3,_13.fld2,_13.fld2];
_11 = '\u{7d267}';
SetDiscriminant(_8, 2);
_13.fld0 = (_1, _5, _10);
_17 = _7 >> RET;
_2 = _1;
_13.fld0.1 = [_7,_3,_17,_15,_15,RET];
_13.fld0 = (_6, _5, _13.fld1);
place!(Field::<u8>(Variant(_8, 2), 2)) = 192_u8 >> _15;
_12 = !195962967191890636605359749427427106483_u128;
Goto(bb4)
}
bb4 = {
_13.fld1 = [(-953828039_i32),43872934_i32,535104216_i32];
_2 = [_15,RET,RET,_13.fld2,_17,_15];
_3 = _17;
_7 = 4481212896977598633_i64 as isize;
_2 = [_13.fld2,_13.fld2,_17,_3,_7,_15];
_14 = !_17;
Goto(bb5)
}
bb5 = {
_17 = _3;
_19 = core::ptr::addr_of_mut!(_9);
_9 = !false;
_2 = [_17,_17,RET,_14,_3,_7];
_13.fld1 = [1807203875_i32,(-90889005_i32),(-1557807182_i32)];
_11 = '\u{ef88e}';
_12 = 253110966248352229959903155369721357032_u128 | 287502251307280486547739247505137144188_u128;
_10 = [(-1081354148_i32),(-232876870_i32),1316433215_i32];
_17 = 1594194036_i32 as isize;
Goto(bb6)
}
bb6 = {
_5 = [_15,_14,_3,RET,_15,_15];
_16 = (-13_i8) as f64;
_5 = [_15,_15,RET,_14,_13.fld2,_14];
_13.fld0 = (_5, _2, _13.fld1);
_17 = _11 as isize;
_20 = ((-106_i8),);
_5 = [_15,_3,_13.fld2,_14,_14,_3];
_9 = false;
RET = -_13.fld2;
_13.fld0.2 = _13.fld1;
_19 = core::ptr::addr_of_mut!(_9);
_10 = [16882576_i32,(-1695066537_i32),(-1725782775_i32)];
_13.fld0 = (_2, _2, _13.fld1);
_13.fld0 = (_5, _5, _13.fld1);
_3 = (*_19) as isize;
_23 = _14;
_3 = _23;
_8 = Adt47::Variant1 { fld0: 19_u8 };
_11 = '\u{10c447}';
_13.fld0.2 = [977246089_i32,1382404085_i32,(-274367860_i32)];
place!(Field::<u8>(Variant(_8, 1), 0)) = 242_u8;
_13.fld1 = [(-1271272379_i32),392635953_i32,1964394618_i32];
(*_19) = _7 != _17;
match _20.0 {
0 => bb5,
340282366920938463463374607431768211350 => bb7,
_ => bb2
}
}
bb7 = {
(*_19) = !false;
_8 = Adt47::Variant1 { fld0: 109_u8 };
_8 = Adt47::Variant1 { fld0: 65_u8 };
_13.fld0.1 = [_14,_14,_3,_23,_3,_3];
_6 = [_14,_3,_14,_23,_15,_14];
_13.fld0.0 = _1;
_19 = core::ptr::addr_of_mut!(_9);
_21 = _13.fld0;
_13 = Adt49 { fld0: _21,fld1: _10,fld2: _15 };
_3 = _12 as isize;
_5 = [_14,_14,_3,_23,_14,_23];
_13.fld1 = [165417359_i32,(-964744404_i32),(-163692809_i32)];
RET = _13.fld2;
_13.fld0.0 = [_23,RET,_14,RET,_14,RET];
_21.1 = _13.fld0.1;
_22 = -811125353540724616_i64;
_13 = Adt49 { fld0: _21,fld1: _10,fld2: _14 };
_28 = 14239_i16 as i32;
_11 = '\u{cdc5c}';
_16 = 227_u8 as f64;
_7 = _17 >> RET;
match _20.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211350 => bb8,
_ => bb5
}
}
bb8 = {
_21.1 = [_13.fld2,_14,_13.fld2,RET,_23,RET];
_13.fld0.1 = [_15,_15,_14,_14,_3,_13.fld2];
match _20.0 {
0 => bb5,
1 => bb7,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768211350 => bb15,
_ => bb14
}
}
bb9 = {
(*_19) = !false;
_8 = Adt47::Variant1 { fld0: 109_u8 };
_8 = Adt47::Variant1 { fld0: 65_u8 };
_13.fld0.1 = [_14,_14,_3,_23,_3,_3];
_6 = [_14,_3,_14,_23,_15,_14];
_13.fld0.0 = _1;
_19 = core::ptr::addr_of_mut!(_9);
_21 = _13.fld0;
_13 = Adt49 { fld0: _21,fld1: _10,fld2: _15 };
_3 = _12 as isize;
_5 = [_14,_14,_3,_23,_14,_23];
_13.fld1 = [165417359_i32,(-964744404_i32),(-163692809_i32)];
RET = _13.fld2;
_13.fld0.0 = [_23,RET,_14,RET,_14,RET];
_21.1 = _13.fld0.1;
_22 = -811125353540724616_i64;
_13 = Adt49 { fld0: _21,fld1: _10,fld2: _14 };
_28 = 14239_i16 as i32;
_11 = '\u{cdc5c}';
_16 = 227_u8 as f64;
_7 = _17 >> RET;
match _20.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211350 => bb8,
_ => bb5
}
}
bb10 = {
_5 = [_15,_14,_3,RET,_15,_15];
_16 = (-13_i8) as f64;
_5 = [_15,_15,RET,_14,_13.fld2,_14];
_13.fld0 = (_5, _2, _13.fld1);
_17 = _11 as isize;
_20 = ((-106_i8),);
_5 = [_15,_3,_13.fld2,_14,_14,_3];
_9 = false;
RET = -_13.fld2;
_13.fld0.2 = _13.fld1;
_19 = core::ptr::addr_of_mut!(_9);
_10 = [16882576_i32,(-1695066537_i32),(-1725782775_i32)];
_13.fld0 = (_2, _2, _13.fld1);
_13.fld0 = (_5, _5, _13.fld1);
_3 = (*_19) as isize;
_23 = _14;
_3 = _23;
_8 = Adt47::Variant1 { fld0: 19_u8 };
_11 = '\u{10c447}';
_13.fld0.2 = [977246089_i32,1382404085_i32,(-274367860_i32)];
place!(Field::<u8>(Variant(_8, 1), 0)) = 242_u8;
_13.fld1 = [(-1271272379_i32),392635953_i32,1964394618_i32];
(*_19) = _7 != _17;
match _20.0 {
0 => bb5,
340282366920938463463374607431768211350 => bb7,
_ => bb2
}
}
bb11 = {
_17 = _3;
_19 = core::ptr::addr_of_mut!(_9);
_9 = !false;
_2 = [_17,_17,RET,_14,_3,_7];
_13.fld1 = [1807203875_i32,(-90889005_i32),(-1557807182_i32)];
_11 = '\u{ef88e}';
_12 = 253110966248352229959903155369721357032_u128 | 287502251307280486547739247505137144188_u128;
_10 = [(-1081354148_i32),(-232876870_i32),1316433215_i32];
_17 = 1594194036_i32 as isize;
Goto(bb6)
}
bb12 = {
_13.fld1 = [(-953828039_i32),43872934_i32,535104216_i32];
_2 = [_15,RET,RET,_13.fld2,_17,_15];
_3 = _17;
_7 = 4481212896977598633_i64 as isize;
_2 = [_13.fld2,_13.fld2,_17,_3,_7,_15];
_14 = !_17;
Goto(bb5)
}
bb13 = {
_1 = [_7,_15,RET,_3,_13.fld2,_13.fld2];
_11 = '\u{7d267}';
SetDiscriminant(_8, 2);
_13.fld0 = (_1, _5, _10);
_17 = _7 >> RET;
_2 = _1;
_13.fld0.1 = [_7,_3,_17,_15,_15,RET];
_13.fld0 = (_6, _5, _13.fld1);
place!(Field::<u8>(Variant(_8, 2), 2)) = 192_u8 >> _15;
_12 = !195962967191890636605359749427427106483_u128;
Goto(bb4)
}
bb14 = {
RET = -_7;
_3 = !_7;
_4 = _5;
_2 = [RET,RET,_7,_3,_7,RET];
_11 = '\u{310b2}';
_2 = _1;
place!(Field::<u8>(Variant(_8, 1), 0)) = !61_u8;
_1 = [_7,_3,_7,RET,_7,_3];
SetDiscriminant(_8, 1);
_8 = Adt47::Variant1 { fld0: 174_u8 };
RET = _3 | _7;
place!(Field::<u8>(Variant(_8, 1), 0)) = !81_u8;
_10 = [1075993210_i32,(-176388012_i32),(-1661238221_i32)];
_7 = !RET;
_10 = [2061354822_i32,2037537870_i32,867074252_i32];
_10 = [(-1619541870_i32),(-314934301_i32),(-1536434148_i32)];
_5 = [_7,_7,_7,RET,RET,_7];
_10 = [1703873092_i32,(-1958796446_i32),(-1888927296_i32)];
_9 = RET >= _7;
_13.fld1 = [2141984999_i32,1574026313_i32,159438766_i32];
_4 = [_7,_7,RET,RET,RET,RET];
_8 = Adt47::Variant1 { fld0: 207_u8 };
_9 = false;
_13.fld1 = [924619719_i32,(-1493770257_i32),1940230644_i32];
_12 = 2798807763_u32 as u128;
_2 = _4;
Call(_6 = fn9(RET, RET, _4, RET, _7, _5, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
(*_19) = !false;
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(8_usize, 17_usize, Move(_17), 6_usize, Move(_6), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(8_usize, 7_usize, Move(_7), 9_usize, Move(_9), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(8_usize, 21_usize, Move(_21), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: [isize; 6],mut _4: isize,mut _5: isize,mut _6: [isize; 6],mut _7: isize) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _8: ();
let _9: ();
{
_3 = [_7,_2,_4,_1,_7,_5];
_5 = _2 << _2;
Goto(bb1)
}
bb1 = {
RET = [_5,_5,_1,_1,_5,_5];
_7 = !_4;
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(9_usize, 2_usize, Move(_2), 3_usize, Move(_3), 1_usize, Move(_1), 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [usize; 8],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 6],mut _5: bool) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _6: Adt53;
let _7: [bool; 2];
let _8: ([i32; 3],);
let _9: f32;
let _10: char;
let _11: f64;
let _12: ([i32; 3],);
let _13: [u8; 2];
let _14: Adt41;
let _15: i16;
let _16: i8;
let _17: bool;
let _18: bool;
let _19: f32;
let _20: isize;
let _21: usize;
let _22: isize;
let _23: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _24: Adt49;
let _25: ();
let _26: ();
{
_2 = [9223372036854775807_isize,16_isize,(-9_isize),(-2_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _4;
_3 = [(-9223372036854775808_isize),(-116_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),96_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = [9223372036854775807_isize,(-32_isize),(-105_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_4 = _2;
_2 = _4;
Goto(bb1)
}
bb1 = {
RET = [1_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize)];
_7 = [_5,_5];
_1 = [5_usize,4_usize,6_usize,6_usize,6_usize,2_usize,16940499815347103407_usize,2_usize];
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-127_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _4;
_9 = 11415264039807425274_u64 as f32;
_7 = [_5,_5];
_10 = '\u{f4da3}';
_8.0 = [(-65427527_i32),(-40805419_i32),1597206176_i32];
_4 = RET;
_5 = _9 > _9;
Goto(bb2)
}
bb2 = {
_3 = _4;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,20_isize,(-48_isize),9223372036854775807_isize,9223372036854775807_isize];
_8.0 = [(-908901602_i32),(-999354762_i32),(-1593843081_i32)];
RET = _4;
_8.0 = [(-508842064_i32),(-588892054_i32),2106781074_i32];
_12.0 = [1747100219_i32,(-7506013_i32),1324531296_i32];
_13 = [210_u8,34_u8];
RET = [9223372036854775807_isize,83_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-25_isize),9223372036854775807_isize];
_10 = '\u{c37e1}';
_10 = '\u{ffc00}';
_13 = [85_u8,33_u8];
_8.0 = [1720193786_i32,940782574_i32,(-1731389822_i32)];
_1 = [1082139997795142714_usize,3_usize,5_usize,4_usize,14393438933442463772_usize,4265064515446841138_usize,4_usize,5_usize];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-100_isize),9223372036854775807_isize];
_5 = false;
_12 = _8;
RET = [9223372036854775807_isize,(-9223372036854775808_isize),35_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = (-65_i8) as f64;
_8.0 = [(-643470107_i32),(-1921645689_i32),(-1980291867_i32)];
_12 = (_8.0,);
_11 = 20419_i16 as f64;
Call(_7 = fn11(_3, _2, _13, _2, _11, _5, RET, _3, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12.0 = [1371240677_i32,(-1033621694_i32),(-1152386791_i32)];
_10 = '\u{cd5d4}';
_8.0 = _12.0;
_13 = [159_u8,8_u8];
_5 = true;
RET = [(-57_isize),(-9223372036854775808_isize),62_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_15 = !28359_i16;
_16 = (-1052989964_i32) as i8;
_4 = _3;
_9 = 38728957754213113512335995008489977840_i128 as f32;
_12 = (_8.0,);
_5 = _9 != _9;
_8 = _12;
_9 = 13434770286418367408_usize as f32;
_15 = 18875_i16 & (-29922_i16);
_10 = '\u{855dd}';
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-82_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_16 = !(-74_i8);
_16 = !79_i8;
RET = _3;
_16 = 2_i8;
_17 = _5;
_10 = '\u{32577}';
_16 = 122_i8 | (-93_i8);
_12.0 = [917493263_i32,1720113172_i32,(-944064175_i32)];
_10 = '\u{d239b}';
_5 = !_17;
Goto(bb5)
}
bb5 = {
RET = [(-9223372036854775808_isize),87_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = (_12.0,);
_18 = _17 <= _5;
_18 = _17 >= _5;
_8.0 = [(-1120843490_i32),(-1799080315_i32),(-1139080289_i32)];
_7 = [_18,_18];
_18 = _5 ^ _5;
_10 = '\u{4b98b}';
_20 = (-9223372036854775808_isize) - (-11_isize);
_21 = 5_usize;
_2[_21] = !_4[_21];
_12 = _8;
match _4[_21] {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
9223372036854775807 => bb13,
_ => bb12
}
}
bb6 = {
_12.0 = [1371240677_i32,(-1033621694_i32),(-1152386791_i32)];
_10 = '\u{cd5d4}';
_8.0 = _12.0;
_13 = [159_u8,8_u8];
_5 = true;
RET = [(-57_isize),(-9223372036854775808_isize),62_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_15 = !28359_i16;
_16 = (-1052989964_i32) as i8;
_4 = _3;
_9 = 38728957754213113512335995008489977840_i128 as f32;
_12 = (_8.0,);
_5 = _9 != _9;
_8 = _12;
_9 = 13434770286418367408_usize as f32;
_15 = 18875_i16 & (-29922_i16);
_10 = '\u{855dd}';
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-82_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_16 = !(-74_i8);
_16 = !79_i8;
RET = _3;
_16 = 2_i8;
_17 = _5;
_10 = '\u{32577}';
_16 = 122_i8 | (-93_i8);
_12.0 = [917493263_i32,1720113172_i32,(-944064175_i32)];
_10 = '\u{d239b}';
_5 = !_17;
Goto(bb5)
}
bb7 = {
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_3 = _4;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,20_isize,(-48_isize),9223372036854775807_isize,9223372036854775807_isize];
_8.0 = [(-908901602_i32),(-999354762_i32),(-1593843081_i32)];
RET = _4;
_8.0 = [(-508842064_i32),(-588892054_i32),2106781074_i32];
_12.0 = [1747100219_i32,(-7506013_i32),1324531296_i32];
_13 = [210_u8,34_u8];
RET = [9223372036854775807_isize,83_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-25_isize),9223372036854775807_isize];
_10 = '\u{c37e1}';
_10 = '\u{ffc00}';
_13 = [85_u8,33_u8];
_8.0 = [1720193786_i32,940782574_i32,(-1731389822_i32)];
_1 = [1082139997795142714_usize,3_usize,5_usize,4_usize,14393438933442463772_usize,4265064515446841138_usize,4_usize,5_usize];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-100_isize),9223372036854775807_isize];
_5 = false;
_12 = _8;
RET = [9223372036854775807_isize,(-9223372036854775808_isize),35_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = (-65_i8) as f64;
_8.0 = [(-643470107_i32),(-1921645689_i32),(-1980291867_i32)];
_12 = (_8.0,);
_11 = 20419_i16 as f64;
Call(_7 = fn11(_3, _2, _13, _2, _11, _5, RET, _3, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = [1_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize)];
_7 = [_5,_5];
_1 = [5_usize,4_usize,6_usize,6_usize,6_usize,2_usize,16940499815347103407_usize,2_usize];
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-127_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _4;
_9 = 11415264039807425274_u64 as f32;
_7 = [_5,_5];
_10 = '\u{f4da3}';
_8.0 = [(-65427527_i32),(-40805419_i32),1597206176_i32];
_4 = RET;
_5 = _9 > _9;
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
_3[_21] = RET[_21];
_8.0 = [176136616_i32,(-1685558302_i32),1185572906_i32];
_19 = -_9;
RET = [_3[_21],_3[_21],_2[_21],_2[_21],_3[_21],_20];
_19 = -_9;
_3[_21] = _1[_21] as isize;
_3[_21] = _15 as isize;
_8.0 = [1065249617_i32,(-500886708_i32),951214031_i32];
RET = _3;
_8 = (_12.0,);
_4[_21] = 152129027795035168281033399122579671652_i128 as isize;
_23.6.0 = !6614573642893184533_u64;
_3 = [_20,_2[_21],_20,_4[_21],_2[_21],RET[_21]];
RET = _2;
_3[_21] = _18 as isize;
RET[_21] = !_2[_21];
_12 = (_8.0,);
_23.4.0[_21] = _3[_21] * RET[_21];
_23.0 = [RET[_21],RET[_21]];
_23.2.1 = _23.4.0[_21];
RET = _2;
Goto(bb14)
}
bb14 = {
_20 = -_2[_21];
_18 = _17;
_23.2.0 = -_11;
_23.3 = [49478358768941795447333856449940825474_u128,185737743428968434442930343440484545362_u128,257832021301722710004988923217995728271_u128,85759948511910147358324337425174936228_u128,241220913117732434280965500363444062026_u128,333964288008112299117370540472817411795_u128];
_24.fld0.1[_21] = 85_u8 as isize;
_23.4 = (_2,);
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(10_usize, 13_usize, Move(_13), 20_usize, Move(_20), 4_usize, Move(_4), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(10_usize, 18_usize, Move(_18), 1_usize, Move(_1), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 6],mut _2: [isize; 6],mut _3: [u8; 2],mut _4: [isize; 6],mut _5: f64,mut _6: bool,mut _7: [isize; 6],mut _8: [isize; 6],mut _9: [isize; 6]) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _10: isize;
let _11: [u64; 6];
let _12: char;
let _13: [isize; 2];
let _14: u128;
let _15: (f64, isize);
let _16: *const [i16; 3];
let _17: i32;
let _18: usize;
let _19: Adt44;
let _20: Adt47;
let _21: *const ([i32; 3],);
let _22: isize;
let _23: Adt39;
let _24: [u8; 2];
let _25: f64;
let _26: u128;
let _27: Adt44;
let _28: f64;
let _29: bool;
let _30: isize;
let _31: *const [i16; 3];
let _32: isize;
let _33: Adt53;
let _34: i64;
let _35: usize;
let _36: ();
let _37: ();
{
_10 = (-9223372036854775808_isize);
_3 = [57_u8,98_u8];
_1 = _9;
_8 = [_10,_10,_10,_10,_10,_10];
_7 = [_10,_10,_10,_10,_10,_10];
_6 = !true;
_4 = _9;
_13 = [_10,_10];
_15 = (_5, _10);
_10 = (-8714659956485516863_i64) as isize;
_8 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_15.0 = (-1715543884_i32) as f64;
_4 = [_10,_15.1,_15.1,_10,_10,_10];
_14 = (-667076608_i32) as u128;
_11 = [12999492366497919969_u64,5451727894886305088_u64,3387898261183978241_u64,1596169961930640_u64,1513135366682957152_u64,9006509236348468044_u64];
_5 = -_15.0;
_12 = '\u{12545}';
_14 = 206456656040396968190504032361951761003_u128;
match _15.1 {
340282366920938463454151235394913435648 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_14 = _6 as u128;
_5 = _15.0;
RET = [_6,_6];
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_18 = 2_usize + 6_usize;
_2 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_1 = [_10,_10,_10,_15.1,_15.1,_10];
RET = [_6,_6];
_5 = _15.0;
_3 = [180_u8,236_u8];
_12 = '\u{45a21}';
_7 = [_15.1,_10,_15.1,_15.1,_15.1,_15.1];
_15.1 = _10;
RET = [_6,_6];
Goto(bb3)
}
bb3 = {
_15 = (_5, _10);
_2 = [_15.1,_10,_10,_15.1,_15.1,_15.1];
_2 = [_15.1,_15.1,_10,_15.1,_10,_15.1];
_12 = '\u{77025}';
_12 = '\u{1046c1}';
_15.1 = _10;
RET = [_6,_6];
_2 = _1;
_3 = [148_u8,18_u8];
_17 = 754897636_i32;
_4 = [_10,_10,_15.1,_10,_15.1,_15.1];
_10 = _15.1 * _15.1;
_6 = _15.0 != _5;
match _17 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
754897636 => bb10,
_ => bb9
}
}
bb4 = {
_14 = _6 as u128;
_5 = _15.0;
RET = [_6,_6];
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_18 = 2_usize + 6_usize;
_2 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_1 = [_10,_10,_10,_15.1,_15.1,_10];
RET = [_6,_6];
_5 = _15.0;
_3 = [180_u8,236_u8];
_12 = '\u{45a21}';
_7 = [_15.1,_10,_15.1,_15.1,_15.1,_15.1];
_15.1 = _10;
RET = [_6,_6];
Goto(bb3)
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
_15.1 = _10 ^ _10;
_12 = '\u{20c74}';
RET = [_6,_6];
_24 = [89_u8,31_u8];
RET = [_6,_6];
_22 = _10;
_14 = 106541473992088049839947145923870976897_u128 & 42497657729977486944260937462148638491_u128;
_25 = -_5;
_11 = [8409568004903738953_u64,11515580124300031391_u64,4119963820561716920_u64,4620161835796750043_u64,10288152891726778544_u64,3398028535959614089_u64];
_15.1 = _10 >> _18;
_26 = _14 | _14;
_1 = _9;
_6 = _25 != _5;
_11 = [11056363913063594232_u64,2784433566690773321_u64,10468241288505635882_u64,11727136336986044950_u64,3990697735078054636_u64,13962959858798757991_u64];
Goto(bb11)
}
bb11 = {
_7 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_17 = (-1896340847_i32);
_10 = _15.1 - _22;
_24 = _3;
_22 = -_15.1;
_8 = [_10,_22,_10,_22,_10,_15.1];
_28 = _5;
_7 = _8;
_29 = !_6;
_29 = !_6;
_5 = _28 - _15.0;
_20 = Adt47::Variant1 { fld0: 120_u8 };
_15 = (_5, _10);
_25 = _14 as f64;
_26 = 508909678_u32 as u128;
_15.1 = _12 as isize;
_15.0 = _25 * _25;
_7 = [_10,_15.1,_10,_10,_22,_10];
_3 = [198_u8,92_u8];
_20 = Adt47::Variant1 { fld0: 36_u8 };
match _17 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463463374607429871870609 => bb19,
_ => bb18
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
Return()
}
bb17 = {
_14 = _6 as u128;
_5 = _15.0;
RET = [_6,_6];
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_18 = 2_usize + 6_usize;
_2 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_1 = [_10,_10,_10,_15.1,_15.1,_10];
RET = [_6,_6];
_5 = _15.0;
_3 = [180_u8,236_u8];
_12 = '\u{45a21}';
_7 = [_15.1,_10,_15.1,_15.1,_15.1,_15.1];
_15.1 = _10;
RET = [_6,_6];
Goto(bb3)
}
bb18 = {
_14 = _6 as u128;
_5 = _15.0;
RET = [_6,_6];
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_18 = 2_usize + 6_usize;
_2 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_1 = [_10,_10,_10,_15.1,_15.1,_10];
RET = [_6,_6];
_5 = _15.0;
_3 = [180_u8,236_u8];
_12 = '\u{45a21}';
_7 = [_15.1,_10,_15.1,_15.1,_15.1,_15.1];
_15.1 = _10;
RET = [_6,_6];
Goto(bb3)
}
bb19 = {
_6 = _5 != _5;
_18 = 12015034323472223550_usize;
_20 = Adt47::Variant1 { fld0: 186_u8 };
_15 = (_5, _22);
_5 = _17 as f64;
place!(Field::<u8>(Variant(_20, 1), 0)) = 94_u8;
_7 = [_22,_15.1,_22,_15.1,_22,_22];
_8 = _7;
_32 = _15.0 as isize;
place!(Field::<u8>(Variant(_20, 1), 0)) = 8940826285038523450_u64 as u8;
_18 = 921071695_u32 as usize;
RET = [_6,_29];
SetDiscriminant(_20, 1);
_4 = _8;
_25 = _15.0 - _15.0;
_11 = [17956371218461791725_u64,16450895615196295451_u64,17939727869517674605_u64,8481355977088546041_u64,9126883396662070380_u64,18171527575301550026_u64];
_1 = [_22,_15.1,_32,_10,_15.1,_10];
_3 = [13_u8,206_u8];
_12 = '\u{9dc1f}';
RET = [_29,_6];
_30 = _14 as isize;
_13 = [_15.1,_30];
_11 = [7267830693535621742_u64,15547644146753951978_u64,3071340406816203222_u64,9440335550936373318_u64,2882767326326250460_u64,6001470946915247513_u64];
_4 = [_10,_32,_10,_10,_15.1,_22];
_15.0 = 3092740895_u32 as f64;
_8 = _1;
_34 = -(-3014043929376140411_i64);
Goto(bb20)
}
bb20 = {
Call(_36 = dump_var(11_usize, 11_usize, Move(_11), 1_usize, Move(_1), 18_usize, Move(_18), 34_usize, Move(_34)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_36 = dump_var(11_usize, 22_usize, Move(_22), 24_usize, Move(_24), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_36 = dump_var(11_usize, 12_usize, Move(_12), 3_usize, Move(_3), 6_usize, Move(_6), 37_usize, _37), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [isize; 6],mut _2: bool) -> ([i32; 3],) {
mir! {
type RET = ([i32; 3],);
let _3: Adt49;
let _4: Adt52;
let _5: bool;
let _6: [isize; 2];
let _7: ([isize; 6], [isize; 6], [i32; 3]);
let _8: ([i32; 3],);
let _9: [u64; 6];
let _10: [isize; 2];
let _11: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _12: [isize; 6];
let _13: Adt50;
let _14: Adt54;
let _15: Adt40;
let _16: i16;
let _17: f64;
let _18: Adt39;
let _19: isize;
let _20: i16;
let _21: (f64, isize);
let _22: i8;
let _23: u64;
let _24: &'static f32;
let _25: i64;
let _26: i32;
let _27: bool;
let _28: ([i32; 3],);
let _29: &'static f32;
let _30: [usize; 8];
let _31: [isize; 6];
let _32: Adt49;
let _33: u32;
let _34: Adt55;
let _35: i8;
let _36: char;
let _37: *const [i16; 3];
let _38: ();
let _39: ();
{
RET.0 = [1432289390_i32,1167877161_i32,1239199566_i32];
_1 = [(-27_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),92_isize,(-9223372036854775808_isize)];
_1 = [76_isize,43_isize,7_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-98_isize)];
RET.0 = [(-188112329_i32),(-783820221_i32),(-805218048_i32)];
Goto(bb1)
}
bb1 = {
_2 = 2502909096_u32 <= 1023163923_u32;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-68_isize),125_isize];
_1 = [9223372036854775807_isize,(-72_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET.0 = [(-146931678_i32),660605433_i32,444683623_i32];
Goto(bb2)
}
bb2 = {
RET.0 = [(-769645877_i32),1161556051_i32,1860078427_i32];
_4.fld1 = (_1,);
_4.fld2.fld2 = 32_i8 as isize;
_4.fld4 = [210698390582076370050101548614064197054_u128,268651578115895251089489817737829077452_u128,30855361480372556730234973026678102318_u128,100713351977821368133690742985744611162_u128,175825788728194169215108551018193969145_u128,249963156176916526990527944558764456153_u128];
_4.fld2.fld2 = -(-9223372036854775808_isize);
_4.fld2.fld1 = [886568540_i32,(-406647556_i32),(-1371163191_i32)];
RET.0 = _4.fld2.fld1;
_3.fld2 = 2094666528_u32 as isize;
_3.fld1 = RET.0;
_6 = [_3.fld2,_4.fld2.fld2];
_4.fld1 = (_1,);
_7.2 = [(-1270292984_i32),969609926_i32,(-1037170566_i32)];
_3.fld0.2 = [(-1631538816_i32),(-1340606504_i32),954328700_i32];
_3.fld0.1 = [_3.fld2,_3.fld2,_4.fld2.fld2,_3.fld2,_4.fld2.fld2,_3.fld2];
_4.fld2.fld0.2 = [(-1685103262_i32),(-1214546177_i32),(-841177061_i32)];
Goto(bb3)
}
bb3 = {
_8 = (_3.fld0.2,);
_5 = _2;
_11.6.4 = _5 as i128;
_9 = [1352131547131018455_u64,2346538902146449471_u64,3220406836728647077_u64,899263627875507407_u64,18029498572767522159_u64,4105701812905225411_u64];
_4.fld2.fld0.1 = [_3.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2,_3.fld2,_3.fld2];
_4.fld1.0 = [_4.fld2.fld2,_3.fld2,_3.fld2,_3.fld2,_4.fld2.fld2,_3.fld2];
_5 = _2 != _2;
_6 = [_3.fld2,_4.fld2.fld2];
_11.2.0 = 1578809382_i32 as f64;
_11.6.2 = _11.6.4 | _11.6.4;
_4.fld1.0 = _3.fld0.1;
_11.6 = (11729373078515279336_u64, 31036_i16, 63735722649894342257218427980078257119_i128, _11.2.0, 71543016008872890774166114355504712535_i128);
_3.fld0.2 = [(-1095410628_i32),(-1002985789_i32),(-1589306977_i32)];
_11.4.0 = [_4.fld2.fld2,_3.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2];
_11.4 = (_1,);
_11.2 = (_11.6.3, _3.fld2);
Call(_11.6.3 = core::intrinsics::fmaf64(_11.2.0, _11.2.0, _11.2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.4 = (_1,);
_3.fld0.0 = [_3.fld2,_3.fld2,_3.fld2,_11.2.1,_11.2.1,_11.2.1];
_4.fld2.fld1 = [978831504_i32,677924607_i32,(-1267233977_i32)];
_3.fld0.0 = _1;
_4.fld2.fld1 = [1428404185_i32,967435103_i32,(-177483010_i32)];
Call(_17 = core::intrinsics::fmaf64(_11.6.3, _11.2.0, _11.6.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4.fld2.fld0.2 = [11812029_i32,1469892602_i32,587115879_i32];
_3.fld2 = -_4.fld2.fld2;
_11.6.0 = _11.2.1 as u64;
_3.fld0 = (_11.4.0, _11.4.0, _4.fld2.fld1);
_11.2.0 = _11.6.3 * _11.6.3;
_11.6.1 = !(-14600_i16);
_11.1 = !101189915995644276867721644559463493254_u128;
_7.1 = _1;
Goto(bb6)
}
bb6 = {
_11.3 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_8.0 = RET.0;
_15 = Adt40::Variant1 { fld0: 24198_u16,fld1: _11.6.0,fld2: _11.4.0,fld3: 2_usize,fld4: _4.fld1,fld5: _6 };
_4.fld2.fld1 = [777547814_i32,1330651840_i32,1015023625_i32];
place!(Field::<([isize; 6],)>(Variant(_15, 1), 4)) = _4.fld1;
_14 = Adt54::Variant2 { fld0: _3.fld2 };
_11.3 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_4.fld5 = -_11.6.4;
_11.5 = _9;
_11.6.0 = _11.6.1 as u64;
place!(Field::<([isize; 6],)>(Variant(_15, 1), 4)) = (_3.fld0.0,);
_12 = [_11.2.1,_4.fld2.fld2,_3.fld2,_4.fld2.fld2,_11.2.1,_11.2.1];
_4.fld4 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_2 = !_5;
_20 = _11.6.1;
_4.fld2.fld0 = _3.fld0;
_11.6.3 = -_11.2.0;
_11.6.4 = -_4.fld5;
place!(Field::<isize>(Variant(_14, 2), 0)) = (-1576397044_i32) as isize;
_11.6.4 = _4.fld5 * _4.fld5;
_10 = [_3.fld2,Field::<isize>(Variant(_14, 2), 0)];
RET = (_3.fld1,);
_3.fld0.0 = _1;
_11.6.0 = Field::<u64>(Variant(_15, 1), 1);
_17 = _11.6.3 + _11.2.0;
_5 = !_2;
_12 = _4.fld2.fld0.0;
_11.4 = _4.fld1;
Call(_11 = fn13(_4.fld2.fld0, _4.fld2, _12, _12, _4.fld2.fld0, _1, _3.fld2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_21.1 = _3.fld2 >> _4.fld5;
_11.5 = [Field::<u64>(Variant(_15, 1), 1),Field::<u64>(Variant(_15, 1), 1),_11.6.0,_11.6.0,Field::<u64>(Variant(_15, 1), 1),_11.6.0];
_8.0 = _7.2;
_19 = _21.1 + _21.1;
_11.4 = (Field::<[isize; 6]>(Variant(_15, 1), 2),);
_21 = (_11.2.0, _19);
_17 = _11.1 as f64;
_11.2.0 = _17;
_3.fld2 = !_21.1;
_15 = Adt40::Variant1 { fld0: 56130_u16,fld1: _11.6.0,fld2: _7.1,fld3: 16823938003680444984_usize,fld4: _11.4,fld5: _11.0 };
_11.4.0 = _4.fld2.fld0.1;
_11.6.4 = _4.fld5 * _4.fld5;
_11.0 = [_21.1,_19];
_19 = _3.fld2 << _11.6.2;
place!(Field::<u16>(Variant(_15, 1), 0)) = 61140_u16 - 52944_u16;
_4.fld2.fld0.1 = [_4.fld2.fld2,_21.1,_19,_3.fld2,_3.fld2,_19];
Goto(bb8)
}
bb8 = {
_25 = -(-2438723538611008020_i64);
_4.fld1 = (_7.1,);
_7.2 = RET.0;
_3.fld2 = !_19;
_5 = _17 >= _17;
_16 = -_20;
_19 = _11.6.1 as isize;
Goto(bb9)
}
bb9 = {
_4.fld2.fld0.2 = _3.fld1;
_4.fld1 = (_12,);
place!(Field::<u16>(Variant(_15, 1), 0)) = 5574_u16 - 24760_u16;
place!(Field::<u64>(Variant(_15, 1), 1)) = _11.6.0;
SetDiscriminant(_14, 0);
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_14, 0), 1)).3 = -_11.2.0;
Goto(bb10)
}
bb10 = {
_7.1 = [_3.fld2,_21.1,_3.fld2,_11.2.1,_19,_3.fld2];
_8.0 = _7.2;
_3.fld0.2 = [(-2044821371_i32),1894031997_i32,(-1246401494_i32)];
place!(Field::<[isize; 6]>(Variant(_15, 1), 2)) = [_3.fld2,_3.fld2,_21.1,_3.fld2,_21.1,_3.fld2];
_30 = [2_usize,1_usize,6_usize,0_usize,5254743100480493677_usize,12568958642407621253_usize,17170983122575704112_usize,7_usize];
_19 = _21.1;
_7 = _4.fld2.fld0;
place!(Field::<u16>(Variant(_15, 1), 0)) = 31969_u16;
_4.fld2.fld0.1 = [_21.1,_21.1,_21.1,_3.fld2,_21.1,_3.fld2];
_4.fld2.fld1 = _8.0;
_31 = [_3.fld2,_11.2.1,_21.1,_3.fld2,_21.1,_19];
_3.fld2 = 92_u8 as isize;
_21.0 = 4_usize as f64;
_11.6.0 = Field::<u64>(Variant(_15, 1), 1) - Field::<u64>(Variant(_15, 1), 1);
_14 = Adt54::Variant2 { fld0: _21.1 };
_8.0 = RET.0;
_23 = _11.6.0;
SetDiscriminant(_14, 1);
RET.0 = [(-1616164154_i32),(-1633715769_i32),(-638413246_i32)];
match Field::<u16>(Variant(_15, 1), 0) {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb6,
4 => bb11,
5 => bb12,
6 => bb13,
31969 => bb15,
_ => bb14
}
}
bb11 = {
_4.fld2.fld0.2 = _3.fld1;
_4.fld1 = (_12,);
place!(Field::<u16>(Variant(_15, 1), 0)) = 5574_u16 - 24760_u16;
place!(Field::<u64>(Variant(_15, 1), 1)) = _11.6.0;
SetDiscriminant(_14, 0);
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_14, 0), 1)).3 = -_11.2.0;
Goto(bb10)
}
bb12 = {
_8 = (_3.fld0.2,);
_5 = _2;
_11.6.4 = _5 as i128;
_9 = [1352131547131018455_u64,2346538902146449471_u64,3220406836728647077_u64,899263627875507407_u64,18029498572767522159_u64,4105701812905225411_u64];
_4.fld2.fld0.1 = [_3.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2,_3.fld2,_3.fld2];
_4.fld1.0 = [_4.fld2.fld2,_3.fld2,_3.fld2,_3.fld2,_4.fld2.fld2,_3.fld2];
_5 = _2 != _2;
_6 = [_3.fld2,_4.fld2.fld2];
_11.2.0 = 1578809382_i32 as f64;
_11.6.2 = _11.6.4 | _11.6.4;
_4.fld1.0 = _3.fld0.1;
_11.6 = (11729373078515279336_u64, 31036_i16, 63735722649894342257218427980078257119_i128, _11.2.0, 71543016008872890774166114355504712535_i128);
_3.fld0.2 = [(-1095410628_i32),(-1002985789_i32),(-1589306977_i32)];
_11.4.0 = [_4.fld2.fld2,_3.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2,_4.fld2.fld2];
_11.4 = (_1,);
_11.2 = (_11.6.3, _3.fld2);
Call(_11.6.3 = core::intrinsics::fmaf64(_11.2.0, _11.2.0, _11.2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
RET.0 = [(-769645877_i32),1161556051_i32,1860078427_i32];
_4.fld1 = (_1,);
_4.fld2.fld2 = 32_i8 as isize;
_4.fld4 = [210698390582076370050101548614064197054_u128,268651578115895251089489817737829077452_u128,30855361480372556730234973026678102318_u128,100713351977821368133690742985744611162_u128,175825788728194169215108551018193969145_u128,249963156176916526990527944558764456153_u128];
_4.fld2.fld2 = -(-9223372036854775808_isize);
_4.fld2.fld1 = [886568540_i32,(-406647556_i32),(-1371163191_i32)];
RET.0 = _4.fld2.fld1;
_3.fld2 = 2094666528_u32 as isize;
_3.fld1 = RET.0;
_6 = [_3.fld2,_4.fld2.fld2];
_4.fld1 = (_1,);
_7.2 = [(-1270292984_i32),969609926_i32,(-1037170566_i32)];
_3.fld0.2 = [(-1631538816_i32),(-1340606504_i32),954328700_i32];
_3.fld0.1 = [_3.fld2,_3.fld2,_4.fld2.fld2,_3.fld2,_4.fld2.fld2,_3.fld2];
_4.fld2.fld0.2 = [(-1685103262_i32),(-1214546177_i32),(-841177061_i32)];
Goto(bb3)
}
bb14 = {
_2 = 2502909096_u32 <= 1023163923_u32;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-68_isize),125_isize];
_1 = [9223372036854775807_isize,(-72_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET.0 = [(-146931678_i32),660605433_i32,444683623_i32];
Goto(bb2)
}
bb15 = {
_11.1 = 66423207019384139363511374668486623507_u128 << _11.6.4;
_3.fld0.0 = [_21.1,_11.2.1,_19,_19,_19,_19];
RET.0 = [822194182_i32,1822006562_i32,(-1181101768_i32)];
_16 = _11.6.1 << _11.6.0;
_8 = (_3.fld0.2,);
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(12_usize, 7_usize, Move(_7), 8_usize, Move(_8), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(12_usize, 31_usize, Move(_31), 23_usize, Move(_23), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: ([isize; 6], [isize; 6], [i32; 3]),mut _2: Adt49,mut _3: [isize; 6],mut _4: [isize; 6],mut _5: ([isize; 6], [isize; 6], [i32; 3]),mut _6: [isize; 6],mut _7: isize) -> ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128)) {
mir! {
type RET = ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _8: i32;
let _9: [isize; 6];
let _10: isize;
let _11: ([isize; 6],);
let _12: f32;
let _13: i8;
let _14: i64;
let _15: u32;
let _16: *const ([i32; 3],);
let _17: u16;
let _18: Adt52;
let _19: ([i32; 3],);
let _20: isize;
let _21: ([isize; 6], [isize; 6], [i32; 3]);
let _22: [bool; 2];
let _23: char;
let _24: f32;
let _25: Adt40;
let _26: Adt43;
let _27: u64;
let _28: [u8; 2];
let _29: Adt43;
let _30: i16;
let _31: (f64, isize);
let _32: bool;
let _33: Adt47;
let _34: char;
let _35: f32;
let _36: i8;
let _37: [isize; 6];
let _38: Adt51;
let _39: Adt49;
let _40: Adt46;
let _41: Adt50;
let _42: (f64, isize);
let _43: char;
let _44: u64;
let _45: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _46: Adt52;
let _47: isize;
let _48: [u64; 6];
let _49: i32;
let _50: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _51: [bool; 2];
let _52: ([isize; 6], [isize; 6], [i32; 3]);
let _53: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _54: f64;
let _55: Adt41;
let _56: isize;
let _57: [u128; 6];
let _58: (f64, isize);
let _59: u8;
let _60: i16;
let _61: ([isize; 6], [isize; 6], [i32; 3]);
let _62: (f64, isize);
let _63: i128;
let _64: ();
let _65: ();
{
RET.6.1 = 31490_i16;
_2.fld2 = !_7;
_5 = (_1.0, _2.fld0.0, _2.fld1);
_5.0 = [_2.fld2,_2.fld2,_7,_2.fld2,_2.fld2,_7];
RET.2.1 = _2.fld2;
_2.fld0.0 = [_7,RET.2.1,RET.2.1,_2.fld2,RET.2.1,_7];
RET.4.0 = _1.0;
_5.1 = [_7,_2.fld2,_7,_2.fld2,_2.fld2,RET.2.1];
RET.5 = [11191140460310645073_u64,16496967870830245363_u64,15971536497833903660_u64,14810886073845309364_u64,18231268231923502013_u64,3558519461457303884_u64];
RET.5 = [18423394553523943169_u64,8986981124735393550_u64,7906821725747420042_u64,14260884300114662369_u64,14952429960129192722_u64,6208098383301391974_u64];
_11.0 = [RET.2.1,_2.fld2,_7,RET.2.1,_2.fld2,RET.2.1];
RET.6.3 = 40754605084198667991115728328751147434_i128 as f64;
_5.2 = [581589903_i32,1991940018_i32,566792751_i32];
_3 = [_7,RET.2.1,RET.2.1,RET.2.1,RET.2.1,_2.fld2];
RET.6.1 = 6347471078001551540_usize as i16;
_1.2 = [1793804587_i32,50845519_i32,1820726641_i32];
_11 = (_4,);
Goto(bb1)
}
bb1 = {
RET.6.2 = 93594458869092647777843340871716139767_i128 ^ (-147101130205366960812929340527860657646_i128);
RET.3 = [136197367503191239471621801272790587920_u128,5998177211391570224114040220592685032_u128,290895361294527144254194801571957689299_u128,312465902271895402668130243004220126190_u128,226070528406038473339690407916381244539_u128,191401415921502233168128198629531145725_u128];
RET.6.0 = !10971817254481912435_u64;
RET.6.4 = -RET.6.2;
RET.0 = [RET.2.1,_2.fld2];
RET.6.2 = RET.6.4 * RET.6.4;
_6 = [_7,_7,_7,RET.2.1,_2.fld2,RET.2.1];
_1 = (_2.fld0.1, _2.fld0.1, _2.fld0.2);
RET.1 = 170242193995165840920854334848232279657_u128 - 8876125555639450436644814071579821887_u128;
_12 = _7 as f32;
RET.5 = [RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0];
_3 = RET.4.0;
_9 = [RET.2.1,RET.2.1,_7,_2.fld2,_2.fld2,RET.2.1];
RET.4.0 = _3;
Call(_18.fld0 = fn14(RET.6, _1, _5.2, _1.1, _2.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = [_7,_7];
_11.0 = [_2.fld2,RET.2.1,_7,_7,_7,_2.fld2];
_18.fld1 = (_5.1,);
_1.1 = [_2.fld2,_7,RET.2.1,RET.2.1,RET.2.1,_2.fld2];
_2.fld0.1 = [_2.fld2,_7,_7,RET.2.1,RET.2.1,_2.fld2];
RET.4.0 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_7];
RET.4 = (_4,);
_1 = (RET.4.0, _5.0, _2.fld1);
_14 = !(-2473524013668239736_i64);
RET.1 = !80450766291027283035794363168366861947_u128;
_3 = _4;
RET.5 = [RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0];
Goto(bb3)
}
bb3 = {
_2.fld0 = _1;
_17 = !49481_u16;
_11.0 = [_2.fld2,RET.2.1,RET.2.1,_7,_2.fld2,_7];
_15 = 3370116989_u32;
_7 = _2.fld2;
_2.fld0 = (_3, _3, _5.2);
_18.fld2.fld0.1 = _6;
_1.2 = [318219094_i32,1645011268_i32,(-33785438_i32)];
_19 = (_2.fld1,);
_18.fld4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET.2.1 = _2.fld2 + _7;
RET.6.2 = RET.6.4 * RET.6.4;
_9 = [_2.fld2,RET.2.1,_2.fld2,_7,_7,_2.fld2];
_18.fld2.fld1 = [(-1412242953_i32),(-1840262365_i32),(-1292164854_i32)];
_30 = RET.6.3 as i16;
RET.2 = (RET.6.3, _2.fld2);
_25 = Adt40::Variant1 { fld0: _17,fld1: RET.6.0,fld2: _4,fld3: 9667964998922981556_usize,fld4: RET.4,fld5: RET.0 };
Goto(bb4)
}
bb4 = {
_21.1 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_2.fld2];
_13 = 21_i8;
RET.6.1 = _30;
_5 = (_9, _4, _1.2);
RET.2 = (RET.6.3, _2.fld2);
RET.5 = [RET.6.0,RET.6.0,Field::<u64>(Variant(_25, 1), 1),RET.6.0,Field::<u64>(Variant(_25, 1), 1),Field::<u64>(Variant(_25, 1), 1)];
RET.5 = [Field::<u64>(Variant(_25, 1), 1),RET.6.0,Field::<u64>(Variant(_25, 1), 1),Field::<u64>(Variant(_25, 1), 1),RET.6.0,RET.6.0];
RET.2.0 = RET.6.3;
_18.fld2.fld0.2 = [(-1417195433_i32),(-1077231635_i32),(-1598807995_i32)];
RET.4.0 = [_2.fld2,_2.fld2,_7,_2.fld2,_7,RET.2.1];
RET.4.0 = [_2.fld2,RET.2.1,_2.fld2,RET.2.1,RET.2.1,_2.fld2];
_18.fld5 = -RET.6.4;
_31 = (RET.6.3, _7);
RET.2.1 = _31.1 + _2.fld2;
_21.0 = _4;
_24 = _15 as f32;
RET.4.0 = [RET.2.1,_7,_31.1,RET.2.1,_31.1,RET.2.1];
RET.6.2 = _24 as i128;
_18.fld1.0 = _2.fld0.0;
_11.0 = RET.4.0;
_8 = -671317114_i32;
Goto(bb5)
}
bb5 = {
_20 = -_2.fld2;
_18.fld2.fld1 = [_8,_8,_8];
RET.4 = (_3,);
RET.6.2 = RET.6.4 << _8;
place!(Field::<[isize; 2]>(Variant(_25, 1), 5)) = [_31.1,RET.2.1];
_28 = [229_u8,150_u8];
RET.5 = [RET.6.0,Field::<u64>(Variant(_25, 1), 1),Field::<u64>(Variant(_25, 1), 1),RET.6.0,Field::<u64>(Variant(_25, 1), 1),RET.6.0];
_17 = !Field::<u16>(Variant(_25, 1), 0);
_27 = !Field::<u64>(Variant(_25, 1), 1);
place!(Field::<u16>(Variant(_25, 1), 0)) = !_17;
_18.fld2.fld0.2 = _1.2;
_36 = _14 as i8;
_45 = (Field::<[isize; 2]>(Variant(_25, 1), 5), RET.1, RET.2, RET.3, _18.fld1, RET.5, RET.6);
Goto(bb6)
}
bb6 = {
RET.2 = (_45.6.3, _31.1);
_32 = !true;
_36 = _13 ^ _13;
RET.5 = _45.5;
_1.1 = _5.0;
_42.0 = _8 as f64;
_18.fld2.fld1 = [_8,_8,_8];
place!(Field::<u64>(Variant(_25, 1), 1)) = RET.6.0;
_5.2 = [_8,_8,_8];
_46.fld2.fld2 = _45.6.2 as isize;
_46.fld2.fld0.1 = _9;
_33 = Adt47::Variant1 { fld0: 200_u8 };
RET.2 = (_45.6.3, _46.fld2.fld2);
_39 = _2;
_9 = [_46.fld2.fld2,_45.2.1,RET.2.1,_39.fld2,_46.fld2.fld2,_2.fld2];
_45.2.0 = _31.0 + _42.0;
_45.6.4 = RET.6.2 - RET.6.4;
_39.fld2 = _32 as isize;
place!(Field::<[isize; 6]>(Variant(_25, 1), 2)) = [_45.2.1,_20,_46.fld2.fld2,RET.2.1,_20,_2.fld2];
_18.fld2 = Adt49 { fld0: _5,fld1: _39.fld1,fld2: _7 };
Call(_46.fld5 = core::intrinsics::transmute(RET.6.4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET.2.1 = _46.fld2.fld2 | _7;
_5 = _39.fld0;
_47 = _18.fld2.fld2;
_31.0 = _18.fld5 as f64;
_18.fld1.0 = [_7,_45.2.1,_7,_45.2.1,_46.fld2.fld2,_20];
_32 = false;
_15 = 3332024583_u32;
_21.2 = [_8,_8,_8];
_21 = (_1.0, _1.1, _5.2);
_46.fld1 = RET.4;
_50.6.4 = !_45.6.2;
_30 = RET.6.1;
_15 = 778360108_u32 | 4088629717_u32;
_33 = Adt47::Variant1 { fld0: 195_u8 };
_27 = _45.6.0 + RET.6.0;
_1.2 = [_8,_8,_8];
_23 = '\u{e082f}';
_18.fld2.fld0.0 = [_45.2.1,RET.2.1,RET.2.1,RET.2.1,RET.2.1,_46.fld2.fld2];
_46.fld2.fld0.2 = [_8,_8,_8];
_19 = (_21.2,);
_39.fld1 = [_8,_8,_8];
match _13 {
21 => bb9,
_ => bb8
}
}
bb8 = {
RET.0 = [_7,_7];
_11.0 = [_2.fld2,RET.2.1,_7,_7,_7,_2.fld2];
_18.fld1 = (_5.1,);
_1.1 = [_2.fld2,_7,RET.2.1,RET.2.1,RET.2.1,_2.fld2];
_2.fld0.1 = [_2.fld2,_7,_7,RET.2.1,RET.2.1,_2.fld2];
RET.4.0 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_7];
RET.4 = (_4,);
_1 = (RET.4.0, _5.0, _2.fld1);
_14 = !(-2473524013668239736_i64);
RET.1 = !80450766291027283035794363168366861947_u128;
_3 = _4;
RET.5 = [RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0];
Goto(bb3)
}
bb9 = {
_50.5 = [RET.6.0,_45.6.0,_27,Field::<u64>(Variant(_25, 1), 1),_45.6.0,_45.6.0];
_5.2 = _39.fld0.2;
RET.4 = _46.fld1;
_31.1 = -RET.2.1;
_27 = _23 as u64;
_43 = _23;
_39.fld0 = _2.fld0;
_2 = Adt49 { fld0: _18.fld2.fld0,fld1: _39.fld0.2,fld2: _46.fld2.fld2 };
_50.6.4 = RET.6.4;
RET.6 = _45.6;
_43 = _23;
_45.6 = (Field::<u64>(Variant(_25, 1), 1), RET.6.1, RET.6.4, _45.2.0, _46.fld5);
_50.1 = RET.1;
_7 = RET.2.1 | _20;
Goto(bb10)
}
bb10 = {
_52 = (_18.fld1.0, _18.fld2.fld0.1, _18.fld2.fld1);
_10 = RET.6.4 as isize;
_46.fld1 = (_18.fld1.0,);
_39.fld0.1 = _46.fld2.fld0.1;
_2.fld2 = _47;
_53.4.0 = [_31.1,_31.1,_18.fld2.fld2,RET.2.1,_31.1,RET.2.1];
RET.1 = _50.1 | _45.1;
_50.5 = [_45.6.0,Field::<u64>(Variant(_25, 1), 1),_27,Field::<u64>(Variant(_25, 1), 1),_27,RET.6.0];
_16 = core::ptr::addr_of!(_19);
_53.6.2 = _7 as i128;
_53.0 = [_31.1,_10];
_50.6.3 = _8 as f64;
_46.fld2 = Adt49 { fld0: _52,fld1: _21.2,fld2: _2.fld2 };
_53.2.0 = _45.1 as f64;
_2.fld1 = (*_16).0;
Goto(bb11)
}
bb11 = {
_7 = !_20;
RET.1 = 6_usize as u128;
_1 = (_18.fld2.fld0.1, RET.4.0, _2.fld1);
_50.6.2 = -_50.6.4;
_50.6 = RET.6;
RET.4 = (_5.1,);
_2 = Adt49 { fld0: _5,fld1: _18.fld2.fld0.2,fld2: _45.2.1 };
_21 = (_4, _46.fld2.fld0.0, _39.fld0.2);
RET.6.0 = _45.6.0 << _31.1;
_45.4.0 = [_10,_31.1,_45.2.1,_10,_10,_18.fld2.fld2];
_53.6 = _50.6;
_46.fld2.fld0.2 = [_8,_8,_8];
_53.3 = [_45.1,_45.1,RET.1,_45.1,_50.1,_50.1];
place!(Field::<usize>(Variant(_25, 1), 3)) = 11163311103240984524_usize * 0_usize;
_56 = RET.2.1;
_53.2 = (_45.2.0, _46.fld2.fld2);
_53.5 = [RET.6.0,RET.6.0,_45.6.0,_45.6.0,RET.6.0,RET.6.0];
_39.fld0.1 = [_10,_10,_10,_39.fld2,_10,_56];
_3 = [_56,_2.fld2,_56,_45.2.1,_10,_47];
RET = (_53.0, _50.1, _53.2, _45.3, _45.4, _45.5, _53.6);
_18.fld2.fld0 = _52;
_45.6.3 = RET.2.0 + _50.6.3;
_46.fld2.fld0.2 = [_8,_8,_8];
_18.fld5 = _14 as i128;
_50.6.0 = _45.6.0;
SetDiscriminant(_25, 1);
_45.6.2 = _14 as i128;
_50.5 = [RET.6.0,RET.6.0,_45.6.0,RET.6.0,_27,_27];
RET.4 = _53.4;
RET.6.1 = _50.6.1 >> _2.fld2;
Call(_18.fld5 = core::intrinsics::bswap(_46.fld5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2.fld0.2 = [_8,_8,_8];
_53.6.0 = _50.6.0;
_53.6.4 = RET.1 as i128;
RET.1 = _50.1 << RET.6.4;
_39.fld0.1 = _2.fld0.1;
_27 = RET.6.0 >> _47;
_23 = _43;
_32 = _36 > _36;
place!(Field::<u16>(Variant(_25, 1), 0)) = _17 >> _2.fld2;
_42 = _53.2;
_46.fld1 = (_53.4.0,);
RET.6.4 = 187_u8 as i128;
_16 = core::ptr::addr_of!(_19);
_18.fld2.fld0.0 = [_56,_10,RET.2.1,_2.fld2,_39.fld2,_20];
_50 = (_53.0, RET.1, _53.2, RET.3, _46.fld1, _53.5, _45.6);
_12 = -_24;
_22 = [_32,_32];
_48 = _50.5;
place!(Field::<u8>(Variant(_33, 1), 0)) = 218_u8;
_53.1 = _50.1 - _50.1;
_44 = _27;
_50.6.0 = !_44;
_39.fld2 = _56 ^ _10;
Goto(bb13)
}
bb13 = {
_18.fld5 = -_45.6.4;
_42.1 = _10;
_43 = _23;
SetDiscriminant(_33, 1);
_50.6.3 = 15148625374441601198_usize as f64;
_1.1 = [_45.2.1,_10,_42.1,_39.fld2,_31.1,_31.1];
_49 = !_8;
_46.fld2.fld0 = _21;
_53.2.0 = -_50.6.3;
place!(Field::<u8>(Variant(_33, 1), 0)) = _24 as u8;
_52.0 = [_18.fld2.fld2,_20,_7,_42.1,_39.fld2,_56];
SetDiscriminant(_33, 2);
_18.fld4 = [RET.1,_53.1,_50.1,_53.1,_50.1,_53.1];
_23 = _43;
_25 = Adt40::Variant1 { fld0: _17,fld1: _53.6.0,fld2: _52.0,fld3: 5453295159964632306_usize,fld4: _18.fld1,fld5: RET.0 };
_2.fld2 = _47 * _39.fld2;
_45.0 = [_42.1,_2.fld2];
_46.fld2.fld1 = [_49,_8,_49];
_18.fld2 = _39;
match _13 {
0 => bb1,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
21 => bb21,
_ => bb20
}
}
bb14 = {
_21.1 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_2.fld2];
_13 = 21_i8;
RET.6.1 = _30;
_5 = (_9, _4, _1.2);
RET.2 = (RET.6.3, _2.fld2);
RET.5 = [RET.6.0,RET.6.0,Field::<u64>(Variant(_25, 1), 1),RET.6.0,Field::<u64>(Variant(_25, 1), 1),Field::<u64>(Variant(_25, 1), 1)];
RET.5 = [Field::<u64>(Variant(_25, 1), 1),RET.6.0,Field::<u64>(Variant(_25, 1), 1),Field::<u64>(Variant(_25, 1), 1),RET.6.0,RET.6.0];
RET.2.0 = RET.6.3;
_18.fld2.fld0.2 = [(-1417195433_i32),(-1077231635_i32),(-1598807995_i32)];
RET.4.0 = [_2.fld2,_2.fld2,_7,_2.fld2,_7,RET.2.1];
RET.4.0 = [_2.fld2,RET.2.1,_2.fld2,RET.2.1,RET.2.1,_2.fld2];
_18.fld5 = -RET.6.4;
_31 = (RET.6.3, _7);
RET.2.1 = _31.1 + _2.fld2;
_21.0 = _4;
_24 = _15 as f32;
RET.4.0 = [RET.2.1,_7,_31.1,RET.2.1,_31.1,RET.2.1];
RET.6.2 = _24 as i128;
_18.fld1.0 = _2.fld0.0;
_11.0 = RET.4.0;
_8 = -671317114_i32;
Goto(bb5)
}
bb15 = {
_7 = !_20;
RET.1 = 6_usize as u128;
_1 = (_18.fld2.fld0.1, RET.4.0, _2.fld1);
_50.6.2 = -_50.6.4;
_50.6 = RET.6;
RET.4 = (_5.1,);
_2 = Adt49 { fld0: _5,fld1: _18.fld2.fld0.2,fld2: _45.2.1 };
_21 = (_4, _46.fld2.fld0.0, _39.fld0.2);
RET.6.0 = _45.6.0 << _31.1;
_45.4.0 = [_10,_31.1,_45.2.1,_10,_10,_18.fld2.fld2];
_53.6 = _50.6;
_46.fld2.fld0.2 = [_8,_8,_8];
_53.3 = [_45.1,_45.1,RET.1,_45.1,_50.1,_50.1];
place!(Field::<usize>(Variant(_25, 1), 3)) = 11163311103240984524_usize * 0_usize;
_56 = RET.2.1;
_53.2 = (_45.2.0, _46.fld2.fld2);
_53.5 = [RET.6.0,RET.6.0,_45.6.0,_45.6.0,RET.6.0,RET.6.0];
_39.fld0.1 = [_10,_10,_10,_39.fld2,_10,_56];
_3 = [_56,_2.fld2,_56,_45.2.1,_10,_47];
RET = (_53.0, _50.1, _53.2, _45.3, _45.4, _45.5, _53.6);
_18.fld2.fld0 = _52;
_45.6.3 = RET.2.0 + _50.6.3;
_46.fld2.fld0.2 = [_8,_8,_8];
_18.fld5 = _14 as i128;
_50.6.0 = _45.6.0;
SetDiscriminant(_25, 1);
_45.6.2 = _14 as i128;
_50.5 = [RET.6.0,RET.6.0,_45.6.0,RET.6.0,_27,_27];
RET.4 = _53.4;
RET.6.1 = _50.6.1 >> _2.fld2;
Call(_18.fld5 = core::intrinsics::bswap(_46.fld5), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_52 = (_18.fld1.0, _18.fld2.fld0.1, _18.fld2.fld1);
_10 = RET.6.4 as isize;
_46.fld1 = (_18.fld1.0,);
_39.fld0.1 = _46.fld2.fld0.1;
_2.fld2 = _47;
_53.4.0 = [_31.1,_31.1,_18.fld2.fld2,RET.2.1,_31.1,RET.2.1];
RET.1 = _50.1 | _45.1;
_50.5 = [_45.6.0,Field::<u64>(Variant(_25, 1), 1),_27,Field::<u64>(Variant(_25, 1), 1),_27,RET.6.0];
_16 = core::ptr::addr_of!(_19);
_53.6.2 = _7 as i128;
_53.0 = [_31.1,_10];
_50.6.3 = _8 as f64;
_46.fld2 = Adt49 { fld0: _52,fld1: _21.2,fld2: _2.fld2 };
_53.2.0 = _45.1 as f64;
_2.fld1 = (*_16).0;
Goto(bb11)
}
bb17 = {
_50.5 = [RET.6.0,_45.6.0,_27,Field::<u64>(Variant(_25, 1), 1),_45.6.0,_45.6.0];
_5.2 = _39.fld0.2;
RET.4 = _46.fld1;
_31.1 = -RET.2.1;
_27 = _23 as u64;
_43 = _23;
_39.fld0 = _2.fld0;
_2 = Adt49 { fld0: _18.fld2.fld0,fld1: _39.fld0.2,fld2: _46.fld2.fld2 };
_50.6.4 = RET.6.4;
RET.6 = _45.6;
_43 = _23;
_45.6 = (Field::<u64>(Variant(_25, 1), 1), RET.6.1, RET.6.4, _45.2.0, _46.fld5);
_50.1 = RET.1;
_7 = RET.2.1 | _20;
Goto(bb10)
}
bb18 = {
RET.0 = [_7,_7];
_11.0 = [_2.fld2,RET.2.1,_7,_7,_7,_2.fld2];
_18.fld1 = (_5.1,);
_1.1 = [_2.fld2,_7,RET.2.1,RET.2.1,RET.2.1,_2.fld2];
_2.fld0.1 = [_2.fld2,_7,_7,RET.2.1,RET.2.1,_2.fld2];
RET.4.0 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_7];
RET.4 = (_4,);
_1 = (RET.4.0, _5.0, _2.fld1);
_14 = !(-2473524013668239736_i64);
RET.1 = !80450766291027283035794363168366861947_u128;
_3 = _4;
RET.5 = [RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0];
Goto(bb3)
}
bb19 = {
RET.0 = [_7,_7];
_11.0 = [_2.fld2,RET.2.1,_7,_7,_7,_2.fld2];
_18.fld1 = (_5.1,);
_1.1 = [_2.fld2,_7,RET.2.1,RET.2.1,RET.2.1,_2.fld2];
_2.fld0.1 = [_2.fld2,_7,_7,RET.2.1,RET.2.1,_2.fld2];
RET.4.0 = [_7,_2.fld2,_2.fld2,RET.2.1,_7,_7];
RET.4 = (_4,);
_1 = (RET.4.0, _5.0, _2.fld1);
_14 = !(-2473524013668239736_i64);
RET.1 = !80450766291027283035794363168366861947_u128;
_3 = _4;
RET.5 = [RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0,RET.6.0];
Goto(bb3)
}
bb20 = {
RET.2 = (_45.6.3, _31.1);
_32 = !true;
_36 = _13 ^ _13;
RET.5 = _45.5;
_1.1 = _5.0;
_42.0 = _8 as f64;
_18.fld2.fld1 = [_8,_8,_8];
place!(Field::<u64>(Variant(_25, 1), 1)) = RET.6.0;
_5.2 = [_8,_8,_8];
_46.fld2.fld2 = _45.6.2 as isize;
_46.fld2.fld0.1 = _9;
_33 = Adt47::Variant1 { fld0: 200_u8 };
RET.2 = (_45.6.3, _46.fld2.fld2);
_39 = _2;
_9 = [_46.fld2.fld2,_45.2.1,RET.2.1,_39.fld2,_46.fld2.fld2,_2.fld2];
_45.2.0 = _31.0 + _42.0;
_45.6.4 = RET.6.2 - RET.6.4;
_39.fld2 = _32 as isize;
place!(Field::<[isize; 6]>(Variant(_25, 1), 2)) = [_45.2.1,_20,_46.fld2.fld2,RET.2.1,_20,_2.fld2];
_18.fld2 = Adt49 { fld0: _5,fld1: _39.fld1,fld2: _7 };
Call(_46.fld5 = core::intrinsics::transmute(RET.6.4), ReturnTo(bb7), UnwindUnreachable())
}
bb21 = {
_53.6.2 = _30 as i128;
_1.0 = [_31.1,_39.fld2,_2.fld2,_2.fld2,_45.2.1,_2.fld2];
_53.6.4 = _53.6.2;
_53.5 = [RET.6.0,_27,_50.6.0,RET.6.0,_45.6.0,_27];
place!(Field::<Adt40>(Variant(_33, 2), 3)) = Adt40::Variant1 { fld0: Field::<u16>(Variant(_25, 1), 0),fld1: Field::<u64>(Variant(_25, 1), 1),fld2: _46.fld1.0,fld3: 7_usize,fld4: Field::<([isize; 6],)>(Variant(_25, 1), 4),fld5: RET.0 };
place!(Field::<[isize; 6]>(Variant(_25, 1), 2)) = [_10,_39.fld2,_56,_39.fld2,_45.2.1,_10];
_58 = _42;
RET.6 = (_45.6.0, _50.6.1, _50.6.4, _53.2.0, _45.6.2);
_50.2.1 = _14 as isize;
RET.6.0 = _17 as u64;
place!(Field::<([isize; 6],)>(Variant(place!(Field::<Adt40>(Variant(_33, 2), 3)), 1), 4)) = (_2.fld0.0,);
Goto(bb22)
}
bb22 = {
Call(_64 = dump_var(13_usize, 36_usize, Move(_36), 22_usize, Move(_22), 47_usize, Move(_47), 30_usize, Move(_30)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_64 = dump_var(13_usize, 10_usize, Move(_10), 9_usize, Move(_9), 5_usize, Move(_5), 56_usize, Move(_56)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_64 = dump_var(13_usize, 1_usize, Move(_1), 19_usize, Move(_19), 48_usize, Move(_48), 23_usize, Move(_23)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_64 = dump_var(13_usize, 43_usize, Move(_43), 17_usize, Move(_17), 21_usize, Move(_21), 65_usize, _65), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: (u64, i16, i128, f64, i128),mut _2: ([isize; 6], [isize; 6], [i32; 3]),mut _3: [i32; 3],mut _4: [isize; 6],mut _5: [i32; 3]) -> *const [i16; 3] {
mir! {
type RET = *const [i16; 3];
let _6: isize;
let _7: isize;
let _8: Adt44;
let _9: u64;
let _10: bool;
let _11: usize;
let _12: Adt54;
let _13: u64;
let _14: (i8,);
let _15: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _16: bool;
let _17: &'static f32;
let _18: usize;
let _19: f32;
let _20: [u64; 6];
let _21: f64;
let _22: (i16, u128, [u128; 6], (i8,), u8);
let _23: Adt50;
let _24: [u128; 6];
let _25: [bool; 2];
let _26: i32;
let _27: (bool,);
let _28: ([isize; 6],);
let _29: f64;
let _30: Adt39;
let _31: i128;
let _32: isize;
let _33: f32;
let _34: isize;
let _35: i16;
let _36: f64;
let _37: Adt42;
let _38: [u8; 2];
let _39: (i16, u128, [u128; 6], (i8,), u8);
let _40: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _41: f64;
let _42: bool;
let _43: Adt44;
let _44: (i8,);
let _45: isize;
let _46: u64;
let _47: Adt40;
let _48: char;
let _49: [u64; 6];
let _50: Adt43;
let _51: usize;
let _52: i8;
let _53: [isize; 2];
let _54: u64;
let _55: Adt50;
let _56: f64;
let _57: &'static f32;
let _58: Adt49;
let _59: (i16, u128, [u128; 6], (i8,), u8);
let _60: Adt51;
let _61: Adt47;
let _62: (u64, i16, i128, f64, i128);
let _63: Adt41;
let _64: u128;
let _65: *const [i16; 3];
let _66: f64;
let _67: i16;
let _68: Adt50;
let _69: f32;
let _70: Adt42;
let _71: [i32; 3];
let _72: *const [i16; 3];
let _73: [i32; 3];
let _74: i32;
let _75: Adt41;
let _76: Adt54;
let _77: isize;
let _78: Adt44;
let _79: (u64, i16, i128, f64, i128);
let _80: i8;
let _81: bool;
let _82: [i16; 3];
let _83: (bool,);
let _84: u64;
let _85: i8;
let _86: ();
let _87: ();
{
_1.1 = (-4856_i16);
_2 = (_4, _4, _5);
_1.1 = -(-31517_i16);
_1.2 = _1.4 & _1.4;
_1.4 = 3684065726_u32 as i128;
_1.0 = false as u64;
_7 = _1.3 as isize;
_1.3 = _1.1 as f64;
_3 = [(-793746083_i32),(-1449333135_i32),312449650_i32];
_7 = 9223372036854775807_isize & (-9223372036854775808_isize);
_6 = !_7;
_1.2 = (-1110082117_i32) as i128;
_1.0 = 5_usize as u64;
_1.1 = 7319_i16 | (-84_i16);
_6 = -_7;
_4 = [_7,_6,_6,_7,_6,_6];
_2.0 = [_7,_6,_7,_7,_6,_7];
_1.3 = 47170_u16 as f64;
_4 = [_7,_7,_7,_7,_6,_6];
Call(_1.1 = fn15(_6, _2.0, _5, _1.0, _2, _2.0, _2.1, _2, _2.1, _2, _1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.0 = 10202474073288727407_u64 | 1434438592746796012_u64;
_2.1 = _4;
_6 = !_7;
_5 = [325519949_i32,(-1095143622_i32),1512432947_i32];
_9 = !_1.0;
_2 = (_4, _4, _5);
_3 = [230542458_i32,(-1388838469_i32),1228776703_i32];
_1.3 = 5_usize as f64;
_1.2 = (-1045302187_i32) as i128;
Goto(bb2)
}
bb2 = {
_6 = _7 >> _9;
_6 = _7;
_3 = [(-1179970176_i32),(-1018557707_i32),(-44927548_i32)];
_1.1 = !(-15493_i16);
_2.2 = _3;
_1.0 = _9 & _9;
_6 = '\u{d7893}' as isize;
_10 = true;
_1.0 = !_9;
_15.6.4 = _1.2;
_14.0 = -41_i8;
_9 = _1.0;
Goto(bb3)
}
bb3 = {
_15.2 = (_1.3, _7);
_1.0 = _9 & _9;
_15.1 = _10 as u128;
_6 = _15.2.1 | _15.2.1;
_15.6 = _1;
_15.6 = (_1.0, _1.1, _1.4, _15.2.0, _1.4);
_2.2 = _3;
_18 = 9905892820090385489_usize & 9378490445480417715_usize;
_1 = _15.6;
_15.6.1 = !_1.1;
_19 = 4632235_i32 as f32;
_13 = '\u{c3ab8}' as u64;
_22.4 = !225_u8;
_10 = _15.6.0 > _1.0;
_1.0 = _15.6.0 * _13;
_2 = (_4, _4, _3);
_1.2 = -_1.4;
_15.4.0 = [_6,_15.2.1,_6,_7,_7,_6];
_9 = _1.0;
Goto(bb4)
}
bb4 = {
_17 = &_19;
_15.3 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_15.4.0 = _4;
_15.6.4 = 1070607384_i32 as i128;
_2 = (_4, _15.4.0, _5);
_22 = (_15.6.1, _15.1, _15.3, _14, 55_u8);
_15.1 = !_22.1;
_15.5 = [_1.0,_9,_9,_9,_1.0,_15.6.0];
_13 = !_1.0;
_15.3 = _22.2;
_22.4 = 1414834689_u32 as u8;
Goto(bb5)
}
bb5 = {
_15.6.0 = _9 | _13;
_13 = _1.0;
_15.6.3 = -_1.3;
_1 = (_15.6.0, _15.6.1, _15.6.2, _15.6.3, _15.6.2);
_4 = [_6,_7,_7,_7,_6,_7];
_15.6.2 = -_15.6.4;
_27 = (_10,);
_18 = 0_usize;
_22.3 = _14;
_1.3 = _22.2[_18] as f64;
_17 = &_19;
_25[_18] = _27.0 | _10;
_22.2 = [_15.3[_18],_15.3[_18],_22.1,_22.1,_15.1,_15.3[_18]];
_28.0[_18] = _4[_18];
_15.6.4 = _18 as i128;
_1.0 = _15.5[_18];
_15.4 = (_4,);
_15.6.1 = _1.1 * _1.1;
_28.0 = [_4[_18],_6,_6,_15.2.1,_4[_18],_2.1[_18]];
_15.6.3 = -_1.3;
_14.0 = _15.6.1 as i8;
_17 = &(*_17);
_29 = _15.2.0 - _1.3;
_24 = _22.2;
match _18 {
0 => bb6,
_ => bb4
}
}
bb6 = {
_15.4.0 = _28.0;
_1 = (_15.6.0, _22.0, _15.6.2, _15.6.3, _15.6.2);
_33 = (*_17) * (*_17);
match _18 {
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
0 => bb13,
_ => bb12
}
}
bb7 = {
_15.6.0 = _9 | _13;
_13 = _1.0;
_15.6.3 = -_1.3;
_1 = (_15.6.0, _15.6.1, _15.6.2, _15.6.3, _15.6.2);
_4 = [_6,_7,_7,_7,_6,_7];
_15.6.2 = -_15.6.4;
_27 = (_10,);
_18 = 0_usize;
_22.3 = _14;
_1.3 = _22.2[_18] as f64;
_17 = &_19;
_25[_18] = _27.0 | _10;
_22.2 = [_15.3[_18],_15.3[_18],_22.1,_22.1,_15.1,_15.3[_18]];
_28.0[_18] = _4[_18];
_15.6.4 = _18 as i128;
_1.0 = _15.5[_18];
_15.4 = (_4,);
_15.6.1 = _1.1 * _1.1;
_28.0 = [_4[_18],_6,_6,_15.2.1,_4[_18],_2.1[_18]];
_15.6.3 = -_1.3;
_14.0 = _15.6.1 as i8;
_17 = &(*_17);
_29 = _15.2.0 - _1.3;
_24 = _22.2;
match _18 {
0 => bb6,
_ => bb4
}
}
bb8 = {
_17 = &_19;
_15.3 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_15.4.0 = _4;
_15.6.4 = 1070607384_i32 as i128;
_2 = (_4, _15.4.0, _5);
_22 = (_15.6.1, _15.1, _15.3, _14, 55_u8);
_15.1 = !_22.1;
_15.5 = [_1.0,_9,_9,_9,_1.0,_15.6.0];
_13 = !_1.0;
_15.3 = _22.2;
_22.4 = 1414834689_u32 as u8;
Goto(bb5)
}
bb9 = {
_15.2 = (_1.3, _7);
_1.0 = _9 & _9;
_15.1 = _10 as u128;
_6 = _15.2.1 | _15.2.1;
_15.6 = _1;
_15.6 = (_1.0, _1.1, _1.4, _15.2.0, _1.4);
_2.2 = _3;
_18 = 9905892820090385489_usize & 9378490445480417715_usize;
_1 = _15.6;
_15.6.1 = !_1.1;
_19 = 4632235_i32 as f32;
_13 = '\u{c3ab8}' as u64;
_22.4 = !225_u8;
_10 = _15.6.0 > _1.0;
_1.0 = _15.6.0 * _13;
_2 = (_4, _4, _3);
_1.2 = -_1.4;
_15.4.0 = [_6,_15.2.1,_6,_7,_7,_6];
_9 = _1.0;
Goto(bb4)
}
bb10 = {
_6 = _7 >> _9;
_6 = _7;
_3 = [(-1179970176_i32),(-1018557707_i32),(-44927548_i32)];
_1.1 = !(-15493_i16);
_2.2 = _3;
_1.0 = _9 & _9;
_6 = '\u{d7893}' as isize;
_10 = true;
_1.0 = !_9;
_15.6.4 = _1.2;
_14.0 = -41_i8;
_9 = _1.0;
Goto(bb3)
}
bb11 = {
_1.0 = 10202474073288727407_u64 | 1434438592746796012_u64;
_2.1 = _4;
_6 = !_7;
_5 = [325519949_i32,(-1095143622_i32),1512432947_i32];
_9 = !_1.0;
_2 = (_4, _4, _5);
_3 = [230542458_i32,(-1388838469_i32),1228776703_i32];
_1.3 = 5_usize as f64;
_1.2 = (-1045302187_i32) as i128;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_15.2.0 = -_29;
_21 = -_15.2.0;
_20 = [_1.0,_13,_1.0,_15.5[_18],_15.6.0,_1.0];
_15.2 = (_1.3, _7);
_27 = (_25[_18],);
_10 = _1.0 != _20[_18];
_15.0[_18] = _4[_18] >> _1.0;
_15.0 = [_2.1[_18],_6];
_24[_18] = _22.2[_18];
_28 = (_4,);
_2.0[_18] = -_15.4.0[_18];
_2.1 = [_4[_18],_7,_15.0[_18],_2.0[_18],_4[_18],_7];
_34 = _4[_18] ^ _2.0[_18];
_1.4 = _1.2;
_4[_18] = _14.0 as isize;
_26 = _2.2[_18];
_35 = _1.1 << _5[_18];
_36 = _29 * _21;
_26 = _2.2[_18];
_25[_18] = _10;
_15.4 = (_28.0,);
_31 = _15.6.4;
_22.2[_18] = !_15.3[_18];
_15.6.0 = !_13;
_22 = (_35, _15.3[_18], _15.3, _14, 246_u8);
_34 = -_7;
Goto(bb14)
}
bb14 = {
_22.2[_18] = _19 as u128;
_1.0 = '\u{3612e}' as u64;
_11 = 3859912952_u32 as usize;
_16 = _25[_18];
_16 = !_10;
_20 = [_15.6.0,_13,_9,_13,_9,_13];
_20[_18] = !_9;
_2.2[_18] = _5[_18];
_16 = _25[_18];
_27.0 = _16;
_28.0 = [_6,_34,_2.0[_18],_15.2.1,_15.2.1,_15.2.1];
_22.1 = _18 as u128;
match _22.4 {
0 => bb8,
1 => bb6,
2 => bb3,
3 => bb15,
4 => bb16,
246 => bb18,
_ => bb17
}
}
bb15 = {
_15.2.0 = -_29;
_21 = -_15.2.0;
_20 = [_1.0,_13,_1.0,_15.5[_18],_15.6.0,_1.0];
_15.2 = (_1.3, _7);
_27 = (_25[_18],);
_10 = _1.0 != _20[_18];
_15.0[_18] = _4[_18] >> _1.0;
_15.0 = [_2.1[_18],_6];
_24[_18] = _22.2[_18];
_28 = (_4,);
_2.0[_18] = -_15.4.0[_18];
_2.1 = [_4[_18],_7,_15.0[_18],_2.0[_18],_4[_18],_7];
_34 = _4[_18] ^ _2.0[_18];
_1.4 = _1.2;
_4[_18] = _14.0 as isize;
_26 = _2.2[_18];
_35 = _1.1 << _5[_18];
_36 = _29 * _21;
_26 = _2.2[_18];
_25[_18] = _10;
_15.4 = (_28.0,);
_31 = _15.6.4;
_22.2[_18] = !_15.3[_18];
_15.6.0 = !_13;
_22 = (_35, _15.3[_18], _15.3, _14, 246_u8);
_34 = -_7;
Goto(bb14)
}
bb16 = {
_17 = &_19;
_15.3 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_15.4.0 = _4;
_15.6.4 = 1070607384_i32 as i128;
_2 = (_4, _15.4.0, _5);
_22 = (_15.6.1, _15.1, _15.3, _14, 55_u8);
_15.1 = !_22.1;
_15.5 = [_1.0,_9,_9,_9,_1.0,_15.6.0];
_13 = !_1.0;
_15.3 = _22.2;
_22.4 = 1414834689_u32 as u8;
Goto(bb5)
}
bb17 = {
_15.2 = (_1.3, _7);
_1.0 = _9 & _9;
_15.1 = _10 as u128;
_6 = _15.2.1 | _15.2.1;
_15.6 = _1;
_15.6 = (_1.0, _1.1, _1.4, _15.2.0, _1.4);
_2.2 = _3;
_18 = 9905892820090385489_usize & 9378490445480417715_usize;
_1 = _15.6;
_15.6.1 = !_1.1;
_19 = 4632235_i32 as f32;
_13 = '\u{c3ab8}' as u64;
_22.4 = !225_u8;
_10 = _15.6.0 > _1.0;
_1.0 = _15.6.0 * _13;
_2 = (_4, _4, _3);
_1.2 = -_1.4;
_15.4.0 = [_6,_15.2.1,_6,_7,_7,_6];
_9 = _1.0;
Goto(bb4)
}
bb18 = {
_1.4 = _31;
_6 = _28.0[_18];
_34 = _2.0[_18] - _6;
_15.6.3 = _36;
_22.2[_18] = !_15.3[_18];
_15.2 = (_36, _28.0[_18]);
_2.2 = [_26,_5[_18],_5[_18]];
_18 = 2043897529_u32 as usize;
_39.4 = (*_17) as u8;
_15.4.0 = [_6,_34,_6,_7,_15.2.1,_34];
_2.2 = [_26,_26,_26];
_26 = !(-1467102390_i32);
_40.2.0 = _15.2.0;
_6 = _34 & _34;
_35 = '\u{89450}' as i16;
_26 = (-1396548623_i32) & 229949976_i32;
_15.6.2 = _16 as i128;
match _22.4 {
0 => bb1,
1 => bb14,
2 => bb7,
3 => bb12,
4 => bb5,
5 => bb6,
246 => bb20,
_ => bb19
}
}
bb19 = {
_15.4.0 = _28.0;
_1 = (_15.6.0, _22.0, _15.6.2, _15.6.3, _15.6.2);
_33 = (*_17) * (*_17);
match _18 {
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
0 => bb13,
_ => bb12
}
}
bb20 = {
_40.6.0 = _13 + _15.6.0;
_15.4 = (_4,);
_39.0 = _35 | _1.1;
_40.6 = (_15.6.0, _15.6.1, _15.6.2, _15.2.0, _15.6.2);
_16 = _10;
_40.5 = _20;
_1.2 = _40.6.4;
_31 = !_1.2;
_15.0 = [_34,_6];
_10 = _27.0;
_40.6 = (_15.6.0, _39.0, _31, _36, _15.6.2);
_14.0 = _22.3.0;
_45 = _6 * _34;
_40.5 = _20;
_40.4 = (_28.0,);
_7 = _45;
_38 = [_39.4,_22.4];
Goto(bb21)
}
bb21 = {
_15.6 = (_9, _22.0, _1.2, _40.6.3, _40.6.2);
_40.2 = (_15.2.0, _7);
_45 = _40.2.1 * _7;
_1.1 = -_22.0;
_14.0 = -_22.3.0;
_15.4 = _28;
_1 = _40.6;
_17 = &_19;
_31 = _40.6.2;
_22 = (_35, _15.1, _24, _14, _39.4);
_32 = 3888141611_u32 as isize;
_36 = -_40.6.3;
_22.1 = _15.1 + _15.1;
_33 = -(*_17);
_27 = (_10,);
_10 = _27.0 ^ _27.0;
_2.2 = [_26,_26,_26];
_17 = &(*_17);
_33 = _19;
Goto(bb22)
}
bb22 = {
_40.1 = '\u{c6bfd}' as u128;
_15.6.3 = _39.0 as f64;
_48 = '\u{105e5f}';
_39.2 = _22.2;
_1.4 = _10 as i128;
_1.2 = 20800_u16 as i128;
Goto(bb23)
}
bb23 = {
_1.4 = _31;
_40.3 = [_22.1,_15.1,_22.1,_40.1,_15.1,_22.1];
_40.0 = [_40.2.1,_45];
_40.1 = _22.1 + _22.1;
_2.2 = _3;
_40.2.0 = _15.6.3;
_1.3 = -_40.6.3;
_39.1 = !_40.1;
_40.6.0 = _1.0;
_22.3.0 = _48 as i8;
_3 = _5;
_39.3 = (_14.0,);
_45 = _21 as isize;
_38 = [_22.4,_22.4];
_17 = &_33;
_40.2.1 = _48 as isize;
_5 = [_26,_26,_26];
_14.0 = !_39.3.0;
_28.0 = [_7,_34,_7,_7,_45,_7];
_40.1 = !_15.1;
_3 = [_26,_26,_26];
_39.3 = (_14.0,);
Goto(bb24)
}
bb24 = {
_11 = !_18;
_40.6.4 = _40.6.2;
_9 = _1.0 << _40.6.2;
_1.4 = _1.1 as i128;
_40.6.2 = _15.6.2;
_48 = '\u{afb6a}';
_40.6.4 = _15.6.1 as i128;
_22.2 = [_15.1,_39.1,_39.1,_39.1,_39.1,_39.1];
_1.4 = _34 as i128;
_49 = [_9,_9,_9,_13,_9,_9];
_56 = _29;
_17 = &(*_17);
_58 = Adt49 { fld0: _2,fld1: _3,fld2: _7 };
_46 = _39.1 as u64;
_40.6.2 = _15.6.2;
_16 = _10 & _10;
_40.1 = 39498_u16 as u128;
_22.2 = [_22.1,_22.1,_40.1,_39.1,_40.1,_22.1];
_1.3 = -_40.2.0;
_14.0 = _39.3.0;
_24 = _22.2;
_58.fld2 = _48 as isize;
Goto(bb25)
}
bb25 = {
_27 = (_16,);
_58 = Adt49 { fld0: _2,fld1: _5,fld2: _6 };
Goto(bb26)
}
bb26 = {
_39.0 = _1.1;
_32 = _39.0 as isize;
_58.fld0 = (_28.0, _4, _3);
_40.6.1 = _35;
_22.0 = _15.6.1;
_18 = _40.6.4 as usize;
_39.0 = _15.6.1 | _15.6.1;
_40.6.4 = !_1.2;
_51 = _18 << _7;
_53 = _40.0;
_16 = _27.0;
_15.4 = _28;
_67 = -_35;
_59 = (_22.0, _39.1, _22.2, _39.3, _22.4);
_66 = _29 - _40.2.0;
_44 = (_22.3.0,);
_40.2 = (_66, _6);
_57 = &(*_17);
Call(_58.fld0.2 = fn19(_15.0, _51, _34, _49, _40.6.2, _40.0, _7), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_62.2 = !_1.4;
_69 = (*_57) * (*_57);
_39.1 = _59.1;
_10 = _16;
_12 = Adt54::Variant2 { fld0: _40.2.1 };
_52 = -_44.0;
_16 = _10;
_15.6.1 = !_39.0;
Goto(bb28)
}
bb28 = {
_48 = '\u{2bd75}';
_6 = 6307340412957248053_i64 as isize;
_7 = 11088_u16 as isize;
_31 = _62.2 - _15.6.4;
_45 = -_40.2.1;
_59.4 = !_39.4;
_41 = _15.6.3;
_39.0 = -_15.6.1;
_59.3 = (_14.0,);
_20 = _49;
SetDiscriminant(_12, 0);
_38 = [_39.4,_22.4];
_58.fld0.0 = [_40.2.1,_34,_40.2.1,_45,_58.fld2,_58.fld2];
_36 = _66 - _40.2.0;
_15.4.0 = [_58.fld2,_45,_40.2.1,_34,_58.fld2,_45];
_66 = _36 + _40.6.3;
_17 = Move(_57);
_39.1 = _59.1 - _59.1;
_62.0 = _9;
Goto(bb29)
}
bb29 = {
_15.6.2 = !_15.6.4;
_2.2 = _3;
_40.2 = _15.2;
_6 = -_45;
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1)).2 = !_15.6.4;
_40.1 = _39.1;
_46 = _26 as u64;
_73 = _58.fld0.2;
_71 = [_26,_26,_26];
_10 = _27.0;
_28 = _15.4;
_79 = _15.6;
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1)) = _40.6;
_15.2.1 = _6 | _6;
_13 = !_62.0;
_59.3 = (_14.0,);
_79.2 = !_1.4;
_71 = _58.fld0.2;
_22.2 = _59.2;
Goto(bb30)
}
bb30 = {
_58.fld0 = _2;
_59.2 = [_59.1,_39.1,_40.1,_59.1,_39.1,_22.1];
_81 = _27.0 < _27.0;
_24 = _22.2;
_15.1 = _48 as u128;
_15.6.3 = _66;
_44.0 = !_59.3.0;
_25 = [_81,_81];
_39.0 = !_1.1;
_1.1 = 43006_u16 as i16;
_39.0 = -_15.6.1;
place!(Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1)).2 = _31 & _62.2;
_15.4.0 = _40.4.0;
_22.3 = (_52,);
_67 = Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1).1;
_59.0 = _15.6.1;
_10 = _15.6.3 < _66;
_58.fld0.1 = _28.0;
_40.6 = (Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1).0, _59.0, _15.6.2, _66, Field::<(u64, i16, i128, f64, i128)>(Variant(_12, 0), 1).2);
_62.3 = _40.6.3 - _40.2.0;
_39.1 = _40.1 & _40.1;
_29 = _66 + _66;
_79.3 = _40.6.3 + _29;
_57 = &_69;
_2.2 = _71;
_64 = _40.1 + _39.1;
_15.3 = [_59.1,_59.1,_64,_64,_64,_64];
_62 = (_9, _40.6.1, _79.2, _15.6.3, _79.4);
_39.3 = (_59.3.0,);
_40.6.2 = !_31;
Goto(bb31)
}
bb31 = {
_5 = _73;
_2 = _58.fld0;
_14 = _44;
_58.fld0.1 = [_15.2.1,_34,_58.fld2,_15.2.1,_34,_15.2.1];
RET = core::ptr::addr_of!(_82);
_62.0 = _13 + _9;
Goto(bb32)
}
bb32 = {
Call(_86 = dump_var(14_usize, 14_usize, Move(_14), 48_usize, Move(_48), 6_usize, Move(_6), 51_usize, Move(_51)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_86 = dump_var(14_usize, 81_usize, Move(_81), 28_usize, Move(_28), 4_usize, Move(_4), 45_usize, Move(_45)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_86 = dump_var(14_usize, 73_usize, Move(_73), 27_usize, Move(_27), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_86 = dump_var(14_usize, 32_usize, Move(_32), 5_usize, Move(_5), 67_usize, Move(_67), 39_usize, Move(_39)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_86 = dump_var(14_usize, 25_usize, Move(_25), 13_usize, Move(_13), 38_usize, Move(_38), 18_usize, Move(_18)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: [isize; 6],mut _3: [i32; 3],mut _4: u64,mut _5: ([isize; 6], [isize; 6], [i32; 3]),mut _6: [isize; 6],mut _7: [isize; 6],mut _8: ([isize; 6], [isize; 6], [i32; 3]),mut _9: [isize; 6],mut _10: ([isize; 6], [isize; 6], [i32; 3]),mut _11: u64) -> i16 {
mir! {
type RET = i16;
let _12: f64;
let _13: i16;
let _14: char;
let _15: bool;
let _16: f64;
let _17: u64;
let _18: u32;
let _19: &'static f32;
let _20: [isize; 2];
let _21: f32;
let _22: (f64, isize);
let _23: Adt45;
let _24: f64;
let _25: Adt52;
let _26: f32;
let _27: (u64, i16, i128, f64, i128);
let _28: [usize; 8];
let _29: i8;
let _30: (bool,);
let _31: Adt42;
let _32: isize;
let _33: Adt51;
let _34: *const ([i32; 3],);
let _35: i32;
let _36: Adt42;
let _37: u32;
let _38: Adt49;
let _39: i8;
let _40: isize;
let _41: u32;
let _42: u16;
let _43: ([isize; 6],);
let _44: Adt40;
let _45: u8;
let _46: Adt40;
let _47: (i8,);
let _48: isize;
let _49: (i8,);
let _50: [u64; 6];
let _51: i8;
let _52: [u128; 6];
let _53: f64;
let _54: u32;
let _55: Adt42;
let _56: i8;
let _57: ();
let _58: ();
{
_5.1 = _8.0;
Goto(bb1)
}
bb1 = {
RET = 2_usize as i16;
_8.0 = _7;
_2 = [_1,_1,_1,_1,_1,_1];
RET = (-30024_i16);
_7 = _5.1;
_13 = RET;
RET = !_13;
_10.0 = [_1,_1,_1,_1,_1,_1];
_8.1 = [_1,_1,_1,_1,_1,_1];
_16 = 10098_u16 as f64;
_11 = !_4;
_12 = 1232552833_u32 as f64;
_13 = -RET;
_14 = '\u{3c494}';
Goto(bb2)
}
bb2 = {
_8.0 = [_1,_1,_1,_1,_1,_1];
_7 = _9;
_4 = _12 as u64;
_11 = _4;
_5.2 = [(-603266645_i32),(-314177984_i32),(-852661506_i32)];
_4 = !_11;
_5 = _8;
_13 = -RET;
_16 = _12;
_10.0 = [_1,_1,_1,_1,_1,_1];
_10.2 = [(-73552199_i32),1135639469_i32,(-1426420941_i32)];
Goto(bb3)
}
bb3 = {
_10.0 = _6;
_8.0 = [_1,_1,_1,_1,_1,_1];
_6 = _8.0;
_1 = (-9223372036854775808_isize);
RET = -_13;
_8.2 = [(-1544996656_i32),(-1040745413_i32),88885979_i32];
_17 = _11;
_10.0 = [_1,_1,_1,_1,_1,_1];
_1 = -(-9223372036854775808_isize);
_9 = [_1,_1,_1,_1,_1,_1];
_8.0 = _10.0;
_15 = false ^ true;
_8.0 = _10.1;
_4 = !_17;
_3 = [(-1046974609_i32),737515592_i32,(-1565161150_i32)];
_2 = [_1,_1,_1,_1,_1,_1];
_12 = _1 as f64;
_8.2 = [1011996128_i32,(-1156218321_i32),170739993_i32];
_10.0 = [_1,_1,_1,_1,_1,_1];
RET = _13 >> _13;
_14 = '\u{37d18}';
_10.2 = [1694552679_i32,340863106_i32,1566158026_i32];
_2 = [_1,_1,_1,_1,_1,_1];
_15 = false;
_13 = -RET;
_14 = '\u{255d1}';
_10.0 = [_1,_1,_1,_1,_1,_1];
_8.0 = [_1,_1,_1,_1,_1,_1];
Goto(bb4)
}
bb4 = {
_13 = -RET;
_8.2 = [238961101_i32,(-1663720132_i32),904412246_i32];
_8.2 = _10.2;
Goto(bb5)
}
bb5 = {
_5 = _10;
_22.1 = _15 as isize;
_3 = [698697228_i32,(-654365670_i32),(-2100251217_i32)];
_20 = [_22.1,_22.1];
_14 = '\u{89a3b}';
_5 = (_10.1, _8.0, _8.2);
_20 = [_22.1,_1];
_9 = _7;
_8.0 = _9;
_22 = (_16, _1);
_12 = -_22.0;
RET = -_13;
_10.1 = [_22.1,_22.1,_22.1,_22.1,_1,_1];
_15 = false;
_3 = _5.2;
_22 = (_16, _1);
_15 = false & false;
_7 = [_22.1,_22.1,_1,_22.1,_1,_22.1];
_21 = 61819_u16 as f32;
_16 = 2434900111_u32 as f64;
_8.2 = [(-1217862959_i32),524316144_i32,(-1221830924_i32)];
_5 = (_8.0, _8.1, _10.2);
RET = -_13;
_24 = _4 as f64;
Call(_24 = core::intrinsics::fmaf64(_22.0, _12, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19 = &_21;
_9 = _8.0;
_25.fld1.0 = _6;
_16 = 1964343242_u32 as f64;
_25.fld2.fld1 = _8.2;
_4 = _11;
RET = _13;
_10 = _8;
_25.fld2 = Adt49 { fld0: _5,fld1: _8.2,fld2: _1 };
_13 = RET;
_27.0 = !_4;
_12 = -_16;
Goto(bb7)
}
bb7 = {
_25.fld2.fld2 = _22.1 ^ _1;
_6 = [_25.fld2.fld2,_25.fld2.fld2,_25.fld2.fld2,_25.fld2.fld2,_1,_1];
_27.3 = _12 - _24;
_18 = 4945096048940617441_usize as u32;
_25.fld2 = Adt49 { fld0: _5,fld1: _3,fld2: _22.1 };
_27.2 = (-110373545520662914187648089274584143820_i128);
_12 = _18 as f64;
_25.fld1.0 = _25.fld2.fld0.0;
_25.fld1 = (_6,);
_20 = [_25.fld2.fld2,_25.fld2.fld2];
_19 = &(*_19);
_4 = _17;
_25.fld2.fld0.2 = _10.2;
_8.1 = _10.1;
_5.1 = [_22.1,_22.1,_25.fld2.fld2,_25.fld2.fld2,_22.1,_22.1];
_25.fld5 = -_27.2;
_25.fld2.fld2 = _27.3 as isize;
_28 = [15270632351113036057_usize,10943237330861323622_usize,1_usize,4_usize,6989333731224247368_usize,1482995848602806400_usize,6_usize,16813913492616888353_usize];
Goto(bb8)
}
bb8 = {
_6 = _25.fld2.fld0.1;
_7 = [_1,_1,_25.fld2.fld2,_1,_25.fld2.fld2,_1];
_3 = [(-1404986792_i32),311772518_i32,1972192819_i32];
_5 = (_9, _9, _10.2);
_25.fld2.fld0.2 = _8.2;
_28 = [6899960433056342970_usize,5_usize,5_usize,9378800519540123396_usize,0_usize,10565823459528758417_usize,11942764347191111066_usize,1_usize];
_25.fld2.fld0.2 = [(-1164768234_i32),(-826156941_i32),(-1066515817_i32)];
_27.1 = -RET;
_26 = RET as f32;
_1 = _13 as isize;
_24 = _22.0;
_2 = _5.1;
_17 = _27.0;
_25.fld2.fld0.0 = [_25.fld2.fld2,_1,_1,_1,_25.fld2.fld2,_25.fld2.fld2];
_25.fld4 = [48657263239180738493577937209014506294_u128,83550583361835559777789114532867337047_u128,203039895621084009358897549326203580788_u128,289512296257300058109444183077892040823_u128,277495159407229251697155300581179429814_u128,316420302691072538517203685683455218063_u128];
_25.fld2.fld0.1 = _5.0;
_25.fld2.fld0.0 = [_25.fld2.fld2,_25.fld2.fld2,_1,_25.fld2.fld2,_25.fld2.fld2,_25.fld2.fld2];
_10.1 = [_22.1,_1,_22.1,_25.fld2.fld2,_25.fld2.fld2,_1];
_5.1 = [_1,_25.fld2.fld2,_25.fld2.fld2,_25.fld2.fld2,_1,_22.1];
_5.1 = _2;
_25.fld2.fld0 = (_5.0, _2, _8.2);
_30.0 = _15;
_10.2 = _8.2;
_27 = (_11, _13, _25.fld5, _24, _25.fld5);
Goto(bb9)
}
bb9 = {
_8.2 = [(-43233693_i32),(-134754341_i32),1027838418_i32];
_11 = _4 & _4;
_2 = _9;
_26 = (*_19) + (*_19);
_2 = [_1,_1,_25.fld2.fld2,_22.1,_1,_22.1];
_30.0 = !_15;
_25.fld1.0 = [_22.1,_1,_1,_25.fld2.fld2,_1,_1];
_11 = _4;
_8 = (_25.fld2.fld0.0, _25.fld2.fld0.1, _5.2);
_30 = (_15,);
_4 = _14 as u64;
_8.1 = [_25.fld2.fld2,_1,_1,_25.fld2.fld2,_1,_1];
Call(_27.3 = core::intrinsics::transmute(_25.fld2.fld2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.2 = [81777582_i32,(-162641815_i32),(-1043252931_i32)];
_13 = !_27.1;
_9 = [_25.fld2.fld2,_25.fld2.fld2,_25.fld2.fld2,_22.1,_25.fld2.fld2,_25.fld2.fld2];
_30 = (_15,);
_25.fld2.fld0.0 = [_25.fld2.fld2,_1,_25.fld2.fld2,_22.1,_25.fld2.fld2,_1];
_10.2 = _3;
_25.fld2 = Adt49 { fld0: _10,fld1: _3,fld2: _1 };
_25.fld5 = _27.4 | _27.4;
_7 = _25.fld1.0;
_12 = -_22.0;
_40 = !_25.fld2.fld2;
_38.fld0.2 = [(-142376122_i32),(-685631103_i32),1237869622_i32];
_27 = (_4, RET, _25.fld5, _16, _25.fld5);
Call(_25.fld0 = fn16(_25.fld2.fld2, _10, _18), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_38.fld1 = [555361327_i32,(-713243554_i32),429174025_i32];
_27.2 = _18 as i128;
Goto(bb12)
}
bb12 = {
_2 = _6;
_35 = (-1800490123_i32) ^ 2141294404_i32;
_13 = RET;
_8.2 = [_35,_35,_35];
_38.fld0.2 = _10.2;
RET = 37_u8 as i16;
_27.0 = _17;
_43 = _25.fld1;
_38.fld0.0 = _25.fld2.fld0.0;
Goto(bb13)
}
bb13 = {
_27.3 = _22.0;
RET = _27.1 | _13;
_5 = (_25.fld1.0, _8.0, _38.fld1);
_11 = 224_u8 as u64;
_38.fld1 = [_35,_35,_35];
_16 = _24;
_8.1 = [_25.fld2.fld2,_25.fld2.fld2,_22.1,_1,_40,_1];
_30 = (_15,);
_32 = 9_i8 as isize;
_5.2 = [_35,_35,_35];
_25.fld2 = Adt49 { fld0: _8,fld1: _5.2,fld2: _40 };
_38.fld0.1 = [_40,_25.fld2.fld2,_40,_40,_40,_40];
_43.0 = _7;
_25.fld1 = (_25.fld2.fld0.1,);
_25.fld2.fld2 = _32;
_30 = (_15,);
_48 = _25.fld2.fld2;
_25.fld2 = Adt49 { fld0: _8,fld1: _3,fld2: _1 };
_48 = _25.fld2.fld2 << RET;
_8.1 = _5.1;
Goto(bb14)
}
bb14 = {
_30 = (_15,);
_10.1 = [_48,_48,_40,_48,_48,_48];
_52 = _25.fld4;
_38.fld0.0 = [_1,_48,_40,_25.fld2.fld2,_32,_48];
_22 = (_24, _48);
_17 = !_11;
_41 = _18 ^ _18;
_9 = [_22.1,_40,_1,_22.1,_22.1,_40];
_22.1 = _1;
_45 = 113_u8 * 125_u8;
_53 = _16;
_5 = (_10.1, _10.1, _25.fld2.fld0.2);
_10 = _38.fld0;
_20 = [_32,_48];
_17 = _4;
_38 = Adt49 { fld0: _8,fld1: _3,fld2: _1 };
_49 = ((-3_i8),);
_2 = _25.fld2.fld0.1;
_56 = _49.0 - _49.0;
_27.1 = _13;
_25.fld1.0 = [_22.1,_1,_22.1,_40,_40,_38.fld2];
_2 = _9;
_11 = _17 & _27.0;
Goto(bb15)
}
bb15 = {
Call(_57 = dump_var(15_usize, 9_usize, Move(_9), 1_usize, Move(_1), 8_usize, Move(_8), 56_usize, Move(_56)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(15_usize, 10_usize, Move(_10), 11_usize, Move(_11), 28_usize, Move(_28), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(15_usize, 17_usize, Move(_17), 49_usize, Move(_49), 15_usize, Move(_15), 45_usize, Move(_45)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_57 = dump_var(15_usize, 35_usize, Move(_35), 4_usize, Move(_4), 58_usize, _58, 58_usize, _58), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: ([isize; 6], [isize; 6], [i32; 3]),mut _3: u32) -> *const [i16; 3] {
mir! {
type RET = *const [i16; 3];
let _4: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _5: Adt42;
let _6: isize;
let _7: [u128; 6];
let _8: (i16, u128, [u128; 6], (i8,), u8);
let _9: Adt42;
let _10: u128;
let _11: f64;
let _12: Adt40;
let _13: Adt49;
let _14: ([isize; 6], [isize; 6], [i32; 3]);
let _15: char;
let _16: isize;
let _17: Adt42;
let _18: isize;
let _19: ([isize; 6], [isize; 6], [i32; 3]);
let _20: [i32; 3];
let _21: u16;
let _22: Adt43;
let _23: bool;
let _24: ([isize; 6], [isize; 6], [i32; 3]);
let _25: u16;
let _26: isize;
let _27: u64;
let _28: ([isize; 6], [isize; 6], [i32; 3]);
let _29: Adt48;
let _30: Adt39;
let _31: (i16, u128, [u128; 6], (i8,), u8);
let _32: i16;
let _33: Adt45;
let _34: [i32; 3];
let _35: i16;
let _36: Adt52;
let _37: Adt50;
let _38: Adt41;
let _39: *const [i16; 3];
let _40: isize;
let _41: bool;
let _42: Adt50;
let _43: Adt42;
let _44: [isize; 2];
let _45: isize;
let _46: bool;
let _47: Adt49;
let _48: Adt49;
let _49: f32;
let _50: [bool; 2];
let _51: isize;
let _52: isize;
let _53: u16;
let _54: Adt43;
let _55: char;
let _56: f32;
let _57: i64;
let _58: bool;
let _59: isize;
let _60: [isize; 2];
let _61: Adt53;
let _62: [u8; 2];
let _63: Adt45;
let _64: [usize; 8];
let _65: ([isize; 6], [isize; 6], [i32; 3]);
let _66: *const ([i32; 3],);
let _67: isize;
let _68: isize;
let _69: [usize; 8];
let _70: Adt42;
let _71: [usize; 8];
let _72: Adt40;
let _73: [bool; 2];
let _74: Adt48;
let _75: ();
let _76: ();
{
_2.1 = [_1,_1,_1,_1,_1,_1];
_2.2 = [(-358389938_i32),(-1987416701_i32),76202185_i32];
_4.6.2 = (-47921929189900866558838330619990248923_i128);
_4.4 = (_2.0,);
_4.2.0 = _1 as f64;
_4.2.0 = 1675809682_i32 as f64;
_3 = 2730407486_u32 ^ 2871173940_u32;
_4.5 = [6600711679781001839_u64,9884676176642732506_u64,23053564235051250_u64,13671137668901842381_u64,3152921234802933185_u64,10579653638733044176_u64];
_2.2 = [(-389117122_i32),1264233017_i32,923066948_i32];
_4.6.3 = _4.2.0;
_4.2.1 = -_1;
_3 = 3084468376_u32;
_3 = !4215789982_u32;
_6 = _1 ^ _1;
_4.6.1 = 3359_i16 | (-32333_i16);
_4.2 = (_4.6.3, _1);
_4.6.1 = _6 as i16;
_4.3 = [242558556359508779773409898783822531328_u128,137034019783612087961147369944409135496_u128,125372406139187914499275728156460295717_u128,264861817150294862901570849914158954858_u128,113681094909795811927239034986144804232_u128,336220780705248025012697756404927119439_u128];
_8.3 = (31_i8,);
match _8.3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
31 => bb6,
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
_1 = _6 ^ _6;
_1 = _4.2.1;
_4.6 = (7341166803249421036_u64, (-25199_i16), (-159700457507941674051383744987830288950_i128), _4.2.0, (-118266717946888893339823608275423357265_i128));
_8.4 = 81_u8 >> _6;
_4.1 = 52213249732457794382334005295067063629_u128;
_4.1 = 107389177570641113616390313121542918096_u128 ^ 103658546426439010836726437301419318810_u128;
_4.6.2 = _4.6.0 as i128;
Goto(bb7)
}
bb7 = {
_4.6.3 = _4.2.0 + _4.2.0;
_8.4 = !197_u8;
_8.3 = ((-58_i8),);
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_4.4 = (_2.1,);
_6 = _1 | _1;
_8.3 = (110_i8,);
_4.6.1 = !21780_i16;
_4.0 = [_4.2.1,_1];
_7 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_8.3.0 = 103_i8;
_4.4 = (_2.0,);
_4.2.1 = _6 >> _4.6.4;
_8.3.0 = (-115_i8);
_8.4 = 117_u8 | 176_u8;
_8.1 = _4.6.3 as u128;
_4.4.0 = [_4.2.1,_6,_4.2.1,_6,_6,_6];
_4.6.0 = 467897262919761836_u64 + 6183338822416975563_u64;
_8.3.0 = 70_i8 * 73_i8;
_7 = [_4.1,_8.1,_4.1,_8.1,_4.1,_4.1];
_4.2 = (_4.6.3, _6);
_4.2.0 = _4.6.4 as f64;
_4.4 = (_2.0,);
match _4.6.4 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb8,
222015648974049570123550999156344854191 => bb10,
_ => bb9
}
}
bb8 = {
_1 = _6 ^ _6;
_1 = _4.2.1;
_4.6 = (7341166803249421036_u64, (-25199_i16), (-159700457507941674051383744987830288950_i128), _4.2.0, (-118266717946888893339823608275423357265_i128));
_8.4 = 81_u8 >> _6;
_4.1 = 52213249732457794382334005295067063629_u128;
_4.1 = 107389177570641113616390313121542918096_u128 ^ 103658546426439010836726437301419318810_u128;
_4.6.2 = _4.6.0 as i128;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_8.2 = _4.3;
_14.0 = [_4.2.1,_4.2.1,_6,_4.2.1,_4.2.1,_4.2.1];
_14 = (_4.4.0, _4.4.0, _2.2);
_2.0 = [_4.2.1,_6,_1,_4.2.1,_4.2.1,_1];
_13.fld0.1 = [_4.2.1,_6,_1,_6,_4.2.1,_4.2.1];
_4.2.0 = _8.4 as f64;
match _4.6.4 {
0 => bb1,
1 => bb2,
2 => bb9,
222015648974049570123550999156344854191 => bb11,
_ => bb8
}
}
bb11 = {
_13.fld0.1 = [_4.2.1,_4.2.1,_4.2.1,_4.2.1,_1,_4.2.1];
_4.2 = (_4.6.3, _6);
_4.4 = (_2.0,);
_4.2 = (_4.6.3, _6);
_13.fld1 = [(-657956148_i32),(-140579619_i32),673272788_i32];
_13.fld0.2 = [1969104094_i32,(-1511088556_i32),401310656_i32];
_4.6.2 = _1 as i128;
_13.fld1 = _2.2;
_11 = _4.6.3 - _4.2.0;
_14.1 = _2.0;
_4.4 = (_2.0,);
_11 = _4.6.3;
_8.0 = 1973810515223273184_usize as i16;
_19.2 = [(-509575659_i32),121111516_i32,(-742708367_i32)];
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
match _4.6.4 {
0 => bb9,
222015648974049570123550999156344854191 => bb12,
_ => bb10
}
}
bb12 = {
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0];
_14.0 = [_6,_6,_1,_6,_1,_6];
_2.1 = _2.0;
_4.3 = [_8.1,_8.1,_4.1,_8.1,_4.1,_8.1];
_4.0 = [_6,_4.2.1];
_7 = [_8.1,_8.1,_4.1,_4.1,_4.1,_8.1];
_7 = _4.3;
_4.6.2 = _8.4 as i128;
_4.4 = (_14.0,);
_8.2 = [_8.1,_8.1,_4.1,_8.1,_8.1,_4.1];
match _4.6.4 {
0 => bb11,
1 => bb8,
2 => bb13,
222015648974049570123550999156344854191 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_4.2.0 = _11 + _11;
_10 = _4.1;
_13.fld0.0 = _2.0;
_4.6.3 = -_4.2.0;
_4.0 = [_1,_1];
_8.3.0 = (-713890784447905049_i64) as i8;
Goto(bb16)
}
bb16 = {
_8.4 = 210_u8 * 175_u8;
_4.6.4 = _4.6.2 | _4.6.2;
_14 = (_4.4.0, _2.1, _13.fld1);
_4.4.0 = [_6,_6,_4.2.1,_6,_6,_4.2.1];
_19.1 = [_4.2.1,_6,_4.2.1,_6,_4.2.1,_6];
_4.6.2 = _4.6.1 as i128;
_8.0 = _4.6.1;
_26 = true as isize;
_14.1 = [_1,_4.2.1,_4.2.1,_1,_4.2.1,_6];
_2.2 = [(-1276651544_i32),(-420466978_i32),749386673_i32];
_19.1 = _14.0;
_27 = !_4.6.0;
_8.3 = (55_i8,);
_16 = _8.0 as isize;
_15 = '\u{38073}';
_25 = 47087_u16;
_10 = _4.1 - _8.1;
_4.6.0 = _27;
_13.fld1 = [1822475845_i32,1814192721_i32,609511183_i32];
_24.2 = [506606058_i32,(-1851671755_i32),(-100723788_i32)];
match _25 {
47087 => bb18,
_ => bb17
}
}
bb17 = {
_1 = _6 ^ _6;
_1 = _4.2.1;
_4.6 = (7341166803249421036_u64, (-25199_i16), (-159700457507941674051383744987830288950_i128), _4.2.0, (-118266717946888893339823608275423357265_i128));
_8.4 = 81_u8 >> _6;
_4.1 = 52213249732457794382334005295067063629_u128;
_4.1 = 107389177570641113616390313121542918096_u128 ^ 103658546426439010836726437301419318810_u128;
_4.6.2 = _4.6.0 as i128;
Goto(bb7)
}
bb18 = {
_13 = Adt49 { fld0: _14,fld1: _19.2,fld2: _1 };
_4.6.3 = _3 as f64;
_31.0 = _4.6.1;
_31 = _8;
_15 = '\u{46428}';
_14.0 = _13.fld0.1;
_21 = _25 & _25;
_24.0 = _14.1;
_4.4 = (_14.1,);
_31.4 = !_8.4;
_26 = _21 as isize;
_24.2 = [(-1052771772_i32),(-1460139497_i32),1138405375_i32];
_29.fld1 = 10645307296426785632_usize * 11786873865059035011_usize;
_4.3 = _31.2;
_4.1 = !_10;
_4.2 = (_11, _13.fld2);
_4.2.0 = _10 as f64;
_25 = _21;
_28 = _14;
_20 = _24.2;
_24 = _28;
_4.6.4 = _4.6.2 * _4.6.2;
_16 = _4.6.0 as isize;
_19 = _28;
_8.1 = _10 & _4.1;
Goto(bb19)
}
bb19 = {
_13.fld0 = (_14.0, _2.1, _19.2);
_12 = Adt40::Variant1 { fld0: _25,fld1: _27,fld2: _2.0,fld3: _29.fld1,fld4: _4.4,fld5: _4.0 };
_4.3 = [_8.1,_10,_8.1,_8.1,_8.1,_8.1];
_14.2 = _2.2;
_4.5 = [Field::<u64>(Variant(_12, 1), 1),Field::<u64>(Variant(_12, 1), 1),_4.6.0,_4.6.0,_4.6.0,_4.6.0];
Goto(bb20)
}
bb20 = {
place!(Field::<u64>(Variant(_12, 1), 1)) = !_4.6.0;
_31.1 = _8.1;
_13.fld0.2 = [(-1228303165_i32),1896566794_i32,715358977_i32];
_28 = (_19.1, _24.0, _13.fld0.2);
_29.fld2 = [_8.4,_31.4];
_13.fld0.2 = [(-190611448_i32),859327866_i32,(-87164315_i32)];
_36.fld2.fld0 = (_4.4.0, _24.0, _20);
_29.fld3 = _12;
_36.fld2.fld2 = _4.6.2 as isize;
SetDiscriminant(_29.fld3, 0);
_4.4.0 = _2.1;
_14.0 = [_16,_6,_16,_6,_16,_13.fld2];
SetDiscriminant(_12, 1);
place!(Field::<([isize; 6],)>(Variant(_12, 1), 4)) = _4.4;
place!(Field::<[usize; 8]>(Variant(_29.fld3, 0), 2)) = [_29.fld1,_29.fld1,_29.fld1,_29.fld1,_29.fld1,_29.fld1,_29.fld1,_29.fld1];
_36.fld2.fld0 = (_2.0, _13.fld0.1, _20);
_8.2 = _4.3;
_11 = -_4.2.0;
place!(Field::<[isize; 6]>(Variant(_12, 1), 2)) = _28.1;
_23 = _11 > _4.6.3;
_4.6 = (_27, _8.0, 108780709760627184616712674109626682620_i128, _11, 145526939537161821308739194012132972302_i128);
place!(Field::<(bool,)>(Variant(_29.fld3, 0), 0)).0 = !_23;
Call(_43 = fn17(_36.fld2.fld0, _4.4.0, Field::<(bool,)>(Variant(_29.fld3, 0), 0), _4.0, _6), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
SetDiscriminant(_43, 1);
place!(Field::<([i32; 3],)>(Variant(_29.fld3, 0), 4)).0 = [1257137895_i32,554093331_i32,(-1747495759_i32)];
_29.fld2 = [_31.4,_31.4];
_14.1 = [_13.fld2,_13.fld2,_13.fld2,_1,_1,_4.2.1];
_39 = core::ptr::addr_of!(place!(Field::<[i16; 3]>(Variant(_43, 1), 0)));
_14.2 = [799735622_i32,(-1069603375_i32),1961556559_i32];
_36.fld1.0 = [_6,_16,_26,_16,_4.2.1,_13.fld2];
_23 = Field::<(bool,)>(Variant(_29.fld3, 0), 0).0;
_36.fld0 = core::ptr::addr_of!(place!(Field::<[i16; 3]>(Variant(_43, 1), 0)));
_31.3.0 = (-6341585011511639640_i64) as i8;
_31.4 = _31.3.0 as u8;
_36.fld2 = Adt49 { fld0: _13.fld0,fld1: _28.2,fld2: _4.2.1 };
_19 = (_24.1, _28.0, _24.2);
_32 = _8.0 * _31.0;
_14.0 = [_6,_13.fld2,_6,_13.fld2,_1,_26];
place!(Field::<u128>(Variant(_29.fld3, 0), 5)) = !_31.1;
place!(Field::<[i16; 3]>(Variant(_43, 1), 0)) = [_4.6.1,_32,_31.0];
_36.fld5 = _15 as i128;
_31.2 = _4.3;
_4.6.3 = _31.1 as f64;
_28 = _13.fld0;
place!(Field::<f64>(Variant(_29.fld3, 0), 1)) = _32 as f64;
place!(Field::<[isize; 2]>(Variant(_43, 1), 2)) = [_6,_1];
Goto(bb22)
}
bb22 = {
Goto(bb23)
}
bb23 = {
_4.2.1 = _6 >> _27;
_48 = Adt49 { fld0: _14,fld1: _20,fld2: _1 };
place!(Field::<u16>(Variant(_12, 1), 0)) = _21 - _21;
_41 = Field::<(bool,)>(Variant(_29.fld3, 0), 0).0;
_4.6 = (_27, _8.0, _36.fld5, _11, _36.fld5);
_4.1 = _8.1;
_13.fld0.2 = _36.fld2.fld0.2;
_32 = 1394675724_i32 as i16;
_47.fld0.0 = [_6,_4.2.1,_48.fld2,_48.fld2,_4.2.1,_4.2.1];
_14 = _24;
_48.fld0.1 = [_4.2.1,_4.2.1,_1,_1,_4.2.1,_4.2.1];
place!(Field::<([isize; 6],)>(Variant(_12, 1), 4)) = (_2.0,);
_36.fld2.fld0.2 = [1207444981_i32,752179576_i32,(-810509749_i32)];
_47.fld2 = _26 * _4.2.1;
_28.2 = [1539938936_i32,922809043_i32,(-188054124_i32)];
_44 = [_47.fld2,_47.fld2];
_4.5 = [_27,_4.6.0,_4.6.0,_4.6.0,_27,_27];
_31.2 = [_8.1,_31.1,_10,_10,_8.1,_10];
_2.0 = _19.1;
_48.fld2 = _4.1 as isize;
_39 = core::ptr::addr_of!(place!(Field::<[i16; 3]>(Variant(_43, 1), 0)));
_14.2 = _28.2;
_4.1 = _8.4 as u128;
_36.fld2.fld2 = _11 as isize;
Goto(bb24)
}
bb24 = {
_36.fld2.fld0.0 = [_47.fld2,_6,_13.fld2,_47.fld2,_6,_47.fld2];
match _8.3.0 {
55 => bb25,
_ => bb8
}
}
bb25 = {
_31.4 = _8.4;
_51 = !_47.fld2;
_47.fld0.0 = [_36.fld2.fld2,_36.fld2.fld2,_4.2.1,_6,_47.fld2,_47.fld2];
_44 = [_51,_47.fld2];
_13.fld0.1 = [_6,_16,_51,_51,_4.2.1,_47.fld2];
_4.6.4 = _4.6.2;
_31.4 = !_8.4;
_4.3 = [Field::<u128>(Variant(_29.fld3, 0), 5),Field::<u128>(Variant(_29.fld3, 0), 5),_31.1,_8.1,_8.1,_31.1];
_47.fld0.1 = _19.1;
place!(Field::<[isize; 2]>(Variant(_43, 1), 2)) = [_26,_4.2.1];
_4.6.0 = !_27;
_4.4 = Field::<([isize; 6],)>(Variant(_12, 1), 4);
_31.3 = _8.3;
_31.0 = !_8.0;
_4.6.4 = !_36.fld5;
_19.1 = [_47.fld2,_6,_51,_1,_47.fld2,_48.fld2];
_44 = [_26,_4.2.1];
_17 = Adt42::Variant1 { fld0: (*_39),fld1: _4.6.4,fld2: Field::<[isize; 2]>(Variant(_43, 1), 2) };
_36.fld2.fld2 = _6;
_31.3.0 = _3 as i8;
_31.2 = _8.2;
_19.0 = _47.fld0.0;
_2.2 = [1232084274_i32,1823332937_i32,(-2058054275_i32)];
_28.2 = _48.fld1;
match _8.3.0 {
55 => bb26,
_ => bb14
}
}
bb26 = {
_29.fld2 = [_31.4,_31.4];
place!(Field::<u64>(Variant(_12, 1), 1)) = _27 + _4.6.0;
_2.1 = [_4.2.1,_26,_13.fld2,_26,_4.2.1,_48.fld2];
_56 = _32 as f32;
SetDiscriminant(_17, 1);
place!(Field::<(bool,)>(Variant(_29.fld3, 0), 0)).0 = _10 <= _31.1;
_6 = _47.fld2 >> Field::<u128>(Variant(_29.fld3, 0), 5);
_4.6 = (_27, _8.0, _36.fld5, _4.2.0, _36.fld5);
place!(Field::<[isize; 2]>(Variant(_43, 1), 2)) = [_51,_51];
_48.fld1 = Field::<([i32; 3],)>(Variant(_29.fld3, 0), 4).0;
_12 = Adt40::Variant1 { fld0: _21,fld1: _27,fld2: _14.0,fld3: _29.fld1,fld4: _4.4,fld5: Field::<[isize; 2]>(Variant(_43, 1), 2) };
_4.6 = (_27, _8.0, _36.fld5, _4.2.0, _36.fld5);
SetDiscriminant(_12, 1);
_5 = Adt42::Variant1 { fld0: (*_39),fld1: _4.6.4,fld2: Field::<[isize; 2]>(Variant(_43, 1), 2) };
_14.1 = [_6,_13.fld2,_1,_48.fld2,_47.fld2,_1];
place!(Field::<[isize; 2]>(Variant(_12, 1), 5)) = Field::<[isize; 2]>(Variant(_5, 1), 2);
place!(Field::<[u8; 2]>(Variant(_29.fld3, 0), 3)) = [_8.4,_31.4];
Goto(bb27)
}
bb27 = {
_4.4.0 = [_6,_6,_36.fld2.fld2,_13.fld2,_26,_47.fld2];
_40 = _1 + _26;
_3 = 1242299959_u32 - 4187026947_u32;
_59 = _6 >> _47.fld2;
_43 = Move(_5);
_5 = Adt42::Variant1 { fld0: Field::<[i16; 3]>(Variant(_43, 1), 0),fld1: _36.fld5,fld2: _44 };
_36.fld4 = [_10,_8.1,Field::<u128>(Variant(_29.fld3, 0), 5),_31.1,_8.1,_31.1];
_36.fld2.fld0.2 = [1794187207_i32,(-236779261_i32),81275829_i32];
place!(Field::<f64>(Variant(_29.fld3, 0), 1)) = _4.2.0;
_4.5 = [_4.6.0,_4.6.0,_4.6.0,_4.6.0,_4.6.0,_27];
_4.6.1 = _31.0;
_34 = [(-375269545_i32),(-1085108658_i32),(-995185715_i32)];
Call(_60 = core::intrinsics::transmute(_8.1), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_19 = (_13.fld0.0, _13.fld0.1, Field::<([i32; 3],)>(Variant(_29.fld3, 0), 4).0);
_28.1 = _36.fld2.fld0.0;
_50 = [Field::<(bool,)>(Variant(_29.fld3, 0), 0).0,_23];
_28.1 = [_6,_59,_6,_48.fld2,_51,_51];
_10 = _31.3.0 as u128;
_31 = _8;
_65.2 = [(-129139492_i32),389357712_i32,(-1082825968_i32)];
place!(Field::<[u8; 2]>(Variant(_29.fld3, 0), 3)) = _29.fld2;
_65.0 = [_4.2.1,_59,_59,_6,_59,_6];
_13.fld0.1 = _19.1;
place!(Field::<(bool,)>(Variant(_29.fld3, 0), 0)) = (_41,);
_35 = _56 as i16;
SetDiscriminant(_43, 1);
_47.fld0.1 = _65.0;
_9 = Move(_5);
SetDiscriminant(_9, 1);
place!(Field::<([isize; 6],)>(Variant(_12, 1), 4)).0 = [_40,_4.2.1,_48.fld2,_36.fld2.fld2,_4.2.1,_4.2.1];
_31.4 = _29.fld1 as u8;
_36.fld1.0 = [_36.fld2.fld2,_48.fld2,_6,_48.fld2,_59,_36.fld2.fld2];
_28 = (_13.fld0.0, _19.1, _13.fld0.2);
_10 = _25 as u128;
Goto(bb29)
}
bb29 = {
place!(Field::<i128>(Variant(_17, 1), 1)) = -_36.fld5;
_25 = !_21;
_47.fld0 = _13.fld0;
place!(Field::<[isize; 2]>(Variant(_9, 1), 2)) = Field::<[isize; 2]>(Variant(_12, 1), 5);
_31.4 = _8.4;
_19 = (_36.fld1.0, _4.4.0, _13.fld0.2);
place!(Field::<[isize; 2]>(Variant(_43, 1), 2)) = _44;
match _31.3.0 {
0 => bb12,
55 => bb30,
_ => bb17
}
}
bb30 = {
place!(Field::<[bool; 2]>(Variant(_29.fld3, 0), 6)) = [Field::<(bool,)>(Variant(_29.fld3, 0), 0).0,_23];
_20 = [1597744779_i32,(-642986443_i32),1858035151_i32];
_74.fld3 = _29.fld3;
place!(Field::<i128>(Variant(_9, 1), 1)) = -Field::<i128>(Variant(_17, 1), 1);
_4.4.0 = [_48.fld2,_4.2.1,_59,_36.fld2.fld2,_59,_6];
place!(Field::<([i32; 3],)>(Variant(_74.fld3, 0), 4)).0 = [(-510668278_i32),(-178501500_i32),1271326419_i32];
_16 = _8.3.0 as isize;
SetDiscriminant(_29.fld3, 1);
_47.fld0 = _28;
_48 = _36.fld2;
_4.6.3 = -Field::<f64>(Variant(_74.fld3, 0), 1);
SetDiscriminant(_74.fld3, 0);
_32 = _35;
_74.fld3 = Adt40::Variant1 { fld0: _25,fld1: _27,fld2: _14.1,fld3: _29.fld1,fld4: _36.fld1,fld5: Field::<[isize; 2]>(Variant(_12, 1), 5) };
_47.fld0 = _19;
_36.fld4 = [_31.1,_31.1,_8.1,_10,_4.1,_10];
_46 = !_23;
RET = core::ptr::addr_of!(place!(Field::<[i16; 3]>(Variant(_9, 1), 0)));
_24.0 = [_48.fld2,_47.fld2,_1,_13.fld2,_6,_59];
_74.fld2 = _29.fld2;
_13.fld0 = (_65.0, Field::<([isize; 6],)>(Variant(_74.fld3, 1), 4).0, _24.2);
Goto(bb31)
}
bb31 = {
Call(_75 = dump_var(16_usize, 23_usize, Move(_23), 41_usize, Move(_41), 40_usize, Move(_40), 15_usize, Move(_15)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_75 = dump_var(16_usize, 60_usize, Move(_60), 21_usize, Move(_21), 35_usize, Move(_35), 6_usize, Move(_6)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_75 = dump_var(16_usize, 27_usize, Move(_27), 3_usize, Move(_3), 2_usize, Move(_2), 59_usize, Move(_59)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_75 = dump_var(16_usize, 25_usize, Move(_25), 7_usize, Move(_7), 44_usize, Move(_44), 76_usize, _76), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: ([isize; 6], [isize; 6], [i32; 3]),mut _2: [isize; 6],mut _3: (bool,),mut _4: [isize; 2],mut _5: isize) -> Adt42 {
mir! {
type RET = Adt42;
let _6: i8;
let _7: Adt49;
let _8: [i16; 3];
let _9: isize;
let _10: [bool; 2];
let _11: *mut bool;
let _12: Adt41;
let _13: bool;
let _14: ([isize; 6],);
let _15: isize;
let _16: [isize; 2];
let _17: u8;
let _18: i128;
let _19: isize;
let _20: [usize; 8];
let _21: bool;
let _22: [bool; 2];
let _23: (i16, u128, [u128; 6], (i8,), u8);
let _24: Adt46;
let _25: (i16, u128, [u128; 6], (i8,), u8);
let _26: (i16, u128, [u128; 6], (i8,), u8);
let _27: [u8; 2];
let _28: [isize; 6];
let _29: [u128; 6];
let _30: isize;
let _31: u16;
let _32: Adt42;
let _33: isize;
let _34: ([i32; 3],);
let _35: u16;
let _36: (i8,);
let _37: isize;
let _38: (u64, i16, i128, f64, i128);
let _39: Adt51;
let _40: ();
let _41: ();
{
_1.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_6 = !(-33_i8);
Goto(bb1)
}
bb1 = {
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_7.fld0.2 = [1563629367_i32,1905622522_i32,(-310234024_i32)];
_7.fld0 = (_2, _1.0, _1.2);
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_1.1 = _7.fld0.1;
_7.fld1 = [2143643298_i32,609447534_i32,1774505359_i32];
_9 = _5;
_4 = [_5,_9];
_5 = _3.0 as isize;
_7.fld0.2 = _1.2;
_9 = _5;
_1 = (_7.fld0.0, _7.fld0.0, _7.fld0.2);
_7.fld0 = _1;
_3 = (true,);
Goto(bb2)
}
bb2 = {
_1.2 = _7.fld0.2;
_10 = [_3.0,_3.0];
_2 = _1.1;
_5 = 321808776293195122382959239148380563714_u128 as isize;
_3.0 = true;
_7.fld2 = _9;
_7.fld0.2 = [1268796652_i32,(-1209327172_i32),(-842200570_i32)];
_6 = 6_i8;
_7 = Adt49 { fld0: _1,fld1: _1.2,fld2: _9 };
_11 = core::ptr::addr_of_mut!(_3.0);
_2 = [_9,_9,_9,_9,_9,_7.fld2];
_6 = 21_i8;
_7.fld0 = (_2, _1.1, _1.2);
_7.fld0.0 = [_5,_5,_9,_9,_9,_7.fld2];
_1.2 = [(-1287618276_i32),(-1085924187_i32),345121781_i32];
_1.1 = _1.0;
_7.fld0.2 = [(-1636727562_i32),1388912067_i32,1670736915_i32];
_7.fld0.0 = [_9,_9,_5,_7.fld2,_9,_9];
_1.2 = [(-1500200171_i32),195314592_i32,(-1809083821_i32)];
_9 = !_7.fld2;
_1.0 = [_7.fld2,_9,_7.fld2,_7.fld2,_5,_5];
_5 = _7.fld2;
_13 = _3.0;
_3 = (_13,);
_14 = (_1.0,);
_1 = _7.fld0;
_7.fld0 = _1;
match _6 {
21 => bb4,
_ => bb3
}
}
bb3 = {
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_7.fld0.2 = [1563629367_i32,1905622522_i32,(-310234024_i32)];
_7.fld0 = (_2, _1.0, _1.2);
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_1.1 = _7.fld0.1;
_7.fld1 = [2143643298_i32,609447534_i32,1774505359_i32];
_9 = _5;
_4 = [_5,_9];
_5 = _3.0 as isize;
_7.fld0.2 = _1.2;
_9 = _5;
_1 = (_7.fld0.0, _7.fld0.0, _7.fld0.2);
_7.fld0 = _1;
_3 = (true,);
Goto(bb2)
}
bb4 = {
_4 = [_7.fld2,_9];
_8 = [(-24521_i16),(-20225_i16),24696_i16];
_7.fld2 = -_9;
_13 = _5 != _5;
match _6 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
21 => bb9,
_ => bb8
}
}
bb5 = {
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_7.fld0.2 = [1563629367_i32,1905622522_i32,(-310234024_i32)];
_7.fld0 = (_2, _1.0, _1.2);
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_1.1 = _7.fld0.1;
_7.fld1 = [2143643298_i32,609447534_i32,1774505359_i32];
_9 = _5;
_4 = [_5,_9];
_5 = _3.0 as isize;
_7.fld0.2 = _1.2;
_9 = _5;
_1 = (_7.fld0.0, _7.fld0.0, _7.fld0.2);
_7.fld0 = _1;
_3 = (true,);
Goto(bb2)
}
bb6 = {
_1.2 = _7.fld0.2;
_10 = [_3.0,_3.0];
_2 = _1.1;
_5 = 321808776293195122382959239148380563714_u128 as isize;
_3.0 = true;
_7.fld2 = _9;
_7.fld0.2 = [1268796652_i32,(-1209327172_i32),(-842200570_i32)];
_6 = 6_i8;
_7 = Adt49 { fld0: _1,fld1: _1.2,fld2: _9 };
_11 = core::ptr::addr_of_mut!(_3.0);
_2 = [_9,_9,_9,_9,_9,_7.fld2];
_6 = 21_i8;
_7.fld0 = (_2, _1.1, _1.2);
_7.fld0.0 = [_5,_5,_9,_9,_9,_7.fld2];
_1.2 = [(-1287618276_i32),(-1085924187_i32),345121781_i32];
_1.1 = _1.0;
_7.fld0.2 = [(-1636727562_i32),1388912067_i32,1670736915_i32];
_7.fld0.0 = [_9,_9,_5,_7.fld2,_9,_9];
_1.2 = [(-1500200171_i32),195314592_i32,(-1809083821_i32)];
_9 = !_7.fld2;
_1.0 = [_7.fld2,_9,_7.fld2,_7.fld2,_5,_5];
_5 = _7.fld2;
_13 = _3.0;
_3 = (_13,);
_14 = (_1.0,);
_1 = _7.fld0;
_7.fld0 = _1;
match _6 {
21 => bb4,
_ => bb3
}
}
bb7 = {
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_7.fld0.2 = [1563629367_i32,1905622522_i32,(-310234024_i32)];
_7.fld0 = (_2, _1.0, _1.2);
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_1.1 = _7.fld0.1;
_7.fld1 = [2143643298_i32,609447534_i32,1774505359_i32];
_9 = _5;
_4 = [_5,_9];
_5 = _3.0 as isize;
_7.fld0.2 = _1.2;
_9 = _5;
_1 = (_7.fld0.0, _7.fld0.0, _7.fld0.2);
_7.fld0 = _1;
_3 = (true,);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_8 = [2823_i16,32574_i16,22447_i16];
Goto(bb10)
}
bb10 = {
_10 = [_3.0,_13];
(*_11) = !_13;
Goto(bb11)
}
bb11 = {
_4 = [_9,_5];
(*_11) = !_13;
_14.0 = [_7.fld2,_9,_9,_7.fld2,_7.fld2,_5];
_7.fld0.1 = _14.0;
_7.fld0 = (_1.1, _1.1, _7.fld1);
_16 = [_9,_9];
_18 = 12718_u16 as i128;
_1 = _7.fld0;
_18 = 95850385758296356999230569698815341669_i128;
RET = Adt42::Variant1 { fld0: _8,fld1: _18,fld2: _4 };
_7.fld1 = [(-53636765_i32),(-1396096479_i32),700759838_i32];
_5 = _7.fld2 | _9;
place!(Field::<[i16; 3]>(Variant(RET, 1), 0)) = _8;
_3 = (_13,);
_20 = [7136628969368657961_usize,5_usize,7_usize,2_usize,2_usize,7_usize,3515109609343781638_usize,6_usize];
SetDiscriminant(RET, 0);
_13 = (*_11);
place!(Field::<[u128; 6]>(Variant(RET, 0), 2)) = [64783045822449229333531667348614716447_u128,68615545735068792224428631183844593237_u128,33733654315556756659245596605728031554_u128,132984138158192366897574188356756680682_u128,305587042742535752065080227190026908758_u128,176012620045188740160441372984075016393_u128];
place!(Field::<[u128; 6]>(Variant(RET, 0), 2)) = [113341715282651016850364424407004592421_u128,34915397507017799922543110531925874355_u128,139932703898980091375534351721172677423_u128,2299974605069132329753591131325274180_u128,80972162919470802597484538239051120377_u128,318731511833179156885030567186760128580_u128];
_10 = [(*_11),(*_11)];
_1.1 = [_9,_5,_5,_5,_9,_9];
_7.fld0.2 = _7.fld1;
_22 = [(*_11),_13];
place!(Field::<[bool; 2]>(Variant(RET, 0), 3)) = [(*_11),(*_11)];
Call(_19 = fn18(_22, _5, _1, _7, _22, _3, _3.0, _4, _7.fld0.1, _3.0, (*_11), _1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<[i32; 3]>(Variant(RET, 0), 5)) = [(-458561870_i32),(-82620783_i32),(-386478594_i32)];
_1.1 = [_9,_7.fld2,_7.fld2,_19,_7.fld2,_7.fld2];
_15 = _9;
_1.1 = _7.fld0.0;
place!(Field::<([i32; 3],)>(Variant(RET, 0), 0)).0 = [(-227893324_i32),(-1698767630_i32),1845645926_i32];
_14 = (_2,);
_3 = (_13,);
place!(Field::<i64>(Variant(RET, 0), 1)) = _6 as i64;
match _18 {
0 => bb1,
1 => bb13,
95850385758296356999230569698815341669 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_4 = [_5,_5];
_7.fld0.2 = [1563629367_i32,1905622522_i32,(-310234024_i32)];
_7.fld0 = (_2, _1.0, _1.2);
_7.fld0.1 = [_5,_5,_5,_5,_5,_5];
_1.1 = _7.fld0.1;
_7.fld1 = [2143643298_i32,609447534_i32,1774505359_i32];
_9 = _5;
_4 = [_5,_9];
_5 = _3.0 as isize;
_7.fld0.2 = _1.2;
_9 = _5;
_1 = (_7.fld0.0, _7.fld0.0, _7.fld0.2);
_7.fld0 = _1;
_3 = (true,);
Goto(bb2)
}
bb15 = {
place!(Field::<([i32; 3],)>(Variant(RET, 0), 0)).0 = [(-1537990613_i32),(-2118417937_i32),(-859375232_i32)];
place!(Field::<([i32; 3],)>(Variant(RET, 0), 0)).0 = [(-1413172963_i32),(-1781901739_i32),(-70588230_i32)];
_25.2 = [134211611538193621604987604790035956080_u128,204952662887293051251553509167873053740_u128,75209526010767385331527059417272477182_u128,293526554848902876113447243193024573265_u128,91839685922398362688852948885347372074_u128,263658458004943734950255093000769550986_u128];
_23.2 = [193058421156607064577246892362663311418_u128,320397899850721740940566741275121821880_u128,129774269015511710047380395801417241287_u128,207633103369246014060200037140011955440_u128,234935713134439302249606745854020237905_u128,100144079708034802079801308421346556849_u128];
Goto(bb16)
}
bb16 = {
_23.1 = !114155759325613965087249175224259326149_u128;
_4 = [_7.fld2,_15];
_3 = (_13,);
_23.0 = 19925_i16;
_23.0 = 19119_i16;
_17 = 197_u8 | 23_u8;
_7.fld1 = [(-1282480795_i32),(-1788017876_i32),(-88158413_i32)];
place!(Field::<i64>(Variant(RET, 0), 1)) = !8020382497963558272_i64;
Goto(bb17)
}
bb17 = {
place!(Field::<[i32; 3]>(Variant(RET, 0), 5)) = Field::<([i32; 3],)>(Variant(RET, 0), 0).0;
_21 = (*_11) == (*_11);
RET = Adt42::Variant1 { fld0: _8,fld1: _18,fld2: _16 };
_26.0 = _23.0 | _23.0;
RET = Adt42::Variant1 { fld0: _8,fld1: _18,fld2: _16 };
_7.fld0.0 = [_9,_15,_15,_5,_5,_9];
_1.2 = [1442807077_i32,479832156_i32,(-810423054_i32)];
_1.1 = _2;
_7.fld0.1 = _1.1;
RET = Adt42::Variant1 { fld0: _8,fld1: _18,fld2: _16 };
_14.0 = _1.1;
_27 = [_17,_17];
_14 = (_1.0,);
_25.0 = -_26.0;
_26.1 = _23.1;
_19 = _15 | _15;
_3 = (_21,);
_23.3 = (_6,);
_21 = !(*_11);
_23.3.0 = !_6;
_7.fld0 = (_1.1, _14.0, _1.2);
(*_11) = _21;
_25.4 = _17;
_18 = Field::<i128>(Variant(RET, 1), 1) * Field::<i128>(Variant(RET, 1), 1);
SetDiscriminant(RET, 1);
Goto(bb18)
}
bb18 = {
Goto(bb19)
}
bb19 = {
_22 = [_21,_21];
_7.fld0 = (_1.1, _2, _1.2);
Call(place!(Field::<i128>(Variant(RET, 1), 1)) = core::intrinsics::transmute(_16), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_7.fld2 = -_19;
_8 = [_26.0,_25.0,_26.0];
_26.3 = (_23.3.0,);
_17 = _18 as u8;
_23 = (_25.0, _26.1, _25.2, _26.3, _17);
_31 = 30495_u16;
_26.1 = _23.1;
_7.fld0.2 = [1674896827_i32,407810811_i32,1459698115_i32];
_27 = [_25.4,_25.4];
_1.1 = [_5,_15,_7.fld2,_19,_7.fld2,_5];
_23.3 = _26.3;
place!(Field::<[i16; 3]>(Variant(RET, 1), 0)) = [_23.0,_26.0,_23.0];
_18 = Field::<i128>(Variant(RET, 1), 1);
_23.0 = -_26.0;
_7.fld0 = (_14.0, _1.1, _1.2);
_23.3 = (_6,);
_1.2 = [(-1271280697_i32),(-376655169_i32),1322748001_i32];
_35 = _31 + _31;
_30 = _15;
place!(Field::<[isize; 2]>(Variant(RET, 1), 2)) = _16;
_25.4 = 1076187051709996552_i64 as u8;
_25.0 = !_26.0;
_23.2 = _25.2;
_38.0 = 1401751636738746491_u64 + 2727809671237918385_u64;
_23.3 = _26.3;
Goto(bb21)
}
bb21 = {
Call(_40 = dump_var(17_usize, 10_usize, Move(_10), 2_usize, Move(_2), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_40 = dump_var(17_usize, 21_usize, Move(_21), 14_usize, Move(_14), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_40 = dump_var(17_usize, 1_usize, Move(_1), 15_usize, Move(_15), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [bool; 2],mut _2: isize,mut _3: ([isize; 6], [isize; 6], [i32; 3]),mut _4: Adt49,mut _5: [bool; 2],mut _6: (bool,),mut _7: bool,mut _8: [isize; 2],mut _9: [isize; 6],mut _10: bool,mut _11: bool,mut _12: ([isize; 6], [isize; 6], [i32; 3])) -> isize {
mir! {
type RET = isize;
let _13: isize;
let _14: bool;
let _15: Adt50;
let _16: Adt43;
let _17: i8;
let _18: i32;
let _19: [isize; 2];
let _20: bool;
let _21: isize;
let _22: [isize; 6];
let _23: i8;
let _24: f64;
let _25: f64;
let _26: u128;
let _27: (f64, isize);
let _28: *mut bool;
let _29: [bool; 2];
let _30: [u8; 2];
let _31: ([isize; 6], [isize; 6], [i32; 3]);
let _32: *const [i16; 3];
let _33: Adt40;
let _34: [u8; 2];
let _35: [isize; 6];
let _36: Adt53;
let _37: [u128; 6];
let _38: usize;
let _39: Adt40;
let _40: f32;
let _41: isize;
let _42: ();
let _43: ();
{
_1 = [_7,_7];
_6.0 = _11;
_12.1 = [_2,_2,_4.fld2,_4.fld2,_2,_4.fld2];
_9 = [_2,_2,_2,_2,_2,_2];
RET = !_4.fld2;
_4.fld0.0 = [_2,_2,RET,_2,_4.fld2,RET];
_3 = (_4.fld0.0, _12.0, _12.2);
_2 = 101237402862253627_i64 as isize;
_7 = _11;
_12.2 = _4.fld1;
_6.0 = !_11;
_4.fld2 = -RET;
_8 = [_2,_4.fld2];
_14 = !_6.0;
_11 = !_14;
_3 = _4.fld0;
_4.fld1 = [(-1734276864_i32),1945788567_i32,602315476_i32];
_12 = _4.fld0;
Goto(bb1)
}
bb1 = {
RET = !_2;
_4.fld2 = !RET;
_12.1 = [RET,_4.fld2,_2,_2,RET,_2];
RET = -_4.fld2;
_6 = (_11,);
_12.2 = _4.fld1;
_8 = [_4.fld2,RET];
_3.0 = [_2,_4.fld2,_4.fld2,RET,_2,_4.fld2];
_4.fld0.0 = _12.0;
_18 = 2020565914_i32;
_12.0 = [_2,_4.fld2,_4.fld2,_4.fld2,_2,_2];
_18 = (-1771432955_i32);
_12.1 = [RET,_4.fld2,_2,RET,_4.fld2,_2];
_4 = Adt49 { fld0: _12,fld1: _3.2,fld2: RET };
_7 = _6.0;
_8 = [_4.fld2,_4.fld2];
_4.fld2 = 151744866521258623898458823356049201156_i128 as isize;
match _18 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607429996778501 => bb8,
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
_13 = _4.fld2 - _2;
_12.1 = [_13,_13,_13,_2,_2,RET];
_3.2 = [_18,_18,_18];
_12.2 = [_18,_18,_18];
_3.1 = _4.fld0.1;
_13 = 1985226430_u32 as isize;
_4.fld2 = _2;
_3 = (_9, _9, _4.fld0.2);
_12.0 = [_4.fld2,RET,_4.fld2,_2,RET,_13];
_24 = 39043_u16 as f64;
_7 = _6.0;
_13 = -_4.fld2;
_8 = [_2,_4.fld2];
Call(_12.2 = core::intrinsics::transmute(_4.fld1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_22 = _9;
_5 = [_10,_7];
_12 = (_9, _3.0, _3.2);
_19 = [_4.fld2,RET];
_12.1 = [_2,RET,_13,RET,RET,_4.fld2];
_19 = [_4.fld2,_2];
_26 = 260722309715536272135489994136665065383_u128;
_10 = _14;
_4.fld2 = (-1203_i16) as isize;
_6 = (_7,);
_3 = (_9, _9, _4.fld1);
RET = _13;
_4 = Adt49 { fld0: _3,fld1: _12.2,fld2: _2 };
_23 = 110_i8;
_9 = [_13,_4.fld2,_13,_4.fld2,_4.fld2,_13];
_4.fld2 = -_13;
_4.fld0.2 = [_18,_18,_18];
_6.0 = _11;
_8 = _19;
_27 = (_24, _4.fld2);
_21 = 19175_i16 as isize;
_24 = -_27.0;
_7 = _11;
_26 = !102140116933551428162890728744355006965_u128;
match _23 {
0 => bb1,
1 => bb3,
110 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_6 = (_10,);
_29 = [_10,_7];
_26 = 3880322201_u32 as u128;
_2 = -_13;
match _23 {
110 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_22 = _4.fld0.0;
_6.0 = _23 > _23;
_26 = 144063585188809789996903720890664392783_u128;
_6.0 = _11;
_29 = [_10,_10];
_4.fld0.0 = _3.1;
_12.1 = _9;
_20 = _6.0;
_4 = Adt49 { fld0: _12,fld1: _3.2,fld2: _2 };
_29 = [_10,_20];
RET = _27.1 * _4.fld2;
_30 = [129_u8,228_u8];
_18 = _23 as i32;
_4.fld0.2 = _12.2;
_21 = 8_u8 as isize;
_19 = [RET,_13];
_31.1 = [_4.fld2,_21,RET,_27.1,RET,_21];
_35 = [_2,_27.1,_4.fld2,_13,_21,RET];
_3 = (_12.0, _12.0, _4.fld1);
match _26 {
0 => bb14,
144063585188809789996903720890664392783 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_29 = [_7,_20];
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(18_usize, 18_usize, Move(_18), 23_usize, Move(_23), 30_usize, Move(_30), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(18_usize, 10_usize, Move(_10), 5_usize, Move(_5), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(18_usize, 11_usize, Move(_11), 1_usize, Move(_1), 21_usize, Move(_21), 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [isize; 2],mut _2: usize,mut _3: isize,mut _4: [u64; 6],mut _5: i128,mut _6: [isize; 2],mut _7: isize) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _8: char;
let _9: *mut bool;
let _10: (f64, isize);
let _11: Adt42;
let _12: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128));
let _13: (bool,);
let _14: i128;
let _15: [u64; 6];
let _16: ([i32; 3],);
let _17: char;
let _18: u8;
let _19: bool;
let _20: Adt41;
let _21: Adt50;
let _22: *const ([i32; 3],);
let _23: ([isize; 6],);
let _24: isize;
let _25: i128;
let _26: Adt52;
let _27: Adt41;
let _28: f64;
let _29: Adt45;
let _30: [i32; 3];
let _31: bool;
let _32: [usize; 8];
let _33: isize;
let _34: u64;
let _35: ([isize; 6],);
let _36: [isize; 2];
let _37: Adt49;
let _38: Adt47;
let _39: f64;
let _40: Adt46;
let _41: ();
let _42: ();
{
_7 = _3 >> _2;
_7 = _3 + _3;
_3 = _7;
Goto(bb1)
}
bb1 = {
_4 = [15127101695517071698_u64,5915900979908855176_u64,16462855709432864_u64,13996068578791381175_u64,10441877940284052595_u64,638442821841331573_u64];
RET = [1739832125_i32,748670640_i32,39758538_i32];
_8 = '\u{1a320}';
_4 = [12040270343367718037_u64,7978125421720373033_u64,760254451840831497_u64,5993833645126881513_u64,6624525339970706232_u64,1685374015882064303_u64];
_5 = -140446424323919762393460772150297685468_i128;
_8 = '\u{75b3e}';
_3 = _7;
_5 = (-18514391848609719321446070688334495602_i128);
_1 = _6;
_1 = [_7,_3];
_10.0 = 59693616822674506082503970084616566717_u128 as f64;
_5 = 60241452291646374811497663456759592789_i128 - 105826146034882947712745443304515818552_i128;
_7 = _3 + _3;
_5 = 87792529169919713778512546126434904262_i128 * (-113968473911714324751151222351560306529_i128);
_8 = '\u{7965e}';
_10.0 = 740052280_u32 as f64;
Goto(bb2)
}
bb2 = {
_8 = '\u{e428e}';
_2 = 5_usize;
RET = [(-1060090261_i32),1224202679_i32,1982177007_i32];
_12.2 = (_10.0, _3);
_5 = !(-163528645274905374686027134836580757311_i128);
_12.5[_2] = _4[_2];
_5 = _2 as i128;
_12.4.0 = [_7,_7,_12.2.1,_7,_7,_7];
_10 = (_12.2.0, _12.4.0[_2]);
RET = [114063359_i32,140973592_i32,597786881_i32];
_2 = !5_usize;
_12.6.3 = _10.0 * _10.0;
_12.6.2 = _5;
_12.6.4 = 30_u8 as i128;
_12.6.1 = true as i16;
_12.1 = _12.6.1 as u128;
_5 = (-4880741643024587783_i64) as i128;
_12.0 = [_3,_10.1];
_12.3 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_12.6.2 = _5;
_12.6.3 = _12.2.0 + _12.2.0;
_13 = (false,);
_1 = _12.0;
_12.6.0 = !9688597781050362168_u64;
_8 = '\u{49c14}';
_12.5 = _4;
_14 = _12.6.2 + _12.6.4;
Goto(bb3)
}
bb3 = {
_12.6.4 = _5 & _5;
_12.0 = [_10.1,_7];
_12.0 = [_3,_7];
_12.2.1 = _10.1 << _10.1;
RET = [2130792821_i32,(-1770295385_i32),(-1222122089_i32)];
_12.2.1 = _7;
RET = [1693459164_i32,(-1759287412_i32),1190059874_i32];
_12.6 = (1842440678680910822_u64, (-31701_i16), _5, _10.0, _14);
_4 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_12.2.1 = 64_u8 as isize;
_8 = '\u{d283c}';
_17 = _8;
_12.6 = (18096506787743817042_u64, (-9088_i16), _14, _12.2.0, _14);
_12.2.1 = _10.1 - _10.1;
_12.6.2 = -_14;
Goto(bb4)
}
bb4 = {
_1 = [_10.1,_12.2.1];
_10.0 = 1280959887_i32 as f64;
_3 = !_7;
_16 = (RET,);
_3 = !_10.1;
_16.0 = RET;
RET = [714272482_i32,(-989042239_i32),776377776_i32];
_12.6.0 = !18003957070714094159_u64;
_12.6.3 = _12.2.0 * _12.2.0;
_1 = [_10.1,_12.2.1];
_8 = _17;
_10 = (_12.6.3, _12.2.1);
_15 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_13 = (false,);
_12.2.1 = _10.1 & _7;
_10.0 = _12.2.0;
_12.3 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_13.0 = true ^ true;
_17 = _8;
_4 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_10 = _12.2;
_10.0 = _12.6.3;
_12.2.0 = _12.6.3 * _10.0;
_14 = _12.6.1 as i128;
_16 = (RET,);
match _12.6.1 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768202368 => bb11,
_ => bb10
}
}
bb5 = {
_12.6.4 = _5 & _5;
_12.0 = [_10.1,_7];
_12.0 = [_3,_7];
_12.2.1 = _10.1 << _10.1;
RET = [2130792821_i32,(-1770295385_i32),(-1222122089_i32)];
_12.2.1 = _7;
RET = [1693459164_i32,(-1759287412_i32),1190059874_i32];
_12.6 = (1842440678680910822_u64, (-31701_i16), _5, _10.0, _14);
_4 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_12.2.1 = 64_u8 as isize;
_8 = '\u{d283c}';
_17 = _8;
_12.6 = (18096506787743817042_u64, (-9088_i16), _14, _12.2.0, _14);
_12.2.1 = _10.1 - _10.1;
_12.6.2 = -_14;
Goto(bb4)
}
bb6 = {
_8 = '\u{e428e}';
_2 = 5_usize;
RET = [(-1060090261_i32),1224202679_i32,1982177007_i32];
_12.2 = (_10.0, _3);
_5 = !(-163528645274905374686027134836580757311_i128);
_12.5[_2] = _4[_2];
_5 = _2 as i128;
_12.4.0 = [_7,_7,_12.2.1,_7,_7,_7];
_10 = (_12.2.0, _12.4.0[_2]);
RET = [114063359_i32,140973592_i32,597786881_i32];
_2 = !5_usize;
_12.6.3 = _10.0 * _10.0;
_12.6.2 = _5;
_12.6.4 = 30_u8 as i128;
_12.6.1 = true as i16;
_12.1 = _12.6.1 as u128;
_5 = (-4880741643024587783_i64) as i128;
_12.0 = [_3,_10.1];
_12.3 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_12.6.2 = _5;
_12.6.3 = _12.2.0 + _12.2.0;
_13 = (false,);
_1 = _12.0;
_12.6.0 = !9688597781050362168_u64;
_8 = '\u{49c14}';
_12.5 = _4;
_14 = _12.6.2 + _12.6.4;
Goto(bb3)
}
bb7 = {
_4 = [15127101695517071698_u64,5915900979908855176_u64,16462855709432864_u64,13996068578791381175_u64,10441877940284052595_u64,638442821841331573_u64];
RET = [1739832125_i32,748670640_i32,39758538_i32];
_8 = '\u{1a320}';
_4 = [12040270343367718037_u64,7978125421720373033_u64,760254451840831497_u64,5993833645126881513_u64,6624525339970706232_u64,1685374015882064303_u64];
_5 = -140446424323919762393460772150297685468_i128;
_8 = '\u{75b3e}';
_3 = _7;
_5 = (-18514391848609719321446070688334495602_i128);
_1 = _6;
_1 = [_7,_3];
_10.0 = 59693616822674506082503970084616566717_u128 as f64;
_5 = 60241452291646374811497663456759592789_i128 - 105826146034882947712745443304515818552_i128;
_7 = _3 + _3;
_5 = 87792529169919713778512546126434904262_i128 * (-113968473911714324751151222351560306529_i128);
_8 = '\u{7965e}';
_10.0 = 740052280_u32 as f64;
Goto(bb2)
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
_2 = 1_usize;
_16.0[_2] = RET[_2];
_17 = _8;
_9 = core::ptr::addr_of_mut!(_19);
(*_9) = _12.2.1 != _10.1;
_5 = !_14;
_10 = (_12.2.0, _12.0[_2]);
_16 = (RET,);
_12.4.0[_2] = _2 as isize;
(*_9) = !_13.0;
_1 = [_12.2.1,_12.2.1];
_6[_2] = _12.2.1 & _12.2.1;
(*_9) = _13.0;
_24 = _1[_2];
_12.2 = (_10.0, _10.1);
_2 = 1_usize * 16911523205739638253_usize;
Goto(bb12)
}
bb12 = {
_26.fld2.fld0.2 = RET;
_26.fld2.fld2 = -_12.2.1;
_15 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_4 = [_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0,_12.6.0];
_16 = (RET,);
_23 = (_12.4.0,);
_16 = (RET,);
_12.6.2 = _5 | _14;
_26.fld2.fld1 = [738879199_i32,(-1077660745_i32),1872238403_i32];
_35.0 = [_26.fld2.fld2,_12.2.1,_12.2.1,_24,_26.fld2.fld2,_24];
_26.fld2.fld0.0 = [_7,_10.1,_26.fld2.fld2,_24,_24,_12.2.1];
_26.fld1 = (_26.fld2.fld0.0,);
_25 = (-647308968_i32) as i128;
_18 = !208_u8;
_32 = [_2,_2,_2,_2,_2,_2,_2,_2];
_26.fld5 = !_12.6.2;
_12.2 = (_10.0, _26.fld2.fld2);
_12.2 = (_12.6.3, _24);
_10 = _12.2;
_13 = ((*_9),);
_26.fld2.fld0 = (_23.0, _26.fld1.0, _16.0);
_12.4 = (_26.fld1.0,);
_1 = _6;
Goto(bb13)
}
bb13 = {
_37.fld0.0 = [_24,_10.1,_24,_3,_12.2.1,_26.fld2.fld2];
_37.fld1 = [873137306_i32,(-537610971_i32),2064139697_i32];
RET = [(-450645119_i32),(-690102428_i32),1520958339_i32];
_26.fld2.fld0.0 = [_12.2.1,_24,_10.1,_3,_10.1,_26.fld2.fld2];
_33 = _26.fld2.fld2;
_6 = _1;
_37.fld0 = _26.fld2.fld0;
_12.2.1 = _12.6.2 as isize;
_17 = _8;
_16.0 = [531881857_i32,1671692860_i32,772923003_i32];
_35.0 = [_33,_24,_10.1,_10.1,_10.1,_24];
_12.4.0 = _37.fld0.0;
_26.fld2.fld0.1 = [_3,_10.1,_10.1,_3,_3,_26.fld2.fld2];
_29 = Adt45::Variant1 { fld0: _13.0,fld1: 57472_u16,fld2: _9,fld3: _32,fld4: _12.6.0,fld5: 629496859_i32,fld6: _18,fld7: _13 };
_37.fld0.1 = _35.0;
place!(Field::<*mut bool>(Variant(_29, 1), 2)) = core::ptr::addr_of_mut!(_13.0);
place!(Field::<u16>(Variant(_29, 1), 1)) = Field::<u64>(Variant(_29, 1), 4) as u16;
_36 = _1;
_28 = _12.2.0;
_26.fld2.fld0.1 = [_10.1,_10.1,_10.1,_10.1,_24,_10.1];
_37.fld0.1 = _23.0;
_26.fld2.fld0 = (_37.fld0.0, _12.4.0, RET);
match _12.6.1 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb10,
4 => bb7,
340282366920938463463374607431768202368 => bb14,
_ => bb6
}
}
bb14 = {
_10.1 = _3;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(19_usize, 36_usize, Move(_36), 5_usize, Move(_5), 18_usize, Move(_18), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(19_usize, 19_usize, Move(_19), 24_usize, Move(_24), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(19_usize, 15_usize, Move(_15), 17_usize, Move(_17), 6_usize, Move(_6), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(52044_u16), std::hint::black_box(146065455026249368737733237759759671525_i128), std::hint::black_box((-9223372036854775808_isize)));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: ([i32; 3],),
fld1: ([isize; 6],),
fld2: [u8; 2],
fld3: i8,
fld4: u64,
fld5: [usize; 8],
fld6: f32,

},
Variant1{
fld0: *const [i16; 3],

},
Variant2{
fld0: [usize; 8],
fld1: f32,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: (bool,),
fld1: f64,
fld2: [usize; 8],
fld3: [u8; 2],
fld4: ([i32; 3],),
fld5: u128,
fld6: [bool; 2],

},
Variant1{
fld0: u16,
fld1: u64,
fld2: [isize; 6],
fld3: usize,
fld4: ([isize; 6],),
fld5: [isize; 2],

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: u8,
fld1: Adt39,
fld2: (u64, i16, i128, f64, i128),
fld3: i8,
fld4: [bool; 2],
fld5: [i16; 3],

},
Variant1{
fld0: bool,
fld1: [i16; 3],
fld2: Adt39,
fld3: i8,
fld4: f32,
fld5: ([isize; 6],),
fld6: i64,
fld7: u8,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ([i32; 3],),
fld1: i64,
fld2: [u128; 6],
fld3: [bool; 2],
fld4: Adt41,
fld5: [i32; 3],

},
Variant1{
fld0: [i16; 3],
fld1: i128,
fld2: [isize; 2],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: [isize; 6],
fld2: (f64, isize),
fld3: Adt42,
fld4: [i32; 3],
fld5: u128,
fld6: [u128; 6],
fld7: u64,

},
Variant1{
fld0: [i16; 3],
fld1: *const [i16; 3],
fld2: i32,
fld3: Adt41,
fld4: [u8; 2],

},
Variant2{
fld0: u8,
fld1: *mut bool,

},
Variant3{
fld0: [bool; 2],
fld1: char,
fld2: f64,
fld3: i8,
fld4: u8,
fld5: [isize; 2],
fld6: (i8,),
fld7: [i32; 3],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u16,
fld1: i64,
fld2: [usize; 8],
fld3: ([isize; 2], u128, (f64, isize), [u128; 6], ([isize; 6],), [u64; 6], (u64, i16, i128, f64, i128)),
fld4: Adt41,
fld5: i32,

},
Variant1{
fld0: u128,
fld1: Adt41,
fld2: Adt43,
fld3: i64,
fld4: [bool; 2],
fld5: Adt39,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: Adt41,
fld1: ([isize; 6], [isize; 6], [i32; 3]),
fld2: u64,
fld3: (i8,),
fld4: (u64, i16, i128, f64, i128),
fld5: [u128; 6],

},
Variant1{
fld0: bool,
fld1: u16,
fld2: *mut bool,
fld3: [usize; 8],
fld4: u64,
fld5: i32,
fld6: u8,
fld7: (bool,),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f32,
fld1: ([isize; 6],),
fld2: ([i32; 3],),
fld3: i8,
fld4: [usize; 8],
fld5: (i16, u128, [u128; 6], (i8,), u8),
fld6: *const [i16; 3],
fld7: (i8,),

},
Variant1{
fld0: i128,
fld1: char,
fld2: [usize; 8],
fld3: f32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (u64, i16, i128, f64, i128),
fld1: [u64; 6],
fld2: Adt43,
fld3: i8,
fld4: u64,
fld5: [u8; 2],
fld6: Adt42,

},
Variant1{
fld0: u8,

},
Variant2{
fld0: *const ([i32; 3],),
fld1: Adt42,
fld2: u8,
fld3: Adt40,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: *const ([i32; 3],),
fld1: usize,
fld2: [u8; 2],
fld3: Adt40,
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: ([isize; 6], [isize; 6], [i32; 3]),
fld1: [i32; 3],
fld2: isize,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f64,
fld1: char,
fld2: (u64, i16, i128, f64, i128),
fld3: ([i32; 3],),
fld4: [u128; 6],
fld5: Adt42,
fld6: Adt48,

},
Variant1{
fld0: *mut bool,
fld1: Adt40,
fld2: u32,
fld3: [u128; 6],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u64,

},
Variant1{
fld0: [u128; 6],
fld1: (i16, u128, [u128; 6], (i8,), u8),
fld2: isize,
fld3: i8,
fld4: *mut bool,
fld5: (u64, i16, i128, f64, i128),
fld6: u8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: *const [i16; 3],
fld1: ([isize; 6],),
fld2: Adt49,
fld3: Adt46,
fld4: [u128; 6],
fld5: i128,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u16,
fld1: usize,
fld2: Adt44,
fld3: Adt50,
fld4: Adt51,
fld5: Adt41,
fld6: Adt45,

},
Variant1{
fld0: Adt46,

},
Variant2{
fld0: i64,
fld1: (i16, u128, [u128; 6], (i8,), u8),
fld2: i128,
fld3: i32,
fld4: Adt50,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt46,
fld1: (u64, i16, i128, f64, i128),
fld2: Adt44,

},
Variant1{
fld0: [u128; 6],
fld1: Adt43,
fld2: isize,
fld3: ([i32; 3],),

},
Variant2{
fld0: isize,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ([isize; 6], [isize; 6], [i32; 3]),
fld1: i64,
fld2: f64,
fld3: Adt53,

},
Variant1{
fld0: f64,

},
Variant2{
fld0: Adt40,
fld1: char,
fld2: Adt41,
fld3: u32,
fld4: [i16; 3],
fld5: ([isize; 6], [isize; 6], [i32; 3]),
fld6: Adt51,
fld7: Adt53,

},
Variant3{
fld0: u64,
fld1: Adt50,
fld2: [bool; 2],
fld3: u32,
fld4: f64,
fld5: u8,

}}

