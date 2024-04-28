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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: isize,mut _4: i8,mut _5: i64) -> i16 {
mir! {
type RET = i16;
let _6: [bool; 5];
let _7: Adt53;
let _8: &'static f64;
let _9: [u128; 6];
let _10: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _11: Adt43;
let _12: (char, u128);
let _13: (u8, f64);
let _14: [bool; 5];
let _15: (isize,);
let _16: f32;
let _17: ();
let _18: ();
{
_4 = (-94_i8) << 186213244416342663114195007432756064350_u128;
_5 = 65034267823939950986072263633949909880_u128 as i64;
_5 = 1734457945955912462_i64 & 4350289123145076376_i64;
RET = 3674903828_u32 as i16;
_4 = !110_i8;
_4 = -(-55_i8);
Call(_2 = fn1(_5, _5, _5, _4, _5, RET, _4, _4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -(-5330_i16);
_4 = -(-68_i8);
_4 = -(-40_i8);
_6 = [true,false,true,true,false];
RET = 298838788719619924884179960693348615304_u128 as i16;
_5 = 4759204708330752340_i64;
_6 = [true,true,true,true,false];
_1 = true;
_4 = 90_i8 ^ (-45_i8);
_1 = true;
Goto(bb2)
}
bb2 = {
RET = _1 as i16;
_5 = (-8180306888089907284_i64) + 8383412635656636310_i64;
_3 = (-9223372036854775808_isize) >> _4;
RET = _5 as i16;
_4 = (-970070918_i32) as i8;
_1 = false;
_2 = 43843_u16 >> _5;
_5 = -3439898582211925083_i64;
_3 = -9223372036854775807_isize;
_6 = [_1,_1,_1,_1,_1];
_1 = true;
RET = _3 as i16;
RET = 6715_i16;
_1 = _3 != _3;
_3 = !(-82_isize);
_6 = [_1,_1,_1,_1,_1];
_6 = [_1,_1,_1,_1,_1];
_3 = RET as isize;
_5 = RET as i64;
_5 = 171_u8 as i64;
RET = 12287_i16 ^ (-9554_i16);
_10.5 = ('\u{77f1e}', 183395721729731413358487676857597199131_u128);
_6 = [_1,_1,_1,_1,_1];
_10.0 = [71434012483773626_u64,784531975360245102_u64,10441616836850750594_u64,806572805728290859_u64,5051928156132446243_u64,12436858068770311514_u64,2113890696809008355_u64];
_5 = 2279710617_u32 as i64;
match _10.5.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
183395721729731413358487676857597199131 => bb11,
_ => bb10
}
}
bb3 = {
RET = -(-5330_i16);
_4 = -(-68_i8);
_4 = -(-40_i8);
_6 = [true,false,true,true,false];
RET = 298838788719619924884179960693348615304_u128 as i16;
_5 = 4759204708330752340_i64;
_6 = [true,true,true,true,false];
_1 = true;
_4 = 90_i8 ^ (-45_i8);
_1 = true;
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
_8 = &_11.fld5.0;
_11.fld3 = (_10.5,);
_10.0 = [15101446333375523180_u64,15617604451957797221_u64,7848914170753588288_u64,9726907393102148993_u64,8186304543426627555_u64,3710226262071592778_u64,2207606166261970367_u64];
_10.5.0 = _11.fld3.0.0;
_12.0 = _10.5.0;
_11.fld6 = _5;
_10.4 = _3;
_10.1 = (3604825201_u32, _4);
_11.fld3 = (_10.5,);
match _10.5.1 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb12,
5 => bb13,
183395721729731413358487676857597199131 => bb15,
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
RET = -(-5330_i16);
_4 = -(-68_i8);
_4 = -(-40_i8);
_6 = [true,false,true,true,false];
RET = 298838788719619924884179960693348615304_u128 as i16;
_5 = 4759204708330752340_i64;
_6 = [true,true,true,true,false];
_1 = true;
_4 = 90_i8 ^ (-45_i8);
_1 = true;
Goto(bb2)
}
bb15 = {
_11.fld4 = 4848740401285770389_u64 | 15970354738552353123_u64;
_10.1.1 = _4 * _4;
_11.fld4 = 7830607198661096623_u64;
_11.fld3.0.1 = _10.5.1;
_10.1 = (3240894798_u32, _4);
_13.1 = _11.fld6 as f64;
_6 = [_1,_1,_1,_1,_1];
_11.fld0 = _1 & _1;
_11.fld4 = !1732628960232374842_u64;
_12.1 = !_10.5.1;
_11.fld7 = !108751643586737752879938047341543717511_i128;
_13.0 = 41_u8 & 239_u8;
_9 = [_10.5.1,_10.5.1,_11.fld3.0.1,_12.1,_11.fld3.0.1,_11.fld3.0.1];
_11.fld6 = _1 as i64;
_13.1 = _13.0 as f64;
_11.fld3.0 = _10.5;
_13.0 = 47_u8 + 87_u8;
RET = 123708507_i32 as i16;
_14 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_3 = _10.4 | _10.4;
Goto(bb16)
}
bb16 = {
Call(_17 = dump_var(0_usize, 5_usize, Move(_5), 2_usize, Move(_2), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i8,mut _5: i64,mut _6: i16,mut _7: i8,mut _8: i8,mut _9: i64) -> u16 {
mir! {
type RET = u16;
let _10: isize;
let _11: char;
let _12: Adt44;
let _13: f64;
let _14: ((&'static f64, bool, bool, u64, usize),);
let _15: bool;
let _16: &'static f64;
let _17: u64;
let _18: isize;
let _19: (f64,);
let _20: bool;
let _21: [u128; 6];
let _22: isize;
let _23: [u128; 6];
let _24: [bool; 5];
let _25: f64;
let _26: [i64; 7];
let _27: [u128; 6];
let _28: f64;
let _29: *const &'static f64;
let _30: i32;
let _31: char;
let _32: ();
let _33: ();
{
_5 = _2 + _9;
_6 = 22166_i16;
_4 = _8;
_2 = -_5;
_5 = _2;
_6 = 21773_i16 >> _2;
_3 = _9 * _2;
_8 = -_4;
_5 = _8 as i64;
_8 = !_4;
RET = 9556_u16 ^ 40721_u16;
RET = _3 as u16;
_5 = _3;
_9 = _1 | _1;
RET = 12351679083609378054_u64 as u16;
_10 = 1926172910_i32 as isize;
Call(_5 = fn2(_8, _7, _6, _1, RET, _1, _2, _10, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 64650_u16 | 61425_u16;
_3 = '\u{591a1}' as i64;
_2 = !_5;
_1 = _2;
_2 = _6 as i64;
RET = (-127251390450985972537437012654090346410_i128) as u16;
Call(_2 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = '\u{103482}';
_8 = _7;
_7 = _6 as i8;
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = (-68074363158105431559123365750047146066_i128) as i64;
_8 = !_7;
_3 = _10 as i64;
_11 = '\u{25e01}';
_7 = _6 as i8;
_13 = 285695095013100145620858836919093669807_u128 as f64;
_9 = _2;
_7 = RET as i8;
_1 = -_5;
RET = 641540504_i32 as u16;
_9 = _2;
_9 = 5_usize as i64;
_3 = _2;
_14.0.4 = !6552446244905727546_usize;
_3 = _2;
_9 = _10 as i64;
_6 = (-26406_i16) * (-31095_i16);
_14.0.2 = !true;
_10 = -(-9223372036854775808_isize);
_14.0.0 = &_13;
_4 = !_8;
_14.0.1 = _14.0.2 | _14.0.2;
Goto(bb3)
}
bb3 = {
_7 = _8 & _8;
_14.0.2 = !_14.0.1;
_15 = !_14.0.1;
_8 = -_7;
_9 = !_3;
_13 = _8 as f64;
_1 = -_2;
_14.0.0 = &_13;
_1 = _4 as i64;
RET = 36833_u16;
_13 = 3959031831344640606_u64 as f64;
Call(_6 = fn8(_9, _3, _9, _3, _9, _2, _3, _2, _3, _9, _9, _9, _3, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = _13 as i8;
Call(_13 = fn14(_9, _2, _2, _3, _9, _9, _9, _3, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = &_13;
_8 = _4 & _7;
_14.0.0 = &_13;
_11 = '\u{3f3fc}';
_4 = _9 as i8;
_14.0 = (Move(_16), _15, _15, 8646477508729744330_u64, 7_usize);
_16 = &_13;
_18 = -_10;
RET = !5514_u16;
_9 = !_3;
_5 = -_9;
_14.0 = (Move(_16), _15, _15, 12127655279658256810_u64, 2_usize);
_6 = (-27951_i16) * 11441_i16;
_18 = _10;
match _14.0.4 {
0 => bb4,
1 => bb2,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
2 => bb11,
_ => bb10
}
}
bb6 = {
_7 = _13 as i8;
Call(_13 = fn14(_9, _2, _2, _3, _9, _9, _9, _3, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_7 = _8 & _8;
_14.0.2 = !_14.0.1;
_15 = !_14.0.1;
_8 = -_7;
_9 = !_3;
_13 = _8 as f64;
_1 = -_2;
_14.0.0 = &_13;
_1 = _4 as i64;
RET = 36833_u16;
_13 = 3959031831344640606_u64 as f64;
Call(_6 = fn8(_9, _3, _9, _3, _9, _2, _3, _2, _3, _9, _9, _9, _3, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_11 = '\u{103482}';
_8 = _7;
_7 = _6 as i8;
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = (-68074363158105431559123365750047146066_i128) as i64;
_8 = !_7;
_3 = _10 as i64;
_11 = '\u{25e01}';
_7 = _6 as i8;
_13 = 285695095013100145620858836919093669807_u128 as f64;
_9 = _2;
_7 = RET as i8;
_1 = -_5;
RET = 641540504_i32 as u16;
_9 = _2;
_9 = 5_usize as i64;
_3 = _2;
_14.0.4 = !6552446244905727546_usize;
_3 = _2;
_9 = _10 as i64;
_6 = (-26406_i16) * (-31095_i16);
_14.0.2 = !true;
_10 = -(-9223372036854775808_isize);
_14.0.0 = &_13;
_4 = !_8;
_14.0.1 = _14.0.2 | _14.0.2;
Goto(bb3)
}
bb9 = {
RET = 64650_u16 | 61425_u16;
_3 = '\u{591a1}' as i64;
_2 = !_5;
_1 = _2;
_2 = _6 as i64;
RET = (-127251390450985972537437012654090346410_i128) as u16;
Call(_2 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_6 = (-15467_i16) >> _2;
_13 = 100_u8 as f64;
_19.0 = _13 + _13;
_1 = _2 & _5;
_21 = [266613317307713985258032354841923987412_u128,318061817564308685352103170140677391605_u128,132639296810747678865449014796299398379_u128,108384798875271243377570493276301582479_u128,165990711432658624968731056303204236394_u128,54262110636599018138529387382225483027_u128];
_19 = (_13,);
_4 = _13 as i8;
Goto(bb12)
}
bb12 = {
RET = 21016_u16;
Call(_3 = core::intrinsics::bswap(_9), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10 = 61053805665274869546267784019041242981_u128 as isize;
_8 = _11 as i8;
_13 = _19.0;
_11 = '\u{5f76a}';
_10 = _18;
_4 = _8;
_4 = !_8;
_19 = (_13,);
_19.0 = -_13;
_14.0.1 = _15;
_13 = -_19.0;
_16 = &_13;
_15 = !_14.0.2;
_1 = -_9;
_9 = !_2;
_19.0 = _13 * _13;
_14.0.4 = !4_usize;
_19 = (_13,);
_17 = _14.0.4 as u64;
_23 = [315859719467016788072101058147379488841_u128,329825302686795768829968715290721196010_u128,126158307352410375024227285079613738184_u128,323817969868719278324284585698540082692_u128,160946055764375775974769187467869591318_u128,268258146762481400726792065388723025361_u128];
_14.0 = (Move(_16), _15, _15, _17, 2_usize);
match _14.0.4 {
0 => bb4,
2 => bb15,
_ => bb14
}
}
bb14 = {
_11 = '\u{103482}';
_8 = _7;
_7 = _6 as i8;
_10 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = (-68074363158105431559123365750047146066_i128) as i64;
_8 = !_7;
_3 = _10 as i64;
_11 = '\u{25e01}';
_7 = _6 as i8;
_13 = 285695095013100145620858836919093669807_u128 as f64;
_9 = _2;
_7 = RET as i8;
_1 = -_5;
RET = 641540504_i32 as u16;
_9 = _2;
_9 = 5_usize as i64;
_3 = _2;
_14.0.4 = !6552446244905727546_usize;
_3 = _2;
_9 = _10 as i64;
_6 = (-26406_i16) * (-31095_i16);
_14.0.2 = !true;
_10 = -(-9223372036854775808_isize);
_14.0.0 = &_13;
_4 = !_8;
_14.0.1 = _14.0.2 | _14.0.2;
Goto(bb3)
}
bb15 = {
_17 = _14.0.3 >> _5;
_22 = _10 >> _1;
_10 = _22 * _18;
_15 = !_14.0.1;
_2 = _1;
_1 = _17 as i64;
_3 = !_5;
_14.0.3 = _17;
_25 = _19.0;
_19 = (_25,);
_20 = _14.0.2 | _14.0.1;
_3 = _5 - _1;
_14.0.3 = _17;
_25 = _19.0;
_29 = core::ptr::addr_of!(_16);
_15 = _20;
_17 = _14.0.3;
_26 = [_1,_1,_9,_3,_9,_2,_1];
(*_29) = &_19.0;
_7 = _4;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(1_usize, 17_usize, Move(_17), 2_usize, Move(_2), 18_usize, Move(_18), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(1_usize, 11_usize, Move(_11), 4_usize, Move(_4), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(1_usize, 3_usize, Move(_3), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i8,mut _2: i8,mut _3: i16,mut _4: i64,mut _5: u16,mut _6: i64,mut _7: i64,mut _8: isize,mut _9: i64) -> i64 {
mir! {
type RET = i64;
let _10: i16;
let _11: [bool; 5];
let _12: u32;
let _13: *const f64;
let _14: u8;
let _15: (char, u128);
let _16: [i64; 7];
let _17: char;
let _18: [i64; 7];
let _19: (u8, f64);
let _20: i8;
let _21: [u128; 6];
let _22: f64;
let _23: isize;
let _24: char;
let _25: i128;
let _26: char;
let _27: isize;
let _28: ();
let _29: ();
{
RET = _7 - _6;
RET = _9;
_10 = _3;
RET = _7 ^ _7;
RET = _4 & _9;
_10 = -_3;
RET = _8 as i64;
RET = -_7;
_8 = 9223372036854775807_isize << _9;
RET = 176_u8 as i64;
_8 = (-9223372036854775808_isize) - 23_isize;
Call(_10 = fn3(_9, _3, _9, _8, _9, _6, _9, _4, _1, _7, _7, _3, RET, _8, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = !_7;
RET = _9 & _6;
_1 = _2;
_6 = -_4;
_1 = (-157118384627521812804732303074443491071_i128) as i8;
Goto(bb2)
}
bb2 = {
_15 = ('\u{c2551}', 267108316459484418411279285371718286825_u128);
_15.1 = true as u128;
RET = _6;
RET = 1212487131_u32 as i64;
_6 = _9 >> _4;
_15.0 = '\u{e471b}';
RET = -_6;
_5 = !33827_u16;
_2 = _1 + _1;
_15.0 = '\u{d8238}';
RET = 10753554157164158708_usize as i64;
_16 = [_6,_6,_4,_7,_9,_4,_4];
_2 = -_1;
_16 = [_9,_6,_9,_6,_7,_6,_6];
_7 = _6;
_17 = _15.0;
_15 = (_17, 246184709504414572631104515946659310482_u128);
_2 = false as i8;
_2 = (-1771572104_i32) as i8;
_11 = [false,false,true,false,false];
_3 = _10;
_19.0 = 62_u8 - 243_u8;
_12 = 3540780022_u32;
match _15.1 {
246184709504414572631104515946659310482 => bb3,
_ => bb1
}
}
bb3 = {
_15.0 = _17;
_15.1 = 154788892902514357823189730535487936194_u128;
RET = !_6;
_4 = _6 - _6;
_15.1 = 101649777390825725812128366677029733867_u128 - 335223827894442328437730163160403340243_u128;
_15.0 = _17;
_19.0 = 27_u8;
_18 = _16;
_19.0 = !152_u8;
Goto(bb4)
}
bb4 = {
_13 = core::ptr::addr_of!(_19.1);
_15.1 = (-1382458500_i32) as u128;
_20 = _1 >> _6;
_18 = [RET,_4,_4,_9,_4,_6,RET];
_7 = !RET;
_11 = [true,true,false,false,false];
_11 = [false,false,false,false,false];
(*_13) = 142968073_i32 as f64;
_23 = _8 & _8;
_6 = _4 << _20;
_5 = !48766_u16;
_5 = !64627_u16;
_14 = true as u8;
_21 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_26 = _15.0;
_26 = _17;
_15.1 = 323730181507615248897183057319847561671_u128;
_3 = _19.0 as i16;
_13 = core::ptr::addr_of!(_22);
_15.0 = _17;
RET = _12 as i64;
_19.1 = _15.1 as f64;
Call(_17 = fn6(_6, _20, _14, _16, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = !_6;
_8 = _6 as isize;
RET = _6 | _9;
_16 = _18;
_17 = _26;
_27 = _8 & _8;
(*_13) = -_19.1;
_1 = 11236864765020546810_usize as i8;
_25 = -125053124260826969177447317610993314802_i128;
Goto(bb6)
}
bb6 = {
Call(_28 = dump_var(2_usize, 2_usize, Move(_2), 7_usize, Move(_7), 17_usize, Move(_17), 23_usize, Move(_23)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_28 = dump_var(2_usize, 14_usize, Move(_14), 16_usize, Move(_16), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_28 = dump_var(2_usize, 21_usize, Move(_21), 9_usize, Move(_9), 5_usize, Move(_5), 29_usize, _29), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i64,mut _2: i16,mut _3: i64,mut _4: isize,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i8,mut _10: i64,mut _11: i64,mut _12: i16,mut _13: i64,mut _14: isize,mut _15: u16) -> i16 {
mir! {
type RET = i16;
let _16: (u8, f64);
let _17: char;
let _18: u32;
let _19: [i64; 7];
let _20: [i64; 7];
let _21: isize;
let _22: f32;
let _23: char;
let _24: i32;
let _25: Adt54;
let _26: isize;
let _27: bool;
let _28: Adt49;
let _29: *const &'static f64;
let _30: i64;
let _31: [i64; 7];
let _32: Adt52;
let _33: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _34: f32;
let _35: f64;
let _36: u8;
let _37: [i64; 7];
let _38: [u128; 6];
let _39: (isize,);
let _40: f32;
let _41: isize;
let _42: u16;
let _43: f32;
let _44: ();
let _45: ();
{
RET = _2 | _12;
_13 = _9 as i64;
_5 = -_10;
RET = -_12;
_13 = !_1;
_18 = 2220786025_u32 << _2;
RET = _12 - _12;
_15 = !23962_u16;
_13 = -_3;
_19 = [_3,_8,_10,_1,_3,_7,_5];
_17 = '\u{c4783}';
_1 = _13;
_18 = 389109263_u32 * 3971578024_u32;
_19 = [_7,_13,_13,_7,_7,_11,_11];
_20 = [_5,_10,_10,_11,_7,_7,_13];
_22 = 196926804033592521948811189362802857975_u128 as f32;
_15 = _9 as u16;
_16.1 = _9 as f64;
_12 = 2206396601164284916_u64 as i16;
_23 = _17;
_20 = _19;
_2 = !RET;
_7 = _5 ^ _11;
Goto(bb1)
}
bb1 = {
RET = _15 as i16;
_13 = _5 & _5;
_20 = _19;
RET = _12;
_24 = 259724523_i32;
_4 = !_14;
_2 = RET;
_19 = _20;
_16.0 = 47_u8;
_24 = -1068363124_i32;
_11 = _3;
_22 = _18 as f32;
_12 = !RET;
_10 = _11;
_19 = [_5,_6,_1,_11,_10,_7,_1];
_26 = _4;
_11 = -_10;
_11 = _22 as i64;
_7 = _5;
_5 = _4 as i64;
_16.1 = _24 as f64;
_8 = _13;
_8 = 4_usize as i64;
_13 = !_6;
_26 = _14;
_8 = !_3;
_21 = _26 | _26;
_24 = _23 as i32;
match _16.0 {
0 => bb2,
1 => bb3,
2 => bb4,
47 => bb6,
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
_27 = !true;
RET = _2 & _2;
_28.fld4.1 = _22 as f64;
_23 = _17;
_11 = _10;
Call(RET = core::intrinsics::transmute(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_28.fld4.0 = _16.0;
_28.fld0 = _7;
_24 = (-1517880427_i32);
_11 = _8 - _28.fld0;
_16.0 = _18 as u8;
_35 = _16.1;
_28.fld0 = !_7;
_13 = _7 & _1;
_33.5 = (_17, 16048625791121177466122548939005485573_u128);
_33.1.1 = 5692385992590148199_usize as i8;
_9 = -_33.1.1;
_26 = _15 as isize;
Call(_33.5.1 = fn4(_7, _11, _1, _7, _16.1, _13, _21, _11, _10, _10, _11, _1, RET, _6, _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_28.fld4.1 = _33.5.1 as f64;
_19 = _20;
Goto(bb9)
}
bb9 = {
_6 = _8 & _11;
_33.5.1 = 2973988748849281151_u64 as u128;
_3 = 5_usize as i64;
_11 = _7;
_34 = _22;
_34 = _22;
_35 = _28.fld4.1 + _16.1;
_28.fld4.1 = _9 as f64;
_1 = _13 >> _7;
match _24 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb7,
4 => bb5,
5 => bb10,
6 => bb11,
340282366920938463463374607430250331029 => bb13,
_ => bb12
}
}
bb10 = {
_28.fld4.1 = _33.5.1 as f64;
_19 = _20;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_33.4 = _21 >> _8;
_2 = RET;
_28.fld4.1 = -_16.1;
_28.fld2 = core::ptr::addr_of!(_35);
_14 = -_4;
_33.0 = [15288617180532425375_u64,2165957469252393819_u64,3648166599164496589_u64,8993380668550502166_u64,16026994934165981011_u64,2434799614591835305_u64,2371902541767171624_u64];
_30 = _8;
_36 = !_16.0;
_40 = -_22;
_39.0 = -_33.4;
_33.5.0 = _17;
_33.1.0 = _18 | _18;
_16.1 = _18 as f64;
_27 = !true;
_31 = _19;
_33.3 = !_33.1.1;
_30 = _7;
_28.fld0 = _7 - _5;
_18 = _33.1.0 + _33.1.0;
_35 = _28.fld4.1 - _16.1;
match _28.fld4.0 {
47 => bb14,
_ => bb10
}
}
bb14 = {
_14 = 2590168557342543480_u64 as isize;
_11 = -_1;
_38 = [_33.5.1,_33.5.1,_33.5.1,_33.5.1,_33.5.1,_33.5.1];
_28.fld4 = (_16.0, _35);
_32 = Adt52::Variant3 { fld0: _33.0 };
_18 = _33.3 as u32;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(3_usize, 2_usize, Move(_2), 9_usize, Move(_9), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(3_usize, 38_usize, Move(_38), 8_usize, Move(_8), 36_usize, Move(_36), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(3_usize, 24_usize, Move(_24), 11_usize, Move(_11), 19_usize, Move(_19), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(3_usize, 3_usize, Move(_3), 12_usize, Move(_12), 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: f64,mut _6: i64,mut _7: isize,mut _8: i64,mut _9: i64,mut _10: i64,mut _11: i64,mut _12: i64,mut _13: i16,mut _14: i64,mut _15: i64) -> u128 {

	mir! {
type RET = u128;
let _16: isize;
let _17: bool;
let _18: bool;
let _19: char;
let _20: Adt53;
let _21: Adt51;
let _22: (u32, i8);
let _23: char;
let _24: bool;
let _25: bool;
let _26: (i8, i16);
let _27: (u8, f64);
let _28: (isize,);
let _29: (u8, f64);
let _30: isize;
let _31: Adt52;
let _32: u128;
let _33: isize;
let _34: isize;
let _35: f32;
let _36: [i64; 7];
let _37: f32;
let _38: bool;
let _39: isize;
let _40: isize;
let _41: ();
let _42: ();
{

_10 = !_11;
_3 = 55_u8 as i64;
_5 = 13907830427119554310_u64 as f64;
_4 = -_2;
_6 = _11 & _11;
_8 = _15 << _6;
_7 = !(-9223372036854775808_isize);
_5 = 5_usize as f64;
_5 = 16692680020138226209_u64 as f64;
_3 = 207332630_u32 as i64;
_6 = 20495_u16 as i64;
RET = 393245234_i32 as u128;
_14 = _8;
_9 = (-30392990472635475392619489525493691062_i128) as i64;
_15 = _10;
_12 = _11 << _2;
_17 = _14 > _12;
_14 = _4 << _10;
_8 = 4191730443_u32 as i64;
_7 = 9223372036854775807_isize;
_16 = _7;
_14 = _12;
_10 = _6;
_2 = '\u{8b165}' as i64;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9223372036854775807 => bb8,
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
_4 = 1_usize as i64;
_11 = -_1;
_18 = !_17;
_19 = '\u{f805c}';
RET = 74301296716274321249262891161580542144_u128 & 270775014465479859762188032769227419614_u128;
_12 = _16 as i64;
_7 = _16 | _16;
_11 = _14 ^ _1;
_12 = !_11;
_7 = _16 << _4;
_26.1 = !_13;
_22.0 = 4008817421_u32 + 1527741867_u32;
match _16 {
0 => bb4,
1 => bb5,
2 => bb9,
9223372036854775807 => bb11,
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
_26.1 = _13;
_24 = _18;
_1 = _14 * _14;
_27 = (77_u8, _5);
_26.1 = _5 as i16;
_22.0 = 16479521781485507855_u64 as u32;
_29 = (_27.0, _5);
_4 = !_14;
_28 = (_16,);
Call(_24 = fn5(_4, _1, _7, _14, _18), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_26.0 = 4183362274305673086_usize as i8;
_19 = '\u{b1e5d}';
_27.0 = 4_usize as u8;
_19 = '\u{218}';
_26.0 = -(-103_i8);
_8 = _1;
Goto(bb13)
}
bb13 = {
_29.1 = -_27.1;
_22.1 = _26.0;
_30 = _7 * _16;
_28 = (_7,);
_26.0 = _13 as i8;
_28 = (_30,);
_18 = _24;
_32 = !RET;
_15 = _1;
_6 = !_15;
RET = _32 + _32;
RET = _32;
_25 = _24;
_22.0 = !4245940620_u32;
_30 = _7;
_28.0 = _18 as isize;
Goto(bb14)
}
bb14 = {
_22.0 = _26.1 as u32;
_15 = (-156355528862674960316788708196255924203_i128) as i64;
_32 = RET - RET;
_19 = '\u{63a70}';
_24 = !_17;
_29 = _27;
_22.1 = _26.0 * _26.0;
_27.1 = -_29.1;
_17 = _24;
_33 = _28.0 * _28.0;
_2 = _14;
_35 = _22.0 as f32;
_28 = (_33,);
_4 = _29.0 as i64;
_36 = [_1,_1,_1,_11,_6,_8,_11];
_11 = !_8;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(4_usize, 19_usize, Move(_19), 15_usize, Move(_15), 33_usize, Move(_33), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(4_usize, 7_usize, Move(_7), 28_usize, Move(_28), 30_usize, Move(_30), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(4_usize, 24_usize, Move(_24), 6_usize, Move(_6), 26_usize, Move(_26), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(4_usize, 9_usize, Move(_9), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i64,mut _2: i64,mut _3: isize,mut _4: i64,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: Adt50;
let _7: *mut *mut (&'static f64, bool, bool, u64, usize);
let _8: (u8, f64);
let _9: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _10: &'static f64;
let _11: ();
let _12: ();
{
_1 = _4 & _2;
_5 = !false;
_4 = _1;
RET = _5 | _5;
_8.1 = 8395_u16 as f64;
_9.1.1 = 81_i8 >> _2;
_9.4 = !_3;
_9.5 = ('\u{be1c5}', 36647839458338701246405998646430873189_u128);
_9.3 = _9.1.1 ^ _9.1.1;
_9.5 = ('\u{820fc}', 46300916698151800823176878049931200442_u128);
_9.5 = ('\u{7d625}', 181283367785034841392848553727914732951_u128);
RET = _5;
_1 = _4 & _4;
_9.3 = 14323_u16 as i8;
_3 = -_9.4;
_8.1 = (-28218791744966557436515660788509360060_i128) as f64;
_2 = _1 + _1;
RET = _4 == _2;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(5_usize, 2_usize, Move(_2), 5_usize, Move(_5), 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i64,mut _2: i8,mut _3: u8,mut _4: [i64; 7],mut _5: i64) -> char {
mir! {
type RET = char;
let _6: Adt47;
let _7: u64;
let _8: [u128; 6];
let _9: (u8, f64);
let _10: Adt48;
let _11: [u128; 6];
let _12: [u64; 7];
let _13: ([u64; 7], (&'static f64, bool, bool, u64, usize));
let _14: Adt42;
let _15: *mut usize;
let _16: char;
let _17: (char, u128);
let _18: f64;
let _19: usize;
let _20: isize;
let _21: f32;
let _22: i16;
let _23: (f64,);
let _24: i32;
let _25: u16;
let _26: isize;
let _27: i128;
let _28: isize;
let _29: bool;
let _30: char;
let _31: isize;
let _32: isize;
let _33: *mut *mut (&'static f64, bool, bool, u64, usize);
let _34: (f64,);
let _35: ();
let _36: ();
{
_6.fld4.0 = core::ptr::addr_of_mut!(_6.fld3);
RET = '\u{1c8c}';
_6.fld1 = -(-12140126163496877133454091628565459309_i128);
_6.fld6 = (97_isize,);
RET = '\u{4c606}';
_6.fld2 = 12964231184080779_u64;
_3 = _2 as u8;
_6.fld6 = ((-9223372036854775808_isize),);
_6.fld4.0 = core::ptr::addr_of_mut!(_2);
_3 = !58_u8;
_6.fld0.0.0 = RET;
_6.fld0.0 = (RET, 222365440411991799033654650968237136273_u128);
_4 = [_5,_1,_5,_1,_1,_1,_1];
_2 = 33_i8;
_1 = _5;
_4 = [_5,_1,_1,_1,_1,_1,_5];
_6.fld6.0 = 21992_i16 as isize;
RET = _6.fld0.0.0;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
33 => bb6,
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
_6.fld1 = !75046127612200587544945044456486452335_i128;
_2 = -87_i8;
_6.fld2 = !3750648877750924741_u64;
_6.fld1 = (-154612759806837003613655401325400876514_i128) >> _5;
_5 = _1;
RET = _6.fld0.0.0;
_6.fld3 = -_2;
_5 = 58462_u16 as i64;
_8 = [_6.fld0.0.1,_6.fld0.0.1,_6.fld0.0.1,_6.fld0.0.1,_6.fld0.0.1,_6.fld0.0.1];
Call(_9.0 = core::intrinsics::bswap(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9.0 = _3 - _3;
_6.fld0.0.1 = _1 as u128;
_9.1 = 1959877532_i32 as f64;
_2 = _6.fld0.0.0 as i8;
_6.fld0.0.1 = 197082842505760048219963732226171252154_u128;
_7 = !_6.fld2;
_6.fld2 = !_7;
_6.fld3 = _9.1 as i8;
RET = _6.fld0.0.0;
RET = _6.fld0.0.0;
_6.fld0.0.1 = 91599498122421497056351489596311678296_u128;
_10.fld2.fld6 = _6.fld6;
_10.fld1 = (_9.1,);
_6.fld0.0 = (RET, 156049336345861198900096861257007621917_u128);
_10.fld2.fld2 = _6.fld2 + _7;
_9.1 = _6.fld1 as f64;
_10.fld2.fld2 = _7 | _7;
_10.fld2.fld4 = (_6.fld4.0,);
_6.fld0.0.0 = RET;
_6.fld6 = (_10.fld2.fld6.0,);
_10.fld2.fld0.0 = _6.fld0.0;
_10.fld1 = (_9.1,);
_10.fld2.fld4.0 = _6.fld4.0;
_10.fld2.fld0.0 = (_6.fld0.0.0, _6.fld0.0.1);
_10.fld2.fld1 = -_6.fld1;
_6.fld4.0 = _10.fld2.fld4.0;
_10.fld2.fld3 = _2;
_10.fld2.fld0.0 = (_6.fld0.0.0, _6.fld0.0.1);
Goto(bb8)
}
bb8 = {
_10.fld2.fld0 = (_6.fld0.0,);
_9 = (_3, _10.fld1.0);
_3 = _9.1 as u8;
_6.fld2 = _7 + _10.fld2.fld2;
_4 = [_1,_1,_1,_1,_1,_1,_1];
_10.fld2.fld1 = _6.fld1 * _6.fld1;
_10.fld1 = (_9.1,);
_6.fld3 = _10.fld2.fld3 >> _10.fld2.fld1;
_10.fld0 = _6.fld0.0.1;
_10.fld1.0 = _9.1;
_6.fld2 = _10.fld2.fld2 | _7;
_6.fld6.0 = _10.fld2.fld0.0.0 as isize;
_9.0 = _3 * _3;
_3 = _6.fld6.0 as u8;
_13.1.1 = !false;
_11 = _8;
_10.fld2.fld3 = -_6.fld3;
_7 = !_10.fld2.fld2;
Call(_9.1 = core::intrinsics::fmaf64(_10.fld1.0, _10.fld1.0, _10.fld1.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6.fld4 = _10.fld2.fld4;
_12 = [_7,_7,_10.fld2.fld2,_7,_10.fld2.fld2,_10.fld2.fld2,_10.fld2.fld2];
_6.fld1 = _6.fld2 as i128;
_6.fld6 = (_10.fld2.fld6.0,);
_9.0 = 1859_u16 as u8;
_10.fld2.fld2 = 297749011_i32 as u64;
_15 = core::ptr::addr_of_mut!(_13.1.4);
RET = _10.fld2.fld0.0.0;
_10.fld2.fld6.0 = _13.1.1 as isize;
_10.fld2.fld0 = (_6.fld0.0,);
RET = _6.fld0.0.0;
_13.1.1 = !true;
_17.0 = RET;
_15 = core::ptr::addr_of_mut!((*_15));
_10.fld0 = _10.fld2.fld0.0.1;
_16 = _10.fld2.fld0.0.0;
_9 = (_3, _10.fld1.0);
_18 = _10.fld0 as f64;
_10.fld2.fld0.0.1 = _10.fld0 / _10.fld0;
_11 = _8;
_10.fld2.fld2 = _6.fld2 * _7;
_15 = core::ptr::addr_of_mut!(_19);
Goto(bb10)
}
bb10 = {
_10.fld2.fld6.0 = _1 as isize;
_13.1.0 = &_10.fld1.0;
_9.0 = _6.fld3 as u8;
_6.fld1 = _10.fld2.fld1 | _10.fld2.fld1;
_9.0 = _3 << _10.fld2.fld2;
_17.0 = _16;
_6.fld2 = _10.fld2.fld2 >> _6.fld1;
_20 = -_10.fld2.fld6.0;
RET = _16;
_10.fld2.fld6.0 = !_20;
_13.1.2 = _13.1.1;
_8 = _11;
_6.fld0.0.1 = _10.fld2.fld0.0.1;
_9.1 = _1 as f64;
_6.fld0.0 = _10.fld2.fld0.0;
_13.1.0 = &_9.1;
_13.1.3 = _6.fld2 - _6.fld2;
_21 = 32163_u16 as f32;
_6.fld4.0 = core::ptr::addr_of_mut!(_6.fld3);
_13.1.0 = &_10.fld1.0;
_13.1.3 = (-1698452551_i32) as u64;
_1 = -_5;
Call(_13.1.4 = fn7(_10.fld2.fld6.0, _10.fld2.fld1, _10.fld2.fld6, _6.fld4.0, _10.fld1.0, _6.fld1, _10.fld2.fld6, _10.fld2.fld0.0.1, _10.fld1, _6.fld2, _6.fld4, _6.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9.0 = _21 as u8;
_17 = (RET, _6.fld0.0.1);
_10.fld2.fld1 = _6.fld1;
_23.0 = _10.fld1.0;
_6.fld4.0 = core::ptr::addr_of_mut!(_6.fld3);
_8 = _11;
_15 = core::ptr::addr_of_mut!(_13.1.4);
_6.fld6.0 = _20 >> _6.fld3;
_1 = -_5;
_18 = _10.fld2.fld1 as f64;
_13.1.1 = _13.1.2;
_6.fld0.0.0 = RET;
_1 = !_5;
_5 = _1;
_13.1.1 = _6.fld3 == _6.fld3;
_6.fld6.0 = _13.1.1 as isize;
_21 = _9.1 as f32;
_10.fld1.0 = -_23.0;
match _10.fld0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb12,
156049336345861198900096861257007621917 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_9.0 = 9536_i16 as u8;
(*_15) = 4_usize;
_13.0 = [_6.fld2,_6.fld2,_6.fld2,_6.fld2,_6.fld2,_6.fld2,_6.fld2];
_10.fld2.fld0 = _6.fld0;
_6.fld3 = 3882925305_u32 as i8;
_8 = [_10.fld2.fld0.0.1,_17.1,_6.fld0.0.1,_10.fld2.fld0.0.1,_10.fld0,_6.fld0.0.1];
_30 = _17.0;
_10.fld2.fld0 = (_17,);
_27 = _10.fld2.fld1 << _6.fld1;
_10.fld2.fld0.0 = (_6.fld0.0.0, _6.fld0.0.1);
_29 = _13.1.1;
_17.1 = !_10.fld2.fld0.0.1;
_23.0 = _18 + _18;
_10.fld1.0 = _23.0;
_6.fld0.0.0 = _17.0;
_24 = (-720843582_i32) & (-408113623_i32);
_30 = RET;
_32 = !_6.fld6.0;
(*_15) = 0_usize;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(6_usize, 3_usize, Move(_3), 17_usize, Move(_17), 32_usize, Move(_32), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(6_usize, 16_usize, Move(_16), 7_usize, Move(_7), 30_usize, Move(_30), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: i128,mut _3: (isize,),mut _4: *mut i8,mut _5: f64,mut _6: i128,mut _7: (isize,),mut _8: u128,mut _9: (f64,),mut _10: u64,mut _11: (*mut i8,),mut _12: ((char, u128),)) -> usize {
mir! {
type RET = usize;
let _13: u8;
let _14: (u8, f64);
let _15: (char, u128);
let _16: isize;
let _17: (char, u128);
let _18: *mut (&'static f64, bool, bool, u64, usize);
let _19: (u8, f64);
let _20: (u8, f64);
let _21: Adt41;
let _22: *mut *mut (&'static f64, bool, bool, u64, usize);
let _23: [u64; 7];
let _24: ((&'static f64, bool, bool, u64, usize),);
let _25: char;
let _26: [u128; 6];
let _27: i16;
let _28: ((char, u128),);
let _29: (f64,);
let _30: u32;
let _31: ((char, u128),);
let _32: [bool; 5];
let _33: [u64; 7];
let _34: Adt48;
let _35: [i64; 7];
let _36: f32;
let _37: ();
let _38: ();
{
_3.0 = !_1;
RET = _12.0.0 as usize;
(*_4) = (-88_i8);
_11 = (_4,);
_9 = (_5,);
_7 = _3;
_3.0 = _8 as isize;
_12.0 = ('\u{71c0a}', _8);
_10 = 102_u8 as u64;
_11 = (_4,);
_7 = _3;
_14.0 = 215_u8 ^ 74_u8;
Call(_8 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15.1 = RET as u128;
match (*_4) {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211368 => bb6,
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
_6 = _2 + _2;
_10 = (*_4) as u64;
_12.0.1 = !_8;
_12.0.0 = '\u{4d73e}';
_14.1 = _5;
_14.1 = _9.0;
_9 = (_5,);
_12.0.0 = '\u{86d4c}';
_15.0 = _12.0.0;
_15.0 = _12.0.0;
_12.0.1 = _8 - _8;
_12.0.0 = _15.0;
_17.1 = !_12.0.1;
_11 = (_4,);
_9.0 = _5;
_17.0 = _15.0;
_3.0 = !_1;
_17 = (_15.0, _12.0.1);
_15.0 = _12.0.0;
_11 = (_4,);
_2 = _6;
_12.0 = _17;
_12.0.0 = _17.0;
_15.0 = _17.0;
(*_4) = (-85_i8);
Goto(bb7)
}
bb7 = {
_9.0 = -_14.1;
_17.1 = _12.0.1 << _6;
_15.1 = _12.0.1;
RET = 4267213234085359564_usize & 6_usize;
(*_4) = (-8_i8) + (-92_i8);
_19 = _14;
_17.1 = 28854_i16 as u128;
_13 = _19.0 ^ _19.0;
_17.1 = !_12.0.1;
_6 = !_2;
_17.0 = _12.0.0;
_20.0 = _14.0;
(*_4) = _10 as i8;
_20.1 = _19.1;
_9.0 = _19.1 - _5;
_16 = _15.0 as isize;
_20 = (_19.0, _9.0);
_15.0 = _12.0.0;
_11 = (_4,);
_12.0.1 = _17.1 >> _17.1;
_6 = _2 << _3.0;
Goto(bb8)
}
bb8 = {
_19.0 = _12.0.1 as u8;
_15.1 = _9.0 as u128;
_14.0 = !_19.0;
_15 = (_12.0.0, _17.1);
_23 = [_10,_10,_10,_10,_10,_10,_10];
_19.1 = -_9.0;
_15.1 = _17.1;
_24.0.1 = !true;
Goto(bb9)
}
bb9 = {
_28.0.1 = _12.0.1 * _17.1;
(*_4) = 9_i8;
_15.1 = !_12.0.1;
_20.0 = !_19.0;
_9.0 = _20.1 + _19.1;
RET = 4_usize ^ 12916287186233074672_usize;
_7.0 = _1;
_18 = core::ptr::addr_of_mut!(_24.0);
_32 = [(*_18).1,(*_18).1,(*_18).1,(*_18).1,(*_18).1];
(*_18).0 = &_20.1;
_29 = (_5,);
(*_18).0 = &_14.1;
_34.fld2.fld4.0 = _4;
_34.fld2.fld2 = _10;
_34.fld2.fld1 = _2 & _6;
_34.fld2.fld0.0.1 = _17.1;
_34.fld2.fld2 = (-628007955_i32) as u64;
_33 = [_10,_34.fld2.fld2,_10,_34.fld2.fld2,_34.fld2.fld2,_34.fld2.fld2,_10];
_31.0 = _15;
_27 = (-25313_i16);
_9 = _29;
_34.fld2.fld6 = _3;
_4 = _34.fld2.fld4.0;
match _27 {
0 => bb10,
1 => bb11,
2 => bb12,
340282366920938463463374607431768186143 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_9.0 = -_14.1;
_17.1 = _12.0.1 << _6;
_15.1 = _12.0.1;
RET = 4267213234085359564_usize & 6_usize;
(*_4) = (-8_i8) + (-92_i8);
_19 = _14;
_17.1 = 28854_i16 as u128;
_13 = _19.0 ^ _19.0;
_17.1 = !_12.0.1;
_6 = !_2;
_17.0 = _12.0.0;
_20.0 = _14.0;
(*_4) = _10 as i8;
_20.1 = _19.1;
_9.0 = _19.1 - _5;
_16 = _15.0 as isize;
_20 = (_19.0, _9.0);
_15.0 = _12.0.0;
_11 = (_4,);
_12.0.1 = _17.1 >> _17.1;
_6 = _2 << _3.0;
Goto(bb8)
}
bb12 = {
_15.1 = RET as u128;
match (*_4) {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211368 => bb6,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
(*_18).4 = _7.0 as usize;
(*_18).1 = _6 < _6;
_24.0.0 = &_9.0;
_24.0.0 = &_20.1;
_17.0 = _12.0.0;
_34.fld2.fld1 = _6 * _6;
_3 = (_1,);
(*_4) = (-101_i8) << _15.1;
_34.fld1 = (_29.0,);
_29 = _9;
_16 = _7.0 ^ _7.0;
_24.0.1 = true;
_19.0 = !_14.0;
_17.1 = _34.fld2.fld0.0.1 ^ _15.1;
(*_18).2 = (*_18).1;
_24.0.2 = _20.0 <= _20.0;
_34.fld0 = _17.1;
(*_18).0 = &_29.0;
(*_18).4 = RET - RET;
_24.0.0 = &_34.fld1.0;
_19 = (_20.0, _9.0);
_20.0 = _14.0;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 17_usize, Move(_17), 7_usize, Move(_7), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 6_usize, Move(_6), 10_usize, Move(_10), 31_usize, Move(_31), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64,mut _10: i64,mut _11: i64,mut _12: i64,mut _13: i64,mut _14: i64) -> i16 {
mir! {
type RET = i16;
let _15: Adt47;
let _16: [i64; 7];
let _17: i32;
let _18: [i64; 7];
let _19: u16;
let _20: Adt55;
let _21: Adt48;
let _22: (u32, i8);
let _23: Adt54;
let _24: [u128; 6];
let _25: char;
let _26: [i64; 7];
let _27: bool;
let _28: Adt44;
let _29: [i64; 7];
let _30: (u32, i8);
let _31: bool;
let _32: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _33: f32;
let _34: (isize,);
let _35: [u128; 6];
let _36: Adt51;
let _37: ();
let _38: ();
{
_5 = _1 * _3;
Call(_6 = fn9(_5, _5, _4, _1, _7, _11, _14, _9, _8, _5, _1, _2, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _8;
RET = -(-1510_i16);
Call(_15.fld6 = fn10(_14, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = _6 - _1;
_15.fld1 = (-74123542864906355037060467584179887766_i128);
_6 = -_8;
_15.fld4.0 = core::ptr::addr_of_mut!(_15.fld3);
_14 = _8;
_15.fld0.0 = ('\u{1882d}', 29501959462567886536053553285326288229_u128);
_18 = [_1,_1,_8,_13,_14,_12,_14];
_18 = [_8,_4,_9,_5,_12,_4,_2];
_15.fld6.0 = (-79_i8) as isize;
match _15.fld0.0.1 {
0 => bb1,
29501959462567886536053553285326288229 => bb4,
_ => bb3
}
}
bb3 = {
_5 = _8;
RET = -(-1510_i16);
Call(_15.fld6 = fn10(_14, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_6 = _11;
_21.fld2.fld6.0 = _15.fld6.0;
_7 = _10 | _13;
_21.fld2.fld6 = (_15.fld6.0,);
_13 = 17525804009188328757_u64 as i64;
_15.fld2 = 4106770925224016209_u64;
_9 = 164_u8 as i64;
_11 = !_2;
_21.fld2.fld3 = 1831033682_i32 as i8;
_21.fld2.fld4.0 = core::ptr::addr_of_mut!(_15.fld3);
_6 = _3;
Goto(bb5)
}
bb5 = {
_21.fld2.fld1 = RET as i128;
_22.0 = !3984927449_u32;
_15.fld3 = _21.fld2.fld3 & _21.fld2.fld3;
_13 = _4;
_17 = 1031784008_i32;
_21.fld2.fld0.0 = _15.fld0.0;
_22 = (1272385133_u32, _15.fld3);
_21.fld2.fld0.0 = (_15.fld0.0.0, _15.fld0.0.1);
_21.fld2.fld2 = _15.fld6.0 as u64;
_18 = [_8,_11,_3,_7,_7,_12,_5];
_15.fld6 = _21.fld2.fld6;
_2 = 1_usize as i64;
_24 = [_21.fld2.fld0.0.1,_15.fld0.0.1,_21.fld2.fld0.0.1,_21.fld2.fld0.0.1,_21.fld2.fld0.0.1,_21.fld2.fld0.0.1];
_7 = _14;
_21.fld1.0 = 162_u8 as f64;
_21.fld2.fld0.0 = (_15.fld0.0.0, _15.fld0.0.1);
_15.fld0.0.1 = _21.fld2.fld2 as u128;
_21.fld2.fld6 = _15.fld6;
_16 = [_1,_13,_10,_8,_12,_1,_6];
_16 = [_5,_3,_1,_11,_13,_12,_6];
Goto(bb6)
}
bb6 = {
_4 = _6;
_26 = [_11,_12,_8,_12,_10,_12,_6];
_21.fld2.fld6 = _15.fld6;
_21.fld2.fld0.0 = _15.fld0.0;
_21.fld2.fld3 = _15.fld2 as i8;
Call(_26 = fn13(_10, _12, _6, _5, _3, _18, _10, _12, _18, _15.fld4.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = !_6;
_15.fld6.0 = _21.fld2.fld6.0 - _21.fld2.fld6.0;
_15.fld0 = (_21.fld2.fld0.0,);
_21.fld2.fld0 = (_15.fld0.0,);
_25 = _15.fld0.0.0;
_27 = _6 > _5;
_7 = _6 & _5;
_7 = _3 & _8;
_21.fld2.fld0 = _15.fld0;
_9 = _1 & _3;
_12 = 24_u8 as i64;
_22.1 = _21.fld2.fld3;
_30.1 = _22.1;
_13 = _1;
_30.0 = _22.0;
_21.fld2.fld3 = _15.fld3;
match _30.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1272385133 => bb8,
_ => bb6
}
}
bb8 = {
_1 = _8 << _9;
_12 = _1 * _8;
Goto(bb9)
}
bb9 = {
RET = 27550_i16;
_21.fld0 = _21.fld2.fld0.0.1 ^ _15.fld0.0.1;
_15.fld0.0 = (_21.fld2.fld0.0.0, _21.fld0);
_32.3 = _21.fld2.fld2 as i8;
_19 = _21.fld2.fld2 as u16;
_32.1 = (_30.0, _15.fld3);
_8 = _5;
_21.fld2.fld0.0.0 = _15.fld0.0.0;
_21.fld2.fld2 = _27 as u64;
_32.0 = [_21.fld2.fld2,_21.fld2.fld2,_21.fld2.fld2,_21.fld2.fld2,_21.fld2.fld2,_21.fld2.fld2,_21.fld2.fld2];
_21.fld2.fld0.0 = _15.fld0.0;
_21.fld2.fld2 = _30.0 as u64;
_11 = 42_u8 as i64;
_32.5.1 = _15.fld2 as u128;
_21.fld2.fld4.0 = _15.fld4.0;
_31 = _6 != _4;
_21.fld2.fld3 = _32.1.1;
_30.1 = _21.fld2.fld3;
_24 = [_15.fld0.0.1,_21.fld2.fld0.0.1,_21.fld2.fld0.0.1,_15.fld0.0.1,_21.fld0,_21.fld0];
_21.fld2.fld2 = _21.fld2.fld6.0 as u64;
_32.1 = (_22.0, _21.fld2.fld3);
match _22.0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
1272385133 => bb15,
_ => bb14
}
}
bb10 = {
_5 = _8;
RET = -(-1510_i16);
Call(_15.fld6 = fn10(_14, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_8 = !_6;
_15.fld6.0 = _21.fld2.fld6.0 - _21.fld2.fld6.0;
_15.fld0 = (_21.fld2.fld0.0,);
_21.fld2.fld0 = (_15.fld0.0,);
_25 = _15.fld0.0.0;
_27 = _6 > _5;
_7 = _6 & _5;
_7 = _3 & _8;
_21.fld2.fld0 = _15.fld0;
_9 = _1 & _3;
_12 = 24_u8 as i64;
_22.1 = _21.fld2.fld3;
_30.1 = _22.1;
_13 = _1;
_30.0 = _22.0;
_21.fld2.fld3 = _15.fld3;
match _30.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1272385133 => bb8,
_ => bb6
}
}
bb12 = {
_4 = _6;
_26 = [_11,_12,_8,_12,_10,_12,_6];
_21.fld2.fld6 = _15.fld6;
_21.fld2.fld0.0 = _15.fld0.0;
_21.fld2.fld3 = _15.fld2 as i8;
Call(_26 = fn13(_10, _12, _6, _5, _3, _18, _10, _12, _18, _15.fld4.0), ReturnTo(bb7), UnwindUnreachable())
}
bb13 = {
_5 = _8;
RET = -(-1510_i16);
Call(_15.fld6 = fn10(_14, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_12 = _6 - _1;
_15.fld1 = (-74123542864906355037060467584179887766_i128);
_6 = -_8;
_15.fld4.0 = core::ptr::addr_of_mut!(_15.fld3);
_14 = _8;
_15.fld0.0 = ('\u{1882d}', 29501959462567886536053553285326288229_u128);
_18 = [_1,_1,_8,_13,_14,_12,_14];
_18 = [_8,_4,_9,_5,_12,_4,_2];
_15.fld6.0 = (-79_i8) as isize;
match _15.fld0.0.1 {
0 => bb1,
29501959462567886536053553285326288229 => bb4,
_ => bb3
}
}
bb15 = {
_21.fld2.fld0.0.0 = _15.fld0.0.0;
_29 = [_13,_12,_1,_9,_4,_1,_6];
_34.0 = _21.fld2.fld6.0 + _15.fld6.0;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(8_usize, 19_usize, Move(_19), 2_usize, Move(_2), 1_usize, Move(_1), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(8_usize, 16_usize, Move(_16), 7_usize, Move(_7), 10_usize, Move(_10), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(8_usize, 13_usize, Move(_13), 4_usize, Move(_4), 31_usize, Move(_31), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(8_usize, 29_usize, Move(_29), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64,mut _10: i64,mut _11: i64,mut _12: i64,mut _13: i64) -> i64 {
mir! {
type RET = i64;
let _14: (f64,);
let _15: char;
let _16: i64;
let _17: ();
let _18: ();
{
_11 = !_6;
_12 = 7_usize as i64;
_9 = (-1082034103_i32) as i64;
_3 = _1 * _8;
RET = _4 + _6;
RET = _1 >> _10;
_14.0 = (-39_i8) as f64;
_16 = -_11;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(9_usize, 10_usize, Move(_10), 13_usize, Move(_13), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(9_usize, 6_usize, Move(_6), 16_usize, Move(_16), 9_usize, Move(_9), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i64,mut _2: i64) -> (isize,) {
mir! {
type RET = (isize,);
let _3: i16;
let _4: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _5: (u32, i8);
let _6: (&'static f64, bool, bool, u64, usize);
let _7: char;
let _8: [bool; 5];
let _9: Adt46;
let _10: (u8, f64);
let _11: *mut (&'static f64, bool, bool, u64, usize);
let _12: u128;
let _13: char;
let _14: Adt54;
let _15: isize;
let _16: u128;
let _17: u16;
let _18: isize;
let _19: ([u64; 7], (&'static f64, bool, bool, u64, usize));
let _20: i64;
let _21: f32;
let _22: isize;
let _23: char;
let _24: Adt47;
let _25: ();
let _26: ();
{
RET = ((-9223372036854775808_isize),);
Call(RET.0 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = ((-9223372036854775808_isize),);
_2 = _1 - _1;
_3 = (-10418_i16) << _1;
RET.0 = 86486261945913683044173645074274773367_i128 as isize;
RET = ((-56_isize),);
RET.0 = (-114_isize);
_2 = _1 & _1;
_2 = _1 - _1;
_2 = _1 + _1;
RET = ((-9223372036854775808_isize),);
_3 = -(-18113_i16);
RET.0 = (-9223372036854775808_isize) | (-125_isize);
_3 = (-23889_i16) | (-29446_i16);
RET.0 = 9223372036854775807_isize << _1;
_2 = !_1;
_2 = -_1;
RET = ((-9223372036854775808_isize),);
_4.1 = (3898050273_u32, 63_i8);
_4.1.1 = 8_i8;
_4.3 = _4.1.0 as i8;
RET = ((-9223372036854775808_isize),);
RET = ((-9223372036854775808_isize),);
_5.0 = _4.1.0;
_4.5 = ('\u{83a4}', 52591429189189590965512832231524937953_u128);
Goto(bb2)
}
bb2 = {
_7 = _4.5.0;
_4.5.1 = 226792586759911383128277350769922411628_u128;
_5.1 = 7_usize as i8;
_4.3 = _4.5.1 as i8;
_6.3 = (-104798307836192101219978325581764319946_i128) as u64;
_2 = !_1;
RET = ((-9223372036854775808_isize),);
_4.1 = (_5.0, _5.1);
RET.0 = 80_isize;
RET.0 = (-9223372036854775808_isize);
_4.1.1 = _4.3 | _4.3;
_6.4 = 4_usize ^ 441792704794067215_usize;
_4.5.0 = _7;
_4.1 = (_5.0, _4.3);
_4.0 = [_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3];
_4.0 = [_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3];
_4.2 = _4.1.0 as i128;
_8 = [false,true,false,false,false];
RET.0 = _5.1 as isize;
_6.1 = false;
_5.0 = !_4.1.0;
_4.4 = -RET.0;
_4.4 = RET.0 + RET.0;
_4.1.0 = !_5.0;
_3 = _4.5.1 as i16;
_5.0 = _6.3 as u32;
_6.0 = &_10.1;
Goto(bb3)
}
bb3 = {
_4.5.0 = _7;
_8 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_6.2 = _2 < _1;
_2 = _1;
_2 = _3 as i64;
_7 = _4.5.0;
_3 = 20579_i16;
_4.2 = _4.4 as i128;
_10.0 = _4.3 as u8;
_2 = _1;
RET.0 = -_4.4;
_8 = [_6.2,_6.2,_6.2,_6.2,_6.2];
_6.4 = 12686054259351822052_usize ^ 1_usize;
_11 = core::ptr::addr_of_mut!(_6);
(*_11).3 = 4055915467630368580_u64 & 4459695348015948683_u64;
_5 = _4.1;
(*_11).1 = (*_11).2 | (*_11).2;
_4.4 = !RET.0;
_4.5.1 = 126540190650106672410088975925243851256_u128;
(*_11).2 = (*_11).1 | _6.1;
Call(_1 = core::intrinsics::transmute((*_11).3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = _2 * _2;
_12 = _4.5.1;
_4.2 = 9089702167352537798491925168242715052_i128;
_4.1.1 = -_5.1;
Call(_10.0 = fn11((*_11).2, Move(_6), Move(_11), _8, _1, _1, _2, _8, _8, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5.1 = 10831_u16 as i8;
_6.1 = !true;
Goto(bb6)
}
bb6 = {
_6.4 = _3 as usize;
_6.2 = _2 >= _1;
_10.1 = 54145_u16 as f64;
Call(_4.1 = fn12(_1, _10.0, _10, _8, _6.2, _8, _8, _1, _10.0, _10, _10.0, _10.0, _1, _10, _1, _12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7 = _4.5.0;
_10.0 = !13_u8;
_4.4 = RET.0 << _1;
_6.4 = 18137970947496937893_usize ^ 11815407158222695509_usize;
_4.5 = (_7, _12);
_5.1 = _4.3 ^ _4.3;
_4.1 = (_5.0, _4.3);
match _4.5.1 {
0 => bb8,
126540190650106672410088975925243851256 => bb10,
_ => bb9
}
}
bb8 = {
_4.5.0 = _7;
_8 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_6.2 = _2 < _1;
_2 = _1;
_2 = _3 as i64;
_7 = _4.5.0;
_3 = 20579_i16;
_4.2 = _4.4 as i128;
_10.0 = _4.3 as u8;
_2 = _1;
RET.0 = -_4.4;
_8 = [_6.2,_6.2,_6.2,_6.2,_6.2];
_6.4 = 12686054259351822052_usize ^ 1_usize;
_11 = core::ptr::addr_of_mut!(_6);
(*_11).3 = 4055915467630368580_u64 & 4459695348015948683_u64;
_5 = _4.1;
(*_11).1 = (*_11).2 | (*_11).2;
_4.4 = !RET.0;
_4.5.1 = 126540190650106672410088975925243851256_u128;
(*_11).2 = (*_11).1 | _6.1;
Call(_1 = core::intrinsics::transmute((*_11).3), ReturnTo(bb4), UnwindUnreachable())
}
bb9 = {
_1 = _2 * _2;
_12 = _4.5.1;
_4.2 = 9089702167352537798491925168242715052_i128;
_4.1.1 = -_5.1;
Call(_10.0 = fn11((*_11).2, Move(_6), Move(_11), _8, _1, _1, _2, _8, _8, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb10 = {
_15 = _4.4;
_6.0 = &_10.1;
_6.4 = _15 as usize;
_4.1 = (_5.0, _4.3);
_4.1 = (_5.0, _5.1);
_11 = core::ptr::addr_of_mut!(_6);
_4.4 = _15 + _15;
(*_11).3 = 1020745638983428572_u64 >> _15;
_4.5 = (_7, _12);
_6.2 = _15 >= _15;
(*_11).2 = (*_11).1;
(*_11).1 = (*_11).2 | _6.2;
_16 = _12 & _12;
_4.1.1 = (*_11).1 as i8;
(*_11).2 = !(*_11).1;
_4.2 = (*_11).2 as i128;
_17 = 5555_u16 - 55397_u16;
Goto(bb11)
}
bb11 = {
_4.5.0 = _7;
_4.5.0 = _7;
_10.1 = _10.0 as f64;
(*_11).3 = 4440386676471319268_u64 | 16429380475416233766_u64;
_2 = _1;
(*_11).0 = &_10.1;
(*_11).0 = &_10.1;
_19.1.4 = !_6.4;
(*_11).1 = _6.2;
RET = (_15,);
_19.1.2 = RET.0 != _4.4;
_13 = _7;
_1 = _7 as i64;
_1 = -_2;
(*_11).1 = _19.1.2;
_4.1 = (_5.0, _5.1);
_4.1.0 = !_5.0;
_18 = _15 << (*_11).4;
(*_11).1 = !_19.1.2;
Goto(bb12)
}
bb12 = {
Call(_25 = dump_var(10_usize, 17_usize, Move(_17), 16_usize, Move(_16), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_25 = dump_var(10_usize, 3_usize, Move(_3), 2_usize, Move(_2), 26_usize, _26, 26_usize, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: (&'static f64, bool, bool, u64, usize),mut _3: *mut (&'static f64, bool, bool, u64, usize),mut _4: [bool; 5],mut _5: i64,mut _6: i64,mut _7: i64,mut _8: [bool; 5],mut _9: [bool; 5],mut _10: [bool; 5]) -> u8 {
mir! {
type RET = u8;
let _11: i8;
let _12: isize;
let _13: bool;
let _14: u32;
let _15: ();
let _16: ();
{
_1 = _5 > _7;
_4 = _10;
RET = (-574542556_i32) as u8;
_8 = [_1,_1,_1,_1,_2.1];
_10 = [_2.2,_2.2,_1,_2.1,_2.1];
_3 = core::ptr::addr_of_mut!(_2);
_7 = _6 + _6;
_1 = (*_3).1;
(*_3).3 = !10254960807413560702_u64;
_2.3 = 1915227444398413100_u64;
_13 = _2.2 == (*_3).2;
(*_3).3 = !6605299336165536834_u64;
_8 = [(*_3).2,_13,(*_3).1,(*_3).1,_2.2];
_7 = !_5;
_2.4 = !6_usize;
(*_3).3 = 16457561762479318382_u64 << _6;
_12 = 4_isize + 79_isize;
RET = 203_u8 << _6;
(*_3).2 = (*_3).1;
(*_3).4 = (-1034780524204431017270445674551417217_i128) as usize;
_8 = [_2.1,(*_3).2,(*_3).1,_1,(*_3).1];
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(11_usize, 10_usize, Move(_10), 9_usize, Move(_9), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(11_usize, 12_usize, Move(_12), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i64,mut _2: u8,mut _3: (u8, f64),mut _4: [bool; 5],mut _5: bool,mut _6: [bool; 5],mut _7: [bool; 5],mut _8: i64,mut _9: u8,mut _10: (u8, f64),mut _11: u8,mut _12: u8,mut _13: i64,mut _14: (u8, f64),mut _15: i64,mut _16: u128) -> (u32, i8) {
mir! {
type RET = (u32, i8);
let _17: Adt56;
let _18: (char, u128);
let _19: (f64,);
let _20: (char, u128);
let _21: i128;
let _22: Adt54;
let _23: f64;
let _24: u128;
let _25: isize;
let _26: f64;
let _27: ();
let _28: ();
{
_10.1 = _3.1 + _14.1;
_10.1 = _14.1 * _14.1;
_1 = _8 ^ _15;
_14.0 = !_10.0;
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
126540190650106672410088975925243851256 => bb5,
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
_2 = !_10.0;
_3.0 = 11263_i16 as u8;
_7 = [_5,_5,_5,_5,_5];
_1 = _13;
_18 = ('\u{ced27}', _16);
match _16 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
126540190650106672410088975925243851256 => bb11,
_ => bb10
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
RET = (983860772_u32, (-80_i8));
_6 = [_5,_5,_5,_5,_5];
_3 = (_14.0, _14.1);
_2 = _10.0 | _14.0;
_10.0 = !_12;
_18.0 = '\u{10e2ad}';
match RET.1 {
340282366920938463463374607431768211376 => bb12,
_ => bb4
}
}
bb12 = {
_17 = Adt56::Variant1 { fld0: _5,fld1: _12 };
_4 = [_5,_5,Field::<bool>(Variant(_17, 1), 0),Field::<bool>(Variant(_17, 1), 0),Field::<bool>(Variant(_17, 1), 0)];
RET.0 = 1469279365_u32 ^ 2106822455_u32;
_3.0 = !_14.0;
_18 = ('\u{ceb0c}', _16);
_20.1 = RET.0 as u128;
_1 = _13 >> _8;
SetDiscriminant(_17, 1);
place!(Field::<bool>(Variant(_17, 1), 0)) = _5;
_1 = _15;
_16 = 122805334968392243677140735422794390260_i128 as u128;
_14 = (_2, _10.1);
_19 = (_10.1,);
_9 = 102819611536585740562560507681561463926_i128 as u8;
_18 = ('\u{5ea0b}', _20.1);
_8 = !_13;
RET.1 = 12_i8;
_20.0 = _18.0;
RET = (4165810364_u32, (-110_i8));
_11 = _10.0;
match RET.1 {
0 => bb13,
340282366920938463463374607431768211346 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_2 = !_10.0;
_3.0 = 11263_i16 as u8;
_7 = [_5,_5,_5,_5,_5];
_1 = _13;
_18 = ('\u{ced27}', _16);
match _16 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
126540190650106672410088975925243851256 => bb11,
_ => bb10
}
}
bb15 = {
_18 = (_20.0, _20.1);
_4 = [_5,_5,Field::<bool>(Variant(_17, 1), 0),_5,_5];
_6 = [_5,Field::<bool>(Variant(_17, 1), 0),_5,Field::<bool>(Variant(_17, 1), 0),Field::<bool>(Variant(_17, 1), 0)];
_13 = _15;
_8 = 30362_u16 as i64;
_20.0 = _18.0;
_9 = _10.0;
_20 = (_18.0, _16);
RET.1 = !(-6_i8);
place!(Field::<u8>(Variant(_17, 1), 1)) = _2;
_10 = (_14.0, _3.1);
SetDiscriminant(_17, 1);
_25 = -(-9223372036854775808_isize);
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(12_usize, 7_usize, Move(_7), 11_usize, Move(_11), 13_usize, Move(_13), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(12_usize, 12_usize, Move(_12), 9_usize, Move(_9), 25_usize, Move(_25), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: [i64; 7],mut _7: i64,mut _8: i64,mut _9: [i64; 7],mut _10: *mut i8) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _11: Adt43;
let _12: char;
let _13: bool;
let _14: f64;
let _15: char;
let _16: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128));
let _17: *mut (u8, f64);
let _18: f64;
let _19: bool;
let _20: u128;
let _21: ((char, u128),);
let _22: ();
let _23: ();
{
_8 = _5 * _4;
Goto(bb1)
}
bb1 = {
_11.fld6 = _7 & _8;
_11.fld0 = false;
_4 = 33288_u16 as i64;
_11.fld3.0.1 = !325467705543709195192541663466494081143_u128;
_13 = _11.fld0;
_13 = !_11.fld0;
_11.fld3.0.1 = !212613259377652757994169259844637317174_u128;
Goto(bb2)
}
bb2 = {
_15 = '\u{eda79}';
_16.0 = [17407678876091176173_u64,18392278440561209445_u64,18132968005928495758_u64,2496146587088455626_u64,4434704226077045699_u64,17095152449608288328_u64,12850716006677685445_u64];
_5 = _11.fld6 & _3;
_14 = 9948_u16 as f64;
_15 = '\u{13f89}';
RET = [_7,_2,_11.fld6,_2,_5,_3,_5];
_10 = core::ptr::addr_of_mut!(_16.1.1);
_11.fld4 = 13695561869184197328_u64 >> _2;
_16.2 = 73286564138636640043275279489886121418_i128 + 85930368920327954086199135886369500959_i128;
_16.5 = (_15, _11.fld3.0.1);
_16.5 = (_15, _11.fld3.0.1);
_16.1.1 = -(-29_i8);
_16.3 = _16.2 as i8;
_9 = [_8,_8,_5,_3,_7,_11.fld6,_1];
RET = [_2,_2,_8,_2,_5,_1,_11.fld6];
_11.fld3.0.0 = _16.5.0;
_16.5.0 = _11.fld3.0.0;
_16.4 = 38_isize | 88_isize;
_4 = _5;
_15 = _11.fld3.0.0;
_20 = _8 as u128;
_12 = _11.fld3.0.0;
_11.fld2 = _16.4;
_20 = _16.5.1 >> _4;
_11.fld1 = core::ptr::addr_of!(_14);
Goto(bb3)
}
bb3 = {
Call(_22 = dump_var(13_usize, 6_usize, Move(_6), 2_usize, Move(_2), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_22 = dump_var(13_usize, 12_usize, Move(_12), 4_usize, Move(_4), 23_usize, _23, 23_usize, _23), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: i64,mut _10: i8) -> f64 {
mir! {
type RET = f64;
let _11: (u32, i8);
let _12: isize;
let _13: i64;
let _14: u16;
let _15: [i64; 7];
let _16: [u128; 6];
let _17: u128;
let _18: (isize,);
let _19: f32;
let _20: [i64; 7];
let _21: isize;
let _22: Adt54;
let _23: (u8, f64);
let _24: char;
let _25: isize;
let _26: ();
let _27: ();
{
_6 = -_5;
_6 = _8;
_1 = _2 + _6;
RET = 2025972038_i32 as f64;
_5 = _2;
_10 = (-74_i8);
_9 = '\u{24cea}' as i64;
_8 = _3;
_12 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_7 = RET as i64;
match _10 {
0 => bb1,
340282366920938463463374607431768211382 => bb3,
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
_11 = (3095818854_u32, _10);
_7 = !_6;
_6 = _5 | _4;
_8 = -_2;
_11.1 = -_10;
_4 = _7;
RET = 58780_u16 as f64;
_7 = _4;
_5 = _4 << _3;
_13 = !_4;
_11.1 = !_10;
_6 = (-37260293056685954288891085877114484368_i128) as i64;
_9 = !_4;
_1 = _2 - _7;
RET = 456_u16 as f64;
_11.0 = !1995343717_u32;
_8 = (-8824210738416804975967946777141508561_i128) as i64;
_12 = _11.0 as isize;
RET = _11.0 as f64;
_8 = -_7;
_4 = _8;
_3 = (-649480132_i32) as i64;
match _10 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431768211382 => bb8,
_ => bb7
}
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
_6 = -_8;
_11.0 = 2561668590_u32;
_9 = -_13;
_9 = -_4;
RET = _12 as f64;
_4 = 17788104813087662905_u64 as i64;
_5 = 6937937568079845871129544584573808782_u128 as i64;
_9 = -_2;
_10 = RET as i8;
_11 = (2749709396_u32, _10);
_8 = 164190166850610963542812285584207682411_u128 as i64;
_14 = 3967_u16;
RET = _10 as f64;
_14 = !5741_u16;
_9 = -_13;
_4 = _9 ^ _2;
_12 = !(-111_isize);
_15 = [_4,_1,_7,_1,_13,_13,_13];
_10 = !_11.1;
_6 = _13 ^ _4;
_2 = _6;
_12 = -(-37_isize);
match _11.0 {
0 => bb1,
1 => bb2,
2749709396 => bb9,
_ => bb3
}
}
bb9 = {
_5 = _13 - _6;
_11.1 = _12 as i8;
_16 = [315949326893211667278635066278234511012_u128,322648903825451228516825274630240761909_u128,277952921737534554998938919354148294561_u128,117299309629879779063682387720385562186_u128,55159546355934147998537704876933381575_u128,176294951255083150446445273344519398741_u128];
_2 = -_7;
Call(_6 = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = _4 << _1;
_6 = !_7;
RET = _11.1 as f64;
RET = _11.0 as f64;
_7 = _2;
_18.0 = _12;
_11.0 = !1000231277_u32;
_14 = _13 as u16;
_17 = _2 as u128;
_1 = _13;
_6 = _11.0 as i64;
_14 = _11.1 as u16;
_20 = [_7,_4,_4,_1,_7,_4,_1];
_10 = _11.1;
_8 = _7 | _4;
_16 = [_17,_17,_17,_17,_17,_17];
_14 = !58718_u16;
_9 = 12627886933413389006_u64 as i64;
_1 = _8 - _2;
_11.1 = _10 * _10;
RET = 12853409343551339152_u64 as f64;
Goto(bb11)
}
bb11 = {
_16 = [_17,_17,_17,_17,_17,_17];
_12 = _18.0 << _5;
_19 = 161_u8 as f32;
_8 = false as i64;
_4 = _7;
_6 = false as i64;
_3 = _2;
_20 = [_5,_4,_3,_5,_13,_1,_4];
_19 = _17 as f32;
_8 = _7;
RET = 234_u8 as f64;
_4 = !_1;
_15 = [_3,_1,_1,_2,_3,_4,_2];
_18.0 = !_12;
_11.0 = !4204885012_u32;
_7 = _2 - _3;
_6 = _3 * _8;
_16 = [_17,_17,_17,_17,_17,_17];
_15 = [_8,_3,_5,_1,_7,_5,_1];
_19 = 2946908932033865843_u64 as f32;
_15 = [_7,_6,_4,_3,_3,_1,_4];
_15 = [_5,_2,_4,_1,_3,_7,_6];
_2 = !_6;
_11 = (221388425_u32, _10);
RET = (-1872_i16) as f64;
_21 = _17 as isize;
_11.0 = 877537557_u32 & 26306469_u32;
_23 = (63_u8, RET);
_16 = [_17,_17,_17,_17,_17,_17];
match _23.0 {
0 => bb12,
63 => bb14,
_ => bb13
}
}
bb12 = {
_2 = _4 << _1;
_6 = !_7;
RET = _11.1 as f64;
RET = _11.0 as f64;
_7 = _2;
_18.0 = _12;
_11.0 = !1000231277_u32;
_14 = _13 as u16;
_17 = _2 as u128;
_1 = _13;
_6 = _11.0 as i64;
_14 = _11.1 as u16;
_20 = [_7,_4,_4,_1,_7,_4,_1];
_10 = _11.1;
_8 = _7 | _4;
_16 = [_17,_17,_17,_17,_17,_17];
_14 = !58718_u16;
_9 = 12627886933413389006_u64 as i64;
_1 = _8 - _2;
_11.1 = _10 * _10;
RET = 12853409343551339152_u64 as f64;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_19 = _14 as f32;
_12 = _18.0 + _21;
_24 = '\u{6f434}';
_19 = 8991506143184923470_u64 as f32;
_4 = _19 as i64;
_16 = [_17,_17,_17,_17,_17,_17];
_16 = [_17,_17,_17,_17,_17,_17];
_23 = (55_u8, RET);
_23 = (96_u8, RET);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(14_usize, 15_usize, Move(_15), 24_usize, Move(_24), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(14_usize, 8_usize, Move(_8), 14_usize, Move(_14), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(14_usize, 7_usize, Move(_7), 20_usize, Move(_20), 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(15983_u16), std::hint::black_box((-102_isize)), std::hint::black_box((-46_i8)), std::hint::black_box(691098930746575142_i64));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: u32,
fld1: i16,
fld2: u128,

},
Variant1{
fld0: u16,
fld1: (u32, i8),
fld2: *mut usize,
fld3: [bool; 5],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [u64; 7],
fld1: char,
fld2: *mut usize,
fld3: (u8, f64),
fld4: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128)),
fld5: (i8, i16),
fld6: (isize,),

},
Variant1{
fld0: bool,
fld1: (char, u128),
fld2: u32,
fld3: u64,
fld4: i16,
fld5: (u32, i8),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: bool,
fld1: *const f64,
fld2: isize,
fld3: ((char, u128),),
fld4: u64,
fld5: (f64,),
fld6: i64,
fld7: i128,
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: Adt43,

},
Variant1{
fld0: ((char, u128),),
fld1: (u32, i8),
fld2: *mut i8,
fld3: (f64,),
fld4: i16,
fld5: f32,
fld6: u64,

},
Variant2{
fld0: bool,
fld1: (isize,),

},
Variant3{
fld0: i32,
fld1: (u32, i8),
fld2: isize,
fld3: (char, u128),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
fld0: Adt44,
fld1: (isize,),
fld2: [bool; 5],
fld3: (char, u128),
fld4: *const f64,

},
Variant1{
fld0: (i8, i16),
fld1: f32,
fld2: i64,
fld3: i8,
fld4: (char, u128),
fld5: (isize,),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut i8,
fld1: (u32, i8),
fld2: u8,

},
Variant1{
fld0: *const f64,

},
Variant2{
fld0: Adt41,
fld1: *mut usize,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: ((char, u128),),
fld1: i128,
fld2: u64,
fld3: i8,
fld4: (*mut i8,),
fld5: Adt45,
fld6: (isize,),
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: u128,
fld1: (f64,),
fld2: Adt47,
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: i64,
fld1: char,
fld2: *const f64,
fld3: i8,
fld4: (u8, f64),
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: i16,

},
Variant1{
fld0: Adt41,
fld1: Adt45,
fld2: (u32, i8),
fld3: *mut (u8, f64),

},
Variant2{
fld0: bool,
fld1: char,
fld2: Adt47,
fld3: Adt43,
fld4: i16,
fld5: Adt44,
fld6: i64,
fld7: u128,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [bool; 5],
fld1: Adt48,
fld2: u16,
fld3: i8,
fld4: *mut (u8, f64),
fld5: i32,
fld6: f32,
fld7: *mut usize,

},
Variant1{
fld0: i64,
fld1: [u128; 6],
fld2: (u8, f64),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt47,
fld1: *mut usize,
fld2: *const f64,

},
Variant1{
fld0: (u32, i8),
fld1: f32,
fld2: Adt42,
fld3: Adt46,
fld4: f64,
fld5: u8,
fld6: i64,

},
Variant2{
fld0: (u32, i8),
fld1: [u64; 7],
fld2: usize,
fld3: Adt45,
fld4: (isize,),
fld5: u8,
fld6: Adt42,

},
Variant3{
fld0: [u64; 7],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: f64,
fld1: usize,
fld2: isize,
fld3: Adt41,
fld4: [i64; 7],
fld5: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128)),
fld6: (i8, i16),

},
Variant1{
fld0: (*mut i8,),
fld1: i64,
fld2: (u8, f64),

},
Variant2{
fld0: Adt50,
fld1: char,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: (u32, i8),
fld2: Adt43,
fld3: Adt51,
fld4: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128)),
fld5: i32,
fld6: (char, u128),
fld7: Adt53,

},
Variant1{
fld0: u128,
fld1: Adt49,
fld2: ([u64; 7], (u32, i8), i128, i8, isize, (char, u128)),
fld3: (u8, f64),
fld4: *mut (u8, f64),
fld5: i32,
fld6: u64,
fld7: Adt45,

},
Variant2{
fld0: u16,
fld1: Adt46,
fld2: *mut i8,
fld3: Adt42,
fld4: Adt53,
fld5: i32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: (u8, f64),
fld1: Adt52,
fld2: Adt49,

},
Variant1{
fld0: Adt45,
fld1: (isize,),
fld2: Adt54,
fld3: f64,
fld4: Adt48,

},
Variant2{
fld0: Adt53,
fld1: *const f64,
fld2: (*mut i8,),
fld3: (u32, i8),

},
Variant3{
fld0: bool,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt46,
fld1: char,
fld2: (i8, i16),
fld3: (isize,),
fld4: u128,
fld5: Adt51,
fld6: *mut i8,

},
Variant1{
fld0: bool,
fld1: u8,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [i64; 7],
fld1: (i8, i16),

},
Variant1{
fld0: u16,
fld1: (isize,),
fld2: i16,

},
Variant2{
fld0: Adt42,
fld1: i32,
fld2: u8,
fld3: [u128; 6],

}}

