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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u128,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64) -> isize {
mir! {
type RET = isize;
let _14: [u32; 4];
let _15: [u128; 7];
let _16: isize;
let _17: Adt52;
let _18: isize;
let _19: [i128; 4];
let _20: (isize, i32, *const i32, i128);
let _21: f64;
let _22: [u32; 4];
let _23: [u128; 7];
let _24: Adt45;
let _25: u64;
let _26: [i64; 5];
let _27: u8;
let _28: Adt43;
let _29: Adt45;
let _30: u16;
let _31: Adt44;
let _32: Adt47;
let _33: [u128; 7];
let _34: i64;
let _35: u8;
let _36: f64;
let _37: [u32; 4];
let _38: u16;
let _39: [u32; 4];
let _40: Adt56;
let _41: ();
let _42: ();
{
_6 = -(-211326308_i32);
_14 = [3286216933_u32,1328932591_u32,1513383526_u32,2288966197_u32];
RET = (-126_isize);
_11 = 14521_u16 ^ 55021_u16;
_14 = [1031811476_u32,2443448849_u32,499874247_u32,2807489807_u32];
_4 = 105_i8;
_3 = RET;
_7 = !245482334336939954781617806718936217280_u128;
_8 = (-131724724511111934000680094209789754033_i128) + 22130093391165315566658563022079488794_i128;
_2 = '\u{f04ed}';
_3 = RET;
RET = _3 - _3;
_4 = (-72_i8);
_7 = _2 as u128;
_9 = 6_usize - 17879099708710996993_usize;
_11 = _8 as u16;
RET = _11 as isize;
_8 = 108856175450540083618203464317653681634_i128 - (-53581257696527991560303906644413851408_i128);
_6 = 232221442_i32 * (-1454688605_i32);
RET = _3;
_15 = [_7,_7,_7,_7,_7,_7,_7];
_3 = RET;
_6 = (-1329424561_i32) << _9;
_2 = '\u{3733e}';
_13 = !4199668958770496984_u64;
_16 = -RET;
_2 = '\u{5d69a}';
_13 = 3379461852972137800_u64;
_4 = (-68_i8);
match _3 {
0 => bb1,
340282366920938463463374607431768211330 => bb3,
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
_4 = _13 as i8;
_6 = _4 as i32;
RET = _16 + _3;
_1 = !true;
_6 = 758755530_i32;
RET = _3;
_8 = 3932175875_u32 as i128;
_10 = 134_u8 >> _11;
_8 = !(-77805661834460763825886445130329926876_i128);
_8 = (-104622627322246609896921920837200772493_i128);
_4 = -(-31_i8);
_12 = _10 as u32;
_3 = !_16;
RET = _7 as isize;
_18 = RET ^ _3;
_8 = 140911044828479628951431557163783069916_i128;
_10 = 202_u8 ^ 101_u8;
_14 = [_12,_12,_12,_12];
_7 = !24500311931964482800894090251620565432_u128;
_10 = !168_u8;
match _6 {
758755530 => bb4,
_ => bb2
}
}
bb4 = {
_1 = false;
_5 = 31088_i16;
_7 = 130480284817602082210888571896915169518_u128;
_3 = _9 as isize;
match _8 {
140911044828479628951431557163783069916 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
RET = _13 as isize;
_8 = (-69434439460908652908318821795658998783_i128) >> _6;
_19 = [_8,_8,_8,_8];
_20.1 = _6;
_5 = (-19720_i16);
_20.2 = core::ptr::addr_of!(_6);
_10 = !139_u8;
_4 = !30_i8;
_5 = _4 as i16;
_16 = _3;
_10 = _13 as u8;
_8 = !(-11839781193622401602848417960219752668_i128);
_1 = true & true;
_10 = !69_u8;
_20.3 = _8;
_11 = 20719_u16;
RET = _3;
_13 = !18311288828004560649_u64;
_16 = _2 as isize;
RET = _3 << _20.3;
RET = _10 as isize;
_13 = _2 as u64;
_20.0 = _2 as isize;
_23 = [_7,_7,_7,_7,_7,_7,_7];
_14 = [_12,_12,_12,_12];
_6 = _20.1 + _20.1;
Goto(bb7)
}
bb7 = {
_10 = !143_u8;
_23 = [_7,_7,_7,_7,_7,_7,_7];
_22 = [_12,_12,_12,_12];
_11 = 53309_u16;
_12 = !684601447_u32;
_15 = [_7,_7,_7,_7,_7,_7,_7];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
130480284817602082210888571896915169518 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_5 = _9 as i16;
_20.2 = core::ptr::addr_of!(_20.1);
_7 = _12 as u128;
_13 = !9130587402429040520_u64;
_15 = [_7,_7,_7,_7,_7,_7,_7];
_24.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
_24.fld0 = [_10,_10,_10,_10,_10,_10,_10];
_21 = _13 as f64;
_11 = 29350_u16;
_17 = Adt52::Variant1 { fld0: _11 };
_21 = _12 as f64;
SetDiscriminant(_17, 0);
_24.fld1.2 = [_2,_2];
Goto(bb10)
}
bb10 = {
_8 = _9 as i128;
_6 = _1 as i32;
_9 = !2_usize;
_20.0 = !_16;
_2 = '\u{9413e}';
RET = _10 as isize;
place!(Field::<(u32, *const u128, isize)>(Variant(_17, 0), 0)).0 = _12;
_24.fld2 = _9 as isize;
_19 = [_8,_20.3,_8,_8];
_17 = Adt52::Variant1 { fld0: _11 };
_24.fld0 = [_10,_10,_10,_10,_10,_10,_10];
_24.fld1.2 = [_2,_2];
_13 = !7753710644916177801_u64;
_26 = [1458827583645921386_i64,2208950567261505140_i64,(-5448515122594782702_i64),(-6848789427504504194_i64),1243508282042814896_i64];
_2 = '\u{9751b}';
_27 = !_10;
_20.0 = _3;
_2 = '\u{7f3e2}';
_26 = [517152493492681127_i64,(-3101552709961587413_i64),8945688748814556969_i64,8913858292897873134_i64,8104002099158283495_i64];
_27 = !_10;
_18 = _5 as isize;
_24.fld0 = [_10,_27,_27,_27,_10,_10,_10];
_24.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
place!(Field::<u16>(Variant(_17, 1), 0)) = _11;
_12 = !4064616078_u32;
_20.1 = _6 * _6;
Goto(bb11)
}
bb11 = {
_5 = _13 as i16;
_20.2 = core::ptr::addr_of!(_20.1);
_5 = _9 as i16;
_12 = 3283979042_u32 & 3666807937_u32;
_13 = _1 as u64;
_16 = RET * _3;
_16 = _24.fld2;
_30 = _9 as u16;
_29.fld1.0 = _24.fld1.2;
_24.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
_20.1 = _6;
_27 = _10 | _10;
_5 = 20181_i16;
_29.fld1 = (_24.fld1.2, _24.fld1.1, _24.fld1.2);
_6 = !_20.1;
_20.1 = _6 ^ _6;
_32.fld1 = (_24.fld1.2,);
_2 = '\u{e0ae0}';
_20.1 = _6;
_27 = _10;
Goto(bb12)
}
bb12 = {
_22 = [_12,_12,_12,_12];
_32.fld3 = _4 * _4;
SetDiscriminant(_17, 1);
_34 = (-8048223947522588124_i64);
_24.fld1.0 = _32.fld1.0;
_32.fld2 = [_34,_34,_34,_34,_34];
_22 = [_12,_12,_12,_12];
_22 = [_12,_12,_12,_12];
_4 = -_32.fld3;
_35 = _8 as u8;
_29 = _24;
_29.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
place!(Field::<u16>(Variant(_17, 1), 0)) = _30 - _11;
_6 = !_20.1;
_32.fld4 = core::ptr::addr_of_mut!(_5);
_11 = Field::<u16>(Variant(_17, 1), 0) << _3;
SetDiscriminant(_17, 1);
_24 = Adt45 { fld0: _29.fld0,fld1: _29.fld1,fld2: _18 };
_32.fld4 = core::ptr::addr_of_mut!(_5);
_24 = Adt45 { fld0: _29.fld0,fld1: _29.fld1,fld2: _20.0 };
_13 = 298922587464287219_u64 * 16026126429882894032_u64;
_32.fld1.0 = [_2,_2];
_32.fld1.0 = [_2,_2];
Call(_31 = fn1(_26, _15, _35, _26, _5, _6), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.2 = core::ptr::addr_of!(_20.1);
_36 = _21 + _21;
_1 = !false;
_37 = _22;
_10 = _27;
_24.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
_33 = _15;
_32.fld3 = _4 << RET;
_14 = [_12,_12,_12,_12];
_24.fld2 = _36 as isize;
_3 = _34 as isize;
_9 = !2_usize;
_29.fld2 = -_16;
_39 = _22;
_4 = !_32.fld3;
_25 = _13;
_14 = _22;
_19 = [_20.3,_20.3,_20.3,_20.3];
_33 = [_7,_7,_7,_7,_7,_7,_7];
_24 = Adt45 { fld0: _29.fld0,fld1: _29.fld1,fld2: _3 };
_22 = [_12,_12,_12,_12];
_15 = [_7,_7,_7,_7,_7,_7,_7];
_25 = _13;
_23 = [_7,_7,_7,_7,_7,_7,_7];
Goto(bb14)
}
bb14 = {
_2 = '\u{ff4b}';
_1 = !false;
_32.fld0 = _34 * _34;
_17 = Adt52::Variant1 { fld0: _11 };
_29.fld1.1 = _24.fld1.1;
SetDiscriminant(_17, 1);
_32.fld1.0 = [_2,_2];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(0_usize, 7_usize, Move(_7), 35_usize, Move(_35), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(0_usize, 26_usize, Move(_26), 14_usize, Move(_14), 39_usize, Move(_39), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(0_usize, 8_usize, Move(_8), 33_usize, Move(_33), 11_usize, Move(_11), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(0_usize, 30_usize, Move(_30), 15_usize, Move(_15), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: [i64; 5],mut _2: [u128; 7],mut _3: u8,mut _4: [i64; 5],mut _5: i16,mut _6: i32) -> Adt44 {
mir! {
type RET = Adt44;
let _7: [i64; 5];
let _8: isize;
let _9: *mut f64;
let _10: *const u128;
let _11: u8;
let _12: [char; 2];
let _13: bool;
let _14: [u128; 7];
let _15: ([char; 2],);
let _16: ([char; 2], [char; 7], [char; 2]);
let _17: Adt53;
let _18: Adt52;
let _19: (u32, *const u128, isize);
let _20: ([char; 2], [char; 7], [char; 2]);
let _21: f64;
let _22: isize;
let _23: ();
let _24: ();
{
_1 = [4835276035853193471_i64,(-3311031643258392271_i64),3655358560534657147_i64,2450451434923450405_i64,(-8923331253802725663_i64)];
_3 = 129_u8 >> _6;
_2 = [316301701558266274866634786827506645153_u128,169319279574696079540091965855228503325_u128,19260532593385759343707164704676266635_u128,335839687265992695714991321192986796789_u128,60173150993065314752819042537686602613_u128,323263442217469381989620277438696581409_u128,188298958217599536801973805336791368447_u128];
_4 = [(-7453285331358348252_i64),5651318818027266298_i64,(-8957128900373247878_i64),7857208556945602910_i64,5970377884213802585_i64];
_6 = !(-851250538_i32);
_3 = 250_u8 * 128_u8;
_7 = [(-1954358141858023251_i64),(-3037763722445654932_i64),(-2416234771787613192_i64),(-6370405917471325204_i64),(-2511003351305519752_i64)];
_6 = 1623827831_i32 << _5;
_1 = _7;
RET.fld0 = [3552936848_u32,284233123_u32,2876781424_u32,3792950385_u32];
_3 = 5_u8 << _6;
_1 = _7;
RET.fld0 = [2477303933_u32,2584487764_u32,4269875848_u32,38547845_u32];
_6 = (-1617192429_i32) * 942545470_i32;
_6 = -(-184662505_i32);
_5 = 17466340083468502225_u64 as i16;
_4 = [(-3590660842556349573_i64),(-5394410512769161512_i64),5514755109022398156_i64,(-5523402588398593546_i64),8658636237514007168_i64];
_6 = (-460115383_i32) | (-912985918_i32);
_6 = 1406062917_i32 + (-1977918902_i32);
_5 = !10545_i16;
_8 = (-9223372036854775808_isize);
_1 = [5710492523760448215_i64,(-3782089930890538609_i64),3532800945447204269_i64,(-6380371202848503698_i64),(-6845413499796320223_i64)];
_2 = [182762557877878678129567195569114018675_u128,201752084242645973247948489417379586309_u128,149520348275056214494847963939798820683_u128,244512172924075760453232444667222707340_u128,261311958485127756357867477338353701694_u128,249933940969919503438912259610676777397_u128,51346147799505876911377695644508579168_u128];
_4 = _7;
_1 = [7222091471749993423_i64,(-8910523786186004841_i64),7236176658202308323_i64,5288902289649056285_i64,(-3578633365746301634_i64)];
_5 = true as i16;
_7 = [(-8913887778958735528_i64),3897924075391449123_i64,(-4989777478559282836_i64),2561960658508131099_i64,5234168842903897291_i64];
RET.fld0 = [3872952266_u32,329269311_u32,2850223043_u32,2193352874_u32];
Call(_3 = fn2(Move(RET), _8, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.fld0 = [3374392142_u32,3261723379_u32,1203379435_u32,3908644065_u32];
_2 = [37493157269801391721990668103618640312_u128,293797020528684951007420099823003835346_u128,223218272458887486344608922384241163542_u128,171103044154996979532443765090112347783_u128,46971911137967407809821735542017236802_u128,292891839038000865367256272862244569589_u128,216790522444701336998842559757326386272_u128];
_6 = (-1123294444_i32);
Goto(bb2)
}
bb2 = {
_6 = 9004393519536109755_i64 as i32;
_4 = [3762954962481362128_i64,2756430479526173347_i64,(-4286219786189228909_i64),8010729272630912932_i64,8639335703304367534_i64];
_1 = [5472859268169841176_i64,(-2544246844264444004_i64),8907085251948333891_i64,6113844920384768589_i64,4723427386780062269_i64];
_3 = 242_u8 >> _5;
RET.fld0 = [3587615871_u32,3523215166_u32,2915665734_u32,3347141593_u32];
_8 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
RET.fld0 = [1406254778_u32,3994178960_u32,877644878_u32,2876618513_u32];
_2 = [159585683361834580398904083091573004790_u128,293697399863608256753136626594507124710_u128,174286236058081310961296708715756507732_u128,143856430271418040809661939936442803658_u128,115588936927356111872185286317870442679_u128,233428864381484915199814416680810759554_u128,277109927555249038418864775027457260437_u128];
_5 = 442_i16;
RET.fld0 = [1903058549_u32,3432929920_u32,2619460923_u32,2608130202_u32];
_14 = [53901049122732777071830671532659778890_u128,73154029876561568346213508070759938820_u128,124404490369693230995975271668233133364_u128,68096735236321735478946631297809394958_u128,322064254785448745551988607507904139389_u128,314067395028673005842042231986363235965_u128,301343861973482103886441336390809533997_u128];
_8 = (-92_isize) >> _6;
_6 = 12135519488813162147_u64 as i32;
_3 = 8024878629423561384_usize as u8;
Goto(bb3)
}
bb3 = {
_15.0 = ['\u{f97ee}','\u{45674}'];
_13 = !true;
_7 = _1;
_13 = false;
_14 = [135069149052088659332748840454130487081_u128,113739954690216393678989810323908969180_u128,308012806150826417278411184916981347635_u128,29044982563815681070171362572781448288_u128,5765840964575650189085264079061813947_u128,42389006641549468884260083602227229372_u128,2663414499419432912380738047885656278_u128];
_3 = 37_u8;
_13 = !false;
_6 = _5 as i32;
_3 = 75_u8 >> _8;
_16.2 = ['\u{21f75}','\u{63c4b}'];
_3 = !218_u8;
_2 = [164663617471996940413801130548875440325_u128,250771525761056014599897201112030765492_u128,303771263806201420809090933749275154177_u128,260634893942312592307595216087770696214_u128,249216001177143892968573780384392631596_u128,243087877631879742583279600518617182634_u128,78771140400978320399158439217378468114_u128];
_12 = ['\u{10ffcc}','\u{23bad}'];
_1 = [3491368496103048696_i64,(-2824038137871714605_i64),(-8196379471189298472_i64),(-4366511038788088405_i64),(-1127890479860745245_i64)];
_6 = 271584518_i32;
_20.1 = ['\u{10edab}','\u{301f7}','\u{69e7f}','\u{ac84a}','\u{fc7e0}','\u{88d75}','\u{6e993}'];
_16.1 = ['\u{bcfb2}','\u{691fa}','\u{5f52e}','\u{b905b}','\u{7309a}','\u{92831}','\u{b0b7d}'];
_16.1 = _20.1;
_19.0 = 3893799768_u32 ^ 1084257851_u32;
_16 = (_12, _20.1, _12);
Goto(bb4)
}
bb4 = {
_20 = (_16.2, _16.1, _12);
_19.2 = _8 + _8;
RET.fld0 = [_19.0,_19.0,_19.0,_19.0];
_20 = _16;
_16.1 = ['\u{ce867}','\u{5ee8e}','\u{95459}','\u{3f47d}','\u{5041f}','\u{508e5}','\u{e2874}'];
_11 = _13 as u8;
_19.2 = -_8;
_20.1 = ['\u{cefbc}','\u{46392}','\u{328e6}','\u{b3938}','\u{7895a}','\u{4e8ed}','\u{419f1}'];
_16 = (_20.2, _20.1, _15.0);
_16.2 = _20.2;
_19.0 = !517675073_u32;
_14 = _2;
_4 = [514027728824893373_i64,8571453913010286445_i64,453696209522446061_i64,(-7992814642610909638_i64),5895675523183160528_i64];
_3 = _6 as u8;
_15.0 = ['\u{bd386}','\u{102f18}'];
_9 = core::ptr::addr_of_mut!(_21);
_14 = [215015252128871358539148070408431516292_u128,178261432668128072551753905843963657659_u128,58444186356563359588947446536678191928_u128,258184649099550095908294696711123009578_u128,113916046166540034493941689385270647976_u128,197513971852534517022897680876697355156_u128,310236416499503000285804345565647639039_u128];
_4 = _7;
(*_9) = 12597894231248667196_u64 as f64;
_12 = _15.0;
_20 = (_12, _16.1, _15.0);
_8 = !_19.2;
_20.0 = _12;
(*_9) = _19.2 as f64;
_16.1 = ['\u{10a7bb}','\u{64599}','\u{d2f3d}','\u{10fc76}','\u{9261}','\u{35bed}','\u{466b6}'];
_5 = -15518_i16;
_20 = _16;
_14 = [253305646148318737709824826569811110495_u128,160928262306435705930846288459337849710_u128,222887447676660182126257193527766600433_u128,55413562412026074984253973318370501425_u128,308547504357365065236642927550315588833_u128,72286959033029167614940946118832457922_u128,136261830925874755176440627379595757031_u128];
(*_9) = 150392426008917173184749622591352334042_i128 as f64;
_19.2 = -_8;
_20.2 = ['\u{b7c29}','\u{3f577}'];
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
271584518 => bb10,
_ => bb9
}
}
bb5 = {
_15.0 = ['\u{f97ee}','\u{45674}'];
_13 = !true;
_7 = _1;
_13 = false;
_14 = [135069149052088659332748840454130487081_u128,113739954690216393678989810323908969180_u128,308012806150826417278411184916981347635_u128,29044982563815681070171362572781448288_u128,5765840964575650189085264079061813947_u128,42389006641549468884260083602227229372_u128,2663414499419432912380738047885656278_u128];
_3 = 37_u8;
_13 = !false;
_6 = _5 as i32;
_3 = 75_u8 >> _8;
_16.2 = ['\u{21f75}','\u{63c4b}'];
_3 = !218_u8;
_2 = [164663617471996940413801130548875440325_u128,250771525761056014599897201112030765492_u128,303771263806201420809090933749275154177_u128,260634893942312592307595216087770696214_u128,249216001177143892968573780384392631596_u128,243087877631879742583279600518617182634_u128,78771140400978320399158439217378468114_u128];
_12 = ['\u{10ffcc}','\u{23bad}'];
_1 = [3491368496103048696_i64,(-2824038137871714605_i64),(-8196379471189298472_i64),(-4366511038788088405_i64),(-1127890479860745245_i64)];
_6 = 271584518_i32;
_20.1 = ['\u{10edab}','\u{301f7}','\u{69e7f}','\u{ac84a}','\u{fc7e0}','\u{88d75}','\u{6e993}'];
_16.1 = ['\u{bcfb2}','\u{691fa}','\u{5f52e}','\u{b905b}','\u{7309a}','\u{92831}','\u{b0b7d}'];
_16.1 = _20.1;
_19.0 = 3893799768_u32 ^ 1084257851_u32;
_16 = (_12, _20.1, _12);
Goto(bb4)
}
bb6 = {
_6 = 9004393519536109755_i64 as i32;
_4 = [3762954962481362128_i64,2756430479526173347_i64,(-4286219786189228909_i64),8010729272630912932_i64,8639335703304367534_i64];
_1 = [5472859268169841176_i64,(-2544246844264444004_i64),8907085251948333891_i64,6113844920384768589_i64,4723427386780062269_i64];
_3 = 242_u8 >> _5;
RET.fld0 = [3587615871_u32,3523215166_u32,2915665734_u32,3347141593_u32];
_8 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
RET.fld0 = [1406254778_u32,3994178960_u32,877644878_u32,2876618513_u32];
_2 = [159585683361834580398904083091573004790_u128,293697399863608256753136626594507124710_u128,174286236058081310961296708715756507732_u128,143856430271418040809661939936442803658_u128,115588936927356111872185286317870442679_u128,233428864381484915199814416680810759554_u128,277109927555249038418864775027457260437_u128];
_5 = 442_i16;
RET.fld0 = [1903058549_u32,3432929920_u32,2619460923_u32,2608130202_u32];
_14 = [53901049122732777071830671532659778890_u128,73154029876561568346213508070759938820_u128,124404490369693230995975271668233133364_u128,68096735236321735478946631297809394958_u128,322064254785448745551988607507904139389_u128,314067395028673005842042231986363235965_u128,301343861973482103886441336390809533997_u128];
_8 = (-92_isize) >> _6;
_6 = 12135519488813162147_u64 as i32;
_3 = 8024878629423561384_usize as u8;
Goto(bb3)
}
bb7 = {
RET.fld0 = [3374392142_u32,3261723379_u32,1203379435_u32,3908644065_u32];
_2 = [37493157269801391721990668103618640312_u128,293797020528684951007420099823003835346_u128,223218272458887486344608922384241163542_u128,171103044154996979532443765090112347783_u128,46971911137967407809821735542017236802_u128,292891839038000865367256272862244569589_u128,216790522444701336998842559757326386272_u128];
_6 = (-1123294444_i32);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_20.1 = _16.1;
_3 = _11 - _11;
_16.1 = _20.1;
_16.1 = ['\u{acf9e}','\u{92c8}','\u{107d3b}','\u{2735a}','\u{f384a}','\u{bb9bd}','\u{50ca1}'];
match _6 {
0 => bb4,
1 => bb6,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
271584518 => bb16,
_ => bb15
}
}
bb11 = {
_20 = (_16.2, _16.1, _12);
_19.2 = _8 + _8;
RET.fld0 = [_19.0,_19.0,_19.0,_19.0];
_20 = _16;
_16.1 = ['\u{ce867}','\u{5ee8e}','\u{95459}','\u{3f47d}','\u{5041f}','\u{508e5}','\u{e2874}'];
_11 = _13 as u8;
_19.2 = -_8;
_20.1 = ['\u{cefbc}','\u{46392}','\u{328e6}','\u{b3938}','\u{7895a}','\u{4e8ed}','\u{419f1}'];
_16 = (_20.2, _20.1, _15.0);
_16.2 = _20.2;
_19.0 = !517675073_u32;
_14 = _2;
_4 = [514027728824893373_i64,8571453913010286445_i64,453696209522446061_i64,(-7992814642610909638_i64),5895675523183160528_i64];
_3 = _6 as u8;
_15.0 = ['\u{bd386}','\u{102f18}'];
_9 = core::ptr::addr_of_mut!(_21);
_14 = [215015252128871358539148070408431516292_u128,178261432668128072551753905843963657659_u128,58444186356563359588947446536678191928_u128,258184649099550095908294696711123009578_u128,113916046166540034493941689385270647976_u128,197513971852534517022897680876697355156_u128,310236416499503000285804345565647639039_u128];
_4 = _7;
(*_9) = 12597894231248667196_u64 as f64;
_12 = _15.0;
_20 = (_12, _16.1, _15.0);
_8 = !_19.2;
_20.0 = _12;
(*_9) = _19.2 as f64;
_16.1 = ['\u{10a7bb}','\u{64599}','\u{d2f3d}','\u{10fc76}','\u{9261}','\u{35bed}','\u{466b6}'];
_5 = -15518_i16;
_20 = _16;
_14 = [253305646148318737709824826569811110495_u128,160928262306435705930846288459337849710_u128,222887447676660182126257193527766600433_u128,55413562412026074984253973318370501425_u128,308547504357365065236642927550315588833_u128,72286959033029167614940946118832457922_u128,136261830925874755176440627379595757031_u128];
(*_9) = 150392426008917173184749622591352334042_i128 as f64;
_19.2 = -_8;
_20.2 = ['\u{b7c29}','\u{3f577}'];
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
271584518 => bb10,
_ => bb9
}
}
bb12 = {
Return()
}
bb13 = {
RET.fld0 = [3374392142_u32,3261723379_u32,1203379435_u32,3908644065_u32];
_2 = [37493157269801391721990668103618640312_u128,293797020528684951007420099823003835346_u128,223218272458887486344608922384241163542_u128,171103044154996979532443765090112347783_u128,46971911137967407809821735542017236802_u128,292891839038000865367256272862244569589_u128,216790522444701336998842559757326386272_u128];
_6 = (-1123294444_i32);
Goto(bb2)
}
bb14 = {
_6 = 9004393519536109755_i64 as i32;
_4 = [3762954962481362128_i64,2756430479526173347_i64,(-4286219786189228909_i64),8010729272630912932_i64,8639335703304367534_i64];
_1 = [5472859268169841176_i64,(-2544246844264444004_i64),8907085251948333891_i64,6113844920384768589_i64,4723427386780062269_i64];
_3 = 242_u8 >> _5;
RET.fld0 = [3587615871_u32,3523215166_u32,2915665734_u32,3347141593_u32];
_8 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
RET.fld0 = [1406254778_u32,3994178960_u32,877644878_u32,2876618513_u32];
_2 = [159585683361834580398904083091573004790_u128,293697399863608256753136626594507124710_u128,174286236058081310961296708715756507732_u128,143856430271418040809661939936442803658_u128,115588936927356111872185286317870442679_u128,233428864381484915199814416680810759554_u128,277109927555249038418864775027457260437_u128];
_5 = 442_i16;
RET.fld0 = [1903058549_u32,3432929920_u32,2619460923_u32,2608130202_u32];
_14 = [53901049122732777071830671532659778890_u128,73154029876561568346213508070759938820_u128,124404490369693230995975271668233133364_u128,68096735236321735478946631297809394958_u128,322064254785448745551988607507904139389_u128,314067395028673005842042231986363235965_u128,301343861973482103886441336390809533997_u128];
_8 = (-92_isize) >> _6;
_6 = 12135519488813162147_u64 as i32;
_3 = 8024878629423561384_usize as u8;
Goto(bb3)
}
bb15 = {
_15.0 = ['\u{f97ee}','\u{45674}'];
_13 = !true;
_7 = _1;
_13 = false;
_14 = [135069149052088659332748840454130487081_u128,113739954690216393678989810323908969180_u128,308012806150826417278411184916981347635_u128,29044982563815681070171362572781448288_u128,5765840964575650189085264079061813947_u128,42389006641549468884260083602227229372_u128,2663414499419432912380738047885656278_u128];
_3 = 37_u8;
_13 = !false;
_6 = _5 as i32;
_3 = 75_u8 >> _8;
_16.2 = ['\u{21f75}','\u{63c4b}'];
_3 = !218_u8;
_2 = [164663617471996940413801130548875440325_u128,250771525761056014599897201112030765492_u128,303771263806201420809090933749275154177_u128,260634893942312592307595216087770696214_u128,249216001177143892968573780384392631596_u128,243087877631879742583279600518617182634_u128,78771140400978320399158439217378468114_u128];
_12 = ['\u{10ffcc}','\u{23bad}'];
_1 = [3491368496103048696_i64,(-2824038137871714605_i64),(-8196379471189298472_i64),(-4366511038788088405_i64),(-1127890479860745245_i64)];
_6 = 271584518_i32;
_20.1 = ['\u{10edab}','\u{301f7}','\u{69e7f}','\u{ac84a}','\u{fc7e0}','\u{88d75}','\u{6e993}'];
_16.1 = ['\u{bcfb2}','\u{691fa}','\u{5f52e}','\u{b905b}','\u{7309a}','\u{92831}','\u{b0b7d}'];
_16.1 = _20.1;
_19.0 = 3893799768_u32 ^ 1084257851_u32;
_16 = (_12, _20.1, _12);
Goto(bb4)
}
bb16 = {
_9 = core::ptr::addr_of_mut!(_21);
Goto(bb17)
}
bb17 = {
Call(_23 = dump_var(1_usize, 16_usize, Move(_16), 12_usize, Move(_12), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_23 = dump_var(1_usize, 4_usize, Move(_4), 3_usize, Move(_3), 2_usize, Move(_2), 24_usize, _24), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: Adt44,mut _2: isize,mut _3: [i64; 5]) -> u8 {
mir! {
type RET = u8;
let _4: bool;
let _5: Adt48;
let _6: [u32; 4];
let _7: Adt49;
let _8: Adt44;
let _9: i64;
let _10: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,));
let _11: u8;
let _12: isize;
let _13: f64;
let _14: isize;
let _15: isize;
let _16: bool;
let _17: [i128; 4];
let _18: i16;
let _19: Adt51;
let _20: i128;
let _21: i64;
let _22: [u32; 4];
let _23: Adt44;
let _24: f64;
let _25: Adt54;
let _26: *const i32;
let _27: ();
let _28: ();
{
RET = 254_u8 & 4_u8;
RET = 19_u8;
RET = !164_u8;
_3 = [(-6104160343515380527_i64),(-6988483808528500897_i64),3604528564615470338_i64,5245197906961307237_i64,4784292267109503038_i64];
_2 = (-9223372036854775808_isize) + 54_isize;
_6 = _1.fld0;
_3 = [(-6641820961716562992_i64),801319198040945633_i64,2928488980155835982_i64,3357133392697793667_i64,(-349776277557053073_i64)];
_4 = !false;
_1 = Adt44 { fld0: _6 };
RET = 183_u8 + 37_u8;
_6 = [2226554657_u32,2178341462_u32,3598505996_u32,3580896934_u32];
_1.fld0 = [2406414466_u32,646055604_u32,2329401919_u32,931103313_u32];
RET = !203_u8;
_1 = Adt44 { fld0: _6 };
_1.fld0 = [3297509317_u32,1526655363_u32,159953331_u32,1067513621_u32];
RET = !68_u8;
RET = (-58828031418108919949137976920522781632_i128) as u8;
RET = 185_u8 + 241_u8;
RET = 149_u8 << _2;
_3 = [4669877322662764257_i64,(-274667182932886413_i64),(-5894713790802200097_i64),(-9146320898523955348_i64),(-6318318824880102840_i64)];
Goto(bb1)
}
bb1 = {
_8 = Adt44 { fld0: _6 };
_2 = -70_isize;
_1.fld0 = [2955283595_u32,1328089095_u32,2020373605_u32,4063615368_u32];
RET = 30_u8 ^ 77_u8;
_2 = 0_isize * (-9223372036854775808_isize);
_2 = (-113_isize);
_3 = [4434379499487926330_i64,1037838545948826238_i64,(-6110250645202282356_i64),(-744278617635724460_i64),3439923072265426704_i64];
_1.fld0 = [3291919227_u32,1196385380_u32,3396941932_u32,2343382923_u32];
_3 = [(-7184283559541800804_i64),5340673118800386346_i64,5414175211352307125_i64,3163557768960974040_i64,65996255964609880_i64];
_3 = [(-7590259573913857533_i64),(-1762520589510287299_i64),7938729946192779433_i64,6630319169394614332_i64,(-32282583949642647_i64)];
_1 = Move(_8);
RET = !102_u8;
RET = !224_u8;
Call(_6 = fn3(_1.fld0, _2, _1.fld0, _3, _3, _3, _1.fld0, Move(_1), _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.fld0 = [1222235827_u32,2076610123_u32,2361939102_u32,1476400978_u32];
_6 = _1.fld0;
_8 = Adt44 { fld0: _6 };
_10.4 = [318064413656875208936290657112611876847_u128,268010736386451053688245375219199928248_u128,279240541258340364604368897104384791780_u128,115749542415165868215759883709774976952_u128,148045369354783506761063860918797625605_u128,229569134357149072789416192409220082632_u128,69935606789304792035679195866250893119_u128];
_10.4 = [54089108110977236334969471547378014518_u128,148872023917144472645259493309301441375_u128,92269697920833533786396010871225078913_u128,87980928528976052127738699517726904895_u128,283516028565766819272212149980132158959_u128,199058929080157024395425901981346667831_u128,206005023008550297327903210135398669602_u128];
_10.3 = RET << _2;
_8.fld0 = [2472467647_u32,3385565635_u32,1828976725_u32,781593180_u32];
Call(_10.0 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1.fld0 = [381352042_u32,638806326_u32,620174679_u32,4060191513_u32];
_10.2 = _2 & _2;
_1.fld0 = _6;
_10.2 = 5072405444481500265_u64 as isize;
_10.0 = '\u{bfc58}' as isize;
_10.2 = _2;
Call(_10.0 = core::intrinsics::bswap(_10.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _10.3 ^ _10.3;
RET = _10.3 | _10.3;
_11 = !_10.3;
_11 = _10.3;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_10.3 = RET;
_8.fld0 = [3252998065_u32,2232698321_u32,3594247479_u32,1920828041_u32];
_13 = 54_i8 as f64;
_3 = [6479914174630194354_i64,(-3899285056542641122_i64),(-4192389168966019035_i64),594053452330314695_i64,(-8627093728247565806_i64)];
_9 = (-2605946630328465669_i64);
_9 = (-3643319482822454665_i64) + 8814522901551736711_i64;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_4 = true | true;
RET = !_11;
_8 = Move(_1);
_10.4 = [202690638029973680556907645749325675692_u128,223836657776347871009401118094191344090_u128,47135908433331373691413428332945282280_u128,329219757732688277675069915069841319701_u128,262703559153712552684429740991490588499_u128,50402326712411527329746550658289499150_u128,321572324444913387127996926961873726210_u128];
_1 = Adt44 { fld0: _8.fld0 };
_10.4 = [44622941319223932846725700017040095707_u128,132740726741746189991208315996420667753_u128,232228425518164490833219754687626414965_u128,254868368373166423361537984126726092310_u128,73827272915747367017536203053786157163_u128,264011937584460778794736979716597315428_u128,284698822440658193686681184730509963123_u128];
_2 = 164857628_u32 as isize;
_3 = [_9,_9,_9,_9,_9];
_10.2 = !_2;
_12 = 2845582308_u32 as isize;
_15 = _12;
Goto(bb5)
}
bb5 = {
_14 = !_10.2;
_10.3 = !_11;
_10.0 = _10.2;
_6 = [213481490_u32,1134498148_u32,249156470_u32,3164500170_u32];
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_3 = [_9,_9,_9,_9,_9];
_2 = (-306289870_i32) as isize;
_9 = 1754248259325041127_i64;
RET = _10.3;
_6 = [104566059_u32,2254031981_u32,4069210779_u32,3158657217_u32];
RET = !_11;
_12 = -_10.0;
_11 = (-120160858549912853475363768028350699648_i128) as u8;
_16 = _4;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_18 = -10841_i16;
Goto(bb6)
}
bb6 = {
_6 = [2736780085_u32,481638882_u32,103105098_u32,3574255339_u32];
_6 = [994246065_u32,3482371265_u32,1865098246_u32,3669683722_u32];
_13 = RET as f64;
_13 = 7397926466616933649_u64 as f64;
_8 = Move(_1);
_10.4 = [72292927649537618320134817322408546593_u128,304348527838335806309958909047213006071_u128,89236238854886697094932944031206750444_u128,222209446919490974611803915963781749327_u128,333153518331614268225490290540703038707_u128,202734137225948400186722297047647240013_u128,62899066074202435498117728549955518655_u128];
_6 = [2983990368_u32,3956351151_u32,432707604_u32,193333741_u32];
_10.2 = _14 - _15;
_10.3 = _11;
_10.3 = !RET;
_17 = [(-142020407402092728414068836333614150176_i128),147278668678352806965473656944748111368_i128,(-35564213141005952387433619270757072074_i128),(-110895753324558921318399492472107112830_i128)];
_10.3 = RET;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_4 = !_16;
match _9 {
0 => bb3,
1 => bb7,
2 => bb8,
1754248259325041127 => bb10,
_ => bb9
}
}
bb7 = {
_8 = Adt44 { fld0: _6 };
_2 = -70_isize;
_1.fld0 = [2955283595_u32,1328089095_u32,2020373605_u32,4063615368_u32];
RET = 30_u8 ^ 77_u8;
_2 = 0_isize * (-9223372036854775808_isize);
_2 = (-113_isize);
_3 = [4434379499487926330_i64,1037838545948826238_i64,(-6110250645202282356_i64),(-744278617635724460_i64),3439923072265426704_i64];
_1.fld0 = [3291919227_u32,1196385380_u32,3396941932_u32,2343382923_u32];
_3 = [(-7184283559541800804_i64),5340673118800386346_i64,5414175211352307125_i64,3163557768960974040_i64,65996255964609880_i64];
_3 = [(-7590259573913857533_i64),(-1762520589510287299_i64),7938729946192779433_i64,6630319169394614332_i64,(-32282583949642647_i64)];
_1 = Move(_8);
RET = !102_u8;
RET = !224_u8;
Call(_6 = fn3(_1.fld0, _2, _1.fld0, _3, _3, _3, _1.fld0, Move(_1), _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
RET = _10.3 ^ _10.3;
RET = _10.3 | _10.3;
_11 = !_10.3;
_11 = _10.3;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_10.3 = RET;
_8.fld0 = [3252998065_u32,2232698321_u32,3594247479_u32,1920828041_u32];
_13 = 54_i8 as f64;
_3 = [6479914174630194354_i64,(-3899285056542641122_i64),(-4192389168966019035_i64),594053452330314695_i64,(-8627093728247565806_i64)];
_9 = (-2605946630328465669_i64);
_9 = (-3643319482822454665_i64) + 8814522901551736711_i64;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_4 = true | true;
RET = !_11;
_8 = Move(_1);
_10.4 = [202690638029973680556907645749325675692_u128,223836657776347871009401118094191344090_u128,47135908433331373691413428332945282280_u128,329219757732688277675069915069841319701_u128,262703559153712552684429740991490588499_u128,50402326712411527329746550658289499150_u128,321572324444913387127996926961873726210_u128];
_1 = Adt44 { fld0: _8.fld0 };
_10.4 = [44622941319223932846725700017040095707_u128,132740726741746189991208315996420667753_u128,232228425518164490833219754687626414965_u128,254868368373166423361537984126726092310_u128,73827272915747367017536203053786157163_u128,264011937584460778794736979716597315428_u128,284698822440658193686681184730509963123_u128];
_2 = 164857628_u32 as isize;
_3 = [_9,_9,_9,_9,_9];
_10.2 = !_2;
_12 = 2845582308_u32 as isize;
_15 = _12;
Goto(bb5)
}
bb9 = {
_1.fld0 = [381352042_u32,638806326_u32,620174679_u32,4060191513_u32];
_10.2 = _2 & _2;
_1.fld0 = _6;
_10.2 = 5072405444481500265_u64 as isize;
_10.0 = '\u{bfc58}' as isize;
_10.2 = _2;
Call(_10.0 = core::intrinsics::bswap(_10.2), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_1.fld0 = _8.fld0;
_3 = [_9,_9,_9,_9,_9];
RET = _15 as u8;
_10.3 = _18 as u8;
_2 = _10.0;
_2 = _10.2 << _18;
_19 = Adt51::Variant1 { fld0: 40_i8 };
_16 = _4;
_8 = Adt44 { fld0: _6 };
RET = 1798036740_u32 as u8;
_10.0 = _10.2;
_13 = _11 as f64;
_16 = !_4;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_4 = !_16;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_19 = Adt51::Variant1 { fld0: (-28_i8) };
_8.fld0 = [2658004271_u32,4007101296_u32,4259025750_u32,2280931939_u32];
RET = !_11;
_10.0 = _2 + _10.2;
match _9 {
0 => bb11,
1754248259325041127 => bb13,
_ => bb12
}
}
bb11 = {
_8 = Adt44 { fld0: _6 };
_2 = -70_isize;
_1.fld0 = [2955283595_u32,1328089095_u32,2020373605_u32,4063615368_u32];
RET = 30_u8 ^ 77_u8;
_2 = 0_isize * (-9223372036854775808_isize);
_2 = (-113_isize);
_3 = [4434379499487926330_i64,1037838545948826238_i64,(-6110250645202282356_i64),(-744278617635724460_i64),3439923072265426704_i64];
_1.fld0 = [3291919227_u32,1196385380_u32,3396941932_u32,2343382923_u32];
_3 = [(-7184283559541800804_i64),5340673118800386346_i64,5414175211352307125_i64,3163557768960974040_i64,65996255964609880_i64];
_3 = [(-7590259573913857533_i64),(-1762520589510287299_i64),7938729946192779433_i64,6630319169394614332_i64,(-32282583949642647_i64)];
_1 = Move(_8);
RET = !102_u8;
RET = !224_u8;
Call(_6 = fn3(_1.fld0, _2, _1.fld0, _3, _3, _3, _1.fld0, Move(_1), _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_6 = [2736780085_u32,481638882_u32,103105098_u32,3574255339_u32];
_6 = [994246065_u32,3482371265_u32,1865098246_u32,3669683722_u32];
_13 = RET as f64;
_13 = 7397926466616933649_u64 as f64;
_8 = Move(_1);
_10.4 = [72292927649537618320134817322408546593_u128,304348527838335806309958909047213006071_u128,89236238854886697094932944031206750444_u128,222209446919490974611803915963781749327_u128,333153518331614268225490290540703038707_u128,202734137225948400186722297047647240013_u128,62899066074202435498117728549955518655_u128];
_6 = [2983990368_u32,3956351151_u32,432707604_u32,193333741_u32];
_10.2 = _14 - _15;
_10.3 = _11;
_10.3 = !RET;
_17 = [(-142020407402092728414068836333614150176_i128),147278668678352806965473656944748111368_i128,(-35564213141005952387433619270757072074_i128),(-110895753324558921318399492472107112830_i128)];
_10.3 = RET;
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_4 = !_16;
match _9 {
0 => bb3,
1 => bb7,
2 => bb8,
1754248259325041127 => bb10,
_ => bb9
}
}
bb13 = {
place!(Field::<i8>(Variant(_19, 1), 0)) = 33_i8;
_6 = [3046842578_u32,3540117689_u32,1699158139_u32,846329328_u32];
_14 = (-383515220_i32) as isize;
_6 = _1.fld0;
_10.2 = !_12;
_15 = -_10.2;
_21 = _9;
_13 = 33223_u16 as f64;
_10.2 = -_10.0;
RET = _10.3 ^ _10.3;
_2 = 49368_u16 as isize;
_3 = [_9,_21,_9,_9,_21];
_20 = -94623351094462152939087686984405952075_i128;
place!(Field::<i8>(Variant(_19, 1), 0)) = 71_i8;
_1 = Adt44 { fld0: _8.fld0 };
_8 = Adt44 { fld0: _6 };
_9 = _21 >> _20;
_1 = Adt44 { fld0: _8.fld0 };
_10.3 = !RET;
RET = _10.3 - _10.3;
_4 = _16;
place!(Field::<i8>(Variant(_19, 1), 0)) = -(-31_i8);
_23.fld0 = [386351082_u32,1456818754_u32,2557012266_u32,3380224530_u32];
_11 = RET | RET;
RET = _11;
_13 = _10.0 as f64;
_10.3 = _11;
_15 = _14 - _12;
_21 = 1310630523_u32 as i64;
RET = _11;
_10.2 = 7_usize as isize;
Call(RET = core::intrinsics::bswap(_11), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
_10.1 = core::ptr::addr_of_mut!(_10.5.0);
RET = _11;
place!(Field::<i8>(Variant(_19, 1), 0)) = 5304200874420525367_u64 as i8;
_17 = [_20,_20,_20,_20];
_11 = 59210_u16 as u8;
_16 = _10.3 == _10.3;
RET = _10.3;
_2 = -_15;
_25.fld3 = _13 + _13;
_25.fld5.fld0 = 7533_u16 as i64;
_10.5.0 = core::ptr::addr_of!(_20);
_25.fld5.fld1.0 = ['\u{7694b}','\u{ee41}'];
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(2_usize, 11_usize, Move(_11), 20_usize, Move(_20), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(2_usize, 6_usize, Move(_6), 21_usize, Move(_21), 2_usize, Move(_2), 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u32; 4],mut _2: isize,mut _3: [u32; 4],mut _4: [i64; 5],mut _5: [i64; 5],mut _6: [i64; 5],mut _7: [u32; 4],mut _8: Adt44,mut _9: isize,mut _10: [i64; 5]) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _11: *mut u16;
let _12: ([char; 2],);
let _13: isize;
let _14: i32;
let _15: char;
let _16: u16;
let _17: f32;
let _18: f64;
let _19: ([char; 2],);
let _20: f64;
let _21: [u128; 7];
let _22: ([char; 2], [char; 7], [char; 2]);
let _23: bool;
let _24: i32;
let _25: Adt46;
let _26: f32;
let _27: isize;
let _28: ([char; 2],);
let _29: [i64; 5];
let _30: i32;
let _31: usize;
let _32: (u32, i64);
let _33: char;
let _34: isize;
let _35: [char; 2];
let _36: [char; 2];
let _37: Adt48;
let _38: ([char; 2],);
let _39: *const (u32, *const u128, isize);
let _40: ();
let _41: ();
{
_8 = Adt44 { fld0: _3 };
_8.fld0 = _3;
_8.fld0 = _3;
_5 = _4;
_8.fld0 = _3;
_10 = [971552514108421561_i64,6665877813560610366_i64,(-6146935862546039517_i64),9203237799265250703_i64,(-8017886492629105757_i64)];
_4 = _5;
Goto(bb1)
}
bb1 = {
RET = [293955238_u32,2307764333_u32,182918646_u32,2242278156_u32];
RET = _1;
_13 = _9;
RET = _8.fld0;
_11 = core::ptr::addr_of_mut!(_16);
_6 = _10;
_1 = _3;
(*_11) = 46362_u16;
RET = [867969864_u32,1932784703_u32,3335866961_u32,1635613789_u32];
_9 = _2;
_6 = _10;
_16 = 78291696_u32 as u16;
_17 = 61_i16 as f32;
_8.fld0 = RET;
_14 = 1353422286_u32 as i32;
_5 = _4;
_18 = _14 as f64;
_16 = 14285_u16 + 5676_u16;
_13 = _14 as isize;
_9 = !_2;
(*_11) = !27811_u16;
Goto(bb2)
}
bb2 = {
_18 = _17 as f64;
_3 = _8.fld0;
(*_11) = 47962_u16;
_19.0 = ['\u{2d6f6}','\u{7282a}'];
_8.fld0 = _3;
_12 = _19;
_19.0 = ['\u{7fe8}','\u{60f53}'];
Call(_10 = fn4(Move(_8), _1, _19, _17, (*_11), _2, _7, _2, _4, _4, RET, _16, _19, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = _12;
_5 = _4;
_20 = _18;
_8.fld0 = [524761951_u32,79136512_u32,896083222_u32,2783163345_u32];
_18 = _20 + _20;
_6 = _5;
_9 = -_13;
_18 = _20;
_8.fld0 = _1;
_3 = [569502560_u32,555164318_u32,3150321995_u32,1267278135_u32];
_18 = _20;
_2 = _9 | _9;
_22.2 = ['\u{1c03c}','\u{f5e6d}'];
_19.0 = _22.2;
_7 = [555251092_u32,55444628_u32,1259783413_u32,4134308789_u32];
_20 = -_18;
_22.2 = ['\u{5a1d3}','\u{43894}'];
(*_11) = !17296_u16;
_22.1 = ['\u{2552e}','\u{a26d4}','\u{77a67}','\u{80f75}','\u{b40c}','\u{e6587}','\u{d52b6}'];
_13 = _2;
_19 = _12;
_17 = 2861310002475344632_u64 as f32;
_15 = '\u{feb45}';
_22.2 = [_15,_15];
Goto(bb4)
}
bb4 = {
_11 = core::ptr::addr_of_mut!((*_11));
_16 = 59470_u16 + 28651_u16;
_1 = [913776057_u32,3058119383_u32,2803525580_u32,3778773542_u32];
_22.0 = [_15,_15];
RET = _7;
_1 = [3126064089_u32,4215445641_u32,748963912_u32,2504363954_u32];
(*_11) = 46692_u16;
_21 = [2941415780526056944981932333844748442_u128,6341901538290605472116730855354311381_u128,310160118086605676132224955231407412357_u128,238020880804614925994021795067042104677_u128,31230961180903175234235047814163384802_u128,83616911565024249446154068487760221848_u128,119561462252642532369164919369462324701_u128];
_3 = [2048968371_u32,4162862025_u32,675770875_u32,1647135739_u32];
_16 = 14395_u16 + 62412_u16;
_13 = _9 * _9;
RET = [1245386106_u32,2954198290_u32,3541603944_u32,748251725_u32];
Call(_26 = core::intrinsics::transmute(_15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = _8.fld0;
_21 = [158538169864479249753967932261687422454_u128,71711804450659516486318719494932611698_u128,174981812067358288528596703767106693612_u128,297971973644678324857285077422606601679_u128,270241118225316460931737676395758265599_u128,77327531412220570522068680846404231408_u128,205356706641509365676160494475085180628_u128];
_19 = (_22.0,);
RET = [307662590_u32,287842788_u32,153995980_u32,3850631108_u32];
_22.1 = [_15,_15,_15,_15,_15,_15,_15];
_16 = 56058_u16;
_15 = '\u{692aa}';
RET = [3574728830_u32,326998439_u32,2341771994_u32,2586745055_u32];
(*_11) = 50137_u16;
_27 = !_13;
_4 = [1060665054995331864_i64,(-7499450996539059104_i64),(-6168648399410243511_i64),6674908124370220491_i64,(-438738039352840890_i64)];
_8 = Adt44 { fld0: _1 };
_2 = -_13;
_15 = '\u{1bf2b}';
_21 = [199314764452024572163899174855054342731_u128,267380018771099849689763195399676327211_u128,196623475133505943003294272921761953720_u128,263464687871801113553578209662242261686_u128,264366140638390426808959215469684494512_u128,182828356398335418551947690004935940599_u128,193874733776141396268256872547915100264_u128];
_6 = [(-3949855700799695076_i64),(-6434354335599687538_i64),(-8915043121666756530_i64),(-4122974786566949105_i64),3316416608962941155_i64];
_24 = _14;
_24 = !_14;
_23 = !true;
_15 = '\u{93233}';
_18 = _20 - _20;
_12.0 = [_15,_15];
_12 = (_22.0,);
_24 = _14;
match _16 {
50137 => bb7,
_ => bb6
}
}
bb6 = {
_11 = core::ptr::addr_of_mut!((*_11));
_16 = 59470_u16 + 28651_u16;
_1 = [913776057_u32,3058119383_u32,2803525580_u32,3778773542_u32];
_22.0 = [_15,_15];
RET = _7;
_1 = [3126064089_u32,4215445641_u32,748963912_u32,2504363954_u32];
(*_11) = 46692_u16;
_21 = [2941415780526056944981932333844748442_u128,6341901538290605472116730855354311381_u128,310160118086605676132224955231407412357_u128,238020880804614925994021795067042104677_u128,31230961180903175234235047814163384802_u128,83616911565024249446154068487760221848_u128,119561462252642532369164919369462324701_u128];
_3 = [2048968371_u32,4162862025_u32,675770875_u32,1647135739_u32];
_16 = 14395_u16 + 62412_u16;
_13 = _9 * _9;
RET = [1245386106_u32,2954198290_u32,3541603944_u32,748251725_u32];
Call(_26 = core::intrinsics::transmute(_15), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_22.1 = [_15,_15,_15,_15,_15,_15,_15];
_19.0 = [_15,_15];
_11 = core::ptr::addr_of_mut!((*_11));
_1 = [2154351962_u32,3288333084_u32,1378788730_u32,2555855194_u32];
_8.fld0 = _7;
_21 = [242396849389396731456441482547765130825_u128,72870538235239853945537298495942016218_u128,129993245129381457474925990705500615847_u128,173562948917180023136897061499186410058_u128,323093107546836391186320294227669539921_u128,222457284258872201010784289751932547646_u128,162366488234365066719954801380241000325_u128];
_14 = 5642065292376409729_u64 as i32;
match _16 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb8,
50137 => bb10,
_ => bb9
}
}
bb8 = {
RET = [293955238_u32,2307764333_u32,182918646_u32,2242278156_u32];
RET = _1;
_13 = _9;
RET = _8.fld0;
_11 = core::ptr::addr_of_mut!(_16);
_6 = _10;
_1 = _3;
(*_11) = 46362_u16;
RET = [867969864_u32,1932784703_u32,3335866961_u32,1635613789_u32];
_9 = _2;
_6 = _10;
_16 = 78291696_u32 as u16;
_17 = 61_i16 as f32;
_8.fld0 = RET;
_14 = 1353422286_u32 as i32;
_5 = _4;
_18 = _14 as f64;
_16 = 14285_u16 + 5676_u16;
_13 = _14 as isize;
_9 = !_2;
(*_11) = !27811_u16;
Goto(bb2)
}
bb9 = {
_19 = _12;
_5 = _4;
_20 = _18;
_8.fld0 = [524761951_u32,79136512_u32,896083222_u32,2783163345_u32];
_18 = _20 + _20;
_6 = _5;
_9 = -_13;
_18 = _20;
_8.fld0 = _1;
_3 = [569502560_u32,555164318_u32,3150321995_u32,1267278135_u32];
_18 = _20;
_2 = _9 | _9;
_22.2 = ['\u{1c03c}','\u{f5e6d}'];
_19.0 = _22.2;
_7 = [555251092_u32,55444628_u32,1259783413_u32,4134308789_u32];
_20 = -_18;
_22.2 = ['\u{5a1d3}','\u{43894}'];
(*_11) = !17296_u16;
_22.1 = ['\u{2552e}','\u{a26d4}','\u{77a67}','\u{80f75}','\u{b40c}','\u{e6587}','\u{d52b6}'];
_13 = _2;
_19 = _12;
_17 = 2861310002475344632_u64 as f32;
_15 = '\u{feb45}';
_22.2 = [_15,_15];
Goto(bb4)
}
bb10 = {
_25 = Adt46::Variant3 { fld0: _22.1,fld1: _12.0 };
_18 = 4_usize as f64;
_5 = [(-2030067389038502532_i64),(-3737816315964489750_i64),(-8195226564633073530_i64),(-7382292528944501255_i64),(-6964269174141968459_i64)];
_9 = _27;
_3 = [1325500451_u32,2757553236_u32,2002829145_u32,1961120739_u32];
_9 = (-7115985899742594917_i64) as isize;
_22.1 = [_15,_15,_15,_15,_15,_15,_15];
place!(Field::<[char; 7]>(Variant(_25, 3), 0)) = [_15,_15,_15,_15,_15,_15,_15];
match _16 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb11,
6 => bb12,
50137 => bb14,
_ => bb13
}
}
bb11 = {
_11 = core::ptr::addr_of_mut!((*_11));
_16 = 59470_u16 + 28651_u16;
_1 = [913776057_u32,3058119383_u32,2803525580_u32,3778773542_u32];
_22.0 = [_15,_15];
RET = _7;
_1 = [3126064089_u32,4215445641_u32,748963912_u32,2504363954_u32];
(*_11) = 46692_u16;
_21 = [2941415780526056944981932333844748442_u128,6341901538290605472116730855354311381_u128,310160118086605676132224955231407412357_u128,238020880804614925994021795067042104677_u128,31230961180903175234235047814163384802_u128,83616911565024249446154068487760221848_u128,119561462252642532369164919369462324701_u128];
_3 = [2048968371_u32,4162862025_u32,675770875_u32,1647135739_u32];
_16 = 14395_u16 + 62412_u16;
_13 = _9 * _9;
RET = [1245386106_u32,2954198290_u32,3541603944_u32,748251725_u32];
Call(_26 = core::intrinsics::transmute(_15), ReturnTo(bb5), UnwindUnreachable())
}
bb12 = {
_19 = _12;
_5 = _4;
_20 = _18;
_8.fld0 = [524761951_u32,79136512_u32,896083222_u32,2783163345_u32];
_18 = _20 + _20;
_6 = _5;
_9 = -_13;
_18 = _20;
_8.fld0 = _1;
_3 = [569502560_u32,555164318_u32,3150321995_u32,1267278135_u32];
_18 = _20;
_2 = _9 | _9;
_22.2 = ['\u{1c03c}','\u{f5e6d}'];
_19.0 = _22.2;
_7 = [555251092_u32,55444628_u32,1259783413_u32,4134308789_u32];
_20 = -_18;
_22.2 = ['\u{5a1d3}','\u{43894}'];
(*_11) = !17296_u16;
_22.1 = ['\u{2552e}','\u{a26d4}','\u{77a67}','\u{80f75}','\u{b40c}','\u{e6587}','\u{d52b6}'];
_13 = _2;
_19 = _12;
_17 = 2861310002475344632_u64 as f32;
_15 = '\u{feb45}';
_22.2 = [_15,_15];
Goto(bb4)
}
bb13 = {
_22.1 = [_15,_15,_15,_15,_15,_15,_15];
_19.0 = [_15,_15];
_11 = core::ptr::addr_of_mut!((*_11));
_1 = [2154351962_u32,3288333084_u32,1378788730_u32,2555855194_u32];
_8.fld0 = _7;
_21 = [242396849389396731456441482547765130825_u128,72870538235239853945537298495942016218_u128,129993245129381457474925990705500615847_u128,173562948917180023136897061499186410058_u128,323093107546836391186320294227669539921_u128,222457284258872201010784289751932547646_u128,162366488234365066719954801380241000325_u128];
_14 = 5642065292376409729_u64 as i32;
match _16 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb8,
50137 => bb10,
_ => bb9
}
}
bb14 = {
_29 = [(-2265315668022246203_i64),(-4709025496933633400_i64),(-1978105470521579936_i64),(-3876825483015931782_i64),(-2179691674018499666_i64)];
_3 = [317399441_u32,1019516807_u32,4132856588_u32,555023484_u32];
_27 = (-80480382844705416384554750361387941624_i128) as isize;
_32.1 = _23 as i64;
place!(Field::<[char; 7]>(Variant(_25, 3), 0)) = [_15,_15,_15,_15,_15,_15,_15];
_31 = 11080811923006615329_u64 as usize;
_31 = !0_usize;
_19.0 = [_15,_15];
_30 = _14 << (*_11);
_14 = -_30;
RET = _7;
_10 = [_32.1,_32.1,_32.1,_32.1,_32.1];
_1 = [1239853319_u32,2253896422_u32,3013759161_u32,1593637484_u32];
_3 = [1602142567_u32,1222009095_u32,1185140417_u32,3448285328_u32];
_22 = (Field::<[char; 2]>(Variant(_25, 3), 1), Field::<[char; 7]>(Variant(_25, 3), 0), Field::<[char; 2]>(Variant(_25, 3), 1));
_32.1 = !6919345073787115699_i64;
_6 = [_32.1,_32.1,_32.1,_32.1,_32.1];
SetDiscriminant(_25, 0);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(3_usize, 7_usize, Move(_7), 15_usize, Move(_15), 19_usize, Move(_19), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(3_usize, 24_usize, Move(_24), 10_usize, Move(_10), 22_usize, Move(_22), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(3_usize, 9_usize, Move(_9), 12_usize, Move(_12), 30_usize, Move(_30), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: Adt44,mut _2: [u32; 4],mut _3: ([char; 2],),mut _4: f32,mut _5: u16,mut _6: isize,mut _7: [u32; 4],mut _8: isize,mut _9: [i64; 5],mut _10: [i64; 5],mut _11: [u32; 4],mut _12: u16,mut _13: ([char; 2],),mut _14: [u32; 4],mut _15: [i64; 5]) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _16: Adt44;
let _17: [i128; 4];
let _18: isize;
let _19: i8;
let _20: *const i32;
let _21: ([char; 2],);
let _22: Adt45;
let _23: Adt49;
let _24: char;
let _25: u8;
let _26: i32;
let _27: [u128; 7];
let _28: isize;
let _29: isize;
let _30: [u32; 4];
let _31: bool;
let _32: ();
let _33: ();
{
_8 = _6;
_13 = _3;
RET = _15;
_3.0 = ['\u{4dbd6}','\u{3def6}'];
_10 = _9;
_16.fld0 = [1550837704_u32,4020083005_u32,2488846737_u32,3733197310_u32];
_7 = [2618628723_u32,44956369_u32,3414270358_u32,1616824436_u32];
_14 = _2;
_17 = [108301645663176518027060415712077953909_i128,89055928736496993603455543899238782247_i128,(-105742496334366475605840714846114704480_i128),93249390420121045272993741757356607452_i128];
_7 = [2560366500_u32,550359888_u32,3839475955_u32,995692156_u32];
_6 = _8 ^ _8;
_18 = _12 as isize;
_6 = _8 >> _12;
_4 = _5 as f32;
_14 = [810662355_u32,3245794351_u32,3706072884_u32,4052743410_u32];
_16.fld0 = _2;
match _12 {
47962 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3.0 = ['\u{a2c9b}','\u{dd9c7}'];
_19 = _4 as i8;
_12 = _5 ^ _5;
_5 = _12;
RET = _15;
_3.0 = _13.0;
_13 = (_3.0,);
_16.fld0 = [1157389609_u32,1598921714_u32,2966214473_u32,851927256_u32];
_17 = [88142850389349988180592392697863576148_i128,31394440679961460168091160586911002719_i128,(-103777975796805693535008364390023598350_i128),(-88216873073316859286808536095844831707_i128)];
_6 = _18;
_3.0 = ['\u{d3cff}','\u{9a91}'];
_5 = !_12;
_2 = [2681140335_u32,2859435829_u32,2526361360_u32,1908305531_u32];
_4 = (-8139_i16) as f32;
_2 = [2960902028_u32,3407271582_u32,1730188458_u32,2980000071_u32];
_3 = (_13.0,);
_14 = _2;
_4 = (-167828219389294113908585739778801392145_i128) as f32;
_17 = [83431056106826483474020469632363327265_i128,(-106232637111398904355869263672294019732_i128),(-37241041663775312813706623740418577793_i128),131407531267474583404319619527475865163_i128];
_8 = _18;
_16.fld0 = [3341708434_u32,2194421051_u32,1164283545_u32,1471500257_u32];
_12 = _5;
_4 = (-42387902157567644133513553899209599467_i128) as f32;
_7 = _16.fld0;
RET = [4523075688812211618_i64,(-8128994326167476131_i64),5516407429315178658_i64,5647521427809421699_i64,6148710873071689454_i64];
_13.0 = ['\u{6c0ec}','\u{d174c}'];
_5 = !_12;
RET = [(-1393308663351961398_i64),869410494782863566_i64,(-1988110461301342455_i64),(-8285266322308826978_i64),(-1419116387903104680_i64)];
_15 = [6429604080574778384_i64,8702056010988218813_i64,(-4628092123310450340_i64),(-5679954700399722471_i64),6972491929960217943_i64];
_3 = _13;
Goto(bb3)
}
bb3 = {
_21 = (_13.0,);
_7 = [2480206292_u32,3238548503_u32,2229250203_u32,1153396421_u32];
_13 = (_3.0,);
_16.fld0 = [1216390120_u32,2645408511_u32,1283608710_u32,430134609_u32];
_10 = [(-5690489799597355584_i64),1012342488453702915_i64,4198217694443445761_i64,998891431643624522_i64,1990590250528097003_i64];
Call(_20 = fn5(_7, _8, _12, _3, _12, _21.0, _7, _19, _11, _7, _9, _17, _16.fld0, _10, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.fld0 = [44399956_u32,4128991141_u32,2259491031_u32,3739420108_u32];
_14 = _16.fld0;
_7 = [1106807184_u32,1755590588_u32,1952809761_u32,899543813_u32];
Goto(bb5)
}
bb5 = {
_21 = _13;
_2 = [4017796032_u32,41820774_u32,2664511215_u32,2542334189_u32];
_3 = (_13.0,);
_5 = _12;
_13.0 = ['\u{858}','\u{c569e}'];
_22.fld0 = [39_u8,119_u8,33_u8,36_u8,3_u8,84_u8,13_u8];
_22.fld2 = -_6;
_15 = [(-7770156256350133642_i64),933270556653117490_i64,(-3249151676552771067_i64),(-6122314249893974660_i64),2809976323399125327_i64];
Call(_19 = core::intrinsics::bswap((-104_i8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = Move(_1);
_18 = 19456_i16 as isize;
_9 = _10;
_18 = _12 as isize;
Goto(bb7)
}
bb7 = {
_22.fld1.2 = ['\u{bd22c}','\u{93f5c}'];
_16.fld0 = _2;
_1 = Adt44 { fld0: _16.fld0 };
_19 = (-1137448291_i32) as i8;
_6 = _8;
_21.0 = ['\u{be250}','\u{861e}'];
_22.fld1.0 = ['\u{2807}','\u{108b2f}'];
_25 = 28_u8;
_10 = _9;
_21 = _3;
_15 = _10;
_1 = Move(_16);
_20 = core::ptr::addr_of!(_26);
_6 = _4 as isize;
_14 = _11;
_22.fld1.1 = ['\u{813a0}','\u{40a38}','\u{9d925}','\u{cf69f}','\u{ad0f5}','\u{32994}','\u{b4257}'];
(*_20) = 907046250_i32;
_27 = [309961988116340614828154434695487304082_u128,276616097189278797098218905828142278908_u128,103022625761485741627469511779393748651_u128,45326238008515648840612649040662354075_u128,87785949768484260314711380026314793499_u128,205301827577718467905612111286823241874_u128,233352817619942295264163903429940159925_u128];
_20 = core::ptr::addr_of!(_26);
RET = _15;
_26 = -1103015388_i32;
_16.fld0 = [2521291327_u32,3595316438_u32,3330730282_u32,2705447720_u32];
_4 = 2967780867_u32 as f32;
_12 = _5;
_28 = _18 ^ _18;
(*_20) = 1660735317_i32 - (-903919828_i32);
_22.fld1.0 = ['\u{bf163}','\u{ec8d3}'];
match _25 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
28 => bb14,
_ => bb13
}
}
bb8 = {
_16 = Move(_1);
_18 = 19456_i16 as isize;
_9 = _10;
_18 = _12 as isize;
Goto(bb7)
}
bb9 = {
_21 = _13;
_2 = [4017796032_u32,41820774_u32,2664511215_u32,2542334189_u32];
_3 = (_13.0,);
_5 = _12;
_13.0 = ['\u{858}','\u{c569e}'];
_22.fld0 = [39_u8,119_u8,33_u8,36_u8,3_u8,84_u8,13_u8];
_22.fld2 = -_6;
_15 = [(-7770156256350133642_i64),933270556653117490_i64,(-3249151676552771067_i64),(-6122314249893974660_i64),2809976323399125327_i64];
Call(_19 = core::intrinsics::bswap((-104_i8)), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_1.fld0 = [44399956_u32,4128991141_u32,2259491031_u32,3739420108_u32];
_14 = _16.fld0;
_7 = [1106807184_u32,1755590588_u32,1952809761_u32,899543813_u32];
Goto(bb5)
}
bb11 = {
_21 = (_13.0,);
_7 = [2480206292_u32,3238548503_u32,2229250203_u32,1153396421_u32];
_13 = (_3.0,);
_16.fld0 = [1216390120_u32,2645408511_u32,1283608710_u32,430134609_u32];
_10 = [(-5690489799597355584_i64),1012342488453702915_i64,4198217694443445761_i64,998891431643624522_i64,1990590250528097003_i64];
Call(_20 = fn5(_7, _8, _12, _3, _12, _21.0, _7, _19, _11, _7, _9, _17, _16.fld0, _10, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_3.0 = ['\u{a2c9b}','\u{dd9c7}'];
_19 = _4 as i8;
_12 = _5 ^ _5;
_5 = _12;
RET = _15;
_3.0 = _13.0;
_13 = (_3.0,);
_16.fld0 = [1157389609_u32,1598921714_u32,2966214473_u32,851927256_u32];
_17 = [88142850389349988180592392697863576148_i128,31394440679961460168091160586911002719_i128,(-103777975796805693535008364390023598350_i128),(-88216873073316859286808536095844831707_i128)];
_6 = _18;
_3.0 = ['\u{d3cff}','\u{9a91}'];
_5 = !_12;
_2 = [2681140335_u32,2859435829_u32,2526361360_u32,1908305531_u32];
_4 = (-8139_i16) as f32;
_2 = [2960902028_u32,3407271582_u32,1730188458_u32,2980000071_u32];
_3 = (_13.0,);
_14 = _2;
_4 = (-167828219389294113908585739778801392145_i128) as f32;
_17 = [83431056106826483474020469632363327265_i128,(-106232637111398904355869263672294019732_i128),(-37241041663775312813706623740418577793_i128),131407531267474583404319619527475865163_i128];
_8 = _18;
_16.fld0 = [3341708434_u32,2194421051_u32,1164283545_u32,1471500257_u32];
_12 = _5;
_4 = (-42387902157567644133513553899209599467_i128) as f32;
_7 = _16.fld0;
RET = [4523075688812211618_i64,(-8128994326167476131_i64),5516407429315178658_i64,5647521427809421699_i64,6148710873071689454_i64];
_13.0 = ['\u{6c0ec}','\u{d174c}'];
_5 = !_12;
RET = [(-1393308663351961398_i64),869410494782863566_i64,(-1988110461301342455_i64),(-8285266322308826978_i64),(-1419116387903104680_i64)];
_15 = [6429604080574778384_i64,8702056010988218813_i64,(-4628092123310450340_i64),(-5679954700399722471_i64),6972491929960217943_i64];
_3 = _13;
Goto(bb3)
}
bb13 = {
Return()
}
bb14 = {
_22.fld1.0 = ['\u{22853}','\u{43cbe}'];
_2 = [1589747067_u32,845936917_u32,3260083005_u32,891773409_u32];
_6 = !_28;
_12 = !_5;
_24 = '\u{1084ea}';
_1 = Adt44 { fld0: _2 };
_1 = Adt44 { fld0: _2 };
_21 = (_13.0,);
_24 = '\u{fb345}';
_22.fld1.0 = [_24,_24];
_7 = [1422612102_u32,246180487_u32,4109579639_u32,1189659898_u32];
_3 = _13;
_18 = _28 >> (*_20);
_1.fld0 = [1022214546_u32,215232717_u32,3853401956_u32,3003051235_u32];
_29 = -_6;
_18 = -_6;
_29 = _6;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(4_usize, 5_usize, Move(_5), 21_usize, Move(_21), 12_usize, Move(_12), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(4_usize, 7_usize, Move(_7), 24_usize, Move(_24), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(4_usize, 2_usize, Move(_2), 29_usize, Move(_29), 14_usize, Move(_14), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u32; 4],mut _2: isize,mut _3: u16,mut _4: ([char; 2],),mut _5: u16,mut _6: [char; 2],mut _7: [u32; 4],mut _8: i8,mut _9: [u32; 4],mut _10: [u32; 4],mut _11: [i64; 5],mut _12: [i128; 4],mut _13: [u32; 4],mut _14: [i64; 5],mut _15: [i64; 5]) -> *const i32 {
mir! {
type RET = *const i32;
let _16: char;
let _17: usize;
let _18: Adt51;
let _19: isize;
let _20: [u128; 7];
let _21: [char; 2];
let _22: [u32; 4];
let _23: Adt57;
let _24: (u32, i64);
let _25: ([char; 2],);
let _26: *const i32;
let _27: [i128; 4];
let _28: Adt45;
let _29: [i128; 4];
let _30: isize;
let _31: [u32; 4];
let _32: bool;
let _33: ([char; 2], [char; 7], [char; 2]);
let _34: Adt45;
let _35: Adt46;
let _36: Adt57;
let _37: isize;
let _38: u64;
let _39: [u32; 4];
let _40: f32;
let _41: f64;
let _42: Adt42;
let _43: [char; 2];
let _44: i32;
let _45: f64;
let _46: u32;
let _47: char;
let _48: Adt52;
let _49: [char; 7];
let _50: usize;
let _51: i16;
let _52: ([char; 2], [char; 7], [char; 2]);
let _53: Adt44;
let _54: f32;
let _55: bool;
let _56: u16;
let _57: char;
let _58: [u8; 7];
let _59: [char; 7];
let _60: [u128; 7];
let _61: isize;
let _62: bool;
let _63: f32;
let _64: (u32, i64);
let _65: Adt50;
let _66: isize;
let _67: bool;
let _68: isize;
let _69: f32;
let _70: [i64; 5];
let _71: ();
let _72: ();
{
_13 = [2910668681_u32,240313629_u32,994633071_u32,4069900851_u32];
_1 = [1280901531_u32,3938830973_u32,3605561870_u32,2286471148_u32];
_6 = _4.0;
_15 = [6584752609004481317_i64,3841332018615872852_i64,(-4196701002662911411_i64),618649590909788091_i64,(-955056893995220672_i64)];
_14 = [1933465930688021490_i64,6355490141636176109_i64,(-5885728070487482975_i64),(-7871387861454017049_i64),7740780835786295857_i64];
_2 = _8 as isize;
_11 = _14;
_13 = _7;
_17 = 1976161780672064712_i64 as usize;
_10 = [1369313323_u32,1958119239_u32,2531565325_u32,1817718697_u32];
_4.0 = ['\u{ec626}','\u{e23a0}'];
_16 = '\u{b884}';
_17 = !1_usize;
Call(_1 = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (-128_i8) & (-57_i8);
_17 = !7_usize;
_17 = !7_usize;
_4.0 = [_16,_16];
_16 = '\u{bcba7}';
_3 = _5 >> _2;
_6 = [_16,_16];
_10 = [2621638444_u32,128100646_u32,245454657_u32,3286038599_u32];
_1 = [3750747425_u32,1468157363_u32,3852243123_u32,4261753927_u32];
_14 = [(-2311853010796614262_i64),8869489777829997104_i64,(-3444026259062078300_i64),5672791835289360039_i64,(-7486010479445010751_i64)];
_8 = 17_i8;
_5 = _3 - _3;
_19 = _2;
_4 = (_6,);
Goto(bb2)
}
bb2 = {
_4 = (_6,);
_10 = [2483501836_u32,4276838349_u32,3548432113_u32,4106615554_u32];
_15 = [(-1872666783605347553_i64),(-269546624575428860_i64),2881074748330460605_i64,1357750094322707260_i64,(-4102039331048241996_i64)];
_14 = _15;
_6 = [_16,_16];
_5 = !_3;
_22 = [4138211925_u32,1452064894_u32,1523587311_u32,3307205928_u32];
Call(_20 = fn6(_9, _22, _11, _6, _12, _19, _11, _13, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _14;
_13 = _9;
_15 = [5607309279082641119_i64,(-4613406316595911477_i64),(-4987749814706894679_i64),1218475940737705283_i64,(-4818522402082161362_i64)];
_9 = _13;
_21 = [_16,_16];
_10 = [4132928639_u32,624829586_u32,1732344591_u32,3980894151_u32];
_2 = _19 << _5;
_4.0 = [_16,_16];
_15 = [(-3854322846553552164_i64),(-9104396448021160029_i64),2150436041782906050_i64,(-4751456474633902237_i64),(-8701052181941149037_i64)];
_17 = 10364299132948642093_usize;
_24.0 = 2414148863_u32 << _17;
_18 = Adt51::Variant1 { fld0: _8 };
_5 = !_3;
_4 = (_21,);
_17 = !7_usize;
_4 = (_6,);
_13 = [_24.0,_24.0,_24.0,_24.0];
_24.1 = -(-4245929553390771411_i64);
_27 = [(-62849468721609156055121550457706353041_i128),(-89853219326233664381979967814226511488_i128),133919864911553264643622597169585451133_i128,(-147310638006789939603906833455124638910_i128)];
_4 = (_21,);
_24.1 = (-8758225655759608066_i64) | (-2437359661944725115_i64);
_5 = _3 + _3;
_24.0 = 119009070_u32;
Goto(bb4)
}
bb4 = {
_7 = [_24.0,_24.0,_24.0,_24.0];
_17 = 2082342182617307458_usize & 12762367593067694764_usize;
_25 = _4;
_24 = (3101929741_u32, (-7351554039909471996_i64));
_24.1 = (-236462303222868465620146805595624359_i128) as i64;
_1 = _9;
_5 = _3;
_32 = false;
_9 = [_24.0,_24.0,_24.0,_24.0];
_28.fld1.2 = _21;
_28.fld1.0 = [_16,_16];
_28.fld2 = (-2011509138_i32) as isize;
_24.1 = (-4265293742364640501_i64);
_31 = [_24.0,_24.0,_24.0,_24.0];
SetDiscriminant(_18, 0);
_29 = _27;
place!(Field::<i64>(Variant(_18, 0), 2)) = _24.1 << _5;
_34.fld1.1 = [_16,_16,_16,_16,_16,_16,_16];
_3 = !_5;
_14 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)) = (_6, _34.fld1.1, _4.0);
_30 = _19 << _24.0;
_7 = [_24.0,_24.0,_24.0,_24.0];
_8 = _17 as i8;
Goto(bb5)
}
bb5 = {
_33.0 = _28.fld1.2;
_28.fld1.0 = [_16,_16];
_33.1 = [_16,_16,_16,_16,_16,_16,_16];
_24.0 = 311349716_u32 >> _30;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).0 = [_16,_16];
_13 = [_24.0,_24.0,_24.0,_24.0];
_28.fld0 = [28_u8,138_u8,147_u8,31_u8,65_u8,236_u8,97_u8];
_15 = _14;
_28.fld1.2 = _21;
Goto(bb6)
}
bb6 = {
_34 = Adt45 { fld0: _28.fld0,fld1: Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0),fld2: _30 };
_28.fld1.2 = [_16,_16];
_28 = _34;
_29 = [83304728993526549974954200696024801314_i128,136001483896210714542510147878273054003_i128,34272081013769099192784123283237191983_i128,16928591213792813895810669544505140368_i128];
_33.1 = [_16,_16,_16,_16,_16,_16,_16];
_24.0 = 4010325762_u32;
_4.0 = [_16,_16];
_5 = _3 + _3;
_2 = -_19;
_33.0 = [_16,_16];
_28.fld1.1 = [_16,_16,_16,_16,_16,_16,_16];
_41 = 189832254_i32 as f64;
_34.fld2 = 319831040590009707025040507465676198238_u128 as isize;
_41 = 14609862187508381867_u64 as f64;
_28.fld1.2 = [_16,_16];
_34.fld1.2 = _34.fld1.0;
_5 = 218_u8 as u16;
_25 = (_34.fld1.2,);
_33 = (Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0, _28.fld1.1, _34.fld1.2);
Goto(bb7)
}
bb7 = {
_34.fld0 = [91_u8,66_u8,123_u8,57_u8,139_u8,56_u8,134_u8];
_15 = [Field::<i64>(Variant(_18, 0), 2),_24.1,Field::<i64>(Variant(_18, 0), 2),_24.1,_24.1];
_49 = [_16,_16,_16,_16,_16,_16,_16];
place!(Field::<Adt44>(Variant(_18, 0), 3)).fld0 = [_24.0,_24.0,_24.0,_24.0];
_10 = _13;
_33.0 = [_16,_16];
_12 = [(-8002965345412024065571335830552609913_i128),(-101278979810386087752036148335842890068_i128),(-147741109959779016939974160516075024566_i128),59145412032413242688672656147330642074_i128];
_41 = 241_u8 as f64;
_6 = _34.fld1.2;
_16 = '\u{10a64f}';
_31 = _10;
_46 = !_24.0;
_44 = !2131365830_i32;
_31 = [_24.0,_46,_46,_46];
match _24.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
4010325762 => bb10,
_ => bb9
}
}
bb8 = {
_34 = Adt45 { fld0: _28.fld0,fld1: Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0),fld2: _30 };
_28.fld1.2 = [_16,_16];
_28 = _34;
_29 = [83304728993526549974954200696024801314_i128,136001483896210714542510147878273054003_i128,34272081013769099192784123283237191983_i128,16928591213792813895810669544505140368_i128];
_33.1 = [_16,_16,_16,_16,_16,_16,_16];
_24.0 = 4010325762_u32;
_4.0 = [_16,_16];
_5 = _3 + _3;
_2 = -_19;
_33.0 = [_16,_16];
_28.fld1.1 = [_16,_16,_16,_16,_16,_16,_16];
_41 = 189832254_i32 as f64;
_34.fld2 = 319831040590009707025040507465676198238_u128 as isize;
_41 = 14609862187508381867_u64 as f64;
_28.fld1.2 = [_16,_16];
_34.fld1.2 = _34.fld1.0;
_5 = 218_u8 as u16;
_25 = (_34.fld1.2,);
_33 = (Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0, _28.fld1.1, _34.fld1.2);
Goto(bb7)
}
bb9 = {
_4 = (_6,);
_10 = [2483501836_u32,4276838349_u32,3548432113_u32,4106615554_u32];
_15 = [(-1872666783605347553_i64),(-269546624575428860_i64),2881074748330460605_i64,1357750094322707260_i64,(-4102039331048241996_i64)];
_14 = _15;
_6 = [_16,_16];
_5 = !_3;
_22 = [4138211925_u32,1452064894_u32,1523587311_u32,3307205928_u32];
Call(_20 = fn6(_9, _22, _11, _6, _12, _19, _11, _13, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_34.fld1.0 = [_16,_16];
_52.1 = [_16,_16,_16,_16,_16,_16,_16];
RET = core::ptr::addr_of!(_44);
_11 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
RET = core::ptr::addr_of!(_44);
_10 = [_46,_46,_46,_46];
_51 = 29606_i16;
_12 = [75463779916858436845693652539890628003_i128,119664365990463465812385604067932413984_i128,(-67248695131284601645005928303859381997_i128),110160771717304717763421210600616029300_i128];
_10 = _7;
RET = core::ptr::addr_of!((*RET));
_34.fld1.0 = [_16,_16];
_47 = _16;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).0 = [_47,_16];
_28.fld1 = (_34.fld1.0, _52.1, _6);
_1 = [_46,_24.0,_46,_46];
place!(Field::<Adt44>(Variant(_18, 0), 3)) = Adt44 { fld0: _13 };
_52.0 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0;
_14 = [Field::<i64>(Variant(_18, 0), 2),_24.1,Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
_55 = _32;
_17 = _3 as usize;
Goto(bb11)
}
bb11 = {
_15 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
_52.0 = [_16,_16];
_28.fld1.1 = [_16,_47,_47,_16,_47,_47,_47];
_39 = [_24.0,_24.0,_46,_46];
_28.fld1.1 = [_16,_16,_47,_47,_16,_47,_16];
_43 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).2;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).1 = [_47,_16,_16,_47,_47,_47,_16];
(*RET) = _34.fld2 as i32;
_22 = [_46,_24.0,_46,_46];
place!(Field::<Adt44>(Variant(_18, 0), 3)).fld0 = [_46,_24.0,_24.0,_24.0];
_28.fld1 = _34.fld1;
_48 = Adt52::Variant1 { fld0: _3 };
SetDiscriminant(_48, 1);
_4 = (Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).2,);
(*RET) = (-433260981_i32) >> _30;
_34.fld1 = (_43, _52.1, _28.fld1.0);
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).2 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0;
_34.fld1.0 = [_47,_16];
_1 = [_46,_46,_46,_24.0];
_4.0 = [_47,_16];
_25 = (_43,);
_52.2 = _6;
_63 = _41 as f32;
match _24.0 {
4010325762 => bb12,
_ => bb7
}
}
bb12 = {
_49 = [_47,_47,_16,_16,_47,_47,_47];
(*RET) = -(-1225510541_i32);
_33.1 = [_47,_47,_16,_16,_16,_47,_16];
_6 = _43;
_60 = [33046286551273614767705561403171193393_u128,23767079637607004700611880301572450948_u128,237297507741733160835774075030682366584_u128,90616188860568510862344904220862026335_u128,336438005325140252765728632158326055899_u128,300170876623542665310077529640174752658_u128,296225378425904203746704163211808708853_u128];
Goto(bb13)
}
bb13 = {
_58 = _34.fld0;
match _24.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
4010325762 => bb21,
_ => bb20
}
}
bb14 = {
_33.0 = _28.fld1.2;
_28.fld1.0 = [_16,_16];
_33.1 = [_16,_16,_16,_16,_16,_16,_16];
_24.0 = 311349716_u32 >> _30;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).0 = [_16,_16];
_13 = [_24.0,_24.0,_24.0,_24.0];
_28.fld0 = [28_u8,138_u8,147_u8,31_u8,65_u8,236_u8,97_u8];
_15 = _14;
_28.fld1.2 = _21;
Goto(bb6)
}
bb15 = {
_15 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
_52.0 = [_16,_16];
_28.fld1.1 = [_16,_47,_47,_16,_47,_47,_47];
_39 = [_24.0,_24.0,_46,_46];
_28.fld1.1 = [_16,_16,_47,_47,_16,_47,_16];
_43 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).2;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).1 = [_47,_16,_16,_47,_47,_47,_16];
(*RET) = _34.fld2 as i32;
_22 = [_46,_24.0,_46,_46];
place!(Field::<Adt44>(Variant(_18, 0), 3)).fld0 = [_46,_24.0,_24.0,_24.0];
_28.fld1 = _34.fld1;
_48 = Adt52::Variant1 { fld0: _3 };
SetDiscriminant(_48, 1);
_4 = (Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).2,);
(*RET) = (-433260981_i32) >> _30;
_34.fld1 = (_43, _52.1, _28.fld1.0);
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).2 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0;
_34.fld1.0 = [_47,_16];
_1 = [_46,_46,_46,_24.0];
_4.0 = [_47,_16];
_25 = (_43,);
_52.2 = _6;
_63 = _41 as f32;
match _24.0 {
4010325762 => bb12,
_ => bb7
}
}
bb16 = {
_34.fld1.0 = [_16,_16];
_52.1 = [_16,_16,_16,_16,_16,_16,_16];
RET = core::ptr::addr_of!(_44);
_11 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
RET = core::ptr::addr_of!(_44);
_10 = [_46,_46,_46,_46];
_51 = 29606_i16;
_12 = [75463779916858436845693652539890628003_i128,119664365990463465812385604067932413984_i128,(-67248695131284601645005928303859381997_i128),110160771717304717763421210600616029300_i128];
_10 = _7;
RET = core::ptr::addr_of!((*RET));
_34.fld1.0 = [_16,_16];
_47 = _16;
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)).0 = [_47,_16];
_28.fld1 = (_34.fld1.0, _52.1, _6);
_1 = [_46,_24.0,_46,_46];
place!(Field::<Adt44>(Variant(_18, 0), 3)) = Adt44 { fld0: _13 };
_52.0 = Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0;
_14 = [Field::<i64>(Variant(_18, 0), 2),_24.1,Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
_55 = _32;
_17 = _3 as usize;
Goto(bb11)
}
bb17 = {
_8 = (-128_i8) & (-57_i8);
_17 = !7_usize;
_17 = !7_usize;
_4.0 = [_16,_16];
_16 = '\u{bcba7}';
_3 = _5 >> _2;
_6 = [_16,_16];
_10 = [2621638444_u32,128100646_u32,245454657_u32,3286038599_u32];
_1 = [3750747425_u32,1468157363_u32,3852243123_u32,4261753927_u32];
_14 = [(-2311853010796614262_i64),8869489777829997104_i64,(-3444026259062078300_i64),5672791835289360039_i64,(-7486010479445010751_i64)];
_8 = 17_i8;
_5 = _3 - _3;
_19 = _2;
_4 = (_6,);
Goto(bb2)
}
bb18 = {
_4 = (_6,);
_10 = [2483501836_u32,4276838349_u32,3548432113_u32,4106615554_u32];
_15 = [(-1872666783605347553_i64),(-269546624575428860_i64),2881074748330460605_i64,1357750094322707260_i64,(-4102039331048241996_i64)];
_14 = _15;
_6 = [_16,_16];
_5 = !_3;
_22 = [4138211925_u32,1452064894_u32,1523587311_u32,3307205928_u32];
Call(_20 = fn6(_9, _22, _11, _6, _12, _19, _11, _13, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_7 = [_24.0,_24.0,_24.0,_24.0];
_17 = 2082342182617307458_usize & 12762367593067694764_usize;
_25 = _4;
_24 = (3101929741_u32, (-7351554039909471996_i64));
_24.1 = (-236462303222868465620146805595624359_i128) as i64;
_1 = _9;
_5 = _3;
_32 = false;
_9 = [_24.0,_24.0,_24.0,_24.0];
_28.fld1.2 = _21;
_28.fld1.0 = [_16,_16];
_28.fld2 = (-2011509138_i32) as isize;
_24.1 = (-4265293742364640501_i64);
_31 = [_24.0,_24.0,_24.0,_24.0];
SetDiscriminant(_18, 0);
_29 = _27;
place!(Field::<i64>(Variant(_18, 0), 2)) = _24.1 << _5;
_34.fld1.1 = [_16,_16,_16,_16,_16,_16,_16];
_3 = !_5;
_14 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0)) = (_6, _34.fld1.1, _4.0);
_30 = _19 << _24.0;
_7 = [_24.0,_24.0,_24.0,_24.0];
_8 = _17 as i8;
Goto(bb5)
}
bb20 = {
_34 = Adt45 { fld0: _28.fld0,fld1: Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0),fld2: _30 };
_28.fld1.2 = [_16,_16];
_28 = _34;
_29 = [83304728993526549974954200696024801314_i128,136001483896210714542510147878273054003_i128,34272081013769099192784123283237191983_i128,16928591213792813895810669544505140368_i128];
_33.1 = [_16,_16,_16,_16,_16,_16,_16];
_24.0 = 4010325762_u32;
_4.0 = [_16,_16];
_5 = _3 + _3;
_2 = -_19;
_33.0 = [_16,_16];
_28.fld1.1 = [_16,_16,_16,_16,_16,_16,_16];
_41 = 189832254_i32 as f64;
_34.fld2 = 319831040590009707025040507465676198238_u128 as isize;
_41 = 14609862187508381867_u64 as f64;
_28.fld1.2 = [_16,_16];
_34.fld1.2 = _34.fld1.0;
_5 = 218_u8 as u16;
_25 = (_34.fld1.2,);
_33 = (Field::<([char; 2], [char; 7], [char; 2])>(Variant(_18, 0), 0).0, _28.fld1.1, _34.fld1.2);
Goto(bb7)
}
bb21 = {
_12 = [(-68264132164028202202796571977150759090_i128),122642281316381805731066152360648552913_i128,141871147325342396156264655610918815039_i128,(-47642186646295840233985724763520293232_i128)];
(*RET) = !515728560_i32;
place!(Field::<u16>(Variant(_48, 1), 0)) = !_5;
_14 = [Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2),_24.1,Field::<i64>(Variant(_18, 0), 2),Field::<i64>(Variant(_18, 0), 2)];
_37 = -_34.fld2;
Goto(bb22)
}
bb22 = {
Call(_71 = dump_var(5_usize, 58_usize, Move(_58), 19_usize, Move(_19), 52_usize, Move(_52), 15_usize, Move(_15)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_71 = dump_var(5_usize, 1_usize, Move(_1), 32_usize, Move(_32), 13_usize, Move(_13), 60_usize, Move(_60)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_71 = dump_var(5_usize, 16_usize, Move(_16), 24_usize, Move(_24), 55_usize, Move(_55), 14_usize, Move(_14)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_71 = dump_var(5_usize, 49_usize, Move(_49), 46_usize, Move(_46), 31_usize, Move(_31), 20_usize, Move(_20)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_71 = dump_var(5_usize, 12_usize, Move(_12), 27_usize, Move(_27), 47_usize, Move(_47), 11_usize, Move(_11)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [u32; 4],mut _2: [u32; 4],mut _3: [i64; 5],mut _4: [char; 2],mut _5: [i128; 4],mut _6: isize,mut _7: [i64; 5],mut _8: [u32; 4],mut _9: [u32; 4]) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _10: Adt47;
let _11: Adt45;
let _12: (u32, i64);
let _13: [char; 7];
let _14: bool;
let _15: ([char; 2],);
let _16: (isize, i32, *const i32, i128);
let _17: [char; 7];
let _18: [u128; 7];
let _19: f32;
let _20: isize;
let _21: isize;
let _22: f32;
let _23: isize;
let _24: isize;
let _25: ();
let _26: ();
{
_6 = 15_isize - 9223372036854775807_isize;
_2 = [2065780847_u32,2377136061_u32,158379387_u32,2013004652_u32];
_7 = _3;
RET = [46777853443965470805237039043796900828_u128,210766757542028993492886782615790634294_u128,177129833152468195544410684578124408400_u128,135596792674976890703201374438919284887_u128,324107872269092845251946265809180361544_u128,137766722845111595888819697527575234986_u128,329825997602582072207492563417653621581_u128];
_4 = ['\u{14d4}','\u{1148}'];
_7 = [(-3328473795546064189_i64),197856240024250479_i64,3158324700044842677_i64,1189564191524188007_i64,(-9064950665525855474_i64)];
RET = [138530837146004090561485678167602773545_u128,64258235563645983717443972632357688067_u128,262386932466096852257592219344081591853_u128,91405593523849088527629057255689233546_u128,8963302566081324970223925016045703925_u128,171962802440419963666634209443604651735_u128,318023735616345103262516135974847584144_u128];
_6 = '\u{11349}' as isize;
_3 = [2224905229180880288_i64,(-4059838574911640858_i64),(-7995420224734741025_i64),(-6504717552594919947_i64),(-7157059263944142663_i64)];
_10.fld0 = (-6486346746939520596_i64);
_10.fld1.0 = _4;
_5 = [9975673046658307388540415928118563765_i128,82890853904394167436774921616651355965_i128,(-144338668372763079356281951889779502044_i128),(-111297808069594135115699107220906608688_i128)];
_10.fld3 = -96_i8;
_11.fld2 = -_6;
_12 = (752213797_u32, _10.fld0);
_9 = _8;
_11.fld1.2 = _10.fld1.0;
_8 = [_12.0,_12.0,_12.0,_12.0];
_1 = [_12.0,_12.0,_12.0,_12.0];
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
752213797 => bb9,
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
_11.fld0 = [155_u8,123_u8,59_u8,241_u8,51_u8,7_u8,172_u8];
_1 = [_12.0,_12.0,_12.0,_12.0];
_13 = ['\u{1bbc2}','\u{fcb5e}','\u{6bf22}','\u{4371d}','\u{5d304}','\u{631c1}','\u{b395b}'];
_12.1 = _10.fld0 | _10.fld0;
_11.fld1 = (_4, _13, _4);
_10.fld3 = 61_i8 - 36_i8;
_10.fld0 = !_12.1;
_12.0 = 4277037698_u32 + 3657594746_u32;
Goto(bb10)
}
bb10 = {
_11.fld1.2 = ['\u{c520a}','\u{74b23}'];
_6 = _11.fld2 & _11.fld2;
_10.fld3 = 48_i8;
_11.fld1.0 = ['\u{83ad3}','\u{73b90}'];
_3 = _7;
_10.fld1 = (_11.fld1.0,);
RET = [123620883304977794059238140923064393503_u128,23942814668045171025992375953625651361_u128,293635412905560995649187969219258404161_u128,87567614451650593984392194918064346652_u128,100379309043924545895745457052386196063_u128,125930660298660309951050913591572652490_u128,246748672265456254186207794060380396341_u128];
_9 = [_12.0,_12.0,_12.0,_12.0];
_13 = ['\u{87a4c}','\u{d8c2b}','\u{5de65}','\u{b5431}','\u{a9449}','\u{96b4c}','\u{8c651}'];
Call(_1 = fn7(_9, _11, _12.1, _11.fld1.0, _8, _10.fld1.0, _12.1, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = _7;
_5 = [31640064313528115804460142550979496421_i128,44333371523475140485885641628384135105_i128,102855958338340259614319138815415690489_i128,(-87408506901550339591238941648658798116_i128)];
RET = [309349537382402893576371583496327111015_u128,92451161489385241293310226289385934259_u128,188757992713971353569226098632647969075_u128,226981727407966814671780032091678918597_u128,223667684552489946635930196132213603252_u128,97135275466977774113150893454373939847_u128,301417534062088239801230501192984741907_u128];
_11.fld1 = (_10.fld1.0, _13, _10.fld1.0);
_13 = ['\u{2027b}','\u{78f88}','\u{ee83f}','\u{aeca4}','\u{10212b}','\u{1114}','\u{dd05c}'];
_10.fld1 = (_11.fld1.0,);
_15 = _10.fld1;
_9 = _2;
_16.0 = _6;
_12.0 = !2197910657_u32;
_10.fld1 = _15;
_14 = _10.fld0 <= _10.fld0;
match _10.fld3 {
0 => bb6,
1 => bb5,
48 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_2 = [_12.0,_12.0,_12.0,_12.0];
_10.fld2 = [_10.fld0,_10.fld0,_12.1,_12.1,_12.1];
_16.3 = 12490960099750837906_u64 as i128;
_19 = _10.fld3 as f32;
_17 = ['\u{89853}','\u{65463}','\u{10ad09}','\u{e81ec}','\u{96d0a}','\u{ad911}','\u{f0b06}'];
_10.fld1.0 = ['\u{19951}','\u{3b84e}'];
_11.fld1.1 = ['\u{9ca50}','\u{9c451}','\u{c5fa4}','\u{3912d}','\u{f5ec4}','\u{e537a}','\u{dde10}'];
_9 = [_12.0,_12.0,_12.0,_12.0];
_19 = 1228898712019206367_u64 as f32;
_16.1 = (-29620_i16) as i32;
_8 = [_12.0,_12.0,_12.0,_12.0];
_16.3 = 113401932215855449623404061467994120298_i128;
_13 = ['\u{cbba9}','\u{aae61}','\u{9ea21}','\u{2d3d3}','\u{453b5}','\u{71f99}','\u{a82cf}'];
_6 = _14 as isize;
_11.fld1.1 = ['\u{840e}','\u{1a288}','\u{b3f1e}','\u{4b126}','\u{1fc0b}','\u{c7163}','\u{f708a}'];
_12.1 = _10.fld0;
_20 = _12.0 as isize;
_15.0 = ['\u{7cd21}','\u{934d0}'];
_17 = _13;
_15 = (_10.fld1.0,);
_21 = _10.fld3 as isize;
_18 = [263713965587657828336623004478508046189_u128,216497813489860173700055152983652997230_u128,232515375002071377443037312774185059843_u128,315141483541715387933299675482939861920_u128,1399940098281439483763341923344041884_u128,172200955794623560090841413411784174880_u128,66893721822006516456092748453562279646_u128];
_16.2 = core::ptr::addr_of!(_16.1);
_12 = (2264599205_u32, _10.fld0);
_6 = (-30005_i16) as isize;
_16.3 = !47551547239765883120798708176241007204_i128;
_19 = _16.1 as f32;
_6 = _16.0 - _11.fld2;
Goto(bb14)
}
bb14 = {
_12 = (3942783825_u32, _10.fld0);
_10.fld0 = 7937336366561135489_usize as i64;
_11.fld1.2 = ['\u{ae271}','\u{322bc}'];
RET = _18;
_12.0 = 2324373439_u32 | 3534185283_u32;
_10.fld1.0 = _15.0;
_10.fld2 = _3;
_21 = _11.fld2;
_22 = _12.0 as f32;
_14 = !true;
_11.fld2 = 8977202061535141221_u64 as isize;
_7 = [_10.fld0,_12.1,_12.1,_10.fld0,_10.fld0];
_15.0 = ['\u{72a91}','\u{7e1c8}'];
_10.fld3 = 42_i8 >> _12.0;
_11.fld1.2 = ['\u{6d82d}','\u{5d779}'];
_10.fld1 = _15;
_16.0 = 7_usize as isize;
_10.fld1 = (_11.fld1.2,);
_13 = ['\u{1066af}','\u{10172c}','\u{a5a13}','\u{25544}','\u{94b58}','\u{84b04}','\u{4f971}'];
_16.2 = core::ptr::addr_of!(_16.1);
_9 = [_12.0,_12.0,_12.0,_12.0];
_11.fld1.2 = ['\u{14498}','\u{dd6d6}'];
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(6_usize, 15_usize, Move(_15), 14_usize, Move(_14), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(6_usize, 1_usize, Move(_1), 9_usize, Move(_9), 12_usize, Move(_12), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [u32; 4],mut _2: Adt45,mut _3: i64,mut _4: [char; 2],mut _5: [u32; 4],mut _6: [char; 2],mut _7: i64,mut _8: [u32; 4]) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _9: ([char; 2], [char; 7], [char; 2]);
let _10: isize;
let _11: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,));
let _12: ([char; 2],);
let _13: [u8; 7];
let _14: f64;
let _15: Adt56;
let _16: Adt44;
let _17: Adt52;
let _18: Adt50;
let _19: f64;
let _20: Adt53;
let _21: bool;
let _22: [char; 2];
let _23: bool;
let _24: f64;
let _25: [u32; 4];
let _26: Adt45;
let _27: isize;
let _28: i128;
let _29: char;
let _30: [char; 2];
let _31: u32;
let _32: f64;
let _33: (u32, i64);
let _34: isize;
let _35: isize;
let _36: Adt41;
let _37: ();
let _38: ();
{
_1 = _8;
_2.fld0 = [189_u8,144_u8,228_u8,14_u8,32_u8,149_u8,162_u8];
_9.1 = ['\u{64f38}','\u{cc4b6}','\u{46ca4}','\u{3a3fa}','\u{105c53}','\u{1d5b1}','\u{e6b87}'];
_4 = ['\u{5a37c}','\u{309c5}'];
_4 = ['\u{1f288}','\u{db9b4}'];
_10 = _2.fld2 << _3;
_2.fld0 = [233_u8,215_u8,145_u8,152_u8,245_u8,252_u8,9_u8];
_4 = _6;
_5 = [2557577657_u32,1536986053_u32,1667157386_u32,1637250810_u32];
_8 = [3566385934_u32,228487554_u32,786561792_u32,350453242_u32];
_2.fld2 = (-75_i8) as isize;
_9 = (_4, _2.fld1.1, _4);
RET = [434348356_u32,1751707192_u32,2129275560_u32,1560171314_u32];
_1 = [2336258697_u32,3371446907_u32,158336719_u32,376221026_u32];
_5 = [3718670591_u32,1098517083_u32,288767207_u32,3863592583_u32];
_9.1 = _2.fld1.1;
_9.0 = _2.fld1.2;
_6 = ['\u{e2b8}','\u{101b0e}'];
_11.0 = _2.fld2 << _10;
_2.fld1.2 = ['\u{d406a}','\u{a1395}'];
_11.3 = 176_u8 >> _7;
_1 = [794825781_u32,2873978277_u32,3299857178_u32,4039010058_u32];
Goto(bb1)
}
bb1 = {
_8 = _1;
Goto(bb2)
}
bb2 = {
_7 = _3;
Goto(bb3)
}
bb3 = {
RET = [276172216_u32,1291082937_u32,2850548029_u32,2804420875_u32];
_4 = ['\u{726d2}','\u{45075}'];
_13 = [_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3];
Goto(bb4)
}
bb4 = {
_2.fld1 = _9;
_5 = [789043092_u32,1771002069_u32,1710879923_u32,1972395617_u32];
RET = [2738484172_u32,1982254481_u32,1796487882_u32,986002860_u32];
_14 = _7 as f64;
_11.4 = [3849032532348659475273209486615035035_u128,21434113566978626577221919485270287148_u128,146054155137668600731050949197183180581_u128,282534518510523547564537742822758152268_u128,131049329367533413774199300789842228939_u128,99223568099099915326462148697296123996_u128,81708897518453129690951260670217585969_u128];
_11.2 = _11.0 & _11.0;
_9.1 = ['\u{fc85a}','\u{e1dbe}','\u{615fe}','\u{1798f}','\u{75777}','\u{5c857}','\u{bb1e8}'];
_9.0 = _6;
_13 = [_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3];
_11.2 = _11.0 >> _11.0;
_3 = 30772_u16 as i64;
_13 = [_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3];
_7 = _3 | _3;
RET = [4034609628_u32,3417297611_u32,4150324962_u32,674150594_u32];
_12.0 = ['\u{ba505}','\u{d44e9}'];
_9 = _2.fld1;
Goto(bb5)
}
bb5 = {
_6 = _9.2;
_11.1 = core::ptr::addr_of_mut!(_11.5.0);
_4 = _12.0;
_2.fld1.0 = ['\u{f0452}','\u{92547}'];
_10 = !_11.2;
_5 = _8;
_2.fld1.0 = ['\u{58f78}','\u{7451f}'];
Goto(bb6)
}
bb6 = {
_2.fld1.2 = ['\u{aa136}','\u{19d90}'];
_9.1 = ['\u{2900b}','\u{fdeca}','\u{41696}','\u{60423}','\u{4979}','\u{d755c}','\u{28b20}'];
_2.fld1.1 = ['\u{658cf}','\u{a7a16}','\u{bd4cd}','\u{f421a}','\u{61667}','\u{10f453}','\u{5d41c}'];
_2.fld1.2 = _12.0;
_12 = (_9.0,);
_6 = _9.0;
_9.2 = _12.0;
_4 = ['\u{8832b}','\u{29732}'];
_11.2 = _11.0 >> _7;
_10 = _11.2;
_18 = Adt50::Variant2 { fld0: 130876583409677749345217094473074618242_u128,fld1: _11.3,fld2: _2.fld0,fld3: (-50_i8),fld4: _3 };
_22 = ['\u{cf407}','\u{71014}'];
_6 = _9.0;
_21 = !false;
_7 = (-140511614845190660730825708345402502370_i128) as i64;
_18 = Adt50::Variant2 { fld0: 106922920770268190294340881969733069276_u128,fld1: _11.3,fld2: _2.fld0,fld3: 92_i8,fld4: _3 };
Goto(bb7)
}
bb7 = {
_12.0 = _2.fld1.0;
_12.0 = _9.0;
RET = [3893525144_u32,396847305_u32,2580191042_u32,3316937627_u32];
_16.fld0 = [659442597_u32,141056025_u32,438887040_u32,2777441796_u32];
place!(Field::<u128>(Variant(_18, 2), 0)) = !2848737189715731778983652571058324842_u128;
place!(Field::<u8>(Variant(_18, 2), 1)) = _11.3;
_11.4 = [Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0)];
_11.3 = Field::<u8>(Variant(_18, 2), 1);
_7 = Field::<i64>(Variant(_18, 2), 4) & _3;
RET = [3365819424_u32,2984496433_u32,1606148830_u32,3737614536_u32];
place!(Field::<[u8; 7]>(Variant(_18, 2), 2)) = [Field::<u8>(Variant(_18, 2), 1),_11.3,Field::<u8>(Variant(_18, 2), 1),_11.3,Field::<u8>(Variant(_18, 2), 1),_11.3,_11.3];
_18 = Adt50::Variant2 { fld0: 229420825434720963277583255556668283548_u128,fld1: _11.3,fld2: _2.fld0,fld3: (-35_i8),fld4: _7 };
_13 = _2.fld0;
place!(Field::<[u8; 7]>(Variant(_18, 2), 2)) = _13;
_11.3 = Field::<u8>(Variant(_18, 2), 1);
place!(Field::<u128>(Variant(_18, 2), 0)) = 309818331142343982804999434538827382550_u128 << Field::<u8>(Variant(_18, 2), 1);
Goto(bb8)
}
bb8 = {
_11.4 = [Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0)];
RET = [2650290856_u32,1408527333_u32,907638971_u32,2193829_u32];
_16.fld0 = [2388570177_u32,1442549240_u32,2887928056_u32,1724873863_u32];
_11.1 = core::ptr::addr_of_mut!(_11.5.0);
_5 = [3834443110_u32,2281345937_u32,2724573001_u32,1668838032_u32];
_24 = -_14;
_2 = Adt45 { fld0: _13,fld1: _9,fld2: _10 };
_16 = Adt44 { fld0: _8 };
Call(place!(Field::<i8>(Variant(_18, 2), 3)) = fn8(_9.1, _2.fld1, Move(_16), _10, _2.fld1, _11.2, _7, _2, _2, _13, Field::<u128>(Variant(_18, 2), 0), _2, _2.fld1, _13), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<[u8; 7]>(Variant(_18, 2), 2)) = [_11.3,_11.3,_11.3,_11.3,_11.3,Field::<u8>(Variant(_18, 2), 1),Field::<u8>(Variant(_18, 2), 1)];
_22 = ['\u{5ec8d}','\u{5493a}'];
place!(Field::<u128>(Variant(_18, 2), 0)) = Field::<i8>(Variant(_18, 2), 3) as u128;
Goto(bb10)
}
bb10 = {
_16 = Adt44 { fld0: _1 };
_9 = _2.fld1;
_17 = Adt52::Variant1 { fld0: 56898_u16 };
place!(Field::<u128>(Variant(_18, 2), 0)) = 131348458473838598758223203447716739814_u128 & 156415079473229349300251137183978280674_u128;
place!(Field::<u16>(Variant(_17, 1), 0)) = 3686_u16 | 56306_u16;
place!(Field::<u16>(Variant(_17, 1), 0)) = 1243711669_u32 as u16;
_14 = -_24;
Goto(bb11)
}
bb11 = {
place!(Field::<u8>(Variant(_18, 2), 1)) = _11.3 | _11.3;
_26.fld1.1 = _2.fld1.1;
_4 = _22;
Goto(bb12)
}
bb12 = {
SetDiscriminant(_17, 1);
_26.fld0 = [_11.3,Field::<u8>(Variant(_18, 2), 1),Field::<u8>(Variant(_18, 2), 1),Field::<u8>(Variant(_18, 2), 1),Field::<u8>(Variant(_18, 2), 1),Field::<u8>(Variant(_18, 2), 1),_11.3];
place!(Field::<u16>(Variant(_17, 1), 0)) = !60579_u16;
_26.fld2 = _11.2;
_12 = (_4,);
_23 = _21;
_9.2 = ['\u{86b50}','\u{69e82}'];
_27 = 803794408_i32 as isize;
_26.fld1.2 = ['\u{bc382}','\u{8561b}'];
_28 = (-140548487508078661610791090900924454879_i128) * (-138082517187270942663646187601845391171_i128);
_16.fld0 = [2499006225_u32,1483052627_u32,3563045446_u32,2357083130_u32];
place!(Field::<i8>(Variant(_18, 2), 3)) = !124_i8;
_26.fld1.2 = ['\u{9b6f9}','\u{d9153}'];
_4 = _6;
Goto(bb13)
}
bb13 = {
_26 = Adt45 { fld0: _2.fld0,fld1: _9,fld2: _10 };
_21 = _23;
_7 = -Field::<i64>(Variant(_18, 2), 4);
_1 = [4124122474_u32,2388307071_u32,2759408939_u32,2510567235_u32];
_11.4 = [Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0),Field::<u128>(Variant(_18, 2), 0)];
_24 = -_14;
_26.fld1.1 = _9.1;
Call(place!(Field::<u8>(Variant(_18, 2), 1)) = core::intrinsics::transmute(_11.3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_25 = [3802089589_u32,3227696231_u32,694791693_u32,1757451615_u32];
_11.5.0 = core::ptr::addr_of!(_28);
_11.3 = Field::<u8>(Variant(_18, 2), 1);
_2.fld1 = (_26.fld1.0, _9.1, _26.fld1.0);
_11.0 = Field::<u16>(Variant(_17, 1), 0) as isize;
_29 = '\u{7902e}';
_12 = (_26.fld1.0,);
_7 = _3 - Field::<i64>(Variant(_18, 2), 4);
_9.1 = [_29,_29,_29,_29,_29,_29,_29];
_6 = [_29,_29];
_7 = 2150405254_u32 as i64;
_11.2 = _26.fld2;
_33.1 = _7 * _3;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 8_usize, Move(_8), 23_usize, Move(_23), 9_usize, Move(_9), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 3_usize, Move(_3), 25_usize, Move(_25), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 7_usize, Move(_7), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [char; 7],mut _2: ([char; 2], [char; 7], [char; 2]),mut _3: Adt44,mut _4: isize,mut _5: ([char; 2], [char; 7], [char; 2]),mut _6: isize,mut _7: i64,mut _8: Adt45,mut _9: Adt45,mut _10: [u8; 7],mut _11: u128,mut _12: Adt45,mut _13: ([char; 2], [char; 7], [char; 2]),mut _14: [u8; 7]) -> i8 {
mir! {
type RET = i8;
let _15: isize;
let _16: [u128; 7];
let _17: *mut [u128; 7];
let _18: i16;
let _19: char;
let _20: u8;
let _21: u32;
let _22: usize;
let _23: f32;
let _24: ([char; 2], [char; 7], [char; 2]);
let _25: Adt51;
let _26: [i128; 4];
let _27: u64;
let _28: usize;
let _29: [i128; 4];
let _30: *mut *const i128;
let _31: Adt54;
let _32: *mut u16;
let _33: Adt48;
let _34: [char; 2];
let _35: u16;
let _36: ();
let _37: ();
{
RET = (-124_i8);
_13.2 = ['\u{96553}','\u{4e2fe}'];
_2.2 = _5.0;
_14 = _12.fld0;
_5 = (_9.fld1.2, _13.1, _8.fld1.0);
_13.2 = ['\u{709c}','\u{c9a17}'];
_12.fld0 = _14;
_8.fld1.1 = _9.fld1.1;
_1 = ['\u{6aa5a}','\u{19276}','\u{32020}','\u{b9a3e}','\u{8fa10}','\u{e63d6}','\u{e6713}'];
_9 = Adt45 { fld0: _12.fld0,fld1: _2,fld2: _12.fld2 };
_4 = -_6;
_15 = 8254200803768780810_usize as isize;
_8.fld1 = (_13.2, _9.fld1.1, _13.0);
_2.0 = ['\u{2d3f6}','\u{afc99}'];
_9.fld2 = -_8.fld2;
_9.fld0 = _12.fld0;
_12.fld1.0 = ['\u{aedee}','\u{17f99}'];
_8.fld0 = _10;
_2.0 = ['\u{ee5d2}','\u{65b25}'];
_8.fld1.0 = ['\u{6a981}','\u{10591d}'];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211332 => bb5,
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
_13.0 = ['\u{71331}','\u{7e35}'];
_8.fld1 = (_2.2, _9.fld1.1, _2.0);
_16 = [_11,_11,_11,_11,_11,_11,_11];
_10 = _8.fld0;
Call(_18 = fn9(_8.fld0, _9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.2 = ['\u{4a851}','\u{98d39}'];
_8.fld0 = [231_u8,218_u8,169_u8,63_u8,88_u8,106_u8,167_u8];
_8.fld1.2 = ['\u{2f989}','\u{10f358}'];
_12 = Adt45 { fld0: _14,fld1: _8.fld1,fld2: _6 };
_11 = false as u128;
_9.fld1.0 = _12.fld1.2;
_5.2 = ['\u{970d9}','\u{10680f}'];
_6 = _8.fld2;
_2.1 = ['\u{b074d}','\u{41d91}','\u{6e3c2}','\u{189ca}','\u{8dc4e}','\u{2ec80}','\u{c8e}'];
_12.fld1.2 = ['\u{e0898}','\u{106478}'];
_5.1 = ['\u{483fe}','\u{96620}','\u{643c1}','\u{35b1c}','\u{7063}','\u{b5f2}','\u{66ea9}'];
_4 = _6 ^ _8.fld2;
_19 = '\u{52799}';
_13.0 = [_19,_19];
Goto(bb7)
}
bb7 = {
_12 = Adt45 { fld0: _10,fld1: _5,fld2: _8.fld2 };
RET = 10_u8 as i8;
_16 = [_11,_11,_11,_11,_11,_11,_11];
_15 = !_4;
_9.fld1 = _8.fld1;
_7 = 15496598860642889536_usize as i64;
_14 = [99_u8,196_u8,246_u8,106_u8,230_u8,27_u8,151_u8];
_1 = [_19,_19,_19,_19,_19,_19,_19];
_24.1 = [_19,_19,_19,_19,_19,_19,_19];
_5 = (_12.fld1.2, _12.fld1.1, _8.fld1.0);
_8.fld1.0 = _5.0;
_12.fld2 = _9.fld2;
_10 = [56_u8,52_u8,16_u8,62_u8,249_u8,243_u8,89_u8];
_14 = _10;
_24.0 = _13.2;
_25 = Adt51::Variant1 { fld0: RET };
_13.1 = [_19,_19,_19,_19,_19,_19,_19];
_22 = 54680_u16 as usize;
SetDiscriminant(_25, 2);
_8.fld1.0 = [_19,_19];
_2.0 = _9.fld1.0;
_4 = -_12.fld2;
RET = 27_i8 << _12.fld2;
_9.fld1.1 = [_19,_19,_19,_19,_19,_19,_19];
_8.fld1 = (_9.fld1.2, _2.1, _13.2);
_26 = [124374886861492904109391309009082227520_i128,(-161454361400103362757249891230763695999_i128),50236361830904080665978308269672190595_i128,(-63113277982990919042527643461244120959_i128)];
place!(Field::<char>(Variant(_25, 2), 1)) = _19;
Goto(bb8)
}
bb8 = {
_12 = _8;
_2.0 = _9.fld1.0;
_8 = _12;
_9.fld2 = !_12.fld2;
Goto(bb9)
}
bb9 = {
_14 = [157_u8,155_u8,234_u8,178_u8,24_u8,167_u8,187_u8];
_13 = (_8.fld1.0, _5.1, _8.fld1.0);
_8.fld1 = (_5.0, _2.1, _24.0);
Goto(bb10)
}
bb10 = {
_31.fld6 = _11;
_31.fld6 = _22 as u128;
_23 = 154285003070685943683770431976222147247_i128 as f32;
_18 = 2841_i16 & 23577_i16;
_31.fld3 = _7 as f64;
_20 = !170_u8;
_18 = (-13906_i16);
_31.fld2 = core::ptr::addr_of!(_31.fld6);
_31.fld5.fld3 = RET;
place!(Field::<[char; 2]>(Variant(_25, 2), 0)) = [_19,_19];
_23 = _31.fld6 as f32;
_14 = _9.fld0;
_9 = _12;
_29 = [(-77765904115134454833576353329143807273_i128),97105016452483786613059980058071477886_i128,38111903727839841884318039370542781897_i128,(-165099619512681863191963181651629221438_i128)];
_9.fld2 = -_12.fld2;
_15 = _9.fld2 | _12.fld2;
_31.fld5.fld4 = core::ptr::addr_of_mut!(_18);
_31.fld5.fld1 = (_2.2,);
match _18 {
0 => bb3,
1 => bb5,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607431768197550 => bb15,
_ => bb14
}
}
bb11 = {
_14 = [157_u8,155_u8,234_u8,178_u8,24_u8,167_u8,187_u8];
_13 = (_8.fld1.0, _5.1, _8.fld1.0);
_8.fld1 = (_5.0, _2.1, _24.0);
Goto(bb10)
}
bb12 = {
Return()
}
bb13 = {
_12 = Adt45 { fld0: _10,fld1: _5,fld2: _8.fld2 };
RET = 10_u8 as i8;
_16 = [_11,_11,_11,_11,_11,_11,_11];
_15 = !_4;
_9.fld1 = _8.fld1;
_7 = 15496598860642889536_usize as i64;
_14 = [99_u8,196_u8,246_u8,106_u8,230_u8,27_u8,151_u8];
_1 = [_19,_19,_19,_19,_19,_19,_19];
_24.1 = [_19,_19,_19,_19,_19,_19,_19];
_5 = (_12.fld1.2, _12.fld1.1, _8.fld1.0);
_8.fld1.0 = _5.0;
_12.fld2 = _9.fld2;
_10 = [56_u8,52_u8,16_u8,62_u8,249_u8,243_u8,89_u8];
_14 = _10;
_24.0 = _13.2;
_25 = Adt51::Variant1 { fld0: RET };
_13.1 = [_19,_19,_19,_19,_19,_19,_19];
_22 = 54680_u16 as usize;
SetDiscriminant(_25, 2);
_8.fld1.0 = [_19,_19];
_2.0 = _9.fld1.0;
_4 = -_12.fld2;
RET = 27_i8 << _12.fld2;
_9.fld1.1 = [_19,_19,_19,_19,_19,_19,_19];
_8.fld1 = (_9.fld1.2, _2.1, _13.2);
_26 = [124374886861492904109391309009082227520_i128,(-161454361400103362757249891230763695999_i128),50236361830904080665978308269672190595_i128,(-63113277982990919042527643461244120959_i128)];
place!(Field::<char>(Variant(_25, 2), 1)) = _19;
Goto(bb8)
}
bb14 = {
Return()
}
bb15 = {
_20 = 159_u8;
_8.fld1.1 = _5.1;
_2 = (_13.2, _8.fld1.1, _5.2);
_31.fld4 = core::ptr::addr_of_mut!(_31.fld3);
_21 = 1890885827_u32;
_12.fld1 = (_5.0, _2.1, _31.fld5.fld1.0);
_31.fld5.fld2 = [_7,_7,_7,_7,_7];
_31.fld0 = _22 << _22;
_20 = 117_u8;
_31.fld5.fld1 = (_5.0,);
_12.fld1.2 = Field::<[char; 2]>(Variant(_25, 2), 0);
_12.fld1.1 = _5.1;
_12.fld1.2 = _2.2;
_10 = _12.fld0;
_31.fld5.fld1.0 = [_19,Field::<char>(Variant(_25, 2), 1)];
_31.fld5.fld0 = _31.fld0 as i64;
_4 = -_8.fld2;
_12.fld1.0 = _12.fld1.2;
_13.2 = [Field::<char>(Variant(_25, 2), 1),Field::<char>(Variant(_25, 2), 1)];
_21 = !3287028270_u32;
_2.1 = _13.1;
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(8_usize, 16_usize, Move(_16), 11_usize, Move(_11), 2_usize, Move(_2), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(8_usize, 5_usize, Move(_5), 14_usize, Move(_14), 10_usize, Move(_10), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(8_usize, 19_usize, Move(_19), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [u8; 7],mut _2: Adt45) -> i16 {
mir! {
type RET = i16;
let _3: (u32, i64);
let _4: char;
let _5: [char; 7];
let _6: isize;
let _7: u32;
let _8: &'static u64;
let _9: [i64; 5];
let _10: (u32, i64);
let _11: Adt55;
let _12: [char; 2];
let _13: isize;
let _14: Adt44;
let _15: isize;
let _16: [char; 2];
let _17: u16;
let _18: isize;
let _19: [char; 7];
let _20: *mut [u128; 7];
let _21: [char; 7];
let _22: f32;
let _23: [char; 7];
let _24: [char; 7];
let _25: char;
let _26: bool;
let _27: *mut i16;
let _28: i128;
let _29: f32;
let _30: ([char; 2], [char; 7], [char; 2]);
let _31: u16;
let _32: u32;
let _33: Adt43;
let _34: u8;
let _35: Adt44;
let _36: Adt48;
let _37: u16;
let _38: f64;
let _39: usize;
let _40: *const (u32, *const u128, isize);
let _41: Adt54;
let _42: char;
let _43: f32;
let _44: f64;
let _45: Adt44;
let _46: bool;
let _47: ();
let _48: ();
{
_2.fld1.2 = _2.fld1.0;
_1 = [149_u8,230_u8,16_u8,154_u8,245_u8,56_u8,223_u8];
RET = !(-18387_i16);
RET = 21437_i16 * (-30126_i16);
_1 = [79_u8,238_u8,242_u8,138_u8,222_u8,171_u8,104_u8];
RET = !(-6183_i16);
_2.fld2 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = (-11660_i16) - (-15087_i16);
_3.1 = 5834078684929987801_i64 | 161639630379739194_i64;
RET = (-19066_i16) * (-26397_i16);
_3.1 = 3484141051025480971_i64 & (-5618592476791241707_i64);
_5 = ['\u{e7626}','\u{9b339}','\u{ce79f}','\u{84ce2}','\u{85ada}','\u{89de7}','\u{c3e20}'];
_3.0 = 1909023148_u32 >> RET;
Goto(bb1)
}
bb1 = {
_4 = '\u{f0b32}';
_7 = 5_usize as u32;
RET = (-316_i16);
_2.fld2 = -9223372036854775807_isize;
_7 = 5_usize as u32;
_4 = '\u{4e4e2}';
_2.fld1.2 = _2.fld1.0;
RET = (-16437_i16) | 3748_i16;
_9 = [_3.1,_3.1,_3.1,_3.1,_3.1];
RET = !11675_i16;
_7 = 6214065332410353787_u64 as u32;
_3 = (_7, (-6886921322269592588_i64));
_10.0 = _7;
_11.fld4 = (_2.fld1.0,);
_4 = '\u{8c7de}';
_11.fld0 = [_4,_4];
Goto(bb2)
}
bb2 = {
_13 = _2.fld2 | _2.fld2;
_3 = (_10.0, (-9146527914686749277_i64));
_10.1 = _3.1;
_11.fld4 = (_2.fld1.0,);
_2.fld2 = _13;
RET = -22072_i16;
_2.fld1 = (_11.fld4.0, _5, _11.fld4.0);
RET = 7770_i16;
_9 = [_10.1,_3.1,_3.1,_3.1,_10.1];
_15 = !_13;
_3.1 = _10.1 | _10.1;
_2.fld1 = (_11.fld4.0, _5, _11.fld4.0);
_2.fld1.1 = _5;
_10 = (_3.0, _3.1);
_1 = _2.fld0;
_7 = _10.0 | _10.0;
_11.fld0 = _11.fld4.0;
_4 = '\u{63e18}';
_2.fld0 = [138_u8,143_u8,56_u8,90_u8,125_u8,155_u8,153_u8];
_6 = _13;
_11.fld0 = _11.fld4.0;
_13 = -_6;
_14.fld0 = [_10.0,_7,_3.0,_7];
_9 = [_3.1,_3.1,_10.1,_3.1,_3.1];
match RET {
0 => bb1,
7770 => bb4,
_ => bb3
}
}
bb3 = {
_4 = '\u{f0b32}';
_7 = 5_usize as u32;
RET = (-316_i16);
_2.fld2 = -9223372036854775807_isize;
_7 = 5_usize as u32;
_4 = '\u{4e4e2}';
_2.fld1.2 = _2.fld1.0;
RET = (-16437_i16) | 3748_i16;
_9 = [_3.1,_3.1,_3.1,_3.1,_3.1];
RET = !11675_i16;
_7 = 6214065332410353787_u64 as u32;
_3 = (_7, (-6886921322269592588_i64));
_10.0 = _7;
_11.fld4 = (_2.fld1.0,);
_4 = '\u{8c7de}';
_11.fld0 = [_4,_4];
Goto(bb2)
}
bb4 = {
_3.1 = _10.1;
_2.fld0 = [184_u8,101_u8,230_u8,225_u8,217_u8,119_u8,58_u8];
_10.0 = _3.0;
_11.fld4 = (_2.fld1.0,);
_3.1 = _10.1 << _7;
_2.fld0 = [48_u8,123_u8,0_u8,200_u8,254_u8,178_u8,26_u8];
RET = 14344180617311998389_u64 as i16;
_2.fld2 = true as isize;
_3 = (_7, _10.1);
_17 = !29546_u16;
_12 = _2.fld1.0;
_10.1 = _17 as i64;
_14.fld0 = [_10.0,_3.0,_7,_3.0];
_3.1 = _10.1;
Goto(bb5)
}
bb5 = {
_3.1 = 99_i8 as i64;
_18 = false as isize;
_3.1 = RET as i64;
RET = !(-16700_i16);
_2.fld2 = _10.1 as isize;
_11.fld1 = [238156128335300007791058161724492581326_u128,201065895912449487994731835161353848425_u128,193508686726828755291505757827230386539_u128,87168069674736185841006114233361355300_u128,290317271647768267349475159870517561693_u128,200589953651433508645108196759198124887_u128,3808534501931505999073343585401616237_u128];
_17 = _4 as u16;
_11.fld0 = [_4,_4];
_1 = [122_u8,46_u8,10_u8,255_u8,240_u8,21_u8,186_u8];
_5 = [_4,_4,_4,_4,_4,_4,_4];
_14.fld0 = [_3.0,_3.0,_7,_3.0];
_18 = false as isize;
Call(_13 = fn10(_2.fld0, _2.fld1.1, _2.fld1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = (_7, _3.1);
_11.fld1 = [234945054057834492809142644941568214530_u128,177588300398848813276908406867822296942_u128,175020732195932137924637007955145898133_u128,63757279141775295669006159623031581720_u128,254565949884800630357825573516010307202_u128,212968600121846633362021278440852564247_u128,168607250368984904486801409231047756044_u128];
_10.0 = _7 ^ _7;
_3.0 = _10.0 >> _7;
_1 = [180_u8,119_u8,241_u8,222_u8,20_u8,162_u8,28_u8];
_11.fld0 = [_4,_4];
_11.fld4.0 = _2.fld1.0;
_6 = 138_u8 as isize;
Goto(bb7)
}
bb7 = {
_14.fld0 = [_3.0,_3.0,_3.0,_7];
_13 = _15;
_11.fld0 = _2.fld1.0;
_2.fld1 = (_11.fld0, _5, _12);
_17 = false as u16;
_2.fld0 = _1;
_3.0 = !_10.0;
_10 = (_3.0, _3.1);
_19 = [_4,_4,_4,_4,_4,_4,_4];
_6 = _13 & _13;
_10 = _3;
_13 = _6;
_2.fld2 = !_13;
_11.fld4 = (_12,);
_14.fld0 = [_3.0,_10.0,_3.0,_3.0];
_3.1 = _10.1;
_11.fld4 = (_2.fld1.0,);
_6 = _2.fld2;
_10.1 = 0_usize as i64;
_21 = [_4,_4,_4,_4,_4,_4,_4];
_11.fld1 = [121394498011245055290111260505616087962_u128,113485495445232125869665347494350124492_u128,146466258741533392905716661661629917613_u128,201312948525228293789633350453846972232_u128,134333150196929913971367411114684653220_u128,145987246475842094748663326172947374279_u128,328877890268713662648677289372539807296_u128];
_13 = _2.fld2 - _2.fld2;
_3.0 = !_10.0;
_9 = [_3.1,_3.1,_3.1,_3.1,_3.1];
_11.fld4 = (_12,);
_16 = _2.fld1.0;
Goto(bb8)
}
bb8 = {
RET = _3.0 as i16;
_2.fld1.0 = [_4,_4];
_20 = core::ptr::addr_of_mut!(_11.fld1);
_3.1 = !_10.1;
_16 = [_4,_4];
_7 = !_10.0;
_16 = [_4,_4];
RET = 2624_i16 + (-4206_i16);
_20 = core::ptr::addr_of_mut!((*_20));
_2.fld2 = !_6;
RET = !1365_i16;
_2.fld2 = _15;
_16 = [_4,_4];
_22 = 818758304_i32 as f32;
_22 = 112_i8 as f32;
_13 = _6;
RET = 32607_i16 << _18;
_2.fld0 = [178_u8,39_u8,130_u8,20_u8,27_u8,236_u8,145_u8];
RET = -(-13519_i16);
_6 = _3.1 as isize;
_11.fld1 = [117990705437233044148166667628950187943_u128,249000895562407533744852590204017365195_u128,74946830223490621555726664542298830373_u128,178994508146958455617491646418229276384_u128,306561204324753673193230443902012580996_u128,154220038421755382021367481641499334548_u128,207706288673806701484255874067311339219_u128];
Call(_18 = fn19(_9, _11.fld4.0, _1, _3.0, _20, _14.fld0, _2, (*_20), Move(_14), _15, _11.fld0, _2.fld1.1, _11.fld4.0, _11.fld1, _12, (*_20)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = _2.fld0;
_24 = _2.fld1.1;
_2.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_23 = [_4,_4,_4,_4,_4,_4,_4];
Call(RET = core::intrinsics::bswap((-12962_i16)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21 = [_4,_4,_4,_4,_4,_4,_4];
_19 = [_4,_4,_4,_4,_4,_4,_4];
_2.fld1 = (_12, _23, _12);
_13 = _15;
_3 = (_7, _10.1);
_21 = [_4,_4,_4,_4,_4,_4,_4];
_11.fld4.0 = _2.fld1.0;
_11.fld1 = [146655159813841275376153635740021924822_u128,119554390156969477868559524374235849195_u128,30871224470366967890648513754476229629_u128,5429053911558899591142143765591519500_u128,68198011795109057242258899501264928047_u128,100426295165458926792484314424482768308_u128,257581784963585867899898948398207795785_u128];
_16 = _12;
(*_20) = [252187144084467101475961339584498853831_u128,306176128442239789234436346277852307342_u128,131557569370702356524045499131528642421_u128,19370437289935690982302725122106259967_u128,81388112662601221751950745879513312213_u128,327152850734257945555481381616164639715_u128,162509901826780688849443338264140008376_u128];
_27 = core::ptr::addr_of_mut!(RET);
_2.fld2 = 220_u8 as isize;
_13 = (-2015590290_i32) as isize;
_14.fld0 = [_10.0,_7,_7,_10.0];
(*_20) = [226109126016789266724051206069385306743_u128,325034264837225043207299544490971697105_u128,333113083224072641656632219345513845033_u128,135957628598699463471793721448016284712_u128,44967843401005186573455230756405912151_u128,204363350017134641343877803777331168482_u128,12851133186882918898725735172860212551_u128];
_6 = 119330863411452746671260300139852956372_u128 as isize;
_3.1 = _10.1;
_24 = [_4,_4,_4,_4,_4,_4,_4];
_2.fld1 = (_12, _5, _11.fld4.0);
(*_20) = [39213892738459127391752742630876534714_u128,128126494528659515663096737902321157471_u128,93510802920839738958710475199480177889_u128,141921642767624936054121240286962484111_u128,165231227513700091654935169528806715733_u128,170516763910741381176590951249148801208_u128,240980577756803926700945490720639261014_u128];
_1 = [22_u8,220_u8,106_u8,25_u8,174_u8,164_u8,207_u8];
Goto(bb11)
}
bb11 = {
RET = _3.0 as i16;
_6 = false as isize;
_1 = [201_u8,74_u8,123_u8,128_u8,151_u8,245_u8,113_u8];
(*_20) = [125360634669296651931297747280170968691_u128,53184511575532976722091821205435695921_u128,16895824285196741290281493300729660377_u128,127179579733172145247070121589220728194_u128,167707744956227233056165104697468643588_u128,189953648778145361217596742553772585037_u128,264530147361047463209002107283020998745_u128];
_20 = core::ptr::addr_of_mut!(_11.fld1);
RET = _4 as i16;
_10 = (_7, _3.1);
_11.fld4 = (_2.fld1.2,);
_13 = _18;
(*_20) = [7001381143725264116199885527809399147_u128,145345332736541765376992519311302148669_u128,268175612921306591618607446151030564774_u128,222824953265375025638282010730836837130_u128,52377947768627951576091414612779382557_u128,328308654870422181993659900358241992357_u128,256968171615186862447165851569900679799_u128];
_4 = '\u{38e8f}';
_27 = core::ptr::addr_of_mut!(RET);
_4 = '\u{91b8a}';
_15 = _13;
Goto(bb12)
}
bb12 = {
_29 = _22 - _22;
_4 = '\u{3876a}';
_4 = '\u{ba857}';
_11.fld0 = [_4,_4];
(*_27) = -30482_i16;
(*_20) = [234110351268632756742497311094742522071_u128,125435114351731527836568329507669959265_u128,287445536964613969285543595640416001653_u128,16839289013492704452764142220654235734_u128,237616405531641852340871523751128257852_u128,189335091731841233944040196956268915665_u128,65717085883586406141438041556560650011_u128];
_2.fld1.0 = _11.fld4.0;
_24 = [_4,_4,_4,_4,_4,_4,_4];
_24 = [_4,_4,_4,_4,_4,_4,_4];
_35.fld0 = [_7,_7,_3.0,_7];
_11.fld1 = [218611686058717603407471146416393726941_u128,68592802971313759183432705333647323160_u128,72351758409933533673368359000348961234_u128,277867108125999333909637755936231041990_u128,159312656138956186124627975921648774164_u128,301702555290330555787257834679757682538_u128,218587329025539659814584660377758460664_u128];
_16 = _12;
_4 = '\u{f2468}';
_26 = !false;
RET = (-16695_i16);
_10.1 = _3.1 ^ _3.1;
_22 = _29 + _29;
_38 = _17 as f64;
_35 = Adt44 { fld0: _14.fld0 };
_31 = !_17;
_2.fld1 = (_11.fld4.0, _23, _11.fld4.0);
_30.2 = [_4,_4];
Goto(bb13)
}
bb13 = {
_41.fld2 = core::ptr::addr_of!(_41.fld6);
_41.fld4 = core::ptr::addr_of_mut!(_41.fld3);
_30.1 = [_4,_4,_4,_4,_4,_4,_4];
_27 = core::ptr::addr_of_mut!((*_27));
_42 = _4;
_2.fld1 = (_11.fld4.0, _23, _12);
_20 = core::ptr::addr_of_mut!(_11.fld1);
_41.fld3 = -_38;
(*_27) = (-64_i8) as i16;
_41.fld3 = -_38;
_2.fld1.2 = _11.fld0;
_43 = -_22;
_5 = _23;
Goto(bb14)
}
bb14 = {
_31 = _17;
_11.fld1 = [119272540783115070950262604131620349291_u128,175550916625887542376634957154792772677_u128,255132851521901776711408530409643484274_u128,260534830731127803237250191876130339931_u128,95227028736664289742908614860558315171_u128,278735715085787534582803498091533011322_u128,228392771000296526178000711329377576334_u128];
_11.fld4.0 = [_42,_4];
_34 = 125_u8 << _13;
_43 = 17417717430361927070_u64 as f32;
(*_20) = [239258755613407757139025956616707350334_u128,210080235677358688763166375627561372162_u128,337766304177656965563757931153919275129_u128,242970107365857966316575610269928908118_u128,247809281320683618150218321885031265599_u128,5997241062291956658835258081976355124_u128,330046475114221898830112150730237549604_u128];
_30 = (_11.fld0, _21, _11.fld4.0);
_21 = [_4,_42,_42,_4,_42,_42,_42];
_14.fld0 = _35.fld0;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(9_usize, 5_usize, Move(_5), 13_usize, Move(_13), 31_usize, Move(_31), 34_usize, Move(_34)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(9_usize, 3_usize, Move(_3), 9_usize, Move(_9), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(9_usize, 17_usize, Move(_17), 16_usize, Move(_16), 24_usize, Move(_24), 48_usize, _48), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [u8; 7],mut _2: [char; 7],mut _3: ([char; 2], [char; 7], [char; 2])) -> isize {
mir! {
type RET = isize;
let _4: Adt50;
let _5: [char; 2];
let _6: Adt42;
let _7: u64;
let _8: u128;
let _9: [char; 7];
let _10: Adt53;
let _11: Adt44;
let _12: bool;
let _13: Adt47;
let _14: f32;
let _15: i8;
let _16: u8;
let _17: f32;
let _18: (u32, *const u128, isize);
let _19: f32;
let _20: Adt52;
let _21: Adt44;
let _22: *mut i16;
let _23: [char; 7];
let _24: f64;
let _25: u64;
let _26: i64;
let _27: (isize, i32, *const i32, i128);
let _28: isize;
let _29: u8;
let _30: isize;
let _31: isize;
let _32: f64;
let _33: Adt42;
let _34: ();
let _35: ();
{
RET = (-604268277_i32) as isize;
_4 = Adt50::Variant2 { fld0: 177886094731161229315669993886004969561_u128,fld1: 31_u8,fld2: _1,fld3: 112_i8,fld4: (-1665892952781020343_i64) };
place!(Field::<u128>(Variant(_4, 2), 0)) = 245782648126569598453156920223674598843_u128;
_3.1 = ['\u{8f462}','\u{5d818}','\u{bbc3a}','\u{6bfd8}','\u{bad9b}','\u{782ec}','\u{b341e}'];
_3.2 = ['\u{48a44}','\u{3f925}'];
_3.1 = ['\u{52ca0}','\u{52778}','\u{e3563}','\u{99a5f}','\u{4c154}','\u{b6d98}','\u{9c053}'];
_3.2 = ['\u{23e1}','\u{127ac}'];
match Field::<u128>(Variant(_4, 2), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
245782648126569598453156920223674598843 => bb6,
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
_7 = !5682346289732397685_u64;
place!(Field::<i8>(Variant(_4, 2), 3)) = (-21_i8) * (-51_i8);
_3.2 = ['\u{2969}','\u{64e7d}'];
_2 = _3.1;
RET = 79_isize;
place!(Field::<i64>(Variant(_4, 2), 4)) = (-3989782320762061784_i64) ^ 667348981792177674_i64;
RET = '\u{b8b9d}' as isize;
place!(Field::<i8>(Variant(_4, 2), 3)) = -4_i8;
place!(Field::<u8>(Variant(_4, 2), 1)) = 56_u8 | 146_u8;
_3.2 = _3.0;
place!(Field::<i8>(Variant(_4, 2), 3)) = (-19944_i16) as i8;
_4 = Adt50::Variant2 { fld0: 26954415720301503469779893976120180076_u128,fld1: 193_u8,fld2: _1,fld3: 55_i8,fld4: (-4460159563437021424_i64) };
_3.0 = _3.2;
place!(Field::<u8>(Variant(_4, 2), 1)) = 121_u8;
place!(Field::<i64>(Variant(_4, 2), 4)) = (-7551564712951221318_i64);
RET = 9223372036854775807_isize << _7;
RET = 73_isize * 9223372036854775807_isize;
_3.1 = _2;
_5 = ['\u{105cfc}','\u{8fe03}'];
_3.2 = _5;
_5 = ['\u{13771}','\u{d4b86}'];
place!(Field::<i64>(Variant(_4, 2), 4)) = 8119263139554449916_i64 ^ 870865091605441633_i64;
_7 = 12349160476379538640_u64;
RET = !77_isize;
_7 = 7_usize as u64;
place!(Field::<u128>(Variant(_4, 2), 0)) = !57431361608119132556259051593315490542_u128;
Call(_7 = core::intrinsics::bswap(12323273961825061970_u64), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = ['\u{643c1}','\u{7c61a}'];
_3.2 = _5;
RET = !9223372036854775807_isize;
_5 = ['\u{6639c}','\u{36954}'];
_3.0 = _3.2;
_8 = Field::<u128>(Variant(_4, 2), 0) | Field::<u128>(Variant(_4, 2), 0);
_3.0 = _5;
_8 = true as u128;
_5 = ['\u{5adc6}','\u{6bc52}'];
_3.2 = ['\u{fd751}','\u{f04eb}'];
_5 = ['\u{4162c}','\u{b30d0}'];
_2 = _3.1;
_9 = ['\u{1e721}','\u{2c0c9}','\u{55b3e}','\u{31016}','\u{f30f1}','\u{3c718}','\u{8a71e}'];
place!(Field::<i64>(Variant(_4, 2), 4)) = 5130034526972326378_i64;
RET = !(-9223372036854775808_isize);
place!(Field::<[u8; 7]>(Variant(_4, 2), 2)) = [Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1)];
place!(Field::<[u8; 7]>(Variant(_4, 2), 2)) = [Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1),Field::<u8>(Variant(_4, 2), 1)];
RET = 9223372036854775807_isize;
_7 = 955599662304585043_usize as u64;
_9 = ['\u{dee62}','\u{56e5c}','\u{cc7be}','\u{f7d69}','\u{ea4c0}','\u{3beae}','\u{ee497}'];
_8 = Field::<u128>(Variant(_4, 2), 0) << Field::<u128>(Variant(_4, 2), 0);
RET = 9223372036854775807_isize;
place!(Field::<i8>(Variant(_4, 2), 3)) = 1771551020_i32 as i8;
_13.fld0 = 1189565245756376202_usize as i64;
Call(_5 = fn11(_3.2, Field::<[u8; 7]>(Variant(_4, 2), 2), Move(_4), _2, _9, _2, _3, _8, _3.2, _3.1, _8, _3.1, _2, _3, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13.fld3 = 926146031_i32 as i8;
_15 = _13.fld3 >> _8;
_13.fld2 = [_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0];
RET = !75_isize;
_3 = (_5, _2, _5);
RET = (-73_isize);
_13.fld1 = (_3.0,);
_8 = 28490753080177394394604494155903074920_u128 + 254743048943654503002119267746206761894_u128;
_13.fld1 = (_3.0,);
_14 = _15 as f32;
_3.1 = _9;
_18.2 = 247_u8 as isize;
_18.2 = _13.fld0 as isize;
Goto(bb9)
}
bb9 = {
_14 = 1124097205_i32 as f32;
_15 = 3611937467_u32 as i8;
_18.1 = core::ptr::addr_of!(_8);
_2 = ['\u{47478}','\u{45134}','\u{fede4}','\u{4ba9c}','\u{1077cd}','\u{234fd}','\u{89abb}'];
_16 = 161_u8;
_16 = 35_u8;
_3.1 = ['\u{1b00e}','\u{3bf0c}','\u{6e02e}','\u{2eb6e}','\u{bb39}','\u{b7230}','\u{f0839}'];
_13.fld0 = !(-7422133512586936196_i64);
_12 = !false;
_17 = -_14;
_2 = _9;
_9 = ['\u{3900e}','\u{150dc}','\u{3862a}','\u{35648}','\u{69488}','\u{561c4}','\u{1293}'];
_18.2 = RET ^ RET;
_13.fld0 = 4451587898129757925_i64 & 3238073486337442744_i64;
_1 = [_16,_16,_16,_16,_16,_16,_16];
_21.fld0 = [2281235310_u32,4264522383_u32,1937890876_u32,642460316_u32];
Goto(bb10)
}
bb10 = {
_3.0 = _3.2;
_19 = _17;
_1 = [_16,_16,_16,_16,_16,_16,_16];
RET = _18.2 ^ _18.2;
_11.fld0 = [1061915873_u32,3613612754_u32,1140624445_u32,2129542847_u32];
_8 = '\u{c7be2}' as u128;
_21.fld0 = [179077574_u32,1298359786_u32,3161865721_u32,3726161158_u32];
_18.2 = -RET;
_18.2 = _13.fld0 as isize;
_13.fld0 = '\u{5b344}' as i64;
RET = _18.2;
_18.1 = core::ptr::addr_of!(_8);
_4 = Adt50::Variant2 { fld0: _8,fld1: _16,fld2: _1,fld3: _15,fld4: _13.fld0 };
_11 = Move(_21);
place!(Field::<u128>(Variant(_4, 2), 0)) = _8;
_13.fld1.0 = ['\u{f8931}','\u{55b0e}'];
_24 = RET as f64;
_1 = Field::<[u8; 7]>(Variant(_4, 2), 2);
_13.fld3 = -Field::<i8>(Variant(_4, 2), 3);
_18.2 = RET & RET;
_3.2 = ['\u{eee95}','\u{7f12a}'];
_18.1 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_4, 2), 0)));
_3.1 = _2;
_20 = Adt52::Variant1 { fld0: 4919_u16 };
Call(place!(Field::<u8>(Variant(_4, 2), 1)) = core::intrinsics::bswap(_16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13.fld1 = (_3.2,);
RET = _7 as isize;
place!(Field::<u8>(Variant(_4, 2), 1)) = !_16;
_27.1 = 2054650833_i32 ^ (-886827954_i32);
_12 = !false;
_14 = -_19;
place!(Field::<i64>(Variant(_4, 2), 4)) = !_13.fld0;
_27.0 = _14 as isize;
RET = _18.2;
_26 = Field::<i64>(Variant(_4, 2), 4);
_24 = 13815_i16 as f64;
_19 = _17 * _14;
_13.fld1 = (_3.0,);
match _16 {
0 => bb12,
1 => bb13,
2 => bb14,
35 => bb16,
_ => bb15
}
}
bb12 = {
_3.0 = _3.2;
_19 = _17;
_1 = [_16,_16,_16,_16,_16,_16,_16];
RET = _18.2 ^ _18.2;
_11.fld0 = [1061915873_u32,3613612754_u32,1140624445_u32,2129542847_u32];
_8 = '\u{c7be2}' as u128;
_21.fld0 = [179077574_u32,1298359786_u32,3161865721_u32,3726161158_u32];
_18.2 = -RET;
_18.2 = _13.fld0 as isize;
_13.fld0 = '\u{5b344}' as i64;
RET = _18.2;
_18.1 = core::ptr::addr_of!(_8);
_4 = Adt50::Variant2 { fld0: _8,fld1: _16,fld2: _1,fld3: _15,fld4: _13.fld0 };
_11 = Move(_21);
place!(Field::<u128>(Variant(_4, 2), 0)) = _8;
_13.fld1.0 = ['\u{f8931}','\u{55b0e}'];
_24 = RET as f64;
_1 = Field::<[u8; 7]>(Variant(_4, 2), 2);
_13.fld3 = -Field::<i8>(Variant(_4, 2), 3);
_18.2 = RET & RET;
_3.2 = ['\u{eee95}','\u{7f12a}'];
_18.1 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_4, 2), 0)));
_3.1 = _2;
_20 = Adt52::Variant1 { fld0: 4919_u16 };
Call(place!(Field::<u8>(Variant(_4, 2), 1)) = core::intrinsics::bswap(_16), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_13.fld3 = 926146031_i32 as i8;
_15 = _13.fld3 >> _8;
_13.fld2 = [_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0];
RET = !75_isize;
_3 = (_5, _2, _5);
RET = (-73_isize);
_13.fld1 = (_3.0,);
_8 = 28490753080177394394604494155903074920_u128 + 254743048943654503002119267746206761894_u128;
_13.fld1 = (_3.0,);
_14 = _15 as f32;
_3.1 = _9;
_18.2 = 247_u8 as isize;
_18.2 = _13.fld0 as isize;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
_30 = _18.2 << _27.1;
RET = _18.2 >> _26;
_18.2 = 163119748035279061279613610415186394599_i128 as isize;
place!(Field::<u16>(Variant(_20, 1), 0)) = 13019297726460971464_usize as u16;
_16 = Field::<u8>(Variant(_4, 2), 1);
_13.fld2 = [Field::<i64>(Variant(_4, 2), 4),_26,_26,Field::<i64>(Variant(_4, 2), 4),_13.fld0];
place!(Field::<i64>(Variant(_4, 2), 4)) = !_13.fld0;
_21.fld0 = _11.fld0;
_3.0 = ['\u{a1c8b}','\u{f7d51}'];
RET = -_30;
_7 = !5705414482592412867_u64;
_18.0 = 4140946181_u32 + 1495471215_u32;
_20 = Adt52::Variant1 { fld0: 51707_u16 };
_20 = Adt52::Variant1 { fld0: 34336_u16 };
_27.1 = -1888359077_i32;
_18.2 = _30 >> RET;
_30 = _18.2;
_25 = !_7;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(10_usize, 16_usize, Move(_16), 25_usize, Move(_25), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(10_usize, 1_usize, Move(_1), 2_usize, Move(_2), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [char; 2],mut _2: [u8; 7],mut _3: Adt50,mut _4: [char; 7],mut _5: [char; 7],mut _6: [char; 7],mut _7: ([char; 2], [char; 7], [char; 2]),mut _8: u128,mut _9: [char; 2],mut _10: [char; 7],mut _11: u128,mut _12: [char; 7],mut _13: [char; 7],mut _14: ([char; 2], [char; 7], [char; 2]),mut _15: ([char; 2], [char; 7], [char; 2])) -> [char; 2] {
mir! {
type RET = [char; 2];
let _16: f32;
let _17: u128;
let _18: f64;
let _19: [u128; 7];
let _20: [i64; 5];
let _21: Adt56;
let _22: f64;
let _23: Adt41;
let _24: bool;
let _25: char;
let _26: isize;
let _27: isize;
let _28: (*const i128,);
let _29: bool;
let _30: ();
let _31: ();
{
_14 = _15;
_13 = ['\u{f4b81}','\u{e1c35}','\u{106ac9}','\u{4136c}','\u{16ebf}','\u{34bbc}','\u{2a69b}'];
_14 = _15;
_1 = _14.2;
_11 = _8;
match Field::<i64>(Variant(_3, 2), 4) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
5130034526972326378 => bb9,
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
RET = _7.2;
_7.2 = _14.0;
Call(_9 = fn12(_15, _10, _7, _10, _7.0, _7, _12, RET, Field::<u8>(Variant(_3, 2), 1), _7.1, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = ['\u{82fed}','\u{a53d9}'];
_7 = (_15.2, _4, _1);
_14.2 = _9;
_15.0 = ['\u{fb4e}','\u{2b834}'];
_1 = ['\u{963e7}','\u{ddb82}'];
_2 = Field::<[u8; 7]>(Variant(_3, 2), 2);
_7 = _14;
place!(Field::<[u8; 7]>(Variant(_3, 2), 2)) = _2;
place!(Field::<u128>(Variant(_3, 2), 0)) = _8;
_11 = Field::<i64>(Variant(_3, 2), 4) as u128;
_17 = 9223372036854775807_isize as u128;
_15.1 = ['\u{8418f}','\u{276ed}','\u{48896}','\u{22479}','\u{e467b}','\u{505f9}','\u{ba891}'];
_15.1 = _6;
_9 = ['\u{c75a2}','\u{92505}'];
_13 = ['\u{2c851}','\u{6d55b}','\u{fb61d}','\u{e16c0}','\u{396ab}','\u{b03bb}','\u{7d486}'];
_4 = ['\u{10495d}','\u{dea92}','\u{e93be}','\u{7164d}','\u{1b2ca}','\u{25aeb}','\u{8ebbd}'];
_14.2 = _7.0;
_2 = [Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1),Field::<u8>(Variant(_3, 2), 1)];
SetDiscriminant(_3, 1);
place!(Field::<f64>(Variant(_3, 1), 0)) = _17 as f64;
place!(Field::<*mut *const i128>(Variant(_3, 1), 3)) = core::ptr::addr_of_mut!(place!(Field::<*const i128>(Variant(_3, 1), 1)));
_19 = [_17,_8,_17,_11,_8,_8,_8];
_16 = Field::<f64>(Variant(_3, 1), 0) as f32;
_4 = ['\u{5cff3}','\u{856d8}','\u{2ab27}','\u{b0f4}','\u{90f50}','\u{375d6}','\u{9f90a}'];
_12 = ['\u{e366f}','\u{c5f84}','\u{a9b56}','\u{87f4e}','\u{67623}','\u{a1199}','\u{a5077}'];
Goto(bb11)
}
bb11 = {
_7.1 = ['\u{10744d}','\u{653a1}','\u{4ac91}','\u{e512e}','\u{e276a}','\u{947a0}','\u{3a7ee}'];
_13 = ['\u{9cc99}','\u{8cf3c}','\u{50c8d}','\u{8b84c}','\u{bd111}','\u{f96e}','\u{b411}'];
_13 = _5;
place!(Field::<i16>(Variant(_3, 1), 4)) = 7548563296898710291_u64 as i16;
_14 = _15;
RET = ['\u{f40f6}','\u{f4440}'];
_7.1 = ['\u{99b3b}','\u{ec685}','\u{4c87c}','\u{e8bf8}','\u{8ba86}','\u{ede4e}','\u{88db3}'];
_6 = ['\u{fa2b7}','\u{a648d}','\u{79e8d}','\u{662fd}','\u{c80dd}','\u{64684}','\u{a48c9}'];
_17 = _8;
_19 = [_11,_17,_17,_8,_17,_8,_8];
place!(Field::<[i64; 5]>(Variant(_3, 1), 7)) = [8880210367251512726_i64,(-8071147740404754962_i64),(-832772723579618473_i64),(-3335429160262942966_i64),(-7082445781839173139_i64)];
_16 = (-142268032807567316835427452504770261027_i128) as f32;
_14.1 = ['\u{aabf5}','\u{6647d}','\u{a802f}','\u{641e8}','\u{da264}','\u{26ee4}','\u{d2fb6}'];
_7.1 = ['\u{4eefb}','\u{6b17c}','\u{d10a2}','\u{31fa1}','\u{27823}','\u{7c7c3}','\u{7ffa9}'];
place!(Field::<[i64; 5]>(Variant(_3, 1), 7)) = [7622151886914293572_i64,(-8161248245688874787_i64),5765308591629404868_i64,(-2125590235358276714_i64),486131936864587171_i64];
_7 = _15;
Goto(bb12)
}
bb12 = {
place!(Field::<isize>(Variant(_3, 1), 2)) = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_3, 1), 4)) = -11951_i16;
place!(Field::<isize>(Variant(_3, 1), 2)) = Field::<f64>(Variant(_3, 1), 0) as isize;
_5 = _4;
_22 = Field::<f64>(Variant(_3, 1), 0) - Field::<f64>(Variant(_3, 1), 0);
_20 = Field::<[i64; 5]>(Variant(_3, 1), 7);
_26 = Field::<i16>(Variant(_3, 1), 4) as isize;
place!(Field::<f64>(Variant(_3, 1), 0)) = _22 * _22;
RET = _1;
place!(Field::<[i64; 5]>(Variant(_3, 1), 7)) = _20;
_7.2 = RET;
place!(Field::<f64>(Variant(_3, 1), 0)) = 64325_u16 as f64;
_10 = ['\u{da3e5}','\u{9fa10}','\u{989c9}','\u{82ef3}','\u{47378}','\u{b58e2}','\u{68282}'];
_25 = '\u{674e2}';
_3 = Adt50::Variant2 { fld0: _8,fld1: 16_u8,fld2: _2,fld3: (-7_i8),fld4: 38551843104647029_i64 };
_7.1 = [_25,_25,_25,_25,_25,_25,_25];
place!(Field::<i8>(Variant(_3, 2), 3)) = -68_i8;
_16 = 440607866_u32 as f32;
_22 = 101620006161519137819084385349154985228_i128 as f64;
_1 = [_25,_25];
_9 = [_25,_25];
Goto(bb13)
}
bb13 = {
_10 = [_25,_25,_25,_25,_25,_25,_25];
Goto(bb14)
}
bb14 = {
_20 = [6439271920759463696_i64,(-2914531217973910586_i64),(-2766103342216668952_i64),5885249319127167477_i64,(-1813257694632031451_i64)];
_27 = Field::<i8>(Variant(_3, 2), 3) as isize;
_12 = [_25,_25,_25,_25,_25,_25,_25];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(11_usize, 10_usize, Move(_10), 25_usize, Move(_25), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(11_usize, 2_usize, Move(_2), 15_usize, Move(_15), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(11_usize, 4_usize, Move(_4), 17_usize, Move(_17), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([char; 2], [char; 7], [char; 2]),mut _2: [char; 7],mut _3: ([char; 2], [char; 7], [char; 2]),mut _4: [char; 7],mut _5: [char; 2],mut _6: ([char; 2], [char; 7], [char; 2]),mut _7: [char; 7],mut _8: [char; 2],mut _9: u8,mut _10: [char; 7],mut _11: u128) -> [char; 2] {
mir! {
type RET = [char; 2];
let _12: isize;
let _13: [i64; 5];
let _14: u64;
let _15: f32;
let _16: [u8; 7];
let _17: [u8; 7];
let _18: Adt51;
let _19: *const u128;
let _20: u8;
let _21: bool;
let _22: [i128; 4];
let _23: [char; 2];
let _24: [i64; 5];
let _25: isize;
let _26: Adt46;
let _27: u8;
let _28: ();
let _29: ();
{
_6.1 = _1.1;
_1.0 = _3.0;
_4 = ['\u{f0e5a}','\u{22d9d}','\u{ba7c0}','\u{ce709}','\u{ac56f}','\u{c4d5f}','\u{629d4}'];
_1.0 = _8;
_4 = _10;
_3.2 = ['\u{b2b69}','\u{87da1}'];
RET = _6.0;
_15 = 27148_i16 as f32;
_5 = ['\u{54037}','\u{303f9}'];
_16 = [_9,_9,_9,_9,_9,_9,_9];
_6 = (_1.0, _1.1, RET);
_7 = ['\u{b6ac}','\u{8c9b2}','\u{6f860}','\u{6c5fe}','\u{6ae6b}','\u{9ddb3}','\u{107619}'];
_1.0 = _1.2;
_1.0 = ['\u{4fad8}','\u{b822f}'];
_16 = [_9,_9,_9,_9,_9,_9,_9];
RET = ['\u{15af1}','\u{e2511}'];
_1 = (_3.2, _10, RET);
_15 = 9223372036854775807_isize as f32;
_17 = [_9,_9,_9,_9,_9,_9,_9];
RET = _3.2;
_7 = ['\u{7ff89}','\u{46e2a}','\u{e94bf}','\u{ec01}','\u{7469c}','\u{e1944}','\u{8bfbb}'];
_3.1 = ['\u{a3f9d}','\u{57880}','\u{437df}','\u{6a4ee}','\u{61bbb}','\u{d30a9}','\u{719a9}'];
Call(_1 = fn13(_3.1, _4, _3, _6.1, _6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = 7946500217110684949_u64;
_1.2 = ['\u{f2893}','\u{3fffb}'];
_6 = (_3.0, _3.1, RET);
_14 = !8363444883415549093_u64;
_6 = (_8, _2, _1.0);
_12 = 62994_u16 as isize;
_8 = ['\u{ca6d0}','\u{105393}'];
RET = _3.2;
_20 = _9 >> _9;
RET = ['\u{308fe}','\u{c7207}'];
_14 = 15848923814695080082_u64;
_12 = -(-34_isize);
_15 = 28715932889146677560941964021972275567_i128 as f32;
_19 = core::ptr::addr_of!(_11);
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
15848923814695080082 => bb9,
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
_21 = true | true;
_14 = !8436882477668470249_u64;
_1.2 = _5;
_13 = [2336133620826233566_i64,3203203615884700855_i64,8461788938029781092_i64,3382072835512816933_i64,7229107439187073742_i64];
_4 = ['\u{ec441}','\u{2bb3e}','\u{8af45}','\u{10c918}','\u{10771c}','\u{afddc}','\u{cb8bb}'];
_3 = (_5, _10, RET);
_11 = !229974962026125853497017046351620362882_u128;
_1 = (_6.0, _4, _5);
match _9 {
121 => bb10,
_ => bb8
}
}
bb10 = {
_14 = !14784947490055267442_u64;
_16 = [_9,_9,_20,_20,_9,_20,_20];
_19 = core::ptr::addr_of!(_11);
_1.1 = ['\u{8e9e4}','\u{fa28a}','\u{2452f}','\u{58c1}','\u{77c1f}','\u{ff06a}','\u{26b34}'];
_3 = _6;
_1 = _6;
_6.1 = ['\u{7bfe4}','\u{100d9d}','\u{bdfae}','\u{cd9a1}','\u{10ba5d}','\u{cc801}','\u{5331b}'];
_18 = Adt51::Variant1 { fld0: (-97_i8) };
_1.2 = ['\u{f73c8}','\u{cc7cf}'];
_22 = [118123510396756157602635720409040150461_i128,(-15501105372995453731471164324625962347_i128),34728006031681053846412965983915072978_i128,(-84302376141282894339875821197855624482_i128)];
_16 = [_20,_9,_20,_9,_9,_9,_20];
_23 = RET;
Goto(bb11)
}
bb11 = {
_6.0 = ['\u{6a780}','\u{cfff5}'];
match _9 {
0 => bb1,
1 => bb12,
121 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_21 = true | true;
_14 = !8436882477668470249_u64;
_1.2 = _5;
_13 = [2336133620826233566_i64,3203203615884700855_i64,8461788938029781092_i64,3382072835512816933_i64,7229107439187073742_i64];
_4 = ['\u{ec441}','\u{2bb3e}','\u{8af45}','\u{10c918}','\u{10771c}','\u{afddc}','\u{cb8bb}'];
_3 = (_5, _10, RET);
_11 = !229974962026125853497017046351620362882_u128;
_1 = (_6.0, _4, _5);
match _9 {
121 => bb10,
_ => bb8
}
}
bb14 = {
_18 = Adt51::Variant1 { fld0: (-116_i8) };
_7 = ['\u{7a92}','\u{8d2ba}','\u{397f0}','\u{f22f9}','\u{55cd7}','\u{e059e}','\u{2a7b7}'];
_11 = 95156514300243762433257327936908499941_u128;
_24 = [5524358178594121263_i64,227618065657242579_i64,(-6120196668504226375_i64),2347773574246805506_i64,(-1941273937614912076_i64)];
_4 = _2;
_18 = Adt51::Variant1 { fld0: 18_i8 };
_3.1 = _1.1;
_25 = !_12;
place!(Field::<i8>(Variant(_18, 1), 0)) = 104_i8 | (-122_i8);
_15 = (-106210914081868312051920978518602228426_i128) as f32;
_6 = (_5, _2, _1.0);
_5 = _1.2;
_8 = ['\u{8245}','\u{3e9a8}'];
SetDiscriminant(_18, 1);
_6 = (_5, _3.1, _1.0);
_16 = _17;
_1.0 = ['\u{3e9a3}','\u{ce942}'];
_2 = ['\u{7cfb8}','\u{b335f}','\u{b7b83}','\u{103a74}','\u{9d338}','\u{103f91}','\u{fd90f}'];
_4 = _7;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(12_usize, 25_usize, Move(_25), 1_usize, Move(_1), 5_usize, Move(_5), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(12_usize, 13_usize, Move(_13), 2_usize, Move(_2), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(12_usize, 12_usize, Move(_12), 16_usize, Move(_16), 20_usize, Move(_20), 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [char; 7],mut _2: [char; 7],mut _3: ([char; 2], [char; 7], [char; 2]),mut _4: [char; 7],mut _5: [char; 2]) -> ([char; 2], [char; 7], [char; 2]) {
mir! {
type RET = ([char; 2], [char; 7], [char; 2]);
let _6: Adt51;
let _7: isize;
let _8: bool;
let _9: i32;
let _10: i8;
let _11: bool;
let _12: [i128; 4];
let _13: [u8; 7];
let _14: [i64; 5];
let _15: char;
let _16: u64;
let _17: [u8; 7];
let _18: (u32, i64);
let _19: Adt41;
let _20: u32;
let _21: u8;
let _22: bool;
let _23: u64;
let _24: [char; 2];
let _25: bool;
let _26: char;
let _27: f32;
let _28: u32;
let _29: Adt41;
let _30: Adt57;
let _31: isize;
let _32: Adt50;
let _33: i64;
let _34: ([char; 2],);
let _35: ([char; 2], [char; 7], [char; 2]);
let _36: [u128; 7];
let _37: Adt56;
let _38: [u128; 7];
let _39: char;
let _40: (*const i128,);
let _41: &'static u64;
let _42: f64;
let _43: [u8; 7];
let _44: Adt49;
let _45: u128;
let _46: (u32, i64);
let _47: f64;
let _48: ();
let _49: ();
{
_5 = ['\u{89f72}','\u{1c8b0}'];
RET.0 = ['\u{a6b6b}','\u{ac766}'];
_2 = ['\u{7cd3d}','\u{75e7e}','\u{100820}','\u{f3230}','\u{73ac}','\u{a0df4}','\u{8d6b7}'];
_4 = _3.1;
_4 = ['\u{10128c}','\u{6868f}','\u{10d2c6}','\u{10a03a}','\u{104348}','\u{2d14d}','\u{6c275}'];
_1 = ['\u{f7c79}','\u{3123c}','\u{28671}','\u{3d6bd}','\u{2b439}','\u{d0d17}','\u{82d94}'];
RET.0 = _5;
_2 = _4;
RET.1 = ['\u{66e8e}','\u{513e2}','\u{104535}','\u{fc182}','\u{a6c7f}','\u{db0c7}','\u{33663}'];
_3.0 = ['\u{ff452}','\u{40d2d}'];
RET.2 = _3.0;
RET = _3;
RET.2 = ['\u{693f3}','\u{69e8c}'];
_2 = ['\u{31a6d}','\u{910b5}','\u{d8723}','\u{871e8}','\u{1fd92}','\u{8d052}','\u{11b7d}'];
_5 = ['\u{c46c7}','\u{ec1e5}'];
_3.1 = ['\u{9c3b7}','\u{15bbe}','\u{49227}','\u{343da}','\u{19cf8}','\u{74b33}','\u{45f7d}'];
_10 = 277979056112923556239698220455089793379_u128 as i8;
_10 = 169981056673787216049932780269912111023_i128 as i8;
_7 = (-1856082863867204023_i64) as isize;
RET.2 = _5;
_1 = ['\u{e5c09}','\u{e72bd}','\u{63094}','\u{101f27}','\u{9e13e}','\u{64e66}','\u{af1cd}'];
Goto(bb1)
}
bb1 = {
_4 = ['\u{fd0dd}','\u{174d5}','\u{559da}','\u{7d668}','\u{4c1c4}','\u{5e318}','\u{ffa30}'];
_3.2 = ['\u{b121c}','\u{7ef83}'];
_8 = false;
_2 = ['\u{bc330}','\u{de200}','\u{212c9}','\u{8aa52}','\u{16259}','\u{a532a}','\u{31bbb}'];
RET.0 = ['\u{ce6f}','\u{4b242}'];
_6 = Adt51::Variant1 { fld0: _10 };
_5 = ['\u{9d4a7}','\u{f0078}'];
_7 = !32_isize;
_11 = _8 ^ _8;
RET.0 = ['\u{c43b1}','\u{41eed}'];
_10 = (-129451828258285308784719649813654483989_i128) as i8;
RET.2 = ['\u{ff58c}','\u{60ab7}'];
_10 = Field::<i8>(Variant(_6, 1), 0) & Field::<i8>(Variant(_6, 1), 0);
_8 = !_11;
SetDiscriminant(_6, 1);
_7 = !(-9223372036854775808_isize);
_8 = _10 != _10;
_9 = (-1650848749_i32) & 1709104638_i32;
Goto(bb2)
}
bb2 = {
_5 = ['\u{5f6c}','\u{ffdaf}'];
RET.2 = RET.0;
_11 = _8 >= _8;
_9 = _8 as i32;
_1 = ['\u{febba}','\u{28ed7}','\u{c0ea9}','\u{1a409}','\u{fdb05}','\u{bf9ac}','\u{2491b}'];
RET.0 = ['\u{148c0}','\u{1b6e4}'];
RET.0 = RET.2;
_2 = _3.1;
RET.1 = ['\u{dab4b}','\u{d3903}','\u{7f89d}','\u{e80c0}','\u{5f6e9}','\u{b674d}','\u{2e9ef}'];
_1 = _2;
_6 = Adt51::Variant1 { fld0: _10 };
RET.0 = ['\u{5e7fc}','\u{5237e}'];
RET.2 = _5;
_10 = Field::<i8>(Variant(_6, 1), 0) * Field::<i8>(Variant(_6, 1), 0);
_3.0 = ['\u{bef6a}','\u{c4aba}'];
Goto(bb3)
}
bb3 = {
_1 = _3.1;
_11 = Field::<i8>(Variant(_6, 1), 0) < _10;
_3.2 = ['\u{fafc9}','\u{3a585}'];
SetDiscriminant(_6, 2);
_3.0 = _3.2;
_8 = _11;
_3.0 = _3.2;
RET.2 = RET.0;
RET.1 = _3.1;
_3.0 = ['\u{6cc24}','\u{b5e07}'];
_2 = ['\u{4f4e3}','\u{1f39b}','\u{b85f4}','\u{8e59}','\u{4ee28}','\u{6411d}','\u{21266}'];
place!(Field::<char>(Variant(_6, 2), 1)) = '\u{b0070}';
_2 = [Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1)];
_5 = [Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1)];
RET.0 = [Field::<char>(Variant(_6, 2), 1),Field::<char>(Variant(_6, 2), 1)];
RET.1 = _1;
_6 = Adt51::Variant1 { fld0: _10 };
_8 = !_11;
place!(Field::<i8>(Variant(_6, 1), 0)) = _10 >> _10;
_10 = -Field::<i8>(Variant(_6, 1), 0);
_7 = 30038_i16 as isize;
_12 = [(-152382534829243267143290503639626300747_i128),118085356600025587168102541342551541386_i128,166144106311861842034986949482343602976_i128,93683542539082167695503705332469473634_i128];
_8 = !_11;
RET.2 = ['\u{2460}','\u{47d39}'];
_5 = ['\u{5b71d}','\u{a106e}'];
Call(_13 = fn14(_8, _11, _4, _9, _7, Move(_6), _3.1, _11, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET.2 = ['\u{54123}','\u{f4e0e}'];
RET.1 = ['\u{101322}','\u{f1c8c}','\u{cc018}','\u{5c14b}','\u{18ee3}','\u{ec438}','\u{41bb5}'];
RET.1 = _1;
RET.0 = _3.0;
_10 = !69_i8;
RET.2 = ['\u{4c3ef}','\u{659a7}'];
_8 = _11 >= _11;
_11 = !_8;
_6 = Adt51::Variant1 { fld0: _10 };
_15 = '\u{4a477}';
_12 = [2264191192101894183888289739871684855_i128,(-69170264371259869114110346706734937407_i128),157618892604525499445679052167962477820_i128,39131734917455402066681956907254896458_i128];
SetDiscriminant(_6, 1);
_2 = [_15,_15,_15,_15,_15,_15,_15];
RET.2 = [_15,_15];
_7 = 11377621393052383463_u64 as isize;
place!(Field::<i8>(Variant(_6, 1), 0)) = _10;
_1 = [_15,_15,_15,_15,_15,_15,_15];
RET.2 = RET.0;
_10 = !Field::<i8>(Variant(_6, 1), 0);
_16 = 4357902245501595112_u64;
_12 = [(-76923966471415869710539433350239422645_i128),116687122774100426488119668121473334798_i128,1484424768530405555964941473178420155_i128,138881377235911473872194686403868084148_i128];
_6 = Adt51::Variant1 { fld0: _10 };
place!(Field::<i8>(Variant(_6, 1), 0)) = _10 >> _9;
_2 = _3.1;
place!(Field::<i8>(Variant(_6, 1), 0)) = _16 as i8;
Goto(bb5)
}
bb5 = {
_18 = (3787869447_u32, 1819128822422203981_i64);
_3 = (RET.2, RET.1, RET.2);
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_17 = [69_u8,54_u8,66_u8,80_u8,201_u8,64_u8,188_u8];
_18 = (2575413523_u32, (-5788884131094627209_i64));
_3.0 = RET.0;
_20 = _18.0;
_10 = 65090_u16 as i8;
_5 = [_15,_15];
_9 = _10 as i32;
_7 = (-9223372036854775808_isize);
_18.0 = _20;
RET.1 = [_15,_15,_15,_15,_15,_15,_15];
SetDiscriminant(_6, 1);
_5 = [_15,_15];
_2 = _4;
Goto(bb6)
}
bb6 = {
_21 = 130_u8;
RET.2 = [_15,_15];
_22 = _8 == _11;
_18.1 = 2610077748928784611_i64 ^ (-2813995178983533296_i64);
_16 = _22 as u64;
RET.1 = [_15,_15,_15,_15,_15,_15,_15];
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_16 = _21 as u64;
_4 = [_15,_15,_15,_15,_15,_15,_15];
_3 = RET;
RET = (_3.2, _2, _3.0);
RET.0 = [_15,_15];
_19 = Adt41::Variant2 { fld0: _22,fld1: _12 };
RET.2 = _3.0;
RET.2 = [_15,_15];
Goto(bb7)
}
bb7 = {
_23 = 11513298094078410511_usize as u64;
_17 = [_21,_21,_21,_21,_21,_21,_21];
_7 = 62_isize;
_21 = 228_u8;
_25 = !_22;
place!(Field::<i8>(Variant(_6, 1), 0)) = -_10;
_1 = [_15,_15,_15,_15,_15,_15,_15];
_18.1 = (-7681491411256282606_i64);
_9 = (-176131212_i32);
_4 = [_15,_15,_15,_15,_15,_15,_15];
_3.0 = [_15,_15];
RET = (_5, _2, _3.0);
_3.0 = [_15,_15];
_3.2 = [_15,_15];
_27 = 48137486688943353563882727385476756567_i128 as f32;
RET.2 = _3.2;
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_10 = -Field::<i8>(Variant(_6, 1), 0);
_3 = (_5, _2, RET.0);
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_4 = _3.1;
_15 = '\u{bfe0a}';
_3 = (RET.2, RET.1, RET.2);
SetDiscriminant(_6, 2);
_10 = (-40_i8) - (-125_i8);
RET.2 = [_15,_15];
RET.2 = RET.0;
SetDiscriminant(_19, 2);
_2 = [_15,_15,_15,_15,_15,_15,_15];
RET.0 = _3.0;
Goto(bb8)
}
bb8 = {
_28 = (-17718_i16) as u32;
match _18.1 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463455693116020511928850 => bb9,
_ => bb4
}
}
bb9 = {
_9 = _7 as i32;
_9 = -1356040860_i32;
_11 = _22;
_24 = [_15,_15];
RET.2 = _5;
place!(Field::<[i128; 4]>(Variant(_19, 2), 1)) = [(-53192246920684091641236893016510333234_i128),22445171991797791909120177440626617933_i128,(-126955450034996144497398446681474169661_i128),31534083535314982132090748411036317481_i128];
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
place!(Field::<bool>(Variant(_19, 2), 0)) = !_25;
_22 = !Field::<bool>(Variant(_19, 2), 0);
_15 = '\u{10df3a}';
_22 = _11;
RET.1 = [_15,_15,_15,_15,_15,_15,_15];
match _21 {
228 => bb10,
_ => bb7
}
}
bb10 = {
_21 = 33_u8 * 246_u8;
_3.1 = [_15,_15,_15,_15,_15,_15,_15];
RET.2 = RET.0;
_31 = !_7;
_2 = [_15,_15,_15,_15,_15,_15,_15];
SetDiscriminant(_19, 0);
_32 = Adt50::Variant2 { fld0: 293070617654486875428408118786999128358_u128,fld1: _21,fld2: _13,fld3: _10,fld4: _18.1 };
_8 = !_22;
place!(Field::<(*const i128,)>(Variant(_19, 0), 4)).0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_19, 0), 1)));
place!(Field::<[u8; 7]>(Variant(_32, 2), 2)) = [Field::<u8>(Variant(_32, 2), 1),Field::<u8>(Variant(_32, 2), 1),Field::<u8>(Variant(_32, 2), 1),Field::<u8>(Variant(_32, 2), 1),_21,Field::<u8>(Variant(_32, 2), 1),Field::<u8>(Variant(_32, 2), 1)];
_7 = _31;
_18.1 = !Field::<i64>(Variant(_32, 2), 4);
RET.0 = [_15,_15];
RET.2 = [_15,_15];
place!(Field::<u128>(Variant(_32, 2), 0)) = !263684697871723878134344668906198655032_u128;
_18.1 = Field::<u128>(Variant(_32, 2), 0) as i64;
place!(Field::<i128>(Variant(_19, 0), 1)) = 141404976409346461188286473585399005676_i128;
_19 = Adt41::Variant2 { fld0: _22,fld1: _12 };
match _18.0 {
0 => bb7,
1 => bb5,
2 => bb6,
2575413523 => bb12,
_ => bb11
}
}
bb11 = {
_5 = ['\u{5f6c}','\u{ffdaf}'];
RET.2 = RET.0;
_11 = _8 >= _8;
_9 = _8 as i32;
_1 = ['\u{febba}','\u{28ed7}','\u{c0ea9}','\u{1a409}','\u{fdb05}','\u{bf9ac}','\u{2491b}'];
RET.0 = ['\u{148c0}','\u{1b6e4}'];
RET.0 = RET.2;
_2 = _3.1;
RET.1 = ['\u{dab4b}','\u{d3903}','\u{7f89d}','\u{e80c0}','\u{5f6e9}','\u{b674d}','\u{2e9ef}'];
_1 = _2;
_6 = Adt51::Variant1 { fld0: _10 };
RET.0 = ['\u{5e7fc}','\u{5237e}'];
RET.2 = _5;
_10 = Field::<i8>(Variant(_6, 1), 0) * Field::<i8>(Variant(_6, 1), 0);
_3.0 = ['\u{bef6a}','\u{c4aba}'];
Goto(bb3)
}
bb12 = {
_29 = Adt41::Variant1 { fld0: _16 };
_18.0 = _28 + _20;
place!(Field::<[char; 2]>(Variant(_6, 2), 0)) = [_15,_15];
_35 = (_3.0, _4, RET.2);
RET.1 = [_15,_15,_15,_15,_15,_15,_15];
SetDiscriminant(_32, 0);
_35.1 = _4;
place!(Field::<bool>(Variant(_32, 0), 0)) = _25;
SetDiscriminant(_29, 2);
Goto(bb13)
}
bb13 = {
_11 = !_8;
_29 = Move(_19);
_13 = [_21,_21,_21,_21,_21,_21,_21];
_9 = !(-365132008_i32);
_35 = (_24, _2, Field::<[char; 2]>(Variant(_6, 2), 0));
SetDiscriminant(_29, 0);
place!(Field::<*mut [u128; 7]>(Variant(_6, 2), 2)) = core::ptr::addr_of_mut!(_36);
place!(Field::<i128>(Variant(_29, 0), 1)) = !(-28727918876112656949977659070303666208_i128);
_38 = [24146273583646343856534587616096363159_u128,318877367357020991958414449620879537539_u128,323075098242610693401258638492327193538_u128,124875294864526799934525142813167939638_u128,228983531717902373348793935288734963723_u128,75031024103101456356566943463878712035_u128,289786930823196766774674952799830662727_u128];
_28 = !_18.0;
RET = (_24, _4, _5);
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
place!(Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1)).1 = core::ptr::addr_of_mut!(place!(Field::<(*const i128,)>(Variant(_29, 0), 4)).0);
_1 = _4;
Goto(bb14)
}
bb14 = {
place!(Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1)).2 = _7;
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_40.0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_29, 0), 1)));
place!(Field::<u128>(Variant(_29, 0), 2)) = !194751025196468774949159238606247088059_u128;
_1 = _4;
place!(Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1)).3 = _21;
_39 = _15;
_18 = (_28, (-5644405654163083013_i64));
_19 = Adt41::Variant0 { fld0: _35.2,fld1: Field::<i128>(Variant(_29, 0), 1),fld2: Field::<u128>(Variant(_29, 0), 2),fld3: _14,fld4: _40 };
_34 = (_3.2,);
place!(Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1)).0 = _31;
place!(Field::<[u32; 4]>(Variant(_32, 0), 4)) = [_20,_18.0,_18.0,_28];
_13 = [_21,_21,_21,Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1).3,_21,Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1).3,_21];
_29 = Move(_19);
_3 = (RET.0, RET.1, _35.0);
_33 = _18.1;
place!(Field::<isize>(Variant(_32, 0), 2)) = -Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1).0;
_23 = Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1).3 as u64;
place!(Field::<(isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,))>(Variant(_32, 0), 1)).5 = _40;
_34.0 = Field::<[char; 2]>(Variant(_29, 0), 0);
RET.0 = _3.2;
_18.1 = 3486_i16 as i64;
_43 = _17;
place!(Field::<*mut [u128; 7]>(Variant(_6, 2), 2)) = core::ptr::addr_of_mut!(_38);
place!(Field::<(*const i128,)>(Variant(_29, 0), 4)).0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_29, 0), 1)));
RET = (_34.0, _3.1, _35.2);
place!(Field::<[char; 2]>(Variant(_6, 2), 0)) = [_39,_15];
_38 = [Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2),Field::<u128>(Variant(_29, 0), 2)];
place!(Field::<[char; 2]>(Variant(_6, 2), 0)) = Field::<[char; 2]>(Variant(_29, 0), 0);
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(13_usize, 22_usize, Move(_22), 13_usize, Move(_13), 24_usize, Move(_24), 34_usize, Move(_34)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(13_usize, 11_usize, Move(_11), 17_usize, Move(_17), 21_usize, Move(_21), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(13_usize, 7_usize, Move(_7), 9_usize, Move(_9), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(13_usize, 38_usize, Move(_38), 33_usize, Move(_33), 18_usize, Move(_18), 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: bool,mut _3: [char; 7],mut _4: i32,mut _5: isize,mut _6: Adt51,mut _7: [char; 7],mut _8: bool,mut _9: i8) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _10: i16;
let _11: [char; 7];
let _12: i32;
let _13: u128;
let _14: *mut u16;
let _15: isize;
let _16: char;
let _17: Adt45;
let _18: [u8; 7];
let _19: Adt41;
let _20: [u128; 7];
let _21: isize;
let _22: f32;
let _23: ([char; 2],);
let _24: [u32; 4];
let _25: i8;
let _26: Adt42;
let _27: ();
let _28: ();
{
_2 = !_8;
_9 = Field::<i8>(Variant(_6, 1), 0);
_5 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_2 = _1 & _1;
_1 = _2 & _2;
RET = [161_u8,180_u8,173_u8,153_u8,246_u8,78_u8,10_u8];
_9 = Field::<i8>(Variant(_6, 1), 0) | Field::<i8>(Variant(_6, 1), 0);
place!(Field::<i8>(Variant(_6, 1), 0)) = _5 as i8;
_10 = 32058_i16 + 21023_i16;
RET = [104_u8,181_u8,201_u8,12_u8,183_u8,71_u8,233_u8];
_10 = -23565_i16;
RET = [216_u8,55_u8,23_u8,180_u8,103_u8,63_u8,147_u8];
SetDiscriminant(_6, 0);
_1 = _8 > _2;
_1 = _2;
_8 = _2 != _1;
RET = [52_u8,254_u8,185_u8,61_u8,231_u8,38_u8,87_u8];
Call(place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_6, 0), 0)).1 = fn15(RET, _9, _2, _5, _9, _9, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = ['\u{c960e}','\u{ea2b4}','\u{ae4b4}','\u{e53a7}','\u{f2cd1}','\u{e2a30}','\u{bc68f}'];
_7 = ['\u{d72e0}','\u{6848b}','\u{cdb38}','\u{181b4}','\u{5d0b3}','\u{30cf}','\u{a93a1}'];
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_6, 0), 0)).1 = ['\u{64737}','\u{b70c8}','\u{ffb70}','\u{dea3c}','\u{1fd5a}','\u{e40a4}','\u{c4b1e}'];
Goto(bb2)
}
bb2 = {
_9 = 82_i8;
_5 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
place!(Field::<i64>(Variant(_6, 0), 2)) = _4 as i64;
_11 = ['\u{122b9}','\u{10e115}','\u{2a872}','\u{b6dea}','\u{e4be7}','\u{cee39}','\u{217c8}'];
place!(Field::<i64>(Variant(_6, 0), 2)) = (-185805053753588870_i64);
place!(Field::<Adt44>(Variant(_6, 0), 3)).fld0 = [3512192956_u32,2235455562_u32,1984307897_u32,1073869010_u32];
_9 = -(-48_i8);
_10 = (-17148_i16);
RET = [197_u8,100_u8,226_u8,133_u8,112_u8,49_u8,79_u8];
Goto(bb3)
}
bb3 = {
_6 = Adt51::Variant1 { fld0: _9 };
_10 = (-6691_i16);
RET = [96_u8,243_u8,136_u8,224_u8,80_u8,21_u8,215_u8];
_12 = _4;
_10 = 67876400840316491633979621020592996826_u128 as i16;
_1 = _2;
Goto(bb4)
}
bb4 = {
_5 = -9223372036854775807_isize;
_7 = _3;
RET = [25_u8,221_u8,47_u8,139_u8,163_u8,122_u8,184_u8];
_6 = Adt51::Variant1 { fld0: _9 };
_5 = (-33_isize);
RET = [190_u8,147_u8,229_u8,71_u8,126_u8,119_u8,237_u8];
_3 = ['\u{167ac}','\u{4eb18}','\u{ac55e}','\u{da687}','\u{edda9}','\u{e47b1}','\u{c53ec}'];
_13 = 5352968200214796436_usize as u128;
_5 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1 = _2 > _2;
place!(Field::<i8>(Variant(_6, 1), 0)) = _12 as i8;
_12 = 33748_u16 as i32;
place!(Field::<i8>(Variant(_6, 1), 0)) = _9;
_11 = ['\u{aa878}','\u{9ccfe}','\u{f4f8f}','\u{ce6f6}','\u{8c7ae}','\u{bdede}','\u{4e945}'];
_16 = '\u{70b93}';
_11 = _7;
Call(_13 = core::intrinsics::bswap(84758424899054258825241150943578578629_u128), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17.fld1.0 = [_16,_16];
RET = [103_u8,14_u8,120_u8,239_u8,41_u8,95_u8,133_u8];
SetDiscriminant(_6, 1);
_3 = [_16,_16,_16,_16,_16,_16,_16];
_17.fld2 = -_5;
_17.fld1.2 = [_16,_16];
_15 = _17.fld2;
_15 = _17.fld2 - _5;
_17.fld1.2 = _17.fld1.0;
_9 = (-87_i8);
_13 = !158820149444705965412098900708264740531_u128;
place!(Field::<i8>(Variant(_6, 1), 0)) = !_9;
_12 = _4 >> _4;
_19 = Adt41::Variant1 { fld0: 7581098059611425015_u64 };
_16 = '\u{794f8}';
_18 = [69_u8,195_u8,92_u8,118_u8,211_u8,187_u8,218_u8];
match _9 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463463374607431768211369 => bb13,
_ => bb12
}
}
bb6 = {
_5 = -9223372036854775807_isize;
_7 = _3;
RET = [25_u8,221_u8,47_u8,139_u8,163_u8,122_u8,184_u8];
_6 = Adt51::Variant1 { fld0: _9 };
_5 = (-33_isize);
RET = [190_u8,147_u8,229_u8,71_u8,126_u8,119_u8,237_u8];
_3 = ['\u{167ac}','\u{4eb18}','\u{ac55e}','\u{da687}','\u{edda9}','\u{e47b1}','\u{c53ec}'];
_13 = 5352968200214796436_usize as u128;
_5 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1 = _2 > _2;
place!(Field::<i8>(Variant(_6, 1), 0)) = _12 as i8;
_12 = 33748_u16 as i32;
place!(Field::<i8>(Variant(_6, 1), 0)) = _9;
_11 = ['\u{aa878}','\u{9ccfe}','\u{f4f8f}','\u{ce6f6}','\u{8c7ae}','\u{bdede}','\u{4e945}'];
_16 = '\u{70b93}';
_11 = _7;
Call(_13 = core::intrinsics::bswap(84758424899054258825241150943578578629_u128), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_6 = Adt51::Variant1 { fld0: _9 };
_10 = (-6691_i16);
RET = [96_u8,243_u8,136_u8,224_u8,80_u8,21_u8,215_u8];
_12 = _4;
_10 = 67876400840316491633979621020592996826_u128 as i16;
_1 = _2;
Goto(bb4)
}
bb8 = {
_9 = 82_i8;
_5 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
place!(Field::<i64>(Variant(_6, 0), 2)) = _4 as i64;
_11 = ['\u{122b9}','\u{10e115}','\u{2a872}','\u{b6dea}','\u{e4be7}','\u{cee39}','\u{217c8}'];
place!(Field::<i64>(Variant(_6, 0), 2)) = (-185805053753588870_i64);
place!(Field::<Adt44>(Variant(_6, 0), 3)).fld0 = [3512192956_u32,2235455562_u32,1984307897_u32,1073869010_u32];
_9 = -(-48_i8);
_10 = (-17148_i16);
RET = [197_u8,100_u8,226_u8,133_u8,112_u8,49_u8,79_u8];
Goto(bb3)
}
bb9 = {
_3 = ['\u{c960e}','\u{ea2b4}','\u{ae4b4}','\u{e53a7}','\u{f2cd1}','\u{e2a30}','\u{bc68f}'];
_7 = ['\u{d72e0}','\u{6848b}','\u{cdb38}','\u{181b4}','\u{5d0b3}','\u{30cf}','\u{a93a1}'];
place!(Field::<([char; 2], [char; 7], [char; 2])>(Variant(_6, 0), 0)).1 = ['\u{64737}','\u{b70c8}','\u{ffb70}','\u{dea3c}','\u{1fd5a}','\u{e40a4}','\u{c4b1e}'];
Goto(bb2)
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
_12 = 17_u8 as i32;
_1 = _2 <= _2;
RET = [231_u8,151_u8,140_u8,27_u8,121_u8,114_u8,38_u8];
place!(Field::<u64>(Variant(_19, 1), 0)) = _15 as u64;
_5 = -_15;
_5 = !_15;
_17.fld1.0 = [_16,_16];
place!(Field::<i8>(Variant(_6, 1), 0)) = _9 * _9;
place!(Field::<u64>(Variant(_19, 1), 0)) = !9398055123840569405_u64;
Goto(bb14)
}
bb14 = {
_20 = [_13,_13,_13,_13,_13,_13,_13];
_17.fld0 = [85_u8,165_u8,120_u8,24_u8,20_u8,44_u8,226_u8];
_17.fld2 = _15;
_4 = _12;
_10 = (-897_i16) << _15;
_21 = Field::<u64>(Variant(_19, 1), 0) as isize;
_24 = [1447677976_u32,862359455_u32,3390183977_u32,1220959179_u32];
_18 = [45_u8,225_u8,185_u8,233_u8,126_u8,149_u8,223_u8];
_2 = !_1;
_17.fld0 = RET;
_24 = [413249482_u32,3924013050_u32,4075044044_u32,3007115871_u32];
_22 = 6_usize as f32;
_5 = _17.fld2 + _15;
_22 = _10 as f32;
SetDiscriminant(_19, 2);
SetDiscriminant(_6, 1);
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 8_usize, Move(_8), 1_usize, Move(_1), 20_usize, Move(_20), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 7_usize, Move(_7), 3_usize, Move(_3), 10_usize, Move(_10), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(14_usize, 18_usize, Move(_18), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u8; 7],mut _2: i8,mut _3: bool,mut _4: isize,mut _5: i8,mut _6: i8,mut _7: bool) -> [char; 7] {
mir! {
type RET = [char; 7];
let _8: *mut i16;
let _9: u8;
let _10: [i64; 5];
let _11: i16;
let _12: i32;
let _13: (u32, i64);
let _14: *const i128;
let _15: isize;
let _16: [char; 2];
let _17: bool;
let _18: [i64; 5];
let _19: Adt52;
let _20: [char; 2];
let _21: isize;
let _22: bool;
let _23: isize;
let _24: [char; 7];
let _25: Adt42;
let _26: Adt53;
let _27: [char; 7];
let _28: Adt46;
let _29: isize;
let _30: u16;
let _31: f32;
let _32: f64;
let _33: bool;
let _34: Adt45;
let _35: [char; 2];
let _36: bool;
let _37: i32;
let _38: (u32, i64);
let _39: f32;
let _40: ();
let _41: ();
{
_6 = 229_u8 as i8;
RET = ['\u{690ef}','\u{33edf}','\u{20b01}','\u{10b26e}','\u{10cbb8}','\u{dab82}','\u{53290}'];
_9 = 184_u8 & 173_u8;
_5 = -_2;
_9 = 7848204772933508392_u64 as u8;
_5 = _9 as i8;
_2 = _3 as i8;
_2 = _6;
_7 = _5 >= _2;
_5 = _6 - _2;
_5 = -_6;
_10 = [7705728472365999458_i64,(-3902585541346600208_i64),(-2576623871069331930_i64),3996099039000310820_i64,(-4192905188055646848_i64)];
RET = ['\u{9050a}','\u{ec5ea}','\u{7cb12}','\u{d3b6b}','\u{c8d9}','\u{d2d5e}','\u{cf27c}'];
_10 = [(-2824875869191193900_i64),5746983604192733735_i64,(-4425444848873562953_i64),(-5897117912337056254_i64),(-8750035863862580115_i64)];
_2 = _5 - _6;
_4 = !9223372036854775807_isize;
_3 = _7 < _7;
Goto(bb1)
}
bb1 = {
_4 = !(-78_isize);
_6 = 10742434650989349293_u64 as i8;
_1 = [_9,_9,_9,_9,_9,_9,_9];
_9 = 217_u8 | 139_u8;
RET = ['\u{b121e}','\u{358c3}','\u{92595}','\u{dd0b6}','\u{8197f}','\u{96d6e}','\u{61b17}'];
RET = ['\u{c8752}','\u{926ca}','\u{6a3a8}','\u{4ed7a}','\u{be210}','\u{8cab8}','\u{41ecc}'];
_6 = -_5;
_7 = !_3;
_8 = core::ptr::addr_of_mut!(_11);
_7 = !_3;
_10 = [1577521887673848812_i64,(-6897841215967721315_i64),6528973314136939369_i64,157583936522559416_i64,8052127455603277919_i64];
_9 = _6 as u8;
_11 = (-21040_i16) & (-7558_i16);
_10 = [(-7013250825371005767_i64),8009725427386417952_i64,6669912016862985402_i64,9038384131216700466_i64,(-6537440578995516974_i64)];
_3 = !_7;
_6 = _2;
_6 = 1691537321_i32 as i8;
_3 = _7;
_10 = [41966788642882690_i64,(-4201664821701431069_i64),(-6489368783853307273_i64),150500620189035150_i64,6647085118087559453_i64];
_4 = (-17_isize);
_10 = [3734120973157719219_i64,681178586898797144_i64,6660447481779895052_i64,(-303044807260548689_i64),125170864821627220_i64];
_12 = 1403844414_i32 | (-2131021283_i32);
_1 = [_9,_9,_9,_9,_9,_9,_9];
RET = ['\u{b9529}','\u{97e33}','\u{14b5f}','\u{22229}','\u{ec4b}','\u{39c87}','\u{100118}'];
_11 = (-18640_i16);
Call(_3 = fn16((*_8), _4, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = (-120130750247086952533186645995981812333_i128) as i8;
_10 = [(-549817962532140551_i64),8647970155788575983_i64,8797466966732789403_i64,(-5964685373347130603_i64),712887723782316580_i64];
_6 = 97643166088258414835470430506739049882_i128 as i8;
_4 = (-9223372036854775808_isize);
(*_8) = 2111_i16 * 16971_i16;
_5 = !_2;
_13 = (402866969_u32, (-94057740509261340_i64));
_13.1 = 1818082650133962033_i64;
(*_8) = !(-2069_i16);
_12 = '\u{1e7e9}' as i32;
_13 = (176773630_u32, 1225830379946011300_i64);
_16 = ['\u{96f44}','\u{a5847}'];
RET = ['\u{c5755}','\u{8cb02}','\u{bbde5}','\u{a874}','\u{adb7c}','\u{14e5d}','\u{6267d}'];
_4 = (-9223372036854775808_isize) & 66_isize;
_11 = (-719_i16) * 26257_i16;
_1 = [_9,_9,_9,_9,_9,_9,_9];
_5 = _12 as i8;
Goto(bb3)
}
bb3 = {
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_4 = !33_isize;
_9 = 188_u8;
_5 = -_6;
_13.1 = (-1270069262623766824_i64) | 3406667985911917724_i64;
_6 = 89147362870114838617219718620615435761_i128 as i8;
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
RET = ['\u{d79d3}','\u{8445f}','\u{5b56d}','\u{b5a93}','\u{48508}','\u{a0892}','\u{5d0bc}'];
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_4 = -9223372036854775807_isize;
_15 = _4 | _4;
_12 = _13.0 as i32;
_1 = [_9,_9,_9,_9,_9,_9,_9];
_17 = _15 == _4;
_12 = (-1731642635_i32);
Goto(bb4)
}
bb4 = {
_4 = !_15;
RET = ['\u{5b9d2}','\u{f06f3}','\u{10f5a6}','\u{1a7cd}','\u{19dbc}','\u{3a6f1}','\u{9563a}'];
_8 = core::ptr::addr_of_mut!(_11);
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_2 = _5 >> _5;
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
RET = ['\u{6476f}','\u{cdb16}','\u{71c7b}','\u{868f}','\u{18daa}','\u{ae46c}','\u{734e9}'];
_7 = _3;
_4 = !_15;
_18 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_2 = _6;
Call(_8 = fn17(RET, _12, (*_8), _3, (*_8), _13, _4, _13.1, RET, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = core::ptr::addr_of_mut!(_11);
_18 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_7 = _17 <= _3;
_13.1 = 2056119077027144012_i64;
_19 = Adt52::Variant1 { fld0: 58459_u16 };
_22 = _7;
Goto(bb6)
}
bb6 = {
_11 = (-23230_i16);
_12 = _13.0 as i32;
_10 = _18;
_20 = _16;
_9 = 216_u8;
_13 = (1966469961_u32, 8499463367748467247_i64);
Goto(bb7)
}
bb7 = {
_18 = _10;
_20 = ['\u{4d480}','\u{3a1f7}'];
_13 = (3330405236_u32, 5814692852306650178_i64);
match _13.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3330405236 => bb8,
_ => bb5
}
}
bb8 = {
_5 = _6;
_13.1 = _12 as i64;
_21 = (*_8) as isize;
_13.0 = _11 as u32;
place!(Field::<u16>(Variant(_19, 1), 0)) = 4_usize as u16;
_12 = 969270274_i32 & 1459804812_i32;
_23 = -_15;
_24 = ['\u{a4809}','\u{962e4}','\u{364f}','\u{8057f}','\u{7d74e}','\u{b4fa0}','\u{a7344}'];
_15 = _21 << _2;
RET = ['\u{39f73}','\u{9dbd1}','\u{a0afc}','\u{10b0ca}','\u{566cf}','\u{ede83}','\u{d9a5c}'];
_7 = _21 != _23;
_15 = _21;
_8 = core::ptr::addr_of_mut!(_11);
_1 = [_9,_9,_9,_9,_9,_9,_9];
_2 = (*_8) as i8;
_13 = (3308053306_u32, (-7791116246674230543_i64));
_6 = _2 - _2;
Call(_2 = core::intrinsics::transmute(_7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13.0 = 2560466039_u32 - 288706895_u32;
(*_8) = 30804_i16;
RET = _24;
_2 = !_5;
_1 = [_9,_9,_9,_9,_9,_9,_9];
(*_8) = (-1674_i16);
_20 = ['\u{31e2c}','\u{10861e}'];
_23 = Field::<u16>(Variant(_19, 1), 0) as isize;
_12 = Field::<u16>(Variant(_19, 1), 0) as i32;
_23 = _4;
_28 = Adt46::Variant3 { fld0: _24,fld1: _16 };
RET = ['\u{1d064}','\u{990ef}','\u{cbaa6}','\u{c221b}','\u{a5203}','\u{420c}','\u{16399}'];
_20 = ['\u{b4d02}','\u{a9de4}'];
_4 = !_23;
_13 = (704030941_u32, 4567089232299742140_i64);
_27 = ['\u{100e22}','\u{6c6b4}','\u{ef826}','\u{7b1fe}','\u{c173d}','\u{b522b}','\u{100359}'];
_29 = _9 as isize;
_11 = !13901_i16;
_18 = [_13.1,_13.1,_13.1,_13.1,_13.1];
Goto(bb10)
}
bb10 = {
_6 = _4 as i8;
_4 = _13.0 as isize;
place!(Field::<[char; 2]>(Variant(_28, 3), 1)) = ['\u{3cafd}','\u{a0a1e}'];
_15 = _4;
_20 = ['\u{104335}','\u{10bc4d}'];
_10 = _18;
_15 = _23;
_22 = !_3;
Call(_19 = fn18(_3, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13 = (1762148070_u32, (-7820773741253917591_i64));
(*_8) = !23650_i16;
RET = ['\u{bb55f}','\u{eb281}','\u{bddbd}','\u{88ac7}','\u{3754d}','\u{ed782}','\u{e6397}'];
(*_8) = 26721_i16;
_12 = -(-1226713503_i32);
_9 = _12 as u8;
_13 = (3660512989_u32, 8708408302427710106_i64);
_6 = _2 ^ _2;
_32 = 2041697411409681493_u64 as f64;
_13 = (2886445863_u32, (-5229593938239546229_i64));
_9 = 252_u8;
_30 = Field::<u16>(Variant(_19, 1), 0) << (*_8);
place!(Field::<[char; 2]>(Variant(_28, 3), 1)) = ['\u{662dc}','\u{56f62}'];
_20 = ['\u{d9c64}','\u{2de58}'];
_15 = _29 - _4;
_24 = ['\u{84e09}','\u{4bf2}','\u{8e789}','\u{d8e35}','\u{56c53}','\u{53603}','\u{9085c}'];
Goto(bb12)
}
bb12 = {
_6 = 126839114593973427041395544968522421546_i128 as i8;
_34.fld0 = [_9,_9,_9,_9,_9,_9,_9];
RET = _27;
(*_8) = _13.0 as i16;
_5 = _2 | _6;
RET = Field::<[char; 7]>(Variant(_28, 3), 0);
_31 = 18048879072106427236_u64 as f32;
(*_8) = -25948_i16;
_18 = [_13.1,_13.1,_13.1,_13.1,_13.1];
match _13.1 {
0 => bb1,
1 => bb6,
2 => bb4,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463458145013493528665227 => bb17,
_ => bb16
}
}
bb13 = {
_11 = (-23230_i16);
_12 = _13.0 as i32;
_10 = _18;
_20 = _16;
_9 = 216_u8;
_13 = (1966469961_u32, 8499463367748467247_i64);
Goto(bb7)
}
bb14 = {
_6 = _4 as i8;
_4 = _13.0 as isize;
place!(Field::<[char; 2]>(Variant(_28, 3), 1)) = ['\u{3cafd}','\u{a0a1e}'];
_15 = _4;
_20 = ['\u{104335}','\u{10bc4d}'];
_10 = _18;
_15 = _23;
_22 = !_3;
Call(_19 = fn18(_3, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_4 = !(-78_isize);
_6 = 10742434650989349293_u64 as i8;
_1 = [_9,_9,_9,_9,_9,_9,_9];
_9 = 217_u8 | 139_u8;
RET = ['\u{b121e}','\u{358c3}','\u{92595}','\u{dd0b6}','\u{8197f}','\u{96d6e}','\u{61b17}'];
RET = ['\u{c8752}','\u{926ca}','\u{6a3a8}','\u{4ed7a}','\u{be210}','\u{8cab8}','\u{41ecc}'];
_6 = -_5;
_7 = !_3;
_8 = core::ptr::addr_of_mut!(_11);
_7 = !_3;
_10 = [1577521887673848812_i64,(-6897841215967721315_i64),6528973314136939369_i64,157583936522559416_i64,8052127455603277919_i64];
_9 = _6 as u8;
_11 = (-21040_i16) & (-7558_i16);
_10 = [(-7013250825371005767_i64),8009725427386417952_i64,6669912016862985402_i64,9038384131216700466_i64,(-6537440578995516974_i64)];
_3 = !_7;
_6 = _2;
_6 = 1691537321_i32 as i8;
_3 = _7;
_10 = [41966788642882690_i64,(-4201664821701431069_i64),(-6489368783853307273_i64),150500620189035150_i64,6647085118087559453_i64];
_4 = (-17_isize);
_10 = [3734120973157719219_i64,681178586898797144_i64,6660447481779895052_i64,(-303044807260548689_i64),125170864821627220_i64];
_12 = 1403844414_i32 | (-2131021283_i32);
_1 = [_9,_9,_9,_9,_9,_9,_9];
RET = ['\u{b9529}','\u{97e33}','\u{14b5f}','\u{22229}','\u{ec4b}','\u{39c87}','\u{100118}'];
_11 = (-18640_i16);
Call(_3 = fn16((*_8), _4, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_6 = (-120130750247086952533186645995981812333_i128) as i8;
_10 = [(-549817962532140551_i64),8647970155788575983_i64,8797466966732789403_i64,(-5964685373347130603_i64),712887723782316580_i64];
_6 = 97643166088258414835470430506739049882_i128 as i8;
_4 = (-9223372036854775808_isize);
(*_8) = 2111_i16 * 16971_i16;
_5 = !_2;
_13 = (402866969_u32, (-94057740509261340_i64));
_13.1 = 1818082650133962033_i64;
(*_8) = !(-2069_i16);
_12 = '\u{1e7e9}' as i32;
_13 = (176773630_u32, 1225830379946011300_i64);
_16 = ['\u{96f44}','\u{a5847}'];
RET = ['\u{c5755}','\u{8cb02}','\u{bbde5}','\u{a874}','\u{adb7c}','\u{14e5d}','\u{6267d}'];
_4 = (-9223372036854775808_isize) & 66_isize;
_11 = (-719_i16) * 26257_i16;
_1 = [_9,_9,_9,_9,_9,_9,_9];
_5 = _12 as i8;
Goto(bb3)
}
bb17 = {
_30 = Field::<u16>(Variant(_19, 1), 0);
_24 = RET;
_33 = _22 == _22;
(*_8) = (-22438_i16);
_34.fld2 = _9 as isize;
_11 = _30 as i16;
_5 = -_6;
_35 = ['\u{c48ab}','\u{c0a07}'];
_27 = ['\u{fce77}','\u{f77d}','\u{27b8}','\u{10d6db}','\u{594b1}','\u{30466}','\u{93b43}'];
_20 = Field::<[char; 2]>(Variant(_28, 3), 1);
_20 = ['\u{d350a}','\u{9335a}'];
_37 = _12 >> Field::<u16>(Variant(_19, 1), 0);
_17 = _22 == _33;
_34.fld1.2 = _16;
_15 = -_29;
_10 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_5 = _3 as i8;
_24 = ['\u{955a}','\u{5a530}','\u{a9205}','\u{3fda0}','\u{7f56c}','\u{b89a8}','\u{22b30}'];
(*_8) = (-27856_i16);
Goto(bb18)
}
bb18 = {
Call(_40 = dump_var(15_usize, 37_usize, Move(_37), 29_usize, Move(_29), 13_usize, Move(_13), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(15_usize, 30_usize, Move(_30), 15_usize, Move(_15), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(15_usize, 3_usize, Move(_3), 35_usize, Move(_35), 21_usize, Move(_21), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_40 = dump_var(15_usize, 7_usize, Move(_7), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i16,mut _2: isize,mut _3: [i64; 5]) -> bool {
mir! {
type RET = bool;
let _4: isize;
let _5: char;
let _6: isize;
let _7: i64;
let _8: [u128; 7];
let _9: Adt51;
let _10: Adt41;
let _11: Adt50;
let _12: [char; 2];
let _13: i64;
let _14: ([char; 2],);
let _15: bool;
let _16: Adt43;
let _17: Adt45;
let _18: f32;
let _19: [i128; 4];
let _20: ([char; 2],);
let _21: char;
let _22: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,));
let _23: (u32, i64);
let _24: char;
let _25: Adt44;
let _26: isize;
let _27: ([char; 2], [char; 7], [char; 2]);
let _28: isize;
let _29: (u32, *const u128, isize);
let _30: Adt52;
let _31: bool;
let _32: u32;
let _33: [u32; 4];
let _34: ([char; 2], [char; 7], [char; 2]);
let _35: [i128; 4];
let _36: *mut i16;
let _37: ();
let _38: ();
{
_2 = (-9223372036854775808_isize) | 115_isize;
_2 = 2748551643541580755_usize as isize;
_4 = _2 & _2;
_4 = _2;
_2 = (-146666896764902254559131537922760249606_i128) as isize;
_2 = _4 * _4;
_1 = 21876_i16;
_2 = '\u{4201f}' as isize;
_4 = 0_usize as isize;
RET = !false;
_5 = '\u{10448}';
_4 = _5 as isize;
_5 = '\u{43444}';
_1 = (-11602_i16) & 11503_i16;
RET = false;
_1 = 5774721035439089836_usize as i16;
_4 = _2 | _2;
_3 = [2036693906289581499_i64,(-2207824906195051294_i64),9184836242345351962_i64,1005566689274694417_i64,4910691989742349880_i64];
RET = true;
_1 = (-29005_i16) + 502_i16;
RET = !true;
Goto(bb1)
}
bb1 = {
_3 = [5595510425031860555_i64,(-4443994153361497556_i64),857654169531013876_i64,6626412315907194949_i64,3584037902965150253_i64];
_2 = !_4;
_3 = [(-5214941823807141436_i64),831637668496776760_i64,8925229033636412744_i64,6234858779238614617_i64,(-4307820322675739405_i64)];
RET = false | true;
_2 = _4;
_4 = _2 >> _1;
RET = _4 <= _2;
_5 = '\u{6413e}';
_6 = _4;
_7 = -(-4316646987414334510_i64);
_4 = -_6;
_7 = (-5847226284086944658_i64);
_5 = '\u{24986}';
_8 = [224073963678322718087401826630745785668_u128,136090767293899757790113006769631207041_u128,260732726953266540047991108760089630552_u128,205208565819515304300573520092515113986_u128,43785054948239039202578947939116787394_u128,213218055389375986118786186381960711353_u128,317876009118871500881761657329626144559_u128];
_2 = !_4;
Goto(bb2)
}
bb2 = {
_10 = Adt41::Variant1 { fld0: 12187875283002005566_u64 };
_1 = -20014_i16;
_8 = [110179953768933404683862276483302893650_u128,285823712840103442264684282611434445104_u128,337806972693688110825618273150654210935_u128,284359475336254342765143222798707891019_u128,73951463050438813000342801264956200768_u128,233581699376572776934919905034155917652_u128,312460709096915749618898006512378462120_u128];
_1 = (-27311_i16) + 3593_i16;
RET = true;
_12 = [_5,_5];
RET = _2 == _6;
RET = !true;
_12 = [_5,_5];
_5 = '\u{27b29}';
_2 = _4 - _6;
RET = false;
_6 = 29478_u16 as isize;
_2 = _4;
_12 = [_5,_5];
place!(Field::<u64>(Variant(_10, 1), 0)) = 19_i8 as u64;
_6 = 61917_u16 as isize;
_6 = _2 >> _1;
RET = !true;
_1 = (-23835_i16);
_6 = _4;
_9 = Adt51::Variant1 { fld0: 103_i8 };
_12 = [_5,_5];
Goto(bb3)
}
bb3 = {
_12 = [_5,_5];
_9 = Adt51::Variant1 { fld0: (-118_i8) };
_6 = _4;
_7 = (-7130750390315462350_i64) >> _4;
_14 = (_12,);
_9 = Adt51::Variant1 { fld0: (-70_i8) };
_14.0 = [_5,_5];
_7 = (-114_i8) as i64;
place!(Field::<i8>(Variant(_9, 1), 0)) = !(-50_i8);
_3 = [_7,_7,_7,_7,_7];
RET = false;
_14 = (_12,);
_7 = 4483279747408690147_usize as i64;
_13 = _7 & _7;
place!(Field::<u64>(Variant(_10, 1), 0)) = 5969574838057843739_u64 - 18351969489433966844_u64;
_4 = Field::<u64>(Variant(_10, 1), 0) as isize;
_3 = [_7,_7,_13,_7,_13];
_13 = _7 + _7;
place!(Field::<i8>(Variant(_9, 1), 0)) = (-49_i8) | (-6_i8);
_13 = _1 as i64;
Goto(bb4)
}
bb4 = {
_17.fld1.2 = _14.0;
_14.0 = [_5,_5];
_17.fld1.2 = [_5,_5];
_17.fld1.1 = [_5,_5,_5,_5,_5,_5,_5];
_14 = (_12,);
_2 = _6 + _4;
_4 = _6 - _6;
_8 = [40155790993126264338854211246366307215_u128,306616503621163663019456425304375744212_u128,152640679025745647952862474102155866306_u128,329679116655144549078295594114296133289_u128,269617620412227792442581631218866057506_u128,87611602039779450327643162870627111043_u128,211541509470847012003997487632359063319_u128];
_17.fld1.0 = [_5,_5];
_13 = _7;
_2 = -_6;
_17.fld2 = _1 as isize;
SetDiscriminant(_9, 2);
_3 = [_7,_13,_13,_13,_7];
_5 = '\u{9dfba}';
_5 = '\u{64b81}';
RET = false;
_15 = Field::<u64>(Variant(_10, 1), 0) <= Field::<u64>(Variant(_10, 1), 0);
SetDiscriminant(_10, 1);
_17.fld0 = [114_u8,156_u8,245_u8,76_u8,211_u8,50_u8,193_u8];
_8 = [225620160481206920204381799984238717735_u128,20264305153846295449819125549296934592_u128,30231634096567171500918051972131289490_u128,279009234951620970633019571782422672117_u128,98879110016752960607027999391564672116_u128,257551863581250713098473201655706300584_u128,31549690663021700765887824993305565207_u128];
match _1 {
0 => bb5,
340282366920938463463374607431768187621 => bb7,
_ => bb6
}
}
bb5 = {
_12 = [_5,_5];
_9 = Adt51::Variant1 { fld0: (-118_i8) };
_6 = _4;
_7 = (-7130750390315462350_i64) >> _4;
_14 = (_12,);
_9 = Adt51::Variant1 { fld0: (-70_i8) };
_14.0 = [_5,_5];
_7 = (-114_i8) as i64;
place!(Field::<i8>(Variant(_9, 1), 0)) = !(-50_i8);
_3 = [_7,_7,_7,_7,_7];
RET = false;
_14 = (_12,);
_7 = 4483279747408690147_usize as i64;
_13 = _7 & _7;
place!(Field::<u64>(Variant(_10, 1), 0)) = 5969574838057843739_u64 - 18351969489433966844_u64;
_4 = Field::<u64>(Variant(_10, 1), 0) as isize;
_3 = [_7,_7,_13,_7,_13];
_13 = _7 + _7;
place!(Field::<i8>(Variant(_9, 1), 0)) = (-49_i8) | (-6_i8);
_13 = _1 as i64;
Goto(bb4)
}
bb6 = {
_10 = Adt41::Variant1 { fld0: 12187875283002005566_u64 };
_1 = -20014_i16;
_8 = [110179953768933404683862276483302893650_u128,285823712840103442264684282611434445104_u128,337806972693688110825618273150654210935_u128,284359475336254342765143222798707891019_u128,73951463050438813000342801264956200768_u128,233581699376572776934919905034155917652_u128,312460709096915749618898006512378462120_u128];
_1 = (-27311_i16) + 3593_i16;
RET = true;
_12 = [_5,_5];
RET = _2 == _6;
RET = !true;
_12 = [_5,_5];
_5 = '\u{27b29}';
_2 = _4 - _6;
RET = false;
_6 = 29478_u16 as isize;
_2 = _4;
_12 = [_5,_5];
place!(Field::<u64>(Variant(_10, 1), 0)) = 19_i8 as u64;
_6 = 61917_u16 as isize;
_6 = _2 >> _1;
RET = !true;
_1 = (-23835_i16);
_6 = _4;
_9 = Adt51::Variant1 { fld0: 103_i8 };
_12 = [_5,_5];
Goto(bb3)
}
bb7 = {
_19 = [(-60351475535065603795394172868795874972_i128),(-35513736350901121786947443057235269204_i128),(-157211160634536818696810984330776384882_i128),(-22292732134466443439979182241969278288_i128)];
place!(Field::<[char; 2]>(Variant(_9, 2), 0)) = [_5,_5];
_14 = (_17.fld1.0,);
_20 = _14;
_21 = _5;
_17.fld2 = _4;
_12 = [_5,_21];
_22.1 = core::ptr::addr_of_mut!(_22.5.0);
place!(Field::<char>(Variant(_9, 2), 1)) = _21;
_20 = (_17.fld1.0,);
_23.1 = _21 as i64;
_18 = 107879498544310392309995082706022866036_u128 as f32;
place!(Field::<[char; 2]>(Variant(_9, 2), 0)) = [_21,_21];
_3 = [_13,_23.1,_23.1,_13,_7];
_12 = [_21,_21];
_23 = (4166892622_u32, _13);
_23 = (2488316762_u32, _13);
_22.3 = 4_u8 - 149_u8;
_5 = _21;
_10 = Adt41::Variant1 { fld0: 10995601209468865245_u64 };
_17.fld1.1 = [_5,_21,_5,_21,Field::<char>(Variant(_9, 2), 1),_21,_21];
RET = _15 & _15;
_5 = Field::<char>(Variant(_9, 2), 1);
_23 = (919981815_u32, _13);
_17.fld1.2 = [_21,_5];
_18 = _22.3 as f32;
match _23.0 {
0 => bb8,
1 => bb9,
2 => bb10,
919981815 => bb12,
_ => bb11
}
}
bb8 = {
_10 = Adt41::Variant1 { fld0: 12187875283002005566_u64 };
_1 = -20014_i16;
_8 = [110179953768933404683862276483302893650_u128,285823712840103442264684282611434445104_u128,337806972693688110825618273150654210935_u128,284359475336254342765143222798707891019_u128,73951463050438813000342801264956200768_u128,233581699376572776934919905034155917652_u128,312460709096915749618898006512378462120_u128];
_1 = (-27311_i16) + 3593_i16;
RET = true;
_12 = [_5,_5];
RET = _2 == _6;
RET = !true;
_12 = [_5,_5];
_5 = '\u{27b29}';
_2 = _4 - _6;
RET = false;
_6 = 29478_u16 as isize;
_2 = _4;
_12 = [_5,_5];
place!(Field::<u64>(Variant(_10, 1), 0)) = 19_i8 as u64;
_6 = 61917_u16 as isize;
_6 = _2 >> _1;
RET = !true;
_1 = (-23835_i16);
_6 = _4;
_9 = Adt51::Variant1 { fld0: 103_i8 };
_12 = [_5,_5];
Goto(bb3)
}
bb9 = {
_12 = [_5,_5];
_9 = Adt51::Variant1 { fld0: (-118_i8) };
_6 = _4;
_7 = (-7130750390315462350_i64) >> _4;
_14 = (_12,);
_9 = Adt51::Variant1 { fld0: (-70_i8) };
_14.0 = [_5,_5];
_7 = (-114_i8) as i64;
place!(Field::<i8>(Variant(_9, 1), 0)) = !(-50_i8);
_3 = [_7,_7,_7,_7,_7];
RET = false;
_14 = (_12,);
_7 = 4483279747408690147_usize as i64;
_13 = _7 & _7;
place!(Field::<u64>(Variant(_10, 1), 0)) = 5969574838057843739_u64 - 18351969489433966844_u64;
_4 = Field::<u64>(Variant(_10, 1), 0) as isize;
_3 = [_7,_7,_13,_7,_13];
_13 = _7 + _7;
place!(Field::<i8>(Variant(_9, 1), 0)) = (-49_i8) | (-6_i8);
_13 = _1 as i64;
Goto(bb4)
}
bb10 = {
_17.fld1.2 = _14.0;
_14.0 = [_5,_5];
_17.fld1.2 = [_5,_5];
_17.fld1.1 = [_5,_5,_5,_5,_5,_5,_5];
_14 = (_12,);
_2 = _6 + _4;
_4 = _6 - _6;
_8 = [40155790993126264338854211246366307215_u128,306616503621163663019456425304375744212_u128,152640679025745647952862474102155866306_u128,329679116655144549078295594114296133289_u128,269617620412227792442581631218866057506_u128,87611602039779450327643162870627111043_u128,211541509470847012003997487632359063319_u128];
_17.fld1.0 = [_5,_5];
_13 = _7;
_2 = -_6;
_17.fld2 = _1 as isize;
SetDiscriminant(_9, 2);
_3 = [_7,_13,_13,_13,_7];
_5 = '\u{9dfba}';
_5 = '\u{64b81}';
RET = false;
_15 = Field::<u64>(Variant(_10, 1), 0) <= Field::<u64>(Variant(_10, 1), 0);
SetDiscriminant(_10, 1);
_17.fld0 = [114_u8,156_u8,245_u8,76_u8,211_u8,50_u8,193_u8];
_8 = [225620160481206920204381799984238717735_u128,20264305153846295449819125549296934592_u128,30231634096567171500918051972131289490_u128,279009234951620970633019571782422672117_u128,98879110016752960607027999391564672116_u128,257551863581250713098473201655706300584_u128,31549690663021700765887824993305565207_u128];
match _1 {
0 => bb5,
340282366920938463463374607431768187621 => bb7,
_ => bb6
}
}
bb11 = {
_10 = Adt41::Variant1 { fld0: 12187875283002005566_u64 };
_1 = -20014_i16;
_8 = [110179953768933404683862276483302893650_u128,285823712840103442264684282611434445104_u128,337806972693688110825618273150654210935_u128,284359475336254342765143222798707891019_u128,73951463050438813000342801264956200768_u128,233581699376572776934919905034155917652_u128,312460709096915749618898006512378462120_u128];
_1 = (-27311_i16) + 3593_i16;
RET = true;
_12 = [_5,_5];
RET = _2 == _6;
RET = !true;
_12 = [_5,_5];
_5 = '\u{27b29}';
_2 = _4 - _6;
RET = false;
_6 = 29478_u16 as isize;
_2 = _4;
_12 = [_5,_5];
place!(Field::<u64>(Variant(_10, 1), 0)) = 19_i8 as u64;
_6 = 61917_u16 as isize;
_6 = _2 >> _1;
RET = !true;
_1 = (-23835_i16);
_6 = _4;
_9 = Adt51::Variant1 { fld0: 103_i8 };
_12 = [_5,_5];
Goto(bb3)
}
bb12 = {
_4 = _17.fld2;
_15 = !RET;
_7 = !_13;
_1 = 18240_i16 ^ (-20298_i16);
_25.fld0 = [_23.0,_23.0,_23.0,_23.0];
_18 = _23.0 as f32;
_22.0 = !_17.fld2;
_27.0 = [_21,Field::<char>(Variant(_9, 2), 1)];
RET = !_15;
place!(Field::<char>(Variant(_9, 2), 1)) = _5;
place!(Field::<[char; 2]>(Variant(_9, 2), 0)) = [_5,_5];
_12 = _17.fld1.0;
_26 = !_4;
_27.1 = [Field::<char>(Variant(_9, 2), 1),_21,Field::<char>(Variant(_9, 2), 1),_21,Field::<char>(Variant(_9, 2), 1),_5,Field::<char>(Variant(_9, 2), 1)];
_24 = _5;
_12 = [Field::<char>(Variant(_9, 2), 1),_21];
_18 = 27354_u16 as f32;
_2 = !_26;
_21 = _24;
_10 = Adt41::Variant2 { fld0: RET,fld1: _19 };
Goto(bb13)
}
bb13 = {
_5 = _24;
_23 = (2085749187_u32, _13);
_22.3 = 18126358571741494244_u64 as u8;
SetDiscriminant(_10, 2);
place!(Field::<*mut [u128; 7]>(Variant(_9, 2), 2)) = core::ptr::addr_of_mut!(_22.4);
_29.2 = _17.fld2 >> _26;
_31 = !_15;
_23 = (133803499_u32, _13);
_23.1 = -_7;
_28 = _29.2 & _4;
_22.4 = [199834543044467036771243893405822341561_u128,290607647401030634350185339518346314879_u128,161298610973820750920023347336221346733_u128,204269383189955086455159257788296894448_u128,33463269584822029023597883822554787926_u128,17696826643714516553408439404560351297_u128,207885133843539970981341589443720715023_u128];
place!(Field::<bool>(Variant(_10, 2), 0)) = _31 | _15;
_27.2 = _14.0;
place!(Field::<char>(Variant(_9, 2), 1)) = _24;
_27.0 = [_5,Field::<char>(Variant(_9, 2), 1)];
_2 = _28 + _28;
_26 = _7 as isize;
_17.fld0 = [_22.3,_22.3,_22.3,_22.3,_22.3,_22.3,_22.3];
_12 = _27.2;
_34 = _17.fld1;
_34.2 = _27.2;
_27.0 = _17.fld1.0;
_35 = _19;
match _23.0 {
0 => bb14,
133803499 => bb16,
_ => bb15
}
}
bb14 = {
_17.fld1.2 = _14.0;
_14.0 = [_5,_5];
_17.fld1.2 = [_5,_5];
_17.fld1.1 = [_5,_5,_5,_5,_5,_5,_5];
_14 = (_12,);
_2 = _6 + _4;
_4 = _6 - _6;
_8 = [40155790993126264338854211246366307215_u128,306616503621163663019456425304375744212_u128,152640679025745647952862474102155866306_u128,329679116655144549078295594114296133289_u128,269617620412227792442581631218866057506_u128,87611602039779450327643162870627111043_u128,211541509470847012003997487632359063319_u128];
_17.fld1.0 = [_5,_5];
_13 = _7;
_2 = -_6;
_17.fld2 = _1 as isize;
SetDiscriminant(_9, 2);
_3 = [_7,_13,_13,_13,_7];
_5 = '\u{9dfba}';
_5 = '\u{64b81}';
RET = false;
_15 = Field::<u64>(Variant(_10, 1), 0) <= Field::<u64>(Variant(_10, 1), 0);
SetDiscriminant(_10, 1);
_17.fld0 = [114_u8,156_u8,245_u8,76_u8,211_u8,50_u8,193_u8];
_8 = [225620160481206920204381799984238717735_u128,20264305153846295449819125549296934592_u128,30231634096567171500918051972131289490_u128,279009234951620970633019571782422672117_u128,98879110016752960607027999391564672116_u128,257551863581250713098473201655706300584_u128,31549690663021700765887824993305565207_u128];
match _1 {
0 => bb5,
340282366920938463463374607431768187621 => bb7,
_ => bb6
}
}
bb15 = {
_12 = [_5,_5];
_9 = Adt51::Variant1 { fld0: (-118_i8) };
_6 = _4;
_7 = (-7130750390315462350_i64) >> _4;
_14 = (_12,);
_9 = Adt51::Variant1 { fld0: (-70_i8) };
_14.0 = [_5,_5];
_7 = (-114_i8) as i64;
place!(Field::<i8>(Variant(_9, 1), 0)) = !(-50_i8);
_3 = [_7,_7,_7,_7,_7];
RET = false;
_14 = (_12,);
_7 = 4483279747408690147_usize as i64;
_13 = _7 & _7;
place!(Field::<u64>(Variant(_10, 1), 0)) = 5969574838057843739_u64 - 18351969489433966844_u64;
_4 = Field::<u64>(Variant(_10, 1), 0) as isize;
_3 = [_7,_7,_13,_7,_13];
_13 = _7 + _7;
place!(Field::<i8>(Variant(_9, 1), 0)) = (-49_i8) | (-6_i8);
_13 = _1 as i64;
Goto(bb4)
}
bb16 = {
_17.fld1.0 = _27.2;
place!(Field::<*mut [u128; 7]>(Variant(_9, 2), 2)) = core::ptr::addr_of_mut!(_8);
_28 = _17.fld2 ^ _2;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(16_usize, 15_usize, Move(_15), 12_usize, Move(_12), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(16_usize, 14_usize, Move(_14), 28_usize, Move(_28), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(16_usize, 21_usize, Move(_21), 35_usize, Move(_35), 31_usize, Move(_31), 38_usize, _38), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [char; 7],mut _2: i32,mut _3: i16,mut _4: bool,mut _5: i16,mut _6: (u32, i64),mut _7: isize,mut _8: i64,mut _9: [char; 7],mut _10: [i64; 5]) -> *mut i16 {
mir! {
type RET = *mut i16;
let _11: [i128; 4];
let _12: isize;
let _13: [u128; 7];
let _14: u8;
let _15: Adt44;
let _16: usize;
let _17: Adt56;
let _18: [i64; 5];
let _19: (u32, *const u128, isize);
let _20: char;
let _21: u16;
let _22: [u8; 7];
let _23: ([char; 2],);
let _24: char;
let _25: Adt56;
let _26: usize;
let _27: Adt44;
let _28: [i128; 4];
let _29: f32;
let _30: char;
let _31: *const i32;
let _32: f32;
let _33: [i64; 5];
let _34: ();
let _35: ();
{
_6.1 = _8 ^ _8;
_11 = [116489818817123909340735788352137840962_i128,(-103006526070622271479990832867283406415_i128),13367733463006032311996283353066232105_i128,121621848277623914780800951673425795874_i128];
_9 = ['\u{d860f}','\u{576a7}','\u{efe24}','\u{81a49}','\u{a2d90}','\u{cc2ba}','\u{2b76c}'];
_5 = -_3;
_12 = _4 as isize;
_13 = [117493858278235490192444237456652175732_u128,117444037346275285035092034374856419211_u128,147551947725741516885241819291152831472_u128,188078266849188021558844414031714452047_u128,207745550029217578438198816042878741340_u128,97201165004404589866235267148824355545_u128,65913088147522189297029639079850017864_u128];
_6.0 = !333034755_u32;
_14 = _12 as u8;
_7 = _12 + _12;
_15.fld0 = [_6.0,_6.0,_6.0,_6.0];
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of_mut!(_3);
_7 = !_12;
_6 = (582906131_u32, _8);
RET = core::ptr::addr_of_mut!(_3);
_12 = _7 << _14;
_14 = 87_u8;
_12 = _4 as isize;
_16 = 5551341324994415206_usize;
_5 = !(*RET);
(*RET) = _5;
(*RET) = _5 >> _12;
match _2 {
0 => bb2,
340282366920938463463374607430036568821 => bb4,
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
(*RET) = _4 as i16;
(*RET) = _5 & _5;
_15.fld0 = [_6.0,_6.0,_6.0,_6.0];
_6 = (3656575220_u32, _8);
_2 = (-1750391738_i32) + (-1794894604_i32);
_11 = [(-132335663083034255332769906099108655579_i128),(-82182146624093415498096639389168402765_i128),(-76410369981488823759925375422720169513_i128),(-164019934767961185238720757940105955253_i128)];
_11 = [(-168758957325795678505214855298393529449_i128),17923308994408013828910202371458346703_i128,94669187969606845198161915349132836095_i128,61518902278413572967471956387655697755_i128];
Goto(bb5)
}
bb5 = {
_15.fld0 = [_6.0,_6.0,_6.0,_6.0];
_2 = (-1273998066_i32);
_6.1 = !_8;
_7 = !_12;
_19.2 = _12 | _12;
_18 = [_8,_6.1,_6.1,_6.1,_6.1];
_10 = _18;
_5 = _6.0 as i16;
_7 = _19.2 ^ _12;
_6.1 = _2 as i64;
_13 = [159343632126296180623818751809690147725_u128,236390455066018158381893056984366832563_u128,47650307541830658275176652927803833869_u128,289734627387694723050005280240036696423_u128,81338669183343597845171801406378741575_u128,63413163764039618988042496263861466995_u128,285451715062818266451453334483543393068_u128];
_11 = [(-89557402639931966280270920759352400471_i128),151175943787308274512910441008466375823_i128,94026552199606103437354255588805299941_i128,47123905539386125450974399351497268503_i128];
_14 = 81_u8 >> _12;
_1 = ['\u{6af95}','\u{f2331}','\u{ca9b3}','\u{ae25}','\u{61d04}','\u{10e04b}','\u{a5932}'];
_19.0 = _4 as u32;
(*RET) = !_5;
_10 = [_8,_6.1,_6.1,_8,_8];
_11 = [(-104444460296902031342797277304910886183_i128),149697839749649730956047317302321948574_i128,152825094365285860702175844920008008244_i128,14986630133758109555458245204859898064_i128];
RET = core::ptr::addr_of_mut!(_3);
_15.fld0 = [_19.0,_19.0,_6.0,_19.0];
_13 = [50263881295421837939235872328359842419_u128,92920782860127739835373003035581201922_u128,266019830885957058325323832031329974874_u128,193003309416871404117210387482963873234_u128,199571616636578098505883110013714445461_u128,176596399992252700370938086545965626382_u128,79987027544719643277099944940246684000_u128];
_8 = -_6.1;
_1 = ['\u{b32fc}','\u{a1ab5}','\u{d1a46}','\u{3f63c}','\u{defaa}','\u{8f606}','\u{29da0}'];
_6.1 = _8 << _7;
_6.1 = _2 as i64;
Goto(bb6)
}
bb6 = {
_10 = [_6.1,_8,_6.1,_8,_8];
Goto(bb7)
}
bb7 = {
_19.2 = -_7;
_20 = '\u{4990f}';
_6 = (_19.0, _8);
_15.fld0 = [_19.0,_6.0,_6.0,_19.0];
_10 = _18;
_6 = (_19.0, _8);
_6.0 = !_19.0;
_4 = (*RET) > (*RET);
_3 = _5;
_19.0 = _6.0;
_22 = [_14,_14,_14,_14,_14,_14,_14];
(*RET) = _5;
(*RET) = _5 | _5;
_1 = _9;
Goto(bb8)
}
bb8 = {
(*RET) = _5;
Goto(bb9)
}
bb9 = {
_10 = [_8,_8,_8,_6.1,_6.1];
_14 = 10_u8 + 250_u8;
_24 = _20;
_14 = !163_u8;
match _2 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
340282366920938463463374607430494213390 => bb10,
_ => bb6
}
}
bb10 = {
(*RET) = 39297_u16 as i16;
Goto(bb11)
}
bb11 = {
_11 = [114254048544999077159098364560605888444_i128,(-69867465208765661404503140400178830795_i128),(-166917266801292420737313075590316201855_i128),85102045538889276489298265521074631025_i128];
_29 = _8 as f32;
_18 = _10;
_19.2 = !_7;
_6.0 = !_19.0;
_13 = [329892484508709023497051381537619881379_u128,195950591742827811786500486654130322136_u128,10154906684540951622238718783993880410_u128,226335280592625688982872580477071846183_u128,334841321138167636656022153473236166295_u128,339958467540824262566205618796874821843_u128,29280304629982832141915864574137295444_u128];
_27.fld0 = _15.fld0;
_15 = Move(_27);
_12 = _7 + _7;
_14 = _2 as u8;
_8 = _6.1 * _6.1;
_22 = [_14,_14,_14,_14,_14,_14,_14];
_1 = _9;
_28 = [(-71572655273688820215785953110724257626_i128),28422538008254360909231839105168102032_i128,(-160600794067895752727987954107067887186_i128),88432469524718752665288417299074556903_i128];
_19.0 = _24 as u32;
_26 = !_16;
_12 = -_19.2;
_31 = core::ptr::addr_of!(_2);
_11 = [100610210558169107499486109504487896660_i128,(-88933231264242008967561609382064490168_i128),131238454899178899107469061084392132796_i128,(-70858358137327807840920152954484900592_i128)];
_18 = _10;
(*_31) = -(-1096574996_i32);
match _16 {
0 => bb2,
1 => bb12,
2 => bb13,
5551341324994415206 => bb15,
_ => bb14
}
}
bb12 = {
_10 = [_6.1,_8,_6.1,_8,_8];
Goto(bb7)
}
bb13 = {
_10 = [_8,_8,_8,_6.1,_6.1];
_14 = 10_u8 + 250_u8;
_24 = _20;
_14 = !163_u8;
match _2 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
340282366920938463463374607430494213390 => bb10,
_ => bb6
}
}
bb14 = {
(*RET) = _4 as i16;
(*RET) = _5 & _5;
_15.fld0 = [_6.0,_6.0,_6.0,_6.0];
_6 = (3656575220_u32, _8);
_2 = (-1750391738_i32) + (-1794894604_i32);
_11 = [(-132335663083034255332769906099108655579_i128),(-82182146624093415498096639389168402765_i128),(-76410369981488823759925375422720169513_i128),(-164019934767961185238720757940105955253_i128)];
_11 = [(-168758957325795678505214855298393529449_i128),17923308994408013828910202371458346703_i128,94669187969606845198161915349132836095_i128,61518902278413572967471956387655697755_i128];
Goto(bb5)
}
bb15 = {
_10 = [_8,_6.1,_8,_8,_8];
(*RET) = _5;
_5 = !(*RET);
_23.0 = [_20,_20];
_14 = _7 as u8;
_22 = [_14,_14,_14,_14,_14,_14,_14];
(*RET) = -_5;
_20 = _24;
_6 = (_19.0, _8);
_22 = [_14,_14,_14,_14,_14,_14,_14];
_1 = _9;
_31 = core::ptr::addr_of!(_2);
_32 = -_29;
_29 = _32;
_9 = [_20,_20,_20,_24,_20,_24,_24];
RET = core::ptr::addr_of_mut!((*RET));
_11 = _28;
_31 = core::ptr::addr_of!((*_31));
_15.fld0 = [_6.0,_6.0,_19.0,_6.0];
_24 = _20;
_31 = core::ptr::addr_of!((*_31));
_27 = Adt44 { fld0: _15.fld0 };
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(17_usize, 28_usize, Move(_28), 4_usize, Move(_4), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(17_usize, 10_usize, Move(_10), 22_usize, Move(_22), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(17_usize, 12_usize, Move(_12), 18_usize, Move(_18), 14_usize, Move(_14), 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool,mut _2: [char; 7]) -> Adt52 {
mir! {
type RET = Adt52;
let _3: i64;
let _4: u16;
let _5: *mut u16;
let _6: *mut [u128; 7];
let _7: [i64; 5];
let _8: Adt44;
let _9: [i64; 5];
let _10: (isize, i32, *const i32, i128);
let _11: f32;
let _12: ([char; 2],);
let _13: (u32, i64);
let _14: u128;
let _15: i16;
let _16: u64;
let _17: [char; 7];
let _18: u16;
let _19: u8;
let _20: char;
let _21: Adt55;
let _22: [u32; 4];
let _23: isize;
let _24: isize;
let _25: [u128; 7];
let _26: Adt44;
let _27: isize;
let _28: u16;
let _29: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,));
let _30: usize;
let _31: [u8; 7];
let _32: u32;
let _33: *const u128;
let _34: bool;
let _35: f64;
let _36: isize;
let _37: [u32; 4];
let _38: char;
let _39: i8;
let _40: u64;
let _41: ();
let _42: ();
{
_2 = ['\u{732b2}','\u{d4f3e}','\u{10c5f1}','\u{d5f0a}','\u{8549f}','\u{102f86}','\u{108253}'];
_2 = ['\u{2a3ef}','\u{5cdbd}','\u{3b555}','\u{a0911}','\u{37ad1}','\u{293f1}','\u{74ae1}'];
_2 = ['\u{43e87}','\u{4209c}','\u{eed7d}','\u{8a956}','\u{f3441}','\u{4eb}','\u{b3330}'];
RET = Adt52::Variant1 { fld0: 8657_u16 };
place!(Field::<u16>(Variant(RET, 1), 0)) = 97_i8 as u16;
Call(place!(Field::<u16>(Variant(RET, 1), 0)) = core::intrinsics::bswap(3052_u16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !true;
_4 = 15646_i16 as u16;
place!(Field::<u16>(Variant(RET, 1), 0)) = _4 & _4;
RET = Adt52::Variant1 { fld0: _4 };
_3 = Field::<u16>(Variant(RET, 1), 0) as i64;
place!(Field::<u16>(Variant(RET, 1), 0)) = _4 * _4;
_1 = false;
_5 = core::ptr::addr_of_mut!(_4);
(*_5) = Field::<u16>(Variant(RET, 1), 0);
Goto(bb2)
}
bb2 = {
_1 = _3 >= _3;
_2 = ['\u{14a26}','\u{72b59}','\u{b58cb}','\u{94cb2}','\u{8c27c}','\u{e0b2a}','\u{11575}'];
_2 = ['\u{18e16}','\u{100a0f}','\u{68355}','\u{d4d2d}','\u{d76ee}','\u{eba5d}','\u{bbf2e}'];
place!(Field::<u16>(Variant(RET, 1), 0)) = !_4;
_7 = [_3,_3,_3,_3,_3];
(*_5) = Field::<u16>(Variant(RET, 1), 0) | Field::<u16>(Variant(RET, 1), 0);
place!(Field::<u16>(Variant(RET, 1), 0)) = (*_5) << (*_5);
_4 = Field::<u16>(Variant(RET, 1), 0) ^ Field::<u16>(Variant(RET, 1), 0);
(*_5) = !Field::<u16>(Variant(RET, 1), 0);
RET = Adt52::Variant1 { fld0: (*_5) };
_4 = 1040822652_i32 as u16;
(*_5) = 15112_i16 as u16;
_3 = 6025876982746657340_i64;
_5 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(RET, 1), 0)));
_8.fld0 = [884272570_u32,3754450634_u32,1564866949_u32,2535464318_u32];
_9 = [_3,_3,_3,_3,_3];
RET = Adt52::Variant1 { fld0: _4 };
_1 = !false;
place!(Field::<u16>(Variant(RET, 1), 0)) = 15614336349626517545_u64 as u16;
place!(Field::<u16>(Variant(RET, 1), 0)) = !_4;
_10.0 = (-67_i8) as isize;
_10.3 = 73715153882348920989207949744960028463_i128;
_10.0 = !(-9223372036854775808_isize);
_10.1 = (-482321951_i32) * (-930015619_i32);
_2 = ['\u{afe0b}','\u{bd436}','\u{46435}','\u{39173}','\u{1b054}','\u{401c}','\u{ba38f}'];
_4 = !Field::<u16>(Variant(RET, 1), 0);
_10.1 = -(-216044597_i32);
_10.1 = !1816820477_i32;
Goto(bb3)
}
bb3 = {
_8.fld0 = [4054660174_u32,2010685336_u32,3439777361_u32,1906470728_u32];
_10.0 = 69_isize;
_4 = !Field::<u16>(Variant(RET, 1), 0);
SetDiscriminant(RET, 0);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 2103752200_u32;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0 ^ _10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 1574315585_u32 + 1542674931_u32;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = -_10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 2187120400_u32;
_8.fld0 = [Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0];
_10.2 = core::ptr::addr_of!(_10.1);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0 & _10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0;
_10.0 = Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0 as isize;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 450370092_u32 | 518213797_u32;
_1 = _4 <= _4;
_11 = _10.1 as f32;
RET = Adt52::Variant1 { fld0: _4 };
_11 = 254_u8 as f32;
_13 = (3915739357_u32, _3);
RET = Adt52::Variant1 { fld0: _4 };
match _13.0 {
3915739357 => bb5,
_ => bb4
}
}
bb4 = {
_1 = !true;
_4 = 15646_i16 as u16;
place!(Field::<u16>(Variant(RET, 1), 0)) = _4 & _4;
RET = Adt52::Variant1 { fld0: _4 };
_3 = Field::<u16>(Variant(RET, 1), 0) as i64;
place!(Field::<u16>(Variant(RET, 1), 0)) = _4 * _4;
_1 = false;
_5 = core::ptr::addr_of_mut!(_4);
(*_5) = Field::<u16>(Variant(RET, 1), 0);
Goto(bb2)
}
bb5 = {
_14 = !148523136881251264462830526374131999168_u128;
place!(Field::<u16>(Variant(RET, 1), 0)) = _14 as u16;
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_11 = _4 as f32;
Goto(bb6)
}
bb6 = {
place!(Field::<u16>(Variant(RET, 1), 0)) = '\u{2207a}' as u16;
_10.1 = (-949771202_i32) & 1726588512_i32;
_12.0 = ['\u{6a697}','\u{1a14e}'];
_7 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_10.3 = 93652846802163225442833358865585673781_i128 | 151073903527331030819972112497993146220_i128;
_10.3 = (-59865240281213116911672856121378909252_i128) - 26191878786580214373569802403771858635_i128;
_19 = 49_u8;
_17 = _2;
_18 = _10.3 as u16;
_4 = _3 as u16;
_1 = !true;
SetDiscriminant(RET, 1);
_3 = -_13.1;
place!(Field::<u16>(Variant(RET, 1), 0)) = _11 as u16;
_21.fld4.0 = _12.0;
_5 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(RET, 1), 0)));
_7 = [_3,_3,_13.1,_13.1,_3];
_1 = true;
_13 = (1050073490_u32, _3);
_6 = core::ptr::addr_of_mut!(_21.fld1);
Call(_4 = core::intrinsics::transmute((*_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10.0 = 9223372036854775807_isize;
SetDiscriminant(RET, 0);
_22 = _8.fld0;
_21.fld4.0 = ['\u{f5b8a}','\u{2dd77}'];
(*_6) = [_14,_14,_14,_14,_14,_14,_14];
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).1 = core::ptr::addr_of!(_14);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = _13.0;
_21.fld4.0 = _12.0;
(*_6) = [_14,_14,_14,_14,_14,_14,_14];
_6 = core::ptr::addr_of_mut!(_21.fld1);
_1 = true;
_2 = ['\u{5adb6}','\u{c4841}','\u{a6a59}','\u{1005ae}','\u{c13af}','\u{ccb93}','\u{30105}'];
_18 = _13.1 as u16;
_16 = _10.3 as u64;
_21.fld4 = (_12.0,);
_2 = ['\u{fbcb4}','\u{c51b0}','\u{b31dc}','\u{1020e6}','\u{bbeaf}','\u{c58ea}','\u{d85b7}'];
_15 = (-3969_i16);
_23 = -_10.0;
_21.fld3 = Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).1;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)) = (_13.0, _21.fld3, _23);
Goto(bb8)
}
bb8 = {
_4 = _10.1 as u16;
_28 = _4 << _13.1;
_21.fld0 = ['\u{1a8fa}','\u{7f2be}'];
_16 = 18184975795141482381_u64 * 15386119706061987367_u64;
_10.3 = !128868909659021469855788095452570595292_i128;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = -_10.0;
_10.2 = core::ptr::addr_of!(_10.1);
_24 = -Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).2;
_16 = 12932518029264671697_u64 ^ 1818049263340434384_u64;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _23;
_19 = !165_u8;
_4 = !_18;
_25 = [_14,_14,_14,_14,_14,_14,_14];
_7 = [_3,_3,_3,_13.1,_3];
_4 = !_28;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = _13.0;
_8.fld0 = [_13.0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,_13.0,_13.0];
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)) = (_13.0, _21.fld3, _23);
_13.0 = Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0;
_13 = (Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0, _3);
_26.fld0 = [Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,_13.0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0];
_29.2 = -_10.0;
_30 = !14221770547982291465_usize;
match _10.0 {
0 => bb3,
1 => bb4,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
9223372036854775807 => bb14,
_ => bb13
}
}
bb9 = {
_10.0 = 9223372036854775807_isize;
SetDiscriminant(RET, 0);
_22 = _8.fld0;
_21.fld4.0 = ['\u{f5b8a}','\u{2dd77}'];
(*_6) = [_14,_14,_14,_14,_14,_14,_14];
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).1 = core::ptr::addr_of!(_14);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = _13.0;
_21.fld4.0 = _12.0;
(*_6) = [_14,_14,_14,_14,_14,_14,_14];
_6 = core::ptr::addr_of_mut!(_21.fld1);
_1 = true;
_2 = ['\u{5adb6}','\u{c4841}','\u{a6a59}','\u{1005ae}','\u{c13af}','\u{ccb93}','\u{30105}'];
_18 = _13.1 as u16;
_16 = _10.3 as u64;
_21.fld4 = (_12.0,);
_2 = ['\u{fbcb4}','\u{c51b0}','\u{b31dc}','\u{1020e6}','\u{bbeaf}','\u{c58ea}','\u{d85b7}'];
_15 = (-3969_i16);
_23 = -_10.0;
_21.fld3 = Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).1;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)) = (_13.0, _21.fld3, _23);
Goto(bb8)
}
bb10 = {
place!(Field::<u16>(Variant(RET, 1), 0)) = '\u{2207a}' as u16;
_10.1 = (-949771202_i32) & 1726588512_i32;
_12.0 = ['\u{6a697}','\u{1a14e}'];
_7 = [_13.1,_13.1,_13.1,_13.1,_13.1];
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_10.3 = 93652846802163225442833358865585673781_i128 | 151073903527331030819972112497993146220_i128;
_10.3 = (-59865240281213116911672856121378909252_i128) - 26191878786580214373569802403771858635_i128;
_19 = 49_u8;
_17 = _2;
_18 = _10.3 as u16;
_4 = _3 as u16;
_1 = !true;
SetDiscriminant(RET, 1);
_3 = -_13.1;
place!(Field::<u16>(Variant(RET, 1), 0)) = _11 as u16;
_21.fld4.0 = _12.0;
_5 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(RET, 1), 0)));
_7 = [_3,_3,_13.1,_13.1,_3];
_1 = true;
_13 = (1050073490_u32, _3);
_6 = core::ptr::addr_of_mut!(_21.fld1);
Call(_4 = core::intrinsics::transmute((*_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_14 = !148523136881251264462830526374131999168_u128;
place!(Field::<u16>(Variant(RET, 1), 0)) = _14 as u16;
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_8.fld0 = [_13.0,_13.0,_13.0,_13.0];
_11 = _4 as f32;
Goto(bb6)
}
bb12 = {
_1 = _3 >= _3;
_2 = ['\u{14a26}','\u{72b59}','\u{b58cb}','\u{94cb2}','\u{8c27c}','\u{e0b2a}','\u{11575}'];
_2 = ['\u{18e16}','\u{100a0f}','\u{68355}','\u{d4d2d}','\u{d76ee}','\u{eba5d}','\u{bbf2e}'];
place!(Field::<u16>(Variant(RET, 1), 0)) = !_4;
_7 = [_3,_3,_3,_3,_3];
(*_5) = Field::<u16>(Variant(RET, 1), 0) | Field::<u16>(Variant(RET, 1), 0);
place!(Field::<u16>(Variant(RET, 1), 0)) = (*_5) << (*_5);
_4 = Field::<u16>(Variant(RET, 1), 0) ^ Field::<u16>(Variant(RET, 1), 0);
(*_5) = !Field::<u16>(Variant(RET, 1), 0);
RET = Adt52::Variant1 { fld0: (*_5) };
_4 = 1040822652_i32 as u16;
(*_5) = 15112_i16 as u16;
_3 = 6025876982746657340_i64;
_5 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(RET, 1), 0)));
_8.fld0 = [884272570_u32,3754450634_u32,1564866949_u32,2535464318_u32];
_9 = [_3,_3,_3,_3,_3];
RET = Adt52::Variant1 { fld0: _4 };
_1 = !false;
place!(Field::<u16>(Variant(RET, 1), 0)) = 15614336349626517545_u64 as u16;
place!(Field::<u16>(Variant(RET, 1), 0)) = !_4;
_10.0 = (-67_i8) as isize;
_10.3 = 73715153882348920989207949744960028463_i128;
_10.0 = !(-9223372036854775808_isize);
_10.1 = (-482321951_i32) * (-930015619_i32);
_2 = ['\u{afe0b}','\u{bd436}','\u{46435}','\u{39173}','\u{1b054}','\u{401c}','\u{ba38f}'];
_4 = !Field::<u16>(Variant(RET, 1), 0);
_10.1 = -(-216044597_i32);
_10.1 = !1816820477_i32;
Goto(bb3)
}
bb13 = {
_8.fld0 = [4054660174_u32,2010685336_u32,3439777361_u32,1906470728_u32];
_10.0 = 69_isize;
_4 = !Field::<u16>(Variant(RET, 1), 0);
SetDiscriminant(RET, 0);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 2103752200_u32;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0 ^ _10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 1574315585_u32 + 1542674931_u32;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = -_10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 2187120400_u32;
_8.fld0 = [Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0];
_10.2 = core::ptr::addr_of!(_10.1);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0 & _10.0;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0;
_10.0 = Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0 as isize;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).0 = 450370092_u32 | 518213797_u32;
_1 = _4 <= _4;
_11 = _10.1 as f32;
RET = Adt52::Variant1 { fld0: _4 };
_11 = 254_u8 as f32;
_13 = (3915739357_u32, _3);
RET = Adt52::Variant1 { fld0: _4 };
match _13.0 {
3915739357 => bb5,
_ => bb4
}
}
bb14 = {
_13.0 = _23 as u32;
_21.fld0 = ['\u{be995}','\u{512f8}'];
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)) = (_13.0, _21.fld3, _23);
_9 = _7;
_27 = -Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).2;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)) = (_13.0, _21.fld3, _27);
_8 = Adt44 { fld0: _26.fld0 };
_29.3 = _19 << _30;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = !_10.0;
_6 = core::ptr::addr_of_mut!(_29.4);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _29.2;
_21.fld4 = (_21.fld0,);
_29.1 = core::ptr::addr_of_mut!(_29.5.0);
_29.0 = _10.0 * _23;
_11 = _29.3 as f32;
_25 = [_14,_14,_14,_14,_14,_14,_14];
Goto(bb15)
}
bb15 = {
_15 = 24669_i16 | (-4102_i16);
_29.5.0 = core::ptr::addr_of!(_10.3);
_28 = _4 | _4;
_13.1 = _3 >> _15;
_34 = _1;
_2 = _17;
_21.fld1 = [_14,_14,_14,_14,_14,_14,_14];
_20 = '\u{e493d}';
Goto(bb16)
}
bb16 = {
_10.0 = _24 | _29.2;
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).2 = _10.0;
_33 = core::ptr::addr_of!(_14);
_16 = !9879880642773703372_u64;
_29.1 = core::ptr::addr_of_mut!(_29.5.0);
place!(Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0)).1 = _21.fld3;
_10.0 = _29.2 | _29.0;
_19 = _29.3 & _29.3;
_8.fld0 = [Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0,_13.0,Field::<(u32, *const u128, isize)>(Variant(RET, 0), 0).0];
_10.1 = (-540255224_i32) & (-1633227124_i32);
RET = Adt52::Variant1 { fld0: _28 };
_15 = (-14402_i16) + 19688_i16;
_31 = [_29.3,_19,_19,_29.3,_29.3,_29.3,_29.3];
_25 = [(*_33),_14,(*_33),(*_33),(*_33),(*_33),(*_33)];
_1 = _34;
_7 = _9;
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(18_usize, 3_usize, Move(_3), 2_usize, Move(_2), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(18_usize, 27_usize, Move(_27), 30_usize, Move(_30), 19_usize, Move(_19), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(18_usize, 31_usize, Move(_31), 12_usize, Move(_12), 25_usize, Move(_25), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [i64; 5],mut _2: [char; 2],mut _3: [u8; 7],mut _4: u32,mut _5: *mut [u128; 7],mut _6: [u32; 4],mut _7: Adt45,mut _8: [u128; 7],mut _9: Adt44,mut _10: isize,mut _11: [char; 2],mut _12: [char; 7],mut _13: [char; 2],mut _14: [u128; 7],mut _15: [char; 2],mut _16: [u128; 7]) -> isize {
mir! {
type RET = isize;
let _17: ();
let _18: ();
{
_1 = [1192269265083328943_i64,7487516158859752928_i64,(-6068077146558472566_i64),4730853224874197402_i64,2339891556813718049_i64];
_7.fld2 = _10 >> _4;
RET = _7.fld2 & _7.fld2;
_7.fld1.0 = ['\u{7ecc4}','\u{10ccda}'];
_11 = ['\u{ef58c}','\u{25d35}'];
(*_5) = [287405833031526019048271887686679397589_u128,290787072764474051874537723836067032122_u128,338697514791436196905950475061937419363_u128,146532298682647894109543545268585275281_u128,224630901440721461962076761232443173941_u128,15141549145024453979269534256107849441_u128,203798098477304432308931607703688030125_u128];
_7.fld0 = [105_u8,18_u8,87_u8,72_u8,42_u8,103_u8,90_u8];
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(19_usize, 12_usize, Move(_12), 11_usize, Move(_11), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(19_usize, 8_usize, Move(_8), 14_usize, Move(_14), 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{d74ff}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(125_i8), std::hint::black_box(30159_i16), std::hint::black_box(571560254_i32), std::hint::black_box(248425478692980645078882944870339057951_u128), std::hint::black_box((-119708863034781015238512767598254495630_i128)), std::hint::black_box(5035376455238966938_usize), std::hint::black_box(58_u8), std::hint::black_box(40687_u16), std::hint::black_box(4247081180_u32), std::hint::black_box(10990072327691842871_u64));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: [char; 2],
fld1: i128,
fld2: u128,
fld3: [i64; 5],
fld4: (*const i128,),

},
Variant1{
fld0: u64,

},
Variant2{
fld0: bool,
fld1: [i128; 4],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ([char; 2], [char; 7], [char; 2]),
fld1: *mut *const i128,
fld2: u32,
fld3: usize,
fld4: f32,
fld5: [char; 7],

},
Variant1{
fld0: bool,
fld1: [char; 2],
fld2: *mut *const i128,
fld3: *mut f64,
fld4: ([char; 2], [char; 7], [char; 2]),
fld5: [u8; 7],

},
Variant2{
fld0: (isize, i32, *const i32, i128),
fld1: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,)),
fld2: *mut i16,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *mut i16,

},
Variant1{
fld0: ([char; 2],),
fld1: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,)),
fld2: isize,
fld3: [u32; 4],
fld4: i16,
fld5: i32,
fld6: ([char; 2], [char; 7], [char; 2]),
fld7: *const (u32, *const u128, isize),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: [u32; 4],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: [u8; 7],
fld1: ([char; 2], [char; 7], [char; 2]),
fld2: isize,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u8; 7],
fld1: char,
fld2: Adt45,
fld3: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,)),
fld4: *const u128,
fld5: (*const i128,),

},
Variant1{
fld0: *const (u32, *const u128, isize),

},
Variant2{
fld0: *mut f64,
fld1: Adt45,
fld2: u128,
fld3: *const i128,
fld4: i16,
fld5: Adt43,

},
Variant3{
fld0: [char; 7],
fld1: [char; 2],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: i64,
fld1: ([char; 2],),
fld2: [i64; 5],
fld3: i8,
fld4: *mut i16,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (*const i128,),
fld1: u64,
fld2: *const (u32, *const u128, isize),
fld3: *mut i16,
fld4: *const i32,
fld5: f32,
fld6: i64,
fld7: (u32, *const u128, isize),

},
Variant1{
fld0: *mut [u128; 7],
fld1: f64,
fld2: [u128; 7],
fld3: u16,
fld4: *mut *const i128,
fld5: i32,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: (u32, *const u128, isize),
fld1: *const u128,
fld2: isize,
fld3: Adt42,
fld4: u16,

},
Variant1{
fld0: Adt47,
fld1: ([char; 2], [char; 7], [char; 2]),
fld2: u128,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: bool,
fld1: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,)),
fld2: isize,
fld3: *mut u16,
fld4: [u32; 4],
fld5: *const (u32, *const u128, isize),

},
Variant1{
fld0: f64,
fld1: *const i128,
fld2: isize,
fld3: *mut *const i128,
fld4: i16,
fld5: [u32; 4],
fld6: Adt43,
fld7: [i64; 5],

},
Variant2{
fld0: u128,
fld1: u8,
fld2: [u8; 7],
fld3: i8,
fld4: i64,

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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: ([char; 2], [char; 7], [char; 2]),
fld1: *const (u32, *const u128, isize),
fld2: i64,
fld3: Adt44,
fld4: Adt42,

},
Variant1{
fld0: i8,

},
Variant2{
fld0: [char; 2],
fld1: char,
fld2: *mut [u128; 7],

}}
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (u32, *const u128, isize),
fld1: Adt42,

},
Variant1{
fld0: u16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt51,
fld1: (isize, i32, *const i32, i128),
fld2: isize,
fld3: Adt43,
fld4: [i128; 4],
fld5: u8,
fld6: i64,
fld7: ([char; 2],),

},
Variant1{
fld0: (*const i128,),
fld1: *const i128,
fld2: u32,
fld3: Adt48,
fld4: Adt41,
fld5: u128,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: usize,
fld1: *const i32,
fld2: *const u128,
fld3: f64,
fld4: *mut f64,
fld5: Adt47,
fld6: u128,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [char; 2],
fld1: [u128; 7],
fld2: Adt48,
fld3: *const u128,
fld4: ([char; 2],),
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
fld0: *mut u16,
fld1: Adt54,
fld2: Adt44,

},
Variant1{
fld0: u8,
fld1: (isize, *mut *const i128, isize, u8, [u128; 7], (*const i128,)),
fld2: *const i128,
fld3: *mut [u128; 7],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt49,
fld1: *const (u32, *const u128, isize),
fld2: ([char; 2],),
fld3: [char; 2],

},
Variant1{
fld0: Adt49,
fld1: Adt55,
fld2: (u32, i64),
fld3: Adt56,
fld4: u64,
fld5: Adt44,
fld6: [i64; 5],
fld7: u32,

}}

