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
pub fn fn0(mut _1: u16,mut _2: u8,mut _3: isize,mut _4: u128,mut _5: usize,mut _6: u32,mut _7: i128) -> (u128,) {
mir! {
type RET = (u128,);
let _8: bool;
let _9: f32;
let _10: usize;
let _11: [i32; 3];
let _12: Adt34;
let _13: Adt81;
let _14: u128;
let _15: i128;
let _16: isize;
let _17: i128;
let _18: i128;
let _19: i128;
let _20: [u64; 8];
let _21: u16;
let _22: &'static &'static *const *const usize;
let _23: i32;
let _24: Adt43;
let _25: isize;
let _26: [i8; 2];
let _27: (u128, i8, *const usize);
let _28: bool;
let _29: (u16, char, u16, [u64; 8]);
let _30: u16;
let _31: u64;
let _32: u8;
let _33: (isize, i32, i64, i32);
let _34: Adt81;
let _35: Adt34;
let _36: Adt47;
let _37: *mut ((u16, char, u16, [u64; 8]), Adt43, Adt34, ((u8, &'static f32),));
let _38: char;
let _39: bool;
let _40: ();
let _41: ();
{
RET = (285625659966477668254603950611444182367_u128,);
_4 = (-84635686420854030046838824108913979956_i128) as u128;
_7 = 64981924922241733333513289660614665593_i128 - 114947257486441369207412169010054367583_i128;
_2 = 161_u8;
_3 = -(-9223372036854775808_isize);
_6 = 643206212_u32 >> _2;
_4 = RET.0 - RET.0;
_7 = 112042916205018303440086645641609035462_i128;
RET.0 = _4 + _4;
_8 = true;
_1 = '\u{b53da}' as u16;
_1 = 22434_u16;
_4 = !RET.0;
Goto(bb1)
}
bb1 = {
_10 = !2_usize;
_5 = !_10;
_5 = _10;
_5 = (-74_i8) as usize;
RET = (_4,);
_11 = [2114513438_i32,(-1094678915_i32),1055376659_i32];
_9 = 819829505_i32 as f32;
_8 = RET.0 > _4;
_7 = 3603861451538187610_u64 as i128;
RET = (_4,);
RET.0 = !_4;
RET = (_4,);
_12 = Adt34::Variant0 { fld0: _3 };
_9 = 29870_i16 as f32;
_9 = Field::<isize>(Variant(_12, 0), 0) as f32;
_11 = [1692968782_i32,(-707887076_i32),570234564_i32];
RET = (_4,);
_6 = 664324804_u32;
_5 = _10 & _10;
_12 = Adt34::Variant0 { fld0: _3 };
_3 = _5 as isize;
_9 = 55_i8 as f32;
match _1 {
22434 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_7 = 51_i8 as i128;
_10 = !_5;
_8 = !true;
place!(Field::<isize>(Variant(_12, 0), 0)) = _3 ^ _3;
SetDiscriminant(_12, 1);
RET.0 = _8 as u128;
_7 = (-62365442199829485702555806028964591869_i128);
_2 = _10 as u8;
RET.0 = _4;
_12 = Adt34::Variant0 { fld0: _3 };
_8 = false;
RET = (_4,);
_11 = [163565750_i32,870633521_i32,1354171021_i32];
place!(Field::<isize>(Variant(_12, 0), 0)) = _3;
RET.0 = !_4;
RET.0 = _7 as u128;
_7 = !157895529391442465847022100119806738503_i128;
_11 = [1935777372_i32,(-476619836_i32),(-1188676653_i32)];
place!(Field::<isize>(Variant(_12, 0), 0)) = _7 as isize;
_3 = -Field::<isize>(Variant(_12, 0), 0);
_1 = 57819_u16 >> _4;
_6 = _2 as u32;
Call(_12 = fn1(_1, _7, _9, _7, _2, _1, _1, _5, _9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = [(-1333595941_i32),1625973050_i32,1880432338_i32];
RET.0 = !_4;
RET = (_4,);
SetDiscriminant(_12, 0);
_5 = 341_i16 as usize;
RET = (_4,);
_4 = !RET.0;
_4 = !RET.0;
RET = (_4,);
_15 = _7;
_3 = (-9223372036854775808_isize);
RET.0 = !_4;
_7 = !_15;
_6 = !74156712_u32;
_2 = 42_u8 >> _1;
_3 = 8248122552519908747_i64 as isize;
_4 = !RET.0;
_9 = 14263423253267780320_u64 as f32;
RET.0 = !_4;
_5 = _10 ^ _10;
RET.0 = !_4;
_5 = _9 as usize;
RET = (_4,);
Call(_12 = fn2(RET.0, _2, _11, _2, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = _15 << _4;
_1 = Field::<i32>(Variant(_12, 1), 1) as u16;
_5 = _10;
_4 = RET.0;
_21 = Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0).0 ^ Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0).0;
_17 = -_18;
_5 = _10;
_18 = _17 - _17;
_20 = [18089047824238166153_u64,16998072785615893772_u64,7907087739742746800_u64,7639116577704767119_u64,5566321965414300113_u64,6830468822858959398_u64,8023834056485904170_u64,13728686525234378904_u64];
RET.0 = _4 - _4;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).3 = [15985237351510945650_u64,107372060959113693_u64,18359119558267227943_u64,7805896023894265143_u64,9031659783566658718_u64,10777150846585027121_u64,17302285457824507046_u64,6912311171655843417_u64];
_18 = _17 | _17;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).1 = '\u{31dcb}';
_14 = RET.0 << Field::<i32>(Variant(_12, 1), 1);
SetDiscriminant(_12, 1);
_6 = 20316344_u32 ^ 2414966141_u32;
_5 = _10;
_23 = 79055972_i32 * 754787216_i32;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).3 = _20;
RET.0 = _14;
Goto(bb6)
}
bb6 = {
_11 = [_23,_23,_23];
_21 = _1;
RET.0 = _14;
_3 = (-9223372036854775808_isize) | 1_isize;
_16 = !_3;
_16 = _3;
_21 = !_1;
_2 = 4_u8 - 117_u8;
_25 = (-8437894666103440796_i64) as isize;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).2 = _1;
place!(Field::<i32>(Variant(_12, 1), 1)) = _14 as i32;
_25 = !_3;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).0 = !_21;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).0 = _1 >> _3;
_5 = _10 >> RET.0;
_20 = [5828113326727105058_u64,12404370310754284991_u64,821368208028319159_u64,17700657067418055317_u64,11017968167173949940_u64,14991523726400013072_u64,13563145400978814790_u64,13867045270023308229_u64];
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).1 = '\u{93338}';
_16 = !_25;
_27.1 = 96_i8 ^ (-15_i8);
Call(_17 = fn4(_12, Field::<i32>(Variant(_12, 1), 1), RET, Field::<i32>(Variant(_12, 1), 1), RET.0, _18, _20), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = Field::<i32>(Variant(_12, 1), 1) as u128;
_19 = _18;
_27.2 = core::ptr::addr_of!(_10);
_9 = _16 as f32;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)) = (_21, '\u{89f87}', _1, _20);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0)).0 = Field::<(u16, char, u16, [u64; 8])>(Variant(_12, 1), 0).2;
_10 = !_5;
_28 = RET.0 > RET.0;
_12 = Adt34::Variant0 { fld0: _25 };
_27.0 = !RET.0;
_1 = _21 ^ _21;
Goto(bb8)
}
bb8 = {
place!(Field::<isize>(Variant(_12, 0), 0)) = !_25;
_29.3 = [7680739406968313249_u64,10063743532111632814_u64,11616257484006433567_u64,8409909227916129897_u64,16611355083037539957_u64,11463858333847194047_u64,18039628953032243543_u64,16152533969674259859_u64];
_17 = -_15;
_11 = [_23,_23,_23];
_33.0 = _19 as isize;
_14 = _27.0;
_4 = _27.0 ^ _27.0;
_33.1 = _23 * _23;
_31 = '\u{7b224}' as u64;
SetDiscriminant(_12, 0);
_33 = (_25, _23, (-5751364291813898966_i64), _23);
_12 = Adt34::Variant0 { fld0: _3 };
RET.0 = _10 as u128;
_26 = [_27.1,_27.1];
_3 = !Field::<isize>(Variant(_12, 0), 0);
_29.0 = _31 as u16;
Goto(bb9)
}
bb9 = {
Call(_40 = dump_var(0_usize, 20_usize, Move(_20), 19_usize, Move(_19), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_40 = dump_var(0_usize, 21_usize, Move(_21), 6_usize, Move(_6), 23_usize, Move(_23), 28_usize, Move(_28)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_40 = dump_var(0_usize, 7_usize, Move(_7), 5_usize, Move(_5), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u16,mut _2: i128,mut _3: f32,mut _4: i128,mut _5: u8,mut _6: u16,mut _7: u16,mut _8: usize,mut _9: f32) -> Adt34 {
mir! {
type RET = Adt34;
let _10: *mut ((u16, char, u16, [u64; 8]), Adt43, Adt34, ((u8, &'static f32),));
let _11: *const u64;
let _12: u128;
let _13: isize;
let _14: f64;
let _15: f64;
let _16: f32;
let _17: f32;
let _18: [isize; 3];
let _19: &'static i16;
let _20: char;
let _21: *mut &'static f32;
let _22: &'static (i128, u128, ((u16, char, u16, [u64; 8]),), char);
let _23: Adt35;
let _24: isize;
let _25: *mut *mut usize;
let _26: &'static i16;
let _27: (Adt51, i8, [i8; 2], *mut isize);
let _28: [i32; 1];
let _29: bool;
let _30: (isize, i32, i64, i32);
let _31: ();
let _32: ();
{
_1 = !_6;
_5 = 249_u8 + 79_u8;
_2 = !_4;
RET = Adt34::Variant0 { fld0: (-9223372036854775808_isize) };
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize;
_7 = _2 as u16;
_2 = !_4;
_1 = _5 as u16;
RET = Adt34::Variant0 { fld0: 9223372036854775807_isize };
_1 = _6;
_13 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_14 = 1901605025_i32 as f64;
_8 = 971524829532644630_usize * 5_usize;
_14 = _9 as f64;
_14 = (-74_i8) as f64;
_12 = 193210828551065621072350369874901641045_u128;
_5 = 135_u8 | 214_u8;
RET = Adt34::Variant0 { fld0: _13 };
_2 = _4;
Goto(bb1)
}
bb1 = {
_4 = _2 - _2;
_3 = (-20603_i16) as f32;
place!(Field::<isize>(Variant(RET, 0), 0)) = _13;
_8 = 11140566805454733245_usize >> _12;
_2 = -_4;
_6 = _8 as u16;
_8 = 1011535966054039596_usize - 12708827236809786877_usize;
_8 = 2680048117539311039_usize;
_13 = Field::<isize>(Variant(RET, 0), 0) >> _6;
_18 = [_13,_13,_13];
RET = Adt34::Variant0 { fld0: _13 };
_6 = _5 as u16;
_4 = _12 as i128;
_8 = 25_i8 as usize;
_2 = _4 >> _1;
_14 = (-2700337340669502887_i64) as f64;
_13 = Field::<isize>(Variant(RET, 0), 0) | Field::<isize>(Variant(RET, 0), 0);
_13 = Field::<isize>(Variant(RET, 0), 0) | Field::<isize>(Variant(RET, 0), 0);
place!(Field::<isize>(Variant(RET, 0), 0)) = _13 << _12;
_5 = 223_u8 - 23_u8;
_7 = _9 as u16;
_8 = _13 as usize;
_17 = -_9;
_8 = 1_usize;
place!(Field::<isize>(Variant(RET, 0), 0)) = !_13;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
193210828551065621072350369874901641045 => bb9,
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
_18 = [_13,Field::<isize>(Variant(RET, 0), 0),_13];
_9 = _14 as f32;
_16 = _3;
_15 = _18[_8] as f64;
_1 = !_6;
_20 = '\u{e07ec}';
match _8 {
0 => bb4,
2 => bb5,
3 => bb10,
1 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_2 = _20 as i128;
_16 = _13 as f32;
_23.fld0 = _2 as usize;
_9 = _16 * _16;
place!(Field::<isize>(Variant(RET, 0), 0)) = !_18[_8];
place!(Field::<isize>(Variant(RET, 0), 0)) = _18[_8];
_18 = [_13,Field::<isize>(Variant(RET, 0), 0),Field::<isize>(Variant(RET, 0), 0)];
_14 = _15;
_9 = _16;
_23.fld1 = core::ptr::addr_of_mut!(_8);
_12 = !135321762030698099194401399002998327712_u128;
_14 = _15;
Call(_4 = core::intrinsics::transmute(_12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_24 = !_13;
_12 = _2 as u128;
place!(Field::<isize>(Variant(RET, 0), 0)) = -_24;
_23.fld0 = _15 as usize;
_6 = (-20593_i16) as u16;
_12 = false as u128;
_25 = core::ptr::addr_of_mut!(_23.fld1);
_15 = _14 + _14;
_17 = _9;
_27.3 = core::ptr::addr_of_mut!(_18[_8]);
_27.1 = 36_i8 & (-95_i8);
_13 = _7 as isize;
place!(Field::<isize>(Variant(RET, 0), 0)) = !_18[_8];
_9 = _3;
Goto(bb14)
}
bb14 = {
_24 = _18[_8] + _18[_8];
_7 = _1;
_27.2 = [_27.1,_27.1];
RET = Adt34::Variant0 { fld0: _24 };
_27.2 = [_27.1,_27.1];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 7_usize, Move(_7), 4_usize, Move(_4), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 24_usize, Move(_24), 13_usize, Move(_13), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u128,mut _2: u8,mut _3: [i32; 3],mut _4: u8,mut _5: u16) -> Adt34 {
mir! {
type RET = Adt34;
let _6: (u16, char, u16, [u64; 8]);
let _7: &'static (u128, i8, *const usize);
let _8: f64;
let _9: *mut *mut f32;
let _10: Adt81;
let _11: (u128,);
let _12: *mut *mut (isize, i32, i64, i32);
let _13: ();
let _14: ();
{
_5 = 31732_u16;
RET = Adt34::Variant0 { fld0: (-118_isize) };
_3 = [1445548355_i32,1003654986_i32,(-1159923265_i32)];
_6.1 = '\u{ae657}';
_6.0 = !_5;
_6.3 = [11819648494628451559_u64,10705930554577214474_u64,17109737428135611239_u64,17143595857671852018_u64,14436414368917873202_u64,2862957044676159640_u64,14396902486204499213_u64,3743085633575241991_u64];
_6.3 = [5605633845862712915_u64,15735297804862229367_u64,5103396717193300547_u64,13266687448352966610_u64,2838801773374582056_u64,14039647289057881137_u64,7119251694831933089_u64,12316806869820950580_u64];
_6.2 = _2 as u16;
_6.2 = (-6447257155932585522_i64) as u16;
RET = Adt34::Variant0 { fld0: (-9223372036854775808_isize) };
_3 = [(-30620507_i32),131651944_i32,(-1541267222_i32)];
RET = Adt34::Variant0 { fld0: (-9223372036854775808_isize) };
_5 = _6.0;
place!(Field::<isize>(Variant(RET, 0), 0)) = _6.1 as isize;
RET = Adt34::Variant0 { fld0: (-26_isize) };
RET = Adt34::Variant1 { fld0: _6,fld1: 371660935_i32 };
_4 = 2236741061272617547_i64 as u8;
_6 = (Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).2, Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).1, Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0, Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).3);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).3 = [16065370585693863195_u64,13716660831535183537_u64,10340380301560663623_u64,370309494213828804_u64,15107957166293324942_u64,5697808345481247125_u64,2129849931040123964_u64,16991127258122614656_u64];
_11 = (_1,);
_6.2 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0;
_11 = (_1,);
_5 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).2 ^ Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0;
Call(RET = fn3(_11.0, _6, _2, _2, _1, _6.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _6.1;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).0 = !Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).2;
_5 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).2;
_6.3 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).3;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _6.1;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)) = _6;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(2_usize, 4_usize, Move(_4), 3_usize, Move(_3), 6_usize, Move(_6), 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u128,mut _2: (u16, char, u16, [u64; 8]),mut _3: u8,mut _4: u8,mut _5: u128,mut _6: [u64; 8]) -> Adt34 {
mir! {
type RET = Adt34;
let _7: [i8; 1];
let _8: isize;
let _9: &'static [i8; 2];
let _10: (u8, &'static f32);
let _11: u128;
let _12: isize;
let _13: Adt42;
let _14: [isize; 3];
let _15: (u16, char, u16, [u64; 8]);
let _16: bool;
let _17: [i8; 2];
let _18: ();
let _19: ();
{
_2.1 = '\u{2f0a}';
_2.1 = '\u{55f4e}';
RET = Adt34::Variant0 { fld0: 26_isize };
place!(Field::<isize>(Variant(RET, 0), 0)) = (-785295380_i32) as isize;
_2.3 = [7580827177848608051_u64,5319813652311660954_u64,8038647983080604725_u64,10850497502863840139_u64,16301422778272283373_u64,1047831645846868730_u64,6309072439072060420_u64,17167781270776242280_u64];
_2.3 = [1307435525420122297_u64,15690984524344883434_u64,11570065069506892648_u64,6609621530709115783_u64,13300723266311605288_u64,15239106323417826176_u64,12934439755316989966_u64,8786000937740679909_u64];
place!(Field::<isize>(Variant(RET, 0), 0)) = -9223372036854775807_isize;
_7 = [(-20_i8)];
_2 = (9573_u16, '\u{89b48}', 21667_u16, _6);
_8 = Field::<isize>(Variant(RET, 0), 0);
place!(Field::<isize>(Variant(RET, 0), 0)) = _8 ^ _8;
_5 = 2602022313_u32 as u128;
RET = Adt34::Variant1 { fld0: _2,fld1: 1353277935_i32 };
_4 = 397730121_i32 as u8;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).2 = !_2.0;
RET = Adt34::Variant1 { fld0: _2,fld1: 469583061_i32 };
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)) = (_2.2, _2.1, _2.0, _6);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _2.1;
Goto(bb1)
}
bb1 = {
_1 = !_5;
_11 = _1 & _5;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _2.1;
place!(Field::<i32>(Variant(RET, 1), 1)) = (-552963947_i32);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).0 = !_2.2;
_2.0 = (-25684_i16) as u16;
place!(Field::<i32>(Variant(RET, 1), 1)) = (-122064540_i32);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _2.1;
_12 = -_8;
_11 = _5;
_10.0 = _3 ^ _3;
_2.0 = _2.2;
_8 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).1 as isize;
_15 = (_2.0, Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).1, Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0, _6);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).0 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).2;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).1 = _2.1;
_2.3 = _15.3;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)).3 = [251944969927955478_u64,11259609296394951276_u64,10628833554898929863_u64,13237271112355482270_u64,9481018710974174477_u64,11614883602745019227_u64,16276154156360101124_u64,10204129398927030823_u64];
_2.0 = Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0 << Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0).0;
_11 = _5 - _1;
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(RET, 1), 0)) = (_15.0, _15.1, _2.0, _15.3);
_1 = _5;
_7 = [53_i8];
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(3_usize, 7_usize, Move(_7), 15_usize, Move(_15), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(3_usize, 2_usize, Move(_2), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: Adt34,mut _2: i32,mut _3: (u128,),mut _4: i32,mut _5: u128,mut _6: i128,mut _7: [u64; 8]) -> i128 {
mir! {
type RET = i128;
let _8: f32;
let _9: f32;
let _10: ();
let _11: ();
{
_3.0 = !_5;
_3.0 = _5;
_7 = [13590139677751492_u64,14161807324523139503_u64,6785071258285441693_u64,8570430141628619970_u64,8106420136105932121_u64,558784242511221099_u64,8600860886334741848_u64,17527951110838153730_u64];
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_1, 1), 0)).2 = Field::<(u16, char, u16, [u64; 8])>(Variant(_1, 1), 0).0;
_3 = (_5,);
_3 = (_5,);
RET = _6 * _6;
_4 = Field::<i32>(Variant(_1, 1), 1);
_6 = -RET;
_4 = _2 | Field::<i32>(Variant(_1, 1), 1);
place!(Field::<(u16, char, u16, [u64; 8])>(Variant(_1, 1), 0)).1 = '\u{423b1}';
place!(Field::<i32>(Variant(_1, 1), 1)) = 7817489600466968041_i64 as i32;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(4_usize, 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4), 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(62872_u16), std::hint::black_box(170_u8), std::hint::black_box(60_isize), std::hint::black_box(322150575297747341311998257102924657022_u128), std::hint::black_box(0_usize), std::hint::black_box(1221837498_u32), std::hint::black_box((-128655381471126540328936355540356526638_i128)));
                
            }
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: isize,

},
Variant1{
fld0: (u16, char, u16, [u64; 8]),
fld1: i32,

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt35{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt35 {
fld0: usize,
fld1: *mut usize,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: u128,
fld2: isize,
fld3: i64,
fld4: *mut [u128; 2],

},
Variant1{
fld0: (u128,),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: Adt42,
fld1: char,
fld2: [i32; 3],

},
Variant1{
fld0: isize,
fld1: *mut usize,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut f32,

},
Variant1{
fld0: bool,
fld1: *mut [u128; 2],
fld2: *mut isize,
fld3: [i16; 4],
fld4: (*mut isize, [u128; 2], *const usize),

},
Variant2{
fld0: bool,
fld1: f32,
fld2: isize,
fld3: [bool; 7],
fld4: *const usize,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt47,
fld1: [i16; 4],
fld2: u64,
fld3: usize,
fld4: [i8; 1],
fld5: (*mut isize, [u128; 2], *const usize),
fld6: (isize, i32, i64, i32),

},
Variant1{
fld0: Adt42,
fld1: *mut (isize, i32, i64, i32),
fld2: *mut usize,

},
Variant2{
fld0: u128,
fld1: [u64; 8],
fld2: [u128; 2],
fld3: [i32; 3],
fld4: *mut (isize, i32, i64, i32),
fld5: *const f32,
fld6: *const usize,
fld7: *const isize,

},
Variant3{
fld0: f32,
fld1: (*mut isize, [u128; 2], *const usize),

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: [u64; 3],
fld1: u128,
fld2: u16,
fld3: i8,
fld4: (i128, u128, ((u16, char, u16, [u64; 8]),), char),

},
Variant1{
fld0: *mut f32,
fld1: char,
fld2: *const isize,
fld3: i8,
fld4: *mut (isize, i32, i64, i32),
fld5: *const usize,
fld6: i64,
fld7: (*const *const usize, Adt34),

},
Variant2{
fld0: *const f32,
fld1: char,
fld2: [i32; 1],
fld3: u128,
fld4: Adt34,
fld5: (u128, i8, *const usize),
fld6: *mut usize,

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: f32,
fld1: [u128; 2],
fld2: Adt34,
fld3: [isize; 4],
fld4: i16,
fld5: *const u64,
fld6: u16,

},
Variant1{
fld0: i32,
fld1: char,
fld2: [u128; 2],

}}

