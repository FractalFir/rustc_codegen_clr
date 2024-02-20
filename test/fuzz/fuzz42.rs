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
pub fn fn0() -> (u64, [i128; 4], u32, f64) {
mir! {
type RET = (u64, [i128; 4], u32, f64);
let _1: f32;
let _2: f64;
let _3: ([i16; 4], f32, char);
let _4: u8;
let _5: i32;
let _6: i16;
let _7: [i32; 5];
let _8: isize;
let _9: Adt80;
let _10: isize;
let _11: f32;
let _12: isize;
let _13: u64;
let _14: [u32; 5];
let _15: *mut &'static [usize; 1];
let _16: char;
let _17: ([i128; 4],);
let _18: *const usize;
let _19: Adt64;
let _20: [u16; 1];
let _21: &'static &'static isize;
let _22: i64;
let _23: ();
let _24: ();
{
RET.3 = (-15242_i16) as f64;
RET.0 = 1359358221_i32 as u64;
RET.2 = !521101763_u32;
RET.2 = 483364160_u32;
RET.3 = 56993_u16 as f64;
RET.0 = 12170977185027853543_u64;
RET.1 = [2302694325108680670493497440284940275_i128,(-17911302610213268447439721105629750687_i128),(-97402653829106025503758663915447366213_i128),111226522886504055059028392243010949636_i128];
_1 = RET.3 as f32;
RET.3 = 1893459769_i32 as f64;
RET.0 = 13507923840277289848_u64;
_3.2 = '\u{f8de3}';
_3.1 = -_1;
match RET.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
483364160 => bb7,
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
RET.2 = 3493863619_u32 & 1623519731_u32;
_1 = _3.1;
Call(RET.2 = fn1(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET.3 = 721403118103370835_usize as f64;
RET.1 = [(-124571925090495220246689183790946038915_i128),(-148363087160056696228701647428761936066_i128),44815330341363128226408222127019934970_i128,12463679769865673541567191941811145513_i128];
_3.0 = [25148_i16,12648_i16,12382_i16,(-30520_i16)];
RET.2 = _3.2 as u32;
RET.2 = 1786287611_u32 << RET.0;
RET.2 = 4194728328_u32;
RET.1 = [43267412456467706884254305004931088379_i128,126554362356124179889365118424894880466_i128,(-121570679814695502411423792699050275180_i128),(-146055820312041471798165654363497147052_i128)];
RET.3 = 107717526332984802207375592232162502109_u128 as f64;
_3.1 = (-8779_i16) as f32;
RET.1 = [(-142561376771786925524771301683059242411_i128),(-7291600372182670623516425271859849486_i128),142947560861206625785774359228427836743_i128,64594534050997612220208255201383209014_i128];
RET.3 = 5949513240457866481_i64 as f64;
_3.1 = -_1;
_1 = _3.1;
_4 = 166_u8;
RET.0 = 9345639167849837181_u64;
_2 = 6_i8 as f64;
RET.1 = [126303530321764353083636783550636855901_i128,(-92967776622754690673453387007659499607_i128),(-81848373482661096780315026908930033010_i128),(-21354751081061906823880788220954658113_i128)];
_7 = [(-282105748_i32),871722021_i32,(-1753987832_i32),(-1290558895_i32),(-1244017197_i32)];
_3.2 = '\u{1b00e}';
RET.2 = 32165050_i32 as u32;
_6 = 26572_i16 * 31696_i16;
_4 = !163_u8;
RET.3 = _2;
_8 = -9223372036854775807_isize;
RET.2 = !2042753049_u32;
_5 = (-941805664_i32);
match _5 {
0 => bb2,
1 => bb9,
2 => bb10,
340282366920938463463374607430826405792 => bb12,
_ => bb11
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
_3.1 = _1;
_2 = -RET.3;
RET.1 = [(-113737944160343098400540680262282107428_i128),(-158622005344589292236759686173539313295_i128),148494462212666302858932049103470651141_i128,(-169464653138849885094445166606211405939_i128)];
_3.1 = _4 as f32;
_8 = !(-17_isize);
RET.3 = _2 + _2;
_12 = _8;
_2 = RET.3 + RET.3;
match RET.0 {
0 => bb7,
9345639167849837181 => bb13,
_ => bb5
}
}
bb13 = {
RET.3 = _2;
_8 = _12;
_13 = 29288_u16 as u64;
_12 = _2 as isize;
_13 = 3_usize as u64;
RET.1 = [(-113813462508822349400226219700944739131_i128),100875599301630397678943683736654452496_i128,(-115190660248610943824058869854912294814_i128),(-115831641115323706173290182568749778799_i128)];
RET.1 = [(-80772734242078861368704179043504169727_i128),(-21731972042215002517332093343328350242_i128),136663470093271251505492628242750201889_i128,(-135318950070677935661206703479979841856_i128)];
_14 = [RET.2,RET.2,RET.2,RET.2,RET.2];
_10 = _12;
_11 = _1;
_3.1 = -_11;
_5 = 297680127_i32;
match _5 {
297680127 => bb14,
_ => bb12
}
}
bb14 = {
_14 = [RET.2,RET.2,RET.2,RET.2,RET.2];
RET.1 = [7567825177912322847965261834333029563_i128,1838890045927588699986647871989012498_i128,106361942167144933255580600408698798441_i128,(-128009423255257139272006938630462211611_i128)];
_2 = RET.3 * RET.3;
RET.3 = _2;
_16 = _3.2;
_17 = (RET.1,);
RET = (_13, _17.0, 3026040331_u32, _2);
_7 = [_5,_5,_5,_5,_5];
_3.1 = 71686912558197567504156494463876100658_u128 as f32;
_2 = RET.3;
_3.0 = [_6,_6,_6,_6];
_16 = _3.2;
RET.3 = _2;
_20 = [25709_u16];
_20 = [32704_u16];
RET.3 = _2 - _2;
_13 = RET.0 + RET.0;
_16 = _3.2;
RET.1 = [(-80026608629308830769253602651398885579_i128),(-25539781355215844758395280862725640831_i128),136256349432926821595626875852679263317_i128,(-97293978067709221529081193070532911330_i128)];
_8 = _10 | _10;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(0_usize, 14_usize, Move(_14), 17_usize, Move(_17), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(0_usize, 13_usize, Move(_13), 12_usize, Move(_12), 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1() -> u32 {
mir! {
type RET = u32;
let _1: bool;
let _2: &'static i64;
let _3: u16;
let _4: char;
let _5: [u8; 1];
let _6: char;
let _7: f64;
let _8: &'static i8;
let _9: &'static ([i128; 4],);
let _10: *mut &'static [usize; 1];
let _11: [i16; 4];
let _12: ([i16; 4], f32, char);
let _13: i128;
let _14: *mut ([i32; 1],);
let _15: f32;
let _16: (*mut ([i32; 1],),);
let _17: *const [i32; 1];
let _18: [i16; 4];
let _19: u16;
let _20: [usize; 7];
let _21: char;
let _22: isize;
let _23: isize;
let _24: bool;
let _25: *const i32;
let _26: f32;
let _27: u32;
let _28: ();
let _29: ();
{
RET = 4267221339_u32;
RET = false as u32;
RET = 1839930432_u32 | 370637386_u32;
_1 = !true;
RET = !179347177_u32;
RET = (-19_i8) as u32;
_1 = RET > RET;
RET = 1174100921_u32;
RET = 3746909253_u32;
RET = 125877946_u32 + 2255305932_u32;
_1 = true;
Goto(bb1)
}
bb1 = {
RET = _1 as u32;
_1 = !false;
_3 = (-7_i8) as u16;
_3 = 5312_u16 | 51683_u16;
RET = !1409722193_u32;
_1 = false;
_3 = !64981_u16;
RET = 2083020488_u32;
RET = 1729635575_i32 as u32;
RET = 128389284_u32;
_1 = true;
_3 = 34078_u16 << RET;
RET = 51_i8 as u32;
_3 = 19246_u16;
RET = !1805867354_u32;
_3 = 15163_u16 | 44941_u16;
RET = 506410057_u32 + 3773907982_u32;
_3 = !11519_u16;
RET = 1537891166_u32 & 824925261_u32;
_3 = '\u{3041d}' as u16;
RET = 2260201300_u32;
_1 = false;
RET = 1041372563_u32 * 2968163188_u32;
_4 = '\u{4b17c}';
Goto(bb2)
}
bb2 = {
_3 = !48452_u16;
_5 = [220_u8];
_5 = [201_u8];
RET = 3819201042_u32 + 940663033_u32;
_6 = _4;
_5 = [46_u8];
_5 = [207_u8];
_6 = _4;
_3 = 88_u8 as u16;
_7 = 170_u8 as f64;
Goto(bb3)
}
bb3 = {
RET = (-128820627596610505393530008982612262706_i128) as u32;
_3 = (-59_isize) as u16;
_7 = (-128110173211951416743823972055882147821_i128) as f64;
_4 = _6;
_7 = 28_u8 as f64;
_1 = false;
_3 = !31961_u16;
_6 = _4;
_6 = _4;
RET = 3120408164_u32;
RET = !1584799153_u32;
_6 = _4;
RET = 8491663417469811667950323652834573489_i128 as u32;
_4 = _6;
_7 = _3 as f64;
Call(_3 = fn2(_7, RET, _4, _1, _6, _7, _4, RET, _5, RET), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = !32913_u16;
_4 = _6;
_4 = _6;
Goto(bb5)
}
bb5 = {
_6 = _4;
_3 = (-631245605395539231_i64) as u16;
_7 = 17941657320684666500_u64 as f64;
_6 = _4;
_3 = _6 as u16;
_1 = _7 > _7;
_3 = 56459_u16;
_4 = _6;
_3 = 20577_u16 << RET;
_6 = _4;
_3 = !31822_u16;
_1 = _6 <= _6;
_4 = _6;
_12.0 = [(-14906_i16),(-9784_i16),(-11190_i16),(-20826_i16)];
_12.2 = _6;
RET = 61358640_u32 ^ 2954265065_u32;
_7 = 165500081682639606299495429932066709441_i128 as f64;
_3 = 43162_u16;
RET = !1740966476_u32;
_5 = [87_u8];
_6 = _4;
_12.2 = _6;
_11 = _12.0;
_12.0 = _11;
RET = 2073240356_u32;
_1 = false;
_12.1 = (-2026412262629771361_i64) as f32;
_1 = _3 <= _3;
Goto(bb6)
}
bb6 = {
_4 = _12.2;
Goto(bb7)
}
bb7 = {
RET = 574323626_u32;
_4 = _6;
RET = 9191464637521307730_i64 as u32;
RET = 1911191867_i32 as u32;
_13 = (-914032131_i32) as i128;
_4 = _6;
_4 = _6;
_12.1 = _3 as f32;
_5 = [195_u8];
_13 = (-135005111871940524465977038329373696899_i128);
_12.1 = 6_u8 as f32;
Goto(bb8)
}
bb8 = {
_12.0 = [1654_i16,13847_i16,(-5386_i16),(-2706_i16)];
_3 = !12219_u16;
match _13 {
205277255048997938997397569102394514557 => bb9,
_ => bb7
}
}
bb9 = {
_12.2 = _6;
_13 = 42792160199871279502193007751352299122_i128 + (-71532514287562029491904055600558193005_i128);
RET = 1335280232_u32;
_12.2 = _4;
match RET {
0 => bb8,
1 => bb2,
2 => bb6,
1335280232 => bb11,
_ => bb10
}
}
bb10 = {
RET = _1 as u32;
_1 = !false;
_3 = (-7_i8) as u16;
_3 = 5312_u16 | 51683_u16;
RET = !1409722193_u32;
_1 = false;
_3 = !64981_u16;
RET = 2083020488_u32;
RET = 1729635575_i32 as u32;
RET = 128389284_u32;
_1 = true;
_3 = 34078_u16 << RET;
RET = 51_i8 as u32;
_3 = 19246_u16;
RET = !1805867354_u32;
_3 = 15163_u16 | 44941_u16;
RET = 506410057_u32 + 3773907982_u32;
_3 = !11519_u16;
RET = 1537891166_u32 & 824925261_u32;
_3 = '\u{3041d}' as u16;
RET = 2260201300_u32;
_1 = false;
RET = 1041372563_u32 * 2968163188_u32;
_4 = '\u{4b17c}';
Goto(bb2)
}
bb11 = {
RET = 1049545763_u32 - 1310958429_u32;
_12.1 = 36_i8 as f32;
_4 = _6;
_12.0 = [(-133_i16),(-19807_i16),30911_i16,(-3201_i16)];
_7 = (-9223372036854775808_isize) as f64;
_5 = [210_u8];
_6 = _4;
_11 = _12.0;
_13 = 67433585951865689508723829866915600477_i128 + (-28400861101116178687141750274548189076_i128);
_12.2 = _4;
_4 = _12.2;
_3 = 155987790334209941263752916487481102505_u128 as u16;
_5 = [132_u8];
RET = 1058585645_u32;
_12.0 = [(-21336_i16),15637_i16,(-26745_i16),(-14423_i16)];
_7 = (-120_i8) as f64;
_6 = _12.2;
RET = !3164549000_u32;
_12.1 = 21364_i16 as f32;
_15 = _3 as f32;
_1 = !false;
RET = 54_i8 as u32;
RET = !2051132361_u32;
_11 = _12.0;
_12 = (_11, _15, _4);
_4 = _6;
_3 = !7481_u16;
_7 = _3 as f64;
_6 = _12.2;
RET = !2857181872_u32;
Goto(bb12)
}
bb12 = {
_3 = 20160_u16 * 1266_u16;
_12.2 = _6;
_3 = !64714_u16;
_4 = _12.2;
_5 = [199_u8];
_5 = [139_u8];
RET = !3188800921_u32;
_12.2 = _6;
_13 = -(-45840606239014738424111790687142310357_i128);
_4 = _12.2;
_7 = _13 as f64;
_12 = (_11, _15, _6);
_4 = _12.2;
_1 = false ^ false;
_15 = _12.1 * _12.1;
_1 = !false;
Call(_12.0 = core::intrinsics::transmute(_11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = _12.2;
_6 = _4;
_12.1 = (-7151405099429012610_i64) as f32;
_20 = [1_usize,13614817545697687942_usize,4_usize,9696376510900846976_usize,3912392945618482834_usize,7652344291107903901_usize,3_usize];
_4 = _12.2;
_5 = [35_u8];
_5 = [231_u8];
_12.1 = _15 * _15;
Goto(bb14)
}
bb14 = {
_13 = (-409991152_i32) as i128;
_19 = _3 << RET;
_3 = !_19;
_12.1 = _15 - _15;
_18 = _11;
_12.2 = _4;
_12.1 = 7518823638434248519_i64 as f32;
_20 = [2_usize,7508783606179016460_usize,6_usize,3_usize,15995286066692139374_usize,5_usize,7_usize];
_23 = _7 as isize;
_12.1 = _15 - _15;
_24 = _1;
_20 = [13315427491746616647_usize,11957317778875276633_usize,9422303183598959551_usize,5489259076929913754_usize,3_usize,1_usize,18355925513479876964_usize];
_6 = _4;
_22 = !_23;
_11 = [26819_i16,(-8370_i16),(-2077_i16),21345_i16];
_21 = _4;
_12 = (_18, _15, _21);
_12.1 = _15;
_12 = (_11, _15, _6);
_5 = [118_u8];
_22 = 118545026665660714766163254348597194769_u128 as isize;
_1 = _24;
_13 = (-2124618088_i32) as i128;
_12 = (_11, _15, _4);
_26 = -_12.1;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(1_usize, 4_usize, Move(_4), 3_usize, Move(_3), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(1_usize, 24_usize, Move(_24), 20_usize, Move(_20), 1_usize, Move(_1), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: f64,mut _2: u32,mut _3: char,mut _4: bool,mut _5: char,mut _6: f64,mut _7: char,mut _8: u32,mut _9: [u8; 1],mut _10: u32) -> u16 {
mir! {
type RET = u16;
let _11: [i16; 3];
let _12: i32;
let _13: [i64; 3];
let _14: *const Adt77;
let _15: ([u8; 6], ([i16; 4], f32, char), (u64, [i128; 4], u32, f64), [i32; 1]);
let _16: i32;
let _17: (i64, *const u128, [u32; 5]);
let _18: f64;
let _19: u16;
let _20: bool;
let _21: char;
let _22: isize;
let _23: [bool; 3];
let _24: f64;
let _25: Adt64;
let _26: &'static &'static isize;
let _27: u8;
let _28: *const i32;
let _29: &'static i64;
let _30: [i32; 3];
let _31: [usize; 2];
let _32: f64;
let _33: &'static [i128; 4];
let _34: *mut &'static [usize; 1];
let _35: [i16; 4];
let _36: [i32; 5];
let _37: i64;
let _38: bool;
let _39: isize;
let _40: ([i16; 4], f32, char);
let _41: Adt21;
let _42: &'static i8;
let _43: (u64, [i128; 4], u32, f64);
let _44: ();
let _45: ();
{
RET = 17317_u16 & 17682_u16;
_9 = [207_u8];
_13 = [7900449343351497602_i64,3892595376125888842_i64,(-3971089084997500880_i64)];
_4 = _10 <= _10;
_9 = [51_u8];
_4 = !true;
_5 = _3;
_11 = [(-13797_i16),10489_i16,(-24917_i16)];
_9 = [93_u8];
Goto(bb1)
}
bb1 = {
RET = 40310_u16;
_3 = _7;
_1 = 212_u8 as f64;
Goto(bb2)
}
bb2 = {
_17.2 = [_8,_10,_10,_8,_8];
_13 = [(-7389413533488806692_i64),(-6968422956350818973_i64),3734569563968532006_i64];
_15.0 = [186_u8,151_u8,61_u8,48_u8,175_u8,141_u8];
_15.0 = [106_u8,242_u8,223_u8,133_u8,30_u8,50_u8];
_15.2.2 = _4 as u32;
_8 = !_10;
_15.3 = [695848535_i32];
_17.2 = [_15.2.2,_15.2.2,_2,_10,_10];
_15.2.3 = 55_isize as f64;
_5 = _3;
_15.3 = [932520383_i32];
_3 = _7;
_15.2.3 = -_1;
_18 = _15.2.3;
Call(_15.2.1 = fn3(_8, _3, _2, _8, _6, _5, RET, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15.2.2 = _2;
_9 = [241_u8];
_18 = -_6;
_15.1.2 = _3;
_9 = [78_u8];
_7 = _5;
_13 = [1908696571375901612_i64,(-1135959240001054374_i64),(-3662486507471497991_i64)];
_19 = !RET;
_16 = (-1678800334_i32);
RET = _19;
_4 = _15.2.3 > _1;
_11 = [(-2146_i16),29340_i16,(-27069_i16)];
Goto(bb4)
}
bb4 = {
_6 = _18;
_15.2.2 = _6 as u32;
_6 = _15.2.3;
_11 = [6502_i16,(-10118_i16),(-8074_i16)];
RET = _19;
_18 = RET as f64;
_17.0 = 142680152989257105_i64 & 759496799425446582_i64;
_20 = _16 < _16;
_9 = [76_u8];
_19 = 6_usize as u16;
_20 = _4;
_20 = _4;
_15.2.0 = 15778355252800723432_usize as u64;
_17.0 = 297056065290479153193797968615011655108_u128 as i64;
_15.2.1 = [(-126601063550164401710969529188618755015_i128),(-144085732988647488250599050262704334596_i128),66995775815219055985621490914510725779_i128,149768599295932219222953905954552205006_i128];
_23 = [_4,_20,_20];
RET = _17.0 as u16;
_21 = _5;
Goto(bb5)
}
bb5 = {
_5 = _3;
_12 = _16;
_23 = [_4,_4,_4];
RET = _17.0 as u16;
RET = _19 | _19;
_2 = !_15.2.2;
_17.2 = [_10,_8,_10,_2,_15.2.2];
_15.1.2 = _7;
_11 = [(-32332_i16),(-31365_i16),(-27153_i16)];
_2 = !_10;
_15.1.1 = _17.0 as f32;
_15.1.1 = 66_i8 as f32;
_9 = [215_u8];
_21 = _7;
_15.2.0 = 8768286998543350181_u64 >> RET;
_21 = _5;
_15.1.0 = [3956_i16,5857_i16,(-4595_i16),10902_i16];
Goto(bb6)
}
bb6 = {
_9 = [197_u8];
_20 = _4 & _4;
_5 = _7;
_8 = _2;
RET = _19;
RET = 22067876585041053176781093266099613712_i128 as u16;
RET = _19 | _19;
_15.0 = [104_u8,66_u8,34_u8,205_u8,252_u8,19_u8];
_15.1.0 = [8463_i16,(-7650_i16),30952_i16,20211_i16];
_15.0 = [118_u8,13_u8,209_u8,26_u8,224_u8,99_u8];
_16 = -_12;
_15.1.2 = _5;
_19 = 15_u8 as u16;
_1 = 66_u8 as f64;
_10 = _8;
_22 = (-9223372036854775808_isize) << _15.2.0;
_24 = _15.2.3 + _15.2.3;
match _12 {
0 => bb4,
340282366920938463463374607430089411122 => bb7,
_ => bb2
}
}
bb7 = {
_22 = !9223372036854775807_isize;
_29 = &_17.0;
_15.2.1 = [(-114089578256379987578256249994437071717_i128),(-46148758632766935557865051888883556195_i128),(-130057038207444769734064753610361882972_i128),(-8902780278326962542317173392681782623_i128)];
_12 = _16;
_33 = &_15.2.1;
_15.1.2 = _3;
RET = _19;
_30 = [_16,_16,_16];
_20 = _4 | _4;
_8 = _2 & _10;
_1 = -_6;
_28 = core::ptr::addr_of!(_16);
_11 = [24591_i16,(-16307_i16),(-14430_i16)];
_2 = _3 as u32;
_9 = [5_u8];
Goto(bb8)
}
bb8 = {
_15.2.2 = _10;
_30 = [_16,_16,_12];
_28 = core::ptr::addr_of!((*_28));
_30 = [_16,(*_28),_16];
_17.0 = 5962838090346698239_i64;
_12 = (*_28) - (*_28);
_32 = _24;
_5 = _21;
_15.2.1 = [131377990090890102251465076569334576605_i128,145961973446962917041597657303643677991_i128,(-110997107858131865648278029774286406306_i128),(-166163533106223927314290902861395820161_i128)];
_2 = _15.2.2 | _10;
_35 = [22132_i16,21803_i16,27587_i16,2982_i16];
_13 = [_17.0,_17.0,_17.0];
_20 = !_4;
(*_28) = _12;
_11 = [5553_i16,10348_i16,16462_i16];
_33 = &_15.2.1;
_17.0 = 7062643207283225940_i64;
_15.2.1 = [(-159177152684307762672696445159198347633_i128),(-82295678177331730453740099905628588163_i128),82903239843693021605355451257165096558_i128,64547695027437894286686242074061351004_i128];
_17.2 = [_10,_2,_10,_2,_8];
_30 = [_12,(*_28),_12];
_21 = _3;
_35 = _15.1.0;
match _17.0 {
0 => bb2,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
7062643207283225940 => bb16,
_ => bb15
}
}
bb9 = {
_22 = !9223372036854775807_isize;
_29 = &_17.0;
_15.2.1 = [(-114089578256379987578256249994437071717_i128),(-46148758632766935557865051888883556195_i128),(-130057038207444769734064753610361882972_i128),(-8902780278326962542317173392681782623_i128)];
_12 = _16;
_33 = &_15.2.1;
_15.1.2 = _3;
RET = _19;
_30 = [_16,_16,_16];
_20 = _4 | _4;
_8 = _2 & _10;
_1 = -_6;
_28 = core::ptr::addr_of!(_16);
_11 = [24591_i16,(-16307_i16),(-14430_i16)];
_2 = _3 as u32;
_9 = [5_u8];
Goto(bb8)
}
bb10 = {
_9 = [197_u8];
_20 = _4 & _4;
_5 = _7;
_8 = _2;
RET = _19;
RET = 22067876585041053176781093266099613712_i128 as u16;
RET = _19 | _19;
_15.0 = [104_u8,66_u8,34_u8,205_u8,252_u8,19_u8];
_15.1.0 = [8463_i16,(-7650_i16),30952_i16,20211_i16];
_15.0 = [118_u8,13_u8,209_u8,26_u8,224_u8,99_u8];
_16 = -_12;
_15.1.2 = _5;
_19 = 15_u8 as u16;
_1 = 66_u8 as f64;
_10 = _8;
_22 = (-9223372036854775808_isize) << _15.2.0;
_24 = _15.2.3 + _15.2.3;
match _12 {
0 => bb4,
340282366920938463463374607430089411122 => bb7,
_ => bb2
}
}
bb11 = {
_5 = _3;
_12 = _16;
_23 = [_4,_4,_4];
RET = _17.0 as u16;
RET = _19 | _19;
_2 = !_15.2.2;
_17.2 = [_10,_8,_10,_2,_15.2.2];
_15.1.2 = _7;
_11 = [(-32332_i16),(-31365_i16),(-27153_i16)];
_2 = !_10;
_15.1.1 = _17.0 as f32;
_15.1.1 = 66_i8 as f32;
_9 = [215_u8];
_21 = _7;
_15.2.0 = 8768286998543350181_u64 >> RET;
_21 = _5;
_15.1.0 = [3956_i16,5857_i16,(-4595_i16),10902_i16];
Goto(bb6)
}
bb12 = {
_6 = _18;
_15.2.2 = _6 as u32;
_6 = _15.2.3;
_11 = [6502_i16,(-10118_i16),(-8074_i16)];
RET = _19;
_18 = RET as f64;
_17.0 = 142680152989257105_i64 & 759496799425446582_i64;
_20 = _16 < _16;
_9 = [76_u8];
_19 = 6_usize as u16;
_20 = _4;
_20 = _4;
_15.2.0 = 15778355252800723432_usize as u64;
_17.0 = 297056065290479153193797968615011655108_u128 as i64;
_15.2.1 = [(-126601063550164401710969529188618755015_i128),(-144085732988647488250599050262704334596_i128),66995775815219055985621490914510725779_i128,149768599295932219222953905954552205006_i128];
_23 = [_4,_20,_20];
RET = _17.0 as u16;
_21 = _5;
Goto(bb5)
}
bb13 = {
_15.2.2 = _2;
_9 = [241_u8];
_18 = -_6;
_15.1.2 = _3;
_9 = [78_u8];
_7 = _5;
_13 = [1908696571375901612_i64,(-1135959240001054374_i64),(-3662486507471497991_i64)];
_19 = !RET;
_16 = (-1678800334_i32);
RET = _19;
_4 = _15.2.3 > _1;
_11 = [(-2146_i16),29340_i16,(-27069_i16)];
Goto(bb4)
}
bb14 = {
_17.2 = [_8,_10,_10,_8,_8];
_13 = [(-7389413533488806692_i64),(-6968422956350818973_i64),3734569563968532006_i64];
_15.0 = [186_u8,151_u8,61_u8,48_u8,175_u8,141_u8];
_15.0 = [106_u8,242_u8,223_u8,133_u8,30_u8,50_u8];
_15.2.2 = _4 as u32;
_8 = !_10;
_15.3 = [695848535_i32];
_17.2 = [_15.2.2,_15.2.2,_2,_10,_10];
_15.2.3 = 55_isize as f64;
_5 = _3;
_15.3 = [932520383_i32];
_3 = _7;
_15.2.3 = -_1;
_18 = _15.2.3;
Call(_15.2.1 = fn3(_8, _3, _2, _8, _6, _5, RET, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
RET = 40310_u16;
_3 = _7;
_1 = 212_u8 as f64;
Goto(bb2)
}
bb16 = {
_16 = _12;
_38 = _32 < _15.2.3;
_33 = &_15.2.1;
_2 = _8 << _10;
_6 = -_1;
_40.2 = _7;
_40.2 = _21;
(*_28) = _22 as i32;
_20 = !_4;
_30 = [_12,_12,(*_28)];
_13 = [_17.0,_17.0,_17.0];
_7 = _5;
_40.0 = [6949_i16,25763_i16,(-7451_i16),3497_i16];
_15.3 = [(*_28)];
_40.0 = _15.1.0;
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(2_usize, 35_usize, Move(_35), 38_usize, Move(_38), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(2_usize, 9_usize, Move(_9), 2_usize, Move(_2), 20_usize, Move(_20), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(2_usize, 10_usize, Move(_10), 12_usize, Move(_12), 45_usize, _45, 45_usize, _45), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u32,mut _2: char,mut _3: u32,mut _4: u32,mut _5: f64,mut _6: char,mut _7: u16,mut _8: [i16; 3]) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _9: *const usize;
let _10: &'static u128;
let _11: i8;
let _12: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _13: i128;
let _14: Adt77;
let _15: &'static i64;
let _16: bool;
let _17: *const i32;
let _18: isize;
let _19: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _20: [u8; 6];
let _21: [bool; 3];
let _22: &'static ([i128; 4],);
let _23: *const u32;
let _24: Adt64;
let _25: [u8; 1];
let _26: Adt64;
let _27: ();
let _28: ();
{
_7 = !47088_u16;
_6 = _2;
RET = [111528117294708555321743652107374251499_i128,28944189204755525105923927258435158537_i128,96045572636070890703490266353780225778_i128,(-92241983301158340754738388969589089088_i128)];
_8 = [(-15963_i16),18185_i16,(-3135_i16)];
_7 = 14513_u16 - 65441_u16;
_1 = _3;
_7 = !20172_u16;
RET = [157045091562639095395204042201730117736_i128,33962403159223764185867422994425715975_i128,(-43944683952377510619122504747678932225_i128),153074858272575948689669417077745737966_i128];
RET = [(-139998613264025824989099220672413654057_i128),52398056809667191848601896451605745037_i128,147804291436850369894893668485960136335_i128,(-59878591988493944573617950333071384522_i128)];
_8 = [16690_i16,(-13332_i16),(-30758_i16)];
_1 = !_3;
RET = [18967833661007185339975865675283089491_i128,(-50397524526112711582381534133879655214_i128),141816958164642398447054233991620227994_i128,(-19992928073481325059452989889140277032_i128)];
_6 = _2;
_1 = _3;
_4 = 6_usize as u32;
_7 = 34319_u16 & 18575_u16;
_4 = 226_u8 as u32;
_8 = [(-6232_i16),(-14442_i16),16523_i16];
_3 = _1 * _4;
_4 = 10257267437481882440_u64 as u32;
RET = [3024118944152313125717450886318438579_i128,(-3319170129741919432128437353142096596_i128),(-26259677969826986129780553293110471261_i128),(-65607706930365660237765064566118860613_i128)];
_5 = 10278_i16 as f64;
_3 = _1;
_7 = (-9223372036854775808_isize) as u16;
_1 = _7 as u32;
Goto(bb1)
}
bb1 = {
_3 = _4;
_2 = _6;
_7 = 8011_u16 & 36859_u16;
_2 = _6;
RET = [(-81518454818814264618270950080905376385_i128),(-78011060313204579853899376375977297624_i128),(-52727374527119049167560700288591534962_i128),15973775819899700358640807781820648944_i128];
_3 = !_1;
_7 = !14408_u16;
_3 = 11682241269435831115_usize as u32;
_12.1.2 = _4 * _3;
_13 = 5472588805296808406246274863293965691_i128 | 25846949678892576337500943573130364220_i128;
_13 = (-45721705113532776006703431298083309054_i128) - (-52093729964915737441865756672487585974_i128);
_7 = !45594_u16;
_12.3.0 = [_13,_13,_13,_13];
_12.1.3 = _5;
_12.0 = core::ptr::addr_of_mut!(_9);
_16 = !true;
_12.1.0 = _13 as u64;
_12.1 = (6341717463385568499_u64, RET, _4, _5);
RET = [_13,_13,_13,_13];
_11 = 32_i8;
_12.1.2 = !_3;
match _12.1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6341717463385568499 => bb9,
_ => bb8
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
RET = [_13,_13,_13,_13];
_12.1.1 = _12.3.0;
Goto(bb10)
}
bb10 = {
_2 = _6;
_12.3 = (RET,);
_4 = 6_usize as u32;
_6 = _2;
_18 = (-26_isize) | 9223372036854775807_isize;
_5 = (-528445361_i32) as f64;
_8 = [903_i16,26873_i16,3328_i16];
_5 = _12.1.3 - _12.1.3;
_5 = -_12.1.3;
RET = _12.3.0;
_19.1 = _12.1;
_12.1.2 = _19.1.2;
_19.0 = core::ptr::addr_of_mut!(_9);
_2 = _6;
_12.0 = core::ptr::addr_of_mut!(_9);
Call(_12.3 = fn4(_1, _13, _12.1.1, _12.1.0, _4, _19.1.0, _19.1.0, _19.1.0, _13, _5, _19.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = true;
_12.3 = (_12.1.1,);
_5 = _12.1.3 - _19.1.3;
RET = [_13,_13,_13,_13];
_20 = [139_u8,5_u8,130_u8,161_u8,243_u8,88_u8];
_2 = _6;
_19.1.2 = _12.1.0 as u32;
_20 = [74_u8,238_u8,247_u8,87_u8,253_u8,95_u8];
_12.1.1 = RET;
_13 = 44108766865279745181402058440582705861_i128;
_11 = _2 as i8;
_19.0 = core::ptr::addr_of_mut!(_9);
match _19.1.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb8,
6341717463385568499 => bb13,
_ => bb12
}
}
bb12 = {
_2 = _6;
_12.3 = (RET,);
_4 = 6_usize as u32;
_6 = _2;
_18 = (-26_isize) | 9223372036854775807_isize;
_5 = (-528445361_i32) as f64;
_8 = [903_i16,26873_i16,3328_i16];
_5 = _12.1.3 - _12.1.3;
_5 = -_12.1.3;
RET = _12.3.0;
_19.1 = _12.1;
_12.1.2 = _19.1.2;
_19.0 = core::ptr::addr_of_mut!(_9);
_2 = _6;
_12.0 = core::ptr::addr_of_mut!(_9);
Call(_12.3 = fn4(_1, _13, _12.1.1, _12.1.0, _4, _19.1.0, _19.1.0, _19.1.0, _13, _5, _19.1), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_16 = !false;
_23 = core::ptr::addr_of!(_1);
_19.3 = (_12.1.1,);
_12.2.0 = [1631055625_i32];
_19.2.0 = [(-1737327111_i32)];
_22 = &_12.3;
match _12.1.0 {
0 => bb1,
1 => bb11,
2 => bb10,
3 => bb5,
6341717463385568499 => bb15,
_ => bb14
}
}
bb14 = {
_2 = _6;
_12.3 = (RET,);
_4 = 6_usize as u32;
_6 = _2;
_18 = (-26_isize) | 9223372036854775807_isize;
_5 = (-528445361_i32) as f64;
_8 = [903_i16,26873_i16,3328_i16];
_5 = _12.1.3 - _12.1.3;
_5 = -_12.1.3;
RET = _12.3.0;
_19.1 = _12.1;
_12.1.2 = _19.1.2;
_19.0 = core::ptr::addr_of_mut!(_9);
_2 = _6;
_12.0 = core::ptr::addr_of_mut!(_9);
Call(_12.3 = fn4(_1, _13, _12.1.1, _12.1.0, _4, _19.1.0, _19.1.0, _19.1.0, _13, _5, _19.1), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_12.2.0 = [1005494566_i32];
_8 = [(-24502_i16),(-6524_i16),(-4261_i16)];
_19.3 = (_19.1.1,);
_19.3 = (RET,);
_19.0 = core::ptr::addr_of_mut!(_9);
_7 = _12.1.3 as u16;
_19.1.2 = !(*_23);
_12.1.2 = _3 & _1;
_3 = (*_23);
_5 = (-22543_i16) as f64;
_13 = 5765329965585342689_usize as i128;
RET = _19.3.0;
_19.2.0 = [309104100_i32];
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(3_usize, 18_usize, Move(_18), 3_usize, Move(_3), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(3_usize, 2_usize, Move(_2), 13_usize, Move(_13), 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u32,mut _2: i128,mut _3: [i128; 4],mut _4: u64,mut _5: u32,mut _6: u64,mut _7: u64,mut _8: u64,mut _9: i128,mut _10: f64,mut _11: (u64, [i128; 4], u32, f64)) -> ([i128; 4],) {
mir! {
type RET = ([i128; 4],);
let _12: isize;
let _13: char;
let _14: i128;
let _15: i8;
let _16: [i16; 3];
let _17: bool;
let _18: &'static f64;
let _19: *const u32;
let _20: isize;
let _21: ([u8; 6], ([i16; 4], f32, char), (u64, [i128; 4], u32, f64), [i32; 1]);
let _22: char;
let _23: char;
let _24: u16;
let _25: isize;
let _26: [i16; 4];
let _27: Adt40;
let _28: *const usize;
let _29: ();
let _30: ();
{
_11.1 = _3;
RET = (_11.1,);
_10 = _11.3;
_2 = -_9;
_11 = (_8, RET.0, _1, _10);
_3 = [_2,_2,_9,_2];
Call(_9 = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = [_2,_9,_2,_9];
_16 = [(-23189_i16),13080_i16,(-25486_i16)];
RET = (_3,);
_13 = '\u{104979}';
_3 = [_9,_9,_9,_2];
_11 = (_6, _3, _1, _10);
RET = (_3,);
_18 = &_11.3;
_9 = _2 << _4;
_4 = _6 + _8;
_2 = _9;
RET.0 = [_9,_9,_9,_2];
RET.0 = [_9,_2,_2,_2];
Goto(bb2)
}
bb2 = {
_1 = (-9223372036854775808_isize) as u32;
_11 = (_4, _3, _5, _10);
RET = (_3,);
_15 = (-48_i8);
_11.3 = 47806_u16 as f64;
RET = (_3,);
_12 = !(-9223372036854775808_isize);
_17 = true;
_15 = !93_i8;
_11.1 = [_9,_9,_9,_9];
Call(RET = fn5(_11.0, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = 1137329529_i32 as isize;
_18 = &_10;
_7 = _11.0;
_21.1.1 = (-28831_i16) as f32;
_21.1.2 = _13;
_21.0 = [187_u8,42_u8,47_u8,217_u8,234_u8,163_u8];
_21.2.2 = !_5;
_21.1.1 = _11.2 as f32;
_21.2 = (_6, _11.1, _11.2, (*_18));
_22 = _21.1.2;
_11.1 = _21.2.1;
_22 = _21.1.2;
_21.1.2 = _22;
_21.0 = [211_u8,56_u8,163_u8,152_u8,111_u8,243_u8];
Goto(bb4)
}
bb4 = {
_8 = _21.2.0 & _4;
_5 = _21.2.2;
_9 = _2 >> _7;
_11.0 = _8;
_11.0 = _9 as u64;
RET = (_21.2.1,);
_23 = _13;
_11.3 = -(*_18);
_21.3 = [737921425_i32];
_21.2.2 = _1;
_12 = (-3_isize) - (-14_isize);
RET.0 = [_9,_9,_9,_2];
_21.1.1 = 62409_u16 as f32;
_7 = 115433147598914219288783554746450356427_u128 as u64;
_6 = _11.0 << _2;
_14 = _9 * _9;
_1 = _21.2.2 << _7;
_25 = _12 ^ _12;
_21.1.1 = 185_u8 as f32;
_5 = _21.2.2;
_8 = _11.0;
_21.1.1 = _14 as f32;
_4 = _5 as u64;
_22 = _23;
Goto(bb5)
}
bb5 = {
Call(_29 = dump_var(4_usize, 5_usize, Move(_5), 12_usize, Move(_12), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_29 = dump_var(4_usize, 25_usize, Move(_25), 17_usize, Move(_17), 22_usize, Move(_22), 14_usize, Move(_14)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_29 = dump_var(4_usize, 4_usize, Move(_4), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u64,mut _2: u64) -> ([i128; 4],) {
mir! {
type RET = ([i128; 4],);
let _3: &'static [usize; 1];
let _4: [i32; 3];
let _5: [bool; 5];
let _6: *mut *const usize;
let _7: &'static u16;
let _8: Adt40;
let _9: char;
let _10: Adt80;
let _11: [u128; 5];
let _12: bool;
let _13: *mut *const usize;
let _14: u32;
let _15: i16;
let _16: i8;
let _17: [i64; 5];
let _18: i64;
let _19: *mut ([i32; 1],);
let _20: Adt32;
let _21: i128;
let _22: f32;
let _23: *const i32;
let _24: *mut [usize; 1];
let _25: isize;
let _26: f64;
let _27: &'static u16;
let _28: bool;
let _29: isize;
let _30: &'static i64;
let _31: [i8; 3];
let _32: isize;
let _33: bool;
let _34: *mut usize;
let _35: *const i32;
let _36: ();
let _37: ();
{
_2 = 11447630679158681441_usize as u64;
_2 = !_1;
RET.0 = [45384065337800138813022347755712452103_i128,(-102376127426930971740570757911346812046_i128),(-112531610759895820861093483150241765744_i128),32920068292587631905608133049176284392_i128];
RET.0 = [(-160404791070758726395126903725077999955_i128),(-48889793740303320855399630239102709183_i128),(-165949295462446426126061399105045765743_i128),(-94598039904996068404532084324709388510_i128)];
RET.0 = [5961069487382041205765808621757817821_i128,5775191950188326632926515034309149515_i128,170111934637990329108367469644355395144_i128,51544393313516607664134769902382757692_i128];
RET.0 = [(-138587543049007840802583169031228757589_i128),162807608305301932002416369553858777576_i128,6062724570673137836160309903334669338_i128,125849378843896841501696320933755501131_i128];
_4 = [1370544115_i32,(-704047802_i32),2063628881_i32];
RET.0 = [134906199347110584979771544156484792073_i128,(-91332704089916503385824147671444572127_i128),52087139410244759950617271143395179715_i128,(-148109430821103989761092797112643189383_i128)];
_2 = _1 & _1;
_4 = [(-241194708_i32),(-647872394_i32),1769525711_i32];
_5 = [false,false,false,false,true];
_4 = [16581851_i32,(-731449885_i32),1373947675_i32];
_5 = [false,false,false,false,false];
Goto(bb1)
}
bb1 = {
RET.0 = [76152049860431222856129947076016485001_i128,(-46900315134718200821486339882921381713_i128),(-24427610853751568637685604103078400141_i128),(-17526282323486478882625229655331461458_i128)];
RET.0 = [(-52958351690140963315328291805008058693_i128),152922595193264461531789915976571080506_i128,30841474271128436462156374108925818831_i128,29667991977681692848072503359965575455_i128];
RET.0 = [(-138751230183288896267118621551812397724_i128),73901297897318536826353927618380318543_i128,(-62597649971056995905646532581846085790_i128),162599862417080788452122286463389590982_i128];
RET.0 = [(-111379260584951155331197575162196538469_i128),135286145718126552483300711657836014711_i128,(-141731341578354066905673833569876582097_i128),(-129287692310043060203510560835955086401_i128)];
_1 = _2;
RET.0 = [(-87356050637179980487397815124334084448_i128),114826090651806427616945423794051676539_i128,(-62586981573682271423939581074302397092_i128),85209333214300967519295105409531362074_i128];
_5 = [false,false,true,true,false];
RET.0 = [107361973900104699007415759433209301200_i128,(-68485212494012478649081401839189824400_i128),(-164347375722223466705136996646474228824_i128),126714226739624711306167789555894087286_i128];
RET.0 = [84009065513369052385660784438836630346_i128,82471202721532201514415295683523158145_i128,163656976169121722879354544804402647616_i128,(-153104450932897679781832014245763300771_i128)];
_5 = [false,true,true,true,false];
_5 = [true,false,false,false,true];
RET.0 = [96714533230574381221313502886721136720_i128,(-108302404928313449922487970214636276201_i128),(-38375891004063806046585866790426357551_i128),141351466935476674236732479397853362997_i128];
RET.0 = [(-48317029896750207169786199986047038095_i128),12097251155167349210209812714476322112_i128,(-38692347739775576426757495745334138608_i128),105520434625668001077105003748349604347_i128];
_4 = [(-660071596_i32),(-2000334447_i32),1545644302_i32];
_4 = [(-2003501113_i32),(-2048530897_i32),808356236_i32];
_2 = _1;
RET.0 = [(-61710432409304276549478758766545139157_i128),(-76323532759651905269089406419767594552_i128),154807087278162928723746549494164639970_i128,114434684995824027818234237387836257545_i128];
_4 = [(-1857142418_i32),1511713713_i32,774473663_i32];
_2 = !_1;
_5 = [true,true,false,false,false];
RET.0 = [19853762925186059564538755445870477877_i128,(-156624601865580989211294198211365667348_i128),(-60304031181860238663702981386538960547_i128),(-94152314893653437163680586256990565016_i128)];
_2 = 7_usize as u64;
_1 = !_2;
_5 = [true,true,false,true,false];
Call(_1 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = true as u64;
_1 = !_2;
_2 = 242144910_i32 as u64;
_4 = [(-117766872_i32),1190619601_i32,(-58379566_i32)];
_4 = [1827446856_i32,874236560_i32,646854778_i32];
_5 = [false,false,true,false,true];
RET.0 = [100630241006453609991795132097728451819_i128,57608852114170903039542049081871862189_i128,148001772695286354687611618296534317444_i128,148014117917175820551374925872939828915_i128];
_1 = 251805965927412619503554411693444100155_u128 as u64;
_4 = [(-1098925344_i32),(-1783237607_i32),1945898061_i32];
RET.0 = [(-26660909439661563621273804834868949960_i128),94832073855481579601915991343211607407_i128,98454214930182082031032634885311167109_i128,(-117171585805087971690469253536073909493_i128)];
_1 = _2;
_9 = '\u{83487}';
_4 = [133229432_i32,1487817339_i32,854914980_i32];
RET.0 = [71890748472938845187252734222738346701_i128,43527345547609960561950835546307917187_i128,(-62555388692594290830168456669421320666_i128),(-139057738945085454478546750947901282810_i128)];
Goto(bb3)
}
bb3 = {
RET.0 = [(-134769749708991666530793371962207764922_i128),(-90956959281955616280816884821655421353_i128),158609638922706048285294505828356211784_i128,47866692357940055267102360742630396488_i128];
_9 = '\u{10ce72}';
_2 = !_1;
_5 = [false,false,false,false,true];
_4 = [1379028651_i32,1592137916_i32,(-1457646724_i32)];
_2 = _1 | _1;
RET.0 = [(-51459777192168507447607306075097339499_i128),(-132140594170423656316270088036892900079_i128),132755577049806866590736205445937065225_i128,(-36674426645934809776728983906128831052_i128)];
_4 = [(-2056441503_i32),1115747203_i32,505988070_i32];
_5 = [false,false,false,false,true];
RET.0 = [3559174408041163639472230819037978679_i128,(-89547531791854328662928201291285201844_i128),(-152656635392570833232655217794593314272_i128),134926933634400837782752868299163338685_i128];
RET.0 = [24489061418140570173409783054780267321_i128,169550473523908952014553918124100763973_i128,129609251061176102268421511899991397046_i128,(-58213015709900836177834485544662437720_i128)];
_5 = [true,false,false,false,false];
_9 = '\u{329c9}';
RET.0 = [(-75565638780172668311179434496915829075_i128),(-164921718641050903596581373597591227783_i128),(-109929299470197994157181495349692323641_i128),147869016269386762718027951623799243000_i128];
_2 = _1 >> _1;
RET.0 = [(-39306303856626215902697258181161057231_i128),(-72364728680125721503239804167609681120_i128),33218823886742618861791245876376935475_i128,37141930531626505931221477082628610043_i128];
_5 = [false,true,true,true,false];
_4 = [1962992512_i32,(-540509475_i32),201441361_i32];
_9 = '\u{47757}';
_4 = [(-1786401172_i32),1763842079_i32,1525217613_i32];
_5 = [false,true,true,false,true];
_9 = '\u{820de}';
RET.0 = [115915557713353405130681919877624371966_i128,(-131959249704421705361710553770454644006_i128),(-62243928996571322569431092720525181207_i128),153317188549759494648482697391010512732_i128];
RET.0 = [36313067049299147286961815263282105278_i128,155481327104392124662939358254799621765_i128,(-116816032215187469538639419553687262_i128),(-2983703185257554147407781507679015068_i128)];
_11 = [229960960510238282754602100619072903974_u128,120364139152600048094839576943327488002_u128,8605237237650492153493822150091165747_u128,198180340402403779545567236356874558750_u128,45470205671989229169526250462018252929_u128];
_1 = !_2;
Goto(bb4)
}
bb4 = {
_11 = [33118937850188521822044541986553548270_u128,299194696139519113285605895970935780959_u128,77542074776017521403158371517101451522_u128,174123869217766120935736047401941130570_u128,317307745650260903958486505308097713639_u128];
_9 = '\u{cff19}';
RET.0 = [100413504241245168998577683434887945204_i128,(-124443952736140600707397212525634545575_i128),(-15612383596459113270105778915014195690_i128),(-95137542204160690926787533838362965865_i128)];
_1 = 5674_i16 as u64;
Call(_6 = fn6(_5, _5, _9, _2, _11, RET), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = [1989757811_i32,(-1330217912_i32),(-1717230391_i32)];
_13 = Move(_6);
_12 = !true;
_11 = [56277798883489272191615682453274554052_u128,65426464293030049300771301495083962905_u128,54745417872129181246427682121174363938_u128,34135294423042567311983295040355317847_u128,86697394899322729312590311211417248551_u128];
_1 = _2 * _2;
_5 = [_12,_12,_12,_12,_12];
_1 = _2 << _2;
_14 = 4033564249_u32 - 2723061210_u32;
RET.0 = [115045826135687346846811003322861329257_i128,(-109113519993958949078504448680779005151_i128),71930774345369384053616817453375134965_i128,(-145932730843049351598604102766741338140_i128)];
_9 = '\u{c6a15}';
_4 = [1450388315_i32,(-478809775_i32),(-1730591322_i32)];
_12 = _14 > _14;
_9 = '\u{9fcec}';
_6 = Move(_13);
_13 = Move(_6);
_6 = Move(_13);
RET.0 = [(-167200988472429710956547721815891149921_i128),(-15639622996433296261533419322130503082_i128),(-39445407187198504449324880396139957745_i128),(-18701716692704219881420214571478197133_i128)];
_17 = [(-2222987007769088476_i64),6706811336065407825_i64,(-5812382559116482619_i64),(-3529465203048038432_i64),2501067986324916410_i64];
Goto(bb6)
}
bb6 = {
_20 = Adt32::Variant1 { fld0: 38498_u16,fld1: _9,fld2: (-313916360_i32) };
_15 = (-12048_i16) | 9311_i16;
_20 = Adt32::Variant1 { fld0: 54892_u16,fld1: _9,fld2: 364345978_i32 };
_14 = _1 as u32;
place!(Field::<u16>(Variant(_20, 1), 0)) = !27689_u16;
_18 = 4707772500273059404_i64;
_1 = (-164070780158410612462064704377447641176_i128) as u64;
place!(Field::<i32>(Variant(_20, 1), 2)) = 9223372036854775807_isize as i32;
_9 = Field::<char>(Variant(_20, 1), 1);
Goto(bb7)
}
bb7 = {
_13 = Move(_6);
_16 = (-31_i8) | (-59_i8);
Goto(bb8)
}
bb8 = {
_14 = 207761953_u32;
place!(Field::<i32>(Variant(_20, 1), 2)) = (-1857830283_i32);
_4 = [Field::<i32>(Variant(_20, 1), 2),Field::<i32>(Variant(_20, 1), 2),Field::<i32>(Variant(_20, 1), 2)];
_22 = _15 as f32;
_2 = !_1;
_21 = 3_usize as i128;
_1 = _2;
_9 = Field::<char>(Variant(_20, 1), 1);
place!(Field::<char>(Variant(_20, 1), 1)) = _9;
_23 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_20, 1), 2)));
_6 = Move(_13);
_5 = [_12,_12,_12,_12,_12];
_16 = 38_i8 & 27_i8;
place!(Field::<i32>(Variant(_20, 1), 2)) = 838621156_i32 ^ 119386367_i32;
_17 = [_18,_18,_18,_18,_18];
_4 = [(*_23),(*_23),(*_23)];
_22 = _21 as f32;
_23 = core::ptr::addr_of!((*_23));
(*_23) = !(-1131136980_i32);
Goto(bb9)
}
bb9 = {
_7 = &place!(Field::<u16>(Variant(_20, 1), 0));
_11 = [168624902494096947920417154543296447610_u128,196871496483711707023358215013784913578_u128,255241741031236274106631318640102189407_u128,122262713045554739650228136891739120354_u128,338465593891544519936528520140081485103_u128];
_13 = Move(_6);
_25 = 9223372036854775807_isize << Field::<i32>(Variant(_20, 1), 2);
_1 = !_2;
_18 = (-3785612301722385555_i64);
place!(Field::<i32>(Variant(_20, 1), 2)) = (-977515071_i32);
Goto(bb10)
}
bb10 = {
(*_23) = -1637839731_i32;
_11 = [35493023290480295202893904651405270871_u128,129279616822985112948673901586152626356_u128,53862315101945279687529880703267275062_u128,271728238854981862059016273366285106010_u128,297131294009100582368214104666860272396_u128];
_9 = Field::<char>(Variant(_20, 1), 1);
(*_23) = _12 as i32;
_6 = Move(_13);
_21 = (-39243244441604868147477667260305634700_i128) | 78035986714843849790699446852985748570_i128;
_18 = 5260777123040016187_i64 ^ (-5686901777458184939_i64);
RET.0 = [_21,_21,_21,_21];
_5 = [_12,_12,_12,_12,_12];
SetDiscriminant(_20, 3);
_21 = (-128323270503338745736844223855811867103_i128) & 57024774444984749754095919783019924254_i128;
_13 = Move(_6);
_18 = _16 as i64;
_29 = _25 + _25;
place!(Field::<char>(Variant(_20, 3), 1)) = _9;
_1 = _2 << _18;
_21 = (-162086475446942144931076051626434093898_i128) | (-118570969266122590326307184888568227884_i128);
place!(Field::<[i32; 1]>(Variant(_20, 3), 2)) = [1847709468_i32];
_22 = _1 as f32;
_20 = Adt32::Variant2 { fld0: _12,fld1: _14 };
_9 = '\u{a03d9}';
_30 = &_18;
_5 = [Field::<bool>(Variant(_20, 2), 0),_12,Field::<bool>(Variant(_20, 2), 0),_12,Field::<bool>(Variant(_20, 2), 0)];
_14 = Field::<u32>(Variant(_20, 2), 1) & Field::<u32>(Variant(_20, 2), 1);
_2 = _1 + _1;
Goto(bb11)
}
bb11 = {
_15 = _22 as i16;
_6 = Move(_13);
_5 = [Field::<bool>(Variant(_20, 2), 0),Field::<bool>(Variant(_20, 2), 0),Field::<bool>(Variant(_20, 2), 0),_12,Field::<bool>(Variant(_20, 2), 0)];
_22 = _21 as f32;
_2 = _1;
_13 = Move(_6);
_15 = _2 as i16;
_2 = _1 - _1;
_12 = _18 <= _18;
Goto(bb12)
}
bb12 = {
_18 = !(-8170003017063565210_i64);
_32 = _25 ^ _25;
_28 = _32 != _29;
_12 = Field::<bool>(Variant(_20, 2), 0) | _28;
match Field::<u32>(Variant(_20, 2), 1) {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb10,
5 => bb6,
6 => bb7,
207761953 => bb13,
_ => bb8
}
}
bb13 = {
_30 = &_18;
Goto(bb14)
}
bb14 = {
_32 = _29 | _25;
_33 = _12;
_1 = 37197_u16 as u64;
_26 = _2 as f64;
_5 = [_12,Field::<bool>(Variant(_20, 2), 0),_28,_12,_28];
_2 = 9644_u16 as u64;
_33 = !_12;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(5_usize, 17_usize, Move(_17), 4_usize, Move(_4), 32_usize, Move(_32), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(5_usize, 21_usize, Move(_21), 12_usize, Move(_12), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(5_usize, 28_usize, Move(_28), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [bool; 5],mut _2: [bool; 5],mut _3: char,mut _4: u64,mut _5: [u128; 5],mut _6: ([i128; 4],)) -> *mut *const usize {
mir! {
type RET = *mut *const usize;
let _7: *const u32;
let _8: f64;
let _9: u8;
let _10: f64;
let _11: isize;
let _12: ();
let _13: ();
{
_3 = '\u{a9024}';
_6.0 = [(-3188094332480183456542748807041794261_i128),69221202296723830240282447492077718252_i128,(-169717328590570055408434996420232556203_i128),48378810743555657044593785677667363905_i128];
_2 = [false,false,true,true,true];
_1 = [false,true,true,false,false];
_1 = _2;
_4 = 73975910499915227606612362795496346580_u128 as u64;
_1 = [false,true,true,true,false];
_5 = [313991605418898711620837231239112189242_u128,225559231824260358137225265840326010573_u128,63468496565362192370550534064290985460_u128,179700768418085621195287127044776548169_u128,258204585586092916118598543875975128399_u128];
_3 = '\u{10305a}';
_1 = _2;
_6.0 = [(-78983858542097537096468410503942448686_i128),(-72964385165016976971603133824214749746_i128),118129835019608077024523635616319643400_i128,56865467184107437685278434656092256224_i128];
_5 = [52237894362162137141140013399255222357_u128,12327051247486951601916950683497724416_u128,167923498741725362105271532326782295271_u128,285630801070730535574889365951990620420_u128,329035709522199019406853522935111107561_u128];
_6.0 = [(-165420166824260176484260637394255580348_i128),45245150082846740324381286518294639063_i128,112343103334038505988113050520887599276_i128,136383768383419767872106220669729803561_i128];
_8 = 893761950_u32 as f64;
_3 = '\u{69f03}';
_9 = !234_u8;
_9 = (-44928652357676568967614373438335432364_i128) as u8;
_9 = 165106936393919036145236203838595175680_i128 as u8;
_10 = _8 + _8;
_6.0 = [(-30762519534506663154277231622388530312_i128),119446873987628964239316091128405628224_i128,(-13858825275587990067417749301488862551_i128),79860501206449472521100503941267337283_i128];
_3 = '\u{5134f}';
_1 = [false,false,false,false,false];
_3 = '\u{3f13c}';
Goto(bb1)
}
bb1 = {
_4 = (-520512133_i32) as u64;
_8 = _10;
_6.0 = [109488847160494942089604095086486884756_i128,8556151384260480386300530050130929243_i128,107494942465265731893689635303723021852_i128,54343103302420594420665230021045438823_i128];
Call(RET = fn7(_9, _1, _8, _1, _3, _1, _6, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = (-1146114312_i32) as f64;
_6.0 = [(-144143015735585682696028780091553460128_i128),(-60558419579643956619849393930894066987_i128),(-54483417592518886270027129049779993669_i128),(-109084101659926489824438992547166460070_i128)];
_6.0 = [(-65912723247703847229188292563283028420_i128),(-91244644862274840632999590601828842804_i128),126874646816234206010182941254845846660_i128,127820152240694510966276509803344156120_i128];
_8 = _10 * _10;
_3 = '\u{35aaf}';
_6.0 = [(-44363018416739469705337386978899290038_i128),(-143049537443747378906567123311953551999_i128),(-54016961493654660410637558636783765069_i128),133181557285390321590548693365787272193_i128];
_4 = _10 as u64;
_1 = _2;
_2 = _1;
_4 = 15654057636035010073_u64 - 7741124911531861696_u64;
_4 = 4659213992063090791_u64;
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(6_usize, 3_usize, Move(_3), 5_usize, Move(_5), 1_usize, Move(_1), 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u8,mut _2: [bool; 5],mut _3: f64,mut _4: [bool; 5],mut _5: char,mut _6: [bool; 5],mut _7: ([i128; 4],),mut _8: [u128; 5]) -> *mut *const usize {
mir! {
type RET = *mut *const usize;
let _9: &'static u128;
let _10: Adt77;
let _11: f32;
let _12: *const u32;
let _13: &'static &'static isize;
let _14: isize;
let _15: *const u32;
let _16: u32;
let _17: &'static u128;
let _18: [u8; 6];
let _19: &'static f64;
let _20: &'static ([i128; 4],);
let _21: bool;
let _22: i32;
let _23: Adt77;
let _24: isize;
let _25: [u8; 1];
let _26: ([i16; 4], f32, char);
let _27: bool;
let _28: &'static [usize; 1];
let _29: isize;
let _30: *const i32;
let _31: f32;
let _32: ([i32; 1],);
let _33: bool;
let _34: f64;
let _35: &'static u128;
let _36: Adt43;
let _37: [i8; 3];
let _38: [u8; 1];
let _39: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _40: *const Adt77;
let _41: f32;
let _42: u128;
let _43: [isize; 5];
let _44: bool;
let _45: i8;
let _46: &'static u128;
let _47: Adt40;
let _48: [usize; 2];
let _49: f64;
let _50: f32;
let _51: bool;
let _52: &'static char;
let _53: u128;
let _54: [i16; 4];
let _55: isize;
let _56: *const [i32; 1];
let _57: &'static i64;
let _58: f64;
let _59: [u16; 8];
let _60: i32;
let _61: bool;
let _62: *const [i32; 1];
let _63: [i16; 5];
let _64: Adt40;
let _65: bool;
let _66: [i16; 4];
let _67: &'static i64;
let _68: *const u128;
let _69: i128;
let _70: isize;
let _71: i16;
let _72: [i8; 2];
let _73: *const usize;
let _74: i128;
let _75: bool;
let _76: &'static [i128; 4];
let _77: isize;
let _78: isize;
let _79: &'static [i128; 4];
let _80: [isize; 5];
let _81: *const u32;
let _82: f32;
let _83: ();
let _84: ();
{
_1 = 118_u8;
_4 = _6;
_3 = 39062_u16 as f64;
_5 = '\u{b54d4}';
_1 = 37_u8 - 92_u8;
_1 = 63_u8 ^ 3_u8;
_7.0 = [(-74003473305038966896390981033777256862_i128),(-109625361821815540351165346558658228499_i128),(-107262591498026114216959185779164518477_i128),(-138987524697319990832720163967633866279_i128)];
_1 = _5 as u8;
_4 = _2;
_5 = '\u{2974f}';
_7.0 = [(-253876365494713920030161222920927311_i128),102913139902492791902701926503574119796_i128,(-44574294112028796680474423082394202402_i128),48311760088434344199385668399239150021_i128];
_1 = false as u8;
_8 = [254337103151795513923461674193555741785_u128,195670388460637599036702264107810888659_u128,68990175574812234992457022887292595303_u128,9955397271153397380010069586704311981_u128,340183872461969045224665013492329871432_u128];
_11 = _3 as f32;
_4 = _6;
_2 = [true,true,true,true,true];
_7.0 = [53196515489813694222322165688192454944_i128,44511478955184408593022010708667206041_i128,79794879549467834569056512120660833017_i128,167062999194961835050087715923764608199_i128];
_2 = [true,true,true,true,false];
_4 = _2;
_3 = (-9223372036854775808_isize) as f64;
_8 = [302352779257148112583021070960280863279_u128,102904832849117100106338955800562351243_u128,147473737087564514117155450168434657682_u128,100122192703402392993582846915597084476_u128,56720264519754458538086864115264154294_u128];
_11 = 13187287073256789227_usize as f32;
_2 = [true,false,false,false,true];
Goto(bb1)
}
bb1 = {
_1 = 60_u8;
_2 = _6;
_14 = !(-9223372036854775808_isize);
_3 = 4454243984223214387_u64 as f64;
_14 = !(-9223372036854775808_isize);
_4 = [true,true,false,true,false];
_4 = _2;
_1 = 166_u8;
_5 = '\u{9ab3a}';
_2 = [false,false,true,false,true];
_11 = 16940793306327671780_u64 as f32;
_5 = '\u{d857e}';
_1 = !253_u8;
_8 = [32581655619946289568936222653223004751_u128,338444785173222414943513887870484954520_u128,246024033675498417557245320188251133979_u128,338683642303583460150777048413392922242_u128,311549134336732124792017367878450950432_u128];
_11 = _3 as f32;
_4 = _2;
_5 = '\u{55c50}';
_6 = _4;
_14 = (-9223372036854775808_isize);
_12 = core::ptr::addr_of!(_16);
_2 = _4;
_3 = 6169415406621397326_usize as f64;
_6 = [false,true,true,false,false];
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_16 = 1281081954_u32 & 3921942002_u32;
_14 = (-9223372036854775808_isize);
_6 = _4;
_15 = Move(_12);
_12 = core::ptr::addr_of!(_16);
_6 = _2;
(*_12) = 353789721_u32 & 2843365377_u32;
_16 = !1943236085_u32;
_4 = [false,true,true,false,false];
_11 = _1 as f32;
_2 = [true,false,false,true,false];
_6 = _4;
_7.0 = [46336727418275302183120256332267092748_i128,(-5446461850480333999192575463668480094_i128),51903122982140244813019794984096466146_i128,85767569491256003900277779768935962531_i128];
_12 = Move(_15);
_7.0 = [(-65204405330663402428845824331887997894_i128),(-102799795544999209240718725760874295455_i128),(-34288972960159479178326837657266712327_i128),(-25916553870325639626147116685526437802_i128)];
_11 = 106_i8 as f32;
_11 = 1748_u16 as f32;
_8 = [100301389029622016736909444597036797234_u128,222890668743839480341558963008296082455_u128,78086052874970297609091834687741235112_u128,42324502471840643402283071647339556770_u128,286521108064037505168064522017924681179_u128];
_14 = 35_isize ^ 103_isize;
_5 = '\u{10a0d6}';
_14 = -(-9223372036854775808_isize);
Goto(bb7)
}
bb7 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = [true,true,false,true,false];
_12 = core::ptr::addr_of!(_16);
_12 = core::ptr::addr_of!((*_12));
(*_12) = 4154704111_u32;
_12 = core::ptr::addr_of!((*_12));
_11 = 126_i8 as f32;
_7.0 = [(-37042569841908371284964410069988327361_i128),81250330789262617010453619656037888585_i128,(-112859597344890154212113757340835347466_i128),27816761374010661250487676373917869236_i128];
(*_12) = 2132857733129781077_i64 as u32;
_15 = Move(_12);
_2 = [false,true,false,false,false];
_8 = [39889730720404177446890357997608223451_u128,277756279336763631486898187705963693777_u128,1041069269202205091237487053244672542_u128,107526772740680372429712705981719339125_u128,141979033312065393272962161803182551174_u128];
_12 = Move(_15);
Goto(bb9)
}
bb9 = {
_12 = core::ptr::addr_of!(_16);
_15 = Move(_12);
_6 = [false,true,true,true,false];
_8 = [170625869126459285554317573991390203625_u128,96855858959290030663680280139831157107_u128,104836146537271703769838837263816783573_u128,220500606450264222465235423640728978617_u128,243750140775554395483926516311566404747_u128];
_11 = 12880692751888625024_usize as f32;
_8 = [129444240673681667944069940550762006475_u128,42855865011559236411240759267705170886_u128,200539290339906804046612344490300607420_u128,122956231862906588177137391888279172359_u128,49493343239721571985425728015858211451_u128];
_14 = 9223372036854775807_isize - 9223372036854775807_isize;
_12 = Move(_15);
_15 = Move(_12);
_4 = [true,true,false,true,false];
_4 = [true,true,false,true,true];
_11 = _16 as f32;
_6 = _2;
_4 = _6;
Goto(bb10)
}
bb10 = {
_3 = (-83_i8) as f64;
_6 = [false,true,false,true,false];
_12 = Move(_15);
_16 = 3670021402_u32;
_12 = core::ptr::addr_of!(_16);
_14 = 9223372036854775807_isize;
_14 = !(-9223372036854775808_isize);
_5 = '\u{108f9d}';
_18 = [_1,_1,_1,_1,_1,_1];
_11 = _3 as f32;
_6 = [false,true,true,true,true];
_11 = _3 as f32;
_21 = _16 >= _16;
(*_12) = !1896915964_u32;
_20 = &_7;
(*_12) = 1640555526_u32 >> _14;
_3 = (-97_i8) as f64;
_11 = (-604173123_i32) as f32;
_4 = [_21,_21,_21,_21,_21];
_1 = 20_u8;
_20 = &_7;
_5 = '\u{426f4}';
match _1 {
0 => bb1,
1 => bb9,
2 => bb3,
20 => bb11,
_ => bb4
}
}
bb11 = {
_1 = 235_u8 ^ 222_u8;
(*_12) = 5917818662689618997_i64 as u32;
_14 = 9223372036854775807_isize;
_19 = &_3;
(*_12) = !4282324171_u32;
(*_12) = !4128706_u32;
_24 = _14 - _14;
_22 = 988195074_i32;
_25 = [_1];
_26.2 = _5;
Goto(bb12)
}
bb12 = {
_22 = (-1897965737_i32) + 1070436966_i32;
_26.1 = -_11;
_15 = core::ptr::addr_of!((*_12));
_24 = !_14;
_12 = Move(_15);
_26.0 = [(-3392_i16),25332_i16,(-21814_i16),(-1017_i16)];
_18 = [_1,_1,_1,_1,_1,_1];
_29 = _24;
_7.0 = [155934670833952439954906937711673289736_i128,(-22884124356138752521697335423540833816_i128),(-124159021159413343412016999098929114919_i128),(-95244787463979353316875506931757035666_i128)];
_7.0 = [107573041950042096663622930287770606735_i128,70244441057458718117820055930556055932_i128,(-138049216732884194486495447415782397554_i128),16659853399153578187581508357048879199_i128];
_24 = _14;
_20 = &_7;
_14 = !_24;
_27 = _21 ^ _21;
_14 = _24 + _24;
_26.2 = _5;
_22 = 477213443_i32;
_24 = _14 ^ _14;
_26.2 = _5;
match _22 {
0 => bb13,
477213443 => bb15,
_ => bb14
}
}
bb13 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_3 = (-83_i8) as f64;
_6 = [false,true,false,true,false];
_12 = Move(_15);
_16 = 3670021402_u32;
_12 = core::ptr::addr_of!(_16);
_14 = 9223372036854775807_isize;
_14 = !(-9223372036854775808_isize);
_5 = '\u{108f9d}';
_18 = [_1,_1,_1,_1,_1,_1];
_11 = _3 as f32;
_6 = [false,true,true,true,true];
_11 = _3 as f32;
_21 = _16 >= _16;
(*_12) = !1896915964_u32;
_20 = &_7;
(*_12) = 1640555526_u32 >> _14;
_3 = (-97_i8) as f64;
_11 = (-604173123_i32) as f32;
_4 = [_21,_21,_21,_21,_21];
_1 = 20_u8;
_20 = &_7;
_5 = '\u{426f4}';
match _1 {
0 => bb1,
1 => bb9,
2 => bb3,
20 => bb11,
_ => bb4
}
}
bb15 = {
_15 = Move(_12);
_18 = [_1,_1,_1,_1,_1,_1];
_21 = _27;
_15 = core::ptr::addr_of!(_16);
_1 = 86_u8 | 220_u8;
_5 = _26.2;
_27 = _11 <= _26.1;
_29 = 5122161879377933823_u64 as isize;
_3 = 2964617377132184755_i64 as f64;
_25 = [_1];
_5 = _26.2;
_26.2 = _5;
_7.0 = [(-77349819445847845334904445578249857554_i128),158309206518248454250733383683203698209_i128,27947505036998740438026496253496498759_i128,(-39867576028880464652525285031147281720_i128)];
_26.2 = _5;
match _22 {
0 => bb5,
1 => bb2,
2 => bb13,
3 => bb16,
4 => bb17,
477213443 => bb19,
_ => bb18
}
}
bb16 = {
_12 = core::ptr::addr_of!(_16);
_15 = Move(_12);
_6 = [false,true,true,true,false];
_8 = [170625869126459285554317573991390203625_u128,96855858959290030663680280139831157107_u128,104836146537271703769838837263816783573_u128,220500606450264222465235423640728978617_u128,243750140775554395483926516311566404747_u128];
_11 = 12880692751888625024_usize as f32;
_8 = [129444240673681667944069940550762006475_u128,42855865011559236411240759267705170886_u128,200539290339906804046612344490300607420_u128,122956231862906588177137391888279172359_u128,49493343239721571985425728015858211451_u128];
_14 = 9223372036854775807_isize - 9223372036854775807_isize;
_12 = Move(_15);
_15 = Move(_12);
_4 = [true,true,false,true,false];
_4 = [true,true,false,true,true];
_11 = _16 as f32;
_6 = _2;
_4 = _6;
Goto(bb10)
}
bb17 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_1 = 60_u8;
_2 = _6;
_14 = !(-9223372036854775808_isize);
_3 = 4454243984223214387_u64 as f64;
_14 = !(-9223372036854775808_isize);
_4 = [true,true,false,true,false];
_4 = _2;
_1 = 166_u8;
_5 = '\u{9ab3a}';
_2 = [false,false,true,false,true];
_11 = 16940793306327671780_u64 as f32;
_5 = '\u{d857e}';
_1 = !253_u8;
_8 = [32581655619946289568936222653223004751_u128,338444785173222414943513887870484954520_u128,246024033675498417557245320188251133979_u128,338683642303583460150777048413392922242_u128,311549134336732124792017367878450950432_u128];
_11 = _3 as f32;
_4 = _2;
_5 = '\u{55c50}';
_6 = _4;
_14 = (-9223372036854775808_isize);
_12 = core::ptr::addr_of!(_16);
_2 = _4;
_3 = 6169415406621397326_usize as f64;
_6 = [false,true,true,false,false];
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb19 = {
_12 = core::ptr::addr_of!((*_15));
_16 = !3891086800_u32;
_11 = 32994_u16 as f32;
_6 = [_21,_21,_21,_21,_27];
_30 = core::ptr::addr_of!(_22);
(*_30) = _27 as i32;
_1 = 330842619893061940589426255263468435941_u128 as u8;
(*_15) = 3646309059_u32;
_29 = 16225438696956310695_usize as isize;
_15 = core::ptr::addr_of!(_16);
Goto(bb20)
}
bb20 = {
_8 = [22490927164033432326948060952781619655_u128,135570999992948999071937309632827340333_u128,167328565095202443340078524413074910496_u128,22878312266100437325110016631565397663_u128,244132127620404865812415068622381289803_u128];
(*_15) = 4166729734_u32 >> _22;
_19 = &_3;
_24 = 157173331071250909697841942483460440472_i128 as isize;
_7.0 = [91402578364406687175089763462321011979_i128,(-130812049516056687424291964966440057050_i128),(-12036863628897198531170934580763079923_i128),(-17980123219888768774488681581076825258_i128)];
_26.0 = [(-8538_i16),1372_i16,2454_i16,(-28844_i16)];
(*_15) = 3938035388_u32;
Goto(bb21)
}
bb21 = {
_20 = &_7;
_4 = [_21,_21,_21,_21,_21];
_12 = Move(_15);
_15 = Move(_12);
_16 = 3693636680_u32 << _29;
_33 = !_21;
_7.0 = [(-98131882403889819698797329653620316972_i128),116824220126455187751197393721052274453_i128,20688417208348605716453170187138250364_i128,(-54452366756490280755479454024945970906_i128)];
(*_30) = (-301757806_i32);
_14 = (-5589529067092595777_i64) as isize;
_6 = _2;
(*_30) = (-1692388809_i32) >> _16;
(*_30) = (-90802813_i32) << _24;
_39.2.0 = [_22];
_39.1.3 = _3;
_5 = _26.2;
_38 = [_1];
_26.1 = _11;
_31 = 84230519476757153656191632107922068230_i128 as f32;
Goto(bb22)
}
bb22 = {
_39.1.1 = [18466357650938421899972750988556200786_i128,113472410462722369093290463231597273870_i128,40044165864586647131890104680053642092_i128,80415987034687721490797011584882289633_i128];
_12 = Move(_15);
_7 = (_39.1.1,);
(*_30) = (-2057903798_i32) + (-75472982_i32);
_39.3 = (_39.1.1,);
_14 = 109276928259409563195071841527495284637_u128 as isize;
_32.0 = [(*_30)];
_26.2 = _5;
_39.2 = (_32.0,);
_29 = _14 * _24;
_32.0 = [(*_30)];
_42 = (-6803779237736190415_i64) as u128;
_39.3 = (_39.1.1,);
_38 = _25;
_25 = [_1];
_30 = core::ptr::addr_of!((*_30));
Goto(bb23)
}
bb23 = {
_40 = core::ptr::addr_of!(_23);
_44 = _33;
_39.1.1 = _7.0;
_39.3 = (_7.0,);
_32 = _39.2;
_16 = !3297745531_u32;
_32 = (_39.2.0,);
_37 = [20_i8,(-49_i8),(-66_i8)];
_41 = 8140760036092857268_usize as f32;
_15 = core::ptr::addr_of!(_39.1.2);
_39.1.1 = [(-120461898377632339531948405314200661624_i128),(-112014914381886351811449257520419640557_i128),72085541747800465541370661024318446543_i128,(-149363434296119423109653994756627348721_i128)];
_20 = &_7;
_39.1.1 = [(-168442528990072877246364788112341394123_i128),(-114078576098367938898376447837816903722_i128),166913080886723113138955472293478479117_i128,(-74889542152326949011639668844728680080_i128)];
_18 = [_1,_1,_1,_1,_1,_1];
_42 = !235177286938016925367761579063956210091_u128;
(*_15) = !_16;
_7 = _39.3;
_8 = [_42,_42,_42,_42,_42];
_22 = (-1611182696_i32);
_17 = &_42;
_32 = (_39.2.0,);
match _22 {
0 => bb22,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
6 => bb29,
340282366920938463463374607430157028760 => bb31,
_ => bb30
}
}
bb24 = {
_1 = 60_u8;
_2 = _6;
_14 = !(-9223372036854775808_isize);
_3 = 4454243984223214387_u64 as f64;
_14 = !(-9223372036854775808_isize);
_4 = [true,true,false,true,false];
_4 = _2;
_1 = 166_u8;
_5 = '\u{9ab3a}';
_2 = [false,false,true,false,true];
_11 = 16940793306327671780_u64 as f32;
_5 = '\u{d857e}';
_1 = !253_u8;
_8 = [32581655619946289568936222653223004751_u128,338444785173222414943513887870484954520_u128,246024033675498417557245320188251133979_u128,338683642303583460150777048413392922242_u128,311549134336732124792017367878450950432_u128];
_11 = _3 as f32;
_4 = _2;
_5 = '\u{55c50}';
_6 = _4;
_14 = (-9223372036854775808_isize);
_12 = core::ptr::addr_of!(_16);
_2 = _4;
_3 = 6169415406621397326_usize as f64;
_6 = [false,true,true,false,false];
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb25 = {
Return()
}
bb26 = {
_8 = [22490927164033432326948060952781619655_u128,135570999992948999071937309632827340333_u128,167328565095202443340078524413074910496_u128,22878312266100437325110016631565397663_u128,244132127620404865812415068622381289803_u128];
(*_15) = 4166729734_u32 >> _22;
_19 = &_3;
_24 = 157173331071250909697841942483460440472_i128 as isize;
_7.0 = [91402578364406687175089763462321011979_i128,(-130812049516056687424291964966440057050_i128),(-12036863628897198531170934580763079923_i128),(-17980123219888768774488681581076825258_i128)];
_26.0 = [(-8538_i16),1372_i16,2454_i16,(-28844_i16)];
(*_15) = 3938035388_u32;
Goto(bb21)
}
bb27 = {
_12 = core::ptr::addr_of!((*_15));
_16 = !3891086800_u32;
_11 = 32994_u16 as f32;
_6 = [_21,_21,_21,_21,_27];
_30 = core::ptr::addr_of!(_22);
(*_30) = _27 as i32;
_1 = 330842619893061940589426255263468435941_u128 as u8;
(*_15) = 3646309059_u32;
_29 = 16225438696956310695_usize as isize;
_15 = core::ptr::addr_of!(_16);
Goto(bb20)
}
bb28 = {
_1 = 235_u8 ^ 222_u8;
(*_12) = 5917818662689618997_i64 as u32;
_14 = 9223372036854775807_isize;
_19 = &_3;
(*_12) = !4282324171_u32;
(*_12) = !4128706_u32;
_24 = _14 - _14;
_22 = 988195074_i32;
_25 = [_1];
_26.2 = _5;
Goto(bb12)
}
bb29 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb30 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb31 = {
_16 = (-29_i8) as u32;
_14 = _29 + _24;
_35 = &(*_17);
_39.1.0 = _1 as u64;
_46 = Move(_35);
_4 = [_44,_44,_21,_33,_27];
(*_30) = (-110_i8) as i32;
_20 = &_7;
_51 = _33 <= _27;
_43 = [_29,_29,_14,_14,_14];
_29 = !_24;
_9 = Move(_17);
_39.1.0 = (-10806_i16) as u64;
_3 = _26.1 as f64;
_39.2.0 = [(*_30)];
_19 = &_49;
Goto(bb32)
}
bb32 = {
_48 = [12074761749091011761_usize,4_usize];
_55 = _29 | _14;
_36 = Adt43::Variant0 { fld0: _39.1.0,fld1: _26,fld2: 1980415078872043953_usize,fld3: 87_i8 };
_19 = &_39.1.3;
_8 = [_42,_42,_42,_42,_42];
_35 = &_53;
_39.1.2 = !_16;
_39.1.1 = (*_20).0;
_32 = (_39.2.0,);
_56 = core::ptr::addr_of!(_32.0);
_31 = 4_usize as f32;
_53 = !_42;
_6 = [_51,_51,_51,_51,_33];
place!(Field::<i8>(Variant(_36, 0), 3)) = 112_i8 | (-98_i8);
_1 = !52_u8;
_52 = &_5;
_15 = Move(_12);
_4 = [_51,_33,_51,_51,_51];
_34 = (*_30) as f64;
_8 = [_53,_53,_42,_42,_53];
Goto(bb33)
}
bb33 = {
_45 = Field::<i8>(Variant(_36, 0), 3) + Field::<i8>(Variant(_36, 0), 3);
_39.1.3 = -_3;
_9 = &_42;
(*_56) = [(*_30)];
_60 = !_22;
Goto(bb34)
}
bb34 = {
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).2 = _5;
_7 = _39.3;
_26.0 = [(-13832_i16),13828_i16,31949_i16,3127_i16];
_46 = Move(_9);
_42 = _45 as u128;
_50 = Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1).1 * _26.1;
_17 = &_53;
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).1 = -_26.1;
_7 = (_39.3.0,);
_25 = _38;
place!(Field::<u64>(Variant(_36, 0), 0)) = _39.1.0;
_29 = _14 * _55;
place!(Field::<u64>(Variant(_36, 0), 0)) = _39.1.0;
_46 = Move(_17);
(*_56) = [(*_30)];
Goto(bb35)
}
bb35 = {
_2 = [_33,_33,_44,_33,_21];
_40 = core::ptr::addr_of!(_10);
_39.1.1 = _39.3.0;
_7.0 = [(-164568734263669753877549106777131182383_i128),2790077444186567414425040979114733047_i128,(-142362072095719038450074616877605532284_i128),(-123246025832107851288790468960931080256_i128)];
_14 = _29 << _24;
_2 = [_33,_33,_21,_27,_51];
_39.2 = (_32.0,);
_30 = core::ptr::addr_of!((*_30));
_39.3.0 = [(-3982737329471166200910667053759089935_i128),(-1693221386587544976324086874751446390_i128),103066642307251981036353699272782496838_i128,114887605571230441533052058668000674830_i128];
_39.1.0 = !Field::<u64>(Variant(_36, 0), 0);
_7.0 = [(-12424329233928762970608183894770560498_i128),(-11474007101806755594916355702166160491_i128),(-167075073942028917221979736361020491461_i128),88881762216004384957806819604444628922_i128];
_45 = -Field::<i8>(Variant(_36, 0), 3);
(*_30) = 58607_u16 as i32;
_30 = core::ptr::addr_of!((*_30));
_33 = !_51;
_7 = (_39.1.1,);
_59 = [23054_u16,37549_u16,27465_u16,53962_u16,25019_u16,914_u16,4701_u16,6168_u16];
_62 = core::ptr::addr_of!(_39.2.0);
place!(Field::<usize>(Variant(_36, 0), 2)) = _14 as usize;
_58 = -_34;
Call(_45 = core::intrinsics::transmute(_21), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
_58 = _34 - _34;
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).2 = (*_52);
_37 = [_45,_45,Field::<i8>(Variant(_36, 0), 3)];
_21 = !_33;
_39.1.2 = (*_52) as u32;
_15 = core::ptr::addr_of!(_16);
_39.2 = ((*_56),);
_39.1.1 = [38103529608774698321768169551477463964_i128,106968734926547997626933486709435660080_i128,133078309654496935653852614321650067487_i128,107330253928866041126975178123376717934_i128];
_17 = &_42;
_14 = -_29;
(*_30) = _60 << _14;
(*_15) = 116_u16 as u32;
_61 = _33;
_39.1.1 = [(-86438009993783822370573817669354440163_i128),(-83200404108063863900733649715457150521_i128),(-56428512225987786122493747592157648330_i128),132664899852632851199724554109902705403_i128];
_17 = &_53;
_22 = _60 >> Field::<usize>(Variant(_36, 0), 2);
_3 = _1 as f64;
(*_62) = [(*_30)];
(*_15) = !_39.1.2;
_37 = [Field::<i8>(Variant(_36, 0), 3),_45,_45];
_29 = _55;
_63 = [(-13244_i16),1587_i16,6976_i16,(-20007_i16),(-16614_i16)];
_49 = _39.1.3 * _58;
Goto(bb37)
}
bb37 = {
_8 = [_42,_53,_42,_42,_42];
_39.2.0 = [(*_30)];
_27 = _22 != (*_30);
_12 = Move(_15);
_39.1.0 = Field::<u64>(Variant(_36, 0), 0) << Field::<usize>(Variant(_36, 0), 2);
_48 = [Field::<usize>(Variant(_36, 0), 2),Field::<usize>(Variant(_36, 0), 2)];
_42 = !(*_17);
SetDiscriminant(_36, 0);
_35 = &(*_17);
_46 = &(*_35);
Goto(bb38)
}
bb38 = {
_66 = [(-6551_i16),30387_i16,(-5254_i16),(-18121_i16)];
_39.1.3 = _34 * _49;
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).2 = _5;
_39.1.1 = [(-150973307194767629196568870055365329357_i128),107343162286142382349794978706387374857_i128,(-75447595504629905753955055919727545716_i128),(-71372338745699313195768855928953984504_i128)];
_7.0 = _39.1.1;
_26.2 = (*_52);
place!(Field::<i8>(Variant(_36, 0), 3)) = !_45;
_2 = [_27,_27,_27,_61,_33];
_9 = Move(_35);
place!(Field::<i8>(Variant(_36, 0), 3)) = _14 as i8;
place!(Field::<u64>(Variant(_36, 0), 0)) = !_39.1.0;
place!(Field::<u64>(Variant(_36, 0), 0)) = _39.1.0 + _39.1.0;
Goto(bb39)
}
bb39 = {
_71 = -(-20780_i16);
_9 = &_42;
_8 = [_42,(*_9),(*_17),_42,(*_46)];
_5 = _26.2;
_29 = 55778_u16 as isize;
Goto(bb40)
}
bb40 = {
_4 = [_27,_27,_33,_61,_27];
_18 = [_1,_1,_1,_1,_1,_1];
_32 = ((*_62),);
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).0 = [_71,_71,_71,_71];
_22 = _60;
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)) = (_26.0, _31, _26.2);
_32.0 = _39.2.0;
_27 = !_21;
_58 = -_34;
_41 = _31 * _50;
place!(Field::<u64>(Variant(_36, 0), 0)) = _14 as u64;
_60 = (*_30) - (*_30);
_69 = (-61159968862173579574086984937864267400_i128);
match _69 {
0 => bb22,
1 => bb17,
2 => bb3,
3 => bb11,
4 => bb41,
5 => bb42,
6 => bb43,
279122398058764883889287622493903944056 => bb45,
_ => bb44
}
}
bb41 = {
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).2 = _5;
_7 = _39.3;
_26.0 = [(-13832_i16),13828_i16,31949_i16,3127_i16];
_46 = Move(_9);
_42 = _45 as u128;
_50 = Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1).1 * _26.1;
_17 = &_53;
place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).1 = -_26.1;
_7 = (_39.3.0,);
_25 = _38;
place!(Field::<u64>(Variant(_36, 0), 0)) = _39.1.0;
_29 = _14 * _55;
place!(Field::<u64>(Variant(_36, 0), 0)) = _39.1.0;
_46 = Move(_17);
(*_56) = [(*_30)];
Goto(bb35)
}
bb42 = {
_1 = 235_u8 ^ 222_u8;
(*_12) = 5917818662689618997_i64 as u32;
_14 = 9223372036854775807_isize;
_19 = &_3;
(*_12) = !4282324171_u32;
(*_12) = !4128706_u32;
_24 = _14 - _14;
_22 = 988195074_i32;
_25 = [_1];
_26.2 = _5;
Goto(bb12)
}
bb43 = {
_4 = [true,true,false,true,false];
_12 = core::ptr::addr_of!(_16);
_12 = core::ptr::addr_of!((*_12));
(*_12) = 4154704111_u32;
_12 = core::ptr::addr_of!((*_12));
_11 = 126_i8 as f32;
_7.0 = [(-37042569841908371284964410069988327361_i128),81250330789262617010453619656037888585_i128,(-112859597344890154212113757340835347466_i128),27816761374010661250487676373917869236_i128];
(*_12) = 2132857733129781077_i64 as u32;
_15 = Move(_12);
_2 = [false,true,false,false,false];
_8 = [39889730720404177446890357997608223451_u128,277756279336763631486898187705963693777_u128,1041069269202205091237487053244672542_u128,107526772740680372429712705981719339125_u128,141979033312065393272962161803182551174_u128];
_12 = Move(_15);
Goto(bb9)
}
bb44 = {
_39.1.1 = [18466357650938421899972750988556200786_i128,113472410462722369093290463231597273870_i128,40044165864586647131890104680053642092_i128,80415987034687721490797011584882289633_i128];
_12 = Move(_15);
_7 = (_39.1.1,);
(*_30) = (-2057903798_i32) + (-75472982_i32);
_39.3 = (_39.1.1,);
_14 = 109276928259409563195071841527495284637_u128 as isize;
_32.0 = [(*_30)];
_26.2 = _5;
_39.2 = (_32.0,);
_29 = _14 * _24;
_32.0 = [(*_30)];
_42 = (-6803779237736190415_i64) as u128;
_39.3 = (_39.1.1,);
_38 = _25;
_25 = [_1];
_30 = core::ptr::addr_of!((*_30));
Goto(bb23)
}
bb45 = {
_26.0 = [_71,_71,_71,_71];
_26 = (Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1).0, Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1).1, _5);
_43 = [_14,_55,_24,_14,_14];
_55 = _69 as isize;
_19 = &_39.1.3;
_68 = core::ptr::addr_of!((*_9));
_54 = _66;
_58 = _39.1.3;
_70 = _14;
_51 = _44;
_53 = (*_68) * _42;
_46 = Move(_9);
_76 = &_39.3.0;
_74 = !_69;
_73 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_36, 0), 2)));
_24 = _14;
_12 = core::ptr::addr_of!(_16);
_15 = core::ptr::addr_of!(_39.1.2);
_39.0 = core::ptr::addr_of_mut!(_73);
_25 = _38;
_26.1 = _41 + _50;
_7.0 = [_74,_69,_69,_74];
Goto(bb46)
}
bb46 = {
_58 = (*_19);
(*_62) = [_22];
_49 = _39.1.3;
_24 = _14 + _14;
_50 = _26.1 * _41;
_36 = Adt43::Variant0 { fld0: _39.1.0,fld1: _26,fld2: 6100568027600740793_usize,fld3: _45 };
_2 = [_61,_33,_27,_44,_21];
match _69 {
0 => bb32,
1 => bb47,
2 => bb48,
3 => bb49,
279122398058764883889287622493903944056 => bb51,
_ => bb50
}
}
bb47 = {
_1 = 235_u8 ^ 222_u8;
(*_12) = 5917818662689618997_i64 as u32;
_14 = 9223372036854775807_isize;
_19 = &_3;
(*_12) = !4282324171_u32;
(*_12) = !4128706_u32;
_24 = _14 - _14;
_22 = 988195074_i32;
_25 = [_1];
_26.2 = _5;
Goto(bb12)
}
bb48 = {
_71 = -(-20780_i16);
_9 = &_42;
_8 = [_42,(*_9),(*_17),_42,(*_46)];
_5 = _26.2;
_29 = 55778_u16 as isize;
Goto(bb40)
}
bb49 = {
_16 = 1197660915_u32 << _14;
_4 = _6;
Call(_2 = fn8(_7.0, _11, _8, _4, Move(_12), _4, _1, _7.0, _14, _4, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb50 = {
_1 = 235_u8 ^ 222_u8;
(*_12) = 5917818662689618997_i64 as u32;
_14 = 9223372036854775807_isize;
_19 = &_3;
(*_12) = !4282324171_u32;
(*_12) = !4128706_u32;
_24 = _14 - _14;
_22 = 988195074_i32;
_25 = [_1];
_26.2 = _5;
Goto(bb12)
}
bb51 = {
_51 = _24 != _24;
_78 = !_24;
(*_15) = (*_12) & _16;
_39.1.3 = _49;
_15 = Move(_12);
_73 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_36, 0), 2)));
RET = Move(_39.0);
_52 = &place!(Field::<([i16; 4], f32, char)>(Variant(_36, 0), 1)).2;
_33 = !_51;
_63 = [_71,_71,_71,_71,_71];
_32.0 = [(*_30)];
_60 = (*_30);
_19 = &_58;
_27 = _33 ^ _51;
_7 = (_39.1.1,);
_39.3.0 = [_74,_69,_74,_69];
_56 = Move(_62);
_65 = _33 == _21;
_1 = 174_u8 ^ 145_u8;
_72 = [_45,_45];
_40 = core::ptr::addr_of!((*_40));
(*_73) = (*_68) as usize;
_81 = Move(_15);
_32.0 = [_22];
_6 = _4;
_25 = [_1];
_74 = _69;
_9 = &_53;
_62 = Move(_56);
Goto(bb52)
}
bb52 = {
Call(_83 = dump_var(7_usize, 65_usize, Move(_65), 44_usize, Move(_44), 63_usize, Move(_63), 4_usize, Move(_4)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(7_usize, 70_usize, Move(_70), 72_usize, Move(_72), 71_usize, Move(_71), 61_usize, Move(_61)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_83 = dump_var(7_usize, 1_usize, Move(_1), 22_usize, Move(_22), 42_usize, Move(_42), 55_usize, Move(_55)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_83 = dump_var(7_usize, 21_usize, Move(_21), 53_usize, Move(_53), 66_usize, Move(_66), 32_usize, Move(_32)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_83 = dump_var(7_usize, 60_usize, Move(_60), 48_usize, Move(_48), 2_usize, Move(_2), 29_usize, Move(_29)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [i128; 4],mut _2: f32,mut _3: [u128; 5],mut _4: [bool; 5],mut _5: *const u32,mut _6: [bool; 5],mut _7: u8,mut _8: [i128; 4],mut _9: isize,mut _10: [bool; 5],mut _11: [bool; 5]) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _12: &'static &'static isize;
let _13: [usize; 1];
let _14: *mut &'static [usize; 1];
let _15: [i16; 4];
let _16: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _17: *mut ([i32; 1],);
let _18: &'static u128;
let _19: *mut &'static [usize; 1];
let _20: &'static isize;
let _21: isize;
let _22: f64;
let _23: [i64; 3];
let _24: isize;
let _25: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _26: f32;
let _27: (u64, [i128; 4], u32, f64);
let _28: ([u8; 6], ([i16; 4], f32, char), (u64, [i128; 4], u32, f64), [i32; 1]);
let _29: isize;
let _30: ();
let _31: ();
{
RET = _10;
_11 = [false,false,false,true,false];
_2 = _9 as f32;
Goto(bb1)
}
bb1 = {
_3 = [91986162062056632885324358321997417799_u128,330429239515719912637227556595983717032_u128,32227976775567821053786944327175299503_u128,99581980325073370640165941735672538443_u128,210017330026815904659847811881563222936_u128];
_1 = [(-63379397221061695187027352232478186467_i128),(-95605320737048636682447788179530302136_i128),(-135586516618019538793910620501334471483_i128),31112573715593796037363548445995934774_i128];
_8 = [(-57983584093212549575320661698647568701_i128),143020160470646100030496373668323616509_i128,121262804015566444096420313379978073914_i128,111325349548501076336770225839806310364_i128];
_1 = [(-135485415327929086526167815518397918754_i128),(-95011403301937203270379437970147327280_i128),(-67895893813619312677032685573372240147_i128),105343093701981960228947737824794467894_i128];
_8 = _1;
RET = [true,false,false,false,false];
RET = [false,true,false,true,true];
_3 = [48517293379216593936641973078468415069_u128,70597433191066104589702151462916908907_u128,274086579589442656039417250405702740888_u128,288220892637650375601349150265721114012_u128,334156580414570931664622775459855749796_u128];
RET = [false,false,true,false,false];
_7 = 132_u8;
_10 = [false,true,false,false,false];
_10 = [true,false,true,true,true];
_10 = _6;
_3 = [76336765852524141848586075621340242368_u128,42219764288611575954739895994344490396_u128,53802845418015653847598229073874116566_u128,107281679477313726121547454411083325498_u128,45968153151868310546580262100741081577_u128];
_10 = [false,false,true,false,false];
_7 = !215_u8;
_3 = [222637497195451542976853078130023594139_u128,206759246337751838426275957495693955263_u128,30509161610807641520874134757238615660_u128,123904452399240328403945344249079781451_u128,11978400014242421390774441985195495431_u128];
_6 = [false,true,true,true,false];
RET = [true,true,false,true,false];
_1 = [(-49895846564173967302567155570739226004_i128),167286344299623439772811450018251621006_i128,(-115865516148146191099878739940556472231_i128),(-162453233438137087100191255690851319242_i128)];
RET = [false,false,true,true,true];
_10 = RET;
_10 = _4;
_9 = !(-9223372036854775808_isize);
RET = [false,false,false,false,false];
_6 = _11;
Goto(bb2)
}
bb2 = {
_4 = [false,true,false,true,true];
_1 = _8;
_9 = _2 as isize;
Call(_10 = fn9(_3, _8, _3, _4, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = [247198640819697539940371342813036226736_u128,31667224331517500605505673956976836007_u128,32100679411283038885199590995321319971_u128,44394068398717365119932730527436637682_u128,105414662186672688961670112760905903324_u128];
_1 = [(-10628796872200653969476777708074411508_i128),52871502581992844118792286839535435215_i128,(-99987248210334442331582974803290053023_i128),(-104381089880342889432322021702510601256_i128)];
_1 = [12646954582957231063548123891814431481_i128,(-75184891242055618329600047623165339623_i128),(-152651466250388381361271500311690592911_i128),70513627719152555797456192453998114290_i128];
_10 = _6;
_4 = _10;
_8 = [92717248803063929657874847587167923950_i128,147087190708379708697784728981263267078_i128,(-144227543363626725053656183579054410797_i128),(-104083971739981168655875542108397480521_i128)];
_3 = [179986410412292288423288699063434285564_u128,312672285255640789170214653634831851619_u128,186043991452725056933380378869351352032_u128,288747632423117901111568314671423962270_u128,302641072874362683628998876036066477973_u128];
_8 = _1;
_7 = (-64316471959634901485785282337886619413_i128) as u8;
_4 = [true,false,true,true,false];
_9 = (-9223372036854775808_isize) << _7;
_6 = [true,false,false,true,true];
_4 = [false,true,true,false,false];
_6 = [true,false,false,false,true];
_13 = [5074517060474332202_usize];
_1 = [(-89596212851942034982555596342654635197_i128),(-24851062637034306089635413686863117084_i128),33921921165360476278454412405130151741_i128,(-105082880935101501284612017982696874559_i128)];
_16.1.3 = _7 as f64;
_10 = [false,true,true,true,true];
_7 = 13176_i16 as u8;
_3 = [139292137836075730586134646728245147456_u128,55483963568295604873941496159165684777_u128,320549632448466089674021817712504692642_u128,83943944303217350404376957364181168487_u128,335814341938778809144378607688855891266_u128];
_16.2.0 = [(-2022899738_i32)];
_11 = [true,true,false,true,true];
_15 = [11391_i16,30217_i16,(-4118_i16),(-21596_i16)];
_10 = _6;
Goto(bb4)
}
bb4 = {
_13 = [3_usize];
_11 = [false,true,false,true,true];
_1 = [(-40786646095229766317229797179772054731_i128),(-139091770745437512799626034871724173111_i128),2494760208857626757043992082371988394_i128,9923353100577410106529908982534913765_i128];
_16.1.0 = !13661976682436179931_u64;
_8 = [(-158921945699900611973580366115674081048_i128),(-68603658301503887149726557793460967412_i128),72662204481469349040470134907168932068_i128,41798760471508771889527933298603913963_i128];
_16.1.2 = _9 as u32;
_16.1.0 = 15684065994962688001_u64;
_7 = 120_u8 & 30_u8;
match _16.1.0 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
15684065994962688001 => bb12,
_ => bb11
}
}
bb5 = {
_3 = [247198640819697539940371342813036226736_u128,31667224331517500605505673956976836007_u128,32100679411283038885199590995321319971_u128,44394068398717365119932730527436637682_u128,105414662186672688961670112760905903324_u128];
_1 = [(-10628796872200653969476777708074411508_i128),52871502581992844118792286839535435215_i128,(-99987248210334442331582974803290053023_i128),(-104381089880342889432322021702510601256_i128)];
_1 = [12646954582957231063548123891814431481_i128,(-75184891242055618329600047623165339623_i128),(-152651466250388381361271500311690592911_i128),70513627719152555797456192453998114290_i128];
_10 = _6;
_4 = _10;
_8 = [92717248803063929657874847587167923950_i128,147087190708379708697784728981263267078_i128,(-144227543363626725053656183579054410797_i128),(-104083971739981168655875542108397480521_i128)];
_3 = [179986410412292288423288699063434285564_u128,312672285255640789170214653634831851619_u128,186043991452725056933380378869351352032_u128,288747632423117901111568314671423962270_u128,302641072874362683628998876036066477973_u128];
_8 = _1;
_7 = (-64316471959634901485785282337886619413_i128) as u8;
_4 = [true,false,true,true,false];
_9 = (-9223372036854775808_isize) << _7;
_6 = [true,false,false,true,true];
_4 = [false,true,true,false,false];
_6 = [true,false,false,false,true];
_13 = [5074517060474332202_usize];
_1 = [(-89596212851942034982555596342654635197_i128),(-24851062637034306089635413686863117084_i128),33921921165360476278454412405130151741_i128,(-105082880935101501284612017982696874559_i128)];
_16.1.3 = _7 as f64;
_10 = [false,true,true,true,true];
_7 = 13176_i16 as u8;
_3 = [139292137836075730586134646728245147456_u128,55483963568295604873941496159165684777_u128,320549632448466089674021817712504692642_u128,83943944303217350404376957364181168487_u128,335814341938778809144378607688855891266_u128];
_16.2.0 = [(-2022899738_i32)];
_11 = [true,true,false,true,true];
_15 = [11391_i16,30217_i16,(-4118_i16),(-21596_i16)];
_10 = _6;
Goto(bb4)
}
bb6 = {
_4 = [false,true,false,true,true];
_1 = _8;
_9 = _2 as isize;
Call(_10 = fn9(_3, _8, _3, _4, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = [91986162062056632885324358321997417799_u128,330429239515719912637227556595983717032_u128,32227976775567821053786944327175299503_u128,99581980325073370640165941735672538443_u128,210017330026815904659847811881563222936_u128];
_1 = [(-63379397221061695187027352232478186467_i128),(-95605320737048636682447788179530302136_i128),(-135586516618019538793910620501334471483_i128),31112573715593796037363548445995934774_i128];
_8 = [(-57983584093212549575320661698647568701_i128),143020160470646100030496373668323616509_i128,121262804015566444096420313379978073914_i128,111325349548501076336770225839806310364_i128];
_1 = [(-135485415327929086526167815518397918754_i128),(-95011403301937203270379437970147327280_i128),(-67895893813619312677032685573372240147_i128),105343093701981960228947737824794467894_i128];
_8 = _1;
RET = [true,false,false,false,false];
RET = [false,true,false,true,true];
_3 = [48517293379216593936641973078468415069_u128,70597433191066104589702151462916908907_u128,274086579589442656039417250405702740888_u128,288220892637650375601349150265721114012_u128,334156580414570931664622775459855749796_u128];
RET = [false,false,true,false,false];
_7 = 132_u8;
_10 = [false,true,false,false,false];
_10 = [true,false,true,true,true];
_10 = _6;
_3 = [76336765852524141848586075621340242368_u128,42219764288611575954739895994344490396_u128,53802845418015653847598229073874116566_u128,107281679477313726121547454411083325498_u128,45968153151868310546580262100741081577_u128];
_10 = [false,false,true,false,false];
_7 = !215_u8;
_3 = [222637497195451542976853078130023594139_u128,206759246337751838426275957495693955263_u128,30509161610807641520874134757238615660_u128,123904452399240328403945344249079781451_u128,11978400014242421390774441985195495431_u128];
_6 = [false,true,true,true,false];
RET = [true,true,false,true,false];
_1 = [(-49895846564173967302567155570739226004_i128),167286344299623439772811450018251621006_i128,(-115865516148146191099878739940556472231_i128),(-162453233438137087100191255690851319242_i128)];
RET = [false,false,true,true,true];
_10 = RET;
_10 = _4;
_9 = !(-9223372036854775808_isize);
RET = [false,false,false,false,false];
_6 = _11;
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
_10 = _11;
_16.1.3 = _16.1.2 as f64;
_22 = _16.1.3 * _16.1.3;
_12 = &_20;
_16.1.1 = [25139928761879685214935223574345875009_i128,147721291906455566439815803159463647818_i128,(-81911872873684509143185431624587371935_i128),(-343096037863784979922674490092872893_i128)];
_16.1.1 = [71277270398668493362481118457918005416_i128,97946439990851425162860375474157375007_i128,97368742521098925704930610988603493284_i128,(-104372711705608821602736573672331204671_i128)];
_9 = (-9223372036854775808_isize);
_16.3.0 = _8;
_1 = _16.3.0;
RET = [true,true,false,true,false];
_25.2 = (_16.2.0,);
_7 = !12_u8;
_10 = [false,false,true,false,false];
_24 = _16.1.2 as isize;
_27.3 = _16.1.3 + _22;
_25.3 = _16.3;
_10 = [true,true,false,true,true];
match _9 {
0 => bb1,
1 => bb2,
2 => bb11,
3 => bb8,
4 => bb6,
5 => bb13,
6 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = [247198640819697539940371342813036226736_u128,31667224331517500605505673956976836007_u128,32100679411283038885199590995321319971_u128,44394068398717365119932730527436637682_u128,105414662186672688961670112760905903324_u128];
_1 = [(-10628796872200653969476777708074411508_i128),52871502581992844118792286839535435215_i128,(-99987248210334442331582974803290053023_i128),(-104381089880342889432322021702510601256_i128)];
_1 = [12646954582957231063548123891814431481_i128,(-75184891242055618329600047623165339623_i128),(-152651466250388381361271500311690592911_i128),70513627719152555797456192453998114290_i128];
_10 = _6;
_4 = _10;
_8 = [92717248803063929657874847587167923950_i128,147087190708379708697784728981263267078_i128,(-144227543363626725053656183579054410797_i128),(-104083971739981168655875542108397480521_i128)];
_3 = [179986410412292288423288699063434285564_u128,312672285255640789170214653634831851619_u128,186043991452725056933380378869351352032_u128,288747632423117901111568314671423962270_u128,302641072874362683628998876036066477973_u128];
_8 = _1;
_7 = (-64316471959634901485785282337886619413_i128) as u8;
_4 = [true,false,true,true,false];
_9 = (-9223372036854775808_isize) << _7;
_6 = [true,false,false,true,true];
_4 = [false,true,true,false,false];
_6 = [true,false,false,false,true];
_13 = [5074517060474332202_usize];
_1 = [(-89596212851942034982555596342654635197_i128),(-24851062637034306089635413686863117084_i128),33921921165360476278454412405130151741_i128,(-105082880935101501284612017982696874559_i128)];
_16.1.3 = _7 as f64;
_10 = [false,true,true,true,true];
_7 = 13176_i16 as u8;
_3 = [139292137836075730586134646728245147456_u128,55483963568295604873941496159165684777_u128,320549632448466089674021817712504692642_u128,83943944303217350404376957364181168487_u128,335814341938778809144378607688855891266_u128];
_16.2.0 = [(-2022899738_i32)];
_11 = [true,true,false,true,true];
_15 = [11391_i16,30217_i16,(-4118_i16),(-21596_i16)];
_10 = _6;
Goto(bb4)
}
bb16 = {
_25.3.0 = [(-157702404263494344300237325930352935816_i128),(-122398676506505794902867029057012111391_i128),133986733185926723847051601481085063172_i128,(-111551485407522004554912775658986856103_i128)];
_24 = -_9;
_12 = &(*_12);
_16.1.2 = _7 as u32;
_5 = core::ptr::addr_of!(_25.1.2);
_25.1.1 = [(-1219648604266297127351608355060828829_i128),(-138652406940834966918631058574048504259_i128),(-107526661171702253409865354644702112740_i128),24566857057428957049807400190936844511_i128];
_24 = _9 | _9;
_24 = _9;
_25.1.0 = _16.1.0;
_27.2 = _2 as u32;
_28.2.3 = -_27.3;
_25.1.2 = 10154_u16 as u32;
_28.2 = (_16.1.0, _1, (*_5), _22);
_21 = _24;
_17 = core::ptr::addr_of_mut!(_16.2);
_16.1.2 = _27.2 & (*_5);
_16.1.1 = _1;
_28.1.0 = _15;
(*_17).0 = [(-1477776620_i32)];
_11 = RET;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(8_usize, 15_usize, Move(_15), 4_usize, Move(_4), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(8_usize, 21_usize, Move(_21), 6_usize, Move(_6), 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [u128; 5],mut _2: [i128; 4],mut _3: [u128; 5],mut _4: [bool; 5],mut _5: *const u32) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _6: [isize; 5];
let _7: char;
let _8: [u128; 5];
let _9: i64;
let _10: &'static f64;
let _11: u16;
let _12: *const u32;
let _13: [u16; 8];
let _14: usize;
let _15: u32;
let _16: [i64; 5];
let _17: Adt40;
let _18: [usize; 6];
let _19: [usize; 1];
let _20: (*mut ([i32; 1],),);
let _21: &'static &'static isize;
let _22: Adt64;
let _23: &'static &'static isize;
let _24: [i64; 7];
let _25: ();
let _26: ();
{
_1 = [34895334584896595483979467199310816072_u128,26341492703586958417004775339255247664_u128,105556582626970431414842009063341590949_u128,101765290465681381596261740589565646144_u128,247629189592846559051934608285315295453_u128];
_6 = [(-109_isize),(-9223372036854775808_isize),(-36_isize),9223372036854775807_isize,(-34_isize)];
_6 = [(-9_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = _3;
RET = [true,true,false,false,false];
RET = [true,false,true,false,true];
RET = _4;
_1 = _3;
Goto(bb1)
}
bb1 = {
_7 = '\u{121de}';
_2 = [(-118752016341596726645345000910911752703_i128),142547095984894024016857115657146744046_i128,(-104013081281720576834367267570190596441_i128),(-18216944042864408926184638615660215856_i128)];
_6 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [62701548592911030315566175423367214158_u128,125269950971325861719145352561772335663_u128,52763872046033638727874597382252515614_u128,72675895169743254408481780312876824619_u128,108908992571124470129928738375585658393_u128];
_1 = [80084427289825090203337558789864315294_u128,79582234435971876549900418635977802753_u128,9703585398644523075053512211187945502_u128,155315427991466213021560919425135647557_u128,87045348482937187266390705534895317340_u128];
_2 = [68164514443771877227362120182375788893_i128,73645947161357000331506855273594431324_i128,135671239655657218724703869540141926774_i128,(-125861932903689926163421378565424210772_i128)];
_7 = '\u{68811}';
_2 = [31645855328761698053113904117405511450_i128,80246555962894509074593936644045281746_i128,(-122660109089068654905692458698078206041_i128),142216336588368839106325821908977252239_i128];
RET = [false,false,true,false,true];
_4 = RET;
_1 = [206903601617907284096353335387924772346_u128,297115991567399516336586830161066358508_u128,282128192287197741020040758816744562372_u128,283249986313385180378826289080577981891_u128,36592686337015493373782088617489461796_u128];
RET = [false,true,false,false,true];
_9 = (-6275362697365708787_i64);
_7 = '\u{e1efa}';
_7 = '\u{50c9}';
_3 = [128555121162215594736700413704484142998_u128,313443924051021813840365853002206643483_u128,268399070027843401029432113899846207154_u128,91734641055533550490008522094151395682_u128,28550876656728568728720668930431082434_u128];
_7 = '\u{3a3e0}';
_4 = [false,false,true,false,true];
_6 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),32_isize,(-9223372036854775808_isize)];
RET = [false,true,false,true,false];
Call(_11 = fn10(_4, Move(_5), _7, _6, _4, _4, _1, _6, _1, _1, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = 6508_u16 | 13730_u16;
_3 = [45999156259995531966624479137024585077_u128,156511679404588365729322154332845670321_u128,168719033713818816766272662167968770112_u128,317572503448692733083151408773179469883_u128,334405793320036336492597747519488331519_u128];
_4 = [false,true,true,false,false];
_8 = _1;
_6 = [(-115_isize),9223372036854775807_isize,(-13_isize),(-9223372036854775808_isize),89_isize];
_4 = [true,false,true,false,false];
_8 = _1;
_8 = _3;
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_4 = [true,false,false,false,true];
_1 = [286450971559264423500511616479591485291_u128,315341633171133628982603552681311513275_u128,22760633259406047073913883027010020154_u128,305975076036046347986510501693952348243_u128,79687070390415350676044271338337397939_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 29678_u16 & 1833_u16;
_3 = [154054391389331164109576926720146717637_u128,195445436922235566507517647616395139034_u128,253968868840880372269510444860739319276_u128,100387370821542501767757322758838536309_u128,8474876285658405742122106358130896976_u128];
_1 = [89597506873256837440711930173940159959_u128,100734770528366572765805328087051649047_u128,203847188232284809456002503226340879556_u128,228467982657902004862669569767831457298_u128,184822389171334456977712265366582976637_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_1 = [181072479587861300769493565075423365048_u128,84311414583895608614093572870415008314_u128,137477700931208630806524845314076147200_u128,220755630143539155544485430650813322928_u128,319781767371185336504921340687029687781_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463457099244734402502669 => bb8,
_ => bb7
}
}
bb3 = {
_7 = '\u{121de}';
_2 = [(-118752016341596726645345000910911752703_i128),142547095984894024016857115657146744046_i128,(-104013081281720576834367267570190596441_i128),(-18216944042864408926184638615660215856_i128)];
_6 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [62701548592911030315566175423367214158_u128,125269950971325861719145352561772335663_u128,52763872046033638727874597382252515614_u128,72675895169743254408481780312876824619_u128,108908992571124470129928738375585658393_u128];
_1 = [80084427289825090203337558789864315294_u128,79582234435971876549900418635977802753_u128,9703585398644523075053512211187945502_u128,155315427991466213021560919425135647557_u128,87045348482937187266390705534895317340_u128];
_2 = [68164514443771877227362120182375788893_i128,73645947161357000331506855273594431324_i128,135671239655657218724703869540141926774_i128,(-125861932903689926163421378565424210772_i128)];
_7 = '\u{68811}';
_2 = [31645855328761698053113904117405511450_i128,80246555962894509074593936644045281746_i128,(-122660109089068654905692458698078206041_i128),142216336588368839106325821908977252239_i128];
RET = [false,false,true,false,true];
_4 = RET;
_1 = [206903601617907284096353335387924772346_u128,297115991567399516336586830161066358508_u128,282128192287197741020040758816744562372_u128,283249986313385180378826289080577981891_u128,36592686337015493373782088617489461796_u128];
RET = [false,true,false,false,true];
_9 = (-6275362697365708787_i64);
_7 = '\u{e1efa}';
_7 = '\u{50c9}';
_3 = [128555121162215594736700413704484142998_u128,313443924051021813840365853002206643483_u128,268399070027843401029432113899846207154_u128,91734641055533550490008522094151395682_u128,28550876656728568728720668930431082434_u128];
_7 = '\u{3a3e0}';
_4 = [false,false,true,false,true];
_6 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),32_isize,(-9223372036854775808_isize)];
RET = [false,true,false,true,false];
Call(_11 = fn10(_4, Move(_5), _7, _6, _4, _4, _1, _6, _1, _1, _3, _1), ReturnTo(bb2), UnwindUnreachable())
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
_6 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_6 = [48_isize,(-43_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [69610426981011307589752005522169044963_i128,60016141662452848157242216705270782915_i128,(-19041711966417749119276534970233926738_i128),92046924107503923152545413359504818513_i128];
_11 = 48750_u16;
RET = _4;
_14 = 3_usize;
_3[_14] = _8[_14];
_12 = core::ptr::addr_of!(_15);
_1[_14] = _8[_14] >> _3[_14];
_7 = '\u{e2914}';
_7 = '\u{69e05}';
_7 = '\u{6870b}';
_16[_14] = _9;
_8 = [_1[_14],_1[_14],_1[_14],_3[_14],_1[_14]];
_4 = [RET[_14],RET[_14],RET[_14],RET[_14],RET[_14]];
_2[_14] = _7 as i128;
_9 = _14 as i64;
_16 = [_9,_9,_9,_9,_9];
_14 = !9195279805269367610_usize;
_5 = core::ptr::addr_of!((*_12));
_8 = [69199467024483810948268610599319225071_u128,176132720897968840716525110157023807904_u128,141830814602708719068180716830096431548_u128,151552898795352966241434614370190609379_u128,1145216144668818658008786893729027328_u128];
_6 = [87_isize,(-9223372036854775808_isize),(-42_isize),(-80_isize),9223372036854775807_isize];
Goto(bb9)
}
bb9 = {
RET = [false,false,false,true,true];
_14 = 5_usize;
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_18[_14] = 245_u8 as usize;
_18 = [_14,_14,_14,_14,_14,_14];
_18[_14] = false as usize;
_13[_14] = _11 * _11;
_11 = !_13[_14];
_2 = [94175784353765049617607342893239194790_i128,28390551138897383428454779608123632581_i128,4057857724036759829649724870162400546_i128,(-47088756643590343052473226352266005238_i128)];
_7 = '\u{ea86}';
_18 = [_14,_14,_14,_14,_14,_14];
RET = [true,false,false,true,false];
_5 = core::ptr::addr_of!((*_5));
_19 = [_18[_14]];
_11 = _13[_14];
_1 = _3;
_3 = _8;
_2 = [63098905687625213514212981200874759108_i128,157198914636387850667450257803051660013_i128,(-91223584243397754493633934455515913956_i128),127152730914591215895818764159564303324_i128];
_8 = [271484712744585322347075963684001881633_u128,82619876022240545657235432951991634489_u128,269241287123686385733013492338518386461_u128,202913409413250751163549559778560750029_u128,316399326566571791869009836210822902381_u128];
match _18[_14] {
0 => bb5,
1 => bb2,
2 => bb6,
3 => bb4,
5 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_12 = core::ptr::addr_of!((*_5));
RET = _4;
(*_12) = 581098087_u32;
_1 = [77126137988149247643219149916603012856_u128,147479344785807108469541740829901718789_u128,110984306452916859016726949635040879039_u128,208415075208390045381052418094832639130_u128,184218825887426727377609179148066471017_u128];
_7 = '\u{7279b}';
_3 = [112094341830988895377050903464848908550_u128,145462200723741878672611516722243295049_u128,111800568793622486048611453841398928812_u128,281451155825376334069476531844204332180_u128,57137457444637274676004393217294146663_u128];
_7 = '\u{d2edb}';
RET = [false,true,true,true,true];
_5 = core::ptr::addr_of!((*_12));
match (*_12) {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb12,
581098087 => bb14,
_ => bb13
}
}
bb12 = {
_11 = 6508_u16 | 13730_u16;
_3 = [45999156259995531966624479137024585077_u128,156511679404588365729322154332845670321_u128,168719033713818816766272662167968770112_u128,317572503448692733083151408773179469883_u128,334405793320036336492597747519488331519_u128];
_4 = [false,true,true,false,false];
_8 = _1;
_6 = [(-115_isize),9223372036854775807_isize,(-13_isize),(-9223372036854775808_isize),89_isize];
_4 = [true,false,true,false,false];
_8 = _1;
_8 = _3;
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_4 = [true,false,false,false,true];
_1 = [286450971559264423500511616479591485291_u128,315341633171133628982603552681311513275_u128,22760633259406047073913883027010020154_u128,305975076036046347986510501693952348243_u128,79687070390415350676044271338337397939_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 29678_u16 & 1833_u16;
_3 = [154054391389331164109576926720146717637_u128,195445436922235566507517647616395139034_u128,253968868840880372269510444860739319276_u128,100387370821542501767757322758838536309_u128,8474876285658405742122106358130896976_u128];
_1 = [89597506873256837440711930173940159959_u128,100734770528366572765805328087051649047_u128,203847188232284809456002503226340879556_u128,228467982657902004862669569767831457298_u128,184822389171334456977712265366582976637_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_1 = [181072479587861300769493565075423365048_u128,84311414583895608614093572870415008314_u128,137477700931208630806524845314076147200_u128,220755630143539155544485430650813322928_u128,319781767371185336504921340687029687781_u128];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463457099244734402502669 => bb8,
_ => bb7
}
}
bb13 = {
Return()
}
bb14 = {
_7 = '\u{4ac5f}';
_8 = [288617806534648283097787364641089646775_u128,198489795216495738600415515726696301411_u128,313557762451308851038367088825062519934_u128,265984358347587716415262458682339003778_u128,176503465291387707023637197070602595709_u128];
RET = [true,false,false,true,false];
_16 = [_9,_9,_9,_9,_9];
_4 = [false,true,false,true,false];
_16 = [_9,_9,_9,_9,_9];
RET = [true,false,true,false,false];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = _4;
_24[_14] = -_9;
_8 = [315380302657072801154265672125230678396_u128,69755770897598617774819928080037392444_u128,161824153807697532032044193761131371363_u128,193599459896803031184656695905125123833_u128,139975495241011226209752936231084550133_u128];
(*_12) = true as u32;
_18[_14] = !_14;
_7 = '\u{27ce}';
(*_5) = _13[_14] as u32;
_3 = [268531050174950242660601674701562913801_u128,165985802619549812267055102968724811756_u128,129794271262660673125970515673623856129_u128,52854158548265063629943148801527418759_u128,181148874037898904257626003441328920813_u128];
RET = [true,true,true,true,false];
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(9_usize, 11_usize, Move(_11), 19_usize, Move(_19), 1_usize, Move(_1), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(9_usize, 4_usize, Move(_4), 6_usize, Move(_6), 16_usize, Move(_16), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [bool; 5],mut _2: *const u32,mut _3: char,mut _4: [isize; 5],mut _5: [bool; 5],mut _6: [bool; 5],mut _7: [u128; 5],mut _8: [isize; 5],mut _9: [u128; 5],mut _10: [u128; 5],mut _11: [u128; 5],mut _12: [u128; 5]) -> u16 {
mir! {
type RET = u16;
let _13: *mut [usize; 1];
let _14: f32;
let _15: &'static &'static isize;
let _16: char;
let _17: *mut usize;
let _18: isize;
let _19: f64;
let _20: *mut *const usize;
let _21: usize;
let _22: isize;
let _23: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _24: Adt32;
let _25: usize;
let _26: *const i32;
let _27: isize;
let _28: [bool; 5];
let _29: bool;
let _30: &'static [usize; 1];
let _31: [bool; 3];
let _32: bool;
let _33: &'static f64;
let _34: [i64; 3];
let _35: [u128; 8];
let _36: Adt21;
let _37: [u8; 3];
let _38: isize;
let _39: &'static u16;
let _40: *mut usize;
let _41: *const [i32; 1];
let _42: char;
let _43: ();
let _44: ();
{
_1 = [false,true,true,true,false];
Goto(bb1)
}
bb1 = {
RET = 38271_u16;
_11 = _9;
_7 = [158754149525730982218982280506467910464_u128,187260167579430939400423760010127810278_u128,43892108286688854990486671967406368271_u128,141072137698448073246723537179330787015_u128,190781158611988049837016583980842952395_u128];
_12 = _10;
_3 = '\u{b11c4}';
_10 = _7;
_1 = _6;
_12 = [34357287702396052016562722362287050329_u128,225413641362300557703822884900050516227_u128,165029081696934177001150739075567099558_u128,211654317467020066756071334663970109161_u128,282051087630425875184160829425401966669_u128];
_5 = [true,true,false,false,false];
RET = !32309_u16;
_10 = _12;
_14 = 7060_i16 as f32;
Goto(bb2)
}
bb2 = {
_9 = _12;
RET = 31_u8 as u16;
_11 = [294703886520405596955613962090774060793_u128,200861165401535750864633710049697818624_u128,133936155610479946164603399433172849158_u128,27130187020525561485609258591451766767_u128,338247904126942315844557940662646799392_u128];
_4 = _8;
_12 = [157827882864579785892550405747462690419_u128,23684789312807033410211108958545524596_u128,237985057585228755174100810645590477045_u128,27926045197684323512015467033932440800_u128,178962910923413352212420294260659240834_u128];
_16 = _3;
_16 = _3;
_4 = _8;
_6 = _5;
RET = 47660_u16;
_14 = 27780_i16 as f32;
_4 = _8;
_16 = _3;
_12 = [216756100659416967518487567877441863474_u128,292559938987232951116613877850313613257_u128,314325414994602156725579869589798605141_u128,283333467255659194018753725133640467536_u128,175028949307104864471919622979477152293_u128];
RET = false as u16;
_3 = _16;
_12 = [247354265190017145073487862482103605726_u128,225032585217714120697552533469496866970_u128,80231304751817766202850348029330449681_u128,254722942445534068800883870020313181542_u128,229892318375541890403110401977791028601_u128];
_19 = 192_u8 as f64;
_3 = _16;
_18 = (-9223372036854775808_isize);
_19 = 52503754805791660948424077198772507477_u128 as f64;
_12 = [268706416409400271614188469210829607260_u128,224362378013943594841827640375400503823_u128,196136751002302026354158530496562332695_u128,31109342255298796685783410788718800161_u128,117078272541093197082183436924082497311_u128];
RET = 2832_u16 & 17807_u16;
_14 = 59_u8 as f32;
Goto(bb3)
}
bb3 = {
_17 = core::ptr::addr_of_mut!(_21);
_8 = [_18,_18,_18,_18,_18];
Call(_10 = fn11(_6, _6, _16, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = [_18,_18,_18,_18,_18];
_1 = [false,false,false,false,false];
_17 = core::ptr::addr_of_mut!((*_17));
_10 = [73264503641051645284382171293938788257_u128,185970743489065180048122394549892533072_u128,332347388824855075714339586818500822905_u128,235879698014177731822716113210019584061_u128,217944105816039976940836966677738327077_u128];
RET = 5799_u16;
(*_17) = 4_usize >> RET;
(*_17) = !17726619858084715776_usize;
_18 = -9223372036854775807_isize;
_16 = _3;
RET = 15644_u16 >> (*_17);
_10 = _11;
_18 = !9223372036854775807_isize;
Goto(bb5)
}
bb5 = {
_23.1.2 = 2221370188_u32 * 4242263119_u32;
_23.1.3 = _19 - _19;
_9 = _12;
_16 = _3;
_6 = [true,false,false,false,true];
_3 = _16;
_9 = [1763546772944276512077874567185733568_u128,301487909983647431124071104865122402030_u128,2548642776754173171666387573794007585_u128,50523900027101701208195322102266466702_u128,207999149874407373574007705165947542723_u128];
_22 = _18;
_12 = [230842404394179528035896326037716433472_u128,39002132024801867088812658731315035808_u128,59727285172936965787686253655095489698_u128,281986883607893000996651590374653018336_u128,114884917833376781435887721768820956180_u128];
_23.1.0 = 9475605720864504868_u64;
_9 = [275884749004412713412370546772165805697_u128,18467178595995139607072773422193172875_u128,164292512392870232020762709909798615619_u128,261909683027007682069345669112203589676_u128,182665448697813251523393522233435749503_u128];
_22 = _18 & _18;
_23.2.0 = [(-2088636388_i32)];
_23.1.1 = [(-100330976006227753620890180399444656714_i128),(-125301333530829753650332567846957133805_i128),136437918814333624022253793445838195244_i128,20687483607085522926948744734847107259_i128];
RET = (*_17) as u16;
_24 = Adt32::Variant1 { fld0: RET,fld1: _3,fld2: 1470015478_i32 };
Call((*_17) = core::intrinsics::transmute(_22), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_24, 1), 2)));
_21 = true as usize;
(*_26) = 46_i8 as i32;
place!(Field::<char>(Variant(_24, 1), 1)) = _16;
_9 = [25195754523637760576786718076650864604_u128,244149915694422789855769139266722463507_u128,744385335485230664434559162384088358_u128,28538884537971565227029650536831284317_u128,295254205221557344413964343600215825606_u128];
_18 = (*_17) as isize;
_7 = [264082268398590629301785597458023937629_u128,190553854850545266214065945401301369875_u128,304665320416550051716515371944027089772_u128,169455544812542907411353093405338608387_u128,200667139438242293724353550634233008851_u128];
_23.3.0 = [112363253694370045726428432742653120182_i128,(-127192916037587632159896082276649294365_i128),(-110021685168558930955605751089076431521_i128),12597042547464710847952727368418901877_i128];
(*_17) = false as usize;
_23.1.3 = _23.1.0 as f64;
_6 = [true,false,false,false,false];
_25 = _21 << _22;
RET = _23.1.0 as u16;
_14 = _19 as f32;
_23.1.2 = 1129703107_u32 ^ 2667493008_u32;
_21 = !_25;
SetDiscriminant(_24, 0);
_10 = [257510439617908740153403037557909172799_u128,62317357910880901923875885882375677336_u128,320917475396694055080614245172459211222_u128,282496999544652435233076447879131720976_u128,15689234430367445311124836597859426826_u128];
_16 = _3;
_27 = _22 & _18;
_16 = _3;
_21 = _3 as usize;
_18 = _22 * _22;
_7 = [160611323623270560781391673359268362290_u128,104162611701113266864441620825178600327_u128,163598574412469932994760687845424337386_u128,193206278435927989480745181710441911351_u128,200377018087224421510388057142009284139_u128];
match _23.1.0 {
0 => bb3,
1 => bb7,
2 => bb8,
9475605720864504868 => bb10,
_ => bb9
}
}
bb7 = {
_9 = _12;
RET = 31_u8 as u16;
_11 = [294703886520405596955613962090774060793_u128,200861165401535750864633710049697818624_u128,133936155610479946164603399433172849158_u128,27130187020525561485609258591451766767_u128,338247904126942315844557940662646799392_u128];
_4 = _8;
_12 = [157827882864579785892550405747462690419_u128,23684789312807033410211108958545524596_u128,237985057585228755174100810645590477045_u128,27926045197684323512015467033932440800_u128,178962910923413352212420294260659240834_u128];
_16 = _3;
_16 = _3;
_4 = _8;
_6 = _5;
RET = 47660_u16;
_14 = 27780_i16 as f32;
_4 = _8;
_16 = _3;
_12 = [216756100659416967518487567877441863474_u128,292559938987232951116613877850313613257_u128,314325414994602156725579869589798605141_u128,283333467255659194018753725133640467536_u128,175028949307104864471919622979477152293_u128];
RET = false as u16;
_3 = _16;
_12 = [247354265190017145073487862482103605726_u128,225032585217714120697552533469496866970_u128,80231304751817766202850348029330449681_u128,254722942445534068800883870020313181542_u128,229892318375541890403110401977791028601_u128];
_19 = 192_u8 as f64;
_3 = _16;
_18 = (-9223372036854775808_isize);
_19 = 52503754805791660948424077198772507477_u128 as f64;
_12 = [268706416409400271614188469210829607260_u128,224362378013943594841827640375400503823_u128,196136751002302026354158530496562332695_u128,31109342255298796685783410788718800161_u128,117078272541093197082183436924082497311_u128];
RET = 2832_u16 & 17807_u16;
_14 = 59_u8 as f32;
Goto(bb3)
}
bb8 = {
_4 = [_18,_18,_18,_18,_18];
_1 = [false,false,false,false,false];
_17 = core::ptr::addr_of_mut!((*_17));
_10 = [73264503641051645284382171293938788257_u128,185970743489065180048122394549892533072_u128,332347388824855075714339586818500822905_u128,235879698014177731822716113210019584061_u128,217944105816039976940836966677738327077_u128];
RET = 5799_u16;
(*_17) = 4_usize >> RET;
(*_17) = !17726619858084715776_usize;
_18 = -9223372036854775807_isize;
_16 = _3;
RET = 15644_u16 >> (*_17);
_10 = _11;
_18 = !9223372036854775807_isize;
Goto(bb5)
}
bb9 = {
RET = 38271_u16;
_11 = _9;
_7 = [158754149525730982218982280506467910464_u128,187260167579430939400423760010127810278_u128,43892108286688854990486671967406368271_u128,141072137698448073246723537179330787015_u128,190781158611988049837016583980842952395_u128];
_12 = _10;
_3 = '\u{b11c4}';
_10 = _7;
_1 = _6;
_12 = [34357287702396052016562722362287050329_u128,225413641362300557703822884900050516227_u128,165029081696934177001150739075567099558_u128,211654317467020066756071334663970109161_u128,282051087630425875184160829425401966669_u128];
_5 = [true,true,false,false,false];
RET = !32309_u16;
_10 = _12;
_14 = 7060_i16 as f32;
Goto(bb2)
}
bb10 = {
_28 = _5;
_4 = [_22,_18,_18,_22,_18];
_5 = [true,false,false,true,false];
_31 = [false,false,true];
match _23.1.0 {
9475605720864504868 => bb12,
_ => bb11
}
}
bb11 = {
_4 = [_18,_18,_18,_18,_18];
_1 = [false,false,false,false,false];
_17 = core::ptr::addr_of_mut!((*_17));
_10 = [73264503641051645284382171293938788257_u128,185970743489065180048122394549892533072_u128,332347388824855075714339586818500822905_u128,235879698014177731822716113210019584061_u128,217944105816039976940836966677738327077_u128];
RET = 5799_u16;
(*_17) = 4_usize >> RET;
(*_17) = !17726619858084715776_usize;
_18 = -9223372036854775807_isize;
_16 = _3;
RET = 15644_u16 >> (*_17);
_10 = _11;
_18 = !9223372036854775807_isize;
Goto(bb5)
}
bb12 = {
(*_17) = _25;
_24 = Adt32::Variant2 { fld0: true,fld1: _23.1.2 };
_23.2.0 = [(-192623667_i32)];
_16 = _3;
_11 = _10;
_3 = _16;
place!(Field::<bool>(Variant(_24, 2), 0)) = !false;
_29 = !Field::<bool>(Variant(_24, 2), 0);
_11 = [222002983699061450102453831768680856464_u128,187578855127422615136961139477147000001_u128,281937956980616073715374284778247445432_u128,203375096178131658521241296112193399154_u128,195538725841844857723150687014474957151_u128];
_18 = !_27;
_11 = [10261568331226213822099916629387332628_u128,316483156479092758660253057501677075625_u128,160581278230027590868033819245401333051_u128,5273124268988640884271995221911517293_u128,322368118055694251991962999825264331127_u128];
_23.3 = (_23.1.1,);
_25 = (*_17);
place!(Field::<bool>(Variant(_24, 2), 0)) = _29;
_5 = [_29,Field::<bool>(Variant(_24, 2), 0),Field::<bool>(Variant(_24, 2), 0),_29,Field::<bool>(Variant(_24, 2), 0)];
(*_17) = _25;
Goto(bb13)
}
bb13 = {
_24 = Adt32::Variant3 { fld0: 1015952333_i32,fld1: _16,fld2: _23.2.0,fld3: _23.1.1 };
_16 = Field::<char>(Variant(_24, 3), 1);
_16 = _3;
_10 = [194164020823364757260545762597927378058_u128,226019313716903941575895930071408487858_u128,88446411343749401639220058149146484680_u128,55779530679305325585241371144916974950_u128,316880077479485285100663465391415461356_u128];
_1 = _28;
match _23.1.0 {
0 => bb8,
9475605720864504868 => bb14,
_ => bb11
}
}
bb14 = {
_27 = !_22;
_42 = _3;
_6 = _1;
_9 = [41005486864996263984268786730476014602_u128,270130736002084004626828563875978044868_u128,215385153469376597971447885057382653458_u128,149681724862637363638958971008772126705_u128,245715463796914343274319858101592550279_u128];
_27 = _22;
_24 = Adt32::Variant2 { fld0: _29,fld1: _23.1.2 };
_9 = _12;
_37 = [162_u8,219_u8,43_u8];
_37 = [242_u8,176_u8,62_u8];
(*_17) = _25 | _25;
_16 = _42;
SetDiscriminant(_24, 1);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(10_usize, 16_usize, Move(_16), 4_usize, Move(_4), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(10_usize, 3_usize, Move(_3), 21_usize, Move(_21), 9_usize, Move(_9), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 27_usize, Move(_27), 12_usize, Move(_12), 5_usize, Move(_5), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [bool; 5],mut _2: [bool; 5],mut _3: char,mut _4: [u128; 5]) -> [u128; 5] {
mir! {
type RET = [u128; 5];
let _5: Adt21;
let _6: &'static i64;
let _7: *const usize;
let _8: &'static ([i128; 4],);
let _9: &'static [i128; 4];
let _10: [i128; 4];
let _11: *mut [usize; 1];
let _12: u64;
let _13: &'static u16;
let _14: (u64, [i128; 4], u32, f64);
let _15: [u8; 1];
let _16: i16;
let _17: &'static [i128; 4];
let _18: [u32; 5];
let _19: Adt64;
let _20: &'static char;
let _21: isize;
let _22: [i32; 1];
let _23: isize;
let _24: Adt40;
let _25: ([i128; 4],);
let _26: u128;
let _27: Adt80;
let _28: [i16; 4];
let _29: u128;
let _30: ();
let _31: ();
{
RET = _4;
_1 = [true,true,false,false,false];
_1 = [false,false,false,true,true];
RET = [162943059554993648745266058099530916309_u128,280388294609068801973666592842349771330_u128,132470131931100978741151492760944279628_u128,196648396820945768531635565316619698057_u128,229097058402048779165106569007592004604_u128];
RET = [235874916196056251304760535432507866238_u128,61564870687026837498445584946504979397_u128,328822692499854952525903780622558116814_u128,5453328730969129388254409329590746637_u128,19747698687902693906458623556218256359_u128];
_2 = [false,false,false,true,true];
_2 = [true,true,false,false,false];
_3 = '\u{d0aef}';
_4 = RET;
_3 = '\u{4fc7e}';
_4 = [9585514521202042662965682203230287674_u128,43319459113451590490855979160215728909_u128,292155096839799012091477239556893742771_u128,167447641288021209859886122173940561836_u128,253614016088738051459265034003356417080_u128];
Goto(bb1)
}
bb1 = {
_3 = '\u{777e8}';
RET = [107552137073891240529312354989615448674_u128,115081585800298152746650757971444741489_u128,265667950035994022376986263569151515216_u128,174771595119715951038964562305024154421_u128,38948338659727195404004932115477486955_u128];
RET = [281449217674113324318778080077624465233_u128,274019349572909105153939066159636654114_u128,18125605158633818083479414012088008960_u128,310769874715228840957972519408630239304_u128,43739176874861879271409200523288080347_u128];
RET = [261282149240566199393392460805989601461_u128,299817481856508439516427499177124681966_u128,66889876841662277366253128384348247631_u128,297197411224130950751018181453974903434_u128,292344692258704977155991515035386371522_u128];
_3 = '\u{a9da}';
_4 = RET;
Goto(bb2)
}
bb2 = {
_1 = _2;
RET = _4;
RET = [122717931603212765598495567599540810594_u128,176048732676297659882903223095790241639_u128,189246587292721980581331966667083383007_u128,170428700736066780287013580439429217601_u128,129186384593018405431079589646255477604_u128];
_3 = '\u{c9936}';
_3 = '\u{d92e6}';
_1 = [false,false,false,true,true];
RET = [141268641534269847653732652220808574570_u128,24158738266255303501640016544248014789_u128,66494981458897484589104924769267498581_u128,95197424532894167695653162029592205192_u128,273543038407926769822720903281403000102_u128];
_5 = Adt21::Variant0 { fld0: 1088525117747493828_u64,fld1: (-42_isize) };
place!(Field::<isize>(Variant(_5, 0), 1)) = (-6_isize) | 9223372036854775807_isize;
RET = _4;
_1 = [true,true,false,true,true];
place!(Field::<u64>(Variant(_5, 0), 0)) = 4133984150_u32 as u64;
place!(Field::<isize>(Variant(_5, 0), 1)) = (-68_isize) - (-9223372036854775808_isize);
_2 = [false,true,false,true,true];
_2 = [false,true,true,true,true];
Goto(bb3)
}
bb3 = {
place!(Field::<u64>(Variant(_5, 0), 0)) = !12369210783840365507_u64;
place!(Field::<u64>(Variant(_5, 0), 0)) = 18019744866961640450_u64 >> Field::<isize>(Variant(_5, 0), 1);
_9 = &_10;
_2 = _1;
_3 = '\u{3dfb3}';
place!(Field::<isize>(Variant(_5, 0), 1)) = 0_u8 as isize;
_10 = [73340693193450160458348122238819504615_i128,(-117755473277433377110073672066058688689_i128),163793349888113638542263148083043517987_i128,(-129103529153127978043476050191261218600_i128)];
_12 = Field::<u64>(Variant(_5, 0), 0) * Field::<u64>(Variant(_5, 0), 0);
_9 = &_10;
_5 = Adt21::Variant1 { fld0: 8943_u16 };
_10 = [63888660423821798871942303771400419639_i128,48190258137206506221152964915320086556_i128,136701057553463430869935423946978219310_i128,107106853136610033988274367737192123988_i128];
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
place!(Field::<u16>(Variant(_5, 1), 0)) = 87593221530605553377023887099802853087_u128 as u16;
_3 = '\u{bb7ba}';
RET = [43883497566806741990235640777139081447_u128,165596431926984029826654856362118753267_u128,283370353832274940335234373970400617873_u128,329267056648386291944140569372953712230_u128,247259312791212062142822846500293225116_u128];
_14.3 = 5_usize as f64;
RET = _4;
_14.2 = 9140110610261983914_usize as u32;
_12 = 9223372036854775807_isize as u64;
_12 = 7818556801315723626_usize as u64;
_15 = [52_u8];
_9 = &_10;
RET = [251327653218951542381623121273587205007_u128,96872602277187901190022057383377149397_u128,50379265601241924587806738657177575251_u128,331380008969076535131878610180816904399_u128,150103990705041517273991663799565958524_u128];
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
Call(RET = fn12(Move(_9), Move(_13), (*_9), (*_13), _3, _1, Field::<u16>(Variant(_5, 1), 0), _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.1 = [25218654463224818122969828049712073135_i128,120712558264094237432810624595822131062_i128,114515168747267207915719546104105726720_i128,(-146246673762081125525637774717577019298_i128)];
_14.2 = 3117618922_u32 - 210026923_u32;
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
_14.3 = 9223372036854775807_isize as f64;
Goto(bb5)
}
bb5 = {
_3 = '\u{1fccf}';
RET = [17356727536112715523765606407796160104_u128,281241027566639459170130146854101288955_u128,29086991769678135073788863665145713322_u128,103977465233140145299290533455863834410_u128,51304000802071299495921910654225946178_u128];
_17 = &_10;
_17 = &_14.1;
_14.1 = _10;
_9 = &_10;
_14.2 = 3934180252_u32;

_1 = _2;
_5 = Adt21::Variant2 { fld0: 65208965131948858390786062094399667321_u128,fld1: _14.3,fld2: (-9223372036854775808_isize) };
_17 = &(*_9);
_10 = _14.1;
_16 = 21825_i16;
RET = [294116789989398935005549726128741187199_u128,322824966771604538791731930411394349929_u128,327516950276018926785522550230408334259_u128,135603085945348743616813404582334532999_u128,42078763607075260880106717477536348368_u128];
match _16 {
21825 => bb7,
_ => bb6
}
}
bb6 = {
_1 = _2;
RET = _4;
RET = [122717931603212765598495567599540810594_u128,176048732676297659882903223095790241639_u128,189246587292721980581331966667083383007_u128,170428700736066780287013580439429217601_u128,129186384593018405431079589646255477604_u128];
_3 = '\u{c9936}';
_3 = '\u{d92e6}';
_1 = [false,false,false,true,true];
RET = [141268641534269847653732652220808574570_u128,24158738266255303501640016544248014789_u128,66494981458897484589104924769267498581_u128,95197424532894167695653162029592205192_u128,273543038407926769822720903281403000102_u128];
_5 = Adt21::Variant0 { fld0: 1088525117747493828_u64,fld1: (-42_isize) };
place!(Field::<isize>(Variant(_5, 0), 1)) = (-6_isize) | 9223372036854775807_isize;
RET = _4;
_1 = [true,true,false,true,true];
place!(Field::<u64>(Variant(_5, 0), 0)) = 4133984150_u32 as u64;
place!(Field::<isize>(Variant(_5, 0), 1)) = (-68_isize) - (-9223372036854775808_isize);
_2 = [false,true,false,true,true];
_2 = [false,true,true,true,true];
Goto(bb3)
}
bb7 = {
_9 = &_14.1;
_20 = &_3;
place!(Field::<f64>(Variant(_5, 2), 1)) = _14.3;
_9 = &(*_9);
_9 = &_14.1;
place!(Field::<u128>(Variant(_5, 2), 0)) = _14.2 as u128;
place!(Field::<isize>(Variant(_5, 2), 2)) = (-46_i8) as isize;
_9 = &(*_9);
_17 = &(*_9);
_25.0 = [139113506296256134765892622509014950043_i128,(-19976225072599904039413523221093357531_i128),5971976957775757367779315139402850952_i128,20695684036021511825951917611433115251_i128];
_14 = (_12, _10, 2133558043_u32, Field::<f64>(Variant(_5, 2), 1));
_25 = (_14.1,);
_20 = &_3;
_4 = RET;
_23 = _12 as isize;
_14.2 = !1016691643_u32;
_25 = (_14.1,);
Call(_30 = dump_var(11_usize, 10_usize, Move(_10), 23_usize, Move(_23), 12_usize, _12, 14_usize, _14), ReturnTo(t1), UnwindUnreachable())
}
t1 = {
_20 = &(*_20);
match _16 {
0 => bb2,
1 => bb8,
2 => bb9,
21825 => bb11,
_ => bb10
}
}
bb8 = {
place!(Field::<u64>(Variant(_5, 0), 0)) = !12369210783840365507_u64;
place!(Field::<u64>(Variant(_5, 0), 0)) = 18019744866961640450_u64 >> Field::<isize>(Variant(_5, 0), 1);
_9 = &_10;
_2 = _1;
_3 = '\u{3dfb3}';
place!(Field::<isize>(Variant(_5, 0), 1)) = 0_u8 as isize;
_10 = [73340693193450160458348122238819504615_i128,(-117755473277433377110073672066058688689_i128),163793349888113638542263148083043517987_i128,(-129103529153127978043476050191261218600_i128)];
_12 = Field::<u64>(Variant(_5, 0), 0) * Field::<u64>(Variant(_5, 0), 0);
_9 = &_10;
_5 = Adt21::Variant1 { fld0: 8943_u16 };
_10 = [63888660423821798871942303771400419639_i128,48190258137206506221152964915320086556_i128,136701057553463430869935423946978219310_i128,107106853136610033988274367737192123988_i128];
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
place!(Field::<u16>(Variant(_5, 1), 0)) = 87593221530605553377023887099802853087_u128 as u16;
_3 = '\u{bb7ba}';
RET = [43883497566806741990235640777139081447_u128,165596431926984029826654856362118753267_u128,283370353832274940335234373970400617873_u128,329267056648386291944140569372953712230_u128,247259312791212062142822846500293225116_u128];
_14.3 = 5_usize as f64;
RET = _4;
_14.2 = 9140110610261983914_usize as u32;
_12 = 9223372036854775807_isize as u64;
_12 = 7818556801315723626_usize as u64;
_15 = [52_u8];
_9 = &_10;
RET = [251327653218951542381623121273587205007_u128,96872602277187901190022057383377149397_u128,50379265601241924587806738657177575251_u128,331380008969076535131878610180816904399_u128,150103990705041517273991663799565958524_u128];
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
Call(RET = fn12(Move(_9), Move(_13), (*_9), (*_13), _3, _1, Field::<u16>(Variant(_5, 1), 0), _4), ReturnTo(bb4), UnwindUnreachable())
}
bb9 = {
_3 = '\u{777e8}';
RET = [107552137073891240529312354989615448674_u128,115081585800298152746650757971444741489_u128,265667950035994022376986263569151515216_u128,174771595119715951038964562305024154421_u128,38948338659727195404004932115477486955_u128];
RET = [281449217674113324318778080077624465233_u128,274019349572909105153939066159636654114_u128,18125605158633818083479414012088008960_u128,310769874715228840957972519408630239304_u128,43739176874861879271409200523288080347_u128];
RET = [261282149240566199393392460805989601461_u128,299817481856508439516427499177124681966_u128,66889876841662277366253128384348247631_u128,297197411224130950751018181453974903434_u128,292344692258704977155991515035386371522_u128];
_3 = '\u{a9da}';
_4 = RET;
Goto(bb2)
}
bb10 = {
_14.1 = [25218654463224818122969828049712073135_i128,120712558264094237432810624595822131062_i128,114515168747267207915719546104105726720_i128,(-146246673762081125525637774717577019298_i128)];
_14.2 = 3117618922_u32 - 210026923_u32;
_13 = &place!(Field::<u16>(Variant(_5, 1), 0));
_14.3 = 9223372036854775807_isize as f64;
Goto(bb5)
}
bb11 = {
_25.0 = [111701396983119611555029208310003812045_i128,(-37030519433742625005781722308263786841_i128),(-90288876115234758650653797717722439245_i128),115019749157267444577812875775352586003_i128];
_23 = _14.2 as isize;
Goto(bb12)
}
bb12 = {
_22 = [(-1643234803_i32)];
_21 = _23 - Field::<isize>(Variant(_5, 2), 2);
Call(_30 = dump_var(11_usize, 21_usize, Move(_21), 23_usize, Move(_23), 5_usize, _5, 14_usize, _14), ReturnTo(t0), UnwindUnreachable())
}
t0 = {
_9 = &_10;
_10 = _25.0;
_9 = &_25.0;
_15 = [29_u8];
_20 = &_3;
_12 = true as u64;
_14.2 = 3413750678_u32 ^ 3788463960_u32;
_10 = (*_9);
_25 = (_10,);
_26 = !Field::<u128>(Variant(_5, 2), 0);
Goto(bb13)
}
bb13 = {
_14 = (_12, _10, 1219259876_u32, Field::<f64>(Variant(_5, 2), 1));
_14.3 = _16 as f64;
_4 = [_26,Field::<u128>(Variant(_5, 2), 0),Field::<u128>(Variant(_5, 2), 0),_26,Field::<u128>(Variant(_5, 2), 0)];
RET = [_26,Field::<u128>(Variant(_5, 2), 0),Field::<u128>(Variant(_5, 2), 0),_26,_26];
_23 = _21 | _21;
_9 = &_14.1;
Goto(bb14)
}
bb14 = {
RET = [_26,Field::<u128>(Variant(_5, 2), 0),Field::<u128>(Variant(_5, 2), 0),_26,_26];
_8 = &_25;
_14.3 = 12688107698368763961_usize as f64;
SetDiscriminant(_5, 1);
_16 = -(-29656_i16);
_14.3 = _21 as f64;
_26 = 34482380520238291957246249154581351184_u128;
_1 = [false,false,false,false,true];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(11_usize, 16_usize, Move(_16), 15_usize, Move(_15), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(11_usize, 12_usize, Move(_12), 10_usize, Move(_10), 31_usize, _31, 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: &'static [i128; 4],mut _2: &'static u16,mut _3: [i128; 4],mut _4: u16,mut _5: char,mut _6: [bool; 5],mut _7: u16,mut _8: [u128; 5]) -> [u128; 5] {
mir! {
type RET = [u128; 5];
let _9: *mut [usize; 1];
let _10: bool;
let _11: &'static [i128; 4];
let _12: u128;
let _13: ([isize; 5], i16);
let _14: &'static isize;
let _15: &'static i8;
let _16: char;
let _17: [i64; 5];
let _18: i128;
let _19: f64;
let _20: i64;
let _21: (*mut ([i32; 1],),);
let _22: [i32; 1];
let _23: [u8; 3];
let _24: i64;
let _25: u128;
let _26: *mut &'static [usize; 1];
let _27: [usize; 6];
let _28: char;
let _29: char;
let _30: &'static i8;
let _31: char;
let _32: *const [i32; 1];
let _33: *mut usize;
let _34: &'static isize;
let _35: i8;
let _36: *const [i32; 1];
let _37: &'static &'static isize;
let _38: bool;
let _39: isize;
let _40: ();
let _41: ();
{
_6 = [true,true,true,false,false];
_7 = _4;
_5 = '\u{c2b0}';
_2 = &_4;
_6 = [false,true,true,true,true];
RET = [193375862957884783662122931176292859541_u128,315932585996491317130505654262526052436_u128,113144215329847990648280770687439870722_u128,228244392026129738937232663635797896200_u128,241893893119637055937038904122202113402_u128];
RET = _8;
_5 = '\u{817f7}';
_8 = [144486261918370568053787969418836547030_u128,79923579198496410488994711343101070541_u128,24769345697459081221680550070952170229_u128,151654622178103834022732980566766680770_u128,39250328322718615968661061991751283082_u128];
Call(_3 = fn13(Move(_2), RET, _7, _7, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = &_3;
_4 = !_7;
_4 = _7 | _7;
_3 = [(-159221629181190311005269815937727792906_i128),(-124683851775305301297405763843303648981_i128),119763207422959517914395277464809704970_i128,(-26824475339035071289610254918996231900_i128)];
_1 = &_3;
_2 = &_7;
Goto(bb2)
}
bb2 = {
_6 = [true,false,false,false,true];
_6 = [true,false,true,true,false];
RET = [148709288843844176123315626652385709697_u128,148009638107809698168264356304693287608_u128,255111042531359641970959814375632699119_u128,215410050793958601948031717595553977605_u128,257964391018576759408805613374996405132_u128];
_4 = 18161076320243707667_usize as u16;
_1 = &(*_1);
_13.0 = [(-101_isize),9223372036854775807_isize,6_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_3 = [116483124780638355428604125411965288368_i128,105758714488927434769708078941719913421_i128,(-49497823722243799629468503110877007201_i128),(-67193844784654692784340917580874084971_i128)];
_8 = [200360014699825830057646156416054278036_u128,42766292078179762182893715147234104104_u128,323973029533264875811498844775344958805_u128,125902054947030759737357208332323539741_u128,68366273214379448184497879594252278285_u128];
_11 = &_3;
_1 = Move(_11);
_13.1 = 264749347719213971952965184898747417787_u128 as i16;
_7 = 5_usize as u16;
_4 = _7 - _7;
_5 = '\u{cf3fc}';
_3 = [(-22447306098696109503723636745925468335_i128),(-16166850108257262941214799681833462147_i128),57680640688562533357509712439279817068_i128,144934308653202579051778922134760217463_i128];
Goto(bb3)
}
bb3 = {
_10 = _13.1 <= _13.1;
_3 = [(-19927301421453265654138414219028698394_i128),8193427432830835485336734450615593685_i128,142996891684766875430319123700982527865_i128,(-18407852655483371904526466417763444087_i128)];
_11 = &_3;
_10 = true | false;
_2 = &_7;
RET = [213871079925692800050630866256421453488_u128,170600602582365444159656372804836614660_u128,47874175055812798120899107422305260538_u128,197638420221802077896783438588529839778_u128,101240014418153684812751447905725981643_u128];
_12 = 88188954500509884713747686127455535361_u128 * 79904316295067973082093689910784865131_u128;
_17 = [4223533948297207529_i64,(-6776350753148119407_i64),5603710440183586220_i64,7206377115278465213_i64,648834657124874523_i64];
RET = [_12,_12,_12,_12,_12];
_4 = !(*_2);
Goto(bb4)
}
bb4 = {
_1 = &(*_11);
_2 = &(*_2);
_10 = false;
_7 = 86_i8 as u16;
_18 = 105593286736625147774592034389472346460_i128 | (-3282761379032968514460883308034627186_i128);
_2 = &_4;
_16 = _5;
_7 = 1571432253353012759_i64 as u16;
_8 = [_12,_12,_12,_12,_12];
_19 = _7 as f64;
_8 = [_12,_12,_12,_12,_12];
RET = [_12,_12,_12,_12,_12];
_8 = RET;
_13.1 = -(-9499_i16);
_11 = &(*_1);
_22 = [(-1889170802_i32)];
Goto(bb5)
}
bb5 = {
_17 = [321695711336068914_i64,2351325063902708290_i64,2738611144263244804_i64,7172993576636296264_i64,6066158024090524451_i64];
_23 = [61_u8,210_u8,25_u8];
_6 = [_10,_10,_10,_10,_10];
_20 = !(-2405479573775080749_i64);
_13.0 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6 = [_10,_10,_10,_10,_10];
RET = [_12,_12,_12,_12,_12];
_8 = [_12,_12,_12,_12,_12];
_8 = RET;
RET = _8;
_20 = 7167035518773399927_i64 ^ 544349616732029055_i64;
_8 = [_12,_12,_12,_12,_12];
_17 = [_20,_20,_20,_20,_20];
_1 = &(*_11);
_6 = [_10,_10,_10,_10,_10];
_20 = !(-6854576027850856985_i64);
_11 = &_3;
Goto(bb6)
}
bb6 = {
_4 = !_7;
_10 = !false;
_7 = _4;
RET = _8;
_4 = _10 as u16;
_13.1 = !(-18838_i16);
_4 = 46_i8 as u16;
Call(_6 = fn14(_18, (*_11), _23, _5, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7 = _16 as u16;
_12 = 96_i8 as u128;
_16 = _5;
_13.1 = -407_i16;
_13.0 = [(-90_isize),54_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_11 = &_3;
_2 = &_4;
Call(_17 = fn15(Move(_2), _13.0, _13, _13.0, (*_11)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = [_18,_18,_18,_18];
_25 = _12;
_1 = &_3;
_10 = false;
_24 = _20;
_10 = true ^ true;
Goto(bb9)
}
bb9 = {
_3 = [_18,_18,_18,_18];
_8 = [_12,_25,_25,_25,_12];
_12 = _25;
_20 = _19 as i64;
_13.0 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,24_isize,9223372036854775807_isize];
_29 = _5;
_19 = 1953174437_i32 as f64;
_8 = RET;
_25 = !_12;
_7 = 1801241995022013465_u64 as u16;
_1 = &_3;
_3 = [_18,_18,_18,_18];
_5 = _29;
_2 = &_4;
_28 = _16;
_3 = [_18,_18,_18,_18];
_28 = _29;
_12 = _25 >> (*_2);
Goto(bb10)
}
bb10 = {
_1 = &_3;
_27 = [13958401654910747691_usize,5_usize,1075958018934356708_usize,491584945740299984_usize,4605708613636306443_usize,5_usize];
Goto(bb11)
}
bb11 = {
_31 = _28;
_6 = [_10,_10,_10,_10,_10];
_17 = [_20,_20,_20,_24,_24];
_22 = [(-1152680289_i32)];
_16 = _5;
_29 = _16;
_22 = [(-1346248804_i32)];
_20 = 2652231992_u32 as i64;
_24 = _20 << _4;
_10 = true ^ true;
_19 = 3603505807_u32 as f64;
_24 = _20 ^ _20;
_18 = (-39510194598336989329448978974902954457_i128) ^ (-147850780855389537849438164862680524660_i128);
_8 = [_12,_12,_25,_12,_12];
_16 = _29;
_22 = [1782162058_i32];
_32 = core::ptr::addr_of!(_22);
_18 = (-142046021658158260171566404732481850882_i128);
_25 = !_12;
_10 = true;
_13.1 = (-29147_i16) ^ (-29355_i16);
_23 = [170_u8,237_u8,103_u8];
_22 = [(-1215864780_i32)];
_8 = [_25,_25,_25,_25,_12];
_24 = !_20;
_28 = _16;
Goto(bb12)
}
bb12 = {
_11 = Move(_1);
(*_32) = [(-772431735_i32)];
_36 = Move(_32);
_5 = _31;
_30 = &_35;
_29 = _28;
_8 = [_12,_12,_25,_12,_12];
_22 = [(-1603896600_i32)];
match _18 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
198236345262780203291808202699286360574 => bb18,
_ => bb17
}
}
bb13 = {
_31 = _28;
_6 = [_10,_10,_10,_10,_10];
_17 = [_20,_20,_20,_24,_24];
_22 = [(-1152680289_i32)];
_16 = _5;
_29 = _16;
_22 = [(-1346248804_i32)];
_20 = 2652231992_u32 as i64;
_24 = _20 << _4;
_10 = true ^ true;
_19 = 3603505807_u32 as f64;
_24 = _20 ^ _20;
_18 = (-39510194598336989329448978974902954457_i128) ^ (-147850780855389537849438164862680524660_i128);
_8 = [_12,_12,_25,_12,_12];
_16 = _29;
_22 = [1782162058_i32];
_32 = core::ptr::addr_of!(_22);
_18 = (-142046021658158260171566404732481850882_i128);
_25 = !_12;
_10 = true;
_13.1 = (-29147_i16) ^ (-29355_i16);
_23 = [170_u8,237_u8,103_u8];
_22 = [(-1215864780_i32)];
_8 = [_25,_25,_25,_25,_12];
_24 = !_20;
_28 = _16;
Goto(bb12)
}
bb14 = {
_10 = _13.1 <= _13.1;
_3 = [(-19927301421453265654138414219028698394_i128),8193427432830835485336734450615593685_i128,142996891684766875430319123700982527865_i128,(-18407852655483371904526466417763444087_i128)];
_11 = &_3;
_10 = true | false;
_2 = &_7;
RET = [213871079925692800050630866256421453488_u128,170600602582365444159656372804836614660_u128,47874175055812798120899107422305260538_u128,197638420221802077896783438588529839778_u128,101240014418153684812751447905725981643_u128];
_12 = 88188954500509884713747686127455535361_u128 * 79904316295067973082093689910784865131_u128;
_17 = [4223533948297207529_i64,(-6776350753148119407_i64),5603710440183586220_i64,7206377115278465213_i64,648834657124874523_i64];
RET = [_12,_12,_12,_12,_12];
_4 = !(*_2);
Goto(bb4)
}
bb15 = {
_4 = !_7;
_10 = !false;
_7 = _4;
RET = _8;
_4 = _10 as u16;
_13.1 = !(-18838_i16);
_4 = 46_i8 as u16;
Call(_6 = fn14(_18, (*_11), _23, _5, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
_6 = [true,false,false,false,true];
_6 = [true,false,true,true,false];
RET = [148709288843844176123315626652385709697_u128,148009638107809698168264356304693287608_u128,255111042531359641970959814375632699119_u128,215410050793958601948031717595553977605_u128,257964391018576759408805613374996405132_u128];
_4 = 18161076320243707667_usize as u16;
_1 = &(*_1);
_13.0 = [(-101_isize),9223372036854775807_isize,6_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_3 = [116483124780638355428604125411965288368_i128,105758714488927434769708078941719913421_i128,(-49497823722243799629468503110877007201_i128),(-67193844784654692784340917580874084971_i128)];
_8 = [200360014699825830057646156416054278036_u128,42766292078179762182893715147234104104_u128,323973029533264875811498844775344958805_u128,125902054947030759737357208332323539741_u128,68366273214379448184497879594252278285_u128];
_11 = &_3;
_1 = Move(_11);
_13.1 = 264749347719213971952965184898747417787_u128 as i16;
_7 = 5_usize as u16;
_4 = _7 - _7;
_5 = '\u{cf3fc}';
_3 = [(-22447306098696109503723636745925468335_i128),(-16166850108257262941214799681833462147_i128),57680640688562533357509712439279817068_i128,144934308653202579051778922134760217463_i128];
Goto(bb3)
}
bb17 = {
_17 = [321695711336068914_i64,2351325063902708290_i64,2738611144263244804_i64,7172993576636296264_i64,6066158024090524451_i64];
_23 = [61_u8,210_u8,25_u8];
_6 = [_10,_10,_10,_10,_10];
_20 = !(-2405479573775080749_i64);
_13.0 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6 = [_10,_10,_10,_10,_10];
RET = [_12,_12,_12,_12,_12];
_8 = [_12,_12,_12,_12,_12];
_8 = RET;
RET = _8;
_20 = 7167035518773399927_i64 ^ 544349616732029055_i64;
_8 = [_12,_12,_12,_12,_12];
_17 = [_20,_20,_20,_20,_20];
_1 = &(*_11);
_6 = [_10,_10,_10,_10,_10];
_20 = !(-6854576027850856985_i64);
_11 = &_3;
Goto(bb6)
}
bb18 = {
_2 = &_4;
_7 = (*_2);
_25 = _12;
_2 = &_4;
_1 = &_3;
_16 = _5;
_13.0 = [(-9223372036854775808_isize),(-82_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_37 = &_34;
_13.0 = [76_isize,100_isize,68_isize,(-88_isize),(-9223372036854775808_isize)];
Goto(bb19)
}
bb19 = {
Call(_40 = dump_var(12_usize, 13_usize, Move(_13), 25_usize, Move(_25), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(12_usize, 23_usize, Move(_23), 20_usize, Move(_20), 22_usize, Move(_22), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_40 = dump_var(12_usize, 27_usize, Move(_27), 17_usize, Move(_17), 41_usize, _41, 41_usize, _41), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: &'static u16,mut _2: [u128; 5],mut _3: u16,mut _4: u16,mut _5: [bool; 5]) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _6: isize;
let _7: isize;
let _8: i64;
let _9: char;
let _10: ([i16; 4], f32, char);
let _11: (i8,);
let _12: i16;
let _13: *mut usize;
let _14: *mut *const usize;
let _15: f32;
let _16: isize;
let _17: f32;
let _18: isize;
let _19: [i16; 5];
let _20: Adt64;
let _21: f32;
let _22: i16;
let _23: u32;
let _24: &'static &'static isize;
let _25: [usize; 7];
let _26: &'static [usize; 1];
let _27: ();
let _28: ();
{
_2 = [153611259760152523938111382551558900807_u128,37415002550925872080087493748634035169_u128,338130637543512354399594740207152981791_u128,67537282296988518722774707422981536021_u128,287912617905732937102423869263397986502_u128];
_1 = &_3;
RET = [(-12059571730950284653777111390047435706_i128),111551345493959782999409063174307181028_i128,89926315652221724942402364527413588086_i128,(-156527946835053289031031651328337753556_i128)];
_2 = [330297113032695876857585996614558345761_u128,60578047233789528005647535453889313296_u128,306212686470761025133567400597998841399_u128,50810666968990399149437170725104247018_u128,217813364334154443619290268858399622488_u128];
_5 = [false,true,false,false,false];
Goto(bb1)
}
bb1 = {
RET = [17390117862513372137386072502602904719_i128,(-165997037125380623654225060038868081481_i128),(-36546381572106494616215719759181294427_i128),49839939078356246722544360102096748288_i128];
_5 = [true,true,true,true,true];
_4 = _3 << (*_1);
_7 = 9223372036854775807_isize & (-9223372036854775808_isize);
_7 = (-80_isize);
_3 = !_4;
_8 = (-344431863671445284_i64);
_2 = [316668845289257986767651261772281143255_u128,139400021116952043373741844761456731728_u128,108578891302805898645351124465681679110_u128,323388874109038356837998396173433570132_u128,305666452445008951828286984335495059530_u128];
_1 = &_4;
_6 = !_7;
_8 = (-9075087205635724862_i64);
_9 = '\u{b1f26}';
_3 = 4702702693567585834_u64 as u16;
Goto(bb2)
}
bb2 = {
_1 = &(*_1);
RET = [(-105251270632174451694494873209701996505_i128),(-5028234464773315885716707657615487301_i128),95756290297417143290775024731684590577_i128,(-53173455502348211991608992928299607345_i128)];
_1 = &_4;
_7 = _6;
_3 = _4;
_7 = -_6;
RET = [(-72314713813700297523706757568089678247_i128),(-112782149599031213425026776704286428504_i128),127315209753920283694078328307711874057_i128,(-76061009904060609559472156552775246380_i128)];
_8 = (-3081895314508093248_i64);
_2 = [231427247305846787943634566109076768327_u128,247245759725480317937090854165583709592_u128,267092560894615848931475793169019522699_u128,39115330261422570576543087352093529509_u128,273547150014135702239023373460510298678_u128];
_1 = &(*_1);
RET = [45714979353238818822529259312583014413_i128,124225595835457107486518100025394839353_i128,12234300419373812071799349969061681516_i128,35214725684439601402765011937507470928_i128];
_1 = &(*_1);
_2 = [9157401393773663497285443002890609525_u128,244909366777314732057677138054552479871_u128,9976852535861643797368949265700722974_u128,120524948348796485802226951466858256654_u128,315551689621593378395979680162859055295_u128];
RET = [169197515589334518624519786208933892246_i128,(-82379405441563333604559836790594021760_i128),(-54717675126004723447369449812766815274_i128),(-146665842256521410723569251740648536586_i128)];
_10.1 = (*_1) as f32;
_2 = [175702195955862607892597285092569755466_u128,280799520807181382902991990450263175465_u128,80762571599678554452490786059838144090_u128,64473930194988468233318157573896125569_u128,237819533841342076837226804232664437898_u128];
_5 = [true,false,true,false,true];
_11 = (115_i8,);
_9 = '\u{12972}';
_10.0 = [21647_i16,13195_i16,(-997_i16),20048_i16];
_3 = _4;
_8 = 684953021443594389_i64 | (-2861861542859871357_i64);
_8 = 8021184215453394907_i64 << (*_1);
RET = [(-37787168906662228787656089207109872172_i128),(-163435668237794221664744586414862261831_i128),36186254094410621913591020168196076220_i128,(-42036114136766457577966908591932382337_i128)];
_5 = [true,true,true,false,true];
match _11.0 {
0 => bb3,
1 => bb4,
2 => bb5,
115 => bb7,
_ => bb6
}
}
bb3 = {
RET = [17390117862513372137386072502602904719_i128,(-165997037125380623654225060038868081481_i128),(-36546381572106494616215719759181294427_i128),49839939078356246722544360102096748288_i128];
_5 = [true,true,true,true,true];
_4 = _3 << (*_1);
_7 = 9223372036854775807_isize & (-9223372036854775808_isize);
_7 = (-80_isize);
_3 = !_4;
_8 = (-344431863671445284_i64);
_2 = [316668845289257986767651261772281143255_u128,139400021116952043373741844761456731728_u128,108578891302805898645351124465681679110_u128,323388874109038356837998396173433570132_u128,305666452445008951828286984335495059530_u128];
_1 = &_4;
_6 = !_7;
_8 = (-9075087205635724862_i64);
_9 = '\u{b1f26}';
_3 = 4702702693567585834_u64 as u16;
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
_11 = ((-11_i8),);
_10.2 = _9;
_5 = [true,false,false,true,true];
_5 = [true,true,false,false,true];
_3 = _4 ^ (*_1);
_10.1 = _8 as f32;
_7 = _6 - _6;
_1 = &(*_1);
_3 = !(*_1);
_10.0 = [25918_i16,15137_i16,9066_i16,(-13614_i16)];
_1 = &_4;
_6 = _7 - _7;
_11 = ((-62_i8),);
_1 = &(*_1);
_1 = &(*_1);
_2 = [162768480247164530259149217637115001305_u128,113392432447938103715999842203388163412_u128,316370674641832680155980707894462155004_u128,252525519650444829403435231156463090639_u128,95453175258657909485783677636852615972_u128];
_10.1 = _11.0 as f32;
match _11.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211394 => bb8,
_ => bb5
}
}
bb8 = {
_11.0 = 9087517550521871472_u64 as i8;
_3 = false as u16;
RET = [103935999834365655853352558288482862175_i128,29240947633739901559686679223770211852_i128,65913182361553288585815134432445571999_i128,168780328017684863260387347622965228500_i128];
_11.0 = 16161259943055300969_u64 as i8;
_7 = false as isize;
_9 = _10.2;
_9 = _10.2;
_6 = 754582343_u32 as isize;
_1 = &_4;
_4 = !_3;
_10.0 = [(-16787_i16),25281_i16,(-5038_i16),(-20884_i16)];
_11.0 = 124_i8 + 8_i8;
_10.1 = 31696_i16 as f32;
RET = [150472673801277588187261565118449651697_i128,61378851365594495170069948343748764625_i128,(-154098210368503786866097651656965690069_i128),164048922304303687470400815386371241033_i128];
_1 = &_4;
_3 = !_4;
_10.0 = [32519_i16,(-32025_i16),(-1442_i16),12550_i16];
RET = [13607198661508060957621705977170864724_i128,116192910602405226138646459004702065402_i128,(-33767913030166109120297699005444606793_i128),131661525118692712530164481698716672771_i128];
_9 = _10.2;
_4 = 81986167388791427705049908870824316628_i128 as u16;
_1 = &_4;
Call(_8 = core::intrinsics::bswap((-972984919111349515_i64)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = [(-130653561185926037923899713692479999642_i128),(-104481090035283987227110895273553749310_i128),154507412309672326228286138351387899177_i128,32846707616123073265694541868691633127_i128];
_8 = (-533066239998344164_i64) ^ 8809881531794398275_i64;
RET = [(-28424048480319093067036358487668491715_i128),(-165334062888296356014839975714733458083_i128),(-134720083408929296651480284128409761452_i128),25478717171884477621907353102138095346_i128];
_11 = ((-73_i8),);
_10.0 = [23213_i16,12179_i16,(-28333_i16),5357_i16];
RET = [83722781001240187798488180403463755753_i128,97807286867455122293017916050992216149_i128,108779497246875538233311024541287275508_i128,(-40015895995340013569165804693169896304_i128)];
Goto(bb10)
}
bb10 = {
_5 = [false,false,false,true,true];
RET = [4190243304329471978755267489035108356_i128,(-47101670039682804145513681092012949762_i128),68452725651723880840552740330081587887_i128,4610432429310799412639041616788171883_i128];
_10.1 = _11.0 as f32;
_11 = (14_i8,);
_12 = (-229823100_i32) as i16;
_15 = _10.1;
_1 = &_4;
_4 = _3;
RET = [(-86171037011344382483798198434766186860_i128),83105081481675055239580612110541502855_i128,113760787242135405874533779230512066948_i128,86987077752166475794961760158203780883_i128];
_10.2 = _9;
_8 = 1516976689_u32 as i64;
RET = [(-155452437248438436535825822482588341924_i128),23983521314255022908283959264171691359_i128,(-33896000511896970900017417265152384781_i128),(-124295501706855526978761442894451333033_i128)];
RET = [(-58524848556807796595213225992135650964_i128),(-95922313727646682460733608726645145855_i128),(-148928708546476270698260795075389106928_i128),18915863750951866239842029023812817847_i128];
_2 = [273180001348350201797517802427502352620_u128,107292622164499777439490119412677896961_u128,180018350607009210598114033076661350876_u128,204428303913582212560767864412160599591_u128,303319310387402178336697054848677346532_u128];
_16 = _6 >> _12;
Goto(bb11)
}
bb11 = {
_5 = [true,false,true,true,false];
_10.1 = 2685894094358699696_u64 as f32;
_1 = &_3;
_17 = _10.1 + _15;
_9 = _10.2;
_10.0 = [_12,_12,_12,_12];
_3 = _4 - _4;
_19 = [_12,_12,_12,_12,_12];
_6 = _16 << _3;
_9 = _10.2;
_4 = !_3;
_1 = &_4;
_3 = (*_1);
_3 = !(*_1);
_11 = (46_i8,);
_18 = _6;
_10.1 = _17 - _17;
_2 = [112973787184180400805434801131356015652_u128,248380989772063332948706534899399530299_u128,42644833604759291641193608794315390453_u128,209262813968516367737531121781917815585_u128,141075821224898144870719254601476796690_u128];
_10.0 = [_12,_12,_12,_12];
_4 = 7340198632919020996_u64 as u16;
_16 = -_6;
match _11.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb8,
4 => bb10,
5 => bb12,
46 => bb14,
_ => bb13
}
}
bb12 = {
_1 = &(*_1);
RET = [(-105251270632174451694494873209701996505_i128),(-5028234464773315885716707657615487301_i128),95756290297417143290775024731684590577_i128,(-53173455502348211991608992928299607345_i128)];
_1 = &_4;
_7 = _6;
_3 = _4;
_7 = -_6;
RET = [(-72314713813700297523706757568089678247_i128),(-112782149599031213425026776704286428504_i128),127315209753920283694078328307711874057_i128,(-76061009904060609559472156552775246380_i128)];
_8 = (-3081895314508093248_i64);
_2 = [231427247305846787943634566109076768327_u128,247245759725480317937090854165583709592_u128,267092560894615848931475793169019522699_u128,39115330261422570576543087352093529509_u128,273547150014135702239023373460510298678_u128];
_1 = &(*_1);
RET = [45714979353238818822529259312583014413_i128,124225595835457107486518100025394839353_i128,12234300419373812071799349969061681516_i128,35214725684439601402765011937507470928_i128];
_1 = &(*_1);
_2 = [9157401393773663497285443002890609525_u128,244909366777314732057677138054552479871_u128,9976852535861643797368949265700722974_u128,120524948348796485802226951466858256654_u128,315551689621593378395979680162859055295_u128];
RET = [169197515589334518624519786208933892246_i128,(-82379405441563333604559836790594021760_i128),(-54717675126004723447369449812766815274_i128),(-146665842256521410723569251740648536586_i128)];
_10.1 = (*_1) as f32;
_2 = [175702195955862607892597285092569755466_u128,280799520807181382902991990450263175465_u128,80762571599678554452490786059838144090_u128,64473930194988468233318157573896125569_u128,237819533841342076837226804232664437898_u128];
_5 = [true,false,true,false,true];
_11 = (115_i8,);
_9 = '\u{12972}';
_10.0 = [21647_i16,13195_i16,(-997_i16),20048_i16];
_3 = _4;
_8 = 684953021443594389_i64 | (-2861861542859871357_i64);
_8 = 8021184215453394907_i64 << (*_1);
RET = [(-37787168906662228787656089207109872172_i128),(-163435668237794221664744586414862261831_i128),36186254094410621913591020168196076220_i128,(-42036114136766457577966908591932382337_i128)];
_5 = [true,true,true,false,true];
match _11.0 {
0 => bb3,
1 => bb4,
2 => bb5,
115 => bb7,
_ => bb6
}
}
bb13 = {
Return()
}
bb14 = {
_22 = _12;
_5 = [false,false,true,false,true];
_11 = ((-119_i8),);
_3 = !_4;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(13_usize, 18_usize, Move(_18), 19_usize, Move(_19), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(13_usize, 16_usize, Move(_16), 6_usize, Move(_6), 12_usize, Move(_12), 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i128,mut _2: [i128; 4],mut _3: [u8; 3],mut _4: char,mut _5: [u8; 3]) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _6: *const u32;
let _7: u8;
let _8: i64;
let _9: i8;
let _10: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _11: isize;
let _12: i8;
let _13: &'static ([i128; 4],);
let _14: *mut ([i32; 1],);
let _15: f64;
let _16: u128;
let _17: *mut [usize; 1];
let _18: ();
let _19: ();
{
RET = [false,true,true,false,false];
RET = [true,true,true,true,true];
_1 = 283081199037631601112355796999840608126_u128 as i128;
_7 = 5_usize as u8;
_4 = '\u{ba1cb}';
_3 = _5;
_3 = [_7,_7,_7];
_4 = '\u{4dbbc}';
Goto(bb1)
}
bb1 = {
_3 = [_7,_7,_7];
_4 = '\u{1991d}';
_2 = [_1,_1,_1,_1];
_3 = [_7,_7,_7];
_1 = (-140610897218185261825496475029871049069_i128);
_3 = [_7,_7,_7];
_7 = 96_u8;
_2 = [_1,_1,_1,_1];
_3 = [_7,_7,_7];
_8 = 8724668122551243028_i64 * (-4110528268526165085_i64);
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
96 => bb8,
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
_10.1.2 = !3863148842_u32;
_4 = '\u{f685f}';
_10.3 = (_2,);
_2 = [_1,_1,_1,_1];
_10.1.1 = [_1,_1,_1,_1];
_8 = (-1035776524680218_i64) + 27790008579975334_i64;
RET = [false,false,false,true,true];
_10.2.0 = [(-1993568678_i32)];
_10.1.2 = 4132172336_u32 | 556581848_u32;
_13 = &_10.3;
_11 = 3147_u16 as isize;
_1 = 6778_i16 as i128;
_14 = core::ptr::addr_of_mut!(_10.2);
_12 = 39_i8;
(*_14).0 = [1687204210_i32];
_2 = [_1,_1,_1,_1];
Goto(bb9)
}
bb9 = {
_5 = [_7,_7,_7];
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
96 => bb18,
_ => bb17
}
}
bb10 = {
_10.1.2 = !3863148842_u32;
_4 = '\u{f685f}';
_10.3 = (_2,);
_2 = [_1,_1,_1,_1];
_10.1.1 = [_1,_1,_1,_1];
_8 = (-1035776524680218_i64) + 27790008579975334_i64;
RET = [false,false,false,true,true];
_10.2.0 = [(-1993568678_i32)];
_10.1.2 = 4132172336_u32 | 556581848_u32;
_13 = &_10.3;
_11 = 3147_u16 as isize;
_1 = 6778_i16 as i128;
_14 = core::ptr::addr_of_mut!(_10.2);
_12 = 39_i8;
(*_14).0 = [1687204210_i32];
_2 = [_1,_1,_1,_1];
Goto(bb9)
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
Return()
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_3 = [_7,_7,_7];
_4 = '\u{1991d}';
_2 = [_1,_1,_1,_1];
_3 = [_7,_7,_7];
_1 = (-140610897218185261825496475029871049069_i128);
_3 = [_7,_7,_7];
_7 = 96_u8;
_2 = [_1,_1,_1,_1];
_3 = [_7,_7,_7];
_8 = 8724668122551243028_i64 * (-4110528268526165085_i64);
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
96 => bb8,
_ => bb7
}
}
bb18 = {
_10.1.0 = 10025328659772591387_u64 >> _8;
RET = [false,true,true,true,false];
_15 = _11 as f64;
_16 = _4 as u128;
RET = [true,true,false,true,true];
RET = [true,false,true,true,true];
_7 = false as u8;
_14 = core::ptr::addr_of_mut!((*_14));
_5 = [_7,_7,_7];
_5 = [_7,_7,_7];
Goto(bb19)
}
bb19 = {
Call(_18 = dump_var(14_usize, 12_usize, Move(_12), 4_usize, Move(_4), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_18 = dump_var(14_usize, 16_usize, Move(_16), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static u16,mut _2: [isize; 5],mut _3: ([isize; 5], i16),mut _4: [isize; 5],mut _5: [i128; 4]) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _6: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],));
let _7: i16;
let _8: u64;
let _9: char;
let _10: *mut *const usize;
let _11: u32;
let _12: isize;
let _13: *const [i32; 1];
let _14: i32;
let _15: *const u128;
let _16: u128;
let _17: f64;
let _18: char;
let _19: u16;
let _20: f32;
let _21: *const [i32; 1];
let _22: isize;
let _23: char;
let _24: f64;
let _25: ();
let _26: ();
{
_3.0 = [102_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-27_isize),9223372036854775807_isize];
RET = [(-8751324121653099357_i64),(-7946244385037468198_i64),(-851822103112019563_i64),4556119895126651647_i64,8504826904066551233_i64];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),110_isize,9223372036854775807_isize];
_5 = [(-11116217965585609811896038861856652698_i128),(-59070189287878666527603282682267860094_i128),19303267153048752935748551180662085501_i128,27870826810136645805701774073110870517_i128];
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_4 = [(-77_isize),9223372036854775807_isize,9223372036854775807_isize,92_isize,(-9223372036854775808_isize)];
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-34_isize),9223372036854775807_isize,80_isize];
Call(_2 = fn16(_3, _3.0, _3.0, _4, _3, _3, _4, _3.0, _3, _3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = (_4, (-16790_i16));
_4 = _2;
_3.1 = -11059_i16;
RET = [2965068753362750458_i64,(-3155572401715224387_i64),7557607435599170238_i64,(-8360290660200839627_i64),(-806056740919717353_i64)];
_3.1 = (-7659_i16);
_6.1.2 = 3464427693_u32 | 1399941875_u32;
_6.2.0 = [(-779289330_i32)];
_3 = (_2, (-16405_i16));
_6.1.1 = [106398692901069182119146854657488552550_i128,(-90586485097819667085074621190736582563_i128),(-94377734314251307065957468959308897913_i128),(-77788300772841217362821464129024748626_i128)];
_2 = [(-76_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-58_isize),(-9223372036854775808_isize)];
RET = [7520537422730576621_i64,(-8373440891379545481_i64),7646245466930408696_i64,(-4628476822334782908_i64),(-8461425554804363470_i64)];
_6.1.1 = _5;
_6.3.0 = [142745606449714672647352447588101252017_i128,167769499904695368646987439614220890052_i128,11025030483366958500997321801964187841_i128,(-165473439746812956628704670296679707775_i128)];
_5 = [134216922249280506575203091788397326774_i128,41882735564554989299867114288628402753_i128,(-107993622822704109259382935440101645278_i128),(-129677601265258770463645833757247474261_i128)];
_2 = [(-9223372036854775808_isize),(-57_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = [3132730287803132364880451345525072278_i128,99726768576139694638937330233821962400_i128,59281737685028476538385149693940214803_i128,(-123588231548549482718760507343192109738_i128)];
RET = [2613568477619411056_i64,(-6938759154127919438_i64),(-6340252087641427757_i64),7782316368967652016_i64,3397354521313518239_i64];
_6.1.2 = 17474793127827496911_usize as u32;
_5 = [13071100105514945029166721857864481122_i128,4736235824331338995040170399418033449_i128,(-139270363320661686516912929806684514172_i128),(-125901404280771477861344402224041182146_i128)];
_6.1.3 = 168048687981454986588808202233824445397_u128 as f64;
_4 = [(-109_isize),120_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize];
_3.1 = !(-4425_i16);
_6.1.0 = 15394854003463312125_u64 * 14779896166456281995_u64;
_3 = (_4, (-26257_i16));
_6.1.3 = (-97657878408065617016536791671207422324_i128) as f64;
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
Goto(bb2)
}
bb2 = {
_3.0 = [(-108_isize),117_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_2 = [24_isize,(-9223372036854775808_isize),(-84_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _3.1;
_3.1 = -_7;
_6.1.0 = !5474731873947680229_u64;
_7 = _3.1 ^ _3.1;
_6.1.0 = 1323140530_i32 as u64;
RET = [1506300774341790399_i64,4556899841193058599_i64,641419207734751607_i64,(-6644945702159823276_i64),4543879464843995002_i64];
_4 = [17_isize,(-126_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.3 = (_5,);
_8 = _6.1.0 - _6.1.0;
_4 = _2;
_3.1 = _7 & _7;
_7 = -_3.1;
_5 = [(-12540078705404169360072842660405890424_i128),(-119383927714661093440848854647268366530_i128),120452299236736261724008047234644440503_i128,(-81328974893137680954645356939786499483_i128)];
_5 = _6.3.0;
_2 = [13_isize,9223372036854775807_isize,107_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6.1.0 = _8 >> _7;
_6.1.2 = 3647683887_u32 << _7;
RET = [(-3976392534724999691_i64),(-1643427683507678360_i64),366387540994846596_i64,9157897386443386596_i64,(-3521725103662379602_i64)];
_8 = !_6.1.0;
_6.1.1 = _5;
_6.1.2 = !309634445_u32;
Goto(bb3)
}
bb3 = {
_6.1.2 = 3373892292_u32;
_12 = -39_isize;
_8 = !_6.1.0;
_7 = 94_u8 as i16;
RET = [(-6479808779882462313_i64),4603327341612065580_i64,(-1512623418876320456_i64),(-809154812720361183_i64),(-2763579758227261238_i64)];
_6.1.3 = (-329143321_i32) as f64;
_5 = [(-13708580842014870631517629774242044142_i128),158580438609756397029855118039304884306_i128,62069592975046603449771930458332345189_i128,22915360168238591005021703611825795229_i128];
_6.1.1 = _5;
RET = [(-6006401711035812308_i64),(-4818549631949071158_i64),(-2576428119528639748_i64),2376321534653139622_i64,7057341368209967491_i64];
RET = [(-3437391004083401775_i64),6718605850150545222_i64,652766941368891993_i64,2236800867359189516_i64,5892802160101739694_i64];
_6.3.0 = [44236726361427991139980102127815208094_i128,73647843262534631939463766625651315376_i128,(-96187319286759252245147560786707218003_i128),125127477053979106043341155872757336009_i128];
_6.1.0 = !_8;
_7 = _3.1 ^ _3.1;
_8 = _6.1.0 >> _3.1;
_6.1.0 = _8 & _8;
_3.1 = -_7;
match _6.1.2 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
3373892292 => bb9,
_ => bb8
}
}
bb4 = {
_3.0 = [(-108_isize),117_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_2 = [24_isize,(-9223372036854775808_isize),(-84_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _3.1;
_3.1 = -_7;
_6.1.0 = !5474731873947680229_u64;
_7 = _3.1 ^ _3.1;
_6.1.0 = 1323140530_i32 as u64;
RET = [1506300774341790399_i64,4556899841193058599_i64,641419207734751607_i64,(-6644945702159823276_i64),4543879464843995002_i64];
_4 = [17_isize,(-126_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.3 = (_5,);
_8 = _6.1.0 - _6.1.0;
_4 = _2;
_3.1 = _7 & _7;
_7 = -_3.1;
_5 = [(-12540078705404169360072842660405890424_i128),(-119383927714661093440848854647268366530_i128),120452299236736261724008047234644440503_i128,(-81328974893137680954645356939786499483_i128)];
_5 = _6.3.0;
_2 = [13_isize,9223372036854775807_isize,107_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6.1.0 = _8 >> _7;
_6.1.2 = 3647683887_u32 << _7;
RET = [(-3976392534724999691_i64),(-1643427683507678360_i64),366387540994846596_i64,9157897386443386596_i64,(-3521725103662379602_i64)];
_8 = !_6.1.0;
_6.1.1 = _5;
_6.1.2 = !309634445_u32;
Goto(bb3)
}
bb5 = {
_3 = (_4, (-16790_i16));
_4 = _2;
_3.1 = -11059_i16;
RET = [2965068753362750458_i64,(-3155572401715224387_i64),7557607435599170238_i64,(-8360290660200839627_i64),(-806056740919717353_i64)];
_3.1 = (-7659_i16);
_6.1.2 = 3464427693_u32 | 1399941875_u32;
_6.2.0 = [(-779289330_i32)];
_3 = (_2, (-16405_i16));
_6.1.1 = [106398692901069182119146854657488552550_i128,(-90586485097819667085074621190736582563_i128),(-94377734314251307065957468959308897913_i128),(-77788300772841217362821464129024748626_i128)];
_2 = [(-76_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-58_isize),(-9223372036854775808_isize)];
RET = [7520537422730576621_i64,(-8373440891379545481_i64),7646245466930408696_i64,(-4628476822334782908_i64),(-8461425554804363470_i64)];
_6.1.1 = _5;
_6.3.0 = [142745606449714672647352447588101252017_i128,167769499904695368646987439614220890052_i128,11025030483366958500997321801964187841_i128,(-165473439746812956628704670296679707775_i128)];
_5 = [134216922249280506575203091788397326774_i128,41882735564554989299867114288628402753_i128,(-107993622822704109259382935440101645278_i128),(-129677601265258770463645833757247474261_i128)];
_2 = [(-9223372036854775808_isize),(-57_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = [3132730287803132364880451345525072278_i128,99726768576139694638937330233821962400_i128,59281737685028476538385149693940214803_i128,(-123588231548549482718760507343192109738_i128)];
RET = [2613568477619411056_i64,(-6938759154127919438_i64),(-6340252087641427757_i64),7782316368967652016_i64,3397354521313518239_i64];
_6.1.2 = 17474793127827496911_usize as u32;
_5 = [13071100105514945029166721857864481122_i128,4736235824331338995040170399418033449_i128,(-139270363320661686516912929806684514172_i128),(-125901404280771477861344402224041182146_i128)];
_6.1.3 = 168048687981454986588808202233824445397_u128 as f64;
_4 = [(-109_isize),120_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize];
_3.1 = !(-4425_i16);
_6.1.0 = 15394854003463312125_u64 * 14779896166456281995_u64;
_3 = (_4, (-26257_i16));
_6.1.3 = (-97657878408065617016536791671207422324_i128) as f64;
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
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
_9 = '\u{8bb0e}';
RET = [(-841784374422857417_i64),(-765968600431057456_i64),2489700662393144898_i64,(-787075937107698723_i64),2699346271362690150_i64];
_11 = (-138332923899261990075113031079442648868_i128) as u32;
_12 = (-141958643548972212163573462358374815307_i128) as isize;
_9 = '\u{8c58}';
_3.1 = _7;
_6.3.0 = _5;
_9 = '\u{398bd}';
_11 = _6.1.2;
_15 = core::ptr::addr_of!(_16);
RET = [(-4576036124096354397_i64),2260837005595807783_i64,1416270857108362117_i64,(-7379253551850643008_i64),(-292197831486925237_i64)];
(*_15) = 2090748879009178228_i64 as u128;
Goto(bb10)
}
bb10 = {
_7 = _3.1 | _3.1;
_6.1.0 = _8;
_6.2.0 = [2109914763_i32];
_13 = core::ptr::addr_of!(_6.2.0);
_15 = core::ptr::addr_of!(_16);
_3.1 = !_7;
_21 = core::ptr::addr_of!((*_13));
_6.3 = (_5,);
_6.2.0 = [(-1975179192_i32)];
(*_15) = 228384949463957360761962892917100127770_u128;
(*_21) = [699080903_i32];
(*_15) = 26889984984763263147092381932327587754_u128 * 37258377965968414136516132549683886013_u128;
_17 = -_6.1.3;
(*_15) = 139216763441900948320897975083987250297_u128;
match (*_15) {
0 => bb4,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
139216763441900948320897975083987250297 => bb17,
_ => bb16
}
}
bb11 = {
_3.0 = [(-108_isize),117_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_2 = [24_isize,(-9223372036854775808_isize),(-84_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _3.1;
_3.1 = -_7;
_6.1.0 = !5474731873947680229_u64;
_7 = _3.1 ^ _3.1;
_6.1.0 = 1323140530_i32 as u64;
RET = [1506300774341790399_i64,4556899841193058599_i64,641419207734751607_i64,(-6644945702159823276_i64),4543879464843995002_i64];
_4 = [17_isize,(-126_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.3 = (_5,);
_8 = _6.1.0 - _6.1.0;
_4 = _2;
_3.1 = _7 & _7;
_7 = -_3.1;
_5 = [(-12540078705404169360072842660405890424_i128),(-119383927714661093440848854647268366530_i128),120452299236736261724008047234644440503_i128,(-81328974893137680954645356939786499483_i128)];
_5 = _6.3.0;
_2 = [13_isize,9223372036854775807_isize,107_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6.1.0 = _8 >> _7;
_6.1.2 = 3647683887_u32 << _7;
RET = [(-3976392534724999691_i64),(-1643427683507678360_i64),366387540994846596_i64,9157897386443386596_i64,(-3521725103662379602_i64)];
_8 = !_6.1.0;
_6.1.1 = _5;
_6.1.2 = !309634445_u32;
Goto(bb3)
}
bb12 = {
_3 = (_4, (-16790_i16));
_4 = _2;
_3.1 = -11059_i16;
RET = [2965068753362750458_i64,(-3155572401715224387_i64),7557607435599170238_i64,(-8360290660200839627_i64),(-806056740919717353_i64)];
_3.1 = (-7659_i16);
_6.1.2 = 3464427693_u32 | 1399941875_u32;
_6.2.0 = [(-779289330_i32)];
_3 = (_2, (-16405_i16));
_6.1.1 = [106398692901069182119146854657488552550_i128,(-90586485097819667085074621190736582563_i128),(-94377734314251307065957468959308897913_i128),(-77788300772841217362821464129024748626_i128)];
_2 = [(-76_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-58_isize),(-9223372036854775808_isize)];
RET = [7520537422730576621_i64,(-8373440891379545481_i64),7646245466930408696_i64,(-4628476822334782908_i64),(-8461425554804363470_i64)];
_6.1.1 = _5;
_6.3.0 = [142745606449714672647352447588101252017_i128,167769499904695368646987439614220890052_i128,11025030483366958500997321801964187841_i128,(-165473439746812956628704670296679707775_i128)];
_5 = [134216922249280506575203091788397326774_i128,41882735564554989299867114288628402753_i128,(-107993622822704109259382935440101645278_i128),(-129677601265258770463645833757247474261_i128)];
_2 = [(-9223372036854775808_isize),(-57_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = [3132730287803132364880451345525072278_i128,99726768576139694638937330233821962400_i128,59281737685028476538385149693940214803_i128,(-123588231548549482718760507343192109738_i128)];
RET = [2613568477619411056_i64,(-6938759154127919438_i64),(-6340252087641427757_i64),7782316368967652016_i64,3397354521313518239_i64];
_6.1.2 = 17474793127827496911_usize as u32;
_5 = [13071100105514945029166721857864481122_i128,4736235824331338995040170399418033449_i128,(-139270363320661686516912929806684514172_i128),(-125901404280771477861344402224041182146_i128)];
_6.1.3 = 168048687981454986588808202233824445397_u128 as f64;
_4 = [(-109_isize),120_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize];
_3.1 = !(-4425_i16);
_6.1.0 = 15394854003463312125_u64 * 14779896166456281995_u64;
_3 = (_4, (-26257_i16));
_6.1.3 = (-97657878408065617016536791671207422324_i128) as f64;
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = (_4, (-16790_i16));
_4 = _2;
_3.1 = -11059_i16;
RET = [2965068753362750458_i64,(-3155572401715224387_i64),7557607435599170238_i64,(-8360290660200839627_i64),(-806056740919717353_i64)];
_3.1 = (-7659_i16);
_6.1.2 = 3464427693_u32 | 1399941875_u32;
_6.2.0 = [(-779289330_i32)];
_3 = (_2, (-16405_i16));
_6.1.1 = [106398692901069182119146854657488552550_i128,(-90586485097819667085074621190736582563_i128),(-94377734314251307065957468959308897913_i128),(-77788300772841217362821464129024748626_i128)];
_2 = [(-76_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-58_isize),(-9223372036854775808_isize)];
RET = [7520537422730576621_i64,(-8373440891379545481_i64),7646245466930408696_i64,(-4628476822334782908_i64),(-8461425554804363470_i64)];
_6.1.1 = _5;
_6.3.0 = [142745606449714672647352447588101252017_i128,167769499904695368646987439614220890052_i128,11025030483366958500997321801964187841_i128,(-165473439746812956628704670296679707775_i128)];
_5 = [134216922249280506575203091788397326774_i128,41882735564554989299867114288628402753_i128,(-107993622822704109259382935440101645278_i128),(-129677601265258770463645833757247474261_i128)];
_2 = [(-9223372036854775808_isize),(-57_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = [3132730287803132364880451345525072278_i128,99726768576139694638937330233821962400_i128,59281737685028476538385149693940214803_i128,(-123588231548549482718760507343192109738_i128)];
RET = [2613568477619411056_i64,(-6938759154127919438_i64),(-6340252087641427757_i64),7782316368967652016_i64,3397354521313518239_i64];
_6.1.2 = 17474793127827496911_usize as u32;
_5 = [13071100105514945029166721857864481122_i128,4736235824331338995040170399418033449_i128,(-139270363320661686516912929806684514172_i128),(-125901404280771477861344402224041182146_i128)];
_6.1.3 = 168048687981454986588808202233824445397_u128 as f64;
_4 = [(-109_isize),120_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize];
_3.1 = !(-4425_i16);
_6.1.0 = 15394854003463312125_u64 * 14779896166456281995_u64;
_3 = (_4, (-26257_i16));
_6.1.3 = (-97657878408065617016536791671207422324_i128) as f64;
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
Goto(bb2)
}
bb16 = {
_3.0 = [(-108_isize),117_isize,9223372036854775807_isize,(-115_isize),(-9223372036854775808_isize)];
_2 = [24_isize,(-9223372036854775808_isize),(-84_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _3.1;
_3.1 = -_7;
_6.1.0 = !5474731873947680229_u64;
_7 = _3.1 ^ _3.1;
_6.1.0 = 1323140530_i32 as u64;
RET = [1506300774341790399_i64,4556899841193058599_i64,641419207734751607_i64,(-6644945702159823276_i64),4543879464843995002_i64];
_4 = [17_isize,(-126_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.3 = (_5,);
_8 = _6.1.0 - _6.1.0;
_4 = _2;
_3.1 = _7 & _7;
_7 = -_3.1;
_5 = [(-12540078705404169360072842660405890424_i128),(-119383927714661093440848854647268366530_i128),120452299236736261724008047234644440503_i128,(-81328974893137680954645356939786499483_i128)];
_5 = _6.3.0;
_2 = [13_isize,9223372036854775807_isize,107_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6.1.0 = _8 >> _7;
_6.1.2 = 3647683887_u32 << _7;
RET = [(-3976392534724999691_i64),(-1643427683507678360_i64),366387540994846596_i64,9157897386443386596_i64,(-3521725103662379602_i64)];
_8 = !_6.1.0;
_6.1.1 = _5;
_6.1.2 = !309634445_u32;
Goto(bb3)
}
bb17 = {
_6.1 = (_8, _5, _11, _17);
_9 = '\u{e029}';
_12 = (-46_isize);
_8 = !_6.1.0;
(*_15) = !94638309377441592089376847105195823199_u128;
_4 = [_12,_12,_12,_12,_12];
_1 = &_19;
_5 = [102292889370642911431845885030843057888_i128,23697929905801167222138709734072960602_i128,75024007331500189094144271434620142378_i128,135063273383423667608868431038255770669_i128];
_11 = !_6.1.2;
(*_21) = [2004906826_i32];
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(15_usize, 4_usize, Move(_4), 3_usize, Move(_3), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(15_usize, 7_usize, Move(_7), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: ([isize; 5], i16),mut _2: [isize; 5],mut _3: [isize; 5],mut _4: [isize; 5],mut _5: ([isize; 5], i16),mut _6: ([isize; 5], i16),mut _7: [isize; 5],mut _8: [isize; 5],mut _9: ([isize; 5], i16),mut _10: ([isize; 5], i16),mut _11: [isize; 5]) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _12: i8;
let _13: *const usize;
let _14: &'static [usize; 1];
let _15: char;
let _16: [isize; 5];
let _17: [i8; 2];
let _18: bool;
let _19: u16;
let _20: Adt80;
let _21: &'static f64;
let _22: i16;
let _23: ([i16; 4], f32, char);
let _24: isize;
let _25: bool;
let _26: f64;
let _27: [usize; 1];
let _28: char;
let _29: &'static i64;
let _30: isize;
let _31: f64;
let _32: isize;
let _33: u32;
let _34: [u16; 8];
let _35: (i64, *const u128, [u32; 5]);
let _36: isize;
let _37: *const u32;
let _38: isize;
let _39: (*mut ([i32; 1],),);
let _40: [isize; 6];
let _41: [i16; 4];
let _42: (i64, *const u128, [u32; 5]);
let _43: &'static u128;
let _44: ();
let _45: ();
{
_6.1 = -_5.1;
_8 = _7;
_11 = [9223372036854775807_isize,(-9223372036854775808_isize),23_isize,(-9223372036854775808_isize),32_isize];
RET = [11_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = _10;
_9.0 = _2;
_1 = (_9.0, _9.1);
_9.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-20_isize),(-9223372036854775808_isize)];
_5.1 = 3825999916_u32 as i16;
_6 = _1;
_1.1 = _9.1;
_5.1 = !_6.1;
Call(_4 = fn17(_2, _8, _8, _2, _6.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [(-34_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-120_isize),9223372036854775807_isize];
_1.0 = [(-9223372036854775808_isize),(-17_isize),89_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = [(-5_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_9.1 = _6.1 | _1.1;
_7 = [(-9223372036854775808_isize),9223372036854775807_isize,(-87_isize),(-9223372036854775808_isize),56_isize];
_15 = '\u{6ed6b}';
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [16_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-98_isize)];
_1.0 = _5.0;
_10 = _9;
Call(_9.1 = core::intrinsics::transmute(_6.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.0 = [25_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-118_isize)];
_4 = _5.0;
_6.0 = _7;
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_16 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),103_isize];
_10.1 = _15 as i16;
_10.0 = _7;
_1 = (_2, _5.1);
_1 = _9;
_9.1 = _1.1;
Goto(bb3)
}
bb3 = {
_7 = [(-96_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-52_isize),(-120_isize)];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,0_isize];
_15 = '\u{b3a2d}';
_5 = (_9.0, _6.1);
_5 = (_10.0, _6.1);
_1 = _10;
_5.0 = _11;
_17 = [(-91_i8),114_i8];
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,55_isize,(-9223372036854775808_isize),(-68_isize)];
_1 = (_8, _9.1);
_16 = [9223372036854775807_isize,9223372036854775807_isize,(-12_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_12 = !21_i8;
_5.0 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb4)
}
bb4 = {
_6 = (_2, _5.1);
_19 = !60016_u16;
_18 = _9.1 == _6.1;
_5 = _1;
_1.1 = _9.1 | _10.1;
_23.0 = [_1.1,_6.1,_9.1,_1.1];
_9 = (_8, _1.1);
_5 = (_2, _10.1);
_10.1 = 63571333076726562492384329127830717768_i128 as i16;
_23.2 = _15;
_10.1 = _1.1 >> _19;
_10 = (_5.0, _9.1);
_22 = _1.1 | _9.1;
Goto(bb5)
}
bb5 = {
_5 = (_6.0, _10.1);
_15 = _23.2;
_5.1 = (-9223372036854775808_isize) as i16;
_10.0 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,64_isize];
_18 = !false;
_6 = _10;
_27 = [3246384020470524961_usize];
_21 = &_26;
_10.1 = 7639553545700569888_u64 as i16;
_19 = (-1128812060_i32) as u16;
_21 = &_26;
_17 = [_12,_12];
_6 = (_16, _9.1);
_1.1 = _19 as i16;
_21 = &_26;
_16 = _5.0;
_2 = RET;
_27 = [17301172085672528844_usize];
_7 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = _9.0;
_26 = 5522121106901653618_i64 as f64;
_9 = (_11, _22);
Goto(bb6)
}
bb6 = {
_17 = [_12,_12];
Call(_11 = fn18(), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9.1 = (-4_isize) as i16;
_9.0 = [(-9223372036854775808_isize),(-109_isize),89_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = _18 as i16;
_3 = [36_isize,9223372036854775807_isize,(-9223372036854775808_isize),13_isize,(-9223372036854775808_isize)];
_15 = _23.2;
_24 = _15 as isize;
_25 = !_18;
_21 = &_26;
_30 = -_24;
_18 = _22 > _22;
_28 = _15;
_33 = !3173994608_u32;
_9.1 = (*_21) as i16;
_4 = [_24,_24,_24,_24,_24];
_26 = 12785239510295908712_u64 as f64;
_23.2 = _15;
Goto(bb8)
}
bb8 = {
_1.0 = _3;
_23.1 = 172_u8 as f32;
_31 = _26;
_33 = 3512428119_u32 ^ 1766413411_u32;
_33 = 1284762883_u32;
RET = [_24,_30,_24,_24,_24];
_26 = -_31;
_5 = _1;
_5.1 = !_22;
_1.0 = _11;
_2 = [_24,_24,_30,_30,_30];
_21 = &_26;
_6.1 = _24 as i16;
match _33 {
0 => bb3,
1 => bb9,
2 => bb10,
1284762883 => bb12,
_ => bb11
}
}
bb9 = {
_9.1 = (-4_isize) as i16;
_9.0 = [(-9223372036854775808_isize),(-109_isize),89_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = _18 as i16;
_3 = [36_isize,9223372036854775807_isize,(-9223372036854775808_isize),13_isize,(-9223372036854775808_isize)];
_15 = _23.2;
_24 = _15 as isize;
_25 = !_18;
_21 = &_26;
_30 = -_24;
_18 = _22 > _22;
_28 = _15;
_33 = !3173994608_u32;
_9.1 = (*_21) as i16;
_4 = [_24,_24,_24,_24,_24];
_26 = 12785239510295908712_u64 as f64;
_23.2 = _15;
Goto(bb8)
}
bb10 = {
_8 = [(-34_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-120_isize),9223372036854775807_isize];
_1.0 = [(-9223372036854775808_isize),(-17_isize),89_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = [(-5_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_9.1 = _6.1 | _1.1;
_7 = [(-9223372036854775808_isize),9223372036854775807_isize,(-87_isize),(-9223372036854775808_isize),56_isize];
_15 = '\u{6ed6b}';
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [16_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-98_isize)];
_1.0 = _5.0;
_10 = _9;
Call(_9.1 = core::intrinsics::transmute(_6.1), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_7 = [(-96_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-52_isize),(-120_isize)];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,0_isize];
_15 = '\u{b3a2d}';
_5 = (_9.0, _6.1);
_5 = (_10.0, _6.1);
_1 = _10;
_5.0 = _11;
_17 = [(-91_i8),114_i8];
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,55_isize,(-9223372036854775808_isize),(-68_isize)];
_1 = (_8, _9.1);
_16 = [9223372036854775807_isize,9223372036854775807_isize,(-12_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_12 = !21_i8;
_5.0 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb4)
}
bb12 = {
_10.0 = _5.0;
_1 = _5;
_19 = !28913_u16;
_14 = &_27;
_1 = (_16, _22);
_23.1 = 59_u8 as f32;
_7 = _11;
_26 = -_31;
_10.0 = _1.0;
_36 = _5.1 as isize;
_26 = _31 + _31;
_6.0 = [_24,_24,_30,_36,_36];
_10.0 = [_36,_36,_36,_36,_36];
_35.0 = 2803021015981191213_i64 >> _5.1;
_23.2 = _15;
_30 = _36 & _36;
_11 = [_30,_30,_30,_30,_30];
_2 = [_30,_30,_30,_30,_30];
Goto(bb13)
}
bb13 = {
_2 = [_30,_36,_30,_30,_24];
_10 = (_2, _6.1);
_25 = _18 == _18;
_4 = _5.0;
_37 = core::ptr::addr_of!(_33);
_6 = _1;
_11 = [_30,_30,_36,_30,_36];
_5.0 = _8;
_11 = _3;
_12 = 74_i8 + 110_i8;
Goto(bb14)
}
bb14 = {
_10.0 = [_30,_36,_30,_30,_30];
_5.0 = [_30,_30,_30,_30,_30];
_8 = [_24,_30,_36,_36,_30];
_21 = &_26;
_28 = _23.2;
_9 = (_1.0, _22);
_34 = [_19,_19,_19,_19,_19,_19,_19,_19];
_23.2 = _15;
_40 = [_30,_30,_36,_36,_36,_30];
_25 = _18;
_31 = (*_37) as f64;
_23.1 = (*_37) as f32;
_36 = !_30;
_6 = (_16, _9.1);
_42.0 = _30 as i64;
_41 = [_1.1,_6.1,_6.1,_9.1];
_24 = !_36;
_1 = (_6.0, _9.1);
_10 = _1;
_2 = [_36,_24,_24,_36,_30];
_10.0 = [_24,_36,_36,_24,_36];
_18 = _26 >= _26;
_23.1 = 44_u8 as f32;
_12 = -8_i8;
_17 = [_12,_12];
_32 = _24;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(16_usize, 22_usize, Move(_22), 19_usize, Move(_19), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(16_usize, 4_usize, Move(_4), 34_usize, Move(_34), 33_usize, Move(_33), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(16_usize, 1_usize, Move(_1), 27_usize, Move(_27), 24_usize, Move(_24), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(16_usize, 32_usize, Move(_32), 17_usize, Move(_17), 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [isize; 5],mut _2: [isize; 5],mut _3: [isize; 5],mut _4: [isize; 5],mut _5: i16) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _6: [u8; 6];
let _7: *const u128;
let _8: isize;
let _9: &'static u128;
let _10: char;
let _11: bool;
let _12: isize;
let _13: f32;
let _14: f32;
let _15: [u8; 3];
let _16: [u128; 8];
let _17: ([i32; 1],);
let _18: [i16; 3];
let _19: char;
let _20: u16;
let _21: Adt43;
let _22: &'static isize;
let _23: i128;
let _24: Adt64;
let _25: isize;
let _26: Adt21;
let _27: isize;
let _28: i16;
let _29: f64;
let _30: u32;
let _31: f64;
let _32: u16;
let _33: [u16; 8];
let _34: &'static i64;
let _35: ();
let _36: ();
{
RET = _2;
RET = [(-9223372036854775808_isize),(-103_isize),30_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = 13430_i16;
RET = [9223372036854775807_isize,92_isize,17_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_6 = [150_u8,10_u8,190_u8,217_u8,124_u8,16_u8];
_4 = RET;
_6 = [147_u8,208_u8,63_u8,187_u8,134_u8,81_u8];
RET = [(-13_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
13430 => bb7,
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
_4 = _1;
_3 = RET;
_8 = 9223372036854775807_isize & (-36_isize);
_2 = [_8,_8,_8,_8,_8];
RET = [_8,_8,_8,_8,_8];
_2 = RET;
_8 = 115_isize * 9223372036854775807_isize;
RET = [_8,_8,_8,_8,_8];
_11 = true;
_5 = 15766839751199622976_usize as i16;
_4 = [_8,_8,_8,_8,_8];
_1 = [_8,_8,_8,_8,_8];
_10 = '\u{a79ae}';
_4 = _2;
_2 = [_8,_8,_8,_8,_8];
_1 = [_8,_8,_8,_8,_8];
Goto(bb8)
}
bb8 = {
RET = _4;
_6 = [157_u8,42_u8,159_u8,30_u8,92_u8,103_u8];
_12 = _8 >> _5;
_10 = '\u{3f7e5}';
_4 = [_12,_12,_8,_8,_8];
_11 = _12 == _8;
_3 = RET;
_15 = [11_u8,14_u8,212_u8];
_11 = true ^ true;
RET = [_12,_12,_12,_12,_8];
_1 = RET;
_16 = [117807262771850257090014285386763362887_u128,136605774986179090378463741467955422424_u128,292267792322870331864979667241002747757_u128,256822549040707419607740955677184091516_u128,275132515861536678489180197337101447886_u128,189959756520598117823964593355079334753_u128,290451370775188173775787463816641308749_u128,323999736811458897089298699341504683147_u128];
_6 = [234_u8,16_u8,204_u8,111_u8,117_u8,27_u8];
_4 = [_12,_8,_12,_8,_12];
_6 = [139_u8,110_u8,194_u8,255_u8,197_u8,104_u8];
_14 = 4_usize as f32;
_3 = [_12,_12,_12,_12,_12];
_19 = _10;
_10 = _19;
_10 = _19;
_4 = [_12,_12,_12,_12,_8];
Goto(bb9)
}
bb9 = {
_6 = [213_u8,23_u8,153_u8,255_u8,97_u8,241_u8];
_8 = _12 | _12;
RET = _2;
_11 = false & false;
_17.0 = [2004573181_i32];
RET = [_12,_8,_12,_8,_8];
_16 = [238907334163327173949523981990195420520_u128,25877514126177427212754362088707644816_u128,291875372594927210769681851149816652236_u128,82617115939592551979602969852553141464_u128,248920580951354162960897585128661559216_u128,37995468210109116989700125003914861192_u128,36038183384745361704021551576065828100_u128,126807791671767795229078731065678312411_u128];
_17.0 = [(-2105404796_i32)];
_14 = 32858482285008063944144503249666173790_i128 as f32;
Goto(bb10)
}
bb10 = {
_12 = _8 & _8;
_15 = [0_u8,195_u8,201_u8];
_8 = (-1507399288_i32) as isize;
_16 = [78644424829284964072772617983844890198_u128,128627647113801569898507275843841413909_u128,137540643584017916121050332080437605344_u128,227835583663195656628930904804346170571_u128,210985894427965330862114244377691475823_u128,182533256061294301450075060722143842526_u128,325595515329956204614343969556334421482_u128,169498623727548458262166032349393280958_u128];
_4 = [_8,_12,_12,_12,_12];
_22 = &_8;
_18 = [_5,_5,_5];
_5 = (-13604_i16) - 943_i16;
_13 = 189276263476359421736784838563372334965_u128 as f32;
_20 = 20510_u16;
_19 = _10;
_14 = 4_usize as f32;
_4 = [_12,_12,_12,(*_22),_12];
_18 = [_5,_5,_5];
_12 = (*_22) ^ (*_22);
_22 = &(*_22);
_18 = [_5,_5,_5];
Goto(bb11)
}
bb11 = {
_17.0 = [29797677_i32];
_3 = [_12,_12,(*_22),_12,_12];
_12 = 6151610484616689569_u64 as isize;
_25 = -_8;
_29 = 5_usize as f64;
RET = [_8,_8,(*_22),_25,_8];
_18 = [_5,_5,_5];
_14 = _13;
_3 = [_12,_25,_25,_8,_8];
_10 = _19;
_22 = &(*_22);
Goto(bb12)
}
bb12 = {
_17.0 = [(-303162374_i32)];
_27 = _25 << _12;
_28 = _5 << _8;
_27 = _12 ^ (*_22);
_29 = 3133280180_u32 as f64;
RET = _2;
_25 = _27 << (*_22);
_3 = [_12,_25,_12,_25,_25];
_16 = [50277388400161661009091125991127025325_u128,46682091070407524416843416656938577803_u128,309316890592499455505858372556278240200_u128,70110140728141241556736509896650861372_u128,35891713963146892116543049826821509890_u128,236540431481400231039247816998901684260_u128,26208550261709301957608055609714649194_u128,75842987362454862360450836049253420627_u128];
RET = [_25,_27,_25,(*_22),_25];
_12 = -_27;
_23 = -100882655112709461732379081061924299398_i128;
_19 = _10;
_30 = (-2_i8) as u32;
Call(_27 = core::intrinsics::bswap(_8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26 = Adt21::Variant2 { fld0: 310281866950397220188055568926536638660_u128,fld1: _29,fld2: _25 };
_8 = 779699260_i32 as isize;
_1 = [_12,_25,_12,_25,_25];
_23 = _11 as i128;
_8 = !_12;
_1 = [_25,_8,_27,_8,_8];
_6 = [188_u8,199_u8,166_u8,182_u8,25_u8,103_u8];
_7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_26, 2), 0)));
RET = [Field::<isize>(Variant(_26, 2), 2),_8,_25,_12,Field::<isize>(Variant(_26, 2), 2)];
_31 = _29 * Field::<f64>(Variant(_26, 2), 1);
_22 = &_27;
place!(Field::<f64>(Variant(_26, 2), 1)) = _29 - _29;
_13 = -_14;
(*_7) = 8656232240880224098574756883695719797_u128;
_33 = [_20,_20,_20,_20,_20,_20,_20,_20];
Goto(bb14)
}
bb14 = {
_12 = _27 ^ _27;
_22 = &_12;
_2 = [_12,_12,_12,_8,_25];
_32 = 12901937145520917543_u64 as u16;
_22 = &(*_22);
_25 = 11940210383087282765_u64 as isize;
_22 = &(*_22);
(*_7) = 233999982648187787754958202869255749133_u128;
place!(Field::<f64>(Variant(_26, 2), 1)) = 69_i8 as f64;
_2 = [_12,(*_22),_12,_12,_8];
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(17_usize, 12_usize, Move(_12), 23_usize, Move(_23), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(17_usize, 16_usize, Move(_16), 15_usize, Move(_15), 25_usize, Move(_25), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(17_usize, 3_usize, Move(_3), 28_usize, Move(_28), 18_usize, Move(_18), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _1: &'static &'static isize;
let _2: &'static char;
let _3: [u8; 6];
let _4: ([i16; 4], f32, char);
let _5: isize;
let _6: usize;
let _7: isize;
let _8: bool;
let _9: isize;
let _10: f64;
let _11: i128;
let _12: bool;
let _13: *mut &'static [usize; 1];
let _14: f32;
let _15: isize;
let _16: [u128; 8];
let _17: f64;
let _18: (i8,);
let _19: u16;
let _20: ();
let _21: ();
{
RET = [99_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-32_isize),104_isize];
RET = [(-30_isize),9223372036854775807_isize,(-9223372036854775808_isize),26_isize,(-9223372036854775808_isize)];
RET = [(-9223372036854775808_isize),72_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-109_isize)];
RET = [(-9223372036854775808_isize),22_isize,(-9223372036854775808_isize),71_isize,40_isize];
Goto(bb1)
}
bb1 = {
RET = [(-52_isize),9223372036854775807_isize,9223372036854775807_isize,(-101_isize),47_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
RET = [(-124_isize),(-9223372036854775808_isize),(-57_isize),11_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),34_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),21_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-57_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET = [9223372036854775807_isize,(-36_isize),(-9223372036854775808_isize),(-58_isize),(-44_isize)];
RET = [56_isize,(-77_isize),9223372036854775807_isize,36_isize,12_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-88_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,(-122_isize),61_isize,9223372036854775807_isize,6_isize];
_4.0 = [18673_i16,17566_i16,(-17279_i16),1507_i16];
_4.1 = 7792554477248497958_i64 as f32;
Goto(bb2)
}
bb2 = {
_4.0 = [1084_i16,6854_i16,(-32044_i16),(-5819_i16)];
_4.2 = '\u{d56ac}';
_2 = &_4.2;
_5 = -9223372036854775807_isize;
_6 = 17495976027139356885_usize;
_5 = !(-9223372036854775808_isize);
_3 = [1_u8,122_u8,226_u8,148_u8,172_u8,23_u8];
_3 = [29_u8,216_u8,254_u8,25_u8,93_u8,176_u8];
_3 = [228_u8,154_u8,0_u8,248_u8,196_u8,117_u8];
_7 = !_5;
_4.2 = '\u{47fe8}';
_7 = _5;
_4.1 = 63977_u16 as f32;
RET = [_5,_7,_7,_5,_7];
_8 = !true;
_4.2 = '\u{7a17f}';
_2 = &_4.2;
_5 = -_7;
_7 = _8 as isize;
_6 = 16503540131649008984_usize;
_3 = [156_u8,104_u8,189_u8,250_u8,2_u8,194_u8];
RET = [_7,_5,_7,_5,_7];
_9 = -_5;
_2 = &_4.2;
_2 = &(*_2);
_3 = [195_u8,158_u8,195_u8,117_u8,225_u8,32_u8];
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_4.2 = '\u{37f26}';
_2 = &_4.2;
_8 = true;
_10 = _5 as f64;
_4.2 = '\u{aefe3}';
_4.1 = _10 as f32;
_2 = &_4.2;
_10 = 86_i8 as f64;
_10 = _7 as f64;
_6 = 1_usize;
_11 = 16696255691878099308_u64 as i128;
RET[_6] = -_5;
match _3[_6] {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
158 => bb10,
_ => bb9
}
}
bb5 = {
Goto(bb4)
}
bb6 = {
_4.0 = [1084_i16,6854_i16,(-32044_i16),(-5819_i16)];
_4.2 = '\u{d56ac}';
_2 = &_4.2;
_5 = -9223372036854775807_isize;
_6 = 17495976027139356885_usize;
_5 = !(-9223372036854775808_isize);
_3 = [1_u8,122_u8,226_u8,148_u8,172_u8,23_u8];
_3 = [29_u8,216_u8,254_u8,25_u8,93_u8,176_u8];
_3 = [228_u8,154_u8,0_u8,248_u8,196_u8,117_u8];
_7 = !_5;
_4.2 = '\u{47fe8}';
_7 = _5;
_4.1 = 63977_u16 as f32;
RET = [_5,_7,_7,_5,_7];
_8 = !true;
_4.2 = '\u{7a17f}';
_2 = &_4.2;
_5 = -_7;
_7 = _8 as isize;
_6 = 16503540131649008984_usize;
_3 = [156_u8,104_u8,189_u8,250_u8,2_u8,194_u8];
RET = [_7,_5,_7,_5,_7];
_9 = -_5;
_2 = &_4.2;
_2 = &(*_2);
_3 = [195_u8,158_u8,195_u8,117_u8,225_u8,32_u8];
Goto(bb3)
}
bb7 = {
RET = [(-52_isize),9223372036854775807_isize,9223372036854775807_isize,(-101_isize),47_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
RET = [(-124_isize),(-9223372036854775808_isize),(-57_isize),11_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),34_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),21_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-57_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET = [9223372036854775807_isize,(-36_isize),(-9223372036854775808_isize),(-58_isize),(-44_isize)];
RET = [56_isize,(-77_isize),9223372036854775807_isize,36_isize,12_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-88_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,(-122_isize),61_isize,9223372036854775807_isize,6_isize];
_4.0 = [18673_i16,17566_i16,(-17279_i16),1507_i16];
_4.1 = 7792554477248497958_i64 as f32;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_4.0[_6] = -(-2197_i16);
_8 = _6 < _6;
_4.1 = _6 as f32;
RET = [_5,_7,_9,_7,_9];
RET = [_5,_5,_5,_7,_5];
RET = [_9,_9,_5,_5,_9];
_4.0[_6] = -27695_i16;
_8 = !false;
_9 = RET[_6] << _3[_6];
_11 = !127836857424427642381958917732940625148_i128;
RET[_6] = !_9;
match _6 {
0 => bb3,
1 => bb12,
_ => bb11
}
}
bb11 = {
Goto(bb4)
}
bb12 = {
_15 = _9 << _5;
_10 = (-31_i8) as f64;
_4.0[_6] = -25649_i16;
_8 = !true;
RET[_6] = !_9;
_4.2 = '\u{c2cbd}';
RET[_6] = _8 as isize;
_10 = 136241820991620387107818796298077857388_u128 as f64;
Goto(bb13)
}
bb13 = {
_3 = [26_u8,238_u8,170_u8,141_u8,60_u8,74_u8];
_14 = -_4.1;
_11 = 151385454526004608055739285308625504377_i128 | 29733956938008520659531437563906265171_i128;
RET = [_9,_9,_15,_9,_15];
_11 = _10 as i128;
RET = [_9,_9,_15,_15,_15];
_7 = (-2716081023402621554_i64) as isize;
_12 = _10 > _10;
_7 = -RET[_6];
_16[_6] = !149679831499693813225096264974554580086_u128;
_5 = RET[_6] & _9;
_12 = _8;
_2 = &_4.2;
_3 = [243_u8,242_u8,181_u8,6_u8,97_u8,42_u8];
_4.2 = '\u{c0882}';
_9 = 5147948169396734675_u64 as isize;
_7 = -_15;
_4.1 = _14 * _14;
RET[_6] = _15;
_4.1 = -_14;
Call(_5 = fn19(_3, _3[_6], _15, _7, _8, _3, _3[_6]), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18.0 = !(-126_i8);
_5 = _7;
_17 = _10;
_11 = 103725961139736032113351419939174863275_i128;
_4.1 = 28736_i16 as f32;
RET = [_5,_15,_7,_5,_7];
_4.1 = -_14;
_18.0 = 37_i8 * (-59_i8);
_18 = ((-103_i8),);
_18 = ((-107_i8),);
_3 = [223_u8,126_u8,140_u8,108_u8,1_u8,67_u8];
_16 = [234499682233742654443490494562081433120_u128,27031172294633269961362809360281688821_u128,113228775269541643300956301185717254583_u128,335478003353271847646930258254613256653_u128,29321802151878575900569468366298082534_u128,104577920524124778654294686793159569901_u128,314407616852117013472394853397654752327_u128,146083506391858800236708099835590058695_u128];
_3 = [199_u8,13_u8,180_u8,211_u8,32_u8,228_u8];
_18.0 = (-59_i8);
_6 = _14 as usize;
_18 = (49_i8,);
_4.2 = '\u{71486}';
_18 = (19_i8,);
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(18_usize, 12_usize, Move(_12), 16_usize, Move(_16), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(18_usize, 18_usize, Move(_18), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [u8; 6],mut _2: u8,mut _3: isize,mut _4: isize,mut _5: bool,mut _6: [u8; 6],mut _7: u8) -> isize {
mir! {
type RET = isize;
let _8: Adt40;
let _9: ();
let _10: ();
{
_5 = !false;
RET = 67_i8 as isize;
RET = !_4;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
242 => bb6,
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
_7 = _2 * _2;
_3 = 312505750867089066527057272875184888210_u128 as isize;
_3 = _4;
_3 = 25643_u16 as isize;
RET = !_4;
_7 = _2 & _2;
RET = !_4;
_5 = false;
_3 = RET << _7;
_5 = !false;
_4 = !_3;
_6 = [_7,_7,_7,_2,_2,_2];
_2 = _7 * _7;
RET = -_4;
_5 = !true;
_1 = [_2,_7,_2,_2,_2,_7];
RET = !_3;
RET = !_4;
_1 = [_7,_7,_7,_7,_2,_7];
RET = -_4;
RET = _4;
RET = _4;
_7 = !_2;
_4 = _5 as isize;
_4 = _3;
_5 = _4 >= _4;
_6 = _1;
RET = _4;
Goto(bb7)
}
bb7 = {
Call(_9 = dump_var(19_usize, 1_usize, Move(_1), 7_usize, Move(_7), 5_usize, Move(_5), 10_usize, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: u64,
fld1: isize,

},
Variant1{
fld0: u16,

},
Variant2{
fld0: u128,
fld1: f64,
fld2: isize,

},
Variant3{
fld0: u16,
fld1: char,
fld2: f32,
fld3: u32,
fld4: [i16; 4],

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: [i16; 4],

},
Variant1{
fld0: u16,
fld1: char,
fld2: i32,

},
Variant2{
fld0: bool,
fld1: u32,

},
Variant3{
fld0: i32,
fld1: char,
fld2: [i32; 1],
fld3: [i128; 4],

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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: *const u128,
fld1: char,
fld2: isize,
fld3: i8,
fld4: ([i16; 4], f32, char),
fld5: [u8; 6],
fld6: Adt32,

},
Variant1{
fld0: usize,
fld1: [i128; 4],
fld2: u64,
fld3: i8,
fld4: [isize; 6],
fld5: Adt32,
fld6: ([i16; 4], f32, char),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [i64; 3],
fld1: ([i128; 4],),

},
Variant1{
fld0: [u8; 6],
fld1: char,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: u64,
fld1: ([i16; 4], f32, char),
fld2: usize,
fld3: i8,

},
Variant1{
fld0: Adt21,

}}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){unsafe{printf("Adt64::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: Adt32,
fld1: [usize; 2],
fld2: (u64, [i128; 4], u32, f64),

},
Variant1{
fld0: [i64; 7],
fld1: [i8; 2],
fld2: [u16; 1],
fld3: [u8; 6],
fld4: Adt42,

},
Variant2{
fld0: Adt43,
fld1: ([i16; 4], f32, char),
fld2: *mut usize,
fld3: i8,
fld4: [i64; 5],

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i16; 4],
fld1: [u32; 5],
fld2: i128,
fld3: *const [i32; 1],
fld4: [u8; 6],

},
Variant1{
fld0: (*mut *const usize, (u64, [i128; 4], u32, f64), ([i32; 1],), ([i128; 4],)),
fld1: Adt42,

}}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){unsafe{printf("Adt80::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt80 {
Variant0{
fld0: Adt43,
fld1: [i32; 5],
fld2: isize,
fld3: i8,
fld4: u8,
fld5: u16,
fld6: i64,

},
Variant1{
fld0: [usize; 2],
fld1: [bool; 5],
fld2: [i16; 3],
fld3: Adt21,

}}

