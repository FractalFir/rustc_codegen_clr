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
pub fn fn0(mut _1: u128,mut _2: char,mut _3: i128,mut _4: i64,mut _5: u64) -> (u128,) {
mir! {
type RET = (u128,);
let _6: Adt28;
let _7: f64;
let _8: u16;
let _9: (u8, usize, f32, isize);
let _10: *mut *const u64;
let _11: u128;
let _12: &'static (u8, usize, f32, isize);
let _13: &'static (u128, u128, *const &'static i32);
let _14: *const ([u16; 6], Adt35, *const &'static f32);
let _15: Adt35;
let _16: char;
let _17: isize;
let _18: &'static f32;
let _19: Adt35;
let _20: (u8, usize, f32, isize);
let _21: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _22: [usize; 3];
let _23: isize;
let _24: u8;
let _25: usize;
let _26: Adt36;
let _27: isize;
let _28: &'static &'static char;
let _29: [i16; 5];
let _30: ();
let _31: ();
{
_4 = 2869490754132026755_i64 ^ (-4246842511437253264_i64);
RET.0 = 120197486308447728146691362352269113133_u128;
Call(RET = fn1(_4, _4, _4, _4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = RET.0;
RET.0 = !_1;
_1 = RET.0;
_5 = 2566159506758019600_u64;
_5 = 1612704126404233170_u64;
_2 = '\u{c2d08}';
_5 = 6276175666219774359_u64;
_9.3 = -9223372036854775807_isize;
_9.0 = _5 as u8;
RET.0 = _1;
_4 = 8683740322068064030_i64;
_6 = Adt28::Variant0 { fld0: (-12136_i16),fld1: 54830_u16,fld2: 2181694250_u32 };
Call(place!(Field::<u16>(Variant(_6, 0), 1)) = core::intrinsics::bswap(41302_u16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = RET.0 << Field::<u16>(Variant(_6, 0), 1);
_12 = &_9;
_3 = (-98461564315066415585165264042707749843_i128) * 154914524756789497717839125551510700513_i128;
_9.1 = 0_usize ^ 7_usize;
place!(Field::<u32>(Variant(_6, 0), 2)) = 3481264284_u32;
_9.1 = (*_12).0 as usize;
_8 = Field::<u16>(Variant(_6, 0), 1) >> (*_12).0;
_4 = (-4339238917687294567_i64) + 3866063041025853560_i64;
_3 = 65090750428263445701125218719139902543_i128;
_7 = (-15124_i16) as f64;
_11 = !RET.0;
place!(Field::<u16>(Variant(_6, 0), 1)) = !_8;
Goto(bb3)
}
bb3 = {
_5 = 12179918437512194982_u64 | 4443254006500332256_u64;
_9.3 = (-9223372036854775808_isize);
_7 = _8 as f64;
_8 = Field::<u16>(Variant(_6, 0), 1);
_11 = _5 as u128;
_1 = RET.0 & _11;
place!(Field::<i16>(Variant(_6, 0), 0)) = (-1199927016_i32) as i16;
_12 = &_9;
_12 = &_9;
_9.3 = 82_i8 as isize;
_9.2 = (-87_i8) as f32;
_12 = &_9;
_4 = (-8352569814556548051_i64);
RET.0 = _11;
SetDiscriminant(_6, 0);
_5 = _8 as u64;
_9.3 = !9223372036854775807_isize;
_15 = Adt35::Variant1 { fld0: (*_12).1 };
_9.3 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_7 = (-1490585284_i32) as f64;
RET = (_1,);
SetDiscriminant(_15, 3);
Goto(bb4)
}
bb4 = {
_3 = !76977834380654582480731604535360757235_i128;
place!(Field::<u16>(Variant(_6, 0), 1)) = (*_12).2 as u16;
place!(Field::<isize>(Variant(_15, 3), 2)) = _9.3;
_7 = (-691171773_i32) as f64;
place!(Field::<i32>(Variant(_15, 3), 5)) = -(-1390341744_i32);
place!(Field::<usize>(Variant(_15, 3), 3)) = !(*_12).1;
_9.3 = Field::<isize>(Variant(_15, 3), 2) | Field::<isize>(Variant(_15, 3), 2);
place!(Field::<i32>(Variant(_15, 3), 5)) = _8 as i32;
place!(Field::<u32>(Variant(_6, 0), 2)) = _8 as u32;
RET = (_11,);
_9.3 = (*_12).1 as isize;
_7 = (*_12).1 as f64;
_5 = 12074659569777624991_u64 >> Field::<u32>(Variant(_6, 0), 2);
_9.1 = Field::<usize>(Variant(_15, 3), 3);
RET = (_11,);
_12 = &_9;
_15 = Adt35::Variant1 { fld0: _9.1 };
_8 = _5 as u16;
RET.0 = _1 | _1;
place!(Field::<usize>(Variant(_15, 1), 0)) = _9.1;
_16 = _2;
place!(Field::<i16>(Variant(_6, 0), 0)) = (-20741_i16) >> _5;
_9.3 = _9.2 as isize;
_9.0 = 169_u8 | 152_u8;
_3 = !23453850242999100099185019045332885128_i128;
_7 = _8 as f64;
_9.3 = !9223372036854775807_isize;
_12 = &_9;
_9.0 = !136_u8;
match _4 {
340282366920938463455022037617211663405 => bb6,
_ => bb5
}
}
bb5 = {
_5 = 12179918437512194982_u64 | 4443254006500332256_u64;
_9.3 = (-9223372036854775808_isize);
_7 = _8 as f64;
_8 = Field::<u16>(Variant(_6, 0), 1);
_11 = _5 as u128;
_1 = RET.0 & _11;
place!(Field::<i16>(Variant(_6, 0), 0)) = (-1199927016_i32) as i16;
_12 = &_9;
_12 = &_9;
_9.3 = 82_i8 as isize;
_9.2 = (-87_i8) as f32;
_12 = &_9;
_4 = (-8352569814556548051_i64);
RET.0 = _11;
SetDiscriminant(_6, 0);
_5 = _8 as u64;
_9.3 = !9223372036854775807_isize;
_15 = Adt35::Variant1 { fld0: (*_12).1 };
_9.3 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_7 = (-1490585284_i32) as f64;
RET = (_1,);
SetDiscriminant(_15, 3);
Goto(bb4)
}
bb6 = {
_8 = Field::<u16>(Variant(_6, 0), 1);
_9.2 = 545889664_i32 as f32;
_4 = (-2819858807571012785_i64) << Field::<i16>(Variant(_6, 0), 0);
place!(Field::<i16>(Variant(_6, 0), 0)) = (-30308_i16) * (-18599_i16);
_1 = !_11;
_9.2 = _9.1 as f32;
_18 = &_9.2;
SetDiscriminant(_15, 0);
RET.0 = (*_12).1 as u128;
_17 = !(*_12).3;
_5 = _1 as u64;
Call(_9.3 = core::intrinsics::transmute(_4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_18 = &(*_18);
RET = (_11,);
place!(Field::<i16>(Variant(_6, 0), 0)) = -(-18839_i16);
_15 = Adt35::Variant0 { fld0: _4 };
_12 = &_9;
_6 = Adt28::Variant0 { fld0: 15855_i16,fld1: _8,fld2: 748665792_u32 };
_11 = RET.0;
_17 = (*_12).3 | (*_12).3;
_12 = &(*_12);
_18 = &_9.2;
_2 = _16;
_17 = -_9.3;
_16 = _2;
_5 = 10870_i16 as u64;
_17 = (*_12).3;
_12 = &_9;
_3 = (-22301_i16) as i128;
SetDiscriminant(_15, 1);
_20 = ((*_12).0, _9.1, _9.2, _17);
_18 = &(*_18);
_8 = Field::<u16>(Variant(_6, 0), 1) << (*_12).3;
_8 = Field::<u16>(Variant(_6, 0), 1);
_21.0 = -(*_12).3;
_1 = (*_12).3 as u128;
_9.2 = _20.2;
Goto(bb8)
}
bb8 = {
_22 = [(*_12).1,_20.1,_20.1];
_21.1 = _9;
_15 = Adt35::Variant0 { fld0: _4 };
_19 = Adt35::Variant1 { fld0: _9.1 };
_9 = (_20.0, Field::<usize>(Variant(_19, 1), 0), _21.1.2, _17);
Goto(bb9)
}
bb9 = {
_21.0 = _9.3;
_20.0 = 1339981983_i32 as u8;
_20 = (_21.1.0, Field::<usize>(Variant(_19, 1), 0), _21.1.2, _9.3);
_2 = _16;
_20 = _9;
_20.3 = _5 as isize;
_17 = !_21.0;
place!(Field::<usize>(Variant(_19, 1), 0)) = _2 as usize;
place!(Field::<i16>(Variant(_6, 0), 0)) = (-20484_i16) ^ 20533_i16;
_20.2 = _9.2;
_21.2 = &_9.2;
RET = (_1,);
_25 = !Field::<usize>(Variant(_19, 1), 0);
_21.1.0 = _9.0;
_12 = &_21.1;
_2 = _16;
SetDiscriminant(_19, 1);
place!(Field::<usize>(Variant(_19, 1), 0)) = !(*_12).1;
_21.3 = !true;
_19 = Adt35::Variant0 { fld0: _4 };
_9.3 = -(*_12).3;
_18 = &_21.1.2;
_18 = Move(_21.2);
_21.2 = &_21.1.2;
Goto(bb10)
}
bb10 = {
_2 = _16;
_6 = Adt28::Variant2 { fld0: _9,fld1: _2,fld2: _5,fld3: _1 };
_20.3 = _21.0 & (*_12).3;
_24 = Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).0;
_20 = (Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).0, _9.1, _9.2, Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).3);
place!(Field::<char>(Variant(_6, 2), 1)) = _2;
RET = (Field::<u128>(Variant(_6, 2), 3),);
RET.0 = _1 - _1;
_21.0 = -(*_12).3;
place!(Field::<u64>(Variant(_6, 2), 2)) = _21.3 as u64;
_26 = Adt36::Variant2 { fld0: _2 };
_21.3 = _17 >= _9.3;
_9.1 = Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).1;
_27 = !(*_12).3;
place!(Field::<u64>(Variant(_6, 2), 2)) = 266823238_u32 as u64;
_20.3 = (-28454_i16) as isize;
_5 = !Field::<u64>(Variant(_6, 2), 2);
_5 = !Field::<u64>(Variant(_6, 2), 2);
_24 = _20.0;
_1 = Field::<u128>(Variant(_6, 2), 3);
_27 = Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).3 + (*_12).3;
_1 = !RET.0;
place!(Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0)).0 = _21.3 as u8;
place!(Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0)).2 = _9.2;
_21.0 = Field::<(u8, usize, f32, isize)>(Variant(_6, 2), 0).0 as isize;
_21.1.2 = _9.2;
Goto(bb11)
}
bb11 = {
Call(_30 = dump_var(0_usize, 11_usize, Move(_11), 5_usize, Move(_5), 16_usize, Move(_16), 17_usize, Move(_17)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_30 = dump_var(0_usize, 22_usize, Move(_22), 8_usize, Move(_8), 31_usize, _31, 31_usize, _31), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64) -> (u128,) {
mir! {
type RET = (u128,);
let _7: usize;
let _8: Adt83;
let _9: i64;
let _10: f64;
let _11: &'static i32;
let _12: f64;
let _13: f32;
let _14: i64;
let _15: *mut *const u64;
let _16: f32;
let _17: char;
let _18: Adt65;
let _19: u128;
let _20: [i8; 1];
let _21: &'static &'static char;
let _22: u64;
let _23: i64;
let _24: *const f64;
let _25: i8;
let _26: ();
let _27: ();
{
RET = (83832440855082319865828829358238584505_u128,);
_2 = RET.0 as i64;
_5 = _6 + _6;
RET = (203972869744678392375370414213959101752_u128,);
_4 = 61059_u16 as i64;
RET.0 = !299148956577694084876078729213704996771_u128;
_7 = 17281336122728893396_usize;
_2 = _1;
_2 = _3;
RET = (132816611111793854596714842386672059075_u128,);
_4 = _2;
_6 = _1 + _1;
_1 = _3 | _6;
_1 = -_5;
Call(_8 = fn2(_2, _5, RET.0, _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<i16>(Variant(_8, 0), 0)) = 23055_i16 ^ 6618_i16;
_5 = _3;
RET.0 = 34928074550700154739142438925228505659_u128;
_7 = Field::<usize>(Variant(_8, 0), 2) + Field::<usize>(Variant(_8, 0), 2);
place!(Field::<i128>(Variant(_8, 0), 1)) = 63109323752615474365530532094508911005_i128 >> Field::<usize>(Variant(_8, 0), 2);
_9 = true as i64;
_4 = -_1;
_2 = Field::<i128>(Variant(_8, 0), 1) as i64;
place!(Field::<usize>(Variant(_8, 0), 2)) = _7 << _7;
_6 = 13695523053090906598_u64 as i64;
place!(Field::<usize>(Variant(_8, 0), 2)) = _7 * _7;
place!(Field::<i16>(Variant(_8, 0), 0)) = (-22652_i16);
RET = (95636591247038063842297221520983776103_u128,);
place!(Field::<i16>(Variant(_8, 0), 0)) = (-19368_i16) & 16482_i16;
_8 = Adt83::Variant1 { fld0: '\u{7dd89}' };
RET = (240453558452797499952935134110244084689_u128,);
_10 = _3 as f64;
_10 = _1 as f64;
RET.0 = 231906510015827141486596053898122738564_u128;
place!(Field::<char>(Variant(_8, 1), 0)) = '\u{ed0bc}';
RET.0 = 710669458_u32 as u128;
_4 = -_2;
_12 = _10 + _10;
_12 = (-19502800868646034283065035995652838185_i128) as f64;
_1 = !_4;
Goto(bb2)
}
bb2 = {
_3 = _2;
Goto(bb3)
}
bb3 = {
_12 = _10 - _10;
_8 = Adt83::Variant0 { fld0: 1272_i16,fld1: (-154178676690476633097108048551997389821_i128),fld2: _7 };
_7 = Field::<usize>(Variant(_8, 0), 2) | Field::<usize>(Variant(_8, 0), 2);
RET.0 = 151819663995234151945346431892367867083_u128 - 110143197286912730952473978575276449601_u128;
_7 = _3 as usize;
_10 = -_12;
RET.0 = (-48_isize) as u128;
_6 = _4 + _5;
RET = (281313846943129620234025576587674022493_u128,);
_6 = _1 + _1;
_3 = 10307872220603948534_u64 as i64;
_6 = _1 >> Field::<usize>(Variant(_8, 0), 2);
RET.0 = 168420634008415619236635014512722055510_u128;
_8 = Adt83::Variant0 { fld0: 16290_i16,fld1: (-116613082516416452048365310359832155213_i128),fld2: _7 };
_5 = _9;
_3 = _4 ^ _1;
_8 = Adt83::Variant0 { fld0: 6535_i16,fld1: (-106991213465955834597897974176840697626_i128),fld2: _7 };
place!(Field::<i16>(Variant(_8, 0), 0)) = (-14725_i16);
_12 = _10 * _10;
_18.fld0 = 154_u8;
place!(Field::<i128>(Variant(_8, 0), 1)) = !82665503468399639790860805368409666301_i128;
SetDiscriminant(_8, 1);
Goto(bb4)
}
bb4 = {
_18.fld0 = !254_u8;
RET.0 = 40554985314670827419399498215266428377_u128;
_20 = [(-116_i8)];
_13 = 5529_u16 as f32;
_9 = !_4;
Goto(bb5)
}
bb5 = {
_16 = _13;
_19 = RET.0 | RET.0;
_1 = _2;
_1 = _6;
_18 = Adt65 { fld0: 28_u8 };
_20 = [(-95_i8)];
match RET.0 {
0 => bb1,
1 => bb2,
40554985314670827419399498215266428377 => bb6,
_ => bb4
}
}
bb6 = {
_5 = _6 << _1;
match _18.fld0 {
0 => bb1,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
28 => bb13,
_ => bb12
}
}
bb7 = {
_16 = _13;
_19 = RET.0 | RET.0;
_1 = _2;
_1 = _6;
_18 = Adt65 { fld0: 28_u8 };
_20 = [(-95_i8)];
match RET.0 {
0 => bb1,
1 => bb2,
40554985314670827419399498215266428377 => bb6,
_ => bb4
}
}
bb8 = {
_18.fld0 = !254_u8;
RET.0 = 40554985314670827419399498215266428377_u128;
_20 = [(-116_i8)];
_13 = 5529_u16 as f32;
_9 = !_4;
Goto(bb5)
}
bb9 = {
_12 = _10 - _10;
_8 = Adt83::Variant0 { fld0: 1272_i16,fld1: (-154178676690476633097108048551997389821_i128),fld2: _7 };
_7 = Field::<usize>(Variant(_8, 0), 2) | Field::<usize>(Variant(_8, 0), 2);
RET.0 = 151819663995234151945346431892367867083_u128 - 110143197286912730952473978575276449601_u128;
_7 = _3 as usize;
_10 = -_12;
RET.0 = (-48_isize) as u128;
_6 = _4 + _5;
RET = (281313846943129620234025576587674022493_u128,);
_6 = _1 + _1;
_3 = 10307872220603948534_u64 as i64;
_6 = _1 >> Field::<usize>(Variant(_8, 0), 2);
RET.0 = 168420634008415619236635014512722055510_u128;
_8 = Adt83::Variant0 { fld0: 16290_i16,fld1: (-116613082516416452048365310359832155213_i128),fld2: _7 };
_5 = _9;
_3 = _4 ^ _1;
_8 = Adt83::Variant0 { fld0: 6535_i16,fld1: (-106991213465955834597897974176840697626_i128),fld2: _7 };
place!(Field::<i16>(Variant(_8, 0), 0)) = (-14725_i16);
_12 = _10 * _10;
_18.fld0 = 154_u8;
place!(Field::<i128>(Variant(_8, 0), 1)) = !82665503468399639790860805368409666301_i128;
SetDiscriminant(_8, 1);
Goto(bb4)
}
bb10 = {
_3 = _2;
Goto(bb3)
}
bb11 = {
place!(Field::<i16>(Variant(_8, 0), 0)) = 23055_i16 ^ 6618_i16;
_5 = _3;
RET.0 = 34928074550700154739142438925228505659_u128;
_7 = Field::<usize>(Variant(_8, 0), 2) + Field::<usize>(Variant(_8, 0), 2);
place!(Field::<i128>(Variant(_8, 0), 1)) = 63109323752615474365530532094508911005_i128 >> Field::<usize>(Variant(_8, 0), 2);
_9 = true as i64;
_4 = -_1;
_2 = Field::<i128>(Variant(_8, 0), 1) as i64;
place!(Field::<usize>(Variant(_8, 0), 2)) = _7 << _7;
_6 = 13695523053090906598_u64 as i64;
place!(Field::<usize>(Variant(_8, 0), 2)) = _7 * _7;
place!(Field::<i16>(Variant(_8, 0), 0)) = (-22652_i16);
RET = (95636591247038063842297221520983776103_u128,);
place!(Field::<i16>(Variant(_8, 0), 0)) = (-19368_i16) & 16482_i16;
_8 = Adt83::Variant1 { fld0: '\u{7dd89}' };
RET = (240453558452797499952935134110244084689_u128,);
_10 = _3 as f64;
_10 = _1 as f64;
RET.0 = 231906510015827141486596053898122738564_u128;
place!(Field::<char>(Variant(_8, 1), 0)) = '\u{ed0bc}';
RET.0 = 710669458_u32 as u128;
_4 = -_2;
_12 = _10 + _10;
_12 = (-19502800868646034283065035995652838185_i128) as f64;
_1 = !_4;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_19 = RET.0 ^ RET.0;
_20 = [(-72_i8)];
_8 = Adt83::Variant0 { fld0: 22938_i16,fld1: 82917114086326381360946221610248419394_i128,fld2: _7 };
_17 = '\u{38b8b}';
_18 = Adt65 { fld0: 109_u8 };
_9 = _3;
_13 = (-20278_i16) as f32;
_23 = _6 << _4;
place!(Field::<i16>(Variant(_8, 0), 0)) = (-114143650047848664589405985266821778713_i128) as i16;
_14 = _23;
_17 = '\u{18842}';
_6 = _1 ^ _5;
_20 = [(-17_i8)];
Goto(bb14)
}
bb14 = {
_14 = -_9;
_19 = RET.0;
place!(Field::<i128>(Variant(_8, 0), 1)) = 76946283584616715444824849323204440312_i128;
_9 = true as i64;
RET = (_19,);
_24 = core::ptr::addr_of!(_10);
_5 = _6;
RET = (_19,);
SetDiscriminant(_8, 1);
_7 = (*_24) as usize;
_18 = Adt65 { fld0: 197_u8 };
_1 = _6;
_12 = (*_24);
RET = (_19,);
_12 = 1305995370_u32 as f64;
_14 = _5;
RET.0 = _19;
_6 = _18.fld0 as i64;
_1 = (-160771555927151263799323684962947934395_i128) as i64;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(1_usize, 14_usize, Move(_14), 7_usize, Move(_7), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(1_usize, 17_usize, Move(_17), 2_usize, Move(_2), 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64,mut _2: i64,mut _3: u128,mut _4: i64,mut _5: usize) -> Adt83 {
mir! {
type RET = Adt83;
let _6: u16;
let _7: Adt83;
let _8: Adt35;
let _9: (u8,);
let _10: ();
let _11: ();
{
RET = Adt83::Variant0 { fld0: 20232_i16,fld1: 58115066372558977054661742772125407356_i128,fld2: _5 };
place!(Field::<i128>(Variant(RET, 0), 1)) = -(-79571682741980048648453441809666658430_i128);
place!(Field::<i16>(Variant(RET, 0), 0)) = 777085979_u32 as i16;
_1 = 14900270755819764013_u64 as i64;
place!(Field::<i16>(Variant(RET, 0), 0)) = (-11359_i16);
place!(Field::<usize>(Variant(RET, 0), 2)) = _5;
Call(_5 = fn3(_4, _2, _2, _2, Field::<i128>(Variant(RET, 0), 1), _3, Field::<i16>(Variant(RET, 0), 0), RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<usize>(Variant(RET, 0), 2)) = _5 >> _4;
place!(Field::<i128>(Variant(RET, 0), 1)) = (-8772906413480774487431557398793629267_i128);
_5 = !Field::<usize>(Variant(RET, 0), 2);
_3 = 285615213647823565586109051806759590994_u128;
RET = Adt83::Variant0 { fld0: (-3238_i16),fld1: 41373279821381012176101200361816367697_i128,fld2: _5 };
_1 = !_4;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
285615213647823565586109051806759590994 => bb6,
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
_4 = _1;
RET = Adt83::Variant0 { fld0: 4916_i16,fld1: 118750167677769273613604235868866469907_i128,fld2: _5 };
_4 = _5 as i64;
RET = Adt83::Variant0 { fld0: 28702_i16,fld1: (-86011440403347933331887880322316739382_i128),fld2: _5 };
_1 = _4 << Field::<usize>(Variant(RET, 0), 2);
_4 = -_1;
place!(Field::<i16>(Variant(RET, 0), 0)) = 22437_i16;
place!(Field::<i128>(Variant(RET, 0), 1)) = !(-116700225025864985186446346670055966564_i128);
_7 = Adt83::Variant0 { fld0: Field::<i16>(Variant(RET, 0), 0),fld1: Field::<i128>(Variant(RET, 0), 1),fld2: Field::<usize>(Variant(RET, 0), 2) };
_6 = 2519_u16 | 42782_u16;
_2 = _4;
Goto(bb7)
}
bb7 = {
Call(_10 = dump_var(2_usize, 4_usize, Move(_4), 5_usize, Move(_5), 6_usize, Move(_6), 11_usize, _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i128,mut _6: u128,mut _7: i16,mut _8: Adt83,mut _9: Adt83,mut _10: Adt83) -> usize {
mir! {
type RET = usize;
let _11: u128;
let _12: [u64; 8];
let _13: (u8, usize, f32, isize);
let _14: *mut [char; 1];
let _15: isize;
let _16: isize;
let _17: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _18: (u128, u8, usize);
let _19: f64;
let _20: Adt83;
let _21: i128;
let _22: (u128,);
let _23: [i8; 3];
let _24: [i8; 1];
let _25: u16;
let _26: bool;
let _27: f32;
let _28: f32;
let _29: *mut *const u64;
let _30: bool;
let _31: [i16; 5];
let _32: char;
let _33: bool;
let _34: *const &'static f32;
let _35: isize;
let _36: &'static &'static char;
let _37: i32;
let _38: isize;
let _39: &'static Adt36;
let _40: ();
let _41: ();
{
RET = !Field::<usize>(Variant(_9, 0), 2);
place!(Field::<i16>(Variant(_8, 0), 0)) = Field::<i16>(Variant(_9, 0), 0) + _7;
_10 = _8;
place!(Field::<i128>(Variant(_9, 0), 1)) = _5 * Field::<i128>(Variant(_8, 0), 1);
SetDiscriminant(_10, 0);
place!(Field::<i128>(Variant(_10, 0), 1)) = 38413_u16 as i128;
place!(Field::<i16>(Variant(_8, 0), 0)) = -Field::<i16>(Variant(_9, 0), 0);
_1 = _2 * _4;
_7 = !Field::<i16>(Variant(_8, 0), 0);
place!(Field::<i16>(Variant(_8, 0), 0)) = Field::<i16>(Variant(_9, 0), 0) & _7;
place!(Field::<i16>(Variant(_9, 0), 0)) = Field::<i16>(Variant(_8, 0), 0);
_11 = _6;
place!(Field::<usize>(Variant(_10, 0), 2)) = Field::<usize>(Variant(_8, 0), 2);
_10 = _9;
_3 = _4;
_1 = _4 ^ _3;
place!(Field::<usize>(Variant(_10, 0), 2)) = Field::<usize>(Variant(_9, 0), 2);
place!(Field::<i128>(Variant(_10, 0), 1)) = '\u{f1c4e}' as i128;
_10 = Adt83::Variant1 { fld0: '\u{fcd24}' };
RET = Field::<usize>(Variant(_8, 0), 2);
_6 = !_11;
_1 = 72_i8 as i64;
Goto(bb1)
}
bb1 = {
_2 = _4;
place!(Field::<char>(Variant(_10, 1), 0)) = '\u{10b212}';
_12 = [13549710371722378541_u64,10752887405222346492_u64,17708870220305012519_u64,11611750977929664076_u64,4259571295170466490_u64,16770540586899800321_u64,14575802953043056893_u64,1858036509925761586_u64];
_13.3 = 9223372036854775807_isize - 9223372036854775807_isize;
_11 = _6 >> Field::<usize>(Variant(_9, 0), 2);
_13.0 = 18_u8;
SetDiscriminant(_9, 0);
_10 = _8;
_7 = -Field::<i16>(Variant(_8, 0), 0);
place!(Field::<usize>(Variant(_9, 0), 2)) = '\u{89eda}' as usize;
_11 = _6 + _6;
_11 = _3 as u128;
place!(Field::<i128>(Variant(_10, 0), 1)) = -_5;
_9 = Adt83::Variant0 { fld0: Field::<i16>(Variant(_8, 0), 0),fld1: Field::<i128>(Variant(_10, 0), 1),fld2: Field::<usize>(Variant(_10, 0), 2) };
place!(Field::<i128>(Variant(_9, 0), 1)) = Field::<i128>(Variant(_8, 0), 1);
_11 = !_6;
place!(Field::<i16>(Variant(_10, 0), 0)) = '\u{ed7c2}' as i16;
RET = !Field::<usize>(Variant(_9, 0), 2);
place!(Field::<usize>(Variant(_8, 0), 2)) = _13.0 as usize;
Call(_13.1 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13.2 = 1765423886_i32 as f32;
_6 = _5 as u128;
_9 = _8;
_3 = !_4;
_4 = !_2;
place!(Field::<usize>(Variant(_10, 0), 2)) = RET >> Field::<i16>(Variant(_10, 0), 0);
SetDiscriminant(_8, 1);
_9 = Adt83::Variant0 { fld0: _7,fld1: Field::<i128>(Variant(_10, 0), 1),fld2: RET };
_3 = _1;
_13.0 = !117_u8;
SetDiscriminant(_10, 1);
place!(Field::<char>(Variant(_8, 1), 0)) = '\u{7028a}';
_8 = _9;
_2 = _3;
SetDiscriminant(_9, 0);
_7 = true as i16;
_5 = _13.3 as i128;
place!(Field::<usize>(Variant(_9, 0), 2)) = _5 as usize;
place!(Field::<char>(Variant(_10, 1), 0)) = '\u{c7b0c}';
_5 = Field::<i128>(Variant(_8, 0), 1) * Field::<i128>(Variant(_8, 0), 1);
_9 = _8;
_13.3 = _6 as isize;
place!(Field::<char>(Variant(_10, 1), 0)) = '\u{e9061}';
_12 = [7859286510468220600_u64,9930111513050639482_u64,147821490607225606_u64,16918325236602096880_u64,11668184129369771932_u64,15464379781770492521_u64,5860249069766343373_u64,13804266610554448915_u64];
_8 = Adt83::Variant0 { fld0: Field::<i16>(Variant(_9, 0), 0),fld1: Field::<i128>(Variant(_9, 0), 1),fld2: Field::<usize>(Variant(_9, 0), 2) };
place!(Field::<i128>(Variant(_8, 0), 1)) = !_5;
place!(Field::<i128>(Variant(_8, 0), 1)) = _5;
Call(_7 = fn4(_12, _13, _13, _4, _13, _4, Field::<i16>(Variant(_8, 0), 0), _13.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17.3.2 = false;
_13.0 = !168_u8;
_16 = 3207061445_u32 as isize;
place!(Field::<usize>(Variant(_9, 0), 2)) = _13.1;
_16 = _13.3 | _13.3;
_4 = _17.3.2 as i64;
_3 = _4 << _7;
_2 = _1;
_14 = core::ptr::addr_of_mut!(_17.1);
_17.0 = !12967018_u32;
place!(Field::<usize>(Variant(_8, 0), 2)) = RET;
_17.3.3 = Field::<usize>(Variant(_8, 0), 2);
_17.1 = [Field::<char>(Variant(_10, 1), 0)];
_13.3 = _16 * _16;
place!(Field::<i16>(Variant(_9, 0), 0)) = _7 & _7;
_17.3.0 = _17.3.2 as i8;
_17.3.3 = _13.1;
SetDiscriminant(_9, 1);
_18.0 = _6 >> _3;
_18 = (_11, _13.0, _17.3.3);
place!(Field::<char>(Variant(_10, 1), 0)) = '\u{fe917}';
_18.0 = !_11;
_18.0 = _13.3 as u128;
Goto(bb4)
}
bb4 = {
_7 = Field::<i16>(Variant(_8, 0), 0) >> _3;
place!(Field::<char>(Variant(_9, 1), 0)) = Field::<char>(Variant(_10, 1), 0);
_18.2 = _17.3.3;
_18.0 = !_11;
_22 = (_11,);
_19 = _13.2 as f64;
_11 = _6;
SetDiscriminant(_10, 1);
_12 = [4852301747927180943_u64,11826058359225757546_u64,8585848503682063695_u64,6474743217088259355_u64,677815792246660135_u64,2241311875414357329_u64,8099869732431304377_u64,6521169557582215884_u64];
_3 = _1 - _1;
_21 = -Field::<i128>(Variant(_8, 0), 1);
Call(_13.0 = core::intrinsics::bswap(_18.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = _13.2 as usize;
_15 = _13.3;
_7 = 50026_u16 as i16;
_21 = Field::<i128>(Variant(_8, 0), 1) ^ _5;
SetDiscriminant(_8, 0);
RET = (-75288256_i32) as usize;
_18.1 = _7 as u8;
_17.3.2 = true | false;
_17.3.0 = 43_i8;
_13.3 = _15 | _15;
_15 = -_13.3;
Goto(bb6)
}
bb6 = {
RET = !_13.1;
_2 = _3;
_17.3.3 = RET >> _13.3;
_17.3.0 = _22.0 as i8;
_11 = _6 >> _16;
place!(Field::<usize>(Variant(_8, 0), 2)) = _17.3.3 * RET;
_18.0 = _6;
_20 = Adt83::Variant1 { fld0: Field::<char>(Variant(_9, 1), 0) };
(*_14) = [Field::<char>(Variant(_9, 1), 0)];
_3 = _13.2 as i64;
_10 = Adt83::Variant0 { fld0: _7,fld1: _5,fld2: Field::<usize>(Variant(_8, 0), 2) };
place!(Field::<usize>(Variant(_10, 0), 2)) = _18.2;
_23 = [_17.3.0,_17.3.0,_17.3.0];
_19 = _21 as f64;
_1 = _3 >> _6;
SetDiscriminant(_9, 1);
_13.1 = _18.2 | _17.3.3;
_18 = (_11, _13.0, Field::<usize>(Variant(_8, 0), 2));
_18.2 = Field::<usize>(Variant(_8, 0), 2);
_17.3.3 = _17.3.2 as usize;
_18 = (_6, _13.0, Field::<usize>(Variant(_8, 0), 2));
Goto(bb7)
}
bb7 = {
_19 = _13.2 as f64;
SetDiscriminant(_10, 0);
_26 = _13.1 <= Field::<usize>(Variant(_8, 0), 2);
_9 = _20;
place!(Field::<i16>(Variant(_8, 0), 0)) = !_7;
_11 = _18.0 ^ _6;
place!(Field::<i128>(Variant(_10, 0), 1)) = Field::<i16>(Variant(_8, 0), 0) as i128;
_7 = Field::<i16>(Variant(_8, 0), 0);
_9 = Adt83::Variant0 { fld0: _7,fld1: _21,fld2: _13.1 };
_17.3.2 = _26;
SetDiscriminant(_9, 0);
_30 = _17.3.2 ^ _17.3.2;
_13.3 = !_15;
(*_14) = [Field::<char>(Variant(_20, 1), 0)];
_26 = _30 & _30;
_15 = -_13.3;
_28 = _11 as f32;
_23 = [_17.3.0,_17.3.0,_17.3.0];
_17.3.3 = !_18.2;
_31 = [Field::<i16>(Variant(_8, 0), 0),Field::<i16>(Variant(_8, 0), 0),Field::<i16>(Variant(_8, 0), 0),_7,Field::<i16>(Variant(_8, 0), 0)];
_7 = 6078199357043474733_u64 as i16;
_23 = [_17.3.0,_17.3.0,_17.3.0];
SetDiscriminant(_20, 1);
place!(Field::<usize>(Variant(_10, 0), 2)) = _18.2;
_3 = !_4;
Goto(bb8)
}
bb8 = {
_35 = _13.3 * _15;
_18.0 = !_22.0;
_22.0 = !_11;
_16 = !_35;
_33 = _26 <= _30;
_18.2 = Field::<usize>(Variant(_10, 0), 2) - _13.1;
place!(Field::<i16>(Variant(_9, 0), 0)) = -Field::<i16>(Variant(_8, 0), 0);
RET = !_13.1;
place!(Field::<usize>(Variant(_9, 0), 2)) = _18.2 * _13.1;
_24 = [_17.3.0];
place!(Field::<i128>(Variant(_10, 0), 1)) = _5;
_28 = _13.2 + _13.2;
_10 = Adt83::Variant1 { fld0: '\u{39695}' };
_11 = !_6;
_24 = [_17.3.0];
Goto(bb9)
}
bb9 = {
Call(_40 = dump_var(3_usize, 4_usize, Move(_4), 12_usize, Move(_12), 31_usize, Move(_31), 22_usize, Move(_22)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_40 = dump_var(3_usize, 33_usize, Move(_33), 18_usize, Move(_18), 21_usize, Move(_21), 16_usize, Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_40 = dump_var(3_usize, 7_usize, Move(_7), 11_usize, Move(_11), 41_usize, _41, 41_usize, _41), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [u64; 8],mut _2: (u8, usize, f32, isize),mut _3: (u8, usize, f32, isize),mut _4: i64,mut _5: (u8, usize, f32, isize),mut _6: i64,mut _7: i16,mut _8: usize) -> i16 {
mir! {
type RET = i16;
let _9: *const *const ([u16; 6], Adt35, *const &'static f32);
let _10: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _11: f32;
let _12: u32;
let _13: f32;
let _14: f64;
let _15: (u128, u8, usize);
let _16: (Adt36, [u16; 6]);
let _17: f64;
let _18: f32;
let _19: i8;
let _20: isize;
let _21: ();
let _22: ();
{
_2 = _5;
_6 = _4;
_1 = [2888687294735409415_u64,395927026787352950_u64,3952142460450342394_u64,12612734517733659141_u64,2583999408380274816_u64,165650996818784281_u64,5563977185606130746_u64,16789286700500559287_u64];
_5.1 = 56_i8 as usize;
_5 = (_3.0, _8, _3.2, _2.3);
_4 = !_6;
_2.0 = !_5.0;
_2.1 = !_5.1;
Call(_2.0 = fn5(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.3 = _5.3 ^ _2.3;
RET = _7;
_3.3 = -_2.3;
RET = _7 | _7;
_3.0 = !_2.0;
_3.1 = (-596542682_i32) as usize;
_2.2 = -_5.2;
_3.2 = -_5.2;
_5.1 = !_8;
_3 = _2;
_5.0 = _3.0;
_11 = _2.2;
_1 = [1527176821959297656_u64,2527843328226654924_u64,16151918936413850807_u64,3154680085600552154_u64,18314344872046785279_u64,7482977978947338953_u64,6125740174209598451_u64,17697827987877971152_u64];
_5.2 = _2.2;
Call(_3.0 = core::intrinsics::transmute(_2.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.2 = _3.2 * _11;
_3.2 = -_11;
_3.0 = 35961018600903526041116512446244340996_u128 as u8;
_1 = [16317028115341429022_u64,2823261952459229587_u64,1929800071660099052_u64,13859407448244900502_u64,15803389561444159051_u64,9997188126484476637_u64,12193639210910073606_u64,18274467143896120172_u64];
_5.2 = 11778969460693933408_u64 as f32;
_3.0 = !_5.0;
_12 = 1837749883_u32 >> _2.0;
_5.0 = !_2.0;
_8 = _5.1 ^ _2.1;
_5.0 = _3.0;
RET = _7 * _7;
_2.3 = 594146399_i32 as isize;
_2.1 = !_5.1;
_5.2 = -_3.2;
Call(_2.2 = core::intrinsics::transmute(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.0 = 316806051904005515135434159214100195353_u128 as u8;
RET = _7;
_13 = _5.2;
_5 = _3;
_3.0 = _2.0;
_5.1 = _8 & _2.1;
_13 = _2.2 - _2.2;
_4 = _6;
RET = -_7;
RET = '\u{f5420}' as i16;
_2.0 = _5.0 << _12;
_15.1 = _3.0;
_2.1 = _5.1;
_4 = (-1148510102_i32) as i64;
_2.1 = true as usize;
_3.2 = -_2.2;
Goto(bb4)
}
bb4 = {
_3.3 = _5.3;
_8 = !_5.1;
Goto(bb5)
}
bb5 = {
_7 = -RET;
Goto(bb6)
}
bb6 = {
_2.0 = _3.0 - _3.0;
_15.1 = _2.0;
_11 = _5.2 * _13;
_5.3 = -_3.3;
_1 = [15206749221904997656_u64,1027818943500772898_u64,16857429177485433726_u64,4473071806940354695_u64,6334141378079652483_u64,13049297550718789007_u64,13605136112440878696_u64,11662734694906033423_u64];
_1 = [7137476215266439944_u64,4626569131825024087_u64,13887996027487142811_u64,9616280465629564765_u64,18237738340698070699_u64,5110077644476915594_u64,8554121136588313825_u64,5762233667405127933_u64];
_3.3 = -_2.3;
RET = _5.3 as i16;
_15.0 = '\u{3d758}' as u128;
_15.1 = _15.0 as u8;
_5 = _2;
_17 = _12 as f64;
_15 = (80429564901546150784515988824109744494_u128, _5.0, _8);
_3.0 = _5.3 as u8;
_5.0 = _15.1;
_2 = _3;
_1 = [14346640915306507764_u64,9034285573625962902_u64,18438252878951648776_u64,6879645409477569994_u64,14112514271728232101_u64,15418155549512654637_u64,12888399748756186314_u64,7397240150448982736_u64];
_3.0 = !_5.0;
_5.1 = !_8;
_6 = _4;
_5.3 = 571343093042263992_u64 as isize;
RET = _7 >> _15.0;
_11 = _3.2;
Goto(bb7)
}
bb7 = {
Call(_21 = dump_var(4_usize, 1_usize, Move(_1), 4_usize, Move(_4), 12_usize, Move(_12), 22_usize, _22), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5() -> u8 {
mir! {
type RET = u8;
let _1: &'static Adt35;
let _2: &'static f32;
let _3: u64;
let _4: char;
let _5: &'static &'static *const u64;
let _6: i16;
let _7: f64;
let _8: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _9: u32;
let _10: [u64; 7];
let _11: f32;
let _12: bool;
let _13: [bool; 3];
let _14: f32;
let _15: (u128, u128, *const &'static i32);
let _16: bool;
let _17: [u16; 5];
let _18: u128;
let _19: f64;
let _20: u16;
let _21: (i8, &'static &'static i32, bool, usize);
let _22: u64;
let _23: ();
let _24: ();
{
RET = 114_u8 + 207_u8;
RET = 9_u8 << 1943598857_i32;
RET = true as u8;
RET = 220_u8;
RET = 148_u8 << 2025164529374890462_u64;
RET = true as u8;
RET = !91_u8;
RET = 77_u8;
RET = !53_u8;
RET = 175_u8 >> 298988504_u32;
RET = 22_u8;
RET = 137_u8;
RET = !49_u8;
RET = (-116408465911290693869809667051789200394_i128) as u8;
RET = !78_u8;
RET = !194_u8;
RET = !209_u8;
RET = !96_u8;
RET = !46_u8;
RET = (-903534611_i32) as u8;
RET = 31_u8;
RET = 6_u8 | 242_u8;
RET = 32_u8;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
32 => bb5,
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
RET = 133403410_i32 as u8;
RET = 66_u8 << 69_i8;
RET = !46_u8;
RET = !248_u8;
RET = 51762_u16 as u8;
RET = 3_u8;
RET = 100_u8 | 132_u8;
RET = !100_u8;
RET = !226_u8;
_4 = '\u{fc285}';
_4 = '\u{75d79}';
RET = !102_u8;
Call(_4 = fn6(RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = 62_u8;
RET = 45_u8 | 51_u8;
_6 = 18712_i16;
_6 = !(-8327_i16);
RET = 208_u8 + 113_u8;
_7 = 2026451287744894529_u64 as f64;
_4 = '\u{7b603}';
RET = _4 as u8;
_4 = '\u{1085cf}';
_6 = (-21777_i16);
_6 = 17508_i16 << RET;
_3 = (-9223372036854775808_isize) as u64;
_8.1 = [_4];
_8.3.3 = 1603247474128409840_usize;
RET = (-106_i8) as u8;
_6 = (-55_i8) as i16;
match _8.3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1603247474128409840 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_6 = 110216914328517003276778432183787796522_u128 as i16;
_11 = 686326958_u32 as f32;
_8.0 = 3593859062_u32;
_8.3.3 = 17458873808467818036_usize;
RET = 236_u8 - 93_u8;
_4 = '\u{bd8b7}';
_8.3.2 = false;
_8.3.0 = (-71_isize) as i8;
_8.3.3 = _7 as usize;
_2 = &_14;
_13 = [_8.3.2,_8.3.2,_8.3.2];
_8.1 = [_4];
_16 = _8.3.2;
_7 = 12916_u16 as f64;
_11 = _8.3.3 as f32;
_4 = '\u{aa1ba}';
_3 = !11233693895811035090_u64;
_15.1 = !191917147077586101605007455444896209527_u128;
_8.3.3 = _7 as usize;
_3 = !6752621728023324648_u64;
_13 = [_16,_8.3.2,_8.3.2];
_9 = (-7075619478621221872_i64) as u32;
_13 = [_8.3.2,_16,_8.3.2];
Goto(bb9)
}
bb9 = {
_9 = !_8.0;
_2 = &(*_2);
_2 = &_14;
_15.0 = !_15.1;
_15.1 = (-3787983664102516959_i64) as u128;
_2 = &(*_2);
RET = 208_u8;
match RET {
0 => bb4,
1 => bb6,
208 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
RET = !76_u8;
_16 = _8.3.2;
_15.0 = (-15888825079880565822343384855030968268_i128) as u128;
Goto(bb12)
}
bb12 = {
_11 = _3 as f32;
_11 = _3 as f32;
_16 = _8.3.2;
_14 = -_11;
_8.3.3 = 8962069147108146875_usize - 2188486809100152895_usize;
_21.2 = !_16;
RET = 231_u8 + 229_u8;
_2 = &_14;
_20 = _6 as u16;
RET = (-45052180367724886536085533492371132670_i128) as u8;
_8.3.3 = !7_usize;
_16 = _8.3.2 ^ _8.3.2;
_8.3.2 = _21.2;
_22 = !_3;
_21.2 = _8.3.2;
_19 = -_7;
_10 = [_22,_22,_22,_3,_3,_22,_22];
_8.1 = [_4];
Goto(bb13)
}
bb13 = {
_17 = [_20,_20,_20,_20,_20];
_13 = [_21.2,_21.2,_8.3.2];
_14 = -_11;
_19 = _7 + _7;
_2 = &_14;
RET = 240_u8 << _3;
_22 = !_3;
match _8.0 {
0 => bb6,
1 => bb12,
2 => bb3,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
3593859062 => bb19,
_ => bb18
}
}
bb14 = {
RET = 133403410_i32 as u8;
RET = 66_u8 << 69_i8;
RET = !46_u8;
RET = !248_u8;
RET = 51762_u16 as u8;
RET = 3_u8;
RET = 100_u8 | 132_u8;
RET = !100_u8;
RET = !226_u8;
_4 = '\u{fc285}';
_4 = '\u{75d79}';
RET = !102_u8;
Call(_4 = fn6(RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
RET = !76_u8;
_16 = _8.3.2;
_15.0 = (-15888825079880565822343384855030968268_i128) as u128;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_6 = 110216914328517003276778432183787796522_u128 as i16;
_11 = 686326958_u32 as f32;
_8.0 = 3593859062_u32;
_8.3.3 = 17458873808467818036_usize;
RET = 236_u8 - 93_u8;
_4 = '\u{bd8b7}';
_8.3.2 = false;
_8.3.0 = (-71_isize) as i8;
_8.3.3 = _7 as usize;
_2 = &_14;
_13 = [_8.3.2,_8.3.2,_8.3.2];
_8.1 = [_4];
_16 = _8.3.2;
_7 = 12916_u16 as f64;
_11 = _8.3.3 as f32;
_4 = '\u{aa1ba}';
_3 = !11233693895811035090_u64;
_15.1 = !191917147077586101605007455444896209527_u128;
_8.3.3 = _7 as usize;
_3 = !6752621728023324648_u64;
_13 = [_16,_8.3.2,_8.3.2];
_9 = (-7075619478621221872_i64) as u32;
_13 = [_8.3.2,_16,_8.3.2];
Goto(bb9)
}
bb19 = {
_15.1 = _15.0 + _15.0;
_6 = (-21813_i16) - 24554_i16;
_8.1 = [_4];
_18 = _15.0;
_21.0 = _8.3.0;
Goto(bb20)
}
bb20 = {
Call(_23 = dump_var(5_usize, 22_usize, Move(_22), 9_usize, Move(_9), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_23 = dump_var(5_usize, 6_usize, Move(_6), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u8,mut _2: u8,mut _3: u8,mut _4: u8,mut _5: u8,mut _6: u8,mut _7: u8,mut _8: u8,mut _9: u8) -> char {
mir! {
type RET = char;
let _10: i8;
let _11: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _12: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _13: bool;
let _14: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _15: &'static char;
let _16: (u8,);
let _17: *const (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _18: usize;
let _19: [u16; 5];
let _20: [u64; 7];
let _21: i32;
let _22: f64;
let _23: *mut (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _24: isize;
let _25: u8;
let _26: i128;
let _27: bool;
let _28: (i8, &'static &'static i32, bool, usize);
let _29: [i8; 3];
let _30: [u16; 6];
let _31: &'static Adt26;
let _32: u8;
let _33: bool;
let _34: usize;
let _35: Adt75;
let _36: f32;
let _37: Adt26;
let _38: bool;
let _39: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _40: u8;
let _41: f64;
let _42: usize;
let _43: ([u16; 6], Adt35, *const &'static f32);
let _44: i128;
let _45: i8;
let _46: [u64; 8];
let _47: &'static &'static char;
let _48: (u8,);
let _49: isize;
let _50: f32;
let _51: f32;
let _52: Adt36;
let _53: u8;
let _54: f32;
let _55: *const i128;
let _56: char;
let _57: *const i64;
let _58: f32;
let _59: f32;
let _60: [i64; 8];
let _61: [u64; 7];
let _62: (Adt36, [u16; 6]);
let _63: *const (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _64: ();
let _65: ();
{
_8 = _4;
_4 = !_8;
_2 = 7873893702630253395_i64 as u8;
_4 = 8567033519049782206_u64 as u8;
RET = '\u{4d07b}';
RET = '\u{c133f}';
_1 = !_4;
_9 = !_7;
_1 = 15979751097677950931_u64 as u8;
RET = '\u{a57d7}';
_5 = _2 + _2;
_11.3.3 = 1_usize;
_11.3.2 = !false;
_5 = !_4;
_11.3.3 = 4_usize ^ 10109758256885066182_usize;
_6 = 9223372036854775807_isize as u8;
_11.3.0 = (-40_i8);
_11.3.2 = false;
Goto(bb1)
}
bb1 = {
_12.1.0 = _1;
_12.1.2 = 964692114_i32 as f32;
_13 = _11.3.2;
_12.3 = _13 ^ _11.3.2;
_11.3.3 = 18377172995592141031_usize;
_12.1.0 = _12.1.2 as u8;
_1 = !_6;
_12.1.1 = _11.3.3;
_12.0 = _11.3.2 as isize;
_1 = _4 + _4;
_4 = _12.0 as u8;
_12.2 = &_12.1.2;
_4 = !_6;
_11.0 = 1867689469_u32;
_12.1.1 = _11.3.3 ^ _11.3.3;
_13 = _5 > _3;
_8 = _6;
_12.1.0 = 11569540297909166436_u64 as u8;
RET = '\u{4e162}';
_12.1.3 = !_12.0;
_12.0 = _2 as isize;
_10 = _11.3.0 ^ _11.3.0;
Call(_12.1.3 = fn7(Move(_12.2), _1, _11.3.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = !_12.1.0;
_12.2 = &_12.1.2;
_12.1.0 = _1 * _1;
_15 = &RET;
_16 = (_4,);
_11.3.0 = _10;
_8 = !_6;
_12.1.2 = (-13208836986724620269197991138298972116_i128) as f32;
_11.1 = [(*_15)];
_12.2 = &_12.1.2;
_12.1.0 = _9 & _16.0;
_11.3.0 = !_10;
_11.0 = RET as u32;
_12.1.3 = _12.0 >> _11.3.3;
_15 = &(*_15);
_6 = !_9;
_12.1.1 = _11.3.3;
_5 = _6 * _3;
_7 = !_5;
_17 = core::ptr::addr_of!(_11);
_8 = _7 + _9;
_1 = 23780_i16 as u8;
RET = '\u{78b9b}';
(*_17).0 = RET as u32;
_18 = (*_17).3.3;
_12.1.2 = _11.3.0 as f32;
Call(_20 = fn17(Move(_17), _11.3.2, (*_17).3.3, _12.1.2, _12.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _1;
RET = '\u{89512}';
_13 = _12.3;
_12.2 = &_12.1.2;
_21 = (-153683529_i32);
_16.0 = _6;
_20 = [1260398401257904452_u64,7844850231202273068_u64,1725916937906500694_u64,7801568394559008930_u64,15703570595144762551_u64,16972153798129806107_u64,8187128344121195710_u64];
_11.3.3 = 30696_u16 as usize;
_10 = _11.3.0 * _11.3.0;
RET = '\u{79fb2}';
_9 = _4;
_17 = core::ptr::addr_of!(_11);
(*_17).3.0 = _10;
_19 = [43397_u16,60068_u16,30282_u16,52565_u16,20878_u16];
(*_17).0 = 1719947261_u32 | 654373810_u32;
(*_17).0 = !3459713223_u32;
_7 = _5;
RET = '\u{6c40}';
_11.3.2 = _13;
_11.3.3 = _12.1.1;
_7 = _8 - _6;
_22 = (*_17).3.3 as f64;
_11.3.0 = -_10;
_23 = core::ptr::addr_of_mut!((*_17));
RET = '\u{2230a}';
_11.3.3 = _18 ^ _18;
_15 = &RET;
Call(_16.0 = core::intrinsics::bswap(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_17).3.2 = !_12.3;
_12.1.3 = (*_23).0 as isize;
(*_17).3.3 = !_18;
_9 = !_7;
_18 = (*_23).3.3 - _11.3.3;
(*_23).1 = [(*_15)];
_11.3.2 = !_12.3;
_20 = [6244335254723574983_u64,18000989040155964587_u64,15399429328935375282_u64,9726662227173365593_u64,995785928823766647_u64,14798800207525647595_u64,15032372266908833048_u64];
(*_23).3.0 = _10;
_28.0 = -(*_17).3.0;
(*_23).3.0 = _28.0 * _28.0;
_26 = 85357673643124715976400770204961065340_i128;
match _26 {
85357673643124715976400770204961065340 => bb5,
_ => bb2
}
}
bb5 = {
_12.1.3 = (*_17).3.0 as isize;
(*_17).3.3 = _5 as usize;
_27 = (*_23).3.2 | (*_17).3.2;
_29 = [_28.0,_11.3.0,(*_23).3.0];
(*_23).0 = _26 as u32;
_11.3.0 = (*_17).3.3 as i8;
(*_23).1 = [RET];
_1 = _5;
(*_17).3.3 = _12.1.1 >> _12.0;
_3 = !_12.1.0;
_28.2 = _11.3.2;
_26 = 3926347756767401657782312589442849345_i128 >> _12.1.3;
(*_23).0 = 3705131228_u32;
_22 = _21 as f64;
_2 = _9;
_18 = _11.3.3;
(*_17).0 = !920347326_u32;
_26 = 20947480931857773296246980285135284711_i128;
Goto(bb6)
}
bb6 = {
_28.3 = !(*_23).3.3;
(*_23).3.3 = !_18;
_12.1.1 = (*_17).3.3;
_13 = !(*_17).3.2;
_30 = [61155_u16,59350_u16,8102_u16,53236_u16,62989_u16,33523_u16];
Call((*_23).3.3 = core::intrinsics::transmute(_12.1.3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = &RET;
(*_23).1 = [RET];
_24 = (*_15) as isize;
(*_17).1 = [(*_15)];
(*_23).1 = [(*_15)];
_4 = _6;
(*_17).3.3 = !_28.3;
_4 = !_12.1.0;
_33 = _12.3 & _28.2;
RET = '\u{12ad1}';
(*_17).3.3 = !_12.1.1;
(*_17).3.0 = -_28.0;
_20 = [9426544591561069329_u64,5710260299803905285_u64,2608331066626386419_u64,12332297714003782345_u64,1876990856996152830_u64,11190642549937254875_u64,2508817932629997664_u64];
_17 = core::ptr::addr_of!((*_17));
_36 = _12.1.2;
_1 = _9;
(*_17).3.0 = -_28.0;
(*_17).0 = !1191297081_u32;
_39.0 = _12.1.3;
Goto(bb8)
}
bb8 = {
_21 = !105271542_i32;
_12.1.0 = !_6;
_12.1.1 = !_28.3;
(*_23).1 = [RET];
(*_17).0 = (*_23).3.3 as u32;
_39.0 = _12.1.3 << (*_17).3.0;
_23 = core::ptr::addr_of_mut!((*_17));
_34 = 23928_i16 as usize;
_17 = core::ptr::addr_of!((*_17));
(*_23).0 = 709187211_u32 + 3468017124_u32;
_12.1 = (_9, (*_23).3.3, _36, _39.0);
(*_23).3.2 = !_33;
(*_23).3.0 = _10 >> _8;
_8 = _9 | _16.0;
_39.1.2 = _36;
_39.1.1 = (*_17).3.3;
Call(_11.0 = fn18(Move(_17), Move(_23), (*_17).3.2, _1, _39.0, _39.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = _9;
_15 = &RET;
_47 = &_15;
_23 = core::ptr::addr_of_mut!(_11);
_41 = -_22;
(*_23).3.2 = _13;
_45 = _11.3.0;
_28.0 = (*_23).3.0 | _45;
_25 = !_1;
_39.3 = _28.0 > _28.0;
(*_23).0 = 1202539021_u32;
_40 = !_8;
(*_23).3.0 = _10;
_35 = Adt75::Variant1 { fld0: (-7450309811667816128_i64) };
_1 = _12.1.0 * _9;
_39.1 = (_2, _34, _12.1.2, _12.1.3);
_32 = (*_23).0 as u8;
_5 = !_25;
_26 = !43536992634414531648490629908606534297_i128;
_12.1 = (_16.0, _11.3.3, _36, _12.0);
Goto(bb10)
}
bb10 = {
_11.3.0 = _45;
_20 = [5312955169652721979_u64,16615782410685638793_u64,9874548849047122573_u64,13142617783006758028_u64,3674153838109102299_u64,11742286264939147942_u64,17233638803102213718_u64];
_48 = (_1,);
_12.1.2 = -_39.1.2;
_44 = _26;
_38 = _39.3;
_26 = _12.1.2 as i128;
_10 = !_11.3.0;
_22 = _41;
_8 = _48.0 | _25;
_40 = !_4;
_30 = [14686_u16,28480_u16,48522_u16,31620_u16,12126_u16,36189_u16];
place!(Field::<i64>(Variant(_35, 1), 0)) = 4551325416621734848_i64;
_43.0 = _30;
_11.3.0 = _34 as i8;
_16 = _48;
_48 = (_5,);
SetDiscriminant(_35, 0);
Call(place!(Field::<(u8, usize, f32, isize)>(Variant(_35, 0), 2)).0 = fn19(Move(_47), Move(_23), _16.0, _39.0, _39.1.3, _20, _11.1, _12.1.0, _38, (*_23).3.3, _30, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<[u16; 6]>(Variant(_35, 0), 1)) = _43.0;
_12.1.0 = _11.0 as u8;
_7 = _6;
_21 = (-108686401_i32) * (-1310696207_i32);
place!(Field::<Adt69>(Variant(_35, 0), 3)) = Adt69 { fld0: 35245_u16,fld1: _11.1 };
_48.0 = !_8;
_24 = _39.0;
place!(Field::<u64>(Variant(_35, 0), 7)) = _28.0 as u64;
_33 = !_39.3;
_55 = core::ptr::addr_of!(_44);
_33 = _39.3;
_37 = Adt26::Variant0 { fld0: _39.3,fld1: _36,fld2: _22,fld3: _11.3.0,fld4: Field::<u64>(Variant(_35, 0), 7),fld5: _21,fld6: Move(_55) };
place!(Field::<*const Adt36>(Variant(_35, 0), 6)) = core::ptr::addr_of!(_52);
_42 = _18;
place!(Field::<(u8, usize, f32, isize)>(Variant(_35, 0), 2)) = (_2, _34, _36, _12.0);
_12.0 = _39.1.3;
_43.1 = Adt35::Variant3 { fld0: _39.3,fld1: _16.0,fld2: _39.0,fld3: _18,fld4: _11.1,fld5: Field::<i32>(Variant(_37, 0), 5) };
_12.1.3 = !_12.0;
Goto(bb12)
}
bb12 = {
_34 = _18;
place!(Field::<u32>(Variant(_35, 0), 4)) = _11.0;
place!(Field::<(u8, usize, f32, isize)>(Variant(_35, 0), 2)).1 = _34;
_22 = Field::<f64>(Variant(_37, 0), 2);
_25 = _4;
_31 = &_37;
place!(Field::<(u8, usize, f32, isize)>(Variant(_35, 0), 2)).2 = Field::<f32>(Variant((*_31), 0), 1) - Field::<f32>(Variant((*_31), 0), 1);
_55 = Move(Field::<*const i128>(Variant((*_31), 0), 6));
_49 = !_39.1.3;
_5 = _6 | _8;
_50 = 333878185399132811185270322278723274416_u128 as f32;
place!(Field::<Adt69>(Variant(_35, 0), 3)).fld1 = [(*_15)];
match Field::<u32>(Variant(_35, 0), 4) {
1202539021 => bb14,
_ => bb13
}
}
bb13 = {
_12.1.3 = (*_17).3.0 as isize;
(*_17).3.3 = _5 as usize;
_27 = (*_23).3.2 | (*_17).3.2;
_29 = [_28.0,_11.3.0,(*_23).3.0];
(*_23).0 = _26 as u32;
_11.3.0 = (*_17).3.3 as i8;
(*_23).1 = [RET];
_1 = _5;
(*_17).3.3 = _12.1.1 >> _12.0;
_3 = !_12.1.0;
_28.2 = _11.3.2;
_26 = 3926347756767401657782312589442849345_i128 >> _12.1.3;
(*_23).0 = 3705131228_u32;
_22 = _21 as f64;
_2 = _9;
_18 = _11.3.3;
(*_17).0 = !920347326_u32;
_26 = 20947480931857773296246980285135284711_i128;
Goto(bb6)
}
bb14 = {
place!(Field::<bool>(Variant(_35, 0), 0)) = Field::<bool>(Variant((*_31), 0), 0);
_32 = _49 as u8;
_12.2 = &_54;
_16.0 = !_48.0;
_51 = Field::<f64>(Variant((*_31), 0), 2) as f32;
_41 = Field::<f64>(Variant(_37, 0), 2);
_37 = Adt26::Variant0 { fld0: _38,fld1: Field::<(u8, usize, f32, isize)>(Variant(_35, 0), 2).2,fld2: _22,fld3: _45,fld4: Field::<u64>(Variant(_35, 0), 7),fld5: Field::<i32>(Variant(_43.1, 3), 5),fld6: Move(_55) };
Goto(bb15)
}
bb15 = {
Call(_64 = dump_var(6_usize, 6_usize, Move(_6), 5_usize, Move(_5), 38_usize, Move(_38), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_64 = dump_var(6_usize, 18_usize, Move(_18), 33_usize, Move(_33), 45_usize, Move(_45), 40_usize, Move(_40)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_64 = dump_var(6_usize, 16_usize, Move(_16), 8_usize, Move(_8), 27_usize, Move(_27), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_64 = dump_var(6_usize, 32_usize, Move(_32), 20_usize, Move(_20), 7_usize, Move(_7), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: &'static f32,mut _2: u8,mut _3: usize) -> isize {
mir! {
type RET = isize;
let _4: i16;
let _5: f64;
let _6: &'static char;
let _7: char;
let _8: f32;
let _9: Adt69;
let _10: *const *const ([u16; 6], Adt35, *const &'static f32);
let _11: Adt35;
let _12: &'static Adt36;
let _13: isize;
let _14: isize;
let _15: *const Adt26;
let _16: (isize, (u8, usize, f32, isize), &'static Adt26);
let _17: [i8; 1];
let _18: [char; 1];
let _19: f32;
let _20: (u8, usize, f32, isize);
let _21: isize;
let _22: (u128,);
let _23: [i16; 5];
let _24: [i16; 5];
let _25: *const ([u16; 6], Adt35, *const &'static f32);
let _26: *mut u16;
let _27: (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _28: usize;
let _29: i128;
let _30: ();
let _31: ();
{
_3 = (-1738181827_i32) as usize;
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = (-76_isize) ^ 9223372036854775807_isize;
RET = (-9223372036854775808_isize) * (-46_isize);
_4 = false as i16;
RET = 35_isize;
_4 = 17150_i16 | (-293_i16);
_3 = 17048477941507305444_usize ^ 3_usize;
_5 = (-121709581_i32) as f64;
_5 = 15284_u16 as f64;
_5 = (-26_i8) as f64;
_2 = 225_u8 + 59_u8;
_4 = 8181_i16;
_5 = _4 as f64;
_3 = 1892910315404905792_usize ^ 5_usize;
_5 = (-3675104197913951691_i64) as f64;
RET = (-48_isize) & 9223372036854775807_isize;
_5 = 270102991784074229475349318095346499153_u128 as f64;
_5 = (-100_i8) as f64;
_3 = 7_usize >> _2;
_2 = 253_u8 << _3;
_2 = 102_u8;
_4 = (-31039_i16) | (-779_i16);
_2 = !253_u8;
_5 = _3 as f64;
RET = (-9223372036854775808_isize) - (-81_isize);
RET = _5 as isize;
Call(_4 = fn8(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !142_u8;
_3 = 48475684095450573759953164701027990653_u128 as usize;
_3 = 3_usize + 14921912073315209410_usize;
RET = (-9223372036854775808_isize) << _3;
RET = (-9223372036854775808_isize);
_4 = (-5163280637349561436_i64) as i16;
_7 = '\u{8f604}';
_6 = &_7;
RET = (-9223372036854775808_isize);
_7 = '\u{1004f2}';
_9.fld1 = [_7];
_6 = &_7;
_9.fld0 = 44438_u16;
_5 = _3 as f64;
_9.fld1 = [_7];
RET = 16074450528384435773_u64 as isize;
Goto(bb2)
}
bb2 = {
_6 = &(*_6);
_3 = (-28566818479069661586574783393461444476_i128) as usize;
_13 = RET;
_1 = &_8;
_7 = '\u{b339c}';
_4 = 1437700491_u32 as i16;
_8 = 672030726991198811_u64 as f32;
_1 = &_8;
_6 = &_7;
RET = _13;
RET = _13;
match _9.fld0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
44438 => bb8,
_ => bb7
}
}
bb3 = {
_2 = !142_u8;
_3 = 48475684095450573759953164701027990653_u128 as usize;
_3 = 3_usize + 14921912073315209410_usize;
RET = (-9223372036854775808_isize) << _3;
RET = (-9223372036854775808_isize);
_4 = (-5163280637349561436_i64) as i16;
_7 = '\u{8f604}';
_6 = &_7;
RET = (-9223372036854775808_isize);
_7 = '\u{1004f2}';
_9.fld1 = [_7];
_6 = &_7;
_9.fld0 = 44438_u16;
_5 = _3 as f64;
_9.fld1 = [_7];
RET = 16074450528384435773_u64 as isize;
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
RET = _13 * _13;
RET = _13;
RET = _4 as isize;
_1 = &(*_1);
_11 = Adt35::Variant0 { fld0: (-2216874013920849163_i64) };
_14 = RET - RET;
_5 = _4 as f64;
_5 = 86_i8 as f64;
_11 = Adt35::Variant0 { fld0: (-1783759115245581971_i64) };
_2 = 188_u8;
_6 = &(*_6);
_5 = _8 as f64;
_13 = RET;
_1 = &(*_1);
_9.fld1 = [(*_6)];
place!(Field::<i64>(Variant(_11, 0), 0)) = 990163645879150897_i64 + (-185731499496672419_i64);
_1 = &(*_1);
RET = !_14;
_6 = &(*_6);
_3 = 8154196386265369859_usize;
_9.fld1 = [_7];
_7 = '\u{31b77}';
RET = _14 | _14;
_14 = RET + _13;
_9.fld1 = [_7];
_1 = &(*_1);
_16.0 = RET;
_16.1.0 = _2;
Goto(bb9)
}
bb9 = {
RET = _14 - _16.0;
_2 = _16.1.0;
_2 = _16.1.0;
_16.1.3 = RET * RET;
RET = 134611923728821081786220930536039585904_i128 as isize;
match _16.1.0 {
0 => bb5,
1 => bb2,
2 => bb10,
188 => bb12,
_ => bb11
}
}
bb10 = {
RET = _13 * _13;
RET = _13;
RET = _4 as isize;
_1 = &(*_1);
_11 = Adt35::Variant0 { fld0: (-2216874013920849163_i64) };
_14 = RET - RET;
_5 = _4 as f64;
_5 = 86_i8 as f64;
_11 = Adt35::Variant0 { fld0: (-1783759115245581971_i64) };
_2 = 188_u8;
_6 = &(*_6);
_5 = _8 as f64;
_13 = RET;
_1 = &(*_1);
_9.fld1 = [(*_6)];
place!(Field::<i64>(Variant(_11, 0), 0)) = 990163645879150897_i64 + (-185731499496672419_i64);
_1 = &(*_1);
RET = !_14;
_6 = &(*_6);
_3 = 8154196386265369859_usize;
_9.fld1 = [_7];
_7 = '\u{31b77}';
RET = _14 | _14;
_14 = RET + _13;
_9.fld1 = [_7];
_1 = &(*_1);
_16.0 = RET;
_16.1.0 = _2;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
_16.1.1 = _3;
_16.1.2 = -(*_1);
_1 = &_8;
_6 = &_7;
_18 = [(*_6)];
_16.1.3 = _14;
_14 = !_13;
_6 = &(*_6);
_1 = &_19;
_19 = _9.fld0 as f32;
_20.3 = _16.0 + _16.0;
_5 = 2239048099_u32 as f64;
SetDiscriminant(_11, 1);
_16.0 = _16.1.3;
_21 = _20.3 >> _20.3;
_20.1 = _16.0 as usize;
_16.1.1 = _20.1 | _3;
_5 = (-36_i8) as f64;
_6 = &(*_6);
_20.3 = -_16.0;
_21 = _4 as isize;
_20.0 = _2;
Goto(bb13)
}
bb13 = {
_9 = Adt69 { fld0: 36891_u16,fld1: _18 };
_1 = &_20.2;
_22.0 = 59627315122560902319138000661747753803_u128;
RET = _5 as isize;
_9.fld1 = [_7];
_16.1 = (_20.0, _20.1, _8, _16.0);
_11 = Adt35::Variant3 { fld0: true,fld1: _16.1.0,fld2: _21,fld3: _16.1.1,fld4: _18,fld5: 663302906_i32 };
_16.0 = _20.3;
place!(Field::<bool>(Variant(_11, 3), 0)) = false;
_20.1 = Field::<usize>(Variant(_11, 3), 3);
_9 = Adt69 { fld0: 16037_u16,fld1: _18 };
_17 = [(-65_i8)];
place!(Field::<usize>(Variant(_11, 3), 3)) = _16.1.1;
_23 = [_4,_4,_4,_4,_4];
_17 = [115_i8];
_5 = _20.3 as f64;
_16.1 = (_2, _20.1, _8, _20.3);
RET = _20.3 << Field::<isize>(Variant(_11, 3), 2);
_20.0 = !_2;
_20 = (_16.1.0, _3, _8, _14);
Goto(bb14)
}
bb14 = {
place!(Field::<[char; 1]>(Variant(_11, 3), 4)) = _9.fld1;
place!(Field::<bool>(Variant(_11, 3), 0)) = true & true;
place!(Field::<usize>(Variant(_11, 3), 3)) = _22.0 as usize;
_24 = [_4,_4,_4,_4,_4];
_9 = Adt69 { fld0: 25371_u16,fld1: Field::<[char; 1]>(Variant(_11, 3), 4) };
_16.1.2 = _19 * _8;
_10 = core::ptr::addr_of!(_25);
_9 = Adt69 { fld0: 40406_u16,fld1: Field::<[char; 1]>(Variant(_11, 3), 4) };
_4 = _16.1.1 as i16;
_5 = (-1006662944_i32) as f64;
_3 = !_16.1.1;
_8 = _19;
_20.1 = !_16.1.1;
_9.fld1 = [(*_6)];
_27.3.0 = (-28_i8);
place!(Field::<isize>(Variant(_11, 3), 2)) = _16.1.3;
place!(Field::<usize>(Variant(_11, 3), 3)) = _3 << _16.1.3;
_20.2 = (-59346538709631763366081803762216062276_i128) as f32;
_11 = Adt35::Variant1 { fld0: _20.1 };
_13 = !_16.1.3;
_28 = !Field::<usize>(Variant(_11, 1), 0);
_6 = &(*_6);
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(7_usize, 7_usize, Move(_7), 18_usize, Move(_18), 24_usize, Move(_24), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(7_usize, 23_usize, Move(_23), 22_usize, Move(_22), 31_usize, _31, 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8() -> i16 {
mir! {
type RET = i16;
let _1: Adt36;
let _2: Adt65;
let _3: (isize, (u8, usize, f32, isize), &'static Adt26);
let _4: Adt26;
let _5: isize;
let _6: f64;
let _7: (i8, &'static &'static i32, bool, usize);
let _8: *mut (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _9: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _10: u8;
let _11: ([u16; 6], Adt35, *const &'static f32);
let _12: &'static &'static *const u64;
let _13: isize;
let _14: u32;
let _15: [i64; 8];
let _16: &'static (u128, u128, *const &'static i32);
let _17: isize;
let _18: (u8,);
let _19: bool;
let _20: isize;
let _21: &'static &'static Adt26;
let _22: (Adt36, [u16; 6]);
let _23: *const ([u16; 6], Adt35, *const &'static f32);
let _24: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _25: &'static (u128, u128, *const &'static i32);
let _26: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _27: f64;
let _28: (isize, (u8, usize, f32, isize), &'static Adt26);
let _29: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _30: char;
let _31: *const &'static *const &'static f32;
let _32: f32;
let _33: (Adt36, [u16; 6]);
let _34: ();
let _35: ();
{
RET = 9749907040224767208_usize as i16;
RET = !(-6663_i16);
RET = -6642_i16;
RET = !(-25181_i16);
RET = 306_i16 & 14592_i16;
RET = 217_u8 as i16;
RET = 22365_i16 << 1399920522_u32;
RET = 653_i16;
RET = -6006_i16;
RET = !(-16683_i16);
RET = 30191_u16 as i16;
RET = (-132429217_i32) as i16;
RET = 14039_i16;
RET = '\u{c78bf}' as i16;
_2.fld0 = 291525262960803306474294182223177184540_u128 as u8;
_2.fld0 = !251_u8;
_2.fld0 = 31903726267439499753560969755344363254_u128 as u8;
RET = !(-3792_i16);
RET = (-17244_i16) * (-29916_i16);
RET = -(-25811_i16);
_2.fld0 = !95_u8;
_2 = Adt65 { fld0: 177_u8 };
Call(RET = core::intrinsics::bswap((-7443_i16)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.1.0 = RET as u8;
_3.1.2 = 846419955368144785_i64 as f32;
_3.1.3 = 566374419_u32 as isize;
_3.0 = _3.1.3 * _3.1.3;
_3.1.2 = 268181840705469758608141586848445752203_u128 as f32;
_3.0 = -_3.1.3;
_2 = Adt65 { fld0: _3.1.0 };
_3.0 = !_3.1.3;
_3.0 = -_3.1.3;
_3.1.0 = !_2.fld0;
_5 = _3.1.3;
_2 = Adt65 { fld0: _3.1.0 };
RET = !30418_i16;
_3.1.0 = !_2.fld0;
_2 = Adt65 { fld0: _3.1.0 };
RET = -(-1296_i16);
_2 = Adt65 { fld0: _3.1.0 };
_7.0 = !102_i8;
_7.2 = true ^ true;
Call(RET = fn9(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.3 = 7_usize;
_3.1.2 = 1294843449180082775_i64 as f32;
_3.1.2 = RET as f32;
_2 = Adt65 { fld0: _3.1.0 };
_3.2 = &_4;
_3.2 = &_4;
_3.2 = &_4;
_7.3 = _7.0 as usize;
_3.1.0 = 169485078009409236550530794146166294152_u128 as u8;
RET = 9909_i16 ^ (-11262_i16);
_6 = 1032131537720763824_u64 as f64;
_2 = Adt65 { fld0: _3.1.0 };
_6 = 1614390031_i32 as f64;
_3.1.1 = !_7.3;
RET = 7585655085609751959_u64 as i16;
_3.1.2 = _3.0 as f32;
_2 = Adt65 { fld0: _3.1.0 };
_3.1.2 = 3342443430814227070_u64 as f32;
_6 = _3.0 as f64;
Call(_3.1 = fn15(_7.2, _7.2, _3.0, _5, _2.fld0, _7.2, _7.3, _3.0, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7.0 = !68_i8;
RET = (-18466_i16);
_2.fld0 = _3.1.0 >> RET;
_3.1.2 = RET as f32;
_3.2 = &_4;
_2.fld0 = _3.1.0 ^ _3.1.0;
_1 = Adt36::Variant2 { fld0: '\u{ac7a2}' };
_3.1.0 = !_2.fld0;
_3.1.3 = _3.0;
match RET {
0 => bb4,
1 => bb5,
340282366920938463463374607431768192990 => bb7,
_ => bb6
}
}
bb4 = {
_7.3 = 7_usize;
_3.1.2 = 1294843449180082775_i64 as f32;
_3.1.2 = RET as f32;
_2 = Adt65 { fld0: _3.1.0 };
_3.2 = &_4;
_3.2 = &_4;
_3.2 = &_4;
_7.3 = _7.0 as usize;
_3.1.0 = 169485078009409236550530794146166294152_u128 as u8;
RET = 9909_i16 ^ (-11262_i16);
_6 = 1032131537720763824_u64 as f64;
_2 = Adt65 { fld0: _3.1.0 };
_6 = 1614390031_i32 as f64;
_3.1.1 = !_7.3;
RET = 7585655085609751959_u64 as i16;
_3.1.2 = _3.0 as f32;
_2 = Adt65 { fld0: _3.1.0 };
_3.1.2 = 3342443430814227070_u64 as f32;
_6 = _3.0 as f64;
Call(_3.1 = fn15(_7.2, _7.2, _3.0, _5, _2.fld0, _7.2, _7.3, _3.0, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3.1.0 = RET as u8;
_3.1.2 = 846419955368144785_i64 as f32;
_3.1.3 = 566374419_u32 as isize;
_3.0 = _3.1.3 * _3.1.3;
_3.1.2 = 268181840705469758608141586848445752203_u128 as f32;
_3.0 = -_3.1.3;
_2 = Adt65 { fld0: _3.1.0 };
_3.0 = !_3.1.3;
_3.0 = -_3.1.3;
_3.1.0 = !_2.fld0;
_5 = _3.1.3;
_2 = Adt65 { fld0: _3.1.0 };
RET = !30418_i16;
_3.1.0 = !_2.fld0;
_2 = Adt65 { fld0: _3.1.0 };
RET = -(-1296_i16);
_2 = Adt65 { fld0: _3.1.0 };
_7.0 = !102_i8;
_7.2 = true ^ true;
Call(RET = fn9(), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_7.3 = !_3.1.1;
RET = !28454_i16;
_1 = Adt36::Variant0 { fld0: RET,fld1: _2.fld0 };
_6 = 61442_u16 as f64;
_7.0 = 90_i8;
_3.1.2 = 139044323288139914073526993389509136616_i128 as f32;
_11.0 = [30655_u16,48192_u16,39463_u16,46372_u16,50786_u16,26761_u16];
_3.1.1 = _7.3;
_11.0 = [55104_u16,60806_u16,40862_u16,12387_u16,24228_u16,18129_u16];
_3.1.0 = Field::<u8>(Variant(_1, 0), 1);
_10 = _3.1.1 as u8;
SetDiscriminant(_1, 1);
place!(Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1)).0 = _3.1.0 | _10;
_3.1.0 = Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).0 + Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).0;
_3.0 = _3.1.3;
place!(Field::<u16>(Variant(_1, 1), 2)) = 13525_u16 ^ 54966_u16;
place!(Field::<i8>(Variant(_1, 1), 3)) = _7.0 ^ _7.0;
_3.0 = _6 as isize;
_7.0 = -Field::<i8>(Variant(_1, 1), 3);
place!(Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1)) = (_2.fld0, _3.1.1, _3.1.2, _5);
Goto(bb8)
}
bb8 = {
_3.2 = &_4;
_11.0 = [Field::<u16>(Variant(_1, 1), 2),Field::<u16>(Variant(_1, 1), 2),Field::<u16>(Variant(_1, 1), 2),Field::<u16>(Variant(_1, 1), 2),Field::<u16>(Variant(_1, 1), 2),Field::<u16>(Variant(_1, 1), 2)];
place!(Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1)) = _3.1;
_15 = [(-3741062427288193562_i64),3555655662601199024_i64,5592686491993309962_i64,138774513279809870_i64,(-7539478940748987391_i64),1750588654410270803_i64,2051158748397622816_i64,(-2826922716390318221_i64)];
place!(Field::<u8>(Variant(_1, 1), 5)) = !_3.1.0;
_13 = _3.0 * Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).3;
place!(Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1)).2 = Field::<i8>(Variant(_1, 1), 3) as f32;
_11.1 = Adt35::Variant1 { fld0: Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).1 };
place!(Field::<i8>(Variant(_1, 1), 3)) = _7.0;
_5 = _13;
_3.1 = (Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).0, Field::<usize>(Variant(_11.1, 1), 0), Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).2, _13);
_3.1.0 = _3.1.3 as u8;
SetDiscriminant(_11.1, 1);
_10 = !Field::<(u8, usize, f32, isize)>(Variant(_1, 1), 1).0;
_3.1.1 = _7.3;
_1 = Adt36::Variant0 { fld0: RET,fld1: _2.fld0 };
_3.1.0 = _10;
_7.2 = false;
_17 = -_13;
_2 = Adt65 { fld0: _3.1.0 };
_14 = 3930106535_u32 ^ 206696580_u32;
_6 = 634007608801409749_u64 as f64;
RET = Field::<i16>(Variant(_1, 0), 0);
place!(Field::<u8>(Variant(_1, 0), 1)) = !_2.fld0;
Goto(bb9)
}
bb9 = {
_18 = (_2.fld0,);
SetDiscriminant(_1, 2);
Goto(bb10)
}
bb10 = {
_19 = _7.2 & _7.2;
_7.2 = _19;
_18.0 = _10;
_15 = [8856020653893475810_i64,4696707072905932329_i64,(-6170674122650959677_i64),(-5429616251345734412_i64),654333968037824168_i64,2517002325679049243_i64,2940186439333015318_i64,5036184201942734091_i64];
RET = !(-26307_i16);
_7.2 = _13 == _3.1.3;
place!(Field::<usize>(Variant(_11.1, 1), 0)) = _7.3 << _10;
_20 = _3.1.0 as isize;
_3.1.2 = RET as f32;
_3.0 = _13 << _3.1.0;
_3.0 = _17;
place!(Field::<usize>(Variant(_11.1, 1), 0)) = (-658070706_i32) as usize;
_3.2 = &_4;
_1 = Adt36::Variant2 { fld0: '\u{3e9e0}' };
place!(Field::<char>(Variant(_1, 2), 0)) = '\u{386f1}';
_3.1.1 = _18.0 as usize;
Goto(bb11)
}
bb11 = {
_7.2 = _19;
_22 = (Move(_1), _11.0);
_1 = Adt36::Variant0 { fld0: RET,fld1: _18.0 };
_19 = _7.2;
_19 = _7.2;
_5 = _3.1.1 as isize;
Goto(bb12)
}
bb12 = {
_24.3 = _7.2;
_3.2 = &_4;
_15 = [3931575197648151030_i64,7526882979427580388_i64,(-5415601197134985689_i64),(-407874801517719294_i64),(-3817723174402868608_i64),930532727608130092_i64,2019036029958055124_i64,5459017279300056431_i64];
_17 = _20;
_24.2 = &_24.1.2;
_18.0 = _2.fld0 | _2.fld0;
_21 = &_3.2;
_24.1.3 = _5;
_3.2 = &_4;
_3.1.1 = _7.3 | _7.3;
_24.2 = &_24.1.2;
_3.2 = &_4;
_3.1.0 = !_18.0;
_28.2 = &_4;
_19 = _5 == _24.1.3;
_3.2 = &_4;
Goto(bb13)
}
bb13 = {
_10 = (-993531836780716618_i64) as u8;
_24.0 = _6 as isize;
_21 = &_28.2;
_28.1 = (_3.1.0, _3.1.1, _3.1.2, _20);
_20 = -_5;
_3.2 = &_4;
_10 = !_18.0;
_18.0 = Field::<i16>(Variant(_1, 0), 0) as u8;
_20 = _28.1.2 as isize;
_28.1.2 = _3.1.2;
Call(_28.1.2 = fn16(_3.1, Field::<u8>(Variant(_1, 0), 1), _5), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_29.1.0 = _6 as u8;
_29.3 = !_19;
_23 = core::ptr::addr_of!(_11);
(*_23).0 = [10713_u16,58372_u16,4601_u16,64383_u16,19142_u16,792_u16];
_3.1.3 = _28.1.3;
_20 = _3.1.3 >> RET;
_7.0 = 55_i8;
_11.1 = Adt35::Variant1 { fld0: _7.3 };
_28.0 = _28.1.3 | _20;
place!(Field::<usize>(Variant((*_23).1, 1), 0)) = _3.1.1 - _3.1.1;
_19 = _29.3;
_11.0 = [62058_u16,63744_u16,2372_u16,55794_u16,35023_u16,26304_u16];
_28.1.2 = -_3.1.2;
_28.1.0 = _10;
_3.2 = &_4;
_24.1 = (_10, _3.1.1, _28.1.2, _28.0);
_29.1.2 = _24.1.2;
place!(Field::<char>(Variant(_22.0, 2), 0)) = '\u{8e5ed}';
_29.2 = &_3.1.2;
place!(Field::<usize>(Variant((*_23).1, 1), 0)) = _19 as usize;
_3.1 = (_28.1.0, _24.1.1, _29.1.2, _24.1.3);
RET = Field::<i16>(Variant(_1, 0), 0) >> _3.0;
_30 = Field::<char>(Variant(_22.0, 2), 0);
place!(Field::<i16>(Variant(_1, 0), 0)) = RET | RET;
_28.1.2 = -_29.1.2;
RET = !Field::<i16>(Variant(_1, 0), 0);
_11.0 = [45726_u16,3179_u16,33586_u16,5720_u16,43649_u16,39618_u16];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(8_usize, 14_usize, Move(_14), 30_usize, Move(_30), 10_usize, Move(_10), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(8_usize, 19_usize, Move(_19), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9() -> i16 {
mir! {
type RET = i16;
let _1: [u16; 5];
let _2: i8;
let _3: &'static (u128, u128, *const &'static i32);
let _4: f32;
let _5: isize;
let _6: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _7: u32;
let _8: f32;
let _9: char;
let _10: u16;
let _11: isize;
let _12: &'static &'static *const u64;
let _13: i128;
let _14: i16;
let _15: &'static *const &'static f32;
let _16: u16;
let _17: f64;
let _18: u64;
let _19: isize;
let _20: [u16; 5];
let _21: f32;
let _22: [char; 1];
let _23: bool;
let _24: Adt83;
let _25: Adt28;
let _26: f64;
let _27: *mut u16;
let _28: &'static f32;
let _29: [u16; 5];
let _30: &'static &'static char;
let _31: ();
let _32: ();
{
RET = 4656_i16;
RET = 32140_i16;
RET = 5327_i16 - (-5964_i16);
RET = !(-7403_i16);
RET = 220432577821256940_u64 as i16;
RET = (-5275_i16) * 11048_i16;
RET = 56701130905598674661872603597232835854_u128 as i16;
RET = 21298_i16;
RET = (-26245_i16) & 508_i16;
RET = 32307_i16;
Goto(bb1)
}
bb1 = {
RET = 25774_i16;
RET = !(-3807_i16);
RET = 7608_i16 & (-23790_i16);
RET = 1461480729_u32 as i16;
RET = !(-23549_i16);
RET = (-28128_i16) & (-13713_i16);
RET = true as i16;
RET = (-31980_i16) ^ (-13189_i16);
RET = -(-2038_i16);
RET = (-5401_i16) * (-4966_i16);
RET = '\u{10db5d}' as i16;
RET = 12791_i16 + 31077_i16;
Goto(bb2)
}
bb2 = {
_1 = [57886_u16,14494_u16,45362_u16,36225_u16,21583_u16];
_1 = [20102_u16,26717_u16,51148_u16,20133_u16,40849_u16];
_2 = !24_i8;
_1 = [26164_u16,35_u16,49299_u16,40185_u16,52490_u16];
_1 = [26238_u16,26112_u16,28797_u16,57888_u16,38958_u16];
_2 = -18_i8;
RET = 286655669493953721_usize as i16;
_4 = _2 as f32;
_2 = 95_i8;
_4 = _2 as f32;
RET = (-16440_i16) - 31881_i16;
_4 = 964713556571252490_i64 as f32;
_2 = !117_i8;
_1 = [20841_u16,9274_u16,6021_u16,29880_u16,27137_u16];
_4 = (-98420564926822889218031126797944082154_i128) as f32;
_6.3 = !true;
_6.1.0 = (-1115186194_i32) as u8;
_8 = RET as f32;
_2 = 32_i8;
Call(_5 = fn10(_6.1.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = 4052527179_u32 as i8;
_6.1.0 = 152_u8 >> _2;
_2 = 96_i8;
_4 = -_8;
_6.1 = (168_u8, 5_usize, _4, _5);
_6.1.0 = 84_u8;
_7 = 1355046358_u32;
_6.1.3 = _5 ^ _5;
_9 = '\u{f5c18}';
_10 = 110590116210719489677685455606434054773_u128 as u16;
_11 = -_6.1.3;
RET = 1111_i16 ^ (-30293_i16);
match _6.1.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb11,
_ => bb10
}
}
bb4 = {
_1 = [57886_u16,14494_u16,45362_u16,36225_u16,21583_u16];
_1 = [20102_u16,26717_u16,51148_u16,20133_u16,40849_u16];
_2 = !24_i8;
_1 = [26164_u16,35_u16,49299_u16,40185_u16,52490_u16];
_1 = [26238_u16,26112_u16,28797_u16,57888_u16,38958_u16];
_2 = -18_i8;
RET = 286655669493953721_usize as i16;
_4 = _2 as f32;
_2 = 95_i8;
_4 = _2 as f32;
RET = (-16440_i16) - 31881_i16;
_4 = 964713556571252490_i64 as f32;
_2 = !117_i8;
_1 = [20841_u16,9274_u16,6021_u16,29880_u16,27137_u16];
_4 = (-98420564926822889218031126797944082154_i128) as f32;
_6.3 = !true;
_6.1.0 = (-1115186194_i32) as u8;
_8 = RET as f32;
_2 = 32_i8;
Call(_5 = fn10(_6.1.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = 25774_i16;
RET = !(-3807_i16);
RET = 7608_i16 & (-23790_i16);
RET = 1461480729_u32 as i16;
RET = !(-23549_i16);
RET = (-28128_i16) & (-13713_i16);
RET = true as i16;
RET = (-31980_i16) ^ (-13189_i16);
RET = -(-2038_i16);
RET = (-5401_i16) * (-4966_i16);
RET = '\u{10db5d}' as i16;
RET = 12791_i16 + 31077_i16;
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
Return()
}
bb11 = {
_10 = 37808_u16 * 60619_u16;
_2 = 84_i8;
_6.2 = &_6.1.2;
_6.1.3 = -_11;
_6.1.2 = _6.1.1 as f32;
_1 = [_10,_10,_10,_10,_10];
_6.2 = &_6.1.2;
_10 = 813_u16;
_5 = _11 << _6.1.3;
_8 = _4 + _6.1.2;
_6.1.2 = _8 + _8;
_14 = RET << _11;
_6.0 = _5 & _6.1.3;
_6.0 = _5 >> _10;
_6.2 = &_6.1.2;
_4 = -_6.1.2;
_9 = '\u{aa020}';
RET = _14;
_6.1.2 = -_4;
Call(_6.1.3 = fn11(_6.1.1, RET, _6.1.1, _8, _6.1.1, _6.1.2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = '\u{e47ba}';
_16 = _10;
_10 = 37092187476509553740551250492411109177_i128 as u16;
_18 = !822823640465945673_u64;
_9 = '\u{8e6df}';
_20 = [_10,_16,_10,_16,_10];
Goto(bb13)
}
bb13 = {
Goto(bb14)
}
bb14 = {
_18 = 3228598822575904716_u64;
_13 = 85714354117460550419640330299838728502_i128;
_6.2 = &_6.1.2;
_21 = _6.1.2;
_16 = !_10;
_5 = _6.0 + _6.0;
_13 = 107546690729693531928359484141326830530_i128;
_17 = 5366043219739529233_i64 as f64;
_9 = '\u{64096}';
_1 = [_16,_10,_16,_10,_10];
_4 = _18 as f32;
_6.0 = _5;
RET = _9 as i16;
_21 = _6.1.2;
_7 = !199191905_u32;
_23 = _6.1.2 > _6.1.2;
_28 = &_6.1.2;
_7 = 646339207_u32 & 2267732144_u32;
_10 = !_16;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(9_usize, 16_usize, Move(_16), 14_usize, Move(_14), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(9_usize, 11_usize, Move(_11), 13_usize, Move(_13), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u8) -> isize {
mir! {
type RET = isize;
let _2: [i8; 3];
let _3: bool;
let _4: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _5: Adt75;
let _6: char;
let _7: f64;
let _8: char;
let _9: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _10: [u64; 7];
let _11: usize;
let _12: *const (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _13: [u16; 6];
let _14: f32;
let _15: i32;
let _16: isize;
let _17: u16;
let _18: isize;
let _19: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _20: i128;
let _21: f64;
let _22: [u64; 7];
let _23: char;
let _24: bool;
let _25: u128;
let _26: (u8,);
let _27: &'static &'static *const u64;
let _28: *const (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _29: [i64; 8];
let _30: &'static &'static Adt26;
let _31: &'static Adt26;
let _32: bool;
let _33: &'static &'static i32;
let _34: Adt83;
let _35: f32;
let _36: *const i128;
let _37: f32;
let _38: [u16; 5];
let _39: isize;
let _40: u16;
let _41: f32;
let _42: *const i128;
let _43: &'static &'static char;
let _44: bool;
let _45: (isize, (u8, usize, f32, isize), &'static Adt26);
let _46: *const *const ([u16; 6], Adt35, *const &'static f32);
let _47: f32;
let _48: f32;
let _49: u32;
let _50: u128;
let _51: ();
let _52: ();
{
RET = !9223372036854775807_isize;
RET = 59_i8 as isize;
_2 = [24_i8,(-15_i8),(-99_i8)];
RET = (-9223372036854775808_isize);
_3 = false & true;
Goto(bb1)
}
bb1 = {
_1 = 26557_i16 as u8;
RET = (-9223372036854775808_isize);
_1 = !226_u8;
_4.2 = &_4.1.2;
_4.0 = -RET;
_4.3 = _3;
_4.1.2 = _1 as f32;
_4.0 = -RET;
_4.3 = _3;
_4.1.1 = 4070842472_u32 as usize;
_2 = [98_i8,66_i8,(-10_i8)];
RET = _4.1.1 as isize;
_4.2 = &_4.1.2;
_4.1.0 = _4.1.2 as u8;
_1 = _4.1.0 ^ _4.1.0;
_6 = '\u{1030e6}';
_4.0 = RET << _1;
_6 = '\u{9106}';
_4.2 = &_4.1.2;
_4.0 = -RET;
Call(_4.0 = core::intrinsics::bswap(RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.1.1 = 7_usize;
_4.1.0 = _1;
RET = 2843_i16 as isize;
_1 = _4.1.0 ^ _4.1.0;
_6 = '\u{729c6}';
_5 = Adt75::Variant1 { fld0: 133057273692524472_i64 };
_7 = 125714778736278830866581574193286227050_i128 as f64;
_4.1.2 = _4.1.0 as f32;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: (-4268530552869367953_i64) };
RET = (-6358924011698506818345219130846059699_i128) as isize;
_4.2 = &_4.1.2;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: 3785646822057082640_i64 };
_5 = Adt75::Variant1 { fld0: 742955212868376286_i64 };
place!(Field::<i64>(Variant(_5, 1), 0)) = 974887457_u32 as i64;
RET = -_4.0;
_4.1.0 = 319828579169074760973372484540408040929_u128 as u8;
_8 = _6;
place!(Field::<i64>(Variant(_5, 1), 0)) = 1392176445352884676_i64;
_4.0 = RET | RET;
_11 = _4.0 as usize;
Goto(bb3)
}
bb3 = {
_8 = _6;
_10 = [14565613711089888007_u64,6677411716499168484_u64,3152249520542443199_u64,8143518787380158225_u64,1316167028099614132_u64,13217843071997859713_u64,14648677372927851781_u64];
_3 = _4.3;
RET = !_4.0;
RET = -_4.0;
_7 = 10085825514434132617_u64 as f64;
_7 = _11 as f64;
Goto(bb4)
}
bb4 = {
_4.0 = RET;
_4.3 = _3;
_4.3 = !_3;
_7 = _1 as f64;
_4.1.1 = _11;
_1 = _4.1.1 as u8;
_1 = _4.1.0;
_14 = -_4.1.2;
_16 = !RET;
SetDiscriminant(_5, 0);
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_1, _11, _14, _4.0);
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).0 = _4.1.0 << _4.0;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: 2010269027946398175_i64 };
_10 = [10212309878467595914_u64,16632066701347908662_u64,17001056936812828825_u64,17915893573531483352_u64,13843498971754233693_u64,18100896052527703479_u64,6899234399273106294_u64];
_13 = [29589_u16,14158_u16,34393_u16,35471_u16,56550_u16,48768_u16];
_8 = _6;
_5 = Adt75::Variant1 { fld0: 8518663110428777470_i64 };
_19.1.1 = _11 + _4.1.1;
place!(Field::<i64>(Variant(_5, 1), 0)) = 7555806518562059456_i64 ^ 8592879965972021206_i64;
_18 = 9277811384424934467179281409405185656_u128 as isize;
_19.0 = _4.0;
_19.1.0 = 3781950057_u32 as u8;
Call(RET = core::intrinsics::transmute(_4.1.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19.3 = !_4.3;
_14 = -_4.1.2;
_4.1 = (_1, _19.1.1, _14, _16);
_4.1.3 = _16 & _18;
_19.1.2 = _14;
_16 = _4.3 as isize;
_16 = _19.0;
_11 = !_4.1.1;
_19.1.1 = !_4.1.1;
_13 = [39436_u16,282_u16,56108_u16,13870_u16,22841_u16,4392_u16];
_17 = 47580_u16;
SetDiscriminant(_5, 0);
_24 = _4.1.2 < _14;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).0 = _19.1.0;
_13 = [_17,_17,_17,_17,_17,_17];
place!(Field::<bool>(Variant(_5, 0), 0)) = _19.3;
Goto(bb6)
}
bb6 = {
place!(Field::<u64>(Variant(_5, 0), 7)) = 17594273562581793121_u64;
_20 = _11 as i128;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).2 = _19.1.2;
_6 = _8;
RET = _4.3 as isize;
_19.2 = &_19.1.2;
place!(Field::<bool>(Variant(_5, 0), 0)) = !_24;
_4.3 = Field::<bool>(Variant(_5, 0), 0);
_19.1.1 = _14 as usize;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).1 = !_11;
_14 = _4.1.2;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_1, _11, _19.1.2, _16);
_22 = [Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7)];
_21 = _7;
_15 = (-82689196_i32) << _17;
_10 = _22;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).3 = Field::<u64>(Variant(_5, 0), 7) as isize;
_25 = 7391714672500918569518043779192073223_u128;
place!(Field::<*const i128>(Variant(_5, 0), 5)) = core::ptr::addr_of!(_20);
Goto(bb7)
}
bb7 = {
_2 = [(-79_i8),90_i8,(-118_i8)];
_23 = _6;
_4.2 = &_4.1.2;
_24 = !_19.3;
_26.0 = !_1;
_18 = -Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).3;
_17 = !61392_u16;
_4.1.2 = -_19.1.2;
_16 = _4.1.3;
_4 = (_16, Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2), Move(_19.2), _19.3);
_19.2 = &_14;
_10 = [Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7)];
_4.1 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2);
_7 = _21;
_13 = [_17,_17,_17,_17,_17,_17];
_24 = _4.3;
_30 = &_31;
_29 = [(-8503084855579325813_i64),(-207651649959657248_i64),6258997851074179091_i64,(-8612940874526485319_i64),(-7785899299155489808_i64),5799233886684781842_i64,914750828832400994_i64,(-2991060218114134388_i64)];
_4.2 = Move(_19.2);
_1 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).0 << _18;
place!(Field::<Adt69>(Variant(_5, 0), 3)).fld0 = _17;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_4.1.0, _19.1.1, _4.1.2, _4.1.3);
Goto(bb8)
}
bb8 = {
_4.0 = _4.1.2 as isize;
_37 = -_4.1.2;
_13 = [_17,Field::<Adt69>(Variant(_5, 0), 3).fld0,_17,_17,_17,Field::<Adt69>(Variant(_5, 0), 3).fld0];
_24 = _4.3;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_19.1.0, _11, _19.1.2, _16);
_20 = !17634726328765409755922238502118169416_i128;
_4.0 = (-55_i8) as isize;
_32 = _24;
_10 = [Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7)];
_4.0 = RET ^ Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).3;
_39 = _19.1.0 as isize;
_6 = _23;
_19.1.3 = !_39;
_19.1 = (_26.0, _4.1.1, _37, _16);
_24 = _4.3;
_2 = [(-80_i8),(-63_i8),40_i8];
_37 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).2 * _14;
_3 = !_24;
RET = _16;
_2 = [58_i8,(-8_i8),66_i8];
_15 = (-672889803_i32) * (-1020478260_i32);
_26.0 = _19.1.0;
_8 = _23;
match Field::<u64>(Variant(_5, 0), 7) {
0 => bb1,
1 => bb7,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
17594273562581793121 => bb14,
_ => bb13
}
}
bb9 = {
_2 = [(-79_i8),90_i8,(-118_i8)];
_23 = _6;
_4.2 = &_4.1.2;
_24 = !_19.3;
_26.0 = !_1;
_18 = -Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).3;
_17 = !61392_u16;
_4.1.2 = -_19.1.2;
_16 = _4.1.3;
_4 = (_16, Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2), Move(_19.2), _19.3);
_19.2 = &_14;
_10 = [Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7)];
_4.1 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2);
_7 = _21;
_13 = [_17,_17,_17,_17,_17,_17];
_24 = _4.3;
_30 = &_31;
_29 = [(-8503084855579325813_i64),(-207651649959657248_i64),6258997851074179091_i64,(-8612940874526485319_i64),(-7785899299155489808_i64),5799233886684781842_i64,914750828832400994_i64,(-2991060218114134388_i64)];
_4.2 = Move(_19.2);
_1 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).0 << _18;
place!(Field::<Adt69>(Variant(_5, 0), 3)).fld0 = _17;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_4.1.0, _19.1.1, _4.1.2, _4.1.3);
Goto(bb8)
}
bb10 = {
_4.1.1 = 7_usize;
_4.1.0 = _1;
RET = 2843_i16 as isize;
_1 = _4.1.0 ^ _4.1.0;
_6 = '\u{729c6}';
_5 = Adt75::Variant1 { fld0: 133057273692524472_i64 };
_7 = 125714778736278830866581574193286227050_i128 as f64;
_4.1.2 = _4.1.0 as f32;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: (-4268530552869367953_i64) };
RET = (-6358924011698506818345219130846059699_i128) as isize;
_4.2 = &_4.1.2;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: 3785646822057082640_i64 };
_5 = Adt75::Variant1 { fld0: 742955212868376286_i64 };
place!(Field::<i64>(Variant(_5, 1), 0)) = 974887457_u32 as i64;
RET = -_4.0;
_4.1.0 = 319828579169074760973372484540408040929_u128 as u8;
_8 = _6;
place!(Field::<i64>(Variant(_5, 1), 0)) = 1392176445352884676_i64;
_4.0 = RET | RET;
_11 = _4.0 as usize;
Goto(bb3)
}
bb11 = {
_19.3 = !_4.3;
_14 = -_4.1.2;
_4.1 = (_1, _19.1.1, _14, _16);
_4.1.3 = _16 & _18;
_19.1.2 = _14;
_16 = _4.3 as isize;
_16 = _19.0;
_11 = !_4.1.1;
_19.1.1 = !_4.1.1;
_13 = [39436_u16,282_u16,56108_u16,13870_u16,22841_u16,4392_u16];
_17 = 47580_u16;
SetDiscriminant(_5, 0);
_24 = _4.1.2 < _14;
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).0 = _19.1.0;
_13 = [_17,_17,_17,_17,_17,_17];
place!(Field::<bool>(Variant(_5, 0), 0)) = _19.3;
Goto(bb6)
}
bb12 = {
_4.0 = RET;
_4.3 = _3;
_4.3 = !_3;
_7 = _1 as f64;
_4.1.1 = _11;
_1 = _4.1.1 as u8;
_1 = _4.1.0;
_14 = -_4.1.2;
_16 = !RET;
SetDiscriminant(_5, 0);
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)) = (_1, _11, _14, _4.0);
place!(Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2)).0 = _4.1.0 << _4.0;
_4.3 = !_3;
_5 = Adt75::Variant1 { fld0: 2010269027946398175_i64 };
_10 = [10212309878467595914_u64,16632066701347908662_u64,17001056936812828825_u64,17915893573531483352_u64,13843498971754233693_u64,18100896052527703479_u64,6899234399273106294_u64];
_13 = [29589_u16,14158_u16,34393_u16,35471_u16,56550_u16,48768_u16];
_8 = _6;
_5 = Adt75::Variant1 { fld0: 8518663110428777470_i64 };
_19.1.1 = _11 + _4.1.1;
place!(Field::<i64>(Variant(_5, 1), 0)) = 7555806518562059456_i64 ^ 8592879965972021206_i64;
_18 = 9277811384424934467179281409405185656_u128 as isize;
_19.0 = _4.0;
_19.1.0 = 3781950057_u32 as u8;
Call(RET = core::intrinsics::transmute(_4.1.1), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_1 = 26557_i16 as u8;
RET = (-9223372036854775808_isize);
_1 = !226_u8;
_4.2 = &_4.1.2;
_4.0 = -RET;
_4.3 = _3;
_4.1.2 = _1 as f32;
_4.0 = -RET;
_4.3 = _3;
_4.1.1 = 4070842472_u32 as usize;
_2 = [98_i8,66_i8,(-10_i8)];
RET = _4.1.1 as isize;
_4.2 = &_4.1.2;
_4.1.0 = _4.1.2 as u8;
_1 = _4.1.0 ^ _4.1.0;
_6 = '\u{1030e6}';
_4.0 = RET << _1;
_6 = '\u{9106}';
_4.2 = &_4.1.2;
_4.0 = -RET;
Call(_4.0 = core::intrinsics::bswap(RET), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_10 = [Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7),Field::<u64>(Variant(_5, 0), 7)];
_4.1 = _19.1;
_41 = _37 + _4.1.2;
_19.3 = !_4.3;
place!(Field::<Adt69>(Variant(_5, 0), 3)).fld1 = [_23];
_19.3 = _4.3;
place!(Field::<u32>(Variant(_5, 0), 4)) = 1149751580_u32;
place!(Field::<bool>(Variant(_5, 0), 0)) = !_24;
_26.0 = _4.1.0;
_26 = (_1,);
_39 = _25 as isize;
_4.1.1 = Field::<(u8, usize, f32, isize)>(Variant(_5, 0), 2).1;
_19.3 = !_3;
place!(Field::<u32>(Variant(_5, 0), 4)) = 2635610290_u32;
_5 = Adt75::Variant1 { fld0: (-4092295747682569077_i64) };
_7 = _21 - _21;
_24 = _3 | _4.3;
_4.2 = &_45.1.2;
_24 = _4.1.3 != _4.1.3;
_2 = [120_i8,38_i8,(-12_i8)];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(10_usize, 1_usize, Move(_1), 11_usize, Move(_11), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(10_usize, 17_usize, Move(_17), 3_usize, Move(_3), 22_usize, Move(_22), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(10_usize, 13_usize, Move(_13), 16_usize, Move(_16), 52_usize, _52, 52_usize, _52), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: usize,mut _2: i16,mut _3: usize,mut _4: f32,mut _5: usize,mut _6: f32) -> isize {
mir! {
type RET = isize;
let _7: u16;
let _8: [i8; 3];
let _9: Adt35;
let _10: [i8; 3];
let _11: isize;
let _12: [i8; 3];
let _13: &'static &'static i32;
let _14: f64;
let _15: &'static *const &'static f32;
let _16: u32;
let _17: isize;
let _18: u64;
let _19: char;
let _20: &'static i32;
let _21: (u8,);
let _22: (u128,);
let _23: [i64; 5];
let _24: Adt83;
let _25: isize;
let _26: f64;
let _27: *const i64;
let _28: Adt26;
let _29: [u64; 7];
let _30: char;
let _31: [u64; 8];
let _32: (u8, usize, f32, isize);
let _33: isize;
let _34: u128;
let _35: ((i8, &'static &'static i32, bool, usize), &'static &'static i32, (u128, u128, *const &'static i32), &'static &'static Adt26);
let _36: u32;
let _37: [i64; 5];
let _38: isize;
let _39: ();
let _40: ();
{
_6 = _4;
RET = !(-9223372036854775808_isize);
_1 = _3 << _5;
_2 = !30051_i16;
_7 = '\u{2705b}' as u16;
_1 = _5 >> RET;
_2 = (-15539_i16);
_6 = _4 + _4;
RET = (-90_isize);
_1 = _3 + _3;
RET = (-114_isize);
_3 = !_5;
_2 = (-9556_i16);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211342 => bb7,
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
_5 = !_3;
_1 = !_3;
_2 = (-3729_i16) - (-11553_i16);
RET = 135_u8 as isize;
_7 = 18965_u16 >> _5;
RET = -28_isize;
RET = _3 as isize;
Goto(bb8)
}
bb8 = {
_5 = _3 + _3;
RET = 9223372036854775807_isize;
_10 = [(-28_i8),83_i8,102_i8];
_3 = 55778935690836127262375893247219769945_u128 as usize;
_8 = [(-92_i8),73_i8,23_i8];
_2 = _7 as i16;
_10 = [(-51_i8),112_i8,76_i8];
_10 = [(-127_i8),6_i8,(-49_i8)];
_8 = _10;
_2 = (-16428_i16) - 7494_i16;
_3 = _5;
_1 = !_5;
_7 = 33475_u16;
_1 = _3;
_6 = _4 - _4;
_5 = _3;
_1 = _2 as usize;
_3 = '\u{5ded4}' as usize;
_5 = _1;
Call(_8 = fn12(_7, _4, _6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11 = -RET;
_4 = _6 + _6;
_11 = -RET;
_5 = !_1;
_6 = -_4;
RET = _11;
_8 = [25_i8,83_i8,(-85_i8)];
_7 = false as u16;
_9 = Adt35::Variant1 { fld0: _1 };
_5 = !_1;
place!(Field::<usize>(Variant(_9, 1), 0)) = !_5;
Goto(bb10)
}
bb10 = {
_12 = [(-51_i8),(-7_i8),20_i8];
_11 = RET | RET;
_12 = [(-51_i8),10_i8,35_i8];
RET = _11 ^ _11;
_16 = !4071140742_u32;
_17 = RET;
_11 = RET >> _5;
RET = _17;
_2 = (-15656_i16);
_11 = -_17;
_19 = '\u{10d39a}';
_10 = _12;
_6 = Field::<usize>(Variant(_9, 1), 0) as f32;
_16 = !2251474263_u32;
_19 = '\u{749ef}';
_14 = RET as f64;
_17 = 104465359722803883034230510740889627927_u128 as isize;
_10 = _8;
_12 = [(-18_i8),80_i8,112_i8];
_22 = (297153106085942091236431838299174090649_u128,);
place!(Field::<usize>(Variant(_9, 1), 0)) = !_5;
_22.0 = 124429254155987531114429251928756264111_u128 << _16;
match _2 {
0 => bb9,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768195800 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
RET = _11 - _11;
_21.0 = _7 as u8;
_3 = RET as usize;
_9 = Adt35::Variant1 { fld0: _3 };
_21 = (65_u8,);
_14 = 44306269_i32 as f64;
_2 = 19705_i16 | 22689_i16;
_21 = (216_u8,);
_23 = [(-6123976678147589433_i64),(-2234256112422890489_i64),7078164672537829775_i64,4565766354664459414_i64,2809008400993318399_i64];
_18 = 9333245762533205260_u64 << Field::<usize>(Variant(_9, 1), 0);
_12 = _10;
_8 = [(-125_i8),(-83_i8),(-120_i8)];
_21.0 = 118_u8 + 204_u8;
_6 = _4 + _4;
RET = _14 as isize;
_6 = _4;
_8 = _10;
_24 = Adt83::Variant0 { fld0: _2,fld1: 29272114977279995636030525069229517595_i128,fld2: Field::<usize>(Variant(_9, 1), 0) };
_26 = _14;
_6 = _4 + _4;
_4 = _6;
_1 = !Field::<usize>(Variant(_9, 1), 0);
_24 = Adt83::Variant1 { fld0: _19 };
_12 = _8;
Goto(bb13)
}
bb13 = {
_10 = _8;
_4 = _6;
RET = !_11;
_14 = 8655550053443050595_i64 as f64;
SetDiscriminant(_24, 1);
_23 = [1629179072335004690_i64,(-4001664593091713886_i64),(-6198359028317209941_i64),950994870935257562_i64,(-450047024811529230_i64)];
_13 = &_20;
_21 = (164_u8,);
_7 = !35515_u16;
_14 = _26;
_21 = (18_u8,);
_21.0 = (-1976783751_i32) as u8;
_8 = _12;
_19 = '\u{7edaa}';
_29 = [_18,_18,_18,_18,_18,_18,_18];
_9 = Adt35::Variant1 { fld0: _1 };
_8 = [112_i8,(-81_i8),(-100_i8)];
_18 = !11542863086889920482_u64;
_18 = 7902756898119186511_u64 << Field::<usize>(Variant(_9, 1), 0);
place!(Field::<char>(Variant(_24, 1), 0)) = _19;
_25 = RET;
Call(_6 = core::intrinsics::transmute(_16), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = (183201230996699652947269371921446304752_u128,);
_32 = (_21.0, _3, _4, _25);
_23 = [(-2666307777808446520_i64),(-8570786179259718788_i64),6526653093735304144_i64,(-5855698631169083410_i64),8324448500462893259_i64];
_22.0 = _32.1 as u128;
_21.0 = _7 as u8;
_5 = !_3;
_25 = 63_i8 as isize;
_26 = -_14;
_33 = _17;
_11 = !RET;
SetDiscriminant(_9, 2);
RET = -_11;
_32.1 = !_1;
place!(Field::<usize>(Variant(_9, 2), 0)) = _22.0 as usize;
place!(Field::<(u8, usize, f32, isize)>(Variant(_9, 2), 2)).3 = RET;
_1 = _32.1;
_35.2.2 = core::ptr::addr_of!((*_13));
_32.2 = _4 - _4;
SetDiscriminant(_24, 0);
_35.0.2 = true;
_17 = _26 as isize;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(11_usize, 25_usize, Move(_25), 8_usize, Move(_8), 11_usize, Move(_11), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(11_usize, 17_usize, Move(_17), 22_usize, Move(_22), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(11_usize, 12_usize, Move(_12), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u16,mut _2: f32,mut _3: f32) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _4: isize;
let _5: (u8,);
let _6: isize;
let _7: char;
let _8: isize;
let _9: (u128, u8, usize);
let _10: *const &'static i32;
let _11: i16;
let _12: Adt28;
let _13: f32;
let _14: [i8; 3];
let _15: (isize, (u8, usize, f32, isize), &'static f32, bool);
let _16: [u16; 5];
let _17: char;
let _18: isize;
let _19: (Adt36, [u16; 6]);
let _20: bool;
let _21: f64;
let _22: u64;
let _23: (u8,);
let _24: ();
let _25: ();
{
RET = [23_i8,(-108_i8),62_i8];
RET = [(-116_i8),32_i8,124_i8];
Call(_5 = fn13(_2, _3, _2, _1, RET, RET, _2, _3, _1, _3, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb2 = {
_2 = -_3;
_7 = '\u{ae13}';
_2 = -_3;
_5.0 = !233_u8;
_5 = (181_u8,);
_9.2 = !4_usize;
RET = [(-125_i8),29_i8,(-18_i8)];
_9.0 = !312412405082239629780310527858944291362_u128;
_9.1 = _5.0;
_4 = 47781424595889528660799512088941227542_i128 as isize;
_9.1 = _5.0;
_3 = -_2;
_1 = !64185_u16;
_7 = '\u{7b24f}';
_3 = (-153176456576384617502720369230378176517_i128) as f32;
RET = [(-105_i8),85_i8,97_i8];
_2 = _3 * _3;
_9.2 = !14162528260087424420_usize;
_6 = _4 * _8;
match _9.1 {
0 => bb3,
1 => bb4,
181 => bb6,
_ => bb5
}
}
bb3 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET = [58_i8,87_i8,51_i8];
RET = [7_i8,40_i8,45_i8];
_4 = -_6;
_11 = 2539_i16;
_9 = (222044009225338647519284513376811045753_u128, _5.0, 7_usize);
RET = [(-73_i8),(-84_i8),(-7_i8)];
_8 = (-8632140440936518069_i64) as isize;
_9.1 = _5.0 * _5.0;
_9.0 = 659780813_u32 as u128;
RET = [(-13_i8),100_i8,(-90_i8)];
_11 = false as i16;
RET = [42_i8,55_i8,69_i8];
_13 = -_2;
_14 = [70_i8,92_i8,(-36_i8)];
_2 = _11 as f32;
RET = [85_i8,(-73_i8),(-57_i8)];
RET = [126_i8,(-17_i8),(-7_i8)];
_8 = _6 & _4;
_15.0 = _4 + _8;
_14 = [(-51_i8),101_i8,(-99_i8)];
_9.2 = _9.0 as usize;
_2 = -_13;
_15.2 = &_13;
_15.1.3 = _15.0;
_2 = _13 - _3;
match _5.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
181 => bb8,
_ => bb7
}
}
bb7 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb8 = {
_2 = -_13;
_9.1 = _5.0;
_9 = (164669387289349053390591277077599730782_u128, _5.0, 1_usize);
_9.2 = 17399889289178452864_usize;
_15.1.0 = !_9.1;
_1 = !23925_u16;
_15.1 = (_9.1, _9.2, _2, _6);
_14 = RET;
_5 = (_9.1,);
_15.1.1 = _9.2 & _9.2;
_9 = (107183291439839357945488444278601626358_u128, _15.1.0, _15.1.1);
_17 = _7;
_9.1 = true as u8;
RET = [(-114_i8),69_i8,24_i8];
_15.1 = (_9.1, _9.2, _2, _8);
_5.0 = _9.1;
_2 = _15.1.2 * _13;
_16 = [_1,_1,_1,_1,_1];
_15.1.3 = _4 & _8;
_12 = Adt28::Variant2 { fld0: _15.1,fld1: _17,fld2: 252993467733186060_u64,fld3: _9.0 };
match _9.0 {
107183291439839357945488444278601626358 => bb10,
_ => bb9
}
}
bb9 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb10 = {
_13 = _2;
Call(place!(Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0)).2 = fn14(Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).3, _15.0, _8, _4, _15.0, _15.1.3, _8, _15.1, _15.1.3, _7, _11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_18 = !_15.1.3;
_1 = 1643_u16 << _15.1.3;
RET = [114_i8,30_i8,10_i8];
_1 = !51157_u16;
_15.1.0 = 3189813879845539378_i64 as u8;
_4 = _15.1.3 + _8;
_15.3 = Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2 == Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2;
_9.2 = Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).1;
_5 = (_15.1.0,);
_19.1 = [_1,_1,_1,_1,_1,_1];
place!(Field::<char>(Variant(_12, 2), 1)) = _17;
_9.0 = Field::<u128>(Variant(_12, 2), 3) & Field::<u128>(Variant(_12, 2), 3);
place!(Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0)).0 = 1925387332_i32 as u8;
_13 = Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2 + Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2;
_19.1 = [_1,_1,_1,_1,_1,_1];
_15.1.0 = _5.0;
_19.1 = [_1,_1,_1,_1,_1,_1];
place!(Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0)).0 = _15.1.0;
match Field::<u128>(Variant(_12, 2), 3) {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
107183291439839357945488444278601626358 => bb17,
_ => bb16
}
}
bb12 = {
Return()
}
bb13 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_1 = 56562_u16 ^ 3438_u16;
_6 = (-9223372036854775808_isize) - (-29_isize);
_6 = 9223372036854775807_isize;
_1 = 84551761010487008437880823906355156023_i128 as u16;
_1 = !24837_u16;
RET = [(-45_i8),110_i8,67_i8];
_2 = _3;
RET = [24_i8,121_i8,(-71_i8)];
_5.0 = !183_u8;
_2 = _3 - _3;
RET = [(-114_i8),62_i8,77_i8];
_6 = (-86_isize);
_4 = _6 - _6;
_8 = _1 as isize;
_4 = 6_usize as isize;
_8 = _6;
_9.2 = (-62_i8) as usize;
Goto(bb2)
}
bb16 = {
RET = [58_i8,87_i8,51_i8];
RET = [7_i8,40_i8,45_i8];
_4 = -_6;
_11 = 2539_i16;
_9 = (222044009225338647519284513376811045753_u128, _5.0, 7_usize);
RET = [(-73_i8),(-84_i8),(-7_i8)];
_8 = (-8632140440936518069_i64) as isize;
_9.1 = _5.0 * _5.0;
_9.0 = 659780813_u32 as u128;
RET = [(-13_i8),100_i8,(-90_i8)];
_11 = false as i16;
RET = [42_i8,55_i8,69_i8];
_13 = -_2;
_14 = [70_i8,92_i8,(-36_i8)];
_2 = _11 as f32;
RET = [85_i8,(-73_i8),(-57_i8)];
RET = [126_i8,(-17_i8),(-7_i8)];
_8 = _6 & _4;
_15.0 = _4 + _8;
_14 = [(-51_i8),101_i8,(-99_i8)];
_9.2 = _9.0 as usize;
_2 = -_13;
_15.2 = &_13;
_15.1.3 = _15.0;
_2 = _13 - _3;
match _5.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
181 => bb8,
_ => bb7
}
}
bb17 = {
_4 = 4119426070_u32 as isize;
_4 = -_18;
_15.1.2 = -Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2;
_13 = Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0).2;
place!(Field::<(u8, usize, f32, isize)>(Variant(_12, 2), 0)).2 = _13;
_23.0 = _5.0 & _5.0;
_17 = Field::<char>(Variant(_12, 2), 1);
Goto(bb18)
}
bb18 = {
Call(_24 = dump_var(12_usize, 8_usize, Move(_8), 9_usize, Move(_9), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_24 = dump_var(12_usize, 14_usize, Move(_14), 11_usize, Move(_11), 25_usize, _25, 25_usize, _25), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: u16,mut _5: [i8; 3],mut _6: [i8; 3],mut _7: f32,mut _8: f32,mut _9: u16,mut _10: f32,mut _11: f32,mut _12: f32) -> (u8,) {
mir! {
type RET = (u8,);
let _13: &'static &'static *const u64;
let _14: [i16; 5];
let _15: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _16: Adt69;
let _17: *const ([u16; 6], Adt35, *const &'static f32);
let _18: isize;
let _19: isize;
let _20: (i8, &'static &'static i32, bool, usize);
let _21: f32;
let _22: char;
let _23: &'static &'static char;
let _24: f32;
let _25: Adt69;
let _26: &'static [i8; 1];
let _27: [u64; 7];
let _28: (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _29: bool;
let _30: bool;
let _31: u128;
let _32: u16;
let _33: char;
let _34: &'static i32;
let _35: i64;
let _36: ();
let _37: ();
{
_11 = _12;
_12 = _8;
_8 = _9 as f32;
_9 = 1024079688_u32 as u16;
RET.0 = 9_u8;
RET = (109_u8,);
_3 = _10;
_12 = _1;
_4 = !_9;
_12 = _2;
_1 = 105276120589957266472215230369037503481_i128 as f32;
_15.1 = ['\u{581f7}'];
_8 = -_3;
_2 = -_7;
RET = (26_u8,);
_15.1 = ['\u{c4cfc}'];
_15.3.3 = 7524288422391668538_u64 as usize;
_3 = _4 as f32;
_5 = [63_i8,67_i8,(-86_i8)];
Call(_7 = core::intrinsics::transmute(_15.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = 631627243824357454_i64 as f32;
_1 = -_8;
_15.3.0 = 47_i8;
_10 = 16473300426943703467_u64 as f32;
_16.fld1 = ['\u{63914}'];
_12 = _8 * _8;
_15.3.3 = 5_usize;
_2 = (-1400279840_i32) as f32;
_3 = _15.3.0 as f32;
_6 = _5;
_15.0 = !2251199515_u32;
_15.1 = ['\u{102c8a}'];
_16 = Adt69 { fld0: _9,fld1: _15.1 };
_2 = _15.3.0 as f32;
_15.3.2 = false;
_15.3.2 = false;
_2 = _12;
_6 = [_15.3.0,_15.3.0,_15.3.0];
RET = (193_u8,);
_1 = 58_isize as f32;
Call(_1 = core::intrinsics::transmute(_16.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [_15.3.0,_15.3.0,_15.3.0];
RET.0 = 233_u8;
_14 = [1415_i16,31998_i16,21879_i16,17923_i16,26177_i16];
_14 = [(-20040_i16),(-1566_i16),28161_i16,(-32601_i16),14491_i16];
_18 = 9223372036854775807_isize;
_5 = [_15.3.0,_15.3.0,_15.3.0];
_10 = _12;
_4 = !_9;
_7 = _2 * _2;
_16 = Adt69 { fld0: _4,fld1: _15.1 };
RET.0 = !126_u8;
_15.3.2 = _12 < _12;
_16.fld1 = ['\u{92929}'];
_12 = _15.0 as f32;
match _15.3.3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb9,
_ => bb8
}
}
bb3 = {
_10 = 631627243824357454_i64 as f32;
_1 = -_8;
_15.3.0 = 47_i8;
_10 = 16473300426943703467_u64 as f32;
_16.fld1 = ['\u{63914}'];
_12 = _8 * _8;
_15.3.3 = 5_usize;
_2 = (-1400279840_i32) as f32;
_3 = _15.3.0 as f32;
_6 = _5;
_15.0 = !2251199515_u32;
_15.1 = ['\u{102c8a}'];
_16 = Adt69 { fld0: _9,fld1: _15.1 };
_2 = _15.3.0 as f32;
_15.3.2 = false;
_15.3.2 = false;
_2 = _12;
_6 = [_15.3.0,_15.3.0,_15.3.0];
RET = (193_u8,);
_1 = 58_isize as f32;
Call(_1 = core::intrinsics::transmute(_16.fld1), ReturnTo(bb2), UnwindUnreachable())
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
_16.fld1 = _15.1;
_15.3.2 = false;
_2 = _15.0 as f32;
_21 = _10 * _7;
_20.3 = _15.3.3 << _15.3.3;
_7 = _15.0 as f32;
RET = (59_u8,);
_16 = Adt69 { fld0: _9,fld1: _15.1 };
_11 = _21;
_1 = _21;
Goto(bb10)
}
bb10 = {
_15.3.0 = 272716465764925139423814735567246084159_u128 as i8;
_14 = [(-7608_i16),(-7796_i16),(-3744_i16),26888_i16,29688_i16];
_11 = _15.0 as f32;
_2 = -_1;
_24 = _9 as f32;
_16.fld1 = ['\u{fee01}'];
RET = (242_u8,);
_22 = '\u{5f669}';
_12 = 89865841722887503852423581648022587449_u128 as f32;
_21 = -_1;
match _15.3.3 {
0 => bb6,
5 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_9 = !_16.fld0;
_25.fld1 = [_22];
_16.fld1 = [_22];
_25 = _16;
_25 = _16;
_7 = _1 - _1;
_9 = _4;
_15.3.3 = !_20.3;
_12 = 12810_i16 as f32;
_15.1 = _16.fld1;
_20.2 = !_15.3.2;
_6 = [_15.3.0,_15.3.0,_15.3.0];
_12 = _21 + _7;
_9 = _25.fld0;
_20.0 = 1035751855139540359_i64 as i8;
_5 = [_15.3.0,_15.3.0,_15.3.0];
_28.3.0 = (-2800_i16) as i8;
_15.0 = 2979841327_u32;
RET = (106_u8,);
_15.3.2 = _20.2;
match _15.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
2979841327 => bb21,
_ => bb20
}
}
bb13 = {
_10 = 631627243824357454_i64 as f32;
_1 = -_8;
_15.3.0 = 47_i8;
_10 = 16473300426943703467_u64 as f32;
_16.fld1 = ['\u{63914}'];
_12 = _8 * _8;
_15.3.3 = 5_usize;
_2 = (-1400279840_i32) as f32;
_3 = _15.3.0 as f32;
_6 = _5;
_15.0 = !2251199515_u32;
_15.1 = ['\u{102c8a}'];
_16 = Adt69 { fld0: _9,fld1: _15.1 };
_2 = _15.3.0 as f32;
_15.3.2 = false;
_15.3.2 = false;
_2 = _12;
_6 = [_15.3.0,_15.3.0,_15.3.0];
RET = (193_u8,);
_1 = 58_isize as f32;
Call(_1 = core::intrinsics::transmute(_16.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_15.3.0 = 272716465764925139423814735567246084159_u128 as i8;
_14 = [(-7608_i16),(-7796_i16),(-3744_i16),26888_i16,29688_i16];
_11 = _15.0 as f32;
_2 = -_1;
_24 = _9 as f32;
_16.fld1 = ['\u{fee01}'];
RET = (242_u8,);
_22 = '\u{5f669}';
_12 = 89865841722887503852423581648022587449_u128 as f32;
_21 = -_1;
match _15.3.3 {
0 => bb6,
5 => bb12,
_ => bb11
}
}
bb15 = {
_10 = 631627243824357454_i64 as f32;
_1 = -_8;
_15.3.0 = 47_i8;
_10 = 16473300426943703467_u64 as f32;
_16.fld1 = ['\u{63914}'];
_12 = _8 * _8;
_15.3.3 = 5_usize;
_2 = (-1400279840_i32) as f32;
_3 = _15.3.0 as f32;
_6 = _5;
_15.0 = !2251199515_u32;
_15.1 = ['\u{102c8a}'];
_16 = Adt69 { fld0: _9,fld1: _15.1 };
_2 = _15.3.0 as f32;
_15.3.2 = false;
_15.3.2 = false;
_2 = _12;
_6 = [_15.3.0,_15.3.0,_15.3.0];
RET = (193_u8,);
_1 = 58_isize as f32;
Call(_1 = core::intrinsics::transmute(_16.fld1), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb20 = {
_5 = [_15.3.0,_15.3.0,_15.3.0];
RET.0 = 233_u8;
_14 = [1415_i16,31998_i16,21879_i16,17923_i16,26177_i16];
_14 = [(-20040_i16),(-1566_i16),28161_i16,(-32601_i16),14491_i16];
_18 = 9223372036854775807_isize;
_5 = [_15.3.0,_15.3.0,_15.3.0];
_10 = _12;
_4 = !_9;
_7 = _2 * _2;
_16 = Adt69 { fld0: _4,fld1: _15.1 };
RET.0 = !126_u8;
_15.3.2 = _12 < _12;
_16.fld1 = ['\u{92929}'];
_12 = _15.0 as f32;
match _15.3.3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb9,
_ => bb8
}
}
bb21 = {
_28.3.0 = 15017552999033764156_u64 as i8;
_16.fld1 = [_22];
_5 = [_20.0,_28.3.0,_28.3.0];
RET.0 = 83_u8;
_29 = !_15.3.2;
RET = (124_u8,);
_30 = _29;
_15.0 = 2826620792_u32;
_16.fld0 = _25.fld0;
_28.1 = &_2;
_18 = 9223372036854775807_isize;
_9 = _4 >> _20.3;
_19 = -_18;
_15.2 = &_34;
Goto(bb22)
}
bb22 = {
Call(_36 = dump_var(13_usize, 19_usize, Move(_19), 9_usize, Move(_9), 30_usize, Move(_30), 18_usize, Move(_18)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_36 = dump_var(13_usize, 4_usize, Move(_4), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: (u8, usize, f32, isize),mut _9: isize,mut _10: char,mut _11: i16) -> f32 {
mir! {
type RET = f32;
let _12: char;
let _13: [char; 1];
let _14: [u16; 5];
let _15: &'static &'static Adt26;
let _16: isize;
let _17: char;
let _18: i128;
let _19: (i8, &'static &'static i32, bool, usize);
let _20: [u16; 6];
let _21: u32;
let _22: ();
let _23: ();
{
RET = -_8.2;
_8.2 = -RET;
_8.1 = !11783744516522010947_usize;
_8.1 = 1_usize;
_2 = _11 as isize;
RET = _11 as f32;
_2 = _8.3 << _1;
_8.0 = 216_u8;
_11 = (-197620207_i32) as i16;
_1 = false as isize;
Goto(bb1)
}
bb1 = {
_8.0 = !25_u8;
_7 = !_6;
_8.3 = 66_i8 as isize;
_8.3 = _10 as isize;
_1 = _3;
_12 = _10;
_8.0 = 1296_u16 as u8;
RET = _8.1 as f32;
_3 = -_6;
_8.3 = _2 ^ _5;
_1 = _8.3 & _8.3;
_6 = !_2;
_4 = _3;
_2 = !_3;
_8.0 = !96_u8;
_14 = [63577_u16,52153_u16,30910_u16,41942_u16,4044_u16];
_3 = !_1;
RET = (-1619168310969834840_i64) as f32;
RET = _8.2;
_13 = [_10];
_13 = [_12];
_8.3 = _3;
_16 = !_8.3;
_11 = (-4587_i16);
_13 = [_10];
_3 = _8.3;
_12 = _10;
_3 = _1;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768206869 => bb8,
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
_7 = _3 & _8.3;
_7 = _8.3 >> _8.3;
_13 = [_12];
RET = -_8.2;
_13 = [_10];
_8 = (187_u8, 3_usize, RET, _3);
_6 = 2955937976_u32 as isize;
_8.0 = 22_u8;
RET = 11386007704826446294645898362922808457_u128 as f32;
_9 = 3475477890_u32 as isize;
RET = _8.2 + _8.2;
_2 = -_8.3;
_13 = [_10];
_10 = _12;
_12 = _10;
_8.3 = 2654389897_u32 as isize;
_16 = _1 * _3;
_18 = 10_i8 as i128;
_11 = !(-30837_i16);
_6 = !_9;
_19.3 = _8.1;
RET = _7 as f32;
_8 = (182_u8, _19.3, RET, _7);
_19.2 = !true;
_14 = [48723_u16,12969_u16,26059_u16,8727_u16,49544_u16];
_17 = _12;
_21 = 55_i8 as u32;
Goto(bb9)
}
bb9 = {
Call(_22 = dump_var(14_usize, 17_usize, Move(_17), 21_usize, Move(_21), 3_usize, Move(_3), 18_usize, Move(_18)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_22 = dump_var(14_usize, 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: bool,mut _3: isize,mut _4: isize,mut _5: u8,mut _6: bool,mut _7: usize,mut _8: isize,mut _9: Adt65) -> (u8, usize, f32, isize) {
mir! {
type RET = (u8, usize, f32, isize);
let _10: isize;
let _11: u64;
let _12: *const Adt36;
let _13: u16;
let _14: [u64; 8];
let _15: isize;
let _16: Adt36;
let _17: f64;
let _18: i16;
let _19: char;
let _20: *mut (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _21: i64;
let _22: u16;
let _23: ();
let _24: ();
{
RET.0 = _9.fld0 >> _7;
_8 = -_4;
_9 = Adt65 { fld0: RET.0 };
_10 = (-3540571155259237530_i64) as isize;
RET.2 = 12145144_u32 as f32;
_3 = -_8;
RET.3 = _10 * _3;
_6 = _8 >= _4;
Goto(bb1)
}
bb1 = {
RET.0 = _9.fld0;
_7 = RET.0 as usize;
_10 = _8 + _3;
_15 = _10 - RET.3;
_1 = _2;
_11 = 14416120449179342151_u64 << RET.0;
RET.2 = 3342258986_u32 as f32;
_9 = Adt65 { fld0: RET.0 };
RET.2 = (-63_i8) as f32;
RET.0 = _9.fld0 + _9.fld0;
_3 = _1 as isize;
_8 = _3;
_3 = _15 << _11;
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
_12 = core::ptr::addr_of!(_16);
_7 = !7704153834379508773_usize;
_5 = RET.0 - RET.0;
_16 = Adt36::Variant2 { fld0: '\u{e8017}' };
RET.3 = RET.2 as isize;
Goto(bb2)
}
bb2 = {
_17 = (-3751482329724151390_i64) as f64;
_18 = 16237_i16 + (-2811_i16);
place!(Field::<char>(Variant((*_12), 2), 0)) = '\u{46dce}';
_9.fld0 = RET.0 ^ RET.0;
RET.1 = _7;
_5 = _9.fld0 << RET.0;
RET.1 = !_7;
RET.3 = _11 as isize;
RET.3 = _10 * _3;
RET.1 = !_7;
SetDiscriminant((*_12), 1);
place!(Field::<i8>(Variant((*_12), 1), 3)) = -(-81_i8);
Goto(bb3)
}
bb3 = {
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)).3 = 5795010323547458869_i64 as isize;
place!(Field::<i64>(Variant(_16, 1), 4)) = 8234844359727773974_i64;
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)).3 = RET.3 + _3;
_8 = Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1).3;
place!(Field::<[u16; 5]>(Variant(_16, 1), 0)) = [29502_u16,20133_u16,15654_u16,36119_u16,34842_u16];
_6 = _1;
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)).1 = _7;
place!(Field::<i8>(Variant((*_12), 1), 3)) = -29_i8;
RET.3 = !Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1).3;
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)) = (_9.fld0, RET.1, RET.2, _8);
RET.0 = 1710455062_i32 as u8;
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)) = (_5, _7, RET.2, _8);
place!(Field::<u16>(Variant(_16, 1), 2)) = !23290_u16;
RET.0 = (-109466033627233691956329411448024724979_i128) as u8;
place!(Field::<[u16; 5]>(Variant(_16, 1), 0)) = [Field::<u16>(Variant(_16, 1), 2),Field::<u16>(Variant(_16, 1), 2),Field::<u16>(Variant((*_12), 1), 2),Field::<u16>(Variant((*_12), 1), 2),Field::<u16>(Variant(_16, 1), 2)];
place!(Field::<(u8, usize, f32, isize)>(Variant((*_12), 1), 1)) = (_5, RET.1, RET.2, RET.3);
place!(Field::<(u8, usize, f32, isize)>(Variant(_16, 1), 1)).1 = RET.1 * _7;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(15_usize, 5_usize, Move(_5), 14_usize, Move(_14), 18_usize, Move(_18), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(15_usize, 15_usize, Move(_15), 7_usize, Move(_7), 24_usize, _24, 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: (u8, usize, f32, isize),mut _2: u8,mut _3: isize) -> f32 {
mir! {
type RET = f32;
let _4: ((i8, &'static &'static i32, bool, usize), &'static &'static i32, (u128, u128, *const &'static i32), &'static &'static Adt26);
let _5: u16;
let _6: [bool; 3];
let _7: (u128,);
let _8: (u8, usize, f32, isize);
let _9: [i8; 3];
let _10: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _11: ((i8, &'static &'static i32, bool, usize), &'static &'static i32, (u128, u128, *const &'static i32), &'static &'static Adt26);
let _12: i32;
let _13: *mut *const u64;
let _14: ();
let _15: ();
{
RET = _1.3 as f32;
Goto(bb1)
}
bb1 = {
_3 = !_1.3;
_4.0.2 = _3 != _3;
_1.0 = !_2;
_2 = _1.0;
_4.0.2 = true;
Goto(bb2)
}
bb2 = {
_1.0 = _2 << _2;
_4.0.0 = (-23_i8) << _1.0;
_1.2 = RET;
_3 = !_1.3;
RET = -_1.2;
_3 = _1.0 as isize;
_1.2 = -RET;
_1 = (_2, 13212455616308381639_usize, RET, _3);
_6 = [_4.0.2,_4.0.2,_4.0.2];
_4.2.1 = !272145051759578642378023508595733831852_u128;
_1.3 = _3 - _3;
Goto(bb3)
}
bb3 = {
_1 = (_2, 7685602140883379634_usize, RET, _3);
_1.0 = _1.3 as u8;
RET = -_1.2;
_4.0.3 = _1.1 ^ _1.1;
_7.0 = _4.0.0 as u128;
_5 = 27104_u16;
_4.2.0 = _4.0.0 as u128;
_1.0 = _2 * _2;
_4.2.0 = _7.0 - _7.0;
_4.0.2 = !true;
_1 = (_2, _4.0.3, RET, _3);
_4.2.1 = _4.2.0;
_2 = 4986866434171820378_u64 as u8;
_4.2.0 = _4.2.1 ^ _4.2.1;
RET = _1.2 * _1.2;
_1.3 = _3;
_4.2.0 = _4.2.1;
_4.0.0 = -120_i8;
_4.0.0 = 35_i8;
_1.1 = _4.0.3;
_3 = !_1.3;
match _4.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
35 => bb12,
_ => bb11
}
}
bb4 = {
_1.0 = _2 << _2;
_4.0.0 = (-23_i8) << _1.0;
_1.2 = RET;
_3 = !_1.3;
RET = -_1.2;
_3 = _1.0 as isize;
_1.2 = -RET;
_1 = (_2, 13212455616308381639_usize, RET, _3);
_6 = [_4.0.2,_4.0.2,_4.0.2];
_4.2.1 = !272145051759578642378023508595733831852_u128;
_1.3 = _3 - _3;
Goto(bb3)
}
bb5 = {
_3 = !_1.3;
_4.0.2 = _3 != _3;
_1.0 = !_2;
_2 = _1.0;
_4.0.2 = true;
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
Return()
}
bb11 = {
Return()
}
bb12 = {
_7 = (_4.2.0,);
_2 = _1.0;
_7 = (_4.2.1,);
_4.2.0 = !_7.0;
_6 = [_4.0.2,_4.0.2,_4.0.2];
_1.0 = !_2;
_8.0 = _2;
_4.2.1 = _4.2.0;
_1.3 = -_3;
_8.2 = RET + RET;
_1.1 = (-31997_i16) as usize;
_9 = [_4.0.0,_4.0.0,_4.0.0];
_4.0.2 = !true;
_10.3.0 = _5 as i8;
_6 = [_4.0.2,_4.0.2,_4.0.2];
_11.2.0 = !_4.2.0;
_8.1 = _4.0.0 as usize;
_11.2.1 = _8.2 as u128;
_12 = 841470445_i32;
_5 = _7.0 as u16;
_6 = [_4.0.2,_4.0.2,_4.0.2];
_10.3.2 = _4.0.2;
_10.3.3 = !_4.0.3;
_9 = [_4.0.0,_4.0.0,_4.0.0];
_4.0.2 = _10.3.2;
_4.2.0 = _7.0 << _11.2.1;
_11.2.1 = !_4.2.1;
_10.1 = ['\u{5f034}'];
Goto(bb13)
}
bb13 = {
_11.0.2 = _10.3.2;
RET = _8.2;
RET = _8.2 * _8.2;
_4.2.1 = _11.0.2 as u128;
_7 = (_11.2.1,);
Goto(bb14)
}
bb14 = {
Call(_14 = dump_var(16_usize, 9_usize, Move(_9), 7_usize, Move(_7), 2_usize, Move(_2), 15_usize, _15), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *const (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize)),mut _2: bool,mut _3: usize,mut _4: f32,mut _5: (u8, usize, f32, isize)) -> [u64; 7] {
mir! {
type RET = [u64; 7];
let _6: (u128, u128, *const &'static i32);
let _7: i16;
let _8: (u128,);
let _9: Adt65;
let _10: char;
let _11: [i16; 6];
let _12: (i8, &'static &'static i32, bool, usize);
let _13: f64;
let _14: u64;
let _15: u8;
let _16: isize;
let _17: ();
let _18: ();
{
RET = [17085209242805683671_u64,3602389352146221301_u64,2186649037436254685_u64,15661540326081006905_u64,9358021155899624255_u64,2379593087913548355_u64,9066693607793223245_u64];
_2 = true;
_5.2 = (-107_i8) as f32;
_2 = false;
RET = [16306304383019182241_u64,16306657561623416827_u64,16445838878716609347_u64,9880635281276469364_u64,15593724377017029308_u64,6532768899210464993_u64,4682203796287889054_u64];
_6.1 = 310564326639477481351126483244328071019_u128;
_5.0 = 183_u8;
_4 = _5.2;
_5 = (65_u8, _3, _4, (-9223372036854775808_isize));
_3 = _5.1;
_7 = (-29144_i16) | (-26553_i16);
_7 = !15080_i16;
_5.2 = _7 as f32;
match _5.3 {
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
_8.0 = !_6.1;
_5.0 = (-33466224332597445454226966329550465658_i128) as u8;
match _5.3 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_5.2 = _4;
_5.3 = (-10_isize) ^ 99_isize;
_6.0 = _6.1;
_8 = (_6.0,);
_3 = 16299827502104808498_u64 as usize;
_8 = (_6.0,);
RET = [14110588870910320940_u64,1301585737755734788_u64,18090228547807842295_u64,9756202039976300204_u64,7859652030692944867_u64,9928370422880031065_u64,4320245915772165017_u64];
_6.0 = _8.0;
_5.0 = 136_u8 ^ 70_u8;
_5.0 = 248_u8;
RET = [6679422034534435125_u64,1751818165690656029_u64,3014539528199234877_u64,6652588084144103014_u64,1256290330404219761_u64,7674118224546463473_u64,2486448951123950451_u64];
match _8.0 {
0 => bb5,
310564326639477481351126483244328071019 => bb9,
_ => bb8
}
}
bb8 = {
_8.0 = !_6.1;
_5.0 = (-33466224332597445454226966329550465658_i128) as u8;
match _5.3 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb9 = {
_8 = (_6.1,);
_5.1 = !_3;
Goto(bb10)
}
bb10 = {
_9 = Adt65 { fld0: _5.0 };
_6.1 = _6.0;
_11 = [_7,_7,_7,_7,_7,_7];
_12.0 = 96_i8 << _5.1;
_10 = '\u{60f80}';
match _9.fld0 {
0 => bb3,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
248 => bb17,
_ => bb16
}
}
bb11 = {
_8 = (_6.1,);
_5.1 = !_3;
Goto(bb10)
}
bb12 = {
_8.0 = !_6.1;
_5.0 = (-33466224332597445454226966329550465658_i128) as u8;
match _5.3 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
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
_8.0 = !_6.1;
_5.0 = (-33466224332597445454226966329550465658_i128) as u8;
match _5.3 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb17 = {
_5.1 = (-7118055671926285174_i64) as usize;
_12.0 = (-95_i8) | (-55_i8);
_12.3 = (-77393796927085906051140968307873946567_i128) as usize;
_12.0 = (-115_i8);
_5.1 = !_3;
_10 = '\u{6e694}';
_12.2 = _2 ^ _2;
_5.3 = 0_isize;
_5.0 = !_9.fld0;
_5.2 = -_4;
_6.1 = !_8.0;
_5.3 = 9223372036854775807_isize << _5.1;
RET = [3594675541227868222_u64,2697513407731028697_u64,12673752522454065871_u64,4015032624382923792_u64,11436775437996354388_u64,1768316447471311974_u64,1134081108045429150_u64];
_12.2 = _9.fld0 != _5.0;
_13 = _5.3 as f64;
Goto(bb18)
}
bb18 = {
Call(_17 = dump_var(17_usize, 10_usize, Move(_10), 3_usize, Move(_3), 11_usize, Move(_11), 18_usize, _18), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *const (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize)),mut _2: *mut (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize)),mut _3: bool,mut _4: u8,mut _5: isize,mut _6: isize) -> u32 {
mir! {
type RET = u32;
let _7: (*const &'static i32, &'static f32, &'static &'static Adt26, (i8, &'static &'static i32, bool, usize));
let _8: isize;
let _9: (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize));
let _10: isize;
let _11: u64;
let _12: isize;
let _13: i64;
let _14: isize;
let _15: f32;
let _16: [i8; 1];
let _17: isize;
let _18: ();
let _19: ();
{
_4 = 214_u8 << _5;
_5 = _6 - _6;
RET = 36_i8 as u32;
_3 = !true;
_9.3.0 = (-41955731103675198696508941464852465500_i128) as i8;
_7.3.2 = _3;
RET = 3230582191_u32 - 3558562119_u32;
_8 = 5_usize as isize;
_11 = (-241116500862919268_i64) as u64;
_9.0 = RET;
_9.3.3 = 3461375272165133970_usize;
_9.3.3 = 3_usize ^ 8412115891269892622_usize;
_12 = 100380605832499004329950915077450450292_i128 as isize;
_7.3.3 = _9.3.3;
Goto(bb1)
}
bb1 = {
_9.1 = ['\u{5f53d}'];
_9.3.3 = !_7.3.3;
_3 = !_7.3.2;
RET = _9.0 << _4;
_7.3.0 = _9.3.0 >> _6;
_11 = 1685035124107476581_u64;
_9.3.0 = !_7.3.0;
_4 = 114_u8 + 97_u8;
_1 = core::ptr::addr_of!(_9);
_14 = -_6;
(*_1).1 = ['\u{5fcab}'];
_9.3.3 = _7.3.3 >> _9.3.0;
(*_1).0 = !RET;
_11 = 1776483447160863777_u64;
_8 = _9.3.0 as isize;
_9.3.3 = _7.3.3;
_7.3.0 = -(*_1).3.0;
(*_1).1 = ['\u{7193c}'];
RET = (*_1).0 >> (*_1).0;
(*_1).3.2 = _7.3.2;
_9.3.3 = !_7.3.3;
_2 = core::ptr::addr_of_mut!((*_1));
(*_2).3.3 = _7.3.3;
(*_1).3.2 = !_7.3.2;
(*_1).3.0 = !_7.3.0;
_15 = 7277391503510041035_i64 as f32;
RET = (*_2).0;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(18_usize, 8_usize, Move(_8), 6_usize, Move(_6), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static &'static char,mut _2: *mut (u32, [char; 1], &'static &'static i32, (i8, &'static &'static i32, bool, usize)),mut _3: u8,mut _4: isize,mut _5: isize,mut _6: [u64; 7],mut _7: [char; 1],mut _8: u8,mut _9: bool,mut _10: usize,mut _11: [u16; 6],mut _12: char) -> u8 {
mir! {
type RET = u8;
let _13: bool;
let _14: isize;
let _15: f64;
let _16: (u128,);
let _17: ();
let _18: ();
{
_10 = 8745972272130016580_usize * 1_usize;
_10 = (-8490613717554926716_i64) as usize;
_10 = 1_usize + 1_usize;
_5 = _4 >> _4;
_12 = '\u{35342}';
_4 = -_5;
_11 = [16723_u16,33404_u16,46179_u16,44720_u16,53129_u16,14137_u16];
_7 = [_12];
_13 = !_9;
_4 = !_5;
_12 = '\u{e859d}';
RET = !_3;
RET = _3 * _3;
_10 = !5_usize;
_11 = [14246_u16,60333_u16,56650_u16,60578_u16,16537_u16,10630_u16];
_5 = 1242885566_u32 as isize;
_14 = RET as isize;
_7 = [_12];
_3 = !RET;
_14 = _3 as isize;
_14 = !_4;
_13 = !_9;
_5 = _4;
_12 = '\u{7d0b0}';
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(19_usize, 7_usize, Move(_7), 5_usize, Move(_5), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(19_usize, 4_usize, Move(_4), 6_usize, Move(_6), 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(192574444005266276899045037053164890658_u128), std::hint::black_box('\u{e03e8}'), std::hint::black_box((-65209439681218551336621480294092621428_i128)), std::hint::black_box((-2405728042615792013_i64)), std::hint::black_box(6279102322945760721_u64));
                
            }
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: bool,
fld1: f32,
fld2: f64,
fld3: i8,
fld4: u64,
fld5: i32,
fld6: *const i128,

},
Variant1{
fld0: bool,
fld1: (u128, u8, usize),
fld2: i64,

}}
impl PrintFDebug for Adt28{
	unsafe fn printf_debug(&self){unsafe{printf("Adt28::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt28 {
Variant0{
fld0: i16,
fld1: u16,
fld2: u32,

},
Variant1{
fld0: (u128, u8, usize),
fld1: i128,
fld2: isize,
fld3: u128,
fld4: i16,
fld5: [char; 1],

},
Variant2{
fld0: (u8, usize, f32, isize),
fld1: char,
fld2: u64,
fld3: u128,

},
Variant3{
fld0: (u128, u8, usize),
fld1: i128,
fld2: f64,
fld3: i8,
fld4: *const i128,
fld5: f32,
fld6: i64,

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: i64,

},
Variant1{
fld0: usize,

},
Variant2{
fld0: usize,
fld1: u32,
fld2: (u8, usize, f32, isize),
fld3: [u16; 5],
fld4: u64,
fld5: u16,

},
Variant3{
fld0: bool,
fld1: u8,
fld2: isize,
fld3: usize,
fld4: [char; 1],
fld5: i32,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: i16,
fld1: u8,

},
Variant1{
fld0: [u16; 5],
fld1: (u8, usize, f32, isize),
fld2: u16,
fld3: i8,
fld4: i64,
fld5: u8,

},
Variant2{
fld0: char,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt65{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt65 {
fld0: u8,
}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt69{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt69 {
fld0: u16,
fld1: [char; 1],
}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: bool,
fld1: [u16; 6],
fld2: (u8, usize, f32, isize),
fld3: Adt69,
fld4: u32,
fld5: *const i128,
fld6: *const Adt36,
fld7: u64,

},
Variant1{
fld0: i64,

}}
impl PrintFDebug for Adt83{
	unsafe fn printf_debug(&self){unsafe{printf("Adt83::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt83 {
Variant0{
fld0: i16,
fld1: i128,
fld2: usize,

},
Variant1{
fld0: char,

}}

