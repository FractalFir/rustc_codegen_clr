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
            printf(c"%i".as_ptr(),*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf(c"[".as_ptr());
            for b in self{
                b.printf_debug();
                printf(c",".as_ptr());
            }
            printf(c"]".as_ptr());
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf(c"%lx%lx".as_ptr(), (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf(c"true".as_ptr());
            }
            else{
                printf(c"false".as_ptr());
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf(c"()".as_ptr());
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",)".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c",".as_ptr());
            self.11.printf_debug();
            printf(c")".as_ptr());
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
            printf(c"fn%u:_%u = ".as_ptr(),f,var0);
            val0.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var1);
            val1.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var2);
            val2.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var3);
            val3.printf_debug();
            printf(c"\n".as_ptr());
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: i64,mut _2: u32,mut _3: isize,mut _4: u8,mut _5: usize) -> isize {
mir! {
type RET = isize;
let _6: Adt43;
let _7: (u16, i64, i32, u64);
let _8: (i64, i64, u8, u128, i8);
let _9: Adt45;
let _10: f32;
let _11: Adt52;
let _12: Adt55;
let _13: f64;
let _14: Adt41;
let _15: &'static (u16, i64, i32, u64);
let _16: isize;
let _17: i16;
let _18: isize;
let _19: Adt55;
let _20: i128;
let _21: [i64; 7];
let _22: Adt47;
let _23: Adt45;
let _24: (i8,);
let _25: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]);
let _26: usize;
let _27: [u8; 7];
let _28: f64;
let _29: [i8; 4];
let _30: [u32; 6];
let _31: ();
let _32: ();
{
_3 = !30_isize;
_4 = 79_u8;
_7.3 = 6751957051114642142_u64;
_7 = (45476_u16, (-4990909918766583459_i64), 217582337_i32, 14193941808121129264_u64);
_7.2 = 884210415_i32;
RET = 113_i8 as isize;
_7.3 = _7.1 as u64;
_5 = 2160604373506661497_usize >> _7.1;
match _7.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
45476 => bb7,
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
_1 = _7.1;
RET = !_3;
_2 = _7.3 as u32;
_1 = _7.1 + _7.1;
_7.3 = 15443246727752385898_u64;
_7.0 = !29474_u16;
_9.fld6 = [_1,_1,_7.1,_7.1,_7.1,_1,_1];
_9.fld4 = (false,);
_5 = '\u{b7a4a}' as usize;
_9.fld2.1 = _2 | _2;
_9.fld7 = _7.1 as f32;
_9.fld2.2 = [_7.3];
_7.1 = _1 | _1;
_8.2 = _7.0 as u8;
_7.0 = 29273_u16 | 22217_u16;
_5 = !6_usize;
_8 = (_1, _7.1, _4, 280640100191312316257081491519375705324_u128, (-3_i8));
_1 = _7.1 | _8.0;
_8.3 = !74449476315530004482345990452601850942_u128;
_9.fld2.0 = [_9.fld2.1,_9.fld2.1,_9.fld2.1,_9.fld2.1,_9.fld2.1,_2];
_9.fld2.0 = [_9.fld2.1,_2,_2,_9.fld2.1,_9.fld2.1,_2];
RET = _3;
_9.fld3 = _7.3;
_8.0 = !_1;
_8.2 = _4;
_8.3 = 194940391152812581250349653999641528810_u128 >> _8.1;
_1 = _8.0 << _7.1;
match _8.4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211453 => bb8,
_ => bb6
}
}
bb8 = {
_9.fld2.2 = [_9.fld3];
_2 = !_9.fld2.1;
RET = -_3;
_7 = (5411_u16, _1, 1880383570_i32, _9.fld3);
_9.fld2.2 = [_7.3];
_11.fld0 = _8.1 as u8;
_9.fld1 = _5 * _5;
_9.fld2.0 = [_2,_2,_9.fld2.1,_2,_9.fld2.1,_2];
_3 = _9.fld4.0 as isize;
_7.0 = !28247_u16;
_1 = !_7.1;
_9.fld4.0 = _1 > _7.1;
_9.fld5 = _7.2;
_9.fld3 = _9.fld2.1 as u64;
_13 = _7.0 as f64;
_4 = _11.fld0 * _11.fld0;
_4 = !_11.fld0;
_7 = (19599_u16, _8.0, _9.fld5, _9.fld3);
_8.1 = !_8.0;
_8 = (_7.1, _1, _4, 168333784510614541995002243017086957912_u128, (-20_i8));
_4 = _8.2 << _8.4;
_9.fld2.2 = [_7.3];
_9.fld5 = -_7.2;
Call(RET = fn1(_1, _8.4, _7.0, _8.4, _9.fld4, _7.0, _7.1, _4, _7.2, _4, _4, _7.2, _8, _7.0, _8.1, _8.4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = _9.fld7;
_13 = 9521221646416959094106487960581359254_i128 as f64;
_9.fld4.0 = _8.2 == _4;
_8 = (_1, _1, _4, 108311972729552690222228361122026075882_u128, 58_i8);
_9.fld4 = (true,);
_9.fld3 = _7.3;
_13 = RET as f64;
_9.fld0 = Adt43::Variant1 { fld0: _9.fld4,fld1: _8,fld2: _9.fld6 };
_8.2 = _4;
_9.fld4.0 = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0 >= _8.0;
_7 = (40210_u16, Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0, _9.fld5, _9.fld3);
_7.0 = _9.fld4.0 as u16;
_8 = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1);
_9.fld4 = (Field::<(bool,)>(Variant(_9.fld0, 1), 0).0,);
_4 = _13 as u8;
_9.fld2.0 = [_2,_9.fld2.1,_9.fld2.1,_2,_9.fld2.1,_2];
RET = _3 >> _8.3;
_6 = Adt43::Variant1 { fld0: _9.fld4,fld1: Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1),fld2: _9.fld6 };
_8 = Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1);
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)).3 = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).3 % Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).3;
_4 = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).2;
_11.fld0 = _7.2 as u8;
_9.fld4 = Field::<(bool,)>(Variant(_9.fld0, 1), 0);
_9.fld5 = _7.2 | _7.2;
place!(Field::<[i64; 7]>(Variant(_6, 1), 2)) = [Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0,_8.1,_8.1,Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0,Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0,Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).0,_8.0];
_7.0 = '\u{2b348}' as u16;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)).1 = Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).0;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1)).2 = _8.2 | _4;
RET = _3;
Goto(bb10)
}
bb10 = {
_9.fld5 = -_7.2;
RET = -_3;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1)).3 = _11.fld0 as u128;
_8.3 = !Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).3;
_7 = (27550_u16, _1, _9.fld5, _9.fld3);
_9.fld2.0 = [_2,_9.fld2.1,_9.fld2.1,_2,_9.fld2.1,_2];
place!(Field::<[i64; 7]>(Variant(_6, 1), 2)) = [Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).0,_7.1,_1,Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).0,_7.1,_8.1,Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).0];
_7.3 = !_9.fld3;
_13 = Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).1 as f64;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)) = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1);
_3 = -RET;
_16 = RET + _3;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)) = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1);
_2 = !_9.fld2.1;
_15 = &_7;
_9.fld4 = (Field::<(bool,)>(Variant(_6, 1), 0).0,);
RET = !_3;
place!(Field::<(bool,)>(Variant(_6, 1), 0)).0 = Field::<(bool,)>(Variant(_9.fld0, 1), 0).0 ^ Field::<(bool,)>(Variant(_9.fld0, 1), 0).0;
_7.1 = _1;
_16 = _7.3 as isize;
_17 = -(-8446_i16);
SetDiscriminant(_6, 0);
_9.fld3 = !_7.3;
SetDiscriminant(_9.fld0, 0);
place!(Field::<bool>(Variant(_6, 0), 0)) = _9.fld4.0;
Goto(bb11)
}
bb11 = {
place!(Field::<[u8; 7]>(Variant(_9.fld0, 0), 1)) = [_8.2,_4,_11.fld0,_4,_4,_4,_4];
place!(Field::<(u8,)>(Variant(_6, 0), 2)) = (_4,);
_8 = (_7.1, _7.1, _4, 244074675041775152984845595148766694347_u128, (-18_i8));
_7 = (52413_u16, _8.0, _9.fld5, _9.fld3);
_20 = (-89169191715046352834066717805843714290_i128) << Field::<(u8,)>(Variant(_6, 0), 2).0;
_1 = _8.0;
_7.1 = _8.1 - _8.1;
_7.3 = _9.fld3;
_16 = _8.4 as isize;
place!(Field::<i8>(Variant(_9.fld0, 0), 3)) = _8.4;
_2 = !_9.fld2.1;
_9.fld1 = _5 | _5;
_15 = &_7;
place!(Field::<i16>(Variant(_9.fld0, 0), 4)) = -_17;
_7.1 = _13 as i64;
place!(Field::<[u8; 7]>(Variant(_6, 0), 1)) = Field::<[u8; 7]>(Variant(_9.fld0, 0), 1);
_2 = _4 as u32;
place!(Field::<([u32; 6], u32, [u64; 1])>(Variant(_9.fld0, 0), 5)) = _9.fld2;
place!(Field::<u32>(Variant(_6, 0), 7)) = _2 ^ _2;
_22.fld3 = _8.4;
_22.fld3 = Field::<i8>(Variant(_9.fld0, 0), 3) << _16;
place!(Field::<i8>(Variant(_6, 0), 3)) = Field::<i8>(Variant(_9.fld0, 0), 3);
_22.fld3 = Field::<i8>(Variant(_6, 0), 3);
_23.fld2.0 = [Field::<u32>(Variant(_6, 0), 7),Field::<u32>(Variant(_6, 0), 7),Field::<u32>(Variant(_6, 0), 7),Field::<u32>(Variant(_6, 0), 7),Field::<u32>(Variant(_6, 0), 7),_2];
_9.fld2.0 = Field::<([u32; 6], u32, [u64; 1])>(Variant(_9.fld0, 0), 5).0;
place!(Field::<u32>(Variant(_9.fld0, 0), 7)) = !_2;
match _8.4 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb7,
340282366920938463463374607431768211438 => bb12,
_ => bb8
}
}
bb12 = {
_23.fld5 = -_9.fld5;
place!(Field::<(u8,)>(Variant(_6, 0), 2)).0 = _8.2;
place!(Field::<[u8; 7]>(Variant(_9.fld0, 0), 1)) = [_8.2,_4,_8.2,_4,_4,_4,_8.2];
_5 = _9.fld1;
place!(Field::<u32>(Variant(_9.fld0, 0), 7)) = _7.0 as u32;
_23.fld2.2 = [_9.fld3];
_22.fld0 = Adt39::Variant2 { fld0: _9.fld2.2,fld1: _8.4 };
_15 = &_7;
_6 = Adt43::Variant1 { fld0: _9.fld4,fld1: _8,fld2: _9.fld6 };
_23 = Adt45 { fld0: _6,fld1: _9.fld1,fld2: _9.fld2,fld3: (*_15).3,fld4: Field::<(bool,)>(Variant(_6, 1), 0),fld5: (*_15).2,fld6: _9.fld6,fld7: _9.fld7 };
_9.fld7 = _23.fld7 - _23.fld7;
_2 = Field::<([u32; 6], u32, [u64; 1])>(Variant(_9.fld0, 0), 5).1;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)).1 = _7.1 << _16;
_9.fld6 = [_1,(*_15).1,Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).0,Field::<(i64, i64, u8, u128, i8)>(Variant(_23.fld0, 1), 1).1,_8.0,Field::<(i64, i64, u8, u128, i8)>(Variant(_23.fld0, 1), 1).0,_7.1];
_22.fld4 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_9.fld0, 0), 4)));
SetDiscriminant(_23.fld0, 0);
RET = _9.fld3 as isize;
_22.fld1 = [Field::<u32>(Variant(_9.fld0, 0), 7),Field::<u32>(Variant(_9.fld0, 0), 7),Field::<u32>(Variant(_9.fld0, 0), 7),Field::<u32>(Variant(_9.fld0, 0), 7),Field::<u32>(Variant(_9.fld0, 0), 7),Field::<u32>(Variant(_9.fld0, 0), 7)];
_5 = Field::<u32>(Variant(_9.fld0, 0), 7) as usize;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)).1 = -_1;
place!(Field::<[u64; 1]>(Variant(_22.fld0, 2), 0)) = [_9.fld3];
SetDiscriminant(_6, 1);
_9.fld4 = (_23.fld4.0,);
place!(Field::<f32>(Variant(_23.fld0, 0), 6)) = -_9.fld7;
match (*_15).0 {
0 => bb11,
1 => bb10,
2 => bb3,
3 => bb7,
4 => bb5,
52413 => bb13,
_ => bb6
}
}
bb13 = {
_1 = _23.fld4.0 as i64;
place!(Field::<(u8,)>(Variant(_23.fld0, 0), 2)) = (_4,);
_25.0 = (_22.fld3,);
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)).4 = Field::<i8>(Variant(_9.fld0, 0), 3) >> _8.3;
place!(Field::<bool>(Variant(_23.fld0, 0), 0)) = !_23.fld4.0;
_21 = [(*_15).1,(*_15).1,_8.1,_7.1,(*_15).1,_7.1,(*_15).1];
place!(Field::<u32>(Variant(_9.fld0, 0), 7)) = _2;
_15 = &_7;
_9.fld0 = Adt43::Variant1 { fld0: _9.fld4,fld1: _8,fld2: _21 };
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1)).0 = _8.0 * (*_15).1;
_9.fld1 = _5 + _5;
_7 = (36880_u16, _8.0, _9.fld5, _23.fld3);
_25.1 = _22.fld3 as i32;
place!(Field::<i16>(Variant(_23.fld0, 0), 4)) = -_17;
place!(Field::<u32>(Variant(_23.fld0, 0), 7)) = _23.fld2.1;
_9.fld7 = -_10;
Goto(bb14)
}
bb14 = {
_9.fld1 = !_5;
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1)) = (_8.0, _8.0, Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).2, _8.3, _22.fld3);
_15 = &_7;
place!(Field::<(u8,)>(Variant(_23.fld0, 0), 2)).0 = _4 >> _8.1;
place!(Field::<i8>(Variant(_23.fld0, 0), 3)) = _8.4;
_23.fld3 = (*_15).3 ^ (*_15).3;
_9.fld3 = _23.fld3 * _7.3;
place!(Field::<(bool,)>(Variant(_6, 1), 0)).0 = _23.fld4.0;
_25.6 = [Field::<(u8,)>(Variant(_23.fld0, 0), 2).0,_8.2,Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).2,_8.2,_4,Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).2,_8.2];
_11 = Adt52 { fld0: Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).2 };
place!(Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1)) = (Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).1, (*_15).1, Field::<(u8,)>(Variant(_23.fld0, 0), 2).0, Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).3, _22.fld3);
_11.fld0 = !Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).2;
_23.fld3 = Field::<(i64, i64, u8, u128, i8)>(Variant(_9.fld0, 1), 1).3 as u64;
_4 = _8.2 ^ _8.2;
_23.fld2 = _9.fld2;
_25.0 = (Field::<(i64, i64, u8, u128, i8)>(Variant(_6, 1), 1).4,);
_18 = _16;
_23.fld0 = _9.fld0;
_22.fld3 = -Field::<(i64, i64, u8, u128, i8)>(Variant(_23.fld0, 1), 1).4;
place!(Field::<(bool,)>(Variant(_6, 1), 0)) = (Field::<(bool,)>(Variant(_9.fld0, 1), 0).0,);
SetDiscriminant(_23.fld0, 1);
_3 = _18;
_22.fld5 = _7.0 as i32;
_6 = _9.fld0;
_29 = [Field::<i8>(Variant(_22.fld0, 2), 1),_22.fld3,_22.fld3,Field::<i8>(Variant(_22.fld0, 2), 1)];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 20_usize, Move(_20), 1_usize, Move(_1), 4_usize, Move(_4), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 2_usize, Move(_2), 7_usize, Move(_7), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i64,mut _2: i8,mut _3: u16,mut _4: i8,mut _5: (bool,),mut _6: u16,mut _7: i64,mut _8: u8,mut _9: i32,mut _10: u8,mut _11: u8,mut _12: i32,mut _13: (i64, i64, u8, u128, i8),mut _14: u16,mut _15: i64,mut _16: i8) -> isize {
mir! {
type RET = isize;
let _17: u64;
let _18: f64;
let _19: Adt54;
let _20: i16;
let _21: char;
let _22: Adt53;
let _23: (&'static (u16, i64, i32, u64), [i64; 7], [u32; 5]);
let _24: usize;
let _25: Adt44;
let _26: isize;
let _27: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8));
let _28: [i64; 7];
let _29: [u32; 6];
let _30: *mut i16;
let _31: f32;
let _32: isize;
let _33: Adt44;
let _34: Adt42;
let _35: (i64, i64, u8, u128, i8);
let _36: f32;
let _37: bool;
let _38: u128;
let _39: char;
let _40: Adt46;
let _41: [u8; 7];
let _42: i128;
let _43: f32;
let _44: Adt47;
let _45: i128;
let _46: [i8; 8];
let _47: Adt43;
let _48: ();
let _49: ();
{
_13.2 = _10 ^ _10;
_9 = !_12;
RET = _11 as isize;
_4 = -_13.4;
_11 = _10 + _10;
_11 = !_13.2;
_5.0 = true;
_15 = _5.0 as i64;
_16 = -_2;
_3 = _14;
_7 = !_13.0;
_9 = -_12;
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_16 = _2 | _13.4;
_5 = (true,);
_13 = (_7, _7, _8, 321014280047872416029793091468714896061_u128, _16);
_4 = _16 - _13.4;
_9 = !_12;
_11 = _9 as u8;
_16 = -_4;
_4 = _16;
_11 = !_8;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
1880383570 => bb5,
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
_3 = _14 ^ _6;
_14 = _6 / _6;
_2 = _13.3 as i8;
RET = !(-9223372036854775808_isize);
_7 = _6 as i64;
_16 = _4 & _4;
_9 = _12 & _12;
_21 = '\u{a28e1}';
_13.3 = !13460041630324093081065429740111603114_u128;
_2 = 5501274382974893885629405409107847307_i128 as i8;
_6 = 15499377971799471416_u64 as u16;
Goto(bb6)
}
bb6 = {
_23.2 = [2154113938_u32,3998851647_u32,2366159566_u32,2088879575_u32,1915180584_u32];
_4 = -_16;
_12 = _13.2 as i32;
_24 = 2_usize;
_13.4 = _4;
_23.1[_24] = _13.1 + _13.0;
_10 = !_13.2;
_16 = !_4;
match _24 {
0 => bb1,
1 => bb5,
2 => bb7,
_ => bb4
}
}
bb7 = {
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_13.4 = _4 - _16;
RET = _13.3 as isize;
_23.1 = [_7,_1,_7,_7,_7,_7,_7];
_16 = RET as i8;
_23.2[_24] = 3482739854_u32;
_2 = 14220531529528539587_u64 as i8;
_8 = !_10;
_11 = _13.2;
_16 = _4 & _4;
_2 = _16;
_23.1 = [_1,_7,_7,_7,_1,_13.0,_7];
_24 = 15500669075569236878_usize * 2138107584935339784_usize;
_23.2 = [2828516019_u32,1924471009_u32,505216937_u32,3087174500_u32,3580679086_u32];
_12 = !_9;
_23.1 = [_7,_7,_13.0,_1,_1,_13.0,_7];
RET = -107_isize;
_13.4 = -_16;
_3 = !_14;
_21 = '\u{278ef}';
_10 = _8;
_14 = !_3;
_13.1 = _7;
_13.3 = 139436327791682017348971597691456951581_u128 * 130085638585743072443459447872876664176_u128;
Goto(bb8)
}
bb8 = {
_9 = _12 >> _3;
_13.4 = !_4;
_13.2 = _10;
RET = -(-9223372036854775808_isize);
_26 = RET + RET;
_6 = !_14;
_2 = -_4;
_13.3 = !230545160250738752490026288258670076597_u128;
_2 = -_4;
_27.1 = !_13.3;
_27.0 = (_7, _13.0, _11, _27.1, _4);
_18 = _27.1 as f64;
_20 = (-27107_i16);
_13.1 = _7 >> _13.2;
_27.0.3 = 67413109794508872890054234719669369913_i128 as u128;
_4 = -_27.0.4;
_27.0 = (_7, _7, _13.2, _13.3, _4);
_27.3.1 = !_1;
_5 = (false,);
_13.1 = -_27.0.1;
RET = !_26;
_27.2 = _3 << _4;
_27.1 = !_27.0.3;
_17 = 3840884348_u32 as u64;
_29 = [4097457050_u32,2777408653_u32,2930272029_u32,3849985253_u32,1420407774_u32,1834614399_u32];
_27.3.3 = !_27.0.3;
Call(_8 = fn2(_13.0, _4, _27.2, _27.0, _23.1, _27.0.2, _23.1, _27.0.1, _27.2, _14), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = _6 as f64;
_27.3 = _27.0;
_15 = !_1;
_27.3.2 = _27.0.2;
_24 = 7287997914195136972_usize;
_18 = _9 as f64;
_27.2 = _21 as u16;
_18 = _26 as f64;
_27.0.2 = !_10;
_21 = '\u{98191}';
_27.3.1 = _7 & _27.0.0;
_28 = [_15,_27.0.0,_27.3.1,_15,_27.3.1,_13.0,_27.3.1];
_28 = [_27.0.0,_1,_13.1,_13.1,_15,_27.0.0,_27.3.0];
_36 = _20 as f32;
_35.0 = _27.3.1;
match _24 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
7287997914195136972 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_13.4 = _4 - _16;
RET = _13.3 as isize;
_23.1 = [_7,_1,_7,_7,_7,_7,_7];
_16 = RET as i8;
_23.2[_24] = 3482739854_u32;
_2 = 14220531529528539587_u64 as i8;
_8 = !_10;
_11 = _13.2;
_16 = _4 & _4;
_2 = _16;
_23.1 = [_1,_7,_7,_7,_1,_13.0,_7];
_24 = 15500669075569236878_usize * 2138107584935339784_usize;
_23.2 = [2828516019_u32,1924471009_u32,505216937_u32,3087174500_u32,3580679086_u32];
_12 = !_9;
_23.1 = [_7,_7,_13.0,_1,_1,_13.0,_7];
RET = -107_isize;
_13.4 = -_16;
_3 = !_14;
_21 = '\u{278ef}';
_10 = _8;
_14 = !_3;
_13.1 = _7;
_13.3 = 139436327791682017348971597691456951581_u128 * 130085638585743072443459447872876664176_u128;
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
_3 = _14 ^ _6;
_14 = _6 / _6;
_2 = _13.3 as i8;
RET = !(-9223372036854775808_isize);
_7 = _6 as i64;
_16 = _4 & _4;
_9 = _12 & _12;
_21 = '\u{a28e1}';
_13.3 = !13460041630324093081065429740111603114_u128;
_2 = 5501274382974893885629405409107847307_i128 as i8;
_6 = 15499377971799471416_u64 as u16;
Goto(bb6)
}
bb14 = {
Return()
}
bb15 = {
_31 = _36 - _36;
_29 = [1402086103_u32,3853154526_u32,412687665_u32,522191837_u32,1038078860_u32,2842261465_u32];
_35.2 = 123602738354745918415268036750923118674_i128 as u8;
_23.2 = [1988212758_u32,1261101927_u32,2550846232_u32,223447508_u32,3340269227_u32];
_29 = [7609028_u32,1163560083_u32,368544913_u32,2258215869_u32,3467733010_u32,2496782691_u32];
_12 = _9;
_35.2 = _13.2;
_40.fld0.2 = _10;
_9 = _5.0 as i32;
_38 = !_27.3.3;
_4 = _16 | _16;
_23.2 = [2410668693_u32,1971804873_u32,459981027_u32,1886656414_u32,616643527_u32];
_27.3.2 = _17 as u8;
_9 = _12 & _12;
_27.0.1 = -_13.1;
_7 = _31 as i64;
_40.fld0.0 = _27.3.0;
_44.fld1 = _29;
_43 = _31 * _36;
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(1_usize, 8_usize, Move(_8), 21_usize, Move(_21), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(1_usize, 16_usize, Move(_16), 11_usize, Move(_11), 3_usize, Move(_3), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(1_usize, 24_usize, Move(_24), 12_usize, Move(_12), 9_usize, Move(_9), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64,mut _2: i8,mut _3: u16,mut _4: (i64, i64, u8, u128, i8),mut _5: [i64; 7],mut _6: u8,mut _7: [i64; 7],mut _8: i64,mut _9: u16,mut _10: u16) -> u8 {
mir! {
type RET = u8;
let _11: Adt39;
let _12: Adt49;
let _13: Adt45;
let _14: bool;
let _15: [u8; 2];
let _16: ();
let _17: ();
{
_1 = 54_isize as i64;
_4.3 = 8323906403822667683_u64 as u128;
_9 = _3;
RET = !_6;
_4.2 = RET;
_4.0 = _4.1;
_2 = -_4.4;
_2 = _4.4 * _4.4;
_9 = '\u{11fdd}' as u16;
_13.fld4.0 = true ^ false;
_2 = -_4.4;
_13.fld3 = !527517313764177358_u64;
_13.fld2.1 = 32675_i16 as u32;
_7 = _5;
_2 = -_4.4;
_4.4 = _2 >> _10;
_13.fld7 = _13.fld3 as f32;
_13.fld5 = 1889659599_i32 & 113487255_i32;
_13.fld6 = [_4.0,_4.1,_4.0,_8,_4.1,_4.1,_4.1];
_4 = (_8, _8, _6, 332065110728718310171752080549478425437_u128, _2);
_6 = _4.2;
RET = !_6;
_8 = (-155275101140321199172670836505156872773_i128) as i64;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(2_usize, 10_usize, Move(_10), 6_usize, Move(_6), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(2_usize, 2_usize, Move(_2), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(6215623249884972830_i64), std::hint::black_box(2247322505_u32), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(127_u8), std::hint::black_box(4_usize));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: f64,
fld1: [i8; 8],

},
Variant1{
fld0: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld1: [i8; 4],
fld2: u64,
fld3: i8,
fld4: u128,
fld5: i32,
fld6: *mut i16,

},
Variant2{
fld0: [u64; 1],
fld1: i8,

},
Variant3{
fld0: (u16, i64, i32, u64),
fld1: u16,
fld2: ([u32; 6], u32, [u64; 1]),
fld3: [u32; 5],
fld4: u64,
fld5: [i64; 7],

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: [char; 5],
fld1: *mut i16,
fld2: f64,
fld3: i8,
fld4: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]),
fld5: (bool,),

},
Variant1{
fld0: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld1: *mut i16,
fld2: ([u32; 6], u32, [u64; 1]),
fld3: [i8; 8],
fld4: u64,
fld5: [u32; 5],
fld6: u8,
fld7: [i64; 7],

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: [i8; 4],
fld1: [u32; 6],
fld2: (i64, i64, u8, u128, i8),
fld3: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld4: i16,
fld5: i32,
fld6: (i8,),

},
Variant1{
fld0: [u64; 1],
fld1: u128,
fld2: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]),
fld3: u32,
fld4: [i64; 7],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: Adt41,
fld1: [u64; 1],
fld2: [u32; 5],
fld3: i8,
fld4: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld5: u8,

},
Variant1{
fld0: bool,
fld1: [i8; 4],
fld2: (u8,),
fld3: u64,
fld4: [i64; 7],
fld5: Adt40,
fld6: [i8; 8],
fld7: i128,

},
Variant2{
fld0: [u32; 5],
fld1: Adt40,
fld2: isize,
fld3: u16,
fld4: u32,
fld5: [u64; 1],

},
Variant3{
fld0: (u16, i64, i32, u64),
fld1: Adt40,
fld2: u16,
fld3: (i64, i64, u8, u128, i8),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: [u8; 7],
fld2: (u8,),
fld3: i8,
fld4: i16,
fld5: ([u32; 6], u32, [u64; 1]),
fld6: f32,
fld7: u32,

},
Variant1{
fld0: (bool,),
fld1: (i64, i64, u8, u128, i8),
fld2: [i64; 7],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i8; 4],
fld1: f32,
fld2: [u8; 7],
fld3: [u32; 6],

},
Variant1{
fld0: *mut i16,

},
Variant2{
fld0: usize,
fld1: u32,
fld2: [i8; 4],
fld3: [char; 5],
fld4: i16,
fld5: [u32; 6],
fld6: u128,

},
Variant3{
fld0: bool,
fld1: usize,
fld2: [u8; 7],
fld3: u32,
fld4: Adt40,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: Adt43,
fld1: usize,
fld2: ([u32; 6], u32, [u64; 1]),
fld3: u64,
fld4: (bool,),
fld5: i32,
fld6: [i64; 7],
fld7: f32,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (i64, i64, u8, u128, i8),
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: Adt39,
fld1: [u32; 6],
fld2: Adt40,
fld3: i8,
fld4: *mut i16,
fld5: i32,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: bool,
fld1: (i8,),
fld2: usize,
fld3: [char; 5],
fld4: Adt47,
fld5: Adt41,
fld6: u128,
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt39,
fld1: [u64; 1],
fld2: *mut i16,
fld3: u128,
fld4: u32,
fld5: Adt40,

},
Variant1{
fld0: i32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f32,

},
Variant1{
fld0: Adt40,
fld1: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]),
fld2: (bool,),
fld3: Adt43,

},
Variant2{
fld0: (bool,),
fld1: Adt45,
fld2: u128,

},
Variant3{
fld0: Adt42,
fld1: [u64; 1],
fld2: [i8; 4],
fld3: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]),
fld4: [u32; 5],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [u8; 7],
fld1: ((i8,), i32, [char; 5], bool, u32, u64, [u8; 7]),
fld2: Adt41,
fld3: [i64; 7],
fld4: u64,
fld5: u16,

},
Variant1{
fld0: i64,
fld1: [u64; 1],
fld2: Adt46,
fld3: i32,

},
Variant2{
fld0: (bool,),
fld1: i128,
fld2: [u8; 7],
fld3: u16,
fld4: [i8; 8],
fld5: Adt43,

},
Variant3{
fld0: [i8; 4],
fld1: f32,
fld2: Adt45,
fld3: [u8; 3],
fld4: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld5: ([u32; 6], u32, [u64; 1]),
fld6: i64,
fld7: Adt41,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: u8,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt43,
fld1: Adt51,
fld2: isize,
fld3: [u64; 1],
fld4: [u32; 6],

},
Variant1{
fld0: Adt41,
fld1: ((i64, i64, u8, u128, i8), u128, u16, (i64, i64, u8, u128, i8)),
fld2: (i64, i64, u8, u128, i8),
fld3: [i64; 7],
fld4: *mut i16,
fld5: f64,
fld6: [i8; 4],

},
Variant2{
fld0: Adt39,
fld1: (bool,),
fld2: u8,
fld3: [u8; 2],
fld4: Adt52,
fld5: Adt49,
fld6: [i8; 8],
fld7: f32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt49,
fld1: u64,
fld2: (bool,),
fld3: i8,
fld4: i16,

},
Variant1{
fld0: [u8; 7],
fld1: char,
fld2: Adt39,
fld3: u32,
fld4: [u8; 3],
fld5: Adt40,
fld6: Adt47,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt49,
fld1: u32,
fld2: Adt50,
fld3: i64,

},
Variant1{
fld0: bool,
fld1: Adt42,
fld2: Adt51,
fld3: [u8; 7],
fld4: i16,
fld5: [u32; 6],
fld6: Adt40,
fld7: Adt54,

}}

