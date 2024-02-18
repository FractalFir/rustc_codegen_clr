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
pub fn fn0(mut _1: i8) -> Adt34 {
mir! {
type RET = Adt34;
let _2: f32;
let _3: char;
let _4: (u8,);
let _5: f64;
let _6: f32;
let _7: char;
let _8: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _9: f32;
let _10: f32;
let _11: i16;
let _12: isize;
let _13: f32;
let _14: bool;
let _15: i8;
let _16: [i8; 5];
let _17: char;
let _18: char;
let _19: *mut ([u16; 7], usize);
let _20: &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _21: u64;
let _22: u32;
let _23: char;
let _24: &'static i8;
let _25: ((f32,), &'static f32);
let _26: [i128; 1];
let _27: Adt42;
let _28: ((u32,), *mut [usize; 7], [u16; 7]);
let _29: ([u16; 7], usize);
let _30: isize;
let _31: u16;
let _32: u64;
let _33: char;
let _34: f32;
let _35: i128;
let _36: isize;
let _37: ((f32,), &'static f32);
let _38: isize;
let _39: [bool; 3];
let _40: ();
let _41: ();
{
RET.fld3 = 17_i8 | (-47_i8);
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
RET.fld0.0 = 11139423785283427283_usize as u8;
RET.fld3 = (-88_i8) << RET.fld0.0;
RET.fld4 = 208883003664728330818211071192187342743_u128;
_1 = (-9223372036854775808_isize) as i8;
_3 = '\u{dfc31}';
_2 = 1_usize as f32;
RET.fld0 = (83_u8,);
RET.fld3 = _1;
Call(RET.fld4 = fn1(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.fld2 = [6375157659634447308_usize,13269462791043533831_usize,15181271072106359849_usize,15048876634299832018_usize,12390029256828735633_usize,11334768566533618077_usize,33491236182233334_usize];
RET.fld4 = 68660296998246784151703868104497748694_u128;
_2 = 765133124_u32 as f32;
RET.fld0.0 = !114_u8;
_4.0 = 48606_u16 as u8;
RET.fld3 = 673046587_i32 as i8;
RET.fld4 = !197024179438374990379639500736477855638_u128;
_5 = _4.0 as f64;
RET.fld0.0 = _4.0 * _4.0;
RET.fld0 = _4;
RET.fld4 = 110535791116883203499909510188785840726_u128 * 195554769382552737760956405994516013894_u128;
RET.fld3 = !_1;
_6 = _2;
RET.fld2 = [6_usize,1_usize,10358198929834501119_usize,2_usize,3_usize,3757246605628124861_usize,1_usize];
Goto(bb2)
}
bb2 = {
RET.fld0.0 = !_4.0;
RET.fld2 = [10381055136869404607_usize,6785245295930817037_usize,3_usize,8378505939462228646_usize,11342891640448223052_usize,15123466079818937118_usize,7_usize];
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
RET.fld3 = 12762979198423777098_u64 as i8;
_4.0 = 33030_u16 as u8;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_2 = -_6;
Call(RET.fld3 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _3;
_6 = -_2;
RET.fld3 = 2317351621_u32 as i8;
_4 = (RET.fld0.0,);
Goto(bb4)
}
bb4 = {
RET.fld3 = _1 & _1;
RET.fld0.0 = _4.0;
_4.0 = RET.fld0.0 | RET.fld0.0;
RET.fld3 = _1 - _1;
_6 = -_2;
_5 = _2 as f64;
_7 = _3;
_4 = RET.fld0;
_8.2.0 = RET.fld0.0 - RET.fld0.0;
RET.fld4 = !311892775569153004313703802734308675005_u128;
RET.fld3 = -_1;
_5 = _2 as f64;
Goto(bb5)
}
bb5 = {
RET.fld4 = !25984467864240717902377543115616578817_u128;
RET.fld3 = _1 + _1;
_1 = RET.fld3 - RET.fld3;
RET.fld0 = _8.2;
_11 = 9007_i16 | 13728_i16;
_8.2 = (RET.fld0.0,);
_5 = 12181566984404756144_u64 as f64;
_8.2.0 = RET.fld0.0 + RET.fld0.0;
_7 = _3;
_8.2.0 = 2177074316222055371_u64 as u8;
_10 = _6 + _2;
_14 = !true;
_13 = _10;
_12 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_15 = 3021330661_u32 as i8;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
RET.fld2 = [5_usize,1457865165844993148_usize,1_usize,12789450299912537511_usize,12387903959268401382_usize,17950800717284234084_usize,0_usize];
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_13 = _10;
RET.fld4 = !134840304049939258284392682203637493739_u128;
_2 = -_10;
_17 = _7;
_4 = (_8.2.0,);
Goto(bb6)
}
bb6 = {
RET.fld4 = 51725685438122238890166004472350605715_u128 >> _15;
_18 = _17;
_9 = _6 * _13;
_9 = (-33769471492332132956494073997877594433_i128) as f32;
_12 = _1 as isize;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_9 = -_13;
RET.fld2 = [1_usize,4_usize,7789282271089751930_usize,0_usize,6_usize,15118161515647948908_usize,14276779558223404530_usize];
_4 = RET.fld0;
RET.fld0.0 = _8.2.0 >> _12;
_16 = [RET.fld3,RET.fld3,_1,_1,RET.fld3];
_15 = (-2106500174_i32) as i8;
_15 = RET.fld3 - _1;
Goto(bb7)
}
bb7 = {
_4.0 = 3507570441_u32 as u8;
_17 = _7;
RET.fld0.0 = !_8.2.0;
_15 = RET.fld3 * _1;
RET.fld3 = _12 as i8;
_10 = _6 - _6;
RET.fld2 = [10713867946760543200_usize,5_usize,1_usize,1773642056779027704_usize,5_usize,2_usize,1_usize];
_12 = _1 as isize;
_17 = _18;
_18 = _7;
_6 = _2 + _10;
_22 = 126940573091067955506707313278510197531_i128 as u32;
_13 = -_2;
RET.fld2 = [8399785501012112988_usize,1_usize,12605248636072425361_usize,12334935880448456230_usize,4_usize,0_usize,1_usize];
_10 = _6;
_25.1 = &_10;
RET.fld3 = !_15;
_6 = _10;
_21 = 3708380713147156319_u64 + 5622817137757843162_u64;
_9 = _1 as f32;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_9 = _2;
_21 = RET.fld4 as u64;
_25.0.0 = -_10;
RET.fld2 = [3_usize,12810786448229932241_usize,5111814037606850099_usize,7_usize,4_usize,10524858661498027346_usize,5_usize];
_25.0 = (_10,);
Goto(bb8)
}
bb8 = {
_8.2.0 = RET.fld0.0;
_2 = _22 as f32;
_9 = _10;
_4.0 = !_8.2.0;
_21 = !4282972010368760329_u64;
Goto(bb9)
}
bb9 = {
RET.fld2 = [6_usize,5_usize,2975848473987425899_usize,2961280376185267093_usize,4_usize,2_usize,8783897226280712146_usize];
_18 = _17;
_2 = _25.0.0;
_7 = _3;
RET.fld2 = [3_usize,6285161699576497378_usize,7012068154377832483_usize,2_usize,13841539995754846137_usize,5_usize,9977191532440028580_usize];
_8.2 = _4;
_29.0 = [61405_u16,26851_u16,18579_u16,57966_u16,62904_u16,12157_u16,5116_u16];
_16 = [_15,RET.fld3,_15,_15,_1];
_1 = !_15;
_8.2.0 = !RET.fld0.0;
_30 = _12 << RET.fld3;
_23 = _18;
_28.1 = core::ptr::addr_of_mut!(RET.fld2);
RET.fld2 = [10907028507394340603_usize,2_usize,4371267342283506157_usize,4854668407263979523_usize,12028323884205786266_usize,6_usize,3_usize];
_29.0 = [26357_u16,38036_u16,2645_u16,10678_u16,11442_u16,49538_u16,6373_u16];
RET.fld4 = _30 as u128;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_10 = RET.fld4 as f32;
_16 = [_15,RET.fld3,RET.fld3,_15,RET.fld3];
Goto(bb10)
}
bb10 = {
_10 = _6;
_11 = (-16155_i16) * 8075_i16;
_28.1 = core::ptr::addr_of_mut!(RET.fld2);
Goto(bb11)
}
bb11 = {
_32 = _21 >> _30;
_28.2 = _29.0;
_24 = &_1;
_4.0 = RET.fld3 as u8;
_6 = _25.0.0 + _10;
_29 = (_28.2, 6_usize);
match _29.1 {
0 => bb1,
6 => bb12,
_ => bb10
}
}
bb12 = {
_29.1 = _4.0 as usize;
Call(_21 = core::intrinsics::transmute(_30), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_28.2 = [38030_u16,58783_u16,57132_u16,20562_u16,28311_u16,29601_u16,56461_u16];
_17 = _7;
_1 = RET.fld3;
_1 = -_15;
_6 = _30 as f32;
_29.0 = [31838_u16,48570_u16,11396_u16,39320_u16,44680_u16,22879_u16,5163_u16];
RET.fld3 = _15;
_24 = &_15;
Goto(bb14)
}
bb14 = {
_29 = (_28.2, 7_usize);
_2 = _9;
_2 = _6;
_11 = _5 as i16;
_31 = 49911_u16;
_6 = _9 * _10;
_7 = _23;
RET.fld4 = 9550818905302096784020459790153438451_u128 >> _1;
RET.fld0 = (_4.0,);
_25.1 = &_25.0.0;
_16 = [(*_24),(*_24),_15,(*_24),(*_24)];
_35 = (-66700433857875419612288641108444525294_i128);
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
_22 = 3789867530_u32 ^ 2988842468_u32;
_2 = _10;
RET.fld0.0 = _21 as u8;
RET.fld1 = core::ptr::addr_of_mut!(RET.fld4);
RET.fld3 = -_15;
_25.1 = &_37.0.0;
_5 = _10 as f64;
_14 = false;
_37.1 = &_37.0.0;
_21 = _32 & _32;
_13 = _6;
RET.fld4 = !90513197232821813249976271115151247969_u128;
_22 = 3714865517_u32;
_19 = core::ptr::addr_of_mut!(_29);
_37.0 = (_13,);
_6 = _10 + _13;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(0_usize, 18_usize, Move(_18), 11_usize, Move(_11), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(0_usize, 21_usize, Move(_21), 17_usize, Move(_17), 29_usize, Move(_29), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(0_usize, 32_usize, Move(_32), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1() -> u128 {
mir! {
type RET = u128;
let _1: &'static u128;
let _2: u8;
let _3: &'static i16;
let _4: [u32; 1];
let _5: f64;
let _6: [i8; 2];
let _7: char;
let _8: [i8; 5];
let _9: isize;
let _10: (u128,);
let _11: f64;
let _12: &'static Adt62;
let _13: *const Adt26;
let _14: f64;
let _15: [u16; 7];
let _16: *mut f32;
let _17: *mut f64;
let _18: (bool, f64);
let _19: f64;
let _20: [i8; 5];
let _21: [u32; 1];
let _22: (u8,);
let _23: &'static f32;
let _24: char;
let _25: [usize; 7];
let _26: ();
let _27: ();
{
_1 = &RET;
_1 = &(*_1);
_2 = 202_u8;
RET = 154905786412999693623742446393793340367_u128;
_4 = [3887354695_u32];
_1 = &RET;
_4 = [2370774848_u32];
_2 = !202_u8;
_2 = (-6967882593219916934_i64) as u8;
match RET {
0 => bb1,
154905786412999693623742446393793340367 => bb3,
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
_5 = 5844346551083781338_i64 as f64;
_2 = 5602102582584443795_usize as u8;
_1 = &RET;
_1 = &(*_1);
_6 = [(-126_i8),124_i8];
RET = !294910244030614424286060764502514213960_u128;
_2 = '\u{820f5}' as u8;
_6 = [(-16_i8),(-56_i8)];
_1 = &RET;
RET = 304971725908298320613151999267190072168_u128;
_1 = &RET;
_4 = [254786521_u32];
_8 = [(-37_i8),(-53_i8),(-87_i8),13_i8,(-15_i8)];
_1 = &(*_1);
_2 = 16069_u16 as u8;
_8 = [(-90_i8),(-3_i8),(-80_i8),99_i8,(-107_i8)];
_5 = 4561250804816947718_u64 as f64;
_8 = [(-93_i8),41_i8,106_i8,(-43_i8),29_i8];
RET = 17426857351805050363094074760065135164_u128 ^ 336894334226383820842645695496696328833_u128;
_1 = &RET;
_2 = 9_u8 >> (*_1);
_8 = [104_i8,(-93_i8),(-54_i8),68_i8,101_i8];
_6 = [9_i8,96_i8];
_8 = [105_i8,(-24_i8),(-60_i8),75_i8,(-84_i8)];
_6 = [(-52_i8),64_i8];
_5 = 16282_u16 as f64;
RET = 85_isize as u128;
_4 = [2383383399_u32];
Goto(bb4)
}
bb4 = {
_7 = '\u{9ea6b}';
_9 = !(-9223372036854775808_isize);
_9 = -50_isize;
_10.0 = RET ^ RET;
_4 = [4101718755_u32];
_2 = !245_u8;
_6 = [88_i8,(-2_i8)];
_9 = (-9223372036854775808_isize) << _2;
_4 = [763571143_u32];
_8 = [96_i8,(-50_i8),(-91_i8),76_i8,99_i8];
RET = (-76935167421809037945705184173005422639_i128) as u128;
_1 = &RET;
RET = _10.0;
_5 = 13179613450639804276_u64 as f64;
_10 = (RET,);
_9 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_2 = !42_u8;
_6 = [24_i8,(-44_i8)];
_8 = [(-19_i8),82_i8,(-10_i8),(-11_i8),(-71_i8)];
_1 = &_10.0;
_7 = '\u{99ef9}';
RET = 1234013186_u32 as u128;
_4 = [4275247155_u32];
_6 = [1_i8,22_i8];
_10.0 = 7554643822899812322_u64 as u128;
_1 = &RET;
_1 = &(*_1);
RET = 11966737182954681150_u64 as u128;
_9 = !15_isize;
Goto(bb5)
}
bb5 = {
RET = _10.0;
_8 = [55_i8,(-75_i8),(-73_i8),54_i8,(-55_i8)];
_10.0 = 11536233689537852339_u64 as u128;
Goto(bb6)
}
bb6 = {
RET = _10.0;
_11 = _5;
_1 = &_10.0;
_7 = '\u{c90f4}';
_1 = &RET;
_9 = 9223372036854775807_isize & (-9223372036854775808_isize);
_1 = &(*_1);
_5 = -_11;
_14 = _5 + _11;
RET = _10.0;
_10.0 = RET;
_8 = [105_i8,56_i8,(-27_i8),36_i8,20_i8];
_6 = [39_i8,(-26_i8)];
_5 = _11 * _14;
_1 = &RET;
RET = _10.0;
_1 = &RET;
_6 = [116_i8,97_i8];
Call(_18.1 = fn2(Move(_1), _5, _10.0, _9, _8, _5, _4, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17 = core::ptr::addr_of_mut!(_5);
_18 = (true, (*_17));
_11 = _18.1 - (*_17);
_15 = [55632_u16,34608_u16,11215_u16,48383_u16,32067_u16,46540_u16,29830_u16];
RET = 70260678117776712313519798908768920001_i128 as u128;
_19 = 16365759198759941088_u64 as f64;
_11 = (-1047946604_i32) as f64;
_11 = _18.1 + (*_17);
_8 = [96_i8,7_i8,(-108_i8),(-74_i8),38_i8];
_8 = [102_i8,11_i8,20_i8,48_i8,(-60_i8)];
_14 = _5 - (*_17);
Goto(bb8)
}
bb8 = {
_1 = &RET;
_9 = !(-42_isize);
_18.1 = -_14;
_20 = [(-92_i8),(-124_i8),97_i8,10_i8,77_i8];
_7 = '\u{97f64}';
_1 = &_10.0;
RET = _10.0 & (*_1);
Call(_9 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14 = -(*_17);
_20 = [(-92_i8),(-106_i8),104_i8,11_i8,(-28_i8)];
Goto(bb10)
}
bb10 = {
_8 = [(-54_i8),87_i8,115_i8,(-79_i8),116_i8];
_18.0 = RET != (*_1);
_18.1 = 9_i8 as f64;
_18.0 = false;
(*_17) = _14;
RET = (*_1) & _10.0;
Goto(bb11)
}
bb11 = {
_18 = (true, _14);
_6 = [(-43_i8),(-87_i8)];
_15 = [37006_u16,29268_u16,50181_u16,24577_u16,6924_u16,32226_u16,23659_u16];
_1 = &RET;
_7 = '\u{716c7}';
(*_17) = _14;
_9 = _7 as isize;
_21 = [496419955_u32];
_8 = [7_i8,0_i8,124_i8,38_i8,110_i8];
_6 = [(-18_i8),(-94_i8)];
_17 = core::ptr::addr_of_mut!(_14);
_22 = (_2,);
_11 = (*_17) * _5;
_18.0 = !false;
_11 = -_5;
(*_17) = -_11;
_17 = core::ptr::addr_of_mut!((*_17));
(*_17) = _11;
_10.0 = 4148615439214936950_i64 as u128;
_5 = _9 as f64;
_22 = (_2,);
Goto(bb12)
}
bb12 = {
_7 = '\u{dfa6a}';
_10.0 = RET * (*_1);
_15 = [12263_u16,6817_u16,52902_u16,22325_u16,44448_u16,18570_u16,27637_u16];
Goto(bb13)
}
bb13 = {
_1 = &(*_1);
_5 = (*_17);
_9 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_18 = (true, (*_17));
_11 = (*_17);
Goto(bb14)
}
bb14 = {
_1 = &_10.0;
_14 = -_11;
_10 = (RET,);
_22.0 = !_2;
_18 = (false, (*_17));
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(1_usize, 2_usize, Move(_2), 22_usize, Move(_22), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(1_usize, 4_usize, Move(_4), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: &'static u128,mut _2: f64,mut _3: u128,mut _4: isize,mut _5: [i8; 5],mut _6: f64,mut _7: [u32; 1],mut _8: f64) -> f64 {
mir! {
type RET = f64;
let _9: u8;
let _10: ((f32,), &'static f32);
let _11: Adt78;
let _12: i64;
let _13: f32;
let _14: u16;
let _15: f32;
let _16: u64;
let _17: char;
let _18: f64;
let _19: i32;
let _20: Adt69;
let _21: *mut i128;
let _22: &'static [u32; 1];
let _23: Adt50;
let _24: [bool; 4];
let _25: &'static (bool, f64);
let _26: (Adt26, *mut i128);
let _27: *mut (f32,);
let _28: &'static *mut i128;
let _29: *const Adt34;
let _30: (u128,);
let _31: (&'static usize, u8, [char; 7], *const Adt26);
let _32: bool;
let _33: f32;
let _34: [i8; 2];
let _35: [bool; 4];
let _36: i8;
let _37: u16;
let _38: char;
let _39: &'static ((f32,), &'static f32);
let _40: (((f32,), &'static f32),);
let _41: char;
let _42: &'static *mut Adt26;
let _43: *const &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _44: (&'static usize, u8, [char; 7], *const Adt26);
let _45: &'static u128;
let _46: f64;
let _47: *mut (bool, f64);
let _48: Adt76;
let _49: f32;
let _50: ();
let _51: ();
{
_6 = _8;
_1 = &_3;
_4 = (-30_isize);
RET = _2 * _2;
RET = _2;
_6 = 241878025_u32 as f64;
_7 = [2043203582_u32];
_5 = [(-81_i8),(-110_i8),(-123_i8),(-7_i8),35_i8];
_6 = _8;
_4 = 9223372036854775807_isize;
RET = _8;
_9 = !162_u8;
_5 = [(-94_i8),(-30_i8),(-3_i8),1_i8,(-9_i8)];
_4 = -9223372036854775807_isize;
_9 = RET as u8;
_4 = !9223372036854775807_isize;
RET = 24436_i16 as f64;
_10.0.0 = (-13808_i16) as f32;
_1 = &_3;
_10.1 = &_10.0.0;
_3 = (-21_i8) as u128;
_12 = '\u{c10}' as i64;
_7 = [2295726029_u32];
_1 = &_3;
Goto(bb1)
}
bb1 = {
_2 = -_8;
_2 = -_6;
_13 = _10.0.0;
Goto(bb2)
}
bb2 = {
_3 = !91765510227623513473363606261391173506_u128;
_14 = 32419_u16;
_2 = -RET;
_13 = -_10.0.0;
_4 = (-84_isize);
_13 = _10.0.0 * _10.0.0;
_4 = !9223372036854775807_isize;
_6 = _8;
_9 = 97_u8 - 27_u8;
_9 = 177_u8;
_15 = 25147_i16 as f32;
RET = -_8;
_2 = RET;
_12 = !(-1800918325539980995_i64);
RET = (-750975899_i32) as f64;
_10.1 = &_13;
_13 = -_10.0.0;
_10.0.0 = 14418045224582084162_u64 as f32;
_17 = '\u{4235f}';
_10.1 = &_15;
_4 = !9223372036854775807_isize;
_16 = 1772044322527442753_u64 | 17803950498750052205_u64;
_15 = _13 * _13;
_1 = &_3;
_9 = !66_u8;
match _14 {
0 => bb1,
1 => bb3,
2 => bb4,
32419 => bb6,
_ => bb5
}
}
bb3 = {
_2 = -_8;
_2 = -_6;
_13 = _10.0.0;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_16 = 15756022119626266124_u64 ^ 2246748950548972687_u64;
_8 = -RET;
RET = -_2;
_6 = -RET;
_6 = _13 as f64;
_20.fld2 = (true, _8);
_10.0 = (_13,);
_15 = _10.0.0 * _10.0.0;
Call(_19 = fn3(Move(_1), _20.fld2.0, _5, _15, RET, RET, _5, (*_1)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _13 as u128;
_12 = 514807864584732201_i64 | 7388051985873340208_i64;
_10.1 = &_13;
_10.1 = &_15;
_10.0 = (_15,);
_18 = -RET;
_15 = -_13;
_15 = -_13;
_20.fld1 = _7;
_20.fld1 = [3943926709_u32];
_9 = _19 as u8;
_7 = _20.fld1;
_10.1 = &_10.0.0;
_5 = [116_i8,(-82_i8),(-48_i8),(-48_i8),92_i8];
_17 = '\u{44306}';
_10.0.0 = _15;
_10.1 = &_15;
RET = -_8;
_11 = Adt78::Variant1 { fld0: _19 };
RET = _6 - _2;
SetDiscriminant(_11, 0);
_4 = 125049052513525411543588854569866486654_i128 as isize;
place!(Field::<(u8,)>(Variant(_11, 0), 1)) = (_9,);
place!(Field::<(u8,)>(Variant(_11, 0), 1)) = (_9,);
_20.fld2.0 = !false;
_20.fld2.0 = !false;
match _14 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
32419 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_22 = &_7;
_6 = RET;
_22 = &_7;
_2 = 636891220_u32 as f64;
_22 = &(*_22);
_18 = _2;
Goto(bb10)
}
bb10 = {
_7 = [3413600879_u32];
_6 = -RET;
_11 = Adt78::Variant1 { fld0: _19 };
_5 = [(-75_i8),30_i8,(-82_i8),(-70_i8),114_i8];
_20.fld2 = (false, _6);
_27 = core::ptr::addr_of_mut!(_10.0);
_14 = 1564_u16 ^ 380_u16;
_7 = _20.fld1;
_26.0 = Adt26::Variant0 { fld0: 3485371874_u32,fld1: 1063052963659788138_usize,fld2: _4,fld3: _9,fld4: 39634853759929163073501782356881004603_i128 };
_16 = 5962623941330801609_u64;
_18 = _20.fld2.1;
_30 = (_3,);
_26.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_26.0, 0), 4)));
_3 = _30.0;
place!(Field::<i128>(Variant(_26.0, 0), 4)) = _15 as i128;
_10.1 = &(*_27).0;
Call(place!(Field::<isize>(Variant(_26.0, 0), 2)) = core::intrinsics::transmute(_12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_26.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_26.0, 0), 4)));
_7 = [3250854920_u32];
_31.0 = &place!(Field::<usize>(Variant(_26.0, 0), 1));
_10.0.0 = _13 - _15;
_31.1 = Field::<u8>(Variant(_26.0, 0), 3);
_2 = _20.fld2.1;
_34 = [67_i8,60_i8];
_15 = Field::<u8>(Variant(_26.0, 0), 3) as f32;
_18 = -_20.fld2.1;
_30.0 = !_3;
_14 = _12 as u16;
_37 = _14;
_38 = _17;
_20.fld2 = (false, _18);
match _16 {
0 => bb8,
1 => bb2,
5962623941330801609 => bb12,
_ => bb4
}
}
bb12 = {
SetDiscriminant(_11, 1);
_19 = 1087034838_i32 >> Field::<isize>(Variant(_26.0, 0), 2);
_1 = &_3;
_31.1 = Field::<u8>(Variant(_26.0, 0), 3);
_20.fld2 = (true, _2);
place!(Field::<i128>(Variant(_26.0, 0), 4)) = 6812369855991834249585607042540026450_i128 - (-35866479400921399736778929959148159343_i128);
_26.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_26.0, 0), 4)));
_24 = [_20.fld2.0,_20.fld2.0,_20.fld2.0,_20.fld2.0];
_40.0.0 = (_10.0.0,);
_31.2 = [_38,_38,_38,_17,_17,_17,_38];
match _16 {
0 => bb9,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
5962623941330801609 => bb20,
_ => bb19
}
}
bb13 = {
_26.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_26.0, 0), 4)));
_7 = [3250854920_u32];
_31.0 = &place!(Field::<usize>(Variant(_26.0, 0), 1));
_10.0.0 = _13 - _15;
_31.1 = Field::<u8>(Variant(_26.0, 0), 3);
_2 = _20.fld2.1;
_34 = [67_i8,60_i8];
_15 = Field::<u8>(Variant(_26.0, 0), 3) as f32;
_18 = -_20.fld2.1;
_30.0 = !_3;
_14 = _12 as u16;
_37 = _14;
_38 = _17;
_20.fld2 = (false, _18);
match _16 {
0 => bb8,
1 => bb2,
5962623941330801609 => bb12,
_ => bb4
}
}
bb14 = {
_7 = [3413600879_u32];
_6 = -RET;
_11 = Adt78::Variant1 { fld0: _19 };
_5 = [(-75_i8),30_i8,(-82_i8),(-70_i8),114_i8];
_20.fld2 = (false, _6);
_27 = core::ptr::addr_of_mut!(_10.0);
_14 = 1564_u16 ^ 380_u16;
_7 = _20.fld1;
_26.0 = Adt26::Variant0 { fld0: 3485371874_u32,fld1: 1063052963659788138_usize,fld2: _4,fld3: _9,fld4: 39634853759929163073501782356881004603_i128 };
_16 = 5962623941330801609_u64;
_18 = _20.fld2.1;
_30 = (_3,);
_26.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_26.0, 0), 4)));
_3 = _30.0;
place!(Field::<i128>(Variant(_26.0, 0), 4)) = _15 as i128;
_10.1 = &(*_27).0;
Call(place!(Field::<isize>(Variant(_26.0, 0), 2)) = core::intrinsics::transmute(_12), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_2 = -_8;
_2 = -_6;
_13 = _10.0.0;
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_2 = -_8;
_2 = -_6;
_13 = _10.0.0;
Goto(bb2)
}
bb18 = {
_16 = 15756022119626266124_u64 ^ 2246748950548972687_u64;
_8 = -RET;
RET = -_2;
_6 = -RET;
_6 = _13 as f64;
_20.fld2 = (true, _8);
_10.0 = (_13,);
_15 = _10.0.0 * _10.0.0;
Call(_19 = fn3(Move(_1), _20.fld2.0, _5, _15, RET, RET, _5, (*_1)), ReturnTo(bb7), UnwindUnreachable())
}
bb19 = {
Return()
}
bb20 = {
_39 = &_10;
_12 = (-8343362682539142846_i64);
_19 = _2 as i32;
_33 = (*_27).0;
_15 = _20.fld2.1 as f32;
_44.2 = _31.2;
place!(Field::<i32>(Variant(_11, 1), 0)) = _16 as i32;
_31.3 = core::ptr::addr_of!(_26.0);
_30.0 = _3 | (*_1);
_24 = [_20.fld2.0,_20.fld2.0,_20.fld2.0,_20.fld2.0];
place!(Field::<i32>(Variant(_11, 1), 0)) = _19;
_27 = core::ptr::addr_of_mut!((*_27));
SetDiscriminant(_11, 1);
_37 = _14 & _14;
_10.0.0 = _15 - _15;
_41 = _38;
_25 = &_20.fld2;
place!(Field::<u32>(Variant(_26.0, 0), 0)) = 1314155516_u32 >> _30.0;
Goto(bb21)
}
bb21 = {
Call(_50 = dump_var(2_usize, 16_usize, Move(_16), 4_usize, Move(_4), 38_usize, Move(_38), 3_usize, Move(_3)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_50 = dump_var(2_usize, 30_usize, Move(_30), 37_usize, Move(_37), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: &'static u128,mut _2: bool,mut _3: [i8; 5],mut _4: f32,mut _5: f64,mut _6: f64,mut _7: [i8; 5],mut _8: u128) -> i32 {
mir! {
type RET = i32;
let _9: usize;
let _10: i16;
let _11: Adt34;
let _12: f64;
let _13: *mut ((u32,), *mut [usize; 7], [u16; 7]);
let _14: &'static [u32; 1];
let _15: f64;
let _16: isize;
let _17: ((u32,), *mut [usize; 7], [u16; 7]);
let _18: f32;
let _19: [u16; 7];
let _20: [u32; 1];
let _21: u128;
let _22: &'static *mut Adt26;
let _23: char;
let _24: u64;
let _25: u128;
let _26: ([u16; 7], usize);
let _27: Adt34;
let _28: isize;
let _29: bool;
let _30: f32;
let _31: [i128; 1];
let _32: Adt69;
let _33: [usize; 7];
let _34: (f32,);
let _35: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _36: *mut (f32,);
let _37: isize;
let _38: isize;
let _39: &'static i32;
let _40: [i128; 1];
let _41: f64;
let _42: f64;
let _43: &'static [bool; 4];
let _44: (i128, &'static i16, &'static *mut i128, (u32,));
let _45: f64;
let _46: (u32,);
let _47: *const Adt26;
let _48: Adt50;
let _49: char;
let _50: *mut [usize; 7];
let _51: ();
let _52: ();
{
_1 = &_8;
RET = -(-246713691_i32);
_4 = _5 as f32;
_5 = _6;
_4 = 65584349414950645504904067921558587633_i128 as f32;
RET = !442595211_i32;
_7 = [(-87_i8),(-38_i8),(-7_i8),66_i8,(-1_i8)];
_4 = (-90_isize) as f32;
_10 = -4549_i16;
_3 = _7;
_8 = _2 as u128;
_4 = (-9223372036854775808_isize) as f32;
_3 = [88_i8,(-94_i8),50_i8,45_i8,51_i8];
_6 = -_5;
RET = (-1778479696_i32);
_2 = true;
_7 = [75_i8,25_i8,83_i8,0_i8,(-40_i8)];
RET = 18418835413916976654_u64 as i32;
_11.fld0.0 = !29_u8;
_9 = 12548382514033967627_usize + 4225210979436568920_usize;
_11.fld1 = core::ptr::addr_of_mut!(_8);
_3 = [(-83_i8),77_i8,(-46_i8),(-3_i8),33_i8];
_11.fld0 = (252_u8,);
_1 = &_11.fld4;
RET = 1747389946_i32 * (-2089838550_i32);
_11.fld1 = core::ptr::addr_of_mut!(_8);
Goto(bb1)
}
bb1 = {
_11.fld3 = -(-7_i8);
_11.fld1 = core::ptr::addr_of_mut!(_8);
_12 = _8 as f64;
RET = (-1372573182_i32) >> _8;
_7 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_2 = true;
_3 = _7;
_5 = _6;
_1 = &_11.fld4;
_7 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_8 = !182873648659886526039637419535609761482_u128;
_11.fld2 = [_9,_9,_9,_9,_9,_9,_9];
Call(_4 = fn4(RET, _11.fld0.0, _9, _11.fld2, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11.fld2 = [_9,_9,_9,_9,_9,_9,_9];
_17.0.0 = !1262723294_u32;
_11.fld0.0 = 146090806914704786743403093080214863669_i128 as u8;
Call(_11.fld0.0 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17.2 = [43318_u16,56238_u16,39063_u16,36066_u16,59037_u16,50873_u16,41140_u16];
_12 = _5;
_17.1 = core::ptr::addr_of_mut!(_11.fld2);
_11.fld0 = (68_u8,);
_12 = -_6;
_15 = _12;
_11.fld4 = _8;
_11.fld3 = -(-108_i8);
_10 = (-30084_i16);
_1 = &_8;
_16 = 2_isize;
_18 = 1039263935357129651_i64 as f32;
_17.1 = core::ptr::addr_of_mut!(_11.fld2);
_11.fld0 = (46_u8,);
_19 = [38277_u16,55630_u16,17824_u16,4148_u16,6488_u16,56747_u16,44427_u16];
_17.1 = core::ptr::addr_of_mut!(_11.fld2);
Goto(bb4)
}
bb4 = {
_8 = _11.fld4;
_3 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_18 = -_4;
_14 = &_20;
_3 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_24 = _11.fld3 as u64;
_3 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_17.2 = _19;
_6 = _15 - _12;
_8 = _11.fld4;
_7 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_17.2 = _19;
_27.fld0 = (_11.fld0.0,);
_20 = [_17.0.0];
_16 = (-160296031053574416912834521153404948905_i128) as isize;
_11.fld1 = core::ptr::addr_of_mut!(_25);
Goto(bb5)
}
bb5 = {
_26.0 = [38267_u16,41557_u16,15833_u16,59207_u16,63485_u16,19874_u16,19097_u16];
_11.fld4 = !_8;
_27.fld4 = 125055654533602115426306673389887149973_i128 as u128;
_10 = (-5150_i16) + 3638_i16;
_28 = _16 >> _9;
_26.1 = _9 ^ _9;
_21 = _10 as u128;
_13 = core::ptr::addr_of_mut!(_17);
_5 = _6 - _12;
_20 = [_17.0.0];
_27.fld4 = _21;
_11.fld2 = [_9,_9,_26.1,_26.1,_26.1,_26.1,_26.1];
_14 = &_20;
_20 = [_17.0.0];
RET = 1631605502_i32 * 1543167804_i32;
_26 = ((*_13).2, _9);
_28 = _16;
_12 = _9 as f64;
_21 = _27.fld4;
_17.1 = core::ptr::addr_of_mut!(_11.fld2);
_11.fld0.0 = _27.fld0.0;
_21 = _27.fld4;
_6 = _5 - _5;
_26.1 = !_9;
Goto(bb6)
}
bb6 = {
_27 = Move(_11);
_17.0 = (767811312_u32,);
Goto(bb7)
}
bb7 = {
_27.fld4 = _8;
_26 = (_17.2, _9);
_24 = 5975651182618027070_u64 ^ 3240423227461776068_u64;
_19 = _17.2;
_4 = (-7067271391602378709_i64) as f32;
_31 = [(-78831952082474705453372327173163280319_i128)];
(*_13).1 = core::ptr::addr_of_mut!(_27.fld2);
_15 = _6;
_30 = _21 as f32;
match _17.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
767811312 => bb8,
_ => bb6
}
}
bb8 = {
_3 = _7;
_29 = !_2;
_32.fld2.1 = -_6;
_25 = _21 + _21;
_11.fld1 = core::ptr::addr_of_mut!(_8);
_1 = &_25;
(*_13).1 = core::ptr::addr_of_mut!(_27.fld2);
_30 = -_18;
_32.fld0 = core::ptr::addr_of_mut!(_33);
_27.fld1 = core::ptr::addr_of_mut!(_25);
_26 = ((*_13).2, _9);
_17.1 = core::ptr::addr_of_mut!(_33);
_11.fld4 = !_25;
match (*_13).0.0 {
767811312 => bb9,
_ => bb1
}
}
bb9 = {
_7 = _3;
_24 = 11313843756385090731_u64 & 12991792893835538557_u64;
(*_13).1 = core::ptr::addr_of_mut!(_33);
_37 = _10 as isize;
_32.fld0 = core::ptr::addr_of_mut!(_27.fld2);
_26 = ((*_13).2, _9);
_32.fld2.0 = !_29;
(*_13).1 = Move(_32.fld0);
_39 = &RET;
_23 = '\u{bbcbb}';
_32.fld2.0 = _2 | _29;
(*_13).0 = (3799904778_u32,);
_11.fld3 = _32.fld2.1 as i8;
_4 = -_18;
(*_13).0 = (1165758362_u32,);
_32.fld2.0 = !_2;
_36 = core::ptr::addr_of_mut!(_34);
_35.2.0 = !_27.fld0.0;
_11.fld4 = (*_1) >> _8;
Call(_4 = fn19(Move(_39), Move(_1), _17.2, Move(_27.fld1), Move((*_13))), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19 = _26.0;
_14 = &_20;
_40 = [133675533780715384463130236206035985729_i128];
_24 = 3098615187204712785_u64 * 175271638321218923_u64;
_36 = core::ptr::addr_of_mut!(_34);
_30 = -_4;
(*_36).0 = _4 + _18;
(*_13).0.0 = _32.fld2.0 as u32;
_3 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_16 = _27.fld4 as isize;
(*_13).1 = core::ptr::addr_of_mut!(_33);
_4 = _34.0;
_39 = &RET;
_27.fld0.0 = _35.2.0 & _35.2.0;
_24 = 12727100776624180771_u64;
_17.2 = _26.0;
_14 = &_20;
(*_36).0 = -_18;
_19 = [13008_u16,59183_u16,42527_u16,29267_u16,31976_u16,38930_u16,33493_u16];
_27.fld3 = _11.fld3;
_38 = -_37;
_32.fld1 = (*_14);
_27.fld0 = _35.2;
_10 = 11811_i16 & (-31426_i16);
Call(_27.fld0.0 = core::intrinsics::bswap(_35.2.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_32.fld0 = core::ptr::addr_of_mut!(_11.fld2);
_20 = [_17.0.0];
_17.0 = (305053430_u32,);
_38 = -_37;
_21 = _8;
_26.1 = !_9;
(*_36).0 = _18 * _18;
_11.fld4 = _25;
_7 = _3;
_10 = -(-20951_i16);
_41 = _15 * _5;
(*_13).1 = core::ptr::addr_of_mut!(_11.fld2);
_26.0 = [26036_u16,25849_u16,30508_u16,40853_u16,3814_u16,41510_u16,60133_u16];
_37 = _28 | _16;
_16 = -_37;
_30 = (*_36).0 - (*_36).0;
_5 = _10 as f64;
_25 = _26.1 as u128;
_27.fld4 = _30 as u128;
_21 = !_27.fld4;
match (*_13).0.0 {
0 => bb7,
1 => bb10,
2 => bb6,
3 => bb12,
4 => bb13,
5 => bb14,
305053430 => bb16,
_ => bb15
}
}
bb12 = {
_11.fld2 = [_9,_9,_9,_9,_9,_9,_9];
_17.0.0 = !1262723294_u32;
_11.fld0.0 = 146090806914704786743403093080214863669_i128 as u8;
Call(_11.fld0.0 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_27 = Move(_11);
_17.0 = (767811312_u32,);
Goto(bb7)
}
bb14 = {
_11.fld3 = -(-7_i8);
_11.fld1 = core::ptr::addr_of_mut!(_8);
_12 = _8 as f64;
RET = (-1372573182_i32) >> _8;
_7 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_2 = true;
_3 = _7;
_5 = _6;
_1 = &_11.fld4;
_7 = [_11.fld3,_11.fld3,_11.fld3,_11.fld3,_11.fld3];
_8 = !182873648659886526039637419535609761482_u128;
_11.fld2 = [_9,_9,_9,_9,_9,_9,_9];
Call(_4 = fn4(RET, _11.fld0.0, _9, _11.fld2, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_27.fld4 = _8;
_26 = (_17.2, _9);
_24 = 5975651182618027070_u64 ^ 3240423227461776068_u64;
_19 = _17.2;
_4 = (-7067271391602378709_i64) as f32;
_31 = [(-78831952082474705453372327173163280319_i128)];
(*_13).1 = core::ptr::addr_of_mut!(_27.fld2);
_15 = _6;
_30 = _21 as f32;
match _17.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
767811312 => bb8,
_ => bb6
}
}
bb16 = {
_30 = _11.fld3 as f32;
_11.fld1 = core::ptr::addr_of_mut!(_27.fld4);
_27.fld0.0 = !_35.2.0;
_44.3.0 = _11.fld4 as u32;
_2 = _29 & _32.fld2.0;
_45 = _32.fld2.1 * _6;
_17.2 = [6385_u16,23588_u16,59860_u16,57999_u16,49143_u16,39087_u16,46432_u16];
(*_13).1 = core::ptr::addr_of_mut!(_27.fld2);
_27.fld1 = Move(_11.fld1);
_28 = _23 as isize;
(*_36) = (_18,);
_45 = _15 - _41;
_13 = core::ptr::addr_of_mut!((*_13));
_32.fld2.0 = _2;
_32.fld0 = core::ptr::addr_of_mut!(_27.fld2);
_38 = _28 & _16;
_38 = _16;
Goto(bb17)
}
bb17 = {
Call(_51 = dump_var(3_usize, 19_usize, Move(_19), 31_usize, Move(_31), 9_usize, Move(_9), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(3_usize, 37_usize, Move(_37), 2_usize, Move(_2), 40_usize, Move(_40), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(3_usize, 23_usize, Move(_23), 29_usize, Move(_29), 52_usize, _52, 52_usize, _52), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i32,mut _2: u8,mut _3: usize,mut _4: [usize; 7],mut _5: bool,mut _6: i16) -> f32 {
mir! {
type RET = f32;
let _7: [i128; 8];
let _8: &'static i32;
let _9: isize;
let _10: u8;
let _11: u8;
let _12: usize;
let _13: [u16; 8];
let _14: Adt78;
let _15: char;
let _16: *mut f32;
let _17: isize;
let _18: &'static ((f32,), &'static f32);
let _19: (Adt26, *mut i128);
let _20: bool;
let _21: [char; 7];
let _22: &'static f32;
let _23: [bool; 4];
let _24: f64;
let _25: bool;
let _26: char;
let _27: i32;
let _28: u8;
let _29: &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _30: u32;
let _31: f32;
let _32: &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _33: f64;
let _34: (u32,);
let _35: (f32,);
let _36: i64;
let _37: isize;
let _38: bool;
let _39: isize;
let _40: [i8; 5];
let _41: u16;
let _42: f64;
let _43: ();
let _44: ();
{
RET = (-101870841329478417941383717823976339951_i128) as f32;
_5 = true & false;
RET = _6 as f32;
_5 = !false;
Call(RET = fn5(_3, _1, _2, _4, _1, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !true;
_1 = 1052795623_i32;
_7 = [(-34269680869315869451656283153301039338_i128),(-29568327408076218236870032959890167209_i128),(-30263273374526107922177873687615896264_i128),(-8409805808991505096147057176260209110_i128),133372604913454614948224744807383067225_i128,(-58427332824625250275424156720669183371_i128),81213734468531638816021701739698869852_i128,(-106045373073764004581338985976667106400_i128)];
_7 = [63336776526919514576789792679424209506_i128,(-146358128511298484825622230458957737324_i128),(-18268599259294623660438593430470989011_i128),(-168477174399431505766561921402794801578_i128),(-96836095833531178694433980909094781992_i128),64066612920165187945582266793810278372_i128,30117942789680344508695287808745155046_i128,(-85158052989956031831502687349181267276_i128)];
_7 = [(-40509348736096342470195420649865016599_i128),14143506164761808541571498689817891330_i128,125541242019951471915591101132259579515_i128,(-27014575838279333180197464910248169271_i128),78671679970367897878187847462716545041_i128,155825300132340929034265578642555099969_i128,(-67717483325422440430042029984649069973_i128),154262028007191713375902801337989099877_i128];
_1 = 838052800_i32;
_8 = &_1;
_6 = 126467136006319624295732873036125552938_u128 as i16;
_9 = 25162_u16 as isize;
_7 = [109526453946546052173563759108198630237_i128,115353396762697520863213467054421216809_i128,22389874612187027489117673819248183386_i128,(-118818831860246431354631300945994029936_i128),(-165096024682797294873650242714094507138_i128),(-74743950930508865557219874868949805148_i128),(-36668897226416067906724201309834089814_i128),148404315874362389055687055361093509871_i128];
_7 = [88068548323284789473476760133092932966_i128,(-167712815941878670024763177399720093873_i128),(-146662557352741732454461404374623948008_i128),(-23390453771722010828775175450471082054_i128),(-23601836163367992392782038621407150284_i128),(-65305054621046779940775643826311000436_i128),53407629821566571898999085109700694657_i128,54261038250288986920162808086647173675_i128];
_5 = RET < RET;
_2 = 118_u8 & 38_u8;
RET = _3 as f32;
_10 = _2;
_10 = _3 as u8;
_6 = 85814752810430296054019437513268559626_u128 as i16;
_9 = -9223372036854775807_isize;
Call(_6 = core::intrinsics::bswap(12914_i16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = [72970059585383736420228185854910912350_i128,(-141898168319411973090886716846859213685_i128),163817638705883185309934057067799785916_i128,(-99740277034910358817068295625680108344_i128),(-31517711932731825917751207140257193641_i128),16514413767210585892361139464021134574_i128,(-85511224234556051434456515384388963848_i128),(-33914079401064770114248894143371480193_i128)];
_13 = [3823_u16,8772_u16,63837_u16,51413_u16,36390_u16,23257_u16,8129_u16,65308_u16];
_3 = 4_usize >> _9;
_13 = [34275_u16,1515_u16,61961_u16,37821_u16,33681_u16,5333_u16,44092_u16,431_u16];
_2 = _10 << _10;
_15 = '\u{6fc0f}';
_7 = [(-46277491330428475125669136600983085103_i128),(-37582338312312353539506713529575538817_i128),155257652128161596293535988738698217826_i128,(-58609918655942114697960098635938849801_i128),1660284019321253870476276243158825215_i128,116822200311853342318471751292224690581_i128,157522977535839288151753386177440910037_i128,(-57287902155885992290997388594299741812_i128)];
_11 = _10;
_8 = &(*_8);
_1 = _3 as i32;
_12 = 1373831146_u32 as usize;
_3 = _9 as usize;
RET = _11 as f32;
_14 = Adt78::Variant1 { fld0: _1 };
Goto(bb3)
}
bb3 = {
_17 = _9;
_19.0 = Adt26::Variant1 { fld0: RET,fld1: _7 };
_13 = [1465_u16,5003_u16,1968_u16,46934_u16,30325_u16,16416_u16,17319_u16,3063_u16];
_11 = _2 ^ _2;
_13 = [30877_u16,18651_u16,49886_u16,37029_u16,44908_u16,39776_u16,55510_u16,16771_u16];
_4 = [_12,_3,_12,_12,_12,_3,_12];
_9 = _11 as isize;
_8 = &_1;
_8 = &(*_8);
_4 = [_3,_3,_12,_3,_3,_3,_12];
place!(Field::<i32>(Variant(_14, 1), 0)) = 67571056476899381502673282828271289755_u128 as i32;
_5 = false;
place!(Field::<f32>(Variant(_19.0, 1), 0)) = RET;
place!(Field::<[i128; 8]>(Variant(_19.0, 1), 1)) = [(-57055049586459820038494368937598653134_i128),(-130928167887008483586088066109702026804_i128),(-10603557676642539576392746956164890312_i128),(-95680881258887673861253637066760424885_i128),(-108849640243395914322823006582517658889_i128),(-148207978329993799534012459386013261883_i128),(-157103461640174286956400404394407097668_i128),48625497090276445222442746994006425228_i128];
_8 = &_1;
_2 = _11 + _11;
_16 = core::ptr::addr_of_mut!(RET);
_21 = [_15,_15,_15,_15,_15,_15,_15];
_22 = &place!(Field::<f32>(Variant(_19.0, 1), 0));
_9 = _15 as isize;
_21 = [_15,_15,_15,_15,_15,_15,_15];
_22 = &(*_16);
RET = _11 as f32;
_14 = Adt78::Variant1 { fld0: (*_8) };
_10 = _2;
_22 = &place!(Field::<f32>(Variant(_19.0, 1), 0));
Goto(bb4)
}
bb4 = {
_21 = [_15,_15,_15,_15,_15,_15,_15];
place!(Field::<i32>(Variant(_14, 1), 0)) = -(*_8);
_5 = !true;
_16 = core::ptr::addr_of_mut!((*_22));
_20 = _5;
Goto(bb5)
}
bb5 = {
SetDiscriminant(_19.0, 0);
_4 = [_12,_12,_12,_3,_12,_12,_3];
SetDiscriminant(_14, 1);
_23 = [_5,_5,_20,_5];
_9 = _17;
_10 = _20 as u8;
_24 = 7369984482611591661408961100992876840_i128 as f64;
_22 = &RET;
place!(Field::<usize>(Variant(_19.0, 0), 1)) = _3 << _9;
place!(Field::<isize>(Variant(_19.0, 0), 2)) = _9;
_14 = Adt78::Variant1 { fld0: _1 };
_17 = _9;
_16 = core::ptr::addr_of_mut!((*_22));
_10 = !_2;
_20 = _5;
_19.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_19.0, 0), 4)));
Goto(bb6)
}
bb6 = {
_22 = &(*_22);
SetDiscriminant(_14, 1);
_8 = &(*_8);
_14 = Adt78::Variant1 { fld0: (*_8) };
_19.0 = Adt26::Variant1 { fld0: (*_22),fld1: _7 };
SetDiscriminant(_14, 0);
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [(-121_i8),(-37_i8)];
_7 = [(-8837631171516184410869553174982807932_i128),(-97096486968105798793799893923524648883_i128),(-169666740282976658960296821630308275764_i128),132891330835943311078396285679241116741_i128,143797008762578632916637893937946960801_i128,79250762656188432198861040503097664039_i128,(-63504731584736466430100886698439626017_i128),(-8988860341598344242705559426929640305_i128)];
_5 = !_20;
place!(Field::<(u8,)>(Variant(_14, 0), 1)) = (_10,);
_9 = 1887148985625258233_u64 as isize;
_8 = &_1;
Goto(bb7)
}
bb7 = {
_13 = [19137_u16,57622_u16,36690_u16,44904_u16,381_u16,35268_u16,44050_u16,51368_u16];
_28 = !_2;
_22 = &(*_16);
_25 = Field::<(u8,)>(Variant(_14, 0), 1).0 >= _11;
_30 = 690419327_u32 | 251225318_u32;
_24 = _1 as f64;
_22 = &_31;
_27 = (*_8);
_17 = _27 as isize;
_27 = !(*_8);
_1 = _27;
_1 = _27 | _27;
_11 = _28 >> _28;
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [(-44_i8),32_i8];
_17 = (*_16) as isize;
_2 = !_11;
_17 = _5 as isize;
Goto(bb8)
}
bb8 = {
_23 = [_25,_25,_25,_25];
_24 = _11 as f64;
RET = -Field::<f32>(Variant(_19.0, 1), 0);
_24 = 141181373415121293028828478226507672341_i128 as f64;
_21 = [_15,_15,_15,_15,_15,_15,_15];
SetDiscriminant(_19.0, 1);
_34 = (_30,);
_5 = _25;
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [63_i8,(-107_i8)];
RET = _24 as f32;
_28 = !_2;
_15 = '\u{5d07f}';
_24 = (-146146774911692094828795060923920465010_i128) as f64;
_28 = !_11;
_19.0 = Adt26::Variant0 { fld0: _34.0,fld1: _12,fld2: _17,fld3: _2,fld4: (-136467465064801733038099679064004924802_i128) };
_6 = 22156_i16 ^ 4371_i16;
Goto(bb9)
}
bb9 = {
_19.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_19.0, 0), 4)));
place!(Field::<usize>(Variant(_19.0, 0), 1)) = !_12;
_3 = !Field::<usize>(Variant(_19.0, 0), 1);
place!(Field::<u32>(Variant(_19.0, 0), 0)) = _30 ^ _30;
_36 = !4809618876512404271_i64;
_34 = (_30,);
_14 = Adt78::Variant1 { fld0: _1 };
_19.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_19.0, 0), 4)));
_26 = _15;
RET = Field::<u32>(Variant(_19.0, 0), 0) as f32;
_5 = _2 != _2;
SetDiscriminant(_14, 0);
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [(-114_i8),(-61_i8)];
_19.0 = Adt26::Variant1 { fld0: RET,fld1: _7 };
_10 = 62_i8 as u8;
_33 = _24;
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [47_i8,26_i8];
_2 = !_11;
SetDiscriminant(_19.0, 1);
Call(_33 = fn13(Move(_19.1), Move(_16), _2, _2, _23, _23, _2, _13, _1, _23, _27, _28), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_23 = [_25,_5,_5,_25];
_38 = !_5;
_12 = !_3;
_28 = _15 as u8;
_31 = RET;
_35 = (RET,);
place!(Field::<(u8,)>(Variant(_14, 0), 1)) = (_2,);
_33 = _24;
place!(Field::<f32>(Variant(_19.0, 1), 0)) = _31;
_24 = _33;
_34.0 = !_30;
_6 = _12 as i16;
_37 = _17;
_4 = [_3,_3,_3,_12,_12,_3,_12];
_39 = -_9;
place!(Field::<[i8; 2]>(Variant(_14, 0), 0)) = [8_i8,71_i8];
place!(Field::<[i128; 8]>(Variant(_19.0, 1), 1)) = [22283250443236756624049315753424572236_i128,(-91090344322930718753613156377478628123_i128),105283972904242086965947917560898147245_i128,(-88025868701851368252133708369407148652_i128),(-95953831206238794083436124224060985548_i128),(-104618888835298939224412313736550777406_i128),(-149632330332941210238104633232703407414_i128),112591946670349307314000264230454018204_i128];
_22 = &_35.0;
_23 = [_25,_38,_38,_25];
_2 = _11;
_17 = _9 & _9;
_36 = _39 as i64;
_17 = !_37;
_6 = 30964_i16 & (-7235_i16);
_20 = (*_22) <= _35.0;
_11 = !_2;
SetDiscriminant(_19.0, 1);
Goto(bb11)
}
bb11 = {
_28 = _2;
_27 = 104978236857064568144800194896970041058_i128 as i32;
place!(Field::<(u8,)>(Variant(_14, 0), 1)).0 = !_11;
place!(Field::<f32>(Variant(_19.0, 1), 0)) = -RET;
_38 = _5 ^ _5;
_25 = _38 | _38;
RET = Field::<f32>(Variant(_19.0, 1), 0) - _35.0;
_35.0 = 110213976093678191558384023893094337067_u128 as f32;
_10 = !_2;
_25 = _11 != _28;
_4 = [_12,_12,_12,_12,_3,_3,_12];
_40 = [(-64_i8),29_i8,(-25_i8),89_i8,(-55_i8)];
_8 = &_1;
_40 = [36_i8,(-113_i8),(-2_i8),21_i8,63_i8];
_22 = &place!(Field::<f32>(Variant(_19.0, 1), 0));
_10 = (-25_i8) as u8;
_21 = [_15,_15,_26,_26,_15,_26,_15];
_31 = _12 as f32;
_19.0 = Adt26::Variant0 { fld0: _30,fld1: _12,fld2: _17,fld3: Field::<(u8,)>(Variant(_14, 0), 1).0,fld4: (-10375519620237884309290390540310232193_i128) };
place!(Field::<usize>(Variant(_19.0, 0), 1)) = _12 * _3;
_5 = !_25;
Goto(bb12)
}
bb12 = {
place!(Field::<u8>(Variant(_19.0, 0), 3)) = _2 & Field::<(u8,)>(Variant(_14, 0), 1).0;
_24 = _33;
_25 = _38;
_9 = !_17;
place!(Field::<usize>(Variant(_19.0, 0), 1)) = _12 << Field::<u8>(Variant(_19.0, 0), 3);
Goto(bb13)
}
bb13 = {
place!(Field::<usize>(Variant(_19.0, 0), 1)) = !_3;
_33 = _24 * _24;
_9 = -Field::<isize>(Variant(_19.0, 0), 2);
_1 = _27 & _27;
_34 = (Field::<u32>(Variant(_19.0, 0), 0),);
_5 = _25 ^ _25;
_19.0 = Adt26::Variant0 { fld0: _34.0,fld1: _3,fld2: _39,fld3: _28,fld4: (-136560159078757472411608463193929249213_i128) };
_19.0 = Adt26::Variant1 { fld0: RET,fld1: _7 };
_3 = (-69_i8) as usize;
_36 = 4791341814092046146_i64;
_16 = core::ptr::addr_of_mut!(RET);
match _36 {
0 => bb14,
4791341814092046146 => bb16,
_ => bb15
}
}
bb14 = {
_17 = _9;
_19.0 = Adt26::Variant1 { fld0: RET,fld1: _7 };
_13 = [1465_u16,5003_u16,1968_u16,46934_u16,30325_u16,16416_u16,17319_u16,3063_u16];
_11 = _2 ^ _2;
_13 = [30877_u16,18651_u16,49886_u16,37029_u16,44908_u16,39776_u16,55510_u16,16771_u16];
_4 = [_12,_3,_12,_12,_12,_3,_12];
_9 = _11 as isize;
_8 = &_1;
_8 = &(*_8);
_4 = [_3,_3,_12,_3,_3,_3,_12];
place!(Field::<i32>(Variant(_14, 1), 0)) = 67571056476899381502673282828271289755_u128 as i32;
_5 = false;
place!(Field::<f32>(Variant(_19.0, 1), 0)) = RET;
place!(Field::<[i128; 8]>(Variant(_19.0, 1), 1)) = [(-57055049586459820038494368937598653134_i128),(-130928167887008483586088066109702026804_i128),(-10603557676642539576392746956164890312_i128),(-95680881258887673861253637066760424885_i128),(-108849640243395914322823006582517658889_i128),(-148207978329993799534012459386013261883_i128),(-157103461640174286956400404394407097668_i128),48625497090276445222442746994006425228_i128];
_8 = &_1;
_2 = _11 + _11;
_16 = core::ptr::addr_of_mut!(RET);
_21 = [_15,_15,_15,_15,_15,_15,_15];
_22 = &place!(Field::<f32>(Variant(_19.0, 1), 0));
_9 = _15 as isize;
_21 = [_15,_15,_15,_15,_15,_15,_15];
_22 = &(*_16);
RET = _11 as f32;
_14 = Adt78::Variant1 { fld0: (*_8) };
_10 = _2;
_22 = &place!(Field::<f32>(Variant(_19.0, 1), 0));
Goto(bb4)
}
bb15 = {
SetDiscriminant(_19.0, 0);
_4 = [_12,_12,_12,_3,_12,_12,_3];
SetDiscriminant(_14, 1);
_23 = [_5,_5,_20,_5];
_9 = _17;
_10 = _20 as u8;
_24 = 7369984482611591661408961100992876840_i128 as f64;
_22 = &RET;
place!(Field::<usize>(Variant(_19.0, 0), 1)) = _3 << _9;
place!(Field::<isize>(Variant(_19.0, 0), 2)) = _9;
_14 = Adt78::Variant1 { fld0: _1 };
_17 = _9;
_16 = core::ptr::addr_of_mut!((*_22));
_10 = !_2;
_20 = _5;
_19.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_19.0, 0), 4)));
Goto(bb6)
}
bb16 = {
_3 = _12;
_22 = &(*_16);
Goto(bb17)
}
bb17 = {
Call(_43 = dump_var(4_usize, 6_usize, Move(_6), 20_usize, Move(_20), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(4_usize, 23_usize, Move(_23), 36_usize, Move(_36), 11_usize, Move(_11), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_43 = dump_var(4_usize, 9_usize, Move(_9), 37_usize, Move(_37), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_43 = dump_var(4_usize, 7_usize, Move(_7), 38_usize, Move(_38), 44_usize, _44, 44_usize, _44), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: usize,mut _2: i32,mut _3: u8,mut _4: [usize; 7],mut _5: i32,mut _6: i16) -> f32 {
mir! {
type RET = f32;
let _7: i128;
let _8: [bool; 3];
let _9: f64;
let _10: *mut f32;
let _11: char;
let _12: [bool; 3];
let _13: usize;
let _14: usize;
let _15: (bool, f64);
let _16: (u16,);
let _17: isize;
let _18: *mut Adt26;
let _19: isize;
let _20: &'static &'static usize;
let _21: &'static u128;
let _22: ((f32,), &'static f32);
let _23: Adt50;
let _24: Adt42;
let _25: char;
let _26: ((u32,), *mut [usize; 7], [u16; 7]);
let _27: usize;
let _28: isize;
let _29: (u128,);
let _30: f32;
let _31: f64;
let _32: ((u16,), [i128; 1], i8, &'static f32);
let _33: (&'static usize, u8, [char; 7], *const Adt26);
let _34: bool;
let _35: ();
let _36: ();
{
_3 = !27_u8;
_5 = _2 >> _6;
_6 = 32_isize as i16;
RET = (-11_i8) as f32;
_5 = _2 & _2;
RET = 1007244584_u32 as f32;
_1 = 4_usize;
_4[_1] = _1 | _1;
_7 = -128585900808869670726142308676059142755_i128;
_7 = _5 as i128;
_2 = _5;
_4 = [_1,_1,_1,_1,_1,_1,_1];
_1 = !_4[_1];
RET = 35697339906867841801003830155252381806_u128 as f32;
_7 = -150476125085141892814795330709719539575_i128;
RET = 334407948960085590_i64 as f32;
_7 = !146064809202076827820258886781116338667_i128;
_7 = (-64813076881003758052511400460869413400_i128) & (-86484521566156305565139979666369810668_i128);
_3 = !149_u8;
_6 = -26295_i16;
_3 = _1 as u8;
_6 = 24680_i16;
_9 = (-50_i8) as f64;
_8 = [true,true,true];
_4 = [_1,_1,_1,_1,_1,_1,_1];
_7 = 261714618694861796883771342679340319162_u128 as i128;
_5 = -_2;
Goto(bb1)
}
bb1 = {
_4 = [_1,_1,_1,_1,_1,_1,_1];
_6 = (-32410_i16) ^ 31184_i16;
RET = 45412_u16 as f32;
Goto(bb2)
}
bb2 = {
_9 = _6 as f64;
_11 = '\u{b3469}';
_5 = _2;
_4 = [_1,_1,_1,_1,_1,_1,_1];
Goto(bb3)
}
bb3 = {
_3 = 170_u8 & 208_u8;
_5 = _2 * _2;
_3 = 105398723524219800725478114912959124210_u128 as u8;
_12 = [true,true,true];
RET = _3 as f32;
_8 = [true,true,false];
RET = 52500_u16 as f32;
Call(_7 = fn6(_6, _2, _4, _9, _5, _8, _5, _12, _12, _2, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = _9 as i32;
_7 = _9 as i128;
_12 = [true,true,false];
_5 = _2 ^ _2;
_2 = _5;
RET = _9 as f32;
_3 = !155_u8;
_12 = [false,false,true];
_10 = core::ptr::addr_of_mut!(RET);
_12 = _8;
Goto(bb5)
}
bb5 = {
_13 = !_1;
_5 = (*_10) as i32;
_16 = (44656_u16,);
_2 = -_5;
_8 = [true,false,false];
RET = 16464237487470111927511420263576754123_u128 as f32;
_15.0 = false;
_15.1 = _9;
_6 = 9223372036854775807_isize as i16;
RET = _6 as f32;
_11 = '\u{1ab88}';
_11 = '\u{c8cb3}';
Goto(bb6)
}
bb6 = {
_6 = (-4589_i16) << _1;
_12 = [_15.0,_15.0,_15.0];
_14 = _13 | _1;
_14 = _13;
RET = _7 as f32;
_6 = _5 as i16;
_17 = 9223372036854775807_isize;
_2 = 3083547616_u32 as i32;
_15.0 = !true;
_19 = !_17;
_13 = _1;
_12 = _8;
(*_10) = 12744913468841223753_u64 as f32;
RET = 2245931778_u32 as f32;
(*_10) = _14 as f32;
_12 = [_15.0,_15.0,_15.0];
Goto(bb7)
}
bb7 = {
_2 = !_5;
_16.0 = 18404_u16;
_5 = _2 * _2;
_16.0 = 258681847550619191514768289188661068950_u128 as u16;
_12 = [_15.0,_15.0,_15.0];
_10 = core::ptr::addr_of_mut!((*_10));
_8 = [_15.0,_15.0,_15.0];
_3 = !179_u8;
_15 = (true, _9);
_15.1 = _9;
_15.1 = _9 - _9;
_8 = [_15.0,_15.0,_15.0];
_1 = _3 as usize;
_14 = _13 & _1;
_16.0 = !481_u16;
_3 = 9_u8;
_22.0 = ((*_10),);
_5 = _14 as i32;
Goto(bb8)
}
bb8 = {
_2 = _15.0 as i32;
_2 = _5;
_7 = (-87392464692491490201926474507859395254_i128) | 99947677459201860220537434148944284917_i128;
_7 = (-104834890061473452888295339135747848132_i128) | (-123472858719646260514192224379133568273_i128);
RET = _22.0.0 + _22.0.0;
_8 = _12;
_4 = [_13,_14,_13,_1,_14,_14,_1];
RET = _22.0.0 - _22.0.0;
_17 = !_19;
_6 = -27350_i16;
_9 = (-16_i8) as f64;
_16 = (49340_u16,);
_16 = (1638_u16,);
_13 = _19 as usize;
_16 = (9474_u16,);
_19 = -_17;
_16.0 = !31422_u16;
(*_10) = _7 as f32;
_25 = _11;
_22.0.0 = RET;
_11 = _25;
_16 = (19993_u16,);
_26.0.0 = 1639284110_u32;
_22.0.0 = RET - (*_10);
Goto(bb9)
}
bb9 = {
_3 = _2 as u8;
(*_10) = _3 as f32;
RET = _22.0.0 * _22.0.0;
_10 = core::ptr::addr_of_mut!((*_10));
_9 = _22.0.0 as f64;
_26.1 = core::ptr::addr_of_mut!(_4);
_22.1 = &RET;
match _16.0 {
0 => bb7,
1 => bb3,
19993 => bb11,
_ => bb10
}
}
bb10 = {
_2 = _9 as i32;
_7 = _9 as i128;
_12 = [true,true,false];
_5 = _2 ^ _2;
_2 = _5;
RET = _9 as f32;
_3 = !155_u8;
_12 = [false,false,true];
_10 = core::ptr::addr_of_mut!(RET);
_12 = _8;
Goto(bb5)
}
bb11 = {
_22.0 = (RET,);
_6 = -(-2270_i16);
_19 = -_17;
_21 = &_29.0;
_11 = _25;
_15.1 = _9;
_22.1 = &(*_10);
_29.0 = _7 as u128;
_26.0.0 = 1803166343_u32;
_29.0 = 175177404441227747742615093228991207935_u128 | 7433486647701748445137017141516488715_u128;
_26.2 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_15 = (true, _9);
_22.0.0 = (*_10) + (*_10);
_28 = _17 + _19;
_1 = _14 + _14;
Goto(bb12)
}
bb12 = {
_4 = [_1,_14,_14,_1,_14,_1,_1];
_29 = (25178480445646423791774177964062047614_u128,);
_15.1 = _9;
_28 = !_19;
_12 = _8;
_15.1 = _6 as f64;
_28 = !_17;
_16.0 = _26.0.0 as u16;
_28 = _26.0.0 as isize;
_1 = _13;
_1 = _26.0.0 as usize;
_6 = 15843_i16;
_8 = [_15.0,_15.0,_15.0];
_15.0 = !true;
_30 = RET;
_32.0 = _16;
_12 = _8;
_5 = _2 - _2;
_20 = &_33.0;
RET = -_22.0.0;
_17 = _19;
_27 = _13 - _13;
_33.0 = &_14;
_16 = (_32.0.0,);
_32.2 = !(-64_i8);
_16 = _32.0;
Goto(bb13)
}
bb13 = {
Call(_35 = dump_var(5_usize, 19_usize, Move(_19), 17_usize, Move(_17), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_35 = dump_var(5_usize, 29_usize, Move(_29), 11_usize, Move(_11), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_35 = dump_var(5_usize, 25_usize, Move(_25), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i16,mut _2: i32,mut _3: [usize; 7],mut _4: f64,mut _5: i32,mut _6: [bool; 3],mut _7: i32,mut _8: [bool; 3],mut _9: [bool; 3],mut _10: i32,mut _11: [bool; 3]) -> i128 {
mir! {
type RET = i128;
let _12: &'static f32;
let _13: isize;
let _14: f64;
let _15: *mut f32;
let _16: ([u16; 7], usize);
let _17: bool;
let _18: &'static (bool, f64);
let _19: u128;
let _20: (Adt26, *mut i128);
let _21: isize;
let _22: isize;
let _23: f64;
let _24: f64;
let _25: Adt69;
let _26: isize;
let _27: ((f32,), &'static f32);
let _28: ([u16; 7], usize);
let _29: ();
let _30: ();
{
_10 = !_5;
_8 = [true,false,true];
_6 = _11;
RET = 141142448369458028514350004758792499776_i128;
_1 = 26098_i16;
_7 = _10;
_11 = _6;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
141142448369458028514350004758792499776 => bb9,
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
_8 = [false,false,false];
RET = 7_usize as i128;
_11 = _9;
_5 = 18431026543102930787_u64 as i32;
_5 = -_2;
RET = (-82454877505726746904002321031172908792_i128) | (-130499943724927579829947807019425942766_i128);
_8 = _6;
_7 = _10;
Call(_1 = fn7(_7, _9, _9, _2, _4, _7, _7, _7, RET), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = -_7;
_8 = [false,false,true];
_16.1 = !18214340160021221270_usize;
Call(_7 = fn8(_1, _5, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = _2 >> _10;
_16.1 = !6_usize;
_2 = _10;
_13 = (-9223372036854775808_isize) >> _7;
_3 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_1 = _4 as i16;
_17 = true;
_2 = !_7;
_8 = [_17,_17,_17];
_10 = _2 * _2;
_4 = _10 as f64;
_13 = _1 as isize;
_6 = [_17,_17,_17];
_3 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_1 = (-11733_i16) ^ 25958_i16;
_23 = 4164570514_u32 as f64;
_7 = _2 ^ _10;
_1 = 143_u8 as i16;
_16.0 = [24411_u16,33414_u16,33936_u16,19289_u16,53489_u16,53773_u16,64191_u16];
_2 = _10 >> _10;
_14 = -_4;
_3 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_22 = _13;
_4 = _14;
_22 = _13 ^ _13;
Goto(bb12)
}
bb12 = {
_25.fld2 = (_17, _14);
_20.1 = core::ptr::addr_of_mut!(RET);
_19 = _22 as u128;
_11 = _9;
_16.1 = 1271110766105482407_u64 as usize;
_3 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_25.fld1 = [1323680821_u32];
_20.0 = Adt26::Variant0 { fld0: 3783912932_u32,fld1: _16.1,fld2: _13,fld3: 217_u8,fld4: RET };
_3 = [Field::<usize>(Variant(_20.0, 0), 1),Field::<usize>(Variant(_20.0, 0), 1),_16.1,Field::<usize>(Variant(_20.0, 0), 1),Field::<usize>(Variant(_20.0, 0), 1),_16.1,Field::<usize>(Variant(_20.0, 0), 1)];
_21 = _17 as isize;
_22 = Field::<isize>(Variant(_20.0, 0), 2) * Field::<isize>(Variant(_20.0, 0), 2);
_18 = &_25.fld2;
Goto(bb13)
}
bb13 = {
_15 = core::ptr::addr_of_mut!(_27.0.0);
(*_15) = (*_18).1 as f32;
_24 = -_14;
_1 = (-16196_i16) | 32023_i16;
place!(Field::<u8>(Variant(_20.0, 0), 3)) = !125_u8;
Call(_6 = fn12(Move(_18), _22, _24, (*_18).1, Move(_15), _22, _27.0.0, (*_15), _27.0, _25.fld2.1, _10, _2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_12 = &_27.0.0;
_25.fld0 = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant(_20.0, 0), 4)) = !RET;
_18 = &_25.fld2;
_25.fld0 = core::ptr::addr_of_mut!(_3);
place!(Field::<isize>(Variant(_20.0, 0), 2)) = _22 & _22;
_27.0.0 = 12820_u16 as f32;
_27.1 = &_27.0.0;
_12 = &_27.0.0;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(6_usize, 8_usize, Move(_8), 16_usize, Move(_16), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(6_usize, 22_usize, Move(_22), 5_usize, Move(_5), 17_usize, Move(_17), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i32,mut _2: [bool; 3],mut _3: [bool; 3],mut _4: i32,mut _5: f64,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: i128) -> i16 {
mir! {
type RET = i16;
let _10: Adt50;
let _11: &'static ((f32,), &'static f32);
let _12: [u32; 1];
let _13: &'static *mut Adt26;
let _14: ();
let _15: ();
{
_5 = 9223372036854775807_isize as f64;
_8 = _4;
_5 = 126_i8 as f64;
_4 = 168331241102666838772225204817949678040_u128 as i32;
RET = -1670_i16;
_7 = _1 + _6;
_8 = _7;
RET = !(-16829_i16);
_8 = 24483_u16 as i32;
RET = -(-16057_i16);
_3 = [false,false,false];
_3 = [true,true,true];
_9 = 48693067399526543802648128670840588887_i128;
_4 = !_1;
_7 = _4;
_8 = -_1;
_6 = -_4;
_12 = [2855164353_u32];
_4 = _8;
_3 = [true,false,true];
RET = (-9983_i16) >> _6;
_7 = 18122759873722836239_u64 as i32;
_12 = [1333524241_u32];
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(7_usize, 3_usize, Move(_3), 1_usize, Move(_1), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i16,mut _2: i32,mut _3: i32) -> i32 {
mir! {
type RET = i32;
let _4: f32;
let _5: (f32,);
let _6: isize;
let _7: isize;
let _8: Adt69;
let _9: (f32,);
let _10: [u32; 1];
let _11: &'static usize;
let _12: (u128,);
let _13: Adt26;
let _14: [u32; 1];
let _15: isize;
let _16: u8;
let _17: bool;
let _18: i32;
let _19: ();
let _20: ();
{
_3 = !_2;
RET = _3 << _1;
RET = _3;
_3 = !RET;
RET = 82_i8 as i32;
RET = _2;
Call(RET = fn9(_2, _1, _1, _1, _3, _3, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 42670_u16 as i32;
_2 = (-4529909580298940762_i64) as i32;
RET = _3;
_3 = 160671902888410399962440374293973234789_i128 as i32;
Goto(bb2)
}
bb2 = {
_6 = (-17_isize) * (-9223372036854775808_isize);
_4 = RET as f32;
RET = (-16_i8) as i32;
_2 = (-7900612074369116753_i64) as i32;
Call(_5.0 = fn10(_1, _1, _1, _6, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _3;
RET = 2240556667599797089_i64 as i32;
_1 = 23230_i16 >> _6;
_1 = false as i16;
_2 = 171179319524735313320959622544248350004_u128 as i32;
RET = _3 * _2;
_4 = _5.0;
_5 = (_4,);
_3 = 112411863819217591754633964352685595172_u128 as i32;
_1 = 21792_i16 * 18554_i16;
_4 = -_5.0;
_3 = RET | _2;
_8.fld2.0 = _6 <= _6;
_6 = 7_isize + 109_isize;
_1 = 2129251996275994862_u64 as i16;
_9.0 = 2282926111_u32 as f32;
_5.0 = _6 as f32;
_5.0 = _4;
Goto(bb4)
}
bb4 = {
_4 = -_9.0;
_8.fld2.1 = 94_u8 as f64;
_4 = 201472411312565374200921421519964116380_u128 as f32;
_5 = _9;
Goto(bb5)
}
bb5 = {
_5 = (_4,);
_12.0 = 80147032169144680771742380693058995540_u128 << _3;
_12 = (173526565897371542802749927362155911967_u128,);
_2 = _3;
_10 = [1801020018_u32];
_8.fld2.0 = false;
_9.0 = _4;
_15 = _6 - _6;
_7 = _6;
RET = _3;
_14 = _10;
_9.0 = _5.0;
_5 = (_9.0,);
_4 = _9.0 - _5.0;
_18 = RET - _3;
_9 = (_4,);
_17 = _8.fld2.0;
_12.0 = 92866948632405704949931637327647682195_u128 + 172891273086110545715457192185221067390_u128;
_7 = (-65681750021534372155495243792031763162_i128) as isize;
_3 = _2;
_8.fld1 = [3309037519_u32];
Call(RET = fn11(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = 13266_u16 as u8;
_12.0 = 30200646193478684053890781805241401948_u128;
Goto(bb7)
}
bb7 = {
Call(_19 = dump_var(8_usize, 15_usize, Move(_15), 17_usize, Move(_17), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_19 = dump_var(8_usize, 3_usize, Move(_3), 2_usize, Move(_2), 20_usize, _20, 20_usize, _20), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i32,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i32,mut _6: i32,mut _7: i16,mut _8: i32) -> i32 {
mir! {
type RET = i32;
let _9: u128;
let _10: (u128,);
let _11: &'static usize;
let _12: u16;
let _13: (i128, &'static i16, &'static *mut i128, (u32,));
let _14: f32;
let _15: &'static [char; 7];
let _16: [bool; 4];
let _17: [u16; 7];
let _18: i64;
let _19: i64;
let _20: (bool, f64);
let _21: [u16; 7];
let _22: *mut Adt26;
let _23: ();
let _24: ();
{
_8 = _5;
RET = 59153_u16 as i32;
RET = _8;
_9 = !303136522402889989317386788447351644041_u128;
_9 = 142038318502103158322348576642095413217_u128;
RET = _6;
_4 = _2 & _7;
_3 = 16462343610351143385_usize as i16;
_8 = -_5;
_8 = RET;
Call(_7 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 3000394055_u32 as i32;
_10 = (_9,);
_9 = _2 as u128;
RET = _5 & _8;
_2 = _7;
Goto(bb2)
}
bb2 = {
_7 = _2;
_5 = RET;
_3 = 12197985262722370259_u64 as i16;
_12 = 8301029582125971998_i64 as u16;
_13.1 = &_2;
_2 = -_7;
_12 = 4631298851362926491_usize as u16;
_6 = _1;
_13.3 = (1196845031_u32,);
_10.0 = _9;
_13.3.0 = 3863189130_u32;
_9 = 111_u8 as u128;
_7 = _2 >> _2;
_13.1 = &_2;
_16 = [false,false,false,false];
_7 = '\u{a5956}' as i16;
_7 = 7_u8 as i16;
_4 = 22_u8 as i16;
_9 = !_10.0;
_17 = [_12,_12,_12,_12,_12,_12,_12];
_14 = _12 as f32;
Goto(bb3)
}
bb3 = {
_12 = !41470_u16;
_1 = _6;
_4 = -_2;
_2 = 5_usize as i16;
_14 = _12 as f32;
_1 = -_5;
_13.3.0 = 2104602836_u32 + 2390497533_u32;
_18 = _4 as i64;
_13.0 = -128624730584508372197126380171642378889_i128;
_14 = (-96_i8) as f32;
_7 = _4;
_13.3 = (2687569771_u32,);
_14 = 4582590513385080201_u64 as f32;
_9 = _10.0;
_13.3 = (2900727179_u32,);
RET = (-55_i8) as i32;
_10.0 = _9;
_13.3.0 = 3324875633_u32;
RET = !_5;
_16 = [true,false,false,false];
_20.1 = RET as f64;
_19 = 9223372036854775807_isize as i64;
RET = _8;
_20.0 = false;
_21 = [_12,_12,_12,_12,_12,_12,_12];
RET = _5 << _10.0;
_13.3 = (545868859_u32,);
_5 = _1 + _1;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(9_usize, 18_usize, Move(_18), 19_usize, Move(_19), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(9_usize, 16_usize, Move(_16), 10_usize, Move(_10), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: isize,mut _5: i16,mut _6: i16) -> f32 {
mir! {
type RET = f32;
let _7: char;
let _8: char;
let _9: u128;
let _10: f64;
let _11: char;
let _12: [i128; 1];
let _13: &'static i8;
let _14: bool;
let _15: Adt78;
let _16: char;
let _17: Adt42;
let _18: [i8; 2];
let _19: isize;
let _20: *mut (bool, f64);
let _21: isize;
let _22: *mut Adt26;
let _23: (&'static usize, u8, [char; 7], *const Adt26);
let _24: &'static i16;
let _25: f64;
let _26: (u128,);
let _27: f64;
let _28: i16;
let _29: i32;
let _30: [i128; 8];
let _31: i128;
let _32: u128;
let _33: [i128; 8];
let _34: *mut u128;
let _35: *const Adt34;
let _36: f64;
let _37: [usize; 7];
let _38: ();
let _39: ();
{
RET = _2 as f32;
_7 = '\u{6637c}';
_3 = 1298576467_i32 as i16;
_4 = 107_isize - (-9223372036854775808_isize);
_6 = !_1;
RET = 221926271753393947747692381659049185481_u128 as f32;
_5 = 25210_u16 as i16;
_2 = _6;
_6 = -_2;
_5 = 327440302703514626608417870448175804050_u128 as i16;
_7 = '\u{82abc}';
_7 = '\u{29fcb}';
_1 = !_2;
_6 = _2;
RET = 4062338838_u32 as f32;
RET = 710891572_u32 as f32;
_7 = '\u{2c574}';
_3 = 316242436973826254086033599355938625932_u128 as i16;
_7 = '\u{dafc2}';
_7 = '\u{c2847}';
_6 = !_2;
Call(_5 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = -_2;
_4 = (-122_isize);
_10 = _4 as f64;
RET = _2 as f32;
RET = 2018590286542285621_u64 as f32;
_10 = 14072317640905632665_u64 as f64;
_6 = _5;
_8 = _7;
_9 = 102510894946422530266559939609134400031_u128;
_11 = _7;
_2 = 7679673832125286976_i64 as i16;
RET = 2092731064504555388_u64 as f32;
_7 = _11;
_4 = _11 as isize;
_4 = !(-35_isize);
_9 = true as u128;
RET = _4 as f32;
_8 = _7;
_8 = _7;
_11 = _7;
_12 = [46394534692837020642936370601869294214_i128];
_11 = _8;
_6 = _1;
_1 = 4579234616875237205_u64 as i16;
_10 = 611711690_u32 as f64;
Goto(bb2)
}
bb2 = {
_4 = !4_isize;
_14 = !true;
_8 = _7;
_11 = _8;
_2 = _3;
_4 = 58664_u16 as isize;
_12 = [113938140844820370424874854717011256186_i128];
_3 = !_5;
_3 = _2 * _6;
_2 = 50202_u16 as i16;
_12 = [46439896026669509450728188293368231878_i128];
RET = 1692674952_u32 as f32;
_10 = RET as f64;
_7 = _8;
_11 = _8;
_15 = Adt78::Variant1 { fld0: (-1118610033_i32) };
_3 = _6;
_10 = _4 as f64;
_16 = _8;
_5 = _6;
_7 = _16;
_8 = _7;
_14 = _8 <= _8;
place!(Field::<i32>(Variant(_15, 1), 0)) = !(-1448726159_i32);
place!(Field::<i32>(Variant(_15, 1), 0)) = -258547685_i32;
_4 = _16 as isize;
Goto(bb3)
}
bb3 = {
RET = _9 as f32;
SetDiscriminant(_15, 0);
place!(Field::<[i8; 2]>(Variant(_15, 0), 0)) = [(-49_i8),14_i8];
_16 = _8;
_3 = _6 | _6;
place!(Field::<(u8,)>(Variant(_15, 0), 1)).0 = 27_u8 + 148_u8;
_5 = !_3;
place!(Field::<(u8,)>(Variant(_15, 0), 1)) = (163_u8,);
RET = 169398293069525150896109181987057337270_i128 as f32;
_9 = !25166465933805223390583666055492768997_u128;
_18 = [(-103_i8),(-32_i8)];
_6 = _3;
SetDiscriminant(_15, 1);
_8 = _7;
_11 = _8;
Goto(bb4)
}
bb4 = {
_5 = _6 ^ _6;
_2 = _5 * _6;
RET = _10 as f32;
_11 = _7;
_3 = -_2;
_11 = _16;
_4 = 15443099821637442452_u64 as isize;
_9 = 170536379100540560310938846515305798232_u128;
_21 = -_4;
_7 = _8;
_11 = _8;
_5 = -_3;
_14 = !true;
_5 = 3_usize as i16;
RET = 2546250539955661881_i64 as f32;
_16 = _7;
_7 = _8;
_5 = _2 - _3;
_19 = _21 & _4;
Goto(bb5)
}
bb5 = {
_9 = 277277362868992316441723789145788545008_u128;
_23.1 = 2623264956_u32 as u8;
_2 = -_3;
_14 = _3 <= _3;
_23.2 = [_8,_7,_7,_16,_11,_7,_7];
_10 = 19580192881991416010268136405047636161_i128 as f64;
_23.2 = [_8,_7,_11,_11,_8,_7,_11];
_6 = !_3;
_10 = 2948254211_u32 as f64;
_14 = true;
_24 = &_2;
Goto(bb6)
}
bb6 = {
_12 = [73866563638358717858864672018517897157_i128];
_21 = _19;
place!(Field::<i32>(Variant(_15, 1), 0)) = (-1663548817_i32) | 1293513347_i32;
SetDiscriminant(_15, 1);
_1 = 65853837001129721358878870631313493238_i128 as i16;
_23.1 = 171_u8;
_18 = [112_i8,10_i8];
place!(Field::<i32>(Variant(_15, 1), 0)) = _3 as i32;
_3 = Field::<i32>(Variant(_15, 1), 0) as i16;
_16 = _11;
Goto(bb7)
}
bb7 = {
_23.2 = [_16,_7,_16,_8,_16,_7,_7];
_25 = _9 as f64;
_18 = [(-28_i8),(-1_i8)];
_7 = _16;
_10 = _25 - _25;
_23.1 = 21_u8 * 172_u8;
_8 = _11;
_23.2 = [_16,_7,_11,_7,_7,_7,_7];
_5 = _6;
place!(Field::<i32>(Variant(_15, 1), 0)) = (-1586552730_i32);
_23.1 = 4_u8;
_11 = _7;
_27 = RET as f64;
_7 = _11;
_28 = _2;
_28 = !_3;
_7 = _16;
_14 = false;
_12 = [153624008780402424917900143036108462795_i128];
_25 = _27 + _10;
Goto(bb8)
}
bb8 = {
_12 = [64065322162523831977006468836265183043_i128];
_23.2 = [_16,_8,_8,_16,_11,_16,_7];
_21 = 8479204120778011772_u64 as isize;
SetDiscriminant(_15, 1);
_19 = _14 as isize;
_29 = 480790229_i32 * 810466927_i32;
_12 = [128346738305922655490190112654026924770_i128];
_29 = (-852048500714635112_i64) as i32;
_4 = _21;
_21 = _19;
_29 = _9 as i32;
_23.1 = !214_u8;
_14 = !true;
_15 = Adt78::Variant1 { fld0: _29 };
_2 = 14874386048167062821_u64 as i16;
RET = _29 as f32;
_7 = _16;
_11 = _8;
_5 = _28 + _28;
_23.1 = !203_u8;
_31 = (-71418425944519511684389913080111527043_i128);
match _31 {
0 => bb3,
1 => bb2,
268863940976418951778984694351656684413 => bb10,
_ => bb9
}
}
bb9 = {
_5 = _6 ^ _6;
_2 = _5 * _6;
RET = _10 as f32;
_11 = _7;
_3 = -_2;
_11 = _16;
_4 = 15443099821637442452_u64 as isize;
_9 = 170536379100540560310938846515305798232_u128;
_21 = -_4;
_7 = _8;
_11 = _8;
_5 = -_3;
_14 = !true;
_5 = 3_usize as i16;
RET = 2546250539955661881_i64 as f32;
_16 = _7;
_7 = _8;
_5 = _2 - _3;
_19 = _21 & _4;
Goto(bb5)
}
bb10 = {
_15 = Adt78::Variant1 { fld0: _29 };
_3 = _5;
_28 = _5 * _2;
_15 = Adt78::Variant1 { fld0: _29 };
_26.0 = !_9;
_30 = [_31,_31,_31,_31,_31,_31,_31,_31];
_16 = _8;
_11 = _8;
_26 = (_9,);
_31 = 55982285634683006616777685077960155856_i128;
_15 = Adt78::Variant1 { fld0: _29 };
_24 = &_2;
RET = Field::<i32>(Variant(_15, 1), 0) as f32;
Goto(bb11)
}
bb11 = {
_28 = _31 as i16;
_11 = _16;
place!(Field::<i32>(Variant(_15, 1), 0)) = _4 as i32;
_1 = _6;
_18 = [(-125_i8),(-18_i8)];
_19 = 15606635255406963758_u64 as isize;
_30 = [_31,_31,_31,_31,_31,_31,_31,_31];
_29 = -Field::<i32>(Variant(_15, 1), 0);
_19 = _21 - _4;
_2 = -_5;
_10 = -_27;
_34 = core::ptr::addr_of_mut!(_26.0);
_36 = _10;
match (*_34) {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb12,
4 => bb13,
277277362868992316441723789145788545008 => bb15,
_ => bb14
}
}
bb12 = {
_15 = Adt78::Variant1 { fld0: _29 };
_3 = _5;
_28 = _5 * _2;
_15 = Adt78::Variant1 { fld0: _29 };
_26.0 = !_9;
_30 = [_31,_31,_31,_31,_31,_31,_31,_31];
_16 = _8;
_11 = _8;
_26 = (_9,);
_31 = 55982285634683006616777685077960155856_i128;
_15 = Adt78::Variant1 { fld0: _29 };
_24 = &_2;
RET = Field::<i32>(Variant(_15, 1), 0) as f32;
Goto(bb11)
}
bb13 = {
_12 = [73866563638358717858864672018517897157_i128];
_21 = _19;
place!(Field::<i32>(Variant(_15, 1), 0)) = (-1663548817_i32) | 1293513347_i32;
SetDiscriminant(_15, 1);
_1 = 65853837001129721358878870631313493238_i128 as i16;
_23.1 = 171_u8;
_18 = [112_i8,10_i8];
place!(Field::<i32>(Variant(_15, 1), 0)) = _3 as i32;
_3 = Field::<i32>(Variant(_15, 1), 0) as i16;
_16 = _11;
Goto(bb7)
}
bb14 = {
_23.2 = [_16,_7,_16,_8,_16,_7,_7];
_25 = _9 as f64;
_18 = [(-28_i8),(-1_i8)];
_7 = _16;
_10 = _25 - _25;
_23.1 = 21_u8 * 172_u8;
_8 = _11;
_23.2 = [_16,_7,_11,_7,_7,_7,_7];
_5 = _6;
place!(Field::<i32>(Variant(_15, 1), 0)) = (-1586552730_i32);
_23.1 = 4_u8;
_11 = _7;
_27 = RET as f64;
_7 = _11;
_28 = _2;
_28 = !_3;
_7 = _16;
_14 = false;
_12 = [153624008780402424917900143036108462795_i128];
_25 = _27 + _10;
Goto(bb8)
}
bb15 = {
_31 = (-87066815490798464284184111364846848311_i128);
_23.2 = [_11,_16,_8,_16,_7,_8,_7];
_25 = -_10;
_23.2 = [_16,_7,_7,_11,_8,_16,_7];
_29 = -Field::<i32>(Variant(_15, 1), 0);
_5 = _2;
SetDiscriminant(_15, 0);
place!(Field::<(u8,)>(Variant(_15, 0), 1)) = (_23.1,);
_2 = !_5;
(*_34) = _9 % _9;
_24 = &_2;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(10_usize, 30_usize, Move(_30), 3_usize, Move(_3), 31_usize, Move(_31), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(10_usize, 11_usize, Move(_11), 26_usize, Move(_26), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(10_usize, 16_usize, Move(_16), 8_usize, Move(_8), 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i32) -> i32 {
mir! {
type RET = i32;
let _2: isize;
let _3: &'static Adt34;
let _4: Adt76;
let _5: isize;
let _6: Adt34;
let _7: isize;
let _8: isize;
let _9: char;
let _10: (i128, &'static i16, &'static *mut i128, (u32,));
let _11: [i8; 5];
let _12: f64;
let _13: *mut f64;
let _14: ([u16; 7], usize);
let _15: ();
let _16: ();
{
RET = (-84_isize) as i32;
RET = _1 - _1;
RET = 6_usize as i32;
RET = 16027516677907647660_u64 as i32;
RET = !_1;
RET = -_1;
_1 = RET & RET;
_1 = RET >> RET;
_1 = !RET;
RET = _1 ^ _1;
RET = _1;
_1 = RET;
RET = _1;
RET = _1 & _1;
_1 = RET;
RET = !_1;
RET = _1;
RET = _1;
RET = -_1;
_1 = -RET;
RET = -_1;
RET = (-64_i8) as i32;
_2 = !(-34_isize);
Goto(bb1)
}
bb1 = {
_1 = (-169411323090521998184510858643475283500_i128) as i32;
_2 = (-8659386195578459467_i64) as isize;
RET = _1;
_1 = '\u{f7591}' as i32;
RET = _1 ^ _1;
_1 = RET;
RET = !_1;
RET = _1 + _1;
_2 = 12_u8 as isize;
_4.fld1.fld5 = [14545893040764369286_usize,3294275389853626071_usize,6_usize,2_usize,5827272200814351991_usize,4_usize,2_usize];
_4.fld1.fld2 = (233_u8,);
_4.fld1.fld5 = [0_usize,1_usize,5_usize,1613036714604051129_usize,445037861839418009_usize,3_usize,7758762566414855429_usize];
_1 = RET << RET;
_1 = RET >> RET;
_4.fld1.fld5 = [1_usize,12078025112217617460_usize,7275300553044186984_usize,3_usize,4_usize,12751017678409795663_usize,8629738246062258423_usize];
RET = !_1;
_1 = RET & RET;
_4.fld1.fld3 = (62651_u16,);
_4.fld2 = [94_i8,31_i8,72_i8,61_i8,89_i8];
_4.fld1.fld4.0 = Adt26::Variant0 { fld0: 3656699590_u32,fld1: 7903018883425877487_usize,fld2: _2,fld3: _4.fld1.fld2.0,fld4: 162153453400787527243769370137987508174_i128 };
_2 = !Field::<isize>(Variant(_4.fld1.fld4.0, 0), 2);
place!(Field::<u32>(Variant(_4.fld1.fld4.0, 0), 0)) = !2227488584_u32;
_1 = RET;
_5 = Field::<isize>(Variant(_4.fld1.fld4.0, 0), 2);
place!(Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1)) = !6_usize;
match _4.fld1.fld3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
62651 => bb8,
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
_4.fld1.fld1 = _4.fld1.fld3.0;
_6.fld2 = [Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_4.fld1.fld4.0, 0), 1)];
_4.fld1.fld3 = (_4.fld1.fld1,);
_6.fld3 = 76_i8;
RET = true as i32;
RET = _1 ^ _1;
place!(Field::<u32>(Variant(_4.fld1.fld4.0, 0), 0)) = 2400406099_u32;
place!(Field::<i128>(Variant(_4.fld1.fld4.0, 0), 4)) = true as i128;
_4.fld1.fld5 = _6.fld2;
_4.fld1.fld3 = (_4.fld1.fld1,);
_4.fld1.fld1 = _4.fld1.fld3.0 / _4.fld1.fld3.0;
_4.fld2 = [_6.fld3,_6.fld3,_6.fld3,_6.fld3,_6.fld3];
_4.fld2 = [_6.fld3,_6.fld3,_6.fld3,_6.fld3,_6.fld3];
_1 = -RET;
_6.fld1 = core::ptr::addr_of_mut!(_6.fld4);
_6.fld3 = 42_i8;
Goto(bb9)
}
bb9 = {
Call(_15 = dump_var(11_usize, 5_usize, Move(_5), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: &'static (bool, f64),mut _2: isize,mut _3: f64,mut _4: f64,mut _5: *mut f32,mut _6: isize,mut _7: f32,mut _8: f32,mut _9: (f32,),mut _10: f64,mut _11: i32,mut _12: i32) -> [bool; 3] {
mir! {
type RET = [bool; 3];
let _13: ([u16; 7], usize);
let _14: isize;
let _15: &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _16: f32;
let _17: f32;
let _18: [bool; 3];
let _19: isize;
let _20: Adt69;
let _21: *mut [usize; 7];
let _22: ((u16,), [i128; 1], i8, &'static f32);
let _23: *mut (bool, f64);
let _24: (u8,);
let _25: char;
let _26: &'static *mut i128;
let _27: ([u16; 7], usize);
let _28: ();
let _29: ();
{
_4 = -_3;
RET = [false,false,true];
_3 = 157125943904067784659349056070025822604_i128 as f64;
_8 = _7 + _9.0;
RET = [false,true,true];
_7 = -_8;
_2 = _6 ^ _6;
_2 = 26_u8 as isize;
_3 = _10 + _10;
_12 = _11;
_13.0 = [55948_u16,36965_u16,23337_u16,41761_u16,10387_u16,286_u16,12210_u16];
_13.0 = [45996_u16,59448_u16,1990_u16,40743_u16,30851_u16,2703_u16,61250_u16];
RET = [false,false,true];
Goto(bb1)
}
bb1 = {
_8 = _3 as f32;
RET = [true,true,false];
_13.0 = [7661_u16,4624_u16,39309_u16,63418_u16,62521_u16,7071_u16,25238_u16];
_11 = _12 + _12;
_8 = -_9.0;
_13.0 = [26327_u16,28568_u16,36266_u16,33948_u16,24356_u16,16453_u16,58349_u16];
RET = [true,true,true];
_14 = (-22279_i16) as isize;
_4 = _3;
_11 = !_12;
_13.1 = '\u{abfe8}' as usize;
_12 = _3 as i32;
_9 = (_7,);
_8 = -_9.0;
_13.1 = 0_usize;
_13.0 = [25854_u16,21573_u16,22077_u16,679_u16,43156_u16,8862_u16,16713_u16];
_14 = -_6;
match _13.1 {
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
0 => bb10,
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
_12 = 1593916537_u32 as i32;
_3 = _10;
_2 = 3332254717_u32 as isize;
_13.1 = 3918915994734343913_usize ^ 7533791297496198369_usize;
_6 = _14 << _11;
_7 = _8;
_13.0 = [37348_u16,27364_u16,32227_u16,51539_u16,37842_u16,62598_u16,24242_u16];
_14 = 39446_u16 as isize;
_3 = _10 + _10;
_9.0 = _8 + _7;
RET = [true,true,false];
_16 = _9.0;
_9.0 = _8;
_10 = _4 * _4;
_17 = _16 * _7;
_9.0 = -_17;
_14 = _6;
_1 = &_20.fld2;
_9.0 = _8 - _17;
_6 = _14 * _14;
_19 = -_14;
_9.0 = -_7;
RET = [false,false,false];
Goto(bb11)
}
bb11 = {
_7 = -_9.0;
_10 = -_4;
_8 = -_9.0;
_6 = _19;
_3 = _10;
_22.0.0 = _13.1 as u16;
_24.0 = 128_u8 >> _6;
_4 = 160132360906029730875333437599079963593_u128 as f64;
_12 = _24.0 as i32;
_22.2 = -68_i8;
_22.0 = (26960_u16,);
_16 = _7;
_8 = _12 as f32;
_14 = _19 - _19;
Goto(bb12)
}
bb12 = {
_22.1 = [(-104864717418864907599062056653589378625_i128)];
_10 = _3 - _3;
_20.fld1 = [1825211278_u32];
_3 = _10;
_2 = -_14;
_20.fld2 = (false, _3);
_23 = core::ptr::addr_of_mut!(_20.fld2);
RET = [(*_23).0,_20.fld2.0,_20.fld2.0];
_22.3 = &_8;
_20.fld2.0 = _20.fld2.1 < _10;
_7 = _8 * _16;
_9 = (_16,);
_22.0.0 = 35825_u16;
_22.3 = &_8;
Goto(bb13)
}
bb13 = {
Call(_28 = dump_var(12_usize, 6_usize, Move(_6), 14_usize, Move(_14), 13_usize, Move(_13), 24_usize, Move(_24)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut i128,mut _2: *mut f32,mut _3: u8,mut _4: u8,mut _5: [bool; 4],mut _6: [bool; 4],mut _7: u8,mut _8: [u16; 8],mut _9: i32,mut _10: [bool; 4],mut _11: i32,mut _12: u8) -> f64 {
mir! {
type RET = f64;
let _13: i64;
let _14: isize;
let _15: isize;
let _16: i64;
let _17: isize;
let _18: *mut f32;
let _19: &'static u128;
let _20: (u128,);
let _21: *mut i128;
let _22: &'static *mut Adt26;
let _23: [bool; 3];
let _24: ();
let _25: ();
{
_8 = [30273_u16,60805_u16,28383_u16,29811_u16,17037_u16,11903_u16,14136_u16,39356_u16];
_9 = 135634417_u32 as i32;
_13 = 8010537518716849560_i64;
Goto(bb1)
}
bb1 = {
RET = 8166811843364345052_u64 as f64;
_5 = [true,false,true,false];
RET = (-94983920174583246978673347619072210817_i128) as f64;
_10 = [false,true,false,true];
_7 = !_3;
RET = _9 as f64;
_4 = _7 >> _7;
_7 = !_12;
_9 = 1666863121_u32 as i32;
RET = (-9223372036854775808_isize) as f64;
_7 = _3;
_8 = [15503_u16,48590_u16,60658_u16,51483_u16,44569_u16,53140_u16,61274_u16,10657_u16];
_8 = [1412_u16,42697_u16,43598_u16,36626_u16,50014_u16,59927_u16,62963_u16,30380_u16];
_4 = 33623_u16 as u8;
_11 = '\u{92bcb}' as i32;
_14 = (-9223372036854775808_isize) * 12_isize;
_3 = !_7;
_13 = 5834835958191864196_i64 | 5480446038181496437_i64;
_8 = [57176_u16,46560_u16,33793_u16,16019_u16,10265_u16,55270_u16,60840_u16,27210_u16];
_5 = [false,false,true,false];
Call(_14 = fn14(Move(_1), _8, _7, _6, _7, _6, _12, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = _7 | _7;
_7 = _12;
_12 = !_3;
_15 = _3 as isize;
_15 = _14 - _14;
_5 = [true,false,false,false];
_10 = _6;
Goto(bb3)
}
bb3 = {
_13 = 7959609349601916646_i64;
_16 = _13 ^ _13;
_3 = _7;
match _13 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
7959609349601916646 => bb10,
_ => bb9
}
}
bb4 = {
_12 = _7 | _7;
_7 = _12;
_12 = !_3;
_15 = _3 as isize;
_15 = _14 - _14;
_5 = [true,false,false,false];
_10 = _6;
Goto(bb3)
}
bb5 = {
RET = 8166811843364345052_u64 as f64;
_5 = [true,false,true,false];
RET = (-94983920174583246978673347619072210817_i128) as f64;
_10 = [false,true,false,true];
_7 = !_3;
RET = _9 as f64;
_4 = _7 >> _7;
_7 = !_12;
_9 = 1666863121_u32 as i32;
RET = (-9223372036854775808_isize) as f64;
_7 = _3;
_8 = [15503_u16,48590_u16,60658_u16,51483_u16,44569_u16,53140_u16,61274_u16,10657_u16];
_8 = [1412_u16,42697_u16,43598_u16,36626_u16,50014_u16,59927_u16,62963_u16,30380_u16];
_4 = 33623_u16 as u8;
_11 = '\u{92bcb}' as i32;
_14 = (-9223372036854775808_isize) * 12_isize;
_3 = !_7;
_13 = 5834835958191864196_i64 | 5480446038181496437_i64;
_8 = [57176_u16,46560_u16,33793_u16,16019_u16,10265_u16,55270_u16,60840_u16,27210_u16];
_5 = [false,false,true,false];
Call(_14 = fn14(Move(_1), _8, _7, _6, _7, _6, _12, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
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
_12 = _3;
_16 = 35159_u16 as i64;
_4 = _7;
_13 = 107210737079095079162393623160015942318_i128 as i64;
_9 = _11;
_11 = _9 & _9;
Goto(bb11)
}
bb11 = {
_10 = [true,true,false,false];
Goto(bb12)
}
bb12 = {
RET = 271120566718190040767635203319090030480_u128 as f64;
_16 = -_13;
_11 = _9 + _9;
_15 = _14;
RET = 18084428753262428226_usize as f64;
_6 = [true,false,true,true];
_16 = _13;
RET = 20805_i16 as f64;
_7 = true as u8;
_20 = (52778141732424271553759867137720869607_u128,);
_10 = [true,false,true,false];
_17 = -_14;
_10 = [false,true,true,false];
_13 = _16;
_13 = _20.0 as i64;
_12 = _4 << _3;
RET = 5835_u16 as f64;
_14 = (-5229_i16) as isize;
_5 = [false,true,false,false];
_3 = _12 ^ _12;
match _20.0 {
0 => bb7,
1 => bb5,
2 => bb3,
52778141732424271553759867137720869607 => bb13,
_ => bb4
}
}
bb13 = {
_17 = !_15;
_9 = true as i32;
_15 = _17 | _14;
_13 = _16;
_8 = [37320_u16,2024_u16,6998_u16,5017_u16,33463_u16,17042_u16,28595_u16,31882_u16];
_8 = [32969_u16,23037_u16,38604_u16,26677_u16,5210_u16,44754_u16,43121_u16,61945_u16];
_14 = _15 - _15;
_6 = _5;
_12 = _11 as u8;
_8 = [28535_u16,26271_u16,9576_u16,26084_u16,33386_u16,8369_u16,39258_u16,35888_u16];
_13 = !_16;
_20 = (268896032278645776434746843841837084242_u128,);
_17 = false as isize;
_23 = [true,false,false];
_7 = false as u8;
_16 = _13;
_11 = !_9;
_14 = !_15;
_12 = _20.0 as u8;
_10 = [true,false,false,true];
RET = _16 as f64;
Goto(bb14)
}
bb14 = {
_12 = RET as u8;
_20 = (141901821542172720240448546580181176360_u128,);
_9 = !_11;
_16 = -_13;
_6 = [false,true,true,true];
_3 = _4 & _4;
_12 = _3;
_20.0 = 89672243040041952461116770358806313031_u128;
RET = 5829669524805984130_u64 as f64;
_20.0 = 255104439603145647440592392417824683969_u128 - 102256598325205974313498374941757507637_u128;
_23 = [false,false,false];
_16 = _13;
_20 = (253775311129881107968175488006083122382_u128,);
_15 = _14;
_13 = _16 - _16;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(13_usize, 4_usize, Move(_4), 3_usize, Move(_3), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(13_usize, 20_usize, Move(_20), 9_usize, Move(_9), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *mut i128,mut _2: [u16; 8],mut _3: u8,mut _4: [bool; 4],mut _5: u8,mut _6: [bool; 4],mut _7: u8,mut _8: *mut f32) -> isize {
mir! {
type RET = isize;
let _9: usize;
let _10: i128;
let _11: u16;
let _12: u64;
let _13: bool;
let _14: isize;
let _15: &'static usize;
let _16: *mut ((u32,), *mut [usize; 7], [u16; 7]);
let _17: (bool, f64);
let _18: u64;
let _19: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _20: u32;
let _21: &'static i8;
let _22: u8;
let _23: f64;
let _24: &'static [bool; 4];
let _25: ((u32,), *mut [usize; 7], [u16; 7]);
let _26: ((f32,), &'static f32);
let _27: ([u16; 7], usize);
let _28: (bool, f64);
let _29: i64;
let _30: [u16; 7];
let _31: isize;
let _32: ([u16; 7], usize);
let _33: &'static &'static usize;
let _34: f32;
let _35: Adt50;
let _36: (f32,);
let _37: ((u32,), *mut [usize; 7], [u16; 7]);
let _38: char;
let _39: i64;
let _40: *const &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _41: [bool; 4];
let _42: ();
let _43: ();
{
_6 = _4;
Goto(bb1)
}
bb1 = {
_6 = [false,true,false,false];
_3 = _5;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_2 = [17739_u16,47199_u16,45331_u16,61401_u16,18974_u16,45806_u16,62944_u16,20478_u16];
_2 = [19319_u16,9354_u16,1657_u16,52415_u16,39341_u16,57323_u16,14378_u16,23111_u16];
_5 = 5_usize as u8;
_4 = _6;
_10 = 79159860998771501032878859269719343964_i128 - (-40533482221325011208385293988981770237_i128);
_1 = core::ptr::addr_of_mut!(_10);
_12 = 13569576502482202893_u64 >> _3;
(*_1) = (-69562113168559047305390581036908494046_i128) << _3;
_9 = !3656393726894249845_usize;
_2 = [20640_u16,42267_u16,44537_u16,49380_u16,36163_u16,16757_u16,45632_u16,28812_u16];
(*_1) = (-95935481541721606741684063896653971785_i128);
_7 = 444190282_u32 as u8;
_11 = 23095_u16 | 11189_u16;
_10 = -(-24786504257492386257656679908974399815_i128);
_12 = 3880133676532460980_u64;
_12 = 1582164125_i32 as u64;
RET = (-9223372036854775808_isize);
_11 = 38703_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = [false,true,true,false];
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = _3 < _3;
RET = !(-9223372036854775808_isize);
_10 = 62453994612439752623240639902209456303_i128 >> _3;
Goto(bb3)
}
bb3 = {
(*_1) = 96139134_i32 as i128;
_13 = false;
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
38703 => bb6,
_ => bb5
}
}
bb4 = {
_2 = [17739_u16,47199_u16,45331_u16,61401_u16,18974_u16,45806_u16,62944_u16,20478_u16];
_2 = [19319_u16,9354_u16,1657_u16,52415_u16,39341_u16,57323_u16,14378_u16,23111_u16];
_5 = 5_usize as u8;
_4 = _6;
_10 = 79159860998771501032878859269719343964_i128 - (-40533482221325011208385293988981770237_i128);
_1 = core::ptr::addr_of_mut!(_10);
_12 = 13569576502482202893_u64 >> _3;
(*_1) = (-69562113168559047305390581036908494046_i128) << _3;
_9 = !3656393726894249845_usize;
_2 = [20640_u16,42267_u16,44537_u16,49380_u16,36163_u16,16757_u16,45632_u16,28812_u16];
(*_1) = (-95935481541721606741684063896653971785_i128);
_7 = 444190282_u32 as u8;
_11 = 23095_u16 | 11189_u16;
_10 = -(-24786504257492386257656679908974399815_i128);
_12 = 3880133676532460980_u64;
_12 = 1582164125_i32 as u64;
RET = (-9223372036854775808_isize);
_11 = 38703_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = [false,true,true,false];
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = _3 < _3;
RET = !(-9223372036854775808_isize);
_10 = 62453994612439752623240639902209456303_i128 >> _3;
Goto(bb3)
}
bb5 = {
_6 = [false,true,false,false];
_3 = _5;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
Goto(bb2)
}
bb6 = {
(*_1) = 19847575540252162185616232189999444246_i128;
_15 = &_9;
_13 = true | false;
_4 = [_13,_13,_13,_13];
_3 = _5;
_14 = RET;
_14 = RET;
_17.1 = _10 as f64;
(*_1) = -107897740900596152963918214738852872148_i128;
(*_1) = !97645969229267970829430011316632503680_i128;
_7 = _5 >> _9;
_1 = core::ptr::addr_of_mut!(_10);
_19.2.0 = !_5;
_14 = RET;
_17.0 = !_13;
_20 = _17.1 as u32;
(*_1) = (-130631683745431431040541308819114198173_i128) | (-114866806353620683001064971422478341352_i128);
_17.0 = !_13;
_5 = _7 | _19.2.0;
_5 = 203209711414169361637886915269300459685_u128 as u8;
_5 = !_7;
_18 = 35_i8 as u64;
_17.1 = _9 as f64;
Call(_19.2 = fn15(Move(_15), _10, Move(_8), _14, _2, _17.0, Move(_1), _10, _6, _17, _6, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_22 = _10 as u8;
_18 = _12;
_4 = [_13,_13,_17.0,_17.0];
_20 = 3975773783_u32;
_17.0 = _13;
_1 = core::ptr::addr_of_mut!(_10);
_25.0.0 = !_20;
_17.0 = _13;
_18 = !_12;
_5 = _19.2.0;
_4 = _6;
_14 = -RET;
_12 = (-15132_i16) as u64;
RET = -_14;
_25.0 = (_20,);
_8 = core::ptr::addr_of_mut!(_26.0.0);
_25.2 = [_11,_11,_11,_11,_11,_11,_11];
_25.2 = [_11,_11,_11,_11,_11,_11,_11];
RET = !_14;
_27.0 = [_11,_11,_11,_11,_11,_11,_11];
_26.1 = &(*_8);
_27 = (_25.2, _9);
_16 = core::ptr::addr_of_mut!(_25);
_15 = &_9;
_1 = core::ptr::addr_of_mut!(_10);
match _11 {
0 => bb8,
38703 => bb10,
_ => bb9
}
}
bb8 = {
_2 = [17739_u16,47199_u16,45331_u16,61401_u16,18974_u16,45806_u16,62944_u16,20478_u16];
_2 = [19319_u16,9354_u16,1657_u16,52415_u16,39341_u16,57323_u16,14378_u16,23111_u16];
_5 = 5_usize as u8;
_4 = _6;
_10 = 79159860998771501032878859269719343964_i128 - (-40533482221325011208385293988981770237_i128);
_1 = core::ptr::addr_of_mut!(_10);
_12 = 13569576502482202893_u64 >> _3;
(*_1) = (-69562113168559047305390581036908494046_i128) << _3;
_9 = !3656393726894249845_usize;
_2 = [20640_u16,42267_u16,44537_u16,49380_u16,36163_u16,16757_u16,45632_u16,28812_u16];
(*_1) = (-95935481541721606741684063896653971785_i128);
_7 = 444190282_u32 as u8;
_11 = 23095_u16 | 11189_u16;
_10 = -(-24786504257492386257656679908974399815_i128);
_12 = 3880133676532460980_u64;
_12 = 1582164125_i32 as u64;
RET = (-9223372036854775808_isize);
_11 = 38703_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = [false,true,true,false];
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = _3 < _3;
RET = !(-9223372036854775808_isize);
_10 = 62453994612439752623240639902209456303_i128 >> _3;
Goto(bb3)
}
bb9 = {
_2 = [17739_u16,47199_u16,45331_u16,61401_u16,18974_u16,45806_u16,62944_u16,20478_u16];
_2 = [19319_u16,9354_u16,1657_u16,52415_u16,39341_u16,57323_u16,14378_u16,23111_u16];
_5 = 5_usize as u8;
_4 = _6;
_10 = 79159860998771501032878859269719343964_i128 - (-40533482221325011208385293988981770237_i128);
_1 = core::ptr::addr_of_mut!(_10);
_12 = 13569576502482202893_u64 >> _3;
(*_1) = (-69562113168559047305390581036908494046_i128) << _3;
_9 = !3656393726894249845_usize;
_2 = [20640_u16,42267_u16,44537_u16,49380_u16,36163_u16,16757_u16,45632_u16,28812_u16];
(*_1) = (-95935481541721606741684063896653971785_i128);
_7 = 444190282_u32 as u8;
_11 = 23095_u16 | 11189_u16;
_10 = -(-24786504257492386257656679908974399815_i128);
_12 = 3880133676532460980_u64;
_12 = 1582164125_i32 as u64;
RET = (-9223372036854775808_isize);
_11 = 38703_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = [false,true,true,false];
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = _3 < _3;
RET = !(-9223372036854775808_isize);
_10 = 62453994612439752623240639902209456303_i128 >> _3;
Goto(bb3)
}
bb10 = {
_23 = -_17.1;
_26.0.0 = _14 as f32;
_13 = _17.0;
_7 = !_5;
_10 = 103865467482486525447614125834914534650_i128;
_27.0 = (*_16).2;
RET = !_14;
_24 = &_4;
RET = -_14;
_13 = _17.0 | _17.0;
Goto(bb11)
}
bb11 = {
_15 = &(*_15);
_9 = _13 as usize;
_11 = (*_8) as u16;
_26.0.0 = _11 as f32;
_29 = -2262613818878084333_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_24 = &_6;
_28 = (_13, _17.1);
_25.0 = (_20,);
_9 = _27.1;
_7 = !_19.2.0;
_9 = _14 as usize;
(*_1) = 19246184308172453218885284708613656168_i128;
_31 = _10 as isize;
_1 = core::ptr::addr_of_mut!((*_1));
_24 = &(*_24);
_25.2 = [_11,_11,_11,_11,_11,_11,_11];
_34 = _26.0.0;
_24 = &(*_24);
_15 = &_9;
_20 = !(*_16).0.0;
RET = (-27845_i16) as isize;
_25.0 = (_20,);
Goto(bb12)
}
bb12 = {
RET = _27.1 as isize;
_27.1 = _9;
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_15 = &(*_15);
_28.1 = _23;
(*_16).0 = (_20,);
_27.0 = (*_16).2;
_32 = (_25.2, (*_15));
_30 = [_11,_11,_11,_11,_11,_11,_11];
_16 = core::ptr::addr_of_mut!((*_16));
_11 = 65409_u16 >> _19.2.0;
_4 = [_13,_13,_28.0,_13];
(*_16).2 = _32.0;
_36.0 = (*_16).0.0 as f32;
_34 = _14 as f32;
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6 = [_13,_28.0,_17.0,_13];
_14 = _31 >> _5;
_2 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = _28.0;
_26.1 = &(*_8);
match (*_1) {
19246184308172453218885284708613656168 => bb13,
_ => bb8
}
}
bb13 = {
_36.0 = (*_8) * (*_8);
_26.0 = _36;
Goto(bb14)
}
bb14 = {
_30 = [_11,_11,_11,_11,_11,_11,_11];
_25.0 = (_20,);
_14 = _31;
_13 = !_28.0;
_37.2 = [_11,_11,_11,_11,_11,_11,_11];
_23 = (*_16).0.0 as f64;
_22 = !_7;
_26.0.0 = _36.0 - _34;
_1 = core::ptr::addr_of_mut!((*_1));
_9 = _19.2.0 as usize;
_23 = _17.1;
(*_16).0 = (_20,);
_27.1 = _28.0 as usize;
(*_16).2 = [_11,_11,_11,_11,_11,_11,_11];
_25.0 = (_20,);
_20 = _25.0.0;
_38 = '\u{dc469}';
_24 = &_4;
_36 = ((*_8),);
(*_16).0.0 = !_20;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(14_usize, 7_usize, Move(_7), 11_usize, Move(_11), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(14_usize, 32_usize, Move(_32), 30_usize, Move(_30), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(14_usize, 14_usize, Move(_14), 6_usize, Move(_6), 43_usize, _43, 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static usize,mut _2: i128,mut _3: *mut f32,mut _4: isize,mut _5: [u16; 8],mut _6: bool,mut _7: *mut i128,mut _8: i128,mut _9: [bool; 4],mut _10: (bool, f64),mut _11: [bool; 4],mut _12: u8) -> (u8,) {
mir! {
type RET = (u8,);
let _13: char;
let _14: &'static i32;
let _15: char;
let _16: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _17: u128;
let _18: *mut i128;
let _19: Adt34;
let _20: &'static i32;
let _21: Adt69;
let _22: u8;
let _23: *const &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _24: f64;
let _25: [i128; 8];
let _26: bool;
let _27: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _28: u32;
let _29: u16;
let _30: f64;
let _31: ([u16; 7], usize);
let _32: [usize; 7];
let _33: ((f32,), &'static f32);
let _34: bool;
let _35: usize;
let _36: (bool, f64);
let _37: (u32,);
let _38: Adt62;
let _39: i16;
let _40: [bool; 3];
let _41: bool;
let _42: char;
let _43: &'static [char; 7];
let _44: &'static i32;
let _45: *mut f32;
let _46: &'static [char; 7];
let _47: &'static Adt34;
let _48: ();
let _49: ();
{
_6 = _8 < _2;
_13 = '\u{6f94e}';
RET.0 = _12 >> _12;
_7 = core::ptr::addr_of_mut!(_2);
(*_7) = !_8;
_8 = (*_7) & _2;
_11 = [_10.0,_6,_6,_6];
_13 = '\u{a40d8}';
_11 = _9;
Goto(bb1)
}
bb1 = {
_15 = _13;
_2 = 40617_u16 as i128;
_5 = [27401_u16,52304_u16,8898_u16,1318_u16,11258_u16,34000_u16,32170_u16,36013_u16];
_7 = core::ptr::addr_of_mut!(_2);
_18 = Move(_7);
_9 = [_6,_6,_6,_6];
_11 = _9;
RET.0 = 4556818565997206641_i64 as u8;
_8 = _2 & _2;
Call(_19.fld0.0 = fn16(Move(_3), _10.0, _5, _13, _11, Move(_18), _15, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = 26100258747971591359929501392957770341_u128 + 31454761596376913423442728199343680672_u128;
_21.fld2 = (_6, _10.1);
_18 = core::ptr::addr_of_mut!(_2);
_9 = [_10.0,_21.fld2.0,_6,_6];
_21.fld1 = [4259224124_u32];
_21.fld0 = core::ptr::addr_of_mut!(_19.fld2);
Goto(bb3)
}
bb3 = {
_21.fld0 = core::ptr::addr_of_mut!(_19.fld2);
(*_18) = _8 + _8;
_22 = !_12;
_10.1 = _21.fld2.1 * _21.fld2.1;
_10.0 = !_21.fld2.0;
_21.fld2 = (_6, _10.1);
_19.fld1 = core::ptr::addr_of_mut!(_19.fld4);
_19.fld0.0 = RET.0 | _12;
_19.fld0.0 = _22 - _22;
_19.fld4 = !_17;
_21.fld2.1 = _10.1;
RET.0 = _12 - _22;
_19.fld0 = (_12,);
_6 = !_21.fld2.0;
(*_18) = _8;
_6 = !_10.0;
Goto(bb4)
}
bb4 = {
_19.fld3 = (-67_i8) - 39_i8;
(*_18) = _8 ^ _8;
_19.fld1 = core::ptr::addr_of_mut!(_19.fld4);
_19.fld2 = [8158671426916588200_usize,1_usize,2_usize,7_usize,18273488923590675841_usize,4_usize,7_usize];
_5 = [55901_u16,43036_u16,41596_u16,40952_u16,63544_u16,39475_u16,23769_u16,63057_u16];
_25 = [_8,_8,(*_18),_2,(*_18),_2,(*_18),_2];
_23 = core::ptr::addr_of!(_16.1);
_21.fld0 = core::ptr::addr_of_mut!(_19.fld2);
RET.0 = _19.fld0.0 * _19.fld0.0;
(*_18) = !_8;
Goto(bb5)
}
bb5 = {
_28 = 1394417014_u32;
_16.2.0 = RET.0 | RET.0;
_19.fld1 = core::ptr::addr_of_mut!(_17);
_21.fld2.1 = (-544943040_i32) as f64;
_29 = !63642_u16;
_10.0 = _21.fld2.0;
_19.fld4 = _17 - _17;
_26 = _6;
_27.2 = (_12,);
RET.0 = _19.fld0.0;
_25 = [_2,(*_18),_8,_8,_2,(*_18),(*_18),_8];
_12 = _16.2.0 ^ _16.2.0;
_10.0 = _26;
_25 = [_2,(*_18),(*_18),(*_18),_2,(*_18),_2,(*_18)];
_12 = !_16.2.0;
_11 = _9;
_10.0 = _21.fld2.0;
_27.2.0 = _12;
_28 = _4 as u32;
_6 = !_21.fld2.0;
_19.fld0 = (_27.2.0,);
RET = _27.2;
_19.fld3 = 39_i8;
Goto(bb6)
}
bb6 = {
_10 = _21.fld2;
_5 = [_29,_29,_29,_29,_29,_29,_29,_29];
_1 = &_31.1;
_32 = _19.fld2;
_10.1 = _21.fld2.1 + _21.fld2.1;
_29 = !10293_u16;
_25 = [(*_18),(*_18),(*_18),(*_18),_8,_8,(*_18),(*_18)];
_16.2 = _27.2;
_31.0 = [_29,_29,_29,_29,_29,_29,_29];
_8 = (*_18);
_31.1 = 14614264928858645031_usize;
_11 = [_21.fld2.0,_6,_6,_21.fld2.0];
_16.2.0 = _12 << _29;
_34 = _2 != (*_18);
match _19.fld3 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
39 => bb11,
_ => bb10
}
}
bb7 = {
_28 = 1394417014_u32;
_16.2.0 = RET.0 | RET.0;
_19.fld1 = core::ptr::addr_of_mut!(_17);
_21.fld2.1 = (-544943040_i32) as f64;
_29 = !63642_u16;
_10.0 = _21.fld2.0;
_19.fld4 = _17 - _17;
_26 = _6;
_27.2 = (_12,);
RET.0 = _19.fld0.0;
_25 = [_2,(*_18),_8,_8,_2,(*_18),(*_18),_8];
_12 = _16.2.0 ^ _16.2.0;
_10.0 = _26;
_25 = [_2,(*_18),(*_18),(*_18),_2,(*_18),_2,(*_18)];
_12 = !_16.2.0;
_11 = _9;
_10.0 = _21.fld2.0;
_27.2.0 = _12;
_28 = _4 as u32;
_6 = !_21.fld2.0;
_19.fld0 = (_27.2.0,);
RET = _27.2;
_19.fld3 = 39_i8;
Goto(bb6)
}
bb8 = {
_15 = _13;
_2 = 40617_u16 as i128;
_5 = [27401_u16,52304_u16,8898_u16,1318_u16,11258_u16,34000_u16,32170_u16,36013_u16];
_7 = core::ptr::addr_of_mut!(_2);
_18 = Move(_7);
_9 = [_6,_6,_6,_6];
_11 = _9;
RET.0 = 4556818565997206641_i64 as u8;
_8 = _2 & _2;
Call(_19.fld0.0 = fn16(Move(_3), _10.0, _5, _13, _11, Move(_18), _15, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_21.fld0 = core::ptr::addr_of_mut!(_19.fld2);
(*_18) = _8 + _8;
_22 = !_12;
_10.1 = _21.fld2.1 * _21.fld2.1;
_10.0 = !_21.fld2.0;
_21.fld2 = (_6, _10.1);
_19.fld1 = core::ptr::addr_of_mut!(_19.fld4);
_19.fld0.0 = RET.0 | _12;
_19.fld0.0 = _22 - _22;
_19.fld4 = !_17;
_21.fld2.1 = _10.1;
RET.0 = _12 - _22;
_19.fld0 = (_12,);
_6 = !_21.fld2.0;
(*_18) = _8;
_6 = !_10.0;
Goto(bb4)
}
bb10 = {
_17 = 26100258747971591359929501392957770341_u128 + 31454761596376913423442728199343680672_u128;
_21.fld2 = (_6, _10.1);
_18 = core::ptr::addr_of_mut!(_2);
_9 = [_10.0,_21.fld2.0,_6,_6];
_21.fld1 = [4259224124_u32];
_21.fld0 = core::ptr::addr_of_mut!(_19.fld2);
Goto(bb3)
}
bb11 = {
_25 = [(*_18),(*_18),_2,(*_18),_8,(*_18),(*_18),_8];
_1 = &_31.1;
_4 = 27758_i16 as isize;
_5 = [_29,_29,_29,_29,_29,_29,_29,_29];
_19.fld1 = core::ptr::addr_of_mut!(_17);
_21.fld1 = [_28];
_19.fld0.0 = !_12;
Goto(bb12)
}
bb12 = {
_30 = _10.1 + _21.fld2.1;
_13 = _15;
_21.fld2 = (_26, _10.1);
_2 = _8 - _8;
_4 = 9223372036854775807_isize;
_36.0 = !_34;
_23 = core::ptr::addr_of!((*_23));
RET = (_19.fld0.0,);
(*_18) = _8 + _8;
_1 = &_35;
_19.fld2 = [_31.1,_31.1,_31.1,_31.1,_31.1,_31.1,_31.1];
_2 = -_8;
_39 = (-23123_i16) + 6727_i16;
_8 = !(*_18);
_5 = [_29,_29,_29,_29,_29,_29,_29,_29];
_33.0.0 = _31.1 as f32;
_38.fld4.0 = Adt26::Variant0 { fld0: _28,fld1: _31.1,fld2: _4,fld3: _19.fld0.0,fld4: (*_18) };
Goto(bb13)
}
bb13 = {
_6 = _34 | _34;
_29 = 974_u16;
_21.fld2 = (_6, _30);
_7 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_38.fld4.0, 0), 4)));
_19.fld0 = RET;
place!(Field::<u8>(Variant(_38.fld4.0, 0), 3)) = !_16.2.0;
_16.0 = core::ptr::addr_of!(_38.fld4.0);
_15 = _13;
_38.fld0 = core::ptr::addr_of_mut!(_19.fld4);
_16.2.0 = Field::<u8>(Variant(_38.fld4.0, 0), 3) >> _19.fld4;
_40 = [_26,_36.0,_36.0];
_21.fld2.1 = 14499941158885155328_u64 as f64;
_10 = _21.fld2;
place!(Field::<i128>(Variant(_38.fld4.0, 0), 4)) = (*_18) + _2;
_28 = _19.fld4 as u32;
_19.fld0.0 = Field::<u8>(Variant(_38.fld4.0, 0), 3) << _39;
match Field::<isize>(Variant(_38.fld4.0, 0), 2) {
9223372036854775807 => bb15,
_ => bb14
}
}
bb14 = {
_10 = _21.fld2;
_5 = [_29,_29,_29,_29,_29,_29,_29,_29];
_1 = &_31.1;
_32 = _19.fld2;
_10.1 = _21.fld2.1 + _21.fld2.1;
_29 = !10293_u16;
_25 = [(*_18),(*_18),(*_18),(*_18),_8,_8,(*_18),(*_18)];
_16.2 = _27.2;
_31.0 = [_29,_29,_29,_29,_29,_29,_29];
_8 = (*_18);
_31.1 = 14614264928858645031_usize;
_11 = [_21.fld2.0,_6,_6,_21.fld2.0];
_16.2.0 = _12 << _29;
_34 = _2 != (*_18);
match _19.fld3 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
39 => bb11,
_ => bb10
}
}
bb15 = {
_38.fld2 = (_27.2.0,);
_36.0 = _28 > _28;
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(15_usize, 13_usize, Move(_13), 26_usize, Move(_26), 22_usize, Move(_22), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(15_usize, 6_usize, Move(_6), 28_usize, Move(_28), 12_usize, Move(_12), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(15_usize, 25_usize, Move(_25), 4_usize, Move(_4), 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *mut f32,mut _2: bool,mut _3: [u16; 8],mut _4: char,mut _5: [bool; 4],mut _6: *mut i128,mut _7: char,mut _8: [u16; 8]) -> u8 {
mir! {
type RET = u8;
let _9: &'static (i128, &'static i16, &'static *mut i128, (u32,));
let _10: isize;
let _11: char;
let _12: char;
let _13: ((f32,), &'static f32);
let _14: [u16; 8];
let _15: &'static f32;
let _16: isize;
let _17: isize;
let _18: f64;
let _19: u64;
let _20: f32;
let _21: &'static [bool; 4];
let _22: u128;
let _23: &'static Adt69;
let _24: *mut ((u32,), *mut [usize; 7], [u16; 7]);
let _25: &'static i16;
let _26: (((f32,), &'static f32),);
let _27: (u16,);
let _28: &'static i8;
let _29: *mut f64;
let _30: usize;
let _31: u8;
let _32: i64;
let _33: Adt62;
let _34: f32;
let _35: [i128; 8];
let _36: [i8; 2];
let _37: isize;
let _38: i16;
let _39: [bool; 3];
let _40: ();
let _41: ();
{
_7 = _4;
RET = 97_u8;
_8 = [48003_u16,50692_u16,61058_u16,40260_u16,48002_u16,29486_u16,57785_u16,17844_u16];
_8 = [19604_u16,27677_u16,13999_u16,2879_u16,36445_u16,28103_u16,36359_u16,63164_u16];
RET = 78_u8;
_2 = !false;
RET = 73_u8;
RET = !175_u8;
RET = 10_u8;
_3 = _8;
_10 = -9223372036854775807_isize;
_11 = _4;
_7 = _11;
_12 = _11;
_11 = _7;
_5 = [_2,_2,_2,_2];
_10 = (-19_isize);
_11 = _12;
RET = 10735419072756554067_u64 as u8;
_4 = _11;
match _10 {
340282366920938463463374607431768211437 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
Goto(bb3)
}
bb3 = {
_12 = _4;
_8 = _3;
_10 = 315593944396954597809637122562218240476_u128 as isize;
RET = 231_u8;
_7 = _11;
_13.1 = &_13.0.0;
_2 = true;
_7 = _12;
_8 = [14467_u16,49494_u16,39697_u16,25516_u16,2917_u16,26050_u16,47981_u16,26915_u16];
_4 = _11;
_13.0.0 = (-45_i8) as f32;
_2 = true;
_1 = core::ptr::addr_of_mut!(_13.0.0);
_13.1 = &(*_1);
_5 = [_2,_2,_2,_2];
_10 = 11505_u16 as isize;
_14 = _8;
_2 = !false;
_13.1 = &(*_1);
_2 = _13.0.0 < _13.0.0;
Goto(bb4)
}
bb4 = {
_10 = !9223372036854775807_isize;
_8 = [784_u16,62877_u16,65413_u16,4092_u16,65268_u16,52338_u16,41885_u16,19634_u16];
_14 = [10727_u16,19887_u16,27389_u16,43608_u16,6991_u16,46663_u16,37129_u16,2693_u16];
_10 = 12592639490117095839_u64 as isize;
_12 = _4;
_10 = 74_isize * (-25_isize);
_10 = (-9223372036854775808_isize);
_2 = !true;
(*_1) = RET as f32;
(*_1) = 489779826_i32 as f32;
Goto(bb5)
}
bb5 = {
_10 = 1941997413_u32 as isize;
(*_1) = 3961462807040731445_i64 as f32;
_15 = &(*_1);
_17 = (-2261465475076905983_i64) as isize;
_16 = !_10;
_8 = [41969_u16,15334_u16,64008_u16,46306_u16,36206_u16,29022_u16,64821_u16,35545_u16];
Call(_5 = fn17(Move(_15), _16, _13.0, _14, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15 = &(*_1);
(*_1) = (-21150_i16) as f32;
_8 = [13698_u16,43403_u16,18444_u16,40448_u16,48526_u16,14331_u16,64837_u16,46949_u16];
_15 = &_13.0.0;
(*_1) = (-7494245054247883092_i64) as f32;
(*_1) = RET as f32;
_13.1 = &(*_1);
(*_1) = 122638257678593519952055733094478791492_u128 as f32;
RET = 29_u8;
_8 = [64513_u16,10385_u16,4493_u16,53156_u16,12929_u16,3489_u16,32285_u16,10998_u16];
_15 = &_13.0.0;
_18 = 12875221673173761592_usize as f64;
_7 = _4;
_10 = -_16;
_13.0.0 = 13636703533547452054_u64 as f32;
_7 = _11;
_8 = [16239_u16,19348_u16,36568_u16,31997_u16,20848_u16,59693_u16,49620_u16,7587_u16];
_19 = 13742107997896729979_u64 * 7295910907094162649_u64;
_22 = (-24094_i16) as u128;
_10 = -_17;
_19 = _13.0.0 as u64;
Goto(bb7)
}
bb7 = {
_21 = &_5;
Goto(bb8)
}
bb8 = {
_17 = _10 + _16;
_20 = 1745553380_i32 as f32;
_10 = _16;
_10 = _16 & _16;
_5 = [_2,_2,_2,_2];
RET = !208_u8;
(*_1) = _18 as f32;
_13.0.0 = -_20;
_13.1 = &_13.0.0;
Goto(bb9)
}
bb9 = {
_15 = Move(_13.1);
_12 = _4;
_12 = _7;
_8 = _3;
_11 = _4;
_10 = !_17;
_16 = _10;
_2 = !false;
_20 = (*_1) - (*_1);
_26.0.1 = &(*_1);
_18 = _22 as f64;
_26.0.0 = (_13.0.0,);
_13.0 = (_20,);
_5 = [_2,_2,_2,_2];
_18 = (-141663254156650348200348028180585343879_i128) as f64;
_19 = (-10143_i16) as u64;
_3 = _14;
_11 = _7;
_16 = _17 - _17;
_27.0 = 42429_u16;
_15 = &_13.0.0;
_14 = _3;
_17 = _16 * _16;
_11 = _12;
match _27.0 {
42429 => bb10,
_ => bb8
}
}
bb10 = {
_13.0 = _26.0.0;
(*_1) = _26.0.0.0;
_14 = [_27.0,_27.0,_27.0,_27.0,_27.0,_27.0,_27.0,_27.0];
RET = 139_u8 ^ 165_u8;
_13.1 = &(*_1);
_12 = _4;
_21 = &_5;
_26.0 = (_13.0, Move(_13.1));
_3 = [_27.0,_27.0,_27.0,_27.0,_27.0,_27.0,_27.0,_27.0];
_7 = _11;
_27.0 = !44290_u16;
_19 = 17508582622164303934_u64;
_2 = !true;
_29 = core::ptr::addr_of_mut!(_18);
(*_1) = -_26.0.0.0;
_26.0.1 = &(*_1);
_13.0 = (_26.0.0.0,);
_19 = 4333542008856370783_u64 + 12945401155178595121_u64;
_10 = -_16;
_15 = &(*_1);
Goto(bb11)
}
bb11 = {
_26.0 = (_13.0, Move(_15));
_26.0.0 = (_20,);
_13.1 = &_13.0.0;
_13.0 = (_26.0.0.0,);
_8 = _3;
_13.1 = &_20;
_13.0.0 = -_20;
RET = _18 as u8;
_26.0.1 = Move(_13.1);
_29 = core::ptr::addr_of_mut!(_18);
_22 = 951137506315302739_i64 as u128;
_13.1 = &(*_1);
_8 = _3;
_16 = -_17;
RET = !224_u8;
_33.fld4.0 = Adt26::Variant0 { fld0: 2530820298_u32,fld1: 4_usize,fld2: _16,fld3: RET,fld4: 23535011917446525885030497081453313435_i128 };
_33.fld3.0 = _27.0;
_26.0.0 = _13.0;
_22 = 14596933619836490560651009153128108774_u128 | 39702259385227227991432564421586724533_u128;
place!(Field::<u32>(Variant(_33.fld4.0, 0), 0)) = 2799199531_u32 * 1246418048_u32;
_26.0 = Move(_13);
_19 = 12322002609702946970_u64 | 10816098591157695840_u64;
Call(place!(Field::<u8>(Variant(_33.fld4.0, 0), 3)) = core::intrinsics::bswap(RET), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3 = [_33.fld3.0,_27.0,_27.0,_27.0,_27.0,_33.fld3.0,_27.0,_33.fld3.0];
_31 = Field::<u8>(Variant(_33.fld4.0, 0), 3) - Field::<u8>(Variant(_33.fld4.0, 0), 3);
_33.fld0 = core::ptr::addr_of_mut!(_22);
_33.fld4.1 = Move(_6);
_32 = (-1340495813356410935_i64) >> _16;
_20 = Field::<u8>(Variant(_33.fld4.0, 0), 3) as f32;
_13.0 = _26.0.0;
_18 = Field::<u32>(Variant(_33.fld4.0, 0), 0) as f64;
Goto(bb13)
}
bb13 = {
(*_1) = (*_29) as f32;
_8 = [_33.fld3.0,_33.fld3.0,_27.0,_27.0,_27.0,_27.0,_27.0,_27.0];
_33.fld4.0 = Adt26::Variant0 { fld0: 2501459347_u32,fld1: 8601836049502763232_usize,fld2: _10,fld3: _31,fld4: 89390005631981874470837270203598776687_i128 };
_33.fld5 = [6_usize,3_usize,9995385908290547422_usize,2_usize,13952523366357612007_usize,0_usize,1_usize];
_13.1 = &_26.0.0.0;
_18 = _19 as f64;
_15 = Move(_13.1);
_33.fld2.0 = !RET;
_27 = (_33.fld3.0,);
_19 = 17389090957897525475_u64;
_27 = _33.fld3;
_1 = core::ptr::addr_of_mut!(_13.0.0);
_31 = (-1373173633_i32) as u8;
_31 = RET & _33.fld2.0;
_13.1 = &_20;
_33.fld0 = core::ptr::addr_of_mut!(_22);
_27 = _33.fld3;
place!(Field::<i128>(Variant(_33.fld4.0, 0), 4)) = 104919438_u32 as i128;
_2 = false;
_33.fld4.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_33.fld4.0, 0), 4)));
_33.fld1 = _27.0;
_19 = !17161132972083349818_u64;
place!(Field::<u32>(Variant(_33.fld4.0, 0), 0)) = 1315867718_u32;
_34 = (*_1);
_16 = -Field::<isize>(Variant(_33.fld4.0, 0), 2);
_30 = 17084766824021756711_usize + 13270400849904273535_usize;
Goto(bb14)
}
bb14 = {
_31 = RET;
(*_1) = _20;
_33.fld5 = [_30,_30,_30,_30,_30,_30,_30];
_16 = -_17;
place!(Field::<usize>(Variant(_33.fld4.0, 0), 1)) = _30;
_26.0 = Move(_13);
_33.fld2 = (_31,);
place!(Field::<i128>(Variant(_33.fld4.0, 0), 4)) = (-29213379924712687693302985274682527395_i128);
(*_1) = -_34;
_7 = _4;
_7 = _4;
_26.0.0 = (_13.0.0,);
_15 = &_34;
RET = _2 as u8;
_32 = 91631520_i32 as i64;
_26.0 = (_13.0, Move(_15));
_33.fld4.0 = Adt26::Variant0 { fld0: 3685206834_u32,fld1: _30,fld2: _17,fld3: _33.fld2.0,fld4: (-70920674704752653733516853658607012944_i128) };
_22 = _19 as u128;
_39 = [_2,_2,_2];
_20 = _26.0.0.0;
_15 = &_13.0.0;
_35 = [45210961693866264218668148185130612843_i128,153542602953638156844007511261553166901_i128,(-41470135734278407161879408331370602418_i128),89757439440077603384639449336812211608_i128,106802060889224926950156595644158441108_i128,(-87160104457225083749758226572888028248_i128),(-76484320848349403825016768015864326367_i128),(-26989039480353603437475189177792761969_i128)];
_33.fld2 = (Field::<u8>(Variant(_33.fld4.0, 0), 3),);
_37 = _10;
_6 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_33.fld4.0, 0), 4)));
_33.fld4.0 = Adt26::Variant1 { fld0: (*_1),fld1: _35 };
_14 = [_27.0,_33.fld1,_33.fld1,_33.fld3.0,_33.fld1,_27.0,_27.0,_27.0];
_13 = (_26.0.0, Move(_15));
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(16_usize, 27_usize, Move(_27), 12_usize, Move(_12), 39_usize, Move(_39), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(16_usize, 17_usize, Move(_17), 3_usize, Move(_3), 30_usize, Move(_30), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 14_usize, Move(_14), 31_usize, Move(_31), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static f32,mut _2: isize,mut _3: (f32,),mut _4: [u16; 8],mut _5: [u16; 8]) -> [bool; 4] {
mir! {
type RET = [bool; 4];
let _6: u16;
let _7: (&'static usize, u8, [char; 7], *const Adt26);
let _8: i64;
let _9: isize;
let _10: isize;
let _11: [u16; 8];
let _12: (i128, &'static i16, &'static *mut i128, (u32,));
let _13: isize;
let _14: f32;
let _15: &'static u128;
let _16: Adt26;
let _17: ((u16,), [i128; 1], i8, &'static f32);
let _18: Adt62;
let _19: u32;
let _20: isize;
let _21: *mut i128;
let _22: Adt42;
let _23: usize;
let _24: i8;
let _25: (*const Adt26, &'static (i128, &'static i16, &'static *mut i128, (u32,)), (u8,));
let _26: ();
let _27: ();
{
_4 = [4425_u16,16530_u16,19209_u16,56846_u16,21063_u16,20834_u16,696_u16,42071_u16];
_2 = (-2_isize);
_2 = 9223372036854775807_isize | 9223372036854775807_isize;
_1 = &_3.0;
_6 = 8534_u16;
RET = [false,true,false,true];
_7.1 = 77_u8 & 144_u8;
_7.1 = 204_u8;
_2 = 228020911740837279711848839261904105257_u128 as isize;
_3.0 = 1358453920_u32 as f32;
_1 = &_3.0;
_4 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = _2 << _6;
_4 = _5;
_2 = _9 - _9;
RET = [true,false,false,false];
RET = [false,false,true,false];
_8 = 96546648346533891572859555103562673249_u128 as i64;
_6 = !46321_u16;
RET = [true,true,false,true];
match _7.1 {
204 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_8 = false as i64;
_2 = _9;
RET = [false,false,true,true];
_7.1 = 151_u8 >> _2;
_5 = _11;
Goto(bb3)
}
bb3 = {
_3.0 = 14979078310467891674_usize as f32;
_10 = _2;
_7.2 = ['\u{10d6ef}','\u{d137b}','\u{4cad2}','\u{213f3}','\u{d6888}','\u{5ceb5}','\u{109bb7}'];
_7.1 = '\u{6a6c2}' as u8;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = -_2;
_5 = _4;
_9 = -_2;
_1 = &_3.0;
RET = [false,true,true,false];
_8 = (-7512072004024534458_i64);
RET = [true,false,false,true];
_5 = [_6,_6,_6,_6,_6,_6,_6,_6];
_13 = _9;
_12.3.0 = 2168981876_u32;
_17.3 = Move(_1);
_8 = _6 as i64;
_3.0 = _12.3.0 as f32;
_16 = Adt26::Variant0 { fld0: _12.3.0,fld1: 0_usize,fld2: _9,fld3: _7.1,fld4: 125025934620140788137719303074317213887_i128 };
place!(Field::<isize>(Variant(_16, 0), 2)) = 6_usize as isize;
_17.0.0 = _6 - _6;
_9 = (-5170_i16) as isize;
Goto(bb4)
}
bb4 = {
_17.1 = [(-69099810766966353041320572864609740391_i128)];
_17.2 = 1364889553448576840_u64 as i8;
_12.0 = !1897326358325028405180954984177289082_i128;
_10 = _13 << _12.0;
_11 = [_17.0.0,_17.0.0,_6,_17.0.0,_6,_17.0.0,_17.0.0,_6];
match _12.3.0 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
2168981876 => bb11,
_ => bb10
}
}
bb5 = {
_3.0 = 14979078310467891674_usize as f32;
_10 = _2;
_7.2 = ['\u{10d6ef}','\u{d137b}','\u{4cad2}','\u{213f3}','\u{d6888}','\u{5ceb5}','\u{109bb7}'];
_7.1 = '\u{6a6c2}' as u8;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = -_2;
_5 = _4;
_9 = -_2;
_1 = &_3.0;
RET = [false,true,true,false];
_8 = (-7512072004024534458_i64);
RET = [true,false,false,true];
_5 = [_6,_6,_6,_6,_6,_6,_6,_6];
_13 = _9;
_12.3.0 = 2168981876_u32;
_17.3 = Move(_1);
_8 = _6 as i64;
_3.0 = _12.3.0 as f32;
_16 = Adt26::Variant0 { fld0: _12.3.0,fld1: 0_usize,fld2: _9,fld3: _7.1,fld4: 125025934620140788137719303074317213887_i128 };
place!(Field::<isize>(Variant(_16, 0), 2)) = 6_usize as isize;
_17.0.0 = _6 - _6;
_9 = (-5170_i16) as isize;
Goto(bb4)
}
bb6 = {
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_8 = false as i64;
_2 = _9;
RET = [false,false,true,true];
_7.1 = 151_u8 >> _2;
_5 = _11;
Goto(bb3)
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
_17.3 = &_3.0;
_1 = &_3.0;
_8 = -(-3873020082058079040_i64);
_17.0.0 = !_6;
_6 = _17.0.0 ^ _17.0.0;
_18.fld2 = (Field::<u8>(Variant(_16, 0), 3),);
_7.3 = core::ptr::addr_of!(_18.fld4.0);
_18.fld4.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_16, 0), 4)));
place!(Field::<u8>(Variant(_16, 0), 3)) = (-2054340414_i32) as u8;
_18.fld1 = _6 + _17.0.0;
_12.2 = &_18.fld4.1;
match _12.3.0 {
0 => bb1,
1 => bb10,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb9,
2168981876 => bb12,
_ => bb7
}
}
bb12 = {
_11 = [_18.fld1,_6,_17.0.0,_17.0.0,_6,_18.fld1,_18.fld1,_18.fld1];
_18.fld2.0 = _10 as u8;
_12.2 = &_18.fld4.1;
_20 = _17.2 as isize;
_14 = (*_1);
_1 = &_14;
_7.0 = &place!(Field::<usize>(Variant(_16, 0), 1));
RET = [true,false,false,false];
_3 = (_14,);
_16 = Adt26::Variant0 { fld0: _12.3.0,fld1: 2_usize,fld2: _9,fld3: _18.fld2.0,fld4: _12.0 };
_12.3 = (Field::<u32>(Variant(_16, 0), 0),);
_13 = _10 - _10;
_17.1 = [Field::<i128>(Variant(_16, 0), 4)];
_18.fld5 = [4_usize,1_usize,5_usize,10335850827220451929_usize,13254454070867332540_usize,1_usize,2577156099281030135_usize];
_6 = !_18.fld1;
_12.3 = (Field::<u32>(Variant(_16, 0), 0),);
_7.0 = &place!(Field::<usize>(Variant(_16, 0), 1));
_18.fld2.0 = '\u{10f630}' as u8;
_7.3 = core::ptr::addr_of!(_18.fld4.0);
_7.2 = ['\u{ab252}','\u{7a015}','\u{2280f}','\u{93dec}','\u{106aeb}','\u{8ad86}','\u{93130}'];
Call(place!(Field::<usize>(Variant(_16, 0), 1)) = fn18(_3.0, _12.3.0, _11, _2, _13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_7.2 = ['\u{3abce}','\u{d532b}','\u{aee22}','\u{375a5}','\u{4945c}','\u{dc2f8}','\u{cbd4c}'];
RET = [true,true,false,true];
_7.2 = ['\u{4b0b}','\u{ad5e5}','\u{7fdc}','\u{379bd}','\u{76089}','\u{754c8}','\u{c64e7}'];
_18.fld2 = (Field::<u8>(Variant(_16, 0), 3),);
_18.fld2.0 = !_7.1;
_12.2 = &_21;
_6 = !_18.fld1;
_18.fld3.0 = !_6;
SetDiscriminant(_16, 1);
_8 = -6220318350044807702_i64;
_18.fld4.1 = core::ptr::addr_of_mut!(_12.0);
_10 = _13;
_13 = -_9;
_4 = [_18.fld3.0,_6,_6,_6,_6,_18.fld1,_6,_6];
_12.2 = &_21;
_17.3 = &place!(Field::<f32>(Variant(_16, 1), 0));
_22 = Adt42::Variant0 { fld0: true,fld1: '\u{17a28}',fld2: Move(_18.fld4.1),fld3: 2_usize,fld4: _6 };
place!(Field::<f32>(Variant(_16, 1), 0)) = (*_1) + (*_1);
_17.1 = [_12.0];
place!(Field::<bool>(Variant(_22, 0), 0)) = false;
_19 = _12.3.0;
_12.3.0 = _19 % _19;
place!(Field::<char>(Variant(_22, 0), 1)) = '\u{45588}';
_17.3 = &_3.0;
_7.0 = &_23;
match _19 {
0 => bb10,
1 => bb11,
2 => bb3,
3 => bb6,
4 => bb14,
5 => bb15,
6 => bb16,
2168981876 => bb18,
_ => bb17
}
}
bb14 = {
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_8 = false as i64;
_2 = _9;
RET = [false,false,true,true];
_7.1 = 151_u8 >> _2;
_5 = _11;
Goto(bb3)
}
bb15 = {
_17.3 = &_3.0;
_1 = &_3.0;
_8 = -(-3873020082058079040_i64);
_17.0.0 = !_6;
_6 = _17.0.0 ^ _17.0.0;
_18.fld2 = (Field::<u8>(Variant(_16, 0), 3),);
_7.3 = core::ptr::addr_of!(_18.fld4.0);
_18.fld4.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_16, 0), 4)));
place!(Field::<u8>(Variant(_16, 0), 3)) = (-2054340414_i32) as u8;
_18.fld1 = _6 + _17.0.0;
_12.2 = &_18.fld4.1;
match _12.3.0 {
0 => bb1,
1 => bb10,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb9,
2168981876 => bb12,
_ => bb7
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_3.0 = (*_1);
_13 = _2 * _10;
_24 = (-917247370_i32) as i8;
place!(Field::<usize>(Variant(_22, 0), 3)) = !5372495440559343843_usize;
_25.1 = &_12;
place!(Field::<[i128; 8]>(Variant(_16, 1), 1)) = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_18.fld4.0 = Adt26::Variant1 { fld0: (*_1),fld1: Field::<[i128; 8]>(Variant(_16, 1), 1) };
_24 = !_17.2;
_7.2 = [Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1),Field::<char>(Variant(_22, 0), 1)];
_1 = &place!(Field::<f32>(Variant(_16, 1), 0));
_19 = !_12.3.0;
_17.0.0 = Field::<u16>(Variant(_22, 0), 4);
_25.2.0 = _7.1 >> Field::<u16>(Variant(_22, 0), 4);
_18.fld3 = (_17.0.0,);
_18.fld3.0 = _8 as u16;
_3 = ((*_1),);
Goto(bb19)
}
bb19 = {
Call(_26 = dump_var(17_usize, 13_usize, Move(_13), 9_usize, Move(_9), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_26 = dump_var(17_usize, 4_usize, Move(_4), 8_usize, Move(_8), 27_usize, _27, 27_usize, _27), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: f32,mut _2: u32,mut _3: [u16; 8],mut _4: isize,mut _5: isize) -> usize {
mir! {
type RET = usize;
let _6: f32;
let _7: isize;
let _8: i32;
let _9: [u32; 1];
let _10: [u32; 1];
let _11: &'static &'static usize;
let _12: i128;
let _13: bool;
let _14: char;
let _15: [i128; 1];
let _16: f32;
let _17: Adt76;
let _18: i16;
let _19: &'static ((f32,), &'static f32);
let _20: (&'static usize, u8, [char; 7], *const Adt26);
let _21: &'static &'static usize;
let _22: &'static ((f32,), &'static f32);
let _23: usize;
let _24: bool;
let _25: [i8; 5];
let _26: ();
let _27: ();
{
_5 = _4 - _4;
RET = 8148358232262019190_usize + 12978392525442916497_usize;
_6 = 18100751963484437369_u64 as f32;
_6 = _1 * _1;
_5 = (-119016202984238351251211415076913101605_i128) as isize;
_2 = !2104324407_u32;
_2 = (-162871940450892922936578366932599099730_i128) as u32;
_7 = _1 as isize;
_7 = 15184003656925388474_u64 as isize;
_5 = !_4;
_2 = 2381078020_u32 + 1900961053_u32;
_9 = [_2];
_1 = _6;
_1 = _6 - _6;
Goto(bb1)
}
bb1 = {
_5 = _7;
_6 = 96490118220814575109176779669329399904_i128 as f32;
Goto(bb2)
}
bb2 = {
_12 = 169339736922697898997180054935957182521_i128 & 73329546822057547617278795797841712893_i128;
_3 = [50225_u16,64781_u16,39904_u16,32266_u16,34499_u16,4653_u16,42878_u16,19417_u16];
_2 = 51138_u16 as u32;
_12 = 145756404789085968957713581938574549230_i128 * (-136030984236282601697635992572811085856_i128);
_2 = 3118180376362597118_i64 as u32;
_3 = [28396_u16,1107_u16,58738_u16,25131_u16,57537_u16,46058_u16,7338_u16,62142_u16];
_12 = !(-83555868301486201825298495775248817328_i128);
_7 = _4;
_8 = false as i32;
_10 = [_2];
RET = !14813750716682745274_usize;
Goto(bb3)
}
bb3 = {
RET = !3_usize;
_1 = _6 + _6;
_3 = [42680_u16,40559_u16,58775_u16,7116_u16,25678_u16,22968_u16,34235_u16,21754_u16];
_5 = _7 | _4;
Goto(bb4)
}
bb4 = {
_9 = [_2];
_7 = -_5;
_12 = (-94257394587701356980145739129427902078_i128) << _7;
RET = 4_usize;
_3 = [55266_u16,21923_u16,50945_u16,6490_u16,36941_u16,9786_u16,21629_u16,10078_u16];
_2 = !1767629543_u32;
RET = 19042_i16 as usize;
RET = 4_usize & 0_usize;
_4 = !_7;
_6 = _1 + _1;
Goto(bb5)
}
bb5 = {
_13 = _4 < _5;
RET = _12 as usize;
_4 = -_7;
Call(_3 = core::intrinsics::transmute(_12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = [7454_u16,60821_u16,29211_u16,6319_u16,62596_u16,61923_u16,14052_u16,30822_u16];
_8 = 1101600687_i32 & (-756254225_i32);
RET = 2986830075247683143_usize - 5_usize;
_6 = _1 - _1;
_1 = _6 - _6;
_13 = _1 >= _1;
_10 = _9;
RET = !6_usize;
_6 = _1;
RET = !4845887665737494939_usize;
_6 = _1 - _1;
RET = (-116_i8) as usize;
_2 = !2801249128_u32;
_14 = '\u{a9aa7}';
_15 = [_12];
_12 = _13 as i128;
_13 = _7 > _5;
_6 = -_1;
_5 = _7 & _4;
_2 = !1177009410_u32;
_1 = _6;
_5 = _4 * _4;
Goto(bb7)
}
bb7 = {
_3 = [65174_u16,32990_u16,42643_u16,1726_u16,31300_u16,3428_u16,62113_u16,6129_u16];
RET = 6_usize;
_16 = _1;
_13 = false;
_16 = _8 as f32;
_2 = _8 as u32;
_10 = _9;
_13 = true;
_17.fld1.fld3 = (_3[RET],);
_13 = !true;
_1 = -_6;
RET = 0_usize;
_15 = [_12];
_17.fld1.fld4.0 = Adt26::Variant0 { fld0: _2,fld1: RET,fld2: _7,fld3: 9_u8,fld4: _15[RET] };
_15[RET] = Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4) << Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4);
place!(Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4)) = _15[RET];
RET = Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1);
_17.fld1.fld2 = (195_u8,);
match _17.fld1.fld3.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
62113 => bb13,
_ => bb12
}
}
bb8 = {
_3 = [7454_u16,60821_u16,29211_u16,6319_u16,62596_u16,61923_u16,14052_u16,30822_u16];
_8 = 1101600687_i32 & (-756254225_i32);
RET = 2986830075247683143_usize - 5_usize;
_6 = _1 - _1;
_1 = _6 - _6;
_13 = _1 >= _1;
_10 = _9;
RET = !6_usize;
_6 = _1;
RET = !4845887665737494939_usize;
_6 = _1 - _1;
RET = (-116_i8) as usize;
_2 = !2801249128_u32;
_14 = '\u{a9aa7}';
_15 = [_12];
_12 = _13 as i128;
_13 = _7 > _5;
_6 = -_1;
_5 = _7 & _4;
_2 = !1177009410_u32;
_1 = _6;
_5 = _4 * _4;
Goto(bb7)
}
bb9 = {
_13 = _4 < _5;
RET = _12 as usize;
_4 = -_7;
Call(_3 = core::intrinsics::transmute(_12), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_9 = [_2];
_7 = -_5;
_12 = (-94257394587701356980145739129427902078_i128) << _7;
RET = 4_usize;
_3 = [55266_u16,21923_u16,50945_u16,6490_u16,36941_u16,9786_u16,21629_u16,10078_u16];
_2 = !1767629543_u32;
RET = 19042_i16 as usize;
RET = 4_usize & 0_usize;
_4 = !_7;
_6 = _1 + _1;
Goto(bb5)
}
bb11 = {
RET = !3_usize;
_1 = _6 + _6;
_3 = [42680_u16,40559_u16,58775_u16,7116_u16,25678_u16,22968_u16,34235_u16,21754_u16];
_5 = _7 | _4;
Goto(bb4)
}
bb12 = {
_12 = 169339736922697898997180054935957182521_i128 & 73329546822057547617278795797841712893_i128;
_3 = [50225_u16,64781_u16,39904_u16,32266_u16,34499_u16,4653_u16,42878_u16,19417_u16];
_2 = 51138_u16 as u32;
_12 = 145756404789085968957713581938574549230_i128 * (-136030984236282601697635992572811085856_i128);
_2 = 3118180376362597118_i64 as u32;
_3 = [28396_u16,1107_u16,58738_u16,25131_u16,57537_u16,46058_u16,7338_u16,62142_u16];
_12 = !(-83555868301486201825298495775248817328_i128);
_7 = _4;
_8 = false as i32;
_10 = [_2];
RET = !14813750716682745274_usize;
Goto(bb3)
}
bb13 = {
_17.fld1.fld3.0 = _3[RET] * _3[RET];
place!(Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4)) = !_12;
_17.fld2 = [9_i8,(-62_i8),(-107_i8),120_i8,26_i8];
place!(Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4)) = !_15[RET];
_18 = _17.fld1.fld3.0 as i16;
RET = Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1) * Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1);
RET = Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1);
_20.1 = _17.fld1.fld2.0;
_20.0 = &_17.fld1.fld5[RET];
_17.fld1.fld5 = [RET,RET,RET,RET,RET,Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1),RET];
place!(Field::<u8>(Variant(_17.fld1.fld4.0, 0), 3)) = !_17.fld1.fld2.0;
_17.fld1.fld3 = (_3[RET],);
_21 = &_20.0;
_17.fld2 = [(-82_i8),(-103_i8),24_i8,23_i8,(-2_i8)];
_17.fld1.fld5 = [RET,RET,Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1),Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1),RET,Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1)];
_17.fld1.fld4.1 = core::ptr::addr_of_mut!(_15[RET]);
_20.2[RET] = _14;
_17.fld1.fld3.0 = 12219022048474496248453001986963576755_u128 as u16;
_20.0 = &place!(Field::<usize>(Variant(_17.fld1.fld4.0, 0), 1));
_12 = _15[RET];
_10[RET] = Field::<u32>(Variant(_17.fld1.fld4.0, 0), 0) << Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4);
_17.fld1.fld5[RET] = _3[RET] as usize;
_20.1 = _17.fld1.fld2.0;
Goto(bb14)
}
bb14 = {
_12 = -_15[RET];
_8 = !839878504_i32;
_21 = &_20.0;
place!(Field::<u32>(Variant(_17.fld1.fld4.0, 0), 0)) = _13 as u32;
place!(Field::<isize>(Variant(_17.fld1.fld4.0, 0), 2)) = 289502391302547776767848663737661303461_u128 as isize;
place!(Field::<u32>(Variant(_17.fld1.fld4.0, 0), 0)) = !_10[RET];
place!(Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4)) = _12;
_3[RET] = !_17.fld1.fld3.0;
_17.fld1.fld1 = _17.fld1.fld3.0;
_15[RET] = Field::<i128>(Variant(_17.fld1.fld4.0, 0), 4);
_25 = _17.fld2;
_20.2 = [_14,_14,_14,_14,_14,_14,_14];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(18_usize, 25_usize, Move(_25), 7_usize, Move(_7), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(18_usize, 14_usize, Move(_14), 8_usize, Move(_8), 13_usize, Move(_13), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static i32,mut _2: &'static u128,mut _3: [u16; 7],mut _4: *mut u128,mut _5: ((u32,), *mut [usize; 7], [u16; 7])) -> f32 {
mir! {
type RET = f32;
let _6: isize;
let _7: f32;
let _8: isize;
let _9: (i128, &'static i16, &'static *mut i128, (u32,));
let _10: u64;
let _11: isize;
let _12: isize;
let _13: &'static f32;
let _14: *mut Adt26;
let _15: char;
let _16: [i8; 5];
let _17: isize;
let _18: [u16; 7];
let _19: (u8,);
let _20: f32;
let _21: char;
let _22: ();
let _23: ();
{
_5.2 = _3;
_3 = [14973_u16,2637_u16,47303_u16,48663_u16,45723_u16,3143_u16,53180_u16];
_5.2 = _3;
RET = 50846_u16 as f32;
RET = 11075_u16 as f32;
_5.0.0 = 2305437467_u32;
RET = 8194017979498338917_u64 as f32;
_5.2 = [58874_u16,64277_u16,15237_u16,41587_u16,24006_u16,32040_u16,23539_u16];
_5.0 = (2862862405_u32,);
RET = 9380_u16 as f32;
RET = 9223372036854775807_isize as f32;
_3 = _5.2;
RET = _5.0.0 as f32;
_5.2 = [13787_u16,3672_u16,45236_u16,44528_u16,20014_u16,8907_u16,57930_u16];
_5.0.0 = 457719282_u32;
_3 = _5.2;
_7 = (-9223372036854775808_isize) as f32;
_5.0.0 = !661032380_u32;
_7 = RET - RET;
_5.2 = _3;
_8 = 76_isize;
_5.2 = _3;
_5.0.0 = '\u{c1e75}' as u32;
_5.2 = _3;
_6 = !_8;
_5.2 = [43125_u16,5536_u16,48307_u16,8748_u16,49038_u16,61367_u16,49726_u16];
_8 = _6 >> _5.0.0;
_8 = !_6;
Goto(bb1)
}
bb1 = {
_5.0 = (1711990182_u32,);
RET = _7 * _7;
_5.0 = (1658840827_u32,);
_5.2 = [17402_u16,22447_u16,36209_u16,51195_u16,24056_u16,8315_u16,41648_u16];
_5.0.0 = 1527499533_u32;
_5.0 = (2256159249_u32,);
_9.3 = (_5.0.0,);
_3 = [64710_u16,14642_u16,37733_u16,432_u16,1177_u16,57392_u16,9046_u16];
match _5.0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2256159249 => bb10,
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
_8 = 6404377558393955593_i64 as isize;
_7 = RET - RET;
_9.3.0 = 112_u8 as u32;
_5.2 = _3;
RET = _7;
RET = _7 + _7;
_5.0.0 = (-91839572_i32) as u32;
_6 = !_8;
_7 = 203663577508733503272299798991087289211_u128 as f32;
_9.0 = (-119834907669912357407360622640227637133_i128);
_5.0.0 = _9.3.0;
_7 = RET;
_9.3 = (_5.0.0,);
_8 = _6;
_7 = (-124_i8) as f32;
_5.2 = [53901_u16,37158_u16,47262_u16,10394_u16,52094_u16,59877_u16,36046_u16];
RET = 19316_u16 as f32;
_3 = [56257_u16,14123_u16,39090_u16,2541_u16,18843_u16,25744_u16,6866_u16];
match _9.0 {
220447459251026106056013984791540574323 => bb11,
_ => bb2
}
}
bb11 = {
_5.0 = (_9.3.0,);
_10 = 9003178454541158645_u64 >> _5.0.0;
_9.3 = _5.0;
_5.0 = (_9.3.0,);
_5.2 = _3;
_11 = -_8;
_13 = &RET;
_12 = '\u{b9f48}' as isize;
_5.0 = _9.3;
_10 = _7 as u64;
_8 = 14637232035419752335_usize as isize;
_9.3 = (_5.0.0,);
_15 = '\u{e6734}';
_13 = &_7;
match _9.0 {
0 => bb7,
1 => bb6,
2 => bb3,
220447459251026106056013984791540574323 => bb12,
_ => bb4
}
}
bb12 = {
_6 = false as isize;
_13 = &_7;
_17 = _11;
_8 = _9.0 as isize;
_8 = -_11;
Goto(bb13)
}
bb13 = {
_9.3 = _5.0;
_16 = [127_i8,9_i8,(-9_i8),126_i8,(-12_i8)];
_7 = -RET;
_9.3 = (_5.0.0,);
_5.0.0 = !_9.3.0;
match _9.0 {
0 => bb12,
1 => bb14,
2 => bb15,
220447459251026106056013984791540574323 => bb17,
_ => bb16
}
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
_8 = _15 as isize;
_5.0 = (_9.3.0,);
_11 = !_6;
_16 = [(-53_i8),120_i8,(-18_i8),(-77_i8),(-45_i8)];
_18 = _3;
_5.0 = _9.3;
_15 = '\u{9ff71}';
_13 = &_7;
_15 = '\u{80b98}';
_16 = [78_i8,(-92_i8),56_i8,(-78_i8),(-22_i8)];
_10 = 12928500782971640662_usize as u64;
_9.0 = 112527948859108645205007954624469586224_i128 ^ (-76469774091946598195327958969565532146_i128);
_7 = RET;
_11 = 56_i8 as isize;
_13 = &_7;
RET = _5.0.0 as f32;
_16 = [(-97_i8),24_i8,122_i8,59_i8,76_i8];
_19.0 = !239_u8;
_19 = (246_u8,);
_11 = !_17;
Goto(bb18)
}
bb18 = {
Call(_22 = dump_var(19_usize, 16_usize, Move(_16), 19_usize, Move(_19), 6_usize, Move(_6), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_22 = dump_var(19_usize, 10_usize, Move(_10), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-93_i8)));
                
            }
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: u32,
fld1: usize,
fld2: isize,
fld3: u8,
fld4: i128,

},
Variant1{
fld0: f32,
fld1: [i128; 8],

}}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt34{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt34 {
fld0: (u8,),
fld1: *mut u128,
fld2: [usize; 7],
fld3: i8,
fld4: u128,
}
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: char,
fld2: *mut i128,
fld3: usize,
fld4: u16,

},
Variant1{
fld0: *mut (bool, f64),
fld1: Adt26,
fld2: u64,
fld3: u16,
fld4: [u16; 7],

},
Variant2{
fld0: bool,
fld1: u128,
fld2: u64,
fld3: u8,
fld4: *mut u128,
fld5: *mut i128,
fld6: usize,

},
Variant3{
fld0: [u16; 7],
fld1: i64,
fld2: u16,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f32,
fld1: *mut f32,
fld2: u16,
fld3: (u32,),
fld4: *const Adt34,

},
Variant1{
fld0: i128,
fld1: [usize; 7],

},
Variant2{
fld0: [u16; 7],
fld1: *mut u128,

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt62{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt62 {
fld0: *mut u128,
fld1: u16,
fld2: (u8,),
fld3: (u16,),
fld4: (Adt26, *mut i128),
fld5: [usize; 7],
}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt69{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt69 {
fld0: *mut [usize; 7],
fld1: [u32; 1],
fld2: (bool, f64),
}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt76{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt76 {
fld0: *mut ([u16; 7], usize),
fld1: Adt62,
fld2: [i8; 5],
fld3: *mut f64,
}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){unsafe{printf("Adt78::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
fld0: [i8; 2],
fld1: (u8,),

},
Variant1{
fld0: i32,

}}

