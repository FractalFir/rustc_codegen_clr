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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: u128,mut _4: u64,mut _5: i128) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _6: (f64, &'static f32, (Adt18, Adt18), *const &'static u32);
let _7: f32;
let _8: (*const i8, isize);
let _9: *mut i32;
let _10: char;
let _11: u128;
let _12: [usize; 3];
let _13: f32;
let _14: [u16; 4];
let _15: Adt75;
let _16: [i8; 8];
let _17: f32;
let _18: [bool; 5];
let _19: [u8; 2];
let _20: bool;
let _21: (isize,);
let _22: ();
let _23: ();
{
_5 = 6275871575095167933_i64 as i128;
RET = [_5,_5,_5,_5];
RET = [_5,_5,_5,_5];
_6.2.0 = Adt18::Variant0 { fld0: 4891319134318645159_i64,fld1: 58408_u16,fld2: 94_i8 };
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = !(-85_i8);
_6.2.0 = Adt18::Variant0 { fld0: 1218255943021912772_i64,fld1: 42636_u16,fld2: (-18_i8) };
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = 108_i8;
_6.2.1 = Adt18::Variant0 { fld0: 6866254265228469911_i64,fld1: 20913_u16,fld2: Field::<i8>(Variant(_6.2.0, 0), 2) };
_7 = _5 as f32;
_8.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_6.2.0, 0), 2)));
_8.1 = 9223372036854775807_isize << Field::<i8>(Variant(_6.2.1, 0), 2);
place!(Field::<i8>(Variant(_6.2.1, 0), 2)) = Field::<i8>(Variant(_6.2.0, 0), 2) & Field::<i8>(Variant(_6.2.0, 0), 2);
_6.0 = 19802_u16 as f64;
_5 = (-146277615373635448605218254037128985575_i128);
RET = [_5,_5,_5,_5];
place!(Field::<i64>(Variant(_6.2.1, 0), 0)) = _5 as i64;
place!(Field::<u16>(Variant(_6.2.0, 0), 1)) = 48965_u16;
_8.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_6.2.0, 0), 2)));
_2 = 64_u8;
_6.1 = &_7;
place!(Field::<u16>(Variant(_6.2.1, 0), 1)) = Field::<u16>(Variant(_6.2.0, 0), 1) ^ Field::<u16>(Variant(_6.2.0, 0), 1);
Goto(bb1)
}
bb1 = {
_1 = false;
RET = [_5,_5,_5,_5];
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = _2 as i8;
_11 = !285272716058782953157627523179994826191_u128;
place!(Field::<i64>(Variant(_6.2.0, 0), 0)) = Field::<i64>(Variant(_6.2.1, 0), 0);
_1 = !true;
SetDiscriminant(_6.2.0, 0);
_4 = 2034440778043888026_u64 | 16362296704139043816_u64;
_11 = 270363403166603587128509108882020391107_u128 & 211235099827031730549439259073099304919_u128;
SetDiscriminant(_6.2.1, 3);
place!(Field::<i128>(Variant(_6.2.1, 3), 0)) = !_5;
place!(Field::<u64>(Variant(_6.2.1, 3), 3)) = _4;
_10 = '\u{9edb8}';
Goto(bb2)
}
bb2 = {
_6.0 = 47673_u16 as f64;
_12 = [0_usize,17411716160124389571_usize,7440167407482193450_usize];
_11 = !153462307142227350118002780565798346845_u128;
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = (-120_i8);
_6.2.0 = Adt18::Variant3 { fld0: _5,fld1: (-7299061743291729170_i64),fld2: 1016995541_u32,fld3: _4,fld4: 17267_i16 };
RET = [Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.0, 3), 0),_5];
_4 = (-24502_i16) as u64;
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5];
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.1, 3), 0)];
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = (-99_i8) as i16;
place!(Field::<u32>(Variant(_6.2.0, 3), 2)) = _7 as u32;
_1 = false;
_14 = [17237_u16,16087_u16,31085_u16,22393_u16];
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-2625026831545215546_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
_13 = (-112_i8) as f32;
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = Field::<i16>(Variant(_6.2.1, 3), 4) * Field::<i16>(Variant(_6.2.1, 3), 4);
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-5635600766691905720_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
match Field::<i128>(Variant(_6.2.1, 3), 0) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
194004751547303014858156353394639225881 => bb8,
_ => bb7
}
}
bb3 = {
_1 = false;
RET = [_5,_5,_5,_5];
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = _2 as i8;
_11 = !285272716058782953157627523179994826191_u128;
place!(Field::<i64>(Variant(_6.2.0, 0), 0)) = Field::<i64>(Variant(_6.2.1, 0), 0);
_1 = !true;
SetDiscriminant(_6.2.0, 0);
_4 = 2034440778043888026_u64 | 16362296704139043816_u64;
_11 = 270363403166603587128509108882020391107_u128 & 211235099827031730549439259073099304919_u128;
SetDiscriminant(_6.2.1, 3);
place!(Field::<i128>(Variant(_6.2.1, 3), 0)) = !_5;
place!(Field::<u64>(Variant(_6.2.1, 3), 3)) = _4;
_10 = '\u{9edb8}';
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
_6.2.1 = Adt18::Variant2 { fld0: _1,fld1: Field::<u32>(Variant(_6.2.0, 3), 2),fld2: _2,fld3: Field::<i128>(Variant(_6.2.0, 3), 0),fld4: Field::<i16>(Variant(_6.2.0, 3), 4),fld5: _11,fld6: 4080176839114809786_i64 };
Call(place!(Field::<i64>(Variant(_6.2.0, 3), 1)) = fn1(Move(_6.1), Field::<i16>(Variant(_6.2.0, 3), 4), _14, _14, Field::<i16>(Variant(_6.2.0, 3), 4), _12, _12, Move(_8), _14, Field::<i128>(Variant(_6.2.1, 2), 3), _2, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14 = [298_u16,29132_u16,28892_u16,53975_u16];
_11 = 5116303049675709426_usize as u128;
place!(Field::<i64>(Variant(_6.2.1, 2), 6)) = _13 as i64;
_8.1 = (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_6.2.1, 2), 4)) = Field::<i16>(Variant(_6.2.0, 3), 4);
_6.2.1 = _6.2.0;
place!(Field::<i128>(Variant(_6.2.1, 3), 0)) = -Field::<i128>(Variant(_6.2.0, 3), 0);
_13 = Field::<i16>(Variant(_6.2.1, 3), 4) as f32;
_4 = Field::<u32>(Variant(_6.2.0, 3), 2) as u64;
_6.1 = &_13;
_15 = Adt75::Variant1 { fld0: _13 };
_8.1 = 9223372036854775807_isize;
place!(Field::<i64>(Variant(_6.2.0, 3), 1)) = Field::<i64>(Variant(_6.2.1, 3), 1);
place!(Field::<u32>(Variant(_6.2.0, 3), 2)) = !Field::<u32>(Variant(_6.2.1, 3), 2);
_13 = (-508655812_i32) as f32;
match Field::<i128>(Variant(_6.2.0, 3), 0) {
0 => bb1,
1 => bb7,
2 => bb10,
194004751547303014858156353394639225881 => bb12,
_ => bb11
}
}
bb10 = {
_6.0 = 47673_u16 as f64;
_12 = [0_usize,17411716160124389571_usize,7440167407482193450_usize];
_11 = !153462307142227350118002780565798346845_u128;
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = (-120_i8);
_6.2.0 = Adt18::Variant3 { fld0: _5,fld1: (-7299061743291729170_i64),fld2: 1016995541_u32,fld3: _4,fld4: 17267_i16 };
RET = [Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.0, 3), 0),_5];
_4 = (-24502_i16) as u64;
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5];
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.1, 3), 0)];
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = (-99_i8) as i16;
place!(Field::<u32>(Variant(_6.2.0, 3), 2)) = _7 as u32;
_1 = false;
_14 = [17237_u16,16087_u16,31085_u16,22393_u16];
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-2625026831545215546_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
_13 = (-112_i8) as f32;
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = Field::<i16>(Variant(_6.2.1, 3), 4) * Field::<i16>(Variant(_6.2.1, 3), 4);
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-5635600766691905720_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
match Field::<i128>(Variant(_6.2.1, 3), 0) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
194004751547303014858156353394639225881 => bb8,
_ => bb7
}
}
bb11 = {
Return()
}
bb12 = {
_3 = _2 as u128;
_11 = _3;
_6.2.1 = Adt18::Variant3 { fld0: _5,fld1: Field::<i64>(Variant(_6.2.0, 3), 1),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
_16 = [(-105_i8),27_i8,(-6_i8),88_i8,(-11_i8),(-123_i8),(-74_i8),57_i8];
_17 = _13;
RET = [_5,_5,Field::<i128>(Variant(_6.2.0, 3), 0),_5];
_11 = 51_i8 as u128;
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = !Field::<i16>(Variant(_6.2.1, 3), 4);
_16 = [(-105_i8),(-23_i8),49_i8,81_i8,105_i8,126_i8,(-128_i8),(-54_i8)];
_21 = (_8.1,);
_4 = Field::<u64>(Variant(_6.2.1, 3), 3) * Field::<u64>(Variant(_6.2.1, 3), 3);
match _2 {
0 => bb1,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
64 => bb19,
_ => bb18
}
}
bb13 = {
Return()
}
bb14 = {
_1 = false;
RET = [_5,_5,_5,_5];
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = _2 as i8;
_11 = !285272716058782953157627523179994826191_u128;
place!(Field::<i64>(Variant(_6.2.0, 0), 0)) = Field::<i64>(Variant(_6.2.1, 0), 0);
_1 = !true;
SetDiscriminant(_6.2.0, 0);
_4 = 2034440778043888026_u64 | 16362296704139043816_u64;
_11 = 270363403166603587128509108882020391107_u128 & 211235099827031730549439259073099304919_u128;
SetDiscriminant(_6.2.1, 3);
place!(Field::<i128>(Variant(_6.2.1, 3), 0)) = !_5;
place!(Field::<u64>(Variant(_6.2.1, 3), 3)) = _4;
_10 = '\u{9edb8}';
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_6.0 = 47673_u16 as f64;
_12 = [0_usize,17411716160124389571_usize,7440167407482193450_usize];
_11 = !153462307142227350118002780565798346845_u128;
place!(Field::<i8>(Variant(_6.2.0, 0), 2)) = (-120_i8);
_6.2.0 = Adt18::Variant3 { fld0: _5,fld1: (-7299061743291729170_i64),fld2: 1016995541_u32,fld3: _4,fld4: 17267_i16 };
RET = [Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.0, 3), 0),_5];
_4 = (-24502_i16) as u64;
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5];
RET = [Field::<i128>(Variant(_6.2.1, 3), 0),Field::<i128>(Variant(_6.2.0, 3), 0),_5,Field::<i128>(Variant(_6.2.1, 3), 0)];
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = (-99_i8) as i16;
place!(Field::<u32>(Variant(_6.2.0, 3), 2)) = _7 as u32;
_1 = false;
_14 = [17237_u16,16087_u16,31085_u16,22393_u16];
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-2625026831545215546_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
_13 = (-112_i8) as f32;
place!(Field::<i16>(Variant(_6.2.0, 3), 4)) = Field::<i16>(Variant(_6.2.1, 3), 4) * Field::<i16>(Variant(_6.2.1, 3), 4);
_6.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6.2.0, 3), 0),fld1: (-5635600766691905720_i64),fld2: Field::<u32>(Variant(_6.2.0, 3), 2),fld3: Field::<u64>(Variant(_6.2.0, 3), 3),fld4: Field::<i16>(Variant(_6.2.0, 3), 4) };
match Field::<i128>(Variant(_6.2.1, 3), 0) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
194004751547303014858156353394639225881 => bb8,
_ => bb7
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_6.0 = Field::<u64>(Variant(_6.2.0, 3), 3) as f64;
place!(Field::<u32>(Variant(_6.2.0, 3), 2)) = Field::<i16>(Variant(_6.2.1, 3), 4) as u32;
_11 = !_3;
_19 = [_2,_2];
place!(Field::<f32>(Variant(_15, 1), 0)) = Field::<u32>(Variant(_6.2.1, 3), 2) as f32;
SetDiscriminant(_15, 2);
_6.0 = (-810312233_i32) as f64;
_20 = _1 & _1;
Goto(bb20)
}
bb20 = {
Call(_22 = dump_var(0_usize, 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_22 = dump_var(0_usize, 11_usize, Move(_11), 19_usize, Move(_19), 23_usize, _23, 23_usize, _23), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static f32,mut _2: i16,mut _3: [u16; 4],mut _4: [u16; 4],mut _5: i16,mut _6: [usize; 3],mut _7: [usize; 3],mut _8: (*const i8, isize),mut _9: [u16; 4],mut _10: i128,mut _11: u8,mut _12: u64) -> i64 {
mir! {
type RET = i64;
let _13: f32;
let _14: *const f32;
let _15: Adt22;
let _16: Adt18;
let _17: Adt18;
let _18: *mut [char; 2];
let _19: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _20: [i8; 8];
let _21: u32;
let _22: f32;
let _23: *const bool;
let _24: isize;
let _25: f32;
let _26: f64;
let _27: usize;
let _28: [i128; 4];
let _29: ();
let _30: ();
{
_13 = _11 as f32;
_13 = 94_i8 as f32;
_7 = [5194562641223169076_usize,11802372055085244080_usize,8361633060824267925_usize];
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
64 => bb7,
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
_1 = &_13;
RET = _2 as i64;
_5 = -_2;
Call(RET = fn2(Move(_1), _9, _3, _7, _2, _3, _7, _3, _9, Move(_8), _3, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9 = _4;
_10 = 3615308462_u32 as i128;
_13 = RET as f32;
_2 = !_5;
RET = _13 as i64;
_12 = 9591830897043181999_u64;
_3 = _4;
_10 = (-132420268863574184896556742395641219739_i128) & (-138399840710705792962144868348607619096_i128);
_4 = [4440_u16,10771_u16,15976_u16,61087_u16];
Goto(bb9)
}
bb9 = {
_14 = core::ptr::addr_of!(_13);
_7 = _6;
_4 = [3874_u16,16353_u16,11537_u16,52325_u16];
_1 = &(*_14);
_8.1 = _10 as isize;
_8.1 = -(-56_isize);
_14 = core::ptr::addr_of!((*_1));
_9 = _3;
_4 = _9;
RET = _10 as i64;
_13 = _12 as f32;
_16 = Adt18::Variant1 { fld0: true,fld1: '\u{2acb1}',fld2: 1369656514_u32,fld3: 28443_u16,fld4: _10,fld5: 6_usize,fld6: 62972137179279310860855031815854503800_u128 };
place!(Field::<u16>(Variant(_16, 1), 3)) = 27923_u16;
RET = 3797170090912529853_i64 - 2121228009126455441_i64;
_6 = _7;
place!(Field::<usize>(Variant(_16, 1), 5)) = !5_usize;
_16 = Adt18::Variant3 { fld0: _10,fld1: RET,fld2: 2413824204_u32,fld3: _12,fld4: _5 };
_16 = Adt18::Variant1 { fld0: false,fld1: '\u{42cc8}',fld2: 4032258659_u32,fld3: 27319_u16,fld4: _10,fld5: 5_usize,fld6: 6752388318246567992496432925951459915_u128 };
match _12 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
9591830897043181999 => bb10,
_ => bb6
}
}
bb10 = {
place!(Field::<bool>(Variant(_16, 1), 0)) = _2 > _5;
_8.1 = -(-9223372036854775808_isize);
_17 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_16, 1), 0),fld1: 2960000573_u32,fld2: _11,fld3: _10,fld4: _5,fld5: 152981133709770413891865140289309588858_u128,fld6: RET };
place!(Field::<u128>(Variant(_17, 2), 5)) = 264823498014556860407666091264777875026_u128 ^ 266740688344470164542184684868598466282_u128;
place!(Field::<u128>(Variant(_17, 2), 5)) = 294669732019897760287019162705940843274_u128 ^ 114120249648620757333529759984049969059_u128;
place!(Field::<u128>(Variant(_16, 1), 6)) = !Field::<u128>(Variant(_17, 2), 5);
_19.0.0 = 16966092916590789097_usize as f64;
_4 = [237_u16,25958_u16,60221_u16,53991_u16];
_6 = _7;
place!(Field::<u32>(Variant(_17, 2), 1)) = !2515890101_u32;
_19.0.2 = (_17, _17);
place!(Field::<u32>(Variant(_16, 1), 2)) = _19.0.0 as u32;
_1 = &_13;
place!(Field::<u128>(Variant(_17, 2), 5)) = Field::<u128>(Variant(_19.0.2.0, 2), 5) >> Field::<u8>(Variant(_19.0.2.0, 2), 2);
place!(Field::<bool>(Variant(_19.0.2.0, 2), 0)) = Field::<i128>(Variant(_17, 2), 3) <= Field::<i128>(Variant(_19.0.2.0, 2), 3);
RET = -Field::<i64>(Variant(_17, 2), 6);
match Field::<u8>(Variant(_19.0.2.1, 2), 2) {
0 => bb8,
1 => bb2,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
64 => bb17,
_ => bb16
}
}
bb11 = {
_14 = core::ptr::addr_of!(_13);
_7 = _6;
_4 = [3874_u16,16353_u16,11537_u16,52325_u16];
_1 = &(*_14);
_8.1 = _10 as isize;
_8.1 = -(-56_isize);
_14 = core::ptr::addr_of!((*_1));
_9 = _3;
_4 = _9;
RET = _10 as i64;
_13 = _12 as f32;
_16 = Adt18::Variant1 { fld0: true,fld1: '\u{2acb1}',fld2: 1369656514_u32,fld3: 28443_u16,fld4: _10,fld5: 6_usize,fld6: 62972137179279310860855031815854503800_u128 };
place!(Field::<u16>(Variant(_16, 1), 3)) = 27923_u16;
RET = 3797170090912529853_i64 - 2121228009126455441_i64;
_6 = _7;
place!(Field::<usize>(Variant(_16, 1), 5)) = !5_usize;
_16 = Adt18::Variant3 { fld0: _10,fld1: RET,fld2: 2413824204_u32,fld3: _12,fld4: _5 };
_16 = Adt18::Variant1 { fld0: false,fld1: '\u{42cc8}',fld2: 4032258659_u32,fld3: 27319_u16,fld4: _10,fld5: 5_usize,fld6: 6752388318246567992496432925951459915_u128 };
match _12 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
9591830897043181999 => bb10,
_ => bb6
}
}
bb12 = {
_9 = _4;
_10 = 3615308462_u32 as i128;
_13 = RET as f32;
_2 = !_5;
RET = _13 as i64;
_12 = 9591830897043181999_u64;
_3 = _4;
_10 = (-132420268863574184896556742395641219739_i128) & (-138399840710705792962144868348607619096_i128);
_4 = [4440_u16,10771_u16,15976_u16,61087_u16];
Goto(bb9)
}
bb13 = {
_1 = &_13;
RET = _2 as i64;
_5 = -_2;
Call(RET = fn2(Move(_1), _9, _3, _7, _2, _3, _7, _3, _9, Move(_8), _3, _10), ReturnTo(bb8), UnwindUnreachable())
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
place!(Field::<i64>(Variant(_19.0.2.0, 2), 6)) = Field::<u128>(Variant(_17, 2), 5) as i64;
place!(Field::<u8>(Variant(_19.0.2.0, 2), 2)) = 41880_u16 as u8;
_19.0.1 = &(*_1);
place!(Field::<u32>(Variant(_16, 1), 2)) = !Field::<u32>(Variant(_17, 2), 1);
_16 = _17;
place!(Field::<u32>(Variant(_19.0.2.0, 2), 1)) = !Field::<u32>(Variant(_17, 2), 1);
_22 = -_13;
_21 = Field::<i64>(Variant(_19.0.2.0, 2), 6) as u32;
_19.0.2.0 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_16, 2), 0),fld1: '\u{1ce8}',fld2: Field::<u32>(Variant(_17, 2), 1),fld3: 10325_u16,fld4: Field::<i128>(Variant(_16, 2), 3),fld5: 5_usize,fld6: Field::<u128>(Variant(_17, 2), 5) };
place!(Field::<i128>(Variant(_19.0.2.1, 2), 3)) = !Field::<i128>(Variant(_16, 2), 3);
place!(Field::<bool>(Variant(_19.0.2.1, 2), 0)) = Field::<bool>(Variant(_19.0.2.0, 1), 0);
place!(Field::<i16>(Variant(_17, 2), 4)) = -Field::<i16>(Variant(_16, 2), 4);
_3 = [57786_u16,4204_u16,31281_u16,23059_u16];
place!(Field::<bool>(Variant(_16, 2), 0)) = !Field::<bool>(Variant(_19.0.2.0, 1), 0);
_19.0.0 = (-992393480_i32) as f64;
_19.0.2 = (_16, _16);
_28 = [Field::<i128>(Variant(_19.0.2.0, 2), 3),Field::<i128>(Variant(_16, 2), 3),Field::<i128>(Variant(_19.0.2.1, 2), 3),Field::<i128>(Variant(_16, 2), 3)];
place!(Field::<i16>(Variant(_16, 2), 4)) = _2;
_8.1 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_23 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_19.0.2.0, 2), 0)));
Goto(bb18)
}
bb18 = {
Call(_29 = dump_var(1_usize, 11_usize, Move(_11), 28_usize, Move(_28), 21_usize, Move(_21), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(1_usize, 12_usize, Move(_12), 10_usize, Move(_10), 30_usize, _30, 30_usize, _30), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: &'static f32,mut _2: [u16; 4],mut _3: [u16; 4],mut _4: [usize; 3],mut _5: i16,mut _6: [u16; 4],mut _7: [usize; 3],mut _8: [u16; 4],mut _9: [u16; 4],mut _10: (*const i8, isize),mut _11: [u16; 4],mut _12: i128) -> i64 {
mir! {
type RET = i64;
let _13: *const Adt18;
let _14: *mut [char; 2];
let _15: isize;
let _16: i32;
let _17: [u32; 3];
let _18: *const [isize; 8];
let _19: isize;
let _20: *mut *const u16;
let _21: f64;
let _22: char;
let _23: [char; 2];
let _24: Adt22;
let _25: f32;
let _26: isize;
let _27: i8;
let _28: f64;
let _29: bool;
let _30: i16;
let _31: isize;
let _32: i128;
let _33: f32;
let _34: i32;
let _35: ();
let _36: ();
{
_7 = [0_usize,5_usize,2_usize];
_5 = 13787_i16 << _10.1;
_12 = -(-77949593120587553419132288655048069498_i128);
RET = 7992940090177971415_i64;
RET = -(-6701298241164685119_i64);
_7 = [9077107219366949619_usize,7_usize,7_usize];
_12 = 117876989150874585780873089140794883910_i128 | (-156669840151406103130798277651010995623_i128);
_8 = _6;
_9 = _2;
_5 = 6306_i16;
_6 = _11;
_5 = 23_u8 as i16;
_8 = [14764_u16,57300_u16,13740_u16,56851_u16];
_10.1 = 9223372036854775807_isize >> _12;
_3 = _9;
_3 = _8;
_4 = [10752405715274370017_usize,15953046625347793500_usize,9731276695894188509_usize];
_8 = [46877_u16,55848_u16,55295_u16,2565_u16];
_9 = [11084_u16,32530_u16,47320_u16,58831_u16];
_2 = _3;
_8 = _9;
Call(RET = fn3(Move(_10), _12, _8, _11, _3, _7, _3, _9, _11, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _5 as i64;
RET = (-4239570269651555513_i64);
_4 = [17724538914003756383_usize,2194653051743570539_usize,6_usize];
_2 = _9;
_2 = [52551_u16,36157_u16,16319_u16,9141_u16];
_6 = [16489_u16,28444_u16,6978_u16,34315_u16];
_8 = [55916_u16,22647_u16,1154_u16,39919_u16];
_15 = (-9223372036854775808_isize) * 95_isize;
_15 = -60_isize;
_2 = [49155_u16,38093_u16,27290_u16,24217_u16];
RET = !(-6957940330978950434_i64);
_15 = (-9223372036854775808_isize) << _12;
_10.1 = _15 * _15;
RET = (-1178329457457554824_i64);
_15 = -_10.1;
_16 = 18910_u16 as i32;
_2 = _11;
_6 = [29371_u16,5444_u16,37743_u16,39930_u16];
_6 = [39228_u16,22063_u16,31967_u16,1992_u16];
_11 = _8;
_9 = [63407_u16,48773_u16,14691_u16,51620_u16];
_6 = [24008_u16,8801_u16,649_u16,31696_u16];
RET = (-8339835205526727051_i64);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463455034772226241484405 => bb6,
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
_8 = _11;
RET = 37_u8 as i64;
_9 = [16061_u16,25377_u16,39975_u16,15505_u16];
_17 = [574118654_u32,3998627468_u32,1585979164_u32];
_15 = 30828363193544502353824605726020444208_u128 as isize;
_6 = _2;
_2 = [31056_u16,49327_u16,17570_u16,1755_u16];
_12 = (-169280407311733152220334847789860240393_i128) ^ (-8054090169707200382534943696070791505_i128);
_11 = _2;
_19 = 1349596186_u32 as isize;
_6 = [340_u16,36841_u16,60313_u16,19098_u16];
RET = 0_usize as i64;
RET = (-7706421354538224082_i64) ^ (-317764943583423218_i64);
_9 = [7411_u16,51624_u16,4513_u16,62325_u16];
_9 = [3525_u16,37199_u16,16885_u16,55079_u16];
_7 = _4;
_16 = 387108299_u32 as i32;
_8 = [52872_u16,37046_u16,61499_u16,30901_u16];
_8 = [7454_u16,26724_u16,64147_u16,18906_u16];
_10.1 = -_15;
_21 = RET as f64;
_21 = 635450302_u32 as f64;
_19 = _10.1 >> _12;
_3 = [39214_u16,36310_u16,4439_u16,30745_u16];
_7 = [15189170662515537616_usize,16666620898165763831_usize,4_usize];
_2 = [52077_u16,48646_u16,20346_u16,3952_u16];
Call(RET = core::intrinsics::bswap(1611049685300898772_i64), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = (-3005139612994669391_i64);
_3 = [30880_u16,46152_u16,40084_u16,9347_u16];
_21 = 5095653983194524971_u64 as f64;
_16 = (-1469428557_i32);
_6 = [25998_u16,7695_u16,5875_u16,37252_u16];
_2 = _3;
_11 = _2;
_7 = [5_usize,4_usize,18382975069188764500_usize];
_21 = 908501581_u32 as f64;
_17 = [297651203_u32,2660977581_u32,2384718567_u32];
_4 = _7;
RET = 1_usize as i64;
RET = _5 as i64;
_7 = _4;
_8 = [20497_u16,55457_u16,16573_u16,12470_u16];
_17 = [613112419_u32,1284637152_u32,77506095_u32];
_15 = 188_u8 as isize;
_16 = '\u{c6e1c}' as i32;
_16 = (-1189965536_i32) * (-1958269879_i32);
_11 = [60596_u16,35622_u16,7435_u16,19955_u16];
_6 = [48707_u16,24325_u16,28436_u16,35766_u16];
_5 = 22599_i16;
RET = (-4764924220128976985_i64);
_4 = _7;
_16 = 2812743056_u32 as i32;
_8 = [20081_u16,2211_u16,37868_u16,54623_u16];
_9 = [47830_u16,20920_u16,28693_u16,54242_u16];
Goto(bb8)
}
bb8 = {
_11 = _8;
_3 = [33629_u16,46936_u16,7694_u16,19609_u16];
_14 = core::ptr::addr_of_mut!(_23);
_22 = '\u{eb489}';
_11 = _2;
_22 = '\u{74f29}';
_1 = &_25;
RET = -(-403925802939619791_i64);
(*_14) = [_22,_22];
_17 = [4234023949_u32,361684601_u32,2623936753_u32];
_6 = [48026_u16,8142_u16,46123_u16,10188_u16];
_11 = [58266_u16,19344_u16,61482_u16,53287_u16];
_11 = [25360_u16,49739_u16,6626_u16,4813_u16];
(*_14) = [_22,_22];
_25 = 249992378262563310553834214627831077156_u128 as f32;
_7 = [14187522385294606781_usize,1378084494721027581_usize,6_usize];
_10.0 = core::ptr::addr_of!(_27);
_26 = -_19;
_27 = 126_i8 ^ (-11_i8);
_23 = [_22,_22];
_9 = _6;
_15 = _26;
Goto(bb9)
}
bb9 = {
RET = 6586021239720903885_i64 * 8480153048673342152_i64;
_17 = [434456333_u32,820499281_u32,3783111313_u32];
_3 = _2;
_29 = !true;
_22 = '\u{c47a5}';
_17 = [970322270_u32,1436336445_u32,1868284043_u32];
_16 = 934623789_i32 + 352887847_i32;
_30 = -_5;
_1 = &_25;
_7 = _4;
_23 = [_22,_22];
_9 = _3;
_28 = _21;
_24 = Adt22::Variant1 { fld0: _25 };
_14 = core::ptr::addr_of_mut!((*_14));
match _5 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
22599 => bb17,
_ => bb16
}
}
bb10 = {
_11 = _8;
_3 = [33629_u16,46936_u16,7694_u16,19609_u16];
_14 = core::ptr::addr_of_mut!(_23);
_22 = '\u{eb489}';
_11 = _2;
_22 = '\u{74f29}';
_1 = &_25;
RET = -(-403925802939619791_i64);
(*_14) = [_22,_22];
_17 = [4234023949_u32,361684601_u32,2623936753_u32];
_6 = [48026_u16,8142_u16,46123_u16,10188_u16];
_11 = [58266_u16,19344_u16,61482_u16,53287_u16];
_11 = [25360_u16,49739_u16,6626_u16,4813_u16];
(*_14) = [_22,_22];
_25 = 249992378262563310553834214627831077156_u128 as f32;
_7 = [14187522385294606781_usize,1378084494721027581_usize,6_usize];
_10.0 = core::ptr::addr_of!(_27);
_26 = -_19;
_27 = 126_i8 ^ (-11_i8);
_23 = [_22,_22];
_9 = _6;
_15 = _26;
Goto(bb9)
}
bb11 = {
RET = (-3005139612994669391_i64);
_3 = [30880_u16,46152_u16,40084_u16,9347_u16];
_21 = 5095653983194524971_u64 as f64;
_16 = (-1469428557_i32);
_6 = [25998_u16,7695_u16,5875_u16,37252_u16];
_2 = _3;
_11 = _2;
_7 = [5_usize,4_usize,18382975069188764500_usize];
_21 = 908501581_u32 as f64;
_17 = [297651203_u32,2660977581_u32,2384718567_u32];
_4 = _7;
RET = 1_usize as i64;
RET = _5 as i64;
_7 = _4;
_8 = [20497_u16,55457_u16,16573_u16,12470_u16];
_17 = [613112419_u32,1284637152_u32,77506095_u32];
_15 = 188_u8 as isize;
_16 = '\u{c6e1c}' as i32;
_16 = (-1189965536_i32) * (-1958269879_i32);
_11 = [60596_u16,35622_u16,7435_u16,19955_u16];
_6 = [48707_u16,24325_u16,28436_u16,35766_u16];
_5 = 22599_i16;
RET = (-4764924220128976985_i64);
_4 = _7;
_16 = 2812743056_u32 as i32;
_8 = [20081_u16,2211_u16,37868_u16,54623_u16];
_9 = [47830_u16,20920_u16,28693_u16,54242_u16];
Goto(bb8)
}
bb12 = {
_8 = _11;
RET = 37_u8 as i64;
_9 = [16061_u16,25377_u16,39975_u16,15505_u16];
_17 = [574118654_u32,3998627468_u32,1585979164_u32];
_15 = 30828363193544502353824605726020444208_u128 as isize;
_6 = _2;
_2 = [31056_u16,49327_u16,17570_u16,1755_u16];
_12 = (-169280407311733152220334847789860240393_i128) ^ (-8054090169707200382534943696070791505_i128);
_11 = _2;
_19 = 1349596186_u32 as isize;
_6 = [340_u16,36841_u16,60313_u16,19098_u16];
RET = 0_usize as i64;
RET = (-7706421354538224082_i64) ^ (-317764943583423218_i64);
_9 = [7411_u16,51624_u16,4513_u16,62325_u16];
_9 = [3525_u16,37199_u16,16885_u16,55079_u16];
_7 = _4;
_16 = 387108299_u32 as i32;
_8 = [52872_u16,37046_u16,61499_u16,30901_u16];
_8 = [7454_u16,26724_u16,64147_u16,18906_u16];
_10.1 = -_15;
_21 = RET as f64;
_21 = 635450302_u32 as f64;
_19 = _10.1 >> _12;
_3 = [39214_u16,36310_u16,4439_u16,30745_u16];
_7 = [15189170662515537616_usize,16666620898165763831_usize,4_usize];
_2 = [52077_u16,48646_u16,20346_u16,3952_u16];
Call(RET = core::intrinsics::bswap(1611049685300898772_i64), ReturnTo(bb7), UnwindUnreachable())
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
RET = _5 as i64;
RET = (-4239570269651555513_i64);
_4 = [17724538914003756383_usize,2194653051743570539_usize,6_usize];
_2 = _9;
_2 = [52551_u16,36157_u16,16319_u16,9141_u16];
_6 = [16489_u16,28444_u16,6978_u16,34315_u16];
_8 = [55916_u16,22647_u16,1154_u16,39919_u16];
_15 = (-9223372036854775808_isize) * 95_isize;
_15 = -60_isize;
_2 = [49155_u16,38093_u16,27290_u16,24217_u16];
RET = !(-6957940330978950434_i64);
_15 = (-9223372036854775808_isize) << _12;
_10.1 = _15 * _15;
RET = (-1178329457457554824_i64);
_15 = -_10.1;
_16 = 18910_u16 as i32;
_2 = _11;
_6 = [29371_u16,5444_u16,37743_u16,39930_u16];
_6 = [39228_u16,22063_u16,31967_u16,1992_u16];
_11 = _8;
_9 = [63407_u16,48773_u16,14691_u16,51620_u16];
_6 = [24008_u16,8801_u16,649_u16,31696_u16];
RET = (-8339835205526727051_i64);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463455034772226241484405 => bb6,
_ => bb5
}
}
bb17 = {
_30 = -_5;
_12 = -103142804762514124702567884886426154050_i128;
_25 = -Field::<f32>(Variant(_24, 1), 0);
_10.0 = core::ptr::addr_of!(_27);
place!(Field::<f32>(Variant(_24, 1), 0)) = -_25;
_21 = -_28;
RET = _30 as i64;
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(2_usize, 9_usize, Move(_9), 3_usize, Move(_3), 23_usize, Move(_23), 30_usize, Move(_30)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(2_usize, 2_usize, Move(_2), 4_usize, Move(_4), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(2_usize, 12_usize, Move(_12), 17_usize, Move(_17), 36_usize, _36, 36_usize, _36), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: (*const i8, isize),mut _2: i128,mut _3: [u16; 4],mut _4: [u16; 4],mut _5: [u16; 4],mut _6: [usize; 3],mut _7: [u16; 4],mut _8: [u16; 4],mut _9: [u16; 4],mut _10: [u16; 4],mut _11: [usize; 3]) -> i64 {
mir! {
type RET = i64;
let _12: f32;
let _13: &'static i64;
let _14: u32;
let _15: Adt83;
let _16: f32;
let _17: isize;
let _18: &'static i64;
let _19: i32;
let _20: u128;
let _21: &'static char;
let _22: &'static char;
let _23: Adt24;
let _24: isize;
let _25: u128;
let _26: isize;
let _27: isize;
let _28: bool;
let _29: [bool; 5];
let _30: f32;
let _31: *mut [isize; 8];
let _32: *const &'static i64;
let _33: [i128; 4];
let _34: Adt40;
let _35: f32;
let _36: [u16; 4];
let _37: Adt18;
let _38: (*const i8, isize);
let _39: bool;
let _40: [u16; 4];
let _41: *mut i32;
let _42: (i32, i32);
let _43: &'static u32;
let _44: [u32; 3];
let _45: ();
let _46: ();
{
RET = 17912_u16 as i64;
_12 = _2 as f32;
_6 = [12323255328564481329_usize,2654312409162557743_usize,10360286266900648094_usize];
_15.fld0.0 = -_12;
_8 = [20687_u16,12464_u16,47210_u16,44240_u16];
_6 = _11;
_12 = -_15.fld0.0;
_15.fld4 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
_15.fld3.0.0 = -1824485646_i32;
_10 = [33789_u16,11430_u16,14547_u16,52348_u16];
_15.fld3.0.1 = -_15.fld3.0.0;
Goto(bb1)
}
bb1 = {
_15.fld4 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
Call(_15.fld0.3 = fn4(_15.fld4, Move(_1.0), _1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.fld3.0 = (868977365_i32, (-510624328_i32));
_11 = [6_usize,3_usize,2856836167091998830_usize];
_16 = -_15.fld0.0;
_16 = -_12;
_12 = _16 + _16;
_9 = [18408_u16,59664_u16,59432_u16,37956_u16];
_15.fld3.0.1 = _15.fld3.0.0 << _1.1;
_1.1 = RET as isize;
_15.fld5 = core::ptr::addr_of!(_15.fld0.1);
_5 = [19349_u16,32870_u16,20252_u16,40078_u16];
_15.fld5 = Move(_15.fld0.3);
_12 = _15.fld0.0 * _15.fld0.0;
_11 = [0_usize,9673366205159739959_usize,4_usize];
_16 = _12;
_15.fld3.0.1 = true as i32;
_18 = &RET;
_15.fld0.1 = 111_i8 as i128;
RET = 3747542662988503694_i64;
_15.fld3.0.1 = _15.fld3.0.0 | _15.fld3.0.0;
RET = 45331_u16 as i64;
_18 = &RET;
RET = 3_usize as i64;
_4 = [61896_u16,1484_u16,13253_u16,60363_u16];
_16 = -_15.fld0.0;
_18 = &RET;
Goto(bb3)
}
bb3 = {
_4 = _7;
_14 = _15.fld0.1 as u32;
_8 = [29202_u16,32986_u16,6023_u16,34706_u16];
_21 = &_23.fld1;
_13 = &(*_18);
_23.fld2 = Adt18::Variant3 { fld0: _2,fld1: (*_13),fld2: _14,fld3: 4225562422403873642_u64,fld4: 27222_i16 };
_6 = [3_usize,17883424642506392992_usize,0_usize];
_19 = _1.1 as i32;
_1.1 = -(-9223372036854775808_isize);
_2 = Field::<i128>(Variant(_23.fld2, 3), 0) << (*_13);
_23.fld1 = '\u{d37bb}';
_15.fld0.3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_23.fld2, 3), 0)));
_25 = !148572917232304067310548422386903911852_u128;
_17 = 87_u8 as isize;
RET = 7141760343203823587_usize as i64;
place!(Field::<u64>(Variant(_23.fld2, 3), 3)) = !4220689524870994314_u64;
RET = -Field::<i64>(Variant(_23.fld2, 3), 1);
_23.fld0 = _2;
_26 = _1.1;
_22 = &_23.fld1;
_6 = _11;
match _15.fld3.0.0 {
0 => bb1,
1 => bb4,
868977365 => bb6,
_ => bb5
}
}
bb4 = {
_15.fld3.0 = (868977365_i32, (-510624328_i32));
_11 = [6_usize,3_usize,2856836167091998830_usize];
_16 = -_15.fld0.0;
_16 = -_12;
_12 = _16 + _16;
_9 = [18408_u16,59664_u16,59432_u16,37956_u16];
_15.fld3.0.1 = _15.fld3.0.0 << _1.1;
_1.1 = RET as isize;
_15.fld5 = core::ptr::addr_of!(_15.fld0.1);
_5 = [19349_u16,32870_u16,20252_u16,40078_u16];
_15.fld5 = Move(_15.fld0.3);
_12 = _15.fld0.0 * _15.fld0.0;
_11 = [0_usize,9673366205159739959_usize,4_usize];
_16 = _12;
_15.fld3.0.1 = true as i32;
_18 = &RET;
_15.fld0.1 = 111_i8 as i128;
RET = 3747542662988503694_i64;
_15.fld3.0.1 = _15.fld3.0.0 | _15.fld3.0.0;
RET = 45331_u16 as i64;
_18 = &RET;
RET = 3_usize as i64;
_4 = [61896_u16,1484_u16,13253_u16,60363_u16];
_16 = -_15.fld0.0;
_18 = &RET;
Goto(bb3)
}
bb5 = {
_15.fld4 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
Call(_15.fld0.3 = fn4(_15.fld4, Move(_1.0), _1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_15.fld3.0 = (_19, _19);
_15.fld0.0 = -_12;
_15.fld0.0 = _16;
_19 = _15.fld3.0.0;
_2 = _12 as i128;
place!(Field::<i128>(Variant(_23.fld2, 3), 0)) = 1_usize as i128;
_10 = [17053_u16,41000_u16,38598_u16,63448_u16];
_28 = !true;
_15.fld2 = -_17;
_6 = [4_usize,4_usize,15381327184232149710_usize];
_5 = _3;
place!(Field::<i64>(Variant(_23.fld2, 3), 1)) = RET & RET;
_21 = &_23.fld1;
_31 = core::ptr::addr_of_mut!(_15.fld4);
_30 = -_16;
_1.1 = _17;
_13 = &RET;
_32 = core::ptr::addr_of!(_13);
_6 = _11;
_29 = [_28,_28,_28,_28,_28];
_15.fld0.3 = Move(_15.fld5);
_8 = [52727_u16,45955_u16,50949_u16,1197_u16];
_11 = _6;
Call(_34.fld0 = fn5(Move(_32), Move(_21)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_34.fld4 = -_30;
Goto(bb8)
}
bb8 = {
_15.fld3.0.0 = 67_i8 as i32;
_18 = &place!(Field::<i64>(Variant(_23.fld2, 3), 1));
_15.fld4 = [_1.1,_26,_26,_15.fld2,_1.1,_26,_26,_15.fld2];
_15.fld1 = _16 - _30;
_18 = &place!(Field::<i64>(Variant(_23.fld2, 3), 1));
_15.fld0.0 = (-38_i8) as f32;
_25 = !11815898866371956680581710771632483936_u128;
_34.fld2 = !_1.1;
_15.fld0.1 = !_2;
_15.fld2 = _17;
_15.fld0.3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_23.fld2, 3), 0)));
_15.fld0.0 = _15.fld1 + _15.fld1;
_28 = !false;
_15.fld0.1 = _2;
_15.fld0.0 = _15.fld1 * _34.fld4;
Goto(bb9)
}
bb9 = {
_15.fld3.1 = core::ptr::addr_of!(_23.fld2);
_4 = _3;
_20 = _25;
_15.fld2 = _17 + _17;
_34.fld3 = core::ptr::addr_of!(_16);
_15.fld0.2 = core::ptr::addr_of_mut!(_28);
place!(Field::<i64>(Variant(_23.fld2, 3), 1)) = -(*_13);
_24 = _15.fld3.0.1 as isize;
Goto(bb10)
}
bb10 = {
_11 = [6926417091597598589_usize,4_usize,2_usize];
_15.fld2 = (-13724_i16) as isize;
_32 = core::ptr::addr_of!(_13);
_35 = _15.fld1;
_15.fld3.1 = core::ptr::addr_of!(_23.fld2);
_23.fld2 = Adt18::Variant0 { fld0: (*_13),fld1: 967_u16,fld2: (-126_i8) };
_15.fld1 = _34.fld4 * _35;
_23.fld1 = '\u{b5874}';
_30 = 32439_i16 as f32;
_33 = [_2,_15.fld0.1,_15.fld0.1,_2];
_12 = (-22_i8) as f32;
place!(Field::<i64>(Variant(_23.fld2, 0), 0)) = !(*_13);
_36 = [11540_u16,14743_u16,8433_u16,43989_u16];
_4 = [32671_u16,47634_u16,7755_u16,2056_u16];
place!(Field::<i64>(Variant(_23.fld2, 0), 0)) = -(*_13);
_15.fld3.0 = (_19, _19);
_7 = [60546_u16,46877_u16,46009_u16,14108_u16];
_18 = Move((*_32));
_25 = _20;
_21 = &_23.fld1;
_22 = Move(_21);
_32 = core::ptr::addr_of!(_13);
_30 = _15.fld1 * _35;
Goto(bb11)
}
bb11 = {
_15.fld4 = [_1.1,_26,_34.fld2,_1.1,_24,_1.1,_34.fld2,_34.fld2];
_33 = [_2,_2,_15.fld0.1,_2];
_14 = 2899502855_u32;
_1.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_23.fld2, 0), 2)));
_2 = -_15.fld0.1;
place!(Field::<i8>(Variant(_23.fld2, 0), 2)) = !(-3_i8);
_22 = &_23.fld1;
Call(_25 = core::intrinsics::bswap(_20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_32) = &place!(Field::<i64>(Variant(_23.fld2, 0), 0));
_24 = _34.fld2 << _25;
_15.fld3.0 = (_19, _19);
_15.fld5 = Move(_15.fld0.3);
_17 = 8671457414569344728_u64 as isize;
_33 = [_2,_15.fld0.1,_23.fld0,_2];
_23.fld1 = '\u{19b53}';
_34.fld2 = -_17;
_7 = _3;
_13 = &(*_13);
Goto(bb13)
}
bb13 = {
_6 = [4_usize,5_usize,2_usize];
_23.fld2 = Adt18::Variant0 { fld0: RET,fld1: 41398_u16,fld2: 58_i8 };
_29 = [_28,_28,_28,_28,_28];
_9 = _8;
_23.fld2 = Adt18::Variant1 { fld0: _28,fld1: _23.fld1,fld2: _14,fld3: 63487_u16,fld4: _23.fld0,fld5: 2_usize,fld6: _20 };
place!(Field::<usize>(Variant(_23.fld2, 1), 5)) = Field::<i128>(Variant(_23.fld2, 1), 4) as usize;
_21 = &place!(Field::<char>(Variant(_23.fld2, 1), 1));
_8 = _7;
_38.1 = _34.fld2 | _1.1;
_13 = &RET;
place!(Field::<bool>(Variant(_23.fld2, 1), 0)) = _28;
_32 = core::ptr::addr_of!((*_32));
_34.fld0 = core::ptr::addr_of!(place!(Field::<u16>(Variant(_23.fld2, 1), 3)));
_34.fld1 = [_14,_14,Field::<u32>(Variant(_23.fld2, 1), 2)];
_26 = _1.1;
_34.fld4 = _35 - _30;
_11 = [Field::<usize>(Variant(_23.fld2, 1), 5),Field::<usize>(Variant(_23.fld2, 1), 5),Field::<usize>(Variant(_23.fld2, 1), 5)];
_34.fld3 = core::ptr::addr_of!(_34.fld4);
_15.fld2 = !_1.1;
_9 = [63432_u16,52370_u16,27293_u16,62339_u16];
_5 = _8;
_23.fld3 = Adt22::Variant1 { fld0: _30 };
_7 = [16936_u16,61535_u16,3471_u16,14456_u16];
_19 = _15.fld3.0.1 * _15.fld3.0.0;
_15.fld3.0.0 = _15.fld3.0.1;
_42.1 = _15.fld3.0.0;
_15.fld0.3 = Move(_15.fld5);
_27 = _24 >> _2;
Goto(bb14)
}
bb14 = {
_42 = _15.fld3.0;
(*_32) = &(*_13);
_34.fld1 = [Field::<u32>(Variant(_23.fld2, 1), 2),_14,_14];
_30 = _34.fld4 * _15.fld0.0;
_14 = Field::<u32>(Variant(_23.fld2, 1), 2) ^ Field::<u32>(Variant(_23.fld2, 1), 2);
SetDiscriminant(_23.fld3, 0);
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(3_usize, 24_usize, Move(_24), 42_usize, Move(_42), 28_usize, Move(_28), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(3_usize, 2_usize, Move(_2), 7_usize, Move(_7), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(3_usize, 4_usize, Move(_4), 33_usize, Move(_33), 17_usize, Move(_17), 46_usize, _46), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [isize; 8],mut _2: *const i8,mut _3: isize,mut _4: [u16; 4]) -> *const i128 {
mir! {
type RET = *const i128;
let _5: *mut i32;
let _6: f64;
let _7: *mut bool;
let _8: *mut *const u16;
let _9: [char; 2];
let _10: isize;
let _11: isize;
let _12: Adt18;
let _13: f32;
let _14: *const [isize; 8];
let _15: (i32, [u32; 3], *const &'static u32);
let _16: *const &'static i64;
let _17: bool;
let _18: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _19: *mut [isize; 8];
let _20: isize;
let _21: *const usize;
let _22: u128;
let _23: Adt40;
let _24: *mut i32;
let _25: char;
let _26: *const [isize; 8];
let _27: ();
let _28: ();
{
_3 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = -(-9223372036854775808_isize);
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = [49450_u16,13881_u16,34053_u16,46447_u16];
Goto(bb1)
}
bb1 = {
_4 = [36237_u16,32640_u16,3838_u16,26249_u16];
_3 = 9223372036854775807_isize;
_4 = [56732_u16,40795_u16,22446_u16,58723_u16];
_3 = 119_isize;
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_6 = 32409_i16 as f64;
_4 = [38169_u16,61676_u16,26916_u16,28478_u16];
_4 = [51552_u16,7909_u16,33779_u16,57753_u16];
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
119 => bb9,
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
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = -9223372036854775807_isize;
_6 = 3880451453_u32 as f64;
_3 = 9223372036854775807_isize;
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = (-9223372036854775808_isize) - (-115_isize);
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_9 = ['\u{2057a}','\u{2a503}'];
_6 = 8986_u16 as f64;
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_6 = 646846670_i32 as f64;
_6 = 1634344626_u32 as f64;
_4 = [35584_u16,9190_u16,26944_u16,61042_u16];
_3 = -(-9223372036854775808_isize);
_4 = [57244_u16,45420_u16,12266_u16,46948_u16];
_1 = [_3,_3,_3,_3,_3,_3,_3,_3];
_12 = Adt18::Variant2 { fld0: true,fld1: 791974459_u32,fld2: 65_u8,fld3: (-37775034390420399058050418789349622456_i128),fld4: 18789_i16,fld5: 240850963272211310981513453358661267965_u128,fld6: (-813141972874326551_i64) };
_11 = _3;
place!(Field::<u32>(Variant(_12, 2), 1)) = '\u{6443}' as u32;
place!(Field::<i16>(Variant(_12, 2), 4)) = 31137_i16;
match Field::<i16>(Variant(_12, 2), 4) {
31137 => bb10,
_ => bb7
}
}
bb10 = {
_10 = !_11;
place!(Field::<u128>(Variant(_12, 2), 5)) = 152360972574291281079240867056529497658_u128 - 654147049005729407702387906104678141_u128;
_13 = Field::<i16>(Variant(_12, 2), 4) as f32;
_13 = _6 as f32;
_3 = _11 * _10;
place!(Field::<i16>(Variant(_12, 2), 4)) = -13593_i16;
place!(Field::<u128>(Variant(_12, 2), 5)) = 326615359403529171396754074770172778752_u128 ^ 333467613483494890123689461379999623838_u128;
_3 = -_10;
_12 = Adt18::Variant2 { fld0: false,fld1: 2557952592_u32,fld2: 8_u8,fld3: 50569908226583158028255150668207949324_i128,fld4: 25711_i16,fld5: 31607418181223279072674912631063677088_u128,fld6: (-5621059222014239758_i64) };
RET = core::ptr::addr_of!(place!(Field::<i128>(Variant(_12, 2), 3)));
(*RET) = 137706715707811340444646452899552224814_i128 >> _10;
_3 = _11;
place!(Field::<u8>(Variant(_12, 2), 2)) = 25293_i16 as u8;
_12 = Adt18::Variant2 { fld0: false,fld1: 3414711497_u32,fld2: 57_u8,fld3: (-95743589893708186722992219836006523774_i128),fld4: (-31584_i16),fld5: 244420014565278495528692045613164012148_u128,fld6: (-8143677227941079625_i64) };
_14 = core::ptr::addr_of!(_1);
place!(Field::<i16>(Variant(_12, 2), 4)) = -1809_i16;
_12 = Adt18::Variant2 { fld0: true,fld1: 4242014122_u32,fld2: 154_u8,fld3: (-72096880784817744993425297461061894457_i128),fld4: 9017_i16,fld5: 249107861148855013162357910674451600984_u128,fld6: (-8113387005005758402_i64) };
(*_14) = [_10,_3,_10,_11,_11,_10,_11,_10];
Goto(bb11)
}
bb11 = {
_14 = core::ptr::addr_of!(_1);
RET = core::ptr::addr_of!(place!(Field::<i128>(Variant(_12, 2), 3)));
_14 = core::ptr::addr_of!((*_14));
(*RET) = (-42102137472067145291993322185347895274_i128) + 33841447014636767814828076259182351819_i128;
place!(Field::<i128>(Variant(_12, 2), 3)) = _3 as i128;
place!(Field::<i16>(Variant(_12, 2), 4)) = !(-7013_i16);
place!(Field::<i16>(Variant(_12, 2), 4)) = 5838567608991394822_u64 as i16;
_15.1 = [2761894301_u32,2946098494_u32,386385660_u32];
(*RET) = (-72108373515083625090072556498103235170_i128);
place!(Field::<u128>(Variant(_12, 2), 5)) = 260574301329940275531457879817802851237_u128 * 195888013864764328073352305172480555211_u128;
RET = core::ptr::addr_of!(place!(Field::<i128>(Variant(_12, 2), 3)));
_12 = Adt18::Variant0 { fld0: 7401924103208314848_i64,fld1: 667_u16,fld2: (-105_i8) };
_15.1 = [504770222_u32,3345618897_u32,2095496918_u32];
_5 = core::ptr::addr_of_mut!(_15.0);
_15.0 = (-2041719113_i32) | (-249309730_i32);
_2 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_12, 0), 2)));
_15.0 = (-166342630_i32);
_20 = _11 * _10;
(*_14) = [_10,_20,_11,_20,_3,_10,_3,_3];
(*_2) = (-115_i8) << (*_5);
_15.1 = [419863478_u32,6437564_u32,1939524274_u32];
place!(Field::<i64>(Variant(_12, 0), 0)) = (-1757214895383536181_i64) + (-5788563297106134634_i64);
_14 = core::ptr::addr_of!(_1);
_6 = _10 as f64;
_17 = false;
_7 = core::ptr::addr_of_mut!(_17);
Goto(bb12)
}
bb12 = {
_15.1 = [3883705664_u32,4289295692_u32,2970538084_u32];
_15.0 = (-715848352_i32) - (-2064036777_i32);
_7 = core::ptr::addr_of_mut!(_17);
_19 = core::ptr::addr_of_mut!((*_14));
_23.fld4 = -_13;
_17 = _6 == _6;
(*_2) = (-102_i8) | 106_i8;
(*_2) = 37_i8;
_23.fld2 = _20 >> (*_2);
_11 = !_10;
place!(Field::<u16>(Variant(_12, 0), 1)) = 56155_u16;
_20 = _23.fld2 + _11;
(*_5) = -763674320_i32;
place!(Field::<i8>(Variant(_12, 0), 2)) = 18_i8;
_10 = _23.fld2;
_22 = 29097100560523740171661391116795371371_u128;
place!(Field::<u16>(Variant(_12, 0), 1)) = !59248_u16;
place!(Field::<i64>(Variant(_12, 0), 0)) = 15512261929778263449_usize as i64;
_3 = _20 - _20;
_23.fld1 = _15.1;
match Field::<i8>(Variant(_12, 0), 2) {
0 => bb9,
1 => bb13,
2 => bb14,
3 => bb15,
18 => bb17,
_ => bb16
}
}
bb13 = {
Return()
}
bb14 = {
_10 = !_11;
place!(Field::<u128>(Variant(_12, 2), 5)) = 152360972574291281079240867056529497658_u128 - 654147049005729407702387906104678141_u128;
_13 = Field::<i16>(Variant(_12, 2), 4) as f32;
_13 = _6 as f32;
_3 = _11 * _10;
place!(Field::<i16>(Variant(_12, 2), 4)) = -13593_i16;
place!(Field::<u128>(Variant(_12, 2), 5)) = 326615359403529171396754074770172778752_u128 ^ 333467613483494890123689461379999623838_u128;
_3 = -_10;
_12 = Adt18::Variant2 { fld0: false,fld1: 2557952592_u32,fld2: 8_u8,fld3: 50569908226583158028255150668207949324_i128,fld4: 25711_i16,fld5: 31607418181223279072674912631063677088_u128,fld6: (-5621059222014239758_i64) };
RET = core::ptr::addr_of!(place!(Field::<i128>(Variant(_12, 2), 3)));
(*RET) = 137706715707811340444646452899552224814_i128 >> _10;
_3 = _11;
place!(Field::<u8>(Variant(_12, 2), 2)) = 25293_i16 as u8;
_12 = Adt18::Variant2 { fld0: false,fld1: 3414711497_u32,fld2: 57_u8,fld3: (-95743589893708186722992219836006523774_i128),fld4: (-31584_i16),fld5: 244420014565278495528692045613164012148_u128,fld6: (-8143677227941079625_i64) };
_14 = core::ptr::addr_of!(_1);
place!(Field::<i16>(Variant(_12, 2), 4)) = -1809_i16;
_12 = Adt18::Variant2 { fld0: true,fld1: 4242014122_u32,fld2: 154_u8,fld3: (-72096880784817744993425297461061894457_i128),fld4: 9017_i16,fld5: 249107861148855013162357910674451600984_u128,fld6: (-8113387005005758402_i64) };
(*_14) = [_10,_3,_10,_11,_11,_10,_11,_10];
Goto(bb11)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_22 = 98439739622868766762836224550278782075_u128;
_14 = core::ptr::addr_of!((*_14));
_22 = !144498337320085694550066049930240520572_u128;
SetDiscriminant(_12, 2);
place!(Field::<u32>(Variant(_12, 2), 1)) = 155026983026191480720165640412144737361_i128 as u32;
_6 = 205_u8 as f64;
(*_14) = [_20,_3,_20,_3,_3,_11,_3,_10];
(*_14) = [_20,_3,_10,_3,_3,_3,_3,_3];
(*_19) = [_10,_23.fld2,_20,_20,_3,_3,_10,_23.fld2];
_4 = [1452_u16,12644_u16,12001_u16,27837_u16];
place!(Field::<bool>(Variant(_12, 2), 0)) = _17;
place!(Field::<i64>(Variant(_12, 2), 6)) = 4441266532859433461_i64;
_24 = Move(_5);
RET = core::ptr::addr_of!(place!(Field::<i128>(Variant(_12, 2), 3)));
_13 = _23.fld4 * _23.fld4;
_8 = core::ptr::addr_of_mut!(_23.fld0);
_5 = Move(_24);
_23.fld3 = core::ptr::addr_of!(_23.fld4);
_15.1 = [Field::<u32>(Variant(_12, 2), 1),Field::<u32>(Variant(_12, 2), 1),Field::<u32>(Variant(_12, 2), 1)];
(*_7) = _3 < _3;
_12 = Adt18::Variant0 { fld0: 792937319189289863_i64,fld1: 17823_u16,fld2: (-62_i8) };
_8 = core::ptr::addr_of_mut!((*_8));
_15.1 = [3724570935_u32,1568328029_u32,2418239435_u32];
(*_8) = core::ptr::addr_of!(place!(Field::<u16>(Variant(_12, 0), 1)));
_4 = [23454_u16,60477_u16,14226_u16,23198_u16];
(*_8) = core::ptr::addr_of!(place!(Field::<u16>(Variant(_12, 0), 1)));
_25 = '\u{35edd}';
_23.fld2 = 6409033736069085585_usize as isize;
(*_14) = [_3,_3,_3,_3,_10,_20,_20,_20];
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(4_usize, 3_usize, Move(_3), 4_usize, Move(_4), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(4_usize, 10_usize, Move(_10), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *const &'static i64,mut _2: &'static char) -> *const u16 {
mir! {
type RET = *const u16;
let _3: f32;
let _4: f64;
let _5: [i8; 8];
let _6: f64;
let _7: i64;
let _8: *const f32;
let _9: ([isize; 8], *const *mut i64, *const *mut *const u16, *mut i64);
let _10: f32;
let _11: [u16; 4];
let _12: isize;
let _13: f32;
let _14: [isize; 8];
let _15: &'static i32;
let _16: Adt18;
let _17: char;
let _18: &'static u32;
let _19: ((i32, i32), *const Adt18);
let _20: u128;
let _21: bool;
let _22: *const &'static i64;
let _23: [u32; 3];
let _24: f32;
let _25: usize;
let _26: char;
let _27: f64;
let _28: *mut i32;
let _29: *mut *const u16;
let _30: Adt18;
let _31: (&'static f32, [char; 2], u128, [u16; 4]);
let _32: (*const i8, isize);
let _33: i128;
let _34: [isize; 8];
let _35: ();
let _36: ();
{
_4 = 28781_u16 as f64;
_3 = 130_u8 as f32;
_3 = 4135_i16 as f32;
_4 = (-3281252270323802728_i64) as f64;
_3 = 2854878061_u32 as f32;
_4 = (-1825963143_i32) as f64;
_4 = (-9223372036854775808_isize) as f64;
_4 = 52170_u16 as f64;
_3 = 19_u8 as f32;
_3 = 185_u8 as f32;
_4 = 1750_u16 as f64;
_4 = 141502146193265069877838551755336309769_u128 as f64;
_3 = 9223372036854775807_isize as f32;
_3 = _4 as f32;
_3 = 6654632457698230171_i64 as f32;
Call(_3 = fn6(Move(_1), _4, _4, _4, _4, _4, _4, _4, _4, _4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-2332694631760334687_i64) as f64;
_3 = (-1896075079_i32) as f32;
_3 = 24890_u16 as f32;
_4 = 10939803108924386571_u64 as f64;
_3 = (-105959847836368910864591904350822624066_i128) as f32;
_4 = 21_u8 as f64;
_7 = (-1678514958692186317_i64);
_6 = _4 * _4;
_6 = -_4;
_5 = [23_i8,91_i8,79_i8,20_i8,(-78_i8),98_i8,(-2_i8),(-51_i8)];
_9.0 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-43_isize)];
_8 = core::ptr::addr_of!(_3);
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463461696092473076025139 => bb7,
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
_3 = 3_usize as f32;
_9.1 = core::ptr::addr_of!(_9.3);
_5 = [(-114_i8),(-126_i8),59_i8,78_i8,(-3_i8),(-58_i8),119_i8,17_i8];
_12 = !9223372036854775807_isize;
_10 = (*_8);
(*_8) = _10;
_4 = _6;
(*_8) = _10 + _10;
_9.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_9.3 = core::ptr::addr_of_mut!(_7);
_6 = -_4;
_17 = '\u{3f5c7}';
_2 = &_17;
_8 = core::ptr::addr_of!(_13);
_3 = _10;
_13 = -_10;
_5 = [113_i8,(-29_i8),(-109_i8),(-31_i8),(-45_i8),120_i8,(-3_i8),105_i8];
match _7 {
0 => bb1,
1 => bb8,
2 => bb9,
340282366920938463461696092473076025139 => bb11,
_ => bb10
}
}
bb8 = {
_4 = (-2332694631760334687_i64) as f64;
_3 = (-1896075079_i32) as f32;
_3 = 24890_u16 as f32;
_4 = 10939803108924386571_u64 as f64;
_3 = (-105959847836368910864591904350822624066_i128) as f32;
_4 = 21_u8 as f64;
_7 = (-1678514958692186317_i64);
_6 = _4 * _4;
_6 = -_4;
_5 = [23_i8,91_i8,79_i8,20_i8,(-78_i8),98_i8,(-2_i8),(-51_i8)];
_9.0 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-43_isize)];
_8 = core::ptr::addr_of!(_3);
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463461696092473076025139 => bb7,
_ => bb6
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_16 = Adt18::Variant1 { fld0: false,fld1: (*_2),fld2: 3331097098_u32,fld3: 24543_u16,fld4: (-69296385122970773196677375618372804264_i128),fld5: 14466347657461155920_usize,fld6: 248166190484030951013602033917420906962_u128 };
place!(Field::<i128>(Variant(_16, 1), 4)) = !145187909583505480558569805188992269557_i128;
_19.0.1 = (-207362243_i32) << _7;
_2 = &_17;
_18 = &place!(Field::<u32>(Variant(_16, 1), 2));
_2 = &place!(Field::<char>(Variant(_16, 1), 1));
place!(Field::<usize>(Variant(_16, 1), 5)) = 4201686693677269917_usize | 86525467015002138_usize;
place!(Field::<bool>(Variant(_16, 1), 0)) = true;
_11 = [37215_u16,56196_u16,51714_u16,51718_u16];
_13 = (-21_i8) as f32;
_14 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15 = &_19.0.1;
_9.1 = core::ptr::addr_of!(_9.3);
RET = core::ptr::addr_of!(place!(Field::<u16>(Variant(_16, 1), 3)));
_18 = &(*_18);
(*RET) = !11771_u16;
_18 = &place!(Field::<u32>(Variant(_16, 1), 2));
(*RET) = !54105_u16;
_9.3 = core::ptr::addr_of_mut!(_7);
_2 = &place!(Field::<char>(Variant(_16, 1), 1));
place!(Field::<u128>(Variant(_16, 1), 6)) = 109998956793449499298654619756446244673_u128 >> _7;
Goto(bb12)
}
bb12 = {
_19.0.1 = !1983439349_i32;
_11 = [(*RET),(*RET),(*RET),(*RET)];
_14 = [_12,_12,_12,_12,_12,_12,_12,_12];
_16 = Adt18::Variant1 { fld0: false,fld1: _17,fld2: 2989664232_u32,fld3: 49051_u16,fld4: (-98713119186351788814161224243791564907_i128),fld5: 3148372261328842187_usize,fld6: 339186076238655499650782969902222492347_u128 };
place!(Field::<i128>(Variant(_16, 1), 4)) = _10 as i128;
_16 = Adt18::Variant0 { fld0: _7,fld1: 34321_u16,fld2: (-20_i8) };
_13 = _10;
place!(Field::<u16>(Variant(_16, 0), 1)) = 18918_u16;
_5 = [(-22_i8),(-116_i8),(-113_i8),(-115_i8),50_i8,(-62_i8),94_i8,85_i8];
_19.0 = ((-775905129_i32), (-1235684647_i32));
place!(Field::<i8>(Variant(_16, 0), 2)) = 9525427508199057798_u64 as i8;
_2 = &_17;
_21 = !false;
_19.0 = (1951047429_i32, 686204855_i32);
_25 = _3 as usize;
place!(Field::<u16>(Variant(_16, 0), 1)) = 56700_u16;
_19.1 = core::ptr::addr_of!(_16);
_9.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_5 = [Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2),Field::<i8>(Variant(_16, 0), 2)];
_2 = &_26;
_13 = _10;
Goto(bb13)
}
bb13 = {
_9.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_23 = [1154929730_u32,3190627080_u32,2052170851_u32];
_20 = 84404621953212940423416304873432782807_u128;
_19.0 = (176060775_i32, 477707346_i32);
_7 = -Field::<i64>(Variant(_16, 0), 0);
SetDiscriminant(_16, 1);
_26 = _17;
_17 = _26;
_9.2 = core::ptr::addr_of!(_29);
_27 = _25 as f64;
_20 = 227205970798636654046667301630225729308_u128 * 10679333119880975935208565090299889351_u128;
_14 = [_12,_12,_12,_12,_12,_12,_12,_12];
_9.2 = core::ptr::addr_of!(_29);
place!(Field::<i128>(Variant(_16, 1), 4)) = 87925722962390073499413778464596716478_i128 - (-59818704044981156666644285533433596737_i128);
place!(Field::<bool>(Variant(_16, 1), 0)) = _21;
RET = core::ptr::addr_of!(place!(Field::<u16>(Variant(_16, 1), 3)));
place!(Field::<u16>(Variant(_16, 1), 3)) = 13261_u16 - 18795_u16;
_9.3 = core::ptr::addr_of_mut!(_7);
_31.1 = [_26,_26];
(*RET) = 28363_u16 * 6562_u16;
_31.3 = _11;
_26 = _17;
_19.0.1 = !_19.0.0;
match _19.0.0 {
176060775 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_9.2 = core::ptr::addr_of!(_29);
place!(Field::<u16>(Variant(_16, 1), 3)) = 10149_u16;
(*RET) = !2105_u16;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(5_usize, 25_usize, Move(_25), 23_usize, Move(_23), 11_usize, Move(_11), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(5_usize, 21_usize, Move(_21), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *const &'static i64,mut _2: f64,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64,mut _8: f64,mut _9: f64,mut _10: f64,mut _11: f64,mut _12: f64) -> f32 {
mir! {
type RET = f32;
let _13: i128;
let _14: &'static *mut [isize; 8];
let _15: isize;
let _16: &'static *mut [isize; 8];
let _17: *mut i32;
let _18: isize;
let _19: isize;
let _20: char;
let _21: &'static char;
let _22: (i32, i32);
let _23: [char; 2];
let _24: isize;
let _25: [usize; 3];
let _26: i128;
let _27: &'static (Adt18, Adt18);
let _28: *mut *const u16;
let _29: i128;
let _30: char;
let _31: [u8; 2];
let _32: (f64, &'static f32, (Adt18, Adt18), *const &'static u32);
let _33: u8;
let _34: u16;
let _35: *mut *mut i32;
let _36: Adt18;
let _37: [bool; 5];
let _38: u64;
let _39: f64;
let _40: isize;
let _41: (f32, i128, *mut bool, *const i128);
let _42: f64;
let _43: i8;
let _44: (f32, i128, *mut bool, *const i128);
let _45: [usize; 3];
let _46: *const *const f32;
let _47: u128;
let _48: Adt24;
let _49: *const *mut *const u16;
let _50: f64;
let _51: u128;
let _52: ();
let _53: ();
{
RET = 13839_i16 as f32;
RET = _5 as f32;
_8 = _10 + _9;
_2 = -_10;
_2 = _4;
_11 = _6;
_3 = -_5;
_10 = _7;
RET = 3132387511155588514_i64 as f32;
_9 = _8;
_9 = -_8;
_4 = _3;
_3 = -_8;
RET = 18651_u16 as f32;
_7 = _5 + _12;
_10 = -_9;
_9 = _6 - _6;
_4 = -_8;
_8 = _5 - _6;
RET = (-7859656002055120797_i64) as f32;
_11 = -_4;
_8 = _9;
RET = 26912_u16 as f32;
Call(_8 = fn7(Move(_1), _2, _3, _6, _2, _6, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = -_9;
_13 = !(-65853701697070743523127062563939662518_i128);
Goto(bb2)
}
bb2 = {
_9 = _4;
_3 = _7 * _4;
RET = (-7242383139601392762_i64) as f32;
RET = 90_i8 as f32;
RET = 13497380532134365604_u64 as f32;
_9 = -_4;
_9 = 2199577663_u32 as f64;
_13 = 122439730321125663275822343476203924638_i128;
_13 = 121_u8 as i128;
_9 = _3;
_5 = _3;
_9 = _7 + _3;
_9 = _4 - _5;
_12 = 203_u8 as f64;
_4 = 6153293113603407702_i64 as f64;
_13 = (-40870780850431588747979808963931083454_i128);
RET = 17670376255860497697_u64 as f32;
_11 = _8 + _5;
_6 = _11;
_7 = 27250_i16 as f64;
_8 = _5;
_3 = _10 + _6;
Goto(bb3)
}
bb3 = {
RET = 6556223857943226279_usize as f32;
_18 = !(-9223372036854775808_isize);
_4 = _3;
_13 = 164547219736421621510762209443671698438_i128 - (-139907756340647162509950525962862249735_i128);
_5 = _3;
_5 = 23579_u16 as f64;
_10 = _4 - _9;
_9 = -_10;
_19 = _18;
_12 = _10 + _9;
_20 = '\u{b32c5}';
_10 = -_4;
_21 = &_20;
_10 = 4186832502317733031_u64 as f64;
_13 = -2297268386870220989584802441300464551_i128;
_3 = _4 * _11;
_15 = _18 + _19;
_3 = _18 as f64;
_3 = _12 - _9;
_19 = 166791693_u32 as isize;
_22.0 = 1278420630_i32 & 1170650962_i32;
_19 = false as isize;
_25 = [9603627220029504135_usize,4_usize,15481501381924828368_usize];
RET = 12241797069361411672_u64 as f32;
Goto(bb4)
}
bb4 = {
_4 = 11526_u16 as f64;
_20 = '\u{1659e}';
_19 = -_15;
_17 = core::ptr::addr_of_mut!(_22.1);
(*_17) = _22.0 * _22.0;
_23 = [_20,_20];
_2 = 3236033501165539199_i64 as f64;
_9 = _3 * _12;
_24 = !_15;
_18 = _19 + _19;
_9 = _13 as f64;
_17 = core::ptr::addr_of_mut!(_22.1);
_19 = 16944_i16 as isize;
_9 = _12;
_13 = -69013501973935077358432690234146725025_i128;
_26 = !_13;
_25 = [5_usize,5_usize,1096945882262999943_usize];
(*_17) = _22.0 * _22.0;
_15 = _12 as isize;
Call(_22.1 = fn18(_12, _3, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = -_9;
_18 = _15;
_3 = _12;
(*_17) = !_22.0;
Call(_8 = core::intrinsics::fmaf64(_3, _12, _9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = _3 - _3;
_5 = _10 * _10;
_13 = !_26;
_21 = &_20;
_18 = RET as isize;
_9 = (*_17) as f64;
_23 = [(*_21),(*_21)];
_5 = -_8;
Goto(bb7)
}
bb7 = {
RET = 32781_u16 as f32;
(*_17) = _22.0 + _22.0;
_4 = _10;
_10 = (-3683097739699204100_i64) as f64;
_8 = -_7;
_22.1 = -_22.0;
_30 = (*_21);
_7 = _4;
_18 = _15 * _15;
_10 = (-1394102508455571858_i64) as f64;
_24 = _18 >> _18;
_9 = _4 * _7;
_32.1 = &RET;
_19 = !_24;
_31 = [5_u8,99_u8];
_26 = !_13;
_29 = _26 << _15;
_20 = _30;
Goto(bb8)
}
bb8 = {
_35 = core::ptr::addr_of_mut!(_17);
_21 = &_20;
_8 = 8135973027609609163_i64 as f64;
_2 = _7 * _9;
RET = 48407_u16 as f32;
_33 = 33_u8;
(*_17) = _22.0 | _22.0;
_15 = _19 - _19;
_32.1 = &RET;
_7 = 3966393525_u32 as f64;
_11 = _3 + _9;
_18 = _24;
_32.1 = &RET;
_26 = RET as i128;
Goto(bb9)
}
bb9 = {
_24 = _15 - _15;
_30 = _20;
_10 = _11;
_35 = core::ptr::addr_of_mut!(_17);
_39 = _2 + _9;
_9 = 4081347262_u32 as f64;
_15 = _18 >> _18;
_37 = [false,false,true,true,true];
_4 = 3359_i16 as f64;
_27 = &_32.2;
_32.2.0 = Adt18::Variant0 { fld0: 8492423101616811599_i64,fld1: 25355_u16,fld2: (-10_i8) };
(*_35) = core::ptr::addr_of_mut!((*_17));
_32.1 = &RET;
_18 = 1353573168_u32 as isize;
(*_35) = core::ptr::addr_of_mut!((*_17));
_34 = 35185_u16 | 18462_u16;
_23 = [_20,(*_21)];
_20 = _30;
match _33 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb7,
33 => bb10,
_ => bb5
}
}
bb10 = {
_19 = _15 - _15;
_20 = _30;
_41.3 = core::ptr::addr_of!(_13);
_3 = _11;
place!(Field::<u16>(Variant(_32.2.0, 0), 1)) = _34 & _34;
_4 = -_3;
_37 = [false,false,false,true,false];
_5 = Field::<u16>(Variant(_32.2.0, 0), 1) as f64;
_22 = (1977399385_i32, (-1106655774_i32));
_38 = 17383438731074936161_u64;
_36 = Adt18::Variant2 { fld0: true,fld1: 376120768_u32,fld2: _33,fld3: _29,fld4: (-24597_i16),fld5: 308926143259091041471662022129616518953_u128,fld6: 626532293560978307_i64 };
_37 = [false,false,false,false,true];
place!(Field::<i16>(Variant(_36, 2), 4)) = 2_usize as i16;
_23 = [_20,_30];
Call(_41 = fn19(Move(_27), _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_31 = [_33,_33];
_26 = !Field::<i128>(Variant(_36, 2), 3);
place!(Field::<u128>(Variant(_36, 2), 5)) = (*_17) as u128;
_32.2.1 = Adt18::Variant2 { fld0: true,fld1: 1561963269_u32,fld2: _33,fld3: Field::<i128>(Variant(_36, 2), 3),fld4: Field::<i16>(Variant(_36, 2), 4),fld5: Field::<u128>(Variant(_36, 2), 5),fld6: (-6286625814396217083_i64) };
place!(Field::<i8>(Variant(_32.2.0, 0), 2)) = _33 as i8;
place!(Field::<bool>(Variant(_32.2.1, 2), 0)) = !true;
_41.0 = _34 as f32;
(*_17) = _22.0;
_8 = -_11;
place!(Field::<u32>(Variant(_32.2.1, 2), 1)) = Field::<i8>(Variant(_32.2.0, 0), 2) as u32;
_34 = Field::<u16>(Variant(_32.2.0, 0), 1) - Field::<u16>(Variant(_32.2.0, 0), 1);
place!(Field::<u128>(Variant(_36, 2), 5)) = !Field::<u128>(Variant(_32.2.1, 2), 5);
_35 = core::ptr::addr_of_mut!((*_35));
_32.0 = -_4;
Goto(bb12)
}
bb12 = {
place!(Field::<u8>(Variant(_36, 2), 2)) = !_33;
_35 = core::ptr::addr_of_mut!(_17);
_2 = _32.0;
_42 = -_32.0;
_26 = Field::<i128>(Variant(_32.2.1, 2), 3) & Field::<i128>(Variant(_36, 2), 3);
_40 = _19;
place!(Field::<i8>(Variant(_32.2.0, 0), 2)) = Field::<u128>(Variant(_32.2.1, 2), 5) as i8;
place!(Field::<i8>(Variant(_32.2.0, 0), 2)) = 122_i8 - 125_i8;
place!(Field::<i128>(Variant(_36, 2), 3)) = -_26;
place!(Field::<bool>(Variant(_36, 2), 0)) = Field::<bool>(Variant(_32.2.1, 2), 0) ^ Field::<bool>(Variant(_32.2.1, 2), 0);
_2 = Field::<i16>(Variant(_32.2.1, 2), 4) as f64;
_42 = _10 + _32.0;
_33 = Field::<u8>(Variant(_36, 2), 2) * Field::<u8>(Variant(_32.2.1, 2), 2);
_3 = -_42;
_27 = &_32.2;
place!(Field::<u32>(Variant(_32.2.1, 2), 1)) = !3751671817_u32;
place!(Field::<i128>(Variant(_36, 2), 3)) = Field::<i128>(Variant(_32.2.1, 2), 3);
_32.0 = -_3;
_44.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant((*_27).1, 2), 0)));
_41.3 = core::ptr::addr_of!(_44.1);
_44 = Move(_41);
_21 = &_20;
place!(Field::<u32>(Variant(_36, 2), 1)) = Field::<u32>(Variant(_32.2.1, 2), 1) | Field::<u32>(Variant(_32.2.1, 2), 1);
place!(Field::<u8>(Variant(_32.2.1, 2), 2)) = Field::<u8>(Variant(_36, 2), 2) + Field::<u8>(Variant(_36, 2), 2);
place!(Field::<u32>(Variant(_32.2.1, 2), 1)) = Field::<u32>(Variant(_36, 2), 1);
place!(Field::<i128>(Variant(_36, 2), 3)) = _29;
_33 = Field::<u8>(Variant(_36, 2), 2) + Field::<u8>(Variant(_32.2.1, 2), 2);
Goto(bb13)
}
bb13 = {
_48.fld1 = _30;
_6 = -_32.0;
_32.2.0 = Adt18::Variant1 { fld0: Field::<bool>(Variant((*_27).1, 2), 0),fld1: (*_21),fld2: Field::<u32>(Variant(_36, 2), 1),fld3: _34,fld4: _26,fld5: 6474368534263186906_usize,fld6: Field::<u128>(Variant(_32.2.1, 2), 5) };
_29 = _26;
place!(Field::<i64>(Variant(_32.2.1, 2), 6)) = 7058743426856399836_i64;
_44.0 = Field::<u16>(Variant(_32.2.0, 1), 3) as f32;
_48.fld3 = Adt22::Variant2 { fld0: _44.0,fld1: (*_21),fld2: _4,fld3: _32.2.1 };
Goto(bb14)
}
bb14 = {
_22.1 = _22.0 << _24;
_32.2.0 = _32.2.1;
place!(Field::<u128>(Variant(place!(Field::<Adt18>(Variant(_48.fld3, 2), 3)), 2), 5)) = Field::<u128>(Variant((*_27).1, 2), 5) & Field::<u128>(Variant((*_27).1, 2), 5);
RET = -_44.0;
_51 = !Field::<u128>(Variant(Field::<Adt18>(Variant(_48.fld3, 2), 3), 2), 5);
_41 = Move(_44);
_38 = 10575337172323099127_u64;
place!(Field::<i64>(Variant(_36, 2), 6)) = !Field::<i64>(Variant(_32.2.0, 2), 6);
_42 = Field::<u8>(Variant(_32.2.1, 2), 2) as f64;
(*_35) = core::ptr::addr_of_mut!((*_17));
_44.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant((*_27).1, 2), 0)));
SetDiscriminant(_48.fld3, 0);
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(6_usize, 19_usize, Move(_19), 23_usize, Move(_23), 40_usize, Move(_40), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(6_usize, 22_usize, Move(_22), 34_usize, Move(_34), 26_usize, Move(_26), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_52 = dump_var(6_usize, 20_usize, Move(_20), 53_usize, _53, 53_usize, _53, 53_usize, _53), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const &'static i64,mut _2: f64,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64) -> f64 {
mir! {
type RET = f64;
let _8: *mut bool;
let _9: bool;
let _10: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _11: *mut [char; 2];
let _12: [i16; 7];
let _13: (Adt18, Adt18);
let _14: [i8; 8];
let _15: isize;
let _16: u8;
let _17: usize;
let _18: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _19: i8;
let _20: *const i8;
let _21: usize;
let _22: *const *mut *const u16;
let _23: i16;
let _24: isize;
let _25: isize;
let _26: u16;
let _27: i128;
let _28: (f32, i128, *mut bool, *const i128);
let _29: [i128; 4];
let _30: ();
let _31: ();
{
RET = _2 - _7;
_5 = _3 + RET;
_6 = -_5;
_3 = 4968521661061375133_i64 as f64;
_3 = (-1242137766_i32) as f64;
_9 = false;
_3 = RET * RET;
_7 = -_2;
Goto(bb1)
}
bb1 = {
_3 = 115_i8 as f64;
_8 = core::ptr::addr_of_mut!(_9);
_9 = true;
(*_8) = !false;
_9 = true;
Goto(bb2)
}
bb2 = {
_6 = _3;
_9 = !false;
_6 = -_5;
_3 = _5;
_3 = 93_i8 as f64;
_7 = -_6;
RET = 6092822657468855088_usize as f64;
_2 = (-2187_i16) as f64;
_7 = -_5;
_7 = 104_i8 as f64;
(*_8) = false;
_9 = true | true;
_4 = _5;
_3 = _4 - _6;
_12 = [(-20198_i16),(-14839_i16),(-22530_i16),16977_i16,25296_i16,32323_i16,(-4984_i16)];
_2 = _3;
Goto(bb3)
}
bb3 = {
(*_8) = true;
_3 = _4;
_7 = _4;
_12 = [4647_i16,(-26224_i16),(-10226_i16),(-22913_i16),(-16103_i16),11239_i16,(-19482_i16)];
_8 = core::ptr::addr_of_mut!(_9);
_6 = _5 - _5;
_5 = _6;
_2 = _6 - _5;
_6 = -_2;
Goto(bb4)
}
bb4 = {
(*_8) = false;
_6 = -_2;
_8 = core::ptr::addr_of_mut!((*_8));
RET = 9998_u16 as f64;
_12 = [2433_i16,(-15862_i16),(-17009_i16),10638_i16,12699_i16,(-2587_i16),(-7570_i16)];
(*_8) = true;
_9 = false;
(*_8) = true;
Call(_6 = fn8(Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = -_7;
RET = 43685_u16 as f64;
(*_8) = !false;
_2 = 2013009732427979887_usize as f64;
RET = 7891148115978863830_u64 as f64;
_6 = _5 + _5;
_7 = _6;
_6 = -_7;
_3 = _7;
_4 = 29_u8 as f64;
_3 = _6;
(*_8) = !true;
_8 = core::ptr::addr_of_mut!((*_8));
_5 = (-1539565862_i32) as f64;
_5 = _6 * _6;
Goto(bb6)
}
bb6 = {
_6 = 3741677350665386147_u64 as f64;
_9 = true;
_13.1 = Adt18::Variant2 { fld0: (*_8),fld1: 2489257853_u32,fld2: 62_u8,fld3: (-121894536471968393188281496730341224724_i128),fld4: 30829_i16,fld5: 67094062638505699136758738646465768796_u128,fld6: (-2280310563364802136_i64) };
_2 = _5 + _3;
_13.1 = Adt18::Variant3 { fld0: 121124360854379911540290368131053769897_i128,fld1: (-4133257182053138770_i64),fld2: 3552858063_u32,fld3: 5370084833396570246_u64,fld4: 17086_i16 };
_14 = [99_i8,99_i8,(-127_i8),(-67_i8),(-112_i8),(-124_i8),(-82_i8),105_i8];
place!(Field::<u64>(Variant(_13.1, 3), 3)) = !4345858280296507117_u64;
_6 = _2 * _7;
_2 = -_3;
_4 = -_6;
_16 = 1906837991_i32 as u8;
place!(Field::<i16>(Variant(_13.1, 3), 4)) = -(-3742_i16);
Goto(bb7)
}
bb7 = {
_6 = _5;
_15 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_20 = core::ptr::addr_of!(_19);
Goto(bb8)
}
bb8 = {
_6 = 7_usize as f64;
_18.0.2.0 = Adt18::Variant3 { fld0: (-28336937483906545441589401818649478074_i128),fld1: 1972124079094306485_i64,fld2: 319519331_u32,fld3: Field::<u64>(Variant(_13.1, 3), 3),fld4: Field::<i16>(Variant(_13.1, 3), 4) };
_13.1 = Adt18::Variant1 { fld0: (*_8),fld1: '\u{8e057}',fld2: 685864410_u32,fld3: 24307_u16,fld4: 84191539887419966119548317520671370854_i128,fld5: 3_usize,fld6: 220407505601619247800947043576961117074_u128 };
place!(Field::<u128>(Variant(_13.1, 1), 6)) = 278839052101844460548439792414490121051_u128 ^ 176604123228909922323779166637369751390_u128;
place!(Field::<i128>(Variant(_13.1, 1), 4)) = !(-100462364733156057921040256825735793369_i128);
Goto(bb9)
}
bb9 = {
_13.0 = Adt18::Variant0 { fld0: 1343018319827918720_i64,fld1: 57971_u16,fld2: (-43_i8) };
Goto(bb10)
}
bb10 = {
place!(Field::<u16>(Variant(_13.0, 0), 1)) = !3055_u16;
place!(Field::<char>(Variant(_13.1, 1), 1)) = '\u{58552}';
_24 = _15;
Goto(bb11)
}
bb11 = {
_21 = _15 as usize;
(*_8) = !Field::<bool>(Variant(_13.1, 1), 0);
place!(Field::<u32>(Variant(_13.1, 1), 2)) = _16 as u32;
_26 = _9 as u16;
place!(Field::<u32>(Variant(_18.0.2.0, 3), 2)) = Field::<u32>(Variant(_13.1, 1), 2);
_17 = (-27_i8) as usize;
place!(Field::<i64>(Variant(_13.0, 0), 0)) = !(-414345831134841513_i64);
_3 = -_2;
_21 = _16 as usize;
place!(Field::<u16>(Variant(_13.1, 1), 3)) = Field::<u16>(Variant(_13.0, 0), 1);
Goto(bb12)
}
bb12 = {
_24 = _15;
_18.0.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_13.1, 1), 4),fld1: Field::<i64>(Variant(_13.0, 0), 0),fld2: Field::<u32>(Variant(_13.1, 1), 2),fld3: Field::<u64>(Variant(_18.0.2.0, 3), 3),fld4: Field::<i16>(Variant(_18.0.2.0, 3), 4) };
_2 = _5 + _5;
Goto(bb13)
}
bb13 = {
place!(Field::<u16>(Variant(_13.1, 1), 3)) = (-975872320_i32) as u16;
place!(Field::<u32>(Variant(_18.0.2.1, 3), 2)) = Field::<u32>(Variant(_18.0.2.0, 3), 2);
_4 = _6 - _7;
(*_8) = Field::<bool>(Variant(_13.1, 1), 0);
_19 = 113_i8;
_5 = -_2;
match (*_20) {
0 => bb14,
113 => bb16,
_ => bb15
}
}
bb14 = {
(*_8) = true;
_3 = _4;
_7 = _4;
_12 = [4647_i16,(-26224_i16),(-10226_i16),(-22913_i16),(-16103_i16),11239_i16,(-19482_i16)];
_8 = core::ptr::addr_of_mut!(_9);
_6 = _5 - _5;
_5 = _6;
_2 = _6 - _5;
_6 = -_2;
Goto(bb4)
}
bb15 = {
_6 = 7_usize as f64;
_18.0.2.0 = Adt18::Variant3 { fld0: (-28336937483906545441589401818649478074_i128),fld1: 1972124079094306485_i64,fld2: 319519331_u32,fld3: Field::<u64>(Variant(_13.1, 3), 3),fld4: Field::<i16>(Variant(_13.1, 3), 4) };
_13.1 = Adt18::Variant1 { fld0: (*_8),fld1: '\u{8e057}',fld2: 685864410_u32,fld3: 24307_u16,fld4: 84191539887419966119548317520671370854_i128,fld5: 3_usize,fld6: 220407505601619247800947043576961117074_u128 };
place!(Field::<u128>(Variant(_13.1, 1), 6)) = 278839052101844460548439792414490121051_u128 ^ 176604123228909922323779166637369751390_u128;
place!(Field::<i128>(Variant(_13.1, 1), 4)) = !(-100462364733156057921040256825735793369_i128);
Goto(bb9)
}
bb16 = {
place!(Field::<i128>(Variant(_18.0.2.1, 3), 0)) = (*_8) as i128;
SetDiscriminant(_18.0.2.1, 0);
_17 = !_21;
_3 = _5 * _5;
_5 = -_2;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(7_usize, 16_usize, Move(_16), 15_usize, Move(_15), 17_usize, Move(_17), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(7_usize, 24_usize, Move(_24), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *const &'static i64) -> f64 {
mir! {
type RET = f64;
let _2: [i128; 4];
let _3: isize;
let _4: i64;
let _5: Adt24;
let _6: Adt18;
let _7: f64;
let _8: u16;
let _9: [u32; 3];
let _10: &'static char;
let _11: (*const u16, (isize, &'static u128, usize, [i8; 8]), *const Adt18, (*const i8, isize));
let _12: &'static f32;
let _13: [bool; 5];
let _14: i16;
let _15: f32;
let _16: ();
let _17: ();
{
RET = 118_i8 as f64;
RET = 55090776751029608525574649387972027427_u128 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = (-9223372036854775808_isize) as f64;
RET = (-49_i8) as f64;
RET = 134131293736239944191539043741559514097_u128 as f64;
RET = (-87_isize) as f64;
RET = (-577131244_i32) as f64;
RET = (-6552_i16) as f64;
RET = 210_u8 as f64;
RET = (-27446526216439959087745203656985267871_i128) as f64;
RET = 33918_u16 as f64;
RET = 1205224139_i32 as f64;
RET = 11547895338809891697_u64 as f64;
_2 = [128226357534085508099289553824396138368_i128,(-152092059275891302703362981363354789240_i128),(-169344257342362613929842191860669969417_i128),97097940001947529504357751420959739216_i128];
RET = 5_usize as f64;
RET = 255258660286948411460999584053665761856_u128 as f64;
RET = 676017131_i32 as f64;
RET = (-71_i8) as f64;
_3 = (-30540_i16) as isize;
_2 = [48276359341264958504726885346393922326_i128,24468283636282501690279284405216677565_i128,60734970342881581383078861791216134588_i128,132740624591063233534827324609747007507_i128];
_2 = [(-36265794191208844858624888449009817571_i128),(-93088525802644820515516412595886180310_i128),54993911224336388121818482024614029278_i128,(-108295312109042663514297270380398510234_i128)];
RET = 48_u8 as f64;
Goto(bb1)
}
bb1 = {
RET = (-4103877548552079138_i64) as f64;
_4 = !(-9084872560027916943_i64);
_5.fld1 = '\u{bfb8}';
RET = _4 as f64;
_5.fld0 = (-50867059756142961524837941969906188279_i128);
_5.fld0 = 58683080811204076772062570679353204033_i128;
_5.fld2 = Adt18::Variant3 { fld0: _5.fld0,fld1: _4,fld2: 2525383809_u32,fld3: 10240835490710995651_u64,fld4: (-21180_i16) };
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = !8188153509322685697_u64;
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = !2632501618_u32;
RET = 89_i8 as f64;
Call(_4 = fn9(Move(_1), RET, _5.fld1, _5.fld0, Field::<u32>(Variant(_5.fld2, 3), 2), _5.fld1, _5.fld1, Field::<i128>(Variant(_5.fld2, 3), 0)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.fld1 = '\u{5a0c8}';
_5.fld1 = '\u{d5c1b}';
place!(Field::<i16>(Variant(_5.fld2, 3), 4)) = (-18807_i16);
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = false as u32;
_6 = Adt18::Variant2 { fld0: false,fld1: Field::<u32>(Variant(_5.fld2, 3), 2),fld2: 13_u8,fld3: Field::<i128>(Variant(_5.fld2, 3), 0),fld4: Field::<i16>(Variant(_5.fld2, 3), 4),fld5: 153475521624045984095099117642062560424_u128,fld6: _4 };
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = 2674055435587475748_u64;
SetDiscriminant(_5.fld2, 3);
place!(Field::<u32>(Variant(_6, 2), 1)) = !1533203624_u32;
_5.fld2 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_6, 2), 3),fld1: _4,fld2: Field::<u32>(Variant(_6, 2), 1),fld3: 13951099340299695453_u64,fld4: Field::<i16>(Variant(_6, 2), 4) };
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = 18011202995894349116_u64;
match Field::<i16>(Variant(_6, 2), 4) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768192649 => bb7,
_ => bb6
}
}
bb3 = {
RET = (-4103877548552079138_i64) as f64;
_4 = !(-9084872560027916943_i64);
_5.fld1 = '\u{bfb8}';
RET = _4 as f64;
_5.fld0 = (-50867059756142961524837941969906188279_i128);
_5.fld0 = 58683080811204076772062570679353204033_i128;
_5.fld2 = Adt18::Variant3 { fld0: _5.fld0,fld1: _4,fld2: 2525383809_u32,fld3: 10240835490710995651_u64,fld4: (-21180_i16) };
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = !8188153509322685697_u64;
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = !2632501618_u32;
RET = 89_i8 as f64;
Call(_4 = fn9(Move(_1), RET, _5.fld1, _5.fld0, Field::<u32>(Variant(_5.fld2, 3), 2), _5.fld1, _5.fld1, Field::<i128>(Variant(_5.fld2, 3), 0)), ReturnTo(bb2), UnwindUnreachable())
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
_2 = [Field::<i128>(Variant(_5.fld2, 3), 0),Field::<i128>(Variant(_6, 2), 3),Field::<i128>(Variant(_6, 2), 3),Field::<i128>(Variant(_6, 2), 3)];
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = 348747426893791482_u64 * 14014161821891821162_u64;
place!(Field::<u8>(Variant(_6, 2), 2)) = 294786866363934830807232594421755818179_u128 as u8;
_6 = _5.fld2;
_5.fld0 = Field::<i128>(Variant(_5.fld2, 3), 0) << Field::<u32>(Variant(_6, 3), 2);
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = 20707_u16 as u32;
_7 = RET;
Call(RET = core::intrinsics::transmute(Field::<i64>(Variant(_6, 3), 1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<i128>(Variant(_5.fld2, 3), 0)) = -_5.fld0;
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = Field::<u32>(Variant(_6, 3), 2);
_4 = Field::<i64>(Variant(_6, 3), 1);
_5.fld1 = '\u{2ce0c}';
place!(Field::<i16>(Variant(_6, 3), 4)) = -Field::<i16>(Variant(_5.fld2, 3), 4);
_5.fld0 = -Field::<i128>(Variant(_5.fld2, 3), 0);
_6 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_5.fld2, 3), 0),fld1: Field::<i64>(Variant(_5.fld2, 3), 1),fld2: Field::<u32>(Variant(_5.fld2, 3), 2),fld3: Field::<u64>(Variant(_5.fld2, 3), 3),fld4: Field::<i16>(Variant(_5.fld2, 3), 4) };
place!(Field::<i64>(Variant(_5.fld2, 3), 1)) = _7 as i64;
_3 = 28_isize << Field::<u32>(Variant(_5.fld2, 3), 2);
_5.fld2 = _6;
_3 = _7 as isize;
match Field::<i16>(Variant(_6, 3), 4) {
0 => bb9,
1 => bb10,
340282366920938463463374607431768192649 => bb12,
_ => bb11
}
}
bb9 = {
_2 = [Field::<i128>(Variant(_5.fld2, 3), 0),Field::<i128>(Variant(_6, 2), 3),Field::<i128>(Variant(_6, 2), 3),Field::<i128>(Variant(_6, 2), 3)];
place!(Field::<u64>(Variant(_5.fld2, 3), 3)) = 348747426893791482_u64 * 14014161821891821162_u64;
place!(Field::<u8>(Variant(_6, 2), 2)) = 294786866363934830807232594421755818179_u128 as u8;
_6 = _5.fld2;
_5.fld0 = Field::<i128>(Variant(_5.fld2, 3), 0) << Field::<u32>(Variant(_6, 3), 2);
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = 20707_u16 as u32;
_7 = RET;
Call(RET = core::intrinsics::transmute(Field::<i64>(Variant(_6, 3), 1)), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_8 = _7 as u16;
RET = _7;
place!(Field::<u64>(Variant(_6, 3), 3)) = Field::<u64>(Variant(_5.fld2, 3), 3) | Field::<u64>(Variant(_5.fld2, 3), 3);
_8 = 64388_u16;
place!(Field::<i128>(Variant(_5.fld2, 3), 0)) = _5.fld0 - Field::<i128>(Variant(_6, 3), 0);
_7 = RET;
place!(Field::<i64>(Variant(_6, 3), 1)) = Field::<i64>(Variant(_5.fld2, 3), 1) - Field::<i64>(Variant(_5.fld2, 3), 1);
place!(Field::<u32>(Variant(_5.fld2, 3), 2)) = Field::<u32>(Variant(_6, 3), 2);
_8 = 42122_u16;
Call(place!(Field::<i128>(Variant(_6, 3), 0)) = core::intrinsics::transmute(Field::<i128>(Variant(_5.fld2, 3), 0)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_11.0 = core::ptr::addr_of!(_8);
SetDiscriminant(_5.fld2, 1);
_5.fld0 = !Field::<i128>(Variant(_6, 3), 0);
Goto(bb14)
}
bb14 = {
place!(Field::<i16>(Variant(_6, 3), 4)) = (-31522_i16) | (-27617_i16);
_3 = !(-9223372036854775808_isize);
place!(Field::<i128>(Variant(_5.fld2, 1), 4)) = Field::<i128>(Variant(_6, 3), 0);
place!(Field::<usize>(Variant(_5.fld2, 1), 5)) = !4581589425310954066_usize;
_10 = &_5.fld1;
_11.1.2 = Field::<usize>(Variant(_5.fld2, 1), 5);
place!(Field::<u32>(Variant(_5.fld2, 1), 2)) = Field::<u32>(Variant(_6, 3), 2) << _5.fld0;
place!(Field::<u128>(Variant(_5.fld2, 1), 6)) = 35500236766515839000140690343340221672_u128;
_11.1.1 = &place!(Field::<u128>(Variant(_5.fld2, 1), 6));
place!(Field::<i128>(Variant(_6, 3), 0)) = Field::<i128>(Variant(_5.fld2, 1), 4) & _5.fld0;
place!(Field::<usize>(Variant(_5.fld2, 1), 5)) = _11.1.2;
place!(Field::<u128>(Variant(_5.fld2, 1), 6)) = 233660090296457849182207890212418437912_u128 * 326101642107134974034710494091499030118_u128;
_11.1.0 = Field::<i128>(Variant(_6, 3), 0) as isize;
place!(Field::<i128>(Variant(_5.fld2, 1), 4)) = _11.1.0 as i128;
_5.fld1 = '\u{a47c7}';
_11.1.1 = &place!(Field::<u128>(Variant(_5.fld2, 1), 6));
_3 = Field::<i64>(Variant(_6, 3), 1) as isize;
_11.1.2 = false as usize;
_5.fld2 = Adt18::Variant1 { fld0: false,fld1: _5.fld1,fld2: Field::<u32>(Variant(_6, 3), 2),fld3: _8,fld4: _5.fld0,fld5: _11.1.2,fld6: 3803579174438447919468350158818141095_u128 };
place!(Field::<u128>(Variant(_5.fld2, 1), 6)) = 302038704024424051284324786793941937742_u128;
RET = _7;
place!(Field::<char>(Variant(_5.fld2, 1), 1)) = _5.fld1;
_10 = &place!(Field::<char>(Variant(_5.fld2, 1), 1));
_10 = &(*_10);
_13 = [false,false,false,true,true];
_9 = [Field::<u32>(Variant(_6, 3), 2),Field::<u32>(Variant(_5.fld2, 1), 2),Field::<u32>(Variant(_5.fld2, 1), 2)];
_11.1.2 = 133_u8 as usize;
place!(Field::<usize>(Variant(_5.fld2, 1), 5)) = _11.1.2 + _11.1.2;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(8_usize, 3_usize, Move(_3), 13_usize, Move(_13), 8_usize, Move(_8), 17_usize, _17), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *const &'static i64,mut _2: f64,mut _3: char,mut _4: i128,mut _5: u32,mut _6: char,mut _7: char,mut _8: i128) -> i64 {
mir! {
type RET = i64;
let _9: isize;
let _10: u128;
let _11: bool;
let _12: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _13: (isize,);
let _14: bool;
let _15: f64;
let _16: u64;
let _17: *mut [char; 2];
let _18: isize;
let _19: *const i8;
let _20: *mut bool;
let _21: [u128; 5];
let _22: u128;
let _23: Adt75;
let _24: ((i32, i32), *const Adt18);
let _25: usize;
let _26: *mut *const u16;
let _27: bool;
let _28: *const usize;
let _29: i64;
let _30: *mut [char; 2];
let _31: f64;
let _32: i32;
let _33: (f32, i16, *mut bool, char);
let _34: bool;
let _35: [u128; 5];
let _36: i32;
let _37: ();
let _38: ();
{
RET = -(-3464369661236464231_i64);
_2 = (-9223372036854775808_isize) as f64;
_4 = _8 ^ _8;
RET = !(-7685464359991066043_i64);
_9 = !54_isize;
_6 = _7;
_5 = 1217462807_u32 - 1001277188_u32;
_3 = _7;
RET = 5976241756225799165_i64 >> _4;
_7 = _3;
RET = _4 as i64;
_3 = _7;
RET = _5 as i64;
_6 = _7;
RET = 3443741178870064948_i64;
_2 = 11839255966378532874_u64 as f64;
RET = -158910171356133715_i64;
_4 = 72_i8 as i128;
_8 = _4;
_4 = _8;
_10 = 36000664759325415645515940755012031887_u128;
_9 = !12_isize;
_8 = _4 & _4;
_8 = -_4;
RET = !(-737244670243010819_i64);
RET = true as i64;
_3 = _6;
_5 = 2268148229_u32 >> _9;
_3 = _7;
Goto(bb1)
}
bb1 = {
_4 = 9345208662050851347_u64 as i128;
_9 = 95_i8 as isize;
_11 = !true;
_14 = !_11;
_11 = _3 > _6;
_4 = -_8;
_10 = 4457137148158656889_usize as u128;
_9 = (-5300_i16) as isize;
_13.0 = _9 | _9;
_16 = !548039995927231766_u64;
_13 = (_9,);
RET = 6684286359167197047_usize as i64;
Goto(bb2)
}
bb2 = {
_12.0.0 = 243_u8 as f64;
_13.0 = _9 + _9;
RET = 8484403911848033120_i64 ^ 7244035355822551742_i64;
_8 = _4;
_6 = _3;
_12.0.0 = -_2;
_12.0.2.0 = Adt18::Variant0 { fld0: RET,fld1: 32281_u16,fld2: 67_i8 };
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = !31684_u16;
_4 = _8;
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = 17327_u16 | 38203_u16;
_14 = _11;
_5 = 2211232697_u32 + 2117564750_u32;
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _14 as u16;
_7 = _3;
place!(Field::<i8>(Variant(_12.0.2.0, 0), 2)) = 8_i8 - 22_i8;
Goto(bb3)
}
bb3 = {
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _12.0.0 as u16;
SetDiscriminant(_12.0.2.0, 1);
place!(Field::<bool>(Variant(_12.0.2.0, 1), 0)) = _14;
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 110_u8,fld3: _8,fld4: 7657_i16,fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 254_u8,fld3: _4,fld4: (-3423_i16),fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _3,fld2: _5,fld3: 52322_u16,fld4: _4,fld5: 4_usize,fld6: _10 };
_3 = Field::<char>(Variant(_12.0.2.1, 1), 1);
_4 = _8 << _5;
_12.0.2.1 = Adt18::Variant0 { fld0: RET,fld1: 43300_u16,fld2: (-109_i8) };
_12.0.0 = 45816_u16 as f64;
Goto(bb4)
}
bb4 = {
place!(Field::<u16>(Variant(_12.0.2.1, 0), 1)) = !27633_u16;
_21 = [_10,_10,_10,_10,_10];
place!(Field::<i128>(Variant(_12.0.2.0, 1), 4)) = _4 - _4;
_12.0.2.1 = Adt18::Variant1 { fld0: _11,fld1: _6,fld2: _5,fld3: 12590_u16,fld4: _4,fld5: 4871935718598968165_usize,fld6: _10 };
_2 = _12.0.0;
_12.0.0 = _5 as f64;
_25 = 209_u8 as usize;
_16 = Field::<bool>(Variant(_12.0.2.0, 1), 0) as u64;
place!(Field::<u128>(Variant(_12.0.2.0, 1), 6)) = Field::<u128>(Variant(_12.0.2.1, 1), 6);
RET = !3293066622858720697_i64;
_15 = -_12.0.0;
_24.0 = (1526581157_i32, (-1666956852_i32));
place!(Field::<u16>(Variant(_12.0.2.1, 1), 3)) = _25 as u16;
place!(Field::<usize>(Variant(_12.0.2.0, 1), 5)) = _25;
place!(Field::<i128>(Variant(_12.0.2.0, 1), 4)) = _4;
place!(Field::<char>(Variant(_12.0.2.0, 1), 1)) = _3;
Call(_13.0 = fn10(Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12.0.2.0 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.1, 1), 0),fld1: _6,fld2: Field::<u32>(Variant(_12.0.2.1, 1), 2),fld3: Field::<u16>(Variant(_12.0.2.1, 1), 3),fld4: _8,fld5: _25,fld6: _10 };
place!(Field::<usize>(Variant(_12.0.2.0, 1), 5)) = _25;
_25 = Field::<usize>(Variant(_12.0.2.0, 1), 5);
_13.0 = Field::<u16>(Variant(_12.0.2.1, 1), 3) as isize;
_24.1 = core::ptr::addr_of!(_12.0.2.1);
_27 = _14;
place!(Field::<char>(Variant(_12.0.2.1, 1), 1)) = Field::<char>(Variant(_12.0.2.0, 1), 1);
_9 = _6 as isize;
match _24.0.0 {
0 => bb6,
1 => bb7,
1526581157 => bb9,
_ => bb8
}
}
bb6 = {
_4 = 9345208662050851347_u64 as i128;
_9 = 95_i8 as isize;
_11 = !true;
_14 = !_11;
_11 = _3 > _6;
_4 = -_8;
_10 = 4457137148158656889_usize as u128;
_9 = (-5300_i16) as isize;
_13.0 = _9 | _9;
_16 = !548039995927231766_u64;
_13 = (_9,);
RET = 6684286359167197047_usize as i64;
Goto(bb2)
}
bb7 = {
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _12.0.0 as u16;
SetDiscriminant(_12.0.2.0, 1);
place!(Field::<bool>(Variant(_12.0.2.0, 1), 0)) = _14;
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 110_u8,fld3: _8,fld4: 7657_i16,fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 254_u8,fld3: _4,fld4: (-3423_i16),fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _3,fld2: _5,fld3: 52322_u16,fld4: _4,fld5: 4_usize,fld6: _10 };
_3 = Field::<char>(Variant(_12.0.2.1, 1), 1);
_4 = _8 << _5;
_12.0.2.1 = Adt18::Variant0 { fld0: RET,fld1: 43300_u16,fld2: (-109_i8) };
_12.0.0 = 45816_u16 as f64;
Goto(bb4)
}
bb8 = {
_12.0.0 = 243_u8 as f64;
_13.0 = _9 + _9;
RET = 8484403911848033120_i64 ^ 7244035355822551742_i64;
_8 = _4;
_6 = _3;
_12.0.0 = -_2;
_12.0.2.0 = Adt18::Variant0 { fld0: RET,fld1: 32281_u16,fld2: 67_i8 };
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = !31684_u16;
_4 = _8;
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = 17327_u16 | 38203_u16;
_14 = _11;
_5 = 2211232697_u32 + 2117564750_u32;
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _14 as u16;
_7 = _3;
place!(Field::<i8>(Variant(_12.0.2.0, 0), 2)) = 8_i8 - 22_i8;
Goto(bb3)
}
bb9 = {
place!(Field::<u16>(Variant(_12.0.2.1, 1), 3)) = Field::<u16>(Variant(_12.0.2.0, 1), 3);
_24.0.0 = !_24.0.1;
_12.0.2.0 = Adt18::Variant0 { fld0: RET,fld1: Field::<u16>(Variant(_12.0.2.1, 1), 3),fld2: (-100_i8) };
_10 = 47_u8 as u128;
_4 = _8 >> Field::<u32>(Variant(_12.0.2.1, 1), 2);
_24.1 = core::ptr::addr_of!(_12.0.2.0);
place!(Field::<bool>(Variant(_12.0.2.1, 1), 0)) = !_11;
_13 = (_9,);
_9 = !_13.0;
_16 = !15953939399922381522_u64;
_6 = _3;
place!(Field::<u32>(Variant(_12.0.2.1, 1), 2)) = !_5;
_10 = Field::<u128>(Variant(_12.0.2.1, 1), 6);
_18 = _9 ^ _9;
place!(Field::<usize>(Variant(_12.0.2.1, 1), 5)) = !_25;
place!(Field::<u16>(Variant(_12.0.2.1, 1), 3)) = Field::<u16>(Variant(_12.0.2.0, 0), 1) << _4;
_22 = _10 | Field::<u128>(Variant(_12.0.2.1, 1), 6);
_20 = core::ptr::addr_of_mut!(_14);
_14 = _11;
(*_20) = !Field::<bool>(Variant(_12.0.2.1, 1), 0);
SetDiscriminant(_12.0.2.1, 1);
_9 = _18;
_24.0 = (2027488602_i32, 2105561619_i32);
RET = 220_u8 as i64;
_11 = _4 >= _4;
match _24.0.0 {
0 => bb5,
1 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
2027488602 => bb16,
_ => bb15
}
}
bb10 = {
_4 = 9345208662050851347_u64 as i128;
_9 = 95_i8 as isize;
_11 = !true;
_14 = !_11;
_11 = _3 > _6;
_4 = -_8;
_10 = 4457137148158656889_usize as u128;
_9 = (-5300_i16) as isize;
_13.0 = _9 | _9;
_16 = !548039995927231766_u64;
_13 = (_9,);
RET = 6684286359167197047_usize as i64;
Goto(bb2)
}
bb11 = {
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _12.0.0 as u16;
SetDiscriminant(_12.0.2.0, 1);
place!(Field::<bool>(Variant(_12.0.2.0, 1), 0)) = _14;
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 110_u8,fld3: _8,fld4: 7657_i16,fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 254_u8,fld3: _4,fld4: (-3423_i16),fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _3,fld2: _5,fld3: 52322_u16,fld4: _4,fld5: 4_usize,fld6: _10 };
_3 = Field::<char>(Variant(_12.0.2.1, 1), 1);
_4 = _8 << _5;
_12.0.2.1 = Adt18::Variant0 { fld0: RET,fld1: 43300_u16,fld2: (-109_i8) };
_12.0.0 = 45816_u16 as f64;
Goto(bb4)
}
bb12 = {
_4 = 9345208662050851347_u64 as i128;
_9 = 95_i8 as isize;
_11 = !true;
_14 = !_11;
_11 = _3 > _6;
_4 = -_8;
_10 = 4457137148158656889_usize as u128;
_9 = (-5300_i16) as isize;
_13.0 = _9 | _9;
_16 = !548039995927231766_u64;
_13 = (_9,);
RET = 6684286359167197047_usize as i64;
Goto(bb2)
}
bb13 = {
_12.0.2.0 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.1, 1), 0),fld1: _6,fld2: Field::<u32>(Variant(_12.0.2.1, 1), 2),fld3: Field::<u16>(Variant(_12.0.2.1, 1), 3),fld4: _8,fld5: _25,fld6: _10 };
place!(Field::<usize>(Variant(_12.0.2.0, 1), 5)) = _25;
_25 = Field::<usize>(Variant(_12.0.2.0, 1), 5);
_13.0 = Field::<u16>(Variant(_12.0.2.1, 1), 3) as isize;
_24.1 = core::ptr::addr_of!(_12.0.2.1);
_27 = _14;
place!(Field::<char>(Variant(_12.0.2.1, 1), 1)) = Field::<char>(Variant(_12.0.2.0, 1), 1);
_9 = _6 as isize;
match _24.0.0 {
0 => bb6,
1 => bb7,
1526581157 => bb9,
_ => bb8
}
}
bb14 = {
_12.0.0 = 243_u8 as f64;
_13.0 = _9 + _9;
RET = 8484403911848033120_i64 ^ 7244035355822551742_i64;
_8 = _4;
_6 = _3;
_12.0.0 = -_2;
_12.0.2.0 = Adt18::Variant0 { fld0: RET,fld1: 32281_u16,fld2: 67_i8 };
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = !31684_u16;
_4 = _8;
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = 17327_u16 | 38203_u16;
_14 = _11;
_5 = 2211232697_u32 + 2117564750_u32;
_13 = (_9,);
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _14 as u16;
_7 = _3;
place!(Field::<i8>(Variant(_12.0.2.0, 0), 2)) = 8_i8 - 22_i8;
Goto(bb3)
}
bb15 = {
place!(Field::<u16>(Variant(_12.0.2.0, 0), 1)) = _12.0.0 as u16;
SetDiscriminant(_12.0.2.0, 1);
place!(Field::<bool>(Variant(_12.0.2.0, 1), 0)) = _14;
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 110_u8,fld3: _8,fld4: 7657_i16,fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _5,fld2: 254_u8,fld3: _4,fld4: (-3423_i16),fld5: _10,fld6: RET };
_12.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_12.0.2.0, 1), 0),fld1: _3,fld2: _5,fld3: 52322_u16,fld4: _4,fld5: 4_usize,fld6: _10 };
_3 = Field::<char>(Variant(_12.0.2.1, 1), 1);
_4 = _8 << _5;
_12.0.2.1 = Adt18::Variant0 { fld0: RET,fld1: 43300_u16,fld2: (-109_i8) };
_12.0.0 = 45816_u16 as f64;
Goto(bb4)
}
bb16 = {
RET = Field::<i64>(Variant(_12.0.2.0, 0), 0);
place!(Field::<i8>(Variant(_12.0.2.0, 0), 2)) = (-70_i8);
_8 = _4 | _4;
(*_20) = _4 < _4;
_15 = -_2;
place!(Field::<bool>(Variant(_12.0.2.1, 1), 0)) = (*_20);
_11 = _8 > _8;
place!(Field::<char>(Variant(_12.0.2.1, 1), 1)) = _6;
place!(Field::<u128>(Variant(_12.0.2.1, 1), 6)) = _5 as u128;
_14 = Field::<bool>(Variant(_12.0.2.1, 1), 0);
(*_20) = _11;
_28 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.0.2.1, 1), 5)));
(*_20) = Field::<u16>(Variant(_12.0.2.0, 0), 1) <= Field::<u16>(Variant(_12.0.2.0, 0), 1);
place!(Field::<u16>(Variant(_12.0.2.1, 1), 3)) = !Field::<u16>(Variant(_12.0.2.0, 0), 1);
RET = !Field::<i64>(Variant(_12.0.2.0, 0), 0);
place!(Field::<usize>(Variant(_12.0.2.1, 1), 5)) = 6_u8 as usize;
_12.0.1 = &_33.0;
place!(Field::<i8>(Variant(_12.0.2.0, 0), 2)) = (-114_i8);
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(9_usize, 6_usize, Move(_6), 5_usize, Move(_5), 27_usize, Move(_27), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(9_usize, 14_usize, Move(_14), 4_usize, Move(_4), 8_usize, Move(_8), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *const &'static i64) -> isize {
mir! {
type RET = isize;
let _2: u16;
let _3: [u32; 3];
let _4: f64;
let _5: (&'static f32, [char; 2], u128, [u16; 4]);
let _6: isize;
let _7: *mut i64;
let _8: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _9: u16;
let _10: f64;
let _11: [i128; 4];
let _12: u64;
let _13: [u128; 5];
let _14: (i32, [u32; 3], *const &'static u32);
let _15: *const &'static i64;
let _16: i32;
let _17: f64;
let _18: (isize,);
let _19: bool;
let _20: *const [isize; 8];
let _21: char;
let _22: *mut u8;
let _23: i32;
let _24: *mut i32;
let _25: isize;
let _26: ();
let _27: ();
{
RET = -9223372036854775807_isize;
RET = !(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) * 40_isize;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
RET = 30441_u16 as isize;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize;
RET = !(-9223372036854775808_isize);
RET = (-84_isize);
RET = -(-78_isize);
_3 = [2449843205_u32,2682522414_u32,3492377167_u32];
RET = !9223372036854775807_isize;
_2 = 53176_u16;
_2 = 47996_u16;
RET = 149_u8 as isize;
RET = 125_isize;
RET = 64_isize | 9223372036854775807_isize;
_3 = [2304450496_u32,3944590679_u32,875771163_u32];
_2 = 28129_u16 >> RET;
_4 = 268685501_i32 as f64;
_5.1 = ['\u{74349}','\u{a98a1}'];
RET = !9223372036854775807_isize;
_6 = RET - RET;
_5.3 = [_2,_2,_2,_2];
_5.3 = [_2,_2,_2,_2];
Call(_4 = fn11(Move(_1), RET, _5.1, _5.1, _6, _5.1, _5.3, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !RET;
_5.1 = ['\u{60e1a}','\u{5776b}'];
_4 = 23770_i16 as f64;
RET = -_6;
_5.2 = 79359459436283809078814741348934173895_u128;
_5.3 = [_2,_2,_2,_2];
_5.2 = 139_u8 as u128;
_5.1 = ['\u{9a3a}','\u{10a9e4}'];
_5.2 = !294245797846233073966779483219999240539_u128;
_3 = [3378458026_u32,1094722185_u32,911510673_u32];
_4 = _6 as f64;
_9 = (-1501416050405917621_i64) as u16;
_9 = _2;
RET = 530496769090157726_usize as isize;
_10 = -_4;
_8 = core::ptr::addr_of!(_5);
RET = 44_u8 as isize;
_11 = [70502206390099726188509015388876920810_i128,(-21803824077784084102509296004879959637_i128),108077450280316611391267026409664460905_i128,23544512676627977741154427487598954477_i128];
(*_8).1 = ['\u{ffa3d}','\u{2f5a0}'];
_9 = !_2;
(*_8).2 = 132622190324204607756957432347964390821_u128;
_3 = [2683246792_u32,2532404967_u32,1769661647_u32];
_8 = core::ptr::addr_of!(_5);
(*_8).3 = [_2,_9,_2,_9];
Goto(bb2)
}
bb2 = {
_2 = _9 >> (*_8).2;
_10 = _4;
_5.3 = [_9,_2,_2,_2];
RET = !_6;
(*_8).1 = ['\u{cdf3b}','\u{38cbe}'];
_8 = core::ptr::addr_of!((*_8));
(*_8).1 = ['\u{b6716}','\u{10861f}'];
(*_8).2 = 318853251024150378788887769583571486435_u128;
Goto(bb3)
}
bb3 = {
(*_8).2 = (-2025783968_i32) as u128;
_5.2 = !273311845315382459457405959025610186467_u128;
_3 = [4064248263_u32,819820291_u32,1058600349_u32];
_2 = !_9;
_9 = (-122859517_i32) as u16;
Goto(bb4)
}
bb4 = {
(*_8).3 = [_2,_2,_2,_9];
_10 = _4;
_4 = _9 as f64;
Call(_12 = core::intrinsics::transmute(_5.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5.1 = ['\u{62b70}','\u{104034}'];
_14.1 = _3;
(*_8).1 = ['\u{e46e7}','\u{718e8}'];
_6 = RET + RET;
_11 = [(-129495662237238171136060814122700901083_i128),16872113615033450424047772987973075566_i128,162380831299316465957450359186152492933_i128,(-23608155916058282241699953367113067921_i128)];
_16 = 530787716_i32 + (-526834597_i32);
(*_8).1 = ['\u{9c0f9}','\u{5d7b2}'];
_3 = _14.1;
RET = _6;
_11 = [(-111084447647577389559609413200566832990_i128),109862345861513557414808142022381131854_i128,(-54376711112374861781578001056420842150_i128),(-152203391483173050529748843721147778914_i128)];
_5.2 = !216130595532633322315710921663743949752_u128;
_2 = !_9;
_10 = _4;
_5.3 = [_9,_9,_9,_2];
_9 = _2 >> RET;
_6 = RET;
_14.0 = _16 & _16;
_14.0 = -_16;
_6 = !RET;
_13 = [(*_8).2,_5.2,_5.2,(*_8).2,(*_8).2];
_6 = false as isize;
RET = _6 >> _12;
(*_8).2 = !25713336995527546797283046581039606453_u128;
_3 = [2253934397_u32,3093686826_u32,437677857_u32];
Goto(bb6)
}
bb6 = {
_2 = !_9;
(*_8).3 = [_2,_2,_2,_2];
_17 = _4 + _10;
_9 = _2 + _2;
Goto(bb7)
}
bb7 = {
(*_8).3 = [_2,_2,_2,_9];
_5.3 = [_9,_9,_9,_2];
_14.1 = [375767006_u32,2888886529_u32,2591293441_u32];
_5.1 = ['\u{96071}','\u{556c1}'];
_5.2 = !36513811858240370405875844027312770003_u128;
(*_8).2 = 121_i8 as u128;
_5.2 = '\u{34806}' as u128;
(*_8).3 = [_9,_2,_2,_9];
_16 = (-5454659176235690336_i64) as i32;
_17 = RET as f64;
_6 = _12 as isize;
(*_8).3 = [_9,_9,_9,_2];
Goto(bb8)
}
bb8 = {
_18 = (RET,);
_5.1 = ['\u{74d70}','\u{e2f99}'];
_18.0 = _6 ^ RET;
_5.1 = ['\u{5113b}','\u{e8cba}'];
(*_8).2 = 64652428575392173524202719481009023077_u128;
(*_8).3 = [_9,_9,_2,_9];
_12 = 2661653647312366520_u64 >> _9;
_2 = _9 * _9;
_18.0 = _2 as isize;
_17 = _10;
_5.3 = [_2,_2,_2,_9];
_11 = [(-137121089670201159682814882117498679526_i128),(-102152719730473405939320376584311296838_i128),(-142934167457717580468060301040213391224_i128),103274134651899280437854455924299982377_i128];
Call(_2 = fn12(Move(_8), (*_8).3, _5.3, _18.0, (*_8).3, _18, (*_8).3, _18.0, _18.0, _12, _18), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = 16647076330209359550_u64;
_18.0 = !RET;
_14.0 = -_16;
_4 = _5.2 as f64;
_10 = _4;
_5.3 = [_9,_2,_2,_9];
_5.1 = ['\u{1910d}','\u{fd108}'];
Goto(bb10)
}
bb10 = {
_16 = _14.0;
_13 = [_5.2,_5.2,_5.2,_5.2,_5.2];
_21 = '\u{543c3}';
_4 = (-3113_i16) as f64;
_12 = 18370656099836107987_u64;
_6 = !RET;
RET = _6;
_19 = true;
_18 = (RET,);
_13 = [_5.2,_5.2,_5.2,_5.2,_5.2];
match _5.2 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
64652428575392173524202719481009023077 => bb16,
_ => bb15
}
}
bb11 = {
_12 = 16647076330209359550_u64;
_18.0 = !RET;
_14.0 = -_16;
_4 = _5.2 as f64;
_10 = _4;
_5.3 = [_9,_2,_2,_9];
_5.1 = ['\u{1910d}','\u{fd108}'];
Goto(bb10)
}
bb12 = {
_18 = (RET,);
_5.1 = ['\u{74d70}','\u{e2f99}'];
_18.0 = _6 ^ RET;
_5.1 = ['\u{5113b}','\u{e8cba}'];
(*_8).2 = 64652428575392173524202719481009023077_u128;
(*_8).3 = [_9,_9,_2,_9];
_12 = 2661653647312366520_u64 >> _9;
_2 = _9 * _9;
_18.0 = _2 as isize;
_17 = _10;
_5.3 = [_2,_2,_2,_9];
_11 = [(-137121089670201159682814882117498679526_i128),(-102152719730473405939320376584311296838_i128),(-142934167457717580468060301040213391224_i128),103274134651899280437854455924299982377_i128];
Call(_2 = fn12(Move(_8), (*_8).3, _5.3, _18.0, (*_8).3, _18, (*_8).3, _18.0, _18.0, _12, _18), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
(*_8).2 = (-2025783968_i32) as u128;
_5.2 = !273311845315382459457405959025610186467_u128;
_3 = [4064248263_u32,819820291_u32,1058600349_u32];
_2 = !_9;
_9 = (-122859517_i32) as u16;
Goto(bb4)
}
bb14 = {
(*_8).3 = [_2,_2,_2,_9];
_10 = _4;
_4 = _9 as f64;
Call(_12 = core::intrinsics::transmute(_5.1), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_2 = _9 >> (*_8).2;
_10 = _4;
_5.3 = [_9,_2,_2,_2];
RET = !_6;
(*_8).1 = ['\u{cdf3b}','\u{38cbe}'];
_8 = core::ptr::addr_of!((*_8));
(*_8).1 = ['\u{b6716}','\u{10861f}'];
(*_8).2 = 318853251024150378788887769583571486435_u128;
Goto(bb3)
}
bb16 = {
_23 = _2 as i32;
_3 = _14.1;
RET = !_6;
_4 = 2_usize as f64;
_10 = _23 as f64;
_16 = 1862638791_u32 as i32;
_17 = _10;
_23 = (-9445811676959270866563698290963457218_i128) as i32;
_25 = _5.2 as isize;
_16 = _23;
_10 = -_17;
_14.0 = _23 >> _5.2;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(10_usize, 2_usize, Move(_2), 18_usize, Move(_18), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(10_usize, 21_usize, Move(_21), 16_usize, Move(_16), 27_usize, _27, 27_usize, _27), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *const &'static i64,mut _2: isize,mut _3: [char; 2],mut _4: [char; 2],mut _5: isize,mut _6: [char; 2],mut _7: [u16; 4],mut _8: isize) -> f64 {
mir! {
type RET = f64;
let _9: [u128; 5];
let _10: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _11: ([isize; 8], *const *mut i64, *const *mut *const u16, *mut i64);
let _12: &'static char;
let _13: u64;
let _14: u16;
let _15: &'static i32;
let _16: isize;
let _17: *mut *mut i32;
let _18: *const *mut *const u16;
let _19: [u128; 5];
let _20: usize;
let _21: u64;
let _22: isize;
let _23: Adt75;
let _24: *const Adt18;
let _25: (f64, &'static f32, (Adt18, Adt18), *const &'static u32);
let _26: isize;
let _27: *mut [char; 2];
let _28: [bool; 5];
let _29: [char; 2];
let _30: f32;
let _31: [i16; 7];
let _32: ([isize; 8], *const *mut i64, *const *mut *const u16, *mut i64);
let _33: ();
let _34: ();
{
_5 = _8;
_4 = ['\u{51eab}','\u{4bb9f}'];
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 1229512786_i32 as f64;
_2 = _8 | _8;
RET = 11365213186375628251_usize as f64;
_2 = _5 >> _5;
_8 = !_2;
RET = 126_u8 as f64;
RET = 3_i8 as f64;
_2 = -_5;
_4 = _6;
RET = 34607_u16 as f64;
_4 = _6;
_6 = _4;
_9 = [216596512866817514372051876321289362487_u128,311381771904198224592691177389046864736_u128,292383997940998140117233180004592172459_u128,246119780446224485958513666937136010755_u128,44004720565386703771485368384829357898_u128];
RET = _2 as f64;
_5 = _8;
_2 = !_8;
_6 = ['\u{543ca}','\u{9482b}'];
_2 = _8;
_6 = ['\u{36eb5}','\u{991bf}'];
_2 = _8 + _5;
_4 = _6;
_5 = (-3_i8) as isize;
_4 = _3;
Goto(bb2)
}
bb2 = {
_4 = ['\u{98e23}','\u{2347f}'];
_9 = [266725352919949100899576943066144343780_u128,106285725667690426879644787565688215377_u128,137119568882654054855021575504550624759_u128,157612942385896380087507695112301319381_u128,7575453112406435937810374569039167942_u128];
_2 = 719317948_i32 as isize;
RET = 14685186897629991650_usize as f64;
_3 = _4;
_4 = ['\u{eaed7}','\u{e94b2}'];
Call(_8 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11.0 = [_2,_8,_2,_5,_8,_8,_5,_8];
_9 = [40481567634825086699609037484687260799_u128,65850346936370554123043391751587488518_u128,100327007974832254437209490732581879223_u128,1053105581048043135991279614196813852_u128,339239378932839478188801603206876668252_u128];
_3 = ['\u{3df18}','\u{1d36f}'];
_6 = _4;
_7 = [3867_u16,30146_u16,37703_u16,4244_u16];
_3 = ['\u{38bce}','\u{77fdf}'];
_11.1 = core::ptr::addr_of!(_11.3);
_11.1 = core::ptr::addr_of!(_11.3);
RET = (-61433178535702809748055634471289975246_i128) as f64;
RET = 571_u16 as f64;
_8 = _5 - _5;
_2 = _5 ^ _8;
_4 = ['\u{99d37}','\u{41f4e}'];
_9 = [119580816102302966118258975522665348744_u128,98065245261467233098565273877112928056_u128,152235623916097047427184995207383651962_u128,148318396522423559702076948411358057442_u128,146410596156856152278056822111218205316_u128];
_3 = _6;
_7 = [59376_u16,18547_u16,52315_u16,48540_u16];
_5 = _8;
_4 = ['\u{1074f0}','\u{ce1e1}'];
_14 = _8 as u16;
RET = 123386019933157912685245123871610158351_u128 as f64;
RET = 19255_i16 as f64;
_13 = 3830815636004307735_u64 + 17133568557207713969_u64;
_8 = (-1160488654189475500_i64) as isize;
Goto(bb4)
}
bb4 = {
_7 = [_14,_14,_14,_14];
_6 = ['\u{ba221}','\u{9013b}'];
_6 = ['\u{c463b}','\u{10d50a}'];
RET = _14 as f64;
_7 = [_14,_14,_14,_14];
_6 = _3;
_6 = _4;
RET = 36_i8 as f64;
_9 = [245212688462696643890986005464185491152_u128,73714049300276040394463987532744760718_u128,126133187661695210197109692169138061121_u128,139353535678399653465549663063054910975_u128,238696684055081616729703696963106549030_u128];
_4 = ['\u{f6598}','\u{9eed0}'];
_16 = RET as isize;
_9 = [75368199095126686496891290872456834839_u128,49373375277422706828328686337715948472_u128,175097097758671841695378742471145264388_u128,16884309521393530939224431116721638794_u128,94589803627769766826018451118840843298_u128];
_16 = _5;
_6 = ['\u{d45f}','\u{742d6}'];
_4 = _3;
_13 = 4563597257493611434_u64;
_19 = _9;
_20 = !6_usize;
_8 = _5 | _2;
_21 = !_13;
_7 = [_14,_14,_14,_14];
_21 = _2 as u64;
_9 = [19308613162821581561362588173722264634_u128,234395033964505131387134413120980097439_u128,269155281968628201829477605561001452573_u128,78383711127077469256156305427402678711_u128,134884992342924303459344281381141596356_u128];
RET = 307741472639503498076777685003311917071_u128 as f64;
_21 = !_13;
_19 = [192977658678283012282802169385448798259_u128,243259627101754601384121871870034785746_u128,258858607858109947456086808245007449798_u128,139032116553114460048715324779368460786_u128,279284454425633333494387980891277913644_u128];
match _13 {
0 => bb1,
1 => bb2,
4563597257493611434 => bb5,
_ => bb3
}
}
bb5 = {
_4 = ['\u{9c68d}','\u{11720}'];
_11.0 = [_8,_8,_8,_16,_8,_16,_5,_2];
_6 = ['\u{93ce9}','\u{100762}'];
_4 = ['\u{10b69e}','\u{284d0}'];
_14 = 56456_u16 >> _20;
_19 = [303287892537011622160696285317814441878_u128,244884446298374443716537502141521806279_u128,63160131018510642556967649648134689961_u128,9286437839112826365379788971811136243_u128,237457405753871259899109381008612322300_u128];
RET = 21656_i16 as f64;
_22 = _5 - _2;
_22 = -_8;
_21 = _13 * _13;
_20 = 3_usize >> _8;
RET = 60_u8 as f64;
_9 = _19;
_16 = !_8;
_14 = 50427_u16 * 59324_u16;
_4 = _6;
_2 = !_16;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
4563597257493611434 => bb9,
_ => bb8
}
}
bb6 = {
_7 = [_14,_14,_14,_14];
_6 = ['\u{ba221}','\u{9013b}'];
_6 = ['\u{c463b}','\u{10d50a}'];
RET = _14 as f64;
_7 = [_14,_14,_14,_14];
_6 = _3;
_6 = _4;
RET = 36_i8 as f64;
_9 = [245212688462696643890986005464185491152_u128,73714049300276040394463987532744760718_u128,126133187661695210197109692169138061121_u128,139353535678399653465549663063054910975_u128,238696684055081616729703696963106549030_u128];
_4 = ['\u{f6598}','\u{9eed0}'];
_16 = RET as isize;
_9 = [75368199095126686496891290872456834839_u128,49373375277422706828328686337715948472_u128,175097097758671841695378742471145264388_u128,16884309521393530939224431116721638794_u128,94589803627769766826018451118840843298_u128];
_16 = _5;
_6 = ['\u{d45f}','\u{742d6}'];
_4 = _3;
_13 = 4563597257493611434_u64;
_19 = _9;
_20 = !6_usize;
_8 = _5 | _2;
_21 = !_13;
_7 = [_14,_14,_14,_14];
_21 = _2 as u64;
_9 = [19308613162821581561362588173722264634_u128,234395033964505131387134413120980097439_u128,269155281968628201829477605561001452573_u128,78383711127077469256156305427402678711_u128,134884992342924303459344281381141596356_u128];
RET = 307741472639503498076777685003311917071_u128 as f64;
_21 = !_13;
_19 = [192977658678283012282802169385448798259_u128,243259627101754601384121871870034785746_u128,258858607858109947456086808245007449798_u128,139032116553114460048715324779368460786_u128,279284454425633333494387980891277913644_u128];
match _13 {
0 => bb1,
1 => bb2,
4563597257493611434 => bb5,
_ => bb3
}
}
bb7 = {
_11.0 = [_2,_8,_2,_5,_8,_8,_5,_8];
_9 = [40481567634825086699609037484687260799_u128,65850346936370554123043391751587488518_u128,100327007974832254437209490732581879223_u128,1053105581048043135991279614196813852_u128,339239378932839478188801603206876668252_u128];
_3 = ['\u{3df18}','\u{1d36f}'];
_6 = _4;
_7 = [3867_u16,30146_u16,37703_u16,4244_u16];
_3 = ['\u{38bce}','\u{77fdf}'];
_11.1 = core::ptr::addr_of!(_11.3);
_11.1 = core::ptr::addr_of!(_11.3);
RET = (-61433178535702809748055634471289975246_i128) as f64;
RET = 571_u16 as f64;
_8 = _5 - _5;
_2 = _5 ^ _8;
_4 = ['\u{99d37}','\u{41f4e}'];
_9 = [119580816102302966118258975522665348744_u128,98065245261467233098565273877112928056_u128,152235623916097047427184995207383651962_u128,148318396522423559702076948411358057442_u128,146410596156856152278056822111218205316_u128];
_3 = _6;
_7 = [59376_u16,18547_u16,52315_u16,48540_u16];
_5 = _8;
_4 = ['\u{1074f0}','\u{ce1e1}'];
_14 = _8 as u16;
RET = 123386019933157912685245123871610158351_u128 as f64;
RET = 19255_i16 as f64;
_13 = 3830815636004307735_u64 + 17133568557207713969_u64;
_8 = (-1160488654189475500_i64) as isize;
Goto(bb4)
}
bb8 = {
RET = 1229512786_i32 as f64;
_2 = _8 | _8;
RET = 11365213186375628251_usize as f64;
_2 = _5 >> _5;
_8 = !_2;
RET = 126_u8 as f64;
RET = 3_i8 as f64;
_2 = -_5;
_4 = _6;
RET = 34607_u16 as f64;
_4 = _6;
_6 = _4;
_9 = [216596512866817514372051876321289362487_u128,311381771904198224592691177389046864736_u128,292383997940998140117233180004592172459_u128,246119780446224485958513666937136010755_u128,44004720565386703771485368384829357898_u128];
RET = _2 as f64;
_5 = _8;
_2 = !_8;
_6 = ['\u{543ca}','\u{9482b}'];
_2 = _8;
_6 = ['\u{36eb5}','\u{991bf}'];
_2 = _8 + _5;
_4 = _6;
_5 = (-3_i8) as isize;
_4 = _3;
Goto(bb2)
}
bb9 = {
_6 = _3;
_5 = !_2;
_16 = _22 ^ _8;
_11.0 = [_5,_16,_8,_16,_16,_2,_2,_8];
_14 = _21 as u16;
_21 = 7767496273863255550_i64 as u64;
_22 = _5;
RET = (-258344764_i32) as f64;
_11.1 = core::ptr::addr_of!(_11.3);
_5 = -_16;
_2 = _16;
_13 = _21;
_22 = _5;
_3 = ['\u{e5e9c}','\u{ac2d0}'];
_16 = _20 as isize;
_11.0 = [_22,_22,_8,_22,_2,_16,_22,_8];
_11.1 = core::ptr::addr_of!(_11.3);
_24 = core::ptr::addr_of!(_25.2.1);
_25.0 = RET - RET;
_6 = ['\u{bb6a6}','\u{918ee}'];
_7 = [_14,_14,_14,_14];
_8 = (-73822463972559926477700218366113993558_i128) as isize;
Goto(bb10)
}
bb10 = {
_14 = 4571_u16;
_25.0 = (-54_i8) as f64;
match _14 {
4571 => bb11,
_ => bb7
}
}
bb11 = {
_24 = core::ptr::addr_of!((*_24));
_25.2.0 = Adt18::Variant3 { fld0: (-156167306195252650435958013977831894662_i128),fld1: 3497479435942537209_i64,fld2: 4020459686_u32,fld3: _13,fld4: 15015_i16 };
match _14 {
0 => bb10,
1 => bb12,
2 => bb13,
3 => bb14,
4571 => bb16,
_ => bb15
}
}
bb12 = {
_4 = ['\u{9c68d}','\u{11720}'];
_11.0 = [_8,_8,_8,_16,_8,_16,_5,_2];
_6 = ['\u{93ce9}','\u{100762}'];
_4 = ['\u{10b69e}','\u{284d0}'];
_14 = 56456_u16 >> _20;
_19 = [303287892537011622160696285317814441878_u128,244884446298374443716537502141521806279_u128,63160131018510642556967649648134689961_u128,9286437839112826365379788971811136243_u128,237457405753871259899109381008612322300_u128];
RET = 21656_i16 as f64;
_22 = _5 - _2;
_22 = -_8;
_21 = _13 * _13;
_20 = 3_usize >> _8;
RET = 60_u8 as f64;
_9 = _19;
_16 = !_8;
_14 = 50427_u16 * 59324_u16;
_4 = _6;
_2 = !_16;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
4563597257493611434 => bb9,
_ => bb8
}
}
bb13 = {
_6 = _3;
_5 = !_2;
_16 = _22 ^ _8;
_11.0 = [_5,_16,_8,_16,_16,_2,_2,_8];
_14 = _21 as u16;
_21 = 7767496273863255550_i64 as u64;
_22 = _5;
RET = (-258344764_i32) as f64;
_11.1 = core::ptr::addr_of!(_11.3);
_5 = -_16;
_2 = _16;
_13 = _21;
_22 = _5;
_3 = ['\u{e5e9c}','\u{ac2d0}'];
_16 = _20 as isize;
_11.0 = [_22,_22,_8,_22,_2,_16,_22,_8];
_11.1 = core::ptr::addr_of!(_11.3);
_24 = core::ptr::addr_of!(_25.2.1);
_25.0 = RET - RET;
_6 = ['\u{bb6a6}','\u{918ee}'];
_7 = [_14,_14,_14,_14];
_8 = (-73822463972559926477700218366113993558_i128) as isize;
Goto(bb10)
}
bb14 = {
_7 = [_14,_14,_14,_14];
_6 = ['\u{ba221}','\u{9013b}'];
_6 = ['\u{c463b}','\u{10d50a}'];
RET = _14 as f64;
_7 = [_14,_14,_14,_14];
_6 = _3;
_6 = _4;
RET = 36_i8 as f64;
_9 = [245212688462696643890986005464185491152_u128,73714049300276040394463987532744760718_u128,126133187661695210197109692169138061121_u128,139353535678399653465549663063054910975_u128,238696684055081616729703696963106549030_u128];
_4 = ['\u{f6598}','\u{9eed0}'];
_16 = RET as isize;
_9 = [75368199095126686496891290872456834839_u128,49373375277422706828328686337715948472_u128,175097097758671841695378742471145264388_u128,16884309521393530939224431116721638794_u128,94589803627769766826018451118840843298_u128];
_16 = _5;
_6 = ['\u{d45f}','\u{742d6}'];
_4 = _3;
_13 = 4563597257493611434_u64;
_19 = _9;
_20 = !6_usize;
_8 = _5 | _2;
_21 = !_13;
_7 = [_14,_14,_14,_14];
_21 = _2 as u64;
_9 = [19308613162821581561362588173722264634_u128,234395033964505131387134413120980097439_u128,269155281968628201829477605561001452573_u128,78383711127077469256156305427402678711_u128,134884992342924303459344281381141596356_u128];
RET = 307741472639503498076777685003311917071_u128 as f64;
_21 = !_13;
_19 = [192977658678283012282802169385448798259_u128,243259627101754601384121871870034785746_u128,258858607858109947456086808245007449798_u128,139032116553114460048715324779368460786_u128,279284454425633333494387980891277913644_u128];
match _13 {
0 => bb1,
1 => bb2,
4563597257493611434 => bb5,
_ => bb3
}
}
bb15 = {
_11.0 = [_2,_8,_2,_5,_8,_8,_5,_8];
_9 = [40481567634825086699609037484687260799_u128,65850346936370554123043391751587488518_u128,100327007974832254437209490732581879223_u128,1053105581048043135991279614196813852_u128,339239378932839478188801603206876668252_u128];
_3 = ['\u{3df18}','\u{1d36f}'];
_6 = _4;
_7 = [3867_u16,30146_u16,37703_u16,4244_u16];
_3 = ['\u{38bce}','\u{77fdf}'];
_11.1 = core::ptr::addr_of!(_11.3);
_11.1 = core::ptr::addr_of!(_11.3);
RET = (-61433178535702809748055634471289975246_i128) as f64;
RET = 571_u16 as f64;
_8 = _5 - _5;
_2 = _5 ^ _8;
_4 = ['\u{99d37}','\u{41f4e}'];
_9 = [119580816102302966118258975522665348744_u128,98065245261467233098565273877112928056_u128,152235623916097047427184995207383651962_u128,148318396522423559702076948411358057442_u128,146410596156856152278056822111218205316_u128];
_3 = _6;
_7 = [59376_u16,18547_u16,52315_u16,48540_u16];
_5 = _8;
_4 = ['\u{1074f0}','\u{ce1e1}'];
_14 = _8 as u16;
RET = 123386019933157912685245123871610158351_u128 as f64;
RET = 19255_i16 as f64;
_13 = 3830815636004307735_u64 + 17133568557207713969_u64;
_8 = (-1160488654189475500_i64) as isize;
Goto(bb4)
}
bb16 = {
place!(Field::<u32>(Variant(_25.2.0, 3), 2)) = 255399083538602425869546421823275968773_u128 as u32;
_14 = 33025_u16 >> _20;
_25.2.1 = Adt18::Variant1 { fld0: false,fld1: '\u{c6593}',fld2: Field::<u32>(Variant(_25.2.0, 3), 2),fld3: _14,fld4: (-137536696126991796958492104755010096705_i128),fld5: _20,fld6: 34502478588762576672749187407489850923_u128 };
place!(Field::<usize>(Variant((*_24), 1), 5)) = !_20;
_28 = [true,false,true,false,false];
_31 = [(-26940_i16),(-17547_i16),(-5739_i16),(-28745_i16),29943_i16,27523_i16,17819_i16];
_30 = 96_u8 as f32;
_27 = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant(_25.2.1, 1), 4)) = (-14843660349409459098646330137115900843_i128);
place!(Field::<u128>(Variant((*_24), 1), 6)) = !51206232623561894381097329842690064994_u128;
(*_27) = ['\u{545c3}','\u{a816}'];
place!(Field::<i128>(Variant(_25.2.0, 3), 0)) = Field::<i128>(Variant((*_24), 1), 4);
RET = _25.0 * _25.0;
_21 = _13 << Field::<usize>(Variant(_25.2.1, 1), 5);
(*_24) = Adt18::Variant3 { fld0: Field::<i128>(Variant(_25.2.0, 3), 0),fld1: 1049618977744326542_i64,fld2: Field::<u32>(Variant(_25.2.0, 3), 2),fld3: _21,fld4: 18785_i16 };
_2 = _5;
_32.3 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_25.2.0, 3), 1)));
(*_24) = Adt18::Variant0 { fld0: (-2227964135272474571_i64),fld1: _14,fld2: 70_i8 };
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(11_usize, 2_usize, Move(_2), 31_usize, Move(_31), 20_usize, Move(_20), 28_usize, Move(_28)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(11_usize, 4_usize, Move(_4), 13_usize, Move(_13), 21_usize, Move(_21), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: *const (&'static f32, [char; 2], u128, [u16; 4]),mut _2: [u16; 4],mut _3: [u16; 4],mut _4: isize,mut _5: [u16; 4],mut _6: (isize,),mut _7: [u16; 4],mut _8: isize,mut _9: isize,mut _10: u64,mut _11: (isize,)) -> u16 {
mir! {
type RET = u16;
let _12: &'static (Adt18, Adt18);
let _13: u32;
let _14: i8;
let _15: i32;
let _16: *const bool;
let _17: *mut i64;
let _18: char;
let _19: (*const *mut i64, *const &'static u32, f64);
let _20: isize;
let _21: isize;
let _22: usize;
let _23: (isize,);
let _24: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _25: u64;
let _26: *const usize;
let _27: &'static f32;
let _28: (&'static f32, [char; 2], u128, [u16; 4]);
let _29: &'static [i8; 8];
let _30: [u32; 3];
let _31: bool;
let _32: (*const *mut i64, *const &'static u32, f64);
let _33: u64;
let _34: bool;
let _35: f64;
let _36: (Adt18, Adt18);
let _37: isize;
let _38: i32;
let _39: ();
let _40: ();
{
_4 = _6.0 | _11.0;
_11 = (_4,);
_2 = [19145_u16,35561_u16,24518_u16,11693_u16];
_7 = [57924_u16,26607_u16,15087_u16,54006_u16];
RET = 7591_u16 - 15149_u16;
RET = 2_usize as u16;
_6.0 = 137343788443639529222036941521806998535_i128 as isize;
_9 = _11.0 | _8;
_13 = 640636935_u32;
_13 = 2247137251_u32 & 3545657157_u32;
RET = 1616879147905087761_usize as u16;
RET = 11024_u16 >> _8;
_8 = _9 | _4;
RET = 16511_u16;
_11 = (_9,);
_5 = [RET,RET,RET,RET];
_14 = (-7973354219766912006_i64) as i8;
_6 = _11;
_19.2 = 112139052406473636000303376092877821422_u128 as f64;
_6 = (_8,);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
16511 => bb9,
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
_19.0 = core::ptr::addr_of!(_17);
RET = !24324_u16;
RET = 13025_u16;
_6 = _11;
RET = 58670914419450227121223317437588871998_i128 as u16;
_14 = 123_i8;
RET = 47583_u16;
_20 = _8 >> _8;
_13 = 1720053700_u32;
_22 = 3_usize;
_23 = _11;
_11 = (_20,);
_5[_22] = !_2[_22];
_8 = -_11.0;
_9 = -_8;
RET = _7[_22] << _20;
RET = !_3[_22];
RET = !_3[_22];
_13 = 12398_i16 as u32;
_6 = (_9,);
_15 = (-1864699355_i32);
_2 = _3;
Call(_24 = fn13(Move(_1), _8, _9, _2, _6, _6, _23, _6.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = _3;
RET = 33529_u16 - 32753_u16;
_18 = '\u{a015c}';
_6 = (_11.0,);
_18 = '\u{de5f1}';
_1 = core::ptr::addr_of!(_28);
_28.1 = [_18,_18];
_21 = -_20;
(*_1).1 = [_18,_18];
_23.0 = !_6.0;
(*_1).2 = !278415813482266749786495989303027941166_u128;
_28.2 = _10 as u128;
Goto(bb11)
}
bb11 = {
_21 = _13 as isize;
_28.2 = _15 as u128;
_19.0 = core::ptr::addr_of!(_17);
_4 = _23.0;
_28.3 = _2;
_13 = 3608559972_u32;
_21 = _6.0 - _11.0;
_9 = !_20;
_2 = [RET,RET,RET,RET];
_13 = 3977804875_u32 ^ 143164389_u32;
_30 = [_13,_13,_13];
_2 = [RET,RET,RET,RET];
match _14 {
0 => bb1,
1 => bb10,
2 => bb8,
3 => bb4,
123 => bb13,
_ => bb12
}
}
bb12 = {
_19.0 = core::ptr::addr_of!(_17);
RET = !24324_u16;
RET = 13025_u16;
_6 = _11;
RET = 58670914419450227121223317437588871998_i128 as u16;
_14 = 123_i8;
RET = 47583_u16;
_20 = _8 >> _8;
_13 = 1720053700_u32;
_22 = 3_usize;
_23 = _11;
_11 = (_20,);
_5[_22] = !_2[_22];
_8 = -_11.0;
_9 = -_8;
RET = _7[_22] << _20;
RET = !_3[_22];
RET = !_3[_22];
_13 = 12398_i16 as u32;
_6 = (_9,);
_15 = (-1864699355_i32);
_2 = _3;
Call(_24 = fn13(Move(_1), _8, _9, _2, _6, _6, _23, _6.0), ReturnTo(bb10), UnwindUnreachable())
}
bb13 = {
(*_1).3 = [RET,RET,RET,RET];
_32.0 = core::ptr::addr_of!(_17);
_5 = _7;
_2 = (*_1).3;
_4 = -_21;
_32.2 = -_19.2;
(*_1).2 = 29711672235364719183807914249874135690_u128 & 57331130580302385797867914835365942435_u128;
Goto(bb14)
}
bb14 = {
_34 = !true;
_28.3 = [RET,RET,RET,RET];
_24 = core::ptr::addr_of!((*_1));
_9 = _21;
(*_1).2 = 107508650840451225071440538786649727843_u128;
_19.0 = core::ptr::addr_of!(_17);
_25 = _10 >> _11.0;
_28.1 = [_18,_18];
_12 = &_36;
_6.0 = _20 | _11.0;
_19.0 = core::ptr::addr_of!(_17);
_33 = _25 << _9;
_30 = [_13,_13,_13];
_1 = Move(_24);
_23.0 = -_21;
_6 = _11;
_5 = [RET,RET,RET,RET];
_24 = Move(_1);
_1 = core::ptr::addr_of!(_28);
_28.2 = 91701778251620781788837109549583423668_u128 & 35639828003749522607591397513002271001_u128;
_36.0 = Adt18::Variant2 { fld0: _34,fld1: _13,fld2: 30_u8,fld3: (-34694897970909768894730790949473601211_i128),fld4: 10969_i16,fld5: (*_1).2,fld6: 4759860717176412898_i64 };
_28.1 = [_18,_18];
(*_1).2 = !Field::<u128>(Variant(_36.0, 2), 5);
place!(Field::<u32>(Variant(_36.0, 2), 1)) = _13;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(12_usize, 18_usize, Move(_18), 21_usize, Move(_21), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(12_usize, 6_usize, Move(_6), 33_usize, Move(_33), 23_usize, Move(_23), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(12_usize, 25_usize, Move(_25), 4_usize, Move(_4), 5_usize, Move(_5), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *const (&'static f32, [char; 2], u128, [u16; 4]),mut _2: isize,mut _3: isize,mut _4: [u16; 4],mut _5: (isize,),mut _6: (isize,),mut _7: (isize,),mut _8: isize) -> *const (&'static f32, [char; 2], u128, [u16; 4]) {
mir! {
type RET = *const (&'static f32, [char; 2], u128, [u16; 4]);
let _9: u128;
let _10: Adt79;
let _11: isize;
let _12: u64;
let _13: u8;
let _14: (&'static f32, [char; 2], u128, [u16; 4]);
let _15: (i32, i32);
let _16: (Adt18, Adt18);
let _17: u32;
let _18: char;
let _19: bool;
let _20: ((i32, i32), *const Adt18);
let _21: isize;
let _22: (isize,);
let _23: [i128; 4];
let _24: [u128; 5];
let _25: Adt75;
let _26: ();
let _27: ();
{
_4 = [37088_u16,26633_u16,46411_u16,17770_u16];
_6.0 = !_8;
RET = Move(_1);
_4 = [65253_u16,7577_u16,30876_u16,15571_u16];
_7 = (_2,);
_2 = _7.0 | _6.0;
_7 = (_5.0,);
_5 = (_7.0,);
_5.0 = 3515537392261811930_u64 as isize;
_6.0 = _7.0;
_5 = _6;
_6.0 = _5.0;
_7.0 = _6.0 ^ _2;
_11 = _7.0;
_12 = !9362818134173429155_u64;
_7.0 = _8;
_4 = [30158_u16,49980_u16,30622_u16,19130_u16];
_12 = 14407566061450871739_u64;
_2 = -_11;
_5 = (_6.0,);
_7.0 = _2 * _8;
_1 = Move(RET);
Goto(bb1)
}
bb1 = {
_11 = -_2;
RET = Move(_1);
_7 = (_8,);
_9 = 31086495457557801819073544192514625561_u128 ^ 106381260372127109129266528754830051773_u128;
_11 = !_8;
_7.0 = 63616_u16 as isize;
_14.3 = [38918_u16,56934_u16,3622_u16,46493_u16];
_14.1 = ['\u{5a25b}','\u{6ada8}'];
_7 = (_5.0,);
_3 = _2 ^ _11;
_7.0 = !_8;
_1 = Move(RET);
_14.2 = _9 - _9;
_14.3 = [43623_u16,59381_u16,27614_u16,20492_u16];
_15.1 = 143704185_i32 - (-1182673075_i32);
_7 = (_3,);
_15 = (1169279243_i32, (-1633539897_i32));
_14.1 = ['\u{f3bbe}','\u{b1d08}'];
RET = core::ptr::addr_of!(_14);
_16.0 = Adt18::Variant0 { fld0: (-8856028269647399615_i64),fld1: 64870_u16,fld2: (-114_i8) };
_7.0 = _3;
(*RET).2 = 99003158949236216284061605807868124248_i128 as u128;
_16.1 = Adt18::Variant0 { fld0: (-1221211810142014341_i64),fld1: 52031_u16,fld2: 56_i8 };
(*RET).1 = ['\u{4c91d}','\u{155ee}'];
(*RET).1 = ['\u{648e0}','\u{cb960}'];
_13 = 95_u8 >> _2;
(*RET).1 = ['\u{b6752}','\u{33bc8}'];
match _15.0 {
1169279243 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_16.0 = Adt18::Variant2 { fld0: false,fld1: 651375348_u32,fld2: _13,fld3: 138926986603743350109725889343736299870_i128,fld4: 22284_i16,fld5: _9,fld6: (-2917999752542028014_i64) };
(*RET).1 = ['\u{29df7}','\u{ee3bb}'];
_16.1 = Adt18::Variant0 { fld0: 6684098674299708922_i64,fld1: 35628_u16,fld2: 6_i8 };
_11 = _3;
_5.0 = _8 | _2;
(*RET).1 = ['\u{6a8d0}','\u{4ab4b}'];
_1 = core::ptr::addr_of!((*RET));
_15.0 = _15.1;
place!(Field::<u16>(Variant(_16.1, 0), 1)) = 13582_u16 | 64347_u16;
_1 = core::ptr::addr_of!((*_1));
place!(Field::<i8>(Variant(_16.1, 0), 2)) = (-106_i8) ^ 119_i8;
(*RET).3 = _4;
place!(Field::<u16>(Variant(_16.1, 0), 1)) = _5.0 as u16;
_18 = '\u{63307}';
(*RET).3 = [Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1)];
(*RET).3 = _4;
_14.1 = [_18,_18];
(*RET).3 = [Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1)];
(*RET).1 = [_18,_18];
(*_1).1 = [_18,_18];
(*RET).2 = _8 as u128;
_20.1 = core::ptr::addr_of!(_16.1);
_18 = '\u{4c0}';
_16.0 = Adt18::Variant3 { fld0: 38768748923928947730147909479172086761_i128,fld1: 8965171458522462350_i64,fld2: 1688654840_u32,fld3: _12,fld4: (-19272_i16) };
match _15.1 {
340282366920938463463374607430134671559 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
(*RET).3 = [Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1),Field::<u16>(Variant(_16.1, 0), 1)];
(*RET).2 = (-16251_i16) as u128;
Goto(bb6)
}
bb6 = {
_16.0 = Adt18::Variant0 { fld0: 1494059328780194112_i64,fld1: Field::<u16>(Variant(_16.1, 0), 1),fld2: Field::<i8>(Variant(_16.1, 0), 2) };
_19 = true;
Call((*RET).2 = fn14(Move(RET), Move(_1), (*RET).3, _6, Field::<u16>(Variant(_16.1, 0), 1), _14.3, _8, _7, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = Field::<i8>(Variant(_16.0, 0), 2) as u128;
_18 = '\u{97f1f}';
place!(Field::<i8>(Variant(_16.0, 0), 2)) = Field::<i8>(Variant(_16.1, 0), 2);
Call(_9 = fn15(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20.0.1 = !_15.0;
place!(Field::<i64>(Variant(_16.0, 0), 0)) = !282858586889943594_i64;
_5.0 = _15.1 as isize;
_21 = -_7.0;
place!(Field::<i64>(Variant(_16.1, 0), 0)) = Field::<i64>(Variant(_16.0, 0), 0) - Field::<i64>(Variant(_16.0, 0), 0);
_17 = Field::<i64>(Variant(_16.1, 0), 0) as u32;
RET = core::ptr::addr_of!(_14);
_22.0 = _11 << _7.0;
_16.0 = Adt18::Variant3 { fld0: (-79993632659249717582675962929235039874_i128),fld1: Field::<i64>(Variant(_16.1, 0), 0),fld2: _17,fld3: _12,fld4: 15814_i16 };
(*RET).2 = _9 | _9;
place!(Field::<u32>(Variant(_16.0, 3), 2)) = !_17;
(*RET).2 = _9 >> _7.0;
place!(Field::<i16>(Variant(_16.0, 3), 4)) = 26700_i16;
_22.0 = _18 as isize;
Goto(bb9)
}
bb9 = {
Call(_26 = dump_var(13_usize, 7_usize, Move(_7), 3_usize, Move(_3), 4_usize, Move(_4), 17_usize, Move(_17)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_26 = dump_var(13_usize, 21_usize, Move(_21), 8_usize, Move(_8), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *const (&'static f32, [char; 2], u128, [u16; 4]),mut _2: *const (&'static f32, [char; 2], u128, [u16; 4]),mut _3: [u16; 4],mut _4: (isize,),mut _5: u16,mut _6: [u16; 4],mut _7: isize,mut _8: (isize,),mut _9: (isize,)) -> u128 {
mir! {
type RET = u128;
let _10: f32;
let _11: *const bool;
let _12: [u32; 3];
let _13: (i32, [u32; 3], *const &'static u32);
let _14: *const bool;
let _15: u16;
let _16: Adt40;
let _17: ([isize; 8], *const *mut i64, *const *mut *const u16, *mut i64);
let _18: char;
let _19: (isize, &'static u128, usize, [i8; 8]);
let _20: (&'static f32, [char; 2], u128, [u16; 4]);
let _21: u128;
let _22: *const f32;
let _23: [i16; 7];
let _24: Adt24;
let _25: [bool; 5];
let _26: ();
let _27: ();
{
RET = 134192231881905046270165707999531392277_u128 * 123736872734031699793938870065322895831_u128;
_8.0 = (-148236351819012633455545660171214333422_i128) as isize;
_4.0 = _7;
_9 = _4;
_4.0 = _9.0 | _9.0;
_8 = (_9.0,);
_5 = 40765_u16 | 35580_u16;
_7 = -_8.0;
RET = 172923392869728503050306413083009116334_u128 - 302498349319946155149167271137302167731_u128;
_4.0 = _7 << _9.0;
_1 = Move(_2);
_2 = Move(_1);
RET = 54848815440263571129366447374772132785_u128 >> _9.0;
RET = false as u128;
_3 = [_5,_5,_5,_5];
_4.0 = _9.0;
RET = 334563610730681124459294205795753222052_u128 * 25718629287032016998187853469432713165_u128;
Goto(bb1)
}
bb1 = {
_3 = [_5,_5,_5,_5];
_4.0 = RET as isize;
_7 = _8.0;
_3 = [_5,_5,_5,_5];
_8 = (_9.0,);
_8.0 = _7;
_3 = [_5,_5,_5,_5];
_8 = (_9.0,);
RET = 245270302015887046397973353167382423030_u128 & 274417674870379051537878421369425899538_u128;
_8 = _9;
RET = 325146996805935767903218801689836293060_u128 * 117252392955217650089414909597092482283_u128;
_9.0 = -_8.0;
_5 = (-66_i8) as u16;
RET = 6_usize as u128;
_7 = _8.0;
_10 = _8.0 as f32;
_5 = 42365_u16;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
42365 => bb8,
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
_9 = (_7,);
_12 = [2277751005_u32,4241904371_u32,152799306_u32];
_13.1 = [481805123_u32,784733903_u32,1167516261_u32];
_9 = (_8.0,);
_12 = [1042047735_u32,3252789668_u32,856503777_u32];
_16.fld0 = core::ptr::addr_of!(_5);
_4 = (_9.0,);
_13.1 = _12;
_16.fld0 = core::ptr::addr_of!(_5);
_10 = 183_u8 as f32;
_9.0 = _8.0;
_16.fld4 = 92555722657181574637664411742584791639_i128 as f32;
_13.1 = [1448336760_u32,3228896277_u32,3169194689_u32];
_1 = Move(_2);
_9.0 = _7;
Goto(bb9)
}
bb9 = {
_13.1 = _12;
_3 = [_5,_5,_5,_5];
Goto(bb10)
}
bb10 = {
_16.fld3 = core::ptr::addr_of!(_16.fld4);
_8.0 = _4.0 & _7;
_8 = (_7,);
_8.0 = !_9.0;
_9 = (_4.0,);
Goto(bb11)
}
bb11 = {
_16.fld1 = [2643883604_u32,1768637878_u32,3299805614_u32];
_4.0 = _8.0 * _7;
_19.3 = [125_i8,(-69_i8),115_i8,(-86_i8),110_i8,125_i8,(-112_i8),36_i8];
_8 = (_9.0,);
_19.0 = !_4.0;
_16.fld0 = core::ptr::addr_of!(_5);
_18 = '\u{10f837}';
_19.1 = &RET;
_20.2 = !RET;
_4 = (_19.0,);
Goto(bb12)
}
bb12 = {
_20.0 = &_10;
_17.1 = core::ptr::addr_of!(_17.3);
Goto(bb13)
}
bb13 = {
_9.0 = _8.0 & _7;
RET = _20.2 & _20.2;
_8.0 = true as isize;
_4 = _9;
_9.0 = _7 + _19.0;
_3 = _6;
_15 = _5 << _9.0;
_13.0 = (-2064243864_i32) ^ 1831749714_i32;
_21 = 4897233699430438098_usize as u128;
_19.2 = 7999795633266257225_usize;
_19.3 = [(-8_i8),40_i8,(-96_i8),(-97_i8),59_i8,71_i8,74_i8,(-50_i8)];
_18 = '\u{3c537}';
_16.fld1 = [2356973341_u32,1382097787_u32,4212738506_u32];
match _5 {
0 => bb3,
42365 => bb14,
_ => bb12
}
}
bb14 = {
_20.3 = [_15,_15,_15,_15];
_20.1 = [_18,_18];
_1 = core::ptr::addr_of!(_20);
_10 = _16.fld4;
_6 = [_15,_15,_15,_15];
_19.0 = !_4.0;
_23 = [379_i16,15849_i16,27252_i16,(-933_i16),(-22789_i16),32009_i16,23916_i16];
_17.1 = core::ptr::addr_of!(_17.3);
_8 = (_9.0,);
(*_1).0 = &_10;
_16.fld4 = _10 + _10;
_19.1 = &RET;
_16.fld3 = core::ptr::addr_of!(_10);
_23 = [22616_i16,(-32704_i16),26373_i16,15018_i16,13398_i16,15631_i16,6132_i16];
_3 = [_15,_15,_15,_15];
_19.0 = _7 * _8.0;
_5 = 217_u8 as u16;
_24.fld1 = _18;
_20.1 = [_24.fld1,_24.fld1];
_24.fld2 = Adt18::Variant3 { fld0: 40125295543626023708068202283143713241_i128,fld1: (-8650790440571140712_i64),fld2: 4291427624_u32,fld3: 4570560946921107239_u64,fld4: (-30655_i16) };
_8.0 = _4.0;
_16.fld0 = core::ptr::addr_of!(_5);
(*_1).1 = [_24.fld1,_24.fld1];
_20.3 = [_15,_15,_15,_15];
(*_1).3 = _6;
_24.fld0 = (-13577861621565111616801105423985339633_i128) - 167234643338225506010527751061647033266_i128;
_4 = (_7,);
_16.fld2 = 78_i8 as isize;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(14_usize, 12_usize, Move(_12), 8_usize, Move(_8), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(14_usize, 15_usize, Move(_15), 23_usize, Move(_23), 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15() -> u128 {
mir! {
type RET = u128;
let _1: [isize; 8];
let _2: bool;
let _3: isize;
let _4: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _5: isize;
let _6: u16;
let _7: [u128; 5];
let _8: (f64, &'static f32, (Adt18, Adt18), *const &'static u32);
let _9: i16;
let _10: f64;
let _11: *const usize;
let _12: (f32, i128, *mut bool, *const i128);
let _13: [char; 2];
let _14: [u8; 2];
let _15: i32;
let _16: u8;
let _17: [u8; 2];
let _18: f32;
let _19: *const i8;
let _20: (isize,);
let _21: [i16; 7];
let _22: f64;
let _23: *const *mut *const u16;
let _24: char;
let _25: u64;
let _26: char;
let _27: *const usize;
let _28: Adt83;
let _29: ();
let _30: ();
{
RET = 24425353025285048638105259254877487014_u128;
RET = !10307573669962120014281447249078306924_u128;
RET = '\u{176f4}' as u128;
RET = !299950677971032731841223650752494724254_u128;
RET = 34738364162027327113153513241638175493_u128;
RET = false as u128;
RET = (-66038238747490239277831253272010129342_i128) as u128;
_1 = [7_isize,22_isize,(-9223372036854775808_isize),(-97_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET = '\u{5fd9d}' as u128;
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-47_isize),9223372036854775807_isize,(-43_isize),(-47_isize),(-9223372036854775808_isize),109_isize];
RET = 47814976605497100360361285225738119860_u128 ^ 90881980574559345032555186263989466610_u128;
_1 = [(-51_isize),103_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-86_isize),(-9223372036854775808_isize)];
_1 = [(-88_isize),72_isize,51_isize,104_isize,(-120_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET = 137568698249734056275728925354929434452_u128;
RET = 52_u8 as u128;
RET = !20575052335117608996537555485716103295_u128;
RET = !243731666496032808520188318892949388206_u128;
RET = (-13012_i16) as u128;
Goto(bb1)
}
bb1 = {
_1 = [57_isize,(-35_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,77_isize,(-9223372036854775808_isize)];
RET = 215417917644580737071791269472024772152_u128;
RET = (-1582375809_i32) as u128;
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),36_isize,(-9223372036854775808_isize),(-35_isize),(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,90_isize,6_isize,75_isize,9223372036854775807_isize];
RET = (-25_i8) as u128;
Goto(bb2)
}
bb2 = {
RET = 4993_u16 as u128;
_1 = [(-98_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = [(-125_isize),(-67_isize),9223372036854775807_isize,9223372036854775807_isize,(-28_isize),(-3_isize),(-18_isize),9223372036854775807_isize];
_1 = [(-9223372036854775808_isize),122_isize,9223372036854775807_isize,42_isize,(-98_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-25_isize)];
RET = !28946020457506635007897545560657112429_u128;
RET = 57469_u16 as u128;
RET = 110938415897186845845510103199317654569_u128 & 175687698760428201839535367581986456745_u128;
_2 = true;
Goto(bb3)
}
bb3 = {
_4.0.2.0 = Adt18::Variant1 { fld0: _2,fld1: '\u{ea62d}',fld2: 654255245_u32,fld3: 11485_u16,fld4: 156743738251999388665036131017484274997_i128,fld5: 5_usize,fld6: RET };
Goto(bb4)
}
bb4 = {
place!(Field::<u32>(Variant(_4.0.2.0, 1), 2)) = 1606384268_u32 << RET;
_4.0.2.0 = Adt18::Variant1 { fld0: _2,fld1: '\u{b6980}',fld2: 4049913336_u32,fld3: 22141_u16,fld4: 56428702725851556247918202589495463039_i128,fld5: 12980231832757091020_usize,fld6: RET };
place!(Field::<i128>(Variant(_4.0.2.0, 1), 4)) = 163345213886165742909000357668658809137_i128;
_4.0.0 = 10430447546876784737_u64 as f64;
place!(Field::<i128>(Variant(_4.0.2.0, 1), 4)) = '\u{3a2b}' as i128;
place!(Field::<u128>(Variant(_4.0.2.0, 1), 6)) = !RET;
_4.0.2.0 = Adt18::Variant1 { fld0: _2,fld1: '\u{19ded}',fld2: 2019974758_u32,fld3: 3409_u16,fld4: 154687738474656922898108965793623146037_i128,fld5: 14962449928620069610_usize,fld6: RET };
_4.0.0 = 6_usize as f64;
Call(RET = fn16(_1, Field::<bool>(Variant(_4.0.2.0, 1), 0), _1, Field::<bool>(Variant(_4.0.2.0, 1), 0), _2, Field::<bool>(Variant(_4.0.2.0, 1), 0), _1, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<bool>(Variant(_4.0.2.0, 1), 0)) = RET == RET;
_4.0.2.1 = Adt18::Variant3 { fld0: 8160243614007650818441739714666762889_i128,fld1: (-7352057865679929802_i64),fld2: 3578492776_u32,fld3: 6852672889006880713_u64,fld4: 16614_i16 };
place!(Field::<u128>(Variant(_4.0.2.0, 1), 6)) = RET + RET;
place!(Field::<i16>(Variant(_4.0.2.1, 3), 4)) = -9717_i16;
RET = 5141662854853478535_i64 as u128;
place!(Field::<i64>(Variant(_4.0.2.1, 3), 1)) = 3004689457457696493_i64;
place!(Field::<u64>(Variant(_4.0.2.1, 3), 3)) = Field::<u128>(Variant(_4.0.2.0, 1), 6) as u64;
place!(Field::<i16>(Variant(_4.0.2.1, 3), 4)) = 31989_i16;
_4.0.2.1 = Adt18::Variant0 { fld0: (-8703176416263443229_i64),fld1: 6882_u16,fld2: 107_i8 };
place!(Field::<usize>(Variant(_4.0.2.0, 1), 5)) = !7_usize;
_6 = !17966_u16;
_1 = [(-118_isize),52_isize,9223372036854775807_isize,23_isize,9223372036854775807_isize,9223372036854775807_isize,93_isize,9223372036854775807_isize];
_4.0.2.0 = Adt18::Variant3 { fld0: 147354028769056173233636539471655710229_i128,fld1: 7452051723118376537_i64,fld2: 2723419590_u32,fld3: 3198621098612543384_u64,fld4: (-3191_i16) };
place!(Field::<i64>(Variant(_4.0.2.1, 0), 0)) = -(-8722900441835320595_i64);
place!(Field::<u64>(Variant(_4.0.2.0, 3), 3)) = 36_u8 as u64;
RET = !337809816321231255859575479510672682593_u128;
_4.0.2.1 = Adt18::Variant1 { fld0: _2,fld1: '\u{8f287}',fld2: 3380304530_u32,fld3: _6,fld4: (-102380039440286822777714869695639604888_i128),fld5: 1145958409782382715_usize,fld6: RET };
_1 = [(-42_isize),21_isize,9223372036854775807_isize,9223372036854775807_isize,(-122_isize),9223372036854775807_isize,50_isize,(-45_isize)];
place!(Field::<i128>(Variant(_4.0.2.0, 3), 0)) = -126305118348540581080386976370763876846_i128;
_3 = Field::<i128>(Variant(_4.0.2.0, 3), 0) as isize;
place!(Field::<bool>(Variant(_4.0.2.1, 1), 0)) = _2;
place!(Field::<char>(Variant(_4.0.2.1, 1), 1)) = '\u{498a7}';
place!(Field::<u32>(Variant(_4.0.2.1, 1), 2)) = 3334953820_u32 & 3660633180_u32;
RET = Field::<u128>(Variant(_4.0.2.1, 1), 6) >> _6;
place!(Field::<i128>(Variant(_4.0.2.1, 1), 4)) = Field::<i128>(Variant(_4.0.2.0, 3), 0);
Call(_9 = core::intrinsics::bswap(19759_i16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = Field::<char>(Variant(_4.0.2.1, 1), 1) as isize;
_6 = Field::<u16>(Variant(_4.0.2.1, 1), 3) ^ Field::<u16>(Variant(_4.0.2.1, 1), 3);
_2 = Field::<bool>(Variant(_4.0.2.1, 1), 0);
place!(Field::<bool>(Variant(_4.0.2.1, 1), 0)) = _2;
place!(Field::<bool>(Variant(_4.0.2.1, 1), 0)) = RET <= RET;
_9 = _6 as i16;
place!(Field::<u64>(Variant(_4.0.2.0, 3), 3)) = 1198950154305967856_u64 | 4314210488129698325_u64;
_4.0.2.1 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_4.0.2.0, 3), 0),fld1: (-6946007862718774712_i64),fld2: 1158049331_u32,fld3: Field::<u64>(Variant(_4.0.2.0, 3), 3),fld4: _9 };
place!(Field::<i64>(Variant(_4.0.2.1, 3), 1)) = !4979474399690558502_i64;
_4.0.2.0 = Adt18::Variant2 { fld0: _2,fld1: 322265433_u32,fld2: 192_u8,fld3: Field::<i128>(Variant(_4.0.2.1, 3), 0),fld4: _9,fld5: RET,fld6: Field::<i64>(Variant(_4.0.2.1, 3), 1) };
place!(Field::<u128>(Variant(_4.0.2.0, 2), 5)) = _4.0.0 as u128;
_13 = ['\u{3aadc}','\u{bc1b}'];
_10 = Field::<i128>(Variant(_4.0.2.0, 2), 3) as f64;
RET = Field::<u128>(Variant(_4.0.2.0, 2), 5) + Field::<u128>(Variant(_4.0.2.0, 2), 5);
_8.1 = &_12.0;
_13 = ['\u{2537d}','\u{1e61a}'];
Goto(bb7)
}
bb7 = {
_12.3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_4.0.2.1, 3), 0)));
_8.2.1 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_4.0.2.0, 2), 6),fld1: _6,fld2: 76_i8 };
_4.0.1 = &_12.0;
RET = !Field::<u128>(Variant(_4.0.2.0, 2), 5);
place!(Field::<u32>(Variant(_4.0.2.0, 2), 1)) = 917324346_u32;
place!(Field::<u128>(Variant(_4.0.2.0, 2), 5)) = RET;
_8.2.0 = Adt18::Variant2 { fld0: _2,fld1: Field::<u32>(Variant(_4.0.2.0, 2), 1),fld2: 38_u8,fld3: Field::<i128>(Variant(_4.0.2.0, 2), 3),fld4: Field::<i16>(Variant(_4.0.2.1, 3), 4),fld5: RET,fld6: Field::<i64>(Variant(_4.0.2.0, 2), 6) };
_12.1 = '\u{cb4c4}' as i128;
Goto(bb8)
}
bb8 = {
_4.0.2.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_4.0.2.0, 2), 0),fld1: Field::<u32>(Variant(_8.2.0, 2), 1),fld2: 196_u8,fld3: Field::<i128>(Variant(_8.2.0, 2), 3),fld4: Field::<i16>(Variant(_8.2.0, 2), 4),fld5: RET,fld6: Field::<i64>(Variant(_8.2.1, 0), 0) };
RET = Field::<u128>(Variant(_4.0.2.1, 2), 5) * Field::<u128>(Variant(_8.2.0, 2), 5);
_15 = -(-743367031_i32);
_8.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_4.0.2.1, 2), 0),fld1: '\u{b7442}',fld2: Field::<u32>(Variant(_4.0.2.1, 2), 1),fld3: _6,fld4: Field::<i128>(Variant(_4.0.2.0, 2), 3),fld5: 4_usize,fld6: Field::<u128>(Variant(_4.0.2.1, 2), 5) };
_8.0 = _5 as f64;
_16 = _8.0 as u8;
_4.0.2.0 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_8.2.0, 2), 6),fld1: _6,fld2: 70_i8 };
place!(Field::<i64>(Variant(_4.0.2.0, 0), 0)) = -Field::<i64>(Variant(_8.2.0, 2), 6);
_11 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_8.2.1, 1), 5)));
_4.0.1 = &_12.0;
RET = !Field::<u128>(Variant(_8.2.1, 1), 6);
_14 = [_16,_16];
_12.0 = 13815576905615394733_u64 as f32;
place!(Field::<u8>(Variant(_8.2.0, 2), 2)) = Field::<u32>(Variant(_4.0.2.1, 2), 1) as u8;
place!(Field::<usize>(Variant(_8.2.1, 1), 5)) = 15703958684054295370_usize + 7_usize;
place!(Field::<u8>(Variant(_8.2.0, 2), 2)) = !_16;
_4.0.0 = _8.0 * _10;
place!(Field::<i8>(Variant(_4.0.2.0, 0), 2)) = (-124_i8) << Field::<u32>(Variant(_4.0.2.1, 2), 1);
SetDiscriminant(_4.0.2.0, 0);
_13 = ['\u{d0de}','\u{29cbb}'];
Goto(bb9)
}
bb9 = {
place!(Field::<char>(Variant(_8.2.1, 1), 1)) = '\u{4bd62}';
place!(Field::<i64>(Variant(_8.2.0, 2), 6)) = Field::<i16>(Variant(_4.0.2.1, 2), 4) as i64;
_9 = _15 as i16;
_2 = Field::<bool>(Variant(_8.2.0, 2), 0);
_4.0.1 = &_12.0;
_7 = [RET,Field::<u128>(Variant(_8.2.1, 1), 6),Field::<u128>(Variant(_8.2.0, 2), 5),Field::<u128>(Variant(_8.2.1, 1), 6),Field::<u128>(Variant(_4.0.2.1, 2), 5)];
place!(Field::<u32>(Variant(_4.0.2.1, 2), 1)) = (*_11) as u32;
_8.2.0 = _8.2.1;
_8.2.0 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_4.0.2.1, 2), 6),fld1: Field::<u16>(Variant(_8.2.1, 1), 3),fld2: 81_i8 };
place!(Field::<i8>(Variant(_4.0.2.0, 0), 2)) = Field::<i128>(Variant(_8.2.1, 1), 4) as i8;
_4.0.0 = _10 * _10;
place!(Field::<bool>(Variant(_8.2.1, 1), 0)) = Field::<usize>(Variant(_8.2.1, 1), 5) > (*_11);
Goto(bb10)
}
bb10 = {
_9 = Field::<i16>(Variant(_4.0.2.1, 2), 4);
(*_11) = !0_usize;
_3 = _5;
place!(Field::<bool>(Variant(_4.0.2.1, 2), 0)) = Field::<bool>(Variant(_8.2.1, 1), 0) ^ Field::<bool>(Variant(_8.2.1, 1), 0);
_21 = [_9,_9,Field::<i16>(Variant(_4.0.2.1, 2), 4),_9,_9,_9,_9];
place!(Field::<i64>(Variant(_8.2.0, 0), 0)) = Field::<i64>(Variant(_4.0.2.1, 2), 6);
match Field::<u32>(Variant(_8.2.1, 1), 2) {
917324346 => bb12,
_ => bb11
}
}
bb11 = {
_12.3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_4.0.2.1, 3), 0)));
_8.2.1 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_4.0.2.0, 2), 6),fld1: _6,fld2: 76_i8 };
_4.0.1 = &_12.0;
RET = !Field::<u128>(Variant(_4.0.2.0, 2), 5);
place!(Field::<u32>(Variant(_4.0.2.0, 2), 1)) = 917324346_u32;
place!(Field::<u128>(Variant(_4.0.2.0, 2), 5)) = RET;
_8.2.0 = Adt18::Variant2 { fld0: _2,fld1: Field::<u32>(Variant(_4.0.2.0, 2), 1),fld2: 38_u8,fld3: Field::<i128>(Variant(_4.0.2.0, 2), 3),fld4: Field::<i16>(Variant(_4.0.2.1, 3), 4),fld5: RET,fld6: Field::<i64>(Variant(_4.0.2.0, 2), 6) };
_12.1 = '\u{cb4c4}' as i128;
Goto(bb8)
}
bb12 = {
_7 = [Field::<u128>(Variant(_4.0.2.1, 2), 5),RET,RET,RET,RET];
_13 = [Field::<char>(Variant(_8.2.1, 1), 1),Field::<char>(Variant(_8.2.1, 1), 1)];
_19 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_8.2.0, 0), 2)));
_4.0.0 = _8.0 - _8.0;
place!(Field::<u8>(Variant(_4.0.2.1, 2), 2)) = !_16;
_11 = core::ptr::addr_of!((*_11));
(*_11) = 9274750072027605129_usize + 4_usize;
place!(Field::<i64>(Variant(_4.0.2.0, 0), 0)) = Field::<i64>(Variant(_4.0.2.1, 2), 6);
_17 = _14;
_11 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_8.2.1, 1), 5)));
SetDiscriminant(_4.0.2.1, 0);
Call(_4.0.2 = fn17(Move(_4.0.1), Move(_11), Field::<u128>(Variant(_8.2.1, 1), 6), Move(_19), _8.2.1, _2, _8.2.1, _1, _1, _9), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3 = _5;
SetDiscriminant(_8.2.1, 1);
_8.1 = &_12.0;
_4.0.2.0 = Adt18::Variant1 { fld0: _2,fld1: Field::<char>(Variant(_4.0.2.1, 1), 1),fld2: Field::<u32>(Variant(_4.0.2.1, 1), 2),fld3: Field::<u16>(Variant(_4.0.2.1, 1), 3),fld4: _12.1,fld5: Field::<usize>(Variant(_4.0.2.1, 1), 5),fld6: Field::<u128>(Variant(_4.0.2.1, 1), 6) };
_2 = _5 < _5;
_6 = Field::<u128>(Variant(_4.0.2.0, 1), 6) as u16;
place!(Field::<u16>(Variant(_4.0.2.0, 1), 3)) = Field::<usize>(Variant(_4.0.2.1, 1), 5) as u16;
_11 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.0.2.0, 1), 5)));
_6 = !Field::<u16>(Variant(_4.0.2.0, 1), 3);
_22 = _8.0 - _10;
place!(Field::<u128>(Variant(_8.2.1, 1), 6)) = !Field::<u128>(Variant(_4.0.2.0, 1), 6);
_2 = !Field::<bool>(Variant(_4.0.2.1, 1), 0);
_12.1 = _4.0.0 as i128;
match Field::<u32>(Variant(_4.0.2.1, 1), 2) {
0 => bb1,
1 => bb12,
2 => bb14,
917324346 => bb16,
_ => bb15
}
}
bb14 = {
_9 = Field::<i16>(Variant(_4.0.2.1, 2), 4);
(*_11) = !0_usize;
_3 = _5;
place!(Field::<bool>(Variant(_4.0.2.1, 2), 0)) = Field::<bool>(Variant(_8.2.1, 1), 0) ^ Field::<bool>(Variant(_8.2.1, 1), 0);
_21 = [_9,_9,Field::<i16>(Variant(_4.0.2.1, 2), 4),_9,_9,_9,_9];
place!(Field::<i64>(Variant(_8.2.0, 0), 0)) = Field::<i64>(Variant(_4.0.2.1, 2), 6);
match Field::<u32>(Variant(_8.2.1, 1), 2) {
917324346 => bb12,
_ => bb11
}
}
bb15 = {
place!(Field::<bool>(Variant(_4.0.2.0, 1), 0)) = RET == RET;
_4.0.2.1 = Adt18::Variant3 { fld0: 8160243614007650818441739714666762889_i128,fld1: (-7352057865679929802_i64),fld2: 3578492776_u32,fld3: 6852672889006880713_u64,fld4: 16614_i16 };
place!(Field::<u128>(Variant(_4.0.2.0, 1), 6)) = RET + RET;
place!(Field::<i16>(Variant(_4.0.2.1, 3), 4)) = -9717_i16;
RET = 5141662854853478535_i64 as u128;
place!(Field::<i64>(Variant(_4.0.2.1, 3), 1)) = 3004689457457696493_i64;
place!(Field::<u64>(Variant(_4.0.2.1, 3), 3)) = Field::<u128>(Variant(_4.0.2.0, 1), 6) as u64;
place!(Field::<i16>(Variant(_4.0.2.1, 3), 4)) = 31989_i16;
_4.0.2.1 = Adt18::Variant0 { fld0: (-8703176416263443229_i64),fld1: 6882_u16,fld2: 107_i8 };
place!(Field::<usize>(Variant(_4.0.2.0, 1), 5)) = !7_usize;
_6 = !17966_u16;
_1 = [(-118_isize),52_isize,9223372036854775807_isize,23_isize,9223372036854775807_isize,9223372036854775807_isize,93_isize,9223372036854775807_isize];
_4.0.2.0 = Adt18::Variant3 { fld0: 147354028769056173233636539471655710229_i128,fld1: 7452051723118376537_i64,fld2: 2723419590_u32,fld3: 3198621098612543384_u64,fld4: (-3191_i16) };
place!(Field::<i64>(Variant(_4.0.2.1, 0), 0)) = -(-8722900441835320595_i64);
place!(Field::<u64>(Variant(_4.0.2.0, 3), 3)) = 36_u8 as u64;
RET = !337809816321231255859575479510672682593_u128;
_4.0.2.1 = Adt18::Variant1 { fld0: _2,fld1: '\u{8f287}',fld2: 3380304530_u32,fld3: _6,fld4: (-102380039440286822777714869695639604888_i128),fld5: 1145958409782382715_usize,fld6: RET };
_1 = [(-42_isize),21_isize,9223372036854775807_isize,9223372036854775807_isize,(-122_isize),9223372036854775807_isize,50_isize,(-45_isize)];
place!(Field::<i128>(Variant(_4.0.2.0, 3), 0)) = -126305118348540581080386976370763876846_i128;
_3 = Field::<i128>(Variant(_4.0.2.0, 3), 0) as isize;
place!(Field::<bool>(Variant(_4.0.2.1, 1), 0)) = _2;
place!(Field::<char>(Variant(_4.0.2.1, 1), 1)) = '\u{498a7}';
place!(Field::<u32>(Variant(_4.0.2.1, 1), 2)) = 3334953820_u32 & 3660633180_u32;
RET = Field::<u128>(Variant(_4.0.2.1, 1), 6) >> _6;
place!(Field::<i128>(Variant(_4.0.2.1, 1), 4)) = Field::<i128>(Variant(_4.0.2.0, 3), 0);
Call(_9 = core::intrinsics::bswap(19759_i16), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_24 = Field::<char>(Variant(_4.0.2.0, 1), 1);
SetDiscriminant(_4.0.2.0, 3);
_3 = !_5;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(15_usize, 16_usize, Move(_16), 2_usize, Move(_2), 3_usize, Move(_3), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(15_usize, 5_usize, Move(_5), 9_usize, Move(_9), 7_usize, Move(_7), 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [isize; 8],mut _2: bool,mut _3: [isize; 8],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: [isize; 8],mut _8: [isize; 8]) -> u128 {
mir! {
type RET = u128;
let _9: Adt22;
let _10: (&'static f32, [char; 2], u128, [u16; 4]);
let _11: f32;
let _12: *mut i64;
let _13: char;
let _14: [char; 2];
let _15: f64;
let _16: *mut [char; 2];
let _17: *mut [isize; 8];
let _18: &'static *mut [isize; 8];
let _19: u128;
let _20: (Adt18, Adt18);
let _21: i16;
let _22: *const &'static u32;
let _23: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _24: [i16; 7];
let _25: bool;
let _26: *const &'static i64;
let _27: u32;
let _28: isize;
let _29: i32;
let _30: *mut [char; 2];
let _31: &'static u32;
let _32: isize;
let _33: Adt71;
let _34: [u128; 5];
let _35: ((f64, &'static f32, (Adt18, Adt18), *const &'static u32),);
let _36: u8;
let _37: [u32; 3];
let _38: &'static *mut [isize; 8];
let _39: [char; 2];
let _40: i32;
let _41: isize;
let _42: usize;
let _43: Adt24;
let _44: [usize; 3];
let _45: i128;
let _46: &'static *const u16;
let _47: Adt71;
let _48: ();
let _49: ();
{
_4 = _2;
_4 = _2;
_8 = [(-9223372036854775808_isize),(-65_isize),(-43_isize),(-9223372036854775808_isize),103_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-86_isize)];
RET = !238353358921025433547540269527334138017_u128;
_10.3 = [26276_u16,25398_u16,18642_u16,34451_u16];
_10.1 = ['\u{c9bc5}','\u{17d21}'];
RET = !13641172306230287841879854816842937504_u128;
_10.2 = !RET;
_1 = _3;
_1 = [(-124_isize),40_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-93_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET = _10.2 * _10.2;
Goto(bb1)
}
bb1 = {
_8 = _3;
_15 = 4_usize as f64;
_13 = '\u{1389e}';
RET = _10.2 | _10.2;
_10.0 = &_11;
Goto(bb2)
}
bb2 = {
_3 = _7;
_6 = _5 & _5;
_2 = _6 ^ _5;
RET = _2 as u128;
_3 = _8;
_10.3 = [58166_u16,50504_u16,36559_u16,50433_u16];
_6 = _2 ^ _2;
_15 = 76_u8 as f64;
_2 = _6;
_6 = !_2;
_14 = [_13,_13];
_15 = 2546196094_u32 as f64;
RET = !_10.2;
_20.0 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 3171608463_u32,fld3: 38329_u16,fld4: (-113199435642788729792044735316111657221_i128),fld5: 1094817686430837390_usize,fld6: _10.2 };
_11 = 5292532478643473489_usize as f32;
_19 = !_10.2;
_1 = [(-44_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),(-9223372036854775808_isize)];
Call(place!(Field::<u32>(Variant(_20.0, 1), 2)) = core::intrinsics::transmute(Field::<char>(Variant(_20.0, 1), 1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _2 ^ Field::<bool>(Variant(_20.0, 1), 0);
place!(Field::<char>(Variant(_20.0, 1), 1)) = _13;
place!(Field::<bool>(Variant(_20.0, 1), 0)) = _6 >= _6;
_17 = core::ptr::addr_of_mut!(_7);
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),87_isize,104_isize,(-39_isize),9223372036854775807_isize,9223372036854775807_isize,54_isize];
place!(Field::<i128>(Variant(_20.0, 1), 4)) = !31927027054662799384905143063403782453_i128;
_13 = Field::<char>(Variant(_20.0, 1), 1);
_10.0 = &_11;
_1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-82_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20.0 = Adt18::Variant1 { fld0: _6,fld1: _13,fld2: 74236859_u32,fld3: 41778_u16,fld4: 88260171104883403498142176733029064318_i128,fld5: 2_usize,fld6: _19 };
_20.0 = Adt18::Variant2 { fld0: _2,fld1: 3719323112_u32,fld2: 80_u8,fld3: 164134513328381288705983453630725335547_i128,fld4: 9142_i16,fld5: _19,fld6: 6930156061856264288_i64 };
_16 = core::ptr::addr_of_mut!(_10.1);
_23.0.2.1 = Adt18::Variant0 { fld0: 593391576749531405_i64,fld1: 63917_u16,fld2: 41_i8 };
_17 = core::ptr::addr_of_mut!(_8);
Goto(bb4)
}
bb4 = {
place!(Field::<i64>(Variant(_23.0.2.1, 0), 0)) = (-7466803111037112392_i64) * 160983610316307753_i64;
(*_17) = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,114_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-70_isize)];
_20.1 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 1723731752_u32,fld3: 4363_u16,fld4: (-2545747357044535394382293543583442824_i128),fld5: 3999295640806384871_usize,fld6: RET };
_1 = [(-53_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-48_isize),9223372036854775807_isize,(-47_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<i64>(Variant(_20.0, 2), 6)) = !Field::<i64>(Variant(_23.0.2.1, 0), 0);
_10.2 = !Field::<u128>(Variant(_20.1, 1), 6);
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 10595715277000559776_usize + 12032217758260171505_usize;
_6 = !Field::<bool>(Variant(_20.0, 2), 0);
_10.1 = _14;
_23.0.2.1 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_20.0, 2), 6),fld1: 23050_u16,fld2: 115_i8 };
place!(Field::<u8>(Variant(_20.0, 2), 2)) = (-92_isize) as u8;
place!(Field::<u128>(Variant(_20.1, 1), 6)) = !_10.2;
place!(Field::<i128>(Variant(_20.1, 1), 4)) = !71022137859124588721809640572684087932_i128;
_21 = 14543299821687161115_u64 as i16;
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 2_usize * 136793490246000349_usize;
(*_16) = _14;
place!(Field::<u32>(Variant(_20.0, 2), 1)) = 3598532676_u32;
Goto(bb5)
}
bb5 = {
_15 = 1646747025_i32 as f64;
_16 = core::ptr::addr_of_mut!((*_16));
place!(Field::<i128>(Variant(_20.0, 2), 3)) = _11 as i128;
RET = !Field::<u128>(Variant(_20.0, 2), 5);
_14 = [_13,Field::<char>(Variant(_20.1, 1), 1)];
_5 = !Field::<bool>(Variant(_20.0, 2), 0);
_17 = core::ptr::addr_of_mut!(_7);
match Field::<u32>(Variant(_20.0, 2), 1) {
0 => bb6,
1 => bb7,
2 => bb8,
3598532676 => bb10,
_ => bb9
}
}
bb6 = {
place!(Field::<i64>(Variant(_23.0.2.1, 0), 0)) = (-7466803111037112392_i64) * 160983610316307753_i64;
(*_17) = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,114_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-70_isize)];
_20.1 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 1723731752_u32,fld3: 4363_u16,fld4: (-2545747357044535394382293543583442824_i128),fld5: 3999295640806384871_usize,fld6: RET };
_1 = [(-53_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-48_isize),9223372036854775807_isize,(-47_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<i64>(Variant(_20.0, 2), 6)) = !Field::<i64>(Variant(_23.0.2.1, 0), 0);
_10.2 = !Field::<u128>(Variant(_20.1, 1), 6);
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 10595715277000559776_usize + 12032217758260171505_usize;
_6 = !Field::<bool>(Variant(_20.0, 2), 0);
_10.1 = _14;
_23.0.2.1 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_20.0, 2), 6),fld1: 23050_u16,fld2: 115_i8 };
place!(Field::<u8>(Variant(_20.0, 2), 2)) = (-92_isize) as u8;
place!(Field::<u128>(Variant(_20.1, 1), 6)) = !_10.2;
place!(Field::<i128>(Variant(_20.1, 1), 4)) = !71022137859124588721809640572684087932_i128;
_21 = 14543299821687161115_u64 as i16;
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 2_usize * 136793490246000349_usize;
(*_16) = _14;
place!(Field::<u32>(Variant(_20.0, 2), 1)) = 3598532676_u32;
Goto(bb5)
}
bb7 = {
_6 = _2 ^ Field::<bool>(Variant(_20.0, 1), 0);
place!(Field::<char>(Variant(_20.0, 1), 1)) = _13;
place!(Field::<bool>(Variant(_20.0, 1), 0)) = _6 >= _6;
_17 = core::ptr::addr_of_mut!(_7);
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),87_isize,104_isize,(-39_isize),9223372036854775807_isize,9223372036854775807_isize,54_isize];
place!(Field::<i128>(Variant(_20.0, 1), 4)) = !31927027054662799384905143063403782453_i128;
_13 = Field::<char>(Variant(_20.0, 1), 1);
_10.0 = &_11;
_1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-82_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20.0 = Adt18::Variant1 { fld0: _6,fld1: _13,fld2: 74236859_u32,fld3: 41778_u16,fld4: 88260171104883403498142176733029064318_i128,fld5: 2_usize,fld6: _19 };
_20.0 = Adt18::Variant2 { fld0: _2,fld1: 3719323112_u32,fld2: 80_u8,fld3: 164134513328381288705983453630725335547_i128,fld4: 9142_i16,fld5: _19,fld6: 6930156061856264288_i64 };
_16 = core::ptr::addr_of_mut!(_10.1);
_23.0.2.1 = Adt18::Variant0 { fld0: 593391576749531405_i64,fld1: 63917_u16,fld2: 41_i8 };
_17 = core::ptr::addr_of_mut!(_8);
Goto(bb4)
}
bb8 = {
_3 = _7;
_6 = _5 & _5;
_2 = _6 ^ _5;
RET = _2 as u128;
_3 = _8;
_10.3 = [58166_u16,50504_u16,36559_u16,50433_u16];
_6 = _2 ^ _2;
_15 = 76_u8 as f64;
_2 = _6;
_6 = !_2;
_14 = [_13,_13];
_15 = 2546196094_u32 as f64;
RET = !_10.2;
_20.0 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 3171608463_u32,fld3: 38329_u16,fld4: (-113199435642788729792044735316111657221_i128),fld5: 1094817686430837390_usize,fld6: _10.2 };
_11 = 5292532478643473489_usize as f32;
_19 = !_10.2;
_1 = [(-44_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),(-9223372036854775808_isize)];
Call(place!(Field::<u32>(Variant(_20.0, 1), 2)) = core::intrinsics::transmute(Field::<char>(Variant(_20.0, 1), 1)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_8 = _3;
_15 = 4_usize as f64;
_13 = '\u{1389e}';
RET = _10.2 | _10.2;
_10.0 = &_11;
Goto(bb2)
}
bb10 = {
_2 = !Field::<bool>(Variant(_20.1, 1), 0);
RET = _19 - Field::<u128>(Variant(_20.1, 1), 6);
_23.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_20.1, 1), 0),fld1: _13,fld2: Field::<u32>(Variant(_20.0, 2), 1),fld3: 62615_u16,fld4: Field::<i128>(Variant(_20.1, 1), 4),fld5: Field::<usize>(Variant(_20.1, 1), 5),fld6: _19 };
_28 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_23.0.2.0 = Adt18::Variant2 { fld0: _6,fld1: Field::<u32>(Variant(_23.0.2.1, 1), 2),fld2: Field::<u8>(Variant(_20.0, 2), 2),fld3: Field::<i128>(Variant(_20.0, 2), 3),fld4: _21,fld5: Field::<u128>(Variant(_23.0.2.1, 1), 6),fld6: Field::<i64>(Variant(_20.0, 2), 6) };
place!(Field::<u32>(Variant(_20.1, 1), 2)) = Field::<u32>(Variant(_23.0.2.1, 1), 2);
_27 = Field::<u32>(Variant(_20.0, 2), 1);
_35.0.2.1 = Adt18::Variant2 { fld0: _6,fld1: Field::<u32>(Variant(_20.1, 1), 2),fld2: Field::<u8>(Variant(_20.0, 2), 2),fld3: Field::<i128>(Variant(_23.0.2.1, 1), 4),fld4: Field::<i16>(Variant(_23.0.2.0, 2), 4),fld5: Field::<u128>(Variant(_23.0.2.1, 1), 6),fld6: Field::<i64>(Variant(_23.0.2.0, 2), 6) };
place!(Field::<u32>(Variant(_35.0.2.1, 2), 1)) = Field::<u32>(Variant(_20.0, 2), 1) << Field::<u32>(Variant(_23.0.2.1, 1), 2);
place!(Field::<bool>(Variant(_20.1, 1), 0)) = !_4;
_35.0.2.0 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_23.0.2.1, 1), 4),fld1: Field::<i64>(Variant(_23.0.2.0, 2), 6),fld2: Field::<u32>(Variant(_23.0.2.0, 2), 1),fld3: 11996540589789905279_u64,fld4: Field::<i16>(Variant(_23.0.2.0, 2), 4) };
_23.0.0 = _15 + _15;
_13 = Field::<char>(Variant(_20.1, 1), 1);
place!(Field::<i128>(Variant(_23.0.2.1, 1), 4)) = Field::<i128>(Variant(_35.0.2.1, 2), 3) >> RET;
place!(Field::<u128>(Variant(_35.0.2.1, 2), 5)) = Field::<u128>(Variant(_23.0.2.0, 2), 5);
_12 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_35.0.2.1, 2), 6)));
_20 = (_23.0.2.0, _35.0.2.1);
place!(Field::<u8>(Variant(_20.0, 2), 2)) = !Field::<u8>(Variant(_23.0.2.0, 2), 2);
SetDiscriminant(_20.1, 2);
_8 = [_28,_28,_28,_28,_28,_28,_28,_28];
_13 = Field::<char>(Variant(_23.0.2.1, 1), 1);
Call(_35.0.0 = core::intrinsics::fmaf64(_23.0.0, _15, _15), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_27 = _28 as u32;
place!(Field::<u128>(Variant(_23.0.2.0, 2), 5)) = 7929726310654488601_u64 as u128;
SetDiscriminant(_23.0.2.0, 3);
place!(Field::<bool>(Variant(_20.1, 2), 0)) = _6;
_20.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_20.0, 2), 0),fld1: _13,fld2: Field::<u32>(Variant(_35.0.2.0, 3), 2),fld3: 47311_u16,fld4: Field::<i128>(Variant(_23.0.2.1, 1), 4),fld5: Field::<usize>(Variant(_23.0.2.1, 1), 5),fld6: RET };
_2 = !Field::<bool>(Variant(_23.0.2.1, 1), 0);
_20 = (_35.0.2.1, _35.0.2.1);
_35.0.1 = &_11;
_23.0.1 = Move(_10.0);
place!(Field::<u16>(Variant(_23.0.2.1, 1), 3)) = !29142_u16;
_22 = core::ptr::addr_of!(_31);
place!(Field::<u32>(Variant(_23.0.2.0, 3), 2)) = Field::<u32>(Variant(_35.0.2.1, 2), 1);
place!(Field::<i128>(Variant(_20.1, 2), 3)) = !Field::<i128>(Variant(_35.0.2.0, 3), 0);
_35.0.2.1 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: Field::<u32>(Variant(_20.0, 2), 1),fld3: Field::<u16>(Variant(_23.0.2.1, 1), 3),fld4: Field::<i128>(Variant(_20.0, 2), 3),fld5: Field::<usize>(Variant(_23.0.2.1, 1), 5),fld6: Field::<u128>(Variant(_23.0.2.1, 1), 6) };
place!(Field::<i128>(Variant(_35.0.2.1, 1), 4)) = !Field::<i128>(Variant(_35.0.2.0, 3), 0);
_29 = -(-701377581_i32);
place!(Field::<i128>(Variant(_20.1, 2), 3)) = Field::<i128>(Variant(_35.0.2.1, 1), 4);
Goto(bb12)
}
bb12 = {
_38 = &_17;
match Field::<u32>(Variant(_35.0.2.0, 3), 2) {
0 => bb7,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
3598532676 => bb19,
_ => bb18
}
}
bb13 = {
_8 = _3;
_15 = 4_usize as f64;
_13 = '\u{1389e}';
RET = _10.2 | _10.2;
_10.0 = &_11;
Goto(bb2)
}
bb14 = {
_2 = !Field::<bool>(Variant(_20.1, 1), 0);
RET = _19 - Field::<u128>(Variant(_20.1, 1), 6);
_23.0.2.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_20.1, 1), 0),fld1: _13,fld2: Field::<u32>(Variant(_20.0, 2), 1),fld3: 62615_u16,fld4: Field::<i128>(Variant(_20.1, 1), 4),fld5: Field::<usize>(Variant(_20.1, 1), 5),fld6: _19 };
_28 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_23.0.2.0 = Adt18::Variant2 { fld0: _6,fld1: Field::<u32>(Variant(_23.0.2.1, 1), 2),fld2: Field::<u8>(Variant(_20.0, 2), 2),fld3: Field::<i128>(Variant(_20.0, 2), 3),fld4: _21,fld5: Field::<u128>(Variant(_23.0.2.1, 1), 6),fld6: Field::<i64>(Variant(_20.0, 2), 6) };
place!(Field::<u32>(Variant(_20.1, 1), 2)) = Field::<u32>(Variant(_23.0.2.1, 1), 2);
_27 = Field::<u32>(Variant(_20.0, 2), 1);
_35.0.2.1 = Adt18::Variant2 { fld0: _6,fld1: Field::<u32>(Variant(_20.1, 1), 2),fld2: Field::<u8>(Variant(_20.0, 2), 2),fld3: Field::<i128>(Variant(_23.0.2.1, 1), 4),fld4: Field::<i16>(Variant(_23.0.2.0, 2), 4),fld5: Field::<u128>(Variant(_23.0.2.1, 1), 6),fld6: Field::<i64>(Variant(_23.0.2.0, 2), 6) };
place!(Field::<u32>(Variant(_35.0.2.1, 2), 1)) = Field::<u32>(Variant(_20.0, 2), 1) << Field::<u32>(Variant(_23.0.2.1, 1), 2);
place!(Field::<bool>(Variant(_20.1, 1), 0)) = !_4;
_35.0.2.0 = Adt18::Variant3 { fld0: Field::<i128>(Variant(_23.0.2.1, 1), 4),fld1: Field::<i64>(Variant(_23.0.2.0, 2), 6),fld2: Field::<u32>(Variant(_23.0.2.0, 2), 1),fld3: 11996540589789905279_u64,fld4: Field::<i16>(Variant(_23.0.2.0, 2), 4) };
_23.0.0 = _15 + _15;
_13 = Field::<char>(Variant(_20.1, 1), 1);
place!(Field::<i128>(Variant(_23.0.2.1, 1), 4)) = Field::<i128>(Variant(_35.0.2.1, 2), 3) >> RET;
place!(Field::<u128>(Variant(_35.0.2.1, 2), 5)) = Field::<u128>(Variant(_23.0.2.0, 2), 5);
_12 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_35.0.2.1, 2), 6)));
_20 = (_23.0.2.0, _35.0.2.1);
place!(Field::<u8>(Variant(_20.0, 2), 2)) = !Field::<u8>(Variant(_23.0.2.0, 2), 2);
SetDiscriminant(_20.1, 2);
_8 = [_28,_28,_28,_28,_28,_28,_28,_28];
_13 = Field::<char>(Variant(_23.0.2.1, 1), 1);
Call(_35.0.0 = core::intrinsics::fmaf64(_23.0.0, _15, _15), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_3 = _7;
_6 = _5 & _5;
_2 = _6 ^ _5;
RET = _2 as u128;
_3 = _8;
_10.3 = [58166_u16,50504_u16,36559_u16,50433_u16];
_6 = _2 ^ _2;
_15 = 76_u8 as f64;
_2 = _6;
_6 = !_2;
_14 = [_13,_13];
_15 = 2546196094_u32 as f64;
RET = !_10.2;
_20.0 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 3171608463_u32,fld3: 38329_u16,fld4: (-113199435642788729792044735316111657221_i128),fld5: 1094817686430837390_usize,fld6: _10.2 };
_11 = 5292532478643473489_usize as f32;
_19 = !_10.2;
_1 = [(-44_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),(-9223372036854775808_isize)];
Call(place!(Field::<u32>(Variant(_20.0, 1), 2)) = core::intrinsics::transmute(Field::<char>(Variant(_20.0, 1), 1)), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_3 = _7;
_6 = _5 & _5;
_2 = _6 ^ _5;
RET = _2 as u128;
_3 = _8;
_10.3 = [58166_u16,50504_u16,36559_u16,50433_u16];
_6 = _2 ^ _2;
_15 = 76_u8 as f64;
_2 = _6;
_6 = !_2;
_14 = [_13,_13];
_15 = 2546196094_u32 as f64;
RET = !_10.2;
_20.0 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 3171608463_u32,fld3: 38329_u16,fld4: (-113199435642788729792044735316111657221_i128),fld5: 1094817686430837390_usize,fld6: _10.2 };
_11 = 5292532478643473489_usize as f32;
_19 = !_10.2;
_1 = [(-44_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),(-9223372036854775808_isize)];
Call(place!(Field::<u32>(Variant(_20.0, 1), 2)) = core::intrinsics::transmute(Field::<char>(Variant(_20.0, 1), 1)), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_6 = _2 ^ Field::<bool>(Variant(_20.0, 1), 0);
place!(Field::<char>(Variant(_20.0, 1), 1)) = _13;
place!(Field::<bool>(Variant(_20.0, 1), 0)) = _6 >= _6;
_17 = core::ptr::addr_of_mut!(_7);
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),87_isize,104_isize,(-39_isize),9223372036854775807_isize,9223372036854775807_isize,54_isize];
place!(Field::<i128>(Variant(_20.0, 1), 4)) = !31927027054662799384905143063403782453_i128;
_13 = Field::<char>(Variant(_20.0, 1), 1);
_10.0 = &_11;
_1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-82_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20.0 = Adt18::Variant1 { fld0: _6,fld1: _13,fld2: 74236859_u32,fld3: 41778_u16,fld4: 88260171104883403498142176733029064318_i128,fld5: 2_usize,fld6: _19 };
_20.0 = Adt18::Variant2 { fld0: _2,fld1: 3719323112_u32,fld2: 80_u8,fld3: 164134513328381288705983453630725335547_i128,fld4: 9142_i16,fld5: _19,fld6: 6930156061856264288_i64 };
_16 = core::ptr::addr_of_mut!(_10.1);
_23.0.2.1 = Adt18::Variant0 { fld0: 593391576749531405_i64,fld1: 63917_u16,fld2: 41_i8 };
_17 = core::ptr::addr_of_mut!(_8);
Goto(bb4)
}
bb18 = {
place!(Field::<i64>(Variant(_23.0.2.1, 0), 0)) = (-7466803111037112392_i64) * 160983610316307753_i64;
(*_17) = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,114_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-70_isize)];
_20.1 = Adt18::Variant1 { fld0: _2,fld1: _13,fld2: 1723731752_u32,fld3: 4363_u16,fld4: (-2545747357044535394382293543583442824_i128),fld5: 3999295640806384871_usize,fld6: RET };
_1 = [(-53_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-48_isize),9223372036854775807_isize,(-47_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<i64>(Variant(_20.0, 2), 6)) = !Field::<i64>(Variant(_23.0.2.1, 0), 0);
_10.2 = !Field::<u128>(Variant(_20.1, 1), 6);
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 10595715277000559776_usize + 12032217758260171505_usize;
_6 = !Field::<bool>(Variant(_20.0, 2), 0);
_10.1 = _14;
_23.0.2.1 = Adt18::Variant0 { fld0: Field::<i64>(Variant(_20.0, 2), 6),fld1: 23050_u16,fld2: 115_i8 };
place!(Field::<u8>(Variant(_20.0, 2), 2)) = (-92_isize) as u8;
place!(Field::<u128>(Variant(_20.1, 1), 6)) = !_10.2;
place!(Field::<i128>(Variant(_20.1, 1), 4)) = !71022137859124588721809640572684087932_i128;
_21 = 14543299821687161115_u64 as i16;
place!(Field::<usize>(Variant(_20.1, 1), 5)) = 2_usize * 136793490246000349_usize;
(*_16) = _14;
place!(Field::<u32>(Variant(_20.0, 2), 1)) = 3598532676_u32;
Goto(bb5)
}
bb19 = {
SetDiscriminant(_23.0.2.1, 3);
_40 = (-2_i8) as i32;
_17 = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant(_23.0.2.0, 3), 0)) = Field::<i128>(Variant(_20.0, 2), 3) ^ Field::<i128>(Variant(_35.0.2.1, 1), 4);
_43.fld0 = Field::<i128>(Variant(_35.0.2.1, 1), 4);
place!(Field::<i64>(Variant(_23.0.2.1, 3), 1)) = Field::<i64>(Variant(_20.1, 2), 6);
(*_22) = &place!(Field::<u32>(Variant(_20.0, 2), 1));
place!(Field::<i16>(Variant(_20.0, 2), 4)) = Field::<i16>(Variant(_20.1, 2), 4);
place!(Field::<i64>(Variant(_23.0.2.0, 3), 1)) = Field::<i64>(Variant(_23.0.2.1, 3), 1) * Field::<i64>(Variant(_20.1, 2), 6);
place!(Field::<i64>(Variant(_23.0.2.0, 3), 1)) = Field::<i64>(Variant(_35.0.2.0, 3), 1);
(*_22) = &place!(Field::<u32>(Variant(_35.0.2.1, 1), 2));
_10.3 = [Field::<u16>(Variant(_35.0.2.1, 1), 3),Field::<u16>(Variant(_35.0.2.1, 1), 3),Field::<u16>(Variant(_35.0.2.1, 1), 3),Field::<u16>(Variant(_35.0.2.1, 1), 3)];
_35.0.2.0 = _20.1;
place!(Field::<i128>(Variant(_20.1, 2), 3)) = _43.fld0;
_43.fld1 = _13;
_35.0.3 = core::ptr::addr_of!((*_22));
place!(Field::<i64>(Variant(_23.0.2.1, 3), 1)) = -Field::<i64>(Variant(_20.0, 2), 6);
_43.fld0 = Field::<i128>(Variant(_23.0.2.0, 3), 0) & Field::<i128>(Variant(_23.0.2.0, 3), 0);
_34 = [_10.2,Field::<u128>(Variant(_20.0, 2), 5),Field::<u128>(Variant(_20.1, 2), 5),Field::<u128>(Variant(_20.1, 2), 5),RET];
place!(Field::<u128>(Variant(_35.0.2.1, 1), 6)) = !Field::<u128>(Variant(_20.0, 2), 5);
_20.0 = _20.1;
_30 = Move(_16);
_31 = &place!(Field::<u32>(Variant(_20.1, 2), 1));
_11 = _15 as f32;
_40 = _29;
(*_22) = &place!(Field::<u32>(Variant(_23.0.2.0, 3), 2));
Goto(bb20)
}
bb20 = {
Call(_48 = dump_var(16_usize, 7_usize, Move(_7), 4_usize, Move(_4), 29_usize, Move(_29), 21_usize, Move(_21)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(16_usize, 6_usize, Move(_6), 1_usize, Move(_1), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: &'static f32,mut _2: *const usize,mut _3: u128,mut _4: *const i8,mut _5: Adt18,mut _6: bool,mut _7: Adt18,mut _8: [isize; 8],mut _9: [isize; 8],mut _10: i16) -> (Adt18, Adt18) {
mir! {
type RET = (Adt18, Adt18);
let _11: Adt83;
let _12: [i16; 7];
let _13: [isize; 8];
let _14: isize;
let _15: *const f32;
let _16: (i32, [u32; 3], *const &'static u32);
let _17: char;
let _18: [i128; 4];
let _19: i128;
let _20: *const [isize; 8];
let _21: *const (&'static f32, [char; 2], u128, [u16; 4]);
let _22: *const &'static u32;
let _23: (Adt18, Adt18);
let _24: ();
let _25: ();
{
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(_5, 1), 1);
place!(Field::<u128>(Variant(_5, 1), 6)) = 8835048294507860885_i64 as u128;
_11.fld0.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_5, 1), 0)));
RET = (_5, _5);
_11.fld5 = core::ptr::addr_of!(_11.fld0.1);
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
RET.0 = Adt18::Variant1 { fld0: _6,fld1: Field::<char>(Variant(RET.1, 1), 1),fld2: Field::<u32>(Variant(_7, 1), 2),fld3: Field::<u16>(Variant(_7, 1), 3),fld4: Field::<i128>(Variant(_7, 1), 4),fld5: Field::<usize>(Variant(_5, 1), 5),fld6: Field::<u128>(Variant(RET.1, 1), 6) };
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(_5, 1), 1);
Goto(bb1)
}
bb1 = {
place!(Field::<u128>(Variant(RET.1, 1), 6)) = !Field::<u128>(Variant(_5, 1), 6);
place!(Field::<i128>(Variant(RET.0, 1), 4)) = Field::<i128>(Variant(_5, 1), 4);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = (-57_i8) as i128;
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
place!(Field::<i128>(Variant(_7, 1), 4)) = (-9223372036854775808_isize) as i128;
place!(Field::<u128>(Variant(RET.0, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
place!(Field::<bool>(Variant(_7, 1), 0)) = _6 ^ Field::<bool>(Variant(_5, 1), 0);
place!(Field::<u128>(Variant(_7, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
_11.fld3.0 = (299140196_i32, 493403606_i32);
RET.0 = _5;
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(RET.0, 1), 1);
place!(Field::<u128>(Variant(RET.1, 1), 6)) = _3 & Field::<u128>(Variant(_7, 1), 6);
place!(Field::<u16>(Variant(_5, 1), 3)) = Field::<u16>(Variant(_7, 1), 3);
place!(Field::<char>(Variant(_5, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
_11.fld0.1 = Field::<i128>(Variant(RET.0, 1), 4);
_11.fld4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),20_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<u32>(Variant(_7, 1), 2)) = Field::<u32>(Variant(_5, 1), 2);
place!(Field::<bool>(Variant(RET.0, 1), 0)) = Field::<bool>(Variant(RET.1, 1), 0);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = -_11.fld0.1;
_11.fld0.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_5, 1), 0)));
_14 = (-9223372036854775808_isize) + (-71_isize);
match _11.fld3.0.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
493403606 => bb8,
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
place!(Field::<i128>(Variant(RET.1, 1), 4)) = _11.fld0.1 - Field::<i128>(Variant(RET.0, 1), 4);
_11.fld0.3 = Move(_11.fld5);
place!(Field::<usize>(Variant(RET.1, 1), 5)) = !Field::<usize>(Variant(RET.0, 1), 5);
place!(Field::<i128>(Variant(RET.0, 1), 4)) = !Field::<i128>(Variant(_7, 1), 4);
SetDiscriminant(RET.0, 2);
place!(Field::<u128>(Variant(RET.1, 1), 6)) = !Field::<u128>(Variant(_5, 1), 6);
match Field::<u32>(Variant(_7, 1), 2) {
0 => bb7,
1 => bb4,
2 => bb3,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
917324346 => bb14,
_ => bb13
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<u128>(Variant(RET.1, 1), 6)) = !Field::<u128>(Variant(_5, 1), 6);
place!(Field::<i128>(Variant(RET.0, 1), 4)) = Field::<i128>(Variant(_5, 1), 4);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = (-57_i8) as i128;
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
place!(Field::<i128>(Variant(_7, 1), 4)) = (-9223372036854775808_isize) as i128;
place!(Field::<u128>(Variant(RET.0, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
place!(Field::<bool>(Variant(_7, 1), 0)) = _6 ^ Field::<bool>(Variant(_5, 1), 0);
place!(Field::<u128>(Variant(_7, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
_11.fld3.0 = (299140196_i32, 493403606_i32);
RET.0 = _5;
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(RET.0, 1), 1);
place!(Field::<u128>(Variant(RET.1, 1), 6)) = _3 & Field::<u128>(Variant(_7, 1), 6);
place!(Field::<u16>(Variant(_5, 1), 3)) = Field::<u16>(Variant(_7, 1), 3);
place!(Field::<char>(Variant(_5, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
_11.fld0.1 = Field::<i128>(Variant(RET.0, 1), 4);
_11.fld4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),20_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<u32>(Variant(_7, 1), 2)) = Field::<u32>(Variant(_5, 1), 2);
place!(Field::<bool>(Variant(RET.0, 1), 0)) = Field::<bool>(Variant(RET.1, 1), 0);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = -_11.fld0.1;
_11.fld0.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_5, 1), 0)));
_14 = (-9223372036854775808_isize) + (-71_isize);
match _11.fld3.0.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
493403606 => bb8,
_ => bb7
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<bool>(Variant(RET.1, 1), 0)) = Field::<bool>(Variant(_5, 1), 0);
_11.fld5 = core::ptr::addr_of!(_11.fld0.1);
place!(Field::<i128>(Variant(_7, 1), 4)) = -Field::<i128>(Variant(RET.1, 1), 4);
match _11.fld3.0.1 {
0 => bb15,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
493403606 => bb21,
_ => bb20
}
}
bb15 = {
Return()
}
bb16 = {
place!(Field::<u128>(Variant(RET.1, 1), 6)) = !Field::<u128>(Variant(_5, 1), 6);
place!(Field::<i128>(Variant(RET.0, 1), 4)) = Field::<i128>(Variant(_5, 1), 4);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = (-57_i8) as i128;
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
place!(Field::<i128>(Variant(_7, 1), 4)) = (-9223372036854775808_isize) as i128;
place!(Field::<u128>(Variant(RET.0, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
place!(Field::<bool>(Variant(_7, 1), 0)) = _6 ^ Field::<bool>(Variant(_5, 1), 0);
place!(Field::<u128>(Variant(_7, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
_11.fld3.0 = (299140196_i32, 493403606_i32);
RET.0 = _5;
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(RET.0, 1), 1);
place!(Field::<u128>(Variant(RET.1, 1), 6)) = _3 & Field::<u128>(Variant(_7, 1), 6);
place!(Field::<u16>(Variant(_5, 1), 3)) = Field::<u16>(Variant(_7, 1), 3);
place!(Field::<char>(Variant(_5, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
_11.fld0.1 = Field::<i128>(Variant(RET.0, 1), 4);
_11.fld4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),20_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<u32>(Variant(_7, 1), 2)) = Field::<u32>(Variant(_5, 1), 2);
place!(Field::<bool>(Variant(RET.0, 1), 0)) = Field::<bool>(Variant(RET.1, 1), 0);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = -_11.fld0.1;
_11.fld0.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_5, 1), 0)));
_14 = (-9223372036854775808_isize) + (-71_isize);
match _11.fld3.0.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
493403606 => bb8,
_ => bb7
}
}
bb17 = {
place!(Field::<u128>(Variant(RET.1, 1), 6)) = !Field::<u128>(Variant(_5, 1), 6);
place!(Field::<i128>(Variant(RET.0, 1), 4)) = Field::<i128>(Variant(_5, 1), 4);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = (-57_i8) as i128;
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
place!(Field::<i128>(Variant(_7, 1), 4)) = (-9223372036854775808_isize) as i128;
place!(Field::<u128>(Variant(RET.0, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
place!(Field::<bool>(Variant(_7, 1), 0)) = _6 ^ Field::<bool>(Variant(_5, 1), 0);
place!(Field::<u128>(Variant(_7, 1), 6)) = !Field::<u128>(Variant(RET.1, 1), 6);
_11.fld3.0 = (299140196_i32, 493403606_i32);
RET.0 = _5;
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(RET.0, 1), 1);
place!(Field::<u128>(Variant(RET.1, 1), 6)) = _3 & Field::<u128>(Variant(_7, 1), 6);
place!(Field::<u16>(Variant(_5, 1), 3)) = Field::<u16>(Variant(_7, 1), 3);
place!(Field::<char>(Variant(_5, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
_11.fld0.1 = Field::<i128>(Variant(RET.0, 1), 4);
_11.fld4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),20_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<u32>(Variant(_7, 1), 2)) = Field::<u32>(Variant(_5, 1), 2);
place!(Field::<bool>(Variant(RET.0, 1), 0)) = Field::<bool>(Variant(RET.1, 1), 0);
place!(Field::<i128>(Variant(RET.1, 1), 4)) = -_11.fld0.1;
_11.fld0.2 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_5, 1), 0)));
_14 = (-9223372036854775808_isize) + (-71_isize);
match _11.fld3.0.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
493403606 => bb8,
_ => bb7
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
place!(Field::<bool>(Variant(RET.0, 2), 0)) = Field::<bool>(Variant(_5, 1), 0);
place!(Field::<i16>(Variant(RET.0, 2), 4)) = !_10;
_3 = Field::<u128>(Variant(RET.1, 1), 6) & Field::<u128>(Variant(RET.1, 1), 6);
place!(Field::<u32>(Variant(_7, 1), 2)) = Field::<u32>(Variant(_5, 1), 2) & Field::<u32>(Variant(_5, 1), 2);
_11.fld3.0 = (219150015_i32, (-1149608177_i32));
place!(Field::<u8>(Variant(RET.0, 2), 2)) = 156_u8;
RET.1 = Adt18::Variant0 { fld0: 5181650178221964825_i64,fld1: Field::<u16>(Variant(_5, 1), 3),fld2: (-39_i8) };
place!(Field::<i8>(Variant(RET.1, 0), 2)) = 12514553904899629812_u64 as i8;
_11.fld0.2 = core::ptr::addr_of_mut!(_6);
Call(place!(Field::<i64>(Variant(RET.0, 2), 6)) = core::intrinsics::bswap((-3938434252212138993_i64)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
RET.0 = Adt18::Variant1 { fld0: _6,fld1: Field::<char>(Variant(_5, 1), 1),fld2: Field::<u32>(Variant(_5, 1), 2),fld3: Field::<u16>(Variant(_5, 1), 3),fld4: Field::<i128>(Variant(_5, 1), 4),fld5: Field::<usize>(Variant(_7, 1), 5),fld6: _3 };
_11.fld1 = _14 as f32;
_17 = Field::<char>(Variant(_5, 1), 1);
place!(Field::<bool>(Variant(_7, 1), 0)) = Field::<bool>(Variant(RET.0, 1), 0);
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
place!(Field::<bool>(Variant(_7, 1), 0)) = _11.fld1 < _11.fld1;
place!(Field::<i8>(Variant(RET.1, 0), 2)) = !(-90_i8);
_7 = _5;
place!(Field::<i64>(Variant(RET.1, 0), 0)) = _11.fld1 as i64;
_9 = _11.fld4;
_16.1 = [Field::<u32>(Variant(_5, 1), 2),Field::<u32>(Variant(_7, 1), 2),Field::<u32>(Variant(_5, 1), 2)];
RET.1 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_7, 1), 0),fld1: Field::<u32>(Variant(_5, 1), 2),fld2: 237_u8,fld3: Field::<i128>(Variant(RET.0, 1), 4),fld4: _10,fld5: Field::<u128>(Variant(RET.0, 1), 6),fld6: 3200598054654919222_i64 };
_14 = 9223372036854775807_isize * 9223372036854775807_isize;
place!(Field::<u16>(Variant(RET.0, 1), 3)) = Field::<u16>(Variant(_7, 1), 3);
place!(Field::<i16>(Variant(RET.1, 2), 4)) = _10 | _10;
place!(Field::<bool>(Variant(RET.1, 2), 0)) = !Field::<bool>(Variant(_5, 1), 0);
RET.1 = Adt18::Variant1 { fld0: Field::<bool>(Variant(_7, 1), 0),fld1: _17,fld2: Field::<u32>(Variant(_5, 1), 2),fld3: Field::<u16>(Variant(_5, 1), 3),fld4: Field::<i128>(Variant(RET.0, 1), 4),fld5: Field::<usize>(Variant(RET.0, 1), 5),fld6: Field::<u128>(Variant(RET.0, 1), 6) };
place!(Field::<u32>(Variant(RET.0, 1), 2)) = Field::<u128>(Variant(_5, 1), 6) as u32;
_5 = RET.1;
_16.0 = _11.fld3.0.0 & _11.fld3.0.0;
_11.fld3.0.0 = _11.fld3.0.1 | _16.0;
place!(Field::<char>(Variant(RET.0, 1), 1)) = Field::<char>(Variant(RET.1, 1), 1);
place!(Field::<char>(Variant(RET.1, 1), 1)) = Field::<char>(Variant(_7, 1), 1);
place!(Field::<char>(Variant(RET.1, 1), 1)) = Field::<char>(Variant(RET.0, 1), 1);
Goto(bb23)
}
bb23 = {
Call(_24 = dump_var(17_usize, 14_usize, Move(_14), 8_usize, Move(_8), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: f64,mut _2: f64,mut _3: f64) -> i32 {
mir! {
type RET = i32;
let _4: *mut bool;
let _5: f64;
let _6: isize;
let _7: [usize; 3];
let _8: *const bool;
let _9: isize;
let _10: [i16; 7];
let _11: (i32, [u32; 3], *const &'static u32);
let _12: *const Adt18;
let _13: (isize,);
let _14: *const i128;
let _15: (i32, i32);
let _16: isize;
let _17: isize;
let _18: [u8; 2];
let _19: Adt18;
let _20: Adt71;
let _21: ();
let _22: ();
{
RET = (-2013117014_i32) << (-9223372036854775808_isize);
_1 = _2;
_3 = 1135083813_u32 as f64;
RET = (-1127785529_i32);
RET = (-1991550334_i32) - (-202855274_i32);
_1 = _2;
_5 = _1;
RET = -(-1005710727_i32);
RET = 676086938_i32;
_7 = [2_usize,12933128381357592370_usize,3_usize];
_5 = 238_u8 as f64;
_2 = 52_u8 as f64;
_5 = -_1;
RET = 25740_i16 as i32;
RET = -1262183828_i32;
_7 = [0_usize,12680634022197555869_usize,275854162303927608_usize];
_7 = [7_usize,5_usize,3935432035702689949_usize];
RET = (-29_isize) as i32;
_3 = _5 - _5;
_10 = [9105_i16,4950_i16,(-27207_i16),(-13967_i16),(-11998_i16),(-3642_i16),(-4988_i16)];
RET = !914668537_i32;
_5 = _1;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
Call(_6 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = -_3;
_9 = !_6;
_1 = 47_i8 as f64;
Goto(bb2)
}
bb2 = {
_2 = _5 * _3;
_13.0 = _9 + _9;
_7 = [676466613757546934_usize,5075433117472567668_usize,7_usize];
_7 = [15567106405701243207_usize,6_usize,4_usize];
_10 = [26355_i16,(-29884_i16),2395_i16,3650_i16,(-6315_i16),(-21457_i16),(-25143_i16)];
_6 = _13.0 ^ _13.0;
Goto(bb3)
}
bb3 = {
_11.1 = [1934222931_u32,841491091_u32,3500424622_u32];
RET = 8449252934678832847_i64 as i32;
_11.0 = 0_usize as i32;
_13 = (_6,);
RET = 12091774362794389321100456701421777955_i128 as i32;
_7 = [4163295872084833882_usize,3684221291369859602_usize,10023019921362952086_usize];
Goto(bb4)
}
bb4 = {
RET = -_11.0;
_13.0 = 717082814861207380_i64 as isize;
_6 = !_9;
_15.1 = _3 as i32;
_15.0 = !_15.1;
_11.0 = _15.0;
_13 = (_6,);
_10 = [(-498_i16),7002_i16,(-28276_i16),(-31472_i16),19171_i16,(-10774_i16),22348_i16];
_13.0 = _9 ^ _9;
_3 = -_2;
_13.0 = _9;
_5 = _2 * _3;
_6 = '\u{c83f1}' as isize;
_11.1 = [3517502077_u32,2869627226_u32,1142357459_u32];
_15.0 = -_11.0;
_6 = _11.0 as isize;
_1 = -_5;
Goto(bb5)
}
bb5 = {
_3 = -_5;
_5 = _2 * _3;
_1 = _6 as f64;
_7 = [12314427960210493897_usize,562713786943949322_usize,13265312646218801570_usize];
_5 = 20561_u16 as f64;
_10 = [27963_i16,28545_i16,27104_i16,29447_i16,(-4422_i16),(-15191_i16),25191_i16];
_15 = (_11.0, _11.0);
_1 = _5;
RET = (-9035782508094444590_i64) as i32;
_9 = _6 + _6;
_2 = -_3;
_18 = [224_u8,247_u8];
_18 = [161_u8,178_u8];
_3 = _2 + _2;
RET = _15.1 >> _6;
_3 = _2;
_13.0 = -_9;
_10 = [(-29279_i16),21582_i16,(-20053_i16),31445_i16,8353_i16,(-32321_i16),5972_i16];
RET = !_15.0;
_5 = _2 - _3;
_1 = _3 - _3;
Goto(bb6)
}
bb6 = {
Call(_21 = dump_var(18_usize, 15_usize, Move(_15), 10_usize, Move(_10), 6_usize, Move(_6), 22_usize, _22), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static (Adt18, Adt18),mut _2: f64) -> (f32, i128, *mut bool, *const i128) {
mir! {
type RET = (f32, i128, *mut bool, *const i128);
let _3: isize;
let _4: [isize; 8];
let _5: [isize; 8];
let _6: [u16; 4];
let _7: (f32, i128, *mut bool, *const i128);
let _8: char;
let _9: isize;
let _10: [u128; 5];
let _11: bool;
let _12: &'static *mut [isize; 8];
let _13: *mut u8;
let _14: &'static u128;
let _15: isize;
let _16: ();
let _17: ();
{
RET.3 = core::ptr::addr_of!(RET.1);
_2 = 221_u8 as f64;
RET.1 = (-134813358395749642728885537038699790869_i128);
Goto(bb1)
}
bb1 = {
_4 = [113_isize,9223372036854775807_isize,116_isize,9223372036854775807_isize,(-106_isize),18_isize,(-99_isize),9223372036854775807_isize];
RET.1 = (-131336148336230201286664614729578627807_i128);
RET.1 = !(-140230643816086673493702597418924907310_i128);
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = (-50892322109675650667825066718483321696_i128);
RET.1 = _2 as i128;
RET.0 = (-9223372036854775808_isize) as f32;
RET.1 = 69183365557262963423742418349852622280_i128;
_3 = (-9223372036854775808_isize);
_3 = 9223372036854775807_isize;
_5 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = !(-122254656213018266647501381156325264711_i128);
_2 = 63_i8 as f64;
match _3 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET.3 = core::ptr::addr_of!(RET.1);
RET.3 = core::ptr::addr_of!(RET.1);
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_5 = _4;
_7.0 = 34_u8 as f32;
RET.1 = 1969524112_u32 as i128;
RET.1 = (-2737803527688439887132634996987590268_i128) << _3;
_7.0 = -RET.0;
_6 = [874_u16,13729_u16,62824_u16,46025_u16];
RET.0 = -_7.0;
_2 = 306208877_i32 as f64;
_7.1 = RET.1 * RET.1;
RET.3 = core::ptr::addr_of!(_7.1);
_4 = _5;
RET.1 = -_7.1;
RET.1 = _7.1 >> _7.1;
_7.3 = core::ptr::addr_of!(_7.1);
RET.0 = 113808694947092693389953878149714777416_u128 as f32;
_3 = (-9223372036854775808_isize);
_8 = '\u{32a8b}';
_8 = '\u{36bb3}';
match _3 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb4 = {
Return()
}
bb5 = {
_4 = [113_isize,9223372036854775807_isize,116_isize,9223372036854775807_isize,(-106_isize),18_isize,(-99_isize),9223372036854775807_isize];
RET.1 = (-131336148336230201286664614729578627807_i128);
RET.1 = !(-140230643816086673493702597418924907310_i128);
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = (-50892322109675650667825066718483321696_i128);
RET.1 = _2 as i128;
RET.0 = (-9223372036854775808_isize) as f32;
RET.1 = 69183365557262963423742418349852622280_i128;
_3 = (-9223372036854775808_isize);
_3 = 9223372036854775807_isize;
_5 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = !(-122254656213018266647501381156325264711_i128);
_2 = 63_i8 as f64;
match _3 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET.2 = core::ptr::addr_of_mut!(_11);
RET.1 = _7.1 ^ _7.1;
_2 = (-8094625230972947738_i64) as f64;
_5 = [_3,_3,_3,_3,_3,_3,_3,_3];
match _3 {
0 => bb2,
1 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb9 = {
_4 = [113_isize,9223372036854775807_isize,116_isize,9223372036854775807_isize,(-106_isize),18_isize,(-99_isize),9223372036854775807_isize];
RET.1 = (-131336148336230201286664614729578627807_i128);
RET.1 = !(-140230643816086673493702597418924907310_i128);
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = (-50892322109675650667825066718483321696_i128);
RET.1 = _2 as i128;
RET.0 = (-9223372036854775808_isize) as f32;
RET.1 = 69183365557262963423742418349852622280_i128;
_3 = (-9223372036854775808_isize);
_3 = 9223372036854775807_isize;
_5 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = !(-122254656213018266647501381156325264711_i128);
_2 = 63_i8 as f64;
match _3 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb10 = {
RET.3 = core::ptr::addr_of!(RET.1);
RET.3 = core::ptr::addr_of!(RET.1);
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_5 = _4;
_7.0 = 34_u8 as f32;
RET.1 = 1969524112_u32 as i128;
RET.1 = (-2737803527688439887132634996987590268_i128) << _3;
_7.0 = -RET.0;
_6 = [874_u16,13729_u16,62824_u16,46025_u16];
RET.0 = -_7.0;
_2 = 306208877_i32 as f64;
_7.1 = RET.1 * RET.1;
RET.3 = core::ptr::addr_of!(_7.1);
_4 = _5;
RET.1 = -_7.1;
RET.1 = _7.1 >> _7.1;
_7.3 = core::ptr::addr_of!(_7.1);
RET.0 = 113808694947092693389953878149714777416_u128 as f32;
_3 = (-9223372036854775808_isize);
_8 = '\u{32a8b}';
_8 = '\u{36bb3}';
match _3 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb11 = {
_8 = '\u{3cae6}';
_9 = 2989267615_u32 as isize;
_10 = [93689853880233137379274988784696064830_u128,295400776185671240347024826594915388419_u128,33547338684747173456469360485930970049_u128,156827104999822014301110201142851464866_u128,33191952474966501052661005128514848856_u128];
_7.2 = core::ptr::addr_of_mut!(_11);
_11 = !false;
_7 = (RET.0, RET.1, Move(RET.2), Move(RET.3));
RET.2 = Move(_7.2);
RET.1 = !_7.1;
RET.3 = core::ptr::addr_of!(_7.1);
_4 = [_9,_9,_3,_3,_3,_3,_3,_3];
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
RET.0 = -_7.0;
RET.1 = _7.1 + _7.1;
_7.1 = RET.1;
RET.3 = Move(_7.3);
_7.0 = RET.0;
RET.1 = _8 as i128;
_7.3 = core::ptr::addr_of!(_7.1);
RET.1 = _7.1;
RET.2 = core::ptr::addr_of_mut!(_11);
RET.0 = -_7.0;
RET.3 = core::ptr::addr_of!(RET.1);
RET.1 = _9 as i128;
_5 = [_9,_3,_9,_3,_9,_3,_3,_9];
Goto(bb14)
}
bb14 = {
Call(_16 = dump_var(19_usize, 4_usize, Move(_4), 9_usize, Move(_9), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(226_u8), std::hint::black_box(191264267476413042211506891430219772312_u128), std::hint::black_box(17885386871861723326_u64), std::hint::black_box((-14702801230402026209335771865615417282_i128)));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: i64,
fld1: u16,
fld2: i8,

},
Variant1{
fld0: bool,
fld1: char,
fld2: u32,
fld3: u16,
fld4: i128,
fld5: usize,
fld6: u128,

},
Variant2{
fld0: bool,
fld1: u32,
fld2: u8,
fld3: i128,
fld4: i16,
fld5: u128,
fld6: i64,

},
Variant3{
fld0: i128,
fld1: i64,
fld2: u32,
fld3: u64,
fld4: i16,

}}
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: *mut bool,
fld1: f64,
fld2: f32,
fld3: i8,
fld4: u16,
fld5: u128,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: f32,

},
Variant2{
fld0: f32,
fld1: char,
fld2: f64,
fld3: Adt18,

},
Variant3{
fld0: f64,
fld1: (i32, i32),
fld2: isize,
fld3: usize,

}}
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: i128,
fld1: char,
fld2: Adt18,
fld3: Adt22,
}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: *const u16,
fld1: [u32; 3],
fld2: isize,
fld3: *const f32,
fld4: f32,
}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){unsafe{printf("Adt71::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt71 {
Variant0{
fld0: Adt40,
fld1: Adt24,
fld2: [i16; 7],
fld3: (Adt18, Adt18),
fld4: *const i128,
fld5: [u16; 4],

},
Variant1{
fld0: usize,
fld1: *const Adt18,
fld2: isize,
fld3: *mut bool,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: (f32, i16, *mut bool, char),
fld1: *const i128,
fld2: *const f32,

},
Variant1{
fld0: f32,

},
Variant2{
fld0: u32,
fld1: (isize,),
fld2: *const *mut i64,
fld3: usize,
fld4: [u16; 4],
fld5: (Adt18, Adt18),
fld6: *const i8,
fld7: [usize; 3],

},
Variant3{
fld0: Adt40,

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: *mut *mut u8,
fld1: *const f32,
fld2: u16,
fld3: *const Adt18,
fld4: *mut *const u16,
fld5: Adt40,
fld6: u64,
fld7: f32,

},
Variant1{
fld0: bool,
fld1: [char; 2],
fld2: isize,
fld3: i8,
fld4: ([isize; 8], *const *mut i64, *const *mut *const u16, *mut i64),
fld5: *mut *const u16,
fld6: *const i8,

}}
impl PrintFDebug for Adt83{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt83{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt83 {
fld0: (f32, i128, *mut bool, *const i128),
fld1: f32,
fld2: isize,
fld3: ((i32, i32), *const Adt18),
fld4: [isize; 8],
fld5: *const i128,
}

