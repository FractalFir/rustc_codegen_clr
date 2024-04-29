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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u32,mut _9: usize,mut _10: u8,mut _11: u16) -> bool {
mir! {
type RET = bool;
let _12: Adt47;
let _13: [i128; 8];
let _14: Adt41;
let _15: isize;
let _16: Adt38;
let _17: (i16, [i16; 3]);
let _18: isize;
let _19: char;
let _20: f32;
let _21: i64;
let _22: [u64; 8];
let _23: ();
let _24: ();
{
_5 = 7824_i16;
_8 = (-1107766806_i32) as u32;
_2 = 54956586082194255598960671144474472949_u128;
RET = !true;
_4 = 59_i8;
_1 = !RET;
_9 = 7_usize >> _8;
_7 = (-2204455794579482456_i64);
_11 = 63767006801581490101049731318500110475_i128 as u16;
_6 = 245_u8 as i32;
RET = !_1;
_1 = RET;
_2 = 44018053266611450573116616477411527305_u128 >> _9;
_8 = !147538353_u32;
_10 = (-160530132611478950756479108463551501640_i128) as u8;
_10 = 5956418692592081520_u64 as u8;
_4 = _5 as i8;
_2 = 246471009573481043598835759841329528462_u128;
_5 = _1 as i16;
_3 = 9223372036854775807_isize - (-9_isize);
_10 = !26_u8;
_1 = !RET;
_10 = !76_u8;
_13 = [140165833530802247835330206363910233992_i128,84567340927506661638755245808838476638_i128,(-166573993462509718085698822638465965776_i128),(-132538444744693657435508969066356594850_i128),22019030164956059560256664405350439913_i128,(-84708638591006174276834934172798269926_i128),(-65174855061200859617414686775315061298_i128),135096118280699819265389816758259562085_i128];
_9 = !17696341247652473653_usize;
Goto(bb1)
}
bb1 = {
RET = _1;
_7 = (-5121204196013149979_i64);
_6 = (-829631154_i32);
_10 = 243_u8;
_4 = (-50_i8);
_11 = RET as u16;
_1 = _8 < _8;
_2 = 168774882736487644676632331439249856654_u128 ^ 52854808541419346767590652364239991058_u128;
_9 = _11 as usize;
_15 = !_3;
RET = _1;
Call(_16.fld1.0 = fn1(_3, _2, _15, _13, _2, _10, _15, _10, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_1;
_3 = _15;
_3 = _15;
_2 = !232866994458789569725367233096654663990_u128;
_13 = [110273240015175592736778989275935476219_i128,(-30181594275186182534395462639504378729_i128),(-113328295385047624409262784843274341959_i128),(-105738467467233324836814624248543031510_i128),5554387103004097455873345886679751648_i128,1245479479545770896015629135087346568_i128,26600564637429018672102129814014263834_i128,(-138305365040370591265523255214581552625_i128)];
RET = !_1;
_8 = 238092604_u32 | 649778260_u32;
_3 = _5 as isize;
_9 = _5 as usize;
_1 = RET;
_13 = [160812431927035035510437646949346243635_i128,(-75612383528800296452150345406727216233_i128),(-60139717257472797653148169465076559095_i128),130342186916783144738124794967988128428_i128,68000020155249332575847000010767167013_i128,59923494696816007438849108960921963036_i128,(-79929953549425807939395216260463294888_i128),(-50759110356330716134754642219110160556_i128)];
_13 = [19293968253067584225508359462819460935_i128,155658626529300253765531979512197399303_i128,(-46380893783525152566807287489093431702_i128),160389190071306724131168022246634323948_i128,51467735757913884407862432805471197258_i128,(-130790504463485215626060956924449261764_i128),(-120151148306903358255668418991683022347_i128),59831375483228164819768098437198453969_i128];
_1 = RET >= RET;
_11 = 21154_u16 + 39495_u16;
_10 = 7_u8 ^ 169_u8;
_10 = _2 as u8;
_8 = _10 as u32;
_6 = !436788534_i32;
_17.1 = [_5,_5,_5];
_2 = _7 as u128;
_5 = 12676_i16 * 11389_i16;
_2 = !205713727045112920207001396883433893412_u128;
_17.1 = [_5,_5,_5];
_17.0 = _5;
_2 = !339031554933695153807402201586950675900_u128;
_18 = _15 - _16.fld1.0;
Call(_15 = core::intrinsics::bswap(_18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = '\u{6756f}';
_17.0 = !_5;
_9 = 5_usize * 6005066357913453693_usize;
_5 = !_17.0;
_4 = 127_i8;
_13 = [127850771699080389007177043639781660535_i128,128857343681760839084614471958535721901_i128,43169345836569279879831067034646342612_i128,63559603820931403793970978825659937194_i128,156058045784914276462142995789067408917_i128,11727700992983865702984611049092197929_i128,97463961233867373269229028299061483579_i128,(-120691547530861881413303656820813599624_i128)];
_6 = (-386408240_i32);
_17.0 = _5;
_9 = _11 as usize;
_20 = _8 as f32;
_17.0 = -_5;
Goto(bb4)
}
bb4 = {
_10 = _8 as u8;
_20 = _4 as f32;
_9 = 14846753649453892892_usize;
_17.0 = _5;
_16.fld1.0 = _18;
Goto(bb5)
}
bb5 = {
_1 = _11 != _11;
_8 = !3886356231_u32;
_1 = RET;
_18 = !_3;
_19 = '\u{1406a}';
_9 = 6_usize;
_16.fld1.1 = [16220105344587331001_u64,3577154569955536872_u64];
_17.0 = _16.fld1.0 as i16;
match _13[_9] {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
97463961233867373269229028299061483579 => bb14,
_ => bb13
}
}
bb6 = {
_10 = _8 as u8;
_20 = _4 as f32;
_9 = 14846753649453892892_usize;
_17.0 = _5;
_16.fld1.0 = _18;
Goto(bb5)
}
bb7 = {
_19 = '\u{6756f}';
_17.0 = !_5;
_9 = 5_usize * 6005066357913453693_usize;
_5 = !_17.0;
_4 = 127_i8;
_13 = [127850771699080389007177043639781660535_i128,128857343681760839084614471958535721901_i128,43169345836569279879831067034646342612_i128,63559603820931403793970978825659937194_i128,156058045784914276462142995789067408917_i128,11727700992983865702984611049092197929_i128,97463961233867373269229028299061483579_i128,(-120691547530861881413303656820813599624_i128)];
_6 = (-386408240_i32);
_17.0 = _5;
_9 = _11 as usize;
_20 = _8 as f32;
_17.0 = -_5;
Goto(bb4)
}
bb8 = {
RET = !_1;
_3 = _15;
_3 = _15;
_2 = !232866994458789569725367233096654663990_u128;
_13 = [110273240015175592736778989275935476219_i128,(-30181594275186182534395462639504378729_i128),(-113328295385047624409262784843274341959_i128),(-105738467467233324836814624248543031510_i128),5554387103004097455873345886679751648_i128,1245479479545770896015629135087346568_i128,26600564637429018672102129814014263834_i128,(-138305365040370591265523255214581552625_i128)];
RET = !_1;
_8 = 238092604_u32 | 649778260_u32;
_3 = _5 as isize;
_9 = _5 as usize;
_1 = RET;
_13 = [160812431927035035510437646949346243635_i128,(-75612383528800296452150345406727216233_i128),(-60139717257472797653148169465076559095_i128),130342186916783144738124794967988128428_i128,68000020155249332575847000010767167013_i128,59923494696816007438849108960921963036_i128,(-79929953549425807939395216260463294888_i128),(-50759110356330716134754642219110160556_i128)];
_13 = [19293968253067584225508359462819460935_i128,155658626529300253765531979512197399303_i128,(-46380893783525152566807287489093431702_i128),160389190071306724131168022246634323948_i128,51467735757913884407862432805471197258_i128,(-130790504463485215626060956924449261764_i128),(-120151148306903358255668418991683022347_i128),59831375483228164819768098437198453969_i128];
_1 = RET >= RET;
_11 = 21154_u16 + 39495_u16;
_10 = 7_u8 ^ 169_u8;
_10 = _2 as u8;
_8 = _10 as u32;
_6 = !436788534_i32;
_17.1 = [_5,_5,_5];
_2 = _7 as u128;
_5 = 12676_i16 * 11389_i16;
_2 = !205713727045112920207001396883433893412_u128;
_17.1 = [_5,_5,_5];
_17.0 = _5;
_2 = !339031554933695153807402201586950675900_u128;
_18 = _15 - _16.fld1.0;
Call(_15 = core::intrinsics::bswap(_18), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = _1;
_7 = (-5121204196013149979_i64);
_6 = (-829631154_i32);
_10 = 243_u8;
_4 = (-50_i8);
_11 = RET as u16;
_1 = _8 < _8;
_2 = 168774882736487644676632331439249856654_u128 ^ 52854808541419346767590652364239991058_u128;
_9 = _11 as usize;
_15 = !_3;
RET = _1;
Call(_16.fld1.0 = fn1(_3, _2, _15, _13, _2, _10, _15, _10, _15), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb14 = {
_17.0 = -_5;
RET = _16.fld1.0 <= _15;
_13 = [(-36764121130155572151392385970276756978_i128),(-148528834558135475923647858263366207576_i128),165075637815820910304830996202869578374_i128,(-36031988305163002585900701315130112876_i128),146566479097853495860846061907438307014_i128,150671868307291869464586641222548017871_i128,(-3902509061943005380468186991536214872_i128),106474868721378349271807317384233091966_i128];
_21 = _7;
_17.0 = RET as i16;
_3 = _16.fld1.0;
_21 = _7;
_2 = !327900817846122794865008696181300925179_u128;
_6 = 752526712_i32;
_20 = _13[_9] as f32;
_22 = [17441074766199210793_u64,13766901928654786663_u64,16510592263119112766_u64,305034767828445203_u64,15207586858377458927_u64,1571448510959494344_u64,2528896804637230082_u64,8695193506364365615_u64];
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(0_usize, 15_usize, Move(_15), 22_usize, Move(_22), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(0_usize, 7_usize, Move(_7), 21_usize, Move(_21), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(0_usize, 1_usize, Move(_1), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u128,mut _3: isize,mut _4: [i128; 8],mut _5: u128,mut _6: u8,mut _7: isize,mut _8: u8,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: f32;
let _11: u8;
let _12: Adt53;
let _13: bool;
let _14: [u64; 8];
let _15: u32;
let _16: u8;
let _17: Adt43;
let _18: Adt45;
let _19: u64;
let _20: Adt54;
let _21: (u32,);
let _22: isize;
let _23: (i16, [i16; 3]);
let _24: i128;
let _25: isize;
let _26: i128;
let _27: isize;
let _28: bool;
let _29: u32;
let _30: [u64; 8];
let _31: (u32,);
let _32: [i128; 8];
let _33: isize;
let _34: [i8; 7];
let _35: bool;
let _36: &'static u8;
let _37: i128;
let _38: (isize, [u64; 2]);
let _39: isize;
let _40: f32;
let _41: f32;
let _42: u8;
let _43: f64;
let _44: i32;
let _45: (i16, usize, [u64; 2], i16);
let _46: [u128; 6];
let _47: ([i8; 7], f32, [i128; 8]);
let _48: isize;
let _49: u64;
let _50: ();
let _51: ();
{
_5 = _2;
_4 = [(-107913976849304639374214109463692687921_i128),68823404809805781288681619889991177993_i128,87382527267766486312191346141139694206_i128,(-41835507952066406945560018134291909285_i128),(-22929453070252477437825301278501502985_i128),163134416398492248065072476295249926165_i128,111185105016208417261837651719379908376_i128,19516111513684849311187741989811901094_i128];
_11 = (-18_i8) as u8;
RET = !_1;
_6 = _8 / _8;
_10 = (-6824330819993987189_i64) as f32;
_9 = _1;
_5 = _2;
RET = _3 ^ _9;
_11 = !_8;
_7 = -_3;
_3 = RET;
_6 = !_8;
_6 = !_11;
_11 = !_6;
_1 = 8884162665870012926_usize as isize;
_2 = _5 * _5;
_13 = false;
_1 = -_7;
_1 = _9 + _7;
_1 = -_3;
_10 = 61_i8 as f32;
_5 = _2;
_13 = false;
_3 = '\u{eb812}' as isize;
_5 = _2;
RET = _9;
Goto(bb1)
}
bb1 = {
_14 = [414587055400930028_u64,10773196195270739297_u64,7961230248476292497_u64,2183921669751628222_u64,15247711562333274812_u64,17614473748760283156_u64,10297612380335443003_u64,15791365987459510854_u64];
_2 = _5;
_9 = 13614881117492534157_usize as isize;
_11 = _8;
_3 = _7 * _7;
_15 = 4161604527_u32 + 939194569_u32;
_6 = 37547_u16 as u8;
RET = !_7;
Call(_12.fld0 = fn2(_1, _2, _3, _1, _8, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _2 as isize;
_6 = _11 % _11;
_4 = [(-124228061874377781059363027887313320439_i128),40876150724652592900578231083460185888_i128,60825099047359864316975605654366471974_i128,(-26949414369648436875836843240959453409_i128),163740482881130041348921663652194710328_i128,(-101039745570735112621931870569372158682_i128),(-165639553469883029957405428689184481114_i128),(-43241507925325568049440342008437799061_i128)];
_15 = 1863953471_u32 >> _2;
_19 = 56_i8 as u64;
_6 = _11 + _8;
_8 = _11;
RET = -_1;
_13 = !true;
_19 = 16195090492102943268_u64 & 15095369146129846002_u64;
_6 = _8 * _8;
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
_11 = _6;
RET = _2 as isize;
_8 = !_6;
_2 = _5;
RET = _3 - _7;
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
Goto(bb3)
}
bb3 = {
_11 = _8;
_10 = 14128_u16 as f32;
_1 = 12972_u16 as isize;
_6 = _8;
_16 = !_11;
_21 = (_15,);
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
_21 = (_15,);
_11 = !_6;
_15 = _21.0;
_22 = _10 as isize;
_22 = _7 - _7;
_17 = Adt43::Variant0 { fld0: _12.fld0 };
_12.fld0 = Field::<*mut u16>(Variant(_17, 0), 0);
_15 = _21.0 * _21.0;
_2 = !_5;
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
_23.1 = [8812_i16,18928_i16,(-22279_i16)];
Goto(bb4)
}
bb4 = {
_19 = '\u{602f4}' as u64;
_21 = (_15,);
_21 = (_15,);
_21 = (_15,);
_9 = !_3;
RET = _22;
SetDiscriminant(_17, 3);
place!(Field::<usize>(Variant(_17, 3), 2)) = 6_usize << _15;
_8 = _6;
RET = !_3;
place!(Field::<i16>(Variant(_17, 3), 4)) = -9470_i16;
_1 = RET;
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).0 = [(-1_i8),(-19_i8),(-50_i8),(-24_i8),76_i8,18_i8,60_i8];
_11 = _6;
place!(Field::<i16>(Variant(_17, 3), 4)) = 9778_i16;
_25 = -_1;
_10 = _16 as f32;
place!(Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5)).1 = (_21.0,);
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).1 = _10;
_11 = _19 as u8;
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).2 = [54632065431067617901822616586868570657_i128,(-135936349277853097510631821784548233547_i128),(-131515521290348615061656129709131180262_i128),89580654241706476834572254346599605121_i128,(-165288115204246277961810601657904848866_i128),(-151428784295561386979511184157953417616_i128),158800780972102687369695453106504076196_i128,(-62355252855910830175581882719065860659_i128)];
_15 = Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5).1.0 >> _2;
place!(Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5)) = (Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0).0, _21);
_17 = Adt43::Variant0 { fld0: _12.fld0 };
_4 = [(-148342143923814295249347668456436312394_i128),(-4625165898450822470118494935151048308_i128),(-24228189914076890412298723168150651349_i128),122348509119853262841105521457189871262_i128,(-101218832464435023902872224172901880565_i128),113347945063820987413127947670900370395_i128,(-56512119261515218100706352721619526401_i128),10997299797616667135216722949821804939_i128];
Goto(bb5)
}
bb5 = {
_22 = -_25;
RET = -_1;
_21.0 = (-165971764919383912809257160777243332738_i128) as u32;
_24 = _11 as i128;
_7 = !_9;
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
_15 = _21.0 & _21.0;
_3 = -_1;
_9 = -_1;
_23.0 = (-19403_i16) << _2;
_11 = _6;
Goto(bb6)
}
bb6 = {
_13 = !true;
_21.0 = _15;
_29 = !_21.0;
_27 = _25;
_5 = !_2;
_26 = _24 | _24;
_12 = Adt53 { fld0: Field::<*mut u16>(Variant(_17, 0), 0) };
SetDiscriminant(_17, 1);
RET = _22 | _27;
place!(Field::<isize>(Variant(_17, 1), 2)) = !RET;
_25 = Field::<isize>(Variant(_17, 1), 2);
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).3 = _23.0;
_21 = (_29,);
_15 = _2 as u32;
_10 = _24 as f32;
_3 = 9454_u16 as isize;
Goto(bb7)
}
bb7 = {
place!(Field::<f64>(Variant(_17, 1), 5)) = _10 as f64;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).2 = [_19,_19];
RET = _23.0 as isize;
_23.0 = Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).3 * Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).3;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).3 = _15 as i16;
_27 = !RET;
_9 = (-12_i8) as isize;
_32 = [_26,_26,_24,_26,_26,_24,_24,_26];
_7 = -_25;
_31.0 = _29;
place!(Field::<(isize, [u64; 2])>(Variant(_17, 1), 3)) = (_1, Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).2);
_15 = _31.0 & _31.0;
place!(Field::<isize>(Variant(_17, 1), 2)) = RET;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)) = (_23.0, 7_usize, Field::<(isize, [u64; 2])>(Variant(_17, 1), 3).1, _23.0);
_13 = true;
match Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).1 {
0 => bb3,
1 => bb8,
7 => bb10,
_ => bb9
}
}
bb8 = {
_19 = '\u{602f4}' as u64;
_21 = (_15,);
_21 = (_15,);
_21 = (_15,);
_9 = !_3;
RET = _22;
SetDiscriminant(_17, 3);
place!(Field::<usize>(Variant(_17, 3), 2)) = 6_usize << _15;
_8 = _6;
RET = !_3;
place!(Field::<i16>(Variant(_17, 3), 4)) = -9470_i16;
_1 = RET;
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).0 = [(-1_i8),(-19_i8),(-50_i8),(-24_i8),76_i8,18_i8,60_i8];
_11 = _6;
place!(Field::<i16>(Variant(_17, 3), 4)) = 9778_i16;
_25 = -_1;
_10 = _16 as f32;
place!(Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5)).1 = (_21.0,);
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).1 = _10;
_11 = _19 as u8;
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0)).2 = [54632065431067617901822616586868570657_i128,(-135936349277853097510631821784548233547_i128),(-131515521290348615061656129709131180262_i128),89580654241706476834572254346599605121_i128,(-165288115204246277961810601657904848866_i128),(-151428784295561386979511184157953417616_i128),158800780972102687369695453106504076196_i128,(-62355252855910830175581882719065860659_i128)];
_15 = Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5).1.0 >> _2;
place!(Field::<([i8; 7], (u32,))>(Variant(_17, 3), 5)) = (Field::<([i8; 7], f32, [i128; 8])>(Variant(_17, 3), 0).0, _21);
_17 = Adt43::Variant0 { fld0: _12.fld0 };
_4 = [(-148342143923814295249347668456436312394_i128),(-4625165898450822470118494935151048308_i128),(-24228189914076890412298723168150651349_i128),122348509119853262841105521457189871262_i128,(-101218832464435023902872224172901880565_i128),113347945063820987413127947670900370395_i128,(-56512119261515218100706352721619526401_i128),10997299797616667135216722949821804939_i128];
Goto(bb5)
}
bb9 = {
_14 = [414587055400930028_u64,10773196195270739297_u64,7961230248476292497_u64,2183921669751628222_u64,15247711562333274812_u64,17614473748760283156_u64,10297612380335443003_u64,15791365987459510854_u64];
_2 = _5;
_9 = 13614881117492534157_usize as isize;
_11 = _8;
_3 = _7 * _7;
_15 = 4161604527_u32 + 939194569_u32;
_6 = 37547_u16 as u8;
RET = !_7;
Call(_12.fld0 = fn2(_1, _2, _3, _1, _8, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_9 = Field::<isize>(Variant(_17, 1), 2);
_34 = [(-90_i8),93_i8,104_i8,(-88_i8),(-41_i8),(-111_i8),(-76_i8)];
_6 = !_8;
_35 = !_13;
_21 = _31;
_23.0 = !Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).3;
_23.1 = [Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).0,_23.0,_23.0];
_29 = '\u{6f7a3}' as u32;
place!(Field::<f64>(Variant(_17, 1), 5)) = _9 as f64;
place!(Field::<[u64; 2]>(Variant(_17, 1), 4)) = Field::<(isize, [u64; 2])>(Variant(_17, 1), 3).1;
_10 = Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).1 as f32;
_28 = _35;
_9 = !_25;
_35 = !_13;
_33 = Field::<isize>(Variant(_17, 1), 2);
_7 = Field::<isize>(Variant(_17, 1), 2);
_29 = !_21.0;
_7 = _25;
_28 = _35 | _35;
_28 = _21.0 == _15;
_37 = _28 as i128;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).0 = _23.0 & _23.0;
_6 = _35 as u8;
_21 = _31;
Call(_38.1 = core::intrinsics::transmute(_24), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<(isize, [u64; 2])>(Variant(_17, 1), 3)).1 = [_19,_19];
_38.0 = _9 << Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).1;
_33 = Field::<isize>(Variant(_17, 1), 2) - _38.0;
_21 = (_31.0,);
_34 = [(-55_i8),115_i8,(-44_i8),(-29_i8),113_i8,(-21_i8),66_i8];
RET = Field::<f64>(Variant(_17, 1), 5) as isize;
_38.0 = _9 >> _2;
_6 = !_8;
_31.0 = !_15;
_17 = Adt43::Variant0 { fld0: _12.fld0 };
_29 = !_15;
_38.1 = [_19,_19];
_40 = _10 - _10;
_40 = -_10;
_31.0 = _29 + _29;
Goto(bb12)
}
bb12 = {
_39 = 45276_u16 as isize;
_30 = _14;
_42 = !_8;
_10 = _40 - _40;
_42 = _19 as u8;
_10 = _40 * _40;
_12 = Adt53 { fld0: Field::<*mut u16>(Variant(_17, 0), 0) };
_39 = (-3852481249064931073_i64) as isize;
_31.0 = _15 << _2;
_31 = (_21.0,);
_5 = !_2;
_23.1 = [_23.0,_23.0,_23.0];
_8 = _6;
_25 = _9;
_38.0 = _23.0 as isize;
_31 = (_21.0,);
_14 = [_19,_19,_19,_19,_19,_19,_19,_19];
_37 = _40 as i128;
_12 = Adt53 { fld0: Field::<*mut u16>(Variant(_17, 0), 0) };
SetDiscriminant(_17, 1);
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).2 = [_19,_19];
_9 = _7 >> _5;
_14 = _30;
_4 = [_37,_37,_37,_26,_37,_37,_37,_37];
Goto(bb13)
}
bb13 = {
_39 = (-25857554_i32) as isize;
_2 = _40 as u128;
_44 = 14628635880031869257_usize as i32;
_27 = !_33;
RET = 83_i8 as isize;
_33 = _27 >> _2;
_45.0 = _23.0;
_13 = !_28;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).0 = _23.0;
_22 = _45.0 as isize;
_31 = (_15,);
place!(Field::<(isize, [u64; 2])>(Variant(_17, 1), 3)).1 = _38.1;
place!(Field::<(isize, [u64; 2])>(Variant(_17, 1), 3)).0 = _27 * _33;
_41 = _40 + _40;
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).2 = _38.1;
_23.0 = !Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).0;
_15 = !_29;
_45.3 = !Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0).0;
_13 = !_28;
Goto(bb14)
}
bb14 = {
place!(Field::<(i16, usize, [u64; 2], i16)>(Variant(_17, 1), 0)).0 = '\u{dfca5}' as i16;
_36 = &_6;
_23.0 = _45.3;
_21 = _31;
_6 = _16 ^ _8;
_45.1 = 2898633841765296626_usize;
_25 = Field::<(isize, [u64; 2])>(Variant(_17, 1), 3).0 ^ Field::<(isize, [u64; 2])>(Variant(_17, 1), 3).0;
_4 = [_37,_37,_37,_37,_37,_37,_37,_37];
_16 = _45.1 as u8;
_47.2 = [_37,_37,_37,_37,_37,_37,_37,_37];
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(1_usize, 5_usize, Move(_5), 32_usize, Move(_32), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(1_usize, 7_usize, Move(_7), 13_usize, Move(_13), 16_usize, Move(_16), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(1_usize, 21_usize, Move(_21), 15_usize, Move(_15), 19_usize, Move(_19), 28_usize, Move(_28)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(1_usize, 29_usize, Move(_29), 11_usize, Move(_11), 25_usize, Move(_25), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(1_usize, 31_usize, Move(_31), 51_usize, _51, 51_usize, _51, 51_usize, _51), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: u128,mut _3: isize,mut _4: isize,mut _5: u8,mut _6: isize) -> *mut u16 {
mir! {
type RET = *mut u16;
let _7: ([i8; 7], f32, [i128; 8]);
let _8: isize;
let _9: isize;
let _10: f64;
let _11: i16;
let _12: (i16, [i16; 3]);
let _13: Adt45;
let _14: Adt49;
let _15: f32;
let _16: [u64; 8];
let _17: Adt54;
let _18: i128;
let _19: Adt39;
let _20: char;
let _21: ([i8; 7], f32, [i128; 8]);
let _22: (i16, usize, [u64; 2], i16);
let _23: ([i8; 7], (u32,));
let _24: Adt41;
let _25: i128;
let _26: u16;
let _27: Adt45;
let _28: [i8; 7];
let _29: Adt44;
let _30: Adt50;
let _31: ();
let _32: ();
{
_2 = !232665831399152489787120465661314325202_u128;
_5 = 21_u8 - 6_u8;
_5 = 136_u8;
_3 = _1 << _6;
_6 = 1548986349_i32 as isize;
_5 = 38_u8;
_2 = 3_usize as u128;
_2 = 60351667953739603128204160238816165624_u128 + 105154014011831163988958462300279423938_u128;
_5 = 72_u8 << _1;
_1 = 477244066_i32 as isize;
_3 = _4;
_7.2 = [159532075098036004563480904314852289741_i128,141142200205325769180903252999684418649_i128,(-33409700328072912421340831458132691230_i128),52666149583081157293817335788767949453_i128,169993344470349482195473996917159552303_i128,130226427825439781390530970541975661307_i128,(-75334652282148745533897747573816496133_i128),(-102510222627270160186317685657750896142_i128)];
_1 = !_3;
_7.1 = 31516_u16 as f32;
_7.0 = [(-127_i8),35_i8,(-3_i8),(-71_i8),(-44_i8),90_i8,46_i8];
_7.2 = [(-38520155022190778615874116082930328977_i128),142901110094710092442191358085609036283_i128,56781912889841782878068798952085514729_i128,(-153178280966761890672717951087450805842_i128),108626919077098714059343431532073280380_i128,(-106110246914925565852054116900479484204_i128),158393136578085633438212399828088621995_i128,80605377736372488227441662296790275686_i128];
_3 = _4;
_7.1 = 6973_i16 as f32;
_6 = _3 - _3;
_7.1 = _2 as f32;
_6 = _3 << _1;
_3 = !_6;
_1 = true as isize;
_2 = 180435592676863452525233617244756498614_u128 - 112320214880684136354417242230470581746_u128;
_2 = !254477929154387380972012769106057313691_u128;
_5 = (-163249271_i32) as u8;
Goto(bb1)
}
bb1 = {
_7.2 = [(-145518036326207441587223206554521911515_i128),(-14145453862799282558086336916764161870_i128),(-122791381115666983085774559007866041056_i128),135917201609514592954103896342440113636_i128,44073950678426519402846325856547968529_i128,49516661675047119207356075426039385489_i128,(-109123907757830246442921773158931296913_i128),139781823634752407290934472795211676571_i128];
_1 = !_6;
_9 = _7.1 as isize;
_2 = 35152_u16 as u128;
_2 = 178009642482043694026358539748257003489_u128 | 101531224398407437127308680026922896973_u128;
_4 = -_1;
_1 = -_6;
_4 = '\u{e2ed7}' as isize;
_7.2 = [153903191590789330749527224187025947218_i128,137176158309013650492943386067050263027_i128,(-4002532991299753501759871650059632586_i128),(-65781135211751653163167795765585524311_i128),(-48743197524983590400307201345954342196_i128),(-121537038903792874615048164554700700698_i128),66410863254384490589556734931385962085_i128,16889921414613158941408142460788592776_i128];
_7.0 = [(-113_i8),(-80_i8),(-7_i8),(-70_i8),12_i8,122_i8,83_i8];
_2 = 152258143217760930934545816824763323132_u128 ^ 207582805762611319078710357720297676616_u128;
_8 = _6 - _6;
_11 = 16370_i16;
_12.1 = [_11,_11,_11];
_10 = _11 as f64;
_2 = 75376848093201554287032197379091734695_u128 | 117842152903350704573197593192449673312_u128;
_12.1 = [_11,_11,_11];
_10 = (-9131502523542492279_i64) as f64;
_9 = (-2112030256_i32) as isize;
_12.0 = _11;
_9 = -_6;
_2 = 307839604_i32 as u128;
_2 = 316702113336191797267795385031128875842_u128;
_8 = _6;
_5 = 217_u8 & 178_u8;
Call(_8 = fn3(_7.2, _7.2, _7.0, _3, _3, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _9 + _9;
_10 = _5 as f64;
_9 = !_1;
_3 = 2222733098_u32 as isize;
_3 = _1 >> _6;
_3 = -_6;
_7.2 = [(-43856702615891984222363659450813546923_i128),(-137544189001768888379181234600329292371_i128),119775276157939860263085720995838394640_i128,(-151217528095015158320492771152489837236_i128),46293818841673542949971327661865033226_i128,(-145742981070635328183603205720650867211_i128),133732551279438649531756123118622978899_i128,4932354340301644224776116392729106926_i128];
_7.0 = [(-62_i8),(-1_i8),(-112_i8),2_i8,(-105_i8),18_i8,51_i8];
Goto(bb3)
}
bb3 = {
_3 = _1 | _9;
_9 = -_3;
_8 = _3 & _1;
_5 = false as u8;
_15 = 10752080655183545419_usize as f32;
_10 = 709164880_i32 as f64;
_12.0 = (-545921726_i32) as i16;
_12.0 = _11;
_5 = 37_u8;
_6 = -_8;
_16 = [15272657231225215409_u64,1089681921373484888_u64,17455157668073176813_u64,716471859015477494_u64,9195366434035105265_u64,9050421286380356037_u64,4927973194587928510_u64,8172159872731847562_u64];
_8 = _6 & _9;
_7.1 = -_15;
_7.1 = (-112250258155976539492232919401523706391_i128) as f32;
_10 = _11 as f64;
_3 = '\u{696c3}' as isize;
_12.1 = [_11,_12.0,_11];
_7.0 = [82_i8,76_i8,(-35_i8),15_i8,90_i8,(-88_i8),(-59_i8)];
_16 = [16857727020431697358_u64,9331637194466800167_u64,9927356776986638870_u64,13040334101662972653_u64,14428562379079736731_u64,5121730739368461260_u64,16590569348147730778_u64,9149182627840777977_u64];
_3 = !_8;
_5 = !107_u8;
_7.1 = -_15;
_12.0 = _11 + _11;
_8 = _7.1 as isize;
Goto(bb4)
}
bb4 = {
_7.0 = [(-19_i8),28_i8,100_i8,(-47_i8),(-110_i8),(-32_i8),(-19_i8)];
_12.1 = [_12.0,_11,_11];
_12.0 = 10005424414766428176_usize as i16;
_4 = _3 & _3;
match _2 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
316702113336191797267795385031128875842 => bb11,
_ => bb10
}
}
bb5 = {
_3 = _1 | _9;
_9 = -_3;
_8 = _3 & _1;
_5 = false as u8;
_15 = 10752080655183545419_usize as f32;
_10 = 709164880_i32 as f64;
_12.0 = (-545921726_i32) as i16;
_12.0 = _11;
_5 = 37_u8;
_6 = -_8;
_16 = [15272657231225215409_u64,1089681921373484888_u64,17455157668073176813_u64,716471859015477494_u64,9195366434035105265_u64,9050421286380356037_u64,4927973194587928510_u64,8172159872731847562_u64];
_8 = _6 & _9;
_7.1 = -_15;
_7.1 = (-112250258155976539492232919401523706391_i128) as f32;
_10 = _11 as f64;
_3 = '\u{696c3}' as isize;
_12.1 = [_11,_12.0,_11];
_7.0 = [82_i8,76_i8,(-35_i8),15_i8,90_i8,(-88_i8),(-59_i8)];
_16 = [16857727020431697358_u64,9331637194466800167_u64,9927356776986638870_u64,13040334101662972653_u64,14428562379079736731_u64,5121730739368461260_u64,16590569348147730778_u64,9149182627840777977_u64];
_3 = !_8;
_5 = !107_u8;
_7.1 = -_15;
_12.0 = _11 + _11;
_8 = _7.1 as isize;
Goto(bb4)
}
bb6 = {
_1 = _9 + _9;
_10 = _5 as f64;
_9 = !_1;
_3 = 2222733098_u32 as isize;
_3 = _1 >> _6;
_3 = -_6;
_7.2 = [(-43856702615891984222363659450813546923_i128),(-137544189001768888379181234600329292371_i128),119775276157939860263085720995838394640_i128,(-151217528095015158320492771152489837236_i128),46293818841673542949971327661865033226_i128,(-145742981070635328183603205720650867211_i128),133732551279438649531756123118622978899_i128,4932354340301644224776116392729106926_i128];
_7.0 = [(-62_i8),(-1_i8),(-112_i8),2_i8,(-105_i8),18_i8,51_i8];
Goto(bb3)
}
bb7 = {
_7.2 = [(-145518036326207441587223206554521911515_i128),(-14145453862799282558086336916764161870_i128),(-122791381115666983085774559007866041056_i128),135917201609514592954103896342440113636_i128,44073950678426519402846325856547968529_i128,49516661675047119207356075426039385489_i128,(-109123907757830246442921773158931296913_i128),139781823634752407290934472795211676571_i128];
_1 = !_6;
_9 = _7.1 as isize;
_2 = 35152_u16 as u128;
_2 = 178009642482043694026358539748257003489_u128 | 101531224398407437127308680026922896973_u128;
_4 = -_1;
_1 = -_6;
_4 = '\u{e2ed7}' as isize;
_7.2 = [153903191590789330749527224187025947218_i128,137176158309013650492943386067050263027_i128,(-4002532991299753501759871650059632586_i128),(-65781135211751653163167795765585524311_i128),(-48743197524983590400307201345954342196_i128),(-121537038903792874615048164554700700698_i128),66410863254384490589556734931385962085_i128,16889921414613158941408142460788592776_i128];
_7.0 = [(-113_i8),(-80_i8),(-7_i8),(-70_i8),12_i8,122_i8,83_i8];
_2 = 152258143217760930934545816824763323132_u128 ^ 207582805762611319078710357720297676616_u128;
_8 = _6 - _6;
_11 = 16370_i16;
_12.1 = [_11,_11,_11];
_10 = _11 as f64;
_2 = 75376848093201554287032197379091734695_u128 | 117842152903350704573197593192449673312_u128;
_12.1 = [_11,_11,_11];
_10 = (-9131502523542492279_i64) as f64;
_9 = (-2112030256_i32) as isize;
_12.0 = _11;
_9 = -_6;
_2 = 307839604_i32 as u128;
_2 = 316702113336191797267795385031128875842_u128;
_8 = _6;
_5 = 217_u8 & 178_u8;
Call(_8 = fn3(_7.2, _7.2, _7.0, _3, _3, _7), ReturnTo(bb2), UnwindUnreachable())
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
_5 = 47_u8;
_12.1 = [_12.0,_11,_11];
_11 = !_12.0;
_7.2 = [131078406417569563341191417009235715884_i128,145940140188011919808281592152308754604_i128,(-73430085912228368846348434932361626394_i128),(-60986411767955897949811922755980616400_i128),(-62932927890932244696067126091608658411_i128),127790452817475222830649338728216315467_i128,(-136082374859264153224532697296801074386_i128),(-143374737093464601155357650095650970536_i128)];
_2 = 194362419671870093910511652787490107941_u128;
_10 = 1870088573_u32 as f64;
_11 = _2 as i16;
_8 = _4 - _4;
_22.2 = [18198258177555819150_u64,4698019830910649811_u64];
_22.0 = _12.0 & _11;
Goto(bb12)
}
bb12 = {
_21.1 = _7.1;
_22.0 = _11;
_23.0 = [(-9_i8),(-22_i8),104_i8,(-107_i8),96_i8,35_i8,(-99_i8)];
_18 = 167279343310090485889329222728958508826_i128 * (-47659857982286399454852602918929525262_i128);
_10 = _8 as f64;
_23.1 = (2084540079_u32,);
_2 = 190150935794873924685303082714739998941_u128 ^ 143497457533058244659346280779941944289_u128;
_22.3 = !_22.0;
_12.1 = [_12.0,_12.0,_22.0];
_23.1 = (3735083871_u32,);
_6 = _3;
_23.1 = (908582253_u32,);
_21.2 = [_18,_18,_18,_18,_18,_18,_18,_18];
_21 = _7;
_4 = _6 << _8;
_4 = -_9;
_21.0 = _7.0;
_21.0 = [(-127_i8),(-79_i8),(-101_i8),74_i8,(-56_i8),(-118_i8),53_i8];
_22.2 = [2542673251850373318_u64,3828070098731454871_u64];
_7.0 = [35_i8,17_i8,(-4_i8),51_i8,(-119_i8),(-115_i8),(-125_i8)];
Call(_18 = core::intrinsics::bswap((-12941550585294060999320562073453038744_i128)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_15 = -_21.1;
_21.0 = _23.0;
match _23.1.0 {
0 => bb7,
1 => bb12,
2 => bb14,
908582253 => bb16,
_ => bb15
}
}
bb14 = {
_7.2 = [(-145518036326207441587223206554521911515_i128),(-14145453862799282558086336916764161870_i128),(-122791381115666983085774559007866041056_i128),135917201609514592954103896342440113636_i128,44073950678426519402846325856547968529_i128,49516661675047119207356075426039385489_i128,(-109123907757830246442921773158931296913_i128),139781823634752407290934472795211676571_i128];
_1 = !_6;
_9 = _7.1 as isize;
_2 = 35152_u16 as u128;
_2 = 178009642482043694026358539748257003489_u128 | 101531224398407437127308680026922896973_u128;
_4 = -_1;
_1 = -_6;
_4 = '\u{e2ed7}' as isize;
_7.2 = [153903191590789330749527224187025947218_i128,137176158309013650492943386067050263027_i128,(-4002532991299753501759871650059632586_i128),(-65781135211751653163167795765585524311_i128),(-48743197524983590400307201345954342196_i128),(-121537038903792874615048164554700700698_i128),66410863254384490589556734931385962085_i128,16889921414613158941408142460788592776_i128];
_7.0 = [(-113_i8),(-80_i8),(-7_i8),(-70_i8),12_i8,122_i8,83_i8];
_2 = 152258143217760930934545816824763323132_u128 ^ 207582805762611319078710357720297676616_u128;
_8 = _6 - _6;
_11 = 16370_i16;
_12.1 = [_11,_11,_11];
_10 = _11 as f64;
_2 = 75376848093201554287032197379091734695_u128 | 117842152903350704573197593192449673312_u128;
_12.1 = [_11,_11,_11];
_10 = (-9131502523542492279_i64) as f64;
_9 = (-2112030256_i32) as isize;
_12.0 = _11;
_9 = -_6;
_2 = 307839604_i32 as u128;
_2 = 316702113336191797267795385031128875842_u128;
_8 = _6;
_5 = 217_u8 & 178_u8;
Call(_8 = fn3(_7.2, _7.2, _7.0, _3, _3, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_12.0 = _11;
_15 = _21.1;
_22.3 = _22.0 - _12.0;
_3 = _6 << _6;
_5 = _22.3 as u8;
_22.3 = _11 * _11;
_21.0 = [(-116_i8),(-122_i8),53_i8,87_i8,(-15_i8),12_i8,103_i8];
_7.0 = _21.0;
_26 = !4428_u16;
_5 = _9 as u8;
_18 = 158288671310314669162793209187067960055_i128 << _8;
_7 = (_23.0, _15, _21.2);
_5 = 7_i8 as u8;
_8 = _3;
_23.1 = (3408811303_u32,);
_21.2 = [_18,_18,_18,_18,_18,_18,_18,_18];
RET = core::ptr::addr_of_mut!(_26);
_5 = !159_u8;
RET = core::ptr::addr_of_mut!((*RET));
_7.1 = _26 as f32;
_29 = Adt44::Variant2 { fld0: _16,fld1: (-42_i8) };
RET = core::ptr::addr_of_mut!(_26);
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(2_usize, 16_usize, Move(_16), 12_usize, Move(_12), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(2_usize, 9_usize, Move(_9), 6_usize, Move(_6), 26_usize, Move(_26), 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i8; 7],mut _4: isize,mut _5: isize,mut _6: ([i8; 7], f32, [i128; 8])) -> isize {
mir! {
type RET = isize;
let _7: ([i8; 7], f32, [i128; 8]);
let _8: i64;
let _9: f64;
let _10: [i8; 7];
let _11: *const [u128; 6];
let _12: [i16; 3];
let _13: f64;
let _14: ([u128; 6], &'static u8);
let _15: f32;
let _16: (isize, [u64; 2]);
let _17: isize;
let _18: [i16; 8];
let _19: bool;
let _20: [u64; 8];
let _21: f64;
let _22: [u64; 2];
let _23: i8;
let _24: f32;
let _25: isize;
let _26: isize;
let _27: isize;
let _28: isize;
let _29: [i16; 8];
let _30: f32;
let _31: (u32,);
let _32: Adt43;
let _33: [i16; 8];
let _34: Adt45;
let _35: f64;
let _36: f32;
let _37: ([i8; 7], (u32,));
let _38: *mut i128;
let _39: Adt54;
let _40: (isize, [u64; 2]);
let _41: isize;
let _42: ();
let _43: ();
{
RET = _5 - _5;
_6.0 = [(-48_i8),(-63_i8),89_i8,(-128_i8),59_i8,78_i8,28_i8];
RET = _5 | _5;
_1 = [(-45163831451606906213253415048766124683_i128),(-50638560475896648800958387152189092725_i128),(-27030421272181804948235634703519131198_i128),161852886740166356042221330468154830641_i128,(-79801485025310467282941451775707847659_i128),(-133444462858278634973614315613792876280_i128),150799572279193560614758988880731235473_i128,(-1777630361025397178974981117705352255_i128)];
RET = _5;
_3 = _6.0;
_1 = [153658964417427361856050269101619779364_i128,(-3454712723518415350864443428359849970_i128),(-166568205864453084359429049738954645945_i128),38693275295421090025863817001238884786_i128,98861833991502255953174658389860771445_i128,(-125174929690286885037035221874001939620_i128),(-23484231819159561415501405675267385683_i128),(-79650968071753958510771490305000355599_i128)];
_6.1 = 2654007343109847342454114619235765799_i128 as f32;
_1 = [165830162732266458627442198878320310121_i128,(-135972849341099164539473129877387542468_i128),51354791280368186289069411156103371746_i128,159144410191069884192709388658346569056_i128,108625740306816271811265635684008295709_i128,(-16532046023459142337385558329492197790_i128),7050589155965322142160619135724943220_i128,(-122551511517158702222502964650214432659_i128)];
_6.1 = 40367902740831978587274135377597953543_u128 as f32;
_5 = (-1957120891064798501_i64) as isize;
_7.2 = [169883779285236008884523164088143712587_i128,168512459823311459059710458435188519258_i128,(-9905436505177214567017944612533042863_i128),(-95021515537566534634157947839380428596_i128),118664667814966328444933555494615828538_i128,76510013393288382632040099031143177_i128,(-102711499408933922519392772274125494944_i128),(-13255327901249837598387619181899457370_i128)];
_6.2 = [129579151320896828263676073528630668226_i128,54266982538418795226711954308506947719_i128,(-9143424573911391165054032479917073379_i128),(-98246549255698272103837037982666097169_i128),(-84784191217565242173120665639798850708_i128),(-5313667125543163642639612193578787745_i128),(-91462085534459257563293119735456970119_i128),151724505414616528388033382301113153463_i128];
RET = 2697669799_u32 as isize;
_7.0 = [97_i8,20_i8,(-75_i8),28_i8,57_i8,84_i8,120_i8];
_4 = _5;
_5 = !RET;
_2 = [(-154377892802662759851272920783829102146_i128),(-35285565652115802481106017182683920774_i128),(-125213267653166879934146934070366953605_i128),(-33444175802907915526430187318588454406_i128),(-62246239017219072698854069380973825421_i128),(-131992606278157765164752274323906987370_i128),153541561228475290216749073676773317951_i128,(-140068708600312082881717228656053156481_i128)];
_2 = [(-118578612513279631281879767673165251541_i128),89780152958179728795981478106499961534_i128,(-41133033276656646972068578685828896844_i128),(-31818284920315409562363580876497017100_i128),167483093227268439900910594057777281794_i128,73442464925688239143035442009331587976_i128,(-169884171186223582137102575595874587539_i128),134618814744650945069296269473975248111_i128];
_4 = RET >> RET;
_7 = _6;
_7 = _6;
_1 = [50308973653122489625160471217534063460_i128,14995824998799323604043054029722168352_i128,49082410229458675142956376473557397061_i128,(-63716293589784601132032426550200407745_i128),154715925351691879792982468559016481049_i128,(-112351273252793354734855550943127166726_i128),(-8635434262888628703065924459510881921_i128),44454348889909575033113315656401614715_i128];
_7.1 = 5_usize as f32;
RET = _4;
_4 = _6.1 as isize;
_6.0 = _3;
_7.0 = [(-99_i8),51_i8,113_i8,(-108_i8),(-17_i8),(-120_i8),(-21_i8)];
_6.2 = _7.2;
Goto(bb1)
}
bb1 = {
_3 = _6.0;
_7 = (_3, _6.1, _6.2);
_7.1 = _6.1 * _6.1;
_7 = (_3, _6.1, _6.2);
_7.2 = [95535049389586558753901717374211863127_i128,15263407597974737264488799076226117771_i128,(-111955711824698493436036826176906231824_i128),8933870194355282736410741153798836687_i128,(-46637024033121724279269861012377810786_i128),(-157350177529910088163795847982754568176_i128),(-97429115598600477410021975042611557587_i128),(-126352235111821054024126384728346394924_i128)];
_3 = _7.0;
_5 = 69370058405860001524056152301220294030_u128 as isize;
_4 = 75_u8 as isize;
_7.2 = _1;
_10 = [113_i8,76_i8,(-100_i8),68_i8,111_i8,(-15_i8),0_i8];
_9 = (-31885330000785112342113646498292743159_i128) as f64;
_4 = -RET;
_5 = 1374291238501637422_u64 as isize;
_6.1 = -_7.1;
_6 = (_10, _7.1, _2);
_10 = _6.0;
_8 = '\u{2c5c0}' as i64;
RET = _4 | _4;
RET = -_5;
_4 = true as isize;
_3 = [121_i8,82_i8,81_i8,126_i8,(-61_i8),31_i8,0_i8];
_7.1 = -_6.1;
_6.0 = _7.0;
Call(RET = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = RET >> RET;
_12 = [(-5364_i16),(-11681_i16),28300_i16];
_4 = _5;
_2 = [(-41412564954190938574431691958189996583_i128),(-123816877955357701518642579613665555986_i128),(-126912397627085585902485715146650531255_i128),(-72978446677086735715611588923471605591_i128),120934386854793086937253777686981879011_i128,(-23887038992049505152257295234403812232_i128),(-84443102479691007540290191651760067221_i128),(-107813941749609108234666958886274731181_i128)];
_9 = 23_u8 as f64;
_7.0 = [(-52_i8),36_i8,41_i8,(-9_i8),(-62_i8),72_i8,(-36_i8)];
_7.2 = [22345751678380405513284968405513126100_i128,(-146937056115920567955429880866790960708_i128),32108447719783806626703181973522548884_i128,(-46019009580658996533264574927428208684_i128),22039921041355971626181137148889224945_i128,(-59718541435769932251547319396942694435_i128),126444147813504213219007648553432160960_i128,159221270870971852350261591704223135736_i128];
_7.0 = _10;
_6 = (_3, _7.1, _2);
_13 = -_9;
_6.1 = 2455_u16 as f32;
_10 = [79_i8,20_i8,(-38_i8),86_i8,70_i8,15_i8,(-50_i8)];
_2 = _1;
_6.2 = [(-99130522619482900341890375241070083375_i128),38137779569205123803249332026983735767_i128,(-18212364221559152013234160649778228350_i128),35389029608751866033239764518834239390_i128,110743245847750577235264333139074575841_i128,(-111040279531954958462349124657400366490_i128),(-32669657729323589053372398365527412793_i128),150745667896528085867670790359144328612_i128];
_5 = (-474100837_i32) as isize;
_6.2 = [(-168853001004899799725305262375603288682_i128),88215518862030057465721509300351529056_i128,155472081895523787191733418867539241350_i128,167849968286553131454112697366890002278_i128,98248015328242103383958315807006718925_i128,(-101761186034669402017255229012079877316_i128),(-12781277707137537521029434510082896050_i128),130673878349290024635615879292319738085_i128];
_6.2 = [(-60500323863787960338025221339674437420_i128),(-30031048450756492411826545902789356507_i128),70028816779280315524030456484493010498_i128,71776120528445762068050912351178163887_i128,68675994041954168057631823932091162704_i128,140463434496061840618373379148758508238_i128,21924775191868047660433544113034837843_i128,101540928605407930302993462572439928854_i128];
_2 = [(-55771176269845232687076530405343759178_i128),(-115494081678900146042682468129934351871_i128),157207387064301861084236162308164323311_i128,(-4281362586669575526018196336111583008_i128),(-79677439212860833462318481239661588979_i128),140775257971296450973822333263703073375_i128,(-100270472523845624690423405488126912338_i128),96489763260963430917140023929092455540_i128];
_11 = core::ptr::addr_of!(_14.0);
Call(_13 = fn4(_4, _12, _7, _2, _6.0, _3, _6.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = (_6.0, _6.1, _6.2);
_1 = _2;
(*_11) = [74059330777307328375496202632542759396_u128,47371435090632164190287156177764571781_u128,10754467076869875526501636892317668257_u128,263915835609179276250106019711703715812_u128,337765300852841548661912190221421174034_u128,144514999873822214412491145362285113306_u128];
_7.2 = _2;
_7 = (_10, _6.1, _2);
(*_11) = [281108252159923170120511997564800999277_u128,326099400100634626142662971004503965266_u128,275268467302490066756113982372842618701_u128,17057424084216970187817890265471387678_u128,190549018357423284871910194325288651725_u128,14485766216077777115364924149909259353_u128];
_15 = 182958822523765386708354029305408260533_u128 as f32;
(*_11) = [262046852160872481770465925328752478112_u128,253605835535298474729028275633584681237_u128,106299461387798331235983527646864216637_u128,154900563906900845264377722549777689012_u128,98559063872821739326532711941229610729_u128,331525125864308069072309643306472870101_u128];
_16.0 = _4;
_11 = core::ptr::addr_of!((*_11));
_4 = 152_u8 as isize;
_6 = (_10, _15, _2);
_7 = _6;
_9 = -_13;
_16.0 = _5 << _4;
_18 = [(-7422_i16),11283_i16,(-18989_i16),(-32350_i16),8909_i16,(-10669_i16),(-16982_i16),20973_i16];
_18 = [(-29024_i16),15355_i16,(-8253_i16),31432_i16,(-30734_i16),30068_i16,10003_i16,(-11445_i16)];
_4 = _5 * _16.0;
_14.0 = [127134398272986884091340220715853040675_u128,57139523362764693260362629950411550904_u128,325932253659914974712405535937453419989_u128,141137992637576838524412473479898438894_u128,246003555646429454616913408968382443365_u128,122687697173598429416613757012846893966_u128];
_9 = 13784192294182230909_u64 as f64;
(*_11) = [109612075773370473580111354526135520038_u128,204334977587773257864547280376717656189_u128,130166613007899796613156952994056224487_u128,118565182990298030732792180394692983147_u128,88633260085729262090893452880105562375_u128,82372803069455527573560191208744358743_u128];
_16.1 = [16422274131614385520_u64,7390057045474126454_u64];
Goto(bb4)
}
bb4 = {
_8 = 276958499918447503242041629302606154504_u128 as i64;
_10 = [(-26_i8),(-91_i8),(-23_i8),(-91_i8),17_i8,(-45_i8),86_i8];
Goto(bb5)
}
bb5 = {
_12 = [(-24419_i16),(-6537_i16),32314_i16];
_6 = _7;
_9 = _13;
_12 = [6323_i16,(-32323_i16),(-842_i16)];
_6.1 = _15 + _15;
_21 = _13;
_13 = -_21;
Goto(bb6)
}
bb6 = {
_19 = !true;
RET = _4 ^ _16.0;
RET = !_4;
_12 = [(-9105_i16),22734_i16,(-3989_i16)];
_6.2 = [3353286443242615606663180890591835271_i128,53586516390377755165268697639766050879_i128,10162236374506922687506107634249298342_i128,(-2888447895594787731793869528394847891_i128),37810403953271290774512670440571276287_i128,160578778509158199733606663533333651688_i128,(-89738009420333632045106709477689605314_i128),58208781686255369330134218629069740357_i128];
_24 = 183_u8 as f32;
_23 = (-61_i8);
_19 = !true;
(*_11) = [151912017309027368259054049711744255467_u128,121222791848346818323377692372853148454_u128,139593348561653925226676526058761200908_u128,109208326107466404669596655041840058292_u128,213691273592213523055163151583270825448_u128,318246073519520336511832463354942486019_u128];
_8 = 6682999739717860875_i64 << RET;
_2 = [(-161208534999512060985750390555978248350_i128),114638110173407164575810375125447022220_i128,(-81104269051606251739299969031421971540_i128),(-134510717264609407092905413080142157884_i128),18518377540030956000898110775531109852_i128,(-105800176369876224950050818043143107546_i128),162099485514513767098443091035915914383_i128,(-157045121420084773999928148708965035912_i128)];
_15 = -_7.1;
_13 = -_21;
_7 = (_6.0, _24, _1);
Goto(bb7)
}
bb7 = {
_6.0 = [_23,_23,_23,_23,_23,_23,_23];
_6.0 = [_23,_23,_23,_23,_23,_23,_23];
_4 = 14749762971369377871_u64 as isize;
_12 = [13643_i16,(-12112_i16),(-4009_i16)];
_20 = [7489313155866236577_u64,543262477304720267_u64,17413170412612919188_u64,14050940787074736805_u64,11457232346713216418_u64,15541775078124089950_u64,8050065377578654783_u64,8714273363023011168_u64];
Goto(bb8)
}
bb8 = {
_14.0 = [223474848791120124920794313784495835762_u128,73129396286121156241339361685526867729_u128,276154536184709181289137306525344843632_u128,164776116059339684006455762810448474715_u128,261462893416337649866528254955053843782_u128,224479733054198565453466364749139131993_u128];
_1 = [98891755076923209135299571197374599481_i128,(-152956418064868954658651514905174783639_i128),(-127133920498154537685933871175488922524_i128),(-110811361502646878347928218498284747274_i128),76530290647870987255567085653871110725_i128,(-100583625268324343611085423096132488696_i128),58692851746359517196201192001626337581_i128,82769513070124012304580348237100243840_i128];
RET = _5 ^ _16.0;
_17 = !RET;
_7.1 = -_24;
_14.0 = [189111784804786200071887478025972972311_u128,12139654675480005614416608421422085472_u128,47396188978592499371773644411817525723_u128,266990646230842828974870832881066182950_u128,227586107597363523648035314883817673992_u128,31529811902832803419008039443132012287_u128];
_5 = _21 as isize;
_18 = [(-11152_i16),24890_i16,(-13307_i16),(-9630_i16),13084_i16,(-30874_i16),(-27339_i16),(-11514_i16)];
_7 = _6;
_28 = -RET;
_30 = _15 * _6.1;
_25 = _5 << _16.0;
_31.0 = 3314043268_u32;
_12 = [(-5062_i16),(-5321_i16),(-21105_i16)];
_1 = _6.2;
_13 = _15 as f64;
_29 = [31603_i16,32350_i16,(-20420_i16),(-20358_i16),447_i16,7816_i16,11410_i16,(-7492_i16)];
_14.0 = [217347625745481089355912941120203464662_u128,169385371337210195879096128665760538529_u128,129402758780420724272722627798552562805_u128,153383160752903068672452377119492479442_u128,10249390677296909592895138065744838413_u128,27511355414992795165005989926602052187_u128];
match _23 {
340282366920938463463374607431768211395 => bb9,
_ => bb4
}
}
bb9 = {
(*_11) = [315505705301821916327060226869503699281_u128,121532244778872273705515241645634844751_u128,123875204762774779309237388103448910937_u128,192738910093992064434413456464472149532_u128,189255264366992805435182768773729962784_u128,202444120853863170170151817551557823018_u128];
_25 = _19 as isize;
_10 = _3;
_16.1 = [14604567032570926873_u64,17223037895190402943_u64];
_26 = !_17;
match _31.0 {
0 => bb7,
1 => bb6,
3314043268 => bb10,
_ => bb5
}
}
bb10 = {
_11 = core::ptr::addr_of!(_14.0);
_20 = [10823598615236486779_u64,590733410282802279_u64,2636036234811080188_u64,9543001839874830587_u64,18279161772245045647_u64,1032524261915953568_u64,11001344717158903210_u64,15289668395666760785_u64];
_31.0 = 319902894_u32;
_4 = _16.0;
_19 = true ^ false;
_3 = [_23,_23,_23,_23,_23,_23,_23];
_16.0 = _30 as isize;
_12 = [(-29769_i16),25222_i16,5317_i16];
_4 = _25 << _16.0;
_2 = _7.2;
Call(_27 = core::intrinsics::bswap(_26), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_22 = [5772168352680927888_u64,7742851290287581026_u64];
_16 = (_26, _22);
_13 = _21;
_21 = -_9;
_33 = _18;
_14.0 = [175770012802357393769592282416945335702_u128,255623981715684646047539746503755811753_u128,259772576567172687905875847471734940945_u128,153114544057699874292900845518857345709_u128,202699137937049548306187228559622556315_u128,256744597901607780153876180059890481948_u128];
_22 = [12638281584240189516_u64,3853120083798714045_u64];
_7 = _6;
_20 = [4675912206444231588_u64,3140874361591925650_u64,903430002225100559_u64,2046362178710635637_u64,13414114362570636445_u64,2943875527069185457_u64,4975641599222206473_u64,16927691581559664294_u64];
_4 = 4972753076303585429_u64 as isize;
_12 = [(-2254_i16),6501_i16,(-884_i16)];
_16 = (RET, _22);
_1 = _7.2;
_6.2 = [(-37731273726307183025995463395506036091_i128),(-15981914008250545526588332735515169276_i128),137467154033776665645486525048825204022_i128,(-102061005292369918214191525759917907416_i128),(-46346142015089913746247018554295803258_i128),(-49551130366304102813654946086173288163_i128),(-33141173544406238746107006125768140188_i128),(-41383616589406199578070701784050578479_i128)];
_10 = _3;
_28 = _5 & RET;
_16 = (_26, _22);
_12 = [(-16259_i16),(-10602_i16),30212_i16];
_26 = _5;
_36 = _30;
match _31.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
319902894 => bb12,
_ => bb5
}
}
bb12 = {
_21 = 13861170575948765733_u64 as f64;
_9 = 8149193581760405985_u64 as f64;
RET = _16.0;
_2 = [107240809841675675974537489449346743348_i128,(-153755696788647493704072146312480942415_i128),(-134312230904191984450626198558371975248_i128),124226061463040860919505347963716965891_i128,(-162947894805985326240975840406938789934_i128),(-32724287527556883819698933648778675628_i128),(-42972686593969817643392802519382463207_i128),18402131996186957861721184001286171610_i128];
_37.1.0 = _31.0 * _31.0;
_7.0 = _6.0;
_1 = [(-136760905184939220229665780725843526163_i128),121123027151125484428184396382078592353_i128,(-54635315418129067450369777373534315380_i128),(-111664081410869875694171098629832758892_i128),43714756012514424368561800686221915706_i128,(-498469598249315641128189839452517518_i128),(-19699812729372012310159624469560739557_i128),(-86068916073132804352933493653042054034_i128)];
_22 = [15410428910540520889_u64,14815382605178781075_u64];
_29 = _18;
_6 = _7;
_20 = [7114292837222215524_u64,15328952767371147150_u64,11274035962365295221_u64,14406792057663082171_u64,16610189630659371829_u64,1923470079716036597_u64,2490117779015010224_u64,6134818615014331231_u64];
_35 = _13 - _21;
_20 = [11298096512494006524_u64,7503570549386621911_u64,14805418708138376512_u64,13049262141779119385_u64,10984006718782228689_u64,11522684274803965065_u64,4994429650262751383_u64,4089796597608043949_u64];
RET = !_26;
match _23 {
0 => bb13,
340282366920938463463374607431768211395 => bb15,
_ => bb14
}
}
bb13 = {
_12 = [(-24419_i16),(-6537_i16),32314_i16];
_6 = _7;
_9 = _13;
_12 = [6323_i16,(-32323_i16),(-842_i16)];
_6.1 = _15 + _15;
_21 = _13;
_13 = -_21;
Goto(bb6)
}
bb14 = {
_11 = core::ptr::addr_of!(_14.0);
_20 = [10823598615236486779_u64,590733410282802279_u64,2636036234811080188_u64,9543001839874830587_u64,18279161772245045647_u64,1032524261915953568_u64,11001344717158903210_u64,15289668395666760785_u64];
_31.0 = 319902894_u32;
_4 = _16.0;
_19 = true ^ false;
_3 = [_23,_23,_23,_23,_23,_23,_23];
_16.0 = _30 as isize;
_12 = [(-29769_i16),25222_i16,5317_i16];
_4 = _25 << _16.0;
_2 = _7.2;
Call(_27 = core::intrinsics::bswap(_26), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_8 = -3948806920032503668_i64;
_30 = -_7.1;
_1 = [(-101143804103979398608348793793693414431_i128),(-106549209560107646815853841020886075378_i128),(-113366704848781052537548672741461241560_i128),157037382328415361479276136924274215154_i128,(-95286156155127216645283738707680572483_i128),(-29179553342002242503493599439129094470_i128),114524501801683982613410248256839870378_i128,3523869536377877090720641950886648558_i128];
_4 = 12251094155985655878_usize as isize;
RET = 10393332948377597991_u64 as isize;
_14.0 = [175424740679698187687395910278651131996_u128,148892805254090359378333878611199551015_u128,152342772984580261485382603248066668261_u128,89673290179019013848974414509309685115_u128,182680261330447634559149619993631805485_u128,131413266595855808042071369669863548205_u128];
_37.0 = [_23,_23,_23,_23,_23,_23,_23];
_9 = _13 * _35;
_14.0 = [29256482295932603752551131429746649493_u128,294060376401038065428501574415140607943_u128,88962754065753369026692492438074057276_u128,255343590898941656475015532699343068606_u128,299391908726720967801990431538678607939_u128,285128197544982849702186199296605841449_u128];
_29 = _33;
_29 = [17406_i16,(-10158_i16),(-30956_i16),4834_i16,2379_i16,15667_i16,6584_i16,(-16490_i16)];
_34 = Adt45::Variant2 { fld0: 7506307088646504693_u64,fld1: _37,fld2: (-141691879_i32) };
_40.1 = [8521847942168558724_u64,2276167920592296173_u64];
_40 = _16;
_12 = [(-2251_i16),5388_i16,30223_i16];
place!(Field::<([i8; 7], (u32,))>(Variant(_34, 2), 1)).0 = [_23,_23,_23,_23,_23,_23,_23];
_12 = [20276_i16,25491_i16,(-10333_i16)];
_35 = _13;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(3_usize, 8_usize, Move(_8), 4_usize, Move(_4), 17_usize, Move(_17), 37_usize, Move(_37)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(3_usize, 16_usize, Move(_16), 27_usize, Move(_27), 28_usize, Move(_28), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(3_usize, 33_usize, Move(_33), 2_usize, Move(_2), 26_usize, Move(_26), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: [i16; 3],mut _3: ([i8; 7], f32, [i128; 8]),mut _4: [i128; 8],mut _5: [i8; 7],mut _6: [i8; 7],mut _7: [i128; 8]) -> f64 {
mir! {
type RET = f64;
let _8: *const [u128; 6];
let _9: u128;
let _10: ([i8; 7], (u32,));
let _11: ([i8; 7], f32, [i128; 8]);
let _12: i64;
let _13: bool;
let _14: isize;
let _15: Adt43;
let _16: (i16, [i16; 3]);
let _17: [u128; 6];
let _18: (u32,);
let _19: Adt39;
let _20: [i128; 8];
let _21: isize;
let _22: ([i8; 7], f32, [i128; 8]);
let _23: i32;
let _24: bool;
let _25: isize;
let _26: [i128; 8];
let _27: [u64; 8];
let _28: ([u128; 6], &'static u8);
let _29: f64;
let _30: bool;
let _31: i8;
let _32: char;
let _33: isize;
let _34: [u128; 6];
let _35: (isize, [u64; 2]);
let _36: Adt51;
let _37: isize;
let _38: bool;
let _39: bool;
let _40: *mut [u128; 6];
let _41: Adt49;
let _42: Adt47;
let _43: (isize, [u64; 2]);
let _44: ((i16, [i16; 3]), &'static u8, ([i8; 7], f32, [i128; 8]), ([i8; 7], (u32,)));
let _45: isize;
let _46: [i128; 8];
let _47: bool;
let _48: Adt44;
let _49: ();
let _50: ();
{
_3.0 = [(-18_i8),(-23_i8),53_i8,92_i8,18_i8,(-19_i8),(-53_i8)];
RET = 27_i8 as f64;
_1 = -9223372036854775807_isize;
RET = 63028_u16 as f64;
_3.2 = [(-72970969308791254141796421099001548153_i128),143313942294034701086668415397819056159_i128,(-23947302566526620492738441206134466094_i128),38411726171129684381942115928167462180_i128,143050278462367758883939261532876281034_i128,(-64362624098885746247185976743082593164_i128),2403018861300844123357228504177251048_i128,135894613747061474319450223239117037003_i128];
_4 = [42704997979291831354444337027261807970_i128,50183415564923242997758645780457947851_i128,(-2543829273876576925484578534194257272_i128),43824194215823421468499097319616228519_i128,137636852124549519800398488961637743844_i128,(-97564058209802943718907781912851977011_i128),(-124373469531428015081679310424930550674_i128),158857782102424291345427489505122765011_i128];
Call(_3.2 = fn5(_7, _4, _6, _6, _7, _2, RET, _5, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _7;
_3.1 = 1271552865569896314_u64 as f32;
_2 = [(-12665_i16),(-24072_i16),(-20768_i16)];
RET = 5414043952210463652_usize as f64;
_9 = 24089_i16 as u128;
_5 = [(-39_i8),17_i8,97_i8,124_i8,(-32_i8),80_i8,(-109_i8)];
_3.2 = [(-118662896118047033617088574585157542212_i128),(-132181585555711631458535284238890487609_i128),(-95308285869280996341498039837882730189_i128),151266456507985699795421805125827115384_i128,(-75662629869897764735780453241236069414_i128),(-85340096764725921789961382604173590532_i128),(-148379893733232122327563690463871109507_i128),(-148926801516957165407670128078105900882_i128)];
_1 = 9223372036854775807_isize;
RET = _1 as f64;
_10.0 = _6;
_12 = (-1509832691565766934_i64) << _1;
_3.0 = [82_i8,104_i8,44_i8,29_i8,98_i8,(-116_i8),78_i8];
_11.0 = _10.0;
_5 = _10.0;
_2 = [(-11163_i16),(-3101_i16),21449_i16];
match _1 {
0 => bb2,
9223372036854775807 => bb4,
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
_11 = _3;
_5 = _3.0;
Goto(bb5)
}
bb5 = {
RET = _11.1 as f64;
_9 = 199183613542961786003099767996438880775_u128;
_10.1.0 = 1392028398_u32 << _9;
_16.0 = (-31624_i16);
_14 = _1 - _1;
_3 = _11;
_10.0 = [44_i8,73_i8,(-66_i8),(-43_i8),(-6_i8),(-49_i8),71_i8];
_16.1 = [_16.0,_16.0,_16.0];
_3.1 = _11.1 * _11.1;
_11.0 = [(-65_i8),(-5_i8),20_i8,86_i8,(-98_i8),31_i8,41_i8];
_10.1 = (3319095487_u32,);
_11.0 = [(-96_i8),(-60_i8),(-101_i8),(-48_i8),75_i8,(-35_i8),(-20_i8)];
_10.1.0 = _9 as u32;
_11 = (_6, _3.1, _3.2);
_16 = (15140_i16, _2);
_6 = [123_i8,(-42_i8),20_i8,(-18_i8),(-110_i8),97_i8,(-82_i8)];
_22.2 = _7;
_16.0 = (-10421_i16);
_3.2 = _11.2;
_21 = -_14;
_11 = _3;
RET = 2057208464_i32 as f64;
match _9 {
0 => bb4,
1 => bb2,
2 => bb6,
199183613542961786003099767996438880775 => bb8,
_ => bb7
}
}
bb6 = {
_11 = _3;
_5 = _3.0;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
_5 = [101_i8,(-115_i8),100_i8,76_i8,(-8_i8),36_i8,(-94_i8)];
_9 = '\u{7ea4f}' as u128;
_24 = _16.0 <= _16.0;
_3.1 = -_11.1;
Goto(bb9)
}
bb9 = {
_10.0 = _3.0;
_13 = !_24;
_22.1 = _9 as f32;
_18.0 = (-13_i8) as u32;
_8 = core::ptr::addr_of!(_17);
_3.1 = _11.1;
_26 = _22.2;
_10.1.0 = _18.0 << _14;
_18.0 = _10.1.0;
_16.0 = !8615_i16;
_18.0 = _10.1.0 * _10.1.0;
_22.0 = [100_i8,(-58_i8),110_i8,85_i8,47_i8,(-123_i8),124_i8];
_9 = 217610352372760145826428418752002568172_u128 ^ 123306550798709065724446061966925058770_u128;
_23 = !(-1366406864_i32);
_17 = [_9,_9,_9,_9,_9,_9];
RET = 73_u8 as f64;
_11 = _3;
_25 = _14;
_23 = 2010269089_i32 & (-274358567_i32);
Call(_12 = core::intrinsics::transmute(_21), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = ((-7801_i16), _2);
_5 = [82_i8,(-114_i8),(-75_i8),30_i8,(-15_i8),94_i8,118_i8];
_29 = _16.0 as f64;
_24 = !_13;
_27 = [6726215340273175330_u64,12022270738868065969_u64,1900253630434791506_u64,10007823102684525944_u64,13536338018012478012_u64,7803257475897902242_u64,12447665265483586003_u64,17092909596034123658_u64];
_1 = _25 | _21;
_25 = _1;
_21 = _12 as isize;
_23 = 1137718590_i32;
_22.0 = _6;
Goto(bb11)
}
bb11 = {
_12 = _9 as i64;
_22.1 = 121752003535261512969510047197466067253_i128 as f32;
_18 = _10.1;
_35.1 = [3121978265652472991_u64,11735252066353390904_u64];
_8 = core::ptr::addr_of!((*_8));
_31 = 93_i8;
_3 = (_10.0, _11.1, _22.2);
Goto(bb12)
}
bb12 = {
_34 = [_9,_9,_9,_9,_9,_9];
_35.0 = _9 as isize;
_10 = (_11.0, _18);
_16.1 = [_16.0,_16.0,_16.0];
_10.1.0 = !_18.0;
_16 = (13250_i16, _2);
RET = -_29;
_27 = [3602474347250750927_u64,2654994919330123728_u64,16928992541821512473_u64,16691189385943211233_u64,6154079461645145864_u64,16003829068740421719_u64,9870094981405293520_u64,12738060713716123114_u64];
Call(_33 = core::intrinsics::bswap(_1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26 = _7;
_30 = _13;
_4 = [(-126821542128030095261586688559078337107_i128),130900911515468772620516658793364007085_i128,(-165297387155520779902798789060022434102_i128),120468269266659457293941309044604585679_i128,(-139305342779256879261094848543625054980_i128),(-62250178414948149355009569852878152512_i128),65860448742485401099497956649397780814_i128,149530247960512632543901268143108158826_i128];
_18 = (_10.1.0,);
_17 = [_9,_9,_9,_9,_9,_9];
_10.0 = [_31,_31,_31,_31,_31,_31,_31];
RET = _12 as f64;
_18 = (_10.1.0,);
_35.0 = _1 * _21;
_3.2 = [(-167448054122126194353128839181418883102_i128),75072293171340999585266966874444321385_i128,(-27763126419834235631255623871008277133_i128),(-82929470420347684614718419533717305144_i128),(-30263796508249686648263095848673235479_i128),(-120842709482417754161265934364763061220_i128),132530262454215972892886768990666453407_i128,96828027016029582480726874522388473534_i128];
RET = _29;
_3.1 = _31 as f32;
_39 = !_30;
Goto(bb14)
}
bb14 = {
_2 = [_16.0,_16.0,_16.0];
_43 = (_25, _35.1);
_10.1.0 = _35.0 as u32;
_44.2 = (_11.0, _11.1, _4);
_9 = _10.1.0 as u128;
_35.0 = _43.0;
_11.1 = _31 as f32;
_9 = 174435335117008830498935783127748163133_u128 ^ 92671483572061200946808689551253025777_u128;
_37 = _25;
_11.0 = _44.2.0;
_44.3.1 = (_10.1.0,);
_10.1 = _18;
_35.0 = -_43.0;
_17 = _34;
_36.fld0 = _8;
RET = -_29;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(4_usize, 35_usize, Move(_35), 16_usize, Move(_16), 43_usize, Move(_43), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(4_usize, 39_usize, Move(_39), 2_usize, Move(_2), 27_usize, Move(_27), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(4_usize, 26_usize, Move(_26), 31_usize, Move(_31), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(4_usize, 23_usize, Move(_23), 5_usize, Move(_5), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i8; 7],mut _4: [i8; 7],mut _5: [i128; 8],mut _6: [i16; 3],mut _7: f64,mut _8: [i8; 7],mut _9: [i128; 8]) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _10: isize;
let _11: char;
let _12: Adt52;
let _13: *mut i128;
let _14: [i8; 7];
let _15: bool;
let _16: bool;
let _17: bool;
let _18: ([i8; 7], f32, [i128; 8]);
let _19: ([i8; 7], f32, [i128; 8]);
let _20: ([i8; 7], (u32,));
let _21: bool;
let _22: f32;
let _23: (i16, usize, [u64; 2], i16);
let _24: Adt53;
let _25: (isize, [u64; 2]);
let _26: char;
let _27: Adt51;
let _28: usize;
let _29: bool;
let _30: (i16, [i16; 3]);
let _31: (i16, [i16; 3]);
let _32: usize;
let _33: [u64; 2];
let _34: i16;
let _35: f32;
let _36: ([i8; 7], f32, [i128; 8]);
let _37: isize;
let _38: [i16; 8];
let _39: Adt53;
let _40: i64;
let _41: u64;
let _42: [u64; 8];
let _43: [i16; 3];
let _44: f64;
let _45: &'static u8;
let _46: i16;
let _47: ();
let _48: ();
{
_9 = [(-168851746773062962959237273779692268723_i128),(-106796660876785500422665433560422042008_i128),(-163321815814008665040837641287117047441_i128),(-54529376870027164320308684348410031558_i128),37788666375666029513707413157105969063_i128,(-124081251558976148176895247991584557222_i128),113947160000933356357738325235041222251_i128,(-135137477902079822230944469682356634571_i128)];
_1 = [(-104533401805453129286794323457325455060_i128),96998911637489125847446636410779855746_i128,113989056949680753628674865924399583029_i128,69123238455091596258281371465260537930_i128,(-34731411909399263922421638392005466777_i128),116623766060161846314090870987028098601_i128,(-23855442289880105084212140884159377626_i128),(-7188261295483720679459917444043235204_i128)];
RET = [51985953891556933512128179372381869552_i128,99427424506921578292299472788371822226_i128,163913489292382855326919643056205293995_i128,(-4467225411694869323962296360527654635_i128),(-98522939207659299835248257872685103441_i128),(-117198561506394629679146880207889227311_i128),(-112927715223692634183026559707835654367_i128),(-1473270612388847669634133056707955567_i128)];
RET = _1;
_8 = [(-2_i8),(-47_i8),(-1_i8),72_i8,87_i8,(-103_i8),117_i8];
_10 = 233_u8 as isize;
Call(_9 = fn6(_8, _2, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = true as isize;
_2 = _5;
_10 = 9223372036854775807_isize + 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_7 = 103_i8 as f64;
_9 = RET;
RET = _9;
_10 = 9223372036854775807_isize << 15554516743814033064_usize;
_7 = 6282591127023171277_i64 as f64;
_4 = [124_i8,4_i8,(-107_i8),(-118_i8),(-119_i8),(-92_i8),(-20_i8)];
_3 = _4;
RET = [(-152971539193786787887440188165418991052_i128),(-99243046577294657139204234847462391067_i128),(-19837935707519301081918846195070188286_i128),11094386660234537326195579457152110918_i128,14479238634001651731260885746919613704_i128,76232339604956876243698222421147352450_i128,(-57304892250152621089455238376665007571_i128),(-130120342650617586597211184400396583845_i128)];
_7 = 192613835822508047258193540630023739346_u128 as f64;
_7 = 12767589547923706385009216629493077962_u128 as f64;
_3 = [(-48_i8),(-4_i8),(-18_i8),(-59_i8),(-65_i8),(-94_i8),(-78_i8)];
_9 = [75720641814025649184483787191638132243_i128,(-28591195389478744105029804858849516288_i128),155053738652845487477275642732482817573_i128,80042579020326852142632535590985956364_i128,59643847429272620654937296909584348795_i128,(-49153321952310376353701044149619494207_i128),26899328473670889352348192300137192005_i128,(-134797691701101860050144906749134122400_i128)];
_10 = (-111_isize);
_11 = '\u{f6ed5}';
_4 = _3;
_1 = [47785365913542146840309412115535355219_i128,53943671394133055653882799408044139358_i128,21129962465671890764108556128825416917_i128,(-46836767709109744746990865110876611223_i128),118087900378607989195756051029712806913_i128,(-25238905677641757295269024000041603543_i128),108661342563970421029627758618034755175_i128,(-138736238498760712744908091941856899799_i128)];
match _10 {
0 => bb1,
1 => bb3,
340282366920938463463374607431768211345 => bb5,
_ => bb4
}
}
bb3 = {
_10 = true as isize;
_2 = _5;
_10 = 9223372036854775807_isize + 9223372036854775807_isize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_1 = [106863549016307533487742998215561906867_i128,126776197555928329918107895153549782201_i128,(-60056653716791347985315795093655856077_i128),153501493374956746495305469241037877233_i128,19621092018970786063180691235879566504_i128,121019664179267371865672956521879769995_i128,(-13445577198018963402559072789699150946_i128),89496853858426409772389974908735975304_i128];
_8 = [72_i8,(-114_i8),98_i8,(-86_i8),121_i8,(-8_i8),(-92_i8)];
_8 = _3;
_8 = [86_i8,25_i8,(-107_i8),(-62_i8),(-125_i8),59_i8,(-100_i8)];
_2 = [(-79602482067505274229391736526043164217_i128),(-55015154147320652772814039147219658667_i128),(-6097021542812369887256714625756902485_i128),(-107727087919215530810278402129753817370_i128),125562878983334264079055289973050971483_i128,71855037806533419644291916808679572558_i128,80705220023964028011446583650273756327_i128,(-160043859688289867231845450755224982901_i128)];
_11 = '\u{581f7}';
_4 = [(-70_i8),(-117_i8),64_i8,0_i8,22_i8,14_i8,70_i8];
_11 = '\u{fb628}';
_16 = true ^ true;
_14 = [(-111_i8),(-125_i8),(-119_i8),(-94_i8),(-15_i8),22_i8,5_i8];
_17 = _16;
_11 = '\u{9aa5c}';
_11 = '\u{a31fb}';
_14 = [(-121_i8),(-19_i8),(-100_i8),66_i8,50_i8,(-9_i8),112_i8];
_14 = [34_i8,42_i8,38_i8,102_i8,(-30_i8),(-16_i8),97_i8];
_11 = '\u{250ac}';
_10 = !(-60_isize);
_15 = _17 > _17;
_18.1 = 90502297009236293431747162982392587669_u128 as f32;
_18.2 = [161361598751106540280259932085140596982_i128,44431734185189473833513758559996524566_i128,(-38361190680461311190425788904768440347_i128),(-36664323522744461041954879235576396271_i128),(-32303938708367178650504171266072947608_i128),(-58551374483593166434103551759551494347_i128),63076448511495206190153996376401252392_i128,113037332196204548144135717570373292752_i128];
Goto(bb6)
}
bb6 = {
_11 = '\u{b71b5}';
_1 = [117252509369441534783433951897128765807_i128,139831255069592831921542506567649802833_i128,(-20496664798109676844716972477043726644_i128),(-129077643952786275741431732016446973355_i128),77024007493050239337946626848682019969_i128,125716199530268902688257508592076902259_i128,149224941222657132188726286808608164417_i128,(-143188132680465900381566406072586863934_i128)];
_3 = [(-109_i8),54_i8,92_i8,(-30_i8),(-98_i8),55_i8,(-112_i8)];
_14 = _3;
_2 = [90809796896050565803986663979147272389_i128,139789311931449899916543497544144166641_i128,(-117557525553853794926565406005923373254_i128),(-93452832095740540888884925504991029122_i128),149784882777409659380755698739539369723_i128,(-90020962350893905313112649129963704883_i128),152941836132796071721935499507540963873_i128,96926832626997175216798246840726745509_i128];
_9 = _5;
_18.0 = [58_i8,73_i8,88_i8,18_i8,42_i8,59_i8,37_i8];
_19 = (_8, _18.1, _9);
_19 = (_4, _18.1, _5);
_11 = '\u{60688}';
_19.2 = [106823492369206050437189705492455331549_i128,(-11118868907001453321405209610130206188_i128),47546619556494376191757791931654459040_i128,162852299473456870900524939579308216957_i128,(-12477588880222992874339221213866301663_i128),21709960740879533480830418272457290164_i128,41779080106101301306356071711683697195_i128,42204667230543453354493559046853375123_i128];
_21 = _16 & _15;
_19 = (_14, _18.1, _2);
_22 = _19.1;
_19.2 = [33111496545182166918543498214802605478_i128,48860288196814455972411098314734540448_i128,(-84460317286730377739817093555137452814_i128),(-101456902323291463982038993955646006752_i128),(-39765535061567794455072368411956531497_i128),(-151930945011332264042500316243102877111_i128),(-55667307118207706372527068616404953870_i128),(-104962728355634785182118177356040144507_i128)];
RET = [(-33469151828338242926447918383861345188_i128),(-2455187413355703748784036605784458110_i128),146292406335123315778101904868039047074_i128,(-73760497120566791426975807953898095404_i128),(-59191390297016869132458472831932790311_i128),140659824284995915363300521586664583426_i128,168177716090603922875654069794669103624_i128,(-31513047246529461832580042436933434410_i128)];
Goto(bb7)
}
bb7 = {
_20.1 = (3553214868_u32,);
_9 = [80777783609182558557423274931375736759_i128,164324180862860999773784724579311917499_i128,35013943468661903061235833878775470664_i128,23974947329862831766793098599277014344_i128,118366048589030823867854569399905605989_i128,(-12196512751479479183132712524586209822_i128),(-78264137005601676915658163008278667228_i128),(-80332243813487232735471610779791536244_i128)];
_21 = _17;
_20.1 = (308350707_u32,);
_23.2 = [2055899555838186505_u64,4039559106227909395_u64];
_20.1.0 = !2213359052_u32;
_25.0 = _10;
_18.1 = -_22;
_20.0 = [5_i8,(-6_i8),(-38_i8),0_i8,(-121_i8),36_i8,(-72_i8)];
_2 = _5;
_27.fld2 = 2_usize * 15488022653093454164_usize;
_20.0 = [94_i8,61_i8,(-50_i8),(-63_i8),44_i8,(-7_i8),73_i8];
_22 = 56240_u16 as f32;
_18.0 = [(-33_i8),76_i8,60_i8,(-117_i8),93_i8,(-91_i8),62_i8];
_28 = !_27.fld2;
_23.3 = !29325_i16;
_28 = !_27.fld2;
_4 = _19.0;
_19.2 = [(-80670648982555778331381818076501969648_i128),77446890732481220539911172302738713388_i128,64118066459513954762789854171337821187_i128,(-57016663177350564759609820253541639016_i128),41790368788944106588858353789100437508_i128,(-108782392335535617798026383686529004881_i128),(-128825566411987772003089813033834803695_i128),118069674533319992381845809926933299599_i128];
_3 = [92_i8,(-42_i8),52_i8,(-12_i8),67_i8,(-24_i8),62_i8];
Goto(bb8)
}
bb8 = {
RET = _1;
_25.1 = [11125011488189234259_u64,9802517189899041315_u64];
_11 = '\u{475d1}';
_18.2 = [(-47596344005980943105474095494610297911_i128),98540136306682284509699562416829706739_i128,4884830675873538915679782872581589162_i128,138763658482932893895195964342949884816_i128,(-131641523430189418836812024290023491801_i128),38953731128101590055383835656316094925_i128,78845363009459477737943455982634152328_i128,(-76453471198102436932402662784819207166_i128)];
_2 = _19.2;
_20.1.0 = !3899035944_u32;
_23.1 = !_28;
_23.1 = _28 << _27.fld2;
_25 = (_10, _23.2);
_17 = !_21;
_23.0 = !_23.3;
_19.2 = [(-145640362955235325220968477983776108222_i128),(-105234795117312491665668918623732899908_i128),91687689706841261542798552578706946999_i128,(-23240867087020391494621691274755071738_i128),127089251127515739099937972025332358188_i128,(-43607795942393032883002351979052051478_i128),69306121148403412668016233148538270863_i128,(-158135730823928304620936153363081596487_i128)];
_23.0 = 11372704937755119597_u64 as i16;
_10 = _15 as isize;
_23.1 = _28 * _27.fld2;
_6 = [_23.0,_23.3,_23.3];
_20.1 = (4130006786_u32,);
_2 = RET;
_29 = _17;
RET = [124649407261124957866333251597294770611_i128,763610707301033157951713866757329220_i128,(-96171972832122944579277315094121384687_i128),(-37635637530308830806216969390899147157_i128),(-29079552919720991016277133686273462316_i128),(-8074283616557808494872085682444631959_i128),49760821699843130032935559042484307654_i128,(-84981836217138652099580858404232615042_i128)];
_3 = _8;
_19.0 = [(-43_i8),89_i8,(-35_i8),(-66_i8),116_i8,(-71_i8),(-38_i8)];
_30.1 = [_23.3,_23.0,_23.0];
_14 = [96_i8,55_i8,41_i8,106_i8,13_i8,(-75_i8),51_i8];
_9 = _19.2;
_7 = _10 as f64;
match _20.1.0 {
4130006786 => bb10,
_ => bb9
}
}
bb9 = {
_11 = '\u{b71b5}';
_1 = [117252509369441534783433951897128765807_i128,139831255069592831921542506567649802833_i128,(-20496664798109676844716972477043726644_i128),(-129077643952786275741431732016446973355_i128),77024007493050239337946626848682019969_i128,125716199530268902688257508592076902259_i128,149224941222657132188726286808608164417_i128,(-143188132680465900381566406072586863934_i128)];
_3 = [(-109_i8),54_i8,92_i8,(-30_i8),(-98_i8),55_i8,(-112_i8)];
_14 = _3;
_2 = [90809796896050565803986663979147272389_i128,139789311931449899916543497544144166641_i128,(-117557525553853794926565406005923373254_i128),(-93452832095740540888884925504991029122_i128),149784882777409659380755698739539369723_i128,(-90020962350893905313112649129963704883_i128),152941836132796071721935499507540963873_i128,96926832626997175216798246840726745509_i128];
_9 = _5;
_18.0 = [58_i8,73_i8,88_i8,18_i8,42_i8,59_i8,37_i8];
_19 = (_8, _18.1, _9);
_19 = (_4, _18.1, _5);
_11 = '\u{60688}';
_19.2 = [106823492369206050437189705492455331549_i128,(-11118868907001453321405209610130206188_i128),47546619556494376191757791931654459040_i128,162852299473456870900524939579308216957_i128,(-12477588880222992874339221213866301663_i128),21709960740879533480830418272457290164_i128,41779080106101301306356071711683697195_i128,42204667230543453354493559046853375123_i128];
_21 = _16 & _15;
_19 = (_14, _18.1, _2);
_22 = _19.1;
_19.2 = [33111496545182166918543498214802605478_i128,48860288196814455972411098314734540448_i128,(-84460317286730377739817093555137452814_i128),(-101456902323291463982038993955646006752_i128),(-39765535061567794455072368411956531497_i128),(-151930945011332264042500316243102877111_i128),(-55667307118207706372527068616404953870_i128),(-104962728355634785182118177356040144507_i128)];
RET = [(-33469151828338242926447918383861345188_i128),(-2455187413355703748784036605784458110_i128),146292406335123315778101904868039047074_i128,(-73760497120566791426975807953898095404_i128),(-59191390297016869132458472831932790311_i128),140659824284995915363300521586664583426_i128,168177716090603922875654069794669103624_i128,(-31513047246529461832580042436933434410_i128)];
Goto(bb7)
}
bb10 = {
_30.1 = _6;
_26 = _11;
_7 = 61_u8 as f64;
_6 = _30.1;
_19.0 = [33_i8,58_i8,86_i8,119_i8,(-6_i8),86_i8,79_i8];
_6 = [_23.0,_23.3,_23.3];
_20.0 = [39_i8,90_i8,(-11_i8),32_i8,80_i8,(-51_i8),(-39_i8)];
_20.1 = (1265962453_u32,);
RET = [(-152282614191875044853981331688437305405_i128),(-102839899738495321342437947837947994471_i128),(-34824817227266529301335174187039531923_i128),(-61714346730280274404241850475499557062_i128),(-69263903061201516089562540699343205458_i128),(-67474726525179003882488311654793815651_i128),15755535429802813047611661840433141598_i128,(-92479008577795113368267079695867734127_i128)];
_18.2 = [54970894398479490118038804356879523837_i128,66285721836339876332669779433463718020_i128,123306466454853095956458988562525737422_i128,124447057693478821485346204085606718397_i128,83006634889646845293353320434395307435_i128,(-61723323971089845271474132257016336414_i128),(-5444874005763386387865044432450695877_i128),154364953786262259693165385586934943616_i128];
_7 = 253_u8 as f64;
_21 = _17 ^ _29;
_25.1 = [11386920688750412837_u64,10624689408052587442_u64];
_20.1 = (738570176_u32,);
_31.0 = _26 as i16;
_11 = _26;
_22 = -_19.1;
_19 = (_8, _22, _1);
_2 = [(-59011193801636990748768801207595843967_i128),30374174606121920304957032804924032810_i128,(-67924358552890826859899990676316221030_i128),73141569496641982006017620342806250799_i128,134458934318882970274264214220933860643_i128,(-138945228608108217999240457036343027628_i128),38684124163911235921648473087726039428_i128,(-127713227670353388463717409244130401803_i128)];
RET = [(-132261766741203694456715092179223743433_i128),(-154327945593777251379626369180265911681_i128),(-148460060293866822022619690492718987256_i128),(-112152952478860436442931894871010048040_i128),(-88883404077351518689638105301541109742_i128),127631104234197512020245082163061445702_i128,154229649283295759534317265619662640125_i128,68398064151802347969287365878914237896_i128];
_36.1 = _10 as f32;
_16 = !_29;
Goto(bb11)
}
bb11 = {
_11 = _26;
_23.3 = _7 as i16;
_27.fld2 = !_23.1;
_19.2 = RET;
_10 = _25.0 >> _23.3;
_35 = _36.1;
_31.1 = [_31.0,_23.3,_31.0];
_34 = _36.1 as i16;
_4 = _18.0;
_23.2 = [17148043459085126812_u64,17423705275472563367_u64];
_19.0 = [11_i8,94_i8,18_i8,54_i8,(-71_i8),(-46_i8),24_i8];
_11 = _26;
_23.0 = _34;
_30 = _31;
RET = _19.2;
RET = _5;
_30.0 = _23.0;
match _20.1.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb10,
738570176 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_34 = _7 as i16;
_31 = _30;
_33 = [18138537954432464548_u64,7299924074390473116_u64];
_31.1 = [_31.0,_23.0,_23.0];
RET = [46753045347512432545622053183406853155_i128,(-66046501559664899910307913182126453963_i128),52563015532612481667772653846650154989_i128,(-53912556959855688582373566360314658618_i128),(-25526216260723784543112501686826193223_i128),83592793418864368736487559921115630303_i128,(-86415102458460148451755847635088804916_i128),(-106848652450585997161443797040690905187_i128)];
_31.1 = [_23.0,_23.0,_23.0];
_18.2 = _19.2;
_29 = !_21;
_5 = [76815823609297844688898628385053112663_i128,28559033109476204277911106308308392963_i128,(-145162322430615430209391939598465939307_i128),77435606893239989256759322329450203248_i128,137761171449299939157474113240634717432_i128,141257629956159452935331104288525164551_i128,29060216099537776298861848654225399823_i128,(-38168701653097821311397394164695176773_i128)];
_18 = (_3, _35, _9);
_36 = (_3, _18.1, _19.2);
_25.1 = [291610206786505200_u64,320919900211329330_u64];
_16 = !_17;
_18.1 = -_35;
Goto(bb14)
}
bb14 = {
_41 = 6238220004469199833_u64;
_4 = _20.0;
_5 = [(-97038724907606925711322205931633090639_i128),(-163245039071665946774552946741956519535_i128),(-21242398061505122887199040248565600632_i128),(-127720494834062883286586973811871582375_i128),64958719784687557377303595115997291467_i128,(-15486741409187148673400099306905573717_i128),11893432911451587500622437176483660774_i128,150615349498661611036348962036427611983_i128];
_10 = _25.0;
_25.1 = [_41,_41];
_10 = !_25.0;
_23.3 = _30.0 * _31.0;
_30.0 = _27.fld2 as i16;
_27.fld3 = Adt45::Variant2 { fld0: _41,fld1: _20,fld2: (-1552974589_i32) };
_38 = [_23.3,_23.3,_23.3,_30.0,_23.3,_23.3,_23.3,_23.0];
_43 = _30.1;
place!(Field::<u64>(Variant(_27.fld3, 2), 0)) = _41;
place!(Field::<i32>(Variant(_27.fld3, 2), 2)) = (-800282306_i32) * 1281223251_i32;
_18 = (_3, _19.1, _19.2);
_25.0 = -_10;
_18 = (_3, _35, RET);
_35 = _18.1;
_20.0 = [70_i8,(-87_i8),116_i8,(-47_i8),2_i8,(-27_i8),(-53_i8)];
_19 = (_20.0, _36.1, _2);
_23.0 = _23.3 >> _23.3;
_37 = (-138141508270083667_i64) as isize;
_35 = _18.1;
_30.1 = _31.1;
_25.1 = [Field::<u64>(Variant(_27.fld3, 2), 0),Field::<u64>(Variant(_27.fld3, 2), 0)];
_23 = (_31.0, _28, _33, _30.0);
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(5_usize, 41_usize, Move(_41), 20_usize, Move(_20), 1_usize, Move(_1), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(5_usize, 26_usize, Move(_26), 11_usize, Move(_11), 43_usize, Move(_43), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(5_usize, 10_usize, Move(_10), 5_usize, Move(_5), 29_usize, Move(_29), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(5_usize, 6_usize, Move(_6), 9_usize, Move(_9), 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i8; 7],mut _2: [i128; 8],mut _3: [i8; 7]) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _4: [u64; 2];
let _5: Adt42;
let _6: i64;
let _7: u8;
let _8: char;
let _9: isize;
let _10: u8;
let _11: ([i8; 7], f32, [i128; 8]);
let _12: (i16, [i16; 3]);
let _13: Adt50;
let _14: Adt51;
let _15: i8;
let _16: isize;
let _17: [i128; 8];
let _18: (i16, [i16; 3]);
let _19: [i128; 8];
let _20: i8;
let _21: Adt39;
let _22: ([u128; 6], &'static u8);
let _23: f64;
let _24: [u64; 2];
let _25: [u64; 8];
let _26: ([i8; 7], f32, [i128; 8]);
let _27: [i16; 3];
let _28: (isize, [u64; 2]);
let _29: &'static u8;
let _30: [u64; 8];
let _31: [i16; 8];
let _32: char;
let _33: isize;
let _34: isize;
let _35: [i16; 8];
let _36: ((i16, [i16; 3]), &'static u8, ([i8; 7], f32, [i128; 8]), ([i8; 7], (u32,)));
let _37: [i16; 8];
let _38: u8;
let _39: f32;
let _40: [u64; 2];
let _41: ();
let _42: ();
{
RET = [50505717477137647530328123607293422476_i128,46667907145830931400929936945576539072_i128,(-2765809014150981115858459904283200770_i128),47231695384060628915347918750960341060_i128,(-160296110254074570289162468545491823047_i128),(-167221940196309571117501460840962876639_i128),(-163547543902797379141165170652175973413_i128),(-89639175567941221498405134370982255703_i128)];
RET = _2;
_4 = [12198278799086862171_u64,16871999522375089375_u64];
_3 = [(-6_i8),100_i8,13_i8,43_i8,123_i8,(-79_i8),(-71_i8)];
RET = [14445003227112190551728714372896694579_i128,960834281365400864651718261897940869_i128,138422063812671045057257139502987814800_i128,25004377272960938190675771603705975510_i128,52069892854566769466564588395707303695_i128,(-131387398238331947119387593644737701667_i128),(-115195054515866748765732526394251403243_i128),(-74654256380339660798466732540669274420_i128)];
_3 = _1;
Call(_4 = fn7(RET, _3, RET, _3, _2, _1, _2, _1, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [5919451924737562054_u64,3887436140899052130_u64];
_1 = [21_i8,(-23_i8),26_i8,(-15_i8),(-20_i8),101_i8,65_i8];
_2 = [139979576752933941082515032669184770051_i128,45280383919670984627734736410124668703_i128,(-143449078570438405403223431149268278804_i128),(-121607131922065708072761377792179514356_i128),155350711113369263533731295844962183768_i128,(-165655300621393730111887259766772678644_i128),(-7735701242452298371885545191905324070_i128),95196087573479945308641839762266430821_i128];
RET = [111963877701172102546869069819963138200_i128,(-161901290584636222714215960319177229859_i128),(-72638525696550649564993457608119382379_i128),16804559993856826731088630445771694929_i128,138982051663061171872021485778656125302_i128,70352433894175514496838645181164131984_i128,(-103931175725667391572769083373490973054_i128),18243201493732452141453045980080748518_i128];
_3 = _1;
_4 = [6902254536773072054_u64,2838615659136509110_u64];
_3 = [(-95_i8),64_i8,(-31_i8),(-39_i8),30_i8,(-18_i8),(-26_i8)];
_4 = [6125995935775837186_u64,3436098678021656475_u64];
_6 = (-8226436746367743064_i64) & (-3553954456410352783_i64);
_7 = 7_u8;
Call(_7 = fn8(_1, _1, _1, _3, _4, _6, _6, _1, _3, _2, _3, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = '\u{12adc}';
_9 = (-9223372036854775808_isize);
_6 = 2557937507075926593_usize as i64;
_3 = [86_i8,(-6_i8),(-38_i8),(-100_i8),27_i8,34_i8,(-128_i8)];
_9 = (-96_isize) ^ 9223372036854775807_isize;
_3 = [85_i8,113_i8,(-60_i8),42_i8,(-15_i8),(-28_i8),(-16_i8)];
_3 = [103_i8,60_i8,(-122_i8),(-118_i8),(-126_i8),(-32_i8),(-115_i8)];
_11.1 = (-35_i8) as f32;
_2 = RET;
_4 = [15276823207981500420_u64,16096098612410155822_u64];
_10 = _7;
_6 = !(-5393322961147432103_i64);
_10 = false as u8;
_11.2 = RET;
RET = [(-88249039116687376647948366215430438958_i128),(-154180860033925071963827995244920191610_i128),(-20492759130046750035828894090045333606_i128),(-52920310362667978796369734393588630696_i128),(-125325817895933400636097464558037066581_i128),103946159965887502102965334649896843407_i128,97309271508503709748656674846015189752_i128,115579598570912299983500105276558086575_i128];
_9 = _8 as isize;
_12.1 = [6946_i16,(-2239_i16),5056_i16];
_11.0 = _1;
_7 = _10;
_11.0 = [77_i8,114_i8,(-43_i8),70_i8,(-111_i8),(-53_i8),(-51_i8)];
Goto(bb3)
}
bb3 = {
_13.fld5.0 = 3199924769_u32;
_13.fld0 = _2;
_13.fld7 = _9 as f64;
_13.fld2 = (_3, _13.fld5);
_11.1 = _6 as f32;
_11.2 = _13.fld0;
_2 = [(-131539496952331541553083075069199668824_i128),125304796011291751677709063073307880511_i128,84661075924210512275548839294135612891_i128,(-71498526083554643122573375810317335435_i128),107843415497001553189391594807348192684_i128,(-148235415672070911510151495728633830601_i128),67736603158677777159721786884364222893_i128,140812008719258660403731845627594528510_i128];
_10 = (-78122736872673757647913552753346954088_i128) as u8;
_6 = !(-5282389400228525609_i64);
_6 = !8488902371121171522_i64;
_14.fld2 = !4761286062108054904_usize;
_10 = _7 + _7;
_9 = (-9223372036854775808_isize) << _13.fld5.0;
_11.1 = 31065_i16 as f32;
_13.fld0 = [(-114821949031174158951046807217846661744_i128),169745198422219725860651779829476044627_i128,6407181955130480707064218999203575634_i128,68703306554729367169457391984618817200_i128,(-123009434689240329771857065255861458045_i128),1107671734812411740882007580481839445_i128,89136581737162846060392867466093703382_i128,(-119678235420118777231087871293119682874_i128)];
_11.2 = [(-153644566200366450687108562661327588525_i128),(-98551911754852631089553457418599681528_i128),(-95056320213927609126793107527070170962_i128),(-78678851992386164924339484143715861346_i128),142729612897137224386825384881615665569_i128,(-148283980363224303443653004076023796075_i128),10036133194376927582612998668215568396_i128,91837519385839708783906516080553426336_i128];
_9 = (-9223372036854775808_isize);
_12.0 = 8023_i16;
_13.fld0 = [(-52162290175642896742879937445407773532_i128),(-25853271071920979439899286646672696830_i128),125254663821509508235456806785085839416_i128,61348248053228920431297622961984201164_i128,62347060996604049343598511959061880351_i128,(-33078251896281983438885216244095503031_i128),122949161671171598099300222470317778585_i128,141986699959346975214795005415137027714_i128];
_16 = _9 + _9;
_6 = 8881484564190844460_i64;
_13.fld0 = [9073939514031475326000361652232624032_i128,(-32973051740582725144589600243280111181_i128),77284975223481603774958142414047213314_i128,(-93233457786718980956120610178202901567_i128),167924667589803017238837681888873774377_i128,110014780218457476542958072242281593465_i128,87163687063455878630898973758850748924_i128,(-56273678122340866027261578848026120830_i128)];
_8 = '\u{c6278}';
_14.fld3 = Adt45::Variant2 { fld0: 6446992883016821961_u64,fld1: _13.fld2,fld2: 2036180352_i32 };
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)).1.0 = _13.fld2.1.0 | _13.fld2.1.0;
Goto(bb4)
}
bb4 = {
_17 = [60375962586935842013230289305413412325_i128,40000909670832407107323667019730389986_i128,(-26159868984762551128360003344661065118_i128),(-140220405888423373796498539436281659303_i128),31541808848189277710010223355319876357_i128,(-71489345188874371372321732915497864661_i128),11858011674675748209717838027000888181_i128,(-85712728810030424040139199345781288184_i128)];
_11.1 = _12.0 as f32;
_11.1 = _6 as f32;
RET = [(-124239541978546714715206986620523506025_i128),159561062651520677372261501786856719687_i128,(-129249122269977013897258470439575398992_i128),114816857418250941407196585329163132865_i128,77472265045115979246781777211246488032_i128,(-72913672722979456996427708622109485675_i128),91145388794966210268283027687710140713_i128,149632642753095216036493650736042057101_i128];
_13.fld6 = !3004194071792318356_u64;
_11.2 = [91751547627226483226270773368460186675_i128,(-124996502840033146437276338683927751395_i128),19567149899635785846413136011673058537_i128,(-75719644999367655647814299749254269430_i128),(-87210346384952146224176621599523150346_i128),85666758033789964414238048586790016293_i128,95372146083260548660521715587049369654_i128,(-107794053314316618058186624611849915993_i128)];
_11.0 = [56_i8,(-21_i8),74_i8,(-122_i8),(-8_i8),75_i8,3_i8];
_13.fld6 = 15131162020369011758_u64 | 3005912165419856861_u64;
_11.2 = _2;
_9 = !_16;
match _6 {
0 => bb2,
8881484564190844460 => bb6,
_ => bb5
}
}
bb5 = {
_8 = '\u{12adc}';
_9 = (-9223372036854775808_isize);
_6 = 2557937507075926593_usize as i64;
_3 = [86_i8,(-6_i8),(-38_i8),(-100_i8),27_i8,34_i8,(-128_i8)];
_9 = (-96_isize) ^ 9223372036854775807_isize;
_3 = [85_i8,113_i8,(-60_i8),42_i8,(-15_i8),(-28_i8),(-16_i8)];
_3 = [103_i8,60_i8,(-122_i8),(-118_i8),(-126_i8),(-32_i8),(-115_i8)];
_11.1 = (-35_i8) as f32;
_2 = RET;
_4 = [15276823207981500420_u64,16096098612410155822_u64];
_10 = _7;
_6 = !(-5393322961147432103_i64);
_10 = false as u8;
_11.2 = RET;
RET = [(-88249039116687376647948366215430438958_i128),(-154180860033925071963827995244920191610_i128),(-20492759130046750035828894090045333606_i128),(-52920310362667978796369734393588630696_i128),(-125325817895933400636097464558037066581_i128),103946159965887502102965334649896843407_i128,97309271508503709748656674846015189752_i128,115579598570912299983500105276558086575_i128];
_9 = _8 as isize;
_12.1 = [6946_i16,(-2239_i16),5056_i16];
_11.0 = _1;
_7 = _10;
_11.0 = [77_i8,114_i8,(-43_i8),70_i8,(-111_i8),(-53_i8),(-51_i8)];
Goto(bb3)
}
bb6 = {
_22.1 = &_7;
place!(Field::<u64>(Variant(_14.fld3, 2), 0)) = _13.fld6;
_11.0 = [(-13_i8),20_i8,22_i8,72_i8,73_i8,(-7_i8),(-88_i8)];
_24 = [Field::<u64>(Variant(_14.fld3, 2), 0),Field::<u64>(Variant(_14.fld3, 2), 0)];
_13.fld7 = _12.0 as f64;
_24 = [Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6];
RET = [(-7211391057868240603452269947889008846_i128),112015875759508357975796055187606854344_i128,160511888660300216983628040222735036908_i128,94486308056050448353054416313050401346_i128,(-34120199971295443645398787372754147261_i128),35413926106550662247433724264807957992_i128,(-107468117621191316317772330678281200472_i128),80644794685938407177194823356540076506_i128];
_25 = [_13.fld6,_13.fld6,_13.fld6,_13.fld6,_13.fld6,_13.fld6,Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6];
_13.fld2 = (_1, _13.fld5);
place!(Field::<i32>(Variant(_14.fld3, 2), 2)) = _6 as i32;
_22.1 = &_10;
_22.0 = [17784238529038948145537680879162119321_u128,337277646162528761532687781976548299831_u128,303081858017703066081976969257337486328_u128,220308528142261164708187093135960999781_u128,201308787759028956664249551969626504265_u128,34589982143044214770742871869585775040_u128];
Goto(bb7)
}
bb7 = {
_22.1 = &_7;
_11.2 = [(-48814643587502307904800545948076840157_i128),(-137229379527046391957028760577333171746_i128),(-166126333898476260120337648009011524275_i128),137418153122292268884408959006462727103_i128,(-115728765457937704137575539101763219208_i128),(-20986153474183920833489950947380678864_i128),(-31911542568511428077547575792571435279_i128),(-106688780130321525617209623425321090802_i128)];
_27 = [_12.0,_12.0,_12.0];
_22.1 = &_10;
_18 = (_12.0, _12.1);
_28.1 = _24;
_14.fld0 = core::ptr::addr_of!(_22.0);
_13.fld5.0 = _13.fld2.1.0;
_18.1 = _12.1;
_30 = [Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6,_13.fld6,Field::<u64>(Variant(_14.fld3, 2), 0),Field::<u64>(Variant(_14.fld3, 2), 0),Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6,_13.fld6];
_2 = [(-59502713198655568768603807522953996877_i128),146718175103493611241738783374788971150_i128,35201556590113181160262690077733833759_i128,(-5059038969726961041687879073961316516_i128),(-110053247462484733614470374825513520745_i128),(-157399650455345340446270433134797716456_i128),98779886475525474234400519030033798396_i128,102394367454000600337227204354868421443_i128];
_13.fld2.0 = _1;
_13.fld2 = (_11.0, _13.fld5);
_15 = -(-70_i8);
_10 = _7;
_3 = _13.fld2.0;
_26.2 = [(-168350937615396768101492672634861713879_i128),162070818648277694070592666610302139593_i128,117958235063229605054195239166539554624_i128,141426620450725163484581637389765950719_i128,(-45472062190380700442529508172292379006_i128),(-45481288498422667674862248666016363163_i128),(-110729386483451734835456040957609847083_i128),56889089688442506089618912755713038023_i128];
_27 = [_12.0,_12.0,_18.0];
match _18.0 {
0 => bb8,
1 => bb9,
2 => bb10,
8023 => bb12,
_ => bb11
}
}
bb8 = {
_22.1 = &_7;
place!(Field::<u64>(Variant(_14.fld3, 2), 0)) = _13.fld6;
_11.0 = [(-13_i8),20_i8,22_i8,72_i8,73_i8,(-7_i8),(-88_i8)];
_24 = [Field::<u64>(Variant(_14.fld3, 2), 0),Field::<u64>(Variant(_14.fld3, 2), 0)];
_13.fld7 = _12.0 as f64;
_24 = [Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6];
RET = [(-7211391057868240603452269947889008846_i128),112015875759508357975796055187606854344_i128,160511888660300216983628040222735036908_i128,94486308056050448353054416313050401346_i128,(-34120199971295443645398787372754147261_i128),35413926106550662247433724264807957992_i128,(-107468117621191316317772330678281200472_i128),80644794685938407177194823356540076506_i128];
_25 = [_13.fld6,_13.fld6,_13.fld6,_13.fld6,_13.fld6,_13.fld6,Field::<u64>(Variant(_14.fld3, 2), 0),_13.fld6];
_13.fld2 = (_1, _13.fld5);
place!(Field::<i32>(Variant(_14.fld3, 2), 2)) = _6 as i32;
_22.1 = &_10;
_22.0 = [17784238529038948145537680879162119321_u128,337277646162528761532687781976548299831_u128,303081858017703066081976969257337486328_u128,220308528142261164708187093135960999781_u128,201308787759028956664249551969626504265_u128,34589982143044214770742871869585775040_u128];
Goto(bb7)
}
bb9 = {
_4 = [5919451924737562054_u64,3887436140899052130_u64];
_1 = [21_i8,(-23_i8),26_i8,(-15_i8),(-20_i8),101_i8,65_i8];
_2 = [139979576752933941082515032669184770051_i128,45280383919670984627734736410124668703_i128,(-143449078570438405403223431149268278804_i128),(-121607131922065708072761377792179514356_i128),155350711113369263533731295844962183768_i128,(-165655300621393730111887259766772678644_i128),(-7735701242452298371885545191905324070_i128),95196087573479945308641839762266430821_i128];
RET = [111963877701172102546869069819963138200_i128,(-161901290584636222714215960319177229859_i128),(-72638525696550649564993457608119382379_i128),16804559993856826731088630445771694929_i128,138982051663061171872021485778656125302_i128,70352433894175514496838645181164131984_i128,(-103931175725667391572769083373490973054_i128),18243201493732452141453045980080748518_i128];
_3 = _1;
_4 = [6902254536773072054_u64,2838615659136509110_u64];
_3 = [(-95_i8),64_i8,(-31_i8),(-39_i8),30_i8,(-18_i8),(-26_i8)];
_4 = [6125995935775837186_u64,3436098678021656475_u64];
_6 = (-8226436746367743064_i64) & (-3553954456410352783_i64);
_7 = 7_u8;
Call(_7 = fn8(_1, _1, _1, _3, _4, _6, _6, _1, _3, _2, _3, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_17 = [60375962586935842013230289305413412325_i128,40000909670832407107323667019730389986_i128,(-26159868984762551128360003344661065118_i128),(-140220405888423373796498539436281659303_i128),31541808848189277710010223355319876357_i128,(-71489345188874371372321732915497864661_i128),11858011674675748209717838027000888181_i128,(-85712728810030424040139199345781288184_i128)];
_11.1 = _12.0 as f32;
_11.1 = _6 as f32;
RET = [(-124239541978546714715206986620523506025_i128),159561062651520677372261501786856719687_i128,(-129249122269977013897258470439575398992_i128),114816857418250941407196585329163132865_i128,77472265045115979246781777211246488032_i128,(-72913672722979456996427708622109485675_i128),91145388794966210268283027687710140713_i128,149632642753095216036493650736042057101_i128];
_13.fld6 = !3004194071792318356_u64;
_11.2 = [91751547627226483226270773368460186675_i128,(-124996502840033146437276338683927751395_i128),19567149899635785846413136011673058537_i128,(-75719644999367655647814299749254269430_i128),(-87210346384952146224176621599523150346_i128),85666758033789964414238048586790016293_i128,95372146083260548660521715587049369654_i128,(-107794053314316618058186624611849915993_i128)];
_11.0 = [56_i8,(-21_i8),74_i8,(-122_i8),(-8_i8),75_i8,3_i8];
_13.fld6 = 15131162020369011758_u64 | 3005912165419856861_u64;
_11.2 = _2;
_9 = !_16;
match _6 {
0 => bb2,
8881484564190844460 => bb6,
_ => bb5
}
}
bb11 = {
_13.fld5.0 = 3199924769_u32;
_13.fld0 = _2;
_13.fld7 = _9 as f64;
_13.fld2 = (_3, _13.fld5);
_11.1 = _6 as f32;
_11.2 = _13.fld0;
_2 = [(-131539496952331541553083075069199668824_i128),125304796011291751677709063073307880511_i128,84661075924210512275548839294135612891_i128,(-71498526083554643122573375810317335435_i128),107843415497001553189391594807348192684_i128,(-148235415672070911510151495728633830601_i128),67736603158677777159721786884364222893_i128,140812008719258660403731845627594528510_i128];
_10 = (-78122736872673757647913552753346954088_i128) as u8;
_6 = !(-5282389400228525609_i64);
_6 = !8488902371121171522_i64;
_14.fld2 = !4761286062108054904_usize;
_10 = _7 + _7;
_9 = (-9223372036854775808_isize) << _13.fld5.0;
_11.1 = 31065_i16 as f32;
_13.fld0 = [(-114821949031174158951046807217846661744_i128),169745198422219725860651779829476044627_i128,6407181955130480707064218999203575634_i128,68703306554729367169457391984618817200_i128,(-123009434689240329771857065255861458045_i128),1107671734812411740882007580481839445_i128,89136581737162846060392867466093703382_i128,(-119678235420118777231087871293119682874_i128)];
_11.2 = [(-153644566200366450687108562661327588525_i128),(-98551911754852631089553457418599681528_i128),(-95056320213927609126793107527070170962_i128),(-78678851992386164924339484143715861346_i128),142729612897137224386825384881615665569_i128,(-148283980363224303443653004076023796075_i128),10036133194376927582612998668215568396_i128,91837519385839708783906516080553426336_i128];
_9 = (-9223372036854775808_isize);
_12.0 = 8023_i16;
_13.fld0 = [(-52162290175642896742879937445407773532_i128),(-25853271071920979439899286646672696830_i128),125254663821509508235456806785085839416_i128,61348248053228920431297622961984201164_i128,62347060996604049343598511959061880351_i128,(-33078251896281983438885216244095503031_i128),122949161671171598099300222470317778585_i128,141986699959346975214795005415137027714_i128];
_16 = _9 + _9;
_6 = 8881484564190844460_i64;
_13.fld0 = [9073939514031475326000361652232624032_i128,(-32973051740582725144589600243280111181_i128),77284975223481603774958142414047213314_i128,(-93233457786718980956120610178202901567_i128),167924667589803017238837681888873774377_i128,110014780218457476542958072242281593465_i128,87163687063455878630898973758850748924_i128,(-56273678122340866027261578848026120830_i128)];
_8 = '\u{c6278}';
_14.fld3 = Adt45::Variant2 { fld0: 6446992883016821961_u64,fld1: _13.fld2,fld2: 2036180352_i32 };
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)).1.0 = _13.fld2.1.0 | _13.fld2.1.0;
Goto(bb4)
}
bb12 = {
_14.fld0 = core::ptr::addr_of!(_22.0);
_13.fld2.1 = _13.fld5;
_12.0 = !_18.0;
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)).1.0 = !_13.fld5.0;
SetDiscriminant(_14.fld3, 2);
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)) = (_3, _13.fld2.1);
_2 = [14465711893978526360912822003058196266_i128,110997884230006944052212840108877446384_i128,78953138598334049570737405363726399897_i128,(-130147262029523879327958823970608305838_i128),(-20876728335578432015578962062953370806_i128),133755572316084881020253806917076414555_i128,107840306663706099547340949370613396896_i128,(-99370220690085163650764730627786203619_i128)];
Goto(bb13)
}
bb13 = {
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)).1.0 = !_13.fld5.0;
_11.0 = _1;
_13.fld5 = (Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1).1.0,);
_20 = _15;
_10 = _11.1 as u8;
_24 = _4;
_18.0 = 61365870_i32 as i16;
_18 = (_12.0, _12.1);
Goto(bb14)
}
bb14 = {
_24 = [_13.fld6,_13.fld6];
_29 = &_10;
_13.fld6 = 10057696756702443483_u64 * 10205186304066787152_u64;
RET = [23910288998832712866778174766281932233_i128,(-68815791605790798499532720815686030316_i128),(-31369043912983690185281212127174414156_i128),(-148727399031475192091374089916222164464_i128),108373724036835277773700467202535193534_i128,(-30985785864168181356693259605265796249_i128),(-115666525034088049347579948633232667840_i128),(-164883884048809723740147110884685240054_i128)];
_26.2 = _2;
_28 = (_16, _4);
_34 = !_16;
_25 = _30;
_11.2 = [32513956569654621582123998106374604396_i128,122855400028713698270209173011498084422_i128,25009516344193983374016481864059668761_i128,(-108932673550081075922549802240158243608_i128),(-11994370519296658663512009793347128134_i128),92629878471024824613790111176815222840_i128,63653641030390996139060443481433208522_i128,15042095758362501801184345921751144018_i128];
place!(Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1)) = (_1, _13.fld2.1);
_4 = _28.1;
_26 = _11;
_12.1 = [_18.0,_18.0,_12.0];
_36.0.0 = -_12.0;
_1 = [_15,_20,_20,_15,_15,_20,_15];
_36.2.2 = RET;
_26.2 = [(-79300736161067383954624979757224202711_i128),17948284148304525533981140475155136293_i128,(-69611536092439670473179438785945596126_i128),86785661032581123125270991327217911576_i128,140106467238611323724521579347440218720_i128,99880458490518256115437871819927084665_i128,133374835580737080403562752118156331507_i128,15476471893932829052708811615869536580_i128];
place!(Field::<u64>(Variant(_14.fld3, 2), 0)) = (*_29) as u64;
_36.2.2 = _2;
_11.2 = [29166930250916069264030812468407000814_i128,63721223193435879431684769154913193227_i128,(-85178562760900008565267242892087974356_i128),(-800061597167129614332111182709388822_i128),111726787377754694762186890403324979572_i128,15549271785329626555673852172134780650_i128,113855609628934022723122446706292354425_i128,77801569679921226475268713781102400851_i128];
_11.2 = [(-18105011319026185621676908035944283443_i128),5007883468069418761563942726142368370_i128,(-131424377932142433497418793182459538959_i128),(-32523973443669069126073644116098877528_i128),127560681696755165052000090330649622571_i128,55549452680631834967974952979853237320_i128,(-65737674099271465894004545216149771735_i128),58907986595696691813900791222945613468_i128];
_20 = _15;
_20 = _15 + _15;
_36.3 = (_13.fld2.0, Field::<([i8; 7], (u32,))>(Variant(_14.fld3, 2), 1).1);
_36.3.1.0 = _28.0 as u32;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(6_usize, 28_usize, Move(_28), 20_usize, Move(_20), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(6_usize, 15_usize, Move(_15), 17_usize, Move(_17), 27_usize, Move(_27), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(6_usize, 9_usize, Move(_9), 10_usize, Move(_10), 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i128; 8],mut _2: [i8; 7],mut _3: [i128; 8],mut _4: [i8; 7],mut _5: [i128; 8],mut _6: [i8; 7],mut _7: [i128; 8],mut _8: [i8; 7],mut _9: [i8; 7],mut _10: [i8; 7]) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _11: Adt41;
let _12: i32;
let _13: Adt43;
let _14: ([i8; 7], f32, [i128; 8]);
let _15: i8;
let _16: isize;
let _17: Adt44;
let _18: f64;
let _19: u32;
let _20: (i16, [i16; 3]);
let _21: Adt39;
let _22: ([u128; 6], &'static u8);
let _23: bool;
let _24: Adt40;
let _25: [i8; 7];
let _26: [i16; 3];
let _27: &'static u8;
let _28: ();
let _29: ();
{
_7 = _5;
_6 = [(-49_i8),(-19_i8),46_i8,84_i8,(-15_i8),73_i8,64_i8];
_4 = [(-48_i8),(-48_i8),3_i8,32_i8,34_i8,(-21_i8),1_i8];
_3 = _7;
_10 = _9;
_7 = [(-69996167476200681127089864571927303976_i128),(-76681376701385274293348385607626824102_i128),(-103308269225298319816839834663186170563_i128),(-47057730708630639128049477064527241127_i128),16263469539863550952967224670866341794_i128,(-79597740956619044504292439203733420780_i128),(-109290365106036200603872902642976211256_i128),31034837887978128777283204413958298455_i128];
_9 = [111_i8,(-85_i8),(-4_i8),74_i8,99_i8,(-18_i8),82_i8];
RET = [4611960487715884151_u64,18261292653609185171_u64];
_7 = [119252419678579396337490611579296376915_i128,104182719696469465686769055192672476467_i128,(-55122765580948691823885966922212088683_i128),(-59540099951043185537324073815446430741_i128),99078024078669566730694283597241408810_i128,8617948620644941049073589585671666019_i128,(-37006759736685366183681130003366123845_i128),(-137176186331582484171997674368357027834_i128)];
_5 = [(-147244017301170344024615612872713803355_i128),(-51281969833049654670117024713942560987_i128),(-169975096904507706311044383375948718150_i128),23030542850817937063984113716380696496_i128,(-106018073388854326083129930123901161328_i128),52062836471497764457915180408304114205_i128,(-145274160322591097460719260963497829781_i128),(-106538541175600830774748925941870903741_i128)];
_10 = [42_i8,(-62_i8),63_i8,(-27_i8),10_i8,88_i8,(-31_i8)];
RET = [1030244142502426185_u64,11800925422906324645_u64];
_9 = [106_i8,32_i8,(-71_i8),(-53_i8),(-17_i8),119_i8,(-42_i8)];
RET = [18054061506819202448_u64,6271691124647153214_u64];
Goto(bb1)
}
bb1 = {
_8 = _9;
RET = [3305446213218086382_u64,9948715396973292406_u64];
_3 = [39833674593822661427508586815287217738_i128,116542519412494264167600918106664063201_i128,168969446451097208326943031178972934014_i128,(-91119929364463902119713569759442277180_i128),(-133108442171099093609536284098571985452_i128),(-145234480667413820050179854454299100336_i128),(-7955338429376802906980709390544993675_i128),53198776437776844945502215757008408237_i128];
_8 = [15_i8,(-95_i8),(-53_i8),(-71_i8),(-64_i8),101_i8,5_i8];
_2 = [(-55_i8),(-79_i8),2_i8,87_i8,44_i8,111_i8,104_i8];
_2 = [101_i8,(-43_i8),20_i8,(-73_i8),31_i8,72_i8,(-8_i8)];
_12 = (-834945637_i32);
_1 = [(-109532594565661520179075251748953456470_i128),41805518001479869146929988921361600809_i128,3832367649668362517649674442375083069_i128,36784086720690657282808029369505470443_i128,(-68404886537302796982326271865851285140_i128),136867249437271326231548102872268249073_i128,71460559880771810442777518938513776545_i128,7267901615411893049786277416941288453_i128];
_2 = [(-31_i8),(-54_i8),(-21_i8),114_i8,98_i8,90_i8,(-99_i8)];
_5 = [(-143848141095152122395469886137957305743_i128),168521050270925300508242721316953384518_i128,6664835640149341926143874079533936128_i128,(-148768382531662005112642615043262206740_i128),80912445151945526623360769769643661586_i128,(-50854446935503269706334192781788807150_i128),(-140718753956930722522503426741936961512_i128),22826275540648711587078746530091363443_i128];
_4 = _8;
_7 = [(-166901988198960782487479241342056114632_i128),(-78240803459191651451149751836398104131_i128),(-70507971075201250674277001840333126280_i128),41662400950241679684597327262378280185_i128,20447798959296557749019953159601454532_i128,(-164300068001268483501465852198784829141_i128),(-17510956203426773278185133461993247937_i128),59951085118489959851513555683939732753_i128];
_2 = [(-119_i8),25_i8,(-84_i8),(-83_i8),111_i8,(-57_i8),(-54_i8)];
_2 = _6;
Goto(bb2)
}
bb2 = {
_2 = [66_i8,(-123_i8),35_i8,105_i8,111_i8,43_i8,(-36_i8)];
_8 = [(-121_i8),2_i8,47_i8,(-31_i8),7_i8,(-57_i8),61_i8];
_1 = [66331780798505705018996200331581673608_i128,(-118488746344624492595623832579964932308_i128),85603155666153707197752809091012550744_i128,(-137794453638964196977114878309087459520_i128),(-139447366952563066330031105142064198132_i128),(-143313231831671917758001902247468401788_i128),(-26672874879819217656965211999156946371_i128),(-161666982116492958769104828249582926576_i128)];
Goto(bb3)
}
bb3 = {
_4 = [47_i8,104_i8,21_i8,49_i8,(-102_i8),(-63_i8),83_i8];
_14.1 = 209852596392385206473768041308416501101_u128 as f32;
_10 = _2;
_3 = _1;
_15 = (-105_i8) >> _12;
_14.1 = 15848089914215752243_u64 as f32;
_15 = -(-22_i8);
_14.2 = _5;
_2 = _9;
_2 = [_15,_15,_15,_15,_15,_15,_15];
_8 = _6;
match _12 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607430933265819 => bb9,
_ => bb8
}
}
bb4 = {
_2 = [66_i8,(-123_i8),35_i8,105_i8,111_i8,43_i8,(-36_i8)];
_8 = [(-121_i8),2_i8,47_i8,(-31_i8),7_i8,(-57_i8),61_i8];
_1 = [66331780798505705018996200331581673608_i128,(-118488746344624492595623832579964932308_i128),85603155666153707197752809091012550744_i128,(-137794453638964196977114878309087459520_i128),(-139447366952563066330031105142064198132_i128),(-143313231831671917758001902247468401788_i128),(-26672874879819217656965211999156946371_i128),(-161666982116492958769104828249582926576_i128)];
Goto(bb3)
}
bb5 = {
_8 = _9;
RET = [3305446213218086382_u64,9948715396973292406_u64];
_3 = [39833674593822661427508586815287217738_i128,116542519412494264167600918106664063201_i128,168969446451097208326943031178972934014_i128,(-91119929364463902119713569759442277180_i128),(-133108442171099093609536284098571985452_i128),(-145234480667413820050179854454299100336_i128),(-7955338429376802906980709390544993675_i128),53198776437776844945502215757008408237_i128];
_8 = [15_i8,(-95_i8),(-53_i8),(-71_i8),(-64_i8),101_i8,5_i8];
_2 = [(-55_i8),(-79_i8),2_i8,87_i8,44_i8,111_i8,104_i8];
_2 = [101_i8,(-43_i8),20_i8,(-73_i8),31_i8,72_i8,(-8_i8)];
_12 = (-834945637_i32);
_1 = [(-109532594565661520179075251748953456470_i128),41805518001479869146929988921361600809_i128,3832367649668362517649674442375083069_i128,36784086720690657282808029369505470443_i128,(-68404886537302796982326271865851285140_i128),136867249437271326231548102872268249073_i128,71460559880771810442777518938513776545_i128,7267901615411893049786277416941288453_i128];
_2 = [(-31_i8),(-54_i8),(-21_i8),114_i8,98_i8,90_i8,(-99_i8)];
_5 = [(-143848141095152122395469886137957305743_i128),168521050270925300508242721316953384518_i128,6664835640149341926143874079533936128_i128,(-148768382531662005112642615043262206740_i128),80912445151945526623360769769643661586_i128,(-50854446935503269706334192781788807150_i128),(-140718753956930722522503426741936961512_i128),22826275540648711587078746530091363443_i128];
_4 = _8;
_7 = [(-166901988198960782487479241342056114632_i128),(-78240803459191651451149751836398104131_i128),(-70507971075201250674277001840333126280_i128),41662400950241679684597327262378280185_i128,20447798959296557749019953159601454532_i128,(-164300068001268483501465852198784829141_i128),(-17510956203426773278185133461993247937_i128),59951085118489959851513555683939732753_i128];
_2 = [(-119_i8),25_i8,(-84_i8),(-83_i8),111_i8,(-57_i8),(-54_i8)];
_2 = _6;
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
_1 = [(-34579754224178939486934512825844891374_i128),(-11804713175090955541303255418323894424_i128),41939497908261949609911035806887747420_i128,139076743704705012040769318514658647987_i128,(-83280714265103149536388496543760515175_i128),140842415465507292252449650831416824358_i128,(-128414295231008683814942511010708231223_i128),113329134262566748906245958633996811056_i128];
_12 = (-1265622725_i32) << _15;
_2 = [_15,_15,_15,_15,_15,_15,_15];
_14.0 = [_15,_15,_15,_15,_15,_15,_15];
_8 = _6;
_1 = [69906307587470016162615728321302216873_i128,(-153198769791760676797460499127250816760_i128),161905523203792730088995843777990519668_i128,48161527763191210164661803553751090960_i128,150750937981677063279165855240406505683_i128,104685997017262217716641788981245057234_i128,(-78515522349989552570708493716494237285_i128),21508487634096313037688578122453730362_i128];
_7 = [(-121399183662856166886376017889859523727_i128),17957068587791327531929578043672370273_i128,(-125603012290278132211644315503186733667_i128),7637563283619871875619091074182548096_i128,50038992725585647906792631149442612877_i128,(-57327799859076975088217308079173025110_i128),(-140794995317454230945255956901519651827_i128),92502429127001601030132121240439299025_i128];
_6 = [_15,_15,_15,_15,_15,_15,_15];
_3 = [(-48631280783601076858022096544084896748_i128),113395842137556673001382125211143667540_i128,31479595656059320777728030486582368674_i128,82451588885008911243559296265012487405_i128,(-2876822782431552695847750867181295456_i128),50850277748968932236764712222693580641_i128,(-158105527340478087045874178882015631094_i128),(-42247372467229540102949639726927896358_i128)];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_4 = _9;
_18 = 67_u8 as f64;
_4 = _8;
_16 = -9_isize;
_7 = [4214838684301557156004142591549549667_i128,125781531040414257529688523180132889137_i128,144043775213125467644256263781471879208_i128,(-37499320412260476200733106264652804620_i128),139436940416806030926598615647823776992_i128,(-44694476369855866793832411518978719508_i128),125789106874204803633709935085099729821_i128,(-59946020111259096550077066533371984026_i128)];
_3 = [(-61365760842670281243669078210846396084_i128),142674707934530351519134888284259419945_i128,(-123922068992642393274926322117917391672_i128),135256634970561153355327836106849522790_i128,(-42700235725516476730958606625325366728_i128),71686959016722866991415453957780081251_i128,(-75182995973082481192093101376966259150_i128),(-127740384871496247221685408551681902813_i128)];
_12 = -1098614657_i32;
_5 = [107730240344179626546212786516106920973_i128,(-60888760975190199768005192837041952869_i128),(-55778565482618377840702753063897541722_i128),(-83992390134479276728592468278204191188_i128),(-45342132740474467374221812563460409121_i128),(-137824900891667845806470326756229026233_i128),120077384276089383572490641811300447938_i128,(-50582814288771807691516486300083750495_i128)];
_4 = _6;
_20.0 = 24116_i16;
_19 = true as u32;
_1 = [(-3716340825382585374583871765426672715_i128),73190947674024121639148797353765060088_i128,(-118961150014941519610249334955244915177_i128),44628528593524310086975994266161979317_i128,138625461028567743963078995640795949029_i128,(-46773981361101939700651461672854707954_i128),(-26854794826659917503922018162103012239_i128),(-129257489991587348554658838561784306274_i128)];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_20.1 = [_20.0,_20.0,_20.0];
Goto(bb10)
}
bb10 = {
_4 = [_15,_15,_15,_15,_15,_15,_15];
_3 = [166004430797106200391856224430905816072_i128,(-7540392948801649717006498713769123138_i128),132902643951284225683524485776417553617_i128,106600353642488758461455344434287637284_i128,(-155708432342640866999404675644460977104_i128),(-57193783575435587614260261233653681919_i128),(-163676606945519729579663147957752030051_i128),155017625077668977223543314459180783545_i128];
_22.0 = [87526890236636550316275173662925092327_u128,106047172988632804468351019907888188863_u128,331413630200770989847916439296075759200_u128,20516703598998571862538105366316923240_u128,160747304155231782125614816994568483897_u128,114121030476180162290243322010631682553_u128];
_12 = -(-892992400_i32);
_12 = 582633603_i32 << _19;
_14.2 = [(-91302496561820791839161623435292660857_i128),(-124459581977819591038436428756624705431_i128),45027459267734228322750395905057669045_i128,(-22062872357568847265013219366031984853_i128),(-38685875884807977616950947560952467434_i128),(-13040930131261384905138482482533092506_i128),108329755619391069929353894653802843996_i128,(-67516528122849743157892216498595851506_i128)];
_14.2 = [(-124893101342628055825536263709634377967_i128),31975826326295512601853298967323887057_i128,(-148283228188336335442221471488847731455_i128),55983047179194893299935244879507569698_i128,(-69264935995289688811094608481348891670_i128),21026870969214719807946775619021530904_i128,46122857285993633997069088746050356226_i128,37806087489988778865778429604322190820_i128];
RET = [13363380555823926400_u64,9592151687165206516_u64];
_22.0 = [264629480636725530535263014649884572967_u128,278887966323838824505565054837949229903_u128,143371255298988482465182954955893979389_u128,176176047618289523457079937246902807973_u128,73887513822581936564551765021793316110_u128,110934181456194637865636233045657394519_u128];
_6 = _8;
_10 = [_15,_15,_15,_15,_15,_15,_15];
_23 = !true;
_22.0 = [205200039459822277360914072644745243059_u128,324968784723397334763573595078315391310_u128,8588362291379520135312045891624341694_u128,19511924050746230357723836663964764620_u128,309699431412370727337930197522915237492_u128,252071418916466215274158819690748206475_u128];
_16 = -(-116_isize);
_4 = [_15,_15,_15,_15,_15,_15,_15];
_4 = [_15,_15,_15,_15,_15,_15,_15];
_20.0 = 30151_i16;
_22.0 = [331663600781942836071570990760144687397_u128,175289103571315429341972966016967299975_u128,78300846044223577659880851684815149444_u128,73455119996330756714393854645273879595_u128,180065207968943396219244932153494921388_u128,122249852399749626147617252329065682907_u128];
_15 = 125_i8;
_16 = 9223372036854775807_isize;
_16 = 16397_u16 as isize;
_14.1 = 60937_u16 as f32;
_18 = (-135765660241638942019409139540944402873_i128) as f64;
_14.2 = [(-27818918514626571249851055551520437666_i128),(-76453865141336630090813332943844733784_i128),(-65702343227623315299531540800609994213_i128),70955472783615173348548880207099437578_i128,(-55123421372517854076020962167710389737_i128),43148495135125298500998609333613989997_i128,117015291294235128169553920670043983199_i128,(-38855512422662066454705252414951705707_i128)];
match _20.0 {
0 => bb6,
1 => bb2,
2 => bb11,
3 => bb12,
4 => bb13,
30151 => bb15,
_ => bb14
}
}
bb11 = {
_1 = [(-34579754224178939486934512825844891374_i128),(-11804713175090955541303255418323894424_i128),41939497908261949609911035806887747420_i128,139076743704705012040769318514658647987_i128,(-83280714265103149536388496543760515175_i128),140842415465507292252449650831416824358_i128,(-128414295231008683814942511010708231223_i128),113329134262566748906245958633996811056_i128];
_12 = (-1265622725_i32) << _15;
_2 = [_15,_15,_15,_15,_15,_15,_15];
_14.0 = [_15,_15,_15,_15,_15,_15,_15];
_8 = _6;
_1 = [69906307587470016162615728321302216873_i128,(-153198769791760676797460499127250816760_i128),161905523203792730088995843777990519668_i128,48161527763191210164661803553751090960_i128,150750937981677063279165855240406505683_i128,104685997017262217716641788981245057234_i128,(-78515522349989552570708493716494237285_i128),21508487634096313037688578122453730362_i128];
_7 = [(-121399183662856166886376017889859523727_i128),17957068587791327531929578043672370273_i128,(-125603012290278132211644315503186733667_i128),7637563283619871875619091074182548096_i128,50038992725585647906792631149442612877_i128,(-57327799859076975088217308079173025110_i128),(-140794995317454230945255956901519651827_i128),92502429127001601030132121240439299025_i128];
_6 = [_15,_15,_15,_15,_15,_15,_15];
_3 = [(-48631280783601076858022096544084896748_i128),113395842137556673001382125211143667540_i128,31479595656059320777728030486582368674_i128,82451588885008911243559296265012487405_i128,(-2876822782431552695847750867181295456_i128),50850277748968932236764712222693580641_i128,(-158105527340478087045874178882015631094_i128),(-42247372467229540102949639726927896358_i128)];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_4 = _9;
_18 = 67_u8 as f64;
_4 = _8;
_16 = -9_isize;
_7 = [4214838684301557156004142591549549667_i128,125781531040414257529688523180132889137_i128,144043775213125467644256263781471879208_i128,(-37499320412260476200733106264652804620_i128),139436940416806030926598615647823776992_i128,(-44694476369855866793832411518978719508_i128),125789106874204803633709935085099729821_i128,(-59946020111259096550077066533371984026_i128)];
_3 = [(-61365760842670281243669078210846396084_i128),142674707934530351519134888284259419945_i128,(-123922068992642393274926322117917391672_i128),135256634970561153355327836106849522790_i128,(-42700235725516476730958606625325366728_i128),71686959016722866991415453957780081251_i128,(-75182995973082481192093101376966259150_i128),(-127740384871496247221685408551681902813_i128)];
_12 = -1098614657_i32;
_5 = [107730240344179626546212786516106920973_i128,(-60888760975190199768005192837041952869_i128),(-55778565482618377840702753063897541722_i128),(-83992390134479276728592468278204191188_i128),(-45342132740474467374221812563460409121_i128),(-137824900891667845806470326756229026233_i128),120077384276089383572490641811300447938_i128,(-50582814288771807691516486300083750495_i128)];
_4 = _6;
_20.0 = 24116_i16;
_19 = true as u32;
_1 = [(-3716340825382585374583871765426672715_i128),73190947674024121639148797353765060088_i128,(-118961150014941519610249334955244915177_i128),44628528593524310086975994266161979317_i128,138625461028567743963078995640795949029_i128,(-46773981361101939700651461672854707954_i128),(-26854794826659917503922018162103012239_i128),(-129257489991587348554658838561784306274_i128)];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_20.1 = [_20.0,_20.0,_20.0];
Goto(bb10)
}
bb12 = {
Return()
}
bb13 = {
_2 = [66_i8,(-123_i8),35_i8,105_i8,111_i8,43_i8,(-36_i8)];
_8 = [(-121_i8),2_i8,47_i8,(-31_i8),7_i8,(-57_i8),61_i8];
_1 = [66331780798505705018996200331581673608_i128,(-118488746344624492595623832579964932308_i128),85603155666153707197752809091012550744_i128,(-137794453638964196977114878309087459520_i128),(-139447366952563066330031105142064198132_i128),(-143313231831671917758001902247468401788_i128),(-26672874879819217656965211999156946371_i128),(-161666982116492958769104828249582926576_i128)];
Goto(bb3)
}
bb14 = {
_8 = _9;
RET = [3305446213218086382_u64,9948715396973292406_u64];
_3 = [39833674593822661427508586815287217738_i128,116542519412494264167600918106664063201_i128,168969446451097208326943031178972934014_i128,(-91119929364463902119713569759442277180_i128),(-133108442171099093609536284098571985452_i128),(-145234480667413820050179854454299100336_i128),(-7955338429376802906980709390544993675_i128),53198776437776844945502215757008408237_i128];
_8 = [15_i8,(-95_i8),(-53_i8),(-71_i8),(-64_i8),101_i8,5_i8];
_2 = [(-55_i8),(-79_i8),2_i8,87_i8,44_i8,111_i8,104_i8];
_2 = [101_i8,(-43_i8),20_i8,(-73_i8),31_i8,72_i8,(-8_i8)];
_12 = (-834945637_i32);
_1 = [(-109532594565661520179075251748953456470_i128),41805518001479869146929988921361600809_i128,3832367649668362517649674442375083069_i128,36784086720690657282808029369505470443_i128,(-68404886537302796982326271865851285140_i128),136867249437271326231548102872268249073_i128,71460559880771810442777518938513776545_i128,7267901615411893049786277416941288453_i128];
_2 = [(-31_i8),(-54_i8),(-21_i8),114_i8,98_i8,90_i8,(-99_i8)];
_5 = [(-143848141095152122395469886137957305743_i128),168521050270925300508242721316953384518_i128,6664835640149341926143874079533936128_i128,(-148768382531662005112642615043262206740_i128),80912445151945526623360769769643661586_i128,(-50854446935503269706334192781788807150_i128),(-140718753956930722522503426741936961512_i128),22826275540648711587078746530091363443_i128];
_4 = _8;
_7 = [(-166901988198960782487479241342056114632_i128),(-78240803459191651451149751836398104131_i128),(-70507971075201250674277001840333126280_i128),41662400950241679684597327262378280185_i128,20447798959296557749019953159601454532_i128,(-164300068001268483501465852198784829141_i128),(-17510956203426773278185133461993247937_i128),59951085118489959851513555683939732753_i128];
_2 = [(-119_i8),25_i8,(-84_i8),(-83_i8),111_i8,(-57_i8),(-54_i8)];
_2 = _6;
Goto(bb2)
}
bb15 = {
RET = [6782271431627767116_u64,15650422076570687013_u64];
_14.2 = [18177363328265124049083292242750083888_i128,112568070369935689879643097593232673202_i128,37069847139983667282101476091466695267_i128,(-10450787524480149061982131560216522315_i128),67067304106995507682196382801270651434_i128,18426991640371342006227738750978126828_i128,(-82665989835349677004612531375659783416_i128),115555733149431185233511486305800127111_i128];
_10 = [_15,_15,_15,_15,_15,_15,_15];
_19 = 2181635193_u32 >> _20.0;
_1 = _14.2;
_2 = _8;
_26 = [_20.0,_20.0,_20.0];
_3 = [142646509470666975154748103763800077186_i128,(-58030920334512155056466717491455445006_i128),96612438975187007971145692921251478738_i128,(-47116585703854459910917352709497920616_i128),34078838761753404008767717259928048837_i128,(-24873664597752380769602724637456600105_i128),(-82585080083537011671267733389386274273_i128),(-84539074862504335980797457750933918628_i128)];
_14.1 = 6718226525897255031_u64 as f32;
_12 = (-1423306731_i32);
_6 = _4;
_18 = _14.1 as f64;
_19 = !128022723_u32;
_5 = [165866085745121549954889911267880263786_i128,(-50668468976561044017708706353386765772_i128),54894227456326068693311386454092251292_i128,5795982126944629134806399022685589398_i128,(-139872684028369808368883171493371592846_i128),104099685639233022973725505326086764128_i128,(-87166831377738396832463437869558149794_i128),(-113452661865517320029558721976863493785_i128)];
_6 = [_15,_15,_15,_15,_15,_15,_15];
_3 = _7;
_15 = 48_i8;
_25 = [_15,_15,_15,_15,_15,_15,_15];
_24 = Adt40::Variant1 { fld0: 10013333961514238118_u64,fld1: 3_usize,fld2: _20.0 };
_10 = _8;
_9 = [_15,_15,_15,_15,_15,_15,_15];
_26 = _20.1;
_20 = (Field::<i16>(Variant(_24, 1), 2), _26);
_2 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(7_usize, 16_usize, Move(_16), 4_usize, Move(_4), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(7_usize, 8_usize, Move(_8), 10_usize, Move(_10), 23_usize, Move(_23), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(7_usize, 7_usize, Move(_7), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i8; 7],mut _2: [i8; 7],mut _3: [i8; 7],mut _4: [i8; 7],mut _5: [u64; 2],mut _6: i64,mut _7: i64,mut _8: [i8; 7],mut _9: [i8; 7],mut _10: [i128; 8],mut _11: [i8; 7],mut _12: [i128; 8],mut _13: [i128; 8],mut _14: [i128; 8]) -> u8 {
mir! {
type RET = u8;
let _15: u32;
let _16: u16;
let _17: bool;
let _18: f64;
let _19: [i16; 8];
let _20: u128;
let _21: [i8; 7];
let _22: [i16; 8];
let _23: (isize, [u64; 2]);
let _24: Adt41;
let _25: bool;
let _26: bool;
let _27: f64;
let _28: char;
let _29: i128;
let _30: i128;
let _31: i128;
let _32: f64;
let _33: (u32,);
let _34: (i16, usize, [u64; 2], i16);
let _35: usize;
let _36: [i16; 3];
let _37: bool;
let _38: [i16; 3];
let _39: (i16, [i16; 3]);
let _40: ();
let _41: ();
{
RET = 112_u8 + 236_u8;
RET = 124_u8;
_12 = [(-53370032443758125299609576329688263010_i128),(-91671142788120109923204989882201906540_i128),85604160207449037861414566369271875800_i128,(-77725037647058469724565436647079528375_i128),43892445640026674160643997531335111956_i128,82983622168943716992301623054269896538_i128,(-121060307675028240141330312722913990113_i128),(-77016583237308060022984847938731664563_i128)];
_15 = !1136204734_u32;
match RET {
0 => bb1,
124 => bb3,
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
_10 = [(-95680931337393842127610522176736662245_i128),5113711404276266580581010725440075113_i128,(-109480971747832177614420894834622865514_i128),(-85431226060759355955078619861996744913_i128),67329624112577987506708174722835436742_i128,(-118686000298610656937958054472283999828_i128),(-45261806540961730014776640246561604434_i128),(-58857346232522275886153785176121169369_i128)];
_17 = true;
_16 = 3417569424200714024_usize as u16;
_4 = [(-49_i8),(-88_i8),(-23_i8),102_i8,103_i8,(-38_i8),10_i8];
_13 = _12;
_19 = [12138_i16,22087_i16,12127_i16,(-9257_i16),(-10542_i16),(-9746_i16),(-16221_i16),24540_i16];
_9 = [106_i8,65_i8,(-33_i8),41_i8,5_i8,(-19_i8),15_i8];
_9 = [(-89_i8),20_i8,(-93_i8),(-109_i8),32_i8,119_i8,45_i8];
_18 = 33227341636046641450615799492474417902_u128 as f64;
Goto(bb4)
}
bb4 = {
_9 = [(-4_i8),72_i8,(-116_i8),(-110_i8),72_i8,(-36_i8),(-121_i8)];
_13 = _10;
_12 = _13;
_12 = _13;
_1 = _8;
_5 = [8339836776673888161_u64,4343867379682757438_u64];
_18 = 542266914_i32 as f64;
RET = 55_u8 * 185_u8;
_20 = !191171455949207906960863339676760348747_u128;
_10 = [(-82942103333637710699869517929738804443_i128),65222174095813523250771555442350333594_i128,139455413806644064302664279780750327790_i128,108850504325785403212800279552361071264_i128,(-78400203338904321271602744874429478849_i128),(-93819135584190246968278738482782373761_i128),5027384521389862561632832857858146193_i128,154390228815728806407240894377209415369_i128];
RET = 24_u8 | 162_u8;
_19 = [1705_i16,(-28948_i16),(-27837_i16),3720_i16,(-3742_i16),(-32597_i16),14489_i16,19274_i16];
_3 = [78_i8,127_i8,(-57_i8),(-54_i8),(-126_i8),35_i8,76_i8];
_2 = [(-29_i8),(-81_i8),(-58_i8),(-79_i8),23_i8,(-86_i8),(-50_i8)];
Call(_7 = fn9(_13, _12, _9, _11, _13, _2, _12, _9, _13, _3, _10, _10, _19, _12, _12, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10 = [(-12902485290264929540360250224809414433_i128),(-73061781588411692720676795981327997666_i128),(-53021344726777229457720713222090282091_i128),99069795489328925402174427814314072228_i128,(-132987769245026265906700057288881361878_i128),48090497545597212466369320377040661700_i128,113065208605106733839002366195166010937_i128,(-44826771827529931052002305542616871342_i128)];
_2 = _9;
_3 = [(-45_i8),(-41_i8),40_i8,(-126_i8),(-35_i8),34_i8,(-1_i8)];
_2 = [50_i8,(-126_i8),57_i8,105_i8,(-70_i8),(-68_i8),(-45_i8)];
_7 = !_6;
_15 = 615995420_u32 | 3103130597_u32;
_22 = [(-27446_i16),16709_i16,25778_i16,(-26316_i16),25330_i16,(-12575_i16),(-18687_i16),(-12391_i16)];
_8 = [67_i8,(-78_i8),43_i8,28_i8,58_i8,(-46_i8),(-72_i8)];
_25 = _17 ^ _17;
_23 = ((-9223372036854775808_isize), _5);
_10 = [95879555702080722596437817283298786899_i128,142655996898601362400634747871420647683_i128,(-132483217575146275275825217883061407953_i128),32393849207193610228121394958726394594_i128,91672272118244949544405639081375736400_i128,168980470517952631314205163401571576511_i128,(-75421472163781171225445241531601931356_i128),(-86780753684139116594679688593840955333_i128)];
_4 = [63_i8,(-61_i8),109_i8,61_i8,74_i8,(-65_i8),67_i8];
_16 = '\u{6cd82}' as u16;
_21 = [124_i8,(-76_i8),(-111_i8),(-34_i8),(-126_i8),(-61_i8),(-99_i8)];
RET = !88_u8;
_20 = 298515052619672251361222057912597298591_u128 << _15;
_16 = '\u{fa153}' as u16;
Call(_8 = core::intrinsics::transmute(_4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8 = _9;
match _23.0 {
0 => bb5,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_19 = _22;
_22 = [29322_i16,(-1336_i16),(-21846_i16),(-13149_i16),19669_i16,30798_i16,(-14934_i16),12977_i16];
_10 = [92902688341130495655388657987384778560_i128,92904130537348200069096145677650718821_i128,(-66755109607791263555856699285391652202_i128),(-114019052965819747368477093138707810412_i128),(-233845886833074148626148726603646345_i128),7883351788424206091216720777353378757_i128,42773363386973973329060724616326139439_i128,(-119789352940686685172146869419625339813_i128)];
_16 = 18414_u16;
_18 = 123_i8 as f64;
_18 = (-44_i8) as f64;
_20 = 217184502307367692219160316299288380386_u128 >> _15;
_29 = 11823539225065994806108784385448810585_i128 * 21443142658965017375513802244126581107_i128;
_15 = 516254865_u32;
_9 = _8;
_26 = !_25;
_5 = [1994010368522947677_u64,15470641948880242897_u64];
Goto(bb9)
}
bb9 = {
_26 = _25;
_2 = [94_i8,(-62_i8),91_i8,42_i8,79_i8,8_i8,(-30_i8)];
_9 = [53_i8,32_i8,67_i8,(-70_i8),(-31_i8),(-1_i8),77_i8];
_21 = [(-76_i8),(-97_i8),29_i8,(-53_i8),48_i8,84_i8,(-26_i8)];
match _23.0 {
0 => bb2,
1 => bb10,
340282366920938463454151235394913435648 => bb12,
_ => bb11
}
}
bb10 = {
_10 = [(-95680931337393842127610522176736662245_i128),5113711404276266580581010725440075113_i128,(-109480971747832177614420894834622865514_i128),(-85431226060759355955078619861996744913_i128),67329624112577987506708174722835436742_i128,(-118686000298610656937958054472283999828_i128),(-45261806540961730014776640246561604434_i128),(-58857346232522275886153785176121169369_i128)];
_17 = true;
_16 = 3417569424200714024_usize as u16;
_4 = [(-49_i8),(-88_i8),(-23_i8),102_i8,103_i8,(-38_i8),10_i8];
_13 = _12;
_19 = [12138_i16,22087_i16,12127_i16,(-9257_i16),(-10542_i16),(-9746_i16),(-16221_i16),24540_i16];
_9 = [106_i8,65_i8,(-33_i8),41_i8,5_i8,(-19_i8),15_i8];
_9 = [(-89_i8),20_i8,(-93_i8),(-109_i8),32_i8,119_i8,45_i8];
_18 = 33227341636046641450615799492474417902_u128 as f64;
Goto(bb4)
}
bb11 = {
Return()
}
bb12 = {
_30 = _29;
_22 = [25354_i16,17776_i16,(-30981_i16),(-5133_i16),22665_i16,23421_i16,17013_i16,18260_i16];
_3 = _8;
_1 = [55_i8,127_i8,(-78_i8),(-11_i8),(-73_i8),(-104_i8),105_i8];
_28 = '\u{8bbdf}';
_13 = [_29,_30,_30,_29,_29,_29,_30,_29];
_5 = [10212473666375667542_u64,15111046274783092921_u64];
_23.0 = -9223372036854775807_isize;
_14 = [_29,_29,_29,_29,_29,_29,_29,_29];
_23.1 = _5;
_3 = _8;
_10 = [_29,_30,_30,_29,_29,_30,_29,_29];
_11 = [(-10_i8),(-42_i8),61_i8,(-72_i8),(-125_i8),91_i8,(-76_i8)];
_21 = [(-65_i8),(-89_i8),30_i8,9_i8,(-120_i8),52_i8,(-76_i8)];
_9 = _3;
_23.1 = [1629939328945028678_u64,3734997754743432312_u64];
_17 = _25;
_28 = '\u{e9d27}';
_33 = (_15,);
_33.0 = _15;
_34.0 = !16907_i16;
_32 = 1452850857_i32 as f64;
Goto(bb13)
}
bb13 = {
_6 = _20 as i64;
_30 = _29;
_15 = _6 as u32;
_34.3 = -_34.0;
_23 = (9223372036854775807_isize, _5);
RET = 32_u8;
_36 = [_34.3,_34.3,_34.0];
_32 = -_18;
_4 = [(-52_i8),(-118_i8),55_i8,(-23_i8),95_i8,(-83_i8),(-56_i8)];
_4 = _21;
_31 = !_29;
_2 = [0_i8,(-71_i8),(-62_i8),(-99_i8),(-128_i8),(-5_i8),(-79_i8)];
_28 = '\u{f32da}';
_37 = !_26;
_3 = [7_i8,(-126_i8),108_i8,90_i8,(-122_i8),(-78_i8),112_i8];
_20 = _16 as u128;
_33 = (_15,);
_25 = !_26;
_16 = !27915_u16;
_15 = _33.0 & _33.0;
RET = 123_u8;
_32 = -_18;
_2 = [91_i8,(-23_i8),(-43_i8),(-44_i8),(-108_i8),90_i8,(-12_i8)];
_29 = _31;
_15 = _33.0;
RET = !139_u8;
Goto(bb14)
}
bb14 = {
_34.1 = 1_usize << _33.0;
_34.2 = [18042322736640256615_u64,1904982818317763142_u64];
_14 = [_31,_31,_29,_30,_30,_31,_31,_30];
RET = 207_u8;
_27 = -_32;
_15 = _33.0;
_35 = _34.1 >> _33.0;
_13 = [_31,_29,_30,_31,_29,_29,_30,_30];
RET = _23.0 as u8;
_36 = [_34.3,_34.0,_34.3];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(8_usize, 22_usize, Move(_22), 11_usize, Move(_11), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(8_usize, 16_usize, Move(_16), 31_usize, Move(_31), 26_usize, Move(_26), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(8_usize, 2_usize, Move(_2), 37_usize, Move(_37), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(8_usize, 21_usize, Move(_21), 4_usize, Move(_4), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i8; 7],mut _4: [i8; 7],mut _5: [i128; 8],mut _6: [i8; 7],mut _7: [i128; 8],mut _8: [i8; 7],mut _9: [i128; 8],mut _10: [i8; 7],mut _11: [i128; 8],mut _12: [i128; 8],mut _13: [i16; 8],mut _14: [i128; 8],mut _15: [i128; 8],mut _16: [i8; 7]) -> i64 {
mir! {
type RET = i64;
let _17: bool;
let _18: isize;
let _19: *mut [u128; 6];
let _20: (isize, [u64; 2]);
let _21: isize;
let _22: f32;
let _23: [u128; 6];
let _24: &'static u8;
let _25: [i128; 8];
let _26: isize;
let _27: [i16; 3];
let _28: [i16; 8];
let _29: ([i8; 7], f32, [i128; 8]);
let _30: isize;
let _31: u32;
let _32: isize;
let _33: f64;
let _34: bool;
let _35: u16;
let _36: Adt47;
let _37: i16;
let _38: Adt51;
let _39: [i16; 3];
let _40: u64;
let _41: f32;
let _42: [i128; 8];
let _43: ([i8; 7], f32, [i128; 8]);
let _44: bool;
let _45: isize;
let _46: usize;
let _47: [u64; 2];
let _48: *const *mut [u128; 6];
let _49: Adt47;
let _50: bool;
let _51: (i16, [i16; 3]);
let _52: i8;
let _53: Adt40;
let _54: isize;
let _55: bool;
let _56: usize;
let _57: (isize, [u64; 2]);
let _58: [u64; 2];
let _59: [i16; 3];
let _60: ();
let _61: ();
{
_16 = [(-65_i8),(-96_i8),(-117_i8),125_i8,(-57_i8),78_i8,(-44_i8)];
_5 = _9;
RET = -4493884789196353012_i64;
RET = (-357088096292008401_i64);
_2 = [(-122403933639340360388192995146551240156_i128),(-818884316298507932886057685444584537_i128),(-41983715175194383216666028878713612943_i128),(-33551280079674702721121769887654299242_i128),(-122198490624995181945659836912626267765_i128),(-143554179888782241169372734342749851075_i128),(-58042711899517149505818147664000917492_i128),169495098500030077541724128980814789236_i128];
RET = 5283485739436579901_i64;
_7 = [(-67735930217374512687544821641163336870_i128),(-95111288692232338714853715376724266076_i128),50665993037711937026395839825334170160_i128,(-156037468662772681150242699821411933068_i128),(-97767109353329956896030525449726341049_i128),(-116927919856523504112573312291622975_i128),(-132110289449081896040819749127764206640_i128),74887350973115510289046674837823007545_i128];
_3 = _4;
_8 = [27_i8,(-80_i8),(-51_i8),34_i8,62_i8,1_i8,98_i8];
_13 = [23690_i16,(-1416_i16),(-25132_i16),5568_i16,(-20136_i16),945_i16,(-13349_i16),21805_i16];
_12 = _15;
_17 = false | true;
RET = (-7003884856214179350_i64);
_9 = _15;
_3 = _8;
_15 = [(-116300164883206861700226343630295072005_i128),(-166940715010883201448772078767077257009_i128),24069949556680150519082184696695693745_i128,(-10841603340022447916123676369914292254_i128),153104116068581161708934293768532946216_i128,160913123732553921849534883385002930115_i128,62946190403477415522299069146728186068_i128,(-25534399148537322277637367706310441052_i128)];
RET = (-6807987553111067129_i64) | (-7105244602378369610_i64);
_5 = [(-61573098183579354194726554688934382608_i128),169043686929529647781948384801930952258_i128,(-158192913583723563185759342790903456167_i128),10830895104965337398775471128527993824_i128,(-125735266970317082543229767251230611022_i128),(-131787767404324678269300798276368366059_i128),155000206882489385071356945238408433379_i128,45741641535926333158788432528324152145_i128];
_2 = [16721596700564222951232415475975672565_i128,(-47071641008153346150151813144075185576_i128),118767702162800168824269748618104460103_i128,(-141907713200176169249775446243942399093_i128),(-21632615571056731925341648284852066947_i128),(-121744716542788752910717025979429220070_i128),(-116347457602157954144630076913047745122_i128),138222226499435306048743877516742622285_i128];
_14 = [2481018564120298761775270624270062204_i128,(-48983199536655449611637327330088517481_i128),(-111401624243703119099439525795568685863_i128),6175096699098020562409719398096606279_i128,22909925143550723438717856072429106251_i128,11242592330671118284520598431722287521_i128,(-82866627996464016132282730851197871141_i128),(-46399114278118393317327936851893433980_i128)];
_2 = [(-47047134167941516828580668234259998038_i128),(-163633957304684040470394863377468832750_i128),35398503964359703701273551650834318923_i128,(-10372878533131878440872570816960290950_i128),(-31401915710136194159564994696841625200_i128),84500975820363733913942403623421820625_i128,(-109069807491878532615608041486375123495_i128),91374215854561775511884402709734438209_i128];
_14 = [(-127126470893273800423546683246723227211_i128),78504822638475677614887963763619821379_i128,(-16788339341807701226900866622973988900_i128),(-123514078895073240017622660413152694235_i128),(-36137470693073669306591170468076093668_i128),8312041255000679705814913016619116647_i128,(-77708548808652437362911140725105320876_i128),(-5792352328681968505720626113667473016_i128)];
_3 = [(-97_i8),(-22_i8),86_i8,99_i8,(-39_i8),(-53_i8),(-72_i8)];
_11 = [(-21308010423063067723625167385823302870_i128),12691866128825048381025888332048980388_i128,(-113622287165062569498034058344749583909_i128),130297829101309453014250776081716265167_i128,89265993753323823149895542732516120723_i128,168336834850875550357814976243307975182_i128,(-157121921046772769126880517682297918490_i128),33321037106337925740787215355843031193_i128];
_2 = [(-160948790889077407052895401252203253605_i128),80162818218738349669324273638899938340_i128,132055601301227880301403795801810042088_i128,93167203674838279208669517817090564233_i128,72361037360831875149188861363836151891_i128,(-38557675078022769358743957956511986121_i128),36271131104749295627201123323242138146_i128,11937763551062350138111918849214611623_i128];
Call(_14 = fn10(_9, _3, _10, _9, RET, _3, _12, _6, _8, _15, _12, _5, _11, _2, _12, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = [58517771148946237700335500443233498353_i128,67146524757064767972849324091377537285_i128,39135876484323675161288731215613566543_i128,(-134171937333495295202806518507755654756_i128),153214464485525427489850212711236341066_i128,(-163582403606676348458316034099805260474_i128),(-143150347563814299922016469508976594573_i128),104038047280606410372039746702026651813_i128];
_18 = 9223372036854775807_isize;
_7 = [162931375381526780431582311212460657731_i128,(-16330151521342019130277395222058232512_i128),(-133972618428945187825718559235583910142_i128),(-152064687377163195170053717867920395271_i128),(-149442303627800597447651500224451548350_i128),143058074214538803910510153397050070219_i128,(-34065113414590867441180591919679658474_i128),123728019647161389080704986523892714595_i128];
_16 = [(-117_i8),106_i8,(-40_i8),(-110_i8),76_i8,32_i8,23_i8];
_8 = [(-111_i8),(-20_i8),(-103_i8),87_i8,81_i8,(-86_i8),(-31_i8)];
_15 = [(-9904734375150842799239439921602420287_i128),28969416644084019214149759756575103257_i128,142938038327754936986759207514018533064_i128,31863000393192986575709859102765309731_i128,31911558733986300673866090611272305051_i128,(-88451191233338663944234771974001475228_i128),(-43834578271234007764689393793271726195_i128),(-94688440973214844181557590549508061354_i128)];
_17 = !false;
_17 = RET != RET;
RET = 1735965898705111502_i64 & 7607619554665735251_i64;
Goto(bb2)
}
bb2 = {
_9 = [62488861543562158926976992671622706941_i128,22674641952980776701733568037427387622_i128,(-23182074338155354703456628813644923362_i128),59568650517378965426526950837154950750_i128,24821216026059906857972274271134981943_i128,145873140963683393430795850764807412385_i128,59539359615167563275998701462770599300_i128,132699410114859647737416971197915753293_i128];
_4 = [(-45_i8),(-125_i8),42_i8,(-18_i8),127_i8,76_i8,120_i8];
_6 = [71_i8,(-99_i8),118_i8,9_i8,(-116_i8),(-105_i8),48_i8];
_11 = [(-117840977556309177714158665699075707457_i128),(-116005805307687072194757772230684402143_i128),(-48211698427012742111872336688310661481_i128),34710267891286119252353294306068516947_i128,(-103501709116270907084560867007147375766_i128),(-124335736159393227511497993810420238186_i128),(-79354611118067918952136029979511975254_i128),93136484895844651254552940505646649328_i128];
_21 = -_18;
_12 = [95285617819960518230887871910823029475_i128,16197096312343923279429824914049503745_i128,(-150675956335928149713899369275294579392_i128),(-71529919460773307493152889183131724801_i128),52008433174591538052043079586580737038_i128,(-37325901244515045549543091781239754103_i128),(-97315269568752383854437050555564711337_i128),(-9455907635275697608792294672576312882_i128)];
_20.0 = _21 ^ _21;
_16 = _8;
_14 = _1;
_18 = _21 & _20.0;
_21 = !_20.0;
RET = (-6294343524768606764_i64);
_11 = _2;
Goto(bb3)
}
bb3 = {
_10 = _16;
_18 = _20.0;
_17 = false;
_12 = [135117886824936803971647340856462558425_i128,(-67403702471846669944806181501041401477_i128),(-83190963075316367277351969400373960724_i128),106863723059994569683140480106452551011_i128,110930242547761311248040140601023767231_i128,67408386080808783728074851609301107452_i128,(-136294621653321485457907295668503649599_i128),40977268746260550257865289380950025711_i128];
_5 = [(-56753153289929236934887586852801207174_i128),(-117252611098426064596814420611286662952_i128),(-4703854491976412066206040707135134285_i128),142973630627875765350280467404735657933_i128,150134331131298188108671364230493498947_i128,(-139047356864101225419800600243384288973_i128),(-19583228124561013975322960231047021044_i128),46866682969649464379179929252467420372_i128];
_23 = [276916163092978584244110109821356307988_u128,22275580191991891307387191553212766545_u128,104504228985327196642382221465032113199_u128,158565109066250472784506125213165131185_u128,92472027603021783528940463031097470174_u128,291197826491282086904401819276216814804_u128];
_20.1 = [9331985010723159317_u64,7888471767717899049_u64];
_7 = [(-168123538641146330706549212384579776985_i128),104807061463706315357086053134475244753_i128,(-94187717753747539123970939608294584036_i128),(-5060775140893058598663656174110760367_i128),(-144400032879883836432757051547495025900_i128),(-13543873525519654094125827819311555077_i128),(-41915473730948461873614423368833409800_i128),86794283193739830558971631512801176593_i128];
_10 = [99_i8,35_i8,84_i8,(-46_i8),(-113_i8),53_i8,108_i8];
_15 = _9;
RET = (-1452710145354997641_i64) | 3661263453234027758_i64;
_1 = _7;
_25 = [52451504738983172102685701923902439920_i128,(-130592077117579060926022130507149588269_i128),68229850674561789862333563350111768056_i128,70677253520335611375430106853737857183_i128,66514982915726462497882815245417399058_i128,31987328680294572806917930046140737791_i128,29339242236257227864590726227191708452_i128,7443193677931018572160878039287667354_i128];
_4 = [(-113_i8),(-12_i8),(-33_i8),(-37_i8),126_i8,0_i8,20_i8];
_11 = [(-154101641344298821873093725425627035835_i128),47183065527971262673503111557803747258_i128,152918702339210878324398669505645988257_i128,106523017864857984566309500533347138816_i128,119385729655481782141091847649998083642_i128,18642817724892060413223437485376960697_i128,88521022180485844744964283189598926435_i128,140646414181440838892972928476449699062_i128];
_20.1 = [1668649097491699947_u64,4122651851730294354_u64];
_23 = [336942529304220144074494891826012984398_u128,51576169174902378414194457177027555013_u128,186891249601456324564212616229556779431_u128,187132109590070657082271383816930239352_u128,328594820267200049295441898015166024917_u128,44721645121946640317053150387366690004_u128];
_21 = _20.0;
_19 = core::ptr::addr_of_mut!(_23);
_7 = [73783606917736952818334939417641518613_i128,102890192428503678722205174229238653978_i128,46018664715242394859055085781283139930_i128,82951551255073750259680581381705321286_i128,(-98026647383473905534490388097525470716_i128),(-65757142998841615070793077076648126850_i128),(-166731826478538898642900543727219601298_i128),125058538871400727329407916310342313619_i128];
_20.0 = -_18;
_12 = [(-71132472743841383500977064914271173106_i128),119707526330868804601462593900387002796_i128,(-122818338507203436677424637691350824956_i128),(-35755204293161571359308254455430991564_i128),(-88643679412222012395718524096940868017_i128),57617599855973332978344226096311146001_i128,(-168125152071840177618993989314300697218_i128),110136794782008039289496818890361185570_i128];
_15 = _25;
Goto(bb4)
}
bb4 = {
_14 = [(-167883430699582202532728076721640768476_i128),(-146846061094687121793438360953579373389_i128),(-152919977234965823282722618500828652463_i128),(-24604183686642949022397934810105792064_i128),149753685775255279952490200774898487971_i128,87853821465019881610366897843334169889_i128,75419723454243066332213872884146752974_i128,(-154541411203122558613291159513753717382_i128)];
(*_19) = [45216831455930784077823284652226394271_u128,153310501130250870988768724170627590099_u128,170281712152404756119198003459170983307_u128,226277855397425069641041717357982607185_u128,560396362928960107336001460771740947_u128,119349742185686706835178758070036159496_u128];
RET = 121_i8 as i64;
Call(_14 = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = _10;
_14 = [67916857978855766484551939168817301037_i128,(-99620974805441213536021055110415431064_i128),(-82352676049595526206174742321281094666_i128),128680932250145921573505585322662790169_i128,155840389139883126446622468722114260507_i128,164638431106559133268023636720502518013_i128,87184754432596658582007626586228019850_i128,(-136217248285996889966470547317068835951_i128)];
_17 = !false;
(*_19) = [327123623036540712935658634612007548305_u128,193136398194580805458365935569290480897_u128,250473495998555036214127092588118319617_u128,116931274391915837486718032613136915893_u128,138827846645578479723482770124544533199_u128,191675241040398843293313695227221787109_u128];
_5 = [115444311427306370176122377632974851440_i128,14856063036967944642440199924886869729_i128,(-59200105120145862624067952814499730310_i128),157747199577963286882523933162729995539_i128,95542847628192588380495958741851521120_i128,(-107162318003953284817715293718314433325_i128),132495995488371275743201519338765928501_i128,96299756704100052050613927365420785264_i128];
_15 = _1;
_11 = _25;
_29.0 = [(-6_i8),(-44_i8),(-6_i8),106_i8,(-123_i8),(-75_i8),(-110_i8)];
_29.2 = [(-148498951011241621016902756437903476891_i128),6332147937541057610673835091220730282_i128,82495229591117035964450422862421677872_i128,99358964682407983117633131413760732637_i128,(-421290534392829706746687338235884467_i128),43307645235009182425086664458244271638_i128,(-165630606228005431234654684695867398176_i128),(-130263397416377432629698378719161082375_i128)];
_2 = _11;
_27 = [20510_i16,(-28606_i16),1591_i16];
_26 = '\u{f8353}' as isize;
_2 = [139944521518389730222707007534739443543_i128,99650929620674957199520254908981170984_i128,(-50858014700451421862697714248804350255_i128),102918378927103879707491475261143341882_i128,67171514168866138128577385869372709979_i128,131839429685961542744167889941619665079_i128,(-144653921710433431106512076115420084160_i128),162217451688331522370026794421388127479_i128];
_17 = true ^ false;
_18 = _21;
_28 = _13;
_29.1 = 127636649466647184375182145355750797860_i128 as f32;
_18 = _26 + _21;
_2 = [25978404875577948302471158961069260841_i128,27023480866926030522813308481743634371_i128,2672690268690202267199235436090333492_i128,(-46421825285649865437365450623565758416_i128),(-71332629614687695786158991202858779025_i128),(-100957210462411249322505569272193971460_i128),119225205506453550003350250636140043074_i128,(-61625609016423982084322478783790770955_i128)];
_25 = [(-62136966688387271514544518708263996650_i128),128246542971947086227003074049913786154_i128,(-93065597104404129613151047030100641765_i128),(-107838702326983016686818852793676013183_i128),(-100423908161323563745715272811365707387_i128),(-79412102185494290234743381510125725034_i128),58288650945632098413147470947753567843_i128,(-51835414995487722861648005444402150366_i128)];
_30 = _26;
_31 = 2275239689_u32 << _30;
_20.1 = [13051471369993008664_u64,8461306845551247243_u64];
(*_19) = [37485747295945488269178741736428392780_u128,192186176068524215830136397695026116653_u128,243343386780694910514742321655793192591_u128,242162950107866487336237103784377696342_u128,20043940505548945524675550618830468563_u128,53878312662668062104943710566401311171_u128];
_3 = _29.0;
Goto(bb6)
}
bb6 = {
_34 = !_17;
_3 = [74_i8,(-102_i8),101_i8,104_i8,(-124_i8),(-47_i8),(-62_i8)];
_16 = _3;
_32 = _18;
_28 = [26244_i16,9080_i16,14117_i16,(-25721_i16),2781_i16,18121_i16,8184_i16,(-26398_i16)];
_15 = _12;
_5 = [(-10634669862210158723015000970750697459_i128),(-31714251504434698518686075738011199992_i128),90391356170250761092669711891083430140_i128,(-72809889445259056227193417847748592373_i128),(-153335705385286203706843168740686761026_i128),119837314941179160447367721472508746000_i128,(-89567693582567665655260955250074140471_i128),92480456748406897772006681655546434726_i128];
_22 = _29.1 - _29.1;
_29.2 = [(-110186397277532554944937793664613123875_i128),(-119451177027147646512830946547609929953_i128),(-101825984398519137938778289732511501801_i128),(-156329136583392280342815933539883613893_i128),(-9237693588416063733185373801206790806_i128),(-128563188260569596973654812233652231619_i128),72560330130014303327770314909881355423_i128,9290293613422031012910584374192174772_i128];
_30 = RET as isize;
Goto(bb7)
}
bb7 = {
_17 = !_34;
_31 = !3437121004_u32;
_4 = [100_i8,31_i8,(-105_i8),(-52_i8),8_i8,(-49_i8),4_i8];
_19 = core::ptr::addr_of_mut!(_23);
_15 = _9;
_29 = (_4, _22, _7);
_35 = 60070_u16;
_21 = _32;
_23 = [279521320246221160608018262953315754573_u128,208418245242164144892282276236051882063_u128,143950418803896565187338774648993611565_u128,87669397099400400470200284406413920173_u128,40906536919042566546644745226242678208_u128,111131612181393358705978449449770720392_u128];
_3 = _10;
_4 = [73_i8,(-35_i8),76_i8,104_i8,(-94_i8),(-41_i8),(-27_i8)];
RET = 137927293787111058780370609758019980345_i128 as i64;
_33 = 16139382812875383199_u64 as f64;
_35 = 64580_u16;
_31 = (-1916480282_i32) as u32;
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
_4 = _16;
_20.0 = _32 * _18;
_35 = 52492_u16;
_7 = [(-93532726568866170099518358300826086323_i128),(-9063260206624590472157134089735152932_i128),144085543453764102753394288519230833431_i128,(-168495197645939826056656723173495965256_i128),50016844615510740448394978827035812920_i128,80506222733241980039107051589428632254_i128,86100768461629679644671385545001438588_i128,36376130546065030248610697506590489602_i128];
_2 = [(-79403339812875209398074869248713482571_i128),92377287891855613929596390982575056953_i128,108532905950297531922438505624518446124_i128,(-14265059419750143623745907441158929262_i128),(-113993880102864735936574787605796990009_i128),(-42853451088850169223331099478213021543_i128),(-116319025152023533521350822698316616438_i128),6377343414687255485382980571391017723_i128];
_27 = [(-21689_i16),(-3521_i16),28549_i16];
_38.fld0 = core::ptr::addr_of!((*_19));
_12 = [(-107511736951241692113248480878510232252_i128),136702648808247283617832548437338340367_i128,72237399773059325810561409511549948602_i128,(-66063030715733316866053778447261810654_i128),(-143047784820546072671366766360770282042_i128),41385515477471221838733627415554604680_i128,131595866300546804071591525486988577886_i128,54923987455724753403694543565873037446_i128];
_29.0 = [39_i8,(-118_i8),57_i8,113_i8,5_i8,121_i8,59_i8];
_20.0 = _21;
_41 = -_29.1;
Goto(bb10)
}
bb10 = {
_14 = _9;
_22 = _29.1;
_41 = _29.1 + _29.1;
_29.1 = _41 + _41;
_27 = [840_i16,17495_i16,9883_i16];
_3 = [(-67_i8),74_i8,(-92_i8),(-60_i8),127_i8,(-76_i8),81_i8];
_38.fld2 = 9525708588142411821_usize + 5249437460448910042_usize;
_31 = 2497756069_u32 << _18;
_43.0 = [18_i8,(-126_i8),(-87_i8),26_i8,(-36_i8),(-97_i8),(-59_i8)];
_18 = -_32;
_35 = 52351_u16;
_13 = [13821_i16,24330_i16,(-19088_i16),(-15183_i16),7458_i16,(-13688_i16),24273_i16,(-17671_i16)];
_38.fld0 = core::ptr::addr_of!(_23);
match _35 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb8,
52351 => bb11,
_ => bb5
}
}
bb11 = {
_4 = _29.0;
_45 = _20.0;
_7 = [162234637080001669684145684775055817508_i128,40175663560169669239431154434261611935_i128,98421443753367637672348436822056701849_i128,(-77541202960250913636951586508557106683_i128),41691827211173103958239872569268496115_i128,65853889224702870137197830433428753790_i128,(-65772246507658204636650277943868345355_i128),(-71932315216090076748959388305207762685_i128)];
_43.2 = [31988643803621779065390155832138226647_i128,11704772916895433106448107664950584893_i128,29547019022408639548106763678627982251_i128,21891394314657754983841223071251035973_i128,(-71244158687953173166354364383665239192_i128),269976722560512446971650228220636065_i128,11658502945076463534812315698029293684_i128,(-76976857710631799683267320623013672971_i128)];
_43.2 = _5;
_37 = _31 as i16;
_6 = _3;
(*_19) = [284040936784381019985473997221598684997_u128,324424772716474846436773057648344005589_u128,71077798826425207833264182530661737542_u128,164854636238984741988381693604665075685_u128,202084606424883716321674483066991591277_u128,283775844215363517874331432661620954547_u128];
_15 = _5;
_16 = [65_i8,93_i8,(-62_i8),90_i8,24_i8,97_i8,(-119_i8)];
_35 = 49306_u16 & 40679_u16;
_47 = _20.1;
_1 = [(-137933983781207360829502544619946188954_i128),(-41481604347409642489052341074945876339_i128),(-146219387215014671478106638298016511901_i128),(-87280046754489290858827410306636304488_i128),138097541590038849603189466151196061849_i128,(-119468289667242882982731699305358553540_i128),(-95291513139051524579463314969231403276_i128),(-107365635598312095726008756371064920228_i128)];
_27 = [_37,_37,_37];
_43 = (_8, _41, _11);
_42 = [153170286281816160699211017047877164585_i128,(-104293683181227926009582945490290073977_i128),161921281385864530139787678562735949223_i128,148584459873966925992088245661384310982_i128,157480874011971689164567444029951947603_i128,(-93706504025592247861042466721220870272_i128),66490370221841046826677159321287477390_i128,92999086148317833950233478038616781276_i128];
Goto(bb12)
}
bb12 = {
_51 = (_37, _27);
_27 = [_37,_37,_51.0];
_47 = [3011665757494295319_u64,16165497662725067354_u64];
_45 = !_18;
_38.fld1 = Adt42::Variant2 { fld0: _38.fld0,fld1: 3066514283923345159_u64,fld2: _7 };
_21 = !_45;
_25 = [(-82736464544566162794106387324298107991_i128),(-165512461017823607412845616850051363273_i128),10283995077339882504992838176635259432_i128,(-57728498274659472195511556649446265219_i128),(-34563442759559204792124208942064802377_i128),(-164442133860485239797513091161450589851_i128),82151371218697619988436165932080467890_i128,32951138677127749624638493523247109812_i128];
Goto(bb13)
}
bb13 = {
_40 = 3466026872333098540_u64;
_43 = (_16, _29.1, _11);
place!(Field::<[i128; 8]>(Variant(_38.fld1, 2), 2)) = [21823117445801228105788167170149024708_i128,(-152174609761232290583309017999099572734_i128),43545190206280807050961311213012976092_i128,(-11595591077056608137139949531736653136_i128),(-134513471314915765701574232894848763041_i128),9792328455616479378609171871732494704_i128,(-8268900654302624923412791120205778206_i128),(-64903459194748179032969044391790648901_i128)];
_48 = core::ptr::addr_of!(_19);
_20.0 = _45;
place!(Field::<*const [u128; 6]>(Variant(_38.fld1, 2), 0)) = core::ptr::addr_of!(_23);
_57.1 = [_40,_40];
_29.1 = _41;
Goto(bb14)
}
bb14 = {
_26 = '\u{fbb34}' as isize;
(*_19) = [261844012815403056836236085193184449943_u128,27881463322494866032174806203224693837_u128,100174266189288892508460020894037628205_u128,130840108234498120711517638934636205128_u128,316930523722820407056397126150100762300_u128,113640634303910532237358086418584358729_u128];
_39 = [_51.0,_51.0,_51.0];
_23 = [319484015270208369634218264203712901161_u128,2734537537676622580671936316345572362_u128,130811012456362396398333323060748580960_u128,164294172120385681657207469913654729837_u128,331218304708631253651065142045385826546_u128,20477918490797708348134530847715529374_u128];
_37 = _40 as i16;
_16 = [(-19_i8),4_i8,(-19_i8),90_i8,66_i8,(-80_i8),28_i8];
_31 = !2323959811_u32;
_56 = !_38.fld2;
_46 = _56;
place!(Field::<u64>(Variant(_38.fld1, 2), 1)) = !_40;
(*_48) = core::ptr::addr_of_mut!(_23);
_57.0 = _26;
SetDiscriminant(_38.fld1, 2);
_56 = _46 - _38.fld2;
_5 = _7;
_1 = _2;
place!(Field::<u64>(Variant(_38.fld1, 2), 1)) = _40;
_26 = _20.0 - _32;
place!(Field::<u64>(Variant(_38.fld1, 2), 1)) = _40 - _40;
(*_19) = [54895548339285451281250270552923773976_u128,254613028424072262739221819804257415618_u128,296660691272191217647741112767333833276_u128,216706384510462192586166793346770688503_u128,196384707909287649482003855612592111140_u128,82767826749796502402081039383225140502_u128];
Goto(bb15)
}
bb15 = {
Call(_60 = dump_var(9_usize, 23_usize, Move(_23), 45_usize, Move(_45), 42_usize, Move(_42), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_60 = dump_var(9_usize, 1_usize, Move(_1), 3_usize, Move(_3), 20_usize, Move(_20), 40_usize, Move(_40)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_60 = dump_var(9_usize, 26_usize, Move(_26), 17_usize, Move(_17), 13_usize, Move(_13), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_60 = dump_var(9_usize, 27_usize, Move(_27), 32_usize, Move(_32), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_60 = dump_var(9_usize, 37_usize, Move(_37), 28_usize, Move(_28), 21_usize, Move(_21), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i128; 8],mut _2: [i8; 7],mut _3: [i8; 7],mut _4: [i128; 8],mut _5: i64,mut _6: [i8; 7],mut _7: [i128; 8],mut _8: [i8; 7],mut _9: [i8; 7],mut _10: [i128; 8],mut _11: [i128; 8],mut _12: [i128; 8],mut _13: [i128; 8],mut _14: [i128; 8],mut _15: [i128; 8],mut _16: [i128; 8]) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _17: [i16; 8];
let _18: i32;
let _19: f32;
let _20: ([u128; 6], &'static u8);
let _21: char;
let _22: [u128; 6];
let _23: *const *mut [u128; 6];
let _24: u8;
let _25: f64;
let _26: u32;
let _27: u8;
let _28: (isize, [u64; 2]);
let _29: char;
let _30: *const *mut [u128; 6];
let _31: isize;
let _32: [u64; 2];
let _33: ([i8; 7], (u32,));
let _34: isize;
let _35: [u128; 6];
let _36: char;
let _37: i32;
let _38: Adt43;
let _39: (i16, usize, [u64; 2], i16);
let _40: [u64; 8];
let _41: i32;
let _42: (u32,);
let _43: char;
let _44: isize;
let _45: (i16, usize, [u64; 2], i16);
let _46: ([i8; 7], (u32,));
let _47: u16;
let _48: ();
let _49: ();
{
RET = [(-12854684034962782389150622236095023959_i128),79301609984348785611388570485363168151_i128,19442970726731628012649082590220139499_i128,159038540904258020000094737563102332868_i128,104038549841603209154019862817896624966_i128,(-87366083216942599685128482370620812483_i128),(-166667796410830818630959998896466675602_i128),(-69467828851033844502619265071697435392_i128)];
_16 = [(-36214923542386013671563658016234800289_i128),(-26490689510278624058129778838620382885_i128),66422344053824922214755062389870612335_i128,(-135206439368663280842747319539777543024_i128),33800229820264590108006374523142592605_i128,(-58730641536885210512880234482798770281_i128),(-47604302095780548124249203315739913865_i128),96168576444105620283119658647891152374_i128];
_2 = [96_i8,(-34_i8),71_i8,88_i8,47_i8,(-15_i8),36_i8];
_4 = [(-45405770788408238178203518618002143953_i128),48780947968116774048654633372809902053_i128,(-92008248669486670708245905704564187650_i128),90594780321947325741320080196603735651_i128,(-104998638553414629273319106588593837051_i128),(-64597403063598279569151509899377479809_i128),152076069932501852073066530643348310613_i128,79385131539603727033753525389503703366_i128];
_14 = _12;
_5 = 7480810496177020838_i64;
_13 = _12;
_16 = _14;
_4 = [105289649364532386404835052502160481457_i128,98324778016674128541939543990725577187_i128,(-7545341512147437346963066940187146239_i128),(-27317091451162080492628874306205782975_i128),138838377645246578776153636217095140522_i128,150633706664111681286585929560235726187_i128,94336206545437202636703855391437724215_i128,136033150606803213637220902066654638734_i128];
_8 = [89_i8,(-2_i8),112_i8,(-74_i8),116_i8,35_i8,(-49_i8)];
_1 = [138996125735148499878906506389355783889_i128,(-141161529402266345552458493373112024144_i128),85658621553394382245513547152256660472_i128,(-3849182594043953457353743670759145404_i128),151367175161612546470492794380666766857_i128,160355066585823282383327186083637372283_i128,(-94779886063993951541157860424533188766_i128),(-110797177778267232837630377463966184375_i128)];
_14 = [131211840785769652084217905699400837126_i128,(-160647566833796789305499059790914280756_i128),109313198488280950506309152480259718814_i128,(-113764963663132327481523343522168485606_i128),(-88535234508507234727312720144606419138_i128),98394387488472481125120452235776689798_i128,122377446657351432747430252579826428285_i128,123710579198788438258330484278556469067_i128];
_15 = [(-56630880783564038666771544247954479466_i128),133526611532238098348077168351255888291_i128,(-144195176186365297443550687675874921283_i128),90084413867549330584734255284067365888_i128,(-169088503455417677320822225096747980294_i128),117685852359662499761520447837310327758_i128,(-95365254739602429964805055082101641442_i128),165977577903199678859822543634892779_i128];
_14 = _15;
_15 = [(-167630235219060329392973558410982996597_i128),4140098964984530978574568853462021292_i128,(-75935682879790861876284108341207967111_i128),(-23107044168086916991279185330005054596_i128),144007610942429369021507999044134569360_i128,(-91259273434730364831054279239569165249_i128),(-126304144013790068919502635648785950305_i128),(-78022776352741698445252417299870912309_i128)];
_4 = [(-27973824226211460968688817887742133145_i128),(-131707286925675996838368687889425609738_i128),159184862278042948254746337524678727494_i128,(-51943514182045063390571622861080126798_i128),165384609054264358905985079656143734177_i128,96662727291087221679299008489675638960_i128,(-17778939484472312094726435010691858715_i128),702894797620241812170732921270729806_i128];
_11 = [131058577033995908634718530384382433183_i128,92177857284623566404310968526576084372_i128,(-17175831849238708985007945622088429912_i128),(-169960126447587989219140539461991122808_i128),(-12989513651190506968340627401479080283_i128),52334165640423854366611612537553836992_i128,(-102388845318535454748407241573640529415_i128),14602521301925849909988243547042340703_i128];
_9 = _8;
_11 = [(-42878257252429723042147666613206780106_i128),(-155868874787130371523287779014334777631_i128),(-54272547826165892015089784999649556397_i128),(-137529124174623501078691734229368356356_i128),149139699471697088482460350082253130511_i128,(-163459516668029336224209996290279349522_i128),22934679994931858974518288566250654446_i128,(-8283219257186734997648532393722039521_i128)];
_15 = _14;
RET = _15;
_5 = (-7819387369722147306_i64);
_14 = [23232314278863511308143128862263724227_i128,120595945363238939600834595227344787839_i128,(-124792630148398656615480811217410469508_i128),(-126921358801120712033841537992784869035_i128),(-63551779970395229660949779817294971404_i128),123048479041147812396873364655861509827_i128,97835196664979826955044459787149592866_i128,(-137545909661537018150701255673845103236_i128)];
_16 = _12;
RET = _12;
_9 = [(-56_i8),(-64_i8),(-36_i8),125_i8,97_i8,82_i8,106_i8];
_6 = [126_i8,(-7_i8),2_i8,44_i8,(-40_i8),(-30_i8),(-8_i8)];
_16 = [151969267184795757977259815339614238816_i128,(-79377515169789650467373731804318834843_i128),(-31846302164159956462904040149051593862_i128),(-81341917698451011704226744739319513152_i128),151775301005821892990957821688333511985_i128,(-22478267587938474614564789796493229278_i128),(-165478998751015008747049948446996580787_i128),67147677861601799097098733275085906424_i128];
Goto(bb1)
}
bb1 = {
_12 = _7;
_6 = _9;
_12 = _7;
_4 = [(-60104642623125131650010206367424857161_i128),(-31606534483732900278116658508689817670_i128),123252140124536940061788621698685497605_i128,(-40111229371388150634661253456017260486_i128),(-130598469824474393909884682643344067196_i128),(-28276568172385473144073297554819940759_i128),(-104013118790688258388748734947375215259_i128),(-8386994985336103108069855472541057109_i128)];
RET = [(-148654874336237576625752517327170923945_i128),50874277717885916129405896940946730905_i128,60735416316650977654694798501599585956_i128,51605451168772516849572512941283245935_i128,140738968270316642489691667908968554156_i128,120016067240377568901116695484854138120_i128,109227898673984009103651683888967197485_i128,(-156577068848869237215127770337844917153_i128)];
_1 = _16;
_16 = _12;
_14 = [142857643133166900670774687733638957906_i128,72409234609324350200528479912003836527_i128,(-100637646490778356318342926889856200536_i128),128338025254011142225270323103225487538_i128,(-59373909151841530767597547011258268715_i128),53661764231962543542753997342898165463_i128,(-107569536485349055690353762780260340896_i128),(-131845796335994113074651954065499091809_i128)];
_13 = _10;
_14 = _10;
_9 = [(-8_i8),84_i8,(-94_i8),(-14_i8),122_i8,(-128_i8),(-50_i8)];
_3 = [(-66_i8),(-81_i8),(-1_i8),(-103_i8),62_i8,(-55_i8),(-30_i8)];
_7 = _13;
_2 = _3;
_4 = [138806773811168554476647120818848045948_i128,(-10669588989427516270271586653200312764_i128),147450050461772052676980398135373199965_i128,101574207990716225445496423564816316568_i128,143686217187204716611511200726134754227_i128,108481341388950131423800593107634253322_i128,(-116158221910805729554944779526583139080_i128),141277145567560359246361560447944955703_i128];
_15 = [(-154410004906747911291214711509790162015_i128),114904705573487810611787644706851276316_i128,(-39600257905743051436677592885925763000_i128),44252841750079777243725908138993910343_i128,4214976973696119657427139112082587973_i128,6137808169913271430272610782096529464_i128,16708636801504501982435083987826583356_i128,24520233136016504962306996697066953198_i128];
_14 = _11;
_3 = [19_i8,47_i8,83_i8,24_i8,75_i8,(-110_i8),98_i8];
_9 = _8;
_6 = _9;
_16 = [33877072540193865150579646010280281674_i128,(-83179677302439463220555597052181030669_i128),(-136228935458056122087607447836291144712_i128),(-132698116136809355767141604288262686069_i128),(-99544404827211220813463038843707474555_i128),(-108197198833188234092826629119496526984_i128),131031682089607697540576234732372366690_i128,51015305877362524458360538454433571133_i128];
RET = _10;
_13 = [(-18960117158312959419916858675086335185_i128),23302787669403869735294163597387540983_i128,(-69583825612011460013201876526984331854_i128),127556112560751641810374052082647312382_i128,107011571957560110773249252941029089067_i128,45989518321807040638571455037180899415_i128,90113535276896428290933317406446492925_i128,(-68954013171868956172634684160642073528_i128)];
_5 = 9191946267912248302_u64 as i64;
RET = [(-119464742926657083500931488733834907424_i128),(-55075572922320333223193655270249301756_i128),165723881302237676456233029573293678891_i128,15445423183310227782574465483626599634_i128,77007982750986949153360665967583246629_i128,(-48022017935318174800369137731601826646_i128),83413565322893600750644521261991758699_i128,1803248645402281538885340261769826107_i128];
_5 = (-9223372036854775808_isize) as i64;
_14 = [85832248675298075729112956575742103047_i128,(-111730827252732459957362825042672181253_i128),106159005886060147241166394102963940766_i128,18177945195528801390400572469027126647_i128,15871373047371388912450022742982620634_i128,(-143408471110213433818527576949980703328_i128),(-63564173618924071098132554855018927426_i128),80930022093282545936171960973819837359_i128];
_12 = _13;
RET = _14;
_11 = _4;
_15 = [(-68257309293972659192485539581391821335_i128),(-108659065141822391803173612755823963573_i128),(-86584886625309424544428252918903733848_i128),116421470525293260194298734159658609102_i128,(-91509028415184889600091922447381258564_i128),104072882495403566502052677077323897467_i128,(-40026639910391035922098696156924406329_i128),(-35366615483899785310949313532097097094_i128)];
Call(_15 = fn11(_6, _8, _3, _16, _8, _9, _10, _16, _4, _12, _14, RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 4981141422456016144_i64 >> 252132731488832698725828633320158821056_u128;
_1 = _13;
_16 = [9448936439279997966831753002431687602_i128,52188160456348972782403828304853143776_i128,88050610887509467583219467144817299325_i128,11320088200530962355854050783312349209_i128,13840903616396222076608448680656228124_i128,36668884240641281239975134904319144654_i128,(-85230365840462352566266705919217236550_i128),(-165502294440718620774325860278295995037_i128)];
_6 = [23_i8,(-101_i8),106_i8,14_i8,4_i8,114_i8,(-42_i8)];
_13 = [50020707083615552593263466110881130229_i128,8525634870427854037433161060500899319_i128,(-26316971392692582894022532205548501230_i128),163880986831900148206095313614147988025_i128,94662990334635825251610809923582914987_i128,131047747009026463241231514038945031833_i128,43028132883317025902130382736485385876_i128,(-151835317710185470007163485283998970770_i128)];
_5 = !(-5180757357436845823_i64);
_18 = 753193065_i32;
_2 = [71_i8,13_i8,(-14_i8),80_i8,9_i8,(-13_i8),(-125_i8)];
_8 = _9;
_18 = -(-184728830_i32);
_17 = [(-7542_i16),3983_i16,31006_i16,17422_i16,7915_i16,19376_i16,(-28786_i16),(-15425_i16)];
_7 = [(-24555245916219418097009886782058034532_i128),(-154834668571686584157103270236076578384_i128),5441742703547552524059266336919587478_i128,9013050275771538895901087809263025878_i128,(-158985054577416137955034463135248607534_i128),79413933814277724842124856179770267560_i128,37368693579086096970254601295692552379_i128,(-65974839644785799551752751242186107988_i128)];
_2 = [(-13_i8),115_i8,(-122_i8),95_i8,1_i8,54_i8,(-80_i8)];
_5 = (-128_i8) as i64;
_19 = 1876397552845852102_u64 as f32;
_15 = RET;
_20.0 = [308317182882876047747273038614458799366_u128,20115806513194337347898877351575515899_u128,42063531764118148338168956040265904771_u128,62151849369452806430353927302455165231_u128,60998534710850584249241185423798476545_u128,317213392239308817057593232170277101884_u128];
_1 = [(-69019922039163310127407303992829270803_i128),(-48828996924045268371069034317549768063_i128),(-20897702342993639338411682357818420526_i128),(-140159892380351279057409570673281288202_i128),(-163568602632571278262827881000624063196_i128),(-157754694763234868284980304597237720997_i128),(-24568470894337703756883223917484056140_i128),133542966361965269577250354645959745028_i128];
_5 = !1888406597386879552_i64;
_4 = [(-139628553928119584753644768368273672401_i128),45208418867867513600809745118766965365_i128,(-98118588846825588624133591784762610046_i128),(-139792102706042974795052154514150086656_i128),(-86598122026606775066916890448799329414_i128),(-157254120560562753547657613138983519247_i128),152182078890082050056579145011315841523_i128,88326349010862276746373368440537688878_i128];
Call(_14 = core::intrinsics::transmute(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = [24722818012378422661672466245971922471_i128,81523227072912691843139840310989310691_i128,73345119983707870125525864649878825635_i128,61459663610390421167821837474836204560_i128,(-110977105497606721256131645965341876700_i128),(-31854041112371751592531671803900094850_i128),(-144926025104452621749011866494338678122_i128),5533402018863921768651714928616903180_i128];
_9 = [(-119_i8),(-90_i8),(-23_i8),61_i8,52_i8,45_i8,80_i8];
_5 = !(-1476904795049488917_i64);
_24 = !36_u8;
_7 = [(-73567581115755058166640627677378065942_i128),(-113309947584641436948038089217633774917_i128),(-50839472767575763071209393956459466797_i128),7748099238115099673594313888752536857_i128,(-74610114374373451911182271143030984221_i128),54103342652477969954965013907842384074_i128,(-6589998712738149991763961934158602053_i128),55730519898047385509603019488435779545_i128];
Goto(bb4)
}
bb4 = {
Goto(bb5)
}
bb5 = {
_6 = [(-105_i8),18_i8,31_i8,72_i8,(-108_i8),(-61_i8),76_i8];
_19 = 37603_u16 as f32;
_19 = 244834106835727585639367095074299738072_u128 as f32;
_5 = !(-7295839073217559414_i64);
Goto(bb6)
}
bb6 = {
_21 = '\u{c0757}';
RET = _13;
_20.0 = [39033992273846548605136184360412720386_u128,267068296607107198421028594556441477441_u128,5062164752616508851704549930456121395_u128,157174286023990418755118634169590606253_u128,131815204788166454438776682755710463798_u128,25839289353738047436701449847438886361_u128];
_15 = [(-60960039712524563589567251607345355633_i128),(-104823372665685172890444702577500548709_i128),32590616019235919188607875832780592371_i128,(-105807721744144281099296552407372208533_i128),(-82571611046759730053111397278208535526_i128),85289257517802245419896088055457721995_i128,43911460389704209306439498976277798811_i128,(-107881010283992429105525936688248472118_i128)];
_22 = [181741030691739019450955013754720637311_u128,127509571916984306089620375656249130641_u128,106311150958396172280245026856017232417_u128,321876681086166469287879156808361617847_u128,197127955258424267685770697620732371252_u128,58918679878902776387611759991648549820_u128];
RET = _13;
_24 = 68236107918470335618422962769131079712_i128 as u8;
_31 = 10372188264795820968_u64 as isize;
_31 = -(-82_isize);
_2 = [2_i8,4_i8,40_i8,(-105_i8),(-31_i8),35_i8,50_i8];
_12 = RET;
_29 = _21;
_13 = _7;
Goto(bb7)
}
bb7 = {
_9 = [21_i8,1_i8,(-84_i8),(-65_i8),(-57_i8),(-96_i8),(-121_i8)];
_27 = _24 | _24;
_16 = [(-91750972457532286190343763180951349702_i128),100737133064622085504740334166355675326_i128,92607494925712727116777990144890602334_i128,(-131654003196093110154969905401624222753_i128),107362218272480415084328946249743917241_i128,78228772434048106334727902153551349231_i128,122899351995505562434432814909707280293_i128,138885779438640259741540112125170792875_i128];
_19 = 819248809697354720_u64 as f32;
_21 = _29;
_12 = _4;
_11 = [68246484282398364629182008425159361837_i128,(-104951980030430668655596688897434596103_i128),29098676081620232420195629806276900626_i128,(-143996473057578256197383673245321695132_i128),(-108905023965881455443778287331800929681_i128),15491662688218363419399086567714811035_i128,(-158640523358420397728554205942831139837_i128),95431061751855000389360761304368458398_i128];
_29 = _21;
_32 = [3951975828458817455_u64,5934774757360286073_u64];
_34 = _31 & _31;
_24 = !_27;
RET = [87104031075865112009716984906018101380_i128,30768984628984679306994498721227340557_i128,(-163460527973170026065936244962331925024_i128),55365797040754232644561673544948870705_i128,100302268482693116033918493684770455379_i128,(-166159521763958817685214518572712191084_i128),35679499236188111914228350639178649472_i128,(-108556822401276991539797153197429937922_i128)];
_29 = _21;
_25 = 19673_u16 as f64;
_20.0 = [154730876471831456664431269339562860256_u128,236080840307291393293423930634954804244_u128,103124865912233277849554260868575897723_u128,198211350686666435704511196669122639834_u128,72425961183289710980751841014721407877_u128,339566017383621646773539757245365411457_u128];
_28.0 = -_34;
_28.0 = _34;
_5 = (-5693838401887064451_i64) + (-3879155908602277110_i64);
_33.1 = (3219117872_u32,);
_2 = _8;
Goto(bb8)
}
bb8 = {
_15 = RET;
_9 = [(-71_i8),43_i8,69_i8,(-90_i8),84_i8,(-40_i8),36_i8];
RET = _4;
_28.1 = [3427104288069351436_u64,11458148842062765920_u64];
_21 = _29;
_19 = 142978228999139616020277194659999069482_i128 as f32;
_20.1 = &_24;
_33.1 = (2058527360_u32,);
_18 = 9376331852065543351_u64 as i32;
_35 = [50334496271708386539690348653125579231_u128,115106396669244173796315077192978611994_u128,218997693956840848218238360888468074144_u128,137186669248775631415838111188256470688_u128,81844628038958421375924206317884884917_u128,171320677023932599301135628707024578554_u128];
_3 = _8;
_36 = _21;
_33.0 = _2;
_15 = [(-162583954831714213957459804871851087773_i128),38771309935282341053401123893503636021_i128,(-119568962713170706844757918725006044584_i128),2994890125933008554877532222090476765_i128,34768399795565104509045088161029719544_i128,142060695335505214470658256731128457005_i128,(-23846064819260887503778977570045694005_i128),126125036469582669165596568514225959741_i128];
_28 = (_31, _32);
_31 = 22629_i16 as isize;
_32 = _28.1;
_33.1 = (2100109767_u32,);
_28.0 = _34 ^ _31;
_28.0 = _31 + _34;
_16 = [(-100889047885670742671180567082882611911_i128),33619846539544677289696492372202127655_i128,73397018120616321385458658366986417134_i128,52221388844558070734179269643573584355_i128,(-103010272627313931193686868102068829090_i128),79930969197712797171216919999513520901_i128,27191765857575724305444405698334303311_i128,38298947814340652985785135336677273373_i128];
_18 = !488929881_i32;
_37 = _18;
Goto(bb9)
}
bb9 = {
_17 = [(-7489_i16),(-31095_i16),(-27914_i16),29094_i16,15800_i16,28618_i16,21457_i16,16569_i16];
_14 = [(-85563091624425190929420384723213264331_i128),(-159373292504405513812054302789559505100_i128),59777698852064253634309944178032418519_i128,(-159822707814157117773691392778475200417_i128),(-125844625573073908328464207048549784442_i128),(-120310358425346759913039276295270532274_i128),147738313463806780098948340622400314993_i128,81097685891763292179569161726289534553_i128];
_11 = [(-39220348043559690939707046155857087140_i128),149496441341259006721918184569223091522_i128,160566972032039785742085690570325471810_i128,(-109116203036347552520270245734415119966_i128),(-156350509592232266226827584652199067231_i128),18781946536619745553015786152483646315_i128,(-156869301839272969958032102392485511040_i128),64053406790661960393936588666566946416_i128];
_13 = [47339135034967992192217743190263525821_i128,153168889804384191093631146179913838358_i128,(-138622245250217974253053621067509007361_i128),39384925765969287556361444815592209443_i128,36838884264824356221687489539276947346_i128,(-146190068808690340061634953730053390109_i128),(-101669296215865168448895130412019929964_i128),(-67978745994900545338218465302148099636_i128)];
_13 = [17411252454430359235514829901766435525_i128,(-97925197921282347438677815003391289508_i128),(-9454422719920710942538096252597281194_i128),20789823522777675076280965869058873082_i128,156307697889618399431460591314908160825_i128,(-41046844669623940031482704963110419403_i128),98273042344090175235898296997553390430_i128,(-87465478537848329420153991136792699565_i128)];
_9 = _33.0;
_28.1 = _32;
_20.0 = [129780423660807290729701513328948504928_u128,119798585673522834705890929078669616607_u128,80146969181552274915982440190791616766_u128,333695093465177546661009046072257119439_u128,290598758556180401946379668801091603462_u128,4890319274246847142878681344201323894_u128];
_39.1 = 3_usize * 16122706751583815888_usize;
_39.1 = 7_usize + 4541579072296189054_usize;
_33.1.0 = !2417109100_u32;
_31 = !_28.0;
_20.1 = &_24;
_42 = _33.1;
_37 = _18;
_42 = (_33.1.0,);
Goto(bb10)
}
bb10 = {
_24 = _27;
_14 = [(-31535588464974415069851102966236875698_i128),138711742170135295899001538849136897976_i128,121481990427622340784587104987575725114_i128,75660837210762060436078463412926437275_i128,89951845800672034996974796659826977262_i128,146349375771905224601882311597537047786_i128,69016319203363458570130689461150708899_i128,147072971689050583383060912104375583833_i128];
_1 = _7;
RET = _11;
_29 = _21;
_45.3 = (-14914_i16) | (-16578_i16);
_7 = [(-113586301826654741007392058429362337428_i128),(-93332726129361928845206133369435708936_i128),26662727279567418929136547232492713160_i128,(-2770787940553638505016909942171498469_i128),19898256412523267001709011095829953391_i128,157840831205855013069266058198043613745_i128,74692050549452562587923056896828337246_i128,150816402572864724609066984607253400476_i128];
_39 = (_45.3, 7_usize, _28.1, _45.3);
_44 = _34 & _31;
_29 = _36;
RET = [135968018217224146605233616245308223615_i128,85997526817413736808006748239455032287_i128,(-33917568926788741135339139490991643854_i128),(-95940068113499949414908869407033157776_i128),5701071405068701606066089089043226542_i128,(-158716450998890906365513414844638198572_i128),(-12172285571609391527418196634101709491_i128),6680057063973811685203141336724190996_i128];
match _39.1 {
0 => bb8,
1 => bb6,
2 => bb7,
3 => bb4,
7 => bb12,
_ => bb11
}
}
bb11 = {
_21 = '\u{c0757}';
RET = _13;
_20.0 = [39033992273846548605136184360412720386_u128,267068296607107198421028594556441477441_u128,5062164752616508851704549930456121395_u128,157174286023990418755118634169590606253_u128,131815204788166454438776682755710463798_u128,25839289353738047436701449847438886361_u128];
_15 = [(-60960039712524563589567251607345355633_i128),(-104823372665685172890444702577500548709_i128),32590616019235919188607875832780592371_i128,(-105807721744144281099296552407372208533_i128),(-82571611046759730053111397278208535526_i128),85289257517802245419896088055457721995_i128,43911460389704209306439498976277798811_i128,(-107881010283992429105525936688248472118_i128)];
_22 = [181741030691739019450955013754720637311_u128,127509571916984306089620375656249130641_u128,106311150958396172280245026856017232417_u128,321876681086166469287879156808361617847_u128,197127955258424267685770697620732371252_u128,58918679878902776387611759991648549820_u128];
RET = _13;
_24 = 68236107918470335618422962769131079712_i128 as u8;
_31 = 10372188264795820968_u64 as isize;
_31 = -(-82_isize);
_2 = [2_i8,4_i8,40_i8,(-105_i8),(-31_i8),35_i8,50_i8];
_12 = RET;
_29 = _21;
_13 = _7;
Goto(bb7)
}
bb12 = {
_37 = _18 + _18;
_45 = (_39.3, _39.1, _39.2, _39.0);
_8 = [2_i8,45_i8,(-103_i8),(-55_i8),3_i8,118_i8,71_i8];
_19 = _39.1 as f32;
_28.0 = _44 | _31;
_2 = [4_i8,30_i8,99_i8,(-93_i8),(-23_i8),73_i8,124_i8];
_43 = _21;
RET = [159726196946707472709003747631421191591_i128,65660446178099038037021916651807496898_i128,13837638277486652137087069739803345660_i128,(-97444775084074763730082110718546732006_i128),60315461963740912419529415566197012783_i128,82920513996914966362300168964549716296_i128,45544237082325788615775127303279829056_i128,(-87896620888062149768968475103087296641_i128)];
_24 = _27;
_45 = _39;
_3 = _2;
Goto(bb13)
}
bb13 = {
RET = _11;
_32 = [16084068823496834542_u64,15058381696682123014_u64];
_3 = _8;
_45.0 = _42.0 as i16;
_45.2 = [9855046043224796343_u64,15979737652143294595_u64];
_26 = 7459943474230224074_u64 as u32;
_15 = [23865149488499880922905739676494189839_i128,38258289606454021307743757789099747294_i128,44490413800491057818972661519202374168_i128,(-96802335263503761757435108405213812045_i128),28532178139038278095218480549802084760_i128,84444294836241631812101493803134006814_i128,166916131449054965871548496379502199336_i128,(-137602854024956881784541663145146666504_i128)];
_2 = [(-82_i8),(-100_i8),(-9_i8),(-71_i8),37_i8,(-73_i8),(-85_i8)];
_24 = _5 as u8;
_22 = [183578354980000971265775088919136616800_u128,22106265034245713566040042104383118474_u128,264095066905882773039591810620859703340_u128,208916279693029593039272516784063523937_u128,190046864251121215301724273872739092806_u128,44313429427329282698072352732943305455_u128];
match _39.1 {
7 => bb15,
_ => bb14
}
}
bb14 = {
Goto(bb5)
}
bb15 = {
_45 = (_39.3, _39.1, _28.1, _39.3);
_29 = _43;
_21 = _43;
_43 = _21;
_41 = !_37;
_19 = 5627_u16 as f32;
_20.1 = &_27;
_28 = (_44, _45.2);
_46.0 = [21_i8,49_i8,(-21_i8),(-100_i8),125_i8,(-127_i8),126_i8];
_32 = [8934017414861385475_u64,3166062266581011300_u64];
_45 = (_39.0, _39.1, _28.1, _39.3);
_39 = (_45.0, _45.1, _28.1, _45.0);
_33 = (_3, _42);
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(10_usize, 3_usize, Move(_3), 5_usize, Move(_5), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(10_usize, 22_usize, Move(_22), 8_usize, Move(_8), 28_usize, Move(_28), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(10_usize, 14_usize, Move(_14), 24_usize, Move(_24), 26_usize, Move(_26), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(10_usize, 7_usize, Move(_7), 44_usize, Move(_44), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(10_usize, 29_usize, Move(_29), 12_usize, Move(_12), 27_usize, Move(_27), 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i8; 7],mut _2: [i8; 7],mut _3: [i8; 7],mut _4: [i128; 8],mut _5: [i8; 7],mut _6: [i8; 7],mut _7: [i128; 8],mut _8: [i128; 8],mut _9: [i128; 8],mut _10: [i128; 8],mut _11: [i128; 8],mut _12: [i128; 8],mut _13: [i128; 8]) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _14: i64;
let _15: bool;
let _16: Adt47;
let _17: (u32,);
let _18: Adt51;
let _19: char;
let _20: ((i16, [i16; 3]), &'static u8, ([i8; 7], f32, [i128; 8]), ([i8; 7], (u32,)));
let _21: Adt38;
let _22: i16;
let _23: f32;
let _24: u64;
let _25: ([i8; 7], f32, [i128; 8]);
let _26: Adt40;
let _27: Adt52;
let _28: i8;
let _29: isize;
let _30: Adt45;
let _31: [u64; 8];
let _32: [i8; 7];
let _33: [u64; 8];
let _34: f64;
let _35: [u64; 2];
let _36: (i16, [i16; 3]);
let _37: u128;
let _38: u128;
let _39: *const [u128; 6];
let _40: ([i8; 7], (u32,));
let _41: ([i8; 7], f32, [i128; 8]);
let _42: char;
let _43: ();
let _44: ();
{
_8 = [(-113074630440079816630969689903194658287_i128),(-16127409987859726377476210523343250012_i128),91569814872913760141575777283775347226_i128,(-164211522946441658207994173517875673620_i128),141566192059161145952804948462114153696_i128,(-83954106088684708119642457783952191159_i128),61775059946195088314941291700322074972_i128,(-88874601970307191866029473178017412863_i128)];
_13 = [124335660403435837793252584655043802064_i128,(-85912766917650893141258898587455838648_i128),(-157794560799395693413611611857117613728_i128),166006515745509590948099377873421552418_i128,82583809157034723986369749543399613435_i128,(-124087827316520653152689410944626151823_i128),(-93149606575063466413344037344725980472_i128),91972622584908759133628087550704648023_i128];
_3 = [112_i8,14_i8,3_i8,36_i8,(-93_i8),(-58_i8),96_i8];
_10 = [158394627014682766238162789445269196961_i128,(-3943342282766551946414450946234225785_i128),127161448810350035226661663610046042529_i128,48528971352139924252597870301726427242_i128,107257025552532348972540044704572771486_i128,35435864970184499938986100681247630620_i128,(-64519323258633496443386087800920486944_i128),(-27376169382968009109907704159885905685_i128)];
RET = [21391009581744752030099275446789184937_i128,110166861433073469361649912481078150534_i128,117499830206010906006582657284002191775_i128,99078409782415072965832590373635295539_i128,10457461502155803645418919376134091274_i128,(-72491492685828339507872700172531784387_i128),80408114258322423705936733410601946474_i128,(-142567528147816798016471226639824147468_i128)];
_14 = 1341650576118928056_i64 & 3341225903306697704_i64;
_5 = [33_i8,12_i8,38_i8,(-93_i8),(-36_i8),(-44_i8),66_i8];
_14 = (-1899199063126593215_i64);
_11 = [(-6043599614693219747264423837611490763_i128),164146144110354344201956249905319918591_i128,79200953683629863810959660913510423372_i128,(-146796250502081421820814586159174385276_i128),(-168364098442793974799535252579848972013_i128),51341065650528889613095969707284979795_i128,(-33726021059439538255915622584868195583_i128),(-84839358323463859859131421111817484458_i128)];
_13 = [30273223104848223488968904049627061899_i128,(-144245263260250758575776134946611750042_i128),(-167493287370049405542516510320763837772_i128),43857809679699066133771242194882121839_i128,(-91210275007630457299039135508057433397_i128),45311103011955335552090206764313648865_i128,(-20199138475904754567570445216215870282_i128),65752169441746159979477189499549130244_i128];
_4 = [(-155518408493636916230048549743771822072_i128),8264256111963553441009552102637283461_i128,(-31105558195799428253355422384359253773_i128),30364749086234996276266658653190053621_i128,(-76676705206430043828618003729905227299_i128),43486323796459091423206022143003738508_i128,132414836770364077331003300174756834214_i128,5169292474825550784422402482139841283_i128];
RET = [134613166687587829302338038469303973000_i128,(-49907515957041089221206550949100254458_i128),(-40105483237495880792726940233952529777_i128),161157901089064882139526603353791386891_i128,(-115931964189522377639131024859724274703_i128),(-114261656600024728780990822370220630637_i128),120368936010086911807923752585034980556_i128,(-97111771796785094126466834496347098149_i128)];
_8 = [69046774639673646195288037077452766823_i128,66897791473017598442716287668732081151_i128,(-57346647536669552126649309737290475337_i128),14154258614484499633437409864104777877_i128,19704404199283793887091436134402609742_i128,(-129149223589209232657220128774679231050_i128),(-133433359702794973308832923327804487783_i128),(-70669684734811563953643238212441971004_i128)];
_4 = [(-155286420704926421437719024380402912566_i128),133177815360430083955404739910777651497_i128,(-7351074393585375681205554016574622859_i128),158499833166791804192017152285671123311_i128,72291698049051540781052047382909335727_i128,(-67859632637470592059112359759661557485_i128),123275732897672005543129977045641046877_i128,72109001354025062860540105847535519369_i128];
_14 = !18565505208749962_i64;
_10 = _11;
_14 = (-8092762871695781342_i64) >> 137642340595563984_u64;
_8 = [105539890208056545583411948617814936971_i128,(-177200764064981644794323808330222003_i128),(-13843402098921365691554185967825533855_i128),(-97122688012362649616762654816963389357_i128),(-123252391125610200355332033694408262579_i128),158141588194515812545539179719132146813_i128,4791293442820246680940314357904920401_i128,141152759488057701017068105542759134486_i128];
Goto(bb1)
}
bb1 = {
_6 = _3;
_5 = _1;
_5 = [(-94_i8),99_i8,(-18_i8),(-47_i8),10_i8,(-21_i8),(-66_i8)];
_15 = false;
_9 = [128831033812077947498349237280767077671_i128,(-73680758140227395557240377365870387479_i128),(-23175468052881110645755310639119216043_i128),153640472949545403609679297373115993694_i128,37291223257642003208657816844736801177_i128,(-62052277175943375044359455766101674335_i128),(-94955155743683354554556358359320758080_i128),126362230192148006686110382186268781202_i128];
_4 = [137347803450568120756090351880011632960_i128,88774006756702887601380439154409112272_i128,21395536742767087338256736887312302706_i128,(-107983893597996644217274879391977229541_i128),151400433067252218167368083936058676094_i128,108404093779857640120817371730494467194_i128,(-157949976973009659570135601666675037497_i128),(-98953186049071774539250435828739192520_i128)];
_12 = [86418487528095422927656988434226462037_i128,137226204300001304520566574252848647802_i128,(-94383154854113353288703341827646224023_i128),(-66990730485270075943841498593444906698_i128),(-45069052068186455354832886527318485531_i128),(-40689786395938659385715752779919736369_i128),139284298229150749788410401594687895617_i128,86473618422424150056831803738731690503_i128];
RET = [50087861024984930709046228978524935092_i128,(-166040944167042070572437503325627796007_i128),37331572332475470965313289629538163897_i128,(-33903665084820824504004944727422585814_i128),56413462159607678499061265824453508179_i128,(-13546750374741155417805342895947352973_i128),125675788343026108917653681761299710042_i128,(-75882132453840775021507958218438161195_i128)];
_9 = [139520579632027234213165445198434090100_i128,64069832018242141361807866820308822753_i128,169476558733717790710713503660470962586_i128,(-33203772103089247302053680783430839655_i128),107043650775238769844878460760477684253_i128,57029469434214099146001440308249205539_i128,25562308800526321036724480245432906246_i128,(-162034953443761555034737628396584721564_i128)];
_8 = [(-103768727935865283717126349163115357853_i128),144401000428015072302096749085110382774_i128,(-89518264321778169321452680834419345941_i128),12495629698802037499668739483812957811_i128,(-148690590232339790267207733303292806712_i128),43644260653577668324534817518916462490_i128,126617649974822446363661540180360444902_i128,(-15337404829580873809339341551520306966_i128)];
_8 = [(-93774349999086584781419408487077669859_i128),(-44931744323537897215301006764712324414_i128),5893645782866676389258871343259434037_i128,(-136215566602635901698029330475316564771_i128),67052865131478384675644722534339922994_i128,(-54783144742352326190948678778352569136_i128),(-46996507111148284904487524586314966450_i128),68774085335293735344489013708535196668_i128];
_9 = _4;
_12 = [144219135288603878874893404362326999494_i128,(-9205626231995936156047200525581309840_i128),(-58365395159487695222886538507955772867_i128),(-63554672505411698043317997478947067884_i128),148415837785572761192165615004638177688_i128,(-45457873718186822133874024729370222159_i128),(-112441007356945349261740337658941194528_i128),137241690527618440825553410250500465974_i128];
_17.0 = _14 as u32;
_20.3.1 = (_17.0,);
_17.0 = _20.3.1.0;
_10 = [93479952791824881411764793984101495085_i128,(-117757524733820703216957045460966179467_i128),(-94860166087259448328958602507342864395_i128),(-35260375872572900935113830468050159585_i128),(-9255304455887126172489925541077424241_i128),(-46188628114607013894715001544477649824_i128),(-160099211375871897617881027775048802696_i128),140499161284817898155640689630153818295_i128];
_7 = [27360369828249619864964229542064972798_i128,68723412200651751738287332272921242295_i128,91336898101721245487119291922161707374_i128,(-3821916493837382208996342333401067022_i128),(-96456856938489807678733253408033835149_i128),12567431969819992269497721206859668177_i128,(-159912954283133541206445287196954367373_i128),(-96904049301356053201895449035064875403_i128)];
_8 = [153176024422213977829706157479176988810_i128,(-87534177462151205226509746395097196264_i128),(-132440930067326623351756500294060376635_i128),(-19159235282912705595742719866385158748_i128),(-72902729557628421350937105613717178536_i128),6815328654620870776805613269733220239_i128,(-53492956342492210903129792631229184217_i128),(-22548820869168184526051321011737113566_i128)];
Call(_13 = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.0 = _20.3.1.0 + _20.3.1.0;
_13 = _9;
_21.fld1.1 = [9263894681548190487_u64,9608998987667488038_u64];
_20.2.2 = [84357997772674737019501343584608310064_i128,66973100674869248736912534213205019572_i128,39274853252882847194902837403985747970_i128,74855967879516235612853005058996358513_i128,(-138984698055788341056131791653423934830_i128),104314219731042219806231285680196243953_i128,(-6106334501032310285758856622135110683_i128),(-58249641947654135310763650431565255334_i128)];
_20.2.1 = (-88_i8) as f32;
_21.fld1.1 = [9725308133192266930_u64,12160944738136720446_u64];
_20.2.1 = 229653141084068153837405214761659089755_u128 as f32;
_19 = '\u{d7a9d}';
_20.2.2 = [(-139119705152274125352244883175692552643_i128),133934832035305835761053458610512109970_i128,160237528992480721902664865487359691114_i128,39564352024938623844298834682060056248_i128,10327581956118973044803790008959259936_i128,17749879129533373480174404783491913094_i128,161868835810800533339655537588757216_i128,107827930347046580850595591036782789217_i128];
RET = [(-73944608996108330832290729326395656412_i128),77099926939272078044632781265506035418_i128,(-98664292164056856671018146574821184397_i128),54144356546124168842442944704734715467_i128,(-23160463493112919409463001456915825129_i128),164210935228524317158113109649105070683_i128,36041343716766514047109938765520849888_i128,27943366722057856837168762428397633214_i128];
Call(_28 = fn12(_12, _20.2.2, _4, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_28 = _15 as i8;
_11 = [(-126869297793017590988050436703147877882_i128),(-160351505974072617957495665551087831734_i128),(-75053829767691099287533591650469356964_i128),141180728712101279894828374828663867651_i128,72033256901248559633745281395340009114_i128,(-50431377091298476229687660680082558794_i128),(-23665747045091653168461540183034707973_i128),(-110340137013315239144563029345660978359_i128)];
_2 = [_28,_28,_28,_28,_28,_28,_28];
_25.2 = _8;
_20.3.1 = (_17.0,);
_24 = 3811423002925578974_u64;
_10 = [(-98042832093661841352403419211369022187_i128),(-30951938710255456909074881369597716144_i128),(-21337440217491522711848134338097528981_i128),(-116071281975436289849948981328343729514_i128),(-41437125394866397401755397326270680419_i128),68451430124919567538585971267196344250_i128,55882083538184982900133523602720953848_i128,166024514788910564166614810216712782455_i128];
_20.3.0 = [_28,_28,_28,_28,_28,_28,_28];
_20.2.2 = _25.2;
_25.2 = [(-160767241109243770664360197509160763713_i128),(-163598227332267547254074150392677642957_i128),10781285319242589392504814358579693081_i128,(-100080312941729528599960291382711562208_i128),(-56463899081215292502377359272038720673_i128),153329010195165422780782011495511674341_i128,113240420530523531346104727195122127617_i128,47347351551506761735667815971840163421_i128];
_21.fld1.0 = !(-9223372036854775808_isize);
_25.2 = _4;
_2 = _6;
Call(_20.2 = fn13(_13, _19, _3, _5, _10, _10, _7, _2, _3, _25.2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_25.0 = _2;
_20.2.0 = [_28,_28,_28,_28,_28,_28,_28];
_25.2 = [(-169724359874156205375489987438993422864_i128),(-108576176620467297646454972150325804896_i128),75972556879198155866906698676532847539_i128,(-40463775911234288966540058243662926405_i128),(-77351185358361239801372646235323115037_i128),81056637883754293399345738546985428373_i128,(-106946105676024971556341965475667343588_i128),5073484756843151211200185189204954826_i128];
_21.fld1.0 = !55_isize;
_18.fld2 = 0_usize & 7_usize;
_22 = 10192_i16;
_22 = _14 as i16;
_25 = _20.2;
_32 = _2;
_15 = false;
_20.0.1 = [_22,_22,_22];
_24 = 10834322773437810876_u64;
_29 = _21.fld1.0 + _21.fld1.0;
RET = _7;
_25.1 = _20.2.1;
_22 = -(-12268_i16);
_1 = [_28,_28,_28,_28,_28,_28,_28];
_11 = [(-130039614565728789890257981493943862328_i128),(-161633205774021233302444302532611420990_i128),92613632470305967589281519688344760184_i128,13985533456218835020613271875814136051_i128,3065194125135343528154735261528349529_i128,(-168662865836349578958458052043274007378_i128),127148300298414137609066277478260944763_i128,86997596145441333982025296207242359723_i128];
_1 = [_28,_28,_28,_28,_28,_28,_28];
_5 = [_28,_28,_28,_28,_28,_28,_28];
_34 = _14 as f64;
_13 = [(-82265547327455296510953440828305946290_i128),(-40670694959570734071342065035653274058_i128),(-58739182492443628697401352935241708485_i128),(-58168990390151025968944550780050661956_i128),136849304753624751609721076529259132684_i128,164185055712305948453724224880127952979_i128,29800604721739345382119166155478649415_i128,(-132778951411315568644099571670065689489_i128)];
match _24 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
10834322773437810876 => bb10,
_ => bb9
}
}
bb5 = {
_28 = _15 as i8;
_11 = [(-126869297793017590988050436703147877882_i128),(-160351505974072617957495665551087831734_i128),(-75053829767691099287533591650469356964_i128),141180728712101279894828374828663867651_i128,72033256901248559633745281395340009114_i128,(-50431377091298476229687660680082558794_i128),(-23665747045091653168461540183034707973_i128),(-110340137013315239144563029345660978359_i128)];
_2 = [_28,_28,_28,_28,_28,_28,_28];
_25.2 = _8;
_20.3.1 = (_17.0,);
_24 = 3811423002925578974_u64;
_10 = [(-98042832093661841352403419211369022187_i128),(-30951938710255456909074881369597716144_i128),(-21337440217491522711848134338097528981_i128),(-116071281975436289849948981328343729514_i128),(-41437125394866397401755397326270680419_i128),68451430124919567538585971267196344250_i128,55882083538184982900133523602720953848_i128,166024514788910564166614810216712782455_i128];
_20.3.0 = [_28,_28,_28,_28,_28,_28,_28];
_20.2.2 = _25.2;
_25.2 = [(-160767241109243770664360197509160763713_i128),(-163598227332267547254074150392677642957_i128),10781285319242589392504814358579693081_i128,(-100080312941729528599960291382711562208_i128),(-56463899081215292502377359272038720673_i128),153329010195165422780782011495511674341_i128,113240420530523531346104727195122127617_i128,47347351551506761735667815971840163421_i128];
_21.fld1.0 = !(-9223372036854775808_isize);
_25.2 = _4;
_2 = _6;
Call(_20.2 = fn13(_13, _19, _3, _5, _10, _10, _7, _2, _3, _25.2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_17.0 = _20.3.1.0 + _20.3.1.0;
_13 = _9;
_21.fld1.1 = [9263894681548190487_u64,9608998987667488038_u64];
_20.2.2 = [84357997772674737019501343584608310064_i128,66973100674869248736912534213205019572_i128,39274853252882847194902837403985747970_i128,74855967879516235612853005058996358513_i128,(-138984698055788341056131791653423934830_i128),104314219731042219806231285680196243953_i128,(-6106334501032310285758856622135110683_i128),(-58249641947654135310763650431565255334_i128)];
_20.2.1 = (-88_i8) as f32;
_21.fld1.1 = [9725308133192266930_u64,12160944738136720446_u64];
_20.2.1 = 229653141084068153837405214761659089755_u128 as f32;
_19 = '\u{d7a9d}';
_20.2.2 = [(-139119705152274125352244883175692552643_i128),133934832035305835761053458610512109970_i128,160237528992480721902664865487359691114_i128,39564352024938623844298834682060056248_i128,10327581956118973044803790008959259936_i128,17749879129533373480174404783491913094_i128,161868835810800533339655537588757216_i128,107827930347046580850595591036782789217_i128];
RET = [(-73944608996108330832290729326395656412_i128),77099926939272078044632781265506035418_i128,(-98664292164056856671018146574821184397_i128),54144356546124168842442944704734715467_i128,(-23160463493112919409463001456915825129_i128),164210935228524317158113109649105070683_i128,36041343716766514047109938765520849888_i128,27943366722057856837168762428397633214_i128];
Call(_28 = fn12(_12, _20.2.2, _4, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_6 = _3;
_5 = _1;
_5 = [(-94_i8),99_i8,(-18_i8),(-47_i8),10_i8,(-21_i8),(-66_i8)];
_15 = false;
_9 = [128831033812077947498349237280767077671_i128,(-73680758140227395557240377365870387479_i128),(-23175468052881110645755310639119216043_i128),153640472949545403609679297373115993694_i128,37291223257642003208657816844736801177_i128,(-62052277175943375044359455766101674335_i128),(-94955155743683354554556358359320758080_i128),126362230192148006686110382186268781202_i128];
_4 = [137347803450568120756090351880011632960_i128,88774006756702887601380439154409112272_i128,21395536742767087338256736887312302706_i128,(-107983893597996644217274879391977229541_i128),151400433067252218167368083936058676094_i128,108404093779857640120817371730494467194_i128,(-157949976973009659570135601666675037497_i128),(-98953186049071774539250435828739192520_i128)];
_12 = [86418487528095422927656988434226462037_i128,137226204300001304520566574252848647802_i128,(-94383154854113353288703341827646224023_i128),(-66990730485270075943841498593444906698_i128),(-45069052068186455354832886527318485531_i128),(-40689786395938659385715752779919736369_i128),139284298229150749788410401594687895617_i128,86473618422424150056831803738731690503_i128];
RET = [50087861024984930709046228978524935092_i128,(-166040944167042070572437503325627796007_i128),37331572332475470965313289629538163897_i128,(-33903665084820824504004944727422585814_i128),56413462159607678499061265824453508179_i128,(-13546750374741155417805342895947352973_i128),125675788343026108917653681761299710042_i128,(-75882132453840775021507958218438161195_i128)];
_9 = [139520579632027234213165445198434090100_i128,64069832018242141361807866820308822753_i128,169476558733717790710713503660470962586_i128,(-33203772103089247302053680783430839655_i128),107043650775238769844878460760477684253_i128,57029469434214099146001440308249205539_i128,25562308800526321036724480245432906246_i128,(-162034953443761555034737628396584721564_i128)];
_8 = [(-103768727935865283717126349163115357853_i128),144401000428015072302096749085110382774_i128,(-89518264321778169321452680834419345941_i128),12495629698802037499668739483812957811_i128,(-148690590232339790267207733303292806712_i128),43644260653577668324534817518916462490_i128,126617649974822446363661540180360444902_i128,(-15337404829580873809339341551520306966_i128)];
_8 = [(-93774349999086584781419408487077669859_i128),(-44931744323537897215301006764712324414_i128),5893645782866676389258871343259434037_i128,(-136215566602635901698029330475316564771_i128),67052865131478384675644722534339922994_i128,(-54783144742352326190948678778352569136_i128),(-46996507111148284904487524586314966450_i128),68774085335293735344489013708535196668_i128];
_9 = _4;
_12 = [144219135288603878874893404362326999494_i128,(-9205626231995936156047200525581309840_i128),(-58365395159487695222886538507955772867_i128),(-63554672505411698043317997478947067884_i128),148415837785572761192165615004638177688_i128,(-45457873718186822133874024729370222159_i128),(-112441007356945349261740337658941194528_i128),137241690527618440825553410250500465974_i128];
_17.0 = _14 as u32;
_20.3.1 = (_17.0,);
_17.0 = _20.3.1.0;
_10 = [93479952791824881411764793984101495085_i128,(-117757524733820703216957045460966179467_i128),(-94860166087259448328958602507342864395_i128),(-35260375872572900935113830468050159585_i128),(-9255304455887126172489925541077424241_i128),(-46188628114607013894715001544477649824_i128),(-160099211375871897617881027775048802696_i128),140499161284817898155640689630153818295_i128];
_7 = [27360369828249619864964229542064972798_i128,68723412200651751738287332272921242295_i128,91336898101721245487119291922161707374_i128,(-3821916493837382208996342333401067022_i128),(-96456856938489807678733253408033835149_i128),12567431969819992269497721206859668177_i128,(-159912954283133541206445287196954367373_i128),(-96904049301356053201895449035064875403_i128)];
_8 = [153176024422213977829706157479176988810_i128,(-87534177462151205226509746395097196264_i128),(-132440930067326623351756500294060376635_i128),(-19159235282912705595742719866385158748_i128),(-72902729557628421350937105613717178536_i128),6815328654620870776805613269733220239_i128,(-53492956342492210903129792631229184217_i128),(-22548820869168184526051321011737113566_i128)];
Call(_13 = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_36.0 = _22 | _22;
_20.0.0 = 206_u8 as i16;
_35 = [_24,_24];
_30 = Adt45::Variant2 { fld0: _24,fld1: _20.3,fld2: 1971276490_i32 };
_36.1 = _20.0.1;
_17.0 = Field::<([i8; 7], (u32,))>(Variant(_30, 2), 1).1.0 & _20.3.1.0;
_7 = [97486535939856688119567303049384605741_i128,141462012466913524265445085226175233764_i128,134563791877028081877254382415695803720_i128,38512173780543806859159897920237502848_i128,(-101195509614939569310173205689555447921_i128),93301700650412934813221462674745955566_i128,138189827492771937879190152229112649749_i128,(-71046158066154041192370937608569117876_i128)];
_18.fld2 = !6_usize;
_7 = [56621846111490673156419559247457339953_i128,(-32518259101051673398853704623311924025_i128),(-145159404162285591238074932419420709209_i128),(-88500178827831902279712437055143948501_i128),79316817751092962108705874389889622073_i128,(-116280023597958041771800119261199578845_i128),759759891337760581206813778678984277_i128,158758831496569618373536410207812039635_i128];
_17 = Field::<([i8; 7], (u32,))>(Variant(_30, 2), 1).1;
_24 = !Field::<u64>(Variant(_30, 2), 0);
_20.0 = _36;
_21.fld1.1 = _35;
_23 = _34 as f32;
_35 = [Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0)];
_15 = !true;
_36.1 = [_36.0,_36.0,_36.0];
_19 = '\u{10374e}';
match Field::<u64>(Variant(_30, 2), 0) {
0 => bb11,
1 => bb12,
10834322773437810876 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
_6 = _3;
_5 = _1;
_5 = [(-94_i8),99_i8,(-18_i8),(-47_i8),10_i8,(-21_i8),(-66_i8)];
_15 = false;
_9 = [128831033812077947498349237280767077671_i128,(-73680758140227395557240377365870387479_i128),(-23175468052881110645755310639119216043_i128),153640472949545403609679297373115993694_i128,37291223257642003208657816844736801177_i128,(-62052277175943375044359455766101674335_i128),(-94955155743683354554556358359320758080_i128),126362230192148006686110382186268781202_i128];
_4 = [137347803450568120756090351880011632960_i128,88774006756702887601380439154409112272_i128,21395536742767087338256736887312302706_i128,(-107983893597996644217274879391977229541_i128),151400433067252218167368083936058676094_i128,108404093779857640120817371730494467194_i128,(-157949976973009659570135601666675037497_i128),(-98953186049071774539250435828739192520_i128)];
_12 = [86418487528095422927656988434226462037_i128,137226204300001304520566574252848647802_i128,(-94383154854113353288703341827646224023_i128),(-66990730485270075943841498593444906698_i128),(-45069052068186455354832886527318485531_i128),(-40689786395938659385715752779919736369_i128),139284298229150749788410401594687895617_i128,86473618422424150056831803738731690503_i128];
RET = [50087861024984930709046228978524935092_i128,(-166040944167042070572437503325627796007_i128),37331572332475470965313289629538163897_i128,(-33903665084820824504004944727422585814_i128),56413462159607678499061265824453508179_i128,(-13546750374741155417805342895947352973_i128),125675788343026108917653681761299710042_i128,(-75882132453840775021507958218438161195_i128)];
_9 = [139520579632027234213165445198434090100_i128,64069832018242141361807866820308822753_i128,169476558733717790710713503660470962586_i128,(-33203772103089247302053680783430839655_i128),107043650775238769844878460760477684253_i128,57029469434214099146001440308249205539_i128,25562308800526321036724480245432906246_i128,(-162034953443761555034737628396584721564_i128)];
_8 = [(-103768727935865283717126349163115357853_i128),144401000428015072302096749085110382774_i128,(-89518264321778169321452680834419345941_i128),12495629698802037499668739483812957811_i128,(-148690590232339790267207733303292806712_i128),43644260653577668324534817518916462490_i128,126617649974822446363661540180360444902_i128,(-15337404829580873809339341551520306966_i128)];
_8 = [(-93774349999086584781419408487077669859_i128),(-44931744323537897215301006764712324414_i128),5893645782866676389258871343259434037_i128,(-136215566602635901698029330475316564771_i128),67052865131478384675644722534339922994_i128,(-54783144742352326190948678778352569136_i128),(-46996507111148284904487524586314966450_i128),68774085335293735344489013708535196668_i128];
_9 = _4;
_12 = [144219135288603878874893404362326999494_i128,(-9205626231995936156047200525581309840_i128),(-58365395159487695222886538507955772867_i128),(-63554672505411698043317997478947067884_i128),148415837785572761192165615004638177688_i128,(-45457873718186822133874024729370222159_i128),(-112441007356945349261740337658941194528_i128),137241690527618440825553410250500465974_i128];
_17.0 = _14 as u32;
_20.3.1 = (_17.0,);
_17.0 = _20.3.1.0;
_10 = [93479952791824881411764793984101495085_i128,(-117757524733820703216957045460966179467_i128),(-94860166087259448328958602507342864395_i128),(-35260375872572900935113830468050159585_i128),(-9255304455887126172489925541077424241_i128),(-46188628114607013894715001544477649824_i128),(-160099211375871897617881027775048802696_i128),140499161284817898155640689630153818295_i128];
_7 = [27360369828249619864964229542064972798_i128,68723412200651751738287332272921242295_i128,91336898101721245487119291922161707374_i128,(-3821916493837382208996342333401067022_i128),(-96456856938489807678733253408033835149_i128),12567431969819992269497721206859668177_i128,(-159912954283133541206445287196954367373_i128),(-96904049301356053201895449035064875403_i128)];
_8 = [153176024422213977829706157479176988810_i128,(-87534177462151205226509746395097196264_i128),(-132440930067326623351756500294060376635_i128),(-19159235282912705595742719866385158748_i128),(-72902729557628421350937105613717178536_i128),6815328654620870776805613269733220239_i128,(-53492956342492210903129792631229184217_i128),(-22548820869168184526051321011737113566_i128)];
Call(_13 = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_28 = _15 as i8;
_11 = [(-126869297793017590988050436703147877882_i128),(-160351505974072617957495665551087831734_i128),(-75053829767691099287533591650469356964_i128),141180728712101279894828374828663867651_i128,72033256901248559633745281395340009114_i128,(-50431377091298476229687660680082558794_i128),(-23665747045091653168461540183034707973_i128),(-110340137013315239144563029345660978359_i128)];
_2 = [_28,_28,_28,_28,_28,_28,_28];
_25.2 = _8;
_20.3.1 = (_17.0,);
_24 = 3811423002925578974_u64;
_10 = [(-98042832093661841352403419211369022187_i128),(-30951938710255456909074881369597716144_i128),(-21337440217491522711848134338097528981_i128),(-116071281975436289849948981328343729514_i128),(-41437125394866397401755397326270680419_i128),68451430124919567538585971267196344250_i128,55882083538184982900133523602720953848_i128,166024514788910564166614810216712782455_i128];
_20.3.0 = [_28,_28,_28,_28,_28,_28,_28];
_20.2.2 = _25.2;
_25.2 = [(-160767241109243770664360197509160763713_i128),(-163598227332267547254074150392677642957_i128),10781285319242589392504814358579693081_i128,(-100080312941729528599960291382711562208_i128),(-56463899081215292502377359272038720673_i128),153329010195165422780782011495511674341_i128,113240420530523531346104727195122127617_i128,47347351551506761735667815971840163421_i128];
_21.fld1.0 = !(-9223372036854775808_isize);
_25.2 = _4;
_2 = _6;
Call(_20.2 = fn13(_13, _19, _3, _5, _10, _10, _7, _2, _3, _25.2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_18.fld2 = 2_usize | 13931512800273270074_usize;
RET = _12;
_37 = 111928450550997354238696492231382233700_u128;
_38 = _37 % _37;
_33 = [Field::<u64>(Variant(_30, 2), 0),_24,_24,_24,Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0),_24,Field::<u64>(Variant(_30, 2), 0)];
_2 = [_28,_28,_28,_28,_28,_28,_28];
_4 = _10;
_23 = _25.1 - _20.2.1;
place!(Field::<i32>(Variant(_30, 2), 2)) = !(-428366792_i32);
_1 = [_28,_28,_28,_28,_28,_28,_28];
_21.fld1.0 = _29 + _29;
_30 = Adt45::Variant2 { fld0: _24,fld1: _20.3,fld2: (-954313507_i32) };
_3 = _32;
place!(Field::<i32>(Variant(_30, 2), 2)) = -(-1140525253_i32);
_6 = [_28,_28,_28,_28,_28,_28,_28];
_30 = Adt45::Variant2 { fld0: _24,fld1: _20.3,fld2: 641131392_i32 };
_15 = false;
_33 = [Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0),_24,_24,Field::<u64>(Variant(_30, 2), 0),Field::<u64>(Variant(_30, 2), 0)];
_14 = -1079365686054947925_i64;
_25.1 = _23 + _23;
_9 = [142033027443003534881013755567206417004_i128,131492339575967200147666123348744655327_i128,122637349795780652242279893218886446289_i128,(-116778651685009872042759514209106149660_i128),(-104931122554636042821117210119832646982_i128),(-39211288899829886894764836657859385019_i128),34796035873944737574309315096426035310_i128,168331628240663172789245997080059045678_i128];
place!(Field::<([i8; 7], (u32,))>(Variant(_30, 2), 1)).1.0 = _29 as u32;
_6 = _32;
_40.1 = _20.3.1;
_19 = '\u{5e6e1}';
_17.0 = _40.1.0 << _20.3.1.0;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(11_usize, 5_usize, Move(_5), 32_usize, Move(_32), 37_usize, Move(_37), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(11_usize, 3_usize, Move(_3), 7_usize, Move(_7), 15_usize, Move(_15), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(11_usize, 38_usize, Move(_38), 4_usize, Move(_4), 22_usize, Move(_22), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(11_usize, 13_usize, Move(_13), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i128; 8],mut _4: [i8; 7],mut _5: [i128; 8]) -> i8 {
mir! {
type RET = i8;
let _6: [i128; 8];
let _7: isize;
let _8: Adt43;
let _9: u16;
let _10: Adt38;
let _11: u64;
let _12: Adt47;
let _13: i128;
let _14: f64;
let _15: (i16, [i16; 3]);
let _16: i128;
let _17: [i16; 3];
let _18: u64;
let _19: Adt45;
let _20: isize;
let _21: bool;
let _22: [u64; 2];
let _23: (isize, [u64; 2]);
let _24: ();
let _25: ();
{
_2 = [120698333166216086919510135467668174059_i128,130459039809755472857476631110096572528_i128,94281030080344253987343295087328309414_i128,57264916977336835551112625944735753773_i128,(-31219770060572119509084304568609595188_i128),(-36856333065349860687078302113133821068_i128),(-73707893072972278992379167457269207078_i128),(-163939325933491131537766613324347443673_i128)];
_2 = _5;
_2 = [(-621649421092906232310671963450389020_i128),(-81798651036080666899403631842456143912_i128),48613364282594769098838042784188521470_i128,102597061423531204023677457921770784481_i128,(-26167577793792844439846300881493493491_i128),46574646460622504142551827208487968536_i128,109613642431148111078816441134364024642_i128,75296646105205877733290902609607778059_i128];
_1 = [104448102924430992309606035289662162069_i128,(-77652944219984916921010110455133385202_i128),66809720654864660681068127713846166875_i128,12743559511601218723383916529199583616_i128,(-120989071826522011803602519305519640110_i128),2038349268920298478889413903226367869_i128,(-156691854701224828906604082972382313873_i128),(-52461789707693307713736784321411689858_i128)];
_1 = _2;
_6 = _1;
_10.fld1.0 = (-9223372036854775808_isize);
RET = (-117_i8) & 92_i8;
_4 = [RET,RET,RET,RET,RET,RET,RET];
_9 = 53772_u16;
_7 = -_10.fld1.0;
match _10.fld1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
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
RET = 109_i8;
_7 = !_10.fld1.0;
_10.fld1.0 = 127_u8 as isize;
_4 = [RET,RET,RET,RET,RET,RET,RET];
RET = -61_i8;
_9 = 6283_u16 ^ 51181_u16;
_9 = 24362_u16;
_9 = false as u16;
_1 = _2;
_1 = _3;
RET = 16369816196679882287_u64 as i8;
_4 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb7)
}
bb7 = {
_10.fld1.1 = [1406097031193787272_u64,3944725216234744176_u64];
_11 = 7126680190101904998_u64;
_11 = (-15902_i16) as u64;
_1 = [(-146578467496683223855075723588549052114_i128),132525217181846253611395853808492829062_i128,(-108977858626696935233660791259715787761_i128),86110998881240675269753291902767893193_i128,(-123042971887142138471953426337361135139_i128),(-2562298210599404774456712669467150294_i128),(-88716030114203789444439095885094331597_i128),(-136664311031520890262704222374933834829_i128)];
_5 = _6;
_9 = 15231_u16 << _7;
_3 = [(-165665684333573466956079770773044864227_i128),155136650350140291606143570505668324196_i128,124193828733854975861448054883018226996_i128,(-77982677454545429404990130083275900383_i128),47932915232547248576070398955967678253_i128,44796558376475663996787170434519559043_i128,85093570561167391779038643763598028234_i128,38666706922365009115786952299630546794_i128];
_14 = _7 as f64;
_3 = [116548533039844970800275833095204960384_i128,(-22373908533986515737001761615093858035_i128),(-85671198331591929627194885909736621220_i128),(-75070857037512505041346893546419873379_i128),(-79127594620776592513418432839995960275_i128),(-25791759256504946721708457344099632406_i128),149546868330224328173428112413551719468_i128,(-156787212504661200355309580311468073876_i128)];
_4 = [RET,RET,RET,RET,RET,RET,RET];
_9 = 7804_u16;
_14 = 154928606202072802257623995703443654798_u128 as f64;
RET = !(-14_i8);
match _9 {
0 => bb8,
1 => bb9,
2 => bb10,
7804 => bb12,
_ => bb11
}
}
bb8 = {
RET = 109_i8;
_7 = !_10.fld1.0;
_10.fld1.0 = 127_u8 as isize;
_4 = [RET,RET,RET,RET,RET,RET,RET];
RET = -61_i8;
_9 = 6283_u16 ^ 51181_u16;
_9 = 24362_u16;
_9 = false as u16;
_1 = _2;
_1 = _3;
RET = 16369816196679882287_u64 as i8;
_4 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb7)
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
_15.1 = [(-1290_i16),21664_i16,18357_i16];
_10.fld1.0 = _7 ^ _7;
match _9 {
0 => bb13,
1 => bb14,
7804 => bb16,
_ => bb15
}
}
bb13 = {
RET = 109_i8;
_7 = !_10.fld1.0;
_10.fld1.0 = 127_u8 as isize;
_4 = [RET,RET,RET,RET,RET,RET,RET];
RET = -61_i8;
_9 = 6283_u16 ^ 51181_u16;
_9 = 24362_u16;
_9 = false as u16;
_1 = _2;
_1 = _3;
RET = 16369816196679882287_u64 as i8;
_4 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb7)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_4 = [RET,RET,RET,RET,RET,RET,RET];
_7 = _10.fld1.0;
_3 = [(-148269772449997139012590352165294078807_i128),(-104279074194166414430404328721013340853_i128),(-30573318855843173872737201641731190920_i128),(-46407923454157876479995379301790869649_i128),(-16074468060226591008400530604833267241_i128),(-163035660174073776030400440024744903591_i128),95643216015111505106147156923709994237_i128,61310501811881543607634244018510672052_i128];
_13 = (-115289311134282505530006028982924347757_i128);
_14 = 12410660107720821143_usize as f64;
_5 = _3;
_17 = [18259_i16,32415_i16,22487_i16];
RET = 64_i8;
_23.0 = _7;
_11 = 90517349781671450723918132940548376322_u128 as u64;
_16 = '\u{6b6e8}' as i128;
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(12_usize, 17_usize, Move(_17), 3_usize, Move(_3), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(12_usize, 7_usize, Move(_7), 4_usize, Move(_4), 25_usize, _25, 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i128; 8],mut _2: char,mut _3: [i8; 7],mut _4: [i8; 7],mut _5: [i128; 8],mut _6: [i128; 8],mut _7: [i128; 8],mut _8: [i8; 7],mut _9: [i8; 7],mut _10: [i128; 8],mut _11: [i8; 7]) -> ([i8; 7], f32, [i128; 8]) {
mir! {
type RET = ([i8; 7], f32, [i128; 8]);
let _12: isize;
let _13: i8;
let _14: u128;
let _15: [u64; 2];
let _16: (u32,);
let _17: i128;
let _18: (i16, [i16; 3]);
let _19: u128;
let _20: Adt39;
let _21: [i128; 8];
let _22: (isize, [u64; 2]);
let _23: [u64; 2];
let _24: Adt43;
let _25: f32;
let _26: f64;
let _27: i16;
let _28: bool;
let _29: f64;
let _30: (i16, [i16; 3]);
let _31: char;
let _32: [i16; 8];
let _33: &'static u8;
let _34: *mut u16;
let _35: i64;
let _36: Adt40;
let _37: Adt41;
let _38: i64;
let _39: char;
let _40: Adt48;
let _41: (u32,);
let _42: (isize, [u64; 2]);
let _43: i16;
let _44: ([i8; 7], (u32,));
let _45: Adt52;
let _46: [u128; 6];
let _47: [i16; 3];
let _48: bool;
let _49: [i8; 7];
let _50: Adt45;
let _51: i32;
let _52: char;
let _53: bool;
let _54: ((i16, [i16; 3]), &'static u8, ([i8; 7], f32, [i128; 8]), ([i8; 7], (u32,)));
let _55: f32;
let _56: i32;
let _57: [i128; 8];
let _58: ();
let _59: ();
{
RET.1 = 51794_u16 as f32;
RET.2 = _10;
_6 = _1;
RET.0 = [(-3_i8),(-108_i8),47_i8,(-13_i8),(-98_i8),74_i8,(-124_i8)];
_8 = [(-61_i8),(-38_i8),9_i8,(-101_i8),(-123_i8),(-43_i8),(-15_i8)];
_12 = 9223372036854775807_isize >> 190_u8;
_6 = RET.2;
RET.2 = [16845957341090999894458716716901554830_i128,89615537101715112156518109276006829560_i128,115076025092238108936659868167423515331_i128,168772989439668372859345796476115373785_i128,(-117475904930111022199871817312577219100_i128),78914077015438552129914415407120932527_i128,(-52520252395854697968149436339788917344_i128),6726297818705304659290432933096482130_i128];
RET.0 = [14_i8,(-96_i8),56_i8,15_i8,(-79_i8),21_i8,(-38_i8)];
RET.0 = [6_i8,69_i8,94_i8,(-120_i8),(-37_i8),121_i8,(-50_i8)];
_6 = _1;
_7 = [(-99996651909250086508633603964926659134_i128),84144979329277783602344990729521286568_i128,(-111924008887379088989030841092563043120_i128),151283311044881498017223311505424812616_i128,(-159029230042596849002005145310780683957_i128),(-144532213857687642427096956785561021989_i128),90911430209144395756235975743057308102_i128,(-20847527778379939245761231333325650151_i128)];
RET.2 = _7;
_4 = RET.0;
RET.2 = [(-167883957044358340133702561853804371097_i128),43233128119366679019866931264908180579_i128,(-19049264242001432898373761637280197077_i128),112704353043757508111267683426978436747_i128,(-73292102554380703024444928580197322663_i128),152473756568331129985897911998942219383_i128,(-22134315839973824162774946701698686032_i128),(-161256613303580430782659837988683284338_i128)];
_9 = _11;
_4 = [(-128_i8),30_i8,30_i8,70_i8,33_i8,(-108_i8),(-3_i8)];
_16.0 = !2062363679_u32;
_16.0 = 15496_i16 as u32;
_15 = [9616104383022307053_u64,16324654183765249893_u64];
Goto(bb1)
}
bb1 = {
_1 = _6;
_6 = _7;
RET.2 = [131278478698481264471692731702347426668_i128,(-56468281788695110735320313062001701880_i128),166568709368764144605533134365499791058_i128,134480257419807091018392632698910894687_i128,160370100711445543866947245079299908653_i128,(-134659971497867003210542751669266700051_i128),(-18556996524450675349542074272811734407_i128),(-162459794934865608972013743734084701217_i128)];
_13 = -49_i8;
_9 = [_13,_13,_13,_13,_13,_13,_13];
Call(_15 = fn14(_7, _1, _8, _13, _3, _12, _5, RET.0, _12, _1, RET.2, _1, _5, _3, RET.2, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = 49748813907296637557094993504791285829_u128;
_7 = [(-21260479412995141910799684846274734498_i128),(-36614801769330711710725145869478311505_i128),(-76622594787689768325394858212829893308_i128),(-44788437448924415393877249194791332299_i128),116005853226509201141149599478607674680_i128,(-146894147774622763160407202725906970156_i128),91549410450151638736014762980603927739_i128,149892977807637407829799140194107735971_i128];
RET.0 = [_13,_13,_13,_13,_13,_13,_13];
_16.0 = 2100343218_u32 << _14;
_5 = [(-139968552303300428395419388837545424407_i128),96188729796611245427410676544476877765_i128,(-8238327385493119062545298801808225968_i128),(-64252560934663923137608899530011755103_i128),(-19265193920756723389605000421624217294_i128),124466344350851634698815962979273597343_i128,2512295981688779479463212089697690966_i128,(-60631331725518491275584164558762650511_i128)];
RET.1 = 72345831603138186198962112658375586384_i128 as f32;
_11 = _3;
_4 = _8;
_17 = -(-13158764279817847419439294002029438794_i128);
_19 = !_14;
_16 = (3672659366_u32,);
_18.1 = [(-782_i16),12303_i16,(-12322_i16)];
_12 = -(-114_isize);
_16.0 = !2858235236_u32;
_15 = [9933425452427273617_u64,4079367271994647628_u64];
RET.1 = _12 as f32;
_21 = [_17,_17,_17,_17,_17,_17,_17,_17];
_6 = [_17,_17,_17,_17,_17,_17,_17,_17];
_16 = (1253950025_u32,);
_22 = (_12, _15);
RET.2 = [_17,_17,_17,_17,_17,_17,_17,_17];
_4 = [_13,_13,_13,_13,_13,_13,_13];
_23 = _22.1;
_11 = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb3)
}
bb3 = {
RET.0 = _8;
_2 = '\u{10c74a}';
_4 = [_13,_13,_13,_13,_13,_13,_13];
_12 = !_22.0;
_13 = 15_i8;
_3 = [_13,_13,_13,_13,_13,_13,_13];
_19 = _14;
_23 = [7363645378590642997_u64,10932909982994831538_u64];
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
_25 = RET.1;
RET.0 = _8;
_6 = _5;
_12 = !_22.0;
_17 = !51482965291070739307306932449981300506_i128;
_18.1 = [30404_i16,4264_i16,26622_i16];
_1 = [_17,_17,_17,_17,_17,_17,_17,_17];
RET.1 = _25;
_26 = 1405222490021598955_i64 as f64;
_22.0 = _12;
_4 = [_13,_13,_13,_13,_13,_13,_13];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb4)
}
bb4 = {
_16 = (3870795018_u32,);
_11 = [_13,_13,_13,_13,_13,_13,_13];
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_28 = false;
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
RET = (_9, _25, _5);
_29 = _16.0 as f64;
_30.1 = _18.1;
RET.0 = [_13,_13,_13,_13,_13,_13,_13];
RET.0 = _3;
_18 = ((-836_i16), _30.1);
RET.0 = [_13,_13,_13,_13,_13,_13,_13];
_25 = -RET.1;
_22 = (_12, _23);
match _18.0 {
0 => bb3,
340282366920938463463374607431768210620 => bb5,
_ => bb2
}
}
bb5 = {
RET = (_4, _25, _6);
_29 = _26 + _26;
_10 = RET.2;
_30.0 = _18.0;
_22 = (_12, _15);
_15 = [14136584854489257255_u64,12009310478419776976_u64];
_8 = _3;
_21 = _6;
_8 = [_13,_13,_13,_13,_13,_13,_13];
RET.1 = _25;
RET.2 = _10;
_10 = [_17,_17,_17,_17,_17,_17,_17,_17];
RET.2 = [_17,_17,_17,_17,_17,_17,_17,_17];
RET.2 = [_17,_17,_17,_17,_17,_17,_17,_17];
_17 = _29 as i128;
RET = (_4, _25, _5);
_3 = [_13,_13,_13,_13,_13,_13,_13];
RET.2 = _5;
_32 = [_18.0,_30.0,_18.0,_30.0,_30.0,_18.0,_30.0,_30.0];
_36 = Adt40::Variant1 { fld0: 16969691235273248957_u64,fld1: 0_usize,fld2: _30.0 };
place!(Field::<i16>(Variant(_36, 1), 2)) = !_18.0;
Goto(bb6)
}
bb6 = {
_28 = !true;
RET.0 = _11;
_36 = Adt40::Variant1 { fld0: 10028407457759912146_u64,fld1: 6_usize,fld2: _18.0 };
_26 = _29 + _29;
_27 = _30.0 ^ Field::<i16>(Variant(_36, 1), 2);
_35 = 2773336328820686390_usize as i64;
_9 = [_13,_13,_13,_13,_13,_13,_13];
RET.1 = -_25;
_23 = [5397935473029185068_u64,4755697238085556096_u64];
_5 = _6;
_40.fld7 = _9;
Goto(bb7)
}
bb7 = {
_13 = (-57_i8) * 74_i8;
_40.fld0.1 = (_16.0,);
_26 = _29 * _29;
RET.0 = [_13,_13,_13,_13,_13,_13,_13];
place!(Field::<i16>(Variant(_36, 1), 2)) = _30.0 >> _27;
_15 = [11539333440742862828_u64,11159667279000299063_u64];
_40.fld6 = [17323995362442483582_u64,5234524492396911777_u64];
RET.0 = _40.fld7;
_12 = !_22.0;
_40.fld4 = Field::<i16>(Variant(_36, 1), 2) ^ _27;
_30.1 = [Field::<i16>(Variant(_36, 1), 2),Field::<i16>(Variant(_36, 1), 2),Field::<i16>(Variant(_36, 1), 2)];
_42.0 = !_12;
_40.fld0 = (_40.fld7, _16);
_44 = (_11, _16);
_10 = [_17,_17,_17,_17,_17,_17,_17,_17];
place!(Field::<usize>(Variant(_36, 1), 1)) = 7_usize << _35;
_35 = 8039682075862845381_i64 << _27;
_22.1 = _15;
place!(Field::<i16>(Variant(_36, 1), 2)) = !_40.fld4;
_44.1.0 = _16.0;
_18.0 = _40.fld4;
_40.fld0.1.0 = _16.0 % _16.0;
_40.fld4 = -_18.0;
Goto(bb8)
}
bb8 = {
_39 = _2;
_40.fld3 = _35 as f32;
_29 = _40.fld4 as f64;
_42 = _22;
_44 = (_40.fld0.0, _40.fld0.1);
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
_2 = _39;
_40.fld3 = _25;
_43 = Field::<i16>(Variant(_36, 1), 2);
_30.0 = _18.0;
_15 = [16780530866324281530_u64,11821971126120858130_u64];
_27 = -_40.fld4;
_7 = _10;
_43 = _40.fld4;
_22.1 = [9802116731699304781_u64,5400392358131336738_u64];
_30.0 = _18.0 >> _43;
_27 = -_43;
place!(Field::<i16>(Variant(_36, 1), 2)) = _18.0;
_2 = _39;
_46 = [_14,_19,_19,_14,_14,_14];
place!(Field::<i16>(Variant(_36, 1), 2)) = _43 * _30.0;
_31 = _2;
_47 = _30.1;
_42.0 = _22.0;
_39 = _2;
_36 = Adt40::Variant1 { fld0: 7232095207450461603_u64,fld1: 8673137719205620221_usize,fld2: _27 };
place!(Field::<u64>(Variant(_36, 1), 0)) = !5581682307442633226_u64;
_48 = _28 ^ _28;
match _16.0 {
0 => bb9,
1 => bb10,
3870795018 => bb12,
_ => bb11
}
}
bb9 = {
_13 = (-57_i8) * 74_i8;
_40.fld0.1 = (_16.0,);
_26 = _29 * _29;
RET.0 = [_13,_13,_13,_13,_13,_13,_13];
place!(Field::<i16>(Variant(_36, 1), 2)) = _30.0 >> _27;
_15 = [11539333440742862828_u64,11159667279000299063_u64];
_40.fld6 = [17323995362442483582_u64,5234524492396911777_u64];
RET.0 = _40.fld7;
_12 = !_22.0;
_40.fld4 = Field::<i16>(Variant(_36, 1), 2) ^ _27;
_30.1 = [Field::<i16>(Variant(_36, 1), 2),Field::<i16>(Variant(_36, 1), 2),Field::<i16>(Variant(_36, 1), 2)];
_42.0 = !_12;
_40.fld0 = (_40.fld7, _16);
_44 = (_11, _16);
_10 = [_17,_17,_17,_17,_17,_17,_17,_17];
place!(Field::<usize>(Variant(_36, 1), 1)) = 7_usize << _35;
_35 = 8039682075862845381_i64 << _27;
_22.1 = _15;
place!(Field::<i16>(Variant(_36, 1), 2)) = !_40.fld4;
_44.1.0 = _16.0;
_18.0 = _40.fld4;
_40.fld0.1.0 = _16.0 % _16.0;
_40.fld4 = -_18.0;
Goto(bb8)
}
bb10 = {
_28 = !true;
RET.0 = _11;
_36 = Adt40::Variant1 { fld0: 10028407457759912146_u64,fld1: 6_usize,fld2: _18.0 };
_26 = _29 + _29;
_27 = _30.0 ^ Field::<i16>(Variant(_36, 1), 2);
_35 = 2773336328820686390_usize as i64;
_9 = [_13,_13,_13,_13,_13,_13,_13];
RET.1 = -_25;
_23 = [5397935473029185068_u64,4755697238085556096_u64];
_5 = _6;
_40.fld7 = _9;
Goto(bb7)
}
bb11 = {
RET.0 = _8;
_2 = '\u{10c74a}';
_4 = [_13,_13,_13,_13,_13,_13,_13];
_12 = !_22.0;
_13 = 15_i8;
_3 = [_13,_13,_13,_13,_13,_13,_13];
_19 = _14;
_23 = [7363645378590642997_u64,10932909982994831538_u64];
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
_25 = RET.1;
RET.0 = _8;
_6 = _5;
_12 = !_22.0;
_17 = !51482965291070739307306932449981300506_i128;
_18.1 = [30404_i16,4264_i16,26622_i16];
_1 = [_17,_17,_17,_17,_17,_17,_17,_17];
RET.1 = _25;
_26 = 1405222490021598955_i64 as f64;
_22.0 = _12;
_4 = [_13,_13,_13,_13,_13,_13,_13];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb4)
}
bb12 = {
_49 = _40.fld0.0;
place!(Field::<i16>(Variant(_36, 1), 2)) = -_18.0;
_44.0 = [_13,_13,_13,_13,_13,_13,_13];
_4 = [_13,_13,_13,_13,_13,_13,_13];
_32 = [_40.fld4,Field::<i16>(Variant(_36, 1), 2),_40.fld4,_40.fld4,_30.0,_27,Field::<i16>(Variant(_36, 1), 2),Field::<i16>(Variant(_36, 1), 2)];
_3 = RET.0;
RET = (_4, _40.fld3, _21);
_1 = _21;
_54.2.1 = -RET.1;
_12 = _22.0 - _42.0;
Goto(bb13)
}
bb13 = {
_55 = _12 as f32;
_51 = 360598285_i32;
_56 = !_51;
Goto(bb14)
}
bb14 = {
_54.0 = (Field::<i16>(Variant(_36, 1), 2), _30.1);
RET.2 = [_17,_17,_17,_17,_17,_17,_17,_17];
_54.2.1 = -_55;
_30 = (_54.0.0, _47);
_54.3.0 = _3;
_9 = [_13,_13,_13,_13,_13,_13,_13];
_40.fld5 = [_27,_30.0,_27];
_2 = _39;
_40.fld7 = _11;
Goto(bb15)
}
bb15 = {
Call(_58 = dump_var(13_usize, 14_usize, Move(_14), 27_usize, Move(_27), 35_usize, Move(_35), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_58 = dump_var(13_usize, 28_usize, Move(_28), 17_usize, Move(_17), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_58 = dump_var(13_usize, 46_usize, Move(_46), 2_usize, Move(_2), 23_usize, Move(_23), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_58 = dump_var(13_usize, 16_usize, Move(_16), 43_usize, Move(_43), 51_usize, Move(_51), 49_usize, Move(_49)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_58 = dump_var(13_usize, 30_usize, Move(_30), 44_usize, Move(_44), 19_usize, Move(_19), 59_usize, _59), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i8; 7],mut _4: i8,mut _5: [i8; 7],mut _6: isize,mut _7: [i128; 8],mut _8: [i8; 7],mut _9: isize,mut _10: [i128; 8],mut _11: [i128; 8],mut _12: [i128; 8],mut _13: [i128; 8],mut _14: [i8; 7],mut _15: [i128; 8],mut _16: [i128; 8]) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _17: isize;
let _18: f32;
let _19: i16;
let _20: char;
let _21: bool;
let _22: isize;
let _23: isize;
let _24: *const [u128; 6];
let _25: [u64; 8];
let _26: (isize, [u64; 2]);
let _27: Adt54;
let _28: Adt53;
let _29: char;
let _30: Adt39;
let _31: bool;
let _32: Adt44;
let _33: [i16; 3];
let _34: f32;
let _35: i128;
let _36: (i16, usize, [u64; 2], i16);
let _37: [i128; 8];
let _38: Adt42;
let _39: i64;
let _40: [i8; 7];
let _41: u128;
let _42: Adt41;
let _43: bool;
let _44: i8;
let _45: u128;
let _46: f32;
let _47: (i16, [i16; 3]);
let _48: isize;
let _49: Adt53;
let _50: bool;
let _51: [i16; 8];
let _52: [i16; 3];
let _53: char;
let _54: isize;
let _55: u64;
let _56: Adt47;
let _57: ();
let _58: ();
{
_5 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [_4,_4,_4,_4,_4,_4,_4];
_13 = [(-61807401092388836111316650703940351948_i128),90003745718396520704520197715029901784_i128,(-72580794813616401596121130565080150654_i128),45548974892039128514110532175674133974_i128,(-89169793939024399712739117875348432896_i128),(-155718866540455036283432100496921215932_i128),(-145451863239197818024174656636520308264_i128),159920154995378599320285062738928150632_i128];
_17 = _6;
_15 = [170014746764023554959338165059219698837_i128,(-154621243634165607113433776767739798798_i128),37917870022814037277656242989171305901_i128,8061349979285932053186581902357232265_i128,105314128676305531902968996027639226375_i128,(-117923529098407883322831105722912472216_i128),(-162779723934459177813991606849628706724_i128),(-85246186559424542170110383375204984188_i128)];
_13 = [(-146493477792052051188744829292807412862_i128),61949249927710175637977899783505677997_i128,(-94567425632055653208873939331122344451_i128),168027414370636696127312869578220827723_i128,122232111708364034153800703289979874919_i128,99305697206418997216315488612506384659_i128,(-164669913483530013820398291081213525316_i128),(-80233431843906958443609129010214680605_i128)];
RET = [8175917017262209805_u64,1236052514155258420_u64];
_1 = [(-10242257473597711977384889971757335962_i128),3775133390260266595872763729181933975_i128,113996819138001529084151373724523231866_i128,(-38384445767805921881558684367845075140_i128),75722321452883549216043272190816880305_i128,(-122981184995168834668646760690446795990_i128),111071759363847089228915845749373886941_i128,(-117023627536091312631736802425097577492_i128)];
_10 = _16;
_11 = [104485751196926175635936282512942734617_i128,164040332791564135043119614264817809170_i128,(-143259821643155542246138513002305200363_i128),15286024244670758937261607123698298225_i128,(-31105496658628874284661389207428057813_i128),(-140700168536050734078128204468351198595_i128),109356565541370842365525575183284854674_i128,(-9246515279328590932746153366732786282_i128)];
RET = [8000250387764620612_u64,8619995716876110476_u64];
Goto(bb1)
}
bb1 = {
_5 = _8;
_20 = '\u{b4e7a}';
_7 = _11;
_19 = 16628_i16;
RET = [5164824941794957789_u64,13602533122969587566_u64];
_10 = [56483598507858159923202251532440759800_i128,107707797635947857362603769648327710036_i128,111011503510603470538074912347391862040_i128,82308906456793231882506235946417904284_i128,75049861603869333428308633796135591034_i128,9715869088542254213724264134444782030_i128,(-43524802514894521984855455713086139322_i128),134967533909778428033503745666491711359_i128];
_4 = (-77_i8) | 13_i8;
RET = [5604274206871103401_u64,7229332691975285438_u64];
Call(_6 = fn15(_1, _1, _5, _8, _11, _11, _17, _14, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = _15;
_21 = !true;
_1 = _15;
_15 = [(-49173112629360163927441085719489661839_i128),128695364125991156989551906722896922749_i128,(-111148318621596124783029691868291801569_i128),70084155017070735341913710753202483214_i128,135109035899742538716612571261022124881_i128,70426480587188490522607519461216512641_i128,(-53841664757408887587532699356238330837_i128),(-76920626543155752975054768392638150050_i128)];
_20 = '\u{99dd4}';
_19 = 9689_i16 - (-29496_i16);
_18 = _19 as f32;
_16 = [35506558672582244819587280379400032906_i128,129484850186482238998895817977724745025_i128,104427371730039219518797144743974163512_i128,(-131265388068525777680277566129493414087_i128),(-7404880556345422332342632363059204256_i128),(-15435505888071777428019643880636856489_i128),(-147948590480583994769773087392091493932_i128),(-135112698760162340479657738657937295192_i128)];
_20 = '\u{3b4e7}';
_22 = _9;
_10 = [62835309421441051256875717850936841436_i128,(-157828834998099362995158684697296530446_i128),145441946134527726466714117761318380626_i128,20496651956472873290218484964964491738_i128,148817234282363421520848408577180175143_i128,(-48817897008636989910329505163602019715_i128),(-156717322924519607598341570157116705834_i128),(-121200275745659045782460131105827106345_i128)];
_3 = _8;
_16 = _7;
_8 = _3;
_26.0 = !_17;
_7 = _16;
_11 = [153566281949045922756794440967264984217_i128,134136866887129464380298796247735582778_i128,(-164616762744786341221731151301535440104_i128),(-91658772675219822082671388884214723941_i128),(-53049610433855293574524592613899562840_i128),(-118097595834516458710986207712417086620_i128),54905306725833709858583919063107652846_i128,(-143090142403289913414803592572474538208_i128)];
_15 = [150911502339989958866087123065217246445_i128,78525507762692533070295693086029757287_i128,99626287701728010235388070519330331436_i128,(-4742055051806038374959974720371764091_i128),46591311702962004349470843591462503966_i128,144170452097282274744334834501094970880_i128,34287703682879892775482218222034910996_i128,(-88957380934849683411585234910297496150_i128)];
_6 = _9 + _26.0;
_1 = _11;
Goto(bb3)
}
bb3 = {
_12 = _10;
_23 = _20 as isize;
_3 = [_4,_4,_4,_4,_4,_4,_4];
_11 = [13621873541071336442639501145487508339_i128,(-118461894227369836875871725328828274959_i128),(-16484001562238578718508462486968430042_i128),(-98200130468958427918571708156530780314_i128),139000238098632289527818757325120021042_i128,(-25831436414456599313797568267562244997_i128),93331928067383656905378116402170052904_i128,26885919999602756668549505951517979143_i128];
RET = [4533664447088446436_u64,17731605143979717027_u64];
_29 = _20;
_26 = (_22, RET);
_13 = [(-38730976397178751250293801175407038118_i128),77664232099606503384221262297530181426_i128,158773196363403968980805238938124721047_i128,141225616454496786213229893921391037980_i128,27284290513420645455938326803372559436_i128,126099164139693180129376351203573329491_i128,(-134352042508018396185883745585036725823_i128),(-128002497440877633239433472082706851006_i128)];
_26.0 = _23;
_2 = _13;
Goto(bb4)
}
bb4 = {
_6 = _26.0 ^ _26.0;
_9 = 322992640075771055716334094817833792395_u128 as isize;
_18 = 1576610362_u32 as f32;
_7 = [67924208245509452381128220444489225615_i128,81129101173949334228281752333843309622_i128,(-145508831827034560057370403095720383972_i128),(-47385916023676181670180935067855472510_i128),63012660748947590513627760006062194425_i128,128792052227450754046529057729403398023_i128,43034990055300564039978884686781877342_i128,(-102129231706172641870831394728446548772_i128)];
Goto(bb5)
}
bb5 = {
_15 = [137209198074444717757608777713946372258_i128,(-42392077625317716551380282897140042799_i128),165349798378571910224604647012433100763_i128,(-115974278521712420302144517451460672344_i128),(-111925390311105444744337254278084576377_i128),(-87005213310774321205089835017708331172_i128),(-50225222661965201450839125833717432269_i128),(-113023023599473106473644968756866330141_i128)];
_1 = _15;
_15 = _2;
_25 = [5102210926916302353_u64,5690793627739359461_u64,10880205989721054736_u64,7320768845966476035_u64,14634323664220711855_u64,7852346210069777725_u64,5525724187511602966_u64,11342760169502715622_u64];
_18 = 2935563182403566979_usize as f32;
_23 = !_9;
_26.1 = [13628671351657634756_u64,16994853051606882633_u64];
Goto(bb6)
}
bb6 = {
_32 = Adt44::Variant2 { fld0: _25,fld1: _4 };
_8 = [_4,_4,Field::<i8>(Variant(_32, 2), 1),_4,_4,Field::<i8>(Variant(_32, 2), 1),_4];
_26 = (_23, RET);
_12 = _7;
_4 = _29 as i8;
_3 = _14;
_26.0 = _23 >> _17;
_18 = 9024422194118840706_i64 as f32;
_2 = [81049701648379977403956932307477251036_i128,(-13053241383108441981965296425845727392_i128),162016020378856913200914436525688313181_i128,31684300932815983441692310326513444049_i128,(-38432967848107861499527078231618239625_i128),32189848544681935950572384451780181897_i128,115830933666351966649960182546927488000_i128,(-110107540153051001678493943322465500961_i128)];
_10 = _13;
_22 = _26.0 | _17;
_26 = (_22, RET);
_4 = 158_u8 as i8;
_2 = [(-33393617469587542094840957633667694222_i128),(-165442918220465793205471378534420484521_i128),(-160540046399969649448958618011036303836_i128),(-50179362034977832444923948553939059276_i128),(-152587891497936155737736995069853043526_i128),11154615899221856558043927180953667513_i128,128557261308779814023707273276176177617_i128,129703387771447126805883815599460986509_i128];
_6 = !_26.0;
_34 = -_18;
_20 = _29;
_22 = 18083769216899362676_u64 as isize;
_14 = _3;
_33 = [_19,_19,_19];
Call(RET = fn18(_25, _6, _6, _1, _14, _26.0, _25, Field::<[u64; 8]>(Variant(_32, 2), 0), _12, Field::<[u64; 8]>(Variant(_32, 2), 0), _1, _12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = _8;
_36.0 = _19;
_36 = (_19, 5_usize, RET, _19);
_31 = !_21;
_17 = 200_u8 as isize;
_2 = [135221311359885822890251499039440030075_i128,(-41360139748338128661089146641873586054_i128),(-123523102907826855135079977579420462644_i128),(-21011737016614725178968891953095652645_i128),(-99242675587669079592401454637545395036_i128),19643591535101037156036164174944230782_i128,(-71731689846807389832312368078271355369_i128),115026382278381099076453959699658186396_i128];
place!(Field::<[u64; 8]>(Variant(_32, 2), 0)) = _25;
_26.1 = [2761633255241128900_u64,4760816016895229785_u64];
_39 = (-3984236022825936681_i64) | (-2170315249824266623_i64);
_12 = [106210996188614392281963347918994722801_i128,(-34508058096774622551824260414240961197_i128),(-30563301511226358598997446341193056563_i128),81862689914244133031478488803823193065_i128,(-40504415531748035259052265701609366971_i128),(-60727584170508411416997880852909510396_i128),(-11270098219366655041923746018762359974_i128),22182527689683287703446870890007368849_i128];
_40 = [Field::<i8>(Variant(_32, 2), 1),_4,Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1)];
RET = [12691117334789924581_u64,13013880637931378769_u64];
Goto(bb8)
}
bb8 = {
place!(Field::<i8>(Variant(_32, 2), 1)) = _4;
_36.0 = _36.3 ^ _36.3;
_7 = [(-122999466205260132363174068551956010541_i128),(-22649504392921306175427509538620192733_i128),85406773313570993212397603827596083998_i128,(-5635111816728209784135020874560336528_i128),(-61936012118887859440301949955162895526_i128),156539834136354273206728771688364959282_i128,54394295044537576683833058111105333255_i128,18467489525855630675245416801142832900_i128];
_20 = _29;
_5 = [Field::<i8>(Variant(_32, 2), 1),_4,_4,Field::<i8>(Variant(_32, 2), 1),_4,Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1)];
_35 = !95267060755453117696370834208634608899_i128;
_31 = _36.3 <= _36.0;
_3 = [Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),_4,_4,_4];
_36 = (_19, 0_usize, _26.1, _19);
_29 = _20;
match _36.1 {
0 => bb10,
_ => bb9
}
}
bb9 = {
_5 = _8;
_20 = '\u{b4e7a}';
_7 = _11;
_19 = 16628_i16;
RET = [5164824941794957789_u64,13602533122969587566_u64];
_10 = [56483598507858159923202251532440759800_i128,107707797635947857362603769648327710036_i128,111011503510603470538074912347391862040_i128,82308906456793231882506235946417904284_i128,75049861603869333428308633796135591034_i128,9715869088542254213724264134444782030_i128,(-43524802514894521984855455713086139322_i128),134967533909778428033503745666491711359_i128];
_4 = (-77_i8) | 13_i8;
RET = [5604274206871103401_u64,7229332691975285438_u64];
Call(_6 = fn15(_1, _1, _5, _8, _11, _11, _17, _14, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_2 = [_35,_35,_35,_35,_35,_35,_35,_35];
_40 = [Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),_4,Field::<i8>(Variant(_32, 2), 1),_4,_4,_4];
SetDiscriminant(_32, 3);
_7 = [_35,_35,_35,_35,_35,_35,_35,_35];
_32 = Adt44::Variant2 { fld0: _25,fld1: _4 };
place!(Field::<[u64; 8]>(Variant(_32, 2), 0)) = [13368846096489783205_u64,4335149170113671710_u64,9565737120444761426_u64,17397277329032406127_u64,5040994651832057169_u64,17280232427257490445_u64,14998065737808985599_u64,5720802154241651365_u64];
_29 = _20;
_9 = _26.0 << _39;
_35 = Field::<i8>(Variant(_32, 2), 1) as i128;
_37 = [_35,_35,_35,_35,_35,_35,_35,_35];
_45 = 304908641303387631255459941712217591663_u128 >> Field::<i8>(Variant(_32, 2), 1);
_22 = _9;
_13 = [_35,_35,_35,_35,_35,_35,_35,_35];
_23 = _22 << _22;
_6 = -_22;
_18 = _35 as f32;
place!(Field::<i8>(Variant(_32, 2), 1)) = -_4;
_5 = _40;
_36 = (_19, 3_usize, _26.1, _19);
_35 = 97856354930409827733838156528048156943_i128 << _36.1;
_47.1 = _33;
match _36.1 {
0 => bb6,
1 => bb11,
3 => bb13,
_ => bb12
}
}
bb11 = {
_5 = _8;
_20 = '\u{b4e7a}';
_7 = _11;
_19 = 16628_i16;
RET = [5164824941794957789_u64,13602533122969587566_u64];
_10 = [56483598507858159923202251532440759800_i128,107707797635947857362603769648327710036_i128,111011503510603470538074912347391862040_i128,82308906456793231882506235946417904284_i128,75049861603869333428308633796135591034_i128,9715869088542254213724264134444782030_i128,(-43524802514894521984855455713086139322_i128),134967533909778428033503745666491711359_i128];
_4 = (-77_i8) | 13_i8;
RET = [5604274206871103401_u64,7229332691975285438_u64];
Call(_6 = fn15(_1, _1, _5, _8, _11, _11, _17, _14, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_14 = _8;
_36.0 = _19;
_36 = (_19, 5_usize, RET, _19);
_31 = !_21;
_17 = 200_u8 as isize;
_2 = [135221311359885822890251499039440030075_i128,(-41360139748338128661089146641873586054_i128),(-123523102907826855135079977579420462644_i128),(-21011737016614725178968891953095652645_i128),(-99242675587669079592401454637545395036_i128),19643591535101037156036164174944230782_i128,(-71731689846807389832312368078271355369_i128),115026382278381099076453959699658186396_i128];
place!(Field::<[u64; 8]>(Variant(_32, 2), 0)) = _25;
_26.1 = [2761633255241128900_u64,4760816016895229785_u64];
_39 = (-3984236022825936681_i64) | (-2170315249824266623_i64);
_12 = [106210996188614392281963347918994722801_i128,(-34508058096774622551824260414240961197_i128),(-30563301511226358598997446341193056563_i128),81862689914244133031478488803823193065_i128,(-40504415531748035259052265701609366971_i128),(-60727584170508411416997880852909510396_i128),(-11270098219366655041923746018762359974_i128),22182527689683287703446870890007368849_i128];
_40 = [Field::<i8>(Variant(_32, 2), 1),_4,Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1)];
RET = [12691117334789924581_u64,13013880637931378769_u64];
Goto(bb8)
}
bb13 = {
_15 = _10;
_36.0 = _19 << _23;
_48 = !_23;
SetDiscriminant(_32, 3);
_3 = [_4,_4,_4,_4,_4,_4,_4];
_47.0 = _36.0 * _36.0;
_20 = _29;
_25 = [1854380767500861743_u64,17492701152891624811_u64,305368475042626333_u64,9922683521575244011_u64,9858662100970104515_u64,12587145422298677615_u64,2498343927012385997_u64,17170989952649208785_u64];
_44 = 59890_u16 as i8;
_46 = _34;
_50 = _31 ^ _21;
_26 = (_48, _36.2);
_31 = _50;
_7 = [_35,_35,_35,_35,_35,_35,_35,_35];
_13 = _1;
_11 = [_35,_35,_35,_35,_35,_35,_35,_35];
_7 = _16;
_10 = [_35,_35,_35,_35,_35,_35,_35,_35];
_47.0 = !_36.0;
_53 = _29;
_47.1 = [_47.0,_36.3,_36.0];
_50 = !_31;
_14 = [_44,_4,_44,_4,_44,_4,_44];
match _36.1 {
3 => bb15,
_ => bb14
}
}
bb14 = {
_14 = _8;
_36.0 = _19;
_36 = (_19, 5_usize, RET, _19);
_31 = !_21;
_17 = 200_u8 as isize;
_2 = [135221311359885822890251499039440030075_i128,(-41360139748338128661089146641873586054_i128),(-123523102907826855135079977579420462644_i128),(-21011737016614725178968891953095652645_i128),(-99242675587669079592401454637545395036_i128),19643591535101037156036164174944230782_i128,(-71731689846807389832312368078271355369_i128),115026382278381099076453959699658186396_i128];
place!(Field::<[u64; 8]>(Variant(_32, 2), 0)) = _25;
_26.1 = [2761633255241128900_u64,4760816016895229785_u64];
_39 = (-3984236022825936681_i64) | (-2170315249824266623_i64);
_12 = [106210996188614392281963347918994722801_i128,(-34508058096774622551824260414240961197_i128),(-30563301511226358598997446341193056563_i128),81862689914244133031478488803823193065_i128,(-40504415531748035259052265701609366971_i128),(-60727584170508411416997880852909510396_i128),(-11270098219366655041923746018762359974_i128),22182527689683287703446870890007368849_i128];
_40 = [Field::<i8>(Variant(_32, 2), 1),_4,Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1),Field::<i8>(Variant(_32, 2), 1)];
RET = [12691117334789924581_u64,13013880637931378769_u64];
Goto(bb8)
}
bb15 = {
_43 = _31;
place!(Field::<i32>(Variant(_32, 3), 0)) = !748760285_i32;
_36 = (_47.0, 1445579247620394236_usize, RET, _47.0);
RET = [18357122334404385184_u64,3316784022804354985_u64];
_6 = !_48;
_26 = (_6, _36.2);
_47.1 = _33;
_17 = _26.0 | _23;
_35 = 46809830780520069926176477732474272768_i128;
_36.2 = RET;
_23 = _6;
_53 = _29;
_11 = _7;
_51 = [_36.3,_36.3,_19,_36.3,_36.3,_47.0,_47.0,_47.0];
_53 = _20;
Goto(bb16)
}
bb16 = {
Call(_57 = dump_var(14_usize, 12_usize, Move(_12), 20_usize, Move(_20), 35_usize, Move(_35), 44_usize, Move(_44)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(14_usize, 43_usize, Move(_43), 23_usize, Move(_23), 40_usize, Move(_40), 45_usize, Move(_45)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_57 = dump_var(14_usize, 51_usize, Move(_51), 47_usize, Move(_47), 13_usize, Move(_13), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_57 = dump_var(14_usize, 6_usize, Move(_6), 4_usize, Move(_4), 3_usize, Move(_3), 29_usize, Move(_29)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_57 = dump_var(14_usize, 53_usize, Move(_53), 37_usize, Move(_37), 17_usize, Move(_17), 39_usize, Move(_39)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i8; 7],mut _4: [i8; 7],mut _5: [i128; 8],mut _6: [i128; 8],mut _7: isize,mut _8: [i8; 7],mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: i128;
let _11: i16;
let _12: Adt46;
let _13: i128;
let _14: Adt46;
let _15: char;
let _16: Adt54;
let _17: [u128; 6];
let _18: Adt51;
let _19: ([i8; 7], (u32,));
let _20: isize;
let _21: f64;
let _22: i64;
let _23: [i16; 8];
let _24: i8;
let _25: ([i8; 7], (u32,));
let _26: *const *mut [u128; 6];
let _27: Adt46;
let _28: u64;
let _29: [i8; 7];
let _30: Adt47;
let _31: (isize, [u64; 2]);
let _32: f64;
let _33: Adt41;
let _34: isize;
let _35: isize;
let _36: [i16; 3];
let _37: u32;
let _38: Adt41;
let _39: f64;
let _40: i16;
let _41: [i8; 7];
let _42: i64;
let _43: *mut [u128; 6];
let _44: u16;
let _45: i128;
let _46: char;
let _47: ((i16, [i16; 3]), &'static u8, ([i8; 7], f32, [i128; 8]), ([i8; 7], (u32,)));
let _48: ([i8; 7], (u32,));
let _49: *const *mut [u128; 6];
let _50: u8;
let _51: f64;
let _52: ();
let _53: ();
{
_3 = [77_i8,96_i8,(-2_i8),52_i8,(-110_i8),114_i8,64_i8];
RET = 33_i8 as isize;
RET = _7;
RET = _7;
_2 = [82606186836129551977322004247099550396_i128,46849920267407980855551609550231423337_i128,(-32747804621147480165880934834560186507_i128),(-48487142487254724010569452636191855553_i128),67044055812637924575889779275471954482_i128,137587721140520866580427833021601836092_i128,(-117344795705413577140445588149617029146_i128),117936739382581288426023516300005291444_i128];
_4 = [(-6_i8),87_i8,(-105_i8),91_i8,13_i8,(-52_i8),59_i8];
_7 = '\u{23e72}' as isize;
_7 = _9;
_9 = _7;
_7 = -_9;
_5 = [(-78433645249343073909716944057809011701_i128),(-75378765802633737696115270312172196030_i128),(-104383582492907681686108589809852135403_i128),(-92086159481144450892026482581069948152_i128),156177794723383791745924991673467138368_i128,(-92509528784064191160685114046800703681_i128),(-154530652203853830489524377435176055627_i128),(-140505391553677381633632118127537297307_i128)];
RET = _7 ^ _9;
_4 = [(-86_i8),23_i8,16_i8,(-99_i8),(-45_i8),109_i8,(-67_i8)];
Goto(bb1)
}
bb1 = {
_3 = _4;
_5 = [59939107758882835873504273033504886499_i128,76844964529889413073841442662865756688_i128,(-54260638599009922497259599227387515472_i128),157168086394514641511740015439996768434_i128,(-145472572303640711781278021337152729482_i128),168417864273046635150698179606301624135_i128,(-127144107522251363196527260285623271726_i128),97136099220381694081622573036812255033_i128];
_3 = _8;
RET = -_7;
_7 = _9;
_3 = [(-22_i8),(-79_i8),(-55_i8),122_i8,(-42_i8),(-90_i8),(-6_i8)];
_4 = [75_i8,(-68_i8),(-84_i8),(-114_i8),9_i8,(-86_i8),(-31_i8)];
_5 = [(-91658971928657335238491737816304205440_i128),99730986204576483398898146417172249062_i128,(-143117330776591628728706445008252036772_i128),(-43259969465123846693038057420057548631_i128),134122040199105350034484483920334927742_i128,52440218246682815357892622542888031199_i128,80460186276866335906076526584355118990_i128,141418447925309148660569997160107164378_i128];
_8 = [(-123_i8),30_i8,(-66_i8),(-107_i8),74_i8,38_i8,58_i8];
RET = _7;
_10 = 93074103213236645596969621365912928999_i128 - 142076774681182789610949856077591620406_i128;
_2 = _6;
_3 = [44_i8,86_i8,(-48_i8),27_i8,(-27_i8),(-84_i8),34_i8];
_6 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = -(-103266462215976643585821970874127281902_i128);
_11 = 6_usize as i16;
_1 = [_10,_10,_10,_10,_10,_10,_10,_10];
_8 = _4;
_10 = !76572882355038934275793782467836913569_i128;
_9 = 3_u8 as isize;
_10 = (-165849427613682226376711305837440163671_i128) | 66022239919380441725288904157129450144_i128;
_7 = !RET;
_4 = [37_i8,108_i8,(-106_i8),(-17_i8),(-121_i8),(-117_i8),(-80_i8)];
RET = false as isize;
_6 = [_10,_10,_10,_10,_10,_10,_10,_10];
Call(_13 = fn16(_4, _5, _2, _2, _10, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [(-82_i8),13_i8,(-88_i8),(-118_i8),88_i8,13_i8,(-113_i8)];
_9 = -_7;
_10 = _13;
_8 = [(-80_i8),(-103_i8),91_i8,62_i8,85_i8,113_i8,(-54_i8)];
_4 = [(-60_i8),102_i8,94_i8,77_i8,(-16_i8),(-70_i8),(-5_i8)];
_19.1 = (1596518650_u32,);
_7 = 685916337_i32 as isize;
_15 = '\u{86246}';
_17 = [285138765590272532345823367141052751618_u128,136636348980916803175632394576455026080_u128,309353495786745559490408141615671477182_u128,214046918919868960311331284666010334280_u128,85248730266279007367749505813851644237_u128,167935607225280980460585453053119068623_u128];
_9 = _7;
_13 = RET as i128;
_2 = [_13,_10,_13,_10,_10,_13,_10,_10];
Goto(bb3)
}
bb3 = {
_18.fld2 = 2_usize;
_19.1 = (525057221_u32,);
_3 = [(-62_i8),119_i8,73_i8,(-43_i8),(-37_i8),86_i8,(-104_i8)];
_7 = RET;
_15 = '\u{71fa4}';
_20 = 14314_u16 as isize;
_11 = !(-17563_i16);
_1 = _5;
_18.fld2 = !2_usize;
_19.0 = [50_i8,42_i8,(-9_i8),34_i8,(-83_i8),(-99_i8),(-122_i8)];
_11 = (-25025_i16);
_18.fld0 = core::ptr::addr_of!(_17);
Call(_13 = fn17(_18.fld0, _11, _5, _11, _17, _4, _19.1.0, _18.fld0, _8, _3, _20, _5, _5, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_22 = 5779562621456499594_i64;
_8 = [85_i8,33_i8,33_i8,75_i8,74_i8,38_i8,(-39_i8)];
_19.0 = [26_i8,30_i8,42_i8,(-73_i8),(-35_i8),(-77_i8),92_i8];
_9 = _11 as isize;
_6 = [_13,_13,_13,_10,_13,_10,_10,_10];
_20 = _19.1.0 as isize;
_19.0 = [86_i8,(-126_i8),40_i8,(-74_i8),(-39_i8),108_i8,(-53_i8)];
_19.1 = (1470195170_u32,);
_9 = (-1081969949_i32) as isize;
_15 = '\u{522a3}';
_25.1 = (_19.1.0,);
_18.fld3 = Adt45::Variant2 { fld0: 11396551614355004913_u64,fld1: _19,fld2: (-128192753_i32) };
place!(Field::<u64>(Variant(_18.fld3, 2), 0)) = 10435709225205295227_u64 + 11014345584168292470_u64;
_7 = _20;
_4 = _3;
_18.fld0 = core::ptr::addr_of!(_17);
place!(Field::<i32>(Variant(_18.fld3, 2), 2)) = 678384529_i32 & (-2075072507_i32);
_25.0 = [(-18_i8),36_i8,60_i8,(-6_i8),102_i8,105_i8,(-95_i8)];
SetDiscriminant(_18.fld3, 2);
_23 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = _22 as i16;
_25.1 = _19.1;
_20 = _7 + _9;
match _25.1.0 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
1470195170 => bb9,
_ => bb8
}
}
bb5 = {
_18.fld2 = 2_usize;
_19.1 = (525057221_u32,);
_3 = [(-62_i8),119_i8,73_i8,(-43_i8),(-37_i8),86_i8,(-104_i8)];
_7 = RET;
_15 = '\u{71fa4}';
_20 = 14314_u16 as isize;
_11 = !(-17563_i16);
_1 = _5;
_18.fld2 = !2_usize;
_19.0 = [50_i8,42_i8,(-9_i8),34_i8,(-83_i8),(-99_i8),(-122_i8)];
_11 = (-25025_i16);
_18.fld0 = core::ptr::addr_of!(_17);
Call(_13 = fn17(_18.fld0, _11, _5, _11, _17, _4, _19.1.0, _18.fld0, _8, _3, _20, _5, _5, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_4 = [(-82_i8),13_i8,(-88_i8),(-118_i8),88_i8,13_i8,(-113_i8)];
_9 = -_7;
_10 = _13;
_8 = [(-80_i8),(-103_i8),91_i8,62_i8,85_i8,113_i8,(-54_i8)];
_4 = [(-60_i8),102_i8,94_i8,77_i8,(-16_i8),(-70_i8),(-5_i8)];
_19.1 = (1596518650_u32,);
_7 = 685916337_i32 as isize;
_15 = '\u{86246}';
_17 = [285138765590272532345823367141052751618_u128,136636348980916803175632394576455026080_u128,309353495786745559490408141615671477182_u128,214046918919868960311331284666010334280_u128,85248730266279007367749505813851644237_u128,167935607225280980460585453053119068623_u128];
_9 = _7;
_13 = RET as i128;
_2 = [_13,_10,_13,_10,_10,_13,_10,_10];
Goto(bb3)
}
bb7 = {
_3 = _4;
_5 = [59939107758882835873504273033504886499_i128,76844964529889413073841442662865756688_i128,(-54260638599009922497259599227387515472_i128),157168086394514641511740015439996768434_i128,(-145472572303640711781278021337152729482_i128),168417864273046635150698179606301624135_i128,(-127144107522251363196527260285623271726_i128),97136099220381694081622573036812255033_i128];
_3 = _8;
RET = -_7;
_7 = _9;
_3 = [(-22_i8),(-79_i8),(-55_i8),122_i8,(-42_i8),(-90_i8),(-6_i8)];
_4 = [75_i8,(-68_i8),(-84_i8),(-114_i8),9_i8,(-86_i8),(-31_i8)];
_5 = [(-91658971928657335238491737816304205440_i128),99730986204576483398898146417172249062_i128,(-143117330776591628728706445008252036772_i128),(-43259969465123846693038057420057548631_i128),134122040199105350034484483920334927742_i128,52440218246682815357892622542888031199_i128,80460186276866335906076526584355118990_i128,141418447925309148660569997160107164378_i128];
_8 = [(-123_i8),30_i8,(-66_i8),(-107_i8),74_i8,38_i8,58_i8];
RET = _7;
_10 = 93074103213236645596969621365912928999_i128 - 142076774681182789610949856077591620406_i128;
_2 = _6;
_3 = [44_i8,86_i8,(-48_i8),27_i8,(-27_i8),(-84_i8),34_i8];
_6 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = -(-103266462215976643585821970874127281902_i128);
_11 = 6_usize as i16;
_1 = [_10,_10,_10,_10,_10,_10,_10,_10];
_8 = _4;
_10 = !76572882355038934275793782467836913569_i128;
_9 = 3_u8 as isize;
_10 = (-165849427613682226376711305837440163671_i128) | 66022239919380441725288904157129450144_i128;
_7 = !RET;
_4 = [37_i8,108_i8,(-106_i8),(-17_i8),(-121_i8),(-117_i8),(-80_i8)];
RET = false as isize;
_6 = [_10,_10,_10,_10,_10,_10,_10,_10];
Call(_13 = fn16(_4, _5, _2, _2, _10, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_24 = (-115_i8) << _20;
_11 = (-2007724957_i32) as i16;
_20 = _7 + _7;
_6 = _1;
_29 = [_24,_24,_24,_24,_24,_24,_24];
_22 = 1901590408843086971_i64;
_25 = (_4, _19.1);
_7 = -_9;
place!(Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1)).1 = _25.1;
_9 = 6361550283372277380_u64 as isize;
_28 = 10811860818645333815_u64 ^ 16412542786253401656_u64;
_25.1.0 = _7 as u32;
_17 = [291305324517785141211318365024399107852_u128,235833284875348789973721643984306851014_u128,100927512201390390709162216132979081203_u128,237036824219019572162119988697177027338_u128,335263034538190190876664478688570065730_u128,180155296780346112486929712259062488424_u128];
_3 = [_24,_24,_24,_24,_24,_24,_24];
_21 = _24 as f64;
_3 = [_24,_24,_24,_24,_24,_24,_24];
_2 = [_10,_13,_13,_10,_13,_10,_10,_10];
_9 = _7;
_1 = _5;
_32 = _21;
_31.1 = [_28,_28];
Goto(bb10)
}
bb10 = {
place!(Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1)) = _19;
_2 = [_13,_10,_10,_13,_10,_10,_10,_13];
_6 = [_13,_13,_10,_13,_10,_10,_10,_13];
place!(Field::<i32>(Variant(_18.fld3, 2), 2)) = -(-8925357_i32);
_21 = _19.1.0 as f64;
place!(Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1)).1.0 = _25.1.0;
_20 = -_9;
_25 = (Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1).0, _19.1);
RET = !_20;
_25.1 = (_19.1.0,);
place!(Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1)).0 = [_24,_24,_24,_24,_24,_24,_24];
_7 = !_20;
RET = Field::<i32>(Variant(_18.fld3, 2), 2) as isize;
place!(Field::<u64>(Variant(_18.fld3, 2), 0)) = _28 & _28;
_31.0 = _9;
_7 = -_31.0;
_20 = _7 >> RET;
_4 = [_24,_24,_24,_24,_24,_24,_24];
SetDiscriminant(_18.fld3, 1);
place!(Field::<[i8; 7]>(Variant(_18.fld3, 1), 3)) = [_24,_24,_24,_24,_24,_24,_24];
_10 = _13 << _13;
_4 = _19.0;
_25.1.0 = !_19.1.0;
_10 = _13 | _13;
Goto(bb11)
}
bb11 = {
_19 = (_8, _25.1);
_20 = _9 + _7;
_3 = _19.0;
RET = -_31.0;
place!(Field::<u32>(Variant(_18.fld3, 1), 1)) = _19.1.0 << _20;
place!(Field::<u32>(Variant(_18.fld3, 1), 1)) = _25.1.0;
_19.0 = _8;
_15 = '\u{8d729}';
_25 = _19;
place!(Field::<isize>(Variant(_18.fld3, 1), 2)) = _31.0;
_31.1 = [_28,_28];
_8 = _25.0;
_28 = 3489045904666796593_u64 & 9230202694134062059_u64;
_2 = [_10,_13,_10,_10,_10,_10,_10,_13];
_8 = [_24,_24,_24,_24,_24,_24,_24];
_20 = _9;
_25 = (Field::<[i8; 7]>(Variant(_18.fld3, 1), 3), _19.1);
Goto(bb12)
}
bb12 = {
_22 = 57244_u16 as i64;
_2 = [_10,_10,_10,_10,_10,_10,_10,_13];
place!(Field::<[i8; 7]>(Variant(_18.fld3, 1), 3)) = [_24,_24,_24,_24,_24,_24,_24];
RET = -_20;
_22 = 63744968354015778460903875765080163939_u128 as i64;
_32 = _11 as f64;
_36 = [_11,_11,_11];
_4 = [_24,_24,_24,_24,_24,_24,_24];
_37 = _25.1.0 | _19.1.0;
_25.0 = [_24,_24,_24,_24,_24,_24,_24];
place!(Field::<isize>(Variant(_18.fld3, 1), 2)) = -_20;
_22 = 1990805402902740953_i64;
_4 = [_24,_24,_24,_24,_24,_24,_24];
_9 = Field::<isize>(Variant(_18.fld3, 1), 2) | RET;
_41 = _8;
_31.0 = _20 | _20;
_9 = _31.0;
_32 = _21 + _21;
_26 = core::ptr::addr_of!(_43);
_25.0 = [_24,_24,_24,_24,_24,_24,_24];
_18.fld2 = 6_usize ^ 14620313406582370972_usize;
_25.1 = (_37,);
match _13 {
0 => bb1,
1 => bb8,
2 => bb7,
306481533882002022023725336857641207171 => bb13,
_ => bb4
}
}
bb13 = {
_35 = _7 >> _25.1.0;
_24 = _13 as i8;
_29 = _25.0;
_2 = _5;
_32 = _21;
_21 = _32;
_32 = -_21;
_9 = -_7;
_32 = _28 as f64;
_29 = [_24,_24,_24,_24,_24,_24,_24];
RET = -Field::<isize>(Variant(_18.fld3, 1), 2);
_47.3.0 = _3;
_36 = [_11,_11,_11];
_47.2.2 = _2;
_47.0 = (_11, _36);
_18.fld0 = core::ptr::addr_of!(_17);
_47.3.1.0 = _37 >> _9;
(*_26) = core::ptr::addr_of_mut!(_17);
_47.1 = &_50;
_47.0.1 = [_11,_47.0.0,_47.0.0];
_47.2.1 = 193481488243077183546949345464694588074_u128 as f32;
_19 = (_47.3.0, _47.3.1);
_45 = true as i128;
Goto(bb14)
}
bb14 = {
_7 = _35;
_20 = !_31.0;
_34 = _47.0.0 as isize;
_18.fld2 = _47.3.1.0 as usize;
RET = _20;
_47.0 = (_11, _36);
_47.3.1 = (_19.1.0,);
_48.1 = (_47.3.1.0,);
(*_26) = core::ptr::addr_of_mut!(_17);
_47.2.1 = _28 as f32;
_7 = -RET;
_18.fld3 = Adt45::Variant2 { fld0: _28,fld1: _47.3,fld2: (-576518008_i32) };
_47.1 = &_50;
_31.1 = [_28,_28];
RET = _7;
_23 = [_11,_11,_47.0.0,_47.0.0,_11,_11,_11,_11];
_46 = _15;
_42 = !_22;
place!(Field::<([i8; 7], (u32,))>(Variant(_18.fld3, 2), 1)).1 = (_19.1.0,);
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(15_usize, 35_usize, Move(_35), 10_usize, Move(_10), 36_usize, Move(_36), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(15_usize, 11_usize, Move(_11), 45_usize, Move(_45), 42_usize, Move(_42), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_52 = dump_var(15_usize, 19_usize, Move(_19), 2_usize, Move(_2), 4_usize, Move(_4), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_52 = dump_var(15_usize, 5_usize, Move(_5), 28_usize, Move(_28), 41_usize, Move(_41), 53_usize, _53), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i8; 7],mut _2: [i128; 8],mut _3: [i128; 8],mut _4: [i128; 8],mut _5: i128,mut _6: [i8; 7]) -> i128 {
mir! {
type RET = i128;
let _7: Adt54;
let _8: i64;
let _9: f32;
let _10: Adt43;
let _11: Adt40;
let _12: [i16; 3];
let _13: f32;
let _14: f64;
let _15: char;
let _16: i16;
let _17: &'static u8;
let _18: [u64; 2];
let _19: [u128; 6];
let _20: Adt44;
let _21: ();
let _22: ();
{
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = _5;
_6 = _1;
RET = 89_u16 as i128;
_4 = _2;
_6 = _1;
_8 = (-9223372036854775808_isize) as i64;
_6 = _1;
Goto(bb1)
}
bb1 = {
_8 = 8590177991693060240_i64 & 1973898265066149215_i64;
_4 = _3;
_9 = 24_i8 as f32;
_4 = _2;
_9 = 251_u8 as f32;
RET = !_5;
_6 = _1;
_3 = [RET,_5,RET,_5,RET,_5,_5,_5];
_3 = [_5,_5,RET,RET,RET,RET,RET,RET];
_3 = [_5,RET,_5,RET,_5,RET,_5,RET];
_5 = !RET;
RET = !_5;
_6 = [44_i8,(-66_i8),80_i8,36_i8,12_i8,126_i8,(-41_i8)];
_11 = Adt40::Variant1 { fld0: 3713372001832967774_u64,fld1: 1884798609354339139_usize,fld2: (-27680_i16) };
_5 = !RET;
place!(Field::<i16>(Variant(_11, 1), 2)) = (-18711_i16) | 15180_i16;
place!(Field::<i16>(Variant(_11, 1), 2)) = 1944529465_u32 as i16;
_1 = [54_i8,(-108_i8),4_i8,(-107_i8),(-62_i8),63_i8,(-103_i8)];
place!(Field::<usize>(Variant(_11, 1), 1)) = 7_usize;
_2 = [_5,_5,RET,_5,RET,RET,RET,RET];
_8 = (-5350108079660512745_i64) & (-8883910143136319464_i64);
_6 = [90_i8,(-77_i8),(-99_i8),27_i8,27_i8,3_i8,(-100_i8)];
match Field::<usize>(Variant(_11, 1), 1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7 => bb10,
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
_14 = RET as f64;
_13 = 105_i8 as f32;
RET = !_5;
_12 = [Field::<i16>(Variant(_11, 1), 2),Field::<i16>(Variant(_11, 1), 2),Field::<i16>(Variant(_11, 1), 2)];
RET = (-1891483454_i32) as i128;
_6 = [(-29_i8),(-78_i8),(-124_i8),94_i8,(-126_i8),(-100_i8),(-99_i8)];
_5 = 143_u8 as i128;
_1 = [(-97_i8),10_i8,(-30_i8),45_i8,51_i8,(-14_i8),(-72_i8)];
_1 = [116_i8,(-68_i8),86_i8,(-48_i8),122_i8,98_i8,(-3_i8)];
place!(Field::<usize>(Variant(_11, 1), 1)) = !11609186429435137633_usize;
_11 = Adt40::Variant1 { fld0: 1906562776705208662_u64,fld1: 8895099262287838628_usize,fld2: (-24364_i16) };
_11 = Adt40::Variant1 { fld0: 12281248509080750027_u64,fld1: 7524367329389089811_usize,fld2: (-23380_i16) };
_3 = _4;
Goto(bb11)
}
bb11 = {
_1 = _6;
_5 = -RET;
place!(Field::<i16>(Variant(_11, 1), 2)) = !(-16575_i16);
_16 = Field::<i16>(Variant(_11, 1), 2) & Field::<i16>(Variant(_11, 1), 2);
_13 = _8 as f32;
_8 = 4194043534_u32 as i64;
_14 = 2_usize as f64;
_2 = [_5,RET,_5,_5,RET,_5,RET,RET];
_15 = '\u{c0f16}';
_8 = -(-2400040053625011323_i64);
_9 = _13 + _13;
_13 = _9 + _9;
RET = _5;
place!(Field::<usize>(Variant(_11, 1), 1)) = !2_usize;
_4 = [RET,_5,_5,RET,RET,RET,RET,_5];
_3 = _4;
Goto(bb12)
}
bb12 = {
place!(Field::<u64>(Variant(_11, 1), 0)) = 3740320069610910911_u64;
Goto(bb13)
}
bb13 = {
RET = _5;
_4 = [_5,RET,RET,_5,RET,RET,_5,RET];
_9 = _13 * _13;
place!(Field::<u64>(Variant(_11, 1), 0)) = 10539614008487346978_u64 >> Field::<usize>(Variant(_11, 1), 1);
SetDiscriminant(_11, 1);
_12 = [_16,_16,_16];
_16 = (-32577_i16) << _5;
place!(Field::<u64>(Variant(_11, 1), 0)) = (-43_i8) as u64;
Goto(bb14)
}
bb14 = {
_19 = [206540401359101465365506396387399166351_u128,196669343118848364073233884585917816312_u128,148243670873776355042251607661193520186_u128,209643835178077429266029808235888499851_u128,61992522215614884395841774733455351822_u128,284946650139125210365676617130891508519_u128];
_6 = [64_i8,43_i8,57_i8,124_i8,(-98_i8),(-27_i8),(-1_i8)];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(16_usize, 2_usize, Move(_2), 3_usize, Move(_3), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(16_usize, 8_usize, Move(_8), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *const [u128; 6],mut _2: i16,mut _3: [i128; 8],mut _4: i16,mut _5: [u128; 6],mut _6: [i8; 7],mut _7: u32,mut _8: *const [u128; 6],mut _9: [i8; 7],mut _10: [i8; 7],mut _11: isize,mut _12: [i128; 8],mut _13: [i128; 8],mut _14: [i128; 8]) -> i128 {
mir! {
type RET = i128;
let _15: [u64; 8];
let _16: isize;
let _17: f32;
let _18: isize;
let _19: bool;
let _20: isize;
let _21: f32;
let _22: [i128; 8];
let _23: f32;
let _24: i16;
let _25: bool;
let _26: ([i8; 7], f32, [i128; 8]);
let _27: (i16, usize, [u64; 2], i16);
let _28: i32;
let _29: [i8; 7];
let _30: Adt43;
let _31: [u128; 6];
let _32: Adt47;
let _33: bool;
let _34: u128;
let _35: f32;
let _36: f64;
let _37: i16;
let _38: *mut u16;
let _39: isize;
let _40: usize;
let _41: [u64; 8];
let _42: (u32,);
let _43: Adt53;
let _44: char;
let _45: ([i8; 7], f32, [i128; 8]);
let _46: i128;
let _47: bool;
let _48: ();
let _49: ();
{
_7 = !3414338654_u32;
(*_8) = _5;
_14 = [123065993583063142807109173481669078491_i128,162666770833245695662293808921444190641_i128,(-25946002258129234185176972834009780143_i128),(-9039059359640146380127751037766002054_i128),145724168467953006714978402034328217783_i128,(-118576220975549106442004600754178431879_i128),(-133551319239300704191062291924205059427_i128),(-155565471051371461374701989438711777484_i128)];
(*_1) = [124632045479613102363575576755960717774_u128,227421733714088107403604231377459288052_u128,307962504885235163183666347188966710629_u128,211062321595318629139849082115706748897_u128,248142338439765804259086489447998217582_u128,246683495286921454059473296831536944856_u128];
RET = -(-156393940239783945517893142769576853186_i128);
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = _8;
Call(_6 = core::intrinsics::transmute(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [192908208671557626941909964615030242466_u128,141613626072016313833274454731901623816_u128,109928625034573161892947123898807121265_u128,315844182949366658044913548875207771374_u128,328247120520545318281428863893525065746_u128,269142811561803638566853565993259738022_u128];
_9 = [(-34_i8),(-97_i8),(-32_i8),39_i8,(-63_i8),15_i8,26_i8];
(*_8) = _5;
RET = 9439_u16 as i128;
_9 = _10;
_11 = -(-9223372036854775808_isize);
(*_1) = [291047759472574766692654784315270258226_u128,264773659581655459681015288939110057870_u128,235991560548991516730900323026623200322_u128,284173280638893825143898374619213633543_u128,170247968619473566736353715714716842064_u128,170854736606174356188859249267719551877_u128];
_15 = [12830049699228653924_u64,1854957716999402963_u64,3043845692118062453_u64,6921498927298517922_u64,14594080442755482301_u64,11481711623558578338_u64,11380534510307984928_u64,16350045769276687568_u64];
(*_8) = [142967416029050901901006288466847385292_u128,158346522401862851316131553035637887785_u128,333249624461664261662202337808927018120_u128,320530616868539815029158272613047010055_u128,223936345618044953110516270995415761931_u128,240923899227011204381678867179077803211_u128];
(*_1) = [51397429086873089297818613989027622488_u128,67359944112504431226411304081866432385_u128,206329369205129642228435217388651987466_u128,253228348762606904461510936679750375184_u128,237994980382375637902091673850608179320_u128,66795238369706408438147675853700733505_u128];
_17 = _11 as f32;
(*_8) = [11482415772572001638210416931580989462_u128,133267728989401061376841275037656031376_u128,28304452999275308271127617953625710811_u128,196821104788559937687622501635114896842_u128,337475873315227043376928697475160037109_u128,41433963922324246211486082865541773333_u128];
_16 = _11;
_17 = (-96_i8) as f32;
_16 = _11;
_6 = [(-46_i8),(-29_i8),21_i8,(-9_i8),(-123_i8),(-92_i8),(-2_i8)];
_3 = _13;
(*_8) = [2533008723488780524664328627010546011_u128,296989905784050049376710036162799683569_u128,260421038329586563904029701150112302780_u128,156730526184455334840685744902070693022_u128,54910419597948580502105999804343381537_u128,268509132375504409841462599958008260287_u128];
_16 = '\u{346b2}' as isize;
_15 = [9811778597605040174_u64,10484504963505775301_u64,6277002143821645381_u64,14790485665507492348_u64,9289262592614420422_u64,15362091304384757124_u64,748894117427396701_u64,4997913015419436926_u64];
(*_1) = [160464887863750388753786801929993723425_u128,329060154622078043575100943880268864437_u128,110120758562922421750110853444786703303_u128,78680483935948957002946069458283455399_u128,20696223897070546745907908223443190533_u128,202135165889530300292283814976271303426_u128];
RET = (-1687880422476163665148944279502120246_i128) ^ (-91128966456430574048063953886531853312_i128);
Goto(bb2)
}
bb2 = {
_5 = (*_1);
_20 = _16;
_18 = 16035771118838628766_u64 as isize;
(*_1) = [250986553454016207564960867485104065155_u128,198009251304042070330306479756142229639_u128,40824599095443060898800103072837381561_u128,155775616543041458165196174575485180341_u128,311857993562629871845043793410280663652_u128,337936338836639958287207565866039219993_u128];
_1 = _8;
_10 = [68_i8,(-47_i8),(-86_i8),73_i8,6_i8,62_i8,121_i8];
Call(_16 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = (-167489925169369401258258777146462859464_i128);
_12 = _14;
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = 93113856275749943693400152312139418507_i128;
_19 = !true;
_8 = _1;
RET = (-43_i8) as i128;
(*_1) = _5;
_7 = 1740581533_u32 - 1299828422_u32;
_19 = false ^ false;
_8 = _1;
(*_8) = _5;
Goto(bb4)
}
bb4 = {
_12 = _14;
_4 = _2;
_7 = !1363762488_u32;
_21 = 237480709740442914075570827997264743996_u128 as f32;
(*_8) = _5;
(*_8) = [47218996938302660042630450858669789793_u128,52053446060573790103474428428351478985_u128,98250339791826317942933673814801821779_u128,12123427752929482805832039905360017854_u128,239902876753549103911447032214087502672_u128,274152878689082845262324965707936235987_u128];
RET = _7 as i128;
_2 = _4 ^ _4;
_24 = -_2;
_22 = _12;
_18 = (-53_i8) as isize;
(*_1) = _5;
_6 = [(-39_i8),87_i8,(-38_i8),127_i8,106_i8,(-122_i8),81_i8];
RET = -52635910420643013481961969836158531117_i128;
_16 = _11 + _11;
_2 = _24 ^ _4;
_9 = [(-19_i8),(-90_i8),35_i8,96_i8,(-80_i8),31_i8,(-35_i8)];
(*_1) = _5;
_21 = _17 + _17;
_24 = -_2;
_16 = _20;
(*_1) = [12674114343041498671611193619670498018_u128,315992143005604631484326037974278367312_u128,133780183564487236974440587935481845167_u128,190348663190521037257598253311653204843_u128,326120822170695828031634498578978680874_u128,140647544983168124521934123609534231129_u128];
Call(_12 = core::intrinsics::transmute(_22), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_8) = _5;
_12 = _22;
_26.1 = _17 - _21;
_17 = _21;
_26.0 = _6;
_26 = (_10, _21, _12);
(*_8) = [272314433147887864444112933584123604602_u128,141303519234569002833925070080788608728_u128,306152384831314295276499585169689298682_u128,330408358169011369014607701559120285488_u128,200556069910110348308108683648485949889_u128,24408437498205315032328777989990538634_u128];
_17 = 31_u8 as f32;
_22 = _14;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = _1;
_25 = !_19;
_24 = _4 * _2;
_20 = _18 | _16;
(*_1) = [110743479222970736576421481275084199423_u128,67526914237471304506185514272781355456_u128,7771594097303392684462135433421838262_u128,185035785491269651493287713036303383587_u128,189934903937820009448253661790088360335_u128,334632799057703997159903087712445776356_u128];
_27.3 = _24;
_27.1 = !2_usize;
_2 = !_24;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_6 = _10;
_8 = _1;
match _4 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
340282366920938463463374607431768186431 => bb9,
_ => bb8
}
}
bb6 = {
_12 = _14;
_4 = _2;
_7 = !1363762488_u32;
_21 = 237480709740442914075570827997264743996_u128 as f32;
(*_8) = _5;
(*_8) = [47218996938302660042630450858669789793_u128,52053446060573790103474428428351478985_u128,98250339791826317942933673814801821779_u128,12123427752929482805832039905360017854_u128,239902876753549103911447032214087502672_u128,274152878689082845262324965707936235987_u128];
RET = _7 as i128;
_2 = _4 ^ _4;
_24 = -_2;
_22 = _12;
_18 = (-53_i8) as isize;
(*_1) = _5;
_6 = [(-39_i8),87_i8,(-38_i8),127_i8,106_i8,(-122_i8),81_i8];
RET = -52635910420643013481961969836158531117_i128;
_16 = _11 + _11;
_2 = _24 ^ _4;
_9 = [(-19_i8),(-90_i8),35_i8,96_i8,(-80_i8),31_i8,(-35_i8)];
(*_1) = _5;
_21 = _17 + _17;
_24 = -_2;
_16 = _20;
(*_1) = [12674114343041498671611193619670498018_u128,315992143005604631484326037974278367312_u128,133780183564487236974440587935481845167_u128,190348663190521037257598253311653204843_u128,326120822170695828031634498578978680874_u128,140647544983168124521934123609534231129_u128];
Call(_12 = core::intrinsics::transmute(_22), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_5 = [192908208671557626941909964615030242466_u128,141613626072016313833274454731901623816_u128,109928625034573161892947123898807121265_u128,315844182949366658044913548875207771374_u128,328247120520545318281428863893525065746_u128,269142811561803638566853565993259738022_u128];
_9 = [(-34_i8),(-97_i8),(-32_i8),39_i8,(-63_i8),15_i8,26_i8];
(*_8) = _5;
RET = 9439_u16 as i128;
_9 = _10;
_11 = -(-9223372036854775808_isize);
(*_1) = [291047759472574766692654784315270258226_u128,264773659581655459681015288939110057870_u128,235991560548991516730900323026623200322_u128,284173280638893825143898374619213633543_u128,170247968619473566736353715714716842064_u128,170854736606174356188859249267719551877_u128];
_15 = [12830049699228653924_u64,1854957716999402963_u64,3043845692118062453_u64,6921498927298517922_u64,14594080442755482301_u64,11481711623558578338_u64,11380534510307984928_u64,16350045769276687568_u64];
(*_8) = [142967416029050901901006288466847385292_u128,158346522401862851316131553035637887785_u128,333249624461664261662202337808927018120_u128,320530616868539815029158272613047010055_u128,223936345618044953110516270995415761931_u128,240923899227011204381678867179077803211_u128];
(*_1) = [51397429086873089297818613989027622488_u128,67359944112504431226411304081866432385_u128,206329369205129642228435217388651987466_u128,253228348762606904461510936679750375184_u128,237994980382375637902091673850608179320_u128,66795238369706408438147675853700733505_u128];
_17 = _11 as f32;
(*_8) = [11482415772572001638210416931580989462_u128,133267728989401061376841275037656031376_u128,28304452999275308271127617953625710811_u128,196821104788559937687622501635114896842_u128,337475873315227043376928697475160037109_u128,41433963922324246211486082865541773333_u128];
_16 = _11;
_17 = (-96_i8) as f32;
_16 = _11;
_6 = [(-46_i8),(-29_i8),21_i8,(-9_i8),(-123_i8),(-92_i8),(-2_i8)];
_3 = _13;
(*_8) = [2533008723488780524664328627010546011_u128,296989905784050049376710036162799683569_u128,260421038329586563904029701150112302780_u128,156730526184455334840685744902070693022_u128,54910419597948580502105999804343381537_u128,268509132375504409841462599958008260287_u128];
_16 = '\u{346b2}' as isize;
_15 = [9811778597605040174_u64,10484504963505775301_u64,6277002143821645381_u64,14790485665507492348_u64,9289262592614420422_u64,15362091304384757124_u64,748894117427396701_u64,4997913015419436926_u64];
(*_1) = [160464887863750388753786801929993723425_u128,329060154622078043575100943880268864437_u128,110120758562922421750110853444786703303_u128,78680483935948957002946069458283455399_u128,20696223897070546745907908223443190533_u128,202135165889530300292283814976271303426_u128];
RET = (-1687880422476163665148944279502120246_i128) ^ (-91128966456430574048063953886531853312_i128);
Goto(bb2)
}
bb8 = {
_5 = (*_1);
_20 = _16;
_18 = 16035771118838628766_u64 as isize;
(*_1) = [250986553454016207564960867485104065155_u128,198009251304042070330306479756142229639_u128,40824599095443060898800103072837381561_u128,155775616543041458165196174575485180341_u128,311857993562629871845043793410280663652_u128,337936338836639958287207565866039219993_u128];
_1 = _8;
_10 = [68_i8,(-47_i8),(-86_i8),73_i8,6_i8,62_i8,121_i8];
Call(_16 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_1 = _8;
_15 = [2351190533723541897_u64,13831943002822848654_u64,379833230858966164_u64,12179000442808498212_u64,10512355781803353585_u64,3870081036146361516_u64,8486572302755327826_u64,12370877752278912753_u64];
_27.0 = _2;
_28 = 562689418_i32 | 1762273610_i32;
_11 = _24 as isize;
_6 = [89_i8,(-16_i8),(-28_i8),92_i8,(-4_i8),(-6_i8),(-52_i8)];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = _11 - _11;
_24 = _27.1 as i16;
_27.1 = !3_usize;
_23 = -_17;
_27.2 = [8773510537298447773_u64,5253432995615070218_u64];
(*_8) = _5;
_27.0 = -_2;
(*_8) = [4217009071520314821152412218381675883_u128,242943170037143003438516076004774621879_u128,80064443863480398440143775613816166056_u128,11109320979186243574902750423424398890_u128,180796521056167430084052264728363245159_u128,292453942904985616917480131061528130926_u128];
_16 = _26.1 as isize;
_23 = _26.1 * _26.1;
_7 = '\u{62f05}' as u32;
_26.1 = _23 * _23;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_15 = [10034526859225848891_u64,17589100886249768766_u64,13358261970223824467_u64,9277976361796222323_u64,6262926100981566071_u64,14778349072607172489_u64,9743178167396390784_u64,12911819968828026641_u64];
_2 = _19 as i16;
_27.3 = _4;
Call(_17 = core::intrinsics::transmute(_28), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_23 = _17;
(*_8) = [57132326320964233071791873348790096371_u128,10563957783055883088616862084410336511_u128,162748135945650039540731523525876118673_u128,212715146978379915853558734614613854146_u128,285532445988460166804062320304191183938_u128,164775668911644091469933891292333629025_u128];
_24 = _27.1 as i16;
_27.1 = 1676952496623678074_u64 as usize;
_18 = _16;
_27.1 = _7 as usize;
_31 = (*_1);
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_21 = (-91_i8) as f32;
(*_8) = [101941715467754959211258818958991895329_u128,173159533480130203742496559593958759239_u128,246593572018939368559139088817984388718_u128,214452570676304749480515982320500714103_u128,310625110060111133031538082298772482042_u128,328174217082366181281812886717018737349_u128];
_9 = [95_i8,1_i8,30_i8,(-67_i8),102_i8,19_i8,107_i8];
_2 = 207_u8 as i16;
Goto(bb11)
}
bb11 = {
_27.0 = 16786133071306424525_u64 as i16;
_8 = core::ptr::addr_of!(_31);
_35 = _21 + _21;
_24 = -_4;
_10 = [(-103_i8),(-115_i8),66_i8,8_i8,103_i8,(-117_i8),(-121_i8)];
_29 = [61_i8,(-61_i8),(-10_i8),(-53_i8),(-50_i8),12_i8,11_i8];
_23 = -_26.1;
_36 = 5473843663192094122_u64 as f64;
_16 = 2250348234864815725_i64 as isize;
_8 = _1;
_33 = _26.1 == _26.1;
Goto(bb12)
}
bb12 = {
_26.2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = 1731611735_u32 | 2340015441_u32;
_26.1 = _17;
_39 = _11;
(*_1) = _31;
_3 = _22;
_39 = _20 * _18;
_24 = !_27.3;
(*_8) = [67926979220027003793448487487790955861_u128,286540872604582357033313407686436330381_u128,155305359927890384951465554677587605910_u128,319807111007581926661528899648142429231_u128,189360970001887171284168874674147653053_u128,319933318205821727836663535670136238341_u128];
_9 = [(-80_i8),56_i8,87_i8,45_i8,44_i8,(-11_i8),98_i8];
_27.0 = 127_i8 as i16;
match _4 {
0 => bb10,
1 => bb5,
2 => bb9,
340282366920938463463374607431768186431 => bb13,
_ => bb4
}
}
bb13 = {
(*_8) = [85190935097365444986720411744100601763_u128,304087987721561540875766260859963002363_u128,265979789805323130520500233394025266494_u128,204567267828643538778472173948087609064_u128,120961145377246615326858996470825321028_u128,133499011295088326070923920503123706483_u128];
_24 = !_27.0;
_14 = _3;
_19 = !_33;
_20 = _23 as isize;
_39 = _20 * _11;
_37 = 97_u8 as i16;
_26 = (_9, _23, _14);
_26.1 = 5516059046136003172_i64 as f32;
_40 = !_27.1;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_29 = [(-3_i8),47_i8,(-82_i8),(-14_i8),(-123_i8),(-16_i8),(-78_i8)];
_16 = _19 as isize;
_36 = RET as f64;
_25 = _19;
_9 = [116_i8,25_i8,44_i8,0_i8,(-118_i8),(-11_i8),(-31_i8)];
_17 = -_23;
_33 = _19 == _25;
_24 = _27.3;
RET = (-43291841781475702725214728125378736472_i128);
_22 = _3;
_23 = RET as f32;
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_34 = 183811229069508689951123717426916422444_u128 >> _7;
_42 = (_7,);
_27.0 = -_27.3;
_41 = [6477181015475471313_u64,15751935635256598664_u64,18004096013139373613_u64,380113994222166736_u64,9744256062239610342_u64,18264266292971156655_u64,1040021816040239299_u64,7835296068265306131_u64];
match RET {
0 => bb11,
1 => bb12,
2 => bb14,
296990525139462760738159879306389474984 => bb16,
_ => bb15
}
}
bb14 = {
_26.2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = 1731611735_u32 | 2340015441_u32;
_26.1 = _17;
_39 = _11;
(*_1) = _31;
_3 = _22;
_39 = _20 * _18;
_24 = !_27.3;
(*_8) = [67926979220027003793448487487790955861_u128,286540872604582357033313407686436330381_u128,155305359927890384951465554677587605910_u128,319807111007581926661528899648142429231_u128,189360970001887171284168874674147653053_u128,319933318205821727836663535670136238341_u128];
_9 = [(-80_i8),56_i8,87_i8,45_i8,44_i8,(-11_i8),98_i8];
_27.0 = 127_i8 as i16;
match _4 {
0 => bb10,
1 => bb5,
2 => bb9,
340282366920938463463374607431768186431 => bb13,
_ => bb4
}
}
bb15 = {
_5 = [192908208671557626941909964615030242466_u128,141613626072016313833274454731901623816_u128,109928625034573161892947123898807121265_u128,315844182949366658044913548875207771374_u128,328247120520545318281428863893525065746_u128,269142811561803638566853565993259738022_u128];
_9 = [(-34_i8),(-97_i8),(-32_i8),39_i8,(-63_i8),15_i8,26_i8];
(*_8) = _5;
RET = 9439_u16 as i128;
_9 = _10;
_11 = -(-9223372036854775808_isize);
(*_1) = [291047759472574766692654784315270258226_u128,264773659581655459681015288939110057870_u128,235991560548991516730900323026623200322_u128,284173280638893825143898374619213633543_u128,170247968619473566736353715714716842064_u128,170854736606174356188859249267719551877_u128];
_15 = [12830049699228653924_u64,1854957716999402963_u64,3043845692118062453_u64,6921498927298517922_u64,14594080442755482301_u64,11481711623558578338_u64,11380534510307984928_u64,16350045769276687568_u64];
(*_8) = [142967416029050901901006288466847385292_u128,158346522401862851316131553035637887785_u128,333249624461664261662202337808927018120_u128,320530616868539815029158272613047010055_u128,223936345618044953110516270995415761931_u128,240923899227011204381678867179077803211_u128];
(*_1) = [51397429086873089297818613989027622488_u128,67359944112504431226411304081866432385_u128,206329369205129642228435217388651987466_u128,253228348762606904461510936679750375184_u128,237994980382375637902091673850608179320_u128,66795238369706408438147675853700733505_u128];
_17 = _11 as f32;
(*_8) = [11482415772572001638210416931580989462_u128,133267728989401061376841275037656031376_u128,28304452999275308271127617953625710811_u128,196821104788559937687622501635114896842_u128,337475873315227043376928697475160037109_u128,41433963922324246211486082865541773333_u128];
_16 = _11;
_17 = (-96_i8) as f32;
_16 = _11;
_6 = [(-46_i8),(-29_i8),21_i8,(-9_i8),(-123_i8),(-92_i8),(-2_i8)];
_3 = _13;
(*_8) = [2533008723488780524664328627010546011_u128,296989905784050049376710036162799683569_u128,260421038329586563904029701150112302780_u128,156730526184455334840685744902070693022_u128,54910419597948580502105999804343381537_u128,268509132375504409841462599958008260287_u128];
_16 = '\u{346b2}' as isize;
_15 = [9811778597605040174_u64,10484504963505775301_u64,6277002143821645381_u64,14790485665507492348_u64,9289262592614420422_u64,15362091304384757124_u64,748894117427396701_u64,4997913015419436926_u64];
(*_1) = [160464887863750388753786801929993723425_u128,329060154622078043575100943880268864437_u128,110120758562922421750110853444786703303_u128,78680483935948957002946069458283455399_u128,20696223897070546745907908223443190533_u128,202135165889530300292283814976271303426_u128];
RET = (-1687880422476163665148944279502120246_i128) ^ (-91128966456430574048063953886531853312_i128);
Goto(bb2)
}
bb16 = {
_39 = !_20;
_36 = 1926699890455399142_u64 as f64;
_20 = _39;
_34 = 178589898503864014812408649816673515142_u128 | 85481936673069212189255587441279849617_u128;
_10 = [(-90_i8),(-22_i8),28_i8,(-62_i8),38_i8,(-35_i8),28_i8];
(*_1) = _5;
_31 = [_34,_34,_34,_34,_34,_34];
_34 = _27.1 as u128;
_5 = (*_1);
_40 = _27.1;
(*_8) = _5;
(*_1) = _5;
_8 = _1;
_26.1 = 4384071593391432892_i64 as f32;
RET = (-33800833038936441439649270574127004285_i128);
_25 = !_33;
(*_8) = [_34,_34,_34,_34,_34,_34];
_26.0 = [94_i8,(-125_i8),(-99_i8),67_i8,(-29_i8),(-25_i8),8_i8];
_7 = _17 as u32;
_45 = _26;
_22 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = _6;
_26.0 = [(-122_i8),109_i8,(-116_i8),120_i8,(-15_i8),(-81_i8),60_i8];
_15 = _41;
_41 = [2768537054294732514_u64,12725117346709632363_u64,16965766362287780203_u64,17757315161057895408_u64,499837290605988613_u64,5506272429428253773_u64,9392665985767708316_u64,5156200972132772519_u64];
Goto(bb17)
}
bb17 = {
Call(_48 = dump_var(17_usize, 41_usize, Move(_41), 42_usize, Move(_42), 31_usize, Move(_31), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(17_usize, 2_usize, Move(_2), 14_usize, Move(_14), 5_usize, Move(_5), 40_usize, Move(_40)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(17_usize, 33_usize, Move(_33), 24_usize, Move(_24), 18_usize, Move(_18), 25_usize, Move(_25)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(17_usize, 12_usize, Move(_12), 10_usize, Move(_10), 28_usize, Move(_28), 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [u64; 8],mut _2: isize,mut _3: isize,mut _4: [i128; 8],mut _5: [i8; 7],mut _6: isize,mut _7: [u64; 8],mut _8: [u64; 8],mut _9: [i128; 8],mut _10: [u64; 8],mut _11: [i128; 8],mut _12: [i128; 8]) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _13: isize;
let _14: Adt45;
let _15: i8;
let _16: (i16, usize, [u64; 2], i16);
let _17: (u32,);
let _18: u128;
let _19: i64;
let _20: *mut u16;
let _21: Adt46;
let _22: char;
let _23: Adt42;
let _24: [u128; 6];
let _25: (isize, [u64; 2]);
let _26: [u128; 6];
let _27: bool;
let _28: f32;
let _29: (u32,);
let _30: (isize, [u64; 2]);
let _31: ();
let _32: ();
{
_4 = _9;
_8 = [4657479690228794495_u64,17962625204356063870_u64,16514509317380128927_u64,13806090802720518552_u64,11997446215876449330_u64,2091653785348717392_u64,9575631529390616777_u64,656975993354457120_u64];
_3 = true as isize;
_4 = [(-141637144779760478009313099214634110636_i128),5760971269859352051587950871644884736_i128,139747700580183950734066628012808272014_i128,(-109368884545004817462294622686850806462_i128),39822154154663503719618544369446047156_i128,(-167574597543925336672044809759711415689_i128),(-13448343161948616955856221661892307527_i128),144683314850923290652599108012114379141_i128];
_6 = _2 + _2;
_5 = [(-38_i8),(-73_i8),(-65_i8),(-93_i8),59_i8,(-37_i8),(-30_i8)];
RET = [9017697008572847254_u64,11863768439082810889_u64];
_1 = [14111824370361480494_u64,11659554558742199004_u64,1781002466905086955_u64,5892152880960096136_u64,14069658966111530657_u64,17860062412635245422_u64,13357075313191753058_u64,12716031697772195701_u64];
_2 = _6;
_3 = -_2;
_13 = !_3;
_15 = !(-25_i8);
Goto(bb1)
}
bb1 = {
_9 = _12;
_9 = [(-65409412941886472404102056018079870300_i128),(-163525248416736763390010830654869065141_i128),(-74755168554392970744462356910358835501_i128),(-148958266929494113515892082613794412880_i128),(-126739790412360083526470617435450887652_i128),(-71793603682659738629519470455283516713_i128),(-117699326690329058332022353677231737175_i128),(-129035838348235688580870418755777437124_i128)];
_7 = [8195542336031449479_u64,10986095278399585658_u64,13775929674092407072_u64,133278125202753327_u64,5814530586816418990_u64,3934874358952695919_u64,15278112949380029639_u64,18298552630399666294_u64];
_15 = !(-47_i8);
_10 = _1;
RET = [12813014783213248245_u64,10252792093608429305_u64];
_3 = (-918812447478144106_i64) as isize;
_12 = [15019965656155857943682409365351335810_i128,41576296844697416645970822963379632263_i128,(-268860825423632547926302599190002354_i128),(-5168126646430358142472705391360102197_i128),(-44537670225910190909538750971289527073_i128),130902828930601526899516340064756014047_i128,(-152009514772303332830513317213778113773_i128),80254525733095005085468486475630264594_i128];
_6 = _2;
_16 = ((-2956_i16), 6053236819165136145_usize, RET, 4199_i16);
_10 = _7;
RET = _16.2;
_8 = [1567057976691377341_u64,15129148599334567865_u64,11264958742514987788_u64,10698750757880871597_u64,14730519527342215021_u64,4704493623966347737_u64,2591919704722599476_u64,15520687879793149668_u64];
_4 = _11;
_11 = [(-67810230016583119856716617672951513589_i128),(-43585252473564945621986414584645146353_i128),(-45414408018639613452001429687738165505_i128),(-142018777530002748004200123409481595698_i128),(-138335624831208239412019698727186341444_i128),(-143384326367475691284169619649501388072_i128),89476235833174647486361565308725208082_i128,(-167976832996229983709757037529349419098_i128)];
_11 = [138588877901165638126892605673379873486_i128,(-45740131989294403297734778515410066186_i128),(-11619468305656722663257424115579996414_i128),(-95750081376675119000957822574635456240_i128),1165197253196291796878577804698064190_i128,(-70634172850192345508957144538966306447_i128),(-140063329025443428028839522048567222701_i128),44882223924503253086342249500642394645_i128];
_1 = [631948444557936944_u64,1396499364847036972_u64,5557413684379704215_u64,8288231510252274566_u64,15814148068860051359_u64,13677787620036583743_u64,1072707962827700659_u64,12427989964950040637_u64];
Call(_16.0 = core::intrinsics::transmute(_16.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = (3076420602_u32,);
_4 = _9;
_3 = _2 * _13;
_12 = _11;
_7 = [4188485918546147528_u64,9548524792242066900_u64,12393579751967311263_u64,16425912059847071556_u64,10797128217440000857_u64,5168493808594779850_u64,11785320060430647321_u64,323105402395238954_u64];
RET = _16.2;
_16 = (27025_i16, 3_usize, RET, 24691_i16);
_12 = [(-101314446433241631396782298754926427347_i128),159601721882554616935504384491144887949_i128,(-14119136689951416717794289923350010034_i128),(-140593987815052112981421240703979467066_i128),(-134568707212606736295487693175571231992_i128),52423607857658552526379424613740366133_i128,57714119152632833953149422281405624195_i128,162565502783754525903318693853386307875_i128];
Goto(bb3)
}
bb3 = {
_17.0 = 3488995674_u32;
_7 = [5500592260430560808_u64,15490326278191306199_u64,1614133367574418942_u64,8476553357456445506_u64,2603160162933954675_u64,16005127970616761517_u64,5756112259902108850_u64,5880216127447009628_u64];
_16.2 = [4415560204098124944_u64,9246515103719388100_u64];
_13 = '\u{fb3d8}' as isize;
_19 = (-5897171746077917132_i64);
_3 = (-1861638694_i32) as isize;
_7 = _10;
_17.0 = 967065554_u32 | 3658107358_u32;
Goto(bb4)
}
bb4 = {
_4 = [(-112931346652888593864790545728825172019_i128),(-108539493118357086013026109476727776931_i128),(-95461529116947411870892678028994497649_i128),(-14072860392912511398273175036397271331_i128),(-168640368273699516749133946500647077661_i128),(-89434329149849610953210298635026673679_i128),(-139088223229525806097641833478428867256_i128),(-65397005659224411839325481287907981362_i128)];
_13 = -_2;
_4 = [(-133914495978688607354137869471746962977_i128),(-84421250321883079891083389725167268560_i128),(-73974249084177363539167929696607489051_i128),165472475562297509448367806632315853686_i128,(-128378204704013278373996033870643573150_i128),(-12365756482301483367710883957784051394_i128),(-117199159861833805110342893237599535283_i128),129754362995426571749149137576125959227_i128];
_9 = [137191501338620520556987237273458061262_i128,(-88171764196177965658808551165094569833_i128),(-61641057652417856056709091502594256617_i128),156541819239400506408173326148538046373_i128,(-47071899807242387150502188241136003356_i128),(-92566787362696289050678398003130166243_i128),(-81624834143017949124933470179626390075_i128),11726757143242289822557949029662961680_i128];
Call(RET = fn19(_2, _6, _13, _11, _16, _13, _16.1, _8, _16.1, _8, _11, _16, _6, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = 1506227482_i32 as u128;
_11 = [155531133577247479433696488040724734426_i128,132846723293981380006416018018088419871_i128,(-167555069467432743636063514281533485631_i128),(-159057667843260524178376585750304627179_i128),167841940588196633756822724789815405424_i128,126806403217256555381243081482949618515_i128,(-45047597603186600891891779560364493709_i128),153241410891256563040671061468947823122_i128];
_4 = [146290662254423655166210269480383573834_i128,(-30859379430524168338304593513510719883_i128),4764315100547945677781955088817587858_i128,(-104472461198489864422912465463736796930_i128),19374581446262383562998720811178517586_i128,(-132999668915007662033944529195167698561_i128),(-54366466332538130200045385320394192596_i128),(-90494609173889347192507197121145421162_i128)];
_4 = [(-55179658418664350264547760356191201027_i128),(-63010567275975479664293703230631075945_i128),155161667026917651946206196171365294886_i128,69712864628758166307171527055714359642_i128,50437728810698026870005709941399234988_i128,120229773906843491749245710939363578371_i128,(-48745423606825062849526145256266964732_i128),151976951482448449119180587431257704734_i128];
_16.3 = _16.0 >> _13;
_17.0 = _18 as u32;
_3 = true as isize;
RET = _16.2;
_16 = (31086_i16, 2008297785574500883_usize, RET, (-23615_i16));
_16.2 = RET;
_25 = (_13, _16.2);
_16.3 = _16.0 ^ _16.0;
_5 = [_15,_15,_15,_15,_15,_15,_15];
_1 = [11350329800027948698_u64,8168531983411225047_u64,17787918851308673255_u64,8543141773492549965_u64,1240227503493567601_u64,15538289112119440624_u64,8912894942278359421_u64,8257381503565428650_u64];
_28 = 2128413545_i32 as f32;
_28 = _15 as f32;
_28 = _17.0 as f32;
_16.1 = 1576141252675544652_usize << _6;
_28 = _16.3 as f32;
_16.2 = _25.1;
_16 = (12725_i16, 3_usize, _25.1, (-25355_i16));
_15 = -(-63_i8);
RET = _16.2;
match _16.1 {
0 => bb6,
1 => bb7,
2 => bb8,
4 => bb10,
5 => bb11,
6 => bb12,
3 => bb14,
_ => bb13
}
}
bb6 = {
_4 = [(-112931346652888593864790545728825172019_i128),(-108539493118357086013026109476727776931_i128),(-95461529116947411870892678028994497649_i128),(-14072860392912511398273175036397271331_i128),(-168640368273699516749133946500647077661_i128),(-89434329149849610953210298635026673679_i128),(-139088223229525806097641833478428867256_i128),(-65397005659224411839325481287907981362_i128)];
_13 = -_2;
_4 = [(-133914495978688607354137869471746962977_i128),(-84421250321883079891083389725167268560_i128),(-73974249084177363539167929696607489051_i128),165472475562297509448367806632315853686_i128,(-128378204704013278373996033870643573150_i128),(-12365756482301483367710883957784051394_i128),(-117199159861833805110342893237599535283_i128),129754362995426571749149137576125959227_i128];
_9 = [137191501338620520556987237273458061262_i128,(-88171764196177965658808551165094569833_i128),(-61641057652417856056709091502594256617_i128),156541819239400506408173326148538046373_i128,(-47071899807242387150502188241136003356_i128),(-92566787362696289050678398003130166243_i128),(-81624834143017949124933470179626390075_i128),11726757143242289822557949029662961680_i128];
Call(RET = fn19(_2, _6, _13, _11, _16, _13, _16.1, _8, _16.1, _8, _11, _16, _6, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_17.0 = 3488995674_u32;
_7 = [5500592260430560808_u64,15490326278191306199_u64,1614133367574418942_u64,8476553357456445506_u64,2603160162933954675_u64,16005127970616761517_u64,5756112259902108850_u64,5880216127447009628_u64];
_16.2 = [4415560204098124944_u64,9246515103719388100_u64];
_13 = '\u{fb3d8}' as isize;
_19 = (-5897171746077917132_i64);
_3 = (-1861638694_i32) as isize;
_7 = _10;
_17.0 = 967065554_u32 | 3658107358_u32;
Goto(bb4)
}
bb8 = {
_17 = (3076420602_u32,);
_4 = _9;
_3 = _2 * _13;
_12 = _11;
_7 = [4188485918546147528_u64,9548524792242066900_u64,12393579751967311263_u64,16425912059847071556_u64,10797128217440000857_u64,5168493808594779850_u64,11785320060430647321_u64,323105402395238954_u64];
RET = _16.2;
_16 = (27025_i16, 3_usize, RET, 24691_i16);
_12 = [(-101314446433241631396782298754926427347_i128),159601721882554616935504384491144887949_i128,(-14119136689951416717794289923350010034_i128),(-140593987815052112981421240703979467066_i128),(-134568707212606736295487693175571231992_i128),52423607857658552526379424613740366133_i128,57714119152632833953149422281405624195_i128,162565502783754525903318693853386307875_i128];
Goto(bb3)
}
bb9 = {
_9 = _12;
_9 = [(-65409412941886472404102056018079870300_i128),(-163525248416736763390010830654869065141_i128),(-74755168554392970744462356910358835501_i128),(-148958266929494113515892082613794412880_i128),(-126739790412360083526470617435450887652_i128),(-71793603682659738629519470455283516713_i128),(-117699326690329058332022353677231737175_i128),(-129035838348235688580870418755777437124_i128)];
_7 = [8195542336031449479_u64,10986095278399585658_u64,13775929674092407072_u64,133278125202753327_u64,5814530586816418990_u64,3934874358952695919_u64,15278112949380029639_u64,18298552630399666294_u64];
_15 = !(-47_i8);
_10 = _1;
RET = [12813014783213248245_u64,10252792093608429305_u64];
_3 = (-918812447478144106_i64) as isize;
_12 = [15019965656155857943682409365351335810_i128,41576296844697416645970822963379632263_i128,(-268860825423632547926302599190002354_i128),(-5168126646430358142472705391360102197_i128),(-44537670225910190909538750971289527073_i128),130902828930601526899516340064756014047_i128,(-152009514772303332830513317213778113773_i128),80254525733095005085468486475630264594_i128];
_6 = _2;
_16 = ((-2956_i16), 6053236819165136145_usize, RET, 4199_i16);
_10 = _7;
RET = _16.2;
_8 = [1567057976691377341_u64,15129148599334567865_u64,11264958742514987788_u64,10698750757880871597_u64,14730519527342215021_u64,4704493623966347737_u64,2591919704722599476_u64,15520687879793149668_u64];
_4 = _11;
_11 = [(-67810230016583119856716617672951513589_i128),(-43585252473564945621986414584645146353_i128),(-45414408018639613452001429687738165505_i128),(-142018777530002748004200123409481595698_i128),(-138335624831208239412019698727186341444_i128),(-143384326367475691284169619649501388072_i128),89476235833174647486361565308725208082_i128,(-167976832996229983709757037529349419098_i128)];
_11 = [138588877901165638126892605673379873486_i128,(-45740131989294403297734778515410066186_i128),(-11619468305656722663257424115579996414_i128),(-95750081376675119000957822574635456240_i128),1165197253196291796878577804698064190_i128,(-70634172850192345508957144538966306447_i128),(-140063329025443428028839522048567222701_i128),44882223924503253086342249500642394645_i128];
_1 = [631948444557936944_u64,1396499364847036972_u64,5557413684379704215_u64,8288231510252274566_u64,15814148068860051359_u64,13677787620036583743_u64,1072707962827700659_u64,12427989964950040637_u64];
Call(_16.0 = core::intrinsics::transmute(_16.3), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb14 = {
_7 = [3240912427591735659_u64,15922621054021155844_u64,6417438425840279380_u64,13904214572395329048_u64,11926333502073802924_u64,8313661163436628496_u64,3598660557190955245_u64,15578932009514709053_u64];
_25 = (_6, RET);
_11 = _12;
RET = [17672048736569861530_u64,18154891196658848801_u64];
_29 = _17;
_26 = [_18,_18,_18,_18,_18,_18];
_26 = [_18,_18,_18,_18,_18,_18];
_8 = [15884051173773892800_u64,13207319262187240881_u64,45726118610553553_u64,637735882080785621_u64,29055398232162916_u64,8776443477355110764_u64,2947060978120953818_u64,11702572719125506061_u64];
RET = _25.1;
_27 = !true;
_26 = [_18,_18,_18,_18,_18,_18];
_22 = '\u{bd7ce}';
_25.1 = RET;
RET = _16.2;
_11 = _4;
_30.1 = [4558954571222723280_u64,16157601462763416092_u64];
_17.0 = _29.0 & _29.0;
_16 = ((-17169_i16), 6568804313906483744_usize, _25.1, 24300_i16);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(18_usize, 27_usize, Move(_27), 26_usize, Move(_26), 18_usize, Move(_18), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(18_usize, 7_usize, Move(_7), 11_usize, Move(_11), 12_usize, Move(_12), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(18_usize, 3_usize, Move(_3), 15_usize, Move(_15), 10_usize, Move(_10), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [i128; 8],mut _5: (i16, usize, [u64; 2], i16),mut _6: isize,mut _7: usize,mut _8: [u64; 8],mut _9: usize,mut _10: [u64; 8],mut _11: [i128; 8],mut _12: (i16, usize, [u64; 2], i16),mut _13: isize,mut _14: isize) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _15: *mut u16;
let _16: f32;
let _17: i16;
let _18: ([i8; 7], f32, [i128; 8]);
let _19: bool;
let _20: Adt46;
let _21: isize;
let _22: char;
let _23: isize;
let _24: i32;
let _25: [i16; 3];
let _26: (i16, usize, [u64; 2], i16);
let _27: u8;
let _28: i8;
let _29: isize;
let _30: (u32,);
let _31: f32;
let _32: *const [u128; 6];
let _33: f64;
let _34: f32;
let _35: u16;
let _36: Adt45;
let _37: bool;
let _38: u16;
let _39: isize;
let _40: f32;
let _41: bool;
let _42: isize;
let _43: u32;
let _44: isize;
let _45: *const *mut [u128; 6];
let _46: bool;
let _47: ();
let _48: ();
{
_5.1 = _9;
_9 = _7;
_11[_7] = _4[_7] & _4[_7];
Call(_14 = core::intrinsics::bswap(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_8[_7],_8[_7]];
_5.3 = !_12.3;
_2 = !_3;
_12.0 = _5.3 ^ _12.3;
_11[_7] = -_4[_7];
_9 = 4022968057_u32 as usize;
_11 = _4;
_5.3 = _5.0;
_8 = _10;
_5.2 = [_10[_7],_10[_7]];
RET = _12.2;
RET = [_8[_7],_10[_7]];
_4[_7] = _11[_7] ^ _11[_7];
_5 = _12;
_4 = [_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7]];
_5 = (_12.0, _7, RET, _12.3);
_5.1 = _12.1 % _12.1;
_10[_7] = _8[_7];
Goto(bb2)
}
bb2 = {
_19 = !false;
_5.2 = [_8[_7],_10[_7]];
_18.2[_7] = _4[_7];
_16 = 29334_u16 as f32;
_4 = _11;
_5.1 = _10[_7] as usize;
_12.3 = -_12.0;
_17 = !_5.0;
_11[_7] = _18.2[_7] & _4[_7];
_5.0 = 292281735_u32 as i16;
_18.1 = _16 + _16;
_18.2 = [_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7],_11[_7]];
_18.2[_7] = _11[_7] & _4[_7];
_8 = [_10[_7],_10[_7],_10[_7],_10[_7],_10[_7],_10[_7],_10[_7],_10[_7]];
Goto(bb3)
}
bb3 = {
_10 = [_8[_7],_8[_7],_8[_7],_8[_7],_8[_7],_8[_7],_8[_7],_8[_7]];
_18.2[_7] = _11[_7] | _11[_7];
_2 = _1;
_5 = _12;
_12 = (_5.0, _7, RET, _5.0);
_12.1 = 4491778182001279368_i64 as usize;
_7 = _5.1 * _12.1;
_4 = [68347167691825219745343413599094207830_i128,(-27335853724699585788848744818227929959_i128),85480418011359519967129588146722845015_i128,64194485629303274861877979274506975489_i128,(-163585380584616524382003017320619189579_i128),16259720581957283545885419254786508090_i128,38923978711005274935903781693231453161_i128,(-48030442878101061663135702350839865602_i128)];
_3 = 1_i8 as isize;
_5 = (_12.0, _9, _12.2, _17);
_14 = 252014385739554358473429052604480686060_u128 as isize;
_18.0 = [(-115_i8),31_i8,108_i8,20_i8,114_i8,(-11_i8),(-101_i8)];
RET = _12.2;
_16 = -_18.1;
_19 = true;
_5.3 = _12.3;
_18.2 = _11;
_2 = 4868048_u32 as isize;
_12.2 = _5.2;
_5.0 = _12.3;
Goto(bb4)
}
bb4 = {
_12 = (_5.3, _5.1, _5.2, _5.0);
_12 = (_5.3, _9, RET, _5.0);
_3 = 1755067515_u32 as isize;
_16 = _18.1;
_12 = (_17, _7, _5.2, _5.0);
_5.2 = RET;
_12.1 = 21768_u16 as usize;
Goto(bb5)
}
bb5 = {
_4 = _11;
_19 = _5.3 <= _5.0;
_5.0 = _12.0;
_10 = [4278935921522730157_u64,12986392050210927092_u64,14710626951810013808_u64,1305804977836677993_u64,2214012990310395360_u64,4329685609138381975_u64,10937383781375825175_u64,17411591899045020318_u64];
RET = _12.2;
_5.0 = 243744452826410232804538219652138218365_u128 as i16;
Goto(bb6)
}
bb6 = {
_27 = 93_u8 * 233_u8;
_23 = 59825_u16 as isize;
_25 = [_17,_12.0,_17];
_26.2 = _5.2;
_5.0 = !_17;
_9 = _27 as usize;
_9 = _27 as usize;
_12.3 = 608447874_i32 as i16;
_1 = !_6;
_29 = _13 * _14;
_12.2 = [8803212646978051499_u64,11185612061195941094_u64];
RET = _5.2;
Goto(bb7)
}
bb7 = {
_12.0 = _5.3;
_26.3 = _5.3 + _12.0;
_30.0 = 8777170137578368311_i64 as u32;
_24 = 93216767822460795879337156035836093877_i128 as i32;
_12.2 = [13848236470275328668_u64,4894359157081409273_u64];
_4 = [136304151927026135624892990915313311141_i128,15585733763281532674393793991180824575_i128,(-137733694597754824619117419920326991975_i128),159203857847000217251360303336113757727_i128,(-22405050930595923566729016640846067075_i128),30808507255024391193627893555866136471_i128,(-160381161257091865982408024384898808042_i128),23834952419351251128306409482588162209_i128];
_19 = false;
_26.1 = 123886053227088610769142094794278038561_i128 as usize;
Goto(bb8)
}
bb8 = {
_30 = (1493801142_u32,);
_12.3 = -_26.3;
_25 = [_12.3,_5.0,_26.3];
_35 = 104_i8 as u16;
_25 = [_5.3,_26.3,_12.3];
_5 = _12;
_16 = -_18.1;
_21 = -_1;
_26.0 = _5.0;
_18.1 = _16 * _16;
_5.3 = _12.0 * _26.3;
_12.0 = 14704891406271647635_u64 as i16;
_5.1 = _9;
_30 = (191468281_u32,);
_8 = _10;
_24 = (-99627785_i32);
_12.0 = 9354300972842388641000498045193373747_u128 as i16;
_18.1 = -_16;
_26.3 = _18.1 as i16;
_4 = [(-83982712948463880263548033551856966087_i128),83563756167243324432216061965213175302_i128,(-145697948945770157828819312699207778800_i128),(-76544831154354046757993499445921028970_i128),(-124838702767781140283248603121179619665_i128),(-161599900437805052032583169479231115574_i128),73123300658395933753802890441809652736_i128,50043991177250802832790698428325704411_i128];
_24 = 1866966975_i32;
_22 = '\u{7d36e}';
_18.2 = [96329775635243773242299432295203334154_i128,3169307395287091749539227367167271277_i128,(-134270250442701845333527079941079431225_i128),(-66783970310458357011582560755145347920_i128),(-58063049650004260719966319837024295132_i128),(-70942926475990540489907953939206722013_i128),(-122936032786294493829786252953208436508_i128),(-73737154545566874157704207670692968271_i128)];
_26 = (_12.3, _7, _12.2, _5.3);
match _24 {
0 => bb5,
1 => bb7,
1866966975 => bb9,
_ => bb4
}
}
bb9 = {
_26.3 = _12.3;
_19 = !false;
_33 = _12.0 as f64;
_5.0 = 41_i8 as i16;
_12.1 = _26.1;
_26 = (_12.3, _7, RET, _5.3);
_18.2 = _11;
_33 = _24 as f64;
_30.0 = 1857476130_u32;
_26.3 = !_26.0;
_12.0 = _30.0 as i16;
_26.1 = !_5.1;
_20 = Adt46::Variant1 { fld0: _19,fld1: _18,fld2: _3,fld3: 65_i8,fld4: _5.3,fld5: _18.1,fld6: _30.0,fld7: _33 };
_23 = _1;
_12.1 = !_5.1;
_31 = Field::<([i8; 7], f32, [i128; 8])>(Variant(_20, 1), 1).1 * _16;
Goto(bb10)
}
bb10 = {
_30 = (Field::<u32>(Variant(_20, 1), 6),);
_26.1 = _9;
Goto(bb11)
}
bb11 = {
_7 = _26.1;
_40 = _33 as f32;
_18.1 = _16 + _31;
Goto(bb12)
}
bb12 = {
RET = [3235222544648991603_u64,9802342178543513069_u64];
_1 = _6 & _6;
_26.3 = _26.0;
_3 = !_21;
_5.1 = _9;
_12.2 = _26.2;
match Field::<u32>(Variant(_20, 1), 6) {
1857476130 => bb14,
_ => bb13
}
}
bb13 = {
_7 = _26.1;
_40 = _33 as f32;
_18.1 = _16 + _31;
Goto(bb12)
}
bb14 = {
_5.1 = _26.1;
_21 = !_6;
_18.0 = [(-103_i8),55_i8,(-104_i8),30_i8,(-127_i8),83_i8,(-1_i8)];
_31 = -_16;
_5 = (_12.3, _7, _12.2, _26.3);
_42 = !_21;
_21 = _30.0 as isize;
place!(Field::<f32>(Variant(_20, 1), 5)) = _31;
_12.2 = [4013208313902796416_u64,333275393900540931_u64];
_37 = Field::<bool>(Variant(_20, 1), 0);
place!(Field::<([i8; 7], f32, [i128; 8])>(Variant(_20, 1), 1)) = _18;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(19_usize, 25_usize, Move(_25), 37_usize, Move(_37), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(19_usize, 1_usize, Move(_1), 13_usize, Move(_13), 2_usize, Move(_2), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(19_usize, 19_usize, Move(_19), 7_usize, Move(_7), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(19_usize, 12_usize, Move(_12), 21_usize, Move(_21), 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(149006510000398851939153504407088452749_u128), std::hint::black_box(84_isize), std::hint::black_box((-79_i8)), std::hint::black_box(17371_i16), std::hint::black_box(1924694970_i32), std::hint::black_box((-7899481690003227247_i64)), std::hint::black_box(2311774174_u32), std::hint::black_box(1576841902362332506_usize), std::hint::black_box(115_u8), std::hint::black_box(17488_u16));
                
            }
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: *const *mut [u128; 6],
fld1: (isize, [u64; 2]),
}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: [u64; 8],
fld1: u128,
fld2: [i128; 8],
fld3: *mut u16,
fld4: (isize, [u64; 2]),

},
Variant1{
fld0: u128,
fld1: char,
fld2: isize,
fld3: u64,
fld4: usize,
fld5: *mut [u128; 6],
fld6: i64,
fld7: ([i8; 7], f32, [i128; 8]),

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: [u64; 8],
fld1: char,
fld2: isize,
fld3: [u128; 6],
fld4: f32,
fld5: u64,
fld6: Adt39,
fld7: (u32,),

},
Variant1{
fld0: u64,
fld1: usize,
fld2: i16,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *const [u128; 6],
fld1: [u64; 8],
fld2: *mut u16,
fld3: ([i8; 7], (u32,)),
fld4: i16,
fld5: (u32,),

},
Variant1{
fld0: ([i8; 7], f32, [i128; 8]),
fld1: [i128; 8],
fld2: *mut i128,
fld3: [u64; 8],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: [i16; 3],
fld2: Adt39,
fld3: i8,
fld4: u16,

},
Variant1{
fld0: usize,
fld1: u64,
fld2: Adt40,

},
Variant2{
fld0: *const [u128; 6],
fld1: u64,
fld2: [i128; 8],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *mut u16,

},
Variant1{
fld0: (i16, usize, [u64; 2], i16),
fld1: *const [u128; 6],
fld2: isize,
fld3: (isize, [u64; 2]),
fld4: [u64; 2],
fld5: f64,

},
Variant2{
fld0: u32,
fld1: [i16; 8],
fld2: isize,
fld3: Adt39,
fld4: *mut [u128; 6],
fld5: [u64; 2],
fld6: *mut u16,

},
Variant3{
fld0: ([i8; 7], f32, [i128; 8]),
fld1: Adt38,
fld2: usize,
fld3: Adt39,
fld4: i16,
fld5: ([i8; 7], (u32,)),
fld6: u16,
fld7: Adt42,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u128,
fld1: (i16, usize, [u64; 2], i16),
fld2: [u64; 8],

},
Variant1{
fld0: (u32,),
fld1: Adt42,
fld2: *mut u16,
fld3: [u64; 2],
fld4: [i128; 8],
fld5: (i16, usize, [u64; 2], i16),
fld6: Adt38,
fld7: i128,

},
Variant2{
fld0: [u64; 8],
fld1: i8,

},
Variant3{
fld0: i32,
fld1: Adt42,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,
fld1: (u32,),
fld2: ([i8; 7], (u32,)),
fld3: Adt41,
fld4: u128,
fld5: (i16, [i16; 3]),

},
Variant1{
fld0: Adt42,
fld1: u32,
fld2: isize,
fld3: [i8; 7],

},
Variant2{
fld0: u64,
fld1: ([i8; 7], (u32,)),
fld2: i32,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: i32,
fld2: f64,
fld3: (i16, usize, [u64; 2], i16),

},
Variant1{
fld0: bool,
fld1: ([i8; 7], f32, [i128; 8]),
fld2: isize,
fld3: i8,
fld4: i16,
fld5: f32,
fld6: u32,
fld7: f64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u8,
fld1: [i16; 8],
fld2: f64,
fld3: (u32,),
fld4: Adt41,
fld5: Adt43,

},
Variant1{
fld0: *mut i128,
fld1: Adt40,
fld2: [u64; 8],
fld3: [i128; 8],
fld4: Adt39,
fld5: i32,
fld6: Adt41,

},
Variant2{
fld0: u64,
fld1: Adt46,
fld2: [i128; 8],
fld3: [i16; 8],
fld4: *mut [u128; 6],
fld5: (isize, [u64; 2]),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: ([i8; 7], (u32,)),
fld1: Adt39,
fld2: Adt40,
fld3: f32,
fld4: i16,
fld5: [i16; 3],
fld6: [u64; 2],
fld7: [i8; 7],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [u64; 8],
fld1: usize,
fld2: isize,
fld3: Adt45,
fld4: Adt38,
fld5: [i16; 3],
fld6: Adt46,

},
Variant1{
fld0: ([i8; 7], f32, [i128; 8]),
fld1: [u64; 8],
fld2: [i16; 3],
fld3: i64,
fld4: [u64; 2],
fld5: *mut i128,

},
Variant2{
fld0: Adt38,
fld1: [u64; 8],
fld2: [u64; 2],
fld3: *mut u16,
fld4: *const *mut [u128; 6],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [i128; 8],
fld1: Adt41,
fld2: ([i8; 7], (u32,)),
fld3: *const *mut [u128; 6],
fld4: u128,
fld5: (u32,),
fld6: u64,
fld7: f64,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: *const [u128; 6],
fld1: Adt42,
fld2: usize,
fld3: Adt45,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: f64,
fld1: Adt50,

},
Variant1{
fld0: Adt44,
fld1: f64,
fld2: Adt48,
fld3: usize,
fld4: [u64; 8],
fld5: i32,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: *mut u16,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt50,
fld1: f32,
fld2: Adt41,
fld3: Adt46,
fld4: Adt45,
fld5: Adt43,
fld6: [u64; 8],

},
Variant1{
fld0: ([i8; 7], (u32,)),
fld1: Adt45,
fld2: [u64; 2],
fld3: f32,
fld4: [u128; 6],

}}

