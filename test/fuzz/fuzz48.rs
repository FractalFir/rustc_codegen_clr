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
pub fn fn0(mut _1: bool,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u32,mut _8: i128,mut _9: u128,mut _10: u8) -> Adt55 {
mir! {
type RET = Adt55;
let _11: (u8, u64, f32);
let _12: bool;
let _13: ([u32; 2], i16);
let _14: Adt42;
let _15: [u32; 5];
let _16: (*const i128, [i16; 6], u64);
let _17: Adt55;
let _18: f64;
let _19: i32;
let _20: *const (u64, u128, u32, isize, i8, i16, usize);
let _21: f64;
let _22: f64;
let _23: [bool; 5];
let _24: (i8, u64);
let _25: (u64, u128, u32, isize, i8, i16, usize);
let _26: f32;
let _27: char;
let _28: [u32; 5];
let _29: f32;
let _30: [i16; 8];
let _31: (i8, u64);
let _32: f32;
let _33: i32;
let _34: Adt53;
let _35: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _36: ([i64; 7], [u32; 7], u32);
let _37: isize;
let _38: char;
let _39: Adt58;
let _40: (u64, u128, u32, isize, i8, i16, usize);
let _41: i8;
let _42: [bool; 5];
let _43: u16;
let _44: [u64; 2];
let _45: u64;
let _46: [i16; 8];
let _47: Adt53;
let _48: Adt58;
let _49: [i128; 4];
let _50: (u8, u64, f32);
let _51: [i128; 4];
let _52: f32;
let _53: f32;
let _54: f64;
let _55: char;
let _56: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32);
let _57: Adt54;
let _58: f64;
let _59: [bool; 5];
let _60: Adt54;
let _61: [i16; 8];
let _62: [i16; 6];
let _63: f32;
let _64: [u64; 2];
let _65: char;
let _66: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _67: ([i64; 7], [u32; 7], u32);
let _68: isize;
let _69: [u32; 7];
let _70: ([i64; 7], [u32; 7], u32);
let _71: (*const i128, [i16; 6], u64);
let _72: [i16; 8];
let _73: f32;
let _74: (u64, u128, u32, isize, i8, i16, usize);
let _75: Adt56;
let _76: i64;
let _77: [i64; 7];
let _78: [i128; 4];
let _79: isize;
let _80: i128;
let _81: i32;
let _82: isize;
let _83: Adt58;
let _84: Adt42;
let _85: ();
let _86: ();
{
_3 = (-115_isize);
_1 = false;
_4 = 551037378_i32 as i8;
_11.0 = 234_u8;
_5 = (-923_i16) | (-16731_i16);
_11.1 = 13917251981558062505_u64;
_5 = 3579_i16 & 16095_i16;
_9 = 14478011412741542176_usize as u128;
_10 = _11.0 * _11.0;
_12 = !_1;
_12 = _9 > _9;
_6 = (-1708220327_i32) & 442753857_i32;
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
234 => bb7,
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
RET = Adt55::Variant3 { fld0: _12 };
_5 = (-10374_i16);
_1 = _10 < _10;
_9 = !66124215064843095444173918590662143106_u128;
_11.2 = _5 as f32;
_4 = 98_i8;
place!(Field::<bool>(Variant(RET, 3), 0)) = _12 & _1;
_13.1 = _5;
_2 = _11.1;
place!(Field::<bool>(Variant(RET, 3), 0)) = !_1;
_4 = (-6957880788109162674_i64) as i8;
SetDiscriminant(RET, 1);
_11.1 = (-67371748112524700925768038408881425254_i128) as u64;
_12 = _13.1 >= _5;
RET = Adt55::Variant3 { fld0: _12 };
_9 = 153767096466764168249958808609613916228_u128;
_11.1 = _2;
_15 = [3640378803_u32,3541941442_u32,1655233790_u32,2975562052_u32,3977880552_u32];
_12 = _1 <= _1;
_8 = _13.1 as i128;
_6 = (-891442121_i32);
_18 = _11.1 as f64;
_1 = !Field::<bool>(Variant(RET, 3), 0);
_3 = _2 as isize;
Call(_8 = fn1(_12, _18, Move(RET), _3, _13.1, _4, _5, _11.2, _5, _12, _15, _9, _12, _10, _5, _11.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = _11.1;
_13.0 = [2283775998_u32,3538932893_u32];
_11.0 = _10 << _3;
_21 = _6 as f64;
_8 = 12781480723988736855195635615874332107_i128 << _2;
Goto(bb9)
}
bb9 = {
_19 = _4 as i32;
_23 = [_12,_12,_12,_12,_12];
_9 = 96905456233014165432441379419087829483_u128 << _4;
_22 = -_18;
_16.2 = _2;
_18 = _22 + _21;
Goto(bb10)
}
bb10 = {
_25.2 = 49046_u16 as u32;
_24 = (_4, _11.1);
_16.0 = core::ptr::addr_of!(_8);
_27 = '\u{7524a}';
_25.0 = _6 as u64;
_14 = Adt42::Variant1 { fld0: _13.1,fld1: _24,fld2: _3 };
_13.1 = -_5;
_22 = -_21;
_9 = !73876905601023303108341035664830284938_u128;
_6 = _19;
_16.0 = core::ptr::addr_of!(_8);
_22 = _8 as f64;
match _11.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
13917251981558062505 => bb11,
_ => bb6
}
}
bb11 = {
_28 = [_25.2,_25.2,_25.2,_25.2,_25.2];
_23 = [_12,_12,_12,_12,_12];
_5 = Field::<i16>(Variant(_14, 1), 0);
_11.0 = _10 * _10;
_1 = _18 < _21;
place!(Field::<isize>(Variant(_14, 1), 2)) = _3 | _3;
_23 = [_12,_12,_12,_12,_12];
SetDiscriminant(_14, 1);
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)).0 = _4 | _4;
_26 = -_11.2;
_25.1 = _9 ^ _9;
_30 = [_5,_5,_13.1,_13.1,_5,_13.1,_5,_13.1];
_3 = 9223372036854775807_isize >> _11.0;
_25.6 = !4_usize;
_27 = '\u{7ebff}';
_25.4 = _4;
_18 = _21;
Goto(bb12)
}
bb12 = {
place!(Field::<isize>(Variant(_14, 1), 2)) = _3 + _3;
_20 = core::ptr::addr_of!(_25);
(*_20).6 = 8120951537058182696_usize;
_30 = [_13.1,_13.1,_5,_13.1,_5,_13.1,_5,_13.1];
Call(_5 = core::intrinsics::transmute(_13.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26 = (*_20).2 as f32;
_29 = -_26;
(*_20).3 = Field::<isize>(Variant(_14, 1), 2) << _9;
_16.0 = core::ptr::addr_of!(_8);
_11.1 = !(*_20).0;
place!(Field::<i16>(Variant(_14, 1), 0)) = _1 as i16;
_31 = ((*_20).4, _2);
(*_20) = (_11.1, _9, 3596311351_u32, Field::<isize>(Variant(_14, 1), 2), _31.0, Field::<i16>(Variant(_14, 1), 0), 3_usize);
_25.0 = _11.0 as u64;
(*_20).3 = _3;
_34.fld0 = core::ptr::addr_of!((*_20));
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)).0 = !_4;
(*_20) = (_2, _9, 2817652376_u32, _3, _24.0, _13.1, 17678464581639033613_usize);
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)) = (_4, _31.1);
_25.4 = _22 as i8;
_7 = (*_20).2;
place!(Field::<isize>(Variant(_14, 1), 2)) = _24.0 as isize;
_25.1 = _9;
match _25.6 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
17678464581639033613 => bb21,
_ => bb20
}
}
bb14 = {
Return()
}
bb15 = {
_28 = [_25.2,_25.2,_25.2,_25.2,_25.2];
_23 = [_12,_12,_12,_12,_12];
_5 = Field::<i16>(Variant(_14, 1), 0);
_11.0 = _10 * _10;
_1 = _18 < _21;
place!(Field::<isize>(Variant(_14, 1), 2)) = _3 | _3;
_23 = [_12,_12,_12,_12,_12];
SetDiscriminant(_14, 1);
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)).0 = _4 | _4;
_26 = -_11.2;
_25.1 = _9 ^ _9;
_30 = [_5,_5,_13.1,_13.1,_5,_13.1,_5,_13.1];
_3 = 9223372036854775807_isize >> _11.0;
_25.6 = !4_usize;
_27 = '\u{7ebff}';
_25.4 = _4;
_18 = _21;
Goto(bb12)
}
bb16 = {
_25.2 = 49046_u16 as u32;
_24 = (_4, _11.1);
_16.0 = core::ptr::addr_of!(_8);
_27 = '\u{7524a}';
_25.0 = _6 as u64;
_14 = Adt42::Variant1 { fld0: _13.1,fld1: _24,fld2: _3 };
_13.1 = -_5;
_22 = -_21;
_9 = !73876905601023303108341035664830284938_u128;
_6 = _19;
_16.0 = core::ptr::addr_of!(_8);
_22 = _8 as f64;
match _11.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
13917251981558062505 => bb11,
_ => bb6
}
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
Return()
}
bb21 = {
_34.fld2.2 = core::ptr::addr_of!(_35.1);
_11 = (_10, _24.1, _26);
_24.1 = (*_20).0;
SetDiscriminant(_14, 3);
_36.0 = [23714737460091871_i64,(-8702167479719249733_i64),8165666954913568424_i64,(-5294012945078587987_i64),3968047886119658516_i64,9127859462655174865_i64,(-7891999803489366072_i64)];
_35.0 = _3;
_27 = '\u{103f79}';
_34.fld2.0.0 = [(-8863657551763628861_i64),(-8625131517663534107_i64),3682750412905977647_i64,7617254890110108535_i64,(-821704354385674941_i64),(-5020717124171718397_i64),8805940105260287937_i64];
place!(Field::<(i8, u64)>(Variant(_14, 3), 2)) = (_31.0, (*_20).0);
_35.1.1 = (*_20).1;
Goto(bb22)
}
bb22 = {
_10 = _11.0;
_30 = [_13.1,_5,(*_20).5,_13.1,_25.5,(*_20).5,(*_20).5,(*_20).5];
_11.2 = -_29;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)).2 = -_35.0;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)).0 = _16.0;
(*_20) = (_16.2, _35.1.1, _7, _35.0, Field::<(i8, u64)>(Variant(_14, 3), 2).0, _13.1, 5_usize);
place!(Field::<[i64; 7]>(Variant(_14, 3), 1)) = [7403716720763426859_i64,(-1471801051348331509_i64),632073364938599611_i64,(-2738905549589150369_i64),2995749120028947227_i64,5183636434203463437_i64,247048994226217483_i64];
_11.2 = _29;
(*_20).3 = -_35.0;
_17 = Adt55::Variant3 { fld0: _1 };
Goto(bb23)
}
bb23 = {
_25.3 = Field::<(i8, u64)>(Variant(_14, 3), 2).0 as isize;
_20 = _34.fld0;
_13.1 = !_25.5;
_40.3 = _3 >> (*_20).2;
_35.1 = (_25.0, _9, (*_20).2, _40.3, _24.0, _25.5, (*_20).6);
_33 = _8 as i32;
_40.0 = _35.1.0 ^ _31.1;
_5 = _13.1;
(*_20).1 = !_9;
place!(Field::<bool>(Variant(_17, 3), 0)) = !_12;
_40.2 = !(*_20).2;
_35.2 = _10;
RET = Move(_17);
_1 = Field::<bool>(Variant(RET, 3), 0);
_17 = Move(RET);
place!(Field::<([i64; 7], [u32; 7], u32)>(Variant(_14, 3), 0)).0 = [8657546575531532924_i64,(-8659985756334208050_i64),(-8481711119342349434_i64),7893724746417099451_i64,6663557001600622305_i64,2694329023884488390_i64,2044583204588475742_i64];
_17 = Adt55::Variant3 { fld0: _12 };
_36.2 = _35.1.2 % (*_20).2;
_40.1 = !_9;
match _24.1 {
0 => bb11,
1 => bb17,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
6 => bb28,
13917251981558062505 => bb30,
_ => bb29
}
}
bb24 = {
_10 = _11.0;
_30 = [_13.1,_5,(*_20).5,_13.1,_25.5,(*_20).5,(*_20).5,(*_20).5];
_11.2 = -_29;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)).2 = -_35.0;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)).0 = _16.0;
(*_20) = (_16.2, _35.1.1, _7, _35.0, Field::<(i8, u64)>(Variant(_14, 3), 2).0, _13.1, 5_usize);
place!(Field::<[i64; 7]>(Variant(_14, 3), 1)) = [7403716720763426859_i64,(-1471801051348331509_i64),632073364938599611_i64,(-2738905549589150369_i64),2995749120028947227_i64,5183636434203463437_i64,247048994226217483_i64];
_11.2 = _29;
(*_20).3 = -_35.0;
_17 = Adt55::Variant3 { fld0: _1 };
Goto(bb23)
}
bb25 = {
_19 = _4 as i32;
_23 = [_12,_12,_12,_12,_12];
_9 = 96905456233014165432441379419087829483_u128 << _4;
_22 = -_18;
_16.2 = _2;
_18 = _22 + _21;
Goto(bb10)
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
_26 = (*_20).2 as f32;
_29 = -_26;
(*_20).3 = Field::<isize>(Variant(_14, 1), 2) << _9;
_16.0 = core::ptr::addr_of!(_8);
_11.1 = !(*_20).0;
place!(Field::<i16>(Variant(_14, 1), 0)) = _1 as i16;
_31 = ((*_20).4, _2);
(*_20) = (_11.1, _9, 3596311351_u32, Field::<isize>(Variant(_14, 1), 2), _31.0, Field::<i16>(Variant(_14, 1), 0), 3_usize);
_25.0 = _11.0 as u64;
(*_20).3 = _3;
_34.fld0 = core::ptr::addr_of!((*_20));
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)).0 = !_4;
(*_20) = (_2, _9, 2817652376_u32, _3, _24.0, _13.1, 17678464581639033613_usize);
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)) = (_4, _31.1);
_25.4 = _22 as i8;
_7 = (*_20).2;
place!(Field::<isize>(Variant(_14, 1), 2)) = _24.0 as isize;
_25.1 = _9;
match _25.6 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
17678464581639033613 => bb21,
_ => bb20
}
}
bb29 = {
_28 = [_25.2,_25.2,_25.2,_25.2,_25.2];
_23 = [_12,_12,_12,_12,_12];
_5 = Field::<i16>(Variant(_14, 1), 0);
_11.0 = _10 * _10;
_1 = _18 < _21;
place!(Field::<isize>(Variant(_14, 1), 2)) = _3 | _3;
_23 = [_12,_12,_12,_12,_12];
SetDiscriminant(_14, 1);
place!(Field::<(i8, u64)>(Variant(_14, 1), 1)).0 = _4 | _4;
_26 = -_11.2;
_25.1 = _9 ^ _9;
_30 = [_5,_5,_13.1,_13.1,_5,_13.1,_5,_13.1];
_3 = 9223372036854775807_isize >> _11.0;
_25.6 = !4_usize;
_27 = '\u{7ebff}';
_25.4 = _4;
_18 = _21;
Goto(bb12)
}
bb30 = {
_47.fld2.0.0 = [(-7441689399617410016_i64),(-1060427873141336559_i64),(-4021063904595987729_i64),(-7554466796178557676_i64),6764180256653318897_i64,4090177847147432089_i64,(-8052562491557141077_i64)];
RET = Adt55::Variant3 { fld0: _12 };
_40 = (*_20);
_30 = [_40.5,_13.1,_25.5,_13.1,_40.5,_13.1,_5,_13.1];
(*_20).0 = !_2;
SetDiscriminant(_17, 1);
_4 = _35.1.4 ^ _31.0;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)) = (_16.0, _27, _35.1.3, _27, _15, (*_20).6);
_47.fld2.0.2 = (*_20).6 as u32;
(*_20).6 = !Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).5;
(*_20).3 = Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).2 & _35.1.3;
_34.fld1 = [(*_20).5,_40.5,_35.1.5,_35.1.5,_13.1,(*_20).5];
_32 = -_26;
_24.0 = _4 - _25.4;
_42 = [_12,Field::<bool>(Variant(RET, 3), 0),_1,Field::<bool>(Variant(RET, 3), 0),_1];
_24 = (_25.4, _11.1);
_16.1 = [_5,_13.1,_25.5,_40.5,(*_20).5,(*_20).5];
SetDiscriminant(RET, 1);
_47.fld2.0.2 = !(*_20).2;
Call(place!(Field::<([u32; 2], i16)>(Variant(RET, 1), 3)).1 = fn6(_34.fld2.0.0, _34.fld2.2, _34.fld0, Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3), _34.fld0, _34.fld2.2, _3, (*_20).3, _34.fld2.2, Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).2, (*_20).3, Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).5), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_36.0 = [(-4441574117583548415_i64),3544228958592730943_i64,8172153290629766834_i64,(-7264238060293641269_i64),(-8825238871729539597_i64),(-8474115574370499765_i64),(-3089771297330396163_i64)];
place!(Field::<([u32; 2], i16)>(Variant(RET, 1), 3)).0 = _13.0;
(*_20).1 = _35.1.1;
Goto(bb32)
}
bb32 = {
_25.5 = !Field::<([u32; 2], i16)>(Variant(RET, 1), 3).1;
match _7 {
0 => bb7,
1 => bb5,
2 => bb31,
2817652376 => bb34,
_ => bb33
}
}
bb33 = {
_25.2 = 49046_u16 as u32;
_24 = (_4, _11.1);
_16.0 = core::ptr::addr_of!(_8);
_27 = '\u{7524a}';
_25.0 = _6 as u64;
_14 = Adt42::Variant1 { fld0: _13.1,fld1: _24,fld2: _3 };
_13.1 = -_5;
_22 = -_21;
_9 = !73876905601023303108341035664830284938_u128;
_6 = _19;
_16.0 = core::ptr::addr_of!(_8);
_22 = _8 as f64;
match _11.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
13917251981558062505 => bb11,
_ => bb6
}
}
bb34 = {
_37 = -_35.1.3;
_35.2 = !_10;
place!(Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(_17, 1), 1)) = _34.fld2.2;
_49 = [_8,_8,_8,_8];
match _40.6 {
5 => bb36,
_ => bb35
}
}
bb35 = {
Return()
}
bb36 = {
_10 = _33 as u8;
_54 = -_21;
_9 = _10 as u128;
_18 = _35.2 as f64;
_34.fld2.0.2 = _35.1.2 & _36.2;
_55 = _27;
(*_20).1 = _35.1.1 - _35.1.1;
place!(Field::<Adt48>(Variant(RET, 1), 6)) = Adt48::Variant0 { fld0: _16.1,fld1: _55,fld2: _32,fld3: _54,fld4: _49,fld5: _33,fld6: _16.0 };
place!(Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(RET, 1), 1)) = core::ptr::addr_of!((*_20));
_40.0 = _25.0 % _24.1;
RET = Adt55::Variant3 { fld0: _1 };
_50 = (_35.2, _25.0, _11.2);
_35.2 = _10;
SetDiscriminant(RET, 2);
_38 = Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).1;
place!(Field::<[char; 1]>(Variant(_17, 1), 5)) = [_38];
Call(_54 = core::intrinsics::fmaf64(_21, _18, _21), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
_56.0 = [_35.1.0,_40.0];
_35 = (Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).2, (*_20), _50.0);
(*_20).0 = !_40.0;
match Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).5 {
0 => bb38,
5 => bb40,
_ => bb39
}
}
bb38 = {
Return()
}
bb39 = {
Return()
}
bb40 = {
_67.2 = _34.fld2.0.2;
_47.fld2.0.1 = [_7,_67.2,_34.fld2.0.2,_67.2,_34.fld2.0.2,_34.fld2.0.2,_40.2];
_3 = _32 as isize;
place!(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3)) = (_16.0, _55, _25.3, _38, _15, _35.1.6);
_18 = _22;
place!(Field::<u16>(Variant(_17, 1), 2)) = 32534_u16 + 13687_u16;
_25.6 = !_35.1.6;
_50.1 = _25.6 as u64;
_41 = _25.4;
place!(Field::<Adt43>(Variant(_17, 1), 4)) = Adt43::Variant1 { fld0: _11.2,fld1: _6,fld2: Field::<(i8, u64)>(Variant(_14, 3), 2) };
_16.2 = _35.2 as u64;
_34.fld2.0 = (Field::<[i64; 7]>(Variant(_14, 3), 1), _47.fld2.0.1, _67.2);
_31 = (_25.4, _25.0);
_66.0 = _25.3;
SetDiscriminant(Field::<Adt43>(Variant(_17, 1), 4), 1);
_1 = !_12;
place!(Field::<[i64; 7]>(Variant(_14, 3), 1)) = [6002719391555668221_i64,750527047083841746_i64,(-2287387141492101364_i64),2543779342800971615_i64,3601194046899609382_i64,(-653713660271397610_i64),(-1065271767443213783_i64)];
_40.5 = -_35.1.5;
place!(Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(_17, 1), 1)) = _34.fld2.2;
place!(Field::<i32>(Variant(place!(Field::<Adt43>(Variant(_17, 1), 4)), 1), 1)) = !_33;
_54 = _18;
place!(Field::<(i8, u64)>(Variant(place!(Field::<Adt43>(Variant(_17, 1), 4)), 1), 2)).1 = !_50.1;
_34.fld2 = (_47.fld2.0, _15, _20);
place!(Field::<([u32; 2], i16)>(Variant(_17, 1), 3)).0 = _13.0;
_16.0 = Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).0;
_66.1.3 = _50.2 as isize;
_51 = [_8,_8,_8,_8];
Goto(bb41)
}
bb41 = {
_71 = (_16.0, _34.fld1, _40.0);
(*_20).0 = _40.0;
place!(Field::<f32>(Variant(place!(Field::<Adt43>(Variant(_17, 1), 4)), 1), 0)) = -_11.2;
_6 = Field::<i32>(Variant(Field::<Adt43>(Variant(_17, 1), 4), 1), 1) ^ _19;
_25.4 = _41 << (*_20).1;
_35.1.2 = Field::<i32>(Variant(Field::<Adt43>(Variant(_17, 1), 4), 1), 1) as u32;
_71.2 = _40.5 as u64;
place!(Field::<[i64; 7]>(Variant(_14, 3), 1)) = [2915674433709142289_i64,(-1141070516147754542_i64),9151643919978320141_i64,(-5263297943861729869_i64),1658980291945473225_i64,5049514982980367192_i64,(-5071930147051855965_i64)];
_74.2 = _67.2 & _36.2;
_47 = _34;
place!(Field::<([i64; 7], [u32; 7], u32)>(Variant(_14, 3), 0)).1 = [_74.2,(*_20).2,_67.2,_74.2,_7,_34.fld2.0.2,_74.2];
_4 = _1 as i8;
_65 = Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).1;
_66.2 = _22 as u8;
_67.0 = _36.0;
_34.fld2 = (_47.fld2.0, _15, Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(_17, 1), 1));
_27 = _55;
_66.1.4 = _35.1.4 * _41;
Call(_70 = fn17(Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).5, _34, Field::<(*const i128, char, isize, char, [u32; 5], usize)>(Variant(_14, 3), 3).2, _34.fld0, _34.fld2, Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(_17, 1), 1)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
place!(Field::<[u64; 2]>(Variant(RET, 2), 0)) = [_31.1,_71.2];
_47.fld2.1 = [_25.2,_74.2,(*_20).2,_74.2,_70.2];
_27 = _65;
place!(Field::<(i8, u64)>(Variant(_14, 3), 2)).0 = _66.1.4;
_25.5 = _35.1.5;
_47.fld2.0.1 = [_74.2,_74.2,_74.2,_34.fld2.0.2,_70.2,_67.2,_70.2];
Goto(bb43)
}
bb43 = {
Call(_85 = dump_var(0_usize, 41_usize, Move(_41), 1_usize, Move(_1), 27_usize, Move(_27), 12_usize, Move(_12)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_85 = dump_var(0_usize, 19_usize, Move(_19), 4_usize, Move(_4), 31_usize, Move(_31), 55_usize, Move(_55)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_85 = dump_var(0_usize, 3_usize, Move(_3), 30_usize, Move(_30), 37_usize, Move(_37), 2_usize, Move(_2)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_85 = dump_var(0_usize, 7_usize, Move(_7), 49_usize, Move(_49), 23_usize, Move(_23), 28_usize, Move(_28)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: f64,mut _3: Adt55,mut _4: isize,mut _5: i16,mut _6: i8,mut _7: i16,mut _8: f32,mut _9: i16,mut _10: bool,mut _11: [u32; 5],mut _12: u128,mut _13: bool,mut _14: u8,mut _15: i16,mut _16: u8) -> i128 {
mir! {
type RET = i128;
let _17: f32;
let _18: &'static (u64, u128, u32, isize, i8, i16, usize);
let _19: [u32; 7];
let _20: [u32; 7];
let _21: isize;
let _22: Adt51;
let _23: Adt50;
let _24: *const i128;
let _25: Adt47;
let _26: i16;
let _27: u8;
let _28: i64;
let _29: u8;
let _30: f32;
let _31: Adt43;
let _32: (u64, u128, u32, isize, i8, i16, usize);
let _33: i8;
let _34: [u32; 7];
let _35: [u32; 2];
let _36: (u64, u128, u32, isize, i8, i16, usize);
let _37: ();
let _38: ();
{
RET = 60885_u16 as i128;
_1 = Field::<bool>(Variant(_3, 3), 0) <= Field::<bool>(Variant(_3, 3), 0);
Goto(bb1)
}
bb1 = {
_1 = _13 >= _13;
_13 = _1 ^ _10;
_7 = '\u{314de}' as i16;
_14 = _16 + _16;
RET = _16 as i128;
match _12 {
0 => bb2,
1 => bb3,
153767096466764168249958808609613916228 => bb5,
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
_5 = _8 as i16;
match _16 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
234 => bb10,
_ => bb9
}
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
_1 = _13 >= _13;
_13 = _1 ^ _10;
_7 = '\u{314de}' as i16;
_14 = _16 + _16;
RET = _16 as i128;
match _12 {
0 => bb2,
1 => bb3,
153767096466764168249958808609613916228 => bb5,
_ => bb4
}
}
bb10 = {
_17 = _12 as f32;
place!(Field::<bool>(Variant(_3, 3), 0)) = _1;
_6 = 89_i8;
SetDiscriminant(_3, 1);
_15 = _5 * _9;
_6 = 98_i8 - (-98_i8);
place!(Field::<[i16; 6]>(Variant(_3, 1), 0)) = [_15,_15,_5,_5,_15,_7];
_1 = !_10;
RET = -52801258263779441839431343835354666963_i128;
_11 = [39321384_u32,85174953_u32,3652726126_u32,18878284_u32,394847960_u32];
_9 = _7 & _7;
Goto(bb11)
}
bb11 = {
place!(Field::<u16>(Variant(_3, 1), 2)) = !16141_u16;
_1 = !_13;
_16 = !_14;
_17 = _16 as f32;
_4 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_13 = _10;
_12 = !201650117832723469451850342490169524012_u128;
_12 = 225492229158899520386767859619014371764_u128 ^ 95129175255615710619550921695450733047_u128;
_12 = !281034529196955465273258115680332609901_u128;
_14 = _2 as u8;
RET = -(-169962256880947314423297255282690549502_i128);
_13 = _16 >= _14;
_14 = _16;
_19 = [3895402407_u32,1080482350_u32,2994350207_u32,3856174797_u32,348233623_u32,3483798195_u32,2528947117_u32];
_20 = _19;
place!(Field::<[i16; 6]>(Variant(_3, 1), 0)) = [_15,_9,_9,_9,_7,_15];
Call(_27 = fn2(_4, _4, _1, _10, _1, _10, _13), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<([u32; 2], i16)>(Variant(_3, 1), 3)).0 = [3034248628_u32,1166628734_u32];
_17 = RET as f32;
Call(_5 = fn3(_11, _19, _1, _4, _14), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_2 = (-1365165957_i32) as f64;
_19 = _20;
_16 = !_14;
place!(Field::<([u32; 2], i16)>(Variant(_3, 1), 3)).1 = _5 & _7;
_28 = !(-1351402318510056798_i64);
_26 = Field::<u16>(Variant(_3, 1), 2) as i16;
_6 = (-10_i8) + (-77_i8);
place!(Field::<[i16; 6]>(Variant(_3, 1), 0)) = [Field::<([u32; 2], i16)>(Variant(_3, 1), 3).1,_26,Field::<([u32; 2], i16)>(Variant(_3, 1), 3).1,_15,_9,_9];
place!(Field::<u16>(Variant(_3, 1), 2)) = (-493040805_i32) as u16;
_1 = !_10;
_29 = _14 >> _9;
_13 = !_1;
place!(Field::<[i16; 6]>(Variant(_3, 1), 0)) = [_15,_9,_9,_15,_5,Field::<([u32; 2], i16)>(Variant(_3, 1), 3).1];
_8 = _17 * _17;
_6 = !24_i8;
_22 = Adt51::Variant0 { fld0: 1966644428_i32,fld1: Field::<u16>(Variant(_3, 1), 2) };
RET = Field::<u16>(Variant(_3, 1), 2) as i128;
_14 = _29 - _29;
_12 = 328434382952758792296733995284700158696_u128;
_24 = core::ptr::addr_of!(RET);
Goto(bb14)
}
bb14 = {
_13 = !_10;
place!(Field::<[char; 1]>(Variant(_3, 1), 5)) = ['\u{b09d}'];
_29 = _15 as u8;
place!(Field::<([u32; 2], i16)>(Variant(_3, 1), 3)).0 = [565337235_u32,1409202330_u32];
_17 = _8 + _8;
_17 = -_8;
_2 = 3183209878_u32 as f64;
_20 = [2859823859_u32,912733530_u32,1543290507_u32,4293814796_u32,2432785363_u32,565698048_u32,2402206519_u32];
_12 = 19856033203536506074537217137754999097_u128;
_21 = _4 * _4;
_32.1 = _13 as u128;
_32.2 = 14761913924452699813_u64 as u32;
_22 = Adt51::Variant0 { fld0: 651559043_i32,fld1: Field::<u16>(Variant(_3, 1), 2) };
_10 = _32.1 > _32.1;
_16 = _27;
_13 = _10 ^ _10;
_9 = (*_24) as i16;
_12 = _32.1 << _32.1;
_14 = !_27;
_12 = 46345820_i32 as u128;
place!(Field::<([u32; 2], i16)>(Variant(_3, 1), 3)).0 = [_32.2,_32.2];
_32.6 = 10939507576743315267_usize * 0_usize;
_32.3 = !_21;
place!(Field::<*const (u64, u128, u32, isize, i8, i16, usize)>(Variant(_3, 1), 1)) = core::ptr::addr_of!(_32);
_36.3 = -_21;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(1_usize, 26_usize, Move(_26), 1_usize, Move(_1), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(1_usize, 14_usize, Move(_14), 4_usize, Move(_4), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(1_usize, 21_usize, Move(_21), 10_usize, Move(_10), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> u8 {
mir! {
type RET = u8;
let _8: f32;
let _9: i16;
let _10: *const i128;
let _11: Adt57;
let _12: (u64, u128, u32, isize, i8, i16, usize);
let _13: [u32; 5];
let _14: bool;
let _15: i64;
let _16: i64;
let _17: &'static (u64, u128, u32, isize, i8, i16, usize);
let _18: Adt50;
let _19: bool;
let _20: u8;
let _21: u16;
let _22: i16;
let _23: u32;
let _24: u8;
let _25: isize;
let _26: bool;
let _27: [i16; 6];
let _28: [i16; 8];
let _29: ();
let _30: ();
{
_6 = !_4;
RET = 194_u8;
_4 = !_3;
_8 = _1 as f32;
RET = !38_u8;
_7 = !_5;
_7 = _5 & _5;
_5 = _4;
RET = 197_u8 - 147_u8;
_7 = !_6;
RET = !43_u8;
RET = 48_u8 ^ 65_u8;
_9 = 9312_i16;
_2 = _1;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
9312 => bb5,
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
_1 = _8 as isize;
_1 = _2 >> _2;
_12.2 = !502717921_u32;
_12.5 = _9;
_12.0 = 7284873174651484721_u64 - 16329206241998469198_u64;
_12.2 = 2802005260_u32 ^ 4028491043_u32;
_12.2 = !2118727875_u32;
Goto(bb6)
}
bb6 = {
_12 = (12292788565578540696_u64, 65418317604124958576765429129304439485_u128, 546033775_u32, _2, (-57_i8), _9, 13924604473185087385_usize);
RET = 250_u8;
_12.6 = (-5486506194324715989_i64) as usize;
RET = _12.0 as u8;
_8 = (-4737329679793695995_i64) as f32;
_12.6 = !5_usize;
_12.2 = _12.4 as u32;
_12.3 = _2;
_8 = _1 as f32;
_3 = _12.1 < _12.1;
_12.4 = -22_i8;
_14 = !_3;
_7 = _14;
RET = 214_u8;
_15 = _12.4 as i64;
_13 = [_12.2,_12.2,_12.2,_12.2,_12.2];
_12 = (18398103704571281200_u64, 136307767872723906643837930770309282993_u128, 1090104834_u32, _2, 95_i8, _9, 6988626577523802064_usize);
_12.2 = 2230258000_u32;
_3 = !_4;
RET = 32866_u16 as u8;
RET = !159_u8;
_12.0 = 16859475796906068518_u64;
_14 = _5 <= _6;
_17 = &_12;
match (*_17).1 {
0 => bb7,
1 => bb8,
136307767872723906643837930770309282993 => bb10,
_ => bb9
}
}
bb7 = {
_1 = _8 as isize;
_1 = _2 >> _2;
_12.2 = !502717921_u32;
_12.5 = _9;
_12.0 = 7284873174651484721_u64 - 16329206241998469198_u64;
_12.2 = 2802005260_u32 ^ 4028491043_u32;
_12.2 = !2118727875_u32;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_7 = !_5;
RET = 39_u8;
_12.5 = _9 & _9;
_17 = &_12;
_1 = !(*_17).3;
_12.2 = 796512940_u32;
_20 = !RET;
RET = _20 << _12.5;
_12.4 = 34_i8 - (-128_i8);
_15 = _14 as i64;
_4 = _7;
_6 = _5 ^ _7;
_19 = !_4;
_7 = _15 != _15;
_19 = !_5;
_12.5 = _12.6 as i16;
_8 = _12.6 as f32;
_17 = &_12;
_19 = !_6;
_19 = !_4;
RET = _20;
_16 = _15 ^ _15;
match (*_17).6 {
0 => bb11,
6988626577523802064 => bb13,
_ => bb12
}
}
bb11 = {
_12 = (12292788565578540696_u64, 65418317604124958576765429129304439485_u128, 546033775_u32, _2, (-57_i8), _9, 13924604473185087385_usize);
RET = 250_u8;
_12.6 = (-5486506194324715989_i64) as usize;
RET = _12.0 as u8;
_8 = (-4737329679793695995_i64) as f32;
_12.6 = !5_usize;
_12.2 = _12.4 as u32;
_12.3 = _2;
_8 = _1 as f32;
_3 = _12.1 < _12.1;
_12.4 = -22_i8;
_14 = !_3;
_7 = _14;
RET = 214_u8;
_15 = _12.4 as i64;
_13 = [_12.2,_12.2,_12.2,_12.2,_12.2];
_12 = (18398103704571281200_u64, 136307767872723906643837930770309282993_u128, 1090104834_u32, _2, 95_i8, _9, 6988626577523802064_usize);
_12.2 = 2230258000_u32;
_3 = !_4;
RET = 32866_u16 as u8;
RET = !159_u8;
_12.0 = 16859475796906068518_u64;
_14 = _5 <= _6;
_17 = &_12;
match (*_17).1 {
0 => bb7,
1 => bb8,
136307767872723906643837930770309282993 => bb10,
_ => bb9
}
}
bb12 = {
_1 = _8 as isize;
_1 = _2 >> _2;
_12.2 = !502717921_u32;
_12.5 = _9;
_12.0 = 7284873174651484721_u64 - 16329206241998469198_u64;
_12.2 = 2802005260_u32 ^ 4028491043_u32;
_12.2 = !2118727875_u32;
Goto(bb6)
}
bb13 = {
_3 = _7;
_6 = !_4;
_19 = _3 | _6;
_7 = _19;
_8 = (-1574668349_i32) as f32;
_24 = _12.5 as u8;
_13 = [(*_17).2,(*_17).2,_12.2,(*_17).2,(*_17).2];
_5 = !_19;
_9 = !(*_17).5;
_21 = 34108_u16;
_14 = _5;
_7 = !_5;
_21 = !10155_u16;
_12.3 = 1006220060_i32 as isize;
_22 = _12.0 as i16;
_25 = _12.4 as isize;
_21 = _12.2 as u16;
_1 = -_2;
_5 = _7 | _6;
_12.4 = 109_i8;
_12.4 = '\u{a9a5e}' as i8;
Goto(bb14)
}
bb14 = {
_15 = -_16;
_20 = _24;
_20 = _24 >> _15;
_24 = _20;
_23 = _19 as u32;
_12.1 = 4401310025991904021821817537894740438_u128 - 102393079438642928618224802490620847198_u128;
_3 = _5;
_16 = -_15;
_16 = _19 as i64;
_25 = _24 as isize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 25_usize, Move(_25), 21_usize, Move(_21), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 20_usize, Move(_20), 15_usize, Move(_15), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(2_usize, 9_usize, Move(_9), 24_usize, Move(_24), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u32; 5],mut _2: [u32; 7],mut _3: bool,mut _4: isize,mut _5: u8) -> i16 {
mir! {
type RET = i16;
let _6: *const i128;
let _7: char;
let _8: (u64, u128, u32, isize, i8, i16, usize);
let _9: [i16; 6];
let _10: Adt45;
let _11: Adt56;
let _12: [u32; 7];
let _13: [i16; 6];
let _14: u8;
let _15: i128;
let _16: u32;
let _17: [bool; 5];
let _18: isize;
let _19: i8;
let _20: char;
let _21: char;
let _22: [i64; 7];
let _23: [u32; 5];
let _24: [i128; 4];
let _25: f64;
let _26: [i16; 8];
let _27: (u64, u128, u32, isize, i8, i16, usize);
let _28: [u32; 2];
let _29: isize;
let _30: ();
let _31: ();
{
_3 = _5 < _5;
RET = 52458437319955591_i64 as i16;
RET = 19697_i16 & 31751_i16;
Goto(bb1)
}
bb1 = {
_5 = RET as u8;
_4 = 9223372036854775807_isize;
RET = -(-32687_i16);
_8.5 = -RET;
_8.3 = _4 >> _5;
Goto(bb2)
}
bb2 = {
_8.6 = !4681618035141309890_usize;
_8 = (6139909949602329147_u64, 240891224022218836457620686297664170198_u128, 1500375344_u32, _4, 47_i8, RET, 5_usize);
_7 = '\u{876b6}';
_8.1 = 99612079232632878878939572907489109912_u128 - 270243866361834098474712653078445188551_u128;
_8.5 = _8.6 as i16;
_8.6 = !4_usize;
_8.3 = !_4;
_8.3 = _4 + _4;
_8 = (6835971359351630390_u64, 179616346911548693270735865568702818013_u128, 3885918187_u32, _4, 82_i8, RET, 5_usize);
_5 = 36235_u16 as u8;
_7 = '\u{4d5a2}';
_4 = _8.1 as isize;
Call(RET = core::intrinsics::transmute(_8.5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8.4 = 74_i8 * 113_i8;
_8.6 = 16169878486236247781_usize & 7632194954459621143_usize;
_8.5 = -RET;
_7 = '\u{8aa32}';
_8.0 = 15312056533919061215_u64;
_8.5 = RET + RET;
_8.3 = _4;
_8.4 = 14_i8 << _8.3;
_8.6 = _8.0 as usize;
_9 = [_8.5,_8.5,RET,_8.5,_8.5,RET];
_8.5 = 10507712260031837745161493462978523259_i128 as i16;
_8.5 = RET * RET;
_8.0 = _4 as u64;
_8.1 = !217014938415681385509270538427274669896_u128;
_1 = [_8.2,_8.2,_8.2,_8.2,_8.2];
_8.6 = 10688605622209463151_usize << _8.3;
_5 = 25_u8 - 129_u8;
_7 = '\u{b2eb2}';
RET = _8.5 - _8.5;
_12 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_8.0 = 9290530746314678417_u64;
_8.4 = (-27_i8);
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3885918187 => bb10,
_ => bb9
}
}
bb4 = {
_8.6 = !4681618035141309890_usize;
_8 = (6139909949602329147_u64, 240891224022218836457620686297664170198_u128, 1500375344_u32, _4, 47_i8, RET, 5_usize);
_7 = '\u{876b6}';
_8.1 = 99612079232632878878939572907489109912_u128 - 270243866361834098474712653078445188551_u128;
_8.5 = _8.6 as i16;
_8.6 = !4_usize;
_8.3 = !_4;
_8.3 = _4 + _4;
_8 = (6835971359351630390_u64, 179616346911548693270735865568702818013_u128, 3885918187_u32, _4, 82_i8, RET, 5_usize);
_5 = 36235_u16 as u8;
_7 = '\u{4d5a2}';
_4 = _8.1 as isize;
Call(RET = core::intrinsics::transmute(_8.5), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = RET as u8;
_4 = 9223372036854775807_isize;
RET = -(-32687_i16);
_8.5 = -RET;
_8.3 = _4 >> _5;
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
_13 = [RET,RET,_8.5,_8.5,RET,RET];
_10 = Adt45::Variant2 { fld0: _3,fld1: _8.5 };
_5 = 241_u8;
_15 = (-4645520062720542268199900833306004522_i128);
Goto(bb11)
}
bb11 = {
_8.5 = RET & RET;
RET = (-2449949634443788309_i64) as i16;
_8 = (9596951749274074321_u64, 132256855034935197339960906858770712183_u128, 179990546_u32, _4, (-12_i8), Field::<i16>(Variant(_10, 2), 1), 12034265260682952584_usize);
_13 = [_8.5,_8.5,Field::<i16>(Variant(_10, 2), 1),_8.5,_8.5,Field::<i16>(Variant(_10, 2), 1)];
_4 = !_8.3;
_6 = core::ptr::addr_of!(_15);
(*_6) = (-65946973681201032101772126402667482858_i128) * 34593964488150612107277102233535337464_i128;
RET = !_8.5;
_1 = [_8.2,_8.2,_8.2,_8.2,_8.2];
_16 = !_8.2;
_2 = [_16,_8.2,_16,_16,_8.2,_8.2,_8.2];
(*_6) = (-169432075830601301434437376982701779445_i128) & (-158166378296109722989670760271013058373_i128);
Call(_14 = fn4(_8.1, _8.1, _12, _8.2, _8.1, _8.0, _8.0, _8, Field::<i16>(Variant(_10, 2), 1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8.0 = 15893943552631800306_u64;
_19 = _8.4;
_16 = _5 as u32;
_14 = 1650797615815526722_i64 as u8;
_8.6 = !7_usize;
_4 = -_8.3;
RET = _8.5 + _8.5;
_8.5 = _7 as i16;
_13 = [RET,_8.5,RET,RET,RET,RET];
_12 = _2;
(*_6) = -103456422440655302421813005862693635104_i128;
RET = Field::<i16>(Variant(_10, 2), 1);
SetDiscriminant(_10, 1);
_2 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_21 = _7;
_8.3 = _4;
match _19 {
340282366920938463463374607431768211444 => bb13,
_ => bb3
}
}
bb13 = {
_19 = _8.4 - _8.4;
place!(Field::<(([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize))>(Variant(_10, 1), 1)).0.1 = _12;
_16 = !_8.2;
place!(Field::<[u32; 2]>(Variant(_10, 1), 5)) = [_8.2,_8.2];
place!(Field::<[i64; 7]>(Variant(_10, 1), 0)) = [(-455268371291823178_i64),(-3522370985080968846_i64),1716589410857570796_i64,(-6768518463554682927_i64),3629440453020650997_i64,330989677370626725_i64,6312176165912629177_i64];
place!(Field::<[bool; 5]>(Variant(_10, 1), 3)) = [_3,_3,_3,_3,_3];
_25 = _8.1 as f64;
_15 = (-36946559832789323070220594301911552637_i128) + (-116183437240796214373464812676379211652_i128);
_23 = [_16,_8.2,_16,_16,_8.2];
_15 = (-20221834061353146613318457451260158639_i128) ^ (-25290506465444938195356688634139479276_i128);
_23 = _1;
_15 = 99342488835178945607991193844932479318_i128 | (-23691957457966569461396592398299531494_i128);
place!(Field::<(([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize))>(Variant(_10, 1), 1)).1 = [_16,_8.2,_16,_16,_16];
_8.6 = !3612790361963452263_usize;
_26 = [RET,RET,_8.5,_8.5,RET,RET,RET,RET];
_19 = !_8.4;
_7 = _21;
place!(Field::<[u32; 7]>(Variant(_10, 1), 4)) = [_16,_8.2,_8.2,_8.2,_16,_8.2,_16];
_27.3 = _8.3 | _8.3;
place!(Field::<(([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize))>(Variant(_10, 1), 1)).2 = core::ptr::addr_of!(_27);
_27.4 = !_19;
_4 = _8.3 >> _8.2;
place!(Field::<(([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize))>(Variant(_10, 1), 1)).0.2 = !_8.2;
place!(Field::<[i64; 7]>(Variant(_10, 1), 0)) = [(-6796575474972852715_i64),2084695600151078054_i64,(-1610406342512735377_i64),(-9168417161892505509_i64),(-8971718394573531050_i64),(-7734863154222422549_i64),1330796965577556036_i64];
place!(Field::<[u32; 2]>(Variant(_10, 1), 5)) = [_16,Field::<(([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize))>(Variant(_10, 1), 1).0.2];
_17 = Field::<[bool; 5]>(Variant(_10, 1), 3);
match _8.4 {
0 => bb1,
1 => bb7,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb14,
6 => bb15,
340282366920938463463374607431768211444 => bb17,
_ => bb16
}
}
bb14 = {
_8.4 = 74_i8 * 113_i8;
_8.6 = 16169878486236247781_usize & 7632194954459621143_usize;
_8.5 = -RET;
_7 = '\u{8aa32}';
_8.0 = 15312056533919061215_u64;
_8.5 = RET + RET;
_8.3 = _4;
_8.4 = 14_i8 << _8.3;
_8.6 = _8.0 as usize;
_9 = [_8.5,_8.5,RET,_8.5,_8.5,RET];
_8.5 = 10507712260031837745161493462978523259_i128 as i16;
_8.5 = RET * RET;
_8.0 = _4 as u64;
_8.1 = !217014938415681385509270538427274669896_u128;
_1 = [_8.2,_8.2,_8.2,_8.2,_8.2];
_8.6 = 10688605622209463151_usize << _8.3;
_5 = 25_u8 - 129_u8;
_7 = '\u{b2eb2}';
RET = _8.5 - _8.5;
_12 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_8.0 = 9290530746314678417_u64;
_8.4 = (-27_i8);
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3885918187 => bb10,
_ => bb9
}
}
bb15 = {
_8.5 = RET & RET;
RET = (-2449949634443788309_i64) as i16;
_8 = (9596951749274074321_u64, 132256855034935197339960906858770712183_u128, 179990546_u32, _4, (-12_i8), Field::<i16>(Variant(_10, 2), 1), 12034265260682952584_usize);
_13 = [_8.5,_8.5,Field::<i16>(Variant(_10, 2), 1),_8.5,_8.5,Field::<i16>(Variant(_10, 2), 1)];
_4 = !_8.3;
_6 = core::ptr::addr_of!(_15);
(*_6) = (-65946973681201032101772126402667482858_i128) * 34593964488150612107277102233535337464_i128;
RET = !_8.5;
_1 = [_8.2,_8.2,_8.2,_8.2,_8.2];
_16 = !_8.2;
_2 = [_16,_8.2,_16,_16,_8.2,_8.2,_8.2];
(*_6) = (-169432075830601301434437376982701779445_i128) & (-158166378296109722989670760271013058373_i128);
Call(_14 = fn4(_8.1, _8.1, _12, _8.2, _8.1, _8.0, _8.0, _8, Field::<i16>(Variant(_10, 2), 1)), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_5 = RET as u8;
_4 = 9223372036854775807_isize;
RET = -(-32687_i16);
_8.5 = -RET;
_8.3 = _4 >> _5;
Goto(bb2)
}
bb17 = {
_8.4 = _5 as i8;
_27.0 = _8.0;
_28 = [_16,_16];
_27.0 = _8.0;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(3_usize, 9_usize, Move(_9), 19_usize, Move(_19), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(3_usize, 26_usize, Move(_26), 12_usize, Move(_12), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(3_usize, 21_usize, Move(_21), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u128,mut _2: u128,mut _3: [u32; 7],mut _4: u32,mut _5: u128,mut _6: u64,mut _7: u64,mut _8: (u64, u128, u32, isize, i8, i16, usize),mut _9: i16) -> u8 {
mir! {
type RET = u8;
let _10: i8;
let _11: u16;
let _12: [char; 1];
let _13: isize;
let _14: f64;
let _15: [i64; 7];
let _16: Adt58;
let _17: char;
let _18: char;
let _19: u64;
let _20: u16;
let _21: Adt58;
let _22: u8;
let _23: Adt58;
let _24: i64;
let _25: [u32; 2];
let _26: Adt42;
let _27: *mut &'static (u64, u128, u32, isize, i8, i16, usize);
let _28: Adt48;
let _29: [i128; 4];
let _30: [i16; 8];
let _31: ();
let _32: ();
{
_8 = (_7, _2, _4, 114_isize, (-15_i8), _9, 6668525399695398175_usize);
_6 = _8.0;
_1 = _5;
_8.2 = !_4;
RET = 143_u8;
_8.2 = !_4;
_1 = _8.1;
RET = !204_u8;
_8.3 = 9223372036854775807_isize >> _2;
_12 = ['\u{dbaf2}'];
_11 = 23988_u16;
Call(_10 = fn5(_8.4, _8, _8.4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = ['\u{8b1ad}'];
RET = '\u{104c32}' as u8;
_1 = _5;
_8.0 = _7 - _6;
RET = 27_u8;
_3 = [_4,_4,_4,_4,_4,_4,_8.2];
_8.4 = _8.2 as i8;
_8 = (_7, _5, _4, 9223372036854775807_isize, _10, _9, 4795084841239455347_usize);
_11 = 19779_u16;
_18 = '\u{50bb7}';
_9 = 5636614397326545703_i64 as i16;
_8.5 = !_9;
_3 = [_4,_4,_4,_8.2,_8.2,_8.2,_8.2];
_11 = !14388_u16;
_18 = '\u{d66b3}';
_8.5 = !_9;
_1 = !_2;
_18 = '\u{553c6}';
_7 = 1143394159_i32 as u64;
_7 = _8.0 % _6;
_8.0 = !_6;
_20 = _11;
_20 = !_11;
match _5 {
0 => bb2,
1 => bb3,
132256855034935197339960906858770712183 => bb5,
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
Call(_8.3 = core::intrinsics::transmute(_6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = _8.4 as u128;
_12 = [_18];
_14 = _8.2 as f64;
_19 = !_8.0;
_9 = RET as i16;
_15 = [(-1735491021448275597_i64),(-7623202482705632109_i64),(-7263301993552436488_i64),(-334612081512077396_i64),(-729480065818552576_i64),(-4315550712609095977_i64),7971406890915063768_i64];
_17 = _18;
_7 = _6 * _6;
_9 = _8.5 ^ _8.5;
_24 = 4719687277394288726_i64;
_24 = -(-6665270722519833985_i64);
_5 = _8.1;
_10 = _8.4;
RET = !204_u8;
_18 = _17;
_25 = [_8.2,_8.2];
_5 = !_1;
_8.5 = !_9;
_18 = _17;
_19 = _7;
_3 = [_8.2,_8.2,_4,_4,_8.2,_4,_8.2];
_8.5 = -_9;
_8.6 = 3_usize ^ 13211785610151576629_usize;
Goto(bb7)
}
bb7 = {
_7 = _8.0;
RET = _17 as u8;
_8 = (_19, _1, _4, 65_isize, _10, _9, 11166388390910672859_usize);
_10 = _8.4;
match _6 {
0 => bb1,
1 => bb6,
2 => bb5,
9596951749274074321 => bb9,
_ => bb8
}
}
bb8 = {
_1 = _8.4 as u128;
_12 = [_18];
_14 = _8.2 as f64;
_19 = !_8.0;
_9 = RET as i16;
_15 = [(-1735491021448275597_i64),(-7623202482705632109_i64),(-7263301993552436488_i64),(-334612081512077396_i64),(-729480065818552576_i64),(-4315550712609095977_i64),7971406890915063768_i64];
_17 = _18;
_7 = _6 * _6;
_9 = _8.5 ^ _8.5;
_24 = 4719687277394288726_i64;
_24 = -(-6665270722519833985_i64);
_5 = _8.1;
_10 = _8.4;
RET = !204_u8;
_18 = _17;
_25 = [_8.2,_8.2];
_5 = !_1;
_8.5 = !_9;
_18 = _17;
_19 = _7;
_3 = [_8.2,_8.2,_4,_4,_8.2,_4,_8.2];
_8.5 = -_9;
_8.6 = 3_usize ^ 13211785610151576629_usize;
Goto(bb7)
}
bb9 = {
_11 = _4 as u16;
match _2 {
0 => bb5,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
132256855034935197339960906858770712183 => bb16,
_ => bb15
}
}
bb10 = {
_1 = _8.4 as u128;
_12 = [_18];
_14 = _8.2 as f64;
_19 = !_8.0;
_9 = RET as i16;
_15 = [(-1735491021448275597_i64),(-7623202482705632109_i64),(-7263301993552436488_i64),(-334612081512077396_i64),(-729480065818552576_i64),(-4315550712609095977_i64),7971406890915063768_i64];
_17 = _18;
_7 = _6 * _6;
_9 = _8.5 ^ _8.5;
_24 = 4719687277394288726_i64;
_24 = -(-6665270722519833985_i64);
_5 = _8.1;
_10 = _8.4;
RET = !204_u8;
_18 = _17;
_25 = [_8.2,_8.2];
_5 = !_1;
_8.5 = !_9;
_18 = _17;
_19 = _7;
_3 = [_8.2,_8.2,_4,_4,_8.2,_4,_8.2];
_8.5 = -_9;
_8.6 = 3_usize ^ 13211785610151576629_usize;
Goto(bb7)
}
bb11 = {
_12 = ['\u{8b1ad}'];
RET = '\u{104c32}' as u8;
_1 = _5;
_8.0 = _7 - _6;
RET = 27_u8;
_3 = [_4,_4,_4,_4,_4,_4,_8.2];
_8.4 = _8.2 as i8;
_8 = (_7, _5, _4, 9223372036854775807_isize, _10, _9, 4795084841239455347_usize);
_11 = 19779_u16;
_18 = '\u{50bb7}';
_9 = 5636614397326545703_i64 as i16;
_8.5 = !_9;
_3 = [_4,_4,_4,_8.2,_8.2,_8.2,_8.2];
_11 = !14388_u16;
_18 = '\u{d66b3}';
_8.5 = !_9;
_1 = !_2;
_18 = '\u{553c6}';
_7 = 1143394159_i32 as u64;
_7 = _8.0 % _6;
_8.0 = !_6;
_20 = _11;
_20 = !_11;
match _5 {
0 => bb2,
1 => bb3,
132256855034935197339960906858770712183 => bb5,
_ => bb4
}
}
bb12 = {
_1 = _8.4 as u128;
_12 = [_18];
_14 = _8.2 as f64;
_19 = !_8.0;
_9 = RET as i16;
_15 = [(-1735491021448275597_i64),(-7623202482705632109_i64),(-7263301993552436488_i64),(-334612081512077396_i64),(-729480065818552576_i64),(-4315550712609095977_i64),7971406890915063768_i64];
_17 = _18;
_7 = _6 * _6;
_9 = _8.5 ^ _8.5;
_24 = 4719687277394288726_i64;
_24 = -(-6665270722519833985_i64);
_5 = _8.1;
_10 = _8.4;
RET = !204_u8;
_18 = _17;
_25 = [_8.2,_8.2];
_5 = !_1;
_8.5 = !_9;
_18 = _17;
_19 = _7;
_3 = [_8.2,_8.2,_4,_4,_8.2,_4,_8.2];
_8.5 = -_9;
_8.6 = 3_usize ^ 13211785610151576629_usize;
Goto(bb7)
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
RET = !0_u8;
_9 = _8.5 >> _8.0;
_22 = RET;
_8.3 = false as isize;
_8.2 = !_4;
_20 = _8.2 as u16;
_20 = _11 | _11;
_13 = _10 as isize;
_8.0 = !_19;
_1 = !_8.1;
_8.5 = _9;
_18 = _17;
_6 = _19 << _8.1;
_2 = _9 as u128;
_4 = !_8.2;
_3 = [_4,_4,_8.2,_8.2,_4,_8.2,_4];
_5 = _18 as u128;
_9 = _8.5;
_5 = _8.0 as u128;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(4_usize, 8_usize, Move(_8), 13_usize, Move(_13), 19_usize, Move(_19), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(4_usize, 6_usize, Move(_6), 1_usize, Move(_1), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(4_usize, 3_usize, Move(_3), 12_usize, Move(_12), 32_usize, _32, 32_usize, _32), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i8,mut _2: (u64, u128, u32, isize, i8, i16, usize),mut _3: i8) -> i8 {
mir! {
type RET = i8;
let _4: [char; 1];
let _5: bool;
let _6: ();
let _7: ();
{
_2.5 = 5551_i16 * 11764_i16;
_2.3 = (-946813751596099751_i64) as isize;
_2.4 = _3 + _1;
RET = _3 * _1;
RET = _2.4 - _3;
_3 = _2.3 as i8;
RET = _1;
_1 = !_2.4;
RET = _2.1 as i8;
_2.2 = !3336693778_u32;
_3 = _1 ^ _1;
_1 = _2.4;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(5_usize, 1_usize, Move(_1), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i64; 7],mut _2: *const (u64, u128, u32, isize, i8, i16, usize),mut _3: *const (u64, u128, u32, isize, i8, i16, usize),mut _4: (*const i128, char, isize, char, [u32; 5], usize),mut _5: *const (u64, u128, u32, isize, i8, i16, usize),mut _6: *const (u64, u128, u32, isize, i8, i16, usize),mut _7: isize,mut _8: isize,mut _9: *const (u64, u128, u32, isize, i8, i16, usize),mut _10: isize,mut _11: isize,mut _12: usize) -> i16 {
mir! {
type RET = i16;
let _13: *const (u64, u128, u32, isize, i8, i16, usize);
let _14: Adt54;
let _15: u64;
let _16: [i16; 6];
let _17: [i64; 7];
let _18: i64;
let _19: Adt53;
let _20: Adt46;
let _21: i16;
let _22: [u32; 7];
let _23: *mut [i128; 4];
let _24: bool;
let _25: char;
let _26: Adt51;
let _27: Adt54;
let _28: isize;
let _29: i32;
let _30: (u64, u128, u32, isize, i8, i16, usize);
let _31: char;
let _32: [bool; 5];
let _33: f64;
let _34: char;
let _35: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _36: ([u32; 2], i16);
let _37: char;
let _38: f64;
let _39: char;
let _40: ();
let _41: ();
{
(*_9).2 = (*_3).2;
(*_6).0 = (*_3).0;
(*_5).4 = -(*_9).4;
(*_3) = ((*_6).0, (*_2).1, (*_6).2, (*_2).3, (*_6).4, (*_2).5, _12);
(*_2).4 = _4.5 as i8;
_13 = _3;
(*_2).1 = (*_5).1 * (*_13).1;
(*_5).2 = false as u32;
(*_13) = ((*_2).0, (*_6).1, (*_6).2, _8, (*_6).4, (*_9).5, (*_2).6);
(*_6).0 = (*_5).0;
_4.5 = _12;
(*_5).1 = (*_3).2 as u128;
_2 = _3;
_15 = !(*_5).0;
(*_5).6 = !_4.5;
(*_2).5 = 142_u8 as i16;
(*_6).4 = (-1605100122_i32) as i8;
(*_9).1 = (*_2).6 as u128;
(*_9) = ((*_2).0, (*_3).1, (*_5).2, (*_13).3, (*_2).4, (*_3).5, (*_2).6);
_16 = [(*_2).5,(*_9).5,(*_13).5,(*_3).5,(*_2).5,(*_13).5];
(*_9).0 = _15;
(*_5).5 = _8 as i16;
Goto(bb1)
}
bb1 = {
(*_6).6 = _4.1 as usize;
_6 = core::ptr::addr_of!((*_2));
(*_2).6 = _12 + _4.5;
Call((*_13).1 = core::intrinsics::transmute((*_9).1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_9).5 = (*_3).5 << (*_3).6;
(*_2) = ((*_9).0, (*_9).1, (*_9).2, _4.2, (*_9).4, (*_9).5, _12);
(*_2).2 = (*_9).2 + (*_9).2;
(*_5).3 = _8 + _11;
_12 = _4.3 as usize;
(*_6).6 = _4.5;
(*_3).5 = -(*_9).5;
_19.fld1 = _16;
(*_5).1 = !(*_9).1;
_20.fld1 = 138_u8 as u128;
_20.fld0 = (-165495855731549532570211500005149262134_i128);
_19.fld2.2 = _3;
_19.fld2.1 = _4.4;
(*_5).1 = (*_9).1 ^ (*_9).1;
(*_6).1 = (*_9).1 ^ (*_9).1;
(*_6).2 = (*_9).2;
_1 = [532842495157544907_i64,(-6642383754499914459_i64),(-8153334847254089827_i64),(-5221367662957018982_i64),4507006106898365287_i64,(-2344076352870280333_i64),(-6389192847664124021_i64)];
_6 = _2;
(*_9).0 = (*_6).1 as u64;
(*_2).2 = !(*_9).2;
(*_9) = (*_5);
(*_3).5 = false as i16;
Goto(bb3)
}
bb3 = {
(*_6).3 = (*_5).4 as isize;
(*_3).0 = !(*_9).0;
(*_6).0 = _15 | _15;
(*_3).6 = _4.5;
(*_2).3 = true as isize;
RET = _20.fld0 as i16;
_3 = core::ptr::addr_of!((*_2));
_22 = [(*_13).2,(*_3).2,(*_3).2,(*_5).2,(*_2).2,(*_6).2,(*_2).2];
(*_2).3 = 1819740187_i32 as isize;
(*_3).6 = !_4.5;
(*_9) = ((*_6).0, (*_6).1, (*_2).2, _11, (*_5).4, (*_6).5, _4.5);
(*_2).0 = (*_5).1 as u64;
(*_5).3 = _11;
(*_2).1 = (*_9).1;
_15 = (*_5).0 & (*_2).0;
Call((*_13).5 = core::intrinsics::bswap((*_9).5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = [(-6997586944988557758_i64),8915321048830969120_i64,(-3626804005498387275_i64),8862534176597800009_i64,(-8166934852451528896_i64),1140300977276834634_i64,(-3512262769261687493_i64)];
_4.3 = _4.1;
(*_13) = (*_9);
(*_3).6 = (*_9).6 ^ (*_9).6;
(*_9).0 = _4.1 as u64;
(*_3).3 = _8 ^ (*_9).3;
(*_5).4 = (*_9).4;
_4.0 = core::ptr::addr_of!(_20.fld0);
_3 = core::ptr::addr_of!((*_6));
_19.fld2.0 = (_1, _22, (*_6).2);
(*_5).6 = _4.5;
_18 = -(-9136358382106497118_i64);
(*_5).0 = _15 + (*_9).0;
(*_2).5 = !(*_9).5;
(*_9).2 = (*_13).2;
_20.fld2 = !14099_u16;
(*_6).5 = !RET;
(*_5).0 = !(*_9).0;
(*_9).2 = (*_2).2 + (*_6).2;
_13 = _2;
(*_9).1 = (*_2).1;
_19.fld0 = core::ptr::addr_of!((*_6));
(*_5).2 = (*_9).2;
Call((*_13) = fn7((*_9), _19.fld2.2, _13, _11, _13, _4.5, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_9).2 = _18 as u32;
Goto(bb6)
}
bb6 = {
_30.3 = (*_9).3;
(*_13).0 = _15 - _15;
(*_9).4 = (*_13).4 << (*_3).3;
(*_2).3 = _30.3;
_32 = [false,false,true,false,false];
(*_6) = (*_9);
(*_2).3 = !_7;
(*_5).0 = _15 & _15;
Call((*_2).5 = fn8((*_9), _19.fld0, (*_5).0, (*_2).1, (*_5).4, _2, (*_5).0, (*_5).2, (*_5).6, (*_9).4, (*_6).0, _5, (*_13).4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_2).4 = -(*_9).4;
(*_5).2 = _19.fld2.0.2;
(*_9) = (*_5);
(*_13).1 = !(*_9).1;
(*_13).4 = _18 as i8;
(*_6).4 = (*_9).4 | (*_9).4;
(*_13) = ((*_9).0, (*_9).1, _19.fld2.0.2, _11, (*_9).4, RET, (*_9).6);
(*_5).4 = (*_9).4;
(*_13).1 = (*_9).1 * (*_9).1;
(*_3).0 = !(*_9).0;
_33 = (*_5).1 as f64;
_31 = _4.3;
(*_5).0 = _15;
(*_2).0 = (*_9).0;
_30 = ((*_9).0, (*_13).1, (*_6).2, (*_5).3, (*_6).4, (*_13).5, (*_5).6);
(*_2).0 = !(*_9).0;
(*_6).0 = !_30.0;
(*_3).5 = _30.0 as i16;
match _4.5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
_ => bb6
}
}
bb8 = {
_31 = _4.1;
_4.2 = false as isize;
(*_9).1 = (*_6).1 << (*_3).3;
(*_13) = (*_9);
match _4.5 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb11,
_ => bb10
}
}
bb9 = {
(*_6).6 = _4.1 as usize;
_6 = core::ptr::addr_of!((*_2));
(*_2).6 = _12 + _4.5;
Call((*_13).1 = core::intrinsics::transmute((*_9).1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_30.3 = (*_9).3;
(*_13).0 = _15 - _15;
(*_9).4 = (*_13).4 << (*_3).3;
(*_2).3 = _30.3;
_32 = [false,false,true,false,false];
(*_6) = (*_9);
(*_2).3 = !_7;
(*_5).0 = _15 & _15;
Call((*_2).5 = fn8((*_9), _19.fld0, (*_5).0, (*_2).1, (*_5).4, _2, (*_5).0, (*_5).2, (*_5).6, (*_9).4, (*_6).0, _5, (*_13).4), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
(*_6) = (*_9);
(*_6).2 = !(*_9).2;
(*_9).0 = !(*_3).0;
_1 = [_18,_18,_18,_18,_18,_18,_18];
(*_5) = ((*_9).0, (*_9).1, (*_9).2, _8, (*_9).4, (*_9).5, (*_9).6);
_28 = _20.fld2 as isize;
(*_6).0 = _15 >> (*_5).4;
_20.fld0 = !70483187238872727260008745655902337948_i128;
_35.1.2 = 67_u8 as u32;
(*_13).2 = !_19.fld2.0.2;
_17 = [_18,_18,_18,_18,_18,_18,_18];
(*_5) = (*_9);
(*_3).1 = (*_9).6 as u128;
(*_2) = ((*_9).0, (*_9).1, _30.2, (*_9).3, _30.4, _30.5, (*_9).6);
RET = (*_9).1 as i16;
(*_13).6 = (-1306456927_i32) as usize;
_19.fld2.0.2 = (*_6).2;
(*_6) = (_30.0, (*_9).1, _19.fld2.0.2, (*_9).3, (*_9).4, RET, (*_9).6);
(*_9).2 = (*_6).2 & (*_5).2;
_30.6 = (*_3).6;
(*_13).4 = (*_6).1 as i8;
(*_2).1 = !(*_9).1;
(*_3).4 = _33 as i8;
(*_6) = _30;
(*_6).4 = (*_9).4 + _30.4;
_35 = ((*_9).3, (*_2), 250_u8);
(*_6).0 = _30.0;
Goto(bb12)
}
bb12 = {
Call(_40 = dump_var(6_usize, 8_usize, Move(_8), 31_usize, Move(_31), 16_usize, Move(_16), 11_usize, Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_40 = dump_var(6_usize, 28_usize, Move(_28), 1_usize, Move(_1), 30_usize, Move(_30), 12_usize, Move(_12)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (u64, u128, u32, isize, i8, i16, usize),mut _2: *const (u64, u128, u32, isize, i8, i16, usize),mut _3: *const (u64, u128, u32, isize, i8, i16, usize),mut _4: isize,mut _5: *const (u64, u128, u32, isize, i8, i16, usize),mut _6: usize,mut _7: *const (u64, u128, u32, isize, i8, i16, usize)) -> (u64, u128, u32, isize, i8, i16, usize) {
mir! {
type RET = (u64, u128, u32, isize, i8, i16, usize);
let _8: ();
let _9: ();
{
RET.0 = _6 as u64;
_5 = _3;
RET.0 = _1.1 as u64;
RET.3 = _4;
RET = _1;
RET.0 = 2877435817780199133_i64 as u64;
RET.6 = !_6;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(7_usize, 6_usize, Move(_6), 9_usize, _9, 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: (u64, u128, u32, isize, i8, i16, usize),mut _2: *const (u64, u128, u32, isize, i8, i16, usize),mut _3: u64,mut _4: u128,mut _5: i8,mut _6: *const (u64, u128, u32, isize, i8, i16, usize),mut _7: u64,mut _8: u32,mut _9: usize,mut _10: i8,mut _11: u64,mut _12: *const (u64, u128, u32, isize, i8, i16, usize),mut _13: i8) -> i16 {
mir! {
type RET = i16;
let _14: (u8, u64, f32);
let _15: i128;
let _16: [char; 1];
let _17: u16;
let _18: u8;
let _19: [u32; 5];
let _20: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize));
let _21: isize;
let _22: isize;
let _23: Adt49;
let _24: (*const i128, [i16; 6], u64);
let _25: Adt53;
let _26: [i16; 6];
let _27: [i16; 8];
let _28: i128;
let _29: (*const i128, [i16; 6], u64);
let _30: [u32; 2];
let _31: f64;
let _32: Adt48;
let _33: [bool; 5];
let _34: ([i64; 7], [u32; 7], u32);
let _35: u16;
let _36: [u64; 2];
let _37: Adt48;
let _38: isize;
let _39: ();
let _40: ();
{
_14.2 = (*_12).0 as f32;
_2 = _12;
(*_2).6 = (*_2).0 as usize;
_14.1 = !_3;
(*_2).1 = _4 ^ _4;
(*_6).2 = (*_12).4 as u32;
Goto(bb1)
}
bb1 = {
(*_12).2 = _8;
_9 = '\u{39a05}' as usize;
_12 = core::ptr::addr_of!(_1);
_14.0 = !145_u8;
(*_12).5 = !32295_i16;
(*_2).2 = 503_u16 as u32;
(*_12).4 = _13 >> _4;
_14.2 = (*_12).6 as f32;
_4 = !(*_6).1;
_4 = (*_2).1 * (*_6).1;
_1.6 = (*_6).6;
_13 = (*_6).4;
_14.0 = !140_u8;
_8 = !(*_2).2;
(*_2).3 = 164302153356941141459615524222413317934_i128 as isize;
_9 = (*_12).6 ^ (*_2).6;
Goto(bb2)
}
bb2 = {
RET = !(*_12).5;
_1.2 = !(*_2).2;
(*_6).2 = !(*_12).2;
_7 = 30895_u16 as u64;
_20.0.2 = !_8;
(*_12).6 = !_9;
(*_12).6 = (*_2).6;
(*_12) = ((*_2).0, _4, (*_6).2, (*_2).3, _13, RET, (*_2).6);
_20.2 = _6;
(*_12).4 = -(*_2).4;
_1 = ((*_6).0, (*_2).1, (*_6).2, (*_6).3, _13, RET, _9);
(*_2).0 = _5 as u64;
_1.0 = !(*_2).0;
(*_2).0 = 58198_u16 as u64;
_20.2 = core::ptr::addr_of!((*_12));
Call(_20.1 = fn9((*_12), (*_6).4, _2, _12, (*_12).6, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = _20.1;
_11 = (*_2).0;
(*_12).2 = _20.0.2;
(*_12).2 = _8;
_1 = (_3, (*_2).1, (*_2).2, (*_6).3, _13, RET, (*_6).6);
_1.6 = !(*_6).6;
(*_12) = ((*_2).0, (*_2).1, (*_6).2, (*_6).3, (*_2).4, RET, (*_6).6);
_17 = 22187_u16 * 3919_u16;
_1.2 = (*_2).2 * (*_6).2;
(*_12).1 = (*_12).5 as u128;
_20.0.0 = [(-8342803383829930528_i64),418264349714660203_i64,(-6178707102963383072_i64),1085616776319872484_i64,(-7123037882693977247_i64),(-6140268865711672060_i64),3919268585786813618_i64];
(*_12).4 = (*_6).4;
(*_6).2 = (-882296818_i32) as u32;
Call((*_2).4 = core::intrinsics::bswap((*_12).4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_2).4 = _1.4 ^ (*_12).4;
(*_2).2 = (*_2).3 as u32;
_14.0 = !172_u8;
_24.0 = core::ptr::addr_of!(_15);
Goto(bb5)
}
bb5 = {
_22 = !(*_12).3;
_20.2 = _2;
(*_12).4 = (*_2).6 as i8;
_25.fld0 = _2;
_15 = !(-776546476263465024841903928134785908_i128);
_25.fld2.1 = _20.1;
(*_6).1 = _4;
_12 = _25.fld0;
Goto(bb6)
}
bb6 = {
(*_2).6 = _9 & _9;
_20.0.1 = [(*_6).2,(*_12).2,(*_2).2,(*_2).2,(*_6).2,(*_2).2,(*_2).2];
_21 = (*_12).3;
_18 = _14.0 ^ _14.0;
(*_2).2 = _8;
Goto(bb7)
}
bb7 = {
(*_6).3 = (-8225110576517483163_i64) as isize;
_9 = !(*_2).6;
_5 = !(*_12).4;
_2 = core::ptr::addr_of!(_1);
(*_2).1 = (*_6).1 * (*_12).1;
_1.3 = _21 + _21;
_20.0.1 = [(*_6).2,_20.0.2,(*_2).2,_1.2,_20.0.2,_1.2,(*_6).2];
_1.1 = (*_6).1 ^ (*_12).1;
_24.1 = [(*_2).5,(*_2).5,(*_2).5,(*_2).5,(*_2).5,(*_2).5];
(*_12).1 = (*_2).1;
(*_2) = ((*_12).0, (*_6).1, (*_12).2, _21, (*_6).4, RET, (*_6).6);
(*_2).1 = '\u{10a784}' as u128;
_1.1 = (*_2).5 as u128;
(*_2).4 = !(*_12).4;
_11 = _14.2 as u64;
(*_6).3 = (*_12).1 as isize;
_27 = [(*_2).5,(*_2).5,RET,(*_2).5,(*_2).5,(*_2).5,(*_2).5,_1.5];
_25.fld2.0.2 = _1.5 as u32;
_21 = (*_2).3;
(*_6).0 = _14.1;
Goto(bb8)
}
bb8 = {
(*_6).3 = _1.3 * (*_2).3;
(*_6).4 = !(*_2).4;
(*_2).1 = (*_6).1 + (*_12).1;
_2 = _20.2;
(*_12).0 = _14.1;
(*_12).3 = _1.3;
_25 = Adt53 { fld0: _20.2,fld1: _24.1,fld2: _20 };
(*_6).3 = _21;
(*_2).1 = _4;
_20.2 = core::ptr::addr_of!(_1);
(*_6).0 = _1.0;
_20.0.2 = (*_2).2 << (*_6).4;
_4 = (*_6).1;
(*_2).0 = _11;
_20.0 = _25.fld2.0;
_29.0 = core::ptr::addr_of!(_28);
(*_6).3 = -_22;
Goto(bb9)
}
bb9 = {
_1 = (_3, (*_12).1, (*_6).2, (*_6).3, (*_6).4, RET, (*_2).6);
_22 = _1.3 << (*_2).6;
Call((*_12).3 = core::intrinsics::bswap(_21), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_25.fld2 = _20;
_25.fld0 = core::ptr::addr_of!(_1);
_29 = (_24.0, _25.fld1, _1.0);
_30 = [(*_2).2,(*_12).2];
(*_12).4 = _5 | _5;
_1.4 = (*_6).4;
(*_2).2 = _20.0.2;
_25.fld0 = core::ptr::addr_of!(_1);
Goto(bb11)
}
bb11 = {
(*_12).1 = _1.1 ^ _4;
_25.fld2.0.1 = [_25.fld2.0.2,_1.2,_25.fld2.0.2,(*_6).2,(*_2).2,_8,(*_2).2];
_25.fld2.0 = (_20.0.0, _20.0.1, _20.0.2);
_25.fld0 = _20.2;
_19 = _25.fld2.1;
_1.4 = (*_6).2 as i8;
_25.fld2.0.1 = [(*_12).2,(*_12).2,(*_6).2,(*_12).2,_8,(*_6).2,_8];
_9 = _1.6 - (*_2).6;
_29.0 = core::ptr::addr_of!(_15);
_13 = !(*_12).4;
(*_12).3 = _21 | _1.3;
(*_6).2 = _20.0.2 >> _22;
_2 = core::ptr::addr_of!(_1);
_1.3 = _21;
(*_12).1 = (*_2).1;
(*_12).1 = (*_2).1;
_4 = !(*_12).1;
Goto(bb12)
}
bb12 = {
(*_2).3 = -_22;
_10 = true as i8;
_3 = _29.2 << _9;
_2 = core::ptr::addr_of!((*_2));
(*_2).5 = RET;
_28 = !_15;
_30 = [(*_12).2,(*_6).2];
Goto(bb13)
}
bb13 = {
(*_2).0 = !_29.2;
_25.fld0 = _6;
_1.0 = (*_2).6 as u64;
(*_6).0 = (*_2).0 & (*_2).0;
_20.0 = _25.fld2.0;
(*_12).1 = (*_2).1 >> (*_12).4;
Goto(bb14)
}
bb14 = {
(*_12).4 = -_13;
_14.0 = 1561810188_i32 as u8;
_25.fld2.0.1 = _20.0.1;
_8 = (*_6).2 | (*_12).2;
_20.0.1 = [(*_12).2,(*_12).2,(*_6).2,_8,(*_12).2,(*_6).2,(*_12).2];
_33 = [true,true,true,true,false];
(*_6).3 = (*_2).3 * (*_2).3;
(*_6).0 = !_3;
(*_2).0 = !(*_6).0;
_1 = (_11, (*_6).1, (*_12).2, (*_6).3, (*_12).4, RET, _9);
(*_6).4 = _13;
_5 = (*_2).4 + (*_12).4;
_21 = (-1370714820274589585_i64) as isize;
_6 = _2;
_33 = [true,true,true,false,false];
(*_6).3 = !(*_12).3;
_22 = (*_12).3 & _1.3;
(*_12).2 = _1.2;
_10 = _1.4 << (*_2).1;
_24.2 = !_3;
(*_6).6 = (*_12).1 as usize;
(*_2).5 = (*_2).6 as i16;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(8_usize, 1_usize, Move(_1), 33_usize, Move(_33), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(8_usize, 27_usize, Move(_27), 8_usize, Move(_8), 18_usize, Move(_18), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(8_usize, 17_usize, Move(_17), 30_usize, Move(_30), 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (u64, u128, u32, isize, i8, i16, usize),mut _2: i8,mut _3: *const (u64, u128, u32, isize, i8, i16, usize),mut _4: *const (u64, u128, u32, isize, i8, i16, usize),mut _5: usize,mut _6: *const (u64, u128, u32, isize, i8, i16, usize)) -> [u32; 5] {
mir! {
type RET = [u32; 5];
let _7: *const (u64, u128, u32, isize, i8, i16, usize);
let _8: f32;
let _9: bool;
let _10: u8;
let _11: f64;
let _12: Adt50;
let _13: [i128; 4];
let _14: Adt53;
let _15: f32;
let _16: f64;
let _17: [i16; 8];
let _18: bool;
let _19: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _20: u128;
let _21: ();
let _22: ();
{
(*_3).1 = (*_4).6 as u128;
_1 = (*_4);
_4 = core::ptr::addr_of!(_1);
(*_3).3 = (*_6).3 >> (*_3).1;
_1 = (*_6);
(*_4).6 = _5;
(*_3).0 = (*_6).0 + (*_4).0;
(*_4).6 = !(*_3).6;
(*_3).2 = 4618_u16 as u32;
_6 = _4;
(*_4) = ((*_3).0, (*_3).1, (*_3).2, (*_3).3, (*_3).4, 21154_i16, (*_3).6);
Call((*_6).3 = fn10((*_4).5, _3, _5, (*_4).0, _1.5, (*_3).0, (*_3).4, (*_4).1, _1.6, (*_3).1, (*_3).3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4).1 = true as u128;
(*_6).2 = !(*_3).2;
(*_4) = ((*_3).0, (*_3).1, (*_3).2, (*_3).3, (*_3).4, 5450_i16, (*_3).6);
(*_6).0 = !(*_3).0;
_1.1 = (*_3).1 << (*_4).5;
(*_4).1 = (*_3).1;
(*_4).5 = -(-20586_i16);
(*_6).4 = _2 ^ (*_3).4;
(*_6).4 = -(*_3).4;
(*_4).4 = _2;
(*_6) = ((*_3).0, (*_3).1, (*_3).2, (*_3).3, _2, 23839_i16, _5);
_10 = 231_u8;
(*_4).6 = (*_3).6;
_1.0 = !(*_3).0;
(*_6).3 = '\u{df8ac}' as isize;
(*_4).0 = true as u64;
_9 = false;
_3 = _6;
_1.0 = 17437560003037020581_u64 ^ 17512778109056359809_u64;
(*_4).5 = !(-4308_i16);
(*_4).6 = _5 ^ _5;
match _10 {
0 => bb2,
231 => bb4,
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
(*_3).6 = _5 - _5;
(*_6).5 = 1781813043_i32 as i16;
(*_6).6 = !_5;
_1 = (1620643083442127134_u64, 174619463036987729918118157746270676118_u128, 165085436_u32, 9223372036854775807_isize, _2, 15711_i16, _5);
_2 = !(*_6).4;
(*_4).2 = _10 as u32;
_14.fld0 = _3;
_14.fld2.1 = [_1.2,(*_3).2,_1.2,(*_6).2,(*_6).2];
(*_3).6 = _5 | _5;
_14.fld2.0.0 = [7193304538781716965_i64,693988681597976631_i64,(-69462866860256935_i64),1052845578488935037_i64,3858360496370493724_i64,(-4448391951533037636_i64),(-2890246807044653007_i64)];
RET = [(*_6).2,_1.2,(*_4).2,(*_3).2,(*_4).2];
_3 = core::ptr::addr_of!((*_3));
(*_6) = (2260631169786190413_u64, 54283310902700349792597988873564823165_u128, 2435045598_u32, (-9223372036854775808_isize), _2, (-6119_i16), _5);
(*_6).2 = 785696503_u32 & 2077885443_u32;
(*_3).1 = 306388096526736067232301692828036589215_u128;
Call((*_4).1 = fn11(_1.6, _1.5, (*_6).0, (*_3).3, _6, (*_6).5, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_4).5 = (-15173_i16);
_14.fld2.0.2 = !(*_4).2;
_1.2 = !_14.fld2.0.2;
(*_3).2 = _14.fld2.0.2 + _14.fld2.0.2;
_11 = (*_3).1 as f64;
(*_6).6 = !_5;
(*_3).4 = _1.3 as i8;
(*_6).2 = !_14.fld2.0.2;
_1 = (5617865697230763114_u64, 18561780051866928665184468554080945259_u128, _14.fld2.0.2, (-26_isize), _2, (-6532_i16), _5);
(*_6).4 = _1.5 as i8;
(*_3).1 = !107889457660558835887030637968415695142_u128;
RET = [(*_6).2,(*_4).2,(*_3).2,(*_3).2,(*_3).2];
_3 = _14.fld0;
(*_6).4 = '\u{7ab55}' as i8;
(*_3).5 = !2870_i16;
_13 = [43486137783710179960023239804533752936_i128,129657975948898638631383884424095202450_i128,(-119411447580603120741014343501385430023_i128),(-34190673286430684584052584180092024857_i128)];
(*_3).0 = 12352102959374372391_u64 - 15027609683968729283_u64;
_7 = core::ptr::addr_of!((*_4));
(*_3).1 = 276970913098836965542133810549864257326_u128 << (*_6).6;
(*_3).2 = (-103062009503961060802347428188764496621_i128) as u32;
_14.fld2.2 = core::ptr::addr_of!((*_6));
_1.4 = !_2;
(*_3).2 = _14.fld2.0.2 + _14.fld2.0.2;
Goto(bb6)
}
bb6 = {
Call(_21 = dump_var(9_usize, 1_usize, Move(_1), 13_usize, Move(_13), 9_usize, Move(_9), 22_usize, _22), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i16,mut _2: *const (u64, u128, u32, isize, i8, i16, usize),mut _3: usize,mut _4: u64,mut _5: i16,mut _6: u64,mut _7: i8,mut _8: u128,mut _9: usize,mut _10: u128,mut _11: isize) -> isize {
mir! {
type RET = isize;
let _12: char;
let _13: ();
let _14: ();
{
_1 = _5 * _5;
RET = _11;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(10_usize, 10_usize, Move(_10), 11_usize, Move(_11), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(10_usize, 7_usize, Move(_7), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: usize,mut _2: i16,mut _3: u64,mut _4: isize,mut _5: *const (u64, u128, u32, isize, i8, i16, usize),mut _6: i16,mut _7: *const (u64, u128, u32, isize, i8, i16, usize)) -> u128 {
mir! {
type RET = u128;
let _8: isize;
let _9: ([u32; 2], i16);
let _10: i8;
let _11: [u32; 7];
let _12: i32;
let _13: i8;
let _14: isize;
let _15: ();
let _16: ();
{
(*_7).3 = _4 & _4;
(*_7).5 = 154_u8 as i16;
(*_5).5 = _6 * _6;
(*_5).0 = !_3;
(*_7).5 = (*_7).4 as i16;
(*_7).4 = -83_i8;
(*_7).0 = _3 >> (*_5).3;
_7 = _5;
_1 = !(*_7).6;
(*_5).0 = (*_7).5 as u64;
_7 = _5;
(*_7).6 = _1;
_7 = _5;
_1 = (*_5).6;
(*_5).3 = _4 | _4;
(*_7).5 = _6;
(*_7).5 = -_6;
_9.1 = _6 >> (*_5).6;
_9.0 = [(*_7).2,(*_7).2];
(*_5).3 = 43_u8 as isize;
(*_5).6 = !_1;
(*_7).5 = !_2;
(*_7).2 = 2625848961_u32;
_7 = _5;
(*_5).2 = '\u{10752d}' as u32;
(*_5).6 = _1;
Call((*_7).3 = fn12(_4, (*_5).5, (*_7).5, _9, (*_7).0, _2, (*_5).6, _5, _7, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_7).3 = '\u{f9b8d}' as isize;
_7 = _5;
_8 = '\u{88370}' as isize;
(*_5).0 = 157550812268736381722869418684091618301_u128 as u64;
_2 = _9.1;
(*_5).5 = _9.1;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
2260631169786190413 => bb6,
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
(*_5).3 = 64849_u16 as isize;
(*_7).6 = _1 | _1;
_5 = _7;
RET = (-1344031903643443029_i64) as u128;
_5 = _7;
(*_7).3 = _4;
(*_7).4 = -(-54_i8);
(*_5).0 = _3;
_8 = !_4;
(*_7).4 = false as i8;
RET = (*_5).3 as u128;
_9.1 = (-3930547175668093170047267342348403054_i128) as i16;
(*_7).5 = _6;
(*_5).5 = _2;
_2 = (*_5).5 | (*_7).5;
_7 = _5;
_1 = !(*_7).6;
(*_7).3 = _4 - _8;
_12 = 1343591587_i32;
_11 = [(*_5).2,(*_5).2,(*_5).2,(*_5).2,(*_5).2,(*_7).2,(*_5).2];
Goto(bb7)
}
bb7 = {
Call(_15 = dump_var(11_usize, 12_usize, Move(_12), 11_usize, Move(_11), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: i16,mut _3: i16,mut _4: ([u32; 2], i16),mut _5: u64,mut _6: i16,mut _7: usize,mut _8: *const (u64, u128, u32, isize, i8, i16, usize),mut _9: *const (u64, u128, u32, isize, i8, i16, usize),mut _10: *const (u64, u128, u32, isize, i8, i16, usize)) -> isize {
mir! {
type RET = isize;
let _11: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _12: Adt47;
let _13: usize;
let _14: char;
let _15: f64;
let _16: ();
let _17: ();
{
(*_10).6 = _7 * _7;
RET = (-149925827218073249598765091567949201561_i128) as isize;
_11.1.3 = -_1;
(*_8).0 = !_5;
_6 = !(*_10).5;
_11.1 = ((*_9).0, 298386151960832592119057966899482002645_u128, (*_10).2, _1, (*_8).4, (*_10).5, (*_9).6);
_3 = -(*_8).5;
(*_9).0 = _5 & _5;
(*_9).2 = _11.1.2;
(*_8).2 = _11.1.2 << (*_10).6;
(*_9).2 = (*_10).0 as u32;
_2 = (*_10).5;
Call(RET = fn13((*_10).0, _11.1.3, _11.1.6, (*_8).0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.1.6 = (-6409265511709923795_i64) as usize;
(*_10).5 = _2 & _4.1;
_1 = _11.1.3 + _11.1.3;
_6 = (*_9).5;
RET = _11.1.3 + _1;
_11.1.6 = !_7;
(*_8).0 = _11.1.0 ^ _5;
_13 = !_11.1.6;
_15 = _11.1.6 as f64;
(*_8).5 = _15 as i16;
_11.2 = 55_u8 | 8_u8;
_11.1.3 = -_1;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(12_usize, 5_usize, Move(_5), 3_usize, Move(_3), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u64,mut _2: isize,mut _3: usize,mut _4: u64) -> isize {
mir! {
type RET = isize;
let _5: Adt42;
let _6: u128;
let _7: Adt42;
let _8: Adt48;
let _9: char;
let _10: char;
let _11: [u64; 2];
let _12: [u64; 2];
let _13: (u8, u64, f32);
let _14: isize;
let _15: usize;
let _16: bool;
let _17: Adt47;
let _18: isize;
let _19: Adt56;
let _20: Adt52;
let _21: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize));
let _22: f32;
let _23: f32;
let _24: u64;
let _25: [char; 1];
let _26: ([i64; 7], [u32; 7], u32);
let _27: Adt44;
let _28: f64;
let _29: bool;
let _30: [i16; 8];
let _31: Adt56;
let _32: f64;
let _33: [i16; 8];
let _34: Adt51;
let _35: f64;
let _36: isize;
let _37: u128;
let _38: (u64, u128, u32, isize, i8, i16, usize);
let _39: *const (u64, u128, u32, isize, i8, i16, usize);
let _40: i128;
let _41: ();
let _42: ();
{
_2 = 27_isize | 9223372036854775807_isize;
_3 = '\u{5240f}' as usize;
_4 = _1;
_2 = 1725741752_i32 as isize;
_1 = _4 >> _4;
RET = _2 | _2;
_2 = RET << _4;
_3 = !5_usize;
RET = _2 - _2;
_6 = 180605389238567250583413902390891599804_u128;
_1 = !_4;
RET = (-50_i8) as isize;
_4 = (-65276576495616003872017392913508721391_i128) as u64;
_2 = RET;
_6 = !115509714749506633389378749390891665875_u128;
_3 = 4_usize ^ 1_usize;
_6 = 23693821967092093364119438070618844466_u128 + 119139983154296695160036489378553493633_u128;
_11 = [_1,_1];
_9 = '\u{2ff38}';
RET = _2 & _2;
_11 = [_1,_1];
_3 = 3_usize & 10101521548763206100_usize;
_10 = _9;
Goto(bb1)
}
bb1 = {
RET = 23990_i16 as isize;
_6 = 230161726269239719662460988950990099996_u128 - 137861440994187836978390796720607073398_u128;
_4 = 970288601_i32 as u64;
_1 = _4;
_4 = _1;
_12 = [_4,_4];
_10 = _9;
_3 = true as usize;
_13.0 = 237_u8;
_3 = 0_usize * 6_usize;
_4 = _1 - _1;
Goto(bb2)
}
bb2 = {
_1 = !_4;
RET = _2 | _2;
_10 = _9;
_1 = _4 - _4;
_6 = (-18_i8) as u128;
_6 = 240033740749600893353877738293661531447_u128;
_3 = (-60_i8) as usize;
Goto(bb3)
}
bb3 = {
_10 = _9;
_1 = !_4;
_11 = [_1,_4];
_4 = 14541_u16 as u64;
_13.2 = 6661_u16 as f32;
_11 = _12;
_2 = RET;
_13.2 = (-660975321_i32) as f32;
_13.1 = !_1;
_16 = false;
_14 = _13.0 as isize;
_13.1 = !_1;
_11 = _12;
Call(_12 = fn14(_13, _10, RET), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_21.0.2 = 4089705698_u32 | 1939140657_u32;
_3 = 328914554058524958_usize * 1713419203771496743_usize;
_15 = _14 as usize;
_10 = _9;
_12 = [_4,_1];
_12 = [_1,_1];
_21.0.2 = (-77361219979159470125803236255945006231_i128) as u32;
_14 = RET;
_15 = _3;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
RET = _13.1 as isize;
_21.0.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_21.0.0 = [8081056667836822285_i64,5928531432995055869_i64,(-8118419852653306051_i64),2078898779056363862_i64,(-4452841084810249794_i64),7354936742852717743_i64,(-7277387331797416726_i64)];
_21.0.0 = [8726877059938071593_i64,(-8419038575939339133_i64),3595576737810486732_i64,3896261415368266352_i64,8929905907191604845_i64,(-1084617031720469763_i64),(-1276623491231130665_i64)];
_9 = _10;
_13.1 = _1;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_16 = false;
_14 = !RET;
_22 = -_13.2;
_12 = [_4,_4];
_21.0.2 = 730019029_u32;
_1 = !_13.1;
_23 = _13.2 * _22;
_21.0.2 = !156794950_u32;
Goto(bb5)
}
bb5 = {
_13.2 = -_22;
_4 = (-16149_i16) as u64;
_21.0.0 = [3935459463383795131_i64,6893582150708509369_i64,(-6257141767444346653_i64),(-2373865521684793853_i64),(-5946730980802688236_i64),(-2289594871187625778_i64),4925845089833297330_i64];
_18 = _14;
_12 = _11;
_26.2 = _21.0.2;
RET = _2;
_10 = _9;
_21.0.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_26.2,_21.0.2,_26.2];
_26 = _21.0;
_26.1 = [_21.0.2,_21.0.2,_21.0.2,_26.2,_26.2,_21.0.2,_21.0.2];
_28 = _2 as f64;
_24 = _3 as u64;
_22 = -_13.2;
_26.0 = [938257860248305688_i64,1114052786813808669_i64,8904787243557691821_i64,4523626324172374717_i64,(-1974778492565439600_i64),(-1653539644731440441_i64),2284514201885181275_i64];
_12 = [_24,_13.1];
RET = _18 + _14;
_21.0.2 = _26.2 & _26.2;
_22 = _23;
_1 = _24;
_29 = _16;
match _6 {
0 => bb1,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
240033740749600893353877738293661531447 => bb11,
_ => bb10
}
}
bb6 = {
_21.0.2 = 4089705698_u32 | 1939140657_u32;
_3 = 328914554058524958_usize * 1713419203771496743_usize;
_15 = _14 as usize;
_10 = _9;
_12 = [_4,_1];
_12 = [_1,_1];
_21.0.2 = (-77361219979159470125803236255945006231_i128) as u32;
_14 = RET;
_15 = _3;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
RET = _13.1 as isize;
_21.0.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_21.0.0 = [8081056667836822285_i64,5928531432995055869_i64,(-8118419852653306051_i64),2078898779056363862_i64,(-4452841084810249794_i64),7354936742852717743_i64,(-7277387331797416726_i64)];
_21.0.0 = [8726877059938071593_i64,(-8419038575939339133_i64),3595576737810486732_i64,3896261415368266352_i64,8929905907191604845_i64,(-1084617031720469763_i64),(-1276623491231130665_i64)];
_9 = _10;
_13.1 = _1;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_16 = false;
_14 = !RET;
_22 = -_13.2;
_12 = [_4,_4];
_21.0.2 = 730019029_u32;
_1 = !_13.1;
_23 = _13.2 * _22;
_21.0.2 = !156794950_u32;
Goto(bb5)
}
bb7 = {
_10 = _9;
_1 = !_4;
_11 = [_1,_4];
_4 = 14541_u16 as u64;
_13.2 = 6661_u16 as f32;
_11 = _12;
_2 = RET;
_13.2 = (-660975321_i32) as f32;
_13.1 = !_1;
_16 = false;
_14 = _13.0 as isize;
_13.1 = !_1;
_11 = _12;
Call(_12 = fn14(_13, _10, RET), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1 = !_4;
RET = _2 | _2;
_10 = _9;
_1 = _4 - _4;
_6 = (-18_i8) as u128;
_6 = 240033740749600893353877738293661531447_u128;
_3 = (-60_i8) as usize;
Goto(bb3)
}
bb9 = {
RET = 23990_i16 as isize;
_6 = 230161726269239719662460988950990099996_u128 - 137861440994187836978390796720607073398_u128;
_4 = 970288601_i32 as u64;
_1 = _4;
_4 = _1;
_12 = [_4,_4];
_10 = _9;
_3 = true as usize;
_13.0 = 237_u8;
_3 = 0_usize * 6_usize;
_4 = _1 - _1;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_28 = _6 as f64;
_13 = (155_u8, _24, _23);
_21.0.0 = [(-8935744881173252942_i64),(-4724136414001829992_i64),(-8281502417201156054_i64),(-1432409580790438482_i64),6853444650853806256_i64,6106200076612396296_i64,(-6634487501757223167_i64)];
_6 = (-15494_i16) as u128;
_26.2 = _21.0.2;
_21.0.1 = _26.1;
_12 = _11;
_26 = _21.0;
_14 = _23 as isize;
_30 = [21671_i16,9176_i16,21972_i16,23347_i16,(-9081_i16),(-29197_i16),(-7315_i16),31562_i16];
Call(_21.2 = fn15(_21.0, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _14 - _14;
_13.2 = _1 as f32;
_24 = _13.1 ^ _1;
_28 = 35759_u16 as f64;
_30 = [(-18246_i16),25034_i16,(-565_i16),24491_i16,15371_i16,(-29092_i16),(-6981_i16),29083_i16];
_26.0 = [689271613750384454_i64,(-2375540446532146287_i64),1168564685105738747_i64,(-6348516503468301789_i64),7564159960041825436_i64,4231123385602844185_i64,4504105453388005456_i64];
_15 = _3 | _3;
_29 = _24 >= _1;
_13.2 = _22;
_23 = -_13.2;
_14 = !RET;
_26.2 = _21.0.2 * _21.0.2;
_22 = _13.2 - _23;
_21.0 = _26;
RET = _28 as isize;
_34 = Adt51::Variant0 { fld0: (-2112964732_i32),fld1: 51097_u16 };
_11 = _12;
_23 = _22;
_26.1 = [_21.0.2,_21.0.2,_21.0.2,_26.2,_26.2,_21.0.2,_26.2];
_13.0 = 19_u8;
_21.1 = [_26.2,_21.0.2,_26.2,_21.0.2,_21.0.2];
place!(Field::<i32>(Variant(_34, 0), 0)) = _6 as i32;
_21.0.1 = _26.1;
_26.0 = [24903926715863425_i64,8361395093748024052_i64,(-9165155178071662660_i64),2753275756750984756_i64,3825846218945678118_i64,1969281545341278463_i64,7065915208651624223_i64];
_13.1 = _6 as u64;
Goto(bb13)
}
bb13 = {
_33 = [(-2921_i16),(-16766_i16),5152_i16,15947_i16,2544_i16,7359_i16,(-29943_i16),(-24865_i16)];
_35 = _28;
_15 = 31024_i16 as usize;
_10 = _9;
_32 = _28 + _28;
_12 = [_1,_13.1];
place!(Field::<i32>(Variant(_34, 0), 0)) = 758960608_i32;
_28 = _3 as f64;
_10 = _9;
_23 = _15 as f32;
_10 = _9;
_36 = _18;
_30 = [(-2886_i16),(-6319_i16),(-21498_i16),847_i16,(-29428_i16),(-29971_i16),28608_i16,(-1743_i16)];
match _13.0 {
0 => bb14,
1 => bb15,
2 => bb16,
19 => bb18,
_ => bb17
}
}
bb14 = {
_1 = !_4;
RET = _2 | _2;
_10 = _9;
_1 = _4 - _4;
_6 = (-18_i8) as u128;
_6 = 240033740749600893353877738293661531447_u128;
_3 = (-60_i8) as usize;
Goto(bb3)
}
bb15 = {
_28 = _6 as f64;
_13 = (155_u8, _24, _23);
_21.0.0 = [(-8935744881173252942_i64),(-4724136414001829992_i64),(-8281502417201156054_i64),(-1432409580790438482_i64),6853444650853806256_i64,6106200076612396296_i64,(-6634487501757223167_i64)];
_6 = (-15494_i16) as u128;
_26.2 = _21.0.2;
_21.0.1 = _26.1;
_12 = _11;
_26 = _21.0;
_14 = _23 as isize;
_30 = [21671_i16,9176_i16,21972_i16,23347_i16,(-9081_i16),(-29197_i16),(-7315_i16),31562_i16];
Call(_21.2 = fn15(_21.0, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_21.0.2 = 4089705698_u32 | 1939140657_u32;
_3 = 328914554058524958_usize * 1713419203771496743_usize;
_15 = _14 as usize;
_10 = _9;
_12 = [_4,_1];
_12 = [_1,_1];
_21.0.2 = (-77361219979159470125803236255945006231_i128) as u32;
_14 = RET;
_15 = _3;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
RET = _13.1 as isize;
_21.0.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_21.0.0 = [8081056667836822285_i64,5928531432995055869_i64,(-8118419852653306051_i64),2078898779056363862_i64,(-4452841084810249794_i64),7354936742852717743_i64,(-7277387331797416726_i64)];
_21.0.0 = [8726877059938071593_i64,(-8419038575939339133_i64),3595576737810486732_i64,3896261415368266352_i64,8929905907191604845_i64,(-1084617031720469763_i64),(-1276623491231130665_i64)];
_9 = _10;
_13.1 = _1;
_21.1 = [_21.0.2,_21.0.2,_21.0.2,_21.0.2,_21.0.2];
_16 = false;
_14 = !RET;
_22 = -_13.2;
_12 = [_4,_4];
_21.0.2 = 730019029_u32;
_1 = !_13.1;
_23 = _13.2 * _22;
_21.0.2 = !156794950_u32;
Goto(bb5)
}
bb17 = {
RET = 23990_i16 as isize;
_6 = 230161726269239719662460988950990099996_u128 - 137861440994187836978390796720607073398_u128;
_4 = 970288601_i32 as u64;
_1 = _4;
_4 = _1;
_12 = [_4,_4];
_10 = _9;
_3 = true as usize;
_13.0 = 237_u8;
_3 = 0_usize * 6_usize;
_4 = _1 - _1;
Goto(bb2)
}
bb18 = {
_14 = _13.0 as isize;
_22 = _13.2;
_23 = _22 + _22;
_2 = _18 | _36;
_21.0.1 = [_21.0.2,_26.2,_21.0.2,_26.2,_26.2,_26.2,_26.2];
_13.1 = !_24;
_26.2 = _21.0.2 | _21.0.2;
_18 = _13.0 as isize;
_38.5 = 849_i16;
_22 = _23 * _13.2;
_28 = -_32;
_13.2 = _23 - _22;
_34 = Adt51::Variant0 { fld0: 978367849_i32,fld1: 38291_u16 };
_38.1 = _6 * _6;
_26.2 = 56388_u16 as u32;
_33 = _30;
_3 = !_15;
_11 = [_13.1,_1];
_38.6 = _28 as usize;
_10 = _9;
_34 = Adt51::Variant0 { fld0: 246180661_i32,fld1: 35456_u16 };
_23 = _6 as f32;
_39 = core::ptr::addr_of!(_38);
RET = _14;
(*_39) = (_13.1, _6, _21.0.2, RET, 33_i8, (-11946_i16), _15);
_38.4 = -(-109_i8);
_4 = _36 as u64;
Goto(bb19)
}
bb19 = {
Call(_41 = dump_var(13_usize, 38_usize, Move(_38), 11_usize, Move(_11), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_41 = dump_var(13_usize, 24_usize, Move(_24), 4_usize, Move(_4), 16_usize, Move(_16), 10_usize, Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_41 = dump_var(13_usize, 15_usize, Move(_15), 3_usize, Move(_3), 42_usize, _42, 42_usize, _42), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (u8, u64, f32),mut _2: char,mut _3: isize) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _4: f32;
let _5: isize;
let _6: i32;
let _7: Adt43;
let _8: isize;
let _9: [u32; 2];
let _10: bool;
let _11: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _12: [i16; 8];
let _13: isize;
let _14: Adt46;
let _15: u128;
let _16: Adt51;
let _17: bool;
let _18: &'static (u64, u128, u32, isize, i8, i16, usize);
let _19: [char; 1];
let _20: [i64; 7];
let _21: isize;
let _22: Adt58;
let _23: *const (u64, u128, u32, isize, i8, i16, usize);
let _24: f64;
let _25: u32;
let _26: Adt56;
let _27: [i64; 7];
let _28: isize;
let _29: bool;
let _30: char;
let _31: (isize, (u64, u128, u32, isize, i8, i16, usize), u8);
let _32: ();
let _33: ();
{
_3 = 21_isize;
RET = [_1.1,_1.1];
_2 = '\u{a7518}';
_3 = -9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
RET = [_1.1,_1.1];
_4 = _1.2;
_6 = 155269235672848994108188889101814209514_u128 as i32;
_2 = '\u{f40cf}';
_5 = 38242753644531028228038866056492527629_u128 as isize;
_6 = -1268506304_i32;
_1 = (182_u8, 11471494354448340002_u64, _4);
_2 = '\u{10c9ac}';
RET = [_1.1,_1.1];
_1.1 = 2981421599923620380_u64 + 5736873216067036851_u64;
_1.1 = !3345153124014809060_u64;
RET = [_1.1,_1.1];
RET = [_1.1,_1.1];
_2 = '\u{e84e8}';
_3 = !_5;
_6 = 1009777015_i32 * 183633887_i32;
_4 = -_1.2;
Goto(bb2)
}
bb2 = {
RET = [_1.1,_1.1];
_1.2 = -_4;
_8 = !_3;
_1.1 = 17252259525038704522_u64;
_8 = _5;
RET = [_1.1,_1.1];
_1.0 = 112_u8;
_3 = (-2975192950503485424_i64) as isize;
_4 = _1.2 + _1.2;
_3 = 2458_i16 as isize;
_2 = '\u{859d4}';
_5 = _3 + _8;
_1 = (117_u8, 8114535039178311941_u64, _4);
_1.0 = !175_u8;
_8 = _5;
_9 = [2676727982_u32,261604133_u32];
_5 = 4234855105_u32 as isize;
_3 = !_8;
_8 = !_3;
_1.0 = !111_u8;
Goto(bb3)
}
bb3 = {
RET = [_1.1,_1.1];
_1 = (84_u8, 18104793471159601131_u64, _4);
RET = [_1.1,_1.1];
Goto(bb4)
}
bb4 = {
_3 = !_8;
_11.2 = _1.0;
RET = [_1.1,_1.1];
_6 = 883335496_i32;
_10 = _3 > _8;
_14.fld2 = _1.1 as u16;
RET = [_1.1,_1.1];
_11.0 = _8;
_11.1 = (_1.1, 15364156370327178183383570960192978986_u128, 1898925850_u32, _8, 72_i8, (-4033_i16), 12486468274550581136_usize);
_11.1.4 = 45_i8 | (-57_i8);
_14 = Adt46 { fld0: 149679004551596458991982343070051367056_i128,fld1: _11.1.1,fld2: 44459_u16 };
Goto(bb5)
}
bb5 = {
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_4 = _11.1.3 as f32;
_15 = _14.fld1 * _11.1.1;
_8 = _11.0 & _3;
_11.1.1 = _15 >> _11.1.2;
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_11.1.4 = !36_i8;
_1.0 = !_11.2;
_10 = _11.1.2 == _11.1.2;
Goto(bb6)
}
bb6 = {
_6 = (-7786223183825514463_i64) as i32;
_11.1.2 = 2956003757_u32;
_11.1.0 = _1.1;
_19 = [_2];
_8 = !_11.1.3;
_11.1.3 = _11.2 as isize;
_19 = [_2];
_1.0 = !_11.2;
_14.fld1 = _11.1.5 as u128;
_13 = !_11.1.3;
_1.2 = _4 + _4;
_9 = [_11.1.2,_11.1.2];
_20 = [548678376165934187_i64,6758988895672955164_i64,(-5260211260049211253_i64),(-7112302458352427828_i64),8652817405776441317_i64,6223015041161476747_i64,(-8379934706299019889_i64)];
_5 = _11.1.3;
_14.fld1 = _11.1.1 * _15;
RET = [_11.1.0,_11.1.0];
_5 = _8;
_11.1.5 = -29300_i16;
RET = [_1.1,_1.1];
_11.0 = _11.1.3;
_16 = Adt51::Variant0 { fld0: _6,fld1: _14.fld2 };
_2 = '\u{d9b90}';
match _11.1.6 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb9,
12486468274550581136 => bb11,
_ => bb10
}
}
bb7 = {
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_4 = _11.1.3 as f32;
_15 = _14.fld1 * _11.1.1;
_8 = _11.0 & _3;
_11.1.1 = _15 >> _11.1.2;
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_11.1.4 = !36_i8;
_1.0 = !_11.2;
_10 = _11.1.2 == _11.1.2;
Goto(bb6)
}
bb8 = {
_3 = !_8;
_11.2 = _1.0;
RET = [_1.1,_1.1];
_6 = 883335496_i32;
_10 = _3 > _8;
_14.fld2 = _1.1 as u16;
RET = [_1.1,_1.1];
_11.0 = _8;
_11.1 = (_1.1, 15364156370327178183383570960192978986_u128, 1898925850_u32, _8, 72_i8, (-4033_i16), 12486468274550581136_usize);
_11.1.4 = 45_i8 | (-57_i8);
_14 = Adt46 { fld0: 149679004551596458991982343070051367056_i128,fld1: _11.1.1,fld2: 44459_u16 };
Goto(bb5)
}
bb9 = {
RET = [_1.1,_1.1];
_4 = _1.2;
_6 = 155269235672848994108188889101814209514_u128 as i32;
_2 = '\u{f40cf}';
_5 = 38242753644531028228038866056492527629_u128 as isize;
_6 = -1268506304_i32;
_1 = (182_u8, 11471494354448340002_u64, _4);
_2 = '\u{10c9ac}';
RET = [_1.1,_1.1];
_1.1 = 2981421599923620380_u64 + 5736873216067036851_u64;
_1.1 = !3345153124014809060_u64;
RET = [_1.1,_1.1];
RET = [_1.1,_1.1];
_2 = '\u{e84e8}';
_3 = !_5;
_6 = 1009777015_i32 * 183633887_i32;
_4 = -_1.2;
Goto(bb2)
}
bb10 = {
RET = [_1.1,_1.1];
_1.2 = -_4;
_8 = !_3;
_1.1 = 17252259525038704522_u64;
_8 = _5;
RET = [_1.1,_1.1];
_1.0 = 112_u8;
_3 = (-2975192950503485424_i64) as isize;
_4 = _1.2 + _1.2;
_3 = 2458_i16 as isize;
_2 = '\u{859d4}';
_5 = _3 + _8;
_1 = (117_u8, 8114535039178311941_u64, _4);
_1.0 = !175_u8;
_8 = _5;
_9 = [2676727982_u32,261604133_u32];
_5 = 4234855105_u32 as isize;
_3 = !_8;
_8 = !_3;
_1.0 = !111_u8;
Goto(bb3)
}
bb11 = {
_14 = Adt46 { fld0: (-149565300730245893166375190961902431727_i128),fld1: _11.1.1,fld2: Field::<u16>(Variant(_16, 0), 1) };
_14 = Adt46 { fld0: 138166640472207498854233525190159912984_i128,fld1: _15,fld2: Field::<u16>(Variant(_16, 0), 1) };
place!(Field::<i32>(Variant(_16, 0), 0)) = _6;
_11.1 = (_1.1, _15, 1687778695_u32, _11.0, 6_i8, (-29090_i16), 14575365914744225234_usize);
_2 = '\u{288eb}';
place!(Field::<i32>(Variant(_16, 0), 0)) = _6 ^ _6;
_24 = _11.1.4 as f64;
_17 = !_10;
SetDiscriminant(_16, 0);
_11.1.5 = _24 as i16;
_11.1.3 = _11.0 & _8;
match _11.1.4 {
0 => bb6,
1 => bb12,
2 => bb13,
6 => bb15,
_ => bb14
}
}
bb12 = {
RET = [_1.1,_1.1];
_1.2 = -_4;
_8 = !_3;
_1.1 = 17252259525038704522_u64;
_8 = _5;
RET = [_1.1,_1.1];
_1.0 = 112_u8;
_3 = (-2975192950503485424_i64) as isize;
_4 = _1.2 + _1.2;
_3 = 2458_i16 as isize;
_2 = '\u{859d4}';
_5 = _3 + _8;
_1 = (117_u8, 8114535039178311941_u64, _4);
_1.0 = !175_u8;
_8 = _5;
_9 = [2676727982_u32,261604133_u32];
_5 = 4234855105_u32 as isize;
_3 = !_8;
_8 = !_3;
_1.0 = !111_u8;
Goto(bb3)
}
bb13 = {
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_4 = _11.1.3 as f32;
_15 = _14.fld1 * _11.1.1;
_8 = _11.0 & _3;
_11.1.1 = _15 >> _11.1.2;
_12 = [_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5,_11.1.5];
_11.1.4 = !36_i8;
_1.0 = !_11.2;
_10 = _11.1.2 == _11.1.2;
Goto(bb6)
}
bb14 = {
_3 = !_8;
_11.2 = _1.0;
RET = [_1.1,_1.1];
_6 = 883335496_i32;
_10 = _3 > _8;
_14.fld2 = _1.1 as u16;
RET = [_1.1,_1.1];
_11.0 = _8;
_11.1 = (_1.1, 15364156370327178183383570960192978986_u128, 1898925850_u32, _8, 72_i8, (-4033_i16), 12486468274550581136_usize);
_11.1.4 = 45_i8 | (-57_i8);
_14 = Adt46 { fld0: 149679004551596458991982343070051367056_i128,fld1: _11.1.1,fld2: 44459_u16 };
Goto(bb5)
}
bb15 = {
_11.0 = _11.1.0 as isize;
_1.1 = _11.1.0 & _11.1.0;
_11.1.1 = !_14.fld1;
_11.1.3 = _11.0 | _11.0;
_11.1.5 = !21845_i16;
_11.0 = !_11.1.3;
_11.1.2 = 1068471418_u32 & 2312429449_u32;
_29 = _17;
RET = [_1.1,_1.1];
_27 = _20;
_30 = _2;
_29 = _17 | _10;
_19 = [_2];
_11.0 = _13 + _3;
place!(Field::<u16>(Variant(_16, 0), 1)) = _14.fld2;
_11.1.3 = _13 & _11.0;
_11.1.6 = !7_usize;
_28 = _11.1.3 << _11.0;
_11.1.1 = _1.1 as u128;
_10 = !_17;
_8 = _11.1.3 | _28;
_14.fld0 = -(-102170414802674508356349715681220163193_i128);
_17 = _10;
_31.1.1 = !_14.fld1;
_4 = -_1.2;
_8 = _30 as isize;
_24 = _11.1.5 as f64;
_3 = _13 ^ _5;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(14_usize, 5_usize, Move(_5), 20_usize, Move(_20), 29_usize, Move(_29), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(14_usize, 13_usize, Move(_13), 17_usize, Move(_17), 27_usize, Move(_27), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(14_usize, 8_usize, Move(_8), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: ([i64; 7], [u32; 7], u32),mut _2: f32) -> *const (u64, u128, u32, isize, i8, i16, usize) {
mir! {
type RET = *const (u64, u128, u32, isize, i8, i16, usize);
let _3: isize;
let _4: Adt53;
let _5: isize;
let _6: ([i64; 7], [u32; 7], u32);
let _7: ([i64; 7], [u32; 7], u32);
let _8: [i16; 8];
let _9: f64;
let _10: f64;
let _11: i128;
let _12: Adt46;
let _13: ([u32; 2], i16);
let _14: Adt58;
let _15: *mut &'static (u64, u128, u32, isize, i8, i16, usize);
let _16: f64;
let _17: *mut [i128; 4];
let _18: isize;
let _19: isize;
let _20: isize;
let _21: bool;
let _22: Adt45;
let _23: ([i64; 7], [u32; 7], u32);
let _24: (*const i128, [i16; 6], u64);
let _25: [i16; 8];
let _26: (*const i128, [i16; 6], u64);
let _27: u32;
let _28: Adt46;
let _29: (u64, u128, u32, isize, i8, i16, usize);
let _30: *const (u64, u128, u32, isize, i8, i16, usize);
let _31: bool;
let _32: Adt51;
let _33: u16;
let _34: f32;
let _35: bool;
let _36: Adt56;
let _37: bool;
let _38: *mut [i128; 4];
let _39: ();
let _40: ();
{
_1.2 = (-125333149880540734274099149335460053535_i128) as u32;
_1.0 = [5736931597154781935_i64,(-4497651272516003518_i64),5065106498325907679_i64,(-6698031182988413961_i64),909346039287375428_i64,1011819269201436050_i64,7937065764532524273_i64];
_3 = 9223372036854775807_isize;
_2 = _1.2 as f32;
_1.0 = [947023405140046974_i64,639455452792751255_i64,(-2601300993449451907_i64),(-197358335990926432_i64),(-473825904266547128_i64),6660817744953554170_i64,5102239116911327720_i64];
_4.fld2.0.2 = !_1.2;
_4.fld2.0.0 = [(-3637267508916933588_i64),3462645773839855096_i64,1344277146433594887_i64,(-5109860339222833515_i64),(-7178028885314491112_i64),2840159371891386620_i64,(-896689388280460330_i64)];
_6.1 = _1.1;
_7 = (_1.0, _6.1, _1.2);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
9223372036854775807 => bb5,
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
_4.fld2.0.0 = _7.0;
_1.1 = _7.1;
_7.2 = !_1.2;
_7 = _1;
_7.2 = !_1.2;
_6.0 = [2117217768655747758_i64,(-4667902545521443339_i64),(-5553546582759965181_i64),(-3358090736600719413_i64),2553553292172845128_i64,(-9206655207204097814_i64),7312066062827053062_i64];
_7.0 = [(-2257222307072494489_i64),4155785184256329778_i64,(-1998774603383435060_i64),(-3879905922657380286_i64),2518506351541792926_i64,8974770183343006331_i64,7273316858806368688_i64];
_4.fld2.0 = (_7.0, _6.1, _1.2);
_8 = [19576_i16,9671_i16,4147_i16,(-31446_i16),18468_i16,16702_i16,(-28466_i16),(-15656_i16)];
_4.fld2.1 = [_4.fld2.0.2,_1.2,_4.fld2.0.2,_7.2,_7.2];
_5 = true as isize;
_6.0 = _4.fld2.0.0;
_6.0 = [4708037818864004253_i64,(-3700218279971959387_i64),2207399131659125817_i64,3814358595741652526_i64,(-2746785302267120038_i64),8035793486400647981_i64,(-6051017748868372363_i64)];
_5 = -_3;
_1.0 = [843646211512252097_i64,(-8110666411452308034_i64),(-9053489129124610459_i64),(-3624133536120064328_i64),247699382801836479_i64,(-8564106071167653699_i64),7631909096688817804_i64];
_4.fld2.0 = (_6.0, _7.1, _7.2);
_4.fld2.1 = [_4.fld2.0.2,_4.fld2.0.2,_1.2,_1.2,_4.fld2.0.2];
_5 = 13165654894966748458_u64 as isize;
_2 = 42792_u16 as f32;
match _3 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
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
_6.0 = _7.0;
_3 = _5;
_1.1 = [_7.2,_1.2,_4.fld2.0.2,_1.2,_7.2,_4.fld2.0.2,_1.2];
_4.fld2.0 = (_1.0, _1.1, _7.2);
_7.0 = [2022741355674315253_i64,(-2152596492409449285_i64),(-7149874207193389705_i64),(-125219450962526054_i64),7864702652546899450_i64,(-4705529492596544004_i64),1272152116406220700_i64];
_4.fld1 = [(-16815_i16),(-15331_i16),1327_i16,(-20920_i16),22168_i16,(-3074_i16)];
_6.0 = [7584342513308435668_i64,6307218315687261343_i64,(-2114258745512076659_i64),(-4401060730170440831_i64),(-1029758627409789329_i64),7953418247443749923_i64,5346717363531910881_i64];
_6 = (_1.0, _4.fld2.0.1, _7.2);
_5 = 5755714690181734158_i64 as isize;
_6.1 = [_4.fld2.0.2,_1.2,_1.2,_6.2,_6.2,_6.2,_1.2];
_4.fld2.0.1 = [_6.2,_6.2,_7.2,_7.2,_7.2,_4.fld2.0.2,_7.2];
_1 = (_7.0, _4.fld2.0.1, _4.fld2.0.2);
_1.2 = 85_i8 as u32;
_11 = (-19_i8) as i128;
_7.0 = [(-4153848367828684635_i64),5278982766418233371_i64,3041679411861993878_i64,5100551703804124174_i64,6884889762425014031_i64,8513148717771409227_i64,3018985363981601796_i64];
_4.fld1 = [(-15035_i16),(-21180_i16),21202_i16,(-25360_i16),27837_i16,(-31805_i16)];
_6 = _7;
_4.fld2.1 = [_4.fld2.0.2,_4.fld2.0.2,_1.2,_6.2,_6.2];
_12 = Adt46 { fld0: _11,fld1: 43439045629096027257736508517217108164_u128,fld2: 9060_u16 };
_11 = _12.fld0;
Call(_10 = core::intrinsics::transmute(_3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4.fld2.0.1 = _7.1;
_4.fld1 = [(-14315_i16),26929_i16,(-30621_i16),(-26420_i16),21755_i16,14113_i16];
Call(_2 = fn16(_8, _3, _4.fld2.0, _7, _4.fld2.0, _1, Move(_12), _8, _6.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_3 = 58348_u16 as isize;
_12 = Adt46 { fld0: _11,fld1: 274406136107122936007268206082244578680_u128,fld2: 51465_u16 };
_7.1 = _4.fld2.0.1;
_7.1 = _6.1;
_7.0 = [7052819474496095599_i64,1089524065766377131_i64,(-3925202327947696625_i64),(-4987707488458727113_i64),8489146905639427032_i64,8441821088470994603_i64,(-9103403994387086248_i64)];
_12 = Adt46 { fld0: _11,fld1: 95681589047789823686386429816542628028_u128,fld2: 17657_u16 };
_4.fld2.0.0 = _1.0;
_13.0 = [_1.2,_6.2];
_1.0 = _4.fld2.0.0;
_13.1 = 209_i16;
_9 = _10;
_1.2 = !_6.2;
_1 = (_6.0, _6.1, _4.fld2.0.2);
_13.1 = -1562_i16;
_1.2 = !_4.fld2.0.2;
_4.fld2.0 = _6;
_13.1 = 28396_i16 << _12.fld0;
_16 = -_9;
_13.1 = !(-13187_i16);
_12.fld2 = 53415_u16 >> _12.fld1;
_6 = (_7.0, _1.1, _4.fld2.0.2);
_7.2 = !_1.2;
_1.2 = _7.2 << _13.1;
_4.fld2.0 = (_7.0, _1.1, _1.2);
_18 = false as isize;
_4.fld2.0.2 = 230_u8 as u32;
Goto(bb15)
}
bb15 = {
_6.1 = _7.1;
_1.2 = _2 as u32;
_4.fld1 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_1.2 = _7.2;
_4.fld2.0 = _7;
_6.2 = _1.2;
Goto(bb16)
}
bb16 = {
_4.fld1 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_20 = -_3;
_21 = !true;
_1 = (_6.0, _7.1, _6.2);
_19 = -_18;
_1.2 = !_6.2;
_21 = _1.2 <= _7.2;
_10 = -_9;
_9 = _10;
_2 = (-1437239337_i32) as f32;
_8 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_2 = _12.fld2 as f32;
_20 = _12.fld2 as isize;
_3 = _2 as isize;
_7.1 = [_7.2,_7.2,_7.2,_6.2,_7.2,_1.2,_4.fld2.0.2];
_18 = 40_i8 as isize;
_8 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_6.1 = [_6.2,_6.2,_7.2,_4.fld2.0.2,_7.2,_1.2,_1.2];
_7 = (_1.0, _1.1, _6.2);
_6.1 = [_7.2,_6.2,_4.fld2.0.2,_6.2,_1.2,_7.2,_6.2];
match _12.fld1 {
95681589047789823686386429816542628028 => bb18,
_ => bb17
}
}
bb17 = {
Return()
}
bb18 = {
_1.1 = [_6.2,_7.2,_6.2,_7.2,_7.2,_1.2,_6.2];
_7.1 = [_1.2,_1.2,_7.2,_6.2,_7.2,_4.fld2.0.2,_6.2];
_1.0 = _6.0;
_6.2 = (-7727711511652197410_i64) as u32;
_11 = _12.fld1 as i128;
_8 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_12 = Adt46 { fld0: _11,fld1: 289707638407267862925172619582186443260_u128,fld2: 12494_u16 };
_23.0 = [(-4083591660751650105_i64),560082949245356703_i64,886159875579703698_i64,(-1594606017762270970_i64),(-4676124749234780761_i64),1778884949453037119_i64,8959118995404336166_i64];
_12.fld1 = 75773424527392641319047276792821337556_u128 | 283439982717844843976314738678907895725_u128;
_11 = 3935848812593846765_u64 as i128;
Goto(bb19)
}
bb19 = {
_1.0 = [(-4879094085581515941_i64),6492080155687454844_i64,331028395405082317_i64,8693672962549576505_i64,(-763450596056969739_i64),163226624283342486_i64,(-8853540560116202907_i64)];
_26.0 = core::ptr::addr_of!(_12.fld0);
_7.0 = _23.0;
_23.1 = [_4.fld2.0.2,_4.fld2.0.2,_1.2,_1.2,_6.2,_6.2,_4.fld2.0.2];
_23.0 = _1.0;
_4.fld2.0.0 = _7.0;
_19 = _18;
_12 = Adt46 { fld0: _11,fld1: 181098027096956609799518067184063785731_u128,fld2: 33043_u16 };
_6.1 = [_1.2,_1.2,_6.2,_4.fld2.0.2,_6.2,_7.2,_7.2];
_4.fld2.0.2 = _7.2;
_26.1 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_23 = (_6.0, _4.fld2.0.1, _1.2);
_21 = true;
_24.1 = _4.fld1;
_13.1 = (-11499_i16) + (-28683_i16);
_22 = Adt45::Variant3 { fld0: _7 };
_12 = Adt46 { fld0: _11,fld1: 1529113502330908005622041864475429521_u128,fld2: 36953_u16 };
_6 = _23;
_3 = !_20;
_18 = !_19;
_21 = !true;
_26.2 = 12352827012262780932_u64 * 18416019987902056607_u64;
Call(_29.6 = core::intrinsics::transmute(_26.2), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_4.fld0 = core::ptr::addr_of!(_29);
_7 = (_4.fld2.0.0, _6.1, _1.2);
_1.1 = [_4.fld2.0.2,_1.2,Field::<([i64; 7], [u32; 7], u32)>(Variant(_22, 3), 0).2,_4.fld2.0.2,_4.fld2.0.2,_4.fld2.0.2,_1.2];
_4.fld2.0.1 = [_4.fld2.0.2,_7.2,Field::<([i64; 7], [u32; 7], u32)>(Variant(_22, 3), 0).2,_23.2,_6.2,_6.2,_4.fld2.0.2];
_30 = core::ptr::addr_of!(_29);
(*_30) = (_26.2, _12.fld1, _1.2, _19, (-40_i8), _13.1, 4_usize);
_29.4 = (-7_i8);
_12 = Adt46 { fld0: _11,fld1: (*_30).1,fld2: 9384_u16 };
_6 = (_23.0, _4.fld2.0.1, (*_30).2);
SetDiscriminant(_22, 0);
_9 = _16 * _16;
_28 = Move(_12);
_24 = (_26.0, _26.1, _29.0);
place!(Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_22, 0), 3)).1.1 = (*_30).1 - (*_30).1;
_22 = Adt45::Variant3 { fld0: _7 };
_33 = _28.fld2;
_18 = _7.2 as isize;
_12.fld2 = _28.fld2 << (*_30).6;
_23 = (_6.0, _6.1, _29.2);
(*_30).3 = _20 >> _29.6;
match (*_30).6 {
0 => bb15,
1 => bb9,
2 => bb3,
4 => bb22,
_ => bb21
}
}
bb21 = {
_1.0 = [(-4879094085581515941_i64),6492080155687454844_i64,331028395405082317_i64,8693672962549576505_i64,(-763450596056969739_i64),163226624283342486_i64,(-8853540560116202907_i64)];
_26.0 = core::ptr::addr_of!(_12.fld0);
_7.0 = _23.0;
_23.1 = [_4.fld2.0.2,_4.fld2.0.2,_1.2,_1.2,_6.2,_6.2,_4.fld2.0.2];
_23.0 = _1.0;
_4.fld2.0.0 = _7.0;
_19 = _18;
_12 = Adt46 { fld0: _11,fld1: 181098027096956609799518067184063785731_u128,fld2: 33043_u16 };
_6.1 = [_1.2,_1.2,_6.2,_4.fld2.0.2,_6.2,_7.2,_7.2];
_4.fld2.0.2 = _7.2;
_26.1 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_23 = (_6.0, _4.fld2.0.1, _1.2);
_21 = true;
_24.1 = _4.fld1;
_13.1 = (-11499_i16) + (-28683_i16);
_22 = Adt45::Variant3 { fld0: _7 };
_12 = Adt46 { fld0: _11,fld1: 1529113502330908005622041864475429521_u128,fld2: 36953_u16 };
_6 = _23;
_3 = !_20;
_18 = !_19;
_21 = !true;
_26.2 = 12352827012262780932_u64 * 18416019987902056607_u64;
Call(_29.6 = core::intrinsics::transmute(_26.2), ReturnTo(bb20), UnwindUnreachable())
}
bb22 = {
_7 = Field::<([i64; 7], [u32; 7], u32)>(Variant(_22, 3), 0);
_29.1 = _28.fld1 * _28.fld1;
_6.1 = [_7.2,_6.2,(*_30).2,_6.2,(*_30).2,_29.2,_4.fld2.0.2];
_24 = _26;
Goto(bb23)
}
bb23 = {
_7.1 = Field::<([i64; 7], [u32; 7], u32)>(Variant(_22, 3), 0).1;
_25 = [_29.5,(*_30).5,(*_30).5,(*_30).5,(*_30).5,(*_30).5,_13.1,(*_30).5];
_21 = false;
_29.2 = _1.2;
_7.2 = _11 as u32;
_12.fld2 = _33 | _33;
_4.fld2.0.2 = !_7.2;
_29.2 = _23.2;
_28 = Adt46 { fld0: _11,fld1: _29.1,fld2: _12.fld2 };
_7.0 = _23.0;
(*_30).6 = 6_usize;
_26 = (_24.0, _4.fld1, (*_30).0);
(*_30).4 = 41_i8;
_28.fld0 = _11;
_22 = Adt45::Variant2 { fld0: _21,fld1: _13.1 };
_13.0 = [(*_30).2,_6.2];
_18 = -_29.3;
_28.fld2 = _12.fld2 + _12.fld2;
match _29.4 {
0 => bb9,
1 => bb7,
2 => bb24,
3 => bb25,
41 => bb27,
_ => bb26
}
}
bb24 = {
_4.fld1 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_20 = -_3;
_21 = !true;
_1 = (_6.0, _7.1, _6.2);
_19 = -_18;
_1.2 = !_6.2;
_21 = _1.2 <= _7.2;
_10 = -_9;
_9 = _10;
_2 = (-1437239337_i32) as f32;
_8 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_2 = _12.fld2 as f32;
_20 = _12.fld2 as isize;
_3 = _2 as isize;
_7.1 = [_7.2,_7.2,_7.2,_6.2,_7.2,_1.2,_4.fld2.0.2];
_18 = 40_i8 as isize;
_8 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_6.1 = [_6.2,_6.2,_7.2,_4.fld2.0.2,_7.2,_1.2,_1.2];
_7 = (_1.0, _1.1, _6.2);
_6.1 = [_7.2,_6.2,_4.fld2.0.2,_6.2,_1.2,_7.2,_6.2];
match _12.fld1 {
95681589047789823686386429816542628028 => bb18,
_ => bb17
}
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
_12.fld0 = _11 | _11;
RET = _4.fld0;
_12.fld0 = _11 & _11;
_29.6 = !102280548373374987_usize;
_29.3 = _18 - _20;
_29.6 = !4_usize;
_24.1 = _4.fld1;
(*RET) = (_24.2, _28.fld1, _4.fld2.0.2, _18, (-63_i8), _13.1, 4539278570847719307_usize);
_34 = -_2;
_23 = _1;
(*_30).4 = (-45_i8);
_1.0 = [9189433564157090015_i64,7605214852306915693_i64,(-6698164006115817830_i64),(-3002952515195999458_i64),(-1496938593830295158_i64),2465413607649210764_i64,(-1271963440293093051_i64)];
_29 = (_26.2, _28.fld1, _6.2, _20, 96_i8, Field::<i16>(Variant(_22, 2), 1), 4_usize);
(*RET) = (_26.2, _28.fld1, _6.2, _3, (-23_i8), _13.1, 0_usize);
(*RET).4 = (-32_i8);
_29.4 = (-50_i8) + 56_i8;
_3 = _29.3;
(*_30).2 = _1.2;
(*_30).6 = (*RET).5 as usize;
_27 = _23.2 & (*RET).2;
RET = core::ptr::addr_of!((*_30));
Goto(bb28)
}
bb28 = {
Call(_39 = dump_var(15_usize, 20_usize, Move(_20), 11_usize, Move(_11), 23_usize, Move(_23), 19_usize, Move(_19)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_39 = dump_var(15_usize, 27_usize, Move(_27), 8_usize, Move(_8), 29_usize, Move(_29), 21_usize, Move(_21)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i16; 8],mut _2: isize,mut _3: ([i64; 7], [u32; 7], u32),mut _4: ([i64; 7], [u32; 7], u32),mut _5: ([i64; 7], [u32; 7], u32),mut _6: ([i64; 7], [u32; 7], u32),mut _7: Adt46,mut _8: [i16; 8],mut _9: [i64; 7]) -> f32 {
mir! {
type RET = f32;
let _10: isize;
let _11: ([u32; 2], i16);
let _12: [i16; 8];
let _13: i16;
let _14: bool;
let _15: f64;
let _16: f32;
let _17: isize;
let _18: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32);
let _19: f32;
let _20: f32;
let _21: Adt45;
let _22: *const (u64, u128, u32, isize, i8, i16, usize);
let _23: u128;
let _24: Adt43;
let _25: isize;
let _26: u8;
let _27: [i64; 7];
let _28: f64;
let _29: i128;
let _30: f64;
let _31: isize;
let _32: ([i64; 7], [u32; 7], u32);
let _33: ();
let _34: ();
{
_6 = (_9, _5.1, _5.2);
_3.0 = [(-174314389406263247_i64),1453202658103810377_i64,6299647219178655747_i64,2005194212947469739_i64,(-2721032774021535983_i64),6475016913985336608_i64,(-1832145702275347357_i64)];
_5 = _4;
_3 = (_5.0, _6.1, _6.2);
_6.0 = _3.0;
_8 = [(-2522_i16),8778_i16,(-15710_i16),16809_i16,20417_i16,5513_i16,29703_i16,2136_i16];
_7 = Adt46 { fld0: 73083484360679414679616419428234453687_i128,fld1: 295153898518410276598777690411876324754_u128,fld2: 37936_u16 };
_3.2 = _6.2 & _5.2;
_7.fld2 = !33711_u16;
_6.1 = [_3.2,_3.2,_3.2,_6.2,_5.2,_3.2,_3.2];
RET = _7.fld2 as f32;
Goto(bb1)
}
bb1 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb2 = {
_7.fld2 = 56815_u16;
_13 = !(-1829_i16);
_7 = Adt46 { fld0: (-95421365210191012335768088620815876650_i128),fld1: 264244460440944665768279673220588341132_u128,fld2: 6682_u16 };
_3.0 = _4.0;
_7.fld0 = 93535142509842737488443364032574776513_i128 >> _13;
_12 = [_13,_13,_13,_13,_13,_13,_13,_13];
_5.1 = [_5.2,_6.2,_3.2,_5.2,_5.2,_4.2,_5.2];
_5 = (_4.0, _6.1, _4.2);
_11.1 = _13 - _13;
_1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_13,_11.1,_11.1];
RET = _6.2 as f32;
_6.1 = [_4.2,_6.2,_3.2,_5.2,_5.2,_3.2,_6.2];
_15 = _7.fld2 as f64;
_5.2 = _6.2 ^ _4.2;
_8 = [_13,_13,_13,_11.1,_13,_13,_13,_11.1];
_5 = (_9, _6.1, _6.2);
_3.0 = _6.0;
_3 = (_9, _6.1, _5.2);
_4.0 = [(-702329531473348654_i64),1423308466704647783_i64,(-5018587249754800106_i64),2054897687510885690_i64,3334303183723540456_i64,1683981987296222300_i64,(-6856347061032411418_i64)];
_13 = !_11.1;
_13 = _11.1 & _11.1;
_18.2 = [5553235238037511387_i64,2759223730017181517_i64,6959089342127606156_i64,702760655527400914_i64,8146017255958164578_i64,6655642263425928899_i64,2927908124238338318_i64];
_3.1 = [_3.2,_6.2,_4.2,_4.2,_3.2,_5.2,_6.2];
_13 = _11.1 | _11.1;
Goto(bb3)
}
bb3 = {
_18.3 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0];
_6.1 = [_5.2,_4.2,_6.2,_5.2,_4.2,_5.2,_4.2];
_7.fld1 = 237970659115875455957335566411893362855_u128;
_18.1 = [false,false,false,true,false];
_7.fld0 = -106599705508394600268764901814755893808_i128;
_4.2 = _3.2 & _6.2;
_6 = (_18.2, _5.1, _5.2);
_4.2 = _6.2;
_7 = Adt46 { fld0: 163255175348513720828042301230618425921_i128,fld1: 140896360116676555906212053606093930133_u128,fld2: 7463_u16 };
_19 = RET;
_18.3 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0];
_6.0 = _18.2;
_2 = -9223372036854775807_isize;
_1 = [_13,_13,_11.1,_11.1,_11.1,_11.1,_13,_13];
_3 = (_4.0, _5.1, _6.2);
_15 = (-75_i8) as f64;
_11.1 = 275480608_i32 as i16;
_3.0 = [6909613033011896105_i64,1655452382738837679_i64,(-5520557088079872552_i64),(-800970628454436900_i64),8731455155665624316_i64,(-6688847677857234318_i64),(-1088393544819797514_i64)];
_20 = -_19;
_9 = [1242739301740314091_i64,3850000132405323227_i64,(-7887687834378113843_i64),4100067810424229070_i64,8167125570844634647_i64,4808652528966995424_i64,(-7416878527216078775_i64)];
match _7.fld0 {
0 => bb1,
163255175348513720828042301230618425921 => bb5,
_ => bb4
}
}
bb4 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb5 = {
_1 = _8;
_4.2 = _7.fld2 as u32;
_21 = Adt45::Variant3 { fld0: _3 };
_4 = (_3.0, Field::<([i64; 7], [u32; 7], u32)>(Variant(_21, 3), 0).1, _3.2);
_18.0 = [13461713583757024304_u64,586022531201506625_u64];
_19 = _20 + RET;
_5 = (_4.0, _4.1, _6.2);
SetDiscriminant(_21, 0);
_10 = -_2;
_15 = 145_u8 as f64;
_5.0 = [6977162666008941699_i64,7694416513061744810_i64,(-14208616108411723_i64),7274801845371320228_i64,(-7778102158727413232_i64),(-5607559108899190148_i64),6980412234291952562_i64];
_18.0 = [13712594035573187710_u64,2525676701348442832_u64];
_22 = core::ptr::addr_of!(place!(Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3)).1);
_8 = _1;
(*_22).3 = !_10;
_10 = _2;
(*_22).6 = 4_usize;
place!(Field::<[char; 1]>(Variant(_21, 0), 7)) = ['\u{beb9b}'];
match _7.fld2 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
7463 => bb12,
_ => bb11
}
}
bb6 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb7 = {
_18.3 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0];
_6.1 = [_5.2,_4.2,_6.2,_5.2,_4.2,_5.2,_4.2];
_7.fld1 = 237970659115875455957335566411893362855_u128;
_18.1 = [false,false,false,true,false];
_7.fld0 = -106599705508394600268764901814755893808_i128;
_4.2 = _3.2 & _6.2;
_6 = (_18.2, _5.1, _5.2);
_4.2 = _6.2;
_7 = Adt46 { fld0: 163255175348513720828042301230618425921_i128,fld1: 140896360116676555906212053606093930133_u128,fld2: 7463_u16 };
_19 = RET;
_18.3 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0];
_6.0 = _18.2;
_2 = -9223372036854775807_isize;
_1 = [_13,_13,_11.1,_11.1,_11.1,_11.1,_13,_13];
_3 = (_4.0, _5.1, _6.2);
_15 = (-75_i8) as f64;
_11.1 = 275480608_i32 as i16;
_3.0 = [6909613033011896105_i64,1655452382738837679_i64,(-5520557088079872552_i64),(-800970628454436900_i64),8731455155665624316_i64,(-6688847677857234318_i64),(-1088393544819797514_i64)];
_20 = -_19;
_9 = [1242739301740314091_i64,3850000132405323227_i64,(-7887687834378113843_i64),4100067810424229070_i64,8167125570844634647_i64,4808652528966995424_i64,(-7416878527216078775_i64)];
match _7.fld0 {
0 => bb1,
163255175348513720828042301230618425921 => bb5,
_ => bb4
}
}
bb8 = {
_7.fld2 = 56815_u16;
_13 = !(-1829_i16);
_7 = Adt46 { fld0: (-95421365210191012335768088620815876650_i128),fld1: 264244460440944665768279673220588341132_u128,fld2: 6682_u16 };
_3.0 = _4.0;
_7.fld0 = 93535142509842737488443364032574776513_i128 >> _13;
_12 = [_13,_13,_13,_13,_13,_13,_13,_13];
_5.1 = [_5.2,_6.2,_3.2,_5.2,_5.2,_4.2,_5.2];
_5 = (_4.0, _6.1, _4.2);
_11.1 = _13 - _13;
_1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_13,_11.1,_11.1];
RET = _6.2 as f32;
_6.1 = [_4.2,_6.2,_3.2,_5.2,_5.2,_3.2,_6.2];
_15 = _7.fld2 as f64;
_5.2 = _6.2 ^ _4.2;
_8 = [_13,_13,_13,_11.1,_13,_13,_13,_11.1];
_5 = (_9, _6.1, _6.2);
_3.0 = _6.0;
_3 = (_9, _6.1, _5.2);
_4.0 = [(-702329531473348654_i64),1423308466704647783_i64,(-5018587249754800106_i64),2054897687510885690_i64,3334303183723540456_i64,1683981987296222300_i64,(-6856347061032411418_i64)];
_13 = !_11.1;
_13 = _11.1 & _11.1;
_18.2 = [5553235238037511387_i64,2759223730017181517_i64,6959089342127606156_i64,702760655527400914_i64,8146017255958164578_i64,6655642263425928899_i64,2927908124238338318_i64];
_3.1 = [_3.2,_6.2,_4.2,_4.2,_3.2,_5.2,_6.2];
_13 = _11.1 | _11.1;
Goto(bb3)
}
bb9 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<[u64; 2]>(Variant(_21, 0), 1)) = _18.0;
_14 = _7.fld1 != _7.fld1;
_18.2 = _9;
place!(Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6)).1 = [_14,_14,_14,_14,_14];
place!(Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6)).2 = [(-1745121420575546362_i64),(-8083364967538098975_i64),(-6981642765303086134_i64),(-5501129948112977988_i64),8115431065298600825_i64,(-2389457693794742122_i64),632763579285941774_i64];
(*_22).2 = _4.2;
place!(Field::<[u32; 7]>(Variant(_21, 0), 2)) = _4.1;
_28 = _15;
_7.fld1 = 190421090649337626844946418335218252398_u128;
place!(Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6)).4 = 100_i8 as f32;
place!(Field::<[char; 1]>(Variant(_21, 0), 7)) = ['\u{532bd}'];
place!(Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3)).1.4 = 97_i8 << _11.1;
_19 = -Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6).4;
_31 = (*_22).3;
_16 = 3977744567261478613_i64 as f32;
_11.0 = [Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3).1.2,(*_22).2];
_3 = _4;
place!(Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3)).1.6 = _15 as usize;
(*_22).6 = (*_22).2 as usize;
place!(Field::<[u32; 7]>(Variant(_21, 0), 2)) = _6.1;
place!(Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6)).3 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0];
_14 = false;
(*_22).2 = _7.fld0 as u32;
Goto(bb13)
}
bb13 = {
RET = (*_22).3 as f32;
RET = _20 * Field::<([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32)>(Variant(_21, 0), 6).4;
_18.4 = _16;
match _7.fld1 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
190421090649337626844946418335218252398 => bb20,
_ => bb19
}
}
bb14 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb17 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb18 = {
_4.0 = [(-2873238307367871529_i64),(-2926202924050159380_i64),(-6747815652319407171_i64),2022088675142186911_i64,(-629593364767491219_i64),(-5457107194484363527_i64),7331138264288990022_i64];
_5.1 = [_3.2,_5.2,_4.2,_5.2,_5.2,_3.2,_3.2];
_7.fld0 = 29853952063261780497660453151712112180_i128 & 100429219359906712091852559304394220269_i128;
_6 = _4;
_6.1 = [_6.2,_4.2,_3.2,_6.2,_6.2,_5.2,_4.2];
_2 = (-99_isize);
_3 = (_5.0, _4.1, _4.2);
_6.1 = _5.1;
RET = _2 as f32;
_5.0 = [3298368335210658864_i64,(-3120859621877160279_i64),(-7687277328076803531_i64),2921813819183253364_i64,8997659330021026987_i64,4951633865973673488_i64,(-8431964731895337930_i64)];
_2 = 9223372036854775807_isize;
_4.0 = [(-6861880304677375214_i64),7461983076483867931_i64,8969775944567120813_i64,2895890407007877342_i64,6592397481089242642_i64,4295695327729411026_i64,4997211732043595365_i64];
_5.2 = !_3.2;
_11.0 = [_3.2,_4.2];
_6 = (_5.0, _5.1, _5.2);
_8 = [4789_i16,(-16299_i16),30005_i16,(-16026_i16),11877_i16,(-2845_i16),21830_i16,(-14311_i16)];
_3 = _6;
_2 = (-93_isize) * 9223372036854775807_isize;
_8 = [6565_i16,(-9073_i16),19386_i16,14883_i16,8763_i16,6153_i16,(-14917_i16),6189_i16];
_7.fld2 = 53516_u16 << _3.2;
_4.0 = [4371007549543716559_i64,5034158733296709050_i64,7381256230806509547_i64,6224282305996248564_i64,4751480779539432784_i64,(-2528803661139404093_i64),5463823641297879251_i64];
_6 = (_3.0, _3.1, _5.2);
RET = (-1962570894_i32) as f32;
_3.0 = _9;
Goto(bb2)
}
bb19 = {
_1 = _8;
_4.2 = _7.fld2 as u32;
_21 = Adt45::Variant3 { fld0: _3 };
_4 = (_3.0, Field::<([i64; 7], [u32; 7], u32)>(Variant(_21, 3), 0).1, _3.2);
_18.0 = [13461713583757024304_u64,586022531201506625_u64];
_19 = _20 + RET;
_5 = (_4.0, _4.1, _6.2);
SetDiscriminant(_21, 0);
_10 = -_2;
_15 = 145_u8 as f64;
_5.0 = [6977162666008941699_i64,7694416513061744810_i64,(-14208616108411723_i64),7274801845371320228_i64,(-7778102158727413232_i64),(-5607559108899190148_i64),6980412234291952562_i64];
_18.0 = [13712594035573187710_u64,2525676701348442832_u64];
_22 = core::ptr::addr_of!(place!(Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3)).1);
_8 = _1;
(*_22).3 = !_10;
_10 = _2;
(*_22).6 = 4_usize;
place!(Field::<[char; 1]>(Variant(_21, 0), 7)) = ['\u{beb9b}'];
match _7.fld2 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
7463 => bb12,
_ => bb11
}
}
bb20 = {
_25 = _31 + Field::<(isize, (u64, u128, u32, isize, i8, i16, usize), u8)>(Variant(_21, 0), 3).1.3;
(*_22).3 = -_10;
(*_22).2 = _5.2 << (*_22).3;
_12 = [_13,_13,_13,_13,_13,_13,_13,_13];
_2 = _7.fld0 as isize;
_6.2 = (*_22).2 + _3.2;
(*_22).1 = _7.fld1;
_32.2 = _4.2;
Goto(bb21)
}
bb21 = {
Call(_33 = dump_var(16_usize, 4_usize, Move(_4), 9_usize, Move(_9), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(16_usize, 6_usize, Move(_6), 10_usize, Move(_10), 8_usize, Move(_8), 34_usize, _34), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: usize,mut _2: Adt53,mut _3: isize,mut _4: *const (u64, u128, u32, isize, i8, i16, usize),mut _5: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize)),mut _6: *const (u64, u128, u32, isize, i8, i16, usize)) -> ([i64; 7], [u32; 7], u32) {
mir! {
type RET = ([i64; 7], [u32; 7], u32);
let _7: usize;
let _8: u16;
let _9: isize;
let _10: Adt53;
let _11: i16;
let _12: f32;
let _13: usize;
let _14: u8;
let _15: (i8, u64);
let _16: (*const i128, [i16; 6], u64);
let _17: isize;
let _18: ();
let _19: ();
{
(*_4).2 = !_5.0.2;
RET.0 = _2.fld2.0.0;
_6 = core::ptr::addr_of!((*_4));
RET = (_2.fld2.0.0, _2.fld2.0.1, (*_6).2);
(*_6).1 = 86262005180393520442976492962728543895_u128 * 121020892719226664278813892267427290451_u128;
_8 = (*_6).0 as u16;
(*_4) = (13663471482230120914_u64, 187908052555140468279893994735806093870_u128, _2.fld2.0.2, _3, 22_i8, (-14835_i16), _1);
_10.fld2 = _2.fld2;
(*_6).4 = 148271150934771544003000087545350483111_i128 as i8;
_5.0.2 = (*_4).2 << (*_4).3;
(*_6).4 = -(-79_i8);
(*_6).1 = !227213994290023861491865318187968825902_u128;
(*_4) = (11041463740710789313_u64, 148270073665726230349590056577836088091_u128, _5.0.2, _3, 8_i8, 22575_i16, _1);
RET.2 = !(*_4).2;
RET.0 = [(-3425121266918415824_i64),(-7564558816308683045_i64),(-7454125469730649972_i64),(-9086056276100845069_i64),34472163653065183_i64,(-9198155075362297453_i64),968017441047317464_i64];
_5.0.0 = [388426558187911404_i64,(-1364626673579762021_i64),(-6086877358403474446_i64),(-6459352751113738563_i64),2969761168504263641_i64,(-103135545540450637_i64),7887145563569078978_i64];
RET.2 = (*_6).4 as u32;
(*_6).4 = -(-37_i8);
_5.0.1 = RET.1;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(17_usize, 8_usize, Move(_8), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(9846965556101541071_u64), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(116_i8), std::hint::black_box(20031_i16), std::hint::black_box((-1355388330_i32)), std::hint::black_box(3743017358_u32), std::hint::black_box((-90744533735138472263533832918072306834_i128)), std::hint::black_box(164220725942874152608540212499808868270_u128), std::hint::black_box(169_u8));
                
            }
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: (u8, u64, f32),
fld2: f64,
fld3: i8,
fld4: [i128; 4],
fld5: i32,

},
Variant1{
fld0: i16,
fld1: (i8, u64),
fld2: isize,

},
Variant2{
fld0: *mut [i128; 4],
fld1: char,
fld2: u64,
fld3: u16,
fld4: (i8, u64),
fld5: *const i128,

},
Variant3{
fld0: ([i64; 7], [u32; 7], u32),
fld1: [i64; 7],
fld2: (i8, u64),
fld3: (*const i128, char, isize, char, [u32; 5], usize),
fld4: [bool; 5],
fld5: i32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u32; 7],
fld1: [u32; 2],
fld2: *const (u64, u128, u32, isize, i8, i16, usize),
fld3: u64,
fld4: (*const i128, [i16; 6], u64),
fld5: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32),

},
Variant1{
fld0: f32,
fld1: i32,
fld2: (i8, u64),

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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *const (u64, u128, u32, isize, i8, i16, usize),
fld1: (*const i128, [i16; 6], u64),
fld2: i32,

},
Variant1{
fld0: (isize, (u64, u128, u32, isize, i8, i16, usize), u8),
fld1: [u32; 2],
fld2: i32,
fld3: *mut [i128; 4],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u128,
fld1: [u64; 2],
fld2: [u32; 7],
fld3: (isize, (u64, u128, u32, isize, i8, i16, usize), u8),
fld4: f64,
fld5: [u32; 5],
fld6: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32),
fld7: [char; 1],

},
Variant1{
fld0: [i64; 7],
fld1: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize)),
fld2: isize,
fld3: [bool; 5],
fld4: [u32; 7],
fld5: [u32; 2],

},
Variant2{
fld0: bool,
fld1: i16,

},
Variant3{
fld0: ([i64; 7], [u32; 7], u32),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: i128,
fld1: u128,
fld2: u16,
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt45,
fld1: usize,

},
Variant1{
fld0: [bool; 5],
fld1: [char; 1],
fld2: ([u32; 2], i16),
fld3: (u64, u128, u32, isize, i8, i16, usize),
fld4: i16,
fld5: i32,
fld6: [i128; 4],
fld7: Adt45,

},
Variant2{
fld0: [i64; 7],
fld1: char,
fld2: u8,
fld3: u16,
fld4: i16,
fld5: (*const i128, char, isize, char, [u32; 5], usize),
fld6: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32),

},
Variant3{
fld0: bool,
fld1: u16,
fld2: (i8, u64),
fld3: Adt46,
fld4: Adt42,
fld5: ([i64; 7], [u32; 7], u32),
fld6: [u32; 7],
fld7: i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i16; 6],
fld1: char,
fld2: f32,
fld3: f64,
fld4: [i128; 4],
fld5: i32,
fld6: *const i128,

},
Variant1{
fld0: Adt43,

},
Variant2{
fld0: ([u64; 2], [bool; 5], [i64; 7], [i128; 4], f32),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u64,
fld1: i128,
fld2: isize,
fld3: [u32; 2],

},
Variant1{
fld0: *mut [i128; 4],
fld1: Adt42,
fld2: [u64; 2],
fld3: Adt44,
fld4: i16,
fld5: u8,
fld6: [i16; 8],
fld7: *const i128,

},
Variant2{
fld0: (i8, u64),
fld1: u16,
fld2: [i16; 8],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (u8, u64, f32),

},
Variant1{
fld0: (u8, u64, f32),
fld1: i64,
fld2: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize)),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i32,
fld1: u16,

},
Variant1{
fld0: f32,
fld1: Adt44,
fld2: i128,
fld3: Adt42,

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
fld0: (*const i128, char, isize, char, [u32; 5], usize),
fld1: u8,

},
Variant1{
fld0: Adt49,
fld1: ([u32; 2], i16),
fld2: [u32; 5],
fld3: usize,
fld4: Adt46,
fld5: (i8, u64),
fld6: (*const i128, char, isize, char, [u32; 5], usize),
fld7: Adt50,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: *const (u64, u128, u32, isize, i8, i16, usize),
fld1: [i16; 6],
fld2: (([i64; 7], [u32; 7], u32), [u32; 5], *const (u64, u128, u32, isize, i8, i16, usize)),
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt49,
fld1: u128,
fld2: [u32; 5],
fld3: i8,
fld4: [u32; 7],
fld5: f32,

},
Variant1{
fld0: ([u32; 2], i16),
fld1: char,
fld2: (i8, u64),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: u8,
fld1: Adt48,

},
Variant1{
fld0: [i16; 6],
fld1: *const (u64, u128, u32, isize, i8, i16, usize),
fld2: u16,
fld3: ([u32; 2], i16),
fld4: Adt43,
fld5: [char; 1],
fld6: Adt48,
fld7: Adt44,

},
Variant2{
fld0: [u64; 2],

},
Variant3{
fld0: bool,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt46,
fld1: Adt45,
fld2: Adt50,
fld3: Adt54,
fld4: [bool; 5],

},
Variant1{
fld0: [bool; 5],
fld1: [i16; 8],
fld2: u32,
fld3: u16,
fld4: i16,
fld5: Adt55,
fld6: ([i64; 7], [u32; 7], u32),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: f64,
fld1: (i8, u64),
fld2: (u64, u128, u32, isize, i8, i16, usize),

},
Variant1{
fld0: [i128; 4],
fld1: *mut [i128; 4],
fld2: u16,
fld3: i8,
fld4: i16,
fld5: Adt51,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
fld0: bool,
fld1: Adt45,
fld2: (u8, u64, f32),
fld3: Adt51,
fld4: [i16; 6],

},
Variant1{
fld0: usize,
fld1: *mut [i128; 4],
fld2: u16,
fld3: Adt48,
fld4: u64,

}}

