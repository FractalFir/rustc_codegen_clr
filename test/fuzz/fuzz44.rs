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
pub fn fn0(mut _1: i64) -> (char,) {
mir! {
type RET = (char,);
let _2: bool;
let _3: f32;
let _4: *const i64;
let _5: *mut isize;
let _6: isize;
let _7: [usize; 1];
let _8: u64;
let _9: isize;
let _10: (char,);
let _11: Adt42;
let _12: *const Adt31;
let _13: [isize; 8];
let _14: *const *const (*const i64,);
let _15: &'static Adt42;
let _16: &'static *const isize;
let _17: bool;
let _18: i8;
let _19: &'static (Adt39, [isize; 8]);
let _20: char;
let _21: ((&'static Adt39,), *const *mut isize);
let _22: *const Adt22;
let _23: [u128; 2];
let _24: [u128; 2];
let _25: ((u8,), (u8, Adt22, f64));
let _26: Adt22;
let _27: i16;
let _28: Adt39;
let _29: ((u8,), (u8, Adt22, f64));
let _30: [usize; 8];
let _31: ();
let _32: ();
{
RET = ('\u{54445}',);
Call(RET = fn1(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = ('\u{639}',);
_1 = true as i64;
RET = ('\u{bc2f6}',);
RET = ('\u{e8e94}',);
RET = ('\u{77a0a}',);
RET.0 = '\u{ef5a7}';
RET.0 = '\u{7ef98}';
RET.0 = '\u{5cd67}';
RET = ('\u{4df2b}',);
_1 = 4743010624498770347_i64;
RET.0 = '\u{8cb5c}';
_1 = (-4646769039308761968_i64);
_2 = false;
RET = ('\u{19851}',);
_2 = !false;
_1 = !(-6328116667828562041_i64);
RET = ('\u{6a505}',);
_1 = (-6073762660182210973_i64);
RET.0 = '\u{4f985}';
RET.0 = '\u{ca4bf}';
Goto(bb2)
}
bb2 = {
RET = ('\u{859fc}',);
_3 = 62208_u16 as f32;
RET.0 = '\u{f8af}';
RET = ('\u{40625}',);
RET.0 = '\u{9c8cc}';
RET.0 = '\u{9a21e}';
_3 = 6_usize as f32;
Call(RET = fn2(_3, _1, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = ('\u{3f4c6}',);
RET = ('\u{7dbe8}',);
RET.0 = '\u{5ae5d}';
_3 = 132448278875467762943771564969615494555_u128 as f32;
_3 = 8240833893055308346_usize as f32;
RET = ('\u{13592}',);
_1 = !(-5247597957517378296_i64);
Call(_1 = core::intrinsics::bswap(6202013212302405981_i64), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = true;
RET = ('\u{10322f}',);
RET.0 = '\u{6bd3b}';
_2 = false;
RET.0 = '\u{456f}';
RET.0 = '\u{a576a}';
_4 = core::ptr::addr_of!(_1);
_4 = core::ptr::addr_of!((*_4));
(*_4) = -7293836126794908913_i64;
RET = ('\u{d72eb}',);
_3 = 45755_u16 as f32;
(*_4) = -534058285692192362_i64;
RET = ('\u{69609}',);
_6 = -(-9223372036854775808_isize);
RET.0 = '\u{c9473}';
RET.0 = '\u{5cdf}';
RET.0 = '\u{f8368}';
RET = ('\u{61e85}',);
_6 = !(-19_isize);
_5 = core::ptr::addr_of_mut!(_6);
_1 = _3 as i64;
RET = ('\u{704ba}',);
_2 = !true;
Goto(bb5)
}
bb5 = {
RET = ('\u{b4d51}',);
RET.0 = '\u{a2208}';
RET.0 = '\u{e7318}';
(*_5) = 79_isize ^ 9223372036854775807_isize;
_5 = core::ptr::addr_of_mut!((*_5));
RET = ('\u{32a24}',);
RET = ('\u{64279}',);
(*_5) = 17221997929770293982_u64 as isize;
_2 = RET.0 > RET.0;
Goto(bb6)
}
bb6 = {
RET = ('\u{e74cf}',);
RET = ('\u{1c30a}',);
RET = ('\u{7b3e2}',);
RET = ('\u{25cbc}',);
RET.0 = '\u{ed040}';
_5 = core::ptr::addr_of_mut!((*_5));
(*_4) = -6024623545360570981_i64;
(*_4) = (-8159531316123062288_i64) * (-3128308686524632249_i64);
(*_4) = (-7876183547124277295_i64) * 7970094442585728337_i64;
_9 = !(*_5);
_4 = core::ptr::addr_of!(_1);
RET = ('\u{dddfa}',);
(*_5) = _9;
RET.0 = '\u{3b3bf}';
(*_5) = _9;
_7 = [330370173453831363_usize];
_10.0 = RET.0;
_8 = 1454037607_u32 as u64;
RET = _10;
RET.0 = _10.0;
(*_4) = !1247968522235332880_i64;
RET.0 = _10.0;
_5 = core::ptr::addr_of_mut!(_9);
RET = (_10.0,);
_9 = _6;
Goto(bb7)
}
bb7 = {
_15 = &_11;
_4 = core::ptr::addr_of!(_1);
_4 = core::ptr::addr_of!(_1);
_6 = _9 | (*_5);
_6 = (*_5) & _9;
RET = (_10.0,);
_10 = (RET.0,);
_2 = !true;
_2 = !true;
RET = (_10.0,);
_3 = 71351107212462469518052710820338056760_i128 as f32;
_6 = _8 as isize;
(*_5) = _6;
RET.0 = _10.0;
(*_4) = -(-2766641545309663920_i64);
_13 = [(*_5),_9,(*_5),_9,_9,(*_5),_9,(*_5)];
_7 = [5_usize];
RET = (_10.0,);
_6 = (-105315240746391750362067435591421325765_i128) as isize;
_4 = core::ptr::addr_of!(_1);
Goto(bb8)
}
bb8 = {
RET = (_10.0,);
_10 = (RET.0,);
(*_5) = _3 as isize;
_13 = [(*_5),(*_5),_6,_6,_9,_9,(*_5),(*_5)];
_7 = [7_usize];
_3 = 102104615520824896718587895094280684662_u128 as f32;
_3 = 96_i8 as f32;
_6 = 21293_i16 as isize;
_8 = 418853891012763548_u64;
_10 = (RET.0,);
(*_5) = !_6;
_17 = _2;
_18 = -56_i8;
_18 = 104_i8 & (-46_i8);
(*_5) = _6 >> _6;
_1 = 6183891432035635981_i64 & 7935546305735797814_i64;
(*_5) = _6;
RET.0 = _10.0;
_13 = [_9,_6,(*_5),(*_5),(*_5),_6,_6,_9];
_1 = 7894914223012942648_i64 >> _9;
RET.0 = _10.0;
_6 = (*_5) * _9;
Call(_18 = core::intrinsics::transmute(_2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_4) = -(-7952348968352979640_i64);
_18 = !88_i8;
(*_4) = 63586455_i32 as i64;
_1 = (-8595899205491481433_i64) >> _6;
RET.0 = _10.0;
RET.0 = _10.0;
_25.0.0 = 163_u8 + 181_u8;
(*_5) = !_6;
_22 = core::ptr::addr_of!(_25.1.1);
_21.1 = core::ptr::addr_of!(_5);
_2 = _17;
_22 = core::ptr::addr_of!((*_22));
_3 = (*_5) as f32;
_4 = core::ptr::addr_of!((*_4));
_25.0 = (53_u8,);
match _8 {
0 => bb10,
1 => bb11,
418853891012763548 => bb13,
_ => bb12
}
}
bb10 = {
RET = (_10.0,);
_10 = (RET.0,);
(*_5) = _3 as isize;
_13 = [(*_5),(*_5),_6,_6,_9,_9,(*_5),(*_5)];
_7 = [7_usize];
_3 = 102104615520824896718587895094280684662_u128 as f32;
_3 = 96_i8 as f32;
_6 = 21293_i16 as isize;
_8 = 418853891012763548_u64;
_10 = (RET.0,);
(*_5) = !_6;
_17 = _2;
_18 = -56_i8;
_18 = 104_i8 & (-46_i8);
(*_5) = _6 >> _6;
_1 = 6183891432035635981_i64 & 7935546305735797814_i64;
(*_5) = _6;
RET.0 = _10.0;
_13 = [_9,_6,(*_5),(*_5),(*_5),_6,_6,_9];
_1 = 7894914223012942648_i64 >> _9;
RET.0 = _10.0;
_6 = (*_5) * _9;
Call(_18 = core::intrinsics::transmute(_2), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
RET = ('\u{639}',);
_1 = true as i64;
RET = ('\u{bc2f6}',);
RET = ('\u{e8e94}',);
RET = ('\u{77a0a}',);
RET.0 = '\u{ef5a7}';
RET.0 = '\u{7ef98}';
RET.0 = '\u{5cd67}';
RET = ('\u{4df2b}',);
_1 = 4743010624498770347_i64;
RET.0 = '\u{8cb5c}';
_1 = (-4646769039308761968_i64);
_2 = false;
RET = ('\u{19851}',);
_2 = !false;
_1 = !(-6328116667828562041_i64);
RET = ('\u{6a505}',);
_1 = (-6073762660182210973_i64);
RET.0 = '\u{4f985}';
RET.0 = '\u{ca4bf}';
Goto(bb2)
}
bb12 = {
RET = ('\u{859fc}',);
_3 = 62208_u16 as f32;
RET.0 = '\u{f8af}';
RET = ('\u{40625}',);
RET.0 = '\u{9c8cc}';
RET.0 = '\u{9a21e}';
_3 = 6_usize as f32;
Call(RET = fn2(_3, _1, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_26.fld0 = 52294_u16 | 27702_u16;
(*_22).fld0 = !_26.fld0;
_24 = [29484896618079730710021221162322497113_u128,112845562186211510158993953099817806617_u128];
(*_22) = _26;
_1 = 3667472209_u32 as i64;
(*_22) = _26;
_21.1 = core::ptr::addr_of!(_5);
_25.1.2 = (-1244027660_i32) as f64;
RET.0 = _10.0;
_6 = (*_5) + (*_5);
_25.1.0 = !_25.0.0;
_23 = [259978757482805928787682553129781777791_u128,284341507214146126762150704012678858866_u128];
(*_22).fld0 = _26.fld0;
(*_22).fld0 = 4_usize as u16;
_4 = core::ptr::addr_of!((*_4));
_21.0.0 = &_28;
_25.0 = (_25.1.0,);
_25.1.1.fld0 = !_26.fld0;
_25.0.0 = _25.1.0;
_29.1.1.fld0 = _25.1.1.fld0;
Goto(bb14)
}
bb14 = {
_25.1.2 = 2130379685_u32 as f64;
_29.1.0 = !_25.1.0;
_25.1.1 = Adt22 { fld0: _26.fld0 };
_29.1 = _25.1;
_25.1.2 = -_29.1.2;
_29 = (_25.0, _25.1);
_29.0.0 = _25.0.0 << (*_5);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 24_usize, Move(_24), 7_usize, Move(_7), 8_usize, Move(_8), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 6_usize, Move(_6), 9_usize, Move(_9), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1() -> (char,) {
mir! {
type RET = (char,);
let _1: (u64,);
let _2: i128;
let _3: &'static [u32; 7];
let _4: *const *mut isize;
let _5: ((u8,), (u8, Adt22, f64));
let _6: bool;
let _7: isize;
let _8: isize;
let _9: usize;
let _10: i128;
let _11: (u8, Adt22, f64);
let _12: Adt68;
let _13: f64;
let _14: bool;
let _15: char;
let _16: [i16; 7];
let _17: isize;
let _18: ((u8,), (u8, Adt22, f64));
let _19: u16;
let _20: u8;
let _21: bool;
let _22: ();
let _23: ();
{
RET.0 = '\u{8a3f4}';
RET.0 = '\u{ccae3}';
RET.0 = '\u{adb35}';
RET = ('\u{5bf7e}',);
RET = ('\u{394c7}',);
RET = ('\u{76c55}',);
RET.0 = '\u{20e78}';
RET = ('\u{d16db}',);
Goto(bb1)
}
bb1 = {
RET.0 = '\u{512e9}';
_1.0 = !14557104079644634233_u64;
_1.0 = 7946356121987239397_u64 | 8110429978592589192_u64;
RET.0 = '\u{3dfb4}';
RET.0 = '\u{39db9}';
Goto(bb2)
}
bb2 = {
_1.0 = 5289195984317199573_u64 - 12751573022906194195_u64;
_1 = (14906437259268592898_u64,);
RET.0 = '\u{92780}';
match _1.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
14906437259268592898 => bb11,
_ => bb10
}
}
bb3 = {
RET.0 = '\u{512e9}';
_1.0 = !14557104079644634233_u64;
_1.0 = 7946356121987239397_u64 | 8110429978592589192_u64;
RET.0 = '\u{3dfb4}';
RET.0 = '\u{39db9}';
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
_1.0 = 195_u8 as u64;
RET.0 = '\u{73b81}';
RET.0 = '\u{50117}';
_5.1.1.fld0 = 601_u16;
_2 = (-10602064214482357161020463797233345102_i128) << _1.0;
_5.1.0 = !131_u8;
_6 = _5.1.0 == _5.1.0;
RET.0 = '\u{616ee}';
_5.1.2 = 249453868693302419029462439007904757291_u128 as f64;
_1 = (9074012295856610695_u64,);
_5.1.1 = Adt22 { fld0: 2375_u16 };
_1.0 = 2829155736827797456_u64 << _2;
_5.0.0 = _5.1.0;
_5.0.0 = !_5.1.0;
RET = ('\u{a4f84}',);
_7 = _5.1.1.fld0 as isize;
_7 = (-9223372036854775808_isize);
Goto(bb12)
}
bb12 = {
RET = ('\u{82522}',);
_5.1.1.fld0 = 37539_u16 | 22775_u16;
RET = ('\u{ccb6e}',);
_5.1.1.fld0 = !62814_u16;
_5.0.0 = _5.1.0;
_5.1.0 = _5.0.0 ^ _5.0.0;
_6 = !true;
_5.1.2 = 1285374498_u32 as f64;
RET.0 = '\u{b50bf}';
RET.0 = '\u{1ef29}';
_5.1.1.fld0 = 10007_u16 ^ 25159_u16;
_2 = -(-139092518864892836236421167381345409907_i128);
_6 = _1.0 < _1.0;
_8 = _7 & _7;
_10 = _2 - _2;
_1.0 = _6 as u64;
_10 = _2 * _2;
_11.1 = Adt22 { fld0: _5.1.1.fld0 };
_5.0.0 = RET.0 as u8;
_1 = (5698567624472003029_u64,);
Goto(bb13)
}
bb13 = {
_11.1 = Adt22 { fld0: _5.1.1.fld0 };
_5.0.0 = _5.1.0;
_11.0 = !_5.0.0;
RET = ('\u{938dc}',);
_5.1.2 = (-30756_i16) as f64;
_1.0 = 35_i8 as u64;
_7 = -_8;
_11.2 = _5.1.2 + _5.1.2;
_1.0 = 2291015863365458186_u64 * 14778063490933007383_u64;
_5.0 = (_11.0,);
_5.0.0 = 21791_i16 as u8;
_8 = !_7;
_10 = _2 * _2;
_5.1 = (_11.0, _11.1, _11.2);
_13 = _1.0 as f64;
_5.1.1.fld0 = !_11.1.fld0;
_1.0 = _2 as u64;
_2 = _10;
_11.1 = _5.1.1;
RET.0 = '\u{59dd}';
RET = ('\u{8595f}',);
_11.1.fld0 = _5.1.1.fld0 >> _8;
_18.1 = _11;
_14 = _6;
_17 = _8 + _8;
Goto(bb14)
}
bb14 = {
_18.0.0 = RET.0 as u8;
RET.0 = '\u{b9943}';
_5.0 = (_11.0,);
_18.1.2 = 13_i8 as f64;
_5.0.0 = !_18.1.0;
_18.1.1 = Adt22 { fld0: _5.1.1.fld0 };
_1.0 = 0_usize as u64;
_10 = _2;
_18.1.1.fld0 = _5.1.1.fld0;
_18.1.1.fld0 = _13 as u16;
_9 = 7291749434657164322_usize ^ 4116545925907804949_usize;
_11 = (_5.0.0, _18.1.1, _18.1.2);
_1.0 = !2889096904687959983_u64;
_5.0.0 = _11.0;
_17 = _8 >> _5.1.0;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(1_usize, 9_usize, Move(_9), 2_usize, Move(_2), 8_usize, Move(_8), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f32,mut _2: i64,mut _3: i64,mut _4: f32) -> (char,) {
mir! {
type RET = (char,);
let _5: u8;
let _6: char;
let _7: ((u8,), (u8, Adt22, f64));
let _8: i32;
let _9: bool;
let _10: Adt31;
let _11: &'static &'static *const isize;
let _12: bool;
let _13: &'static Adt39;
let _14: &'static [u32; 7];
let _15: f64;
let _16: Adt54;
let _17: f64;
let _18: bool;
let _19: &'static [u32; 7];
let _20: usize;
let _21: Adt22;
let _22: *mut *mut i8;
let _23: isize;
let _24: (i32, f64, i32, u64);
let _25: &'static (u8, Adt22, f64);
let _26: f64;
let _27: [u16; 3];
let _28: u16;
let _29: ();
let _30: ();
{
RET = ('\u{695e1}',);
RET = ('\u{5c787}',);
RET.0 = '\u{631ba}';
RET = ('\u{78b6d}',);
_1 = 734891010_i32 as f32;
RET.0 = '\u{cc0f6}';
_4 = _1;
_1 = _4 + _4;
RET.0 = '\u{6c86f}';
_5 = 36_u8;
_1 = -_4;
Call(RET = fn3(_3, _3, _2, _3, _3, _3, _5, _3, _3, _4, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = '\u{e60fd}';
_3 = 10432811650085727975_u64 as i64;
_1 = -_4;
_1 = 40167_u16 as f32;
_2 = -_3;
RET = ('\u{73cd7}',);
RET = ('\u{c22f9}',);
RET.0 = '\u{5f99a}';
RET = ('\u{2cb56}',);
RET = ('\u{64466}',);
_5 = RET.0 as u8;
RET.0 = '\u{59c0d}';
_2 = _5 as i64;
_5 = 239_u8 - 41_u8;
_6 = RET.0;
_3 = _2 ^ _2;
RET.0 = _6;
_7.1.0 = _5;
RET.0 = _6;
_7.1.0 = _5;
Goto(bb2)
}
bb2 = {
_7.1.1.fld0 = !17288_u16;
RET.0 = _6;
RET = (_6,);
_7.0 = (_7.1.0,);
_7.1.2 = 158132995136079457575899388497104708460_u128 as f64;
RET = (_6,);
RET = (_6,);
_7.0 = (_5,);
_4 = _5 as f32;
_7.1.0 = !_5;
_8 = 1845079636_i32 + 151083868_i32;
_6 = RET.0;
_7.1.1 = Adt22 { fld0: 51010_u16 };
_7.1.2 = 4616_i16 as f64;
RET.0 = _6;
_7.0.0 = _5 ^ _7.1.0;
_1 = -_4;
_2 = _3 ^ _3;
RET.0 = _6;
_3 = !_2;
RET = (_6,);
RET = (_6,);
Goto(bb3)
}
bb3 = {
_7.1.1.fld0 = _3 as u16;
_12 = false;
_5 = 8412646911487468893_u64 as u8;
_4 = _1 * _1;
_6 = RET.0;
_1 = _4;
_12 = true ^ true;
_7.0.0 = _7.1.0;
_5 = _7.1.0;
RET = (_6,);
_9 = _12;
_7.1.1.fld0 = 9223372036854775807_isize as u16;
_15 = _7.1.2;
_7.1.1.fld0 = (-105_isize) as u16;
_1 = _4;
RET.0 = _6;
_8 = 1041461576_i32;
_15 = -_7.1.2;
_1 = -_4;
_1 = _4 - _4;
match _8 {
0 => bb4,
1 => bb5,
1041461576 => bb7,
_ => bb6
}
}
bb4 = {
_7.1.1.fld0 = !17288_u16;
RET.0 = _6;
RET = (_6,);
_7.0 = (_7.1.0,);
_7.1.2 = 158132995136079457575899388497104708460_u128 as f64;
RET = (_6,);
RET = (_6,);
_7.0 = (_5,);
_4 = _5 as f32;
_7.1.0 = !_5;
_8 = 1845079636_i32 + 151083868_i32;
_6 = RET.0;
_7.1.1 = Adt22 { fld0: 51010_u16 };
_7.1.2 = 4616_i16 as f64;
RET.0 = _6;
_7.0.0 = _5 ^ _7.1.0;
_1 = -_4;
_2 = _3 ^ _3;
RET.0 = _6;
_3 = !_2;
RET = (_6,);
RET = (_6,);
Goto(bb3)
}
bb5 = {
RET.0 = '\u{e60fd}';
_3 = 10432811650085727975_u64 as i64;
_1 = -_4;
_1 = 40167_u16 as f32;
_2 = -_3;
RET = ('\u{73cd7}',);
RET = ('\u{c22f9}',);
RET.0 = '\u{5f99a}';
RET = ('\u{2cb56}',);
RET = ('\u{64466}',);
_5 = RET.0 as u8;
RET.0 = '\u{59c0d}';
_2 = _5 as i64;
_5 = 239_u8 - 41_u8;
_6 = RET.0;
_3 = _2 ^ _2;
RET.0 = _6;
_7.1.0 = _5;
RET.0 = _6;
_7.1.0 = _5;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_15 = (-67_i8) as f64;
_20 = (-61_i8) as usize;
RET.0 = _6;
_17 = (-9223372036854775808_isize) as f64;
_2 = _3 - _3;
_18 = !_9;
_7.1.1.fld0 = _6 as u16;
_7.0.0 = !_5;
_6 = RET.0;
_18 = !_12;
_15 = 2450_i16 as f64;
_15 = -_17;
_2 = _3 + _3;
_8 = 1099562201_i32;
_4 = _7.1.1.fld0 as f32;
_4 = _1 * _1;
_15 = -_7.1.2;
_18 = _1 <= _1;
_7.1.1 = Adt22 { fld0: 16554_u16 };
_2 = _3;
_1 = _4;
_18 = _12 ^ _9;
_7.1.1.fld0 = 46736_u16 >> _7.1.0;
Goto(bb8)
}
bb8 = {
_7.0 = (_5,);
_18 = _9;
_23 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_24 = (_8, _7.1.2, _8, 8033425727102777484_u64);
_23 = 11631_i16 as isize;
_1 = 62811339552712756489912853501194601888_i128 as f32;
_21.fld0 = _7.1.0 as u16;
_4 = _1 - _1;
RET.0 = _6;
match _24.2 {
0 => bb9,
1 => bb10,
2 => bb11,
1099562201 => bb13,
_ => bb12
}
}
bb9 = {
_7.1.1.fld0 = _3 as u16;
_12 = false;
_5 = 8412646911487468893_u64 as u8;
_4 = _1 * _1;
_6 = RET.0;
_1 = _4;
_12 = true ^ true;
_7.0.0 = _7.1.0;
_5 = _7.1.0;
RET = (_6,);
_9 = _12;
_7.1.1.fld0 = 9223372036854775807_isize as u16;
_15 = _7.1.2;
_7.1.1.fld0 = (-105_isize) as u16;
_1 = _4;
RET.0 = _6;
_8 = 1041461576_i32;
_15 = -_7.1.2;
_1 = -_4;
_1 = _4 - _4;
match _8 {
0 => bb4,
1 => bb5,
1041461576 => bb7,
_ => bb6
}
}
bb10 = {
RET.0 = '\u{e60fd}';
_3 = 10432811650085727975_u64 as i64;
_1 = -_4;
_1 = 40167_u16 as f32;
_2 = -_3;
RET = ('\u{73cd7}',);
RET = ('\u{c22f9}',);
RET.0 = '\u{5f99a}';
RET = ('\u{2cb56}',);
RET = ('\u{64466}',);
_5 = RET.0 as u8;
RET.0 = '\u{59c0d}';
_2 = _5 as i64;
_5 = 239_u8 - 41_u8;
_6 = RET.0;
_3 = _2 ^ _2;
RET.0 = _6;
_7.1.0 = _5;
RET.0 = _6;
_7.1.0 = _5;
Goto(bb2)
}
bb11 = {
_7.1.1.fld0 = !17288_u16;
RET.0 = _6;
RET = (_6,);
_7.0 = (_7.1.0,);
_7.1.2 = 158132995136079457575899388497104708460_u128 as f64;
RET = (_6,);
RET = (_6,);
_7.0 = (_5,);
_4 = _5 as f32;
_7.1.0 = !_5;
_8 = 1845079636_i32 + 151083868_i32;
_6 = RET.0;
_7.1.1 = Adt22 { fld0: 51010_u16 };
_7.1.2 = 4616_i16 as f64;
RET.0 = _6;
_7.0.0 = _5 ^ _7.1.0;
_1 = -_4;
_2 = _3 ^ _3;
RET.0 = _6;
_3 = !_2;
RET = (_6,);
RET = (_6,);
Goto(bb3)
}
bb12 = {
_7.1.1.fld0 = !17288_u16;
RET.0 = _6;
RET = (_6,);
_7.0 = (_7.1.0,);
_7.1.2 = 158132995136079457575899388497104708460_u128 as f64;
RET = (_6,);
RET = (_6,);
_7.0 = (_5,);
_4 = _5 as f32;
_7.1.0 = !_5;
_8 = 1845079636_i32 + 151083868_i32;
_6 = RET.0;
_7.1.1 = Adt22 { fld0: 51010_u16 };
_7.1.2 = 4616_i16 as f64;
RET.0 = _6;
_7.0.0 = _5 ^ _7.1.0;
_1 = -_4;
_2 = _3 ^ _3;
RET.0 = _6;
_3 = !_2;
RET = (_6,);
RET = (_6,);
Goto(bb3)
}
bb13 = {
_7.1.1 = _21;
_9 = !_12;
_7.1.1 = Adt22 { fld0: _21.fld0 };
RET = (_6,);
_26 = _17 + _24.1;
_3 = _2;
_15 = _7.1.2 + _7.1.2;
match _24.3 {
0 => bb1,
1 => bb11,
2 => bb6,
3 => bb4,
4 => bb14,
8033425727102777484 => bb16,
_ => bb15
}
}
bb14 = {
_7.1.1.fld0 = _3 as u16;
_12 = false;
_5 = 8412646911487468893_u64 as u8;
_4 = _1 * _1;
_6 = RET.0;
_1 = _4;
_12 = true ^ true;
_7.0.0 = _7.1.0;
_5 = _7.1.0;
RET = (_6,);
_9 = _12;
_7.1.1.fld0 = 9223372036854775807_isize as u16;
_15 = _7.1.2;
_7.1.1.fld0 = (-105_isize) as u16;
_1 = _4;
RET.0 = _6;
_8 = 1041461576_i32;
_15 = -_7.1.2;
_1 = -_4;
_1 = _4 - _4;
match _8 {
0 => bb4,
1 => bb5,
1041461576 => bb7,
_ => bb6
}
}
bb15 = {
RET.0 = '\u{e60fd}';
_3 = 10432811650085727975_u64 as i64;
_1 = -_4;
_1 = 40167_u16 as f32;
_2 = -_3;
RET = ('\u{73cd7}',);
RET = ('\u{c22f9}',);
RET.0 = '\u{5f99a}';
RET = ('\u{2cb56}',);
RET = ('\u{64466}',);
_5 = RET.0 as u8;
RET.0 = '\u{59c0d}';
_2 = _5 as i64;
_5 = 239_u8 - 41_u8;
_6 = RET.0;
_3 = _2 ^ _2;
RET.0 = _6;
_7.1.0 = _5;
RET.0 = _6;
_7.1.0 = _5;
Goto(bb2)
}
bb16 = {
_27 = [_7.1.1.fld0,_21.fld0,_21.fld0];
_6 = RET.0;
_23 = (-9223372036854775808_isize);
_17 = _15 * _26;
_17 = -_7.1.2;
_24.0 = _8 << _7.1.0;
_26 = _24.2 as f64;
_7.1.1.fld0 = _21.fld0 | _21.fld0;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(2_usize, 6_usize, Move(_6), 20_usize, Move(_20), 3_usize, Move(_3), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(2_usize, 9_usize, Move(_9), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: u8,mut _8: i64,mut _9: i64,mut _10: f32,mut _11: i64) -> (char,) {
mir! {
type RET = (char,);
let _12: i16;
let _13: *const i64;
let _14: &'static Adt42;
let _15: u128;
let _16: i64;
let _17: f64;
let _18: [u32; 3];
let _19: [u32; 3];
let _20: char;
let _21: (u64,);
let _22: isize;
let _23: (u64,);
let _24: *const (*const i64,);
let _25: (u8,);
let _26: usize;
let _27: ((u8,), (u8, Adt22, f64));
let _28: f32;
let _29: *const i32;
let _30: u64;
let _31: Adt39;
let _32: [u8; 4];
let _33: isize;
let _34: (u64,);
let _35: [u64; 1];
let _36: char;
let _37: (char,);
let _38: (u64,);
let _39: [i16; 7];
let _40: [usize; 1];
let _41: u16;
let _42: *const (*const i64,);
let _43: Adt22;
let _44: f32;
let _45: [u128; 2];
let _46: ();
let _47: ();
{
match _5 {
340282366920938463457300844771586000483 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET.0 = '\u{eaa28}';
_11 = _1;
RET = ('\u{9696e}',);
_11 = -_4;
_11 = _5;
Call(_2 = fn4(_5, RET.0, _5, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _3 << _2;
_1 = _2 - _6;
_13 = core::ptr::addr_of!(_2);
_15 = (-9223372036854775808_isize) as u128;
_12 = 21926_i16 * 10551_i16;
_12 = (-150_i16);
_8 = 9223372036854775807_isize as i64;
RET.0 = '\u{85cdc}';
_3 = !_1;
Call(_16 = fn5(_3, _3, (*_13), _3, _3, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = 173_u8 ^ 143_u8;
_15 = 295994536276398314944163351379025937022_u128;
_9 = _2;
_10 = _12 as f32;
_2 = _9 - _16;
_10 = (-58863042779578485200181991866323692436_i128) as f32;
_15 = true as u128;
RET = ('\u{3feb6}',);
(*_13) = _12 as i64;
RET = ('\u{ddc5a}',);
_2 = _16 - _1;
(*_13) = !_1;
_16 = _2 + (*_13);
RET = ('\u{3d702}',);
_8 = false as i64;
RET = ('\u{12b86}',);
_10 = _7 as f32;
_12 = !(-380_i16);
_11 = -_9;
_2 = !_11;
_4 = RET.0 as i64;
RET = ('\u{a53ac}',);
RET = ('\u{9f10c}',);
_6 = _3 - _11;
_17 = _15 as f64;
_16 = (*_13) * _11;
_2 = _9;
Call(_11 = fn6(_9, _16, _3, _1, _9, _1, (*_13), _6, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !_9;
_5 = _11;
_1 = -_2;
_20 = RET.0;
_5 = !_9;
_20 = RET.0;
_13 = core::ptr::addr_of!(_11);
RET = (_20,);
_20 = RET.0;
_11 = -_5;
Goto(bb6)
}
bb6 = {
_12 = _15 as i16;
_20 = RET.0;
_18 = [1139516964_u32,4238733569_u32,2700543267_u32];
_9 = _11 | _2;
_3 = (*_13) - (*_13);
_6 = _12 as i64;
_12 = (-23344_i16) & (-3122_i16);
_2 = _5;
_21 = (3076054485030551183_u64,);
_11 = _3;
_8 = -_9;
_22 = (-98_isize) & (-9223372036854775808_isize);
_5 = _22 as i64;
_19 = [2854235008_u32,3295718783_u32,3761904584_u32];
RET.0 = _20;
RET.0 = _20;
Call(_9 = core::intrinsics::bswap(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_13) = !_2;
_5 = _8 - _3;
_16 = 50586084271497814639323457870504254167_i128 as i64;
(*_13) = _9 >> _8;
_17 = (-60765840682612830880823710784471995530_i128) as f64;
RET = (_20,);
_9 = _11 * (*_13);
_13 = core::ptr::addr_of!(_16);
_1 = _9 ^ _8;
_19 = [1554116388_u32,3760450254_u32,1886893150_u32];
_18 = _19;
_22 = !9223372036854775807_isize;
Call(_4 = core::intrinsics::transmute(_9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = _5 - _1;
RET.0 = _20;
_21 = (9652232528795869835_u64,);
_11 = _21.0 as i64;
RET = (_20,);
_10 = _17 as f32;
RET = (_20,);
_8 = _3 + _9;
(*_13) = -_9;
_17 = _7 as f64;
match _21.0 {
9652232528795869835 => bb10,
_ => bb9
}
}
bb9 = {
_2 = !_9;
_5 = _11;
_1 = -_2;
_20 = RET.0;
_5 = !_9;
_20 = RET.0;
_13 = core::ptr::addr_of!(_11);
RET = (_20,);
_20 = RET.0;
_11 = -_5;
Goto(bb6)
}
bb10 = {
_23 = (_21.0,);
_18 = _19;
_19 = [2814992640_u32,2370228480_u32,3878358443_u32];
_1 = (*_13) | _5;
_27.1.1.fld0 = 64989_u16 - 38227_u16;
_27.0 = (_7,);
_6 = -_3;
_13 = core::ptr::addr_of!(_16);
RET.0 = _20;
_21.0 = _23.0;
_27.0 = (_7,);
RET = (_20,);
RET = (_20,);
_13 = core::ptr::addr_of!(_2);
_4 = _16 << _1;
_7 = _6 as u8;
_27.1.0 = (-136514058121486118511651983394775727228_i128) as u8;
_30 = !_23.0;
_22 = _12 as isize;
_27.1.2 = _17;
_28 = -_10;
_31 = Adt39::Variant2 { fld0: _7,fld1: _27.0,fld2: _27.1.2,fld3: _27.1.1 };
_27.1.1 = Adt22 { fld0: Field::<Adt22>(Variant(_31, 2), 3).fld0 };
Goto(bb11)
}
bb11 = {
SetDiscriminant(_31, 2);
_32 = [_7,_7,_7,_7];
_10 = _15 as f32;
Goto(bb12)
}
bb12 = {
_8 = (*_13) ^ (*_13);
_28 = -_10;
_31 = Adt39::Variant0 { fld0: _23,fld1: RET,fld2: _22,fld3: _27.1.1.fld0,fld4: 51407098077279201763328853776824256753_i128 };
_22 = Field::<isize>(Variant(_31, 0), 2) * Field::<isize>(Variant(_31, 0), 2);
_2 = _9;
_30 = _21.0 << _16;
RET = (_20,);
_22 = -Field::<isize>(Variant(_31, 0), 2);
_4 = (*_13) << _8;
place!(Field::<(char,)>(Variant(_31, 0), 1)).0 = _20;
_35 = [_30];
_21.0 = _30;
(*_13) = -_3;
_27.1.0 = !_7;
RET.0 = _20;
_34.0 = !_30;
_22 = Field::<isize>(Variant(_31, 0), 2) | Field::<isize>(Variant(_31, 0), 2);
_25 = (_7,);
_22 = 1680898881_i32 as isize;
place!(Field::<(u64,)>(Variant(_31, 0), 0)) = _34;
Goto(bb13)
}
bb13 = {
place!(Field::<(char,)>(Variant(_31, 0), 1)).0 = RET.0;
_19 = _18;
place!(Field::<(u64,)>(Variant(_31, 0), 0)) = _21;
_37 = RET;
_36 = Field::<(char,)>(Variant(_31, 0), 1).0;
_23 = (Field::<(u64,)>(Variant(_31, 0), 0).0,);
_33 = _22;
_2 = _4;
_30 = !_21.0;
_40 = [12414684368297455538_usize];
_37.0 = _36;
_3 = (*_13) | _4;
_17 = _27.1.2 + _27.1.2;
_38.0 = Field::<(u64,)>(Variant(_31, 0), 0).0;
_16 = Field::<(u64,)>(Variant(_31, 0), 0).0 as i64;
_3 = 9350201841297382950705065145611523508_i128 as i64;
_16 = 226624040_i32 as i64;
_25.0 = _10 as u8;
_30 = _23.0 - _21.0;
_34.0 = !_21.0;
_6 = _9 | (*_13);
(*_13) = _7 as i64;
_1 = -_5;
_2 = !_8;
_27.1.0 = 3675104615_u32 as u8;
_19 = [317594723_u32,3462340065_u32,1524474852_u32];
_27.1.1.fld0 = Field::<u16>(Variant(_31, 0), 3) & Field::<u16>(Variant(_31, 0), 3);
_12 = 10646_i16;
match _12 {
0 => bb1,
1 => bb12,
2 => bb3,
3 => bb11,
4 => bb5,
5 => bb14,
10646 => bb16,
_ => bb15
}
}
bb14 = {
_7 = 173_u8 ^ 143_u8;
_15 = 295994536276398314944163351379025937022_u128;
_9 = _2;
_10 = _12 as f32;
_2 = _9 - _16;
_10 = (-58863042779578485200181991866323692436_i128) as f32;
_15 = true as u128;
RET = ('\u{3feb6}',);
(*_13) = _12 as i64;
RET = ('\u{ddc5a}',);
_2 = _16 - _1;
(*_13) = !_1;
_16 = _2 + (*_13);
RET = ('\u{3d702}',);
_8 = false as i64;
RET = ('\u{12b86}',);
_10 = _7 as f32;
_12 = !(-380_i16);
_11 = -_9;
_2 = !_11;
_4 = RET.0 as i64;
RET = ('\u{a53ac}',);
RET = ('\u{9f10c}',);
_6 = _3 - _11;
_17 = _15 as f64;
_16 = (*_13) * _11;
_2 = _9;
Call(_11 = fn6(_9, _16, _3, _1, _9, _1, (*_13), _6, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
SetDiscriminant(_31, 2);
_32 = [_7,_7,_7,_7];
_10 = _15 as f32;
Goto(bb12)
}
bb16 = {
_12 = _22 as i16;
RET = (_36,);
_2 = -_6;
RET = Field::<(char,)>(Variant(_31, 0), 1);
_2 = !_4;
_25 = (_7,);
_41 = _21.0 as u16;
_43 = Adt22 { fld0: _41 };
_12 = 15186_i16;
place!(Field::<u16>(Variant(_31, 0), 3)) = _36 as u16;
_13 = core::ptr::addr_of!(_1);
_37.0 = RET.0;
_36 = _20;
_32 = [_7,_7,_7,_7];
_39 = [_12,_12,_12,_12,_12,_12,_12];
_43.fld0 = 7942247271460894765_usize as u16;
_27.0 = (_7,);
_27.0.0 = !_7;
_25.0 = !_27.0.0;
_7 = !_25.0;
_38.0 = _17 as u64;
Goto(bb17)
}
bb17 = {
Call(_46 = dump_var(3_usize, 2_usize, Move(_2), 23_usize, Move(_23), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(3_usize, 41_usize, Move(_41), 21_usize, Move(_21), 40_usize, Move(_40), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(3_usize, 3_usize, Move(_3), 36_usize, Move(_36), 38_usize, Move(_38), 30_usize, Move(_30)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(3_usize, 4_usize, Move(_4), 7_usize, Move(_7), 34_usize, Move(_34), 47_usize, _47), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i64,mut _2: char,mut _3: i64,mut _4: i64) -> i64 {
mir! {
type RET = i64;
let _5: u128;
let _6: u32;
let _7: u32;
let _8: ((&'static Adt39,), *const *mut isize);
let _9: u32;
let _10: [u32; 3];
let _11: [u64; 1];
let _12: ();
let _13: ();
{
_1 = -_3;
RET = _4;
_1 = 26310_i16 as i64;
RET = _1 * _3;
_1 = RET + RET;
RET = !_1;
_5 = 264728087180565316817627733138967309221_u128 << _3;
RET = -_4;
Call(_7 = core::intrinsics::bswap(120976598_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = '\u{9201}';
RET = !_4;
_3 = !_1;
RET = _3;
RET = (-9223372036854775808_isize) as i64;
RET = _2 as i64;
RET = -_3;
RET = _1 + _1;
_4 = RET - RET;
_7 = 31828132_u32;
_1 = RET - _4;
RET = _4 * _4;
_4 = !RET;
_1 = 109902278204919220848513824711571889554_i128 as i64;
_3 = _7 as i64;
_7 = 3206144167_u32 >> RET;
_9 = _7;
_4 = 205_u8 as i64;
_3 = _4 - RET;
_6 = !_7;
_9 = !_7;
_7 = 89_i8 as u32;
_4 = 119_u8 as i64;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(4_usize, 6_usize, Move(_6), 2_usize, Move(_2), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64) -> i64 {
mir! {
type RET = i64;
let _8: Adt30;
let _9: (u64,);
let _10: [bool; 1];
let _11: ();
let _12: ();
{
RET = 4715_u16 as i64;
_5 = (-834460248_i32) as i64;
RET = !_2;
_6 = 1133961770_u32 as i64;
_5 = _4 & _1;
_6 = 4145569003_u32 as i64;
_8 = Adt30::Variant0 { fld0: 14602474564288845300_usize,fld1: 55028_u16,fld2: 6458721040628080567_u64,fld3: _7 };
_4 = -_2;
_5 = _7;
_7 = !_5;
place!(Field::<i64>(Variant(_8, 0), 3)) = _7;
place!(Field::<i64>(Variant(_8, 0), 3)) = _7;
place!(Field::<u64>(Variant(_8, 0), 2)) = 14301347974727934375_u64 >> RET;
RET = _5 ^ _4;
_6 = _1;
place!(Field::<usize>(Variant(_8, 0), 0)) = 0_usize;
place!(Field::<u16>(Variant(_8, 0), 1)) = 21838_u16 >> RET;
place!(Field::<usize>(Variant(_8, 0), 0)) = 7406379687977947536_usize << Field::<u16>(Variant(_8, 0), 1);
_9 = (Field::<u64>(Variant(_8, 0), 2),);
place!(Field::<u64>(Variant(_8, 0), 2)) = !_9.0;
_3 = _2;
place!(Field::<u16>(Variant(_8, 0), 1)) = 18514_u16 >> _5;
_3 = -_4;
_10 = [true];
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(5_usize, 9_usize, Move(_9), 5_usize, Move(_5), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64) -> i64 {
mir! {
type RET = i64;
let _10: isize;
let _11: u64;
let _12: [u32; 7];
let _13: [u32; 7];
let _14: *const *const (*const i64,);
let _15: *mut &'static [u32; 7];
let _16: (&'static Adt42, u8);
let _17: i16;
let _18: ();
let _19: ();
{
_8 = -_9;
_8 = _3;
RET = _5 ^ _8;
RET = (-9223372036854775808_isize) as i64;
_8 = _9 - _1;
_7 = _8;
_3 = _7;
_3 = _5;
RET = _7 | _6;
_1 = !_9;
_9 = 3184664251078322593_usize as i64;
RET = !_8;
RET = !_2;
_1 = !_8;
RET = 38000_u16 as i64;
Call(_7 = fn7(_3, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 235811520101634324396852465866114274871_u128 as i64;
_5 = _8 >> _4;
_6 = !_5;
_9 = _1;
_8 = 13377843605982293229_usize as i64;
_10 = -(-43_isize);
_8 = !_4;
_2 = _6;
_2 = !_3;
_3 = _5;
RET = !_2;
_7 = _5 ^ _6;
_1 = _4 >> _9;
_6 = _5 << _7;
_8 = !_1;
RET = !_5;
_6 = -_3;
_4 = -_5;
_9 = 13560255239408985561_usize as i64;
_9 = -_3;
_10 = 24_i8 as isize;
_6 = _1 >> _1;
_9 = _5;
_4 = _3;
_11 = 16106889460843196031_u64 & 6092282728397160545_u64;
_12 = [2033539006_u32,1470897374_u32,2511700297_u32,2172319906_u32,1543502860_u32,3502379022_u32,1399841849_u32];
RET = _10 as i64;
Call(_7 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = !_3;
_10 = !(-9223372036854775808_isize);
_13 = [2999237433_u32,4004173664_u32,4241367427_u32,3511648725_u32,3496602665_u32,2661884715_u32,4078978914_u32];
_13 = [128552854_u32,2802757954_u32,161188543_u32,194595856_u32,118091948_u32,3841431482_u32,3886719426_u32];
RET = _8 ^ _9;
RET = 32522_u16 as i64;
RET = 220_u8 as i64;
_13 = [3441970673_u32,19314174_u32,3802484589_u32,4188030832_u32,203288992_u32,18184041_u32,4133220692_u32];
_7 = _9;
_11 = 16895672221624762239_u64;
_10 = 9223372036854775807_isize;
_8 = _3;
_11 = 26708_u16 as u64;
_8 = _1 << _3;
_10 = !(-81_isize);
_6 = -_8;
_2 = _4;
_4 = _5 * _1;
_6 = _2;
_6 = _5 ^ _8;
_11 = 13047688723762552315_u64 | 3750489300683506882_u64;
Goto(bb3)
}
bb3 = {
_1 = (-4238_i16) as i64;
_13 = _12;
_2 = _3 | _3;
_8 = 445193743_i32 as i64;
_16.1 = 247_u8;
_17 = (-21199_i16) | (-30243_i16);
RET = _3 & _2;
_10 = -(-9223372036854775808_isize);
_13 = [1092358445_u32,1036026922_u32,2304490207_u32,2404296895_u32,2724955020_u32,2801774990_u32,2642863690_u32];
_5 = 60548_u16 as i64;
_17 = -(-22773_i16);
Goto(bb4)
}
bb4 = {
Call(_18 = dump_var(6_usize, 7_usize, Move(_7), 5_usize, Move(_5), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_18 = dump_var(6_usize, 6_usize, Move(_6), 3_usize, Move(_3), 4_usize, Move(_4), 19_usize, _19), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i64,mut _2: i64) -> i64 {
mir! {
type RET = i64;
let _3: *const *mut isize;
let _4: *const *mut isize;
let _5: (char,);
let _6: (&'static Adt42, u8);
let _7: *const &'static [u32; 7];
let _8: &'static *mut f32;
let _9: *const i32;
let _10: i64;
let _11: *const Adt31;
let _12: f64;
let _13: &'static &'static *const isize;
let _14: [u128; 2];
let _15: i8;
let _16: [u32; 7];
let _17: &'static *const *mut isize;
let _18: isize;
let _19: Adt30;
let _20: u16;
let _21: ((u8,), (u8, Adt22, f64));
let _22: ();
let _23: ();
{
RET = 3118713850_u32 as i64;
_2 = 4024679806_u32 as i64;
RET = _1;
_2 = _1;
_1 = 149_u8 as i64;
RET = _2;
_1 = 28_i8 as i64;
_2 = RET;
RET = !_2;
_2 = RET;
RET = 7_usize as i64;
Call(RET = fn8(_2, _2, _2, _2, _2, _2, _2, _2, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 3270744629253341282_usize as i64;
RET = _1;
_2 = RET & _1;
RET = -_2;
RET = 6312460907242777030_u64 as i64;
_1 = _2;
_1 = 1804406749_u32 as i64;
_2 = -RET;
RET = !_2;
RET = _1 & _1;
RET = _1;
RET = -_1;
RET = _2 - _2;
_2 = !RET;
_2 = RET - _1;
_2 = (-15329_i16) as i64;
RET = _2 & _1;
_2 = -RET;
_2 = (-120_i8) as i64;
RET = _2 ^ _1;
_1 = !RET;
RET = -_1;
_1 = 15455600468640641_u64 as i64;
_2 = !RET;
RET = -_2;
_1 = RET;
Call(_5.0 = fn9(RET, RET, RET, _2, _1, RET, _1, RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_1;
_1 = -_2;
_1 = _2;
_6.1 = 40_u8 - 199_u8;
RET = !_2;
RET = !_2;
RET = _1;
_5 = ('\u{ded85}',);
_5.0 = '\u{e20d7}';
RET = _2;
_5 = ('\u{3714f}',);
RET = 17417_i16 as i64;
_1 = 119035990354368430009394657166169994651_u128 as i64;
_5 = ('\u{d3e4c}',);
_5.0 = '\u{73bba}';
_12 = 15844_u16 as f64;
_10 = !RET;
Goto(bb3)
}
bb3 = {
_2 = !_10;
RET = !_10;
RET = _10 >> _1;
_10 = 90_i8 as i64;
_2 = 10362414295545822404_usize as i64;
_1 = !_10;
_5.0 = '\u{d0255}';
_5.0 = '\u{1166f}';
_14 = [139638867718752459676975442571143947050_u128,99014744238932346466040052357562394480_u128];
_5 = ('\u{c30cc}',);
_14 = [68011001999879373515923293024908787642_u128,297781607750494488467860247996211785948_u128];
_12 = (-92_i8) as f64;
_10 = -RET;
_5.0 = '\u{29eaf}';
_10 = 44272517383184487411487036645789580750_i128 as i64;
_2 = RET;
_15 = (-8_i8) | (-50_i8);
_14 = [325946001966406803232899022702372991335_u128,237211129738120328166222304013383702100_u128];
RET = _2 + _2;
_17 = &_3;
_6.1 = 190_u8 + 61_u8;
_5.0 = '\u{36929}';
RET = _10 >> _15;
_12 = 19586_i16 as f64;
_1 = !_2;
_6.1 = 41_u8;
Call(_4 = fn16(RET, _15, _2, _15, _6.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = Move(_4);
_1 = RET | _10;
_10 = !_1;
_6.1 = 253_u8 | 96_u8;
_4 = Move(_3);
_17 = &_3;
_3 = Move(_4);
_14 = [83822186179008775042701938995546874613_u128,299904661041230035981320250331826072940_u128];
_16 = [2453436993_u32,941250348_u32,3992167511_u32,3823330046_u32,2246838904_u32,2704959290_u32,514463183_u32];
_10 = RET & _2;
_4 = Move(_3);
_6.1 = !84_u8;
_14 = [234183822358797490956245940963990062213_u128,56964353091367888911470567548862125404_u128];
_15 = (-23_i8);
RET = _1 >> _1;
_16 = [2985135423_u32,1918479357_u32,3574661141_u32,2474061884_u32,337968769_u32,97201858_u32,311157262_u32];
RET = _2 - _2;
_3 = Move(_4);
_14 = [9720235239522838800121973214077000126_u128,68434579173359422243830174398778278229_u128];
match _15 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768211433 => bb12,
_ => bb11
}
}
bb5 = {
_2 = !_10;
RET = !_10;
RET = _10 >> _1;
_10 = 90_i8 as i64;
_2 = 10362414295545822404_usize as i64;
_1 = !_10;
_5.0 = '\u{d0255}';
_5.0 = '\u{1166f}';
_14 = [139638867718752459676975442571143947050_u128,99014744238932346466040052357562394480_u128];
_5 = ('\u{c30cc}',);
_14 = [68011001999879373515923293024908787642_u128,297781607750494488467860247996211785948_u128];
_12 = (-92_i8) as f64;
_10 = -RET;
_5.0 = '\u{29eaf}';
_10 = 44272517383184487411487036645789580750_i128 as i64;
_2 = RET;
_15 = (-8_i8) | (-50_i8);
_14 = [325946001966406803232899022702372991335_u128,237211129738120328166222304013383702100_u128];
RET = _2 + _2;
_17 = &_3;
_6.1 = 190_u8 + 61_u8;
_5.0 = '\u{36929}';
RET = _10 >> _15;
_12 = 19586_i16 as f64;
_1 = !_2;
_6.1 = 41_u8;
Call(_4 = fn16(RET, _15, _2, _15, _6.1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
RET = !_1;
_1 = -_2;
_1 = _2;
_6.1 = 40_u8 - 199_u8;
RET = !_2;
RET = !_2;
RET = _1;
_5 = ('\u{ded85}',);
_5.0 = '\u{e20d7}';
RET = _2;
_5 = ('\u{3714f}',);
RET = 17417_i16 as i64;
_1 = 119035990354368430009394657166169994651_u128 as i64;
_5 = ('\u{d3e4c}',);
_5.0 = '\u{73bba}';
_12 = 15844_u16 as f64;
_10 = !RET;
Goto(bb3)
}
bb7 = {
_2 = 3270744629253341282_usize as i64;
RET = _1;
_2 = RET & _1;
RET = -_2;
RET = 6312460907242777030_u64 as i64;
_1 = _2;
_1 = 1804406749_u32 as i64;
_2 = -RET;
RET = !_2;
RET = _1 & _1;
RET = _1;
RET = -_1;
RET = _2 - _2;
_2 = !RET;
_2 = RET - _1;
_2 = (-15329_i16) as i64;
RET = _2 & _1;
_2 = -RET;
_2 = (-120_i8) as i64;
RET = _2 ^ _1;
_1 = !RET;
RET = -_1;
_1 = 15455600468640641_u64 as i64;
_2 = !RET;
RET = -_2;
_1 = RET;
Call(_5.0 = fn9(RET, RET, RET, _2, _1, RET, _1, RET, _1), ReturnTo(bb2), UnwindUnreachable())
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
RET = _12 as i64;
_2 = !_10;
_14 = [100555825902983366050263741674651828415_u128,154507344668169572546525591697298248091_u128];
RET = _1 * _1;
_1 = RET;
_18 = (-75_isize);
_17 = &_3;
Goto(bb13)
}
bb13 = {
_6.1 = 158_u8 * 63_u8;
_10 = _1 + RET;
_15 = 129968216413950298433917055095962360737_u128 as i8;
_20 = !57828_u16;
_21.1.2 = -_12;
_21.0.0 = _18 as u8;
_21.1.1.fld0 = _20 ^ _20;
_6.1 = 5841264743601353120_usize as u8;
_21.1.0 = _6.1;
_5 = ('\u{a4e6d}',);
match _18 {
0 => bb12,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607431768211381 => bb18,
_ => bb17
}
}
bb14 = {
RET = !_1;
_1 = -_2;
_1 = _2;
_6.1 = 40_u8 - 199_u8;
RET = !_2;
RET = !_2;
RET = _1;
_5 = ('\u{ded85}',);
_5.0 = '\u{e20d7}';
RET = _2;
_5 = ('\u{3714f}',);
RET = 17417_i16 as i64;
_1 = 119035990354368430009394657166169994651_u128 as i64;
_5 = ('\u{d3e4c}',);
_5.0 = '\u{73bba}';
_12 = 15844_u16 as f64;
_10 = !RET;
Goto(bb3)
}
bb15 = {
_2 = 3270744629253341282_usize as i64;
RET = _1;
_2 = RET & _1;
RET = -_2;
RET = 6312460907242777030_u64 as i64;
_1 = _2;
_1 = 1804406749_u32 as i64;
_2 = -RET;
RET = !_2;
RET = _1 & _1;
RET = _1;
RET = -_1;
RET = _2 - _2;
_2 = !RET;
_2 = RET - _1;
_2 = (-15329_i16) as i64;
RET = _2 & _1;
_2 = -RET;
_2 = (-120_i8) as i64;
RET = _2 ^ _1;
_1 = !RET;
RET = -_1;
_1 = 15455600468640641_u64 as i64;
_2 = !RET;
RET = -_2;
_1 = RET;
Call(_5.0 = fn9(RET, RET, RET, _2, _1, RET, _1, RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
_3 = Move(_4);
_1 = RET | _10;
_10 = !_1;
_6.1 = 253_u8 | 96_u8;
_4 = Move(_3);
_17 = &_3;
_3 = Move(_4);
_14 = [83822186179008775042701938995546874613_u128,299904661041230035981320250331826072940_u128];
_16 = [2453436993_u32,941250348_u32,3992167511_u32,3823330046_u32,2246838904_u32,2704959290_u32,514463183_u32];
_10 = RET & _2;
_4 = Move(_3);
_6.1 = !84_u8;
_14 = [234183822358797490956245940963990062213_u128,56964353091367888911470567548862125404_u128];
_15 = (-23_i8);
RET = _1 >> _1;
_16 = [2985135423_u32,1918479357_u32,3574661141_u32,2474061884_u32,337968769_u32,97201858_u32,311157262_u32];
RET = _2 - _2;
_3 = Move(_4);
_14 = [9720235239522838800121973214077000126_u128,68434579173359422243830174398778278229_u128];
match _15 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768211433 => bb12,
_ => bb11
}
}
bb18 = {
_21.1.2 = _12 * _12;
RET = !_10;
Goto(bb19)
}
bb19 = {
Call(_22 = dump_var(7_usize, 15_usize, Move(_15), 1_usize, Move(_1), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64) -> i64 {
mir! {
type RET = i64;
let _10: [usize; 8];
let _11: usize;
let _12: i8;
let _13: u128;
let _14: Adt31;
let _15: ();
let _16: ();
{
RET = _6;
_3 = 41671355783001441161423590074258988248_u128 as i64;
_11 = 6_usize;
_9 = _7;
_7 = _8;
_8 = _9 ^ _9;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6 = 201_u8 as i64;
_10[_11] = _11;
_7 = 12260159314610577081_u64 as i64;
_10[_11] = !_11;
_11 = _10[_11];
_13 = 8948053783233499590043178327598368148_u128 >> _8;
_7 = '\u{257c9}' as i64;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_4 = !RET;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(8_usize, 7_usize, Move(_7), 8_usize, Move(_8), 10_usize, Move(_10), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(8_usize, 6_usize, Move(_6), 2_usize, Move(_2), 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64) -> char {
mir! {
type RET = char;
let _10: isize;
let _11: Adt68;
let _12: isize;
let _13: &'static Adt42;
let _14: i16;
let _15: *const Adt22;
let _16: f64;
let _17: char;
let _18: Adt30;
let _19: *const &'static [u32; 7];
let _20: [bool; 1];
let _21: *const i32;
let _22: f64;
let _23: &'static (&'static Adt42, u8);
let _24: [usize; 1];
let _25: u32;
let _26: isize;
let _27: [u64; 1];
let _28: i128;
let _29: ([bool; 1], (u8, Adt22, f64), i16, (u64,));
let _30: char;
let _31: *mut i128;
let _32: *const (u8, Adt22, f64);
let _33: [u32; 7];
let _34: char;
let _35: ();
let _36: ();
{
_8 = _1 & _1;
RET = '\u{59192}';
_4 = !_2;
_8 = (-12780_i16) as i64;
_4 = -_5;
_3 = -_6;
_1 = -_5;
_6 = 22667_i16 as i64;
_9 = !_2;
_2 = _6;
_1 = _6;
_8 = _5 | _6;
_12 = !9223372036854775807_isize;
_4 = (-45_i8) as i64;
_6 = _5;
RET = '\u{ce865}';
RET = '\u{92ff6}';
_10 = 115352977539262611438848160597373322208_i128 as isize;
Goto(bb1)
}
bb1 = {
_4 = _3 ^ _2;
_8 = _3 & _6;
RET = '\u{fe26f}';
_2 = _9 ^ _4;
_5 = !_4;
RET = '\u{180df}';
_5 = _2 >> _3;
Goto(bb2)
}
bb2 = {
_8 = -_5;
_12 = _5 as isize;
_5 = _9;
_10 = _12;
_9 = !_8;
_16 = 2247292191_u32 as f64;
_16 = 1542177293_u32 as f64;
_17 = RET;
_18 = Adt30::Variant0 { fld0: 1_usize,fld1: 38559_u16,fld2: 9297942396004807721_u64,fld3: _8 };
_3 = _9;
place!(Field::<u64>(Variant(_18, 0), 2)) = 798105580129811147_u64;
place!(Field::<i64>(Variant(_18, 0), 3)) = (-2481_i16) as i64;
place!(Field::<u16>(Variant(_18, 0), 1)) = 18391_u16 << _5;
match Field::<u64>(Variant(_18, 0), 2) {
0 => bb1,
1 => bb3,
2 => bb4,
798105580129811147 => bb6,
_ => bb5
}
}
bb3 = {
_4 = _3 ^ _2;
_8 = _3 & _6;
RET = '\u{fe26f}';
_2 = _9 ^ _4;
_5 = !_4;
RET = '\u{180df}';
_5 = _2 >> _3;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_8 = !_9;
place!(Field::<usize>(Variant(_18, 0), 0)) = 14185581194286946295_usize;
_3 = Field::<u64>(Variant(_18, 0), 2) as i64;
RET = _17;
_18 = Adt30::Variant0 { fld0: 9608906880148124538_usize,fld1: 30657_u16,fld2: 10420703837766017477_u64,fld3: _4 };
_1 = 78_i8 as i64;
_8 = -_7;
_8 = Field::<i64>(Variant(_18, 0), 3);
_6 = _2 >> _10;
_18 = Adt30::Variant0 { fld0: 0_usize,fld1: 36569_u16,fld2: 17556922259931279662_u64,fld3: _2 };
place!(Field::<u64>(Variant(_18, 0), 2)) = 3237096469365167967_u64;
_2 = _9 & _3;
place!(Field::<usize>(Variant(_18, 0), 0)) = !2368785419506208904_usize;
_1 = !_4;
_12 = !_10;
_14 = 32219_i16 & 14039_i16;
_22 = (-127246421417267659605010739370305057847_i128) as f64;
_16 = _22;
_16 = _22 * _22;
_3 = _6 & Field::<i64>(Variant(_18, 0), 3);
_3 = _6 ^ _6;
_18 = Adt30::Variant0 { fld0: 3_usize,fld1: 42152_u16,fld2: 392608869260429669_u64,fld3: _3 };
_3 = !Field::<i64>(Variant(_18, 0), 3);
_2 = Field::<i64>(Variant(_18, 0), 3);
_7 = _5 & Field::<i64>(Variant(_18, 0), 3);
_1 = _9;
place!(Field::<u64>(Variant(_18, 0), 2)) = !11315594635755163649_u64;
_8 = -_1;
Goto(bb7)
}
bb7 = {
_9 = Field::<i64>(Variant(_18, 0), 3);
place!(Field::<u16>(Variant(_18, 0), 1)) = !52639_u16;
_8 = _16 as i64;
_7 = _2 & _3;
_10 = true as isize;
_4 = _9;
place!(Field::<u64>(Variant(_18, 0), 2)) = 10590649830918207815_u64;
_1 = -_7;
_20 = [true];
_18 = Adt30::Variant0 { fld0: 2_usize,fld1: 35943_u16,fld2: 6078620217008025902_u64,fld3: _1 };
_18 = Adt30::Variant0 { fld0: 7_usize,fld1: 46757_u16,fld2: 15959888654471966639_u64,fld3: _1 };
place!(Field::<u16>(Variant(_18, 0), 1)) = !18493_u16;
_20 = [true];
_2 = Field::<i64>(Variant(_18, 0), 3) - _7;
_5 = _12 as i64;
RET = _17;
_7 = _1 >> Field::<i64>(Variant(_18, 0), 3);
_14 = !(-23376_i16);
_24 = [4_usize];
_1 = _10 as i64;
Call(_18 = fn10(_2, _24, _7, _2, _2, _3, _7, _7, _2, _7, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12 = Field::<usize>(Variant(_18, 0), 0) as isize;
_27 = [Field::<u64>(Variant(_18, 0), 2)];
Call(_21 = fn12(), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9 = -_7;
_26 = _12 * _10;
_20 = [false];
Goto(bb10)
}
bb10 = {
place!(Field::<usize>(Variant(_18, 0), 0)) = 2136014103_i32 as usize;
_29.3 = (Field::<u64>(Variant(_18, 0), 2),);
_29.3.0 = Field::<u64>(Variant(_18, 0), 2) ^ Field::<u64>(Variant(_18, 0), 2);
place!(Field::<u64>(Variant(_18, 0), 2)) = 86_i8 as u64;
_25 = _12 as u32;
_15 = core::ptr::addr_of!(_29.1.1);
_4 = !Field::<i64>(Variant(_18, 0), 3);
_29.1.2 = _22 - _16;
_2 = _3 ^ _4;
place!(Field::<u64>(Variant(_18, 0), 2)) = _29.3.0 + _29.3.0;
_3 = Field::<u64>(Variant(_18, 0), 2) as i64;
(*_15) = Adt22 { fld0: Field::<u16>(Variant(_18, 0), 1) };
Goto(bb11)
}
bb11 = {
_1 = _7;
_30 = _17;
RET = _17;
SetDiscriminant(_18, 1);
_30 = _17;
_28 = (-98359850575135314263616425836374943144_i128);
(*_15) = Adt22 { fld0: 10820_u16 };
_29.1.2 = _16 - _16;
place!(Field::<(u8, Adt22, f64)>(Variant(_18, 1), 1)).0 = 194_u8 >> _2;
_6 = _29.1.2 as i64;
place!(Field::<isize>(Variant(_18, 1), 2)) = _26;
_3 = _1;
_18 = Adt30::Variant0 { fld0: 3_usize,fld1: (*_15).fld0,fld2: _29.3.0,fld3: _5 };
RET = _17;
_29.1.1 = Adt22 { fld0: Field::<u16>(Variant(_18, 0), 1) };
_25 = 1340178429_u32;
_22 = (-120_i8) as f64;
_26 = _28 as isize;
_2 = _9 << _9;
match (*_15).fld0 {
0 => bb4,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
10820 => bb18,
_ => bb17
}
}
bb12 = {
_4 = _3 ^ _2;
_8 = _3 & _6;
RET = '\u{fe26f}';
_2 = _9 ^ _4;
_5 = !_4;
RET = '\u{180df}';
_5 = _2 >> _3;
Goto(bb2)
}
bb13 = {
_9 = -_7;
_26 = _12 * _10;
_20 = [false];
Goto(bb10)
}
bb14 = {
_12 = Field::<usize>(Variant(_18, 0), 0) as isize;
_27 = [Field::<u64>(Variant(_18, 0), 2)];
Call(_21 = fn12(), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_9 = Field::<i64>(Variant(_18, 0), 3);
place!(Field::<u16>(Variant(_18, 0), 1)) = !52639_u16;
_8 = _16 as i64;
_7 = _2 & _3;
_10 = true as isize;
_4 = _9;
place!(Field::<u64>(Variant(_18, 0), 2)) = 10590649830918207815_u64;
_1 = -_7;
_20 = [true];
_18 = Adt30::Variant0 { fld0: 2_usize,fld1: 35943_u16,fld2: 6078620217008025902_u64,fld3: _1 };
_18 = Adt30::Variant0 { fld0: 7_usize,fld1: 46757_u16,fld2: 15959888654471966639_u64,fld3: _1 };
place!(Field::<u16>(Variant(_18, 0), 1)) = !18493_u16;
_20 = [true];
_2 = Field::<i64>(Variant(_18, 0), 3) - _7;
_5 = _12 as i64;
RET = _17;
_7 = _1 >> Field::<i64>(Variant(_18, 0), 3);
_14 = !(-23376_i16);
_24 = [4_usize];
_1 = _10 as i64;
Call(_18 = fn10(_2, _24, _7, _2, _2, _3, _7, _7, _2, _7, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_8 = !_9;
place!(Field::<usize>(Variant(_18, 0), 0)) = 14185581194286946295_usize;
_3 = Field::<u64>(Variant(_18, 0), 2) as i64;
RET = _17;
_18 = Adt30::Variant0 { fld0: 9608906880148124538_usize,fld1: 30657_u16,fld2: 10420703837766017477_u64,fld3: _4 };
_1 = 78_i8 as i64;
_8 = -_7;
_8 = Field::<i64>(Variant(_18, 0), 3);
_6 = _2 >> _10;
_18 = Adt30::Variant0 { fld0: 0_usize,fld1: 36569_u16,fld2: 17556922259931279662_u64,fld3: _2 };
place!(Field::<u64>(Variant(_18, 0), 2)) = 3237096469365167967_u64;
_2 = _9 & _3;
place!(Field::<usize>(Variant(_18, 0), 0)) = !2368785419506208904_usize;
_1 = !_4;
_12 = !_10;
_14 = 32219_i16 & 14039_i16;
_22 = (-127246421417267659605010739370305057847_i128) as f64;
_16 = _22;
_16 = _22 * _22;
_3 = _6 & Field::<i64>(Variant(_18, 0), 3);
_3 = _6 ^ _6;
_18 = Adt30::Variant0 { fld0: 3_usize,fld1: 42152_u16,fld2: 392608869260429669_u64,fld3: _3 };
_3 = !Field::<i64>(Variant(_18, 0), 3);
_2 = Field::<i64>(Variant(_18, 0), 3);
_7 = _5 & Field::<i64>(Variant(_18, 0), 3);
_1 = _9;
place!(Field::<u64>(Variant(_18, 0), 2)) = !11315594635755163649_u64;
_8 = -_1;
Goto(bb7)
}
bb17 = {
Return()
}
bb18 = {
_29.2 = _1 as i16;
_12 = -_10;
_14 = _29.2 - _29.2;
_29.1.2 = _16 - _22;
_12 = _10;
Goto(bb19)
}
bb19 = {
Call(_35 = dump_var(9_usize, 9_usize, Move(_9), 28_usize, Move(_28), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(9_usize, 20_usize, Move(_20), 30_usize, Move(_30), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_35 = dump_var(9_usize, 24_usize, Move(_24), 27_usize, Move(_27), 36_usize, _36, 36_usize, _36), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: i64,mut _2: [usize; 1],mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64,mut _10: i64,mut _11: i64) -> Adt30 {
mir! {
type RET = Adt30;
let _12: ((&'static Adt39,), *const *mut isize);
let _13: [bool; 1];
let _14: ((u64,), [u32; 7], u128, isize);
let _15: bool;
let _16: [i16; 7];
let _17: Adt22;
let _18: ((&'static Adt39,), *const *mut isize);
let _19: char;
let _20: Adt42;
let _21: f64;
let _22: u64;
let _23: [i16; 7];
let _24: i8;
let _25: &'static i64;
let _26: bool;
let _27: ();
let _28: ();
{
_9 = _10 + _7;
_8 = _9;
_11 = _1 << _3;
_11 = _7 | _5;
_10 = _9 ^ _9;
Call(_10 = fn11(_8, _8, _8, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = _10;
_5 = 9223372036854775807_isize as i64;
_2 = [0_usize];
_15 = true;
_7 = _8 * _1;
RET = Adt30::Variant0 { fld0: 4064547883432687944_usize,fld1: 53393_u16,fld2: 1277066448361118190_u64,fld3: _8 };
_6 = !_8;
_11 = !_8;
_11 = -_9;
_14.2 = !150718883593418466777447847605062516482_u128;
_13 = [_15];
_14.0 = (12215947012446643574_u64,);
_4 = _3;
Goto(bb2)
}
bb2 = {
place!(Field::<u16>(Variant(RET, 0), 1)) = '\u{b0613}' as u16;
_16 = [(-20985_i16),(-21899_i16),30713_i16,(-16691_i16),21006_i16,9232_i16,12611_i16];
_15 = false & false;
_17.fld0 = !Field::<u16>(Variant(RET, 0), 1);
_10 = _3 - Field::<i64>(Variant(RET, 0), 3);
_15 = _8 > _6;
_19 = '\u{37632}';
place!(Field::<u64>(Variant(RET, 0), 2)) = !_14.0.0;
_17 = Adt22 { fld0: Field::<u16>(Variant(RET, 0), 1) };
_3 = _10 ^ _10;
place!(Field::<i64>(Variant(RET, 0), 3)) = _4;
_2 = [7_usize];
_7 = -_9;
_19 = '\u{4ba1b}';
RET = Adt30::Variant0 { fld0: 9416639538675390678_usize,fld1: _17.fld0,fld2: _14.0.0,fld3: _7 };
_16 = [(-28603_i16),(-6997_i16),9129_i16,19605_i16,18039_i16,29633_i16,15824_i16];
_4 = _11;
_24 = 14_i8 - (-9_i8);
_25 = &_5;
match Field::<u64>(Variant(RET, 0), 2) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
12215947012446643574 => bb9,
_ => bb8
}
}
bb3 = {
_11 = _10;
_5 = 9223372036854775807_isize as i64;
_2 = [0_usize];
_15 = true;
_7 = _8 * _1;
RET = Adt30::Variant0 { fld0: 4064547883432687944_usize,fld1: 53393_u16,fld2: 1277066448361118190_u64,fld3: _8 };
_6 = !_8;
_11 = !_8;
_11 = -_9;
_14.2 = !150718883593418466777447847605062516482_u128;
_13 = [_15];
_14.0 = (12215947012446643574_u64,);
_4 = _3;
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
_14.3 = !41_isize;
place!(Field::<usize>(Variant(RET, 0), 0)) = 1_usize ^ 5_usize;
_17 = Adt22 { fld0: Field::<u16>(Variant(RET, 0), 1) };
place!(Field::<u16>(Variant(RET, 0), 1)) = 89_u8 as u16;
Goto(bb10)
}
bb10 = {
Call(_27 = dump_var(10_usize, 19_usize, Move(_19), 13_usize, Move(_13), 8_usize, Move(_8), 15_usize, Move(_15)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_27 = dump_var(10_usize, 4_usize, Move(_4), 10_usize, Move(_10), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64) -> i64 {
mir! {
type RET = i64;
let _5: f32;
let _6: ((u64,), [u32; 7], u128, isize);
let _7: f32;
let _8: f32;
let _9: (u16, (u64,));
let _10: u16;
let _11: (i32, f64, i32, u64);
let _12: ();
let _13: ();
{
RET = _4 * _2;
_3 = 1801688141_u32 as i64;
RET = !_2;
_1 = _2;
_5 = 259004249751086113701089557786194977882_u128 as f32;
_1 = !_2;
_5 = 3265818550_u32 as f32;
RET = 2408128785600839068_usize as i64;
_6.1 = [2138841057_u32,1326550122_u32,3797103641_u32,1729038380_u32,4118879124_u32,2714340768_u32,2431020629_u32];
RET = _1;
_3 = _1;
_6.0 = (14131668179827755904_u64,);
_6.1 = [185663045_u32,4194013776_u32,2484586197_u32,2781904037_u32,3373170454_u32,1997323812_u32,943096000_u32];
_2 = RET;
_5 = (-13286_i16) as f32;
_6.0.0 = !9861938499149709977_u64;
_8 = _5;
_6.2 = 72283266638137551131535328960794634503_u128;
_10 = !41587_u16;
_11.2 = 633161821_i32 - (-1206302603_i32);
_11.0 = _11.2 * _11.2;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(11_usize, 10_usize, Move(_10), 3_usize, Move(_3), 13_usize, _13, 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12() -> *const i32 {
mir! {
type RET = *const i32;
let _1: u16;
let _2: (u8,);
let _3: [i16; 7];
let _4: ((u8,), (u8, Adt22, f64));
let _5: char;
let _6: *const i32;
let _7: u16;
let _8: [isize; 8];
let _9: (Adt39, [isize; 8]);
let _10: f32;
let _11: f32;
let _12: (&'static Adt39,);
let _13: i32;
let _14: usize;
let _15: f64;
let _16: isize;
let _17: i64;
let _18: f64;
let _19: ();
let _20: ();
{
_1 = 19364_u16 >> 116173389266242294842523367980531463244_u128;
_1 = 10496_u16 & 2525_u16;
_1 = 4_u8 as u16;
_2 = (174_u8,);
_1 = 62128_u16 >> _2.0;
_2 = (215_u8,);
_2 = (18_u8,);
_2.0 = !208_u8;
_2 = (34_u8,);
_3 = [(-24193_i16),(-17509_i16),21731_i16,(-23309_i16),15147_i16,(-689_i16),26771_i16];
_2.0 = 120_u8 | 150_u8;
_2.0 = 152_u8;
_1 = 63403_u16 ^ 22974_u16;
_4.1.1 = Adt22 { fld0: _1 };
_4.1.1 = Adt22 { fld0: _1 };
_4.1.2 = 7875939677497510432_i64 as f64;
_4.0 = (_2.0,);
_4.1.0 = _4.0.0 - _2.0;
_4.0 = (_2.0,);
_2.0 = (-12_i8) as u8;
match _4.0.0 {
152 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2.0 = _4.0.0 % _4.0.0;
_4.1.1.fld0 = _4.1.2 as u16;
_4.1.1 = Adt22 { fld0: _1 };
_2 = (_4.1.0,);
match _4.0.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
152 => bb10,
_ => bb9
}
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
_4.0 = (_2.0,);
_2 = _4.0;
_3 = [(-22579_i16),(-29554_i16),18578_i16,16481_i16,20529_i16,(-30994_i16),(-24704_i16)];
_4.1.2 = (-9223372036854775808_isize) as f64;
_4.1.1 = Adt22 { fld0: _1 };
_4.0 = _2;
_4.0.0 = !_4.1.0;
_4.1.0 = _2.0 & _4.0.0;
_3 = [(-27985_i16),(-25482_i16),6876_i16,(-26456_i16),(-8119_i16),(-11923_i16),32305_i16];
Goto(bb11)
}
bb11 = {
_4.1.2 = _4.1.1.fld0 as f64;
_1 = !_4.1.1.fld0;
_4.1.2 = 2770092285_u32 as f64;
_4.0 = (_4.1.0,);
_4.0.0 = _4.1.0;
Call(_4.0.0 = fn13(_2, _2.0, _2.0, _4.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_4.0 = (_4.1.0,);
_1 = _4.1.1.fld0;
Goto(bb13)
}
bb13 = {
_2.0 = !_4.1.0;
_2.0 = !_4.0.0;
_5 = '\u{dcddd}';
_4.1.1 = Adt22 { fld0: _1 };
_1 = !_4.1.1.fld0;
_5 = '\u{f312e}';
_4.1.2 = (-121_i8) as f64;
_4.0 = (_2.0,);
_2.0 = 2330554102514975771_usize as u8;
_2.0 = _4.1.0 | _4.0.0;
_4.0 = (_2.0,);
_1 = _4.1.1.fld0 >> _4.1.1.fld0;
_4.1.2 = 17002_i16 as f64;
_5 = '\u{a825}';
_1 = _4.1.1.fld0;
Call(_4.0.0 = core::intrinsics::transmute(_2.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_4.1.0 = !_4.0.0;
_8 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4.1.2 = 12986173670341761248_usize as f64;
_2.0 = _4.0.0 + _4.0.0;
_8 = [49_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),23_isize,9223372036854775807_isize,(-9223372036854775808_isize),28_isize,9223372036854775807_isize];
_9.1 = [110_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),17_isize,(-75_isize),(-29_isize),41_isize];
_4.1.2 = 214856076336615151771817263163013836402_u128 as f64;
_1 = !_4.1.1.fld0;
Goto(bb15)
}
bb15 = {
_4.0 = (_2.0,);
_12.0 = &_9.0;
_5 = '\u{3db66}';
_7 = !_4.1.1.fld0;
_11 = (-157836938195831496101241961133921249095_i128) as f32;
_4.0 = (_2.0,);
_4.0.0 = _2.0;
_8 = _9.1;
_9.1 = _8;
_6 = core::ptr::addr_of!(_13);
_3 = [27478_i16,21628_i16,22410_i16,3095_i16,(-6275_i16),(-1470_i16),23418_i16];
_2.0 = !_4.0.0;
_14 = 2_usize << _4.0.0;
_10 = _11;
_12.0 = &_9.0;
Call(_7 = core::intrinsics::transmute(_4.1.1.fld0), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_13 = _11 as i32;
RET = Move(_6);
_6 = Move(RET);
_6 = core::ptr::addr_of!(_13);
_7 = !_1;
_12.0 = &_9.0;
_4.1.0 = _4.0.0;
RET = Move(_6);
_12.0 = &_9.0;
_15 = _4.1.2 - _4.1.2;
_4.1.2 = -_15;
_4.1.1 = Adt22 { fld0: _7 };
_4.0.0 = _2.0;
_9.1 = _8;
_3 = [27850_i16,10269_i16,(-18449_i16),22063_i16,15129_i16,11864_i16,(-26050_i16)];
_12.0 = &_9.0;
_10 = 156579150354056946630758732852639365706_i128 as f32;
RET = core::ptr::addr_of!(_13);
_17 = -6951135566362712794_i64;
_7 = _1 << _4.1.0;
_11 = _14 as f32;
_4.0.0 = _2.0;
_12.0 = &_9.0;
_14 = _13 as usize;
_13 = !451078155_i32;
_2 = (_4.1.0,);
_18 = _13 as f64;
_8 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-91_isize),(-86_isize),102_isize];
Goto(bb17)
}
bb17 = {
Call(_19 = dump_var(12_usize, 17_usize, Move(_17), 5_usize, Move(_5), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (u8,),mut _2: u8,mut _3: u8,mut _4: (u8, Adt22, f64)) -> u8 {
mir! {
type RET = u8;
let _5: i128;
let _6: *mut f32;
let _7: u128;
let _8: (u64,);
let _9: (u8, Adt22, f64);
let _10: isize;
let _11: ();
let _12: ();
{
RET = !_1.0;
_4.0 = RET;
_4.2 = (-9223372036854775808_isize) as f64;
_5 = -135269021012773621176387201267621978577_i128;
_3 = !RET;
RET = '\u{7d440}' as u8;
_5 = !(-118854035578984955568981020981011479389_i128);
_4.1 = Adt22 { fld0: 24617_u16 };
_1.0 = _2 >> _2;
_4.1 = Adt22 { fld0: 45782_u16 };
_3 = (-27595_i16) as u8;
RET = _1.0 << _1.0;
_5 = -(-133893103970241402949450028093378578669_i128);
_2 = !_1.0;
_2 = 21_i8 as u8;
_5 = (-71696268239431594834012968610132796335_i128);
_1 = (RET,);
_5 = (-4188192534280825248964438042657914653_i128) & 104465093774220852974486887729651220255_i128;
RET = 4034814583261668123_usize as u8;
_4.1.fld0 = 14836_u16 | 61159_u16;
Goto(bb1)
}
bb1 = {
_1 = (_4.0,);
_8 = (15636855761064337917_u64,);
RET = !_1.0;
_4.0 = (-5327709452451998150_i64) as u8;
_4.2 = _8.0 as f64;
_4.1 = Adt22 { fld0: 62325_u16 };
_1 = (_4.0,);
_1.0 = _4.0 >> _2;
_9.1.fld0 = 9072108874300250068_usize as u16;
_4.2 = (-10770_i16) as f64;
_5 = 35815389048338234731325894707790979724_i128;
_9.2 = _1.0 as f64;
_4.1.fld0 = _9.1.fld0;
_4.1.fld0 = !_9.1.fld0;
_4.1.fld0 = _9.1.fld0 ^ _9.1.fld0;
_9.0 = RET;
RET = _5 as u8;
Goto(bb2)
}
bb2 = {
_4.1.fld0 = _9.1.fld0;
_9.2 = _4.2 + _4.2;
_5 = (-24678_i16) as i128;
_4 = (_9.0, _9.1, _9.2);
_8 = (15839276613870690068_u64,);
_4 = (_1.0, _9.1, _9.2);
Call(RET = fn14(_8.0, _2, _1.0, _1, _9.1, _2, _9.2, _4, _4.1.fld0, _4.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = (10780319278319740540_u64,);
_9.1.fld0 = 1550564711_u32 as u16;
_1.0 = RET;
_4.0 = RET * RET;
_4.2 = 107663643391727774383532371808644411177_u128 as f64;
_9.0 = (-9223372036854775808_isize) as u8;
_9 = (_1.0, _4.1, _4.2);
_9.2 = _4.2 * _4.2;
RET = 6_usize as u8;
RET = (-18039_i16) as u8;
_9.2 = _5 as f64;
_7 = 47377085673491777152089612977624355956_u128;
_1.0 = 23547_i16 as u8;
_9.1 = Adt22 { fld0: _4.1.fld0 };
RET = _4.0 + _9.0;
_4.2 = _7 as f64;
Goto(bb4)
}
bb4 = {
Call(_11 = dump_var(13_usize, 2_usize, Move(_2), 1_usize, Move(_1), 7_usize, Move(_7), 12_usize, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u64,mut _2: u8,mut _3: u8,mut _4: (u8,),mut _5: Adt22,mut _6: u8,mut _7: f64,mut _8: (u8, Adt22, f64),mut _9: u16,mut _10: f64) -> u8 {
mir! {
type RET = u8;
let _11: (u64,);
let _12: char;
let _13: (&'static Adt42, u8);
let _14: f32;
let _15: *const *const (*const i64,);
let _16: &'static Adt39;
let _17: f32;
let _18: (char,);
let _19: f32;
let _20: f32;
let _21: *mut &'static [u32; 7];
let _22: char;
let _23: i32;
let _24: &'static Adt42;
let _25: [u64; 1];
let _26: &'static (Adt39, [isize; 8]);
let _27: ();
let _28: ();
{
RET = !_8.0;
_4 = (_8.0,);
_4 = (_8.0,);
_6 = RET;
RET = _8.0;
Call(_2 = core::intrinsics::bswap(_8.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _3;
_4 = (_8.0,);
RET = _3 | _3;
_2 = !_6;
_3 = RET;
Goto(bb2)
}
bb2 = {
_4.0 = _2;
_8 = (_4.0, _5, _10);
_6 = !_4.0;
_7 = 1315951514_i32 as f64;
_8.0 = _3;
RET = _8.0 << _2;
_3 = (-1304237754_i32) as u8;
_1 = 4173348259605610189_u64;
_4 = (_8.0,);
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4173348259605610189 => bb7,
_ => bb6
}
}
bb3 = {
RET = _3;
_4 = (_8.0,);
RET = _3 | _3;
_2 = !_6;
_3 = RET;
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
_10 = _8.2;
_11 = (_1,);
_4 = (_8.0,);
_4 = (_8.0,);
RET = _3;
_6 = _2;
_5 = Adt22 { fld0: _8.1.fld0 };
_11 = (_1,);
_12 = '\u{d3647}';
_12 = '\u{2d113}';
_10 = -_7;
_8.1 = Adt22 { fld0: _5.fld0 };
_12 = '\u{f9692}';
_13.1 = !_8.0;
_6 = _8.0 & _4.0;
_7 = _8.2 - _8.2;
_12 = '\u{b2d7b}';
_4 = (_6,);
_8 = (_13.1, _5, _7);
_4.0 = _8.0 << _6;
_4.0 = _6;
_2 = _6 | _6;
_14 = 18235197159550111913_usize as f32;
_2 = 18081801285779884710273613434533328206_i128 as u8;
_9 = _5.fld0;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
4173348259605610189 => bb8,
_ => bb4
}
}
bb8 = {
_5 = Adt22 { fld0: _9 };
_1 = 57_isize as u64;
_11.0 = _1 * _1;
_8.1 = Adt22 { fld0: _9 };
_17 = -_14;
_5.fld0 = _9;
_3 = _6 ^ _6;
_8.0 = !_3;
_8.0 = (-50713927896580961730754785323646223548_i128) as u8;
_1 = (-9652654_i32) as u64;
_4 = (_3,);
Call(_8.1.fld0 = core::intrinsics::transmute(_5.fld0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = _13.1;
_4 = (_6,);
RET = !_13.1;
_9 = _8.1.fld0 << _11.0;
_1 = _11.0;
_3 = !_2;
_4 = (_13.1,);
_8 = (_13.1, _5, _7);
_13.1 = RET << _8.0;
_14 = -_17;
_6 = !RET;
_9 = 8169872948456728088_i64 as u16;
_11 = (_1,);
Goto(bb10)
}
bb10 = {
_11.0 = _1;
Call(_15 = fn15(_1, _4.0, _5.fld0, _4.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2 = true as u8;
_5.fld0 = !_8.1.fld0;
_6 = !_13.1;
_5.fld0 = _9;
_20 = _7 as f32;
_3 = _8.0 & _6;
_9 = _12 as u16;
_22 = _12;
RET = _13.1 * _3;
_18.0 = _12;
_10 = -_7;
_11 = (_1,);
_8 = (_13.1, _5, _7);
_1 = _11.0;
_23 = _1 as i32;
_2 = _8.0 - RET;
_18 = (_22,);
_4 = (_2,);
_19 = _17 * _17;
_17 = _20;
_13.1 = (-15987_i16) as u8;
Goto(bb12)
}
bb12 = {
Call(_27 = dump_var(14_usize, 18_usize, Move(_18), 23_usize, Move(_23), 3_usize, Move(_3), 22_usize, Move(_22)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_27 = dump_var(14_usize, 12_usize, Move(_12), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: u64,mut _2: u8,mut _3: u16,mut _4: u8) -> *const *const (*const i64,) {
mir! {
type RET = *const *const (*const i64,);
let _5: [u128; 6];
let _6: usize;
let _7: i32;
let _8: u64;
let _9: [i16; 7];
let _10: f64;
let _11: &'static *mut f32;
let _12: *const i64;
let _13: &'static Adt42;
let _14: Adt22;
let _15: bool;
let _16: isize;
let _17: [u64; 1];
let _18: [u8; 4];
let _19: *const &'static [u32; 7];
let _20: bool;
let _21: ((u64,), [u32; 7], u128, isize);
let _22: [u32; 3];
let _23: (Adt39, [isize; 8]);
let _24: &'static &'static *const isize;
let _25: *const (*const i64,);
let _26: f64;
let _27: isize;
let _28: ();
let _29: ();
{
_1 = !11541540290651505481_u64;
_2 = !_4;
Goto(bb1)
}
bb1 = {
_4 = _2;
_4 = !_2;
_4 = _2;
_1 = (-25276_i16) as u64;
Goto(bb2)
}
bb2 = {
_1 = 2063352292_u32 as u64;
Goto(bb3)
}
bb3 = {
_5 = [273501943177202566086999451754442314706_u128,23246493528806604463412223419091797167_u128,93302388364063083275135615947630228639_u128,212008625975421348335891259906107638175_u128,292688715157211507024834575690904286118_u128,280544909084458656503695357362587926035_u128];
_4 = _2;
_4 = !_2;
_1 = 14592915051323727851_u64 ^ 17979085850724349160_u64;
_4 = _2 & _2;
_3 = !58730_u16;
_1 = !2613557238008356622_u64;
_5 = [13266566789196714229338913782710926664_u128,304001541348929554190895612068747681945_u128,57393288450535270319560753424249987793_u128,219650798904510031611483713579197617104_u128,194773335494737590550352470493813149076_u128,102990914539588982748059899741076978944_u128];
_4 = !_2;
_2 = !_4;
_6 = 1508488149352118610_usize - 6326391942649181998_usize;
_3 = 24507_u16 >> _4;
_5 = [313868940252046770550403964889315650843_u128,70968173040592212307620880434464389512_u128,163282409554628238601098704213051035812_u128,258364945998836203020584635094760197347_u128,42346796996567666592403423131745988612_u128,176222121916282593405579772734901945331_u128];
_4 = !_2;
_8 = _1;
_7 = 228272829172409210575334556275133540353_u128 as i32;
_2 = _4 & _4;
_1 = false as u64;
_2 = '\u{c1482}' as u8;
_3 = !35127_u16;
_6 = 11016739896934134857_usize + 17160535463951014757_usize;
_7 = (-2146207818_i32) * (-1080712213_i32);
Goto(bb4)
}
bb4 = {
_9 = [(-23072_i16),(-7486_i16),(-31293_i16),30603_i16,1821_i16,14253_i16,(-19675_i16)];
_6 = _8 as usize;
_6 = !4_usize;
_6 = (-117_i8) as usize;
_7 = 1033705362_i32;
_3 = !48279_u16;
_8 = _1 >> _3;
_6 = !11979095047206082472_usize;
_1 = _8;
_10 = 33805738009350626118670208768214478733_i128 as f64;
_10 = 2901913373_u32 as f64;
_1 = (-13_isize) as u64;
_6 = !1_usize;
_10 = 1374454953411759344_i64 as f64;
_9 = [24649_i16,22012_i16,(-5967_i16),(-23683_i16),(-29960_i16),30476_i16,(-15020_i16)];
match _7 {
0 => bb1,
1 => bb2,
1033705362 => bb5,
_ => bb3
}
}
bb5 = {
_9 = [(-19299_i16),(-3141_i16),(-11651_i16),(-30406_i16),(-23765_i16),(-20230_i16),7675_i16];
_6 = _10 as usize;
_6 = 1_usize;
_4 = _2 * _2;
_6 = !7_usize;
_1 = _8 & _8;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
1033705362 => bb6,
_ => bb4
}
}
bb6 = {
_8 = _10 as u64;
_14.fld0 = _3;
_2 = !_4;
_10 = _7 as f64;
_3 = 290917544435374660601719270820040887023_u128 as u16;
_7 = 81214664_i32 << _2;
_3 = _14.fld0;
_8 = _1;
_14 = Adt22 { fld0: _3 };
_5 = [196334196275652482910141889414428984701_u128,8004013641855477727321748941683294572_u128,218428577264386452273503496192199636981_u128,73911544845327840727133357785681498724_u128,23772358868240554883834144274091320167_u128,66499596575613937188132668685135711305_u128];
Goto(bb7)
}
bb7 = {
_8 = !_1;
_1 = _14.fld0 as u64;
_4 = !_2;
_3 = (-7313_i16) as u16;
_14 = Adt22 { fld0: _3 };
_7 = !1193349074_i32;
_3 = !_14.fld0;
_1 = !_8;
_14.fld0 = !_3;
_1 = 113_isize as u64;
_3 = !_14.fld0;
_10 = 2219045502_u32 as f64;
_3 = !_14.fld0;
Goto(bb8)
}
bb8 = {
_9 = [31538_i16,8521_i16,(-3590_i16),(-25636_i16),24855_i16,30726_i16,(-17266_i16)];
_15 = true ^ true;
_4 = !_2;
_15 = !true;
_2 = _4;
_6 = 3_isize as usize;
_10 = 104191769590549993010021170254262814009_u128 as f64;
_8 = _1 & _1;
_8 = _1 + _1;
_7 = !(-1695724941_i32);
_17 = [_8];
_10 = _14.fld0 as f64;
_3 = _14.fld0;
_7 = (-1149410051_i32);
_14.fld0 = _3 * _3;
_10 = _7 as f64;
_1 = (-104869897208763041424035134170387484105_i128) as u64;
_18 = [_2,_4,_4,_4];
_16 = 9223372036854775807_isize ^ 5_isize;
_15 = false;
match _7 {
0 => bb1,
1 => bb4,
340282366920938463463374607430618801405 => bb9,
_ => bb7
}
}
bb9 = {
_18 = [_2,_4,_4,_2];
_15 = true;
_2 = _4 & _4;
_5 = [145445622657939431429899088933773317280_u128,241551110243034079636486299610998438505_u128,333837966884095073351822864752529867862_u128,281164286092405135803667266596128388692_u128,187606528409343951752555916576120179865_u128,250323706090196068492354394322304342516_u128];
_2 = _4 | _4;
_1 = _8 ^ _8;
_3 = 5134_i16 as u16;
_1 = !_8;
_14 = Adt22 { fld0: _3 };
_1 = !_8;
_16 = 9223372036854775807_isize;
_4 = !_2;
_4 = !_2;
_21.0.0 = (-8_i8) as u64;
_7 = 824568076_i32 << _21.0.0;
_2 = _7 as u8;
_8 = !_21.0.0;
_21.3 = _4 as isize;
_16 = _21.3;
_21.0.0 = _8 * _8;
_8 = _1 + _1;
_14.fld0 = _3 << _2;
_17 = [_21.0.0];
_14 = Adt22 { fld0: _3 };
_21.2 = 301741795974622438592638717151719753291_u128 ^ 278128468290396103084220864882814157629_u128;
_10 = _21.3 as f64;
Goto(bb10)
}
bb10 = {
_7 = 178174525_i32;
_4 = _15 as u8;
_20 = !_15;
_9 = [6150_i16,24447_i16,(-23947_i16),19091_i16,(-25901_i16),(-7451_i16),(-17584_i16)];
_1 = 20763_i16 as u64;
_5 = [_21.2,_21.2,_21.2,_21.2,_21.2,_21.2];
_21.3 = _16 & _16;
Call(_21.0.0 = core::intrinsics::bswap(_1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_21.0.0 = 1_i8 as u64;
_18 = [_4,_2,_4,_4];
_21.0 = (_1,);
_2 = _7 as u8;
_17 = [_8];
_7 = 281639892_i32 | 1819668650_i32;
_21.2 = _8 as u128;
_22 = [315666750_u32,2136402076_u32,707944545_u32];
_17 = [_8];
_6 = !3_usize;
_14 = Adt22 { fld0: _3 };
_6 = 5026227834230344713_usize;
Goto(bb12)
}
bb12 = {
RET = core::ptr::addr_of!(_25);
_15 = _20 ^ _20;
match _6 {
0 => bb6,
1 => bb2,
2 => bb9,
3 => bb7,
4 => bb8,
5026227834230344713 => bb14,
_ => bb13
}
}
bb13 = {
_1 = 2063352292_u32 as u64;
Goto(bb3)
}
bb14 = {
_17 = [_8];
_16 = _21.3 >> _3;
_7 = !(-1935764094_i32);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(15_usize, 22_usize, Move(_22), 5_usize, Move(_5), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(15_usize, 17_usize, Move(_17), 9_usize, Move(_9), 15_usize, Move(_15), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i64,mut _2: i8,mut _3: i64,mut _4: i8,mut _5: u8) -> *const *mut isize {
mir! {
type RET = *const *mut isize;
let _6: f32;
let _7: Adt39;
let _8: &'static (u8, Adt22, f64);
let _9: [usize; 1];
let _10: Adt31;
let _11: [u128; 6];
let _12: isize;
let _13: bool;
let _14: f32;
let _15: &'static [u32; 7];
let _16: ();
let _17: ();
{
_5 = !102_u8;
_3 = !_1;
_4 = (-4067_i16) as i8;
_4 = 14903_i16 as i8;
_5 = !38_u8;
_6 = 26283_u16 as f32;
_1 = 15848194660591076544_u64 as i64;
_5 = 241_u8;
_3 = _1 - _1;
_4 = 2_usize as i8;
_3 = _1 - _1;
_6 = 13058542631016622141_usize as f32;
_6 = _5 as f32;
_3 = _1;
_6 = _4 as f32;
_3 = -_1;
_6 = (-21174_i16) as f32;
_6 = 114_isize as f32;
_2 = 138612638244702220764519559241776634078_u128 as i8;
_6 = _4 as f32;
_5 = !22_u8;
_5 = !174_u8;
Goto(bb1)
}
bb1 = {
_2 = (-582347077_i32) as i8;
_4 = 321336425111194864478016724038888721345_u128 as i8;
_9 = [5342744345023666992_usize];
_4 = 1266385139_i32 as i8;
_4 = _2;
_3 = _1 >> _5;
_4 = !_2;
_6 = 3812968146_u32 as f32;
_5 = 15_u8;
_9 = [4_usize];
_6 = (-1512891852_i32) as f32;
_11 = [316171020643883834540040741651169752284_u128,220755954702308496722830426246328921849_u128,1839402855747376916019080179091046557_u128,289731116603719019604316092401962041431_u128,312540526118288292804019862902964738804_u128,109073507214039571019516569511429288054_u128];
_11 = [337263873874651989635842858354893041544_u128,223722816878598341130158481207792940216_u128,304550987396827956389144662087922077354_u128,161516767953110665638688594081430401627_u128,107585545527623946315805297823634755536_u128,112158195324721729629812357042450157232_u128];
_2 = _4;
_4 = !_2;
match _5 {
0 => bb2,
1 => bb3,
15 => bb5,
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
_3 = _1;
_1 = 9718_i16 as i64;
_11 = [320125472374181247949233069444654088144_u128,144892136831623334699052200086425981511_u128,173399039327363652706471074457023395226_u128,1377205883333423906924085421054264722_u128,53848514163965667075098627212117428991_u128,140484524788490434165955206710933809899_u128];
_2 = !_4;
_2 = !_4;
_1 = _3 - _3;
_13 = !true;
_3 = _1 * _1;
_3 = _1;
_13 = true;
Call(RET = fn17(_4, _11, _13, _5, _3, _9, _1, _9, _11, _4, _2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _4 as i64;
_1 = _3 - _3;
_4 = _2;
_12 = !(-102_isize);
_14 = -_6;
_13 = true ^ false;
_1 = _13 as i64;
_5 = 18_u8;
_12 = (-119_isize);
_4 = _2 - _2;
_6 = -_14;
_2 = _4;
_3 = 2500909525386144764_u64 as i64;
Goto(bb7)
}
bb7 = {
Call(_16 = dump_var(16_usize, 12_usize, Move(_12), 1_usize, Move(_1), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i8,mut _2: [u128; 6],mut _3: bool,mut _4: u8,mut _5: i64,mut _6: [usize; 1],mut _7: i64,mut _8: [usize; 1],mut _9: [u128; 6],mut _10: i8,mut _11: i8) -> *const *mut isize {
mir! {
type RET = *const *mut isize;
let _12: bool;
let _13: &'static Adt42;
let _14: *const (*const i64,);
let _15: (u8, Adt22, f64);
let _16: ((&'static Adt39,), *const *mut isize);
let _17: f64;
let _18: bool;
let _19: Adt22;
let _20: (*const Adt22, u32, ((u8,), (u8, Adt22, f64)));
let _21: [u128; 6];
let _22: (u8, Adt22, f64);
let _23: (Adt39, [isize; 8]);
let _24: i32;
let _25: *const &'static [u32; 7];
let _26: bool;
let _27: u128;
let _28: (Adt39, [isize; 8]);
let _29: bool;
let _30: f64;
let _31: [u8; 4];
let _32: char;
let _33: isize;
let _34: usize;
let _35: char;
let _36: i16;
let _37: isize;
let _38: f32;
let _39: *const isize;
let _40: i8;
let _41: isize;
let _42: f32;
let _43: usize;
let _44: char;
let _45: (i32, f64, i32, u64);
let _46: *const i32;
let _47: (char,);
let _48: char;
let _49: *const isize;
let _50: i32;
let _51: &'static (u8, Adt22, f64);
let _52: [u64; 1];
let _53: [u128; 2];
let _54: char;
let _55: [i64; 3];
let _56: &'static &'static *const isize;
let _57: *mut isize;
let _58: usize;
let _59: isize;
let _60: char;
let _61: *mut &'static [u32; 7];
let _62: ();
let _63: ();
{
_10 = _1 + _1;
_3 = _10 >= _11;
_9 = _2;
_1 = !_10;
_4 = 122_u8;
_10 = 6267013919795151994_usize as i8;
_7 = 2354503796_u32 as i64;
_2 = [261861684319562194207197366199795375285_u128,185848393019303479216181872335427166322_u128,146295056371347313214112201268375969233_u128,77548381133207175485794681134740654292_u128,87673017645863049493642643688411443219_u128,112426159578271753336937537077808237757_u128];
_8 = [7288681109986197798_usize];
_4 = 121_u8;
_8 = [6265890417070327049_usize];
_12 = !_3;
_4 = !0_u8;
_7 = _5 * _5;
_4 = 28_u8 >> _7;
_10 = _1 - _1;
_12 = !_3;
_8 = _6;
_12 = _3;
_12 = _3 | _3;
_8 = [4_usize];
_3 = _5 >= _7;
_11 = !_10;
_1 = 9223372036854775807_isize as i8;
_5 = _12 as i64;
_8 = [3_usize];
_1 = _10;
_8 = [2_usize];
Goto(bb1)
}
bb1 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
Goto(bb2)
}
bb2 = {
_15.0 = 1047612398_i32 as u8;
_5 = _7 | _7;
_2 = _9;
_11 = _1 >> _1;
_1 = _11;
_15.1.fld0 = 9037355119911318673_u64 as u16;
_10 = _1 | _11;
_15.2 = _5 as f64;
_2 = _9;
_15.2 = (-1211093331_i32) as f64;
Goto(bb3)
}
bb3 = {
_3 = _4 >= _4;
_5 = '\u{fbb4}' as i64;
_5 = -_7;
_15.1 = Adt22 { fld0: 35291_u16 };
_4 = _15.0;
_9 = [263553712390285021891399379641183063398_u128,74481094435953456620179139723327476140_u128,168047922053691898967766412057311281845_u128,60550613916350881066008712921307615992_u128,11103586948147223664800514204280707210_u128,105869049316905296587138731953950631163_u128];
_1 = !_11;
_15.0 = '\u{45596}' as u8;
_20.0 = core::ptr::addr_of!(_15.1);
_20.2.0 = (_4,);
_15.2 = 2665064857_u32 as f64;
_17 = (-25606_i16) as f64;
_11 = _15.1.fld0 as i8;
_6 = _8;
_9 = _2;
_20.2.1.2 = _15.2;
_20.1 = (-106735947_i32) as u32;
_20.2.1 = (_15.0, _15.1, _15.2);
_9 = [96380872037274037195324858476743895101_u128,36705127527185438733541785676777207519_u128,20159339233021045905641380011451240490_u128,137853168563281152855701456892525246436_u128,281007872237705741714821083116991582010_u128,321687126553619839492676204836455903582_u128];
_22 = (_20.2.1.0, _15.1, _15.2);
match _15.1.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
35291 => bb9,
_ => bb8
}
}
bb4 = {
_15.0 = 1047612398_i32 as u8;
_5 = _7 | _7;
_2 = _9;
_11 = _1 >> _1;
_1 = _11;
_15.1.fld0 = 9037355119911318673_u64 as u16;
_10 = _1 | _11;
_15.2 = _5 as f64;
_2 = _9;
_15.2 = (-1211093331_i32) as f64;
Goto(bb3)
}
bb5 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
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
_20.2.1 = (_20.2.0.0, _15.1, _15.2);
_22.1.fld0 = _20.2.1.1.fld0;
_19.fld0 = _22.1.fld0;
_15 = _22;
_1 = _11;
_15.1 = Adt22 { fld0: _20.2.1.1.fld0 };
_15 = _22;
_20.2.1.0 = !_15.0;
_17 = _22.2;
_1 = _10 - _10;
_20.2.1.0 = _15.0 ^ _4;
_22.1 = Adt22 { fld0: _19.fld0 };
_21 = _2;
_20.1 = 253268639_u32 | 3611505371_u32;
_6 = [4_usize];
_20.2.1.1 = Adt22 { fld0: _15.1.fld0 };
_16.0.0 = &_23.0;
_19 = Adt22 { fld0: _20.2.1.1.fld0 };
_23.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,73_isize,(-9223372036854775808_isize),9223372036854775807_isize,127_isize];
match _20.2.1.1.fld0 {
0 => bb5,
1 => bb2,
2 => bb10,
35291 => bb12,
_ => bb11
}
}
bb10 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
Goto(bb2)
}
bb11 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
Goto(bb2)
}
bb12 = {
_11 = _1 << _20.1;
_20.2.1.1.fld0 = _15.1.fld0;
_8 = [1954179626007606731_usize];
_20.2.1.1.fld0 = _5 as u16;
_20.2.1.2 = -_15.2;
_24 = _1 as i32;
_18 = _12;
_21 = [39527522119155578796155798041658879982_u128,208136733222801108031097183836024389712_u128,84220523359669926577074079236952656741_u128,14539597646908704342106379715774907061_u128,117184555946562591038269293399787326054_u128,177662065980858496259093392101990751253_u128];
_24 = _20.1 as i32;
_22.1 = _19;
_2 = [73034961979721947793727087947879496914_u128,65433403452080302317750579358233777725_u128,285518739356368132793110816815696061791_u128,164577840762347486425578435357853698606_u128,229596196564657274639820476906721981431_u128,42044962789676113878096321060826273286_u128];
_17 = _22.2 + _15.2;
_18 = _3;
_18 = !_12;
_20.2.1.1 = Adt22 { fld0: _22.1.fld0 };
_20.2.1.2 = _15.2;
_28.1 = [(-17_isize),(-38_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),26_isize,87_isize,83_isize,51_isize];
_15.2 = _17;
_18 = !_3;
_12 = !_3;
match _20.2.1.1.fld0 {
0 => bb5,
1 => bb6,
2 => bb3,
35291 => bb13,
_ => bb7
}
}
bb13 = {
_22.1 = Adt22 { fld0: _19.fld0 };
_11 = _4 as i8;
_17 = 9435675178822826447_usize as f64;
_18 = _1 > _1;
_1 = _10;
match _15.1.fld0 {
0 => bb9,
1 => bb2,
2 => bb8,
3 => bb12,
4 => bb14,
5 => bb15,
6 => bb16,
35291 => bb18,
_ => bb17
}
}
bb14 = {
_11 = _1 << _20.1;
_20.2.1.1.fld0 = _15.1.fld0;
_8 = [1954179626007606731_usize];
_20.2.1.1.fld0 = _5 as u16;
_20.2.1.2 = -_15.2;
_24 = _1 as i32;
_18 = _12;
_21 = [39527522119155578796155798041658879982_u128,208136733222801108031097183836024389712_u128,84220523359669926577074079236952656741_u128,14539597646908704342106379715774907061_u128,117184555946562591038269293399787326054_u128,177662065980858496259093392101990751253_u128];
_24 = _20.1 as i32;
_22.1 = _19;
_2 = [73034961979721947793727087947879496914_u128,65433403452080302317750579358233777725_u128,285518739356368132793110816815696061791_u128,164577840762347486425578435357853698606_u128,229596196564657274639820476906721981431_u128,42044962789676113878096321060826273286_u128];
_17 = _22.2 + _15.2;
_18 = _3;
_18 = !_12;
_20.2.1.1 = Adt22 { fld0: _22.1.fld0 };
_20.2.1.2 = _15.2;
_28.1 = [(-17_isize),(-38_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),26_isize,87_isize,83_isize,51_isize];
_15.2 = _17;
_18 = !_3;
_12 = !_3;
match _20.2.1.1.fld0 {
0 => bb5,
1 => bb6,
2 => bb3,
35291 => bb13,
_ => bb7
}
}
bb15 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
Goto(bb2)
}
bb16 = {
_15.0 = 1047612398_i32 as u8;
_5 = _7 | _7;
_2 = _9;
_11 = _1 >> _1;
_1 = _11;
_15.1.fld0 = 9037355119911318673_u64 as u16;
_10 = _1 | _11;
_15.2 = _5 as f64;
_2 = _9;
_15.2 = (-1211093331_i32) as f64;
Goto(bb3)
}
bb17 = {
Return()
}
bb18 = {
_19.fld0 = (-9223372036854775808_isize) as u16;
_9 = [149680431888630657769823479025252577331_u128,277305329441904148432575354889041473812_u128,186225509720392787429676016440780372881_u128,73966133242907160149830015309276596632_u128,260148159309139957582854340316271900277_u128,287848538157639077435768236141631323205_u128];
_28.0 = Adt39::Variant2 { fld0: _20.2.1.0,fld1: _20.2.0,fld2: _20.2.1.2,fld3: _20.2.1.1 };
_7 = -_5;
_20.2.0 = Field::<(u8,)>(Variant(_28.0, 2), 1);
_15.1 = Adt22 { fld0: Field::<Adt22>(Variant(_28.0, 2), 3).fld0 };
_2 = [96178504071072460198094526265614916481_u128,126556133697789798619894662385253506808_u128,139425702203617252999386284580336104190_u128,284889452536430973422775421717151221111_u128,168375293780100451656480290236469283812_u128,319885916631220584409008861838328588119_u128];
_37 = -(-9223372036854775808_isize);
place!(Field::<Adt22>(Variant(_28.0, 2), 3)) = Adt22 { fld0: _20.2.1.1.fld0 };
_31 = [_15.0,_15.0,_22.0,Field::<(u8,)>(Variant(_28.0, 2), 1).0];
_20.2.1 = _22;
_36 = _3 as i16;
_36 = 6692_i16 * (-2045_i16);
_23 = (Move(_28.0), _28.1);
_4 = Field::<u8>(Variant(_23.0, 2), 0);
place!(Field::<Adt22>(Variant(_23.0, 2), 3)).fld0 = _15.2 as u16;
place!(Field::<f64>(Variant(_23.0, 2), 2)) = _15.2;
place!(Field::<(u8,)>(Variant(_23.0, 2), 1)) = (_20.2.1.0,);
_34 = !611355849219125208_usize;
_16.0.0 = &_23.0;
_15.1.fld0 = _20.2.1.1.fld0 ^ _20.2.1.1.fld0;
_15.1 = _22.1;
_17 = _20.1 as f64;
Call(_20.2.0.0 = core::intrinsics::bswap(_22.0), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_22.1 = Adt22 { fld0: _15.1.fld0 };
_15 = (_20.2.1.0, _22.1, _17);
_1 = _10 ^ _10;
_20.2.0.0 = Field::<u8>(Variant(_23.0, 2), 0);
_22 = (Field::<(u8,)>(Variant(_23.0, 2), 1).0, Field::<Adt22>(Variant(_23.0, 2), 3), _17);
_22 = _20.2.1;
_16.0.0 = &_23.0;
_1 = _24 as i8;
Call(_18 = fn18(_10), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_28 = (Move(_23.0), _23.1);
_37 = -(-9223372036854775808_isize);
_26 = _12 | _12;
Goto(bb21)
}
bb21 = {
_26 = _3;
_1 = _10;
_20.2.1 = _15;
place!(Field::<(u8,)>(Variant(_28.0, 2), 1)) = (_20.2.0.0,);
_39 = core::ptr::addr_of!(_37);
_28.1 = [(*_39),_37,(*_39),(*_39),_37,(*_39),_37,(*_39)];
_7 = _15.0 as i64;
_1 = _10;
_34 = 5_usize;
_15.2 = -Field::<f64>(Variant(_28.0, 2), 2);
_17 = _24 as f64;
match _21[_34] {
0 => bb19,
177662065980858496259093392101990751253 => bb22,
_ => bb9
}
}
bb22 = {
_1 = !_10;
_10 = -_1;
_24 = (-1143142355_i32);
_20.2.1.2 = _17;
_27 = !_9[_34];
_32 = '\u{24a1d}';
_42 = _10 as f32;
_16.0.0 = &_23.0;
_37 = _23.1[_34];
_41 = _37 - _37;
_30 = Field::<f64>(Variant(_28.0, 2), 2);
_11 = _20.1 as i8;
_15 = (_22.0, Field::<Adt22>(Variant(_28.0, 2), 3), _20.2.1.2);
SetDiscriminant(_28.0, 1);
_20.2.0.0 = _20.2.1.0;
_26 = _7 >= _5;
_20.2.1.0 = _20.2.0.0;
_41 = _37 | _23.1[_34];
place!(Field::<[isize; 8]>(Variant(_28.0, 1), 4)) = _23.1;
_22.1.fld0 = _15.1.fld0 | _20.2.1.1.fld0;
place!(Field::<u64>(Variant(_28.0, 1), 6)) = _3 as u64;
_35 = _32;
place!(Field::<u128>(Variant(_28.0, 1), 2)) = _27 * _21[_34];
Call(_24 = fn19(Move(_16.0), _23.1[_34], (*_39), Field::<u64>(Variant(_28.0, 1), 6), Move(_39), _10, _37, _30, _41, Field::<[isize; 8]>(Variant(_28.0, 1), 4), _10, Field::<[isize; 8]>(Variant(_28.0, 1), 4)[_34]), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_1 = _32 as i8;
_45.2 = !_24;
_43 = _34 << _22.1.fld0;
_22.0 = !_20.2.0.0;
_40 = Field::<u128>(Variant(_28.0, 1), 2) as i8;
_23.1 = Field::<[isize; 8]>(Variant(_28.0, 1), 4);
_15.0 = _22.0 & _20.2.1.0;
_15.0 = !_20.2.1.0;
_29 = !_3;
place!(Field::<[u64; 1]>(Variant(_28.0, 1), 1)) = [Field::<u64>(Variant(_28.0, 1), 6)];
match _20.2.1.1.fld0 {
0 => bb13,
1 => bb8,
2 => bb24,
3 => bb25,
35291 => bb27,
_ => bb26
}
}
bb24 = {
Return()
}
bb25 = {
_3 = _4 >= _4;
_5 = '\u{fbb4}' as i64;
_5 = -_7;
_15.1 = Adt22 { fld0: 35291_u16 };
_4 = _15.0;
_9 = [263553712390285021891399379641183063398_u128,74481094435953456620179139723327476140_u128,168047922053691898967766412057311281845_u128,60550613916350881066008712921307615992_u128,11103586948147223664800514204280707210_u128,105869049316905296587138731953950631163_u128];
_1 = !_11;
_15.0 = '\u{45596}' as u8;
_20.0 = core::ptr::addr_of!(_15.1);
_20.2.0 = (_4,);
_15.2 = 2665064857_u32 as f64;
_17 = (-25606_i16) as f64;
_11 = _15.1.fld0 as i8;
_6 = _8;
_9 = _2;
_20.2.1.2 = _15.2;
_20.1 = (-106735947_i32) as u32;
_20.2.1 = (_15.0, _15.1, _15.2);
_9 = [96380872037274037195324858476743895101_u128,36705127527185438733541785676777207519_u128,20159339233021045905641380011451240490_u128,137853168563281152855701456892525246436_u128,281007872237705741714821083116991582010_u128,321687126553619839492676204836455903582_u128];
_22 = (_20.2.1.0, _15.1, _15.2);
match _15.1.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
35291 => bb9,
_ => bb8
}
}
bb26 = {
_19.fld0 = (-9223372036854775808_isize) as u16;
_9 = [149680431888630657769823479025252577331_u128,277305329441904148432575354889041473812_u128,186225509720392787429676016440780372881_u128,73966133242907160149830015309276596632_u128,260148159309139957582854340316271900277_u128,287848538157639077435768236141631323205_u128];
_28.0 = Adt39::Variant2 { fld0: _20.2.1.0,fld1: _20.2.0,fld2: _20.2.1.2,fld3: _20.2.1.1 };
_7 = -_5;
_20.2.0 = Field::<(u8,)>(Variant(_28.0, 2), 1);
_15.1 = Adt22 { fld0: Field::<Adt22>(Variant(_28.0, 2), 3).fld0 };
_2 = [96178504071072460198094526265614916481_u128,126556133697789798619894662385253506808_u128,139425702203617252999386284580336104190_u128,284889452536430973422775421717151221111_u128,168375293780100451656480290236469283812_u128,319885916631220584409008861838328588119_u128];
_37 = -(-9223372036854775808_isize);
place!(Field::<Adt22>(Variant(_28.0, 2), 3)) = Adt22 { fld0: _20.2.1.1.fld0 };
_31 = [_15.0,_15.0,_22.0,Field::<(u8,)>(Variant(_28.0, 2), 1).0];
_20.2.1 = _22;
_36 = _3 as i16;
_36 = 6692_i16 * (-2045_i16);
_23 = (Move(_28.0), _28.1);
_4 = Field::<u8>(Variant(_23.0, 2), 0);
place!(Field::<Adt22>(Variant(_23.0, 2), 3)).fld0 = _15.2 as u16;
place!(Field::<f64>(Variant(_23.0, 2), 2)) = _15.2;
place!(Field::<(u8,)>(Variant(_23.0, 2), 1)) = (_20.2.1.0,);
_34 = !611355849219125208_usize;
_16.0.0 = &_23.0;
_15.1.fld0 = _20.2.1.1.fld0 ^ _20.2.1.1.fld0;
_15.1 = _22.1;
_17 = _20.1 as f64;
Call(_20.2.0.0 = core::intrinsics::bswap(_22.0), ReturnTo(bb19), UnwindUnreachable())
}
bb27 = {
_45.1 = -_17;
place!(Field::<u64>(Variant(_28.0, 1), 6)) = 15125157122693465574_u64;
_47 = (_35,);
_20.2.1.2 = _43 as f64;
_9 = [Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2),_27,Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2)];
_19 = Adt22 { fld0: _15.1.fld0 };
place!(Field::<*const i32>(Variant(_28.0, 1), 5)) = core::ptr::addr_of!(_45.2);
_49 = core::ptr::addr_of!(_33);
_37 = _41;
match _20.2.1.1.fld0 {
0 => bb3,
1 => bb28,
2 => bb29,
3 => bb30,
35291 => bb32,
_ => bb31
}
}
bb28 = {
_11 = _1 << _20.1;
_20.2.1.1.fld0 = _15.1.fld0;
_8 = [1954179626007606731_usize];
_20.2.1.1.fld0 = _5 as u16;
_20.2.1.2 = -_15.2;
_24 = _1 as i32;
_18 = _12;
_21 = [39527522119155578796155798041658879982_u128,208136733222801108031097183836024389712_u128,84220523359669926577074079236952656741_u128,14539597646908704342106379715774907061_u128,117184555946562591038269293399787326054_u128,177662065980858496259093392101990751253_u128];
_24 = _20.1 as i32;
_22.1 = _19;
_2 = [73034961979721947793727087947879496914_u128,65433403452080302317750579358233777725_u128,285518739356368132793110816815696061791_u128,164577840762347486425578435357853698606_u128,229596196564657274639820476906721981431_u128,42044962789676113878096321060826273286_u128];
_17 = _22.2 + _15.2;
_18 = _3;
_18 = !_12;
_20.2.1.1 = Adt22 { fld0: _22.1.fld0 };
_20.2.1.2 = _15.2;
_28.1 = [(-17_isize),(-38_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),26_isize,87_isize,83_isize,51_isize];
_15.2 = _17;
_18 = !_3;
_12 = !_3;
match _20.2.1.1.fld0 {
0 => bb5,
1 => bb6,
2 => bb3,
35291 => bb13,
_ => bb7
}
}
bb29 = {
_19.fld0 = (-9223372036854775808_isize) as u16;
_9 = [149680431888630657769823479025252577331_u128,277305329441904148432575354889041473812_u128,186225509720392787429676016440780372881_u128,73966133242907160149830015309276596632_u128,260148159309139957582854340316271900277_u128,287848538157639077435768236141631323205_u128];
_28.0 = Adt39::Variant2 { fld0: _20.2.1.0,fld1: _20.2.0,fld2: _20.2.1.2,fld3: _20.2.1.1 };
_7 = -_5;
_20.2.0 = Field::<(u8,)>(Variant(_28.0, 2), 1);
_15.1 = Adt22 { fld0: Field::<Adt22>(Variant(_28.0, 2), 3).fld0 };
_2 = [96178504071072460198094526265614916481_u128,126556133697789798619894662385253506808_u128,139425702203617252999386284580336104190_u128,284889452536430973422775421717151221111_u128,168375293780100451656480290236469283812_u128,319885916631220584409008861838328588119_u128];
_37 = -(-9223372036854775808_isize);
place!(Field::<Adt22>(Variant(_28.0, 2), 3)) = Adt22 { fld0: _20.2.1.1.fld0 };
_31 = [_15.0,_15.0,_22.0,Field::<(u8,)>(Variant(_28.0, 2), 1).0];
_20.2.1 = _22;
_36 = _3 as i16;
_36 = 6692_i16 * (-2045_i16);
_23 = (Move(_28.0), _28.1);
_4 = Field::<u8>(Variant(_23.0, 2), 0);
place!(Field::<Adt22>(Variant(_23.0, 2), 3)).fld0 = _15.2 as u16;
place!(Field::<f64>(Variant(_23.0, 2), 2)) = _15.2;
place!(Field::<(u8,)>(Variant(_23.0, 2), 1)) = (_20.2.1.0,);
_34 = !611355849219125208_usize;
_16.0.0 = &_23.0;
_15.1.fld0 = _20.2.1.1.fld0 ^ _20.2.1.1.fld0;
_15.1 = _22.1;
_17 = _20.1 as f64;
Call(_20.2.0.0 = core::intrinsics::bswap(_22.0), ReturnTo(bb19), UnwindUnreachable())
}
bb30 = {
_15.0 = _4;
_12 = _7 == _5;
_15.1.fld0 = !46712_u16;
_6 = [12399860883179670685_usize];
Goto(bb2)
}
bb31 = {
_3 = _4 >= _4;
_5 = '\u{fbb4}' as i64;
_5 = -_7;
_15.1 = Adt22 { fld0: 35291_u16 };
_4 = _15.0;
_9 = [263553712390285021891399379641183063398_u128,74481094435953456620179139723327476140_u128,168047922053691898967766412057311281845_u128,60550613916350881066008712921307615992_u128,11103586948147223664800514204280707210_u128,105869049316905296587138731953950631163_u128];
_1 = !_11;
_15.0 = '\u{45596}' as u8;
_20.0 = core::ptr::addr_of!(_15.1);
_20.2.0 = (_4,);
_15.2 = 2665064857_u32 as f64;
_17 = (-25606_i16) as f64;
_11 = _15.1.fld0 as i8;
_6 = _8;
_9 = _2;
_20.2.1.2 = _15.2;
_20.1 = (-106735947_i32) as u32;
_20.2.1 = (_15.0, _15.1, _15.2);
_9 = [96380872037274037195324858476743895101_u128,36705127527185438733541785676777207519_u128,20159339233021045905641380011451240490_u128,137853168563281152855701456892525246436_u128,281007872237705741714821083116991582010_u128,321687126553619839492676204836455903582_u128];
_22 = (_20.2.1.0, _15.1, _15.2);
match _15.1.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
35291 => bb9,
_ => bb8
}
}
bb32 = {
_38 = -_42;
place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0)).1 = Adt22 { fld0: _19.fld0 };
place!(Field::<[u64; 1]>(Variant(_28.0, 1), 1)) = [Field::<u64>(Variant(_28.0, 1), 6)];
_17 = _15.2 + _15.2;
place!(Field::<u128>(Variant(_28.0, 1), 2)) = _10 as u128;
_39 = core::ptr::addr_of!((*_49));
_45 = (_24, _20.2.1.2, _24, Field::<u64>(Variant(_28.0, 1), 6));
_48 = _47.0;
_20.2.1.1 = Adt22 { fld0: _22.1.fld0 };
_21 = [Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2),_27,Field::<u128>(Variant(_28.0, 1), 2),Field::<u128>(Variant(_28.0, 1), 2)];
_19 = Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0).1;
_46 = Move(Field::<*const i32>(Variant(_28.0, 1), 5));
_36 = 8473_i16;
_10 = _40;
_45.2 = !_24;
place!(Field::<(char,)>(Variant(_28.0, 1), 3)).0 = _35;
_36 = (-26088_i16);
Goto(bb33)
}
bb33 = {
place!(Field::<[isize; 8]>(Variant(_28.0, 1), 4)) = _23.1;
place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0)).2 = _20.2.1.2;
_37 = _41 & _41;
(*_39) = -_37;
_17 = -Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0).2;
place!(Field::<(char,)>(Variant(_28.0, 1), 3)) = (_35,);
place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0)).0 = _20.2.1.0 << (*_49);
_16.0.0 = &_23.0;
match Field::<u64>(Variant(_28.0, 1), 6) {
0 => bb14,
1 => bb16,
2 => bb6,
3 => bb34,
15125157122693465574 => bb36,
_ => bb35
}
}
bb34 = {
_1 = !_10;
_10 = -_1;
_24 = (-1143142355_i32);
_20.2.1.2 = _17;
_27 = !_9[_34];
_32 = '\u{24a1d}';
_42 = _10 as f32;
_16.0.0 = &_23.0;
_37 = _23.1[_34];
_41 = _37 - _37;
_30 = Field::<f64>(Variant(_28.0, 2), 2);
_11 = _20.1 as i8;
_15 = (_22.0, Field::<Adt22>(Variant(_28.0, 2), 3), _20.2.1.2);
SetDiscriminant(_28.0, 1);
_20.2.0.0 = _20.2.1.0;
_26 = _7 >= _5;
_20.2.1.0 = _20.2.0.0;
_41 = _37 | _23.1[_34];
place!(Field::<[isize; 8]>(Variant(_28.0, 1), 4)) = _23.1;
_22.1.fld0 = _15.1.fld0 | _20.2.1.1.fld0;
place!(Field::<u64>(Variant(_28.0, 1), 6)) = _3 as u64;
_35 = _32;
place!(Field::<u128>(Variant(_28.0, 1), 2)) = _27 * _21[_34];
Call(_24 = fn19(Move(_16.0), _23.1[_34], (*_39), Field::<u64>(Variant(_28.0, 1), 6), Move(_39), _10, _37, _30, _41, Field::<[isize; 8]>(Variant(_28.0, 1), 4), _10, Field::<[isize; 8]>(Variant(_28.0, 1), 4)[_34]), ReturnTo(bb23), UnwindUnreachable())
}
bb35 = {
_11 = _1 << _20.1;
_20.2.1.1.fld0 = _15.1.fld0;
_8 = [1954179626007606731_usize];
_20.2.1.1.fld0 = _5 as u16;
_20.2.1.2 = -_15.2;
_24 = _1 as i32;
_18 = _12;
_21 = [39527522119155578796155798041658879982_u128,208136733222801108031097183836024389712_u128,84220523359669926577074079236952656741_u128,14539597646908704342106379715774907061_u128,117184555946562591038269293399787326054_u128,177662065980858496259093392101990751253_u128];
_24 = _20.1 as i32;
_22.1 = _19;
_2 = [73034961979721947793727087947879496914_u128,65433403452080302317750579358233777725_u128,285518739356368132793110816815696061791_u128,164577840762347486425578435357853698606_u128,229596196564657274639820476906721981431_u128,42044962789676113878096321060826273286_u128];
_17 = _22.2 + _15.2;
_18 = _3;
_18 = !_12;
_20.2.1.1 = Adt22 { fld0: _22.1.fld0 };
_20.2.1.2 = _15.2;
_28.1 = [(-17_isize),(-38_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),26_isize,87_isize,83_isize,51_isize];
_15.2 = _17;
_18 = !_3;
_12 = !_3;
match _20.2.1.1.fld0 {
0 => bb5,
1 => bb6,
2 => bb3,
35291 => bb13,
_ => bb7
}
}
bb36 = {
_45.2 = !_45.0;
_45.1 = _15.2;
_51 = &place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0));
_19 = Adt22 { fld0: _15.1.fld0 };
_54 = _48;
_20.2.1.1.fld0 = (*_51).1.fld0;
_51 = &_22;
place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0)).2 = _43 as f64;
Goto(bb37)
}
bb37 = {
RET = core::ptr::addr_of!(_57);
_46 = core::ptr::addr_of!(_24);
_55 = [_5,_5,_5];
(*RET) = core::ptr::addr_of_mut!((*_49));
_26 = _27 == Field::<u128>(Variant(_28.0, 1), 2);
_15 = (Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0).0, _22.1, _45.1);
_20.0 = core::ptr::addr_of!(place!(Field::<(u8, Adt22, f64)>(Variant(_28.0, 1), 0)).1);
_3 = _26;
(*RET) = core::ptr::addr_of_mut!((*_57));
Goto(bb38)
}
bb38 = {
Call(_62 = dump_var(17_usize, 55_usize, Move(_55), 54_usize, Move(_54), 41_usize, Move(_41), 32_usize, Move(_32)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_62 = dump_var(17_usize, 3_usize, Move(_3), 1_usize, Move(_1), 33_usize, Move(_33), 27_usize, Move(_27)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_62 = dump_var(17_usize, 6_usize, Move(_6), 7_usize, Move(_7), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_62 = dump_var(17_usize, 9_usize, Move(_9), 37_usize, Move(_37), 24_usize, Move(_24), 31_usize, Move(_31)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: i8) -> bool {
mir! {
type RET = bool;
let _2: [u128; 2];
let _3: &'static Adt42;
let _4: isize;
let _5: [u64; 1];
let _6: Adt42;
let _7: f64;
let _8: char;
let _9: i8;
let _10: (u8, Adt22, f64);
let _11: (u16, (u64,));
let _12: &'static (Adt39, [isize; 8]);
let _13: *const (*const i64,);
let _14: (&'static Adt42, u8);
let _15: [i64; 3];
let _16: usize;
let _17: (Adt39, [isize; 8]);
let _18: u64;
let _19: &'static i64;
let _20: isize;
let _21: [u16; 3];
let _22: u16;
let _23: char;
let _24: f32;
let _25: u32;
let _26: Adt22;
let _27: isize;
let _28: f32;
let _29: Adt31;
let _30: [u8; 4];
let _31: *mut i8;
let _32: [usize; 1];
let _33: ((u64,), [u32; 7], u128, isize);
let _34: *mut &'static [u32; 7];
let _35: &'static Adt68;
let _36: isize;
let _37: [u16; 3];
let _38: *const (u8, Adt22, f64);
let _39: *const Adt31;
let _40: f64;
let _41: bool;
let _42: f32;
let _43: i16;
let _44: ();
let _45: ();
{
_2 = [168435766552981533628156353393074015258_u128,253966401059677966531093715852814096719_u128];
_1 = 49833_u16 as i8;
_4 = !(-61_isize);
RET = !true;
RET = true;
_4 = '\u{6542d}' as isize;
_4 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_1 = 7_i8;
RET = true;
RET = !true;
_2 = [328141766685709472540815195287749478829_u128,338633767147985606012515761349569400984_u128];
_2 = [149084560512536741768787962140379603719_u128,15238989098043054150347227117664243448_u128];
_2 = [249954121885937310184614586839881494799_u128,90849816813753755306080075094438730099_u128];
_5 = [5351664221681198168_u64];
RET = _4 >= _4;
_4 = 7810515100547097887_u64 as isize;
_1 = _4 as i8;
_4 = 9223372036854775807_isize;
RET = true;
_1 = !(-73_i8);
_7 = 57778226994942658968030451031902790756_i128 as f64;
_3 = &_6;
_3 = &(*_3);
_4 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_8 = '\u{e1e07}';
_2 = [192162012592622272532159766263618269760_u128,297106038132106428193895734291818917799_u128];
_5 = [16625836903326055951_u64];
RET = _1 < _1;
_8 = '\u{d6250}';
_5 = [7730859738999452255_u64];
_4 = !33_isize;
_5 = [6933347578044712955_u64];
_8 = '\u{279ad}';
_2 = [303456690615467786704326551631747644161_u128,234412910430921628034019659255821576050_u128];
RET = true;
RET = !false;
_9 = 163644911835019562107548382414696016145_i128 as i8;
RET = _9 > _9;
RET = _1 != _1;
_4 = 9223372036854775807_isize * 9223372036854775807_isize;
_3 = &_6;
RET = _8 >= _8;
_9 = _1 ^ _1;
_7 = (-8273450359114839681_i64) as f64;
_4 = !(-9223372036854775808_isize);
RET = false;
Goto(bb2)
}
bb2 = {
_10.2 = -_7;
_11.1.0 = 24836_u16 as u64;
RET = _4 >= _4;
_7 = _10.2;
_11.1.0 = _9 as u64;
_10.1.fld0 = 23973_u16;
_10.0 = 210_u8;
_3 = &(*_3);
_10.1.fld0 = 48319_u16;
_11.1 = (14443970131277106889_u64,);
_11.0 = _10.1.fld0;
_10.1.fld0 = (-1616489165_i32) as u16;
_11.1.0 = 12324464379576732538_u64 | 3845137859332784562_u64;
_2 = [99260623976509112750492746527554789427_u128,230845830060552549230096800143666172863_u128];
Goto(bb3)
}
bb3 = {
_11.1 = (13678027457769071478_u64,);
_8 = '\u{ef365}';
_9 = _1 & _1;
_11.1.0 = 9498948165091962071_u64 * 7528937523834118134_u64;
_8 = '\u{7f23f}';
_3 = &(*_3);
_10.2 = _11.0 as f64;
_11.0 = _10.1.fld0;
_3 = &_6;
_11.1.0 = 16759543316369602234_u64;
_11.0 = !_10.1.fld0;
_10.1 = Adt22 { fld0: _11.0 };
RET = true;
_10.1.fld0 = _11.0 ^ _11.0;
Goto(bb4)
}
bb4 = {
_10.1.fld0 = _11.0 ^ _11.0;
_14.0 = &(*_3);
_10.2 = _10.0 as f64;
RET = false;
_5 = [_11.1.0];
_16 = 14155699955893607869_usize;
_10.2 = _7;
_14.0 = &(*_3);
RET = !false;
_4 = 9223372036854775807_isize >> _10.1.fld0;
_16 = 3_usize;
Goto(bb5)
}
bb5 = {
_11.0 = 93667606375064516674683549155699817798_u128 as u16;
_11.1.0 = 2803851940023395672_u64;
_17.1 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = !true;
_8 = '\u{21715}';
_14.1 = _10.0 << _9;
_14.0 = &_6;
RET = _10.2 <= _10.2;
_10.1.fld0 = _11.0;
RET = !true;
_10.0 = _14.1 | _14.1;
_18 = !_11.1.0;
Goto(bb6)
}
bb6 = {
_10.1.fld0 = _16 as u16;
_15 = [(-1897502050210839165_i64),2679492253040425257_i64,675773805915048721_i64];
_3 = &_6;
_11.0 = _9 as u16;
_10.1.fld0 = _11.0 + _11.0;
_22 = !_10.1.fld0;
Goto(bb7)
}
bb7 = {
_15 = [2757603674598249632_i64,5882958761405762365_i64,1950003885250867463_i64];
_1 = !_9;
_22 = !_10.1.fld0;
_10.1 = Adt22 { fld0: _11.0 };
_14.0 = &_6;
RET = true;
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
_3 = &_6;
_3 = &_6;
_5 = [_11.1.0];
_11.1.0 = _18 >> _10.0;
_20 = _8 as isize;
_20 = _4;
_11.0 = !_22;
_15 = [5634520704658923649_i64,2209357092078089136_i64,8790194721671299336_i64];
_1 = (-91322568561481182893683094432962430400_i128) as i8;
_5 = [_11.1.0];
_11.1 = (_18,);
_24 = 275277141342019700376549511752707076867_u128 as f32;
_26 = Adt22 { fld0: _10.1.fld0 };
_1 = _7 as i8;
_25 = !3207204468_u32;
_10.0 = (-40937436392677856747059369753659590028_i128) as u8;
_27 = _20;
_28 = -_24;
_22 = !_11.0;
_15 = [(-5437414960839561424_i64),(-8638932585900719716_i64),(-7542962961562999224_i64)];
_14.0 = &(*_3);
_20 = !_4;
_14.0 = &_6;
Goto(bb10)
}
bb10 = {
_4 = _20 + _27;
_17.1 = [_20,_4,_27,_4,_4,_4,_4,_4];
_20 = -_17.1[_16];
_26 = Adt22 { fld0: _11.0 };
_24 = -_28;
RET = false;
_1 = _9;
Goto(bb11)
}
bb11 = {
_24 = _28;
_30[_16] = (-1850391756_i32) as u8;
_30 = [_14.1,_14.1,_14.1,_10.0];
_12 = &_17;
_11.1 = (_18,);
_8 = '\u{f21d0}';
_11.0 = !_10.1.fld0;
_14.0 = &(*_3);
_1 = _9;
_12 = &(*_12);
match _16 {
0 => bb10,
1 => bb7,
2 => bb3,
3 => bb12,
_ => bb6
}
}
bb12 = {
_1 = _9;
_5 = [_11.1.0];
_10.2 = -_7;
_10.2 = (-60589125_i32) as f64;
_10.0 = _8 as u8;
_23 = _8;
_16 = 3_usize | 4_usize;
_10.2 = -_7;
_5 = [_11.1.0];
_14.0 = &(*_3);
_31 = core::ptr::addr_of_mut!(_1);
_27 = -_4;
_33.0.0 = _18;
_5 = [_33.0.0];
_15 = [4137569914834458912_i64,(-5047822309580609890_i64),7987410044755556844_i64];
(*_31) = _20 as i8;
(*_31) = -_9;
_25 = 2671832132_u32;
_22 = _26.fld0 + _10.1.fld0;
_8 = _23;
_11.0 = _8 as u16;
_10.1 = _26;
_8 = _23;
_26 = Adt22 { fld0: _22 };
match _25 {
0 => bb10,
1 => bb2,
2 => bb8,
3 => bb4,
2671832132 => bb14,
_ => bb13
}
}
bb13 = {
_11.0 = 93667606375064516674683549155699817798_u128 as u16;
_11.1.0 = 2803851940023395672_u64;
_17.1 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = !true;
_8 = '\u{21715}';
_14.1 = _10.0 << _9;
_14.0 = &_6;
RET = _10.2 <= _10.2;
_10.1.fld0 = _11.0;
RET = !true;
_10.0 = _14.1 | _14.1;
_18 = !_11.1.0;
Goto(bb6)
}
bb14 = {
_30 = [_14.1,_14.1,_14.1,_10.0];
_33.0 = _11.1;
_33.0.0 = _7 as u64;
_23 = _8;
_10 = (_14.1, _26, _7);
_11.1 = _33.0;
_2 = [84069119527242136657470992112298964923_u128,336062466599535053075611067967133890556_u128];
_21 = [_10.1.fld0,_22,_22];
_7 = _10.2 - _10.2;
_39 = core::ptr::addr_of!(_29);
_8 = _23;
_23 = _8;
_22 = !_10.1.fld0;
_33.3 = _20;
_36 = !_33.3;
_17.1 = [_33.3,_33.3,_27,_27,_27,_27,_20,_27];
_41 = !RET;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(18_usize, 5_usize, Move(_5), 36_usize, Move(_36), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(18_usize, 20_usize, Move(_20), 25_usize, Move(_25), 16_usize, Move(_16), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(18_usize, 2_usize, Move(_2), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: (&'static Adt39,),mut _2: isize,mut _3: isize,mut _4: u64,mut _5: *const isize,mut _6: i8,mut _7: isize,mut _8: f64,mut _9: isize,mut _10: [isize; 8],mut _11: i8,mut _12: isize) -> i32 {
mir! {
type RET = i32;
let _13: *mut f32;
let _14: ([bool; 1], (u8, Adt22, f64), i16, (u64,));
let _15: isize;
let _16: u16;
let _17: i8;
let _18: (u8,);
let _19: char;
let _20: u8;
let _21: isize;
let _22: isize;
let _23: isize;
let _24: i64;
let _25: bool;
let _26: *const *mut isize;
let _27: f32;
let _28: u16;
let _29: &'static (&'static Adt42, u8);
let _30: isize;
let _31: *mut &'static [u32; 7];
let _32: *const &'static [u32; 7];
let _33: [u8; 4];
let _34: u128;
let _35: (u64,);
let _36: [u8; 4];
let _37: &'static &'static *const isize;
let _38: isize;
let _39: ();
let _40: ();
{
_10 = [_9,_12,_3,_7,_9,_9,_2,_3];
_8 = 3_u8 as f64;
_11 = _6 << _2;
RET = !(-281559404_i32);
_8 = _7 as f64;
_4 = 6702327836549737551_u64;
_12 = _9 | _3;
_7 = !_12;
_14.3 = (_4,);
_14.0 = [false];
_14.1.1 = Adt22 { fld0: 2298_u16 };
_18.0 = _8 as u8;
_16 = !_14.1.1.fld0;
_14.1.0 = true as u8;
_14.2 = (-4044_i16);
_15 = _12;
Goto(bb1)
}
bb1 = {
_7 = _11 as isize;
_9 = _12 << _11;
_20 = 6925067541434845364_i64 as u8;
_2 = _7;
_17 = !_6;
_14.1.2 = -_8;
_14.3 = (_4,);
_9 = _12;
_14.1.0 = _18.0;
_14.1.0 = _18.0 | _18.0;
_4 = !_14.3.0;
_9 = '\u{474a8}' as isize;
RET = 260034473_i32;
_10 = [_2,_7,_7,_2,_7,_12,_3,_7];
_14.1.1 = Adt22 { fld0: _16 };
_10 = [_7,_12,_2,_2,_7,_2,_2,_3];
_18 = (_14.1.0,);
_6 = -_11;
_21 = _2;
_14.1.2 = _4 as f64;
_8 = _16 as f64;
_14.1.2 = _8 - _8;
_2 = _12 + _21;
_6 = '\u{7441d}' as i8;
_14.1.0 = _18.0 * _18.0;
Goto(bb2)
}
bb2 = {
_12 = _7;
_14.0 = [true];
RET = '\u{6eeb8}' as i32;
_14.0 = [false];
_20 = _14.2 as u8;
_5 = core::ptr::addr_of!(_9);
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
87 => bb10,
_ => bb9
}
}
bb3 = {
_7 = _11 as isize;
_9 = _12 << _11;
_20 = 6925067541434845364_i64 as u8;
_2 = _7;
_17 = !_6;
_14.1.2 = -_8;
_14.3 = (_4,);
_9 = _12;
_14.1.0 = _18.0;
_14.1.0 = _18.0 | _18.0;
_4 = !_14.3.0;
_9 = '\u{474a8}' as isize;
RET = 260034473_i32;
_10 = [_2,_7,_7,_2,_7,_12,_3,_7];
_14.1.1 = Adt22 { fld0: _16 };
_10 = [_7,_12,_2,_2,_7,_2,_2,_3];
_18 = (_14.1.0,);
_6 = -_11;
_21 = _2;
_14.1.2 = _4 as f64;
_8 = _16 as f64;
_14.1.2 = _8 - _8;
_2 = _12 + _21;
_6 = '\u{7441d}' as i8;
_14.1.0 = _18.0 * _18.0;
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
RET = -(-1092787979_i32);
_14.1.2 = _18.0 as f64;
_16 = 2274660739_u32 as u16;
_9 = _2 << _18.0;
_14.3.0 = !_4;
_2 = (*_5);
_19 = '\u{9d564}';
(*_5) = -_12;
_21 = _2 << (*_5);
_18.0 = _14.1.0 - _14.1.0;
_4 = !_14.3.0;
_18 = (_14.1.0,);
_3 = _21;
_11 = _14.1.0 as i8;
_14.1.2 = -_8;
_14.3 = (_4,);
_14.1.0 = !_18.0;
(*_5) = !_3;
_14.1.1 = Adt22 { fld0: _16 };
RET = 1752747476_i32 & 1127471411_i32;
_8 = _14.1.2 - _14.1.2;
_14.1.1.fld0 = !_16;
_16 = _14.1.1.fld0 - _14.1.1.fld0;
_16 = _14.1.1.fld0;
_20 = !_14.1.0;
_24 = _20 as i64;
_12 = RET as isize;
_23 = -_3;
Goto(bb11)
}
bb11 = {
_5 = core::ptr::addr_of!(_9);
_10 = [_12,(*_5),(*_5),(*_5),(*_5),_7,_3,(*_5)];
_9 = !_12;
_14.1.1.fld0 = _16;
(*_5) = -_2;
_15 = _9 & (*_5);
_27 = RET as f32;
_7 = (*_5) - _23;
_14.1.2 = _14.3.0 as f64;
_18.0 = _14.1.1.fld0 as u8;
match _14.2 {
0 => bb5,
1 => bb9,
340282366920938463463374607431768207412 => bb12,
_ => bb6
}
}
bb12 = {
_14.3.0 = _4 & _4;
_21 = !_23;
_3 = _11 as isize;
_13 = core::ptr::addr_of_mut!(_27);
_6 = RET as i8;
_14.1.1 = Adt22 { fld0: _16 };
_28 = !_16;
_15 = _2;
_17 = _11 & _11;
Goto(bb13)
}
bb13 = {
_21 = _9 << _7;
_25 = true;
_4 = !_14.3.0;
_4 = _14.3.0;
_18.0 = !_20;
(*_13) = _14.3.0 as f32;
(*_13) = _14.1.1.fld0 as f32;
_6 = _17;
_15 = -_23;
(*_13) = 4758788987475919202_usize as f32;
_10 = [_7,_7,_21,_21,_15,(*_5),_9,(*_5)];
_14.1.2 = _8 - _8;
_14.1.1 = Adt22 { fld0: _16 };
Call(_17 = core::intrinsics::transmute(_11), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17 = !_6;
(*_5) = _17 as isize;
_10 = [_23,(*_5),_21,_15,_21,(*_5),_7,_15];
_10 = [_12,_7,_21,_3,_23,(*_5),_21,_2];
_14.1.0 = _18.0;
_30 = _23;
_18 = (_14.1.0,);
(*_13) = _7 as f32;
(*_5) = _19 as isize;
_35.0 = _4 ^ _14.3.0;
_35 = (_14.3.0,);
(*_13) = _2 as f32;
_25 = false;
RET = _14.3.0 as i32;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(19_usize, 20_usize, Move(_20), 35_usize, Move(_35), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(19_usize, 19_usize, Move(_19), 28_usize, Move(_28), 21_usize, Move(_21), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(19_usize, 3_usize, Move(_3), 23_usize, Move(_23), 30_usize, Move(_30), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-3621452686395123207_i64)));
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt22{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt22 {
fld0: u16,
}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){unsafe{printf("Adt30::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt30 {
Variant0{
fld0: usize,
fld1: u16,
fld2: u64,
fld3: i64,

},
Variant1{
fld0: *const Adt22,
fld1: (u8, Adt22, f64),
fld2: isize,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: [u32; 7],
fld1: u32,
fld2: *const Adt22,
fld3: i8,
fld4: [u64; 1],
fld5: ((u8,), (u8, Adt22, f64)),
fld6: usize,

},
Variant1{
fld0: u32,
fld1: i128,
fld2: [bool; 1],
fld3: i64,

},
Variant2{
fld0: *const Adt22,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: (u64,),
fld1: (char,),
fld2: isize,
fld3: u16,
fld4: i128,

},
Variant1{
fld0: (u8, Adt22, f64),
fld1: [u64; 1],
fld2: u128,
fld3: (char,),
fld4: [isize; 8],
fld5: *const i32,
fld6: u64,

},
Variant2{
fld0: u8,
fld1: (u8,),
fld2: f64,
fld3: Adt22,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u64,
fld1: char,
fld2: [isize; 8],
fld3: Adt31,
fld4: (u8,),

},
Variant1{
fld0: [u32; 3],
fld1: *const isize,

},
Variant2{
fld0: i128,
fld1: [bool; 1],
fld2: isize,
fld3: Adt31,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u64; 1],
fld1: [u32; 3],
fld2: Adt31,
fld3: i8,

},
Variant1{
fld0: *const Adt22,
fld1: (u8, Adt22, f64),
fld2: [u64; 1],
fld3: i8,
fld4: i16,
fld5: *mut *mut i8,
fld6: usize,
fld7: [u32; 7],

},
Variant2{
fld0: bool,
fld1: Adt30,

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: [u32; 3],
fld1: *mut *mut i8,
fld2: usize,
fld3: ([bool; 1], (u8, Adt22, f64), i16, (u64,)),

},
Variant1{
fld0: [u32; 7],
fld1: Adt31,

}}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){unsafe{printf("Adt80::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt80 {
Variant0{
fld0: u64,
fld1: Adt22,
fld2: u128,

},
Variant1{
fld0: [u128; 2],
fld1: char,
fld2: [u128; 6],
fld3: f32,
fld4: [u8; 4],

},
Variant2{
fld0: *mut isize,
fld1: [isize; 8],
fld2: [u128; 2],
fld3: (u16, (u64,)),

}}

