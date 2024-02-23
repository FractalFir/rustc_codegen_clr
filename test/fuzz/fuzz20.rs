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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: u64,mut _4: usize,mut _5: i16,mut _6: i32,mut _7: u16,mut _8: i128) -> isize {
mir! {
type RET = isize;
let _9: Adt49;
let _10: Adt46;
let _11: Adt57;
let _12: isize;
let _13: [i128; 5];
let _14: Adt49;
let _15: bool;
let _16: [i128; 5];
let _17: i16;
let _18: f64;
let _19: Adt54;
let _20: f64;
let _21: u128;
let _22: [i128; 7];
let _23: i64;
let _24: u128;
let _25: usize;
let _26: isize;
let _27: f64;
let _28: isize;
let _29: i128;
let _30: usize;
let _31: (i16, u16);
let _32: (i64, i32, u32, i8);
let _33: ();
let _34: ();
{
_3 = 9174739924480904989_u64;
_3 = 15923541054274390609_u64;
_9 = Adt49::Variant0 { fld0: _3 };
_5 = 2969_i16;
_8 = (-41555599057753201674830894461165159614_i128);
_6 = (-1642775263_i32);
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_4 = 8350715872445844071_usize;
_5 = _4 as i16;
_2 = _5 as u8;
_1 = _2 != _2;
place!(Field::<u64>(Variant(_9, 0), 0)) = _3 >> _8;
_2 = 79_u8 & 162_u8;
_7 = 5802_u16;
_14 = Adt49::Variant0 { fld0: Field::<u64>(Variant(_9, 0), 0) };
_9 = Move(_14);
SetDiscriminant(_9, 1);
place!(Field::<[u128; 8]>(Variant(_9, 1), 5)) = [7882924997588488409728666789926814949_u128,128374280642065776385038760217447723809_u128,328953286199836761603368778359109024837_u128,252379739645578331826765810713132939377_u128,256147323083249702212911733997104882703_u128,94760443292901285446511312680352288970_u128,181774598363935189916262790418145959054_u128,25258084234137978820862676720250523104_u128];
_3 = 17953914695082299890_u64;
_15 = !_1;
_12 = !(-120_isize);
place!(Field::<[u128; 8]>(Variant(_9, 1), 5)) = [250297540334826510264475489247586055288_u128,92743014608417021844413017949080681153_u128,161931102699385476364060723533098460762_u128,94839590773097435563856152657586966106_u128,236867767634830684559916966894968168619_u128,264269528819052789623882640071638600154_u128,172509233943675480026858927558227509418_u128,17281315606796474420499879276735047774_u128];
place!(Field::<[usize; 2]>(Variant(_9, 1), 2)) = [_4,_4];
_4 = 0_usize;
Goto(bb1)
}
bb1 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = (-6777_i16) | (-9349_i16);
_20 = -_18;
_14 = Adt49::Variant0 { fld0: _3 };
_6 = (-3956726818346113720_i64) as i32;
_20 = _18;
match _8 {
0 => bb1,
1 => bb3,
298726767863185261788543712970603051842 => bb5,
_ => bb4
}
}
bb3 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_18 = _20 - _20;
SetDiscriminant(_14, 0);
_7 = 31554_u16;
_21 = 8558555421622938178_i64 as u128;
_23 = 1294310607027617384_i64;
_14 = Adt49::Variant0 { fld0: _3 };
_20 = _21 as f64;
_23 = 7052343390614951832_i64;
_14 = Adt49::Variant0 { fld0: _3 };
_22 = [_8,_8,_8,_8,_8,_8,_8];
_3 = !Field::<u64>(Variant(_14, 0), 0);
_22 = [_8,_8,_8,_8,_8,_8,_8];
_16 = _13;
_2 = !222_u8;
_6 = _3 as i32;
_15 = _1;
SetDiscriminant(_14, 0);
_1 = !_15;
_12 = _6 as isize;
_23 = (-7716369412548286844_i64) ^ (-2478627646236495525_i64);
_17 = _5 - _5;
place!(Field::<u64>(Variant(_14, 0), 0)) = !_3;
place!(Field::<u64>(Variant(_14, 0), 0)) = '\u{3e6dd}' as u64;
place!(Field::<u64>(Variant(_9, 0), 0)) = _3 * _3;
_24 = !_21;
match _8 {
0 => bb3,
298726767863185261788543712970603051842 => bb7,
_ => bb6
}
}
bb6 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_20 = _18;
_5 = _7 as i16;
_28 = _12 + RET;
_23 = _20 as i64;
_8 = 63962515198073498080707188741140159186_i128 >> _23;
_24 = _1 as u128;
_1 = !_15;
_18 = -_20;
_20 = _18 * _18;
_14 = Move(_9);
_5 = _6 as i16;
_12 = _28;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
31554 => bb8,
_ => bb6
}
}
bb8 = {
_1 = _15 ^ _15;
_9 = Move(_14);
RET = _4 as isize;
_14 = Move(_9);
_18 = _20;
_18 = _24 as f64;
_14 = Adt49::Variant0 { fld0: _3 };
_15 = _1 | _1;
_25 = _2 as usize;
_25 = _4 | _4;
_2 = !192_u8;
_26 = _17 as isize;
_7 = 57340_u16 * 41483_u16;
_18 = _20 + _20;
SetDiscriminant(_14, 1);
place!(Field::<[u128; 8]>(Variant(_14, 1), 5)) = [_21,_24,_24,_21,_21,_21,_24,_24];
place!(Field::<(i64, i32, u32, i8)>(Variant(_14, 1), 3)).3 = -(-102_i8);
place!(Field::<[u128; 8]>(Variant(_14, 1), 5)) = [_24,_21,_21,_24,_24,_24,_24,_24];
_20 = _18 + _18;
place!(Field::<[usize; 2]>(Variant(_14, 1), 2)) = [_4,_25];
place!(Field::<(i64, i32, u32, i8)>(Variant(_14, 1), 3)).2 = '\u{bde60}' as u32;
RET = -_26;
_32.1 = _6 | _6;
place!(Field::<(i64, i32, u32, i8)>(Variant(_14, 1), 3)).1 = _6;
_20 = _18 * _18;
match _4 {
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
0 => bb16,
_ => bb15
}
}
bb9 = {
_20 = _18;
_5 = _7 as i16;
_28 = _12 + RET;
_23 = _20 as i64;
_8 = 63962515198073498080707188741140159186_i128 >> _23;
_24 = _1 as u128;
_1 = !_15;
_18 = -_20;
_20 = _18 * _18;
_14 = Move(_9);
_5 = _6 as i16;
_12 = _28;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
31554 => bb8,
_ => bb6
}
}
bb10 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_18 = _20 - _20;
SetDiscriminant(_14, 0);
_7 = 31554_u16;
_21 = 8558555421622938178_i64 as u128;
_23 = 1294310607027617384_i64;
_14 = Adt49::Variant0 { fld0: _3 };
_20 = _21 as f64;
_23 = 7052343390614951832_i64;
_14 = Adt49::Variant0 { fld0: _3 };
_22 = [_8,_8,_8,_8,_8,_8,_8];
_3 = !Field::<u64>(Variant(_14, 0), 0);
_22 = [_8,_8,_8,_8,_8,_8,_8];
_16 = _13;
_2 = !222_u8;
_6 = _3 as i32;
_15 = _1;
SetDiscriminant(_14, 0);
_1 = !_15;
_12 = _6 as isize;
_23 = (-7716369412548286844_i64) ^ (-2478627646236495525_i64);
_17 = _5 - _5;
place!(Field::<u64>(Variant(_14, 0), 0)) = !_3;
place!(Field::<u64>(Variant(_14, 0), 0)) = '\u{3e6dd}' as u64;
place!(Field::<u64>(Variant(_9, 0), 0)) = _3 * _3;
_24 = !_21;
match _8 {
0 => bb3,
298726767863185261788543712970603051842 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_5 = (-6777_i16) | (-9349_i16);
_20 = -_18;
_14 = Adt49::Variant0 { fld0: _3 };
_6 = (-3956726818346113720_i64) as i32;
_20 = _18;
match _8 {
0 => bb1,
1 => bb3,
298726767863185261788543712970603051842 => bb5,
_ => bb4
}
}
bb15 = {
_1 = !_15;
place!(Field::<(i64, i32, u32, i8)>(Variant(_9, 1), 3)).3 = (-123_i8);
_18 = Field::<[u128; 8]>(Variant(_9, 1), 5)[_4] as f64;
_6 = _5 as i32;
_9 = Adt49::Variant0 { fld0: _3 };
RET = !_12;
_7 = !52809_u16;
_18 = _5 as f64;
_13 = [_8,_8,_8,_8,_8];
SetDiscriminant(_9, 0);
_13[_4] = 3264624167_u32 as i128;
_7 = 32570_u16 & 59638_u16;
_12 = -RET;
_13[_4] = _8 << _8;
RET = _12 ^ _12;
Call(_16[_4] = fn1(_3, _3, _1, RET, _15, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_31.1 = _7;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(0_usize, 15_usize, Move(_15), 26_usize, Move(_26), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(0_usize, 25_usize, Move(_25), 5_usize, Move(_5), 16_usize, Move(_16), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(0_usize, 2_usize, Move(_2), 8_usize, Move(_8), 34_usize, _34, 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u64,mut _2: u64,mut _3: bool,mut _4: isize,mut _5: bool,mut _6: f64) -> i128 {
mir! {
type RET = i128;
let _7: ([u128; 8], u64, i64);
let _8: [i128; 5];
let _9: f64;
let _10: bool;
let _11: *const i16;
let _12: Adt55;
let _13: (((i32,), f32), i32, bool);
let _14: *mut u16;
let _15: u64;
let _16: (((i32,), f32), i32, bool);
let _17: (i64, i32, u32, i8);
let _18: (i16, u16);
let _19: i32;
let _20: char;
let _21: bool;
let _22: f64;
let _23: [usize; 2];
let _24: u16;
let _25: isize;
let _26: Adt58;
let _27: char;
let _28: isize;
let _29: f32;
let _30: (*const i16, (*mut f32, (i32,)), u64);
let _31: Adt48;
let _32: Adt56;
let _33: ((i32,), isize);
let _34: [i16; 3];
let _35: bool;
let _36: Adt60;
let _37: f64;
let _38: Adt55;
let _39: [usize; 2];
let _40: (*mut u16,);
let _41: ();
let _42: ();
{
RET = !(-119352660255875169499806756017061676305_i128);
_1 = !_2;
_2 = _1;
_4 = 22497_u16 as isize;
_5 = _3;
_7.1 = !_2;
_7.1 = _1;
Call(_5 = fn2(_3, _6, _3, _1, _3, _4, _4, _3, _3, _4, _4, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
Goto(bb2)
}
bb2 = {
_5 = !_3;
RET = !106013395990573006895980848089310602046_i128;
RET = !(-95722783290534922046443870948601882005_i128);
Goto(bb3)
}
bb3 = {
RET = !(-95225828725716214359801624276291980119_i128);
_4 = _5 as isize;
_8 = [RET,RET,RET,RET,RET];
_9 = _6;
_9 = _6 + _6;
Goto(bb4)
}
bb4 = {
_3 = _5 == _5;
_7.2 = !(-8863307593221406855_i64);
_8 = [RET,RET,RET,RET,RET];
_4 = (-9223372036854775808_isize) + (-122_isize);
_12.fld1 = RET as u32;
_5 = _7.1 != _7.1;
_5 = !_3;
_13.0.0 = ((-46117898_i32),);
_7.0 = [69071559348280751311189818399627190894_u128,251716590073655943489065940093323115187_u128,112127024647536717693994289000608215211_u128,51559756701073556618882300834270854810_u128,48870637666867373532936391448462597795_u128,328439751205761971586781136645747828856_u128,162425478129497235126850862422388774926_u128,167488646092928481561744488006865925187_u128];
_12.fld0.0 = _12.fld1 as i32;
_3 = !_5;
_15 = !_2;
_13.0.0.0 = _12.fld0.0;
_12.fld0.0 = _13.0.0.0;
_13.0.0.0 = _12.fld0.0 - _12.fld0.0;
_13.0.0 = (_12.fld0.0,);
_10 = _7.2 > _7.2;
_13.0.1 = _7.1 as f32;
_12.fld0.0 = _13.0.0.0;
_13.0.0 = (_12.fld0.0,);
Call(_7.1 = core::intrinsics::bswap(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = _10;
_12.fld1 = 2469160348_u32 & 3021027710_u32;
_13.1 = _5 as i32;
_4 = 9223372036854775807_isize;
RET = _5 as i128;
_17.3 = (-116_i8) - (-21_i8);
_17.0 = !_7.2;
_18.1 = 2578_u16;
_11 = core::ptr::addr_of!(_18.0);
_1 = !_7.1;
_17.1 = _13.0.0.0;
_12.fld0.0 = -_13.1;
_18.0 = (-26386_i16);
_18.1 = 41679_u16 * 37390_u16;
_16.0.0 = _13.0.0;
_17.2 = _12.fld1 & _12.fld1;
_18 = (31942_i16, 47638_u16);
Call(RET = core::intrinsics::bswap((-85764255211550351368735333377334771385_i128)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.0.0 = (_12.fld0.0,);
_16.0.0 = _13.0.0;
(*_11) = !22329_i16;
_16.0.1 = 153429124517308948727692974325006183369_u128 as f32;
_16.0.0 = (_13.0.0.0,);
_14 = core::ptr::addr_of_mut!(_18.1);
_20 = '\u{69967}';
_11 = core::ptr::addr_of!((*_11));
_13.1 = -_16.0.0.0;
RET = (-95986213770123092294960222848250354850_i128);
_15 = _1;
_6 = _9 + _9;
_16 = (_13.0, _13.1, _3);
_19 = !_13.1;
_23 = [6_usize,1_usize];
_15 = !_1;
_21 = !_3;
_13.0 = (_12.fld0, _16.0.1);
Call(_18.1 = core::intrinsics::bswap(56234_u16), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16.0.0.0 = RET as i32;
_12.fld0.0 = _17.2 as i32;
_20 = '\u{10f72d}';
_16 = (_13.0, _12.fld0.0, _5);
_17.1 = RET as i32;
_16.2 = !_10;
_13.0.0.0 = !_17.1;
match _4 {
0 => bb3,
1 => bb4,
9223372036854775807 => bb9,
_ => bb8
}
}
bb8 = {
RET = !(-95225828725716214359801624276291980119_i128);
_4 = _5 as isize;
_8 = [RET,RET,RET,RET,RET];
_9 = _6;
_9 = _6 + _6;
Goto(bb4)
}
bb9 = {
_16.1 = _17.1;
_11 = core::ptr::addr_of!((*_11));
_7.0 = [130995036458591026419639041060567821325_u128,285930927278969305762806683089340909810_u128,21148624536527458962014307811342241331_u128,72499252821400413590802850195247828074_u128,192676007111929052323707923723391650452_u128,333511991125550778915479742953723160340_u128,143195981873095152409330266279000605658_u128,69038668183444094613572440512530148185_u128];
_16.2 = _21;
_22 = _6;
_2 = _15 << _17.2;
_10 = _16.0.0.0 < _12.fld0.0;
_9 = _22 * _6;
_13.0.1 = RET as f32;
_6 = -_22;
_30.0 = _11;
_13.0.0.0 = _18.0 as i32;
_7.1 = _13.0.1 as u64;
_31.fld2 = _7.1;
_12.fld0 = _13.0.0;
_14 = core::ptr::addr_of_mut!((*_14));
_31.fld4 = -(*_11);
_31.fld1 = _30.0;
match _4 {
0 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb10 = {
_5 = _10;
_12.fld1 = 2469160348_u32 & 3021027710_u32;
_13.1 = _5 as i32;
_4 = 9223372036854775807_isize;
RET = _5 as i128;
_17.3 = (-116_i8) - (-21_i8);
_17.0 = !_7.2;
_18.1 = 2578_u16;
_11 = core::ptr::addr_of!(_18.0);
_1 = !_7.1;
_17.1 = _13.0.0.0;
_12.fld0.0 = -_13.1;
_18.0 = (-26386_i16);
_18.1 = 41679_u16 * 37390_u16;
_16.0.0 = _13.0.0;
_17.2 = _12.fld1 & _12.fld1;
_18 = (31942_i16, 47638_u16);
Call(RET = core::intrinsics::bswap((-85764255211550351368735333377334771385_i128)), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
Goto(bb2)
}
bb12 = {
(*_11) = _13.0.1 as i16;
_32.fld1 = (_16.0.0.0,);
_31.fld2 = !_2;
_31.fld2 = !_15;
_13.1 = _16.1;
_31.fld6 = _17.0 - _7.2;
_16.0.0 = (_16.1,);
_13.0.0.0 = _17.3 as i32;
_32.fld0.1 = !(*_14);
_17.2 = _17.1 as u32;
_28 = _4;
_13.0.1 = _16.0.1 * _16.0.1;
_33.0.0 = _17.1 << _31.fld4;
_32.fld0 = _18;
_7.2 = _31.fld6 * _17.0;
_31.fld4 = _16.1 as i16;
_25 = !_28;
_34 = [(*_11),(*_11),(*_11)];
_13.0.1 = _13.1 as f32;
_7.0 = [71149073310561492277694817846512016285_u128,3930957731126744639590374922114082348_u128,149967116781767315186038249022445388117_u128,292288060797287956908831792708779348987_u128,295875445251568730063362142437350385597_u128,247939906644800681360618889075314165284_u128,153654638830260844106970252788877620435_u128,240914347230918217167152260840584194337_u128];
_30.1.1 = (_33.0.0,);
_15 = !_2;
_17.1 = _19 * _13.0.0.0;
Call(_17.0 = fn3(_17.1, (*_14), _16.2, _10, _33.0, _17.1, _33.0, _7.0, _9, _7.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3 = !_16.2;
_38.fld0.0 = _32.fld1.0;
_6 = _22;
_27 = _20;
_32 = Adt56 { fld0: _18,fld1: _16.0.0,fld2: 127800713423589883815391527706262610648_u128 };
_16.0.0.0 = _17.1;
_18 = (_31.fld4, _32.fld0.1);
Goto(bb14)
}
bb14 = {
_16.1 = _32.fld2 as i32;
RET = !(-104311096992468855937366360755508515172_i128);
_27 = _20;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(1_usize, 19_usize, Move(_19), 21_usize, Move(_21), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(1_usize, 7_usize, Move(_7), 10_usize, Move(_10), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(1_usize, 27_usize, Move(_27), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: f64,mut _3: bool,mut _4: u64,mut _5: bool,mut _6: isize,mut _7: isize,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: isize,mut _12: u64) -> bool {
mir! {
type RET = bool;
let _13: [i128; 7];
let _14: isize;
let _15: [u128; 8];
let _16: Adt56;
let _17: isize;
let _18: (*mut f32, (i32,));
let _19: ();
let _20: ();
{
_4 = !_12;
_5 = _8;
_1 = _3 ^ _8;
_5 = _1;
_3 = !_9;
_4 = _12;
_5 = _1 > _8;
RET = _1;
RET = _5;
RET = !_3;
RET = _1;
_10 = _7 ^ _11;
_13 = [(-100792233444700233536090239288501476308_i128),(-115973059615274755793850734001300255780_i128),(-94028769744612096006080241677516915524_i128),(-116268762897658816807768249396264112733_i128),129851764957136821029764018485279970987_i128,(-109273271383999244986851784056100707689_i128),53045542586497188702281712856625418267_i128];
Goto(bb1)
}
bb1 = {
RET = _5 & _1;
_2 = 17607_u16 as f64;
_16.fld2 = 107618048958644416127618657515810825302_u128;
_6 = _7;
RET = _5 == _5;
_14 = _6 ^ _6;
_4 = 3103169699_u32 as u64;
_17 = 52904_u16 as isize;
_15 = [_16.fld2,_16.fld2,_16.fld2,_16.fld2,_16.fld2,_16.fld2,_16.fld2,_16.fld2];
_16.fld0.1 = 1944_u16 | 23880_u16;
_16.fld1 = ((-1208274775_i32),);
_11 = _16.fld0.1 as isize;
_13 = [161981892207859106763195062084995152713_i128,167422148062187184978050431730443105450_i128,(-39696182059634830562192120954005177377_i128),(-22890474876754913589249049966572564769_i128),(-17572655559134484147223695120127094992_i128),(-9062369730622621432998473258521481026_i128),86215393067253921588937303683082223684_i128];
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(2_usize, 6_usize, Move(_6), 13_usize, Move(_13), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(2_usize, 8_usize, Move(_8), 7_usize, Move(_7), 15_usize, Move(_15), 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i32,mut _2: u16,mut _3: bool,mut _4: bool,mut _5: (i32,),mut _6: i32,mut _7: (i32,),mut _8: [u128; 8],mut _9: f64,mut _10: i64) -> i64 {
mir! {
type RET = i64;
let _11: Adt55;
let _12: (i64, i32, u32, i8);
let _13: ((i32,), f32);
let _14: [u128; 8];
let _15: u16;
let _16: f32;
let _17: f64;
let _18: f64;
let _19: [i128; 7];
let _20: i64;
let _21: usize;
let _22: Adt55;
let _23: f64;
let _24: ([u128; 8], u64, i64);
let _25: u16;
let _26: (i64, i32, u32, i8);
let _27: Adt53;
let _28: bool;
let _29: f64;
let _30: u128;
let _31: Adt56;
let _32: ((i32,), isize);
let _33: ();
let _34: ();
{
_12 = (_10, _7.0, 2943665280_u32, (-72_i8));
_5.0 = _7.0 & _6;
_12.3 = (-126009873599927446633098115336714727692_i128) as i8;
_11 = Adt55 { fld0: _5,fld1: _12.2 };
_11 = Adt55 { fld0: _5,fld1: _12.2 };
_14 = [262884620015492984815350597336670352060_u128,97790512698601498487650075667051464388_u128,218500887115449693491611898583106398642_u128,142030383922789965712524561444714767367_u128,267497724564655569493901238293773617935_u128,20217854819251044033939247592369258720_u128,49947909474367125201382834181844442585_u128,49815329613514903252929249213087016082_u128];
_13.0.0 = (-18_isize) as i32;
_12.2 = _11.fld1;
RET = -_12.0;
_14 = _8;
RET = _10;
_5 = (_13.0.0,);
_2 = _10 as u16;
_11 = Adt55 { fld0: _5,fld1: _12.2 };
_4 = _12.2 <= _12.2;
Call(_6 = core::intrinsics::transmute(_12.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.fld0 = (_1,);
_15 = !_2;
_7.0 = _6;
_12 = (RET, _1, _11.fld1, 70_i8);
_7 = _11.fld0;
_11.fld0.0 = _6 ^ _7.0;
_11 = Adt55 { fld0: _7,fld1: _12.2 };
_14 = [251906217137634933979075408787719602615_u128,306895400899666385433159111721285894405_u128,291826163459011567200420434494223342124_u128,214662104617331646956016162557767129493_u128,187183303383875690240890154729989270836_u128,634105163934215421868604245415279524_u128,84440571386984661466699188461813005132_u128,280009033470689098948306820620151973840_u128];
_12.0 = RET * _10;
_12.0 = RET;
_12.0 = RET;
_15 = _2 >> _12.2;
_13.0 = (_1,);
_12.0 = -RET;
_6 = _12.1;
_13.0 = (_1,);
_12.2 = (-31287_i16) as u32;
_6 = _7.0 & _11.fld0.0;
_11 = Adt55 { fld0: _7,fld1: _12.2 };
_4 = _3;
_18 = _9 - _9;
_17 = _9;
Call(_13 = fn4(_12.3, _6, _12, _12, _1, _6, _12, _10, _18, _11.fld0.0, _18, _12.1, _11.fld0, _3, _11.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _7.0 * _11.fld0.0;
_2 = _3 as u16;
_7.0 = _1;
_5.0 = 256073364208431274397550040344624625835_u128 as i32;
RET = _3 as i64;
_5 = _13.0;
_7.0 = _13.0.0 & _6;
_20 = !_12.0;
_18 = _17 + _17;
_11.fld0.0 = _5.0 - _7.0;
Call(_13.0.0 = fn5(_5, _11.fld0.0, _8, _18, _11.fld0.0, _11.fld0.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.0.0 = -_11.fld0.0;
_7.0 = _13.0.0;
_7.0 = -_13.0.0;
_16 = _13.1 + _13.1;
_19 = [67878011811385365997291312029856282476_i128,(-148676711683940110071341952713787116163_i128),(-43510430877643027124859091799915750660_i128),155762394457625957127478273730193626243_i128,(-128461015879858785701197393466873686369_i128),(-21821833812936946964792318504367019617_i128),(-62132057327479861322546514043901987774_i128)];
_20 = 12906794798474004557_u64 as i64;
_12.0 = _10;
_12.0 = -_10;
_13.1 = 77315947804931495052251628013762057322_u128 as f32;
_1 = 9942017078990655635904151734495145955_i128 as i32;
_14 = [192562566281523637579026294116239777571_u128,181385405459017942994852596487373354717_u128,101164771289031032489588586772545417148_u128,299004904128770108001084611395681897264_u128,212838751307702900520195754835904852828_u128,65352036318547069465577660295256734156_u128,150052822792749538340781397206731722666_u128,153046709928216066414437653277713665148_u128];
_12 = (_10, _13.0.0, _11.fld1, 92_i8);
Goto(bb4)
}
bb4 = {
_12.1 = -_13.0.0;
_10 = _12.0 >> _12.3;
Call(_14 = fn6(_12.3, _5, _12.1, _12.3, _11.fld0, _7, _5.0, _12, _10, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12.2 = 154051601972958770891631981178072965881_u128 as u32;
_7 = (_6,);
_18 = _9;
_16 = -_13.1;
_22.fld0.0 = _12.1;
_22.fld0.0 = _12.1;
_13.0 = (_11.fld0.0,);
_13 = (_7, _16);
_11 = Adt55 { fld0: _22.fld0,fld1: _12.2 };
_22.fld1 = !_12.2;
_10 = 8840180888459065554748252326184189494_u128 as i64;
_7 = (_12.1,);
_22.fld0 = (_12.1,);
_1 = _5.0;
_16 = _13.1;
_12.0 = _10 ^ _20;
_12.0 = 275247275027690564010339891720585798505_u128 as i64;
Goto(bb6)
}
bb6 = {
match _12.3 {
0 => bb1,
1 => bb5,
2 => bb7,
92 => bb9,
_ => bb8
}
}
bb7 = {
_13.0.0 = -_11.fld0.0;
_7.0 = _13.0.0;
_7.0 = -_13.0.0;
_16 = _13.1 + _13.1;
_19 = [67878011811385365997291312029856282476_i128,(-148676711683940110071341952713787116163_i128),(-43510430877643027124859091799915750660_i128),155762394457625957127478273730193626243_i128,(-128461015879858785701197393466873686369_i128),(-21821833812936946964792318504367019617_i128),(-62132057327479861322546514043901987774_i128)];
_20 = 12906794798474004557_u64 as i64;
_12.0 = _10;
_12.0 = -_10;
_13.1 = 77315947804931495052251628013762057322_u128 as f32;
_1 = 9942017078990655635904151734495145955_i128 as i32;
_14 = [192562566281523637579026294116239777571_u128,181385405459017942994852596487373354717_u128,101164771289031032489588586772545417148_u128,299004904128770108001084611395681897264_u128,212838751307702900520195754835904852828_u128,65352036318547069465577660295256734156_u128,150052822792749538340781397206731722666_u128,153046709928216066414437653277713665148_u128];
_12 = (_10, _13.0.0, _11.fld1, 92_i8);
Goto(bb4)
}
bb8 = {
_11.fld0 = (_1,);
_15 = !_2;
_7.0 = _6;
_12 = (RET, _1, _11.fld1, 70_i8);
_7 = _11.fld0;
_11.fld0.0 = _6 ^ _7.0;
_11 = Adt55 { fld0: _7,fld1: _12.2 };
_14 = [251906217137634933979075408787719602615_u128,306895400899666385433159111721285894405_u128,291826163459011567200420434494223342124_u128,214662104617331646956016162557767129493_u128,187183303383875690240890154729989270836_u128,634105163934215421868604245415279524_u128,84440571386984661466699188461813005132_u128,280009033470689098948306820620151973840_u128];
_12.0 = RET * _10;
_12.0 = RET;
_12.0 = RET;
_15 = _2 >> _12.2;
_13.0 = (_1,);
_12.0 = -RET;
_6 = _12.1;
_13.0 = (_1,);
_12.2 = (-31287_i16) as u32;
_6 = _7.0 & _11.fld0.0;
_11 = Adt55 { fld0: _7,fld1: _12.2 };
_4 = _3;
_18 = _9 - _9;
_17 = _9;
Call(_13 = fn4(_12.3, _6, _12, _12, _1, _6, _12, _10, _18, _11.fld0.0, _18, _12.1, _11.fld0, _3, _11.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_24.2 = -_10;
_12.0 = RET | RET;
_2 = _15 * _15;
_7 = (_22.fld0.0,);
_4 = _3;
_19 = [(-54525813172749727676616601491780299398_i128),156696187920860934352531424585925083401_i128,159087355592291071208290976468816535367_i128,121556103976114649445439749907675787164_i128,(-118428652385729746555855525485575936881_i128),(-152721357171725054662959553979429794649_i128),39380636284480538356318793572298879225_i128];
_7.0 = !_6;
_11.fld0.0 = _5.0;
_5 = _22.fld0;
_11.fld0.0 = !_1;
_12.3 = (-47_i8) + (-22_i8);
_12.1 = _11.fld0.0 * _5.0;
_13.0 = _5;
_4 = _11.fld0.0 != _13.0.0;
_13.0 = (_12.1,);
_18 = _9;
_5.0 = !_13.0.0;
_26.1 = 132_u8 as i32;
_4 = _1 <= _13.0.0;
Goto(bb10)
}
bb10 = {
_22.fld0.0 = _12.1;
_24.2 = _12.0 + _12.0;
_23 = _9;
RET = -_12.0;
_12.2 = _22.fld1 | _11.fld1;
_28 = !_4;
_12.3 = (-18_i8) * 17_i8;
_11.fld1 = _12.2;
_26.3 = _12.3;
RET = -_24.2;
_5.0 = _12.1 | _7.0;
_11.fld1 = '\u{f155c}' as u32;
_26.2 = _26.3 as u32;
_15 = _2;
_29 = _17;
_24 = (_8, 18092642729774060422_u64, RET);
_25 = !_2;
_12.2 = !_26.2;
_31.fld1 = (_5.0,);
_7.0 = _13.0.0 | _31.fld1.0;
_31.fld1.0 = _2 as i32;
_7 = (_5.0,);
_10 = RET;
_7 = (_12.1,);
_32.0 = (_22.fld0.0,);
Goto(bb11)
}
bb11 = {
Call(_33 = dump_var(3_usize, 10_usize, Move(_10), 15_usize, Move(_15), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_33 = dump_var(3_usize, 28_usize, Move(_28), 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8,mut _2: i32,mut _3: (i64, i32, u32, i8),mut _4: (i64, i32, u32, i8),mut _5: i32,mut _6: i32,mut _7: (i64, i32, u32, i8),mut _8: i64,mut _9: f64,mut _10: i32,mut _11: f64,mut _12: i32,mut _13: (i32,),mut _14: bool,mut _15: (i32,)) -> ((i32,), f32) {
mir! {
type RET = ((i32,), f32);
let _16: f64;
let _17: char;
let _18: bool;
let _19: u128;
let _20: ();
let _21: ();
{
RET.0 = _15;
RET.0.0 = _7.1 - _6;
RET.1 = 19835_i16 as f32;
_10 = !RET.0.0;
_9 = 50332372105359738048391756363699326319_u128 as f64;
_14 = false;
_7.2 = '\u{aec8f}' as u32;
_4.3 = -_3.3;
RET.0 = (_3.1,);
_4 = (_8, _5, _7.2, _3.3);
_11 = _9 + _9;
_16 = _9;
_3.1 = !_5;
_2 = -_10;
_3 = _4;
match _4.3 {
0 => bb1,
1 => bb2,
70 => bb4,
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
RET.0.0 = !_10;
RET.0 = (_2,);
Goto(bb5)
}
bb5 = {
Call(_20 = dump_var(4_usize, 10_usize, Move(_10), 15_usize, Move(_15), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_20 = dump_var(4_usize, 3_usize, Move(_3), 4_usize, Move(_4), 21_usize, _21, 21_usize, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (i32,),mut _2: i32,mut _3: [u128; 8],mut _4: f64,mut _5: i32,mut _6: i32) -> i32 {
mir! {
type RET = i32;
let _7: [i16; 3];
let _8: [u128; 8];
let _9: char;
let _10: u32;
let _11: Adt46;
let _12: ();
let _13: ();
{
RET = -_1.0;
_7 = [31767_i16,(-13493_i16),(-23100_i16)];
_9 = '\u{ff96d}';
_2 = _6;
_10 = 1931676428_u32 | 3240345996_u32;
_2 = -_5;
_8 = [12835550041455926816456104588167980385_u128,324719053382388331318798083278075955860_u128,106878364266491815248341723374195749113_u128,198583379535165898598148282342172683904_u128,39817018910103353662591341795991760313_u128,186374165205077767195877658363602887862_u128,302510722729971643975438977138700862323_u128,119580043828834748369299305623000142020_u128];
_8 = _3;
_3 = [215088066035853062444057180189067130745_u128,25367716061335704006613070487821375102_u128,4896302810879437140293665087160245051_u128,100582989576866639946559784722894474299_u128,179125616201033776530024728899314887458_u128,150482832426345884181418735353061532511_u128,41486589538870985296659592970883712462_u128,186530723747231986844362023997224016945_u128];
_10 = 248745367_u32 >> _6;
_6 = -_2;
_5 = _6;
RET = _6 * _2;
_6 = !_5;
_4 = RET as f64;
RET = _10 as i32;
_7 = [(-19961_i16),(-16792_i16),517_i16];
_6 = 7_usize as i32;
_2 = 67_isize as i32;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(5_usize, 6_usize, Move(_6), 5_usize, Move(_5), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i8,mut _2: (i32,),mut _3: i32,mut _4: i8,mut _5: (i32,),mut _6: (i32,),mut _7: i32,mut _8: (i64, i32, u32, i8),mut _9: i64,mut _10: [i128; 7]) -> [u128; 8] {
mir! {
type RET = [u128; 8];
let _11: char;
let _12: isize;
let _13: f64;
let _14: char;
let _15: [i128; 5];
let _16: char;
let _17: Adt55;
let _18: Adt60;
let _19: ([u128; 8], u64, i64);
let _20: f64;
let _21: f32;
let _22: [usize; 2];
let _23: char;
let _24: [i16; 3];
let _25: isize;
let _26: i8;
let _27: *mut (*mut f32, (i32,));
let _28: [i16; 3];
let _29: isize;
let _30: ((i32,), f32);
let _31: [i128; 5];
let _32: bool;
let _33: u8;
let _34: u64;
let _35: isize;
let _36: i16;
let _37: Adt56;
let _38: f32;
let _39: [i128; 7];
let _40: u64;
let _41: f32;
let _42: char;
let _43: isize;
let _44: Adt55;
let _45: u8;
let _46: char;
let _47: [u128; 8];
let _48: [usize; 2];
let _49: isize;
let _50: ();
let _51: ();
{
_1 = true as i8;
_8 = (_9, _6.0, 3224856676_u32, _4);
RET = [271245314209534665045149145986074743574_u128,104799848831795248242592668458962777434_u128,333544249917650547852348275450965824789_u128,279207032800452463935546147647463886222_u128,62950939082515161689113372414999822308_u128,142497903961087001794145397598022696395_u128,161166981268132591009417991848202138430_u128,315064504264581479254533131610070200665_u128];
_10 = [(-86488793037739747307317440128891054705_i128),122721376873877594913027117099013149117_i128,163433151707954092982492658658685256694_i128,112301984393360880666921982813002805829_i128,82489655996843101337354114407704439296_i128,115108051149627600230144234410137423435_i128,(-25680640691458197839915508315811558765_i128)];
_10 = [(-49526795935479921298556665831771350073_i128),83611745400016855427719242098334131764_i128,4685767842824790104469415470300986798_i128,(-114806767855083073306678914527664838314_i128),(-100534969869443522714394007010326237743_i128),105132141062496555083537670303855358352_i128,132827445137227355414116516379682579880_i128];
_6 = _5;
_5.0 = 6_usize as i32;
_8.2 = 3960819638_u32 + 3621417101_u32;
_8.3 = _4;
_3 = _6.0 >> _9;
_6 = (_8.1,);
_8.1 = _3;
_7 = _3;
_2.0 = _3 ^ _3;
_7 = _2.0;
_8.1 = _3 * _2.0;
_5 = (_8.1,);
Call(_11 = fn7(_8, _2, _8.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_9 = -_8.0;
_8.1 = _2.0 + _2.0;
_3 = _2.0 << _7;
_8.1 = !_2.0;
_2.0 = !_8.1;
RET = [43024755702000323315945951631617441017_u128,58262079927365760445989030316815360546_u128,16612713705061976835916367759352138182_u128,13120686863752784014410458530206877710_u128,209395019460297363617593165827795251597_u128,53162039205325257509832295878854251277_u128,36319987722968333687739984464644442778_u128,228729098661442358311522677405743502613_u128];
_1 = -_4;
_15 = [19067600449682514065210913229817719550_i128,104035369292636617839943495425043909293_i128,154184035203891884699017585183092751747_i128,5997719047419275243775615709674804144_i128,(-154789582980114768162466483874421029362_i128)];
_17.fld0 = (_3,);
_6.0 = -_7;
_17.fld1 = _8.2;
Call(_8.0 = fn9(_2, _5.0, _17.fld0.0, _7, _8.1, _17, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = (_17.fld0.0,);
_4 = !_1;
_19.0 = [211835840443294212550040189957654824980_u128,246982242260861613278987948115152483223_u128,211902523229587661453472428116837760561_u128,23226683684743090810642728369410395763_u128,338372486063767895583888871104890582618_u128,279340335248457470958481837510256438905_u128,109398686280690887530994959719733435892_u128,84718397691144069066723758391788968810_u128];
_16 = _11;
_2 = (_3,);
_16 = _11;
_19.1 = 3170659205934745743_u64 + 2603655714878062489_u64;
_8 = (_9, _5.0, _17.fld1, _1);
_13 = _6.0 as f64;
_15 = [(-15522480296209479909875013362950289773_i128),60524378442595253787848539902147600580_i128,95138152261362108821846154921695957164_i128,110936156234517258791359334999473396795_i128,59512372601353204470678066251117868647_i128];
_16 = _11;
_15 = [40252932560200438534484630073207627849_i128,147791096645620336125310422034264262415_i128,(-27313400741840193123107318343151124586_i128),44916466185090224778000849738193534005_i128,119175593727017464688204700006862063795_i128];
_21 = _8.3 as f32;
_21 = _4 as f32;
_6 = _17.fld0;
_6.0 = _19.1 as i32;
_5.0 = _19.1 as i32;
_19.2 = -_8.0;
_6.0 = _3;
Call(_11 = fn11(_6.0, _7, _2.0, _8, _21, _7, _7, _7, _13, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = _11;
_14 = _16;
RET = [24569930790907564722986516376204843474_u128,181254806937342698374904174625051184136_u128,171370606946877828644957566924706378788_u128,157001936435351943903433982373852356233_u128,186719754435291531808156646191343189587_u128,257624544688955189191231840256299049727_u128,110200141451249947394250332890022693536_u128,130728769257607664859270257762463788496_u128];
_15 = [149163628005144882324572725546741496944_i128,167669242725074927970205995146676892563_i128,(-16549525822776673302416255094337463087_i128),(-78847746836588058063887630831498704558_i128),(-20970492634803872320183874538157700377_i128)];
_2 = _6;
Goto(bb4)
}
bb4 = {
_22 = [1_usize,4_usize];
_15 = [45159390683792869738992331201852284326_i128,113234824046193672104753767179999753472_i128,106545547607677117868560222166135704719_i128,18518555035888985162786452653168477592_i128,(-150101510398353170560475497627052210681_i128)];
_22 = [7_usize,7_usize];
_17.fld1 = !_8.2;
_6.0 = 6_usize as i32;
_19.0 = [173425232744682967105396183812123321851_u128,37944845066992845917845085308191282687_u128,123413577651129070878225101369303999837_u128,95121131096535570790105947878118232469_u128,292573097510817670930991537960625260870_u128,151695168922100049810726108860892999191_u128,37962879127287872195590660051530050006_u128,145599731362478198244027010054634809485_u128];
_21 = _19.1 as f32;
_4 = !_1;
_8.0 = !_19.2;
_19 = (RET, 1287563619538017572_u64, _9);
_19 = (RET, 10769385741947261640_u64, _8.0);
Goto(bb5)
}
bb5 = {
_15 = [(-38250211391037861695642342328687900519_i128),(-87395152604136095272935814591808099426_i128),(-163453604903915314344810911312666525275_i128),129837661895938314243210780595280021285_i128,15676582560581037150175254228256127677_i128];
_12 = (-93_isize);
_25 = _12 + _12;
_17 = Adt55 { fld0: _2,fld1: _8.2 };
_15 = [(-71999968370304841002686481583527961570_i128),137540150353963568888919188538217476816_i128,40534999259795146088552795654560508121_i128,93337040613387287246550438709568387910_i128,105477837134001190310533570685607566879_i128];
_14 = _16;
_8.0 = -_19.2;
_8 = (_19.2, _17.fld0.0, _17.fld1, _1);
_4 = _1 << _8.1;
_8.3 = !_4;
_8.2 = _17.fld1;
_2 = (_8.1,);
_15 = [134838435300372292883729967738125802350_i128,(-37838321030913909605474305119011780825_i128),106764927842445597753873709632003227345_i128,(-163959862539605110031268949085717050450_i128),(-154896274187620729826783183384144020469_i128)];
RET = [138385120739356258989714404821703337891_u128,98568953085470138377814203969128944692_u128,72783438874052316709426431365234670443_u128,101597898315083608978173300751207367663_u128,181405484850992917765401883416803579966_u128,328890578346845958722384951742147644240_u128,308551437514820764956166995192632965180_u128,68184571355715969856221358473304768159_u128];
_9 = _8.0;
_23 = _14;
_19.1 = _8.3 as u64;
_9 = _19.2;
_20 = 234_u8 as f64;
_8.1 = 2_usize as i32;
_8 = (_9, _7, _17.fld1, _4);
Goto(bb6)
}
bb6 = {
Goto(bb7)
}
bb7 = {
_21 = _8.2 as f32;
_22 = [7_usize,3675842290097798196_usize];
_17.fld0.0 = -_3;
_11 = _23;
_11 = _23;
_2 = (_7,);
_17.fld1 = !_8.2;
_32 = !false;
_2.0 = _17.fld0.0 << _4;
_8 = (_9, _2.0, _17.fld1, _4);
_8.2 = _17.fld1;
_17 = Adt55 { fld0: _2,fld1: _8.2 };
_33 = 120_u8 >> _1;
_5 = (_8.1,);
_30.1 = _21 * _21;
Goto(bb8)
}
bb8 = {
_26 = _8.3;
match _12 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb6,
340282366920938463463374607431768211363 => bb9,
_ => bb5
}
}
bb9 = {
_28 = [(-1298_i16),(-6131_i16),15470_i16];
_8.2 = _17.fld1;
Call(_1 = core::intrinsics::bswap(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = Adt55 { fld0: _5,fld1: _8.2 };
_31 = [(-29523033712091698812624554363389930097_i128),(-74758108846636536604300685942616345483_i128),(-155649748485031896231255849724821969970_i128),(-50931963298554795029375094595232681961_i128),147913324060641458698425299303863512836_i128];
_30 = (_2, _21);
_37.fld1 = (_3,);
_21 = 305859507306712131786487098704837740643_u128 as f32;
_19.1 = 296675921572121541794214420670600658131_u128 as u64;
_19.0 = [95645909537979078815428312926612620539_u128,304205812090675376798777906303043732554_u128,333878308224237583322281427182762581261_u128,103652841074377171762116583412902050633_u128,251032210222340185565557459055252499654_u128,85238085626986278859225452008956085109_u128,220258566280813028302690558953279854282_u128,70248638438043611063153001898620365875_u128];
_21 = _30.1;
_34 = !_19.1;
_30.0 = (_3,);
_28 = [11765_i16,(-18428_i16),12883_i16];
_2 = _37.fld1;
_17.fld1 = _8.2 & _8.2;
_15 = _31;
_9 = _8.0;
_37.fld0.0 = !(-22556_i16);
_23 = _16;
_6 = (_17.fld0.0,);
_8 = (_19.2, _6.0, _17.fld1, _4);
_29 = _33 as isize;
_20 = _33 as f64;
_1 = !_4;
_19.0 = [12992858748921520752707205180671637377_u128,229137178859091376102308903040236719415_u128,90003443714034619928019453763260095191_u128,42674064974566471935810719997873899385_u128,134664956111119162324638035537922110876_u128,237717642238808767758325253050394299331_u128,176828169494108193669342766268002374281_u128,275703473436957871612035888682180297504_u128];
_16 = _11;
_22 = [5535533779532063449_usize,10380689606804472151_usize];
Goto(bb11)
}
bb11 = {
_19.1 = _34 << _19.2;
_30.0.0 = _8.1 | _5.0;
_30 = (_6, _21);
Call(_14 = fn18(_19.2, _31, _30, _6, _5, _30.0, _29, _37.fld0.0, _30.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_39 = [130297095970032287130052808249295669949_i128,(-100171999061416201348516976165148031512_i128),(-36238105929725439655425695192325737525_i128),(-122847154428039800772548168757197348645_i128),(-25624984830768091561869190148346484967_i128),(-161439427594613853148153017914673138508_i128),(-78267447226189148001881663150678353454_i128)];
_22 = [6_usize,1381716575207175092_usize];
_12 = 28987337332852791968270834792872930852_i128 as isize;
_36 = _37.fld0.0;
_14 = _11;
_15 = _31;
_6.0 = -_2.0;
_41 = _1 as f32;
_28 = [_36,_36,_36];
_20 = -_13;
_19.2 = _33 as i64;
_17 = Adt55 { fld0: _30.0,fld1: _8.2 };
_42 = _23;
_37.fld0.0 = _32 as i16;
_19.0 = RET;
_38 = 6_usize as f32;
_8.1 = -_30.0.0;
_12 = !_29;
_37.fld0 = (_36, 39242_u16);
Goto(bb13)
}
bb13 = {
_44.fld1 = !_17.fld1;
_44.fld0.0 = _8.2 as i32;
_37.fld2 = !251299078020259099295805734299541461770_u128;
_43 = _12;
_2.0 = _17.fld1 as i32;
_37.fld1 = (_6.0,);
_37.fld0 = (_36, 29615_u16);
Goto(bb14)
}
bb14 = {
_13 = _20 * _20;
_15 = [73954898706952770193959444083986564409_i128,(-94249274663135319325793707740725060646_i128),(-30607109595782213420111849177452666745_i128),(-36513024241711062051919922996191479654_i128),(-36546483824510371576756656696412393825_i128)];
_22 = [5710687168822342590_usize,1_usize];
RET = [_37.fld2,_37.fld2,_37.fld2,_37.fld2,_37.fld2,_37.fld2,_37.fld2,_37.fld2];
_4 = _1 | _8.3;
_9 = !_8.0;
_6.0 = _37.fld2 as i32;
_39 = [(-47238254080311915899921644803067527851_i128),(-127940294143721866843459868446433115838_i128),(-111268153825652270835066679839762706624_i128),(-166914788662500878113183444460728317086_i128),80935682989801252182629908737113905670_i128,130368472500772256377324703565388081611_i128,6692246462544345675317370001244730028_i128];
_8 = (_19.2, _17.fld0.0, _17.fld1, _4);
_15 = _31;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(6_usize, 19_usize, Move(_19), 25_usize, Move(_25), 31_usize, Move(_31), 36_usize, Move(_36)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(6_usize, 3_usize, Move(_3), 34_usize, Move(_34), 39_usize, Move(_39), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(6_usize, 12_usize, Move(_12), 10_usize, Move(_10), 11_usize, Move(_11), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(6_usize, 14_usize, Move(_14), 5_usize, Move(_5), 1_usize, Move(_1), 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (i64, i32, u32, i8),mut _2: (i32,),mut _3: i32) -> char {
mir! {
type RET = char;
let _4: char;
let _5: i128;
let _6: bool;
let _7: (*mut u16,);
let _8: f32;
let _9: ((i32,), f32);
let _10: [i16; 3];
let _11: i16;
let _12: u128;
let _13: Adt59;
let _14: ([u128; 8], u64, i64);
let _15: [i128; 7];
let _16: ([u128; 8], u64, i64);
let _17: isize;
let _18: [u8; 5];
let _19: u16;
let _20: isize;
let _21: ((i32,), f32);
let _22: isize;
let _23: bool;
let _24: u128;
let _25: (i64, i32, u32, i8);
let _26: [usize; 2];
let _27: *const (*mut u16,);
let _28: i8;
let _29: f64;
let _30: isize;
let _31: ([u128; 8], u64, i64);
let _32: usize;
let _33: [u128; 8];
let _34: Adt60;
let _35: f64;
let _36: bool;
let _37: bool;
let _38: u32;
let _39: [u128; 8];
let _40: isize;
let _41: ();
let _42: ();
{
match _1.3 {
0 => bb1,
1 => bb2,
2 => bb3,
92 => bb5,
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
_4 = '\u{445eb}';
_1.1 = 2_usize as i32;
RET = _4;
_1.3 = 9223372036854775807_isize as i8;
RET = _4;
Goto(bb6)
}
bb6 = {
_1.0 = -8340032929038803697_i64;
_3 = _1.2 as i32;
_1.1 = -_2.0;
_6 = false;
_3 = _2.0;
_1 = ((-2113786231032849307_i64), _3, 3226528408_u32, 103_i8);
_2 = (_1.1,);
RET = _4;
_3 = _1.1 + _1.1;
_1.2 = !1940526869_u32;
_1.2 = 1574778236_u32;
RET = _4;
_4 = RET;
_4 = RET;
RET = _4;
_3 = 1576873753764237109_u64 as i32;
_9.0.0 = _2.0 >> _1.3;
_2 = (_1.1,);
_1.0 = -5717772200819291235_i64;
_1 = ((-6074579112850237587_i64), _2.0, 1812385066_u32, (-125_i8));
_1.1 = _9.0.0 * _2.0;
RET = _4;
_9.1 = _1.3 as f32;
RET = _4;
_6 = !true;
_1 = (3691987311723277886_i64, _9.0.0, 4076222834_u32, (-101_i8));
_3 = 12466502101533334545_u64 as i32;
Goto(bb7)
}
bb7 = {
_11 = !(-29102_i16);
_2 = (_9.0.0,);
_1.2 = 2941479907_u32;
_9.0.0 = _1.1;
RET = _4;
_10 = [_11,_11,_11];
_5 = RET as i128;
_10 = [_11,_11,_11];
_8 = _9.1;
_2.0 = _1.1 | _9.0.0;
_1 = (8163647828247802234_i64, _2.0, 1171329501_u32, 70_i8);
_9 = (_2, _8);
match _1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
1171329501 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_11 = _8 as i16;
_1.3 = !(-31_i8);
_2.0 = _1.1 << _1.0;
_9 = (_2, _8);
_12 = _9.1 as u128;
_4 = RET;
_6 = true;
_2.0 = _1.1 - _1.1;
_5 = -72990558661259308832807302248614355240_i128;
_5 = (-59378352538395501789449574486166313699_i128);
_14.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_12 = 245_u8 as u128;
match _1.0 {
0 => bb4,
8163647828247802234 => bb11,
_ => bb10
}
}
bb10 = {
_4 = '\u{445eb}';
_1.1 = 2_usize as i32;
RET = _4;
_1.3 = 9223372036854775807_isize as i8;
RET = _4;
Goto(bb6)
}
bb11 = {
_9 = (_2, _8);
_8 = _9.1 + _9.1;
_1.3 = 9223372036854775807_isize as i8;
_4 = RET;
_9.0 = (_2.0,);
_1.0 = !347818437362329776_i64;
_5 = _9.1 as i128;
_1.2 = 846313286_u32 & 275939792_u32;
_17 = !9223372036854775807_isize;
_15 = [_5,_5,_5,_5,_5,_5,_5];
_7.0 = core::ptr::addr_of_mut!(_19);
_2.0 = _9.0.0;
_6 = false;
_5 = 117108975871373251092545743021218191841_i128;
_11 = 64604_u16 as i16;
_16 = (_14.0, 10558461556113277975_u64, _1.0);
_16.2 = _4 as i64;
_18 = [142_u8,214_u8,78_u8,105_u8,197_u8];
_14.1 = _6 as u64;
_11 = 26422_i16 & 27747_i16;
_9 = (_2, _8);
_14.2 = !_16.2;
_19 = 6886_u16;
_1.1 = _2.0;
Goto(bb12)
}
bb12 = {
_21.0 = (_9.0.0,);
_11 = (-1578_i16) + 6740_i16;
_1.0 = _16.2;
_7.0 = core::ptr::addr_of_mut!(_19);
_21.1 = _9.1 + _9.1;
_19 = 21372_u16;
_14.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_16.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14 = _16;
_9 = _21;
_11 = -(-24938_i16);
_22 = -_17;
_21.0.0 = _2.0;
_5 = 65332020090181366812724617109546719740_i128 & (-42864596376430538174018578703938769619_i128);
_20 = _1.0 as isize;
_20 = -_17;
_2.0 = 111_u8 as i32;
Call(_1.2 = core::intrinsics::transmute(_9.0.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16.0 = _14.0;
_25.1 = -_9.0.0;
_14.1 = !_16.1;
RET = _4;
_17 = _22 | _20;
_19 = 5696_u16 * 64745_u16;
_15 = [_5,_5,_5,_5,_5,_5,_5];
_7.0 = core::ptr::addr_of_mut!(_19);
_3 = _25.1;
_16.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_25 = (_1.0, _9.0.0, _1.2, _1.3);
_8 = -_9.1;
_9.1 = _8 + _8;
_18 = [140_u8,160_u8,141_u8,177_u8,4_u8];
_21 = (_9.0, _8);
_14 = _16;
_1.2 = _25.2 * _25.2;
_31.1 = _14.1;
_1 = (_16.2, _9.0.0, _25.2, _25.3);
_21.1 = _8;
_31.1 = _14.1 & _16.1;
RET = _4;
_31.2 = _14.2;
_17 = _20;
_30 = _22;
Call(_9 = fn8(_21.0, _25.1, _21.0, _21.0, _14, _3, _1, _21.0.0, _25), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_2 = _21.0;
_26 = [4_usize,10650838880108483404_usize];
_29 = 123_u8 as f64;
_5 = (-113084547573588960622743669550829305831_i128);
_8 = _21.1 - _21.1;
_29 = 2_usize as f64;
_24 = _12;
_1.0 = _16.2;
_9.1 = _11 as f32;
_11 = !(-25166_i16);
_21.0.0 = _2.0;
_23 = _6;
_31.0 = _14.0;
_21.0 = (_3,);
_14.1 = !_16.1;
_35 = _25.1 as f64;
_12 = _24 + _24;
_25.3 = _1.3;
_21.1 = _8;
_17 = -_20;
_14.1 = _19 as u64;
_31.2 = !_16.2;
_20 = _22 - _30;
_26 = [4_usize,7_usize];
_31 = _16;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(7_usize, 1_usize, Move(_1), 14_usize, Move(_14), 5_usize, Move(_5), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(7_usize, 30_usize, Move(_30), 20_usize, Move(_20), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(7_usize, 24_usize, Move(_24), 6_usize, Move(_6), 3_usize, Move(_3), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: (i32,),mut _2: i32,mut _3: (i32,),mut _4: (i32,),mut _5: ([u128; 8], u64, i64),mut _6: i32,mut _7: (i64, i32, u32, i8),mut _8: i32,mut _9: (i64, i32, u32, i8)) -> ((i32,), f32) {
mir! {
type RET = ((i32,), f32);
let _10: isize;
let _11: ();
let _12: ();
{
RET.0 = _4;
_7.3 = _9.3;
_4 = (_7.1,);
_9.3 = _7.3 << _5.1;
RET.1 = 13932_u16 as f32;
_9.3 = 27456_u16 as i8;
_9 = _7;
_9.0 = !_7.0;
RET.0 = (_2,);
_5.0 = [218152436203188342521863212886674047225_u128,252358733529983709441422461789846332098_u128,260768379264319687999774523619562681394_u128,170311806663908667156600962417739051228_u128,200225798961136342161494558749392977865_u128,281812151944808922448145682767148723281_u128,172879764390793665934123232133995160290_u128,52632483485067496007612273466333169460_u128];
RET.1 = _9.3 as f32;
_9.3 = -_7.3;
_10 = (-95_isize);
_9 = _7;
_9 = (_7.0, _8, _7.2, _7.3);
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(8_usize, 4_usize, Move(_4), 1_usize, Move(_1), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_11 = dump_var(8_usize, 2_usize, Move(_2), 12_usize, _12, 12_usize, _12, 12_usize, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: (i32,),mut _2: i32,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: Adt55,mut _7: (i32,)) -> i64 {
mir! {
type RET = i64;
let _8: char;
let _9: i8;
let _10: [i128; 5];
let _11: u8;
let _12: char;
let _13: Adt58;
let _14: Adt59;
let _15: ();
let _16: ();
{
_1 = (_7.0,);
_6.fld1 = 4023140816_u32;
RET = !(-5527596387535166993_i64);
_6.fld0 = _7;
_2 = _4;
_7 = _1;
_1 = (_3,);
_6.fld1 = !2103708028_u32;
_6.fld0.0 = _2 + _4;
_9 = -(-10_i8);
_11 = RET as u8;
_5 = _2;
_7 = (_2,);
_10 = [1589775709647078751925780831281299222_i128,(-114116310799647894598838750707001706937_i128),(-31692915359035219440105163300497062860_i128),79259050255287746117670458765922709814_i128,(-81968244993880829017299873134311731640_i128)];
_6.fld0.0 = RET as i32;
RET = (-810463462545774659_i64);
_6.fld1 = 2285303550_u32;
_9 = 25370_u16 as i8;
RET = !652921699867195621_i64;
Call(_1.0 = fn10(_3, _2, _2, _4, _2, _4, _7.0, _3, _3, _7, _5, _7.0, _4, _7.0, _7.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.0 = -_2;
_9 = !(-74_i8);
RET = 5875360632103106918_i64 ^ 570065767740598390_i64;
_9 = (-32_i8) << _5;
_6.fld0 = _1;
_2 = _6.fld0.0;
RET = 1050031045827774789_i64 >> _1.0;
_7.0 = 181873225755715500318056451040183985444_u128 as i32;
_1 = _6.fld0;
_6.fld0 = (_4,);
_9 = 134860275506477641289025029096154305284_i128 as i8;
_6 = Adt55 { fld0: _1,fld1: 3271414460_u32 };
_1 = (_5,);
_4 = _11 as i32;
_4 = _2;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(9_usize, 1_usize, Move(_1), 7_usize, Move(_7), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: i32,mut _10: (i32,),mut _11: i32,mut _12: i32,mut _13: i32,mut _14: i32,mut _15: i32) -> i32 {
mir! {
type RET = i32;
let _16: Adt58;
let _17: Adt51;
let _18: f32;
let _19: Adt60;
let _20: f64;
let _21: ();
let _22: ();
{
_8 = (-9223372036854775808_isize) as i32;
RET = _6 | _6;
_15 = !_11;
_5 = 1145990706_u32 as i32;
_11 = !_2;
RET = _15;
RET = _3 | _7;
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(10_usize, 3_usize, Move(_3), 10_usize, Move(_10), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_21 = dump_var(10_usize, 7_usize, Move(_7), 14_usize, Move(_14), 11_usize, Move(_11), 22_usize, _22), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: (i64, i32, u32, i8),mut _5: f32,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: f64,mut _10: Adt55) -> char {
mir! {
type RET = char;
let _11: usize;
let _12: Adt61;
let _13: Adt55;
let _14: [i16; 3];
let _15: ((i32,), isize);
let _16: *const (*mut u16,);
let _17: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]);
let _18: Adt51;
let _19: i32;
let _20: [usize; 2];
let _21: bool;
let _22: Adt45;
let _23: Adt46;
let _24: bool;
let _25: isize;
let _26: Adt56;
let _27: isize;
let _28: ();
let _29: ();
{
_4.0 = 254_u8 as i64;
_4.3 = -(-19_i8);
_4.2 = 2158_u16 as u32;
_3 = 89423093692595130623035177261062369429_i128 as i32;
_11 = 9215080805428536384_usize;
_10.fld1 = _4.2 & _4.2;
_4.2 = 2241652485805198805_u64 as u32;
_10.fld0.0 = _2 >> _4.1;
_4 = ((-3416163161865523216_i64), _2, _10.fld1, 114_i8);
_10.fld0 = (_7,);
RET = '\u{86c69}';
RET = '\u{379e}';
_4.2 = true as u32;
_13.fld1 = _4.3 as u32;
RET = '\u{e16e9}';
_7 = _8;
_5 = _8 as f32;
_4.2 = _13.fld1;
_10.fld0.0 = RET as i32;
_10.fld0 = (_1,);
_10.fld1 = _13.fld1;
_9 = 198_u8 as f64;
_15 = (_10.fld0, 5_isize);
_8 = _10.fld0.0;
Call(_4.0 = fn12(_6, _1, _4.1, _10.fld1, _1, _4.2, _15.1, _4.3, _5, _13.fld1, _15.1, _6, _5, _15.0, _10.fld0.0, _4.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{92bb0}';
_15 = (_10.fld0, (-9223372036854775808_isize));
_10.fld1 = _4.2;
_10.fld0 = (_2,);
_3 = _15.0.0;
_9 = _5 as f64;
_13.fld1 = _10.fld1;
_17.1.2 = 7646508462580123834_u64 | 9406084694983834984_u64;
_17.3.0 = [283038563104969687975542361368546930632_u128,220972660950525569641526795411874231831_u128,234079748912515693875823442959765171899_u128,226754943605911841104293940395457182302_u128,236156532745713079005036381411712710926_u128,178406472586809253833541689409471111504_u128,171440928202846455255169642645324543556_u128,328596178094214076913302850291286857548_u128];
_13 = _10;
_15.1 = (-9223372036854775808_isize) * 60_isize;
_17.3.0 = [131315000236452810584537791034868114894_u128,201624519569693267083095558573622640811_u128,261209374451299318889034417298386226385_u128,98948232285193877482952331246365722494_u128,71467199674685632540082313108454953599_u128,198984074669881481158997348037518420520_u128,319070621323822065371097396183349028786_u128,211249900189002880747741836524882427506_u128];
_3 = _7 & _1;
_17.7 = _17.3.0;
_4.0 = 4826528172802017348_i64 | (-6679143085081512852_i64);
_4 = (2916862209792151529_i64, _15.0.0, _10.fld1, (-9_i8));
_17.3.1 = !_17.1.2;
_17.3 = (_17.7, _17.1.2, _4.0);
_17.2 = !13156_u16;
match _4.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211447 => bb9,
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
_17.2 = !37868_u16;
_4.2 = _10.fld1;
_17.1.1.0 = core::ptr::addr_of_mut!(_5);
_2 = (-150630378169587410885223411191031152285_i128) as i32;
_17.4 = _4.3;
_4.2 = _11 as u32;
_17.1.1.1.0 = 57_u8 as i32;
_14 = [(-19741_i16),(-5387_i16),(-3579_i16)];
_17.3 = (_17.7, _17.1.2, _4.0);
_17.3.1 = _17.1.2 | _17.1.2;
_17.3.1 = _17.1.2 * _17.1.2;
_10 = Adt55 { fld0: _15.0,fld1: _13.fld1 };
_5 = _17.2 as f32;
_18 = Adt51::Variant2 { fld0: _17.1.1.0,fld1: RET };
_13.fld0.0 = -_15.0.0;
_17.0.0 = core::ptr::addr_of_mut!(_5);
_17.0.1 = [81013982232649388806095797794995815056_i128,(-115819043126373694837642320667629824930_i128),53428249649727964551724903988834141728_i128,20604545840607974511257361574221266524_i128,(-68592518270530651501339836502818359229_i128)];
_15.1 = (-31_isize) ^ 9223372036854775807_isize;
_20 = [_11,_11];
_17.3 = (_17.7, _17.1.2, _4.0);
RET = Field::<char>(Variant(_18, 2), 1);
_14 = [10353_i16,26765_i16,15780_i16];
Goto(bb10)
}
bb10 = {
_2 = _5 as i32;
SetDiscriminant(_18, 2);
_15.0.0 = -_6;
_17.1.1 = (_17.0.0, _15.0);
_15.0.0 = _17.1.1.1.0 - _7;
_17.3.1 = 49_u8 as u64;
_17.1.1 = (_17.0.0, _10.fld0);
_4.3 = _9 as i8;
place!(Field::<char>(Variant(_18, 2), 1)) = RET;
_17.0.2 = _17.3.0;
_7 = _10.fld0.0;
_17.1.1.1.0 = -_3;
_4.2 = _10.fld1 - _13.fld1;
place!(Field::<*mut f32>(Variant(_18, 2), 0)) = _17.1.1.0;
_4.0 = 28_u8 as i64;
Goto(bb11)
}
bb11 = {
_15 = (_13.fld0, (-9223372036854775808_isize));
_17.1.1 = (_17.0.0, _13.fld0);
_10.fld0 = _13.fld0;
SetDiscriminant(_18, 2);
_17.3.1 = !_17.1.2;
_17.7 = [245770958650612310898175127353850482225_u128,262851416127845483237395289329339512771_u128,156493226231609082685775326521319412578_u128,247087713874371949609120153102997825336_u128,76947085076454409201974862431130288296_u128,318190135468245195719697761460726538118_u128,163854224688541647243755610526744505695_u128,225553441868031457356623556040765860763_u128];
_17.1.1 = (_17.0.0, _10.fld0);
_21 = !true;
_17.1.1 = (_17.0.0, _10.fld0);
_17.1.1.1.0 = _3;
_13.fld0 = (_10.fld0.0,);
_13.fld1 = !_4.2;
_13.fld0 = (_3,);
_8 = _3 >> _10.fld0.0;
RET = '\u{2caf9}';
_10 = _13;
_19 = _8;
RET = '\u{3b62}';
_2 = !_6;
Goto(bb12)
}
bb12 = {
RET = '\u{27ec3}';
_17.1.1.1 = _13.fld0;
_11 = 2125329374200793594_usize;
match _17.4 {
0 => bb3,
340282366920938463463374607431768211447 => bb14,
_ => bb13
}
}
bb13 = {
_17.2 = !37868_u16;
_4.2 = _10.fld1;
_17.1.1.0 = core::ptr::addr_of_mut!(_5);
_2 = (-150630378169587410885223411191031152285_i128) as i32;
_17.4 = _4.3;
_4.2 = _11 as u32;
_17.1.1.1.0 = 57_u8 as i32;
_14 = [(-19741_i16),(-5387_i16),(-3579_i16)];
_17.3 = (_17.7, _17.1.2, _4.0);
_17.3.1 = _17.1.2 | _17.1.2;
_17.3.1 = _17.1.2 * _17.1.2;
_10 = Adt55 { fld0: _15.0,fld1: _13.fld1 };
_5 = _17.2 as f32;
_18 = Adt51::Variant2 { fld0: _17.1.1.0,fld1: RET };
_13.fld0.0 = -_15.0.0;
_17.0.0 = core::ptr::addr_of_mut!(_5);
_17.0.1 = [81013982232649388806095797794995815056_i128,(-115819043126373694837642320667629824930_i128),53428249649727964551724903988834141728_i128,20604545840607974511257361574221266524_i128,(-68592518270530651501339836502818359229_i128)];
_15.1 = (-31_isize) ^ 9223372036854775807_isize;
_20 = [_11,_11];
_17.3 = (_17.7, _17.1.2, _4.0);
RET = Field::<char>(Variant(_18, 2), 1);
_14 = [10353_i16,26765_i16,15780_i16];
Goto(bb10)
}
bb14 = {
_4.3 = _17.4 + _17.4;
RET = '\u{ef8ce}';
_20 = [_11,_11];
_17.4 = -_4.3;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(11_usize, 20_usize, Move(_20), 14_usize, Move(_14), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(11_usize, 4_usize, Move(_4), 15_usize, Move(_15), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: u32,mut _5: i32,mut _6: u32,mut _7: isize,mut _8: i8,mut _9: f32,mut _10: u32,mut _11: isize,mut _12: i32,mut _13: f32,mut _14: (i32,),mut _15: i32,mut _16: i8) -> i64 {
mir! {
type RET = i64;
let _17: f32;
let _18: Adt46;
let _19: f64;
let _20: (i32,);
let _21: ((i32,), isize);
let _22: (*mut u16,);
let _23: [i16; 3];
let _24: [usize; 2];
let _25: isize;
let _26: isize;
let _27: bool;
let _28: isize;
let _29: isize;
let _30: Adt59;
let _31: f32;
let _32: ([u128; 8], u64, i64);
let _33: f32;
let _34: *mut f32;
let _35: Adt55;
let _36: (f32, *mut u16);
let _37: Adt51;
let _38: (*const i16, (*mut f32, (i32,)), u64);
let _39: i16;
let _40: Adt57;
let _41: ();
let _42: ();
{
_3 = -_14.0;
RET = 2740983459845276080_i64;
_15 = _2 * _3;
_15 = _1;
_10 = _4;
_5 = -_14.0;
RET = -(-6079587584630020809_i64);
_11 = _7 + _7;
_10 = '\u{abd38}' as u32;
_14 = (_5,);
_8 = _16;
_9 = -_13;
_13 = -_9;
_4 = !_6;
_7 = (-24411_i16) as isize;
_12 = RET as i32;
_16 = _8;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
114 => bb6,
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
_6 = _4;
_15 = _1;
_15 = !_1;
_2 = -_3;
_9 = -_13;
_5 = !_3;
_11 = -_7;
_5 = -_15;
_2 = 85072906836043158726966411764103438951_i128 as i32;
_8 = _16 - _16;
_14 = (_5,);
Call(_19 = fn13(_1, _6, _15, _13, _3, _6, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13 = _9 - _9;
_14 = (_5,);
_4 = !_6;
_17 = _9 + _9;
Call(_8 = core::intrinsics::bswap(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = !_7;
_13 = RET as f32;
_14.0 = _5;
_17 = 3_usize as f32;
_7 = _11;
Call(_9 = fn14(_16, _14, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = (-2637352591692151268_i64);
_5 = -_15;
RET = -(-5787482101324006156_i64);
_15 = _5;
_13 = 12400123463043550949_u64 as f32;
_21.0 = (_3,);
Goto(bb10)
}
bb10 = {
_10 = _16 as u32;
_21.0.0 = 3875905742839202980_u64 as i32;
_21.0 = (_1,);
_24 = [9040697218888587826_usize,10501230548118643069_usize];
_21.1 = -_7;
_3 = -_15;
_11 = _21.1 - _7;
_21.0.0 = _14.0 ^ _1;
_11 = _7;
_20 = (_5,);
_10 = 10_u8 as u32;
_9 = _13;
_16 = _8 ^ _8;
_23 = [(-1625_i16),3096_i16,(-25815_i16)];
_20 = _21.0;
_17 = -_13;
_26 = _21.1 >> _14.0;
_23 = [6665_i16,(-13241_i16),12888_i16];
_1 = _5;
_27 = !true;
_10 = !_6;
_23 = [396_i16,27669_i16,(-30468_i16)];
_6 = !_4;
_10 = _8 as u32;
_21.1 = -_26;
_10 = _4 * _4;
Call(_3 = core::intrinsics::transmute(_15), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = (-6120782192093110353_i64) ^ (-8410795778338165353_i64);
_21.0.0 = _20.0;
_25 = 59261_u16 as isize;
_24 = [14497967706991961983_usize,2_usize];
_13 = 17_u8 as f32;
RET = 131600246826027250571341000716375258254_u128 as i64;
_14 = _20;
_3 = _21.0.0;
_5 = _26 as i32;
_4 = _6 >> _5;
RET = 44_u8 as i64;
_20.0 = _1 + _15;
_21.0 = _20;
Call(_20.0 = core::intrinsics::transmute(_10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_28 = _21.1 << _15;
_2 = !_21.0.0;
_28 = !_26;
Call(_18 = fn15(_20.0, _4, _21, _21.1, _21.0, _14, _21.1, _4, _10, _10, _20.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3 = _14.0 * _14.0;
_4 = RET as u32;
_21.1 = 0_usize as isize;
_2 = _14.0 + _14.0;
SetDiscriminant(_18, 1);
place!(Field::<u128>(Variant(_18, 1), 1)) = 292848384611482436560123659859975030452_u128;
_26 = -_28;
_31 = 50817237718525590902050676735250331529_i128 as f32;
_15 = _3 >> _8;
_21.0.0 = _15 << _28;
_29 = _26;
_14.0 = -_21.0.0;
_23 = [(-30675_i16),(-12413_i16),6455_i16];
_17 = _9;
_29 = -_28;
_35.fld0 = _20;
place!(Field::<((i32,), isize)>(Variant(_18, 1), 3)).1 = _28;
_32.2 = 15064331416349072505_usize as i64;
_15 = 32263_i16 as i32;
match Field::<u128>(Variant(_18, 1), 1) {
0 => bb11,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb12,
5 => bb6,
292848384611482436560123659859975030452 => bb14,
_ => bb7
}
}
bb14 = {
place!(Field::<((i32,), isize)>(Variant(_18, 1), 3)).0 = (_21.0.0,);
_12 = _14.0 - _21.0.0;
_25 = !_26;
_21.0 = (_2,);
_36.0 = _31;
_21 = (_35.fld0, Field::<((i32,), isize)>(Variant(_18, 1), 3).1);
_17 = _13 * _36.0;
_21.1 = _28 & _28;
_38.1.1.0 = _16 as i32;
place!(Field::<bool>(Variant(_18, 1), 0)) = _3 > _38.1.1.0;
_38.1.1.0 = 11673792112359117115_usize as i32;
_31 = _13 + _17;
_38.2 = 9321474643989579887_u64;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(12_usize, 3_usize, Move(_3), 7_usize, Move(_7), 1_usize, Move(_1), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(12_usize, 15_usize, Move(_15), 12_usize, Move(_12), 20_usize, Move(_20), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(12_usize, 8_usize, Move(_8), 28_usize, Move(_28), 4_usize, Move(_4), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i32,mut _2: u32,mut _3: i32,mut _4: f32,mut _5: i32,mut _6: u32,mut _7: u32) -> f64 {
mir! {
type RET = f64;
let _8: i16;
let _9: u32;
let _10: (*mut f32, [i128; 5], [u128; 8]);
let _11: usize;
let _12: Adt57;
let _13: i64;
let _14: ((i32,), f32);
let _15: (((i32,), f32), i32, bool);
let _16: u16;
let _17: ([u128; 8], u64, i64);
let _18: f32;
let _19: isize;
let _20: u128;
let _21: (*mut f32, [i128; 5], [u128; 8]);
let _22: isize;
let _23: [u8; 5];
let _24: Adt51;
let _25: f32;
let _26: u8;
let _27: char;
let _28: Adt54;
let _29: Adt55;
let _30: isize;
let _31: Adt57;
let _32: bool;
let _33: ();
let _34: ();
{
_6 = (-6669925717007153060_i64) as u32;
_5 = _4 as i32;
_8 = (-27904_i16);
_10.2 = [230744445951866728985339812579136957616_u128,127696124496094409148437701432738861624_u128,247125496109589659690172899381520389103_u128,76466734353866368772128078006318608130_u128,11852583107082754831016696658910952091_u128,128766406252546300926197827021821155234_u128,317280179625853399087295299589360339092_u128,128607339361761797667331985583488190637_u128];
_2 = _7 - _7;
_10.0 = core::ptr::addr_of_mut!(_4);
_8 = 16502_i16;
_7 = 41772_u16 as u32;
Goto(bb1)
}
bb1 = {
_8 = 8_i8 as i16;
RET = 1_usize as f64;
_10.1 = [(-69583184927475575224226335354738439471_i128),(-122934919034409235645056121042362880098_i128),(-95818767214051881845400822814465761298_i128),3932493833870936922022910388334169273_i128,(-22135075866287752643531317226099014105_i128)];
_2 = (-45_i8) as u32;
_1 = _3 + _5;
_1 = !_5;
_9 = _2;
_10.2 = [333786113419943068577926459655707402117_u128,167221163004562669026680220578896175223_u128,217752609188020397799672694799271078922_u128,99446280749030120156000723408902679756_u128,255625392912237772363447726729420279354_u128,5122341658572983520093457917299147056_u128,299747764169515515718845528634742514065_u128,44682127450100871532526970520413776428_u128];
_15.0.0.0 = _5 * _1;
_15.0.1 = 4795198969362905021_u64 as f32;
_14.1 = _7 as f32;
_2 = _9 ^ _7;
_17.1 = !6294293064969674562_u64;
_15.0.1 = -_4;
_10.0 = core::ptr::addr_of_mut!(_15.0.1);
_17.0 = _10.2;
_10.0 = core::ptr::addr_of_mut!(_4);
_16 = 55524_u16 - 52739_u16;
Goto(bb2)
}
bb2 = {
_10.2 = _17.0;
_11 = 7_usize;
_21 = _10;
_19 = 84_isize * 9223372036854775807_isize;
_17 = (_21.2, 4053912965167785100_u64, (-2651362497891763608_i64));
_14.0.0 = !_1;
_20 = _10.2[_11];
_14.0.0 = '\u{fe533}' as i32;
_10.2 = _17.0;
_20 = !_17.0[_11];
_3 = _1 & _1;
_4 = -_15.0.1;
_8 = 3644_i16 & (-12979_i16);
Goto(bb3)
}
bb3 = {
_18 = _8 as f32;
_21.1 = [33884251976543629723610293380124527985_i128,(-151133939639666118877657472814209648622_i128),(-67015435870792770284117495952671319242_i128),(-66989769378618054353529565983282002967_i128),(-8457213226445790710927044607438804840_i128)];
RET = _17.1 as f64;
_15.0 = _14;
_20 = !_17.0[_11];
_7 = _6 | _2;
_15.1 = _1;
_9 = !_6;
_9 = _7;
_13 = _17.2;
_15 = (_14, _5, false);
_5 = -_3;
_15 = (_14, _1, true);
_23 = [140_u8,167_u8,217_u8,251_u8,13_u8];
_18 = _4 * _4;
match _21.2[_11] {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
44682127450100871532526970520413776428 => bb8,
_ => bb7
}
}
bb4 = {
_10.2 = _17.0;
_11 = 7_usize;
_21 = _10;
_19 = 84_isize * 9223372036854775807_isize;
_17 = (_21.2, 4053912965167785100_u64, (-2651362497891763608_i64));
_14.0.0 = !_1;
_20 = _10.2[_11];
_14.0.0 = '\u{fe533}' as i32;
_10.2 = _17.0;
_20 = !_17.0[_11];
_3 = _1 & _1;
_4 = -_15.0.1;
_8 = 3644_i16 & (-12979_i16);
Goto(bb3)
}
bb5 = {
_8 = 8_i8 as i16;
RET = 1_usize as f64;
_10.1 = [(-69583184927475575224226335354738439471_i128),(-122934919034409235645056121042362880098_i128),(-95818767214051881845400822814465761298_i128),3932493833870936922022910388334169273_i128,(-22135075866287752643531317226099014105_i128)];
_2 = (-45_i8) as u32;
_1 = _3 + _5;
_1 = !_5;
_9 = _2;
_10.2 = [333786113419943068577926459655707402117_u128,167221163004562669026680220578896175223_u128,217752609188020397799672694799271078922_u128,99446280749030120156000723408902679756_u128,255625392912237772363447726729420279354_u128,5122341658572983520093457917299147056_u128,299747764169515515718845528634742514065_u128,44682127450100871532526970520413776428_u128];
_15.0.0.0 = _5 * _1;
_15.0.1 = 4795198969362905021_u64 as f32;
_14.1 = _7 as f32;
_2 = _9 ^ _7;
_17.1 = !6294293064969674562_u64;
_15.0.1 = -_4;
_10.0 = core::ptr::addr_of_mut!(_15.0.1);
_17.0 = _10.2;
_10.0 = core::ptr::addr_of_mut!(_4);
_16 = 55524_u16 - 52739_u16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_14.0 = (_3,);
_26 = 144_u8;
_8 = !(-14347_i16);
_15.0.1 = _4 * _4;
_1 = _15.1;
_16 = _11 as u16;
_25 = (-16_i8) as f32;
_13 = _17.2 & _17.2;
_21 = (_10.0, _10.1, _17.0);
_8 = !27399_i16;
_17.0[_11] = _21.2[_11] & _21.2[_11];
_8 = 32672_i16;
_22 = _19;
_17.2 = _13 - _13;
match _10.2[_11] {
44682127450100871532526970520413776428 => bb10,
_ => bb9
}
}
bb9 = {
_8 = 8_i8 as i16;
RET = 1_usize as f64;
_10.1 = [(-69583184927475575224226335354738439471_i128),(-122934919034409235645056121042362880098_i128),(-95818767214051881845400822814465761298_i128),3932493833870936922022910388334169273_i128,(-22135075866287752643531317226099014105_i128)];
_2 = (-45_i8) as u32;
_1 = _3 + _5;
_1 = !_5;
_9 = _2;
_10.2 = [333786113419943068577926459655707402117_u128,167221163004562669026680220578896175223_u128,217752609188020397799672694799271078922_u128,99446280749030120156000723408902679756_u128,255625392912237772363447726729420279354_u128,5122341658572983520093457917299147056_u128,299747764169515515718845528634742514065_u128,44682127450100871532526970520413776428_u128];
_15.0.0.0 = _5 * _1;
_15.0.1 = 4795198969362905021_u64 as f32;
_14.1 = _7 as f32;
_2 = _9 ^ _7;
_17.1 = !6294293064969674562_u64;
_15.0.1 = -_4;
_10.0 = core::ptr::addr_of_mut!(_15.0.1);
_17.0 = _10.2;
_10.0 = core::ptr::addr_of_mut!(_4);
_16 = 55524_u16 - 52739_u16;
Goto(bb2)
}
bb10 = {
_22 = _21.2[_11] as isize;
_21 = _10;
_10.0 = core::ptr::addr_of_mut!(_14.1);
match _21.2[_11] {
0 => bb8,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
44682127450100871532526970520413776428 => bb16,
_ => bb15
}
}
bb11 = {
_8 = 8_i8 as i16;
RET = 1_usize as f64;
_10.1 = [(-69583184927475575224226335354738439471_i128),(-122934919034409235645056121042362880098_i128),(-95818767214051881845400822814465761298_i128),3932493833870936922022910388334169273_i128,(-22135075866287752643531317226099014105_i128)];
_2 = (-45_i8) as u32;
_1 = _3 + _5;
_1 = !_5;
_9 = _2;
_10.2 = [333786113419943068577926459655707402117_u128,167221163004562669026680220578896175223_u128,217752609188020397799672694799271078922_u128,99446280749030120156000723408902679756_u128,255625392912237772363447726729420279354_u128,5122341658572983520093457917299147056_u128,299747764169515515718845528634742514065_u128,44682127450100871532526970520413776428_u128];
_15.0.0.0 = _5 * _1;
_15.0.1 = 4795198969362905021_u64 as f32;
_14.1 = _7 as f32;
_2 = _9 ^ _7;
_17.1 = !6294293064969674562_u64;
_15.0.1 = -_4;
_10.0 = core::ptr::addr_of_mut!(_15.0.1);
_17.0 = _10.2;
_10.0 = core::ptr::addr_of_mut!(_4);
_16 = 55524_u16 - 52739_u16;
Goto(bb2)
}
bb12 = {
_10.2 = _17.0;
_11 = 7_usize;
_21 = _10;
_19 = 84_isize * 9223372036854775807_isize;
_17 = (_21.2, 4053912965167785100_u64, (-2651362497891763608_i64));
_14.0.0 = !_1;
_20 = _10.2[_11];
_14.0.0 = '\u{fe533}' as i32;
_10.2 = _17.0;
_20 = !_17.0[_11];
_3 = _1 & _1;
_4 = -_15.0.1;
_8 = 3644_i16 & (-12979_i16);
Goto(bb3)
}
bb13 = {
_10.2 = _17.0;
_11 = 7_usize;
_21 = _10;
_19 = 84_isize * 9223372036854775807_isize;
_17 = (_21.2, 4053912965167785100_u64, (-2651362497891763608_i64));
_14.0.0 = !_1;
_20 = _10.2[_11];
_14.0.0 = '\u{fe533}' as i32;
_10.2 = _17.0;
_20 = !_17.0[_11];
_3 = _1 & _1;
_4 = -_15.0.1;
_8 = 3644_i16 & (-12979_i16);
Goto(bb3)
}
bb14 = {
Return()
}
bb15 = {
_8 = 8_i8 as i16;
RET = 1_usize as f64;
_10.1 = [(-69583184927475575224226335354738439471_i128),(-122934919034409235645056121042362880098_i128),(-95818767214051881845400822814465761298_i128),3932493833870936922022910388334169273_i128,(-22135075866287752643531317226099014105_i128)];
_2 = (-45_i8) as u32;
_1 = _3 + _5;
_1 = !_5;
_9 = _2;
_10.2 = [333786113419943068577926459655707402117_u128,167221163004562669026680220578896175223_u128,217752609188020397799672694799271078922_u128,99446280749030120156000723408902679756_u128,255625392912237772363447726729420279354_u128,5122341658572983520093457917299147056_u128,299747764169515515718845528634742514065_u128,44682127450100871532526970520413776428_u128];
_15.0.0.0 = _5 * _1;
_15.0.1 = 4795198969362905021_u64 as f32;
_14.1 = _7 as f32;
_2 = _9 ^ _7;
_17.1 = !6294293064969674562_u64;
_15.0.1 = -_4;
_10.0 = core::ptr::addr_of_mut!(_15.0.1);
_17.0 = _10.2;
_10.0 = core::ptr::addr_of_mut!(_4);
_16 = 55524_u16 - 52739_u16;
Goto(bb2)
}
bb16 = {
_17.0[_11] = !_21.2[_11];
_21.2 = [_20,_20,_10.2[_11],_20,_10.2[_11],_10.2[_11],_20,_10.2[_11]];
RET = _18 as f64;
_24 = Adt51::Variant2 { fld0: _21.0,fld1: '\u{37764}' };
RET = _7 as f64;
_25 = _15.0.1 - _4;
_1 = _14.0.0 - _15.1;
_14 = (_15.0.0, _15.0.1);
_27 = '\u{b0c3a}';
_6 = _9;
_10.2 = _17.0;
_21.0 = _10.0;
_15.1 = _5 ^ _5;
_17.1 = 4791656950047635959_u64;
_7 = _26 as u32;
_29.fld0 = _14.0;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(13_usize, 27_usize, Move(_27), 8_usize, Move(_8), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(13_usize, 20_usize, Move(_20), 7_usize, Move(_7), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(13_usize, 26_usize, Move(_26), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i8,mut _2: (i32,),mut _3: i32) -> f32 {
mir! {
type RET = f32;
let _4: (i32,);
let _5: [u8; 5];
let _6: bool;
let _7: Adt59;
let _8: (*const i16, (*mut f32, (i32,)), u64);
let _9: Adt45;
let _10: ();
let _11: ();
{
_1 = !(-20_i8);
RET = 17363355098294728170_u64 as f32;
_2 = (_3,);
_2 = (_3,);
_1 = 75_i8 ^ 52_i8;
RET = 46150_u16 as f32;
RET = 12964_i16 as f32;
_3 = _2.0;
_4.0 = !_2.0;
_4.0 = 2183500935317283265_i64 as i32;
Goto(bb1)
}
bb1 = {
_2 = (_3,);
_5 = [131_u8,158_u8,10_u8,1_u8,20_u8];
_4 = _2;
_3 = _2.0;
Goto(bb2)
}
bb2 = {
_2.0 = 124202268890327040714438922254724977212_i128 as i32;
_2.0 = _4.0;
_6 = false & true;
_4 = (_2.0,);
RET = 47744_u16 as f32;
_8.1.1.0 = _3 >> _4.0;
_1 = 75_i8;
_2.0 = _4.0 ^ _3;
_1 = _4.0 as i8;
_2 = (_8.1.1.0,);
_2.0 = 1933032251268555181_u64 as i32;
_5 = [215_u8,68_u8,62_u8,21_u8,193_u8];
_6 = true;
_2.0 = _4.0 * _8.1.1.0;
_4.0 = _2.0 - _3;
_8.1.1 = (_2.0,);
_1 = -(-124_i8);
_5 = [250_u8,7_u8,216_u8,201_u8,0_u8];
_3 = 7_usize as i32;
_3 = !_2.0;
RET = _3 as f32;
Goto(bb3)
}
bb3 = {
Call(_10 = dump_var(14_usize, 1_usize, Move(_1), 4_usize, Move(_4), 6_usize, Move(_6), 11_usize, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i32,mut _2: u32,mut _3: ((i32,), isize),mut _4: isize,mut _5: (i32,),mut _6: (i32,),mut _7: isize,mut _8: u32,mut _9: u32,mut _10: u32,mut _11: i32) -> Adt46 {
mir! {
type RET = Adt46;
let _12: isize;
let _13: f64;
let _14: *mut u16;
let _15: Adt46;
let _16: (f32, *mut u16);
let _17: i128;
let _18: [u8; 5];
let _19: ((i32,), isize);
let _20: usize;
let _21: isize;
let _22: char;
let _23: (*mut f32, [i128; 5], [u128; 8]);
let _24: [i16; 3];
let _25: *mut (*mut f32, (i32,));
let _26: bool;
let _27: f64;
let _28: Adt55;
let _29: [u8; 5];
let _30: Adt45;
let _31: (((i32,), f32), i32, bool);
let _32: char;
let _33: f32;
let _34: [i128; 5];
let _35: usize;
let _36: isize;
let _37: Adt57;
let _38: Adt55;
let _39: ();
let _40: ();
{
_3.1 = _7;
_5 = (_1,);
_5.0 = 33619673539796866961911632412308406304_u128 as i32;
_1 = !_6.0;
_11 = !_1;
_5.0 = (-48_i8) as i32;
_3.0 = (_11,);
_10 = _9 + _9;
_5.0 = -_1;
_6.0 = _11;
_6 = (_5.0,);
_2 = 366855227182010506_i64 as u32;
_4 = _6.0 as isize;
Call(_6.0 = core::intrinsics::transmute(_3.0.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.1 = false as isize;
_12 = _7;
_9 = '\u{801d1}' as u32;
_2 = _8;
_5.0 = _6.0 * _1;
_11 = 114686893220685208150205886876548685021_i128 as i32;
_3 = (_5, _7);
_9 = _10 & _2;
_10 = !_9;
_1 = 0_usize as i32;
_3.0 = (_6.0,);
_7 = !_3.1;
_10 = _8;
_5 = _6;
_8 = !_10;
_3 = (_6, _4);
_8 = !_10;
_3 = (_6, _7);
_7 = 121_i8 as isize;
Goto(bb2)
}
bb2 = {
_8 = _10;
_12 = -_3.1;
_3.0.0 = !_6.0;
_3.0 = (_6.0,);
_13 = 108_i8 as f64;
_13 = _3.0.0 as f64;
Call(_6 = fn16(_4, _3.1, _9, _12, _3.0.0, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = (_3.0.0,);
Call(_3 = fn17(_13, _10, _4, _10, _4, _4, _6.0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5.0 = _3.0.0;
_3 = (_6, _12);
_12 = -_3.1;
_6 = (_3.0.0,);
Goto(bb5)
}
bb5 = {
_13 = _3.1 as f64;
_3.0 = (_6.0,);
Goto(bb6)
}
bb6 = {
_1 = _5.0 - _3.0.0;
_5.0 = _1;
_10 = !_8;
_12 = (-73_i8) as isize;
_3.1 = _4;
_3.0 = (_1,);
_6 = (_3.0.0,);
Goto(bb7)
}
bb7 = {
_6.0 = _3.0.0 + _5.0;
_3.0.0 = -_6.0;
_6.0 = 173_u8 as i32;
_3.0 = (_5.0,);
_13 = _10 as f64;
_6 = (_3.0.0,);
_7 = _3.1 << _3.0.0;
_12 = _4;
_3.0 = _5;
_7 = !_12;
_16.0 = 42761814613666753056491875734969186044_i128 as f32;
_16.0 = 56130_u16 as f32;
_3 = (_5, _12);
_1 = !_5.0;
_4 = !_3.1;
_5.0 = _1 * _6.0;
_3.0 = _6;
_5.0 = _3.0.0;
_1 = _6.0;
_17 = 8815278627520121365254804709902497158_i128;
_3.1 = _12;
Call(_3.0.0 = core::intrinsics::bswap(_5.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5.0 = '\u{a28e8}' as i32;
_3.1 = !_7;
_6 = (_3.0.0,);
_1 = _13 as i32;
_19.0 = _3.0;
_7 = -_4;
_19.0 = _3.0;
_19.0.0 = _6.0 & _3.0.0;
_19 = _3;
_16.0 = _19.1 as f32;
_17 = _13 as i128;
_16.0 = _19.0.0 as f32;
_3.0.0 = !_19.0.0;
_19.1 = !_4;
_7 = _4;
_4 = _12 >> _6.0;
_9 = _2;
_20 = _13 as usize;
_19.1 = _3.1 - _12;
_7 = -_3.1;
Goto(bb9)
}
bb9 = {
_1 = _20 as i32;
_9 = _2;
_3.0.0 = -_19.0.0;
Call(_12 = core::intrinsics::transmute(_20), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = [173_u8,82_u8,82_u8,46_u8,162_u8];
_9 = !_2;
_3.1 = _4;
_21 = _7;
_20 = 32446_i16 as usize;
Goto(bb11)
}
bb11 = {
_19.1 = _12 * _3.1;
_18 = [15_u8,65_u8,191_u8,26_u8,214_u8];
_12 = _20 as isize;
_19.0.0 = _6.0 ^ _6.0;
_6.0 = _4 as i32;
_23.0 = core::ptr::addr_of_mut!(_16.0);
_11 = _3.0.0 >> _17;
_23.1 = [_17,_17,_17,_17,_17];
_17 = 103472032109380477655264734413458348877_i128;
_2 = _10;
_4 = !_7;
_3.0.0 = -_19.0.0;
_13 = 65394657299203251235430112459931603848_u128 as f64;
_8 = _2 & _9;
_7 = _19.1 << _19.0.0;
_28 = Adt55 { fld0: _6,fld1: _8 };
_7 = _21;
match _17 {
0 => bb1,
1 => bb2,
2 => bb5,
103472032109380477655264734413458348877 => bb12,
_ => bb4
}
}
bb12 = {
_4 = _20 as isize;
_15 = Adt46::Variant1 { fld0: false,fld1: 338493592037820903681961288692813354290_u128,fld2: _18,fld3: _3 };
_28 = Adt55 { fld0: _19.0,fld1: _8 };
_20 = 5_usize - 12551992689349691464_usize;
_19.0.0 = _3.0.0;
_23.2 = [234584131225471945624249509619694582328_u128,81058637012857078944257043302211197045_u128,64389811220118955310530623420633684303_u128,184896129658390029820928532941207551120_u128,96389450010273857842710243341887308247_u128,180554227085427439164827439530228088469_u128,24245642450089163170299640855070209617_u128,220535611502977799587835076285161572973_u128];
place!(Field::<((i32,), isize)>(Variant(_15, 1), 3)).1 = _7;
place!(Field::<u128>(Variant(_15, 1), 1)) = _6.0 as u128;
place!(Field::<u128>(Variant(_15, 1), 1)) = _17 as u128;
_3.0 = (Field::<((i32,), isize)>(Variant(_15, 1), 3).0.0,);
_22 = '\u{c4336}';
_16.0 = _8 as f32;
_28.fld0.0 = 15477577468851860195_u64 as i32;
_19.0 = _3.0;
match _17 {
0 => bb4,
103472032109380477655264734413458348877 => bb13,
_ => bb5
}
}
bb13 = {
_19.1 = _3.1 << Field::<((i32,), isize)>(Variant(_15, 1), 3).0.0;
_31.0.0 = (_1,);
_1 = _31.0.0.0 ^ _11;
_31.0.0.0 = _19.0.0;
place!(Field::<((i32,), isize)>(Variant(_15, 1), 3)) = (_3.0, _19.1);
place!(Field::<bool>(Variant(_15, 1), 0)) = !true;
_16.0 = 19969_i16 as f32;
_28.fld1 = _31.0.0.0 as u32;
_23.2 = [Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1),Field::<u128>(Variant(_15, 1), 1)];
_28.fld0 = _6;
_28 = Adt55 { fld0: _31.0.0,fld1: _2 };
SetDiscriminant(_15, 2);
_34 = [_17,_17,_17,_17,_17];
_9 = !_2;
_31.0.0 = _28.fld0;
place!(Field::<(*mut f32, [i128; 5], [u128; 8])>(Variant(_15, 2), 1)).2 = _23.2;
_31.0 = (_6, _16.0);
_15 = Adt46::Variant1 { fld0: true,fld1: 11179863674398450796738813550956592858_u128,fld2: _18,fld3: _19 };
_19.0 = _3.0;
_17 = (-50058031780303874896530445517775785707_i128);
_5 = (_19.0.0,);
_31.1 = _3.0.0;
_8 = !_10;
Goto(bb14)
}
bb14 = {
_33 = _16.0;
_22 = '\u{37e3c}';
_23.1 = _34;
_19.0 = (_31.1,);
place!(Field::<((i32,), isize)>(Variant(_15, 1), 3)).0.0 = _11 ^ _28.fld0.0;
_24 = [14962_i16,(-22720_i16),1516_i16];
place!(Field::<u128>(Variant(_15, 1), 1)) = 159_u8 as u128;
_5 = (Field::<((i32,), isize)>(Variant(_15, 1), 3).0.0,);
_3.0.0 = _31.1 | _19.0.0;
_26 = false & false;
_18 = [33_u8,249_u8,23_u8,95_u8,197_u8];
place!(Field::<((i32,), isize)>(Variant(_15, 1), 3)).0 = (_28.fld0.0,);
_22 = '\u{ff459}';
_19 = Field::<((i32,), isize)>(Variant(_15, 1), 3);
_27 = -_13;
place!(Field::<[u8; 5]>(Variant(_15, 1), 2)) = [132_u8,64_u8,72_u8,225_u8,21_u8];
place!(Field::<((i32,), isize)>(Variant(_15, 1), 3)) = (_31.0.0, _3.1);
match _17 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb9,
4 => bb12,
5 => bb8,
290224335140634588566844161913992425749 => bb16,
_ => bb15
}
}
bb15 = {
_19.1 = _12 * _3.1;
_18 = [15_u8,65_u8,191_u8,26_u8,214_u8];
_12 = _20 as isize;
_19.0.0 = _6.0 ^ _6.0;
_6.0 = _4 as i32;
_23.0 = core::ptr::addr_of_mut!(_16.0);
_11 = _3.0.0 >> _17;
_23.1 = [_17,_17,_17,_17,_17];
_17 = 103472032109380477655264734413458348877_i128;
_2 = _10;
_4 = !_7;
_3.0.0 = -_19.0.0;
_13 = 65394657299203251235430112459931603848_u128 as f64;
_8 = _2 & _9;
_7 = _19.1 << _19.0.0;
_28 = Adt55 { fld0: _6,fld1: _8 };
_7 = _21;
match _17 {
0 => bb1,
1 => bb2,
2 => bb5,
103472032109380477655264734413458348877 => bb12,
_ => bb4
}
}
bb16 = {
_34 = [_17,_17,_17,_17,_17];
_5.0 = !_31.1;
_31.0.0.0 = _5.0 | _28.fld0.0;
_13 = _27;
RET = Adt46::Variant1 { fld0: _26,fld1: Field::<u128>(Variant(_15, 1), 1),fld2: _18,fld3: _19 };
place!(Field::<((i32,), isize)>(Variant(RET, 1), 3)) = (_31.0.0, _21);
_23.1 = [_17,_17,_17,_17,_17];
_8 = _9;
_31.0.0.0 = Field::<((i32,), isize)>(Variant(RET, 1), 3).0.0 << _21;
_22 = '\u{cae4e}';
_28 = Adt55 { fld0: Field::<((i32,), isize)>(Variant(_15, 1), 3).0,fld1: _8 };
Goto(bb17)
}
bb17 = {
Call(_39 = dump_var(15_usize, 3_usize, Move(_3), 12_usize, Move(_12), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(15_usize, 17_usize, Move(_17), 20_usize, Move(_20), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(15_usize, 8_usize, Move(_8), 24_usize, Move(_24), 40_usize, _40, 40_usize, _40), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: u32,mut _4: isize,mut _5: i32,mut _6: isize) -> (i32,) {
mir! {
type RET = (i32,);
let _7: ();
let _8: ();
{
RET = (_5,);
_1 = _2;
_3 = false as u32;
RET.0 = _5;
_4 = _6 >> _2;
_3 = 715652671_u32 & 1083176920_u32;
RET.0 = 115508443218345756342861187218274054661_i128 as i32;
RET = (_5,);
RET = (_5,);
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(16_usize, 6_usize, Move(_6), 2_usize, Move(_2), 1_usize, Move(_1), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f64,mut _2: u32,mut _3: isize,mut _4: u32,mut _5: isize,mut _6: isize,mut _7: i32,mut _8: isize) -> ((i32,), isize) {
mir! {
type RET = ((i32,), isize);
let _9: f64;
let _10: i16;
let _11: ([u128; 8], u64, i64);
let _12: f64;
let _13: f32;
let _14: char;
let _15: Adt53;
let _16: ();
let _17: ();
{
RET.1 = _3;
_4 = _2 - _2;
Goto(bb1)
}
bb1 = {
RET.0.0 = (-6064203851699109666_i64) as i32;
RET.1 = 191702943167943298233611110364123841219_u128 as isize;
_9 = _1 + _1;
_9 = _1 * _1;
_3 = _8;
_6 = 7032286386151497974_i64 as isize;
_5 = 46340_u16 as isize;
_1 = -_9;
_10 = (-19009_i16);
RET.1 = _8 + _3;
RET.0 = (_7,);
RET.1 = _3 >> _3;
_7 = _10 as i32;
RET.0.0 = _7 * _7;
RET.1 = _10 as isize;
_5 = 180_u8 as isize;
_11.2 = 112389060865980671539321796218026317760_i128 as i64;
_11.1 = 14471290850413885793_u64 & 1628416232600543886_u64;
Call(_5 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = (-147360290522573681109447237398721811011_i128) as i16;
_11.0 = [92377111697755857256292293813863043304_u128,249513490011931809129603891131951769738_u128,72767233728358825503018989912928693059_u128,214347583473191528284635753757988177414_u128,149819472029819596816225581283958323313_u128,102477674518294411443070704296525145728_u128,192270578666745737833887557577831633164_u128,202773204450236158207999637306203303697_u128];
_4 = _2;
_5 = _3;
_13 = 244_u8 as f32;
_12 = 157568800797353574491253770617299084649_i128 as f64;
_11.1 = 10505214281041685381_u64 << _3;
_4 = !_2;
_8 = _3;
_2 = _4;
_6 = _5 & _8;
Call(RET.0.0 = core::intrinsics::bswap(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = 12715_u16 as f64;
_11.0 = [339327710683114080171631940540200733766_u128,162174494146893693839654419761731939054_u128,228990413341085574379340889194205934296_u128,323923553665059161225825746288710942653_u128,109152195219920251471882063260084934604_u128,197169853896858307692945847129928439250_u128,65947113151861222687918537168511577766_u128,182853777884820647897912878853223546541_u128];
RET.1 = _5;
_8 = _2 as isize;
RET.0.0 = _9 as i32;
Goto(bb4)
}
bb4 = {
Call(_16 = dump_var(17_usize, 2_usize, Move(_2), 6_usize, Move(_6), 4_usize, Move(_4), 11_usize, Move(_11)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i64,mut _2: [i128; 5],mut _3: ((i32,), f32),mut _4: (i32,),mut _5: (i32,),mut _6: (i32,),mut _7: isize,mut _8: i16,mut _9: i32) -> char {
mir! {
type RET = char;
let _10: bool;
let _11: (i32,);
let _12: Adt49;
let _13: Adt56;
let _14: [i128; 5];
let _15: (((i32,), f32), i32, bool);
let _16: isize;
let _17: i8;
let _18: i8;
let _19: isize;
let _20: ([u128; 8], u64, i64);
let _21: f32;
let _22: char;
let _23: [i16; 3];
let _24: Adt61;
let _25: ();
let _26: ();
{
RET = '\u{b5b71}';
_3.0 = (_9,);
_3.0.0 = _5.0 + _6.0;
_3.0 = (_5.0,);
RET = '\u{6dd14}';
_6 = _3.0;
Goto(bb1)
}
bb1 = {
_2 = [(-132709899170718630669854975183811469879_i128),63100227571854706721424945429906658457_i128,(-33539794236932857063204763318116969723_i128),(-128718035759571241768596492259629847110_i128),147530483839299554029939454213511754275_i128];
_5 = (_3.0.0,);
_5 = (_6.0,);
_7 = (-9223372036854775808_isize);
_10 = true;
_3.0.0 = _10 as i32;
_4.0 = -_6.0;
Goto(bb2)
}
bb2 = {
_3.0.0 = -_4.0;
_8 = (-23098_i16);
_1 = -(-7070793575035523837_i64);
_6 = _3.0;
_2 = [59997846825301728889349519111014475814_i128,87395152244665696034298770304878235180_i128,87107013288986248201250353268111776954_i128,71885776446231477357998935534554224623_i128,5897043992685202264588524857258803001_i128];
_11.0 = 15613727589541374569_u64 as i32;
_4.0 = _6.0;
_3.0 = (_5.0,);
_3.1 = _1 as f32;
Goto(bb3)
}
bb3 = {
_11 = _3.0;
_1 = 4403917286255341724_u64 as i64;
_11 = (_5.0,);
_10 = true | false;
_4.0 = _9 << _11.0;
_2 = [11139862388626018035668113868256978690_i128,(-144821374187040840982955491163439276113_i128),(-147520355230885413231785806648953651346_i128),(-81581519168360598392432415607459453679_i128),3696247964235699706336186757307380638_i128];
_13.fld0 = (_8, 51021_u16);
_3.0.0 = _9 << _5.0;
_13.fld2 = !220431132472798302563552419615917638690_u128;
_3.0 = (_5.0,);
_13.fld0.0 = -_8;
_6.0 = _5.0 ^ _4.0;
RET = '\u{5efe0}';
_3.1 = (-8426518851925255837129469964524912930_i128) as f32;
RET = '\u{10c58a}';
_11 = (_9,);
_13.fld1.0 = _11.0 + _4.0;
_13.fld0.0 = _8;
_13.fld0 = (_8, 52339_u16);
RET = '\u{97472}';
_3.0.0 = _4.0;
_7 = _10 as isize;
_8 = _13.fld0.0;
_14 = [(-167751303506679887065854679158228176257_i128),(-95193987850140570948368352231283755993_i128),168518318176176506333578132015165386226_i128,(-81248672784417483660282870941295189581_i128),42608658541048027238981310319521673214_i128];
RET = '\u{456b3}';
_15.2 = !_10;
_6 = _13.fld1;
match _8 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768188358 => bb11,
_ => bb10
}
}
bb4 = {
_3.0.0 = -_4.0;
_8 = (-23098_i16);
_1 = -(-7070793575035523837_i64);
_6 = _3.0;
_2 = [59997846825301728889349519111014475814_i128,87395152244665696034298770304878235180_i128,87107013288986248201250353268111776954_i128,71885776446231477357998935534554224623_i128,5897043992685202264588524857258803001_i128];
_11.0 = 15613727589541374569_u64 as i32;
_4.0 = _6.0;
_3.0 = (_5.0,);
_3.1 = _1 as f32;
Goto(bb3)
}
bb5 = {
_2 = [(-132709899170718630669854975183811469879_i128),63100227571854706721424945429906658457_i128,(-33539794236932857063204763318116969723_i128),(-128718035759571241768596492259629847110_i128),147530483839299554029939454213511754275_i128];
_5 = (_3.0.0,);
_5 = (_6.0,);
_7 = (-9223372036854775808_isize);
_10 = true;
_3.0.0 = _10 as i32;
_4.0 = -_6.0;
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
Return()
}
bb11 = {
_2 = _14;
_4 = (_11.0,);
_16 = _7 | _7;
_15.2 = !_10;
_11 = (_3.0.0,);
_4 = (_5.0,);
_15 = (_3, _3.0.0, _10);
_11.0 = _15.1;
_1 = 4869034066363203733_i64;
_11.0 = _13.fld1.0 + _13.fld1.0;
_1 = (-5754326778959997047_i64) << _11.0;
_13.fld0.1 = 78768232638458028001481658994153875930_i128 as u16;
_13.fld1.0 = _1 as i32;
RET = '\u{21fe0}';
_3.1 = -_15.0.1;
_3.0.0 = -_9;
RET = '\u{a2b11}';
_15.0.1 = _3.1;
_2 = [(-152659593934963431097846697337842832995_i128),22012295434912678474838256100394211095_i128,(-23880407251051990633389978651718597885_i128),(-12354486311228895735817963804529446263_i128),5466632525790247743971285187053791849_i128];
_13.fld1.0 = _11.0 >> _11.0;
_20.1 = 16765441349747763915_u64 + 4677689061260185015_u64;
Goto(bb12)
}
bb12 = {
_3.1 = -_15.0.1;
_13.fld0.1 = (-66_i8) as u16;
match _8 {
0 => bb13,
1 => bb14,
340282366920938463463374607431768188358 => bb16,
_ => bb15
}
}
bb13 = {
_11 = _3.0;
_1 = 4403917286255341724_u64 as i64;
_11 = (_5.0,);
_10 = true | false;
_4.0 = _9 << _11.0;
_2 = [11139862388626018035668113868256978690_i128,(-144821374187040840982955491163439276113_i128),(-147520355230885413231785806648953651346_i128),(-81581519168360598392432415607459453679_i128),3696247964235699706336186757307380638_i128];
_13.fld0 = (_8, 51021_u16);
_3.0.0 = _9 << _5.0;
_13.fld2 = !220431132472798302563552419615917638690_u128;
_3.0 = (_5.0,);
_13.fld0.0 = -_8;
_6.0 = _5.0 ^ _4.0;
RET = '\u{5efe0}';
_3.1 = (-8426518851925255837129469964524912930_i128) as f32;
RET = '\u{10c58a}';
_11 = (_9,);
_13.fld1.0 = _11.0 + _4.0;
_13.fld0.0 = _8;
_13.fld0 = (_8, 52339_u16);
RET = '\u{97472}';
_3.0.0 = _4.0;
_7 = _10 as isize;
_8 = _13.fld0.0;
_14 = [(-167751303506679887065854679158228176257_i128),(-95193987850140570948368352231283755993_i128),168518318176176506333578132015165386226_i128,(-81248672784417483660282870941295189581_i128),42608658541048027238981310319521673214_i128];
RET = '\u{456b3}';
_15.2 = !_10;
_6 = _13.fld1;
match _8 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768188358 => bb11,
_ => bb10
}
}
bb14 = {
Return()
}
bb15 = {
_3.0.0 = -_4.0;
_8 = (-23098_i16);
_1 = -(-7070793575035523837_i64);
_6 = _3.0;
_2 = [59997846825301728889349519111014475814_i128,87395152244665696034298770304878235180_i128,87107013288986248201250353268111776954_i128,71885776446231477357998935534554224623_i128,5897043992685202264588524857258803001_i128];
_11.0 = 15613727589541374569_u64 as i32;
_4.0 = _6.0;
_3.0 = (_5.0,);
_3.1 = _1 as f32;
Goto(bb3)
}
bb16 = {
_11.0 = _9 | _4.0;
RET = '\u{e8954}';
_12 = Adt49::Variant0 { fld0: _20.1 };
_18 = 113_i8 & (-109_i8);
_20.0 = [_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2];
_19 = (-124985451007693677557593289319915620253_i128) as isize;
_5.0 = !_15.0.0.0;
_13.fld0.0 = _8 - _8;
_20.1 = Field::<u64>(Variant(_12, 0), 0);
RET = '\u{a0e20}';
_3.0 = _15.0.0;
RET = '\u{9e2ec}';
_23 = [_13.fld0.0,_13.fld0.0,_13.fld0.0];
_12 = Adt49::Variant0 { fld0: _20.1 };
_12 = Adt49::Variant0 { fld0: _20.1 };
_5.0 = _4.0 << _1;
_15 = (_3, _3.0.0, _10);
_14 = _2;
_4 = (_9,);
_3.1 = _15.0.1;
_13.fld1.0 = _15.1 << _15.1;
_13.fld0 = (_8, 38363_u16);
_21 = _15.0.1 * _3.1;
_17 = _18 - _18;
_9 = _15.2 as i32;
_13.fld2 = 295258371859407784123173825104498153018_u128;
_13.fld1 = (_3.0.0,);
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(18_usize, 10_usize, Move(_10), 14_usize, Move(_14), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(18_usize, 6_usize, Move(_6), 4_usize, Move(_4), 19_usize, Move(_19), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(95_u8), std::hint::black_box(17974714309156343618_u64), std::hint::black_box(12933255067797767974_usize), std::hint::black_box(16954_i16), std::hint::black_box(2137269799_i32), std::hint::black_box(15292_u16), std::hint::black_box((-1439393641026227026418879305277867112_i128)));
                
            }
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]),
fld1: *const i16,

},
Variant1{
fld0: u8,
fld1: *mut u16,
fld2: [usize; 2],
fld3: (i64, i32, u32, i8),
fld4: (*mut f32, [i128; 5], [u128; 8]),
fld5: [i16; 3],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: (*const i16, (*mut f32, (i32,)), u64),
fld1: [u8; 5],

},
Variant1{
fld0: bool,
fld1: u128,
fld2: [u8; 5],
fld3: ((i32,), isize),

},
Variant2{
fld0: bool,
fld1: (*mut f32, [i128; 5], [u128; 8]),
fld2: [i128; 5],
fld3: *mut u16,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]),

},
Variant1{
fld0: bool,
fld1: (*mut f32, [i128; 5], [u128; 8]),
fld2: (i16, u16),
fld3: *const (*mut u16,),

},
Variant2{
fld0: (i32,),
fld1: *mut (*mut f32, (i32,)),
fld2: (((i32,), f32), i32, bool),
fld3: (i16, u16),
fld4: Adt46,
fld5: i32,

},
Variant3{
fld0: u64,
fld1: (((i32,), f32), i32, bool),
fld2: (i16, u16),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: f32,
fld1: *const i16,
fld2: u64,
fld3: i8,
fld4: i16,
fld5: i32,
fld6: i64,
fld7: [i128; 7],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u64,

},
Variant1{
fld0: [i128; 7],
fld1: *mut f32,
fld2: [usize; 2],
fld3: (i64, i32, u32, i8),
fld4: Adt45,
fld5: [u128; 8],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: u32,
fld1: Adt46,
fld2: isize,
fld3: (*const i16, (*mut f32, (i32,)), u64),
fld4: Adt45,
fld5: [u8; 5],
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: ((i32,), isize),
fld1: (i32,),
fld2: isize,
fld3: [i128; 7],
fld4: Adt48,
fld5: *const (*mut u16,),

},
Variant1{
fld0: *const i32,

},
Variant2{
fld0: *mut f32,
fld1: char,

},
Variant3{
fld0: *const (*mut u16,),
fld1: Adt49,
fld2: Adt46,
fld3: Adt50,
fld4: u16,
fld5: u8,
fld6: *mut u16,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: bool,
fld1: Adt49,
fld2: (*mut f32, (i32,)),
fld3: f32,
fld4: Adt47,
fld5: *const i16,
fld6: [u8; 5],

},
Variant1{
fld0: ((*mut f32, (i32,)), (i16, u16), ([u128; 8], u64, i64), (*mut f32, [i128; 5], [u128; 8]), [i16; 3]),
fld1: Adt46,
fld2: *mut (*mut f32, (i32,)),
fld3: [u128; 8],
fld4: (*mut f32, [i128; 5], [u128; 8]),

},
Variant2{
fld0: Adt45,
fld1: *mut u16,
fld2: i64,
fld3: (i64, i32, u32, i8),
fld4: *mut f32,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const f32,
fld1: [u8; 5],
fld2: u8,
fld3: Adt51,
fld4: Adt49,

},
Variant1{
fld0: Adt52,
fld1: char,
fld2: Adt48,
fld3: ((*mut f32, (i32,)), (i16, u16), ([u128; 8], u64, i64), (*mut f32, [i128; 5], [u128; 8]), [i16; 3]),
fld4: *const (*mut u16,),
fld5: [usize; 2],
fld6: [i128; 7],

},
Variant2{
fld0: u128,
fld1: (i32,),
fld2: (*mut u16,),
fld3: i32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: usize,
fld1: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]),
fld2: u64,
fld3: *mut u16,
fld4: [u8; 5],

},
Variant1{
fld0: u16,
fld1: *const i32,
fld2: Adt51,
fld3: *mut u16,
fld4: [usize; 2],
fld5: i128,
fld6: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]),

},
Variant2{
fld0: (*mut f32, [i128; 5], [u128; 8]),
fld1: *mut f32,
fld2: (i64, i32, u32, i8),
fld3: i8,
fld4: *const i32,
fld5: [i128; 5],
fld6: u32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: (i32,),
fld1: u32,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: (i16, u16),
fld1: (i32,),
fld2: u128,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *const f32,
fld1: *const (*mut u16,),
fld2: Adt54,
fld3: u16,
fld4: *mut f32,
fld5: f32,
fld6: *const i16,
fld7: f64,

},
Variant1{
fld0: [i128; 5],
fld1: [i128; 7],
fld2: Adt50,
fld3: *const (*mut u16,),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: bool,
fld1: Adt50,
fld2: i32,

},
Variant1{
fld0: [i128; 7],
fld1: (f32, *mut u16),
fld2: *const (*mut u16,),
fld3: *mut f32,
fld4: Adt48,

},
Variant2{
fld0: [i16; 3],
fld1: ([u128; 8], u64, i64),
fld2: Adt54,
fld3: *const i16,
fld4: (*mut f32, (i32,)),
fld5: (i32,),
fld6: Adt45,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: f32,
fld1: ((*mut f32, [i128; 5], [u128; 8]), (*const i16, (*mut f32, (i32,)), u64), u16, ([u128; 8], u64, i64), i8, *const (*mut u16,), f64, [u128; 8]),

},
Variant1{
fld0: [usize; 2],
fld1: (*mut f32, (i32,)),

},
Variant2{
fld0: (((i32,), f32), i32, bool),
fld1: u64,
fld2: Adt58,
fld3: Adt53,
fld4: i16,
fld5: ((i32,), f32),

},
Variant3{
fld0: Adt48,
fld1: Adt49,
fld2: [i128; 5],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: bool,
fld1: *const (*mut u16,),

},
Variant1{
fld0: Adt59,
fld1: (*const i16, (*mut f32, (i32,)), u64),
fld2: (i64, i32, u32, i8),

},
Variant2{
fld0: *mut f32,
fld1: (*mut f32, (i32,)),
fld2: isize,

},
Variant3{
fld0: (*mut f32, (i32,)),
fld1: (i16, u16),
fld2: Adt48,
fld3: *mut u16,

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: Adt55,
fld1: (*mut f32, [i128; 5], [u128; 8]),
fld2: Adt60,
fld3: Adt58,
fld4: *const i32,

},
Variant1{
fld0: (*const i16, (*mut f32, (i32,)), u64),
fld1: Adt51,
fld2: ((i32,), isize),
fld3: Adt60,

}}

