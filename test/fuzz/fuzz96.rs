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
pub fn fn0(mut _1: i32,mut _2: usize,mut _3: u8) -> f32 {
mir! {
type RET = f32;
let _4: f64;
let _5: u8;
let _6: (f64, [i64; 3], (i128, u32, u16, char));
let _7: Adt70;
let _8: Adt35;
let _9: Adt36;
let _10: &'static *mut u64;
let _11: u128;
let _12: isize;
let _13: i8;
let _14: [i16; 4];
let _15: *mut i16;
let _16: [i64; 3];
let _17: i32;
let _18: Adt33;
let _19: f32;
let _20: [u64; 6];
let _21: f32;
let _22: Adt21;
let _23: u64;
let _24: isize;
let _25: [i64; 3];
let _26: f32;
let _27: (i128, u32, u16, char);
let _28: bool;
let _29: &'static [i16; 1];
let _30: ();
let _31: ();
{
_3 = 154_u8;
RET = 7614514576020252452_usize as f32;
_1 = 769239219_i32;
_2 = 5_usize << _1;
_1 = _3 as i32;
RET = 3643962417349461728_u64 as f32;
RET = 12329607959005237879_u64 as f32;
RET = _1 as f32;
_2 = 13357626283168966016_usize;
match _3 {
154 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_4 = 61551171021923995925085570005793280650_u128 as f64;
_4 = _2 as f64;
Call(RET = fn1(_3, _3, _3, _3, _3, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = 172_u8 + 64_u8;
RET = _3 as f32;
_5 = _3;
_1 = RET as i32;
_4 = (-31_i8) as f64;
RET = (-3921181270800111037_i64) as f32;
RET = _1 as f32;
RET = 3115548004_u32 as f32;
_6.2.1 = !2002416637_u32;
_6.1 = [559933114618987370_i64,(-5230402379494484566_i64),1507324156018948221_i64];
_8 = Adt35::Variant3 { fld0: _1,fld1: _3 };
_6.0 = _5 as f64;
match _2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
13357626283168966016 => bb8,
_ => bb7
}
}
bb4 = {
_4 = 61551171021923995925085570005793280650_u128 as f64;
_4 = _2 as f64;
Call(RET = fn1(_3, _3, _3, _3, _3, _3), ReturnTo(bb3), UnwindUnreachable())
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
_6.2.3 = '\u{8f89d}';
_1 = -Field::<i32>(Variant(_8, 3), 0);
_2 = 4385_u16 as usize;
_6.2.2 = 57518_u16;
_8 = Adt35::Variant3 { fld0: _1,fld1: _5 };
RET = (-7019358607539193571_i64) as f32;
_1 = 13941463065060192582_u64 as i32;
_1 = -Field::<i32>(Variant(_8, 3), 0);
Goto(bb9)
}
bb9 = {
_6.2.2 = _6.2.1 as u16;
SetDiscriminant(_8, 0);
_13 = 30_i8;
place!(Field::<isize>(Variant(_8, 0), 2)) = !66_isize;
_8 = Adt35::Variant1 { fld0: _6.2.1,fld1: _6.2.3 };
RET = 1_isize as f32;
_6.2.0 = !108434623007046193939099138868281946931_i128;
_2 = _13 as usize;
_6.2.1 = Field::<u32>(Variant(_8, 1), 0);
_14 = [(-20589_i16),(-15358_i16),(-30138_i16),13352_i16];
RET = _5 as f32;
_12 = (-9223372036854775808_isize) >> _13;
_6.2.2 = 61649_u16 + 39054_u16;
_14 = [(-27264_i16),30905_i16,(-12678_i16),32159_i16];
_11 = 318899620251907380280166623432968675375_u128 ^ 248622033194750855645598447295562523603_u128;
_14 = [(-23832_i16),(-10154_i16),17124_i16,8422_i16];
Goto(bb10)
}
bb10 = {
_1 = 2076056867_i32;
_6.2 = ((-114144186194362228660679595714144972218_i128), Field::<u32>(Variant(_8, 1), 0), 47828_u16, Field::<char>(Variant(_8, 1), 1));
_4 = _6.0 - _6.0;
_12 = RET as isize;
_3 = !_5;
_8 = Adt35::Variant3 { fld0: _1,fld1: _5 };
RET = 3118520259600864755_u64 as f32;
place!(Field::<i32>(Variant(_8, 3), 0)) = -_1;
_11 = !146081610709788856215807731647302095133_u128;
_6.2.1 = 3663337282_u32 + 1080434926_u32;
_9.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_6.2.0 = (-78383642738600967518148105278667294882_i128);
_16 = _6.1;
_6.1 = [(-6571663418848078262_i64),(-4652837551505530055_i64),1060469996948770268_i64];
_6.1 = [(-12239463523531889_i64),7243232106276066305_i64,1416793244872932346_i64];
_6.1 = [4231036835581786122_i64,(-5885220095003754266_i64),(-7829301377106537753_i64)];
_6.2.1 = !3202886761_u32;
RET = _6.2.1 as f32;
_4 = _6.0;
_6.2 = (64994501705698629359640894965389576497_i128, 2128612403_u32, 2127_u16, '\u{8ac17}');
RET = 31110_i16 as f32;
RET = 13019895746471117594_u64 as f32;
_13 = 105_i8;
Goto(bb11)
}
bb11 = {
_6.0 = _4;
_18.fld2 = 16647340390921008654_u64 as u16;
SetDiscriminant(_8, 2);
_6.2.3 = '\u{6e81b}';
_11 = 303666645000281829981372676154785089965_u128;
place!(Field::<f64>(Variant(_8, 2), 2)) = _4;
_9.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_6.2.1 = !358502236_u32;
_18.fld6 = [31399_i16,12359_i16,(-2674_i16),(-23098_i16)];
_19 = RET * RET;
_6.1 = _16;
_18.fld1 = [false,true,true,true,true,true,true];
_9.fld1 = [_12,_12,_12,_12,_12,_12,_12];
place!(Field::<i64>(Variant(_8, 2), 1)) = (-5481801467030975163_i64);
RET = _19 + _19;
_21 = _19;
_22 = Adt21::Variant1 { fld0: false,fld1: _2,fld2: _6.2.1,fld3: _6.2.0,fld4: _18.fld1 };
place!(Field::<u32>(Variant(_22, 1), 2)) = !_6.2.1;
_11 = !129102201214467777470647701987233834227_u128;
_21 = -_19;
_18.fld0 = !false;
_18.fld4.0 = _2 + _2;
match _6.2.2 {
0 => bb7,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
2127 => bb18,
_ => bb17
}
}
bb12 = {
_3 = 172_u8 + 64_u8;
RET = _3 as f32;
_5 = _3;
_1 = RET as i32;
_4 = (-31_i8) as f64;
RET = (-3921181270800111037_i64) as f32;
RET = _1 as f32;
RET = 3115548004_u32 as f32;
_6.2.1 = !2002416637_u32;
_6.1 = [559933114618987370_i64,(-5230402379494484566_i64),1507324156018948221_i64];
_8 = Adt35::Variant3 { fld0: _1,fld1: _3 };
_6.0 = _5 as f64;
match _2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
13357626283168966016 => bb8,
_ => bb7
}
}
bb13 = {
Return()
}
bb14 = {
_6.2.3 = '\u{8f89d}';
_1 = -Field::<i32>(Variant(_8, 3), 0);
_2 = 4385_u16 as usize;
_6.2.2 = 57518_u16;
_8 = Adt35::Variant3 { fld0: _1,fld1: _5 };
RET = (-7019358607539193571_i64) as f32;
_1 = 13941463065060192582_u64 as i32;
_1 = -Field::<i32>(Variant(_8, 3), 0);
Goto(bb9)
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
_22 = Adt21::Variant0 { fld0: _18.fld4.0,fld1: _18.fld1,fld2: _3,fld3: _13,fld4: _11,fld5: _6.0,fld6: Field::<i64>(Variant(_8, 2), 1) };
_2 = _18.fld4.0;
_25 = [Field::<i64>(Variant(_22, 0), 6),Field::<i64>(Variant(_8, 2), 1),Field::<i64>(Variant(_8, 2), 1)];
_17 = _1;
_18.fld4 = (_2, Move(_22));
_8 = Adt35::Variant2 { fld0: 15523179581683139027_u64,fld1: Field::<i64>(Variant(_18.fld4.1, 0), 6),fld2: Field::<f64>(Variant(_18.fld4.1, 0), 5) };
_25 = [Field::<i64>(Variant(_8, 2), 1),Field::<i64>(Variant(_18.fld4.1, 0), 6),Field::<i64>(Variant(_18.fld4.1, 0), 6)];
_23 = 8569954106675950569_u64;
place!(Field::<f64>(Variant(_8, 2), 2)) = _4;
place!(Field::<u64>(Variant(_8, 2), 0)) = !_23;
_9.fld1 = [_12,_12,_12,_12,_12,_12,_12];
SetDiscriminant(_18.fld4.1, 0);
place!(Field::<u128>(Variant(_18.fld4.1, 0), 4)) = _6.2.1 as u128;
place!(Field::<f64>(Variant(_8, 2), 2)) = _4;
_3 = !_5;
place!(Field::<usize>(Variant(_18.fld4.1, 0), 0)) = _18.fld4.0 & _18.fld4.0;
_27 = _6.2;
_24 = _12 - _12;
place!(Field::<f64>(Variant(_8, 2), 2)) = _13 as f64;
_1 = -_17;
_28 = _18.fld0 & _18.fld0;
_5 = Field::<usize>(Variant(_18.fld4.1, 0), 0) as u8;
place!(Field::<f64>(Variant(_8, 2), 2)) = _27.0 as f64;
_18.fld1 = [_18.fld0,_28,_28,_18.fld0,_18.fld0,_18.fld0,_28];
Goto(bb19)
}
bb19 = {
Call(_30 = dump_var(0_usize, 24_usize, Move(_24), 5_usize, Move(_5), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(0_usize, 2_usize, Move(_2), 11_usize, Move(_11), 3_usize, Move(_3), 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u8,mut _2: u8,mut _3: u8,mut _4: u8,mut _5: u8,mut _6: u8) -> f32 {
mir! {
type RET = f32;
let _7: bool;
let _8: i64;
let _9: Adt33;
let _10: f32;
let _11: isize;
let _12: i64;
let _13: [i128; 6];
let _14: i16;
let _15: &'static f32;
let _16: bool;
let _17: [i16; 4];
let _18: bool;
let _19: (&'static *mut u64,);
let _20: isize;
let _21: (usize, Adt21);
let _22: ();
let _23: ();
{
_1 = _2 | _3;
RET = (-902032434_i32) as f32;
_6 = !_1;
Call(_2 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_4;
RET = (-9223372036854775808_isize) as f32;
RET = (-9223372036854775808_isize) as f32;
_4 = RET as u8;
_4 = _6 & _1;
_2 = '\u{d4ad7}' as u8;
_3 = _4;
_7 = !false;
_1 = 10696008173772845700_u64 as u8;
_5 = _3 * _3;
_6 = _3 | _4;
_9.fld4.0 = 1419732809837631548_usize;
_7 = false;
_9.fld2 = !65015_u16;
_9.fld2 = _9.fld4.0 as u16;
_9.fld1 = [_7,_7,_7,_7,_7,_7,_7];
RET = 87_i8 as f32;
_9.fld3 = -26_i8;
_10 = RET;
match _9.fld4.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1419732809837631548 => bb10,
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
_10 = RET - RET;
_7 = !true;
RET = (-9223372036854775808_isize) as f32;
_9.fld6 = [2156_i16,18399_i16,16448_i16,(-27791_i16)];
RET = (-163289760701484407859471053355436664819_i128) as f32;
_11 = 3671591473_u32 as isize;
_1 = _9.fld4.0 as u8;
_1 = _5;
_9.fld4.1 = Adt21::Variant1 { fld0: _7,fld1: _9.fld4.0,fld2: 1233402457_u32,fld3: 37027590676051743133436005920004559082_i128,fld4: _9.fld1 };
_9.fld2 = !37060_u16;
place!(Field::<bool>(Variant(_9.fld4.1, 1), 0)) = _7 | _7;
Call(_8 = fn2(_11, _3, _9.fld6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _8 as u8;
_9.fld1 = Field::<[bool; 7]>(Variant(_9.fld4.1, 1), 4);
_1 = !_6;
_4 = _5 & _6;
_6 = !_4;
_9.fld2 = Field::<usize>(Variant(_9.fld4.1, 1), 1) as u16;
Goto(bb12)
}
bb12 = {
_9.fld4.1 = Adt21::Variant1 { fld0: _7,fld1: _9.fld4.0,fld2: 165983800_u32,fld3: (-47634393506833128151705652389544482167_i128),fld4: _9.fld1 };
_9.fld0 = _7 | Field::<bool>(Variant(_9.fld4.1, 1), 0);
_9.fld5 = !7619359153741783956_u64;
place!(Field::<i128>(Variant(_9.fld4.1, 1), 3)) = (-139289976969179236795717891828851730139_i128);
_8 = 6326543597591708308_i64 & (-4206121868512913144_i64);
place!(Field::<u32>(Variant(_9.fld4.1, 1), 2)) = Field::<usize>(Variant(_9.fld4.1, 1), 1) as u32;
_9.fld2 = 7400_u16 ^ 57352_u16;
_12 = _8 | _8;
_9.fld1 = [_7,_7,Field::<bool>(Variant(_9.fld4.1, 1), 0),Field::<bool>(Variant(_9.fld4.1, 1), 0),Field::<bool>(Variant(_9.fld4.1, 1), 0),Field::<bool>(Variant(_9.fld4.1, 1), 0),_7];
_10 = RET + RET;
_1 = !_4;
_9.fld3 = -116_i8;
SetDiscriminant(_9.fld4.1, 0);
place!(Field::<u8>(Variant(_9.fld4.1, 0), 2)) = !_4;
_11 = (-9223372036854775808_isize);
place!(Field::<i8>(Variant(_9.fld4.1, 0), 3)) = _9.fld3 & _9.fld3;
_5 = _1;
place!(Field::<usize>(Variant(_9.fld4.1, 0), 0)) = !_9.fld4.0;
_10 = _12 as f32;
_5 = Field::<u8>(Variant(_9.fld4.1, 0), 2) * Field::<u8>(Variant(_9.fld4.1, 0), 2);
_13 = [1863209832720216269177052965238001074_i128,114941014459785553626075500851028505555_i128,132701664980664786690549251464718703273_i128,115476616447637966584901003973601176854_i128,14779859839364312618544988422878411784_i128,111814989522225775210804141793704994225_i128];
_8 = _12;
place!(Field::<i8>(Variant(_9.fld4.1, 0), 3)) = _9.fld3 << Field::<u8>(Variant(_9.fld4.1, 0), 2);
place!(Field::<u8>(Variant(_9.fld4.1, 0), 2)) = _8 as u8;
_3 = !_5;
RET = _10 * _10;
place!(Field::<i64>(Variant(_9.fld4.1, 0), 6)) = _8 + _8;
match _9.fld4.0 {
0 => bb1,
1419732809837631548 => bb13,
_ => bb8
}
}
bb13 = {
_2 = _5 + _1;
_9.fld1 = [_7,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0];
_13 = [(-45024503721366102023787562928624824848_i128),(-148220987205861279072504150876893758891_i128),142065479880900565198030212470047099443_i128,6665320592452357133695641813439442064_i128,145346342217672111949577247357184216240_i128,150219411900044308526327620407750569329_i128];
place!(Field::<u128>(Variant(_9.fld4.1, 0), 4)) = 269120100350487255501597341446384158335_u128 + 116057814426151475188808353671843722244_u128;
place!(Field::<i64>(Variant(_9.fld4.1, 0), 6)) = (-11620_i16) as i64;
RET = _5 as f32;
_5 = _2;
_9.fld5 = 8701677811843895280_u64 + 13342308486055860265_u64;
_9.fld2 = 16679_u16 | 49420_u16;
RET = -_10;
_4 = !_5;
_5 = !_4;
_9.fld2 = 43665_u16;
_2 = !_4;
_3 = _1 * _6;
place!(Field::<usize>(Variant(_9.fld4.1, 0), 0)) = _9.fld4.0;
_7 = _9.fld0;
_9.fld3 = -Field::<i8>(Variant(_9.fld4.1, 0), 3);
_9.fld5 = 10657236691395226924_u64 * 5652870275641369093_u64;
_9.fld1 = [_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_7];
_9.fld0 = !_7;
_14 = 9515_i16 + (-9127_i16);
Goto(bb14)
}
bb14 = {
_13 = [(-105758056070416083707244717022986728534_i128),(-76629753060380813426912393715479231892_i128),(-154542859274969958022487844603565617666_i128),(-61832035583661391799997942870552684435_i128),(-122935593268516773060680141932453160116_i128),(-141600688712642215697356968057401716925_i128)];
_4 = _2;
place!(Field::<[bool; 7]>(Variant(_9.fld4.1, 0), 1)) = [_9.fld0,_9.fld0,_9.fld0,_7,_7,_9.fld0,_9.fld0];
_9.fld4.0 = !Field::<usize>(Variant(_9.fld4.1, 0), 0);
RET = _10;
_9.fld0 = RET == _10;
_17 = [_14,_14,_14,_14];
_14 = 31965_i16;
_10 = -RET;
_6 = _11 as u8;
_16 = _9.fld0;
_7 = _8 < _12;
_9.fld0 = !_16;
place!(Field::<f64>(Variant(_9.fld4.1, 0), 5)) = _9.fld4.0 as f64;
_15 = &RET;
_7 = !_16;
place!(Field::<u128>(Variant(_9.fld4.1, 0), 4)) = _12 as u128;
_7 = !_9.fld0;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(1_usize, 1_usize, Move(_1), 11_usize, Move(_11), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(1_usize, 4_usize, Move(_4), 16_usize, Move(_16), 6_usize, Move(_6), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: u8,mut _3: [i16; 4]) -> i64 {
mir! {
type RET = i64;
let _4: f32;
let _5: &'static &'static Adt21;
let _6: Adt33;
let _7: f32;
let _8: isize;
let _9: f64;
let _10: char;
let _11: char;
let _12: i16;
let _13: isize;
let _14: Adt40;
let _15: ((usize, Adt21),);
let _16: u8;
let _17: &'static *const &'static *mut usize;
let _18: u128;
let _19: (usize, *const &'static f32);
let _20: (u8, u64);
let _21: &'static &'static [isize; 7];
let _22: (Adt70, (usize, Adt21));
let _23: isize;
let _24: f64;
let _25: Adt21;
let _26: ();
let _27: ();
{
_1 = -112_isize;
RET = (-8563620698878323597_i64);
RET = (-2656327188449054623_i64) * (-5525323460728920550_i64);
_2 = 11633_u16 as u8;
_2 = !152_u8;
RET = -(-4423099656970362845_i64);
RET = 18385782920104537636_u64 as i64;
_4 = 63807_u16 as f32;
RET = false as i64;
RET = (-46146083253507345961871637741323357583_i128) as i64;
_1 = 11340802160560937123_usize as isize;
RET = -(-8216800573509977437_i64);
_6.fld5 = 14632118842311760143_u64;
Goto(bb1)
}
bb1 = {
_6.fld4.0 = !374100079715441132_usize;
_6.fld0 = !false;
_6.fld3 = 41_i8 - 125_i8;
RET = -4030739433352158929_i64;
_6.fld4.1 = Adt21::Variant2 { fld0: _6.fld0,fld1: '\u{39cbc}',fld2: 1652737940_u32,fld3: _4,fld4: 52633_u16,fld5: 1197749119_i32 };
_6.fld3 = (-67_i8);
place!(Field::<bool>(Variant(_6.fld4.1, 2), 0)) = _6.fld0;
_8 = _1;
_13 = _1 >> _6.fld3;
place!(Field::<u32>(Variant(_6.fld4.1, 2), 2)) = 3368568195_u32 << _13;
_14.fld2 = (-30024_i16) as f64;
_6.fld1 = [Field::<bool>(Variant(_6.fld4.1, 2), 0),_6.fld0,Field::<bool>(Variant(_6.fld4.1, 2), 0),Field::<bool>(Variant(_6.fld4.1, 2), 0),_6.fld0,_6.fld0,_6.fld0];
_9 = _14.fld2 - _14.fld2;
place!(Field::<u16>(Variant(_6.fld4.1, 2), 4)) = 20514_u16;
RET = (-2676255896649987174_i64);
_14.fld0 = !Field::<bool>(Variant(_6.fld4.1, 2), 0);
Call(place!(Field::<char>(Variant(_6.fld4.1, 2), 1)) = fn3(_6.fld4.0, _13, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-5055156232196037089_i64);
RET = 1501953621844155500_i64 & (-2297284196867501774_i64);
_12 = !3917_i16;
place!(Field::<u32>(Variant(_6.fld4.1, 2), 2)) = Field::<u16>(Variant(_6.fld4.1, 2), 4) as u32;
place!(Field::<f32>(Variant(_6.fld4.1, 2), 3)) = RET as f32;
place!(Field::<u32>(Variant(_6.fld4.1, 2), 2)) = !1219347094_u32;
match _6.fld3 {
340282366920938463463374607431768211389 => bb3,
_ => bb1
}
}
bb3 = {
_15.0.0 = _6.fld4.0;
_10 = Field::<char>(Variant(_6.fld4.1, 2), 1);
_8 = _1;
place!(Field::<i32>(Variant(_6.fld4.1, 2), 5)) = 1396415661_i32 << _1;
_11 = _10;
RET = (-6833144552167990475_i64) | (-3800460968735185085_i64);
place!(Field::<f32>(Variant(_6.fld4.1, 2), 3)) = _9 as f32;
_10 = Field::<char>(Variant(_6.fld4.1, 2), 1);
_7 = _4;
_12 = !(-14620_i16);
_16 = RET as u8;
_3 = [_12,_12,_12,_12];
_1 = _7 as isize;
place!(Field::<char>(Variant(_6.fld4.1, 2), 1)) = _11;
SetDiscriminant(_6.fld4.1, 1);
place!(Field::<bool>(Variant(_6.fld4.1, 1), 0)) = !_14.fld0;
_10 = _11;
_15.0.0 = !_6.fld4.0;
_14 = Adt40 { fld0: _6.fld0,fld1: _4,fld2: _9 };
_14.fld1 = RET as f32;
_6.fld3 = (-73_i8);
place!(Field::<u32>(Variant(_6.fld4.1, 1), 2)) = 210488515307194606380895239902241865555_u128 as u32;
RET = _12 as i64;
place!(Field::<[bool; 7]>(Variant(_6.fld4.1, 1), 4)) = _6.fld1;
Goto(bb4)
}
bb4 = {
_15.0.1 = Adt21::Variant2 { fld0: _6.fld0,fld1: _10,fld2: Field::<u32>(Variant(_6.fld4.1, 1), 2),fld3: _7,fld4: 5102_u16,fld5: (-554986699_i32) };
place!(Field::<u32>(Variant(_6.fld4.1, 1), 2)) = Field::<u32>(Variant(_15.0.1, 2), 2) | Field::<u32>(Variant(_15.0.1, 2), 2);
_6.fld2 = _12 as u16;
place!(Field::<i128>(Variant(_6.fld4.1, 1), 3)) = 105147881138300595524403484224074195862_i128;
place!(Field::<bool>(Variant(_6.fld4.1, 1), 0)) = _6.fld0 & Field::<bool>(Variant(_15.0.1, 2), 0);
_1 = -_13;
_10 = _11;
_13 = 315649836648080438412825013858539448813_u128 as isize;
_15.0.1 = Adt21::Variant2 { fld0: Field::<bool>(Variant(_6.fld4.1, 1), 0),fld1: _10,fld2: Field::<u32>(Variant(_6.fld4.1, 1), 2),fld3: _14.fld1,fld4: _6.fld2,fld5: 361708416_i32 };
_6.fld2 = Field::<i128>(Variant(_6.fld4.1, 1), 3) as u16;
_15.0.1 = Adt21::Variant1 { fld0: _14.fld0,fld1: _6.fld4.0,fld2: Field::<u32>(Variant(_6.fld4.1, 1), 2),fld3: Field::<i128>(Variant(_6.fld4.1, 1), 3),fld4: Field::<[bool; 7]>(Variant(_6.fld4.1, 1), 4) };
_6 = Adt33 { fld0: _14.fld0,fld1: Field::<[bool; 7]>(Variant(_15.0.1, 1), 4),fld2: 23761_u16,fld3: 88_i8,fld4: Move(_15.0),fld5: 2694779329419205718_u64,fld6: _3 };
_13 = _8 >> _6.fld2;
_6.fld0 = _6.fld2 >= _6.fld2;
_6.fld4.0 = _14.fld2 as usize;
_15 = (Move(_6.fld4),);
Goto(bb5)
}
bb5 = {
_6.fld4 = Move(_15.0);
_14.fld0 = _6.fld0 > _6.fld0;
_15.0.1 = Adt21::Variant2 { fld0: _6.fld0,fld1: _10,fld2: Field::<u32>(Variant(_6.fld4.1, 1), 2),fld3: _4,fld4: _6.fld2,fld5: 121738774_i32 };
place!(Field::<u32>(Variant(_15.0.1, 2), 2)) = Field::<u32>(Variant(_6.fld4.1, 1), 2) - Field::<u32>(Variant(_6.fld4.1, 1), 2);
place!(Field::<i128>(Variant(_6.fld4.1, 1), 3)) = (-16293642351704900195766624397859732069_i128);
place!(Field::<u16>(Variant(_15.0.1, 2), 4)) = _6.fld2;
_18 = _2 as u128;
SetDiscriminant(_6.fld4.1, 1);
place!(Field::<bool>(Variant(_6.fld4.1, 1), 0)) = !_14.fld0;
_14 = Adt40 { fld0: Field::<bool>(Variant(_6.fld4.1, 1), 0),fld1: _7,fld2: _9 };
_14 = Adt40 { fld0: Field::<bool>(Variant(_6.fld4.1, 1), 0),fld1: Field::<f32>(Variant(_15.0.1, 2), 3),fld2: _9 };
_6.fld2 = Field::<u16>(Variant(_15.0.1, 2), 4) % Field::<u16>(Variant(_15.0.1, 2), 4);
place!(Field::<u32>(Variant(_6.fld4.1, 1), 2)) = Field::<u32>(Variant(_15.0.1, 2), 2) >> _13;
place!(Field::<bool>(Variant(_6.fld4.1, 1), 0)) = _6.fld5 < _6.fld5;
RET = (-152670833976030551_i64) << _6.fld3;
place!(Field::<i128>(Variant(_6.fld4.1, 1), 3)) = (-139048113108149791020636521257992476605_i128);
Goto(bb6)
}
bb6 = {
Call(_26 = dump_var(2_usize, 13_usize, Move(_13), 12_usize, Move(_12), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_26 = dump_var(2_usize, 18_usize, Move(_18), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: usize,mut _2: isize,mut _3: i64) -> char {
mir! {
type RET = char;
let _4: f32;
let _5: isize;
let _6: Adt67;
let _7: Adt77;
let _8: isize;
let _9: [isize; 7];
let _10: *const u32;
let _11: isize;
let _12: *mut u64;
let _13: &'static [i16; 1];
let _14: f32;
let _15: isize;
let _16: &'static f32;
let _17: isize;
let _18: isize;
let _19: (Adt40, *mut u64);
let _20: bool;
let _21: bool;
let _22: (usize, Adt21);
let _23: f32;
let _24: Adt35;
let _25: (f32,);
let _26: i32;
let _27: i32;
let _28: *const &'static *mut usize;
let _29: *mut i16;
let _30: bool;
let _31: &'static &'static [isize; 7];
let _32: [i16; 4];
let _33: *mut i16;
let _34: *mut i8;
let _35: bool;
let _36: [u32; 2];
let _37: [u128; 1];
let _38: bool;
let _39: [u8; 6];
let _40: isize;
let _41: i32;
let _42: f64;
let _43: ();
let _44: ();
{
RET = '\u{279b5}';
_3 = _1 as i64;
_1 = 4224066606548961343_usize;
_1 = !16519487822682855163_usize;
_4 = 64014297059906641451991671973012827666_i128 as f32;
_1 = !7202271068852950045_usize;
Goto(bb1)
}
bb1 = {
RET = '\u{1cf4b}';
_3 = (-7266174151670631717_i64);
_4 = _1 as f32;
_1 = 3866648228131408375_usize * 3_usize;
_4 = 8021254299633998664_u64 as f32;
_2 = !9223372036854775807_isize;
RET = '\u{4041f}';
RET = '\u{9ef35}';
_1 = 4_usize * 13209790828281967422_usize;
RET = '\u{e55c5}';
_4 = (-382231225_i32) as f32;
_5 = !_2;
_1 = 1_usize;
RET = '\u{10b635}';
_4 = 0_i8 as f32;
_1 = !0_usize;
_3 = true as i64;
_2 = -_5;
_3 = !7073256717365865584_i64;
_4 = 6_u8 as f32;
_1 = 4_usize;
_4 = (-32380_i16) as f32;
_2 = -_5;
_5 = _2;
Call(_7 = fn4(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = -_5;
_3 = (-7034458333205751670_i64) + 219068610688131816_i64;
_1 = _3 as usize;
RET = '\u{10eec}';
RET = '\u{497bd}';
_5 = _2 * _2;
_1 = !326873336463379579_usize;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [87927035098061742052463570452457607303_i128,(-63121629782014092735741187384643027220_i128),55263227430256382900588683246989906178_i128,133326837470270526824480621348882437468_i128,110315311028872171654946536439450841854_i128,(-76058440194603579924498489527708313924_i128)];
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [38141698330177921299986214060903325786_i128,(-11223829698159036877073093131321114715_i128),136799163342887957486103369392690363211_i128,(-73488805116261556512960489898687372185_i128),(-151201647419506013378720631139107682643_i128),125199159877933824788729041961955607074_i128];
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [5650893953082825247397916254498989166_i128,(-4391294462686374129678682187987236340_i128),16204172509415095007519352154105107601_i128,(-51215232601206309992697582839412652141_i128),47903925780469466217106802515957820235_i128,(-27067347021955905028424171349182637311_i128)];
_4 = 210_u8 as f32;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [166038029396001399712339678823673314352_i128,(-144202926239888410468550384508409797445_i128),157222859313319673788291801325457015393_i128,72051001057790954965572403444306856837_i128,140394091184108283391482532457384299136_i128,56037010999612725258056778607702197015_i128];
_1 = 6706106641185613594_usize ^ 0_usize;
_2 = _5 ^ _5;
_8 = _5;
_5 = 79_u8 as isize;
RET = '\u{8ba16}';
_8 = _5 ^ _2;
_4 = (-19948525614836241553624588306027215484_i128) as f32;
_5 = _2 ^ _2;
_1 = 15100707055237826547_usize + 16091342110759603732_usize;
_11 = -_8;
_5 = _3 as isize;
RET = '\u{fde51}';
RET = '\u{ea59f}';
_3 = _8 as i64;
_1 = !6_usize;
Goto(bb3)
}
bb3 = {
_2 = -_8;
_2 = true as isize;
_5 = !_11;
_5 = _8 ^ _8;
SetDiscriminant(_7, 0);
_1 = 0_usize;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0))[_1] = (-48551266827565459696696314900596628327_i128) * 22438710590112626419602828770125003338_i128;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [(-128754589982759996588674409691997638643_i128),(-126618703771401428650592084415578023498_i128),(-107642504115638608113934628695399984609_i128),(-107970848299675040594172150345181865431_i128),110390791026546857559841207210814010750_i128,(-143153526816715229466222035999530561924_i128)];
_9 = [_8,_5,_5,_5,_5,_11,_5];
_8 = -_5;
_15 = Field::<[i128; 6]>(Variant(_7, 0), 0)[_1] as isize;
RET = '\u{101c75}';
_14 = (-2073632070_i32) as f32;
_3 = 5606888899300790189_i64 & (-5181106742883180860_i64);
RET = '\u{f9d66}';
RET = '\u{2125b}';
Goto(bb4)
}
bb4 = {
_1 = 17340844648818536868_usize | 4_usize;
_14 = -_4;
_17 = _5;
RET = '\u{b85f8}';
_1 = 160759723401033653824940717936637546066_u128 as usize;
_14 = -_4;
RET = '\u{a9fb9}';
_4 = _14;
_9 = [_8,_15,_8,_17,_5,_11,_5];
_16 = &_4;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [(-59167365595344655989914872043105970318_i128),(-103189746543571878168055392066718856981_i128),114927304791585521599517924886346758660_i128,146973522801823416582639857719977656280_i128,26554102631539443978244123967925052112_i128,75568128025559642522322988368540569869_i128];
_3 = -(-659438276763758718_i64);
_18 = (-143895301040605940135965695521648097546_i128) as isize;
_16 = &_4;
_19.0.fld0 = true;
_5 = -_8;
_19.0.fld2 = 22385498451174608532918952397491640191_i128 as f64;
_5 = _3 as isize;
_19.0.fld0 = !true;
_19.0.fld1 = 2023184029_i32 as f32;
Goto(bb5)
}
bb5 = {
_18 = _8 | _8;
_14 = _4;
_16 = &(*_16);
_19.0.fld2 = _4 as f64;
_1 = !0_usize;
_1 = 1712208297_u32 as usize;
_19.0.fld0 = true;
_9 = [_18,_17,_15,_18,_18,_17,_17];
_6 = Adt67::Variant0 { fld0: _19.0,fld1: 11601_i16,fld2: _19.0.fld2 };
_15 = _18;
_20 = !_19.0.fld0;
_23 = -Field::<Adt40>(Variant(_6, 0), 0).fld1;
_1 = 34_u8 as usize;
_4 = Field::<f64>(Variant(_6, 0), 2) as f32;
_22.0 = !_1;
_18 = _15 + _17;
_9 = [_5,_17,_18,_11,_18,_18,_15];
RET = '\u{db34c}';
Goto(bb6)
}
bb6 = {
_18 = 137254416978001739082376413132686366730_i128 as isize;
place!(Field::<Adt40>(Variant(_6, 0), 0)) = Adt40 { fld0: _20,fld1: _14,fld2: Field::<f64>(Variant(_6, 0), 2) };
_19.0.fld1 = 1499973083_u32 as f32;
_22.0 = _1;
RET = '\u{64d32}';
place!(Field::<Adt40>(Variant(_6, 0), 0)).fld1 = _19.0.fld1;
_8 = !_15;
_1 = !_22.0;
place!(Field::<Adt40>(Variant(_6, 0), 0)).fld0 = !_19.0.fld0;
_24 = Adt35::Variant3 { fld0: (-1722351490_i32),fld1: 117_u8 };
_25.0 = _14 * _4;
_24 = Adt35::Variant0 { fld0: Field::<[i128; 6]>(Variant(_7, 0), 0),fld1: _1,fld2: _8 };
_25.0 = _23 * _23;
_9 = [_11,_15,_15,Field::<isize>(Variant(_24, 0), 2),_15,_8,_15];
_16 = &_14;
_26 = (-168621587_i32);
_21 = !_19.0.fld0;
SetDiscriminant(_24, 1);
_19.0.fld2 = Field::<Adt40>(Variant(_6, 0), 0).fld2;
_19.0 = Adt40 { fld0: _21,fld1: _25.0,fld2: Field::<Adt40>(Variant(_6, 0), 0).fld2 };
Goto(bb7)
}
bb7 = {
_19.0.fld0 = Field::<Adt40>(Variant(_6, 0), 0).fld0 | _21;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [12879794624417063512866265863626453460_i128,(-147973927701568296915987913028670534659_i128),32976462412505628971908562603402122859_i128,48517857520031159531194841382358805885_i128,(-147683875488457311940910658411174500503_i128),162833066864205685615975731162990339732_i128];
place!(Field::<u32>(Variant(_24, 1), 0)) = (*_16) as u32;
_6 = Adt67::Variant1 { fld0: _9,fld1: Field::<u32>(Variant(_24, 1), 0) };
Goto(bb8)
}
bb8 = {
_3 = !(-5721942032232676801_i64);
_14 = -_25.0;
_27 = _20 as i32;
_24 = Adt35::Variant1 { fld0: Field::<u32>(Variant(_6, 1), 1),fld1: RET };
place!(Field::<u32>(Variant(_6, 1), 1)) = _19.0.fld1 as u32;
_3 = 18136785442228268221_u64 as i64;
_8 = !_17;
place!(Field::<[isize; 7]>(Variant(_6, 1), 0)) = [_11,_8,_17,_8,_15,_18,_15];
_3 = -(-3632824467673454469_i64);
_2 = 58147_u16 as isize;
_24 = Adt35::Variant2 { fld0: 9947373041836771334_u64,fld1: _3,fld2: _19.0.fld2 };
_4 = -_14;
place!(Field::<*mut u64>(Variant(_7, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<u64>(Variant(_24, 2), 0)));
_27 = _26;
_19.0.fld2 = Field::<f64>(Variant(_24, 2), 2) - Field::<f64>(Variant(_24, 2), 2);
_19.0.fld1 = _4 - _25.0;
place!(Field::<u32>(Variant(_6, 1), 1)) = _3 as u32;
_25 = (_23,);
RET = '\u{67189}';
_10 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_6, 1), 1)));
RET = '\u{962b4}';
_32 = [(-23678_i16),(-8524_i16),10778_i16,(-21099_i16)];
SetDiscriminant(_7, 1);
Goto(bb9)
}
bb9 = {
_16 = &_25.0;
_22.0 = !_1;
_25.0 = _4 * _4;
_27 = -_26;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).0 = Field::<f64>(Variant(_24, 2), 2);
_16 = &_25.0;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2 = ((-168256108296762749735543137913480954079_i128), Field::<u32>(Variant(_6, 1), 1), 45062_u16, RET);
(*_10) = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).2.1 + Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).2.1;
_27 = -_26;
place!(Field::<u16>(Variant(_7, 1), 0)) = 313939545538100807628839515799827178499_u128 as u16;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).0 = Field::<f64>(Variant(_24, 2), 2);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).1 = [_3,_3,_3];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).1 = [_3,Field::<i64>(Variant(_24, 2), 1),Field::<i64>(Variant(_24, 2), 1)];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
place!(Field::<u64>(Variant(_24, 2), 0)) = 15175571509531377209_u64 + 4686120420184364913_u64;
_25 = (_23,);
_24 = Adt35::Variant2 { fld0: 2569197417244413032_u64,fld1: _3,fld2: _19.0.fld2 };
_9 = [_5,_8,_11,_17,_15,_15,_18];
place!(Field::<u64>(Variant(_24, 2), 0)) = 11688194702637284681_u64 * 8062706813615113243_u64;
_19.0.fld2 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).0;
_5 = -_8;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
_20 = _21;
_10 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_6, 1), 1)));
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.1 = (*_10) << _17;
Goto(bb10)
}
bb10 = {
_16 = &_25.0;
_10 = core::ptr::addr_of!((*_10));
_22.0 = _1;
_24 = Adt35::Variant3 { fld0: _26,fld1: 169_u8 };
_35 = !_19.0.fld0;
_21 = _20 & _35;
match Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).2.0 {
0 => bb1,
1 => bb6,
172026258624175713727831469518287257377 => bb11,
_ => bb5
}
}
bb11 = {
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2 = ((-127642411132080303240046115854621610843_i128), (*_10), Field::<u16>(Variant(_7, 1), 0), RET);
match Field::<i32>(Variant(_24, 3), 0) {
0 => bb4,
340282366920938463463374607431599589869 => bb12,
_ => bb2
}
}
bb12 = {
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).1 = [_3,_3,_3];
_18 = _19.0.fld2 as isize;
_15 = !_11;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2 = (11099528798706386203558501885296914738_i128, (*_10), Field::<u16>(Variant(_7, 1), 0), RET);
SetDiscriminant(_7, 2);
(*_10) = !2912242084_u32;
_20 = _19.0.fld0;
(*_10) = _21 as u32;
_19.0.fld0 = _22.0 != _22.0;
_19.0.fld0 = _15 != _11;
_4 = -_14;
match _26 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
340282366920938463463374607431599589869 => bb19,
_ => bb18
}
}
bb13 = {
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2 = ((-127642411132080303240046115854621610843_i128), (*_10), Field::<u16>(Variant(_7, 1), 0), RET);
match Field::<i32>(Variant(_24, 3), 0) {
0 => bb4,
340282366920938463463374607431599589869 => bb12,
_ => bb2
}
}
bb14 = {
_18 = _8 | _8;
_14 = _4;
_16 = &(*_16);
_19.0.fld2 = _4 as f64;
_1 = !0_usize;
_1 = 1712208297_u32 as usize;
_19.0.fld0 = true;
_9 = [_18,_17,_15,_18,_18,_17,_17];
_6 = Adt67::Variant0 { fld0: _19.0,fld1: 11601_i16,fld2: _19.0.fld2 };
_15 = _18;
_20 = !_19.0.fld0;
_23 = -Field::<Adt40>(Variant(_6, 0), 0).fld1;
_1 = 34_u8 as usize;
_4 = Field::<f64>(Variant(_6, 0), 2) as f32;
_22.0 = !_1;
_18 = _15 + _17;
_9 = [_5,_17,_18,_11,_18,_18,_15];
RET = '\u{db34c}';
Goto(bb6)
}
bb15 = {
_16 = &_25.0;
_22.0 = !_1;
_25.0 = _4 * _4;
_27 = -_26;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).0 = Field::<f64>(Variant(_24, 2), 2);
_16 = &_25.0;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2 = ((-168256108296762749735543137913480954079_i128), Field::<u32>(Variant(_6, 1), 1), 45062_u16, RET);
(*_10) = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).2.1 + Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).2.1;
_27 = -_26;
place!(Field::<u16>(Variant(_7, 1), 0)) = 313939545538100807628839515799827178499_u128 as u16;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).0 = Field::<f64>(Variant(_24, 2), 2);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).1 = [_3,_3,_3];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).1 = [_3,Field::<i64>(Variant(_24, 2), 1),Field::<i64>(Variant(_24, 2), 1)];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
place!(Field::<u64>(Variant(_24, 2), 0)) = 15175571509531377209_u64 + 4686120420184364913_u64;
_25 = (_23,);
_24 = Adt35::Variant2 { fld0: 2569197417244413032_u64,fld1: _3,fld2: _19.0.fld2 };
_9 = [_5,_8,_11,_17,_15,_15,_18];
place!(Field::<u64>(Variant(_24, 2), 0)) = 11688194702637284681_u64 * 8062706813615113243_u64;
_19.0.fld2 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1).0;
_5 = -_8;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.3 = RET;
_20 = _21;
_10 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_6, 1), 1)));
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_7, 1), 1)).2.1 = (*_10) << _17;
Goto(bb10)
}
bb16 = {
_3 = !(-5721942032232676801_i64);
_14 = -_25.0;
_27 = _20 as i32;
_24 = Adt35::Variant1 { fld0: Field::<u32>(Variant(_6, 1), 1),fld1: RET };
place!(Field::<u32>(Variant(_6, 1), 1)) = _19.0.fld1 as u32;
_3 = 18136785442228268221_u64 as i64;
_8 = !_17;
place!(Field::<[isize; 7]>(Variant(_6, 1), 0)) = [_11,_8,_17,_8,_15,_18,_15];
_3 = -(-3632824467673454469_i64);
_2 = 58147_u16 as isize;
_24 = Adt35::Variant2 { fld0: 9947373041836771334_u64,fld1: _3,fld2: _19.0.fld2 };
_4 = -_14;
place!(Field::<*mut u64>(Variant(_7, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<u64>(Variant(_24, 2), 0)));
_27 = _26;
_19.0.fld2 = Field::<f64>(Variant(_24, 2), 2) - Field::<f64>(Variant(_24, 2), 2);
_19.0.fld1 = _4 - _25.0;
place!(Field::<u32>(Variant(_6, 1), 1)) = _3 as u32;
_25 = (_23,);
RET = '\u{67189}';
_10 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_6, 1), 1)));
RET = '\u{962b4}';
_32 = [(-23678_i16),(-8524_i16),10778_i16,(-21099_i16)];
SetDiscriminant(_7, 1);
Goto(bb9)
}
bb17 = {
_2 = -_8;
_2 = true as isize;
_5 = !_11;
_5 = _8 ^ _8;
SetDiscriminant(_7, 0);
_1 = 0_usize;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0))[_1] = (-48551266827565459696696314900596628327_i128) * 22438710590112626419602828770125003338_i128;
place!(Field::<[i128; 6]>(Variant(_7, 0), 0)) = [(-128754589982759996588674409691997638643_i128),(-126618703771401428650592084415578023498_i128),(-107642504115638608113934628695399984609_i128),(-107970848299675040594172150345181865431_i128),110390791026546857559841207210814010750_i128,(-143153526816715229466222035999530561924_i128)];
_9 = [_8,_5,_5,_5,_5,_11,_5];
_8 = -_5;
_15 = Field::<[i128; 6]>(Variant(_7, 0), 0)[_1] as isize;
RET = '\u{101c75}';
_14 = (-2073632070_i32) as f32;
_3 = 5606888899300790189_i64 & (-5181106742883180860_i64);
RET = '\u{f9d66}';
RET = '\u{2125b}';
Goto(bb4)
}
bb18 = {
_18 = 137254416978001739082376413132686366730_i128 as isize;
place!(Field::<Adt40>(Variant(_6, 0), 0)) = Adt40 { fld0: _20,fld1: _14,fld2: Field::<f64>(Variant(_6, 0), 2) };
_19.0.fld1 = 1499973083_u32 as f32;
_22.0 = _1;
RET = '\u{64d32}';
place!(Field::<Adt40>(Variant(_6, 0), 0)).fld1 = _19.0.fld1;
_8 = !_15;
_1 = !_22.0;
place!(Field::<Adt40>(Variant(_6, 0), 0)).fld0 = !_19.0.fld0;
_24 = Adt35::Variant3 { fld0: (-1722351490_i32),fld1: 117_u8 };
_25.0 = _14 * _4;
_24 = Adt35::Variant0 { fld0: Field::<[i128; 6]>(Variant(_7, 0), 0),fld1: _1,fld2: _8 };
_25.0 = _23 * _23;
_9 = [_11,_15,_15,Field::<isize>(Variant(_24, 0), 2),_15,_8,_15];
_16 = &_14;
_26 = (-168621587_i32);
_21 = !_19.0.fld0;
SetDiscriminant(_24, 1);
_19.0.fld2 = Field::<Adt40>(Variant(_6, 0), 0).fld2;
_19.0 = Adt40 { fld0: _21,fld1: _25.0,fld2: Field::<Adt40>(Variant(_6, 0), 0).fld2 };
Goto(bb7)
}
bb19 = {
_19.1 = core::ptr::addr_of_mut!(place!(Field::<(u8, u64)>(Variant(_7, 2), 1)).1);
_22.1 = Adt21::Variant2 { fld0: _20,fld1: RET,fld2: (*_10),fld3: _23,fld4: 42515_u16,fld5: Field::<i32>(Variant(_24, 3), 0) };
_19.0.fld0 = !_20;
_25 = (_19.0.fld1,);
place!(Field::<u32>(Variant(_22.1, 2), 2)) = 2_u8 as u32;
place!(Field::<i32>(Variant(_24, 3), 0)) = 14635_u16 as i32;
_30 = Field::<bool>(Variant(_22.1, 2), 0);
place!(Field::<(u8, u64)>(Variant(_7, 2), 1)).1 = !3958013039649011661_u64;
_25 = (_4,);
place!(Field::<u32>(Variant(_6, 1), 1)) = Field::<u32>(Variant(_22.1, 2), 2) & Field::<u32>(Variant(_22.1, 2), 2);
place!(Field::<u16>(Variant(_22.1, 2), 4)) = !33299_u16;
_26 = _27;
_40 = -_5;
RET = Field::<char>(Variant(_22.1, 2), 1);
_4 = -_14;
_17 = _21 as isize;
SetDiscriminant(_6, 2);
_39 = [28_u8,112_u8,98_u8,76_u8,92_u8,185_u8];
place!(Field::<bool>(Variant(_22.1, 2), 0)) = _35;
place!(Field::<u8>(Variant(_24, 3), 1)) = 3_u8 & 193_u8;
_12 = core::ptr::addr_of_mut!(place!(Field::<(u8, u64)>(Variant(_7, 2), 1)).1);
Goto(bb20)
}
bb20 = {
Call(_43 = dump_var(3_usize, 3_usize, Move(_3), 17_usize, Move(_17), 27_usize, Move(_27), 30_usize, Move(_30)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_43 = dump_var(3_usize, 26_usize, Move(_26), 35_usize, Move(_35), 40_usize, Move(_40), 32_usize, Move(_32)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_43 = dump_var(3_usize, 15_usize, Move(_15), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4() -> Adt77 {
mir! {
type RET = Adt77;
let _1: (i128, u32, u16, char);
let _2: &'static &'static u8;
let _3: f64;
let _4: &'static *mut usize;
let _5: isize;
let _6: isize;
let _7: usize;
let _8: f64;
let _9: &'static u64;
let _10: bool;
let _11: &'static i64;
let _12: *mut usize;
let _13: [u16; 7];
let _14: i8;
let _15: f64;
let _16: *mut i16;
let _17: f64;
let _18: *const &'static f32;
let _19: isize;
let _20: f32;
let _21: bool;
let _22: *const &'static *mut usize;
let _23: u64;
let _24: &'static &'static Adt21;
let _25: isize;
let _26: [u128; 1];
let _27: u32;
let _28: i128;
let _29: f32;
let _30: f64;
let _31: Adt40;
let _32: i128;
let _33: (&'static i64,);
let _34: i128;
let _35: &'static &'static Adt21;
let _36: (Adt70,);
let _37: (&'static *mut u64,);
let _38: Adt33;
let _39: u64;
let _40: Adt36;
let _41: f32;
let _42: [i128; 4];
let _43: &'static &'static [isize; 7];
let _44: f32;
let _45: i16;
let _46: f64;
let _47: Adt67;
let _48: i8;
let _49: char;
let _50: u128;
let _51: i32;
let _52: bool;
let _53: [i16; 4];
let _54: char;
let _55: &'static f64;
let _56: &'static *mut usize;
let _57: &'static f64;
let _58: f64;
let _59: (&'static *mut u64,);
let _60: Adt36;
let _61: ([u64; 6], [u128; 1], (usize, Adt21), (Adt35, &'static *mut u64, u128));
let _62: (&'static [i16; 1], char, u64);
let _63: (Adt40, *mut u64);
let _64: (&'static i64,);
let _65: ((usize, Adt21),);
let _66: f64;
let _67: &'static *const &'static *mut usize;
let _68: [i128; 6];
let _69: isize;
let _70: [i16; 4];
let _71: isize;
let _72: (usize, Adt21);
let _73: [usize; 4];
let _74: u8;
let _75: [u32; 2];
let _76: isize;
let _77: f64;
let _78: bool;
let _79: u8;
let _80: f32;
let _81: &'static u64;
let _82: isize;
let _83: f64;
let _84: bool;
let _85: bool;
let _86: i8;
let _87: usize;
let _88: i8;
let _89: (&'static i64,);
let _90: i8;
let _91: [u16; 7];
let _92: Adt36;
let _93: f32;
let _94: &'static *const &'static *mut usize;
let _95: i64;
let _96: (Adt35, &'static *mut u64, u128);
let _97: bool;
let _98: [u32; 2];
let _99: (Adt40, *mut u64);
let _100: isize;
let _101: *const u32;
let _102: (&'static [i16; 1], char, u64);
let _103: [u32; 2];
let _104: i32;
let _105: bool;
let _106: ();
let _107: ();
{
_1 = (8819480094701423681908658909142193580_i128, 66429953_u32, 3515_u16, '\u{99662}');
_1.2 = !62653_u16;
_1.0 = 38735518440223171841422070232076591149_i128 * 81840295493447575640377019686773020799_i128;
_1.2 = 60278_u16 & 37550_u16;
_1.3 = '\u{cf037}';
_1.3 = '\u{daee5}';
_1.0 = (-6025990171010553929_i64) as i128;
_1.1 = 16073080600585155693_u64 as u32;
_3 = 10382835735008353227_u64 as f64;
_1.2 = !54094_u16;
_1 = ((-76809531297935116434241925467149978326_i128), 2363760640_u32, 5037_u16, '\u{eac59}');
_1.0 = 113_i8 as i128;
_1.3 = '\u{a401}';
Call(_1.3 = fn5(_1.1, _1.2, _1.2, _1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = (17412936098868907529691277078043773618_i128, 2651181734_u32, 42971_u16, '\u{4f954}');
_1.2 = (-25502_i16) as u16;
_1.2 = 111_u8 as u16;
_1.3 = '\u{3777b}';
_1.2 = 37938_u16;
Goto(bb2)
}
bb2 = {
_7 = 4291714953701974167_usize ^ 7_usize;
_8 = -_3;
_5 = -95_isize;
_6 = 179_u8 as isize;
_3 = _8 - _8;
_5 = _6;
_1.0 = 117758243396219454145453113323481068771_i128 + (-147121641766645104648002514137601998886_i128);
_1.2 = 47690_u16 ^ 9271_u16;
_8 = _7 as f64;
_10 = true;
_7 = 18651_i16 as usize;
_1.0 = (-44847601383839168315562423967626500161_i128) >> _1.2;
_1.3 = '\u{f4a71}';
_1 = (99659841210044941296769215234232778533_i128, 3091769662_u32, 37636_u16, '\u{72595}');
_6 = _5;
_6 = !_5;
_1 = ((-120168511046275137512864090846320795979_i128), 1970275993_u32, 19869_u16, '\u{53957}');
_4 = &_12;
_7 = !0_usize;
_5 = (-30694_i16) as isize;
_1.1 = 3823498151_u32 >> _1.0;
_1.3 = '\u{2cde5}';
_4 = &(*_4);
match _1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
220113855874663325950510516585447415477 => bb6,
_ => bb5
}
}
bb3 = {
_1 = (17412936098868907529691277078043773618_i128, 2651181734_u32, 42971_u16, '\u{4f954}');
_1.2 = (-25502_i16) as u16;
_1.2 = 111_u8 as u16;
_1.3 = '\u{3777b}';
_1.2 = 37938_u16;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_5 = _6;
_3 = _1.1 as f64;
_1.3 = '\u{9cf6c}';
_6 = _5 >> _1.2;
_1.3 = '\u{79249}';
_5 = _6 ^ _6;
_7 = _10 as usize;
_1 = ((-136065204728609982072318487335829211104_i128), 2401731733_u32, 52373_u16, '\u{b33d9}');
_4 = &(*_4);
Goto(bb7)
}
bb7 = {
_8 = -_3;
_6 = _1.3 as isize;
_1.3 = '\u{c1720}';
_3 = -_8;
_1.0 = 11227486749712530888031007610748322374_i128 & 46828054043842666023092716696262945145_i128;
_13 = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
_12 = core::ptr::addr_of_mut!(_7);
_1.1 = 3987737989_u32 << _6;
_10 = false ^ false;
(*_12) = 9333616576681557999_usize;
_8 = _1.1 as f64;
_3 = _8;
_1.0 = -(-137103951230928935111530788167309686856_i128);
(*_12) = 4_usize >> _5;
_1.0 = !(-31126843830296301358429375878577074857_i128);
_3 = _8;
_15 = -_8;
_4 = &_12;
_17 = _3;
_15 = _8 * _8;
_7 = 5_usize;
_17 = _3;
match (*_12) {
0 => bb4,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
6 => bb12,
5 => bb14,
_ => bb13
}
}
bb8 = {
_5 = _6;
_3 = _1.1 as f64;
_1.3 = '\u{9cf6c}';
_6 = _5 >> _1.2;
_1.3 = '\u{79249}';
_5 = _6 ^ _6;
_7 = _10 as usize;
_1 = ((-136065204728609982072318487335829211104_i128), 2401731733_u32, 52373_u16, '\u{b33d9}');
_4 = &(*_4);
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_1 = (17412936098868907529691277078043773618_i128, 2651181734_u32, 42971_u16, '\u{4f954}');
_1.2 = (-25502_i16) as u16;
_1.2 = 111_u8 as u16;
_1.3 = '\u{3777b}';
_1.2 = 37938_u16;
Goto(bb2)
}
bb12 = {
_7 = 4291714953701974167_usize ^ 7_usize;
_8 = -_3;
_5 = -95_isize;
_6 = 179_u8 as isize;
_3 = _8 - _8;
_5 = _6;
_1.0 = 117758243396219454145453113323481068771_i128 + (-147121641766645104648002514137601998886_i128);
_1.2 = 47690_u16 ^ 9271_u16;
_8 = _7 as f64;
_10 = true;
_7 = 18651_i16 as usize;
_1.0 = (-44847601383839168315562423967626500161_i128) >> _1.2;
_1.3 = '\u{f4a71}';
_1 = (99659841210044941296769215234232778533_i128, 3091769662_u32, 37636_u16, '\u{72595}');
_6 = _5;
_6 = !_5;
_1 = ((-120168511046275137512864090846320795979_i128), 1970275993_u32, 19869_u16, '\u{53957}');
_4 = &_12;
_7 = !0_usize;
_5 = (-30694_i16) as isize;
_1.1 = 3823498151_u32 >> _1.0;
_1.3 = '\u{2cde5}';
_4 = &(*_4);
match _1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
220113855874663325950510516585447415477 => bb6,
_ => bb5
}
}
bb13 = {
_1 = (17412936098868907529691277078043773618_i128, 2651181734_u32, 42971_u16, '\u{4f954}');
_1.2 = (-25502_i16) as u16;
_1.2 = 111_u8 as u16;
_1.3 = '\u{3777b}';
_1.2 = 37938_u16;
Goto(bb2)
}
bb14 = {
_12 = core::ptr::addr_of_mut!((*_12));
_6 = _5 << _1.2;
_1 = ((-11471791358739014910349832200653892425_i128), 10406887_u32, _13[_7], '\u{fa362}');
_14 = 6981550793290361163_u64 as i8;
_19 = _5 >> _5;
_8 = -_3;
_13 = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
_20 = _1.0 as f32;
_5 = _19 - _19;
_10 = true;
_4 = &_12;
_7 = 6_usize - 3_usize;
_5 = _19;
(*_12) = !16591470855068545001_usize;
_5 = -_6;
_17 = _8 + _8;
(*_12) = 11124077866442634194_usize;
_10 = !true;
_23 = 5302398743511912306_u64;
_6 = _5;
_8 = _3;
_14 = !6_i8;
_1.3 = '\u{f305c}';
_8 = 1872425475_i32 as f64;
_7 = !4_usize;
Goto(bb15)
}
bb15 = {
_1.2 = _23 as u16;
_7 = !0_usize;
_22 = core::ptr::addr_of!(_4);
_14 = (-16_i8);
_7 = 1467182759178214227_usize >> _19;
_3 = _15;
_19 = _1.3 as isize;
_1.1 = 2475840592_u32;
_4 = &(*_4);
_13 = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
_8 = _17;
_5 = _6;
_1.2 = 30149_u16 & 42906_u16;
_21 = !_10;
_20 = _1.1 as f32;
_3 = -_15;
(*_22) = &(*_4);
_21 = _10;
_3 = 241_u8 as f64;
_23 = _5 as u64;
_9 = &_23;
_26 = [95028377643680034265515886403247305690_u128];
_3 = _20 as f64;
(*_12) = 10131292210671983778_usize & 5_usize;
_12 = core::ptr::addr_of_mut!(_7);
Call(_6 = core::intrinsics::transmute((*_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_9 = &(*_9);
_20 = 64578671_i32 as f32;
_13 = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
_28 = _1.0;
_21 = !_10;
_5 = _6;
_25 = _19 * _6;
Goto(bb17)
}
bb17 = {
(*_22) = &_12;
_9 = &(*_9);
_1.0 = _28 & _28;
_22 = core::ptr::addr_of!((*_22));
_9 = &(*_9);
_5 = !_6;
_1 = (_28, 143274265_u32, 62615_u16, '\u{4a32d}');
_20 = _23 as f32;
_19 = _5 ^ _6;
_1.3 = '\u{97b33}';
_29 = -_20;
_7 = !3_usize;
_1.1 = 3374335115_u32;
Call(_7 = fn18(Move((*_22)), Move(_22), _23, _15, _3), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
match _1.2 {
0 => bb8,
1 => bb15,
2 => bb3,
3 => bb14,
62615 => bb19,
_ => bb12
}
}
bb19 = {
(*_12) = 8539970142244104008_usize;
_7 = _14 as usize;
(*_12) = !16689180882379762642_usize;
_23 = 17746788750568596571_u64 * 7019670419438862758_u64;
_26 = [205929155441644764510589471906289013924_u128];
_12 = core::ptr::addr_of_mut!((*_12));
_1.2 = 1548_u16 ^ 14263_u16;
_32 = _15 as i128;
_20 = (-24669_i16) as f32;
_31 = Adt40 { fld0: _10,fld1: _29,fld2: _15 };
_23 = 8281116580591568060_u64 & 6742625448370917773_u64;
_15 = _17 * _8;
_34 = _14 as i128;
_7 = 7_usize;
_31 = Adt40 { fld0: _21,fld1: _20,fld2: _15 };
(*_12) = 7_usize + 2_usize;
_32 = _1.1 as i128;
_30 = _15;
_19 = _28 as isize;
Call(_15 = core::intrinsics::transmute(_19), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_5 = _19;
(*_12) = 6_usize;
_38.fld4.0 = (*_12) >> _28;
_38.fld4.0 = !_7;
_20 = _29;
_38.fld1[_7] = !_10;
_15 = _1.1 as f64;
_38.fld6 = [31593_i16,(-21441_i16),(-5352_i16),8101_i16];
_39 = !_23;
_10 = _21;
_3 = (-1772168209_i32) as f64;
_27 = _1.1 - _1.1;
_20 = _29;
_6 = !_25;
_25 = !_19;
_15 = -_30;
_38.fld3 = _14;
_39 = _23;
_1.2 = _13[_7] + _13[_7];
_40.fld1 = [_5,_25,_19,_19,_25,_5,_25];
Goto(bb21)
}
bb21 = {
_40.fld1[_7] = !_25;
_32 = 231_u8 as i128;
_1.2 = _20 as u16;
_38.fld5 = 314179424819205827808105960957491712604_u128 as u64;
_6 = _39 as isize;
_38.fld2 = _1.2;
_15 = _30;
_41 = _29 + _29;
match _28 {
0 => bb22,
1 => bb23,
328810575562199448553024775231114319031 => bb25,
_ => bb24
}
}
bb22 = {
_12 = core::ptr::addr_of_mut!((*_12));
_6 = _5 << _1.2;
_1 = ((-11471791358739014910349832200653892425_i128), 10406887_u32, _13[_7], '\u{fa362}');
_14 = 6981550793290361163_u64 as i8;
_19 = _5 >> _5;
_8 = -_3;
_13 = [_1.2,_1.2,_1.2,_1.2,_1.2,_1.2,_1.2];
_20 = _1.0 as f32;
_5 = _19 - _19;
_10 = true;
_4 = &_12;
_7 = 6_usize - 3_usize;
_5 = _19;
(*_12) = !16591470855068545001_usize;
_5 = -_6;
_17 = _8 + _8;
(*_12) = 11124077866442634194_usize;
_10 = !true;
_23 = 5302398743511912306_u64;
_6 = _5;
_8 = _3;
_14 = !6_i8;
_1.3 = '\u{f305c}';
_8 = 1872425475_i32 as f64;
_7 = !4_usize;
Goto(bb15)
}
bb23 = {
_5 = _6;
_3 = _1.1 as f64;
_1.3 = '\u{9cf6c}';
_6 = _5 >> _1.2;
_1.3 = '\u{79249}';
_5 = _6 ^ _6;
_7 = _10 as usize;
_1 = ((-136065204728609982072318487335829211104_i128), 2401731733_u32, 52373_u16, '\u{b33d9}');
_4 = &(*_4);
Goto(bb7)
}
bb24 = {
Return()
}
bb25 = {
_31 = Adt40 { fld0: _21,fld1: _41,fld2: _30 };
_6 = !_5;
_14 = -_38.fld3;
_19 = !_25;
_38.fld4.1 = Adt21::Variant2 { fld0: _21,fld1: _1.3,fld2: _1.1,fld3: _31.fld1,fld4: _1.2,fld5: 227648113_i32 };
_41 = -Field::<f32>(Variant(_38.fld4.1, 2), 3);
place!(Field::<u32>(Variant(_38.fld4.1, 2), 2)) = _27;
_22 = core::ptr::addr_of!(_4);
_38.fld3 = _14;
_45 = -32594_i16;
_23 = _39;
_30 = _15;
_31.fld1 = _38.fld3 as f32;
_6 = _1.0 as isize;
_22 = core::ptr::addr_of!((*_22));
place!(Field::<char>(Variant(_38.fld4.1, 2), 1)) = _1.3;
place!(Field::<bool>(Variant(_38.fld4.1, 2), 0)) = _38.fld1[_7];
(*_22) = &_12;
Goto(bb26)
}
bb26 = {
_40.fld1 = [_6,_25,_19,_6,_25,_19,_19];
_48 = !_38.fld3;
_38.fld1[_7] = !_21;
_5 = _6 & _19;
_1.2 = 117_u8 as u16;
_42 = [_28,_28,_28,_28];
_5 = _6;
_38.fld1 = [_21,Field::<bool>(Variant(_38.fld4.1, 2), 0),_21,_31.fld0,_21,Field::<bool>(Variant(_38.fld4.1, 2), 0),Field::<bool>(Variant(_38.fld4.1, 2), 0)];
_7 = _38.fld4.0;
(*_22) = &(*_4);
(*_12) = _38.fld4.0;
_28 = _1.0 << _6;
_1.0 = _45 as i128;
_38.fld6 = [_45,_45,_45,_45];
_38.fld0 = _21;
Goto(bb27)
}
bb27 = {
_34 = _28 << _1.0;
_4 = &(*_4);
_38.fld6 = [_45,_45,_45,_45];
match _1.1 {
0 => bb5,
3374335115 => bb28,
_ => bb19
}
}
bb28 = {
(*_12) = _38.fld4.0 >> _38.fld2;
_28 = _34;
_38.fld5 = !_23;
place!(Field::<char>(Variant(_38.fld4.1, 2), 1)) = _1.3;
_31.fld1 = _29 - Field::<f32>(Variant(_38.fld4.1, 2), 3);
match _1.1 {
0 => bb26,
1 => bb2,
2 => bb18,
3 => bb16,
4 => bb5,
3374335115 => bb29,
_ => bb6
}
}
bb29 = {
_9 = &_38.fld5;
place!(Field::<i32>(Variant(_38.fld4.1, 2), 5)) = (-497388662_i32) << _28;
Goto(bb30)
}
bb30 = {
_14 = _48 >> _34;
_1.0 = -_32;
_6 = _25;
_26 = [158277895947399368237446899537532358472_u128];
_5 = Field::<f32>(Variant(_38.fld4.1, 2), 3) as isize;
_1 = (_34, Field::<u32>(Variant(_38.fld4.1, 2), 2), _38.fld2, Field::<char>(Variant(_38.fld4.1, 2), 1));
_31 = Adt40 { fld0: Field::<bool>(Variant(_38.fld4.1, 2), 0),fld1: Field::<f32>(Variant(_38.fld4.1, 2), 3),fld2: _30 };
_31.fld2 = -_30;
_7 = _38.fld4.0 >> _38.fld2;
_51 = Field::<i32>(Variant(_38.fld4.1, 2), 5);
_22 = core::ptr::addr_of!((*_22));
Call(_3 = core::intrinsics::transmute((*_9)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_42 = [_1.0,_34,_28,_34];
Goto(bb32)
}
bb32 = {
_16 = core::ptr::addr_of_mut!(_45);
(*_12) = _38.fld4.0 + _38.fld4.0;
SetDiscriminant(_38.fld4.1, 2);
_4 = &(*_4);
_40.fld0 = Adt21::Variant0 { fld0: (*_12),fld1: _38.fld1,fld2: 0_u8,fld3: _14,fld4: 115310710721624515883671881683856591330_u128,fld5: _31.fld2,fld6: 2718752494293037556_i64 };
place!(Field::<i64>(Variant(_40.fld0, 0), 6)) = (-6107384337851752213_i64);
place!(Field::<u8>(Variant(_40.fld0, 0), 2)) = !21_u8;
_33.0 = &place!(Field::<i64>(Variant(_40.fld0, 0), 6));
place!(Field::<i32>(Variant(_38.fld4.1, 2), 5)) = 213800374617773674296312231855516430112_u128 as i32;
Goto(bb33)
}
bb33 = {
_55 = &_31.fld2;
_1.2 = !_38.fld2;
(*_22) = &_12;
place!(Field::<u16>(Variant(_38.fld4.1, 2), 4)) = _38.fld2;
place!(Field::<u8>(Variant(_40.fld0, 0), 2)) = 180_u8 | 177_u8;
_47 = Adt67::Variant0 { fld0: _31,fld1: _45,fld2: _8 };
SetDiscriminant(_47, 2);
_54 = _1.3;
_11 = Move(_33.0);
place!(Field::<bool>(Variant(_38.fld4.1, 2), 0)) = _38.fld0;
_45 = !(-25779_i16);
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)) = _1;
(*_12) = Field::<usize>(Variant(_40.fld0, 0), 0);
place!(Field::<*mut usize>(Variant(_47, 2), 4)) = core::ptr::addr_of_mut!((*_12));
place!(Field::<i64>(Variant(_40.fld0, 0), 6)) = (-7892092012983391509_i64);
_47 = Adt67::Variant0 { fld0: _31,fld1: (*_16),fld2: _8 };
match Field::<i64>(Variant(_40.fld0, 0), 6) {
340282366920938463455482515418784819947 => bb35,
_ => bb34
}
}
bb34 = {
(*_22) = &_12;
_9 = &(*_9);
_1.0 = _28 & _28;
_22 = core::ptr::addr_of!((*_22));
_9 = &(*_9);
_5 = !_6;
_1 = (_28, 143274265_u32, 62615_u16, '\u{4a32d}');
_20 = _23 as f32;
_19 = _5 ^ _6;
_1.3 = '\u{97b33}';
_29 = -_20;
_7 = !3_usize;
_1.1 = 3374335115_u32;
Call(_7 = fn18(Move((*_22)), Move(_22), _23, _15, _3), ReturnTo(bb18), UnwindUnreachable())
}
bb35 = {
_31.fld1 = _19 as f32;
_31.fld1 = _28 as f32;
_1.1 = _27 ^ _27;
SetDiscriminant(_47, 2);
(*_12) = _38.fld2 as usize;
_33.0 = &place!(Field::<i64>(Variant(_40.fld0, 0), 6));
_46 = _15;
_27 = _1.1 << Field::<i8>(Variant(_40.fld0, 0), 3);
_9 = &(*_9);
_3 = -_30;
place!(Field::<u32>(Variant(_47, 2), 1)) = _27 + _27;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).2.0 = (*_9) as i128;
_38.fld4.1 = Adt21::Variant0 { fld0: _7,fld1: Field::<[bool; 7]>(Variant(_40.fld0, 0), 1),fld2: Field::<u8>(Variant(_40.fld0, 0), 2),fld3: _14,fld4: 82513361028503987739349014301358894015_u128,fld5: _31.fld2,fld6: Field::<i64>(Variant(_40.fld0, 0), 6) };
_54 = _1.3;
_1.1 = !Field::<u32>(Variant(_47, 2), 1);
place!(Field::<u8>(Variant(_38.fld4.1, 0), 2)) = _51 as u8;
_58 = _3 + (*_55);
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)).3 = _54;
_38.fld4.0 = _7;
_62.1 = _1.3;
(*_12) = Field::<usize>(Variant(_38.fld4.1, 0), 0) & Field::<usize>(Variant(_38.fld4.1, 0), 0);
_33.0 = &place!(Field::<i64>(Variant(_38.fld4.1, 0), 6));
_50 = Field::<u8>(Variant(_38.fld4.1, 0), 2) as u128;
Goto(bb36)
}
bb36 = {
_53 = _38.fld6;
(*_22) = &(*_4);
_53 = [(*_16),(*_16),(*_16),(*_16)];
_59.0 = &_63.1;
_15 = Field::<f64>(Variant(_38.fld4.1, 0), 5);
_63.0.fld0 = _38.fld0 | _21;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).2.1 = Field::<u8>(Variant(_38.fld4.1, 0), 2) as u32;
_38.fld0 = !_63.0.fld0;
_63.0.fld0 = !_38.fld0;
_61.3.2 = _50 >> _1.2;
_9 = &_39;
_22 = core::ptr::addr_of!(_4);
_1.2 = _61.3.2 as u16;
_63.0 = Adt40 { fld0: _31.fld0,fld1: _41,fld2: Field::<f64>(Variant(_40.fld0, 0), 5) };
_62.2 = !_23;
_40.fld1 = [_5,_5,_5,_5,_5,_5,_6];
_59.0 = &_63.1;
place!(Field::<[i32; 6]>(Variant(_47, 2), 0)) = [_51,_51,_51,_51,_51,_51];
_31.fld2 = _8;
_61.1 = [_61.3.2];
_52 = !_31.fld0;
(*_22) = &(*_4);
_55 = &_30;
_61.0 = [_38.fld5,(*_9),_23,(*_9),_38.fld5,_39];
Goto(bb37)
}
bb37 = {
_28 = _1.0;
Call(place!(Field::<i64>(Variant(_40.fld0, 0), 6)) = core::intrinsics::bswap(Field::<i64>(Variant(_38.fld4.1, 0), 6)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).1 = [Field::<i64>(Variant(_40.fld0, 0), 6),Field::<i64>(Variant(_38.fld4.1, 0), 6),Field::<i64>(Variant(_38.fld4.1, 0), 6)];
place!(Field::<u128>(Variant(_40.fld0, 0), 4)) = _50;
_44 = -_63.0.fld1;
_65.0 = (_7, Move(_40.fld0));
_11 = &place!(Field::<i64>(Variant(_38.fld4.1, 0), 6));
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).0 = _58;
_40.fld1 = [_19,_19,_19,_6,_25,_19,_19];
_70 = [(*_16),(*_16),(*_16),(*_16)];
_38.fld4.1 = Adt21::Variant2 { fld0: _38.fld0,fld1: _54,fld2: _1.1,fld3: _41,fld4: _1.2,fld5: _51 };
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)).0 = _34;
_38.fld2 = _1.2;
Goto(bb39)
}
bb39 = {
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)) = _1;
_38 = Adt33 { fld0: _21,fld1: Field::<[bool; 7]>(Variant(_65.0.1, 0), 1),fld2: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).2,fld3: Field::<i8>(Variant(_65.0.1, 0), 3),fld4: Move(_65.0),fld5: _62.2,fld6: _70 };
_63.0.fld1 = Field::<u8>(Variant(_38.fld4.1, 0), 2) as f32;
_42 = [_34,Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0,_34,Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0];
place!(Field::<u128>(Variant(_38.fld4.1, 0), 4)) = _50 - _50;
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)) = _1;
_39 = _1.2 as u64;
_75 = [Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1,Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1];
place!(Field::<i8>(Variant(_38.fld4.1, 0), 3)) = _38.fld3;
_42 = [_34,_34,_28,Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0];
place!(Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3)) = (_1.0, Field::<u32>(Variant(_47, 2), 1), _1.2, _62.1);
Goto(bb40)
}
bb40 = {
_61.3.0 = Adt35::Variant2 { fld0: _39,fld1: Field::<i64>(Variant(_38.fld4.1, 0), 6),fld2: _31.fld2 };
_19 = _5 >> _38.fld2;
_61.2.0 = !_38.fld4.0;
(*_22) = &_12;
_72.0 = (*_12);
_31.fld0 = _1.2 > Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).2;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).1 = [Field::<i64>(Variant(_61.3.0, 2), 1),Field::<i64>(Variant(_38.fld4.1, 0), 6),Field::<i64>(Variant(_38.fld4.1, 0), 6)];
_61.3.0 = Adt35::Variant3 { fld0: _51,fld1: Field::<u8>(Variant(_38.fld4.1, 0), 2) };
SetDiscriminant(_38.fld4.1, 1);
place!(Field::<[bool; 7]>(Variant(_38.fld4.1, 1), 4)) = [_31.fld0,_31.fld0,_31.fld0,_31.fld0,_52,_31.fld0,_31.fld0];
_20 = _44;
Goto(bb41)
}
bb41 = {
_60.fld0 = Adt21::Variant2 { fld0: _31.fld0,fld1: _1.3,fld2: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1,fld3: _29,fld4: _1.2,fld5: Field::<i32>(Variant(_61.3.0, 3), 0) };
_16 = core::ptr::addr_of_mut!((*_16));
_72 = ((*_12), Move(_60.fld0));
_57 = &_77;
place!(Field::<[i32; 6]>(Variant(_47, 2), 0)) = [Field::<i32>(Variant(_61.3.0, 3), 0),_51,Field::<i32>(Variant(_72.1, 2), 5),Field::<i32>(Variant(_61.3.0, 3), 0),Field::<i32>(Variant(_72.1, 2), 5),Field::<i32>(Variant(_61.3.0, 3), 0)];
place!(Field::<bool>(Variant(_38.fld4.1, 1), 0)) = _31.fld0;
_57 = Move(_55);
place!(Field::<bool>(Variant(_38.fld4.1, 1), 0)) = !_31.fld0;
_63.1 = core::ptr::addr_of_mut!(_39);
_1.1 = Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1;
_38.fld3 = !_14;
place!(Field::<u8>(Variant(_61.3.0, 3), 1)) = 100_u8;
_62.1 = Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).3;
_71 = _39 as isize;
_75 = [Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1,_27];
(*_16) = (-9248_i16) | 14011_i16;
place!(Field::<bool>(Variant(_72.1, 2), 0)) = _31.fld0;
(*_22) = &_12;
_1.3 = _62.1;
_19 = (*_16) as isize;
_56 = &_12;
_58 = _46;
_50 = !_61.3.2;
_45 = -(-25094_i16);
_37.0 = &_63.1;
_38.fld3 = _14 - _14;
_1.3 = Field::<char>(Variant(_72.1, 2), 1);
Goto(bb42)
}
bb42 = {
_38.fld3 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5).0 as i8;
_75 = [_1.1,Field::<u32>(Variant(_72.1, 2), 2)];
_16 = core::ptr::addr_of_mut!((*_16));
_40.fld0 = Adt21::Variant1 { fld0: _31.fld0,fld1: _38.fld4.0,fld2: _1.1,fld3: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0,fld4: Field::<[bool; 7]>(Variant(_38.fld4.1, 1), 4) };
SetDiscriminant(_40.fld0, 2);
_65.0.0 = !_72.0;
_63.0.fld0 = !_31.fld0;
_61.2.1 = Adt21::Variant0 { fld0: _72.0,fld1: Field::<[bool; 7]>(Variant(_38.fld4.1, 1), 4),fld2: Field::<u8>(Variant(_61.3.0, 3), 1),fld3: _38.fld3,fld4: _61.3.2,fld5: _30,fld6: 6542973390130187_i64 };
_1.3 = Field::<char>(Variant(_72.1, 2), 1);
_38.fld0 = Field::<bool>(Variant(_38.fld4.1, 1), 0);
_80 = _31.fld1 + _20;
_61.1 = [Field::<u128>(Variant(_61.2.1, 0), 4)];
match Field::<u8>(Variant(_61.2.1, 0), 2) {
0 => bb39,
100 => bb43,
_ => bb16
}
}
bb43 = {
SetDiscriminant(_72.1, 1);
_28 = !Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0;
(*_22) = Move(_56);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).1 = [7075468315366196355_i64,7926838311322244865_i64,213402937412493504_i64];
_72.1 = Adt21::Variant2 { fld0: _38.fld0,fld1: _62.1,fld2: Field::<u32>(Variant(_47, 2), 1),fld3: _44,fld4: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).2,fld5: _51 };
_40.fld0 = Adt21::Variant1 { fld0: _38.fld0,fld1: _65.0.0,fld2: Field::<u32>(Variant(_47, 2), 1),fld3: _28,fld4: Field::<[bool; 7]>(Variant(_38.fld4.1, 1), 4) };
_38.fld0 = !Field::<bool>(Variant(_40.fld0, 1), 0);
_65.0 = Move(_72);
_4 = &_12;
_61.2.1 = Adt21::Variant1 { fld0: Field::<bool>(Variant(_65.0.1, 2), 0),fld1: (*_12),fld2: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1,fld3: _34,fld4: Field::<[bool; 7]>(Variant(_40.fld0, 1), 4) };
(*_12) = Field::<u32>(Variant(_61.2.1, 1), 2) as usize;
_5 = Field::<i32>(Variant(_65.0.1, 2), 5) as isize;
_40.fld1 = [_71,_71,_71,_5,_71,_5,_71];
_19 = _25;
_38 = Adt33 { fld0: Field::<bool>(Variant(_40.fld0, 1), 0),fld1: Field::<[bool; 7]>(Variant(_40.fld0, 1), 4),fld2: _1.2,fld3: _14,fld4: Move(_61.2),fld5: _39,fld6: _70 };
_45 = !2712_i16;
_88 = _38.fld3;
_15 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5).0;
_63.0.fld2 = _28 as f64;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).2.1 = _51 as u32;
Goto(bb44)
}
bb44 = {
_20 = _44;
_81 = &_38.fld5;
_82 = Field::<u8>(Variant(_61.3.0, 3), 1) as isize;
_92.fld0 = Move(_40.fld0);
_34 = _28 * Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5).2.0;
_72.1 = Move(_65.0.1);
_63.0.fld2 = (-6260366079729937004_i64) as f64;
(*_12) = !Field::<usize>(Variant(_38.fld4.1, 1), 1);
place!(Field::<u32>(Variant(_38.fld4.1, 1), 2)) = !Field::<u32>(Variant(_72.1, 2), 2);
_76 = _25 - _5;
_65.0 = (Field::<usize>(Variant(_38.fld4.1, 1), 1), Move(_92.fld0));
_63.0.fld0 = !Field::<bool>(Variant(_72.1, 2), 0);
_69 = _5 | _76;
_47 = Adt67::Variant0 { fld0: _31,fld1: (*_16),fld2: _30 };
_73 = [_7,Field::<usize>(Variant(_65.0.1, 1), 1),(*_12),_7];
_50 = _61.3.2 << Field::<u32>(Variant(_72.1, 2), 2);
_51 = Field::<i32>(Variant(_61.3.0, 3), 0) | Field::<i32>(Variant(_72.1, 2), 5);
_94 = &_22;
_72 = (Field::<usize>(Variant(_38.fld4.1, 1), 1), Move(_38.fld4.1));
_64.0 = &_95;
_33.0 = &_95;
_92.fld1 = _40.fld1;
_88 = _14;
SetDiscriminant(_65.0.1, 1);
_40 = Adt36 { fld0: Move(_72.1),fld1: _92.fld1 };
_57 = &_31.fld2;
place!(Field::<bool>(Variant(_65.0.1, 1), 0)) = _38.fld3 <= _88;
match Field::<u8>(Variant(_61.3.0, 3), 1) {
0 => bb45,
1 => bb46,
2 => bb47,
3 => bb48,
4 => bb49,
5 => bb50,
6 => bb51,
100 => bb53,
_ => bb52
}
}
bb45 = {
SetDiscriminant(_72.1, 1);
_28 = !Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).0;
(*_22) = Move(_56);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).1 = [7075468315366196355_i64,7926838311322244865_i64,213402937412493504_i64];
_72.1 = Adt21::Variant2 { fld0: _38.fld0,fld1: _62.1,fld2: Field::<u32>(Variant(_47, 2), 1),fld3: _44,fld4: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).2,fld5: _51 };
_40.fld0 = Adt21::Variant1 { fld0: _38.fld0,fld1: _65.0.0,fld2: Field::<u32>(Variant(_47, 2), 1),fld3: _28,fld4: Field::<[bool; 7]>(Variant(_38.fld4.1, 1), 4) };
_38.fld0 = !Field::<bool>(Variant(_40.fld0, 1), 0);
_65.0 = Move(_72);
_4 = &_12;
_61.2.1 = Adt21::Variant1 { fld0: Field::<bool>(Variant(_65.0.1, 2), 0),fld1: (*_12),fld2: Field::<(i128, u32, u16, char)>(Variant(_47, 2), 3).1,fld3: _34,fld4: Field::<[bool; 7]>(Variant(_40.fld0, 1), 4) };
(*_12) = Field::<u32>(Variant(_61.2.1, 1), 2) as usize;
_5 = Field::<i32>(Variant(_65.0.1, 2), 5) as isize;
_40.fld1 = [_71,_71,_71,_5,_71,_5,_71];
_19 = _25;
_38 = Adt33 { fld0: Field::<bool>(Variant(_40.fld0, 1), 0),fld1: Field::<[bool; 7]>(Variant(_40.fld0, 1), 4),fld2: _1.2,fld3: _14,fld4: Move(_61.2),fld5: _39,fld6: _70 };
_45 = !2712_i16;
_88 = _38.fld3;
_15 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5).0;
_63.0.fld2 = _28 as f64;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).2.1 = _51 as u32;
Goto(bb44)
}
bb46 = {
_1 = (17412936098868907529691277078043773618_i128, 2651181734_u32, 42971_u16, '\u{4f954}');
_1.2 = (-25502_i16) as u16;
_1.2 = 111_u8 as u16;
_1.3 = '\u{3777b}';
_1.2 = 37938_u16;
Goto(bb2)
}
bb47 = {
_34 = _28 << _1.0;
_4 = &(*_4);
_38.fld6 = [_45,_45,_45,_45];
match _1.1 {
0 => bb5,
3374335115 => bb28,
_ => bb19
}
}
bb48 = {
_16 = core::ptr::addr_of_mut!(_45);
(*_12) = _38.fld4.0 + _38.fld4.0;
SetDiscriminant(_38.fld4.1, 2);
_4 = &(*_4);
_40.fld0 = Adt21::Variant0 { fld0: (*_12),fld1: _38.fld1,fld2: 0_u8,fld3: _14,fld4: 115310710721624515883671881683856591330_u128,fld5: _31.fld2,fld6: 2718752494293037556_i64 };
place!(Field::<i64>(Variant(_40.fld0, 0), 6)) = (-6107384337851752213_i64);
place!(Field::<u8>(Variant(_40.fld0, 0), 2)) = !21_u8;
_33.0 = &place!(Field::<i64>(Variant(_40.fld0, 0), 6));
place!(Field::<i32>(Variant(_38.fld4.1, 2), 5)) = 213800374617773674296312231855516430112_u128 as i32;
Goto(bb33)
}
bb49 = {
_7 = 4291714953701974167_usize ^ 7_usize;
_8 = -_3;
_5 = -95_isize;
_6 = 179_u8 as isize;
_3 = _8 - _8;
_5 = _6;
_1.0 = 117758243396219454145453113323481068771_i128 + (-147121641766645104648002514137601998886_i128);
_1.2 = 47690_u16 ^ 9271_u16;
_8 = _7 as f64;
_10 = true;
_7 = 18651_i16 as usize;
_1.0 = (-44847601383839168315562423967626500161_i128) >> _1.2;
_1.3 = '\u{f4a71}';
_1 = (99659841210044941296769215234232778533_i128, 3091769662_u32, 37636_u16, '\u{72595}');
_6 = _5;
_6 = !_5;
_1 = ((-120168511046275137512864090846320795979_i128), 1970275993_u32, 19869_u16, '\u{53957}');
_4 = &_12;
_7 = !0_usize;
_5 = (-30694_i16) as isize;
_1.1 = 3823498151_u32 >> _1.0;
_1.3 = '\u{2cde5}';
_4 = &(*_4);
match _1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
220113855874663325950510516585447415477 => bb6,
_ => bb5
}
}
bb50 = {
_5 = _6;
_3 = _1.1 as f64;
_1.3 = '\u{9cf6c}';
_6 = _5 >> _1.2;
_1.3 = '\u{79249}';
_5 = _6 ^ _6;
_7 = _10 as usize;
_1 = ((-136065204728609982072318487335829211104_i128), 2401731733_u32, 52373_u16, '\u{b33d9}');
_4 = &(*_4);
Goto(bb7)
}
bb51 = {
_5 = _6;
_3 = _1.1 as f64;
_1.3 = '\u{9cf6c}';
_6 = _5 >> _1.2;
_1.3 = '\u{79249}';
_5 = _6 ^ _6;
_7 = _10 as usize;
_1 = ((-136065204728609982072318487335829211104_i128), 2401731733_u32, 52373_u16, '\u{b33d9}');
_4 = &(*_4);
Goto(bb7)
}
bb52 = {
_53 = _38.fld6;
(*_22) = &(*_4);
_53 = [(*_16),(*_16),(*_16),(*_16)];
_59.0 = &_63.1;
_15 = Field::<f64>(Variant(_38.fld4.1, 0), 5);
_63.0.fld0 = _38.fld0 | _21;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_47, 2), 5)).2.1 = Field::<u8>(Variant(_38.fld4.1, 0), 2) as u32;
_38.fld0 = !_63.0.fld0;
_63.0.fld0 = !_38.fld0;
_61.3.2 = _50 >> _1.2;
_9 = &_39;
_22 = core::ptr::addr_of!(_4);
_1.2 = _61.3.2 as u16;
_63.0 = Adt40 { fld0: _31.fld0,fld1: _41,fld2: Field::<f64>(Variant(_40.fld0, 0), 5) };
_62.2 = !_23;
_40.fld1 = [_5,_5,_5,_5,_5,_5,_6];
_59.0 = &_63.1;
place!(Field::<[i32; 6]>(Variant(_47, 2), 0)) = [_51,_51,_51,_51,_51,_51];
_31.fld2 = _8;
_61.1 = [_61.3.2];
_52 = !_31.fld0;
(*_22) = &(*_4);
_55 = &_30;
_61.0 = [_38.fld5,(*_9),_23,(*_9),_38.fld5,_39];
Goto(bb37)
}
bb53 = {
_97 = !Field::<Adt40>(Variant(_47, 0), 0).fld0;
SetDiscriminant(_61.3.0, 3);
_54 = _62.1;
place!(Field::<i128>(Variant(_65.0.1, 1), 3)) = _28;
_92 = Adt36 { fld0: Move(_40.fld0),fld1: _40.fld1 };
_74 = 140_u8 | 95_u8;
_66 = _15;
_61.2 = (Field::<usize>(Variant(_92.fld0, 1), 1), Move(_92.fld0));
_72 = (Field::<usize>(Variant(_61.2.1, 1), 1), Move(_61.2.1));
place!(Field::<Adt40>(Variant(_47, 0), 0)).fld2 = -(*_57);
SetDiscriminant(_47, 1);
_96.0 = Adt35::Variant1 { fld0: _27,fld1: _1.3 };
_63.1 = core::ptr::addr_of_mut!(_39);
_68 = [_1.0,Field::<i128>(Variant(_72.1, 1), 3),_28,_1.0,Field::<i128>(Variant(_72.1, 1), 3),_28];
SetDiscriminant(_96.0, 2);
_56 = &_12;
_58 = _72.0 as f64;
_4 = &(*_4);
place!(Field::<u32>(Variant(_47, 1), 1)) = _27 << Field::<u32>(Variant(_72.1, 1), 2);
_60.fld0 = Move(_72.1);
_63.0.fld0 = _97 | Field::<bool>(Variant(_60.fld0, 1), 0);
_5 = _71;
_78 = _63.0.fld0;
SetDiscriminant(_60.fld0, 0);
_65.0.1 = Adt21::Variant2 { fld0: _97,fld1: _1.3,fld2: _1.1,fld3: _31.fld1,fld4: _1.2,fld5: _51 };
_40 = Adt36 { fld0: Move(_65.0.1),fld1: _92.fld1 };
Goto(bb54)
}
bb54 = {
_61.3.1 = &_63.1;
_60 = Move(_40);
place!(Field::<i32>(Variant(_61.3.0, 3), 0)) = _51 * _51;
SetDiscriminant(_60.fld0, 0);
place!(Field::<u8>(Variant(_61.3.0, 3), 1)) = _74 << Field::<u32>(Variant(_47, 1), 1);
_62.2 = _39 * _39;
_78 = !_97;
_96.1 = Move(_61.3.1);
_72.0 = !_65.0.0;
_42 = [_1.0,_34,_1.0,_28];
_8 = _58;
_37.0 = &_63.1;
_99.0.fld0 = _63.0.fld0 | _31.fld0;
Goto(bb55)
}
bb55 = {
_96.0 = Adt35::Variant0 { fld0: _68,fld1: _38.fld4.0,fld2: _5 };
_54 = _62.1;
_31.fld0 = !_78;
_92.fld0 = Adt21::Variant0 { fld0: _72.0,fld1: _38.fld1,fld2: Field::<u8>(Variant(_61.3.0, 3), 1),fld3: _38.fld3,fld4: _61.3.2,fld5: _66,fld6: 4233942360963885956_i64 };
_95 = 8056732745523956691_i64 << _71;
_16 = core::ptr::addr_of_mut!((*_16));
_40.fld1 = _92.fld1;
place!(Field::<u128>(Variant(_60.fld0, 0), 4)) = _41 as u128;
_38.fld1 = [_99.0.fld0,_78,_78,_63.0.fld0,_63.0.fld0,_38.fld0,_38.fld0];
RET = Adt77::Variant0 { fld0: Field::<[i128; 6]>(Variant(_96.0, 0), 0),fld1: Move(_63.1) };
_52 = !_99.0.fld0;
place!(Field::<*mut u64>(Variant(RET, 0), 1)) = core::ptr::addr_of_mut!(_102.2);
place!(Field::<i64>(Variant(_60.fld0, 0), 6)) = _95;
Goto(bb56)
}
bb56 = {
Call(_106 = dump_var(4_usize, 25_usize, Move(_25), 13_usize, Move(_13), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_106 = dump_var(4_usize, 51_usize, Move(_51), 78_usize, Move(_78), 82_usize, Move(_82), 26_usize, Move(_26)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_106 = dump_var(4_usize, 14_usize, Move(_14), 69_usize, Move(_69), 50_usize, Move(_50), 52_usize, Move(_52)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_106 = dump_var(4_usize, 32_usize, Move(_32), 54_usize, Move(_54), 6_usize, Move(_6), 88_usize, Move(_88)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_106 = dump_var(4_usize, 71_usize, Move(_71), 27_usize, Move(_27), 10_usize, Move(_10), 107_usize, _107), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u32,mut _2: u16,mut _3: u16,mut _4: u32) -> char {
mir! {
type RET = char;
let _5: bool;
let _6: (u128, &'static i128, &'static [isize; 7]);
let _7: (&'static *mut u64,);
let _8: char;
let _9: (Adt70, (usize, Adt21));
let _10: [u16; 7];
let _11: (Adt40, *mut u64);
let _12: f64;
let _13: u32;
let _14: *const u32;
let _15: u32;
let _16: i16;
let _17: u16;
let _18: Adt67;
let _19: f32;
let _20: Adt67;
let _21: char;
let _22: ((usize, *const &'static f32), &'static *mut u64, &'static f32);
let _23: char;
let _24: i32;
let _25: u32;
let _26: Adt70;
let _27: i128;
let _28: [bool; 7];
let _29: &'static u8;
let _30: u16;
let _31: i32;
let _32: [u32; 2];
let _33: isize;
let _34: char;
let _35: &'static u64;
let _36: Adt35;
let _37: i8;
let _38: [i32; 6];
let _39: ();
let _40: ();
{
RET = '\u{f321e}';
_1 = 2954600093095672264_u64 as u32;
_1 = _4 & _4;
_1 = !_4;
_4 = _1 ^ _1;
_1 = (-16682_i16) as u32;
RET = '\u{16494}';
_1 = _4;
RET = '\u{8c253}';
RET = '\u{ca60f}';
_5 = false;
RET = '\u{8ffba}';
RET = '\u{a0635}';
_2 = _3 - _3;
_1 = _4;
RET = '\u{c2ca6}';
RET = '\u{109474}';
_4 = _1 ^ _1;
_6.0 = !322370504645533241974820122496183989813_u128;
_3 = _5 as u16;
RET = '\u{335e9}';
_6.0 = 278627701392078514964845045659806450148_u128 - 157057205473220607484525435806350035794_u128;
_6.0 = 225970936923443143190778570565710849878_u128 ^ 184320782367714109601568995863420002050_u128;
RET = '\u{4e5b1}';
Goto(bb1)
}
bb1 = {
_2 = _3;
_10 = [_3,_3,_3,_2,_3,_3,_3];
_2 = !_3;
_5 = !false;
_8 = RET;
_8 = RET;
_5 = _4 > _4;
_2 = !_3;
_3 = !_2;
Goto(bb2)
}
bb2 = {
_7.0 = &_11.1;
_9.1.0 = !3_usize;
_11.0.fld0 = _5;
_12 = _2 as f64;
_5 = !_11.0.fld0;
_10 = [_3,_3,_2,_3,_3,_3,_2];
_3 = _11.0.fld0 as u16;
_10 = [_2,_3,_3,_3,_3,_3,_3];
_11.0.fld2 = -_12;
_9.1.0 = 17570628662588251349_usize * 7_usize;
Goto(bb3)
}
bb3 = {
RET = _8;
RET = _8;
RET = _8;
_13 = _4 << _1;
_11.0.fld1 = _4 as f32;
RET = _8;
_1 = !_13;
_1 = !_4;
_14 = core::ptr::addr_of!(_13);
_7.0 = &_11.1;
_3 = !_2;
_7.0 = &_11.1;
RET = _8;
RET = _8;
_2 = (-9223372036854775808_isize) as u16;
_4 = (*_14);
RET = _8;
_10 = [_3,_3,_3,_2,_2,_3,_3];
_15 = _9.1.0 as u32;
_12 = _11.0.fld2;
_2 = !_3;
_3 = !_2;
_11.0.fld2 = -_12;
_3 = 8177566910633748115_i64 as u16;
Call(_9.0 = fn6((*_14)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.0.fld2 = 9223372036854775807_isize as f64;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)).fld0 = _11.0.fld1 > _11.0.fld1;
_14 = core::ptr::addr_of!(_13);
(*_14) = _4 << _4;
_11.0.fld2 = _9.1.0 as f64;
_7.0 = &_11.1;
_11.0 = Adt40 { fld0: _5,fld1: Field::<Adt40>(Variant(_9.0, 1), 1).fld1,fld2: Field::<Adt40>(Variant(_9.0, 1), 1).fld2 };
SetDiscriminant(Field::<Adt35>(Variant(_9.0, 1), 0), 3);
_17 = !_2;
RET = _8;
_4 = (*_14);
RET = _8;
_12 = _11.0.fld2 - Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_10 = [_3,_17,_3,_17,_3,_17,_17];
place!(Field::<u8>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 1)) = 47_u8;
_11.0.fld2 = _12;
_2 = !_17;
_11.0 = Adt40 { fld0: Field::<Adt40>(Variant(_9.0, 1), 1).fld0,fld1: Field::<Adt40>(Variant(_9.0, 1), 1).fld1,fld2: _12 };
_11.0.fld2 = _12;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)).fld1 = _11.0.fld1;
place!(Field::<i32>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 0)) = !(-526802078_i32);
Call(_12 = core::intrinsics::transmute(_9.1.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = 15787_i16 << _4;
_4 = _13;
place!(Field::<u8>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 1)) = 150_u8 ^ 90_u8;
_5 = Field::<Adt40>(Variant(_9.0, 1), 1).fld0;
place!(Field::<i32>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 0)) = !476032629_i32;
_17 = _3 * _3;
_2 = !_17;
_21 = _8;
_3 = _2;
_5 = Field::<Adt40>(Variant(_9.0, 1), 1).fld0 | _11.0.fld0;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)) = Adt40 { fld0: _5,fld1: _11.0.fld1,fld2: _12 };
_21 = _8;
RET = _8;
_1 = (*_14);
_11.0.fld2 = -Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_24 = Field::<i32>(Variant(Field::<Adt35>(Variant(_9.0, 1), 0), 3), 0);
_11.0.fld0 = !Field::<Adt40>(Variant(_9.0, 1), 1).fld0;
_3 = !_17;
Goto(bb6)
}
bb6 = {
_22.2 = &_11.0.fld1;
_23 = _8;
_11.0.fld1 = Field::<Adt40>(Variant(_9.0, 1), 1).fld1;
_7.0 = &_11.1;
_2 = _3 << _4;
_16 = Field::<Adt40>(Variant(_9.0, 1), 1).fld2 as i16;
(*_14) = 9223372036854775807_isize as u32;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)) = _11.0;
_10 = [_2,_17,_2,_2,_2,_2,_2];
_22.1 = &_11.1;
_10 = [_2,_2,_17,_2,_2,_2,_2];
_11.0 = Field::<Adt40>(Variant(_9.0, 1), 1);
Goto(bb7)
}
bb7 = {
RET = _21;
_16 = 27055_i16;
RET = _21;
_22.2 = &_11.0.fld1;
_11.0 = Adt40 { fld0: _5,fld1: Field::<Adt40>(Variant(_9.0, 1), 1).fld1,fld2: _12 };
place!(Field::<Adt35>(Variant(_9.0, 1), 0)) = Adt35::Variant3 { fld0: _24,fld1: 56_u8 };
_8 = RET;
_12 = Field::<Adt40>(Variant(_9.0, 1), 1).fld2 + Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_11.0.fld1 = _2 as f32;
_7.0 = &_11.1;
_12 = Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_13 = _1;
_12 = _11.0.fld2 * Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_22.2 = &_11.0.fld1;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)).fld0 = !_11.0.fld0;
_22.1 = &_11.1;
RET = _8;
_8 = _21;
_10 = [_2,_2,_2,_17,_2,_2,_2];
place!(Field::<i32>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 0)) = _24 ^ _24;
_19 = _11.0.fld1 * _11.0.fld1;
_24 = Field::<i32>(Variant(Field::<Adt35>(Variant(_9.0, 1), 0), 3), 0);
_3 = !_2;
_7.0 = &_11.1;
_23 = _8;
_22.0.0 = !_9.1.0;
Goto(bb8)
}
bb8 = {
(*_14) = _22.0.0 as u32;
place!(Field::<u8>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 1)) = !224_u8;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)) = Adt40 { fld0: _11.0.fld0,fld1: _11.0.fld1,fld2: _12 };
_14 = core::ptr::addr_of!(_4);
place!(Field::<Adt40>(Variant(_9.0, 1), 1)).fld0 = _11.0.fld0;
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_26 = Move(_9.0);
_2 = !_3;
_24 = _11.0.fld0 as i32;
_5 = _11.0.fld0;
_22.2 = &_19;
_16 = Field::<Adt40>(Variant(_26, 1), 1).fld0 as i16;
SetDiscriminant(_26, 0);
_6.0 = 35708829582080885320929830477226053118_u128 + 27334882913758515795379193797685123453_u128;
_3 = _2 << (*_14);
RET = _8;
_27 = _5 as i128;
place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).0 = _3 as usize;
place!(Field::<[u16; 7]>(Variant(_26, 0), 0)) = [_3,_3,_3,_2,_2,_2,_3];
place!(Field::<i32>(Variant(_26, 0), 5)) = -_24;
_3 = _2;
place!(Field::<[i16; 3]>(Variant(_26, 0), 2)) = [_16,_16,_16];
_15 = 47_u8 as u32;
place!(Field::<Adt40>(Variant(_26, 0), 4)).fld2 = _16 as f64;
place!(Field::<Adt40>(Variant(_26, 0), 4)).fld0 = _11.0.fld0;
place!(Field::<u128>(Variant(_26, 0), 6)) = _6.0;
_22.2 = &place!(Field::<Adt40>(Variant(_26, 0), 4)).fld1;
RET = _23;
_11.0 = Adt40 { fld0: Field::<Adt40>(Variant(_26, 0), 4).fld0,fld1: _19,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
RET = _21;
Goto(bb10)
}
bb10 = {
_6.0 = !Field::<u128>(Variant(_26, 0), 6);
place!(Field::<Adt40>(Variant(_26, 0), 4)) = Adt40 { fld0: _5,fld1: _19,fld2: _11.0.fld2 };
_3 = _16 as u16;
_14 = core::ptr::addr_of!(_15);
Goto(bb11)
}
bb11 = {
_6.0 = Field::<u128>(Variant(_26, 0), 6);
_24 = 2_u8 as i32;
Goto(bb12)
}
bb12 = {
_21 = RET;
_16 = 7299_i16;
(*_14) = _27 as u32;
place!(Field::<[i16; 3]>(Variant(_26, 0), 2)) = [_16,_16,_16];
_3 = _2 * _2;
_11.0.fld0 = Field::<Adt40>(Variant(_26, 0), 4).fld0;
_28 = [_5,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0];
place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1 = Adt21::Variant1 { fld0: Field::<Adt40>(Variant(_26, 0), 4).fld0,fld1: Field::<(usize, Adt21)>(Variant(_26, 0), 1).0,fld2: (*_14),fld3: _27,fld4: _28 };
SetDiscriminant(Field::<(usize, Adt21)>(Variant(_26, 0), 1).1, 1);
place!(Field::<usize>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 1)) = _5 as usize;
_3 = _2;
place!(Field::<i128>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 3)) = !_27;
_11.0 = Adt40 { fld0: _5,fld1: Field::<Adt40>(Variant(_26, 0), 4).fld1,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
_22.0.1 = core::ptr::addr_of!(_22.2);
_20 = Adt67::Variant0 { fld0: _11.0,fld1: _16,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
Call(place!(Field::<[u16; 7]>(Variant(_26, 0), 0)) = core::intrinsics::transmute(_10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<[u16; 7]>(Variant(_26, 0), 0)) = [_2,_3,_3,_3,_2,_2,_3];
_22.2 = &_11.0.fld1;
_24 = Field::<i32>(Variant(_26, 0), 5);
_6.1 = &_27;
SetDiscriminant(_20, 0);
place!(Field::<Adt40>(Variant(_26, 0), 4)) = Adt40 { fld0: _11.0.fld0,fld1: _19,fld2: _11.0.fld2 };
place!(Field::<Adt40>(Variant(_20, 0), 0)).fld1 = _19;
_7.0 = &_11.1;
place!(Field::<[bool; 7]>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 4)) = [_11.0.fld0,_11.0.fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,_11.0.fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,_11.0.fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0];
match _16 {
0 => bb11,
1 => bb8,
2 => bb14,
3 => bb15,
4 => bb16,
7299 => bb18,
_ => bb17
}
}
bb14 = {
_21 = RET;
_16 = 7299_i16;
(*_14) = _27 as u32;
place!(Field::<[i16; 3]>(Variant(_26, 0), 2)) = [_16,_16,_16];
_3 = _2 * _2;
_11.0.fld0 = Field::<Adt40>(Variant(_26, 0), 4).fld0;
_28 = [_5,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0,Field::<Adt40>(Variant(_26, 0), 4).fld0];
place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1 = Adt21::Variant1 { fld0: Field::<Adt40>(Variant(_26, 0), 4).fld0,fld1: Field::<(usize, Adt21)>(Variant(_26, 0), 1).0,fld2: (*_14),fld3: _27,fld4: _28 };
SetDiscriminant(Field::<(usize, Adt21)>(Variant(_26, 0), 1).1, 1);
place!(Field::<usize>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 1)) = _5 as usize;
_3 = _2;
place!(Field::<i128>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 3)) = !_27;
_11.0 = Adt40 { fld0: _5,fld1: Field::<Adt40>(Variant(_26, 0), 4).fld1,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
_22.0.1 = core::ptr::addr_of!(_22.2);
_20 = Adt67::Variant0 { fld0: _11.0,fld1: _16,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
Call(place!(Field::<[u16; 7]>(Variant(_26, 0), 0)) = core::intrinsics::transmute(_10), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
_6.0 = Field::<u128>(Variant(_26, 0), 6);
_24 = 2_u8 as i32;
Goto(bb12)
}
bb16 = {
RET = _21;
_16 = 27055_i16;
RET = _21;
_22.2 = &_11.0.fld1;
_11.0 = Adt40 { fld0: _5,fld1: Field::<Adt40>(Variant(_9.0, 1), 1).fld1,fld2: _12 };
place!(Field::<Adt35>(Variant(_9.0, 1), 0)) = Adt35::Variant3 { fld0: _24,fld1: 56_u8 };
_8 = RET;
_12 = Field::<Adt40>(Variant(_9.0, 1), 1).fld2 + Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_11.0.fld1 = _2 as f32;
_7.0 = &_11.1;
_12 = Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_13 = _1;
_12 = _11.0.fld2 * Field::<Adt40>(Variant(_9.0, 1), 1).fld2;
_22.2 = &_11.0.fld1;
place!(Field::<Adt40>(Variant(_9.0, 1), 1)).fld0 = !_11.0.fld0;
_22.1 = &_11.1;
RET = _8;
_8 = _21;
_10 = [_2,_2,_2,_17,_2,_2,_2];
place!(Field::<i32>(Variant(place!(Field::<Adt35>(Variant(_9.0, 1), 0)), 3), 0)) = _24 ^ _24;
_19 = _11.0.fld1 * _11.0.fld1;
_24 = Field::<i32>(Variant(Field::<Adt35>(Variant(_9.0, 1), 0), 3), 0);
_3 = !_2;
_7.0 = &_11.1;
_23 = _8;
_22.0.0 = !_9.1.0;
Goto(bb8)
}
bb17 = {
_7.0 = &_11.1;
_9.1.0 = !3_usize;
_11.0.fld0 = _5;
_12 = _2 as f64;
_5 = !_11.0.fld0;
_10 = [_3,_3,_2,_3,_3,_3,_2];
_3 = _11.0.fld0 as u16;
_10 = [_2,_3,_3,_3,_3,_3,_3];
_11.0.fld2 = -_12;
_9.1.0 = 17570628662588251349_usize * 7_usize;
Goto(bb3)
}
bb18 = {
place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1 = Adt21::Variant1 { fld0: Field::<Adt40>(Variant(_26, 0), 4).fld0,fld1: Field::<(usize, Adt21)>(Variant(_26, 0), 1).0,fld2: _15,fld3: _27,fld4: _28 };
_6.1 = &place!(Field::<i128>(Variant(place!(Field::<(usize, Adt21)>(Variant(_26, 0), 1)).1, 1), 3));
place!(Field::<*mut usize>(Variant(_26, 0), 7)) = core::ptr::addr_of_mut!(_9.1.0);
place!(Field::<[i16; 3]>(Variant(_26, 0), 2)) = [_16,_16,_16];
_9.1.1 = Move(Field::<(usize, Adt21)>(Variant(_26, 0), 1).1);
_5 = Field::<Adt40>(Variant(_26, 0), 4).fld0 ^ Field::<bool>(Variant(_9.1.1, 1), 0);
_11.0.fld2 = Field::<Adt40>(Variant(_26, 0), 4).fld2 + Field::<Adt40>(Variant(_26, 0), 4).fld2;
place!(Field::<f64>(Variant(_20, 0), 2)) = -_11.0.fld2;
_24 = Field::<i32>(Variant(_26, 0), 5);
place!(Field::<i16>(Variant(_20, 0), 1)) = _16;
_5 = Field::<Adt40>(Variant(_26, 0), 4).fld1 == Field::<Adt40>(Variant(_26, 0), 4).fld1;
_30 = !_2;
place!(Field::<Adt40>(Variant(_20, 0), 0)).fld0 = !Field::<bool>(Variant(_9.1.1, 1), 0);
place!(Field::<[bool; 7]>(Variant(_9.1.1, 1), 4)) = _28;
_32 = [_4,(*_14)];
_11.0 = Adt40 { fld0: Field::<Adt40>(Variant(_26, 0), 4).fld0,fld1: Field::<Adt40>(Variant(_26, 0), 4).fld1,fld2: Field::<Adt40>(Variant(_26, 0), 4).fld2 };
_22.0.1 = core::ptr::addr_of!(_22.2);
SetDiscriminant(_9.1.1, 2);
Goto(bb19)
}
bb19 = {
Call(_39 = dump_var(5_usize, 24_usize, Move(_24), 13_usize, Move(_13), 23_usize, Move(_23), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(5_usize, 5_usize, Move(_5), 21_usize, Move(_21), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(5_usize, 32_usize, Move(_32), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: u32) -> Adt70 {
mir! {
type RET = Adt70;
let _2: (u128, &'static i128, &'static [isize; 7]);
let _3: &'static u8;
let _4: (&'static *mut u64,);
let _5: &'static f32;
let _6: char;
let _7: u8;
let _8: isize;
let _9: (Adt35, &'static *mut u64, u128);
let _10: (&'static f32, [i16; 1], *mut u64, &'static *mut u64);
let _11: isize;
let _12: (Adt35, &'static *mut u64, u128);
let _13: [u64; 6];
let _14: [isize; 7];
let _15: f64;
let _16: i128;
let _17: isize;
let _18: Adt36;
let _19: ([u64; 6], [u128; 1], (usize, Adt21), (Adt35, &'static *mut u64, u128));
let _20: Adt67;
let _21: ((usize, Adt21),);
let _22: &'static i64;
let _23: bool;
let _24: (u8, u64);
let _25: [bool; 7];
let _26: (&'static [i16; 1], char, u64);
let _27: &'static [i16; 1];
let _28: bool;
let _29: char;
let _30: i64;
let _31: char;
let _32: usize;
let _33: isize;
let _34: [usize; 1];
let _35: char;
let _36: isize;
let _37: isize;
let _38: [bool; 7];
let _39: Adt77;
let _40: f32;
let _41: f64;
let _42: &'static f32;
let _43: (usize, Adt21);
let _44: [u64; 6];
let _45: f32;
let _46: f32;
let _47: bool;
let _48: &'static [i16; 1];
let _49: &'static [usize; 4];
let _50: f64;
let _51: [u64; 4];
let _52: isize;
let _53: (Adt40, *mut u64);
let _54: (f64, [i64; 3], (i128, u32, u16, char));
let _55: [i16; 3];
let _56: &'static *mut usize;
let _57: [isize; 7];
let _58: [u64; 4];
let _59: (&'static *mut u64,);
let _60: isize;
let _61: [i128; 6];
let _62: *mut i8;
let _63: (Adt35, &'static *mut u64, u128);
let _64: (&'static *mut u64,);
let _65: [i128; 6];
let _66: isize;
let _67: ();
let _68: ();
{
_1 = 2837496160_u32 >> 158064509098268417268368804714343620721_u128;
_1 = 187061143_u32 >> 303477112829963397041691555866882784882_u128;
_1 = !2452848521_u32;
_1 = !2083388445_u32;
_1 = 41_u8 as u32;
_2.0 = 166013161836167122526713735604304464420_u128;
_2.0 = 190950189913487350974382890780722353129_u128 << _1;
_1 = !1651352966_u32;
_1 = !3905990627_u32;
_2.0 = !55137312886225840788306750799260003578_u128;
_2.0 = !56758775086775713276889377992513261575_u128;
_1 = 291455543_u32 - 389199858_u32;
_1 = 4077823192_u32;
_1 = (-88230972953912132845294188432475359092_i128) as u32;
_2.0 = !54602007143505766982442235702218452825_u128;
_1 = 780506676_u32 & 2339507539_u32;
_2.0 = 125144618055186154367356619596678568547_u128 >> _1;
_2.0 = !317665583247427728384919118854828225805_u128;
_1 = 813109622_u32 + 2965039957_u32;
_2.0 = 177_u8 as u128;
_2.0 = 159970382296685378235431844886468397292_u128 << _1;
_1 = 104967084_u32;
_1 = 4898927713376634346_i64 as u32;
Goto(bb1)
}
bb1 = {
_2.0 = 131525049359996344566613458349035413143_u128;
_1 = 3273755805_u32 - 1305258743_u32;
_6 = '\u{5878f}';
_1 = 970308197_u32 >> _2.0;
_1 = 45131_u16 as u32;
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
131525049359996344566613458349035413143 => bb6,
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
_6 = '\u{ad1e0}';
_1 = 17172373726343216241777017895503535321_i128 as u32;
_1 = !3999977818_u32;
_3 = &_7;
_2.0 = 293892279759032743768780947200287049913_u128;
_3 = &(*_3);
_3 = &_7;
_7 = 52_u8;
_10.1 = [20387_i16];
_3 = &_7;
Call(_2.0 = fn7(Move(_3), _7, _10.1, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_12.2 = _2.0;
_12.2 = _2.0;
_12.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
place!(Field::<char>(Variant(_12.0, 1), 1)) = _6;
_9.0 = Move(_12.0);
_2.0 = !_12.2;
_9.0 = Adt35::Variant3 { fld0: (-2144627300_i32),fld1: _7 };
_6 = '\u{bb386}';
_3 = &_7;
_13 = [14726561158536460776_u64,13370908390379925468_u64,12478130535848357240_u64,13524629178597968849_u64,4284006170678137603_u64,10710131250611839018_u64];
_11 = (-9223372036854775808_isize) - (-78_isize);
_12.1 = &_10.2;
_3 = &place!(Field::<u8>(Variant(_9.0, 3), 1));
_8 = _11;
place!(Field::<i32>(Variant(_9.0, 3), 0)) = (-1864986285_i32);
_8 = _1 as isize;
_2.1 = &_16;
_12.0 = Move(_9.0);
_16 = !94699151113353656511731402710773673345_i128;
_14 = [_8,_8,_11,_11,_11,_11,_11];
_1 = !1329682695_u32;
_12.1 = &_10.2;
SetDiscriminant(_12.0, 0);
_6 = '\u{fc4d9}';
_14 = [_11,_11,_8,_11,_8,_11,_11];
Call(_12.0 = fn12(_7, _8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2.1 = &_16;
_17 = _8;
_4.0 = &_10.2;
_9.2 = 0_usize as u128;
_17 = -_11;
_15 = _12.2 as f64;
_1 = !1639089385_u32;
_2.2 = &_14;
_13 = [7516337972640531220_u64,8745106171987706218_u64,12268875433477710751_u64,1838033674192582853_u64,8966328630077272496_u64,14018604556873663041_u64];
_16 = 145783594432114661275993642675739928686_i128 >> Field::<u8>(Variant(_12.0, 3), 1);
_15 = _9.2 as f64;
_14 = [_17,_11,_8,_11,_17,_17,_17];
_2.1 = &_16;
_3 = &place!(Field::<u8>(Variant(_12.0, 3), 1));
_14 = [_17,_11,_17,_11,_11,_11,_17];
_19.1 = [_12.2];
_9.2 = _2.0;
_19.2.0 = 10499698542425384965_usize;
Goto(bb9)
}
bb9 = {
_17 = _11;
_19.0 = [14712920256487341627_u64,1013766196008615261_u64,9652682394012183311_u64,1372104885274099617_u64,12602863203832726461_u64,14633375267071566048_u64];
place!(Field::<i32>(Variant(_12.0, 3), 0)) = -1867242326_i32;
_12.0 = Adt35::Variant2 { fld0: 7385391251711570884_u64,fld1: 2410543548051872835_i64,fld2: _15 };
_4.0 = &_10.2;
_9.0 = Adt35::Variant2 { fld0: 17406716186841791466_u64,fld1: (-2278564254140940492_i64),fld2: _15 };
_3 = &_7;
_21.0.0 = 5521716147564971894_i64 as usize;
_10.2 = core::ptr::addr_of_mut!(place!(Field::<u64>(Variant(_12.0, 2), 0)));
_17 = _8;
_16 = _19.2.0 as i128;
match (*_3) {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb8,
52 => bb10,
_ => bb7
}
}
bb10 = {
_19.1 = [_12.2];
_16 = 41665240933781153210144262725746086292_i128 - (-38436558073656990962508194072820562377_i128);
_7 = 129_u8 + 38_u8;
_13 = [13816820101537980233_u64,5221619429259872228_u64,9210875693656141825_u64,6818453555638374542_u64,12232633582815845400_u64,8093024071019069287_u64];
_13 = _19.0;
_4.0 = &_10.2;
_18.fld1 = [_11,_17,_11,_11,_17,_11,_8];
_12.0 = Adt35::Variant3 { fld0: (-1813252992_i32),fld1: _7 };
_12.2 = _9.2;
_12.2 = (-2000269440_i32) as u128;
_19.2.0 = _21.0.0;
_24.1 = 10086917384994431563_u64;
_19.3.2 = !_9.2;
_19.3.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
Goto(bb11)
}
bb11 = {
_2.1 = &_16;
_9.1 = &_10.2;
_11 = _17 - _8;
_8 = _11;
_19.1 = [_12.2];
_15 = Field::<f64>(Variant(_9.0, 2), 2);
place!(Field::<i64>(Variant(_9.0, 2), 1)) = !8413301779026584789_i64;
_19.0 = _13;
_2.2 = &_18.fld1;
_21.0.0 = 309891731_i32 as usize;
place!(Field::<u64>(Variant(_9.0, 2), 0)) = Field::<f64>(Variant(_9.0, 2), 2) as u64;
_2.2 = &_18.fld1;
_6 = Field::<char>(Variant(_19.3.0, 1), 1);
_24.0 = (-444417466_i32) as u8;
_12.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
_21.0.0 = _15 as usize;
place!(Field::<u32>(Variant(_12.0, 1), 0)) = _1 - _1;
_19.2.0 = !_21.0.0;
Goto(bb12)
}
bb12 = {
_20 = Adt67::Variant1 { fld0: _18.fld1,fld1: Field::<u32>(Variant(_19.3.0, 1), 0) };
_16 = !91582130387039785457380508210447498749_i128;
_2.2 = &place!(Field::<[isize; 7]>(Variant(_20, 1), 0));
_23 = !false;
_26.1 = _6;
_14 = _18.fld1;
match _24.1 {
0 => bb1,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
10086917384994431563 => bb19,
_ => bb18
}
}
bb13 = {
_2.0 = 131525049359996344566613458349035413143_u128;
_1 = 3273755805_u32 - 1305258743_u32;
_6 = '\u{5878f}';
_1 = 970308197_u32 >> _2.0;
_1 = 45131_u16 as u32;
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
131525049359996344566613458349035413143 => bb6,
_ => bb5
}
}
bb14 = {
_19.1 = [_12.2];
_16 = 41665240933781153210144262725746086292_i128 - (-38436558073656990962508194072820562377_i128);
_7 = 129_u8 + 38_u8;
_13 = [13816820101537980233_u64,5221619429259872228_u64,9210875693656141825_u64,6818453555638374542_u64,12232633582815845400_u64,8093024071019069287_u64];
_13 = _19.0;
_4.0 = &_10.2;
_18.fld1 = [_11,_17,_11,_11,_17,_11,_8];
_12.0 = Adt35::Variant3 { fld0: (-1813252992_i32),fld1: _7 };
_12.2 = _9.2;
_12.2 = (-2000269440_i32) as u128;
_19.2.0 = _21.0.0;
_24.1 = 10086917384994431563_u64;
_19.3.2 = !_9.2;
_19.3.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
Goto(bb11)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_12.2 = _2.0;
_12.2 = _2.0;
_12.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
place!(Field::<char>(Variant(_12.0, 1), 1)) = _6;
_9.0 = Move(_12.0);
_2.0 = !_12.2;
_9.0 = Adt35::Variant3 { fld0: (-2144627300_i32),fld1: _7 };
_6 = '\u{bb386}';
_3 = &_7;
_13 = [14726561158536460776_u64,13370908390379925468_u64,12478130535848357240_u64,13524629178597968849_u64,4284006170678137603_u64,10710131250611839018_u64];
_11 = (-9223372036854775808_isize) - (-78_isize);
_12.1 = &_10.2;
_3 = &place!(Field::<u8>(Variant(_9.0, 3), 1));
_8 = _11;
place!(Field::<i32>(Variant(_9.0, 3), 0)) = (-1864986285_i32);
_8 = _1 as isize;
_2.1 = &_16;
_12.0 = Move(_9.0);
_16 = !94699151113353656511731402710773673345_i128;
_14 = [_8,_8,_11,_11,_11,_11,_11];
_1 = !1329682695_u32;
_12.1 = &_10.2;
SetDiscriminant(_12.0, 0);
_6 = '\u{fc4d9}';
_14 = [_11,_11,_8,_11,_8,_11,_11];
Call(_12.0 = fn12(_7, _8), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_6 = '\u{ad1e0}';
_1 = 17172373726343216241777017895503535321_i128 as u32;
_1 = !3999977818_u32;
_3 = &_7;
_2.0 = 293892279759032743768780947200287049913_u128;
_3 = &(*_3);
_3 = &_7;
_7 = 52_u8;
_10.1 = [20387_i16];
_3 = &_7;
Call(_2.0 = fn7(Move(_3), _7, _10.1, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb19 = {
_10.3 = Move(_9.1);
_3 = &_7;
_2.1 = &_16;
_19.0 = [Field::<u64>(Variant(_9.0, 2), 0),Field::<u64>(Variant(_9.0, 2), 0),_24.1,_24.1,_24.1,Field::<u64>(Variant(_9.0, 2), 0)];
_25 = [_23,_23,_23,_23,_23,_23,_23];
_12.1 = Move(_4.0);
_10.3 = &_10.2;
_7 = (-28304_i16) as u8;
place!(Field::<u32>(Variant(_12.0, 1), 0)) = Field::<u32>(Variant(_20, 1), 1);
_15 = Field::<f64>(Variant(_9.0, 2), 2);
_12.1 = Move(_10.3);
Goto(bb20)
}
bb20 = {
place!(Field::<u32>(Variant(_20, 1), 1)) = Field::<u32>(Variant(_19.3.0, 1), 0);
_19.3.1 = &_10.2;
_4.0 = &_10.2;
_21.0.1 = Adt21::Variant1 { fld0: _23,fld1: _21.0.0,fld2: Field::<u32>(Variant(_19.3.0, 1), 0),fld3: _16,fld4: _25 };
_9 = (Move(_19.3.0), Move(_4.0), _2.0);
_22 = &_30;
SetDiscriminant(_12.0, 1);
_15 = Field::<u32>(Variant(_20, 1), 1) as f64;
place!(Field::<u32>(Variant(_12.0, 1), 0)) = Field::<u32>(Variant(_20, 1), 1) >> _21.0.0;
_22 = &(*_22);
_14 = _18.fld1;
place!(Field::<usize>(Variant(_21.0.1, 1), 1)) = _19.2.0;
Goto(bb21)
}
bb21 = {
_8 = _19.2.0 as isize;
place!(Field::<u32>(Variant(_21.0.1, 1), 2)) = Field::<u32>(Variant(_20, 1), 1) * _1;
SetDiscriminant(_21.0.1, 1);
_9.1 = Move(_19.3.1);
place!(Field::<[bool; 7]>(Variant(_21.0.1, 1), 4)) = [_23,_23,_23,_23,_23,_23,_23];
place!(Field::<u32>(Variant(_9.0, 1), 0)) = Field::<u32>(Variant(_12.0, 1), 0);
place!(Field::<char>(Variant(_12.0, 1), 1)) = _26.1;
place!(Field::<[isize; 7]>(Variant(_20, 1), 0)) = [_11,_11,_17,_11,_17,_17,_11];
_18.fld1 = [_11,_11,_17,_17,_8,_11,_8];
_28 = _2.0 < _19.3.2;
_32 = !_21.0.0;
_31 = Field::<char>(Variant(_12.0, 1), 1);
_24 = (_7, 1209523962364674097_u64);
_18.fld1 = Field::<[isize; 7]>(Variant(_20, 1), 0);
place!(Field::<bool>(Variant(_21.0.1, 1), 0)) = _28;
_10.3 = &_10.2;
_19.3.1 = &_10.2;
_22 = &(*_22);
_9.2 = 47_i8 as u128;
_27 = &_10.1;
Goto(bb22)
}
bb22 = {
_3 = &_24.0;
_10.3 = Move(_19.3.1);
place!(Field::<u32>(Variant(_9.0, 1), 0)) = Field::<u32>(Variant(_12.0, 1), 0) & _1;
place!(Field::<bool>(Variant(_21.0.1, 1), 0)) = !_23;
_36 = !_11;
Goto(bb23)
}
bb23 = {
_12.1 = &_10.2;
_10.1 = [28738_i16];
_32 = _28 as usize;
_13 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_3 = &(*_3);
_9.1 = Move(_12.1);
place!(Field::<char>(Variant(_9.0, 1), 1)) = _26.1;
_9.2 = !_19.3.2;
place!(Field::<i128>(Variant(_21.0.1, 1), 3)) = _16 ^ _16;
_19.3.2 = _12.2;
_9.0 = Move(_12.0);
place!(Field::<bool>(Variant(_21.0.1, 1), 0)) = _28;
_4.0 = &_10.2;
_26.0 = &_10.1;
_9.2 = !_19.3.2;
_33 = _8 & _11;
_41 = -_15;
place!(Field::<u32>(Variant(_21.0.1, 1), 2)) = _1 - Field::<u32>(Variant(_9.0, 1), 0);
Goto(bb24)
}
bb24 = {
SetDiscriminant(_9.0, 1);
_4.0 = &_10.2;
_26.2 = _24.1;
_26.0 = &_10.1;
_10.0 = &_40;
_5 = &_40;
_34 = [_21.0.0];
match _24.1 {
1209523962364674097 => bb25,
_ => bb20
}
}
bb25 = {
_10.1 = [6186_i16];
_19.2.0 = _32 + _32;
_4.0 = &_10.2;
_14 = [_17,_36,_11,_36,_11,_33,_36];
match _26.2 {
0 => bb8,
1 => bb26,
2 => bb27,
3 => bb28,
4 => bb29,
5 => bb30,
1209523962364674097 => bb32,
_ => bb31
}
}
bb26 = {
Return()
}
bb27 = {
_6 = '\u{ad1e0}';
_1 = 17172373726343216241777017895503535321_i128 as u32;
_1 = !3999977818_u32;
_3 = &_7;
_2.0 = 293892279759032743768780947200287049913_u128;
_3 = &(*_3);
_3 = &_7;
_7 = 52_u8;
_10.1 = [20387_i16];
_3 = &_7;
Call(_2.0 = fn7(Move(_3), _7, _10.1, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb28 = {
_3 = &_24.0;
_10.3 = Move(_19.3.1);
place!(Field::<u32>(Variant(_9.0, 1), 0)) = Field::<u32>(Variant(_12.0, 1), 0) & _1;
place!(Field::<bool>(Variant(_21.0.1, 1), 0)) = !_23;
_36 = !_11;
Goto(bb23)
}
bb29 = {
Return()
}
bb30 = {
_12.2 = _2.0;
_12.2 = _2.0;
_12.0 = Adt35::Variant1 { fld0: _1,fld1: _6 };
place!(Field::<char>(Variant(_12.0, 1), 1)) = _6;
_9.0 = Move(_12.0);
_2.0 = !_12.2;
_9.0 = Adt35::Variant3 { fld0: (-2144627300_i32),fld1: _7 };
_6 = '\u{bb386}';
_3 = &_7;
_13 = [14726561158536460776_u64,13370908390379925468_u64,12478130535848357240_u64,13524629178597968849_u64,4284006170678137603_u64,10710131250611839018_u64];
_11 = (-9223372036854775808_isize) - (-78_isize);
_12.1 = &_10.2;
_3 = &place!(Field::<u8>(Variant(_9.0, 3), 1));
_8 = _11;
place!(Field::<i32>(Variant(_9.0, 3), 0)) = (-1864986285_i32);
_8 = _1 as isize;
_2.1 = &_16;
_12.0 = Move(_9.0);
_16 = !94699151113353656511731402710773673345_i128;
_14 = [_8,_8,_11,_11,_11,_11,_11];
_1 = !1329682695_u32;
_12.1 = &_10.2;
SetDiscriminant(_12.0, 0);
_6 = '\u{fc4d9}';
_14 = [_11,_11,_8,_11,_8,_11,_11];
Call(_12.0 = fn12(_7, _8), ReturnTo(bb8), UnwindUnreachable())
}
bb31 = {
Return()
}
bb32 = {
_29 = _26.1;
place!(Field::<u32>(Variant(_21.0.1, 1), 2)) = !Field::<u32>(Variant(_20, 1), 1);
_16 = Field::<i128>(Variant(_21.0.1, 1), 3) + Field::<i128>(Variant(_21.0.1, 1), 3);
_2.1 = &place!(Field::<i128>(Variant(_21.0.1, 1), 3));
_26.0 = &_10.1;
_11 = _21.0.0 as isize;
_12.1 = &_10.2;
_19.0 = [_26.2,_24.1,_24.1,_24.1,_24.1,_24.1];
_18.fld0 = Adt21::Variant0 { fld0: _21.0.0,fld1: _25,fld2: (*_3),fld3: (-125_i8),fld4: _2.0,fld5: _15,fld6: (-8167082379277847806_i64) };
_35 = _31;
place!(Field::<char>(Variant(_9.0, 1), 1)) = _6;
place!(Field::<f64>(Variant(_18.fld0, 0), 5)) = 1484189882_i32 as f64;
Call(_18.fld1 = core::intrinsics::transmute(Field::<[isize; 7]>(Variant(_20, 1), 0)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_40 = Field::<u32>(Variant(_20, 1), 1) as f32;
_26.2 = !_24.1;
place!(Field::<[bool; 7]>(Variant(_18.fld0, 0), 1)) = [_28,_23,Field::<bool>(Variant(_21.0.1, 1), 0),Field::<bool>(Variant(_21.0.1, 1), 0),_28,_28,_23];
_4 = (Move(_12.1),);
_15 = _41 * Field::<f64>(Variant(_18.fld0, 0), 5);
_41 = _15;
place!(Field::<u32>(Variant(_9.0, 1), 0)) = !Field::<u32>(Variant(_21.0.1, 1), 2);
_41 = 30954_i16 as f64;
_15 = _41;
Goto(bb34)
}
bb34 = {
_2.2 = &_14;
place!(Field::<u128>(Variant(_18.fld0, 0), 4)) = _9.2;
place!(Field::<i64>(Variant(_18.fld0, 0), 6)) = !5480640951302148711_i64;
_11 = _19.2.0 as isize;
SetDiscriminant(_9.0, 1);
place!(Field::<usize>(Variant(_21.0.1, 1), 1)) = Field::<f64>(Variant(_18.fld0, 0), 5) as usize;
_20 = Adt67::Variant1 { fld0: _14,fld1: _1 };
Goto(bb35)
}
bb35 = {
place!(Field::<u32>(Variant(_9.0, 1), 0)) = _1;
place!(Field::<u8>(Variant(_18.fld0, 0), 2)) = _24.0;
SetDiscriminant(_20, 1);
_19.0 = _13;
_9.1 = &_10.2;
_43 = (_32, Move(_21.0.1));
_4.0 = &_10.2;
_12.1 = Move(_9.1);
_25 = Field::<[bool; 7]>(Variant(_18.fld0, 0), 1);
_9.1 = &_10.2;
place!(Field::<u32>(Variant(_20, 1), 1)) = !Field::<u32>(Variant(_9.0, 1), 0);
_19.3.1 = &_10.2;
_22 = &_30;
_50 = _16 as f64;
_18.fld0 = Adt21::Variant2 { fld0: _23,fld1: _6,fld2: Field::<u32>(Variant(_43.1, 1), 2),fld3: _40,fld4: 24430_u16,fld5: (-2130455533_i32) };
_22 = &(*_22);
place!(Field::<u32>(Variant(_20, 1), 1)) = !Field::<u32>(Variant(_43.1, 1), 2);
Goto(bb36)
}
bb36 = {
_18.fld0 = Adt21::Variant2 { fld0: Field::<bool>(Variant(_43.1, 1), 0),fld1: _35,fld2: Field::<u32>(Variant(_43.1, 1), 2),fld3: _40,fld4: 27223_u16,fld5: 1918305230_i32 };
SetDiscriminant(_43.1, 1);
_19.3.1 = &_10.2;
_12.1 = Move(_9.1);
place!(Field::<[bool; 7]>(Variant(_43.1, 1), 4)) = [Field::<bool>(Variant(_18.fld0, 2), 0),_28,_23,Field::<bool>(Variant(_18.fld0, 2), 0),Field::<bool>(Variant(_18.fld0, 2), 0),_28,Field::<bool>(Variant(_18.fld0, 2), 0)];
_12.2 = (-1433366569_i32) as u128;
_25 = [Field::<bool>(Variant(_18.fld0, 2), 0),_28,Field::<bool>(Variant(_18.fld0, 2), 0),_23,_28,Field::<bool>(Variant(_18.fld0, 2), 0),Field::<bool>(Variant(_18.fld0, 2), 0)];
_2.1 = &_16;
place!(Field::<u32>(Variant(_18.fld0, 2), 2)) = Field::<u32>(Variant(_20, 1), 1);
_10.0 = &_46;
Goto(bb37)
}
bb37 = {
_34 = [_21.0.0];
_53.0.fld2 = 16564_i16 as f64;
_15 = _50 + _53.0.fld2;
_21.0.1 = Adt21::Variant0 { fld0: _19.2.0,fld1: Field::<[bool; 7]>(Variant(_43.1, 1), 4),fld2: (*_3),fld3: 56_i8,fld4: _2.0,fld5: _15,fld6: (-7596541132736909313_i64) };
_36 = _11 ^ _33;
_16 = (-17036643915409002697750300979274389342_i128) | 81712243410884745089000525136821232028_i128;
Goto(bb38)
}
bb38 = {
_37 = _36;
_19.2.1 = Adt21::Variant2 { fld0: _28,fld1: _35,fld2: Field::<u32>(Variant(_9.0, 1), 0),fld3: Field::<f32>(Variant(_18.fld0, 2), 3),fld4: 33366_u16,fld5: (-1384433448_i32) };
Goto(bb39)
}
bb39 = {
_19.1 = [_12.2];
_27 = Move(_26.0);
_47 = Field::<u128>(Variant(_21.0.1, 0), 4) < Field::<u128>(Variant(_21.0.1, 0), 4);
_42 = &place!(Field::<f32>(Variant(_18.fld0, 2), 3));
_31 = Field::<char>(Variant(_18.fld0, 2), 1);
_26.0 = &_10.1;
place!(Field::<u32>(Variant(_20, 1), 1)) = Field::<u32>(Variant(_19.2.1, 2), 2) + _1;
_12.2 = _9.2;
place!(Field::<char>(Variant(_18.fld0, 2), 1)) = _29;
_38 = [_28,_47,Field::<bool>(Variant(_18.fld0, 2), 0),Field::<bool>(Variant(_19.2.1, 2), 0),Field::<bool>(Variant(_18.fld0, 2), 0),_28,_47];
_44 = _19.0;
place!(Field::<i32>(Variant(_19.2.1, 2), 5)) = !(-306392204_i32);
_54.2 = (_16, Field::<u32>(Variant(_20, 1), 1), 51623_u16, _31);
_54.2.3 = _6;
_46 = -Field::<f32>(Variant(_19.2.1, 2), 3);
place!(Field::<[bool; 7]>(Variant(_43.1, 1), 4)) = [_47,_28,Field::<bool>(Variant(_19.2.1, 2), 0),_47,_47,_47,_47];
_16 = !_54.2.0;
_38 = [_47,Field::<bool>(Variant(_19.2.1, 2), 0),_28,_23,_28,_28,Field::<bool>(Variant(_18.fld0, 2), 0)];
place!(Field::<i128>(Variant(_43.1, 1), 3)) = -_54.2.0;
place!(Field::<u32>(Variant(_9.0, 1), 0)) = !Field::<u32>(Variant(_19.2.1, 2), 2);
_52 = !_37;
place!(Field::<u128>(Variant(_21.0.1, 0), 4)) = _19.3.2;
match _54.2.2 {
0 => bb29,
1 => bb17,
2 => bb16,
3 => bb7,
4 => bb40,
51623 => bb42,
_ => bb41
}
}
bb40 = {
_2.2 = &_14;
place!(Field::<u128>(Variant(_18.fld0, 0), 4)) = _9.2;
place!(Field::<i64>(Variant(_18.fld0, 0), 6)) = !5480640951302148711_i64;
_11 = _19.2.0 as isize;
SetDiscriminant(_9.0, 1);
place!(Field::<usize>(Variant(_21.0.1, 1), 1)) = Field::<f64>(Variant(_18.fld0, 0), 5) as usize;
_20 = Adt67::Variant1 { fld0: _14,fld1: _1 };
Goto(bb35)
}
bb41 = {
Return()
}
bb42 = {
_19.3.0 = Adt35::Variant1 { fld0: Field::<u32>(Variant(_18.fld0, 2), 2),fld1: _31 };
place!(Field::<[bool; 7]>(Variant(_21.0.1, 0), 1)) = Field::<[bool; 7]>(Variant(_43.1, 1), 4);
place!(Field::<u32>(Variant(_43.1, 1), 2)) = !Field::<u32>(Variant(_19.2.1, 2), 2);
_29 = _54.2.3;
place!(Field::<[bool; 7]>(Variant(_21.0.1, 0), 1)) = _25;
place!(Field::<[isize; 7]>(Variant(_20, 1), 0)) = _18.fld1;
_19.1 = [_2.0];
_27 = Move(_26.0);
_53.0.fld0 = _47;
_6 = Field::<char>(Variant(_18.fld0, 2), 1);
_53.0.fld1 = _7 as f32;
Call(_30 = fn14(Move(_2.2), Move(_4.0), Move(_42), Field::<char>(Variant(_18.fld0, 2), 1), Move(_20), _54.2.2, _52, _26.1), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_18.fld0 = Adt21::Variant2 { fld0: _28,fld1: _26.1,fld2: _54.2.1,fld3: _40,fld4: _54.2.2,fld5: Field::<i32>(Variant(_19.2.1, 2), 5) };
place!(Field::<f32>(Variant(_19.2.1, 2), 3)) = _30 as f32;
place!(Field::<u32>(Variant(_43.1, 1), 2)) = Field::<u32>(Variant(_9.0, 1), 0) << _43.0;
_45 = Field::<f32>(Variant(_18.fld0, 2), 3) - Field::<f32>(Variant(_18.fld0, 2), 3);
_24 = (_7, _26.2);
_43.0 = Field::<usize>(Variant(_21.0.1, 0), 0);
place!(Field::<bool>(Variant(_18.fld0, 2), 0)) = !_28;
Goto(bb44)
}
bb44 = {
_52 = _17;
_3 = &_24.0;
place!(Field::<char>(Variant(_9.0, 1), 1)) = Field::<char>(Variant(_19.3.0, 1), 1);
_10.1 = [19896_i16];
_26.1 = Field::<char>(Variant(_9.0, 1), 1);
_54.0 = _50;
_27 = &_10.1;
_19.3.1 = &_53.1;
place!(Field::<u16>(Variant(_19.2.1, 2), 4)) = 67_i8 as u16;
_26.0 = Move(_27);
match Field::<u16>(Variant(_18.fld0, 2), 4) {
0 => bb1,
51623 => bb46,
_ => bb45
}
}
bb45 = {
_2.0 = 131525049359996344566613458349035413143_u128;
_1 = 3273755805_u32 - 1305258743_u32;
_6 = '\u{5878f}';
_1 = 970308197_u32 >> _2.0;
_1 = 45131_u16 as u32;
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
131525049359996344566613458349035413143 => bb6,
_ => bb5
}
}
bb46 = {
_30 = Field::<i32>(Variant(_19.2.1, 2), 5) as i64;
place!(Field::<char>(Variant(_19.3.0, 1), 1)) = _6;
_26.0 = &_10.1;
_43 = (_21.0.0, Move(_18.fld0));
SetDiscriminant(_19.3.0, 3);
_46 = _45 - _53.0.fld1;
_8 = !_37;
_19.3.1 = &_10.2;
RET = Adt70::Variant1 { fld0: Move(_9.0),fld1: _53.0 };
_19.0 = [_24.1,_26.2,_24.1,_26.2,_24.1,_24.1];
_19.3.0 = Adt35::Variant2 { fld0: _24.1,fld1: _30,fld2: _53.0.fld2 };
_63.2 = !_2.0;
_52 = _33;
_41 = -_50;
_59.0 = &_10.2;
_21 = (Move(_43),);
_61 = [_16,_54.2.0,_54.2.0,_54.2.0,_54.2.0,_54.2.0];
_30 = _15 as i64;
_27 = Move(_26.0);
_10.0 = &_40;
_61 = [_16,_16,_54.2.0,_16,_16,_16];
_63.0 = Adt35::Variant0 { fld0: _61,fld1: _32,fld2: _11 };
_17 = !_11;
_55 = [2031_i16,(-21787_i16),9428_i16];
_27 = &_10.1;
_21.0.0 = _54.2.2 as usize;
place!(Field::<i32>(Variant(_19.2.1, 2), 5)) = _46 as i32;
Goto(bb47)
}
bb47 = {
Call(_67 = dump_var(6_usize, 52_usize, Move(_52), 44_usize, Move(_44), 61_usize, Move(_61), 35_usize, Move(_35)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_67 = dump_var(6_usize, 47_usize, Move(_47), 38_usize, Move(_38), 37_usize, Move(_37), 31_usize, Move(_31)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_67 = dump_var(6_usize, 55_usize, Move(_55), 29_usize, Move(_29), 7_usize, Move(_7), 16_usize, Move(_16)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_67 = dump_var(6_usize, 8_usize, Move(_8), 11_usize, Move(_11), 68_usize, _68, 68_usize, _68), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: &'static u8,mut _2: u8,mut _3: [i16; 1],mut _4: char) -> u128 {
mir! {
type RET = u128;
let _5: isize;
let _6: i64;
let _7: Adt70;
let _8: u8;
let _9: i64;
let _10: f64;
let _11: usize;
let _12: isize;
let _13: i32;
let _14: u128;
let _15: [u16; 7];
let _16: Adt36;
let _17: &'static f32;
let _18: [i128; 6];
let _19: isize;
let _20: &'static f32;
let _21: u128;
let _22: f64;
let _23: [u64; 4];
let _24: u32;
let _25: [u8; 6];
let _26: [isize; 7];
let _27: char;
let _28: [i128; 4];
let _29: ();
let _30: ();
{
RET = 311916711658956149968658065920876686875_u128;
_1 = &_2;
_4 = '\u{fd7fa}';
_4 = '\u{2a817}';
RET = 35817698995393362366069827813623032640_u128 ^ 264477794291023985562576641935564049350_u128;
RET = !32879284367561823985727243808768987196_u128;
_4 = '\u{77dcf}';
_6 = (-1023112023379140137_i64);
_4 = '\u{10a6fa}';
_4 = '\u{8de34}';
RET = (-4086_i16) as u128;
RET = 236200809463523502807010896265353359194_u128;
RET = (-9_i8) as u128;
_1 = &_2;
RET = 166030499636324265951597796168895761564_u128;
RET = _4 as u128;
_5 = (-96_i8) as isize;
_5 = 9223372036854775807_isize;
RET = 295623165289214334696360095391139501365_u128 ^ 219759275763760947714609960026506263938_u128;
_2 = (-6104776638201713675155067704325577555_i128) as u8;
_1 = &_2;
RET = 19111489336326998216889173497266304895_u128;
RET = 125278643937859335309575552192861739770_u128;
RET = 87_i8 as u128;
Call(_5 = fn8(Move(_1), _6, _6, (*_1), (*_1), (*_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = -6702372214814579720_i64;
_1 = &_2;
_6 = 22800_u16 as i64;
RET = 126986138984095964538528731324185771178_u128;
_4 = '\u{9ed8a}';
_6 = (-5883177542528822873_i64);
_3 = [(-31140_i16)];
_4 = '\u{32c24}';
RET = 154749217757633107446769864178215366010_u128 & 64209515685802159366210819992908189786_u128;
_5 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_3 = [(-8599_i16)];
_3 = [1762_i16];
_2 = 249_u8 * 104_u8;
_6 = 805183910769676946_i64 >> _2;
_5 = 76_isize ^ (-82_isize);
_6 = -(-7710297633742192835_i64);
RET = 190833612047089089873607628750274922457_u128;
RET = 289204225902446259690205523695360393958_u128 >> _5;
_6 = 8474960274676106260_i64;
_10 = 25909_u16 as f64;
RET = !6261107669864022444790561928760350254_u128;
_9 = false as i64;
_1 = &_8;
match _6 {
0 => bb2,
8474960274676106260 => bb4,
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
_11 = !5296667948230861007_usize;
RET = !81691830459710635940465192752692393976_u128;
Call(_10 = fn9(_4, _5, RET, _5, _5, _3, _2, _9, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = 9223372036854775807_isize << _9;
_1 = &_8;
_8 = (-46_i8) as u8;
_9 = true as i64;
_1 = &_2;
_5 = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = false as u128;
_9 = _6 << _8;
_9 = 143142176_i32 as i64;
RET = 338489357424333179026487136842504176231_u128;
_12 = 38535_u16 as isize;
RET = 144801599572088539024532135259829544671_u128 << _12;
_4 = '\u{5826b}';
RET = !327587234772130192423555265206340594925_u128;
_14 = RET;
RET = _14 + _14;
_9 = _6;
_5 = _12 << _2;
match _9 {
0 => bb6,
1 => bb7,
8474960274676106260 => bb9,
_ => bb8
}
}
bb6 = {
_6 = -6702372214814579720_i64;
_1 = &_2;
_6 = 22800_u16 as i64;
RET = 126986138984095964538528731324185771178_u128;
_4 = '\u{9ed8a}';
_6 = (-5883177542528822873_i64);
_3 = [(-31140_i16)];
_4 = '\u{32c24}';
RET = 154749217757633107446769864178215366010_u128 & 64209515685802159366210819992908189786_u128;
_5 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_3 = [(-8599_i16)];
_3 = [1762_i16];
_2 = 249_u8 * 104_u8;
_6 = 805183910769676946_i64 >> _2;
_5 = 76_isize ^ (-82_isize);
_6 = -(-7710297633742192835_i64);
RET = 190833612047089089873607628750274922457_u128;
RET = 289204225902446259690205523695360393958_u128 >> _5;
_6 = 8474960274676106260_i64;
_10 = 25909_u16 as f64;
RET = !6261107669864022444790561928760350254_u128;
_9 = false as i64;
_1 = &_8;
match _6 {
0 => bb2,
8474960274676106260 => bb4,
_ => bb3
}
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_8 = !_2;
_6 = _11 as i64;
_4 = '\u{f8bea}';
_1 = &_2;
RET = _6 as u128;
_16.fld1 = [_5,_12,_5,_12,_5,_5,_5];
_8 = (*_1);
Goto(bb10)
}
bb10 = {
_14 = 17290630965721840325969660795409482777_i128 as u128;
_15 = [57288_u16,40526_u16,7244_u16,22885_u16,42354_u16,42454_u16,57187_u16];
_18 = [(-146750063442253394279206220556776540569_i128),(-17393891624204469896072552692251378486_i128),125925949097974174858349429710961750481_i128,(-127842084855123063066730558853923623487_i128),91350651812085453710955655235997876534_i128,(-75817379649373515321159990105021843534_i128)];
_18 = [(-16406161135889620937606598126303457654_i128),(-79762152320863032618594876691539820822_i128),109567009132586763449180096327490835082_i128,100516530917999363752962562466331649524_i128,(-64875363833354709671541787191043627547_i128),52461520290470048768948412655550090138_i128];
RET = _14;
_6 = 1415073187_i32 as i64;
_9 = _6 & _6;
_3 = [(-15511_i16)];
_5 = _12 >> (*_1);
_4 = '\u{a7209}';
Goto(bb11)
}
bb11 = {
_8 = (*_1) + _2;
_13 = (-1325886813_i32);
Goto(bb12)
}
bb12 = {
_1 = &_8;
_15 = [48098_u16,18545_u16,21844_u16,7268_u16,41580_u16,15959_u16,14283_u16];
_10 = (*_1) as f64;
_21 = RET;
RET = !_21;
_1 = &(*_1);
_22 = _13 as f64;
_10 = -_22;
_24 = !2914729403_u32;
_10 = _22 - _22;
match _13 {
0 => bb1,
1 => bb3,
2 => bb13,
3 => bb14,
340282366920938463463374607430442324643 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_14 = 17290630965721840325969660795409482777_i128 as u128;
_15 = [57288_u16,40526_u16,7244_u16,22885_u16,42354_u16,42454_u16,57187_u16];
_18 = [(-146750063442253394279206220556776540569_i128),(-17393891624204469896072552692251378486_i128),125925949097974174858349429710961750481_i128,(-127842084855123063066730558853923623487_i128),91350651812085453710955655235997876534_i128,(-75817379649373515321159990105021843534_i128)];
_18 = [(-16406161135889620937606598126303457654_i128),(-79762152320863032618594876691539820822_i128),109567009132586763449180096327490835082_i128,100516530917999363752962562466331649524_i128,(-64875363833354709671541787191043627547_i128),52461520290470048768948412655550090138_i128];
RET = _14;
_6 = 1415073187_i32 as i64;
_9 = _6 & _6;
_3 = [(-15511_i16)];
_5 = _12 >> (*_1);
_4 = '\u{a7209}';
Goto(bb11)
}
bb15 = {
_8 = !_2;
_6 = _11 as i64;
_4 = '\u{f8bea}';
_1 = &_2;
RET = _6 as u128;
_16.fld1 = [_5,_12,_5,_12,_5,_5,_5];
_8 = (*_1);
Goto(bb10)
}
bb16 = {
_10 = _22 + _22;
RET = !_21;
RET = !_21;
_24 = 4151271791_u32 ^ 2845804082_u32;
_25 = [_8,(*_1),_8,_2,_8,(*_1)];
_25 = [(*_1),(*_1),(*_1),(*_1),_8,(*_1)];
_25 = [(*_1),(*_1),_8,(*_1),_8,_8];
_18 = [(-161816669243082503099450384859816671960_i128),(-129858392320746134573954545560947739194_i128),147276510178700636294972998263431306287_i128,(-65921757724284719407276595374432272214_i128),(-150249393631833137603423035926973027766_i128),(-2729837730503668776441352506233146648_i128)];
_26 = _16.fld1;
_11 = !3_usize;
_13 = _5 as i32;
_6 = _9;
_19 = _12;
_23 = [10308283108042927151_u64,6856410871980431261_u64,5140448495263481068_u64,14977497733151408729_u64];
_13 = (-296459120_i32);
_10 = _22 + _22;
_13 = 1084277345_i32;
_14 = !_21;
_22 = _10;
_9 = _6 >> (*_1);
_27 = _4;
_14 = !RET;
_5 = _19 >> _8;
_26 = [_5,_5,_5,_12,_12,_5,_12];
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(7_usize, 11_usize, Move(_11), 15_usize, Move(_15), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(7_usize, 9_usize, Move(_9), 3_usize, Move(_3), 21_usize, Move(_21), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(7_usize, 8_usize, Move(_8), 26_usize, Move(_26), 30_usize, _30, 30_usize, _30), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: &'static u8,mut _2: i64,mut _3: i64,mut _4: u8,mut _5: u8,mut _6: u8) -> isize {
mir! {
type RET = isize;
let _7: [i64; 3];
let _8: f32;
let _9: &'static [i16; 1];
let _10: (usize, *const &'static f32);
let _11: (Adt70,);
let _12: &'static i128;
let _13: ();
let _14: ();
{
_1 = &_5;
RET = -(-9223372036854775808_isize);
_4 = !(*_1);
_1 = &(*_1);
match _3 {
0 => bb1,
1 => bb2,
340282366920938463462351495408389071319 => bb4,
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
_1 = &_6;
_4 = _6 & _5;
_6 = 11_i8 as u8;
_5 = _4 & _4;
RET = 85_i8 as isize;
_4 = !_5;
_6 = _4 >> _5;
_3 = _2;
RET = true as isize;
_2 = _3 >> _4;
match _3 {
340282366920938463462351495408389071319 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
RET = 3297400351_u32 as isize;
RET = (-9223372036854775808_isize);
_1 = &_4;
_2 = _3;
_1 = &_5;
RET = !(-113_isize);
match _3 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
340282366920938463462351495408389071319 => bb13,
_ => bb12
}
}
bb7 = {
Return()
}
bb8 = {
_1 = &_6;
_4 = _6 & _5;
_6 = 11_i8 as u8;
_5 = _4 & _4;
RET = 85_i8 as isize;
_4 = !_5;
_6 = _4 >> _5;
_3 = _2;
RET = true as isize;
_2 = _3 >> _4;
match _3 {
340282366920938463462351495408389071319 => bb6,
_ => bb5
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
Return()
}
bb13 = {
_1 = &(*_1);
_4 = _6;
_5 = (-693175456_i32) as u8;
_2 = _3 * _3;
_7 = [_2,_2,_3];
_1 = &_5;
_6 = 7123_i16 as u8;
_5 = _4;
Goto(bb14)
}
bb14 = {
_2 = _3;
_1 = &_4;
_1 = &(*_1);
_5 = (*_1);
_5 = _4;
_7 = [_3,_3,_3];
_2 = _3 >> _5;
RET = 9223372036854775807_isize + (-53_isize);
RET = 2853976936_u32 as isize;
_3 = _2;
RET = -(-39_isize);
_6 = 21367_u16 as u8;
RET = (-9223372036854775808_isize);
_5 = _4 * (*_1);
_8 = 8274_u16 as f32;
_1 = &_6;
_6 = _5;
_2 = _3;
_8 = _2 as f32;
_7 = [_2,_3,_3];
_8 = 225907226_i32 as f32;
_2 = _3;
_4 = _6 & _6;
Goto(bb15)
}
bb15 = {
Call(_13 = dump_var(8_usize, 4_usize, Move(_4), 7_usize, Move(_7), 2_usize, Move(_2), 14_usize, _14), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: char,mut _2: isize,mut _3: u128,mut _4: isize,mut _5: isize,mut _6: [i16; 1],mut _7: u8,mut _8: i64,mut _9: char) -> f64 {
mir! {
type RET = f64;
let _10: Adt40;
let _11: (i128, u32, u16, char);
let _12: [i16; 4];
let _13: ([u64; 6], [u128; 1], (usize, Adt21), (Adt35, &'static *mut u64, u128));
let _14: char;
let _15: f32;
let _16: &'static f32;
let _17: bool;
let _18: i64;
let _19: isize;
let _20: f64;
let _21: (&'static i64,);
let _22: (&'static [i16; 1], char, u64);
let _23: (Adt70,);
let _24: ();
let _25: ();
{
_11 = ((-103452192630203717092029096110455590520_i128), 1583234550_u32, 56539_u16, _1);
_6 = [14457_i16];
RET = _11.1 as f64;
_10.fld1 = _7 as f32;
_11.2 = !8465_u16;
RET = 1799140263_i32 as f64;
_11.3 = _1;
_11.1 = !3653141320_u32;
_10.fld0 = _9 >= _11.3;
_6 = [(-7991_i16)];
_12 = [(-4533_i16),(-2047_i16),12040_i16,(-11075_i16)];
_7 = _9 as u8;
_13.2.0 = 2_usize + 3609093122081280947_usize;
_10.fld2 = _7 as f64;
_2 = _5 - _4;
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
236830174290734746371345511321312620936 => bb8,
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
_13.1 = [_3];
_12 = [27398_i16,(-28498_i16),(-24245_i16),30966_i16];
_13.3.0 = Adt35::Variant1 { fld0: _11.1,fld1: _1 };
SetDiscriminant(_13.3.0, 3);
_13.3.0 = Adt35::Variant2 { fld0: 5497850218627490687_u64,fld1: _8,fld2: RET };
_13.1 = [_3];
_11 = ((-5059773181700234265095520683856176100_i128), 3157271707_u32, 1763_u16, _9);
_13.3.2 = _3 | _3;
_13.2.0 = 6475723754875503281_usize | 6796726224217736151_usize;
_10.fld1 = (-11596_i16) as f32;
_11.2 = _10.fld1 as u16;
Call(_4 = fn10(_11, _13.1, _11.0, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14 = _9;
_7 = 116_u8;
place!(Field::<u64>(Variant(_13.3.0, 2), 0)) = _9 as u64;
_17 = !_10.fld0;
_2 = -_5;
_9 = _11.3;
_6 = [11934_i16];
_3 = _13.3.2;
_11 = (39095311519767998679004303105889884305_i128, 2448197997_u32, 53851_u16, _14);
_13.2.0 = 3_usize - 16640519929162238681_usize;
_11.3 = _14;
_1 = _11.3;
_13.1 = [_3];
match _11.1 {
0 => bb1,
1 => bb2,
2448197997 => bb10,
_ => bb4
}
}
bb10 = {
_13.1 = [_3];
_4 = _11.0 as isize;
_13.1 = [_13.3.2];
_13.1 = [_13.3.2];
_17 = !_10.fld0;
place!(Field::<u64>(Variant(_13.3.0, 2), 0)) = !3309640861240000296_u64;
_4 = _5;
_14 = _1;
_11.2 = 14035_u16;
match _11.0 {
0 => bb3,
1 => bb9,
39095311519767998679004303105889884305 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_11.2 = !51066_u16;
RET = -Field::<f64>(Variant(_13.3.0, 2), 2);
Goto(bb13)
}
bb13 = {
_8 = Field::<i64>(Variant(_13.3.0, 2), 1);
_5 = 1024072051_i32 as isize;
_13.3.2 = _3;
_3 = !_13.3.2;
_17 = !_10.fld0;
_2 = _5 >> _13.3.2;
_18 = Field::<i64>(Variant(_13.3.0, 2), 1) - Field::<i64>(Variant(_13.3.0, 2), 1);
_12 = [3827_i16,(-3481_i16),(-25707_i16),(-18199_i16)];
_5 = _2;
_18 = -_8;
_3 = _13.3.2 - _13.3.2;
_1 = _9;
_4 = -_5;
_5 = _11.1 as isize;
_10.fld1 = _13.3.2 as f32;
place!(Field::<u64>(Variant(_13.3.0, 2), 0)) = 16241284669794952148_u64;
_18 = -Field::<i64>(Variant(_13.3.0, 2), 1);
_11.0 = (-81173079453616146648070239031924919674_i128);
_20 = _13.3.2 as f64;
_10.fld0 = !_17;
RET = _20 + Field::<f64>(Variant(_13.3.0, 2), 2);
_9 = _11.3;
_11.3 = _1;
_3 = _13.3.2 >> _5;
_13.2.0 = _10.fld1 as usize;
match _11.1 {
0 => bb9,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
2448197997 => bb20,
_ => bb19
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
_14 = _9;
_7 = 116_u8;
place!(Field::<u64>(Variant(_13.3.0, 2), 0)) = _9 as u64;
_17 = !_10.fld0;
_2 = -_5;
_9 = _11.3;
_6 = [11934_i16];
_3 = _13.3.2;
_11 = (39095311519767998679004303105889884305_i128, 2448197997_u32, 53851_u16, _14);
_13.2.0 = 3_usize - 16640519929162238681_usize;
_11.3 = _14;
_1 = _11.3;
_13.1 = [_3];
match _11.1 {
0 => bb1,
1 => bb2,
2448197997 => bb10,
_ => bb4
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
SetDiscriminant(_13.3.0, 1);
place!(Field::<u32>(Variant(_13.3.0, 1), 0)) = _11.1 / _11.1;
place!(Field::<char>(Variant(_13.3.0, 1), 1)) = _11.3;
_10.fld1 = _2 as f32;
_15 = _18 as f32;
_7 = 5_u8 + 40_u8;
RET = -_20;
_22.2 = 14915874164972039143_u64 ^ 1948853799517197512_u64;
RET = _10.fld2;
_17 = _13.3.2 != _13.3.2;
_22.0 = &_6;
_4 = _3 as isize;
RET = _20 * _10.fld2;
SetDiscriminant(_13.3.0, 3);
place!(Field::<u8>(Variant(_13.3.0, 3), 1)) = _7;
_10.fld2 = _8 as f64;
Goto(bb21)
}
bb21 = {
Call(_24 = dump_var(9_usize, 4_usize, Move(_4), 8_usize, Move(_8), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_24 = dump_var(9_usize, 7_usize, Move(_7), 18_usize, Move(_18), 11_usize, Move(_11), 25_usize, _25), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (i128, u32, u16, char),mut _2: [u128; 1],mut _3: i128,mut _4: i64) -> isize {
mir! {
type RET = isize;
let _5: *const &'static f32;
let _6: Adt40;
let _7: &'static *mut usize;
let _8: u64;
let _9: bool;
let _10: (i128, u32, u16, char);
let _11: (Adt40, *mut u64);
let _12: [u16; 7];
let _13: u32;
let _14: &'static i64;
let _15: [i64; 3];
let _16: (Adt70, (usize, Adt21));
let _17: f64;
let _18: isize;
let _19: f32;
let _20: char;
let _21: Adt21;
let _22: char;
let _23: isize;
let _24: ();
let _25: ();
{
RET = (-9223372036854775808_isize);
_3 = _1.0;
_4 = _3 as i64;
Call(_1.1 = fn11(_1.0, _3, _4, _1.0, _3, _1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = -(-1453478916893353626_i64);
_1.1 = 3477605989_u32 - 1295790783_u32;
_1.0 = _3;
_4 = 7625381495016179999_i64 + 8678537548243967782_i64;
_1.0 = _3 | _3;
_1.3 = '\u{9726b}';
_2 = [268292872103963240191712787045621190424_u128];
RET = (-633675464_i32) as isize;
_1.3 = '\u{10d630}';
_2 = [67410155199510676667808222374201853863_u128];
_4 = (-6476328450423069903_i64) & (-4766470242104212787_i64);
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
_4 = !3470066295961054515_i64;
_1.1 = _1.2 as u32;
RET = 7_usize as isize;
RET = !(-9223372036854775808_isize);
RET = 20_isize >> _3;
_9 = _3 == _3;
_1.2 = 54906_u16;
_6.fld1 = 779574533_i32 as f32;
_6.fld0 = _9;
_2 = [136201043002870244495517173350854955052_u128];
match _1.2 {
0 => bb2,
1 => bb3,
2 => bb4,
54906 => bb6,
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
_3 = _1.0 >> _1.1;
_2 = [227047316763542682194076504518779645500_u128];
_6.fld2 = (-2234_i16) as f64;
_6.fld2 = _6.fld1 as f64;
_10.0 = _3 & _1.0;
_10.1 = _1.1 ^ _1.1;
_10.1 = _1.1 + _1.1;
_9 = !_6.fld0;
_10.2 = !_1.2;
_1.2 = _10.2;
_10.1 = _1.1 * _1.1;
RET = (-134540269_i32) as isize;
_11.0 = Adt40 { fld0: _6.fld0,fld1: _6.fld1,fld2: _6.fld2 };
_11.0.fld0 = !_9;
_10.2 = !_1.2;
_11.1 = core::ptr::addr_of_mut!(_8);
_6.fld1 = _11.0.fld1;
_10 = (_3, _1.1, _1.2, _1.3);
_12 = [_1.2,_10.2,_1.2,_10.2,_10.2,_1.2,_10.2];
_13 = _1.1 + _1.1;
_3 = _10.0;
_10.2 = !_1.2;
_8 = 6616892598246506414_u64;
_15 = [_4,_4,_4];
Goto(bb7)
}
bb7 = {
_1 = (_3, _13, _10.2, _10.3);
_9 = _11.0.fld0;
_10.1 = _1.1 << _1.0;
_11.1 = core::ptr::addr_of_mut!(_8);
_13 = RET as u32;
_11.0.fld1 = _6.fld1 - _6.fld1;
_17 = _6.fld2 * _11.0.fld2;
_6 = Adt40 { fld0: _11.0.fld0,fld1: _11.0.fld1,fld2: _11.0.fld2 };
_10 = (_3, _1.1, _1.2, _1.3);
_10 = (_1.0, _1.1, _1.2, _1.3);
_16.1.1 = Adt21::Variant2 { fld0: _9,fld1: _1.3,fld2: _10.1,fld3: _6.fld1,fld4: _1.2,fld5: (-1005836206_i32) };
Goto(bb8)
}
bb8 = {
place!(Field::<u32>(Variant(_16.1.1, 2), 2)) = _17 as u32;
_4 = (-422732058155779889_i64) - (-778935438810905353_i64);
_6.fld0 = Field::<bool>(Variant(_16.1.1, 2), 0) < _11.0.fld0;
place!(Field::<bool>(Variant(_16.1.1, 2), 0)) = _9;
_11.1 = core::ptr::addr_of_mut!(_8);
match _8 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
6616892598246506414 => bb14,
_ => bb13
}
}
bb9 = {
_1 = (_3, _13, _10.2, _10.3);
_9 = _11.0.fld0;
_10.1 = _1.1 << _1.0;
_11.1 = core::ptr::addr_of_mut!(_8);
_13 = RET as u32;
_11.0.fld1 = _6.fld1 - _6.fld1;
_17 = _6.fld2 * _11.0.fld2;
_6 = Adt40 { fld0: _11.0.fld0,fld1: _11.0.fld1,fld2: _11.0.fld2 };
_10 = (_3, _1.1, _1.2, _1.3);
_10 = (_1.0, _1.1, _1.2, _1.3);
_16.1.1 = Adt21::Variant2 { fld0: _9,fld1: _1.3,fld2: _10.1,fld3: _6.fld1,fld4: _1.2,fld5: (-1005836206_i32) };
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_4 = -(-1453478916893353626_i64);
_1.1 = 3477605989_u32 - 1295790783_u32;
_1.0 = _3;
_4 = 7625381495016179999_i64 + 8678537548243967782_i64;
_1.0 = _3 | _3;
_1.3 = '\u{9726b}';
_2 = [268292872103963240191712787045621190424_u128];
RET = (-633675464_i32) as isize;
_1.3 = '\u{10d630}';
_2 = [67410155199510676667808222374201853863_u128];
_4 = (-6476328450423069903_i64) & (-4766470242104212787_i64);
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
_4 = !3470066295961054515_i64;
_1.1 = _1.2 as u32;
RET = 7_usize as isize;
RET = !(-9223372036854775808_isize);
RET = 20_isize >> _3;
_9 = _3 == _3;
_1.2 = 54906_u16;
_6.fld1 = 779574533_i32 as f32;
_6.fld0 = _9;
_2 = [136201043002870244495517173350854955052_u128];
match _1.2 {
0 => bb2,
1 => bb3,
2 => bb4,
54906 => bb6,
_ => bb5
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_2 = [74256914930354753339821358893860727104_u128];
_11.0.fld2 = _17 - _17;
_11.0.fld0 = _6.fld0 <= Field::<bool>(Variant(_16.1.1, 2), 0);
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(10_usize, 10_usize, Move(_10), 8_usize, Move(_8), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(10_usize, 2_usize, Move(_2), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i128,mut _2: i128,mut _3: i64,mut _4: i128,mut _5: i128,mut _6: i128) -> u32 {
mir! {
type RET = u32;
let _7: u8;
let _8: isize;
let _9: Adt21;
let _10: f32;
let _11: &'static u8;
let _12: [bool; 7];
let _13: u64;
let _14: (u128, &'static i128, &'static [isize; 7]);
let _15: [u64; 6];
let _16: isize;
let _17: *const u32;
let _18: isize;
let _19: &'static f32;
let _20: char;
let _21: [i64; 3];
let _22: (&'static [i16; 1], char, u64);
let _23: (usize, *const &'static f32);
let _24: (Adt35, &'static *mut u64, u128);
let _25: (f64, [i64; 3], (i128, u32, u16, char));
let _26: [u64; 6];
let _27: ();
let _28: ();
{
_5 = !_1;
_2 = -_5;
_7 = !121_u8;
_6 = !_2;
_5 = 223830811245311574289417709100637698555_u128 as i128;
_2 = _4;
_1 = _6 & _4;
_5 = -_6;
_5 = 567624456_u32 as i128;
_1 = 551849466_u32 as i128;
RET = !3285544733_u32;
_11 = &_7;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
335222593739238229198279086747912035356 => bb5,
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
_10 = _3 as f32;
_5 = _2;
_12 = [false,true,false,false,false,true,true];
RET = !3302205408_u32;
_8 = (-9223372036854775808_isize) & 12_isize;
_3 = -6649990129468228901_i64;
_8 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_8 = 102_isize;
_5 = !_6;
_1 = 27396_u16 as i128;
_11 = &(*_11);
RET = '\u{9e7c8}' as u32;
_13 = !492113306580348267_u64;
RET = (-718527104_i32) as u32;
_11 = &_7;
_4 = _10 as i128;
_10 = 115_i8 as f32;
_14.1 = &_1;
Goto(bb6)
}
bb6 = {
_14.1 = &_6;
_5 = _4 - _2;
_9 = Adt21::Variant2 { fld0: false,fld1: '\u{81f24}',fld2: RET,fld3: _10,fld4: 32340_u16,fld5: 2021296388_i32 };
place!(Field::<char>(Variant(_9, 2), 1)) = '\u{63f19}';
_13 = 8056319689796194872_u64;
RET = Field::<u32>(Variant(_9, 2), 2) - Field::<u32>(Variant(_9, 2), 2);
_13 = 2137713890902796766_u64 ^ 9082761750624812690_u64;
_14.1 = &_1;
_14.1 = &_6;
_11 = &_7;
_9 = Adt21::Variant1 { fld0: false,fld1: 7480857486461283235_usize,fld2: RET,fld3: _2,fld4: _12 };
_8 = (-12959_i16) as isize;
place!(Field::<usize>(Variant(_9, 1), 1)) = !7_usize;
_2 = _5 + _6;
_12 = [true,true,false,false,false,true,true];
_12 = [false,true,true,true,true,true,true];
_10 = (-1116568282_i32) as f32;
_16 = _8 & _8;
_3 = (-10360_i16) as i64;
place!(Field::<[bool; 7]>(Variant(_9, 1), 4)) = [false,false,true,false,true,false,false];
_10 = 1791258110_i32 as f32;
_15 = [_13,_13,_13,_13,_13,_13];
place!(Field::<bool>(Variant(_9, 1), 0)) = true ^ true;
place!(Field::<u32>(Variant(_9, 1), 2)) = !RET;
RET = Field::<u32>(Variant(_9, 1), 2) - Field::<u32>(Variant(_9, 1), 2);
_15 = [_13,_13,_13,_13,_13,_13];
Call(place!(Field::<u32>(Variant(_9, 1), 2)) = core::intrinsics::transmute(RET), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13 = Field::<bool>(Variant(_9, 1), 0) as u64;
place!(Field::<u32>(Variant(_9, 1), 2)) = !RET;
_14.0 = 121047656422017710410928756837881366913_u128;
_17 = core::ptr::addr_of!(RET);
(*_17) = Field::<u32>(Variant(_9, 1), 2);
place!(Field::<u32>(Variant(_9, 1), 2)) = _13 as u32;
_2 = _5 | _5;
_19 = &_10;
_14.1 = &place!(Field::<i128>(Variant(_9, 1), 3));
_6 = Field::<i128>(Variant(_9, 1), 3) * _2;
RET = Field::<u32>(Variant(_9, 1), 2) >> _2;
_6 = _5;
place!(Field::<usize>(Variant(_9, 1), 1)) = !12533634137752082078_usize;
place!(Field::<bool>(Variant(_9, 1), 0)) = true;
_10 = _16 as f32;
_18 = -_16;
_6 = 11391_i16 as i128;
_3 = !(-1934613192455407968_i64);
_23.0 = Field::<usize>(Variant(_9, 1), 1) << RET;
place!(Field::<i128>(Variant(_9, 1), 3)) = _7 as i128;
_6 = _2;
Goto(bb8)
}
bb8 = {
Call(_27 = dump_var(11_usize, 4_usize, Move(_4), 3_usize, Move(_3), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_27 = dump_var(11_usize, 6_usize, Move(_6), 13_usize, Move(_13), 28_usize, _28, 28_usize, _28), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u8,mut _2: isize) -> Adt35 {
mir! {
type RET = Adt35;
let _3: i16;
let _4: u16;
let _5: *mut i16;
let _6: &'static i64;
let _7: &'static [isize; 7];
let _8: f64;
let _9: i32;
let _10: f64;
let _11: char;
let _12: i16;
let _13: bool;
let _14: [i16; 1];
let _15: ([u64; 6], [u128; 1], (usize, Adt21), (Adt35, &'static *mut u64, u128));
let _16: &'static [usize; 4];
let _17: [i64; 3];
let _18: (u8, u64);
let _19: [usize; 4];
let _20: isize;
let _21: isize;
let _22: bool;
let _23: isize;
let _24: isize;
let _25: ();
let _26: ();
{
RET = Adt35::Variant3 { fld0: (-64343920_i32),fld1: _1 };
place!(Field::<u8>(Variant(RET, 3), 1)) = _1 ^ _1;
place!(Field::<i32>(Variant(RET, 3), 0)) = (-1683360262_i32) - (-1584227715_i32);
place!(Field::<i32>(Variant(RET, 3), 0)) = (-5711480423560368321_i64) as i32;
RET = Adt35::Variant3 { fld0: (-1105190030_i32),fld1: _1 };
_2 = (-145665566657244676608937557332381417833_i128) as isize;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1 >> _1;
place!(Field::<i32>(Variant(RET, 3), 0)) = 1485945862_i32;
Goto(bb1)
}
bb1 = {
place!(Field::<i32>(Variant(RET, 3), 0)) = !(-1142413388_i32);
_1 = !Field::<u8>(Variant(RET, 3), 1);
place!(Field::<i32>(Variant(RET, 3), 0)) = 26_i8 as i32;
RET = Adt35::Variant3 { fld0: (-855139881_i32),fld1: _1 };
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
Goto(bb2)
}
bb2 = {
_4 = 10519_u16 ^ 9611_u16;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
_4 = 9177988795256887538_i64 as u16;
place!(Field::<i32>(Variant(RET, 3), 0)) = 1052958676_i32;
_3 = !(-3593_i16);
place!(Field::<u8>(Variant(RET, 3), 1)) = _1 * _1;
_4 = true as u16;
_1 = Field::<u8>(Variant(RET, 3), 1);
_4 = 1598_u16 & 36876_u16;
place!(Field::<u8>(Variant(RET, 3), 1)) = !_1;
SetDiscriminant(RET, 3);
place!(Field::<i32>(Variant(RET, 3), 0)) = (-1678900270_i32);
_4 = !451_u16;
place!(Field::<i32>(Variant(RET, 3), 0)) = -1719390178_i32;
_3 = (-31592_i16);
RET = Adt35::Variant1 { fld0: 1177682651_u32,fld1: '\u{1406d}' };
place!(Field::<u32>(Variant(RET, 1), 0)) = !3782890660_u32;
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768179864 => bb9,
_ => bb8
}
}
bb3 = {
place!(Field::<i32>(Variant(RET, 3), 0)) = !(-1142413388_i32);
_1 = !Field::<u8>(Variant(RET, 3), 1);
place!(Field::<i32>(Variant(RET, 3), 0)) = 26_i8 as i32;
RET = Adt35::Variant3 { fld0: (-855139881_i32),fld1: _1 };
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
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
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{109cf9}';
place!(Field::<u32>(Variant(RET, 1), 0)) = !324899326_u32;
place!(Field::<u32>(Variant(RET, 1), 0)) = 2873877970_u32;
_2 = false as isize;
_5 = core::ptr::addr_of_mut!(_3);
(*_5) = (-9337_i16) - 28127_i16;
_3 = 11080_i16;
(*_5) = 23731_i16 + (-20153_i16);
place!(Field::<u32>(Variant(RET, 1), 0)) = 580227604_u32;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{b500d}';
RET = Adt35::Variant3 { fld0: 968680339_i32,fld1: _1 };
_1 = '\u{1242a}' as u8;
_1 = Field::<u8>(Variant(RET, 3), 1) << _4;
place!(Field::<i32>(Variant(RET, 3), 0)) = 7_usize as i32;
place!(Field::<u8>(Variant(RET, 3), 1)) = 10673466863796468647_usize as u8;
_4 = !6123_u16;
Goto(bb10)
}
bb10 = {
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
(*_5) = 17606_i16;
place!(Field::<i32>(Variant(RET, 3), 0)) = 1652362412_i32 << Field::<u8>(Variant(RET, 3), 1);
_4 = 50069_u16;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1 - _1;
place!(Field::<i32>(Variant(RET, 3), 0)) = 6813853516269645233_u64 as i32;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
place!(Field::<u8>(Variant(RET, 3), 1)) = 140257804662214167517980447751545281344_i128 as u8;
(*_5) = (-10364_i16);
_4 = 1673604137_u32 as u16;
place!(Field::<i32>(Variant(RET, 3), 0)) = (-60457779017538715804588243657292394268_i128) as i32;
_8 = (-2_i8) as f64;
_10 = _8 - _8;
place!(Field::<i32>(Variant(RET, 3), 0)) = (-1037342038_i32) | (-1960822533_i32);
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
_11 = '\u{c4dba}';
_9 = Field::<i32>(Variant(RET, 3), 0) ^ Field::<i32>(Variant(RET, 3), 0);
(*_5) = (-26073_i16) >> _1;
place!(Field::<i32>(Variant(RET, 3), 0)) = 159799836835118785981378535902990687486_i128 as i32;
(*_5) = _10 as i16;
_3 = 31494_i16;
place!(Field::<u8>(Variant(RET, 3), 1)) = !_1;
_11 = '\u{ad43b}';
_12 = _3 * (*_5);
_8 = _10;
_10 = _8;
_11 = '\u{392cb}';
Goto(bb11)
}
bb11 = {
_13 = true;
_3 = _12;
RET = Adt35::Variant1 { fld0: 2891201915_u32,fld1: _11 };
_12 = _3;
RET = Adt35::Variant3 { fld0: _9,fld1: _1 };
_10 = -_8;
(*_5) = _12 ^ _12;
_15.1 = [22073698448453292220439567735709451226_u128];
_5 = core::ptr::addr_of_mut!((*_5));
_11 = '\u{6bbff}';
_5 = core::ptr::addr_of_mut!(_12);
_9 = _11 as i32;
RET = Adt35::Variant1 { fld0: 2340675556_u32,fld1: _11 };
RET = Adt35::Variant2 { fld0: 1087725201564072779_u64,fld1: (-7190598058924280708_i64),fld2: _8 };
Call(place!(Field::<u64>(Variant(RET, 2), 0)) = fn13(_1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_13 = !false;
(*_5) = _3 ^ _3;
_8 = _10 - _10;
place!(Field::<i64>(Variant(RET, 2), 1)) = (-5095641538673367210_i64) >> _1;
_6 = &place!(Field::<i64>(Variant(RET, 2), 1));
_15.1 = [124978081582819328765130880905793439042_u128];
_1 = !79_u8;
RET = Adt35::Variant3 { fld0: _9,fld1: _1 };
_4 = _11 as u16;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
_2 = !9223372036854775807_isize;
_3 = -(*_5);
_1 = !Field::<u8>(Variant(RET, 3), 1);
_15.3.0 = Adt35::Variant1 { fld0: 1760021713_u32,fld1: _11 };
_3 = _12;
_13 = false;
_1 = !Field::<u8>(Variant(RET, 3), 1);
_15.2.0 = 5365625790669805675_usize;
place!(Field::<u32>(Variant(_15.3.0, 1), 0)) = 2452129167_u32;
place!(Field::<u8>(Variant(RET, 3), 1)) = _8 as u8;
_1 = 224242409034375306813279468979708482338_u128 as u8;
_1 = !Field::<u8>(Variant(RET, 3), 1);
place!(Field::<i32>(Variant(RET, 3), 0)) = _3 as i32;
_10 = _8;
match Field::<u32>(Variant(_15.3.0, 1), 0) {
0 => bb5,
1 => bb9,
2 => bb11,
3 => bb13,
4 => bb14,
2452129167 => bb16,
_ => bb15
}
}
bb13 = {
place!(Field::<i32>(Variant(RET, 3), 0)) = !(-1142413388_i32);
_1 = !Field::<u8>(Variant(RET, 3), 1);
place!(Field::<i32>(Variant(RET, 3), 0)) = 26_i8 as i32;
RET = Adt35::Variant3 { fld0: (-855139881_i32),fld1: _1 };
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
place!(Field::<u8>(Variant(RET, 3), 1)) = _1;
_18.1 = 11105886030069945329_u64;
_17 = [(-6699894327822000108_i64),4647690766035459970_i64,(-2474536625183695817_i64)];
_19 = [_15.2.0,_15.2.0,_15.2.0,_15.2.0];
(*_5) = (-36072814602511161756698088489658497169_i128) as i16;
_18.0 = _3 as u8;
_2 = 122_isize & 9223372036854775807_isize;
_15.1 = [57432465872862189426036836495118210070_u128];
RET = Move(_15.3.0);
_11 = Field::<char>(Variant(RET, 1), 1);
_4 = !35828_u16;
_12 = _18.1 as i16;
_19 = [_15.2.0,_15.2.0,_15.2.0,_15.2.0];
_18.0 = _15.2.0 as u8;
_9 = 2003080652_i32;
SetDiscriminant(RET, 2);
Goto(bb17)
}
bb17 = {
_20 = !_2;
_21 = _2;
_22 = _13;
place!(Field::<i64>(Variant(RET, 2), 1)) = !273671140812793971_i64;
_15.0 = [_18.1,_18.1,_18.1,_18.1,_18.1,_18.1];
place!(Field::<i64>(Variant(RET, 2), 1)) = -1034668878903963893_i64;
RET = Adt35::Variant3 { fld0: _9,fld1: _18.0 };
_12 = _3;
_15.3.0 = Move(RET);
_20 = 238340866101210892433903225378080360391_u128 as isize;
_8 = -_10;
_19 = [_15.2.0,_15.2.0,_15.2.0,_15.2.0];
RET = Move(_15.3.0);
_20 = _2;
RET = Adt35::Variant3 { fld0: _9,fld1: _1 };
place!(Field::<u8>(Variant(RET, 3), 1)) = _1 - _1;
_15.3.0 = Adt35::Variant1 { fld0: 1205466275_u32,fld1: _11 };
_14 = [_12];
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(12_usize, 21_usize, Move(_21), 1_usize, Move(_1), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(12_usize, 19_usize, Move(_19), 17_usize, Move(_17), 22_usize, Move(_22), 26_usize, _26), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u8) -> u64 {
mir! {
type RET = u64;
let _2: char;
let _3: bool;
let _4: (&'static f32, [i16; 1], *mut u64, &'static *mut u64);
let _5: (&'static i64,);
let _6: ((usize, *const &'static f32), &'static *mut u64, &'static f32);
let _7: u32;
let _8: [i16; 4];
let _9: [i128; 4];
let _10: &'static *mut u64;
let _11: i16;
let _12: [usize; 4];
let _13: &'static i128;
let _14: ();
let _15: ();
{
RET = 4895658978226785648_u64;
RET = !6565241277990834493_u64;
RET = 1777180851514123320_u64;
RET = !12001352408838399517_u64;
_1 = (-67_i8) as u8;
RET = (-137603567654447336567511047814685534699_i128) as u64;
_1 = 103_u8 >> RET;
RET = 7564587507890561129_u64;
RET = 21744_u16 as u64;
_1 = 89_u8 & 202_u8;
_2 = '\u{cef64}';
_2 = '\u{b2671}';
RET = !2277211336469196416_u64;
_2 = '\u{3cf4f}';
_1 = !97_u8;
_1 = (-186994937_i32) as u8;
_2 = '\u{c40e3}';
RET = (-112_isize) as u64;
RET = 542430488969448771_u64;
RET = !2449593108167370984_u64;
RET = 1505982729685036980_u64;
RET = 968913616150852312_u64;
_2 = '\u{5ce61}';
RET = !7001072703435384380_u64;
_2 = '\u{af768}';
RET = 1643057553473857599_u64;
RET = 14639653771566670486_u64 * 18199114317381161756_u64;
RET = 24892_u16 as u64;
Goto(bb1)
}
bb1 = {
_3 = _1 >= _1;
RET = 64_i8 as u64;
_4.1 = [(-20153_i16)];
_4.3 = &_4.2;
RET = !729976274736789083_u64;
_4.2 = core::ptr::addr_of_mut!(RET);
_2 = '\u{d7937}';
_1 = 2_u8;
_6.0.1 = core::ptr::addr_of!(_4.0);
_6.0.0 = 11989_i16 as usize;
_6.0.0 = 7_usize >> RET;
_6.1 = &_4.2;
_4.3 = Move(_6.1);
RET = 14996565933886029692_u64 ^ 5138059201002891512_u64;
_6.1 = &_4.2;
_6.0.0 = !12018760750236062034_usize;
_7 = 2662882328_u32 * 4173163287_u32;
_8 = [(-25734_i16),18135_i16,(-16176_i16),21171_i16];
_3 = !true;
_4.3 = Move(_6.1);
_4.3 = &_4.2;
_3 = false & false;
_8 = [(-13486_i16),7599_i16,(-10767_i16),(-4452_i16)];
match _1 {
0 => bb2,
1 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
2 => bb9,
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
_6.0.1 = core::ptr::addr_of!(_6.2);
RET = !8546700607772012341_u64;
_1 = 1209634625_i32 as u8;
_2 = '\u{f505e}';
_4.3 = &_4.2;
_6.0.1 = core::ptr::addr_of!(_4.0);
_4.3 = &_4.2;
_4.1 = [756_i16];
_6.1 = &_4.2;
_4.3 = Move(_6.1);
RET = 15684313250146549136_u64;
_10 = &_4.2;
Goto(bb10)
}
bb10 = {
_6.0.0 = (-604416040181213211_i64) as usize;
_11 = !(-26366_i16);
_2 = '\u{eba07}';
_6.0.1 = core::ptr::addr_of!(_4.0);
_8 = [_11,_11,_11,_11];
_2 = '\u{d2755}';
_4.3 = &(*_10);
_6.1 = Move(_4.3);
_1 = 9223372036854775807_isize as u8;
RET = 16241732955002946068_u64;
_7 = 9223372036854775807_isize as u32;
match RET {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb11,
16241732955002946068 => bb13,
_ => bb12
}
}
bb11 = {
_6.0.1 = core::ptr::addr_of!(_6.2);
RET = !8546700607772012341_u64;
_1 = 1209634625_i32 as u8;
_2 = '\u{f505e}';
_4.3 = &_4.2;
_6.0.1 = core::ptr::addr_of!(_4.0);
_4.3 = &_4.2;
_4.1 = [756_i16];
_6.1 = &_4.2;
_4.3 = Move(_6.1);
RET = 15684313250146549136_u64;
_10 = &_4.2;
Goto(bb10)
}
bb12 = {
Return()
}
bb13 = {
_4.1 = [_11];
_12 = [_6.0.0,_6.0.0,_6.0.0,_6.0.0];
_4.2 = core::ptr::addr_of_mut!(RET);
_7 = !2092835929_u32;
_6.1 = &_4.2;
match RET {
0 => bb7,
1 => bb8,
2 => bb3,
16241732955002946068 => bb15,
_ => bb14
}
}
bb14 = {
_3 = _1 >= _1;
RET = 64_i8 as u64;
_4.1 = [(-20153_i16)];
_4.3 = &_4.2;
RET = !729976274736789083_u64;
_4.2 = core::ptr::addr_of_mut!(RET);
_2 = '\u{d7937}';
_1 = 2_u8;
_6.0.1 = core::ptr::addr_of!(_4.0);
_6.0.0 = 11989_i16 as usize;
_6.0.0 = 7_usize >> RET;
_6.1 = &_4.2;
_4.3 = Move(_6.1);
RET = 14996565933886029692_u64 ^ 5138059201002891512_u64;
_6.1 = &_4.2;
_6.0.0 = !12018760750236062034_usize;
_7 = 2662882328_u32 * 4173163287_u32;
_8 = [(-25734_i16),18135_i16,(-16176_i16),21171_i16];
_3 = !true;
_4.3 = Move(_6.1);
_4.3 = &_4.2;
_3 = false & false;
_8 = [(-13486_i16),7599_i16,(-10767_i16),(-4452_i16)];
match _1 {
0 => bb2,
1 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
2 => bb9,
_ => bb8
}
}
bb15 = {
_9 = [(-43697562324067817535194414082820240746_i128),(-39996291907174380891972353344201781962_i128),123033657790988647360365932929785232054_i128,63850056406138541804944874867094724545_i128];
_8 = [_11,_11,_11,_11];
RET = !12892151995319405317_u64;
_1 = !214_u8;
_9 = [55202367697567571921701028986879467171_i128,(-70667327140345928598102286366101835813_i128),145948677070866831783775421701989180951_i128,(-130497147353318714179762193450406181830_i128)];
_6.1 = &_4.2;
_6.0.1 = core::ptr::addr_of!(_4.0);
_3 = true;
_10 = &_4.2;
_12 = [_6.0.0,_6.0.0,_6.0.0,_6.0.0];
_6.0.0 = 4_usize;
_11 = !7840_i16;
Goto(bb16)
}
bb16 = {
Call(_14 = dump_var(13_usize, 11_usize, Move(_11), 7_usize, Move(_7), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static [isize; 7],mut _2: &'static *mut u64,mut _3: &'static f32,mut _4: char,mut _5: Adt67,mut _6: u16,mut _7: isize,mut _8: char) -> i64 {
mir! {
type RET = i64;
let _9: Adt67;
let _10: (Adt70,);
let _11: isize;
let _12: f32;
let _13: bool;
let _14: (Adt70,);
let _15: [i128; 6];
let _16: f32;
let _17: (Adt70,);
let _18: isize;
let _19: (&'static *mut u64,);
let _20: isize;
let _21: &'static *mut usize;
let _22: bool;
let _23: Adt77;
let _24: f64;
let _25: &'static *mut u64;
let _26: [i16; 4];
let _27: (f32,);
let _28: f32;
let _29: isize;
let _30: *const u32;
let _31: *const &'static *mut usize;
let _32: &'static &'static *mut u64;
let _33: u8;
let _34: Adt35;
let _35: f64;
let _36: &'static f32;
let _37: isize;
let _38: ();
let _39: ();
{
_8 = _4;
_1 = &place!(Field::<[isize; 7]>(Variant(_5, 1), 0));
_6 = 5465_u16 ^ 17279_u16;
RET = 9045036206271373260_i64;
_6 = 1073536140907311684_u64 as u16;
_7 = Field::<u32>(Variant(_5, 1), 1) as isize;
place!(Field::<u32>(Variant(_5, 1), 1)) = 3149266842_u32;
_8 = _4;
_7 = 9223372036854775807_isize;
_9 = Move(_5);
_1 = &place!(Field::<[isize; 7]>(Variant(_9, 1), 0));
_5 = Move(_9);
Goto(bb1)
}
bb1 = {
RET = -(-1350048708649398653_i64);
_4 = _8;
place!(Field::<[isize; 7]>(Variant(_5, 1), 0)) = [_7,_7,_7,_7,_7,_7,_7];
_1 = &place!(Field::<[isize; 7]>(Variant(_5, 1), 0));
_3 = &_12;
_9 = Adt67::Variant1 { fld0: (*_1),fld1: Field::<u32>(Variant(_5, 1), 1) };
RET = (-1156362308696168399_i64) & 6257830303395273134_i64;
place!(Field::<[isize; 7]>(Variant(_9, 1), 0)) = [_7,_7,_7,_7,_7,_7,_7];
_8 = _4;
RET = 1036198210821927451_i64 * (-526346710784949661_i64);
_5 = Adt67::Variant1 { fld0: Field::<[isize; 7]>(Variant(_9, 1), 0),fld1: Field::<u32>(Variant(_9, 1), 1) };
place!(Field::<u32>(Variant(_5, 1), 1)) = 100_i8 as u32;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_9, 1), 1);
RET = -607856967274201705_i64;
_15 = [27728848951292124765957770421709841124_i128,(-42446836228321776666228676447539845344_i128),(-40848094417970051906210224605777687071_i128),57180213396403617293381020179079460908_i128,5305578953031933523105565999340530788_i128,(-5350570541329875151900051347630459639_i128)];
_1 = &place!(Field::<[isize; 7]>(Variant(_9, 1), 0));
SetDiscriminant(_5, 1);
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = Move(_9);
_12 = RET as f32;
SetDiscriminant(_5, 2);
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).3 = _8;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).3 = _4;
place!(Field::<u32>(Variant(_5, 2), 1)) = 1296416517_u32 + 1255262008_u32;
_3 = &_12;
_15 = [128136781569085358292604870393076552296_i128,109838737318123324434999424369706014615_i128,61206069556165587611369078771896599068_i128,150567747465904032722049252181999450117_i128,110294164237834530712154978705585536178_i128,31522754928208737797530432202696460478_i128];
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).1 = _12 as u32;
_15 = [(-127719594564114201579514160881771927688_i128),93244221748738637169265666266155780222_i128,46024549262746230232528922407173366737_i128,49590636847262544406207917313982419469_i128,(-60761687230888004272844334098605339396_i128),(-169116320061263412370973639807667047205_i128)];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.3 = _4;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).0 = (-87008535281763795875568478723372787333_i128);
RET = 5175240774418253905_i64;
_16 = -(*_3);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = Field::<u32>(Variant(_5, 2), 1);
_11 = _7 & _7;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).0 = _11 as f64;
_8 = _4;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).2 = !_6;
_7 = !_11;
Goto(bb3)
}
bb3 = {
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).0 = _11 as i128;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = Field::<u32>(Variant(_5, 2), 1) >> Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).2;
_16 = _12 + _12;
place!(Field::<[i128; 6]>(Variant(_5, 2), 2)) = [Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0];
_13 = _6 < _6;
RET = !(-8493671318474624241_i64);
Goto(bb4)
}
bb4 = {
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.2 = _6 - _6;
_3 = &(*_3);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.3 = _8;
place!(Field::<[i32; 6]>(Variant(_5, 2), 0)) = [307013122_i32,464749936_i32,(-1907438495_i32),(-1546743024_i32),(-1858140605_i32),(-1536260320_i32)];
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = !Field::<u32>(Variant(_5, 2), 1);
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).2 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.2 | Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.2;
_18 = _7;
_8 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).3;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).3 = _4;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).0 = 141029629719796961717341910730385126995_i128;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)) = ((-54104815685035139269153081018086470335_i128), Field::<u32>(Variant(_5, 2), 1), _6, _8);
Goto(bb5)
}
bb5 = {
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).3 = _4;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3);
RET = (-6747784589955973802_i64) - (-3122941135037228400_i64);
_8 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.3;
_12 = _16;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).0 = 6608566989834260175_u64 as f64;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).0 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0;
_18 = _11 - _11;
_3 = &_16;
_11 = _7;
_18 = _7;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).1;
_8 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.3;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).3 = _4;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = !Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).1;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).1 = [RET,RET,RET];
match Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0 {
0 => bb1,
1 => bb3,
286177551235903324194221526413681741121 => bb7,
_ => bb6
}
}
bb6 = {
RET = -(-1350048708649398653_i64);
_4 = _8;
place!(Field::<[isize; 7]>(Variant(_5, 1), 0)) = [_7,_7,_7,_7,_7,_7,_7];
_1 = &place!(Field::<[isize; 7]>(Variant(_5, 1), 0));
_3 = &_12;
_9 = Adt67::Variant1 { fld0: (*_1),fld1: Field::<u32>(Variant(_5, 1), 1) };
RET = (-1156362308696168399_i64) & 6257830303395273134_i64;
place!(Field::<[isize; 7]>(Variant(_9, 1), 0)) = [_7,_7,_7,_7,_7,_7,_7];
_8 = _4;
RET = 1036198210821927451_i64 * (-526346710784949661_i64);
_5 = Adt67::Variant1 { fld0: Field::<[isize; 7]>(Variant(_9, 1), 0),fld1: Field::<u32>(Variant(_9, 1), 1) };
place!(Field::<u32>(Variant(_5, 1), 1)) = 100_i8 as u32;
place!(Field::<u32>(Variant(_5, 1), 1)) = Field::<u32>(Variant(_9, 1), 1);
RET = -607856967274201705_i64;
_15 = [27728848951292124765957770421709841124_i128,(-42446836228321776666228676447539845344_i128),(-40848094417970051906210224605777687071_i128),57180213396403617293381020179079460908_i128,5305578953031933523105565999340530788_i128,(-5350570541329875151900051347630459639_i128)];
_1 = &place!(Field::<[isize; 7]>(Variant(_9, 1), 0));
SetDiscriminant(_5, 1);
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_15 = [Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0,Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).0,Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0];
RET = 5631084318986953788_i64 + (-2026549399573822626_i64);
_21 = &place!(Field::<*mut usize>(Variant(_5, 2), 4));
_7 = _11;
_7 = _11;
_20 = 1394373630_i32 as isize;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3);
_16 = _12 - _12;
_11 = _20;
_22 = _13 ^ _13;
_24 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).0;
_3 = &_12;
_21 = &(*_21);
RET = 14746076603378351635_usize as i64;
_21 = &(*_21);
Call(place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.3 = fn15(Move(_3), _12, Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).3, (*_3), Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0, _8, _16, Field::<[i32; 6]>(Variant(_5, 2), 0), _18), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7 = 1908824278_i32 as isize;
_24 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).0 - Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).0;
_7 = _18;
_4 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).3;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)) = (Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0, Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.1, Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.2, Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.3);
_4 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.3;
Goto(bb9)
}
bb9 = {
_22 = _13;
_18 = -_20;
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = !Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).1;
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).2 = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.2 << Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0;
Goto(bb10)
}
bb10 = {
_27.0 = 18_u8 as f32;
place!(Field::<[i32; 6]>(Variant(_5, 2), 0)) = [1363989708_i32,224540736_i32,93193020_i32,166462913_i32,461766505_i32,(-1935738131_i32)];
place!(Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3)).0 = 718456084_i32 as i128;
_24 = -Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).0;
Call(_14 = fn17(_16, Field::<u32>(Variant(_5, 2), 1), Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).2, _12, _20, Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).2, Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.0, _20), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = _18;
SetDiscriminant(Field::<Adt35>(Variant(_14.0, 1), 0), 3);
_28 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).1 as f32;
place!(Field::<Adt40>(Variant(_14.0, 1), 1)) = Adt40 { fld0: _13,fld1: _16,fld2: _24 };
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.1 = Field::<(i128, u32, u16, char)>(Variant(_5, 2), 3).1 * Field::<u32>(Variant(_5, 2), 1);
Goto(bb12)
}
bb12 = {
place!(Field::<Adt40>(Variant(_14.0, 1), 1)).fld1 = _16 - _16;
_26 = [(-19886_i16),29985_i16,(-16074_i16),22539_i16];
RET = !4309032372802356046_i64;
place!(Field::<u8>(Variant(place!(Field::<Adt35>(Variant(_14.0, 1), 0)), 3), 1)) = Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5).2.1 as u8;
_27 = (_28,);
_29 = _11;
_5 = Adt67::Variant0 { fld0: Field::<Adt40>(Variant(_14.0, 1), 1),fld1: 28933_i16,fld2: Field::<Adt40>(Variant(_14.0, 1), 1).fld2 };
place!(Field::<Adt40>(Variant(_5, 0), 0)).fld0 = !Field::<Adt40>(Variant(_14.0, 1), 1).fld0;
_11 = 69875770996076861_u64 as isize;
_18 = _11 * _20;
_27 = (Field::<Adt40>(Variant(_14.0, 1), 1).fld1,);
_9 = Adt67::Variant0 { fld0: Field::<Adt40>(Variant(_5, 0), 0),fld1: 20820_i16,fld2: _24 };
place!(Field::<Adt40>(Variant(_14.0, 1), 1)).fld2 = Field::<f64>(Variant(_5, 0), 2) * _24;
_3 = &_28;
place!(Field::<i32>(Variant(place!(Field::<Adt35>(Variant(_14.0, 1), 0)), 3), 0)) = (-933937346_i32);
_15 = [22436145341118151252724221229461275188_i128,66984443380130331704508968789377100984_i128,102387498925378896757677785613110179795_i128,(-81363880364669187513565285890173299849_i128),(-107145469267671587237445167832091620661_i128),92268768684551038342730148958095392275_i128];
Goto(bb13)
}
bb13 = {
_18 = 6_i8 as isize;
_32 = &_19.0;
place!(Field::<f64>(Variant(_5, 0), 2)) = 3097210903_u32 as f64;
_33 = Field::<u8>(Variant(Field::<Adt35>(Variant(_14.0, 1), 0), 3), 1) ^ Field::<u8>(Variant(Field::<Adt35>(Variant(_14.0, 1), 0), 3), 1);
place!(Field::<Adt40>(Variant(_14.0, 1), 1)) = Field::<Adt40>(Variant(_9, 0), 0);
place!(Field::<Adt40>(Variant(_5, 0), 0)).fld2 = -Field::<f64>(Variant(_5, 0), 2);
_35 = _24 * Field::<Adt40>(Variant(_9, 0), 0).fld2;
SetDiscriminant(Field::<Adt35>(Variant(_14.0, 1), 0), 2);
_34 = Adt35::Variant1 { fld0: 523035009_u32,fld1: _4 };
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(_14.0, 1), 0)), 2), 1)) = 14105949206429627832_usize as i64;
place!(Field::<i16>(Variant(_5, 0), 1)) = 11430_i16;
place!(Field::<Adt40>(Variant(_14.0, 1), 1)).fld2 = _6 as f64;
_35 = -Field::<Adt40>(Variant(_5, 0), 0).fld2;
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(_14.0, 1), 0)), 2), 1)) = -RET;
place!(Field::<Adt35>(Variant(_14.0, 1), 0)) = Adt35::Variant3 { fld0: 87298358_i32,fld1: _33 };
match Field::<i16>(Variant(_5, 0), 1) {
11430 => bb14,
_ => bb9
}
}
bb14 = {
SetDiscriminant(_5, 2);
place!(Field::<(f64, [i64; 3], (i128, u32, u16, char))>(Variant(_5, 2), 5)).2.3 = _8;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(14_usize, 6_usize, Move(_6), 4_usize, Move(_4), 11_usize, Move(_11), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(14_usize, 26_usize, Move(_26), 15_usize, Move(_15), 39_usize, _39, 39_usize, _39), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static f32,mut _2: f32,mut _3: char,mut _4: f32,mut _5: i128,mut _6: char,mut _7: f32,mut _8: [i32; 6],mut _9: isize) -> char {
mir! {
type RET = char;
let _10: isize;
let _11: char;
let _12: &'static f64;
let _13: [i16; 4];
let _14: i128;
let _15: u16;
let _16: usize;
let _17: [i16; 4];
let _18: (f32,);
let _19: (&'static *mut u64,);
let _20: &'static *mut usize;
let _21: bool;
let _22: f32;
let _23: &'static [i16; 1];
let _24: i64;
let _25: ((usize, Adt21),);
let _26: ();
let _27: ();
{
_5 = 241457622751790943721620074405560611925_u128 as i128;
_2 = 14236_i16 as f32;
_9 = !(-121_isize);
_5 = 24946_u16 as i128;
_4 = _7;
Goto(bb1)
}
bb1 = {
RET = _3;
_8 = [(-1748419617_i32),1174671219_i32,613861440_i32,376930572_i32,1060907971_i32,(-1176946458_i32)];
_7 = _4;
_2 = _7 * _7;
_11 = _3;
_5 = (-150748343442757830886142433317093210896_i128);
_1 = &_7;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
189534023478180632577232174114675000560 => bb8,
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
RET = _3;
_11 = _3;
match _5 {
0 => bb6,
1 => bb4,
2 => bb9,
189534023478180632577232174114675000560 => bb11,
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
_8 = [80665651_i32,2020785131_i32,(-1769304065_i32),962868337_i32,143043735_i32,(-2052046272_i32)];
_10 = !_9;
_2 = _10 as f32;
_10 = _9 << _5;
RET = _3;
_3 = RET;
RET = _6;
_8 = [1393127098_i32,1543021910_i32,(-494067452_i32),615861718_i32,695885593_i32,(-162953354_i32)];
RET = _3;
_8 = [1070480057_i32,(-1631442499_i32),(-130989561_i32),(-911972808_i32),(-1631462077_i32),1823244749_i32];
_4 = 5353343938565353057_u64 as f32;
_6 = _3;
_1 = &_2;
_5 = -130259146815332833015854499743570835444_i128;
Call(_8 = fn16(_11, _11, _7, _10, _6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7 = _4 * _4;
_10 = -_9;
_9 = (-64_i8) as isize;
RET = _11;
_8 = [1705131499_i32,(-1704652465_i32),(-1407007733_i32),(-2089043311_i32),310282291_i32,1846165065_i32];
_9 = _10;
RET = _6;
_10 = _9 - _9;
_8 = [(-1943562172_i32),1558904859_i32,1815671902_i32,(-372741424_i32),1665456701_i32,(-216899881_i32)];
_5 = (-164635002506541669891959665852266503600_i128);
_10 = !_9;
_13 = [(-29600_i16),(-20101_i16),(-23628_i16),6906_i16];
_14 = 528847243_u32 as i128;
RET = _6;
RET = _6;
RET = _3;
_5 = 956667883_u32 as i128;
_10 = _9 * _9;
RET = _11;
_1 = &(*_1);
_2 = _7 * _7;
_1 = &_7;
Goto(bb13)
}
bb13 = {
_10 = -_9;
RET = _3;
_18 = (_2,);
_17 = [(-16604_i16),32708_i16,(-3451_i16),20353_i16];
_2 = _18.0;
_18.0 = -(*_1);
_16 = 2_usize;
match _16 {
0 => bb8,
1 => bb9,
3 => bb4,
4 => bb5,
5 => bb11,
6 => bb14,
2 => bb16,
_ => bb15
}
}
bb14 = {
RET = _3;
_11 = _3;
match _5 {
0 => bb6,
1 => bb4,
2 => bb9,
189534023478180632577232174114675000560 => bb11,
_ => bb10
}
}
bb15 = {
_8 = [80665651_i32,2020785131_i32,(-1769304065_i32),962868337_i32,143043735_i32,(-2052046272_i32)];
_10 = !_9;
_2 = _10 as f32;
_10 = _9 << _5;
RET = _3;
_3 = RET;
RET = _6;
_8 = [1393127098_i32,1543021910_i32,(-494067452_i32),615861718_i32,695885593_i32,(-162953354_i32)];
RET = _3;
_8 = [1070480057_i32,(-1631442499_i32),(-130989561_i32),(-911972808_i32),(-1631462077_i32),1823244749_i32];
_4 = 5353343938565353057_u64 as f32;
_6 = _3;
_1 = &_2;
_5 = -130259146815332833015854499743570835444_i128;
Call(_8 = fn16(_11, _11, _7, _10, _6), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
RET = _11;
_3 = _6;
_4 = _5 as f32;
_21 = true;
_4 = _16 as f32;
_21 = true;
_2 = _4 - (*_1);
_3 = RET;
_3 = RET;
_14 = !_5;
_5 = _14 << _8[_16];
_10 = _9;
_18.0 = _7 - _2;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(15_usize, 21_usize, Move(_21), 13_usize, Move(_13), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(15_usize, 5_usize, Move(_5), 14_usize, Move(_14), 27_usize, _27, 27_usize, _27), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: char,mut _2: char,mut _3: f32,mut _4: isize,mut _5: char) -> [i32; 6] {
mir! {
type RET = [i32; 6];
let _6: i32;
let _7: &'static *mut usize;
let _8: char;
let _9: *const &'static f32;
let _10: isize;
let _11: u128;
let _12: isize;
let _13: (Adt35, &'static *mut u64, u128);
let _14: f64;
let _15: char;
let _16: i8;
let _17: &'static f32;
let _18: (Adt70, (usize, Adt21));
let _19: Adt21;
let _20: char;
let _21: ();
let _22: ();
{
RET = [(-1564920368_i32),1315643585_i32,(-2133088700_i32),1924780917_i32,57267607_i32,29807982_i32];
_5 = _2;
RET = [428806266_i32,445965156_i32,(-1870066361_i32),(-1537581818_i32),(-931164327_i32),1198865573_i32];
_3 = 213_u8 as f32;
_4 = (-4319070295948209350_i64) as isize;
_5 = _1;
_3 = (-532366427_i32) as f32;
_4 = (-25_isize) ^ (-9223372036854775808_isize);
_2 = _1;
Goto(bb1)
}
bb1 = {
_2 = _1;
_1 = _5;
_6 = 6770_i16 as i32;
_1 = _2;
_2 = _1;
_4 = 9223372036854775807_isize;
RET = [_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6];
_1 = _2;
_6 = (-1776920326_i32);
_4 = 9223372036854775807_isize;
_6 = _5 as i32;
_1 = _2;
_1 = _2;
_2 = _1;
RET = [_6,_6,_6,_6,_6,_6];
_6 = 55220_u16 as i32;
RET = [_6,_6,_6,_6,_6,_6];
_3 = 33289_u16 as f32;
_3 = 67_i8 as f32;
_2 = _1;
Goto(bb2)
}
bb2 = {
_1 = _5;
RET = [_6,_6,_6,_6,_6,_6];
_5 = _2;
_3 = 1456982910_u32 as f32;
_10 = -_4;
_5 = _2;
_6 = 856426017_i32;
_6 = 1289323338_i32 | (-1114610309_i32);
_8 = _1;
_10 = -_4;
_8 = _1;
_11 = 104522252651994319491950519615923242946_u128 << _4;
_5 = _2;
_6 = (-613940122_i32);
RET = [_6,_6,_6,_6,_6,_6];
_5 = _1;
RET = [_6,_6,_6,_6,_6,_6];
_10 = 35781_u16 as isize;
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
9223372036854775807 => bb11,
_ => bb10
}
}
bb3 = {
_2 = _1;
_1 = _5;
_6 = 6770_i16 as i32;
_1 = _2;
_2 = _1;
_4 = 9223372036854775807_isize;
RET = [_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6];
_1 = _2;
_6 = (-1776920326_i32);
_4 = 9223372036854775807_isize;
_6 = _5 as i32;
_1 = _2;
_1 = _2;
_2 = _1;
RET = [_6,_6,_6,_6,_6,_6];
_6 = 55220_u16 as i32;
RET = [_6,_6,_6,_6,_6,_6];
_3 = 33289_u16 as f32;
_3 = 67_i8 as f32;
_2 = _1;
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
_8 = _2;
_5 = _1;
_11 = !238714957346913606710115910124793201399_u128;
RET = [_6,_6,_6,_6,_6,_6];
_3 = 90914552516666032569255484021723869074_i128 as f32;
_13.2 = _11 & _11;
_2 = _1;
_6 = !1323903698_i32;
_1 = _8;
_12 = -_10;
_13.2 = !_11;
Goto(bb12)
}
bb12 = {
_15 = _8;
_3 = _11 as f32;
RET = [_6,_6,_6,_6,_6,_6];
_13.2 = _11;
_15 = _8;
_11 = 17_u8 as u128;
_14 = _4 as f64;
_12 = _10 | _10;
_10 = 34921_u16 as isize;
_6 = 9717798148102480106_usize as i32;
_14 = 3354178027796704229_u64 as f64;
_1 = _8;
Goto(bb13)
}
bb13 = {
_8 = _1;
_10 = _12;
_2 = _8;
RET = [_6,_6,_6,_6,_6,_6];
_16 = (-60915151493485697465456944966662577283_i128) as i8;
_6 = 36418_u16 as i32;
_6 = (-1746549782_i32);
_15 = _1;
RET = [_6,_6,_6,_6,_6,_6];
_12 = -_4;
_14 = 139_u8 as f64;
Goto(bb14)
}
bb14 = {
_17 = &_3;
_10 = 22079_u16 as isize;
_17 = &(*_17);
_10 = _12;
_13.0 = Adt35::Variant3 { fld0: _6,fld1: 7_u8 };
_18.1.0 = 0_usize & 6_usize;
place!(Field::<u8>(Variant(_13.0, 3), 1)) = 249_u8;
_12 = 62956371955325109022939082376074413330_i128 as isize;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(16_usize, 11_usize, Move(_11), 1_usize, Move(_1), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(16_usize, 15_usize, Move(_15), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f32,mut _2: u32,mut _3: u16,mut _4: f32,mut _5: isize,mut _6: u16,mut _7: i128,mut _8: isize) -> (Adt70,) {
mir! {
type RET = (Adt70,);
let _9: f32;
let _10: f64;
let _11: f32;
let _12: [isize; 7];
let _13: &'static [usize; 4];
let _14: &'static i128;
let _15: *mut usize;
let _16: &'static f32;
let _17: f32;
let _18: Adt35;
let _19: *const &'static f32;
let _20: Adt21;
let _21: Adt40;
let _22: &'static &'static [isize; 7];
let _23: char;
let _24: i32;
let _25: (&'static i64,);
let _26: u8;
let _27: *const u32;
let _28: u8;
let _29: f64;
let _30: isize;
let _31: isize;
let _32: [bool; 7];
let _33: [u64; 6];
let _34: isize;
let _35: &'static [usize; 4];
let _36: &'static *mut usize;
let _37: [usize; 4];
let _38: u8;
let _39: Adt33;
let _40: Adt77;
let _41: isize;
let _42: f32;
let _43: Adt36;
let _44: bool;
let _45: (u8, u64);
let _46: (f64, [i64; 3], (i128, u32, u16, char));
let _47: [i128; 4];
let _48: f64;
let _49: &'static [usize; 4];
let _50: i64;
let _51: [i128; 4];
let _52: f64;
let _53: usize;
let _54: ();
let _55: ();
{
_7 = (-55339232908551247419977339668658288196_i128);
_4 = _1;
_7 = !(-63504637777777044522804821578430339146_i128);
_1 = -_4;
_7 = 23550934862241243820469064908520469376_i128 ^ (-145711841571617601142680856752773068488_i128);
_1 = _7 as f32;
_8 = _5;
_9 = _4;
_7 = 20175_i16 as i128;
_8 = _5;
_9 = _1;
_9 = -_1;
_5 = -_8;
_7 = 15803263296722843909_u64 as i128;
_4 = -_9;
_8 = _5 | _5;
_7 = _2 as i128;
_2 = !824435413_u32;
_6 = _3 << _5;
_4 = -_1;
_2 = 2065121335_u32 ^ 4288676887_u32;
_4 = _9;
_8 = false as isize;
_9 = _1;
_3 = !_6;
_3 = _6;
_11 = 4_usize as f32;
Call(_1 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !_3;
_2 = 49_u8 as u32;
_8 = 16354560045318209234_u64 as isize;
_6 = _3 << _8;
_9 = _11 - _4;
_1 = _4;
_12 = [_5,_8,_5,_8,_8,_5,_8];
_10 = _5 as f64;
_2 = !2221310275_u32;
_5 = _8;
_14 = &_7;
_10 = _5 as f64;
_2 = 929342510_u32;
_17 = _9;
_12 = [_8,_5,_5,_5,_8,_5,_5];
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
929342510 => bb10,
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
_9 = _10 as f32;
_11 = _1;
_17 = _1 * _1;
_7 = 153345111108059634665903258232083528270_i128 << _3;
_11 = (-19512_i16) as f32;
_6 = !_3;
_16 = &_9;
_9 = _17 * _4;
_21.fld2 = _10;
_5 = _8;
_2 = 1227741631_u32 >> _3;
_21.fld1 = _21.fld2 as f32;
_16 = &_21.fld1;
_21 = Adt40 { fld0: false,fld1: _17,fld2: _10 };
_3 = _6 ^ _6;
_10 = -_21.fld2;
_21.fld2 = _10 + _10;
_14 = &_7;
_23 = '\u{23ac0}';
_4 = -_9;
_5 = -_8;
_10 = _21.fld2 * _21.fld2;
_9 = -_4;
Goto(bb11)
}
bb11 = {
_21.fld2 = _5 as f64;
_12 = [_5,_5,_8,_8,_8,_5,_8];
_21 = Adt40 { fld0: true,fld1: _17,fld2: _10 };
_4 = -_9;
_27 = core::ptr::addr_of!(_2);
_16 = &_17;
_18 = Adt35::Variant2 { fld0: 17993779340149599666_u64,fld1: (-7005954061636554649_i64),fld2: _10 };
_5 = _8;
_9 = (-2069469606_i32) as f32;
_12 = [_8,_5,_5,_5,_5,_8,_5];
_21 = Adt40 { fld0: false,fld1: _1,fld2: Field::<f64>(Variant(_18, 2), 2) };
_24 = _8 as i32;
_12 = [_5,_5,_8,_8,_5,_8,_8];
_21.fld0 = !false;
_27 = core::ptr::addr_of!((*_27));
_4 = -(*_16);
_9 = _4 - _17;
(*_27) = 3624392103_u32 - 402942641_u32;
_17 = _4 + _1;
_21.fld0 = !false;
_12 = [_8,_5,_8,_8,_8,_5,_8];
_21.fld1 = _9;
_10 = _21.fld2;
_21.fld2 = _10;
_12 = [_8,_8,_8,_8,_5,_5,_5];
(*_27) = 4164382347_u32;
_7 = (-8213730477834420757082348579475378561_i128);
place!(Field::<u64>(Variant(_18, 2), 0)) = 17798207615561923548_u64 ^ 2142017236864483906_u64;
Goto(bb12)
}
bb12 = {
_19 = core::ptr::addr_of!(_16);
Goto(bb13)
}
bb13 = {
(*_19) = &_1;
_7 = -(-161926517796740778144162033946152956818_i128);
(*_27) = 4195943384_u32;
_7 = 19730087470888548919901028496221894708_i128;
_30 = _8;
_4 = (*_16) - _9;
_26 = 215_u8 + 138_u8;
_19 = core::ptr::addr_of!(_16);
_21.fld1 = _1 * _1;
_30 = _5;
_16 = &(*_16);
_9 = _4 * (*_16);
_34 = _8 ^ _30;
_32 = [_21.fld0,_21.fld0,_21.fld0,_21.fld0,_21.fld0,_21.fld0,_21.fld0];
_29 = Field::<u64>(Variant(_18, 2), 0) as f64;
_21.fld1 = Field::<f64>(Variant(_18, 2), 2) as f32;
_8 = _21.fld0 as isize;
Goto(bb14)
}
bb14 = {
place!(Field::<f64>(Variant(_18, 2), 2)) = _29 - _10;
_17 = -_4;
_36 = &_15;
_28 = _26;
_21.fld1 = Field::<u64>(Variant(_18, 2), 0) as f32;
_27 = core::ptr::addr_of!((*_27));
_36 = &(*_36);
_24 = 1042548219_i32 << _6;
_7 = _24 as i128;
_13 = &_37;
_18 = Adt35::Variant3 { fld0: _24,fld1: _26 };
_33 = [961691269748963989_u64,2658403022307509273_u64,5865759176073886386_u64,7327081826357303534_u64,15061735511186959993_u64,13215967487473352628_u64];
_17 = -_4;
_26 = _23 as u8;
_31 = _10 as isize;
(*_27) = !1283262353_u32;
_35 = &_37;
_14 = &_7;
_21.fld1 = _7 as f32;
(*_19) = &_4;
place!(Field::<i32>(Variant(_18, 3), 0)) = _24;
_34 = (-25_i8) as isize;
_8 = _34 >> _3;
_36 = &(*_36);
_38 = Field::<u8>(Variant(_18, 3), 1);
_16 = &(*_16);
(*_27) = 3255444937_u32 * 3270985241_u32;
Goto(bb15)
}
bb15 = {
RET.0 = Adt70::Variant1 { fld0: Move(_18),fld1: _21 };
_34 = _8;
_21.fld2 = Field::<Adt40>(Variant(RET.0, 1), 1).fld2 - Field::<Adt40>(Variant(RET.0, 1), 1).fld2;
_8 = -_34;
_27 = core::ptr::addr_of!(_2);
_37 = [1_usize,7_usize,3858871951224441133_usize,2_usize];
SetDiscriminant(Field::<Adt35>(Variant(RET.0, 1), 0), 2);
_20 = Adt21::Variant0 { fld0: 14526680706687811103_usize,fld1: _32,fld2: _38,fld3: 88_i8,fld4: 144682643863333000870320483911306961287_u128,fld5: _29,fld6: (-720329334022735445_i64) };
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = _21.fld2 * _21.fld2;
_16 = &(*_16);
place!(Field::<i8>(Variant(_20, 0), 3)) = _21.fld2 as i8;
Goto(bb16)
}
bb16 = {
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 1)) = Field::<i8>(Variant(_20, 0), 3) as i64;
place!(Field::<usize>(Variant(_20, 0), 0)) = 6151992481533725376_usize;
place!(Field::<i64>(Variant(_20, 0), 6)) = Field::<Adt40>(Variant(RET.0, 1), 1).fld2 as i64;
_18 = Adt35::Variant2 { fld0: 11215460353048467384_u64,fld1: Field::<i64>(Variant(_20, 0), 6),fld2: Field::<Adt40>(Variant(RET.0, 1), 1).fld2 };
_31 = Field::<i64>(Variant(_18, 2), 1) as isize;
_31 = -_34;
_27 = core::ptr::addr_of!(_2);
(*_27) = 3010371433_u32 - 4071959024_u32;
_39.fld1 = [Field::<Adt40>(Variant(RET.0, 1), 1).fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0];
_35 = &_37;
_8 = Field::<i8>(Variant(_20, 0), 3) as isize;
place!(Field::<f64>(Variant(_20, 0), 5)) = 48176283766317772757452740939364242782_u128 as f64;
match Field::<usize>(Variant(_20, 0), 0) {
0 => bb7,
6151992481533725376 => bb17,
_ => bb4
}
}
bb17 = {
_17 = _9;
_36 = &(*_36);
_39.fld2 = Field::<i64>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 2), 1) as u16;
_3 = _6;
(*_27) = (*_16) as u32;
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = -_21.fld2;
place!(Field::<f64>(Variant(_18, 2), 2)) = -Field::<Adt40>(Variant(RET.0, 1), 1).fld2;
match Field::<usize>(Variant(_20, 0), 0) {
0 => bb8,
6151992481533725376 => bb19,
_ => bb18
}
}
bb18 = {
Return()
}
bb19 = {
_39.fld2 = !_3;
_16 = &_17;
place!(Field::<u8>(Variant(_20, 0), 2)) = !_26;
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 1)) = Field::<i64>(Variant(_18, 2), 1);
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld0 = _34 > _34;
_20 = Adt21::Variant2 { fld0: Field::<Adt40>(Variant(RET.0, 1), 1).fld0,fld1: _23,fld2: _2,fld3: _4,fld4: _6,fld5: _24 };
_19 = core::ptr::addr_of!((*_19));
_11 = 15234327137396428909_u64 as f32;
_39.fld5 = 4434788265548813446_u64 + 16907668902169253873_u64;
place!(Field::<Adt35>(Variant(RET.0, 1), 0)) = Adt35::Variant3 { fld0: Field::<i32>(Variant(_20, 2), 5),fld1: _38 };
_39.fld4 = (2_usize, Move(_20));
SetDiscriminant(_39.fld4.1, 1);
_16 = &_1;
_43.fld1 = [_34,_34,_34,_31,_8,_8,_34];
_43.fld0 = Adt21::Variant0 { fld0: _39.fld4.0,fld1: _32,fld2: _28,fld3: 48_i8,fld4: 323577100608384992503607289236218533105_u128,fld5: _10,fld6: Field::<i64>(Variant(_18, 2), 1) };
SetDiscriminant(Field::<Adt35>(Variant(RET.0, 1), 0), 2);
place!(Field::<u64>(Variant(_18, 2), 0)) = _39.fld5 & _39.fld5;
_12 = [_30,_31,_34,_31,_34,_31,_34];
_21 = Adt40 { fld0: Field::<Adt40>(Variant(RET.0, 1), 1).fld0,fld1: _9,fld2: Field::<Adt40>(Variant(RET.0, 1), 1).fld2 };
place!(Field::<usize>(Variant(_39.fld4.1, 1), 1)) = _39.fld4.0;
place!(Field::<u64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 0)) = _39.fld5 | _39.fld5;
_17 = _21.fld1;
_21.fld1 = (*_16);
SetDiscriminant(_18, 2);
_39.fld0 = Field::<Adt40>(Variant(RET.0, 1), 1).fld0;
Goto(bb20)
}
bb20 = {
_43.fld0 = Adt21::Variant1 { fld0: _39.fld0,fld1: _39.fld4.0,fld2: (*_27),fld3: _7,fld4: _39.fld1 };
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld0 = _39.fld0;
place!(Field::<i128>(Variant(_43.fld0, 1), 3)) = _7;
place!(Field::<Adt35>(Variant(RET.0, 1), 0)) = Adt35::Variant2 { fld0: _39.fld5,fld1: 4655457798675097479_i64,fld2: _21.fld2 };
_39.fld6 = [79_i16,(-22616_i16),11783_i16,25348_i16];
place!(Field::<[bool; 7]>(Variant(_39.fld4.1, 1), 4)) = [_39.fld0,Field::<bool>(Variant(_43.fld0, 1), 0),_39.fld0,_39.fld0,Field::<bool>(Variant(_43.fld0, 1), 0),Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_39.fld0];
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld0 = Field::<bool>(Variant(_43.fld0, 1), 0);
place!(Field::<f64>(Variant(_18, 2), 2)) = _10 + _21.fld2;
_46.2.2 = _39.fld2;
_13 = &_37;
_46.2.0 = _23 as i128;
_39.fld6 = [9204_i16,(-30071_i16),12885_i16,(-28830_i16)];
place!(Field::<i128>(Variant(_43.fld0, 1), 3)) = 7891261777189449368_i64 as i128;
_46.2 = (_7, (*_27), _6, _23);
_20 = Adt21::Variant0 { fld0: Field::<usize>(Variant(_43.fld0, 1), 1),fld1: Field::<[bool; 7]>(Variant(_39.fld4.1, 1), 4),fld2: _38,fld3: (-76_i8),fld4: 91230701504878214850463929396791989299_u128,fld5: _21.fld2,fld6: (-8876074369867131586_i64) };
_29 = -_21.fld2;
SetDiscriminant(_43.fld0, 0);
_46.1 = [(-195007437986346328_i64),4469294713914125396_i64,2166587295037513863_i64];
_24 = (-729697613_i32);
_36 = &(*_36);
_2 = _46.2.1 | _46.2.1;
_39.fld1 = [Field::<Adt40>(Variant(RET.0, 1), 1).fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_39.fld0,_39.fld0];
_17 = _4;
_14 = &place!(Field::<i128>(Variant(_39.fld4.1, 1), 3));
match _39.fld4.0 {
0 => bb4,
2 => bb21,
_ => bb19
}
}
bb21 = {
_46.1 = [695477011173382509_i64,8476823357604314494_i64,(-6831606684871300968_i64)];
_5 = _34 - _34;
_6 = _46.2.2 * _3;
_45 = (_28, _39.fld5);
_2 = Field::<u64>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 2), 0) as u32;
_17 = -_21.fld1;
place!(Field::<usize>(Variant(_20, 0), 0)) = _39.fld4.0;
place!(Field::<i8>(Variant(_43.fld0, 0), 3)) = 86_i8;
place!(Field::<usize>(Variant(_20, 0), 0)) = _21.fld2 as usize;
_3 = _6 + _6;
_32 = Field::<[bool; 7]>(Variant(_20, 0), 1);
_35 = &(*_35);
_46.2 = (_7, _2, _39.fld2, _23);
_25.0 = &place!(Field::<i64>(Variant(_43.fld0, 0), 6));
place!(Field::<[bool; 7]>(Variant(_43.fld0, 0), 1)) = [_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_39.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_39.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0];
_39.fld0 = !_21.fld0;
_45 = (_38, Field::<u64>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 2), 0));
_44 = !Field::<Adt40>(Variant(RET.0, 1), 1).fld0;
_39.fld1 = Field::<[bool; 7]>(Variant(_43.fld0, 0), 1);
place!(Field::<i8>(Variant(_20, 0), 3)) = !Field::<i8>(Variant(_43.fld0, 0), 3);
_20 = Adt21::Variant1 { fld0: _21.fld0,fld1: Field::<usize>(Variant(_39.fld4.1, 1), 1),fld2: _46.2.1,fld3: _7,fld4: _32 };
_21.fld1 = _9 * Field::<Adt40>(Variant(RET.0, 1), 1).fld1;
place!(Field::<f64>(Variant(_43.fld0, 0), 5)) = _21.fld2;
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 1)) = (-1339255836285932452_i64);
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld1 = _21.fld1 * _9;
place!(Field::<u64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 0)) = _46.2.0 as u64;
_46.2.3 = _23;
(*_19) = &_4;
_43 = Adt36 { fld0: Move(_20),fld1: _12 };
SetDiscriminant(Field::<Adt35>(Variant(RET.0, 1), 0), 1);
Goto(bb22)
}
bb22 = {
_50 = -1377858420964662248_i64;
SetDiscriminant(_43.fld0, 0);
_14 = &_46.2.0;
place!(Field::<u32>(Variant(_39.fld4.1, 1), 2)) = _28 as u32;
_46.2.3 = _23;
place!(Field::<i64>(Variant(_18, 2), 1)) = -_50;
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = 237619177130938359012086292831734025780_u128 as f64;
_46.0 = -_29;
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld1 = -_21.fld1;
_18 = Adt35::Variant2 { fld0: _45.1,fld1: _50,fld2: _46.0 };
_42 = (*_16) + (*_16);
_36 = &_15;
_14 = &_46.2.0;
_47 = [(*_14),(*_14),(*_14),_7];
match _39.fld4.0 {
0 => bb23,
1 => bb24,
3 => bb26,
4 => bb27,
5 => bb28,
6 => bb29,
2 => bb31,
_ => bb30
}
}
bb23 = {
Return()
}
bb24 = {
_19 = core::ptr::addr_of!(_16);
Goto(bb13)
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
_17 = _9;
_36 = &(*_36);
_39.fld2 = Field::<i64>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 2), 1) as u16;
_3 = _6;
(*_27) = (*_16) as u32;
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = -_21.fld2;
place!(Field::<f64>(Variant(_18, 2), 2)) = -Field::<Adt40>(Variant(RET.0, 1), 1).fld2;
match Field::<usize>(Variant(_20, 0), 0) {
0 => bb8,
6151992481533725376 => bb19,
_ => bb18
}
}
bb28 = {
place!(Field::<i64>(Variant(place!(Field::<Adt35>(Variant(RET.0, 1), 0)), 2), 1)) = Field::<i8>(Variant(_20, 0), 3) as i64;
place!(Field::<usize>(Variant(_20, 0), 0)) = 6151992481533725376_usize;
place!(Field::<i64>(Variant(_20, 0), 6)) = Field::<Adt40>(Variant(RET.0, 1), 1).fld2 as i64;
_18 = Adt35::Variant2 { fld0: 11215460353048467384_u64,fld1: Field::<i64>(Variant(_20, 0), 6),fld2: Field::<Adt40>(Variant(RET.0, 1), 1).fld2 };
_31 = Field::<i64>(Variant(_18, 2), 1) as isize;
_31 = -_34;
_27 = core::ptr::addr_of!(_2);
(*_27) = 3010371433_u32 - 4071959024_u32;
_39.fld1 = [Field::<Adt40>(Variant(RET.0, 1), 1).fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,_21.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0];
_35 = &_37;
_8 = Field::<i8>(Variant(_20, 0), 3) as isize;
place!(Field::<f64>(Variant(_20, 0), 5)) = 48176283766317772757452740939364242782_u128 as f64;
match Field::<usize>(Variant(_20, 0), 0) {
0 => bb7,
6151992481533725376 => bb17,
_ => bb4
}
}
bb29 = {
RET.0 = Adt70::Variant1 { fld0: Move(_18),fld1: _21 };
_34 = _8;
_21.fld2 = Field::<Adt40>(Variant(RET.0, 1), 1).fld2 - Field::<Adt40>(Variant(RET.0, 1), 1).fld2;
_8 = -_34;
_27 = core::ptr::addr_of!(_2);
_37 = [1_usize,7_usize,3858871951224441133_usize,2_usize];
SetDiscriminant(Field::<Adt35>(Variant(RET.0, 1), 0), 2);
_20 = Adt21::Variant0 { fld0: 14526680706687811103_usize,fld1: _32,fld2: _38,fld3: 88_i8,fld4: 144682643863333000870320483911306961287_u128,fld5: _29,fld6: (-720329334022735445_i64) };
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = _21.fld2 * _21.fld2;
_16 = &(*_16);
place!(Field::<i8>(Variant(_20, 0), 3)) = _21.fld2 as i8;
Goto(bb16)
}
bb30 = {
Return()
}
bb31 = {
_39.fld1 = [_21.fld0,_44,Field::<Adt40>(Variant(RET.0, 1), 1).fld0,_21.fld0,_44,_39.fld0,Field::<Adt40>(Variant(RET.0, 1), 1).fld0];
_35 = Move(_13);
place!(Field::<i64>(Variant(_18, 2), 1)) = _45.0 as i64;
_39.fld5 = _45.1 << _6;
_45.1 = _7 as u64;
_6 = _3;
place!(Field::<f64>(Variant(_18, 2), 2)) = _10;
_39.fld4.1 = Adt21::Variant0 { fld0: _39.fld4.0,fld1: _32,fld2: _45.0,fld3: 38_i8,fld4: 17808474452427403068135957983638412332_u128,fld5: _46.0,fld6: Field::<i64>(Variant(_18, 2), 1) };
_39.fld4.1 = Adt21::Variant0 { fld0: _39.fld4.0,fld1: _39.fld1,fld2: _26,fld3: 97_i8,fld4: 232671252308480099871321870462919034491_u128,fld5: _29,fld6: Field::<i64>(Variant(_18, 2), 1) };
_13 = &_37;
place!(Field::<i8>(Variant(_43.fld0, 0), 3)) = -(-100_i8);
match Field::<usize>(Variant(_39.fld4.1, 0), 0) {
0 => bb1,
1 => bb17,
3 => bb11,
4 => bb25,
5 => bb14,
6 => bb7,
2 => bb33,
_ => bb32
}
}
bb32 = {
Return()
}
bb33 = {
_33 = [_45.1,_45.1,_39.fld5,_39.fld5,_45.1,_39.fld5];
(*_19) = &_21.fld1;
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld2 = -_29;
_49 = Move(_13);
_53 = Field::<usize>(Variant(_39.fld4.1, 0), 0) >> _3;
_46.2.1 = !(*_27);
_39.fld6 = [23828_i16,19371_i16,7390_i16,(-4256_i16)];
_41 = _31 * _34;
place!(Field::<Adt35>(Variant(RET.0, 1), 0)) = Adt35::Variant1 { fld0: (*_27),fld1: _46.2.3 };
_51 = [_7,(*_14),(*_14),_46.2.0];
_48 = _7 as f64;
_45 = (Field::<u8>(Variant(_39.fld4.1, 0), 2), _39.fld5);
place!(Field::<i64>(Variant(_18, 2), 1)) = _50 + Field::<i64>(Variant(_39.fld4.1, 0), 6);
_39.fld4.1 = Adt21::Variant1 { fld0: _39.fld0,fld1: _39.fld4.0,fld2: Field::<u32>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 1), 0),fld3: (*_14),fld4: _32 };
place!(Field::<u64>(Variant(_18, 2), 0)) = 228193966974636034412118159918303461477_u128 as u64;
_16 = &(*_16);
_38 = !_28;
_20 = Move(_39.fld4.1);
place!(Field::<bool>(Variant(_20, 1), 0)) = _21.fld0;
_39.fld4.1 = Adt21::Variant1 { fld0: _39.fld0,fld1: Field::<usize>(Variant(_20, 1), 1),fld2: Field::<u32>(Variant(Field::<Adt35>(Variant(RET.0, 1), 0), 1), 0),fld3: (*_14),fld4: _32 };
place!(Field::<Adt40>(Variant(RET.0, 1), 1)).fld0 = Field::<usize>(Variant(_20, 1), 1) != _39.fld4.0;
_46.1 = [Field::<i64>(Variant(_18, 2), 1),Field::<i64>(Variant(_18, 2), 1),Field::<i64>(Variant(_18, 2), 1)];
SetDiscriminant(_39.fld4.1, 1);
_35 = &_37;
_16 = &_17;
_42 = _1;
_21 = Field::<Adt40>(Variant(RET.0, 1), 1);
_27 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_39.fld4.1, 1), 2)));
_24 = 543215000_i32 + 908471742_i32;
_46.0 = Field::<i64>(Variant(_18, 2), 1) as f64;
_45 = (_38, _39.fld5);
Goto(bb34)
}
bb34 = {
Call(_54 = dump_var(17_usize, 44_usize, Move(_44), 38_usize, Move(_38), 23_usize, Move(_23), 26_usize, Move(_26)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_54 = dump_var(17_usize, 3_usize, Move(_3), 30_usize, Move(_30), 53_usize, Move(_53), 41_usize, Move(_41)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_54 = dump_var(17_usize, 5_usize, Move(_5), 8_usize, Move(_8), 47_usize, Move(_47), 34_usize, Move(_34)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: &'static *mut usize,mut _2: *const &'static *mut usize,mut _3: u64,mut _4: f64,mut _5: f64) -> usize {
mir! {
type RET = usize;
let _6: isize;
let _7: [u128; 1];
let _8: &'static Adt21;
let _9: &'static &'static u8;
let _10: [i16; 1];
let _11: [i16; 4];
let _12: (usize, Adt21);
let _13: &'static [isize; 7];
let _14: [i64; 3];
let _15: &'static [isize; 7];
let _16: [u16; 7];
let _17: isize;
let _18: [u64; 6];
let _19: [usize; 1];
let _20: i128;
let _21: ((usize, Adt21),);
let _22: i64;
let _23: &'static *mut u64;
let _24: (&'static *mut u64,);
let _25: (u8, u64);
let _26: [i16; 4];
let _27: bool;
let _28: f64;
let _29: char;
let _30: char;
let _31: (u8, u64);
let _32: (f64, [i64; 3], (i128, u32, u16, char));
let _33: (usize, *const &'static f32);
let _34: f32;
let _35: isize;
let _36: f32;
let _37: &'static &'static *mut u64;
let _38: u128;
let _39: (&'static f32, [i16; 1], *mut u64, &'static *mut u64);
let _40: (Adt70, (usize, Adt21));
let _41: f64;
let _42: i16;
let _43: &'static f64;
let _44: isize;
let _45: (i128, u32, u16, char);
let _46: [usize; 4];
let _47: u64;
let _48: [u128; 1];
let _49: Adt67;
let _50: (Adt40, *mut u64);
let _51: (usize, Adt21);
let _52: ();
let _53: ();
{
RET = 14789828233941759630_usize << _3;
_4 = _5 * _5;
_4 = -_5;
RET = 15620908426193803187_usize;
RET = 12192828356865442187_usize >> _3;
RET = 4_usize;
_4 = _5 - _5;
_2 = core::ptr::addr_of!(_1);
RET = 8308944367201385796_usize;
_3 = 1418344984_u32 as u64;
_2 = core::ptr::addr_of!((*_2));
_5 = _4;
RET = 6833233063387898224_usize;
_4 = _5 - _5;
_6 = (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_4 = _5;
_4 = RET as f64;
_4 = -_5;
_3 = 17134739672351508512_u64;
Call(_5 = core::intrinsics::fmaf64(_4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = core::ptr::addr_of!((*_2));
_5 = 54_u8 as f64;
_4 = -_5;
_2 = core::ptr::addr_of!(_1);
_4 = _3 as f64;
_6 = 9223372036854775807_isize | (-9223372036854775808_isize);
_6 = (-9223372036854775808_isize);
_6 = !(-119_isize);
_7 = [335360625874927360754770579429163939229_u128];
_6 = (-9223372036854775808_isize);
_6 = 214891603682574312243296154992908608797_u128 as isize;
RET = 8126676781119205980_usize - 14804480116398408830_usize;
Goto(bb3)
}
bb3 = {
RET = 5463362311653531953_usize;
RET = !10553682983522652481_usize;
_7 = [203816889042317227753918942283510483063_u128];
_4 = -_5;
_3 = 7289178682509851251_u64;
_3 = 1874788651610996690_u64;
_5 = _4;
_4 = -_5;
_4 = _5;
_4 = 38_u8 as f64;
_7 = [27320542895440297202341828927476822418_u128];
_3 = 8656958617608367479_u64 >> _6;
_10 = [(-61_i16)];
Goto(bb4)
}
bb4 = {
_11 = [29372_i16,(-16982_i16),1568_i16,(-31426_i16)];
_12.0 = !RET;
_5 = _4 * _4;
_7 = [105567639013648490908541998800701983819_u128];
_4 = -_5;
_5 = _3 as f64;
Goto(bb5)
}
bb5 = {
_11 = [(-6260_i16),14458_i16,13947_i16,1621_i16];
_8 = &_12.1;
RET = _12.0 + _12.0;
_14 = [(-1544510593893390242_i64),7465386727286095879_i64,7913572829629009888_i64];
_11 = [2598_i16,(-14463_i16),19606_i16,(-30587_i16)];
RET = _12.0;
_5 = (-44_i8) as f64;
_10 = [(-4974_i16)];
_5 = _4 - _4;
_12.0 = !RET;
_8 = &(*_8);
_2 = core::ptr::addr_of!((*_2));
Goto(bb6)
}
bb6 = {
_16 = [33487_u16,23775_u16,2021_u16,41010_u16,33555_u16,56860_u16,38308_u16];
_7 = [231447508630271473481156430065232842999_u128];
_10 = [16429_i16];
_19 = [RET];
RET = _12.0 >> _6;
RET = 107_u8 as usize;
_5 = _4 * _4;
_12.0 = RET | RET;
_4 = _5;
_20 = 31618989314367530079096189870355939032_i128;
_8 = &(*_8);
_2 = core::ptr::addr_of!(_1);
_14 = [(-4192302283167142810_i64),4800223050375262053_i64,(-7213734470116342832_i64)];
_16 = [57296_u16,4805_u16,58034_u16,5629_u16,17460_u16,7352_u16,38166_u16];
_20 = 152355364461194880590873305183684703901_i128 + 89119239676529136928793514761329663452_i128;
_10 = [29900_i16];
_20 = -163745181890335101873708261194049833357_i128;
_3 = 272335778644839231818703625715272662003_u128 as u64;
_18 = [_3,_3,_3,_3,_3,_3];
_2 = core::ptr::addr_of!(_1);
_21.0.0 = _12.0 * _12.0;
_18 = [_3,_3,_3,_3,_3,_3];
_21.0.0 = _12.0;
_22 = (-9183768260542182816_i64) * (-2944721016258181724_i64);
_19 = [RET];
Call(RET = core::intrinsics::bswap(_12.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = 10447454968816033610_u64 * 9025567160668988011_u64;
RET = _21.0.0 << _3;
_25.0 = 2745575581_u32 as u8;
_2 = core::ptr::addr_of!(_1);
_4 = -_5;
_12.0 = !RET;
_7 = [161791433412923853409797405527139668352_u128];
RET = _12.0;
_4 = _5 * _5;
_6 = (-101_isize);
_12.0 = RET;
_12.0 = !_21.0.0;
_10 = [(-6090_i16)];
_8 = &_12.1;
_25.1 = _3 + _3;
_20 = (-144389953387022153696868339945537047350_i128) ^ (-64850602113694180011484850937743286917_i128);
_8 = &(*_8);
Goto(bb8)
}
bb8 = {
_6 = 52917_u16 as isize;
_8 = &(*_8);
_14 = [_22,_22,_22];
_11 = [(-18632_i16),30590_i16,(-5808_i16),17019_i16];
_17 = 242046995428514248626701370755152546552_u128 as isize;
_21.0.0 = 135852252426680224264517854909699513057_u128 as usize;
_2 = core::ptr::addr_of!((*_2));
_8 = &(*_8);
_26 = [(-31930_i16),(-12919_i16),16329_i16,(-15833_i16)];
_7 = [89952077313580469990971037013992787733_u128];
_2 = core::ptr::addr_of!((*_2));
_16 = [21344_u16,11451_u16,52933_u16,42091_u16,2666_u16,8992_u16,61655_u16];
_21.0.0 = _12.0 & _12.0;
_11 = [11744_i16,(-10726_i16),8526_i16,(-11796_i16)];
_21.0.0 = _12.0 - RET;
_8 = &(*_8);
_16 = [45886_u16,39016_u16,909_u16,36070_u16,12763_u16,29212_u16,64507_u16];
_2 = core::ptr::addr_of!((*_2));
_7 = [68919685168544218608336402883958532173_u128];
Goto(bb9)
}
bb9 = {
_18 = [_3,_25.1,_3,_25.1,_25.1,_3];
Goto(bb10)
}
bb10 = {
_3 = 16503_i16 as u64;
RET = 3551027435_u32 as usize;
_22 = (-4017190283057386642_i64);
_30 = '\u{272f0}';
_22 = -(-4127292353702007354_i64);
_31.1 = _25.1 - _25.1;
RET = !_21.0.0;
_17 = -_6;
_30 = '\u{dac28}';
_32.1 = _14;
_2 = core::ptr::addr_of!(_1);
_16 = [24041_u16,58067_u16,37921_u16,36819_u16,4719_u16,12258_u16,62548_u16];
_32.0 = 8033_i16 as f64;
_18 = [_31.1,_31.1,_25.1,_31.1,_31.1,_31.1];
_2 = core::ptr::addr_of!((*_2));
_33.0 = _21.0.0 << _31.1;
_27 = !true;
_4 = _32.0 * _5;
_2 = core::ptr::addr_of!(_1);
_29 = _30;
_4 = -_5;
_20 = !9612517398335776165386322883866660573_i128;
_26 = _11;
Goto(bb11)
}
bb11 = {
_32.1 = [_22,_22,_22];
_18 = [_31.1,_31.1,_25.1,_25.1,_25.1,_31.1];
_14 = [_22,_22,_22];
_32.2.3 = _29;
_21.0.0 = _33.0;
_36 = _22 as f32;
_34 = -_36;
_37 = &_23;
_28 = -_5;
_32.2 = (_20, 1892703706_u32, 43340_u16, _30);
_30 = _32.2.3;
_10 = [9043_i16];
_32.2 = (_20, 3555073125_u32, 47823_u16, _30);
_31.1 = _25.1 + _25.1;
Goto(bb12)
}
bb12 = {
_24.0 = &_39.2;
_7 = [244934457648764023056300912389254842353_u128];
_37 = &_39.3;
_30 = _32.2.3;
Call(_33.0 = core::intrinsics::transmute(_21.0.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_36 = _34 + _34;
_7 = [120883164963842704630733594102193840990_u128];
RET = _36 as usize;
_26 = [10989_i16,(-1001_i16),(-11204_i16),(-20543_i16)];
Call(RET = core::intrinsics::transmute(_11), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_33.1 = core::ptr::addr_of!(_39.0);
_22 = !(-4242627381779094863_i64);
_23 = &_39.2;
_32.2 = (_20, 23252462_u32, 7808_u16, _30);
_4 = -_28;
_40.1.0 = _33.0;
_28 = _4 - _5;
_32.0 = _4;
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(18_usize, 3_usize, Move(_3), 27_usize, Move(_27), 26_usize, Move(_26), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(18_usize, 6_usize, Move(_6), 11_usize, Move(_11), 29_usize, Move(_29), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-1563744167_i32)), std::hint::black_box(17006954351682854313_usize), std::hint::black_box(212_u8));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: usize,
fld1: [bool; 7],
fld2: u8,
fld3: i8,
fld4: u128,
fld5: f64,
fld6: i64,

},
Variant1{
fld0: bool,
fld1: usize,
fld2: u32,
fld3: i128,
fld4: [bool; 7],

},
Variant2{
fld0: bool,
fld1: char,
fld2: u32,
fld3: f32,
fld4: u16,
fld5: i32,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: bool,
fld1: [bool; 7],
fld2: u16,
fld3: i8,
fld4: (usize, Adt21),
fld5: u64,
fld6: [i16; 4],
}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: [i128; 6],
fld1: usize,
fld2: isize,

},
Variant1{
fld0: u32,
fld1: char,

},
Variant2{
fld0: u64,
fld1: i64,
fld2: f64,

},
Variant3{
fld0: i32,
fld1: u8,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt36{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt36 {
fld0: Adt21,
fld1: [isize; 7],
}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: bool,
fld1: f32,
fld2: f64,
}
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt67 {
Variant0{
fld0: Adt40,
fld1: i16,
fld2: f64,

},
Variant1{
fld0: [isize; 7],
fld1: u32,

},
Variant2{
fld0: [i32; 6],
fld1: u32,
fld2: [i128; 6],
fld3: (i128, u32, u16, char),
fld4: *mut usize,
fld5: (f64, [i64; 3], (i128, u32, u16, char)),

}}
impl PrintFDebug for Adt70{
	unsafe fn printf_debug(&self){unsafe{printf("Adt70::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt70 {
Variant0{
fld0: [u16; 7],
fld1: (usize, Adt21),
fld2: [i16; 3],
fld3: *mut i8,
fld4: Adt40,
fld5: i32,
fld6: u128,
fld7: *mut usize,

},
Variant1{
fld0: Adt35,
fld1: Adt40,

},
Variant2{
fld0: u64,
fld1: Adt21,
fld2: [i16; 4],
fld3: i128,
fld4: Adt67,

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i128; 6],
fld1: *mut u64,

},
Variant1{
fld0: u16,
fld1: (f64, [i64; 3], (i128, u32, u16, char)),

},
Variant2{
fld0: [u64; 4],
fld1: (u8, u64),
fld2: u16,
fld3: [bool; 7],
fld4: i128,

}}

