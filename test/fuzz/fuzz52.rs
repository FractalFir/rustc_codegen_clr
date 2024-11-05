#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics)]
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
pub fn fn0(mut _1: i128,mut _2: i32,mut _3: u8,mut _4: i8) -> u16 {
mir! {
type RET = u16;
let _5: isize;
let _6: char;
let _7: f32;
let _8: f64;
let _9: [i8; 5];
let _10: Adt52;
let _11: u64;
let _12: Adt48;
let _13: Adt50;
let _14: isize;
let _15: u16;
let _16: i128;
let _17: [i8; 2];
let _18: [i64; 6];
let _19: [u8; 4];
let _20: [i8; 5];
let _21: (u64, usize, u64);
let _22: Adt47;
let _23: f64;
let _24: Adt62;
let _25: char;
let _26: bool;
let _27: *mut i8;
let _28: Adt57;
let _29: isize;
let _30: isize;
let _31: [i8; 5];
let _32: *const ((u8,), u32, u32, char);
let _33: f32;
let _34: bool;
let _35: (i64,);
let _36: [i64; 6];
let _37: u16;
let _38: ((u8,), u32, u32, char);
let _39: i8;
let _40: Adt46;
let _41: [u8; 7];
let _42: (i8, i8, usize, ((u8,), u32, u32, char), u8);
let _43: ((i64,),);
let _44: (i64,);
let _45: u8;
let _46: i32;
let _47: char;
let _48: ([i64; 6], usize, *mut u128);
let _49: u8;
let _50: (u64, usize, u64);
let _51: isize;
let _52: (isize, f64);
let _53: Adt56;
let _54: ((u8,), u32, u32, char);
let _55: ();
let _56: ();
{
_1 = (-79484354155314828401774252000557153868_i128);
_1 = (-77125386976658004610055832777200256284_i128) ^ 129999423949363592886834783035145573935_i128;
_4 = 31595_u16 as i8;
RET = _4 as u16;
_5 = RET as isize;
_2 = _4 as i32;
_3 = 176_u8;
_5 = 70_isize;
RET = !4299_u16;
_4 = 108_i8 + (-99_i8);
_8 = 2117577241123728910_u64 as f64;
Call(RET = fn1(_4, _5, _5, _5, _5, _1, _5, _4, _5, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 32469_u16;
_1 = (-119740730781703444969341772668177447160_i128);
_8 = _3 as f64;
_4 = (-94_i8) >> _2;
_5 = 9223372036854775807_isize;
RET = '\u{d5a81}' as u16;
RET = 42546_u16;
_8 = 2_usize as f64;
_2 = _1 as i32;
_9 = [_4,_4,_4,_4,_4];
_1 = 92401929021017814567803154432361418085_i128;
_6 = '\u{48ced}';
_1 = (-30270186436314928059055010843433105722_i128);
_6 = '\u{ed0db}';
Goto(bb2)
}
bb2 = {
_4 = !(-61_i8);
_9 = [_4,_4,_4,_4,_4];
_1 = (-149405236345612177879650052915571686313_i128);
_8 = _2 as f64;
RET = 26026_u16;
RET = !18515_u16;
_11 = 13044783689340838221_u64 & 1128419905542836712_u64;
_7 = _3 as f32;
_6 = '\u{701fb}';
_6 = '\u{c21a4}';
Call(_1 = fn6(_6, _6, _2, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = 2_usize as i128;
_1 = !88688390153825255545810723283889221609_i128;
_12.fld1.1 = -_8;
Goto(bb4)
}
bb4 = {
_12.fld2.0.2 = !_11;
_12.fld1.1 = -_8;
_12.fld2.0.1 = (-6124414040545158032_i64) as usize;
_12.fld2.1 = [(-7352508470396777726_i64),520992782684962924_i64,(-6423959556797493934_i64),(-6721934338235966581_i64),5742965890593149329_i64,(-6860397473521595302_i64)];
_5 = !9223372036854775807_isize;
_14 = _5 >> _4;
_12.fld2.0 = (_11, 2757341348287297547_usize, _11);
_12.fld2.3 = [_4,_4,_4,_4,_4];
_12.fld1 = (_5, _8);
_12.fld1.1 = -_8;
_12.fld3 = !_4;
_12.fld3 = !_4;
_12.fld1 = (_14, _8);
_12.fld2.0 = (_11, 1253357523385159306_usize, _11);
_12.fld4.0 = _12.fld2.0.1;
_12.fld3 = _4 ^ _4;
_12.fld2.0.2 = !_12.fld2.0.0;
_2 = _6 as i32;
_6 = '\u{d452c}';
_6 = '\u{2077b}';
_12.fld2.0.1 = !_12.fld4.0;
_12.fld2.1 = [(-5716234274943210805_i64),(-6416745853135889640_i64),3567570033227610917_i64,(-8798115577647651338_i64),(-4966738942390949346_i64),(-2497324848911866138_i64)];
_16 = _1;
_9 = [_4,_4,_4,_12.fld3,_12.fld3];
Goto(bb5)
}
bb5 = {
_12.fld2.0.2 = _12.fld2.0.0;
_12.fld2.0.2 = _11;
RET = !33322_u16;
Goto(bb6)
}
bb6 = {
_18 = [1791010681006778835_i64,(-4418709021341837724_i64),(-6318690747590576002_i64),9005685283140446114_i64,8356636257528554845_i64,(-1477965204993653069_i64)];
_6 = '\u{447b6}';
_15 = !RET;
_12.fld4.0 = !_12.fld2.0.1;
RET = _3 as u16;
_12.fld1 = (_5, _8);
_21.1 = _12.fld2.0.1;
_19 = [_3,_3,_3,_3];
_12.fld2.0.1 = _8 as usize;
_12.fld3 = -_4;
_17 = [_12.fld3,_4];
_23 = _11 as f64;
_18 = _12.fld2.1;
_12.fld1 = (_14, _8);
_12.fld2.0.1 = _12.fld4.0 ^ _21.1;
RET = _15 | _15;
RET = !_15;
_12.fld1.0 = _14;
_7 = _12.fld2.0.1 as f32;
_21.0 = _6 as u64;
_21.2 = !_11;
_1 = _5 as i128;
_22 = Adt47::Variant1 { fld0: _12.fld2.0.1,fld1: _19,fld2: _15 };
_21.2 = _12.fld2.0.2 & _11;
Goto(bb7)
}
bb7 = {
_4 = _12.fld1.0 as i8;
_21.2 = _11;
_20 = _9;
_12.fld4 = (_12.fld2.0.1,);
_12.fld1.1 = _8;
_12.fld1.1 = _8 - _23;
_19 = [_3,_3,_3,_3];
_12.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_8 = _23;
_2 = !(-1204791524_i32);
_12.fld2.0 = (_21.2, _21.1, _11);
_12.fld1.1 = _23 + _23;
_21.1 = _12.fld4.0;
SetDiscriminant(_22, 1);
_12.fld4.0 = _1 as usize;
_12.fld1.1 = _23 * _23;
_23 = _8 + _12.fld1.1;
_23 = -_12.fld1.1;
_4 = _12.fld3;
Goto(bb8)
}
bb8 = {
_12.fld3 = _4;
_26 = _23 == _23;
_19 = [_3,_3,_3,_3];
place!(Field::<u16>(Variant(_22, 1), 2)) = !_15;
_17 = [_4,_12.fld3];
_29 = _1 as isize;
_12.fld3 = _4;
_12.fld2.0.0 = 2276926226_u32 as u64;
_9 = [_12.fld3,_4,_4,_4,_4];
_27 = core::ptr::addr_of_mut!(_12.fld3);
_12.fld4 = (_12.fld2.0.1,);
_34 = _26;
_17 = [(*_27),_12.fld3];
_8 = -_12.fld1.1;
_1 = _3 as i128;
_4 = (*_27);
_8 = _5 as f64;
_30 = !_12.fld1.0;
_35 = ((-1538779904485074277_i64),);
_12.fld2.0.1 = 183664618278039938851219726298543505240_u128 as usize;
_23 = 326940979132715913812535583703924882152_u128 as f64;
_11 = _21.0 << _30;
_12.fld1 = (_14, _23);
_3 = 94_u8;
_2 = 978622903_i32 >> _21.1;
_12.fld1.1 = -_23;
Call(_20 = core::intrinsics::transmute(_9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<u16>(Variant(_22, 1), 2)) = RET | _15;
_15 = _4 as u16;
_21.2 = _11 | _11;
_25 = _6;
place!(Field::<u16>(Variant(_22, 1), 2)) = _15;
_33 = _15 as f32;
_38.2 = !3820069058_u32;
_12.fld2.0.1 = _12.fld4.0 << _12.fld4.0;
place!(Field::<usize>(Variant(_22, 1), 0)) = _12.fld2.0.1 * _12.fld2.0.1;
_12.fld4.0 = !Field::<usize>(Variant(_22, 1), 0);
_21 = (_12.fld2.0.2, _12.fld4.0, _11);
(*_27) = !_4;
place!(Field::<[u8; 4]>(Variant(_22, 1), 1)) = _19;
_17 = [(*_27),(*_27)];
_21.1 = (*_27) as usize;
_12.fld1.1 = _8 - _8;
_12.fld1.0 = !_14;
_18 = _12.fld2.1;
_15 = !Field::<u16>(Variant(_22, 1), 2);
(*_27) = _4 * _4;
_31 = [_4,_4,_4,(*_27),_12.fld3];
_42.3.2 = _38.2 ^ _38.2;
RET = _3 as u16;
_12.fld2.0.2 = !_21.0;
Goto(bb10)
}
bb10 = {
_27 = core::ptr::addr_of_mut!(_39);
_12.fld2.3 = _31;
_35.0 = _26 as i64;
_19 = [_3,_3,_3,_3];
_27 = core::ptr::addr_of_mut!((*_27));
_21.0 = !_11;
_12.fld4 = (Field::<usize>(Variant(_22, 1), 0),);
_38.0.0 = _3 & _3;
_42.3.0 = (_38.0.0,);
_41 = _12.fld0;
_20 = _12.fld2.3;
(*_27) = _12.fld3;
_38.3 = _6;
_4 = !_39;
_44 = (_35.0,);
RET = Field::<u16>(Variant(_22, 1), 2) & Field::<u16>(Variant(_22, 1), 2);
Call(_12.fld2.0.1 = core::intrinsics::transmute(_12.fld4.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2 = (-941516023_i32) - (-1135156529_i32);
_34 = _26;
_25 = _38.3;
_38 = (_42.3.0, _42.3.2, _42.3.2, _25);
SetDiscriminant(_22, 1);
_19 = [_38.0.0,_42.3.0.0,_38.0.0,_3];
_15 = RET ^ RET;
_37 = _15 | _15;
_42.1 = -_4;
_4 = -_42.1;
_23 = -_8;
_12.fld2.0.0 = !_12.fld2.0.2;
_40 = Adt46::Variant0 { fld0: _38,fld1: _12.fld1 };
_17 = [(*_27),_39];
Goto(bb12)
}
bb12 = {
_18 = [_35.0,_35.0,_35.0,_35.0,_35.0,_35.0];
place!(Field::<(isize, f64)>(Variant(_40, 0), 1)).1 = _8 * _8;
_20 = _9;
_42.3.0.0 = _3 * Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0).0.0;
_42 = (_4, (*_27), _12.fld2.0.1, Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0), _38.0.0);
_12.fld2.2 = core::ptr::addr_of!(_42.3);
_43.0 = (_35.0,);
_15 = _37;
Call(_42.0 = core::intrinsics::bswap(_42.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0)).0.0 = _42.4 % _3;
_41 = [_42.4,_42.4,_42.4,Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0).0.0,_42.4,_3,Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0).0.0];
_36 = [_44.0,_35.0,_44.0,_44.0,_44.0,_44.0];
_15 = !RET;
_14 = _5 + Field::<(isize, f64)>(Variant(_40, 0), 1).0;
place!(Field::<(isize, f64)>(Variant(_40, 0), 1)).0 = _30;
_9 = _31;
_33 = _7 - _7;
_43.0 = (_35.0,);
_12.fld2.3 = [(*_27),(*_27),_4,(*_27),(*_27)];
_12.fld4 = (_12.fld2.0.1,);
_42.2 = _38.2 as usize;
_16 = -_1;
_34 = !_26;
place!(Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0)).0 = (_42.4,);
_43.0 = (_35.0,);
_45 = Field::<((u8,), u32, u32, char)>(Variant(_40, 0), 0).0.0 << _21.0;
_48.1 = _12.fld2.0.1 | _12.fld2.0.1;
_15 = _48.1 as u16;
SetDiscriminant(_40, 1);
_26 = !_34;
_42.1 = _42.0 ^ _12.fld3;
_12.fld4 = (_48.1,);
place!(Field::<((i64,),)>(Variant(_40, 1), 2)).0.0 = (*_27) as i64;
_34 = _12.fld4.0 < _12.fld4.0;
place!(Field::<((i64,),)>(Variant(_40, 1), 2)).0.0 = _43.0.0;
_50.0 = _11;
Goto(bb14)
}
bb14 = {
_46 = _16 as i32;
_32 = _12.fld2.2;
_50.2 = _21.2;
_12.fld3 = -_42.0;
_25 = _42.3.3;
_47 = _38.3;
(*_32).1 = !(*_32).2;
_25 = _38.3;
place!(Field::<u16>(Variant(_22, 1), 2)) = !_15;
_9 = [_4,_42.0,(*_27),_39,_42.1];
_26 = !_34;
_52.0 = _42.3.1 as isize;
_45 = (*_32).0.0 + _3;
_20 = [(*_27),_42.1,_42.1,_12.fld3,_4];
Goto(bb15)
}
bb15 = {
Call(_55 = dump_var(0_usize, 37_usize, Move(_37), 47_usize, Move(_47), 35_usize, Move(_35), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(0_usize, 17_usize, Move(_17), 46_usize, Move(_46), 11_usize, Move(_11), 41_usize, Move(_41)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(0_usize, 38_usize, Move(_38), 18_usize, Move(_18), 31_usize, Move(_31), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_55 = dump_var(0_usize, 21_usize, Move(_21), 44_usize, Move(_44), 30_usize, Move(_30), 43_usize, Move(_43)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_55 = dump_var(0_usize, 2_usize, Move(_2), 56_usize, _56, 56_usize, _56, 56_usize, _56), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: i128,mut _7: isize,mut _8: i8,mut _9: isize,mut _10: i32) -> u16 {
mir! {
type RET = u16;
let _11: isize;
let _12: i16;
let _13: f64;
let _14: [char; 5];
let _15: i128;
let _16: *const u64;
let _17: isize;
let _18: i64;
let _19: isize;
let _20: [char; 5];
let _21: *mut (u8,);
let _22: f64;
let _23: i128;
let _24: (u8,);
let _25: (isize, f64);
let _26: bool;
let _27: char;
let _28: Adt56;
let _29: *mut *mut i8;
let _30: Adt47;
let _31: i16;
let _32: bool;
let _33: ();
let _34: ();
{
_2 = _7 - _7;
_4 = 1_usize as isize;
_5 = _6 as isize;
_2 = _5;
_1 = _8 * _8;
_6 = 161585878481565396851709842989542626362_i128;
_8 = 9062746206865510823_usize as i8;
_9 = _7 | _2;
_4 = _5 * _5;
_4 = _9;
_1 = _8 << _9;
_9 = _4;
RET = !61340_u16;
_7 = 289414048848685073978042778540634884870_u128 as isize;
_9 = _2;
_5 = !_3;
_1 = _8 * _8;
_2 = !_3;
_10 = 1406116548_i32;
_2 = _4 ^ _4;
RET = 21150_u16 ^ 18658_u16;
_5 = _2 << _1;
_5 = _2;
_1 = _8 + _8;
_7 = !_4;
_8 = 457783695_u32 as i8;
_1 = 6843540833845214035_u64 as i8;
RET = 44476_u16 >> _2;
match _10 {
1406116548 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = _8;
_4 = _6 as isize;
_11 = !_2;
_6 = 11773723777300768168248573668677145119_i128 >> _5;
RET = 13186_u16;
_7 = _11 << _2;
_8 = 7246380026881233797_u64 as i8;
_4 = (-17248_i16) as isize;
_7 = _2 >> _4;
_3 = _5;
_10 = (-836232697_i32) - 587959541_i32;
_12 = 1926_i16 >> _11;
_5 = _2;
_9 = !_7;
_3 = _7;
_12 = _9 as i16;
_13 = _3 as f64;
_9 = !_2;
_7 = _11;
_8 = _1 - _1;
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = -_2;
_9 = _2;
RET = 1857_u16 | 30812_u16;
_5 = _3;
_1 = 64_u8 as i8;
_3 = _11;
_4 = _9;
RET = 20756_u16 - 23986_u16;
_15 = false as i128;
_13 = 14674298250275807624_usize as f64;
_9 = _4 >> _1;
RET = false as u16;
Call(_8 = fn2(_6, _9, _11, _6, _7, _9, _11, _11, _7, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = 1205_u16;
_5 = !_11;
_7 = _9 >> _10;
_5 = _2 & _11;
_7 = 120_u8 as isize;
_1 = _8 - _8;
_17 = 902615360672109213_i64 as isize;
_18 = 2681694207574212320_i64;
_15 = _6;
Call(_13 = fn3(_5, _1, _1, _5, _3, _10, _3, _2, _6, _9, _8, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = !(-6122240073598449353_i64);
_13 = 4940446147963917085_usize as f64;
_3 = _2 - _4;
_18 = !8206612474176863172_i64;
_22 = _13;
_20 = ['\u{8a37f}','\u{7767}','\u{12d1}','\u{e6cfc}','\u{68248}'];
_9 = 1743460792936573708_u64 as isize;
match RET {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
1205 => bb11,
_ => bb10
}
}
bb6 = {
RET = 1205_u16;
_5 = !_11;
_7 = _9 >> _10;
_5 = _2 & _11;
_7 = 120_u8 as isize;
_1 = _8 - _8;
_17 = 902615360672109213_i64 as isize;
_18 = 2681694207574212320_i64;
_15 = _6;
Call(_13 = fn3(_5, _1, _1, _5, _3, _10, _3, _2, _6, _9, _8, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_9 = -_2;
_9 = _2;
RET = 1857_u16 | 30812_u16;
_5 = _3;
_1 = 64_u8 as i8;
_3 = _11;
_4 = _9;
RET = 20756_u16 - 23986_u16;
_15 = false as i128;
_13 = 14674298250275807624_usize as f64;
_9 = _4 >> _1;
RET = false as u16;
Call(_8 = fn2(_6, _9, _11, _6, _7, _9, _11, _11, _7, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1 = _8;
_4 = _6 as isize;
_11 = !_2;
_6 = 11773723777300768168248573668677145119_i128 >> _5;
RET = 13186_u16;
_7 = _11 << _2;
_8 = 7246380026881233797_u64 as i8;
_4 = (-17248_i16) as isize;
_7 = _2 >> _4;
_3 = _5;
_10 = (-836232697_i32) - 587959541_i32;
_12 = 1926_i16 >> _11;
_5 = _2;
_9 = !_7;
_3 = _7;
_12 = _9 as i16;
_13 = _3 as f64;
_9 = !_2;
_7 = _11;
_8 = _1 - _1;
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
Call(_3 = fn5(_1, _1, _5, _5, _2, _11, _5, _8, _8, _12, _8, _8, _7, _1, _8), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = ['\u{102d0}','\u{93b06}','\u{1043ec}','\u{fe03a}','\u{7c44}'];
_17 = RET as isize;
RET = 49528_u16 + 4360_u16;
_9 = _12 as isize;
_2 = 18018556757374252273_u64 as isize;
_19 = true as isize;
_7 = -_3;
_15 = _6 ^ _6;
_24 = (202_u8,);
_25 = (_3, _22);
_15 = 1092826564_u32 as i128;
_23 = _6;
_12 = 26643_i16;
_13 = -_22;
_3 = -_7;
_10 = !(-290505314_i32);
_17 = RET as isize;
Goto(bb13)
}
bb13 = {
_12 = 4350418008704177148_u64 as i16;
_14 = _20;
_22 = 4998349362921620758_usize as f64;
_10 = _6 as i32;
_14 = ['\u{2a038}','\u{56d64}','\u{e3365}','\u{c1985}','\u{c19aa}'];
_27 = '\u{88e4e}';
_17 = _23 as isize;
_11 = _5;
_26 = false;
_5 = _25.0 >> _25.0;
_22 = -_25.1;
_17 = _1 as isize;
RET = !21897_u16;
_10 = -283183626_i32;
_9 = !_5;
_12 = 28235_i16;
_21 = core::ptr::addr_of_mut!(_24);
Goto(bb14)
}
bb14 = {
(*_21) = (7_u8,);
_12 = _1 as i16;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(1_usize, 11_usize, Move(_11), 18_usize, Move(_18), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(1_usize, 17_usize, Move(_17), 15_usize, Move(_15), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(1_usize, 26_usize, Move(_26), 19_usize, Move(_19), 24_usize, Move(_24), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: isize,mut _3: isize,mut _4: i128,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> i8 {
mir! {
type RET = i8;
let _11: u8;
let _12: bool;
let _13: ((u8,), u32, u32, char);
let _14: *mut (isize, f64);
let _15: (usize,);
let _16: [u8; 7];
let _17: ();
let _18: ();
{
_2 = _5;
_9 = _2;
RET = 674886174_i32 as i8;
_8 = -_3;
_10 = _8;
_10 = true as isize;
_8 = _3 | _2;
_8 = false as isize;
Goto(bb1)
}
bb1 = {
_2 = !_3;
_6 = -_3;
_3 = _6 + _2;
_8 = _9 * _7;
_13.1 = 2842963074_u32;
_3 = (-1540417100_i32) as isize;
_13.0 = (145_u8,);
_7 = _8 ^ _8;
_4 = -_1;
_9 = _2 + _7;
_11 = 6_usize as u8;
_13.0.0 = _11 & _11;
_15 = (4_usize,);
match _15.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb7,
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
_15.0 = 4_usize;
_13.3 = '\u{df537}';
RET = !126_i8;
_11 = RET as u8;
_2 = false as isize;
_1 = !_4;
_1 = 5188_u16 as i128;
_1 = !_4;
_13.2 = _13.1 ^ _13.1;
_8 = (-1661286885_i32) as isize;
_10 = _5;
_15.0 = 17735467495668038953_usize - 4_usize;
_12 = !true;
_13.2 = _12 as u32;
_8 = _6;
_13.0.0 = !_11;
_11 = _4 as u8;
_4 = _1 * _1;
_10 = _13.2 as isize;
_13.2 = _13.1 & _13.1;
_9 = _7;
_13.0.0 = _11 | _11;
_12 = _9 == _7;
_10 = _5 * _5;
_15.0 = (-31014539_i32) as usize;
_13.0 = (_11,);
_12 = !true;
_13.0.0 = _11;
match _13.1 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
2842963074 => bb14,
_ => bb13
}
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
Return()
}
bb13 = {
_2 = !_3;
_6 = -_3;
_3 = _6 + _2;
_8 = _9 * _7;
_13.1 = 2842963074_u32;
_3 = (-1540417100_i32) as isize;
_13.0 = (145_u8,);
_7 = _8 ^ _8;
_4 = -_1;
_9 = _2 + _7;
_11 = 6_usize as u8;
_13.0.0 = _11 & _11;
_15 = (4_usize,);
match _15.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb7,
_ => bb6
}
}
bb14 = {
RET = 84_i8 << _9;
_6 = _13.3 as isize;
_16 = [_13.0.0,_11,_11,_13.0.0,_11,_13.0.0,_11];
_13.0 = (_11,);
_15.0 = 13585766561756206465_u64 as usize;
_13.3 = '\u{4c086}';
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(2_usize, 5_usize, Move(_5), 11_usize, Move(_11), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(2_usize, 15_usize, Move(_15), 13_usize, Move(_13), 6_usize, Move(_6), 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: i8,mut _3: i8,mut _4: isize,mut _5: isize,mut _6: i32,mut _7: isize,mut _8: isize,mut _9: i128,mut _10: isize,mut _11: i8,mut _12: isize) -> f64 {
mir! {
type RET = f64;
let _13: [u8; 4];
let _14: f32;
let _15: i8;
let _16: ((u8,), u32, u32, char);
let _17: char;
let _18: ();
let _19: ();
{
_8 = _1 - _4;
_1 = _5 + _12;
_11 = _3 ^ _3;
_12 = _4;
_4 = 137_u8 as isize;
Call(_12 = fn4(_3, _1, _6, _8, _10, _8, _2, _8, _9, _8, _10, _3, _8, _2, _11, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = '\u{96664}' as isize;
_5 = 27878_i16 as isize;
_15 = !_11;
_10 = _12;
RET = _3 as f64;
_2 = _9 as i8;
_16.2 = !645466374_u32;
_11 = !_15;
_12 = !_10;
_9 = 139641872539543915632313655535104188181_i128;
_15 = _11;
_17 = '\u{d6b7}';
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(3_usize, 6_usize, Move(_6), 1_usize, Move(_1), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(3_usize, 3_usize, Move(_3), 17_usize, Move(_17), 12_usize, Move(_12), 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8,mut _2: isize,mut _3: i32,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: i8,mut _8: isize,mut _9: i128,mut _10: isize,mut _11: isize,mut _12: i8,mut _13: isize,mut _14: i8,mut _15: i8,mut _16: i8) -> isize {
mir! {
type RET = isize;
let _17: [u8; 4];
let _18: *mut (u8,);
let _19: Adt48;
let _20: isize;
let _21: ();
let _22: ();
{
_11 = _4;
_5 = _9 as isize;
RET = !_11;
_2 = _5;
_17 = [15_u8,214_u8,219_u8,20_u8];
_17 = [187_u8,243_u8,83_u8,114_u8];
RET = 6_usize as isize;
_12 = _14 << _11;
_8 = 1784038913880395221_i64 as isize;
_3 = 716174410_i32 & (-994756952_i32);
Goto(bb1)
}
bb1 = {
_12 = true as i8;
_16 = _1;
_3 = 1324311059_i32 | (-2018505862_i32);
Goto(bb2)
}
bb2 = {
_4 = _6 | _6;
_16 = _7;
_13 = 11471320886067889208_usize as isize;
_5 = 59959_u16 as isize;
_10 = -_4;
_7 = !_16;
RET = _10;
_14 = !_7;
_15 = _1 * _7;
_19.fld3 = _15 & _15;
_17 = [80_u8,203_u8,220_u8,189_u8];
_19.fld2.0 = (17833349571676949208_u64, 3016597436720462183_usize, 6133579614274791429_u64);
_11 = 47149_u16 as isize;
RET = _4;
_19.fld0 = [1_u8,136_u8,148_u8,186_u8,136_u8,77_u8,41_u8];
_19.fld1.1 = _19.fld2.0.1 as f64;
_20 = -RET;
_19.fld1.0 = _1 as isize;
Goto(bb3)
}
bb3 = {
Call(_21 = dump_var(4_usize, 14_usize, Move(_14), 10_usize, Move(_10), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_21 = dump_var(4_usize, 5_usize, Move(_5), 12_usize, Move(_12), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_21 = dump_var(4_usize, 16_usize, Move(_16), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i8,mut _2: i8,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: i8,mut _9: i8,mut _10: i16,mut _11: i8,mut _12: i8,mut _13: isize,mut _14: i8,mut _15: i8) -> isize {
mir! {
type RET = isize;
let _16: bool;
let _17: Adt46;
let _18: f64;
let _19: isize;
let _20: isize;
let _21: Adt60;
let _22: isize;
let _23: u128;
let _24: ();
let _25: ();
{
_6 = _4;
_12 = _10 as i8;
_3 = _15 as isize;
_8 = _1 ^ _15;
RET = -_5;
Goto(bb1)
}
bb1 = {
_3 = _6;
_7 = _3 | _4;
_5 = _4;
_10 = -(-3143_i16);
_14 = !_8;
_7 = 92001490749349148962817534324464564504_i128 as isize;
_19 = _6;
_18 = _10 as f64;
_7 = _4 | _6;
_20 = _7;
_6 = _3 ^ _5;
_7 = _13 >> _5;
RET = 58_u8 as isize;
_20 = _5 - _5;
_10 = 8427_i16;
_18 = _3 as f64;
_20 = true as isize;
_11 = (-5544345599130089809_i64) as i8;
_8 = (-134760793417947087721298198122322047191_i128) as i8;
_15 = !_1;
_20 = -_19;
RET = _6 + _4;
_12 = _15;
_4 = _20 * RET;
Goto(bb2)
}
bb2 = {
Call(_24 = dump_var(5_usize, 19_usize, Move(_19), 4_usize, Move(_4), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_24 = dump_var(5_usize, 12_usize, Move(_12), 3_usize, Move(_3), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: char,mut _2: char,mut _3: i32,mut _4: u8) -> i128 {
mir! {
type RET = i128;
let _5: (u8,);
let _6: char;
let _7: Adt58;
let _8: f32;
let _9: isize;
let _10: [u8; 7];
let _11: f64;
let _12: *const u16;
let _13: *const [u8; 4];
let _14: f64;
let _15: [u32; 2];
let _16: (u8,);
let _17: i16;
let _18: f32;
let _19: bool;
let _20: i64;
let _21: i32;
let _22: bool;
let _23: *const u16;
let _24: ();
let _25: ();
{
RET = 133722666217358088854828218144877057432_i128 * 140949757511971438025588864308725243408_i128;
_5.0 = _4 >> RET;
_2 = _1;
_5 = (_4,);
RET = 151596693_u32 as i128;
_3 = -(-137012415_i32);
_4 = !_5.0;
_5.0 = _4;
_5 = (_4,);
Goto(bb1)
}
bb1 = {
_5.0 = !_4;
RET = (-125370433406504784651506657909997597694_i128);
_5 = (_4,);
_3 = 1460149896_i32 - (-2126579267_i32);
_3 = -(-1950576282_i32);
_4 = !_5.0;
RET = (-140246652329227357466313191482931853573_i128) - 34153659971286894581381779761754927677_i128;
_4 = _5.0;
_5 = (_4,);
_3 = !818187559_i32;
_3 = (-122_i8) as i32;
_3 = -(-976705809_i32);
Goto(bb2)
}
bb2 = {
RET = (-78661026491883450462893995559846569484_i128) + (-92615913983938001332353605145683588760_i128);
_5.0 = _4 & _4;
_5 = (_4,);
Call(_5 = fn7(_2, _2, _2, _3, _3, _3, _2, _1, RET, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _2;
_5 = (_4,);
_1 = _6;
_7.fld2.1 = !11653367229275971096_usize;
_7.fld1 = [2427295711_u32,3719867169_u32];
_4 = _7.fld2.1 as u8;
_7.fld2.2 = 11842591487260535962_u64;
_2 = _1;
_2 = _1;
_5 = (_4,);
_7.fld2.0 = !_7.fld2.2;
_2 = _1;
_7.fld1 = [1335284698_u32,2821616386_u32];
_7.fld2.2 = _7.fld2.0 * _7.fld2.0;
RET = false as i128;
_4 = _5.0;
_2 = _1;
RET = !111406420562532957215511383531039938515_i128;
_7.fld1 = [335181559_u32,1989701555_u32];
_3 = 276277842_i32 | 1695730288_i32;
Goto(bb4)
}
bb4 = {
_5.0 = _4;
_7.fld2 = (16986504433126131020_u64, 2_usize, 9908835150526528379_u64);
_7.fld2.2 = _7.fld2.0 | _7.fld2.0;
_5.0 = !_4;
_7.fld1 = [3926393762_u32,3666673832_u32];
RET = -77028157725395565618663734926209703495_i128;
_5.0 = !_4;
_6 = _2;
_10 = [_4,_4,_4,_5.0,_4,_4,_5.0];
_5.0 = !_4;
_5 = (_4,);
_8 = (-10603_i16) as f32;
RET = (-151742972932584671347174388652695855726_i128);
_1 = _6;
RET = _6 as i128;
_11 = 202912134_u32 as f64;
_3 = 6932_u16 as i32;
_8 = _4 as f32;
_4 = 84_isize as u8;
_9 = true as isize;
RET = (-19506308657110046512959515457234924758_i128) >> _7.fld2.0;
_11 = 17047_u16 as f64;
_7.fld2.2 = _7.fld2.0;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_6 = _1;
Goto(bb5)
}
bb5 = {
_2 = _1;
_5.0 = _4 + _4;
_6 = _1;
_14 = _11 - _11;
_7.fld2.1 = !0_usize;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_9 = (-81_isize);
_10 = [_4,_5.0,_5.0,_5.0,_4,_4,_4];
_7.fld2.2 = _7.fld2.0;
_5 = (_4,);
_9 = _2 as isize;
RET = 93919111670438480221254685357194142932_i128 >> _7.fld2.1;
_1 = _2;
Goto(bb6)
}
bb6 = {
_3 = RET as i32;
_4 = _5.0 >> _7.fld2.0;
_8 = _14 as f32;
_15 = [1956370633_u32,3478067167_u32];
RET = (-20_i8) as i128;
match _7.fld2.0 {
0 => bb1,
1 => bb5,
2 => bb3,
16986504433126131020 => bb8,
_ => bb7
}
}
bb7 = {
_2 = _1;
_5.0 = _4 + _4;
_6 = _1;
_14 = _11 - _11;
_7.fld2.1 = !0_usize;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_9 = (-81_isize);
_10 = [_4,_5.0,_5.0,_5.0,_4,_4,_4];
_7.fld2.2 = _7.fld2.0;
_5 = (_4,);
_9 = _2 as isize;
RET = 93919111670438480221254685357194142932_i128 >> _7.fld2.1;
_1 = _2;
Goto(bb6)
}
bb8 = {
RET = (-121795737704027747574823998481971804659_i128);
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_3 = 1395932733_i32;
_3 = (-817491308_i32);
_8 = (-88_i8) as f32;
Goto(bb9)
}
bb9 = {
_16.0 = _4 << _9;
_8 = 66_i8 as f32;
_17 = 33308_u16 as i16;
RET = -5362847308260490147823924389853840900_i128;
_5.0 = _4 - _16.0;
_15 = [3458052840_u32,1436065354_u32];
_8 = RET as f32;
_10 = [_5.0,_4,_5.0,_5.0,_4,_5.0,_5.0];
_19 = !true;
_15 = [1616782416_u32,981312948_u32];
_5 = _16;
_11 = 55_i8 as f64;
match _7.fld2.0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
16986504433126131020 => bb18,
_ => bb17
}
}
bb10 = {
RET = (-121795737704027747574823998481971804659_i128);
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_3 = 1395932733_i32;
_3 = (-817491308_i32);
_8 = (-88_i8) as f32;
Goto(bb9)
}
bb11 = {
_2 = _1;
_5.0 = _4 + _4;
_6 = _1;
_14 = _11 - _11;
_7.fld2.1 = !0_usize;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_9 = (-81_isize);
_10 = [_4,_5.0,_5.0,_5.0,_4,_4,_4];
_7.fld2.2 = _7.fld2.0;
_5 = (_4,);
_9 = _2 as isize;
RET = 93919111670438480221254685357194142932_i128 >> _7.fld2.1;
_1 = _2;
Goto(bb6)
}
bb12 = {
_3 = RET as i32;
_4 = _5.0 >> _7.fld2.0;
_8 = _14 as f32;
_15 = [1956370633_u32,3478067167_u32];
RET = (-20_i8) as i128;
match _7.fld2.0 {
0 => bb1,
1 => bb5,
2 => bb3,
16986504433126131020 => bb8,
_ => bb7
}
}
bb13 = {
_2 = _1;
_5.0 = _4 + _4;
_6 = _1;
_14 = _11 - _11;
_7.fld2.1 = !0_usize;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_9 = (-81_isize);
_10 = [_4,_5.0,_5.0,_5.0,_4,_4,_4];
_7.fld2.2 = _7.fld2.0;
_5 = (_4,);
_9 = _2 as isize;
RET = 93919111670438480221254685357194142932_i128 >> _7.fld2.1;
_1 = _2;
Goto(bb6)
}
bb14 = {
_5.0 = _4;
_7.fld2 = (16986504433126131020_u64, 2_usize, 9908835150526528379_u64);
_7.fld2.2 = _7.fld2.0 | _7.fld2.0;
_5.0 = !_4;
_7.fld1 = [3926393762_u32,3666673832_u32];
RET = -77028157725395565618663734926209703495_i128;
_5.0 = !_4;
_6 = _2;
_10 = [_4,_4,_4,_5.0,_4,_4,_5.0];
_5.0 = !_4;
_5 = (_4,);
_8 = (-10603_i16) as f32;
RET = (-151742972932584671347174388652695855726_i128);
_1 = _6;
RET = _6 as i128;
_11 = 202912134_u32 as f64;
_3 = 6932_u16 as i32;
_8 = _4 as f32;
_4 = 84_isize as u8;
_9 = true as isize;
RET = (-19506308657110046512959515457234924758_i128) >> _7.fld2.0;
_11 = 17047_u16 as f64;
_7.fld2.2 = _7.fld2.0;
_7.fld2.2 = _7.fld2.0 ^ _7.fld2.0;
_6 = _1;
Goto(bb5)
}
bb15 = {
_6 = _2;
_5 = (_4,);
_1 = _6;
_7.fld2.1 = !11653367229275971096_usize;
_7.fld1 = [2427295711_u32,3719867169_u32];
_4 = _7.fld2.1 as u8;
_7.fld2.2 = 11842591487260535962_u64;
_2 = _1;
_2 = _1;
_5 = (_4,);
_7.fld2.0 = !_7.fld2.2;
_2 = _1;
_7.fld1 = [1335284698_u32,2821616386_u32];
_7.fld2.2 = _7.fld2.0 * _7.fld2.0;
RET = false as i128;
_4 = _5.0;
_2 = _1;
RET = !111406420562532957215511383531039938515_i128;
_7.fld1 = [335181559_u32,1989701555_u32];
_3 = 276277842_i32 | 1695730288_i32;
Goto(bb4)
}
bb16 = {
RET = (-78661026491883450462893995559846569484_i128) + (-92615913983938001332353605145683588760_i128);
_5.0 = _4 & _4;
_5 = (_4,);
Call(_5 = fn7(_2, _2, _2, _3, _3, _3, _2, _1, RET, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_5.0 = !_4;
RET = (-125370433406504784651506657909997597694_i128);
_5 = (_4,);
_3 = 1460149896_i32 - (-2126579267_i32);
_3 = -(-1950576282_i32);
_4 = !_5.0;
RET = (-140246652329227357466313191482931853573_i128) - 34153659971286894581381779761754927677_i128;
_4 = _5.0;
_5 = (_4,);
_3 = !818187559_i32;
_3 = (-122_i8) as i32;
_3 = -(-976705809_i32);
Goto(bb2)
}
bb18 = {
_22 = !_19;
_8 = _14 as f32;
_7.fld2.1 = !5561667751875138662_usize;
_19 = _5.0 > _4;
_20 = 8342576725390362823_i64;
_10 = [_4,_16.0,_5.0,_4,_5.0,_16.0,_16.0];
_17 = _9 as i16;
_1 = _6;
_1 = _2;
_5 = _16;
Goto(bb19)
}
bb19 = {
Call(_24 = dump_var(6_usize, 2_usize, Move(_2), 17_usize, Move(_17), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_24 = dump_var(6_usize, 3_usize, Move(_3), 19_usize, Move(_19), 15_usize, Move(_15), 25_usize, _25), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: char,mut _2: char,mut _3: char,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: char,mut _8: char,mut _9: i128,mut _10: char,mut _11: u8) -> (u8,) {
mir! {
type RET = (u8,);
let _12: i8;
let _13: [char; 5];
let _14: u64;
let _15: i128;
let _16: (i64,);
let _17: usize;
let _18: [char; 5];
let _19: (u64, usize, u64);
let _20: char;
let _21: u128;
let _22: (u8,);
let _23: isize;
let _24: (u64, usize, u64);
let _25: i64;
let _26: Adt62;
let _27: [i128; 3];
let _28: [char; 5];
let _29: usize;
let _30: Adt60;
let _31: Adt46;
let _32: Adt59;
let _33: (isize, f64);
let _34: usize;
let _35: Adt55;
let _36: char;
let _37: isize;
let _38: [i64; 6];
let _39: *mut (isize, f64);
let _40: Adt57;
let _41: (i8, i8, usize, ((u8,), u32, u32, char), u8);
let _42: f32;
let _43: [u8; 4];
let _44: [i64; 6];
let _45: [i64; 6];
let _46: *mut i8;
let _47: bool;
let _48: Adt49;
let _49: ();
let _50: ();
{
_5 = 19476_u16 as i32;
_9 = -140462799191867492904121397116831833655_i128;
_4 = true as i32;
_7 = _1;
_3 = _7;
_1 = _2;
_6 = -_4;
_5 = _4 >> _6;
_3 = _7;
RET.0 = !_11;
_5 = _8 as i32;
RET.0 = _11 & _11;
RET.0 = _7 as u8;
_9 = 2118345366_u32 as i128;
_7 = _8;
RET = (_11,);
_5 = -_4;
_8 = _3;
_8 = _10;
_13 = [_8,_10,_10,_8,_2];
RET.0 = _11;
_10 = _8;
_10 = _3;
Call(_8 = fn8(_10, _5, _1, _1, _2, _10, _10, _1, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _4;
Goto(bb2)
}
bb2 = {
_7 = _8;
_12 = -4_i8;
_12 = _4 as i8;
_1 = _2;
RET.0 = _11 * _11;
_10 = _8;
_1 = _7;
_4 = !_5;
_1 = _7;
_8 = _3;
_12 = !(-128_i8);
_10 = _1;
_12 = !115_i8;
_13 = [_2,_3,_10,_2,_10];
RET = (_11,);
_8 = _2;
_2 = _8;
_13 = [_7,_10,_1,_8,_2];
_13 = [_3,_7,_3,_2,_2];
Goto(bb3)
}
bb3 = {
RET.0 = !_11;
_3 = _10;
_13 = [_2,_1,_3,_7,_8];
_10 = _1;
RET = (_11,);
_15 = _9;
_15 = _9;
Goto(bb4)
}
bb4 = {
_3 = _1;
RET = (_11,);
_19.2 = !11587485605217422746_u64;
Goto(bb5)
}
bb5 = {
_1 = _2;
Goto(bb6)
}
bb6 = {
_22.0 = !RET.0;
_13 = [_1,_1,_1,_7,_2];
_20 = _3;
_21 = _6 as u128;
_12 = (-35_isize) as i8;
_17 = !7_usize;
_14 = _19.2;
_12 = _15 as i8;
_19 = (_14, _17, _14);
_17 = _19.2 as usize;
_9 = _15 - _15;
_18 = [_8,_1,_3,_2,_8];
Goto(bb7)
}
bb7 = {
_6 = -_5;
RET = (_11,);
_16.0 = 5945330282580332279_i64 - 679927052038631758_i64;
_24.2 = _19.2;
_19.2 = _24.2;
_25 = _16.0;
_3 = _1;
_2 = _8;
_19 = (_14, _17, _14);
_29 = 15_isize as usize;
RET = (_22.0,);
_25 = !_16.0;
_25 = !_16.0;
_16 = (_25,);
_24.2 = _17 as u64;
_23 = 78_isize | 57_isize;
Goto(bb8)
}
bb8 = {
_19.1 = _29;
_24 = (_19.0, _17, _19.2);
_19 = (_14, _24.1, _14);
_10 = _8;
Call(_19.1 = fn10(RET.0, _6, _23, _20, _18, _25, _12, _2, _20, _20, _8, _20, _25, _21, _16, _25), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9 = _15;
_17 = _29;
RET = (_22.0,);
_12 = 89_i8;
_18 = [_1,_7,_7,_7,_1];
_19.1 = _17;
_4 = _6;
_16.0 = _25 << _29;
_7 = _2;
_19.0 = _6 as u64;
_22.0 = RET.0 ^ RET.0;
_33.0 = -_23;
_23 = _33.0 | _33.0;
_28 = _13;
_32.fld0 = core::ptr::addr_of_mut!(_33);
_9 = -_15;
_13 = _28;
_19.0 = _19.2;
_25 = _16.0;
_24 = (_19.2, _17, _19.2);
_16.0 = _25;
_15 = -_9;
_3 = _20;
Goto(bb10)
}
bb10 = {
_23 = _21 as isize;
_33.1 = _12 as f64;
_21 = 254902302131235695806199036918405615201_u128;
_14 = _19.2;
_5 = _29 as i32;
_29 = _19.1 - _24.1;
_22 = (RET.0,);
_33.0 = !_23;
_8 = _2;
_14 = _1 as u64;
_32.fld0 = core::ptr::addr_of_mut!(_33);
_17 = _24.1;
_27 = [_15,_9,_9];
_24.1 = !_17;
_11 = _22.0;
_32.fld0 = core::ptr::addr_of_mut!(_33);
_19.1 = !_24.1;
_7 = _10;
_7 = _20;
_24 = (_19.0, _17, _19.2);
_11 = !_22.0;
Goto(bb11)
}
bb11 = {
_3 = _1;
_22.0 = _11;
_41.3.0 = (RET.0,);
_41.4 = _33.1 as u8;
_41.4 = _41.3.0.0;
_24 = _19;
_27 = [_9,_15,_15];
_11 = _41.4;
_23 = _33.0;
_19.2 = _24.2 >> _16.0;
_41.3.3 = _1;
_22.0 = _7 as u8;
Goto(bb12)
}
bb12 = {
_41.3.1 = 757448634_u32 + 2237225078_u32;
match _21 {
0 => bb8,
1 => bb10,
254902302131235695806199036918405615201 => bb14,
_ => bb13
}
}
bb13 = {
_22.0 = !RET.0;
_13 = [_1,_1,_1,_7,_2];
_20 = _3;
_21 = _6 as u128;
_12 = (-35_isize) as i8;
_17 = !7_usize;
_14 = _19.2;
_12 = _15 as i8;
_19 = (_14, _17, _14);
_17 = _19.2 as usize;
_9 = _15 - _15;
_18 = [_8,_1,_3,_2,_8];
Goto(bb7)
}
bb14 = {
_9 = _41.3.1 as i128;
_45 = [_25,_25,_25,_25,_16.0,_16.0];
_41.3.1 = !1995083054_u32;
_19.0 = _19.2;
_27 = [_9,_9,_9];
_5 = -_4;
_41.3.0 = (_11,);
_41.3 = (RET, 3915700982_u32, 1004001528_u32, _3);
_44 = _45;
_46 = core::ptr::addr_of_mut!(_41.1);
_19.2 = _24.0 | _14;
_39 = _32.fld0;
_34 = _21 as usize;
_44 = [_25,_25,_25,_25,_16.0,_16.0];
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(7_usize, 6_usize, Move(_6), 10_usize, Move(_10), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(7_usize, 28_usize, Move(_28), 5_usize, Move(_5), 19_usize, Move(_19), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(7_usize, 23_usize, Move(_23), 29_usize, Move(_29), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(7_usize, 18_usize, Move(_18), 2_usize, Move(_2), 24_usize, Move(_24), 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: char,mut _2: i32,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: (u8,)) -> char {
mir! {
type RET = char;
let _10: [u8; 7];
let _11: Adt51;
let _12: u128;
let _13: isize;
let _14: u16;
let _15: ((u8,), u32, u32, char);
let _16: i16;
let _17: usize;
let _18: (u8,);
let _19: char;
let _20: i64;
let _21: (isize, f64);
let _22: [i8; 2];
let _23: char;
let _24: isize;
let _25: f32;
let _26: Adt57;
let _27: i8;
let _28: i8;
let _29: isize;
let _30: [u8; 4];
let _31: ();
let _32: ();
{
_4 = _5;
_8 = _4;
_1 = _3;
RET = _1;
_9.0 = (-75_i8) as u8;
_10 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_2 = !753361028_i32;
_10 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_3 = _1;
_9.0 = !173_u8;
_5 = _3;
_12 = 203898608781522276791270667541822254236_u128 & 125569026119039201412793114638778151794_u128;
_7 = _5;
RET = _3;
_4 = RET;
_1 = _6;
_9 = (9_u8,);
_12 = 9223372036854775807_isize as u128;
_7 = _4;
_2 = -(-1082888764_i32);
_4 = _6;
_9 = (102_u8,);
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
102 => bb8,
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
_14 = !63292_u16;
_5 = _3;
_3 = _1;
_7 = _3;
RET = _8;
_15.2 = !3181582236_u32;
_15.0 = _9;
_8 = _6;
RET = _8;
_16 = 18687_i16 << _12;
_15.1 = 5_usize as u32;
_4 = RET;
RET = _1;
_15.0.0 = _9.0 % _9.0;
_8 = _7;
RET = _6;
RET = _4;
_7 = RET;
Call(_18 = fn9(_7, _5, _12, _9, _1, _14, _1, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = _15.0;
_5 = _1;
_15.1 = _15.2 >> _15.0.0;
_17 = _15.1 as usize;
_18.0 = _15.0.0;
_2 = 1990401315_i32;
_15.0.0 = _18.0;
_3 = _5;
_15.3 = _4;
_15.0.0 = _18.0;
_15.1 = !_15.2;
_6 = _1;
_15.1 = 54325756011895665982635065319286910568_i128 as u32;
_15.0.0 = _9.0;
Goto(bb10)
}
bb10 = {
_16 = !28152_i16;
_5 = _8;
_3 = _6;
_5 = _7;
_17 = 3203404016841584832_usize;
_9 = (_18.0,);
_2 = _9.0 as i32;
match _15.0.0 {
0 => bb5,
1 => bb7,
2 => bb11,
102 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_5 = _3;
RET = _5;
_16 = !(-13898_i16);
_19 = _3;
RET = _3;
_6 = _7;
_22 = [24_i8,(-71_i8)];
_7 = _4;
_10 = [_9.0,_9.0,_18.0,_15.0.0,_9.0,_9.0,_18.0];
_25 = _16 as f32;
_18.0 = _9.0 * _9.0;
_12 = _15.2 as u128;
_24 = (-9223372036854775808_isize);
_5 = _8;
_27 = true as i8;
_26 = Adt57::Variant2 { fld0: _17,fld1: 8476139285244482048_u64 };
_18 = (_15.0.0,);
Goto(bb14)
}
bb14 = {
_19 = _8;
_15.0.0 = _2 as u8;
_17 = _15.2 as usize;
_12 = !197650069402281881787920168346361696678_u128;
_10 = [_9.0,_18.0,_9.0,_9.0,_15.0.0,_18.0,_15.0.0];
_1 = RET;
_7 = _6;
_15.2 = _15.1 | _15.1;
_16 = (-23660_i16);
_21.1 = _2 as f64;
_28 = !_27;
_10 = [_15.0.0,_18.0,_18.0,_18.0,_15.0.0,_9.0,_18.0];
_29 = _19 as isize;
_15.3 = _4;
_30 = [_15.0.0,_15.0.0,_15.0.0,_9.0];
_25 = _27 as f32;
_9 = (_15.0.0,);
_18 = (_9.0,);
_15.1 = !_15.2;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(8_usize, 22_usize, Move(_22), 14_usize, Move(_14), 4_usize, Move(_4), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(8_usize, 7_usize, Move(_7), 19_usize, Move(_19), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(8_usize, 29_usize, Move(_29), 3_usize, Move(_3), 12_usize, Move(_12), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: char,mut _2: char,mut _3: u128,mut _4: (u8,),mut _5: char,mut _6: u16,mut _7: char,mut _8: char) -> (u8,) {
mir! {
type RET = (u8,);
let _9: isize;
let _10: f32;
let _11: [u8; 7];
let _12: Adt55;
let _13: f64;
let _14: Adt54;
let _15: isize;
let _16: isize;
let _17: (i64,);
let _18: ();
let _19: ();
{
RET = _4;
_1 = _8;
_4.0 = RET.0;
_4 = (RET.0,);
_4 = (RET.0,);
_1 = _7;
_9 = _2 as isize;
RET.0 = _4.0 & _4.0;
_7 = _8;
_6 = 5534439035835831923_u64 as u16;
RET.0 = _4.0;
RET = (_4.0,);
RET.0 = false as u8;
_3 = 187944294017653530920530253267407116189_u128;
_5 = _7;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
187944294017653530920530253267407116189 => bb7,
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
_5 = _8;
_8 = _2;
_1 = _8;
RET = (_4.0,);
RET = (_4.0,);
RET.0 = _4.0 | _4.0;
_4 = (RET.0,);
_5 = _2;
_9 = !9223372036854775807_isize;
_4.0 = 7010313910898538417_i64 as u8;
RET.0 = _4.0 + _4.0;
RET = (_4.0,);
_3 = 27_i8 as u128;
Goto(bb8)
}
bb8 = {
_9 = (-9223372036854775808_isize);
_5 = _8;
RET = (_4.0,);
RET.0 = !_4.0;
_10 = (-838210519_i32) as f32;
_4.0 = !RET.0;
_1 = _5;
RET = (_4.0,);
RET.0 = _4.0;
_11 = [_4.0,_4.0,_4.0,_4.0,RET.0,_4.0,_4.0];
_2 = _8;
RET.0 = _4.0;
match _9 {
0 => bb6,
1 => bb7,
2 => bb9,
340282366920938463454151235394913435648 => bb11,
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
_11 = [RET.0,_4.0,_4.0,RET.0,_4.0,RET.0,_4.0];
_10 = 839458754_u32 as f32;
RET = (_4.0,);
Goto(bb12)
}
bb12 = {
_3 = _5 as u128;
_6 = _1 as u16;
_10 = 1052505724_i32 as f32;
RET.0 = _4.0;
match _9 {
0 => bb13,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = !189522386429279687157098076812792176849_u128;
RET.0 = !_4.0;
_6 = 9550_u16 | 40910_u16;
_4 = RET;
_13 = 4105693867887553816_i64 as f64;
_8 = _5;
RET.0 = !_4.0;
RET.0 = !_4.0;
_2 = _7;
_1 = _2;
_7 = _1;
Goto(bb16)
}
bb16 = {
Call(_18 = dump_var(9_usize, 5_usize, Move(_5), 8_usize, Move(_8), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_18 = dump_var(9_usize, 1_usize, Move(_1), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u8,mut _2: i32,mut _3: isize,mut _4: char,mut _5: [char; 5],mut _6: i64,mut _7: i8,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: char,mut _13: i64,mut _14: u128,mut _15: (i64,),mut _16: i64) -> usize {
mir! {
type RET = usize;
let _17: isize;
let _18: isize;
let _19: ((i64,),);
let _20: [i8; 5];
let _21: (i64,);
let _22: u32;
let _23: bool;
let _24: u64;
let _25: isize;
let _26: i128;
let _27: f64;
let _28: [i8; 2];
let _29: ((i64,),);
let _30: f32;
let _31: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128));
let _32: Adt56;
let _33: ([i64; 6], usize, *mut u128);
let _34: (usize,);
let _35: usize;
let _36: Adt56;
let _37: u8;
let _38: (u8,);
let _39: u64;
let _40: ((i64,),);
let _41: f32;
let _42: ();
let _43: ();
{
_7 = (-68_i8);
_15 = (_6,);
_2 = (-2011956675_i32) & (-2146481146_i32);
_12 = _8;
_12 = _11;
_15 = (_16,);
_5 = [_10,_12,_4,_9,_4];
_4 = _11;
_19.0 = _15;
_3 = true as isize;
RET = 12749100366268567989_u64 as usize;
_14 = 13395451389280039708_u64 as u128;
RET = 7_usize << _19.0.0;
_20 = [_7,_7,_7,_7,_7];
_8 = _11;
_1 = 105_u8;
RET = 18550_u16 as usize;
_9 = _8;
Goto(bb1)
}
bb1 = {
_18 = 10849971030928333268_u64 as isize;
_7 = 8_i8;
_16 = _10 as i64;
_21 = (_13,);
_9 = _8;
_19.0 = _21;
_10 = _4;
_9 = _10;
_18 = 30708_u16 as isize;
_21.0 = _1 as i64;
_12 = _4;
_1 = 36_u8 + 137_u8;
_18 = _3 * _3;
_14 = !17051360092789611087781208546066841518_u128;
Goto(bb2)
}
bb2 = {
_13 = _7 as i64;
_1 = !85_u8;
_9 = _8;
_21 = (_15.0,);
_13 = _16 & _19.0.0;
_17 = _3;
_23 = true;
RET = 2_usize & 5590960298671870859_usize;
Call(_7 = fn11(_6, _17, _15, _19.0.0, _18, _15, _19, _3, _15.0, _15.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _4;
_12 = _4;
_23 = _18 <= _18;
_14 = _11 as u128;
RET = !11459590105517887552_usize;
_20 = [_7,_7,_7,_7,_7];
_19.0.0 = _13;
_8 = _4;
Goto(bb4)
}
bb4 = {
_8 = _10;
_24 = !15778382349678255942_u64;
_21 = _19.0;
_2 = (-1573499516_i32) - (-692162156_i32);
_19 = (_21,);
_25 = _19.0.0 as isize;
Goto(bb5)
}
bb5 = {
_26 = 1476695138927610165886311407552896704_i128;
_18 = _1 as isize;
_25 = !_3;
_16 = _24 as i64;
_19.0 = (_15.0,);
RET = _1 as usize;
_22 = _2 as u32;
RET = 1_usize;
_19.0 = (_16,);
_9 = _5[RET];
_23 = true;
RET = _24 as usize;
_27 = 21333_u16 as f64;
_25 = -_18;
_22 = !2064970212_u32;
_16 = -_6;
_22 = _17 as u32;
_18 = RET as isize;
RET = 1_usize - 8192367517162762155_usize;
_14 = _23 as u128;
Goto(bb6)
}
bb6 = {
_13 = _6;
_19.0.0 = _15.0 - _13;
RET = !10205955571067138882_usize;
_13 = _19.0.0;
_20 = [_7,_7,_7,_7,_7];
_12 = _10;
RET = _24 as usize;
_19.0.0 = _13 * _16;
_19.0 = (_16,);
_28 = [_7,_7];
_31.2.1 = _3 as usize;
Call(_7 = core::intrinsics::bswap(81_i8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = (-820991030_i32);
_31.2.0 = [_13,_13,_21.0,_13,_15.0,_13];
_8 = _4;
_31.2.0 = [_16,_15.0,_13,_6,_13,_13];
_33.2 = core::ptr::addr_of_mut!(_14);
_23 = !false;
RET = _23 as usize;
_33.2 = core::ptr::addr_of_mut!(_14);
_27 = _2 as f64;
_9 = _11;
_30 = _1 as f32;
_34.0 = _17 as usize;
_31.2.2 = _33.2;
_25 = _17 | _17;
Goto(bb8)
}
bb8 = {
_30 = _34.0 as f32;
_34.0 = RET & RET;
_18 = 8666_i16 as isize;
_31.1 = [_1,_1,_1,_1,_1,_1,_1];
_15 = (_16,);
_26 = 101254435747213379587080153103705447703_i128 << _25;
_29 = (_15,);
_29.0 = (_21.0,);
_7 = (-107_i8) ^ (-29_i8);
_27 = _24 as f64;
_4 = _10;
_38.0 = _1;
_15 = (_6,);
match _2 {
0 => bb2,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463463374607430947220426 => bb13,
_ => bb12
}
}
bb9 = {
_13 = _7 as i64;
_1 = !85_u8;
_9 = _8;
_21 = (_15.0,);
_13 = _16 & _19.0.0;
_17 = _3;
_23 = true;
RET = 2_usize & 5590960298671870859_usize;
Call(_7 = fn11(_6, _17, _15, _19.0.0, _18, _15, _19, _3, _15.0, _15.0), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_13 = _6;
_19.0.0 = _15.0 - _13;
RET = !10205955571067138882_usize;
_13 = _19.0.0;
_20 = [_7,_7,_7,_7,_7];
_12 = _10;
RET = _24 as usize;
_19.0.0 = _13 * _16;
_19.0 = (_16,);
_28 = [_7,_7];
_31.2.1 = _3 as usize;
Call(_7 = core::intrinsics::bswap(81_i8), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_11 = _4;
_12 = _4;
_23 = _18 <= _18;
_14 = _11 as u128;
RET = !11459590105517887552_usize;
_20 = [_7,_7,_7,_7,_7];
_19.0.0 = _13;
_8 = _4;
Goto(bb4)
}
bb12 = {
_8 = _10;
_24 = !15778382349678255942_u64;
_21 = _19.0;
_2 = (-1573499516_i32) - (-692162156_i32);
_19 = (_21,);
_25 = _19.0.0 as isize;
Goto(bb5)
}
bb13 = {
_17 = _3;
_38.0 = _25 as u8;
_40 = (_15,);
_34.0 = _31.2.1;
_10 = _12;
_16 = _19.0.0;
_15.0 = !_6;
_25 = _3 - _17;
_22 = _2 as u32;
_16 = 10002_i16 as i64;
_19.0.0 = _2 as i64;
_31.2.2 = core::ptr::addr_of_mut!(_14);
_21 = (_6,);
_2 = 599430132_i32;
_33 = (_31.2.0, _31.2.1, _31.2.2);
_30 = _1 as f32;
_31.0 = [_7,_7];
_40.0 = (_15.0,);
_2 = (-1953692661_i32) | (-704969206_i32);
_29 = (_21,);
Goto(bb14)
}
bb14 = {
_10 = _8;
_30 = _38.0 as f32;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(10_usize, 34_usize, Move(_34), 25_usize, Move(_25), 28_usize, Move(_28), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(10_usize, 7_usize, Move(_7), 5_usize, Move(_5), 23_usize, Move(_23), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(10_usize, 14_usize, Move(_14), 18_usize, Move(_18), 40_usize, Move(_40), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(10_usize, 19_usize, Move(_19), 22_usize, Move(_22), 15_usize, Move(_15), 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i64,mut _2: isize,mut _3: (i64,),mut _4: i64,mut _5: isize,mut _6: (i64,),mut _7: ((i64,),),mut _8: isize,mut _9: i64,mut _10: i64) -> i8 {
mir! {
type RET = i8;
let _11: *const [u8; 4];
let _12: [u8; 7];
let _13: (usize,);
let _14: [i64; 6];
let _15: u64;
let _16: f32;
let _17: [u32; 2];
let _18: Adt55;
let _19: isize;
let _20: usize;
let _21: Adt59;
let _22: Adt59;
let _23: isize;
let _24: i32;
let _25: *const u64;
let _26: usize;
let _27: isize;
let _28: ();
let _29: ();
{
_3 = _6;
_6.0 = 37093_u16 as i64;
_4 = _2 as i64;
_5 = _8 * _8;
_9 = _4 >> _10;
_7.0 = (_1,);
_6 = (_10,);
RET = (-77_i8) + 104_i8;
_3 = (_4,);
_7 = (_3,);
_12 = [228_u8,25_u8,14_u8,224_u8,12_u8,219_u8,83_u8];
_3.0 = -_6.0;
_1 = 249_u8 as i64;
Call(_4 = core::intrinsics::bswap(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = [_10,_9,_10,_1,_10,_6.0];
_2 = -_5;
_7 = (_6,);
_14 = [_1,_10,_10,_1,_10,_6.0];
_14 = [_3.0,_10,_9,_7.0.0,_3.0,_7.0.0];
_13 = (3_usize,);
_12 = [162_u8,217_u8,125_u8,239_u8,68_u8,2_u8,72_u8];
_10 = !_6.0;
_7.0.0 = _4 >> _4;
_16 = 259183479709818975603779978000574488844_u128 as f32;
RET = (-117_i8) ^ (-16_i8);
_14 = [_10,_7.0.0,_4,_1,_6.0,_10];
_8 = _2;
match _13.0 {
0 => bb2,
1 => bb3,
2 => bb4,
4 => bb6,
5 => bb7,
3 => bb9,
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
_8 = 994925338_u32 as isize;
_13 = (5_usize,);
_3.0 = _1 >> _6.0;
_16 = 50203_u16 as f32;
_17 = [3279166279_u32,251633383_u32];
RET = '\u{3c899}' as i8;
_3 = (_9,);
_3 = (_1,);
_14 = [_7.0.0,_4,_6.0,_10,_10,_6.0];
_16 = (-8068_i16) as f32;
_7 = (_6,);
_3.0 = _7.0.0 ^ _6.0;
RET = !51_i8;
RET = !112_i8;
_6 = (_3.0,);
_8 = -_5;
_3 = _7.0;
_7.0.0 = (-20239464609858188347613245790147853278_i128) as i64;
Goto(bb10)
}
bb10 = {
_5 = -_2;
_13 = (5_usize,);
_10 = _16 as i64;
_7.0.0 = 35892571009880334306445820506623418447_u128 as i64;
_19 = !_5;
_7.0.0 = -_9;
_15 = 5305742024680306183_u64;
_17 = [2523836369_u32,3174400722_u32];
_9 = _8 as i64;
_14 = [_6.0,_9,_9,_7.0.0,_9,_7.0.0];
_2 = -_19;
RET = 39_i8;
_20 = !_13.0;
_13 = (_20,);
_8 = 297703714_i32 as isize;
Call(_1 = fn12(_15, _7, _7.0, RET, _17, _17, _12, _12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = (_7.0.0,);
_16 = 35804_u16 as f32;
_6.0 = _7.0.0 * _3.0;
_7 = (_6,);
_3 = (_9,);
match RET {
0 => bb10,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
39 => bb18,
_ => bb17
}
}
bb12 = {
_5 = -_2;
_13 = (5_usize,);
_10 = _16 as i64;
_7.0.0 = 35892571009880334306445820506623418447_u128 as i64;
_19 = !_5;
_7.0.0 = -_9;
_15 = 5305742024680306183_u64;
_17 = [2523836369_u32,3174400722_u32];
_9 = _8 as i64;
_14 = [_6.0,_9,_9,_7.0.0,_9,_7.0.0];
_2 = -_19;
RET = 39_i8;
_20 = !_13.0;
_13 = (_20,);
_8 = 297703714_i32 as isize;
Call(_1 = fn12(_15, _7, _7.0, RET, _17, _17, _12, _12), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_8 = 994925338_u32 as isize;
_13 = (5_usize,);
_3.0 = _1 >> _6.0;
_16 = 50203_u16 as f32;
_17 = [3279166279_u32,251633383_u32];
RET = '\u{3c899}' as i8;
_3 = (_9,);
_3 = (_1,);
_14 = [_7.0.0,_4,_6.0,_10,_10,_6.0];
_16 = (-8068_i16) as f32;
_7 = (_6,);
_3.0 = _7.0.0 ^ _6.0;
RET = !51_i8;
RET = !112_i8;
_6 = (_3.0,);
_8 = -_5;
_3 = _7.0;
_7.0.0 = (-20239464609858188347613245790147853278_i128) as i64;
Goto(bb10)
}
bb14 = {
_14 = [_10,_9,_10,_1,_10,_6.0];
_2 = -_5;
_7 = (_6,);
_14 = [_1,_10,_10,_1,_10,_6.0];
_14 = [_3.0,_10,_9,_7.0.0,_3.0,_7.0.0];
_13 = (3_usize,);
_12 = [162_u8,217_u8,125_u8,239_u8,68_u8,2_u8,72_u8];
_10 = !_6.0;
_7.0.0 = _4 >> _4;
_16 = 259183479709818975603779978000574488844_u128 as f32;
RET = (-117_i8) ^ (-16_i8);
_14 = [_10,_7.0.0,_4,_1,_6.0,_10];
_8 = _2;
match _13.0 {
0 => bb2,
1 => bb3,
2 => bb4,
4 => bb6,
5 => bb7,
3 => bb9,
_ => bb8
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_20 = 153229692369080204649725203504194925515_u128 as usize;
_4 = _1 - _1;
_24 = 2010292789_i32 * 117806079_i32;
_15 = _20 as u64;
_17 = [3963581483_u32,1128364555_u32];
_14 = [_10,_6.0,_6.0,_4,_6.0,_4];
_25 = core::ptr::addr_of!(_15);
RET = (-40_i8);
_17 = [2887217210_u32,1200263878_u32];
_3 = _7.0;
_17 = [1206806490_u32,2613904129_u32];
_15 = !7859307117579467396_u64;
_23 = _5;
_4 = _1 + _7.0.0;
_12 = [248_u8,78_u8,48_u8,40_u8,105_u8,28_u8,129_u8];
RET = !15_i8;
_13.0 = '\u{71f93}' as usize;
_20 = _13.0;
_13 = (_20,);
_8 = !_2;
(*_25) = 1542371363807444255_u64 - 1054008018249415183_u64;
_13.0 = _20;
Goto(bb19)
}
bb19 = {
Call(_28 = dump_var(11_usize, 7_usize, Move(_7), 12_usize, Move(_12), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(11_usize, 24_usize, Move(_24), 9_usize, Move(_9), 15_usize, Move(_15), 5_usize, Move(_5)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_28 = dump_var(11_usize, 1_usize, Move(_1), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u64,mut _2: ((i64,),),mut _3: (i64,),mut _4: i8,mut _5: [u32; 2],mut _6: [u32; 2],mut _7: [u8; 7],mut _8: [u8; 7]) -> i64 {
mir! {
type RET = i64;
let _9: Adt47;
let _10: u64;
let _11: ([i64; 6], usize, *mut u128);
let _12: u64;
let _13: char;
let _14: *const ((u8,), u32, u32, char);
let _15: [i64; 6];
let _16: *mut (u8,);
let _17: *mut (u8,);
let _18: ((u8,), u32, u32, char);
let _19: Adt57;
let _20: char;
let _21: i32;
let _22: &'static ((i64,),);
let _23: usize;
let _24: [u8; 4];
let _25: i128;
let _26: isize;
let _27: char;
let _28: [i8; 2];
let _29: (u64, usize, u64);
let _30: [i8; 2];
let _31: (u64, usize, u64);
let _32: Adt52;
let _33: ();
let _34: ();
{
RET = 4177805837_u32 as i64;
_8 = [136_u8,60_u8,36_u8,157_u8,158_u8,21_u8,220_u8];
RET = _2.0.0;
_8 = [232_u8,97_u8,173_u8,146_u8,183_u8,251_u8,244_u8];
RET = -_2.0.0;
_2 = (_3,);
_6 = [1786077858_u32,2531429046_u32];
_7 = [145_u8,232_u8,128_u8,168_u8,247_u8,191_u8,225_u8];
_3.0 = _2.0.0;
_6 = [482287479_u32,3993301765_u32];
_1 = !17206151791602836613_u64;
_4 = 71_i8;
Goto(bb1)
}
bb1 = {
_4 = (-70_i8);
_8 = [74_u8,24_u8,209_u8,232_u8,131_u8,148_u8,174_u8];
_2.0 = _3;
_3 = _2.0;
_7 = [59_u8,21_u8,209_u8,141_u8,217_u8,108_u8,61_u8];
_2.0 = (_3.0,);
_10 = _1;
_1 = !_10;
_8 = _7;
_5 = [688520690_u32,1831341431_u32];
_1 = _10 ^ _10;
_1 = !_10;
_3.0 = !_2.0.0;
_8 = _7;
Call(_11.2 = fn13(_2.0, _7, _2.0, _2, _2.0.0, _2.0.0, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = '\u{99097}';
_10 = _1;
RET = _2.0.0;
_11.1 = (-73442874373293259462504142497592895403_i128) as usize;
_3 = _2.0;
_7 = [121_u8,113_u8,221_u8,178_u8,179_u8,29_u8,253_u8];
_3 = (_2.0.0,);
_11.0 = [_3.0,RET,_2.0.0,_2.0.0,_3.0,RET];
_12 = _10 + _10;
_3 = (RET,);
_11.0 = [RET,_2.0.0,RET,_3.0,_3.0,RET];
_2.0.0 = (-30602_i16) as i64;
_13 = '\u{87b9}';
_14 = core::ptr::addr_of!(_18);
(*_14).0.0 = 171_u8;
_10 = _4 as u64;
_18.3 = _13;
(*_14).1 = !3420986117_u32;
_2.0.0 = true as i64;
_15 = [_3.0,RET,_3.0,_3.0,_3.0,_2.0.0];
_5 = _6;
match (*_14).0.0 {
171 => bb3,
_ => bb1
}
}
bb3 = {
RET = -_3.0;
_18.2 = (*_14).1;
_20 = (*_14).3;
_19 = Adt57::Variant2 { fld0: _11.1,fld1: _10 };
SetDiscriminant(_19, 2);
(*_14).2 = _18.1 * (*_14).1;
(*_14).1 = 8983_u16 as u32;
_2.0 = (_3.0,);
(*_14).3 = _13;
_7 = [(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,_18.0.0,(*_14).0.0];
_21 = -(-873669409_i32);
(*_14).0 = (57_u8,);
_16 = core::ptr::addr_of_mut!(_18.0);
place!(Field::<u64>(Variant(_19, 2), 1)) = _1 ^ _1;
_21 = 1524508562_i32;
_10 = _12;
(*_14).0 = (203_u8,);
_8 = _7;
_18.0 = (189_u8,);
_18.1 = _21 as u32;
match _18.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
189 => bb9,
_ => bb8
}
}
bb4 = {
_13 = '\u{99097}';
_10 = _1;
RET = _2.0.0;
_11.1 = (-73442874373293259462504142497592895403_i128) as usize;
_3 = _2.0;
_7 = [121_u8,113_u8,221_u8,178_u8,179_u8,29_u8,253_u8];
_3 = (_2.0.0,);
_11.0 = [_3.0,RET,_2.0.0,_2.0.0,_3.0,RET];
_12 = _10 + _10;
_3 = (RET,);
_11.0 = [RET,_2.0.0,RET,_3.0,_3.0,RET];
_2.0.0 = (-30602_i16) as i64;
_13 = '\u{87b9}';
_14 = core::ptr::addr_of!(_18);
(*_14).0.0 = 171_u8;
_10 = _4 as u64;
_18.3 = _13;
(*_14).1 = !3420986117_u32;
_2.0.0 = true as i64;
_15 = [_3.0,RET,_3.0,_3.0,_3.0,_2.0.0];
_5 = _6;
match (*_14).0.0 {
171 => bb3,
_ => bb1
}
}
bb5 = {
_4 = (-70_i8);
_8 = [74_u8,24_u8,209_u8,232_u8,131_u8,148_u8,174_u8];
_2.0 = _3;
_3 = _2.0;
_7 = [59_u8,21_u8,209_u8,141_u8,217_u8,108_u8,61_u8];
_2.0 = (_3.0,);
_10 = _1;
_1 = !_10;
_8 = _7;
_5 = [688520690_u32,1831341431_u32];
_1 = _10 ^ _10;
_1 = !_10;
_3.0 = !_2.0.0;
_8 = _7;
Call(_11.2 = fn13(_2.0, _7, _2.0, _2, _2.0.0, _2.0.0, _3), ReturnTo(bb2), UnwindUnreachable())
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
_18.3 = _20;
_14 = core::ptr::addr_of!((*_14));
_4 = -(-86_i8);
_2.0.0 = RET ^ RET;
_3 = (_2.0.0,);
(*_14).1 = !(*_14).2;
(*_16) = (242_u8,);
_2.0.0 = _3.0 & RET;
(*_16).0 = _4 as u8;
_25 = -(-70356550575555205484228776745845066982_i128);
_29.2 = _12;
_3.0 = _2.0.0;
_11.0 = [_2.0.0,_2.0.0,_2.0.0,_2.0.0,RET,_2.0.0];
(*_14).2 = (*_14).1 << _3.0;
(*_14).0 = (109_u8,);
(*_14).0 = (31_u8,);
place!(Field::<u64>(Variant(_19, 2), 1)) = _1;
_11.1 = 14166973569886633782_usize;
_10 = _4 as u64;
_11.0 = _15;
_29.0 = !Field::<u64>(Variant(_19, 2), 1);
_29.1 = _11.1 * _11.1;
match (*_14).0.0 {
0 => bb7,
1 => bb2,
2 => bb10,
3 => bb11,
31 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
RET = -_3.0;
_18.2 = (*_14).1;
_20 = (*_14).3;
_19 = Adt57::Variant2 { fld0: _11.1,fld1: _10 };
SetDiscriminant(_19, 2);
(*_14).2 = _18.1 * (*_14).1;
(*_14).1 = 8983_u16 as u32;
_2.0 = (_3.0,);
(*_14).3 = _13;
_7 = [(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,_18.0.0,(*_14).0.0];
_21 = -(-873669409_i32);
(*_14).0 = (57_u8,);
_16 = core::ptr::addr_of_mut!(_18.0);
place!(Field::<u64>(Variant(_19, 2), 1)) = _1 ^ _1;
_21 = 1524508562_i32;
_10 = _12;
(*_14).0 = (203_u8,);
_8 = _7;
_18.0 = (189_u8,);
_18.1 = _21 as u32;
match _18.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
189 => bb9,
_ => bb8
}
}
bb13 = {
_23 = (-113_isize) as usize;
match _18.0.0 {
0 => bb14,
1 => bb15,
31 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
RET = -_3.0;
_18.2 = (*_14).1;
_20 = (*_14).3;
_19 = Adt57::Variant2 { fld0: _11.1,fld1: _10 };
SetDiscriminant(_19, 2);
(*_14).2 = _18.1 * (*_14).1;
(*_14).1 = 8983_u16 as u32;
_2.0 = (_3.0,);
(*_14).3 = _13;
_7 = [(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,(*_14).0.0,_18.0.0,(*_14).0.0];
_21 = -(-873669409_i32);
(*_14).0 = (57_u8,);
_16 = core::ptr::addr_of_mut!(_18.0);
place!(Field::<u64>(Variant(_19, 2), 1)) = _1 ^ _1;
_21 = 1524508562_i32;
_10 = _12;
(*_14).0 = (203_u8,);
_8 = _7;
_18.0 = (189_u8,);
_18.1 = _21 as u32;
match _18.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
189 => bb9,
_ => bb8
}
}
bb17 = {
_3.0 = _2.0.0;
_11.1 = !_23;
_29.1 = _11.1 << (*_14).0.0;
(*_16).0 = 80_u8 ^ 153_u8;
_28 = [_4,_4];
_7 = [_18.0.0,(*_16).0,_18.0.0,(*_14).0.0,(*_16).0,_18.0.0,_18.0.0];
_31 = (_29.2, _23, _12);
_2.0.0 = RET;
RET = _3.0;
RET = _3.0 - _3.0;
(*_14).3 = _13;
place!(Field::<usize>(Variant(_19, 2), 0)) = (*_14).2 as usize;
RET = !_2.0.0;
_2 = (_3,);
(*_16) = (7_u8,);
_2.0.0 = _3.0;
_29.1 = 199907242791733831126784186769723929205_u128 as usize;
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(12_usize, 23_usize, Move(_23), 15_usize, Move(_15), 21_usize, Move(_21), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(12_usize, 3_usize, Move(_3), 5_usize, Move(_5), 8_usize, Move(_8), 31_usize, Move(_31)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(12_usize, 29_usize, Move(_29), 6_usize, Move(_6), 34_usize, _34, 34_usize, _34), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (i64,),mut _2: [u8; 7],mut _3: (i64,),mut _4: ((i64,),),mut _5: i64,mut _6: i64,mut _7: (i64,)) -> *mut u128 {
mir! {
type RET = *mut u128;
let _8: isize;
let _9: f64;
let _10: isize;
let _11: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128));
let _12: isize;
let _13: Adt56;
let _14: char;
let _15: bool;
let _16: Adt58;
let _17: Adt48;
let _18: u16;
let _19: ((u8,), u32, u32, char);
let _20: i32;
let _21: *const [u8; 4];
let _22: ((u8,), u32, u32, char);
let _23: *const u16;
let _24: f64;
let _25: u64;
let _26: (i8, i8, usize, ((u8,), u32, u32, char), u8);
let _27: ([i64; 6], usize, *mut u128);
let _28: f32;
let _29: Adt60;
let _30: u16;
let _31: f64;
let _32: [char; 5];
let _33: f64;
let _34: f32;
let _35: f32;
let _36: Adt53;
let _37: usize;
let _38: usize;
let _39: char;
let _40: ((i64,),);
let _41: Adt62;
let _42: [i8; 2];
let _43: f32;
let _44: Adt61;
let _45: char;
let _46: [i8; 5];
let _47: u32;
let _48: isize;
let _49: (usize,);
let _50: u16;
let _51: [i64; 6];
let _52: f64;
let _53: [i128; 3];
let _54: bool;
let _55: Adt49;
let _56: f32;
let _57: [u32; 2];
let _58: Adt47;
let _59: [char; 5];
let _60: usize;
let _61: u128;
let _62: ();
let _63: ();
{
_2 = [207_u8,194_u8,148_u8,180_u8,64_u8,251_u8,233_u8];
_4.0.0 = _3.0;
_2 = [38_u8,199_u8,98_u8,164_u8,1_u8,55_u8,92_u8];
_4.0 = _3;
_6 = !_3.0;
_5 = !_6;
_5 = 57_i8 as i64;
_8 = '\u{ada86}' as isize;
_1 = _7;
_2 = [105_u8,168_u8,147_u8,38_u8,104_u8,185_u8,200_u8];
_5 = _6 ^ _6;
_8 = !(-71_isize);
_1 = (_7.0,);
_1 = (_5,);
_5 = -_4.0.0;
_8 = 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_6 = _3.0 >> _3.0;
_4.0 = (_7.0,);
_10 = _8;
_2 = [189_u8,166_u8,190_u8,104_u8,44_u8,177_u8,251_u8];
_11.1 = [95_u8,23_u8,242_u8,109_u8,136_u8,51_u8,148_u8];
_3 = (_6,);
_7 = (_1.0,);
_4 = (_3,);
_4 = (_3,);
_11.2.0 = [_5,_1.0,_4.0.0,_5,_6,_7.0];
_1.0 = -_3.0;
_1.0 = -_6;
_8 = -_10;
_13 = Adt56::Variant2 { fld0: _4,fld1: _6 };
_16.fld2.2 = 104368098901189962980820977461215641115_i128 as u64;
_6 = Field::<i64>(Variant(_13, 2), 1);
_16.fld1 = [545703650_u32,2426489688_u32];
_12 = _10;
_3.0 = -_1.0;
_4 = Field::<((i64,),)>(Variant(_13, 2), 0);
_6 = _4.0.0 << _3.0;
_11.2.0 = [_5,_7.0,Field::<i64>(Variant(_13, 2), 1),_5,Field::<i64>(Variant(_13, 2), 1),_3.0];
_8 = '\u{4fb55}' as isize;
place!(Field::<i64>(Variant(_13, 2), 1)) = 465124150_i32 as i64;
_16.fld2.1 = !15691105361086772951_usize;
Call(_15 = fn14(Field::<((i64,),)>(Variant(_13, 2), 0).0, Move(_13), _4.0.0, _12, _10, _4.0.0, _6, _4.0, _4.0, _4, _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.0 = (_6,);
_14 = '\u{6ed9e}';
_1.0 = 288450685000159906228281214103502091662_u128 as i64;
_4.0.0 = (-661850549_i32) as i64;
_14 = '\u{4f43}';
_6 = _4.0.0 - _3.0;
_11.2.1 = _8 as usize;
_16.fld2.1 = !_11.2.1;
Goto(bb3)
}
bb3 = {
_15 = !false;
_16.fld2.2 = (-19299_i16) as u64;
_11.0 = [18_i8,61_i8];
_11.2.0 = [_3.0,_1.0,_3.0,_3.0,_6,_7.0];
_9 = 238209634174616357170041648796506765086_u128 as f64;
_14 = '\u{eeba3}';
_14 = '\u{f9169}';
_17.fld2.0 = (_16.fld2.2, _16.fld2.1, _16.fld2.2);
_16.fld2.0 = !_16.fld2.2;
Call(_17.fld3 = fn15(_6, _1.0, _4.0, _3.0, _7.0, _11.2.0, _11.2.0, _2, _7, _3, _11.0, _4.0.0, _10, _16.fld2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16.fld2 = (_17.fld2.0.0, _17.fld2.0.1, _17.fld2.0.0);
_11.2.0 = [_7.0,_6,_6,_3.0,_7.0,_6];
_2 = _11.1;
_4.0.0 = _5;
_16.fld2 = (_17.fld2.0.0, _17.fld2.0.1, _17.fld2.0.0);
_17.fld1.0 = 256324843557163427842010408265381959034_u128 as isize;
Goto(bb5)
}
bb5 = {
_17.fld0 = _2;
_17.fld2.2 = core::ptr::addr_of!(_19);
_16.fld2.2 = 21794_u16 as u64;
_14 = '\u{ae7bc}';
Goto(bb6)
}
bb6 = {
_17.fld2.0.0 = 197951036905875442908997178207769306391_u128 as u64;
Goto(bb7)
}
bb7 = {
_3 = _7;
_11.1 = [71_u8,150_u8,107_u8,245_u8,98_u8,150_u8,88_u8];
_19.2 = (-32414_i16) as u32;
_17.fld2.0.1 = _16.fld2.1 - _16.fld2.1;
_4.0 = (_5,);
Goto(bb8)
}
bb8 = {
_14 = '\u{14491}';
_15 = !true;
_17.fld2.0.1 = _16.fld2.1;
_11.2.1 = _17.fld2.0.1 & _17.fld2.0.1;
_17.fld2.1 = [_7.0,_5,_3.0,_6,_6,_6];
_19.0.0 = 229_u8 - 222_u8;
_12 = -_8;
_17.fld2.0 = (_16.fld2.2, _11.2.1, _16.fld2.0);
_18 = _9 as u16;
_7 = (_3.0,);
_11.0 = [_17.fld3,_17.fld3];
_22.0 = (_19.0.0,);
_17.fld1.0 = _12;
_19.3 = _14;
_3 = (_4.0.0,);
match _10 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb6,
9223372036854775807 => bb10,
_ => bb9
}
}
bb9 = {
_15 = !false;
_16.fld2.2 = (-19299_i16) as u64;
_11.0 = [18_i8,61_i8];
_11.2.0 = [_3.0,_1.0,_3.0,_3.0,_6,_7.0];
_9 = 238209634174616357170041648796506765086_u128 as f64;
_14 = '\u{eeba3}';
_14 = '\u{f9169}';
_17.fld2.0 = (_16.fld2.2, _16.fld2.1, _16.fld2.2);
_16.fld2.0 = !_16.fld2.2;
Call(_17.fld3 = fn15(_6, _1.0, _4.0, _3.0, _7.0, _11.2.0, _11.2.0, _2, _7, _3, _11.0, _4.0.0, _10, _16.fld2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_17.fld0 = _11.1;
_17.fld4 = (_11.2.1,);
_4.0.0 = _6 & _3.0;
_13 = Adt56::Variant2 { fld0: _4,fld1: _4.0.0 };
_7 = _3;
_23 = core::ptr::addr_of!(_18);
_17.fld2.1 = [_4.0.0,Field::<i64>(Variant(_13, 2), 1),_4.0.0,Field::<i64>(Variant(_13, 2), 1),Field::<((i64,),)>(Variant(_13, 2), 0).0.0,Field::<i64>(Variant(_13, 2), 1)];
_22 = (_19.0, _19.2, _19.2, _19.3);
_17.fld2.0.0 = _16.fld2.0;
_25 = _11.2.1 as u64;
_19.1 = _8 as u32;
_11.0 = [_17.fld3,_17.fld3];
_17.fld2.0 = (_16.fld2.2, _17.fld4.0, _16.fld2.0);
_26.4 = _22.0.0 << _17.fld4.0;
_26.3.2 = !_22.1;
_17.fld2.0 = (_16.fld2.2, _17.fld4.0, _25);
_17.fld1.0 = _8 - _10;
_7.0 = !_4.0.0;
place!(Field::<((i64,),)>(Variant(_13, 2), 0)).0.0 = Field::<i64>(Variant(_13, 2), 1);
_26 = (_17.fld3, _17.fld3, _16.fld2.1, _19, _19.0.0);
match _10 {
9223372036854775807 => bb11,
_ => bb2
}
}
bb11 = {
_17.fld4 = (_17.fld2.0.1,);
_6 = _4.0.0;
Goto(bb12)
}
bb12 = {
_26.3.0.0 = _22.0.0 - _22.0.0;
_19.2 = _19.1;
_17.fld2.1 = _11.2.0;
_17.fld2.1 = [_7.0,_6,_4.0.0,Field::<i64>(Variant(_13, 2), 1),_6,_4.0.0];
place!(Field::<((i64,),)>(Variant(_13, 2), 0)).0 = (_6,);
_17.fld2.3 = [_26.1,_17.fld3,_26.1,_26.0,_26.1];
_19.0.0 = !_22.0.0;
_11.2.0 = [Field::<i64>(Variant(_13, 2), 1),Field::<i64>(Variant(_13, 2), 1),Field::<i64>(Variant(_13, 2), 1),_4.0.0,Field::<((i64,),)>(Variant(_13, 2), 0).0.0,Field::<((i64,),)>(Variant(_13, 2), 0).0.0];
_9 = _17.fld1.0 as f64;
Goto(bb13)
}
bb13 = {
_19.1 = (-23809_i16) as u32;
_17.fld2.0 = (_25, _11.2.1, _25);
_22.2 = _19.2;
_3.0 = Field::<((i64,),)>(Variant(_13, 2), 0).0.0 << _26.0;
_19 = (_26.3.0, _22.2, _26.3.2, _22.3);
_16.fld2.2 = _26.0 as u64;
_17.fld2.0.1 = _26.2;
_8 = (*_23) as isize;
_17.fld3 = _6 as i8;
_1.0 = _26.3.1 as i64;
_17.fld2.0.0 = !_16.fld2.2;
SetDiscriminant(_13, 0);
_26.3 = (_19.0, _19.2, _19.1, _14);
_3 = (_4.0.0,);
_17.fld4.0 = _17.fld2.0.1 ^ _26.2;
_26.3.3 = _22.3;
_11.2.0 = [_4.0.0,_7.0,_4.0.0,_6,_6,_4.0.0];
_19.0.0 = _22.0.0 & _26.3.0.0;
_22.1 = _19.1 ^ _26.3.1;
_11.1 = [_19.0.0,_26.3.0.0,_26.3.0.0,_19.0.0,_22.0.0,_26.4,_26.3.0.0];
_11.2.0 = [_6,_3.0,_4.0.0,_6,_4.0.0,_6];
match _10 {
0 => bb14,
9223372036854775807 => bb16,
_ => bb15
}
}
bb14 = {
_15 = !false;
_16.fld2.2 = (-19299_i16) as u64;
_11.0 = [18_i8,61_i8];
_11.2.0 = [_3.0,_1.0,_3.0,_3.0,_6,_7.0];
_9 = 238209634174616357170041648796506765086_u128 as f64;
_14 = '\u{eeba3}';
_14 = '\u{f9169}';
_17.fld2.0 = (_16.fld2.2, _16.fld2.1, _16.fld2.2);
_16.fld2.0 = !_16.fld2.2;
Call(_17.fld3 = fn15(_6, _1.0, _4.0, _3.0, _7.0, _11.2.0, _11.2.0, _2, _7, _3, _11.0, _4.0.0, _10, _16.fld2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
_17.fld2.0.0 = 197951036905875442908997178207769306391_u128 as u64;
Goto(bb7)
}
bb16 = {
_19.2 = _26.3.2 - _19.1;
_1 = (_6,);
place!(Field::<([i64; 6], usize, *mut u128)>(Variant(_13, 0), 1)).0 = _11.2.0;
_2 = [_26.4,_19.0.0,_19.0.0,_26.3.0.0,_19.0.0,_19.0.0,_19.0.0];
_26.3.1 = _26.3.2;
_16.fld2 = _17.fld2.0;
_28 = (-18368489866411250175685716275603216735_i128) as f32;
_17.fld2.0.1 = _17.fld4.0 | _11.2.1;
_11.0 = [_26.1,_26.0];
match _10 {
0 => bb11,
1 => bb9,
2 => bb8,
3 => bb4,
4 => bb13,
9223372036854775807 => bb17,
_ => bb10
}
}
bb17 = {
_32 = [_26.3.3,_26.3.3,_22.3,_22.3,_19.3];
_26.3.3 = _14;
_23 = core::ptr::addr_of!(_18);
_26.2 = !_17.fld4.0;
_6 = _7.0 * _5;
_31 = _26.1 as f64;
_35 = 551327734_i32 as f32;
_19 = _26.3;
_7 = (_4.0.0,);
_17.fld4.0 = _17.fld2.0.1;
_17.fld0 = [_19.0.0,_26.4,_19.0.0,_26.3.0.0,_26.3.0.0,_19.0.0,_26.3.0.0];
_27.0 = [_6,_3.0,_7.0,_1.0,_7.0,_1.0];
_17.fld1 = (_10, _31);
_2 = _11.1;
_19.2 = _22.1 | _26.3.1;
_19.0 = _26.3.0;
Goto(bb18)
}
bb18 = {
_27.0 = [_3.0,_6,_7.0,_6,_7.0,_7.0];
_19.0 = (_26.4,);
_11.0 = [_26.1,_26.1];
_16.fld2.2 = !_16.fld2.0;
_16.fld2 = _17.fld2.0;
place!(Field::<Adt50>(Variant(_13, 0), 0)) = Adt50::Variant0 { fld0: (*_23),fld1: _26.3.0.0 };
place!(Field::<u16>(Variant(place!(Field::<Adt50>(Variant(_13, 0), 0)), 0), 0)) = !(*_23);
_26.0 = _17.fld2.0.1 as i8;
_22.0 = _26.3.0;
_7 = (_4.0.0,);
_26.3.2 = _19.1 - _19.2;
_11.2.0 = [_6,_3.0,_6,_4.0.0,_6,_7.0];
place!(Field::<u8>(Variant(place!(Field::<Adt50>(Variant(_13, 0), 0)), 0), 1)) = _19.0.0;
_17.fld2.0 = (_16.fld2.0, _16.fld2.1, _16.fld2.0);
_34 = (-14273_i16) as f32;
_26.3.0.0 = _22.0.0;
_16.fld2.0 = _16.fld2.2 - _17.fld2.0.0;
_27.0 = [_7.0,_1.0,_1.0,_6,_5,_1.0];
_34 = _16.fld2.0 as f32;
_44.fld4.3.2 = _19.2;
match _17.fld1.0 {
0 => bb6,
1 => bb15,
2 => bb5,
3 => bb10,
4 => bb19,
9223372036854775807 => bb21,
_ => bb20
}
}
bb19 = {
_32 = [_26.3.3,_26.3.3,_22.3,_22.3,_19.3];
_26.3.3 = _14;
_23 = core::ptr::addr_of!(_18);
_26.2 = !_17.fld4.0;
_6 = _7.0 * _5;
_31 = _26.1 as f64;
_35 = 551327734_i32 as f32;
_19 = _26.3;
_7 = (_4.0.0,);
_17.fld4.0 = _17.fld2.0.1;
_17.fld0 = [_19.0.0,_26.4,_19.0.0,_26.3.0.0,_26.3.0.0,_19.0.0,_26.3.0.0];
_27.0 = [_6,_3.0,_7.0,_1.0,_7.0,_1.0];
_17.fld1 = (_10, _31);
_2 = _11.1;
_19.2 = _22.1 | _26.3.1;
_19.0 = _26.3.0;
Goto(bb18)
}
bb20 = {
_17.fld0 = _11.1;
_17.fld4 = (_11.2.1,);
_4.0.0 = _6 & _3.0;
_13 = Adt56::Variant2 { fld0: _4,fld1: _4.0.0 };
_7 = _3;
_23 = core::ptr::addr_of!(_18);
_17.fld2.1 = [_4.0.0,Field::<i64>(Variant(_13, 2), 1),_4.0.0,Field::<i64>(Variant(_13, 2), 1),Field::<((i64,),)>(Variant(_13, 2), 0).0.0,Field::<i64>(Variant(_13, 2), 1)];
_22 = (_19.0, _19.2, _19.2, _19.3);
_17.fld2.0.0 = _16.fld2.0;
_25 = _11.2.1 as u64;
_19.1 = _8 as u32;
_11.0 = [_17.fld3,_17.fld3];
_17.fld2.0 = (_16.fld2.2, _17.fld4.0, _16.fld2.0);
_26.4 = _22.0.0 << _17.fld4.0;
_26.3.2 = !_22.1;
_17.fld2.0 = (_16.fld2.2, _17.fld4.0, _25);
_17.fld1.0 = _8 - _10;
_7.0 = !_4.0.0;
place!(Field::<((i64,),)>(Variant(_13, 2), 0)).0.0 = Field::<i64>(Variant(_13, 2), 1);
_26 = (_17.fld3, _17.fld3, _16.fld2.1, _19, _19.0.0);
match _10 {
9223372036854775807 => bb11,
_ => bb2
}
}
bb21 = {
_13 = Adt56::Variant2 { fld0: _4,fld1: _4.0.0 };
_16.fld2.0 = !_17.fld2.0.2;
_40.0.0 = -_6;
_40.0 = (Field::<i64>(Variant(_13, 2), 1),);
_17.fld2.3 = [_26.1,_26.1,_17.fld3,_26.1,_26.1];
_24 = _17.fld1.1 + _31;
_44.fld3 = _17.fld4.0 as i8;
SetDiscriminant(_13, 1);
_45 = _26.3.3;
_44.fld4.1 = _26.1;
_44.fld7 = Adt47::Variant2 { fld0: _17.fld1 };
Goto(bb22)
}
bb22 = {
_22.2 = !_26.3.2;
_27.1 = !_17.fld4.0;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_13, 1), 2)).2.1 = !_11.2.1;
place!(Field::<(isize, f64)>(Variant(_13, 1), 0)).1 = _24;
_44.fld4.1 = _26.1 - _26.1;
Call(place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_13, 1), 2)) = fn17(_4.0.0, _17.fld2), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_11.1 = [_26.3.0.0,_22.0.0,_19.0.0,_22.0.0,_19.0.0,_26.3.0.0,_22.0.0];
_13 = Adt56::Variant2 { fld0: _40,fld1: _4.0.0 };
_44.fld4.4 = _22.0.0;
_7 = Field::<((i64,),)>(Variant(_13, 2), 0).0;
_4.0.0 = !Field::<i64>(Variant(_13, 2), 1);
_43 = _34;
_14 = _19.3;
_44.fld5 = Adt46::Variant0 { fld0: _26.3,fld1: _17.fld1 };
_25 = _17.fld2.0.0 | _17.fld2.0.0;
_44.fld4.3.1 = !_19.2;
_33 = _24 + Field::<(isize, f64)>(Variant(_44.fld5, 0), 1).1;
_25 = _17.fld2.0.0 | _17.fld2.0.0;
_31 = _33;
_38 = !_17.fld4.0;
_17.fld0 = [_26.3.0.0,_44.fld4.4,_26.3.0.0,_44.fld4.4,_19.0.0,Field::<((u8,), u32, u32, char)>(Variant(_44.fld5, 0), 0).0.0,_19.0.0];
_19.0.0 = _15 as u8;
_44.fld4.3.0 = _22.0;
_19.0 = (_44.fld4.4,);
_17.fld2.0.2 = !_17.fld2.0.0;
_5 = _17.fld4.0 as i64;
place!(Field::<(isize, f64)>(Variant(_44.fld5, 0), 1)).1 = _31;
SetDiscriminant(_44.fld5, 0);
Goto(bb24)
}
bb24 = {
_42 = _11.0;
_17.fld2.1 = [_4.0.0,_3.0,Field::<((i64,),)>(Variant(_13, 2), 0).0.0,Field::<((i64,),)>(Variant(_13, 2), 0).0.0,_6,_3.0];
_10 = -Field::<(isize, f64)>(Variant(_44.fld7, 2), 0).0;
place!(Field::<((u8,), u32, u32, char)>(Variant(_44.fld5, 0), 0)).2 = _19.2;
_44.fld4.3.0.0 = _19.0.0 & _44.fld4.4;
_3.0 = _1.0 - _6;
_40.0.0 = _1.0 | Field::<((i64,),)>(Variant(_13, 2), 0).0.0;
_39 = _19.3;
Goto(bb25)
}
bb25 = {
_48 = _25 as isize;
_44.fld7 = Adt47::Variant2 { fld0: _17.fld1 };
_14 = _45;
_45 = _14;
_26.3 = (_44.fld4.3.0, _22.1, _44.fld4.3.1, _39);
_26.4 = _44.fld4.3.0.0;
_26.3 = _19;
_26.3.2 = !_22.1;
place!(Field::<(isize, f64)>(Variant(_44.fld5, 0), 1)) = (_48, _24);
_30 = _18 * (*_23);
_22.0 = _26.3.0;
_48 = Field::<(isize, f64)>(Variant(_44.fld5, 0), 1).0;
_39 = _22.3;
_16.fld2.2 = !_17.fld2.0.0;
_44.fld7 = Adt47::Variant2 { fld0: Field::<(isize, f64)>(Variant(_44.fld5, 0), 1) };
_30 = (*_23) ^ (*_23);
Call(_16.fld2 = fn19(_44.fld4.1, _44.fld7, _17.fld2.0, _44.fld7, _44.fld7, _44.fld7, _17.fld1.0, Field::<(isize, f64)>(Variant(_44.fld7, 2), 0), Field::<(isize, f64)>(Variant(_44.fld7, 2), 0).0, _5, Field::<(isize, f64)>(Variant(_44.fld7, 2), 0), _26.1), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
SetDiscriminant(_44.fld7, 1);
_42 = [_44.fld4.1,_44.fld4.1];
_17.fld2.2 = core::ptr::addr_of!(_44.fld4.3);
_21 = core::ptr::addr_of!(place!(Field::<[u8; 4]>(Variant(_44.fld7, 1), 1)));
place!(Field::<((u8,), u32, u32, char)>(Variant(_44.fld5, 0), 0)).3 = _14;
_44.fld1 = Move(_13);
_31 = _33;
place!(Field::<[u8; 4]>(Variant(_44.fld7, 1), 1)) = [_19.0.0,_44.fld4.3.0.0,_44.fld4.3.0.0,_44.fld4.4];
_13 = Adt56::Variant2 { fld0: _4,fld1: _3.0 };
_44.fld4.0 = !_44.fld4.1;
_9 = _31;
_44.fld3 = _44.fld4.1;
match _17.fld1.0 {
0 => bb12,
1 => bb6,
2 => bb18,
3 => bb8,
4 => bb5,
9223372036854775807 => bb28,
_ => bb27
}
}
bb27 = {
_15 = !false;
_16.fld2.2 = (-19299_i16) as u64;
_11.0 = [18_i8,61_i8];
_11.2.0 = [_3.0,_1.0,_3.0,_3.0,_6,_7.0];
_9 = 238209634174616357170041648796506765086_u128 as f64;
_14 = '\u{eeba3}';
_14 = '\u{f9169}';
_17.fld2.0 = (_16.fld2.2, _16.fld2.1, _16.fld2.2);
_16.fld2.0 = !_16.fld2.2;
Call(_17.fld3 = fn15(_6, _1.0, _4.0, _3.0, _7.0, _11.2.0, _11.2.0, _2, _7, _3, _11.0, _4.0.0, _10, _16.fld2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb28 = {
_42 = [_44.fld4.0,_44.fld4.1];
RET = core::ptr::addr_of_mut!(_61);
_16.fld2.0 = _17.fld2.0.0 ^ _16.fld2.2;
_44.fld4.3.1 = Field::<((u8,), u32, u32, char)>(Variant(_44.fld5, 0), 0).2;
_27.2 = core::ptr::addr_of_mut!((*RET));
place!(Field::<i64>(Variant(_13, 2), 1)) = _1.0;
_26.3 = (_44.fld4.3.0, _22.2, _19.1, _14);
_22.2 = _19.2 << _17.fld2.0.0;
place!(Field::<((u8,), u32, u32, char)>(Variant(_44.fld5, 0), 0)).3 = _39;
place!(Field::<(isize, f64)>(Variant(_44.fld5, 0), 1)).1 = _9 * _31;
_27.1 = _18 as usize;
_24 = Field::<(isize, f64)>(Variant(_44.fld5, 0), 1).1 + Field::<(isize, f64)>(Variant(_44.fld5, 0), 1).1;
_34 = -_43;
Goto(bb29)
}
bb29 = {
Call(_62 = dump_var(13_usize, 2_usize, Move(_2), 14_usize, Move(_14), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_62 = dump_var(13_usize, 7_usize, Move(_7), 19_usize, Move(_19), 3_usize, Move(_3), 32_usize, Move(_32)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_62 = dump_var(13_usize, 26_usize, Move(_26), 6_usize, Move(_6), 40_usize, Move(_40), 45_usize, Move(_45)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (i64,),mut _2: Adt56,mut _3: i64,mut _4: isize,mut _5: isize,mut _6: i64,mut _7: i64,mut _8: (i64,),mut _9: (i64,),mut _10: ((i64,),),mut _11: ((i64,),),mut _12: (i64,)) -> bool {
mir! {
type RET = bool;
let _13: bool;
let _14: Adt53;
let _15: *mut u128;
let _16: (usize,);
let _17: bool;
let _18: u128;
let _19: char;
let _20: isize;
let _21: u8;
let _22: i16;
let _23: ();
let _24: ();
{
place!(Field::<i64>(Variant(_2, 2), 1)) = _11.0.0;
_9 = _11.0;
_6 = _7 | Field::<((i64,),)>(Variant(_2, 2), 0).0.0;
_9.0 = 16224187943370805691_u64 as i64;
place!(Field::<((i64,),)>(Variant(_2, 2), 0)).0 = (_7,);
SetDiscriminant(_2, 1);
_7 = -_8.0;
_9 = _10.0;
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).0 = _5;
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).1 = 1129667077_u32 as f64;
_5 = Field::<(isize, f64)>(Variant(_2, 1), 0).0 >> _7;
_10 = (_11.0,);
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).0 = [35_i8,(-38_i8)];
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).1 = [242_u8,242_u8,200_u8,217_u8,37_u8,107_u8,239_u8];
_9.0 = _12.0 ^ _12.0;
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).0 = !_5;
_11.0 = (_6,);
_10.0 = (_11.0.0,);
_9 = (_6,);
_9.0 = 698853865_i32 as i64;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).1 = [165_u8,50_u8,231_u8,225_u8,164_u8,183_u8,89_u8];
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).1 = [100_u8,44_u8,15_u8,97_u8,177_u8,231_u8,19_u8];
Call(_1.0 = core::intrinsics::bswap(_12.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 76_u8 as isize;
_6 = _10.0.0 << _8.0;
_18 = 292079658612624646436224449335144920564_u128;
_9 = _10.0;
place!(Field::<(i64,)>(Variant(_2, 1), 1)) = _10.0;
_16.0 = 4_usize ^ 6_usize;
_17 = true | false;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).2.2 = core::ptr::addr_of_mut!(_18);
_11 = (_9,);
RET = _17;
_10 = _11;
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).0 = _16.0 as isize;
_9 = (_6,);
_18 = !109939036206565947922669019312941594862_u128;
Goto(bb2)
}
bb2 = {
_16 = (1607071652955864813_usize,);
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).2.0 = [_7,_12.0,Field::<(i64,)>(Variant(_2, 1), 1).0,_9.0,_1.0,_9.0];
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).2.2 = core::ptr::addr_of_mut!(_18);
_13 = _11.0.0 <= _11.0.0;
place!(Field::<(i64,)>(Variant(_2, 1), 1)).0 = _6 & _10.0.0;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).2.2 = core::ptr::addr_of_mut!(_18);
RET = _17;
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).0 = 24790_i16 as isize;
_11 = (Field::<(i64,)>(Variant(_2, 1), 1),);
_21 = 105_u8;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).0 = [(-1_i8),(-10_i8)];
place!(Field::<(isize, f64)>(Variant(_2, 1), 0)).0 = _5;
_18 = 114031948803341522501911367817603033381_u128 & 309194375322630956242343587440046481958_u128;
RET = Field::<(i64,)>(Variant(_2, 1), 1).0 == _11.0.0;
place!(Field::<(i64,)>(Variant(_2, 1), 1)).0 = _11.0.0;
_20 = _5 >> _6;
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).2.0 = [_9.0,_9.0,Field::<(i64,)>(Variant(_2, 1), 1).0,_9.0,Field::<(i64,)>(Variant(_2, 1), 1).0,_6];
_16 = (10881494802804414922_usize,);
place!(Field::<([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128))>(Variant(_2, 1), 2)).0 = [(-30_i8),32_i8];
Goto(bb3)
}
bb3 = {
Call(_23 = dump_var(14_usize, 4_usize, Move(_4), 16_usize, Move(_16), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_23 = dump_var(14_usize, 10_usize, Move(_10), 11_usize, Move(_11), 18_usize, Move(_18), 21_usize, Move(_21)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i64,mut _2: i64,mut _3: (i64,),mut _4: i64,mut _5: i64,mut _6: [i64; 6],mut _7: [i64; 6],mut _8: [u8; 7],mut _9: (i64,),mut _10: (i64,),mut _11: [i8; 2],mut _12: i64,mut _13: isize,mut _14: u64) -> i8 {
mir! {
type RET = i8;
let _15: i64;
let _16: Adt62;
let _17: [u8; 7];
let _18: [i128; 3];
let _19: (u64, usize, u64);
let _20: Adt61;
let _21: (i8, i8, usize, ((u8,), u32, u32, char), u8);
let _22: f64;
let _23: [u32; 2];
let _24: f64;
let _25: ((i64,),);
let _26: [u8; 7];
let _27: isize;
let _28: char;
let _29: isize;
let _30: i32;
let _31: Adt57;
let _32: i64;
let _33: (isize, f64);
let _34: ((i64,),);
let _35: [u32; 2];
let _36: [i64; 6];
let _37: i32;
let _38: f64;
let _39: f64;
let _40: ();
let _41: ();
{
_6 = [_1,_4,_4,_10.0,_2,_4];
_12 = _14 as i64;
RET = '\u{344d4}' as i8;
_6 = [_10.0,_10.0,_1,_4,_9.0,_1];
_9 = (_10.0,);
_9.0 = _1 & _5;
_10 = (_4,);
_12 = _4;
_5 = _4 + _12;
_4 = _1 & _5;
_15 = _4 - _4;
_7 = [_15,_4,_10.0,_15,_15,_4];
_8 = [138_u8,89_u8,50_u8,193_u8,41_u8,99_u8,56_u8];
RET = (-47_i8);
_12 = _10.0 >> _9.0;
_5 = _1;
RET = (-27_i8);
_10.0 = _9.0 - _1;
_9.0 = _4;
_10 = (_9.0,);
_13 = 30165_u16 as isize;
_9 = _10;
_9 = (_10.0,);
_10.0 = !_5;
_7 = [_9.0,_4,_9.0,_9.0,_15,_12];
Goto(bb1)
}
bb1 = {
_14 = 14240102708866771034_u64;
_11 = [RET,RET];
RET = 2855172818_u32 as i8;
_10 = (_15,);
_8 = [221_u8,145_u8,233_u8,192_u8,83_u8,237_u8,122_u8];
_19.0 = !_14;
_3.0 = _12;
_17 = [157_u8,148_u8,62_u8,240_u8,236_u8,24_u8,68_u8];
_1 = _10.0 << _10.0;
_20.fld6 = 1519438018_u32 as f64;
_21.4 = 17154361817558666181_usize as u8;
_19.1 = _21.4 as usize;
_21.2 = !_19.1;
_3 = (_12,);
_20.fld4.3.0 = (_21.4,);
_21.1 = RET ^ RET;
_6 = _7;
_2 = 28841_i16 as i64;
_21.3.3 = '\u{fb17d}';
_20.fld3 = _21.1;
_20.fld4.3.2 = 1830095056_u32;
_21.3.1 = !_20.fld4.3.2;
_19.2 = _19.0;
_22 = 281108601285443331180480848070618071288_u128 as f64;
Goto(bb2)
}
bb2 = {
_21.4 = _20.fld4.3.0.0;
_21.1 = _20.fld3;
_21.3.2 = 43652_u16 as u32;
_6 = [_12,_4,_15,_10.0,_12,_3.0];
_21.3.0.0 = _21.2 as u8;
_4 = -_12;
_20.fld4.3.1 = 3064_u16 as u32;
_20.fld4.3 = (_21.3.0, _21.3.2, _21.3.1, _21.3.3);
_9.0 = _19.0 as i64;
_3.0 = _15;
_9.0 = _15;
_21.3.1 = _21.3.2 >> _20.fld3;
_8 = _17;
_20.fld4.4 = _21.4 >> _15;
_20.fld3 = _12 as i8;
_10.0 = !_12;
_20.fld4 = (_21.1, _21.1, _21.2, _21.3, _21.4);
_18 = [(-124313728460382733947718191533179325596_i128),(-95475691488524832625119497087117240389_i128),76902961229624052561249943547502859635_i128];
_21.1 = -_20.fld3;
_9 = _3;
_19.1 = _20.fld4.2;
_2 = _21.3.3 as i64;
_13 = _20.fld3 as isize;
_20.fld4.3 = (_21.3.0, _21.3.2, _21.3.1, _21.3.3);
_21.3.0 = (_20.fld4.3.0.0,);
_20.fld4.3 = (_21.3.0, _21.3.2, _21.3.2, _21.3.3);
_19 = (_14, _21.2, _14);
match _19.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
14240102708866771034 => bb8,
_ => bb7
}
}
bb3 = {
_14 = 14240102708866771034_u64;
_11 = [RET,RET];
RET = 2855172818_u32 as i8;
_10 = (_15,);
_8 = [221_u8,145_u8,233_u8,192_u8,83_u8,237_u8,122_u8];
_19.0 = !_14;
_3.0 = _12;
_17 = [157_u8,148_u8,62_u8,240_u8,236_u8,24_u8,68_u8];
_1 = _10.0 << _10.0;
_20.fld6 = 1519438018_u32 as f64;
_21.4 = 17154361817558666181_usize as u8;
_19.1 = _21.4 as usize;
_21.2 = !_19.1;
_3 = (_12,);
_20.fld4.3.0 = (_21.4,);
_21.1 = RET ^ RET;
_6 = _7;
_2 = 28841_i16 as i64;
_21.3.3 = '\u{fb17d}';
_20.fld3 = _21.1;
_20.fld4.3.2 = 1830095056_u32;
_21.3.1 = !_20.fld4.3.2;
_19.2 = _19.0;
_22 = 281108601285443331180480848070618071288_u128 as f64;
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
_21.0 = _20.fld4.4 as i8;
_20.fld4.3.0.0 = _20.fld4.4 + _21.4;
_21.3.2 = _20.fld4.3.2 - _21.3.1;
_20.fld4.1 = _13 as i8;
_2 = _15;
_9.0 = _21.3.2 as i64;
_21.3.3 = _20.fld4.3.3;
_9.0 = _12 * _1;
Call(_3 = fn16(_2, _1, _9, _21.1, _21.1, _20.fld4.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = [117043784716367716312741911195526872148_i128,74517421435002157008976983134994097937_i128,10581789546256240399642523138505583029_i128];
_19 = (_14, _20.fld4.2, _14);
_2 = (-5684_i16) as i64;
_22 = 8329_u16 as f64;
_22 = -_20.fld6;
_19.0 = _19.2 ^ _19.2;
_18 = [90004303654739782877301053960140387089_i128,(-56560694691859542981382275853247171841_i128),(-2121763099834347940192343041383470172_i128)];
_21.3.1 = _21.3.2 - _21.3.2;
_21.3.3 = _20.fld4.3.3;
_15 = _9.0;
_7 = _6;
_21.0 = _20.fld4.3.0.0 as i8;
_26 = [_20.fld4.3.0.0,_20.fld4.3.0.0,_20.fld4.3.0.0,_20.fld4.3.0.0,_21.3.0.0,_21.3.0.0,_20.fld4.4];
_23 = [_21.3.1,_21.3.2];
_25.0.0 = _3.0 ^ _1;
_10.0 = !_1;
_20.fld3 = _21.1 * _20.fld4.1;
_11 = [_21.1,_20.fld3];
_3.0 = _22 as i64;
_20.fld4.3.0 = (_20.fld4.4,);
_25.0 = (_10.0,);
_18 = [(-44654008439790253212124365615664881858_i128),(-93419392327798449243128482057871468830_i128),(-86840630417828539854557433158482101688_i128)];
_30 = !478557052_i32;
_10 = (_15,);
_20.fld4.4 = !_21.4;
_20.fld4.1 = _20.fld3;
match _14 {
0 => bb1,
1 => bb2,
2 => bb6,
14240102708866771034 => bb10,
_ => bb5
}
}
bb10 = {
_18 = [(-27356039807465436568631226157974892878_i128),38341103613540629576004323084713161686_i128,(-94423562397989608367976793372347964505_i128)];
_24 = _22 + _20.fld6;
_20.fld4.3.2 = _14 as u32;
_21 = _20.fld4;
_20.fld4.3.0.0 = _21.3.0.0;
_20.fld4.3.0 = (_21.4,);
_21.3.3 = _20.fld4.3.3;
_9 = (_25.0.0,);
_20.fld4.3.3 = _21.3.3;
_21.3 = (_20.fld4.3.0, _20.fld4.3.2, _20.fld4.3.1, _20.fld4.3.3);
_14 = _21.2 as u64;
RET = false as i8;
_20.fld6 = _24;
_10.0 = _9.0 >> _25.0.0;
_9 = (_1,);
_10 = (_9.0,);
_24 = _21.1 as f64;
_20.fld3 = RET | _20.fld4.1;
_2 = _25.0.0;
_35 = [_20.fld4.3.2,_21.3.2];
match _19.2 {
0 => bb1,
1 => bb6,
14240102708866771034 => bb11,
_ => bb9
}
}
bb11 = {
_34.0.0 = _10.0;
_22 = -_24;
_33 = (_13, _22);
_20.fld4.0 = _20.fld3;
_32 = 3688_u16 as i64;
_25.0 = (_34.0.0,);
_21 = (_20.fld3, _20.fld4.0, _20.fld4.2, _20.fld4.3, _20.fld4.3.0.0);
_25 = _34;
_25 = (_34.0,);
_20.fld4.3.1 = !_21.3.2;
_27 = _33.1 as isize;
_34.0.0 = 245463120759786400976128764768501991972_u128 as i64;
_20.fld4.3.1 = _21.3.1 & _20.fld4.3.2;
_36 = [_2,_15,_10.0,_5,_9.0,_2];
_20.fld4.3 = _21.3;
_14 = !_19.2;
_35 = _23;
_20.fld4.3 = (_21.3.0, _21.3.1, _21.3.2, _21.3.3);
_33 = (_13, _22);
_32 = _1 ^ _10.0;
_20.fld4.1 = _21.0;
_21.3.1 = _21.3.3 as u32;
_21.3.1 = _20.fld4.3.2 | _21.3.2;
RET = _20.fld3;
_34.0.0 = _1 - _15;
_20.fld6 = (-76913255634543070134749941726988185907_i128) as f64;
_18 = [(-86985698529979382131988351418760435793_i128),(-126466242834519107373674609995136732033_i128),55220106193056957914631626218172454525_i128];
Goto(bb12)
}
bb12 = {
Call(_40 = dump_var(15_usize, 30_usize, Move(_30), 32_usize, Move(_32), 26_usize, Move(_26), 19_usize, Move(_19)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_40 = dump_var(15_usize, 13_usize, Move(_13), 10_usize, Move(_10), 18_usize, Move(_18), 36_usize, Move(_36)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_40 = dump_var(15_usize, 35_usize, Move(_35), 15_usize, Move(_15), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_40 = dump_var(15_usize, 25_usize, Move(_25), 21_usize, Move(_21), 41_usize, _41, 41_usize, _41), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i64,mut _2: i64,mut _3: (i64,),mut _4: i8,mut _5: i8,mut _6: i8) -> (i64,) {
mir! {
type RET = (i64,);
let _7: ((i64,),);
let _8: bool;
let _9: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128));
let _10: Adt51;
let _11: *const ((u8,), u32, u32, char);
let _12: f64;
let _13: ();
let _14: ();
{
_4 = _5;
RET.0 = (-83198148558616223044881302663784509471_i128) as i64;
Call(_5 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_6;
_3.0 = -_2;
RET = (_2,);
_1 = !_2;
_7.0.0 = (-1386877707_i32) as i64;
_9.1 = [203_u8,81_u8,30_u8,83_u8,82_u8,198_u8,194_u8];
_9.2.0 = [_2,_1,_2,_2,_3.0,_1];
_9.2.0 = [_2,_1,RET.0,_2,_1,_1];
_3.0 = _2;
_8 = true;
_9.0 = [_4,_4];
_6 = !_5;
RET.0 = !_1;
_3.0 = -_2;
_3 = (_1,);
_3 = (RET.0,);
_3 = (_1,);
RET.0 = '\u{7f194}' as i64;
_6 = -_4;
RET.0 = !_3.0;
_12 = 73_u8 as f64;
_8 = _3.0 != RET.0;
_3 = (_2,);
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(16_usize, 2_usize, Move(_2), 6_usize, Move(_6), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i64,mut _2: ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5])) -> ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128)) {
mir! {
type RET = ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128));
let _3: (u8,);
let _4: bool;
let _5: isize;
let _6: bool;
let _7: Adt56;
let _8: &'static ((i64,),);
let _9: f32;
let _10: i32;
let _11: [i8; 5];
let _12: usize;
let _13: u128;
let _14: *mut (u8,);
let _15: f64;
let _16: [u8; 7];
let _17: isize;
let _18: bool;
let _19: char;
let _20: *mut i8;
let _21: Adt56;
let _22: Adt50;
let _23: ();
let _24: ();
{
RET.1 = [155_u8,166_u8,244_u8,192_u8,134_u8,242_u8,224_u8];
RET.2.1 = 244_u8 as usize;
_1 = (-99_i8) as i64;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
Goto(bb1)
}
bb1 = {
RET.0 = [65_i8,115_i8];
_2.0.0 = _2.0.2;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
_3.0 = _2.0.1 as u8;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = _2.1;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.1 = _2.0.1;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.1 = !_2.0.1;
_5 = 9223372036854775807_isize << _2.0.0;
RET.2.1 = 201756417516058501506750910082311389271_u128 as usize;
RET.0 = [(-28_i8),(-55_i8)];
_2.0.1 = _1 as usize;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = _2.1;
RET.0 = [10_i8,(-112_i8)];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.0 = [44_i8,81_i8];
_2.0.1 = RET.2.1;
_2.0.1 = RET.2.1;
Call(_2 = fn18(_5, _5, _5, _5, _5, _5, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2.0 = (8672765615309340549_u64, RET.2.1, 4176630935431377238_u64);
_3 = (239_u8,);
_3.0 = _1 as u8;
_4 = true;
_10 = 624592589_i32;
_2.0.2 = _2.0.0;
_2.1 = RET.2.0;
RET.2.1 = _2.0.1 | _2.0.1;
_2.0.1 = RET.2.1;
match _2.0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
8672765615309340549 => bb7,
_ => bb6
}
}
bb3 = {
RET.0 = [65_i8,115_i8];
_2.0.0 = _2.0.2;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
_3.0 = _2.0.1 as u8;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = _2.1;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.1 = _2.0.1;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.1 = !_2.0.1;
_5 = 9223372036854775807_isize << _2.0.0;
RET.2.1 = 201756417516058501506750910082311389271_u128 as usize;
RET.0 = [(-28_i8),(-55_i8)];
_2.0.1 = _1 as usize;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = _2.1;
RET.0 = [10_i8,(-112_i8)];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.0 = [44_i8,81_i8];
_2.0.1 = RET.2.1;
_2.0.1 = RET.2.1;
Call(_2 = fn18(_5, _5, _5, _5, _5, _5, RET.0), ReturnTo(bb2), UnwindUnreachable())
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
_10 = -(-1805494492_i32);
_5 = (-9223372036854775808_isize) ^ 71_isize;
_9 = 110_i8 as f32;
_6 = !_4;
_5 = 76_i8 as isize;
RET.0 = [(-5_i8),48_i8];
_4 = _10 >= _10;
_3 = (127_u8,);
_2.0.0 = 124620822_u32 as u64;
_5 = RET.2.1 as isize;
_1 = 8892951888490331828_i64;
RET.2.2 = core::ptr::addr_of_mut!(_13);
_3 = (41_u8,);
_9 = _1 as f32;
_11 = [(-55_i8),57_i8,53_i8,(-98_i8),(-50_i8)];
_2.0.0 = _4 as u64;
_2.0 = (1666207512971004551_u64, RET.2.1, 15716442990101154946_u64);
_2.0.2 = 210487967150137434719441645887340416420_u128 as u64;
_1 = (-7410642182116212821_i64) | 2270190143196637919_i64;
_1 = -(-1430075920803456712_i64);
Goto(bb8)
}
bb8 = {
RET.2.0 = [_1,_1,_1,_1,_1,_1];
_4 = !_6;
_14 = core::ptr::addr_of_mut!(_3);
RET.1 = [_3.0,(*_14).0,(*_14).0,_3.0,(*_14).0,(*_14).0,(*_14).0];
RET.1 = [_3.0,(*_14).0,(*_14).0,(*_14).0,(*_14).0,(*_14).0,(*_14).0];
(*_14) = (214_u8,);
_12 = RET.2.1;
RET.2.0 = _2.1;
_2.3 = [41_i8,93_i8,(-86_i8),(-34_i8),(-50_i8)];
_5 = '\u{756a}' as isize;
_13 = !70216922678370628284650255492948233545_u128;
_4 = _1 > _1;
_13 = 142408554906947570026795342338890872349_u128;
_4 = !_6;
RET.2.1 = !_12;
_5 = -(-9223372036854775808_isize);
RET.0 = [111_i8,(-109_i8)];
_3 = (2_u8,);
RET.1 = [_3.0,_3.0,_3.0,(*_14).0,(*_14).0,(*_14).0,(*_14).0];
_9 = _5 as f32;
_11 = [(-90_i8),120_i8,(-38_i8),(-4_i8),109_i8];
RET.2.1 = 3640488661_u32 as usize;
_2.0.2 = !_2.0.0;
_2.3 = [101_i8,34_i8,(-8_i8),(-78_i8),(-30_i8)];
_19 = '\u{105e0}';
_11 = [10_i8,98_i8,9_i8,(-11_i8),(-108_i8)];
match _3.0 {
0 => bb9,
1 => bb10,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
2 => bb17,
_ => bb16
}
}
bb9 = {
_10 = -(-1805494492_i32);
_5 = (-9223372036854775808_isize) ^ 71_isize;
_9 = 110_i8 as f32;
_6 = !_4;
_5 = 76_i8 as isize;
RET.0 = [(-5_i8),48_i8];
_4 = _10 >= _10;
_3 = (127_u8,);
_2.0.0 = 124620822_u32 as u64;
_5 = RET.2.1 as isize;
_1 = 8892951888490331828_i64;
RET.2.2 = core::ptr::addr_of_mut!(_13);
_3 = (41_u8,);
_9 = _1 as f32;
_11 = [(-55_i8),57_i8,53_i8,(-98_i8),(-50_i8)];
_2.0.0 = _4 as u64;
_2.0 = (1666207512971004551_u64, RET.2.1, 15716442990101154946_u64);
_2.0.2 = 210487967150137434719441645887340416420_u128 as u64;
_1 = (-7410642182116212821_i64) | 2270190143196637919_i64;
_1 = -(-1430075920803456712_i64);
Goto(bb8)
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
RET.0 = [65_i8,115_i8];
_2.0.0 = _2.0.2;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
_3.0 = _2.0.1 as u8;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = _2.1;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.1 = _2.0.1;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.1 = !_2.0.1;
_5 = 9223372036854775807_isize << _2.0.0;
RET.2.1 = 201756417516058501506750910082311389271_u128 as usize;
RET.0 = [(-28_i8),(-55_i8)];
_2.0.1 = _1 as usize;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = _2.1;
RET.0 = [10_i8,(-112_i8)];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.0 = [44_i8,81_i8];
_2.0.1 = RET.2.1;
_2.0.1 = RET.2.1;
Call(_2 = fn18(_5, _5, _5, _5, _5, _5, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_2.0 = (8672765615309340549_u64, RET.2.1, 4176630935431377238_u64);
_3 = (239_u8,);
_3.0 = _1 as u8;
_4 = true;
_10 = 624592589_i32;
_2.0.2 = _2.0.0;
_2.1 = RET.2.0;
RET.2.1 = _2.0.1 | _2.0.1;
_2.0.1 = RET.2.1;
match _2.0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
8672765615309340549 => bb7,
_ => bb6
}
}
bb15 = {
RET.0 = [65_i8,115_i8];
_2.0.0 = _2.0.2;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
_3.0 = _2.0.1 as u8;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = _2.1;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.1 = _2.0.1;
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.1 = !_2.0.1;
_5 = 9223372036854775807_isize << _2.0.0;
RET.2.1 = 201756417516058501506750910082311389271_u128 as usize;
RET.0 = [(-28_i8),(-55_i8)];
_2.0.1 = _1 as usize;
RET.2.0 = [_1,_1,_1,_1,_1,_1];
RET.2.0 = _2.1;
RET.0 = [10_i8,(-112_i8)];
RET.1 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET.0 = [44_i8,81_i8];
_2.0.1 = RET.2.1;
_2.0.1 = RET.2.1;
Call(_2 = fn18(_5, _5, _5, _5, _5, _5, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
Goto(bb18)
}
bb18 = {
Call(_23 = dump_var(17_usize, 12_usize, Move(_12), 4_usize, Move(_4), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_23 = dump_var(17_usize, 3_usize, Move(_3), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: [i8; 2]) -> ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5]) {
mir! {
type RET = ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5]);
let _8: (u8,);
let _9: (i8, i8, usize, ((u8,), u32, u32, char), u8);
let _10: ();
let _11: ();
{
RET.0.0 = _3 as u64;
RET.1 = [(-5030267200282369573_i64),76244624940602043_i64,6275086282526363769_i64,(-8912404323066271820_i64),(-7061299655087936652_i64),12768859583837224_i64];
_7 = [(-72_i8),(-34_i8)];
_6 = _2;
RET.0.1 = 6823893353227391256_usize;
Goto(bb1)
}
bb1 = {
RET.1 = [7233179686041639991_i64,6871055090056973462_i64,4094176754269615111_i64,(-6234024927395215914_i64),(-1461604913238010958_i64),5498365843889309435_i64];
_8 = (172_u8,);
Goto(bb2)
}
bb2 = {
_8.0 = !187_u8;
RET.0.2 = RET.0.0;
_4 = _6;
_9.1 = 55_i8;
_9.1 = -87_i8;
_9.3.0.0 = _1 as u8;
RET.3 = [_9.1,_9.1,_9.1,_9.1,_9.1];
RET.3 = [_9.1,_9.1,_9.1,_9.1,_9.1];
_9.3 = (_8, 3434133443_u32, 1969207445_u32, '\u{39df6}');
RET.2 = core::ptr::addr_of!(_9.3);
_9.0 = _9.1;
RET.0.0 = RET.0.2 + RET.0.2;
_9.0 = -_9.1;
RET.0.1 = 6_usize;
_9.2 = 249252677020996004577351396413887379627_u128 as usize;
_5 = 6693307534455208048_i64 as isize;
_7 = [_9.0,_9.1];
_9.1 = 6599056864672155900_i64 as i8;
RET.1 = [(-2676450211983007764_i64),(-3432951565846763109_i64),(-5050025912770508584_i64),2761632366301950280_i64,(-4610744823349496610_i64),4081776730219651422_i64];
_9.3.2 = !_9.3.1;
_5 = !_2;
RET.3 = [_9.0,_9.1,_9.1,_9.1,_9.1];
RET.0.0 = !RET.0.2;
Goto(bb3)
}
bb3 = {
Call(_10 = dump_var(18_usize, 6_usize, Move(_6), 7_usize, Move(_7), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i8,mut _2: Adt47,mut _3: (u64, usize, u64),mut _4: Adt47,mut _5: Adt47,mut _6: Adt47,mut _7: isize,mut _8: (isize, f64),mut _9: isize,mut _10: i64,mut _11: (isize, f64),mut _12: i8) -> (u64, usize, u64) {
mir! {
type RET = (u64, usize, u64);
let _13: i16;
let _14: ();
let _15: ();
{
RET = (_3.0, _3.1, _3.2);
place!(Field::<(isize, f64)>(Variant(_4, 2), 0)).1 = _11.1 - _8.1;
_4 = _2;
_9 = Field::<(isize, f64)>(Variant(_6, 2), 0).0 << _1;
place!(Field::<(isize, f64)>(Variant(_2, 2), 0)).0 = -_9;
RET.2 = _3.0;
RET.1 = _3.1;
_3.0 = RET.2;
RET.1 = _3.1 << RET.0;
RET.1 = (-142460074708695701403948797317396100854_i128) as usize;
_8 = (_9, _11.1);
place!(Field::<(isize, f64)>(Variant(_5, 2), 0)) = _11;
_9 = Field::<(isize, f64)>(Variant(_5, 2), 0).0 + Field::<(isize, f64)>(Variant(_2, 2), 0).0;
_8 = (_9, _11.1);
RET.2 = _3.0;
_3 = (RET.0, RET.1, RET.2);
RET.2 = '\u{fd4d8}' as u64;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(19_usize, 7_usize, Move(_7), 12_usize, Move(_12), 1_usize, Move(_1), 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(159860405558781609704061079778350894142_i128), std::hint::black_box((-1422722155_i32)), std::hint::black_box(62_u8), std::hint::black_box(65_i8));
                
            }
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: ((u8,), u32, u32, char),
fld1: (isize, f64),

},
Variant1{
fld0: *const u16,
fld1: (usize,),
fld2: ((i64,),),
fld3: u32,
fld4: *mut i8,
fld5: [u32; 2],

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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const u64,
fld1: char,
fld2: [i8; 2],
fld3: i8,
fld4: ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5]),
fld5: *mut *mut i8,
fld6: [i128; 3],
fld7: (i8, i8, usize, ((u8,), u32, u32, char), u8),

},
Variant1{
fld0: usize,
fld1: [u8; 4],
fld2: u16,

},
Variant2{
fld0: (isize, f64),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: [u8; 7],
fld1: (isize, f64),
fld2: ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5]),
fld3: i8,
fld4: (usize,),
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: i32,
fld1: ((i64,),),
fld2: *mut *mut i8,
fld3: *const u64,

},
Variant1{
fld0: [u8; 7],
fld1: f32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u16,
fld1: u8,

},
Variant1{
fld0: u128,
fld1: *const ((u8,), u32, u32, char),
fld2: ((u8,), u32, u32, char),
fld3: i8,
fld4: [i8; 2],
fld5: i32,

},
Variant2{
fld0: [u32; 2],
fld1: ((i64,),),
fld2: ((u8,), u32, u32, char),

},
Variant3{
fld0: (u64, usize, u64),
fld1: ((i64,),),
fld2: f64,
fld3: u16,
fld4: [u8; 7],
fld5: *mut i8,
fld6: *const u16,
fld7: *mut (isize, f64),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (i8, i8, usize, ((u8,), u32, u32, char), u8),
fld1: i32,

},
Variant1{
fld0: Adt47,
fld1: *mut u128,
fld2: Adt49,
fld3: [u8; 4],
fld4: i16,
fld5: *mut (isize, f64),

},
Variant2{
fld0: [u32; 2],
fld1: u128,
fld2: isize,
fld3: (i64,),
fld4: Adt47,
fld5: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128)),
fld6: *mut (u8,),
fld7: *mut *mut i8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u64,
fld1: char,
fld2: Adt47,
fld3: *mut u128,

},
Variant1{
fld0: ((u8,), u32, u32, char),
fld1: Adt51,
fld2: isize,
fld3: *const ((u8,), u32, u32, char),
fld4: *const u16,
fld5: ((i64,),),
fld6: u16,
fld7: i128,

},
Variant2{
fld0: Adt51,
fld1: [i8; 2],
fld2: *mut (u8,),
fld3: i8,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: usize,
fld1: Adt48,

},
Variant1{
fld0: *mut (isize, f64),
fld1: u8,
fld2: *mut *mut i8,
fld3: f32,
fld4: Adt49,
fld5: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128)),

},
Variant2{
fld0: *mut (u8,),
fld1: char,
fld2: [i128; 3],
fld3: [i8; 5],
fld4: (u8,),
fld5: (isize, f64),

},
Variant3{
fld0: bool,
fld1: Adt50,
fld2: ((i64,),),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: (i64,),
fld1: ((u64, usize, u64), [i64; 6], *const ((u8,), u32, u32, char), [i8; 5]),
fld2: (i8, i8, usize, ((u8,), u32, u32, char), u8),

},
Variant1{
fld0: f64,
fld1: (u64, usize, u64),
fld2: *const u16,
fld3: *mut u128,
fld4: [u8; 7],
fld5: i32,

},
Variant2{
fld0: bool,
fld1: f64,
fld2: *const u16,
fld3: Adt53,
fld4: [i8; 5],
fld5: i32,

},
Variant3{
fld0: [i8; 2],
fld1: *mut i8,
fld2: [i64; 6],
fld3: (i64,),
fld4: f32,
fld5: [char; 5],

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ((i64,),),
fld1: u32,
fld2: *mut *mut i8,
fld3: f32,

},
Variant1{
fld0: [i8; 2],
fld1: Adt47,
fld2: u32,
fld3: i8,
fld4: Adt49,
fld5: u128,
fld6: Adt50,
fld7: [i128; 3],

},
Variant2{
fld0: f64,
fld1: [u8; 7],
fld2: Adt49,
fld3: [i8; 5],
fld4: Adt47,

},
Variant3{
fld0: *mut u128,
fld1: (u64, usize, u64),
fld2: isize,
fld3: i8,
fld4: [i8; 2],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt50,
fld1: ([i64; 6], usize, *mut u128),

},
Variant1{
fld0: (isize, f64),
fld1: (i64,),
fld2: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128)),
fld3: *mut (isize, f64),

},
Variant2{
fld0: ((i64,),),
fld1: i64,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [char; 5],
fld1: Adt52,
fld2: Adt47,
fld3: [i64; 6],
fld4: ([i8; 2], [u8; 7], ([i64; 6], usize, *mut u128)),
fld5: Adt48,
fld6: *const ((u8,), u32, u32, char),
fld7: (i8, i8, usize, ((u8,), u32, u32, char), u8),

},
Variant1{
fld0: [i64; 6],
fld1: ([i64; 6], usize, *mut u128),
fld2: isize,
fld3: [i8; 5],
fld4: [u8; 4],
fld5: [u8; 7],
fld6: Adt48,
fld7: Adt46,

},
Variant2{
fld0: usize,
fld1: u64,

},
Variant3{
fld0: bool,
fld1: i128,
fld2: u8,
fld3: Adt51,
fld4: [u8; 4],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: Adt53,
fld1: [u32; 2],
fld2: (u64, usize, u64),
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: *mut (isize, f64),
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt58,

},
Variant1{
fld0: Adt51,
fld1: i32,

},
Variant2{
fld0: Adt49,
fld1: [u8; 7],
fld2: u64,
fld3: [char; 5],

},
Variant3{
fld0: [char; 5],
fld1: [i8; 2],
fld2: (isize, f64),
fld3: (u64, usize, u64),
fld4: *mut (u8,),
fld5: u128,
fld6: Adt50,

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt61{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt61 {
fld0: *mut *mut i8,
fld1: Adt56,
fld2: Adt60,
fld3: i8,
fld4: (i8, i8, usize, ((u8,), u32, u32, char), u8),
fld5: Adt46,
fld6: f64,
fld7: Adt47,
}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt62::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: Adt55,
fld1: u128,
fld2: [i128; 3],
fld3: ((i64,),),
fld4: Adt56,
fld5: [i8; 5],
fld6: Adt46,

},
Variant1{
fld0: Adt56,
fld1: *mut (u8,),
fld2: i32,

}}

