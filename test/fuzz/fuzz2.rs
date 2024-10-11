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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> Adt45 {
mir! {
type RET = Adt45;
let _15: Adt42;
let _16: (isize, [bool; 7], i16, u128);
let _17: Adt45;
let _18: bool;
let _19: [char; 5];
let _20: u64;
let _21: f64;
let _22: isize;
let _23: char;
let _24: ([i32; 8], i128, (isize, [bool; 7], i16, u128), [u8; 4]);
let _25: [u64; 2];
let _26: (*mut i8,);
let _27: f32;
let _28: Adt49;
let _29: char;
let _30: char;
let _31: Adt43;
let _32: char;
let _33: usize;
let _34: *mut i32;
let _35: bool;
let _36: [i16; 3];
let _37: isize;
let _38: Adt47;
let _39: [i32; 8];
let _40: Adt44;
let _41: char;
let _42: [i16; 5];
let _43: ();
let _44: ();
{
_2 = '\u{2f8e2}';
_8 = 11407080333376617386120416247545265587_i128 | 50265112297462044474307105855792800297_i128;
_15.fld3 = 82_i8;
_11 = 24675_u16 | 58599_u16;
_5 = -(-26481_i16);
_16.2 = -_5;
_10 = 9_u8 * 168_u8;
_15.fld0 = true;
_5 = _16.2;
_4 = _15.fld3;
_13 = 2707097033278071026_u64;
_12 = 1256993907_u32 + 3670275004_u32;
_5 = !_16.2;
_9 = _11 as usize;
_6 = !2133351490_i32;
_15.fld6.0 = core::ptr::addr_of_mut!(_4);
_3 = 9223372036854775807_isize;
_15.fld5 = -_6;
_19 = [_2,_2,_2,_2,_2];
_1 = !_15.fld0;
_14 = 66614899778325033482610945738103582369_u128;
_6 = _15.fld5 >> _8;
_12 = 759374195_u32 * 3362329589_u32;
_15.fld4 = [_2,_2,_2,_2,_2];
Goto(bb1)
}
bb1 = {
_4 = !_15.fld3;
_20 = !_13;
_21 = _9 as f64;
_1 = _15.fld0;
_9 = 7_usize << _5;
_7 = (-8412084336836853206_i64);
Goto(bb2)
}
bb2 = {
_16.2 = _5;
_16.2 = _5;
_4 = _15.fld3;
_15.fld5 = _6;
RET = Adt45::Variant2 { fld0: _15.fld5,fld1: _12,fld2: _3,fld3: _21 };
_15.fld4 = _19;
_5 = -_16.2;
_4 = !_15.fld3;
_21 = -Field::<f64>(Variant(RET, 2), 3);
_14 = !173893209106349812051023939797606557517_u128;
_15.fld3 = _4;
_22 = _11 as isize;
_17 = Move(RET);
_2 = '\u{e3afd}';
_16.3 = !_14;
_21 = Field::<f64>(Variant(_17, 2), 3);
place!(Field::<isize>(Variant(_17, 2), 2)) = !_22;
_15.fld0 = !_1;
_15.fld0 = !_1;
place!(Field::<f64>(Variant(_17, 2), 3)) = _6 as f64;
_15.fld1 = Adt39::Variant0 { fld0: _15.fld5 };
_2 = '\u{1032c2}';
_16.0 = !Field::<isize>(Variant(_17, 2), 2);
Goto(bb3)
}
bb3 = {
_16.1 = [_15.fld0,_1,_15.fld0,_15.fld0,_1,_15.fld0,_15.fld0];
_5 = _16.2;
_18 = !_1;
_24.2.3 = !_14;
_24.2.1 = [_18,_15.fld0,_15.fld0,_15.fld0,_1,_15.fld0,_15.fld0];
_15.fld6.0 = core::ptr::addr_of_mut!(_4);
_24.2.1 = [_18,_18,_15.fld0,_1,_15.fld0,_15.fld0,_1];
_5 = _16.2 << _10;
Goto(bb4)
}
bb4 = {
_14 = _16.3 + _24.2.3;
_15.fld3 = _8 as i8;
RET = Move(_17);
_24.0 = [_15.fld5,Field::<i32>(Variant(RET, 2), 0),Field::<i32>(Variant(_15.fld1, 0), 0),_15.fld5,_15.fld5,Field::<i32>(Variant(_15.fld1, 0), 0),_6,Field::<i32>(Variant(_15.fld1, 0), 0)];
_22 = _7 as isize;
_7 = -(-3691158127634505513_i64);
_19 = _15.fld4;
_26.0 = _15.fld6.0;
_24.2 = (_16.0, _16.1, _5, _14);
place!(Field::<i32>(Variant(RET, 2), 0)) = _15.fld5 - _15.fld5;
_22 = _16.0;
_15.fld5 = _6;
_15.fld4 = [_2,_2,_2,_2,_2];
_25 = [_13,_13];
_2 = '\u{a036b}';
_16.3 = _14;
_31.fld2.2.3 = _2 as u128;
_24.2 = (_22, _16.1, _16.2, _14);
_12 = !Field::<u32>(Variant(RET, 2), 1);
_21 = Field::<f64>(Variant(RET, 2), 3);
_31.fld2.2.0 = _8 as isize;
_17 = Move(RET);
_17 = Adt45::Variant2 { fld0: _6,fld1: _12,fld2: _31.fld2.2.0,fld3: _21 };
_6 = Field::<i32>(Variant(_17, 2), 0);
Goto(bb5)
}
bb5 = {
_24.2 = (_31.fld2.2.0, _16.1, _16.2, _14);
_22 = _3 | _31.fld2.2.0;
_31.fld2.1 = _8;
RET = Move(_17);
_9 = _16.3 as usize;
SetDiscriminant(RET, 0);
_31.fld2.2.2 = _5;
_29 = _2;
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.2 = _24.2;
_32 = _29;
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.2.1 = [_1,_18,_1,_18,_1,_1,_1];
Call(place!(Field::<i8>(Variant(RET, 0), 3)) = fn1(_24.2.0, _31.fld2.1, _24.2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23 = _2;
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.0 = [_6,_6,_15.fld5,_15.fld5,_15.fld5,_6,_6,Field::<i32>(Variant(_15.fld1, 0), 0)];
_30 = _2;
place!(Field::<i128>(Variant(RET, 0), 4)) = !_8;
place!(Field::<i128>(Variant(RET, 0), 4)) = _31.fld2.1 * _31.fld2.1;
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.3 = [_10,_10,_10,_10];
place!(Field::<i64>(Variant(RET, 0), 6)) = !_7;
SetDiscriminant(_15.fld1, 0);
_24.3 = [_10,_10,_10,_10];
_31.fld2 = (_24.0, Field::<i128>(Variant(RET, 0), 4), _24.2, _24.3);
_33 = _9 * _9;
_31.fld2.2.3 = _14;
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.1 = !Field::<i128>(Variant(RET, 0), 4);
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.2.3 = Field::<i128>(Variant(RET, 0), 4) as u128;
_17 = Adt45::Variant2 { fld0: _6,fld1: _12,fld2: _24.2.0,fld3: _21 };
place!(Field::<i8>(Variant(RET, 0), 3)) = _18 as i8;
_24.1 = Field::<i128>(Variant(RET, 0), 4);
_18 = !_1;
place!(Field::<[bool; 7]>(Variant(RET, 0), 0)) = [_18,_15.fld0,_15.fld0,_1,_18,_1,_18];
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld0 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_17, 2), 0)));
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld1 = [_5,_24.2.2,_31.fld2.2.2];
place!(Field::<i8>(Variant(RET, 0), 3)) = _1 as i8;
_15.fld0 = _1;
_24 = (Field::<Adt43>(Variant(RET, 0), 1).fld2.0, _31.fld2.1, Field::<Adt43>(Variant(RET, 0), 1).fld2.2, Field::<Adt43>(Variant(RET, 0), 1).fld2.3);
_37 = Field::<isize>(Variant(_17, 2), 2) - Field::<Adt43>(Variant(RET, 0), 1).fld2.2.0;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2707097033278071026 => bb8,
_ => bb7
}
}
bb7 = {
_14 = _16.3 + _24.2.3;
_15.fld3 = _8 as i8;
RET = Move(_17);
_24.0 = [_15.fld5,Field::<i32>(Variant(RET, 2), 0),Field::<i32>(Variant(_15.fld1, 0), 0),_15.fld5,_15.fld5,Field::<i32>(Variant(_15.fld1, 0), 0),_6,Field::<i32>(Variant(_15.fld1, 0), 0)];
_22 = _7 as isize;
_7 = -(-3691158127634505513_i64);
_19 = _15.fld4;
_26.0 = _15.fld6.0;
_24.2 = (_16.0, _16.1, _5, _14);
place!(Field::<i32>(Variant(RET, 2), 0)) = _15.fld5 - _15.fld5;
_22 = _16.0;
_15.fld5 = _6;
_15.fld4 = [_2,_2,_2,_2,_2];
_25 = [_13,_13];
_2 = '\u{a036b}';
_16.3 = _14;
_31.fld2.2.3 = _2 as u128;
_24.2 = (_22, _16.1, _16.2, _14);
_12 = !Field::<u32>(Variant(RET, 2), 1);
_21 = Field::<f64>(Variant(RET, 2), 3);
_31.fld2.2.0 = _8 as isize;
_17 = Move(RET);
_17 = Adt45::Variant2 { fld0: _6,fld1: _12,fld2: _31.fld2.2.0,fld3: _21 };
_6 = Field::<i32>(Variant(_17, 2), 0);
Goto(bb5)
}
bb8 = {
_31.fld2.2 = (_22, Field::<[bool; 7]>(Variant(RET, 0), 0), _16.2, _16.3);
_8 = Field::<Adt43>(Variant(RET, 0), 1).fld2.1 >> _6;
place!(Field::<i128>(Variant(RET, 0), 4)) = Field::<Adt43>(Variant(RET, 0), 1).fld2.1;
_14 = _1 as u128;
SetDiscriminant(_17, 1);
place!(Field::<Adt43>(Variant(RET, 0), 1)).fld2.2.2 = _5 * _5;
_23 = _29;
place!(Field::<[bool; 4]>(Variant(RET, 0), 5)) = [_18,_15.fld0,_1,_1];
_15.fld0 = Field::<Adt43>(Variant(RET, 0), 1).fld2.2.2 == Field::<Adt43>(Variant(RET, 0), 1).fld2.2.2;
_15.fld2 = Adt38::Variant1 { fld0: Field::<[bool; 4]>(Variant(RET, 0), 5) };
_3 = _31.fld2.2.0;
_22 = _31.fld2.2.0;
place!(Field::<u32>(Variant(_17, 1), 0)) = _12;
RET = Adt45::Variant2 { fld0: _6,fld1: _12,fld2: _22,fld3: _21 };
_24.2 = _31.fld2.2;
_18 = _15.fld0 ^ _15.fld0;
Goto(bb9)
}
bb9 = {
Call(_43 = dump_var(0_usize, 5_usize, Move(_5), 1_usize, Move(_1), 23_usize, Move(_23), 32_usize, Move(_32)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_43 = dump_var(0_usize, 37_usize, Move(_37), 18_usize, Move(_18), 24_usize, Move(_24), 7_usize, Move(_7)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_43 = dump_var(0_usize, 30_usize, Move(_30), 3_usize, Move(_3), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_43 = dump_var(0_usize, 12_usize, Move(_12), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: i128,mut _3: (isize, [bool; 7], i16, u128)) -> i8 {
mir! {
type RET = i8;
let _4: isize;
let _5: Adt47;
let _6: char;
let _7: isize;
let _8: [i32; 8];
let _9: [i32; 8];
let _10: [u32; 1];
let _11: u64;
let _12: [i128; 1];
let _13: char;
let _14: char;
let _15: ([i32; 8], i128, (isize, [bool; 7], i16, u128), [u8; 4]);
let _16: Adt48;
let _17: char;
let _18: [u64; 8];
let _19: (*mut i8,);
let _20: [u32; 1];
let _21: Adt47;
let _22: isize;
let _23: *mut i128;
let _24: ();
let _25: ();
{
_1 = (-83_i8) as isize;
_2 = 104049455376576057656329849955419820953_i128;
_2 = 92639147947232368443652736140677915730_i128;
RET = true as i8;
_3.3 = 143933506692254604626979273220638919206_u128 - 45135415988686321278803883286454531720_u128;
_4 = _1 >> _3.0;
_2 = _1 as i128;
_1 = !_4;
_3.3 = !20345847734667523118821301532786019517_u128;
_3.3 = 221012330636576150949575699559674531093_u128;
RET = -1_i8;
_1 = -_4;
RET = _3.2 as i8;
RET = -45_i8;
match _3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
221012330636576150949575699559674531093 => bb9,
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
RET = (-48_i8);
Call(_2 = fn2(_1, _4, _3.0, RET, _4, _3.3, _3.0, _3.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = 2047001112_i32 as isize;
_3.3 = !4631919579202299002665228585634376136_u128;
_4 = _3.0 ^ _1;
RET = 65_i8;
_2 = (-123230616539774885540794617935310571044_i128) - 118793696600584461643068562014907139346_i128;
RET = 77_i8 | 14_i8;
Goto(bb11)
}
bb11 = {
_3.1 = [false,false,true,false,true,true,false];
_3.2 = (-148_i16);
_3.3 = !285065191414016638289444451087786256098_u128;
_3.2 = (-5376_i16) | (-30438_i16);
_3.1 = [false,false,true,true,false,false,true];
_3.3 = 298129830775499804064822183402104288976_u128 >> _4;
_2 = (-18859186708221252952443957731137137706_i128);
_7 = _1 & _4;
_3.2 = 8042_i16 << _3.3;
_4 = 2036461136_i32 as isize;
_4 = 3979109761_u32 as isize;
_1 = _2 as isize;
_3.0 = 1764065936_i32 as isize;
_4 = _3.0 & _7;
RET = 118_i8 | (-60_i8);
_3.2 = (-4470_i16);
_8 = [(-2101516729_i32),(-1903594071_i32),1032562491_i32,(-324553058_i32),(-1339037848_i32),1258853023_i32,(-1254879802_i32),(-645017008_i32)];
_3.0 = _1;
_3.3 = 299168532659192257935268534082278573171_u128;
_1 = _7 & _4;
Call(_3.3 = core::intrinsics::bswap(16014756556251149770900370265136192981_u128), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10 = [576658516_u32];
Goto(bb13)
}
bb13 = {
_8 = [(-1579504960_i32),(-1635324976_i32),476649962_i32,1206549281_i32,(-357635696_i32),836021817_i32,1110792933_i32,(-127849638_i32)];
_6 = '\u{f8b85}';
RET = 18_i8 >> _1;
RET = (-58_i8);
RET = -(-29_i8);
_9 = [(-1892075537_i32),478732621_i32,(-634365787_i32),1602642873_i32,(-1425078348_i32),(-1426392419_i32),145982894_i32,(-449560589_i32)];
_15.3 = [12_u8,191_u8,253_u8,133_u8];
_15.2.0 = false as isize;
_15.0 = [(-1676191473_i32),(-73593274_i32),206440860_i32,(-1507406011_i32),(-375310234_i32),(-856368268_i32),(-896337681_i32),(-2105818432_i32)];
_14 = _6;
_15.2.1 = _3.1;
_15.2.0 = _1;
RET = _3.2 as i8;
_17 = _6;
_18 = [473293597034132159_u64,5167876907777413449_u64,15645547942782394501_u64,11234960947038522771_u64,17612421762710981389_u64,10938434170745074774_u64,10225900466832500319_u64,11226826562156041131_u64];
_15.2.3 = _3.3;
_15.2.3 = _7 as u128;
_14 = _6;
_15.2.2 = _3.2;
_11 = 2092480937_u32 as u64;
Goto(bb14)
}
bb14 = {
_12 = [_2];
_20 = _10;
_6 = _17;
_3.3 = _15.2.3;
_18 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = _15.2.3 as u64;
_17 = _14;
_15.2.1 = [true,true,true,true,true,true,true];
_15.0 = [1711411128_i32,1902485392_i32,(-1031141934_i32),(-1105144523_i32),2120878003_i32,1689663103_i32,1862943337_i32,1424720946_i32];
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(1_usize, 2_usize, Move(_2), 17_usize, Move(_17), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(1_usize, 18_usize, Move(_18), 11_usize, Move(_11), 9_usize, Move(_9), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}

#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i8,mut _5: isize,mut _6: u128,mut _7: isize,mut _8: isize) -> i128 {
mir! {
type RET = i128;
let _9: i32;
let _10: *mut i32;
let _11: u64;
let _12: f64;
let _13: Adt50;
let _14: f64;
let _15: [i128; 1];
let _16: [u32; 1];
let _17: char;
let _18: [i128; 1];
let _19: usize;
let _20: i64;
let _21: Adt45;
let _22: Adt39;
let _23: u64;
let _24: [char; 5];
let _25: i64;
let _26: Adt48;
let _27: ();
let _28: ();
{
_1 = _2 >> _6;
_4 = 71_i8 & 74_i8;
RET = _6 as i128;
_11 = 8375999496998453650_u64 + 1821979537426269688_u64;
_4 = 14013_i16 as i8;
RET = -(-45456748351428565773250698160747866052_i128);
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
221012330636576150949575699559674531093 => bb6,
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
_2 = true as isize;
_11 = 6769890889135988609_u64;
_1 = _3 & _5;
_2 = _7;
_2 = _5 + _5;
_10 = core::ptr::addr_of_mut!(_9);
_10 = core::ptr::addr_of_mut!((*_10));
_11 = 14200423117125649752_u64 & 8353572707361190301_u64;
_7 = -_8;
_11 = true as u64;
_10 = core::ptr::addr_of_mut!(_9);
(*_10) = !(-1379042502_i32);
_7 = false as isize;
RET = (-54014063298837719852913404256248250234_i128) | 79422296228223089702586978778191098576_i128;
RET = _1 as i128;
_6 = !331439579604368003557879674698790747242_u128;
_2 = -_3;
_1 = -_3;
_4 = _2 as i8;
_12 = 3964241749658048351_i64 as f64;
_7 = !_1;
_10 = core::ptr::addr_of_mut!(_9);
_7 = -_5;
_5 = _7;
(*_10) = 37228_u16 as i32;
_1 = !_8;
_2 = (*_10) as isize;
_2 = _7 & _7;
(*_10) = !75482869_i32;
_7 = _5 << _4;
Goto(bb7)
}
bb7 = {
_3 = 112_u8 as isize;
_4 = 6_i8 & 99_i8;
_9 = 1445408366_i32;
_5 = _6 as isize;
match (*_10) {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb9,
1445408366 => bb11,
_ => bb10
}
}
bb8 = {
_2 = true as isize;
_11 = 6769890889135988609_u64;
_1 = _3 & _5;
_2 = _7;
_2 = _5 + _5;
_10 = core::ptr::addr_of_mut!(_9);
_10 = core::ptr::addr_of_mut!((*_10));
_11 = 14200423117125649752_u64 & 8353572707361190301_u64;
_7 = -_8;
_11 = true as u64;
_10 = core::ptr::addr_of_mut!(_9);
(*_10) = !(-1379042502_i32);
_7 = false as isize;
RET = (-54014063298837719852913404256248250234_i128) | 79422296228223089702586978778191098576_i128;
RET = _1 as i128;
_6 = !331439579604368003557879674698790747242_u128;
_2 = -_3;
_1 = -_3;
_4 = _2 as i8;
_12 = 3964241749658048351_i64 as f64;
_7 = !_1;
_10 = core::ptr::addr_of_mut!(_9);
_7 = -_5;
_5 = _7;
(*_10) = 37228_u16 as i32;
_1 = !_8;
_2 = (*_10) as isize;
_2 = _7 & _7;
(*_10) = !75482869_i32;
_7 = _5 << _4;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_2 = _8;
_10 = core::ptr::addr_of_mut!((*_10));
_5 = (*_10) as isize;
RET = -68729098777090700041794447816632722485_i128;
_3 = 3_usize as isize;
_7 = (*_10) as isize;
_2 = false as isize;
_6 = 51524689774638705400574795369905212989_u128 * 116293450093340493338512216221116389461_u128;
_10 = core::ptr::addr_of_mut!(_9);
(*_10) = (-1473681833_i32);
_18 = [RET];
_15 = [RET];
_20 = 1568749935157236079_i64;
_8 = -_3;
Goto(bb12)
}
bb12 = {
_1 = 102_u8 as isize;
_15 = [RET];
_19 = 7_usize;
_10 = core::ptr::addr_of_mut!((*_10));
_12 = 26622_u16 as f64;
_2 = '\u{9d4a4}' as isize;
_16 = [28509988_u32];
_14 = 39238_u16 as f64;
_5 = _1 >> _9;
_22 = Adt39::Variant0 { fld0: (*_10) };
Call(RET = core::intrinsics::transmute(_6), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = -141093276121251836250281844325270850167_i128;
_3 = !_8;
_7 = 184_u8 as isize;
_11 = 2976618151839434111_u64;
_14 = -_12;
RET = !146445080415064546395799959163798268945_i128;
_15 = _18;
_16 = [466754066_u32];
Goto(bb14)
}
bb14 = {
_17 = '\u{10e1d7}';
_22 = Adt39::Variant0 { fld0: (*_10) };
_8 = _3 << _5;
(*_10) = Field::<i32>(Variant(_22, 0), 0) - Field::<i32>(Variant(_22, 0), 0);
_24 = [_17,_17,_17,_17,_17];
(*_10) = 28704_i16 as i32;
_18 = [RET];
_23 = _11 / _11;
_19 = !3839957329456124985_usize;
_18 = [RET];
_25 = (-9844_i16) as i64;
_9 = Field::<i32>(Variant(_22, 0), 0);
place!(Field::<i32>(Variant(_22, 0), 0)) = _17 as i32;
_14 = -_12;
place!(Field::<i32>(Variant(_22, 0), 0)) = -(*_10);
_15 = [RET];
_3 = (*_10) as isize;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(2_usize, 6_usize, Move(_6), 7_usize, Move(_7), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(2_usize, 25_usize, Move(_25), 8_usize, Move(_8), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(2_usize, 18_usize, Move(_18), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{7bb8}'), std::hint::black_box(124_isize), std::hint::black_box((-34_i8)), std::hint::black_box((-29084_i16)), std::hint::black_box(1612579959_i32), std::hint::black_box((-1174914451604342603_i64)), std::hint::black_box(74431864193187347538102102117630929911_i128), std::hint::black_box(5_usize), std::hint::black_box(136_u8), std::hint::black_box(61890_u16), std::hint::black_box(2945027633_u32), std::hint::black_box(98962019719333069_u64), std::hint::black_box(281741614786473908316406863563858924974_u128));
                
            }
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt37::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: usize,
fld1: char,
fld2: [u64; 2],
fld3: i8,
fld4: [bool; 7],
fld5: [char; 5],
fld6: (*mut i8,),
fld7: [bool; 4],

},
Variant1{
fld0: i64,
fld1: char,
fld2: u32,

},
Variant2{
fld0: f32,
fld1: [bool; 7],
fld2: f64,
fld3: i16,

},
Variant3{
fld0: (*mut i8,),
fld1: i32,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt38::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: *mut i128,
fld1: (char,),

},
Variant1{
fld0: [bool; 4],

},
Variant2{
fld0: f64,
fld1: char,
fld2: f32,
fld3: [bool; 7],
fld4: i16,
fld5: u128,
fld6: (char,),

},
Variant3{
fld0: bool,
fld1: *const usize,
fld2: [bool; 7],
fld3: [i16; 3],

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: i32,

},
Variant1{
fld0: i16,
fld1: [u64; 2],
fld2: i32,
fld3: [i128; 1],

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: usize,
fld1: *mut i32,
}
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i128,
fld1: [bool; 4],
fld2: isize,
fld3: [u8; 4],
fld4: [i16; 5],
fld5: i32,
fld6: ([i32; 8], i128, (isize, [bool; 7], i16, u128), [u8; 4]),

},
Variant1{
fld0: *const usize,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: bool,
fld1: Adt39,
fld2: Adt38,
fld3: i8,
fld4: [char; 5],
fld5: i32,
fld6: (*mut i8,),
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: *mut i32,
fld1: [i16; 3],
fld2: ([i32; 8], i128, (isize, [bool; 7], i16, u128), [u8; 4]),
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: i32,
fld1: i16,
fld2: isize,
fld3: [u8; 4],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [bool; 7],
fld1: Adt43,
fld2: *mut i8,
fld3: i8,
fld4: i128,
fld5: [bool; 4],
fld6: i64,

},
Variant1{
fld0: u32,
fld1: f32,
fld2: isize,
fld3: [i16; 5],

},
Variant2{
fld0: i32,
fld1: u32,
fld2: isize,
fld3: f64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut i32,
fld1: i64,
fld2: [u64; 8],
fld3: i8,
fld4: u128,
fld5: Adt42,

},
Variant1{
fld0: [i128; 1],
fld1: Adt40,
fld2: isize,
fld3: Adt39,
fld4: [u64; 2],
fld5: Adt43,

},
Variant2{
fld0: [u8; 4],
fld1: Adt39,
fld2: *mut i8,

},
Variant3{
fld0: bool,
fld1: Adt41,
fld2: i64,
fld3: [i16; 3],
fld4: Adt45,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i64,
fld1: Adt40,
fld2: [i16; 3],
fld3: [i128; 1],

},
Variant1{
fld0: bool,
fld1: *const usize,
fld2: (*mut i8,),
fld3: [i128; 1],
fld4: [bool; 4],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt41,
fld1: char,
fld2: isize,
fld3: [u8; 4],
fld4: Adt39,
fld5: i32,
fld6: Adt42,

},
Variant1{
fld0: bool,
fld1: u64,
fld2: Adt43,
fld3: (char,),
fld4: Adt38,
fld5: f64,
fld6: Adt46,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: [u8; 4],

},
Variant1{
fld0: i128,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [char; 5],
fld1: char,
fld2: [bool; 7],

},
Variant1{
fld0: Adt47,
fld1: ([i32; 8], i128, (isize, [bool; 7], i16, u128), [u8; 4]),
fld2: u8,
fld3: [bool; 7],
fld4: Adt45,
fld5: u64,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i128,
fld1: Adt49,

},
Variant1{
fld0: [i16; 3],
fld1: [u64; 8],
fld2: [bool; 7],
fld3: f32,
fld4: [i32; 8],
fld5: [i128; 1],

},
Variant2{
fld0: Adt43,
fld1: [i16; 5],
fld2: Adt50,

},
Variant3{
fld0: bool,
fld1: Adt48,
fld2: [bool; 4],
fld3: i8,
fld4: (char,),
fld5: i32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: Adt48,
fld1: Adt37,
fld2: Adt41,
fld3: i8,
fld4: i16,
fld5: (char,),
fld6: Adt50,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: bool,
fld1: char,
fld2: *const usize,
fld3: u128,
fld4: u64,
fld5: Adt45,

},
Variant1{
fld0: [i16; 5],
fld1: u8,
fld2: [bool; 7],
fld3: i8,
fld4: [u8; 4],
fld5: [u64; 2],
fld6: [i32; 8],

},
Variant2{
fld0: f32,
fld1: Adt43,
fld2: usize,
fld3: Adt49,
fld4: (*mut i8,),
fld5: i32,
fld6: i64,

}}

