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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: i16) -> i8 {
mir! {
type RET = i8;
let _4: f64;
let _5: i16;
let _6: [i128; 4];
let _7: [i128; 2];
let _8: Adt42;
let _9: usize;
let _10: bool;
let _11: Adt51;
let _12: Adt38;
let _13: (i128, i32, i128, isize);
let _14: char;
let _15: Adt41;
let _16: isize;
let _17: [bool; 6];
let _18: f32;
let _19: [i128; 2];
let _20: (i128, i32, i128, isize);
let _21: char;
let _22: isize;
let _23: isize;
let _24: (u8, u8);
let _25: [i128; 2];
let _26: [i128; 1];
let _27: [char; 7];
let _28: [i128; 1];
let _29: (i16, i64, i8, &'static u128, u64, (i128, i32, i128, isize));
let _30: Adt49;
let _31: [bool; 6];
let _32: &'static u128;
let _33: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _34: (i128, i32, i128, isize);
let _35: f64;
let _36: ([bool; 8], i16, *mut u8);
let _37: u8;
let _38: Adt50;
let _39: u16;
let _40: ();
let _41: ();
{
_3 = 25339_i16 - 9843_i16;
_2 = 207883019350204626727643563677882205860_u128 as u16;
_4 = (-9223372036854775808_isize) as f64;
_5 = !_3;
_1 = _3 >= _5;
_5 = 2111112729_i32 as i16;
RET = -(-20_i8);
RET = -21_i8;
RET = 92_u8 as i8;
_4 = 17067338794607172524_u64 as f64;
RET = 5_usize as i8;
_1 = false;
_4 = 2458139374_u32 as f64;
_6 = [124581575960623252089183392606235529915_i128,146321103296549283883847515423669215292_i128,101579818250772717330712270457598423693_i128,(-19501165597698431764569378517054245662_i128)];
_3 = _5;
_1 = true;
_3 = '\u{da9e5}' as i16;
_8 = Adt42::Variant1 { fld0: 7_usize,fld1: '\u{e4d8a}' };
_7 = [25271399029417410037990182741227133098_i128,127245420615694007025338729819278373549_i128];
Call(place!(Field::<usize>(Variant(_8, 1), 0)) = fn1(_6, _3, RET, _7, _6, _1, _1, _2, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 19_i8;
_4 = 1608774320_u32 as f64;
_4 = 115919468340128424823968233992554625933_i128 as f64;
place!(Field::<char>(Variant(_8, 1), 1)) = '\u{8a4ee}';
_6 = [131530085222424640736126258981901210108_i128,3073215664518235982085901800729062267_i128,11824798214963812937230962765151029235_i128,(-106734083731263056991705859607439911130_i128)];
place!(Field::<usize>(Variant(_8, 1), 0)) = 6495589337486378065_usize;
RET = (-99_i8) | 27_i8;
_7 = [103396058816379159850238207612939541785_i128,(-25351501931395431714465398450844244945_i128)];
_9 = Field::<usize>(Variant(_8, 1), 0);
_12.fld0 = !6_u8;
_9 = Field::<usize>(Variant(_8, 1), 0);
_12 = Adt38 { fld0: 110_u8,fld1: _6,fld2: 804521512_u32 };
_12.fld2 = 2092926944_u32 + 2937157993_u32;
_10 = !_1;
RET = (-94_i8);
_13.0 = 19795716055131537132360419527198464258_i128;
_7 = [_13.0,_13.0];
_12 = Adt38 { fld0: 212_u8,fld1: _6,fld2: 4034444662_u32 };
RET = !53_i8;
_4 = _12.fld2 as f64;
match _12.fld0 {
212 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_12.fld1 = [_13.0,_13.0,_13.0,_13.0];
place!(Field::<usize>(Variant(_8, 1), 0)) = _9;
_3 = _4 as i16;
_3 = _5;
_17 = [_10,_1,_10,_10,_1,_10];
_7 = [_13.0,_13.0];
_17 = [_10,_10,_1,_1,_1,_10];
_13 = ((-156052057612416255894916756841283179504_i128), (-1674737231_i32), (-143599273763908965076637186585554496751_i128), (-37_isize));
_13.2 = _2 as i128;
_16 = !_13.3;
_14 = Field::<char>(Variant(_8, 1), 1);
_6 = _12.fld1;
_6 = [_13.2,_13.2,_13.0,_13.2];
_14 = Field::<char>(Variant(_8, 1), 1);
RET = 15515778453353699502_u64 as i8;
_14 = Field::<char>(Variant(_8, 1), 1);
_10 = _2 > _2;
_18 = 297299500298760789931238300482076180724_u128 as f32;
_8 = Adt42::Variant1 { fld0: _9,fld1: _14 };
_18 = _12.fld0 as f32;
_10 = _1;
_6 = [_13.0,_13.0,_13.2,_13.0];
SetDiscriminant(_8, 2);
Call(_6 = core::intrinsics::transmute(_12.fld1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_13.0,_13.2,_13.0,_13.2];
_12 = Adt38 { fld0: 117_u8,fld1: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).1,fld2: 280488173_u32 };
_19 = [_13.0,_13.2];
_20.0 = !_13.0;
place!(Field::<i8>(Variant(_8, 2), 3)) = RET - RET;
place!(Field::<u32>(Variant(_8, 2), 4)) = _12.fld2 >> _13.3;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).0 = _18;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = [_20.0,_13.0,_13.0,_20.0];
_20 = (_13.2, _13.1, _13.0, _16);
place!(Field::<bool>(Variant(_8, 2), 0)) = _10;
_13.1 = -_20.1;
_4 = _9 as f64;
_22 = !_16;
_13.0 = -_13.2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).3.1 = _12.fld0 | _12.fld0;
_20.3 = !_22;
Goto(bb5)
}
bb5 = {
_5 = _3 * _3;
_7 = _19;
_12.fld0 = Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1 << Field::<u32>(Variant(_8, 2), 4);
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = _12.fld1;
_24.1 = !_12.fld0;
_4 = 8486017001161474221_u64 as f64;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).3.0 = Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1 >> _12.fld0;
_20 = (_13.2, _13.1, _13.2, _16);
_27 = [_14,_14,_14,_14,_14,_14,_14];
_13.0 = _14 as i128;
_20 = _13;
_13.1 = !_20.1;
_12 = Adt38 { fld0: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0,fld1: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).1,fld2: Field::<u32>(Variant(_8, 2), 4) };
_20 = (_13.2, _13.1, _13.0, _22);
_29.0 = _5;
_29.2 = 11043477322421947528_u64 as i8;
_29.5.0 = -_13.0;
_29.5.2 = _29.5.0;
match _13.3 {
340282366920938463463374607431768211419 => bb7,
_ => bb6
}
}
bb6 = {
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_13.0,_13.2,_13.0,_13.2];
_12 = Adt38 { fld0: 117_u8,fld1: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).1,fld2: 280488173_u32 };
_19 = [_13.0,_13.2];
_20.0 = !_13.0;
place!(Field::<i8>(Variant(_8, 2), 3)) = RET - RET;
place!(Field::<u32>(Variant(_8, 2), 4)) = _12.fld2 >> _13.3;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).0 = _18;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = [_20.0,_13.0,_13.0,_20.0];
_20 = (_13.2, _13.1, _13.0, _16);
place!(Field::<bool>(Variant(_8, 2), 0)) = _10;
_13.1 = -_20.1;
_4 = _9 as f64;
_22 = !_16;
_13.0 = -_13.2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).3.1 = _12.fld0 | _12.fld0;
_20.3 = !_22;
Goto(bb5)
}
bb7 = {
_7 = _19;
_19 = [_20.2,_13.0];
_6 = Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).4 = _20.1 ^ _20.1;
_29.0 = _3 | _5;
_20.3 = -_13.3;
_25 = [_13.2,_20.0];
_29.5 = (_20.2, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).4, _20.0, _22);
_2 = 44207_u16 - 13268_u16;
_13.0 = !_29.5.0;
_28 = [_20.0];
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_29.5.0,_13.2,_13.0,_13.2];
_29.5.2 = _13.0;
_24 = (_12.fld0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1);
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = [_29.5.0,_29.5.0,_29.5.2,_13.2];
_26 = [_13.2];
_21 = _14;
match _13.3 {
0 => bb5,
1 => bb4,
2 => bb6,
3 => bb8,
340282366920938463463374607431768211419 => bb10,
_ => bb9
}
}
bb8 = {
_12.fld1 = [_13.0,_13.0,_13.0,_13.0];
place!(Field::<usize>(Variant(_8, 1), 0)) = _9;
_3 = _4 as i16;
_3 = _5;
_17 = [_10,_1,_10,_10,_1,_10];
_7 = [_13.0,_13.0];
_17 = [_10,_10,_1,_1,_1,_10];
_13 = ((-156052057612416255894916756841283179504_i128), (-1674737231_i32), (-143599273763908965076637186585554496751_i128), (-37_isize));
_13.2 = _2 as i128;
_16 = !_13.3;
_14 = Field::<char>(Variant(_8, 1), 1);
_6 = _12.fld1;
_6 = [_13.2,_13.2,_13.0,_13.2];
_14 = Field::<char>(Variant(_8, 1), 1);
RET = 15515778453353699502_u64 as i8;
_14 = Field::<char>(Variant(_8, 1), 1);
_10 = _2 > _2;
_18 = 297299500298760789931238300482076180724_u128 as f32;
_8 = Adt42::Variant1 { fld0: _9,fld1: _14 };
_18 = _12.fld0 as f32;
_10 = _1;
_6 = [_13.0,_13.0,_13.2,_13.0];
SetDiscriminant(_8, 2);
Call(_6 = core::intrinsics::transmute(_12.fld1), ReturnTo(bb4), UnwindUnreachable())
}
bb9 = {
_5 = _3 * _3;
_7 = _19;
_12.fld0 = Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1 << Field::<u32>(Variant(_8, 2), 4);
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = _12.fld1;
_24.1 = !_12.fld0;
_4 = 8486017001161474221_u64 as f64;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).3.0 = Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1 >> _12.fld0;
_20 = (_13.2, _13.1, _13.2, _16);
_27 = [_14,_14,_14,_14,_14,_14,_14];
_13.0 = _14 as i128;
_20 = _13;
_13.1 = !_20.1;
_12 = Adt38 { fld0: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0,fld1: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).1,fld2: Field::<u32>(Variant(_8, 2), 4) };
_20 = (_13.2, _13.1, _13.0, _22);
_29.0 = _5;
_29.2 = 11043477322421947528_u64 as i8;
_29.5.0 = -_13.0;
_29.5.2 = _29.5.0;
match _13.3 {
340282366920938463463374607431768211419 => bb7,
_ => bb6
}
}
bb10 = {
RET = _29.2 ^ _29.2;
_20.2 = _13.0;
place!(Field::<u32>(Variant(_8, 2), 4)) = _12.fld2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).0 = _18;
_13.0 = _21 as i128;
_29.4 = 8722655205212697934_u64;
_29.5 = (_13.2, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).4, _20.0, _13.3);
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_20.2,_20.2,_20.2,_13.2];
_36.1 = _18 as i16;
_37 = !_24.1;
_34.0 = !_20.2;
_34.3 = _20.3 ^ _29.5.3;
_38.fld1.2 = !_13.2;
_38.fld4 = _24;
Call(_38.fld5.0 = fn11(_38.fld4.0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1, _24.0, _38.fld4, _12.fld1, _24, _38.fld4, _38.fld4, _38.fld4, Move(_12), Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1, _34.3, _38.fld4.0, _29.5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_29.2 = 3566927535463362975_i64 as i8;
_38.fld1.1 = !_29.5.1;
Goto(bb12)
}
bb12 = {
_36.2 = core::ptr::addr_of_mut!(_33.3.1);
_34.1 = _10 as i32;
_24 = (_38.fld4.0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0);
_23 = !_22;
_33.3.0 = _38.fld4.1;
_3 = !_36.1;
_38.fld5.3.1 = _1 as u8;
_23 = !_22;
match _29.5.3 {
0 => bb10,
1 => bb2,
2 => bb6,
3 => bb11,
4 => bb8,
5 => bb13,
6 => bb14,
340282366920938463463374607431768211419 => bb16,
_ => bb15
}
}
bb13 = {
_29.2 = 3566927535463362975_i64 as i8;
_38.fld1.1 = !_29.5.1;
Goto(bb12)
}
bb14 = {
RET = _29.2 ^ _29.2;
_20.2 = _13.0;
place!(Field::<u32>(Variant(_8, 2), 4)) = _12.fld2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).0 = _18;
_13.0 = _21 as i128;
_29.4 = 8722655205212697934_u64;
_29.5 = (_13.2, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).4, _20.0, _13.3);
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_20.2,_20.2,_20.2,_13.2];
_36.1 = _18 as i16;
_37 = !_24.1;
_34.0 = !_20.2;
_34.3 = _20.3 ^ _29.5.3;
_38.fld1.2 = !_13.2;
_38.fld4 = _24;
Call(_38.fld5.0 = fn11(_38.fld4.0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1, _24.0, _38.fld4, _12.fld1, _24, _38.fld4, _38.fld4, _38.fld4, Move(_12), Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0, Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.1, _34.3, _38.fld4.0, _29.5), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).1 = [_13.0,_13.2,_13.0,_13.2];
_12 = Adt38 { fld0: 117_u8,fld1: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).1,fld2: 280488173_u32 };
_19 = [_13.0,_13.2];
_20.0 = !_13.0;
place!(Field::<i8>(Variant(_8, 2), 3)) = RET - RET;
place!(Field::<u32>(Variant(_8, 2), 4)) = _12.fld2 >> _13.3;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).0 = _18;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).2 = [_20.0,_13.0,_13.0,_20.0];
_20 = (_13.2, _13.1, _13.0, _16);
place!(Field::<bool>(Variant(_8, 2), 0)) = _10;
_13.1 = -_20.1;
_4 = _9 as f64;
_22 = !_16;
_13.0 = -_13.2;
place!(Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5)).3.1 = _12.fld0 | _12.fld0;
_20.3 = !_22;
Goto(bb5)
}
bb16 = {
_17 = [_10,_1,Field::<bool>(Variant(_8, 2), 0),Field::<bool>(Variant(_8, 2), 0),Field::<bool>(Variant(_8, 2), 0),_1];
_29.5.3 = _16;
_12 = Adt38 { fld0: Field::<(f32, [i128; 4], [i128; 4], (u8, u8), i32)>(Variant(_8, 2), 5).3.0,fld1: _6,fld2: Field::<u32>(Variant(_8, 2), 4) };
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(0_usize, 9_usize, Move(_9), 26_usize, Move(_26), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(0_usize, 22_usize, Move(_22), 3_usize, Move(_3), 23_usize, Move(_23), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(0_usize, 5_usize, Move(_5), 28_usize, Move(_28), 24_usize, Move(_24), 41_usize, _41), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: [i128; 4],mut _2: i16,mut _3: i8,mut _4: [i128; 2],mut _5: [i128; 4],mut _6: bool,mut _7: bool,mut _8: u16,mut _9: i16) -> usize {
mir! {
type RET = usize;
let _10: isize;
let _11: Adt37;
let _12: f64;
let _13: i32;
let _14: isize;
let _15: Adt38;
let _16: f32;
let _17: i64;
let _18: u64;
let _19: isize;
let _20: f32;
let _21: i8;
let _22: char;
let _23: char;
let _24: [bool; 8];
let _25: isize;
let _26: Adt44;
let _27: isize;
let _28: u8;
let _29: ();
let _30: ();
{
_8 = !10651_u16;
_9 = !_2;
_2 = _9;
_7 = _6 & _6;
_4 = [(-12624472541933844011616198373257392298_i128),114322845559170391869997366044118435488_i128];
_5 = [62583539531217447530291990651202474026_i128,167829971070599227558818143414736488213_i128,(-122212922034778620607906697487368529075_i128),(-101833722724736067498269667677685032559_i128)];
_8 = 22327_u16;
RET = (-4234910183830767299_i64) as usize;
_5 = [(-73122853954424281924224663504397102258_i128),168617399972813674265355819549898226517_i128,103143567126801647914204465788103005920_i128,(-12567551659194536653381667756224690565_i128)];
_2 = _9 | _9;
_9 = _7 as i16;
_4 = [36891538409177184772212428637768707811_i128,(-127094437108344165215097163375646380863_i128)];
RET = 5_usize ^ 9140664709716752269_usize;
_9 = (-5515321166815903320_i64) as i16;
_3 = (-92_i8) - 4_i8;
_2 = -_9;
_9 = 21052120_i32 as i16;
_2 = 56_u8 as i16;
_4 = [(-134690822594779684415391821041300233957_i128),(-50104859738222774595051664695669448321_i128)];
RET = _3 as usize;
_9 = _7 as i16;
_4 = [(-22452344855366534057624807215384677555_i128),(-115491852695974996608540095910500537575_i128)];
_3 = 70_i8;
_9 = _3 as i16;
RET = 5_usize;
_5 = _1;
RET = !5_usize;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
22327 => bb8,
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
_9 = _3 as i16;
_1 = [(-18276527782973114678528597476832387660_i128),(-103058974377664174382725530534904092602_i128),(-45778294162909747490759160358393896020_i128),(-33440990752140575745903252114604640750_i128)];
Call(_1 = fn2(_7, _4, _4, _8, _8, _5, _6, _2, RET, RET, _4, _4, _6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = [(-2401210787776682907598818587083407824_i128),(-117190264067756775546936435735329058812_i128),(-119735232829592446171220416103060747717_i128),11013754359003604196709819395954424904_i128];
_3 = 86_i8 & (-59_i8);
_5 = _1;
_8 = !16088_u16;
_7 = _6 & _6;
_5 = _1;
Goto(bb10)
}
bb10 = {
_6 = _2 == _9;
Call(_3 = fn6(_6, _5, _5, _5, _5, _5, _5, _5, _6, _2, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1 = [(-100261696196246677329700028696235398851_i128),82579994013891706705681533192837330404_i128,13147554916989913201005267245045832496_i128,77797797758690457992083970600829103191_i128];
_5 = [(-32004541430766786265167243960162061883_i128),(-142995753356542596901743418305080991969_i128),(-65007642057571854349252264971520893009_i128),131492572644949669997882493479979630410_i128];
_10 = 1747824921_u32 as isize;
RET = 4_usize;
_6 = !_7;
_10 = -20_isize;
_9 = _2;
_5 = [157475362181361841972028011370618387973_i128,(-32168693025647903775440055550554850324_i128),109494223546049466452887274206944331261_i128,(-70854015739028461339381580535980972573_i128)];
_12 = _3 as f64;
_9 = _2 >> _3;
RET = '\u{7aae3}' as usize;
_13 = 2044172199_i32;
RET = 16685832585331433109_usize - 2906081193451923435_usize;
_8 = 51708_u16 - 35397_u16;
_14 = -_10;
_12 = 316380373916195227794277623808376204486_u128 as f64;
_14 = _10 + _10;
RET = _3 as usize;
Call(RET = core::intrinsics::bswap(13338135311994147932_usize), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = _7;
_14 = _7 as isize;
_2 = _9 * _9;
_13 = 189027005_i32 ^ (-799470141_i32);
_15 = Adt38 { fld0: 4_u8,fld1: _1,fld2: 1316632129_u32 };
_2 = (-2415614296800500581875462520451609208_i128) as i16;
RET = 6698110515125901419_usize | 3_usize;
_6 = !_7;
_15.fld0 = 12_u8 ^ 137_u8;
_8 = !58226_u16;
_14 = !_10;
_3 = -(-86_i8);
_14 = _10;
_12 = 55025805164578758607861631775473087226_u128 as f64;
_15.fld2 = 769395650_u32;
_17 = (-5720672342248234154_i64);
RET = _6 as usize;
_1 = [21290118477144875856449056201207609164_i128,105522220046017118844592675052570119567_i128,(-52043501890179336879142619014164453756_i128),(-10116756081926721891326126637286679184_i128)];
_16 = _9 as f32;
_2 = !_9;
_8 = 38681_u16 << _17;
_12 = _2 as f64;
Goto(bb13)
}
bb13 = {
_9 = _2;
_10 = _14;
RET = _10 as usize;
_15.fld0 = 255_u8;
_20 = _12 as f32;
_21 = !_3;
_19 = _10 * _14;
_18 = 2551574763131648678_u64;
_18 = 9456452038208300841_u64 - 5498640744305195561_u64;
_19 = !_10;
_19 = !_14;
_18 = !7957875433034690235_u64;
_15.fld0 = 252_u8 << RET;
_10 = _16 as isize;
_16 = _20;
_1 = [(-162279965889738607202936876294744976019_i128),59196443570308047366259552808095797979_i128,(-41725838900989342383221120994809472900_i128),108500932814528911155392484165908892148_i128];
_7 = _6 | _6;
_18 = !14016781167622599475_u64;
_15.fld1 = [(-162992914457047608806974748997973336973_i128),808805910039221720230289334023593195_i128,(-21742874572894162399327909205379374452_i128),(-75419753239994715020466682516722122852_i128)];
RET = _21 as usize;
_12 = _18 as f64;
_19 = _10;
match _15.fld2 {
769395650 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_14 = !_10;
_16 = _20 + _20;
_17 = RET as i64;
_12 = _15.fld0 as f64;
_22 = '\u{d6e48}';
_2 = -_9;
_7 = _6;
_23 = _22;
_9 = _2;
RET = !3_usize;
_14 = _10;
_1 = [82476171452146609547037590838685610229_i128,(-73836734793205686247154803649075354144_i128),123362238815093432191943496451236879902_i128,(-160003141241791223194688700769340678490_i128)];
_8 = 45191_u16;
RET = 12271783347550097014_usize;
_9 = _2;
_10 = _19;
_4 = [(-120385982961035375185729527543629070700_i128),(-54536561130533940544523578857369515063_i128)];
_19 = _10 | _14;
_23 = _22;
_15.fld0 = _23 as u8;
_15.fld1 = _5;
RET = !17027730761843368426_usize;
_9 = _2;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(1_usize, 10_usize, Move(_10), 17_usize, Move(_17), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(1_usize, 19_usize, Move(_19), 7_usize, Move(_7), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(1_usize, 5_usize, Move(_5), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: [i128; 2],mut _3: [i128; 2],mut _4: u16,mut _5: u16,mut _6: [i128; 4],mut _7: bool,mut _8: i16,mut _9: usize,mut _10: usize,mut _11: [i128; 2],mut _12: [i128; 2],mut _13: bool) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _14: (i128, i32, i128, isize);
let _15: bool;
let _16: f64;
let _17: i128;
let _18: i128;
let _19: *mut u8;
let _20: &'static u128;
let _21: Adt42;
let _22: Adt46;
let _23: i8;
let _24: usize;
let _25: isize;
let _26: u128;
let _27: char;
let _28: bool;
let _29: Adt48;
let _30: [char; 5];
let _31: i16;
let _32: [char; 5];
let _33: ();
let _34: ();
{
_13 = _1;
_7 = !_1;
_6 = [162620919057071772922387882929038593814_i128,(-149859493620959436974775113914040353910_i128),(-506744248848411592001644728224163473_i128),72852463923684904331558039976782943244_i128];
_4 = _8 as u16;
_16 = 9223372036854775807_isize as f64;
_14.3 = 6737810633341921562_u64 as isize;
_14.3 = -(-9223372036854775808_isize);
_10 = (-5580285080480234815_i64) as usize;
_14.2 = 158242772349764441515556886928583079803_i128 * 115838913459013135062431549604576803331_i128;
_11 = _3;
_14 = ((-7250092191999033501971151073624572964_i128), 2135755398_i32, 114487880498843689917364352981668126434_i128, (-9223372036854775808_isize));
_14.2 = _14.0;
Call(_17 = core::intrinsics::transmute(_14.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _9;
_11 = [_17,_14.2];
RET = [_14.2,_14.0,_17,_14.2];
_2 = [_14.0,_17];
_13 = _7;
_5 = _4 * _4;
_15 = !_7;
_14.1 = 1687141392_i32;
Call(_2 = fn3(_14.2, _14, _6, _6, RET, _13, RET, _14, _14, _5, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [_14.2,_17];
_17 = 727509490_u32 as i128;
_10 = _9 ^ _9;
_9 = _10 << _10;
_16 = _5 as f64;
_3 = [_14.2,_14.0];
_7 = _9 >= _9;
_4 = _5;
Goto(bb3)
}
bb3 = {
_13 = _7;
RET = _6;
_14.0 = _14.2 << _14.3;
_8 = 1995536031173714324799268660589728056_u128 as i16;
_14.0 = -_14.2;
_11 = _2;
_16 = _14.2 as f64;
_5 = !_4;
_14.1 = (-1579610260_i32) >> _9;
_6 = RET;
_17 = _14.3 as i128;
RET = [_17,_14.0,_17,_14.0];
_11 = _3;
_3 = _11;
_14.0 = -_14.2;
RET = _6;
_5 = !_4;
_18 = _14.3 as i128;
_14.3 = 43998899388617458782017965190087007742_u128 as isize;
_13 = _14.1 > _14.1;
_18 = _14.2 >> _9;
_21 = Adt42::Variant1 { fld0: _10,fld1: '\u{1f106}' };
_5 = _14.3 as u16;
Call(_1 = fn4(_14, _14, _10, _14, _5, RET, _13, _14, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.3 = 9223372036854775807_isize & (-9223372036854775808_isize);
_5 = _7 as u16;
_9 = !_10;
_2 = [_18,_14.0];
_15 = _8 > _8;
_1 = _13 & _7;
_1 = !_15;
_24 = !_9;
_14.0 = !_14.2;
_15 = _7 & _7;
_9 = Field::<usize>(Variant(_21, 1), 0);
_10 = _24 * _24;
_8 = 4751_i16;
_24 = !Field::<usize>(Variant(_21, 1), 0);
_4 = 8602875845701721020_i64 as u16;
RET = [_14.2,_14.2,_18,_14.0];
_6 = [_18,_18,_14.0,_17];
_4 = !_5;
_10 = Field::<usize>(Variant(_21, 1), 0);
_5 = 271585240601603779999388537008783984296_u128 as u16;
_4 = _5;
_10 = _8 as usize;
_24 = !_10;
RET = [_14.2,_14.0,_17,_18];
RET = _6;
Goto(bb5)
}
bb5 = {
_14.3 = !(-118_isize);
_23 = 64_i8 | (-6_i8);
_14 = (_18, (-1822136903_i32), _18, (-9223372036854775808_isize));
_3 = _2;
_14.1 = !(-1729799833_i32);
_10 = Field::<usize>(Variant(_21, 1), 0) | _9;
match _8 {
0 => bb3,
1 => bb6,
2 => bb7,
4751 => bb9,
_ => bb8
}
}
bb6 = {
_14.3 = 9223372036854775807_isize & (-9223372036854775808_isize);
_5 = _7 as u16;
_9 = !_10;
_2 = [_18,_14.0];
_15 = _8 > _8;
_1 = _13 & _7;
_1 = !_15;
_24 = !_9;
_14.0 = !_14.2;
_15 = _7 & _7;
_9 = Field::<usize>(Variant(_21, 1), 0);
_10 = _24 * _24;
_8 = 4751_i16;
_24 = !Field::<usize>(Variant(_21, 1), 0);
_4 = 8602875845701721020_i64 as u16;
RET = [_14.2,_14.2,_18,_14.0];
_6 = [_18,_18,_14.0,_17];
_4 = !_5;
_10 = Field::<usize>(Variant(_21, 1), 0);
_5 = 271585240601603779999388537008783984296_u128 as u16;
_4 = _5;
_10 = _8 as usize;
_24 = !_10;
RET = [_14.2,_14.0,_17,_18];
RET = _6;
Goto(bb5)
}
bb7 = {
_10 = _9;
_11 = [_17,_14.2];
RET = [_14.2,_14.0,_17,_14.2];
_2 = [_14.0,_17];
_13 = _7;
_5 = _4 * _4;
_15 = !_7;
_14.1 = 1687141392_i32;
Call(_2 = fn3(_14.2, _14, _6, _6, RET, _13, RET, _14, _14, _5, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_2 = [_14.2,_17];
_17 = 727509490_u32 as i128;
_10 = _9 ^ _9;
_9 = _10 << _10;
_16 = _5 as f64;
_3 = [_14.2,_14.0];
_7 = _9 >= _9;
_4 = _5;
Goto(bb3)
}
bb9 = {
place!(Field::<usize>(Variant(_21, 1), 0)) = !_9;
_13 = _18 < _14.0;
_16 = _14.1 as f64;
RET = [_18,_14.2,_18,_14.2];
_13 = _7;
place!(Field::<usize>(Variant(_21, 1), 0)) = _10 >> _14.0;
_6 = RET;
_14.3 = (-9223372036854775808_isize) ^ 42_isize;
_18 = _17 + _14.0;
place!(Field::<usize>(Variant(_21, 1), 0)) = _10;
_26 = 71758808049696901717595643278614563859_u128 << _14.0;
_2 = _3;
_27 = '\u{3e020}';
_10 = Field::<usize>(Variant(_21, 1), 0);
_4 = _5;
_16 = 1895251966_u32 as f64;
_2 = [_14.2,_18];
_24 = 2504991216_u32 as usize;
_14.2 = 189_u8 as i128;
_9 = _8 as usize;
_14.1 = (-674210183_i32) << _23;
_11 = _3;
_25 = _14.3;
match _8 {
0 => bb4,
1 => bb10,
4751 => bb12,
_ => bb11
}
}
bb10 = {
_2 = [_14.2,_17];
_17 = 727509490_u32 as i128;
_10 = _9 ^ _9;
_9 = _10 << _10;
_16 = _5 as f64;
_3 = [_14.2,_14.0];
_7 = _9 >= _9;
_4 = _5;
Goto(bb3)
}
bb11 = {
_14.3 = !(-118_isize);
_23 = 64_i8 | (-6_i8);
_14 = (_18, (-1822136903_i32), _18, (-9223372036854775808_isize));
_3 = _2;
_14.1 = !(-1729799833_i32);
_10 = Field::<usize>(Variant(_21, 1), 0) | _9;
match _8 {
0 => bb3,
1 => bb6,
2 => bb7,
4751 => bb9,
_ => bb8
}
}
bb12 = {
_24 = _10 + Field::<usize>(Variant(_21, 1), 0);
_32 = [_27,_27,_27,_27,_27];
_4 = _5;
_10 = Field::<usize>(Variant(_21, 1), 0) + _24;
_30 = [_27,_27,_27,_27,_27];
_6 = [_18,_18,_18,_14.0];
_6 = RET;
_5 = _4 & _4;
RET = [_14.0,_14.0,_18,_14.2];
_20 = &_26;
_13 = !_7;
_18 = _14.0 << (*_20);
_6 = RET;
_1 = !_15;
_18 = _15 as i128;
place!(Field::<usize>(Variant(_21, 1), 0)) = _24;
_16 = 1960842817822616728_u64 as f64;
_23 = 17241978765354622908_u64 as i8;
RET = [_18,_18,_17,_18];
Goto(bb13)
}
bb13 = {
Call(_33 = dump_var(2_usize, 26_usize, Move(_26), 12_usize, Move(_12), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_33 = dump_var(2_usize, 5_usize, Move(_5), 7_usize, Move(_7), 11_usize, Move(_11), 14_usize, Move(_14)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_33 = dump_var(2_usize, 25_usize, Move(_25), 6_usize, Move(_6), 18_usize, Move(_18), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i128,mut _2: (i128, i32, i128, isize),mut _3: [i128; 4],mut _4: [i128; 4],mut _5: [i128; 4],mut _6: bool,mut _7: [i128; 4],mut _8: (i128, i32, i128, isize),mut _9: (i128, i32, i128, isize),mut _10: u16,mut _11: u16) -> [i128; 2] {
mir! {
type RET = [i128; 2];
let _12: Adt46;
let _13: Adt41;
let _14: i8;
let _15: i32;
let _16: (i128, i32, i128, isize);
let _17: Adt40;
let _18: Adt52;
let _19: f32;
let _20: ();
let _21: ();
{
_8.2 = _2.1 as i128;
_9.3 = -_8.3;
_9.1 = _10 as i32;
_7 = [_2.2,_9.2,_8.2,_1];
Goto(bb1)
}
bb1 = {
_7 = [_9.0,_2.0,_2.0,_8.0];
_2.1 = _9.3 as i32;
_8 = (_1, _2.1, _2.0, _2.3);
_2.1 = _8.1;
_9.0 = !_9.2;
_10 = !_11;
_5 = [_2.0,_9.2,_2.0,_9.0];
_9.2 = 106_i8 as i128;
RET = [_8.0,_9.0];
_2.3 = _6 as isize;
_2 = _9;
_2.3 = !_8.3;
_2.3 = _9.3 & _9.3;
_2.1 = _8.1 & _8.1;
_5 = _4;
_5 = [_8.0,_9.0,_8.2,_9.0];
_9.2 = -_9.0;
Goto(bb2)
}
bb2 = {
RET = [_1,_9.2];
_4 = [_8.0,_9.0,_2.0,_8.0];
_8.2 = 9328765112337194373_usize as i128;
_4 = [_9.0,_2.0,_1,_8.0];
_2.3 = !_9.3;
_4 = _3;
_4 = [_1,_9.2,_8.2,_2.0];
_5 = _4;
_9 = (_2.2, _2.1, _2.0, _2.3);
RET = [_9.2,_2.0];
_7 = [_1,_2.0,_8.0,_1];
_3 = [_8.0,_1,_1,_2.0];
_8.0 = _2.1 as i128;
_14 = 37_i8;
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
333032274728939429961403456358143638492 => bb10,
_ => bb9
}
}
bb3 = {
_7 = [_9.0,_2.0,_2.0,_8.0];
_2.1 = _9.3 as i32;
_8 = (_1, _2.1, _2.0, _2.3);
_2.1 = _8.1;
_9.0 = !_9.2;
_10 = !_11;
_5 = [_2.0,_9.2,_2.0,_9.0];
_9.2 = 106_i8 as i128;
RET = [_8.0,_9.0];
_2.3 = _6 as isize;
_2 = _9;
_2.3 = !_8.3;
_2.3 = _9.3 & _9.3;
_2.1 = _8.1 & _8.1;
_5 = _4;
_5 = [_8.0,_9.0,_8.2,_9.0];
_9.2 = -_9.0;
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
_16.0 = !_2.0;
_16 = (_8.0, _2.1, _9.2, _9.3);
_9 = (_8.0, _16.1, _16.0, _2.3);
_15 = !_9.1;
_16 = (_2.0, _9.1, _9.2, _8.3);
_16.3 = _2.3 << _2.2;
_8.1 = 2_usize as i32;
_6 = false;
_2 = _16;
_14 = !77_i8;
_2.2 = _9.2 * _9.0;
_1 = 17681_i16 as i128;
Goto(bb11)
}
bb11 = {
RET = [_9.2,_16.2];
_2.0 = _8.0;
_2.2 = !_16.0;
_10 = '\u{1f0af}' as u16;
_9 = _8;
_16 = (_2.0, _2.1, _9.0, _2.3);
_8.2 = !_2.2;
match _9.3 {
0 => bb10,
1 => bb9,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb12 = {
_16.0 = !_2.0;
_16 = (_8.0, _2.1, _9.2, _9.3);
_9 = (_8.0, _16.1, _16.0, _2.3);
_15 = !_9.1;
_16 = (_2.0, _9.1, _9.2, _8.3);
_16.3 = _2.3 << _2.2;
_8.1 = 2_usize as i32;
_6 = false;
_2 = _16;
_14 = !77_i8;
_2.2 = _9.2 * _9.0;
_1 = 17681_i16 as i128;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_7 = [_9.0,_2.0,_2.0,_8.0];
_2.1 = _9.3 as i32;
_8 = (_1, _2.1, _2.0, _2.3);
_2.1 = _8.1;
_9.0 = !_9.2;
_10 = !_11;
_5 = [_2.0,_9.2,_2.0,_9.0];
_9.2 = 106_i8 as i128;
RET = [_8.0,_9.0];
_2.3 = _6 as isize;
_2 = _9;
_2.3 = !_8.3;
_2.3 = _9.3 & _9.3;
_2.1 = _8.1 & _8.1;
_5 = _4;
_5 = [_8.0,_9.0,_8.2,_9.0];
_9.2 = -_9.0;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_7 = [_9.0,_2.0,_2.0,_8.0];
_2.1 = _9.3 as i32;
_8 = (_1, _2.1, _2.0, _2.3);
_2.1 = _8.1;
_9.0 = !_9.2;
_10 = !_11;
_5 = [_2.0,_9.2,_2.0,_9.0];
_9.2 = 106_i8 as i128;
RET = [_8.0,_9.0];
_2.3 = _6 as isize;
_2 = _9;
_2.3 = !_8.3;
_2.3 = _9.3 & _9.3;
_2.1 = _8.1 & _8.1;
_5 = _4;
_5 = [_8.0,_9.0,_8.2,_9.0];
_9.2 = -_9.0;
Goto(bb2)
}
bb17 = {
_2 = (_16.0, _16.1, _9.0, _8.3);
_4 = [_2.2,_8.0,_16.0,_2.2];
_4 = [_2.0,_1,_16.0,_8.2];
_4 = [_8.2,_8.0,_16.2,_16.0];
_3 = _4;
_9.1 = -_2.1;
_2.0 = _2.2 >> _16.2;
Goto(bb18)
}
bb18 = {
Call(_20 = dump_var(3_usize, 2_usize, Move(_2), 9_usize, Move(_9), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_20 = dump_var(3_usize, 1_usize, Move(_1), 14_usize, Move(_14), 4_usize, Move(_4), 21_usize, _21), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (i128, i32, i128, isize),mut _2: (i128, i32, i128, isize),mut _3: usize,mut _4: (i128, i32, i128, isize),mut _5: u16,mut _6: [i128; 4],mut _7: bool,mut _8: (i128, i32, i128, isize),mut _9: [i128; 2]) -> bool {
mir! {
type RET = bool;
let _10: (i128, usize, &'static u128, [i128; 1], *const bool);
let _11: isize;
let _12: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _13: Adt38;
let _14: u8;
let _15: Adt48;
let _16: Adt47;
let _17: f64;
let _18: [bool; 8];
let _19: isize;
let _20: f32;
let _21: Adt53;
let _22: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _23: [i128; 1];
let _24: isize;
let _25: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _26: isize;
let _27: isize;
let _28: i128;
let _29: i128;
let _30: [i128; 2];
let _31: ();
let _32: ();
{
_9 = [_1.0,_8.2];
_4 = _8;
_4.3 = _8.3 | _8.3;
_2 = (_8.2, _1.1, _1.2, _8.3);
RET = !_7;
_8.3 = !_4.3;
_3 = 16004479361254274482_usize;
_3 = !18435634806101619110_usize;
_13 = Adt38 { fld0: 121_u8,fld1: _6,fld2: 2471360284_u32 };
RET = !_7;
_7 = RET;
_13.fld1 = _6;
_4.0 = _2.2 - _2.0;
match _13.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
121 => bb9,
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
_4.0 = -_8.2;
_2.3 = 8373900212338614662_u64 as isize;
_6 = [_2.2,_8.0,_8.0,_1.0];
_11 = _4.3 + _2.3;
_4.3 = (-5478586752304792774_i64) as isize;
_10.1 = '\u{26a4d}' as usize;
_10.0 = _13.fld2 as i128;
_4.0 = _10.0 * _2.2;
Call(_2.1 = fn5(_1, _4.2, _1, _8.2, _2.2, RET, _1, _1.2, _4, _11, _8.3, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = (_2.0, _2.1, _8.0, _1.3);
_10.3 = [_8.0];
_13.fld1 = _6;
_12.1 = [_4.2,_10.0,_1.0,_2.0];
_12.2 = _13.fld1;
RET = !_7;
_4.0 = -_4.2;
_8 = (_10.0, _4.1, _10.0, _2.3);
match _2.0 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb4,
333032274728939429961403456358143638492 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_1.1 = !_8.1;
_8.3 = _11;
_19 = !_11;
_2.1 = _4.1 << _13.fld0;
_18 = [_7,_7,RET,RET,RET,RET,RET,RET];
RET = _4.1 != _1.1;
_22.1 = [_4.0,_10.0,_8.0,_1.0];
_4.3 = 403709410719291343_i64 as isize;
_22.4 = _1.1;
_20 = (-876316858951150453_i64) as f32;
_10.4 = core::ptr::addr_of!(RET);
_12.4 = _4.1 - _22.4;
_12.3 = (_13.fld0, _13.fld0);
RET = _7 ^ _7;
_9 = [_1.0,_8.0];
_12.0 = _20;
RET = !_7;
_1.0 = -_8.0;
_24 = 174293978242332310132233841446083911711_u128 as isize;
_22.2 = [_10.0,_1.0,_1.2,_1.2];
_25.3.1 = _12.3.1;
_10.3 = [_8.2];
_23 = [_4.2];
_2.0 = _12.0 as i128;
_22.1 = [_1.0,_1.0,_4.2,_8.0];
Goto(bb13)
}
bb13 = {
_1.1 = _4.1;
_17 = _5 as f64;
_1 = (_4.0, _8.1, _10.0, _11);
_22.2 = [_10.0,_2.0,_2.2,_8.2];
_25.1 = [_8.2,_1.0,_1.2,_4.2];
_7 = _12.4 < _1.1;
_1.1 = -_12.4;
_3 = _4.0 as usize;
_25.4 = _12.3.1 as i32;
_4.2 = _1.2;
_2.3 = _1.3 & _8.3;
_7 = RET;
_8.2 = -_2.2;
_14 = _17 as u8;
_22 = (_20, _12.1, _13.fld1, _12.3, _1.1);
RET = _7 | _7;
_25.1 = [_4.2,_2.2,_2.0,_2.2];
_18 = [RET,RET,_7,RET,RET,RET,_7,RET];
RET = !_7;
_8.1 = 11642291933923159181_u64 as i32;
_9 = [_4.2,_8.2];
RET = !_7;
_25 = (_20, _6, _22.2, _22.3, _1.1);
_14 = _5 as u8;
_20 = _25.0 - _25.0;
_4.3 = _11;
Goto(bb14)
}
bb14 = {
_5 = 5860_u16 | 36377_u16;
_22.1 = _13.fld1;
_1.1 = _12.4 ^ _12.4;
_8 = _2;
_10.4 = core::ptr::addr_of!(_7);
_26 = !_11;
_20 = (-98_i8) as f32;
_2.3 = -_4.3;
_10.4 = core::ptr::addr_of!(RET);
_2 = _1;
_25.4 = _2.1 ^ _1.1;
_12.0 = -_25.0;
_25.3.1 = _25.3.0 & _22.3.0;
_25.3 = _12.3;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(4_usize, 7_usize, Move(_7), 1_usize, Move(_1), 14_usize, Move(_14), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 8_usize, Move(_8), 19_usize, Move(_19), 5_usize, Move(_5), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (i128, i32, i128, isize),mut _2: i128,mut _3: (i128, i32, i128, isize),mut _4: i128,mut _5: i128,mut _6: bool,mut _7: (i128, i32, i128, isize),mut _8: i128,mut _9: (i128, i32, i128, isize),mut _10: isize,mut _11: isize,mut _12: (i128, i32, i128, isize)) -> i32 {
mir! {
type RET = i32;
let _13: u128;
let _14: (i128, usize, &'static u128, [i128; 1], *const bool);
let _15: f32;
let _16: (i128, usize, &'static u128, [i128; 1], *const bool);
let _17: ();
let _18: ();
{
_9.0 = !_8;
_3.3 = (-55_i8) as isize;
_9.0 = 294395350177788518650536431022891084226_u128 as i128;
_3 = (_12.0, _12.1, _12.0, _7.3);
RET = _3.1 * _12.1;
_1 = _9;
_3 = (_9.2, RET, _5, _1.3);
_4 = _10 as i128;
_9.2 = (-127_i8) as i128;
_3.3 = -_12.3;
_12.0 = _2;
_6 = _12.2 != _1.2;
_12.2 = !_8;
_15 = 3023_u16 as f32;
_1.3 = !_10;
_7.1 = RET >> _3.2;
_1.2 = 5710_i16 as i128;
_1.2 = _12.2;
_3.0 = _5 << _7.1;
match _7.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
333032274728939429961403456358143638492 => bb7,
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
_14.2 = &_13;
_1.2 = _4 * _2;
_8 = (-1409672332405632224_i64) as i128;
_12.2 = _7.2;
_6 = true;
_16.1 = !11549285168424000644_usize;
_16.0 = !_3.2;
RET = _7.1 >> _3.0;
_16.2 = &_13;
_14.3 = [_16.0];
_5 = _7.0 - _7.0;
Goto(bb8)
}
bb8 = {
Call(_17 = dump_var(5_usize, 5_usize, Move(_5), 11_usize, Move(_11), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_17 = dump_var(5_usize, 2_usize, Move(_2), 3_usize, Move(_3), 18_usize, _18, 18_usize, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: [i128; 4],mut _3: [i128; 4],mut _4: [i128; 4],mut _5: [i128; 4],mut _6: [i128; 4],mut _7: [i128; 4],mut _8: [i128; 4],mut _9: bool,mut _10: i16,mut _11: [i128; 4]) -> i8 {
mir! {
type RET = i8;
let _12: i16;
let _13: (i128, i32, i128, isize);
let _14: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _15: Adt48;
let _16: char;
let _17: char;
let _18: [i128; 4];
let _19: Adt41;
let _20: Adt39;
let _21: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _22: char;
let _23: Adt48;
let _24: Adt39;
let _25: u32;
let _26: i128;
let _27: [bool; 6];
let _28: (u128, [i128; 4], (u8, u8));
let _29: isize;
let _30: usize;
let _31: (i128, usize, &'static u128, [i128; 1], *const bool);
let _32: u8;
let _33: [char; 5];
let _34: [i128; 4];
let _35: ();
let _36: ();
{
_7 = [18993039714587260010701507421399898344_i128,54066810046739658973886575887894170457_i128,31499410315497190610161536838091459399_i128,(-85839647605554795773059191711021806390_i128)];
RET = (-69_i8) * 90_i8;
_7 = [6593532880945948462677857201168590964_i128,85829152260386702446294696617313497510_i128,(-79793753435705897304619242159702152737_i128),(-84462113677025318133151096045285705330_i128)];
_2 = _11;
_7 = [157302923988648141151954422362929061809_i128,107394716607684549584567913110211106109_i128,136944360064655770740621167511518372678_i128,149128800157051642568230511109814368596_i128];
RET = !(-123_i8);
RET = 73_i8;
_1 = !_9;
_1 = _9;
Call(_1 = fn7(_4, _4, _2, _11, _6, _5, _7, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [(-142368123972138818360750447117652144280_i128),102227429424507779145525680952159428041_i128,60543448804020791087405894418712888767_i128,(-61833587103975193779142282770315657602_i128)];
_5 = _6;
_8 = [(-40734134567243042517582269935043843812_i128),(-81363290454990215682257854469045994226_i128),18922203151420121869358576505448361957_i128,37685843382334190018255125958783635609_i128];
_10 = 26910_i16 - 21083_i16;
_12 = !_10;
_6 = _2;
_10 = _12 >> _12;
_6 = [167561525121829820219574985876820559404_i128,(-56510221111566579639810866253376691673_i128),143524326414768142890237309211473445645_i128,(-160490095912830440216751122036688939318_i128)];
_7 = [(-141714804801826124362831348347432770557_i128),(-101343362698242831332331514302078156602_i128),(-142570507955488507373564893948935934982_i128),(-55154751622794872457190976016186983027_i128)];
_8 = _2;
_14.3.0 = 153_u8;
_11 = [(-117764737135044138833344928533255870853_i128),(-115226853105491373856477683428512894306_i128),(-70401134953051314939559010204996345752_i128),93254506920547624545349799616506563394_i128];
_14.0 = _12 as f32;
Call(_13.2 = fn10(_1, _14.3.0, _4, _14.3.0, _4, RET, _4, _11, _12, _2, _8, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [_13.2,_13.2,_13.2,_13.2];
_14.4 = _14.0 as i32;
_6 = [_13.2,_13.2,_13.2,_13.2];
_14.1 = [_13.2,_13.2,_13.2,_13.2];
_13.0 = _14.0 as i128;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
73 => bb8,
_ => bb7
}
}
bb3 = {
_8 = [(-142368123972138818360750447117652144280_i128),102227429424507779145525680952159428041_i128,60543448804020791087405894418712888767_i128,(-61833587103975193779142282770315657602_i128)];
_5 = _6;
_8 = [(-40734134567243042517582269935043843812_i128),(-81363290454990215682257854469045994226_i128),18922203151420121869358576505448361957_i128,37685843382334190018255125958783635609_i128];
_10 = 26910_i16 - 21083_i16;
_12 = !_10;
_6 = _2;
_10 = _12 >> _12;
_6 = [167561525121829820219574985876820559404_i128,(-56510221111566579639810866253376691673_i128),143524326414768142890237309211473445645_i128,(-160490095912830440216751122036688939318_i128)];
_7 = [(-141714804801826124362831348347432770557_i128),(-101343362698242831332331514302078156602_i128),(-142570507955488507373564893948935934982_i128),(-55154751622794872457190976016186983027_i128)];
_8 = _2;
_14.3.0 = 153_u8;
_11 = [(-117764737135044138833344928533255870853_i128),(-115226853105491373856477683428512894306_i128),(-70401134953051314939559010204996345752_i128),93254506920547624545349799616506563394_i128];
_14.0 = _12 as f32;
Call(_13.2 = fn10(_1, _14.3.0, _4, _14.3.0, _4, RET, _4, _11, _12, _2, _8, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
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
_1 = !_9;
_14.0 = _13.0 as f32;
_14.3 = (2_u8, 144_u8);
_6 = [_13.0,_13.2,_13.0,_13.0];
_14.3.0 = !_14.3.1;
RET = (-32_i8);
_2 = [_13.0,_13.0,_13.0,_13.0];
_14.2 = [_13.0,_13.0,_13.0,_13.2];
_5 = _4;
_7 = [_13.0,_13.2,_13.0,_13.0];
_4 = [_13.0,_13.0,_13.0,_13.2];
_14.4 = !1510532301_i32;
match _14.3.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
144 => bb10,
_ => bb9
}
}
bb9 = {
_8 = [_13.2,_13.2,_13.2,_13.2];
_14.4 = _14.0 as i32;
_6 = [_13.2,_13.2,_13.2,_13.2];
_14.1 = [_13.2,_13.2,_13.2,_13.2];
_13.0 = _14.0 as i128;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
73 => bb8,
_ => bb7
}
}
bb10 = {
RET = (-29_i8);
_13 = (44681211749787908801356351180300096132_i128, _14.4, 114646175876084942125941752466995526256_i128, (-40_isize));
_4 = [_13.0,_13.2,_13.0,_13.0];
_6 = [_13.0,_13.2,_13.0,_13.0];
_14.3 = (47_u8, 23_u8);
_13.3 = _14.3.1 as isize;
_14.3.1 = 40987_u16 as u8;
_13.1 = _14.4 + _14.4;
_14.3.0 = _13.1 as u8;
_14.0 = RET as f32;
_14.4 = !_13.1;
_14.0 = 9001723622617607609_i64 as f32;
_1 = !_9;
_14.3.1 = _14.3.0 << _13.0;
_13 = ((-154529020705741094677487923348422156007_i128), _14.4, 69282556521206599635375559242508108520_i128, 9223372036854775807_isize);
_5 = [_13.0,_13.2,_13.2,_13.0];
_13.1 = _14.4 - _14.4;
_4 = [_13.0,_13.0,_13.2,_13.0];
_18 = _5;
_17 = '\u{509ab}';
_10 = !_12;
_13.3 = (-9223372036854775808_isize);
Goto(bb11)
}
bb11 = {
_1 = _9 >= _9;
_21.3 = (_14.3.1, _14.3.1);
_21.3 = (_14.3.1, _14.3.1);
_14.3.1 = _21.3.1 + _14.3.0;
_10 = !_12;
_21.1 = _8;
_14.3.1 = 3091883345233381911_usize as u8;
_2 = _3;
_14.3.1 = _17 as u8;
_21.4 = -_14.4;
_18 = [_13.2,_13.2,_13.0,_13.2];
_25 = 3847702785_u32 | 4026567459_u32;
_12 = _10;
_14.3.0 = !_21.3.0;
_25 = 2346282335_u32 & 600594584_u32;
_14.0 = 8808951014771036379_i64 as f32;
_21.0 = _21.3.0 as f32;
_21 = (_14.0, _6, _18, _14.3, _13.1);
_21 = _14;
_3 = [_13.0,_13.2,_13.0,_13.2];
_10 = _12;
_11 = [_13.0,_13.2,_13.0,_13.0];
_14.3.0 = (-1442318764284666173_i64) as u8;
_14.3 = _21.3;
match _13.2 {
69282556521206599635375559242508108520 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_21.4 = _13.1 | _13.1;
_21.4 = _13.1 * _13.1;
_21.0 = _14.0 - _14.0;
_21.2 = [_13.2,_13.0,_13.0,_13.0];
_14.0 = -_21.0;
_21.3 = (_14.3.0, _14.3.0);
_21.3.0 = _17 as u8;
_31.3 = [_13.2];
_31.0 = _13.1 as i128;
_28.2.0 = _25 as u8;
_28.2.0 = _13.2 as u8;
_14.3 = (_28.2.0, _21.3.1);
_26 = _13.2 << _21.4;
_13 = (_26, _21.4, _31.0, (-9223372036854775808_isize));
_21.4 = -_13.1;
_27 = [_9,_1,_9,_1,_1,_1];
_31.2 = &_28.0;
_32 = _13.3 as u8;
Goto(bb14)
}
bb14 = {
_28.2.0 = _21.3.0;
_31.4 = core::ptr::addr_of!(_9);
_16 = _17;
_3 = _21.2;
_33 = [_16,_16,_17,_17,_17];
_29 = _13.3;
_1 = !_9;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(6_usize, 7_usize, Move(_7), 16_usize, Move(_16), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(6_usize, 5_usize, Move(_5), 26_usize, Move(_26), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(6_usize, 11_usize, Move(_11), 32_usize, Move(_32), 27_usize, Move(_27), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i128; 4],mut _2: [i128; 4],mut _3: [i128; 4],mut _4: [i128; 4],mut _5: [i128; 4],mut _6: [i128; 4],mut _7: [i128; 4],mut _8: [i128; 4]) -> bool {
mir! {
type RET = bool;
let _9: Adt48;
let _10: Adt38;
let _11: i16;
let _12: Adt41;
let _13: u128;
let _14: f64;
let _15: f32;
let _16: bool;
let _17: [char; 5];
let _18: f32;
let _19: *mut u8;
let _20: Adt46;
let _21: [u128; 4];
let _22: [char; 7];
let _23: [i128; 4];
let _24: Adt37;
let _25: Adt44;
let _26: (i32, usize, (i16, i64, i8, &'static u128, u64, (i128, i32, i128, isize)), usize);
let _27: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _28: f64;
let _29: Adt49;
let _30: f64;
let _31: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _32: char;
let _33: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _34: (i32, usize, (i16, i64, i8, &'static u128, u64, (i128, i32, i128, isize)), usize);
let _35: ();
let _36: ();
{
RET = !false;
RET = !false;
RET = true | false;
Goto(bb1)
}
bb1 = {
_11 = 27758_i16;
_11 = (-7577277965137317256_i64) as i16;
RET = false;
_5 = [73474788177524015650966829705668452902_i128,(-134459229086795433048162286142609368901_i128),70899525118378790576159110252372519002_i128,(-86856958872455022180301363231508697988_i128)];
_10.fld2 = !1907461971_u32;
RET = _10.fld2 > _10.fld2;
_10.fld1 = _2;
RET = false;
RET = true & true;
_3 = [(-8792752500076620962626951668492874946_i128),(-168614709993190249732144238059405029294_i128),(-107331070523426120192241955763566101182_i128),(-2860276119473812539238347065806887841_i128)];
_8 = [(-100196793100631037858934732481025216147_i128),(-122783444494990372597721174442297035188_i128),(-161908326928524359621314815903840863112_i128),(-67599435544091951951020242384585942679_i128)];
_8 = [66009585676647151088188032294712100035_i128,19611324115538151077738548504421014914_i128,64637280180128387887522758698037243107_i128,121973714394170441853740527464494880367_i128];
_14 = 2073484463009934986_usize as f64;
_5 = [(-6071203604860620715161410919739973907_i128),15401514141375436301462591021019904865_i128,(-52337345374909332991034502067455035868_i128),95558252570719814085950751475973549799_i128];
_10.fld0 = (-158405334956648657201487049843107741047_i128) as u8;
RET = !false;
_13 = !210133744438887476922276868997020165248_u128;
_8 = [(-157535292878784306838764944565438656327_i128),(-52651654866332258366516789010930894902_i128),126778821678377757708506618781477423267_i128,11694022018456264560528727042480505379_i128];
RET = !true;
_10.fld2 = !4262188921_u32;
_8 = [(-168652028270639760144517646476909881888_i128),31211451728841430637606602210916319885_i128,(-2717762338861729009204394449381824820_i128),(-77880333944837178005264906152769601829_i128)];
_13 = 1741181771_i32 as u128;
_15 = (-50723896148261812092276170372814546719_i128) as f32;
_15 = (-1846688780_i32) as f32;
Goto(bb2)
}
bb2 = {
_1 = [(-21383356487328807388145458717973247489_i128),(-12792165414810904176934703489706009257_i128),117076672131005150036179899857241632398_i128,(-12327906025744959703256695398365377695_i128)];
_15 = 109847461229608632555602136239090180244_i128 as f32;
_14 = _11 as f64;
_3 = [139066322424348513673058637816914378738_i128,5111235390196904582670530421670015031_i128,168810223945719772318764739017338380218_i128,146639312232117915996246556506684054763_i128];
_5 = [(-55679909256139724716661470014573360212_i128),91600772647255758555315845411195511287_i128,(-51956531751817440501823355651404109147_i128),(-160651668604077339153048391858347732194_i128)];
_13 = !220365164721602968352214130489215089835_u128;
RET = true;
_15 = _11 as f32;
_7 = _3;
RET = !false;
_13 = !57352725572091625622976838498665165713_u128;
_17 = ['\u{cc2ad}','\u{45d1a}','\u{48388}','\u{62837}','\u{194d8}'];
_2 = [(-71888297690905689715782262551468031261_i128),169711493613253577934752866026850194325_i128,(-146813732598198570212615249992478335050_i128),(-55039241026357137495645000967248736586_i128)];
_18 = _15 + _15;
_11 = -30767_i16;
RET = _15 > _15;
Call(_7 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _10.fld2 < _10.fld2;
_10 = Adt38 { fld0: 186_u8,fld1: _4,fld2: 2950714544_u32 };
_10 = Adt38 { fld0: 120_u8,fld1: _4,fld2: 1740817615_u32 };
_19 = core::ptr::addr_of_mut!(_10.fld0);
_10.fld2 = _11 as u32;
_10.fld0 = !9_u8;
_7 = [(-66658471752515670659934779297050799529_i128),(-13561064430999279573056871893512783272_i128),(-134251795889890434783622935736331419517_i128),(-168387918428242108044282883037188344493_i128)];
_20 = Adt46::Variant0 { fld0: 14967_u16 };
_10.fld2 = 1265718334_u32 << (*_19);
_21 = [_13,_13,_13,_13];
_10 = Adt38 { fld0: 221_u8,fld1: _6,fld2: 90900966_u32 };
_20 = Adt46::Variant0 { fld0: 2262_u16 };
Goto(bb4)
}
bb4 = {
RET = true;
_9 = Adt48::Variant0 { fld0: (-23650693443520065185372448577593669055_i128),fld1: _21 };
_10.fld2 = (-4871784884346478984489803619580093233_i128) as u32;
_1 = [28258528084473919817124720495894298985_i128,(-90638041604322477131199340606177695444_i128),81759795853935092443681039509219813937_i128,(-71963760262417960273001483134988200802_i128)];
_21 = Field::<[u128; 4]>(Variant(_9, 0), 1);
RET = !true;
_23 = [(-49165515844291238745175412146786539754_i128),(-142511532123520401852519422365034364802_i128),(-158652970484140123911604792002359959043_i128),58543796529923622654262553329116532581_i128];
_22 = ['\u{8f111}','\u{c8814}','\u{de175}','\u{4b4a5}','\u{1060d5}','\u{ebf47}','\u{90bea}'];
_2 = [(-58734035676299343595947096247880460732_i128),130106650310665405330213490553936154911_i128,23433226499019755465978853580400413168_i128,167104333658883539987742331349583812533_i128];
_17 = ['\u{bb8f6}','\u{91161}','\u{a3d4a}','\u{f3fe7}','\u{b8786}'];
match (*_19) {
0 => bb2,
1 => bb5,
221 => bb7,
_ => bb6
}
}
bb5 = {
_11 = 27758_i16;
_11 = (-7577277965137317256_i64) as i16;
RET = false;
_5 = [73474788177524015650966829705668452902_i128,(-134459229086795433048162286142609368901_i128),70899525118378790576159110252372519002_i128,(-86856958872455022180301363231508697988_i128)];
_10.fld2 = !1907461971_u32;
RET = _10.fld2 > _10.fld2;
_10.fld1 = _2;
RET = false;
RET = true & true;
_3 = [(-8792752500076620962626951668492874946_i128),(-168614709993190249732144238059405029294_i128),(-107331070523426120192241955763566101182_i128),(-2860276119473812539238347065806887841_i128)];
_8 = [(-100196793100631037858934732481025216147_i128),(-122783444494990372597721174442297035188_i128),(-161908326928524359621314815903840863112_i128),(-67599435544091951951020242384585942679_i128)];
_8 = [66009585676647151088188032294712100035_i128,19611324115538151077738548504421014914_i128,64637280180128387887522758698037243107_i128,121973714394170441853740527464494880367_i128];
_14 = 2073484463009934986_usize as f64;
_5 = [(-6071203604860620715161410919739973907_i128),15401514141375436301462591021019904865_i128,(-52337345374909332991034502067455035868_i128),95558252570719814085950751475973549799_i128];
_10.fld0 = (-158405334956648657201487049843107741047_i128) as u8;
RET = !false;
_13 = !210133744438887476922276868997020165248_u128;
_8 = [(-157535292878784306838764944565438656327_i128),(-52651654866332258366516789010930894902_i128),126778821678377757708506618781477423267_i128,11694022018456264560528727042480505379_i128];
RET = !true;
_10.fld2 = !4262188921_u32;
_8 = [(-168652028270639760144517646476909881888_i128),31211451728841430637606602210916319885_i128,(-2717762338861729009204394449381824820_i128),(-77880333944837178005264906152769601829_i128)];
_13 = 1741181771_i32 as u128;
_15 = (-50723896148261812092276170372814546719_i128) as f32;
_15 = (-1846688780_i32) as f32;
Goto(bb2)
}
bb6 = {
_1 = [(-21383356487328807388145458717973247489_i128),(-12792165414810904176934703489706009257_i128),117076672131005150036179899857241632398_i128,(-12327906025744959703256695398365377695_i128)];
_15 = 109847461229608632555602136239090180244_i128 as f32;
_14 = _11 as f64;
_3 = [139066322424348513673058637816914378738_i128,5111235390196904582670530421670015031_i128,168810223945719772318764739017338380218_i128,146639312232117915996246556506684054763_i128];
_5 = [(-55679909256139724716661470014573360212_i128),91600772647255758555315845411195511287_i128,(-51956531751817440501823355651404109147_i128),(-160651668604077339153048391858347732194_i128)];
_13 = !220365164721602968352214130489215089835_u128;
RET = true;
_15 = _11 as f32;
_7 = _3;
RET = !false;
_13 = !57352725572091625622976838498665165713_u128;
_17 = ['\u{cc2ad}','\u{45d1a}','\u{48388}','\u{62837}','\u{194d8}'];
_2 = [(-71888297690905689715782262551468031261_i128),169711493613253577934752866026850194325_i128,(-146813732598198570212615249992478335050_i128),(-55039241026357137495645000967248736586_i128)];
_18 = _15 + _15;
_11 = -30767_i16;
RET = _15 > _15;
Call(_7 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_5 = _8;
_21 = [_13,_13,_13,_13];
_4 = [(-166484669935042021058933781104041449787_i128),44644792648854142360799809822786762544_i128,(-30764759957406355199225592864616798724_i128),(-121301508652375590292972835384245291233_i128)];
_1 = [(-122897833218057207249297489035571213559_i128),(-89660005321617306128032919361684546448_i128),(-14494683600039190644681210170774413533_i128),36095756024485447002342544317086462621_i128];
RET = false | false;
_10.fld0 = 24799_u16 as u8;
_21 = [_13,_13,_13,_13];
_18 = 6306434041583777697_usize as f32;
_10 = Adt38 { fld0: 214_u8,fld1: _6,fld2: 1251900212_u32 };
_10 = Adt38 { fld0: 175_u8,fld1: _8,fld2: 3714016748_u32 };
_19 = core::ptr::addr_of_mut!(_10.fld0);
_16 = !RET;
place!(Field::<i128>(Variant(_9, 0), 0)) = '\u{5d221}' as i128;
place!(Field::<[u128; 4]>(Variant(_9, 0), 1)) = _21;
RET = _16;
_7 = _6;
_1 = _6;
_19 = core::ptr::addr_of_mut!(_10.fld0);
place!(Field::<[u128; 4]>(Variant(_9, 0), 1)) = _21;
place!(Field::<u16>(Variant(_20, 0), 0)) = 64088_u16;
place!(Field::<i128>(Variant(_9, 0), 0)) = 85670599172367908275195205176268723678_i128 & (-121794878428238450549624556885740700673_i128);
_15 = _10.fld2 as f32;
_2 = [Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0)];
_10.fld0 = _11 as u8;
RET = _16 | _16;
_14 = 111_i8 as f64;
_6 = _1;
_11 = !32463_i16;
_3 = [Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0)];
_4 = [Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0)];
Goto(bb8)
}
bb8 = {
_26.3 = !1_usize;
_4 = _7;
_10 = Adt38 { fld0: 51_u8,fld1: _4,fld2: 59900905_u32 };
_19 = core::ptr::addr_of_mut!((*_19));
_26.2.5 = (Field::<i128>(Variant(_9, 0), 0), (-1694158516_i32), Field::<i128>(Variant(_9, 0), 0), (-9223372036854775808_isize));
SetDiscriminant(_9, 1);
_26.2.0 = _11;
_27.4 = _26.2.5.1 * _26.2.5.1;
Call(place!(Field::<(u128, [i128; 4], (u8, u8))>(Variant(_9, 1), 2)).1 = fn8(Move(_10), _15, _19, _26.2.5.0, _26.2.5, _26.2.5.3, _4, _26.2.5.3, _19, _26.2.5.1, _26.2.5.3, _19, _26.2.5.0, _1, _19, _23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<(u128, [i128; 4], (u8, u8))>(Variant(_9, 1), 2)).2 = (106_u8, 165_u8);
_2 = [_26.2.5.2,_26.2.5.0,_26.2.5.2,_26.2.5.0];
RET = _15 == _15;
_27.3.1 = Field::<(u128, [i128; 4], (u8, u8))>(Variant(_9, 1), 2).2.1;
_9 = Adt48::Variant0 { fld0: _26.2.5.0,fld1: _21 };
_28 = _14 * _14;
SetDiscriminant(_9, 0);
_10.fld0 = _27.3.1 | _27.3.1;
_27.1 = [_26.2.5.0,_26.2.5.2,_26.2.5.0,_26.2.5.2];
place!(Field::<[u128; 4]>(Variant(_9, 0), 1)) = [_13,_13,_13,_13];
_26.2.5.3 = _26.2.5.0 as isize;
_2 = [_26.2.5.2,_26.2.5.0,_26.2.5.2,_26.2.5.2];
_26.3 = !0_usize;
_27.3.1 = _11 as u8;
_26.1 = !_26.3;
_26.2.3 = &_13;
_31.4 = 17692778099980994094_u64 as i32;
place!(Field::<i128>(Variant(_9, 0), 0)) = -_26.2.5.0;
(*_19) = _26.3 as u8;
_26.2.4 = 9062096901886877191_u64 >> (*_19);
_27.3.0 = Field::<u16>(Variant(_20, 0), 0) as u8;
_26.2.2 = !(-21_i8);
_10.fld2 = !2231932696_u32;
Goto(bb10)
}
bb10 = {
Call(_35 = dump_var(7_usize, 4_usize, Move(_4), 13_usize, Move(_13), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_35 = dump_var(7_usize, 22_usize, Move(_22), 23_usize, Move(_23), 17_usize, Move(_17), 36_usize, _36), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: Adt38,mut _2: f32,mut _3: *mut u8,mut _4: i128,mut _5: (i128, i32, i128, isize),mut _6: isize,mut _7: [i128; 4],mut _8: isize,mut _9: *mut u8,mut _10: i32,mut _11: isize,mut _12: *mut u8,mut _13: i128,mut _14: [i128; 4],mut _15: *mut u8,mut _16: [i128; 4]) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _17: u32;
let _18: usize;
let _19: (u128, [i128; 4], (u8, u8));
let _20: u16;
let _21: [i128; 4];
let _22: Adt53;
let _23: Adt37;
let _24: isize;
let _25: [bool; 8];
let _26: isize;
let _27: i32;
let _28: ();
let _29: ();
{
_5 = (_13, _10, _4, _11);
_4 = _5.0;
RET = _7;
_3 = core::ptr::addr_of_mut!(_1.fld0);
_14 = [_5.2,_5.0,_5.0,_4];
_1.fld2 = 2907970519_u32;
_4 = (-4380169221002826276_i64) as i128;
_1.fld2 = !1700558033_u32;
_3 = core::ptr::addr_of_mut!((*_3));
_5.0 = _13 + _13;
_1.fld0 = 135_u8 | 63_u8;
RET = [_5.0,_4,_5.0,_5.0];
_5 = (_13, _10, _13, _11);
_15 = _9;
Call(_6 = fn9(_14, _15, _5.3, _15, _5.3, _5, _5.1, _9, _12, _15, _1.fld1, _3, _15, _12, Move(_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _5.1;
_17 = 11255190927694624057_u64 as u32;
_1.fld0 = !168_u8;
_5.3 = !_6;
_5 = (_4, _10, _13, _11);
_18 = '\u{30220}' as usize;
_5.3 = _6;
_1 = Adt38 { fld0: 183_u8,fld1: _14,fld2: _17 };
_11 = _8 << _5.1;
_18 = 332686378054237886895854127350843622451_u128 as usize;
_1.fld0 = _6 as u8;
_13 = _5.0;
_16 = _7;
RET = [_5.2,_5.2,_4,_5.2];
_3 = core::ptr::addr_of_mut!((*_3));
RET = [_5.0,_4,_4,_13];
_12 = _15;
(*_3) = 79_u8 + 96_u8;
_19.1 = _7;
_14 = _16;
_19.1 = _7;
_3 = core::ptr::addr_of_mut!(_19.2.0);
Goto(bb2)
}
bb2 = {
_5.2 = !_5.0;
_17 = 83_i8 as u32;
(*_3) = _17 as u8;
_9 = _12;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607430074052940 => bb10,
_ => bb9
}
}
bb3 = {
_10 = _5.1;
_17 = 11255190927694624057_u64 as u32;
_1.fld0 = !168_u8;
_5.3 = !_6;
_5 = (_4, _10, _13, _11);
_18 = '\u{30220}' as usize;
_5.3 = _6;
_1 = Adt38 { fld0: 183_u8,fld1: _14,fld2: _17 };
_11 = _8 << _5.1;
_18 = 332686378054237886895854127350843622451_u128 as usize;
_1.fld0 = _6 as u8;
_13 = _5.0;
_16 = _7;
RET = [_5.2,_5.2,_4,_5.2];
_3 = core::ptr::addr_of_mut!((*_3));
RET = [_5.0,_4,_4,_13];
_12 = _15;
(*_3) = 79_u8 + 96_u8;
_19.1 = _7;
_14 = _16;
_19.1 = _7;
_3 = core::ptr::addr_of_mut!(_19.2.0);
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
_18 = !2_usize;
_17 = _1.fld2 | _1.fld2;
_11 = -_6;
_5.3 = !_6;
_9 = _12;
RET = [_5.0,_5.2,_5.0,_4];
_9 = _15;
_15 = core::ptr::addr_of_mut!((*_3));
_20 = !26211_u16;
match _10 {
340282366920938463463374607430074052940 => bb12,
_ => bb11
}
}
bb11 = {
_5.2 = !_5.0;
_17 = 83_i8 as u32;
(*_3) = _17 as u8;
_9 = _12;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607430074052940 => bb10,
_ => bb9
}
}
bb12 = {
_5 = (_13, _10, _4, _6);
match _10 {
0 => bb1,
1 => bb11,
2 => bb9,
3 => bb4,
4 => bb7,
340282366920938463463374607430074052940 => bb13,
_ => bb6
}
}
bb13 = {
_5.0 = 299208118347114160856546669269077369908_u128 as i128;
_1.fld2 = _18 as u32;
_21 = _7;
_19.2 = (_1.fld0, _1.fld0);
_10 = _5.1;
_15 = core::ptr::addr_of_mut!((*_3));
_18 = 1_usize & 6_usize;
_5.2 = _13 | _4;
_5.1 = !_10;
_5.0 = _5.2 * _13;
_1.fld1 = [_4,_5.2,_4,_5.0];
_10 = _5.1 * _5.1;
_19.1 = [_5.0,_5.2,_13,_5.0];
RET = _16;
(*_3) = _1.fld0 - _19.2.1;
_1.fld1 = [_5.2,_5.2,_5.0,_5.0];
_10 = _20 as i32;
_13 = _5.2 | _5.2;
_16 = [_5.0,_4,_13,_13];
_2 = _5.3 as f32;
RET = [_13,_5.0,_5.0,_13];
_6 = _11;
_1.fld1 = [_5.2,_4,_13,_13];
_8 = _2 as isize;
RET = [_5.0,_5.0,_13,_4];
_21 = [_13,_13,_13,_13];
Goto(bb14)
}
bb14 = {
_1.fld1 = [_13,_13,_13,_13];
_6 = _11 + _5.3;
RET = [_5.0,_13,_5.2,_13];
_19.2.0 = !_1.fld0;
_9 = _3;
_1 = Adt38 { fld0: (*_9),fld1: _14,fld2: _17 };
(*_15) = _1.fld0;
_5.2 = !_5.0;
_26 = !_5.3;
_5.0 = _20 as i128;
RET = _1.fld1;
_10 = _5.1;
_5.1 = _10;
_5.2 = _4;
_19.2.0 = _19.2.1;
_15 = core::ptr::addr_of_mut!((*_3));
_5.0 = _4;
_5 = (_4, _10, _4, _26);
_15 = _12;
_26 = _5.3;
_27 = _10 ^ _10;
_26 = _5.3;
_1.fld0 = (*_3);
_24 = -_5.3;
_16 = RET;
_9 = core::ptr::addr_of_mut!(_1.fld0);
_5.2 = _13;
_25 = [true,true,false,true,true,true,true,true];
_23 = Adt37::Variant1 { fld0: 116749271787386626129778421396386143905_u128,fld1: '\u{dea7f}',fld2: _25 };
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(8_usize, 11_usize, Move(_11), 21_usize, Move(_21), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(8_usize, 6_usize, Move(_6), 13_usize, Move(_13), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(8_usize, 25_usize, Move(_25), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i128; 4],mut _2: *mut u8,mut _3: isize,mut _4: *mut u8,mut _5: isize,mut _6: (i128, i32, i128, isize),mut _7: i32,mut _8: *mut u8,mut _9: *mut u8,mut _10: *mut u8,mut _11: [i128; 4],mut _12: *mut u8,mut _13: *mut u8,mut _14: *mut u8,mut _15: Adt38) -> isize {
mir! {
type RET = isize;
let _16: ();
let _17: ();
{
_9 = core::ptr::addr_of_mut!(_15.fld0);
_6.1 = _7;
RET = 4425927314985010164_usize as isize;
_8 = _13;
_13 = core::ptr::addr_of_mut!((*_9));
_9 = core::ptr::addr_of_mut!((*_9));
_3 = _6.3;
_4 = _14;
Goto(bb1)
}
bb1 = {
_7 = _6.1;
_6.2 = _6.0;
(*_9) = 98_u8 * 208_u8;
RET = _6.3 << _6.3;
_7 = !_6.1;
_15.fld0 = 215_u8;
_13 = core::ptr::addr_of_mut!((*_9));
_6.2 = !_6.0;
(*_13) = (-816930067857595423_i64) as u8;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(9_usize, 6_usize, Move(_6), 7_usize, Move(_7), 3_usize, Move(_3), 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: bool,mut _2: u8,mut _3: [i128; 4],mut _4: u8,mut _5: [i128; 4],mut _6: i8,mut _7: [i128; 4],mut _8: [i128; 4],mut _9: i16,mut _10: [i128; 4],mut _11: [i128; 4],mut _12: [i128; 4],mut _13: [i128; 4],mut _14: [i128; 4]) -> i128 {
mir! {
type RET = i128;
let _15: Adt39;
let _16: *const i128;
let _17: Adt41;
let _18: (i128, usize, &'static u128, [i128; 1], *const bool);
let _19: f64;
let _20: isize;
let _21: f32;
let _22: (f32, [i128; 4], [i128; 4], (u8, u8), i32);
let _23: Adt47;
let _24: bool;
let _25: (u128, [i128; 4], (u8, u8));
let _26: [u128; 4];
let _27: ();
let _28: ();
{
_11 = _7;
_4 = _2;
_5 = [(-112084858070397056399381099710656996521_i128),120588754402175383431365547310462591913_i128,127077580817194575786777973798883245079_i128,84932783336070619136816785578340445369_i128];
RET = 300374589537724387370388443078303739722_u128 as i128;
RET = _9 as i128;
_12 = [RET,RET,RET,RET];
_8 = _13;
RET = 118786394921162445070424209539000414329_i128 + (-121382632474655936248611472909512340934_i128);
_9 = 336716455078322729670338643283279324995_u128 as i16;
_8 = [RET,RET,RET,RET];
_12 = [RET,RET,RET,RET];
_18.0 = RET & RET;
_8 = [RET,RET,_18.0,RET];
_18.4 = core::ptr::addr_of!(_1);
_2 = _4;
_18.3 = [_18.0];
_16 = core::ptr::addr_of!(RET);
_8 = [(*_16),_18.0,(*_16),_18.0];
_6 = 17286_u16 as i8;
_18.4 = core::ptr::addr_of!(_1);
_4 = !_2;
match _2 {
153 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = 319414551318821726755946265133429716704_u128 as i128;
RET = _18.0;
_22.3 = (_4, _2);
_1 = RET == _18.0;
_8 = [(*_16),(*_16),_18.0,RET];
_22.4 = !1931021835_i32;
_18.4 = core::ptr::addr_of!(_1);
_22.4 = 1778842354_i32;
_4 = _22.3.0 - _2;
_22.0 = _22.4 as f32;
_18.1 = 7321_u16 as usize;
_19 = _22.0 as f64;
_21 = -_22.0;
_14 = [_18.0,RET,(*_16),(*_16)];
_22.0 = 308195232416316046026068722492487761157_u128 as f32;
_22.2 = [_18.0,RET,(*_16),(*_16)];
_22.2 = [(*_16),RET,(*_16),(*_16)];
_9 = !(-23542_i16);
_22.1 = _7;
_24 = !_1;
_14 = [(*_16),(*_16),(*_16),(*_16)];
RET = 16158113522985838976_u64 as i128;
_18.0 = RET - RET;
_14 = [_18.0,(*_16),(*_16),RET];
_18.3 = [(*_16)];
match _22.4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
1778842354 => bb9,
_ => bb8
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
_11 = _7;
_6 = 4_i8;
_3 = _22.1;
_14 = _7;
_2 = !_22.3.0;
Goto(bb10)
}
bb10 = {
_25.2.0 = '\u{8ff6}' as u8;
_18.2 = &_25.0;
match _22.4 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
1778842354 => bb17,
_ => bb16
}
}
bb11 = {
_11 = _7;
_6 = 4_i8;
_3 = _22.1;
_14 = _7;
_2 = !_22.3.0;
Goto(bb10)
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
_6 = (-102_i8);
_18.2 = &_25.0;
_22.3 = (_25.2.0, _2);
_18.2 = &_25.0;
_22.0 = _21 - _21;
RET = _21 as i128;
_18.4 = core::ptr::addr_of!(_1);
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(10_usize, 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(10_usize, 12_usize, Move(_12), 9_usize, Move(_9), 2_usize, Move(_2), 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: u8,mut _2: u8,mut _3: u8,mut _4: (u8, u8),mut _5: [i128; 4],mut _6: (u8, u8),mut _7: (u8, u8),mut _8: (u8, u8),mut _9: (u8, u8),mut _10: Adt38,mut _11: u8,mut _12: u8,mut _13: isize,mut _14: u8,mut _15: (i128, i32, i128, isize)) -> f32 {
mir! {
type RET = f32;
let _16: i16;
let _17: f32;
let _18: i128;
let _19: [char; 5];
let _20: bool;
let _21: bool;
let _22: ();
let _23: ();
{
_15 = (147989726431307065319625611503792376375_i128, 410752920_i32, (-65087071288627023961761291808161136750_i128), _13);
_10 = Adt38 { fld0: _11,fld1: _5,fld2: 1908857438_u32 };
_4.0 = _1 << _9.0;
RET = _3 as f32;
_19 = ['\u{4b070}','\u{6ff5f}','\u{902bf}','\u{aa669}','\u{6dd3e}'];
_7.1 = !_3;
_8 = (_10.fld0, _7.1);
_17 = -RET;
_7.1 = _15.2 as u8;
_6.1 = true as u8;
_4.0 = !_8.0;
_4 = (_9.0, _7.1);
_17 = -RET;
_10.fld2 = 421775817_u32;
_15.1 = !1735388577_i32;
_7 = (_6.0, _9.0);
_15.3 = true as isize;
_4.1 = 285410683916435957838416966072423840591_u128 as u8;
_11 = _14;
_8 = (_14, _7.0);
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(11_usize, 1_usize, Move(_1), 19_usize, Move(_19), 8_usize, Move(_8), 11_usize, Move(_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(11_usize, 13_usize, Move(_13), 5_usize, Move(_5), 7_usize, Move(_7), 23_usize, _23), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(23554_u16), std::hint::black_box((-19257_i16)));
                
            }
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt37::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: bool,
fld1: i128,
fld2: ([bool; 8], i16, *mut u8),
fld3: u16,
fld4: i32,

},
Variant1{
fld0: u128,
fld1: char,
fld2: [bool; 8],

},
Variant2{
fld0: [i128; 4],
fld1: u8,
fld2: isize,
fld3: (u8, u8),

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: u8,
fld1: [i128; 4],
fld2: u32,
}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: [i128; 4],
fld1: ([bool; 8], i16, *mut u8),
fld2: i64,
fld3: i128,

},
Variant1{
fld0: i64,
fld1: (i128, i32, i128, isize),
fld2: isize,
fld3: Adt37,

},
Variant2{
fld0: f32,
fld1: (i128, i32, i128, isize),
fld2: i128,
fld3: Adt37,
fld4: [char; 7],

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: [i128; 2],

},
Variant1{
fld0: f64,
fld1: char,
fld2: Adt38,
fld3: i128,
fld4: Adt39,
fld5: (u8, u8),
fld6: [u128; 4],

},
Variant2{
fld0: *const bool,
fld1: [i128; 1],
fld2: usize,
fld3: f64,
fld4: *mut u8,
fld5: i32,
fld6: [i128; 4],
fld7: u16,

},
Variant3{
fld0: [bool; 8],
fld1: char,
fld2: u64,
fld3: [i128; 2],
fld4: ([bool; 8], i16, *mut u8),
fld5: f32,
fld6: [u128; 4],

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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *const i128,
fld1: Adt39,
fld2: u32,
fld3: [bool; 8],
fld4: *const bool,
fld5: [char; 7],
fld6: Adt37,

},
Variant1{
fld0: Adt39,
fld1: (u8, u8),

},
Variant2{
fld0: i128,
fld1: Adt39,

},
Variant3{
fld0: (u8, u8),
fld1: [bool; 6],
fld2: [i128; 2],
fld3: *const bool,
fld4: Adt37,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: Adt38,
fld1: u16,
fld2: i128,
fld3: [char; 7],
fld4: [i128; 1],

},
Variant1{
fld0: usize,
fld1: char,

},
Variant2{
fld0: bool,
fld1: *mut u8,
fld2: Adt39,
fld3: i8,
fld4: u32,
fld5: (f32, [i128; 4], [i128; 4], (u8, u8), i32),
fld6: f32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt41,
fld3: Adt40,
fld4: u32,

},
Variant1{
fld0: [i128; 1],

},
Variant2{
fld0: [i128; 1],
fld1: u64,
fld2: Adt42,
fld3: Adt37,
fld4: ([bool; 8], i16, *mut u8),
fld5: (f32, [i128; 4], [i128; 4], (u8, u8), i32),
fld6: (i128, i32, i128, isize),

},
Variant3{
fld0: Adt41,
fld1: [char; 7],
fld2: isize,
fld3: Adt42,
fld4: (u8, u8),
fld5: f32,
fld6: u8,
fld7: i128,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: *const i128,
fld2: usize,
fld3: Adt40,
fld4: (u8, u8),

},
Variant1{
fld0: [i128; 1],
fld1: [i128; 2],
fld2: [bool; 6],
fld3: *const i128,
fld4: (u128, [i128; 4], (u8, u8)),
fld5: *const bool,
fld6: [u128; 4],
fld7: Adt42,

},
Variant2{
fld0: [bool; 8],
fld1: u16,
fld2: (u8, u8),
fld3: Adt43,

},
Variant3{
fld0: bool,
fld1: (u128, [i128; 4], (u8, u8)),
fld2: Adt41,
fld3: [u128; 4],
fld4: u16,
fld5: (i128, i32, i128, isize),
fld6: [bool; 8],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut u8,
fld1: (u128, [i128; 4], (u8, u8)),
fld2: Adt42,
fld3: i8,

},
Variant1{
fld0: u64,
fld1: [char; 5],
fld2: isize,
fld3: Adt42,
fld4: (u8, u8),
fld5: u16,
fld6: [bool; 8],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u16,

},
Variant1{
fld0: f32,
fld1: f64,
fld2: [char; 5],
fld3: u32,
fld4: Adt40,
fld5: usize,
fld6: u128,
fld7: Adt44,

},
Variant2{
fld0: bool,
fld1: f64,
fld2: [bool; 6],
fld3: (u8, u8),

},
Variant3{
fld0: (i128, i32, i128, isize),
fld1: Adt45,
fld2: Adt38,
fld3: *const i128,
fld4: (f32, [i128; 4], [i128; 4], (u8, u8), i32),
fld5: [i128; 2],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt42,
fld1: char,
fld2: (u128, [i128; 4], (u8, u8)),
fld3: [bool; 6],
fld4: *const i128,
fld5: i32,
fld6: f64,
fld7: i128,

},
Variant1{
fld0: [bool; 8],
fld1: (u8, u8),
fld2: u32,
fld3: u64,
fld4: u8,

},
Variant2{
fld0: usize,

},
Variant3{
fld0: (u8, u8),
fld1: u64,
fld2: u32,
fld3: i128,
fld4: (i128, i32, i128, isize),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: i128,
fld1: [u128; 4],

},
Variant1{
fld0: [bool; 8],
fld1: u32,
fld2: (u128, [i128; 4], (u8, u8)),
fld3: [i128; 1],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u128,
fld1: Adt45,
fld2: Adt38,
fld3: Adt39,
fld4: (i128, i32, i128, isize),
fld5: *const bool,
fld6: i64,

},
Variant1{
fld0: Adt40,
fld1: char,
fld2: Adt46,
fld3: Adt42,
fld4: f64,
fld5: Adt44,
fld6: i64,
fld7: Adt47,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [bool; 6],
fld1: (i128, i32, i128, isize),
fld2: u16,
fld3: Adt45,
fld4: (u8, u8),
fld5: (f32, [i128; 4], [i128; 4], (u8, u8), i32),
fld6: Adt37,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt50,
fld1: [i128; 1],
fld2: Adt40,

},
Variant1{
fld0: bool,
fld1: char,
fld2: Adt37,
fld3: *const bool,
fld4: Adt39,
fld5: u64,
fld6: f64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [bool; 8],
fld1: char,
fld2: Adt39,
fld3: Adt42,
fld4: u8,
fld5: usize,

},
Variant1{
fld0: (u8, u8),
fld1: *mut u8,
fld2: isize,
fld3: (i128, i32, i128, isize),
fld4: [u128; 4],
fld5: (u128, [i128; 4], (u8, u8)),

},
Variant2{
fld0: Adt51,
fld1: Adt37,
fld2: u16,

},
Variant3{
fld0: i32,
fld1: Adt51,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: usize,
fld1: char,
fld2: Adt40,
fld3: [i128; 1],
fld4: f32,
fld5: Adt37,
fld6: [i128; 2],
fld7: ([bool; 8], i16, *mut u8),

},
Variant1{
fld0: bool,
fld1: Adt48,
fld2: Adt46,
fld3: [bool; 8],
fld4: i16,
fld5: Adt50,
fld6: [i128; 1],

},
Variant2{
fld0: Adt40,
fld1: Adt49,

}}

