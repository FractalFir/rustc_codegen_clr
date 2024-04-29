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
pub fn fn0(mut _1: i128,mut _2: u32,mut _3: isize) -> f32 {
mir! {
type RET = f32;
let _4: isize;
let _5: u16;
let _6: i64;
let _7: bool;
let _8: Adt44;
let _9: Adt42;
let _10: (f64, i64, u8);
let _11: *mut isize;
let _12: Adt45;
let _13: Adt51;
let _14: [bool; 5];
let _15: (f64, i32, f32);
let _16: bool;
let _17: *mut i16;
let _18: Adt42;
let _19: Adt44;
let _20: *mut i16;
let _21: (f64, i32, f32);
let _22: f32;
let _23: ((*const u128,), (&'static u8,));
let _24: isize;
let _25: (f64, i64, u8);
let _26: ();
let _27: ();
{
RET = 50346052906549625543304438511427900286_u128 as f32;
_3 = (-20_isize);
RET = (-2872_i16) as f32;
_1 = 136395306632509102197010342492321439649_i128;
_3 = 9223372036854775807_isize;
_3 = 9223372036854775807_isize & 9223372036854775807_isize;
RET = (-1746_i16) as f32;
RET = _1 as f32;
_3 = 9223372036854775807_isize;
_2 = 575706258_u32;
_3 = _2 as isize;
_2 = !3636368967_u32;
_2 = 190_u8 as u32;
Goto(bb1)
}
bb1 = {
_2 = 181_u8 as u32;
_4 = !_3;
_3 = !_4;
_4 = '\u{bac93}' as isize;
Call(_1 = fn1(_4, _2, _2, RET, _3, RET, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 17163_u16 << _1;
_3 = -_4;
_1 = -30628288444799379179792770783542593523_i128;
_5 = !47181_u16;
RET = _2 as f32;
Goto(bb3)
}
bb3 = {
_3 = _4;
RET = _3 as f32;
_5 = 14693_u16;
_4 = _3;
_5 = !404_u16;
_6 = (-5383918885708405785_i64);
_3 = 3_usize as isize;
_5 = 13760_u16;
RET = 17924_i16 as f32;
RET = _1 as f32;
RET = _1 as f32;
RET = 7_usize as f32;
_5 = 45127_u16;
_3 = _4 >> _5;
RET = _1 as f32;
RET = _6 as f32;
_3 = _4;
RET = _5 as f32;
Goto(bb4)
}
bb4 = {
_8 = Adt44::Variant0 { fld0: 4223348803888014854_u64,fld1: _5 };
place!(Field::<u16>(Variant(_8, 0), 1)) = !_5;
RET = _4 as f32;
_4 = _3 ^ _3;
_3 = 1_usize as isize;
_1 = 3747310732008978322_usize as i128;
_6 = (-8902065857428940251_i64);
_3 = _4 - _4;
place!(Field::<u64>(Variant(_8, 0), 0)) = !7711899235784444865_u64;
_10.1 = _6;
_8 = Adt44::Variant0 { fld0: 16659328537268452512_u64,fld1: _5 };
_8 = Adt44::Variant0 { fld0: 2961513318191667255_u64,fld1: _5 };
_2 = !449888373_u32;
_2 = 201_u8 as u32;
_7 = false;
_3 = _4 | _4;
match _5 {
0 => bb2,
45127 => bb6,
_ => bb5
}
}
bb5 = {
_3 = _4;
RET = _3 as f32;
_5 = 14693_u16;
_4 = _3;
_5 = !404_u16;
_6 = (-5383918885708405785_i64);
_3 = 3_usize as isize;
_5 = 13760_u16;
RET = 17924_i16 as f32;
RET = _1 as f32;
RET = _1 as f32;
RET = 7_usize as f32;
_5 = 45127_u16;
_3 = _4 >> _5;
RET = _1 as f32;
RET = _6 as f32;
_3 = _4;
RET = _5 as f32;
Goto(bb4)
}
bb6 = {
RET = _2 as f32;
_11 = core::ptr::addr_of_mut!(_3);
_15.1 = -398205609_i32;
_3 = !_4;
_15.0 = _2 as f64;
_10.0 = _15.0;
_7 = true;
place!(Field::<u64>(Variant(_8, 0), 0)) = 11150468049633708036_u64;
_16 = !_7;
match _5 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
45127 => bb12,
_ => bb11
}
}
bb7 = {
_3 = _4;
RET = _3 as f32;
_5 = 14693_u16;
_4 = _3;
_5 = !404_u16;
_6 = (-5383918885708405785_i64);
_3 = 3_usize as isize;
_5 = 13760_u16;
RET = 17924_i16 as f32;
RET = _1 as f32;
RET = _1 as f32;
RET = 7_usize as f32;
_5 = 45127_u16;
_3 = _4 >> _5;
RET = _1 as f32;
RET = _6 as f32;
_3 = _4;
RET = _5 as f32;
Goto(bb4)
}
bb8 = {
_8 = Adt44::Variant0 { fld0: 4223348803888014854_u64,fld1: _5 };
place!(Field::<u16>(Variant(_8, 0), 1)) = !_5;
RET = _4 as f32;
_4 = _3 ^ _3;
_3 = 1_usize as isize;
_1 = 3747310732008978322_usize as i128;
_6 = (-8902065857428940251_i64);
_3 = _4 - _4;
place!(Field::<u64>(Variant(_8, 0), 0)) = !7711899235784444865_u64;
_10.1 = _6;
_8 = Adt44::Variant0 { fld0: 16659328537268452512_u64,fld1: _5 };
_8 = Adt44::Variant0 { fld0: 2961513318191667255_u64,fld1: _5 };
_2 = !449888373_u32;
_2 = 201_u8 as u32;
_7 = false;
_3 = _4 | _4;
match _5 {
0 => bb2,
45127 => bb6,
_ => bb5
}
}
bb9 = {
_3 = _4;
RET = _3 as f32;
_5 = 14693_u16;
_4 = _3;
_5 = !404_u16;
_6 = (-5383918885708405785_i64);
_3 = 3_usize as isize;
_5 = 13760_u16;
RET = 17924_i16 as f32;
RET = _1 as f32;
RET = _1 as f32;
RET = 7_usize as f32;
_5 = 45127_u16;
_3 = _4 >> _5;
RET = _1 as f32;
RET = _6 as f32;
_3 = _4;
RET = _5 as f32;
Goto(bb4)
}
bb10 = {
_5 = 17163_u16 << _1;
_3 = -_4;
_1 = -30628288444799379179792770783542593523_i128;
_5 = !47181_u16;
RET = _2 as f32;
Goto(bb3)
}
bb11 = {
_2 = 181_u8 as u32;
_4 = !_3;
_3 = !_4;
_4 = '\u{bac93}' as isize;
Call(_1 = fn1(_4, _2, _2, RET, _3, RET, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_15.2 = -RET;
_1 = _2 as i128;
_2 = _15.0 as u32;
_5 = Field::<u16>(Variant(_8, 0), 1) << _1;
_10 = (_15.0, _6, 206_u8);
_10 = (_15.0, _6, 89_u8);
_19 = Adt44::Variant0 { fld0: Field::<u64>(Variant(_8, 0), 0),fld1: _5 };
(*_11) = !_4;
_19 = Move(_8);
_22 = _15.2 + RET;
_21 = _15;
RET = _15.2;
_14 = [_7,_7,_7,_7,_16];
match Field::<u16>(Variant(_19, 0), 1) {
45127 => bb13,
_ => bb1
}
}
bb13 = {
_9 = Adt42::Variant2 { fld0: _2,fld1: 14405622540499331232_usize,fld2: _21.0,fld3: Field::<u16>(Variant(_19, 0), 1),fld4: _14 };
_22 = Field::<u64>(Variant(_19, 0), 0) as f32;
_1 = (-32262320425221647021214408080663884118_i128) >> _10.1;
_21.2 = RET;
_10.2 = _10.1 as u8;
RET = _22;
_16 = _7;
_21.2 = 4_usize as f32;
_15 = (_21.0, _21.1, _21.2);
_15.2 = (-117_i8) as f32;
_24 = _3 >> Field::<u64>(Variant(_19, 0), 0);
_6 = _10.1 + _10.1;
_21.2 = RET - _15.2;
Goto(bb14)
}
bb14 = {
RET = _21.1 as f32;
_19 = Adt44::Variant0 { fld0: 4016776698518320672_u64,fld1: _5 };
place!(Field::<f64>(Variant(_9, 2), 2)) = _21.0 + _15.0;
_2 = !Field::<u32>(Variant(_9, 2), 0);
_3 = _4 + _24;
RET = 47_i8 as f32;
_4 = '\u{95aa3}' as isize;
place!(Field::<u64>(Variant(_19, 0), 0)) = 15944_i16 as u64;
place!(Field::<u16>(Variant(_19, 0), 1)) = _5;
_11 = core::ptr::addr_of_mut!(_3);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(0_usize, 16_usize, Move(_16), 24_usize, Move(_24), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(0_usize, 3_usize, Move(_3), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: u32,mut _3: u32,mut _4: f32,mut _5: isize,mut _6: f32,mut _7: isize,mut _8: isize,mut _9: isize) -> i128 {
mir! {
type RET = i128;
let _10: [i8; 3];
let _11: (char,);
let _12: Adt50;
let _13: f32;
let _14: [i64; 7];
let _15: [i64; 7];
let _16: [i8; 5];
let _17: Adt42;
let _18: bool;
let _19: (f64, i64, u8);
let _20: f32;
let _21: Adt42;
let _22: isize;
let _23: isize;
let _24: Adt50;
let _25: f32;
let _26: &'static u8;
let _27: f32;
let _28: Adt53;
let _29: isize;
let _30: [i8; 5];
let _31: [i8; 1];
let _32: char;
let _33: i128;
let _34: u8;
let _35: u16;
let _36: f32;
let _37: u64;
let _38: isize;
let _39: [i8; 1];
let _40: char;
let _41: Adt50;
let _42: ();
let _43: ();
{
RET = (-34828669687276764810484788436502842786_i128);
_10 = [15_i8,80_i8,(-103_i8)];
_4 = 57302_u16 as f32;
_1 = -_8;
_6 = _4;
_3 = _2;
_11.0 = '\u{bb353}';
_12 = Adt50 { fld0: _10 };
_12.fld0 = _10;
RET = -(-88341851970161385072217025240552633690_i128);
_9 = _8 >> _8;
_4 = _6;
RET = (-7982_i16) as i128;
_1 = _9;
_4 = _6 + _6;
_11.0 = '\u{22326}';
_9 = _8 << _8;
_2 = _3;
_12 = Adt50 { fld0: _10 };
_2 = _3 - _3;
_11 = ('\u{7c766}',);
Goto(bb1)
}
bb1 = {
RET = (-115406959855683026755918337356670046786_i128) ^ (-79633472911115453726806931199880105510_i128);
_2 = _3;
_11 = ('\u{87b25}',);
_8 = 51595_u16 as isize;
RET = -(-50229028931515265918052335468546241721_i128);
RET = (-57583881084494960448952925958558834512_i128);
_4 = _6 * _6;
_10 = [65_i8,37_i8,44_i8];
_3 = _4 as u32;
_15 = [906757754293878279_i64,(-4007200712447923321_i64),2144058720267164397_i64,313641690275198665_i64,4735415614558989784_i64,7068879177671224652_i64,6493172924072667090_i64];
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
282698485836443503014421681473209376944 => bb6,
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
_8 = (-7224_i16) as isize;
_11.0 = '\u{9692b}';
_16 = [36_i8,30_i8,(-3_i8),(-41_i8),(-95_i8)];
_14 = _15;
_2 = _3 ^ _3;
_13 = _6;
_10 = [18_i8,(-18_i8),67_i8];
_12.fld0 = _10;
_19.0 = (-27015_i16) as f64;
_18 = _1 <= _8;
_5 = _9 * _1;
_15 = [6026622626787277512_i64,4337836077925923853_i64,(-127448580548138950_i64),(-8887384810679292005_i64),(-8362348781136353691_i64),(-816822801391308967_i64),1775998350138774594_i64];
_19.0 = 7304815046522112495_u64 as f64;
_8 = _1;
_11.0 = '\u{30d38}';
_11 = ('\u{74191}',);
_19.2 = 148_u8;
_4 = -_6;
RET = 113758673576372332351586181394196207049_i128;
_19.0 = 15254052495213186847_u64 as f64;
Goto(bb7)
}
bb7 = {
_6 = 31351_i16 as f32;
Call(_12 = fn2(_7, _5, RET, _8, _14, _18, _8, _14, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15 = [6445145463920621142_i64,(-6713079460526836642_i64),(-2723049192584577267_i64),6092482412410830738_i64,7598669938021022827_i64,(-4273202547591331116_i64),(-3312935176781868206_i64)];
_19.1 = 8122161625098895016_i64 >> _9;
_2 = !_3;
_19.1 = !(-5463126983785582721_i64);
_19.1 = _4 as i64;
_19.2 = 108_u8;
_19.2 = 100_u8;
_16 = [(-85_i8),125_i8,98_i8,(-93_i8),119_i8];
_18 = true & false;
_11 = ('\u{f79d0}',);
_13 = _6;
_1 = !_5;
_13 = _4;
_11 = ('\u{c8e59}',);
_2 = _3;
_19.1 = 5861196135767755862_i64;
_14 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
_10 = [(-12_i8),(-83_i8),(-21_i8)];
_3 = _2 - _2;
_8 = -_5;
_7 = _19.1 as isize;
_24.fld0 = [19_i8,24_i8,(-71_i8)];
match _19.2 {
0 => bb1,
1 => bb6,
2 => bb3,
100 => bb9,
_ => bb4
}
}
bb9 = {
_12.fld0 = _10;
_25 = -_13;
_18 = true;
_8 = 0_usize as isize;
_21 = Adt42::Variant0 { fld0: _25,fld1: _9 };
_15 = _14;
_24 = _12;
place!(Field::<isize>(Variant(_21, 0), 1)) = !_1;
_25 = _19.1 as f32;
_9 = _5 >> _1;
_26 = &_19.2;
_27 = -_4;
_27 = Field::<f32>(Variant(_21, 0), 0) - _4;
_7 = !_9;
_13 = -_6;
_6 = Field::<isize>(Variant(_21, 0), 1) as f32;
_17 = Move(_21);
_12 = Adt50 { fld0: _10 };
_28.fld0 = Adt45::Variant1 { fld0: _19,fld1: _3,fld2: 16634285328332331467_u64 };
_7 = _27 as isize;
_16 = [(-1_i8),(-48_i8),105_i8,(-47_i8),(-15_i8)];
_12 = Adt50 { fld0: _10 };
place!(Field::<isize>(Variant(_17, 0), 1)) = _9;
Goto(bb10)
}
bb10 = {
_10 = _24.fld0;
_1 = _3 as isize;
RET = 127606310661149268745137113165606987633_i128 * (-42688023194170280922912705123638191393_i128);
_16 = [51_i8,74_i8,55_i8,(-71_i8),(-12_i8)];
_12.fld0 = [110_i8,111_i8,85_i8];
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).1 = (-2090582906_i32) as i64;
_28.fld0 = Adt45::Variant1 { fld0: _19,fld1: _2,fld2: 694351768455982878_u64 };
_16 = [(-97_i8),30_i8,4_i8,(-50_i8),54_i8];
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).0 = -_19.0;
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).0 = Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1 as f64;
_7 = _8;
_24 = Adt50 { fld0: _12.fld0 };
_4 = _27 + _6;
_25 = _4 * _6;
_10 = [(-123_i8),(-20_i8),119_i8];
_1 = _9;
_1 = Field::<isize>(Variant(_17, 0), 1);
_3 = !Field::<u32>(Variant(_28.fld0, 1), 1);
_24.fld0 = [(-54_i8),51_i8,(-107_i8)];
_2 = !Field::<u32>(Variant(_28.fld0, 1), 1);
_12.fld0 = _24.fld0;
_13 = _25;
_15 = [Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1,Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1,_19.1,Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1,Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1,Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1,_19.1];
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).0 = _19.0;
_16 = [(-46_i8),126_i8,(-29_i8),82_i8,48_i8];
_27 = _13 * _13;
match (*_26) {
0 => bb4,
1 => bb6,
2 => bb11,
100 => bb13,
_ => bb12
}
}
bb11 = {
_8 = (-7224_i16) as isize;
_11.0 = '\u{9692b}';
_16 = [36_i8,30_i8,(-3_i8),(-41_i8),(-95_i8)];
_14 = _15;
_2 = _3 ^ _3;
_13 = _6;
_10 = [18_i8,(-18_i8),67_i8];
_12.fld0 = _10;
_19.0 = (-27015_i16) as f64;
_18 = _1 <= _8;
_5 = _9 * _1;
_15 = [6026622626787277512_i64,4337836077925923853_i64,(-127448580548138950_i64),(-8887384810679292005_i64),(-8362348781136353691_i64),(-816822801391308967_i64),1775998350138774594_i64];
_19.0 = 7304815046522112495_u64 as f64;
_8 = _1;
_11.0 = '\u{30d38}';
_11 = ('\u{74191}',);
_19.2 = 148_u8;
_4 = -_6;
RET = 113758673576372332351586181394196207049_i128;
_19.0 = 15254052495213186847_u64 as f64;
Goto(bb7)
}
bb12 = {
_15 = [6445145463920621142_i64,(-6713079460526836642_i64),(-2723049192584577267_i64),6092482412410830738_i64,7598669938021022827_i64,(-4273202547591331116_i64),(-3312935176781868206_i64)];
_19.1 = 8122161625098895016_i64 >> _9;
_2 = !_3;
_19.1 = !(-5463126983785582721_i64);
_19.1 = _4 as i64;
_19.2 = 108_u8;
_19.2 = 100_u8;
_16 = [(-85_i8),125_i8,98_i8,(-93_i8),119_i8];
_18 = true & false;
_11 = ('\u{f79d0}',);
_13 = _6;
_1 = !_5;
_13 = _4;
_11 = ('\u{c8e59}',);
_2 = _3;
_19.1 = 5861196135767755862_i64;
_14 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
_10 = [(-12_i8),(-83_i8),(-21_i8)];
_3 = _2 - _2;
_8 = -_5;
_7 = _19.1 as isize;
_24.fld0 = [19_i8,24_i8,(-71_i8)];
match _19.2 {
0 => bb1,
1 => bb6,
2 => bb3,
100 => bb9,
_ => bb4
}
}
bb13 = {
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).1 = -_19.1;
_21 = Move(_17);
RET = (-121546971437304396114587551114276152307_i128);
RET = 22050270605605178658532815169971779999_u128 as i128;
_20 = _25 - _4;
_31 = [(-68_i8)];
_33 = !RET;
_10 = [(-22_i8),(-85_i8),63_i8];
place!(Field::<u64>(Variant(_28.fld0, 1), 2)) = !15952277895722048231_u64;
_4 = _27;
_10 = _12.fld0;
RET = 64441_u16 as i128;
_18 = false;
_37 = Field::<u64>(Variant(_28.fld0, 1), 2);
RET = Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).2 as i128;
_19 = (Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).0, Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).1, Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0).2);
_26 = &_19.2;
SetDiscriminant(_28.fld0, 1);
_11 = ('\u{1d6a3}',);
Goto(bb14)
}
bb14 = {
RET = _33;
place!(Field::<(f64, i64, u8)>(Variant(_28.fld0, 1), 0)).0 = -_19.0;
place!(Field::<u64>(Variant(_28.fld0, 1), 2)) = (-71_i8) as u64;
_15 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
_34 = (*_26);
_27 = -_25;
_35 = !20536_u16;
_22 = _9 - _1;
_12.fld0 = [(-18_i8),94_i8,(-82_i8)];
_39 = [65_i8];
_32 = _11.0;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(1_usize, 39_usize, Move(_39), 7_usize, Move(_7), 35_usize, Move(_35), 33_usize, Move(_33)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(1_usize, 1_usize, Move(_1), 3_usize, Move(_3), 31_usize, Move(_31), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(1_usize, 11_usize, Move(_11), 10_usize, Move(_10), 43_usize, _43, 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: i128,mut _4: isize,mut _5: [i64; 7],mut _6: bool,mut _7: isize,mut _8: [i64; 7],mut _9: f32) -> Adt50 {
mir! {
type RET = Adt50;
let _10: f32;
let _11: (*mut i16, u16, i128, char, *mut i16);
let _12: Adt44;
let _13: i64;
let _14: (*mut isize, [isize; 2]);
let _15: Adt54;
let _16: *const usize;
let _17: Adt50;
let _18: (f64, i64, u8);
let _19: bool;
let _20: isize;
let _21: Adt55;
let _22: i128;
let _23: Adt58;
let _24: Adt55;
let _25: isize;
let _26: *const (*mut i16, u16, i128, char, *mut i16);
let _27: u8;
let _28: i8;
let _29: isize;
let _30: u8;
let _31: Adt58;
let _32: Adt44;
let _33: i64;
let _34: *mut isize;
let _35: i16;
let _36: Adt50;
let _37: ();
let _38: ();
{
RET.fld0 = [20_i8,123_i8,(-9_i8)];
_4 = 771181148_u32 as isize;
RET.fld0 = [(-28_i8),56_i8,(-66_i8)];
_5 = _8;
RET.fld0 = [(-104_i8),(-27_i8),(-68_i8)];
_9 = 693140785_u32 as f32;
_4 = -_2;
Goto(bb1)
}
bb1 = {
_8 = _5;
_3 = 48623218339727444908206390710647032553_i128;
_9 = _3 as f32;
_5 = [2714162314431971805_i64,(-8290067716888869652_i64),5857985691405593574_i64,(-4175388644514754124_i64),3229074913995430718_i64,(-3642582141946746646_i64),(-2152584540712466_i64)];
RET.fld0 = [(-66_i8),82_i8,45_i8];
_11.1 = 47161_u16 + 48340_u16;
_11.3 = '\u{89d26}';
_11.1 = 2854_u16 + 27492_u16;
_11.2 = _3;
_9 = (-17471_i16) as f32;
_2 = _6 as isize;
_9 = _11.2 as f32;
Goto(bb2)
}
bb2 = {
_3 = -_11.2;
_11.3 = '\u{91102}';
_5 = _8;
_4 = -_2;
_6 = !false;
_11.1 = !11474_u16;
_11.2 = 92_i8 as i128;
_4 = (-6438909451055179820_i64) as isize;
_11.2 = (-1891437253_i32) as i128;
_8 = _5;
_10 = -_9;
_10 = _11.1 as f32;
_5 = [5760612413156185937_i64,(-1131445440037464559_i64),(-4428912364092265147_i64),1190328848776293882_i64,(-5035468196959750738_i64),4592704776507075272_i64,(-7206028475113601737_i64)];
_11.3 = '\u{fbde9}';
RET.fld0 = [23_i8,(-103_i8),53_i8];
_11.2 = 3454283911_u32 as i128;
RET.fld0 = [71_i8,31_i8,97_i8];
_13 = 4_usize as i64;
_9 = _10;
Call(_7 = fn3(_9, _8, _11.3, _13, _8, _2, _10, _2, _11.3, _8, _2, _8, _5, RET, _8, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _9;
_14.1 = [_2,_4];
_11.3 = '\u{f90b4}';
RET.fld0 = [(-71_i8),122_i8,49_i8];
RET.fld0 = [95_i8,(-77_i8),42_i8];
RET.fld0 = [27_i8,(-12_i8),(-43_i8)];
_8 = [_13,_13,_13,_13,_13,_13,_13];
_14.1 = [_7,_7];
_7 = !_4;
_11.2 = _3;
_12 = Adt44::Variant0 { fld0: 16100737960571229222_u64,fld1: _11.1 };
RET.fld0 = [(-9_i8),43_i8,(-17_i8)];
_11.1 = !Field::<u16>(Variant(_12, 0), 1);
Goto(bb4)
}
bb4 = {
_5 = _8;
_4 = _1;
_6 = !false;
_12 = Adt44::Variant0 { fld0: 16503088981004510015_u64,fld1: _11.1 };
_17 = RET;
_12 = Adt44::Variant0 { fld0: 5495091912484355005_u64,fld1: _11.1 };
_17.fld0 = [(-92_i8),26_i8,(-57_i8)];
_10 = -_9;
_13 = 7944277629064650693_i64;
RET.fld0 = [(-56_i8),(-117_i8),7_i8];
_11.2 = _3 - _3;
_9 = -_10;
RET = _17;
Goto(bb5)
}
bb5 = {
place!(Field::<u64>(Variant(_12, 0), 0)) = (-244863804_i32) as u64;
RET.fld0 = [(-69_i8),(-98_i8),84_i8];
_18.2 = Field::<u64>(Variant(_12, 0), 0) as u8;
place!(Field::<u64>(Variant(_12, 0), 0)) = 13689248861216070329_u64;
place!(Field::<u64>(Variant(_12, 0), 0)) = 8433994791216169729_u64 << _4;
RET.fld0 = [93_i8,(-54_i8),(-26_i8)];
SetDiscriminant(_12, 1);
_18.1 = 5126696653941429928_u64 as i64;
place!(Field::<i64>(Variant(_12, 1), 2)) = _18.1;
_10 = _9;
_13 = _18.1 + _18.1;
place!(Field::<f64>(Variant(_12, 1), 3)) = 85079164_u32 as f64;
_11.1 = 46888_u16;
place!(Field::<f64>(Variant(_12, 1), 3)) = _9 as f64;
Call(_17.fld0 = core::intrinsics::transmute(RET.fld0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET.fld0 = [(-87_i8),(-59_i8),97_i8];
_17.fld0 = RET.fld0;
_17.fld0 = RET.fld0;
_19 = !_6;
place!(Field::<[i64; 8]>(Variant(_12, 1), 0)) = [_13,_18.1,Field::<i64>(Variant(_12, 1), 2),_18.1,Field::<i64>(Variant(_12, 1), 2),Field::<i64>(Variant(_12, 1), 2),_13,_18.1];
_21.fld1.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_12, 1), 1)));
_12 = Adt44::Variant0 { fld0: 10801526360947762393_u64,fld1: _11.1 };
place!(Field::<u64>(Variant(_12, 0), 0)) = !4219340950382475477_u64;
_22 = -_11.2;
Goto(bb7)
}
bb7 = {
SetDiscriminant(_12, 0);
_18.2 = !8_u8;
_21.fld5 = _11.3 as u64;
_10 = -_9;
_18.0 = _2 as f64;
_9 = _10;
_2 = !_7;
match _11.1 {
0 => bb8,
46888 => bb10,
_ => bb9
}
}
bb8 = {
_3 = -_11.2;
_11.3 = '\u{91102}';
_5 = _8;
_4 = -_2;
_6 = !false;
_11.1 = !11474_u16;
_11.2 = 92_i8 as i128;
_4 = (-6438909451055179820_i64) as isize;
_11.2 = (-1891437253_i32) as i128;
_8 = _5;
_10 = -_9;
_10 = _11.1 as f32;
_5 = [5760612413156185937_i64,(-1131445440037464559_i64),(-4428912364092265147_i64),1190328848776293882_i64,(-5035468196959750738_i64),4592704776507075272_i64,(-7206028475113601737_i64)];
_11.3 = '\u{fbde9}';
RET.fld0 = [23_i8,(-103_i8),53_i8];
_11.2 = 3454283911_u32 as i128;
RET.fld0 = [71_i8,31_i8,97_i8];
_13 = 4_usize as i64;
_9 = _10;
Call(_7 = fn3(_9, _8, _11.3, _13, _8, _2, _10, _2, _11.3, _8, _2, _8, _5, RET, _8, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_8 = _5;
_3 = 48623218339727444908206390710647032553_i128;
_9 = _3 as f32;
_5 = [2714162314431971805_i64,(-8290067716888869652_i64),5857985691405593574_i64,(-4175388644514754124_i64),3229074913995430718_i64,(-3642582141946746646_i64),(-2152584540712466_i64)];
RET.fld0 = [(-66_i8),82_i8,45_i8];
_11.1 = 47161_u16 + 48340_u16;
_11.3 = '\u{89d26}';
_11.1 = 2854_u16 + 27492_u16;
_11.2 = _3;
_9 = (-17471_i16) as f32;
_2 = _6 as isize;
_9 = _11.2 as f32;
Goto(bb2)
}
bb10 = {
_6 = _19;
_20 = _2 ^ _2;
_17.fld0 = [41_i8,(-82_i8),(-58_i8)];
_11.1 = (-512427915_i32) as u16;
_24.fld4 = core::ptr::addr_of_mut!(_20);
_25 = -_1;
_24.fld1.0 = _21.fld1.0;
_8 = _5;
_21.fld2 = _9;
_26 = core::ptr::addr_of!(_11);
_24.fld1.0 = _21.fld1.0;
_8 = [_13,_18.1,_13,_13,_13,_13,_13];
RET = _17;
_17.fld0 = [19_i8,(-117_i8),(-19_i8)];
_18.2 = 198_u8;
Goto(bb11)
}
bb11 = {
place!(Field::<u16>(Variant(_12, 0), 1)) = (*_26).1;
RET = _17;
(*_26).3 = '\u{fb289}';
RET = _17;
_26 = core::ptr::addr_of!(_11);
_21.fld1.0 = _24.fld1.0;
RET.fld0 = [(-78_i8),105_i8,105_i8];
_21.fld2 = -_9;
_11.3 = '\u{1019ea}';
place!(Field::<u16>(Variant(_12, 0), 1)) = _21.fld5 as u16;
_2 = _7;
_11.2 = _22;
_4 = (-16387_i16) as isize;
_21.fld5 = _6 as u64;
_19 = _6 & _6;
place!(Field::<u16>(Variant(_12, 0), 1)) = 2_usize as u16;
Goto(bb12)
}
bb12 = {
(*_26).2 = _21.fld2 as i128;
_26 = core::ptr::addr_of!((*_26));
(*_26).2 = -_22;
(*_26).2 = _22;
Goto(bb13)
}
bb13 = {
_14.1 = [_20,_2];
_24.fld2 = 41522702915422856645736477280775768139_u128 as f32;
(*_26).1 = Field::<u16>(Variant(_12, 0), 1) - Field::<u16>(Variant(_12, 0), 1);
_21.fld5 = (-45_i8) as u64;
_29 = _7 - _20;
_23 = Adt58::Variant2 { fld0: _18,fld1: _14.1 };
_9 = _21.fld5 as f32;
_21.fld4 = core::ptr::addr_of_mut!(_1);
_5 = [_18.1,_13,_18.1,_13,_18.1,Field::<(f64, i64, u8)>(Variant(_23, 2), 0).1,_18.1];
_7 = _1 ^ _29;
_14.0 = core::ptr::addr_of_mut!(_4);
_13 = _18.1 << _29;
_24.fld4 = _14.0;
_24.fld5 = 1179584107680171613_usize as u64;
match Field::<(f64, i64, u8)>(Variant(_23, 2), 0).2 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
198 => bb19,
_ => bb18
}
}
bb14 = {
place!(Field::<u64>(Variant(_12, 0), 0)) = (-244863804_i32) as u64;
RET.fld0 = [(-69_i8),(-98_i8),84_i8];
_18.2 = Field::<u64>(Variant(_12, 0), 0) as u8;
place!(Field::<u64>(Variant(_12, 0), 0)) = 13689248861216070329_u64;
place!(Field::<u64>(Variant(_12, 0), 0)) = 8433994791216169729_u64 << _4;
RET.fld0 = [93_i8,(-54_i8),(-26_i8)];
SetDiscriminant(_12, 1);
_18.1 = 5126696653941429928_u64 as i64;
place!(Field::<i64>(Variant(_12, 1), 2)) = _18.1;
_10 = _9;
_13 = _18.1 + _18.1;
place!(Field::<f64>(Variant(_12, 1), 3)) = 85079164_u32 as f64;
_11.1 = 46888_u16;
place!(Field::<f64>(Variant(_12, 1), 3)) = _9 as f64;
Call(_17.fld0 = core::intrinsics::transmute(RET.fld0), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_3 = -_11.2;
_11.3 = '\u{91102}';
_5 = _8;
_4 = -_2;
_6 = !false;
_11.1 = !11474_u16;
_11.2 = 92_i8 as i128;
_4 = (-6438909451055179820_i64) as isize;
_11.2 = (-1891437253_i32) as i128;
_8 = _5;
_10 = -_9;
_10 = _11.1 as f32;
_5 = [5760612413156185937_i64,(-1131445440037464559_i64),(-4428912364092265147_i64),1190328848776293882_i64,(-5035468196959750738_i64),4592704776507075272_i64,(-7206028475113601737_i64)];
_11.3 = '\u{fbde9}';
RET.fld0 = [23_i8,(-103_i8),53_i8];
_11.2 = 3454283911_u32 as i128;
RET.fld0 = [71_i8,31_i8,97_i8];
_13 = 4_usize as i64;
_9 = _10;
Call(_7 = fn3(_9, _8, _11.3, _13, _8, _2, _10, _2, _11.3, _8, _2, _8, _5, RET, _8, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_6 = _19;
_20 = _2 ^ _2;
_17.fld0 = [41_i8,(-82_i8),(-58_i8)];
_11.1 = (-512427915_i32) as u16;
_24.fld4 = core::ptr::addr_of_mut!(_20);
_25 = -_1;
_24.fld1.0 = _21.fld1.0;
_8 = _5;
_21.fld2 = _9;
_26 = core::ptr::addr_of!(_11);
_24.fld1.0 = _21.fld1.0;
_8 = [_13,_18.1,_13,_13,_13,_13,_13];
RET = _17;
_17.fld0 = [19_i8,(-117_i8),(-19_i8)];
_18.2 = 198_u8;
Goto(bb11)
}
bb17 = {
_8 = _5;
_3 = 48623218339727444908206390710647032553_i128;
_9 = _3 as f32;
_5 = [2714162314431971805_i64,(-8290067716888869652_i64),5857985691405593574_i64,(-4175388644514754124_i64),3229074913995430718_i64,(-3642582141946746646_i64),(-2152584540712466_i64)];
RET.fld0 = [(-66_i8),82_i8,45_i8];
_11.1 = 47161_u16 + 48340_u16;
_11.3 = '\u{89d26}';
_11.1 = 2854_u16 + 27492_u16;
_11.2 = _3;
_9 = (-17471_i16) as f32;
_2 = _6 as isize;
_9 = _11.2 as f32;
Goto(bb2)
}
bb18 = {
RET.fld0 = [(-87_i8),(-59_i8),97_i8];
_17.fld0 = RET.fld0;
_17.fld0 = RET.fld0;
_19 = !_6;
place!(Field::<[i64; 8]>(Variant(_12, 1), 0)) = [_13,_18.1,Field::<i64>(Variant(_12, 1), 2),_18.1,Field::<i64>(Variant(_12, 1), 2),Field::<i64>(Variant(_12, 1), 2),_13,_18.1];
_21.fld1.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_12, 1), 1)));
_12 = Adt44::Variant0 { fld0: 10801526360947762393_u64,fld1: _11.1 };
place!(Field::<u64>(Variant(_12, 0), 0)) = !4219340950382475477_u64;
_22 = -_11.2;
Goto(bb7)
}
bb19 = {
RET = Adt50 { fld0: _17.fld0 };
_10 = _9;
_11.3 = '\u{5ceae}';
RET.fld0 = [(-12_i8),(-4_i8),22_i8];
_18.1 = _19 as i64;
_4 = _7;
_24.fld2 = 2672357117122193060_usize as f32;
SetDiscriminant(_23, 2);
_18.1 = _10 as i64;
_21.fld5 = (*_26).2 as u64;
RET.fld0 = _17.fld0;
_21.fld1.0 = _24.fld1.0;
_28 = -(-41_i8);
place!(Field::<(f64, i64, u8)>(Variant(_23, 2), 0)) = (_18.0, _18.1, _18.2);
_17.fld0 = RET.fld0;
_17.fld0 = [_28,_28,_28];
_24.fld1.0 = _21.fld1.0;
_12 = Adt44::Variant0 { fld0: _21.fld5,fld1: _11.1 };
place!(Field::<u64>(Variant(_12, 0), 0)) = 3514243714_u32 as u64;
_11.0 = core::ptr::addr_of_mut!(_35);
Goto(bb20)
}
bb20 = {
Call(_37 = dump_var(2_usize, 19_usize, Move(_19), 29_usize, Move(_29), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_37 = dump_var(2_usize, 7_usize, Move(_7), 28_usize, Move(_28), 3_usize, Move(_3), 38_usize, _38), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f32,mut _2: [i64; 7],mut _3: char,mut _4: i64,mut _5: [i64; 7],mut _6: isize,mut _7: f32,mut _8: isize,mut _9: char,mut _10: [i64; 7],mut _11: isize,mut _12: [i64; 7],mut _13: [i64; 7],mut _14: Adt50,mut _15: [i64; 7],mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: bool;
let _18: bool;
let _19: i32;
let _20: *mut isize;
let _21: f32;
let _22: [i8; 3];
let _23: (f64, i64, u8);
let _24: (&'static u8, usize, isize, isize);
let _25: *mut isize;
let _26: (char,);
let _27: Adt47;
let _28: (char,);
let _29: i32;
let _30: isize;
let _31: (&'static u8, usize, isize, isize);
let _32: ();
let _33: ();
{
_4 = 53866488862651108266830044005050939116_u128 as i64;
_14.fld0 = [(-98_i8),9_i8,115_i8];
RET = _8;
_17 = _6 < _16;
_12 = [_4,_4,_4,_4,_4,_4,_4];
_18 = _17 ^ _17;
_11 = !_8;
_2 = _15;
_9 = _3;
_19 = -(-1410274044_i32);
_12 = [_4,_4,_4,_4,_4,_4,_4];
Call(_9 = fn4(_5, _18, _16, RET, _10, _14, _14, _18, _10, _4, _18, _11, _14.fld0, _17, _17, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _3;
_17 = !_18;
_13 = [_4,_4,_4,_4,_4,_4,_4];
_2 = [_4,_4,_4,_4,_4,_4,_4];
_20 = core::ptr::addr_of_mut!(RET);
_23.2 = 237_u8;
_6 = (*_20);
_5 = [_4,_4,_4,_4,_4,_4,_4];
Call(_8 = core::intrinsics::transmute((*_20)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = 25921_u16 as isize;
_24.0 = &_23.2;
_24.3 = 20166_u16 as isize;
_10 = [_4,_4,_4,_4,_4,_4,_4];
_24.1 = !11057930855373138295_usize;
_24.2 = _6 * _24.3;
_26 = (_9,);
(*_20) = 105081708688754822679525219214653528552_u128 as isize;
_22 = [83_i8,126_i8,(-54_i8)];
match _23.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
237 => bb10,
_ => bb9
}
}
bb3 = {
_9 = _3;
_17 = !_18;
_13 = [_4,_4,_4,_4,_4,_4,_4];
_2 = [_4,_4,_4,_4,_4,_4,_4];
_20 = core::ptr::addr_of_mut!(RET);
_23.2 = 237_u8;
_6 = (*_20);
_5 = [_4,_4,_4,_4,_4,_4,_4];
Call(_8 = core::intrinsics::transmute((*_20)), ReturnTo(bb2), UnwindUnreachable())
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
_18 = _17 ^ _17;
_3 = _26.0;
_19 = -(-298796572_i32);
_25 = _20;
_14 = Adt50 { fld0: _22 };
RET = _8 << _16;
_23.0 = 55997_u16 as f64;
_5 = [_4,_4,_4,_4,_4,_4,_4];
_4 = (-1210442776211861003_i64) >> (*_25);
_14 = Adt50 { fld0: _22 };
RET = -_11;
_26 = (_3,);
_6 = !(*_20);
_24.0 = &_23.2;
_24.0 = &_23.2;
Goto(bb11)
}
bb11 = {
_14.fld0 = _22;
_4 = (-3022412286096229487_i64);
(*_20) = -_16;
_14.fld0 = _22;
_14.fld0 = _22;
_1 = _7 - _7;
_3 = _26.0;
(*_20) = _11;
_9 = _3;
_1 = _7;
match _4 {
0 => bb1,
1 => bb12,
2 => bb13,
340282366920938463460352195145671981969 => bb15,
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
_9 = _3;
_17 = !_18;
_13 = [_4,_4,_4,_4,_4,_4,_4];
_2 = [_4,_4,_4,_4,_4,_4,_4];
_20 = core::ptr::addr_of_mut!(RET);
_23.2 = 237_u8;
_6 = (*_20);
_5 = [_4,_4,_4,_4,_4,_4,_4];
Call(_8 = core::intrinsics::transmute((*_20)), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_10 = [_4,_4,_4,_4,_4,_4,_4];
_2 = [_4,_4,_4,_4,_4,_4,_4];
_24.1 = 3_usize << _8;
RET = _8;
_5 = _15;
_24.0 = &_23.2;
_28 = (_3,);
_31.0 = &_23.2;
_20 = core::ptr::addr_of_mut!((*_25));
_28 = (_9,);
_23.2 = 138_u8;
_31.3 = (*_20) & RET;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(3_usize, 16_usize, Move(_16), 2_usize, Move(_2), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(3_usize, 13_usize, Move(_13), 26_usize, Move(_26), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(3_usize, 22_usize, Move(_22), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [i64; 7],mut _2: bool,mut _3: isize,mut _4: isize,mut _5: [i64; 7],mut _6: Adt50,mut _7: Adt50,mut _8: bool,mut _9: [i64; 7],mut _10: i64,mut _11: bool,mut _12: isize,mut _13: [i8; 3],mut _14: bool,mut _15: bool,mut _16: f32) -> char {
mir! {
type RET = char;
let _17: isize;
let _18: isize;
let _19: isize;
let _20: Adt56;
let _21: u8;
let _22: Adt43;
let _23: isize;
let _24: Adt42;
let _25: Adt44;
let _26: (f64, i32, f32);
let _27: [i64; 7];
let _28: isize;
let _29: Adt50;
let _30: Adt49;
let _31: Adt44;
let _32: [bool; 5];
let _33: (char,);
let _34: [bool; 5];
let _35: isize;
let _36: f64;
let _37: (f64, i32, f32);
let _38: (char,);
let _39: f64;
let _40: (&'static u8, usize, isize, isize);
let _41: [i8; 3];
let _42: f32;
let _43: bool;
let _44: [i8; 1];
let _45: f32;
let _46: isize;
let _47: i64;
let _48: u128;
let _49: bool;
let _50: Adt43;
let _51: bool;
let _52: Adt42;
let _53: usize;
let _54: f64;
let _55: char;
let _56: *const (*mut i16, u16, i128, char, *mut i16);
let _57: i8;
let _58: *mut isize;
let _59: (*mut i16, u16, i128, char, *mut i16);
let _60: u32;
let _61: ();
let _62: ();
{
_4 = _12;
RET = '\u{333c9}';
_7 = _6;
_1 = [_10,_10,_10,_10,_10,_10,_10];
_8 = _11 & _2;
RET = '\u{b456f}';
_17 = (-90157700078575347419660514877300679659_i128) as isize;
RET = '\u{77800}';
_8 = _11;
_13 = _7.fld0;
_6.fld0 = _7.fld0;
_13 = [20_i8,(-112_i8),(-69_i8)];
_17 = _4;
_4 = _16 as isize;
_16 = 26_u8 as f32;
_17 = 240_u8 as isize;
_4 = _12 & _17;
_13 = [(-83_i8),(-118_i8),(-27_i8)];
_4 = _3 - _12;
_15 = _8 == _2;
_18 = _4 * _17;
_1 = [_10,_10,_10,_10,_10,_10,_10];
Goto(bb1)
}
bb1 = {
_17 = _4;
_8 = _15;
_6.fld0 = [118_i8,76_i8,63_i8];
_2 = _15;
_6.fld0 = _13;
_21 = 219_u8;
_2 = _15 ^ _8;
_22 = Adt43::Variant0 { fld0: (-17943_i16),fld1: _21 };
_21 = _8 as u8;
_15 = _14;
RET = '\u{36269}';
_6.fld0 = [48_i8,90_i8,72_i8];
_23 = RET as isize;
place!(Field::<u8>(Variant(_22, 0), 1)) = _21;
place!(Field::<u8>(Variant(_22, 0), 1)) = 83827553892815816238120121942598556209_i128 as u8;
Call(_7.fld0 = fn5(_13, _3, _11, _21, _18, _21, RET, _2, _21, _5, _2, _8, _2, _2, _8, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_19 = _3 + _4;
place!(Field::<u8>(Variant(_22, 0), 1)) = _21;
_3 = !_18;
_28 = _3;
_17 = _3 & _28;
_15 = !_2;
_26.1 = (-947629145_i32) - 1423314234_i32;
_27 = [_10,_10,_10,_10,_10,_10,_10];
_29 = Adt50 { fld0: _7.fld0 };
_9 = _5;
_9 = [_10,_10,_10,_10,_10,_10,_10];
RET = '\u{1b71}';
Goto(bb3)
}
bb3 = {
_1 = [_10,_10,_10,_10,_10,_10,_10];
_31 = Adt44::Variant0 { fld0: 18174979642066586736_u64,fld1: 21251_u16 };
_32 = [_15,_8,_2,_14,_2];
_37.0 = 85029559870150151161489952632184746185_u128 as f64;
_29 = _7;
_35 = -_18;
place!(Field::<i16>(Variant(_22, 0), 0)) = 23600_i16;
_32 = [_8,_2,_8,_8,_2];
_36 = -_37.0;
Call(_6 = fn16(_32, _15, _7, _7.fld0, Move(_22), _8, _29, _11, _7.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_19 = (-5972149194578489224870183146890849589_i128) as isize;
_38.0 = RET;
_27 = _5;
_7 = Adt50 { fld0: _6.fld0 };
_21 = (-52_i8) as u8;
_16 = 4004913939_u32 as f32;
_2 = _15 >= _15;
_40.2 = 234075217379186083934454376402297382769_u128 as isize;
_39 = -_36;
_18 = _28;
_40.0 = &_21;
_9 = _5;
_37.1 = !_26.1;
_12 = _16 as isize;
_37.1 = 88769281991319390874076620681915717553_u128 as i32;
_7 = Adt50 { fld0: _6.fld0 };
_17 = (-151120038647307732282132597664241491726_i128) as isize;
_1 = [_10,_10,_10,_10,_10,_10,_10];
_29 = Adt50 { fld0: _6.fld0 };
_26.0 = 9212_i16 as f64;
_18 = 163716975851648950822292137828761677756_u128 as isize;
_40.3 = !_35;
_26.2 = 219965291734566847426805718817828964520_u128 as f32;
_13 = [(-107_i8),88_i8,(-115_i8)];
_4 = !_40.3;
Call(_17 = core::intrinsics::bswap(_3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = 39_i8 as f32;
_41 = [60_i8,(-114_i8),46_i8];
_4 = !_35;
_38 = (RET,);
_37.2 = _26.2;
Goto(bb6)
}
bb6 = {
_34 = _32;
_33.0 = RET;
_12 = _40.3;
_28 = !_40.2;
_31 = Adt44::Variant0 { fld0: 2769836753823706953_u64,fld1: 42498_u16 };
_38.0 = RET;
_37.1 = !_26.1;
_26.1 = _37.1;
RET = _38.0;
_17 = _40.2 ^ _40.3;
_3 = _17;
_26.1 = _2 as i32;
Goto(bb7)
}
bb7 = {
RET = _38.0;
_25 = Adt44::Variant0 { fld0: 16926032016850570530_u64,fld1: 39392_u16 };
_26.1 = 101826293384690484777873362758338983276_i128 as i32;
_11 = _15;
_37.1 = _26.1 | _26.1;
Goto(bb8)
}
bb8 = {
RET = _38.0;
_44 = [34_i8];
_26.2 = _37.2;
_14 = !_15;
_8 = _2;
_31 = Adt44::Variant0 { fld0: 13601304946783419245_u64,fld1: 60679_u16 };
_35 = !_12;
_40.1 = !6_usize;
_11 = _2;
_29.fld0 = [(-61_i8),121_i8,(-24_i8)];
_35 = _21 as isize;
RET = _38.0;
_37.2 = -_26.2;
place!(Field::<u16>(Variant(_25, 0), 1)) = !35440_u16;
_42 = 774896074_u32 as f32;
_19 = _26.1 as isize;
_42 = _26.2;
_16 = _40.1 as f32;
_37.0 = _26.0;
_31 = Adt44::Variant0 { fld0: 7958413126898619127_u64,fld1: Field::<u16>(Variant(_25, 0), 1) };
place!(Field::<u16>(Variant(_25, 0), 1)) = Field::<u16>(Variant(_31, 0), 1) >> _4;
place!(Field::<u64>(Variant(_31, 0), 0)) = _26.0 as u64;
Goto(bb9)
}
bb9 = {
_45 = -_26.2;
_33 = (RET,);
SetDiscriminant(_31, 0);
_32 = [_2,_11,_2,_2,_15];
place!(Field::<u64>(Variant(_25, 0), 0)) = 15507631769400278820_u64 + 3742267999723582057_u64;
_3 = Field::<u64>(Variant(_25, 0), 0) as isize;
_40.2 = _40.3 * _40.3;
Goto(bb10)
}
bb10 = {
SetDiscriminant(_25, 1);
_40.2 = _12;
_4 = _23;
_38 = (RET,);
_25 = Adt44::Variant0 { fld0: 13874052905815575873_u64,fld1: 63059_u16 };
_33.0 = _38.0;
place!(Field::<u16>(Variant(_31, 0), 1)) = _40.1 as u16;
_38 = (RET,);
_12 = (-21017_i16) as isize;
_33.0 = RET;
_9 = [_10,_10,_10,_10,_10,_10,_10];
_6.fld0 = [46_i8,5_i8,(-124_i8)];
_49 = !_8;
_12 = -_17;
_7.fld0 = [75_i8,5_i8,(-10_i8)];
_29 = Adt50 { fld0: _41 };
_36 = _37.0;
_7.fld0 = [87_i8,(-29_i8),49_i8];
place!(Field::<u16>(Variant(_31, 0), 1)) = 54576_u16 >> _17;
_17 = _4;
_26.1 = !_37.1;
_32 = [_11,_14,_8,_11,_11];
place!(Field::<u16>(Variant(_25, 0), 1)) = (-41_i8) as u16;
_51 = !_11;
Call(_40.1 = fn17(_32, _49, _8, _14, _15, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_40.2 = _17 & _35;
_32 = [_11,_49,_2,_11,_11];
place!(Field::<u64>(Variant(_25, 0), 0)) = 13335897143875603788_u64;
_23 = -_40.3;
_37.2 = _26.2 * _26.2;
_9 = [_10,_10,_10,_10,_10,_10,_10];
_37 = _26;
SetDiscriminant(_25, 0);
_33.0 = _38.0;
place!(Field::<u64>(Variant(_25, 0), 0)) = 17854374272584637806_u64;
_40.2 = !_23;
_54 = -_39;
_38.0 = _33.0;
place!(Field::<u64>(Variant(_31, 0), 0)) = Field::<u64>(Variant(_25, 0), 0) << Field::<u16>(Variant(_31, 0), 1);
_33.0 = RET;
Goto(bb12)
}
bb12 = {
_16 = -_37.2;
_38 = (_33.0,);
place!(Field::<u16>(Variant(_25, 0), 1)) = !Field::<u16>(Variant(_31, 0), 1);
SetDiscriminant(_31, 0);
_47 = _37.0 as i64;
_47 = _10 | _10;
_7 = Adt50 { fld0: _41 };
_2 = !_8;
_49 = !_8;
_31 = Adt44::Variant0 { fld0: Field::<u64>(Variant(_25, 0), 0),fld1: Field::<u16>(Variant(_25, 0), 1) };
_16 = (-157921069178972043997666954287927021443_i128) as f32;
_35 = 2984514682_u32 as isize;
RET = _38.0;
_52 = Adt42::Variant0 { fld0: _42,fld1: _40.2 };
_51 = !_49;
match Field::<u64>(Variant(_25, 0), 0) {
17854374272584637806 => bb14,
_ => bb13
}
}
bb13 = {
_45 = -_26.2;
_33 = (RET,);
SetDiscriminant(_31, 0);
_32 = [_2,_11,_2,_2,_15];
place!(Field::<u64>(Variant(_25, 0), 0)) = 15507631769400278820_u64 + 3742267999723582057_u64;
_3 = Field::<u64>(Variant(_25, 0), 0) as isize;
_40.2 = _40.3 * _40.3;
Goto(bb10)
}
bb14 = {
_33.0 = _38.0;
RET = _38.0;
_35 = _23;
_44 = [(-2_i8)];
_26.0 = _37.1 as f64;
_14 = _51 != _49;
_53 = _40.1 | _40.1;
_33 = (_38.0,);
_8 = !_51;
_14 = _51 >= _49;
place!(Field::<u64>(Variant(_31, 0), 0)) = Field::<u64>(Variant(_25, 0), 0) + Field::<u64>(Variant(_25, 0), 0);
_35 = 13_i8 as isize;
_7.fld0 = [90_i8,(-120_i8),56_i8];
_21 = (-76_i8) as u8;
_33.0 = _38.0;
place!(Field::<u64>(Variant(_25, 0), 0)) = Field::<u64>(Variant(_31, 0), 0);
SetDiscriminant(_52, 0);
place!(Field::<u16>(Variant(_25, 0), 1)) = Field::<u16>(Variant(_31, 0), 1);
_23 = _40.3;
_58 = core::ptr::addr_of_mut!(_19);
_11 = _8 != _8;
_37.1 = -_26.1;
_59.3 = RET;
_54 = _36;
_38.0 = _59.3;
_53 = _40.1 ^ _40.1;
_33.0 = _38.0;
Goto(bb15)
}
bb15 = {
Call(_61 = dump_var(4_usize, 21_usize, Move(_21), 17_usize, Move(_17), 3_usize, Move(_3), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_61 = dump_var(4_usize, 49_usize, Move(_49), 1_usize, Move(_1), 28_usize, Move(_28), 41_usize, Move(_41)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_61 = dump_var(4_usize, 8_usize, Move(_8), 27_usize, Move(_27), 33_usize, Move(_33), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_61 = dump_var(4_usize, 51_usize, Move(_51), 47_usize, Move(_47), 23_usize, Move(_23), 62_usize, _62), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i8; 3],mut _2: isize,mut _3: bool,mut _4: u8,mut _5: isize,mut _6: u8,mut _7: char,mut _8: bool,mut _9: u8,mut _10: [i64; 7],mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool,mut _16: isize) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _17: *const usize;
let _18: Adt56;
let _19: [i8; 3];
let _20: u32;
let _21: (f64, i32, f32);
let _22: f32;
let _23: [isize; 2];
let _24: (char,);
let _25: f64;
let _26: [i8; 5];
let _27: Adt47;
let _28: [i64; 7];
let _29: u128;
let _30: isize;
let _31: Adt54;
let _32: isize;
let _33: f64;
let _34: i128;
let _35: char;
let _36: f32;
let _37: f64;
let _38: *mut isize;
let _39: i64;
let _40: f64;
let _41: (char,);
let _42: (char,);
let _43: (f64, i32, f32);
let _44: (f64, i64, u8);
let _45: Adt45;
let _46: [isize; 2];
let _47: u16;
let _48: f64;
let _49: char;
let _50: usize;
let _51: [i64; 7];
let _52: usize;
let _53: (f64, i32, f32);
let _54: Adt44;
let _55: [i8; 3];
let _56: Adt57;
let _57: usize;
let _58: u8;
let _59: i128;
let _60: u16;
let _61: *mut isize;
let _62: [i64; 7];
let _63: i8;
let _64: f32;
let _65: u32;
let _66: (char,);
let _67: u32;
let _68: f32;
let _69: f32;
let _70: isize;
let _71: Adt44;
let _72: ();
let _73: ();
{
_10 = [(-3851899311171698633_i64),(-6028547695723982716_i64),8473278297212634359_i64,(-427963086874805674_i64),(-7047273017472408436_i64),(-8022934555839703761_i64),6112019307680104916_i64];
_15 = _8;
_20 = !1066120380_u32;
Call(_2 = fn6(_6, _5, _15, _15, _9, _9, _8, _16, _11, _11, _8, _6, _15, _4, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = '\u{346cd}';
_20 = !899553537_u32;
_19 = _1;
_1 = [(-86_i8),(-57_i8),35_i8];
_21.2 = (-71_i8) as f32;
_9 = !_6;
_20 = 1683785270_u32 << _2;
_4 = _6;
_22 = (-1463073456_i32) as f32;
_22 = -_21.2;
_15 = _13 & _13;
Goto(bb2)
}
bb2 = {
_24 = (_7,);
RET = [(-18_i8),(-62_i8),21_i8];
_8 = _11 & _12;
_24 = (_7,);
_24.0 = _7;
_19 = _1;
_16 = _2 | _2;
_21.0 = (-487175353_i32) as f64;
_14 = !_12;
_20 = 2725304379_u32 >> _16;
RET = _1;
_23 = [_2,_2];
Call(_9 = fn7(_16, _23, _20, _2, _8, _4, _23), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_21.2 = _21.0 as f32;
_7 = _24.0;
_1 = [72_i8,33_i8,117_i8];
RET = _1;
_21.2 = -_22;
_24 = (_7,);
RET = [(-63_i8),9_i8,(-83_i8)];
_8 = _3 == _15;
_3 = !_13;
RET = [(-57_i8),(-122_i8),(-127_i8)];
RET = [100_i8,48_i8,(-47_i8)];
_24 = (_7,);
_14 = !_15;
_21.2 = -_22;
_19 = RET;
_22 = -_21.2;
_3 = _14 >= _14;
_26 = [80_i8,107_i8,(-115_i8),(-29_i8),(-123_i8)];
_21.1 = (-2057884518_i32) | 1400401636_i32;
Goto(bb4)
}
bb4 = {
_4 = (-15637_i16) as u8;
RET = _19;
_13 = _11 | _15;
_3 = _13;
_23 = [_16,_16];
_19 = RET;
_14 = _13;
_24.0 = _7;
_5 = 110952081386108985035087774180649948456_u128 as isize;
_11 = _8;
_21.0 = 8945_u16 as f64;
_34 = 26039_u16 as i128;
_32 = -_2;
_11 = _8;
_33 = _21.0 + _21.0;
_25 = -_33;
_29 = 24891854611215606567551335803299018601_u128 + 337901821149724951669428498745172993544_u128;
_14 = !_8;
_13 = _3;
_6 = !_4;
_30 = _16 - _16;
_19 = [20_i8,(-62_i8),45_i8];
_24.0 = _7;
_15 = !_13;
_35 = _7;
_28 = [6101469672615100572_i64,4689040278828699757_i64,(-1485920144965822289_i64),(-5198368242801759166_i64),(-5259131685097015038_i64),(-5584331103945473818_i64),(-5963707731283720490_i64)];
Call(_24 = fn15(_23, _15, _3, _3, _3, _23, _14, _15, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_24 = (_35,);
_28 = [8660550005468014692_i64,705854115236888993_i64,(-7523465327986998931_i64),(-4565094361001255207_i64),(-7340020241749836194_i64),(-686419529884270882_i64),3256356040490352949_i64];
_7 = _35;
_24.0 = _35;
_32 = !_30;
_24.0 = _7;
_15 = !_3;
_8 = _20 > _20;
_38 = core::ptr::addr_of_mut!(_2);
_9 = _6 - _4;
(*_38) = _16 | _16;
_24 = (_7,);
_9 = 1_usize as u8;
_28 = [962729438578486134_i64,8998153317341112615_i64,(-3003126738970042616_i64),2085456511978014605_i64,7281657776494471001_i64,(-5879043671915622477_i64),496560872039671259_i64];
Goto(bb6)
}
bb6 = {
_39 = _20 as i64;
_43 = _21;
_25 = -_33;
_4 = _34 as u8;
_16 = !_32;
_44 = (_21.0, _39, _6);
_36 = _21.1 as f32;
_32 = _30 | _2;
_41 = (_35,);
_37 = _21.0;
_44 = (_37, _39, _4);
_42 = (_41.0,);
_21.2 = _39 as f32;
_32 = _29 as isize;
RET = [87_i8,90_i8,(-32_i8)];
RET = [(-94_i8),(-84_i8),(-125_i8)];
_46 = [_16,(*_38)];
_47 = _2 as u16;
_36 = _21.2 - _21.2;
(*_38) = _16 << _44.1;
_5 = !_30;
_12 = _43.2 < _21.2;
_20 = 17392918594480974279_usize as u32;
_23 = [(*_38),_30];
_5 = !_30;
Goto(bb7)
}
bb7 = {
_44 = (_43.0, _39, _6);
_44 = (_43.0, _39, _9);
_48 = _44.0 - _43.0;
_15 = _11;
_41.0 = _35;
RET = _19;
_44.0 = _44.1 as f64;
_6 = 15134244687981280860_u64 as u8;
_43.0 = _44.0;
_37 = -_25;
_19 = [(-62_i8),(-39_i8),(-112_i8)];
_53 = (_43.0, _43.1, _36);
_53.2 = _36;
_40 = _43.0;
(*_38) = _5 - _30;
RET = _1;
_20 = _34 as u32;
Goto(bb8)
}
bb8 = {
_46 = _23;
_19 = [15_i8,(-112_i8),(-104_i8)];
_48 = -_44.0;
_32 = !_5;
_13 = _47 == _47;
_43.1 = _21.1;
_41 = (_35,);
Goto(bb9)
}
bb9 = {
_54 = Adt44::Variant0 { fld0: 9718019780431478777_u64,fld1: _47 };
_26 = [(-88_i8),(-121_i8),47_i8,(-74_i8),(-108_i8)];
_33 = _43.0;
_24.0 = _42.0;
Goto(bb10)
}
bb10 = {
_11 = _15;
_43.0 = _48;
Goto(bb11)
}
bb11 = {
_44.0 = _43.0;
_63 = 43_i8 ^ 53_i8;
_35 = _7;
_66 = (_24.0,);
_39 = !_44.1;
_35 = _7;
_51 = [_39,_39,_39,_44.1,_39,_44.1,_44.1];
_66 = (_35,);
_20 = !3200655353_u32;
_13 = _30 > _2;
Goto(bb12)
}
bb12 = {
_57 = 17076043188421440387_usize;
_64 = _21.2 * _53.2;
_12 = _14 ^ _11;
_21.2 = _64;
_12 = _13 & _3;
_53.2 = _36 + _21.2;
RET = [_63,_63,_63];
_58 = _9;
_20 = 823215708_u32;
_59 = _34;
_34 = _59;
_60 = Field::<u16>(Variant(_54, 0), 1);
_44 = (_33, _39, _9);
_68 = _64 + _64;
_47 = Field::<u16>(Variant(_54, 0), 1);
_16 = !(*_38);
_63 = 0_i8 << _39;
_50 = _63 as usize;
_59 = _63 as i128;
_1 = [_63,_63,_63];
_17 = core::ptr::addr_of!(_52);
_43.0 = -_44.0;
_46 = _23;
_28 = [_44.1,_44.1,_39,_44.1,_39,_39,_39];
RET = [_63,_63,_63];
_43.0 = _53.0 + _33;
_30 = !_16;
Goto(bb13)
}
bb13 = {
Call(_72 = dump_var(5_usize, 2_usize, Move(_2), 5_usize, Move(_5), 11_usize, Move(_11), 29_usize, Move(_29)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_72 = dump_var(5_usize, 12_usize, Move(_12), 66_usize, Move(_66), 41_usize, Move(_41), 24_usize, Move(_24)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_72 = dump_var(5_usize, 13_usize, Move(_13), 39_usize, Move(_39), 26_usize, Move(_26), 60_usize, Move(_60)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_72 = dump_var(5_usize, 9_usize, Move(_9), 19_usize, Move(_19), 23_usize, Move(_23), 50_usize, Move(_50)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_72 = dump_var(5_usize, 15_usize, Move(_15), 14_usize, Move(_14), 6_usize, Move(_6), 28_usize, Move(_28)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u8,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: u8,mut _6: u8,mut _7: bool,mut _8: isize,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: u8,mut _13: bool,mut _14: u8,mut _15: bool) -> isize {
mir! {
type RET = isize;
let _16: Adt49;
let _17: [i8; 3];
let _18: ();
let _19: ();
{
_5 = !_1;
_9 = !_3;
_12 = _1;
_11 = _15 & _13;
RET = !_2;
_9 = _8 != _2;
_3 = _4;
_14 = _6;
_15 = _3;
_17 = [(-114_i8),12_i8,95_i8];
_1 = 1320252160_i32 as u8;
_3 = !_10;
RET = _2 << _6;
_5 = !_12;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(6_usize, 14_usize, Move(_14), 17_usize, Move(_17), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(6_usize, 5_usize, Move(_5), 4_usize, Move(_4), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: [isize; 2],mut _3: u32,mut _4: isize,mut _5: bool,mut _6: u8,mut _7: [isize; 2]) -> u8 {
mir! {
type RET = u8;
let _8: isize;
let _9: [i8; 3];
let _10: Adt54;
let _11: Adt47;
let _12: Adt51;
let _13: [i8; 1];
let _14: [i8; 5];
let _15: (&'static u8, usize, isize, isize);
let _16: f32;
let _17: (char,);
let _18: [isize; 2];
let _19: ((*const u128,), (&'static u8,));
let _20: Adt44;
let _21: Adt55;
let _22: usize;
let _23: [i64; 7];
let _24: [i8; 3];
let _25: u64;
let _26: f32;
let _27: f32;
let _28: ();
let _29: ();
{
_2 = [_4,_1];
RET = _6 | _6;
_4 = _1 - _1;
_4 = _1 + _1;
_6 = RET;
_4 = _1 >> RET;
_2 = _7;
RET = _6 >> _4;
_8 = '\u{2bd31}' as isize;
_4 = (-1497198798_i32) as isize;
_8 = _1 & _1;
_8 = _1 + _1;
_5 = !false;
_2 = _7;
_1 = -_8;
_6 = _5 as u8;
_4 = -_8;
_8 = (-22527_i16) as isize;
_5 = false;
_7 = _2;
_1 = (-16454_i16) as isize;
RET = !_6;
RET = !_6;
_7 = [_4,_4];
_8 = 7020345341144557205_i64 as isize;
Call(_12 = fn8(_7, _7, _7, _7, _4, _3, _3, _4, _4, _7, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [_8,_4];
place!(Field::<f32>(Variant(_12, 1), 1)) = 269187398171372752704112784652147270885_u128 as f32;
RET = !_6;
_13 = [Field::<i8>(Variant(_12, 1), 2)];
_9 = [Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2)];
_8 = _4;
_3 = !4043851940_u32;
RET = _6;
Goto(bb2)
}
bb2 = {
_7 = [_4,_8];
RET = _6 ^ _6;
_14 = [Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2)];
_9 = [Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2)];
SetDiscriminant(_12, 2);
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).0 = 13207489353692670186_u64 as f64;
_14 = [94_i8,68_i8,(-79_i8),89_i8,61_i8];
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld2.2 = 1502165053325997002_usize as f32;
_13 = [(-76_i8)];
_15.1 = 23886_i16 as usize;
_15.3 = _8 ^ _8;
_15.1 = 3_usize * 7_usize;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld4 = 9396345613904209696_u64;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld5 = Field::<Adt46>(Variant(_12, 2), 5).fld4 as i32;
RET = Field::<Adt46>(Variant(_12, 2), 5).fld2.2 as u8;
_2 = [_15.3,_4];
match Field::<Adt46>(Variant(_12, 2), 5).fld4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9396345613904209696 => bb9,
_ => bb8
}
}
bb3 = {
_7 = [_8,_4];
place!(Field::<f32>(Variant(_12, 1), 1)) = 269187398171372752704112784652147270885_u128 as f32;
RET = !_6;
_13 = [Field::<i8>(Variant(_12, 1), 2)];
_9 = [Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2),Field::<i8>(Variant(_12, 1), 2)];
_8 = _4;
_3 = !4043851940_u32;
RET = _6;
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
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld2.1 = Field::<Adt46>(Variant(_12, 2), 5).fld5;
place!(Field::<u128>(Variant(_12, 2), 0)) = !331125922303815537534449532695453260829_u128;
_15.2 = '\u{be540}' as isize;
_14 = [(-101_i8),(-23_i8),(-43_i8),112_i8,(-4_i8)];
RET = Field::<u128>(Variant(_12, 2), 0) as u8;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).0 = 23114_u16 as f64;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld4 = 443615108850359168_u64 + 12231783994717384507_u64;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld0 = 12548_u16;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld2.0 = Field::<(f64, i32, f32)>(Variant(_12, 2), 4).0 - Field::<(f64, i32, f32)>(Variant(_12, 2), 4).0;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).0 = _4 as f64;
place!(Field::<u128>(Variant(_12, 2), 0)) = 97076684247032238721464204410893755809_u128 | 123402031219235717801015627430573849066_u128;
_17.0 = '\u{5b79e}';
_14 = [(-91_i8),(-82_i8),68_i8,78_i8,(-65_i8)];
_15.0 = &_6;
Call(place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).2 = fn14(Field::<(f64, i32, f32)>(Variant(_12, 2), 4).0, Field::<(f64, i32, f32)>(Variant(_12, 2), 4).0, Move(_15)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = !_4;
_18 = _7;
_20 = Adt44::Variant0 { fld0: Field::<Adt46>(Variant(_12, 2), 5).fld4,fld1: Field::<Adt46>(Variant(_12, 2), 5).fld0 };
_15.0 = &_6;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld0 = Field::<u16>(Variant(_20, 0), 1);
_15.1 = 6668222537553605146_usize;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).0 = Field::<Adt46>(Variant(_12, 2), 5).fld2.0;
_1 = -_4;
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld2.1 = Field::<Adt46>(Variant(_12, 2), 5).fld5 & Field::<Adt46>(Variant(_12, 2), 5).fld5;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).1 = Field::<Adt46>(Variant(_12, 2), 5).fld2.1;
_19.0.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_12, 2), 0)));
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld1 = Field::<u16>(Variant(_20, 0), 1) as f64;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)) = Field::<Adt46>(Variant(_12, 2), 5).fld2;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)) = (Field::<Adt46>(Variant(_12, 2), 5).fld1, Field::<Adt46>(Variant(_12, 2), 5).fld5, Field::<Adt46>(Variant(_12, 2), 5).fld2.2);
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7 = _18;
place!(Field::<[i64; 8]>(Variant(_12, 2), 1)) = [3667291842710010325_i64,1845707463530312367_i64,4668596709741700444_i64,(-6861022340527606162_i64),(-1697316632745142463_i64),887895975914885684_i64,5273105453766235775_i64,(-3629067837997548634_i64)];
_7 = [_1,_4];
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld4 = !Field::<u64>(Variant(_20, 0), 0);
place!(Field::<u128>(Variant(_12, 2), 0)) = Field::<(f64, i32, f32)>(Variant(_12, 2), 4).2 as u128;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)) = (Field::<Adt46>(Variant(_12, 2), 5).fld2.0, Field::<Adt46>(Variant(_12, 2), 5).fld5, Field::<Adt46>(Variant(_12, 2), 5).fld2.2);
_17.0 = '\u{6cd6a}';
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld2.0 = Field::<(f64, i32, f32)>(Variant(_12, 2), 4).0 * Field::<Adt46>(Variant(_12, 2), 5).fld1;
RET = _6;
_1 = RET as isize;
_21.fld2 = Field::<Adt46>(Variant(_12, 2), 5).fld2.0 as f32;
_2 = _18;
_24 = [(-24_i8),(-5_i8),87_i8];
_6 = 28_i8 as u8;
place!(Field::<Adt45>(Variant(_12, 2), 2)) = Adt45::Variant0 { fld0: _14,fld1: Field::<[i64; 8]>(Variant(_12, 2), 1),fld2: Field::<Adt46>(Variant(_12, 2), 5).fld0 };
place!(Field::<Adt46>(Variant(_12, 2), 5)).fld1 = -Field::<Adt46>(Variant(_12, 2), 5).fld2.0;
place!(Field::<(f64, i32, f32)>(Variant(_12, 2), 4)).2 = _21.fld2 * _21.fld2;
match Field::<u16>(Variant(Field::<Adt45>(Variant(_12, 2), 2), 0), 2) {
0 => bb5,
12548 => bb13,
_ => bb2
}
}
bb13 = {
_23 = [(-8811431577707950384_i64),(-4975765580611872758_i64),6552547349152894783_i64,(-3848798138054316809_i64),2277264642049483235_i64,(-8909783002860892316_i64),(-5328249419247612508_i64)];
place!(Field::<[i64; 8]>(Variant(place!(Field::<Adt45>(Variant(_12, 2), 2)), 0), 1)) = [(-3783718628328549246_i64),7890919265066659360_i64,(-3065652758870258893_i64),5997445949259125959_i64,(-8870175422235591855_i64),(-221861149140703512_i64),(-8511210008609783456_i64),3298120991691362109_i64];
place!(Field::<[i8; 5]>(Variant(place!(Field::<Adt45>(Variant(_12, 2), 2)), 0), 0)) = [109_i8,32_i8,121_i8,(-38_i8),(-111_i8)];
Goto(bb14)
}
bb14 = {
_21.fld4 = core::ptr::addr_of_mut!(_15.3);
SetDiscriminant(Field::<Adt45>(Variant(_12, 2), 2), 0);
_21.fld1 = (_19.0.0,);
_22 = _21.fld2 as usize;
_12 = Adt51::Variant1 { fld0: _5,fld1: _21.fld2,fld2: 86_i8 };
SetDiscriminant(_20, 0);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(7_usize, 14_usize, Move(_14), 5_usize, Move(_5), 22_usize, Move(_22), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: [isize; 2],mut _4: [isize; 2],mut _5: isize,mut _6: u32,mut _7: u32,mut _8: isize,mut _9: isize,mut _10: [isize; 2],mut _11: isize) -> Adt51 {
mir! {
type RET = Adt51;
let _12: i32;
let _13: bool;
let _14: (char,);
let _15: [i8; 5];
let _16: u8;
let _17: f64;
let _18: *mut i16;
let _19: char;
let _20: (f64, i64, u8);
let _21: Adt51;
let _22: [i8; 1];
let _23: (char,);
let _24: [i8; 5];
let _25: char;
let _26: char;
let _27: [bool; 5];
let _28: usize;
let _29: f64;
let _30: [isize; 2];
let _31: bool;
let _32: u128;
let _33: *const [bool; 5];
let _34: f32;
let _35: &'static u8;
let _36: *mut [isize; 2];
let _37: f64;
let _38: isize;
let _39: f64;
let _40: isize;
let _41: (char,);
let _42: f32;
let _43: isize;
let _44: [i8; 5];
let _45: i8;
let _46: [i8; 3];
let _47: Adt58;
let _48: (f64, i32, f32);
let _49: Adt44;
let _50: (*const u128,);
let _51: u64;
let _52: u64;
let _53: u32;
let _54: i128;
let _55: Adt53;
let _56: isize;
let _57: u64;
let _58: [isize; 2];
let _59: Adt57;
let _60: i8;
let _61: isize;
let _62: [isize; 2];
let _63: *const usize;
let _64: (char,);
let _65: ();
let _66: ();
{
_10 = _2;
_9 = -_5;
_10 = [_9,_5];
_12 = 1068710461_i32 & 295337294_i32;
_2 = [_11,_11];
_9 = _11 ^ _11;
_2 = [_8,_9];
_6 = _7 << _11;
_7 = 44713407027275285576350268177176198958_u128 as u32;
_14.0 = '\u{1c8eb}';
_5 = _11;
_12 = (-1540244710_i32);
_14 = ('\u{ffd76}',);
_4 = _1;
_13 = !true;
_9 = 16104100175308157351_u64 as isize;
_6 = !_7;
_7 = !_6;
_11 = _5 | _5;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607430227966746 => bb8,
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
_14 = ('\u{d5106}',);
_16 = 57_i8 as u8;
_9 = _11;
_16 = 253_u8;
_7 = _6 - _6;
_12 = (-178287614_i32) + 935103139_i32;
_7 = _6 + _6;
_15 = [(-38_i8),(-74_i8),(-112_i8),75_i8,(-33_i8)];
_14.0 = '\u{2c20d}';
_17 = (-38_i8) as f64;
_11 = 85963981623847830703419080824392916659_i128 as isize;
_15 = [(-120_i8),123_i8,(-114_i8),(-10_i8),53_i8];
_14.0 = '\u{68e16}';
_14.0 = '\u{62abb}';
_1 = _2;
_17 = _7 as f64;
_6 = !_7;
_13 = _5 < _5;
_3 = _1;
_7 = 5466486916859712294_i64 as u32;
_5 = -_8;
_9 = !_5;
_14 = ('\u{befc9}',);
_1 = [_9,_5];
Goto(bb9)
}
bb9 = {
_8 = -_9;
_7 = _6 + _6;
_14.0 = '\u{f559f}';
_12 = !(-211457708_i32);
_8 = _5 & _5;
_16 = 25_u8 ^ 40_u8;
_17 = _16 as f64;
_2 = _3;
Goto(bb10)
}
bb10 = {
_20.1 = _16 as i64;
_8 = _5;
_6 = _8 as u32;
_14.0 = '\u{102398}';
_23 = _14;
_23.0 = _14.0;
_20.2 = !_16;
_14.0 = _23.0;
_6 = _7 - _7;
_9 = _5 & _8;
_25 = _14.0;
_6 = _7 * _7;
_23 = (_25,);
_16 = _5 as u8;
_17 = (-8289_i16) as f64;
_20.0 = _17 * _17;
_20.2 = _7 as u8;
_10 = [_5,_5];
_1 = [_8,_9];
_12 = _16 as i32;
_14 = (_23.0,);
_12 = 1483360972_i32 | 837542570_i32;
Goto(bb11)
}
bb11 = {
_25 = _23.0;
_20.1 = _12 as i64;
_12 = -297497847_i32;
_16 = 18511_u16 as u8;
_20.1 = 2420011501209256966_i64;
match _20.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
2420011501209256966 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_14.0 = _25;
_27 = [_13,_13,_13,_13,_13];
_10 = [_5,_5];
_14 = (_25,);
_10 = [_9,_8];
_6 = 3_usize as u32;
_19 = _14.0;
_1 = [_9,_9];
_24 = _15;
_5 = _8;
_4 = [_9,_5];
_14.0 = _19;
_14 = (_25,);
_27 = [_13,_13,_13,_13,_13];
_22 = [(-7_i8)];
_22 = [(-108_i8)];
_24 = _15;
_28 = 3129769366509678911_usize;
_32 = _12 as u128;
_10 = [_8,_5];
_34 = _32 as f32;
Call(_7 = fn9(_1, _2, _10, _10, _3, _2, _8, _13, _3, _1, _8, _5, _10), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11 = !_5;
_33 = core::ptr::addr_of!(_27);
_11 = _8;
_20.0 = -_17;
_13 = true;
_12 = -(-603664186_i32);
_33 = core::ptr::addr_of!((*_33));
_20.2 = 3266133425476681023_u64 as u8;
_22 = [(-111_i8)];
_6 = _25 as u32;
_34 = _20.2 as f32;
_34 = 3523802861453409100_u64 as f32;
_16 = _20.2 >> _11;
_20.1 = 6502881540360313932_i64 & (-705131446583697968_i64);
_35 = &_16;
_32 = 103735476124063698850562130540722973982_u128 * 60300671036393139721996428918650615940_u128;
_28 = 6_usize >> _9;
_26 = _25;
_5 = _11;
Call(_21 = fn10(_9, (*_35), (*_33), _2, (*_33), _4, _11, Move(_35), _11, _2, _5, _10, _5, _11), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_36 = core::ptr::addr_of_mut!(_10);
_37 = _17 + _17;
_20 = (_37, 9128942028988813612_i64, _16);
_31 = !Field::<bool>(Variant(_21, 1), 0);
_35 = &_16;
_38 = _5 << _9;
_30 = _10;
_15 = [Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2)];
_22 = [Field::<i8>(Variant(_21, 1), 2)];
place!(Field::<f32>(Variant(_21, 1), 1)) = _34 * _34;
_23.0 = _14.0;
_6 = _32 as u32;
_40 = _38 ^ _11;
_20.0 = -_37;
_26 = _23.0;
_12 = -6514642_i32;
_19 = _14.0;
_23 = (_14.0,);
Goto(bb16)
}
bb16 = {
place!(Field::<i8>(Variant(_21, 1), 2)) = _12 as i8;
_33 = core::ptr::addr_of!((*_33));
_42 = Field::<f32>(Variant(_21, 1), 1) * _34;
_1 = [_11,_40];
_38 = !_8;
_29 = -_17;
_39 = _7 as f64;
_28 = !17916608044390937980_usize;
match _20.1 {
0 => bb17,
1 => bb18,
2 => bb19,
9128942028988813612 => bb21,
_ => bb20
}
}
bb17 = {
_20.1 = _16 as i64;
_8 = _5;
_6 = _8 as u32;
_14.0 = '\u{102398}';
_23 = _14;
_23.0 = _14.0;
_20.2 = !_16;
_14.0 = _23.0;
_6 = _7 - _7;
_9 = _5 & _8;
_25 = _14.0;
_6 = _7 * _7;
_23 = (_25,);
_16 = _5 as u8;
_17 = (-8289_i16) as f64;
_20.0 = _17 * _17;
_20.2 = _7 as u8;
_10 = [_5,_5];
_1 = [_8,_9];
_12 = _16 as i32;
_14 = (_23.0,);
_12 = 1483360972_i32 | 837542570_i32;
Goto(bb11)
}
bb18 = {
_14 = ('\u{d5106}',);
_16 = 57_i8 as u8;
_9 = _11;
_16 = 253_u8;
_7 = _6 - _6;
_12 = (-178287614_i32) + 935103139_i32;
_7 = _6 + _6;
_15 = [(-38_i8),(-74_i8),(-112_i8),75_i8,(-33_i8)];
_14.0 = '\u{2c20d}';
_17 = (-38_i8) as f64;
_11 = 85963981623847830703419080824392916659_i128 as isize;
_15 = [(-120_i8),123_i8,(-114_i8),(-10_i8),53_i8];
_14.0 = '\u{68e16}';
_14.0 = '\u{62abb}';
_1 = _2;
_17 = _7 as f64;
_6 = !_7;
_13 = _5 < _5;
_3 = _1;
_7 = 5466486916859712294_i64 as u32;
_5 = -_8;
_9 = !_5;
_14 = ('\u{befc9}',);
_1 = [_9,_5];
Goto(bb9)
}
bb19 = {
_25 = _23.0;
_20.1 = _12 as i64;
_12 = -297497847_i32;
_16 = 18511_u16 as u8;
_20.1 = 2420011501209256966_i64;
match _20.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
2420011501209256966 => bb13,
_ => bb12
}
}
bb20 = {
Return()
}
bb21 = {
_26 = _14.0;
_43 = _7 as isize;
_14 = (_26,);
(*_36) = [_11,_5];
_26 = _19;
_16 = !_20.2;
_35 = &_20.2;
_6 = _7;
_15 = [Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2),Field::<i8>(Variant(_21, 1), 2)];
_27 = [Field::<bool>(Variant(_21, 1), 0),Field::<bool>(Variant(_21, 1), 0),_31,Field::<bool>(Variant(_21, 1), 0),Field::<bool>(Variant(_21, 1), 0)];
_40 = _11 ^ _8;
_10 = _2;
(*_36) = [_9,_38];
SetDiscriminant(_21, 0);
_19 = _25;
_14.0 = _23.0;
_10 = [_5,_11];
place!(Field::<Adt46>(Variant(_21, 0), 5)).fld2.0 = _7 as f64;
_40 = _5 - _43;
_14 = (_19,);
Goto(bb22)
}
bb22 = {
_48 = (Field::<Adt46>(Variant(_21, 0), 5).fld2.0, _12, _42);
_13 = _16 > _16;
(*_33) = [_31,_13,_13,_13,_31];
_1 = [_11,_5];
_50.0 = core::ptr::addr_of!(_32);
place!(Field::<Adt46>(Variant(_21, 0), 5)).fld0 = 35002_u16;
_12 = _48.1;
RET = Adt51::Variant1 { fld0: _31,fld1: _34,fld2: (-74_i8) };
match _20.1 {
0 => bb13,
1 => bb7,
9128942028988813612 => bb23,
_ => bb12
}
}
bb23 = {
_12 = !_48.1;
_46 = [72_i8,97_i8,(-41_i8)];
_12 = _48.1;
place!(Field::<[i8; 3]>(Variant(_21, 0), 4)) = [(-57_i8),(-114_i8),(-24_i8)];
_14.0 = _25;
_42 = -_34;
_43 = (-50_i8) as isize;
_22 = [(-60_i8)];
_24 = _15;
_40 = _16 as isize;
place!(Field::<Adt46>(Variant(_21, 0), 5)).fld0 = _20.1 as u16;
_43 = _40 - _38;
place!(Field::<Adt46>(Variant(_21, 0), 5)).fld4 = !6368832975054696237_u64;
_19 = _23.0;
Goto(bb24)
}
bb24 = {
_51 = _48.1 as u64;
_41.0 = _25;
_51 = Field::<Adt46>(Variant(_21, 0), 5).fld4 + Field::<Adt46>(Variant(_21, 0), 5).fld4;
_58 = [_5,_5];
_58 = [_43,_40];
place!(Field::<bool>(Variant(RET, 1), 0)) = _13;
_22 = [(-86_i8)];
_14 = (_41.0,);
_7 = _48.1 as u32;
place!(Field::<Adt46>(Variant(_21, 0), 5)).fld4 = _51 - _51;
_48.2 = _34;
place!(Field::<i8>(Variant(RET, 1), 2)) = (-17530_i16) as i8;
_60 = !Field::<i8>(Variant(RET, 1), 2);
place!(Field::<[i8; 1]>(Variant(_21, 0), 3)) = [Field::<i8>(Variant(RET, 1), 2)];
_34 = -Field::<f32>(Variant(RET, 1), 1);
place!(Field::<(*const u128,)>(Variant(_21, 0), 6)) = (_50.0,);
Goto(bb25)
}
bb25 = {
Call(_65 = dump_var(8_usize, 30_usize, Move(_30), 10_usize, Move(_10), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_65 = dump_var(8_usize, 12_usize, Move(_12), 23_usize, Move(_23), 14_usize, Move(_14), 25_usize, Move(_25)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_65 = dump_var(8_usize, 32_usize, Move(_32), 38_usize, Move(_38), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_65 = dump_var(8_usize, 1_usize, Move(_1), 27_usize, Move(_27), 13_usize, Move(_13), 24_usize, Move(_24)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_65 = dump_var(8_usize, 60_usize, Move(_60), 66_usize, _66, 66_usize, _66, 66_usize, _66), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: [isize; 2],mut _4: [isize; 2],mut _5: [isize; 2],mut _6: [isize; 2],mut _7: isize,mut _8: bool,mut _9: [isize; 2],mut _10: [isize; 2],mut _11: isize,mut _12: isize,mut _13: [isize; 2]) -> u32 {
mir! {
type RET = u32;
let _14: ();
let _15: ();
{
RET = 2567164610_u32 << _12;
_5 = [_7,_7];
_9 = [_7,_11];
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(9_usize, 8_usize, Move(_8), 1_usize, Move(_1), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(9_usize, 12_usize, Move(_12), 13_usize, Move(_13), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: u8,mut _3: [bool; 5],mut _4: [isize; 2],mut _5: [bool; 5],mut _6: [isize; 2],mut _7: isize,mut _8: &'static u8,mut _9: isize,mut _10: [isize; 2],mut _11: isize,mut _12: [isize; 2],mut _13: isize,mut _14: isize) -> Adt51 {
mir! {
type RET = Adt51;
let _15: (f64, i32, f32);
let _16: *const (*mut i16, u16, i128, char, *mut i16);
let _17: (char,);
let _18: i32;
let _19: i64;
let _20: f64;
let _21: [isize; 2];
let _22: char;
let _23: [i8; 5];
let _24: [i8; 1];
let _25: Adt43;
let _26: Adt42;
let _27: ();
let _28: ();
{
_10 = _6;
_15.0 = (-24326_i16) as f64;
_11 = _14;
_4 = _6;
_7 = _9;
_7 = _9;
_3 = _5;
_5 = _3;
_13 = true as isize;
_18 = 26823_i16 as i32;
_13 = _1 + _11;
_6 = _12;
_8 = &(*_8);
_15.2 = (-61097474887472770552137705068119036049_i128) as f32;
_1 = _7;
_11 = !_9;
_4 = _10;
_2 = (*_8);
Call(_15.2 = fn11(_6, _4, _1, _6, _13, _10, _7, _11, _7, _13, _2, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = Adt51::Variant1 { fld0: false,fld1: _15.2,fld2: 36_i8 };
_20 = _15.0 - _15.0;
_1 = !_13;
_10 = _12;
RET = Adt51::Variant1 { fld0: true,fld1: _15.2,fld2: 23_i8 };
_15.2 = -Field::<f32>(Variant(RET, 1), 1);
_10 = [_1,_14];
_15.2 = Field::<f32>(Variant(RET, 1), 1) * Field::<f32>(Variant(RET, 1), 1);
_22 = '\u{bbb87}';
_15.1 = _18;
_4 = [_14,_9];
_17 = (_22,);
_6 = [_13,_1];
place!(Field::<f32>(Variant(RET, 1), 1)) = _15.2;
_2 = (-111_i8) as u8;
_11 = _1;
_21 = [_14,_1];
place!(Field::<f32>(Variant(RET, 1), 1)) = _15.2 * _15.2;
Call(_7 = fn12(_13, _1, _6, _12, _21, _14, (*_8), _12, (*_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<f32>(Variant(RET, 1), 1)) = _15.2 + _15.2;
RET = Adt51::Variant1 { fld0: true,fld1: _15.2,fld2: 34_i8 };
place!(Field::<bool>(Variant(RET, 1), 0)) = !true;
_18 = _15.1 & _15.1;
place!(Field::<i8>(Variant(RET, 1), 2)) = (-2694304041637125326_i64) as i8;
place!(Field::<bool>(Variant(RET, 1), 0)) = _13 == _11;
_10 = [_1,_14];
_20 = -_15.0;
_10 = [_9,_11];
_3 = _5;
_22 = _17.0;
_4 = _21;
place!(Field::<f32>(Variant(RET, 1), 1)) = (*_8) as f32;
_15.2 = -Field::<f32>(Variant(RET, 1), 1);
_19 = 1622327623130895136_u64 as i64;
Goto(bb3)
}
bb3 = {
Call(_27 = dump_var(10_usize, 4_usize, Move(_4), 22_usize, Move(_22), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_27 = dump_var(10_usize, 14_usize, Move(_14), 19_usize, Move(_19), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_27 = dump_var(10_usize, 6_usize, Move(_6), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: isize,mut _4: [isize; 2],mut _5: isize,mut _6: [isize; 2],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: u8,mut _12: isize) -> f32 {
mir! {
type RET = f32;
let _13: (f64, i64, u8);
let _14: [i8; 5];
let _15: char;
let _16: char;
let _17: (f64, i32, f32);
let _18: char;
let _19: f64;
let _20: [i8; 1];
let _21: Adt44;
let _22: u8;
let _23: [usize; 5];
let _24: Adt55;
let _25: [i8; 1];
let _26: Adt54;
let _27: Adt58;
let _28: isize;
let _29: u64;
let _30: *const u128;
let _31: isize;
let _32: u8;
let _33: f32;
let _34: Adt44;
let _35: i128;
let _36: isize;
let _37: i64;
let _38: ();
let _39: ();
{
RET = (-125402923363812861681564039916406629170_i128) as f32;
_1 = [_8,_9];
_4 = [_9,_10];
_6 = [_9,_7];
_13.0 = _9 as f64;
_13.2 = (-2178048472967011777_i64) as u8;
_6 = _1;
_3 = _10;
_2 = _1;
_4 = [_3,_10];
_6 = [_3,_3];
Goto(bb1)
}
bb1 = {
_9 = 2777827951245319806_i64 as isize;
_12 = _3;
_8 = !_10;
_10 = _3;
Goto(bb2)
}
bb2 = {
_4 = _6;
_6 = [_5,_12];
_1 = [_7,_12];
_11 = _13.2 & _13.2;
_13.1 = 2453705533290355729_i64 << _12;
_16 = '\u{7b1f6}';
_17.0 = _13.0 - _13.0;
_15 = _16;
_6 = _4;
_17.0 = _13.0 + _13.0;
_1 = [_7,_7];
_12 = 1_usize as isize;
_13 = (_17.0, 1023167610590532885_i64, _11);
_13.1 = 1111668566079871252_i64 & (-3612549601638890305_i64);
_18 = _16;
_13.0 = _17.0;
_15 = _18;
_19 = -_13.0;
_13.0 = _17.0 - _17.0;
_13.2 = !_11;
_17.2 = RET * RET;
_13.2 = _11 << _10;
_19 = _13.0 - _17.0;
_13.1 = 2537284391_u32 as i64;
_3 = !_7;
_17.2 = RET * RET;
_16 = _15;
RET = _13.1 as f32;
Goto(bb3)
}
bb3 = {
_20 = [(-15_i8)];
RET = _17.2;
_13 = (_19, (-2615105045035384832_i64), _11);
RET = 46_i8 as f32;
_14 = [(-82_i8),(-45_i8),(-83_i8),115_i8,13_i8];
_19 = _17.0 - _13.0;
_17 = (_13.0, (-235351028_i32), RET);
_8 = _17.0 as isize;
_14 = [22_i8,74_i8,(-23_i8),97_i8,88_i8];
_1 = [_8,_3];
_22 = _8 as u8;
_18 = _16;
_20 = [64_i8];
_18 = _15;
_7 = 2701046196_u32 as isize;
_24.fld4 = core::ptr::addr_of_mut!(_9);
_21 = Adt44::Variant0 { fld0: 9878407240079333540_u64,fld1: 22196_u16 };
RET = _17.2;
_15 = _16;
_4 = [_3,_10];
_18 = _16;
_22 = _17.1 as u8;
_5 = _8 - _10;
_13.0 = _17.0 + _17.0;
_1 = [_10,_3];
_2 = [_10,_8];
match _17.1 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431532860428 => bb8,
_ => bb7
}
}
bb4 = {
_4 = _6;
_6 = [_5,_12];
_1 = [_7,_12];
_11 = _13.2 & _13.2;
_13.1 = 2453705533290355729_i64 << _12;
_16 = '\u{7b1f6}';
_17.0 = _13.0 - _13.0;
_15 = _16;
_6 = _4;
_17.0 = _13.0 + _13.0;
_1 = [_7,_7];
_12 = 1_usize as isize;
_13 = (_17.0, 1023167610590532885_i64, _11);
_13.1 = 1111668566079871252_i64 & (-3612549601638890305_i64);
_18 = _16;
_13.0 = _17.0;
_15 = _18;
_19 = -_13.0;
_13.0 = _17.0 - _17.0;
_13.2 = !_11;
_17.2 = RET * RET;
_13.2 = _11 << _10;
_19 = _13.0 - _17.0;
_13.1 = 2537284391_u32 as i64;
_3 = !_7;
_17.2 = RET * RET;
_16 = _15;
RET = _13.1 as f32;
Goto(bb3)
}
bb5 = {
_9 = 2777827951245319806_i64 as isize;
_12 = _3;
_8 = !_10;
_10 = _3;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_24.fld4 = core::ptr::addr_of_mut!(_10);
_17.0 = _22 as f64;
_17.1 = 1939788663_i32;
_7 = _8 + _5;
_20 = [81_i8];
_13.2 = !_22;
place!(Field::<u64>(Variant(_21, 0), 0)) = 14194704776046812561_u64;
_12 = _3;
_27 = Adt58::Variant2 { fld0: _13,fld1: _6 };
Call(place!(Field::<(f64, i64, u8)>(Variant(_27, 2), 0)).0 = core::intrinsics::transmute(_13.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = _3;
_17.2 = RET;
match _13.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb7,
340282366920938463460759502386732826624 => bb11,
_ => bb10
}
}
bb10 = {
_4 = _6;
_6 = [_5,_12];
_1 = [_7,_12];
_11 = _13.2 & _13.2;
_13.1 = 2453705533290355729_i64 << _12;
_16 = '\u{7b1f6}';
_17.0 = _13.0 - _13.0;
_15 = _16;
_6 = _4;
_17.0 = _13.0 + _13.0;
_1 = [_7,_7];
_12 = 1_usize as isize;
_13 = (_17.0, 1023167610590532885_i64, _11);
_13.1 = 1111668566079871252_i64 & (-3612549601638890305_i64);
_18 = _16;
_13.0 = _17.0;
_15 = _18;
_19 = -_13.0;
_13.0 = _17.0 - _17.0;
_13.2 = !_11;
_17.2 = RET * RET;
_13.2 = _11 << _10;
_19 = _13.0 - _17.0;
_13.1 = 2537284391_u32 as i64;
_3 = !_7;
_17.2 = RET * RET;
_16 = _15;
RET = _13.1 as f32;
Goto(bb3)
}
bb11 = {
_10 = _5 * _3;
_14 = [(-79_i8),125_i8,(-105_i8),(-65_i8),(-84_i8)];
_29 = Field::<u64>(Variant(_21, 0), 0);
_28 = !_7;
_12 = _28 << _3;
_17 = (_13.0, 1622482917_i32, RET);
_25 = [2_i8];
_18 = _16;
_19 = -_17.0;
_14 = [59_i8,(-42_i8),(-36_i8),51_i8,(-79_i8)];
_23 = [2_usize,3_usize,0_usize,1183054171221722315_usize,4_usize];
_13.0 = _19;
_13.0 = 45752_u16 as f64;
place!(Field::<u64>(Variant(_21, 0), 0)) = _29 & _29;
_22 = !Field::<(f64, i64, u8)>(Variant(_27, 2), 0).2;
place!(Field::<[isize; 2]>(Variant(_27, 2), 1)) = [_7,_28];
match Field::<(f64, i64, u8)>(Variant(_27, 2), 0).1 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb9,
340282366920938463460759502386732826624 => bb12,
_ => bb6
}
}
bb12 = {
_24.fld5 = Field::<u64>(Variant(_21, 0), 0);
Goto(bb13)
}
bb13 = {
_5 = -_7;
RET = -_17.2;
_10 = _18 as isize;
_13 = Field::<(f64, i64, u8)>(Variant(_27, 2), 0);
_28 = _12 | _8;
place!(Field::<(f64, i64, u8)>(Variant(_27, 2), 0)) = (_17.0, _13.1, _22);
place!(Field::<(f64, i64, u8)>(Variant(_27, 2), 0)).1 = _13.1;
_5 = _12;
_25 = [(-13_i8)];
_32 = _22 >> _13.2;
_17.2 = -RET;
place!(Field::<u16>(Variant(_21, 0), 1)) = 55931_u16;
_35 = 49332886532268425445853429174479886546_i128 | 90510469793178969695623066862670325269_i128;
place!(Field::<(f64, i64, u8)>(Variant(_27, 2), 0)).0 = _19;
_28 = _3;
place!(Field::<u64>(Variant(_21, 0), 0)) = !_29;
_34 = Move(_21);
_21 = Move(_34);
_19 = _13.0 - _13.0;
_24.fld2 = _17.1 as f32;
_32 = 10806_i16 as u8;
match _17.1 {
1622482917 => bb15,
_ => bb14
}
}
bb14 = {
_24.fld4 = core::ptr::addr_of_mut!(_10);
_17.0 = _22 as f64;
_17.1 = 1939788663_i32;
_7 = _8 + _5;
_20 = [81_i8];
_13.2 = !_22;
place!(Field::<u64>(Variant(_21, 0), 0)) = 14194704776046812561_u64;
_12 = _3;
_27 = Adt58::Variant2 { fld0: _13,fld1: _6 };
Call(place!(Field::<(f64, i64, u8)>(Variant(_27, 2), 0)).0 = core::intrinsics::transmute(_13.1), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_28 = !_7;
_5 = -_12;
_21 = Adt44::Variant0 { fld0: _24.fld5,fld1: 48906_u16 };
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(11_usize, 5_usize, Move(_5), 32_usize, Move(_32), 11_usize, Move(_11), 35_usize, Move(_35)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(11_usize, 4_usize, Move(_4), 12_usize, Move(_12), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(11_usize, 3_usize, Move(_3), 16_usize, Move(_16), 28_usize, Move(_28), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: [isize; 2],mut _4: [isize; 2],mut _5: [isize; 2],mut _6: isize,mut _7: u8,mut _8: [isize; 2],mut _9: u8) -> isize {
mir! {
type RET = isize;
let _10: (f64, i64, u8);
let _11: isize;
let _12: usize;
let _13: f32;
let _14: Adt52;
let _15: Adt50;
let _16: isize;
let _17: bool;
let _18: i64;
let _19: i8;
let _20: ();
let _21: ();
{
_4 = [_1,_2];
_5 = _8;
_7 = !_9;
RET = 10960_i16 as isize;
Goto(bb1)
}
bb1 = {
_3 = [_2,_1];
_3 = [_1,_1];
_6 = '\u{c3710}' as isize;
_8 = [_1,_1];
_7 = !_9;
_8 = [_1,_1];
_2 = false as isize;
_10.1 = (-7954117735057517471_i64);
_5 = [_1,_1];
_4 = [_1,_1];
_9 = (-13582_i16) as u8;
Call(_8 = fn13(_5, _1, _1, _3, _5, _1, _5, _4, _1, _4, _1, _4, _3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _8;
_10.2 = _7 << _1;
_5 = [_1,_1];
_2 = !_1;
_10.2 = _7 - _7;
_13 = 156055249261705984696694524009533400829_u128 as f32;
_6 = _2;
_12 = 3_usize;
_10.2 = !_7;
_3 = _8;
_8 = [_1,_1];
_4 = [_2,_6];
_6 = (-456911328_i32) as isize;
_10.0 = 13784_i16 as f64;
_11 = _1 | _1;
_13 = _10.2 as f32;
match _12 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb6,
_ => bb5
}
}
bb3 = {
_3 = [_2,_1];
_3 = [_1,_1];
_6 = '\u{c3710}' as isize;
_8 = [_1,_1];
_7 = !_9;
_8 = [_1,_1];
_2 = false as isize;
_10.1 = (-7954117735057517471_i64);
_5 = [_1,_1];
_4 = [_1,_1];
_9 = (-13582_i16) as u8;
Call(_8 = fn13(_5, _1, _1, _3, _5, _1, _5, _4, _1, _4, _1, _4, _3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_9 = _7 >> _11;
_10.0 = 49755_u16 as f64;
_11 = 932197454_u32 as isize;
_4 = _3;
_10.0 = 238205311_u32 as f64;
_10.2 = _7 + _9;
_10.0 = 90501333568595641465940424446942468471_i128 as f64;
RET = 325132175313993308006021015118662335192_u128 as isize;
_6 = -_1;
_6 = _1 >> _9;
_1 = _2 + _6;
_2 = !_1;
_12 = 40376217319535806434440635410291361901_i128 as usize;
_1 = -_6;
_4 = [_2,_1];
_13 = 179052218243239401242937587485080045845_u128 as f32;
_15.fld0 = [23_i8,(-124_i8),2_i8];
RET = (-127_i8) as isize;
_11 = _1 << _2;
_1 = _2 | _2;
_16 = _1;
_9 = _7 << _1;
RET = -_11;
_17 = _2 >= _2;
_3 = [_16,_1];
_11 = !_1;
_17 = !false;
Goto(bb7)
}
bb7 = {
Call(_20 = dump_var(12_usize, 9_usize, Move(_9), 3_usize, Move(_3), 8_usize, Move(_8), 16_usize, Move(_16)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_20 = dump_var(12_usize, 2_usize, Move(_2), 11_usize, Move(_11), 21_usize, _21, 21_usize, _21), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [isize; 2],mut _2: isize,mut _3: isize,mut _4: [isize; 2],mut _5: [isize; 2],mut _6: isize,mut _7: [isize; 2],mut _8: [isize; 2],mut _9: isize,mut _10: [isize; 2],mut _11: isize,mut _12: [isize; 2],mut _13: [isize; 2],mut _14: [isize; 2]) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _15: Adt45;
let _16: [usize; 5];
let _17: [i64; 8];
let _18: ();
let _19: ();
{
_7 = [_2,_9];
_11 = !_3;
RET = _10;
_2 = 175352376_u32 as isize;
_6 = 1667369054_i32 as isize;
_13 = [_11,_9];
_13 = [_11,_11];
_10 = RET;
RET = [_9,_3];
_14 = [_9,_11];
_10 = [_3,_11];
_3 = -_9;
_12 = _7;
_11 = _3 << _9;
_13 = [_11,_3];
_4 = [_11,_9];
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(13_usize, 12_usize, Move(_12), 11_usize, Move(_11), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(13_usize, 9_usize, Move(_9), 3_usize, Move(_3), 7_usize, Move(_7), 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: f64,mut _2: f64,mut _3: (&'static u8, usize, isize, isize)) -> f32 {
mir! {
type RET = f32;
let _4: ();
let _5: ();
{
_3.3 = _3.2;
_3.1 = !13216783136737196902_usize;
_1 = _2 - _2;
_1 = _2 * _2;
RET = _1 as f32;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [isize; 2],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [isize; 2],mut _7: bool,mut _8: bool,mut _9: isize) -> (char,) {
mir! {
type RET = (char,);
let _10: i32;
let _11: (f64, i64, u8);
let _12: [i8; 5];
let _13: i32;
let _14: i64;
let _15: f64;
let _16: u128;
let _17: isize;
let _18: Adt46;
let _19: isize;
let _20: f32;
let _21: [isize; 2];
let _22: i16;
let _23: Adt55;
let _24: (f64, i32, f32);
let _25: *const [bool; 5];
let _26: char;
let _27: u32;
let _28: [usize; 5];
let _29: char;
let _30: [usize; 5];
let _31: *const [bool; 5];
let _32: [i8; 3];
let _33: i32;
let _34: f64;
let _35: [i64; 7];
let _36: ();
let _37: ();
{
_5 = _2;
RET.0 = '\u{f8666}';
_11.2 = 199_u8 - 183_u8;
_2 = _8 | _5;
_3 = _4;
_11.0 = 777323659_u32 as f64;
_9 = 68_isize;
_5 = _4;
_6 = [_9,_9];
_10 = (-617698690_i32) >> _11.2;
_10 = !(-807919105_i32);
_9 = -9223372036854775807_isize;
Call(_11.1 = core::intrinsics::transmute(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = '\u{47f67}';
_13 = _10 * _10;
_7 = !_3;
_2 = _3 | _5;
_11.0 = 1050367900_u32 as f64;
_18.fld2.0 = -_11.0;
_4 = _3;
RET.0 = '\u{457c6}';
Call(_18.fld2.1 = core::intrinsics::bswap(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11.0 = _18.fld2.0;
_12 = [15_i8,(-51_i8),94_i8,113_i8,(-5_i8)];
_15 = _11.0 - _11.0;
_18.fld2.2 = 3890829743_u32 as f32;
_18.fld4 = _18.fld2.0 as u64;
_4 = !_2;
_15 = -_11.0;
Call(_18.fld0 = core::intrinsics::bswap(18843_u16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = ('\u{b6ca8}',);
_4 = _5;
RET = ('\u{518c7}',);
_18.fld2.1 = (-68289207469339065738882681324282131159_i128) as i32;
_18.fld5 = (-11397802118219484072762755294360231986_i128) as i32;
_19 = !_9;
RET.0 = '\u{f0028}';
_18.fld3 = core::ptr::addr_of_mut!(_22);
_23.fld5 = _7 as u64;
_23.fld1.0 = core::ptr::addr_of!(_16);
_19 = !_9;
_8 = _7;
_20 = -_18.fld2.2;
_17 = (-67_i8) as isize;
_20 = _18.fld2.2 + _18.fld2.2;
_18.fld2.1 = _13 - _13;
_11.1 = 8830542574193851205_i64;
_16 = 92346291481076230467639809757851196649_u128;
_23.fld4 = core::ptr::addr_of_mut!(_17);
_20 = _18.fld2.2 - _18.fld2.2;
_12 = [67_i8,(-12_i8),(-49_i8),(-17_i8),(-114_i8)];
_18.fld2.0 = _15;
RET.0 = '\u{5ba98}';
_16 = 296045384935950520707401259798022478260_u128 << _23.fld5;
_8 = _4;
_11.0 = _18.fld2.0 + _18.fld2.0;
_18.fld4 = !_23.fld5;
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
8830542574193851205 => bb8,
_ => bb7
}
}
bb4 = {
_11.0 = _18.fld2.0;
_12 = [15_i8,(-51_i8),94_i8,113_i8,(-5_i8)];
_15 = _11.0 - _11.0;
_18.fld2.2 = 3890829743_u32 as f32;
_18.fld4 = _18.fld2.0 as u64;
_4 = !_2;
_15 = -_11.0;
Call(_18.fld0 = core::intrinsics::bswap(18843_u16), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET.0 = '\u{47f67}';
_13 = _10 * _10;
_7 = !_3;
_2 = _3 | _5;
_11.0 = 1050367900_u32 as f64;
_18.fld2.0 = -_11.0;
_4 = _3;
RET.0 = '\u{457c6}';
Call(_18.fld2.1 = core::intrinsics::bswap(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_23.fld2 = _20;
_22 = 10688_i16 | (-24413_i16);
_23.fld1.0 = core::ptr::addr_of!(_16);
_23.fld1.0 = core::ptr::addr_of!(_16);
_18.fld2.0 = _11.0 * _11.0;
_16 = !159886965160545536009837414694391377991_u128;
_24 = (_11.0, _13, _20);
_18.fld3 = core::ptr::addr_of_mut!(_22);
Goto(bb9)
}
bb9 = {
RET = ('\u{2e867}',);
_22 = 4138_i16 & (-25899_i16);
_15 = _18.fld2.0;
_18.fld5 = _24.1;
_18.fld2.2 = _24.2;
_10 = _24.1;
_23.fld4 = core::ptr::addr_of_mut!(_17);
_15 = _18.fld2.0 + _18.fld2.0;
_23.fld5 = _22 as u64;
_11 = (_15, (-5935182254446862400_i64), 112_u8);
_24 = _18.fld2;
_24 = _18.fld2;
_16 = 27290_u16 as u128;
_22 = 7_usize as i16;
_29 = RET.0;
_12 = [(-35_i8),(-44_i8),127_i8,115_i8,(-35_i8)];
_18.fld1 = _15 * _11.0;
_5 = !_7;
_21 = [_17,_17];
_11.0 = _9 as f64;
_18.fld1 = _15 * _15;
match _11.2 {
0 => bb10,
1 => bb11,
112 => bb13,
_ => bb12
}
}
bb10 = {
RET = ('\u{b6ca8}',);
_4 = _5;
RET = ('\u{518c7}',);
_18.fld2.1 = (-68289207469339065738882681324282131159_i128) as i32;
_18.fld5 = (-11397802118219484072762755294360231986_i128) as i32;
_19 = !_9;
RET.0 = '\u{f0028}';
_18.fld3 = core::ptr::addr_of_mut!(_22);
_23.fld5 = _7 as u64;
_23.fld1.0 = core::ptr::addr_of!(_16);
_19 = !_9;
_8 = _7;
_20 = -_18.fld2.2;
_17 = (-67_i8) as isize;
_20 = _18.fld2.2 + _18.fld2.2;
_18.fld2.1 = _13 - _13;
_11.1 = 8830542574193851205_i64;
_16 = 92346291481076230467639809757851196649_u128;
_23.fld4 = core::ptr::addr_of_mut!(_17);
_20 = _18.fld2.2 - _18.fld2.2;
_12 = [67_i8,(-12_i8),(-49_i8),(-17_i8),(-114_i8)];
_18.fld2.0 = _15;
RET.0 = '\u{5ba98}';
_16 = 296045384935950520707401259798022478260_u128 << _23.fld5;
_8 = _4;
_11.0 = _18.fld2.0 + _18.fld2.0;
_18.fld4 = !_23.fld5;
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
8830542574193851205 => bb8,
_ => bb7
}
}
bb11 = {
_11.0 = _18.fld2.0;
_12 = [15_i8,(-51_i8),94_i8,113_i8,(-5_i8)];
_15 = _11.0 - _11.0;
_18.fld2.2 = 3890829743_u32 as f32;
_18.fld4 = _18.fld2.0 as u64;
_4 = !_2;
_15 = -_11.0;
Call(_18.fld0 = core::intrinsics::bswap(18843_u16), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_24.1 = _18.fld5;
_5 = _8 != _4;
_23.fld5 = _18.fld4;
_23.fld1.0 = core::ptr::addr_of!(_16);
_18.fld2.0 = _18.fld1 - _15;
_30 = [2890484176442640610_usize,1_usize,8899648189760118176_usize,0_usize,1190021465092145245_usize];
_12 = [(-63_i8),39_i8,(-30_i8),24_i8,71_i8];
_24.0 = -_18.fld2.0;
_6 = [_19,_17];
_13 = _18.fld5 >> _24.1;
_14 = !_11.1;
_24.1 = !_18.fld2.1;
RET = (_29,);
_28 = _30;
_4 = !_5;
_19 = _17;
_16 = !305490874516321090685808331951556110721_u128;
_2 = !_4;
_11.1 = -_14;
_8 = _4;
_12 = [38_i8,(-106_i8),86_i8,(-15_i8),45_i8];
_20 = -_18.fld2.2;
_23.fld5 = _18.fld4 - _18.fld4;
_24.0 = _18.fld1;
_18.fld5 = _13 & _24.1;
_18.fld5 = _13 >> _23.fld5;
_5 = _7;
Call(_27 = core::intrinsics::transmute(_24.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_24.1 = -_18.fld5;
_11.2 = 83_u8;
_34 = _18.fld2.0;
_29 = RET.0;
_26 = RET.0;
_24.2 = _18.fld2.2;
_11 = (_18.fld1, _14, 45_u8);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(15_usize, 9_usize, Move(_9), 16_usize, Move(_16), 29_usize, Move(_29), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(15_usize, 6_usize, Move(_6), 26_usize, Move(_26), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(15_usize, 7_usize, Move(_7), 28_usize, Move(_28), 21_usize, Move(_21), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [bool; 5],mut _2: bool,mut _3: Adt50,mut _4: [i8; 3],mut _5: Adt43,mut _6: bool,mut _7: Adt50,mut _8: bool,mut _9: [i8; 3]) -> Adt50 {
mir! {
type RET = Adt50;
let _10: *mut isize;
let _11: [isize; 2];
let _12: u32;
let _13: [i64; 7];
let _14: Adt42;
let _15: u16;
let _16: ();
let _17: ();
{
place!(Field::<u8>(Variant(_5, 0), 1)) = 183_u8 | 6_u8;
RET.fld0 = [(-34_i8),9_i8,64_i8];
_6 = _2;
_7 = Adt50 { fld0: RET.fld0 };
match Field::<i16>(Variant(_5, 0), 0) {
23600 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3 = _7;
_4 = _9;
RET.fld0 = [(-94_i8),23_i8,(-57_i8)];
Goto(bb3)
}
bb3 = {
_11 = [9223372036854775807_isize,9223372036854775807_isize];
_3.fld0 = [(-78_i8),5_i8,96_i8];
_11 = [9223372036854775807_isize,(-34_isize)];
_12 = 2770176696_u32;
_5 = Adt43::Variant0 { fld0: (-21143_i16),fld1: 126_u8 };
RET = _3;
_3.fld0 = [(-72_i8),(-113_i8),46_i8];
_4 = _9;
_11 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_13 = [(-892670351212694878_i64),(-2189102816104441183_i64),(-9111967959172014119_i64),(-3334994041731652811_i64),(-5701549731187554488_i64),9078857064468902638_i64,(-1818123891290575777_i64)];
_7 = _3;
_7 = _3;
_13 = [(-9158804027875911139_i64),(-8054748940595763588_i64),4118801143855448857_i64,823508192423626666_i64,3842135579893238064_i64,(-2026104161120147323_i64),(-4092707012558998220_i64)];
_13 = [644993465262523504_i64,244352493629124080_i64,7118034624357045184_i64,(-4949949534509799400_i64),(-2582762124215562148_i64),(-5636935387777419885_i64),140596394735148035_i64];
place!(Field::<i16>(Variant(_5, 0), 0)) = 214124456805799933502243380316985116314_u128 as i16;
RET = _3;
_6 = _2;
match _12 {
0 => bb4,
1 => bb5,
2 => bb6,
2770176696 => bb8,
_ => bb7
}
}
bb4 = {
_3 = _7;
_4 = _9;
RET.fld0 = [(-94_i8),23_i8,(-57_i8)];
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
_4 = _9;
RET = Adt50 { fld0: _4 };
place!(Field::<u8>(Variant(_5, 0), 1)) = 147_u8 << _12;
_13 = [(-6692463589898288136_i64),(-3425613949747105451_i64),8855279813483594660_i64,6953708365275755603_i64,5176589312614717320_i64,(-8530305597785140190_i64),1573263999148173238_i64];
_13 = [2043651552016590654_i64,1274957632533943182_i64,975401168198938805_i64,(-7839898395753616580_i64),2120408583560619603_i64,6379677590929712989_i64,3008152051244133172_i64];
_13 = [(-2024176185992228718_i64),(-6008328367331836541_i64),(-2670574574647610787_i64),9197538873750341241_i64,668517840772408737_i64,8946763907410271264_i64,(-7308889117574089898_i64)];
Goto(bb9)
}
bb9 = {
Call(_16 = dump_var(16_usize, 8_usize, Move(_8), 11_usize, Move(_11), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [bool; 5],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [i64; 7]) -> usize {
mir! {
type RET = usize;
let _7: bool;
let _8: [bool; 5];
let _9: f32;
let _10: isize;
let _11: char;
let _12: bool;
let _13: isize;
let _14: bool;
let _15: (*const u128,);
let _16: (&'static u8, usize, isize, isize);
let _17: u32;
let _18: u8;
let _19: Adt50;
let _20: (f64, i64, u8);
let _21: f64;
let _22: Adt45;
let _23: [i8; 5];
let _24: f64;
let _25: [isize; 2];
let _26: (f64, i32, f32);
let _27: u64;
let _28: Adt55;
let _29: u8;
let _30: [i8; 5];
let _31: char;
let _32: f64;
let _33: f64;
let _34: Adt56;
let _35: (f64, i32, f32);
let _36: f32;
let _37: Adt52;
let _38: (*mut i16, u16, i128, char, *mut i16);
let _39: [usize; 5];
let _40: i16;
let _41: [i8; 1];
let _42: [usize; 5];
let _43: char;
let _44: i8;
let _45: [i8; 5];
let _46: Adt56;
let _47: Adt44;
let _48: Adt50;
let _49: [i8; 3];
let _50: (char,);
let _51: ();
let _52: ();
{
RET = 11693627510254881149_usize;
RET = 5_usize ^ 594605814269494212_usize;
_4 = _5;
_1 = [_2,_3,_5,_3,_2];
RET = 16286939083195371386_usize;
_5 = _3;
_5 = _3;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
16286939083195371386 => bb5,
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
_9 = 61666_u16 as f32;
RET = 1_usize;
_8 = [_2,_2,_5,_1[RET],_5];
_9 = _6[RET] as f32;
_9 = RET as f32;
_8 = _1;
match _6[RET] {
0 => bb4,
340282366920938463459367406719320288135 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_6 = [(-5726484773896970827_i64),5650025845412757311_i64,4504562563296045532_i64,5637877415868425640_i64,(-5881017965595721586_i64),(-3931692779108721153_i64),8059854924670970147_i64];
_1[RET] = _3;
_10 = 1602001072_u32 as isize;
_7 = _1[RET];
RET = 34408211535354180_usize;
RET = !6_usize;
_1 = _8;
_8 = [_5,_5,_5,_3,_7];
_6 = [(-5397909341346982473_i64),4727878993386252315_i64,(-6365625708074489418_i64),(-7688335615628234527_i64),(-2378149674317457087_i64),(-5705626678068420930_i64),342838297648189219_i64];
_5 = !_7;
Goto(bb8)
}
bb8 = {
_7 = !_2;
_2 = !_7;
Goto(bb9)
}
bb9 = {
RET = 6075656100557794348_usize * 0_usize;
_4 = _5 < _3;
_14 = !_4;
_9 = 20663_u16 as f32;
_13 = (-96996361163131055599682435922835687689_i128) as isize;
_10 = 144_u8 as isize;
_1 = [_7,_3,_14,_7,_5];
RET = 17877261870001315357_usize & 2_usize;
_8 = _1;
_5 = _3 | _14;
_10 = RET as isize;
_6 = [(-2306539150233342477_i64),(-5553242081991670692_i64),(-6811114328680124466_i64),6893282525173440105_i64,8492332936505414437_i64,9027375535419035364_i64,1386970683470867225_i64];
_11 = '\u{fa4a9}';
_14 = _2 < _4;
_1 = [_14,_7,_5,_14,_2];
_12 = !_4;
RET = 17809037800673606220_usize + 10319963563737636427_usize;
_12 = !_3;
_10 = _13;
_4 = _2 <= _2;
_8 = _1;
_4 = _3;
Goto(bb10)
}
bb10 = {
_4 = _3 <= _14;
_16.2 = -_10;
_16.3 = -_10;
_10 = _16.2;
_5 = _2 | _3;
_9 = 245_u8 as f32;
RET = (-10400660616493410569361178292899912364_i128) as usize;
_16.1 = 102_u8 as usize;
_9 = _13 as f32;
_20.2 = 59_u8 & 94_u8;
_10 = -_13;
_21 = 27_i8 as f64;
_19.fld0 = [(-119_i8),(-125_i8),64_i8];
_14 = _4;
_21 = RET as f64;
_20.1 = 3064168901814339809_i64;
_19.fld0 = [(-113_i8),(-28_i8),(-21_i8)];
_20.1 = -2701655019258812334_i64;
_19.fld0 = [95_i8,81_i8,92_i8];
_2 = _4;
_19.fld0 = [(-90_i8),55_i8,(-57_i8)];
_4 = _7;
_8 = [_12,_12,_14,_3,_4];
Goto(bb11)
}
bb11 = {
_21 = (-2138937734_i32) as f64;
_18 = _20.2 | _20.2;
_20.1 = _9 as i64;
_1 = [_12,_7,_2,_2,_4];
_19.fld0 = [10_i8,(-10_i8),17_i8];
_19.fld0 = [80_i8,83_i8,122_i8];
RET = (-40_i8) as usize;
_10 = _16.3;
_17 = 139760376417072237519942831694892021353_u128 as u32;
_26.0 = _16.1 as f64;
_20.0 = _21;
_18 = !_20.2;
_12 = _14;
_25 = [_16.3,_13];
Goto(bb12)
}
bb12 = {
_16.1 = RET * RET;
_28.fld5 = !1650701963123755010_u64;
_27 = _12 as u64;
_20.2 = _18 >> _27;
_3 = _5;
_23 = [(-116_i8),108_i8,(-98_i8),8_i8,83_i8];
_12 = _14 & _7;
_28.fld4 = core::ptr::addr_of_mut!(_16.3);
_12 = !_14;
_17 = _20.2 as u32;
_17 = !2978293183_u32;
_33 = (-16715_i16) as f64;
_16.0 = &_20.2;
_24 = _33 - _26.0;
_14 = _12 != _5;
_28.fld0 = core::ptr::addr_of!(_8);
_28.fld5 = !_27;
_18 = _20.2;
_30 = [(-20_i8),(-90_i8),33_i8,74_i8,(-78_i8)];
_11 = '\u{71975}';
Goto(bb13)
}
bb13 = {
_23 = [93_i8,(-107_i8),(-111_i8),(-30_i8),22_i8];
_22 = Adt45::Variant1 { fld0: _20,fld1: _17,fld2: _28.fld5 };
_35.0 = _21 + _20.0;
_36 = _9;
_26.2 = -_9;
_32 = Field::<(f64, i64, u8)>(Variant(_22, 1), 0).0 + _21;
_33 = _36 as f64;
_20 = Field::<(f64, i64, u8)>(Variant(_22, 1), 0);
_20.2 = _18 >> _28.fld5;
_1 = [_12,_5,_2,_12,_4];
_38.3 = _11;
_35.2 = _26.2 - _36;
_28.fld2 = Field::<(f64, i64, u8)>(Variant(_22, 1), 0).1 as f32;
SetDiscriminant(_22, 1);
_25 = [_10,_16.3];
_17 = 3386_i16 as u32;
_43 = _11;
_26.2 = _35.2 + _36;
_8 = [_7,_7,_12,_12,_12];
_16.2 = -_13;
_35.0 = -_24;
_38.4 = core::ptr::addr_of_mut!(_40);
_38.1 = 54327_u16;
Goto(bb14)
}
bb14 = {
_38.1 = _38.3 as u16;
_6 = [_20.1,_20.1,_20.1,_20.1,_20.1,_20.1,_20.1];
_18 = _20.2;
_16.0 = &_29;
_35.1 = -(-2055713513_i32);
_38.0 = core::ptr::addr_of_mut!(_40);
_44 = _38.3 as i8;
_40 = -(-4725_i16);
_20 = (_35.0, 1495661333310661224_i64, _18);
_26.1 = _35.1 | _35.1;
_43 = _11;
_16.1 = _27 as usize;
_6 = [_20.1,_20.1,_20.1,_20.1,_20.1,_20.1,_20.1];
_28.fld2 = -_36;
_20.1 = 8783436823646205753_i64;
place!(Field::<u64>(Variant(_22, 1), 2)) = !_28.fld5;
_35.2 = -_26.2;
_42 = [_16.1,_16.1,_16.1,_16.1,_16.1];
_38.4 = core::ptr::addr_of_mut!(_40);
_29 = _20.2;
_13 = _10 >> Field::<u64>(Variant(_22, 1), 2);
_22 = Adt45::Variant1 { fld0: _20,fld1: _17,fld2: _28.fld5 };
place!(Field::<u64>(Variant(_22, 1), 2)) = _27 | _28.fld5;
_3 = !_7;
place!(Field::<(f64, i64, u8)>(Variant(_22, 1), 0)).2 = _29;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(17_usize, 30_usize, Move(_30), 4_usize, Move(_4), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(17_usize, 13_usize, Move(_13), 17_usize, Move(_17), 43_usize, Move(_43), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(17_usize, 14_usize, Move(_14), 23_usize, Move(_23), 2_usize, Move(_2), 42_usize, Move(_42)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-145218084643592004298876888076989086905_i128)), std::hint::black_box(3198018087_u32), std::hint::black_box(11_isize));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: f32,
fld1: isize,

},
Variant1{
fld0: *mut i16,
fld1: [bool; 5],
fld2: (f64, i64, u8),
fld3: i8,
fld4: i128,

},
Variant2{
fld0: u32,
fld1: usize,
fld2: f64,
fld3: u16,
fld4: [bool; 5],

},
Variant3{
fld0: [isize; 2],
fld1: (f64, i64, u8),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: i16,
fld1: u8,

},
Variant1{
fld0: *const (*mut i16, u16, i128, char, *mut i16),
fld1: isize,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u64,
fld1: u16,

},
Variant1{
fld0: [i64; 8],
fld1: u128,
fld2: i64,
fld3: f64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [i8; 5],
fld1: [i64; 8],
fld2: u16,

},
Variant1{
fld0: (f64, i64, u8),
fld1: u32,
fld2: u64,

},
Variant2{
fld0: *const usize,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: u16,
fld1: f64,
fld2: (f64, i32, f32),
fld3: *mut i16,
fld4: u64,
fld5: i32,
}
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut i16,
fld1: Adt45,
fld2: (*mut isize, [isize; 2]),
fld3: i8,
fld4: *const [bool; 5],
fld5: Adt44,
fld6: [usize; 5],
fld7: i128,

},
Variant1{
fld0: *const usize,
fld1: *const (*mut i16, u16, i128, char, *mut i16),

},
Variant2{
fld0: [i64; 7],
fld1: char,
fld2: Adt43,
fld3: i8,
fld4: i16,
fld5: Adt45,

},
Variant3{
fld0: *const [bool; 5],
fld1: Adt43,
fld2: u8,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt46,
fld1: *mut i16,
fld2: (*mut i16, u16, i128, char, *mut i16),

},
Variant1{
fld0: *mut i16,
fld1: char,
fld2: isize,
fld3: i64,

},
Variant2{
fld0: u32,
fld1: *const u128,

}}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt45,
fld1: u64,
fld2: Adt43,
fld3: u128,
fld4: (*const u128,),
fld5: *const usize,
fld6: u32,
fld7: *const u128,

},
Variant1{
fld0: (f64, i32, f32),
fld1: f32,
fld2: [usize; 5],
fld3: i8,
fld4: *const usize,
fld5: [i8; 5],
fld6: [isize; 2],
fld7: Adt43,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [i8; 3],
}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt43,
fld1: f64,
fld2: Adt47,
fld3: [i8; 1],
fld4: [i8; 3],
fld5: Adt46,
fld6: (*const u128,),

},
Variant1{
fld0: bool,
fld1: f32,
fld2: i8,

},
Variant2{
fld0: u128,
fld1: [i64; 8],
fld2: Adt45,
fld3: Adt49,
fld4: (f64, i32, f32),
fld5: Adt46,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const usize,
fld1: (*const u128,),
fld2: Adt49,
fld3: *const [bool; 5],
fld4: i16,
fld5: [i64; 8],

},
Variant1{
fld0: u32,
fld1: Adt42,
fld2: *mut isize,
fld3: [i8; 1],
fld4: i16,
fld5: (char,),
fld6: Adt48,
fld7: i128,

},
Variant2{
fld0: Adt48,
fld1: Adt44,
fld2: u128,
fld3: Adt46,
fld4: Adt50,
fld5: *const (*mut i16, u16, i128, char, *mut i16),
fld6: [isize; 2],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: Adt45,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: usize,
fld1: u16,
fld2: u64,
fld3: Adt47,
fld4: [i8; 3],
fld5: [isize; 2],
fld6: i64,
fld7: Adt43,

},
Variant1{
fld0: u64,
fld1: char,
fld2: *mut isize,
fld3: Adt45,
fld4: [i64; 7],
fld5: [i8; 5],
fld6: *const (*mut i16, u16, i128, char, *mut i16),

},
Variant2{
fld0: u128,
fld1: *const u128,
fld2: Adt49,
fld3: i8,
fld4: (*const u128,),
fld5: u8,
fld6: *const usize,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: *const [bool; 5],
fld1: (*const u128,),
fld2: f32,
fld3: Adt49,
fld4: *mut isize,
fld5: u64,
fld6: Adt47,
}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt48,
fld1: char,
fld2: isize,
fld3: u16,
fld4: *mut [isize; 2],
fld5: Adt43,
fld6: u8,
fld7: *const u128,

},
Variant1{
fld0: Adt49,
fld1: i128,
fld2: [i64; 7],
fld3: [i8; 1],

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [i64; 7],
fld1: *mut i16,
fld2: (*const u128,),
fld3: Adt51,
fld4: i128,
fld5: Adt43,
fld6: [i8; 5],

},
Variant1{
fld0: (*mut i16, u16, i128, char, *mut i16),
fld1: Adt50,
fld2: Adt56,
fld3: Adt53,
fld4: f32,
fld5: (char,),

},
Variant2{
fld0: [isize; 2],
fld1: Adt45,
fld2: u8,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [usize; 5],
fld1: Adt54,
fld2: isize,
fld3: [i8; 5],
fld4: (*const u128,),
fld5: i32,
fld6: Adt53,

},
Variant1{
fld0: bool,
fld1: Adt51,
fld2: *const [bool; 5],
fld3: Adt49,
fld4: [isize; 2],
fld5: [i64; 7],
fld6: Adt48,

},
Variant2{
fld0: (f64, i64, u8),
fld1: [isize; 2],

}}

