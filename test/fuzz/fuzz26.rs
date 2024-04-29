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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: u32) -> u16 {
mir! {
type RET = u16;
let _5: (i16, i64, u32);
let _6: [usize; 6];
let _7: u16;
let _8: *mut u16;
let _9: [isize; 2];
let _10: Adt60;
let _11: Adt56;
let _12: *const [i8; 4];
let _13: ((u16, u8, i32, f64),);
let _14: [usize; 6];
let _15: Adt56;
let _16: Adt53;
let _17: (i16, f32, usize);
let _18: isize;
let _19: (i16, i64, u32);
let _20: i16;
let _21: ();
let _22: ();
{
_5.2 = 5652589900212772969_usize as u32;
_4 = 317521221849284229878202760389882256480_u128 as u32;
RET = (-22469_i16) as u16;
_4 = _5.2 & _5.2;
_5 = ((-19906_i16), 5194350632037166674_i64, _4);
_1 = !true;
_3 = 9223372036854775807_isize << _4;
_2 = '\u{a66c}';
_8 = core::ptr::addr_of_mut!(RET);
_6 = [7_usize,9851536941987771615_usize,4_usize,7378768433952179694_usize,7_usize,3_usize];
RET = 59680_u16 + 13750_u16;
(*_8) = _3 as u16;
Call(_5.2 = fn1(_8, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5.1 = !1847131656898993188_i64;
match _5.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768191550 => bb8,
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
_7 = (*_8);
_1 = true ^ false;
(*_8) = _7;
RET = _7;
_2 = '\u{ee199}';
_4 = !_5.2;
_5.2 = _5.1 as u32;
_5 = ((-24127_i16), (-2510933510226291879_i64), _4);
match _5.0 {
0 => bb9,
1 => bb10,
2 => bb11,
340282366920938463463374607431768187329 => bb13,
_ => bb12
}
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
_5.1 = !1847131656898993188_i64;
match _5.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768191550 => bb8,
_ => bb7
}
}
bb13 = {
_3 = !(-9223372036854775808_isize);
_1 = RET <= (*_8);
_5 = ((-10966_i16), (-7088359776720592821_i64), _4);
(*_8) = !_7;
_5.0 = -(-814_i16);
_8 = core::ptr::addr_of_mut!((*_8));
_1 = false ^ true;
_8 = core::ptr::addr_of_mut!(_7);
RET = _7 + (*_8);
_5.2 = _4;
_5.1 = (-8022260821287427902_i64);
(*_8) = !RET;
_5 = (18271_i16, (-3019636672891059142_i64), _4);
_9 = [_3,_3];
_4 = (-111161047_i32) as u32;
_5.0 = 29813_i16 | 5922_i16;
RET = !(*_8);
match _5.1 {
0 => bb8,
1 => bb2,
2 => bb12,
3 => bb4,
4 => bb11,
5 => bb9,
6 => bb14,
340282366920938463460354970758877152314 => bb16,
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
_6 = [4695582374109242600_usize,12912709375212354511_usize,4_usize,16995794502104798998_usize,1440213645482751014_usize,17714041894985320019_usize];
_5.1 = 94_i8 as i64;
(*_8) = !RET;
_5.1 = (-644269591615767715_i64);
RET = (*_8) | _7;
_6 = [6890032026287649106_usize,4_usize,4_usize,13045314930077532431_usize,3_usize,16972446206224437043_usize];
_1 = true;
_3 = (-9223372036854775808_isize) * (-35_isize);
_13.0.1 = 22_u8;
_13.0.1 = 151_u8;
_13.0.2 = _13.0.1 as i32;
_14 = _6;
_13.0.0 = _5.1 as u16;
_17.1 = (-122_i8) as f32;
_7 = (-72_i8) as u16;
(*_8) = !RET;
_5.0 = _5.1 as i16;
_14 = _6;
_6 = [4433511060437092564_usize,2_usize,18100972186851264431_usize,15487832748398918427_usize,2_usize,11648567946496011902_usize];
_5.2 = !_4;
_6 = _14;
_19.2 = _5.2 ^ _5.2;
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(0_usize, 4_usize, Move(_4), 3_usize, Move(_3), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: *mut u16,mut _2: [usize; 6]) -> u32 {
mir! {
type RET = u32;
let _3: f32;
let _4: isize;
let _5: bool;
let _6: i8;
let _7: *const bool;
let _8: (u8, u32);
let _9: [u8; 6];
let _10: (u16, u8, i32, f64);
let _11: i64;
let _12: Adt56;
let _13: *mut f32;
let _14: *const bool;
let _15: ((u16, u8, i32, f64),);
let _16: [u16; 8];
let _17: i64;
let _18: bool;
let _19: isize;
let _20: ();
let _21: ();
{
(*_1) = 1889_u16;
(*_1) = !16477_u16;
_1 = core::ptr::addr_of_mut!((*_1));
RET = 511664638_u32 << (*_1);
(*_1) = 9223372036854775807_isize as u16;
_2 = [7014749271935535862_usize,4_usize,0_usize,7_usize,18238111639624905768_usize,6_usize];
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{29348}' as u16;
RET = !16222038_u32;
RET = 2992767743_u32 * 1303141368_u32;
RET = !4004647068_u32;
RET = 3715564242_u32 << (*_1);
(*_1) = 0_usize as u16;
_1 = core::ptr::addr_of_mut!((*_1));
RET = false as u32;
Call(_2 = fn2(_1, _1, (*_1), _1, RET, (*_1), _1, _1, (*_1), (*_1), _1, (*_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 3053625553_u32;
_3 = (*_1) as f32;
_4 = 17960706735315579763_u64 as isize;
_5 = (*_1) > (*_1);
_2 = [341975234826384504_usize,6803026934255470124_usize,0_usize,16951066291293861003_usize,2_usize,2331100296555268636_usize];
_3 = (*_1) as f32;
_3 = _4 as f32;
match RET {
0 => bb2,
1 => bb3,
3053625553 => bb5,
_ => bb4
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
_8.1 = RET;
_7 = core::ptr::addr_of!(_5);
_3 = 100_u8 as f32;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = [0_usize,7501746285521768860_usize,0_usize,1637314808383195648_usize,3_usize,10858188093575494609_usize];
(*_7) = true;
_4 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_7 = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 60577_u16 - 39600_u16;
_7 = core::ptr::addr_of!(_5);
_8 = (88_u8, RET);
_6 = -(-18_i8);
_8.1 = RET * RET;
(*_7) = false;
_10.3 = (-7228441913067026444_i64) as f64;
_10.1 = !_8.0;
_9 = [_8.0,_10.1,_8.0,_8.0,_8.0,_8.0];
_8.1 = RET;
_1 = core::ptr::addr_of_mut!(_10.0);
_6 = 96_i8 & (-58_i8);
_10.2 = (-146238079527366280069611159198889226758_i128) as i32;
Goto(bb6)
}
bb6 = {
_11 = (-8648840278101723278_i64);
_3 = 8667564116249793583_u64 as f32;
_10.1 = _8.0 + _8.0;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = !(-9223372036854775808_isize);
(*_1) = 32632_u16;
_8.1 = !RET;
(*_1) = 56006_u16;
_7 = core::ptr::addr_of!((*_7));
_8.0 = _10.1;
RET = _8.1 << _10.1;
_8.1 = RET;
_14 = _7;
(*_14) = _11 >= _11;
_1 = core::ptr::addr_of_mut!(_10.0);
_10.1 = _8.0 ^ _8.0;
Goto(bb7)
}
bb7 = {
_13 = core::ptr::addr_of_mut!(_3);
_8.0 = _10.1 * _10.1;
_2 = [12923980475266563870_usize,17839125971262605192_usize,3602855961433933514_usize,6_usize,10263770004669849675_usize,623666126488633614_usize];
_15.0.1 = _10.3 as u8;
_15 = (_10,);
_15.0.3 = -_10.3;
_10.0 = !_15.0.0;
(*_13) = _8.0 as f32;
_8.0 = _10.1;
_3 = _6 as f32;
_16 = [_15.0.0,(*_1),_15.0.0,(*_1),(*_1),(*_1),_10.0,_10.0];
(*_1) = _15.0.0 % _15.0.0;
(*_7) = false;
match _15.0.0 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
56006 => bb14,
_ => bb13
}
}
bb8 = {
_11 = (-8648840278101723278_i64);
_3 = 8667564116249793583_u64 as f32;
_10.1 = _8.0 + _8.0;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = !(-9223372036854775808_isize);
(*_1) = 32632_u16;
_8.1 = !RET;
(*_1) = 56006_u16;
_7 = core::ptr::addr_of!((*_7));
_8.0 = _10.1;
RET = _8.1 << _10.1;
_8.1 = RET;
_14 = _7;
(*_14) = _11 >= _11;
_1 = core::ptr::addr_of_mut!(_10.0);
_10.1 = _8.0 ^ _8.0;
Goto(bb7)
}
bb9 = {
_8.1 = RET;
_7 = core::ptr::addr_of!(_5);
_3 = 100_u8 as f32;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = [0_usize,7501746285521768860_usize,0_usize,1637314808383195648_usize,3_usize,10858188093575494609_usize];
(*_7) = true;
_4 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_7 = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 60577_u16 - 39600_u16;
_7 = core::ptr::addr_of!(_5);
_8 = (88_u8, RET);
_6 = -(-18_i8);
_8.1 = RET * RET;
(*_7) = false;
_10.3 = (-7228441913067026444_i64) as f64;
_10.1 = !_8.0;
_9 = [_8.0,_10.1,_8.0,_8.0,_8.0,_8.0];
_8.1 = RET;
_1 = core::ptr::addr_of_mut!(_10.0);
_6 = 96_i8 & (-58_i8);
_10.2 = (-146238079527366280069611159198889226758_i128) as i32;
Goto(bb6)
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
RET = 3053625553_u32;
_3 = (*_1) as f32;
_4 = 17960706735315579763_u64 as isize;
_5 = (*_1) > (*_1);
_2 = [341975234826384504_usize,6803026934255470124_usize,0_usize,16951066291293861003_usize,2_usize,2331100296555268636_usize];
_3 = (*_1) as f32;
_3 = _4 as f32;
match RET {
0 => bb2,
1 => bb3,
3053625553 => bb5,
_ => bb4
}
}
bb14 = {
RET = '\u{922c7}' as u32;
_10.3 = _15.0.3 * _15.0.3;
_15.0.3 = _11 as f64;
_2 = [4649779825763948625_usize,18073896684367555023_usize,6823200336424344697_usize,6_usize,14947753611565458389_usize,2_usize];
_9 = [_15.0.1,_15.0.1,_15.0.1,_10.1,_10.1,_10.1];
(*_13) = (-146040961547178979881642665818649872157_i128) as f32;
_2 = [5_usize,6_usize,17325142328563692966_usize,2_usize,5885520510136203571_usize,8529381170877476868_usize];
_10.2 = _15.0.2 + _15.0.2;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(1_usize, 2_usize, Move(_2), 16_usize, Move(_16), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: *mut u16,mut _2: *mut u16,mut _3: u16,mut _4: *mut u16,mut _5: u32,mut _6: u16,mut _7: *mut u16,mut _8: *mut u16,mut _9: u16,mut _10: u16,mut _11: *mut u16,mut _12: u16) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _13: u128;
let _14: [u8; 7];
let _15: [u16; 8];
let _16: ([i8; 3],);
let _17: [i8; 3];
let _18: Adt46;
let _19: ((u16, u8, i32, f64),);
let _20: u8;
let _21: isize;
let _22: Adt47;
let _23: [isize; 2];
let _24: i64;
let _25: ([i8; 3],);
let _26: *const bool;
let _27: *mut f32;
let _28: f32;
let _29: ((i16, i64, u32), i16);
let _30: f32;
let _31: f64;
let _32: (*mut u8,);
let _33: Adt59;
let _34: [i128; 4];
let _35: char;
let _36: ((u16, u8, i32, f64),);
let _37: isize;
let _38: Adt50;
let _39: [isize; 2];
let _40: ();
let _41: ();
{
_1 = core::ptr::addr_of_mut!((*_8));
RET = [17024512463578155031_usize,14668743885071747547_usize,2_usize,2_usize,17164733871531584800_usize,11155804469631064884_usize];
(*_8) = !_10;
(*_2) = 11333826928083537024_usize as u16;
_13 = !299697886902804056765288319149969613882_u128;
_6 = !(*_7);
_3 = (*_8) & (*_2);
(*_4) = _3 - _6;
RET = [4126094579092920381_usize,3_usize,8182097215119536768_usize,7_usize,2199951515079176015_usize,5_usize];
(*_1) = _9 + _6;
_14 = [108_u8,7_u8,50_u8,156_u8,47_u8,97_u8,174_u8];
_15 = [(*_1),(*_1),(*_1),_12,(*_1),(*_2),(*_4),(*_1)];
_11 = _7;
(*_7) = !_6;
(*_8) = !_3;
Goto(bb1)
}
bb1 = {
(*_1) = _13 as u16;
_7 = _4;
_16.0 = [69_i8,118_i8,(-69_i8)];
(*_1) = (-82_i8) as u16;
(*_11) = _3 | _12;
_15 = [(*_11),(*_8),(*_8),(*_1),(*_11),(*_1),(*_1),(*_11)];
(*_4) = true as u16;
_11 = core::ptr::addr_of_mut!((*_8));
_2 = core::ptr::addr_of_mut!((*_4));
_12 = !(*_4);
_3 = true as u16;
_11 = core::ptr::addr_of_mut!((*_1));
(*_8) = !_3;
(*_2) = !_3;
_18.fld0 = (63_u8, _5);
_2 = core::ptr::addr_of_mut!(_12);
(*_7) = (*_2);
_9 = (*_1);
_17 = [(-52_i8),(-50_i8),(-8_i8)];
(*_2) = !(*_11);
_12 = true as u16;
_16 = (_17,);
(*_2) = !(*_1);
_19.0.1 = _18.fld0.0;
_7 = _11;
_10 = (*_4) >> _5;
_18.fld0.1 = _5;
Goto(bb2)
}
bb2 = {
(*_7) = !_6;
(*_8) = 16874292213241077772_usize as u16;
_18.fld0.0 = !_19.0.1;
(*_1) = !_3;
_11 = core::ptr::addr_of_mut!(_19.0.0);
_16 = (_17,);
_9 = (*_8) << (*_4);
(*_1) = _10 ^ _9;
_19.0.3 = (-2855414356136461300_i64) as f64;
_10 = 9223372036854775807_isize as u16;
_10 = (*_8);
(*_4) = _6;
_16.0 = _17;
_18.fld0.0 = !_19.0.1;
_19.0.2 = -1772771587_i32;
_13 = !100292912645908212686166591766050242896_u128;
_19.0.3 = 691370092840141111_i64 as f64;
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_23 = [(-9223372036854775808_isize),9223372036854775807_isize];
_18.fld0 = (_19.0.1, _5);
_23 = [9223372036854775807_isize,9223372036854775807_isize];
_15 = [(*_2),(*_7),(*_4),_9,_10,_3,_9,_12];
_5 = !_18.fld0.1;
_20 = '\u{7a1f9}' as u8;
_13 = !199658860396384344729570779902671235548_u128;
_2 = _1;
_16.0 = [(-94_i8),(-34_i8),(-103_i8)];
(*_4) = _10;
_12 = (*_1) & _10;
Call(_11 = fn3(_16, _12, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = !_12;
_18.fld0 = (_19.0.1, _5);
RET = [5831938769248635989_usize,15666265853517602785_usize,15779089303746518720_usize,7_usize,4_usize,9998798469589885279_usize];
_24 = _18.fld0.1 as i64;
_12 = (*_1) - (*_1);
match _19.0.1 {
63 => bb7,
_ => bb6
}
}
bb6 = {
(*_7) = !_6;
(*_8) = 16874292213241077772_usize as u16;
_18.fld0.0 = !_19.0.1;
(*_1) = !_3;
_11 = core::ptr::addr_of_mut!(_19.0.0);
_16 = (_17,);
_9 = (*_8) << (*_4);
(*_1) = _10 ^ _9;
_19.0.3 = (-2855414356136461300_i64) as f64;
_10 = 9223372036854775807_isize as u16;
_10 = (*_8);
(*_4) = _6;
_16.0 = _17;
_18.fld0.0 = !_19.0.1;
_19.0.2 = -1772771587_i32;
_13 = !100292912645908212686166591766050242896_u128;
_19.0.3 = 691370092840141111_i64 as f64;
Goto(bb3)
}
bb7 = {
_6 = (*_7) + _12;
_15 = [_6,(*_4),(*_1),_10,(*_7),(*_7),_6,(*_7)];
_18.fld0.0 = 9223372036854775807_isize as u8;
_3 = 6339757607145057_usize as u16;
_21 = !(-16_isize);
_24 = _19.0.2 as i64;
_18.fld0 = (_19.0.1, _5);
(*_1) = _12;
_17 = [(-126_i8),97_i8,126_i8];
_18.fld0.1 = _5;
_18.fld0.0 = _19.0.1 << _18.fld0.1;
match _19.0.1 {
0 => bb6,
1 => bb5,
2 => bb3,
63 => bb8,
_ => bb4
}
}
bb8 = {
_19.0.0 = 15981361708033234840190536157989573145_i128 as u16;
_14 = [_19.0.1,_19.0.1,_18.fld0.0,_20,_18.fld0.0,_18.fld0.0,_18.fld0.0];
_3 = !_10;
_24 = 7001846517956780375_i64 * (-3064071479731800136_i64);
_12 = (*_2) - (*_2);
_15 = [(*_8),_12,_6,(*_4),(*_4),_6,(*_2),(*_1)];
_28 = _24 as f32;
_10 = !_12;
_19.0.0 = (*_1) | (*_1);
_22 = Adt47::Variant0 { fld0: _5,fld1: (-17229_i16) };
_15 = [_19.0.0,_12,(*_4),_6,_19.0.0,(*_1),(*_7),_12];
match _19.0.1 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb7,
63 => bb10,
_ => bb9
}
}
bb9 = {
(*_7) = !_6;
(*_8) = 16874292213241077772_usize as u16;
_18.fld0.0 = !_19.0.1;
(*_1) = !_3;
_11 = core::ptr::addr_of_mut!(_19.0.0);
_16 = (_17,);
_9 = (*_8) << (*_4);
(*_1) = _10 ^ _9;
_19.0.3 = (-2855414356136461300_i64) as f64;
_10 = 9223372036854775807_isize as u16;
_10 = (*_8);
(*_4) = _6;
_16.0 = _17;
_18.fld0.0 = !_19.0.1;
_19.0.2 = -1772771587_i32;
_13 = !100292912645908212686166591766050242896_u128;
_19.0.3 = 691370092840141111_i64 as f64;
Goto(bb3)
}
bb10 = {
_17 = _16.0;
_7 = _11;
_18.fld0.1 = _5;
_13 = 169340819667344271807301828862889177160_u128;
_12 = !_9;
_29.0 = (16782_i16, _24, Field::<u32>(Variant(_22, 0), 0));
_30 = _28 * _28;
_22 = Adt47::Variant1 { fld0: _29.0 };
(*_1) = Field::<(i16, i64, u32)>(Variant(_22, 1), 0).2 as u16;
_11 = _7;
place!(Field::<(i16, i64, u32)>(Variant(_22, 1), 0)).0 = _29.0.0;
place!(Field::<(i16, i64, u32)>(Variant(_22, 1), 0)).2 = '\u{c44d5}' as u32;
_18.fld0.0 = (-7_i8) as u8;
_16 = (_17,);
_4 = _11;
_6 = _9;
_29 = (Field::<(i16, i64, u32)>(Variant(_22, 1), 0), Field::<(i16, i64, u32)>(Variant(_22, 1), 0).0);
_28 = _6 as f32;
place!(Field::<(i16, i64, u32)>(Variant(_22, 1), 0)).2 = !_18.fld0.1;
_18.fld0.1 = !_29.0.2;
RET = [3652028327546102520_usize,5_usize,12968058298073551814_usize,4_usize,1269434256885737389_usize,5_usize];
_29.0.0 = false as i16;
_19.0.1 = _18.fld0.0 | _20;
_25.0 = _17;
Call(place!(Field::<(i16, i64, u32)>(Variant(_22, 1), 0)).0 = core::intrinsics::transmute(_6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8 = _7;
_16.0 = [(-84_i8),70_i8,(-12_i8)];
_4 = core::ptr::addr_of_mut!(_10);
RET = [5_usize,16743410722134963490_usize,1_usize,4_usize,6_usize,7211285356775157472_usize];
_18.fld0.1 = !Field::<(i16, i64, u32)>(Variant(_22, 1), 0).2;
RET = [7_usize,9892236499097927110_usize,5207628153997277729_usize,12444237747168701473_usize,2_usize,7_usize];
_7 = core::ptr::addr_of_mut!((*_4));
_18.fld0.1 = _29.0.2 * _29.0.2;
_10 = (*_1);
_19.0.3 = _19.0.2 as f64;
_32.0 = core::ptr::addr_of_mut!(_19.0.1);
_36.0.1 = _18.fld0.0;
_27 = core::ptr::addr_of_mut!(_28);
_36.0 = (_12, _19.0.1, _19.0.2, _19.0.3);
_16.0 = [4_i8,(-88_i8),(-127_i8)];
_13 = !69118495881242129679478092766421513673_u128;
_17 = [84_i8,33_i8,15_i8];
_29 = (Field::<(i16, i64, u32)>(Variant(_22, 1), 0), Field::<(i16, i64, u32)>(Variant(_22, 1), 0).0);
Goto(bb12)
}
bb12 = {
_9 = (*_7);
_17 = _16.0;
_36 = (_19.0,);
_29.0.2 = (*_27) as u32;
Goto(bb13)
}
bb13 = {
_19.0 = (_36.0.0, _36.0.1, _36.0.2, _36.0.3);
_1 = _11;
RET = [1_usize,510599973799102435_usize,7_usize,3_usize,7_usize,0_usize];
(*_7) = _19.0.0;
_8 = core::ptr::addr_of_mut!(_6);
_16.0 = _25.0;
_34 = [163416704006504371238773071079597444672_i128,(-159270054527025252215482846430116339342_i128),(-41525413360777221085549765753871118221_i128),159298899664055767348979560135640943203_i128];
_23 = [_21,_21];
_36.0.3 = _19.0.1 as f64;
_29.0.2 = _5 << (*_7);
Goto(bb14)
}
bb14 = {
_37 = _21 - _21;
_35 = '\u{b734c}';
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(2_usize, 6_usize, Move(_6), 20_usize, Move(_20), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(2_usize, 29_usize, Move(_29), 14_usize, Move(_14), 37_usize, Move(_37), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(2_usize, 15_usize, Move(_15), 23_usize, Move(_23), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: ([i8; 3],),mut _2: u16,mut _3: u16) -> *mut u16 {
mir! {
type RET = *mut u16;
let _4: isize;
let _5: i128;
let _6: i8;
let _7: char;
let _8: isize;
let _9: f64;
let _10: isize;
let _11: ();
let _12: ();
{
RET = core::ptr::addr_of_mut!(_2);
_1.0 = [118_i8,19_i8,(-42_i8)];
Call(_1.0 = fn4((*RET), _3, _2, RET, (*RET), _3, _3, _2, (*RET), RET, (*RET), _3, (*RET), _2, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _2 * (*RET);
(*RET) = !_3;
(*RET) = !_3;
(*RET) = _3;
_3 = (*RET);
_3 = !(*RET);
(*RET) = _3 | _3;
_5 = true as i128;
_4 = (-16924_i16) as isize;
_4 = (-9223372036854775808_isize);
RET = core::ptr::addr_of_mut!((*RET));
_1.0 = [23_i8,52_i8,(-30_i8)];
_4 = !9223372036854775807_isize;
_5 = (-16547497381322949024478235301447562957_i128);
(*RET) = _3;
_6 = 120_i8 + 106_i8;
_9 = 8409790401698663843_u64 as f64;
_7 = '\u{fde78}';
_4 = false as isize;
_6 = (-110_i8);
_6 = (-60_i8);
(*RET) = !_3;
_6 = 2785051414_u32 as i8;
_8 = _4 | _4;
Goto(bb2)
}
bb2 = {
Call(_11 = dump_var(3_usize, 8_usize, Move(_8), 6_usize, Move(_6), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: *mut u16,mut _5: u16,mut _6: u16,mut _7: u16,mut _8: u16,mut _9: u16,mut _10: *mut u16,mut _11: u16,mut _12: u16,mut _13: u16,mut _14: u16,mut _15: *mut u16,mut _16: u16) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _17: u128;
let _18: [u8; 6];
let _19: char;
let _20: Adt48;
let _21: i64;
let _22: [u8; 6];
let _23: isize;
let _24: [usize; 6];
let _25: [i8; 4];
let _26: isize;
let _27: bool;
let _28: char;
let _29: bool;
let _30: i16;
let _31: isize;
let _32: (i16, i64, u32);
let _33: bool;
let _34: i16;
let _35: isize;
let _36: [i8; 4];
let _37: ();
let _38: ();
{
RET = [69_i8,(-65_i8),9_i8];
(*_4) = _3;
(*_10) = 31392614035864342409221358685952768291_u128 as u16;
(*_15) = !_11;
(*_15) = _12;
_5 = _12;
_2 = 244255896030383532_i64 as u16;
_10 = core::ptr::addr_of_mut!(_14);
_7 = _13 | _16;
_13 = _3;
(*_10) = _6;
_13 = (*_15) + _8;
_6 = (*_15) | _7;
_5 = (*_15);
_8 = 83_u8 as u16;
_4 = _15;
_7 = !_5;
_7 = 12368846267178120064_u64 as u16;
_11 = (-8513913968045498500_i64) as u16;
_4 = _10;
_5 = _13;
(*_4) = 224906383244031808913671954843741751573_u128 as u16;
_10 = _4;
_17 = !64081761611120617039206852468593816513_u128;
Goto(bb1)
}
bb1 = {
_14 = !_6;
_13 = 15209084853910247727_u64 as u16;
_8 = 9223372036854775807_isize as u16;
Call((*_15) = fn5(_4, (*_10), (*_10), _10, _4, _6, _5, _10, (*_4), (*_10), _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = (*_15);
(*_10) = _13;
_19 = '\u{19b8b}';
_16 = _19 as u16;
(*_15) = !_14;
_4 = _15;
_4 = core::ptr::addr_of_mut!(_5);
(*_4) = (*_10);
_1 = _13 + (*_4);
RET = [(-84_i8),(-35_i8),27_i8];
_8 = !_14;
RET = [(-20_i8),64_i8,57_i8];
(*_15) = _1 ^ (*_10);
RET = [(-8_i8),(-29_i8),45_i8];
Call((*_4) = core::intrinsics::bswap((*_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_10) = !(*_4);
Call(_12 = core::intrinsics::transmute((*_15)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_4) = _3 >> _12;
_1 = _13;
(*_4) = !_14;
_14 = (*_4) + _8;
_19 = '\u{e0a86}';
(*_10) = _13 >> _1;
_8 = (*_15);
_8 = (*_15) + (*_15);
_3 = 730012131_u32 as u16;
_3 = (*_4) ^ (*_4);
_4 = _10;
_15 = core::ptr::addr_of_mut!(_9);
RET = [4_i8,94_i8,80_i8];
(*_4) = _6 | _8;
_22 = [255_u8,81_u8,121_u8,92_u8,173_u8,181_u8];
(*_15) = (*_10) * _1;
_7 = _13 ^ (*_15);
Call((*_10) = fn6(_10, _4, _7, _7, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = !(*_10);
Call(_1 = fn7(_7, _15, _9, _13, _5, (*_15), (*_15)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = _12;
_12 = (-16006_i16) as u16;
_6 = _8 + _3;
RET = [(-109_i8),45_i8,(-87_i8)];
_18 = [112_u8,206_u8,255_u8,102_u8,136_u8,148_u8];
_22 = [228_u8,70_u8,95_u8,45_u8,218_u8,0_u8];
Goto(bb7)
}
bb7 = {
(*_10) = !_5;
_18 = [123_u8,227_u8,13_u8,117_u8,156_u8,75_u8];
(*_15) = _8;
RET = [(-49_i8),(-53_i8),18_i8];
_1 = _5 + _9;
_9 = (*_4) + _13;
RET = [47_i8,78_i8,68_i8];
_17 = 337114535526077765030940838134958085058_u128 + 250307025444242347948411951937454775051_u128;
RET = [(-13_i8),(-26_i8),2_i8];
_13 = (*_15);
_12 = 46_i8 as u16;
Call(RET = fn9(_5, (*_15), (*_4), _14, _5, (*_15), _10, _6, _16, (*_15), _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18 = [168_u8,63_u8,58_u8,216_u8,54_u8,163_u8];
_28 = _19;
_17 = 94815940392434806951609503556697893937_u128;
_17 = 10909324475962444279878707230706841848_u128;
(*_10) = _1 & _3;
_22 = [155_u8,50_u8,50_u8,114_u8,65_u8,252_u8];
RET = [88_i8,118_i8,(-65_i8)];
(*_10) = !_3;
_26 = (-3566586228630546907_i64) as isize;
_15 = _4;
match _17 {
0 => bb7,
1 => bb4,
2 => bb5,
3 => bb9,
4 => bb10,
5 => bb11,
10909324475962444279878707230706841848 => bb13,
_ => bb12
}
}
bb9 = {
(*_10) = !_5;
_18 = [123_u8,227_u8,13_u8,117_u8,156_u8,75_u8];
(*_15) = _8;
RET = [(-49_i8),(-53_i8),18_i8];
_1 = _5 + _9;
_9 = (*_4) + _13;
RET = [47_i8,78_i8,68_i8];
_17 = 337114535526077765030940838134958085058_u128 + 250307025444242347948411951937454775051_u128;
RET = [(-13_i8),(-26_i8),2_i8];
_13 = (*_15);
_12 = 46_i8 as u16;
Call(RET = fn9(_5, (*_15), (*_4), _14, _5, (*_15), _10, _6, _16, (*_15), _14), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_16 = _12;
_12 = (-16006_i16) as u16;
_6 = _8 + _3;
RET = [(-109_i8),45_i8,(-87_i8)];
_18 = [112_u8,206_u8,255_u8,102_u8,136_u8,148_u8];
_22 = [228_u8,70_u8,95_u8,45_u8,218_u8,0_u8];
Goto(bb7)
}
bb11 = {
_14 = !_6;
_13 = 15209084853910247727_u64 as u16;
_8 = 9223372036854775807_isize as u16;
Call((*_15) = fn5(_4, (*_10), (*_10), _10, _4, _6, _5, _10, (*_4), (*_10), _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
(*_4) = _3 >> _12;
_1 = _13;
(*_4) = !_14;
_14 = (*_4) + _8;
_19 = '\u{e0a86}';
(*_10) = _13 >> _1;
_8 = (*_15);
_8 = (*_15) + (*_15);
_3 = 730012131_u32 as u16;
_3 = (*_4) ^ (*_4);
_4 = _10;
_15 = core::ptr::addr_of_mut!(_9);
RET = [4_i8,94_i8,80_i8];
(*_4) = _6 | _8;
_22 = [255_u8,81_u8,121_u8,92_u8,173_u8,181_u8];
(*_15) = (*_10) * _1;
_7 = _13 ^ (*_15);
Call((*_10) = fn6(_10, _4, _7, _7, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_8 = !_7;
(*_4) = 0_u8 as u16;
_25 = [21_i8,111_i8,110_i8,53_i8];
_18 = [241_u8,215_u8,112_u8,159_u8,5_u8,7_u8];
RET = [54_i8,(-124_i8),(-36_i8)];
_4 = _10;
_27 = !false;
_11 = 126_i8 as u16;
_18 = [211_u8,11_u8,52_u8,148_u8,165_u8,159_u8];
_21 = (-7322609407554453135_i64) | (-6912958296959867006_i64);
_22 = _18;
_29 = !_27;
_7 = _21 as u16;
_21 = -(-4141109555984137157_i64);
RET = [(-14_i8),124_i8,59_i8];
match _17 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb6,
5 => bb14,
10909324475962444279878707230706841848 => bb16,
_ => bb15
}
}
bb14 = {
(*_10) = !_5;
_18 = [123_u8,227_u8,13_u8,117_u8,156_u8,75_u8];
(*_15) = _8;
RET = [(-49_i8),(-53_i8),18_i8];
_1 = _5 + _9;
_9 = (*_4) + _13;
RET = [47_i8,78_i8,68_i8];
_17 = 337114535526077765030940838134958085058_u128 + 250307025444242347948411951937454775051_u128;
RET = [(-13_i8),(-26_i8),2_i8];
_13 = (*_15);
_12 = 46_i8 as u16;
Call(RET = fn9(_5, (*_15), (*_4), _14, _5, (*_15), _10, _6, _16, (*_15), _14), ReturnTo(bb8), UnwindUnreachable())
}
bb15 = {
(*_10) = !_5;
_18 = [123_u8,227_u8,13_u8,117_u8,156_u8,75_u8];
(*_15) = _8;
RET = [(-49_i8),(-53_i8),18_i8];
_1 = _5 + _9;
_9 = (*_4) + _13;
RET = [47_i8,78_i8,68_i8];
_17 = 337114535526077765030940838134958085058_u128 + 250307025444242347948411951937454775051_u128;
RET = [(-13_i8),(-26_i8),2_i8];
_13 = (*_15);
_12 = 46_i8 as u16;
Call(RET = fn9(_5, (*_15), (*_4), _14, _5, (*_15), _10, _6, _16, (*_15), _14), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_32.1 = _28 as i64;
_32 = (3518_i16, _21, 3287185048_u32);
(*_15) = _8 >> _6;
_30 = _32.0;
_27 = _29;
_28 = _19;
_14 = 15774295085930971729_u64 as u16;
_32.0 = _30;
_33 = _29;
_31 = _32.2 as isize;
RET = [(-12_i8),(-85_i8),43_i8];
_26 = _31;
_32 = (_30, _21, 2085462584_u32);
_32.2 = 1853018078_u32 | 4160055898_u32;
_34 = _32.0 * _30;
_13 = _8;
(*_10) = !_13;
_18 = [37_u8,9_u8,242_u8,182_u8,137_u8,139_u8];
_32.1 = _21 ^ _21;
_31 = _26 ^ _26;
_16 = _21 as u16;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(4_usize, 17_usize, Move(_17), 6_usize, Move(_6), 19_usize, Move(_19), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(4_usize, 28_usize, Move(_28), 21_usize, Move(_21), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(4_usize, 34_usize, Move(_34), 29_usize, Move(_29), 14_usize, Move(_14), 27_usize, Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_37 = dump_var(4_usize, 32_usize, Move(_32), 1_usize, Move(_1), 38_usize, _38, 38_usize, _38), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut u16,mut _2: u16,mut _3: u16,mut _4: *mut u16,mut _5: *mut u16,mut _6: u16,mut _7: u16,mut _8: *mut u16,mut _9: u16,mut _10: u16,mut _11: *mut u16) -> u16 {
mir! {
type RET = u16;
let _12: isize;
let _13: i32;
let _14: Adt53;
let _15: f32;
let _16: ([i8; 3],);
let _17: ([i8; 3],);
let _18: ();
let _19: ();
{
(*_11) = _7;
(*_5) = _9;
(*_1) = _10 ^ _2;
(*_11) = _7 >> _10;
_10 = !_7;
(*_8) = !_3;
RET = (*_8) - (*_5);
_7 = RET | (*_4);
_9 = _3;
_11 = _8;
(*_8) = RET << _3;
(*_8) = _10;
(*_11) = RET;
_6 = (*_1);
(*_4) = (-83990527_i32) as u16;
(*_1) = _7;
_5 = core::ptr::addr_of_mut!(RET);
_2 = 7_usize as u16;
(*_5) = (*_1);
_12 = -(-9223372036854775808_isize);
(*_8) = !(*_5);
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(5_usize, 9_usize, Move(_9), 10_usize, Move(_10), 12_usize, Move(_12), 19_usize, _19), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut u16,mut _2: *mut u16,mut _3: u16,mut _4: u16,mut _5: *mut u16) -> u16 {
mir! {
type RET = u16;
let _6: Adt46;
let _7: Adt50;
let _8: ((i16, i64, u32), i16);
let _9: Adt54;
let _10: f32;
let _11: isize;
let _12: [u128; 2];
let _13: char;
let _14: [usize; 2];
let _15: (i16, i64, u32);
let _16: isize;
let _17: Adt62;
let _18: *mut u32;
let _19: u128;
let _20: u8;
let _21: bool;
let _22: *const bool;
let _23: usize;
let _24: [i128; 4];
let _25: isize;
let _26: [usize; 2];
let _27: *const [u8; 7];
let _28: Adt52;
let _29: ();
let _30: ();
{
RET = !_3;
_6.fld0.1 = _3 as u32;
RET = _3;
_6.fld0.0 = 206_u8 - 179_u8;
_5 = core::ptr::addr_of_mut!(_3);
_8.1 = 22427_i16 * (-21722_i16);
_8.0.0 = _8.1 >> (*_5);
RET = 8161984346437051810_usize as u16;
Goto(bb1)
}
bb1 = {
_8.0.0 = _6.fld0.1 as i16;
Goto(bb2)
}
bb2 = {
RET = false as u16;
_4 = 104301800042512873626830301890033726611_u128 as u16;
_8.0.0 = _8.1 + _8.1;
_8.0 = (_8.1, (-3331530096914783835_i64), _6.fld0.1);
(*_5) = _6.fld0.0 as u16;
_4 = !RET;
_2 = _1;
_11 = !9223372036854775807_isize;
_10 = _8.0.0 as f32;
_1 = core::ptr::addr_of_mut!((*_5));
_2 = _1;
_8.0.1 = !377710547048942595_i64;
_11 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_2) = _11 as u16;
_8.1 = !_8.0.0;
_15.0 = 6106695118593745624_u64 as i16;
Goto(bb3)
}
bb3 = {
(*_1) = !_4;
_3 = RET;
_14 = [892057496615013700_usize,6_usize];
(*_5) = (-116700674875031584401529269826741342266_i128) as u16;
(*_2) = RET;
_6.fld0 = (133_u8, _8.0.2);
_8.0.1 = 3496086594623846523_i64;
_15.1 = _8.0.2 as i64;
_8.1 = _8.0.0 - _15.0;
(*_5) = RET + _4;
_15.0 = (*_1) as i16;
_15.1 = _8.0.1 & _8.0.1;
_13 = '\u{772a9}';
(*_5) = !_4;
Goto(bb4)
}
bb4 = {
_8.1 = !_15.0;
_15 = (_8.1, _8.0.1, _8.0.2);
_5 = _2;
_12 = [23835675607820375068029652302242123059_u128,209388370796417067055610268761196348998_u128];
_15.2 = (-98_i8) as u32;
_21 = _8.0.2 > _6.fld0.1;
_8.0.2 = _6.fld0.1 - _6.fld0.1;
_7 = Adt50::Variant3 { fld0: _11 };
RET = _3 - _4;
_15 = _8.0;
_12 = [269897964821836249246964297560532967598_u128,315537361692054055787861960392779455582_u128];
match _6.fld0.0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
133 => bb9,
_ => bb8
}
}
bb5 = {
(*_1) = !_4;
_3 = RET;
_14 = [892057496615013700_usize,6_usize];
(*_5) = (-116700674875031584401529269826741342266_i128) as u16;
(*_2) = RET;
_6.fld0 = (133_u8, _8.0.2);
_8.0.1 = 3496086594623846523_i64;
_15.1 = _8.0.2 as i64;
_8.1 = _8.0.0 - _15.0;
(*_5) = RET + _4;
_15.0 = (*_1) as i16;
_15.1 = _8.0.1 & _8.0.1;
_13 = '\u{772a9}';
(*_5) = !_4;
Goto(bb4)
}
bb6 = {
RET = false as u16;
_4 = 104301800042512873626830301890033726611_u128 as u16;
_8.0.0 = _8.1 + _8.1;
_8.0 = (_8.1, (-3331530096914783835_i64), _6.fld0.1);
(*_5) = _6.fld0.0 as u16;
_4 = !RET;
_2 = _1;
_11 = !9223372036854775807_isize;
_10 = _8.0.0 as f32;
_1 = core::ptr::addr_of_mut!((*_5));
_2 = _1;
_8.0.1 = !377710547048942595_i64;
_11 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_2) = _11 as u16;
_8.1 = !_8.0.0;
_15.0 = 6106695118593745624_u64 as i16;
Goto(bb3)
}
bb7 = {
_8.0.0 = _6.fld0.1 as i16;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_6.fld0.0 = !215_u8;
_23 = !16593427406348725442_usize;
_4 = _21 as u16;
RET = !_4;
_14 = [_23,_23];
_5 = _1;
_14 = [_23,_23];
_23 = _10 as usize;
_7 = Adt50::Variant3 { fld0: _11 };
_6.fld0.1 = _8.0.2 - _15.2;
_21 = !false;
SetDiscriminant(_7, 3);
_6.fld0 = (170_u8, _8.0.2);
_16 = _11;
_20 = !_6.fld0.0;
(*_2) = _4 * _4;
_3 = _4;
_8.0.2 = _6.fld0.1;
(*_1) = !_4;
Goto(bb10)
}
bb10 = {
Call(_29 = dump_var(6_usize, 21_usize, Move(_21), 15_usize, Move(_15), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_29 = dump_var(6_usize, 16_usize, Move(_16), 13_usize, Move(_13), 30_usize, _30, 30_usize, _30), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u16,mut _2: *mut u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: u16) -> u16 {
mir! {
type RET = u16;
let _8: *mut u128;
let _9: [i8; 4];
let _10: i16;
let _11: [i8; 3];
let _12: *const [i8; 4];
let _13: ();
let _14: ();
{
RET = 16664429468608306435_u64 as u16;
_4 = _5;
_1 = !(*_2);
_10 = (-10179_i16) * 16155_i16;
_3 = (*_2);
_5 = _3;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = _4 >> _1;
_1 = !_5;
RET = 28370591892717435324030903560868132261_u128 as u16;
_6 = (*_2) >> _4;
_9 = [79_i8,(-21_i8),(-2_i8),87_i8];
_2 = core::ptr::addr_of_mut!(RET);
_11 = [108_i8,93_i8,(-127_i8)];
_6 = 3316470991_u32 as u16;
(*_2) = _4;
_1 = (-108881705985659681401412466757660276305_i128) as u16;
_3 = !(*_2);
_9 = [6_i8,1_i8,59_i8,(-41_i8)];
(*_2) = 55981735008599710297716803128389063956_u128 as u16;
Call(_6 = fn8(_7, _7, _3, _7, _5, _4, _7, _3, _4, _3, _7, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !31171_i16;
RET = 65639458599011755289667684889326693406_i128 as u16;
_5 = !_7;
_6 = _7 & _3;
_3 = !_7;
(*_2) = _4;
_1 = (*_2) + _5;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(7_usize, 9_usize, Move(_9), 11_usize, Move(_11), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: u16,mut _8: u16,mut _9: u16,mut _10: u16,mut _11: u16,mut _12: u16) -> u16 {
mir! {
type RET = u16;
let _13: [u16; 8];
let _14: (i16, i64, u32);
let _15: isize;
let _16: [u128; 2];
let _17: Adt49;
let _18: u32;
let _19: Adt47;
let _20: Adt54;
let _21: Adt56;
let _22: i8;
let _23: (u8, u32);
let _24: (i16, i64, u32);
let _25: i32;
let _26: *mut u16;
let _27: isize;
let _28: char;
let _29: Adt52;
let _30: char;
let _31: Adt46;
let _32: (u128,);
let _33: ();
let _34: ();
{
_14 = ((-22352_i16), (-7297778066729252734_i64), 550089206_u32);
_6 = _5;
RET = (-75678278744530012691507648101067269628_i128) as u16;
_2 = !_4;
RET = (-1550514080_i32) as u16;
_17.fld1.0 = _14.2 as u128;
_17.fld5 = (89_u8, _14.2);
_17.fld4.fld0 = (_17.fld5.0, _14.2);
_17.fld0.0 = core::ptr::addr_of_mut!(_17.fld4.fld0.0);
_13 = [_6,_7,_3,_4,_2,_11,_4,_1];
_17.fld7 = 17576337215893889844_u64 as usize;
_6 = _4;
_14.2 = (-32_i8) as u32;
_17.fld2 = 2311872209296448469_u64 as f32;
_17.fld4 = Adt46 { fld0: _17.fld5 };
match _14.0 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768189104 => bb4,
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
_2 = _1 & _7;
_19 = Adt47::Variant0 { fld0: _17.fld4.fld0.1,fld1: _14.0 };
_7 = _5;
_2 = _17.fld2 as u16;
_4 = _1 + _12;
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld3 = _17.fld2 as i8;
_24.2 = _17.fld5.1;
_17.fld1 = (301356904549681459329898850722412511352_u128,);
SetDiscriminant(_19, 1);
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld0.0 = core::ptr::addr_of_mut!(_23.0);
_14.0 = Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0 | Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0;
_16 = [_17.fld1.0,_17.fld1.0];
_16 = [_17.fld1.0,_17.fld1.0];
_23.1 = !_24.2;
match _17.fld5.1 {
0 => bb5,
1 => bb6,
2 => bb7,
550089206 => bb9,
_ => bb8
}
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
_23 = _17.fld5;
SetDiscriminant(_19, 0);
match _17.fld4.fld0.1 {
0 => bb2,
1 => bb10,
2 => bb11,
550089206 => bb13,
_ => bb12
}
}
bb10 = {
_2 = _1 & _7;
_19 = Adt47::Variant0 { fld0: _17.fld4.fld0.1,fld1: _14.0 };
_7 = _5;
_2 = _17.fld2 as u16;
_4 = _1 + _12;
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld3 = _17.fld2 as i8;
_24.2 = _17.fld5.1;
_17.fld1 = (301356904549681459329898850722412511352_u128,);
SetDiscriminant(_19, 1);
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld0.0 = core::ptr::addr_of_mut!(_23.0);
_14.0 = Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0 | Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0;
_16 = [_17.fld1.0,_17.fld1.0];
_16 = [_17.fld1.0,_17.fld1.0];
_23.1 = !_24.2;
match _17.fld5.1 {
0 => bb5,
1 => bb6,
2 => bb7,
550089206 => bb9,
_ => bb8
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_17.fld4.fld0.1 = _23.1;
_17.fld1.0 = !26925102018226496839258758045930441399_u128;
_25 = (-452815643_i32) + 454388047_i32;
match _17.fld5.1 {
0 => bb10,
1 => bb2,
2 => bb14,
550089206 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_2 = _1 & _7;
_19 = Adt47::Variant0 { fld0: _17.fld4.fld0.1,fld1: _14.0 };
_7 = _5;
_2 = _17.fld2 as u16;
_4 = _1 + _12;
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld3 = _17.fld2 as i8;
_24.2 = _17.fld5.1;
_17.fld1 = (301356904549681459329898850722412511352_u128,);
SetDiscriminant(_19, 1);
_19 = Adt47::Variant1 { fld0: _14 };
_17.fld0.0 = core::ptr::addr_of_mut!(_23.0);
_14.0 = Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0 | Field::<(i16, i64, u32)>(Variant(_19, 1), 0).0;
_16 = [_17.fld1.0,_17.fld1.0];
_16 = [_17.fld1.0,_17.fld1.0];
_23.1 = !_24.2;
match _17.fld5.1 {
0 => bb5,
1 => bb6,
2 => bb7,
550089206 => bb9,
_ => bb8
}
}
bb16 = {
_23.1 = _17.fld7 as u32;
_2 = _5;
_16 = [_17.fld1.0,_17.fld1.0];
_22 = -_17.fld3;
_24 = (_14.0, _14.1, _17.fld5.1);
_4 = _12 + _3;
_30 = '\u{4d24e}';
_14.0 = _24.0;
_31.fld0.0 = !_17.fld5.0;
_14.1 = _24.1 * _24.1;
_26 = core::ptr::addr_of_mut!(_3);
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(8_usize, 8_usize, Move(_8), 3_usize, Move(_3), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(8_usize, 9_usize, Move(_9), 24_usize, Move(_24), 23_usize, Move(_23), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(8_usize, 5_usize, Move(_5), 2_usize, Move(_2), 34_usize, _34, 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: *mut u16,mut _8: u16,mut _9: u16,mut _10: u16,mut _11: u16) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _12: [u8; 7];
let _13: *mut u8;
let _14: char;
let _15: Adt47;
let _16: u128;
let _17: u64;
let _18: Adt49;
let _19: f64;
let _20: isize;
let _21: Adt60;
let _22: [i8; 4];
let _23: i16;
let _24: *mut u8;
let _25: u16;
let _26: u64;
let _27: f64;
let _28: f32;
let _29: isize;
let _30: [u16; 8];
let _31: char;
let _32: [u128; 2];
let _33: u64;
let _34: [u8; 7];
let _35: (u8, u32);
let _36: i8;
let _37: [i8; 3];
let _38: f64;
let _39: *mut [u128; 2];
let _40: char;
let _41: usize;
let _42: Adt57;
let _43: (i16, i64, u32);
let _44: [i128; 4];
let _45: i32;
let _46: ();
let _47: ();
{
RET = [39_i8,77_i8,(-69_i8)];
_6 = !_3;
_3 = !_4;
_12 = [60_u8,237_u8,146_u8,76_u8,36_u8,29_u8,56_u8];
_12 = [81_u8,192_u8,245_u8,247_u8,90_u8,252_u8,45_u8];
_11 = _5 >> _10;
_11 = 439670628_i32 as u16;
_14 = '\u{d123a}';
_9 = !_5;
_3 = (-1119504951594624967087443178601738167_i128) as u16;
_14 = '\u{c626}';
_8 = _9 | _9;
_1 = _5 & _6;
_11 = (-8329706204926694671_i64) as u16;
Call(_12 = fn10((*_7), _2, _9, (*_7), _1, _10, _4, _7, _7, _4, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_7) = (-151585590036282622536808044638975498209_i128) as u16;
_6 = _4 * _10;
_3 = 1726637005_i32 as u16;
_6 = _9;
_8 = _1 * _4;
RET = [(-53_i8),(-62_i8),34_i8];
_8 = 11_i8 as u16;
RET = [47_i8,(-18_i8),(-97_i8)];
_7 = core::ptr::addr_of_mut!(_1);
(*_7) = _10 & _2;
_5 = (-4999251301426967427_i64) as u16;
_15 = Adt47::Variant0 { fld0: 3180295711_u32,fld1: (-18114_i16) };
RET = [(-6_i8),34_i8,125_i8];
_12 = [211_u8,241_u8,191_u8,196_u8,51_u8,66_u8,173_u8];
_2 = !_1;
_3 = 7_usize as u16;
_7 = core::ptr::addr_of_mut!(_4);
_14 = '\u{166b3}';
_14 = '\u{ac6e1}';
_3 = _10;
_8 = _10;
_2 = _9;
_15 = Adt47::Variant0 { fld0: 885695957_u32,fld1: 15658_i16 };
_3 = 64876152542294219762127057077803001731_u128 as u16;
(*_7) = (-9223372036854775808_isize) as u16;
Goto(bb2)
}
bb2 = {
_15 = Adt47::Variant0 { fld0: 1914422938_u32,fld1: 23236_i16 };
_8 = _10;
_16 = 35310778483785694193364562507569997958_u128 & 163364898488491861093285568374494175058_u128;
_11 = !_2;
RET = [(-71_i8),40_i8,(-113_i8)];
place!(Field::<u32>(Variant(_15, 0), 0)) = (-927389495_i32) as u32;
_1 = (-125673932140478851827248756518966183099_i128) as u16;
_14 = '\u{3eecd}';
place!(Field::<i16>(Variant(_15, 0), 1)) = 0_usize as i16;
place!(Field::<u32>(Variant(_15, 0), 0)) = Field::<i16>(Variant(_15, 0), 1) as u32;
_12 = [99_u8,86_u8,40_u8,61_u8,170_u8,165_u8,205_u8];
(*_7) = !_8;
_16 = !204784664974346444360152122731169689272_u128;
_8 = (-257830418528339980_i64) as u16;
_6 = !(*_7);
Call(_6 = fn12(_2, _10, _11, _10, (*_7), (*_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = '\u{e9ac4}';
_5 = !_10;
_7 = core::ptr::addr_of_mut!(_10);
_18.fld5 = (82_u8, Field::<u32>(Variant(_15, 0), 0));
_18.fld1.0 = _16;
_13 = core::ptr::addr_of_mut!(_18.fld5.0);
_18.fld0.0 = _13;
_18.fld5.0 = !165_u8;
_18.fld6 = [_16,_18.fld1.0];
_2 = !_6;
_16 = 7627036131497438738_u64 as u128;
_13 = core::ptr::addr_of_mut!(_18.fld4.fld0.0);
_17 = !15981115599628334096_u64;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld4.fld0.0);
_15 = Adt47::Variant0 { fld0: _18.fld5.1,fld1: (-13231_i16) };
place!(Field::<i16>(Variant(_15, 0), 1)) = (-11306_i16) >> _10;
RET = [2_i8,(-65_i8),(-47_i8)];
_9 = !_5;
_5 = _6 * _11;
Goto(bb4)
}
bb4 = {
SetDiscriminant(_15, 0);
_2 = _4;
_14 = '\u{68831}';
(*_13) = _18.fld5.0;
_6 = (*_7) + _10;
place!(Field::<u32>(Variant(_15, 0), 0)) = _18.fld5.1 - _18.fld5.1;
_3 = !_10;
_7 = core::ptr::addr_of_mut!(_1);
_7 = core::ptr::addr_of_mut!(_11);
_18.fld4 = Adt46 { fld0: _18.fld5 };
_4 = _11;
_11 = _5;
_5 = !_6;
_18.fld3 = 5712473136337036071_usize as i8;
Goto(bb5)
}
bb5 = {
_18.fld1 = (_16,);
_17 = 10059894367711464066_u64;
_19 = Field::<u32>(Variant(_15, 0), 0) as f64;
Goto(bb6)
}
bb6 = {
(*_7) = _10;
_18.fld6 = [_18.fld1.0,_18.fld1.0];
_18.fld7 = 395841099816335259_usize | 6_usize;
_4 = (-1395689059_i32) as u16;
_18.fld4.fld0 = _18.fld5;
_6 = (*_7);
(*_13) = !_18.fld5.0;
_18.fld4 = Adt46 { fld0: _18.fld5 };
_14 = '\u{b4076}';
_18.fld5.0 = 51018456292804007089385187706332839220_i128 as u8;
_18.fld5.1 = _18.fld4.fld0.1 - Field::<u32>(Variant(_15, 0), 0);
_18.fld2 = 2638491667957253660_i64 as f32;
_4 = _6;
RET = [_18.fld3,_18.fld3,_18.fld3];
_3 = _6 + _2;
_7 = core::ptr::addr_of_mut!(_11);
_18.fld5.0 = (-37442770924245904013445421844758529093_i128) as u8;
_20 = -(-74_isize);
_18.fld4.fld0 = _18.fld5;
place!(Field::<i16>(Variant(_15, 0), 1)) = !27249_i16;
(*_7) = _4;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
10059894367711464066 => bb9,
_ => bb8
}
}
bb7 = {
(*_7) = (-151585590036282622536808044638975498209_i128) as u16;
_6 = _4 * _10;
_3 = 1726637005_i32 as u16;
_6 = _9;
_8 = _1 * _4;
RET = [(-53_i8),(-62_i8),34_i8];
_8 = 11_i8 as u16;
RET = [47_i8,(-18_i8),(-97_i8)];
_7 = core::ptr::addr_of_mut!(_1);
(*_7) = _10 & _2;
_5 = (-4999251301426967427_i64) as u16;
_15 = Adt47::Variant0 { fld0: 3180295711_u32,fld1: (-18114_i16) };
RET = [(-6_i8),34_i8,125_i8];
_12 = [211_u8,241_u8,191_u8,196_u8,51_u8,66_u8,173_u8];
_2 = !_1;
_3 = 7_usize as u16;
_7 = core::ptr::addr_of_mut!(_4);
_14 = '\u{166b3}';
_14 = '\u{ac6e1}';
_3 = _10;
_8 = _10;
_2 = _9;
_15 = Adt47::Variant0 { fld0: 885695957_u32,fld1: 15658_i16 };
_3 = 64876152542294219762127057077803001731_u128 as u16;
(*_7) = (-9223372036854775808_isize) as u16;
Goto(bb2)
}
bb8 = {
_15 = Adt47::Variant0 { fld0: 1914422938_u32,fld1: 23236_i16 };
_8 = _10;
_16 = 35310778483785694193364562507569997958_u128 & 163364898488491861093285568374494175058_u128;
_11 = !_2;
RET = [(-71_i8),40_i8,(-113_i8)];
place!(Field::<u32>(Variant(_15, 0), 0)) = (-927389495_i32) as u32;
_1 = (-125673932140478851827248756518966183099_i128) as u16;
_14 = '\u{3eecd}';
place!(Field::<i16>(Variant(_15, 0), 1)) = 0_usize as i16;
place!(Field::<u32>(Variant(_15, 0), 0)) = Field::<i16>(Variant(_15, 0), 1) as u32;
_12 = [99_u8,86_u8,40_u8,61_u8,170_u8,165_u8,205_u8];
(*_7) = !_8;
_16 = !204784664974346444360152122731169689272_u128;
_8 = (-257830418528339980_i64) as u16;
_6 = !(*_7);
Call(_6 = fn12(_2, _10, _11, _10, (*_7), (*_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = [_18.fld3,_18.fld3,_18.fld3];
_14 = '\u{c75fe}';
_14 = '\u{ec13b}';
(*_7) = 139927746_i32 as u16;
_22 = [_18.fld3,_18.fld3,_18.fld3,_18.fld3];
_22 = [_18.fld3,_18.fld3,_18.fld3,_18.fld3];
_18.fld1 = (_16,);
_16 = _18.fld1.0 >> _10;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld5.0);
(*_7) = _2;
_6 = _4;
_18.fld5.1 = _18.fld3 as u32;
(*_13) = _18.fld4.fld0.1 as u8;
_18.fld0.0 = core::ptr::addr_of_mut!((*_13));
_18.fld2 = _19 as f32;
_13 = core::ptr::addr_of_mut!(_18.fld4.fld0.0);
_17 = _16 as u64;
_18.fld1 = (_16,);
(*_7) = _14 as u16;
_18.fld7 = 6_usize;
match _18.fld7 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb8,
6 => bb10,
_ => bb7
}
}
bb10 = {
_5 = _4 - _2;
_7 = core::ptr::addr_of_mut!(_2);
RET = [_18.fld3,_18.fld3,_18.fld3];
RET = [_18.fld3,_18.fld3,_18.fld3];
_19 = _20 as f64;
_25 = _18.fld1.0 as u16;
_4 = (*_7);
_6 = _5;
_14 = '\u{10145d}';
_18.fld0 = (_13,);
_18.fld5.0 = (*_13);
(*_13) = !_18.fld5.0;
_2 = _3 ^ _3;
(*_7) = !_10;
_18.fld4 = Adt46 { fld0: _18.fld5 };
_22 = [_18.fld3,_18.fld3,_18.fld3,_18.fld3];
_10 = (*_7);
_18.fld4 = Adt46 { fld0: _18.fld5 };
(*_13) = _18.fld5.0 - _18.fld5.0;
_28 = -_18.fld2;
_22 = [_18.fld3,_18.fld3,_18.fld3,_18.fld3];
Call(_20 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
SetDiscriminant(_15, 1);
_1 = (*_7);
_18.fld2 = _28 - _28;
_1 = !_4;
_18.fld5.1 = _18.fld4.fld0.1 | _18.fld4.fld0.1;
_29 = _18.fld7 as isize;
_17 = 3304897232210345855_u64;
RET = [_18.fld3,_18.fld3,_18.fld3];
_22 = [_18.fld3,_18.fld3,_18.fld3,_18.fld3];
place!(Field::<(i16, i64, u32)>(Variant(_15, 1), 0)).2 = _18.fld5.1 - _18.fld4.fld0.1;
_19 = _16 as f64;
_31 = _14;
_24 = core::ptr::addr_of_mut!(_18.fld4.fld0.0);
_18.fld5.0 = _18.fld7 as u8;
_4 = !_3;
_27 = 33238795198847720851957398029680998379_i128 as f64;
place!(Field::<(i16, i64, u32)>(Variant(_15, 1), 0)).2 = _18.fld4.fld0.1;
_10 = (*_7);
_18.fld4.fld0.0 = _18.fld5.0 + _18.fld5.0;
_32 = [_16,_16];
_11 = !_9;
RET = [_18.fld3,_18.fld3,_18.fld3];
_17 = 7123051330733256612_i64 as u64;
Goto(bb12)
}
bb12 = {
_18.fld6 = _32;
(*_24) = !_18.fld5.0;
_6 = _17 as u16;
place!(Field::<(i16, i64, u32)>(Variant(_15, 1), 0)) = ((-24724_i16), (-4207478682074407556_i64), _18.fld4.fld0.1);
_33 = _17 >> _3;
_38 = -_19;
_30 = [_25,_10,_10,(*_7),(*_7),_5,_5,_25];
_35.0 = !_18.fld4.fld0.0;
(*_13) = _33 as u8;
_35 = _18.fld4.fld0;
_27 = _38 * _19;
_23 = !Field::<(i16, i64, u32)>(Variant(_15, 1), 0).0;
_27 = _19;
_37 = [_18.fld3,_18.fld3,_18.fld3];
_9 = _25 ^ _3;
_42.fld4 = (_18.fld1.0,);
_18.fld4.fld0.1 = !_18.fld5.1;
_42.fld4.0 = _29 as u128;
RET = _37;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld4.fld0.0);
_20 = _29 ^ _29;
_18.fld0 = (_24,);
_39 = core::ptr::addr_of_mut!(_18.fld6);
_40 = _31;
_15 = Adt47::Variant0 { fld0: _18.fld5.1,fld1: _23 };
Call(_42.fld4.0 = fn13(_35, _1, _1, _35, _13, _25, Move(_18.fld4), _32, (*_39)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<i16>(Variant(_15, 0), 1)) = !_23;
match _18.fld7 {
0 => bb11,
1 => bb9,
2 => bb5,
6 => bb15,
_ => bb14
}
}
bb14 = {
(*_7) = (-151585590036282622536808044638975498209_i128) as u16;
_6 = _4 * _10;
_3 = 1726637005_i32 as u16;
_6 = _9;
_8 = _1 * _4;
RET = [(-53_i8),(-62_i8),34_i8];
_8 = 11_i8 as u16;
RET = [47_i8,(-18_i8),(-97_i8)];
_7 = core::ptr::addr_of_mut!(_1);
(*_7) = _10 & _2;
_5 = (-4999251301426967427_i64) as u16;
_15 = Adt47::Variant0 { fld0: 3180295711_u32,fld1: (-18114_i16) };
RET = [(-6_i8),34_i8,125_i8];
_12 = [211_u8,241_u8,191_u8,196_u8,51_u8,66_u8,173_u8];
_2 = !_1;
_3 = 7_usize as u16;
_7 = core::ptr::addr_of_mut!(_4);
_14 = '\u{166b3}';
_14 = '\u{ac6e1}';
_3 = _10;
_8 = _10;
_2 = _9;
_15 = Adt47::Variant0 { fld0: 885695957_u32,fld1: 15658_i16 };
_3 = 64876152542294219762127057077803001731_u128 as u16;
(*_7) = (-9223372036854775808_isize) as u16;
Goto(bb2)
}
bb15 = {
_18.fld1.0 = 41213518236091925678050574751142066411_i128 as u128;
(*_39) = [_16,_42.fld4.0];
_43.2 = _18.fld5.1;
_42.fld1 = [(*_7),_11,_11,_5,_5,_1,_5,_3];
_42.fld2 = core::ptr::addr_of!(_22);
_18.fld4.fld0 = _35;
_18.fld5 = (_18.fld4.fld0.0, _35.1);
_42.fld2 = core::ptr::addr_of!(_22);
_43.0 = -Field::<i16>(Variant(_15, 0), 1);
SetDiscriminant(_15, 0);
_18.fld5 = _18.fld4.fld0;
_42.fld4 = (_16,);
_18.fld1.0 = _18.fld2 as u128;
place!(Field::<u32>(Variant(_15, 0), 0)) = _18.fld4.fld0.1 ^ _35.1;
_33 = !_17;
_42.fld1 = [_4,_11,_2,_25,_2,_10,_25,_9];
_42.fld6 = _37;
place!(Field::<i16>(Variant(_15, 0), 1)) = _23;
_42.fld0 = [(*_24),(*_13),(*_24),_18.fld5.0,(*_24),_35.0];
_18.fld4.fld0.0 = _18.fld5.0;
SetDiscriminant(_15, 1);
_42.fld3 = _18.fld3 ^ _18.fld3;
_7 = core::ptr::addr_of_mut!((*_7));
_43 = (_23, 519444672679625364_i64, _18.fld5.1);
(*_24) = _35.0;
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(9_usize, 32_usize, Move(_32), 4_usize, Move(_4), 33_usize, Move(_33), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(9_usize, 25_usize, Move(_25), 16_usize, Move(_16), 37_usize, Move(_37), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(9_usize, 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, Move(_8), 35_usize, Move(_35)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(9_usize, 22_usize, Move(_22), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: u16,mut _8: *mut u16,mut _9: *mut u16,mut _10: u16,mut _11: u16) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _12: Adt46;
let _13: Adt62;
let _14: char;
let _15: *mut u16;
let _16: [u128; 2];
let _17: char;
let _18: [usize; 2];
let _19: isize;
let _20: bool;
let _21: f64;
let _22: Adt49;
let _23: Adt59;
let _24: Adt61;
let _25: *mut u16;
let _26: [u128; 2];
let _27: Adt51;
let _28: f32;
let _29: [u8; 7];
let _30: isize;
let _31: isize;
let _32: i8;
let _33: ();
let _34: ();
{
RET = [188_u8,160_u8,231_u8,94_u8,123_u8,48_u8,118_u8];
_10 = (*_8);
_9 = _8;
RET = [175_u8,197_u8,110_u8,80_u8,143_u8,31_u8,198_u8];
_12.fld0 = (238_u8, 1899141546_u32);
_6 = _12.fld0.0 as u16;
match _12.fld0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
238 => bb7,
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
_14 = '\u{9e30d}';
_12.fld0 = (74_u8, 1195892675_u32);
(*_8) = _10;
_12.fld0.1 = !1714921713_u32;
(*_9) = !_4;
_10 = 9223372036854775807_isize as u16;
(*_9) = _2;
_16 = [282349386786033022840861808113665491989_u128,244016343983271172504256394641303572946_u128];
_10 = !_7;
(*_9) = 9223372036854775807_isize as u16;
_12.fld0.1 = true as u32;
_1 = !_4;
_16 = [242411402272284013503054297808245431147_u128,217478527272863993187991591730020206953_u128];
_15 = _8;
_11 = 74514293990894433914916363210230139207_u128 as u16;
_14 = '\u{8ccbe}';
_2 = !_5;
_16 = [10576085839968252883840293446889784028_u128,144025316261348086403330013004612511644_u128];
Goto(bb8)
}
bb8 = {
_7 = !_3;
match _12.fld0.0 {
0 => bb1,
1 => bb5,
2 => bb3,
74 => bb9,
_ => bb4
}
}
bb9 = {
(*_9) = _5 & _5;
(*_15) = _1 + _2;
_4 = (-122_i8) as u16;
_1 = 64468303329423303038687791265915475601_u128 as u16;
RET = [_12.fld0.0,_12.fld0.0,_12.fld0.0,_12.fld0.0,_12.fld0.0,_12.fld0.0,_12.fld0.0];
(*_8) = _10;
(*_15) = !_10;
_4 = _10;
_16 = [272125030873865111277538238795509351484_u128,299439803878447350742300825689938699246_u128];
(*_9) = !_10;
_2 = 1818046314_i32 as u16;
(*_9) = _4;
_5 = (-88_i8) as u16;
_19 = -9223372036854775807_isize;
_20 = _10 != (*_8);
Goto(bb10)
}
bb10 = {
_11 = _12.fld0.1 as u16;
_12.fld0 = (164_u8, 3953199080_u32);
_19 = -(-9223372036854775808_isize);
_17 = _14;
_17 = _14;
_9 = _8;
_16 = [319431023922791034063440031416813668399_u128,68050653799581980383737932057819515080_u128];
_18 = [6_usize,11026941759684463534_usize];
(*_9) = !_10;
_22.fld4.fld0 = (_12.fld0.0, _12.fld0.1);
_12.fld0 = _22.fld4.fld0;
_22.fld6 = [109658651674892419298547001045725657082_u128,58665302139843512647503298150424124996_u128];
_22.fld1 = (225858630477914918064482258488634472514_u128,);
_4 = (*_15);
(*_15) = _7 - _7;
_22.fld4.fld0.0 = _12.fld0.0 % _12.fld0.0;
_22.fld5 = (_22.fld4.fld0.0, _22.fld4.fld0.1);
Call(_22.fld5.1 = core::intrinsics::bswap(_12.fld0.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_18 = [7_usize,2_usize];
(*_9) = _5;
Call(_20 = fn11(_4, _3, _7, _4, _7, _22.fld4.fld0.1, (*_15), _12.fld0, _4, _4, _12.fld0.0, _3, _4, _7, _7, _22.fld5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_12.fld0.0 = _22.fld5.0;
_3 = _10;
_22.fld6 = _16;
_22.fld1 = (117365730996624740370223007755177388482_u128,);
_19 = (-9223372036854775808_isize);
_22.fld1.0 = 55125306872041037021902597675808922744_u128 + 211347855001289925249920362302862934478_u128;
_15 = core::ptr::addr_of_mut!((*_8));
_5 = _7;
_16 = [_22.fld1.0,_22.fld1.0];
_17 = _14;
_12.fld0.0 = 8278_i16 as u8;
_14 = _17;
_25 = _8;
_22.fld0.0 = core::ptr::addr_of_mut!(_22.fld4.fld0.0);
_22.fld2 = _12.fld0.1 as f32;
_5 = !_10;
_1 = _10 << _3;
_5 = _10;
(*_9) = _22.fld4.fld0.1 as u16;
RET = [_22.fld5.0,_22.fld5.0,_22.fld4.fld0.0,_22.fld5.0,_22.fld4.fld0.0,_22.fld4.fld0.0,_22.fld5.0];
_14 = _17;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9 = core::ptr::addr_of_mut!((*_25));
_22.fld6 = [_22.fld1.0,_22.fld1.0];
_12.fld0.0 = _22.fld1.0 as u8;
_25 = core::ptr::addr_of_mut!(_5);
_26 = [_22.fld1.0,_22.fld1.0];
_19 = 9223372036854775807_isize | 70_isize;
_20 = true ^ false;
Goto(bb14)
}
bb14 = {
_28 = _22.fld2;
_1 = (*_25) | (*_25);
_31 = -_19;
_19 = _31 | _31;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(10_usize, 2_usize, Move(_2), 11_usize, Move(_11), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(10_usize, 1_usize, Move(_1), 26_usize, Move(_26), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u32,mut _7: u16,mut _8: (u8, u32),mut _9: u16,mut _10: u16,mut _11: u8,mut _12: u16,mut _13: u16,mut _14: u16,mut _15: u16,mut _16: (u8, u32)) -> bool {
mir! {
type RET = bool;
let _17: u128;
let _18: isize;
let _19: u32;
let _20: (u8, u32);
let _21: *const [i8; 4];
let _22: u8;
let _23: i128;
let _24: ();
let _25: ();
{
_10 = _14;
_12 = _2 + _9;
_1 = _5 - _13;
_8.0 = _16.0 | _16.0;
_15 = _12 ^ _1;
_6 = _8.1;
Goto(bb1)
}
bb1 = {
_8.0 = _16.0;
_15 = _2 & _4;
_3 = _15;
_14 = (-5466_i16) as u16;
RET = _4 <= _3;
_10 = _9 ^ _12;
_6 = _16.1 ^ _16.1;
_20.0 = !_8.0;
_12 = !_10;
_19 = !_6;
_5 = _11 as u16;
_20 = _8;
_15 = _10;
_3 = !_1;
_8.0 = _11 % _11;
_23 = RET as i128;
Goto(bb2)
}
bb2 = {
Call(_24 = dump_var(11_usize, 8_usize, Move(_8), 1_usize, Move(_1), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_24 = dump_var(11_usize, 7_usize, Move(_7), 9_usize, Move(_9), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_24 = dump_var(11_usize, 12_usize, Move(_12), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16) -> u16 {
mir! {
type RET = u16;
let _7: f32;
let _8: u16;
let _9: u32;
let _10: ();
let _11: ();
{
RET = !_3;
_3 = 3928888658944410608_u64 as u16;
_3 = !_1;
_5 = !_6;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(12_usize, 1_usize, Move(_1), 3_usize, Move(_3), 6_usize, Move(_6), 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: (u8, u32),mut _2: u16,mut _3: u16,mut _4: (u8, u32),mut _5: *mut u8,mut _6: u16,mut _7: Adt46,mut _8: [u128; 2],mut _9: [u128; 2]) -> u128 {
mir! {
type RET = u128;
let _10: Adt50;
let _11: [u8; 6];
let _12: char;
let _13: bool;
let _14: i128;
let _15: u64;
let _16: Adt53;
let _17: [u128; 2];
let _18: Adt61;
let _19: u128;
let _20: Adt50;
let _21: Adt59;
let _22: *mut u16;
let _23: f32;
let _24: ();
let _25: ();
{
_11 = [_1.0,_7.fld0.0,_1.0,_7.fld0.0,_7.fld0.0,_1.0];
_7.fld0.0 = !_4.0;
_4.0 = _7.fld0.0;
_4.0 = (-101_i8) as u8;
_7 = Adt46 { fld0: _1 };
_7.fld0.1 = (-114_i8) as u32;
_6 = 312346467057231681790032131000426386310_u128 as u16;
Call(_10 = fn14(_7.fld0.0, _3, _2, _5, Move(_7), _3, _2, _9, _11, _3, _11, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = _1.0 <= _1.0;
_2 = _3;
place!(Field::<isize>(Variant(_10, 3), 0)) = _1.1 as isize;
_7.fld0 = (_1.0, _1.1);
SetDiscriminant(_10, 3);
RET = _7.fld0.1 as u128;
place!(Field::<isize>(Variant(_10, 3), 0)) = -(-9223372036854775808_isize);
_11 = [_1.0,_1.0,_7.fld0.0,_7.fld0.0,_7.fld0.0,_1.0];
_4.1 = !_1.1;
_8 = _9;
_8 = _9;
_7 = Adt46 { fld0: _1 };
_4.1 = _7.fld0.1;
place!(Field::<isize>(Variant(_10, 3), 0)) = _1.1 as isize;
Call(place!(Field::<isize>(Variant(_10, 3), 0)) = fn17(_1.0, _1.0, _7.fld0, _13, _13, _3, _5, _1.0, _7.fld0.0, _7.fld0, _7.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _7.fld0;
_4.1 = _1.1 + _1.1;
RET = 325442030923283992857443768034661243403_u128;
_8 = _9;
_1.1 = _4.1 & _4.1;
_9 = _8;
_7.fld0.0 = !_1.0;
_14 = (-105686321947938664021250353711434211932_i128);
RET = 115_i8 as u128;
_11 = [_7.fld0.0,_1.0,_7.fld0.0,_1.0,_7.fld0.0,_1.0];
RET = 205326722882249756974190742563321017016_u128;
_7 = Adt46 { fld0: _1 };
_16 = Adt53::Variant0 { fld0: _9 };
_6 = _3 >> _3;
_4.0 = _7.fld0.0 * _1.0;
_7.fld0 = (_1.0, _1.1);
_16 = Adt53::Variant0 { fld0: _8 };
RET = 198646333417813663283874512048652938270_u128 << _7.fld0.0;
place!(Field::<[u128; 2]>(Variant(_16, 0), 0)) = [RET,RET];
_4 = (_1.0, _7.fld0.1);
_12 = '\u{8cb75}';
_12 = '\u{3d568}';
_20 = _10;
_7.fld0.0 = _4.0;
_19 = !RET;
Goto(bb3)
}
bb3 = {
Call(_24 = dump_var(13_usize, 8_usize, Move(_8), 11_usize, Move(_11), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_24 = dump_var(13_usize, 1_usize, Move(_1), 2_usize, Move(_2), 25_usize, _25, 25_usize, _25), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u8,mut _2: u16,mut _3: u16,mut _4: *mut u8,mut _5: Adt46,mut _6: u16,mut _7: u16,mut _8: [u128; 2],mut _9: [u8; 6],mut _10: u16,mut _11: [u8; 6],mut _12: *mut u8) -> Adt50 {
mir! {
type RET = Adt50;
let _13: Adt47;
let _14: Adt55;
let _15: i64;
let _16: &'static u16;
let _17: Adt54;
let _18: isize;
let _19: f32;
let _20: i8;
let _21: *mut [u128; 2];
let _22: u32;
let _23: isize;
let _24: (u16, u8, i32, f64);
let _25: f32;
let _26: ();
let _27: ();
{
_10 = _3 << _7;
_6 = _3;
_10 = !_2;
RET = Adt50::Variant3 { fld0: 9223372036854775807_isize };
_7 = 9223372036854775807_isize as u16;
_1 = (-9223372036854775808_isize) as u8;
place!(Field::<isize>(Variant(RET, 3), 0)) = -(-9223372036854775808_isize);
_5.fld0 = (_1, 2946584786_u32);
_14.fld6 = core::ptr::addr_of!(_14.fld0);
place!(Field::<isize>(Variant(RET, 3), 0)) = -9223372036854775807_isize;
_5.fld0 = (_1, 2527678669_u32);
_11 = _9;
_14.fld0 = _2 < _10;
_3 = _2;
_14.fld1 = core::ptr::addr_of_mut!(_5.fld0.1);
_11 = [_1,_5.fld0.0,_5.fld0.0,_5.fld0.0,_5.fld0.0,_1];
_11 = _9;
_14.fld7 = Adt50::Variant3 { fld0: Field::<isize>(Variant(RET, 3), 0) };
_14.fld3 = core::ptr::addr_of_mut!(_14.fld2);
place!(Field::<isize>(Variant(RET, 3), 0)) = Field::<isize>(Variant(_14.fld7, 3), 0);
Goto(bb1)
}
bb1 = {
_14.fld7 = RET;
_5.fld0.1 = 17276_i16 as u32;
_13 = Adt47::Variant0 { fld0: _5.fld0.1,fld1: 805_i16 };
place!(Field::<u32>(Variant(_13, 0), 0)) = '\u{866e4}' as u32;
_14.fld7 = RET;
_5.fld0.0 = _1 * _1;
place!(Field::<isize>(Variant(RET, 3), 0)) = Field::<isize>(Variant(_14.fld7, 3), 0) & Field::<isize>(Variant(_14.fld7, 3), 0);
_2 = _10;
Call(_14.fld4 = fn15(_2, _4, _10, _14.fld6, _14.fld6, Move(_5), _4, _12, _4, _4, _4, _8, _4, _12, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.fld0 = (_1, Field::<u32>(Variant(_13, 0), 0));
_15 = (-4397892790900422497_i64);
place!(Field::<u32>(Variant(_13, 0), 0)) = 2083969035_i32 as u32;
_5.fld0.1 = _2 as u32;
_8 = [226477474723473037937401904462222146175_u128,100425748390255241847020617633502077968_u128];
_14.fld6 = core::ptr::addr_of!(_14.fld0);
_5.fld0.0 = !_1;
_14.fld0 = false;
_14.fld1 = core::ptr::addr_of_mut!(_5.fld0.1);
_5.fld0.1 = !Field::<u32>(Variant(_13, 0), 0);
place!(Field::<i16>(Variant(_13, 0), 1)) = (-24481_i16);
_4 = core::ptr::addr_of_mut!(_1);
_13 = Adt47::Variant0 { fld0: _5.fld0.1,fld1: 23115_i16 };
(*_4) = _5.fld0.0 >> _10;
_1 = _5.fld0.0 | _5.fld0.0;
_5.fld0.0 = _1;
_3 = !_2;
_16 = &_6;
_10 = (*_16);
_4 = _12;
_15 = '\u{dd712}' as i64;
place!(Field::<isize>(Variant(_14.fld7, 3), 0)) = (*_16) as isize;
_12 = core::ptr::addr_of_mut!(_5.fld0.0);
Goto(bb3)
}
bb3 = {
_19 = 72550520_i32 as f32;
_22 = Field::<u32>(Variant(_13, 0), 0);
_16 = &_2;
place!(Field::<isize>(Variant(RET, 3), 0)) = 62_i8 as isize;
_21 = core::ptr::addr_of_mut!(_8);
_14.fld1 = core::ptr::addr_of_mut!(_5.fld0.1);
(*_21) = [309928652516823546252085808883885285992_u128,108721518480782680145841740261021044260_u128];
_2 = _3;
RET = _14.fld7;
_14.fld4.0 = [65_i8,(-35_i8),59_i8];
_11 = _9;
_14.fld7 = RET;
SetDiscriminant(_14.fld7, 3);
_25 = _1 as f32;
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(14_usize, 6_usize, Move(_6), 8_usize, Move(_8), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(14_usize, 3_usize, Move(_3), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u16,mut _2: *mut u8,mut _3: u16,mut _4: *const bool,mut _5: *const bool,mut _6: Adt46,mut _7: *mut u8,mut _8: *mut u8,mut _9: *mut u8,mut _10: *mut u8,mut _11: *mut u8,mut _12: [u128; 2],mut _13: *mut u8,mut _14: *mut u8,mut _15: u16) -> ([i8; 3],) {
mir! {
type RET = ([i8; 3],);
let _16: f64;
let _17: (u16, u8, i32, f64);
let _18: f64;
let _19: *mut [u128; 2];
let _20: u32;
let _21: u8;
let _22: usize;
let _23: (u8, u32);
let _24: isize;
let _25: usize;
let _26: bool;
let _27: char;
let _28: i128;
let _29: isize;
let _30: f64;
let _31: [usize; 6];
let _32: isize;
let _33: char;
let _34: ((u16, u8, i32, f64),);
let _35: f64;
let _36: isize;
let _37: f64;
let _38: isize;
let _39: f32;
let _40: [isize; 2];
let _41: [i8; 3];
let _42: *mut u8;
let _43: *mut f32;
let _44: Adt59;
let _45: ();
let _46: ();
{
RET.0 = [(-42_i8),13_i8,(-112_i8)];
_3 = _15;
_6.fld0.0 = 62_u8;
_17.0 = !_3;
Goto(bb1)
}
bb1 = {
RET.0 = [(-95_i8),120_i8,117_i8];
_9 = core::ptr::addr_of_mut!(_6.fld0.0);
_6.fld0 = (40_u8, 2103655435_u32);
_17.0 = _15 | _15;
_7 = core::ptr::addr_of_mut!(_17.1);
(*_9) = _6.fld0.1 as u8;
(*_9) = !124_u8;
Call(_2 = fn16((*_4), _14, (*_4), _17.0, (*_5), _11, (*_4), _14, _4, _8, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = [(-104_i8),(-90_i8),(-4_i8)];
_6.fld0.1 = !2828242680_u32;
_17.2 = !1922041196_i32;
_2 = _13;
_8 = core::ptr::addr_of_mut!((*_9));
_15 = _3;
_13 = _2;
_20 = _6.fld0.1;
(*_4) = !true;
_23.0 = _17.2 as u8;
_17.3 = (-8572210345210329003_i64) as f64;
_19 = core::ptr::addr_of_mut!(_12);
(*_8) = _23.0 >> _1;
_17.2 = (-1357017929_i32);
_23.1 = !_6.fld0.1;
_18 = _17.3 - _17.3;
_8 = _9;
_5 = core::ptr::addr_of!((*_4));
_22 = '\u{8b70}' as usize;
_16 = _3 as f64;
_17.3 = _16 - _16;
match _17.2 {
340282366920938463463374607430411193527 => bb3,
_ => bb1
}
}
bb3 = {
_17.1 = !_6.fld0.0;
_17.1 = (*_9) & _6.fld0.0;
_18 = -_16;
_8 = core::ptr::addr_of_mut!((*_8));
(*_19) = [124921726384836995709308867562460743559_u128,275388583660793913579954119676186988789_u128];
(*_9) = (*_7) ^ _17.1;
(*_4) = false;
_17.2 = -661716083_i32;
(*_4) = !false;
RET.0 = [(-83_i8),86_i8,(-70_i8)];
(*_9) = !(*_7);
_23 = _6.fld0;
_17.2 = (-1521967982_i32);
(*_9) = !_17.1;
_20 = _6.fld0.1 - _6.fld0.1;
(*_9) = (*_7);
_6.fld0 = ((*_7), _20);
_15 = !_1;
_17 = (_15, (*_9), (-1003484918_i32), _18);
_6 = Adt46 { fld0: _23 };
_9 = core::ptr::addr_of_mut!((*_8));
_22 = !17316437390847873540_usize;
_19 = core::ptr::addr_of_mut!((*_19));
_14 = _8;
match _17.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607430764726538 => bb7,
_ => bb6
}
}
bb4 = {
RET.0 = [(-104_i8),(-90_i8),(-4_i8)];
_6.fld0.1 = !2828242680_u32;
_17.2 = !1922041196_i32;
_2 = _13;
_8 = core::ptr::addr_of_mut!((*_9));
_15 = _3;
_13 = _2;
_20 = _6.fld0.1;
(*_4) = !true;
_23.0 = _17.2 as u8;
_17.3 = (-8572210345210329003_i64) as f64;
_19 = core::ptr::addr_of_mut!(_12);
(*_8) = _23.0 >> _1;
_17.2 = (-1357017929_i32);
_23.1 = !_6.fld0.1;
_18 = _17.3 - _17.3;
_8 = _9;
_5 = core::ptr::addr_of!((*_4));
_22 = '\u{8b70}' as usize;
_16 = _3 as f64;
_17.3 = _16 - _16;
match _17.2 {
340282366920938463463374607430411193527 => bb3,
_ => bb1
}
}
bb5 = {
RET.0 = [(-95_i8),120_i8,117_i8];
_9 = core::ptr::addr_of_mut!(_6.fld0.0);
_6.fld0 = (40_u8, 2103655435_u32);
_17.0 = _15 | _15;
_7 = core::ptr::addr_of_mut!(_17.1);
(*_9) = _6.fld0.1 as u8;
(*_9) = !124_u8;
Call(_2 = fn16((*_4), _14, (*_4), _17.0, (*_5), _11, (*_4), _14, _4, _8, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_13 = core::ptr::addr_of_mut!(_6.fld0.0);
_25 = _22;
_23 = ((*_7), _6.fld0.1);
_1 = _17.0 & _17.0;
_26 = _1 == _1;
_12 = [1309425562594904871387680620819897908_u128,205917236532472917638323446230407918748_u128];
_17.1 = !(*_14);
(*_14) = !(*_7);
_23 = ((*_9), _6.fld0.1);
Goto(bb8)
}
bb8 = {
_29 = !(-9223372036854775808_isize);
(*_4) = !_26;
_29 = (-90_isize);
(*_7) = _23.0;
(*_7) = !_6.fld0.0;
_28 = (-20768599999982364111126695803506727352_i128) << (*_9);
_13 = _14;
_5 = _4;
_6.fld0.1 = _20;
_23.0 = _17.1 ^ (*_8);
(*_4) = _17.0 != _17.0;
_27 = '\u{d28ca}';
_4 = _5;
_31 = [_22,_22,_22,_22,_22,_22];
_13 = _7;
RET.0 = [107_i8,(-47_i8),66_i8];
(*_9) = _17.1;
(*_9) = 103185613123336583984262077692953426964_u128 as u8;
_10 = _7;
_30 = _17.3 * _18;
(*_4) = !_26;
_6.fld0.1 = _20 * _23.1;
_25 = _22;
(*_5) = _17.3 < _30;
_34 = (_17,);
_21 = 13785262838760837907_u64 as u8;
_30 = _34.0.3;
Goto(bb9)
}
bb9 = {
_34.0.3 = _34.0.2 as f64;
_15 = !_1;
(*_5) = _26 ^ _26;
_35 = _16;
_23.1 = _20;
_32 = -_29;
_4 = core::ptr::addr_of!((*_5));
_12 = [307659995176564640708466427836471837802_u128,139575079740921465006133497315751009976_u128];
_17 = _34.0;
_28 = !48160336657106384410509663318559175864_i128;
_7 = _2;
_24 = _28 as isize;
_6.fld0.1 = !_23.1;
_2 = _7;
_17.1 = !_34.0.1;
_17.0 = (-31645_i16) as u16;
_8 = _2;
_23.0 = (*_13) + _34.0.1;
_34.0.1 = (*_10) | _17.1;
(*_13) = (-126_i8) as u8;
_4 = core::ptr::addr_of!((*_4));
_28 = !(-143682324853278562751837873633760649038_i128);
(*_14) = _27 as u8;
_27 = '\u{979d0}';
match _34.0.2 {
0 => bb2,
1 => bb10,
2 => bb11,
340282366920938463463374607430764726538 => bb13,
_ => bb12
}
}
bb10 = {
RET.0 = [(-104_i8),(-90_i8),(-4_i8)];
_6.fld0.1 = !2828242680_u32;
_17.2 = !1922041196_i32;
_2 = _13;
_8 = core::ptr::addr_of_mut!((*_9));
_15 = _3;
_13 = _2;
_20 = _6.fld0.1;
(*_4) = !true;
_23.0 = _17.2 as u8;
_17.3 = (-8572210345210329003_i64) as f64;
_19 = core::ptr::addr_of_mut!(_12);
(*_8) = _23.0 >> _1;
_17.2 = (-1357017929_i32);
_23.1 = !_6.fld0.1;
_18 = _17.3 - _17.3;
_8 = _9;
_5 = core::ptr::addr_of!((*_4));
_22 = '\u{8b70}' as usize;
_16 = _3 as f64;
_17.3 = _16 - _16;
match _17.2 {
340282366920938463463374607430411193527 => bb3,
_ => bb1
}
}
bb11 = {
_17.1 = !_6.fld0.0;
_17.1 = (*_9) & _6.fld0.0;
_18 = -_16;
_8 = core::ptr::addr_of_mut!((*_8));
(*_19) = [124921726384836995709308867562460743559_u128,275388583660793913579954119676186988789_u128];
(*_9) = (*_7) ^ _17.1;
(*_4) = false;
_17.2 = -661716083_i32;
(*_4) = !false;
RET.0 = [(-83_i8),86_i8,(-70_i8)];
(*_9) = !(*_7);
_23 = _6.fld0;
_17.2 = (-1521967982_i32);
(*_9) = !_17.1;
_20 = _6.fld0.1 - _6.fld0.1;
(*_9) = (*_7);
_6.fld0 = ((*_7), _20);
_15 = !_1;
_17 = (_15, (*_9), (-1003484918_i32), _18);
_6 = Adt46 { fld0: _23 };
_9 = core::ptr::addr_of_mut!((*_8));
_22 = !17316437390847873540_usize;
_19 = core::ptr::addr_of_mut!((*_19));
_14 = _8;
match _17.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607430764726538 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
_14 = _7;
_39 = (-118_i8) as f32;
_17.3 = -_16;
RET.0 = [55_i8,66_i8,92_i8];
_25 = 2928475306499411717_i64 as usize;
_22 = _25;
Goto(bb14)
}
bb14 = {
(*_19) = [5734196175397175416917883964397236587_u128,135168120714524614637995029135405115183_u128];
_5 = _4;
_22 = _25 & _25;
_17 = _34.0;
_38 = _24 | _32;
_38 = _29 + _29;
(*_10) = _34.0.1;
_37 = _16;
_43 = core::ptr::addr_of_mut!(_39);
(*_4) = _26;
_27 = '\u{202c0}';
_15 = (-22712_i16) as u16;
_34.0 = (_3, (*_10), _17.2, _37);
_42 = core::ptr::addr_of_mut!(_34.0.1);
_30 = -_34.0.3;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(15_usize, 25_usize, Move(_25), 15_usize, Move(_15), 24_usize, Move(_24), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(15_usize, 23_usize, Move(_23), 32_usize, Move(_32), 27_usize, Move(_27), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: *mut u8,mut _3: bool,mut _4: u16,mut _5: bool,mut _6: *mut u8,mut _7: bool,mut _8: *mut u8,mut _9: *const bool,mut _10: *mut u8,mut _11: bool) -> *mut u8 {
mir! {
type RET = *mut u8;
let _12: isize;
let _13: (i16, i64, u32);
let _14: (*mut u8,);
let _15: ();
let _16: ();
{
RET = _10;
_7 = _3 == _5;
_4 = !35499_u16;
(*_9) = _7;
_5 = _7 < _1;
RET = _8;
_5 = !_3;
_3 = _1 != (*_9);
_13.1 = 2086915564292791662_i64 | (-77151998728918202_i64);
_13 = ((-13294_i16), (-7881094552931792119_i64), 1884504332_u32);
_3 = !(*_9);
_9 = core::ptr::addr_of!((*_9));
_8 = RET;
_10 = _8;
_4 = 18077735935254208071_usize as u16;
_14 = (_2,);
_6 = _8;
_9 = core::ptr::addr_of!(_5);
_9 = core::ptr::addr_of!((*_9));
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(16_usize, 13_usize, Move(_13), 5_usize, Move(_5), 4_usize, Move(_4), 16_usize, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u8,mut _2: u8,mut _3: (u8, u32),mut _4: bool,mut _5: bool,mut _6: u16,mut _7: *mut u8,mut _8: u8,mut _9: u8,mut _10: (u8, u32),mut _11: (u8, u32)) -> isize {
mir! {
type RET = isize;
let _12: [usize; 2];
let _13: f64;
let _14: f64;
let _15: isize;
let _16: char;
let _17: u64;
let _18: (i16, f32, usize);
let _19: [i8; 3];
let _20: [isize; 2];
let _21: (i16, i64, u32);
let _22: isize;
let _23: f64;
let _24: char;
let _25: (u128,);
let _26: [i8; 3];
let _27: u8;
let _28: isize;
let _29: Adt62;
let _30: *mut [u128; 2];
let _31: f32;
let _32: Adt46;
let _33: isize;
let _34: (u8, u32);
let _35: isize;
let _36: ();
let _37: ();
{
_11.1 = _3.1;
_10 = _3;
_11.0 = _9;
RET = !(-73_isize);
_3 = (_1, _11.1);
_2 = 1425832764530147933_u64 as u8;
_2 = _3.0 | _1;
_11.0 = _8 - _1;
_9 = _11.0 ^ _3.0;
_6 = 29834_u16 ^ 24795_u16;
_9 = _11.0 * _10.0;
_3.0 = !_11.0;
_11.1 = _3.1 & _3.1;
RET = (-112_isize) | (-62_isize);
_13 = _6 as f64;
_7 = core::ptr::addr_of_mut!(_8);
_11.0 = _11.1 as u8;
_3 = (_8, _10.1);
RET = 10_isize * (-9223372036854775808_isize);
_11.1 = !_3.1;
_14 = _13;
_3.1 = _10.1;
_3 = _10;
_11.0 = _9 >> _9;
Goto(bb1)
}
bb1 = {
_8 = _2 << _3.0;
_6 = 46275_u16 >> _11.0;
_10.0 = _6 as u8;
_11.0 = (*_7);
RET = (-9223372036854775808_isize) - (-83_isize);
(*_7) = _5 as u8;
(*_7) = _10.0 | _11.0;
_10 = (_3.0, _3.1);
_2 = (*_7) >> _8;
_2 = !_9;
(*_7) = 7990680858026282188_i64 as u8;
_10.0 = 9748760271626838617_usize as u8;
_18.2 = _13 as usize;
_5 = _4;
_10.1 = _11.1;
_11.1 = 301654366158094199492120001367017447000_u128 as u32;
_16 = '\u{3c0a8}';
_15 = RET ^ RET;
_10.1 = _11.1 | _11.1;
_10.1 = _11.1 >> _2;
_3 = (_1, _10.1);
_12 = [_18.2,_18.2];
Goto(bb2)
}
bb2 = {
_10 = _3;
_11 = _3;
_19 = [0_i8,90_i8,(-92_i8)];
(*_7) = _9;
_9 = !_2;
_4 = _5;
_5 = (*_7) >= _1;
Goto(bb3)
}
bb3 = {
_10.1 = !_11.1;
_9 = _3.0 & _8;
RET = _15 + _15;
_18.1 = _13 as f32;
_13 = _14;
_20 = [RET,RET];
RET = -_15;
_20 = [_15,RET];
_11 = (_3.0, _3.1);
_17 = 5441984438298047837_u64;
Goto(bb4)
}
bb4 = {
_16 = '\u{e3007}';
_10.0 = _1 * _2;
_21.2 = _11.1 + _11.1;
_21.0 = !(-3375_i16);
_16 = '\u{496dc}';
_11.1 = _10.0 as u32;
Goto(bb5)
}
bb5 = {
_13 = _14 * _14;
_2 = (*_7);
_21 = ((-10764_i16), 8206989907852173695_i64, _11.1);
_19 = [28_i8,48_i8,(-7_i8)];
Call(_24 = fn18(_9, _21.1, _21), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.1 = !_21.2;
Goto(bb7)
}
bb7 = {
_24 = _16;
_26 = _19;
_16 = _24;
_6 = 17902_u16;
_20 = [_15,_15];
_21.1 = -3720242488086384536_i64;
_11.1 = !_3.1;
_17 = 16940941457315705113_u64 & 1276827095420653288_u64;
_3.0 = !_2;
_3 = (_1, _11.1);
Goto(bb8)
}
bb8 = {
_22 = RET * _15;
_19 = [53_i8,43_i8,(-18_i8)];
_12 = [_18.2,_18.2];
_7 = core::ptr::addr_of_mut!(_9);
_25.0 = 150305026426365124964625314170322077714_u128 >> _9;
_25 = (47930431876184928412515999002177495671_u128,);
_8 = _11.0;
_3.0 = _1 & (*_7);
_18.2 = 11295985428104168186_usize;
_8 = _2;
_3 = (_8, _21.2);
_10.0 = _5 as u8;
_25 = (299548823238892848144811049098981824003_u128,);
_21 = ((-32083_i16), 4728970322040124954_i64, _11.1);
_20 = [RET,_22];
_11.0 = (*_7);
_31 = _18.1;
_25 = (119451238334950536660559609048330009569_u128,);
_18.0 = _21.0 ^ _21.0;
_26 = [(-63_i8),44_i8,3_i8];
RET = -_15;
_32.fld0.1 = _3.1;
match _21.1 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
4728970322040124954 => bb9,
_ => bb6
}
}
bb9 = {
_34.0 = _18.1 as u8;
match _21.1 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
4728970322040124954 => bb16,
_ => bb15
}
}
bb10 = {
_22 = RET * _15;
_19 = [53_i8,43_i8,(-18_i8)];
_12 = [_18.2,_18.2];
_7 = core::ptr::addr_of_mut!(_9);
_25.0 = 150305026426365124964625314170322077714_u128 >> _9;
_25 = (47930431876184928412515999002177495671_u128,);
_8 = _11.0;
_3.0 = _1 & (*_7);
_18.2 = 11295985428104168186_usize;
_8 = _2;
_3 = (_8, _21.2);
_10.0 = _5 as u8;
_25 = (299548823238892848144811049098981824003_u128,);
_21 = ((-32083_i16), 4728970322040124954_i64, _11.1);
_20 = [RET,_22];
_11.0 = (*_7);
_31 = _18.1;
_25 = (119451238334950536660559609048330009569_u128,);
_18.0 = _21.0 ^ _21.0;
_26 = [(-63_i8),44_i8,3_i8];
RET = -_15;
_32.fld0.1 = _3.1;
match _21.1 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
4728970322040124954 => bb9,
_ => bb6
}
}
bb11 = {
_8 = _2 << _3.0;
_6 = 46275_u16 >> _11.0;
_10.0 = _6 as u8;
_11.0 = (*_7);
RET = (-9223372036854775808_isize) - (-83_isize);
(*_7) = _5 as u8;
(*_7) = _10.0 | _11.0;
_10 = (_3.0, _3.1);
_2 = (*_7) >> _8;
_2 = !_9;
(*_7) = 7990680858026282188_i64 as u8;
_10.0 = 9748760271626838617_usize as u8;
_18.2 = _13 as usize;
_5 = _4;
_10.1 = _11.1;
_11.1 = 301654366158094199492120001367017447000_u128 as u32;
_16 = '\u{3c0a8}';
_15 = RET ^ RET;
_10.1 = _11.1 | _11.1;
_10.1 = _11.1 >> _2;
_3 = (_1, _10.1);
_12 = [_18.2,_18.2];
Goto(bb2)
}
bb12 = {
_10.1 = !_21.2;
Goto(bb7)
}
bb13 = {
_13 = _14 * _14;
_2 = (*_7);
_21 = ((-10764_i16), 8206989907852173695_i64, _11.1);
_19 = [28_i8,48_i8,(-7_i8)];
Call(_24 = fn18(_9, _21.1, _21), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_10 = _3;
_11 = _3;
_19 = [0_i8,90_i8,(-92_i8)];
(*_7) = _9;
_9 = !_2;
_4 = _5;
_5 = (*_7) >= _1;
Goto(bb3)
}
bb15 = {
_10.1 = !_11.1;
_9 = _3.0 & _8;
RET = _15 + _15;
_18.1 = _13 as f32;
_13 = _14;
_20 = [RET,RET];
RET = -_15;
_20 = [_15,RET];
_11 = (_3.0, _3.1);
_17 = 5441984438298047837_u64;
Goto(bb4)
}
bb16 = {
_21.1 = 4861414525741329410_i64;
_34 = (_10.0, _10.1);
_10.1 = _16 as u32;
_19 = [(-92_i8),(-122_i8),36_i8];
_32 = Adt46 { fld0: _10 };
_9 = _3.0;
_18.2 = 1_usize;
_12 = [_18.2,_18.2];
_13 = _18.1 as f64;
_11 = ((*_7), _21.2);
_10.0 = !_8;
_13 = _21.2 as f64;
_7 = core::ptr::addr_of_mut!(_34.0);
_18.0 = _21.0 << _1;
_15 = _22;
_18.1 = 43896090740157082809136870767699428793_i128 as f32;
_16 = _24;
_21.2 = _11.1 << _1;
_11.0 = _8;
(*_7) = _2 | _32.fld0.0;
_21.0 = _18.0 - _18.0;
_23 = (-1997386610_i32) as f64;
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(17_usize, 5_usize, Move(_5), 22_usize, Move(_22), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(17_usize, 24_usize, Move(_24), 15_usize, Move(_15), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(17_usize, 9_usize, Move(_9), 8_usize, Move(_8), 10_usize, Move(_10), 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u8,mut _2: i64,mut _3: (i16, i64, u32)) -> char {
mir! {
type RET = char;
let _4: char;
let _5: Adt57;
let _6: u16;
let _7: i64;
let _8: bool;
let _9: f64;
let _10: *const bool;
let _11: Adt58;
let _12: (i16, i64, u32);
let _13: f32;
let _14: Adt61;
let _15: char;
let _16: Adt58;
let _17: f64;
let _18: f32;
let _19: (u8, u32);
let _20: [u8; 6];
let _21: Adt46;
let _22: *mut u16;
let _23: ();
let _24: ();
{
RET = '\u{f6305}';
_3.2 = 4016122305_u32 ^ 4177009086_u32;
_3 = (13027_i16, _2, 1441940029_u32);
RET = '\u{61cc1}';
_4 = RET;
_3.1 = 4_usize as i64;
RET = _4;
_5.fld0 = [_1,_1,_1,_1,_1,_1];
RET = _4;
_5.fld4 = (245036436966232700687570265210512920074_u128,);
_2 = !_3.1;
RET = _4;
_4 = RET;
_5.fld4.0 = 322300808476926690656821793670733225587_u128 * 195184952209867907829362885323793878645_u128;
_3.1 = _2 ^ _2;
RET = _4;
_5.fld4.0 = 245752450617516461035991617907819105739_u128 | 274140701888138253875928730374518603107_u128;
_3.1 = _2;
_5.fld4 = (73638019740466549650333103116599496904_u128,);
RET = _4;
_5.fld4 = (25164625650968444906241034413414930896_u128,);
RET = _4;
_6 = 54556_u16;
_5.fld0 = [_1,_1,_1,_1,_1,_1];
match _3.2 {
0 => bb1,
1 => bb2,
1441940029 => bb4,
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
_9 = _1 as f64;
_5.fld4.0 = 194818057255656548067074462691375103798_u128;
_5.fld3 = (-95_i8) * (-67_i8);
_1 = (-87341260154260613231339158598190255739_i128) as u8;
_3.0 = 18478_i16;
_9 = _5.fld4.0 as f64;
_10 = core::ptr::addr_of!(_8);
_12 = (_3.0, _3.1, _3.2);
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld6 = [_5.fld3,_5.fld3,_5.fld3];
(*_10) = _12.2 == _12.2;
_8 = true;
_10 = core::ptr::addr_of!(_8);
_12.2 = _3.2;
_5.fld4.0 = 40438601204882704469369894742989464257_u128;
(*_10) = false;
_5.fld4 = (3538453953291820629987470484481759904_u128,);
_12 = (_3.0, _2, _3.2);
_15 = RET;
_10 = core::ptr::addr_of!((*_10));
match _3.2 {
0 => bb5,
1441940029 => bb7,
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
_2 = _12.1;
_3.1 = -_2;
(*_10) = false;
_12 = _3;
_13 = _3.2 as f32;
_5.fld4.0 = !128612838020258865828036826992001291894_u128;
_13 = _5.fld4.0 as f32;
_7 = _3.1;
(*_10) = true;
_6 = 26475_u16 + 39059_u16;
_15 = _4;
_5.fld4 = (83188563422903487869994051904705837061_u128,);
_5.fld3 = 21_i8 & 21_i8;
_5.fld4.0 = 202622623974565655077657914355042954193_u128 * 336616264849232449324351706675350161702_u128;
_12 = _3;
RET = _15;
_12 = _3;
_7 = _3.1 >> _3.2;
_5.fld3 = (-9_i8) >> _7;
_9 = 1_usize as f64;
_4 = RET;
RET = _4;
_6 = !21774_u16;
match _12.2 {
0 => bb8,
1 => bb9,
2 => bb10,
1441940029 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_9 = _1 as f64;
_5.fld4.0 = 194818057255656548067074462691375103798_u128;
_5.fld3 = (-95_i8) * (-67_i8);
_1 = (-87341260154260613231339158598190255739_i128) as u8;
_3.0 = 18478_i16;
_9 = _5.fld4.0 as f64;
_10 = core::ptr::addr_of!(_8);
_12 = (_3.0, _3.1, _3.2);
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld6 = [_5.fld3,_5.fld3,_5.fld3];
(*_10) = _12.2 == _12.2;
_8 = true;
_10 = core::ptr::addr_of!(_8);
_12.2 = _3.2;
_5.fld4.0 = 40438601204882704469369894742989464257_u128;
(*_10) = false;
_5.fld4 = (3538453953291820629987470484481759904_u128,);
_12 = (_3.0, _2, _3.2);
_15 = RET;
_10 = core::ptr::addr_of!((*_10));
match _3.2 {
0 => bb5,
1441940029 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
RET = _4;
_10 = core::ptr::addr_of!(_8);
_5.fld6 = [_5.fld3,_5.fld3,_5.fld3];
RET = _15;
_12.1 = _7;
_17 = _7 as f64;
_18 = _13;
_3.2 = 11247573796253586518_usize as u32;
_1 = (-150878197503148212214947957361312894527_i128) as u8;
_3.2 = (-9223372036854775808_isize) as u32;
_6 = 28204_u16 - 48254_u16;
_19 = (_1, _12.2);
_7 = _5.fld3 as i64;
match _19.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
1441940029 => bb21,
_ => bb20
}
}
bb13 = {
Return()
}
bb14 = {
_9 = _1 as f64;
_5.fld4.0 = 194818057255656548067074462691375103798_u128;
_5.fld3 = (-95_i8) * (-67_i8);
_1 = (-87341260154260613231339158598190255739_i128) as u8;
_3.0 = 18478_i16;
_9 = _5.fld4.0 as f64;
_10 = core::ptr::addr_of!(_8);
_12 = (_3.0, _3.1, _3.2);
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld6 = [_5.fld3,_5.fld3,_5.fld3];
(*_10) = _12.2 == _12.2;
_8 = true;
_10 = core::ptr::addr_of!(_8);
_12.2 = _3.2;
_5.fld4.0 = 40438601204882704469369894742989464257_u128;
(*_10) = false;
_5.fld4 = (3538453953291820629987470484481759904_u128,);
_12 = (_3.0, _2, _3.2);
_15 = RET;
_10 = core::ptr::addr_of!((*_10));
match _3.2 {
0 => bb5,
1441940029 => bb7,
_ => bb6
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
Return()
}
bb20 = {
_9 = _1 as f64;
_5.fld4.0 = 194818057255656548067074462691375103798_u128;
_5.fld3 = (-95_i8) * (-67_i8);
_1 = (-87341260154260613231339158598190255739_i128) as u8;
_3.0 = 18478_i16;
_9 = _5.fld4.0 as f64;
_10 = core::ptr::addr_of!(_8);
_12 = (_3.0, _3.1, _3.2);
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.fld6 = [_5.fld3,_5.fld3,_5.fld3];
(*_10) = _12.2 == _12.2;
_8 = true;
_10 = core::ptr::addr_of!(_8);
_12.2 = _3.2;
_5.fld4.0 = 40438601204882704469369894742989464257_u128;
(*_10) = false;
_5.fld4 = (3538453953291820629987470484481759904_u128,);
_12 = (_3.0, _2, _3.2);
_15 = RET;
_10 = core::ptr::addr_of!((*_10));
match _3.2 {
0 => bb5,
1441940029 => bb7,
_ => bb6
}
}
bb21 = {
(*_10) = false;
_13 = _1 as f32;
_5.fld0 = [_19.0,_1,_19.0,_1,_1,_19.0];
_3.2 = _12.2;
_12 = _3;
_21.fld0.0 = _5.fld3 as u8;
_7 = !_3.1;
_18 = _13 - _13;
_21 = Adt46 { fld0: _19 };
RET = _15;
_5.fld4 = (153931702585113332615344044825630275310_u128,);
Goto(bb22)
}
bb22 = {
Call(_23 = dump_var(18_usize, 3_usize, Move(_3), 2_usize, Move(_2), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_23 = dump_var(18_usize, 4_usize, Move(_4), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{8a74a}'), std::hint::black_box(97_isize), std::hint::black_box(4250014886_u32));
                
            }
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (u8, u32),
}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u32,
fld1: i16,

},
Variant1{
fld0: (i16, i64, u32),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut f32,
fld1: [u16; 8],
fld2: isize,
fld3: i128,
fld4: u128,
fld5: f64,

},
Variant1{
fld0: *mut [u128; 2],
fld1: char,
fld2: *mut u8,
fld3: ((u16, u8, i32, f64),),
fld4: u32,
fld5: *const [i8; 4],
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (*mut u8,),
fld1: (u128,),
fld2: f32,
fld3: i8,
fld4: Adt46,
fld5: (u8, u32),
fld6: [u128; 2],
fld7: usize,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [isize; 2],
fld1: u32,
fld2: *mut [u128; 2],
fld3: (u8, u32),

},
Variant1{
fld0: [u8; 6],
fld1: *mut [u128; 2],
fld2: u32,
fld3: (u8, u32),
fld4: *const [u8; 7],

},
Variant2{
fld0: ([i8; 3],),
fld1: *mut u8,
fld2: [u8; 7],
fld3: (i16, f32, usize),

},
Variant3{
fld0: isize,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt50,
fld1: char,
fld2: [isize; 2],
fld3: [u128; 2],
fld4: i16,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: *mut u16,
fld3: *mut f32,
fld4: Adt47,
fld5: [usize; 2],
fld6: ([i8; 3],),
fld7: *const bool,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [isize; 2],
fld1: (i16, i64, u32),
fld2: u128,
fld3: [u8; 6],
fld4: Adt51,
fld5: i32,
fld6: [usize; 2],

},
Variant1{
fld0: ((u16, u8, i32, f64),),
fld1: (i16, i64, u32),
fld2: Adt47,
fld3: i8,
fld4: f32,
fld5: [i8; 3],
fld6: Adt48,
fld7: Adt49,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u128; 2],

},
Variant1{
fld0: [u8; 6],
fld1: char,
fld2: Adt52,
fld3: *const [u8; 7],
fld4: u128,
fld5: ((i16, i64, u32), i16),
fld6: (i16, i64, u32),
fld7: [usize; 2],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u128; 2],
fld1: *mut u8,
fld2: isize,

},
Variant1{
fld0: usize,
fld1: *const [i8; 4],

},
Variant2{
fld0: *mut [u128; 2],

},
Variant3{
fld0: Adt52,
fld1: [u16; 8],
fld2: (*mut u8,),
fld3: i8,
fld4: *mut u128,
fld5: f32,
fld6: ((i16, i64, u32), i16),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: bool,
fld1: *mut u32,
fld2: f32,
fld3: *mut f32,
fld4: ([i8; 3],),
fld5: Adt48,
fld6: *const bool,
fld7: Adt50,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: f64,
fld1: *mut u128,
fld2: i128,

},
Variant1{
fld0: [i8; 4],
fld1: i8,
fld2: [i8; 3],

},
Variant2{
fld0: isize,
fld1: *mut u8,

},
Variant3{
fld0: *const [i8; 4],
fld1: char,
fld2: Adt51,
fld3: usize,
fld4: u32,
fld5: [u8; 6],
fld6: [i8; 4],
fld7: *mut u32,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: [u8; 6],
fld1: [u16; 8],
fld2: *const [i8; 4],
fld3: i8,
fld4: (u128,),
fld5: Adt48,
fld6: [i8; 3],
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: (i16, i64, u32),

},
Variant1{
fld0: [u8; 7],
fld1: Adt57,
fld2: *mut f32,
fld3: Adt55,
fld4: i16,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut u128,
fld1: *const [u8; 7],
fld2: [u16; 8],
fld3: Adt53,
fld4: Adt55,
fld5: [isize; 2],
fld6: [i8; 3],
fld7: u64,

},
Variant1{
fld0: *const bool,
fld1: char,
fld2: *mut u16,
fld3: [isize; 2],
fld4: (u8, u32),
fld5: u8,
fld6: f64,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: u8,
fld1: Adt50,
fld2: isize,
fld3: *mut u32,

},
Variant1{
fld0: Adt51,
fld1: *mut [u128; 2],
fld2: Adt54,

},
Variant2{
fld0: u64,
fld1: *mut f32,
fld2: Adt49,
fld3: Adt47,
fld4: i16,
fld5: u128,
fld6: *const [u8; 7],
fld7: (u8, u32),

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: u64,
fld1: Adt56,
fld2: Adt55,
fld3: u8,
fld4: i128,
fld5: Adt53,

},
Variant1{
fld0: Adt60,
fld1: [isize; 2],
fld2: (u16, u8, i32, f64),
fld3: *mut u8,
fld4: u8,
fld5: Adt57,
fld6: [usize; 6],

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: Adt49,
fld1: Adt59,
fld2: [i8; 4],
fld3: u16,
fld4: u8,
fld5: *mut f32,
fld6: f64,

},
Variant1{
fld0: (i16, f32, usize),
fld1: *const bool,
fld2: [usize; 2],
fld3: [i128; 4],
fld4: i16,
fld5: (u128,),
fld6: [isize; 2],
fld7: *const [u8; 7],

},
Variant2{
fld0: Adt58,
fld1: i64,
fld2: Adt51,
fld3: Adt57,
fld4: f32,

}}

