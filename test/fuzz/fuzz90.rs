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
pub fn fn0() -> ((i32,), usize) {
mir! {
type RET = ((i32,), usize);
let _1: &'static char;
let _2: ((*mut &'static i128,), *const isize);
let _3: bool;
let _4: bool;
let _5: &'static &'static &'static i128;
let _6: isize;
let _7: f32;
let _8: isize;
let _9: *mut ((i32,), usize);
let _10: bool;
let _11: *mut ((i32,), usize);
let _12: isize;
let _13: usize;
let _14: isize;
let _15: isize;
let _16: ((&'static i128,), f64, u16);
let _17: u32;
let _18: ();
let _19: ();
{
RET.0 = (943431443_i32,);
RET.1 = !1_usize;
RET.1 = 4_usize * 4345561240162001510_usize;
RET.0.0 = 9728555619179618380_u64 as i32;
_4 = !true;
RET.1 = 1811169808792916087_usize + 0_usize;
_6 = (-9223372036854775808_isize);
RET.1 = 4199398153283989873_usize << RET.0.0;
_4 = !true;
RET.0 = ((-1935086992_i32),);
RET.0 = (311600281_i32,);
_2.1 = core::ptr::addr_of!(_6);
_3 = _4 | _4;
_6 = (-5874397556030828210_i64) as isize;
_3 = _4 | _4;
_8 = -_6;
RET.0.0 = !(-1762799785_i32);
_4 = !_3;
Call(_2 = fn1(RET.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = (1848613807_i32,);
RET.0 = (1716630253_i32,);
_8 = 4328358476695024789_i64 as isize;
_4 = _3;
_6 = _8 * _8;
RET.0 = (737830402_i32,);
_7 = 20979_i16 as f32;
RET.0 = ((-1739578634_i32),);
Goto(bb2)
}
bb2 = {
_3 = !_4;
_7 = (-92_i8) as f32;
match RET.0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463463374607430028632822 => bb6,
_ => bb5
}
}
bb3 = {
RET.0 = (1848613807_i32,);
RET.0 = (1716630253_i32,);
_8 = 4328358476695024789_i64 as isize;
_4 = _3;
_6 = _8 * _8;
RET.0 = (737830402_i32,);
_7 = 20979_i16 as f32;
RET.0 = ((-1739578634_i32),);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET.0.0 = !37482363_i32;
RET.1 = 246582165_u32 as usize;
RET.1 = !1_usize;
RET.0.0 = 1605424788_i32;
_6 = 174_u8 as isize;
RET.0.0 = !1737730958_i32;
RET.0.0 = (-351387470_i32);
_7 = (-20_i8) as f32;
match RET.0.0 {
340282366920938463463374607431416823986 => bb7,
_ => bb1
}
}
bb7 = {
RET.0 = (233186271_i32,);
RET.0 = (82455713_i32,);
_6 = -_8;
_7 = RET.0.0 as f32;
RET.1 = 3_usize;
_2.1 = core::ptr::addr_of!(_6);
_9 = core::ptr::addr_of_mut!(RET);
(*_9).0 = (1783540808_i32,);
(*_9).0.0 = -11434448_i32;
(*_9).0 = (1719956683_i32,);
RET.1 = 7077010074651200541_usize;
_9 = core::ptr::addr_of_mut!((*_9));
(*_9).0 = (1677085630_i32,);
RET.0 = ((-1355645365_i32),);
_9 = core::ptr::addr_of_mut!((*_9));
_9 = core::ptr::addr_of_mut!(RET);
_10 = _3 | _4;
_8 = _6 * _6;
(*_9).1 = 1_usize;
(*_9).0.0 = (-865997723_i32) >> (*_9).1;
RET.0.0 = (-1365637721_i32);
_3 = _10;
RET.0 = (140187573_i32,);
RET.0.0 = 789374061_i32 >> RET.1;
RET.0.0 = !128548680_i32;
_3 = !_4;
_10 = (*_9).1 == (*_9).1;
RET.1 = 7_usize;
RET.1 = 11336459793665938910_usize + 2473195497923390183_usize;
Goto(bb8)
}
bb8 = {
_4 = _10;
_6 = _8;
_3 = _10;
_10 = !_4;
RET.1 = (-4_i8) as usize;
RET.1 = 4507415631114675342_usize;
_3 = !_10;
(*_9).0.0 = (-1973115885_i32);
(*_9).0.0 = 37254_u16 as i32;
(*_9).0 = ((-1131322654_i32),);
(*_9).0 = ((-1546553040_i32),);
_3 = _4 & _4;
RET.0.0 = 1449377204_i32;
_8 = 18477_u16 as isize;
Goto(bb9)
}
bb9 = {
RET.1 = 655106192301655130_usize >> (*_9).0.0;
RET.0 = (11475030_i32,);
_6 = -_8;
(*_9).0.0 = (-1797803642_i32);
(*_9).0.0 = (-50_i8) as i32;
_6 = -_8;
_3 = (*_9).0.0 <= RET.0.0;
RET.0 = (621876346_i32,);
_9 = core::ptr::addr_of_mut!((*_9));
_11 = Move(_9);
_10 = !_3;
_3 = !_10;
_6 = -_8;
_3 = !_10;
RET.0 = (146000667_i32,);
RET.1 = 15939339300262149507_usize + 0_usize;
_2.1 = core::ptr::addr_of!(_8);
_8 = _6;
_9 = core::ptr::addr_of_mut!(RET);
_9 = Move(_11);
_3 = _10;
_9 = core::ptr::addr_of_mut!(RET);
(*_9).0.0 = !102476421_i32;
RET.1 = !6916211294033812856_usize;
(*_9).0.0 = !1393383998_i32;
RET.0 = ((-1852651634_i32),);
_8 = 3311664395_u32 as isize;
match (*_9).0.0 {
0 => bb5,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607429915559822 => bb16,
_ => bb15
}
}
bb10 = {
RET.0 = (1848613807_i32,);
RET.0 = (1716630253_i32,);
_8 = 4328358476695024789_i64 as isize;
_4 = _3;
_6 = _8 * _8;
RET.0 = (737830402_i32,);
_7 = 20979_i16 as f32;
RET.0 = ((-1739578634_i32),);
Goto(bb2)
}
bb11 = {
RET.0 = (233186271_i32,);
RET.0 = (82455713_i32,);
_6 = -_8;
_7 = RET.0.0 as f32;
RET.1 = 3_usize;
_2.1 = core::ptr::addr_of!(_6);
_9 = core::ptr::addr_of_mut!(RET);
(*_9).0 = (1783540808_i32,);
(*_9).0.0 = -11434448_i32;
(*_9).0 = (1719956683_i32,);
RET.1 = 7077010074651200541_usize;
_9 = core::ptr::addr_of_mut!((*_9));
(*_9).0 = (1677085630_i32,);
RET.0 = ((-1355645365_i32),);
_9 = core::ptr::addr_of_mut!((*_9));
_9 = core::ptr::addr_of_mut!(RET);
_10 = _3 | _4;
_8 = _6 * _6;
(*_9).1 = 1_usize;
(*_9).0.0 = (-865997723_i32) >> (*_9).1;
RET.0.0 = (-1365637721_i32);
_3 = _10;
RET.0 = (140187573_i32,);
RET.0.0 = 789374061_i32 >> RET.1;
RET.0.0 = !128548680_i32;
_3 = !_4;
_10 = (*_9).1 == (*_9).1;
RET.1 = 7_usize;
RET.1 = 11336459793665938910_usize + 2473195497923390183_usize;
Goto(bb8)
}
bb12 = {
RET.0.0 = !37482363_i32;
RET.1 = 246582165_u32 as usize;
RET.1 = !1_usize;
RET.0.0 = 1605424788_i32;
_6 = 174_u8 as isize;
RET.0.0 = !1737730958_i32;
RET.0.0 = (-351387470_i32);
_7 = (-20_i8) as f32;
match RET.0.0 {
340282366920938463463374607431416823986 => bb7,
_ => bb1
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
RET.0 = (1848613807_i32,);
RET.0 = (1716630253_i32,);
_8 = 4328358476695024789_i64 as isize;
_4 = _3;
_6 = _8 * _8;
RET.0 = (737830402_i32,);
_7 = 20979_i16 as f32;
RET.0 = ((-1739578634_i32),);
Goto(bb2)
}
bb16 = {
_2.1 = core::ptr::addr_of!(_8);
(*_9).1 = 5595078032112284131_usize;
_3 = _4;
(*_9).1 = !8700540676720260080_usize;
(*_9).1 = (-23347_i16) as usize;
(*_9).0 = (963376898_i32,);
_9 = core::ptr::addr_of_mut!(RET);
_9 = core::ptr::addr_of_mut!((*_9));
RET.0 = (1444314973_i32,);
RET.0 = ((-1486167638_i32),);
_8 = !_6;
(*_9).1 = 15926494120798465521_usize + 4906257396759943132_usize;
_2.1 = core::ptr::addr_of!(_14);
_13 = (*_9).1;
_2.1 = core::ptr::addr_of!(_6);
_10 = !_3;
_8 = -_6;
RET.0 = ((-1832536764_i32),);
_10 = _4;
_3 = _7 < _7;
(*_9).0.0 = (-1078041864_i32);
(*_9).0.0 = 1873038069_i32 + 436572066_i32;
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(0_usize, 8_usize, Move(_8), 13_usize, Move(_13), 3_usize, Move(_3), 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: usize) -> ((*mut &'static i128,), *const isize) {
mir! {
type RET = ((*mut &'static i128,), *const isize);
let _2: isize;
let _3: &'static char;
let _4: (&'static i128,);
let _5: isize;
let _6: i32;
let _7: f32;
let _8: isize;
let _9: *const u64;
let _10: f32;
let _11: (&'static i128,);
let _12: char;
let _13: f64;
let _14: [isize; 7];
let _15: Adt35;
let _16: i64;
let _17: i128;
let _18: Adt67;
let _19: f64;
let _20: Adt67;
let _21: &'static bool;
let _22: i8;
let _23: i64;
let _24: isize;
let _25: char;
let _26: &'static char;
let _27: &'static (Adt35, char, usize, [isize; 4]);
let _28: u32;
let _29: f64;
let _30: [u128; 7];
let _31: u16;
let _32: bool;
let _33: usize;
let _34: ();
let _35: ();
{
_1 = 70_i8 as usize;
_1 = (-6401380524467242738_i64) as usize;
_1 = !2664873008747270843_usize;
_1 = 12767_u16 as usize;
_1 = 7_usize | 9665365021342193399_usize;
_1 = 4403159811834024167_usize << 3727_u16;
_1 = 2_usize ^ 10273572201816292378_usize;
_1 = 5_usize & 2_usize;
_1 = 4332893194931748814_usize & 4_usize;
Goto(bb1)
}
bb1 = {
_1 = !1531630345417337993_usize;
_1 = !15469809488187330649_usize;
Call(RET.0 = fn2(_1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = !7_usize;
_1 = 4582925985739182961_usize & 6_usize;
_1 = 17266679816979794882_usize;
_1 = 6_usize + 1_usize;
RET.1 = core::ptr::addr_of!(_2);
_1 = !7_usize;
_2 = 9223372036854775807_isize - (-17_isize);
RET.1 = core::ptr::addr_of!(_2);
RET.1 = core::ptr::addr_of!(_2);
Goto(bb3)
}
bb3 = {
RET.1 = core::ptr::addr_of!(_2);
_1 = 79375127576113040794989067963194858259_u128 as usize;
_1 = 6196452318933851796_usize;
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
RET.1 = core::ptr::addr_of!(_5);
_5 = _2 ^ _2;
RET.1 = core::ptr::addr_of!(_5);
_1 = !5_usize;
_2 = 12728_u16 as isize;
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
Goto(bb4)
}
bb4 = {
_6 = 181640951_i32 ^ 1964652986_i32;
_2 = -_5;
_7 = (-7011714213429895733_i64) as f32;
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
_2 = '\u{e25dd}' as isize;
RET.1 = core::ptr::addr_of!(_5);
_7 = _6 as f32;
Goto(bb5)
}
bb5 = {
_2 = 16019069727775536635_u64 as isize;
RET.1 = core::ptr::addr_of!(_8);
_1 = 2553508008804513543_usize >> _5;
_1 = 2087925076531339110_usize;
_8 = !_5;
_6 = 909903533_i32;
_3 = &_12;
_10 = _7 * _7;
_8 = _5 ^ _5;
match _1 {
2087925076531339110 => bb7,
_ => bb6
}
}
bb6 = {
_1 = !1531630345417337993_usize;
_1 = !15469809488187330649_usize;
Call(RET.0 = fn2(_1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_13 = 25034_u16 as f64;
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
_12 = '\u{b3886}';
RET.1 = core::ptr::addr_of!(_5);
_6 = (-1531030123_i32);
_8 = -_5;
_6 = 988442623_i32 >> _5;
_15.fld3 = [(-24631_i16),(-22208_i16),31819_i16];
_14 = [_5,_8,_5,_5,_8,_5,_5];
_16 = 4727283023165536887_i64 >> _6;
_14 = [_5,_2,_8,_5,_8,_5,_8];
_10 = -_7;
_14 = [_5,_2,_8,_8,_5,_5,_5];
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
_5 = _8 | _8;
_3 = &_12;
_17 = !(-42296616963643822955115518751708654181_i128);
_1 = !1700466050745467858_usize;
_15.fld3 = [23276_i16,8754_i16,(-32310_i16)];
_6 = !(-553248362_i32);
_17 = _2 as i128;
_15.fld1 = 37080_u16 as u8;
_1 = !8562405359939862686_usize;
_1 = 1129928440_u32 as usize;
_2 = !_5;
Goto(bb8)
}
bb8 = {
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
_2 = !_5;
_5 = _2;
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
_15.fld2 = _10 as isize;
_15.fld1 = 27_u8;
_12 = '\u{5dc43}';
_11.0 = &_17;
_7 = _13 as f32;
_15.fld3 = [(-1849_i16),(-16352_i16),3536_i16];
_4.0 = Move(_11.0);
_8 = _5 - _15.fld2;
_13 = 1977700170_u32 as f64;
_5 = _2 | _2;
_1 = 3549494250296594021_usize ^ 5629236795274630979_usize;
RET.1 = core::ptr::addr_of!(_15.fld2);
_24 = _2 << _15.fld2;
_11.0 = &_17;
_4.0 = Move(_11.0);
_25 = _12;
match _15.fld1 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
27 => bb9,
_ => bb7
}
}
bb9 = {
RET.1 = core::ptr::addr_of!(_15.fld2);
_15.fld0 = -_10;
RET.1 = core::ptr::addr_of!(_8);
_22 = !(-77_i8);
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
_22 = (-3_i8) << _8;
_19 = _13;
_12 = _25;
_10 = -_15.fld0;
_3 = &_12;
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
_15.fld0 = 163274780738340954887970304514363391949_u128 as f32;
Goto(bb10)
}
bb10 = {
RET.0.0 = core::ptr::addr_of_mut!(_4.0);
_1 = 14096479672660503617_usize;
_4.0 = &_17;
_6 = 478244667_i32;
_4.0 = &_17;
_17 = 29490204848954361902130640020585625960_i128;
_26 = Move(_3);
_15.fld3 = [(-27676_i16),(-4726_i16),(-21890_i16)];
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
_11.0 = &_17;
_4.0 = Move(_11.0);
_15.fld1 = !201_u8;
_23 = -_16;
_15.fld0 = _10;
Goto(bb11)
}
bb11 = {
_13 = -_19;
_24 = _15.fld1 as isize;
_14 = [_24,_2,_5,_2,_2,_5,_5];
_13 = _19 * _19;
_14 = [_2,_5,_8,_5,_15.fld2,_5,_5];
_15.fld3 = [6220_i16,(-31312_i16),(-3564_i16)];
RET.1 = core::ptr::addr_of!(_8);
_15.fld2 = !_8;
_15.fld3 = [(-31891_i16),(-6321_i16),26125_i16];
_7 = _10 + _10;
RET.0.0 = core::ptr::addr_of_mut!(_11.0);
RET.1 = core::ptr::addr_of!(_15.fld2);
_15.fld3 = [(-18344_i16),5134_i16,(-2688_i16)];
_2 = _13 as isize;
RET.1 = core::ptr::addr_of!(_5);
_19 = -_13;
RET.1 = core::ptr::addr_of!(_24);
_11.0 = &_17;
_28 = 1174662452_u32 * 440440011_u32;
_15.fld1 = 254_u8;
_2 = !_5;
_4 = Move(_11);
_8 = _5;
_14 = [_5,_5,_8,_5,_15.fld2,_2,_8];
_5 = !_2;
_24 = _8;
_4.0 = &_17;
Call(_31 = core::intrinsics::bswap(42310_u16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = _13;
_26 = &_12;
_1 = 1_usize & 7006841045457940890_usize;
_11 = (Move(_4.0),);
_3 = Move(_26);
RET.1 = core::ptr::addr_of!(_8);
_17 = _15.fld0 as i128;
_1 = !4099759185199707230_usize;
_32 = !false;
_30 = [213548283991674932766444065968343103302_u128,89330064612805024013467306163405205703_u128,300072220489284332846593161225869104740_u128,223274521086374136058899080486160312853_u128,160614748435425951815231825388441729292_u128,319655373152669877263882318452671865388_u128,195625757120892426227733204923513981716_u128];
_26 = &_12;
_28 = 3622863042_u32 * 1129172821_u32;
_3 = &_12;
_15.fld3 = [26828_i16,(-23934_i16),(-3001_i16)];
Goto(bb13)
}
bb13 = {
Call(_34 = dump_var(1_usize, 22_usize, Move(_22), 31_usize, Move(_31), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_34 = dump_var(1_usize, 8_usize, Move(_8), 5_usize, Move(_5), 25_usize, Move(_25), 28_usize, Move(_28)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize) -> (*mut &'static i128,) {
mir! {
type RET = (*mut &'static i128,);
let _11: bool;
let _12: &'static (*mut &'static (i32,), *mut &'static (i32,));
let _13: i128;
let _14: (*mut &'static i128,);
let _15: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _16: isize;
let _17: isize;
let _18: u128;
let _19: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _20: &'static i128;
let _21: [i16; 3];
let _22: char;
let _23: f32;
let _24: u16;
let _25: (&'static i128,);
let _26: [u64; 2];
let _27: [u32; 6];
let _28: f32;
let _29: isize;
let _30: [u16; 5];
let _31: isize;
let _32: ((i32,), usize);
let _33: char;
let _34: [u16; 5];
let _35: isize;
let _36: isize;
let _37: f64;
let _38: Adt63;
let _39: i32;
let _40: ();
let _41: ();
{
_7 = _10;
_4 = _10 * _10;
_4 = _3 | _3;
_8 = _2;
_8 = 11412172739979828077_u64 as usize;
_2 = (-500_i16) as usize;
_4 = _5 | _8;
_5 = !_8;
_11 = _1 > _4;
_3 = (-1565615715298962315_i64) as usize;
_6 = (-6355_i16) as usize;
_7 = _6 * _4;
_5 = 3739711430011651140_i64 as usize;
_7 = !_4;
_2 = _9 | _7;
_16 = 9223372036854775807_isize;
_2 = _4;
_4 = !_8;
_16 = !(-9223372036854775808_isize);
_6 = !_1;
_4 = _7;
Goto(bb1)
}
bb1 = {
_3 = _4;
_17 = _16;
_1 = _2;
_9 = _7;
_13 = (-11478764493152213166086111550843630899_i128) - (-165578639012202730273889149644472459018_i128);
Goto(bb2)
}
bb2 = {
RET.0 = core::ptr::addr_of_mut!(_20);
_15.2.0 = core::ptr::addr_of_mut!(_20);
_17 = -_16;
_17 = (-94_i8) as isize;
_10 = _4;
_1 = _16 as usize;
_14.0 = core::ptr::addr_of_mut!(_20);
_17 = 65_i8 as isize;
_20 = &_13;
_5 = !_8;
_15.2 = (Move(_14.0),);
RET.0 = Move(_15.2.0);
_6 = _3;
_15.2.0 = core::ptr::addr_of_mut!(_20);
_14 = Move(RET);
_15.2 = (Move(_14.0),);
_19.2.0 = Move(_15.2.0);
_20 = &(*_20);
_15.2 = (Move(_19.2.0),);
Goto(bb3)
}
bb3 = {
RET.0 = Move(_15.2.0);
_22 = '\u{a58ee}';
_23 = 17402666127658511778_u64 as f32;
_19.2 = (Move(RET.0),);
_16 = 1486151142_u32 as isize;
_13 = !169308624766850276578609193605163975345_i128;
_19.1 = &_24;
_3 = _1;
_14 = Move(_19.2);
RET = Move(_14);
_14 = (Move(RET.0),);
_20 = &_13;
_26 = [11200042280947118175_u64,4032288468621848952_u64];
Call(_8 = core::intrinsics::transmute(_17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24 = 33739_u16;
_7 = _10 & _8;
_6 = _7 & _3;
RET.0 = Move(_14.0);
_25.0 = &_13;
_8 = _10;
RET.0 = core::ptr::addr_of_mut!(_25.0);
_21 = [(-1789_i16),18207_i16,(-24710_i16)];
RET.0 = core::ptr::addr_of_mut!(_25.0);
_26 = [10905175168537746310_u64,8331934172854003933_u64];
_15.2.0 = core::ptr::addr_of_mut!(_20);
_19.2.0 = Move(RET.0);
_27 = [1237631001_u32,2342823875_u32,1886294030_u32,175180401_u32,1256629314_u32,2600881453_u32];
_14.0 = Move(_15.2.0);
_17 = -_16;
RET.0 = Move(_14.0);
_25 = (Move(_20),);
_18 = 255158269972133979098355432370714853431_u128 ^ 112437381785275450496008652188448226453_u128;
RET.0 = Move(_19.2.0);
_2 = !_10;
_7 = !_8;
_20 = &_13;
_5 = 2294070833261329493_i64 as usize;
_25 = (Move(_20),);
_19.2.0 = Move(RET.0);
_14 = Move(_19.2);
_15.2 = (Move(_14.0),);
_25.0 = &_13;
Goto(bb5)
}
bb5 = {
_22 = '\u{c7457}';
RET.0 = Move(_15.2.0);
_23 = (-1289946796_i32) as f32;
RET.0 = core::ptr::addr_of_mut!(_25.0);
_14 = Move(RET);
_9 = _8;
_1 = _23 as usize;
_29 = _2 as isize;
_4 = _7;
_10 = _9;
_15.2 = Move(_14);
_24 = !45826_u16;
_15.2.0 = core::ptr::addr_of_mut!(_20);
Call(_14.0 = fn3(Move(_15.2), Move(_25.0), _1, _10, _4, _16, _27), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = !_4;
_25.0 = &_13;
_25.0 = &_13;
_19.1 = &_24;
_15.2.0 = core::ptr::addr_of_mut!(_20);
_15.2.0 = core::ptr::addr_of_mut!(_25.0);
_16 = !_29;
_13 = (-144193685065870107884925681424078238365_i128);
_23 = _6 as f32;
_3 = _1;
_14.0 = Move(_15.2.0);
_25.0 = &_13;
_31 = _29;
_7 = (-33_i8) as usize;
_11 = true ^ true;
match _13 {
0 => bb1,
1 => bb3,
2 => bb7,
3 => bb8,
4 => bb9,
196088681855068355578448926007689973091 => bb11,
_ => bb10
}
}
bb7 = {
_22 = '\u{c7457}';
RET.0 = Move(_15.2.0);
_23 = (-1289946796_i32) as f32;
RET.0 = core::ptr::addr_of_mut!(_25.0);
_14 = Move(RET);
_9 = _8;
_1 = _23 as usize;
_29 = _2 as isize;
_4 = _7;
_10 = _9;
_15.2 = Move(_14);
_24 = !45826_u16;
_15.2.0 = core::ptr::addr_of_mut!(_20);
Call(_14.0 = fn3(Move(_15.2), Move(_25.0), _1, _10, _4, _16, _27), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_3 = _4;
_17 = _16;
_1 = _2;
_9 = _7;
_13 = (-11478764493152213166086111550843630899_i128) - (-165578639012202730273889149644472459018_i128);
Goto(bb2)
}
bb9 = {
RET.0 = Move(_15.2.0);
_22 = '\u{a58ee}';
_23 = 17402666127658511778_u64 as f32;
_19.2 = (Move(RET.0),);
_16 = 1486151142_u32 as isize;
_13 = !169308624766850276578609193605163975345_i128;
_19.1 = &_24;
_3 = _1;
_14 = Move(_19.2);
RET = Move(_14);
_14 = (Move(RET.0),);
_20 = &_13;
_26 = [11200042280947118175_u64,4032288468621848952_u64];
Call(_8 = core::intrinsics::transmute(_17), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
RET.0 = core::ptr::addr_of_mut!(_20);
_15.2.0 = core::ptr::addr_of_mut!(_20);
_17 = -_16;
_17 = (-94_i8) as isize;
_10 = _4;
_1 = _16 as usize;
_14.0 = core::ptr::addr_of_mut!(_20);
_17 = 65_i8 as isize;
_20 = &_13;
_5 = !_8;
_15.2 = (Move(_14.0),);
RET.0 = Move(_15.2.0);
_6 = _3;
_15.2.0 = core::ptr::addr_of_mut!(_20);
_14 = Move(RET);
_15.2 = (Move(_14.0),);
_19.2.0 = Move(_15.2.0);
_20 = &(*_20);
_15.2 = (Move(_19.2.0),);
Goto(bb3)
}
bb11 = {
_31 = 13183_i16 as isize;
_26 = [10251674407693058121_u64,13547819540239400415_u64];
_6 = _2;
_30 = [_24,_24,_24,_24,_24];
_1 = _10;
_2 = _8;
RET.0 = Move(_14.0);
_15.2.0 = Move(RET.0);
_33 = _22;
_34 = [_24,_24,_24,_24,_24];
_2 = !_9;
_8 = _9 & _9;
RET.0 = Move(_15.2.0);
_36 = _16 + _29;
_10 = _13 as usize;
_17 = _16;
_14.0 = Move(RET.0);
_15.2 = Move(_14);
RET = Move(_15.2);
_31 = !_36;
_32.0 = (855045201_i32,);
_3 = _8 >> _13;
_19.2 = (Move(RET.0),);
RET = Move(_19.2);
match _32.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb13,
5 => bb14,
855045201 => bb16,
_ => bb15
}
}
bb12 = {
RET.0 = core::ptr::addr_of_mut!(_20);
_15.2.0 = core::ptr::addr_of_mut!(_20);
_17 = -_16;
_17 = (-94_i8) as isize;
_10 = _4;
_1 = _16 as usize;
_14.0 = core::ptr::addr_of_mut!(_20);
_17 = 65_i8 as isize;
_20 = &_13;
_5 = !_8;
_15.2 = (Move(_14.0),);
RET.0 = Move(_15.2.0);
_6 = _3;
_15.2.0 = core::ptr::addr_of_mut!(_20);
_14 = Move(RET);
_15.2 = (Move(_14.0),);
_19.2.0 = Move(_15.2.0);
_20 = &(*_20);
_15.2 = (Move(_19.2.0),);
Goto(bb3)
}
bb13 = {
RET.0 = Move(_15.2.0);
_22 = '\u{a58ee}';
_23 = 17402666127658511778_u64 as f32;
_19.2 = (Move(RET.0),);
_16 = 1486151142_u32 as isize;
_13 = !169308624766850276578609193605163975345_i128;
_19.1 = &_24;
_3 = _1;
_14 = Move(_19.2);
RET = Move(_14);
_14 = (Move(RET.0),);
_20 = &_13;
_26 = [11200042280947118175_u64,4032288468621848952_u64];
Call(_8 = core::intrinsics::transmute(_17), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_22 = '\u{c7457}';
RET.0 = Move(_15.2.0);
_23 = (-1289946796_i32) as f32;
RET.0 = core::ptr::addr_of_mut!(_25.0);
_14 = Move(RET);
_9 = _8;
_1 = _23 as usize;
_29 = _2 as isize;
_4 = _7;
_10 = _9;
_15.2 = Move(_14);
_24 = !45826_u16;
_15.2.0 = core::ptr::addr_of_mut!(_20);
Call(_14.0 = fn3(Move(_15.2), Move(_25.0), _1, _10, _4, _16, _27), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
RET.0 = Move(_15.2.0);
_22 = '\u{a58ee}';
_23 = 17402666127658511778_u64 as f32;
_19.2 = (Move(RET.0),);
_16 = 1486151142_u32 as isize;
_13 = !169308624766850276578609193605163975345_i128;
_19.1 = &_24;
_3 = _1;
_14 = Move(_19.2);
RET = Move(_14);
_14 = (Move(RET.0),);
_20 = &_13;
_26 = [11200042280947118175_u64,4032288468621848952_u64];
Call(_8 = core::intrinsics::transmute(_17), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
_22 = _33;
_15.1 = &_24;
_1 = _3 << _3;
_32.0 = (1469615081_i32,);
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(2_usize, 3_usize, Move(_3), 27_usize, Move(_27), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(2_usize, 16_usize, Move(_16), 1_usize, Move(_1), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(2_usize, 18_usize, Move(_18), 31_usize, Move(_31), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(2_usize, 26_usize, Move(_26), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: (*mut &'static i128,),mut _2: &'static i128,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: isize,mut _7: [u32; 6]) -> *mut &'static i128 {
mir! {
type RET = *mut &'static i128;
let _8: f32;
let _9: ((*mut &'static i128,), *const isize);
let _10: (&'static i128,);
let _11: [u32; 4];
let _12: char;
let _13: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _14: [u64; 2];
let _15: &'static [i128; 3];
let _16: isize;
let _17: &'static char;
let _18: [i16; 3];
let _19: *const u128;
let _20: &'static (Adt35, char, usize, [isize; 4]);
let _21: usize;
let _22: &'static char;
let _23: i8;
let _24: &'static (i32,);
let _25: i8;
let _26: *const (&'static i128,);
let _27: Adt49;
let _28: f32;
let _29: isize;
let _30: char;
let _31: i16;
let _32: isize;
let _33: &'static [i128; 3];
let _34: Adt49;
let _35: &'static f32;
let _36: i128;
let _37: ((f32,), *const u64, char, u64);
let _38: [u8; 1];
let _39: Adt49;
let _40: (*const u128, u64, char, *const u128);
let _41: char;
let _42: ();
let _43: ();
{
_5 = !_3;
_7 = [3252032188_u32,1558961101_u32,2465564365_u32,3227687275_u32,1977242691_u32,2850727822_u32];
_7 = [4203835871_u32,1229366119_u32,1691775985_u32,342693422_u32,2327178802_u32,325254293_u32];
RET = Move(_1.0);
_4 = _5 >> _6;
_6 = (-3797551589912298498354172959956532214_i128) as isize;
_5 = 65367011966291751930621969677393063504_u128 as usize;
_1.0 = Move(RET);
_6 = 66718909500256553973163154469301544500_u128 as isize;
RET = core::ptr::addr_of_mut!(_2);
_3 = _5 | _4;
_1 = (Move(RET),);
RET = Move(_1.0);
_1.0 = Move(RET);
_8 = (-8565_i16) as f32;
_9.1 = core::ptr::addr_of!(_6);
_9.0 = Move(_1);
_9.1 = core::ptr::addr_of!(_6);
_4 = _3;
_7 = [200951591_u32,3712763150_u32,4057994534_u32,3127119745_u32,3282375835_u32,2080254382_u32];
Goto(bb1)
}
bb1 = {
_1.0 = Move(_9.0.0);
RET = Move(_1.0);
Goto(bb2)
}
bb2 = {
_9.0 = (Move(RET),);
_3 = 1546440984_i32 as usize;
_1 = Move(_9.0);
_13.2 = Move(_1);
_1.0 = Move(_13.2.0);
RET = Move(_1.0);
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = (Move(RET),);
_9.0.0 = Move(_1.0);
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
_4 = (-108_i8) as usize;
_12 = '\u{271bc}';
RET = Move(_9.0.0);
_7 = [270597685_u32,486702422_u32,3912903797_u32,3110918242_u32,2027937640_u32,1696120960_u32];
_8 = 15209813731727021092_u64 as f32;
_13.2 = (Move(RET),);
_6 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_12 = '\u{acad6}';
_8 = 50492_u16 as f32;
_11 = [3558488350_u32,3763726323_u32,3848313_u32,482390386_u32];
_4 = _6 as usize;
_9.0.0 = Move(_13.2.0);
_9.1 = core::ptr::addr_of!(_6);
_1.0 = core::ptr::addr_of_mut!(_2);
_1.0 = Move(_9.0.0);
_6 = -9223372036854775807_isize;
_13.2.0 = Move(_1.0);
_14 = [706849413000271535_u64,16945072341647169014_u64];
_9.0 = (Move(_13.2.0),);
_16 = !_6;
RET = core::ptr::addr_of_mut!(_2);
_3 = _4 >> _5;
_1 = (Move(RET),);
_1.0 = Move(_9.0.0);
Call(_9.0 = fn4(Move(_1.0), _12, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = core::ptr::addr_of_mut!(_2);
_8 = 43_i8 as f32;
_17 = &_12;
_18 = [12596_i16,(-11672_i16),(-8386_i16)];
_18 = [4907_i16,(-2710_i16),23540_i16];
_18 = [(-15387_i16),(-26224_i16),(-6283_i16)];
_11 = [2275686412_u32,3960245336_u32,3830293787_u32,57933840_u32];
_1.0 = core::ptr::addr_of_mut!((*RET));
_12 = '\u{3cf8c}';
_4 = _12 as usize;
_11 = [3209789299_u32,2024007646_u32,3992549104_u32,2496490667_u32];
_7 = [2508391350_u32,920956474_u32,999134846_u32,3756891065_u32,1439666952_u32,3734155837_u32];
_17 = &_12;
_13.2 = Move(_1);
_1 = Move(_13.2);
_13.2.0 = core::ptr::addr_of_mut!((*RET));
_4 = _5;
_3 = _8 as usize;
_9.0.0 = core::ptr::addr_of_mut!((*RET));
RET = Move(_13.2.0);
_6 = !_16;
_14 = [670095803725827429_u64,7396485043475676380_u64];
_1.0 = core::ptr::addr_of_mut!(_2);
_16 = _6;
RET = Move(_1.0);
_13.2.0 = core::ptr::addr_of_mut!(_2);
Call(_19 = fn5(Move(_13.2.0), Move(_9), Move(RET), Move(_17), _3, _12, _5, _14, _16, (*_17)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = core::ptr::addr_of_mut!(_10.0);
RET = core::ptr::addr_of_mut!((*RET));
_9.1 = core::ptr::addr_of!(_16);
_9.1 = core::ptr::addr_of!(_16);
Goto(bb6)
}
bb6 = {
_22 = &_12;
_7 = [2821868211_u32,2326015388_u32,2871139223_u32,226334837_u32,1096543318_u32,2847224825_u32];
_17 = &(*_22);
_5 = _3 << _6;
_9.0.0 = core::ptr::addr_of_mut!((*RET));
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
RET = core::ptr::addr_of_mut!((*RET));
_9.0.0 = core::ptr::addr_of_mut!((*RET));
_4 = _5 - _5;
_8 = 3932040526_u32 as f32;
_14 = [7477491766217218699_u64,10572852545418359826_u64];
_16 = false as isize;
_16 = _6 - _6;
_18 = [(-24905_i16),(-16645_i16),498_i16];
_5 = _4 ^ _4;
_14 = [8948048221653728754_u64,8132725980940007031_u64];
_1.0 = core::ptr::addr_of_mut!(_2);
_5 = _3 >> _16;
_13.2.0 = Move(_1.0);
_7 = [3047685194_u32,3211069870_u32,847095932_u32,454725468_u32,1297149475_u32,1041553003_u32];
_26 = core::ptr::addr_of!(_10);
_28 = _8 + _8;
_16 = (-6153334265431229352_i64) as isize;
Goto(bb7)
}
bb7 = {
_1 = Move(_13.2);
RET = Move(_1.0);
_1 = (Move(RET),);
_23 = false as i8;
_1.0 = core::ptr::addr_of_mut!((*_26).0);
Goto(bb8)
}
bb8 = {
RET = core::ptr::addr_of_mut!((*_26).0);
_12 = '\u{45221}';
_25 = !_23;
_22 = &_12;
_7 = [1451921097_u32,661657194_u32,3609307178_u32,730284737_u32,820729492_u32,2809530003_u32];
_9.0.0 = core::ptr::addr_of_mut!((*_26).0);
_25 = _23 + _23;
_8 = -_28;
_25 = (-996036758_i32) as i8;
Goto(bb9)
}
bb9 = {
_21 = _4;
RET = core::ptr::addr_of_mut!((*RET));
_3 = 18143_u16 as usize;
_17 = Move(_22);
_3 = _5 + _4;
_3 = 41_u8 as usize;
_22 = &_12;
_22 = &_30;
Goto(bb10)
}
bb10 = {
_22 = &(*_22);
RET = core::ptr::addr_of_mut!((*_26).0);
_22 = &_12;
_21 = _4;
RET = core::ptr::addr_of_mut!((*RET));
_9.0.0 = core::ptr::addr_of_mut!((*_26).0);
Goto(bb11)
}
bb11 = {
_13.2.0 = core::ptr::addr_of_mut!(_2);
RET = core::ptr::addr_of_mut!((*RET));
_14 = [18150778324994612239_u64,10332229430865061683_u64];
RET = Move(_13.2.0);
_13.2 = (Move(RET),);
_9.0.0 = core::ptr::addr_of_mut!(_2);
Goto(bb12)
}
bb12 = {
_23 = _25;
_13.2.0 = Move(_9.0.0);
_22 = &(*_22);
_8 = -_28;
_9.0 = (Move(_13.2.0),);
RET = core::ptr::addr_of_mut!(_10.0);
_32 = _6 & _16;
_13.2.0 = core::ptr::addr_of_mut!((*_26).0);
_18 = [7460_i16,(-23923_i16),(-2974_i16)];
_12 = '\u{ec891}';
_14 = [1105663088283132061_u64,7895570600215078093_u64];
_14 = [12195121028520191211_u64,3252957195431234615_u64];
_16 = _6 + _6;
_32 = _6 >> _23;
_18 = [12989_i16,(-25639_i16),(-30407_i16)];
_1 = (Move(_9.0.0),);
_9.0.0 = core::ptr::addr_of_mut!((*_26).0);
_9.0 = (Move(_1.0),);
_14 = [9672655347127459949_u64,17248439116239626552_u64];
_5 = _4;
_29 = _16;
Goto(bb13)
}
bb13 = {
_1.0 = core::ptr::addr_of_mut!((*RET));
_35 = &_28;
_4 = 191_u8 as usize;
_17 = &_30;
_30 = _12;
_14 = [9854920241861674985_u64,8410823542647735523_u64];
_7 = [2207323184_u32,1416967385_u32,1227603058_u32,4115765426_u32,2318544882_u32,3442931267_u32];
_31 = 18349_i16;
_22 = &_30;
_4 = !_3;
(*RET) = &_36;
_16 = _29;
match _31 {
0 => bb9,
1 => bb14,
2 => bb15,
3 => bb16,
18349 => bb18,
_ => bb17
}
}
bb14 = {
_1 = (Move(RET),);
_9.0.0 = Move(_1.0);
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
_4 = (-108_i8) as usize;
_12 = '\u{271bc}';
RET = Move(_9.0.0);
_7 = [270597685_u32,486702422_u32,3912903797_u32,3110918242_u32,2027937640_u32,1696120960_u32];
_8 = 15209813731727021092_u64 as f32;
_13.2 = (Move(RET),);
_6 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_12 = '\u{acad6}';
_8 = 50492_u16 as f32;
_11 = [3558488350_u32,3763726323_u32,3848313_u32,482390386_u32];
_4 = _6 as usize;
_9.0.0 = Move(_13.2.0);
_9.1 = core::ptr::addr_of!(_6);
_1.0 = core::ptr::addr_of_mut!(_2);
_1.0 = Move(_9.0.0);
_6 = -9223372036854775807_isize;
_13.2.0 = Move(_1.0);
_14 = [706849413000271535_u64,16945072341647169014_u64];
_9.0 = (Move(_13.2.0),);
_16 = !_6;
RET = core::ptr::addr_of_mut!(_2);
_3 = _4 >> _5;
_1 = (Move(RET),);
_1.0 = Move(_9.0.0);
Call(_9.0 = fn4(Move(_1.0), _12, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
_9.0 = (Move(RET),);
_3 = 1546440984_i32 as usize;
_1 = Move(_9.0);
_13.2 = Move(_1);
_1.0 = Move(_13.2.0);
RET = Move(_1.0);
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_22 = &_12;
_7 = [2821868211_u32,2326015388_u32,2871139223_u32,226334837_u32,1096543318_u32,2847224825_u32];
_17 = &(*_22);
_5 = _3 << _6;
_9.0.0 = core::ptr::addr_of_mut!((*RET));
_13.2.0 = core::ptr::addr_of_mut!(_10.0);
RET = core::ptr::addr_of_mut!((*RET));
_9.0.0 = core::ptr::addr_of_mut!((*RET));
_4 = _5 - _5;
_8 = 3932040526_u32 as f32;
_14 = [7477491766217218699_u64,10572852545418359826_u64];
_16 = false as isize;
_16 = _6 - _6;
_18 = [(-24905_i16),(-16645_i16),498_i16];
_5 = _4 ^ _4;
_14 = [8948048221653728754_u64,8132725980940007031_u64];
_1.0 = core::ptr::addr_of_mut!(_2);
_5 = _3 >> _16;
_13.2.0 = Move(_1.0);
_7 = [3047685194_u32,3211069870_u32,847095932_u32,454725468_u32,1297149475_u32,1041553003_u32];
_26 = core::ptr::addr_of!(_10);
_28 = _8 + _8;
_16 = (-6153334265431229352_i64) as isize;
Goto(bb7)
}
bb17 = {
_1.0 = Move(_9.0.0);
RET = Move(_1.0);
Goto(bb2)
}
bb18 = {
(*RET) = &_36;
(*RET) = &_36;
_10.0 = &_36;
_17 = Move(_22);
_37.3 = !9977683664569243813_u64;
RET = core::ptr::addr_of_mut!(_2);
_10.0 = &_36;
_25 = _29 as i8;
_21 = 47440_u16 as usize;
_37.2 = _12;
_32 = !_6;
_37.1 = core::ptr::addr_of!(_37.3);
_28 = _8 + _8;
_6 = -_32;
(*RET) = &_36;
(*RET) = &(*_2);
_31 = _5 as i16;
(*RET) = &(*_2);
(*RET) = &(*_2);
_32 = 2563520008_u32 as isize;
RET = core::ptr::addr_of_mut!((*_26).0);
_40.1 = _37.3;
_35 = &_8;
(*RET) = &_36;
_3 = (-2058551516_i32) as usize;
Goto(bb19)
}
bb19 = {
Call(_42 = dump_var(3_usize, 7_usize, Move(_7), 32_usize, Move(_32), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_42 = dump_var(3_usize, 30_usize, Move(_30), 21_usize, Move(_21), 23_usize, Move(_23), 12_usize, Move(_12)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: *mut &'static i128,mut _2: char,mut _3: usize) -> (*mut &'static i128,) {
mir! {
type RET = (*mut &'static i128,);
let _4: f32;
let _5: *const (*mut &'static (i32,), *mut &'static (i32,));
let _6: f32;
let _7: f64;
let _8: isize;
let _9: char;
let _10: ([i8; 1], ((&'static i128,), f64, u16));
let _11: isize;
let _12: bool;
let _13: *const u128;
let _14: isize;
let _15: (bool, Adt52, [u8; 1], f64);
let _16: &'static char;
let _17: f32;
let _18: u32;
let _19: [u64; 1];
let _20: i32;
let _21: (Adt35, char, usize, [isize; 4]);
let _22: *const u128;
let _23: i8;
let _24: isize;
let _25: f32;
let _26: &'static &'static i128;
let _27: char;
let _28: u32;
let _29: char;
let _30: Adt48;
let _31: [u32; 4];
let _32: ();
let _33: ();
{
RET.0 = Move(_1);
_2 = '\u{2344}';
_1 = Move(RET.0);
_4 = 13502_u16 as f32;
_4 = 86_u8 as f32;
RET = (Move(_1),);
_4 = 2785539337787220573_u64 as f32;
_3 = 18424704002765250462_usize;
_2 = '\u{f7b03}';
_2 = '\u{a512e}';
_2 = '\u{eb9a6}';
_4 = 2332662815_u32 as f32;
_1 = Move(RET.0);
RET.0 = Move(_1);
_1 = Move(RET.0);
match _3 {
0 => bb1,
18424704002765250462 => bb3,
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
RET = (Move(_1),);
_1 = Move(RET.0);
RET = (Move(_1),);
_1 = Move(RET.0);
RET = (Move(_1),);
_1 = Move(RET.0);
_6 = _3 as f32;
_4 = _6 * _6;
_3 = 59_i8 as usize;
_8 = (-9223372036854775808_isize) * 92_isize;
RET = (Move(_1),);
Goto(bb4)
}
bb4 = {
_1 = Move(RET.0);
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_4 = _6;
RET = (Move(_1),);
_11 = _8 >> _3;
_9 = _2;
_10.1.2 = !29911_u16;
_12 = !false;
_7 = 1309546975_u32 as f64;
_10.1.2 = !12178_u16;
_10.0 = [51_i8];
_8 = _11;
_10.1.1 = _7 * _7;
_10.1.2 = !46760_u16;
_11 = _8 >> _3;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_10.0 = [21_i8];
_10.1.2 = !17875_u16;
_10.1.1 = 37076140_i32 as f64;
_15.1.fld1 = (-360992653_i32) + 683278591_i32;
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
Call(_15.2 = core::intrinsics::transmute(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12 = _4 >= _4;
_15.1.fld0 = _3;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_10.0 = [(-20_i8)];
_10.1.2 = !45139_u16;
_15.1.fld0 = _11 as usize;
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_14 = 269450515691821057801077434394008893847_u128 as isize;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_16 = &_2;
_4 = _6 * _6;
_8 = _11 * _11;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_12 = !true;
_15.1.fld1 = (-1078389997_i32);
Call(_15.3 = core::intrinsics::fmaf64(_10.1.1, _10.1.1, _10.1.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = _9;
_16 = &_2;
_20 = _15.1.fld1 >> _8;
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_19 = [12735942724896572561_u64];
_10.1.2 = 3308_u16;
match _15.1.fld1 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb9,
340282366920938463463374607430689821459 => bb11,
_ => bb10
}
}
bb7 = {
_12 = _4 >= _4;
_15.1.fld0 = _3;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_10.0 = [(-20_i8)];
_10.1.2 = !45139_u16;
_15.1.fld0 = _11 as usize;
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_14 = 269450515691821057801077434394008893847_u128 as isize;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_16 = &_2;
_4 = _6 * _6;
_8 = _11 * _11;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_12 = !true;
_15.1.fld1 = (-1078389997_i32);
Call(_15.3 = core::intrinsics::fmaf64(_10.1.1, _10.1.1, _10.1.1), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_1 = Move(RET.0);
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_4 = _6;
RET = (Move(_1),);
_11 = _8 >> _3;
_9 = _2;
_10.1.2 = !29911_u16;
_12 = !false;
_7 = 1309546975_u32 as f64;
_10.1.2 = !12178_u16;
_10.0 = [51_i8];
_8 = _11;
_10.1.1 = _7 * _7;
_10.1.2 = !46760_u16;
_11 = _8 >> _3;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_10.0 = [21_i8];
_10.1.2 = !17875_u16;
_10.1.1 = 37076140_i32 as f64;
_15.1.fld1 = (-360992653_i32) + 683278591_i32;
_15.1.fld2 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2,_10.1.2];
Call(_15.2 = core::intrinsics::transmute(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
RET = (Move(_1),);
_1 = Move(RET.0);
RET = (Move(_1),);
_1 = Move(RET.0);
RET = (Move(_1),);
_1 = Move(RET.0);
_6 = _3 as f32;
_4 = _6 * _6;
_3 = 59_i8 as usize;
_8 = (-9223372036854775808_isize) * 92_isize;
RET = (Move(_1),);
Goto(bb4)
}
bb10 = {
Return()
}
bb11 = {
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_4 = -_6;
_21.3 = [_8,_8,_8,_11];
_10.1.2 = 52920562069526059421121645689119882084_u128 as u16;
_4 = -_6;
_1 = core::ptr::addr_of_mut!(_10.1.0.0);
_16 = &_9;
Goto(bb12)
}
bb12 = {
_20 = _15.1.fld1 << _3;
_2 = (*_16);
_21.2 = _15.1.fld0;
_19 = [3112511416974663755_u64];
_4 = _6;
_21.0.fld3 = [(-26563_i16),27714_i16,(-1167_i16)];
_6 = -_4;
_21.0.fld3 = [(-6427_i16),(-27723_i16),(-26728_i16)];
_15.2 = [245_u8];
_7 = _10.1.1 - _10.1.1;
_10.1.1 = 4158741873872586738_u64 as f64;
_11 = _8;
match _15.1.fld1 {
0 => bb1,
1 => bb2,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607430689821459 => bb13,
_ => bb8
}
}
bb13 = {
_21.1 = (*_16);
_17 = -_4;
_15.0 = _12;
_21.1 = _9;
_20 = _15.1.fld1 * _15.1.fld1;
_16 = &(*_16);
_23 = 40_i8;
Goto(bb14)
}
bb14 = {
_3 = _15.1.fld0 ^ _15.1.fld0;
_7 = 28564894016477303525386498503532328879_u128 as f64;
_4 = _21.2 as f32;
RET.0 = core::ptr::addr_of_mut!(_10.1.0.0);
_7 = -_10.1.1;
_26 = &(*_1);
_12 = !_15.0;
_7 = -_10.1.1;
RET.0 = core::ptr::addr_of_mut!((*_1));
_10.1.1 = _7;
_9 = _21.1;
_29 = _21.1;
_21.0.fld2 = _11;
_15.3 = -_10.1.1;
_3 = (-12525882757044028503819674474046964194_i128) as usize;
_27 = _29;
_11 = -_8;
_26 = &(*_26);
_30.fld0 = 1830775296_u32 * 3917340679_u32;
_30.fld4.2 = _9;
_10.1.2 = !64052_u16;
_4 = _17 * _17;
_28 = _30.fld0 * _30.fld0;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(4_usize, 14_usize, Move(_14), 23_usize, Move(_23), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(4_usize, 12_usize, Move(_12), 29_usize, Move(_29), 33_usize, _33, 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut &'static i128,mut _2: ((*mut &'static i128,), *const isize),mut _3: *mut &'static i128,mut _4: &'static char,mut _5: usize,mut _6: char,mut _7: usize,mut _8: [u64; 2],mut _9: isize,mut _10: char) -> *const u128 {
mir! {
type RET = *const u128;
let _11: u128;
let _12: (char, u128, bool);
let _13: [isize; 4];
let _14: *const u64;
let _15: ();
let _16: ();
{
_2.1 = core::ptr::addr_of!(_9);
_2.0 = (Move(_1),);
_6 = _10;
RET = core::ptr::addr_of!(_11);
_4 = &_10;
_1 = Move(_2.0.0);
Call(_8 = fn6(Move(_3), Move(_4), Move(_1), Move(_2.1), _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (-99_isize);
(*RET) = !189068074363160697649733057840631821263_u128;
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_10 = _6;
RET = core::ptr::addr_of!((*RET));
(*RET) = 295094010377656544150180759796621429364_u128;
(*RET) = !26914774017434795417008690590714217085_u128;
_9 = (-122_isize);
(*RET) = 37965128646658347841020315801601007172_u128 - 311567334849476828779509711851607737453_u128;
_11 = 99_u8 as u128;
RET = core::ptr::addr_of!(_11);
(*RET) = _7 as u128;
(*RET) = !56162996233434305394405322667050972793_u128;
_5 = !_7;
_4 = &_6;
(*RET) = !174080167387839127611542792867531263048_u128;
Goto(bb2)
}
bb2 = {
RET = core::ptr::addr_of!(_11);
_5 = _7 ^ _7;
_11 = 252627871123533305942998438694600773609_u128;
_10 = _6;
match (*RET) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
252627871123533305942998438694600773609 => bb10,
_ => bb9
}
}
bb3 = {
_9 = (-99_isize);
(*RET) = !189068074363160697649733057840631821263_u128;
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_10 = _6;
RET = core::ptr::addr_of!((*RET));
(*RET) = 295094010377656544150180759796621429364_u128;
(*RET) = !26914774017434795417008690590714217085_u128;
_9 = (-122_isize);
(*RET) = 37965128646658347841020315801601007172_u128 - 311567334849476828779509711851607737453_u128;
_11 = 99_u8 as u128;
RET = core::ptr::addr_of!(_11);
(*RET) = _7 as u128;
(*RET) = !56162996233434305394405322667050972793_u128;
_5 = !_7;
_4 = &_6;
(*RET) = !174080167387839127611542792867531263048_u128;
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
_12.1 = (*RET);
_12 = (_6, _11, false);
_12.2 = !true;
_12.0 = (*_4);
_9 = (-1201915861_i32) as isize;
match _11 {
0 => bb8,
1 => bb5,
2 => bb3,
3 => bb11,
4 => bb12,
5 => bb13,
252627871123533305942998438694600773609 => bb15,
_ => bb14
}
}
bb11 = {
RET = core::ptr::addr_of!(_11);
_5 = _7 ^ _7;
_11 = 252627871123533305942998438694600773609_u128;
_10 = _6;
match (*RET) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
252627871123533305942998438694600773609 => bb10,
_ => bb9
}
}
bb12 = {
_9 = (-99_isize);
(*RET) = !189068074363160697649733057840631821263_u128;
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_10 = _6;
RET = core::ptr::addr_of!((*RET));
(*RET) = 295094010377656544150180759796621429364_u128;
(*RET) = !26914774017434795417008690590714217085_u128;
_9 = (-122_isize);
(*RET) = 37965128646658347841020315801601007172_u128 - 311567334849476828779509711851607737453_u128;
_11 = 99_u8 as u128;
RET = core::ptr::addr_of!(_11);
(*RET) = _7 as u128;
(*RET) = !56162996233434305394405322667050972793_u128;
_5 = !_7;
_4 = &_6;
(*RET) = !174080167387839127611542792867531263048_u128;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_13 = [_9,_9,_9,_9];
_12.2 = !true;
_12.2 = !true;
_2.1 = core::ptr::addr_of!(_9);
Goto(bb16)
}
bb16 = {
Call(_15 = dump_var(5_usize, 8_usize, Move(_8), 9_usize, Move(_9), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut &'static i128,mut _2: &'static char,mut _3: *mut &'static i128,mut _4: *const isize,mut _5: char) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _6: *const (&'static i128,);
let _7: [usize; 7];
let _8: &'static &'static &'static i128;
let _9: &'static [i128; 3];
let _10: [u128; 7];
let _11: &'static (Adt35, char, usize, [isize; 4]);
let _12: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _13: usize;
let _14: &'static (i32,);
let _15: isize;
let _16: bool;
let _17: i128;
let _18: [u32; 4];
let _19: ();
let _20: ();
{
_3 = Move(_1);
RET = [5068083910417753421_u64,12807196846310805105_u64];
_1 = Move(_3);
_2 = &_5;
_5 = '\u{a63}';
RET = [3881708311592597955_u64,9339648110072096410_u64];
_5 = '\u{63a28}';
_5 = '\u{7621f}';
RET = [6952821989879860312_u64,14794765801108320290_u64];
RET = [8457634310196847209_u64,16366516334438476157_u64];
RET = [638699875412412861_u64,7034767972958648554_u64];
_3 = Move(_1);
_2 = &_5;
RET = [17326579660340891066_u64,15135597219768122073_u64];
Call(_1 = fn7(Move(_3), Move(_2), RET, RET, (*_2), RET, Move(_4), (*_2), _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [16286769464522537598_u64,16760108357620549458_u64];
_2 = &_5;
RET = [10655644970238814194_u64,13436416023221612910_u64];
_5 = '\u{3e50c}';
_7 = [5_usize,14760525927902115247_usize,12881259166139847375_usize,10361623084376014187_usize,17252539572323619073_usize,6923237201243685390_usize,0_usize];
_2 = &_5;
_2 = &(*_2);
_2 = &_5;
_7 = [0_usize,6_usize,2416169136238099347_usize,4_usize,0_usize,18078657475544314563_usize,4_usize];
_5 = '\u{c1695}';
_10 = [88324248807783943238115198969385078328_u128,45661935141203962129883799296065821998_u128,30116371503936895859072467041131956663_u128,229177789368536894115669455153778939194_u128,94899612564360881648220472436405586533_u128,112253848011366615360209637450978990322_u128,223637554182443827518133179624867429399_u128];
_7 = [16623248816075567784_usize,11177011457354318157_usize,6302419265488965160_usize,2735568961139080415_usize,5808497679998149290_usize,5378536797429855890_usize,2_usize];
_2 = &_5;
_10 = [103342655936956804744593175699637803099_u128,281896257872990165069192532101806252539_u128,111173908010312601036563536183322760665_u128,106373770390297766504101144982178757621_u128,289722414105613387700328882786149131385_u128,30658027899899839016710188675181269149_u128,14526680782666198098061409769202064774_u128];
RET = [10130737061866302356_u64,17320511604252376663_u64];
_10 = [249099890757299430946749103280021116859_u128,252811911581522262746679943642948365757_u128,96191671070751884573217980276265621564_u128,266526780208237127799245423506550964068_u128,249686540004536797476555468623772853553_u128,46272352377510665486392959142646222042_u128,88951462938271542169061525671119947964_u128];
_13 = !14703459984146115616_usize;
RET = [8744272829040221994_u64,10046568644334417493_u64];
RET = [17779085569100030626_u64,16679731802947827142_u64];
RET = [8899356393154660553_u64,7261410489881993362_u64];
_13 = 3_usize;
_5 = '\u{b5971}';
RET = [13498333090970820183_u64,13077102181140136224_u64];
match _10[_13] {
0 => bb2,
1 => bb3,
2 => bb4,
266526780208237127799245423506550964068 => bb6,
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
RET = [14553545269108152862_u64,14523930591622767109_u64];
_7 = [_13,_13,_13,_13,_13,_13,_13];
RET = [10155319325746646470_u64,14661087846873499636_u64];
_7[_13] = !_13;
match _10[_13] {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
266526780208237127799245423506550964068 => bb12,
_ => bb11
}
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
RET = [16286769464522537598_u64,16760108357620549458_u64];
_2 = &_5;
RET = [10655644970238814194_u64,13436416023221612910_u64];
_5 = '\u{3e50c}';
_7 = [5_usize,14760525927902115247_usize,12881259166139847375_usize,10361623084376014187_usize,17252539572323619073_usize,6923237201243685390_usize,0_usize];
_2 = &_5;
_2 = &(*_2);
_2 = &_5;
_7 = [0_usize,6_usize,2416169136238099347_usize,4_usize,0_usize,18078657475544314563_usize,4_usize];
_5 = '\u{c1695}';
_10 = [88324248807783943238115198969385078328_u128,45661935141203962129883799296065821998_u128,30116371503936895859072467041131956663_u128,229177789368536894115669455153778939194_u128,94899612564360881648220472436405586533_u128,112253848011366615360209637450978990322_u128,223637554182443827518133179624867429399_u128];
_7 = [16623248816075567784_usize,11177011457354318157_usize,6302419265488965160_usize,2735568961139080415_usize,5808497679998149290_usize,5378536797429855890_usize,2_usize];
_2 = &_5;
_10 = [103342655936956804744593175699637803099_u128,281896257872990165069192532101806252539_u128,111173908010312601036563536183322760665_u128,106373770390297766504101144982178757621_u128,289722414105613387700328882786149131385_u128,30658027899899839016710188675181269149_u128,14526680782666198098061409769202064774_u128];
RET = [10130737061866302356_u64,17320511604252376663_u64];
_10 = [249099890757299430946749103280021116859_u128,252811911581522262746679943642948365757_u128,96191671070751884573217980276265621564_u128,266526780208237127799245423506550964068_u128,249686540004536797476555468623772853553_u128,46272352377510665486392959142646222042_u128,88951462938271542169061525671119947964_u128];
_13 = !14703459984146115616_usize;
RET = [8744272829040221994_u64,10046568644334417493_u64];
RET = [17779085569100030626_u64,16679731802947827142_u64];
RET = [8899356393154660553_u64,7261410489881993362_u64];
_13 = 3_usize;
_5 = '\u{b5971}';
RET = [13498333090970820183_u64,13077102181140136224_u64];
match _10[_13] {
0 => bb2,
1 => bb3,
2 => bb4,
266526780208237127799245423506550964068 => bb6,
_ => bb5
}
}
bb12 = {
_10 = [159063227034104055021805199601407512985_u128,268590803648084605966533825667941986569_u128,240845383571447222037451907034230735432_u128,271467151628308728202744366843007598961_u128,64648569071614551381050299411929312407_u128,1239812358976375846158445371873320179_u128,98489744210514041356886529704360116876_u128];
_4 = core::ptr::addr_of!(_15);
_15 = 84_i8 as isize;
RET = [481272379838320292_u64,14688725728891112709_u64];
_12.0 = core::ptr::addr_of!(_18[_13]);
_12.0 = core::ptr::addr_of!(_18[_13]);
(*_4) = (-103_isize);
_10[_13] = !265795008935487457257564327590871173402_u128;
_12.0 = core::ptr::addr_of!(_18[_13]);
_15 = (-9223372036854775808_isize) >> _13;
(*_4) = _5 as isize;
_7 = [_13,_13,_13,_13,_13,_13,_13];
_18 = [3926627944_u32,662401374_u32,169636011_u32,3754419736_u32];
_5 = '\u{c7b20}';
_5 = '\u{1018c}';
_10 = [176979497263094085470568536099536944625_u128,111631123861476616316602529846671410611_u128,81429785693047014721165666390248091150_u128,242905927431533974402935941085862629295_u128,323272056254970207741330924903640918914_u128,188033238820493303308128428051262491686_u128,18910039985872628920049966222701161472_u128];
_2 = &_5;
_10[_13] = 85963775600982526373067311834656985469_u128;
_10[_13] = !81252696040786493540040986715244036728_u128;
_17 = true as i128;
match _18[_13] {
0 => bb9,
3754419736 => bb14,
_ => bb13
}
}
bb13 = {
RET = [16286769464522537598_u64,16760108357620549458_u64];
_2 = &_5;
RET = [10655644970238814194_u64,13436416023221612910_u64];
_5 = '\u{3e50c}';
_7 = [5_usize,14760525927902115247_usize,12881259166139847375_usize,10361623084376014187_usize,17252539572323619073_usize,6923237201243685390_usize,0_usize];
_2 = &_5;
_2 = &(*_2);
_2 = &_5;
_7 = [0_usize,6_usize,2416169136238099347_usize,4_usize,0_usize,18078657475544314563_usize,4_usize];
_5 = '\u{c1695}';
_10 = [88324248807783943238115198969385078328_u128,45661935141203962129883799296065821998_u128,30116371503936895859072467041131956663_u128,229177789368536894115669455153778939194_u128,94899612564360881648220472436405586533_u128,112253848011366615360209637450978990322_u128,223637554182443827518133179624867429399_u128];
_7 = [16623248816075567784_usize,11177011457354318157_usize,6302419265488965160_usize,2735568961139080415_usize,5808497679998149290_usize,5378536797429855890_usize,2_usize];
_2 = &_5;
_10 = [103342655936956804744593175699637803099_u128,281896257872990165069192532101806252539_u128,111173908010312601036563536183322760665_u128,106373770390297766504101144982178757621_u128,289722414105613387700328882786149131385_u128,30658027899899839016710188675181269149_u128,14526680782666198098061409769202064774_u128];
RET = [10130737061866302356_u64,17320511604252376663_u64];
_10 = [249099890757299430946749103280021116859_u128,252811911581522262746679943642948365757_u128,96191671070751884573217980276265621564_u128,266526780208237127799245423506550964068_u128,249686540004536797476555468623772853553_u128,46272352377510665486392959142646222042_u128,88951462938271542169061525671119947964_u128];
_13 = !14703459984146115616_usize;
RET = [8744272829040221994_u64,10046568644334417493_u64];
RET = [17779085569100030626_u64,16679731802947827142_u64];
RET = [8899356393154660553_u64,7261410489881993362_u64];
_13 = 3_usize;
_5 = '\u{b5971}';
RET = [13498333090970820183_u64,13077102181140136224_u64];
match _10[_13] {
0 => bb2,
1 => bb3,
2 => bb4,
266526780208237127799245423506550964068 => bb6,
_ => bb5
}
}
bb14 = {
_10 = [295873914172487331566443311593329831149_u128,62162107891790404442990198425299475140_u128,219271152167860505259772731441426884263_u128,9676244773948520966828154509882000981_u128,118087279644449645117534306968859201180_u128,212493646960764029292745933648329462339_u128,142822218525446084591592365598174675948_u128];
(*_4) = (-1506080020984309798_i64) as isize;
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(6_usize, 18_usize, Move(_18), 17_usize, Move(_17), 15_usize, Move(_15), 20_usize, _20), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *mut &'static i128,mut _2: &'static char,mut _3: [u64; 2],mut _4: [u64; 2],mut _5: char,mut _6: [u64; 2],mut _7: *const isize,mut _8: char,mut _9: char) -> *mut &'static i128 {
mir! {
type RET = *mut &'static i128;
let _10: ((i32,), usize);
let _11: [u32; 6];
let _12: char;
let _13: u32;
let _14: &'static i128;
let _15: isize;
let _16: Adt35;
let _17: char;
let _18: isize;
let _19: char;
let _20: [usize; 7];
let _21: i32;
let _22: ((f32,), *const u64, char, u64);
let _23: [i16; 3];
let _24: u8;
let _25: usize;
let _26: i128;
let _27: &'static i128;
let _28: isize;
let _29: bool;
let _30: [u128; 7];
let _31: [i8; 1];
let _32: bool;
let _33: Adt63;
let _34: ();
let _35: ();
{
RET = Move(_1);
_2 = &_5;
_1 = Move(RET);
RET = Move(_1);
_1 = Move(RET);
RET = Move(_1);
_3 = [6349506026898551148_u64,17349085854384806282_u64];
_5 = _8;
_4 = [3691880200155413317_u64,5283219159714650035_u64];
_6 = [9115295257074454658_u64,18047841955316706029_u64];
_2 = &_5;
_3 = [5944051165559656919_u64,3212315363863880122_u64];
_9 = _8;
_5 = _8;
_1 = Move(RET);
_5 = _8;
_10.0 = ((-1162998494_i32),);
_12 = _9;
RET = Move(_1);
_12 = _8;
_10.0 = ((-1775223262_i32),);
_4 = [9183623092963756891_u64,7402324138692127054_u64];
_1 = Move(RET);
_11 = [2564156473_u32,2893752296_u32,3929594181_u32,781217485_u32,1324474396_u32,960463969_u32];
_10.0 = (9530139_i32,);
_5 = _8;
_2 = &_12;
Goto(bb1)
}
bb1 = {
_10.0.0 = -1719868699_i32;
_10.0 = (519227846_i32,);
_5 = _9;
_13 = false as u32;
_10.0 = ((-1589662152_i32),);
RET = Move(_1);
_10.0.0 = (-534657622_i32) - 1723692924_i32;
_12 = _5;
_10.1 = 2533225316183249157_usize >> _10.0.0;
_12 = _8;
_10.0 = ((-410208661_i32),);
_10.0.0 = 157964243131870167789046100615950094680_i128 as i32;
_11 = [_13,_13,_13,_13,_13,_13];
_10.1 = 7_usize;
_15 = !(-9223372036854775808_isize);
_6 = [3946685702702278275_u64,10790436138814347275_u64];
_6 = _3;
_1 = core::ptr::addr_of_mut!(_14);
_1 = Move(RET);
_16.fld1 = !186_u8;
RET = core::ptr::addr_of_mut!(_14);
_4 = _3;
_3 = [6029516487064314943_u64,1212183534668279977_u64];
match _10.1 {
0 => bb2,
1 => bb3,
7 => bb5,
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
_2 = &_17;
_6 = [5783597405940642481_u64,15533389581253319397_u64];
_11 = [_13,_13,_13,_13,_13,_13];
_1 = core::ptr::addr_of_mut!((*RET));
_10.1 = 11167389428906340767_usize;
_10.0.0 = (-996663046_i32);
_10.1 = 11642110234557410581_usize;
_9 = _5;
_10.1 = 2_usize << _10.0.0;
_10.1 = 1_usize;
_12 = _9;
_11 = [_13,_13,_13,_13,_13,_13];
_24 = _16.fld1;
_16.fld3 = [1314_i16,4552_i16,(-2702_i16)];
_12 = _8;
_16.fld0 = _10.1 as f32;
Call(_22 = fn8(_10.0, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_21 = _15 as i32;
_21 = !_10.0.0;
_11 = [_13,_13,_13,_13,_13,_13];
_14 = &_26;
_16.fld1 = _22.3 as u8;
_20 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_22.2 = _5;
_8 = _5;
_17 = _5;
_16.fld2 = _15 & _15;
(*RET) = &(*_14);
_8 = _12;
_18 = _16.fld2;
_22.0.0 = _16.fld0;
Goto(bb7)
}
bb7 = {
_1 = core::ptr::addr_of_mut!((*RET));
_12 = _5;
_16.fld0 = _22.0.0;
_7 = core::ptr::addr_of!(_15);
(*RET) = &_26;
_20 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_5 = _12;
_5 = _17;
(*RET) = &(*_14);
(*RET) = &(*_14);
_27 = &(*_14);
_24 = !_16.fld1;
_2 = &_12;
_9 = _12;
_22.1 = core::ptr::addr_of!(_22.3);
_22.3 = !1684189511169243627_u64;
_24 = _10.1 as u8;
_24 = _15 as u8;
RET = core::ptr::addr_of_mut!((*_1));
_16.fld0 = _22.0.0;
_20 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_16.fld3 = [12818_i16,(-29685_i16),25687_i16];
(*_7) = _18 ^ _16.fld2;
_27 = &(*_14);
match _10.0.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
340282366920938463463374607430771548410 => bb16,
_ => bb15
}
}
bb8 = {
_21 = _15 as i32;
_21 = !_10.0.0;
_11 = [_13,_13,_13,_13,_13,_13];
_14 = &_26;
_16.fld1 = _22.3 as u8;
_20 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_22.2 = _5;
_8 = _5;
_17 = _5;
_16.fld2 = _15 & _15;
(*RET) = &(*_14);
_8 = _12;
_18 = _16.fld2;
_22.0.0 = _16.fld0;
Goto(bb7)
}
bb9 = {
_2 = &_17;
_6 = [5783597405940642481_u64,15533389581253319397_u64];
_11 = [_13,_13,_13,_13,_13,_13];
_1 = core::ptr::addr_of_mut!((*RET));
_10.1 = 11167389428906340767_usize;
_10.0.0 = (-996663046_i32);
_10.1 = 11642110234557410581_usize;
_9 = _5;
_10.1 = 2_usize << _10.0.0;
_10.1 = 1_usize;
_12 = _9;
_11 = [_13,_13,_13,_13,_13,_13];
_24 = _16.fld1;
_16.fld3 = [1314_i16,4552_i16,(-2702_i16)];
_12 = _8;
_16.fld0 = _10.1 as f32;
Call(_22 = fn8(_10.0, _3), ReturnTo(bb6), UnwindUnreachable())
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
_10.0.0 = -1719868699_i32;
_10.0 = (519227846_i32,);
_5 = _9;
_13 = false as u32;
_10.0 = ((-1589662152_i32),);
RET = Move(_1);
_10.0.0 = (-534657622_i32) - 1723692924_i32;
_12 = _5;
_10.1 = 2533225316183249157_usize >> _10.0.0;
_12 = _8;
_10.0 = ((-410208661_i32),);
_10.0.0 = 157964243131870167789046100615950094680_i128 as i32;
_11 = [_13,_13,_13,_13,_13,_13];
_10.1 = 7_usize;
_15 = !(-9223372036854775808_isize);
_6 = [3946685702702278275_u64,10790436138814347275_u64];
_6 = _3;
_1 = core::ptr::addr_of_mut!(_14);
_1 = Move(RET);
_16.fld1 = !186_u8;
RET = core::ptr::addr_of_mut!(_14);
_4 = _3;
_3 = [6029516487064314943_u64,1212183534668279977_u64];
match _10.1 {
0 => bb2,
1 => bb3,
7 => bb5,
_ => bb4
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_16.fld3 = [19757_i16,6392_i16,17298_i16];
_22.0 = (_16.fld0,);
_10.0.0 = !_21;
_28 = _15;
_29 = true;
(*_7) = _28;
_19 = _17;
(*_1) = &(*_27);
_30 = [141021592407701176807001627977805519898_u128,232667215159082216764185032953160786977_u128,181154146260962788156831734499777296406_u128,117637334501304542013226597665278912295_u128,231330003692553642558714760948177739300_u128,55021453073639055168210072872362675028_u128,127068887546950100118075199459621692536_u128];
_22.2 = (*_2);
(*_7) = !_28;
_4 = [_22.3,_22.3];
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(7_usize, 6_usize, Move(_6), 20_usize, Move(_20), 28_usize, Move(_28), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(7_usize, 15_usize, Move(_15), 10_usize, Move(_10), 12_usize, Move(_12), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(7_usize, 9_usize, Move(_9), 4_usize, Move(_4), 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: (i32,),mut _2: [u64; 2]) -> ((f32,), *const u64, char, u64) {
mir! {
type RET = ((f32,), *const u64, char, u64);
let _3: (i32,);
let _4: isize;
let _5: f64;
let _6: (Adt35, char, usize, [isize; 4]);
let _7: *const (&'static i128,);
let _8: Adt67;
let _9: &'static (Adt35, char, usize, [isize; 4]);
let _10: bool;
let _11: isize;
let _12: *const (&'static i128,);
let _13: ((&'static i128,), f64, u16);
let _14: isize;
let _15: ();
let _16: ();
{
_2 = [15029172551687032492_u64,271165787468797355_u64];
RET.3 = 81_u8 as u64;
RET.0.0 = 8009_u16 as f32;
RET.3 = 8704818970951446115_u64;
RET.1 = core::ptr::addr_of!(RET.3);
_1.0 = (-9223372036854775808_isize) as i32;
RET.2 = '\u{85845}';
RET.2 = '\u{926ae}';
_3.0 = _1.0;
_1.0 = RET.3 as i32;
_1.0 = _3.0;
RET.0.0 = (-3_i8) as f32;
RET.1 = core::ptr::addr_of!(RET.3);
_2 = [RET.3,RET.3];
_2 = [RET.3,RET.3];
RET.1 = core::ptr::addr_of!(RET.3);
Goto(bb1)
}
bb1 = {
RET.0.0 = _3.0 as f32;
_3 = _1;
RET.2 = '\u{d910d}';
RET.3 = 7175761901507844393_u64 & 333347415732770908_u64;
RET.1 = core::ptr::addr_of!(RET.3);
_2 = [RET.3,RET.3];
RET.2 = '\u{10a1b2}';
_3.0 = !_1.0;
RET.2 = '\u{20656}';
RET.1 = core::ptr::addr_of!(RET.3);
RET.2 = '\u{35df7}';
_1 = (_3.0,);
Goto(bb2)
}
bb2 = {
RET.3 = 705344384471749274_u64;
_1.0 = RET.0.0 as i32;
RET.2 = '\u{4e829}';
_1 = (_3.0,);
_2 = [RET.3,RET.3];
_1 = _3;
_3 = (_1.0,);
_5 = 14422364586796923080_usize as f64;
_6.0.fld0 = (-9223372036854775808_isize) as f32;
RET.0 = (_6.0.fld0,);
RET.3 = 1248850369269849564_u64;
_6.2 = (-98_isize) as usize;
_6.0.fld2 = -(-86_isize);
_3 = (_1.0,);
_6.3 = [_6.0.fld2,_6.0.fld2,_6.0.fld2,_6.0.fld2];
_6.1 = RET.2;
RET.1 = core::ptr::addr_of!(RET.3);
_4 = RET.0.0 as isize;
_6.2 = 0_usize;
_6.0.fld2 = _4 + _4;
_6.0.fld1 = 254_u8;
_6.0.fld3 = [11718_i16,(-6527_i16),(-15540_i16)];
_2 = [RET.3,RET.3];
Call(RET.0 = fn9(_6.0, _6.0, _6.0.fld2, RET.2, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = &_6;
RET.0 = ((*_9).0.fld0,);
_6.0.fld2 = (*_9).0.fld1 as isize;
_3.0 = _5 as i32;
RET.2 = _6.1;
_4 = RET.2 as isize;
_6.3 = [_4,_6.0.fld2,_4,_6.0.fld2];
_2 = [RET.3,RET.3];
RET.0 = (_6.0.fld0,);
_1 = (_3.0,);
_11 = -_4;
RET.0 = (_6.0.fld0,);
RET.0 = ((*_9).0.fld0,);
_6.0.fld2 = -_11;
RET.2 = _6.1;
_6.0.fld3 = [(-4944_i16),(-31651_i16),14776_i16];
RET.0 = ((*_9).0.fld0,);
match _6.2 {
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
0 => bb12,
_ => bb11
}
}
bb4 = {
RET.3 = 705344384471749274_u64;
_1.0 = RET.0.0 as i32;
RET.2 = '\u{4e829}';
_1 = (_3.0,);
_2 = [RET.3,RET.3];
_1 = _3;
_3 = (_1.0,);
_5 = 14422364586796923080_usize as f64;
_6.0.fld0 = (-9223372036854775808_isize) as f32;
RET.0 = (_6.0.fld0,);
RET.3 = 1248850369269849564_u64;
_6.2 = (-98_isize) as usize;
_6.0.fld2 = -(-86_isize);
_3 = (_1.0,);
_6.3 = [_6.0.fld2,_6.0.fld2,_6.0.fld2,_6.0.fld2];
_6.1 = RET.2;
RET.1 = core::ptr::addr_of!(RET.3);
_4 = RET.0.0 as isize;
_6.2 = 0_usize;
_6.0.fld2 = _4 + _4;
_6.0.fld1 = 254_u8;
_6.0.fld3 = [11718_i16,(-6527_i16),(-15540_i16)];
_2 = [RET.3,RET.3];
Call(RET.0 = fn9(_6.0, _6.0, _6.0.fld2, RET.2, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET.0.0 = _3.0 as f32;
_3 = _1;
RET.2 = '\u{d910d}';
RET.3 = 7175761901507844393_u64 & 333347415732770908_u64;
RET.1 = core::ptr::addr_of!(RET.3);
_2 = [RET.3,RET.3];
RET.2 = '\u{10a1b2}';
_3.0 = !_1.0;
RET.2 = '\u{20656}';
RET.1 = core::ptr::addr_of!(RET.3);
RET.2 = '\u{35df7}';
_1 = (_3.0,);
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
_6.0.fld2 = (-1537_i16) as isize;
_2 = [RET.3,RET.3];
RET.2 = (*_9).1;
RET.0.0 = -(*_9).0.fld0;
_11 = _6.0.fld2 >> _3.0;
_9 = &_6;
_1.0 = !_3.0;
_2 = [RET.3,RET.3];
_6.1 = RET.2;
_5 = 2389776145_u32 as f64;
RET.0 = (_6.0.fld0,);
Goto(bb13)
}
bb13 = {
_6.2 = 3_usize + 0_usize;
_6.0.fld1 = _3.0 as u8;
_6.1 = RET.2;
_4 = (-26829_i16) as isize;
RET.0.0 = (*_9).0.fld0;
_2 = [RET.3,RET.3];
_10 = !true;
_6.0.fld2 = _5 as isize;
_6.0.fld3 = [24476_i16,16722_i16,(-6829_i16)];
_6.0.fld0 = _5 as f32;
_6.3 = [_11,_11,_4,_11];
_2 = [RET.3,RET.3];
_6.0.fld1 = 200_u8 | 85_u8;
_6.1 = RET.2;
_13.1 = _5;
_14 = (-57_i8) as isize;
RET.3 = 5283404495124798894_u64;
match RET.3 {
0 => bb9,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
5283404495124798894 => bb20,
_ => bb19
}
}
bb14 = {
_6.0.fld2 = (-1537_i16) as isize;
_2 = [RET.3,RET.3];
RET.2 = (*_9).1;
RET.0.0 = -(*_9).0.fld0;
_11 = _6.0.fld2 >> _3.0;
_9 = &_6;
_1.0 = !_3.0;
_2 = [RET.3,RET.3];
_6.1 = RET.2;
_5 = 2389776145_u32 as f64;
RET.0 = (_6.0.fld0,);
Goto(bb13)
}
bb15 = {
RET.3 = 705344384471749274_u64;
_1.0 = RET.0.0 as i32;
RET.2 = '\u{4e829}';
_1 = (_3.0,);
_2 = [RET.3,RET.3];
_1 = _3;
_3 = (_1.0,);
_5 = 14422364586796923080_usize as f64;
_6.0.fld0 = (-9223372036854775808_isize) as f32;
RET.0 = (_6.0.fld0,);
RET.3 = 1248850369269849564_u64;
_6.2 = (-98_isize) as usize;
_6.0.fld2 = -(-86_isize);
_3 = (_1.0,);
_6.3 = [_6.0.fld2,_6.0.fld2,_6.0.fld2,_6.0.fld2];
_6.1 = RET.2;
RET.1 = core::ptr::addr_of!(RET.3);
_4 = RET.0.0 as isize;
_6.2 = 0_usize;
_6.0.fld2 = _4 + _4;
_6.0.fld1 = 254_u8;
_6.0.fld3 = [11718_i16,(-6527_i16),(-15540_i16)];
_2 = [RET.3,RET.3];
Call(RET.0 = fn9(_6.0, _6.0, _6.0.fld2, RET.2, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET.0.0 = _3.0 as f32;
_3 = _1;
RET.2 = '\u{d910d}';
RET.3 = 7175761901507844393_u64 & 333347415732770908_u64;
RET.1 = core::ptr::addr_of!(RET.3);
_2 = [RET.3,RET.3];
RET.2 = '\u{10a1b2}';
_3.0 = !_1.0;
RET.2 = '\u{20656}';
RET.1 = core::ptr::addr_of!(RET.3);
RET.2 = '\u{35df7}';
_1 = (_3.0,);
Goto(bb2)
}
bb19 = {
Return()
}
bb20 = {
RET.3 = (-15269_i16) as u64;
_6.2 = 9508315885994740793_usize + 4_usize;
_12 = core::ptr::addr_of!(_13.0);
_6.0.fld1 = 70_u8;
_1.0 = !_3.0;
_3.0 = _1.0;
_9 = &_6;
_3 = (_1.0,);
_13.2 = 37160_u16 << (*_9).0.fld2;
RET.3 = 7504288996141668907_u64;
_5 = _13.1;
_9 = &(*_9);
RET.1 = core::ptr::addr_of!(RET.3);
RET.2 = (*_9).1;
_1 = (_3.0,);
_5 = _13.1 + _13.1;
_9 = &(*_9);
_2 = [RET.3,RET.3];
RET.0 = ((*_9).0.fld0,);
_11 = _14 & _6.0.fld2;
_12 = core::ptr::addr_of!((*_12));
_14 = _13.2 as isize;
RET.3 = 171779563064439847312600463131440593541_u128 as u64;
RET.2 = (*_9).1;
RET.0.0 = RET.3 as f32;
_6.0.fld1 = 252_u8 << (*_9).0.fld2;
RET.0.0 = (*_9).0.fld0;
RET.0.0 = -(*_9).0.fld0;
Goto(bb21)
}
bb21 = {
Call(_15 = dump_var(8_usize, 1_usize, Move(_1), 10_usize, Move(_10), 11_usize, Move(_11), 16_usize, _16), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: Adt35,mut _2: Adt35,mut _3: isize,mut _4: char,mut _5: (Adt35, char, usize, [isize; 4])) -> (f32,) {
mir! {
type RET = (f32,);
let _6: isize;
let _7: isize;
let _8: bool;
let _9: f64;
let _10: isize;
let _11: f32;
let _12: (f32,);
let _13: isize;
let _14: i64;
let _15: ((*mut &'static i128,), *const isize);
let _16: usize;
let _17: f64;
let _18: i8;
let _19: isize;
let _20: isize;
let _21: &'static (i32,);
let _22: f32;
let _23: (f32,);
let _24: (*mut &'static i128,);
let _25: isize;
let _26: *const u16;
let _27: isize;
let _28: u8;
let _29: ();
let _30: ();
{
_5.0.fld1 = !_2.fld1;
_2.fld0 = _1.fld0;
_5.0.fld3 = [28659_i16,6296_i16,(-11226_i16)];
_5.0.fld3 = [1804_i16,2742_i16,5740_i16];
match _2.fld1 {
0 => bb1,
1 => bb2,
2 => bb3,
254 => bb5,
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
_2.fld2 = _3 ^ _3;
_6 = (-160819621367688764652630744011448188529_i128) as isize;
_5.1 = _4;
_3 = _2.fld2 * _2.fld2;
RET = (_1.fld0,);
_6 = _5.2 as isize;
_8 = true;
RET.0 = _1.fld0 - _2.fld0;
_7 = 13026_u16 as isize;
_7 = _3;
RET.0 = 2870045976_u32 as f32;
_5.1 = _4;
_2.fld1 = (-85_i8) as u8;
_1.fld0 = RET.0 * RET.0;
_2.fld2 = _3;
_5.0 = Adt35 { fld0: RET.0,fld1: _1.fld1,fld2: _7,fld3: _1.fld3 };
_9 = 10085896093713112309_u64 as f64;
_1.fld0 = 6824927871467511827_u64 as f32;
_9 = (-116_i8) as f64;
_1.fld1 = !_2.fld1;
_2.fld2 = _5.0.fld2 ^ _7;
Goto(bb6)
}
bb6 = {
_2.fld0 = _1.fld0 * _5.0.fld0;
_6 = -_3;
_8 = !true;
_2.fld3 = [13638_i16,(-25601_i16),(-9724_i16)];
RET.0 = _1.fld0;
_5.0.fld1 = _2.fld1 & _2.fld1;
_5.2 = _5.0.fld2 as usize;
_1.fld1 = _2.fld1 ^ _2.fld1;
_5.1 = _4;
_5.0.fld3 = [(-2450_i16),(-22116_i16),30228_i16];
_1.fld1 = _2.fld1 - _5.0.fld1;
_8 = !false;
Call(_7 = fn10(_5.0, _2.fld2, _5.0, _5.0, _3, _1.fld2, _2, _2, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = _7;
_11 = RET.0 * _5.0.fld0;
_5.0 = Adt35 { fld0: _11,fld1: _1.fld1,fld2: _2.fld2,fld3: _1.fld3 };
_5.1 = _4;
_5.3 = [_2.fld2,_10,_3,_10];
_5.2 = !3_usize;
_6 = -_5.0.fld2;
_1 = Adt35 { fld0: _5.0.fld0,fld1: _5.0.fld1,fld2: _5.0.fld2,fld3: _2.fld3 };
_7 = _5.0.fld2 >> _5.0.fld2;
_5.0.fld2 = _10;
RET.0 = 13714850870666564705_u64 as f32;
_5.0 = Adt35 { fld0: RET.0,fld1: _1.fld1,fld2: _10,fld3: _1.fld3 };
_2.fld2 = _10;
_2.fld2 = _1.fld2;
_1 = _2;
_2.fld2 = _6;
RET.0 = -_2.fld0;
_4 = _5.1;
_12.0 = _3 as f32;
_5.3 = [_7,_5.0.fld2,_5.0.fld2,_1.fld2];
Goto(bb8)
}
bb8 = {
_6 = -_10;
Call(_5.0 = fn11(_1, _7, _5.2, _1.fld2, _5.3, _6, _5.3, _7, _12, _7, _2.fld0, _5.3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5.0.fld3 = [(-7216_i16),(-27428_i16),(-2491_i16)];
_5.0.fld2 = _7 & _2.fld2;
Call(RET = fn12(_1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9 = _1.fld0 as f64;
_1 = Adt35 { fld0: _12.0,fld1: _5.0.fld1,fld2: _6,fld3: _5.0.fld3 };
_5.0.fld3 = _1.fld3;
_1.fld0 = _12.0;
_1.fld0 = _12.0 + _5.0.fld0;
_12.0 = RET.0;
_19 = _7 * _7;
_13 = _1.fld2;
_3 = !_1.fld2;
_5.2 = 2_usize;
_5.0.fld3 = [(-12192_i16),25122_i16,21931_i16];
Goto(bb11)
}
bb11 = {
_6 = (-129320786619845825044487536979282725334_i128) as isize;
_17 = _9;
_11 = _5.0.fld0 + _2.fld0;
_6 = _5.0.fld2 & _1.fld2;
_20 = _1.fld2 | _19;
_23 = _12;
_5.3 = [_6,_6,_5.0.fld2,_19];
_5.0.fld3 = [18682_i16,(-1691_i16),(-2625_i16)];
_12.0 = -_5.0.fld0;
_5.0.fld1 = _1.fld1 >> _6;
_2.fld3 = _1.fld3;
_4 = _5.1;
_5.0.fld3 = _2.fld3;
_18 = 68_i8;
_15.1 = core::ptr::addr_of!(_25);
_14 = _9 as i64;
_10 = _7;
RET.0 = _2.fld0 - _1.fld0;
_2.fld2 = 22772714998348447223657098343590208344_u128 as isize;
_23 = (_1.fld0,);
Goto(bb12)
}
bb12 = {
Call(_29 = dump_var(9_usize, 19_usize, Move(_19), 13_usize, Move(_13), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_29 = dump_var(9_usize, 10_usize, Move(_10), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: Adt35,mut _2: isize,mut _3: Adt35,mut _4: Adt35,mut _5: isize,mut _6: isize,mut _7: Adt35,mut _8: Adt35,mut _9: Adt35) -> isize {
mir! {
type RET = isize;
let _10: isize;
let _11: i16;
let _12: *const (i32,);
let _13: &'static &'static u16;
let _14: ();
let _15: ();
{
RET = _3.fld1 as isize;
_1.fld2 = 65392_u16 as isize;
_10 = _8.fld2;
_4.fld1 = _7.fld1;
_9 = Adt35 { fld0: _7.fld0,fld1: _1.fld1,fld2: _7.fld2,fld3: _7.fld3 };
_8.fld2 = -_10;
_7.fld2 = !_3.fld2;
_9.fld1 = _3.fld1 * _1.fld1;
_9.fld1 = _7.fld2 as u8;
_9 = _3;
_3.fld0 = _4.fld0 * _7.fld0;
_3.fld1 = 42_i8 as u8;
_9.fld1 = 281044952323105825168257680326549069838_u128 as u8;
_9 = Adt35 { fld0: _4.fld0,fld1: _1.fld1,fld2: _5,fld3: _1.fld3 };
RET = _2 | _4.fld2;
_3.fld0 = _8.fld0;
_7.fld3 = _9.fld3;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(10_usize, 5_usize, Move(_5), 6_usize, Move(_6), 15_usize, _15, 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: Adt35,mut _2: isize,mut _3: usize,mut _4: isize,mut _5: [isize; 4],mut _6: isize,mut _7: [isize; 4],mut _8: isize,mut _9: (f32,),mut _10: isize,mut _11: f32,mut _12: [isize; 4]) -> Adt35 {
mir! {
type RET = Adt35;
let _13: u32;
let _14: ();
let _15: ();
{
RET.fld3 = [(-27067_i16),(-29277_i16),(-17918_i16)];
RET.fld1 = !_1.fld1;
_1.fld3 = RET.fld3;
RET.fld3 = [30413_i16,16790_i16,27400_i16];
_12 = [_6,_8,_2,_10];
RET.fld1 = '\u{a504d}' as u8;
Goto(bb1)
}
bb1 = {
_10 = -_1.fld2;
_1.fld1 = RET.fld1 | RET.fld1;
_7 = [_6,_2,_8,_4];
_3 = 2067566698_u32 as usize;
_3 = 1255643926_u32 as usize;
_8 = 7842196370444091884_i64 as isize;
RET = Adt35 { fld0: _9.0,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
_1.fld0 = _9.0;
_7 = [_2,RET.fld2,_6,_2];
RET = _1;
_13 = !3644792458_u32;
_7 = [_2,_6,_1.fld2,_1.fld2];
_6 = _2 ^ _2;
RET = Adt35 { fld0: _1.fld0,fld1: _1.fld1,fld2: _6,fld3: _1.fld3 };
_9.0 = 17944_u16 as f32;
_1.fld3 = [(-32013_i16),2217_i16,(-24098_i16)];
_1.fld2 = -_6;
_13 = !1624077026_u32;
_9 = (_11,);
_6 = RET.fld2;
RET.fld3 = _1.fld3;
RET.fld3 = [(-23555_i16),(-23603_i16),(-28131_i16)];
_11 = (-380768404837784963_i64) as f32;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(11_usize, 10_usize, Move(_10), 13_usize, Move(_13), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_14 = dump_var(11_usize, 2_usize, Move(_2), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: Adt35) -> (f32,) {
mir! {
type RET = (f32,);
let _2: (*mut &'static (i32,), *mut &'static (i32,));
let _3: isize;
let _4: *const u16;
let _5: *const (*mut &'static (i32,), *mut &'static (i32,));
let _6: isize;
let _7: *const u16;
let _8: bool;
let _9: Adt73;
let _10: char;
let _11: &'static i128;
let _12: Adt49;
let _13: i64;
let _14: char;
let _15: ((*mut &'static i128,), *const isize);
let _16: *const (*mut &'static (i32,), *mut &'static (i32,));
let _17: f32;
let _18: f64;
let _19: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _20: i32;
let _21: ((i32,), usize);
let _22: u32;
let _23: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _24: ((f32,), *const u64, char, u64);
let _25: *mut &'static i128;
let _26: isize;
let _27: (char, u128, bool);
let _28: &'static &'static u16;
let _29: i8;
let _30: char;
let _31: (*const u128, u64, char, *const u128);
let _32: &'static &'static &'static i128;
let _33: u128;
let _34: ();
let _35: ();
{
_1.fld3 = [26443_i16,22273_i16,(-7496_i16)];
RET.0 = _1.fld0;
RET.0 = _1.fld0;
RET.0 = -_1.fld0;
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld0 = 13285377767178967735721852673629308168_i128 as f32;
RET = (_1.fld0,);
_1.fld2 = 4_isize * 82_isize;
RET = (_1.fld0,);
_1.fld2 = true as isize;
RET.0 = -_1.fld0;
_1.fld0 = (-26601_i16) as f32;
_1.fld0 = (-9032424649600676923_i64) as f32;
RET = (_1.fld0,);
RET = (_1.fld0,);
RET = (_1.fld0,);
RET.0 = _1.fld0 * _1.fld0;
_1.fld0 = RET.0;
_1.fld0 = -RET.0;
_1.fld2 = 78_isize;
RET = (_1.fld0,);
Goto(bb1)
}
bb1 = {
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = 233_u8 | 182_u8;
_3 = (-8184549033274150880_i64) as isize;
RET.0 = _1.fld0 - _1.fld0;
_3 = 14868576273364971854_u64 as isize;
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = !33_u8;
_1.fld1 = _1.fld2 as u8;
RET.0 = _1.fld0 + _1.fld0;
RET.0 = (-5681268837320407281_i64) as f32;
_5 = core::ptr::addr_of!(_2);
RET.0 = -_1.fld0;
_1.fld3 = [11499_i16,13678_i16,28673_i16];
_5 = core::ptr::addr_of!((*_5));
RET = (_1.fld0,);
_1.fld1 = !53_u8;
Call((*_5).1 = fn13(_1, _3, _1.fld0, _1.fld3, _1.fld0, _1, _3, RET.0, _1.fld0, _1.fld0, _3, _1.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = _1.fld0;
_3 = 41_i8 as isize;
_1.fld3 = [(-2733_i16),12407_i16,(-21040_i16)];
_3 = _1.fld2 + _1.fld2;
_5 = core::ptr::addr_of!(_2);
_6 = _3;
_5 = core::ptr::addr_of!(_2);
RET.0 = _1.fld0 + _1.fld0;
_1.fld0 = (-4551353884049138709_i64) as f32;
_1.fld3 = [25152_i16,23167_i16,141_i16];
RET = (_1.fld0,);
_1.fld0 = -RET.0;
match _1.fld2 {
0 => bb3,
78 => bb5,
_ => bb4
}
}
bb3 = {
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = 233_u8 | 182_u8;
_3 = (-8184549033274150880_i64) as isize;
RET.0 = _1.fld0 - _1.fld0;
_3 = 14868576273364971854_u64 as isize;
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = !33_u8;
_1.fld1 = _1.fld2 as u8;
RET.0 = _1.fld0 + _1.fld0;
RET.0 = (-5681268837320407281_i64) as f32;
_5 = core::ptr::addr_of!(_2);
RET.0 = -_1.fld0;
_1.fld3 = [11499_i16,13678_i16,28673_i16];
_5 = core::ptr::addr_of!((*_5));
RET = (_1.fld0,);
_1.fld1 = !53_u8;
Call((*_5).1 = fn13(_1, _3, _1.fld0, _1.fld3, _1.fld0, _1, _3, RET.0, _1.fld0, _1.fld0, _3, _1.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_1.fld1 = 246_u8 - 176_u8;
RET.0 = -_1.fld0;
_8 = false;
match _1.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
78 => bb6,
_ => bb4
}
}
bb6 = {
_10 = '\u{18fc}';
_1.fld0 = RET.0;
_1.fld3 = [8372_i16,(-20176_i16),30554_i16];
_8 = !false;
_3 = _1.fld0 as isize;
_10 = '\u{e307}';
_8 = !true;
_1.fld0 = -RET.0;
_1.fld3 = [(-31468_i16),(-14053_i16),19163_i16];
_8 = !false;
RET.0 = -_1.fld0;
_1.fld3 = [(-27132_i16),(-12656_i16),(-3757_i16)];
_1.fld3 = [(-10159_i16),6930_i16,29204_i16];
RET.0 = _1.fld0;
_3 = _6;
match _1.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
78 => bb7,
_ => bb5
}
}
bb7 = {
RET = (_1.fld0,);
RET = (_1.fld0,);
_10 = '\u{e5e91}';
RET.0 = -_1.fld0;
_13 = 9081276443591199497_i64;
RET = (_1.fld0,);
_8 = true;
_5 = core::ptr::addr_of!((*_5));
_13 = (-99_i8) as i64;
_14 = _10;
_1.fld3 = [(-29062_i16),16855_i16,(-10669_i16)];
_6 = !_3;
_6 = _1.fld2;
RET.0 = (-46_i8) as f32;
_6 = _13 as isize;
_1.fld0 = RET.0;
_14 = _10;
_14 = _10;
_13 = (-52099518439011578899351835161628618607_i128) as i64;
RET = (_1.fld0,);
match _1.fld2 {
0 => bb8,
1 => bb9,
78 => bb11,
_ => bb10
}
}
bb8 = {
RET.0 = _1.fld0;
_3 = 41_i8 as isize;
_1.fld3 = [(-2733_i16),12407_i16,(-21040_i16)];
_3 = _1.fld2 + _1.fld2;
_5 = core::ptr::addr_of!(_2);
_6 = _3;
_5 = core::ptr::addr_of!(_2);
RET.0 = _1.fld0 + _1.fld0;
_1.fld0 = (-4551353884049138709_i64) as f32;
_1.fld3 = [25152_i16,23167_i16,141_i16];
RET = (_1.fld0,);
_1.fld0 = -RET.0;
match _1.fld2 {
0 => bb3,
78 => bb5,
_ => bb4
}
}
bb9 = {
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = 233_u8 | 182_u8;
_3 = (-8184549033274150880_i64) as isize;
RET.0 = _1.fld0 - _1.fld0;
_3 = 14868576273364971854_u64 as isize;
RET = (_1.fld0,);
RET = (_1.fld0,);
_1.fld1 = !33_u8;
_1.fld1 = _1.fld2 as u8;
RET.0 = _1.fld0 + _1.fld0;
RET.0 = (-5681268837320407281_i64) as f32;
_5 = core::ptr::addr_of!(_2);
RET.0 = -_1.fld0;
_1.fld3 = [11499_i16,13678_i16,28673_i16];
_5 = core::ptr::addr_of!((*_5));
RET = (_1.fld0,);
_1.fld1 = !53_u8;
Call((*_5).1 = fn13(_1, _3, _1.fld0, _1.fld3, _1.fld0, _1, _3, RET.0, _1.fld0, _1.fld0, _3, _1.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_10 = _14;
_6 = _1.fld2;
_14 = _10;
RET = (_1.fld0,);
_13 = 4160189117573875104_i64 | 1560667066700923267_i64;
RET = (_1.fld0,);
_1.fld1 = 5622_u16 as u8;
_15.1 = core::ptr::addr_of!(_1.fld2);
_13 = 8977436821404277368_i64;
Goto(bb12)
}
bb12 = {
_16 = core::ptr::addr_of!(_2);
_5 = core::ptr::addr_of!((*_16));
_6 = !_1.fld2;
_10 = _14;
RET = (_1.fld0,);
_16 = core::ptr::addr_of!((*_16));
_1.fld3 = [(-32027_i16),(-18726_i16),(-17022_i16)];
_5 = core::ptr::addr_of!((*_16));
_14 = _10;
_19.2.0 = core::ptr::addr_of_mut!(_11);
_10 = _14;
_1.fld2 = _3;
_18 = _1.fld1 as f64;
_20 = 1860701020_i32;
_17 = (-48_i8) as f32;
_1.fld2 = _6 << _1.fld1;
match _13 {
8977436821404277368 => bb13,
_ => bb11
}
}
bb13 = {
_21.0.0 = _20 | _20;
_20 = _21.0.0;
_15.0.0 = core::ptr::addr_of_mut!(_11);
_16 = core::ptr::addr_of!((*_5));
_21.1 = 65001667354134825016885052939496978185_i128 as usize;
_5 = core::ptr::addr_of!(_2);
RET.0 = _1.fld0;
_6 = !_1.fld2;
RET.0 = _1.fld0;
_1.fld2 = _6;
RET.0 = -_17;
_1.fld2 = _3;
RET = (_1.fld0,);
_5 = core::ptr::addr_of!(_2);
_10 = _14;
RET = (_1.fld0,);
_23.0 = core::ptr::addr_of!(_22);
_1.fld0 = _17;
_21.0 = (_20,);
_8 = true & true;
_16 = core::ptr::addr_of!((*_16));
_20 = _13 as i32;
_20 = _1.fld2 as i32;
_21.1 = !7_usize;
_1.fld2 = !_3;
_16 = core::ptr::addr_of!((*_16));
_15.0.0 = core::ptr::addr_of_mut!(_11);
_9 = Adt73::Variant1 { fld0: _21.0 };
Goto(bb14)
}
bb14 = {
_19.0 = core::ptr::addr_of!(_22);
_23.2.0 = core::ptr::addr_of_mut!(_11);
_27.1 = 288002509979262199016238503100198308796_u128;
_25 = core::ptr::addr_of_mut!(_11);
_1.fld2 = -_6;
_29 = 97_i8;
RET.0 = -_1.fld0;
place!(Field::<(i32,)>(Variant(_9, 1), 0)).0 = !_20;
_27.0 = _14;
_16 = core::ptr::addr_of!((*_5));
_24.0 = (RET.0,);
_24.2 = _27.0;
_17 = RET.0;
place!(Field::<(i32,)>(Variant(_9, 1), 0)).0 = 18886_i16 as i32;
_31.2 = _14;
_19.0 = core::ptr::addr_of!(_22);
_24.0.0 = RET.0 - RET.0;
RET = _24.0;
RET.0 = _20 as f32;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(12_usize, 20_usize, Move(_20), 6_usize, Move(_6), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: Adt35,mut _2: isize,mut _3: f32,mut _4: [i16; 3],mut _5: f32,mut _6: Adt35,mut _7: isize,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: isize,mut _12: isize) -> *mut &'static (i32,) {
mir! {
type RET = *mut &'static (i32,);
let _13: bool;
let _14: Adt63;
let _15: &'static (Adt35, char, usize, [isize; 4]);
let _16: f32;
let _17: bool;
let _18: f64;
let _19: f64;
let _20: f32;
let _21: isize;
let _22: [i16; 3];
let _23: i8;
let _24: char;
let _25: usize;
let _26: ((f32,), *const u64, char, u64);
let _27: isize;
let _28: isize;
let _29: i128;
let _30: char;
let _31: *const isize;
let _32: i64;
let _33: *mut f32;
let _34: i32;
let _35: *mut &'static i128;
let _36: f64;
let _37: Adt52;
let _38: (&'static i128,);
let _39: (u64,);
let _40: *const u64;
let _41: i128;
let _42: Adt67;
let _43: f32;
let _44: Adt48;
let _45: bool;
let _46: isize;
let _47: f32;
let _48: &'static (i32,);
let _49: Adt35;
let _50: &'static &'static i128;
let _51: i8;
let _52: bool;
let _53: bool;
let _54: f32;
let _55: ();
let _56: ();
{
_12 = _2;
_7 = !_6.fld2;
Goto(bb1)
}
bb1 = {
_1.fld1 = 3461402562081750978_u64 as u8;
_12 = _1.fld2 << _6.fld2;
_9 = _1.fld0 - _5;
_1.fld0 = -_5;
_6 = Adt35 { fld0: _1.fld0,fld1: _1.fld1,fld2: _7,fld3: _1.fld3 };
_8 = _5;
_1.fld0 = _5;
_1.fld2 = _12;
_11 = !_1.fld2;
_1.fld2 = 1984040862_i32 as isize;
_1.fld0 = _3 - _9;
_8 = _9;
_1.fld0 = _9 - _9;
_5 = _8 * _1.fld0;
_10 = _6.fld0 + _8;
_1.fld0 = _5 - _5;
_6.fld2 = _2;
_2 = 6881682351741179841352314798262352414_u128 as isize;
_9 = -_1.fld0;
_6.fld3 = [(-3524_i16),25093_i16,(-5873_i16)];
_5 = _9;
_4 = [20741_i16,(-16997_i16),(-18161_i16)];
_7 = _1.fld2;
_16 = _9 + _5;
_17 = false;
_4 = [22887_i16,(-17448_i16),(-30902_i16)];
_13 = !_17;
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
_1.fld0 = _16;
_6.fld1 = _1.fld1;
_3 = _5;
_21 = _6.fld2;
_20 = 91_i8 as f32;
_1.fld3 = [(-9505_i16),(-4584_i16),(-3011_i16)];
_1 = _6;
_12 = _11 * _11;
_8 = _9;
_1.fld0 = -_16;
_6.fld2 = _11 * _11;
_1.fld1 = (-2975161911379778775_i64) as u8;
_19 = (-73_i8) as f64;
_6 = _1;
_6.fld2 = 116_i8 as isize;
_5 = _6.fld0 - _6.fld0;
_6.fld0 = _5 * _5;
_22 = [(-31660_i16),(-25654_i16),21684_i16];
_11 = _1.fld2;
_20 = _6.fld0 * _6.fld0;
_6.fld3 = [8023_i16,(-4755_i16),27244_i16];
Goto(bb4)
}
bb4 = {
_1 = Adt35 { fld0: _16,fld1: _6.fld1,fld2: _2,fld3: _6.fld3 };
_16 = (-659485103700164949_i64) as f32;
_5 = _20 - _20;
_22 = [13231_i16,(-20732_i16),4947_i16];
_21 = _12 ^ _1.fld2;
_18 = _19;
_23 = 683188627744474005_i64 as i8;
_1 = _6;
_23 = 123_i8;
_13 = _17;
_1.fld2 = _12;
_1 = Adt35 { fld0: _5,fld1: _6.fld1,fld2: _21,fld3: _22 };
_4 = [21346_i16,11200_i16,(-25406_i16)];
_6 = Adt35 { fld0: _1.fld0,fld1: _1.fld1,fld2: _1.fld2,fld3: _22 };
_21 = _12 >> _12;
_11 = _18 as isize;
_8 = -_1.fld0;
_20 = -_1.fld0;
Call(_1.fld3 = fn14(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6.fld0 = _16 + _5;
_20 = _9 + _8;
_3 = _5 + _5;
_16 = -_3;
_5 = _16;
_6.fld1 = _1.fld1 & _1.fld1;
_1.fld2 = _7 | _6.fld2;
_24 = '\u{b908}';
_9 = _16;
_8 = _1.fld0;
_26.2 = _24;
_3 = (-36822655881929336554710897914068660325_i128) as f32;
_26.0.0 = (-15576_i16) as f32;
_26.2 = _24;
_23 = 57_i8 << _21;
_1 = Adt35 { fld0: _16,fld1: _6.fld1,fld2: _7,fld3: _4 };
_6 = Adt35 { fld0: _20,fld1: _1.fld1,fld2: _21,fld3: _1.fld3 };
_26.0 = (_6.fld0,);
_19 = (-8409039524652510175_i64) as f64;
_2 = _6.fld2 & _7;
_29 = !(-142080299101704944249535737549385029574_i128);
Goto(bb6)
}
bb6 = {
_23 = (-6_i8);
_24 = _26.2;
_24 = _26.2;
_25 = !10670828708506077460_usize;
_26.3 = 17208843514544876570_u64;
_21 = _6.fld2;
_10 = _8;
_9 = (-5954108288842912750_i64) as f32;
_27 = _6.fld2 - _21;
_26.0 = (_5,);
_27 = _11;
_8 = _23 as f32;
_13 = !_17;
_28 = (-3856598111572851750_i64) as isize;
_27 = _2;
_4 = _22;
_21 = -_27;
_21 = _29 as isize;
_31 = core::ptr::addr_of!(_27);
(*_31) = _21 & _6.fld2;
_32 = (-5366796762557138552_i64);
_26.0.0 = _6.fld0;
_32 = 6803046688322841027_i64 | (-4794876634388891983_i64);
_6.fld3 = [12634_i16,(-28366_i16),17262_i16];
_10 = _26.0.0 * _26.0.0;
_22 = _4;
_16 = (*_31) as f32;
match _26.3 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
17208843514544876570 => bb14,
_ => bb13
}
}
bb7 = {
_6.fld0 = _16 + _5;
_20 = _9 + _8;
_3 = _5 + _5;
_16 = -_3;
_5 = _16;
_6.fld1 = _1.fld1 & _1.fld1;
_1.fld2 = _7 | _6.fld2;
_24 = '\u{b908}';
_9 = _16;
_8 = _1.fld0;
_26.2 = _24;
_3 = (-36822655881929336554710897914068660325_i128) as f32;
_26.0.0 = (-15576_i16) as f32;
_26.2 = _24;
_23 = 57_i8 << _21;
_1 = Adt35 { fld0: _16,fld1: _6.fld1,fld2: _7,fld3: _4 };
_6 = Adt35 { fld0: _20,fld1: _1.fld1,fld2: _21,fld3: _1.fld3 };
_26.0 = (_6.fld0,);
_19 = (-8409039524652510175_i64) as f64;
_2 = _6.fld2 & _7;
_29 = !(-142080299101704944249535737549385029574_i128);
Goto(bb6)
}
bb8 = {
_1 = Adt35 { fld0: _16,fld1: _6.fld1,fld2: _2,fld3: _6.fld3 };
_16 = (-659485103700164949_i64) as f32;
_5 = _20 - _20;
_22 = [13231_i16,(-20732_i16),4947_i16];
_21 = _12 ^ _1.fld2;
_18 = _19;
_23 = 683188627744474005_i64 as i8;
_1 = _6;
_23 = 123_i8;
_13 = _17;
_1.fld2 = _12;
_1 = Adt35 { fld0: _5,fld1: _6.fld1,fld2: _21,fld3: _22 };
_4 = [21346_i16,11200_i16,(-25406_i16)];
_6 = Adt35 { fld0: _1.fld0,fld1: _1.fld1,fld2: _1.fld2,fld3: _22 };
_21 = _12 >> _12;
_11 = _18 as isize;
_8 = -_1.fld0;
_20 = -_1.fld0;
Call(_1.fld3 = fn14(), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_1.fld0 = _16;
_6.fld1 = _1.fld1;
_3 = _5;
_21 = _6.fld2;
_20 = 91_i8 as f32;
_1.fld3 = [(-9505_i16),(-4584_i16),(-3011_i16)];
_1 = _6;
_12 = _11 * _11;
_8 = _9;
_1.fld0 = -_16;
_6.fld2 = _11 * _11;
_1.fld1 = (-2975161911379778775_i64) as u8;
_19 = (-73_i8) as f64;
_6 = _1;
_6.fld2 = 116_i8 as isize;
_5 = _6.fld0 - _6.fld0;
_6.fld0 = _5 * _5;
_22 = [(-31660_i16),(-25654_i16),21684_i16];
_11 = _1.fld2;
_20 = _6.fld0 * _6.fld0;
_6.fld3 = [8023_i16,(-4755_i16),27244_i16];
Goto(bb4)
}
bb10 = {
Goto(bb3)
}
bb11 = {
_1.fld1 = 3461402562081750978_u64 as u8;
_12 = _1.fld2 << _6.fld2;
_9 = _1.fld0 - _5;
_1.fld0 = -_5;
_6 = Adt35 { fld0: _1.fld0,fld1: _1.fld1,fld2: _7,fld3: _1.fld3 };
_8 = _5;
_1.fld0 = _5;
_1.fld2 = _12;
_11 = !_1.fld2;
_1.fld2 = 1984040862_i32 as isize;
_1.fld0 = _3 - _9;
_8 = _9;
_1.fld0 = _9 - _9;
_5 = _8 * _1.fld0;
_10 = _6.fld0 + _8;
_1.fld0 = _5 - _5;
_6.fld2 = _2;
_2 = 6881682351741179841352314798262352414_u128 as isize;
_9 = -_1.fld0;
_6.fld3 = [(-3524_i16),25093_i16,(-5873_i16)];
_5 = _9;
_4 = [20741_i16,(-16997_i16),(-18161_i16)];
_7 = _1.fld2;
_16 = _9 + _5;
_17 = false;
_4 = [22887_i16,(-17448_i16),(-30902_i16)];
_13 = !_17;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
(*_31) = -_2;
_33 = core::ptr::addr_of_mut!(_3);
_1.fld2 = _27 & (*_31);
_26.2 = _24;
_8 = _1.fld2 as f32;
(*_33) = _6.fld0 + _5;
_4 = [31745_i16,3776_i16,7224_i16];
_3 = (-557016790_i32) as f32;
_33 = core::ptr::addr_of_mut!((*_33));
_19 = _18 + _18;
_1.fld1 = !_6.fld1;
Goto(bb15)
}
bb15 = {
_18 = -_19;
_5 = _20 - _6.fld0;
_18 = _25 as f64;
_1 = Adt35 { fld0: _5,fld1: _6.fld1,fld2: (*_31),fld3: _6.fld3 };
_19 = -_18;
_20 = _25 as f32;
_10 = -_6.fld0;
(*_33) = _1.fld0 - _1.fld0;
_34 = (-2026073871_i32);
(*_31) = _2 << _32;
_33 = core::ptr::addr_of_mut!(_1.fld0);
_1.fld0 = _3 + _10;
_18 = _19 - _19;
_29 = _6.fld0 as i128;
_1.fld3 = _6.fld3;
_5 = _6.fld0;
_26.0.0 = -_3;
_2 = !_6.fld2;
(*_31) = _2;
_17 = !_13;
_1.fld3 = [(-9324_i16),(-9252_i16),10501_i16];
match _23 {
0 => bb11,
1 => bb7,
340282366920938463463374607431768211450 => bb16,
_ => bb5
}
}
bb16 = {
_26.3 = 803844840_u32 as u64;
_24 = _26.2;
_1.fld2 = _24 as isize;
_26.0.0 = -(*_33);
_37.fld1 = _34;
(*_33) = _5;
_29 = 55428997711758568802836348703059921587_i128;
_27 = _6.fld2 + _28;
_20 = -(*_33);
_34 = _13 as i32;
(*_31) = 90799855330107129661775992836251695286_u128 as isize;
Call(_30 = fn15((*_33), _6, _6, _1.fld0, _6, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_33 = core::ptr::addr_of_mut!(_6.fld0);
_26.0.0 = _5 - _5;
_27 = -_2;
Call(_34 = fn19(_6, _1.fld0, Move(_31), _6.fld0, (*_33), _6, _26.0, Move(_33), (*_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_4 = [26517_i16,(-28171_i16),(-19867_i16)];
_26.2 = _24;
_16 = _27 as f32;
_26.0 = (_1.fld0,);
_9 = -_20;
_36 = _18;
_39 = (_26.3,);
_11 = _24 as isize;
_7 = _6.fld2 ^ _2;
_12 = _7 - _7;
_23 = !(-60_i8);
Goto(bb19)
}
bb19 = {
_1.fld2 = _27 + _12;
_40 = core::ptr::addr_of!(_26.3);
_1.fld1 = _6.fld1;
_5 = _3;
_35 = core::ptr::addr_of_mut!(_38.0);
_1.fld1 = !_6.fld1;
_32 = _25 as i64;
_4 = [(-25526_i16),(-6980_i16),(-22733_i16)];
_6 = _1;
_6.fld2 = _12 & _2;
_35 = core::ptr::addr_of_mut!(_38.0);
_1.fld2 = _12 + _6.fld2;
_1.fld1 = _13 as u8;
_24 = _30;
_37.fld1 = _34 + _34;
_6.fld3 = _4;
_33 = core::ptr::addr_of_mut!(_8);
_44.fld4.3 = (*_40);
_4 = _22;
(*_35) = &_29;
(*_33) = _1.fld0;
match _29 {
0 => bb8,
1 => bb6,
2 => bb11,
55428997711758568802836348703059921587 => bb21,
_ => bb20
}
}
bb20 = {
Return()
}
bb21 = {
(*_33) = _18 as f32;
_41 = _29 ^ _29;
_31 = core::ptr::addr_of!(_7);
_7 = !_6.fld2;
_32 = -7176586104531702076_i64;
_16 = _32 as f32;
_24 = _26.2;
_39.0 = (*_40) + (*_40);
Goto(bb22)
}
bb22 = {
_40 = core::ptr::addr_of!(_26.3);
_6.fld1 = _29 as u8;
_37.fld0 = _25 - _25;
_44.fld0 = 1353372502_u32;
match _29 {
0 => bb1,
1 => bb8,
2 => bb23,
55428997711758568802836348703059921587 => bb25,
_ => bb24
}
}
bb23 = {
_6.fld0 = _16 + _5;
_20 = _9 + _8;
_3 = _5 + _5;
_16 = -_3;
_5 = _16;
_6.fld1 = _1.fld1 & _1.fld1;
_1.fld2 = _7 | _6.fld2;
_24 = '\u{b908}';
_9 = _16;
_8 = _1.fld0;
_26.2 = _24;
_3 = (-36822655881929336554710897914068660325_i128) as f32;
_26.0.0 = (-15576_i16) as f32;
_26.2 = _24;
_23 = 57_i8 << _21;
_1 = Adt35 { fld0: _16,fld1: _6.fld1,fld2: _7,fld3: _4 };
_6 = Adt35 { fld0: _20,fld1: _1.fld1,fld2: _21,fld3: _1.fld3 };
_26.0 = (_6.fld0,);
_19 = (-8409039524652510175_i64) as f64;
_2 = _6.fld2 & _7;
_29 = !(-142080299101704944249535737549385029574_i128);
Goto(bb6)
}
bb24 = {
Return()
}
bb25 = {
_47 = _5;
_10 = _1.fld0;
_37.fld0 = 86574279811263485070538766525286717931_u128 as usize;
_44.fld4.0 = (_47,);
_44.fld5 = [_6.fld1];
(*_31) = _1.fld2;
(*_40) = 131688647346806671847493998838724610424_u128 as u64;
_16 = -_47;
(*_35) = &_41;
_6.fld1 = _1.fld1;
_44.fld4 = (_26.0, Move(_40), _30, _39.0);
Goto(bb26)
}
bb26 = {
_41 = !_29;
_49.fld3 = _6.fld3;
_29 = !_41;
_8 = -_6.fld0;
_53 = _13;
_44.fld3 = [_37.fld0,_25,_37.fld0,_37.fld0,_37.fld0,_37.fld0,_25];
_46 = _6.fld2 ^ _7;
_4 = [11248_i16,(-10958_i16),(-17420_i16)];
_16 = _20 + _8;
_6.fld1 = _1.fld1;
_8 = -_47;
_47 = _1.fld0;
(*_35) = &_41;
_19 = 58591557994218733537859012180598654710_u128 as f64;
_20 = _5 + (*_33);
_44.fld1 = _44.fld4.2;
_21 = _6.fld2 ^ (*_31);
_7 = _1.fld2 - _1.fld2;
_6.fld2 = 44337265210300045891265018744288637345_u128 as isize;
_26.3 = _32 as u64;
_52 = !_17;
RET = core::ptr::addr_of_mut!(_48);
_44.fld4.0.0 = -_26.0.0;
_44.fld4.1 = core::ptr::addr_of!(_44.fld4.3);
_40 = Move(_44.fld4.1);
_26.0.0 = _16;
_9 = _1.fld0;
Goto(bb27)
}
bb27 = {
Call(_55 = dump_var(13_usize, 53_usize, Move(_53), 32_usize, Move(_32), 2_usize, Move(_2), 34_usize, Move(_34)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_55 = dump_var(13_usize, 52_usize, Move(_52), 24_usize, Move(_24), 4_usize, Move(_4), 41_usize, Move(_41)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_55 = dump_var(13_usize, 12_usize, Move(_12), 23_usize, Move(_23), 28_usize, Move(_28), 56_usize, _56), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14() -> [i16; 3] {
mir! {
type RET = [i16; 3];
let _1: Adt63;
let _2: bool;
let _3: (bool, Adt52, [u8; 1], f64);
let _4: bool;
let _5: (*mut &'static (i32,), *mut &'static (i32,));
let _6: u16;
let _7: isize;
let _8: Adt35;
let _9: u32;
let _10: ([i8; 1], ((&'static i128,), f64, u16));
let _11: *mut &'static i128;
let _12: *const u128;
let _13: bool;
let _14: Adt63;
let _15: char;
let _16: i32;
let _17: [isize; 7];
let _18: bool;
let _19: (bool, Adt52, [u8; 1], f64);
let _20: Adt73;
let _21: bool;
let _22: i128;
let _23: [u32; 3];
let _24: &'static f32;
let _25: f32;
let _26: f64;
let _27: ();
let _28: ();
{
RET = [(-5167_i16),18887_i16,5796_i16];
RET = [(-29254_i16),(-15199_i16),(-16739_i16)];
RET = [(-21961_i16),15299_i16,(-16801_i16)];
RET = [16984_i16,11834_i16,2118_i16];
RET = [(-3481_i16),(-27813_i16),(-29641_i16)];
RET = [3220_i16,10599_i16,10562_i16];
RET = [(-8069_i16),(-806_i16),(-19207_i16)];
RET = [23182_i16,(-18192_i16),(-3280_i16)];
RET = [14607_i16,4841_i16,17230_i16];
RET = [6547_i16,(-2890_i16),(-26899_i16)];
RET = [20930_i16,11623_i16,20481_i16];
RET = [(-14533_i16),4331_i16,(-14725_i16)];
RET = [32622_i16,(-10677_i16),17296_i16];
RET = [263_i16,(-16045_i16),4644_i16];
RET = [(-4847_i16),(-15774_i16),(-3449_i16)];
RET = [(-23462_i16),(-3760_i16),(-7351_i16)];
RET = [(-7274_i16),(-6552_i16),19278_i16];
RET = [15923_i16,(-2501_i16),18519_i16];
RET = [699_i16,13858_i16,30441_i16];
RET = [(-11949_i16),(-29783_i16),(-22176_i16)];
RET = [(-15137_i16),9169_i16,29461_i16];
RET = [(-9811_i16),21809_i16,(-31776_i16)];
RET = [(-11938_i16),32351_i16,(-17041_i16)];
RET = [(-21518_i16),5288_i16,12569_i16];
RET = [931_i16,30633_i16,32421_i16];
RET = [1711_i16,(-22737_i16),(-19023_i16)];
Goto(bb1)
}
bb1 = {
RET = [27744_i16,(-5064_i16),(-2058_i16)];
_2 = false;
RET = [21786_i16,(-11038_i16),19576_i16];
RET = [16864_i16,(-30075_i16),(-21365_i16)];
Goto(bb2)
}
bb2 = {
RET = [(-31937_i16),(-14975_i16),21298_i16];
RET = [(-561_i16),5903_i16,(-26090_i16)];
RET = [27734_i16,16171_i16,(-842_i16)];
_3.2 = [126_u8];
_3.2 = [173_u8];
_3.1.fld1 = (-2126396555_i32);
_3.1.fld0 = 7_usize - 5823477430652696776_usize;
RET = [4393_i16,(-26401_i16),(-29128_i16)];
_3.3 = 54993395833335309_i64 as f64;
_4 = !_2;
_3.1.fld0 = 1528795124684220830_usize;
_3.0 = !_2;
RET = [(-6815_i16),19921_i16,17098_i16];
RET = [(-3690_i16),19270_i16,9918_i16];
_3.1.fld0 = !7_usize;
_4 = _2 & _2;
_3.2 = [227_u8];
_3.1.fld1 = 561846717_i32 ^ 1998713783_i32;
Goto(bb3)
}
bb3 = {
_2 = !_4;
_6 = !2081_u16;
_8.fld3 = [(-3190_i16),21231_i16,(-22514_i16)];
_3.3 = 31907_i16 as f64;
_10.0 = [(-128_i8)];
_8.fld0 = _6 as f32;
RET = [(-30900_i16),21601_i16,(-21496_i16)];
_9 = 3751589800_u32 & 3951784522_u32;
_3.1.fld0 = 13049974194355754366_usize & 7562409999021278739_usize;
_3.1.fld1 = 9223372036854775807_isize as i32;
RET = _8.fld3;
_10.0 = [(-75_i8)];
_8.fld2 = 9223372036854775807_isize << _3.1.fld0;
_10.1.1 = _3.1.fld1 as f64;
_8.fld2 = (-9223372036854775808_isize);
_6 = 28216_u16;
_7 = _8.fld2 + _8.fld2;
_9 = _10.1.1 as u32;
_8.fld1 = 130_u8 | 185_u8;
_3.0 = _3.1.fld0 >= _3.1.fld0;
_13 = _8.fld1 != _8.fld1;
_4 = _3.1.fld0 == _3.1.fld0;
_3.3 = -_10.1.1;
_10.1.1 = (-36511144534298831_i64) as f64;
Goto(bb4)
}
bb4 = {
Goto(bb5)
}
bb5 = {
_11 = core::ptr::addr_of_mut!(_10.1.0.0);
_2 = !_3.0;
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{5ee99}';
_16 = _3.1.fld1 ^ _3.1.fld1;
_4 = _2 | _13;
_3.3 = _10.1.1;
Goto(bb6)
}
bb6 = {
_3.1.fld0 = _8.fld1 as usize;
_16 = _3.1.fld1 >> _6;
_13 = _4;
_17 = [_8.fld2,_8.fld2,_7,_7,_7,_8.fld2,_7];
_2 = !_13;
_3.0 = _13 >= _2;
_8.fld3 = [(-27729_i16),(-17637_i16),11932_i16];
_3.2 = [_8.fld1];
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{9af68}';
_4 = _13;
_3.1.fld0 = !16922749922601563083_usize;
_8.fld1 = !39_u8;
_8.fld3 = RET;
_19.0 = _3.0;
RET = _8.fld3;
_18 = _3.0 >= _3.0;
_19.2 = _3.2;
_10.1.2 = _10.1.1 as u16;
_19.2 = [_8.fld1];
match _6 {
0 => bb1,
28216 => bb7,
_ => bb5
}
}
bb7 = {
_4 = _18 <= _18;
_7 = _9 as isize;
_15 = '\u{2beed}';
_19.1 = Adt52 { fld0: _3.1.fld0,fld1: _3.1.fld1,fld2: _3.1.fld2 };
_19.1.fld0 = _9 as usize;
_19.1 = _3.1;
match _6 {
0 => bb4,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
28216 => bb12,
_ => bb11
}
}
bb8 = {
RET = [27744_i16,(-5064_i16),(-2058_i16)];
_2 = false;
RET = [21786_i16,(-11038_i16),19576_i16];
RET = [16864_i16,(-30075_i16),(-21365_i16)];
Goto(bb2)
}
bb9 = {
_11 = core::ptr::addr_of_mut!(_10.1.0.0);
_2 = !_3.0;
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{5ee99}';
_16 = _3.1.fld1 ^ _3.1.fld1;
_4 = _2 | _13;
_3.3 = _10.1.1;
Goto(bb6)
}
bb10 = {
RET = [(-31937_i16),(-14975_i16),21298_i16];
RET = [(-561_i16),5903_i16,(-26090_i16)];
RET = [27734_i16,16171_i16,(-842_i16)];
_3.2 = [126_u8];
_3.2 = [173_u8];
_3.1.fld1 = (-2126396555_i32);
_3.1.fld0 = 7_usize - 5823477430652696776_usize;
RET = [4393_i16,(-26401_i16),(-29128_i16)];
_3.3 = 54993395833335309_i64 as f64;
_4 = !_2;
_3.1.fld0 = 1528795124684220830_usize;
_3.0 = !_2;
RET = [(-6815_i16),19921_i16,17098_i16];
RET = [(-3690_i16),19270_i16,9918_i16];
_3.1.fld0 = !7_usize;
_4 = _2 & _2;
_3.2 = [227_u8];
_3.1.fld1 = 561846717_i32 ^ 1998713783_i32;
Goto(bb3)
}
bb11 = {
_2 = !_4;
_6 = !2081_u16;
_8.fld3 = [(-3190_i16),21231_i16,(-22514_i16)];
_3.3 = 31907_i16 as f64;
_10.0 = [(-128_i8)];
_8.fld0 = _6 as f32;
RET = [(-30900_i16),21601_i16,(-21496_i16)];
_9 = 3751589800_u32 & 3951784522_u32;
_3.1.fld0 = 13049974194355754366_usize & 7562409999021278739_usize;
_3.1.fld1 = 9223372036854775807_isize as i32;
RET = _8.fld3;
_10.0 = [(-75_i8)];
_8.fld2 = 9223372036854775807_isize << _3.1.fld0;
_10.1.1 = _3.1.fld1 as f64;
_8.fld2 = (-9223372036854775808_isize);
_6 = 28216_u16;
_7 = _8.fld2 + _8.fld2;
_9 = _10.1.1 as u32;
_8.fld1 = 130_u8 | 185_u8;
_3.0 = _3.1.fld0 >= _3.1.fld0;
_13 = _8.fld1 != _8.fld1;
_4 = _3.1.fld0 == _3.1.fld0;
_3.3 = -_10.1.1;
_10.1.1 = (-36511144534298831_i64) as f64;
Goto(bb4)
}
bb12 = {
_19.1.fld1 = !_3.1.fld1;
_22 = 96548257443324889647459918527982406306_i128;
_10.1.0.0 = &_22;
_19.1.fld0 = _7 as usize;
_19 = _3;
_21 = _4 | _13;
_10.1.1 = _19.3 + _19.3;
_19 = (_18, _3.1, _3.2, _10.1.1);
_8.fld3 = [(-713_i16),13907_i16,7651_i16];
_19.3 = _3.1.fld1 as f64;
_18 = _19.0;
match _6 {
0 => bb7,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
28216 => bb19,
_ => bb18
}
}
bb13 = {
_2 = !_4;
_6 = !2081_u16;
_8.fld3 = [(-3190_i16),21231_i16,(-22514_i16)];
_3.3 = 31907_i16 as f64;
_10.0 = [(-128_i8)];
_8.fld0 = _6 as f32;
RET = [(-30900_i16),21601_i16,(-21496_i16)];
_9 = 3751589800_u32 & 3951784522_u32;
_3.1.fld0 = 13049974194355754366_usize & 7562409999021278739_usize;
_3.1.fld1 = 9223372036854775807_isize as i32;
RET = _8.fld3;
_10.0 = [(-75_i8)];
_8.fld2 = 9223372036854775807_isize << _3.1.fld0;
_10.1.1 = _3.1.fld1 as f64;
_8.fld2 = (-9223372036854775808_isize);
_6 = 28216_u16;
_7 = _8.fld2 + _8.fld2;
_9 = _10.1.1 as u32;
_8.fld1 = 130_u8 | 185_u8;
_3.0 = _3.1.fld0 >= _3.1.fld0;
_13 = _8.fld1 != _8.fld1;
_4 = _3.1.fld0 == _3.1.fld0;
_3.3 = -_10.1.1;
_10.1.1 = (-36511144534298831_i64) as f64;
Goto(bb4)
}
bb14 = {
_2 = !_4;
_6 = !2081_u16;
_8.fld3 = [(-3190_i16),21231_i16,(-22514_i16)];
_3.3 = 31907_i16 as f64;
_10.0 = [(-128_i8)];
_8.fld0 = _6 as f32;
RET = [(-30900_i16),21601_i16,(-21496_i16)];
_9 = 3751589800_u32 & 3951784522_u32;
_3.1.fld0 = 13049974194355754366_usize & 7562409999021278739_usize;
_3.1.fld1 = 9223372036854775807_isize as i32;
RET = _8.fld3;
_10.0 = [(-75_i8)];
_8.fld2 = 9223372036854775807_isize << _3.1.fld0;
_10.1.1 = _3.1.fld1 as f64;
_8.fld2 = (-9223372036854775808_isize);
_6 = 28216_u16;
_7 = _8.fld2 + _8.fld2;
_9 = _10.1.1 as u32;
_8.fld1 = 130_u8 | 185_u8;
_3.0 = _3.1.fld0 >= _3.1.fld0;
_13 = _8.fld1 != _8.fld1;
_4 = _3.1.fld0 == _3.1.fld0;
_3.3 = -_10.1.1;
_10.1.1 = (-36511144534298831_i64) as f64;
Goto(bb4)
}
bb15 = {
_11 = core::ptr::addr_of_mut!(_10.1.0.0);
_2 = !_3.0;
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{5ee99}';
_16 = _3.1.fld1 ^ _3.1.fld1;
_4 = _2 | _13;
_3.3 = _10.1.1;
Goto(bb6)
}
bb16 = {
_11 = core::ptr::addr_of_mut!(_10.1.0.0);
_2 = !_3.0;
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{5ee99}';
_16 = _3.1.fld1 ^ _3.1.fld1;
_4 = _2 | _13;
_3.3 = _10.1.1;
Goto(bb6)
}
bb17 = {
_4 = _18 <= _18;
_7 = _9 as isize;
_15 = '\u{2beed}';
_19.1 = Adt52 { fld0: _3.1.fld0,fld1: _3.1.fld1,fld2: _3.1.fld2 };
_19.1.fld0 = _9 as usize;
_19.1 = _3.1;
match _6 {
0 => bb4,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
28216 => bb12,
_ => bb11
}
}
bb18 = {
_3.1.fld0 = _8.fld1 as usize;
_16 = _3.1.fld1 >> _6;
_13 = _4;
_17 = [_8.fld2,_8.fld2,_7,_7,_7,_8.fld2,_7];
_2 = !_13;
_3.0 = _13 >= _2;
_8.fld3 = [(-27729_i16),(-17637_i16),11932_i16];
_3.2 = [_8.fld1];
_3.1.fld2 = [_6,_6,_6,_6,_6];
_15 = '\u{9af68}';
_4 = _13;
_3.1.fld0 = !16922749922601563083_usize;
_8.fld1 = !39_u8;
_8.fld3 = RET;
_19.0 = _3.0;
RET = _8.fld3;
_18 = _3.0 >= _3.0;
_19.2 = _3.2;
_10.1.2 = _10.1.1 as u16;
_19.2 = [_8.fld1];
match _6 {
0 => bb1,
28216 => bb7,
_ => bb5
}
}
bb19 = {
_2 = !_21;
_19.1.fld1 = _3.1.fld1;
(*_11) = &_22;
_24 = &_8.fld0;
_10.1.1 = _3.3;
_3.1.fld0 = _19.1.fld0 << _8.fld1;
_26 = _10.1.1;
Goto(bb20)
}
bb20 = {
Call(_27 = dump_var(14_usize, 6_usize, Move(_6), 21_usize, Move(_21), 17_usize, Move(_17), 18_usize, Move(_18)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_27 = dump_var(14_usize, 22_usize, Move(_22), 9_usize, Move(_9), 28_usize, _28, 28_usize, _28), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: f32,mut _2: Adt35,mut _3: Adt35,mut _4: f32,mut _5: Adt35,mut _6: *mut f32) -> char {
mir! {
type RET = char;
let _7: isize;
let _8: char;
let _9: bool;
let _10: i64;
let _11: u8;
let _12: (&'static i128,);
let _13: Adt52;
let _14: isize;
let _15: ();
let _16: ();
{
_3.fld0 = _5.fld2 as f32;
_2.fld1 = _5.fld1 >> _3.fld2;
_5.fld2 = true as isize;
RET = '\u{df840}';
_3.fld3 = [31316_i16,27200_i16,(-13119_i16)];
_3.fld2 = !_2.fld2;
_5.fld3 = [(-19871_i16),16610_i16,(-5630_i16)];
_2.fld3 = [27123_i16,11875_i16,21194_i16];
_5.fld2 = _2.fld2;
_1 = _4;
_3.fld0 = _5.fld0;
_2 = Adt35 { fld0: _5.fld0,fld1: _3.fld1,fld2: _5.fld2,fld3: _3.fld3 };
_5.fld3 = [16586_i16,(-21710_i16),6165_i16];
_3.fld3 = [(-20241_i16),(-18244_i16),(-11875_i16)];
Goto(bb1)
}
bb1 = {
_4 = _5.fld0 * _1;
Call(_5.fld0 = fn16(Move(_6), _3, _3.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = core::ptr::addr_of_mut!(_1);
_3.fld1 = RET as u8;
_7 = _2.fld2;
_3.fld0 = (*_6) + (*_6);
_2.fld2 = _5.fld2 << _7;
_1 = 15_i8 as f32;
_8 = RET;
_4 = -_2.fld0;
_4 = (-49_i8) as f32;
_9 = !false;
_1 = _3.fld0;
_5 = _3;
_9 = true & true;
_5 = Adt35 { fld0: _2.fld0,fld1: _2.fld1,fld2: _2.fld2,fld3: _2.fld3 };
RET = _8;
Goto(bb3)
}
bb3 = {
_5.fld2 = !_7;
Goto(bb4)
}
bb4 = {
RET = _8;
_2 = Adt35 { fld0: (*_6),fld1: _3.fld1,fld2: _3.fld2,fld3: _5.fld3 };
_10 = 2319704968476361766_i64;
_2.fld3 = [1484_i16,(-28176_i16),(-8520_i16)];
_2.fld2 = !_5.fld2;
_5.fld0 = _2.fld0;
Call(_7 = fn18(_3.fld0, _5, _2.fld0, Move(_6), _5, (*_6), _3, _2, (*_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = _2.fld1;
_2.fld2 = _5.fld2;
_2 = _3;
_2.fld0 = _5.fld0 - _5.fld0;
_11 = _8 as u8;
_2.fld2 = -_7;
_9 = false ^ true;
_3 = Adt35 { fld0: _5.fld0,fld1: _11,fld2: _7,fld3: _2.fld3 };
_5.fld0 = _3.fld0;
RET = _8;
_13.fld2 = [46760_u16,56516_u16,63655_u16,51262_u16,30284_u16];
_11 = _3.fld1 << _10;
_3.fld1 = _5.fld1;
RET = _8;
_13.fld0 = 2_usize;
_3.fld2 = _5.fld2 | _7;
_2.fld1 = _11 | _5.fld1;
_6 = core::ptr::addr_of_mut!(_3.fld0);
_2.fld3 = _3.fld3;
_3.fld0 = _10 as f32;
_3.fld0 = _2.fld1 as f32;
_2.fld1 = _5.fld1 * _11;
_5.fld1 = _2.fld1 | _2.fld1;
_13.fld2 = [8828_u16,16185_u16,22465_u16,9196_u16,56764_u16];
_2.fld1 = 46_i8 as u8;
_13.fld1 = 150577903_i32 - 1275264625_i32;
match _10 {
2319704968476361766 => bb7,
_ => bb6
}
}
bb6 = {
_4 = _5.fld0 * _1;
Call(_5.fld0 = fn16(Move(_6), _3, _3.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_2.fld2 = _3.fld2;
(*_6) = _5.fld0 - _1;
_8 = RET;
_5.fld0 = -(*_6);
_11 = _5.fld1;
_2.fld3 = [(-12482_i16),2409_i16,(-21849_i16)];
_4 = _1 + _1;
(*_6) = 1378829973_u32 as f32;
match _13.fld0 {
0 => bb8,
1 => bb9,
3 => bb11,
4 => bb12,
2 => bb14,
_ => bb13
}
}
bb8 = {
_4 = _5.fld0 * _1;
Call(_5.fld0 = fn16(Move(_6), _3, _3.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_11 = _2.fld1;
_2.fld2 = _5.fld2;
_2 = _3;
_2.fld0 = _5.fld0 - _5.fld0;
_11 = _8 as u8;
_2.fld2 = -_7;
_9 = false ^ true;
_3 = Adt35 { fld0: _5.fld0,fld1: _11,fld2: _7,fld3: _2.fld3 };
_5.fld0 = _3.fld0;
RET = _8;
_13.fld2 = [46760_u16,56516_u16,63655_u16,51262_u16,30284_u16];
_11 = _3.fld1 << _10;
_3.fld1 = _5.fld1;
RET = _8;
_13.fld0 = 2_usize;
_3.fld2 = _5.fld2 | _7;
_2.fld1 = _11 | _5.fld1;
_6 = core::ptr::addr_of_mut!(_3.fld0);
_2.fld3 = _3.fld3;
_3.fld0 = _10 as f32;
_3.fld0 = _2.fld1 as f32;
_2.fld1 = _5.fld1 * _11;
_5.fld1 = _2.fld1 | _2.fld1;
_13.fld2 = [8828_u16,16185_u16,22465_u16,9196_u16,56764_u16];
_2.fld1 = 46_i8 as u8;
_13.fld1 = 150577903_i32 - 1275264625_i32;
match _10 {
2319704968476361766 => bb7,
_ => bb6
}
}
bb10 = {
RET = _8;
_2 = Adt35 { fld0: (*_6),fld1: _3.fld1,fld2: _3.fld2,fld3: _5.fld3 };
_10 = 2319704968476361766_i64;
_2.fld3 = [1484_i16,(-28176_i16),(-8520_i16)];
_2.fld2 = !_5.fld2;
_5.fld0 = _2.fld0;
Call(_7 = fn18(_3.fld0, _5, _2.fld0, Move(_6), _5, (*_6), _3, _2, (*_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_5.fld2 = !_7;
Goto(bb4)
}
bb12 = {
_6 = core::ptr::addr_of_mut!(_1);
_3.fld1 = RET as u8;
_7 = _2.fld2;
_3.fld0 = (*_6) + (*_6);
_2.fld2 = _5.fld2 << _7;
_1 = 15_i8 as f32;
_8 = RET;
_4 = -_2.fld0;
_4 = (-49_i8) as f32;
_9 = !false;
_1 = _3.fld0;
_5 = _3;
_9 = true & true;
_5 = Adt35 { fld0: _2.fld0,fld1: _2.fld1,fld2: _2.fld2,fld3: _2.fld3 };
RET = _8;
Goto(bb3)
}
bb13 = {
_4 = _5.fld0 * _1;
Call(_5.fld0 = fn16(Move(_6), _3, _3.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_9 = false;
_2 = Adt35 { fld0: _4,fld1: _5.fld1,fld2: _3.fld2,fld3: _3.fld3 };
_13.fld2 = [23588_u16,3341_u16,50004_u16,42186_u16,58125_u16];
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(15_usize, 11_usize, Move(_11), 7_usize, Move(_7), 16_usize, _16, 16_usize, _16), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *mut f32,mut _2: Adt35,mut _3: f32) -> f32 {
mir! {
type RET = f32;
let _4: &'static (i32,);
let _5: isize;
let _6: isize;
let _7: usize;
let _8: *mut &'static i128;
let _9: u32;
let _10: u32;
let _11: *const u32;
let _12: Adt49;
let _13: (*const u128, u64, char, *const u128);
let _14: i32;
let _15: &'static (*mut &'static i128,);
let _16: (Adt35, char, usize, [isize; 4]);
let _17: [i8; 1];
let _18: (&'static i128, (f32,));
let _19: bool;
let _20: [u32; 4];
let _21: f32;
let _22: u64;
let _23: usize;
let _24: ((f32,), *const u64, char, u64);
let _25: usize;
let _26: &'static &'static &'static i128;
let _27: isize;
let _28: (Adt35, char, usize, [isize; 4]);
let _29: (&'static i128, (f32,));
let _30: bool;
let _31: (*mut &'static (i32,), *mut &'static (i32,));
let _32: isize;
let _33: isize;
let _34: f32;
let _35: f64;
let _36: bool;
let _37: [u8; 1];
let _38: ((f32,), *const u64, char, u64);
let _39: bool;
let _40: char;
let _41: ((*mut &'static i128,), *const isize);
let _42: f32;
let _43: u16;
let _44: bool;
let _45: isize;
let _46: f64;
let _47: char;
let _48: isize;
let _49: bool;
let _50: u16;
let _51: isize;
let _52: &'static &'static i128;
let _53: ((u8, [usize; 7]), *const u32);
let _54: &'static (*mut &'static i128,);
let _55: isize;
let _56: &'static char;
let _57: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _58: ();
let _59: ();
{
_1 = core::ptr::addr_of_mut!(_2.fld0);
(*_1) = _3 - _3;
_2.fld1 = 113_u8;
RET = 43916636086226027512160565898480840937_i128 as f32;
_5 = _2.fld2;
_2.fld0 = 131402972406406245274021809965963480999_i128 as f32;
_2.fld3 = [(-17595_i16),(-21959_i16),25539_i16];
_1 = core::ptr::addr_of_mut!(_3);
_6 = _2.fld2 << _2.fld2;
_1 = core::ptr::addr_of_mut!(_2.fld0);
_6 = _5;
_5 = '\u{bfb1a}' as isize;
(*_1) = -_3;
_6 = true as isize;
_2.fld1 = 1760960513_i32 as u8;
_1 = core::ptr::addr_of_mut!(_2.fld0);
(*_1) = 25618_u16 as f32;
_2.fld3 = [(-8578_i16),3681_i16,26399_i16];
_1 = core::ptr::addr_of_mut!(_3);
(*_1) = RET;
_6 = !_2.fld2;
_2.fld1 = 24192_u16 as u8;
RET = -_2.fld0;
Goto(bb1)
}
bb1 = {
_2.fld2 = _5;
_7 = 9234825643266010384_usize * 2470484607741117558_usize;
RET = -_2.fld0;
RET = (*_1);
RET = _3;
_9 = 48568_u16 as u32;
_2.fld0 = 288282527269778331634942335880353876408_u128 as f32;
_5 = !_6;
_5 = _6;
_3 = RET;
_2.fld0 = _3;
_10 = true as u32;
RET = (*_1) - _3;
RET = (*_1) * _3;
_10 = 42642843931601346782273490498230120727_i128 as u32;
RET = 192343912602017174402308402447810452294_u128 as f32;
(*_1) = -_2.fld0;
_2.fld1 = !172_u8;
_11 = core::ptr::addr_of!(_9);
Goto(bb2)
}
bb2 = {
_1 = core::ptr::addr_of_mut!(_2.fld0);
Goto(bb3)
}
bb3 = {
_13.1 = 2995975645398375678_u64 >> _6;
(*_1) = _3;
Goto(bb4)
}
bb4 = {
_16.2 = 109613201651944351966503292435938808298_u128 as usize;
_14 = (-1250371347_i32) << _13.1;
_14 = 517938667_i32 * 1692663833_i32;
_11 = core::ptr::addr_of!((*_11));
(*_11) = _7 as u32;
_18.1 = (RET,);
_16.2 = 329511907767383944302507942711138923729_u128 as usize;
_17 = [4_i8];
_2.fld3 = [(-10290_i16),6586_i16,246_i16];
(*_1) = RET;
_16.0.fld3 = [16725_i16,27406_i16,(-11594_i16)];
_13.2 = '\u{34ae5}';
_8 = core::ptr::addr_of_mut!(_18.0);
_20 = [(*_11),(*_11),_10,(*_11)];
_6 = 74850201289359674006389207882045190779_i128 as isize;
Goto(bb5)
}
bb5 = {
_16.0 = _2;
_19 = true ^ true;
_20 = [_9,(*_11),(*_11),(*_11)];
_13.1 = 12980211027371705990_u64 >> _7;
_13.2 = '\u{4daf8}';
_16.1 = _13.2;
_3 = RET;
_2 = _16.0;
_24.0.0 = RET;
_24.1 = core::ptr::addr_of!(_13.1);
_7 = !_16.2;
(*_1) = _16.0.fld0 + RET;
_16.3 = [_5,_5,_2.fld2,_6];
_16.0.fld1 = _2.fld1 >> _2.fld1;
_5 = _14 as isize;
_22 = !_13.1;
_1 = core::ptr::addr_of_mut!(_16.0.fld0);
_18.1.0 = RET;
_2 = Adt35 { fld0: (*_1),fld1: _16.0.fld1,fld2: _16.0.fld2,fld3: _16.0.fld3 };
_11 = core::ptr::addr_of!((*_11));
_17 = [(-39_i8)];
_13.2 = _16.1;
_23 = !_16.2;
_18.1 = _24.0;
_16.0 = Adt35 { fld0: _2.fld0,fld1: _2.fld1,fld2: _2.fld2,fld3: _2.fld3 };
_2.fld2 = 11045_i16 as isize;
_16.3 = [_6,_5,_6,_2.fld2];
Goto(bb6)
}
bb6 = {
_23 = _7;
_21 = _18.1.0;
_24.2 = _13.2;
_2.fld1 = _16.0.fld1;
_2.fld1 = _16.0.fld1 - _16.0.fld1;
(*_11) = _10 * _10;
_2 = Adt35 { fld0: RET,fld1: _16.0.fld1,fld2: _5,fld3: _16.0.fld3 };
_9 = _10;
_20 = [(*_11),(*_11),_10,_10];
RET = -_3;
_16.0.fld0 = _3 + _24.0.0;
(*_11) = _19 as u32;
_28.0.fld3 = [(-2118_i16),24817_i16,(-2484_i16)];
_28.0.fld1 = _2.fld1;
_30 = _14 >= _14;
_17 = [(-106_i8)];
_24.3 = _14 as u64;
_11 = core::ptr::addr_of!((*_11));
_16.1 = _13.2;
_31.1 = core::ptr::addr_of_mut!(_4);
Call(_2.fld2 = core::intrinsics::transmute(_23), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_24.0 = (RET,);
_28.0.fld0 = -_21;
_31.1 = core::ptr::addr_of_mut!(_4);
_10 = !(*_11);
_31.0 = core::ptr::addr_of_mut!(_4);
_24.1 = core::ptr::addr_of!(_24.3);
_35 = _10 as f64;
_28.0.fld2 = (-45_i8) as isize;
_16.2 = _23 - _7;
_31.1 = core::ptr::addr_of_mut!(_4);
_13.2 = _24.2;
_28 = (_2, _16.1, _7, _16.3);
_30 = !_19;
_16.0.fld1 = !_28.0.fld1;
_13.1 = _24.3;
_18.1 = ((*_1),);
_31.0 = core::ptr::addr_of_mut!(_4);
Goto(bb8)
}
bb8 = {
_35 = 60047_u16 as f64;
_29.1.0 = _16.0.fld0;
_16.0.fld2 = -_6;
_31.0 = core::ptr::addr_of_mut!(_4);
(*_1) = RET + _28.0.fld0;
_5 = _2.fld2;
_28.3 = [_6,_6,_5,_2.fld2];
_41.1 = core::ptr::addr_of!(_2.fld2);
_24.0 = _18.1;
_2.fld2 = _16.2 as isize;
_41.0.0 = core::ptr::addr_of_mut!(_29.0);
_33 = _2.fld2;
_22 = _13.1;
_28.0.fld1 = _2.fld1;
_28 = _16;
_2.fld3 = [6255_i16,(-30924_i16),(-29717_i16)];
Goto(bb9)
}
bb9 = {
_9 = _2.fld2 as u32;
_5 = _28.0.fld2 >> _14;
_39 = _30;
_25 = _35 as usize;
_38.3 = !_24.3;
_16.0.fld2 = !_28.0.fld2;
_38.3 = _24.3;
_38.1 = Move(_24.1);
_24.0.0 = _16.0.fld2 as f32;
_18.1.0 = _35 as f32;
_25 = 28693_i16 as usize;
Call(_2.fld1 = fn17(Move(_41), _13.2, _16.0.fld3, (*_11)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = [4_i8];
_29.1 = ((*_1),);
_15 = &_41.0;
_29.1.0 = -(*_1);
_41.1 = core::ptr::addr_of!(_28.0.fld2);
_16.0 = Adt35 { fld0: _18.1.0,fld1: _28.0.fld1,fld2: _33,fld3: _28.0.fld3 };
_27 = _7 as isize;
_28.0.fld3 = [20864_i16,(-16407_i16),(-14224_i16)];
_45 = _28.0.fld1 as isize;
_31.0 = core::ptr::addr_of_mut!(_4);
_27 = !_2.fld2;
_24 = (_29.1, Move(_38.1), _13.2, _38.3);
_29.1 = (_3,);
_34 = RET;
_24.1 = core::ptr::addr_of!(_38.3);
_42 = _34 - (*_1);
Goto(bb11)
}
bb11 = {
_2.fld0 = _25 as f32;
_29.1 = _18.1;
_8 = core::ptr::addr_of_mut!((*_8));
_38.2 = _16.1;
_18.1.0 = -_3;
_13.2 = _16.1;
_38.0.0 = -_18.1.0;
_11 = core::ptr::addr_of!((*_11));
_13.2 = _38.2;
_2.fld0 = 7689_i16 as f32;
RET = -_28.0.fld0;
RET = -_16.0.fld0;
_36 = _18.1.0 <= _2.fld0;
_33 = _36 as isize;
_37 = [_16.0.fld1];
_38.1 = Move(_24.1);
Goto(bb12)
}
bb12 = {
_29.1 = ((*_1),);
_16.2 = _28.2;
_28.0 = _16.0;
_43 = _13.2 as u16;
_51 = !_2.fld2;
_52 = &(*_8);
_40 = _28.1;
_24.2 = _28.1;
_28.0.fld3 = [1745_i16,22103_i16,(-13314_i16)];
Goto(bb13)
}
bb13 = {
_1 = core::ptr::addr_of_mut!(_29.1.0);
_45 = !_2.fld2;
RET = -_28.0.fld0;
_31.0 = core::ptr::addr_of_mut!(_4);
RET = _34 - _24.0.0;
_50 = _43;
_32 = -_5;
_26 = &_52;
_26 = &(*_26);
_24.0.0 = _14 as f32;
_8 = core::ptr::addr_of_mut!((*_52));
_28.0.fld2 = !_5;
_28.1 = _40;
_9 = _10 - _10;
_33 = !_28.0.fld2;
_46 = -_35;
_44 = _2.fld1 >= _16.0.fld1;
_53.0.0 = _28.0.fld1 - _16.0.fld1;
_7 = _28.2 - _25;
_19 = _45 != _51;
_28.0.fld3 = _16.0.fld3;
Goto(bb14)
}
bb14 = {
_24.1 = core::ptr::addr_of!(_13.1);
_47 = _16.1;
_14 = -(-2102196182_i32);
_7 = _28.2 | _28.2;
_18.1.0 = _24.0.0 + (*_1);
_21 = _42;
_38.2 = _28.1;
_16 = _28;
_27 = _28.0.fld2;
RET = _24.0.0;
_55 = _32;
_22 = _24.3 - _13.1;
_32 = _5;
_54 = &_41.0;
_28.2 = _9 as usize;
_31.1 = core::ptr::addr_of_mut!(_4);
_24.0 = _29.1;
_17 = [104_i8];
_33 = !_28.0.fld2;
_16.0.fld0 = _9 as f32;
_2.fld3 = _16.0.fld3;
_16.2 = (*_11) as usize;
_53.0.0 = _28.0.fld1;
_19 = _44;
_2.fld2 = 130293601432684078791806935639968578015_u128 as isize;
_2.fld2 = -_27;
_44 = _39;
_27 = (-293516828487911200_i64) as isize;
Goto(bb15)
}
bb15 = {
Call(_58 = dump_var(16_usize, 5_usize, Move(_5), 40_usize, Move(_40), 30_usize, Move(_30), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_58 = dump_var(16_usize, 37_usize, Move(_37), 33_usize, Move(_33), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_58 = dump_var(16_usize, 47_usize, Move(_47), 19_usize, Move(_19), 43_usize, Move(_43), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_58 = dump_var(16_usize, 20_usize, Move(_20), 59_usize, _59, 59_usize, _59, 59_usize, _59), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: ((*mut &'static i128,), *const isize),mut _2: char,mut _3: [i16; 3],mut _4: u32) -> u8 {
mir! {
type RET = u8;
let _5: (u64,);
let _6: [u16; 5];
let _7: bool;
let _8: isize;
let _9: i32;
let _10: u32;
let _11: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _12: f32;
let _13: isize;
let _14: f64;
let _15: f64;
let _16: isize;
let _17: char;
let _18: f32;
let _19: Adt52;
let _20: u32;
let _21: i16;
let _22: Adt52;
let _23: isize;
let _24: Adt48;
let _25: char;
let _26: &'static bool;
let _27: &'static &'static i128;
let _28: [u8; 1];
let _29: f32;
let _30: Adt80;
let _31: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _32: u16;
let _33: *mut ((i32,), usize);
let _34: Adt63;
let _35: f64;
let _36: f64;
let _37: (bool, Adt52, [u8; 1], f64);
let _38: f64;
let _39: Adt73;
let _40: ();
let _41: ();
{
RET = 211_u8 + 45_u8;
_4 = 1256515321_u32 >> RET;
_3 = [19038_i16,32074_i16,(-30175_i16)];
RET = _2 as u8;
_4 = 3941_i16 as u32;
_5 = (4974111114297179875_u64,);
_2 = '\u{da64c}';
RET = !232_u8;
_5.0 = 12902880896969123754_u64;
RET = 188_u8 ^ 121_u8;
RET = 233_u8;
_4 = 9223372036854775807_isize as u32;
_4 = _2 as u32;
_6 = [26814_u16,21684_u16,61720_u16,65150_u16,39360_u16];
_3 = [(-25857_i16),(-22263_i16),9108_i16];
_6 = [30894_u16,10419_u16,52266_u16,64984_u16,10220_u16];
_5.0 = !9873197278972822876_u64;
_6 = [23426_u16,31575_u16,13300_u16,33074_u16,29093_u16];
_3 = [703_i16,(-30117_i16),(-4779_i16)];
_3 = [(-3128_i16),(-8492_i16),(-20369_i16)];
_8 = (-62_isize);
_1.1 = core::ptr::addr_of!(_8);
_8 = -(-9223372036854775808_isize);
_4 = (-4418210526664440112_i64) as u32;
_1.1 = core::ptr::addr_of!(_8);
_1.1 = core::ptr::addr_of!(_8);
_8 = !(-62_isize);
Goto(bb1)
}
bb1 = {
RET = 60_u8;
_9 = 131243435882391741941174443229986940353_u128 as i32;
_5 = (11802984033246128968_u64,);
_8 = _9 as isize;
RET = 89_u8 + 234_u8;
_7 = true;
Goto(bb2)
}
bb2 = {
RET = 59_u8 ^ 9_u8;
_5 = (7356792878344053415_u64,);
RET = !83_u8;
_13 = -_8;
_3 = [(-25868_i16),30913_i16,(-26011_i16)];
_2 = '\u{6e58c}';
_12 = _9 as f32;
_8 = _13 + _13;
_13 = !_8;
_11.0 = core::ptr::addr_of!(_4);
_10 = _4 & _4;
Goto(bb3)
}
bb3 = {
_7 = true | true;
_9 = -362499019_i32;
_5.0 = 3_usize as u64;
_2 = '\u{c95c2}';
_5 = (18318668850760152388_u64,);
_11.0 = core::ptr::addr_of!(_4);
_6 = [40143_u16,59820_u16,41599_u16,14899_u16,51199_u16];
_8 = _13;
_2 = '\u{6a179}';
RET = 178_u8 >> _8;
_13 = _8;
_13 = _8 << _8;
_9 = !1291309644_i32;
Goto(bb4)
}
bb4 = {
_11.0 = core::ptr::addr_of!(_10);
_2 = '\u{103d7b}';
_9 = (-635256602_i32);
_6 = [24617_u16,48240_u16,2291_u16,56255_u16,49678_u16];
_13 = (-14464778614645465201617138092280112500_i128) as isize;
_12 = 4_usize as f32;
_5 = (1765442221675106591_u64,);
RET = !31_u8;
_11.0 = core::ptr::addr_of!(_10);
_13 = _7 as isize;
RET = 161_u8;
RET = !210_u8;
_9 = (-151480697_i32);
_18 = _12;
_5 = (9160483980064157900_u64,);
_18 = -_12;
_16 = _8 >> _9;
Goto(bb5)
}
bb5 = {
_4 = !_10;
_14 = 43667_u16 as f64;
_17 = _2;
_5.0 = 13161730005775266165_u64 + 236060078478461403_u64;
_7 = _16 != _13;
_19 = Adt52 { fld0: 6_usize,fld1: _9,fld2: _6 };
_19.fld1 = _9 << _4;
_5 = (529632622131420243_u64,);
RET = !13_u8;
_5.0 = 7419854558487246656_u64;
_2 = _17;
_15 = _18 as f64;
Goto(bb6)
}
bb6 = {
_22.fld1 = _19.fld1 * _19.fld1;
_22.fld2 = [65114_u16,28295_u16,18385_u16,9201_u16,7662_u16];
_7 = true;
_7 = true;
_21 = !(-15840_i16);
_1.1 = core::ptr::addr_of!(_13);
_17 = _2;
RET = !158_u8;
_21 = !14190_i16;
match _19.fld0 {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb8,
6 => bb10,
_ => bb9
}
}
bb7 = {
_4 = !_10;
_14 = 43667_u16 as f64;
_17 = _2;
_5.0 = 13161730005775266165_u64 + 236060078478461403_u64;
_7 = _16 != _13;
_19 = Adt52 { fld0: 6_usize,fld1: _9,fld2: _6 };
_19.fld1 = _9 << _4;
_5 = (529632622131420243_u64,);
RET = !13_u8;
_5.0 = 7419854558487246656_u64;
_2 = _17;
_15 = _18 as f64;
Goto(bb6)
}
bb8 = {
RET = 59_u8 ^ 9_u8;
_5 = (7356792878344053415_u64,);
RET = !83_u8;
_13 = -_8;
_3 = [(-25868_i16),30913_i16,(-26011_i16)];
_2 = '\u{6e58c}';
_12 = _9 as f32;
_8 = _13 + _13;
_13 = !_8;
_11.0 = core::ptr::addr_of!(_4);
_10 = _4 & _4;
Goto(bb3)
}
bb9 = {
_7 = true | true;
_9 = -362499019_i32;
_5.0 = 3_usize as u64;
_2 = '\u{c95c2}';
_5 = (18318668850760152388_u64,);
_11.0 = core::ptr::addr_of!(_4);
_6 = [40143_u16,59820_u16,41599_u16,14899_u16,51199_u16];
_8 = _13;
_2 = '\u{6a179}';
RET = 178_u8 >> _8;
_13 = _8;
_13 = _8 << _8;
_9 = !1291309644_i32;
Goto(bb4)
}
bb10 = {
_24.fld1 = _17;
Goto(bb11)
}
bb11 = {
_15 = -_14;
_24.fld0 = !_10;
_4 = _24.fld0 * _24.fld0;
_24.fld4.1 = core::ptr::addr_of!(_5.0);
_19.fld2 = [32591_u16,38826_u16,2612_u16,29114_u16,53594_u16];
_17 = _2;
_24.fld0 = (-61196976501384485693058927449078920536_i128) as u32;
_14 = _18 as f64;
_24.fld4.3 = _14 as u64;
_24.fld5 = [RET];
_23 = (-8250164878184053331_i64) as isize;
_10 = (-105_i8) as u32;
_24.fld4.0 = (_18,);
_24.fld4.0 = (_12,);
_18 = _24.fld4.0.0;
_26 = &_7;
_23 = _8 ^ _13;
_5 = (_24.fld4.3,);
_2 = _24.fld1;
_8 = _23 | _13;
_5.0 = _21 as u64;
_25 = _17;
_16 = _8 & _8;
_19 = Adt52 { fld0: 6_usize,fld1: _9,fld2: _22.fld2 };
_22.fld0 = 41795156898305279878191866877334056435_i128 as usize;
_24.fld4.1 = core::ptr::addr_of!(_5.0);
Call(_24.fld4.3 = core::intrinsics::transmute(_16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22.fld1 = !_9;
_24.fld1 = _17;
_4 = _10 * _10;
_14 = _15 * _15;
_25 = _2;
_9 = _22.fld1 << _19.fld0;
_24.fld4.2 = _25;
_19.fld2 = [63977_u16,34294_u16,55598_u16,52005_u16,32291_u16];
_15 = -_14;
_22.fld2 = [18106_u16,62679_u16,16453_u16,61340_u16,39280_u16];
_11.0 = core::ptr::addr_of!(_20);
_22.fld2 = _6;
_13 = _8;
_4 = _24.fld0 | _10;
_5.0 = _24.fld4.3 & _24.fld4.3;
_8 = _24.fld1 as isize;
_7 = !true;
_24.fld3 = [_19.fld0,_22.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0];
_22.fld0 = !_19.fld0;
_18 = _24.fld4.0.0 * _12;
_10 = _4;
_28 = [RET];
_10 = _24.fld0;
_19 = Adt52 { fld0: _22.fld0,fld1: _9,fld2: _6 };
_26 = &_7;
Goto(bb13)
}
bb13 = {
_26 = &_7;
_24.fld4.2 = _2;
_14 = _12 as f64;
_14 = _15 + _15;
_22.fld2 = [2386_u16,23870_u16,39250_u16,47461_u16,34197_u16];
_24.fld4.0.0 = -_18;
_19.fld1 = -_22.fld1;
_24.fld4.1 = core::ptr::addr_of!(_5.0);
_14 = _15 - _15;
_3 = [_21,_21,_21];
_19.fld0 = !_22.fld0;
_7 = !true;
_24.fld3 = [_22.fld0,_22.fld0,_22.fld0,_19.fld0,_22.fld0,_19.fld0,_19.fld0];
_24.fld4.0 = (_18,);
_26 = &_7;
_29 = _24.fld4.0.0 * _24.fld4.0.0;
_21 = 7482_i16 >> _24.fld4.3;
_4 = _24.fld0 ^ _24.fld0;
_30.fld3 = [RET];
_4 = _24.fld0 | _10;
RET = !132_u8;
_19.fld1 = _9 + _9;
_19.fld0 = !_22.fld0;
_19.fld1 = _9;
Goto(bb14)
}
bb14 = {
_24.fld0 = _4;
_31.1 = &_32;
_19 = _22;
_22 = _19;
_24.fld4.0.0 = -_29;
_24.fld5 = [RET];
_31.1 = &_32;
_15 = (-545405510833759480_i64) as f64;
_22.fld0 = !_19.fld0;
_11.1 = &_32;
_24.fld4.3 = !_5.0;
_19.fld2 = [57019_u16,11294_u16,35284_u16,8742_u16,2137_u16];
_19 = Adt52 { fld0: _22.fld0,fld1: _9,fld2: _22.fld2 };
_19.fld0 = _22.fld0;
_30.fld1 = (-137604022601844221130639068010303102020_i128) as u128;
_10 = !_24.fld0;
_17 = _24.fld1;
_30.fld4 = _17 as i128;
_31.1 = &_32;
RET = _30.fld4 as u8;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(17_usize, 13_usize, Move(_13), 5_usize, Move(_5), 2_usize, Move(_2), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(17_usize, 4_usize, Move(_4), 16_usize, Move(_16), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: f32,mut _2: Adt35,mut _3: f32,mut _4: *mut f32,mut _5: Adt35,mut _6: f32,mut _7: Adt35,mut _8: Adt35,mut _9: f32) -> isize {
mir! {
type RET = isize;
let _10: (*const u128, u64, char, *const u128);
let _11: char;
let _12: *mut &'static (i32,);
let _13: isize;
let _14: ([i8; 1], ((&'static i128,), f64, u16));
let _15: [i8; 1];
let _16: u16;
let _17: &'static (Adt35, char, usize, [isize; 4]);
let _18: &'static (Adt35, char, usize, [isize; 4]);
let _19: u32;
let _20: f32;
let _21: [i16; 3];
let _22: ((i32,), usize);
let _23: &'static f32;
let _24: char;
let _25: char;
let _26: (*const u32, &'static u16, (*mut &'static i128,), &'static [i128; 3]);
let _27: (i32,);
let _28: [i8; 1];
let _29: [u32; 3];
let _30: &'static bool;
let _31: ();
let _32: ();
{
_5.fld2 = _2.fld2;
RET = -_2.fld2;
_7.fld0 = _6 * _2.fld0;
_3 = _2.fld0 - _5.fld0;
_2.fld1 = _7.fld1;
_7.fld0 = 134152742361096929559506699126660666328_i128 as f32;
_2 = Adt35 { fld0: _1,fld1: _8.fld1,fld2: RET,fld3: _7.fld3 };
_1 = -_3;
_2.fld3 = [(-20467_i16),(-15885_i16),(-18770_i16)];
_2.fld2 = _5.fld2;
_2.fld1 = 30928_i16 as u8;
_8.fld0 = _3 * _6;
_6 = -_8.fld0;
_10.2 = '\u{40fcf}';
_2.fld1 = _5.fld1 + _7.fld1;
_10.2 = '\u{80b6d}';
_9 = 1364475810_u32 as f32;
_8.fld2 = !_2.fld2;
Call(_7.fld1 = core::intrinsics::transmute(_2.fld1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5.fld0 = -_1;
_5.fld3 = [13340_i16,3011_i16,(-23328_i16)];
_7.fld2 = !RET;
_3 = _6 + _6;
_8.fld0 = _3;
_7.fld3 = [(-3288_i16),(-25722_i16),(-20689_i16)];
_8 = Adt35 { fld0: _6,fld1: _2.fld1,fld2: RET,fld3: _5.fld3 };
Call(_8.fld2 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_5.fld2;
_3 = -_2.fld0;
_2.fld3 = _7.fld3;
_11 = _10.2;
_4 = core::ptr::addr_of_mut!(_9);
(*_4) = 5917787454422219577_u64 as f32;
_13 = _2.fld2 + _2.fld2;
_14.1.1 = _8.fld0 as f64;
_4 = core::ptr::addr_of_mut!(_2.fld0);
_14.1.2 = 700276221_i32 as u16;
_9 = _8.fld0 * _6;
_5.fld1 = _2.fld1 ^ _7.fld1;
_14.1.1 = 110401057_u32 as f64;
_2.fld2 = 1897321314_u32 as isize;
_8.fld1 = 2635850431900024930_u64 as u8;
Call(_10.1 = core::intrinsics::bswap(11062962572807734866_u64), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _2;
_5.fld0 = _14.1.2 as f32;
_16 = _2.fld0 as u16;
_7 = Adt35 { fld0: _8.fld0,fld1: _5.fld1,fld2: _5.fld2,fld3: _2.fld3 };
_9 = _1 + _1;
_8.fld2 = RET;
_7.fld1 = !_2.fld1;
_7.fld1 = !_5.fld1;
_2.fld2 = _5.fld2 - _5.fld2;
_9 = (-11_i8) as f32;
_6 = -_8.fld0;
_11 = _10.2;
_20 = _1;
_10.2 = _11;
_5.fld1 = _7.fld1 >> _2.fld2;
_19 = 3455210167_u32;
match _19 {
0 => bb2,
1 => bb4,
3455210167 => bb6,
_ => bb5
}
}
bb4 = {
RET = !_5.fld2;
_3 = -_2.fld0;
_2.fld3 = _7.fld3;
_11 = _10.2;
_4 = core::ptr::addr_of_mut!(_9);
(*_4) = 5917787454422219577_u64 as f32;
_13 = _2.fld2 + _2.fld2;
_14.1.1 = _8.fld0 as f64;
_4 = core::ptr::addr_of_mut!(_2.fld0);
_14.1.2 = 700276221_i32 as u16;
_9 = _8.fld0 * _6;
_5.fld1 = _2.fld1 ^ _7.fld1;
_14.1.1 = 110401057_u32 as f64;
_2.fld2 = 1897321314_u32 as isize;
_8.fld1 = 2635850431900024930_u64 as u8;
Call(_10.1 = core::intrinsics::bswap(11062962572807734866_u64), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5.fld0 = -_1;
_5.fld3 = [13340_i16,3011_i16,(-23328_i16)];
_7.fld2 = !RET;
_3 = _6 + _6;
_8.fld0 = _3;
_7.fld3 = [(-3288_i16),(-25722_i16),(-20689_i16)];
_8 = Adt35 { fld0: _6,fld1: _2.fld1,fld2: RET,fld3: _5.fld3 };
Call(_8.fld2 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_24 = _11;
_14.1.2 = _16;
_8 = Adt35 { fld0: _6,fld1: _5.fld1,fld2: RET,fld3: _7.fld3 };
_5.fld2 = !_7.fld2;
_23 = &(*_4);
_24 = _11;
_10.1 = _19 as u64;
_8.fld3 = [(-8257_i16),(-13621_i16),10041_i16];
match _19 {
0 => bb1,
1 => bb5,
2 => bb3,
3455210167 => bb7,
_ => bb4
}
}
bb7 = {
_26.1 = &_16;
_22.0 = ((-616413436_i32),);
_5.fld2 = -_2.fld2;
_8.fld0 = 63566760190180254807362574950118217706_u128 as f32;
RET = _2.fld2;
_7.fld3 = [(-14228_i16),12426_i16,(-5175_i16)];
Goto(bb8)
}
bb8 = {
Call(_31 = dump_var(18_usize, 11_usize, Move(_11), 16_usize, Move(_16), 32_usize, _32, 32_usize, _32), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: Adt35,mut _2: f32,mut _3: *const isize,mut _4: f32,mut _5: f32,mut _6: Adt35,mut _7: (f32,),mut _8: *mut f32,mut _9: f32) -> i32 {
mir! {
type RET = i32;
let _10: [u32; 4];
let _11: *const u32;
let _12: (*mut &'static i128,);
let _13: u16;
let _14: i8;
let _15: u16;
let _16: isize;
let _17: *const u16;
let _18: i16;
let _19: &'static [i128; 3];
let _20: &'static [i128; 3];
let _21: i128;
let _22: bool;
let _23: i128;
let _24: [u32; 3];
let _25: isize;
let _26: u8;
let _27: (u64,);
let _28: [u32; 3];
let _29: &'static bool;
let _30: ((*mut &'static i128,), *const isize);
let _31: bool;
let _32: [u32; 3];
let _33: [u16; 5];
let _34: u128;
let _35: char;
let _36: ([i8; 1], ((&'static i128,), f64, u16));
let _37: ();
let _38: ();
{
_5 = 3440084805_u32 as f32;
_8 = core::ptr::addr_of_mut!(_5);
_2 = -_7.0;
_6.fld0 = _2;
_6.fld2 = _6.fld1 as isize;
_1.fld3 = [25932_i16,(-489_i16),6906_i16];
RET = 1774949315_i32;
_4 = -_6.fld0;
_1.fld1 = _6.fld1 * _6.fld1;
_1.fld1 = !_6.fld1;
_2 = _6.fld0;
_3 = core::ptr::addr_of!(_6.fld2);
_2 = 2514493366872013930_usize as f32;
_6.fld3 = _1.fld3;
_4 = _6.fld0;
_1.fld2 = _6.fld2 - (*_3);
RET = _1.fld1 as i32;
(*_3) = _1.fld2;
_7 = (_1.fld0,);
_7 = (_4,);
(*_3) = _1.fld2;
(*_8) = RET as f32;
Goto(bb1)
}
bb1 = {
_1.fld3 = _6.fld3;
_7 = (_9,);
_1.fld0 = _4 * _9;
_1.fld1 = _6.fld1;
_1.fld2 = _6.fld2 + _6.fld2;
RET = 10541532049300205218_u64 as i32;
_4 = _7.0;
_6.fld2 = !_1.fld2;
_1.fld1 = !_6.fld1;
RET = 197332783_i32 ^ 1181018719_i32;
(*_8) = -_7.0;
(*_8) = 9313663184948218298_u64 as f32;
_1.fld3 = [18411_i16,(-23139_i16),(-1142_i16)];
(*_8) = _4;
_7 = ((*_8),);
_5 = _4 + _4;
_6 = Adt35 { fld0: _7.0,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
(*_3) = !_1.fld2;
(*_8) = _6.fld2 as f32;
(*_8) = _6.fld1 as f32;
_3 = core::ptr::addr_of!((*_3));
_6.fld1 = 21849_u16 as u8;
_6 = Adt35 { fld0: _9,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
_7.0 = _6.fld0;
(*_8) = _4 + _7.0;
_15 = RET as u16;
Goto(bb2)
}
bb2 = {
_3 = core::ptr::addr_of!(_6.fld2);
_6.fld2 = 163986386102051968207280662186438842460_i128 as isize;
_6 = Adt35 { fld0: _5,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
_6.fld0 = _4;
_13 = _15 & _15;
(*_8) = 3931650045_u32 as f32;
_15 = !_13;
_1.fld3 = [23200_i16,(-1362_i16),20174_i16];
_2 = -_6.fld0;
_14 = 19_i8;
_16 = _1.fld1 as isize;
_17 = core::ptr::addr_of!(_15);
_9 = _6.fld0;
_7.0 = -_9;
_6.fld1 = _1.fld1;
_13 = 1_usize as u16;
_1.fld3 = [2456_i16,14411_i16,15633_i16];
_17 = core::ptr::addr_of!(_13);
_8 = core::ptr::addr_of_mut!(_1.fld0);
match _14 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
19 => bb11,
_ => bb10
}
}
bb3 = {
_1.fld3 = _6.fld3;
_7 = (_9,);
_1.fld0 = _4 * _9;
_1.fld1 = _6.fld1;
_1.fld2 = _6.fld2 + _6.fld2;
RET = 10541532049300205218_u64 as i32;
_4 = _7.0;
_6.fld2 = !_1.fld2;
_1.fld1 = !_6.fld1;
RET = 197332783_i32 ^ 1181018719_i32;
(*_8) = -_7.0;
(*_8) = 9313663184948218298_u64 as f32;
_1.fld3 = [18411_i16,(-23139_i16),(-1142_i16)];
(*_8) = _4;
_7 = ((*_8),);
_5 = _4 + _4;
_6 = Adt35 { fld0: _7.0,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
(*_3) = !_1.fld2;
(*_8) = _6.fld2 as f32;
(*_8) = _6.fld1 as f32;
_3 = core::ptr::addr_of!((*_3));
_6.fld1 = 21849_u16 as u8;
_6 = Adt35 { fld0: _9,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
_7.0 = _6.fld0;
(*_8) = _4 + _7.0;
_15 = RET as u16;
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
Return()
}
bb11 = {
_1.fld3 = [(-1336_i16),(-15813_i16),(-6668_i16)];
_10 = [1484244300_u32,1814148479_u32,207337247_u32,1574746063_u32];
_14 = (-32_i8) >> _6.fld2;
_23 = RET as i128;
_6.fld3 = [(-19633_i16),(-10870_i16),(-4341_i16)];
_25 = _6.fld2;
_6.fld0 = -_1.fld0;
_7.0 = 9796289198376426575_u64 as f32;
_7.0 = 7782783955267671479_i64 as f32;
_23 = 3227533986322727091071486538350688035_i128 * 33672729535979883161152142098478210913_i128;
Goto(bb12)
}
bb12 = {
(*_8) = _2 - _9;
_4 = (*_8) + (*_8);
_25 = _6.fld2 ^ (*_3);
_6.fld1 = _23 as u8;
_27 = (3402023585041318674_u64,);
(*_8) = -_4;
_1.fld0 = _9;
(*_3) = _25 >> _15;
_2 = (*_8) * (*_8);
_24 = [2847684676_u32,4081124238_u32,2975277196_u32];
_14 = 1626705747104303580_i64 as i8;
_1 = Adt35 { fld0: _2,fld1: _6.fld1,fld2: (*_3),fld3: _6.fld3 };
(*_17) = _15;
_1.fld0 = _14 as f32;
_1.fld3 = [(-27042_i16),(-20141_i16),(-12321_i16)];
_6 = _1;
_1.fld2 = _6.fld2;
Goto(bb13)
}
bb13 = {
_9 = _4 * _2;
_9 = -_4;
(*_8) = _27.0 as f32;
_24 = [2461115676_u32,2815278422_u32,556167136_u32];
_31 = false;
_13 = _15;
(*_8) = 9793_i16 as f32;
_1.fld0 = _4 - _2;
_33 = [(*_17),(*_17),_13,_13,_15];
_6.fld3 = [26076_i16,(-14797_i16),28325_i16];
_4 = _13 as f32;
_7.0 = -_2;
(*_3) = -_16;
_28 = [2016691447_u32,2643412971_u32,3107160002_u32];
_15 = (*_17) & (*_17);
_6.fld1 = !_1.fld1;
_17 = core::ptr::addr_of!(_15);
_7.0 = (-3322_i16) as f32;
_6.fld0 = -(*_8);
Goto(bb14)
}
bb14 = {
_34 = !339787465911804265680999848866625062212_u128;
_18 = (-1122_i16);
_34 = !171054081209101585210004142629079572566_u128;
_5 = _2 + _2;
(*_8) = -_5;
_30.0.0 = core::ptr::addr_of_mut!(_36.1.0.0);
_7.0 = -_5;
_14 = -(-114_i8);
_7 = (_5,);
_17 = core::ptr::addr_of!(_15);
_1.fld2 = -_6.fld2;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(19_usize, 25_usize, Move(_25), 10_usize, Move(_10), 33_usize, Move(_33), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(19_usize, 15_usize, Move(_15), 23_usize, Move(_23), 34_usize, Move(_34), 38_usize, _38), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
            }
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt35{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt35 {
fld0: f32,
fld1: u8,
fld2: isize,
fld3: [i16; 3],
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: u32,
fld1: char,
fld2: [i8; 1],
fld3: [usize; 7],
fld4: ((f32,), *const u64, char, u64),
fld5: [u8; 1],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u32,
fld1: [i8; 1],
fld2: [u8; 1],
fld3: (u8, [usize; 7]),
fld4: i16,
fld5: f32,
fld6: [isize; 4],

},
Variant1{
fld0: (u8, [usize; 7]),
fld1: i64,
fld2: Adt35,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: usize,
fld1: i32,
fld2: [u16; 5],
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: [u8; 1],
fld1: u8,
fld2: *const isize,
fld3: (char, u128, bool),
fld4: (f32,),
fld5: i32,
fld6: *const u128,

},
Variant1{
fld0: u128,
fld1: *const isize,
fld2: ((i32,), usize),
fld3: i8,
fld4: Adt49,
fld5: u32,
fld6: *const u64,

},
Variant2{
fld0: i64,
fld1: *const u64,
fld2: [i8; 1],
fld3: [isize; 4],
fld4: (f32,),
fld5: *const isize,

},
Variant3{
fld0: *const u32,
fld1: i32,

}}
impl PrintFDebug for Adt67{
	unsafe fn printf_debug(&self){unsafe{printf("Adt67::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt67 {
Variant0{
fld0: u16,
fld1: [u32; 4],
fld2: (u64,),

},
Variant1{
fld0: (bool, Adt52, [u8; 1], f64),
fld1: char,
fld2: [isize; 7],
fld3: f64,
fld4: Adt52,
fld5: usize,
fld6: u64,
fld7: (u64,),

},
Variant2{
fld0: [i16; 3],
fld1: u64,
fld2: f32,
fld3: i64,
fld4: *const u128,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: bool,
fld1: [usize; 5],
fld2: u16,

},
Variant1{
fld0: (i32,),

},
Variant2{
fld0: [u64; 1],

},
Variant3{
fld0: bool,
fld1: Adt48,
fld2: isize,
fld3: (f32,),
fld4: i16,
fld5: (char, u128, bool),
fld6: i64,
fld7: *const isize,

}}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt80{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt80 {
fld0: bool,
fld1: u128,
fld2: [isize; 4],
fld3: [u8; 1],
fld4: i128,
}

