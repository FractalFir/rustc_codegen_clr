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
pub fn fn0(mut _1: u128,mut _2: u64,mut _3: u16,mut _4: i128,mut _5: i16) -> [u32; 5] {
mir! {
type RET = [u32; 5];
let _6: f64;
let _7: &'static (f32,);
let _8: [i8; 3];
let _9: (*const Adt31, [u32; 2], *const Adt31);
let _10: (u64, bool, &'static [i64; 2], bool);
let _11: char;
let _12: u32;
let _13: bool;
let _14: bool;
let _15: isize;
let _16: (f32,);
let _17: &'static i16;
let _18: *const (u32, usize, u32);
let _19: i32;
let _20: [u16; 5];
let _21: char;
let _22: i16;
let _23: f64;
let _24: [u128; 8];
let _25: ();
let _26: ();
{
_4 = 104144159639917744711913144002469862837_i128 ^ (-3227952877324163074884794979206665100_i128);
RET = [3809487637_u32,2723266050_u32,2734744425_u32,3993061686_u32,3012797880_u32];
_1 = !152929813325588158605947005288956406893_u128;
_6 = 2305_u16 as f64;
_5 = (-14316_i16);
_5 = (-23428_i16) + 2732_i16;
RET = [1182115164_u32,667339116_u32,3658154333_u32,116239874_u32,2590157640_u32];
_3 = 45924_u16 * 8611_u16;
_2 = 9223372036854775807_isize as u64;
RET = [738640590_u32,2540574963_u32,1564341859_u32,3841443332_u32,3146252538_u32];
_6 = 4289976899_u32 as f64;
Call(_2 = fn1(RET, RET, _5, RET, RET, _5, RET, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _4 as u64;
RET = [4065790244_u32,4043120438_u32,416756746_u32,2144132191_u32,2153238681_u32];
RET = [1514760000_u32,1350144997_u32,4096581181_u32,1112762388_u32,4014978419_u32];
RET = [1184078976_u32,322756410_u32,1046323092_u32,3622794892_u32,2565928362_u32];
_6 = 26522818_u32 as f64;
RET = [2279994298_u32,1723582013_u32,2251109347_u32,2880576062_u32,76450966_u32];
RET = [4280869567_u32,2462991492_u32,4228872076_u32,1189323466_u32,1519879511_u32];
_6 = _5 as f64;
_3 = 23728_u16 << _5;
_6 = _1 as f64;
_1 = 124083596047901368572108009099734610716_u128;
_4 = _6 as i128;
_4 = 136602877590041639575203525967332358170_i128 ^ (-148455103799316679768507914680240213165_i128);
_4 = -(-90823295374514671178525124256583239499_i128);
_6 = _3 as f64;
_2 = !1131202657962028325_u64;
_3 = 58029_u16;
RET = [1001492429_u32,646112466_u32,3109566436_u32,3115810456_u32,1455265604_u32];
_1 = 90288246488654042712943708433551530983_u128;
_8 = [(-96_i8),50_i8,(-91_i8)];
_3 = 32340_u16 * 17776_u16;
_4 = !135999715264061874799614421168658072527_i128;
_4 = !(-141987428584537621218401503196412531039_i128);
RET = [3944252747_u32,1878517297_u32,1542342670_u32,657402367_u32,734902648_u32];
_9.1 = [447726031_u32,525578404_u32];
Goto(bb2)
}
bb2 = {
_5 = '\u{10fe1e}' as i16;
_8 = [(-4_i8),11_i8,5_i8];
_8 = [65_i8,120_i8,(-72_i8)];
_5 = -(-18550_i16);
_10.3 = !true;
_6 = 14815102257831608779_usize as f64;
_1 = 117_i8 as u128;
_2 = 2651942242516424451_usize as u64;
_10.0 = _2 << _4;
_8 = [120_i8,(-7_i8),22_i8];
_3 = 41756_u16 + 42888_u16;
_2 = _6 as u64;
_11 = '\u{59569}';
_10.3 = _3 != _3;
_12 = 3371144928_u32 | 3322699532_u32;
_8 = [100_i8,(-26_i8),13_i8];
_4 = 40646120980732977819479651934332700793_i128;
_11 = '\u{c68b8}';
_3 = !61981_u16;
_10.0 = _2;
_10.1 = !_10.3;
_10.0 = !_2;
match _4 {
0 => bb3,
40646120980732977819479651934332700793 => bb5,
_ => bb4
}
}
bb3 = {
_2 = _4 as u64;
RET = [4065790244_u32,4043120438_u32,416756746_u32,2144132191_u32,2153238681_u32];
RET = [1514760000_u32,1350144997_u32,4096581181_u32,1112762388_u32,4014978419_u32];
RET = [1184078976_u32,322756410_u32,1046323092_u32,3622794892_u32,2565928362_u32];
_6 = 26522818_u32 as f64;
RET = [2279994298_u32,1723582013_u32,2251109347_u32,2880576062_u32,76450966_u32];
RET = [4280869567_u32,2462991492_u32,4228872076_u32,1189323466_u32,1519879511_u32];
_6 = _5 as f64;
_3 = 23728_u16 << _5;
_6 = _1 as f64;
_1 = 124083596047901368572108009099734610716_u128;
_4 = _6 as i128;
_4 = 136602877590041639575203525967332358170_i128 ^ (-148455103799316679768507914680240213165_i128);
_4 = -(-90823295374514671178525124256583239499_i128);
_6 = _3 as f64;
_2 = !1131202657962028325_u64;
_3 = 58029_u16;
RET = [1001492429_u32,646112466_u32,3109566436_u32,3115810456_u32,1455265604_u32];
_1 = 90288246488654042712943708433551530983_u128;
_8 = [(-96_i8),50_i8,(-91_i8)];
_3 = 32340_u16 * 17776_u16;
_4 = !135999715264061874799614421168658072527_i128;
_4 = !(-141987428584537621218401503196412531039_i128);
RET = [3944252747_u32,1878517297_u32,1542342670_u32,657402367_u32,734902648_u32];
_9.1 = [447726031_u32,525578404_u32];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_8 = [(-32_i8),37_i8,(-57_i8)];
_10.0 = (-9223372036854775808_isize) as u64;
_5 = (-69_i8) as i16;
_4 = _3 as i128;
_10.3 = _10.1 & _10.1;
_13 = _10.3;
_1 = 82456840031291893238253744655538179037_u128 - 71929810700748212004469217233413096366_u128;
_5 = (-13962_i16) >> _12;
_11 = '\u{8d1e3}';
_2 = 13828870974171122251_usize as u64;
_14 = !_10.3;
_17 = &_5;
Call(_15 = core::intrinsics::transmute(_9.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = _10.0 | _10.0;
_3 = 42353_u16;
_11 = '\u{5d338}';
_7 = &_16;
_16.0 = 1798984558_i32 as f32;
_13 = _10.3;
_11 = '\u{b3462}';
_20 = [_3,_3,_3,_3,_3];
_2 = _10.0;
_3 = 39487_u16 + 45126_u16;
_1 = 189121153740351991293806865534042068887_u128;
_19 = 658475035_i32;
_7 = &_16;
Goto(bb7)
}
bb7 = {
_11 = '\u{2a611}';
_7 = &(*_7);
_9.1 = [_12,_12];
_2 = _11 as u64;
_13 = _14 | _10.3;
_19 = 2109822374_i32 ^ (-909915333_i32);
RET = [_12,_12,_12,_12,_12];
_20 = [_3,_3,_3,_3,_3];
_4 = (-142396695392309758988878971918110755852_i128) >> _12;
_7 = &(*_7);
match _1 {
0 => bb4,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
189121153740351991293806865534042068887 => bb12,
_ => bb11
}
}
bb8 = {
_2 = _4 as u64;
RET = [4065790244_u32,4043120438_u32,416756746_u32,2144132191_u32,2153238681_u32];
RET = [1514760000_u32,1350144997_u32,4096581181_u32,1112762388_u32,4014978419_u32];
RET = [1184078976_u32,322756410_u32,1046323092_u32,3622794892_u32,2565928362_u32];
_6 = 26522818_u32 as f64;
RET = [2279994298_u32,1723582013_u32,2251109347_u32,2880576062_u32,76450966_u32];
RET = [4280869567_u32,2462991492_u32,4228872076_u32,1189323466_u32,1519879511_u32];
_6 = _5 as f64;
_3 = 23728_u16 << _5;
_6 = _1 as f64;
_1 = 124083596047901368572108009099734610716_u128;
_4 = _6 as i128;
_4 = 136602877590041639575203525967332358170_i128 ^ (-148455103799316679768507914680240213165_i128);
_4 = -(-90823295374514671178525124256583239499_i128);
_6 = _3 as f64;
_2 = !1131202657962028325_u64;
_3 = 58029_u16;
RET = [1001492429_u32,646112466_u32,3109566436_u32,3115810456_u32,1455265604_u32];
_1 = 90288246488654042712943708433551530983_u128;
_8 = [(-96_i8),50_i8,(-91_i8)];
_3 = 32340_u16 * 17776_u16;
_4 = !135999715264061874799614421168658072527_i128;
_4 = !(-141987428584537621218401503196412531039_i128);
RET = [3944252747_u32,1878517297_u32,1542342670_u32,657402367_u32,734902648_u32];
_9.1 = [447726031_u32,525578404_u32];
Goto(bb2)
}
bb9 = {
_8 = [(-32_i8),37_i8,(-57_i8)];
_10.0 = (-9223372036854775808_isize) as u64;
_5 = (-69_i8) as i16;
_4 = _3 as i128;
_10.3 = _10.1 & _10.1;
_13 = _10.3;
_1 = 82456840031291893238253744655538179037_u128 - 71929810700748212004469217233413096366_u128;
_5 = (-13962_i16) >> _12;
_11 = '\u{8d1e3}';
_2 = 13828870974171122251_usize as u64;
_14 = !_10.3;
_17 = &_5;
Call(_15 = core::intrinsics::transmute(_9.1), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_5 = '\u{10fe1e}' as i16;
_8 = [(-4_i8),11_i8,5_i8];
_8 = [65_i8,120_i8,(-72_i8)];
_5 = -(-18550_i16);
_10.3 = !true;
_6 = 14815102257831608779_usize as f64;
_1 = 117_i8 as u128;
_2 = 2651942242516424451_usize as u64;
_10.0 = _2 << _4;
_8 = [120_i8,(-7_i8),22_i8];
_3 = 41756_u16 + 42888_u16;
_2 = _6 as u64;
_11 = '\u{59569}';
_10.3 = _3 != _3;
_12 = 3371144928_u32 | 3322699532_u32;
_8 = [100_i8,(-26_i8),13_i8];
_4 = 40646120980732977819479651934332700793_i128;
_11 = '\u{c68b8}';
_3 = !61981_u16;
_10.0 = _2;
_10.1 = !_10.3;
_10.0 = !_2;
match _4 {
0 => bb3,
40646120980732977819479651934332700793 => bb5,
_ => bb4
}
}
bb11 = {
_2 = _4 as u64;
RET = [4065790244_u32,4043120438_u32,416756746_u32,2144132191_u32,2153238681_u32];
RET = [1514760000_u32,1350144997_u32,4096581181_u32,1112762388_u32,4014978419_u32];
RET = [1184078976_u32,322756410_u32,1046323092_u32,3622794892_u32,2565928362_u32];
_6 = 26522818_u32 as f64;
RET = [2279994298_u32,1723582013_u32,2251109347_u32,2880576062_u32,76450966_u32];
RET = [4280869567_u32,2462991492_u32,4228872076_u32,1189323466_u32,1519879511_u32];
_6 = _5 as f64;
_3 = 23728_u16 << _5;
_6 = _1 as f64;
_1 = 124083596047901368572108009099734610716_u128;
_4 = _6 as i128;
_4 = 136602877590041639575203525967332358170_i128 ^ (-148455103799316679768507914680240213165_i128);
_4 = -(-90823295374514671178525124256583239499_i128);
_6 = _3 as f64;
_2 = !1131202657962028325_u64;
_3 = 58029_u16;
RET = [1001492429_u32,646112466_u32,3109566436_u32,3115810456_u32,1455265604_u32];
_1 = 90288246488654042712943708433551530983_u128;
_8 = [(-96_i8),50_i8,(-91_i8)];
_3 = 32340_u16 * 17776_u16;
_4 = !135999715264061874799614421168658072527_i128;
_4 = !(-141987428584537621218401503196412531039_i128);
RET = [3944252747_u32,1878517297_u32,1542342670_u32,657402367_u32,734902648_u32];
_9.1 = [447726031_u32,525578404_u32];
Goto(bb2)
}
bb12 = {
_8 = [119_i8,(-13_i8),10_i8];
_21 = _11;
_3 = 23962_u16;
_1 = 172309909397917368209052096254542201780_u128 ^ 194454086375538529515539269509038539618_u128;
_12 = _1 as u32;
_7 = &(*_7);
RET = [_12,_12,_12,_12,_12];
_4 = _15 as i128;
RET = [_12,_12,_12,_12,_12];
_10.1 = _10.3;
_9.1 = [_12,_12];
_14 = !_10.1;
_23 = _6;
_15 = (*_17) as isize;
_9.1 = [_12,_12];
_8 = [2_i8,24_i8,67_i8];
_13 = _14 | _14;
_2 = !_10.0;
_15 = -(-9223372036854775808_isize);
Call(_22 = core::intrinsics::bswap(_5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9.1 = [_12,_12];
_10.3 = !_10.1;
_8 = [(-63_i8),(-35_i8),(-86_i8)];
_14 = _13;
Call(_10.1 = fn18(Move(_17), RET, _13, _14, _13, (*_17), _20), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10.1 = _13;
_23 = _5 as f64;
_17 = &_5;
_23 = _15 as f64;
_10.3 = !_10.1;
_7 = &(*_7);
_8 = [(-80_i8),(-102_i8),(-60_i8)];
_24 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(0_usize, 21_usize, Move(_21), 4_usize, Move(_4), 22_usize, Move(_22), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(0_usize, 14_usize, Move(_14), 8_usize, Move(_8), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [u32; 5],mut _2: [u32; 5],mut _3: i16,mut _4: [u32; 5],mut _5: [u32; 5],mut _6: i16,mut _7: [u32; 5],mut _8: i128,mut _9: i128) -> u64 {
mir! {
type RET = u64;
let _10: u8;
let _11: &'static f64;
let _12: i16;
let _13: [usize; 1];
let _14: (u32, usize, u32);
let _15: f64;
let _16: u16;
let _17: [u128; 1];
let _18: i8;
let _19: [i16; 6];
let _20: [u16; 7];
let _21: [i64; 5];
let _22: usize;
let _23: char;
let _24: u16;
let _25: Adt53;
let _26: i16;
let _27: isize;
let _28: i32;
let _29: f32;
let _30: isize;
let _31: u128;
let _32: *const Adt31;
let _33: f32;
let _34: Adt30;
let _35: [u16; 5];
let _36: i32;
let _37: u16;
let _38: ();
let _39: ();
{
RET = true as u64;
_10 = !45_u8;
_10 = 171_u8;
_7 = [3969135197_u32,1204608954_u32,3341786211_u32,2035614693_u32,3279683610_u32];
_7 = [3454418106_u32,3166866234_u32,3695226429_u32,3813171365_u32,2179214210_u32];
_2 = [3855139993_u32,4089122326_u32,885061894_u32,1436825273_u32,3481579889_u32];
_10 = (-1134672103038872616_i64) as u8;
_9 = _8;
_4 = [1903859317_u32,1693280455_u32,1255694380_u32,2694096567_u32,3095316083_u32];
_5 = [2482291190_u32,2075225860_u32,711988858_u32,2377731212_u32,20795220_u32];
_4 = _1;
_3 = 5049337615292986564_i64 as i16;
_6 = -_3;
_6 = _3;
_6 = 3362149322_u32 as i16;
RET = 5_usize as u64;
_12 = -_6;
_4 = [787363217_u32,3028674776_u32,189300982_u32,2646552942_u32,159340474_u32];
_8 = -_9;
RET = !140202994932152879_u64;
_4 = [3617243635_u32,2411237203_u32,3394153185_u32,3371303910_u32,3624608687_u32];
Goto(bb1)
}
bb1 = {
_1 = [3575038011_u32,2502184383_u32,3215341274_u32,2711453983_u32,4079829179_u32];
_9 = -_8;
_10 = 151_u8 << _8;
_14.2 = 3806177539_u32 & 60469814_u32;
_14 = (3292586067_u32, 4_usize, 3637894308_u32);
_1 = [_14.2,_14.2,_14.0,_14.0,_14.2];
_13 = [_14.1];
_14.1 = RET as usize;
_5 = _2;
_11 = &_15;
_15 = 332480137857453266106240476620811848969_u128 as f64;
_16 = !18803_u16;
_14 = (3532420053_u32, 5551553556272276007_usize, 288219247_u32);
_16 = RET as u16;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.0];
RET = 8834831497442721905_u64;
_13 = [_14.1];
_11 = &_15;
RET = 12493626392361083237_u64 & 6583685607610911654_u64;
Goto(bb2)
}
bb2 = {
_2 = _4;
_11 = &(*_11);
_15 = 4038688520487414503_i64 as f64;
_13 = [_14.1];
_17 = [329586532100532147101717851628932906586_u128];
_10 = _15 as u8;
_11 = &_15;
_19 = [_3,_6,_12,_6,_6,_3];
_17 = [174744101028339502389479783801868561218_u128];
_3 = _6;
_14.0 = !_14.2;
_18 = 111_i8 ^ 118_i8;
_2 = _5;
_12 = -_3;
_14.0 = _16 as u32;
_6 = _18 as i16;
Call(_16 = fn2(Move(_11), _9, _2, _14.2, _5, _14.1, _14.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _5;
_14.1 = '\u{61baa}' as usize;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.2];
_3 = false as i16;
_4 = _5;
RET = 705389584160771782_u64 * 11853818245640126911_u64;
_14.2 = false as u32;
_9 = _8 * _8;
_10 = !230_u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_2 = [_14.2,_14.0,_14.2,_14.2,_14.2];
_13 = [_14.1];
_13 = [_14.1];
_14.2 = _14.0;
_11 = &_15;
_18 = !(-123_i8);
_21 = [2143377233114167571_i64,5540083354759415585_i64,(-3558629236909049509_i64),(-572114168539884266_i64),4807493274487990117_i64];
_17 = [94469628858194959652609226279765091896_u128];
_14 = (1531570598_u32, 5740611986684169699_usize, 921024815_u32);
RET = 15091182699195233391_u64 & 13085574569843292172_u64;
Goto(bb4)
}
bb4 = {
RET = _14.0 as u64;
_4 = [_14.0,_14.2,_14.0,_14.2,_14.0];
_22 = _14.1 ^ _14.1;
_23 = '\u{e5231}';
_23 = '\u{c261e}';
_6 = _3 * _3;
_1 = _2;
_10 = 33_u8 * 203_u8;
_10 = 248_u8;
match _10 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
248 => bb10,
_ => bb9
}
}
bb5 = {
_4 = _5;
_14.1 = '\u{61baa}' as usize;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.2];
_3 = false as i16;
_4 = _5;
RET = 705389584160771782_u64 * 11853818245640126911_u64;
_14.2 = false as u32;
_9 = _8 * _8;
_10 = !230_u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_2 = [_14.2,_14.0,_14.2,_14.2,_14.2];
_13 = [_14.1];
_13 = [_14.1];
_14.2 = _14.0;
_11 = &_15;
_18 = !(-123_i8);
_21 = [2143377233114167571_i64,5540083354759415585_i64,(-3558629236909049509_i64),(-572114168539884266_i64),4807493274487990117_i64];
_17 = [94469628858194959652609226279765091896_u128];
_14 = (1531570598_u32, 5740611986684169699_usize, 921024815_u32);
RET = 15091182699195233391_u64 & 13085574569843292172_u64;
Goto(bb4)
}
bb6 = {
_2 = _4;
_11 = &(*_11);
_15 = 4038688520487414503_i64 as f64;
_13 = [_14.1];
_17 = [329586532100532147101717851628932906586_u128];
_10 = _15 as u8;
_11 = &_15;
_19 = [_3,_6,_12,_6,_6,_3];
_17 = [174744101028339502389479783801868561218_u128];
_3 = _6;
_14.0 = !_14.2;
_18 = 111_i8 ^ 118_i8;
_2 = _5;
_12 = -_3;
_14.0 = _16 as u32;
_6 = _18 as i16;
Call(_16 = fn2(Move(_11), _9, _2, _14.2, _5, _14.1, _14.1), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_1 = [3575038011_u32,2502184383_u32,3215341274_u32,2711453983_u32,4079829179_u32];
_9 = -_8;
_10 = 151_u8 << _8;
_14.2 = 3806177539_u32 & 60469814_u32;
_14 = (3292586067_u32, 4_usize, 3637894308_u32);
_1 = [_14.2,_14.2,_14.0,_14.0,_14.2];
_13 = [_14.1];
_14.1 = RET as usize;
_5 = _2;
_11 = &_15;
_15 = 332480137857453266106240476620811848969_u128 as f64;
_16 = !18803_u16;
_14 = (3532420053_u32, 5551553556272276007_usize, 288219247_u32);
_16 = RET as u16;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.0];
RET = 8834831497442721905_u64;
_13 = [_14.1];
_11 = &_15;
RET = 12493626392361083237_u64 & 6583685607610911654_u64;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_14.1 = (-7149526188119401859_i64) as usize;
_25.fld5 = -1775473532_i32;
_11 = &(*_11);
_18 = _8 as i8;
_4 = [_14.0,_14.0,_14.0,_14.2,_14.0];
_25.fld0 = (RET,);
_4 = _5;
_3 = _12 * _6;
_14 = (3110661580_u32, _22, 294321913_u32);
_25.fld2.0 = _16 as f32;
_25.fld2.0 = _25.fld5 as f32;
_14.2 = 230849936086057107973867199526191474104_u128 as u32;
_26 = _3;
_2 = _4;
_26 = _3 + _6;
_10 = !132_u8;
_24 = (-9223372036854775808_isize) as u16;
_14.1 = _25.fld0.0 as usize;
_11 = &(*_11);
_30 = (-60_isize);
match _14.0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
3110661580 => bb18,
_ => bb17
}
}
bb11 = {
Return()
}
bb12 = {
_1 = [3575038011_u32,2502184383_u32,3215341274_u32,2711453983_u32,4079829179_u32];
_9 = -_8;
_10 = 151_u8 << _8;
_14.2 = 3806177539_u32 & 60469814_u32;
_14 = (3292586067_u32, 4_usize, 3637894308_u32);
_1 = [_14.2,_14.2,_14.0,_14.0,_14.2];
_13 = [_14.1];
_14.1 = RET as usize;
_5 = _2;
_11 = &_15;
_15 = 332480137857453266106240476620811848969_u128 as f64;
_16 = !18803_u16;
_14 = (3532420053_u32, 5551553556272276007_usize, 288219247_u32);
_16 = RET as u16;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.0];
RET = 8834831497442721905_u64;
_13 = [_14.1];
_11 = &_15;
RET = 12493626392361083237_u64 & 6583685607610911654_u64;
Goto(bb2)
}
bb13 = {
_1 = [3575038011_u32,2502184383_u32,3215341274_u32,2711453983_u32,4079829179_u32];
_9 = -_8;
_10 = 151_u8 << _8;
_14.2 = 3806177539_u32 & 60469814_u32;
_14 = (3292586067_u32, 4_usize, 3637894308_u32);
_1 = [_14.2,_14.2,_14.0,_14.0,_14.2];
_13 = [_14.1];
_14.1 = RET as usize;
_5 = _2;
_11 = &_15;
_15 = 332480137857453266106240476620811848969_u128 as f64;
_16 = !18803_u16;
_14 = (3532420053_u32, 5551553556272276007_usize, 288219247_u32);
_16 = RET as u16;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.0];
RET = 8834831497442721905_u64;
_13 = [_14.1];
_11 = &_15;
RET = 12493626392361083237_u64 & 6583685607610911654_u64;
Goto(bb2)
}
bb14 = {
_2 = _4;
_11 = &(*_11);
_15 = 4038688520487414503_i64 as f64;
_13 = [_14.1];
_17 = [329586532100532147101717851628932906586_u128];
_10 = _15 as u8;
_11 = &_15;
_19 = [_3,_6,_12,_6,_6,_3];
_17 = [174744101028339502389479783801868561218_u128];
_3 = _6;
_14.0 = !_14.2;
_18 = 111_i8 ^ 118_i8;
_2 = _5;
_12 = -_3;
_14.0 = _16 as u32;
_6 = _18 as i16;
Call(_16 = fn2(Move(_11), _9, _2, _14.2, _5, _14.1, _14.1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_4 = _5;
_14.1 = '\u{61baa}' as usize;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.2];
_3 = false as i16;
_4 = _5;
RET = 705389584160771782_u64 * 11853818245640126911_u64;
_14.2 = false as u32;
_9 = _8 * _8;
_10 = !230_u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_2 = [_14.2,_14.0,_14.2,_14.2,_14.2];
_13 = [_14.1];
_13 = [_14.1];
_14.2 = _14.0;
_11 = &_15;
_18 = !(-123_i8);
_21 = [2143377233114167571_i64,5540083354759415585_i64,(-3558629236909049509_i64),(-572114168539884266_i64),4807493274487990117_i64];
_17 = [94469628858194959652609226279765091896_u128];
_14 = (1531570598_u32, 5740611986684169699_usize, 921024815_u32);
RET = 15091182699195233391_u64 & 13085574569843292172_u64;
Goto(bb4)
}
bb16 = {
RET = _14.0 as u64;
_4 = [_14.0,_14.2,_14.0,_14.2,_14.0];
_22 = _14.1 ^ _14.1;
_23 = '\u{e5231}';
_23 = '\u{c261e}';
_6 = _3 * _3;
_1 = _2;
_10 = 33_u8 * 203_u8;
_10 = 248_u8;
match _10 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
248 => bb10,
_ => bb9
}
}
bb17 = {
_4 = _5;
_14.1 = '\u{61baa}' as usize;
_7 = [_14.2,_14.2,_14.0,_14.2,_14.2];
_3 = false as i16;
_4 = _5;
RET = 705389584160771782_u64 * 11853818245640126911_u64;
_14.2 = false as u32;
_9 = _8 * _8;
_10 = !230_u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_2 = [_14.2,_14.0,_14.2,_14.2,_14.2];
_13 = [_14.1];
_13 = [_14.1];
_14.2 = _14.0;
_11 = &_15;
_18 = !(-123_i8);
_21 = [2143377233114167571_i64,5540083354759415585_i64,(-3558629236909049509_i64),(-572114168539884266_i64),4807493274487990117_i64];
_17 = [94469628858194959652609226279765091896_u128];
_14 = (1531570598_u32, 5740611986684169699_usize, 921024815_u32);
RET = 15091182699195233391_u64 & 13085574569843292172_u64;
Goto(bb4)
}
bb18 = {
RET = !_25.fld0.0;
_18 = (-8_i8) | (-108_i8);
_23 = '\u{69714}';
_14 = (3675548532_u32, _22, 3610865660_u32);
_30 = -(-9223372036854775808_isize);
_17 = [76661014247293973873223417287926098841_u128];
_14.1 = _22 >> _3;
_35 = [_16,_24,_24,_16,_24];
_11 = &_15;
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(1_usize, 10_usize, Move(_10), 12_usize, Move(_12), 19_usize, Move(_19), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(1_usize, 14_usize, Move(_14), 23_usize, Move(_23), 22_usize, Move(_22), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_38 = dump_var(1_usize, 2_usize, Move(_2), 16_usize, Move(_16), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: &'static f64,mut _2: i128,mut _3: [u32; 5],mut _4: u32,mut _5: [u32; 5],mut _6: usize,mut _7: usize) -> u16 {
mir! {
type RET = u16;
let _8: &'static &'static i32;
let _9: [i32; 6];
let _10: ([usize; 1], (u64,), u8, (u32, usize, u32));
let _11: *const [usize; 1];
let _12: i16;
let _13: (Adt78,);
let _14: i128;
let _15: (Adt31,);
let _16: (char, *const Adt31, *mut *const (u32, usize, u32));
let _17: f32;
let _18: (*const Adt31, [u32; 2], *const Adt31);
let _19: &'static *mut *mut *const &'static i16;
let _20: u32;
let _21: isize;
let _22: [usize; 8];
let _23: (Adt31,);
let _24: &'static *const Adt31;
let _25: [u32; 5];
let _26: usize;
let _27: isize;
let _28: [i128; 3];
let _29: ();
let _30: ();
{
_4 = 1979123708_u32;
_5 = [_4,_4,_4,_4,_4];
RET = '\u{e2be0}' as u16;
RET = true as u16;
_4 = 208_u8 as u32;
_2 = 56976197417329913485966598757544207891_i128;
_6 = !_7;
_4 = 2924325321_u32 | 2281334769_u32;
_4 = 643181714_u32 * 251108873_u32;
_3 = [_4,_4,_4,_4,_4];
_5 = _3;
_6 = !_7;
_6 = !_7;
RET = (-15185_i16) as u16;
_5 = _3;
RET = !6445_u16;
_3 = [_4,_4,_4,_4,_4];
_2 = 8838049303188888303_u64 as i128;
_10.3 = (_4, _7, _4);
Goto(bb1)
}
bb1 = {
_5 = [_4,_10.3.2,_10.3.2,_10.3.0,_10.3.0];
_10.3.0 = 2547112677539553725_u64 as u32;
_11 = core::ptr::addr_of!(_10.0);
(*_11) = [_7];
_10.1.0 = 11918597881802492625_u64;
RET = 61130_u16 << _10.3.1;
_6 = _10.3.1;
_9 = [(-961202979_i32),2143198937_i32,(-331341614_i32),(-1115744692_i32),605882498_i32,(-1258996090_i32)];
_4 = _10.3.0 * _10.3.0;
RET = 52249_u16 ^ 21065_u16;
_10.3 = (_4, _7, _4);
_10.3.2 = _4 & _10.3.0;
_7 = !_6;
RET = 26958_u16;
_10.3.1 = _6 * _7;
_5 = _3;
_5 = [_10.3.2,_10.3.0,_10.3.2,_10.3.0,_10.3.2];
_3 = _5;
_10.1.0 = 8367798698932864625_u64 * 14139388994418524030_u64;
RET = 28416_u16;
_10.0 = [_6];
match _6 {
0 => bb2,
5551553556272276007 => bb4,
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
_10.1.0 = _2 as u64;
_10.2 = !17_u8;
_13.0.fld3 = ('\u{ea494}', _7);
_2 = 32279042671484817338939323986303571905_i128;
_13.0.fld3.0 = '\u{b804a}';
_11 = core::ptr::addr_of!((*_11));
_12 = 11684_i16;
_2 = 55807423033346911544135202264964466300_i128 - (-29391899424605765075672923271255071620_i128);
_13.0.fld4 = 4135954088832356205_i64 as i16;
_13.0.fld2 = Adt30::Variant0 { fld0: _10.1.0,fld1: _2 };
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = _2;
_7 = _13.0.fld3.1;
_11 = core::ptr::addr_of!(_10.0);
_11 = core::ptr::addr_of!(_10.0);
RET = 39663_u16;
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = 78_i8 as i128;
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = _2 + _2;
_6 = !_7;
SetDiscriminant(_13.0.fld2, 2);
place!(Field::<u128>(Variant(_13.0.fld2, 2), 2)) = 81092946196776870032577647393453654643_u128 - 47328566637741198967019362633413925611_u128;
_13.0.fld1 = _13.0.fld3.0;
Goto(bb5)
}
bb5 = {
_2 = 66801401870342646999616304509819777325_i128 + 28607865613236985802009339277562057482_i128;
place!(Field::<(u64,)>(Variant(_13.0.fld2, 2), 3)) = (_10.1.0,);
(*_11) = [_7];
_13.0.fld4 = _12 & _12;
place!(Field::<(u64,)>(Variant(_13.0.fld2, 2), 3)) = _10.1;
_10.0 = [_10.3.1];
_14 = -_2;
_10.0 = [_7];
_16.2 = core::ptr::addr_of_mut!(place!(Field::<*const (u32, usize, u32)>(Variant(_13.0.fld2, 2), 1)));
place!(Field::<i16>(Variant(_13.0.fld2, 2), 4)) = _14 as i16;
RET = 49515_u16;
_7 = !_13.0.fld3.1;
place!(Field::<usize>(Variant(_13.0.fld2, 2), 0)) = _13.0.fld4 as usize;
(*_11) = [_10.3.1];
_10.0 = [_7];
_13.0.fld1 = _13.0.fld3.0;
(*_11) = [_6];
_16.1 = core::ptr::addr_of!(_15.0);
_10.0 = [_10.3.1];
_16.0 = _13.0.fld3.0;
_16.1 = core::ptr::addr_of!(_15.0);
_6 = _10.3.1;
RET = 35651_u16 >> _10.3.1;
Call(_13.0.fld0 = fn3(_13.0.fld3.0, _10, Move(_16.2), _10.3, _13.0.fld3.1, _4, _4, (*_11), _10, (*_11)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16.0 = _13.0.fld1;
_10.3 = (_4, _6, _4);
place!(Field::<usize>(Variant(_13.0.fld2, 2), 0)) = _6 + _6;
_13.0.fld3 = (_13.0.fld1, _6);
_13.0.fld3.0 = _13.0.fld1;
_13.0.fld3.0 = _13.0.fld1;
_10.3.0 = _4 | _4;
_16.1 = core::ptr::addr_of!(_15.0);
_20 = _10.3.2;
_18.2 = core::ptr::addr_of!(_23.0);
_20 = Field::<u128>(Variant(_13.0.fld2, 2), 2) as u32;
match _12 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
11684 => bb14,
_ => bb13
}
}
bb7 = {
_2 = 66801401870342646999616304509819777325_i128 + 28607865613236985802009339277562057482_i128;
place!(Field::<(u64,)>(Variant(_13.0.fld2, 2), 3)) = (_10.1.0,);
(*_11) = [_7];
_13.0.fld4 = _12 & _12;
place!(Field::<(u64,)>(Variant(_13.0.fld2, 2), 3)) = _10.1;
_10.0 = [_10.3.1];
_14 = -_2;
_10.0 = [_7];
_16.2 = core::ptr::addr_of_mut!(place!(Field::<*const (u32, usize, u32)>(Variant(_13.0.fld2, 2), 1)));
place!(Field::<i16>(Variant(_13.0.fld2, 2), 4)) = _14 as i16;
RET = 49515_u16;
_7 = !_13.0.fld3.1;
place!(Field::<usize>(Variant(_13.0.fld2, 2), 0)) = _13.0.fld4 as usize;
(*_11) = [_10.3.1];
_10.0 = [_7];
_13.0.fld1 = _13.0.fld3.0;
(*_11) = [_6];
_16.1 = core::ptr::addr_of!(_15.0);
_10.0 = [_10.3.1];
_16.0 = _13.0.fld3.0;
_16.1 = core::ptr::addr_of!(_15.0);
_6 = _10.3.1;
RET = 35651_u16 >> _10.3.1;
Call(_13.0.fld0 = fn3(_13.0.fld3.0, _10, Move(_16.2), _10.3, _13.0.fld3.1, _4, _4, (*_11), _10, (*_11)), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_10.1.0 = _2 as u64;
_10.2 = !17_u8;
_13.0.fld3 = ('\u{ea494}', _7);
_2 = 32279042671484817338939323986303571905_i128;
_13.0.fld3.0 = '\u{b804a}';
_11 = core::ptr::addr_of!((*_11));
_12 = 11684_i16;
_2 = 55807423033346911544135202264964466300_i128 - (-29391899424605765075672923271255071620_i128);
_13.0.fld4 = 4135954088832356205_i64 as i16;
_13.0.fld2 = Adt30::Variant0 { fld0: _10.1.0,fld1: _2 };
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = _2;
_7 = _13.0.fld3.1;
_11 = core::ptr::addr_of!(_10.0);
_11 = core::ptr::addr_of!(_10.0);
RET = 39663_u16;
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = 78_i8 as i128;
place!(Field::<i128>(Variant(_13.0.fld2, 0), 1)) = _2 + _2;
_6 = !_7;
SetDiscriminant(_13.0.fld2, 2);
place!(Field::<u128>(Variant(_13.0.fld2, 2), 2)) = 81092946196776870032577647393453654643_u128 - 47328566637741198967019362633413925611_u128;
_13.0.fld1 = _13.0.fld3.0;
Goto(bb5)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_5 = [_4,_10.3.2,_10.3.2,_10.3.0,_10.3.0];
_10.3.0 = 2547112677539553725_u64 as u32;
_11 = core::ptr::addr_of!(_10.0);
(*_11) = [_7];
_10.1.0 = 11918597881802492625_u64;
RET = 61130_u16 << _10.3.1;
_6 = _10.3.1;
_9 = [(-961202979_i32),2143198937_i32,(-331341614_i32),(-1115744692_i32),605882498_i32,(-1258996090_i32)];
_4 = _10.3.0 * _10.3.0;
RET = 52249_u16 ^ 21065_u16;
_10.3 = (_4, _7, _4);
_10.3.2 = _4 & _10.3.0;
_7 = !_6;
RET = 26958_u16;
_10.3.1 = _6 * _7;
_5 = _3;
_5 = [_10.3.2,_10.3.0,_10.3.2,_10.3.0,_10.3.2];
_3 = _5;
_10.1.0 = 8367798698932864625_u64 * 14139388994418524030_u64;
RET = 28416_u16;
_10.0 = [_6];
match _6 {
0 => bb2,
5551553556272276007 => bb4,
_ => bb3
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_13.0.fld3.0 = _16.0;
_10.3 = (_4, _6, _20);
_16.2 = core::ptr::addr_of_mut!(place!(Field::<*const (u32, usize, u32)>(Variant(_13.0.fld2, 2), 1)));
_24 = &_18.0;
place!(Field::<*const (u32, usize, u32)>(Variant(_13.0.fld2, 2), 1)) = core::ptr::addr_of!(_10.3);
_6 = _10.3.1 - Field::<usize>(Variant(_13.0.fld2, 2), 0);
_10.1.0 = Field::<(u64,)>(Variant(_13.0.fld2, 2), 3).0;
place!(Field::<usize>(Variant(_13.0.fld2, 2), 0)) = _6;
RET = 52489_u16 + 23347_u16;
_18.1 = [_10.3.2,_10.3.0];
_7 = Field::<usize>(Variant(_13.0.fld2, 2), 0);
_14 = _2;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 7_usize, Move(_7), 4_usize, Move(_4), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 12_usize, Move(_12), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: ([usize; 1], (u64,), u8, (u32, usize, u32)),mut _3: *mut *const (u32, usize, u32),mut _4: (u32, usize, u32),mut _5: usize,mut _6: u32,mut _7: u32,mut _8: [usize; 1],mut _9: ([usize; 1], (u64,), u8, (u32, usize, u32)),mut _10: [usize; 1]) -> *const [usize; 1] {
mir! {
type RET = *const [usize; 1];
let _11: &'static i128;
let _12: f64;
let _13: *const &'static i16;
let _14: isize;
let _15: (Adt78,);
let _16: isize;
let _17: u32;
let _18: u64;
let _19: (*const Adt31, [u32; 2], *const Adt31);
let _20: (*mut *const &'static i16, *const Adt31, Adt30);
let _21: (u128, Adt52, (u32, usize, u32));
let _22: u64;
let _23: (*const Adt31, [u32; 2], *const Adt31);
let _24: i8;
let _25: f64;
let _26: i8;
let _27: &'static (f32,);
let _28: [i128; 8];
let _29: char;
let _30: (u32, usize, u32);
let _31: (f32,);
let _32: f64;
let _33: ([usize; 1], (u64,), u8, (u32, usize, u32));
let _34: bool;
let _35: i16;
let _36: (usize, [i32; 6]);
let _37: i32;
let _38: f32;
let _39: [u32; 2];
let _40: &'static f64;
let _41: i8;
let _42: [u16; 5];
let _43: f32;
let _44: (f32,);
let _45: isize;
let _46: [i32; 6];
let _47: isize;
let _48: ();
let _49: ();
{
_2.0 = _10;
_2.3.2 = _6 | _2.3.0;
_2.3.1 = !_9.3.1;
_2.0 = _8;
_5 = _2.3.1 + _9.3.1;
_12 = 17513_u16 as f64;
_2.1.0 = !_9.1.0;
_9 = _2;
RET = core::ptr::addr_of!(_9.0);
_8 = [_4.1];
_2.3 = (_9.3.2, _9.3.1, _9.3.2);
RET = core::ptr::addr_of!(_9.0);
_2 = ((*RET), _9.1, _9.2, _4);
_2.3.2 = !_4.0;
_2 = (_9.0, _9.1, _9.2, _4);
_12 = 126_i8 as f64;
_2 = _9;
_2.3.2 = !_6;
Goto(bb1)
}
bb1 = {
_15.0.fld2 = Adt30::Variant0 { fld0: _9.1.0,fld1: (-101598960091517730686264691396819217721_i128) };
(*RET) = [_5];
_4.1 = _9.3.1 + _5;
_2.2 = !_9.2;
_2.0 = [_2.3.1];
_14 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!(_10);
(*RET) = [_9.3.1];
_9.3.2 = _4.2 * _9.3.0;
_4.2 = _12 as u32;
_2.2 = 29944_i16 as u8;
_15.0.fld3.1 = _4.1 << _4.1;
_9.0 = _8;
RET = core::ptr::addr_of!(_9.0);
_15.0.fld0 = Move(RET);
_15.0.fld5 = Move(_3);
RET = Move(_15.0.fld0);
_3 = Move(_15.0.fld5);
_15.0.fld3.1 = _2.3.1 ^ _4.1;
_9.1.0 = !_2.1.0;
_17 = _6;
_9 = (_10, _2.1, _2.2, _2.3);
_15.0.fld0 = core::ptr::addr_of!(_9.0);
_15.0.fld3.0 = _1;
Call(_15.0.fld4 = fn4(_15.0.fld3, _15.0.fld3, _2.3.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_19.1 = [_17,_6];
_21.2.2 = _15.0.fld3.1 as u32;
_21.0 = 203463346924959927549015358420706139869_u128;
_19.1 = [_2.3.2,_21.2.2];
_21.2.0 = _21.2.2 * _2.3.0;
_20.0 = core::ptr::addr_of_mut!(_13);
place!(Field::<i128>(Variant(_15.0.fld2, 0), 1)) = _12 as i128;
_6 = !_4.2;
_2.3.2 = _21.2.2 >> _21.2.2;
_23.1 = _19.1;
_2.3.1 = 61756_u16 as usize;
_8 = _9.0;
_24 = 114_i8 << _15.0.fld3.1;
_16 = -_14;
_9.3.1 = _12 as usize;
_11 = &place!(Field::<i128>(Variant(_15.0.fld2, 0), 1));
_8 = [_15.0.fld3.1];
_21.2 = (_4.2, _15.0.fld3.1, _2.3.2);
_15.0.fld0 = core::ptr::addr_of!(_10);
_2.3 = (_7, _21.2.1, _21.2.2);
_9.1 = (Field::<u64>(Variant(_15.0.fld2, 0), 0),);
_2.0 = [_21.2.1];
_2.3.2 = _12 as u32;
match _21.0 {
0 => bb3,
1 => bb4,
2 => bb5,
203463346924959927549015358420706139869 => bb7,
_ => bb6
}
}
bb3 = {
_15.0.fld2 = Adt30::Variant0 { fld0: _9.1.0,fld1: (-101598960091517730686264691396819217721_i128) };
(*RET) = [_5];
_4.1 = _9.3.1 + _5;
_2.2 = !_9.2;
_2.0 = [_2.3.1];
_14 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!(_10);
(*RET) = [_9.3.1];
_9.3.2 = _4.2 * _9.3.0;
_4.2 = _12 as u32;
_2.2 = 29944_i16 as u8;
_15.0.fld3.1 = _4.1 << _4.1;
_9.0 = _8;
RET = core::ptr::addr_of!(_9.0);
_15.0.fld0 = Move(RET);
_15.0.fld5 = Move(_3);
RET = Move(_15.0.fld0);
_3 = Move(_15.0.fld5);
_15.0.fld3.1 = _2.3.1 ^ _4.1;
_9.1.0 = !_2.1.0;
_17 = _6;
_9 = (_10, _2.1, _2.2, _2.3);
_15.0.fld0 = core::ptr::addr_of!(_9.0);
_15.0.fld3.0 = _1;
Call(_15.0.fld4 = fn4(_15.0.fld3, _15.0.fld3, _2.3.2), ReturnTo(bb2), UnwindUnreachable())
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
_9.3 = (_21.2.2, _2.3.1, _2.3.2);
_2.1.0 = Field::<u64>(Variant(_15.0.fld2, 0), 0);
_9.3 = (_21.2.2, _21.2.1, _21.2.2);
_9.0 = [_2.3.1];
_20.2 = Move(_15.0.fld2);
_21.2.2 = _21.0 as u32;
_18 = _2.1.0;
_20.2 = Adt30::Variant0 { fld0: _2.1.0,fld1: 150649856370772409991005019900049349457_i128 };
_23.1 = [_9.3.2,_9.3.0];
_23.1 = [_9.3.0,_9.3.0];
_9.1.0 = _18 - Field::<u64>(Variant(_20.2, 0), 0);
_7 = !_6;
_15.0.fld0 = Move(RET);
RET = core::ptr::addr_of!(_10);
_21.0 = 330544490825291302005967883665489681291_u128 | 106076885134150178799307110984045424661_u128;
_9.0 = [_5];
_19.1 = [_9.3.2,_9.3.0];
place!(Field::<i128>(Variant(_20.2, 0), 1)) = 43911_u16 as i128;
_4.0 = _9.3.0;
_4.2 = _9.3.2 | _4.0;
Goto(bb8)
}
bb8 = {
_4.0 = _4.2 | _9.3.2;
_21.2.1 = _15.0.fld4 as usize;
_4.1 = _5;
SetDiscriminant(_20.2, 1);
_15.0.fld1 = _1;
place!(Field::<i32>(Variant(_20.2, 1), 5)) = -2083964463_i32;
place!(Field::<[u16; 5]>(Variant(_20.2, 1), 6)) = [32727_u16,52949_u16,38936_u16,34380_u16,18683_u16];
RET = core::ptr::addr_of!((*RET));
place!(Field::<[u16; 5]>(Variant(_20.2, 1), 6)) = [35106_u16,29011_u16,8891_u16,22466_u16,53728_u16];
_31.0 = _4.0 as f32;
_15.0.fld2 = Adt30::Variant1 { fld0: _2.2,fld1: (-70381067821771139717345893182771809539_i128),fld2: _16,fld3: _15.0.fld3,fld4: _31.0,fld5: Field::<i32>(Variant(_20.2, 1), 5),fld6: Field::<[u16; 5]>(Variant(_20.2, 1), 6) };
_21.2.1 = !_5;
_14 = Field::<isize>(Variant(_15.0.fld2, 1), 2);
_4.0 = _4.2 + _9.3.2;
_31 = (Field::<f32>(Variant(_15.0.fld2, 1), 4),);
place!(Field::<f32>(Variant(_20.2, 1), 4)) = _31.0 * _31.0;
_21.0 = !217810541438184343821946735870230119501_u128;
_22 = !_18;
place!(Field::<isize>(Variant(_15.0.fld2, 1), 2)) = _14;
_4.1 = Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3).1 * _5;
_21.2.2 = _4.0;
Goto(bb9)
}
bb9 = {
place!(Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3)) = (_1, _9.3.1);
_21.2.0 = _21.2.2 * _4.0;
place!(Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3)).0 = _1;
Goto(bb10)
}
bb10 = {
place!(Field::<i32>(Variant(_15.0.fld2, 1), 5)) = _9.1.0 as i32;
_33.1 = (_9.1.0,);
place!(Field::<(char, usize)>(Variant(_20.2, 1), 3)) = _15.0.fld3;
place!(Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3)).1 = _15.0.fld4 as usize;
place!(Field::<f32>(Variant(_20.2, 1), 4)) = _31.0 - Field::<f32>(Variant(_15.0.fld2, 1), 4);
_33.3.2 = _4.2 - _4.0;
_4.2 = Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3).1 as u32;
_8 = [_2.3.1];
_2.3.0 = 3796018871841262408_i64 as u32;
_33.3 = (_9.3.0, _5, _4.2);
_25 = _21.0 as f64;
_30.2 = !_4.0;
_30 = (_9.3.0, _4.1, _33.3.0);
_17 = _4.0;
place!(Field::<i128>(Variant(_20.2, 1), 1)) = (-111485672807297243205819092485785628998_i128);
_15.0.fld5 = Move(_3);
_9.3.2 = _17;
_2.1.0 = _12 as u64;
_37 = Field::<i32>(Variant(_15.0.fld2, 1), 5);
match Field::<i128>(Variant(_20.2, 1), 1) {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
228796694113641220257555514945982582458 => bb16,
_ => bb15
}
}
bb11 = {
place!(Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3)) = (_1, _9.3.1);
_21.2.0 = _21.2.2 * _4.0;
place!(Field::<(char, usize)>(Variant(_15.0.fld2, 1), 3)).0 = _1;
Goto(bb10)
}
bb12 = {
_15.0.fld2 = Adt30::Variant0 { fld0: _9.1.0,fld1: (-101598960091517730686264691396819217721_i128) };
(*RET) = [_5];
_4.1 = _9.3.1 + _5;
_2.2 = !_9.2;
_2.0 = [_2.3.1];
_14 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!(_10);
(*RET) = [_9.3.1];
_9.3.2 = _4.2 * _9.3.0;
_4.2 = _12 as u32;
_2.2 = 29944_i16 as u8;
_15.0.fld3.1 = _4.1 << _4.1;
_9.0 = _8;
RET = core::ptr::addr_of!(_9.0);
_15.0.fld0 = Move(RET);
_15.0.fld5 = Move(_3);
RET = Move(_15.0.fld0);
_3 = Move(_15.0.fld5);
_15.0.fld3.1 = _2.3.1 ^ _4.1;
_9.1.0 = !_2.1.0;
_17 = _6;
_9 = (_10, _2.1, _2.2, _2.3);
_15.0.fld0 = core::ptr::addr_of!(_9.0);
_15.0.fld3.0 = _1;
Call(_15.0.fld4 = fn4(_15.0.fld3, _15.0.fld3, _2.3.2), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_15.0.fld2 = Adt30::Variant0 { fld0: _9.1.0,fld1: (-101598960091517730686264691396819217721_i128) };
(*RET) = [_5];
_4.1 = _9.3.1 + _5;
_2.2 = !_9.2;
_2.0 = [_2.3.1];
_14 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!(_10);
(*RET) = [_9.3.1];
_9.3.2 = _4.2 * _9.3.0;
_4.2 = _12 as u32;
_2.2 = 29944_i16 as u8;
_15.0.fld3.1 = _4.1 << _4.1;
_9.0 = _8;
RET = core::ptr::addr_of!(_9.0);
_15.0.fld0 = Move(RET);
_15.0.fld5 = Move(_3);
RET = Move(_15.0.fld0);
_3 = Move(_15.0.fld5);
_15.0.fld3.1 = _2.3.1 ^ _4.1;
_9.1.0 = !_2.1.0;
_17 = _6;
_9 = (_10, _2.1, _2.2, _2.3);
_15.0.fld0 = core::ptr::addr_of!(_9.0);
_15.0.fld3.0 = _1;
Call(_15.0.fld4 = fn4(_15.0.fld3, _15.0.fld3, _2.3.2), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_19.1 = [_17,_6];
_21.2.2 = _15.0.fld3.1 as u32;
_21.0 = 203463346924959927549015358420706139869_u128;
_19.1 = [_2.3.2,_21.2.2];
_21.2.0 = _21.2.2 * _2.3.0;
_20.0 = core::ptr::addr_of_mut!(_13);
place!(Field::<i128>(Variant(_15.0.fld2, 0), 1)) = _12 as i128;
_6 = !_4.2;
_2.3.2 = _21.2.2 >> _21.2.2;
_23.1 = _19.1;
_2.3.1 = 61756_u16 as usize;
_8 = _9.0;
_24 = 114_i8 << _15.0.fld3.1;
_16 = -_14;
_9.3.1 = _12 as usize;
_11 = &place!(Field::<i128>(Variant(_15.0.fld2, 0), 1));
_8 = [_15.0.fld3.1];
_21.2 = (_4.2, _15.0.fld3.1, _2.3.2);
_15.0.fld0 = core::ptr::addr_of!(_10);
_2.3 = (_7, _21.2.1, _21.2.2);
_9.1 = (Field::<u64>(Variant(_15.0.fld2, 0), 0),);
_2.0 = [_21.2.1];
_2.3.2 = _12 as u32;
match _21.0 {
0 => bb3,
1 => bb4,
2 => bb5,
203463346924959927549015358420706139869 => bb7,
_ => bb6
}
}
bb16 = {
_9 = _2;
place!(Field::<(char, usize)>(Variant(_20.2, 1), 3)).1 = !_4.1;
_33.0 = [_30.1];
_30 = _21.2;
_30 = (_21.2.0, _5, _21.2.2);
_27 = &_31;
_28 = [Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1)];
_39 = [_30.2,_30.2];
_41 = _24;
place!(Field::<(char, usize)>(Variant(_20.2, 1), 3)).0 = _1;
_9.3.1 = _2.3.1 + _2.3.1;
_28 = [Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1),Field::<i128>(Variant(_20.2, 1), 1)];
_36.0 = _5;
_31.0 = Field::<f32>(Variant(_20.2, 1), 4);
_34 = !false;
place!(Field::<u8>(Variant(_20.2, 1), 0)) = _12 as u8;
place!(Field::<u8>(Variant(_15.0.fld2, 1), 0)) = _21.2.1 as u8;
place!(Field::<(char, usize)>(Variant(_20.2, 1), 3)).1 = _15.0.fld3.1;
_2.1 = (_33.1.0,);
_1 = _15.0.fld1;
_15.0.fld1 = _15.0.fld3.0;
Goto(bb17)
}
bb17 = {
Call(_48 = dump_var(3_usize, 9_usize, Move(_9), 16_usize, Move(_16), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(3_usize, 28_usize, Move(_28), 41_usize, Move(_41), 34_usize, Move(_34), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(3_usize, 17_usize, Move(_17), 10_usize, Move(_10), 49_usize, _49, 49_usize, _49), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: (char, usize),mut _2: (char, usize),mut _3: u32) -> i16 {
mir! {
type RET = i16;
let _4: isize;
let _5: isize;
let _6: (usize, [i32; 6]);
let _7: i8;
let _8: ([i32; 6],);
let _9: char;
let _10: isize;
let _11: u128;
let _12: isize;
let _13: bool;
let _14: u64;
let _15: &'static f64;
let _16: u16;
let _17: usize;
let _18: f64;
let _19: &'static i16;
let _20: [u16; 7];
let _21: &'static i128;
let _22: ([usize; 1], (u64,), u8, (u32, usize, u32));
let _23: (*const Adt31, [u32; 2], *const Adt31);
let _24: f64;
let _25: u8;
let _26: (Adt31,);
let _27: char;
let _28: isize;
let _29: u16;
let _30: (u64, bool, &'static [i64; 2], bool);
let _31: (char, *const Adt31, *mut *const (u32, usize, u32));
let _32: isize;
let _33: Adt53;
let _34: isize;
let _35: bool;
let _36: bool;
let _37: isize;
let _38: (u64,);
let _39: &'static Adt29;
let _40: [i64; 2];
let _41: Adt78;
let _42: [u32; 2];
let _43: char;
let _44: (char, usize);
let _45: ();
let _46: ();
{
_2.0 = _1.0;
_2.0 = _1.0;
RET = _3 as i16;
_4 = (-9223372036854775808_isize);
RET = (-784020569_i32) as i16;
_1.1 = !_2.1;
_5 = _4 & _4;
_1 = (_2.0, _2.1);
_1 = (_2.0, _2.1);
_1.1 = RET as usize;
_3 = 4207037836372995557401775039654731141_u128 as u32;
_3 = !3009213647_u32;
_1.1 = _2.1;
_5 = !_4;
_1.0 = _2.0;
_1.1 = _1.0 as usize;
_1 = _2;
Goto(bb1)
}
bb1 = {
_2 = _1;
RET = (-4567_i16) + 16403_i16;
_7 = _4 as i8;
_7 = !(-37_i8);
_7 = 123_i8 - 126_i8;
_10 = !_4;
_2.1 = _7 as usize;
_8.0 = [(-1460678164_i32),149267266_i32,1474166381_i32,(-664500182_i32),811709037_i32,1113046992_i32];
_9 = _2.0;
_7 = (-63_i8) + (-82_i8);
_6.1 = [532011254_i32,(-58022388_i32),(-941662359_i32),1676856212_i32,(-368568312_i32),1063065498_i32];
Goto(bb2)
}
bb2 = {
_11 = 97209020793180720223234897893171963728_u128;
_14 = !11912558529423949389_u64;
_7 = (-117_i8);
_2.0 = _9;
_2.1 = (-8917349209229311454_i64) as usize;
_13 = !false;
_2.0 = _1.0;
_8 = (_6.1,);
_9 = _2.0;
_1.0 = _9;
_5 = -_4;
RET = (-28641_i16) << _1.1;
_1.0 = _2.0;
_2 = _1;
_1 = (_2.0, _2.1);
_2.1 = _4 as usize;
_6 = (_1.1, _8.0);
_2 = (_9, _1.1);
_5 = !_4;
_9 = _2.0;
RET = _14 as i16;
_6.0 = (-126668952479481862601206839302070161493_i128) as usize;
_13 = !false;
RET = 31625_i16;
Call(_16 = fn5(_2, _2, _5, _2.1, _4, _2.1, _1, _6.1, _2.1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = RET as isize;
_1.0 = _9;
_6.0 = (-50921921643355314712925216052988994168_i128) as usize;
_9 = _1.0;
RET = 3300_i16 - (-19329_i16);
_13 = true & false;
_9 = _2.0;
RET = -(-28505_i16);
_4 = _16 as isize;
_15 = &_18;
_17 = _2.1;
_19 = &RET;
RET = 734_i16;
_1.0 = _9;
_19 = &RET;
_11 = 278712565756459860500726277961705805541_u128;
RET = !(-20125_i16);
_1.0 = _2.0;
_22.3.2 = _3;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_13 = !false;
_22.2 = _4 as u8;
_5 = 157735776_i32 as isize;
_22.2 = 95_u8;
_4 = _10 << _2.1;
Goto(bb4)
}
bb4 = {
_6.0 = _2.1 << _22.2;
_2.0 = _1.0;
_6 = (_1.1, _8.0);
_4 = _10;
_22.2 = !70_u8;
_22.3 = (_3, _2.1, _3);
_22.1 = (_14,);
_15 = &_18;
_3 = _22.3.0;
Goto(bb5)
}
bb5 = {
_22.2 = !226_u8;
RET = 9813_i16 ^ (-30513_i16);
_1 = (_2.0, _17);
_1 = _2;
_4 = -_10;
_12 = _4 - _4;
_2 = (_9, _6.0);
_4 = _7 as isize;
_22.3.1 = !_2.1;
_2.0 = _9;
_14 = _22.1.0;
Call(_4 = fn16(), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_24 = 1155214410_i32 as f64;
_5 = -_4;
_7 = (-67_i8);
_2.1 = _1.1 | _1.1;
_11 = 48226021566083731592890499395568156188_u128 ^ 114470887250320494002622498346950828468_u128;
_28 = _5 ^ _4;
_19 = &RET;
_23.1 = [_3,_3];
_5 = _1.0 as isize;
_13 = false;
_30.3 = _6.0 < _2.1;
_12 = _24 as isize;
_29 = !_16;
_23.2 = core::ptr::addr_of!(_26.0);
_27 = _1.0;
_30.1 = _30.3;
Goto(bb7)
}
bb7 = {
_13 = _30.1;
_13 = !_30.1;
_2.0 = _1.0;
_10 = _4;
_6.0 = _22.3.1 - _17;
_31.0 = _9;
_6.0 = _1.1 & _22.3.1;
_22.0 = [_22.3.1];
_32 = _10 * _4;
_22.3.1 = _1.1 ^ _6.0;
RET = 1610_i16 | (-31271_i16);
_22.3 = (_3, _6.0, _3);
_8 = (_6.1,);
_33.fld0 = (_22.1.0,);
_1.0 = _27;
_1 = (_2.0, _17);
_33.fld3 = core::ptr::addr_of!(_22.3);
Goto(bb8)
}
bb8 = {
_33.fld2.0 = _14 as f32;
_23.0 = core::ptr::addr_of!(_26.0);
_29 = _16 + _16;
_13 = _1.1 >= _17;
_30.1 = _17 == _6.0;
_19 = &RET;
_33.fld0 = _22.1;
_13 = !_30.3;
_31.0 = _9;
_34 = _33.fld2.0 as isize;
_15 = &_18;
_23.1 = [_22.3.0,_22.3.0];
_29 = _16 - _16;
RET = (-2856042582119548428_i64) as i16;
_7 = (-10_i8) | (-23_i8);
_33.fld5 = -1879222391_i32;
_6 = (_2.1, _8.0);
_30.3 = !_13;
_30.0 = _33.fld0.0 << _4;
Goto(bb9)
}
bb9 = {
_19 = &_33.fld4;
_15 = &_18;
_25 = _33.fld5 as u8;
_23.2 = core::ptr::addr_of!(_26.0);
_17 = !_22.3.1;
_28 = _11 as isize;
_2.0 = _1.0;
_2.0 = _9;
_30.3 = !_30.1;
_30.1 = !_30.3;
_22.1 = (_33.fld0.0,);
_4 = -_32;
_30.1 = !_30.3;
_9 = _2.0;
_22.3.2 = _4 as u32;
_37 = _30.1 as isize;
_29 = _2.1 as u16;
_1.1 = !_6.0;
_41.fld3 = (_2.0, _6.0);
_42 = [_22.3.2,_3];
_41.fld3 = _2;
_34 = _37;
_30.3 = _13 | _13;
_11 = 314452612751192654821148714888910702004_u128 | 86532726077034326593657565986965211462_u128;
_33.fld0.0 = _30.3 as u64;
Goto(bb10)
}
bb10 = {
_22.3.0 = _22.3.2 ^ _22.3.2;
_33.fld0 = (_30.0,);
_15 = &_18;
RET = _29 as i16;
_33.fld2.0 = _22.2 as f32;
_22.1 = _33.fld0;
_18 = _24;
Goto(bb11)
}
bb11 = {
Call(_45 = dump_var(4_usize, 32_usize, Move(_32), 16_usize, Move(_16), 28_usize, Move(_28), 25_usize, Move(_25)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_45 = dump_var(4_usize, 4_usize, Move(_4), 13_usize, Move(_13), 34_usize, Move(_34), 2_usize, Move(_2)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_45 = dump_var(4_usize, 7_usize, Move(_7), 5_usize, Move(_5), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_45 = dump_var(4_usize, 12_usize, Move(_12), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (char, usize),mut _2: (char, usize),mut _3: isize,mut _4: usize,mut _5: isize,mut _6: usize,mut _7: (char, usize),mut _8: [i32; 6],mut _9: usize,mut _10: (char, usize)) -> u16 {
mir! {
type RET = u16;
let _11: (char, [i64; 2]);
let _12: u16;
let _13: Adt52;
let _14: *mut Adt29;
let _15: i8;
let _16: (&'static f64, [u16; 7], u16);
let _17: Adt53;
let _18: bool;
let _19: Adt78;
let _20: Adt30;
let _21: f64;
let _22: bool;
let _23: [u16; 7];
let _24: [u32; 2];
let _25: bool;
let _26: [i128; 3];
let _27: isize;
let _28: i8;
let _29: u8;
let _30: Adt78;
let _31: isize;
let _32: (&'static f64, [u16; 7], u16);
let _33: &'static i128;
let _34: &'static i64;
let _35: Adt52;
let _36: char;
let _37: Adt29;
let _38: &'static i32;
let _39: [u128; 1];
let _40: isize;
let _41: *const [usize; 1];
let _42: &'static char;
let _43: &'static *mut *mut *const &'static i16;
let _44: (Adt78,);
let _45: &'static (f32,);
let _46: *mut *mut *const &'static i16;
let _47: *mut *const &'static i16;
let _48: char;
let _49: ();
let _50: ();
{
_10 = (_1.0, _2.1);
_5 = !_3;
_10.1 = _7.1;
RET = !54362_u16;
_10 = _1;
_1 = _10;
RET = !14877_u16;
_5 = -_3;
_9 = 3421658564_u32 as usize;
_2.0 = _10.0;
_2.1 = _4;
_9 = _6 >> _7.1;
_1.1 = !_4;
_2.0 = _7.0;
_10.1 = !_6;
_11.1 = [(-6324932252940489456_i64),(-1726386767474684171_i64)];
_1.0 = _2.0;
_11.0 = _7.0;
_7.0 = _1.0;
_1.0 = _10.0;
_12 = RET - RET;
_7.0 = _11.0;
RET = _12;
_7.0 = _10.0;
_2 = _7;
_15 = 47_i8 ^ 100_i8;
Goto(bb1)
}
bb1 = {
_7.0 = _1.0;
_16.1 = [RET,_12,RET,RET,RET,RET,RET];
_9 = _6 - _6;
_6 = !_9;
_17.fld4 = -(-8566_i16);
_1 = _2;
_2.1 = 80_u8 as usize;
_17.fld1 = _10.0;
_18 = !false;
_19.fld3 = (_11.0, _7.1);
_17.fld2.0 = 291048059_i32 as f32;
_2 = (_11.0, _4);
RET = _12 + _12;
_19.fld3 = _7;
_19.fld3 = _10;
_8 = [(-171760370_i32),1780600128_i32,(-1588955463_i32),(-1120818352_i32),70546411_i32,1119388500_i32];
_21 = (-7543306524368732633_i64) as f64;
_17.fld0.0 = _21 as u64;
_8 = [(-591766820_i32),1991659805_i32,396445319_i32,(-605756771_i32),2014204443_i32,1730893460_i32];
Call(_19.fld5 = fn6(_3, _7, _7, _2, _2.1, _19.fld3, _6, _10, _1, _17.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2.1 = _9 * _1.1;
_7 = (_1.0, _10.1);
_19.fld3.0 = _7.0;
_16.2 = (-1492378873_i32) as u16;
_16.2 = 2413996553_u32 as u16;
Goto(bb3)
}
bb3 = {
RET = _3 as u16;
_17.fld0.0 = 13160175483247823150_u64 - 9023635414760236897_u64;
_17.fld2.0 = _5 as f32;
_11.1 = [4390375220433830946_i64,3394832761143424718_i64];
_6 = !_9;
_3 = _5 & _5;
_2 = (_17.fld1, _1.1);
_2.1 = _19.fld3.1;
_7.1 = _10.1;
_1.0 = _11.0;
_7.1 = _4 & _19.fld3.1;
_16.0 = &_21;
_19.fld3.1 = !_9;
_5 = _17.fld4 as isize;
_12 = RET;
_19.fld4 = _17.fld4;
_26 = [44932987733073520502305846926223957324_i128,(-152691208054626299178811352989804271604_i128),30865235363523353611001160536432447898_i128];
_7.0 = _10.0;
_26 = [(-69103524823792902556900739132310139611_i128),(-104463461626696244826999803660258856875_i128),(-104406165574746931801130743472660319410_i128)];
_17.fld1 = _7.0;
_27 = _3;
Goto(bb4)
}
bb4 = {
_17.fld0 = (2496670192325504866_u64,);
_17.fld0.0 = 12801985164257082672_u64 | 2499957829392458563_u64;
_19.fld4 = _17.fld4 - _17.fld4;
_30.fld4 = _3 as i16;
_19.fld5 = core::ptr::addr_of_mut!(_17.fld3);
RET = _12 * _12;
_30.fld3.0 = _1.0;
_17.fld0 = (620837855575274642_u64,);
_20 = Adt30::Variant0 { fld0: _17.fld0.0,fld1: 117420558839909370466445874265223716045_i128 };
_16.0 = &_21;
_32.0 = &_21;
_15 = (-121_i8) - (-113_i8);
_17.fld0.0 = _15 as u64;
Goto(bb5)
}
bb5 = {
_32.1 = [_12,_12,RET,RET,_12,_12,RET];
_17.fld4 = -_19.fld4;
_30.fld3 = (_11.0, _2.1);
_29 = 79_u8 | 36_u8;
RET = !_12;
Goto(bb6)
}
bb6 = {
_19.fld4 = -_17.fld4;
_24 = [462163403_u32,2415048641_u32];
_23 = [_12,_12,_16.2,RET,RET,_12,_12];
_2.0 = _17.fld1;
_26 = [(-169977078705788407062798924081206950891_i128),164781272792514581556180236895069838923_i128,(-47020595387045024780817094259485166140_i128)];
_32.1 = [RET,_16.2,_12,_16.2,_16.2,RET,_16.2];
Goto(bb7)
}
bb7 = {
_32.2 = RET + RET;
_17.fld5 = 1640888902_i32 >> _19.fld3.1;
place!(Field::<u64>(Variant(_20, 0), 0)) = _17.fld0.0 | _17.fld0.0;
RET = _15 as u16;
_30.fld5 = core::ptr::addr_of_mut!(_17.fld3);
_2.1 = !_6;
Goto(bb8)
}
bb8 = {
_10.0 = _19.fld3.0;
_17.fld0.0 = !Field::<u64>(Variant(_20, 0), 0);
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
_11.0 = _2.0;
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
_16.2 = _12 & RET;
_30.fld3.0 = _2.0;
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
_24 = [2789043909_u32,495587334_u32];
_15 = (-169799093768262766561287381273741450613_i128) as i8;
_7.1 = !_9;
_26 = [(-101537271688888589682193299369667175560_i128),(-125346539034481797409349880513204118434_i128),(-62824585793810384311472968490942733709_i128)];
place!(Field::<u64>(Variant(_20, 0), 0)) = _17.fld0.0 ^ _17.fld0.0;
_37 = Adt29::Variant1 { fld0: _30.fld3 };
place!(Field::<u64>(Variant(_20, 0), 0)) = _16.2 as u64;
_10 = (_2.0, _2.1);
_16.0 = &_21;
_7.1 = _1.1 ^ _9;
place!(Field::<i128>(Variant(_20, 0), 1)) = -(-50874292191534455918724875355206287346_i128);
_17.fld0.0 = Field::<u64>(Variant(_20, 0), 0) & Field::<u64>(Variant(_20, 0), 0);
_12 = RET >> _4;
_26 = [Field::<i128>(Variant(_20, 0), 1),Field::<i128>(Variant(_20, 0), 1),Field::<i128>(Variant(_20, 0), 1)];
Goto(bb9)
}
bb9 = {
_2.0 = _1.0;
_16.2 = _29 as u16;
_36 = _7.0;
_11.1 = [(-6967555150356697157_i64),5321059632050359470_i64];
_15 = 123_i8 & 71_i8;
_36 = _10.0;
_12 = !_32.2;
_18 = !false;
_17.fld0.0 = Field::<u64>(Variant(_20, 0), 0);
_1.0 = _19.fld3.0;
_16.1 = [RET,RET,_12,_16.2,RET,_12,_12];
_17.fld0 = (Field::<u64>(Variant(_20, 0), 0),);
_19.fld3 = (_1.0, _6);
_19.fld1 = _11.0;
_4 = _2.1 & _7.1;
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
_30.fld2 = Adt30::Variant0 { fld0: _17.fld0.0,fld1: (*_33) };
SetDiscriminant(_37, 1);
_30.fld1 = _36;
_17.fld0 = (Field::<u64>(Variant(_30.fld2, 0), 0),);
_1 = (_11.0, _9);
_23 = _16.1;
SetDiscriminant(_20, 0);
_6 = !_4;
Goto(bb10)
}
bb10 = {
place!(Field::<(char, usize)>(Variant(_37, 1), 0)) = (_17.fld1, _4);
_7 = _30.fld3;
_16 = (Move(_32.0), _23, RET);
_30.fld5 = core::ptr::addr_of_mut!(_17.fld3);
_1 = (_19.fld1, _4);
SetDiscriminant(_37, 1);
SetDiscriminant(_30.fld2, 1);
Goto(bb11)
}
bb11 = {
_30.fld3.0 = _19.fld3.0;
_32.1 = [RET,_32.2,_12,_32.2,RET,_16.2,_32.2];
_18 = _10.1 > _6;
_16.0 = &_21;
_10.0 = _17.fld1;
place!(Field::<u64>(Variant(_20, 0), 0)) = !_17.fld0.0;
_31 = _29 as isize;
_30.fld3.0 = _1.0;
_17.fld1 = _2.0;
Goto(bb12)
}
bb12 = {
_1 = _7;
_32 = (Move(_16.0), _23, RET);
_24 = [606613320_u32,1689118663_u32];
_32.0 = &_21;
_11.1 = [(-605099965442301874_i64),4488264622473149292_i64];
RET = _12 * _32.2;
_18 = _2.1 != _2.1;
_30.fld4 = _17.fld4;
place!(Field::<[u16; 5]>(Variant(_30.fld2, 1), 6)) = [_16.2,_32.2,_16.2,_12,RET];
_30.fld3.1 = 1288080053_u32 as usize;
_6 = _4 * _10.1;
_5 = !_27;
_40 = !_5;
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
Goto(bb13)
}
bb13 = {
_8 = [_17.fld5,_17.fld5,_17.fld5,_17.fld5,_17.fld5,_17.fld5];
_17.fld4 = _30.fld4;
_32.2 = !RET;
_42 = &_10.0;
_25 = _9 >= _6;
_1 = (_19.fld3.0, _6);
place!(Field::<(char, usize)>(Variant(_37, 1), 0)).1 = _4;
_23 = [_32.2,_12,_16.2,RET,_12,RET,RET];
_14 = core::ptr::addr_of_mut!(_37);
_33 = &place!(Field::<i128>(Variant(_20, 0), 1));
_16 = Move(_32);
_17.fld2.0 = 1471520775_u32 as f32;
_30.fld4 = -_17.fld4;
(*_14) = Adt29::Variant2 { fld0: 75305423873716984273757972170470093078_i128,fld1: _2 };
_29 = !201_u8;
_32.2 = RET & RET;
place!(Field::<i128>(Variant(_37, 2), 0)) = !(-136577581375443058397090853544555954788_i128);
_6 = _7.1 * _4;
_30.fld1 = _19.fld1;
place!(Field::<isize>(Variant(_30.fld2, 1), 2)) = -_3;
_28 = _17.fld2.0 as i8;
_22 = _2.1 > _9;
_28 = -_15;
place!(Field::<(char, usize)>(Variant((*_14), 2), 1)).0 = _10.0;
_3 = -_27;
Call(_25 = fn15(Move(_42), _22, Field::<(char, usize)>(Variant((*_14), 2), 1), Field::<(char, usize)>(Variant((*_14), 2), 1).1, Move(_14), _4, _1, _9), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<(char, usize)>(Variant(_37, 2), 1)).0 = _17.fld1;
_38 = &_17.fld5;
place!(Field::<(char, usize)>(Variant(_30.fld2, 1), 3)).1 = !_6;
_17.fld0 = (Field::<u64>(Variant(_20, 0), 0),);
_43 = &_46;
_32.1 = [_32.2,_32.2,RET,_12,RET,_12,_32.2];
_19.fld2 = Adt30::Variant1 { fld0: _29,fld1: Field::<i128>(Variant(_37, 2), 0),fld2: Field::<isize>(Variant(_30.fld2, 1), 2),fld3: _1,fld4: _17.fld2.0,fld5: (*_38),fld6: Field::<[u16; 5]>(Variant(_30.fld2, 1), 6) };
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(5_usize, 22_usize, Move(_22), 6_usize, Move(_6), 31_usize, Move(_31), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(5_usize, 2_usize, Move(_2), 8_usize, Move(_8), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(5_usize, 15_usize, Move(_15), 23_usize, Move(_23), 3_usize, Move(_3), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: (char, usize),mut _3: (char, usize),mut _4: (char, usize),mut _5: usize,mut _6: (char, usize),mut _7: usize,mut _8: (char, usize),mut _9: (char, usize),mut _10: (u64,)) -> *mut *const (u32, usize, u32) {
mir! {
type RET = *mut *const (u32, usize, u32);
let _11: f32;
let _12: &'static char;
let _13: Adt29;
let _14: *mut *const &'static i16;
let _15: f64;
let _16: u128;
let _17: isize;
let _18: (u128, Adt52, (u32, usize, u32));
let _19: [i64; 2];
let _20: isize;
let _21: usize;
let _22: *mut &'static i128;
let _23: bool;
let _24: char;
let _25: char;
let _26: isize;
let _27: i16;
let _28: &'static *mut *mut *const &'static i16;
let _29: (Adt78,);
let _30: bool;
let _31: [u16; 7];
let _32: u8;
let _33: [i8; 3];
let _34: [u32; 2];
let _35: u32;
let _36: f32;
let _37: i128;
let _38: isize;
let _39: u8;
let _40: bool;
let _41: (*const &'static i16, Adt30, Adt30, Adt26);
let _42: (Adt31,);
let _43: (char, [i64; 2]);
let _44: bool;
let _45: f64;
let _46: *mut &'static i128;
let _47: Adt53;
let _48: ();
let _49: ();
{
_2.1 = _9.1;
_4.1 = _2.1 * _6.1;
_6.1 = _5 * _8.1;
_7 = !_3.1;
_4.1 = _2.1 + _8.1;
_3.0 = _2.0;
_2.0 = _3.0;
_8.0 = _9.0;
_1 = 9223372036854775807_isize >> _5;
_9.0 = _8.0;
_10 = (4892760923789862392_u64,);
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
4892760923789862392 => bb7,
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
_3.0 = _6.0;
_6.0 = _2.0;
_3.1 = _8.1 >> _2.1;
_4.0 = _6.0;
_13 = Adt29::Variant1 { fld0: _3 };
_11 = 178_u8 as f32;
_3.0 = _9.0;
place!(Field::<(char, usize)>(Variant(_13, 1), 0)) = _6;
_2 = (_4.0, _4.1);
_4.1 = 138268008696086425994695083910494693498_i128 as usize;
_11 = _6.1 as f32;
_16 = !4789568759219862601762225462470218194_u128;
place!(Field::<(char, usize)>(Variant(_13, 1), 0)).1 = _6.1;
_6.0 = _4.0;
_16 = 211813568122449978143165936057429012132_u128;
Goto(bb8)
}
bb8 = {
_2.0 = _6.0;
place!(Field::<(char, usize)>(Variant(_13, 1), 0)).0 = _2.0;
_3.1 = !_2.1;
_12 = &_9.0;
_18.2.0 = 1037104684_u32 >> _9.1;
_15 = 265629821_i32 as f64;
_18.0 = 148553602452867526621457768900265377652_i128 as u128;
_8.0 = _3.0;
place!(Field::<(char, usize)>(Variant(_13, 1), 0)).1 = _18.2.0 as usize;
_17 = _1 - _1;
_6.0 = _9.0;
_8 = (_9.0, _2.1);
_13 = Adt29::Variant2 { fld0: 122794890243012596599182488397935477736_i128,fld1: _6 };
_18.2.2 = _18.2.0 - _18.2.0;
place!(Field::<(char, usize)>(Variant(_13, 2), 1)).1 = !_3.1;
_8 = _2;
_1 = 112738156323547429641197950999590813843_i128 as isize;
place!(Field::<(char, usize)>(Variant(_13, 2), 1)).0 = _6.0;
place!(Field::<(char, usize)>(Variant(_13, 2), 1)).0 = _9.0;
_18.2.0 = _16 as u32;
_15 = 2011525346_i32 as f64;
place!(Field::<i128>(Variant(_13, 2), 0)) = _10.0 as i128;
place!(Field::<(char, usize)>(Variant(_13, 2), 1)) = (_2.0, _8.1);
match _10.0 {
0 => bb9,
4892760923789862392 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_18.2.1 = _18.0 as usize;
_13 = Adt29::Variant1 { fld0: _2 };
_5 = _6.1 ^ _6.1;
place!(Field::<(char, usize)>(Variant(_13, 1), 0)).0 = _9.0;
SetDiscriminant(_13, 0);
_8 = (_6.0, _5);
_17 = !_1;
_3.1 = (-381265274_i32) as usize;
_10.0 = 16890577293094159933_u64;
place!(Field::<(u32, usize, u32)>(Variant(_13, 0), 1)).2 = _18.0 as u32;
place!(Field::<i128>(Variant(_13, 0), 7)) = (-678909775_i32) as i128;
_17 = _1 ^ _1;
place!(Field::<(u32, usize, u32)>(Variant(_13, 0), 1)) = _18.2;
_18.2 = Field::<(u32, usize, u32)>(Variant(_13, 0), 1);
match _10.0 {
16890577293094159933 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
place!(Field::<(u32, usize, u32)>(Variant(_13, 0), 1)) = (_18.2.2, _7, _18.2.2);
_1 = _17 << _8.1;
_6 = _8;
place!(Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2)).0 = [Field::<(u32, usize, u32)>(Variant(_13, 0), 1).1];
_7 = _16 as usize;
place!(Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2)).3.1 = !_5;
place!(Field::<(u32, usize, u32)>(Variant(_13, 0), 1)).0 = Field::<(u32, usize, u32)>(Variant(_13, 0), 1).2 & Field::<(u32, usize, u32)>(Variant(_13, 0), 1).2;
place!(Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2)).3.2 = !Field::<(u32, usize, u32)>(Variant(_13, 0), 1).2;
_3 = (_4.0, _5);
match _16 {
0 => bb3,
1 => bb8,
211813568122449978143165936057429012132 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2)).0 = [_9.1];
_10 = (1664581184629841086_u64,);
_3.0 = _4.0;
_9.0 = _6.0;
_6 = (_4.0, _8.1);
place!(Field::<i32>(Variant(_13, 0), 5)) = 1178616320_i32;
_21 = Field::<i32>(Variant(_13, 0), 5) as usize;
_7 = Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2).3.1 - _6.1;
place!(Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2)).0 = [_5];
_8.1 = _6.1 >> _3.1;
_2 = _3;
place!(Field::<Adt26>(Variant(_13, 0), 0)) = Adt26::Variant0 { fld0: (-25855_i16),fld1: _1 };
_2.0 = _6.0;
_6.0 = _9.0;
_18.2 = (Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2).3.2, _7, Field::<([usize; 1], (u64,), u8, (u32, usize, u32))>(Variant(_13, 0), 2).3.2);
place!(Field::<f64>(Variant(_13, 0), 6)) = _18.0 as f64;
_2.0 = _4.0;
_15 = Field::<f64>(Variant(_13, 0), 6) - Field::<f64>(Variant(_13, 0), 6);
_12 = &_6.0;
_13 = Adt29::Variant2 { fld0: (-147127440889220685471253133977086891439_i128),fld1: _3 };
Goto(bb16)
}
bb16 = {
_4 = (_2.0, _2.1);
place!(Field::<(char, usize)>(Variant(_13, 2), 1)) = _6;
_9 = _6;
_21 = _9.1;
_9.0 = _2.0;
_25 = (*_12);
_12 = &_2.0;
_3.1 = !_21;
_18.2.2 = _1 as u32;
_12 = &_2.0;
_23 = _18.2.0 >= _18.2.2;
_20 = -_1;
_13 = Adt29::Variant1 { fld0: _3 };
_19 = [(-1853816404409138783_i64),(-6488548794210990120_i64)];
_29.0.fld3.0 = Field::<(char, usize)>(Variant(_13, 1), 0).0;
_1 = !_20;
Goto(bb17)
}
bb17 = {
_3.1 = !_8.1;
_29.0.fld3 = _4;
_5 = _11 as usize;
_31 = [20563_u16,31887_u16,38652_u16,48377_u16,63798_u16,37687_u16,64627_u16];
_18.0 = _16;
_32 = 212_u8;
_24 = _2.0;
_4.0 = _6.0;
_29.0.fld3.1 = _1 as usize;
_12 = &place!(Field::<(char, usize)>(Variant(_13, 1), 0)).0;
_8.1 = _21;
_18.2.0 = (-28019_i16) as u32;
_6 = Field::<(char, usize)>(Variant(_13, 1), 0);
_26 = _1;
SetDiscriminant(_13, 2);
Call(_29.0.fld1 = fn7(_4, _26, _29.0.fld3.1, _4, _4.1, _18.2.1), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
place!(Field::<i128>(Variant(_13, 2), 0)) = (-76244789313141166751288039342670499656_i128) - 137712754730151666918614772939835617526_i128;
_9 = _6;
_6.0 = _24;
match _32 {
0 => bb9,
1 => bb7,
2 => bb10,
3 => bb12,
4 => bb19,
212 => bb21,
_ => bb20
}
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_33 = [(-29_i8),(-65_i8),(-80_i8)];
_16 = _18.0;
_29.0.fld3 = _4;
_24 = _8.0;
_12 = &place!(Field::<(char, usize)>(Variant(_13, 2), 1)).0;
_3.1 = _23 as usize;
_5 = 104_i8 as usize;
Goto(bb22)
}
bb22 = {
_37 = _10.0 as i128;
_30 = _3.1 != _7;
place!(Field::<(char, usize)>(Variant(_13, 2), 1)) = _6;
_39 = _32;
_8.1 = (-212606876_i32) as usize;
_27 = !(-2911_i16);
_1 = !_26;
_2.1 = _4.1 + _21;
_20 = _21 as isize;
SetDiscriminant(_13, 2);
_36 = _11;
_26 = -_20;
_17 = _1 ^ _20;
_3.1 = !_9.1;
_15 = _16 as f64;
_3.0 = _25;
_18.2 = (1073832081_u32, _29.0.fld3.1, 3140893658_u32);
_11 = _36;
_3.1 = _7 + _18.2.1;
_2.1 = _4.1 - _3.1;
_30 = _26 >= _20;
_8.0 = _6.0;
_18.2 = (1983860607_u32, _21, 4012735147_u32);
_9.1 = _3.1 ^ _29.0.fld3.1;
_29.0.fld3.1 = _2.1;
_26 = _1;
_19 = [(-1586424489069019780_i64),3667060002041986211_i64];
_10.0 = 6554981694001126783_u64 * 8531922747385937674_u64;
_1 = _37 as isize;
_40 = _30;
match _18.2.2 {
0 => bb21,
1 => bb2,
2 => bb12,
4012735147 => bb23,
_ => bb18
}
}
bb23 = {
_5 = 83_i8 as usize;
_30 = _18.2.0 < _18.2.0;
_6.1 = _3.1;
_34 = [_18.2.2,_18.2.2];
_33 = [(-16_i8),58_i8,50_i8];
_13 = Adt29::Variant2 { fld0: _37,fld1: _6 };
_31 = [95_u16,51713_u16,9528_u16,18691_u16,42112_u16,5121_u16,38922_u16];
_26 = _17 - _17;
_35 = _27 as u32;
_5 = _29.0.fld3.1 << _3.1;
_41.2 = Adt30::Variant0 { fld0: _10.0,fld1: Field::<i128>(Variant(_13, 2), 0) };
Goto(bb24)
}
bb24 = {
_37 = -Field::<i128>(Variant(_13, 2), 0);
SetDiscriminant(_13, 2);
_3.1 = !_9.1;
_24 = _3.0;
_4.0 = _6.0;
_43 = (_2.0, _19);
_18.2 = (_35, _6.1, _35);
_9 = (_43.0, _7);
_38 = _27 as isize;
_39 = _20 as u8;
_18.2.1 = !_3.1;
_39 = _10.0 as u8;
_21 = !_6.1;
_14 = core::ptr::addr_of_mut!(_41.0);
_38 = !_26;
Call(_6.0 = fn13(_4, _29.0.fld3, _9), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
place!(Field::<i128>(Variant(_41.2, 0), 1)) = !_37;
_23 = !_30;
_19 = _43.1;
_33 = [73_i8,41_i8,(-56_i8)];
_9.0 = _43.0;
_29.0.fld4 = -_27;
SetDiscriminant(_41.2, 0);
_10 = (1068050764584312257_u64,);
_2.0 = _4.0;
_14 = core::ptr::addr_of_mut!((*_14));
_26 = _20 << _21;
_2.0 = _4.0;
_45 = _10.0 as f64;
_16 = _18.0 << _21;
_17 = -_26;
_7 = !_5;
_1 = !_38;
_10 = (7572072524853868177_u64,);
RET = core::ptr::addr_of_mut!(_47.fld3);
_47.fld2.0 = _36;
_15 = _16 as f64;
place!(Field::<i128>(Variant(_41.2, 0), 1)) = _37 ^ _37;
_6.0 = _9.0;
_23 = _26 != _20;
(*RET) = core::ptr::addr_of!(_18.2);
_35 = !_18.2.2;
Goto(bb26)
}
bb26 = {
Call(_48 = dump_var(6_usize, 35_usize, Move(_35), 9_usize, Move(_9), 24_usize, Move(_24), 25_usize, Move(_25)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_48 = dump_var(6_usize, 23_usize, Move(_23), 19_usize, Move(_19), 27_usize, Move(_27), 38_usize, Move(_38)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_48 = dump_var(6_usize, 37_usize, Move(_37), 8_usize, Move(_8), 10_usize, Move(_10), 39_usize, Move(_39)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_48 = dump_var(6_usize, 43_usize, Move(_43), 7_usize, Move(_7), 16_usize, Move(_16), 49_usize, _49), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: (char, usize),mut _2: isize,mut _3: usize,mut _4: (char, usize),mut _5: usize,mut _6: usize) -> char {
mir! {
type RET = char;
let _7: [u32; 2];
let _8: isize;
let _9: *mut *mut *const &'static i16;
let _10: u128;
let _11: isize;
let _12: char;
let _13: [u16; 5];
let _14: (*const Adt31, [u32; 2], *const Adt31);
let _15: (*const &'static i16, Adt30, Adt30, Adt26);
let _16: bool;
let _17: &'static i16;
let _18: [i16; 6];
let _19: (u128, Adt52, (u32, usize, u32));
let _20: [i64; 5];
let _21: [i64; 3];
let _22: isize;
let _23: ([usize; 1], (u64,), u8, (u32, usize, u32));
let _24: f64;
let _25: &'static i64;
let _26: (u64, bool, &'static [i64; 2], bool);
let _27: f64;
let _28: f32;
let _29: (u32, usize, u32);
let _30: *mut *const (u32, usize, u32);
let _31: Adt26;
let _32: *mut *mut *const &'static i16;
let _33: (Adt78,);
let _34: u64;
let _35: &'static f64;
let _36: Adt53;
let _37: i64;
let _38: ();
let _39: ();
{
_4 = (_1.0, _3);
_5 = _3;
RET = _4.0;
_6 = _1.1;
_3 = !_5;
RET = _1.0;
_4 = _1;
_1.0 = _4.0;
_8 = !_2;
_4 = (RET, _3);
RET = _1.0;
_1.1 = 23135_u16 as usize;
RET = _1.0;
_4.0 = _1.0;
RET = _4.0;
Call(_4.0 = fn8(_5, _5, _8, _8, _5, _8, _5, _8, _2, _6, _4.1, _4.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.0 = RET;
_4.0 = _1.0;
RET = _4.0;
_2 = _8 + _8;
_5 = _6;
Goto(bb2)
}
bb2 = {
_13 = [45728_u16,10064_u16,26313_u16,20802_u16,53654_u16];
_11 = -_2;
_7 = [2361350281_u32,259968770_u32];
_6 = !_4.1;
_7 = [1293494110_u32,1927811321_u32];
Goto(bb3)
}
bb3 = {
RET = _1.0;
_2 = 24064_i16 as isize;
_1 = (RET, _6);
_13 = [20462_u16,49892_u16,31021_u16,64775_u16,44227_u16];
_14.1 = [723482255_u32,1408255370_u32];
_5 = _4.1;
_12 = _1.0;
_10 = !268307440381932975221159715136016698114_u128;
_16 = false & true;
_5 = _4.1;
_4.1 = _6 * _3;
Goto(bb4)
}
bb4 = {
_1.0 = RET;
_15.0 = core::ptr::addr_of!(_17);
_4.0 = RET;
_15.2 = Adt30::Variant0 { fld0: 8608798749900621258_u64,fld1: 62928457884191038648352401274910403992_i128 };
_15.3 = Adt26::Variant0 { fld0: (-18492_i16),fld1: _8 };
_15.2 = Adt30::Variant0 { fld0: 16193362684718525010_u64,fld1: (-47525544581778573804639073396537435551_i128) };
_19.2.0 = !1879221224_u32;
_15.0 = core::ptr::addr_of!(_17);
Goto(bb5)
}
bb5 = {
_19.2.0 = !3910725854_u32;
_6 = 8906_u16 as usize;
place!(Field::<isize>(Variant(_15.3, 0), 1)) = _8 - _8;
_1 = _4;
_1.1 = _4.1 | _5;
_4.1 = _5;
_19.2.0 = 63281246624600648072027638467792121797_i128 as u32;
Call(_12 = fn10(_11, _11, _1.1, _8, _8, Field::<isize>(Variant(_15.3, 0), 1), _4.1, _1.1, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_11 = !_8;
place!(Field::<u64>(Variant(_15.2, 0), 0)) = (-581734929_i32) as u64;
place!(Field::<u64>(Variant(_15.2, 0), 0)) = _10 as u64;
place!(Field::<isize>(Variant(_15.3, 0), 1)) = !_8;
_3 = !_1.1;
_20 = [3907716127840119213_i64,(-302108359135925009_i64),5474655917382984660_i64,7556657713613884008_i64,731739931276148125_i64];
_21 = [(-3825121665931857196_i64),(-5554758486819669213_i64),6035162952561920000_i64];
_18 = [(-7706_i16),29632_i16,16166_i16,19106_i16,(-23919_i16),31026_i16];
_15.0 = core::ptr::addr_of!(_17);
_1 = (RET, _3);
_19.0 = _10;
_1.1 = _10 as usize;
_2 = _11;
RET = _12;
place!(Field::<i16>(Variant(_15.3, 0), 0)) = -1819_i16;
SetDiscriminant(_15.3, 1);
_16 = !true;
_19.2.1 = _3 + _5;
_19.2 = (2676653103_u32, _3, 804416605_u32);
_19.2.1 = !_5;
_1.1 = _4.1 & _3;
_1 = (_4.0, _6);
Call(_26.1 = fn12(_19.2.1, _19.2, _19.2.0, _2, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23.0 = [_5];
_5 = _4.1 | _4.1;
_21 = [5112790341309324356_i64,1774470758910964980_i64,7493132493064158187_i64];
place!(Field::<i128>(Variant(_15.2, 0), 1)) = (-81_i8) as i128;
_24 = (-7870055629320623914_i64) as f64;
_26.3 = _26.1;
Goto(bb8)
}
bb8 = {
_26.1 = _4.1 != _19.2.1;
place!(Field::<*const (u32, usize, u32)>(Variant(_15.3, 1), 0)) = core::ptr::addr_of!(_23.3);
_8 = Field::<i128>(Variant(_15.2, 0), 1) as isize;
_23.3.0 = _19.2.2 & _19.2.0;
_20 = [(-7654848505729375786_i64),(-6849283726488638035_i64),(-1141947481280122172_i64),(-1676084429270795375_i64),8211827179242676227_i64];
_11 = _2;
_29 = (_19.2.0, _19.2.1, _23.3.0);
_13 = [48261_u16,31622_u16,22426_u16,5132_u16,54598_u16];
match _19.2.2 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb9,
804416605 => bb11,
_ => bb10
}
}
bb9 = {
_23.0 = [_5];
_5 = _4.1 | _4.1;
_21 = [5112790341309324356_i64,1774470758910964980_i64,7493132493064158187_i64];
place!(Field::<i128>(Variant(_15.2, 0), 1)) = (-81_i8) as i128;
_24 = (-7870055629320623914_i64) as f64;
_26.3 = _26.1;
Goto(bb8)
}
bb10 = {
_13 = [45728_u16,10064_u16,26313_u16,20802_u16,53654_u16];
_11 = -_2;
_7 = [2361350281_u32,259968770_u32];
_6 = !_4.1;
_7 = [1293494110_u32,1927811321_u32];
Goto(bb3)
}
bb11 = {
_31 = Move(_15.3);
Goto(bb12)
}
bb12 = {
_23.2 = (-4873_i16) as u8;
_23.1.0 = !Field::<u64>(Variant(_15.2, 0), 0);
_26.1 = _26.3;
_23.3.0 = !_19.2.2;
_16 = _26.1;
_23.3.1 = _19.2.1;
_26.1 = _26.3;
_1 = (_12, _23.3.1);
_26.3 = _23.3.0 > _19.2.2;
Goto(bb13)
}
bb13 = {
_19.2 = (_29.0, _29.1, _29.2);
_26.0 = _29.2 as u64;
_29.0 = _19.2.0;
_19.2.2 = _29.2 & _19.2.0;
_1.1 = _23.3.1 & _4.1;
_29.0 = Field::<i128>(Variant(_15.2, 0), 1) as u32;
_23.3 = (_19.2.2, _1.1, _19.2.0);
_33.0.fld5 = core::ptr::addr_of_mut!(_36.fld3);
_27 = _23.2 as f64;
_29.0 = _26.0 as u32;
_28 = _27 as f32;
_36.fld0.0 = Field::<u64>(Variant(_15.2, 0), 0);
_2 = _24 as isize;
match _23.3.2 {
0 => bb10,
1 => bb14,
2 => bb15,
2676653103 => bb17,
_ => bb16
}
}
bb14 = {
_13 = [45728_u16,10064_u16,26313_u16,20802_u16,53654_u16];
_11 = -_2;
_7 = [2361350281_u32,259968770_u32];
_6 = !_4.1;
_7 = [1293494110_u32,1927811321_u32];
Goto(bb3)
}
bb15 = {
_1.0 = RET;
_15.0 = core::ptr::addr_of!(_17);
_4.0 = RET;
_15.2 = Adt30::Variant0 { fld0: 8608798749900621258_u64,fld1: 62928457884191038648352401274910403992_i128 };
_15.3 = Adt26::Variant0 { fld0: (-18492_i16),fld1: _8 };
_15.2 = Adt30::Variant0 { fld0: 16193362684718525010_u64,fld1: (-47525544581778573804639073396537435551_i128) };
_19.2.0 = !1879221224_u32;
_15.0 = core::ptr::addr_of!(_17);
Goto(bb5)
}
bb16 = {
_13 = [45728_u16,10064_u16,26313_u16,20802_u16,53654_u16];
_11 = -_2;
_7 = [2361350281_u32,259968770_u32];
_6 = !_4.1;
_7 = [1293494110_u32,1927811321_u32];
Goto(bb3)
}
bb17 = {
_15.0 = core::ptr::addr_of!(_17);
_23.1 = (_26.0,);
_36.fld1 = RET;
_21 = [178678046916442397_i64,(-1105157378693729279_i64),(-5686421332610914170_i64)];
_15.3 = Adt26::Variant0 { fld0: (-2564_i16),fld1: _11 };
_19.2.1 = _5 - _29.1;
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(7_usize, 13_usize, Move(_13), 6_usize, Move(_6), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(7_usize, 11_usize, Move(_11), 20_usize, Move(_20), 3_usize, Move(_3), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(7_usize, 10_usize, Move(_10), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: usize,mut _2: usize,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: isize,mut _7: usize,mut _8: isize,mut _9: isize,mut _10: usize,mut _11: usize,mut _12: usize) -> char {
mir! {
type RET = char;
let _13: [usize; 8];
let _14: f32;
let _15: u8;
let _16: f32;
let _17: char;
let _18: *const [usize; 1];
let _19: [i16; 6];
let _20: char;
let _21: (u32, usize, u32);
let _22: isize;
let _23: &'static i64;
let _24: u16;
let _25: *mut Adt29;
let _26: u16;
let _27: u128;
let _28: i32;
let _29: ();
let _30: ();
{
_7 = _11 - _10;
RET = '\u{63d35}';
_1 = !_11;
_10 = _1 >> _11;
_1 = _7 + _11;
_3 = 1187822685986428509_i64 as isize;
_6 = _9;
_1 = _5 * _11;
RET = '\u{895f9}';
RET = '\u{a147}';
_11 = _10;
_11 = 138_u8 as usize;
_10 = 1547145838_i32 as usize;
_11 = !_7;
_8 = -_6;
RET = '\u{de17b}';
_3 = _4;
_13 = [_2,_2,_2,_12,_7,_7,_11,_2];
_12 = _7 * _5;
_12 = !_11;
_5 = 5787753509090633298_u64 as usize;
Call(_4 = fn9(_3, _2, _8, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = 10198_i16 as u8;
_10 = _11;
_7 = _12 * _11;
_12 = true as usize;
_12 = _2 + _10;
_9 = _15 as isize;
_8 = RET as isize;
_16 = _6 as f32;
_6 = 5229749111743593052_i64 as isize;
_4 = _3 + _3;
_14 = _16 + _16;
_7 = !_12;
_10 = 42102_u16 as usize;
_4 = _3 >> _3;
_5 = !_12;
_15 = true as u8;
_11 = !_5;
Goto(bb2)
}
bb2 = {
_17 = RET;
_7 = _5;
_1 = _12 + _7;
_17 = RET;
_1 = 226756534081454706453517415154903390445_u128 as usize;
_4 = !_3;
_12 = _5;
_5 = _2 + _11;
_16 = -_14;
_7 = RET as usize;
_7 = _2;
_19 = [(-18756_i16),(-26930_i16),14369_i16,14082_i16,17236_i16,21099_i16];
_15 = _5 as u8;
_5 = _12;
_7 = _11 << _11;
_21 = (3569154893_u32, _2, 1973103598_u32);
_15 = 212_u8 << _4;
_3 = _4;
_9 = _4 * _4;
_21 = (4226510249_u32, _7, 1123408559_u32);
_12 = _11 & _2;
_13 = [_12,_21.1,_2,_5,_12,_11,_21.1,_2];
match _21.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
1123408559 => bb11,
_ => bb10
}
}
bb3 = {
_15 = 10198_i16 as u8;
_10 = _11;
_7 = _12 * _11;
_12 = true as usize;
_12 = _2 + _10;
_9 = _15 as isize;
_8 = RET as isize;
_16 = _6 as f32;
_6 = 5229749111743593052_i64 as isize;
_4 = _3 + _3;
_14 = _16 + _16;
_7 = !_12;
_10 = 42102_u16 as usize;
_4 = _3 >> _3;
_5 = !_12;
_15 = true as u8;
_11 = !_5;
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
_14 = _16;
_13 = [_7,_7,_10,_2,_12,_12,_21.1,_5];
_20 = _17;
_16 = _15 as f32;
_6 = -_9;
RET = _20;
_17 = _20;
_7 = _21.1 * _2;
RET = _17;
_8 = _3;
_22 = _8;
_8 = 16448386894534612693_u64 as isize;
_13 = [_2,_11,_2,_7,_5,_2,_2,_2];
_4 = _3;
_16 = _15 as f32;
_2 = !_7;
_16 = 43357920722242186761160110330109048055_u128 as f32;
_19 = [19730_i16,(-10679_i16),(-41_i16),(-16805_i16),(-27970_i16),(-24096_i16)];
_19 = [(-14170_i16),2936_i16,22570_i16,5554_i16,904_i16,29736_i16];
_17 = RET;
_21.0 = !_21.2;
_12 = !_5;
_15 = 8702_i16 as u8;
RET = _17;
_14 = 30997_u16 as f32;
_24 = 64515_u16 & 42387_u16;
_10 = _21.1;
_1 = _10 >> _5;
_14 = _16;
match _21.2 {
0 => bb7,
1 => bb9,
1123408559 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_17 = RET;
_16 = _14 - _14;
_19 = [27046_i16,29383_i16,(-22100_i16),(-4767_i16),9200_i16,6798_i16];
_21.0 = _21.2;
_21.0 = _21.2 >> _12;
_7 = !_10;
_7 = !_12;
Call(_10 = core::intrinsics::transmute(_6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = _11;
RET = _20;
_2 = _10;
_26 = 8458204919672558077_u64 as u16;
_1 = _21.1 - _2;
_22 = 7342002605968745575_i64 as isize;
_2 = !_21.1;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(8_usize, 15_usize, Move(_15), 26_usize, Move(_26), 3_usize, Move(_3), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(8_usize, 17_usize, Move(_17), 1_usize, Move(_1), 13_usize, Move(_13), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(8_usize, 22_usize, Move(_22), 20_usize, Move(_20), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: usize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: i8;
let _6: bool;
let _7: ();
let _8: ();
{
RET = !_3;
_5 = -83_i8;
_2 = !3_usize;
_1 = (-919136471_i32) as isize;
RET = _4;
RET = _4 >> _4;
_6 = RET < RET;
_3 = -RET;
_4 = !RET;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(9_usize, 4_usize, Move(_4), 5_usize, Move(_5), 1_usize, Move(_1), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: isize,mut _3: usize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: usize,mut _8: usize,mut _9: usize) -> char {
mir! {
type RET = char;
let _10: [i32; 6];
let _11: f32;
let _12: isize;
let _13: *mut *const (u32, usize, u32);
let _14: *const (u32, usize, u32);
let _15: Adt80;
let _16: f64;
let _17: i16;
let _18: f64;
let _19: u128;
let _20: (Adt31,);
let _21: [usize; 8];
let _22: &'static i64;
let _23: [i8; 3];
let _24: bool;
let _25: char;
let _26: isize;
let _27: Adt78;
let _28: bool;
let _29: f32;
let _30: (u32, usize, u32);
let _31: [u16; 5];
let _32: i8;
let _33: isize;
let _34: ();
let _35: ();
{
_8 = _9;
_9 = (-1650757459_i32) as usize;
_9 = !_3;
_3 = _7;
RET = '\u{40e92}';
_6 = _5;
_11 = (-119869597798763270388321899398560434696_i128) as f32;
_1 = _4;
_7 = !_3;
_9 = !_8;
_4 = _6;
_6 = !_1;
_12 = _4 * _2;
RET = '\u{60d4}';
_1 = _4 | _12;
RET = '\u{48050}';
_10 = [1541501293_i32,(-1558924923_i32),617258768_i32,(-58884820_i32),1653266019_i32,979108938_i32];
RET = '\u{48d9}';
_9 = _3 >> _5;
RET = '\u{667e0}';
_10 = [1426323471_i32,2072122994_i32,(-2063067911_i32),1467598492_i32,1810362781_i32,(-341239604_i32)];
_13 = core::ptr::addr_of_mut!(_14);
_3 = 2812680173_u32 as usize;
Call(_12 = fn11(_2, _7, _4, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _8;
_8 = !_9;
_11 = 3423431026_u32 as f32;
_4 = _1 >> _9;
_12 = _2 >> _6;
_11 = 1359_i16 as f32;
_8 = _9 << _1;
_12 = -_2;
Call(_5 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = 250_u8 as f64;
RET = '\u{43826}';
_11 = _9 as f32;
_2 = _12;
_10 = [(-1474595748_i32),2116460954_i32,1922043379_i32,1233871392_i32,1458495907_i32,(-824025250_i32)];
_2 = _1 * _6;
_5 = 1126791899_i32 as isize;
RET = '\u{23ed2}';
_7 = !_9;
_5 = !_12;
_2 = -_12;
RET = '\u{7bdb0}';
_18 = 180637964081504415933869222712268705398_u128 as f64;
Goto(bb3)
}
bb3 = {
_11 = (-5621087714818843725_i64) as f32;
_10 = [1929955863_i32,179030691_i32,1274712602_i32,1954810398_i32,(-1030698209_i32),(-1873799305_i32)];
RET = '\u{f79ce}';
_21 = [_9,_8,_8,_7,_8,_7,_9,_9];
_11 = _16 as f32;
_11 = 6848_i16 as f32;
_18 = _16;
_16 = _18;
_18 = _16;
RET = '\u{cc76e}';
_12 = _5;
_21 = [_7,_7,_8,_8,_8,_8,_7,_8];
_18 = _16 - _16;
_18 = -_16;
_3 = _9;
_4 = 3715153251204902868_u64 as isize;
_4 = _1;
_17 = (-1915772279_i32) as i16;
_5 = _2 << _12;
_23 = [(-13_i8),16_i8,26_i8];
_11 = 37912800408037167389093807565966546689_i128 as f32;
_6 = _11 as isize;
Goto(bb4)
}
bb4 = {
_13 = core::ptr::addr_of_mut!(_14);
_3 = 205_u8 as usize;
_10 = [513426459_i32,(-894626245_i32),(-1706918918_i32),1860888168_i32,(-807838108_i32),(-690788551_i32)];
_19 = 87254464587701533438000947950053589290_u128 & 105459327091974775192838516694777753514_u128;
_7 = _8 & _9;
Goto(bb5)
}
bb5 = {
_16 = _18;
Goto(bb6)
}
bb6 = {
_7 = 23590535650481112070901877829684784200_i128 as usize;
_9 = !_8;
_23 = [69_i8,102_i8,5_i8];
_13 = core::ptr::addr_of_mut!((*_13));
_13 = core::ptr::addr_of_mut!((*_13));
_6 = _11 as isize;
_3 = _8;
_5 = !_2;
_5 = !_4;
_3 = _9 ^ _8;
_7 = _8;
_4 = 138_u8 as isize;
_24 = !false;
_3 = _9;
_18 = (-1397438499_i32) as f64;
_11 = _18 as f32;
RET = '\u{1b91f}';
_12 = 215_u8 as isize;
_6 = _1;
Goto(bb7)
}
bb7 = {
_24 = true;
_17 = 30514_i16;
Goto(bb8)
}
bb8 = {
_24 = false & true;
_2 = _5;
_12 = _6 >> _6;
_6 = _12 >> _5;
_8 = _9;
_24 = false;
_26 = _6 >> _5;
_16 = _18 - _18;
RET = '\u{cd01f}';
_4 = _12;
_11 = 15508024661515158298_u64 as f32;
_1 = _19 as isize;
_4 = _26 - _2;
_27.fld3 = (RET, _7);
_26 = -_5;
_10 = [398208872_i32,(-1683868937_i32),(-1076048174_i32),(-122576904_i32),(-1424500913_i32),(-804920480_i32)];
RET = _27.fld3.0;
_4 = _2 | _26;
match _17 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
30514 => bb15,
_ => bb14
}
}
bb9 = {
_7 = _8;
_8 = !_9;
_11 = 3423431026_u32 as f32;
_4 = _1 >> _9;
_12 = _2 >> _6;
_11 = 1359_i16 as f32;
_8 = _9 << _1;
_12 = -_2;
Call(_5 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_7 = 23590535650481112070901877829684784200_i128 as usize;
_9 = !_8;
_23 = [69_i8,102_i8,5_i8];
_13 = core::ptr::addr_of_mut!((*_13));
_13 = core::ptr::addr_of_mut!((*_13));
_6 = _11 as isize;
_3 = _8;
_5 = !_2;
_5 = !_4;
_3 = _9 ^ _8;
_7 = _8;
_4 = 138_u8 as isize;
_24 = !false;
_3 = _9;
_18 = (-1397438499_i32) as f64;
_11 = _18 as f32;
RET = '\u{1b91f}';
_12 = 215_u8 as isize;
_6 = _1;
Goto(bb7)
}
bb11 = {
_16 = _18;
Goto(bb6)
}
bb12 = {
_13 = core::ptr::addr_of_mut!(_14);
_3 = 205_u8 as usize;
_10 = [513426459_i32,(-894626245_i32),(-1706918918_i32),1860888168_i32,(-807838108_i32),(-690788551_i32)];
_19 = 87254464587701533438000947950053589290_u128 & 105459327091974775192838516694777753514_u128;
_7 = _8 & _9;
Goto(bb5)
}
bb13 = {
_11 = (-5621087714818843725_i64) as f32;
_10 = [1929955863_i32,179030691_i32,1274712602_i32,1954810398_i32,(-1030698209_i32),(-1873799305_i32)];
RET = '\u{f79ce}';
_21 = [_9,_8,_8,_7,_8,_7,_9,_9];
_11 = _16 as f32;
_11 = 6848_i16 as f32;
_18 = _16;
_16 = _18;
_18 = _16;
RET = '\u{cc76e}';
_12 = _5;
_21 = [_7,_7,_8,_8,_8,_8,_7,_8];
_18 = _16 - _16;
_18 = -_16;
_3 = _9;
_4 = 3715153251204902868_u64 as isize;
_4 = _1;
_17 = (-1915772279_i32) as i16;
_5 = _2 << _12;
_23 = [(-13_i8),16_i8,26_i8];
_11 = 37912800408037167389093807565966546689_i128 as f32;
_6 = _11 as isize;
Goto(bb4)
}
bb14 = {
_16 = 250_u8 as f64;
RET = '\u{43826}';
_11 = _9 as f32;
_2 = _12;
_10 = [(-1474595748_i32),2116460954_i32,1922043379_i32,1233871392_i32,1458495907_i32,(-824025250_i32)];
_2 = _1 * _6;
_5 = 1126791899_i32 as isize;
RET = '\u{23ed2}';
_7 = !_9;
_5 = !_12;
_2 = -_12;
RET = '\u{7bdb0}';
_18 = 180637964081504415933869222712268705398_u128 as f64;
Goto(bb3)
}
bb15 = {
RET = _27.fld3.0;
_8 = _3;
_5 = _19 as isize;
_13 = core::ptr::addr_of_mut!(_14);
_9 = _3 * _27.fld3.1;
_8 = _3;
_29 = 32341_u16 as f32;
_25 = RET;
_27.fld5 = core::ptr::addr_of_mut!((*_13));
_13 = core::ptr::addr_of_mut!((*_13));
_11 = -_29;
_6 = _11 as isize;
_17 = 66_u8 as i16;
_11 = -_29;
_14 = core::ptr::addr_of!(_30);
(*_14) = (3650940815_u32, _7, 3955220295_u32);
(*_13) = core::ptr::addr_of!((*_14));
_26 = _4 * _4;
_27.fld2 = Adt30::Variant0 { fld0: 7207917098166217485_u64,fld1: 124058859526289618386438301852986002085_i128 };
_30.1 = _17 as usize;
RET = _25;
_28 = _24;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(10_usize, 28_usize, Move(_28), 3_usize, Move(_3), 24_usize, Move(_24), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(10_usize, 12_usize, Move(_12), 1_usize, Move(_1), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(10_usize, 2_usize, Move(_2), 4_usize, Move(_4), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: usize,mut _3: isize,mut _4: usize) -> isize {
mir! {
type RET = isize;
let _5: &'static &'static i32;
let _6: &'static (f32,);
let _7: ();
let _8: ();
{
_3 = !_1;
RET = _1;
RET = -_3;
RET = 42759050736684878655106514136484823943_u128 as isize;
RET = !_1;
RET = _3 ^ _1;
_1 = !_3;
_4 = _2;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(11_usize, 4_usize, Move(_4), 1_usize, Move(_1), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: usize,mut _2: (u32, usize, u32),mut _3: u32,mut _4: isize,mut _5: usize) -> bool {
mir! {
type RET = bool;
let _6: f32;
let _7: isize;
let _8: Adt29;
let _9: (char, usize);
let _10: Adt53;
let _11: i128;
let _12: char;
let _13: &'static [i64; 2];
let _14: i8;
let _15: i128;
let _16: Adt52;
let _17: [i16; 6];
let _18: u32;
let _19: u16;
let _20: f64;
let _21: isize;
let _22: [u16; 5];
let _23: [i32; 6];
let _24: char;
let _25: [i128; 8];
let _26: f32;
let _27: ([i32; 6],);
let _28: isize;
let _29: [u16; 7];
let _30: f64;
let _31: [i64; 3];
let _32: isize;
let _33: isize;
let _34: ();
let _35: ();
{
RET = !false;
Goto(bb1)
}
bb1 = {
_5 = RET as usize;
_2.1 = _1;
_2.1 = _1;
_4 = 9223372036854775807_isize & (-9223372036854775808_isize);
_2 = (_3, _1, _3);
_2 = (_3, _1, _3);
Goto(bb2)
}
bb2 = {
_5 = !_1;
_5 = _1 + _1;
_2.2 = 75_u8 as u32;
_2.0 = _3 ^ _3;
_1 = _2.1;
RET = !false;
_1 = !_5;
RET = false ^ true;
_5 = _1;
_2 = (_3, _1, _3);
_4 = (-58_isize);
RET = !true;
_4 = 30_isize;
_6 = 1922_i16 as f32;
_6 = 58544594571958359026473467112702485153_u128 as f32;
_4 = -77_isize;
_2 = (_3, _1, _3);
_3 = _2.0 % _2.0;
RET = false;
_7 = -_4;
_2.0 = _3 ^ _2.2;
_7 = -_4;
_2.0 = (-2330994031177714218018111755742529759_i128) as u32;
Call(_6 = core::intrinsics::transmute(_2.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = true;
_2.0 = RET as u32;
Goto(bb4)
}
bb4 = {
_2.0 = _2.2 * _2.2;
_7 = _4;
_9.1 = _1 << _5;
_4 = 46_u8 as isize;
_2 = (_3, _1, _3);
_10.fld4 = !18743_i16;
_10.fld0 = (777246872574129254_u64,);
_11 = (-27245009189590146106706583588415055136_i128) & (-68060230647417164748165462224573237552_i128);
_10.fld2 = (_6,);
_10.fld5 = 744011145_i32;
_3 = _11 as u32;
_9.0 = '\u{9b6f}';
_1 = _10.fld4 as usize;
match _10.fld0.0 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
777246872574129254 => bb8,
_ => bb7
}
}
bb5 = {
RET = true;
_2.0 = RET as u32;
Goto(bb4)
}
bb6 = {
_5 = !_1;
_5 = _1 + _1;
_2.2 = 75_u8 as u32;
_2.0 = _3 ^ _3;
_1 = _2.1;
RET = !false;
_1 = !_5;
RET = false ^ true;
_5 = _1;
_2 = (_3, _1, _3);
_4 = (-58_isize);
RET = !true;
_4 = 30_isize;
_6 = 1922_i16 as f32;
_6 = 58544594571958359026473467112702485153_u128 as f32;
_4 = -77_isize;
_2 = (_3, _1, _3);
_3 = _2.0 % _2.0;
RET = false;
_7 = -_4;
_2.0 = _3 ^ _2.2;
_7 = -_4;
_2.0 = (-2330994031177714218018111755742529759_i128) as u32;
Call(_6 = core::intrinsics::transmute(_2.2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_5 = RET as usize;
_2.1 = _1;
_2.1 = _1;
_4 = 9223372036854775807_isize & (-9223372036854775808_isize);
_2 = (_3, _1, _3);
_2 = (_3, _1, _3);
Goto(bb2)
}
bb8 = {
_2.0 = _2.2 + _2.2;
_10.fld1 = _9.0;
_12 = _9.0;
_1 = _10.fld4 as usize;
_4 = !_7;
_1 = _3 as usize;
_3 = _2.2 * _2.0;
_12 = _9.0;
_9 = (_12, _2.1);
_4 = _7;
RET = true;
_14 = (-115_i8) ^ (-51_i8);
match _10.fld0.0 {
777246872574129254 => bb9,
_ => bb3
}
}
bb9 = {
_10.fld4 = (-15917_i16);
_11 = (-104031630625795385563440853540560824826_i128);
_10.fld3 = core::ptr::addr_of!(_2);
_9 = (_12, _2.1);
_9.0 = _12;
RET = false;
_4 = _7;
_2.0 = !_3;
_9 = (_12, _5);
Goto(bb10)
}
bb10 = {
_10.fld1 = _9.0;
_9.0 = _10.fld1;
_3 = _2.0 * _2.0;
_4 = _11 as isize;
_8 = Adt29::Variant2 { fld0: _11,fld1: _9 };
_2 = (_3, _9.1, _3);
_15 = Field::<i128>(Variant(_8, 2), 0);
_7 = _4 >> _2.1;
_10.fld3 = core::ptr::addr_of!(_2);
_10.fld2.0 = 6543_u16 as f32;
_4 = _7 ^ _7;
_9.0 = Field::<(char, usize)>(Variant(_8, 2), 1).0;
_6 = _10.fld2.0 + _10.fld2.0;
_20 = _6 as f64;
_18 = RET as u32;
_8 = Adt29::Variant2 { fld0: _11,fld1: _9 };
_2 = (_3, _5, _3);
_21 = _4;
_17 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
_3 = !_2.2;
_23 = [_10.fld5,_10.fld5,_10.fld5,_10.fld5,_10.fld5,_10.fld5];
Goto(bb11)
}
bb11 = {
_9 = (Field::<(char, usize)>(Variant(_8, 2), 1).0, _5);
_19 = _7 as u16;
place!(Field::<i128>(Variant(_8, 2), 0)) = RET as i128;
_15 = _11 & _11;
_24 = Field::<(char, usize)>(Variant(_8, 2), 1).0;
place!(Field::<(char, usize)>(Variant(_8, 2), 1)) = (_12, _5);
_6 = _10.fld2.0 * _10.fld2.0;
_25 = [Field::<i128>(Variant(_8, 2), 0),_11,_11,_15,Field::<i128>(Variant(_8, 2), 0),_15,_11,_15];
_6 = _10.fld2.0;
_15 = _11 << _2.2;
place!(Field::<i128>(Variant(_8, 2), 0)) = !_15;
Call(_19 = core::intrinsics::bswap(18142_u16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10.fld2.0 = _10.fld4 as f32;
_29 = [_19,_19,_19,_19,_19,_19,_19];
_5 = !Field::<(char, usize)>(Variant(_8, 2), 1).1;
_28 = _21 >> _5;
_10.fld2 = (_6,);
_10.fld2 = (_6,);
_12 = _10.fld1;
_12 = Field::<(char, usize)>(Variant(_8, 2), 1).0;
_10.fld0 = (6480276910771040829_u64,);
place!(Field::<i128>(Variant(_8, 2), 0)) = !_15;
_25 = [Field::<i128>(Variant(_8, 2), 0),_15,Field::<i128>(Variant(_8, 2), 0),_15,Field::<i128>(Variant(_8, 2), 0),Field::<i128>(Variant(_8, 2), 0),_15,_15];
_10.fld3 = core::ptr::addr_of!(_2);
RET = false;
_10.fld0 = (18320145123462666544_u64,);
_24 = _9.0;
_6 = _20 as f32;
_9.0 = _24;
_6 = _10.fld2.0 + _10.fld2.0;
SetDiscriminant(_8, 2);
_10.fld0.0 = 13688997863678595080_u64 & 12288856369001739813_u64;
_24 = _9.0;
_17 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
match _10.fld4 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431768195539 => bb17,
_ => bb16
}
}
bb13 = {
_5 = RET as usize;
_2.1 = _1;
_2.1 = _1;
_4 = 9223372036854775807_isize & (-9223372036854775808_isize);
_2 = (_3, _1, _3);
_2 = (_3, _1, _3);
Goto(bb2)
}
bb14 = {
_10.fld1 = _9.0;
_9.0 = _10.fld1;
_3 = _2.0 * _2.0;
_4 = _11 as isize;
_8 = Adt29::Variant2 { fld0: _11,fld1: _9 };
_2 = (_3, _9.1, _3);
_15 = Field::<i128>(Variant(_8, 2), 0);
_7 = _4 >> _2.1;
_10.fld3 = core::ptr::addr_of!(_2);
_10.fld2.0 = 6543_u16 as f32;
_4 = _7 ^ _7;
_9.0 = Field::<(char, usize)>(Variant(_8, 2), 1).0;
_6 = _10.fld2.0 + _10.fld2.0;
_20 = _6 as f64;
_18 = RET as u32;
_8 = Adt29::Variant2 { fld0: _11,fld1: _9 };
_2 = (_3, _5, _3);
_21 = _4;
_17 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
_3 = !_2.2;
_23 = [_10.fld5,_10.fld5,_10.fld5,_10.fld5,_10.fld5,_10.fld5];
Goto(bb11)
}
bb15 = {
RET = true;
_2.0 = RET as u32;
Goto(bb4)
}
bb16 = {
RET = true;
_2.0 = RET as u32;
Goto(bb4)
}
bb17 = {
_27.0 = _23;
_26 = -_10.fld2.0;
_31 = [2513580565621823929_i64,6873993770364871647_i64,3965036788309640751_i64];
_16 = Adt52::Variant1 { fld0: RET,fld1: _27,fld2: 222384616183397657705706792355232187012_u128 };
_28 = -_21;
_9.0 = _12;
_2 = (_3, _5, _3);
RET = Field::<bool>(Variant(_16, 1), 0);
_10.fld4 = !(-4753_i16);
place!(Field::<(char, usize)>(Variant(_8, 2), 1)).1 = _5 - _9.1;
_10.fld0.0 = 15006997689341827664_u64 << _3;
_15 = _11;
_19 = 58127_u16;
_5 = _2.1 - _9.1;
_33 = _21 * _4;
place!(Field::<(char, usize)>(Variant(_8, 2), 1)).1 = !_5;
_4 = _21;
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(12_usize, 7_usize, Move(_7), 4_usize, Move(_4), 31_usize, Move(_31), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(12_usize, 9_usize, Move(_9), 12_usize, Move(_12), 27_usize, Move(_27), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(12_usize, 25_usize, Move(_25), 5_usize, Move(_5), 3_usize, Move(_3), 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: (char, usize),mut _2: (char, usize),mut _3: (char, usize)) -> char {
mir! {
type RET = char;
let _4: [i8; 3];
let _5: bool;
let _6: *mut *mut *const &'static i16;
let _7: &'static &'static i32;
let _8: [u16; 7];
let _9: f32;
let _10: isize;
let _11: u16;
let _12: &'static [i64; 2];
let _13: &'static char;
let _14: (*const Adt31, [u32; 2], *const Adt31);
let _15: u128;
let _16: isize;
let _17: u128;
let _18: isize;
let _19: f32;
let _20: [u16; 5];
let _21: f32;
let _22: i32;
let _23: *mut Adt29;
let _24: i32;
let _25: *const (bool, [usize; 1], &'static i32, (char, usize));
let _26: i8;
let _27: (f32,);
let _28: [i8; 3];
let _29: *const Adt31;
let _30: char;
let _31: [u128; 1];
let _32: u16;
let _33: &'static [i64; 2];
let _34: bool;
let _35: bool;
let _36: [i64; 3];
let _37: [i16; 6];
let _38: isize;
let _39: char;
let _40: [i64; 2];
let _41: i64;
let _42: [u16; 7];
let _43: isize;
let _44: [u128; 8];
let _45: f32;
let _46: f32;
let _47: *mut *mut *const &'static i16;
let _48: i64;
let _49: isize;
let _50: isize;
let _51: (Adt31,);
let _52: i128;
let _53: (Adt78,);
let _54: char;
let _55: isize;
let _56: ();
let _57: ();
{
_1 = _3;
_3.1 = _1.1;
_2 = (_3.0, _1.1);
_5 = false;
RET = _3.0;
_4 = [(-49_i8),(-108_i8),(-99_i8)];
_1.0 = _2.0;
RET = _3.0;
_1.1 = 36475805757145713130937596149682546867_i128 as usize;
_3 = _2;
_3.0 = _1.0;
RET = _3.0;
_5 = true;
_3.1 = _2.1;
_1 = (_3.0, _3.1);
_2.1 = _1.1;
_8 = [57618_u16,12165_u16,17298_u16,34690_u16,46404_u16,21760_u16,17904_u16];
_3.0 = _1.0;
_9 = 6137996663316541313_u64 as f32;
_5 = !false;
_11 = !11990_u16;
_5 = true;
_4 = [62_i8,(-91_i8),(-71_i8)];
Call(_2.1 = core::intrinsics::transmute(_1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.0 = _3.0;
_4 = [112_i8,83_i8,87_i8];
_8 = [_11,_11,_11,_11,_11,_11,_11];
_11 = 48702_u16;
_10 = -28_isize;
_9 = (-18071_i16) as f32;
_14.1 = [2000936338_u32,3072115773_u32];
_15 = !162208300019740299834555523685688520477_u128;
_13 = &_1.0;
_16 = 339249999_u32 as isize;
_10 = -_16;
_10 = _16 * _16;
_16 = RET as isize;
Goto(bb2)
}
bb2 = {
_1.0 = _2.0;
_3.0 = RET;
Goto(bb3)
}
bb3 = {
_19 = -_9;
_19 = _9 - _9;
_1.1 = 619215781_i32 as usize;
_2.1 = (-787966381_i32) as usize;
_18 = _2.0 as isize;
_2.0 = _3.0;
_1 = _3;
_1.0 = _3.0;
_2 = (_3.0, _3.1);
_15 = 2214323522_u32 as u128;
_2.0 = _3.0;
_17 = !_15;
_15 = _17 << _2.1;
_3.1 = !_1.1;
_22 = 64897807478489575308452824711814439766_i128 as i32;
_15 = !_17;
Goto(bb4)
}
bb4 = {
_14.1 = [695199092_u32,3684169823_u32];
_24 = _22;
_3.0 = _1.0;
_13 = &RET;
_2 = ((*_13), _3.1);
_14.1 = [1173815678_u32,2189423345_u32];
_17 = _15 >> _3.1;
_21 = 6276457459833750536_i64 as f32;
_2 = (_3.0, _1.1);
_11 = !23993_u16;
_10 = 114_u8 as isize;
_3 = (RET, _1.1);
_8 = [_11,_11,_11,_11,_11,_11,_11];
_19 = (-8599429512486112977_i64) as f32;
_19 = _9 - _21;
_20 = [_11,_11,_11,_11,_11];
_1.1 = !_3.1;
_19 = (-41486470094205716652896182183887387833_i128) as f32;
_1 = _2;
_2.0 = (*_13);
_3 = _1;
Call(_27 = fn14(_2, Move(_13), _1, _17, _2.1, _17, _17, _3.1, _17, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_27 = (_21,);
_2.1 = !_1.1;
_3.0 = _2.0;
_2 = (_3.0, _3.1);
Goto(bb6)
}
bb6 = {
_26 = (-116_i8) & (-48_i8);
RET = _3.0;
_11 = !32905_u16;
_28 = [_26,_26,_26];
_15 = _24 as u128;
_22 = !_24;
_20 = [_11,_11,_11,_11,_11];
_1.0 = RET;
_26 = (-94_i8) | 38_i8;
Goto(bb7)
}
bb7 = {
_11 = !10898_u16;
_14.1 = [3012977405_u32,2630071444_u32];
_24 = _22 & _22;
_16 = _10 | _10;
_21 = -_19;
_5 = true ^ false;
_13 = &_2.0;
_18 = _10 ^ _16;
RET = (*_13);
RET = _3.0;
_17 = _16 as u128;
_3.0 = _1.0;
_16 = !_18;
_2.1 = _3.1 + _1.1;
_30 = _1.0;
_5 = true & false;
_27 = (_9,);
_24 = _11 as i32;
Goto(bb8)
}
bb8 = {
_31 = [_17];
_2.1 = !_1.1;
_3.1 = _1.1 & _1.1;
_11 = _3.1 as u16;
_3.0 = _30;
_32 = !_11;
_14.1 = [2202242293_u32,933487267_u32];
_19 = -_27.0;
_1.0 = _30;
_27.0 = _21 + _9;
_26 = _11 as i8;
_3.0 = (*_13);
_15 = _17;
_14.1 = [2262041688_u32,1402030514_u32];
_26 = 59_i8;
_5 = _1.1 <= _2.1;
_3.1 = _2.1 ^ _1.1;
_1 = (RET, _3.1);
_2.1 = _16 as usize;
_32 = 68148512214250721626146253510027845365_i128 as u16;
_27 = (_9,);
_2.0 = RET;
match _26 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
59 => bb9,
_ => bb6
}
}
bb9 = {
_34 = _5 < _5;
_5 = _34;
Goto(bb10)
}
bb10 = {
_28 = [_26,_26,_26];
RET = _3.0;
_1 = _3;
_27 = (_21,);
_22 = !_24;
_36 = [4111014255143100178_i64,(-2685404972749571561_i64),(-3297977833349672961_i64)];
_3.1 = !_1.1;
_1 = _3;
_35 = _34;
_34 = !_5;
_1 = (RET, _3.1);
_27 = (_9,);
_16 = !_18;
_27.0 = _1.1 as f32;
_11 = 168_u8 as u16;
_27 = (_21,);
match _26 {
0 => bb8,
1 => bb6,
2 => bb3,
59 => bb11,
_ => bb4
}
}
bb11 = {
_36 = [(-7922885500506234957_i64),3336339424772681107_i64,(-6270577855739980879_i64)];
_4 = [_26,_26,_26];
_26 = -(-124_i8);
_31 = [_17];
_14.1 = [1266145637_u32,1462143767_u32];
_13 = &_3.0;
_2.1 = _3.1;
_37 = [28294_i16,25411_i16,26205_i16,(-3989_i16),3477_i16,32066_i16];
_32 = _11 * _11;
_35 = !_34;
_34 = !_35;
_21 = _24 as f32;
_2.0 = (*_13);
_3.0 = _2.0;
_38 = _18 | _18;
_36 = [1119111176033879328_i64,(-3313449696984771995_i64),(-505172028871008410_i64)];
_3.1 = _1.1 ^ _2.1;
_26 = (-84_i8) - (-37_i8);
_4 = _28;
_24 = _22;
_2.1 = RET as usize;
_34 = _35;
_14.1 = [2987020752_u32,104159634_u32];
_21 = -_27.0;
_24 = _22;
_31 = [_15];
Goto(bb12)
}
bb12 = {
_39 = _30;
_30 = _2.0;
_12 = &_40;
_27 = (_9,);
_31 = [_15];
_27 = (_19,);
_2.0 = RET;
_12 = &(*_12);
_1.0 = _30;
_36 = [8747591786411143542_i64,3314401414845389593_i64,(-8730860973658832517_i64)];
_3.0 = _1.0;
_38 = !_18;
_2.1 = 8010482776069255577_u64 as usize;
_13 = &_2.0;
_43 = !_18;
_44 = [_15,_15,_17,_15,_15,_17,_17,_15];
_3 = (_39, _1.1);
_27.0 = _21 - _21;
_45 = -_21;
_28 = [_26,_26,_26];
_36 = [(-6395952956686623546_i64),(-473934331069511444_i64),(-8056882400602921010_i64)];
_30 = _39;
_35 = !_5;
_12 = &(*_12);
_12 = &_40;
_1 = _3;
_41 = (-215288406106253898_i64);
Goto(bb13)
}
bb13 = {
_36 = [_41,_41,_41];
_3.0 = (*_13);
_26 = (-125_i8);
_20 = [_32,_32,_11,_32,_32];
_42 = _8;
_12 = &_40;
_3.0 = _2.0;
_40 = [_41,_41];
_13 = &_1.0;
_10 = -_18;
_4 = _28;
_30 = _39;
_27.0 = _9;
match _26 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607431768211331 => bb20,
_ => bb19
}
}
bb14 = {
_39 = _30;
_30 = _2.0;
_12 = &_40;
_27 = (_9,);
_31 = [_15];
_27 = (_19,);
_2.0 = RET;
_12 = &(*_12);
_1.0 = _30;
_36 = [8747591786411143542_i64,3314401414845389593_i64,(-8730860973658832517_i64)];
_3.0 = _1.0;
_38 = !_18;
_2.1 = 8010482776069255577_u64 as usize;
_13 = &_2.0;
_43 = !_18;
_44 = [_15,_15,_17,_15,_15,_17,_17,_15];
_3 = (_39, _1.1);
_27.0 = _21 - _21;
_45 = -_21;
_28 = [_26,_26,_26];
_36 = [(-6395952956686623546_i64),(-473934331069511444_i64),(-8056882400602921010_i64)];
_30 = _39;
_35 = !_5;
_12 = &(*_12);
_12 = &_40;
_1 = _3;
_41 = (-215288406106253898_i64);
Goto(bb13)
}
bb15 = {
_27 = (_21,);
_2.1 = !_1.1;
_3.0 = _2.0;
_2 = (_3.0, _3.1);
Goto(bb6)
}
bb16 = {
_28 = [_26,_26,_26];
RET = _3.0;
_1 = _3;
_27 = (_21,);
_22 = !_24;
_36 = [4111014255143100178_i64,(-2685404972749571561_i64),(-3297977833349672961_i64)];
_3.1 = !_1.1;
_1 = _3;
_35 = _34;
_34 = !_5;
_1 = (RET, _3.1);
_27 = (_9,);
_16 = !_18;
_27.0 = _1.1 as f32;
_11 = 168_u8 as u16;
_27 = (_21,);
match _26 {
0 => bb8,
1 => bb6,
2 => bb3,
59 => bb11,
_ => bb4
}
}
bb17 = {
_19 = -_9;
_19 = _9 - _9;
_1.1 = 619215781_i32 as usize;
_2.1 = (-787966381_i32) as usize;
_18 = _2.0 as isize;
_2.0 = _3.0;
_1 = _3;
_1.0 = _3.0;
_2 = (_3.0, _3.1);
_15 = 2214323522_u32 as u128;
_2.0 = _3.0;
_17 = !_15;
_15 = _17 << _2.1;
_3.1 = !_1.1;
_22 = 64897807478489575308452824711814439766_i128 as i32;
_15 = !_17;
Goto(bb4)
}
bb18 = {
_31 = [_17];
_2.1 = !_1.1;
_3.1 = _1.1 & _1.1;
_11 = _3.1 as u16;
_3.0 = _30;
_32 = !_11;
_14.1 = [2202242293_u32,933487267_u32];
_19 = -_27.0;
_1.0 = _30;
_27.0 = _21 + _9;
_26 = _11 as i8;
_3.0 = (*_13);
_15 = _17;
_14.1 = [2262041688_u32,1402030514_u32];
_26 = 59_i8;
_5 = _1.1 <= _2.1;
_3.1 = _2.1 ^ _1.1;
_1 = (RET, _3.1);
_2.1 = _16 as usize;
_32 = 68148512214250721626146253510027845365_i128 as u16;
_27 = (_9,);
_2.0 = RET;
match _26 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
59 => bb9,
_ => bb6
}
}
bb19 = {
_2.0 = _3.0;
_4 = [112_i8,83_i8,87_i8];
_8 = [_11,_11,_11,_11,_11,_11,_11];
_11 = 48702_u16;
_10 = -28_isize;
_9 = (-18071_i16) as f32;
_14.1 = [2000936338_u32,3072115773_u32];
_15 = !162208300019740299834555523685688520477_u128;
_13 = &_1.0;
_16 = 339249999_u32 as isize;
_10 = -_16;
_10 = _16 * _16;
_16 = RET as isize;
Goto(bb2)
}
bb20 = {
_5 = _35;
_48 = _41 | _41;
_41 = _48;
_17 = _15 ^ _15;
_44 = [_17,_17,_15,_17,_15,_17,_17,_17];
_36 = [_48,_48,_48];
_14.2 = core::ptr::addr_of!(_51.0);
_14.0 = core::ptr::addr_of!(_51.0);
_3.1 = !_1.1;
_3.0 = _1.0;
_9 = -_21;
_1 = _3;
_14.0 = core::ptr::addr_of!(_51.0);
_9 = -_27.0;
_14.2 = core::ptr::addr_of!(_51.0);
_54 = RET;
Goto(bb21)
}
bb21 = {
Call(_56 = dump_var(13_usize, 30_usize, Move(_30), 44_usize, Move(_44), 37_usize, Move(_37), 24_usize, Move(_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_56 = dump_var(13_usize, 4_usize, Move(_4), 11_usize, Move(_11), 5_usize, Move(_5), 54_usize, Move(_54)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_56 = dump_var(13_usize, 20_usize, Move(_20), 42_usize, Move(_42), 18_usize, Move(_18), 16_usize, Move(_16)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_56 = dump_var(13_usize, 48_usize, Move(_48), 41_usize, Move(_41), 39_usize, Move(_39), 34_usize, Move(_34)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (char, usize),mut _2: &'static char,mut _3: (char, usize),mut _4: u128,mut _5: usize,mut _6: u128,mut _7: u128,mut _8: usize,mut _9: u128,mut _10: (char, usize)) -> (f32,) {
mir! {
type RET = (f32,);
let _11: Adt52;
let _12: &'static char;
let _13: (*mut *const &'static i16, *const Adt31, Adt30);
let _14: i128;
let _15: i32;
let _16: &'static i128;
let _17: [u32; 5];
let _18: i32;
let _19: (char, *const Adt31, *mut *const (u32, usize, u32));
let _20: &'static &'static i32;
let _21: (f32,);
let _22: f64;
let _23: bool;
let _24: i64;
let _25: u64;
let _26: Adt31;
let _27: isize;
let _28: f32;
let _29: [i8; 3];
let _30: ();
let _31: ();
{
_1.0 = _3.0;
_6 = !_4;
_2 = &_1.0;
RET.0 = _1.1 as f32;
_3 = ((*_2), _10.1);
_4 = _6 + _6;
_8 = _3.1;
_10 = _1;
_3 = (_1.0, _8);
_3 = ((*_2), _8);
_10.1 = !_8;
_1.1 = !_3.1;
_6 = !_4;
_10.0 = _1.0;
_9 = _7 >> _3.1;
_7 = _1.1 as u128;
_10 = _1;
_10.1 = _8 ^ _1.1;
_10.1 = _1.1 * _8;
_10.0 = _1.0;
_3.0 = _1.0;
_3.0 = (*_2);
RET.0 = 113_isize as f32;
_12 = &_3.0;
Goto(bb1)
}
bb1 = {
RET.0 = (-78_isize) as f32;
_5 = _8 - _3.1;
_3 = ((*_2), _5);
_8 = _10.1 & _1.1;
_12 = Move(_2);
_2 = &_10.0;
_12 = &(*_2);
_10.0 = _1.0;
RET.0 = (-23389_i16) as f32;
_15 = 1391625376_i32;
_4 = _6 & _7;
_3.1 = _8;
_9 = 2364082241_u32 as u128;
RET.0 = 127866144_u32 as f32;
_1.1 = _10.1 >> _7;
_2 = &_10.0;
RET.0 = 98_u8 as f32;
_16 = &_14;
_10.0 = _1.0;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1391625376 => bb7,
_ => bb6
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
_9 = _7 | _4;
_3.0 = _1.0;
_5 = !_1.1;
_12 = &_3.0;
_12 = &(*_12);
_7 = 54493_u16 as u128;
RET.0 = 178_u8 as f32;
_6 = _4 ^ _4;
_16 = &(*_16);
_3.0 = _1.0;
_12 = &_1.0;
_7 = _6 + _9;
_6 = _4 | _7;
match _15 {
1391625376 => bb8,
_ => bb3
}
}
bb8 = {
_1 = (_10.0, _5);
_2 = &_10.0;
_1 = ((*_2), _10.1);
_3 = _10;
_6 = _4;
_1.0 = _10.0;
_10.0 = _1.0;
_6 = 2101446025_u32 as u128;
_7 = !_4;
_6 = _4;
_7 = !_6;
_18 = _15 | _15;
_1.1 = _5;
_4 = (-66_i8) as u128;
_5 = !_1.1;
_12 = &_1.0;
_5 = 17084936913716350442_u64 as usize;
_14 = (-166782186915968844724232933555999344796_i128) ^ (-154441659315779244793605282703498157557_i128);
_2 = Move(_12);
_13.2 = Adt30::Variant0 { fld0: 15297028819774748053_u64,fld1: _14 };
Goto(bb9)
}
bb9 = {
_8 = !_1.1;
_10.0 = _3.0;
_14 = Field::<i128>(Variant(_13.2, 0), 1) - Field::<i128>(Variant(_13.2, 0), 1);
_1.0 = _10.0;
RET.0 = _3.1 as f32;
_2 = &_1.0;
_3 = ((*_2), _10.1);
_13.2 = Adt30::Variant0 { fld0: 16529021596113875876_u64,fld1: _14 };
_17 = [2036894568_u32,745654657_u32,364914237_u32,1826217846_u32,2813046430_u32];
_19.0 = (*_2);
_17 = [3076430938_u32,2578680047_u32,2855659750_u32,2732898651_u32,3258681789_u32];
RET.0 = Field::<i128>(Variant(_13.2, 0), 1) as f32;
_18 = !_15;
_21.0 = RET.0 - RET.0;
place!(Field::<i128>(Variant(_13.2, 0), 1)) = _14;
_10.0 = (*_2);
_9 = !_6;
_16 = &place!(Field::<i128>(Variant(_13.2, 0), 1));
_12 = Move(_2);
_4 = !_7;
_7 = (-2281155197094534258_i64) as u128;
_10.1 = 3127830105708194843_i64 as usize;
_7 = !_6;
_3.0 = _10.0;
_1 = _3;
match _15 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
1391625376 => bb16,
_ => bb15
}
}
bb10 = {
_1 = (_10.0, _5);
_2 = &_10.0;
_1 = ((*_2), _10.1);
_3 = _10;
_6 = _4;
_1.0 = _10.0;
_10.0 = _1.0;
_6 = 2101446025_u32 as u128;
_7 = !_4;
_6 = _4;
_7 = !_6;
_18 = _15 | _15;
_1.1 = _5;
_4 = (-66_i8) as u128;
_5 = !_1.1;
_12 = &_1.0;
_5 = 17084936913716350442_u64 as usize;
_14 = (-166782186915968844724232933555999344796_i128) ^ (-154441659315779244793605282703498157557_i128);
_2 = Move(_12);
_13.2 = Adt30::Variant0 { fld0: 15297028819774748053_u64,fld1: _14 };
Goto(bb9)
}
bb11 = {
_9 = _7 | _4;
_3.0 = _1.0;
_5 = !_1.1;
_12 = &_3.0;
_12 = &(*_12);
_7 = 54493_u16 as u128;
RET.0 = 178_u8 as f32;
_6 = _4 ^ _4;
_16 = &(*_16);
_3.0 = _1.0;
_12 = &_1.0;
_7 = _6 + _9;
_6 = _4 | _7;
match _15 {
1391625376 => bb8,
_ => bb3
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
RET.0 = (-78_isize) as f32;
_5 = _8 - _3.1;
_3 = ((*_2), _5);
_8 = _10.1 & _1.1;
_12 = Move(_2);
_2 = &_10.0;
_12 = &(*_2);
_10.0 = _1.0;
RET.0 = (-23389_i16) as f32;
_15 = 1391625376_i32;
_4 = _6 & _7;
_3.1 = _8;
_9 = 2364082241_u32 as u128;
RET.0 = 127866144_u32 as f32;
_1.1 = _10.1 >> _7;
_2 = &_10.0;
RET.0 = 98_u8 as f32;
_16 = &_14;
_10.0 = _1.0;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1391625376 => bb7,
_ => bb6
}
}
bb15 = {
Return()
}
bb16 = {
_1.0 = _10.0;
_12 = &_19.0;
_15 = !_18;
_2 = &_10.0;
_1.0 = (*_12);
RET.0 = _21.0;
_22 = _14 as f64;
_1.0 = _10.0;
_21 = (RET.0,);
_5 = _8;
_7 = _8 as u128;
_23 = !true;
_7 = _6;
_10.1 = _1.1 + _5;
_3 = (_10.0, _5);
_27 = !6_isize;
_17 = [1234378835_u32,3850309862_u32,3914220355_u32,3260972588_u32,3196136164_u32];
_1.0 = (*_2);
_17 = [2633791841_u32,1587718386_u32,1772527198_u32,3052904569_u32,2549259252_u32];
_16 = &(*_16);
_27 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3.1 = _4 as usize;
_16 = &_14;
_15 = _18;
_5 = _1.1 - _1.1;
_23 = false;
_22 = 66_i8 as f64;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(14_usize, 23_usize, Move(_23), 7_usize, Move(_7), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(14_usize, 8_usize, Move(_8), 5_usize, Move(_5), 15_usize, Move(_15), 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static char,mut _2: bool,mut _3: (char, usize),mut _4: usize,mut _5: *mut Adt29,mut _6: usize,mut _7: (char, usize),mut _8: usize) -> bool {
mir! {
type RET = bool;
let _9: ();
let _10: ();
{
RET = _2;
_1 = &_7.0;
_3 = ((*_1), _8);
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(15_usize, 6_usize, Move(_6), 4_usize, Move(_4), 3_usize, Move(_3), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16() -> isize {
mir! {
type RET = isize;
let _1: [u32; 5];
let _2: isize;
let _3: u128;
let _4: f64;
let _5: Adt80;
let _6: char;
let _7: *mut *const (u32, usize, u32);
let _8: f32;
let _9: &'static i32;
let _10: *mut &'static i128;
let _11: [i128; 3];
let _12: f32;
let _13: i8;
let _14: char;
let _15: &'static [i64; 2];
let _16: i8;
let _17: (u128, Adt52, (u32, usize, u32));
let _18: *mut *mut *const &'static i16;
let _19: isize;
let _20: [i64; 3];
let _21: i16;
let _22: [i8; 3];
let _23: char;
let _24: u16;
let _25: *const [usize; 1];
let _26: f32;
let _27: u128;
let _28: ();
let _29: ();
{
RET = 89_u8 as isize;
RET = !95_isize;
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize;
_2 = !RET;
_1 = [2439187681_u32,4150169155_u32,3524017808_u32,3609942519_u32,3077800306_u32];
RET = 143777395147748666776568903978607595491_u128 as isize;
Call(_2 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [3875282440_u32,3067991121_u32,2988889191_u32,3395713966_u32,2009344406_u32];
_2 = RET;
RET = (-1254454198_i32) as isize;
_2 = RET;
RET = false as isize;
_2 = (-170203631_i32) as isize;
_1 = [2496269445_u32,3548692188_u32,333053563_u32,1730662934_u32,2297970057_u32];
RET = 12_u8 as isize;
RET = 110_u8 as isize;
_3 = 42896_u16 as u128;
RET = !_2;
_1 = [3141318219_u32,1440793603_u32,90570017_u32,1662499491_u32,3349156667_u32];
_1 = [4147790864_u32,207202072_u32,123527626_u32,3169546593_u32,314080615_u32];
_2 = RET;
_3 = 204523427411747995068705549885934881213_u128;
_1 = [1075796800_u32,2711190217_u32,252793426_u32,1589134916_u32,1764587653_u32];
Call(_2 = fn17(_1, _1, _1, _3, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = RET * RET;
_2 = RET ^ RET;
_3 = 48804_u16 as u128;
_3 = false as u128;
RET = -_2;
RET = -_2;
_2 = !RET;
_1 = [3207899467_u32,214652109_u32,1883085050_u32,2235799688_u32,2939616460_u32];
_2 = !RET;
_4 = 5686849279344595453_u64 as f64;
_2 = !RET;
_4 = 512533770_i32 as f64;
_1 = [818609993_u32,2458220072_u32,1880779350_u32,3105697506_u32,2268532004_u32];
_1 = [1199202728_u32,950293173_u32,2913727417_u32,590608097_u32,3606063383_u32];
_2 = (-150948790109813318724508893187144371534_i128) as isize;
_4 = 0_u8 as f64;
_2 = -RET;
RET = !_2;
RET = _2;
RET = 58589325291473737177578504661077681633_i128 as isize;
_3 = _4 as u128;
_2 = 4_usize as isize;
_4 = 1819855066_i32 as f64;
_6 = '\u{b1386}';
RET = (-24_i8) as isize;
RET = -_2;
_6 = '\u{e07b3}';
_6 = '\u{e9715}';
Goto(bb3)
}
bb3 = {
_3 = 302093382360154829825457503257687426970_u128;
RET = _2 - _2;
_4 = 84812110_i32 as f64;
_4 = (-141315494986743605417966159824463136017_i128) as f64;
_1 = [3577026822_u32,877180343_u32,3750886355_u32,1542133467_u32,2186288545_u32];
RET = _2;
_4 = 7279875549547059609_usize as f64;
_6 = '\u{4f488}';
_6 = '\u{746c9}';
_1 = [1597392502_u32,3800487684_u32,1575461267_u32,1611454291_u32,3568857537_u32];
RET = -_2;
RET = _2;
_4 = 6706238644336319309_i64 as f64;
_1 = [656382691_u32,142471109_u32,3188454588_u32,1168901022_u32,1855907847_u32];
RET = _2 * _2;
_3 = 52413188825965424980981981300837690313_u128 | 63010369008570825368893742078709535325_u128;
RET = -_2;
_1 = [3226384901_u32,3128829945_u32,1218481403_u32,3054669792_u32,3604055901_u32];
_6 = '\u{b0909}';
Goto(bb4)
}
bb4 = {
_4 = 1135_u16 as f64;
_8 = 1423525544_u32 as f32;
RET = _2;
_6 = '\u{42db5}';
_1 = [1552668908_u32,2233105886_u32,2208002870_u32,1327622517_u32,1617615627_u32];
_6 = '\u{8c940}';
_3 = _4 as u128;
_2 = -RET;
_3 = !238883328194380243123481580502390553356_u128;
Goto(bb5)
}
bb5 = {
_1 = [2154818298_u32,2254517627_u32,1818840523_u32,3110798589_u32,2411560316_u32];
Goto(bb6)
}
bb6 = {
_11 = [58749732052402114559636778372947644608_i128,(-129964939119883504011713581047357731129_i128),14653178679802734651855643952181945571_i128];
_11 = [86922192072924123999695297112727867379_i128,6100342472781828485946007593550242497_i128,165910509422773109263934405287059545699_i128];
RET = 6586_u16 as isize;
RET = 3664273385419368394_u64 as isize;
_3 = 284613801026130057479258779003200825034_u128;
_4 = 12136782672300026632_u64 as f64;
_12 = _8;
_11 = [(-137892312306096164304175880165827087995_i128),(-164045495931403517668385919682212482231_i128),1727381207644727027083784484395908025_i128];
_1 = [764197044_u32,935814205_u32,608113931_u32,3606577753_u32,3249873751_u32];
_14 = _6;
match _3 {
284613801026130057479258779003200825034 => bb7,
_ => bb1
}
}
bb7 = {
_2 = RET;
_13 = -123_i8;
_2 = RET | RET;
_14 = _6;
_6 = _14;
_4 = 3696626295_u32 as f64;
_1 = [3671362747_u32,4237670729_u32,1088990080_u32,3592816388_u32,1742976973_u32];
_8 = -_12;
_8 = _12 * _12;
_12 = _8 * _8;
Goto(bb8)
}
bb8 = {
_17.2.1 = !4_usize;
RET = _2 + _2;
_16 = !_13;
_4 = 159289921161554433829176859325134247800_i128 as f64;
_17.2 = (403735328_u32, 4808686547648441284_usize, 518694525_u32);
_17.0 = _3;
_6 = _14;
_11 = [(-59876478765907663636039788567421889735_i128),(-170095576891488860688982681380453422870_i128),(-15182982377089528152823846648881459665_i128)];
_12 = RET as f32;
_1 = [_17.2.2,_17.2.2,_17.2.2,_17.2.2,_17.2.2];
_4 = _16 as f64;
_17.2 = (1926624837_u32, 3_usize, 3241848493_u32);
_17.2.0 = _17.2.2 ^ _17.2.2;
_17.0 = _3;
_19 = _12 as isize;
_4 = (-961121998791613413_i64) as f64;
_17.2.2 = !_17.2.0;
match _17.2.1 {
0 => bb1,
1 => bb5,
3 => bb10,
_ => bb9
}
}
bb9 = {
_3 = 302093382360154829825457503257687426970_u128;
RET = _2 - _2;
_4 = 84812110_i32 as f64;
_4 = (-141315494986743605417966159824463136017_i128) as f64;
_1 = [3577026822_u32,877180343_u32,3750886355_u32,1542133467_u32,2186288545_u32];
RET = _2;
_4 = 7279875549547059609_usize as f64;
_6 = '\u{4f488}';
_6 = '\u{746c9}';
_1 = [1597392502_u32,3800487684_u32,1575461267_u32,1611454291_u32,3568857537_u32];
RET = -_2;
RET = _2;
_4 = 6706238644336319309_i64 as f64;
_1 = [656382691_u32,142471109_u32,3188454588_u32,1168901022_u32,1855907847_u32];
RET = _2 * _2;
_3 = 52413188825965424980981981300837690313_u128 | 63010369008570825368893742078709535325_u128;
RET = -_2;
_1 = [3226384901_u32,3128829945_u32,1218481403_u32,3054669792_u32,3604055901_u32];
_6 = '\u{b0909}';
Goto(bb4)
}
bb10 = {
_17.2.2 = false as u32;
_11 = [(-99895579345398419680850567707976096120_i128),(-113551301951132321875798774742358398113_i128),7575619976246111953395996562690041133_i128];
_17.0 = _19 as u128;
_12 = -_8;
_11 = [(-28727588588080933272384919757597896405_i128),166177824149207322712268252550036800825_i128,(-78707984041541340489443553876488286358_i128)];
_16 = _17.2.1 as i8;
_16 = _13;
_19 = 13610_i16 as isize;
_12 = _8 * _8;
_2 = !RET;
_21 = 14086_i16;
_17.2 = (2183819482_u32, 16701543486649087943_usize, 3424511963_u32);
_1 = [_17.2.0,_17.2.2,_17.2.0,_17.2.2,_17.2.0];
_3 = _17.0;
_6 = _14;
RET = _19;
_8 = -_12;
match _17.2.2 {
0 => bb4,
1 => bb9,
2 => bb5,
3 => bb11,
3424511963 => bb13,
_ => bb12
}
}
bb11 = {
_2 = RET;
_13 = -123_i8;
_2 = RET | RET;
_14 = _6;
_6 = _14;
_4 = 3696626295_u32 as f64;
_1 = [3671362747_u32,4237670729_u32,1088990080_u32,3592816388_u32,1742976973_u32];
_8 = -_12;
_8 = _12 * _12;
_12 = _8 * _8;
Goto(bb8)
}
bb12 = {
_1 = [2154818298_u32,2254517627_u32,1818840523_u32,3110798589_u32,2411560316_u32];
Goto(bb6)
}
bb13 = {
_19 = _2;
_8 = _12;
_17.2.2 = _3 as u32;
Goto(bb14)
}
bb14 = {
_17.2.2 = _19 as u32;
_4 = _13 as f64;
_17.2.0 = !_17.2.2;
_16 = false as i8;
_2 = _19;
_11 = [39724761397955365946431261362729723958_i128,33568996983982237157296275213646181194_i128,(-60692208937918248408512741285227474530_i128)];
_2 = RET ^ _19;
_8 = -_12;
_24 = (-88055949840689106350774726974089501489_i128) as u16;
_20 = [5571265194800826701_i64,(-7324857306437164388_i64),8626960061526517564_i64];
RET = _19;
_22 = [_13,_13,_16];
_22 = [_16,_13,_16];
_17.0 = _24 as u128;
_24 = 388_u16;
_23 = _14;
_17.0 = _3;
_17.2.1 = _8 as usize;
RET = _2;
_1 = [_17.2.2,_17.2.2,_17.2.2,_17.2.2,_17.2.2];
_20 = [(-6687060436576699113_i64),(-6051173323373888405_i64),2716177980649337095_i64];
_12 = _21 as f32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(16_usize, 14_usize, Move(_14), 16_usize, Move(_16), 23_usize, Move(_23), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(16_usize, 3_usize, Move(_3), 20_usize, Move(_20), 24_usize, Move(_24), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [u32; 5],mut _2: [u32; 5],mut _3: [u32; 5],mut _4: u128,mut _5: [u32; 5],mut _6: [u32; 5]) -> isize {
mir! {
type RET = isize;
let _7: f32;
let _8: Adt30;
let _9: u16;
let _10: (char, *const Adt31, *mut *const (u32, usize, u32));
let _11: &'static i32;
let _12: char;
let _13: i128;
let _14: u16;
let _15: (*mut *const &'static i16, *const Adt31, Adt30);
let _16: *const (bool, [usize; 1], &'static i32, (char, usize));
let _17: u64;
let _18: f32;
let _19: isize;
let _20: u8;
let _21: Adt53;
let _22: isize;
let _23: isize;
let _24: u8;
let _25: bool;
let _26: f64;
let _27: i32;
let _28: [usize; 1];
let _29: (*const Adt31, [u32; 2], *const Adt31);
let _30: char;
let _31: [i16; 6];
let _32: bool;
let _33: [u16; 5];
let _34: [i64; 3];
let _35: isize;
let _36: [u128; 8];
let _37: [i64; 3];
let _38: f32;
let _39: f64;
let _40: u16;
let _41: ();
let _42: ();
{
_5 = _3;
_3 = [4056791776_u32,2835983529_u32,59549496_u32,559302045_u32,3059809021_u32];
_3 = [1332826930_u32,3672382556_u32,1829661785_u32,4128340807_u32,566005957_u32];
RET = (-1_isize);
_1 = [3505975314_u32,3150721479_u32,2238430225_u32,773617309_u32,3552094260_u32];
RET = 9223372036854775807_isize ^ (-97_isize);
_6 = [3039675559_u32,2654957582_u32,1737523781_u32,1201584033_u32,2266999654_u32];
RET = (-149096594184677757899004687663627382652_i128) as isize;
RET = 9223372036854775807_isize;
RET = 25_isize | (-9223372036854775808_isize);
RET = -19_isize;
_6 = [3253268443_u32,222093653_u32,3355970747_u32,349643459_u32,2320422059_u32];
_4 = 14753541106768793836761231166958823745_u128 - 38172336129203765359092744174459888553_u128;
RET = (-9223372036854775808_isize);
_4 = !322303039989385645862812402691210071961_u128;
_5 = [3716340659_u32,3696066995_u32,2372717396_u32,677837488_u32,1766994454_u32];
RET = -80_isize;
RET = (-36_isize) | (-9223372036854775808_isize);
RET = (-6055_i16) as isize;
RET = 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_8 = Adt30::Variant0 { fld0: 13857028902660233643_u64,fld1: 45575509280165024215281749880165493148_i128 };
RET = (-43_isize) * (-9223372036854775808_isize);
_3 = [2765252154_u32,1566158078_u32,2049821749_u32,2052015731_u32,1004579416_u32];
_3 = [4003964897_u32,1578251367_u32,2555734030_u32,1869176513_u32,1683292747_u32];
RET = (-13_isize) ^ 9223372036854775807_isize;
place!(Field::<u64>(Variant(_8, 0), 0)) = 16550476772040889899_u64;
place!(Field::<u64>(Variant(_8, 0), 0)) = 17500182563537859134_u64;
place!(Field::<i128>(Variant(_8, 0), 1)) = 141833898916550964183593108161201856276_i128 ^ 139875249089959850337953666108642768105_i128;
place!(Field::<u64>(Variant(_8, 0), 0)) = 6442134409208545385_u64 * 10040027562653227883_u64;
Goto(bb2)
}
bb2 = {
_7 = Field::<i128>(Variant(_8, 0), 1) as f32;
_7 = 3403312368_u32 as f32;
_2 = _3;
_9 = 26690_u16 >> RET;
_2 = _3;
_5 = [3605288006_u32,2805774624_u32,3118392821_u32,1781092999_u32,1304535435_u32];
RET = 9223372036854775807_isize;
_1 = [3802933748_u32,1169645253_u32,2221317984_u32,1246092169_u32,2316946944_u32];
_2 = [2442967778_u32,2562415802_u32,207933880_u32,3897497385_u32,341040021_u32];
place!(Field::<u64>(Variant(_8, 0), 0)) = (-2049952285_i32) as u64;
_3 = [2388235590_u32,418070776_u32,1328280264_u32,2371610247_u32,1197710864_u32];
_2 = _5;
_1 = _2;
_2 = [1155683672_u32,1541338396_u32,929213044_u32,1788058515_u32,2607492663_u32];
place!(Field::<u64>(Variant(_8, 0), 0)) = '\u{ee807}' as u64;
place!(Field::<u64>(Variant(_8, 0), 0)) = 4639777630815459087_u64 * 6404223852977324699_u64;
RET = (-9223372036854775808_isize) >> Field::<i128>(Variant(_8, 0), 1);
SetDiscriminant(_8, 2);
_12 = '\u{8f30b}';
place!(Field::<usize>(Variant(_8, 2), 0)) = 1_usize - 0_usize;
RET = (-117_i8) as isize;
place!(Field::<u128>(Variant(_8, 2), 2)) = !_4;
place!(Field::<usize>(Variant(_8, 2), 0)) = !16013455802830566816_usize;
_5 = [3430945033_u32,2101612218_u32,2934243404_u32,84488159_u32,1781564254_u32];
_7 = RET as f32;
_12 = '\u{32bb6}';
_7 = 103469527475843078944600603699710779516_i128 as f32;
Goto(bb3)
}
bb3 = {
_1 = _6;
place!(Field::<u128>(Variant(_8, 2), 2)) = _4;
_10.0 = _12;
_14 = _9;
place!(Field::<(u64,)>(Variant(_8, 2), 3)).0 = true as u64;
place!(Field::<usize>(Variant(_8, 2), 0)) = 3_usize + 6_usize;
_13 = (-158201145420138913298805377657277191212_i128);
_13 = -(-63091781947109518471936765170944362474_i128);
_4 = !Field::<u128>(Variant(_8, 2), 2);
_2 = [1374334667_u32,3668858061_u32,3629668195_u32,3357407193_u32,928288506_u32];
place!(Field::<usize>(Variant(_8, 2), 0)) = !0_usize;
place!(Field::<i16>(Variant(_8, 2), 4)) = (-25863_i16);
place!(Field::<i16>(Variant(_8, 2), 4)) = !19557_i16;
_5 = _1;
place!(Field::<usize>(Variant(_8, 2), 0)) = (-7583648956831506428_i64) as usize;
_4 = !Field::<u128>(Variant(_8, 2), 2);
_17 = Field::<(u64,)>(Variant(_8, 2), 3).0;
place!(Field::<(u64,)>(Variant(_8, 2), 3)).0 = _17 - _17;
_20 = !44_u8;
RET = 2717402529_u32 as isize;
_4 = !Field::<u128>(Variant(_8, 2), 2);
_7 = Field::<i16>(Variant(_8, 2), 4) as f32;
_7 = 2094600512_i32 as f32;
RET = true as isize;
Goto(bb4)
}
bb4 = {
_3 = [2728022585_u32,3447234762_u32,16813369_u32,997665421_u32,3509096176_u32];
place!(Field::<(u64,)>(Variant(_8, 2), 3)).0 = !_17;
_10.0 = _12;
_3 = [2108349079_u32,3222564375_u32,1240964921_u32,2135581225_u32,1594106236_u32];
_14 = _9 ^ _9;
_9 = 1316828021_u32 as u16;
_10.2 = core::ptr::addr_of_mut!(place!(Field::<*const (u32, usize, u32)>(Variant(_8, 2), 1)));
RET = !29_isize;
_5 = [2598653399_u32,514581943_u32,69903371_u32,1984638874_u32,1494316573_u32];
Goto(bb5)
}
bb5 = {
_5 = [1033847858_u32,1527329144_u32,2841906697_u32,718782383_u32,2972036938_u32];
_12 = _10.0;
place!(Field::<usize>(Variant(_8, 2), 0)) = 5_usize & 3123360935706750751_usize;
RET = _17 as isize;
_9 = _14;
place!(Field::<(u64,)>(Variant(_8, 2), 3)).0 = _7 as u64;
_21.fld0.0 = _17;
_18 = _7 * _7;
RET = 79_isize;
_20 = 2252009812_u32 as u8;
_23 = _13 as isize;
_18 = -_7;
_1 = [1314390975_u32,1967092989_u32,1937010819_u32,2577835819_u32,367021487_u32];
_13 = !136983993823944869975549740704444460599_i128;
_21.fld0.0 = !_17;
_25 = true;
_26 = 3349005735308169142_i64 as f64;
match RET {
0 => bb1,
79 => bb6,
_ => bb4
}
}
bb6 = {
place!(Field::<usize>(Variant(_8, 2), 0)) = 9546587498070623713_usize;
_1 = [2256722211_u32,3015919411_u32,3436526216_u32,4002928770_u32,1414349721_u32];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
79 => bb7,
_ => bb5
}
}
bb7 = {
_13 = !(-90013961920431425890828301765915974950_i128);
_21.fld0 = Field::<(u64,)>(Variant(_8, 2), 3);
_26 = Field::<usize>(Variant(_8, 2), 0) as f64;
_20 = 139_u8 | 150_u8;
_9 = _14;
_25 = !true;
_12 = _10.0;
match Field::<usize>(Variant(_8, 2), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9546587498070623713 => bb9,
_ => bb8
}
}
bb8 = {
_7 = Field::<i128>(Variant(_8, 0), 1) as f32;
_7 = 3403312368_u32 as f32;
_2 = _3;
_9 = 26690_u16 >> RET;
_2 = _3;
_5 = [3605288006_u32,2805774624_u32,3118392821_u32,1781092999_u32,1304535435_u32];
RET = 9223372036854775807_isize;
_1 = [3802933748_u32,1169645253_u32,2221317984_u32,1246092169_u32,2316946944_u32];
_2 = [2442967778_u32,2562415802_u32,207933880_u32,3897497385_u32,341040021_u32];
place!(Field::<u64>(Variant(_8, 0), 0)) = (-2049952285_i32) as u64;
_3 = [2388235590_u32,418070776_u32,1328280264_u32,2371610247_u32,1197710864_u32];
_2 = _5;
_1 = _2;
_2 = [1155683672_u32,1541338396_u32,929213044_u32,1788058515_u32,2607492663_u32];
place!(Field::<u64>(Variant(_8, 0), 0)) = '\u{ee807}' as u64;
place!(Field::<u64>(Variant(_8, 0), 0)) = 4639777630815459087_u64 * 6404223852977324699_u64;
RET = (-9223372036854775808_isize) >> Field::<i128>(Variant(_8, 0), 1);
SetDiscriminant(_8, 2);
_12 = '\u{8f30b}';
place!(Field::<usize>(Variant(_8, 2), 0)) = 1_usize - 0_usize;
RET = (-117_i8) as isize;
place!(Field::<u128>(Variant(_8, 2), 2)) = !_4;
place!(Field::<usize>(Variant(_8, 2), 0)) = !16013455802830566816_usize;
_5 = [3430945033_u32,2101612218_u32,2934243404_u32,84488159_u32,1781564254_u32];
_7 = RET as f32;
_12 = '\u{32bb6}';
_7 = 103469527475843078944600603699710779516_i128 as f32;
Goto(bb3)
}
bb9 = {
_22 = _23;
_17 = _14 as u64;
_21.fld1 = _10.0;
_18 = -_7;
_21.fld2 = (_18,);
_19 = (-6653039327975980885_i64) as isize;
place!(Field::<u128>(Variant(_8, 2), 2)) = _4;
Call(_28 = core::intrinsics::transmute(Field::<(u64,)>(Variant(_8, 2), 3).0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21.fld0 = Field::<(u64,)>(Variant(_8, 2), 3);
_28 = [Field::<usize>(Variant(_8, 2), 0)];
_25 = false;
_25 = true;
_27 = _13 as i32;
_10.2 = core::ptr::addr_of_mut!(_21.fld3);
_26 = _21.fld2.0 as f64;
_20 = Field::<i16>(Variant(_8, 2), 4) as u8;
_29.1 = [393843324_u32,1606575952_u32];
_23 = _19;
_5 = [1916804824_u32,249575644_u32,4254322539_u32,110791736_u32,3079119878_u32];
Goto(bb11)
}
bb11 = {
_21.fld4 = _4 as i16;
Goto(bb12)
}
bb12 = {
_11 = &_21.fld5;
_9 = _14;
_8 = Adt30::Variant0 { fld0: _17,fld1: _13 };
SetDiscriminant(_8, 2);
match RET {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb6,
79 => bb13,
_ => bb11
}
}
bb13 = {
_2 = [3860341418_u32,4272881178_u32,4155960215_u32,2565981278_u32,1408452092_u32];
_3 = _6;
_18 = -_21.fld2.0;
_24 = !_20;
_15.2 = Adt30::Variant0 { fld0: _17,fld1: _13 };
_3 = [2683636718_u32,2946162175_u32,625689863_u32,2693359947_u32,3091723223_u32];
SetDiscriminant(_15.2, 2);
_27 = !750380907_i32;
_5 = _6;
_19 = _22;
_9 = _14 >> _22;
_2 = [3310999108_u32,1136672399_u32,2752338548_u32,1653242828_u32,3480222443_u32];
place!(Field::<(u64,)>(Variant(_15.2, 2), 3)) = (_17,);
_15.2 = Adt30::Variant0 { fld0: _17,fld1: _13 };
place!(Field::<usize>(Variant(_8, 2), 0)) = 9103817891477253262_usize * 13685426211970341190_usize;
_21.fld0 = (Field::<u64>(Variant(_15.2, 0), 0),);
Goto(bb14)
}
bb14 = {
_19 = _10.0 as isize;
_29.1 = [565317456_u32,2144116332_u32];
place!(Field::<(u64,)>(Variant(_8, 2), 3)).0 = !_21.fld0.0;
_38 = _18 * _18;
_10.2 = core::ptr::addr_of_mut!(place!(Field::<*const (u32, usize, u32)>(Variant(_8, 2), 1)));
_28 = [Field::<usize>(Variant(_8, 2), 0)];
_31 = [_21.fld4,_21.fld4,_21.fld4,_21.fld4,_21.fld4,_21.fld4];
_14 = _9 ^ _9;
_6 = [1575088129_u32,41562496_u32,2136012974_u32,311135943_u32,4025017455_u32];
_11 = &_21.fld5;
place!(Field::<(u64,)>(Variant(_8, 2), 3)) = (Field::<u64>(Variant(_15.2, 0), 0),);
_21.fld2 = (_38,);
_21.fld5 = _27;
_12 = _10.0;
_38 = -_21.fld2.0;
_13 = -Field::<i128>(Variant(_15.2, 0), 1);
_25 = !false;
_17 = Field::<u64>(Variant(_15.2, 0), 0);
_38 = _18;
place!(Field::<u128>(Variant(_8, 2), 2)) = !_4;
_1 = [1734592384_u32,3860430089_u32,157860890_u32,1446472116_u32,840472224_u32];
_34 = [3793734663424619834_i64,(-4167121847318256014_i64),8090422176445654836_i64];
SetDiscriminant(_15.2, 1);
_28 = [Field::<usize>(Variant(_8, 2), 0)];
_21.fld0 = Field::<(u64,)>(Variant(_8, 2), 3);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(17_usize, 34_usize, Move(_34), 19_usize, Move(_19), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(17_usize, 28_usize, Move(_28), 1_usize, Move(_1), 24_usize, Move(_24), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(17_usize, 27_usize, Move(_27), 14_usize, Move(_14), 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: &'static i16,mut _2: [u32; 5],mut _3: bool,mut _4: bool,mut _5: bool,mut _6: i16,mut _7: [u16; 5]) -> bool {
mir! {
type RET = bool;
let _8: &'static &'static i32;
let _9: ();
let _10: ();
{
_2 = [3090247987_u32,3449708524_u32,3395085189_u32,4176230278_u32,2704999418_u32];
_1 = &_6;
_4 = _3;
_2 = [4202682904_u32,3782221567_u32,3269438422_u32,4239259988_u32,1065136144_u32];
RET = _5 != _4;
_2 = [2603919011_u32,2918410368_u32,3055937230_u32,3608023148_u32,3609402141_u32];
_2 = [2497365021_u32,797293118_u32,1895332065_u32,2549688886_u32,3070622255_u32];
_7 = [60626_u16,1609_u16,51270_u16,64292_u16,63610_u16];
RET = _3;
RET = _4;
RET = _5;
_7 = [55001_u16,3977_u16,42309_u16,35618_u16,15593_u16];
RET = !_5;
_3 = _5 != _4;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(18_usize, 3_usize, Move(_3), 7_usize, Move(_7), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(257585053136512761754705907019495240456_u128), std::hint::black_box(12091024233022818836_u64), std::hint::black_box(33848_u16), std::hint::black_box(141834382272745315606669147194143933321_i128), std::hint::black_box(16452_i16));
                
            }
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: i16,
fld1: isize,

},
Variant1{
fld0: *const (u32, usize, u32),

},
Variant2{
fld0: (u64,),
fld1: [u16; 5],
fld2: [usize; 1],
fld3: i8,
fld4: i16,
fld5: i32,

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){unsafe{printf("Adt29::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt29 {
Variant0{
fld0: Adt26,
fld1: (u32, usize, u32),
fld2: ([usize; 1], (u64,), u8, (u32, usize, u32)),
fld3: i8,
fld4: (u64,),
fld5: i32,
fld6: f64,
fld7: i128,

},
Variant1{
fld0: (char, usize),

},
Variant2{
fld0: i128,
fld1: (char, usize),

}}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){unsafe{printf("Adt30::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt30 {
Variant0{
fld0: u64,
fld1: i128,

},
Variant1{
fld0: u8,
fld1: i128,
fld2: isize,
fld3: (char, usize),
fld4: f32,
fld5: i32,
fld6: [u16; 5],

},
Variant2{
fld0: usize,
fld1: *const (u32, usize, u32),
fld2: u128,
fld3: (u64,),
fld4: i16,

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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: usize,
fld1: char,
fld2: u32,
fld3: [u16; 5],
fld4: (u64,),
fld5: *const (u32, usize, u32),
fld6: i64,

},
Variant1{
fld0: f64,
fld1: (char, usize),
fld2: [usize; 1],
fld3: Adt26,
fld4: i16,
fld5: u16,
fld6: i64,
fld7: u64,

},
Variant2{
fld0: bool,
fld1: (char, usize),
fld2: ([usize; 1], (u64,), u8, (u32, usize, u32)),
fld3: (u32, usize, u32),
fld4: u64,
fld5: u128,
fld6: i64,
fld7: Adt29,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const Adt31,
fld1: Adt30,
fld2: u16,
fld3: (u32, usize, u32),
fld4: f64,
fld5: *const [usize; 1],
fld6: ([usize; 1], (u64,), u8, (u32, usize, u32)),
fld7: ([i32; 6],),

},
Variant1{
fld0: bool,
fld1: ([i32; 6],),
fld2: u128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: (u64,),
fld1: char,
fld2: (f32,),
fld3: *const (u32, usize, u32),
fld4: i16,
fld5: i32,
}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt78{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt78 {
fld0: *const [usize; 1],
fld1: char,
fld2: Adt30,
fld3: (char, usize),
fld4: i16,
fld5: *mut *const (u32, usize, u32),
}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){unsafe{printf("Adt80::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt80 {
Variant0{
fld0: (char, *const Adt31, *mut *const (u32, usize, u32)),
fld1: isize,

},
Variant1{
fld0: f32,
fld1: [i128; 8],

},
Variant2{
fld0: [u16; 7],
fld1: [usize; 8],
fld2: isize,
fld3: [u16; 5],
fld4: i16,
fld5: *const Adt31,
fld6: Adt29,
fld7: [u128; 8],

},
Variant3{
fld0: u8,
fld1: Adt78,
fld2: (char, [i64; 2]),
fld3: (u128, Adt52, (u32, usize, u32)),

}}

