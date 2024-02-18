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
pub fn fn0() -> f64 {
mir! {
type RET = f64;
let _1: f32;
let _2: u128;
let _3: u128;
let _4: (*mut usize, u16, *const f64, i16);
let _5: &'static Adt18;
let _6: *const (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _7: isize;
let _8: f32;
let _9: f32;
let _10: &'static char;
let _11: [i8; 1];
let _12: u16;
let _13: i16;
let _14: &'static *mut *mut isize;
let _15: isize;
let _16: isize;
let _17: u8;
let _18: Adt36;
let _19: &'static u128;
let _20: [u128; 5];
let _21: f64;
let _22: ([usize; 4],);
let _23: Adt18;
let _24: *const [u128; 3];
let _25: ();
let _26: ();
{
RET = (-6313130147863600787_i64) as f64;
RET = 17447161632140624441876279933686941153_u128 as f64;
RET = 578569947_u32 as f64;
RET = 445494382_i32 as f64;
RET = 20839_u16 as f64;
RET = 1583032554_i32 as f64;
RET = 3548235964_u32 as f64;
RET = 8810857955430901717_i64 as f64;
RET = (-1768055840883966234_i64) as f64;
Goto(bb1)
}
bb1 = {
RET = 4305020374518707466_i64 as f64;
RET = (-1915126685_i32) as f64;
RET = 531268005_u32 as f64;
RET = 40474_u16 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 10621959530184127333_u64 as f64;
RET = 70_u8 as f64;
RET = 2344420450_u32 as f64;
RET = (-27682_i16) as f64;
RET = 134068810236715890091217325930210059073_u128 as f64;
RET = 9374_i16 as f64;
RET = 660136641623777357_u64 as f64;
RET = (-158928149006224338841488919752656320775_i128) as f64;
RET = 9223372036854775807_isize as f64;
RET = 872606034_u32 as f64;
RET = 11429414457101818694_u64 as f64;
RET = 2561290433167009879_i64 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 34137_u16 as f64;
RET = (-169786918256364764434965634630004501121_i128) as f64;
RET = 3038940522_u32 as f64;
RET = 63_u8 as f64;
RET = 6_usize as f64;
RET = 85_u8 as f64;
RET = 75_u8 as f64;
RET = 12664618176344445116_usize as f64;
_1 = 9223372036854775807_isize as f32;
_2 = '\u{f202}' as u128;
_2 = !337976163911065872080005830234376346276_u128;
Call(RET = fn1(_1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-1477710102_i32) as f64;
_1 = (-16448_i16) as f32;
_2 = !235813286128625271561358656016987935222_u128;
_2 = !62501225411537060640018672431196420683_u128;
_3 = _2;
_2 = RET as u128;
_2 = _3;
_3 = 27758859506958655109728253186043740805_i128 as u128;
RET = 10530_u16 as f64;
_2 = !_3;
RET = 202_u8 as f64;
RET = 1431200932344787723_i64 as f64;
_1 = 5146261667547538693_u64 as f32;
_3 = _2 + _2;
RET = (-23928_i16) as f64;
_4.1 = 41518_u16 - 3943_u16;
_4.2 = core::ptr::addr_of!(RET);
_2 = _3 ^ _3;
_4.2 = core::ptr::addr_of!(RET);
RET = 3131391029_u32 as f64;
_4.3 = !1713_i16;
_4.3 = _4.1 as i16;
_4.3 = 2789_i16;
RET = _1 as f64;
RET = (-114_i8) as f64;
_3 = !_2;
_4.1 = 27919_u16 ^ 12212_u16;
RET = (-7_i8) as f64;
_4.1 = 15617_u16 >> _4.3;
match _4.3 {
0 => bb1,
2789 => bb4,
_ => bb3
}
}
bb3 = {
RET = 4305020374518707466_i64 as f64;
RET = (-1915126685_i32) as f64;
RET = 531268005_u32 as f64;
RET = 40474_u16 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 10621959530184127333_u64 as f64;
RET = 70_u8 as f64;
RET = 2344420450_u32 as f64;
RET = (-27682_i16) as f64;
RET = 134068810236715890091217325930210059073_u128 as f64;
RET = 9374_i16 as f64;
RET = 660136641623777357_u64 as f64;
RET = (-158928149006224338841488919752656320775_i128) as f64;
RET = 9223372036854775807_isize as f64;
RET = 872606034_u32 as f64;
RET = 11429414457101818694_u64 as f64;
RET = 2561290433167009879_i64 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 34137_u16 as f64;
RET = (-169786918256364764434965634630004501121_i128) as f64;
RET = 3038940522_u32 as f64;
RET = 63_u8 as f64;
RET = 6_usize as f64;
RET = 85_u8 as f64;
RET = 75_u8 as f64;
RET = 12664618176344445116_usize as f64;
_1 = 9223372036854775807_isize as f32;
_2 = '\u{f202}' as u128;
_2 = !337976163911065872080005830234376346276_u128;
Call(RET = fn1(_1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_2 = !_3;
_4.3 = (-3118_i16) & (-3239_i16);
Call(_4 = fn2(_3, _3, _2, _3, _3, _3, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = 23_isize as f64;
RET = 1979988905_i32 as f64;
RET = 15528188589833261660_u64 as f64;
_1 = RET as f32;
_3 = _1 as u128;
_4.1 = 4662_u16;
_2 = _3 - _3;
RET = 167841963134660458634598512029035194195_i128 as f64;
_2 = !_3;
match _4.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4662 => bb8,
_ => bb7
}
}
bb6 = {
RET = 4305020374518707466_i64 as f64;
RET = (-1915126685_i32) as f64;
RET = 531268005_u32 as f64;
RET = 40474_u16 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 10621959530184127333_u64 as f64;
RET = 70_u8 as f64;
RET = 2344420450_u32 as f64;
RET = (-27682_i16) as f64;
RET = 134068810236715890091217325930210059073_u128 as f64;
RET = 9374_i16 as f64;
RET = 660136641623777357_u64 as f64;
RET = (-158928149006224338841488919752656320775_i128) as f64;
RET = 9223372036854775807_isize as f64;
RET = 872606034_u32 as f64;
RET = 11429414457101818694_u64 as f64;
RET = 2561290433167009879_i64 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 34137_u16 as f64;
RET = (-169786918256364764434965634630004501121_i128) as f64;
RET = 3038940522_u32 as f64;
RET = 63_u8 as f64;
RET = 6_usize as f64;
RET = 85_u8 as f64;
RET = 75_u8 as f64;
RET = 12664618176344445116_usize as f64;
_1 = 9223372036854775807_isize as f32;
_2 = '\u{f202}' as u128;
_2 = !337976163911065872080005830234376346276_u128;
Call(RET = fn1(_1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
RET = 4305020374518707466_i64 as f64;
RET = (-1915126685_i32) as f64;
RET = 531268005_u32 as f64;
RET = 40474_u16 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 10621959530184127333_u64 as f64;
RET = 70_u8 as f64;
RET = 2344420450_u32 as f64;
RET = (-27682_i16) as f64;
RET = 134068810236715890091217325930210059073_u128 as f64;
RET = 9374_i16 as f64;
RET = 660136641623777357_u64 as f64;
RET = (-158928149006224338841488919752656320775_i128) as f64;
RET = 9223372036854775807_isize as f64;
RET = 872606034_u32 as f64;
RET = 11429414457101818694_u64 as f64;
RET = 2561290433167009879_i64 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 34137_u16 as f64;
RET = (-169786918256364764434965634630004501121_i128) as f64;
RET = 3038940522_u32 as f64;
RET = 63_u8 as f64;
RET = 6_usize as f64;
RET = 85_u8 as f64;
RET = 75_u8 as f64;
RET = 12664618176344445116_usize as f64;
_1 = 9223372036854775807_isize as f32;
_2 = '\u{f202}' as u128;
_2 = !337976163911065872080005830234376346276_u128;
Call(RET = fn1(_1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
RET = 6_usize as f64;
_4.3 = 16647_i16 << _2;
RET = 18248129177830833850_usize as f64;
_1 = 6_usize as f32;
_1 = _3 as f32;
_4.1 = 59295_u16 >> _2;
RET = 62843484_u32 as f64;
RET = 4887070150438950292_u64 as f64;
_1 = (-9_isize) as f32;
_4.2 = core::ptr::addr_of!(RET);
_2 = !_3;
_3 = _2;
_4.3 = !6724_i16;
_2 = _3;
_2 = _3;
_3 = !_2;
_3 = 5577091190260340333_i64 as u128;
_4.2 = core::ptr::addr_of!(RET);
_4.3 = (-22932_i16);
_1 = 974832146_u32 as f32;
_2 = _3 * _3;
_4.1 = !64042_u16;
_4.2 = core::ptr::addr_of!(RET);
_1 = 1931164930_u32 as f32;
_2 = _3 - _3;
RET = _1 as f64;
_1 = 68_i8 as f32;
RET = (-142998761582084916716049849117625264744_i128) as f64;
match _4.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768188524 => bb10,
_ => bb9
}
}
bb9 = {
_2 = !_3;
_4.3 = (-3118_i16) & (-3239_i16);
Call(_4 = fn2(_3, _3, _2, _3, _3, _3, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb10 = {
RET = _3 as f64;
_7 = !(-9223372036854775808_isize);
_2 = _3 | _3;
_1 = 5038872399403815635_i64 as f32;
_1 = 103_u8 as f32;
_4.3 = false as i16;
_2 = _3;
RET = 46_u8 as f64;
_2 = _3;
_4.3 = !(-5659_i16);
RET = 6797282573301526824_i64 as f64;
_4.1 = !23310_u16;
_4.1 = 15372_u16;
_4.2 = core::ptr::addr_of!(RET);
_4.2 = core::ptr::addr_of!(RET);
_2 = 16349171_u32 as u128;
_1 = _3 as f32;
_7 = 5966891458055346581_i64 as isize;
_4.3 = 11015_i16 << _2;
_11 = [(-58_i8)];
_9 = _1;
_9 = -_1;
Goto(bb11)
}
bb11 = {
_4.3 = -28708_i16;
_8 = _1;
_3 = _2;
RET = (-637735177_i32) as f64;
_4.3 = !(-6697_i16);
_13 = (-26_i8) as i16;
_12 = _4.1 * _4.1;
_4.2 = core::ptr::addr_of!(RET);
_7 = (-4_isize);
_4.2 = core::ptr::addr_of!(RET);
_12 = (-53_i8) as u16;
_4.2 = core::ptr::addr_of!(RET);
RET = _3 as f64;
_3 = !_2;
_1 = 103509323958689207016631928117896492457_i128 as f32;
RET = _12 as f64;
_3 = 4_usize as u128;
_7 = !(-9223372036854775808_isize);
_8 = 2879448290765262969_u64 as f32;
Call(_4.3 = core::intrinsics::transmute(_4.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2 = !_3;
_13 = _4.3 >> _12;
_4.2 = core::ptr::addr_of!(RET);
_1 = -_9;
_4.2 = core::ptr::addr_of!(RET);
_8 = _1;
_9 = -_8;
_17 = 125_u8;
_7 = !(-9223372036854775808_isize);
_11 = [11_i8];
_7 = 38_isize;
_12 = _4.1;
_4.3 = !_13;
_15 = false as isize;
_9 = 4347018202861264202_u64 as f32;
_1 = _8 + _9;
_3 = 81198807595346373_i64 as u128;
_12 = _4.1;
_3 = _9 as u128;
_15 = _7 & _7;
_17 = !216_u8;
_4.2 = core::ptr::addr_of!(RET);
Goto(bb13)
}
bb13 = {
_1 = _8 * _9;
_9 = 3094983176_u32 as f32;
_13 = _4.3 + _4.3;
_9 = _1 * _1;
_13 = _4.3;
_4.2 = core::ptr::addr_of!(RET);
_2 = 14347615602091408470_u64 as u128;
_4.1 = true as u16;
_8 = -_1;
_19 = &_3;
_11 = [68_i8];
_19 = &(*_19);
_16 = _13 as isize;
Goto(bb14)
}
bb14 = {
_13 = _4.3;
_22.0 = [16579258609641927608_usize,5_usize,1360806705037734598_usize,1_usize];
_13 = _4.3 - _4.3;
_7 = _16 * _15;
RET = _7 as f64;
_7 = _4.1 as isize;
_19 = &(*_19);
_19 = &_3;
_3 = !_2;
RET = 3_i8 as f64;
_11 = [101_i8];
_15 = _16;
_2 = _3;
_1 = _9;
RET = _12 as f64;
_19 = &_3;
_21 = RET;
_4.3 = !_13;
_4.2 = core::ptr::addr_of!(_21);
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(0_usize, 12_usize, Move(_12), 17_usize, Move(_17), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(0_usize, 11_usize, Move(_11), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f32,mut _2: f32,mut _3: u128) -> f64 {
mir! {
type RET = f64;
let _4: f64;
let _5: &'static *const i64;
let _6: &'static &'static [i128; 7];
let _7: u128;
let _8: [i128; 7];
let _9: (u8,);
let _10: bool;
let _11: &'static f32;
let _12: i32;
let _13: isize;
let _14: char;
let _15: f64;
let _16: f32;
let _17: f64;
let _18: bool;
let _19: [char; 7];
let _20: bool;
let _21: char;
let _22: f32;
let _23: i8;
let _24: isize;
let _25: ();
let _26: ();
{
RET = 185471648_u32 as f64;
RET = 1553347906_u32 as f64;
_1 = (-21710752880042559714783983714742875302_i128) as f32;
_3 = 314251788629607227149457888613651608675_u128 * 194244240442904299663988652234985361192_u128;
Goto(bb1)
}
bb1 = {
_3 = !59786240874484801584398398758529727538_u128;
_4 = 1583562660_u32 as f64;
RET = _4 - _4;
_4 = -RET;
_4 = 9_i8 as f64;
_4 = 3860423462_u32 as f64;
RET = _4;
Goto(bb2)
}
bb2 = {
RET = _4 - _4;
_3 = !93221378118345307467740719981557243739_u128;
_4 = RET * RET;
RET = _4 + _4;
RET = 18012_u16 as f64;
_3 = 237780234188922685860410831247502164481_u128 * 247195347225709508279004819144427692865_u128;
RET = _4;
RET = 2640947748_u32 as f64;
_7 = (-89_i8) as u128;
_1 = _2;
_9 = (211_u8,);
_4 = -RET;
Goto(bb3)
}
bb3 = {
_9.0 = 113_u8 >> _3;
_1 = _2;
RET = _4 - _4;
_9 = (25_u8,);
Goto(bb4)
}
bb4 = {
_8 = [162913943924987441161891187470549211182_i128,105118603406829677717395049257804521182_i128,20718619632165104986357960481029967210_i128,149994701668295336247811841566769773553_i128,106495187534048030626462663143880870063_i128,(-94262291373329301511553083133350221661_i128),(-167299363507836988682957881260202307875_i128)];
RET = _4;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
25 => bb10,
_ => bb9
}
}
bb5 = {
_9.0 = 113_u8 >> _3;
_1 = _2;
RET = _4 - _4;
_9 = (25_u8,);
Goto(bb4)
}
bb6 = {
RET = _4 - _4;
_3 = !93221378118345307467740719981557243739_u128;
_4 = RET * RET;
RET = _4 + _4;
RET = 18012_u16 as f64;
_3 = 237780234188922685860410831247502164481_u128 * 247195347225709508279004819144427692865_u128;
RET = _4;
RET = 2640947748_u32 as f64;
_7 = (-89_i8) as u128;
_1 = _2;
_9 = (211_u8,);
_4 = -RET;
Goto(bb3)
}
bb7 = {
_3 = !59786240874484801584398398758529727538_u128;
_4 = 1583562660_u32 as f64;
RET = _4 - _4;
_4 = -RET;
_4 = 9_i8 as f64;
_4 = 3860423462_u32 as f64;
RET = _4;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_10 = !false;
_9 = (206_u8,);
_9 = (61_u8,);
RET = 131335110332762064381602777428285442935_i128 as f64;
_12 = 696684085_i32 | 2131840264_i32;
_4 = RET;
_12 = !351136079_i32;
_11 = &_1;
RET = _4 * _4;
_3 = !_7;
_9 = (107_u8,);
_1 = 10027626209194778295_u64 as f32;
_2 = -_1;
_4 = 9223372036854775807_isize as f64;
_7 = _3;
_11 = &_1;
match _9.0 {
107 => bb11,
_ => bb4
}
}
bb11 = {
_3 = _7 & _7;
_9.0 = !72_u8;
_9.0 = !161_u8;
_13 = 9223372036854775807_isize << _9.0;
_3 = !_7;
_2 = _3 as f32;
_11 = &(*_11);
_14 = '\u{6c460}';
_11 = &(*_11);
_4 = RET + RET;
_15 = _4;
_9.0 = 57_u8;
_12 = _9.0 as i32;
_10 = !true;
RET = -_4;
RET = _15 + _15;
_18 = _10;
_4 = _15 + RET;
_9 = (145_u8,);
_16 = 8065478320739079202_usize as f32;
_10 = _18;
_16 = (*_11) - (*_11);
Goto(bb12)
}
bb12 = {
_14 = '\u{eee5a}';
_9.0 = 116_u8;
_19 = [_14,_14,_14,_14,_14,_14,_14];
_14 = '\u{93749}';
_13 = 18_isize;
_15 = _13 as f64;
_22 = _16 + _1;
_3 = _7 ^ _7;
_14 = '\u{10f510}';
RET = -_4;
_20 = _10;
_9.0 = 40_u8 >> _13;
_7 = _13 as u128;
_11 = &(*_11);
Goto(bb13)
}
bb13 = {
Call(_25 = dump_var(1_usize, 14_usize, Move(_14), 7_usize, Move(_7), 13_usize, Move(_13), 18_usize, Move(_18)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_25 = dump_var(1_usize, 10_usize, Move(_10), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128) -> (*mut usize, u16, *const f64, i16) {
mir! {
type RET = (*mut usize, u16, *const f64, i16);
let _8: i8;
let _9: char;
let _10: (([u128; 5], [i8; 1], isize), *mut (u8,), *const *const u32);
let _11: f64;
let _12: *const [u128; 3];
let _13: f64;
let _14: isize;
let _15: isize;
let _16: Adt18;
let _17: &'static u64;
let _18: *const f64;
let _19: *const u32;
let _20: [u8; 4];
let _21: isize;
let _22: [i128; 8];
let _23: f32;
let _24: i16;
let _25: isize;
let _26: Adt30;
let _27: *mut usize;
let _28: bool;
let _29: usize;
let _30: isize;
let _31: [usize; 6];
let _32: *const f64;
let _33: i128;
let _34: &'static *const i64;
let _35: ();
let _36: ();
{
_2 = !_1;
RET.1 = (-50176736430469943548485987286180409602_i128) as u16;
_2 = 12_i8 as u128;
_1 = 3778575261169847239_usize as u128;
RET.3 = 7869_i16 * (-8485_i16);
_7 = _4 * _3;
RET.3 = !(-11387_i16);
_3 = _1;
_8 = (-95_i8) | 48_i8;
RET.1 = 55423_u16;
_4 = !_7;
_4 = !_7;
match RET.1 {
0 => bb1,
55423 => bb3,
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
_10.0.1 = [_8];
_6 = !_5;
RET.3 = 2145595089_u32 as i16;
_10.0.0 = [_7,_4,_5,_6,_7];
_5 = _6;
RET.1 = 11503_u16;
Call(_10.0.0 = fn3(_8, _5, _6, _7, _10.0.1, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = '\u{7c7e7}';
_9 = '\u{f9302}';
_9 = '\u{c5a3a}';
RET.1 = 37544_u16;
_10.0.2 = -(-9223372036854775808_isize);
_9 = '\u{29a64}';
_6 = 3253813920636540227_u64 as u128;
_4 = 7350195007924956638_i64 as u128;
_5 = _7;
RET.2 = core::ptr::addr_of!(_11);
_11 = RET.3 as f64;
_10.0.1 = [_8];
RET.3 = 29089_i16;
RET.1 = !52997_u16;
_2 = _7 << _5;
RET.2 = core::ptr::addr_of!(_11);
_10.0.1 = [_8];
_13 = RET.1 as f64;
RET.3 = !27593_i16;
Call(_14 = fn17(_9, _10.0.2, _2, _10.0, _2, _1, _5, _1, _2, _7, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = _5 & _5;
RET.1 = _9 as u16;
_13 = _11 - _11;
RET.1 = 4167_u16 >> _7;
_10.0.0 = [_2,_2,_5,_5,_7];
_15 = _14 << _5;
_5 = true as u128;
_16 = Adt18::Variant2 { fld0: _9 };
_1 = !_4;
_9 = Field::<char>(Variant(_16, 2), 0);
_10.0.1 = [_8];
_15 = -_14;
_10.0.0 = [_2,_2,_2,_7,_2];
_10.2 = core::ptr::addr_of!(_19);
_7 = RET.1 as u128;
RET.2 = core::ptr::addr_of!(_13);
RET.3 = 2539_i16;
_20 = [165_u8,202_u8,12_u8,251_u8];
_13 = _11 + _11;
Goto(bb6)
}
bb6 = {
_11 = -_13;
_6 = !_7;
_21 = !_14;
_15 = !_21;
RET.2 = core::ptr::addr_of!(_11);
_6 = 3_usize as u128;
_2 = !_1;
_4 = 119_u8 as u128;
_2 = _13 as u128;
place!(Field::<char>(Variant(_16, 2), 0)) = _9;
_4 = _7 | _6;
_16 = Adt18::Variant1 { fld0: _8,fld1: RET.1,fld2: 891777631698829221_i64 };
_10.2 = core::ptr::addr_of!(_19);
_15 = _10.0.2 * _21;
_23 = 953727575_i32 as f32;
_13 = _11 + _11;
_24 = RET.3;
place!(Field::<i64>(Variant(_16, 1), 2)) = 2061460187_i32 as i64;
place!(Field::<i8>(Variant(_16, 1), 0)) = _9 as i8;
_3 = !_4;
_22 = [(-35286752458045184456665245906131623505_i128),125032163158680872138830822302994294322_i128,(-54957153754557149176041778680354400951_i128),126400260987205115160888530681158626673_i128,(-95593879132416636594713282223838544343_i128),(-26361437725247284936139599887113687034_i128),(-89152480181599325335223129967925902987_i128),(-48876331893189680153608120248628382740_i128)];
_8 = Field::<i8>(Variant(_16, 1), 0) ^ Field::<i8>(Variant(_16, 1), 0);
_11 = -_13;
_10.2 = core::ptr::addr_of!(_19);
match _24 {
0 => bb3,
1 => bb2,
2539 => bb8,
_ => bb7
}
}
bb7 = {
_7 = _5 & _5;
RET.1 = _9 as u16;
_13 = _11 - _11;
RET.1 = 4167_u16 >> _7;
_10.0.0 = [_2,_2,_5,_5,_7];
_15 = _14 << _5;
_5 = true as u128;
_16 = Adt18::Variant2 { fld0: _9 };
_1 = !_4;
_9 = Field::<char>(Variant(_16, 2), 0);
_10.0.1 = [_8];
_15 = -_14;
_10.0.0 = [_2,_2,_2,_7,_2];
_10.2 = core::ptr::addr_of!(_19);
_7 = RET.1 as u128;
RET.2 = core::ptr::addr_of!(_13);
RET.3 = 2539_i16;
_20 = [165_u8,202_u8,12_u8,251_u8];
_13 = _11 + _11;
Goto(bb6)
}
bb8 = {
RET.1 = Field::<u16>(Variant(_16, 1), 1) * Field::<u16>(Variant(_16, 1), 1);
_21 = _14 - _15;
_13 = _11 + _11;
Goto(bb9)
}
bb9 = {
_7 = !_3;
_18 = core::ptr::addr_of!(_13);
(*_18) = _11 + _11;
Goto(bb10)
}
bb10 = {
_26 = Adt30 { fld0: RET.1 };
match RET.3 {
0 => bb1,
1 => bb9,
2 => bb7,
3 => bb8,
4 => bb6,
2539 => bb12,
_ => bb11
}
}
bb11 = {
_10.0.1 = [_8];
_6 = !_5;
RET.3 = 2145595089_u32 as i16;
_10.0.0 = [_7,_4,_5,_6,_7];
_5 = _6;
RET.1 = 11503_u16;
Call(_10.0.0 = fn3(_8, _5, _6, _7, _10.0.1, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_6 = _3;
_13 = _11 + _11;
(*_18) = 17231135324209822065_usize as f64;
_25 = _21 >> _3;
_9 = '\u{eca7b}';
_29 = _21 as usize;
_21 = Field::<i64>(Variant(_16, 1), 2) as isize;
_27 = core::ptr::addr_of_mut!(_29);
place!(Field::<i64>(Variant(_16, 1), 2)) = _9 as i64;
_15 = _14 << _21;
RET.2 = core::ptr::addr_of!((*_18));
Goto(bb13)
}
bb13 = {
RET.3 = !_24;
_31 = [(*_27),(*_27),(*_27),(*_27),_29,(*_27)];
_28 = !true;
_10.0.1 = [_8];
_8 = Field::<i8>(Variant(_16, 1), 0);
match _24 {
0 => bb6,
1 => bb2,
2 => bb9,
3 => bb12,
2539 => bb15,
_ => bb14
}
}
bb14 = {
_7 = !_3;
_18 = core::ptr::addr_of!(_13);
(*_18) = _11 + _11;
Goto(bb10)
}
bb15 = {
RET.0 = core::ptr::addr_of_mut!(_29);
_7 = Field::<i8>(Variant(_16, 1), 0) as u128;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(2_usize, 22_usize, Move(_22), 3_usize, Move(_3), 31_usize, Move(_31), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(2_usize, 6_usize, Move(_6), 7_usize, Move(_7), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(2_usize, 25_usize, Move(_25), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: [i8; 1],mut _6: u128) -> [u128; 5] {
mir! {
type RET = [u128; 5];
let _7: u8;
let _8: isize;
let _9: u16;
let _10: &'static [i128; 7];
let _11: &'static u128;
let _12: &'static u64;
let _13: char;
let _14: [u32; 3];
let _15: u16;
let _16: u16;
let _17: f32;
let _18: f64;
let _19: &'static Adt36;
let _20: i32;
let _21: char;
let _22: [bool; 5];
let _23: isize;
let _24: &'static Adt18;
let _25: char;
let _26: Adt58;
let _27: &'static f32;
let _28: [u32; 2];
let _29: isize;
let _30: char;
let _31: Adt58;
let _32: (([u128; 5], [i8; 1], isize), *mut (u8,), *const *const u32);
let _33: Adt47;
let _34: *mut [i8; 1];
let _35: isize;
let _36: u64;
let _37: isize;
let _38: *const u32;
let _39: char;
let _40: &'static [i128; 7];
let _41: isize;
let _42: &'static &'static [i128; 7];
let _43: ();
let _44: ();
{
_4 = 6485771704497218467_u64 as u128;
RET = [_2,_3,_3,_6,_6];
_7 = 36_u8 + 189_u8;
_8 = 9223372036854775807_isize;
Call(_2 = fn4(_6, _3, RET, RET, RET, _6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 1102411922_u32 as u128;
_6 = true as u128;
_8 = (-720008891391945229_i64) as isize;
_4 = _3;
RET = [_3,_4,_3,_3,_3];
_4 = !_6;
_2 = _3;
_6 = !_4;
_5 = [_1];
_5 = [_1];
_4 = 24789809_i32 as u128;
_1 = !126_i8;
_2 = _3 ^ _3;
_8 = !(-62_isize);
_3 = !_2;
_6 = 4223056266769831348_i64 as u128;
_3 = _2;
_3 = _4 | _6;
RET = [_2,_2,_2,_2,_4];
_2 = _6;
_4 = _3 * _6;
_11 = &_3;
RET = [_4,(*_11),_4,(*_11),_4];
RET = [_3,_3,_3,_3,(*_11)];
_7 = _8 as u8;
Call(RET = fn5(Move(_11), (*_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _3 - _4;
RET = [_4,_3,_2,_2,_2];
_7 = 209_u8;
_9 = 32954_u16 >> _4;
_2 = _3;
_1 = 124_i8 >> _8;
_4 = _2 ^ _3;
_13 = '\u{e2447}';
_14 = [3380821643_u32,956479507_u32,1067066005_u32];
Goto(bb3)
}
bb3 = {
_15 = _9 + _9;
RET = [_4,_4,_2,_3,_2];
_5 = [_1];
_7 = !245_u8;
_4 = _2;
_1 = !15_i8;
_13 = '\u{5767}';
_16 = _13 as u16;
_4 = _3;
_4 = _2 >> _15;
_4 = _6;
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = &_6;
_20 = _13 as i32;
_23 = 15276_i16 as isize;
RET = [_2,_4,_3,_4,_6];
Goto(bb5)
}
bb5 = {
_9 = _15;
_5 = [_1];
_8 = _23 - _23;
_1 = _7 as i8;
_15 = _9 >> _4;
_23 = !_8;
_25 = _13;
_6 = !_2;
_8 = _23;
_28 = [133827330_u32,622375781_u32];
_14 = [2299497100_u32,428754746_u32,1475304610_u32];
_27 = &_17;
_18 = _23 as f64;
_11 = &_6;
_16 = 6_usize as u16;
_21 = _25;
_28 = [2080251892_u32,598188292_u32];
RET = [_6,(*_11),(*_11),(*_11),_6];
Goto(bb6)
}
bb6 = {
_21 = _25;
_9 = _15;
_29 = !_8;
Goto(bb7)
}
bb7 = {
_22 = [true,true,false,true,true];
_17 = 13406236292993445219_u64 as f32;
_33 = Adt47::Variant1 { fld0: _17 };
_16 = _15;
_30 = _13;
_11 = &_6;
_30 = _13;
_5 = [_1];
_30 = _25;
SetDiscriminant(_33, 1);
Goto(bb8)
}
bb8 = {
_1 = -(-126_i8);
_23 = _29;
_14 = [2440873389_u32,2321191698_u32,2849288196_u32];
Goto(bb9)
}
bb9 = {
_32.0.1 = _5;
_11 = &(*_11);
_13 = _25;
_11 = &_6;
_13 = _21;
_20 = (-1948227905_i32) & 1971255437_i32;
_31 = Adt58::Variant1 { fld0: _29,fld1: 4885047351387799819_usize };
_27 = &_17;
_31 = Adt58::Variant1 { fld0: _29,fld1: 3_usize };
_7 = !75_u8;
_23 = Field::<isize>(Variant(_31, 1), 0);
_21 = _25;
_23 = Field::<isize>(Variant(_31, 1), 0);
_20 = -307151557_i32;
_13 = _21;
_17 = (-2657742890525892504341755441514999656_i128) as f32;
_36 = 6901545857070230543_u64 - 11558850097920352372_u64;
_9 = _16;
place!(Field::<f32>(Variant(_33, 1), 0)) = _17;
_11 = &(*_11);
_11 = &(*_11);
_15 = !_16;
_7 = _13 as u8;
_32.0 = (RET, _5, _23);
place!(Field::<usize>(Variant(_31, 1), 1)) = 5207719365214826066_usize ^ 15943452418942508536_usize;
Call(RET = core::intrinsics::transmute(_32.0.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_12 = &_36;
_34 = core::ptr::addr_of_mut!(_32.0.1);
place!(Field::<f32>(Variant(_33, 1), 0)) = -_17;
_17 = 3926140193_u32 as f32;
place!(Field::<isize>(Variant(_31, 1), 0)) = _29 << _15;
_16 = !_9;
_26 = Move(_31);
_37 = _32.0.2 | Field::<isize>(Variant(_26, 1), 0);
_4 = _3;
_22 = [false,true,true,false,false];
_7 = 251_u8;
RET = [(*_11),_4,(*_11),_4,_4];
_3 = (*_11);
_6 = !_4;
_37 = _8;
_31 = Move(_26);
_13 = _30;
_41 = _20 as isize;
match _7 {
0 => bb11,
1 => bb12,
2 => bb13,
251 => bb15,
_ => bb14
}
}
bb11 = {
_15 = _9 + _9;
RET = [_4,_4,_2,_3,_2];
_5 = [_1];
_7 = !245_u8;
_4 = _2;
_1 = !15_i8;
_13 = '\u{5767}';
_16 = _13 as u16;
_4 = _3;
_4 = _2 >> _15;
_4 = _6;
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_1 = -(-126_i8);
_23 = _29;
_14 = [2440873389_u32,2321191698_u32,2849288196_u32];
Goto(bb9)
}
bb13 = {
_22 = [true,true,false,true,true];
_17 = 13406236292993445219_u64 as f32;
_33 = Adt47::Variant1 { fld0: _17 };
_16 = _15;
_30 = _13;
_11 = &_6;
_30 = _13;
_5 = [_1];
_30 = _25;
SetDiscriminant(_33, 1);
Goto(bb8)
}
bb14 = {
_2 = _3 - _4;
RET = [_4,_3,_2,_2,_2];
_7 = 209_u8;
_9 = 32954_u16 >> _4;
_2 = _3;
_1 = 124_i8 >> _8;
_4 = _2 ^ _3;
_13 = '\u{e2447}';
_14 = [3380821643_u32,956479507_u32,1067066005_u32];
Goto(bb3)
}
bb15 = {
_6 = _3;
_25 = _30;
_9 = _15 * _16;
_39 = _21;
_33 = Adt47::Variant0 { fld0: _28 };
_9 = _20 as u16;
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(3_usize, 3_usize, Move(_3), 8_usize, Move(_8), 36_usize, Move(_36), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(3_usize, 39_usize, Move(_39), 23_usize, Move(_23), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(3_usize, 9_usize, Move(_9), 41_usize, Move(_41), 37_usize, Move(_37), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u128,mut _2: u128,mut _3: [u128; 5],mut _4: [u128; 5],mut _5: [u128; 5],mut _6: u128,mut _7: u128) -> u128 {
mir! {
type RET = u128;
let _8: isize;
let _9: i32;
let _10: [i64; 8];
let _11: [u32; 3];
let _12: u8;
let _13: isize;
let _14: (u32, u8, i32);
let _15: ();
let _16: ();
{
RET = !_7;
_3 = _5;
_6 = RET;
_5 = [RET,_7,_7,RET,_2];
_4 = [_1,_1,_6,_1,_2];
_8 = 2548386106344727531_i64 as isize;
Goto(bb1)
}
bb1 = {
_6 = RET ^ RET;
_5 = [_6,_6,_6,_6,_2];
_7 = !_6;
_2 = RET | _7;
_2 = !RET;
RET = !_7;
_9 = (-168836359_i32);
_7 = _2;
_9 = (-1467562979_i32) & 1655821105_i32;
_1 = (-138618304736631228132996132705334942439_i128) as u128;
_4 = [_6,_2,_7,_6,_2];
_2 = _9 as u128;
RET = !_6;
RET = _6 | _6;
_8 = !(-9223372036854775808_isize);
_10 = [2826691585135513035_i64,(-6427690438906735433_i64),1157758545446823942_i64,(-5283747896496889408_i64),(-2532377098773524151_i64),(-7245654916745869734_i64),5140996637148658797_i64,(-5186394482277501261_i64)];
RET = !_6;
_11 = [1840987552_u32,3168299326_u32,4251290796_u32];
_12 = 101_u8;
_3 = [RET,RET,RET,RET,RET];
_11 = [955105938_u32,3390146677_u32,572166667_u32];
_9 = 6981_i16 as i32;
_10 = [4096622563773280662_i64,5980429604689395681_i64,(-2914304337148596993_i64),4853797250973765385_i64,6015535966705824119_i64,3411322740264270191_i64,3950877067097437478_i64,(-4889251677965228091_i64)];
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(4_usize, 8_usize, Move(_8), 3_usize, Move(_3), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(4_usize, 12_usize, Move(_12), 9_usize, Move(_9), 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: &'static u128,mut _2: u128) -> [u128; 5] {
mir! {
type RET = [u128; 5];
let _3: u128;
let _4: u32;
let _5: &'static &'static &'static [i128; 7];
let _6: *mut f64;
let _7: f32;
let _8: *mut usize;
let _9: [i64; 8];
let _10: usize;
let _11: i16;
let _12: [i128; 7];
let _13: &'static *mut *mut isize;
let _14: [usize; 4];
let _15: [bool; 5];
let _16: &'static u128;
let _17: &'static *const i64;
let _18: i8;
let _19: bool;
let _20: (i128, isize);
let _21: u16;
let _22: &'static i8;
let _23: f32;
let _24: ();
let _25: ();
{
RET = [_2,_2,_2,_2,_2];
_4 = (-4_i8) as u32;
RET = [_2,_2,_2,_2,_2];
_2 = 305916876704779058004836362058347332567_u128;
_1 = &_2;
_4 = !4118141481_u32;
RET = [_2,_2,(*_1),_2,_2];
_3 = (*_1);
RET = [(*_1),(*_1),_3,_3,(*_1)];
match (*_1) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
305916876704779058004836362058347332567 => bb9,
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
_3 = (*_1) % _2;
_7 = _2 as f32;
_2 = _3 + _3;
Goto(bb10)
}
bb10 = {
_9 = [(-4636828817881379449_i64),4195014108973682257_i64,2368651382095117371_i64,8948973205708510698_i64,3969032048997372147_i64,(-7560199376921170616_i64),(-1659277269429495639_i64),(-1332074615973021462_i64)];
_8 = core::ptr::addr_of_mut!(_10);
_10 = _7 as usize;
_1 = &_3;
_3 = _2 << _10;
_1 = &_3;
RET = [_3,(*_1),(*_1),(*_1),(*_1)];
_9 = [7369718294858650528_i64,(-6398778930774181564_i64),3092540639308673057_i64,(-519873141804978753_i64),(-423236184604860442_i64),3633074368314817324_i64,(-6078135163346618492_i64),8659863964614326431_i64];
_1 = &_2;
RET = [_3,(*_1),_2,_2,(*_1)];
Goto(bb11)
}
bb11 = {
_11 = 8996_i16;
_10 = 17851239534254347588_usize << _3;
_11 = 26636_i16;
(*_8) = !7_usize;
_4 = true as u32;
_2 = _3;
_4 = 1818189547_u32;
RET = [_2,_3,_2,_3,_2];
_7 = 391592466_i32 as f32;
(*_8) = 5665495533460670978_usize >> _2;
_11 = 8050_i16;
RET = [_3,_2,_2,_2,_2];
_4 = '\u{b0fe9}' as u32;
_7 = 176_u8 as f32;
_10 = 9629196683559269571_usize - 1622578369389852518_usize;
_10 = 15141926410427646145_usize | 7_usize;
_1 = &_2;
_7 = _11 as f32;
_9 = [(-4973435255051811677_i64),3751103405640134286_i64,1368268841732853867_i64,(-3183616111340490059_i64),4904222577768516710_i64,(-7720008473321712244_i64),5841831716165295221_i64,(-2136226172550071794_i64)];
_11 = (-4342_i16);
RET = [_3,(*_1),_2,_3,(*_1)];
Call(_12 = fn6(Move(_1), (*_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = &_3;
_12 = [(-163613541955845671511315872038781498879_i128),117135882406604397713738575869568329803_i128,89498705877258888740974334951825455982_i128,102896542026571895897199590018017459846_i128,(-47907982897090752248082139913407627428_i128),(-147342414695953726202611513973890277340_i128),(-157461312940054092447180813867781496646_i128)];
_1 = &(*_1);
RET = [(*_1),(*_1),(*_1),_3,_2];
_7 = (*_8) as f32;
_11 = !(-18574_i16);
_11 = !22498_i16;
RET = [(*_1),(*_1),_3,_3,_3];
RET = [_3,(*_1),_3,_3,(*_1)];
_11 = 21623_i16 + (-7622_i16);
_4 = !1086962834_u32;
_11 = (-22249_i16);
_14 = [(*_8),(*_8),(*_8),(*_8)];
_15 = [true,false,true,false,true];
_11 = (-18850_i16);
_3 = _2;
(*_8) = 422662906049562550_usize;
_16 = &_2;
_4 = true as u32;
(*_8) = (-1361584665_i32) as usize;
_1 = &_2;
_7 = 45861_u16 as f32;
_1 = Move(_16);
RET = [_2,_2,_2,_2,_3];
_3 = _2;
_9 = [6886250301829774344_i64,(-9115180612105703775_i64),(-5517449836279880115_i64),(-4428270877112382884_i64),1735282665844833594_i64,6495682582198661219_i64,8383045091892245252_i64,(-1515602773750624516_i64)];
_10 = 7_usize >> _2;
Goto(bb13)
}
bb13 = {
_11 = 22101_i16;
(*_8) = 1_usize;
match (*_8) {
0 => bb6,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb14,
6 => bb15,
1 => bb17,
_ => bb16
}
}
bb14 = {
_3 = (*_1) % _2;
_7 = _2 as f32;
_2 = _3 + _3;
Goto(bb10)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_15 = [true,true,true,false,true];
_2 = _3 - RET[_10];
_9 = [(-245398574318489052_i64),5215849906987693103_i64,(-6758380830531658296_i64),(-588109911277644524_i64),(-5544052301186637841_i64),3111978200851396875_i64,6966850219340445520_i64,(-3578983811116418526_i64)];
_20.1 = (-9223372036854775808_isize);
_9[_10] = 892866755895852468_i64 ^ (-459910719685468034_i64);
RET[_10] = !_3;
_4 = !617212861_u32;
_20.0 = _12[_10] | _12[_10];
_18 = 55_i8 - (-124_i8);
(*_8) = !_14[_10];
_12 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_2 = !_3;
_1 = &_2;
_10 = 3981774035822041873_usize;
_4 = !1883785442_u32;
_10 = 4910053350153933810_usize & 13442083041621918498_usize;
_20.1 = -9223372036854775807_isize;
_21 = 35004_u16;
_14 = [(*_8),(*_8),_10,(*_8)];
_19 = false;
RET = [(*_1),_2,_3,(*_1),_2];
_20.0 = (-8721445610880359360114940326665217840_i128);
_21 = !24343_u16;
RET = [_3,_3,(*_1),_3,_3];
_19 = false & true;
_20.1 = (-755227304_i32) as isize;
_18 = 65_i8;
Goto(bb18)
}
bb18 = {
Call(_24 = dump_var(5_usize, 10_usize, Move(_10), 4_usize, Move(_4), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_24 = dump_var(5_usize, 18_usize, Move(_18), 14_usize, Move(_14), 25_usize, _25, 25_usize, _25), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: &'static u128,mut _2: u128) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _3: &'static &'static [i128; 7];
let _4: *mut isize;
let _5: f64;
let _6: bool;
let _7: [u32; 3];
let _8: *mut isize;
let _9: isize;
let _10: ([i8; 1], [char; 7], *const [u128; 3]);
let _11: char;
let _12: *mut isize;
let _13: *mut [i8; 1];
let _14: f64;
let _15: [i128; 8];
let _16: u16;
let _17: f64;
let _18: f32;
let _19: &'static [i128; 7];
let _20: isize;
let _21: [usize; 6];
let _22: [u8; 4];
let _23: isize;
let _24: &'static Adt60;
let _25: isize;
let _26: char;
let _27: bool;
let _28: &'static i8;
let _29: (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _30: isize;
let _31: &'static Adt41;
let _32: *const *const u32;
let _33: (i128, isize);
let _34: i128;
let _35: f32;
let _36: isize;
let _37: i64;
let _38: (i128, isize);
let _39: (char, &'static [char; 7], *mut isize);
let _40: ();
let _41: ();
{
_1 = &_2;
RET = [(-136214077390354019987460562027411968361_i128),126967631809024556628391957072716866265_i128,(-71309509324565956632670361241033260849_i128),114128079643542748435306776347850774165_i128,120832737540186936258689809745657149204_i128,115017096134454964578611948003379991123_i128,73873198579142311935574273230779722466_i128];
_2 = 38_i8 as u128;
_2 = !95297729901694078849773403498520570616_u128;
_1 = &_2;
_2 = 147999435257325761857951931708680824523_u128;
_1 = &_2;
RET = [(-151764330142347769074104266900435200326_i128),(-62550231384530627651174259785085478022_i128),160477629497020236689453806914576039149_i128,(-115493437310026425491862732571922984299_i128),154474671365135511505260914422190778137_i128,(-97679143046744172128223477356025395406_i128),(-117664676527261920803342151705781740197_i128)];
_1 = &(*_1);
_1 = &(*_1);
_1 = &(*_1);
_2 = '\u{10a162}' as u128;
_1 = &_2;
_7 = [1370956531_u32,146215767_u32,1785777724_u32];
_7 = [1656865260_u32,2066065911_u32,2140556132_u32];
_5 = (-574972981_i32) as f64;
_5 = (-77701543471445690310885291315103353675_i128) as f64;
Call(_6 = fn7(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = &(*_1);
_1 = &(*_1);
_4 = core::ptr::addr_of_mut!(_9);
_5 = 29631_u16 as f64;
_9 = _5 as isize;
RET = [(-126185422324575895279953808451037095055_i128),(-77485657234086876527070105678565312042_i128),(-21767848202970277113137193936979539629_i128),104843972270945246984967098702119775398_i128,112643768319843756736845977090038593158_i128,17161951791564961562990842979245573234_i128,(-2881482347624895120414016610215218174_i128)];
_6 = !true;
_8 = Move(_4);
RET = [24149208290159950888374904630837495071_i128,64289677998356846782492998087988496224_i128,(-18113687705797131079681563882847522941_i128),(-15697938514214949520268337414986700394_i128),134918971758170088718663115114431344272_i128,(-58698166688646435476765398929899605309_i128),99161357533621905224353125476906312372_i128];
_9 = !(-70_isize);
_6 = (*_1) < (*_1);
_6 = false;
_4 = Move(_8);
_9 = (-126_isize) - (-52_isize);
_1 = &_2;
RET = [(-8788319175791204366048542307194442110_i128),111104482189060724855821747526865194816_i128,115838524886253761078065040014450961092_i128,105751254489974665493213044124266880408_i128,97596325628665668930222431653502644487_i128,(-166078998101312979816766241092988889577_i128),(-36459294505884408907266871021877166198_i128)];
_1 = &(*_1);
_5 = _9 as f64;
_7 = [3023010253_u32,1618662460_u32,678880721_u32];
_4 = core::ptr::addr_of_mut!(_9);
_8 = core::ptr::addr_of_mut!(_9);
RET = [(-48314985190336558144699568091537967252_i128),(-121961393500070622090652941153571067623_i128),(-42420264605166192935186067664644747205_i128),(-31194005497795552564875747518749914000_i128),169539931725933114633415048611421613309_i128,139110889945576832610213357100180036053_i128,70623734395995741174102791097010073728_i128];
(*_8) = !(-9223372036854775808_isize);
RET = [(-133066518864070107824994571692076304513_i128),(-140969239506171603886726447768875677720_i128),(-23730796033273819947492516165715641735_i128),2583860044951268659271802047201163390_i128,89370772029954549897558039980915689314_i128,(-6412315496133775908784297771349283371_i128),(-165214402008128247305506265314071292600_i128)];
_10.0 = [96_i8];
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
_1 = &(*_1);
_5 = 4_i8 as f64;
Call(_14 = fn8(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = '\u{1faae}';
_15 = [33029466789731308108696364687978401352_i128,(-115784811520529033609576391821155972324_i128),127839685612015184961784425048322807383_i128,7670901621471436904487257975998259044_i128,45518020106949615540027691532503989916_i128,52398329022430632709844059648011654767_i128,(-108175801830764952439182861431678283511_i128),75032527541465215866143973178727723188_i128];
_9 = -9223372036854775807_isize;
_5 = _14 * _14;
Goto(bb5)
}
bb5 = {
_9 = 10_isize * (-9223372036854775808_isize);
_17 = _5;
_5 = (*_1) as f64;
_10.1 = [_11,_11,_11,_11,_11,_11,_11];
_10.0 = [(-10_i8)];
_18 = _2 as f32;
_10.0 = [123_i8];
(*_8) = 82_isize;
(*_8) = !(-76_isize);
(*_8) = 9223372036854775807_isize >> (*_1);
(*_4) = 51_isize * (-9223372036854775808_isize);
_18 = 764283419_i32 as f32;
(*_4) = (-9223372036854775808_isize);
_18 = 41476_u16 as f32;
_12 = core::ptr::addr_of_mut!((*_4));
(*_8) = !23_isize;
_6 = (*_4) == (*_8);
_9 = (-9223372036854775808_isize) << (*_1);
_4 = Move(_8);
_14 = -_5;
_19 = &RET;
_13 = core::ptr::addr_of_mut!(_10.0);
Goto(bb6)
}
bb6 = {
_18 = (-23562_i16) as f32;
_14 = _17 - _17;
RET = [168579450597186273638571695669928201715_i128,123123001692586255695004634029645254277_i128,106660384713790563738081027187036802677_i128,(-61485626648136314202839488602312754735_i128),(-111412723495597797240087679019300542038_i128),95866190687922847391099114793256267597_i128,(-139478495669718105534042303819814752163_i128)];
_8 = Move(_4);
_11 = '\u{f22e6}';
_19 = &RET;
_16 = 115_u8 as u16;
_17 = (-104_i8) as f64;
_16 = 62338_u16;
_4 = Move(_12);
match _16 {
0 => bb7,
62338 => bb9,
_ => bb8
}
}
bb7 = {
Goto(bb3)
}
bb8 = {
_11 = '\u{1faae}';
_15 = [33029466789731308108696364687978401352_i128,(-115784811520529033609576391821155972324_i128),127839685612015184961784425048322807383_i128,7670901621471436904487257975998259044_i128,45518020106949615540027691532503989916_i128,52398329022430632709844059648011654767_i128,(-108175801830764952439182861431678283511_i128),75032527541465215866143973178727723188_i128];
_9 = -9223372036854775807_isize;
_5 = _14 * _14;
Goto(bb5)
}
bb9 = {
RET = [(-2940713418997955385155744809207580872_i128),16873983763739308865293527283682048914_i128,15370159112671585421386854595566120468_i128,(-134914610939042918948947396998879736896_i128),(-91267262734456515924497763123494002198_i128),(-58000218168509114563654648699864534917_i128),23370833527869812740008273569686016236_i128];
_20 = _9;
_21 = [10031744982792283315_usize,659135846879550005_usize,12850766291930401576_usize,3_usize,1_usize,1_usize];
_3 = &_19;
_16 = 12877_u16;
_3 = &(*_3);
_13 = core::ptr::addr_of_mut!((*_13));
_22 = [45_u8,66_u8,82_u8,9_u8];
_13 = core::ptr::addr_of_mut!(_10.0);
_21 = [12262506741060101558_usize,2_usize,3_usize,2_usize,8983756918062708720_usize,2035520458268007081_usize];
_23 = -_9;
(*_13) = [(-33_i8)];
_12 = Move(_8);
_9 = !_20;
_1 = &(*_1);
_15 = [(-83231667728735079413378837428394665560_i128),130581326142187806790727561650024871901_i128,10497775716987712268734180330950783891_i128,68369776758299955734087523072925998895_i128,(-35643851365013661314889410644312393235_i128),(-93847599487990404790367175601739939793_i128),147058485773526150753042012006091144988_i128,17649030017483230853470775004003584330_i128];
(*_13) = [(-29_i8)];
_26 = _11;
(*_13) = [(-45_i8)];
_14 = _5;
_25 = _20 * _23;
_1 = &(*_1);
Goto(bb10)
}
bb10 = {
_29.3.1.1 = 17156494115434432902_usize as i64;
RET = [(-70350395732789073116633537749267706656_i128),73937888514395065599080367957365715444_i128,1717134227257162278269459645499621439_i128,121713028548796226465410912120942029312_i128,(-126630671810968224008448762664682929872_i128),(-117593433571951909768038667913279396885_i128),161191983839320391714670895240362324249_i128];
_19 = &RET;
_7 = [4216075029_u32,1222675745_u32,3315503301_u32];
_1 = &_2;
_29.3.1.0 = _17 == _17;
_12 = Move(_4);
_18 = _29.3.1.1 as f32;
match _16 {
0 => bb4,
12877 => bb12,
_ => bb11
}
}
bb11 = {
_1 = &(*_1);
_1 = &(*_1);
_4 = core::ptr::addr_of_mut!(_9);
_5 = 29631_u16 as f64;
_9 = _5 as isize;
RET = [(-126185422324575895279953808451037095055_i128),(-77485657234086876527070105678565312042_i128),(-21767848202970277113137193936979539629_i128),104843972270945246984967098702119775398_i128,112643768319843756736845977090038593158_i128,17161951791564961562990842979245573234_i128,(-2881482347624895120414016610215218174_i128)];
_6 = !true;
_8 = Move(_4);
RET = [24149208290159950888374904630837495071_i128,64289677998356846782492998087988496224_i128,(-18113687705797131079681563882847522941_i128),(-15697938514214949520268337414986700394_i128),134918971758170088718663115114431344272_i128,(-58698166688646435476765398929899605309_i128),99161357533621905224353125476906312372_i128];
_9 = !(-70_isize);
_6 = (*_1) < (*_1);
_6 = false;
_4 = Move(_8);
_9 = (-126_isize) - (-52_isize);
_1 = &_2;
RET = [(-8788319175791204366048542307194442110_i128),111104482189060724855821747526865194816_i128,115838524886253761078065040014450961092_i128,105751254489974665493213044124266880408_i128,97596325628665668930222431653502644487_i128,(-166078998101312979816766241092988889577_i128),(-36459294505884408907266871021877166198_i128)];
_1 = &(*_1);
_5 = _9 as f64;
_7 = [3023010253_u32,1618662460_u32,678880721_u32];
_4 = core::ptr::addr_of_mut!(_9);
_8 = core::ptr::addr_of_mut!(_9);
RET = [(-48314985190336558144699568091537967252_i128),(-121961393500070622090652941153571067623_i128),(-42420264605166192935186067664644747205_i128),(-31194005497795552564875747518749914000_i128),169539931725933114633415048611421613309_i128,139110889945576832610213357100180036053_i128,70623734395995741174102791097010073728_i128];
(*_8) = !(-9223372036854775808_isize);
RET = [(-133066518864070107824994571692076304513_i128),(-140969239506171603886726447768875677720_i128),(-23730796033273819947492516165715641735_i128),2583860044951268659271802047201163390_i128,89370772029954549897558039980915689314_i128,(-6412315496133775908784297771349283371_i128),(-165214402008128247305506265314071292600_i128)];
_10.0 = [96_i8];
Goto(bb2)
}
bb12 = {
_29.0 = -_18;
_29.3.1.0 = _6 ^ _6;
_5 = 21614_i16 as f64;
_29.2 = Adt18::Variant1 { fld0: (-67_i8),fld1: _16,fld2: _29.3.1.1 };
_18 = _29.0;
_10.0 = [97_i8];
_12 = core::ptr::addr_of_mut!(_23);
_29.2 = Adt18::Variant1 { fld0: 66_i8,fld1: _16,fld2: _29.3.1.1 };
_29.3.0 = 113301379144619165385019058750999565610_i128 as u16;
RET = [(-153570278455210224964116677376289969960_i128),(-114071286166308378875649329814025840516_i128),65217791887920207719232421152746935401_i128,(-152965384102471055966177364544879237377_i128),(-80267781553962633616435349261633008935_i128),(-140368523867986764689002341049571974225_i128),152305056607409247246549578100760699037_i128];
_29.0 = _18;
_5 = 81527957393176451203094346783077854762_i128 as f64;
_11 = _26;
_8 = Move(_12);
_6 = !_29.3.1.0;
_23 = !_25;
_29.3.1.0 = !_6;
_5 = _17 - _14;
_9 = !_25;
_10.0 = [43_i8];
_29.2 = Adt18::Variant1 { fld0: 113_i8,fld1: _29.3.0,fld2: _29.3.1.1 };
_19 = &_29.3.2;
_4 = core::ptr::addr_of_mut!(_9);
_29.2 = Adt18::Variant0 { fld0: _6,fld1: 11132032600658511801331664072496673392_i128,fld2: 15522150746196076617_u64,fld3: 2_usize,fld4: _29.3.0 };
place!(Field::<u64>(Variant(_29.2, 0), 2)) = (*_1) as u64;
_1 = &_2;
Call(_10.0 = core::intrinsics::transmute(_29.3.1.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_30 = _23 - (*_4);
place!(Field::<usize>(Variant(_29.2, 0), 3)) = 2_usize;
_29.3.1.0 = (*_1) < (*_1);
_33 = (27924413539330407942208464720126719311_i128, _20);
_23 = _29.3.1.1 as isize;
_10.1 = [_11,_11,_11,_26,_26,_26,_26];
_4 = core::ptr::addr_of_mut!(_20);
_21 = [Field::<usize>(Variant(_29.2, 0), 3),Field::<usize>(Variant(_29.2, 0), 3),Field::<usize>(Variant(_29.2, 0), 3),Field::<usize>(Variant(_29.2, 0), 3),Field::<usize>(Variant(_29.2, 0), 3),Field::<usize>(Variant(_29.2, 0), 3)];
_10.1 = [_26,_26,_26,_11,_11,_11,_26];
_1 = &_2;
_27 = !_6;
_13 = core::ptr::addr_of_mut!((*_13));
_12 = Move(_8);
_33 = (136819281892658561339075268753386011778_i128, _25);
_2 = 142620808064113611929498550213902799663_u128;
_29.3.2 = RET;
place!(Field::<u64>(Variant(_29.2, 0), 2)) = 481926659906091580_u64 + 1340514149366620813_u64;
_29.3.1 = (Field::<bool>(Variant(_29.2, 0), 0), 3280910452502018103_i64);
Goto(bb14)
}
bb14 = {
_12 = core::ptr::addr_of_mut!(_25);
_36 = -_9;
_29.3.1 = (Field::<bool>(Variant(_29.2, 0), 0), (-3769140962695281177_i64));
RET = [_33.0,_33.0,_33.0,_33.0,_33.0,_33.0,_33.0];
_10.1 = [_11,_11,_11,_26,_26,_11,_11];
_38.0 = _11 as i128;
_19 = &RET;
_29.1 = 8529_i16 ^ 30314_i16;
_8 = core::ptr::addr_of_mut!(_20);
_16 = Field::<u16>(Variant(_29.2, 0), 4);
_29.3.3 = _33.0 as u8;
_38 = _33;
_29.1 = 27819_i16 << (*_12);
_29.3.0 = !_16;
(*_12) = !_38.1;
_3 = &_19;
_35 = _29.0 * _29.0;
_3 = &_19;
_33.1 = _36;
_29.0 = -_35;
_10.0 = [(-33_i8)];
_29.3.1 = (_27, 3747977235172915782_i64);
_29.1 = (-23207_i16) >> _29.3.1.1;
_38.0 = _33.0 | _33.0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(6_usize, 7_usize, Move(_7), 25_usize, Move(_25), 21_usize, Move(_21), 33_usize, Move(_33)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(6_usize, 38_usize, Move(_38), 20_usize, Move(_20), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(6_usize, 30_usize, Move(_30), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> bool {
mir! {
type RET = bool;
let _1: (char, &'static [char; 7], *mut isize);
let _2: Adt30;
let _3: f64;
let _4: f32;
let _5: Adt30;
let _6: char;
let _7: &'static Adt41;
let _8: [i64; 8];
let _9: [isize; 7];
let _10: *mut Adt36;
let _11: i8;
let _12: *mut (u8,);
let _13: (*mut usize, u16, *const f64, i16);
let _14: f32;
let _15: [u8; 4];
let _16: u32;
let _17: [u8; 6];
let _18: ();
let _19: ();
{
RET = !false;
RET = true | false;
_1.0 = '\u{fa258}';
_1.0 = '\u{69be8}';
RET = _1.0 != _1.0;
_1.0 = '\u{2ad8}';
RET = false | false;
RET = false;
RET = !false;
RET = false;
_2.fld0 = 191_u8 as u16;
RET = true;
Goto(bb1)
}
bb1 = {
_2 = Adt30 { fld0: 48298_u16 };
_3 = (-1168309183_i32) as f64;
_2.fld0 = 46_i8 as u16;
_2.fld0 = 407_u16;
_3 = 140676834_u32 as f64;
_5.fld0 = (-27_i8) as u16;
RET = !true;
_6 = _1.0;
_6 = _1.0;
_1.0 = _6;
RET = false;
_4 = _3 as f32;
RET = _2.fld0 > _2.fld0;
_5.fld0 = (-31927_i16) as u16;
_5 = Move(_2);
_8 = [40549879585588244_i64,(-3286663553936911254_i64),(-2013445612662563005_i64),(-7740885139870365197_i64),(-7321773583471060804_i64),7506026367806722312_i64,9159457264142583969_i64,(-4436371223524132055_i64)];
_9 = [(-120_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_6 = _1.0;
RET = !true;
_1.0 = _6;
_6 = _1.0;
_1.0 = _6;
_2.fld0 = _5.fld0;
_4 = _5.fld0 as f32;
Goto(bb2)
}
bb2 = {
RET = false | false;
_1.0 = _6;
_2 = Adt30 { fld0: _5.fld0 };
_2.fld0 = _1.0 as u16;
_11 = _2.fld0 as i8;
RET = true ^ true;
RET = _4 >= _4;
RET = !true;
_5.fld0 = !_2.fld0;
_11 = 20_i8 & (-95_i8);
_3 = (-165498277127547667638685274338082656757_i128) as f64;
_3 = _5.fld0 as f64;
_4 = 4594570666728133455_u64 as f32;
Goto(bb3)
}
bb3 = {
_13.3 = 11189_i16;
_13.2 = core::ptr::addr_of!(_3);
_2 = Adt30 { fld0: _5.fld0 };
_5.fld0 = _2.fld0;
match _13.3 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
11189 => bb10,
_ => bb9
}
}
bb4 = {
RET = false | false;
_1.0 = _6;
_2 = Adt30 { fld0: _5.fld0 };
_2.fld0 = _1.0 as u16;
_11 = _2.fld0 as i8;
RET = true ^ true;
RET = _4 >= _4;
RET = !true;
_5.fld0 = !_2.fld0;
_11 = 20_i8 & (-95_i8);
_3 = (-165498277127547667638685274338082656757_i128) as f64;
_3 = _5.fld0 as f64;
_4 = 4594570666728133455_u64 as f32;
Goto(bb3)
}
bb5 = {
_2 = Adt30 { fld0: 48298_u16 };
_3 = (-1168309183_i32) as f64;
_2.fld0 = 46_i8 as u16;
_2.fld0 = 407_u16;
_3 = 140676834_u32 as f64;
_5.fld0 = (-27_i8) as u16;
RET = !true;
_6 = _1.0;
_6 = _1.0;
_1.0 = _6;
RET = false;
_4 = _3 as f32;
RET = _2.fld0 > _2.fld0;
_5.fld0 = (-31927_i16) as u16;
_5 = Move(_2);
_8 = [40549879585588244_i64,(-3286663553936911254_i64),(-2013445612662563005_i64),(-7740885139870365197_i64),(-7321773583471060804_i64),7506026367806722312_i64,9159457264142583969_i64,(-4436371223524132055_i64)];
_9 = [(-120_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_6 = _1.0;
RET = !true;
_1.0 = _6;
_6 = _1.0;
_1.0 = _6;
_2.fld0 = _5.fld0;
_4 = _5.fld0 as f32;
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
RET = false;
Call(_11 = core::intrinsics::transmute(RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = (-53_i8) & 93_i8;
_6 = _1.0;
_3 = (-269874244_i32) as f64;
_8 = [(-247453102647242892_i64),5408736419893638445_i64,4241557533379984367_i64,3823539703432212526_i64,741126040408805461_i64,2056759284968729607_i64,5164293810837800862_i64,(-5745415168907281369_i64)];
_6 = _1.0;
RET = _13.3 >= _13.3;
RET = !true;
_2 = Adt30 { fld0: _5.fld0 };
_13.1 = !_2.fld0;
_13.2 = core::ptr::addr_of!(_3);
_5 = Move(_2);
_5 = Adt30 { fld0: _13.1 };
_2.fld0 = _13.1;
_9 = [117_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-7_isize),(-9223372036854775808_isize),(-18_isize)];
RET = true;
_9 = [98_isize,(-9223372036854775808_isize),112_isize,9223372036854775807_isize,75_isize,9223372036854775807_isize,83_isize];
RET = false & false;
RET = _13.3 >= _13.3;
match _13.3 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
11189 => bb20,
_ => bb19
}
}
bb12 = {
RET = false;
Call(_11 = core::intrinsics::transmute(RET), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_2 = Adt30 { fld0: 48298_u16 };
_3 = (-1168309183_i32) as f64;
_2.fld0 = 46_i8 as u16;
_2.fld0 = 407_u16;
_3 = 140676834_u32 as f64;
_5.fld0 = (-27_i8) as u16;
RET = !true;
_6 = _1.0;
_6 = _1.0;
_1.0 = _6;
RET = false;
_4 = _3 as f32;
RET = _2.fld0 > _2.fld0;
_5.fld0 = (-31927_i16) as u16;
_5 = Move(_2);
_8 = [40549879585588244_i64,(-3286663553936911254_i64),(-2013445612662563005_i64),(-7740885139870365197_i64),(-7321773583471060804_i64),7506026367806722312_i64,9159457264142583969_i64,(-4436371223524132055_i64)];
_9 = [(-120_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_6 = _1.0;
RET = !true;
_1.0 = _6;
_6 = _1.0;
_1.0 = _6;
_2.fld0 = _5.fld0;
_4 = _5.fld0 as f32;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
RET = false | false;
_1.0 = _6;
_2 = Adt30 { fld0: _5.fld0 };
_2.fld0 = _1.0 as u16;
_11 = _2.fld0 as i8;
RET = true ^ true;
RET = _4 >= _4;
RET = !true;
_5.fld0 = !_2.fld0;
_11 = 20_i8 & (-95_i8);
_3 = (-165498277127547667638685274338082656757_i128) as f64;
_3 = _5.fld0 as f64;
_4 = 4594570666728133455_u64 as f32;
Goto(bb3)
}
bb18 = {
RET = false | false;
_1.0 = _6;
_2 = Adt30 { fld0: _5.fld0 };
_2.fld0 = _1.0 as u16;
_11 = _2.fld0 as i8;
RET = true ^ true;
RET = _4 >= _4;
RET = !true;
_5.fld0 = !_2.fld0;
_11 = 20_i8 & (-95_i8);
_3 = (-165498277127547667638685274338082656757_i128) as f64;
_3 = _5.fld0 as f64;
_4 = 4594570666728133455_u64 as f32;
Goto(bb3)
}
bb19 = {
_13.3 = 11189_i16;
_13.2 = core::ptr::addr_of!(_3);
_2 = Adt30 { fld0: _5.fld0 };
_5.fld0 = _2.fld0;
match _13.3 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
11189 => bb10,
_ => bb9
}
}
bb20 = {
_14 = -_4;
_14 = _13.3 as f32;
_15 = [180_u8,77_u8,56_u8,197_u8];
_15 = [202_u8,100_u8,27_u8,235_u8];
_14 = -_4;
_15 = [89_u8,16_u8,247_u8,193_u8];
_17 = [72_u8,249_u8,14_u8,253_u8,60_u8,247_u8];
_1.0 = _6;
_4 = _14;
_15 = [210_u8,213_u8,77_u8,67_u8];
Goto(bb21)
}
bb21 = {
Call(_18 = dump_var(7_usize, 6_usize, Move(_6), 11_usize, Move(_11), 9_usize, Move(_9), 19_usize, _19), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8() -> f64 {
mir! {
type RET = f64;
let _1: u64;
let _2: [i8; 1];
let _3: i16;
let _4: ([i8; 1], [char; 7], *const [u128; 3]);
let _5: [u32; 3];
let _6: char;
let _7: isize;
let _8: &'static f32;
let _9: i16;
let _10: [u32; 2];
let _11: Adt47;
let _12: *const u32;
let _13: [u8; 6];
let _14: u128;
let _15: f64;
let _16: &'static *const i64;
let _17: u64;
let _18: *mut *const u32;
let _19: ([u128; 5], [i8; 1], isize);
let _20: (Adt18, (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8)), bool, ([i64; 8], u64));
let _21: &'static &'static [i128; 7];
let _22: isize;
let _23: &'static Adt60;
let _24: i32;
let _25: ();
let _26: ();
{
RET = (-6597456217922266541_i64) as f64;
RET = (-618038243_i32) as f64;
RET = (-28849_i16) as f64;
RET = 107326461823842109948613721164046428743_u128 as f64;
_1 = 1500713577210361757_u64 - 8087848365144337361_u64;
_1 = 3820999342482963901_u64;
RET = _1 as f64;
_1 = 2142767400661936428_u64 - 16531172637747963195_u64;
RET = 1243431183_u32 as f64;
RET = 2167678722753608090_i64 as f64;
RET = 9223372036854775807_isize as f64;
_1 = !13702730648954966589_u64;
_1 = 4665357742302807963_u64 >> 26971_u16;
_2 = [17_i8];
_1 = 30_u8 as u64;
_1 = 6796743634208284322_u64;
RET = (-117560585_i32) as f64;
_2 = [(-8_i8)];
_2 = [31_i8];
_3 = !(-12643_i16);
_4.1 = ['\u{b15f8}','\u{8511c}','\u{c0a40}','\u{9d70c}','\u{87166}','\u{c897a}','\u{1e618}'];
RET = 1_usize as f64;
_4.0 = _2;
_1 = !8126975831973048199_u64;
_4.1 = ['\u{ec735}','\u{8f25a}','\u{28a06}','\u{f44ed}','\u{d7080}','\u{e9cb5}','\u{adfb7}'];
RET = 5290777342587703859_i64 as f64;
Call(RET = fn9(_4.1, _4.1, _4.1, _4.1, _4.1, _4.1, _4.1, _4.1, _4.1, _4.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = '\u{3c0}';
RET = 58195_u16 as f64;
_1 = 7160304067506331285_u64 << _3;
_2 = [(-6_i8)];
_1 = 10116873882398380211_u64 & 1233531015405183086_u64;
_4.0 = _2;
_2 = [107_i8];
_1 = !4102905230249761483_u64;
RET = _1 as f64;
_5 = [3896767384_u32,3021418_u32,3596258573_u32];
_5 = [4165432193_u32,2482256821_u32,3812848705_u32];
_1 = 12088530995796890161_u64 * 9525575258603681946_u64;
_1 = _3 as u64;
_6 = '\u{41969}';
_5 = [3009377875_u32,1846196149_u32,1428677438_u32];
_3 = 38570_u16 as i16;
_4.0 = _2;
_5 = [2051475990_u32,321361343_u32,1453210859_u32];
Goto(bb2)
}
bb2 = {
_5 = [1137559126_u32,9657989_u32,2234867203_u32];
_2 = [4_i8];
_5 = [635458573_u32,2002914408_u32,2951231754_u32];
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_1 = !4592542306036945404_u64;
RET = 69_i8 as f64;
_3 = 4109_i16;
RET = _1 as f64;
_6 = '\u{3b13e}';
_7 = -(-9223372036854775808_isize);
_2 = _4.0;
_2 = _4.0;
_4.0 = _2;
_7 = (-9223372036854775808_isize);
_4.0 = [(-110_i8)];
_2 = _4.0;
_5 = [3335424096_u32,2831457030_u32,476785636_u32];
RET = _7 as f64;
_7 = _3 as isize;
_5 = [2217120145_u32,1955809559_u32,2398753199_u32];
_4.0 = _2;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
4109 => bb6,
_ => bb5
}
}
bb3 = {
_6 = '\u{3c0}';
RET = 58195_u16 as f64;
_1 = 7160304067506331285_u64 << _3;
_2 = [(-6_i8)];
_1 = 10116873882398380211_u64 & 1233531015405183086_u64;
_4.0 = _2;
_2 = [107_i8];
_1 = !4102905230249761483_u64;
RET = _1 as f64;
_5 = [3896767384_u32,3021418_u32,3596258573_u32];
_5 = [4165432193_u32,2482256821_u32,3812848705_u32];
_1 = 12088530995796890161_u64 * 9525575258603681946_u64;
_1 = _3 as u64;
_6 = '\u{41969}';
_5 = [3009377875_u32,1846196149_u32,1428677438_u32];
_3 = 38570_u16 as i16;
_4.0 = _2;
_5 = [2051475990_u32,321361343_u32,1453210859_u32];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_2 = _4.0;
RET = _1 as f64;
_3 = (-30478_i16);
_2 = [(-75_i8)];
_10 = [100979933_u32,2414611816_u32];
_4.0 = [122_i8];
_10 = [3162491457_u32,2935270664_u32];
RET = 329069518922053420715457220041506482991_u128 as f64;
RET = _3 as f64;
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_13 = [179_u8,141_u8,139_u8,232_u8,157_u8,15_u8];
_11 = Adt47::Variant0 { fld0: _10 };
_2 = [(-52_i8)];
_2 = _4.0;
RET = 1303152860_i32 as f64;
place!(Field::<[u32; 2]>(Variant(_11, 0), 0)) = [2670448552_u32,347644861_u32];
_9 = _3 << _3;
_10 = [1530801824_u32,3379743087_u32];
_7 = _6 as isize;
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
match _3 {
0 => bb1,
340282366920938463463374607431768180978 => bb7,
_ => bb4
}
}
bb7 = {
_15 = RET * RET;
_10 = [3133809189_u32,2854387604_u32];
SetDiscriminant(_11, 1);
place!(Field::<f32>(Variant(_11, 1), 0)) = 8258813949283162329_i64 as f32;
_8 = &place!(Field::<f32>(Variant(_11, 1), 0));
_4.0 = _2;
place!(Field::<f32>(Variant(_11, 1), 0)) = 196_u8 as f32;
_8 = &place!(Field::<f32>(Variant(_11, 1), 0));
_5 = [1842904110_u32,771056196_u32,3333943998_u32];
_5 = [2828798486_u32,2182842588_u32,3870253773_u32];
_5 = [2427438609_u32,2480333694_u32,59787892_u32];
_14 = !316806439132733837280238187541047217378_u128;
_7 = !102_isize;
_3 = _9 & _9;
_14 = 110670013199638670399832197039993879735_u128 ^ 176007021991940975878959549513902336411_u128;
_8 = &(*_8);
_18 = core::ptr::addr_of_mut!(_12);
_17 = !_1;
_13 = [125_u8,70_u8,128_u8,189_u8,19_u8,163_u8];
_7 = 9223372036854775807_isize << _3;
_8 = &(*_8);
Goto(bb8)
}
bb8 = {
_15 = RET;
_4.0 = [(-119_i8)];
_6 = '\u{b180f}';
_1 = _17 & _17;
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_3 = _9 & _9;
_4.0 = _2;
_20.1.0 = -(*_8);
SetDiscriminant(_11, 1);
_7 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_8 = &place!(Field::<f32>(Variant(_11, 1), 0));
_20.2 = true;
Goto(bb9)
}
bb9 = {
_19.0 = [_14,_14,_14,_14,_14];
_20.1.3.2 = [(-114231801539603875511817577742514306977_i128),(-27625688788332226227979564481768957440_i128),95829956106913808713331637867811957676_i128,(-98016617165705015906418513293062406204_i128),160222990279049007270992883947270072257_i128,(-84898912145102250293665335561879309299_i128),67107215403355323693528097463195717377_i128];
Goto(bb10)
}
bb10 = {
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_20.1.3.0 = 21639_u16 | 22474_u16;
_7 = !(-112_isize);
_11 = Adt47::Variant1 { fld0: _20.1.0 };
_17 = _1 + _1;
_19.2 = _7 ^ _7;
_20.3.0 = [(-4128445151183787991_i64),4099719722160410715_i64,(-146682361547484555_i64),5533850333167786240_i64,75232556704266158_i64,5728369285104674863_i64,(-6101231134005205721_i64),3532031653680430679_i64];
_20.1.3.1.1 = _3 as i64;
Call(_20.2 = fn16(_13, _20.1.3.2, _13, _20.1.3.2, _17, _20.1.3.0, _10, _19.0, _20.1.3.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20.1.3.3 = 145_u8 & 19_u8;
_20.1.3.3 = !83_u8;
_20.3.1 = _17;
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_7 = !_19.2;
_5 = [1430981228_u32,3018343900_u32,3128835966_u32];
_20.1.0 = Field::<f32>(Variant(_11, 1), 0);
_11 = Adt47::Variant0 { fld0: _10 };
_4.0 = [(-68_i8)];
_20.1.3.0 = !36744_u16;
_2 = _4.0;
_1 = _17;
_22 = _19.2;
_20.1.3.1 = (_20.2, 847469975607274131_i64);
_3 = !_9;
_7 = _20.1.0 as isize;
_20.1.3.2 = [8834868261387778784920036630688404987_i128,32152986284997879039804150166416945041_i128,74177541791032515009041287133914080265_i128,5085523852206301621304114483212441093_i128,154854748744498490101714471439645847510_i128,91129679047550425676422372063833099526_i128,71562372527369884286668112690668587251_i128];
_14 = 220877138689668111258749331662205801500_u128 | 253203865232973144100253027416451471742_u128;
_20.1.2 = Adt18::Variant2 { fld0: _6 };
_14 = 213035309244296707831790097034054926725_u128 & 85997650568187484243724339959606806369_u128;
SetDiscriminant(_11, 1);
SetDiscriminant(_20.1.2, 1);
RET = -_15;
match _20.1.3.1.1 {
0 => bb12,
1 => bb13,
847469975607274131 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_19.0 = [_14,_14,_14,_14,_14];
_20.1.3.2 = [(-114231801539603875511817577742514306977_i128),(-27625688788332226227979564481768957440_i128),95829956106913808713331637867811957676_i128,(-98016617165705015906418513293062406204_i128),160222990279049007270992883947270072257_i128,(-84898912145102250293665335561879309299_i128),67107215403355323693528097463195717377_i128];
Goto(bb10)
}
bb14 = {
_15 = RET;
_4.0 = [(-119_i8)];
_6 = '\u{b180f}';
_1 = _17 & _17;
_4.1 = [_6,_6,_6,_6,_6,_6,_6];
_3 = _9 & _9;
_4.0 = _2;
_20.1.0 = -(*_8);
SetDiscriminant(_11, 1);
_7 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_8 = &place!(Field::<f32>(Variant(_11, 1), 0));
_20.2 = true;
Goto(bb9)
}
bb15 = {
_19.1 = [62_i8];
place!(Field::<i8>(Variant(_20.1.2, 1), 0)) = (-110_i8);
_5 = [2629133834_u32,2839403995_u32,2357152452_u32];
_20.1.3.0 = 4986_u16 << _17;
_4.0 = _19.1;
_20.1.1 = _3;
place!(Field::<f32>(Variant(_11, 1), 0)) = _14 as f32;
SetDiscriminant(_11, 0);
_5 = [826384889_u32,160213170_u32,3650158944_u32];
place!(Field::<[u32; 2]>(Variant(_11, 0), 0)) = [1099338897_u32,2269390876_u32];
_20.1.1 = !_3;
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(8_usize, 6_usize, Move(_6), 2_usize, Move(_2), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(8_usize, 1_usize, Move(_1), 10_usize, Move(_10), 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [char; 7],mut _2: [char; 7],mut _3: [char; 7],mut _4: [char; 7],mut _5: [char; 7],mut _6: [char; 7],mut _7: [char; 7],mut _8: [char; 7],mut _9: [char; 7],mut _10: [char; 7]) -> f64 {
mir! {
type RET = f64;
let _11: isize;
let _12: (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _13: isize;
let _14: bool;
let _15: f64;
let _16: *mut (u8,);
let _17: *const i64;
let _18: i128;
let _19: f64;
let _20: [u128; 3];
let _21: isize;
let _22: i8;
let _23: i128;
let _24: u64;
let _25: [u8; 4];
let _26: &'static f32;
let _27: Adt30;
let _28: f32;
let _29: usize;
let _30: *mut *mut isize;
let _31: (u16, (bool, i64), [i128; 7], u8);
let _32: &'static Adt41;
let _33: i32;
let _34: f32;
let _35: bool;
let _36: &'static u128;
let _37: [i128; 7];
let _38: &'static Adt36;
let _39: (*mut usize, u16, *const f64, i16);
let _40: ();
let _41: ();
{
_8 = ['\u{c4410}','\u{cbc13}','\u{a19ea}','\u{e72e0}','\u{8d371}','\u{f159e}','\u{ce5d}'];
RET = 240_u8 as f64;
_11 = -(-9223372036854775808_isize);
_5 = ['\u{cf12a}','\u{4ff1f}','\u{1ebb8}','\u{855c6}','\u{1ddd2}','\u{ee31f}','\u{1044ba}'];
_2 = _9;
_11 = -9223372036854775807_isize;
_8 = _1;
_1 = _3;
_5 = ['\u{d7812}','\u{84cca}','\u{53375}','\u{73d89}','\u{3773}','\u{5ab91}','\u{dd039}'];
RET = _11 as f64;
_12.3.2 = [(-76415965134284030439225233300637720220_i128),23751998070836329811326537946029687093_i128,169597492907928138959341329664736764981_i128,(-58488332615155904580115888216323622370_i128),109751360665580654526689852838635128116_i128,(-95732424915934965129904755555878060562_i128),(-151425726045836391707321946996670327965_i128)];
_12.3.2 = [67353663715785078964836878940978205389_i128,(-116281218888156072720001309294604117770_i128),(-102967785103815307748679933375103202852_i128),(-95539279546572451213538818513900031905_i128),(-117121759515492839607911144374975924770_i128),(-70104217458126924579830377293391142577_i128),108879431963185541110488253871073314207_i128];
_12.3.1 = (false, (-4774797917422622092_i64));
_12.2 = Adt18::Variant1 { fld0: (-119_i8),fld1: 44587_u16,fld2: _12.3.1.1 };
_12.1 = -(-20827_i16);
_8 = _4;
Goto(bb1)
}
bb1 = {
_13 = (-354751692_i32) as isize;
_5 = ['\u{10612b}','\u{84e6d}','\u{6012b}','\u{66dac}','\u{1f807}','\u{3fc53}','\u{3e2af}'];
_5 = _1;
_14 = _12.1 >= _12.1;
_15 = RET;
place!(Field::<u16>(Variant(_12.2, 1), 1)) = 31679_u16;
_4 = ['\u{a936e}','\u{fbcc5}','\u{42f4f}','\u{c1d69}','\u{c51c5}','\u{10df07}','\u{eb36d}'];
place!(Field::<u16>(Variant(_12.2, 1), 1)) = 12550_u16;
_10 = ['\u{9007c}','\u{98959}','\u{1057bc}','\u{77d46}','\u{51b5}','\u{6322b}','\u{91905}'];
place!(Field::<i8>(Variant(_12.2, 1), 0)) = (-94_i8) * 36_i8;
_18 = !100243288730844867147477122206192346290_i128;
_12.3.0 = Field::<u16>(Variant(_12.2, 1), 1) % Field::<u16>(Variant(_12.2, 1), 1);
_13 = -_11;
_15 = -RET;
_17 = core::ptr::addr_of!(_12.3.1.1);
(*_17) = _15 as i64;
_12.3.1.1 = Field::<i64>(Variant(_12.2, 1), 2) | Field::<i64>(Variant(_12.2, 1), 2);
_18 = 10295445968119924153_u64 as i128;
_6 = _7;
_12.3.3 = _12.3.0 as u8;
_2 = ['\u{f3a62}','\u{afe6b}','\u{b0d00}','\u{78ba7}','\u{42957}','\u{dd31c}','\u{14fd5}'];
place!(Field::<i64>(Variant(_12.2, 1), 2)) = (*_17) * (*_17);
_12.0 = _18 as f32;
RET = -_15;
Call(place!(Field::<u16>(Variant(_12.2, 1), 1)) = core::intrinsics::transmute(_12.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = _12.3.1.0 >= _12.3.1.0;
_3 = _1;
_11 = _13;
place!(Field::<u16>(Variant(_12.2, 1), 1)) = !_12.3.0;
_20 = [267828789310525848983274978753352799962_u128,88349332632306831546458546381281077794_u128,132033090404117575241778500893415052091_u128];
_6 = _2;
_6 = ['\u{1876f}','\u{c2c8c}','\u{102d8d}','\u{4daf0}','\u{d24c9}','\u{29fb9}','\u{c4daa}'];
_5 = ['\u{ccd32}','\u{102de1}','\u{4c34a}','\u{f7210}','\u{de59f}','\u{5c996}','\u{26d9e}'];
place!(Field::<i64>(Variant(_12.2, 1), 2)) = _12.3.1.1;
_10 = _5;
_6 = ['\u{964}','\u{4967e}','\u{d6a96}','\u{b505}','\u{3986f}','\u{1d4c7}','\u{1026e0}'];
_19 = 22063527777457924969905655725939378026_u128 as f64;
_11 = (-942023978_i32) as isize;
_21 = _13;
(*_17) = -Field::<i64>(Variant(_12.2, 1), 2);
Call(_22 = fn10(_9, _12.3.2, _12.3.1.1, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<i8>(Variant(_12.2, 1), 0)) = _22 - _22;
_13 = -_21;
_9 = ['\u{513c}','\u{f898a}','\u{84a24}','\u{29c4a}','\u{63278}','\u{1002e7}','\u{c9740}'];
Call(_2 = fn15(_12.3, _21, _9, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = _13 << Field::<i64>(Variant(_12.2, 1), 2);
SetDiscriminant(_12.2, 1);
_12.3.3 = 88_u8 << _21;
_14 = _12.3.1.0 | _12.3.1.0;
_23 = !_18;
_12.3.1 = (_14, (-6064488382984498507_i64));
_9 = ['\u{e52e3}','\u{6f891}','\u{f17f6}','\u{8030}','\u{8b69f}','\u{82ede}','\u{7c0c6}'];
_12.3.2 = [_23,_23,_18,_23,_23,_23,_18];
_3 = _6;
match (*_17) {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
340282366920938463457310119048783712949 => bb10,
_ => bb9
}
}
bb5 = {
place!(Field::<i8>(Variant(_12.2, 1), 0)) = _22 - _22;
_13 = -_21;
_9 = ['\u{513c}','\u{f898a}','\u{84a24}','\u{29c4a}','\u{63278}','\u{1002e7}','\u{c9740}'];
Call(_2 = fn15(_12.3, _21, _9, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_14 = _12.3.1.0 >= _12.3.1.0;
_3 = _1;
_11 = _13;
place!(Field::<u16>(Variant(_12.2, 1), 1)) = !_12.3.0;
_20 = [267828789310525848983274978753352799962_u128,88349332632306831546458546381281077794_u128,132033090404117575241778500893415052091_u128];
_6 = _2;
_6 = ['\u{1876f}','\u{c2c8c}','\u{102d8d}','\u{4daf0}','\u{d24c9}','\u{29fb9}','\u{c4daa}'];
_5 = ['\u{ccd32}','\u{102de1}','\u{4c34a}','\u{f7210}','\u{de59f}','\u{5c996}','\u{26d9e}'];
place!(Field::<i64>(Variant(_12.2, 1), 2)) = _12.3.1.1;
_10 = _5;
_6 = ['\u{964}','\u{4967e}','\u{d6a96}','\u{b505}','\u{3986f}','\u{1d4c7}','\u{1026e0}'];
_19 = 22063527777457924969905655725939378026_u128 as f64;
_11 = (-942023978_i32) as isize;
_21 = _13;
(*_17) = -Field::<i64>(Variant(_12.2, 1), 2);
Call(_22 = fn10(_9, _12.3.2, _12.3.1.1, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_13 = (-354751692_i32) as isize;
_5 = ['\u{10612b}','\u{84e6d}','\u{6012b}','\u{66dac}','\u{1f807}','\u{3fc53}','\u{3e2af}'];
_5 = _1;
_14 = _12.1 >= _12.1;
_15 = RET;
place!(Field::<u16>(Variant(_12.2, 1), 1)) = 31679_u16;
_4 = ['\u{a936e}','\u{fbcc5}','\u{42f4f}','\u{c1d69}','\u{c51c5}','\u{10df07}','\u{eb36d}'];
place!(Field::<u16>(Variant(_12.2, 1), 1)) = 12550_u16;
_10 = ['\u{9007c}','\u{98959}','\u{1057bc}','\u{77d46}','\u{51b5}','\u{6322b}','\u{91905}'];
place!(Field::<i8>(Variant(_12.2, 1), 0)) = (-94_i8) * 36_i8;
_18 = !100243288730844867147477122206192346290_i128;
_12.3.0 = Field::<u16>(Variant(_12.2, 1), 1) % Field::<u16>(Variant(_12.2, 1), 1);
_13 = -_11;
_15 = -RET;
_17 = core::ptr::addr_of!(_12.3.1.1);
(*_17) = _15 as i64;
_12.3.1.1 = Field::<i64>(Variant(_12.2, 1), 2) | Field::<i64>(Variant(_12.2, 1), 2);
_18 = 10295445968119924153_u64 as i128;
_6 = _7;
_12.3.3 = _12.3.0 as u8;
_2 = ['\u{f3a62}','\u{afe6b}','\u{b0d00}','\u{78ba7}','\u{42957}','\u{dd31c}','\u{14fd5}'];
place!(Field::<i64>(Variant(_12.2, 1), 2)) = (*_17) * (*_17);
_12.0 = _18 as f32;
RET = -_15;
Call(place!(Field::<u16>(Variant(_12.2, 1), 1)) = core::intrinsics::transmute(_12.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_15 = RET * RET;
_12.3.1.0 = !_14;
_3 = ['\u{10dde0}','\u{963d4}','\u{10edb7}','\u{d5ec}','\u{186ba}','\u{aa9fe}','\u{9b445}'];
_12.3.1.0 = _12.3.1.1 > (*_17);
_13 = _11;
_23 = _18;
_2 = _7;
_12.2 = Adt18::Variant1 { fld0: _22,fld1: _12.3.0,fld2: (*_17) };
_12.1 = 17147_i16;
_12.0 = Field::<i8>(Variant(_12.2, 1), 0) as f32;
_18 = !_23;
_12.3.2 = [_18,_23,_23,_18,_23,_18,_23];
_2 = _6;
_27.fld0 = Field::<u16>(Variant(_12.2, 1), 1);
_22 = '\u{8be29}' as i8;
_25 = [_12.3.3,_12.3.3,_12.3.3,_12.3.3];
_5 = ['\u{e370}','\u{7384e}','\u{80553}','\u{12413}','\u{c5b8a}','\u{974d6}','\u{b41e4}'];
RET = _15 - _15;
_4 = ['\u{5d30e}','\u{b8c7c}','\u{23473}','\u{d8138}','\u{5a2bc}','\u{20d55}','\u{a1049}'];
_7 = ['\u{d89d3}','\u{ab312}','\u{af7f3}','\u{b13c8}','\u{9f17d}','\u{da9be}','\u{68d12}'];
place!(Field::<i8>(Variant(_12.2, 1), 0)) = _12.3.1.0 as i8;
_12.1 = (-18507_i16) - (-5006_i16);
_12.3.1 = (_14, Field::<i64>(Variant(_12.2, 1), 2));
RET = _11 as f64;
_9 = ['\u{51986}','\u{48f68}','\u{3454f}','\u{67251}','\u{155a6}','\u{3bea4}','\u{1461e}'];
SetDiscriminant(_12.2, 0);
place!(Field::<i128>(Variant(_12.2, 0), 1)) = _23;
_25 = [_12.3.3,_12.3.3,_12.3.3,_12.3.3];
_12.3.1.1 = -(-4680132228574050533_i64);
_12.3.1.0 = _11 <= _13;
_26 = &_12.0;
Call(RET = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_21 = _13 | _13;
(*_17) = _12.3.0 as i64;
_24 = _22 as u64;
_7 = ['\u{3d3e7}','\u{c02c6}','\u{d97d7}','\u{d6ba6}','\u{4ee3c}','\u{d738b}','\u{8d6d0}'];
place!(Field::<u16>(Variant(_12.2, 0), 4)) = _27.fld0 + _12.3.0;
_18 = (*_17) as i128;
RET = _19;
_3 = ['\u{49ac4}','\u{8baf0}','\u{fb91e}','\u{c1562}','\u{a14df}','\u{62bcb}','\u{1f863}'];
_12.1 = -(-29334_i16);
place!(Field::<bool>(Variant(_12.2, 0), 0)) = !_12.3.1.0;
_4 = ['\u{4623b}','\u{a30ca}','\u{10e58c}','\u{1965b}','\u{d6420}','\u{dd8b9}','\u{bca4f}'];
place!(Field::<usize>(Variant(_12.2, 0), 3)) = !15370222313407234939_usize;
RET = _15;
_12.3.2 = [_18,_18,Field::<i128>(Variant(_12.2, 0), 1),Field::<i128>(Variant(_12.2, 0), 1),_18,_23,_18];
_14 = !Field::<bool>(Variant(_12.2, 0), 0);
_13 = _11;
place!(Field::<bool>(Variant(_12.2, 0), 0)) = !_14;
_27 = Adt30 { fld0: Field::<u16>(Variant(_12.2, 0), 4) };
place!(Field::<u64>(Variant(_12.2, 0), 2)) = !_24;
_12.1 = (-8799_i16);
RET = _19 + _19;
(*_17) = 2204680041301494741_i64 >> _12.1;
_21 = _13;
_18 = !Field::<i128>(Variant(_12.2, 0), 1);
Call(place!(Field::<u64>(Variant(_12.2, 0), 2)) = core::intrinsics::bswap(_24), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_28 = -_12.0;
_4 = _5;
_2 = ['\u{194a}','\u{b2e15}','\u{1b0c}','\u{cdd2d}','\u{e3709}','\u{6c088}','\u{eedf7}'];
_3 = ['\u{798cd}','\u{3141c}','\u{6964c}','\u{87c6d}','\u{be699}','\u{e91ef}','\u{d30b8}'];
RET = _19 - _15;
SetDiscriminant(_12.2, 0);
place!(Field::<i128>(Variant(_12.2, 0), 1)) = _12.3.1.1 as i128;
_27.fld0 = !_12.3.0;
(*_17) = 1867410089_u32 as i64;
_12.1 = 30807_i16;
_13 = !_21;
_12.3.1.1 = -(-7397203881246781926_i64);
_12.2 = Adt18::Variant2 { fld0: '\u{ebc69}' };
_31.0 = !_27.fld0;
_31.1.1 = -_12.3.1.1;
_29 = _15 as usize;
_3 = _8;
match _12.1 {
0 => bb8,
1 => bb2,
2 => bb11,
3 => bb7,
30807 => bb13,
_ => bb5
}
}
bb13 = {
_33 = _12.3.3 as i32;
(*_17) = _31.1.1 + _31.1.1;
_27 = Adt30 { fld0: _31.0 };
_17 = core::ptr::addr_of!(_31.1.1);
_28 = _12.3.1.1 as f32;
_10 = ['\u{e8df8}','\u{531a1}','\u{561d7}','\u{102b1a}','\u{37342}','\u{10f8bb}','\u{795c1}'];
_18 = _23;
_31.1.0 = !_12.3.1.0;
_3 = ['\u{bb029}','\u{52a04}','\u{acab7}','\u{33ebb}','\u{6f730}','\u{c563a}','\u{fa28}'];
_12.1 = _22 as i16;
_25 = [_12.3.3,_12.3.3,_12.3.3,_12.3.3];
_12.1 = 2866082233842721086840801994696296335_u128 as i16;
_12.3.1.0 = _14;
_12.3.2 = [_23,_18,_18,_23,_18,_23,_18];
_14 = _31.1.0 < _31.1.0;
_14 = !_12.3.1.0;
_12.3.1.0 = _31.1.0 & _14;
_5 = ['\u{9ec07}','\u{6755b}','\u{61674}','\u{1007d7}','\u{37985}','\u{108a24}','\u{f55df}'];
RET = _19;
_29 = 1_usize;
match _20[_29] {
0 => bb14,
88349332632306831546458546381281077794 => bb16,
_ => bb15
}
}
bb14 = {
_14 = _12.3.1.0 >= _12.3.1.0;
_3 = _1;
_11 = _13;
place!(Field::<u16>(Variant(_12.2, 1), 1)) = !_12.3.0;
_20 = [267828789310525848983274978753352799962_u128,88349332632306831546458546381281077794_u128,132033090404117575241778500893415052091_u128];
_6 = _2;
_6 = ['\u{1876f}','\u{c2c8c}','\u{102d8d}','\u{4daf0}','\u{d24c9}','\u{29fb9}','\u{c4daa}'];
_5 = ['\u{ccd32}','\u{102de1}','\u{4c34a}','\u{f7210}','\u{de59f}','\u{5c996}','\u{26d9e}'];
place!(Field::<i64>(Variant(_12.2, 1), 2)) = _12.3.1.1;
_10 = _5;
_6 = ['\u{964}','\u{4967e}','\u{d6a96}','\u{b505}','\u{3986f}','\u{1d4c7}','\u{1026e0}'];
_19 = 22063527777457924969905655725939378026_u128 as f64;
_11 = (-942023978_i32) as isize;
_21 = _13;
(*_17) = -Field::<i64>(Variant(_12.2, 1), 2);
Call(_22 = fn10(_9, _12.3.2, _12.3.1.1, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_11 = _13 << Field::<i64>(Variant(_12.2, 1), 2);
SetDiscriminant(_12.2, 1);
_12.3.3 = 88_u8 << _21;
_14 = _12.3.1.0 | _12.3.1.0;
_23 = !_18;
_12.3.1 = (_14, (-6064488382984498507_i64));
_9 = ['\u{e52e3}','\u{6f891}','\u{f17f6}','\u{8030}','\u{8b69f}','\u{82ede}','\u{7c0c6}'];
_12.3.2 = [_23,_23,_18,_23,_23,_23,_18];
_3 = _6;
match (*_17) {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
340282366920938463457310119048783712949 => bb10,
_ => bb9
}
}
bb16 = {
_12.3.1 = _31.1;
_3 = [_9[_29],_5[_29],_6[_29],_2[_29],_2[_29],_9[_29],_5[_29]];
_4 = _9;
_37 = [_18,_23,_18,_12.3.2[_29],_23,_18,_23];
_12.2 = Adt18::Variant1 { fld0: _22,fld1: _31.0,fld2: (*_17) };
_12.3 = (_31.0, _31.1, _37, _25[_29]);
_3 = [_5[_29],_1[_29],_2[_29],_6[_29],_4[_29],_4[_29],_8[_29]];
_12.3.1.0 = _31.1.0;
SetDiscriminant(_12.2, 0);
_39.3 = _12.1 & _12.1;
_6 = [_3[_29],_4[_29],_9[_29],_1[_29],_2[_29],_3[_29],_5[_29]];
_31.0 = _27.fld0;
_6 = [_1[_29],_3[_29],_1[_29],_8[_29],_4[_29],_10[_29],_3[_29]];
_2[_29] = _9[_29];
_39.3 = _21 as i16;
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(9_usize, 23_usize, Move(_23), 3_usize, Move(_3), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(9_usize, 6_usize, Move(_6), 14_usize, Move(_14), 2_usize, Move(_2), 33_usize, Move(_33)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(9_usize, 25_usize, Move(_25), 22_usize, Move(_22), 7_usize, Move(_7), 41_usize, _41), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [char; 7],mut _2: [i128; 7],mut _3: i64,mut _4: [char; 7]) -> i8 {
mir! {
type RET = i8;
let _5: usize;
let _6: i16;
let _7: bool;
let _8: Adt52;
let _9: [bool; 5];
let _10: isize;
let _11: *mut *mut isize;
let _12: bool;
let _13: i128;
let _14: f32;
let _15: ([u128; 5], [i8; 1], isize);
let _16: &'static [char; 7];
let _17: f32;
let _18: isize;
let _19: char;
let _20: isize;
let _21: &'static f32;
let _22: u64;
let _23: &'static u128;
let _24: [i128; 8];
let _25: *const f64;
let _26: (u8,);
let _27: f32;
let _28: ([usize; 4],);
let _29: isize;
let _30: u8;
let _31: &'static Adt41;
let _32: isize;
let _33: isize;
let _34: ();
let _35: ();
{
RET = 1923727115_u32 as i8;
_3 = 8880955320929482448_i64;
RET = 203429395704564961084621320970272221025_u128 as i8;
_4 = _1;
RET = (-93_i8) & 103_i8;
_6 = RET as i16;
_2 = [(-153469748585879528727817401189453652949_i128),66520548612555121285767929383689072806_i128,(-55322738193856943467701013461660307713_i128),51698696208615134607504500698083362268_i128,(-58452335391393319899812773603595422091_i128),109213323580647925597591946720366196563_i128,(-49079129378221682659517713776004402841_i128)];
_5 = 1_usize * 9006520313866805376_usize;
_5 = 11398153426661972586_usize << _6;
_5 = !6_usize;
_8.fld2 = (-750997746_i32);
RET = 107_i8 << _8.fld2;
_8.fld1 = _5 >> RET;
_8.fld3 = Adt18::Variant2 { fld0: '\u{45ed3}' };
_2 = [125517648385513895035340936203421640931_i128,94347360995947066074290897174542478479_i128,(-57465460367417931992086434670265534897_i128),113149157247546703519019384044358176392_i128,(-143689084788185663636238851303304655675_i128),(-129505129598297483475895855506254971621_i128),(-71406914705859409047268237326049545373_i128)];
_9 = [false,true,true,false,true];
match _8.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431017213710 => bb6,
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
place!(Field::<char>(Variant(_8.fld3, 2), 0)) = '\u{6e670}';
RET = !50_i8;
RET = 85_i8;
_5 = _3 as usize;
_9 = [true,true,false,true,true];
_8.fld3 = Adt18::Variant1 { fld0: RET,fld1: 9251_u16,fld2: _3 };
_10 = 266400101965639980962536319972401707982_u128 as isize;
_6 = !27354_i16;
_8.fld1 = _5 * _5;
RET = _6 as i8;
place!(Field::<i8>(Variant(_8.fld3, 1), 0)) = RET | RET;
RET = Field::<i8>(Variant(_8.fld3, 1), 0);
_8.fld3 = Adt18::Variant1 { fld0: RET,fld1: 6035_u16,fld2: _3 };
place!(Field::<u16>(Variant(_8.fld3, 1), 1)) = 165_u16;
RET = -Field::<i8>(Variant(_8.fld3, 1), 0);
_8.fld3 = Adt18::Variant2 { fld0: '\u{31680}' };
Goto(bb7)
}
bb7 = {
_3 = _6 as i64;
_2 = [(-24567220714384004748874279684880052176_i128),(-87337304839814858082946271569586132913_i128),(-90734770973270584793347403085034976486_i128),(-104681446894591968673652013967552531832_i128),(-66501051180458678285002653576644508925_i128),(-96172230625813457709993313457865496102_i128),154607511695239597019808008892322072334_i128];
_2 = [(-80249071091427974869435861466898029103_i128),166462006345726590833612813691499664545_i128,63518485318851970583255779434744287550_i128,(-75454075273579589418446427209780260290_i128),120853064999465607616668434789355235270_i128,167155994618762395203807125415418427588_i128,(-57412051254700712889041680444300132244_i128)];
_10 = _8.fld2 as isize;
Call(_1 = fn11(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<char>(Variant(_8.fld3, 2), 0)) = '\u{1c96}';
_14 = _5 as f32;
_7 = !false;
_15.1 = [RET];
_15.1 = [RET];
_1 = _4;
_2 = [(-111733848465389701023970595943078347785_i128),(-138758688913012916771760977227892375743_i128),84208972314782706236964038135702360726_i128,(-17519373199884032964135942514578363767_i128),168330091745975305816954908102839254676_i128,53541122634585399261827825921903689365_i128,(-153715813307949688231423410027013383846_i128)];
_6 = (-249_i16);
_8.fld3 = Adt18::Variant2 { fld0: '\u{2b960}' };
_8.fld3 = Adt18::Variant1 { fld0: RET,fld1: 36732_u16,fld2: _3 };
_15.1 = [RET];
_19 = '\u{e25e6}';
place!(Field::<i64>(Variant(_8.fld3, 1), 2)) = !_3;
_12 = !_7;
_13 = 650277962_u32 as i128;
place!(Field::<i8>(Variant(_8.fld3, 1), 0)) = RET ^ RET;
_8.fld3 = Adt18::Variant1 { fld0: RET,fld1: 13690_u16,fld2: _3 };
match _8.fld2 {
340282366920938463463374607431017213710 => bb9,
_ => bb7
}
}
bb9 = {
_8.fld2 = _13 as i32;
_19 = '\u{86c54}';
_7 = !_12;
_9 = [_7,_12,_7,_7,_12];
_14 = _8.fld2 as f32;
_15.0 = [186429458667576634159189382920172885488_u128,64697034329583635950142808957794189050_u128,250800068009936860770543274727987313513_u128,109725495741362113051558913992928822412_u128,272918724836485979217973069642334158696_u128];
_2 = [_13,_13,_13,_13,_13,_13,_13];
_15.2 = _10;
_20 = _10;
_16 = &_4;
_8.fld1 = _5;
_8.fld2 = 171823637755418353085888571787367320522_u128 as i32;
_17 = -_14;
_21 = &_14;
_7 = _5 < _8.fld1;
place!(Field::<i8>(Variant(_8.fld3, 1), 0)) = RET & RET;
_10 = _15.2 << Field::<i8>(Variant(_8.fld3, 1), 0);
RET = _17 as i8;
place!(Field::<i64>(Variant(_8.fld3, 1), 2)) = _3;
_5 = !_8.fld1;
_19 = '\u{3cf6b}';
RET = Field::<i8>(Variant(_8.fld3, 1), 0);
_15.0 = [113297990781176017822209372653342675355_u128,68361695642356964411272993836172673597_u128,274799607975774418490357731900185557081_u128,260255931650370925075314801612652387555_u128,89432942231083107628203108211150275195_u128];
_20 = _6 as isize;
Call(_8.fld0 = fn12(Move(_16)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
match _6 {
0 => bb1,
1 => bb2,
2 => bb8,
340282366920938463463374607431768211207 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_13 = _10 as i128;
_7 = !_12;
_8.fld1 = 11723630859924627217_u64 as usize;
_27 = Field::<i64>(Variant(_8.fld3, 1), 2) as f32;
_26 = (188_u8,);
place!(Field::<i64>(Variant(_8.fld3, 1), 2)) = -_3;
_22 = 2497433611735922269_u64 & 5817215964927063090_u64;
_8.fld1 = _3 as usize;
_15.1 = [RET];
_15.1 = [Field::<i8>(Variant(_8.fld3, 1), 0)];
Goto(bb13)
}
bb13 = {
_8.fld1 = !_5;
_21 = &_17;
_15.2 = _10 ^ _10;
_27 = _14;
_6 = 8192_i16;
_18 = _15.2;
_5 = _8.fld1 << RET;
_9 = [_7,_12,_12,_7,_12];
place!(Field::<i8>(Variant(_8.fld3, 1), 0)) = RET >> _22;
RET = !Field::<i8>(Variant(_8.fld3, 1), 0);
_15.1 = [Field::<i8>(Variant(_8.fld3, 1), 0)];
_14 = _22 as f32;
RET = _6 as i8;
_8.fld2 = _7 as i32;
_8.fld3 = Adt18::Variant0 { fld0: _7,fld1: _13,fld2: _22,fld3: _5,fld4: 3210_u16 };
place!(Field::<u16>(Variant(_8.fld3, 0), 4)) = _26.0 as u16;
RET = 6_i8 + 45_i8;
_7 = Field::<bool>(Variant(_8.fld3, 0), 0);
place!(Field::<u64>(Variant(_8.fld3, 0), 2)) = !_22;
_8.fld2 = 219983962_i32;
_20 = _10 - _10;
_2 = [_13,_13,_13,_13,Field::<i128>(Variant(_8.fld3, 0), 1),_13,_13];
SetDiscriminant(_8.fld3, 2);
_28.0 = [_5,_5,_5,_5];
Goto(bb14)
}
bb14 = {
_16 = &_4;
_29 = -_10;
_8.fld3 = Adt18::Variant2 { fld0: _19 };
_26.0 = 69188798034631983648041628746721426282_u128 as u8;
_26.0 = !51_u8;
_29 = Field::<char>(Variant(_8.fld3, 2), 0) as isize;
_15.0 = [157550198719686870355550412953465832753_u128,50265721050486724851855487630694766854_u128,36808186404937735862147567920109397159_u128,175937144049744791674929793415297085012_u128,328268213507199078691493450299887223462_u128];
place!(Field::<char>(Variant(_8.fld3, 2), 0)) = _19;
SetDiscriminant(_8.fld3, 2);
RET = _7 as i8;
RET = 109_i8;
_24 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8.fld3 = Adt18::Variant0 { fld0: _7,fld1: _13,fld2: _22,fld3: _5,fld4: 53419_u16 };
_20 = _15.2 & _18;
_28.0 = [Field::<usize>(Variant(_8.fld3, 0), 3),Field::<usize>(Variant(_8.fld3, 0), 3),_5,_5];
place!(Field::<bool>(Variant(_8.fld3, 0), 0)) = !_12;
_19 = '\u{7fd29}';
_12 = _7 ^ Field::<bool>(Variant(_8.fld3, 0), 0);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(10_usize, 10_usize, Move(_10), 29_usize, Move(_29), 24_usize, Move(_24), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(10_usize, 19_usize, Move(_19), 28_usize, Move(_28), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(10_usize, 15_usize, Move(_15), 20_usize, Move(_20), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11() -> [char; 7] {
mir! {
type RET = [char; 7];
let _1: f64;
let _2: &'static Adt41;
let _3: *mut isize;
let _4: i32;
let _5: *const i64;
let _6: &'static Adt41;
let _7: &'static Adt41;
let _8: [u8; 6];
let _9: &'static &'static [i128; 7];
let _10: *const f64;
let _11: u16;
let _12: i128;
let _13: f64;
let _14: [usize; 6];
let _15: char;
let _16: *mut Adt36;
let _17: f32;
let _18: &'static Adt60;
let _19: &'static Adt36;
let _20: [u8; 4];
let _21: isize;
let _22: ();
let _23: ();
{
RET = ['\u{8bc42}','\u{5d0e9}','\u{a50b2}','\u{783b9}','\u{c6e1d}','\u{a150}','\u{fd0c2}'];
RET = ['\u{8d08}','\u{9a2de}','\u{104b55}','\u{fce84}','\u{4ca45}','\u{10aac3}','\u{10d564}'];
RET = ['\u{6bc02}','\u{2642f}','\u{da1c2}','\u{b014a}','\u{e9920}','\u{110ea}','\u{a032c}'];
_1 = 239_u8 as f64;
_1 = 13774245043057324845_u64 as f64;
_1 = 2833025653_u32 as f64;
_1 = (-8943026548657795663_i64) as f64;
RET = ['\u{29765}','\u{105ca5}','\u{5f37e}','\u{107e56}','\u{55580}','\u{51922}','\u{da1ad}'];
RET = ['\u{1b59}','\u{3ecc0}','\u{d4ed}','\u{f6778}','\u{daa15}','\u{7dbec}','\u{1509}'];
_1 = 143264278288904534356986162101471879310_u128 as f64;
RET = ['\u{f89c6}','\u{acbb9}','\u{cf02}','\u{96e6}','\u{f0e44}','\u{ace8b}','\u{8cae3}'];
RET = ['\u{106403}','\u{10c761}','\u{f832e}','\u{972b9}','\u{48519}','\u{66aeb}','\u{b181a}'];
_1 = (-28614_i16) as f64;
RET = ['\u{5f230}','\u{c10aa}','\u{a8adc}','\u{4491c}','\u{4c8e2}','\u{70e35}','\u{625b2}'];
RET = ['\u{c4347}','\u{34e19}','\u{91306}','\u{8e7af}','\u{1feb8}','\u{faca5}','\u{287d2}'];
RET = ['\u{4206a}','\u{9081}','\u{87de8}','\u{1038b9}','\u{e75fd}','\u{ceeae}','\u{a3dba}'];
RET = ['\u{d52d4}','\u{2029}','\u{c5c4b}','\u{b30e9}','\u{a346b}','\u{c18a2}','\u{eb528}'];
Goto(bb1)
}
bb1 = {
_1 = 183515100667108076911540962602721639912_u128 as f64;
_1 = (-9223372036854775808_isize) as f64;
_1 = 3_i8 as f64;
RET = ['\u{ca3e1}','\u{c7a2}','\u{cddc1}','\u{4b48}','\u{fe33b}','\u{1f6b1}','\u{eb96c}'];
RET = ['\u{dbd4b}','\u{d3ebf}','\u{779a1}','\u{8fecb}','\u{603b7}','\u{86cb9}','\u{cefc0}'];
_1 = (-49559175704443345738775416122923883714_i128) as f64;
_1 = 10172416121314455297_usize as f64;
RET = ['\u{c156a}','\u{a0a77}','\u{9d0b6}','\u{27491}','\u{ce77a}','\u{f586d}','\u{dd626}'];
_1 = 1212211747_i32 as f64;
Goto(bb2)
}
bb2 = {
_1 = 19_i8 as f64;
_4 = (-9223372036854775808_isize) as i32;
_1 = 6790137432150270419_usize as f64;
_4 = (-24456_i16) as i32;
_1 = 2826772909_u32 as f64;
RET = ['\u{2fa4f}','\u{7239c}','\u{b2129}','\u{bd014}','\u{950f3}','\u{d90ab}','\u{c0e4b}'];
RET = ['\u{f6126}','\u{ca4ce}','\u{2f96}','\u{b2f0e}','\u{ba03f}','\u{9b490}','\u{5ba60}'];
RET = ['\u{4e3c1}','\u{780a4}','\u{94223}','\u{2986c}','\u{df7e7}','\u{62e7}','\u{28970}'];
_1 = 191298299987577061854274134404201156267_u128 as f64;
Goto(bb3)
}
bb3 = {
_1 = _4 as f64;
_4 = (-9223372036854775808_isize) as i32;
_4 = (-40_i8) as i32;
RET = ['\u{af384}','\u{8da5a}','\u{e48c6}','\u{31433}','\u{fc156}','\u{31779}','\u{cbbf7}'];
RET = ['\u{154e}','\u{de610}','\u{4203c}','\u{aaeb3}','\u{5cfb3}','\u{10f0b5}','\u{a8989}'];
RET = ['\u{d688a}','\u{dbc75}','\u{e9e3d}','\u{25262}','\u{82fdd}','\u{10f759}','\u{d2306}'];
_1 = 3974724103657118290_u64 as f64;
_4 = (-1330307086_i32) << 1_usize;
_8 = [230_u8,181_u8,66_u8,82_u8,180_u8,248_u8];
Goto(bb4)
}
bb4 = {
RET = ['\u{2003c}','\u{fdfdf}','\u{55545}','\u{5e80f}','\u{d7433}','\u{7cbc8}','\u{a28cd}'];
RET = ['\u{a6e7e}','\u{2c88a}','\u{196b7}','\u{9ed7c}','\u{b0930}','\u{1d92e}','\u{22337}'];
_8 = [179_u8,101_u8,228_u8,153_u8,202_u8,229_u8];
RET = ['\u{93035}','\u{b887d}','\u{9730}','\u{2796b}','\u{c2bf8}','\u{7e0b7}','\u{409db}'];
RET = ['\u{f2a76}','\u{692b}','\u{5575}','\u{7c36f}','\u{33c3}','\u{fe08a}','\u{ef960}'];
_8 = [181_u8,86_u8,22_u8,130_u8,247_u8,181_u8];
RET = ['\u{ebbb6}','\u{38ecc}','\u{dd472}','\u{f57db}','\u{84eab}','\u{107a0d}','\u{80ae5}'];
_8 = [140_u8,253_u8,244_u8,146_u8,14_u8,115_u8];
RET = ['\u{3765f}','\u{f1a60}','\u{3229a}','\u{10601f}','\u{e194c}','\u{92829}','\u{58165}'];
_10 = core::ptr::addr_of!(_1);
_11 = !3835_u16;
RET = ['\u{c0532}','\u{3e4a9}','\u{10e890}','\u{7a49c}','\u{10071b}','\u{69d4a}','\u{fab06}'];
_11 = !25621_u16;
(*_10) = 12965384952365977471_u64 as f64;
_8 = [165_u8,253_u8,69_u8,203_u8,146_u8,4_u8];
_10 = core::ptr::addr_of!(_1);
_8 = [231_u8,221_u8,131_u8,220_u8,184_u8,230_u8];
RET = ['\u{e547a}','\u{d3a21}','\u{7c700}','\u{38585}','\u{ccc4f}','\u{1fdf5}','\u{6c923}'];
RET = ['\u{e414f}','\u{3aa3b}','\u{790ac}','\u{d7117}','\u{75527}','\u{4b4f7}','\u{503cc}'];
_10 = core::ptr::addr_of!((*_10));
_11 = 47701_u16;
match _11 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
47701 => bb12,
_ => bb11
}
}
bb5 = {
_1 = _4 as f64;
_4 = (-9223372036854775808_isize) as i32;
_4 = (-40_i8) as i32;
RET = ['\u{af384}','\u{8da5a}','\u{e48c6}','\u{31433}','\u{fc156}','\u{31779}','\u{cbbf7}'];
RET = ['\u{154e}','\u{de610}','\u{4203c}','\u{aaeb3}','\u{5cfb3}','\u{10f0b5}','\u{a8989}'];
RET = ['\u{d688a}','\u{dbc75}','\u{e9e3d}','\u{25262}','\u{82fdd}','\u{10f759}','\u{d2306}'];
_1 = 3974724103657118290_u64 as f64;
_4 = (-1330307086_i32) << 1_usize;
_8 = [230_u8,181_u8,66_u8,82_u8,180_u8,248_u8];
Goto(bb4)
}
bb6 = {
_1 = 19_i8 as f64;
_4 = (-9223372036854775808_isize) as i32;
_1 = 6790137432150270419_usize as f64;
_4 = (-24456_i16) as i32;
_1 = 2826772909_u32 as f64;
RET = ['\u{2fa4f}','\u{7239c}','\u{b2129}','\u{bd014}','\u{950f3}','\u{d90ab}','\u{c0e4b}'];
RET = ['\u{f6126}','\u{ca4ce}','\u{2f96}','\u{b2f0e}','\u{ba03f}','\u{9b490}','\u{5ba60}'];
RET = ['\u{4e3c1}','\u{780a4}','\u{94223}','\u{2986c}','\u{df7e7}','\u{62e7}','\u{28970}'];
_1 = 191298299987577061854274134404201156267_u128 as f64;
Goto(bb3)
}
bb7 = {
_1 = 183515100667108076911540962602721639912_u128 as f64;
_1 = (-9223372036854775808_isize) as f64;
_1 = 3_i8 as f64;
RET = ['\u{ca3e1}','\u{c7a2}','\u{cddc1}','\u{4b48}','\u{fe33b}','\u{1f6b1}','\u{eb96c}'];
RET = ['\u{dbd4b}','\u{d3ebf}','\u{779a1}','\u{8fecb}','\u{603b7}','\u{86cb9}','\u{cefc0}'];
_1 = (-49559175704443345738775416122923883714_i128) as f64;
_1 = 10172416121314455297_usize as f64;
RET = ['\u{c156a}','\u{a0a77}','\u{9d0b6}','\u{27491}','\u{ce77a}','\u{f586d}','\u{dd626}'];
_1 = 1212211747_i32 as f64;
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
Return()
}
bb12 = {
_8 = [110_u8,2_u8,217_u8,79_u8,248_u8,10_u8];
(*_10) = 24607_i16 as f64;
_4 = !811718765_i32;
_4 = 5_u8 as i32;
_12 = 95609310050291931658770105040626159561_i128;
_10 = core::ptr::addr_of!((*_10));
_11 = 62774_u16;
_13 = _1;
_4 = !(-1187269249_i32);
_11 = 53379_u16;
RET = ['\u{c735e}','\u{c92bc}','\u{44d8b}','\u{e5a29}','\u{927b4}','\u{6a7e1}','\u{d704d}'];
RET = ['\u{5c4dc}','\u{6a9a9}','\u{77867}','\u{c1917}','\u{8269e}','\u{49d13}','\u{48ff0}'];
_11 = 39096_u16;
_1 = _13 * _13;
(*_10) = _13;
Goto(bb13)
}
bb13 = {
_13 = _1 - (*_10);
RET = ['\u{7afee}','\u{6ce78}','\u{7cc1b}','\u{c859a}','\u{d2795}','\u{dac06}','\u{c5c4d}'];
(*_10) = -_13;
_13 = (*_10);
_14 = [6_usize,0_usize,15111708867478562459_usize,6_usize,3_usize,6_usize];
_15 = '\u{fb08f}';
_14 = [17256705751814867366_usize,7_usize,3981081597034761310_usize,2_usize,1_usize,7_usize];
(*_10) = _13;
_15 = '\u{f5749}';
_17 = _12 as f32;
_15 = '\u{2eed0}';
_11 = 56969_u16 << _12;
_17 = 150_u8 as f32;
match _12 {
0 => bb8,
1 => bb10,
95609310050291931658770105040626159561 => bb15,
_ => bb14
}
}
bb14 = {
_1 = 183515100667108076911540962602721639912_u128 as f64;
_1 = (-9223372036854775808_isize) as f64;
_1 = 3_i8 as f64;
RET = ['\u{ca3e1}','\u{c7a2}','\u{cddc1}','\u{4b48}','\u{fe33b}','\u{1f6b1}','\u{eb96c}'];
RET = ['\u{dbd4b}','\u{d3ebf}','\u{779a1}','\u{8fecb}','\u{603b7}','\u{86cb9}','\u{cefc0}'];
_1 = (-49559175704443345738775416122923883714_i128) as f64;
_1 = 10172416121314455297_usize as f64;
RET = ['\u{c156a}','\u{a0a77}','\u{9d0b6}','\u{27491}','\u{ce77a}','\u{f586d}','\u{dd626}'];
_1 = 1212211747_i32 as f64;
Goto(bb2)
}
bb15 = {
_12 = _4 as i128;
RET = [_15,_15,_15,_15,_15,_15,_15];
_17 = 10121130540269694769_usize as f32;
_4 = -(-505427081_i32);
_13 = -(*_10);
_14 = [14017143957946330413_usize,4_usize,7509769226118024954_usize,3951200816446405696_usize,14806164511576650471_usize,18228942411204679071_usize];
RET = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_13 = _1 + _1;
_11 = 58733_u16 | 6363_u16;
(*_10) = 12015701089025115669_usize as f64;
_20 = [190_u8,241_u8,232_u8,2_u8];
RET = [_15,_15,_15,_15,_15,_15,_15];
_1 = _13 - _13;
_11 = 2_usize as u16;
RET = [_15,_15,_15,_15,_15,_15,_15];
_4 = !1715432420_i32;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(11_usize, 14_usize, Move(_14), 15_usize, Move(_15), 8_usize, Move(_8), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: &'static [char; 7]) -> *const u32 {
mir! {
type RET = *const u32;
let _2: bool;
let _3: [u128; 5];
let _4: *const Adt36;
let _5: [i64; 8];
let _6: f64;
let _7: *mut usize;
let _8: *const f64;
let _9: usize;
let _10: isize;
let _11: (u8,);
let _12: i64;
let _13: &'static &'static [i128; 7];
let _14: f64;
let _15: f32;
let _16: f32;
let _17: (bool, i64);
let _18: *const (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _19: [u16; 6];
let _20: [i8; 1];
let _21: i16;
let _22: Adt60;
let _23: isize;
let _24: bool;
let _25: bool;
let _26: ([i8; 1], [char; 7], *const [u128; 3]);
let _27: *mut isize;
let _28: ();
let _29: ();
{
_2 = true;
_2 = 1047039291_u32 >= 576250236_u32;
_3 = [18623713195840709164199636163767947722_u128,241308972413116828943202177228335742136_u128,207135001110777930883420928370196223624_u128,250684680846387367443411465337559628463_u128,191638377427019743167220946530057167403_u128];
_2 = true;
_3 = [332115468644604628886198068394406029759_u128,294684297417998625825077011068418505802_u128,199317375636186764398609854831688417620_u128,99865282716930143585946513422858026577_u128,43818866748543074978863637132701580117_u128];
_2 = false ^ false;
_2 = !true;
_5 = [(-1927508822027175223_i64),(-5810670286920201036_i64),(-7481640695650833020_i64),4163049469072344489_i64,7288444208318974606_i64,4367210668891688739_i64,4082773802666015349_i64,9006938099674371436_i64];
_6 = 58588_u16 as f64;
_8 = core::ptr::addr_of!(_6);
_2 = false;
_2 = true & true;
_3 = [249238984860714635832825520274358565211_u128,187418275214548926996942978915172335220_u128,16490332067667491582092706432532758673_u128,90542284912136936299607458579906502418_u128,113264967893378255657918281109616496906_u128];
_5 = [(-4754739184047183280_i64),(-4033704953707435935_i64),634178042468201488_i64,3747751262992625778_i64,8665807540137465265_i64,(-2180559237104226465_i64),(-5880248023490667423_i64),(-2203659565767150444_i64)];
_2 = (*_8) > _6;
_8 = core::ptr::addr_of!((*_8));
_5 = [6206354540572715882_i64,(-6950287918066008613_i64),(-2563814765287271749_i64),9097937160298160659_i64,7257810853921882590_i64,(-6875544509191165168_i64),6419060722566017236_i64,(-7092484483059382550_i64)];
(*_8) = 6341938996569587039_i64 as f64;
_5 = [(-7717214798366898906_i64),6202614056245277662_i64,(-6358805055287933249_i64),(-7836246323170892510_i64),(-8029958376848864253_i64),(-5305345985048965457_i64),(-8401601457219914493_i64),(-5096865470787021251_i64)];
Goto(bb1)
}
bb1 = {
_6 = 9223372036854775807_isize as f64;
_7 = core::ptr::addr_of_mut!(_9);
_6 = 55_i8 as f64;
(*_7) = !7263196157613837534_usize;
_3 = [175042574294154113246678655689569064660_u128,171708878012190907867632255219368229366_u128,87502861406684791512710239083630668962_u128,83091182316838272582860133528599872461_u128,151032142758555797215822513737142457561_u128];
(*_8) = 14011021718775837633_u64 as f64;
(*_7) = (-156341039102180774192665355635327989935_i128) as usize;
_5 = [(-6857916811779963173_i64),3028035940691879131_i64,5278040464634364856_i64,3068644194894995488_i64,5586842504622190183_i64,7832175850313310411_i64,(-5384811267505964068_i64),1257579281769954310_i64];
_5 = [(-1792998593965411229_i64),(-6093051664488140776_i64),724925168619103598_i64,(-8989941723911598044_i64),(-6636301035938550412_i64),(-4769822701790431001_i64),8708323343250076929_i64,5310877573655016698_i64];
(*_7) = !0_usize;
(*_7) = 1180259897159727986_usize ^ 12670184578414450802_usize;
_7 = core::ptr::addr_of_mut!((*_7));
Goto(bb2)
}
bb2 = {
_8 = core::ptr::addr_of!(_6);
_7 = core::ptr::addr_of_mut!((*_7));
_9 = 13848774655464347408_usize >> 36198_u16;
_3 = [176732076012595480461140711210459349705_u128,217292837226909546794039513478597634069_u128,230380469721233527021447273810502664883_u128,194099225404864708002309635556207522552_u128,140140503601125213654325231242152238398_u128];
(*_7) = !17183589448765166276_usize;
_6 = (-4265002051494440689_i64) as f64;
_11 = (207_u8,);
match _11.0 {
207 => bb3,
_ => bb1
}
}
bb3 = {
_7 = core::ptr::addr_of_mut!((*_7));
_10 = -(-9223372036854775808_isize);
(*_7) = 14786379192363055857_usize - 2922558502564495783_usize;
_11 = (230_u8,);
_11.0 = 3536636293_u32 as u8;
_11 = (80_u8,);
_12 = 7721689091873959469_i64 >> (*_7);
_3 = [300330736942383447792029711525602752313_u128,213294624874052445354022951992586633891_u128,151366290384048025341516765593392451249_u128,41714208995089626027067900898121221264_u128,20118594492552514065140089310431279629_u128];
_2 = true ^ false;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_11.0 = 90_u8 & 106_u8;
_8 = core::ptr::addr_of!(_14);
(*_7) = 17089429918435495883_usize;
(*_7) = 12997650711018751715_usize;
(*_8) = -_6;
Call(_9 = core::intrinsics::transmute(_12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = _12 != _12;
_17.0 = _2 ^ _2;
_11 = (31_u8,);
_10 = !9223372036854775807_isize;
(*_7) = _11.0 as usize;
Goto(bb5)
}
bb5 = {
_11.0 = 28_u8 | 114_u8;
(*_7) = _10 as usize;
_6 = (*_8);
(*_7) = !10491515478363972179_usize;
(*_8) = -_6;
Goto(bb6)
}
bb6 = {
(*_7) = 7_usize;
_11 = (128_u8,);
_9 = 14309874473039463886_usize * 1_usize;
match _11.0 {
0 => bb1,
128 => bb8,
_ => bb7
}
}
bb7 = {
_2 = _12 != _12;
_17.0 = _2 ^ _2;
_11 = (31_u8,);
_10 = !9223372036854775807_isize;
(*_7) = _11.0 as usize;
Goto(bb5)
}
bb8 = {
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
128 => bb9,
_ => bb6
}
}
bb9 = {
_2 = _17.0 & _17.0;
Call(_18 = fn13(_5, _17.0, _2, _2, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19 = [33649_u16,32949_u16,3156_u16,62894_u16,7936_u16,32382_u16];
_17 = (_2, _12);
_15 = (-5857_i16) as f32;
_11 = (220_u8,);
_12 = (*_8) as i64;
_15 = 3174415057_u32 as f32;
_14 = _6;
_5 = [_17.1,_17.1,_17.1,_17.1,_12,_17.1,_17.1,_17.1];
(*_8) = _17.1 as f64;
_17 = (_2, _12);
(*_7) = 0_usize >> _12;
_11.0 = !55_u8;
_7 = core::ptr::addr_of_mut!((*_7));
_5 = [_17.1,_17.1,_17.1,_17.1,_17.1,_12,_12,_12];
_15 = _9 as f32;
_3 = [77399654409370518470045692560139321316_u128,281698057572864265064640433801656640986_u128,108827520030505982257076396343616718307_u128,68487160999811875219318512587019722688_u128,334619320997261565747846710156244005723_u128];
_6 = -_14;
_19 = [18309_u16,20082_u16,16876_u16,22178_u16,19007_u16,24704_u16];
_16 = 268368817_i32 as f32;
_5 = [_17.1,_12,_17.1,_17.1,_17.1,_17.1,_12,_17.1];
(*_8) = _6 - _6;
(*_7) = _17.1 as usize;
_11.0 = '\u{d40e0}' as u8;
_12 = _17.1;
_20 = [103_i8];
_5 = [_17.1,_17.1,_17.1,_12,_17.1,_17.1,_12,_17.1];
_6 = (*_8);
_14 = _6;
_11.0 = 192_u8 + 54_u8;
_3 = [57386327224829151731535154150323641163_u128,11073701418469991211016315758885606353_u128,234145443135936026494926460771814589559_u128,83743268974291147352022187266245340631_u128,232299079413143073464235586618306065392_u128];
Goto(bb11)
}
bb11 = {
_11 = (75_u8,);
(*_7) = _17.0 as usize;
_3 = [173586245959069550190140239937712622127_u128,239557708006390158973160499750981532233_u128,180966837520583864294420921501587244149_u128,300409764459063520464073933767827085498_u128,125499419722253722942906847770662415641_u128];
_15 = -_16;
_11.0 = !216_u8;
_12 = _11.0 as i64;
_11 = (92_u8,);
_17 = (_2, _12);
_12 = _17.1 | _17.1;
_10 = (-93_isize);
_3 = [278336045219660493681024582721434245518_u128,292376132809758157999189730611238332349_u128,136297161133752356087673813680046755595_u128,258299967813961768963206832127444851407_u128,284989372943232389225595314142897678296_u128];
_22.fld1.1.0 = !_2;
_15 = 64900_u16 as f32;
_22.fld6 = _9;
_22.fld5 = (_11.0,);
_22.fld5.0 = !_11.0;
_24 = (*_7) >= (*_7);
_21 = !10182_i16;
_22.fld1.3 = _22.fld5.0 / _11.0;
_7 = core::ptr::addr_of_mut!((*_7));
RET = core::ptr::addr_of!(_22.fld3);
_22.fld1.1 = _17;
(*_8) = _6 + _6;
_22.fld1.2 = [74565967439289576359682348012924566035_i128,105283712798555938862180095439054633555_i128,27038839369830765618830879084211585891_i128,(-72125644943612861795868075214255597905_i128),118500063467469300379925158870566747312_i128,(-154131540324648590193914396506030832420_i128),(-166220740697527363934961678067519951939_i128)];
_22.fld2 = 68840388448084838727055579374423759363_i128 as isize;
_2 = _24;
_21 = 2526_i16 | (-1689_i16);
match _11.0 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb6,
4 => bb5,
5 => bb12,
6 => bb13,
92 => bb15,
_ => bb14
}
}
bb12 = {
_19 = [33649_u16,32949_u16,3156_u16,62894_u16,7936_u16,32382_u16];
_17 = (_2, _12);
_15 = (-5857_i16) as f32;
_11 = (220_u8,);
_12 = (*_8) as i64;
_15 = 3174415057_u32 as f32;
_14 = _6;
_5 = [_17.1,_17.1,_17.1,_17.1,_12,_17.1,_17.1,_17.1];
(*_8) = _17.1 as f64;
_17 = (_2, _12);
(*_7) = 0_usize >> _12;
_11.0 = !55_u8;
_7 = core::ptr::addr_of_mut!((*_7));
_5 = [_17.1,_17.1,_17.1,_17.1,_17.1,_12,_12,_12];
_15 = _9 as f32;
_3 = [77399654409370518470045692560139321316_u128,281698057572864265064640433801656640986_u128,108827520030505982257076396343616718307_u128,68487160999811875219318512587019722688_u128,334619320997261565747846710156244005723_u128];
_6 = -_14;
_19 = [18309_u16,20082_u16,16876_u16,22178_u16,19007_u16,24704_u16];
_16 = 268368817_i32 as f32;
_5 = [_17.1,_12,_17.1,_17.1,_17.1,_17.1,_12,_17.1];
(*_8) = _6 - _6;
(*_7) = _17.1 as usize;
_11.0 = '\u{d40e0}' as u8;
_12 = _17.1;
_20 = [103_i8];
_5 = [_17.1,_17.1,_17.1,_12,_17.1,_17.1,_12,_17.1];
_6 = (*_8);
_14 = _6;
_11.0 = 192_u8 + 54_u8;
_3 = [57386327224829151731535154150323641163_u128,11073701418469991211016315758885606353_u128,234145443135936026494926460771814589559_u128,83743268974291147352022187266245340631_u128,232299079413143073464235586618306065392_u128];
Goto(bb11)
}
bb13 = {
_8 = core::ptr::addr_of!(_6);
_7 = core::ptr::addr_of_mut!((*_7));
_9 = 13848774655464347408_usize >> 36198_u16;
_3 = [176732076012595480461140711210459349705_u128,217292837226909546794039513478597634069_u128,230380469721233527021447273810502664883_u128,194099225404864708002309635556207522552_u128,140140503601125213654325231242152238398_u128];
(*_7) = !17183589448765166276_usize;
_6 = (-4265002051494440689_i64) as f64;
_11 = (207_u8,);
match _11.0 {
207 => bb3,
_ => bb1
}
}
bb14 = {
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
128 => bb9,
_ => bb6
}
}
bb15 = {
_22.fld3 = 8099_u16 as u32;
_9 = !_22.fld6;
_17 = (_24, _12);
_2 = _24;
_6 = (*_8) - _14;
_22.fld4 = _15 as i128;
_22.fld2 = _10 << (*_7);
_20 = [36_i8];
_20 = [(-49_i8)];
_22.fld1.3 = _22.fld5.0;
(*_8) = _6 + _6;
(*RET) = 565600707_u32;
_22.fld1.1.0 = _17.0;
_24 = _22.fld1.1.0;
_22.fld1.3 = !_22.fld5.0;
_2 = !_24;
_17.0 = !_24;
_25 = !_22.fld1.1.0;
_21 = (-32381_i16) * 1480_i16;
_22.fld1.1.1 = _17.1 + _17.1;
_2 = _22.fld1.1.0;
_11 = (_22.fld5.0,);
_6 = (*_8) * _14;
_2 = _24 & _17.0;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(12_usize, 17_usize, Move(_17), 2_usize, Move(_2), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(12_usize, 3_usize, Move(_3), 19_usize, Move(_19), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i64; 8],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: f64) -> *const (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8)) {
mir! {
type RET = *const (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _6: f64;
let _7: (*mut usize, u16, *const f64, i16);
let _8: char;
let _9: Adt58;
let _10: f64;
let _11: f32;
let _12: isize;
let _13: i8;
let _14: u64;
let _15: u16;
let _16: *const *const u32;
let _17: i32;
let _18: f32;
let _19: f64;
let _20: Adt58;
let _21: (*mut usize, u16, *const f64, i16);
let _22: isize;
let _23: i128;
let _24: [usize; 6];
let _25: isize;
let _26: u16;
let _27: &'static f32;
let _28: u16;
let _29: i64;
let _30: *mut (u8,);
let _31: (*mut usize, u16, *const f64, i16);
let _32: ([i8; 1], [char; 7], *const [u128; 3]);
let _33: ([i8; 1], [char; 7], *const [u128; 3]);
let _34: [u128; 3];
let _35: (char, &'static [char; 7], *mut isize);
let _36: (*mut usize, u16, *const f64, i16);
let _37: i8;
let _38: u64;
let _39: *const f64;
let _40: Adt52;
let _41: f32;
let _42: ([i64; 8], u64);
let _43: ([i64; 8], u64);
let _44: i32;
let _45: u16;
let _46: [u8; 6];
let _47: Adt41;
let _48: (Adt18, (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8)), bool, ([i64; 8], u64));
let _49: &'static *mut *mut isize;
let _50: u16;
let _51: *mut [i8; 1];
let _52: (char, &'static [char; 7], *mut isize);
let _53: &'static Adt36;
let _54: *mut isize;
let _55: ();
let _56: ();
{
_1 = [(-4320897062098684615_i64),(-2179482078128272792_i64),4236861044725193281_i64,3540941031360601260_i64,(-704936095396066240_i64),3606428836056239688_i64,1827676753708401521_i64,(-2765344098110968824_i64)];
_1 = [853546870126800890_i64,(-5473857200067282610_i64),(-8429991425871632131_i64),4907173897697887061_i64,(-4981978234970296515_i64),9109876735987798421_i64,706865029648884205_i64,9178882154127638304_i64];
_1 = [(-1507592683972844203_i64),8240812835976886051_i64,(-6027536498246799461_i64),(-1169631267591873470_i64),(-5588502089189081647_i64),(-4749746620587774981_i64),(-2134991855588791406_i64),(-2583355041606811930_i64)];
_5 = 191_u8 as f64;
_1 = [8458013917625841999_i64,(-4046735585346866655_i64),(-8290012670216318922_i64),3098965659578416901_i64,(-1142123931540310819_i64),(-7493459713913225522_i64),(-2486766382062026850_i64),(-2774460615671018327_i64)];
_5 = 11338821262137356678_u64 as f64;
_6 = _5;
_7.1 = (-9223372036854775808_isize) as u16;
_3 = _4;
_8 = '\u{8afd0}';
_8 = '\u{57288}';
_7.3 = 283378175805091167328011634745347650946_u128 as i16;
_1 = [6056532923820885549_i64,6524051517627827721_i64,6667899321184415826_i64,3076010206660296412_i64,(-8153160259810295922_i64),(-5634204648261152712_i64),3574478611286740262_i64,(-4883839988509494435_i64)];
_7.2 = core::ptr::addr_of!(_5);
_1 = [2800615473024195915_i64,(-7576700188029324053_i64),(-8306509220034318082_i64),6663352066513202836_i64,6094619670967388858_i64,(-7651522812519826749_i64),2328288774160670896_i64,(-7638601039356114064_i64)];
_7.1 = !17535_u16;
_6 = _5 + _5;
_6 = _5 + _5;
_1 = [(-2974731950197075172_i64),1974193196029110379_i64,(-458726196121493365_i64),8060219584226692743_i64,(-2672830111435898829_i64),(-8557711292645347595_i64),9185481194615701079_i64,(-2532110221224865023_i64)];
_10 = 11341900055179573773_usize as f64;
_8 = '\u{a2f31}';
_5 = _10;
_3 = _4;
_7.2 = core::ptr::addr_of!(_10);
_7.3 = 114_u8 as i16;
Goto(bb1)
}
bb1 = {
_5 = (-36836676_i32) as f64;
_11 = (-7015732024543098127_i64) as f32;
_6 = _5 * _5;
_11 = 3109621115201097708_u64 as f32;
_10 = _6 + _6;
_4 = _2;
_5 = 64_u8 as f64;
_7.2 = core::ptr::addr_of!(_5);
_2 = !_3;
_2 = _3;
_15 = _7.1;
_12 = -(-9223372036854775808_isize);
_13 = (-52_i8);
_3 = _2 > _4;
_3 = !_2;
_7.3 = 29997_i16;
_10 = 3601535558_u32 as f64;
_14 = 1154817912622273728_u64 - 11410057318244628049_u64;
Goto(bb2)
}
bb2 = {
_8 = '\u{b3956}';
_15 = _7.1;
_9 = Adt58::Variant1 { fld0: _12,fld1: 5449253517613775_usize };
_9 = Adt58::Variant1 { fld0: _12,fld1: 9897693933319434079_usize };
_7.2 = core::ptr::addr_of!(_5);
_12 = -Field::<isize>(Variant(_9, 1), 0);
_2 = _3;
_13 = (-115_i8) & 126_i8;
place!(Field::<usize>(Variant(_9, 1), 1)) = _11 as usize;
_5 = -_6;
_7.3 = (-20539_i16) * 27481_i16;
Goto(bb3)
}
bb3 = {
_7.3 = -21639_i16;
_7.1 = _11 as u16;
_15 = _7.1;
_8 = '\u{86122}';
_17 = -556496220_i32;
_3 = !_4;
_2 = _4;
_14 = 70_u8 as u64;
_8 = '\u{a8dd7}';
SetDiscriminant(_9, 0);
_2 = !_3;
_3 = !_2;
place!(Field::<(i128, isize)>(Variant(_9, 0), 1)) = (113739256482362659820259476642268942773_i128, _12);
_13 = _14 as i8;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld1 = !17154894001671141693_usize;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld2 = -_17;
_10 = -_6;
_19 = _5 * _6;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3 = Adt18::Variant0 { fld0: _3,fld1: Field::<(i128, isize)>(Variant(_9, 0), 1).0,fld2: _14,fld3: Field::<Adt52>(Variant(_9, 0), 0).fld1,fld4: _7.1 };
place!(Field::<u16>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 4)) = 3064077749969055829_i64 as u16;
_18 = _11 * _11;
_11 = _18;
_20 = Adt58::Variant1 { fld0: _12,fld1: Field::<usize>(Variant(Field::<Adt52>(Variant(_9, 0), 0).fld3, 0), 3) };
_21.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_20, 1), 1)));
_7.0 = Move(_21.0);
_21 = (Move(_7.0), _7.1, Move(_7.2), _7.3);
Call(_8 = fn14(_3, Move(Field::<Adt52>(Variant(_9, 0), 0).fld3), Field::<isize>(Variant(_20, 1), 0)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = _3 as u64;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3 = Adt18::Variant0 { fld0: _3,fld1: Field::<(i128, isize)>(Variant(_9, 0), 1).0,fld2: _14,fld3: Field::<usize>(Variant(_20, 1), 1),fld4: _7.1 };
place!(Field::<isize>(Variant(_20, 1), 0)) = !Field::<(i128, isize)>(Variant(_9, 0), 1).1;
match Field::<i128>(Variant(Field::<Adt52>(Variant(_9, 0), 0).fld3, 0), 1) {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
113739256482362659820259476642268942773 => bb9,
_ => bb8
}
}
bb5 = {
_7.3 = -21639_i16;
_7.1 = _11 as u16;
_15 = _7.1;
_8 = '\u{86122}';
_17 = -556496220_i32;
_3 = !_4;
_2 = _4;
_14 = 70_u8 as u64;
_8 = '\u{a8dd7}';
SetDiscriminant(_9, 0);
_2 = !_3;
_3 = !_2;
place!(Field::<(i128, isize)>(Variant(_9, 0), 1)) = (113739256482362659820259476642268942773_i128, _12);
_13 = _14 as i8;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld1 = !17154894001671141693_usize;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld2 = -_17;
_10 = -_6;
_19 = _5 * _6;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3 = Adt18::Variant0 { fld0: _3,fld1: Field::<(i128, isize)>(Variant(_9, 0), 1).0,fld2: _14,fld3: Field::<Adt52>(Variant(_9, 0), 0).fld1,fld4: _7.1 };
place!(Field::<u16>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 4)) = 3064077749969055829_i64 as u16;
_18 = _11 * _11;
_11 = _18;
_20 = Adt58::Variant1 { fld0: _12,fld1: Field::<usize>(Variant(Field::<Adt52>(Variant(_9, 0), 0).fld3, 0), 3) };
_21.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_20, 1), 1)));
_7.0 = Move(_21.0);
_21 = (Move(_7.0), _7.1, Move(_7.2), _7.3);
Call(_8 = fn14(_3, Move(Field::<Adt52>(Variant(_9, 0), 0).fld3), Field::<isize>(Variant(_20, 1), 0)), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_8 = '\u{b3956}';
_15 = _7.1;
_9 = Adt58::Variant1 { fld0: _12,fld1: 5449253517613775_usize };
_9 = Adt58::Variant1 { fld0: _12,fld1: 9897693933319434079_usize };
_7.2 = core::ptr::addr_of!(_5);
_12 = -Field::<isize>(Variant(_9, 1), 0);
_2 = _3;
_13 = (-115_i8) & 126_i8;
place!(Field::<usize>(Variant(_9, 1), 1)) = _11 as usize;
_5 = -_6;
_7.3 = (-20539_i16) * 27481_i16;
Goto(bb3)
}
bb7 = {
_5 = (-36836676_i32) as f64;
_11 = (-7015732024543098127_i64) as f32;
_6 = _5 * _5;
_11 = 3109621115201097708_u64 as f32;
_10 = _6 + _6;
_4 = _2;
_5 = 64_u8 as f64;
_7.2 = core::ptr::addr_of!(_5);
_2 = !_3;
_2 = _3;
_15 = _7.1;
_12 = -(-9223372036854775808_isize);
_13 = (-52_i8);
_3 = _2 > _4;
_3 = !_2;
_7.3 = 29997_i16;
_10 = 3601535558_u32 as f64;
_14 = 1154817912622273728_u64 - 11410057318244628049_u64;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_7 = (Move(_21.0), _21.1, Move(_21.2), _21.3);
place!(Field::<(i128, isize)>(Variant(_9, 0), 1)).0 = (-3338300460817124208_i64) as i128;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3 = Adt18::Variant1 { fld0: _13,fld1: _7.1,fld2: 1102304171905530990_i64 };
place!(Field::<usize>(Variant(_20, 1), 1)) = Field::<Adt52>(Variant(_9, 0), 0).fld1;
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld1 = (-1271077361667997468_i64) as usize;
Goto(bb10)
}
bb10 = {
_21.2 = core::ptr::addr_of!(_5);
place!(Field::<i8>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 1), 0)) = _4 as i8;
_18 = _21.3 as f32;
_7.3 = _11 as i16;
_9 = Adt58::Variant1 { fld0: _12,fld1: Field::<usize>(Variant(_20, 1), 1) };
_22 = _17 as isize;
_2 = _4;
_21 = (Move(_7.0), _7.1, Move(_7.2), _7.3);
_7.1 = !_15;
_27 = &_18;
_28 = _13 as u16;
_24 = [Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_20, 1), 1),Field::<usize>(Variant(_20, 1), 1),Field::<usize>(Variant(_20, 1), 1)];
_1 = [6842626690549625422_i64,(-1361487210415708321_i64),5257519171920077189_i64,1209453131659756172_i64,282720488947457475_i64,(-2886864106166044899_i64),(-2493576785096772985_i64),(-8565807511620031265_i64)];
_21.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_9, 1), 1)));
_24 = [Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_20, 1), 1),Field::<usize>(Variant(_20, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1)];
_21.1 = _15 | _15;
_21.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_20, 1), 1)));
_25 = -Field::<isize>(Variant(_20, 1), 0);
_24 = [Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_20, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_9, 1), 1),Field::<usize>(Variant(_20, 1), 1)];
_7.0 = Move(_21.0);
_29 = (-4719605536625154998_i64);
_14 = 14607798801575867826_u64 - 12759357872888444715_u64;
_23 = Field::<usize>(Variant(_20, 1), 1) as i128;
_26 = !_21.1;
_13 = _8 as i8;
Goto(bb11)
}
bb11 = {
_9 = Adt58::Variant1 { fld0: _12,fld1: Field::<usize>(Variant(_20, 1), 1) };
_19 = _5;
_21.2 = core::ptr::addr_of!(_5);
_21.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_20, 1), 1)));
SetDiscriminant(_20, 1);
_15 = _6 as u16;
_21.0 = Move(_7.0);
_19 = _14 as f64;
_22 = _21.1 as isize;
_6 = _19 + _5;
_18 = -_11;
_29 = _25 as i64;
_33.1 = [_8,_8,_8,_8,_8,_8,_8];
place!(Field::<isize>(Variant(_9, 1), 0)) = _22 + _22;
_22 = Field::<isize>(Variant(_9, 1), 0) & _25;
_22 = _6 as isize;
place!(Field::<isize>(Variant(_20, 1), 0)) = _22 << _15;
SetDiscriminant(_9, 0);
_31.1 = _26;
Goto(bb12)
}
bb12 = {
_22 = Field::<isize>(Variant(_20, 1), 0);
place!(Field::<(i128, isize)>(Variant(_9, 0), 1)).1 = _15 as isize;
_37 = _13 + _13;
_31.3 = 4121142548_u32 as i16;
_35.0 = _8;
_33.0 = [_37];
_31.3 = _21.3 * _7.3;
_19 = _22 as f64;
Goto(bb13)
}
bb13 = {
_37 = _13;
_1 = [_29,_29,_29,_29,_29,_29,_29,_29];
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3 = Adt18::Variant0 { fld0: _3,fld1: _23,fld2: _14,fld3: 3_usize,fld4: _21.1 };
_35.0 = _8;
_17 = _4 as i32;
_7.2 = core::ptr::addr_of!(_19);
_31.2 = Move(_7.2);
_22 = !Field::<(i128, isize)>(Variant(_9, 0), 1).1;
_6 = -_19;
_43 = (_1, Field::<u64>(Variant(Field::<Adt52>(Variant(_9, 0), 0).fld3, 0), 2));
_6 = 65754410335519957514628067495162753838_u128 as f64;
_43 = (_1, Field::<u64>(Variant(Field::<Adt52>(Variant(_9, 0), 0).fld3, 0), 2));
Goto(bb14)
}
bb14 = {
place!(Field::<Adt52>(Variant(_9, 0), 0)).fld1 = 3_usize;
place!(Field::<u16>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 4)) = _28;
_31.1 = 142675556241340654695983129760832354625_u128 as u16;
_7.2 = Move(_31.2);
_36.2 = core::ptr::addr_of!(_10);
_36 = (Move(_21.0), _15, Move(_7.2), _7.3);
_43 = (_1, _14);
_28 = !_26;
_31.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_20, 1), 1)));
_31.1 = _21.1 & _26;
_12 = _22 << _43.1;
_42 = (_1, _14);
_7 = (Move(_31.0), _36.1, Move(_36.2), _31.3);
_40.fld3 = Adt18::Variant1 { fld0: _37,fld1: _36.1,fld2: _29 };
Goto(bb15)
}
bb15 = {
place!(Field::<i128>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 1)) = _23;
_25 = Field::<isize>(Variant(_20, 1), 0);
_14 = _42.1;
SetDiscriminant(_40.fld3, 1);
_26 = _31.1;
place!(Field::<i8>(Variant(_40.fld3, 1), 0)) = _23 as i8;
_21.3 = _31.3 >> _25;
place!(Field::<u64>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 2)) = !_42.1;
_21.2 = core::ptr::addr_of!(_10);
_36.1 = _15 + _26;
_29 = (-8338640840207956991_i64) + 144138432713729344_i64;
_48.0 = Adt18::Variant1 { fld0: _13,fld1: _31.1,fld2: _29 };
place!(Field::<(i128, isize)>(Variant(_9, 0), 1)) = (_23, _25);
_21.0 = core::ptr::addr_of_mut!(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld1);
match Field::<Adt52>(Variant(_9, 0), 0).fld1 {
0 => bb1,
1 => bb9,
2 => bb8,
3 => bb16,
_ => bb13
}
}
bb16 = {
_21.3 = _7.3 | _31.3;
_38 = _42.1 ^ _42.1;
_48.1.2 = Adt18::Variant0 { fld0: _3,fld1: Field::<(i128, isize)>(Variant(_9, 0), 1).0,fld2: _43.1,fld3: Field::<Adt52>(Variant(_9, 0), 0).fld1,fld4: _36.1 };
_48.1.1 = _31.3 + _21.3;
_13 = _37;
_35.1 = &_33.1;
_18 = -_11;
_16 = core::ptr::addr_of!(_40.fld0);
place!(Field::<bool>(Variant(place!(Field::<Adt52>(Variant(_9, 0), 0)).fld3, 0), 0)) = _2;
_48.1.3.1 = (_4, _29);
_40.fld2 = Field::<isize>(Variant(_20, 1), 0) as i32;
_7.1 = !_28;
_15 = Field::<u16>(Variant(_48.1.2, 0), 4);
_46 = [183_u8,166_u8,222_u8,204_u8,104_u8,164_u8];
_48.1.3.3 = 27_u8 ^ 51_u8;
SetDiscriminant(_48.0, 1);
RET = core::ptr::addr_of!(_48.1);
_31.2 = Move(_7.2);
(*RET).3.0 = _38 as u16;
_2 = (*RET).1 > (*RET).1;
_36.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant((*RET).2, 0), 3)));
_42.1 = Field::<u64>(Variant(_48.1.2, 0), 2) + _38;
place!(Field::<usize>(Variant((*RET).2, 0), 3)) = 201968650_u32 as usize;
Goto(bb17)
}
bb17 = {
Call(_55 = dump_var(13_usize, 26_usize, Move(_26), 37_usize, Move(_37), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_55 = dump_var(13_usize, 2_usize, Move(_2), 29_usize, Move(_29), 28_usize, Move(_28), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_55 = dump_var(13_usize, 43_usize, Move(_43), 12_usize, Move(_12), 38_usize, Move(_38), 56_usize, _56), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: Adt18,mut _3: isize) -> char {
mir! {
type RET = char;
let _4: Adt30;
let _5: f32;
let _6: *const [u128; 3];
let _7: [usize; 4];
let _8: i32;
let _9: *const [u128; 3];
let _10: isize;
let _11: [i128; 7];
let _12: f64;
let _13: &'static &'static &'static [i128; 7];
let _14: i16;
let _15: &'static Adt36;
let _16: *mut usize;
let _17: isize;
let _18: isize;
let _19: f64;
let _20: ();
let _21: ();
{
place!(Field::<bool>(Variant(_2, 0), 0)) = _1;
SetDiscriminant(_2, 2);
_3 = (-81620871112632634625673330192596981956_i128) as isize;
_2 = Adt18::Variant1 { fld0: (-35_i8),fld1: 12399_u16,fld2: 1194122856741832127_i64 };
_2 = Adt18::Variant2 { fld0: '\u{f8690}' };
RET = '\u{56aa5}';
Goto(bb1)
}
bb1 = {
_3 = 30252_u16 as isize;
_1 = !false;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_1 = false;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
RET = Field::<char>(Variant(_2, 2), 0);
_1 = _3 <= _3;
_3 = (-64_isize);
_5 = 0_usize as f32;
_4 = Adt30 { fld0: 8215_u16 };
RET = Field::<char>(Variant(_2, 2), 0);
_3 = (-9223372036854775808_isize);
_5 = 14308531347433251630_u64 as f32;
_5 = 49915866533404084870327841233911431013_i128 as f32;
_1 = true ^ true;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_2 = Adt18::Variant1 { fld0: 8_i8,fld1: _4.fld0,fld2: (-6647452790835435366_i64) };
_2 = Adt18::Variant0 { fld0: _1,fld1: 169645886735138074072120672754934177096_i128,fld2: 17456741278611138054_u64,fld3: 4043894479987200307_usize,fld4: _4.fld0 };
place!(Field::<bool>(Variant(_2, 0), 0)) = _1 != _1;
place!(Field::<i128>(Variant(_2, 0), 1)) = 111127781088521512441286203750547852302_i128 ^ (-90672024265569252892111507623621937966_i128);
_5 = (-31286_i16) as f32;
match Field::<u16>(Variant(_2, 0), 4) {
0 => bb2,
1 => bb3,
2 => bb4,
8215 => bb6,
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
_1 = !Field::<bool>(Variant(_2, 0), 0);
RET = '\u{4a6a9}';
_7 = [11116255863188544584_usize,4_usize,9760909700342298929_usize,13252307602431068556_usize];
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<i128>(Variant(_2, 0), 1)) = -118901342633639418503508855220340232881_i128;
_7 = [18251230324447194167_usize,16433216729979656258_usize,12563863502303830712_usize,11650985505569456298_usize];
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<usize>(Variant(_2, 0), 3)) = 14049501158106697068_usize;
RET = '\u{af851}';
place!(Field::<usize>(Variant(_2, 0), 3)) = Field::<i128>(Variant(_2, 0), 1) as usize;
place!(Field::<u64>(Variant(_2, 0), 2)) = _5 as u64;
_7 = [Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3)];
_3 = (-67_i8) as isize;
place!(Field::<usize>(Variant(_2, 0), 3)) = 3_usize * 12164114561800491678_usize;
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<u16>(Variant(_2, 0), 4)) = _4.fld0;
SetDiscriminant(_2, 2);
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_8 = 3003296514_u32 as i32;
_5 = (-54_i8) as f32;
_8 = 195066902_i32;
match _8 {
195066902 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
SetDiscriminant(_2, 0);
_4 = Adt30 { fld0: 6131_u16 };
RET = '\u{539cb}';
RET = '\u{e92d5}';
place!(Field::<i128>(Variant(_2, 0), 1)) = 84696015541839047917410654300155335583_i128;
_12 = Field::<i128>(Variant(_2, 0), 1) as f64;
_10 = _3 ^ _3;
_3 = 137309749750176431033954427683291300516_u128 as isize;
place!(Field::<usize>(Variant(_2, 0), 3)) = 977828249101839637_usize + 15768457869902849542_usize;
_11 = [Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1)];
_14 = _10 as i16;
place!(Field::<i128>(Variant(_2, 0), 1)) = !30245300131278918694376414289424004235_i128;
Goto(bb9)
}
bb9 = {
_2 = Adt18::Variant2 { fld0: RET };
_1 = !true;
Goto(bb10)
}
bb10 = {
_12 = _14 as f64;
_17 = (-4864473703221211507_i64) as isize;
_7 = [1264617323961511819_usize,3_usize,13556626911922505575_usize,15402683329477375079_usize];
SetDiscriminant(_2, 0);
_3 = _8 as isize;
place!(Field::<u16>(Variant(_2, 0), 4)) = _4.fld0;
place!(Field::<i128>(Variant(_2, 0), 1)) = 128294957737389011499083478113604688443_i128;
RET = '\u{fee06}';
_1 = false;
_8 = (-1383575439_i32);
_11 = [Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1),Field::<i128>(Variant(_2, 0), 1)];
_8 = 10046057258179118479_u64 as i32;
_2 = Adt18::Variant1 { fld0: 114_i8,fld1: _4.fld0,fld2: (-6310451076165916633_i64) };
_3 = -_17;
_2 = Adt18::Variant0 { fld0: _1,fld1: 82402777723697086130985577226494921799_i128,fld2: 86224669749116817_u64,fld3: 8331825569500449351_usize,fld4: _4.fld0 };
_11 = [30152618810829570837005526437802326576_i128,22650501581949159455214878979628958677_i128,(-100740945883202088622810339724897495041_i128),(-12908429326217893684624585067756792491_i128),98057342630967198708702550809367047000_i128,(-49316341731103658122870251884413012455_i128),(-25783285457111991487411290843801419520_i128)];
match Field::<u16>(Variant(_2, 0), 4) {
0 => bb11,
1 => bb12,
2 => bb13,
6131 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
_3 = 30252_u16 as isize;
_1 = !false;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_1 = false;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
RET = Field::<char>(Variant(_2, 2), 0);
_1 = _3 <= _3;
_3 = (-64_isize);
_5 = 0_usize as f32;
_4 = Adt30 { fld0: 8215_u16 };
RET = Field::<char>(Variant(_2, 2), 0);
_3 = (-9223372036854775808_isize);
_5 = 14308531347433251630_u64 as f32;
_5 = 49915866533404084870327841233911431013_i128 as f32;
_1 = true ^ true;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_2 = Adt18::Variant1 { fld0: 8_i8,fld1: _4.fld0,fld2: (-6647452790835435366_i64) };
_2 = Adt18::Variant0 { fld0: _1,fld1: 169645886735138074072120672754934177096_i128,fld2: 17456741278611138054_u64,fld3: 4043894479987200307_usize,fld4: _4.fld0 };
place!(Field::<bool>(Variant(_2, 0), 0)) = _1 != _1;
place!(Field::<i128>(Variant(_2, 0), 1)) = 111127781088521512441286203750547852302_i128 ^ (-90672024265569252892111507623621937966_i128);
_5 = (-31286_i16) as f32;
match Field::<u16>(Variant(_2, 0), 4) {
0 => bb2,
1 => bb3,
2 => bb4,
8215 => bb6,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
_1 = !Field::<bool>(Variant(_2, 0), 0);
RET = '\u{4a6a9}';
_7 = [11116255863188544584_usize,4_usize,9760909700342298929_usize,13252307602431068556_usize];
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<i128>(Variant(_2, 0), 1)) = -118901342633639418503508855220340232881_i128;
_7 = [18251230324447194167_usize,16433216729979656258_usize,12563863502303830712_usize,11650985505569456298_usize];
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<usize>(Variant(_2, 0), 3)) = 14049501158106697068_usize;
RET = '\u{af851}';
place!(Field::<usize>(Variant(_2, 0), 3)) = Field::<i128>(Variant(_2, 0), 1) as usize;
place!(Field::<u64>(Variant(_2, 0), 2)) = _5 as u64;
_7 = [Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3),Field::<usize>(Variant(_2, 0), 3)];
_3 = (-67_i8) as isize;
place!(Field::<usize>(Variant(_2, 0), 3)) = 3_usize * 12164114561800491678_usize;
_4 = Adt30 { fld0: Field::<u16>(Variant(_2, 0), 4) };
place!(Field::<u16>(Variant(_2, 0), 4)) = _4.fld0;
SetDiscriminant(_2, 2);
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
place!(Field::<char>(Variant(_2, 2), 0)) = RET;
_8 = 3003296514_u32 as i32;
_5 = (-54_i8) as f32;
_8 = 195066902_i32;
match _8 {
195066902 => bb8,
_ => bb7
}
}
bb15 = {
_10 = _17 & _3;
_16 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_2, 0), 3)));
place!(Field::<usize>(Variant(_2, 0), 3)) = !1105491530086844050_usize;
place!(Field::<usize>(Variant(_2, 0), 3)) = 2970167531412931506_usize;
(*_16) = 5621008219467319596_usize;
_11 = [59109812725599701808283918436654131405_i128,(-28845260509403234423272753071976731923_i128),95310586874559186749107750963590760510_i128,(-70612700395018619237907584972170654869_i128),76542524934474948032166121297798028978_i128,155649703391259061054493567210841941238_i128,(-51619095323140576175360011466308878042_i128)];
place!(Field::<u64>(Variant(_2, 0), 2)) = 189844884025319554563004613972031964957_u128 as u64;
_14 = -30977_i16;
_10 = _17;
_8 = 1683548229_i32;
_1 = _17 <= _17;
_11 = [(-123406814150240855487916436288669977066_i128),100776984863750491343372014066913818383_i128,(-11888989025047831594969125312258809913_i128),(-5986372149128887767929862354691863051_i128),(-92288583502146361541314885779174255424_i128),(-36817184626147477186389012894333845430_i128),60350119142772701650499213686151705311_i128];
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(14_usize, 10_usize, Move(_10), 7_usize, Move(_7), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: (u16, (bool, i64), [i128; 7], u8),mut _2: isize,mut _3: [char; 7],mut _4: [char; 7]) -> [char; 7] {
mir! {
type RET = [char; 7];
let _5: *mut Adt36;
let _6: isize;
let _7: (char, &'static [char; 7], *mut isize);
let _8: isize;
let _9: &'static *mut *mut isize;
let _10: bool;
let _11: [i8; 1];
let _12: *const (f32, i16, Adt18, (u16, (bool, i64), [i128; 7], u8));
let _13: (*mut usize, u16, *const f64, i16);
let _14: f32;
let _15: &'static u64;
let _16: *mut f64;
let _17: usize;
let _18: usize;
let _19: f64;
let _20: Adt58;
let _21: &'static &'static &'static [i128; 7];
let _22: bool;
let _23: (u16, (bool, i64), [i128; 7], u8);
let _24: &'static [char; 7];
let _25: *mut usize;
let _26: &'static &'static &'static [i128; 7];
let _27: [u128; 3];
let _28: u16;
let _29: bool;
let _30: isize;
let _31: char;
let _32: [i8; 1];
let _33: [u32; 2];
let _34: *mut isize;
let _35: ();
let _36: ();
{
_1.1 = (true, (-7864294409886546272_i64));
_1.2 = [(-110636522797813247155409036979145202203_i128),(-74551679330042330914860089467788158708_i128),141290750409913612872370239087159410826_i128,133519780176650915500914852498231194082_i128,(-116074925091043444393534341657241549588_i128),(-82791470164963223614140021754840485581_i128),(-61733572625851501800249904509279408848_i128)];
_1.0 = 59527_u16;
_1.1.1 = (-3573957764556977502_i64) * 3370448612789311546_i64;
_3 = ['\u{5e8aa}','\u{bb919}','\u{8ad5a}','\u{fcd93}','\u{3c84d}','\u{5ab9a}','\u{98bbb}'];
RET = _3;
_1.3 = !248_u8;
_1.1 = (true, (-3482430956796731284_i64));
_1.0 = !26766_u16;
_1.0 = !15397_u16;
_7.1 = &_4;
_1.1 = (false, (-7986297247680329134_i64));
_3 = ['\u{707ce}','\u{d2e5f}','\u{6cd1f}','\u{cb2b9}','\u{4a912}','\u{91562}','\u{d2b3c}'];
_1.2 = [61619080574141470609855434089645085665_i128,160308748931897773328877398469529281488_i128,9338575436118499475523778212227066444_i128,130330175032056902622951294088001866967_i128,92816984654154746912274953618746578135_i128,113562979674351555823876444853597934970_i128,(-102729967247076124830430499839868523363_i128)];
_1.1.1 = -5320574785216949204_i64;
Goto(bb1)
}
bb1 = {
_8 = (-15_i8) as isize;
_4 = ['\u{cc9e5}','\u{1b1dc}','\u{1040e3}','\u{108e7f}','\u{e40a}','\u{8b5ae}','\u{ac19}'];
_7.0 = '\u{51cc}';
RET = _3;
_2 = !_8;
_1.2 = [(-128826921599261654147114206132045848233_i128),89031871502786192883301127928349193933_i128,(-163652041596774608156218314529377409254_i128),(-155356795023776259516341328386074652590_i128),(-89217192530513993359497832606590751335_i128),15283212428625295903515850033274227281_i128,120302762858326650942108728661160927330_i128];
_7.2 = core::ptr::addr_of_mut!(_2);
_7.1 = &_4;
_1.1.1 = (-5332081442460298576_i64);
_1.3 = 217_u8 & 138_u8;
_7.1 = &_4;
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_11 = [116_i8];
_3 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_1.1.0 = !true;
_2 = _8 & _8;
_3 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_1.0 = 3754_u16 >> _2;
_7.1 = &_4;
RET = _4;
RET = _4;
_1.2 = [48703372448946464691413466010469640361_i128,99995665696656788723877160152422626517_i128,148528695517126887428804152793230750279_i128,137647898145306388523083257694415921936_i128,(-54774691572327297685809907713966330995_i128),(-82495351251036985565743920553596574963_i128),125930724958590111331874654553176507840_i128];
_1.3 = 42_u8 - 45_u8;
_3 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
match _1.1.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463458042525989307912880 => bb8,
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
_7.0 = '\u{2255}';
_1.3 = 69_u8 * 119_u8;
_1.3 = 127_u8 ^ 231_u8;
_10 = _1.0 > _1.0;
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_7.1 = &RET;
_7.0 = '\u{1a2a5}';
Goto(bb9)
}
bb9 = {
_13.1 = (-76624190196619898797760831826130338964_i128) as u16;
_13.3 = 24295_i16 | (-32588_i16);
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_1.3 = 225_u8 << _8;
_10 = _1.1.0 | _1.1.0;
_13.3 = 6067_i16 | (-19446_i16);
_7.0 = '\u{30d23}';
_2 = -_8;
_10 = !_1.1.0;
_11 = [94_i8];
_6 = !_2;
_7.1 = &RET;
_1.1.0 = _13.1 < _1.0;
_7.0 = '\u{34242}';
_1.2 = [82424169210069792742614240307773813743_i128,212227561967443048285501912670150340_i128,(-78428067654812068645876709573965450006_i128),107683852292975313726238053276379503187_i128,88256820668046844911658723306612744269_i128,158100223093355290669448457388227662035_i128,1407656630226451842898553568368446558_i128];
_1.1.1 = (-4304326411407256465_i64) - 1280644022441234272_i64;
_8 = _6;
_14 = (-107_i8) as f32;
_11 = [(-9_i8)];
_7.2 = core::ptr::addr_of_mut!(_8);
_7.1 = &RET;
_1.2 = [131446649585147080558778389794284228553_i128,38596116106394930759792381710798448957_i128,(-169910124183705378077952030650376789212_i128),(-77071911995061378809212941029667329551_i128),136131528116568569480577277760471390856_i128,(-122573106588439512324239076890422888184_i128),51765077246342638044038493899665979988_i128];
Goto(bb10)
}
bb10 = {
_17 = 5023318357673055025_usize;
_10 = _1.1.0;
RET = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_13.3 = 19887_i16;
_13.0 = core::ptr::addr_of_mut!(_17);
_1.0 = !_13.1;
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_3 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_1.1.1 = 1716203636468768087_i64 - 2517255865048927607_i64;
_2 = _17 as isize;
_18 = _17 & _17;
_8 = _17 as isize;
_13.1 = _1.0 >> _1.3;
_19 = _14 as f64;
_1.1 = (_10, (-6764320836726713652_i64));
_1.1.0 = _10;
_7.0 = '\u{c18e5}';
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_16 = core::ptr::addr_of_mut!(_19);
RET = _3;
_10 = _1.1.0;
match _13.3 {
0 => bb4,
19887 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_13.1 = _1.0;
_23.0 = (-845462566_i32) as u16;
_1.1.0 = !_10;
_19 = (-1351164236_i32) as f64;
_13.0 = core::ptr::addr_of_mut!(_17);
_16 = core::ptr::addr_of_mut!((*_16));
Goto(bb13)
}
bb13 = {
_2 = -_6;
_19 = 105438181400424244283283265821855722739_u128 as f64;
(*_16) = 1545618958401259791_u64 as f64;
_19 = 4040435257278893432_u64 as f64;
_23.1 = (_10, _1.1.1);
_23.1 = (_1.1.0, _1.1.1);
_22 = _23.1.0;
_7.1 = &_3;
_13.0 = core::ptr::addr_of_mut!(_17);
_23.1.1 = _1.1.1 + _1.1.1;
_23.3 = (-1561121767_i32) as u8;
_4 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_23.0 = (-103_i8) as u16;
RET = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_14 = _17 as f32;
_20 = Adt58::Variant1 { fld0: _8,fld1: _17 };
(*_16) = (-97_i8) as f64;
_1.2 = [(-36804013319269665669268573382617025111_i128),(-83590110699849773397998885934033243061_i128),67520382843865501764194676034351783824_i128,(-101715146294630960833609034984515971927_i128),(-2209960629638588179493921194734580303_i128),(-101007880609740690112768546502933664929_i128),147021743978405036997583857711311311613_i128];
_1.1.1 = _10 as i64;
_7.2 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_20, 1), 0)));
(*_16) = Field::<isize>(Variant(_20, 1), 0) as f64;
SetDiscriminant(_20, 1);
_4 = _3;
_7.0 = '\u{8a5fc}';
(*_16) = _6 as f64;
Goto(bb14)
}
bb14 = {
_1.3 = _23.3 >> _6;
_19 = _18 as f64;
_7.2 = core::ptr::addr_of_mut!(_6);
(*_16) = (-41_i8) as f64;
_23.2 = [(-167312429344194682720350607385282474423_i128),(-54087047951179737492092807815554072424_i128),94846642765154970695229060482819997414_i128,72000387067974979434882197749799263782_i128,(-68218828743785196255453426858404662143_i128),119546590168524812586790204733368391254_i128,56108512340441576969603766663367661541_i128];
_23.1.1 = -_1.1.1;
_13.3 = _17 as i16;
_28 = _1.0;
_23.1 = (_1.1.0, _1.1.1);
_23.1.0 = _22 > _22;
_8 = _6;
_23.2 = [(-44895981063440546422493042502598480767_i128),168051274516225393478652253233219878770_i128,(-49110388791577524321241895496698455197_i128),2453506228512576480827828913357103154_i128,(-86541509729883431234158527703044972621_i128),(-23373684465714151038548308455704129150_i128),(-3686889158073484796765188827850446396_i128)];
_23 = _1;
_7.1 = &_3;
_13.2 = core::ptr::addr_of!((*_16));
_11 = [(-49_i8)];
_13.0 = core::ptr::addr_of_mut!(_18);
_30 = _8 + _8;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(15_usize, 22_usize, Move(_22), 23_usize, Move(_23), 8_usize, Move(_8), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(15_usize, 2_usize, Move(_2), 4_usize, Move(_4), 18_usize, Move(_18), 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [u8; 6],mut _2: [i128; 7],mut _3: [u8; 6],mut _4: [i128; 7],mut _5: u64,mut _6: u16,mut _7: [u32; 2],mut _8: [u128; 5],mut _9: u16) -> bool {
mir! {
type RET = bool;
let _10: u128;
let _11: *const i64;
let _12: Adt41;
let _13: &'static [i128; 7];
let _14: [u16; 6];
let _15: u32;
let _16: f64;
let _17: &'static Adt18;
let _18: [char; 7];
let _19: (u16, (bool, i64), [i128; 7], u8);
let _20: &'static char;
let _21: (i128, isize);
let _22: [i64; 8];
let _23: u8;
let _24: [i128; 8];
let _25: u8;
let _26: &'static *mut *mut isize;
let _27: bool;
let _28: isize;
let _29: i32;
let _30: u8;
let _31: [isize; 7];
let _32: &'static f32;
let _33: isize;
let _34: ();
let _35: ();
{
_5 = 9259660849332042463_u64 ^ 7310428479183777229_u64;
_1 = _3;
_3 = _1;
_6 = _9 ^ _9;
_5 = 5892390104237447301_u64;
RET = _6 == _9;
_5 = 12067096632540023326_u64;
_5 = _6 as u64;
_7 = [1866539833_u32,3918002317_u32];
_10 = (-91394238515715941295006833482323910285_i128) as u128;
_6 = _9 << _10;
_7 = [296660366_u32,1803880171_u32];
_5 = 3559355924748023560_u64;
_9 = 11_isize as u16;
_5 = 7213139874964218309_u64;
RET = !true;
RET = _5 > _5;
_1 = [170_u8,145_u8,204_u8,163_u8,89_u8,135_u8];
RET = true;
_9 = _6 - _6;
_1 = [43_u8,166_u8,212_u8,8_u8,177_u8,222_u8];
RET = false;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
7213139874964218309 => bb8,
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
_10 = 69834200566742319235300040989074151271_u128 | 287888151723490790302964797860486382443_u128;
_13 = &_4;
_3 = [144_u8,12_u8,185_u8,125_u8,78_u8,23_u8];
_6 = _9;
RET = !false;
_3 = _1;
_14 = [_6,_9,_6,_9,_6,_6];
_8 = [_10,_10,_10,_10,_10];
_1 = [127_u8,85_u8,195_u8,167_u8,209_u8,221_u8];
_7 = [3036873480_u32,2110457279_u32];
_15 = 361680628_u32;
_1 = [201_u8,210_u8,174_u8,245_u8,81_u8,56_u8];
_7 = [_15,_15];
_9 = _6;
_19.1 = (RET, 3234866882809770167_i64);
_19.0 = _6 - _9;
_18 = ['\u{d9aa4}','\u{b82a5}','\u{9c6db}','\u{4e357}','\u{af18f}','\u{389cb}','\u{def}'];
_3 = [47_u8,196_u8,246_u8,0_u8,64_u8,145_u8];
_6 = _9 << _5;
RET = _19.1.0 ^ _19.1.0;
_11 = core::ptr::addr_of!(_19.1.1);
_5 = !10093297510138645542_u64;
_10 = 306614735777671821716504642599876766681_u128;
_3 = [48_u8,126_u8,135_u8,60_u8,180_u8,229_u8];
Goto(bb9)
}
bb9 = {
_10 = 338226614436167326742229029307669427004_u128;
_7 = [_15,_15];
_16 = 68_u8 as f64;
_21.1 = (-9223372036854775808_isize);
match (*_11) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
3234866882809770167 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_22 = [(*_11),(*_11),(*_11),_19.1.1,(*_11),_19.1.1,_19.1.1,(*_11)];
_11 = core::ptr::addr_of!((*_11));
_21.1 = -44_isize;
_6 = _5 as u16;
_21.0 = (-24505_i16) as i128;
_19.3 = !187_u8;
_1 = _3;
_10 = (-103_i8) as u128;
_14 = [_9,_19.0,_19.0,_19.0,_9,_9];
_19.1 = (RET, 4346169500091135259_i64);
_2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_19.1.1 = (-4585561681126825056_i64);
_24 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
Call(_25 = core::intrinsics::bswap(_19.3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _19.1.0;
_19.2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_1 = _3;
_23 = _19.3 & _19.3;
_19.1.1 = !5469801295142601151_i64;
_19.1.0 = !RET;
_6 = _19.0 + _19.0;
_19.1.1 = 2826796891027027635_i64;
_15 = 3848912715_u32 + 2754143959_u32;
_16 = _15 as f64;
_15 = !994207971_u32;
_21 = ((-119644804165299548536474986881457971952_i128), 9223372036854775807_isize);
_9 = _6;
_19.2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_22 = [_19.1.1,(*_11),_19.1.1,(*_11),_19.1.1,(*_11),_19.1.1,(*_11)];
Goto(bb13)
}
bb13 = {
_19.1.0 = RET;
_10 = 93156013456333264216084145621124423123_u128;
RET = !_19.1.0;
_8 = [_10,_10,_10,_10,_10];
_19.1 = (RET, 7055319955357221019_i64);
_8 = [_10,_10,_10,_10,_10];
_22 = [(*_11),(*_11),(*_11),(*_11),(*_11),(*_11),_19.1.1,(*_11)];
_23 = _19.3;
_7 = [_15,_15];
_10 = _5 as u128;
_9 = '\u{e7948}' as u16;
_14 = [_6,_6,_6,_6,_6,_19.0];
_21.1 = _19.1.0 as isize;
_1 = _3;
(*_11) = (-8265688399865256576_i64);
_11 = core::ptr::addr_of!((*_11));
_19.0 = _6;
_28 = _21.1;
_14 = [_6,_19.0,_6,_6,_6,_19.0];
_19.1 = (RET, (-2088507462324731002_i64));
_27 = RET;
_9 = _19.0 >> _19.0;
RET = !_27;
_15 = 3250170750_u32;
_27 = !_19.1.0;
Goto(bb14)
}
bb14 = {
_30 = _23 * _23;
_7 = [_15,_15];
_18 = ['\u{6a04f}','\u{409e3}','\u{4c3f0}','\u{bf4e9}','\u{bd9f9}','\u{16710}','\u{8df5}'];
_16 = _15 as f64;
_2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
(*_11) = RET as i64;
_21.0 = (-22063082237210160393888606512238102817_i128) ^ 158614457657297986048794378246073820163_i128;
_13 = &_19.2;
_5 = 10320922142535883857_u64 * 9367087940752777525_u64;
_15 = 1522608383_u32 << _19.1.1;
_11 = core::ptr::addr_of!((*_11));
_19.2 = _4;
_11 = core::ptr::addr_of!((*_11));
_22 = [_19.1.1,(*_11),(*_11),(*_11),(*_11),_19.1.1,(*_11),(*_11)];
_2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
(*_11) = 4446804304965997075_i64 >> _9;
_19.1.1 = -5356752184552625256_i64;
_19.1.1 = -4157680394417770871_i64;
_8 = [_10,_10,_10,_10,_10];
_27 = _19.1.0;
_23 = _30 ^ _30;
_8 = [_10,_10,_10,_10,_10];
_13 = &_4;
_9 = _19.0;
_31 = [_28,_28,_21.1,_28,_28,_21.1,_21.1];
_21.1 = _5 as isize;
_21.1 = !_28;
RET = _19.1.0;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(16_usize, 14_usize, Move(_14), 18_usize, Move(_18), 10_usize, Move(_10), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(16_usize, 1_usize, Move(_1), 8_usize, Move(_8), 22_usize, Move(_22), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(16_usize, 4_usize, Move(_4), 30_usize, Move(_30), 7_usize, Move(_7), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: char,mut _2: isize,mut _3: u128,mut _4: ([u128; 5], [i8; 1], isize),mut _5: u128,mut _6: u128,mut _7: u128,mut _8: u128,mut _9: u128,mut _10: u128,mut _11: u128) -> isize {
mir! {
type RET = isize;
let _12: *mut *const u32;
let _13: [u32; 3];
let _14: *mut usize;
let _15: [i8; 1];
let _16: isize;
let _17: (u32, u8, i32);
let _18: bool;
let _19: isize;
let _20: [u8; 4];
let _21: [i8; 5];
let _22: isize;
let _23: &'static *const i64;
let _24: &'static f32;
let _25: *mut *mut isize;
let _26: [u128; 3];
let _27: (char, &'static [char; 7], *mut isize);
let _28: i64;
let _29: ();
let _30: ();
{
_4.2 = -_2;
RET = (-2750572805648547227_i64) as isize;
_7 = 768640478_u32 as u128;
_6 = _1 as u128;
_11 = 31747060497542093071395463328631199570_i128 as u128;
_9 = !_5;
_5 = _10 * _3;
_8 = (-902189262_i32) as u128;
_9 = 6000964056737746159_i64 as u128;
Goto(bb1)
}
bb1 = {
_5 = _10 * _3;
_1 = '\u{2d859}';
_10 = _3;
_6 = !_10;
RET = _4.2;
_7 = _5;
_13 = [1245217169_u32,1468237692_u32,2410614130_u32];
_3 = !_5;
RET = _2 & _4.2;
_9 = _5;
_15 = [107_i8];
_4.2 = 835265848_i32 as isize;
_4.0 = [_5,_5,_3,_5,_5];
_3 = _5;
_1 = '\u{fa245}';
_10 = _7;
_4.2 = !RET;
_5 = _3 | _3;
RET = !_2;
_13 = [1599708875_u32,4154229912_u32,4093911820_u32];
_4.2 = (-10398_i16) as isize;
RET = _4.2 + _2;
_16 = (-7912112087079885824_i64) as isize;
_17.2 = -(-467475662_i32);
_8 = 21986_u16 as u128;
Goto(bb2)
}
bb2 = {
_13 = [1741405307_u32,1713591202_u32,3088099643_u32];
_4.2 = RET ^ RET;
_16 = (-56879068792092461089676165814184630217_i128) as isize;
_15 = [(-30_i8)];
_17 = (2025849534_u32, 69_u8, 403862861_i32);
Call(_7 = core::intrinsics::bswap(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17.1 = 102_u8;
_15 = _4.1;
_9 = _5;
_11 = !_9;
_4.1 = _15;
_4.1 = [37_i8];
_4.0 = [_5,_9,_11,_11,_5];
_19 = _4.2;
_17.0 = 2698397184_u32 & 1904283829_u32;
_11 = (-25010004156603502975076638494930700825_i128) as u128;
_1 = '\u{96af6}';
RET = _2 + _4.2;
_4.2 = RET & RET;
_8 = 5_usize as u128;
_20 = [_17.1,_17.1,_17.1,_17.1];
RET = _17.2 as isize;
_19 = RET | _4.2;
RET = _4.2;
_1 = '\u{10da63}';
Goto(bb4)
}
bb4 = {
_17.1 = !228_u8;
_22 = _4.2;
_17.1 = _4.2 as u8;
_21 = [(-52_i8),78_i8,(-86_i8),109_i8,(-99_i8)];
_17.0 = 64079_u16 as u32;
_4.2 = _19;
_17.0 = (-8327116729819349429_i64) as u32;
_18 = false;
_15 = [62_i8];
_17.1 = 18134987414598302227_u64 as u8;
_17 = (3135591050_u32, 248_u8, (-76785685_i32));
_4.1 = [15_i8];
_4.2 = 82_i8 as isize;
_18 = true & false;
_17.2 = 9376421458886415172_u64 as i32;
_9 = _1 as u128;
Goto(bb5)
}
bb5 = {
_5 = (-8793205078822236923_i64) as u128;
Goto(bb6)
}
bb6 = {
_8 = _11;
_11 = _3 >> _3;
_4.2 = -_22;
_7 = _6 >> _11;
match _17.1 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
248 => bb12,
_ => bb11
}
}
bb7 = {
_5 = (-8793205078822236923_i64) as u128;
Goto(bb6)
}
bb8 = {
_17.1 = !228_u8;
_22 = _4.2;
_17.1 = _4.2 as u8;
_21 = [(-52_i8),78_i8,(-86_i8),109_i8,(-99_i8)];
_17.0 = 64079_u16 as u32;
_4.2 = _19;
_17.0 = (-8327116729819349429_i64) as u32;
_18 = false;
_15 = [62_i8];
_17.1 = 18134987414598302227_u64 as u8;
_17 = (3135591050_u32, 248_u8, (-76785685_i32));
_4.1 = [15_i8];
_4.2 = 82_i8 as isize;
_18 = true & false;
_17.2 = 9376421458886415172_u64 as i32;
_9 = _1 as u128;
Goto(bb5)
}
bb9 = {
_17.1 = 102_u8;
_15 = _4.1;
_9 = _5;
_11 = !_9;
_4.1 = _15;
_4.1 = [37_i8];
_4.0 = [_5,_9,_11,_11,_5];
_19 = _4.2;
_17.0 = 2698397184_u32 & 1904283829_u32;
_11 = (-25010004156603502975076638494930700825_i128) as u128;
_1 = '\u{96af6}';
RET = _2 + _4.2;
_4.2 = RET & RET;
_8 = 5_usize as u128;
_20 = [_17.1,_17.1,_17.1,_17.1];
RET = _17.2 as isize;
_19 = RET | _4.2;
RET = _4.2;
_1 = '\u{10da63}';
Goto(bb4)
}
bb10 = {
_13 = [1741405307_u32,1713591202_u32,3088099643_u32];
_4.2 = RET ^ RET;
_16 = (-56879068792092461089676165814184630217_i128) as isize;
_15 = [(-30_i8)];
_17 = (2025849534_u32, 69_u8, 403862861_i32);
Call(_7 = core::intrinsics::bswap(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5 = _10 * _3;
_1 = '\u{2d859}';
_10 = _3;
_6 = !_10;
RET = _4.2;
_7 = _5;
_13 = [1245217169_u32,1468237692_u32,2410614130_u32];
_3 = !_5;
RET = _2 & _4.2;
_9 = _5;
_15 = [107_i8];
_4.2 = 835265848_i32 as isize;
_4.0 = [_5,_5,_3,_5,_5];
_3 = _5;
_1 = '\u{fa245}';
_10 = _7;
_4.2 = !RET;
_5 = _3 | _3;
RET = !_2;
_13 = [1599708875_u32,4154229912_u32,4093911820_u32];
_4.2 = (-10398_i16) as isize;
RET = _4.2 + _2;
_16 = (-7912112087079885824_i64) as isize;
_17.2 = -(-467475662_i32);
_8 = 21986_u16 as u128;
Goto(bb2)
}
bb12 = {
_4.0 = [_6,_7,_5,_3,_11];
_15 = [8_i8];
_18 = false ^ false;
_6 = _3;
_11 = !_3;
_5 = 13251534016484864856_usize as u128;
_2 = RET;
_17.1 = 98_u8 >> _7;
_17.2 = _17.0 as i32;
_17.2 = _18 as i32;
_4.1 = [61_i8];
match _17.0 {
0 => bb9,
1 => bb13,
3135591050 => bb15,
_ => bb14
}
}
bb13 = {
_5 = _10 * _3;
_1 = '\u{2d859}';
_10 = _3;
_6 = !_10;
RET = _4.2;
_7 = _5;
_13 = [1245217169_u32,1468237692_u32,2410614130_u32];
_3 = !_5;
RET = _2 & _4.2;
_9 = _5;
_15 = [107_i8];
_4.2 = 835265848_i32 as isize;
_4.0 = [_5,_5,_3,_5,_5];
_3 = _5;
_1 = '\u{fa245}';
_10 = _7;
_4.2 = !RET;
_5 = _3 | _3;
RET = !_2;
_13 = [1599708875_u32,4154229912_u32,4093911820_u32];
_4.2 = (-10398_i16) as isize;
RET = _4.2 + _2;
_16 = (-7912112087079885824_i64) as isize;
_17.2 = -(-467475662_i32);
_8 = 21986_u16 as u128;
Goto(bb2)
}
bb14 = {
_17.1 = !228_u8;
_22 = _4.2;
_17.1 = _4.2 as u8;
_21 = [(-52_i8),78_i8,(-86_i8),109_i8,(-99_i8)];
_17.0 = 64079_u16 as u32;
_4.2 = _19;
_17.0 = (-8327116729819349429_i64) as u32;
_18 = false;
_15 = [62_i8];
_17.1 = 18134987414598302227_u64 as u8;
_17 = (3135591050_u32, 248_u8, (-76785685_i32));
_4.1 = [15_i8];
_4.2 = 82_i8 as isize;
_18 = true & false;
_17.2 = 9376421458886415172_u64 as i32;
_9 = _1 as u128;
Goto(bb5)
}
bb15 = {
_8 = _6 & _7;
_21 = [(-7_i8),(-69_i8),(-52_i8),6_i8,(-55_i8)];
_20 = [_17.1,_17.1,_17.1,_17.1];
_10 = (-25304164539121557_i64) as u128;
_4.2 = _7 as isize;
_1 = '\u{f227e}';
_20 = [_17.1,_17.1,_17.1,_17.1];
_9 = (-56_i8) as u128;
_17.1 = _1 as u8;
_7 = _8;
_19 = _4.2 + _22;
_1 = '\u{9428f}';
_27.0 = _1;
_5 = _8 << _8;
_21 = [(-17_i8),(-97_i8),(-19_i8),15_i8,(-100_i8)];
_21 = [(-96_i8),(-80_i8),(-115_i8),(-46_i8),(-102_i8)];
_19 = _2 & _4.2;
_6 = _8;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(17_usize, 4_usize, Move(_4), 8_usize, Move(_8), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(17_usize, 7_usize, Move(_7), 21_usize, Move(_21), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(17_usize, 11_usize, Move(_11), 18_usize, Move(_18), 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: bool,
fld1: i128,
fld2: u64,
fld3: usize,
fld4: u16,

},
Variant1{
fld0: i8,
fld1: u16,
fld2: i64,

},
Variant2{
fld0: char,

}}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt30{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt30 {
fld0: u16,
}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: Adt30,
fld1: [u32; 2],
fld2: [u128; 5],
fld3: u128,
fld4: f64,
fld5: u16,
fld6: f32,

},
Variant1{
fld0: f32,
fld1: Adt30,
fld2: *const u32,
fld3: u128,

},
Variant2{
fld0: u64,
fld1: [u32; 2],
fld2: [i128; 7],
fld3: [usize; 4],
fld4: u128,
fld5: Adt18,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: [u32; 2],
fld1: (u8,),

},
Variant1{
fld0: [i128; 8],
fld1: char,
fld2: i128,
fld3: i8,
fld4: usize,
fld5: (u32, u8, i32),
fld6: [i8; 1],

},
Variant2{
fld0: *const i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u32; 2],

},
Variant1{
fld0: f32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: *const u32,
fld1: usize,
fld2: i32,
fld3: Adt18,
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt52,
fld1: (i128, isize),

},
Variant1{
fld0: isize,
fld1: usize,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: [char; 7],
fld1: (u16, (bool, i64), [i128; 7], u8),
fld2: isize,
fld3: u32,
fld4: i128,
fld5: (u8,),
fld6: usize,
}

