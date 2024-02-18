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
pub fn fn0(mut _1: i32,mut _2: isize) -> f64 {
mir! {
type RET = f64;
let _3: [u8; 1];
let _4: isize;
let _5: f32;
let _6: [char; 6];
let _7: Adt30;
let _8: [u8; 8];
let _9: (u64, char);
let _10: (char, (i128, isize, i8, (isize, i128, u16)), *mut f32);
let _11: (*const i8, (i16,));
let _12: u32;
let _13: f64;
let _14: [i128; 1];
let _15: isize;
let _16: i16;
let _17: (((i64, bool, [bool; 7], i64), (*mut Adt27, i32, u128, &'static usize)), [i16; 7], [bool; 7], ([i32; 7], u64));
let _18: &'static *const [u32; 6];
let _19: u64;
let _20: i8;
let _21: isize;
let _22: f32;
let _23: u128;
let _24: bool;
let _25: [i32; 7];
let _26: *mut u64;
let _27: Adt27;
let _28: char;
let _29: f32;
let _30: [i128; 1];
let _31: bool;
let _32: Adt30;
let _33: char;
let _34: u8;
let _35: Adt58;
let _36: &'static (*mut Adt27,);
let _37: ();
let _38: ();
{
_1 = 1071209011_i32;
RET = (-99_isize) as f64;
RET = (-4_i8) as f64;
_2 = 9968119491141529850_usize as isize;
RET = _1 as f64;
_3 = [166_u8];
_1 = (-578397185_i32) + (-1864664270_i32);
_3 = [198_u8];
RET = 17386785958188073366_u64 as f64;
_3 = [46_u8];
_4 = !_2;
_4 = _2 ^ _2;
_4 = _2;
_4 = _2 << _1;
_4 = _2;
RET = _2 as f64;
_1 = 1035550558_i32 * (-469133456_i32);
_2 = _4;
_4 = !_2;
Goto(bb1)
}
bb1 = {
_1 = 619845020_i32 - 341279155_i32;
_4 = 13_i8 as isize;
_6 = ['\u{b45a7}','\u{a4e4d}','\u{465d1}','\u{bd0d7}','\u{8893e}','\u{2b1e9}'];
_5 = (-30987322910753382834117563222048514719_i128) as f32;
_3 = [130_u8];
_7 = Adt30::Variant3 { fld0: (-14009_i16),fld1: RET };
place!(Field::<i16>(Variant(_7, 3), 0)) = (-7492_i16);
_7 = Adt30::Variant2 { fld0: 9738536661518153875_usize };
_7 = Adt30::Variant1 { fld0: (-104_i8) };
_2 = !_4;
_6 = ['\u{102cf4}','\u{2e845}','\u{f5c41}','\u{c7f76}','\u{53ea6}','\u{ebdc6}'];
_8 = [196_u8,250_u8,21_u8,199_u8,171_u8,212_u8,116_u8,250_u8];
_6 = ['\u{d2e6b}','\u{43e4d}','\u{12ed3}','\u{8a6c4}','\u{4ee9c}','\u{b13a7}'];
_2 = -_4;
_5 = 3153301986462876298_i64 as f32;
_2 = !_4;
_9 = (18337804305938899793_u64, '\u{a453b}');
place!(Field::<i8>(Variant(_7, 1), 0)) = -(-8_i8);
_8 = [71_u8,255_u8,114_u8,217_u8,166_u8,63_u8,49_u8,141_u8];
RET = 159002738044154629313187588440903145089_i128 as f64;
_9.0 = 15079871956370287788_u64 ^ 16222681232463143340_u64;
_7 = Adt30::Variant3 { fld0: 12979_i16,fld1: RET };
_8 = [169_u8,238_u8,130_u8,254_u8,65_u8,183_u8,10_u8,153_u8];
place!(Field::<i16>(Variant(_7, 3), 0)) = 14644045818979261942_usize as i16;
_2 = _4;
_2 = _4 >> _9.0;
_2 = 153478160697391672437469792351384376032_i128 as isize;
_6 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
Call(_9 = fn1(_2, Field::<f64>(Variant(_7, 3), 1), _8, _6, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.1 = '\u{d762b}';
_10.2 = core::ptr::addr_of_mut!(_5);
SetDiscriminant(_7, 3);
_10.1.0 = 133976459424307991995472089116886086167_i128 ^ 33662414849434818250526456315389775229_i128;
_9.0 = !454875106123689018_u64;
_2 = -_4;
RET = 85_i8 as f64;
_11.1.0 = 18479_i16;
_10.0 = _9.1;
_10.1.3.1 = _10.1.0;
_10.1.3 = (_2, _10.1.0, 20511_u16);
_9 = (6302974099552479883_u64, _10.0);
_11.0 = core::ptr::addr_of!(_10.1.2);
_2 = _10.1.3.0;
place!(Field::<i16>(Variant(_7, 3), 0)) = !_11.1.0;
_9.0 = !349603748598582751_u64;
_9.1 = _10.0;
RET = 234_u8 as f64;
RET = _10.1.0 as f64;
_3 = [162_u8];
_10.1.3.2 = 11801_u16 >> _1;
_1 = RET as i32;
Goto(bb3)
}
bb3 = {
_9.1 = _10.0;
_5 = _10.1.0 as f32;
_2 = !_4;
_14 = [_10.1.3.1];
_10.2 = core::ptr::addr_of_mut!(_5);
_9.0 = !18318530245770464772_u64;
_10.1.1 = _5 as isize;
_6 = [_10.0,_9.1,_9.1,_9.1,_9.1,_9.1];
_13 = RET;
_12 = !309679541_u32;
_9.1 = _10.0;
_10.1.2 = -42_i8;
_10.0 = _9.1;
place!(Field::<i16>(Variant(_7, 3), 0)) = _11.1.0;
_3 = [102_u8];
_1 = 1878043767_i32;
_1 = true as i32;
_12 = 1527183585_u32;
_5 = _11.1.0 as f32;
_17.2 = [false,false,true,false,false,true,true];
_16 = _11.1.0;
_11.0 = core::ptr::addr_of!(_10.1.2);
_17.0.0.1 = !true;
match Field::<i16>(Variant(_7, 3), 0) {
0 => bb4,
1 => bb5,
2 => bb6,
18479 => bb8,
_ => bb7
}
}
bb4 = {
_9.1 = '\u{d762b}';
_10.2 = core::ptr::addr_of_mut!(_5);
SetDiscriminant(_7, 3);
_10.1.0 = 133976459424307991995472089116886086167_i128 ^ 33662414849434818250526456315389775229_i128;
_9.0 = !454875106123689018_u64;
_2 = -_4;
RET = 85_i8 as f64;
_11.1.0 = 18479_i16;
_10.0 = _9.1;
_10.1.3.1 = _10.1.0;
_10.1.3 = (_2, _10.1.0, 20511_u16);
_9 = (6302974099552479883_u64, _10.0);
_11.0 = core::ptr::addr_of!(_10.1.2);
_2 = _10.1.3.0;
place!(Field::<i16>(Variant(_7, 3), 0)) = !_11.1.0;
_9.0 = !349603748598582751_u64;
_9.1 = _10.0;
RET = 234_u8 as f64;
RET = _10.1.0 as f64;
_3 = [162_u8];
_10.1.3.2 = 11801_u16 >> _1;
_1 = RET as i32;
Goto(bb3)
}
bb5 = {
_1 = 619845020_i32 - 341279155_i32;
_4 = 13_i8 as isize;
_6 = ['\u{b45a7}','\u{a4e4d}','\u{465d1}','\u{bd0d7}','\u{8893e}','\u{2b1e9}'];
_5 = (-30987322910753382834117563222048514719_i128) as f32;
_3 = [130_u8];
_7 = Adt30::Variant3 { fld0: (-14009_i16),fld1: RET };
place!(Field::<i16>(Variant(_7, 3), 0)) = (-7492_i16);
_7 = Adt30::Variant2 { fld0: 9738536661518153875_usize };
_7 = Adt30::Variant1 { fld0: (-104_i8) };
_2 = !_4;
_6 = ['\u{102cf4}','\u{2e845}','\u{f5c41}','\u{c7f76}','\u{53ea6}','\u{ebdc6}'];
_8 = [196_u8,250_u8,21_u8,199_u8,171_u8,212_u8,116_u8,250_u8];
_6 = ['\u{d2e6b}','\u{43e4d}','\u{12ed3}','\u{8a6c4}','\u{4ee9c}','\u{b13a7}'];
_2 = -_4;
_5 = 3153301986462876298_i64 as f32;
_2 = !_4;
_9 = (18337804305938899793_u64, '\u{a453b}');
place!(Field::<i8>(Variant(_7, 1), 0)) = -(-8_i8);
_8 = [71_u8,255_u8,114_u8,217_u8,166_u8,63_u8,49_u8,141_u8];
RET = 159002738044154629313187588440903145089_i128 as f64;
_9.0 = 15079871956370287788_u64 ^ 16222681232463143340_u64;
_7 = Adt30::Variant3 { fld0: 12979_i16,fld1: RET };
_8 = [169_u8,238_u8,130_u8,254_u8,65_u8,183_u8,10_u8,153_u8];
place!(Field::<i16>(Variant(_7, 3), 0)) = 14644045818979261942_usize as i16;
_2 = _4;
_2 = _4 >> _9.0;
_2 = 153478160697391672437469792351384376032_i128 as isize;
_6 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
Call(_9 = fn1(_2, Field::<f64>(Variant(_7, 3), 1), _8, _6, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_17.0.1.1 = _1;
_11.1 = (Field::<i16>(Variant(_7, 3), 0),);
_15 = _10.1.2 as isize;
_17.3.1 = _9.0;
_9.1 = _10.0;
RET = _10.1.2 as f64;
_17.0.0.0 = 641820965129485618_i64;
_10.1.3.2 = 21145_u16 * 59175_u16;
_17.0.0.3 = -_17.0.0.0;
_17.0.0.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_17.3.1 = !_9.0;
place!(Field::<f64>(Variant(_7, 3), 1)) = _13 * _13;
_10.1.2 = -(-51_i8);
_10.1.2 = 61_i8 & 53_i8;
_10.1.1 = _10.1.3.0;
_9 = (_17.3.1, _10.0);
Goto(bb9)
}
bb9 = {
_17.0.1.2 = _10.0 as u128;
_10.1.3.1 = !_10.1.0;
_21 = _2 << _10.1.3.2;
RET = Field::<f64>(Variant(_7, 3), 1) + Field::<f64>(Variant(_7, 3), 1);
_7 = Adt30::Variant3 { fld0: _11.1.0,fld1: _13 };
_9.1 = _10.0;
_17.0.1.2 = 217353231722912508971620317795919567055_u128 << _10.1.1;
_10.1.2 = -(-101_i8);
_17.3.0 = [_17.0.1.1,_1,_17.0.1.1,_1,_1,_1,_1];
match _17.0.0.0 {
0 => bb1,
1 => bb6,
2 => bb8,
3 => bb4,
641820965129485618 => bb11,
_ => bb10
}
}
bb10 = {
_9.1 = '\u{d762b}';
_10.2 = core::ptr::addr_of_mut!(_5);
SetDiscriminant(_7, 3);
_10.1.0 = 133976459424307991995472089116886086167_i128 ^ 33662414849434818250526456315389775229_i128;
_9.0 = !454875106123689018_u64;
_2 = -_4;
RET = 85_i8 as f64;
_11.1.0 = 18479_i16;
_10.0 = _9.1;
_10.1.3.1 = _10.1.0;
_10.1.3 = (_2, _10.1.0, 20511_u16);
_9 = (6302974099552479883_u64, _10.0);
_11.0 = core::ptr::addr_of!(_10.1.2);
_2 = _10.1.3.0;
place!(Field::<i16>(Variant(_7, 3), 0)) = !_11.1.0;
_9.0 = !349603748598582751_u64;
_9.1 = _10.0;
RET = 234_u8 as f64;
RET = _10.1.0 as f64;
_3 = [162_u8];
_10.1.3.2 = 11801_u16 >> _1;
_1 = RET as i32;
Goto(bb3)
}
bb11 = {
_19 = _17.3.1 + _9.0;
_10.1.2 = !(-95_i8);
_10.1.1 = !_2;
RET = Field::<f64>(Variant(_7, 3), 1) - Field::<f64>(Variant(_7, 3), 1);
_10.1.2 = (-27_i8) * 93_i8;
_17.0.0.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_17.0.1.2 = 77866210993233016040688619148356590987_u128 & 293889155685508711955033189981166558284_u128;
_14 = [_10.1.3.1];
_17.1 = [_16,_11.1.0,_11.1.0,_16,_16,_11.1.0,Field::<i16>(Variant(_7, 3), 0)];
_17.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_25 = _17.3.0;
_17.2 = _17.0.0.2;
_17.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_20 = _10.1.2;
Goto(bb12)
}
bb12 = {
_4 = _21;
_6 = [_9.1,_10.0,_10.0,_10.0,_9.1,_9.1];
match _17.0.0.0 {
0 => bb10,
641820965129485618 => bb14,
_ => bb13
}
}
bb13 = {
_9.1 = '\u{d762b}';
_10.2 = core::ptr::addr_of_mut!(_5);
SetDiscriminant(_7, 3);
_10.1.0 = 133976459424307991995472089116886086167_i128 ^ 33662414849434818250526456315389775229_i128;
_9.0 = !454875106123689018_u64;
_2 = -_4;
RET = 85_i8 as f64;
_11.1.0 = 18479_i16;
_10.0 = _9.1;
_10.1.3.1 = _10.1.0;
_10.1.3 = (_2, _10.1.0, 20511_u16);
_9 = (6302974099552479883_u64, _10.0);
_11.0 = core::ptr::addr_of!(_10.1.2);
_2 = _10.1.3.0;
place!(Field::<i16>(Variant(_7, 3), 0)) = !_11.1.0;
_9.0 = !349603748598582751_u64;
_9.1 = _10.0;
RET = 234_u8 as f64;
RET = _10.1.0 as f64;
_3 = [162_u8];
_10.1.3.2 = 11801_u16 >> _1;
_1 = RET as i32;
Goto(bb3)
}
bb14 = {
_22 = _5;
_30 = _14;
_17.0.0.3 = !_17.0.0.0;
_10.2 = core::ptr::addr_of_mut!(_22);
RET = _13;
_17.0.0.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_13 = RET;
place!(Field::<f64>(Variant(_7, 3), 1)) = -_13;
_10.1.3.2 = !51258_u16;
_10.1.3.0 = _4 ^ _15;
_14 = [_10.1.0];
place!(Field::<i16>(Variant(_7, 3), 0)) = _11.1.0 >> _10.1.0;
SetDiscriminant(_7, 3);
_26 = core::ptr::addr_of_mut!(_9.0);
_17.0.0.2 = [_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1,_17.0.0.1];
_10.1.2 = _20 + _20;
_9.0 = _17.0.0.1 as u64;
_11.1.0 = _16;
_2 = _15 & _10.1.1;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(0_usize, 1_usize, Move(_1), 16_usize, Move(_16), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(0_usize, 8_usize, Move(_8), 3_usize, Move(_3), 9_usize, Move(_9), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: f64,mut _3: [u8; 8],mut _4: [char; 6],mut _5: [u8; 8]) -> (u64, char) {
mir! {
type RET = (u64, char);
let _6: (*const i8, (i16,));
let _7: f32;
let _8: i128;
let _9: Adt64;
let _10: (*const i8, (i16,));
let _11: f64;
let _12: [u8; 8];
let _13: bool;
let _14: (isize,);
let _15: usize;
let _16: char;
let _17: (i64, bool, [bool; 7], i64);
let _18: char;
let _19: f32;
let _20: bool;
let _21: *mut ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _22: Adt50;
let _23: (u64, char);
let _24: i16;
let _25: i16;
let _26: *const i8;
let _27: ();
let _28: ();
{
_6.1 = (10336_i16,);
_4 = ['\u{10a2ec}','\u{fe882}','\u{2eec9}','\u{d7a11}','\u{106578}','\u{eb827}'];
_6.1.0 = (-10391_i16);
RET.0 = 14042780709802163372_u64;
RET.0 = !13628524687904038816_u64;
RET.0 = 1196651905777851147_u64 << _1;
_2 = (-38445882010878652292722530720414656958_i128) as f64;
_7 = _2 as f32;
_2 = 6977_u16 as f64;
RET = (8442384307947035734_u64, '\u{b61cb}');
_8 = 17109987806932416754862438935476484972_i128 << _6.1.0;
_1 = 9223372036854775807_isize;
_3 = [91_u8,123_u8,157_u8,86_u8,169_u8,195_u8,77_u8,43_u8];
RET.1 = '\u{21187}';
_6.1.0 = 20276_i16;
_7 = _8 as f32;
_10.1 = (_6.1.0,);
_6.1.0 = -_10.1.0;
_11 = _2 + _2;
_12 = [39_u8,9_u8,37_u8,215_u8,218_u8,203_u8,126_u8,104_u8];
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
Call(_10.1.0 = fn2(_11, RET.0, _6.1.0, RET.0, _1, _3, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [51_u8,213_u8,221_u8,50_u8,67_u8,73_u8,7_u8,144_u8];
RET.0 = (-25_i8) as u64;
RET.0 = 17328374876920180478_u64 << _6.1.0;
RET.0 = !18311018446705207628_u64;
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_13 = !true;
_11 = _2;
_14.0 = _1;
RET.1 = '\u{db923}';
_5 = _12;
RET = (10385639489484217535_u64, '\u{28381}');
_6.1.0 = _10.1.0 << _1;
_3 = _12;
Goto(bb2)
}
bb2 = {
RET.1 = '\u{ee05f}';
_6.1 = (_10.1.0,);
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_6.1.0 = _10.1.0 ^ _10.1.0;
_12 = [237_u8,122_u8,129_u8,78_u8,22_u8,114_u8,136_u8,188_u8];
RET.0 = 8462712982213675323_u64;
RET.1 = '\u{83db7}';
RET = (13638159345491776789_u64, '\u{daeac}');
RET = (9179712410705325672_u64, '\u{e2de1}');
_12 = [45_u8,142_u8,101_u8,171_u8,103_u8,57_u8,181_u8,198_u8];
_14.0 = _1;
_10.1 = _6.1;
_6.1.0 = _10.1.0;
_8 = (-139678724630272100121145513151938180050_i128) << _10.1.0;
_6.1.0 = -_10.1.0;
RET.1 = '\u{c0a1e}';
RET = (6758416046904905700_u64, '\u{25986}');
_15 = 9805651848912729625_usize;
_8 = 160490064698366050440988316550278392955_i128 | (-46712858022085566496238314792510197862_i128);
match _1 {
0 => bb3,
1 => bb4,
2 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb3 = {
_3 = [51_u8,213_u8,221_u8,50_u8,67_u8,73_u8,7_u8,144_u8];
RET.0 = (-25_i8) as u64;
RET.0 = 17328374876920180478_u64 << _6.1.0;
RET.0 = !18311018446705207628_u64;
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_13 = !true;
_11 = _2;
_14.0 = _1;
RET.1 = '\u{db923}';
_5 = _12;
RET = (10385639489484217535_u64, '\u{28381}');
_6.1.0 = _10.1.0 << _1;
_3 = _12;
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
_14 = (_1,);
_12 = [79_u8,173_u8,240_u8,255_u8,153_u8,231_u8,222_u8,151_u8];
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_14.0 = _1;
RET.0 = 9616489932214603065_u64 | 9986365841829683543_u64;
RET = (6034521810053186701_u64, '\u{fd9ad}');
RET.0 = !10386048635295537864_u64;
RET.0 = 5508909764248171198_u64;
RET.1 = '\u{1003d5}';
RET = (2214608983592938402_u64, '\u{569ec}');
_8 = 933208991_i32 as i128;
_15 = 13256603442141159838_usize;
_17.2 = [_13,_13,_13,_13,_13,_13,_13];
_6.1 = (_10.1.0,);
_17.3 = -(-4307597987499608648_i64);
_18 = RET.1;
_14.0 = _1 >> _10.1.0;
_20 = _13 | _13;
_16 = _18;
_6.1.0 = _10.1.0;
_16 = RET.1;
_17.0 = !_17.3;
_3 = [218_u8,212_u8,67_u8,106_u8,103_u8,2_u8,240_u8,6_u8];
_12 = _5;
_12 = [41_u8,242_u8,13_u8,52_u8,212_u8,35_u8,137_u8,224_u8];
_4 = [RET.1,RET.1,_18,_16,RET.1,_16];
match RET.0 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
2214608983592938402 => bb12,
_ => bb11
}
}
bb8 = {
RET.1 = '\u{ee05f}';
_6.1 = (_10.1.0,);
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_6.1.0 = _10.1.0 ^ _10.1.0;
_12 = [237_u8,122_u8,129_u8,78_u8,22_u8,114_u8,136_u8,188_u8];
RET.0 = 8462712982213675323_u64;
RET.1 = '\u{83db7}';
RET = (13638159345491776789_u64, '\u{daeac}');
RET = (9179712410705325672_u64, '\u{e2de1}');
_12 = [45_u8,142_u8,101_u8,171_u8,103_u8,57_u8,181_u8,198_u8];
_14.0 = _1;
_10.1 = _6.1;
_6.1.0 = _10.1.0;
_8 = (-139678724630272100121145513151938180050_i128) << _10.1.0;
_6.1.0 = -_10.1.0;
RET.1 = '\u{c0a1e}';
RET = (6758416046904905700_u64, '\u{25986}');
_15 = 9805651848912729625_usize;
_8 = 160490064698366050440988316550278392955_i128 | (-46712858022085566496238314792510197862_i128);
match _1 {
0 => bb3,
1 => bb4,
2 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb9 = {
_3 = [51_u8,213_u8,221_u8,50_u8,67_u8,73_u8,7_u8,144_u8];
RET.0 = (-25_i8) as u64;
RET.0 = 17328374876920180478_u64 << _6.1.0;
RET.0 = !18311018446705207628_u64;
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_13 = !true;
_11 = _2;
_14.0 = _1;
RET.1 = '\u{db923}';
_5 = _12;
RET = (10385639489484217535_u64, '\u{28381}');
_6.1.0 = _10.1.0 << _1;
_3 = _12;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_3 = [51_u8,213_u8,221_u8,50_u8,67_u8,73_u8,7_u8,144_u8];
RET.0 = (-25_i8) as u64;
RET.0 = 17328374876920180478_u64 << _6.1.0;
RET.0 = !18311018446705207628_u64;
_4 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_13 = !true;
_11 = _2;
_14.0 = _1;
RET.1 = '\u{db923}';
_5 = _12;
RET = (10385639489484217535_u64, '\u{28381}');
_6.1.0 = _10.1.0 << _1;
_3 = _12;
Goto(bb2)
}
bb12 = {
_17.1 = _17.0 == _17.0;
Goto(bb13)
}
bb13 = {
_22.fld4.1 = (-111_i8) as i128;
_10.1 = _6.1;
_22.fld1 = (_17.0, _13, _17.2, _17.3);
_17.3 = _16 as i64;
_6.1 = (_10.1.0,);
_16 = _18;
_17.2 = [_20,_17.1,_20,_13,_17.1,_20,_13];
_22.fld1.3 = _22.fld4.1 as i64;
_23.0 = _15 as u64;
_22.fld1.3 = _17.0 * _17.3;
_14.0 = _1;
_15 = 4442987270975128255_usize * 15468304126178590982_usize;
_22.fld2.0 = 62334268672547634714399294470243591081_u128 as i16;
_2 = _11;
_22.fld1.3 = !_17.3;
Goto(bb14)
}
bb14 = {
RET.1 = _16;
_17.1 = !_20;
_2 = -_11;
_19 = -_7;
_22.fld4.0 = -_1;
RET.1 = _18;
_22.fld4.1 = _8;
_22.fld0.2 = [_18,_16,_16,_18,RET.1,_16];
_4 = [_16,_18,RET.1,_18,_16,_16];
_4 = _22.fld0.2;
_22.fld4 = (_14.0, _8, 26230_u16);
_23.0 = !RET.0;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(1_usize, 3_usize, Move(_3), 1_usize, Move(_1), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(1_usize, 17_usize, Move(_17), 16_usize, Move(_16), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f64,mut _2: u64,mut _3: i16,mut _4: u64,mut _5: isize,mut _6: [u8; 8],mut _7: [u8; 8],mut _8: [u8; 8],mut _9: [u8; 8]) -> i16 {
mir! {
type RET = i16;
let _10: Adt66;
let _11: f64;
let _12: [i128; 8];
let _13: bool;
let _14: u32;
let _15: f32;
let _16: bool;
let _17: bool;
let _18: f32;
let _19: isize;
let _20: i64;
let _21: u64;
let _22: &'static (u64, char);
let _23: Adt40;
let _24: isize;
let _25: f32;
let _26: bool;
let _27: (*mut Adt27, i32, u128, &'static usize);
let _28: *mut ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _29: u8;
let _30: char;
let _31: u8;
let _32: *const [u32; 6];
let _33: f32;
let _34: Adt40;
let _35: isize;
let _36: Adt77;
let _37: (*mut (i16,),);
let _38: (isize,);
let _39: (*mut Adt27,);
let _40: f64;
let _41: usize;
let _42: ();
let _43: ();
{
_7 = [96_u8,243_u8,254_u8,76_u8,28_u8,219_u8,227_u8,167_u8];
RET = !_3;
RET = -_3;
_3 = RET ^ RET;
_1 = 153628050040496150216436817710641270351_i128 as f64;
RET = !_3;
_9 = [33_u8,185_u8,152_u8,194_u8,140_u8,184_u8,162_u8,252_u8];
_2 = !_4;
RET = 56274_u16 as i16;
_4 = !_2;
_5 = 9223372036854775807_isize + (-9223372036854775808_isize);
_6 = _9;
RET = _3;
_4 = '\u{5669a}' as u64;
_3 = RET;
_6 = [101_u8,61_u8,143_u8,101_u8,188_u8,147_u8,243_u8,111_u8];
_4 = _2 - _2;
Call(_7 = fn3(_4, _8, _6, _9, _9, _3, _8, _5, _8, _3, _6, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [91_u8,4_u8,224_u8,209_u8,163_u8,71_u8,85_u8,221_u8];
Call(_3 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = [250_u8,199_u8,93_u8,214_u8,46_u8,19_u8,28_u8,189_u8];
_9 = [33_u8,132_u8,34_u8,172_u8,122_u8,42_u8,121_u8,63_u8];
_9 = [47_u8,2_u8,79_u8,51_u8,180_u8,154_u8,51_u8,86_u8];
_9 = [34_u8,234_u8,157_u8,13_u8,199_u8,123_u8,101_u8,166_u8];
RET = 4003066412_u32 as i16;
_9 = [125_u8,15_u8,22_u8,178_u8,93_u8,38_u8,254_u8,205_u8];
_11 = -_1;
_8 = [179_u8,3_u8,240_u8,122_u8,227_u8,227_u8,40_u8,251_u8];
RET = !_3;
_8 = [243_u8,176_u8,180_u8,73_u8,239_u8,5_u8,91_u8,130_u8];
_3 = !RET;
_13 = false;
_5 = 1111759727_i32 as isize;
_7 = [65_u8,24_u8,237_u8,118_u8,214_u8,85_u8,157_u8,185_u8];
_9 = [154_u8,236_u8,228_u8,200_u8,61_u8,66_u8,247_u8,87_u8];
_1 = _11 + _11;
Goto(bb3)
}
bb3 = {
_12 = [67951032543737614684744522952469738114_i128,(-135707504016259527049083099857697401363_i128),(-99414471983730775793057257020161663811_i128),49348951627582761953324214355583766852_i128,12222510198633501790571306688521234396_i128,140384400577440063331163489195685834461_i128,(-122779960849608188196060407811829262795_i128),458715914153915135575829493607584924_i128];
_3 = 51_u8 as i16;
_19 = _1 as isize;
_15 = 338140979670347754486928117807133458204_u128 as f32;
_4 = '\u{e7d3a}' as u64;
_3 = _4 as i16;
_4 = _2;
RET = _3;
_1 = _11 - _11;
_9 = [50_u8,134_u8,236_u8,31_u8,204_u8,184_u8,109_u8,52_u8];
_14 = 18929_u16 as u32;
_17 = _1 >= _1;
_8 = [203_u8,162_u8,148_u8,208_u8,119_u8,227_u8,216_u8,216_u8];
_13 = !_17;
_19 = _5;
_16 = _15 <= _15;
_21 = _4;
_15 = 20289476683003169477692079003903682025_i128 as f32;
_3 = !RET;
Call(RET = core::intrinsics::bswap(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_20 = 3405978927150074316_i64;
_1 = _11;
_11 = _1;
_19 = _5 * _5;
_20 = 4036174720279694689_i64 + (-5031990915361677036_i64);
_11 = (-55_i8) as f64;
_8 = _7;
Goto(bb5)
}
bb5 = {
RET = 29_i8 as i16;
_7 = _9;
Goto(bb6)
}
bb6 = {
_5 = _2 as isize;
_11 = _1 + _1;
_11 = 138366210602866892655579814762857650160_u128 as f64;
_3 = RET + RET;
_14 = _3 as u32;
_23.fld0 = _17;
_1 = (-117_i8) as f64;
_23.fld1 = _5;
_4 = _21;
_24 = _19;
_23.fld1 = _11 as isize;
_5 = _19;
_26 = _23.fld0 < _17;
_11 = _1;
RET = _3;
_18 = _15;
_1 = -_11;
_18 = -_15;
_16 = _26;
_16 = _13;
_8 = [153_u8,240_u8,8_u8,94_u8,62_u8,107_u8,17_u8,61_u8];
_7 = [222_u8,116_u8,48_u8,4_u8,228_u8,245_u8,151_u8,108_u8];
_8 = _6;
_5 = _23.fld1 - _19;
_13 = _5 >= _5;
Call(_6 = fn16(_7, _9, RET, _12, _8, _8, _24, _26, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = 3552068730152608035_usize as u32;
_17 = _23.fld0 & _23.fld0;
_25 = (-633454628_i32) as f32;
Goto(bb8)
}
bb8 = {
_1 = _21 as f64;
_2 = _4;
_14 = 1010362830_u32 * 742880291_u32;
RET = !_3;
RET = _3 - _3;
_24 = -_5;
_18 = -_25;
Goto(bb9)
}
bb9 = {
_18 = _15;
_29 = 181_u8;
_9 = [_29,_29,_29,_29,_29,_29,_29,_29];
_29 = 131_u8 ^ 53_u8;
_7 = _8;
_3 = (-83429524059902787062068779685463183867_i128) as i16;
_15 = _25;
_8 = [_29,_29,_29,_29,_29,_29,_29,_29];
RET = _3 << _4;
_12 = [104013739447648589934193417076329236756_i128,55427042078086521740817412004748380054_i128,29625139741662780764545152155600900529_i128,49327692117075657452967970682607561593_i128,(-39380360593989932191276242592966634197_i128),(-44266423476170358027524450785869941905_i128),(-75722495654179539735417114391234939532_i128),(-99001697410471267722414882161851928982_i128)];
_26 = _17 <= _16;
_27.2 = 62915847788057288225802626207112282276_u128;
_7 = [_29,_29,_29,_29,_29,_29,_29,_29];
_23.fld1 = 7313_u16 as isize;
_23.fld0 = !_26;
_23 = Adt40 { fld0: _16,fld1: _5 };
_7 = [_29,_29,_29,_29,_29,_29,_29,_29];
_21 = !_4;
_25 = 47380_u16 as f32;
_31 = !_29;
_24 = -_19;
_1 = _5 as f64;
_9 = [_29,_29,_29,_29,_29,_31,_29,_29];
_30 = '\u{990d2}';
_17 = !_13;
match _27.2 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
62915847788057288225802626207112282276 => bb10,
_ => bb7
}
}
bb10 = {
_29 = _31;
_21 = _18 as u64;
_14 = !2681909545_u32;
_23.fld1 = _5 >> _31;
_13 = _19 <= _23.fld1;
_1 = -_11;
RET = !_3;
_19 = -_24;
_14 = 3267767678_u32;
_23.fld1 = _5;
_15 = _25;
_4 = _2 >> _29;
_36.fld0 = [228879391_i32,875186656_i32];
_17 = _16;
_24 = _5 - _19;
_21 = _4;
_15 = _29 as f32;
_26 = _16;
_14 = 898094432_u32 - 1721666195_u32;
_18 = (-86_i8) as f32;
_38.0 = _24 | _23.fld1;
Goto(bb11)
}
bb11 = {
_19 = -_24;
_35 = _20 as isize;
_13 = !_26;
RET = !_3;
_34 = Adt40 { fld0: _17,fld1: _35 };
_23 = Adt40 { fld0: _16,fld1: _38.0 };
_8 = _6;
_27.1 = 1726722144_i32;
_38.0 = !_34.fld1;
_16 = _13 & _26;
match _27.2 {
0 => bb1,
1 => bb5,
2 => bb12,
62915847788057288225802626207112282276 => bb14,
_ => bb13
}
}
bb12 = {
_14 = 3552068730152608035_usize as u32;
_17 = _23.fld0 & _23.fld0;
_25 = (-633454628_i32) as f32;
Goto(bb8)
}
bb13 = {
_18 = _15;
_29 = 181_u8;
_9 = [_29,_29,_29,_29,_29,_29,_29,_29];
_29 = 131_u8 ^ 53_u8;
_7 = _8;
_3 = (-83429524059902787062068779685463183867_i128) as i16;
_15 = _25;
_8 = [_29,_29,_29,_29,_29,_29,_29,_29];
RET = _3 << _4;
_12 = [104013739447648589934193417076329236756_i128,55427042078086521740817412004748380054_i128,29625139741662780764545152155600900529_i128,49327692117075657452967970682607561593_i128,(-39380360593989932191276242592966634197_i128),(-44266423476170358027524450785869941905_i128),(-75722495654179539735417114391234939532_i128),(-99001697410471267722414882161851928982_i128)];
_26 = _17 <= _16;
_27.2 = 62915847788057288225802626207112282276_u128;
_7 = [_29,_29,_29,_29,_29,_29,_29,_29];
_23.fld1 = 7313_u16 as isize;
_23.fld0 = !_26;
_23 = Adt40 { fld0: _16,fld1: _5 };
_7 = [_29,_29,_29,_29,_29,_29,_29,_29];
_21 = !_4;
_25 = 47380_u16 as f32;
_31 = !_29;
_24 = -_19;
_1 = _5 as f64;
_9 = [_29,_29,_29,_29,_29,_31,_29,_29];
_30 = '\u{990d2}';
_17 = !_13;
match _27.2 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
62915847788057288225802626207112282276 => bb10,
_ => bb7
}
}
bb14 = {
_40 = -_11;
_29 = 3_usize as u8;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(2_usize, 29_usize, Move(_29), 5_usize, Move(_5), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(2_usize, 17_usize, Move(_17), 30_usize, Move(_30), 21_usize, Move(_21), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(2_usize, 38_usize, Move(_38), 12_usize, Move(_12), 19_usize, Move(_19), 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: [u8; 8],mut _3: [u8; 8],mut _4: [u8; 8],mut _5: [u8; 8],mut _6: i16,mut _7: [u8; 8],mut _8: isize,mut _9: [u8; 8],mut _10: i16,mut _11: [u8; 8],mut _12: isize) -> [u8; 8] {
mir! {
type RET = [u8; 8];
let _13: bool;
let _14: (*mut Adt27,);
let _15: bool;
let _16: bool;
let _17: char;
let _18: u64;
let _19: u32;
let _20: bool;
let _21: i128;
let _22: &'static (*mut Adt27,);
let _23: bool;
let _24: [i16; 7];
let _25: Adt66;
let _26: f32;
let _27: f32;
let _28: usize;
let _29: ([i128; 1], &'static (i64, bool, [bool; 7], i64), [bool; 2], (*mut Adt27, i32, u128, &'static usize));
let _30: [u8; 8];
let _31: (*mut (i16,),);
let _32: i8;
let _33: &'static *const [u32; 6];
let _34: f64;
let _35: f32;
let _36: &'static (u64, char);
let _37: i64;
let _38: f64;
let _39: [u32; 6];
let _40: char;
let _41: isize;
let _42: i32;
let _43: isize;
let _44: (isize,);
let _45: bool;
let _46: Adt50;
let _47: ();
let _48: ();
{
RET = [217_u8,38_u8,0_u8,203_u8,184_u8,52_u8,77_u8,53_u8];
_2 = [139_u8,174_u8,255_u8,68_u8,76_u8,133_u8,185_u8,80_u8];
_2 = [160_u8,119_u8,92_u8,221_u8,114_u8,164_u8,164_u8,218_u8];
_8 = _12 * _12;
_5 = [254_u8,57_u8,176_u8,82_u8,149_u8,59_u8,218_u8,188_u8];
_5 = _11;
_6 = _10;
_6 = _10;
_12 = false as isize;
_11 = [51_u8,69_u8,171_u8,137_u8,251_u8,47_u8,228_u8,1_u8];
_9 = _2;
RET = [224_u8,185_u8,52_u8,142_u8,19_u8,178_u8,168_u8,169_u8];
_1 = 45401_u16 as u64;
Goto(bb1)
}
bb1 = {
_6 = _1 as i16;
_10 = _6 - _6;
_1 = 15039378624065811291_u64 | 9866712989130912558_u64;
_13 = _8 < _8;
RET = _5;
_10 = -_6;
_16 = _10 <= _6;
RET = _7;
_3 = [57_u8,213_u8,94_u8,90_u8,96_u8,237_u8,237_u8,37_u8];
RET = _9;
_16 = _13;
_1 = 1494994377839546033_u64 * 13370024954502666284_u64;
_15 = _13;
_15 = !_16;
_12 = _8;
_6 = (-8262026797205944266_i64) as i16;
_3 = [147_u8,107_u8,177_u8,63_u8,131_u8,191_u8,202_u8,66_u8];
_5 = [74_u8,181_u8,212_u8,1_u8,65_u8,229_u8,6_u8,95_u8];
_2 = [235_u8,192_u8,27_u8,0_u8,201_u8,65_u8,113_u8,163_u8];
_1 = 13489895871563955000_u64 & 1041540201059202148_u64;
_10 = 9200064116083709969_usize as i16;
_5 = _9;
Call(_15 = fn4(_2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = _1 * _1;
_1 = _18 & _18;
_17 = '\u{a7fd9}';
_12 = _17 as isize;
_10 = !_6;
_16 = _1 == _18;
_4 = [210_u8,15_u8,55_u8,111_u8,188_u8,108_u8,83_u8,174_u8];
_3 = _2;
_6 = _10 >> _8;
_10 = _6;
_10 = _6;
_20 = !_13;
_11 = [29_u8,245_u8,174_u8,0_u8,145_u8,229_u8,30_u8,160_u8];
_13 = _16;
_4 = [46_u8,73_u8,108_u8,73_u8,119_u8,189_u8,47_u8,30_u8];
_12 = -_8;
_9 = [222_u8,194_u8,82_u8,242_u8,123_u8,245_u8,180_u8,30_u8];
Call(_17 = fn5(_11, _3, _2, _1, _6, _9, _1, _1, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _11;
_17 = '\u{65572}';
_1 = _18 >> _18;
_16 = !_20;
_21 = _8 as i128;
_22 = &_14;
_11 = [30_u8,167_u8,29_u8,47_u8,230_u8,187_u8,154_u8,114_u8];
_21 = 142014129453520964552048638622851292399_i128;
_15 = _13;
_17 = '\u{f620f}';
_9 = _11;
_23 = !_15;
_17 = '\u{1646}';
_8 = !_12;
_4 = [37_u8,89_u8,82_u8,53_u8,4_u8,182_u8,230_u8,142_u8];
_12 = _8 - _8;
_22 = &(*_22);
_23 = _20;
_6 = !_10;
_6 = 1123077189_u32 as i16;
match _21 {
0 => bb4,
1 => bb5,
142014129453520964552048638622851292399 => bb7,
_ => bb6
}
}
bb4 = {
_18 = _1 * _1;
_1 = _18 & _18;
_17 = '\u{a7fd9}';
_12 = _17 as isize;
_10 = !_6;
_16 = _1 == _18;
_4 = [210_u8,15_u8,55_u8,111_u8,188_u8,108_u8,83_u8,174_u8];
_3 = _2;
_6 = _10 >> _8;
_10 = _6;
_10 = _6;
_20 = !_13;
_11 = [29_u8,245_u8,174_u8,0_u8,145_u8,229_u8,30_u8,160_u8];
_13 = _16;
_4 = [46_u8,73_u8,108_u8,73_u8,119_u8,189_u8,47_u8,30_u8];
_12 = -_8;
_9 = [222_u8,194_u8,82_u8,242_u8,123_u8,245_u8,180_u8,30_u8];
Call(_17 = fn5(_11, _3, _2, _1, _6, _9, _1, _1, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_6 = _1 as i16;
_10 = _6 - _6;
_1 = 15039378624065811291_u64 | 9866712989130912558_u64;
_13 = _8 < _8;
RET = _5;
_10 = -_6;
_16 = _10 <= _6;
RET = _7;
_3 = [57_u8,213_u8,94_u8,90_u8,96_u8,237_u8,237_u8,37_u8];
RET = _9;
_16 = _13;
_1 = 1494994377839546033_u64 * 13370024954502666284_u64;
_15 = _13;
_15 = !_16;
_12 = _8;
_6 = (-8262026797205944266_i64) as i16;
_3 = [147_u8,107_u8,177_u8,63_u8,131_u8,191_u8,202_u8,66_u8];
_5 = [74_u8,181_u8,212_u8,1_u8,65_u8,229_u8,6_u8,95_u8];
_2 = [235_u8,192_u8,27_u8,0_u8,201_u8,65_u8,113_u8,163_u8];
_1 = 13489895871563955000_u64 & 1041540201059202148_u64;
_10 = 9200064116083709969_usize as i16;
_5 = _9;
Call(_15 = fn4(_2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_8 = _12;
_2 = [29_u8,54_u8,125_u8,234_u8,215_u8,166_u8,137_u8,172_u8];
_22 = &_14;
_18 = _1 - _1;
_3 = _11;
_17 = '\u{54c1b}';
_11 = [177_u8,202_u8,192_u8,81_u8,13_u8,204_u8,146_u8,142_u8];
_6 = _10;
_23 = _20 & _15;
_11 = _4;
_1 = !_18;
_19 = 3308444599_u32 << _18;
_20 = _23 != _15;
_24 = [_10,_6,_6,_6,_6,_10,_6];
_23 = _1 != _18;
_26 = _21 as f32;
_1 = 113_i8 as u64;
_2 = [117_u8,193_u8,34_u8,175_u8,135_u8,144_u8,133_u8,204_u8];
_4 = _5;
RET = [46_u8,148_u8,132_u8,144_u8,48_u8,126_u8,175_u8,239_u8];
_7 = [101_u8,71_u8,224_u8,226_u8,253_u8,9_u8,164_u8,25_u8];
_3 = _2;
_22 = &(*_22);
_9 = [191_u8,165_u8,59_u8,191_u8,250_u8,126_u8,118_u8,93_u8];
_13 = _12 <= _8;
match _21 {
0 => bb1,
142014129453520964552048638622851292399 => bb8,
_ => bb2
}
}
bb8 = {
_10 = _19 as i16;
_27 = -_26;
_20 = !_15;
_20 = _23 | _23;
_15 = _23;
_11 = [250_u8,252_u8,144_u8,21_u8,251_u8,103_u8,1_u8,55_u8];
_3 = [55_u8,173_u8,109_u8,32_u8,48_u8,46_u8,74_u8,130_u8];
_28 = 2_usize | 15292431509947044464_usize;
_29.3.2 = !326687863654993209820627425324735315883_u128;
_29.2 = [_23,_23];
_29.3.3 = &_28;
_9 = _3;
_18 = _1;
_2 = [70_u8,201_u8,221_u8,237_u8,106_u8,113_u8,135_u8,98_u8];
_3 = _5;
_21 = _18 as i128;
_19 = !1695399466_u32;
_7 = _5;
_29.0 = [_21];
_32 = (-124_i8) * (-59_i8);
RET = _5;
_20 = !_15;
Goto(bb9)
}
bb9 = {
_26 = _27 * _27;
Goto(bb10)
}
bb10 = {
_30 = [225_u8,236_u8,172_u8,249_u8,31_u8,174_u8,15_u8,235_u8];
_7 = [32_u8,213_u8,237_u8,85_u8,224_u8,118_u8,208_u8,15_u8];
_29.3.3 = &_28;
_24 = [_10,_10,_10,_6,_6,_10,_10];
_16 = _23;
_28 = (-1771272058_i32) as usize;
_38 = _10 as f64;
_34 = -_38;
_10 = _6;
RET = _3;
_7 = _11;
_29.3.1 = (-1342726380_i32) & (-1719278483_i32);
_26 = -_27;
_22 = &(*_22);
_29.2 = [_23,_15];
_32 = -(-125_i8);
_26 = _27;
Goto(bb11)
}
bb11 = {
_1 = _18;
_6 = _10;
_24 = [_6,_10,_6,_10,_10,_10,_6];
_28 = 4_usize;
_27 = _26;
_27 = _26 + _26;
_39[_28] = !_19;
_13 = _20 | _20;
RET = [_2[_28],_9[_28],_2[_28],_11[_28],_3[_28],_4[_28],_3[_28],_11[_28]];
_16 = _13;
_40 = _17;
_37 = (-2205097446239977082_i64) >> _4[_28];
_37 = 5869558976922808989_i64;
_40 = _17;
_22 = &(*_22);
_39 = [_19,_19,_19,_19,_19,_19];
_23 = _20;
RET[_28] = _7[_28];
_6 = _10 << _4[_28];
_12 = _26 as isize;
_7 = [_4[_28],_4[_28],_4[_28],_2[_28],_4[_28],_30[_28],_4[_28],_30[_28]];
_37 = !8984208530013917840_i64;
_32 = -103_i8;
_5[_28] = _19 as u8;
Goto(bb12)
}
bb12 = {
_44.0 = _8;
_24[_28] = _15 as i16;
_27 = -_26;
match _11[_28] {
0 => bb7,
1 => bb6,
2 => bb4,
3 => bb13,
4 => bb14,
251 => bb16,
_ => bb15
}
}
bb13 = {
_1 = _18;
_6 = _10;
_24 = [_6,_10,_6,_10,_10,_10,_6];
_28 = 4_usize;
_27 = _26;
_27 = _26 + _26;
_39[_28] = !_19;
_13 = _20 | _20;
RET = [_2[_28],_9[_28],_2[_28],_11[_28],_3[_28],_4[_28],_3[_28],_11[_28]];
_16 = _13;
_40 = _17;
_37 = (-2205097446239977082_i64) >> _4[_28];
_37 = 5869558976922808989_i64;
_40 = _17;
_22 = &(*_22);
_39 = [_19,_19,_19,_19,_19,_19];
_23 = _20;
RET[_28] = _7[_28];
_6 = _10 << _4[_28];
_12 = _26 as isize;
_7 = [_4[_28],_4[_28],_4[_28],_2[_28],_4[_28],_30[_28],_4[_28],_30[_28]];
_37 = !8984208530013917840_i64;
_32 = -103_i8;
_5[_28] = _19 as u8;
Goto(bb12)
}
bb14 = {
_8 = _12;
_2 = [29_u8,54_u8,125_u8,234_u8,215_u8,166_u8,137_u8,172_u8];
_22 = &_14;
_18 = _1 - _1;
_3 = _11;
_17 = '\u{54c1b}';
_11 = [177_u8,202_u8,192_u8,81_u8,13_u8,204_u8,146_u8,142_u8];
_6 = _10;
_23 = _20 & _15;
_11 = _4;
_1 = !_18;
_19 = 3308444599_u32 << _18;
_20 = _23 != _15;
_24 = [_10,_6,_6,_6,_6,_10,_6];
_23 = _1 != _18;
_26 = _21 as f32;
_1 = 113_i8 as u64;
_2 = [117_u8,193_u8,34_u8,175_u8,135_u8,144_u8,133_u8,204_u8];
_4 = _5;
RET = [46_u8,148_u8,132_u8,144_u8,48_u8,126_u8,175_u8,239_u8];
_7 = [101_u8,71_u8,224_u8,226_u8,253_u8,9_u8,164_u8,25_u8];
_3 = _2;
_22 = &(*_22);
_9 = [191_u8,165_u8,59_u8,191_u8,250_u8,126_u8,118_u8,93_u8];
_13 = _12 <= _8;
match _21 {
0 => bb1,
142014129453520964552048638622851292399 => bb8,
_ => bb2
}
}
bb15 = {
_10 = _19 as i16;
_27 = -_26;
_20 = !_15;
_20 = _23 | _23;
_15 = _23;
_11 = [250_u8,252_u8,144_u8,21_u8,251_u8,103_u8,1_u8,55_u8];
_3 = [55_u8,173_u8,109_u8,32_u8,48_u8,46_u8,74_u8,130_u8];
_28 = 2_usize | 15292431509947044464_usize;
_29.3.2 = !326687863654993209820627425324735315883_u128;
_29.2 = [_23,_23];
_29.3.3 = &_28;
_9 = _3;
_18 = _1;
_2 = [70_u8,201_u8,221_u8,237_u8,106_u8,113_u8,135_u8,98_u8];
_3 = _5;
_21 = _18 as i128;
_19 = !1695399466_u32;
_7 = _5;
_29.0 = [_21];
_32 = (-124_i8) * (-59_i8);
RET = _5;
_20 = !_15;
Goto(bb9)
}
bb16 = {
_9[_28] = _32 as u8;
_44 = (_8,);
_15 = _20;
Goto(bb17)
}
bb17 = {
Call(_47 = dump_var(3_usize, 20_usize, Move(_20), 40_usize, Move(_40), 39_usize, Move(_39), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(3_usize, 19_usize, Move(_19), 7_usize, Move(_7), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(3_usize, 9_usize, Move(_9), 16_usize, Move(_16), 23_usize, Move(_23), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_47 = dump_var(3_usize, 30_usize, Move(_30), 11_usize, Move(_11), 48_usize, _48, 48_usize, _48), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [u8; 8],mut _2: i16) -> bool {
mir! {
type RET = bool;
let _3: i16;
let _4: ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _5: [i128; 1];
let _6: isize;
let _7: char;
let _8: isize;
let _9: char;
let _10: Adt30;
let _11: i16;
let _12: u128;
let _13: [i16; 8];
let _14: *mut Adt27;
let _15: (isize,);
let _16: ();
let _17: ();
{
RET = false | false;
_1 = [239_u8,67_u8,44_u8,66_u8,218_u8,213_u8,0_u8,25_u8];
_3 = (-1192589711323125334_i64) as i16;
_4.0.0 = 7727883870636093335_u64 as isize;
_4.0 = ((-9223372036854775808_isize), (-156567008247016121774800048262002620090_i128), 50482_u16);
_4.0.1 = 135747013456776853004627011695791691021_i128;
Goto(bb1)
}
bb1 = {
_4.0.2 = 18041_u16;
_4.1.2 = ['\u{d0e8e}','\u{6b308}','\u{7e920}','\u{bd043}','\u{b9c24}','\u{8a265}'];
_4.0 = (9223372036854775807_isize, (-144619008992219820409223009194217561726_i128), 60570_u16);
_1 = [232_u8,130_u8,93_u8,158_u8,25_u8,181_u8,55_u8,76_u8];
_4.0.2 = 28674_u16;
_4.0.1 = '\u{b6eb7}' as i128;
RET = true;
RET = false & true;
_3 = -_2;
_1 = [113_u8,73_u8,61_u8,144_u8,13_u8,74_u8,168_u8,185_u8];
match _4.0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
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
RET = !true;
_4.0 = ((-9223372036854775808_isize), 144637451842403942372631629365795966526_i128, 27288_u16);
RET = _4.0.0 > _4.0.0;
_1 = [131_u8,143_u8,249_u8,216_u8,255_u8,99_u8,121_u8,153_u8];
_6 = -_4.0.0;
_4.0.2 = 31205_u16;
_7 = '\u{7dabf}';
_7 = '\u{1075b1}';
_3 = _2 >> _4.0.2;
RET = !false;
RET = false;
RET = _6 > _6;
_4.0.0 = _4.0.1 as isize;
match _4.0.1 {
0 => bb8,
1 => bb9,
2 => bb10,
144637451842403942372631629365795966526 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
_4.0.2 = 18041_u16;
_4.1.2 = ['\u{d0e8e}','\u{6b308}','\u{7e920}','\u{bd043}','\u{b9c24}','\u{8a265}'];
_4.0 = (9223372036854775807_isize, (-144619008992219820409223009194217561726_i128), 60570_u16);
_1 = [232_u8,130_u8,93_u8,158_u8,25_u8,181_u8,55_u8,76_u8];
_4.0.2 = 28674_u16;
_4.0.1 = '\u{b6eb7}' as i128;
RET = true;
RET = false & true;
_3 = -_2;
_1 = [113_u8,73_u8,61_u8,144_u8,13_u8,74_u8,168_u8,185_u8];
match _4.0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_4.0.1 = 88123189177904955558354711843089225929_i128;
_7 = '\u{5aabc}';
_8 = -_6;
_7 = '\u{12147}';
_4.0 = (_8, 127828860181793857009927207617081216826_i128, 17683_u16);
_3 = -_2;
_4.0 = (_6, 41092480498790964235357652463569641465_i128, 31786_u16);
_4.1.2 = [_7,_7,_7,_7,_7,_7];
_4.0.0 = _8 << _4.0.1;
_1 = [123_u8,166_u8,167_u8,114_u8,225_u8,116_u8,123_u8,186_u8];
Goto(bb13)
}
bb13 = {
_8 = _4.0.0;
Goto(bb14)
}
bb14 = {
_4.1.2 = [_7,_7,_7,_7,_7,_7];
_1 = [5_u8,123_u8,73_u8,44_u8,143_u8,67_u8,182_u8,233_u8];
_11 = _3;
_9 = _7;
_3 = _4.0.1 as i16;
_8 = _4.0.0 << _4.0.1;
_1 = [211_u8,138_u8,52_u8,182_u8,33_u8,255_u8,229_u8,71_u8];
_6 = _4.0.0;
_6 = (-1384383389326250607_i64) as isize;
_11 = _3 >> _8;
_11 = -_3;
_8 = 273425922_i32 as isize;
_4.0.1 = 148850723045620220510280202000265027199_i128;
_1 = [135_u8,108_u8,250_u8,1_u8,254_u8,91_u8,229_u8,159_u8];
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(4_usize, 9_usize, Move(_9), 11_usize, Move(_11), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u8; 8],mut _2: [u8; 8],mut _3: [u8; 8],mut _4: u64,mut _5: i16,mut _6: [u8; 8],mut _7: u64,mut _8: u64,mut _9: [u8; 8]) -> char {
mir! {
type RET = char;
let _10: *mut Adt27;
let _11: u16;
let _12: isize;
let _13: char;
let _14: char;
let _15: isize;
let _16: [i16; 6];
let _17: char;
let _18: Adt30;
let _19: i8;
let _20: *mut (i16,);
let _21: &'static (u64, char);
let _22: i16;
let _23: f32;
let _24: Adt40;
let _25: [bool; 7];
let _26: bool;
let _27: usize;
let _28: usize;
let _29: &'static Adt27;
let _30: (u64, char);
let _31: (Adt66,);
let _32: *mut ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _33: isize;
let _34: [char; 6];
let _35: isize;
let _36: ([i32; 7], u64);
let _37: *mut ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _38: f32;
let _39: ();
let _40: ();
{
_8 = _7 >> _7;
_2 = [161_u8,112_u8,72_u8,168_u8,87_u8,8_u8,120_u8,253_u8];
_8 = (-1579768338_i32) as u64;
RET = '\u{6697}';
_6 = [167_u8,132_u8,205_u8,189_u8,16_u8,183_u8,240_u8,207_u8];
_9 = _2;
RET = '\u{77563}';
_6 = [25_u8,185_u8,240_u8,9_u8,169_u8,32_u8,171_u8,175_u8];
_9 = [188_u8,197_u8,253_u8,133_u8,255_u8,80_u8,214_u8,219_u8];
_4 = (-126_i8) as u64;
_6 = [74_u8,82_u8,182_u8,116_u8,102_u8,173_u8,96_u8,26_u8];
Goto(bb1)
}
bb1 = {
_1 = _2;
_2 = _1;
_2 = [52_u8,181_u8,114_u8,1_u8,225_u8,58_u8,107_u8,234_u8];
_6 = [80_u8,68_u8,247_u8,127_u8,36_u8,125_u8,152_u8,225_u8];
_5 = (-3576_i16) & 27815_i16;
_9 = [138_u8,198_u8,127_u8,242_u8,50_u8,212_u8,168_u8,137_u8];
_3 = [142_u8,112_u8,195_u8,175_u8,44_u8,85_u8,70_u8,115_u8];
RET = '\u{24832}';
RET = '\u{f5aed}';
_3 = [219_u8,53_u8,211_u8,59_u8,76_u8,109_u8,64_u8,133_u8];
RET = '\u{f3116}';
_2 = [230_u8,125_u8,117_u8,31_u8,126_u8,154_u8,182_u8,186_u8];
_4 = _7 + _7;
Call(_8 = fn6(_1, _6, _4, _4, _4, _9, _6, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = !52075_u16;
_4 = !_8;
_3 = [24_u8,247_u8,184_u8,100_u8,186_u8,206_u8,39_u8,142_u8];
RET = '\u{5b486}';
_2 = [114_u8,20_u8,100_u8,124_u8,127_u8,181_u8,6_u8,25_u8];
_4 = _8 | _8;
_12 = !(-9223372036854775808_isize);
_2 = _1;
_3 = [190_u8,51_u8,218_u8,184_u8,227_u8,206_u8,169_u8,194_u8];
RET = '\u{f6619}';
_14 = RET;
_5 = !18583_i16;
_12 = !9223372036854775807_isize;
_3 = _9;
_3 = [54_u8,47_u8,163_u8,29_u8,224_u8,111_u8,130_u8,18_u8];
_13 = RET;
Call(_12 = fn7(_4, _1, _8, _8, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = [96_u8,227_u8,227_u8,175_u8,47_u8,34_u8,209_u8,239_u8];
_7 = _4;
_2 = _3;
RET = _14;
RET = _13;
_5 = 10956_i16 >> _4;
_9 = _1;
_16 = [_5,_5,_5,_5,_5,_5];
_2 = [134_u8,240_u8,154_u8,254_u8,179_u8,37_u8,195_u8,137_u8];
_3 = [227_u8,253_u8,141_u8,118_u8,130_u8,70_u8,172_u8,82_u8];
_1 = _3;
_18 = Adt30::Variant1 { fld0: (-42_i8) };
_18 = Adt30::Variant1 { fld0: 25_i8 };
_2 = _9;
_19 = 95_i8;
place!(Field::<i8>(Variant(_18, 1), 0)) = _19;
_9 = [118_u8,194_u8,211_u8,59_u8,182_u8,63_u8,173_u8,130_u8];
RET = _13;
_8 = _4;
_16 = [_5,_5,_5,_5,_5,_5];
_6 = [6_u8,116_u8,82_u8,202_u8,112_u8,95_u8,59_u8,229_u8];
match _19 {
0 => bb1,
1 => bb2,
2 => bb4,
95 => bb6,
_ => bb5
}
}
bb4 = {
_11 = !52075_u16;
_4 = !_8;
_3 = [24_u8,247_u8,184_u8,100_u8,186_u8,206_u8,39_u8,142_u8];
RET = '\u{5b486}';
_2 = [114_u8,20_u8,100_u8,124_u8,127_u8,181_u8,6_u8,25_u8];
_4 = _8 | _8;
_12 = !(-9223372036854775808_isize);
_2 = _1;
_3 = [190_u8,51_u8,218_u8,184_u8,227_u8,206_u8,169_u8,194_u8];
RET = '\u{f6619}';
_14 = RET;
_5 = !18583_i16;
_12 = !9223372036854775807_isize;
_3 = _9;
_3 = [54_u8,47_u8,163_u8,29_u8,224_u8,111_u8,130_u8,18_u8];
_13 = RET;
Call(_12 = fn7(_4, _1, _8, _8, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_1 = _2;
_2 = _1;
_2 = [52_u8,181_u8,114_u8,1_u8,225_u8,58_u8,107_u8,234_u8];
_6 = [80_u8,68_u8,247_u8,127_u8,36_u8,125_u8,152_u8,225_u8];
_5 = (-3576_i16) & 27815_i16;
_9 = [138_u8,198_u8,127_u8,242_u8,50_u8,212_u8,168_u8,137_u8];
_3 = [142_u8,112_u8,195_u8,175_u8,44_u8,85_u8,70_u8,115_u8];
RET = '\u{24832}';
RET = '\u{f5aed}';
_3 = [219_u8,53_u8,211_u8,59_u8,76_u8,109_u8,64_u8,133_u8];
RET = '\u{f3116}';
_2 = [230_u8,125_u8,117_u8,31_u8,126_u8,154_u8,182_u8,186_u8];
_4 = _7 + _7;
Call(_8 = fn6(_1, _6, _4, _4, _4, _9, _6, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_13 = RET;
_15 = -_12;
_4 = !_8;
_1 = [125_u8,153_u8,78_u8,71_u8,241_u8,27_u8,140_u8,153_u8];
_19 = Field::<i8>(Variant(_18, 1), 0) & Field::<i8>(Variant(_18, 1), 0);
_18 = Adt30::Variant1 { fld0: _19 };
_3 = [39_u8,184_u8,101_u8,181_u8,82_u8,201_u8,191_u8,181_u8];
_2 = _3;
RET = _14;
_11 = 16467_u16;
SetDiscriminant(_18, 1);
RET = _13;
_2 = [82_u8,252_u8,141_u8,32_u8,128_u8,182_u8,236_u8,201_u8];
_16 = [_5,_5,_5,_5,_5,_5];
_6 = [28_u8,66_u8,105_u8,25_u8,86_u8,192_u8,249_u8,162_u8];
_11 = _14 as u16;
place!(Field::<i8>(Variant(_18, 1), 0)) = _19;
_25 = [false,false,false,false,false,false,true];
_27 = 3_usize;
_3[_27] = _6[_27];
_26 = _25[_27] & _25[_27];
_17 = _14;
place!(Field::<i8>(Variant(_18, 1), 0)) = !_19;
_3 = _1;
Call(_25[_27] = fn15(_16, _16, _16, _15, _7, _16[_27], _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_24 = Adt40 { fld0: _26,fld1: _15 };
_22 = _5 << _7;
_18 = Adt30::Variant2 { fld0: _27 };
_15 = _24.fld1;
_11 = 47344_u16;
_6 = [27_u8,171_u8,52_u8,118_u8,31_u8,91_u8,49_u8,154_u8];
Goto(bb8)
}
bb8 = {
_24 = Adt40 { fld0: _26,fld1: _12 };
Goto(bb9)
}
bb9 = {
_1 = [39_u8,16_u8,179_u8,42_u8,24_u8,179_u8,28_u8,199_u8];
_12 = _15 * _24.fld1;
_7 = _8 << _4;
_17 = _13;
_5 = !_22;
_15 = -_12;
place!(Field::<usize>(Variant(_18, 2), 0)) = _15 as usize;
_30 = (_8, _14);
_3 = _9;
_30 = (_4, _13);
_30.1 = RET;
_4 = !_8;
match _27 {
0 => bb1,
3 => bb11,
_ => bb10
}
}
bb10 = {
_1 = _2;
_2 = _1;
_2 = [52_u8,181_u8,114_u8,1_u8,225_u8,58_u8,107_u8,234_u8];
_6 = [80_u8,68_u8,247_u8,127_u8,36_u8,125_u8,152_u8,225_u8];
_5 = (-3576_i16) & 27815_i16;
_9 = [138_u8,198_u8,127_u8,242_u8,50_u8,212_u8,168_u8,137_u8];
_3 = [142_u8,112_u8,195_u8,175_u8,44_u8,85_u8,70_u8,115_u8];
RET = '\u{24832}';
RET = '\u{f5aed}';
_3 = [219_u8,53_u8,211_u8,59_u8,76_u8,109_u8,64_u8,133_u8];
RET = '\u{f3116}';
_2 = [230_u8,125_u8,117_u8,31_u8,126_u8,154_u8,182_u8,186_u8];
_4 = _7 + _7;
Call(_8 = fn6(_1, _6, _4, _4, _4, _9, _6, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_12 = _24.fld1;
_27 = Field::<usize>(Variant(_18, 2), 0) ^ Field::<usize>(Variant(_18, 2), 0);
_33 = -_15;
_28 = _24.fld0 as usize;
_26 = !_24.fld0;
match _11 {
0 => bb10,
1 => bb12,
47344 => bb14,
_ => bb13
}
}
bb12 = {
_13 = RET;
_15 = -_12;
_4 = !_8;
_1 = [125_u8,153_u8,78_u8,71_u8,241_u8,27_u8,140_u8,153_u8];
_19 = Field::<i8>(Variant(_18, 1), 0) & Field::<i8>(Variant(_18, 1), 0);
_18 = Adt30::Variant1 { fld0: _19 };
_3 = [39_u8,184_u8,101_u8,181_u8,82_u8,201_u8,191_u8,181_u8];
_2 = _3;
RET = _14;
_11 = 16467_u16;
SetDiscriminant(_18, 1);
RET = _13;
_2 = [82_u8,252_u8,141_u8,32_u8,128_u8,182_u8,236_u8,201_u8];
_16 = [_5,_5,_5,_5,_5,_5];
_6 = [28_u8,66_u8,105_u8,25_u8,86_u8,192_u8,249_u8,162_u8];
_11 = _14 as u16;
place!(Field::<i8>(Variant(_18, 1), 0)) = _19;
_25 = [false,false,false,false,false,false,true];
_27 = 3_usize;
_3[_27] = _6[_27];
_26 = _25[_27] & _25[_27];
_17 = _14;
place!(Field::<i8>(Variant(_18, 1), 0)) = !_19;
_3 = _1;
Call(_25[_27] = fn15(_16, _16, _16, _15, _7, _16[_27], _7), ReturnTo(bb7), UnwindUnreachable())
}
bb13 = {
_11 = !52075_u16;
_4 = !_8;
_3 = [24_u8,247_u8,184_u8,100_u8,186_u8,206_u8,39_u8,142_u8];
RET = '\u{5b486}';
_2 = [114_u8,20_u8,100_u8,124_u8,127_u8,181_u8,6_u8,25_u8];
_4 = _8 | _8;
_12 = !(-9223372036854775808_isize);
_2 = _1;
_3 = [190_u8,51_u8,218_u8,184_u8,227_u8,206_u8,169_u8,194_u8];
RET = '\u{f6619}';
_14 = RET;
_5 = !18583_i16;
_12 = !9223372036854775807_isize;
_3 = _9;
_3 = [54_u8,47_u8,163_u8,29_u8,224_u8,111_u8,130_u8,18_u8];
_13 = RET;
Call(_12 = fn7(_4, _1, _8, _8, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_12 = 3621287396560073223_i64 as isize;
_23 = 1977968022_u32 as f32;
_19 = 23_i8;
_5 = !_22;
_1 = [166_u8,118_u8,170_u8,23_u8,181_u8,0_u8,1_u8,60_u8];
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(5_usize, 17_usize, Move(_17), 7_usize, Move(_7), 3_usize, Move(_3), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(5_usize, 15_usize, Move(_15), 8_usize, Move(_8), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(5_usize, 2_usize, Move(_2), 1_usize, Move(_1), 9_usize, Move(_9), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u8; 8],mut _2: [u8; 8],mut _3: u64,mut _4: u64,mut _5: u64,mut _6: [u8; 8],mut _7: [u8; 8],mut _8: [u8; 8]) -> u64 {
mir! {
type RET = u64;
let _9: bool;
let _10: bool;
let _11: &'static (u64, char);
let _12: (f64, *const f32, (isize, i128, u16), [bool; 7]);
let _13: Adt30;
let _14: ();
let _15: ();
{
_1 = _7;
_9 = !true;
_2 = _6;
_4 = _3 >> _3;
_3 = _4;
_8 = _2;
RET = _3;
RET = _5;
_8 = [217_u8,55_u8,243_u8,170_u8,243_u8,230_u8,178_u8,130_u8];
_2 = _6;
_1 = [12_u8,105_u8,44_u8,228_u8,115_u8,44_u8,73_u8,81_u8];
_1 = [6_u8,139_u8,176_u8,89_u8,93_u8,187_u8,248_u8,66_u8];
RET = !_5;
_6 = [157_u8,50_u8,192_u8,224_u8,8_u8,23_u8,10_u8,75_u8];
_6 = [18_u8,48_u8,198_u8,68_u8,42_u8,237_u8,148_u8,147_u8];
RET = !_3;
_8 = [184_u8,31_u8,195_u8,185_u8,74_u8,75_u8,178_u8,74_u8];
_4 = 141_u8 as u64;
_6 = [69_u8,115_u8,200_u8,171_u8,188_u8,27_u8,149_u8,246_u8];
_12.2 = (9223372036854775807_isize, 122356890779877627392770256819340343635_i128, 30040_u16);
_12.2.2 = 39934_u16;
RET = _3 | _4;
_10 = _4 <= _3;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(6_usize, 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(6_usize, 5_usize, Move(_5), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u64,mut _2: [u8; 8],mut _3: u64,mut _4: u64,mut _5: u64) -> isize {
mir! {
type RET = isize;
let _6: ((isize, i128, u16), (*const [u32; 6], *const [u32; 6], [char; 6]));
let _7: u32;
let _8: i64;
let _9: isize;
let _10: [u8; 1];
let _11: i64;
let _12: ();
let _13: ();
{
_6.1.2 = ['\u{ed0a4}','\u{f2787}','\u{5d537}','\u{f37a1}','\u{bf04c}','\u{163b5}'];
Goto(bb1)
}
bb1 = {
_6.0.1 = 88533341464104601120178789466276170525_i128 * 17998739109177544675060019935063965004_i128;
Call(_6.1.0 = fn8(_5, _4, _1, _1, _2, _6.1.2, _3, _4, _6.1.2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb3)
}
bb3 = {
_2 = [117_u8,46_u8,93_u8,11_u8,51_u8,238_u8,48_u8,252_u8];
_6.1.1 = Move(_6.1.0);
_6.0.2 = 103240258256210323841055403934525311214_u128 as u16;
_3 = !_4;
_1 = !_4;
Call(_6.1.0 = core::intrinsics::arith_offset(_6.1.1, (-9223372036854775808_isize)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = false as u64;
_6.0 = (99_isize, (-102854449731652458619953935281268996377_i128), 5733_u16);
_5 = !_4;
_7 = _6.0.1 as u32;
RET = _6.0.0;
RET = _6.0.0 - _6.0.0;
_5 = 10043_i16 as u64;
_1 = _4;
_3 = _4;
RET = _6.0.0;
RET = _6.0.0 | _6.0.0;
_10 = [64_u8];
Goto(bb5)
}
bb5 = {
_9 = RET * RET;
RET = _9 & _9;
_11 = (-7999538661221982751_i64) << _3;
_4 = _1 & _1;
_2 = [225_u8,201_u8,125_u8,112_u8,145_u8,138_u8,114_u8,241_u8];
_2 = [233_u8,237_u8,251_u8,225_u8,239_u8,184_u8,87_u8,59_u8];
_9 = _6.0.1 as isize;
_9 = -RET;
Goto(bb6)
}
bb6 = {
Call(_12 = dump_var(7_usize, 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u64,mut _2: u64,mut _3: u64,mut _4: u64,mut _5: [u8; 8],mut _6: [char; 6],mut _7: u64,mut _8: u64,mut _9: [char; 6],mut _10: u64) -> *const [u32; 6] {
mir! {
type RET = *const [u32; 6];
let _11: (i16,);
let _12: i16;
let _13: f64;
let _14: [i32; 7];
let _15: Adt40;
let _16: isize;
let _17: f32;
let _18: *mut (i16,);
let _19: *mut bool;
let _20: Adt64;
let _21: [i16; 6];
let _22: (u64, char);
let _23: bool;
let _24: isize;
let _25: &'static Adt27;
let _26: isize;
let _27: char;
let _28: ([i32; 7], u64);
let _29: bool;
let _30: [i128; 1];
let _31: i32;
let _32: *mut f32;
let _33: isize;
let _34: (*mut Adt27,);
let _35: Adt27;
let _36: i8;
let _37: Adt64;
let _38: f32;
let _39: *const (isize, i128, u16);
let _40: i16;
let _41: f64;
let _42: u16;
let _43: (((i64, bool, [bool; 7], i64), (*mut Adt27, i32, u128, &'static usize)), [i16; 7], [bool; 7], ([i32; 7], u64));
let _44: ();
let _45: ();
{
_1 = !_4;
_6 = ['\u{1f554}','\u{4fbe2}','\u{b5b68}','\u{13bd}','\u{4a7d}','\u{7150a}'];
_2 = !_7;
_5 = [250_u8,240_u8,106_u8,212_u8,38_u8,199_u8,102_u8,187_u8];
_6 = _9;
_4 = '\u{7d35c}' as u64;
_9 = _6;
_9 = _6;
_11.0 = (-23846_i16);
_7 = _8;
_2 = !_3;
_11 = ((-18398_i16),);
_6 = ['\u{8bd99}','\u{92dab}','\u{35e61}','\u{1044b}','\u{1e685}','\u{a893d}'];
_6 = _9;
Call(_10 = fn9(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _1;
_1 = !_3;
_12 = false as i16;
_11.0 = (-72331195396482748308338600066557481668_i128) as i16;
_5 = [64_u8,105_u8,138_u8,71_u8,211_u8,241_u8,73_u8,50_u8];
_7 = _2;
_7 = _1 & _3;
_3 = !_7;
_10 = '\u{f69cf}' as u64;
_9 = ['\u{aca0b}','\u{101135}','\u{c5838}','\u{c3de5}','\u{9113c}','\u{3d8be}'];
_8 = !_3;
_10 = _8 >> _8;
_11.0 = _12;
_2 = _1;
_13 = 2561914016_u32 as f64;
_3 = !_8;
_9 = _6;
_11 = (_12,);
_4 = _7;
_6 = ['\u{1ff22}','\u{45a11}','\u{9c1db}','\u{cb782}','\u{3c333}','\u{70ac5}'];
_6 = ['\u{44613}','\u{d8444}','\u{5ec6b}','\u{27744}','\u{55357}','\u{9616}'];
_6 = ['\u{c1ea}','\u{571db}','\u{3afc8}','\u{36782}','\u{34448}','\u{1369e}'];
_9 = ['\u{8d808}','\u{6b739}','\u{a8a10}','\u{e4e33}','\u{a37bb}','\u{d6ca3}'];
_13 = _12 as f64;
_11.0 = _12 >> _7;
Goto(bb2)
}
bb2 = {
_15.fld0 = _10 != _1;
_2 = _3;
_4 = !_3;
_15.fld0 = _11.0 == _11.0;
_15.fld1 = 43892669377686666326361870347774864063_i128 as isize;
_18 = core::ptr::addr_of_mut!(_11);
_14 = [1558630687_i32,(-252384837_i32),863075608_i32,1426439094_i32,(-542000313_i32),(-1483783556_i32),(-114230365_i32)];
_18 = core::ptr::addr_of_mut!((*_18));
_17 = (-10168938719502200786748490254147836137_i128) as f32;
_17 = 233346615465533025008007846943640627205_u128 as f32;
_9 = ['\u{897eb}','\u{7ae26}','\u{5ca2e}','\u{1d288}','\u{be8fd}','\u{6e73}'];
_10 = _3;
_6 = ['\u{79298}','\u{d3f2f}','\u{9b848}','\u{f135f}','\u{1098e8}','\u{f3dd3}'];
_17 = (*_18).0 as f32;
_14 = [(-473819616_i32),1207970799_i32,(-1239876417_i32),149224453_i32,(-232493344_i32),1462752706_i32,894249535_i32];
_18 = core::ptr::addr_of_mut!((*_18));
(*_18).0 = _15.fld1 as i16;
_4 = _7 + _7;
_16 = _15.fld1 << _10;
_15.fld0 = false;
_3 = (*_18).0 as u64;
_1 = _10 | _7;
_18 = core::ptr::addr_of_mut!((*_18));
_19 = core::ptr::addr_of_mut!(_15.fld0);
(*_19) = false;
_16 = _15.fld1 - _15.fld1;
Goto(bb3)
}
bb3 = {
(*_19) = true;
_2 = _4 | _8;
(*_19) = _17 == _17;
_18 = core::ptr::addr_of_mut!((*_18));
_15.fld0 = false & false;
Call(RET = fn10(_2, _2, _2, _2, _7, _17, _15.fld0, _8, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = _4 ^ _8;
_15.fld0 = !false;
_11.0 = _12 + _12;
_14 = [(-1967329059_i32),1745119869_i32,644456475_i32,1661722519_i32,(-1372811134_i32),1255849804_i32,1279889756_i32];
_8 = !_2;
_1 = !_8;
_21 = [_12,_11.0,(*_18).0,_11.0,_11.0,_11.0];
_12 = (*_18).0;
_2 = !_4;
_5 = [162_u8,184_u8,152_u8,238_u8,8_u8,79_u8,95_u8,186_u8];
Goto(bb5)
}
bb5 = {
_22 = (_4, '\u{617a6}');
_8 = _22.0 << _22.0;
_15.fld0 = !false;
(*_19) = !true;
(*_19) = false & true;
_22 = (_1, '\u{511e}');
_23 = _15.fld0;
_4 = _2;
_15 = Adt40 { fld0: _23,fld1: _16 };
_12 = _11.0;
_15.fld1 = !_16;
_6 = _9;
_10 = !_7;
_23 = _8 < _2;
_19 = core::ptr::addr_of_mut!(_15.fld0);
_16 = _15.fld1;
Call((*_18).0 = core::intrinsics::transmute(_12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18 = core::ptr::addr_of_mut!((*_18));
(*_18).0 = -_12;
_5 = [81_u8,8_u8,98_u8,108_u8,8_u8,203_u8,247_u8,215_u8];
_17 = _13 as f32;
(*_18).0 = _12;
_2 = _16 as u64;
_14 = [833262133_i32,1738880243_i32,(-388204539_i32),(-1779548744_i32),(-1957411371_i32),198941196_i32,2048556796_i32];
_24 = (*_18).0 as isize;
_21 = [_12,_12,_11.0,(*_18).0,(*_18).0,_12];
_22.1 = '\u{109a1f}';
_1 = _7;
_13 = 37_i8 as f64;
_21 = [_12,_12,_12,(*_18).0,_11.0,_11.0];
_15.fld1 = _24 & _16;
_11.0 = _12;
_17 = _13 as f32;
_11 = (_12,);
_5 = [111_u8,252_u8,22_u8,34_u8,184_u8,199_u8,114_u8,240_u8];
_15 = Adt40 { fld0: _23,fld1: _16 };
Call(_3 = fn12(_7, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_27 = _22.1;
_23 = (*_19);
(*_18) = (_12,);
_26 = _16;
(*_18).0 = _12 + _12;
(*_19) = _23;
_17 = _13 as f32;
_15.fld0 = _23;
_15.fld0 = !_23;
_1 = _3 | _22.0;
Call(_19 = fn13(_10, _7, _10, _23, (*_19), Move(_15), _22, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_27 = _22.1;
_14 = [(-679751188_i32),2138737526_i32,(-1151966061_i32),2033689169_i32,(-1091888233_i32),18168284_i32,116711511_i32];
_28.1 = _16 as u64;
_16 = _24;
_28.1 = 12080_u16 as u64;
_28 = (_14, _3);
_6 = [_27,_22.1,_22.1,_22.1,_27,_22.1];
_13 = 121811192699283021380497307410145883931_u128 as f64;
_27 = _22.1;
_28 = (_14, _1);
Goto(bb9)
}
bb9 = {
_27 = _22.1;
(*_18).0 = _12;
_14 = [202701002_i32,856266814_i32,1814836278_i32,976303359_i32,(-537453832_i32),938632502_i32,447279773_i32];
_14 = _28.0;
_6 = _9;
_5 = [224_u8,189_u8,218_u8,227_u8,80_u8,117_u8,176_u8,139_u8];
_13 = _24 as f64;
(*_18).0 = !_12;
_31 = (-716976915_i32) << _22.0;
_30 = [64531418274347400226220299145619131170_i128];
_26 = !_24;
(*_18).0 = -_12;
_27 = _22.1;
_26 = 8952430707265317658_i64 as isize;
_23 = false;
_6 = [_22.1,_22.1,_27,_22.1,_22.1,_22.1];
Goto(bb10)
}
bb10 = {
_23 = !true;
_3 = 2438348717985097762_usize as u64;
_19 = core::ptr::addr_of_mut!(_15.fld0);
_15.fld0 = _23;
_1 = (-157033838607071349765521675329769161542_i128) as u64;
_33 = -_26;
_32 = core::ptr::addr_of_mut!(_17);
_34.0 = core::ptr::addr_of_mut!(_35);
_5 = [90_u8,41_u8,153_u8,16_u8,59_u8,11_u8,131_u8,118_u8];
_36 = -(-68_i8);
_29 = _10 >= _22.0;
(*_18) = (_12,);
_15.fld1 = _16 + _33;
_8 = _10;
_10 = _22.0 | _22.0;
Call(_2 = fn14(_28, _15.fld1, _4, _31, _22, _22, _29), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_18).0 = _12 ^ _12;
_23 = _22.0 >= _2;
_28.0 = _14;
_14 = [_31,_31,_31,_31,_31,_31,_31];
_22 = (_7, _27);
_16 = !_26;
(*_18).0 = (-2222113241716067345088363519872114490_i128) as i16;
_3 = _8 ^ _8;
(*_18).0 = 0_usize as i16;
_19 = core::ptr::addr_of_mut!((*_19));
_21 = [_12,_11.0,_12,(*_18).0,(*_18).0,(*_18).0];
_7 = _10 | _2;
_32 = core::ptr::addr_of_mut!((*_32));
_18 = core::ptr::addr_of_mut!(_11);
_12 = _11.0;
_28.1 = _3 + _7;
Goto(bb12)
}
bb12 = {
(*_18).0 = _13 as i16;
_34.0 = core::ptr::addr_of_mut!(_35);
(*_18) = (_12,);
Goto(bb13)
}
bb13 = {
_35 = Adt27::Variant2 { fld0: (*_32) };
_24 = _15.fld1 ^ _15.fld1;
_33 = 133_u8 as isize;
_33 = -_24;
_26 = _24 - _24;
_7 = !_10;
(*_18) = (_12,);
_18 = core::ptr::addr_of_mut!((*_18));
(*_32) = Field::<f32>(Variant(_35, 2), 0) - Field::<f32>(Variant(_35, 2), 0);
_6 = [_22.1,_22.1,_22.1,_22.1,_27,_22.1];
_23 = _29 | _29;
(*_19) = _22.0 <= _8;
_23 = !(*_19);
_24 = _26 & _33;
_41 = _13 + _13;
Goto(bb14)
}
bb14 = {
_5 = [209_u8,150_u8,226_u8,91_u8,167_u8,214_u8,40_u8,117_u8];
_15 = Adt40 { fld0: _23,fld1: _26 };
SetDiscriminant(_35, 0);
_18 = core::ptr::addr_of_mut!(_11);
_27 = _22.1;
_14 = [_31,_31,_31,_31,_31,_31,_31];
_1 = _17 as u64;
_38 = _17 * (*_32);
_7 = (*_19) as u64;
_43.1 = [(*_18).0,(*_18).0,(*_18).0,(*_18).0,(*_18).0,_11.0,(*_18).0];
_34.0 = core::ptr::addr_of_mut!(_35);
_7 = _2 & _10;
(*_18) = (_12,);
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(8_usize, 6_usize, Move(_6), 29_usize, Move(_29), 22_usize, Move(_22), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(8_usize, 10_usize, Move(_10), 24_usize, Move(_24), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(8_usize, 30_usize, Move(_30), 28_usize, Move(_28), 26_usize, Move(_26), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(8_usize, 1_usize, Move(_1), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [char; 6]) -> u64 {
mir! {
type RET = u64;
let _2: f32;
let _3: &'static usize;
let _4: [i128; 8];
let _5: &'static bool;
let _6: f32;
let _7: isize;
let _8: (char, (i128, isize, i8, (isize, i128, u16)), *mut f32);
let _9: [u32; 6];
let _10: [i128; 6];
let _11: ([i128; 1], &'static (i64, bool, [bool; 7], i64), [bool; 2], (*mut Adt27, i32, u128, &'static usize));
let _12: char;
let _13: char;
let _14: *mut Adt27;
let _15: u32;
let _16: isize;
let _17: [i16; 7];
let _18: f32;
let _19: Adt64;
let _20: *const [u32; 6];
let _21: ();
let _22: ();
{
RET = 7_usize as u64;
_1 = ['\u{b4b4}','\u{8131b}','\u{2f70b}','\u{269b5}','\u{9c48a}','\u{7480c}'];
RET = (-117552679_i32) as u64;
_1 = ['\u{5fed0}','\u{bd5ba}','\u{44f75}','\u{94576}','\u{96b1c}','\u{efcd5}'];
_4 = [(-7782672637004089104886785867103583672_i128),(-125285222052372309745471339499384664484_i128),21955965413942637873547067758376468592_i128,8434044984823870584856856803246534356_i128,(-16411037136913392065380288575077857204_i128),93663675815155579649104364469297831519_i128,(-98678886908186787462117884906639466703_i128),61925420763607503827059480385296190000_i128];
_4 = [(-105862351550201422279737909540484232095_i128),105089376895880611434195835837168741134_i128,(-124111802642042858362718955665306431680_i128),(-56168728747202666208057707153063476513_i128),(-68554836443483614107071542817230822904_i128),143994905120246095576387867735665783764_i128,55169829145287479050993725214504850773_i128,(-24872679768741037770393422597322846889_i128)];
_4 = [131104786165069097977085615429464437982_i128,(-1500762775165846116077936084424185916_i128),48276208146990848365575928357929749169_i128,6237404303737779083266151845386467467_i128,96721533718633333661413925974495976451_i128,58318008281765430811456023834690382416_i128,(-29568092621556824558524741648208995541_i128),153369832155368461348384111889648060750_i128];
_2 = 264200373923504566789190427887554263414_u128 as f32;
_6 = 8026_i16 as f32;
_7 = (-9223372036854775808_isize);
match _7 {
340282366920938463454151235394913435648 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_9 = [985652433_u32,2461419011_u32,1011228444_u32,745837020_u32,2368261379_u32,3053427033_u32];
RET = 13351560122136714889_u64 + 3759948985663108087_u64;
_8.0 = '\u{e2ead}';
_9 = [3248583204_u32,1181460680_u32,2005587135_u32,951839369_u32,4167958154_u32,3936152967_u32];
Goto(bb3)
}
bb3 = {
_8.1.0 = (-24599816885535868036799098193261195950_i128);
_8.1.3.0 = (-24300_i16) as isize;
_8.1.3.1 = _8.1.0 - _8.1.0;
_8.1.3.0 = _7 | _7;
RET = _8.1.3.0 as u64;
_6 = _2 + _2;
_8.1.3 = (_7, _8.1.0, 29877_u16);
_10 = [_8.1.3.1,_8.1.0,_8.1.0,_8.1.0,_8.1.3.1,_8.1.3.1];
_11.3.2 = _8.1.3.2 as u128;
_10 = [_8.1.0,_8.1.3.1,_8.1.3.1,_8.1.0,_8.1.3.1,_8.1.0];
RET = 9643666876485375923_u64 + 7830612004717341663_u64;
_8.2 = core::ptr::addr_of_mut!(_6);
Goto(bb4)
}
bb4 = {
_8.1.3.1 = 16372855800108357312_usize as i128;
_11.2 = [false,true];
_8.1.1 = 162_u8 as isize;
_8.1.3.2 = !43987_u16;
_12 = _8.0;
_8.1.3 = (_7, _8.1.0, 58129_u16);
_2 = _8.1.3.2 as f32;
_13 = _12;
_8.1.3.2 = 48961_u16 - 19568_u16;
_11.2 = [false,false];
RET = 15027104850489592482_u64 | 7810536680999117618_u64;
_8.1.3.2 = !11683_u16;
_11.3.1 = 2018334823_i32;
_4 = [_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.0,_8.1.0,_8.1.3.1];
_1 = [_13,_13,_12,_12,_12,_13];
_11.3.1 = (-8411225154058253226_i64) as i32;
match _8.1.3.0 {
0 => bb2,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_15 = 1855992616_u32;
_2 = _6 + _6;
_10 = [_8.1.0,_8.1.3.1,_8.1.0,_8.1.3.1,_8.1.3.1,_8.1.3.1];
_8.1.2 = RET as i8;
_11.2 = [false,true];
_8.1.3.2 = 23847_u16;
_11.3.1 = (-1595241990_i32) >> _11.3.2;
_12 = _8.0;
_8.1.2 = 106_i8 | 55_i8;
match _8.1.3.2 {
0 => bb2,
23847 => bb8,
_ => bb7
}
}
bb7 = {
_9 = [985652433_u32,2461419011_u32,1011228444_u32,745837020_u32,2368261379_u32,3053427033_u32];
RET = 13351560122136714889_u64 + 3759948985663108087_u64;
_8.0 = '\u{e2ead}';
_9 = [3248583204_u32,1181460680_u32,2005587135_u32,951839369_u32,4167958154_u32,3936152967_u32];
Goto(bb3)
}
bb8 = {
_8.1.3.1 = _8.1.0 - _8.1.0;
_8.1.0 = _8.1.3.1 - _8.1.3.1;
RET = 210_u8 as u64;
_8.2 = core::ptr::addr_of_mut!(_6);
_11.3.1 = (-4587_i16) as i32;
_8.1.3 = (_8.1.1, _8.1.0, 18410_u16);
_9 = [_15,_15,_15,_15,_15,_15];
_16 = _7;
_9 = [_15,_15,_15,_15,_15,_15];
_8.1.3.0 = _8.1.1;
_8.0 = _12;
_11.2 = [false,false];
_8.1.3.2 = 51028_u16 - 7236_u16;
_2 = -_6;
_15 = 1546397026_u32;
_12 = _8.0;
_15 = 4422515525113265207_usize as u32;
_4 = [_8.1.3.1,_8.1.0,_8.1.3.1,_8.1.0,_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.3.1];
_11.0 = [_8.1.0];
RET = !2059406772552363829_u64;
_11.2 = [false,true];
RET = _16 as u64;
_11.3.1 = _8.1.3.2 as i32;
match _16 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb9 = {
_9 = [985652433_u32,2461419011_u32,1011228444_u32,745837020_u32,2368261379_u32,3053427033_u32];
RET = 13351560122136714889_u64 + 3759948985663108087_u64;
_8.0 = '\u{e2ead}';
_9 = [3248583204_u32,1181460680_u32,2005587135_u32,951839369_u32,4167958154_u32,3936152967_u32];
Goto(bb3)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_8.1.3.1 = 16372855800108357312_usize as i128;
_11.2 = [false,true];
_8.1.1 = 162_u8 as isize;
_8.1.3.2 = !43987_u16;
_12 = _8.0;
_8.1.3 = (_7, _8.1.0, 58129_u16);
_2 = _8.1.3.2 as f32;
_13 = _12;
_8.1.3.2 = 48961_u16 - 19568_u16;
_11.2 = [false,false];
RET = 15027104850489592482_u64 | 7810536680999117618_u64;
_8.1.3.2 = !11683_u16;
_11.3.1 = 2018334823_i32;
_4 = [_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.3.1,_8.1.0,_8.1.0,_8.1.3.1];
_1 = [_13,_13,_12,_12,_12,_13];
_11.3.1 = (-8411225154058253226_i64) as i32;
match _8.1.3.0 {
0 => bb2,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb13 = {
_8.1.0 = (-24599816885535868036799098193261195950_i128);
_8.1.3.0 = (-24300_i16) as isize;
_8.1.3.1 = _8.1.0 - _8.1.0;
_8.1.3.0 = _7 | _7;
RET = _8.1.3.0 as u64;
_6 = _2 + _2;
_8.1.3 = (_7, _8.1.0, 29877_u16);
_10 = [_8.1.3.1,_8.1.0,_8.1.0,_8.1.0,_8.1.3.1,_8.1.3.1];
_11.3.2 = _8.1.3.2 as u128;
_10 = [_8.1.0,_8.1.3.1,_8.1.3.1,_8.1.0,_8.1.3.1,_8.1.0];
RET = 9643666876485375923_u64 + 7830612004717341663_u64;
_8.2 = core::ptr::addr_of_mut!(_6);
Goto(bb4)
}
bb14 = {
_9 = [985652433_u32,2461419011_u32,1011228444_u32,745837020_u32,2368261379_u32,3053427033_u32];
RET = 13351560122136714889_u64 + 3759948985663108087_u64;
_8.0 = '\u{e2ead}';
_9 = [3248583204_u32,1181460680_u32,2005587135_u32,951839369_u32,4167958154_u32,3936152967_u32];
Goto(bb3)
}
bb15 = {
_17 = [20461_i16,(-30788_i16),1681_i16,32035_i16,25884_i16,15599_i16,3179_i16];
_9 = [_15,_15,_15,_15,_15,_15];
_11.3.2 = !21752916990846474511140791730404597127_u128;
_8.2 = core::ptr::addr_of_mut!(_18);
_17 = [31146_i16,9681_i16,(-6833_i16),25704_i16,(-27642_i16),26537_i16,19212_i16];
_8.2 = core::ptr::addr_of_mut!(_2);
_8.1.2 = 14340_i16 as i8;
_11.3.2 = !336040691235889597398884389040210044983_u128;
_8.2 = core::ptr::addr_of_mut!(_18);
_8.1.3.2 = _11.3.1 as u16;
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(9_usize, 16_usize, Move(_16), 9_usize, Move(_9), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(9_usize, 7_usize, Move(_7), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u64,mut _2: u64,mut _3: u64,mut _4: u64,mut _5: u64,mut _6: f32,mut _7: bool,mut _8: u64,mut _9: u64) -> *const [u32; 6] {
mir! {
type RET = *const [u32; 6];
let _10: isize;
let _11: [u32; 6];
let _12: (i128, isize, i8, (isize, i128, u16));
let _13: [char; 6];
let _14: u64;
let _15: char;
let _16: char;
let _17: f64;
let _18: (((i64, bool, [bool; 7], i64), (*mut Adt27, i32, u128, &'static usize)), [i16; 7], [bool; 7], ([i32; 7], u64));
let _19: f32;
let _20: &'static usize;
let _21: [i16; 6];
let _22: *mut f32;
let _23: (*const i8, (i16,));
let _24: *mut (i16,);
let _25: u128;
let _26: ();
let _27: ();
{
_7 = !false;
_9 = _1;
_8 = _5 * _5;
Goto(bb1)
}
bb1 = {
_10 = -(-28_isize);
_5 = _4 | _3;
_12.3 = (_10, (-56421830557224283420780879380178976120_i128), 35377_u16);
_12.3.1 = 2_usize as i128;
match _12.3.2 {
0 => bb2,
35377 => bb4,
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
_7 = _4 != _1;
RET = core::ptr::addr_of!(_11);
(*RET) = [723263871_u32,3034710425_u32,858967701_u32,1397850327_u32,1886358465_u32,4009927597_u32];
_5 = _4;
_12.0 = _12.3.1 + _12.3.1;
_1 = _4 + _3;
_12.3.2 = 7478_u16 | 28446_u16;
_12.3.0 = _6 as isize;
_12.3.2 = 55302_u16 << _1;
_4 = _2;
(*RET) = [2681700257_u32,3801325979_u32,3381593991_u32,3947912441_u32,3196401113_u32,2784179586_u32];
_9 = 266061290474176838881071981000726686308_u128 as u64;
RET = core::ptr::addr_of!((*RET));
_5 = _8;
_6 = _12.3.0 as f32;
_11 = [627360042_u32,2590329007_u32,4048009206_u32,3300471303_u32,209170839_u32,2167149703_u32];
_13 = ['\u{22eba}','\u{efb8f}','\u{568e8}','\u{397c7}','\u{a716e}','\u{10b4b0}'];
_12.3.1 = _12.0;
_11 = [2310850953_u32,4260173272_u32,3439242211_u32,263313987_u32,1652472524_u32,911977472_u32];
_2 = _3 | _5;
_12.2 = (-48_i8);
_12.3.2 = 5283_u16 + 9318_u16;
_12.3.1 = _12.0;
RET = core::ptr::addr_of!((*RET));
_11 = [3560067792_u32,2336473713_u32,3410784109_u32,308497486_u32,3559469541_u32,3855559208_u32];
_11 = [2509897994_u32,2578718393_u32,3181036096_u32,1748745956_u32,2816865808_u32,1345147534_u32];
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211408 => bb8,
_ => bb7
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_10 = -(-28_isize);
_5 = _4 | _3;
_12.3 = (_10, (-56421830557224283420780879380178976120_i128), 35377_u16);
_12.3.1 = 2_usize as i128;
match _12.3.2 {
0 => bb2,
35377 => bb4,
_ => bb3
}
}
bb8 = {
_13 = ['\u{8955c}','\u{bb602}','\u{63963}','\u{af299}','\u{dcafe}','\u{f96ee}'];
_11 = [2240701941_u32,2066522976_u32,1627055485_u32,2761614081_u32,955985854_u32,4284926629_u32];
_12.0 = _12.3.1 << _4;
_15 = '\u{df8de}';
_12.0 = _12.3.1 ^ _12.3.1;
RET = core::ptr::addr_of!((*RET));
_1 = _3 >> _5;
_14 = !_2;
_3 = _2 ^ _1;
_7 = false;
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211408 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_6 = 307724614379117752836522002140197372447_u128 as f32;
_13 = [_15,_15,_15,_15,_15,_15];
_10 = _12.3.0 * _12.3.0;
_3 = (-3436117340270005246_i64) as u64;
_9 = _7 as u64;
_18.0.1.1 = 1654396371_i32;
_18.3.0 = [_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1];
_12.1 = -_12.3.0;
_12.2 = !(-52_i8);
RET = core::ptr::addr_of!((*RET));
_18.0.0.2 = [_7,_7,_7,_7,_7,_7,_7];
_18.1 = [(-8144_i16),(-24035_i16),32488_i16,(-10784_i16),(-9247_i16),6866_i16,(-14114_i16)];
_18.3.0 = [_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1];
RET = core::ptr::addr_of!((*RET));
_11 = [3229737403_u32,34676752_u32,1662814526_u32,1257997470_u32,3683031801_u32,914895443_u32];
_18.0.0.0 = (-8619080063607328504_i64);
_6 = _12.3.2 as f32;
match _18.0.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
340282366920938463454755527368160882952 => bb11,
_ => bb8
}
}
bb11 = {
_18.1 = [12067_i16,(-29461_i16),(-12571_i16),(-27499_i16),(-7363_i16),19062_i16,18204_i16];
_2 = 2983_i16 as u64;
_18.0.0.3 = !_18.0.0.0;
_3 = !_8;
_5 = _1 - _4;
RET = core::ptr::addr_of!((*RET));
_18.3.1 = !_3;
_18.0.0.1 = _7 | _7;
_18.0.1.2 = 272738872664301268038064640431039481451_u128;
_18.3.1 = 1056387906_u32 as u64;
(*RET) = [1784473453_u32,691318448_u32,1371077776_u32,1645649793_u32,936170878_u32,3048200076_u32];
Goto(bb12)
}
bb12 = {
_18.2 = [_18.0.0.1,_18.0.0.1,_18.0.0.1,_7,_7,_18.0.0.1,_18.0.0.1];
_18.3.1 = 68118430_u32 as u64;
_12.3.1 = !_12.0;
_21 = [(-28482_i16),12445_i16,(-9214_i16),(-16351_i16),(-24976_i16),(-961_i16)];
_18.0.0.2 = _18.2;
_17 = 2109437286_u32 as f64;
_16 = _15;
_18.2 = _18.0.0.2;
_11 = [317405218_u32,1605969736_u32,2587556613_u32,3187385359_u32,3586119935_u32,3073254552_u32];
_12.0 = _12.3.1;
Goto(bb13)
}
bb13 = {
_18.0.0.2 = [_7,_18.0.0.1,_18.0.0.1,_18.0.0.1,_7,_18.0.0.1,_18.0.0.1];
_23.0 = core::ptr::addr_of!(_12.2);
_12.0 = !_12.3.1;
_13 = [_15,_15,_15,_15,_15,_15];
_9 = _1 ^ _5;
_18.0.0.1 = _7;
_18.0.1.1 = 3581884605_u32 as i32;
_18.0.1.1 = 636459498_i32 & (-8365135_i32);
_18.0.0 = (4987082982716282712_i64, _7, _18.2, 5053554591141123825_i64);
_18.0.0.0 = _18.0.0.3;
_18.0.0.2 = [_18.0.0.1,_18.0.0.1,_18.0.0.1,_18.0.0.1,_18.0.0.1,_7,_18.0.0.1];
Call(_2 = fn11(_12.1, _12.3, _12.3, _5, _1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17 = 43_u8 as f64;
_19 = -_6;
_19 = _18.0.1.1 as f32;
_18.0.1.2 = 66887281611280526904388626245348536708_u128 << _12.1;
_14 = _18.0.1.2 as u64;
_18.0.0.0 = _18.0.1.2 as i64;
_18.0.0.1 = _7 & _7;
_18.3.0 = [_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1,_18.0.1.1];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(10_usize, 14_usize, Move(_14), 2_usize, Move(_2), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(10_usize, 4_usize, Move(_4), 11_usize, Move(_11), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: (isize, i128, u16),mut _3: (isize, i128, u16),mut _4: u64,mut _5: u64) -> u64 {
mir! {
type RET = u64;
let _6: ((i64, bool, [bool; 7], i64), (*mut Adt27, i32, u128, &'static usize));
let _7: ();
let _8: ();
{
_3.0 = _1 - _2.0;
RET = _4;
_3.2 = !_2.2;
_5 = !RET;
_3.0 = _1;
_4 = false as u64;
RET = _5 << _3.0;
_5 = RET;
_3.0 = -_1;
_6.0.0 = -(-716316831342253016_i64);
_2.1 = !_3.1;
_2.2 = _3.2;
_6.0.3 = 7_usize as i64;
_2.1 = _3.1 ^ _3.1;
_6.1.1 = (-1828015952_i32) >> _2.1;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(11_usize, 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u64,mut _2: u64) -> u64 {
mir! {
type RET = u64;
let _3: *mut i128;
let _4: ();
let _5: ();
{
_2 = !_1;
_2 = _1;
RET = _2 | _2;
_2 = 139902404835029659862019281572618874555_u128 as u64;
RET = !_1;
Goto(bb1)
}
bb1 = {
Call(_4 = dump_var(12_usize, 2_usize, Move(_2), 5_usize, _5, 5_usize, _5, 5_usize, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u64,mut _2: u64,mut _3: u64,mut _4: bool,mut _5: bool,mut _6: Adt40,mut _7: (u64, char),mut _8: u64) -> *mut bool {
mir! {
type RET = *mut bool;
let _9: Adt64;
let _10: isize;
let _11: ();
let _12: ();
{
_6 = Adt40 { fld0: _4,fld1: 9223372036854775807_isize };
_8 = _1;
_7 = (_2, '\u{3a136}');
_6.fld0 = !_4;
_7 = (_1, '\u{e7185}');
_7 = (_3, '\u{4faea}');
RET = core::ptr::addr_of_mut!(_5);
(*RET) = _6.fld0;
_7.0 = 67057447414864731232694668304587891049_u128 as u64;
(*RET) = _4 & _6.fld0;
_8 = !_3;
_5 = _4;
_6 = Adt40 { fld0: _4,fld1: 56_isize };
(*RET) = _6.fld0;
RET = core::ptr::addr_of_mut!(_4);
(*RET) = _6.fld0;
(*RET) = !_6.fld0;
_1 = _3;
_7.1 = '\u{3db1f}';
_10 = !_6.fld1;
_5 = !_4;
_8 = (-5074_i16) as u64;
(*RET) = !_5;
(*RET) = _5;
(*RET) = _2 != _3;
(*RET) = _6.fld0 >= _6.fld0;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(13_usize, 2_usize, Move(_2), 7_usize, Move(_7), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: ([i32; 7], u64),mut _2: isize,mut _3: u64,mut _4: i32,mut _5: (u64, char),mut _6: (u64, char),mut _7: bool) -> u64 {
mir! {
type RET = u64;
let _8: char;
let _9: (*mut Adt27,);
let _10: [bool; 2];
let _11: (i16,);
let _12: (isize,);
let _13: ();
let _14: ();
{
RET = _6.0 - _5.0;
_1.0 = [_4,_4,_4,_4,_4,_4,_4];
_5.0 = !RET;
_11 = ((-10019_i16),);
_1.1 = _5.0;
_3 = _5.0;
_1.1 = !_5.0;
_5.0 = !_3;
_6 = (_5.0, _5.1);
_12 = (_2,);
_2 = _4 as isize;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(14_usize, 3_usize, Move(_3), 5_usize, Move(_5), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [i16; 6],mut _2: [i16; 6],mut _3: [i16; 6],mut _4: isize,mut _5: u64,mut _6: i16,mut _7: u64) -> bool {
mir! {
type RET = bool;
let _8: (isize,);
let _9: f64;
let _10: f64;
let _11: ();
let _12: ();
{
_7 = _5 << _5;
RET = true;
_4 = (-44_isize);
_5 = !_7;
RET = _7 == _7;
_8.0 = !_4;
_2 = _3;
_9 = 36691494184387725227664593427999662829_i128 as f64;
_2 = [_6,_6,_6,_6,_6,_6];
_1 = [_6,_6,_6,_6,_6,_6];
_9 = 230_u8 as f64;
_2 = [_6,_6,_6,_6,_6,_6];
_10 = -_9;
_1 = [_6,_6,_6,_6,_6,_6];
_9 = 1_usize as f64;
_7 = _5;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(15_usize, 1_usize, Move(_1), 8_usize, Move(_8), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u8; 8],mut _2: [u8; 8],mut _3: i16,mut _4: [i128; 8],mut _5: [u8; 8],mut _6: [u8; 8],mut _7: isize,mut _8: bool,mut _9: u64) -> [u8; 8] {
mir! {
type RET = [u8; 8];
let _10: isize;
let _11: char;
let _12: usize;
let _13: *const [u32; 6];
let _14: [i128; 1];
let _15: f32;
let _16: (f64, &'static Adt27, &'static bool, *const i8);
let _17: i32;
let _18: bool;
let _19: (i128, isize, i8, (isize, i128, u16));
let _20: i8;
let _21: &'static *const [u32; 6];
let _22: (((i64, bool, [bool; 7], i64), (*mut Adt27, i32, u128, &'static usize)), [i16; 7], [bool; 7], ([i32; 7], u64));
let _23: char;
let _24: Adt27;
let _25: usize;
let _26: char;
let _27: (char, (i128, isize, i8, (isize, i128, u16)), *mut f32);
let _28: isize;
let _29: &'static &'static usize;
let _30: (i64, bool, [bool; 7], i64);
let _31: u128;
let _32: (*mut (i16,),);
let _33: *mut i128;
let _34: *const &'static (*mut Adt27,);
let _35: Adt30;
let _36: Adt30;
let _37: (i16,);
let _38: (u64, char);
let _39: bool;
let _40: ();
let _41: ();
{
_8 = _3 > _3;
_8 = !true;
_3 = (-31098_i16);
_8 = !false;
_6 = [238_u8,209_u8,80_u8,234_u8,62_u8,120_u8,188_u8,228_u8];
RET = [245_u8,139_u8,159_u8,230_u8,133_u8,27_u8,122_u8,172_u8];
_5 = _2;
RET = [64_u8,25_u8,86_u8,121_u8,194_u8,24_u8,203_u8,254_u8];
_8 = !false;
_4 = [(-98497064332039185305422595881904199749_i128),(-41843435111564973641656602420646559529_i128),(-151951049202473815244444773473115992988_i128),(-157784063024590812676202171499891856468_i128),80676186967927998773041265711091455292_i128,62778367417341414749854805481255509296_i128,(-91027059983854431308151150823744615670_i128),58779731504642797935807470722761984602_i128];
RET = [15_u8,200_u8,169_u8,81_u8,225_u8,112_u8,143_u8,28_u8];
_5 = _2;
_5 = [152_u8,82_u8,228_u8,254_u8,119_u8,54_u8,237_u8,218_u8];
_4 = [106376832534484293782617614579357127572_i128,(-158865348248018313844577277443098658073_i128),(-117100144216604323305272292419812818453_i128),(-27683993599398469137986468795857545121_i128),33668788606087391561993802092710867718_i128,(-106325528444295480387574636614115297972_i128),146212635224581172218901199989492477014_i128,(-74277542510896803265431519059317082007_i128)];
RET = _6;
_8 = true;
_2 = RET;
_1 = _6;
_4 = [(-97573542221594159683723243626153199971_i128),(-21485992746993004706864619542301355216_i128),(-42035134513471360705946536435445360674_i128),164552900633186172514659211585718573099_i128,75527271348280346946770166941342836227_i128,92811844984803642012145271971161574924_i128,(-36010435809875982436049927146542553330_i128),(-86769684059819051599988229300766295928_i128)];
_4 = [(-148793466450960651935900719761050465334_i128),152806327486984878163762278411565425239_i128,(-52006029777494752456967109255942223540_i128),98073046171734278532977200931511154180_i128,68384779840969508647604354150638444386_i128,(-37731534392690621031547997127888531828_i128),10210532691696990863226423670176224860_i128,(-164956259620797825754883476582657904441_i128)];
_2 = [203_u8,231_u8,15_u8,128_u8,118_u8,73_u8,185_u8,237_u8];
Call(_10 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [189_u8,126_u8,190_u8,138_u8,165_u8,155_u8,98_u8,227_u8];
match _3 {
0 => bb2,
340282366920938463463374607431768180358 => bb4,
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
_2 = _5;
_2 = [187_u8,70_u8,65_u8,155_u8,147_u8,130_u8,1_u8,239_u8];
_7 = _10;
_15 = 110_i8 as f32;
_1 = [42_u8,175_u8,160_u8,82_u8,229_u8,125_u8,247_u8,204_u8];
_5 = [104_u8,60_u8,212_u8,51_u8,228_u8,21_u8,142_u8,39_u8];
_10 = _7;
match _3 {
340282366920938463463374607431768180358 => bb5,
_ => bb2
}
}
bb5 = {
_12 = 7_usize << _7;
Goto(bb6)
}
bb6 = {
_2 = [95_u8,64_u8,68_u8,148_u8,68_u8,65_u8,12_u8,207_u8];
_11 = '\u{593ae}';
_16.0 = _7 as f64;
_4 = [(-8083984621374232728886389405360294467_i128),(-92832801952419393045102209583729647713_i128),61869270696411946509244006149294272006_i128,30442205002749125174437256840015291842_i128,(-5535632990839760098582487807121426672_i128),15029194970808445273286234852429609196_i128,30021874583902389308540803776690759692_i128,(-53310918471287084320345604778184991498_i128)];
_15 = (-1089451956_i32) as f32;
_6 = [143_u8,173_u8,161_u8,75_u8,206_u8,57_u8,84_u8,175_u8];
_3 = 5563_i16;
_19.2 = !76_i8;
_17 = (-466055799_i32);
_15 = _9 as f32;
_12 = 20_u8 as usize;
_19.0 = 834307093755598581374864549671946983_i128;
_19.1 = -_7;
_18 = _15 == _15;
_19.1 = _7 - _10;
_1 = RET;
_14 = [_19.0];
_22.0.0.3 = !(-5436409379265965366_i64);
_19.3.1 = _22.0.0.3 as i128;
_7 = _11 as isize;
_22.0.0.2 = [_18,_18,_18,_18,_18,_8,_18];
_22.0.0.1 = !_8;
Goto(bb7)
}
bb7 = {
_6 = [6_u8,24_u8,184_u8,23_u8,117_u8,124_u8,119_u8,187_u8];
_22.3.1 = !_9;
_21 = &_13;
_22.2 = [_18,_18,_8,_18,_8,_22.0.0.1,_18];
_3 = (-9034_i16) * 16424_i16;
_3 = !(-26281_i16);
_22.1 = [_3,_3,_3,_3,_3,_3,_3];
_22.0.0 = (2027112299261376320_i64, _18, _22.2, (-7126421753603666903_i64));
_5 = [146_u8,198_u8,79_u8,19_u8,138_u8,223_u8,132_u8,7_u8];
_7 = _19.1 * _19.1;
match _22.0.0.3 {
0 => bb5,
1 => bb2,
340282366920938463456248185678164544553 => bb9,
_ => bb8
}
}
bb8 = {
_2 = _5;
_2 = [187_u8,70_u8,65_u8,155_u8,147_u8,130_u8,1_u8,239_u8];
_7 = _10;
_15 = 110_i8 as f32;
_1 = [42_u8,175_u8,160_u8,82_u8,229_u8,125_u8,247_u8,204_u8];
_5 = [104_u8,60_u8,212_u8,51_u8,228_u8,21_u8,142_u8,39_u8];
_10 = _7;
match _3 {
340282366920938463463374607431768180358 => bb5,
_ => bb2
}
}
bb9 = {
_22.3.0 = [_17,_17,_17,_17,_17,_17,_17];
_25 = !_12;
_22.0.1.0 = core::ptr::addr_of_mut!(_24);
_22.0.1.2 = 87_u8 as u128;
_16.0 = _22.0.0.0 as f64;
RET = [114_u8,249_u8,224_u8,119_u8,223_u8,113_u8,205_u8,165_u8];
_19.3 = (_19.1, _19.0, 55881_u16);
_27.1 = (_19.0, _10, _19.2, _19.3);
_19.3.2 = _27.1.3.2 ^ _27.1.3.2;
_27.1.3.0 = 50_u8 as isize;
_22.0.1.2 = !142056475851659211680113452044168014932_u128;
_3 = _9 as i16;
_22.0.0.0 = _22.0.0.3;
_16.3 = core::ptr::addr_of!(_19.2);
_16.2 = &_8;
Goto(bb10)
}
bb10 = {
_22.0.0.1 = _18;
_19 = (_27.1.0, _10, _27.1.2, _27.1.3);
_4 = [_19.0,_27.1.0,_19.0,_19.3.1,_27.1.0,_27.1.0,_19.3.1,_27.1.3.1];
_26 = _11;
match _27.1.3.2 {
0 => bb5,
1 => bb8,
2 => bb11,
3 => bb12,
55881 => bb14,
_ => bb13
}
}
bb11 = {
_22.3.0 = [_17,_17,_17,_17,_17,_17,_17];
_25 = !_12;
_22.0.1.0 = core::ptr::addr_of_mut!(_24);
_22.0.1.2 = 87_u8 as u128;
_16.0 = _22.0.0.0 as f64;
RET = [114_u8,249_u8,224_u8,119_u8,223_u8,113_u8,205_u8,165_u8];
_19.3 = (_19.1, _19.0, 55881_u16);
_27.1 = (_19.0, _10, _19.2, _19.3);
_19.3.2 = _27.1.3.2 ^ _27.1.3.2;
_27.1.3.0 = 50_u8 as isize;
_22.0.1.2 = !142056475851659211680113452044168014932_u128;
_3 = _9 as i16;
_22.0.0.0 = _22.0.0.3;
_16.3 = core::ptr::addr_of!(_19.2);
_16.2 = &_8;
Goto(bb10)
}
bb12 = {
_2 = _5;
_2 = [187_u8,70_u8,65_u8,155_u8,147_u8,130_u8,1_u8,239_u8];
_7 = _10;
_15 = 110_i8 as f32;
_1 = [42_u8,175_u8,160_u8,82_u8,229_u8,125_u8,247_u8,204_u8];
_5 = [104_u8,60_u8,212_u8,51_u8,228_u8,21_u8,142_u8,39_u8];
_10 = _7;
match _3 {
340282366920938463463374607431768180358 => bb5,
_ => bb2
}
}
bb13 = {
_2 = [95_u8,64_u8,68_u8,148_u8,68_u8,65_u8,12_u8,207_u8];
_11 = '\u{593ae}';
_16.0 = _7 as f64;
_4 = [(-8083984621374232728886389405360294467_i128),(-92832801952419393045102209583729647713_i128),61869270696411946509244006149294272006_i128,30442205002749125174437256840015291842_i128,(-5535632990839760098582487807121426672_i128),15029194970808445273286234852429609196_i128,30021874583902389308540803776690759692_i128,(-53310918471287084320345604778184991498_i128)];
_15 = (-1089451956_i32) as f32;
_6 = [143_u8,173_u8,161_u8,75_u8,206_u8,57_u8,84_u8,175_u8];
_3 = 5563_i16;
_19.2 = !76_i8;
_17 = (-466055799_i32);
_15 = _9 as f32;
_12 = 20_u8 as usize;
_19.0 = 834307093755598581374864549671946983_i128;
_19.1 = -_7;
_18 = _15 == _15;
_19.1 = _7 - _10;
_1 = RET;
_14 = [_19.0];
_22.0.0.3 = !(-5436409379265965366_i64);
_19.3.1 = _22.0.0.3 as i128;
_7 = _11 as isize;
_22.0.0.2 = [_18,_18,_18,_18,_18,_8,_18];
_22.0.0.1 = !_8;
Goto(bb7)
}
bb14 = {
_22.0.0.2 = [_22.0.0.1,_22.0.0.1,_22.0.0.1,_18,_8,_22.0.0.1,_22.0.0.1];
_22.2 = [_22.0.0.1,_22.0.0.1,_22.0.0.1,_22.0.0.1,_22.0.0.1,_22.0.0.1,_22.0.0.1];
_22.0.1.0 = core::ptr::addr_of_mut!(_24);
_22.0.1.3 = &_12;
_19.1 = _27.1.1 | _7;
_19.2 = !_27.1.2;
_4 = [_27.1.3.1,_19.0,_27.1.0,_27.1.3.1,_19.3.1,_27.1.3.1,_19.0,_27.1.3.1];
_22.0.0.3 = !_22.0.0.0;
_22.2 = [_18,_18,_22.0.0.1,_22.0.0.1,_8,_18,_18];
_23 = _11;
_22.0.0 = (302691770112154650_i64, _18, _22.2, (-3009470053357989877_i64));
_19.3 = (_19.1, _27.1.3.1, _27.1.3.2);
_27.1.3.1 = _19.0 - _19.3.1;
_11 = _26;
_30.2 = [_22.0.0.1,_18,_22.0.0.1,_22.0.0.1,_18,_22.0.0.1,_22.0.0.1];
_12 = _19.3.1 as usize;
_22.0.1.1 = -_17;
_26 = _23;
_6 = [40_u8,234_u8,209_u8,201_u8,139_u8,226_u8,59_u8,32_u8];
_22.1 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(16_usize, 14_usize, Move(_14), 23_usize, Move(_23), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(16_usize, 1_usize, Move(_1), 25_usize, Move(_25), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 17_usize, Move(_17), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(1952700747_i32), std::hint::black_box((-9223372036854775808_isize)));
                
            }
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: f32,

},
Variant1{
fld0: [bool; 7],
fld1: char,
fld2: [bool; 2],
fld3: *const f32,

},
Variant2{
fld0: f32,

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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt30 {
Variant0{
fld0: u16,
fld1: f64,

},
Variant1{
fld0: i8,

},
Variant2{
fld0: usize,

},
Variant3{
fld0: i16,
fld1: f64,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: bool,
fld1: isize,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: (*const [u32; 6], *const [u32; 6], [char; 6]),
fld1: (i64, bool, [bool; 7], i64),
fld2: (i16,),
fld3: *mut (i16,),
fld4: (isize, i128, u16),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: *const i8,
fld1: f32,
fld2: [i32; 7],
fld3: i8,
fld4: i16,
fld5: [char; 6],
fld6: Adt30,
fld7: usize,

},
Variant1{
fld0: ([i32; 7], u64),
fld1: char,
fld2: *mut (i16,),
fld3: (*mut Adt27,),

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
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: [u32; 6],
fld1: u16,
fld2: ([i32; 7], u64),

},
Variant1{
fld0: (*mut (i16,),),
fld1: (char, (i128, isize, i8, (isize, i128, u16)), *mut f32),
fld2: i128,

},
Variant2{
fld0: [i16; 6],
fld1: (i64, bool, [bool; 7], i64),
fld2: [u32; 6],

},
Variant3{
fld0: *mut ([i32; 7], u64),
fld1: (i64, bool, [bool; 7], i64),
fld2: (char, (i128, isize, i8, (isize, i128, u16)), *mut f32),
fld3: i8,
fld4: i128,

}}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: [i16; 6],
fld1: u64,
fld2: *mut (i16,),
fld3: (*const i8, (i16,)),
fld4: u32,

},
Variant1{
fld0: u16,
fld1: char,
fld2: *const i8,
fld3: (i64, bool, [bool; 7], i64),
fld4: [i128; 8],
fld5: (*mut Adt27,),
fld6: *mut ([i32; 7], u64),

},
Variant2{
fld0: i32,
fld1: [i128; 1],
fld2: *mut ([i32; 7], u64),

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt77{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt77 {
fld0: [i32; 2],
fld1: [i128; 8],
}

