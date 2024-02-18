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
pub fn fn0(mut _1: u32,mut _2: u16,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize) -> isize {
mir! {
type RET = isize;
let _10: *mut *const *const i64;
let _11: ([i64; 8], [i128; 7], (u32,));
let _12: f64;
let _13: usize;
let _14: u16;
let _15: f32;
let _16: i32;
let _17: bool;
let _18: i64;
let _19: [usize; 8];
let _20: ();
let _21: ();
{
_6 = false as i32;
RET = -9223372036854775807_isize;
_5 = (-8038_i16) << _6;
_3 = RET;
_4 = !16_i8;
_4 = -90_i8;
_11.1 = [(-52028307375502268443645981314514151974_i128),(-107576625745514859842827674025072689299_i128),47612777769252633481561088544611577931_i128,(-16304054884162060837877396699816201352_i128),(-96955283144676264628842980947131324837_i128),108157324027985472920461008546476937866_i128,(-155331722055446567227297420162054545996_i128)];
_8 = 142801038324317444770474088933505541557_i128;
_12 = 288337291599708441420153731774895751223_u128 as f64;
RET = false as isize;
_11.2 = (2742318084_u32,);
_5 = (-27897_i16) * (-22870_i16);
_5 = !(-11166_i16);
_6 = -199267002_i32;
_11.1 = [_8,_8,_8,_8,_8,_8,_8];
_11.0 = [(-7843767703150286740_i64),1042532980788990252_i64,(-3180432870449288521_i64),4442602500113589072_i64,4481430377574051238_i64,(-7050310975280416010_i64),(-8120908841408345935_i64),5039002891088856250_i64];
_9 = !57008125647490150_usize;
RET = _3 + _3;
_9 = !9403051526134710837_usize;
_3 = -RET;
_6 = !(-870197069_i32);
_2 = !29440_u16;
_5 = -(-2598_i16);
_8 = (-65315480883782242725748933875440392176_i128);
Call(_9 = fn1(_11.0, _12, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 3060147635114789756_i64;
_12 = _4 as f64;
_1 = _11.2.0 << _8;
_3 = -RET;
_9 = 1_usize + 4_usize;
_11.2 = (_1,);
RET = _3;
_13 = false as usize;
_8 = 84781479710727948411817757277952290547_i128;
_7 = !(-4096751683733133286_i64);
_1 = !_11.2.0;
_11.0 = [_7,_7,_7,_7,_7,_7,_7,_7];
_4 = (-114_i8);
_11.2.0 = _1;
_13 = _9 ^ _9;
_7 = 2323409214021793889_i64;
Goto(bb2)
}
bb2 = {
_12 = RET as f64;
_11.1 = [_8,_8,_8,_8,_8,_8,_8];
_7 = (-2906361069224523503_i64) & 2635148084868188037_i64;
_8 = _4 as i128;
_15 = _5 as f32;
_6 = !(-1466406361_i32);
_12 = RET as f64;
_1 = _11.2.0;
_2 = !10958_u16;
RET = 64210358287612463889137228653913353142_u128 as isize;
_7 = -947627274555800790_i64;
_7 = 2406823913808121991_i64;
_9 = 94_u8 as usize;
RET = !_3;
_14 = !_2;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211342 => bb7,
_ => bb6
}
}
bb3 = {
_7 = 3060147635114789756_i64;
_12 = _4 as f64;
_1 = _11.2.0 << _8;
_3 = -RET;
_9 = 1_usize + 4_usize;
_11.2 = (_1,);
RET = _3;
_13 = false as usize;
_8 = 84781479710727948411817757277952290547_i128;
_7 = !(-4096751683733133286_i64);
_1 = !_11.2.0;
_11.0 = [_7,_7,_7,_7,_7,_7,_7,_7];
_4 = (-114_i8);
_11.2.0 = _1;
_13 = _9 ^ _9;
_7 = 2323409214021793889_i64;
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
_11.0 = [_7,_7,_7,_7,_7,_7,_7,_7];
_11.2 = (_1,);
_15 = 205_u8 as f32;
_15 = 93183894903961364016662889053261900019_u128 as f32;
_13 = _15 as usize;
RET = _3 << _11.2.0;
_2 = _14;
_8 = !141663022304088134818821644069203799605_i128;
_11.2 = (_1,);
_1 = _11.2.0;
_16 = _6 | _6;
_2 = _14;
_13 = _9;
RET = _3 | _3;
_3 = -RET;
_6 = _9 as i32;
_5 = (-20768_i16) - (-17993_i16);
_12 = _14 as f64;
_13 = !_9;
_13 = _9 | _9;
_11.2 = (_1,);
_7 = _3 as i64;
_11.0 = [_7,_7,_7,_7,_7,_7,_7,_7];
RET = _3 ^ _3;
_3 = RET << RET;
RET = -_3;
_11.2.0 = _1 - _1;
Goto(bb8)
}
bb8 = {
Call(_20 = dump_var(0_usize, 9_usize, Move(_9), 13_usize, Move(_13), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_20 = dump_var(0_usize, 11_usize, Move(_11), 1_usize, Move(_1), 21_usize, _21, 21_usize, _21), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i64; 8],mut _2: f64,mut _3: isize) -> usize {
mir! {
type RET = usize;
let _4: f64;
let _5: *const f64;
let _6: isize;
let _7: (u128, Adt42, Adt46, i64);
let _8: f32;
let _9: *const f64;
let _10: u16;
let _11: ([bool; 7], [i16; 7], [u8; 7], *mut *const i64);
let _12: (i64, *const i64, *const i64, i16);
let _13: u32;
let _14: char;
let _15: bool;
let _16: usize;
let _17: f32;
let _18: &'static &'static (u32,);
let _19: u8;
let _20: &'static &'static *mut i16;
let _21: char;
let _22: u128;
let _23: Adt58;
let _24: Adt42;
let _25: *const &'static isize;
let _26: ();
let _27: ();
{
RET = !15853536064371059794_usize;
_4 = _2;
_1 = [3527990340596919828_i64,4408928840928527547_i64,(-940278696856796292_i64),53259235428381555_i64,(-3110198636193204465_i64),2940632637890363614_i64,2706604432718717212_i64,(-8568204048183793960_i64)];
_1 = [(-7999860953146770030_i64),6430689120828911931_i64,2537625189295349896_i64,2064187704637635368_i64,(-727672036961883436_i64),1918198998517712694_i64,3335851120105299126_i64,4133836025070494443_i64];
RET = !6219394914914358243_usize;
_2 = _4 + _4;
RET = 198_u8 as usize;
_3 = !(-9223372036854775808_isize);
_3 = -(-94_isize);
RET = 1616694480969916618_usize ^ 4_usize;
RET = 14813186541291515515_usize;
_5 = core::ptr::addr_of!(_4);
_6 = _3;
_4 = -_2;
_7.2 = Adt46::Variant1 { fld0: (-119_i8) };
_4 = _2 * _2;
_7.0 = 173512623795879493445472733855164933838_u128 | 118054065658672857466330977072685996891_u128;
_7.2 = Adt46::Variant1 { fld0: 124_i8 };
place!(Field::<i8>(Variant(_7.2, 1), 0)) = 13_i8 * 77_i8;
_7.3 = -(-4482102768697225552_i64);
_1 = [_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3];
(*_5) = _2;
_5 = core::ptr::addr_of!((*_5));
_8 = (-20817301785108901073885008378959489210_i128) as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
14813186541291515515 => bb6,
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
_4 = _2;
(*_5) = _2 - _2;
_8 = (-10201086390356212619501733206226490575_i128) as f32;
RET = 4_usize;
RET = 4_usize;
_7.3 = 1843474550_u32 as i64;
(*_5) = -_2;
_2 = -(*_5);
_6 = _3;
_5 = core::ptr::addr_of!((*_5));
place!(Field::<i8>(Variant(_7.2, 1), 0)) = (-123_i8) & 78_i8;
_3 = _6 + _6;
_9 = core::ptr::addr_of!((*_5));
SetDiscriminant(_7.2, 1);
(*_9) = _3 as f64;
_1 = [_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3];
_1 = [_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3,_7.3];
_7.1 = Adt42::Variant2 { fld0: (-22527_i16),fld1: _7.3,fld2: 58974_u16 };
place!(Field::<i64>(Variant(_7.1, 2), 1)) = _7.3;
(*_5) = -_2;
_2 = 22267_u16 as f64;
_1 = [Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),_7.3,_7.3,_7.3,Field::<i64>(Variant(_7.1, 2), 1)];
(*_9) = _2 + _2;
_1[RET] = _7.3 | _7.3;
(*_5) = _2 - _2;
_3 = _6 | _6;
place!(Field::<i16>(Variant(_7.1, 2), 0)) = 25986_i16;
match Field::<i16>(Variant(_7.1, 2), 0) {
25986 => bb7,
_ => bb2
}
}
bb7 = {
(*_5) = _2 + _2;
_1 = [Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),_7.3,_7.3,_7.3,_7.3,_7.3,_7.3];
_4 = _2;
_1 = [Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),_7.3,Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1)];
_7.3 = RET as i64;
_1[RET] = _7.0 as i64;
_8 = Field::<i16>(Variant(_7.1, 2), 0) as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb5,
4 => bb8,
_ => bb4
}
}
bb8 = {
_11.2 = [59_u8,114_u8,51_u8,119_u8,71_u8,17_u8,225_u8];
_11.0[RET] = true | false;
_10 = 39224_u16 >> _1[RET];
_11.1 = [Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0)];
_11.0 = [false,true,false,false,false,true,false];
_12.0 = -_7.3;
_11.2 = [10_u8,97_u8,146_u8,138_u8,2_u8,163_u8,159_u8];
_3 = _6;
_10 = _11.1[RET] as u16;
(*_9) = _2 - _2;
(*_5) = -_2;
_14 = '\u{81be4}';
_3 = _6 - _6;
_12.3 = Field::<i16>(Variant(_7.1, 2), 0);
_5 = core::ptr::addr_of!((*_5));
_11.1[RET] = (-116_i8) as i16;
_1[RET] = _8 as i64;
_13 = !2680849693_u32;
_3 = (-70616519550564086840862980778208780863_i128) as isize;
_1[RET] = _12.0 << _12.0;
_11.2 = [222_u8,76_u8,101_u8,191_u8,71_u8,197_u8,199_u8];
place!(Field::<u16>(Variant(_7.1, 2), 2)) = _10;
_11.3 = core::ptr::addr_of_mut!(_12.2);
Call(_12 = fn2(_11.2, Move(_11), Move(_7.1), Move(_9), _6, RET, _14, _4, _3, _7.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = !RET;
_11.2 = [180_u8,13_u8,220_u8,183_u8,71_u8,27_u8,150_u8];
_7.0 = _4 as u128;
_11.0 = [false,false,false,true,false,false,false];
(*_5) = _2 + _2;
_7.2 = Adt46::Variant1 { fld0: 10_i8 };
_15 = !false;
_5 = core::ptr::addr_of!(_2);
_17 = -_8;
_11.0 = [_15,_15,_15,_15,_15,_15,_15];
_12.0 = _7.3;
(*_5) = _4;
_13 = 1861494390_u32 & 1633163487_u32;
_12.3 = (-26197_i16) >> RET;
_12.2 = Move(_12.1);
(*_5) = _7.0 as f64;
_10 = 43622_u16;
_21 = _14;
_14 = _21;
_12.1 = core::ptr::addr_of!(_12.0);
_9 = Move(_5);
match _10 {
0 => bb1,
1 => bb2,
2 => bb6,
43622 => bb10,
_ => bb4
}
}
bb10 = {
_5 = Move(_9);
_11.3 = core::ptr::addr_of_mut!(_12.1);
_22 = _7.0;
_19 = (-1771540411_i32) as u8;
_19 = _14 as u8;
_13 = _19 as u32;
_23.fld5 = Move(_12.1);
Goto(bb11)
}
bb11 = {
_13 = 2951612904_u32;
_23.fld0 = RET - _16;
_12.3 = _14 as i16;
_3 = !_6;
_21 = _14;
_23.fld3 = _13 >> _23.fld0;
_12 = (_7.3, Move(_23.fld5), Move(_23.fld5), 6646_i16);
_24 = Adt42::Variant2 { fld0: _12.3,fld1: _12.0,fld2: _10 };
_7.1 = Move(_24);
_1 = [Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1),_12.0,_7.3,_12.0,_7.3,Field::<i64>(Variant(_7.1, 2), 1),Field::<i64>(Variant(_7.1, 2), 1)];
_23.fld2 = _17;
_23.fld1 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_7.2, 1), 0)));
place!(Field::<i16>(Variant(_7.1, 2), 0)) = -_12.3;
_23.fld5 = core::ptr::addr_of!(_7.3);
_23.fld3 = Field::<u16>(Variant(_7.1, 2), 2) as u32;
_4 = _3 as f64;
_3 = _6 + _6;
SetDiscriminant(_7.1, 0);
place!(Field::<u8>(Variant(_7.1, 0), 5)) = !_19;
_9 = Move(_5);
RET = _12.0 as usize;
match _13 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb6,
4 => bb5,
5 => bb12,
6 => bb13,
2951612904 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_11.2 = [59_u8,114_u8,51_u8,119_u8,71_u8,17_u8,225_u8];
_11.0[RET] = true | false;
_10 = 39224_u16 >> _1[RET];
_11.1 = [Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0),Field::<i16>(Variant(_7.1, 2), 0)];
_11.0 = [false,true,false,false,false,true,false];
_12.0 = -_7.3;
_11.2 = [10_u8,97_u8,146_u8,138_u8,2_u8,163_u8,159_u8];
_3 = _6;
_10 = _11.1[RET] as u16;
(*_9) = _2 - _2;
(*_5) = -_2;
_14 = '\u{81be4}';
_3 = _6 - _6;
_12.3 = Field::<i16>(Variant(_7.1, 2), 0);
_5 = core::ptr::addr_of!((*_5));
_11.1[RET] = (-116_i8) as i16;
_1[RET] = _8 as i64;
_13 = !2680849693_u32;
_3 = (-70616519550564086840862980778208780863_i128) as isize;
_1[RET] = _12.0 << _12.0;
_11.2 = [222_u8,76_u8,101_u8,191_u8,71_u8,197_u8,199_u8];
place!(Field::<u16>(Variant(_7.1, 2), 2)) = _10;
_11.3 = core::ptr::addr_of_mut!(_12.2);
Call(_12 = fn2(_11.2, Move(_11), Move(_7.1), Move(_9), _6, RET, _14, _4, _3, _7.0), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_5 = Move(_9);
place!(Field::<u8>(Variant(_7.1, 0), 5)) = _19 | _19;
place!(Field::<*const i64>(Variant(_7.1, 0), 1)) = core::ptr::addr_of!(_12.0);
place!(Field::<([char; 8], char)>(Variant(_7.1, 0), 4)).1 = _14;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(1_usize, 13_usize, Move(_13), 3_usize, Move(_3), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(1_usize, 16_usize, Move(_16), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [u8; 7],mut _2: ([bool; 7], [i16; 7], [u8; 7], *mut *const i64),mut _3: Adt42,mut _4: *const f64,mut _5: isize,mut _6: usize,mut _7: char,mut _8: f64,mut _9: isize,mut _10: u128) -> (i64, *const i64, *const i64, i16) {
mir! {
type RET = (i64, *const i64, *const i64, i16);
let _11: u16;
let _12: bool;
let _13: f32;
let _14: isize;
let _15: &'static *const f64;
let _16: f32;
let _17: *mut *mut *const *const i64;
let _18: isize;
let _19: i32;
let _20: ([i64; 8], [i128; 7], (u32,));
let _21: i8;
let _22: *mut u8;
let _23: [bool; 7];
let _24: (char, u128);
let _25: i32;
let _26: *mut i16;
let _27: bool;
let _28: bool;
let _29: [usize; 8];
let _30: Adt63;
let _31: f32;
let _32: bool;
let _33: u128;
let _34: f32;
let _35: i16;
let _36: [bool; 3];
let _37: [u8; 7];
let _38: [char; 4];
let _39: ();
let _40: ();
{
place!(Field::<i64>(Variant(_3, 2), 1)) = !(-6547451674373316739_i64);
RET.2 = core::ptr::addr_of!(RET.0);
RET.1 = core::ptr::addr_of!(RET.0);
RET.2 = core::ptr::addr_of!(RET.0);
Call(place!(Field::<i64>(Variant(_3, 2), 1)) = fn3(_1, _7, _2.0[_6]), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 1897184675_i32 as f64;
_6 = !6049779527694957059_usize;
_13 = 1886808239_u32 as f32;
_1 = [156_u8,185_u8,18_u8,58_u8,197_u8,21_u8,3_u8];
_13 = 15_i8 as f32;
_13 = Field::<i16>(Variant(_3, 2), 0) as f32;
_2.2 = _1;
RET.3 = 4283435381_u32 as i16;
_11 = !Field::<u16>(Variant(_3, 2), 2);
RET.2 = core::ptr::addr_of!(RET.0);
Goto(bb2)
}
bb2 = {
RET.1 = core::ptr::addr_of!(RET.0);
RET.2 = core::ptr::addr_of!(RET.0);
_2.3 = core::ptr::addr_of_mut!(RET.1);
_8 = 113_u8 as f64;
RET.3 = -Field::<i16>(Variant(_3, 2), 0);
_14 = _9;
SetDiscriminant(_3, 2);
RET.3 = _8 as i16;
place!(Field::<u16>(Variant(_3, 2), 2)) = _11 - _11;
_12 = true & true;
RET.2 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_3, 2), 1)));
_6 = 6_usize | 1_usize;
_2.2 = [15_u8,139_u8,133_u8,68_u8,87_u8,243_u8,72_u8];
_9 = -_14;
_9 = _5;
RET.0 = (-6002823105929429754_i64) - (-170900695562136582_i64);
RET.3 = (-7636_i16) ^ (-30233_i16);
_2.2 = [151_u8,22_u8,95_u8,224_u8,77_u8,65_u8,51_u8];
Goto(bb3)
}
bb3 = {
_12 = !false;
_6 = 0_usize;
RET.0 = 5326826559589250160_i64;
place!(Field::<i64>(Variant(_3, 2), 1)) = 747480108_u32 as i64;
_2.2[_6] = _1[_6] | _1[_6];
place!(Field::<i64>(Variant(_3, 2), 1)) = (-12_i8) as i64;
_2.1[_6] = RET.3;
RET.0 = Field::<i64>(Variant(_3, 2), 1);
_19 = 1448552418_i32 | 281739857_i32;
_15 = &_4;
_2.1[_6] = RET.3 + RET.3;
_14 = _5;
_16 = -_13;
RET.2 = Move(RET.1);
RET.0 = _19 as i64;
_10 = 155787682675541515059396023202071162913_u128 + 309686539055035003972091427682412164139_u128;
place!(Field::<i16>(Variant(_3, 2), 0)) = !_2.1[_6];
Goto(bb4)
}
bb4 = {
_13 = _19 as f32;
_19 = -1131828784_i32;
RET.3 = !_2.1[_6];
RET.0 = Field::<i64>(Variant(_3, 2), 1);
_2.2[_6] = _1[_6] % _1[_6];
match _1[_6] {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
156 => bb10,
_ => bb9
}
}
bb5 = {
_12 = !false;
_6 = 0_usize;
RET.0 = 5326826559589250160_i64;
place!(Field::<i64>(Variant(_3, 2), 1)) = 747480108_u32 as i64;
_2.2[_6] = _1[_6] | _1[_6];
place!(Field::<i64>(Variant(_3, 2), 1)) = (-12_i8) as i64;
_2.1[_6] = RET.3;
RET.0 = Field::<i64>(Variant(_3, 2), 1);
_19 = 1448552418_i32 | 281739857_i32;
_15 = &_4;
_2.1[_6] = RET.3 + RET.3;
_14 = _5;
_16 = -_13;
RET.2 = Move(RET.1);
RET.0 = _19 as i64;
_10 = 155787682675541515059396023202071162913_u128 + 309686539055035003972091427682412164139_u128;
place!(Field::<i16>(Variant(_3, 2), 0)) = !_2.1[_6];
Goto(bb4)
}
bb6 = {
RET.1 = core::ptr::addr_of!(RET.0);
RET.2 = core::ptr::addr_of!(RET.0);
_2.3 = core::ptr::addr_of_mut!(RET.1);
_8 = 113_u8 as f64;
RET.3 = -Field::<i16>(Variant(_3, 2), 0);
_14 = _9;
SetDiscriminant(_3, 2);
RET.3 = _8 as i16;
place!(Field::<u16>(Variant(_3, 2), 2)) = _11 - _11;
_12 = true & true;
RET.2 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_3, 2), 1)));
_6 = 6_usize | 1_usize;
_2.2 = [15_u8,139_u8,133_u8,68_u8,87_u8,243_u8,72_u8];
_9 = -_14;
_9 = _5;
RET.0 = (-6002823105929429754_i64) - (-170900695562136582_i64);
RET.3 = (-7636_i16) ^ (-30233_i16);
_2.2 = [151_u8,22_u8,95_u8,224_u8,77_u8,65_u8,51_u8];
Goto(bb3)
}
bb7 = {
_8 = 1897184675_i32 as f64;
_6 = !6049779527694957059_usize;
_13 = 1886808239_u32 as f32;
_1 = [156_u8,185_u8,18_u8,58_u8,197_u8,21_u8,3_u8];
_13 = 15_i8 as f32;
_13 = Field::<i16>(Variant(_3, 2), 0) as f32;
_2.2 = _1;
RET.3 = 4283435381_u32 as i16;
_11 = !Field::<u16>(Variant(_3, 2), 2);
RET.2 = core::ptr::addr_of!(RET.0);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_2.1 = [Field::<i16>(Variant(_3, 2), 0),RET.3,RET.3,RET.3,RET.3,RET.3,Field::<i16>(Variant(_3, 2), 0)];
_5 = _9 * _14;
RET.2 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_3, 2), 1)));
_20.2.0 = _19 as u32;
_9 = !_5;
_2.0 = [_12,_12,_12,_12,_12,_12,_12];
_2.1 = [Field::<i16>(Variant(_3, 2), 0),Field::<i16>(Variant(_3, 2), 0),RET.3,Field::<i16>(Variant(_3, 2), 0),Field::<i16>(Variant(_3, 2), 0),Field::<i16>(Variant(_3, 2), 0),RET.3];
_20.2 = (8183534_u32,);
_21 = _19 as i8;
_18 = _5;
RET.1 = Move(RET.2);
_18 = !_14;
_20.1[_6] = 110110921392034341114444281008338192305_i128;
_5 = _18 << _20.1[_6];
_2.0 = [_12,_12,_12,_12,_12,_12,_12];
_20.1[_6] = 56790881558658612413794213310915220021_i128 + (-168631012327057338376152643873578307134_i128);
RET.2 = Move(RET.1);
_5 = _9;
_18 = _9 * _9;
RET.2 = core::ptr::addr_of!(_20.0[_6]);
_2.0 = [_12,_12,_12,_12,_12,_12,_12];
_20.1[_6] = 75202817282347048370299472249740053413_i128 >> _5;
Call(_6 = fn18(Move(_15), _8, Move(_3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_21 = (-55_i8) ^ 38_i8;
_24.0 = _7;
RET.2 = core::ptr::addr_of!(RET.0);
_20.1 = [(-13543194955898522800380566530086877207_i128),(-113862015075532257768467492809593855415_i128),(-143646637919604732926977483312768885261_i128),(-101624620118250767417434521064478507628_i128),(-45034828199081938674676620691564549860_i128),169481289809296712649636018542592653908_i128,55971816460492813165554891912359777277_i128];
_14 = _18;
_26 = core::ptr::addr_of_mut!(RET.3);
Goto(bb12)
}
bb12 = {
_25 = !_19;
_2.0 = [_12,_12,_12,_12,_12,_12,_12];
_24.0 = _7;
_24.1 = !_10;
_24.0 = _7;
_5 = _9 * _18;
RET.2 = core::ptr::addr_of!(RET.0);
RET.1 = Move(RET.2);
_24 = (_7, _10);
_24.0 = _7;
_16 = _13;
_2.3 = core::ptr::addr_of_mut!(RET.1);
_10 = RET.0 as u128;
RET.2 = Move(RET.1);
_4 = core::ptr::addr_of!(_8);
_20.2.0 = 3371655874_u32 + 2878166454_u32;
_2.2 = [61_u8,14_u8,205_u8,188_u8,222_u8,19_u8,94_u8];
_15 = &_4;
Goto(bb13)
}
bb13 = {
_13 = _16;
_28 = _13 == _13;
Goto(bb14)
}
bb14 = {
_6 = _8 as usize;
RET.1 = core::ptr::addr_of!(RET.0);
_16 = -_13;
(*_26) = _19 as i16;
_20.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
(*_26) = (-8536_i16);
_24.0 = _7;
_35 = !RET.3;
_11 = _13 as u16;
_7 = _24.0;
_15 = &_4;
_24.0 = _7;
_12 = _28;
_31 = _6 as f32;
_38 = [_7,_24.0,_24.0,_7];
_34 = _13 * _13;
RET.0 = 1071881762526183942_i64 | (-2756973776082069978_i64);
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(2_usize, 28_usize, Move(_28), 11_usize, Move(_11), 38_usize, Move(_38), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(2_usize, 1_usize, Move(_1), 5_usize, Move(_5), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(2_usize, 20_usize, Move(_20), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [u8; 7],mut _2: char,mut _3: bool) -> i64 {
mir! {
type RET = i64;
let _4: isize;
let _5: char;
let _6: i8;
let _7: *const (u32,);
let _8: bool;
let _9: [i32; 5];
let _10: (i16, u128);
let _11: Adt60;
let _12: Adt60;
let _13: Adt42;
let _14: u128;
let _15: i8;
let _16: *const f64;
let _17: i64;
let _18: u64;
let _19: char;
let _20: f64;
let _21: *mut *mut *const *const i64;
let _22: *const *const i64;
let _23: Adt59;
let _24: Adt58;
let _25: isize;
let _26: i8;
let _27: ();
let _28: ();
{
_1 = [8_u8,72_u8,6_u8,173_u8,45_u8,217_u8,203_u8];
RET = (-6944159257811531941_i64);
RET = 25_u8 as i64;
RET = -2571303949220726649_i64;
_3 = true;
RET = (-1155918919718369554_i64) | 8778233234333045828_i64;
RET = (-908203939689507341_i64) << 4223542265_u32;
_2 = '\u{ac351}';
RET = 6780337687285118357_i64;
_4 = 9223372036854775807_isize - 9223372036854775807_isize;
_4 = 9223372036854775807_isize;
RET = 1767532993765387968_i64;
_4 = 17075010003863083176372126209814321068_u128 as isize;
RET = -(-1875526559861093012_i64);
RET = (-4556229142728661061_i64) - (-1087688714220222681_i64);
_1 = [220_u8,165_u8,151_u8,189_u8,127_u8,183_u8,2_u8];
RET = 3925071551735666896_i64 + (-7581504530867586250_i64);
_4 = !(-122_isize);
RET = 1163033881204738940_i64 + 3263965477606255854_i64;
_2 = '\u{9f778}';
_4 = (-9223372036854775808_isize);
RET = _4 as i64;
_2 = '\u{f63d6}';
_2 = '\u{fd80}';
RET = 1797285797104048737_i64 << _4;
RET = _4 as i64;
_4 = (-73_isize);
_5 = _2;
Call(_6 = fn4(_4, _1, RET, _4, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_6 = 13_i8;
_8 = !_3;
_3 = _2 > _5;
_1 = [146_u8,101_u8,247_u8,175_u8,198_u8,118_u8,228_u8];
_3 = _8;
RET = !(-3685197269032004639_i64);
_4 = (-9223372036854775808_isize);
_9 = [(-1160742570_i32),1251501691_i32,1150552374_i32,168024702_i32,1431141009_i32];
_1 = [247_u8,238_u8,14_u8,230_u8,187_u8,207_u8,57_u8];
_3 = _4 > _4;
_5 = _2;
_2 = _5;
_10.0 = -(-6831_i16);
_10.1 = 236420880359261564164343058509913233323_u128;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_10 = (8248_i16, 153018255724887726242711301784828003811_u128);
_3 = _10.0 <= _10.0;
_5 = _2;
_1 = [68_u8,122_u8,100_u8,119_u8,175_u8,109_u8,50_u8];
_2 = _5;
_11.fld0 = _10;
_11.fld3.0 = _2;
_11.fld6 = 25234661479744101802248575199260881907_i128 as usize;
_6 = _10.0 as i8;
_2 = _11.fld3.0;
_10 = (_11.fld0.0, _11.fld0.1);
_12.fld4 = (1261940271_u32,);
_12.fld3 = (_2, _10.1);
_12.fld2 = core::ptr::addr_of!(_11.fld4);
_10.0 = _5 as i16;
_12.fld3 = (_5, _11.fld0.1);
_12.fld7 = (-85377536097022635613662903331858941868_i128);
Call(_12.fld0.1 = core::intrinsics::transmute(_12.fld3.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12.fld5 = core::ptr::addr_of_mut!(_10.0);
_1 = [164_u8,4_u8,63_u8,61_u8,169_u8,173_u8,138_u8];
_10 = (_11.fld0.0, _12.fld3.1);
RET = !5173752183221420249_i64;
_11.fld3.0 = _12.fld3.0;
_11.fld4.0 = _12.fld4.0;
_4 = _6 as isize;
_12.fld0.1 = _11.fld0.1 - _12.fld3.1;
_11.fld3 = _12.fld3;
_8 = !_3;
_5 = _12.fld3.0;
_12.fld6 = _11.fld6;
_12.fld0.0 = _10.0 - _11.fld0.0;
_11.fld0.1 = _10.0 as u128;
_11.fld7 = !_12.fld7;
_10.1 = _11.fld7 as u128;
_14 = _11.fld0.1;
_7 = core::ptr::addr_of!(_11.fld4);
_11.fld2 = core::ptr::addr_of!(_11.fld4);
match _12.fld3.1 {
0 => bb7,
1 => bb6,
2 => bb3,
153018255724887726242711301784828003811 => bb9,
_ => bb5
}
}
bb9 = {
_11.fld3 = (_2, _14);
_11.fld4.0 = 63034051490580061_u64 as u32;
_11.fld4 = (_12.fld4.0,);
_11.fld4 = (_12.fld4.0,);
_15 = _6;
_14 = _11.fld0.1 & _12.fld0.1;
_12.fld0.0 = RET as i16;
RET = 5652753151416748580_i64;
RET = -2737117611192078019_i64;
_11.fld0.0 = _12.fld0.0 >> _15;
_12.fld4 = _11.fld4;
_11.fld5 = Move(_12.fld5);
_12.fld3.0 = _2;
_6 = _15;
_11.fld3.1 = 44183_u16 as u128;
Goto(bb10)
}
bb10 = {
_11.fld7 = _12.fld7;
_7 = Move(_12.fld2);
_9 = [(-1940831496_i32),(-2142581171_i32),508515430_i32,1241004744_i32,284225372_i32];
_12.fld2 = core::ptr::addr_of!(_12.fld4);
_11.fld0.1 = _12.fld7 as u128;
_18 = _12.fld6 as u64;
Goto(bb11)
}
bb11 = {
_12.fld5 = core::ptr::addr_of_mut!(_11.fld0.0);
_12.fld4.0 = _11.fld4.0 * _11.fld4.0;
_18 = 15948182491219560647_u64 ^ 14601333966979174143_u64;
_5 = _12.fld3.0;
_12.fld7 = _11.fld7 + _11.fld7;
RET = -(-5931683285025415154_i64);
_16 = core::ptr::addr_of!(_20);
_11.fld2 = Move(_7);
_12.fld3 = _11.fld3;
_18 = _14 as u64;
_12.fld4 = (_11.fld4.0,);
_12.fld2 = core::ptr::addr_of!(_12.fld4);
_12.fld0.0 = _10.0;
Goto(bb12)
}
bb12 = {
_12.fld2 = Move(_11.fld2);
_17 = RET - RET;
_2 = _11.fld3.0;
_10 = (_12.fld0.0, _12.fld0.1);
_9 = [(-1474969207_i32),1584213437_i32,(-1626815720_i32),256565708_i32,1551623980_i32];
match _11.fld7 {
0 => bb13,
1 => bb14,
254904830823915827849711704099909269588 => bb16,
_ => bb15
}
}
bb13 = {
_12.fld5 = core::ptr::addr_of_mut!(_11.fld0.0);
_12.fld4.0 = _11.fld4.0 * _11.fld4.0;
_18 = 15948182491219560647_u64 ^ 14601333966979174143_u64;
_5 = _12.fld3.0;
_12.fld7 = _11.fld7 + _11.fld7;
RET = -(-5931683285025415154_i64);
_16 = core::ptr::addr_of!(_20);
_11.fld2 = Move(_7);
_12.fld3 = _11.fld3;
_18 = _14 as u64;
_12.fld4 = (_11.fld4.0,);
_12.fld2 = core::ptr::addr_of!(_12.fld4);
_12.fld0.0 = _10.0;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_11.fld3 = (_2, _14);
_11.fld4.0 = 63034051490580061_u64 as u32;
_11.fld4 = (_12.fld4.0,);
_11.fld4 = (_12.fld4.0,);
_15 = _6;
_14 = _11.fld0.1 & _12.fld0.1;
_12.fld0.0 = RET as i16;
RET = 5652753151416748580_i64;
RET = -2737117611192078019_i64;
_11.fld0.0 = _12.fld0.0 >> _15;
_12.fld4 = _11.fld4;
_11.fld5 = Move(_12.fld5);
_12.fld3.0 = _2;
_6 = _15;
_11.fld3.1 = 44183_u16 as u128;
Goto(bb10)
}
bb16 = {
_7 = Move(_12.fld2);
_11.fld0 = (_12.fld0.0, _10.1);
_19 = _5;
_12.fld4.0 = _11.fld4.0 & _11.fld4.0;
_12.fld0 = (_11.fld0.0, _10.1);
_24.fld5 = core::ptr::addr_of!(_17);
_11.fld0.1 = _10.1 ^ _14;
_12.fld0.0 = _11.fld0.0 - _11.fld0.0;
_12.fld7 = _11.fld7;
_9 = [234712030_i32,1562467419_i32,(-1129270033_i32),494402327_i32,849363510_i32];
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(3_usize, 2_usize, Move(_2), 19_usize, Move(_19), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(3_usize, 5_usize, Move(_5), 4_usize, Move(_4), 8_usize, Move(_8), 28_usize, _28), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: [u8; 7],mut _3: i64,mut _4: isize,mut _5: [u8; 7]) -> i8 {
mir! {
type RET = i8;
let _6: isize;
let _7: *const [u8; 2];
let _8: &'static isize;
let _9: f32;
let _10: [usize; 8];
let _11: *const (u32,);
let _12: Adt53;
let _13: i32;
let _14: (*const *const i64,);
let _15: u32;
let _16: isize;
let _17: f64;
let _18: [i64; 8];
let _19: [bool; 7];
let _20: [bool; 3];
let _21: f64;
let _22: bool;
let _23: bool;
let _24: &'static &'static u16;
let _25: i64;
let _26: isize;
let _27: &'static &'static *const f64;
let _28: isize;
let _29: &'static &'static u16;
let _30: ((i16, u128),);
let _31: usize;
let _32: ();
let _33: ();
{
RET = 4697456225842108157_u64 as i8;
RET = 51_i8;
_5 = [174_u8,58_u8,204_u8,43_u8,0_u8,221_u8,52_u8];
_6 = -_4;
_6 = !_4;
_1 = 137_u8 as isize;
_4 = _6 ^ _6;
RET = 5835922990366324245_u64 as i8;
RET = 31339_u16 as i8;
_3 = 1134042631023178316_i64;
_3 = 20371_u16 as i64;
_3 = -(-1561750284020816364_i64);
_4 = _1;
_4 = !_1;
_6 = 226_u8 as isize;
_4 = RET as isize;
_2 = [240_u8,61_u8,79_u8,90_u8,224_u8,88_u8,111_u8];
_2 = _5;
RET = !(-33_i8);
_4 = false as isize;
Goto(bb1)
}
bb1 = {
_1 = RET as isize;
_8 = &_4;
RET = 58_i8;
_5 = [148_u8,40_u8,61_u8,123_u8,163_u8,44_u8,91_u8];
_2 = [235_u8,180_u8,218_u8,14_u8,221_u8,146_u8,30_u8];
Goto(bb2)
}
bb2 = {
_5 = [52_u8,176_u8,149_u8,193_u8,102_u8,97_u8,10_u8];
_5 = _2;
_4 = !_6;
_5 = _2;
_2 = [38_u8,226_u8,181_u8,123_u8,7_u8,82_u8,234_u8];
RET = -35_i8;
_3 = 8098638220054372862_i64;
_3 = -2503312151522064919_i64;
RET = 38523_u16 as i8;
RET = -(-51_i8);
_4 = _6 ^ _1;
_1 = _4;
Goto(bb3)
}
bb3 = {
_2 = [198_u8,16_u8,78_u8,171_u8,109_u8,80_u8,110_u8];
_8 = &_4;
_9 = (*_8) as f32;
_2 = [42_u8,64_u8,115_u8,95_u8,75_u8,48_u8,145_u8];
_2 = [92_u8,28_u8,129_u8,148_u8,132_u8,37_u8,226_u8];
_1 = -(*_8);
_9 = 3523802861453409100_u64 as f32;
_6 = !_1;
_4 = 1514070094_i32 as isize;
_1 = '\u{8a2fe}' as isize;
RET = (-22218_i16) as i8;
_2 = _5;
_8 = &_6;
_10 = [208013712022754970_usize,0_usize,1_usize,6_usize,2485992288841583787_usize,11957165259051647778_usize,3_usize,5_usize];
RET = 24_i8;
_6 = 12_u8 as isize;
_10 = [4_usize,5_usize,15861076895412918391_usize,8638717056176701820_usize,4_usize,7_usize,13348562070713227173_usize,2_usize];
_12.fld1 = (-26505_i16) as f32;
_13 = 1408232264_i32 * 1026808655_i32;
_13 = (-958057726_i32) & (-1204576710_i32);
_12.fld6.0 = 19_u8 as u32;
_12.fld2.0 = _12.fld6.0;
RET = -(-4_i8);
Goto(bb4)
}
bb4 = {
_12.fld2.3 = _4;
_12.fld3 = 2_usize as i8;
_12.fld7.1 = [(-25734385989676663521836554330941550876_i128),137456437361388587368661619343048356044_i128,(-2998533554065882523362873034544596306_i128),128074504308091467308875649251757314737_i128,112281924128224372907464308444300980940_i128,(-100389550174805408369857289169548052413_i128),148349334927797940355603931421908151488_i128];
_12.fld0 = ('\u{7375f}', 306641195846341850440137184888420472784_u128);
Goto(bb5)
}
bb5 = {
_15 = _12.fld2.0;
_12.fld2.3 = _12.fld1 as isize;
_12.fld0.0 = '\u{c6390}';
_13 = 225389064_i32;
_12.fld2.1 = _12.fld2.0 & _15;
_8 = &_12.fld2.3;
_12.fld7.2 = _12.fld6;
_12.fld4.1 = _12.fld0.1;
_12.fld2.3 = -_6;
_3 = !928074794277685817_i64;
_13 = (-1049926396_i32);
_20 = [false,true,true];
_12.fld7.2.0 = _12.fld2.0;
_10 = [0_usize,6_usize,8206351319852009276_usize,14159914259987853975_usize,7_usize,660698006963527352_usize,8858057614751169988_usize,1_usize];
RET = _12.fld3;
_11 = core::ptr::addr_of!(_12.fld6);
_12.fld1 = _9;
_11 = core::ptr::addr_of!(_12.fld6);
RET = 14269_i16 as i8;
Call(_12.fld1 = fn5(_12.fld0.0, _12.fld2.0, _12.fld7.2, _12.fld6.0, _12.fld7.1, _12.fld7.1, _12.fld7.1, _12.fld0.0, (*_11)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.fld7.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_18 = [_3,_3,_3,_3,_3,_3,_3,_3];
_15 = (*_11).0;
_9 = _12.fld1;
_2 = _5;
_12.fld0 = ('\u{5ba71}', _12.fld4.1);
_19 = [false,false,false,false,false,true,false];
_19 = [false,false,false,false,false,false,true];
_12.fld4 = (5462_i16, _12.fld0.1);
_16 = _6 >> _12.fld2.1;
_23 = false | false;
_2 = [228_u8,155_u8,54_u8,142_u8,60_u8,57_u8,211_u8];
_13 = 1598244003_i32 | (-336097573_i32);
_18 = [_3,_3,_3,_3,_3,_3,_3,_3];
_18 = _12.fld7.0;
_13 = 2131375009_i32;
_12.fld7.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_12.fld0.0 = '\u{2ca7f}';
_12.fld5 = core::ptr::addr_of_mut!(_14.0);
_12.fld2.2 = (*_11).0 as i8;
_17 = 151_u8 as f64;
(*_11) = (_12.fld2.1,);
_3 = 8037137674559834047_i64;
_26 = _16;
Goto(bb7)
}
bb7 = {
_4 = _26 | _26;
(*_11).0 = !_12.fld2.1;
match _12.fld4.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
5462 => bb15,
_ => bb14
}
}
bb8 = {
_12.fld7.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_18 = [_3,_3,_3,_3,_3,_3,_3,_3];
_15 = (*_11).0;
_9 = _12.fld1;
_2 = _5;
_12.fld0 = ('\u{5ba71}', _12.fld4.1);
_19 = [false,false,false,false,false,true,false];
_19 = [false,false,false,false,false,false,true];
_12.fld4 = (5462_i16, _12.fld0.1);
_16 = _6 >> _12.fld2.1;
_23 = false | false;
_2 = [228_u8,155_u8,54_u8,142_u8,60_u8,57_u8,211_u8];
_13 = 1598244003_i32 | (-336097573_i32);
_18 = [_3,_3,_3,_3,_3,_3,_3,_3];
_18 = _12.fld7.0;
_13 = 2131375009_i32;
_12.fld7.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_12.fld0.0 = '\u{2ca7f}';
_12.fld5 = core::ptr::addr_of_mut!(_14.0);
_12.fld2.2 = (*_11).0 as i8;
_17 = 151_u8 as f64;
(*_11) = (_12.fld2.1,);
_3 = 8037137674559834047_i64;
_26 = _16;
Goto(bb7)
}
bb9 = {
_15 = _12.fld2.0;
_12.fld2.3 = _12.fld1 as isize;
_12.fld0.0 = '\u{c6390}';
_13 = 225389064_i32;
_12.fld2.1 = _12.fld2.0 & _15;
_8 = &_12.fld2.3;
_12.fld7.2 = _12.fld6;
_12.fld4.1 = _12.fld0.1;
_12.fld2.3 = -_6;
_3 = !928074794277685817_i64;
_13 = (-1049926396_i32);
_20 = [false,true,true];
_12.fld7.2.0 = _12.fld2.0;
_10 = [0_usize,6_usize,8206351319852009276_usize,14159914259987853975_usize,7_usize,660698006963527352_usize,8858057614751169988_usize,1_usize];
RET = _12.fld3;
_11 = core::ptr::addr_of!(_12.fld6);
_12.fld1 = _9;
_11 = core::ptr::addr_of!(_12.fld6);
RET = 14269_i16 as i8;
Call(_12.fld1 = fn5(_12.fld0.0, _12.fld2.0, _12.fld7.2, _12.fld6.0, _12.fld7.1, _12.fld7.1, _12.fld7.1, _12.fld0.0, (*_11)), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_12.fld2.3 = _4;
_12.fld3 = 2_usize as i8;
_12.fld7.1 = [(-25734385989676663521836554330941550876_i128),137456437361388587368661619343048356044_i128,(-2998533554065882523362873034544596306_i128),128074504308091467308875649251757314737_i128,112281924128224372907464308444300980940_i128,(-100389550174805408369857289169548052413_i128),148349334927797940355603931421908151488_i128];
_12.fld0 = ('\u{7375f}', 306641195846341850440137184888420472784_u128);
Goto(bb5)
}
bb11 = {
_2 = [198_u8,16_u8,78_u8,171_u8,109_u8,80_u8,110_u8];
_8 = &_4;
_9 = (*_8) as f32;
_2 = [42_u8,64_u8,115_u8,95_u8,75_u8,48_u8,145_u8];
_2 = [92_u8,28_u8,129_u8,148_u8,132_u8,37_u8,226_u8];
_1 = -(*_8);
_9 = 3523802861453409100_u64 as f32;
_6 = !_1;
_4 = 1514070094_i32 as isize;
_1 = '\u{8a2fe}' as isize;
RET = (-22218_i16) as i8;
_2 = _5;
_8 = &_6;
_10 = [208013712022754970_usize,0_usize,1_usize,6_usize,2485992288841583787_usize,11957165259051647778_usize,3_usize,5_usize];
RET = 24_i8;
_6 = 12_u8 as isize;
_10 = [4_usize,5_usize,15861076895412918391_usize,8638717056176701820_usize,4_usize,7_usize,13348562070713227173_usize,2_usize];
_12.fld1 = (-26505_i16) as f32;
_13 = 1408232264_i32 * 1026808655_i32;
_13 = (-958057726_i32) & (-1204576710_i32);
_12.fld6.0 = 19_u8 as u32;
_12.fld2.0 = _12.fld6.0;
RET = -(-4_i8);
Goto(bb4)
}
bb12 = {
_5 = [52_u8,176_u8,149_u8,193_u8,102_u8,97_u8,10_u8];
_5 = _2;
_4 = !_6;
_5 = _2;
_2 = [38_u8,226_u8,181_u8,123_u8,7_u8,82_u8,234_u8];
RET = -35_i8;
_3 = 8098638220054372862_i64;
_3 = -2503312151522064919_i64;
RET = 38523_u16 as i8;
RET = -(-51_i8);
_4 = _6 ^ _1;
_1 = _4;
Goto(bb3)
}
bb13 = {
_1 = RET as isize;
_8 = &_4;
RET = 58_i8;
_5 = [148_u8,40_u8,61_u8,123_u8,163_u8,44_u8,91_u8];
_2 = [235_u8,180_u8,218_u8,14_u8,221_u8,146_u8,30_u8];
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_1 = _26;
_23 = _12.fld2.0 > _12.fld6.0;
_3 = -(-3606003964134763244_i64);
(*_11) = (_12.fld2.1,);
_12.fld1 = _9 + _9;
_9 = -_12.fld1;
_21 = -_17;
_12.fld2.1 = _9 as u32;
_8 = &_28;
_23 = false ^ true;
_13 = 345653726_i32 >> _12.fld7.2.0;
_30.0.1 = _12.fld0.1 * _12.fld0.1;
_23 = false & false;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(4_usize, 6_usize, Move(_6), 10_usize, Move(_10), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(4_usize, 15_usize, Move(_15), 26_usize, Move(_26), 1_usize, Move(_1), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: char,mut _2: u32,mut _3: (u32,),mut _4: u32,mut _5: [i128; 7],mut _6: [i128; 7],mut _7: [i128; 7],mut _8: char,mut _9: (u32,)) -> f32 {
mir! {
type RET = f32;
let _10: u32;
let _11: &'static &'static *mut i16;
let _12: (char, u128);
let _13: [u8; 2];
let _14: ((i16, u128),);
let _15: [i32; 5];
let _16: u16;
let _17: i8;
let _18: f64;
let _19: Adt46;
let _20: isize;
let _21: *mut *mut *const *const i64;
let _22: isize;
let _23: isize;
let _24: u128;
let _25: [char; 4];
let _26: [char; 4];
let _27: *const [i128; 4];
let _28: (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _29: ((i16, u128),);
let _30: isize;
let _31: *const f64;
let _32: i8;
let _33: ();
let _34: ();
{
_3 = (_4,);
_2 = _3.0 - _3.0;
_3 = _9;
_6 = [74839970613419882901959099905916913615_i128,61359953047277118529532616691061532156_i128,39490390156856055317338734103120893457_i128,(-144724755147035462268769035999245309923_i128),(-12056754109346455524488420120659778354_i128),167142717180101265896313347255188933019_i128,164152069705509503820913368430738293076_i128];
_7 = [(-122256380738652137175548066560348305673_i128),102946005166431314693286873279678231089_i128,133180052451083882546836035661148010099_i128,123089570932906256061426369877321399323_i128,(-24159272232637522432311220741691520622_i128),44268030060450262139738660576101757523_i128,(-57037181531472460101809567451311978200_i128)];
RET = _2 as f32;
_1 = _8;
_9.0 = !_2;
_3 = _9;
_8 = _1;
_7 = [(-13189230497494298986221721976581734343_i128),133039991880594443859270908617425798859_i128,16718252549040945892427657329075104954_i128,46861657231851534475282929123309369691_i128,(-157139827225669660848733532255217409243_i128),(-14287107231643754816870376107438928428_i128),(-58014379771902015626392886085289638618_i128)];
_2 = !_3.0;
_10 = !_3.0;
_4 = !_3.0;
_4 = !_9.0;
_8 = _1;
_9 = _3;
_6 = [(-80593123281265900544522995164873964236_i128),(-121260186195467681046849065817476758387_i128),(-119445364277693994135158345626337170037_i128),(-92697966867653930415118352843365398050_i128),150024565588644224679629164468077595109_i128,(-105775962466577905146976775101838401694_i128),(-139018163370847981454096424145269981225_i128)];
_9 = (_2,);
_3 = _9;
_7 = _6;
_9 = _3;
_10 = _9.0 * _2;
_12.0 = _1;
_6 = _5;
_5 = _7;
_3.0 = 8004863341578044882_i64 as u32;
Goto(bb1)
}
bb1 = {
RET = (-83_i8) as f32;
_12.0 = _1;
_6 = _5;
_9.0 = !_4;
_12 = (_8, 49939845037072802875514004668977167905_u128);
_12 = (_8, 93519139243102418530571597597529268264_u128);
_12 = (_8, 209376512870669840460079080653453996583_u128);
_14.0 = (7446_i16, _12.1);
_5 = [(-55582209541504488354114068296268017606_i128),151644520862299264314380263820962978138_i128,104076068646413903932430616688334993657_i128,155344123320484136996665971689135605982_i128,(-104796498778592102552932541331378717814_i128),(-169734396914570088791608372749376086297_i128),(-34744466245144251480632132197861229533_i128)];
_12.0 = _1;
_12.0 = _1;
_5 = [(-55269500584935953973452095444350384143_i128),(-88504904012376239099404617294434606060_i128),103933377468795496736961870423372044089_i128,(-26661919584472529881320909184929471810_i128),(-39206251063793205647484187753038864945_i128),114989802674686131098185299029235305144_i128,46969956608229047452220856957900188893_i128];
_12.0 = _8;
_9.0 = _10;
_12 = (_8, _14.0.1);
_9 = (_4,);
Goto(bb2)
}
bb2 = {
_14.0 = (32761_i16, _12.1);
_2 = _9.0;
_12.1 = _14.0.1 >> _14.0.1;
_4 = !_10;
_12.1 = 121_u8 as u128;
_14.0.0 = (-43764449780454052207455620365769347659_i128) as i16;
_5 = _7;
_8 = _12.0;
_10 = _4;
_2 = false as u32;
_8 = _12.0;
Call(_2 = fn6(_14.0.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = [28_u8,25_u8];
_6 = [(-73682538451945594708219609009172010356_i128),(-18789148080668965736718420960242318643_i128),107526710081075865837239425499866927476_i128,42485407860352392867645254050696039075_i128,(-75490069475919281388235798753737694909_i128),(-48085497562103031336921410625777608104_i128),(-92603714497937962726346472047144862604_i128)];
_7 = [111138887207165839013430260418186008935_i128,7140094623328135143341946157562146968_i128,(-39745933197053691134144545479918883800_i128),(-34648583140795016958894152674856376725_i128),(-66983965821032866556683646473133978287_i128),(-18439509504836426911226170023476836289_i128),8976845446115833637956342225979510866_i128];
_14.0.0 = 8669_i16;
_14.0 = ((-20185_i16), _12.1);
_1 = _8;
_3.0 = _2 - _2;
_5 = [(-139870168382805813133614900966657066434_i128),29500080371248023229204800271742933749_i128,(-12520757331706172891670304205245742804_i128),151178699194077527525074505000955072969_i128,(-85575684834985475066984588663498622213_i128),(-144730954844902474730530663506754766818_i128),43654479947741885915451170703783602662_i128];
_2 = RET as u32;
_14.0.1 = _12.1 ^ _12.1;
_14.0.0 = RET as i16;
_12.0 = _8;
_15 = [1550244672_i32,(-1716675959_i32),2075540053_i32,(-1025862439_i32),(-461378718_i32)];
_5 = _7;
RET = _14.0.1 as f32;
_4 = false as u32;
_4 = _3.0;
_8 = _1;
RET = _14.0.1 as f32;
_9.0 = _3.0 & _4;
_12 = (_8, _14.0.1);
_6 = [146612721733024842010245947077985062252_i128,6748710171059741163477763509543534332_i128,106909931162395267929436749789053977323_i128,124510561671155726107897230533893413218_i128,96543625218180513020633435737601362106_i128,6599662462641346480024813402710426520_i128,(-47631174115460123733939901550538380453_i128)];
_9.0 = _3.0;
_14.0 = ((-31480_i16), _12.1);
_9 = _3;
_1 = _8;
_5 = _6;
_7 = [72961704056800376237138888777540181289_i128,18017683449178617621513591673418925726_i128,(-165604546459245267126576241365900775201_i128),91625102399771279043006661856373508479_i128,(-85817303588464893792702713341307763819_i128),(-105558915838256255570757066418511304194_i128),(-167982566045321882738015651729247307442_i128)];
Call(_17 = core::intrinsics::bswap(119_i8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = 37565_u16;
_3 = (_9.0,);
_15 = [144713320_i32,(-1686407375_i32),2083147077_i32,(-533877455_i32),(-1953242233_i32)];
_12.1 = !_14.0.1;
_9 = (_3.0,);
_4 = _9.0 * _9.0;
_8 = _1;
match _14.0.0 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607431768179976 => bb12,
_ => bb11
}
}
bb5 = {
_13 = [28_u8,25_u8];
_6 = [(-73682538451945594708219609009172010356_i128),(-18789148080668965736718420960242318643_i128),107526710081075865837239425499866927476_i128,42485407860352392867645254050696039075_i128,(-75490069475919281388235798753737694909_i128),(-48085497562103031336921410625777608104_i128),(-92603714497937962726346472047144862604_i128)];
_7 = [111138887207165839013430260418186008935_i128,7140094623328135143341946157562146968_i128,(-39745933197053691134144545479918883800_i128),(-34648583140795016958894152674856376725_i128),(-66983965821032866556683646473133978287_i128),(-18439509504836426911226170023476836289_i128),8976845446115833637956342225979510866_i128];
_14.0.0 = 8669_i16;
_14.0 = ((-20185_i16), _12.1);
_1 = _8;
_3.0 = _2 - _2;
_5 = [(-139870168382805813133614900966657066434_i128),29500080371248023229204800271742933749_i128,(-12520757331706172891670304205245742804_i128),151178699194077527525074505000955072969_i128,(-85575684834985475066984588663498622213_i128),(-144730954844902474730530663506754766818_i128),43654479947741885915451170703783602662_i128];
_2 = RET as u32;
_14.0.1 = _12.1 ^ _12.1;
_14.0.0 = RET as i16;
_12.0 = _8;
_15 = [1550244672_i32,(-1716675959_i32),2075540053_i32,(-1025862439_i32),(-461378718_i32)];
_5 = _7;
RET = _14.0.1 as f32;
_4 = false as u32;
_4 = _3.0;
_8 = _1;
RET = _14.0.1 as f32;
_9.0 = _3.0 & _4;
_12 = (_8, _14.0.1);
_6 = [146612721733024842010245947077985062252_i128,6748710171059741163477763509543534332_i128,106909931162395267929436749789053977323_i128,124510561671155726107897230533893413218_i128,96543625218180513020633435737601362106_i128,6599662462641346480024813402710426520_i128,(-47631174115460123733939901550538380453_i128)];
_9.0 = _3.0;
_14.0 = ((-31480_i16), _12.1);
_9 = _3;
_1 = _8;
_5 = _6;
_7 = [72961704056800376237138888777540181289_i128,18017683449178617621513591673418925726_i128,(-165604546459245267126576241365900775201_i128),91625102399771279043006661856373508479_i128,(-85817303588464893792702713341307763819_i128),(-105558915838256255570757066418511304194_i128),(-167982566045321882738015651729247307442_i128)];
Call(_17 = core::intrinsics::bswap(119_i8), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_14.0 = (32761_i16, _12.1);
_2 = _9.0;
_12.1 = _14.0.1 >> _14.0.1;
_4 = !_10;
_12.1 = 121_u8 as u128;
_14.0.0 = (-43764449780454052207455620365769347659_i128) as i16;
_5 = _7;
_8 = _12.0;
_10 = _4;
_2 = false as u32;
_8 = _12.0;
Call(_2 = fn6(_14.0.0), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = (-83_i8) as f32;
_12.0 = _1;
_6 = _5;
_9.0 = !_4;
_12 = (_8, 49939845037072802875514004668977167905_u128);
_12 = (_8, 93519139243102418530571597597529268264_u128);
_12 = (_8, 209376512870669840460079080653453996583_u128);
_14.0 = (7446_i16, _12.1);
_5 = [(-55582209541504488354114068296268017606_i128),151644520862299264314380263820962978138_i128,104076068646413903932430616688334993657_i128,155344123320484136996665971689135605982_i128,(-104796498778592102552932541331378717814_i128),(-169734396914570088791608372749376086297_i128),(-34744466245144251480632132197861229533_i128)];
_12.0 = _1;
_12.0 = _1;
_5 = [(-55269500584935953973452095444350384143_i128),(-88504904012376239099404617294434606060_i128),103933377468795496736961870423372044089_i128,(-26661919584472529881320909184929471810_i128),(-39206251063793205647484187753038864945_i128),114989802674686131098185299029235305144_i128,46969956608229047452220856957900188893_i128];
_12.0 = _8;
_9.0 = _10;
_12 = (_8, _14.0.1);
_9 = (_4,);
Goto(bb2)
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
_7 = [(-101519137634690164061359045863373738663_i128),167175785941124858463141874724590488543_i128,33505497347717278674191129343852445962_i128,(-102785127088756662491371418111706615140_i128),(-13105062737901909883531044781635051229_i128),(-52188276628878414443183280458666315171_i128),108696116124923433583894118956244505176_i128];
_5 = [57770259147829745343103713891855499920_i128,(-135002839073251751378044014794468597967_i128),29419509083351880341235854980844259146_i128,28034444590007964237250334311026482384_i128,25596503641997997218534885463749506947_i128,(-90976712832421743647885400124155217873_i128),(-120727590009774960796057472268493918049_i128)];
_12.0 = _1;
_18 = _4 as f64;
_2 = _3.0;
_12 = (_8, _14.0.1);
_20 = !9223372036854775807_isize;
_16 = 4299_u16 << _9.0;
_8 = _1;
_14.0.1 = _12.1 ^ _12.1;
_14.0 = ((-28810_i16), _12.1);
_17 = 66_i8;
_8 = _1;
_6 = [(-123714483260030699219624720371738366965_i128),113220145528085410877481082743467778333_i128,40277841548918235446423365879372808029_i128,32227324851639357521274487271964487149_i128,161508174904589801830047044480512564442_i128,8477510739909421831064741117097751512_i128,37578272966416548222305748894966995882_i128];
_18 = 133_u8 as f64;
_16 = _20 as u16;
_19 = Adt46::Variant1 { fld0: _17 };
_3 = (_9.0,);
_22 = _20 << _9.0;
_3 = (_9.0,);
_8 = _12.0;
Call(_14.0.1 = core::intrinsics::transmute(_12.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3.0 = !_9.0;
SetDiscriminant(_19, 0);
_14.0.1 = _12.1 >> _2;
_24 = !_14.0.1;
_6 = _7;
_12 = (_8, _14.0.1);
RET = 8330097596936799195_i64 as f32;
place!(Field::<((u32,), f32, u32)>(Variant(_19, 0), 0)) = (_3, RET, _9.0);
Call(place!(Field::<((u32,), f32, u32)>(Variant(_19, 0), 0)).0.0 = core::intrinsics::transmute(_9.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_28.1.0 = _14.0.0 - _14.0.0;
_29.0.0 = _14.0.0 << _14.0.1;
_14.0 = (_29.0.0, _12.1);
_29.0.1 = 203291336_i32 as u128;
_8 = _12.0;
_29 = _14;
_7 = [120148211696986074305896252579328832348_i128,73051260019368521877012849207615020654_i128,121775496601960341040160946622620588663_i128,122512593134615273715463666466806051599_i128,2314670100740511817388985625370450662_i128,25317555505316821466761083777512280720_i128,(-142589328090453676422574903500337721208_i128)];
_25 = [_8,_12.0,_12.0,_8];
_28.1.1 = _12.1 ^ _29.0.1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(5_usize, 7_usize, Move(_7), 12_usize, Move(_12), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(5_usize, 29_usize, Move(_29), 14_usize, Move(_14), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(5_usize, 5_usize, Move(_5), 20_usize, Move(_20), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i16) -> u32 {
mir! {
type RET = u32;
let _2: isize;
let _3: ((u32,), f32, u32);
let _4: isize;
let _5: (*const *const i64,);
let _6: u8;
let _7: isize;
let _8: *const (u32,);
let _9: &'static (*const *const i64,);
let _10: (*const *const i64,);
let _11: &'static ((i16, u128),);
let _12: (Adt42, i16, [char; 4]);
let _13: f64;
let _14: char;
let _15: i128;
let _16: bool;
let _17: &'static &'static u16;
let _18: isize;
let _19: usize;
let _20: &'static &'static u16;
let _21: i128;
let _22: [i16; 7];
let _23: bool;
let _24: [u64; 1];
let _25: *const (u32,);
let _26: ();
let _27: ();
{
_1 = 9223372036854775807_isize as i16;
_1 = !9907_i16;
_1 = (-22357_i16) - 5251_i16;
RET = 2682608121_u32;
_1 = -(-16772_i16);
_1 = 21888_i16;
_1 = 11325_i16;
_2 = 116_i8 as isize;
RET = 2351398807_u32 >> _1;
_2 = !(-9223372036854775808_isize);
_1 = (-2168847149255647102_i64) as i16;
_1 = 178246143923885604427417164667148930990_u128 as i16;
_1 = (-26573_i16);
_3.1 = _1 as f32;
_3.0 = (RET,);
RET = _2 as u32;
_2 = (-947196807_i32) as isize;
_3.0 = (RET,);
Goto(bb1)
}
bb1 = {
RET = _3.0.0 >> _1;
RET = !_3.0.0;
_3.0 = (RET,);
_3.2 = RET + _3.0.0;
_3.1 = RET as f32;
_3.1 = 523936883835933123_u64 as f32;
_3.0.0 = !_3.2;
_1 = (-19557_i16) - 13924_i16;
_3.0.0 = _3.2 * RET;
_3.0 = (RET,);
_3.0 = (_3.2,);
_6 = 18_i8 as u8;
RET = false as u32;
_7 = _2;
_3.2 = _3.0.0 * RET;
_1 = (-30660_i16) >> _2;
_3.2 = _3.0.0 & _3.0.0;
_7 = !_2;
_3.0 = (_3.2,);
Call(_3 = fn7(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.1 = (-27_i8) as f32;
_4 = _7;
_7 = (-54_i8) as isize;
_3.0 = (_3.2,);
_8 = core::ptr::addr_of!(_3.0);
_2 = 5_i8 as isize;
_8 = core::ptr::addr_of!(_3.0);
_2 = '\u{cdfed}' as isize;
_1 = 63181_u16 as i16;
_2 = _7;
_3.2 = (*_8).0;
(*_8) = (RET,);
_4 = 1_i8 as isize;
Goto(bb3)
}
bb3 = {
(*_8) = (_3.2,);
_6 = 254_u8;
_3.2 = (*_8).0 << (*_8).0;
_8 = core::ptr::addr_of!((*_8));
match _6 {
254 => bb4,
_ => bb2
}
}
bb4 = {
_9 = &_10;
_8 = core::ptr::addr_of!(_3.0);
_4 = _7 | _2;
RET = (*_8).0;
_6 = '\u{df1f9}' as u8;
_4 = _7;
_3.0 = (_3.2,);
(*_8).0 = !RET;
_3.0.0 = _3.2;
(*_8) = (_3.2,);
_6 = !56_u8;
_3.0.0 = !RET;
_3.1 = 134872286590367500826582024131366182881_u128 as f32;
Goto(bb5)
}
bb5 = {
_6 = !62_u8;
_3.1 = _6 as f32;
Call(_12.2 = fn13(RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.2 = ['\u{101f2a}','\u{3daa4}','\u{56ccf}','\u{256a0}'];
_14 = '\u{db3b0}';
_2 = _4 << _3.2;
_13 = 173335825535943094154819336234442108385_u128 as f64;
_8 = core::ptr::addr_of!((*_8));
_18 = -_7;
_9 = &(*_9);
(*_8).0 = _3.2;
_6 = 14_u8;
_3.0 = (_3.2,);
_12.0 = Adt42::Variant2 { fld0: _1,fld1: 3646139797440055972_i64,fld2: 11644_u16 };
place!(Field::<i64>(Variant(_12.0, 2), 1)) = (-4240198743228805086_i64);
_4 = _2 << (*_8).0;
_7 = -_2;
(*_8).0 = !_3.2;
_2 = _6 as isize;
_3.2 = _3.0.0;
RET = !(*_8).0;
_8 = core::ptr::addr_of!((*_8));
_9 = &(*_9);
_8 = core::ptr::addr_of!(_3.0);
_3.1 = 1276910615_i32 as f32;
(*_8) = (RET,);
Call((*_8).0 = core::intrinsics::bswap(_3.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = core::ptr::addr_of!((*_8));
RET = _3.2;
_3.0 = (_3.2,);
_3.2 = _6 as u32;
_16 = false;
_6 = 146_u8 ^ 182_u8;
_7 = _4 >> _4;
(*_8).0 = RET;
_15 = !(-51294393606769609926677974164244574258_i128);
_4 = _7 ^ _7;
_12.0 = Adt42::Variant2 { fld0: _1,fld1: (-4747754940126088979_i64),fld2: 2799_u16 };
_19 = 2_usize ^ 1_usize;
_3.0 = (RET,);
_9 = &_5;
_12.2 = [_14,_14,_14,_14];
_9 = &_5;
_7 = Field::<i16>(Variant(_12.0, 2), 0) as isize;
_9 = &(*_9);
Call(_19 = core::intrinsics::transmute(_4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_8) = (RET,);
_22 = [Field::<i16>(Variant(_12.0, 2), 0),Field::<i16>(Variant(_12.0, 2), 0),Field::<i16>(Variant(_12.0, 2), 0),_1,Field::<i16>(Variant(_12.0, 2), 0),_1,_1];
RET = _13 as u32;
_12.1 = (-499743113661714187_i64) as i16;
place!(Field::<i64>(Variant(_12.0, 2), 1)) = 1180242501733616725_u64 as i64;
_19 = 294628623333054396615174588560938058695_u128 as usize;
RET = (*_8).0 << _4;
_7 = _4;
_3.0 = (RET,);
Goto(bb9)
}
bb9 = {
Call(_26 = dump_var(6_usize, 1_usize, Move(_1), 4_usize, Move(_4), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_26 = dump_var(6_usize, 19_usize, Move(_19), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize) -> ((u32,), f32, u32) {
mir! {
type RET = ((u32,), f32, u32);
let _2: ((i16, u128),);
let _3: (char, u128);
let _4: i128;
let _5: (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _6: isize;
let _7: bool;
let _8: *mut *const i64;
let _9: &'static &'static (u32,);
let _10: usize;
let _11: usize;
let _12: char;
let _13: char;
let _14: i128;
let _15: (i16, u128);
let _16: [i32; 5];
let _17: ([bool; 7], [i16; 7], [u8; 7], *mut *const i64);
let _18: bool;
let _19: *mut u8;
let _20: u64;
let _21: f32;
let _22: ();
let _23: ();
{
RET.0 = (4171436582_u32,);
RET.0 = (1200900116_u32,);
RET.2 = RET.0.0;
RET.2 = !RET.0.0;
RET.0.0 = RET.2 >> RET.2;
RET.0 = (RET.2,);
RET.1 = 71_i8 as f32;
RET.1 = (-1508568284_i32) as f32;
Call(RET.0 = fn8(_1, RET.2, _1, _1, _1, _1, RET.1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = (RET.2,);
RET.1 = 89087413474494673_u64 as f32;
RET.0 = (RET.2,);
RET.2 = RET.0.0 << _1;
_1 = 68_isize;
_1 = (-9223372036854775808_isize);
_3 = ('\u{e3e1b}', 155318362884068065929802593558707548930_u128);
_3 = ('\u{d64d6}', 40850371470550668045817780833653631274_u128);
_2.0.0 = (-11062_i16);
RET.2 = !RET.0.0;
RET.0 = (RET.2,);
RET.1 = 49841_u16 as f32;
RET.0.0 = RET.2 << _3.1;
RET.0.0 = RET.2 & RET.2;
_4 = (-113202977458027179114523557425831088235_i128) | (-154471136315503909249982055384933457148_i128);
_5.2.1 = _3.0;
_5.0.0 = RET.0;
_5.3.2.0 = _5.0.0.0;
_5.0.0.0 = _5.3.2.0 >> RET.2;
_5.1.1 = 235_u8 as u128;
_5.1 = (_2.0.0, _3.1);
Goto(bb2)
}
bb2 = {
_5.2.1 = _3.0;
RET.0.0 = RET.2 * _5.0.0.0;
_2.0 = (_5.1.0, _3.1);
_5.0.0 = (RET.0.0,);
RET.2 = 24611_u16 as u32;
_5.3.1 = [_4,_4,_4,_4,_4,_4,_4];
_5.0 = (RET.0, RET.1, RET.0.0);
_5.0 = (_5.3.2, RET.1, RET.2);
_5.0.0 = (_5.0.2,);
RET.2 = 66_i8 as u32;
_7 = true & true;
_2.0.0 = _5.1.0 | _5.1.0;
_5.0.2 = _5.3.2.0;
_4 = 186_u8 as i128;
_2.0 = (_5.1.0, _5.1.1);
_3.0 = _5.2.1;
_5.1.0 = -_2.0.0;
_5.3.1 = [_4,_4,_4,_4,_4,_4,_4];
RET.2 = _2.0.0 as u32;
_6 = _1;
RET.0.0 = _5.1.0 as u32;
_3.1 = !_2.0.1;
Goto(bb3)
}
bb3 = {
RET.1 = -_5.0.1;
_5.2.0 = [_3.0,_5.2.1,_3.0,_5.2.1,_3.0,_3.0,_3.0,_3.0];
_5.3.2 = (RET.2,);
_5.0 = (_5.3.2, RET.1, _5.3.2.0);
_5.0.1 = -RET.1;
_5.3.0 = [(-7793381817816702011_i64),3046183846792684343_i64,(-2999890863278696437_i64),9201700425365676609_i64,(-1759959249626516211_i64),(-3311547085685348767_i64),2892408257202509017_i64,(-5644320406946536832_i64)];
_11 = !17078751417828763524_usize;
_5.0.1 = RET.1 - RET.1;
RET.0.0 = !_5.0.2;
_5.0.2 = _5.3.2.0 >> _3.1;
_5.2.1 = _3.0;
_5.2.1 = _3.0;
_2.0 = _5.1;
_5.2.0 = [_3.0,_5.2.1,_3.0,_3.0,_3.0,_3.0,_5.2.1,_3.0];
_2 = (_5.1,);
RET.1 = -_5.0.1;
RET.2 = !_5.0.2;
RET.0 = (_5.0.0.0,);
_2.0.0 = -_5.1.0;
_5.2.0 = [_5.2.1,_5.2.1,_5.2.1,_5.2.1,_3.0,_5.2.1,_5.2.1,_5.2.1];
_5.0 = (RET.0, RET.1, RET.2);
_12 = _5.2.1;
RET.2 = _5.0.0.0 - _5.0.2;
_10 = 696763074_i32 as usize;
_5.0.1 = RET.1;
_5.0.0 = (_5.3.2.0,);
Goto(bb4)
}
bb4 = {
_2.0.0 = RET.2 as i16;
_5.2.1 = _3.0;
_5.0.0.0 = RET.2 << RET.2;
_11 = !_10;
_6 = -_1;
_5.1 = _2.0;
_12 = _5.2.1;
_10 = (-44_i8) as usize;
_5.2.0 = [_5.2.1,_3.0,_5.2.1,_5.2.1,_12,_3.0,_3.0,_12];
_5.3.0 = [(-7899641306872585621_i64),6010240867281956560_i64,(-3440232999803021428_i64),(-4116711599411334790_i64),(-4747503846998118812_i64),6142188315087194546_i64,6330898209941441128_i64,3300268584184070540_i64];
Goto(bb5)
}
bb5 = {
RET.0.0 = _5.0.1 as u32;
_2.0 = (_5.1.0, _3.1);
_5.0.2 = _5.0.0.0 & RET.2;
RET.0.0 = !_5.0.0.0;
_6 = -_1;
_5.0.2 = _7 as u32;
RET.1 = _2.0.0 as f32;
_5.3.0 = [(-2822591805719512822_i64),7442366320706401311_i64,5454912146648081215_i64,123444492336047030_i64,7142985384556804106_i64,(-3865526144304958845_i64),(-5541898314239737364_i64),(-3819884770097698437_i64)];
RET.1 = -_5.0.1;
_5.0.1 = RET.1 - RET.1;
_15.1 = !_2.0.1;
_5.0.0 = _5.3.2;
_5.0.0 = (RET.2,);
Goto(bb6)
}
bb6 = {
_3.0 = _5.2.1;
_5.3.2.0 = RET.0.0 ^ _5.0.0.0;
_5.0.0 = (RET.2,);
_17.0 = [_7,_7,_7,_7,_7,_7,_7];
_5.2.1 = _12;
_11 = _10;
_2.0.0 = _5.1.0;
_12 = _5.2.1;
_5.2.1 = _3.0;
_5.1.1 = _11 as u128;
_12 = _3.0;
_5.0 = (RET.0, RET.1, RET.0.0);
_5.3.1 = [_4,_4,_4,_4,_4,_4,_4];
Goto(bb7)
}
bb7 = {
_6 = _5.1.0 as isize;
_3.1 = _15.1;
_2.0.0 = _5.1.0 << _5.0.2;
_2 = (_5.1,);
_4 = (-1631571265853244566235578842907868976_i128) & 164365640017386462850321978694799190202_i128;
_15.0 = !_5.1.0;
_5.2.0 = [_5.2.1,_3.0,_12,_12,_5.2.1,_3.0,_5.2.1,_3.0];
_2.0.0 = !_5.1.0;
_3 = (_5.2.1, _5.1.1);
_5.3.1 = [_4,_4,_4,_4,_4,_4,_4];
_3.0 = _12;
_14 = _3.0 as i128;
_20 = 9072042787221193733_u64 * 12984542021049773174_u64;
_2.0 = _5.1;
RET.0.0 = _5.0.0.0 ^ _5.3.2.0;
_17.1 = [_2.0.0,_15.0,_5.1.0,_2.0.0,_15.0,_5.1.0,_15.0];
_18 = _6 < _6;
_5.1 = _15;
Goto(bb8)
}
bb8 = {
Call(_22 = dump_var(7_usize, 20_usize, Move(_20), 4_usize, Move(_4), 3_usize, Move(_3), 18_usize, Move(_18)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_22 = dump_var(7_usize, 1_usize, Move(_1), 15_usize, Move(_15), 23_usize, _23, 23_usize, _23), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: u32,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: f32,mut _8: isize,mut _9: isize) -> (u32,) {
mir! {
type RET = (u32,);
let _10: *const &'static isize;
let _11: char;
let _12: (char, u128);
let _13: f32;
let _14: *mut i16;
let _15: isize;
let _16: (u8, f32, &'static &'static u16);
let _17: [u8; 7];
let _18: &'static Adt53;
let _19: isize;
let _20: ((i16, u128),);
let _21: [u8; 2];
let _22: Adt58;
let _23: i8;
let _24: *mut *const *const i64;
let _25: [u8; 7];
let _26: i32;
let _27: isize;
let _28: *mut *mut *const *const i64;
let _29: isize;
let _30: i16;
let _31: isize;
let _32: [i128; 7];
let _33: (char, u128);
let _34: *const i8;
let _35: [bool; 3];
let _36: [bool; 7];
let _37: ();
let _38: ();
{
_5 = !_1;
_7 = 7702593100029090120_i64 as f32;
RET.0 = _2 - _2;
_9 = -_8;
RET.0 = 1750352891_i32 as u32;
RET = (_2,);
RET.0 = !_2;
_9 = _1 << _2;
_4 = -_1;
_4 = !_3;
_5 = !_4;
_12.0 = '\u{f9b0d}';
_11 = _12.0;
_2 = RET.0;
_1 = _6;
_1 = _9;
_12.0 = _11;
_8 = _6;
RET = (_2,);
Call(_6 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = (_11, 299354928804487575271589815957609702618_u128);
_3 = _11 as isize;
_1 = _4 - _6;
_9 = -_1;
RET.0 = !_2;
_12 = (_11, 79962036138398046861386440812932629411_u128);
_13 = -_7;
RET = (_2,);
_1 = 231_u8 as isize;
_12 = (_11, 190289604479509326036121501370240186716_u128);
_4 = _8;
_11 = _12.0;
RET = (_2,);
_1 = 10815802824784810430_u64 as isize;
_7 = _13 * _13;
_12 = (_11, 220407428998301036811356138271196420746_u128);
Call(_12.1 = core::intrinsics::bswap(230691771822571642256878265150276415495_u128), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = _9;
_9 = 1667369054_i32 as isize;
_13 = -_7;
RET = (_2,);
Goto(bb3)
}
bb3 = {
RET.0 = _2;
_8 = !_15;
_7 = _13;
_12.0 = _11;
_16.1 = 2123734031863118134_u64 as f32;
_6 = _8 - _1;
_15 = _1;
_16.0 = 27_u8 ^ 103_u8;
_16.1 = _7;
Call(_5 = fn9(_16.1, _16.1, _4, _12.0, RET, _9, _16.1, _12.0, _2, _1, _9, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = _12.0;
_11 = _12.0;
_3 = _5 & _15;
_2 = RET.0;
_9 = _12.1 as isize;
Goto(bb5)
}
bb5 = {
RET = (_2,);
_12.1 = 63729616703337134838520318983549911186_u128 >> _3;
RET.0 = _2;
_19 = 16920244036053647088_u64 as isize;
RET = (_2,);
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_12.1 = 316088334901307529967190032633367597799_u128 - 75118533794447389159711530283158847537_u128;
_12.0 = _11;
_4 = _6;
_4 = _9;
_16.1 = (-537221462_i32) as f32;
_2 = !RET.0;
_20.0.0 = -(-8732_i16);
_5 = -_15;
_2 = RET.0 * RET.0;
_14 = core::ptr::addr_of_mut!(_20.0.0);
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
RET.0 = _2;
_16.1 = _7;
_22.fld2 = 5903944713785951883_u64 as f32;
_20.0 = (7162_i16, _12.1);
_16.1 = _22.fld2;
_22.fld4 = [151551320135955855379682507101911649970_i128,63357376231026138115421988002663080776_i128,14861697075599936906926038150401265425_i128,59225884205442721193412354416097507108_i128];
_9 = !_4;
_11 = _12.0;
Goto(bb6)
}
bb6 = {
RET = (_2,);
_15 = _19 | _3;
_12.1 = _20.0.1 + _20.0.1;
_20.0.0 = 29041_i16;
RET = (_2,);
_5 = -_6;
_5 = _19 << _15;
_21 = [_16.0,_16.0];
(*_14) = 7397741246778904792_i64 as i16;
_22.fld2 = _13;
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_22.fld0 = 14414843643810810106_usize;
_22.fld1 = core::ptr::addr_of!(_23);
_8 = 40037_u16 as isize;
Goto(bb7)
}
bb7 = {
_22.fld2 = _13 + _7;
_15 = !_4;
RET.0 = _2 * _2;
_9 = _16.0 as isize;
_16.0 = 237_u8;
_6 = _8 ^ _5;
_15 = _5;
_25 = _17;
_3 = !_15;
_22.fld0 = 9039897931160516179_usize << _6;
_22.fld3 = _11 as u32;
_11 = _12.0;
Goto(bb8)
}
bb8 = {
_22.fld4 = [42620124477545851936893304455253602646_i128,105632181657870532051426124821945025817_i128,96235879717511417429229368350769507468_i128,(-110793313554120593687995197021390921360_i128)];
_7 = -_22.fld2;
_19 = _6 + _15;
_4 = _3 - _3;
_26 = (-1589701567_i32) >> _3;
_23 = 8359776118628479264_u64 as i8;
RET.0 = !_2;
_28 = core::ptr::addr_of_mut!(_24);
_19 = _4;
_6 = _22.fld0 as isize;
_9 = !_4;
_22.fld2 = -_7;
_14 = core::ptr::addr_of_mut!(_20.0.0);
_29 = _6 >> _5;
_19 = -_9;
_16.1 = _22.fld2;
match _16.0 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
237 => bb14,
_ => bb13
}
}
bb9 = {
_22.fld2 = _13 + _7;
_15 = !_4;
RET.0 = _2 * _2;
_9 = _16.0 as isize;
_16.0 = 237_u8;
_6 = _8 ^ _5;
_15 = _5;
_25 = _17;
_3 = !_15;
_22.fld0 = 9039897931160516179_usize << _6;
_22.fld3 = _11 as u32;
_11 = _12.0;
Goto(bb8)
}
bb10 = {
RET = (_2,);
_15 = _19 | _3;
_12.1 = _20.0.1 + _20.0.1;
_20.0.0 = 29041_i16;
RET = (_2,);
_5 = -_6;
_5 = _19 << _15;
_21 = [_16.0,_16.0];
(*_14) = 7397741246778904792_i64 as i16;
_22.fld2 = _13;
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_22.fld0 = 14414843643810810106_usize;
_22.fld1 = core::ptr::addr_of!(_23);
_8 = 40037_u16 as isize;
Goto(bb7)
}
bb11 = {
RET = (_2,);
_12.1 = 63729616703337134838520318983549911186_u128 >> _3;
RET.0 = _2;
_19 = 16920244036053647088_u64 as isize;
RET = (_2,);
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_12.1 = 316088334901307529967190032633367597799_u128 - 75118533794447389159711530283158847537_u128;
_12.0 = _11;
_4 = _6;
_4 = _9;
_16.1 = (-537221462_i32) as f32;
_2 = !RET.0;
_20.0.0 = -(-8732_i16);
_5 = -_15;
_2 = RET.0 * RET.0;
_14 = core::ptr::addr_of_mut!(_20.0.0);
_17 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
RET.0 = _2;
_16.1 = _7;
_22.fld2 = 5903944713785951883_u64 as f32;
_20.0 = (7162_i16, _12.1);
_16.1 = _22.fld2;
_22.fld4 = [151551320135955855379682507101911649970_i128,63357376231026138115421988002663080776_i128,14861697075599936906926038150401265425_i128,59225884205442721193412354416097507108_i128];
_9 = !_4;
_11 = _12.0;
Goto(bb6)
}
bb12 = {
_12 = (_11, 299354928804487575271589815957609702618_u128);
_3 = _11 as isize;
_1 = _4 - _6;
_9 = -_1;
RET.0 = !_2;
_12 = (_11, 79962036138398046861386440812932629411_u128);
_13 = -_7;
RET = (_2,);
_1 = 231_u8 as isize;
_12 = (_11, 190289604479509326036121501370240186716_u128);
_4 = _8;
_11 = _12.0;
RET = (_2,);
_1 = 10815802824784810430_u64 as isize;
_7 = _13 * _13;
_12 = (_11, 220407428998301036811356138271196420746_u128);
Call(_12.1 = core::intrinsics::bswap(230691771822571642256878265150276415495_u128), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_15 = _9;
_9 = 1667369054_i32 as isize;
_13 = -_7;
RET = (_2,);
Goto(bb3)
}
bb14 = {
_11 = _12.0;
_22.fld0 = !4884928686022362578_usize;
_29 = _2 as isize;
_23 = !11_i8;
_8 = _6 | _3;
_32 = [(-25681831073130395853521520410538479749_i128),79811472953276248039669669700780812879_i128,(-18080751146002394795027912588379384714_i128),(-87696265641076455369084637383551895286_i128),42987464503421438653233775419402409030_i128,(-158666613977724655428419805463597187188_i128),(-62012141879359501017604823320866605734_i128)];
_22.fld2 = -_7;
_22.fld3 = 5860040413489033399_i64 as u32;
RET.0 = false as u32;
_22.fld3 = _2;
_28 = core::ptr::addr_of_mut!(_24);
_11 = _12.0;
_8 = _4;
_33 = (_11, _12.1);
(*_14) = _16.0 as i16;
_28 = core::ptr::addr_of_mut!((*_28));
_30 = _20.0.0 + _20.0.0;
_27 = _3;
RET = (_2,);
_21 = [_16.0,_16.0];
_32 = [62964498064977033736959866412934601019_i128,(-34313720352178210287977915944635010892_i128),(-19902826731250545136140538960060496374_i128),102411066968879140457186369177570984380_i128,(-34860777035762712628163988410147265506_i128),41077980463149367697790540059457844297_i128,34722617480705494822483695927871371525_i128];
_12 = (_11, _33.1);
_34 = Move(_22.fld1);
_7 = _22.fld2;
_11 = _33.0;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(8_usize, 27_usize, Move(_27), 4_usize, Move(_4), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(8_usize, 21_usize, Move(_21), 17_usize, Move(_17), 9_usize, Move(_9), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(8_usize, 20_usize, Move(_20), 30_usize, Move(_30), 33_usize, Move(_33), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: f32,mut _2: f32,mut _3: isize,mut _4: char,mut _5: (u32,),mut _6: isize,mut _7: f32,mut _8: char,mut _9: u32,mut _10: isize,mut _11: isize,mut _12: f32) -> isize {
mir! {
type RET = isize;
let _13: [i128; 4];
let _14: ([i16; 7],);
let _15: [u8; 7];
let _16: isize;
let _17: i16;
let _18: isize;
let _19: (i16, u128);
let _20: *mut *const &'static isize;
let _21: (u32,);
let _22: f64;
let _23: u16;
let _24: u128;
let _25: &'static isize;
let _26: *mut &'static *mut i16;
let _27: &'static &'static isize;
let _28: [bool; 3];
let _29: ([i16; 7],);
let _30: *const [i128; 4];
let _31: i32;
let _32: [u8; 7];
let _33: Adt46;
let _34: [bool; 7];
let _35: char;
let _36: i16;
let _37: (u32, u32, i8, isize);
let _38: (char, u128);
let _39: f64;
let _40: isize;
let _41: ([i64; 8], [i128; 7], (u32,));
let _42: Adt63;
let _43: (u32, u32, i8, isize);
let _44: i16;
let _45: Adt46;
let _46: *const (u32,);
let _47: *mut i16;
let _48: &'static *const f64;
let _49: u128;
let _50: ([bool; 7], [i16; 7], [u8; 7], *mut *const i64);
let _51: ();
let _52: ();
{
_9 = _5.0;
_1 = _7;
_13 = [75511547644759522356686500509433265233_i128,(-90847540735023364738614089662444236944_i128),103032623510882227389842298853899323713_i128,85450838419922253614585647848130966197_i128];
RET = _9 as isize;
_6 = (-94_i8) as isize;
_3 = _11;
_12 = 4_usize as f32;
_3 = RET;
Goto(bb1)
}
bb1 = {
RET = -_10;
_3 = RET;
Goto(bb2)
}
bb2 = {
RET = _6 ^ _3;
_11 = _3 >> _3;
_16 = _8 as isize;
_8 = _4;
Goto(bb3)
}
bb3 = {
_14.0 = [32735_i16,(-11719_i16),27015_i16,6060_i16,3698_i16,16790_i16,(-24426_i16)];
_4 = _8;
_9 = _5.0;
_12 = _7 + _1;
_17 = (-346_i16) * (-23537_i16);
_12 = -_7;
_13 = [83319832729559492896074593850110497328_i128,(-147359030790789988469071913263786520010_i128),145840473938490218512979945110619949657_i128,127794218246517598101747106104468353458_i128];
_17 = 63967078309665276306231763813903548025_u128 as i16;
_7 = -_12;
_16 = 1916876152907732137_i64 as isize;
_19.0 = (-6277418099896506198_i64) as i16;
Call(_15 = fn10(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.0 = [_17,_17,_17,_19.0,_19.0,_19.0,_19.0];
_14.0 = [_19.0,_17,_19.0,_19.0,_17,_17,_19.0];
_1 = -_7;
RET = !_11;
_16 = !_11;
_19.1 = 117_i8 as u128;
_3 = !_16;
_4 = _8;
_23 = 14964_u16;
_21 = (_5.0,);
_5 = (_21.0,);
_4 = _8;
_17 = -_19.0;
_14.0 = [_19.0,_19.0,_17,_17,_17,_19.0,_19.0];
_5 = (_9,);
_19.1 = 62_i8 as u128;
_15 = [242_u8,4_u8,15_u8,132_u8,154_u8,5_u8,53_u8];
_22 = 14170429409950327090_usize as f64;
_3 = _16 - RET;
_3 = !_10;
_16 = _22 as isize;
_17 = _19.0 ^ _19.0;
_25 = &_10;
_22 = 1569834705_i32 as f64;
_23 = 60097_u16;
Call(_17 = core::intrinsics::bswap(_19.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = -_1;
_23 = 7084_u16 | 3773_u16;
_5 = _21;
_22 = 103_i8 as f64;
_25 = &_3;
_17 = -_19.0;
_5 = (_21.0,);
_4 = _8;
_6 = _22 as isize;
_19.0 = !_17;
_10 = RET;
_23 = 57379_u16 - 15956_u16;
RET = _12 as isize;
_24 = _19.1 | _19.1;
_18 = RET + _16;
_18 = _11;
_13 = [(-86672088509646711397747292291342283762_i128),(-94691861914726527874641569752733573064_i128),80041117695192793807783157319025050308_i128,(-60953881862171302090140321665023496875_i128)];
_10 = _3 - RET;
_27 = &_25;
_5 = (_9,);
_2 = _7;
_23 = !62843_u16;
_2 = _7 - _1;
RET = !_11;
_12 = _7;
Goto(bb6)
}
bb6 = {
_2 = _1;
_30 = core::ptr::addr_of!(_13);
_17 = _4 as i16;
_19 = (_17, _24);
_29.0 = [_19.0,_17,_17,_19.0,_17,_17,_17];
_3 = 5956751845521701053_i64 as isize;
_1 = -_7;
_14.0 = _29.0;
_14.0 = [_17,_19.0,_19.0,_17,_17,_19.0,_17];
_5.0 = _9;
_5.0 = _21.0 + _21.0;
(*_30) = [(-18054297938900685622319141772431698508_i128),28208599722380194401272932482317580259_i128,124579805296939243901541005636444621106_i128,33616304066335837317954326727038678044_i128];
_23 = 24346_u16 >> _16;
_2 = _12;
_18 = RET;
_16 = -_18;
_27 = &_25;
_10 = _18 * _3;
_16 = !RET;
_10 = RET;
_14 = _29;
_11 = (-60_i8) as isize;
_14.0 = [_17,_19.0,_19.0,_17,_17,_17,_19.0];
_4 = _8;
_12 = -_2;
_30 = core::ptr::addr_of!((*_30));
_19.0 = _17 ^ _17;
_19 = (_17, _24);
Goto(bb7)
}
bb7 = {
_17 = !_19.0;
_12 = -_1;
_19.0 = _17;
_16 = _10;
(*_30) = [(-151882224689684601478056459826548106177_i128),(-75127307683554475361206960843163882654_i128),86640426326105252206638288941381520844_i128,(-64928191743186645482911185753939964040_i128)];
_15 = [156_u8,175_u8,212_u8,36_u8,228_u8,232_u8,16_u8];
_21 = (_9,);
_14.0 = [_17,_17,_17,_19.0,_19.0,_17,_17];
_19.0 = _17 * _17;
_21.0 = _9 * _5.0;
_4 = _8;
_29.0 = [_19.0,_19.0,_19.0,_17,_19.0,_17,_19.0];
_19.0 = _17 >> _5.0;
Goto(bb8)
}
bb8 = {
_27 = &(*_27);
_10 = -_6;
_32 = [110_u8,246_u8,79_u8,15_u8,134_u8,191_u8,48_u8];
_19.0 = _17 << _21.0;
(*_30) = [(-141279724237837613849496418028604115645_i128),165374988113108130029715472039336987560_i128,(-72798135758320481171939372704318568856_i128),133372931682496359024304552948800579651_i128];
_35 = _4;
Call(_24 = fn11(_32, _35, _21.0, _15, _15, _29, (*_30), _4, _21, _32), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = -_1;
Goto(bb10)
}
bb10 = {
_32 = [123_u8,242_u8,246_u8,87_u8,21_u8,203_u8,47_u8];
_11 = _3 - RET;
_31 = 263186622_i32;
_31 = (-361468691_i32);
_22 = _2 as f64;
_19.1 = 9_u8 as u128;
RET = _19.0 as isize;
_36 = _19.0;
_18 = 1875022274981286511_i64 as isize;
_37.3 = _18;
_18 = !_16;
_37.0 = _21.0;
_21 = _5;
_27 = &_25;
_9 = _5.0;
_13 = [66802210341653366077124936891370957515_i128,135972053958475716681154041481981741241_i128,16951308442322304631796542313081925966_i128,36753401615781699087983179923662963415_i128];
_37.2 = _22 as i8;
_12 = _2;
RET = false as isize;
_38 = (_8, _24);
Goto(bb11)
}
bb11 = {
_22 = 2454976981920306963_usize as f64;
_43.2 = _37.2;
Goto(bb12)
}
bb12 = {
_21.0 = _23 as u32;
_37.2 = _43.2;
_37.1 = !_37.0;
_4 = _35;
_46 = core::ptr::addr_of!(_41.2);
_33 = Adt46::Variant1 { fld0: _37.2 };
_45 = _33;
_43.0 = _9 ^ _9;
(*_30) = [(-10289419630929016253553423074016087579_i128),(-64150646955156714372421242845712720302_i128),(-39027648080457843479314967894413108043_i128),(-101285324881147958753848499841069330004_i128)];
_43.2 = !Field::<i8>(Variant(_45, 1), 0);
_40 = !_18;
SetDiscriminant(_33, 0);
_38.1 = _19.1;
place!(Field::<((u32,), f32, u32)>(Variant(_33, 0), 0)) = (_21, _2, _9);
match _31 {
0 => bb5,
1 => bb9,
2 => bb11,
340282366920938463463374607431406742765 => bb14,
_ => bb13
}
}
bb13 = {
_17 = !_19.0;
_12 = -_1;
_19.0 = _17;
_16 = _10;
(*_30) = [(-151882224689684601478056459826548106177_i128),(-75127307683554475361206960843163882654_i128),86640426326105252206638288941381520844_i128,(-64928191743186645482911185753939964040_i128)];
_15 = [156_u8,175_u8,212_u8,36_u8,228_u8,232_u8,16_u8];
_21 = (_9,);
_14.0 = [_17,_17,_17,_19.0,_19.0,_17,_17];
_19.0 = _17 * _17;
_21.0 = _9 * _5.0;
_4 = _8;
_29.0 = [_19.0,_19.0,_19.0,_17,_19.0,_17,_19.0];
_19.0 = _17 >> _5.0;
Goto(bb8)
}
bb14 = {
_13 = [(-63958805083767444330134500267391826149_i128),126639295358444776820934963341878641488_i128,46246186103679415967501904007694608475_i128,(-21586017158515772888572671350373615999_i128)];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(9_usize, 10_usize, Move(_10), 35_usize, Move(_35), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(9_usize, 37_usize, Move(_37), 18_usize, Move(_18), 14_usize, Move(_14), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(9_usize, 36_usize, Move(_36), 29_usize, Move(_29), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(9_usize, 40_usize, Move(_40), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10() -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _1: [u8; 2];
let _2: char;
let _3: (u32,);
let _4: &'static &'static isize;
let _5: f32;
let _6: ([char; 8], char);
let _7: &'static &'static isize;
let _8: (i64, *const i64, *const i64, i16);
let _9: &'static &'static ((i16, u128),);
let _10: u8;
let _11: i16;
let _12: &'static &'static ((i16, u128),);
let _13: [char; 8];
let _14: Adt46;
let _15: Adt63;
let _16: (i16, u128);
let _17: (u32,);
let _18: u128;
let _19: u64;
let _20: isize;
let _21: f64;
let _22: isize;
let _23: isize;
let _24: i16;
let _25: ((i16, u128),);
let _26: ([char; 8], char);
let _27: *const [i128; 7];
let _28: ();
let _29: ();
{
RET = [125_u8,89_u8,201_u8,87_u8,210_u8,9_u8,125_u8];
Goto(bb1)
}
bb1 = {
RET = [128_u8,33_u8,95_u8,7_u8,187_u8,197_u8,54_u8];
RET = [160_u8,44_u8,179_u8,107_u8,55_u8,170_u8,214_u8];
RET = [208_u8,134_u8,182_u8,188_u8,116_u8,144_u8,73_u8];
_1 = [97_u8,227_u8];
RET = [1_u8,65_u8,125_u8,233_u8,187_u8,148_u8,71_u8];
RET = [178_u8,13_u8,82_u8,232_u8,229_u8,219_u8,246_u8];
_3.0 = 1945868182_u32 >> 9425759609544348868_u64;
_2 = '\u{c6c7d}';
_2 = '\u{5cfea}';
_3.0 = 1466999783_u32;
_2 = '\u{4093b}';
_3.0 = 1016508310_u32;
_3.0 = 3745904638_u32;
_2 = '\u{f7610}';
_3 = (2995015851_u32,);
_2 = '\u{9017b}';
_2 = '\u{6cde6}';
_6.1 = _2;
_3.0 = 545126339_u32;
_8.3 = false as i16;
_3 = (2067705687_u32,);
match _3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2067705687 => bb10,
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
_5 = (-9223372036854775808_isize) as f32;
_10 = !212_u8;
_3 = (970796606_u32,);
_6.1 = _2;
_8.0 = 12084500304166198233_u64 as i64;
_11 = _8.3;
_5 = (-9223372036854775808_isize) as f32;
_3 = (2629327173_u32,);
_3 = (3622546602_u32,);
_2 = _6.1;
_1 = [_10,_10];
Goto(bb11)
}
bb11 = {
RET = [_10,_10,_10,_10,_10,_10,_10];
_8.0 = (-1779595378023057389_i64) + 6892389159712615832_i64;
_10 = 228_u8;
_1 = [_10,_10];
_6.0 = [_6.1,_2,_6.1,_2,_2,_2,_6.1,_6.1];
_3.0 = 2311413954_u32 >> _11;
_8.2 = core::ptr::addr_of!(_8.0);
_2 = _6.1;
_3 = (3607300459_u32,);
_8.3 = _11 + _11;
_3 = (2350551348_u32,);
_1 = [_10,_10];
_3.0 = !3386325532_u32;
_16.0 = _11;
Goto(bb12)
}
bb12 = {
_13 = [_2,_2,_2,_6.1,_6.1,_6.1,_2,_2];
_16.1 = 56767030846663531755838092273123855791_u128 & 158898359600211002262059408006277265362_u128;
_16.1 = !8245473645542739516026057237720378746_u128;
_5 = 0_usize as f32;
_11 = _8.0 as i16;
_6.0 = [_6.1,_2,_6.1,_2,_6.1,_6.1,_2,_6.1];
Goto(bb13)
}
bb13 = {
_17 = (_3.0,);
_18 = 9223372036854775807_isize as u128;
_19 = 1743248489326988729_u64 - 1555254067376721759_u64;
_1 = [_10,_10];
_17.0 = _16.1 as u32;
_18 = _16.1;
_16.0 = _8.0 as i16;
_18 = 9223372036854775807_isize as u128;
_3 = _17;
_1 = [_10,_10];
RET = [_10,_10,_10,_10,_10,_10,_10];
Call(_3.0 = core::intrinsics::bswap(_17.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_8.0 = 7154879614700916114_i64;
_8.2 = core::ptr::addr_of!(_8.0);
_6 = (_13, _2);
_13 = [_6.1,_6.1,_2,_2,_2,_6.1,_2,_6.1];
_16 = (_8.3, _18);
_24 = !_8.3;
_3 = (_17.0,);
_21 = _5 as f64;
_25 = (_16,);
_5 = _17.0 as f32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(10_usize, 24_usize, Move(_24), 6_usize, Move(_6), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(10_usize, 2_usize, Move(_2), 1_usize, Move(_1), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [u8; 7],mut _2: char,mut _3: u32,mut _4: [u8; 7],mut _5: [u8; 7],mut _6: ([i16; 7],),mut _7: [i128; 4],mut _8: char,mut _9: (u32,),mut _10: [u8; 7]) -> u128 {
mir! {
type RET = u128;
let _11: i128;
let _12: u64;
let _13: [char; 8];
let _14: (*const *const i64,);
let _15: bool;
let _16: bool;
let _17: *const (u32,);
let _18: isize;
let _19: &'static Adt53;
let _20: *const [i128; 4];
let _21: ((u32,), f32, u32);
let _22: isize;
let _23: &'static &'static *const f64;
let _24: ([i64; 8], [i128; 7], (u32,));
let _25: u64;
let _26: isize;
let _27: bool;
let _28: *mut *const [i128; 4];
let _29: i16;
let _30: &'static &'static *mut i16;
let _31: *mut *const [i128; 4];
let _32: i64;
let _33: bool;
let _34: *mut (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _35: [i32; 5];
let _36: isize;
let _37: ();
let _38: ();
{
_9.0 = _3;
RET = 162545017827747807970014131780494226216_u128 >> _3;
Goto(bb1)
}
bb1 = {
_9.0 = _3;
_2 = _8;
_6.0 = [12013_i16,(-24899_i16),(-22693_i16),17715_i16,11177_i16,26371_i16,28926_i16];
_2 = _8;
_9.0 = _3 * _3;
_9 = (_3,);
RET = !155823219721469250490342272561620013317_u128;
_4 = _10;
_11 = -(-141744841742771335191382647060323164886_i128);
_6.0 = [12304_i16,13754_i16,(-13118_i16),(-28895_i16),16525_i16,23604_i16,(-5321_i16)];
_9 = (_3,);
_17 = core::ptr::addr_of!(_9);
_16 = !true;
_12 = 17329808938221414323_u64 ^ 7329435394277763738_u64;
_17 = core::ptr::addr_of!(_9);
_1 = [16_u8,248_u8,100_u8,88_u8,88_u8,115_u8,170_u8];
_9.0 = _3;
(*_17) = (_3,);
_9 = (_3,);
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
_13 = [_8,_8,_8,_8,_8,_2,_8,_8];
_15 = _11 != _11;
(*_17).0 = _3;
_18 = (*_17).0 as isize;
_7 = [_11,_11,_11,_11];
_21.0.0 = _12 as u32;
_15 = !_16;
_13 = [_2,_8,_2,_2,_2,_2,_8,_2];
_16 = _15 & _15;
_4 = _1;
_9.0 = _3 & _21.0.0;
(*_17).0 = _3;
_1 = _5;
_7 = [_11,_11,_11,_11];
Goto(bb4)
}
bb4 = {
_18 = -(-61_isize);
(*_17).0 = _8 as u32;
_21.1 = (*_17).0 as f32;
_22 = -_18;
_2 = _8;
_7 = [_11,_11,_11,_11];
_2 = _8;
(*_17) = (_3,);
_20 = core::ptr::addr_of!(_7);
_18 = _16 as isize;
_24.1 = [_11,_11,_11,_11,_11,_11,_11];
_4 = _1;
_24.2.0 = _21.1 as u32;
RET = !64851943576932465481594924621227343789_u128;
_18 = _21.0.0 as isize;
_21.1 = 50587_u16 as f32;
_15 = !_16;
Goto(bb5)
}
bb5 = {
_9 = (_24.2.0,);
_7 = [_11,_11,_11,_11];
_1 = [29_u8,230_u8,191_u8,172_u8,77_u8,58_u8,53_u8];
_21.2 = _3 ^ _9.0;
_13 = [_2,_8,_8,_8,_8,_8,_8,_8];
_1 = [158_u8,160_u8,118_u8,223_u8,132_u8,214_u8,109_u8];
_16 = _15;
_22 = -_18;
_2 = _8;
_8 = _2;
(*_17) = (_3,);
RET = 3840128054884479035_usize as u128;
_6.0 = [(-24580_i16),12552_i16,3828_i16,(-10495_i16),4843_i16,(-1076_i16),30920_i16];
RET = 70232933997219427857987580279227178898_u128;
_11 = (-120060759662596896995863324646058780723_i128);
_29 = (-24207_i16);
_24.0 = [(-5806929346176207881_i64),(-1508669456074549190_i64),(-219768228327891404_i64),4490500793185082566_i64,(-1451928861323395727_i64),(-7052533333801031115_i64),(-2327283556019628929_i64),2232474923822296968_i64];
_17 = core::ptr::addr_of!(_9);
_10 = [147_u8,75_u8,43_u8,92_u8,160_u8,234_u8,5_u8];
_32 = (-754610578_i32) as i64;
_27 = _16;
Goto(bb6)
}
bb6 = {
_24.2.0 = _9.0 & _9.0;
_25 = RET as u64;
_18 = _29 as isize;
_24.2.0 = !_9.0;
_26 = _11 as isize;
_10 = [218_u8,62_u8,114_u8,14_u8,201_u8,129_u8,172_u8];
_29 = 11843_i16 ^ 14191_i16;
_21.0 = _9;
_10 = [80_u8,49_u8,182_u8,245_u8,234_u8,23_u8,153_u8];
(*_17) = _24.2;
match _11 {
0 => bb7,
220221607258341566467511282785709430733 => bb9,
_ => bb8
}
}
bb7 = {
_9 = (_24.2.0,);
_7 = [_11,_11,_11,_11];
_1 = [29_u8,230_u8,191_u8,172_u8,77_u8,58_u8,53_u8];
_21.2 = _3 ^ _9.0;
_13 = [_2,_8,_8,_8,_8,_8,_8,_8];
_1 = [158_u8,160_u8,118_u8,223_u8,132_u8,214_u8,109_u8];
_16 = _15;
_22 = -_18;
_2 = _8;
_8 = _2;
(*_17) = (_3,);
RET = 3840128054884479035_usize as u128;
_6.0 = [(-24580_i16),12552_i16,3828_i16,(-10495_i16),4843_i16,(-1076_i16),30920_i16];
RET = 70232933997219427857987580279227178898_u128;
_11 = (-120060759662596896995863324646058780723_i128);
_29 = (-24207_i16);
_24.0 = [(-5806929346176207881_i64),(-1508669456074549190_i64),(-219768228327891404_i64),4490500793185082566_i64,(-1451928861323395727_i64),(-7052533333801031115_i64),(-2327283556019628929_i64),2232474923822296968_i64];
_17 = core::ptr::addr_of!(_9);
_10 = [147_u8,75_u8,43_u8,92_u8,160_u8,234_u8,5_u8];
_32 = (-754610578_i32) as i64;
_27 = _16;
Goto(bb6)
}
bb8 = {
_13 = [_8,_8,_8,_8,_8,_2,_8,_8];
_15 = _11 != _11;
(*_17).0 = _3;
_18 = (*_17).0 as isize;
_7 = [_11,_11,_11,_11];
_21.0.0 = _12 as u32;
_15 = !_16;
_13 = [_2,_8,_2,_2,_2,_2,_8,_2];
_16 = _15 & _15;
_4 = _1;
_9.0 = _3 & _21.0.0;
(*_17).0 = _3;
_1 = _5;
_7 = [_11,_11,_11,_11];
Goto(bb4)
}
bb9 = {
_6.0 = [_29,_29,_29,_29,_29,_29,_29];
_9.0 = 2674255426101184583_usize as u32;
_33 = _16;
_32 = 64450_u16 as i64;
_13 = [_2,_8,_8,_2,_8,_2,_2,_8];
(*_17).0 = RET as u32;
_13 = [_8,_2,_2,_8,_2,_2,_8,_8];
_29 = 5_usize as i16;
RET = 96_u8 as u128;
_6.0 = [_29,_29,_29,_29,_29,_29,_29];
_21.0.0 = _21.2 << _3;
_9 = (_24.2.0,);
_11 = 106957283440634289259008143052899134249_i128;
_12 = !_25;
_20 = core::ptr::addr_of!((*_20));
_8 = _2;
RET = _29 as u128;
_24.2.0 = _3;
_1 = [58_u8,162_u8,104_u8,92_u8,144_u8,163_u8,26_u8];
match _11 {
0 => bb3,
1 => bb5,
2 => bb10,
3 => bb11,
106957283440634289259008143052899134249 => bb13,
_ => bb12
}
}
bb10 = {
_13 = [_8,_8,_8,_8,_8,_2,_8,_8];
_15 = _11 != _11;
(*_17).0 = _3;
_18 = (*_17).0 as isize;
_7 = [_11,_11,_11,_11];
_21.0.0 = _12 as u32;
_15 = !_16;
_13 = [_2,_8,_2,_2,_2,_2,_8,_2];
_16 = _15 & _15;
_4 = _1;
_9.0 = _3 & _21.0.0;
(*_17).0 = _3;
_1 = _5;
_7 = [_11,_11,_11,_11];
Goto(bb4)
}
bb11 = {
_18 = -(-61_isize);
(*_17).0 = _8 as u32;
_21.1 = (*_17).0 as f32;
_22 = -_18;
_2 = _8;
_7 = [_11,_11,_11,_11];
_2 = _8;
(*_17) = (_3,);
_20 = core::ptr::addr_of!(_7);
_18 = _16 as isize;
_24.1 = [_11,_11,_11,_11,_11,_11,_11];
_4 = _1;
_24.2.0 = _21.1 as u32;
RET = !64851943576932465481594924621227343789_u128;
_18 = _21.0.0 as isize;
_21.1 = 50587_u16 as f32;
_15 = !_16;
Goto(bb5)
}
bb12 = {
_24.2.0 = _9.0 & _9.0;
_25 = RET as u64;
_18 = _29 as isize;
_24.2.0 = !_9.0;
_26 = _11 as isize;
_10 = [218_u8,62_u8,114_u8,14_u8,201_u8,129_u8,172_u8];
_29 = 11843_i16 ^ 14191_i16;
_21.0 = _9;
_10 = [80_u8,49_u8,182_u8,245_u8,234_u8,23_u8,153_u8];
(*_17) = _24.2;
match _11 {
0 => bb7,
220221607258341566467511282785709430733 => bb9,
_ => bb8
}
}
bb13 = {
_31 = core::ptr::addr_of_mut!(_20);
_24.2.0 = !(*_17).0;
_18 = -_26;
_27 = !_33;
Call(_24 = fn12(_16, _22, _4, _21.0.0, _4, (*_17)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_17) = (_3,);
_21.0.0 = _9.0 | _21.2;
_10 = [83_u8,243_u8,156_u8,136_u8,175_u8,54_u8,249_u8];
_22 = !_26;
(*_17) = (_21.0.0,);
_22 = -_18;
_10 = [206_u8,104_u8,108_u8,176_u8,27_u8,152_u8,45_u8];
(*_31) = core::ptr::addr_of!((*_20));
(*_17).0 = _24.2.0;
(*_20) = [_11,_11,_11,_11];
_25 = _12;
_29 = !18515_i16;
_24.2.0 = _9.0;
_1 = _4;
RET = _18 as u128;
(*_31) = core::ptr::addr_of!(_7);
_4 = [240_u8,120_u8,24_u8,228_u8,120_u8,70_u8,186_u8];
RET = 10387627754762800313395512472075214668_u128;
_24.1 = [_11,_11,_11,_11,_11,_11,_11];
_32 = _27 as i64;
_22 = _18;
_24.2 = _21.0;
_16 = _15;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(11_usize, 33_usize, Move(_33), 24_usize, Move(_24), 4_usize, Move(_4), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(11_usize, 22_usize, Move(_22), 26_usize, Move(_26), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(11_usize, 11_usize, Move(_11), 7_usize, Move(_7), 29_usize, Move(_29), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: bool,mut _2: isize,mut _3: [u8; 7],mut _4: u32,mut _5: [u8; 7],mut _6: (u32,)) -> ([i64; 8], [i128; 7], (u32,)) {
mir! {
type RET = ([i64; 8], [i128; 7], (u32,));
let _7: f32;
let _8: i128;
let _9: &'static isize;
let _10: *const *const i64;
let _11: Adt42;
let _12: (u8, f32, &'static &'static u16);
let _13: isize;
let _14: f64;
let _15: bool;
let _16: (char, u128);
let _17: [usize; 8];
let _18: [u8; 7];
let _19: char;
let _20: *mut &'static *mut i16;
let _21: isize;
let _22: char;
let _23: [i128; 4];
let _24: &'static isize;
let _25: isize;
let _26: Adt88;
let _27: &'static &'static *mut i16;
let _28: ();
let _29: ();
{
RET.1 = [127935241291965204175638869981428704012_i128,25828539242811988397545584109675954432_i128,(-127323418391724738925804089601129170726_i128),(-113686064209644022458658411475351937063_i128),(-39872019502610448291827519237269723454_i128),(-52762413790375719991856389786033098665_i128),(-54442017073876018215831994629975613898_i128)];
_4 = _6.0;
RET.2.0 = _4;
RET.2 = _6;
_4 = _6.0;
RET.1 = [(-101912282012576151817969842851101158783_i128),26663382142675319650847771565493016702_i128,103821068829704017794784311858036451989_i128,(-97818573603427696573590682643814180991_i128),(-82521425907660734430114530010251647709_i128),(-119448036594437314834457357820947026903_i128),74219788623819425029844369269670202819_i128];
RET.2 = (_6.0,);
RET.0 = [(-3079998903477427571_i64),7521048718668194263_i64,3845842889610965075_i64,2230470537710419903_i64,(-7945787423116901343_i64),2340837537552700154_i64,5163309578061364398_i64,(-4040697566771499220_i64)];
RET.0 = [(-5781250015408438867_i64),(-5793524711512519982_i64),6622931406412584526_i64,4865132552894186374_i64,4726673334371716288_i64,(-8527165377279163116_i64),4050221009820016121_i64,(-8689987608637450043_i64)];
RET.0 = [(-4259737341506764599_i64),948377074041997269_i64,(-5351601013926295913_i64),7575191938800213441_i64,(-6645828928716057895_i64),8482549375296332316_i64,(-3347242885682034498_i64),1269994784770110294_i64];
_1 = !true;
RET.2 = (_4,);
_8 = 39902379605014282861537072906238883304_i128;
_7 = _2 as f32;
_5 = [141_u8,209_u8,221_u8,110_u8,152_u8,192_u8,250_u8];
RET.2 = _6;
_8 = 125984056064961902235499525651362435082_i128 + 159406418993831599828935487246927005515_i128;
_5 = [130_u8,6_u8,210_u8,28_u8,231_u8,29_u8,141_u8];
Goto(bb1)
}
bb1 = {
_1 = false;
RET.2 = (_6.0,);
_8 = 156703236366654209354863099082924200390_i128 * (-82294959724452864614965044665211099070_i128);
_1 = true;
_9 = &_2;
RET.0 = [(-9133083422100044807_i64),3386852509031503693_i64,843556954144609673_i64,2879260023627297963_i64,2256353100962429226_i64,5516297581302830391_i64,(-4600991320836648350_i64),(-3125163766856074928_i64)];
_7 = 10345818631067353793_u64 as f32;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
_12.1 = _7 - _7;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
RET.0 = [(-3539088511960226564_i64),(-4177237228708292583_i64),8391436742423598_i64,(-1209139603897566843_i64),(-8800923388788875490_i64),(-457748784391514300_i64),(-562875215469280376_i64),(-1450194907564365994_i64)];
_7 = 21859_i16 as f32;
_15 = _1;
RET.2.0 = (-3981629749144229894_i64) as u32;
RET.0 = [6469760748707721750_i64,5611014775823617412_i64,(-8819402618041895115_i64),4279296569703732588_i64,7710504263876374233_i64,(-8411215426158455631_i64),6751314486626840282_i64,(-8820436805078624701_i64)];
_5 = [115_u8,192_u8,136_u8,56_u8,85_u8,105_u8,64_u8];
_12.1 = -_7;
RET.2 = (_4,);
Goto(bb2)
}
bb2 = {
_8 = 26866739224913987500484391825303354148_i128;
_14 = (*_9) as f64;
_16.0 = '\u{6740c}';
_7 = _12.1 * _12.1;
RET.2 = _6;
_1 = _15;
_1 = _15;
RET.2 = (_6.0,);
_16.0 = '\u{62175}';
_19 = _16.0;
_2 = (-9223372036854775808_isize);
_14 = 136_u8 as f64;
match _8 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
26866739224913987500484391825303354148 => bb10,
_ => bb9
}
}
bb3 = {
_1 = false;
RET.2 = (_6.0,);
_8 = 156703236366654209354863099082924200390_i128 * (-82294959724452864614965044665211099070_i128);
_1 = true;
_9 = &_2;
RET.0 = [(-9133083422100044807_i64),3386852509031503693_i64,843556954144609673_i64,2879260023627297963_i64,2256353100962429226_i64,5516297581302830391_i64,(-4600991320836648350_i64),(-3125163766856074928_i64)];
_7 = 10345818631067353793_u64 as f32;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
_12.1 = _7 - _7;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
RET.0 = [(-3539088511960226564_i64),(-4177237228708292583_i64),8391436742423598_i64,(-1209139603897566843_i64),(-8800923388788875490_i64),(-457748784391514300_i64),(-562875215469280376_i64),(-1450194907564365994_i64)];
_7 = 21859_i16 as f32;
_15 = _1;
RET.2.0 = (-3981629749144229894_i64) as u32;
RET.0 = [6469760748707721750_i64,5611014775823617412_i64,(-8819402618041895115_i64),4279296569703732588_i64,7710504263876374233_i64,(-8411215426158455631_i64),6751314486626840282_i64,(-8820436805078624701_i64)];
_5 = [115_u8,192_u8,136_u8,56_u8,85_u8,105_u8,64_u8];
_12.1 = -_7;
RET.2 = (_4,);
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
_2 = -9223372036854775807_isize;
_17 = [7_usize,0_usize,18254838392034013048_usize,2897220590225426392_usize,14767724297926222735_usize,7_usize,0_usize,5_usize];
_8 = (-151295500446087411411271742188657254100_i128) | (-31701597916995645264092866131589693113_i128);
Goto(bb11)
}
bb11 = {
_13 = _8 as isize;
_17 = [5_usize,6626471114599207875_usize,17857420867772285027_usize,13900277627478177958_usize,4_usize,4_usize,12364582133109072617_usize,6345140949016094487_usize];
_16.1 = 87259888042797820156098932774253886862_u128;
_5 = _3;
RET.2 = _6;
RET.2.0 = _15 as u32;
_23 = [_8,_8,_8,_8];
_12.0 = 141_u8;
_6 = (_4,);
_21 = !_13;
_12.1 = _7 * _7;
RET.2 = _6;
_6.0 = RET.2.0 * RET.2.0;
_22 = _16.0;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
_21 = _19 as isize;
_17 = [12008475454199560533_usize,17797598186443914726_usize,7570591569186094882_usize,18230089411650490675_usize,12631492263249947199_usize,1_usize,0_usize,4_usize];
_14 = (-1987111122_i32) as f64;
_18 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_15 = _1;
match _16.1 {
0 => bb1,
1 => bb6,
2 => bb4,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
87259888042797820156098932774253886862 => bb17,
_ => bb16
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_1 = false;
RET.2 = (_6.0,);
_8 = 156703236366654209354863099082924200390_i128 * (-82294959724452864614965044665211099070_i128);
_1 = true;
_9 = &_2;
RET.0 = [(-9133083422100044807_i64),3386852509031503693_i64,843556954144609673_i64,2879260023627297963_i64,2256353100962429226_i64,5516297581302830391_i64,(-4600991320836648350_i64),(-3125163766856074928_i64)];
_7 = 10345818631067353793_u64 as f32;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
_12.1 = _7 - _7;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
RET.0 = [(-3539088511960226564_i64),(-4177237228708292583_i64),8391436742423598_i64,(-1209139603897566843_i64),(-8800923388788875490_i64),(-457748784391514300_i64),(-562875215469280376_i64),(-1450194907564365994_i64)];
_7 = 21859_i16 as f32;
_15 = _1;
RET.2.0 = (-3981629749144229894_i64) as u32;
RET.0 = [6469760748707721750_i64,5611014775823617412_i64,(-8819402618041895115_i64),4279296569703732588_i64,7710504263876374233_i64,(-8411215426158455631_i64),6751314486626840282_i64,(-8820436805078624701_i64)];
_5 = [115_u8,192_u8,136_u8,56_u8,85_u8,105_u8,64_u8];
_12.1 = -_7;
RET.2 = (_4,);
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_9 = &_13;
_8 = 108384604815988847098869198775006653311_i128;
RET.2 = (_4,);
_15 = RET.2.0 < _6.0;
RET.2 = (_6.0,);
_4 = !_6.0;
RET.2.0 = _6.0 << _13;
_22 = _19;
_4 = _13 as u32;
RET.1 = [_8,_8,_8,_8,_8,_8,_8];
_21 = _13 + (*_9);
_4 = RET.2.0 * RET.2.0;
_13 = _16.1 as isize;
_16 = (_22, 243680374355416292745371899032766517990_u128);
RET.0 = [5920678898004849296_i64,7144316550039466172_i64,(-5977883687635601351_i64),(-56075346779405984_i64),1654038995084207622_i64,(-2142085146447882377_i64),(-6490784213524281827_i64),7608840789367355478_i64];
_9 = &_25;
_21 = _2 + _2;
Goto(bb18)
}
bb18 = {
Call(_28 = dump_var(12_usize, 19_usize, Move(_19), 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(12_usize, 5_usize, Move(_5), 18_usize, Move(_18), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u32) -> [char; 4] {
mir! {
type RET = [char; 4];
let _2: f32;
let _3: [i32; 5];
let _4: *const &'static isize;
let _5: usize;
let _6: isize;
let _7: isize;
let _8: isize;
let _9: [bool; 3];
let _10: isize;
let _11: ((i16, u128),);
let _12: [char; 8];
let _13: ([char; 8], char);
let _14: u32;
let _15: isize;
let _16: Adt46;
let _17: u16;
let _18: isize;
let _19: f32;
let _20: bool;
let _21: (*const *const i64,);
let _22: [char; 4];
let _23: i32;
let _24: *const f64;
let _25: isize;
let _26: bool;
let _27: [u16; 1];
let _28: f64;
let _29: bool;
let _30: Adt60;
let _31: usize;
let _32: ();
let _33: ();
{
_1 = !3088498379_u32;
RET = ['\u{108d0a}','\u{1080d1}','\u{fcd69}','\u{e4ae5}'];
RET = ['\u{ef005}','\u{2a54b}','\u{2e51f}','\u{b5ff}'];
_2 = _1 as f32;
_3 = [(-576521408_i32),(-86199960_i32),434696095_i32,(-925659186_i32),566187444_i32];
_3 = [(-1355712439_i32),1795079138_i32,1382864099_i32,448561700_i32,155173192_i32];
RET = ['\u{f39a3}','\u{5137}','\u{62e86}','\u{130eb}'];
RET = ['\u{335bd}','\u{abcdc}','\u{1380a}','\u{107946}'];
_1 = 1825245808_u32 + 64437638_u32;
RET = ['\u{2faeb}','\u{108c38}','\u{5ba2a}','\u{8a4dc}'];
_3 = [1349086489_i32,(-857175485_i32),(-972710687_i32),1746427334_i32,(-825075744_i32)];
RET = ['\u{a8680}','\u{3f5ab}','\u{93c05}','\u{4375c}'];
RET = ['\u{1b1b2}','\u{301f4}','\u{2044a}','\u{fceac}'];
_3 = [1272118976_i32,(-1064764571_i32),(-5698368_i32),1383447131_i32,496852718_i32];
_2 = (-3687_i16) as f32;
RET = ['\u{29dc0}','\u{beeea}','\u{7a970}','\u{68085}'];
_1 = 8187159356155385285_i64 as u32;
_3 = [(-802700264_i32),507289624_i32,993701618_i32,1284416862_i32,1941406344_i32];
_3 = [39982946_i32,1358931979_i32,(-1740616548_i32),2104268864_i32,(-157907873_i32)];
_1 = 1476203238_u32;
_3 = [(-1922558628_i32),(-753026794_i32),487437888_i32,(-493161167_i32),(-755470724_i32)];
_3 = [(-1298244589_i32),721908881_i32,1171305201_i32,1155105638_i32,(-1191470212_i32)];
RET = ['\u{4bb66}','\u{adc34}','\u{990c6}','\u{99387}'];
_2 = 21213_i16 as f32;
RET = ['\u{aa409}','\u{bc413}','\u{856cf}','\u{5892f}'];
RET = ['\u{7d0cf}','\u{101cfc}','\u{c1a49}','\u{42ed6}'];
match _1 {
0 => bb1,
1 => bb2,
1476203238 => bb4,
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
_6 = 35_isize << _1;
_3 = [(-549879182_i32),979726779_i32,1255583137_i32,(-1152307093_i32),1711715493_i32];
Goto(bb5)
}
bb5 = {
RET = ['\u{cd684}','\u{4e316}','\u{25746}','\u{f7d96}'];
RET = ['\u{7e2c}','\u{7b147}','\u{da659}','\u{651c9}'];
_3 = [1400073185_i32,379342853_i32,396229021_i32,1601989093_i32,(-822021607_i32)];
_5 = 9209980978308468399_usize ^ 15082054223497818597_usize;
_6 = 245179724_i32 as isize;
_6 = !20_isize;
RET = ['\u{9a1f0}','\u{5a52b}','\u{6938f}','\u{10407f}'];
_1 = !3429035033_u32;
RET = ['\u{45bef}','\u{a8b71}','\u{e7067}','\u{debcb}'];
_6 = 9223372036854775807_isize & (-9223372036854775808_isize);
_7 = _6;
_2 = _5 as f32;
_1 = 207_u8 as u32;
_2 = 17075_i16 as f32;
_2 = (-26731_i16) as f32;
_1 = false as u32;
RET = ['\u{f61c9}','\u{fdd96}','\u{6a40d}','\u{80898}'];
Goto(bb6)
}
bb6 = {
RET = ['\u{40ab4}','\u{f8c39}','\u{9f978}','\u{884b4}'];
_8 = _6;
_5 = !3_usize;
_6 = !_8;
_7 = _8 + _6;
_9 = [false,false,true];
_3 = [974257583_i32,1059249612_i32,1325210510_i32,(-1057452130_i32),220793072_i32];
_11.0.0 = (-1358084079_i32) as i16;
_13.0 = ['\u{31d66}','\u{856af}','\u{66654}','\u{9f6a8}','\u{b64b9}','\u{fb48}','\u{62870}','\u{67d20}'];
_10 = _7;
_13.1 = '\u{12e5}';
_11.0.1 = 212567012521159758651827714121047835927_u128;
RET = [_13.1,_13.1,_13.1,_13.1];
_12 = _13.0;
_9 = [false,true,false];
_2 = 6426084110012380496_i64 as f32;
_14 = _1;
_12 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_3 = [(-1555151624_i32),(-1107095164_i32),(-1597482675_i32),(-1181785348_i32),1905271181_i32];
_13.0 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_15 = _7;
_14 = _2 as u32;
_2 = _15 as f32;
RET = [_13.1,_13.1,_13.1,_13.1];
_2 = _6 as f32;
Goto(bb7)
}
bb7 = {
_12 = _13.0;
RET = [_13.1,_13.1,_13.1,_13.1];
_1 = 1030486952_i32 as u32;
_13 = (_12, '\u{78733}');
_17 = !49636_u16;
_11.0.0 = !(-30375_i16);
_19 = _2;
_2 = 160606755150955350535052406436488730001_i128 as f32;
_19 = _11.0.0 as f32;
match _11.0.1 {
212567012521159758651827714121047835927 => bb8,
_ => bb4
}
}
bb8 = {
_17 = 49438_u16 >> _7;
_18 = _8 * _7;
_15 = _7 - _7;
_19 = _2;
_15 = -_7;
_13.1 = '\u{8086d}';
_12 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_19 = -_2;
RET = [_13.1,_13.1,_13.1,_13.1];
_10 = _15 << _5;
_20 = !true;
_17 = 53_u8 as u16;
_1 = _14;
Call(_4 = fn14(_6, _9, _11, RET, _11.0.1, _10, _9, _18, _7, _2, _6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_17 = !43756_u16;
_5 = 154_u8 as usize;
RET = [_13.1,_13.1,_13.1,_13.1];
_3 = [(-237901637_i32),(-1988514134_i32),2107269243_i32,990247198_i32,(-1922326084_i32)];
_13.1 = '\u{5a41e}';
_9 = [_20,_20,_20];
_20 = _13.1 < _13.1;
_11.0.0 = 32163_i16;
_22 = [_13.1,_13.1,_13.1,_13.1];
_11.0 = (9709_i16, 18610082610035141479831087110237434179_u128);
_10 = 4919426097024236556_u64 as isize;
_23 = 4173307257098090716_u64 as i32;
_19 = _2;
_8 = _18;
RET = [_13.1,_13.1,_13.1,_13.1];
_11.0 = (3447_i16, 91503402878907627107865816988654789008_u128);
_13.1 = '\u{60e61}';
_13.0 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_13 = (_12, '\u{e6827}');
_13 = (_12, '\u{5089f}');
_13.0 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
match _11.0.0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
3447 => bb15,
_ => bb14
}
}
bb10 = {
_17 = 49438_u16 >> _7;
_18 = _8 * _7;
_15 = _7 - _7;
_19 = _2;
_15 = -_7;
_13.1 = '\u{8086d}';
_12 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_19 = -_2;
RET = [_13.1,_13.1,_13.1,_13.1];
_10 = _15 << _5;
_20 = !true;
_17 = 53_u8 as u16;
_1 = _14;
Call(_4 = fn14(_6, _9, _11, RET, _11.0.1, _10, _9, _18, _7, _2, _6), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_12 = _13.0;
RET = [_13.1,_13.1,_13.1,_13.1];
_1 = 1030486952_i32 as u32;
_13 = (_12, '\u{78733}');
_17 = !49636_u16;
_11.0.0 = !(-30375_i16);
_19 = _2;
_2 = 160606755150955350535052406436488730001_i128 as f32;
_19 = _11.0.0 as f32;
match _11.0.1 {
212567012521159758651827714121047835927 => bb8,
_ => bb4
}
}
bb12 = {
RET = ['\u{40ab4}','\u{f8c39}','\u{9f978}','\u{884b4}'];
_8 = _6;
_5 = !3_usize;
_6 = !_8;
_7 = _8 + _6;
_9 = [false,false,true];
_3 = [974257583_i32,1059249612_i32,1325210510_i32,(-1057452130_i32),220793072_i32];
_11.0.0 = (-1358084079_i32) as i16;
_13.0 = ['\u{31d66}','\u{856af}','\u{66654}','\u{9f6a8}','\u{b64b9}','\u{fb48}','\u{62870}','\u{67d20}'];
_10 = _7;
_13.1 = '\u{12e5}';
_11.0.1 = 212567012521159758651827714121047835927_u128;
RET = [_13.1,_13.1,_13.1,_13.1];
_12 = _13.0;
_9 = [false,true,false];
_2 = 6426084110012380496_i64 as f32;
_14 = _1;
_12 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_3 = [(-1555151624_i32),(-1107095164_i32),(-1597482675_i32),(-1181785348_i32),1905271181_i32];
_13.0 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_15 = _7;
_14 = _2 as u32;
_2 = _15 as f32;
RET = [_13.1,_13.1,_13.1,_13.1];
_2 = _6 as f32;
Goto(bb7)
}
bb13 = {
Return()
}
bb14 = {
_6 = 35_isize << _1;
_3 = [(-549879182_i32),979726779_i32,1255583137_i32,(-1152307093_i32),1711715493_i32];
Goto(bb5)
}
bb15 = {
_27 = [_17];
_12 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_26 = _13.1 > _13.1;
_6 = -_8;
_22 = [_13.1,_13.1,_13.1,_13.1];
_6 = _7 >> _11.0.1;
_26 = _5 == _5;
_13.0 = [_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1,_13.1];
_26 = _20;
_22 = RET;
_12 = _13.0;
_8 = _18 << _18;
_17 = _13.1 as u16;
_24 = core::ptr::addr_of!(_28);
_29 = _5 == _5;
_25 = _8 ^ _18;
_13.1 = '\u{26336}';
_16 = Adt46::Variant1 { fld0: (-109_i8) };
_30.fld2 = core::ptr::addr_of!(_30.fld4);
_30.fld6 = !_5;
place!(Field::<i8>(Variant(_16, 1), 0)) = _11.0.1 as i8;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(13_usize, 25_usize, Move(_25), 18_usize, Move(_18), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(13_usize, 11_usize, Move(_11), 7_usize, Move(_7), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(13_usize, 20_usize, Move(_20), 5_usize, Move(_5), 17_usize, Move(_17), 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: [bool; 3],mut _3: ((i16, u128),),mut _4: [char; 4],mut _5: u128,mut _6: isize,mut _7: [bool; 3],mut _8: isize,mut _9: isize,mut _10: f32,mut _11: isize) -> *const &'static isize {
mir! {
type RET = *const &'static isize;
let _12: isize;
let _13: [char; 4];
let _14: char;
let _15: *mut *const i64;
let _16: &'static (*const *const i64,);
let _17: isize;
let _18: *mut (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _19: u128;
let _20: u16;
let _21: isize;
let _22: f32;
let _23: *const &'static isize;
let _24: [i128; 4];
let _25: i16;
let _26: *mut i16;
let _27: [u8; 7];
let _28: ([char; 8], char);
let _29: &'static &'static (u32,);
let _30: i16;
let _31: f32;
let _32: f64;
let _33: u32;
let _34: [u8; 2];
let _35: isize;
let _36: *mut u8;
let _37: u64;
let _38: isize;
let _39: u64;
let _40: *const (u32,);
let _41: [usize; 8];
let _42: i64;
let _43: &'static isize;
let _44: i64;
let _45: u64;
let _46: u32;
let _47: &'static u16;
let _48: u8;
let _49: *const [i128; 4];
let _50: &'static u16;
let _51: f32;
let _52: i32;
let _53: &'static (u32,);
let _54: Adt58;
let _55: ();
let _56: ();
{
_8 = _9;
_4 = ['\u{10e025}','\u{1724}','\u{743ac}','\u{5a7e5}'];
_11 = !_8;
_5 = 180_u8 as u128;
_12 = _11;
_3.0.1 = _5;
_7 = [false,false,false];
_3.0 = (13811_i16, _5);
_1 = _12 * _9;
_2 = _7;
_1 = _12;
_4 = ['\u{14032}','\u{b398a}','\u{6d6fa}','\u{113b7}'];
_1 = 14754641340521524708_u64 as isize;
_10 = _3.0.0 as f32;
_3.0.0 = (-1092725850_i32) as i16;
_12 = _8 >> _8;
_3.0.0 = (-18228_i16);
_10 = 2017799552_i32 as f32;
_3.0.1 = _5 * _5;
_10 = 2_usize as f32;
Call(_13 = fn15(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _12 >> _12;
_14 = '\u{fdbd}';
_3.0 = ((-10694_i16), _5);
_5 = _3.0.1;
_7 = [false,true,true];
_4 = _13;
Goto(bb2)
}
bb2 = {
_4 = [_14,_14,_14,_14];
_3.0.1 = !_5;
_11 = _12 * _6;
_3.0.1 = _5 << _11;
_3.0.0 = 62098_u16 as i16;
_10 = 48_u8 as f32;
_11 = _10 as isize;
_3.0.1 = _10 as u128;
_5 = 13426611766235416325_usize as u128;
_4 = _13;
_19 = _3.0.1;
_20 = !28220_u16;
_9 = -_6;
_4 = _13;
Goto(bb3)
}
bb3 = {
_7 = _2;
_7 = _2;
_20 = 24732_u16;
_7 = [false,true,false];
_17 = 7_usize as isize;
_20 = 48_u16 * 45862_u16;
_6 = _9 ^ _9;
_10 = (-28_i8) as f32;
_11 = false as isize;
_10 = 1939752279_u32 as f32;
_17 = _6;
_20 = !10394_u16;
_21 = 219_u8 as isize;
_17 = -_9;
_6 = _17 * _12;
_10 = (-124898896559221098430635805209111816936_i128) as f32;
_24 = [(-100350658406757552372167186504421307960_i128),107261742177129394340533455804251586778_i128,78845783430700676170499823178004325273_i128,20978078848581553871923298636382294962_i128];
_3.0.0 = !(-22136_i16);
_1 = 11662268871527094387_u64 as isize;
_20 = !43237_u16;
Call(_11 = core::intrinsics::transmute(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = '\u{f712c}';
_8 = _14 as isize;
_6 = !_8;
_12 = !_9;
_12 = _9 | _17;
_24 = [(-97766329071872983159405314103104793599_i128),95116523497882798665416759649472491057_i128,54041358152836316487019612797783799318_i128,41062390804051479549931331811953246425_i128];
_3.0.0 = (-32378_i16) + (-2524_i16);
_11 = _12 ^ _9;
_1 = _11;
_20 = 52_i8 as u16;
_25 = true as i16;
_5 = 6032505484811954260_u64 as u128;
_26 = core::ptr::addr_of_mut!(_3.0.0);
_12 = _14 as isize;
_17 = _11 ^ _11;
_9 = _17 + _11;
Goto(bb5)
}
bb5 = {
_27 = [26_u8,57_u8,23_u8,64_u8,2_u8,172_u8,204_u8];
_9 = !_1;
_9 = _5 as isize;
_2 = [true,false,true];
_8 = _17;
_12 = _1;
_26 = core::ptr::addr_of_mut!((*_26));
_10 = _19 as f32;
_21 = _1 - _1;
_6 = !_17;
_17 = _8 ^ _8;
_19 = _5;
_7 = _2;
Goto(bb6)
}
bb6 = {
_28.0 = [_14,_14,_14,_14,_14,_14,_14,_14];
_8 = !_6;
_21 = _10 as isize;
_19 = !_3.0.1;
_22 = _10 * _10;
_13 = [_14,_14,_14,_14];
_6 = 7447606891370262514_i64 as isize;
_27 = [249_u8,73_u8,124_u8,52_u8,140_u8,4_u8,134_u8];
_3.0 = (_25, _19);
_30 = !_25;
Call(_3.0.0 = core::intrinsics::transmute(_20), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = _13;
_25 = -_30;
_3.0.1 = !_19;
_31 = (*_26) as f32;
_6 = _8;
_28.1 = _14;
_5 = _3.0.1;
_13 = _4;
_32 = _20 as f64;
(*_26) = _30 & _25;
_12 = -_11;
_10 = -_31;
_20 = 3971_u16 | 10365_u16;
_20 = 51363_u16;
_1 = !_8;
_12 = _17 >> _11;
_7 = _2;
_8 = _12 >> _17;
_13 = _4;
_1 = !_6;
(*_26) = !_25;
_25 = -_3.0.0;
_34 = [251_u8,174_u8];
Goto(bb8)
}
bb8 = {
_17 = _11 - _8;
_3.0.0 = _30 & _25;
_10 = _31;
_21 = (*_26) as isize;
_14 = _28.1;
_17 = _12 << _1;
_20 = false as u16;
(*_26) = _30 >> _1;
_13 = [_14,_14,_14,_14];
_35 = !_8;
_28.0 = [_14,_28.1,_28.1,_14,_14,_14,_28.1,_28.1];
_7 = [false,false,false];
Goto(bb9)
}
bb9 = {
_3.0 = (_25, _19);
_27 = [101_u8,160_u8,182_u8,13_u8,50_u8,12_u8,23_u8];
_22 = _31 - _10;
_4 = _13;
Goto(bb10)
}
bb10 = {
_17 = _12;
_17 = _35;
_6 = _1;
_35 = _11 * _12;
_39 = 5398726033865381765_u64 & 6871894862565147471_u64;
_32 = 1069786053_i32 as f64;
_38 = -_11;
_25 = (*_26);
_11 = _30 as isize;
_9 = _35 - _17;
_37 = !_39;
_17 = _1 >> _1;
_25 = _32 as i16;
_32 = (-2769803609540267643_i64) as f64;
_38 = _28.1 as isize;
_33 = !3594074717_u32;
_28.1 = _14;
_2 = _7;
_5 = _19 & _19;
_30 = _33 as i16;
_3.0.0 = (-11_i8) as i16;
_38 = _32 as isize;
Call(_28.0 = fn16(_12, _12, _17, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_26) = _30 - _25;
_1 = _8 * _12;
_37 = _39 & _39;
_20 = _6 as u16;
(*_26) = _30;
RET = core::ptr::addr_of!(_43);
(*RET) = &_12;
(*RET) = &_21;
Goto(bb12)
}
bb12 = {
_24 = [(-100690539582593867266658517567827663993_i128),149340536819775501490010749831498629469_i128,(-70522608290496285275829495194454426010_i128),(-56213131752773297656962645921388145813_i128)];
(*RET) = &_6;
_44 = (-1821120081126220703_i64) * (-3970296049676906743_i64);
_31 = _22 * _22;
(*RET) = &_1;
_6 = _22 as isize;
(*RET) = &_21;
_25 = -(*_26);
_1 = _8;
_8 = _12;
_32 = _9 as f64;
(*RET) = &_17;
(*_26) = !_25;
(*_26) = _25 << _1;
(*RET) = &_8;
(*_26) = !_25;
_28.0 = [_28.1,_28.1,_28.1,_28.1,_28.1,_28.1,_14,_28.1];
_2 = _7;
_47 = &_20;
RET = core::ptr::addr_of!(_43);
_23 = Move(RET);
Call(_10 = fn17(Move(_23), Move(_47), _1, (*_47), _12, _8, _32, (*_43), (*_43), _32), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3.0 = (_25, _5);
_3.0.1 = _10 as u128;
_35 = _1;
(*_26) = _37 as i16;
_22 = _10 - _10;
_30 = !(*_26);
RET = core::ptr::addr_of!(_43);
_49 = core::ptr::addr_of!(_24);
_36 = core::ptr::addr_of_mut!(_48);
_1 = !_17;
_45 = _37;
(*_36) = !206_u8;
(*_26) = false as i16;
_34 = [(*_36),(*_36)];
_33 = !1700173328_u32;
(*RET) = &_38;
_51 = -_10;
_4 = [_28.1,_28.1,_28.1,_28.1];
_5 = !_3.0.1;
(*_49) = [50988088638079959304714379182382100747_i128,27939507527459648196062504697549886519_i128,(-91991492486968935865459338025298602569_i128),120338978335117397150707540134849949425_i128];
(*_49) = [(-115163865378559627189301996248441611259_i128),(-76697629202003837024185608795585345053_i128),(-92064309143281239352233391189841960924_i128),(-19572680041967595374458832747396046738_i128)];
_46 = _33;
_4 = [_14,_28.1,_14,_28.1];
_47 = &_20;
_54.fld0 = _12 as usize;
_31 = _22 + _22;
(*_26) = _28.1 as i16;
Goto(bb14)
}
bb14 = {
Call(_55 = dump_var(14_usize, 17_usize, Move(_17), 5_usize, Move(_5), 25_usize, Move(_25), 21_usize, Move(_21)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_55 = dump_var(14_usize, 11_usize, Move(_11), 6_usize, Move(_6), 27_usize, Move(_27), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(14_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 44_usize, Move(_44)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(14_usize, 7_usize, Move(_7), 48_usize, Move(_48), 24_usize, Move(_24), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15() -> [char; 4] {
mir! {
type RET = [char; 4];
let _1: (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _2: i16;
let _3: isize;
let _4: *const [u8; 2];
let _5: i32;
let _6: u32;
let _7: [i32; 5];
let _8: ((i16, u128),);
let _9: i16;
let _10: u64;
let _11: *mut *const i64;
let _12: *const *const i64;
let _13: bool;
let _14: bool;
let _15: (Adt42, i16, [char; 4]);
let _16: [u16; 1];
let _17: &'static &'static ((i16, u128),);
let _18: &'static *const f64;
let _19: (((u32,), f32, u32), (i16, u128), ([char; 8], char), ([i64; 8], [i128; 7], (u32,)));
let _20: [i128; 4];
let _21: u8;
let _22: *const [u8; 2];
let _23: [u8; 7];
let _24: i128;
let _25: [i128; 7];
let _26: [u8; 7];
let _27: f64;
let _28: Adt59;
let _29: ([i16; 7],);
let _30: ();
let _31: ();
{
RET = ['\u{338c1}','\u{89f35}','\u{a003c}','\u{f3c07}'];
RET = ['\u{82d67}','\u{83ce3}','\u{dd55d}','\u{2ba39}'];
_1.3.2.0 = 4225502749_u32 ^ 850311988_u32;
_1.1.1 = 66098721489370902188930951277974480181_u128;
_1.1 = ((-6343_i16), 105150278219302518018172700667661295999_u128);
_2 = !_1.1.0;
_1.0.2 = !_1.3.2.0;
_1.0.1 = _2 as f32;
_1.3.2.0 = _1.0.2 * _1.0.2;
_3 = (-9223372036854775808_isize) - (-51_isize);
_1.0.0 = (_1.3.2.0,);
match _1.1.1 {
0 => bb1,
1 => bb2,
105150278219302518018172700667661295999 => bb4,
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
_1.2.0 = ['\u{1bda6}','\u{924e5}','\u{4e203}','\u{86e6a}','\u{120dd}','\u{a70af}','\u{fc106}','\u{f7b4b}'];
_1.2.1 = '\u{660c1}';
_5 = (-2078243412_i32) * (-1522934425_i32);
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.3.2.0 = _1.0.2 + _1.0.0.0;
_5 = 1648206960_i32;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.1.1 = !287855346262359607337378348917420052438_u128;
_1.3.1 = [98421779713225027684273637296486326958_i128,(-28571155216383482954061036618704997615_i128),88283824891510205861597364030022078980_i128,(-139475601695186086986481857477453291090_i128),(-56532671666863331905810291874452622481_i128),(-106269634433853818322664402802038630034_i128),112723477883237321787993320443179236727_i128];
_1.2.1 = '\u{dedbb}';
_1.0.1 = _1.3.2.0 as f32;
_1.0.0.0 = 138866669036131365197118968113439842398_i128 as u32;
_1.1 = (_2, 24382649044723062949922313228266200178_u128);
_6 = _1.0.0.0 >> _1.1.1;
_1.1.0 = _2 + _2;
_1.3.0 = [5587475386170249222_i64,(-2265813407901756486_i64),(-1378432961804689194_i64),(-3157521800268893075_i64),1769111807479468825_i64,(-4222629043175839748_i64),(-764304738808500865_i64),3752695368077495015_i64];
_1.1.0 = _2;
_1.0.0.0 = _6 & _6;
_1.0.0.0 = _1.3.2.0;
_1.0.2 = _6;
_1.3.2 = (_6,);
Goto(bb5)
}
bb5 = {
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_7 = [_5,_5,_5,_5,_5];
_1.3.1 = [165929290049670089233086548373948112477_i128,124310713457198841806454718251133322677_i128,(-51211398023062940311642698930741779162_i128),157539752550158553503331906984615257656_i128,(-147603498727547225808464876491030115394_i128),110926116784234097265368145881793693472_i128,64453055258752396613386552179715624904_i128];
_8 = (_1.1,);
_1.1.1 = _8.0.1 + _8.0.1;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_5 = 1318730666914743499_i64 as i32;
_7 = [_5,_5,_5,_5,_5];
_1.1.0 = _2 << _6;
_1.3.1 = [(-128320677783844263467770460244008545206_i128),46197228373945152575888662819587407556_i128,(-33488677202438634128908029184446428023_i128),(-122899613521796541078763215817860668464_i128),(-88346757180532070095743438342240960383_i128),158592605365544766236163579046329828032_i128,(-1047201910615281496966808281003426592_i128)];
_1.3.0 = [5112181299714559483_i64,6003084162812897418_i64,(-5659632980609266664_i64),7102312591669978058_i64,(-1268945022495549499_i64),(-3143475670599706751_i64),(-2149900227453967887_i64),(-1237197875316132395_i64)];
_7 = [_5,_5,_5,_5,_5];
_5 = 18204400923588525772_u64 as i32;
_3 = -9223372036854775807_isize;
_1.3.2 = (_1.0.2,);
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_6 = _1.3.2.0 & _1.0.0.0;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_2 = !_8.0.0;
_1.2.0 = [_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.3.0 = [5274685178559466875_i64,(-6271727952801878979_i64),(-9196260404133729737_i64),(-5867009356790872279_i64),(-2491083981523034314_i64),(-1661463343041416067_i64),187578616212236517_i64,(-774075420288887659_i64)];
_8.0.0 = _2;
Goto(bb6)
}
bb6 = {
_1.1.0 = !_8.0.0;
_8.0.0 = _1.1.0;
_1.0.2 = _1.3.2.0 | _6;
_1.1.1 = !_8.0.1;
_1.0.0 = (_1.3.2.0,);
_1.2.0 = [_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_5 = (-1189097965_i32);
_1.3.2.0 = _1.0.0.0;
_1.1.1 = _8.0.1 << _1.1.0;
_1.0.2 = !_6;
_2 = _8.0.0;
_1.3.2.0 = _1.0.0.0 | _6;
_1.1.0 = _8.0.0;
_1.2.1 = '\u{c1c7d}';
_1.2.0 = [_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.1.1 = _8.0.1;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.1.0 = -_8.0.0;
_5 = -608690773_i32;
Goto(bb7)
}
bb7 = {
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_8 = (_1.1,);
_1.3.2 = (_1.0.0.0,);
_8 = (_1.1,);
_3 = -82_isize;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_3 = _1.0.1 as isize;
_1.0.0 = (_1.3.2.0,);
_8.0.0 = _2 * _2;
_2 = _3 as i16;
_10 = 13305226889308341702_u64;
_1.0.2 = _8.0.1 as u32;
Goto(bb8)
}
bb8 = {
_8 = (_1.1,);
_1.3.0 = [4609681980330119985_i64,4489562666744444359_i64,794199709719546327_i64,(-9027065668833858194_i64),(-3135867146951383019_i64),(-6216478127532389349_i64),(-3351426614145286385_i64),(-846240083831653723_i64)];
_1.0.0 = (_6,);
_9 = _5 as i16;
_1.3.2 = (_1.0.0.0,);
_1.2.0 = [_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_1.0.0 = (_6,);
_1.0.1 = 210_u8 as f32;
_6 = 104_i8 as u32;
_1.0.2 = _1.0.0.0 << _1.1.1;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_13 = true;
_8.0.1 = (-58731356180876644189848166045816987478_i128) as u128;
_8.0 = (_2, _1.1.1);
_14 = !_13;
Goto(bb9)
}
bb9 = {
_1.3.0 = [(-5378235990173776134_i64),7658017622167319191_i64,(-3318491386608245319_i64),1340045287285912416_i64,8960912131543085184_i64,(-2255359538651845348_i64),(-3492048069889084576_i64),2650890975881279282_i64];
_9 = _8.0.0 ^ _2;
_16 = [6794_u16];
_1.3.1 = [61045942688611430608180371907872009781_i128,95122581722838673668460426618530443267_i128,(-74572802125068705313750314270258372953_i128),(-95788777107358130256548997415293426252_i128),(-108105504731643251850316840960526531325_i128),(-162340428163792447367316999975540157445_i128),(-143990816906321015093402611638281995829_i128)];
_1.2.1 = '\u{ac997}';
_13 = _14;
_15.2 = RET;
_9 = _8.0.0;
_1.1.0 = 212_u8 as i16;
_7 = [_5,_5,_5,_5,_5];
_1.1 = (_8.0.0, _8.0.1);
_8.0.0 = -_1.1.0;
_1.1 = (_9, _8.0.1);
_8.0 = _1.1;
_1.3.2 = (_1.0.0.0,);
_5 = 1164640259_i32 + 889787617_i32;
_2 = !_8.0.0;
_1.3.0 = [(-1026644690167625476_i64),(-1618728372779393879_i64),(-317422734878746806_i64),7935220571361584685_i64,2141924977426809365_i64,(-7061665070100534221_i64),(-3394529331913759474_i64),1256382319366440278_i64];
_1.2.1 = '\u{8cbc}';
_1.2.1 = '\u{185f2}';
_16 = [33520_u16];
_15.1 = _13 as i16;
_1.1.0 = 7_usize as i16;
Goto(bb10)
}
bb10 = {
_14 = _13 | _13;
_1.2.1 = '\u{a1daa}';
_5 = 1582082752_i32 - 445544005_i32;
_19.3 = (_1.3.0, _1.3.1, _1.0.0);
_19.1.1 = !_1.1.1;
_19.2.0 = _1.2.0;
_19.0.1 = 237_u8 as f32;
_19.3.1 = _1.3.1;
_1.1.0 = _9 * _2;
_19.0.0 = (_1.0.0.0,);
_19.3.2.0 = _1.3.2.0;
_3 = _14 as isize;
_19.2 = (_1.2.0, _1.2.1);
match _10 {
0 => bb1,
1 => bb8,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb11,
13305226889308341702 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_8 = (_1.1,);
_1.3.2 = (_1.0.0.0,);
_8 = (_1.1,);
_3 = -82_isize;
RET = [_1.2.1,_1.2.1,_1.2.1,_1.2.1];
_3 = _1.0.1 as isize;
_1.0.0 = (_1.3.2.0,);
_8.0.0 = _2 * _2;
_2 = _3 as i16;
_10 = 13305226889308341702_u64;
_1.0.2 = _8.0.1 as u32;
Goto(bb8)
}
bb13 = {
_14 = !_13;
_15.1 = _1.1.0 * _9;
_8.0 = (_15.1, _19.1.1);
_19.0.2 = _19.3.2.0;
_8.0 = (_15.1, _1.1.1);
_19 = _1;
_19 = _1;
_21 = !4_u8;
_19.2.0 = [_1.2.1,_19.2.1,_19.2.1,_19.2.1,_1.2.1,_19.2.1,_19.2.1,_1.2.1];
_1.1.0 = _15.1;
_8.0.0 = !_19.1.0;
_24 = 166882946787934538587250418115797801021_i128;
_19.3.2.0 = _1.3.2.0;
Goto(bb14)
}
bb14 = {
_19.0 = (_19.3.2, _1.0.1, _1.3.2.0);
_1.0.1 = _19.0.1;
RET = [_1.2.1,_1.2.1,_1.2.1,_19.2.1];
_8.0 = _1.1;
_23 = [_21,_21,_21,_21,_21,_21,_21];
_1.2.1 = _19.2.1;
_1.3.2.0 = !_6;
_19.1.0 = _8.0.0 * _15.1;
_1.3.1 = [_24,_24,_24,_24,_24,_24,_24];
_19.2.0 = [_1.2.1,_1.2.1,_1.2.1,_1.2.1,_19.2.1,_1.2.1,_19.2.1,_19.2.1];
_26 = [_21,_21,_21,_21,_21,_21,_21];
_1.0.2 = _24 as u32;
_1.1.1 = _8.0.1;
_1.0.1 = _19.0.1 + _19.0.1;
_14 = !_13;
_19.1.0 = _15.1;
_19.0.0.0 = _14 as u32;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(15_usize, 9_usize, Move(_9), 26_usize, Move(_26), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(15_usize, 13_usize, Move(_13), 2_usize, Move(_2), 24_usize, Move(_24), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> [char; 8] {
mir! {
type RET = [char; 8];
let _5: char;
let _6: ((i16, u128),);
let _7: char;
let _8: ([i16; 7],);
let _9: &'static &'static *const f64;
let _10: i32;
let _11: ();
let _12: ();
{
_2 = _1;
RET = ['\u{7d374}','\u{3a02}','\u{cd446}','\u{d7c18}','\u{79cd3}','\u{99398}','\u{b2c61}','\u{49512}'];
_2 = _3;
_3 = _4;
RET = ['\u{8a389}','\u{732f5}','\u{c58e9}','\u{449f4}','\u{33faa}','\u{f6807}','\u{94673}','\u{63b27}'];
Goto(bb1)
}
bb1 = {
_1 = _2;
_5 = '\u{1c316}';
_4 = !_2;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_6.0.0 = 17825_i16 | 17658_i16;
_6.0 = ((-29511_i16), 178867107535926782665149139845263120828_u128);
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
match _6.0.1 {
0 => bb2,
1 => bb3,
178867107535926782665149139845263120828 => bb5,
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
_4 = -_3;
_7 = _5;
_2 = !_4;
_4 = 49831_u16 as isize;
_4 = !_1;
_1 = _3 & _4;
_5 = _7;
_6.0.0 = (-64901966999192223107051122545588439528_i128) as i16;
RET = [_7,_7,_7,_7,_7,_7,_5,_7];
_6.0.0 = 31013_i16;
_6.0 = (20315_i16, 317375682728078335093521738022364836113_u128);
match _6.0.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
317375682728078335093521738022364836113 => bb13,
_ => bb12
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
_1 = _2;
_5 = '\u{1c316}';
_4 = !_2;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_6.0.0 = 17825_i16 | 17658_i16;
_6.0 = ((-29511_i16), 178867107535926782665149139845263120828_u128);
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
match _6.0.1 {
0 => bb2,
1 => bb3,
178867107535926782665149139845263120828 => bb5,
_ => bb4
}
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
_5 = _7;
_2 = _4;
_3 = _1;
RET = [_5,_5,_5,_5,_7,_7,_5,_7];
_5 = _7;
RET = [_7,_5,_7,_5,_5,_7,_5,_7];
RET = [_7,_7,_5,_5,_7,_7,_5,_5];
_3 = _4;
RET = [_5,_5,_5,_7,_7,_7,_7,_5];
RET = [_5,_5,_7,_5,_7,_5,_5,_7];
_8.0 = [_6.0.0,_6.0.0,_6.0.0,_6.0.0,_6.0.0,_6.0.0,_6.0.0];
RET = [_5,_5,_5,_5,_7,_7,_5,_7];
_5 = _7;
_3 = _2;
RET = [_5,_7,_5,_7,_5,_7,_7,_7];
_7 = _5;
Goto(bb14)
}
bb14 = {
RET = [_5,_7,_7,_5,_7,_5,_7,_7];
Goto(bb15)
}
bb15 = {
Call(_11 = dump_var(16_usize, 4_usize, Move(_4), 2_usize, Move(_2), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *const &'static isize,mut _2: &'static u16,mut _3: isize,mut _4: u16,mut _5: isize,mut _6: isize,mut _7: f64,mut _8: isize,mut _9: isize,mut _10: f64) -> f32 {
mir! {
type RET = f32;
let _11: bool;
let _12: &'static (u32,);
let _13: [i16; 7];
let _14: Adt53;
let _15: ();
let _16: ();
{
_3 = _6;
Goto(bb1)
}
bb1 = {
RET = _7 as f32;
RET = (-874132099_i32) as f32;
RET = (-28238_i16) as f32;
_2 = &_4;
_9 = _3;
_5 = _9 ^ _9;
_3 = _9 << (*_2);
_9 = _6;
_3 = _5 - _5;
_5 = (-42313457637262998806313414474900735949_i128) as isize;
_3 = !_9;
_7 = 4991975098720240377851335741056831560_i128 as f64;
_8 = _3;
_11 = _4 <= _4;
_3 = !_8;
RET = (-11_i8) as f32;
_10 = _7;
_10 = _7;
RET = 6360102649420508527_u64 as f32;
RET = 1371243513_i32 as f32;
RET = (*_2) as f32;
_2 = &(*_2);
_11 = false;
_12 = &_14.fld7.2;
_13 = [11348_i16,19790_i16,(-6106_i16),31993_i16,(-21401_i16),(-26610_i16),(-14588_i16)];
_14.fld2.1 = 198550236_u32;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(17_usize, 3_usize, Move(_3), 5_usize, Move(_5), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static *const f64,mut _2: f64,mut _3: Adt42) -> usize {
mir! {
type RET = usize;
let _4: Adt60;
let _5: ((i16, u128),);
let _6: i8;
let _7: f64;
let _8: &'static i8;
let _9: Adt63;
let _10: bool;
let _11: usize;
let _12: isize;
let _13: *mut *const i64;
let _14: [i32; 5];
let _15: i8;
let _16: f32;
let _17: f32;
let _18: ();
let _19: ();
{
RET = Field::<i16>(Variant(_3, 2), 0) as usize;
place!(Field::<u16>(Variant(_3, 2), 2)) = Field::<i16>(Variant(_3, 2), 0) as u16;
place!(Field::<i16>(Variant(_3, 2), 0)) = 1924_i16;
match Field::<i16>(Variant(_3, 2), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
1924 => bb8,
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
_4.fld5 = core::ptr::addr_of_mut!(_4.fld0.0);
_4.fld4 = (2407056595_u32,);
_4.fld3.1 = _2 as u128;
_5.0 = (Field::<i16>(Variant(_3, 2), 0), _4.fld3.1);
_4.fld0.0 = Field::<i16>(Variant(_3, 2), 0);
_4.fld0 = (Field::<i16>(Variant(_3, 2), 0), _5.0.1);
_5.0 = (Field::<i16>(Variant(_3, 2), 0), _4.fld0.1);
_2 = 6188154950567941675_u64 as f64;
_4.fld6 = RET;
_2 = _4.fld4.0 as f64;
RET = false as usize;
_4.fld3 = ('\u{70dec}', _4.fld0.1);
_4.fld3.0 = '\u{a898}';
_4.fld3 = ('\u{dc6b3}', _4.fld0.1);
_4.fld3.0 = '\u{c30f2}';
_4.fld4.0 = 1162106619_u32 << _4.fld3.1;
_5.0 = (Field::<i16>(Variant(_3, 2), 0), _4.fld0.1);
_7 = _2 * _2;
RET = _4.fld6 & _4.fld6;
_5.0 = (Field::<i16>(Variant(_3, 2), 0), _4.fld3.1);
Goto(bb9)
}
bb9 = {
RET = _4.fld6;
place!(Field::<i64>(Variant(_3, 2), 1)) = 6528786780064981192_i64;
place!(Field::<i64>(Variant(_3, 2), 1)) = (-8587887592089984211_i64) ^ (-5046468348455434797_i64);
_2 = Field::<i64>(Variant(_3, 2), 1) as f64;
Goto(bb10)
}
bb10 = {
_8 = &_6;
_4.fld0 = (Field::<i16>(Variant(_3, 2), 0), _5.0.1);
place!(Field::<i16>(Variant(_3, 2), 0)) = _5.0.0;
_5.0.0 = 17_i8 as i16;
_8 = &_6;
RET = _5.0.1 as usize;
_4.fld7 = 156166456368844036804234671478167062835_i128 >> _5.0.0;
_4.fld3.0 = '\u{42b9}';
RET = _4.fld6;
_10 = false;
_4.fld4 = (223927777_u32,);
SetDiscriminant(_3, 2);
place!(Field::<i64>(Variant(_3, 2), 1)) = -4613202497427273423_i64;
place!(Field::<u16>(Variant(_3, 2), 2)) = 8446_u16;
_4.fld0 = _5.0;
_6 = Field::<u16>(Variant(_3, 2), 2) as i8;
RET = _4.fld6;
_12 = -(-9223372036854775808_isize);
match _4.fld4.0 {
0 => bb5,
223927777 => bb11,
_ => bb2
}
}
bb11 = {
_5 = (_4.fld0,);
_11 = !_4.fld6;
_12 = 113_isize;
place!(Field::<i16>(Variant(_3, 2), 0)) = _4.fld0.0;
_7 = _2;
_4.fld7 = !(-111104962641951001866383582338326457922_i128);
_4.fld3 = ('\u{1ad64}', _4.fld0.1);
_4.fld2 = core::ptr::addr_of!(_4.fld4);
_8 = &_6;
_10 = !true;
SetDiscriminant(_3, 2);
_5.0 = (_4.fld0.0, _4.fld3.1);
_4.fld4.0 = 1729486596_u32;
place!(Field::<u16>(Variant(_3, 2), 2)) = _4.fld6 as u16;
match _4.fld4.0 {
0 => bb1,
1729486596 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
Call(_5.0.1 = fn19(Move(_8), _10, _4.fld3.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<i64>(Variant(_3, 2), 1)) = 443150083887658143_i64;
_15 = _10 as i8;
place!(Field::<i16>(Variant(_3, 2), 0)) = _4.fld4.0 as i16;
SetDiscriminant(_3, 2);
place!(Field::<u16>(Variant(_3, 2), 2)) = 45430_u16;
_8 = &_15;
_4.fld3.0 = '\u{63fd4}';
_6 = !(*_8);
_4.fld4 = (2790571577_u32,);
_4.fld5 = core::ptr::addr_of_mut!(_5.0.0);
place!(Field::<i64>(Variant(_3, 2), 1)) = -(-4173883888100321968_i64);
_4.fld4 = (3458951393_u32,);
_5.0.0 = _4.fld7 as i16;
_11 = !_4.fld6;
place!(Field::<u16>(Variant(_3, 2), 2)) = !14839_u16;
_8 = &_6;
place!(Field::<i64>(Variant(_3, 2), 1)) = 163172469461992581_i64;
_10 = false;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(18_usize, 12_usize, Move(_12), 6_usize, Move(_6), 11_usize, Move(_11), 19_usize, _19), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static i8,mut _2: bool,mut _3: char) -> u128 {
mir! {
type RET = u128;
let _4: *mut &'static *mut i16;
let _5: *const i8;
let _6: char;
let _7: char;
let _8: [u8; 2];
let _9: f32;
let _10: [usize; 8];
let _11: (char, u128);
let _12: u16;
let _13: u32;
let _14: (Adt42, i16, [char; 4]);
let _15: *const [i128; 4];
let _16: [u8; 2];
let _17: ((u32,), f32, u32);
let _18: u8;
let _19: f64;
let _20: [i32; 5];
let _21: bool;
let _22: u64;
let _23: u8;
let _24: usize;
let _25: bool;
let _26: i64;
let _27: *mut &'static *mut i16;
let _28: (*const *const i64,);
let _29: ();
let _30: ();
{
RET = 95040603875679203760676579906508140408_u128 << 7913257288299717470_i64;
_2 = false;
_2 = !true;
_2 = !false;
_2 = !false;
RET = 1275183485_i32 as u128;
_3 = '\u{3b222}';
_2 = true;
RET = 239099300939021345850844721643696323280_u128 >> 6228174413751817897_u64;
RET = 116815510513894942768587046938463433526_u128;
_2 = _3 >= _3;
RET = !26736733778042695673398266019827607495_u128;
Goto(bb1)
}
bb1 = {
_2 = !false;
_2 = true;
_2 = false;
_3 = '\u{f037b}';
RET = 217791256766496053960730818094397776801_u128 ^ 164612605410208350767666005717642013374_u128;
_3 = '\u{8e048}';
_3 = '\u{68123}';
_3 = '\u{1f58d}';
RET = 9223372036854775807_isize as u128;
RET = (-4652608052709190473_i64) as u128;
RET = !259821511977365816134309666762623554547_u128;
RET = 105402947758249956702766451111298030969_i128 as u128;
_3 = '\u{57345}';
_2 = false;
_2 = !false;
RET = 291882366888540341408956860527163618403_u128;
_3 = '\u{43fe2}';
match RET {
0 => bb2,
291882366888540341408956860527163618403 => bb4,
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
RET = !123894837609079274738419474789130383309_u128;
Goto(bb5)
}
bb5 = {
_2 = false & true;
_2 = RET == RET;
_6 = _3;
_3 = _6;
_6 = _3;
_3 = _6;
_6 = _3;
_6 = _3;
_2 = false;
Goto(bb6)
}
bb6 = {
_2 = !false;
_7 = _3;
_8 = [36_u8,115_u8];
RET = 303589600114233975513182245363007980412_u128 ^ 281205708698828387470041865585440521571_u128;
_3 = _6;
_10 = [17747373502730935661_usize,2576149187029354493_usize,18420432733425489438_usize,3371055286338025492_usize,5341873459540559184_usize,1657690821540066676_usize,3_usize,17463546252114451960_usize];
_2 = true & true;
RET = 7698_u16 as u128;
_6 = _3;
_3 = _6;
_12 = !51053_u16;
_9 = 4069133704939700676_usize as f32;
RET = !16847740491160507265650215164195411392_u128;
RET = 227588597462384289070144270338643837622_u128 - 134363342443792028484483527274501394361_u128;
_11.0 = _3;
_7 = _11.0;
_3 = _6;
Goto(bb7)
}
bb7 = {
_11.1 = !RET;
_11.1 = RET;
_11.0 = _7;
_7 = _11.0;
_8 = [49_u8,255_u8];
_11.1 = RET;
_13 = 3953327397_u32 * 3229700113_u32;
_11.1 = !RET;
_6 = _11.0;
_13 = (-19024_i16) as u32;
Goto(bb8)
}
bb8 = {
_11.1 = (-18_isize) as u128;
_11.1 = _12 as u128;
RET = !_11.1;
Goto(bb9)
}
bb9 = {
RET = _11.1;
_11.0 = _6;
_6 = _3;
_14.2 = [_3,_11.0,_11.0,_7];
Goto(bb10)
}
bb10 = {
_14.2 = [_11.0,_7,_11.0,_11.0];
_14.2 = [_7,_11.0,_11.0,_7];
_14.2 = [_6,_6,_3,_11.0];
_11 = (_7, RET);
_11 = (_7, RET);
_14.1 = 19750_i16;
_2 = _6 <= _7;
_11.0 = _3;
_11.0 = _3;
_14.2 = [_6,_7,_7,_6];
_9 = _13 as f32;
_10 = [6_usize,7_usize,2_usize,7_usize,14542719454286735312_usize,5_usize,2_usize,1747407602831419541_usize];
_11.0 = _3;
_8 = [76_u8,121_u8];
_8 = [12_u8,5_u8];
_17.1 = _9 + _9;
_7 = _11.0;
_17.0.0 = _2 as u32;
_17.2 = _13 >> _12;
_8 = [42_u8,19_u8];
RET = !_11.1;
_17.2 = !_17.0.0;
_2 = true;
_14.2 = [_6,_6,_7,_11.0];
_17.0.0 = _17.2 * _17.2;
Goto(bb11)
}
bb11 = {
_11.1 = !RET;
_16 = [194_u8,178_u8];
_19 = _14.1 as f64;
_19 = 119_u8 as f64;
_14.1 = (-18629_i16);
_3 = _11.0;
_13 = _17.0.0;
_22 = 3614780365914586917_u64 | 14382139473736578014_u64;
_13 = 83_u8 as u32;
match _14.1 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb12,
340282366920938463463374607431768192827 => bb14,
_ => bb13
}
}
bb12 = {
_2 = !false;
_7 = _3;
_8 = [36_u8,115_u8];
RET = 303589600114233975513182245363007980412_u128 ^ 281205708698828387470041865585440521571_u128;
_3 = _6;
_10 = [17747373502730935661_usize,2576149187029354493_usize,18420432733425489438_usize,3371055286338025492_usize,5341873459540559184_usize,1657690821540066676_usize,3_usize,17463546252114451960_usize];
_2 = true & true;
RET = 7698_u16 as u128;
_6 = _3;
_3 = _6;
_12 = !51053_u16;
_9 = 4069133704939700676_usize as f32;
RET = !16847740491160507265650215164195411392_u128;
RET = 227588597462384289070144270338643837622_u128 - 134363342443792028484483527274501394361_u128;
_11.0 = _3;
_7 = _11.0;
_3 = _6;
Goto(bb7)
}
bb13 = {
RET = !123894837609079274738419474789130383309_u128;
Goto(bb5)
}
bb14 = {
_18 = 4_u8;
_17.1 = _9;
_14.1 = 18315_i16 << _12;
_23 = _18;
_17.1 = _17.0.0 as f32;
_11 = (_7, RET);
RET = _11.1 << _17.2;
RET = (-400160001_i32) as u128;
_21 = !_2;
_22 = !70557038155801035_u64;
_8 = [_18,_23];
_11 = (_7, RET);
_6 = _3;
_11.1 = _21 as u128;
_24 = _19 as usize;
_21 = _2;
_16 = [_23,_18];
_19 = 773109417246617500_i64 as f64;
_11.1 = RET;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(19_usize, 11_usize, Move(_11), 3_usize, Move(_3), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(19_usize, 22_usize, Move(_22), 24_usize, Move(_24), 7_usize, Move(_7), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(828352768_u32), std::hint::black_box(35291_u16), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(28_i8), std::hint::black_box(30571_i16), std::hint::black_box((-836272157_i32)), std::hint::black_box(4389482019796578673_i64), std::hint::black_box((-155721926198519899014692635670767545804_i128)), std::hint::black_box(1_usize));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u64,
fld1: *const i64,
fld2: u128,
fld3: f32,
fld4: ([char; 8], char),
fld5: u8,

},
Variant1{
fld0: (u32, u32, i8, isize),
fld1: u32,
fld2: *mut u8,
fld3: i8,
fld4: ((u32,), f32, u32),
fld5: i32,
fld6: i64,

},
Variant2{
fld0: i16,
fld1: i64,
fld2: u16,

},
Variant3{
fld0: u8,
fld1: *const *const i64,
fld2: *const f64,
fld3: i8,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: ((u32,), f32, u32),

},
Variant1{
fld0: i8,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: (char, u128),
fld1: f32,
fld2: (u32, u32, i8, isize),
fld3: i8,
fld4: (i16, u128),
fld5: *mut *const *const i64,
fld6: (u32,),
fld7: ([i64; 8], [i128; 7], (u32,)),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: usize,
fld1: *const i8,
fld2: f32,
fld3: u32,
fld4: [i128; 4],
fld5: *const i64,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: Adt53,
fld1: (char, u128),
fld2: u16,
fld3: f32,

},
Variant1{
fld0: *const [i128; 4],
fld1: u16,
fld2: ([char; 8], char),

},
Variant2{
fld0: u64,
fld1: *mut i16,
fld2: isize,
fld3: u8,
fld4: [char; 8],
fld5: *mut *const i64,
fld6: u32,
fld7: [u8; 7],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: (i16, u128),
fld1: *const [i128; 4],
fld2: *const (u32,),
fld3: (char, u128),
fld4: (u32,),
fld5: *mut i16,
fld6: usize,
fld7: i128,
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: *mut *const [i128; 4],

},
Variant1{
fld0: (i64, *const i64, *const i64, i16),
fld1: char,
fld2: *const i64,
fld3: u64,
fld4: Adt58,
fld5: *mut *const *const i64,
fld6: u16,

},
Variant2{
fld0: *mut *mut *const *const i64,

},
Variant3{
fld0: u64,
fld1: f64,
fld2: Adt59,
fld3: Adt46,

}}
impl PrintFDebug for Adt88{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt88{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt88 {
fld0: *mut i16,
fld1: *const i8,
}

