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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u128,mut _10: u8,mut _11: u16,mut _12: u32) -> [char; 6] {
mir! {
type RET = [char; 6];
let _13: Adt51;
let _14: isize;
let _15: f32;
let _16: char;
let _17: i8;
let _18: Adt50;
let _19: usize;
let _20: (i16,);
let _21: Adt42;
let _22: f64;
let _23: (i16,);
let _24: *const *mut [char; 6];
let _25: u8;
let _26: f32;
let _27: usize;
let _28: *mut [char; 6];
let _29: bool;
let _30: [i64; 1];
let _31: bool;
let _32: i64;
let _33: Adt41;
let _34: i8;
let _35: char;
let _36: char;
let _37: u8;
let _38: [u16; 4];
let _39: [char; 6];
let _40: isize;
let _41: i32;
let _42: f32;
let _43: i128;
let _44: f32;
let _45: bool;
let _46: ();
let _47: ();
{
_7 = 38_u8 as i64;
RET = ['\u{a763e}','\u{431e6}','\u{e202e}','\u{6583}','\u{57215}','\u{7cf9f}'];
Goto(bb1)
}
bb1 = {
_4 = (-1672186342_i32) as i8;
RET = ['\u{106db3}','\u{99c17}','\u{37209}','\u{c4bf5}','\u{ad6dd}','\u{48255}'];
_13.fld1 = true as i16;
_6 = -(-310196254_i32);
_3 = !9223372036854775807_isize;
_4 = _6 as i8;
_10 = !183_u8;
_11 = 55134_u16 + 5655_u16;
_1 = false;
_3 = _1 as isize;
_4 = -124_i8;
Goto(bb2)
}
bb2 = {
RET = ['\u{e3d1d}','\u{47597}','\u{f575c}','\u{d0949}','\u{29a99}','\u{d6717}'];
RET = ['\u{ecb09}','\u{7ff73}','\u{bdadb}','\u{d368f}','\u{10be31}','\u{efc3b}'];
_10 = 56_u8 - 143_u8;
Goto(bb3)
}
bb3 = {
_5 = _3 as i16;
_10 = 0_usize as u8;
_2 = '\u{e4793}';
_11 = 59363_u16;
_16 = _2;
_1 = false ^ false;
_8 = 83083294392505488683012684229809168934_i128 | 61747992909225105024248736754628658770_i128;
_12 = 13808821380171476765_u64 as u32;
_15 = _12 as f32;
_17 = _7 as i8;
_11 = !3177_u16;
_11 = 40218_u16;
Call(_11 = core::intrinsics::bswap(6090_u16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = [_16,_16,_2,_2,_16,_16];
_14 = _3 >> _11;
_13.fld0 = [_8,_8,_8];
_15 = 3166522422079882645_usize as f32;
_10 = 178_u8 >> _4;
_5 = _13.fld1 + _13.fld1;
_9 = 240389208582748965487259288865827782898_u128;
_17 = _10 as i8;
_11 = 22932_u16 << _8;
_12 = _6 as u32;
_16 = _2;
_9 = !299030487266117898338834463922218948573_u128;
_5 = _13.fld1;
_7 = !(-4865040570568086690_i64);
_14 = 17045417233805600865_u64 as isize;
_20.0 = _5 - _5;
_13.fld0 = [_8,_8,_8];
_19 = !13845618864462844489_usize;
Goto(bb5)
}
bb5 = {
_5 = _17 as i16;
_20.0 = _19 as i16;
RET = [_16,_2,_2,_2,_2,_2];
_10 = _15 as u8;
_7 = (-4925537794307816466_i64);
_20 = (_5,);
_11 = 37781_u16;
_6 = 12147622956468132554_u64 as i32;
_3 = _19 as isize;
_4 = -_17;
_23 = (_5,);
_13.fld1 = !_20.0;
_15 = _4 as f32;
_7 = !2486148284450274053_i64;
_6 = _11 as i32;
_7 = -543262864726497004_i64;
_16 = _2;
_24 = core::ptr::addr_of!(_28);
_8 = !(-148236017317921835599374360715146210483_i128);
_2 = _16;
_2 = _16;
_2 = _16;
Call(_14 = fn1(_13.fld1, _20.0, _16, _5, _1, _20.0, _4, _13.fld1, _23.0, _3, _23, _10, _17, _2, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_31 = _1;
(*_24) = core::ptr::addr_of_mut!(RET);
_30 = [_7];
_22 = _20.0 as f64;
_31 = !_1;
_3 = !_14;
(*_24) = core::ptr::addr_of_mut!((*_28));
_2 = _16;
_25 = 9783522868085495390_u64 as u8;
_11 = _7 as u16;
_24 = core::ptr::addr_of!((*_24));
_30 = [_7];
_22 = _8 as f64;
(*_24) = core::ptr::addr_of_mut!((*_28));
_11 = !10053_u16;
_23.0 = !_13.fld1;
_8 = (-162855581123838439141224370607476315585_i128) & (-162137497971078549376274127387043376144_i128);
_27 = _19;
_18 = Adt50::Variant2 { fld0: _7 };
_29 = !_31;
_20 = (_5,);
_20 = (_23.0,);
_5 = _11 as i16;
_10 = 1916537035007207660_u64 as u8;
(*_28) = [_2,_2,_2,_16,_2,_2];
Goto(bb7)
}
bb7 = {
_7 = -Field::<i64>(Variant(_18, 2), 0);
_30 = [_7];
RET = [_16,_2,_16,_16,_16,_2];
_29 = _14 == _3;
_31 = _29;
_27 = _19;
_13.fld0 = [_8,_8,_8];
_19 = !_27;
_20 = (_23.0,);
_31 = !_29;
SetDiscriminant(_18, 1);
Call(_2 = fn4(_3, (*_24), _14, _31, _29, _31, _31, _3, _3, _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_30 = [_7];
_26 = _15 * _15;
_18 = Adt50::Variant2 { fld0: _7 };
_32 = _17 as i64;
_17 = !_4;
_36 = _2;
SetDiscriminant(_18, 1);
Goto(bb9)
}
bb9 = {
_32 = _22 as i64;
RET = [_16,_16,_2,_2,_16,_16];
_23.0 = _17 as i16;
_26 = _27 as f32;
_14 = _3;
_28 = core::ptr::addr_of_mut!((*_28));
_28 = core::ptr::addr_of_mut!((*_28));
_9 = _26 as u128;
_38 = [_11,_11,_11,_11];
_13.fld0 = [_8,_8,_8];
_33 = Adt41::Variant0 { fld0: _23,fld1: _2 };
_13.fld1 = Field::<(i16,)>(Variant(_33, 0), 0).0 + _5;
_6 = _25 as i32;
_17 = _10 as i8;
_25 = _27 as u8;
_3 = _6 as isize;
Goto(bb10)
}
bb10 = {
_39 = [_16,_2,_2,_16,_16,_36];
_36 = Field::<char>(Variant(_33, 0), 1);
_37 = !_10;
_5 = _23.0 & _13.fld1;
place!(Field::<(i16,)>(Variant(_33, 0), 0)).0 = _5;
_20 = (_13.fld1,);
_29 = !_31;
_32 = _7 + _7;
_23.0 = Field::<char>(Variant(_33, 0), 1) as i16;
_1 = _29;
_20.0 = _13.fld1 << _9;
Goto(bb11)
}
bb11 = {
_38 = [_11,_11,_11,_11];
_11 = _14 as u16;
_39 = [_36,_16,_2,_2,Field::<char>(Variant(_33, 0), 1),_16];
_39 = [_36,Field::<char>(Variant(_33, 0), 1),_36,Field::<char>(Variant(_33, 0), 1),_16,_16];
_11 = _16 as u16;
_41 = _6;
_13.fld1 = _20.0 & _20.0;
_36 = _2;
(*_24) = core::ptr::addr_of_mut!(_39);
_2 = _36;
_38 = [_11,_11,_11,_11];
place!(Field::<[u16; 4]>(Variant(_18, 1), 0)) = [_11,_11,_11,_11];
_30 = [_32];
_20.0 = _6 as i16;
place!(Field::<(i16,)>(Variant(_33, 0), 0)) = (_20.0,);
_30 = [_32];
_9 = !295133301260861847743506084378759585550_u128;
_40 = -_14;
_15 = -_26;
_24 = core::ptr::addr_of!((*_24));
place!(Field::<[u16; 4]>(Variant(_18, 1), 0)) = _38;
Goto(bb12)
}
bb12 = {
_13.fld1 = _23.0 ^ _5;
_31 = _29 & _29;
place!(Field::<[u16; 4]>(Variant(_18, 1), 0)) = _38;
_17 = _4;
_20.0 = _13.fld1 + _13.fld1;
_17 = !_4;
_30 = [_7];
(*_24) = core::ptr::addr_of_mut!(_39);
_16 = _2;
_8 = 76147444768004303669563988306514537405_i128;
_15 = _40 as f32;
_32 = -_7;
_38 = [_11,_11,_11,_11];
match _8 {
0 => bb6,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb10,
76147444768004303669563988306514537405 => bb14,
_ => bb13
}
}
bb13 = {
RET = ['\u{e3d1d}','\u{47597}','\u{f575c}','\u{d0949}','\u{29a99}','\u{d6717}'];
RET = ['\u{ecb09}','\u{7ff73}','\u{bdadb}','\u{d368f}','\u{10be31}','\u{efc3b}'];
_10 = 56_u8 - 143_u8;
Goto(bb3)
}
bb14 = {
_29 = !_1;
_37 = _10;
_11 = _10 as u16;
_4 = -_17;
_24 = core::ptr::addr_of!((*_24));
_12 = 4076564964_u32;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(0_usize, 23_usize, Move(_23), 37_usize, Move(_37), 6_usize, Move(_6), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(0_usize, 40_usize, Move(_40), 32_usize, Move(_32), 1_usize, Move(_1), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(0_usize, 8_usize, Move(_8), 4_usize, Move(_4), 29_usize, Move(_29), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(0_usize, 38_usize, Move(_38), 20_usize, Move(_20), 3_usize, Move(_3), 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i16,mut _2: i16,mut _3: char,mut _4: i16,mut _5: bool,mut _6: i16,mut _7: i8,mut _8: i16,mut _9: i16,mut _10: isize,mut _11: (i16,),mut _12: u8,mut _13: i8,mut _14: char,mut _15: (i16,)) -> isize {
mir! {
type RET = isize;
let _16: i16;
let _17: Adt47;
let _18: char;
let _19: bool;
let _20: i16;
let _21: f64;
let _22: Adt47;
let _23: i64;
let _24: isize;
let _25: f64;
let _26: *mut (i16,);
let _27: ();
let _28: ();
{
RET = !_10;
_10 = RET;
Call(_11.0 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = -_15.0;
_13 = 15334075683706701149_u64 as i8;
_5 = false;
_3 = _14;
_3 = _14;
_15 = _11;
_13 = _7 >> _2;
_2 = _6 << _6;
_16 = -_6;
_3 = _14;
_1 = 237112701548450119169482478287451805306_u128 as i16;
_3 = _14;
_8 = _2;
_11 = (_8,);
_3 = _14;
_2 = 27895_u16 as i16;
_2 = _4;
RET = _10;
_16 = _15.0 ^ _9;
_8 = !_4;
RET = _10;
_6 = _16 - _16;
_2 = _6 + _11.0;
_18 = _14;
Goto(bb2)
}
bb2 = {
_4 = (-2620696459185744583_i64) as i16;
_10 = RET;
_11 = (_6,);
_15.0 = _11.0;
_9 = _2;
_13 = -_7;
_17 = Adt47::Variant0 { fld0: _7 };
place!(Field::<i8>(Variant(_17, 0), 0)) = _7 ^ _13;
_6 = !_15.0;
_2 = -_6;
_12 = 8149855946228338589_u64 as u8;
_18 = _14;
_8 = _6 | _2;
_19 = _5;
_8 = !_9;
_11 = _15;
_6 = _10 as i16;
_15 = (_9,);
_9 = !_16;
_21 = 256593956_u32 as f64;
SetDiscriminant(_17, 1);
_13 = _7 + _7;
_20 = 9898878658562669059_usize as i16;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld2 = 10125932727397135308_u64 >> _11.0;
Call(_15.0 = core::intrinsics::transmute(_11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _10 << _9;
place!(Field::<u16>(Variant(_17, 1), 3)) = !21720_u16;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld1 = !(-1134536969_i32);
_15.0 = _8;
_18 = _3;
_15.0 = !_2;
place!(Field::<u16>(Variant(_17, 1), 3)) = !53156_u16;
_10 = 544842452_u32 as isize;
_5 = !_19;
_16 = _9;
_18 = _3;
_22 = Adt47::Variant0 { fld0: _13 };
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld5 = RET as u32;
_5 = _19;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld1 = (-1409378259_i32) << _10;
_16 = _9;
_20 = _2;
SetDiscriminant(_22, 1);
Call(_6 = fn2(Field::<Adt37>(Variant(_17, 1), 0).fld2.fld2, _3, _18, _14, _20, _15.0, RET, Field::<Adt37>(Variant(_17, 1), 0).fld2.fld2, _2, Field::<Adt37>(Variant(_17, 1), 0).fld2.fld5, _21, Field::<Adt37>(Variant(_17, 1), 0).fld2.fld2, _8, _20, _2, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = (_6,);
_13 = _19 as i8;
_14 = _3;
_11.0 = Field::<Adt37>(Variant(_17, 1), 0).fld2.fld2 as i16;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld1 = _18;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld2.fld4 = _6;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld2.fld5 = Field::<Adt37>(Variant(_17, 1), 0).fld2.fld5 & Field::<Adt37>(Variant(_17, 1), 0).fld2.fld5;
_15 = (_6,);
RET = _10;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld0 = !115259613870189311578922865333931492662_u128;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld1 = Field::<Adt37>(Variant(_17, 1), 0).fld2.fld0 as i32;
_8 = _15.0;
place!(Field::<*mut (i16,)>(Variant(_17, 1), 2)) = core::ptr::addr_of_mut!(_15);
_23 = -(-75259845011508101_i64);
_8 = -_15.0;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld2.fld1 = _3;
_3 = _14;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld2 = _12 as u64;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld3 = _7 >> _6;
_10 = RET;
Call(place!(Field::<Adt37>(Variant(_17, 1), 0)).fld3 = fn3(_15.0, Field::<*mut (i16,)>(Variant(_17, 1), 2), Field::<Adt37>(Variant(_22, 1), 0).fld3, _8, Field::<Adt37>(Variant(_22, 1), 0).fld2.fld4, _15.0, _8, Field::<*mut (i16,)>(Variant(_17, 1), 2), Field::<Adt37>(Variant(_22, 1), 0).fld2.fld4, Field::<Adt37>(Variant(_22, 1), 0).fld3, _8, Field::<*mut (i16,)>(Variant(_17, 1), 2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = _15;
place!(Field::<Adt37>(Variant(_22, 1), 0)).fld1 = Field::<Adt37>(Variant(_17, 1), 0).fld1 & Field::<Adt37>(Variant(_17, 1), 0).fld1;
_7 = -Field::<Adt37>(Variant(_17, 1), 0).fld3;
RET = _10;
_15.0 = _11.0;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld1 = Field::<Adt37>(Variant(_22, 1), 0).fld2.fld1;
_14 = _18;
_10 = RET;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld4 = _15.0 + _15.0;
place!(Field::<Adt37>(Variant(_17, 1), 0)).fld2.fld0 = 47654963228314268094305587453455102106_u128;
RET = _10 >> Field::<Adt37>(Variant(_22, 1), 0).fld3;
Goto(bb6)
}
bb6 = {
Call(_27 = dump_var(1_usize, 7_usize, Move(_7), 15_usize, Move(_15), 12_usize, Move(_12), 19_usize, Move(_19)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_27 = dump_var(1_usize, 4_usize, Move(_4), 3_usize, Move(_3), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_27 = dump_var(1_usize, 5_usize, Move(_5), 11_usize, Move(_11), 28_usize, _28, 28_usize, _28), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u64,mut _2: char,mut _3: char,mut _4: char,mut _5: i16,mut _6: i16,mut _7: isize,mut _8: u64,mut _9: i16,mut _10: u32,mut _11: f64,mut _12: u64,mut _13: i16,mut _14: i16,mut _15: i16,mut _16: char) -> i16 {
mir! {
type RET = i16;
let _17: isize;
let _18: *mut (i16,);
let _19: i128;
let _20: Adt49;
let _21: bool;
let _22: isize;
let _23: [u16; 4];
let _24: isize;
let _25: u128;
let _26: Adt48;
let _27: [u16; 4];
let _28: [char; 6];
let _29: (i16,);
let _30: f64;
let _31: char;
let _32: ();
let _33: ();
{
_12 = _7 as u64;
_4 = _2;
RET = _6;
_13 = _6;
_9 = -_13;
_1 = !_8;
_7 = -57_isize;
RET = _14;
RET = 189887938591414492814356345370479312057_u128 as i16;
_3 = _16;
_11 = _1 as f64;
_16 = _4;
_1 = _12 ^ _12;
_7 = !94_isize;
_14 = (-2421453334617262816176647664074686622_i128) as i16;
_12 = _8 >> _9;
_4 = _3;
_6 = !_15;
_3 = _2;
_9 = _6 >> _8;
_5 = -RET;
_9 = _15 * _6;
_13 = -_15;
_17 = _7;
_6 = _9;
_13 = false as i16;
Goto(bb1)
}
bb1 = {
_9 = _10 as i16;
_2 = _3;
_15 = RET * _6;
_2 = _3;
Goto(bb2)
}
bb2 = {
_7 = _2 as isize;
_13 = _7 as i16;
_6 = _5 + _15;
_9 = _6 | _6;
_14 = !_9;
Goto(bb3)
}
bb3 = {
_6 = 71812888650513036190212082835430825043_u128 as i16;
_14 = _15 & _15;
_3 = _4;
_5 = 4579436310910237348_i64 as i16;
_6 = !_14;
_21 = !true;
_5 = 27291_u16 as i16;
_8 = _1 + _12;
_1 = _10 as u64;
_17 = _7;
_10 = 393024014_u32 >> _8;
_4 = _16;
_17 = _7 & _7;
_5 = _11 as i16;
_8 = _1 & _12;
_9 = (-76_i8) as i16;
_19 = (-86455269335140670088366128398454452463_i128) & 79662313790737747776667647743282998444_i128;
RET = _14;
RET = _7 as i16;
_22 = _17 & _7;
_8 = _1;
_23 = [29274_u16,26830_u16,26309_u16,48513_u16];
_19 = (-5098629911825873097284467230531905239_i128) - (-121922366482593097423061046196135875877_i128);
_3 = _16;
_14 = _5 >> _6;
_8 = _12 ^ _12;
Goto(bb4)
}
bb4 = {
_17 = _22 * _7;
_24 = _7 ^ _17;
_17 = _24;
_15 = 223949227442952951577831561993682703206_u128 as i16;
_24 = !_17;
_6 = !_14;
_17 = !_24;
_13 = !_6;
_3 = _2;
Goto(bb5)
}
bb5 = {
_25 = !110436433612484579091313413838249102852_u128;
RET = _14 >> _14;
_2 = _3;
_13 = !_6;
_6 = _13;
_27 = [47530_u16,710_u16,54845_u16,6228_u16];
_16 = _3;
RET = -_13;
_6 = _10 as i16;
RET = -_6;
_12 = _1;
_14 = _6 - _6;
_29 = (_13,);
RET = _13 << _8;
_29 = (RET,);
RET = !_29.0;
_7 = _24;
_7 = _24 ^ _24;
_16 = _4;
_17 = _7 - _7;
_7 = _17 >> _10;
Goto(bb6)
}
bb6 = {
Call(_32 = dump_var(2_usize, 23_usize, Move(_23), 21_usize, Move(_21), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_32 = dump_var(2_usize, 4_usize, Move(_4), 27_usize, Move(_27), 29_usize, Move(_29), 24_usize, Move(_24)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_32 = dump_var(2_usize, 13_usize, Move(_13), 22_usize, Move(_22), 25_usize, Move(_25), 7_usize, Move(_7)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i16,mut _2: *mut (i16,),mut _3: i8,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: i16,mut _8: *mut (i16,),mut _9: i16,mut _10: i8,mut _11: i16,mut _12: *mut (i16,)) -> i8 {
mir! {
type RET = i8;
let _13: ();
let _14: ();
{
_11 = 7291804696343587585_i64 as i16;
(*_2) = (_5,);
RET = _3;
(*_12) = (_5,);
_5 = _1;
(*_2) = (_5,);
_3 = RET;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(3_usize, 11_usize, Move(_11), 6_usize, Move(_6), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: *mut [char; 6],mut _3: isize,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: isize,mut _9: isize,mut _10: isize) -> char {
mir! {
type RET = char;
let _11: f64;
let _12: Adt41;
let _13: Adt51;
let _14: [i128; 3];
let _15: u128;
let _16: bool;
let _17: Adt37;
let _18: f64;
let _19: i128;
let _20: [u16; 4];
let _21: isize;
let _22: f64;
let _23: [char; 6];
let _24: isize;
let _25: [char; 6];
let _26: [u16; 4];
let _27: [i128; 3];
let _28: bool;
let _29: Adt36;
let _30: ();
let _31: ();
{
_7 = _5;
(*_2) = ['\u{86ea7}','\u{bad}','\u{98f7}','\u{da073}','\u{41ce1}','\u{34657}'];
_4 = !_6;
(*_2) = ['\u{f3c6b}','\u{ccb2}','\u{fc379}','\u{53c59}','\u{a4043}','\u{dfd45}'];
RET = '\u{dd568}';
_5 = _6;
_9 = _8 << _3;
RET = '\u{dafd3}';
Goto(bb1)
}
bb1 = {
_5 = _3 < _10;
_6 = _9 < _8;
_5 = _4;
_9 = 1856289070_i32 as isize;
_3 = _1;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [RET,RET,RET,RET,RET,RET];
_1 = 49417218642308541344774611360906775617_u128 as isize;
_1 = !_8;
_7 = !_5;
_4 = _3 != _10;
_13.fld0 = [8895590277177623406575246271887509058_i128,19217084053969718032100237382637721772_i128,(-76159657061003842712337717368964960044_i128)];
_13.fld1 = (-27376_i16) << _8;
_11 = 1310587209_u32 as f64;
_6 = !_7;
_10 = (-3012453277077078385_i64) as isize;
Call(_12 = fn5(_13, _1, _3, _13, _5, _3, _13.fld1, _5, _5, _9, _6, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = _5 as u128;
_17.fld2.fld5 = !4022040645_u32;
_4 = _6;
_1 = -_3;
_15 = 98908711087834481417390406827960963278_u128;
_16 = _5;
SetDiscriminant(_12, 1);
_19 = -(-149178528014607619150386088460381993326_i128);
place!(Field::<i64>(Variant(_12, 1), 1)) = (-4203580094046207815_i64) | 9024783399533637469_i64;
_1 = 1339172864_i32 as isize;
_5 = _7;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
(*_2) = [RET,RET,RET,RET,RET,RET];
_15 = 239850493284266738658828080589073817523_u128 + 237911504113922677500851345462877199477_u128;
place!(Field::<i64>(Variant(_12, 1), 1)) = -(-3892395139181224321_i64);
_17.fld3 = !65_i8;
_3 = _17.fld2.fld5 as isize;
_17.fld1 = _11 as i32;
_5 = _6;
Call(RET = fn9(_16, _13, _8, _13.fld1, _13, _8, _8, _6, _4, _13.fld1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _17.fld3 as isize;
_17.fld2.fld5 = 7917789629443403301_u64 as u32;
_4 = !_16;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
RET = '\u{63b75}';
_17.fld2.fld5 = _15 as u32;
Goto(bb4)
}
bb4 = {
_17.fld2.fld2 = !3761935469364998178_u64;
_17.fld3 = Field::<i64>(Variant(_12, 1), 1) as i8;
_14 = [_19,_19,_19];
RET = '\u{62992}';
_13.fld1 = (-6746_i16);
place!(Field::<i64>(Variant(_12, 1), 1)) = (-5753877294846134175_i64) + (-420402782185313775_i64);
_12 = Adt41::Variant1 { fld0: _17.fld3,fld1: (-3085280195607407367_i64) };
Goto(bb5)
}
bb5 = {
_2 = _17.fld2.fld3;
_20 = [26791_u16,27691_u16,53913_u16,20220_u16];
_16 = _7;
_14 = [_19,_19,_19];
_17.fld2.fld4 = !_13.fld1;
_12 = Adt41::Variant1 { fld0: _17.fld3,fld1: (-2980198408350824222_i64) };
_17.fld2.fld4 = -_13.fld1;
_17.fld3 = Field::<i8>(Variant(_12, 1), 0);
_17.fld2.fld1 = RET;
place!(Field::<i64>(Variant(_12, 1), 1)) = (-5065426571881398504_i64) - 8563354578007018845_i64;
SetDiscriminant(_12, 0);
_17.fld2.fld0 = !_15;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
_14 = [_19,_19,_19];
_3 = -_8;
RET = _17.fld2.fld1;
_2 = _17.fld2.fld3;
_9 = _17.fld2.fld4 as isize;
(*_2) = [_17.fld2.fld1,_17.fld2.fld1,RET,_17.fld2.fld1,RET,_17.fld2.fld1];
_13 = Adt51 { fld0: _14,fld1: _17.fld2.fld4 };
_18 = _11 * _11;
_5 = _6;
_1 = _3 - _3;
_17.fld2.fld4 = _17.fld2.fld0 as i16;
_8 = 128_u8 as isize;
_14 = [_19,_19,_19];
_17.fld2.fld3 = _2;
_1 = _3;
Goto(bb6)
}
bb6 = {
_2 = core::ptr::addr_of_mut!((*_2));
place!(Field::<char>(Variant(_12, 0), 1)) = _17.fld2.fld1;
_17.fld0 = 7143830897968949125_i64;
(*_2) = [Field::<char>(Variant(_12, 0), 1),_17.fld2.fld1,Field::<char>(Variant(_12, 0), 1),Field::<char>(Variant(_12, 0), 1),RET,Field::<char>(Variant(_12, 0), 1)];
Goto(bb7)
}
bb7 = {
Call(_5 = fn19(_3, _3, _4, _16, _16, _4, _7, _16, _3, _4, _3, _1, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = [21408_u16,30008_u16,31361_u16,37033_u16];
_9 = _4 as isize;
_17.fld2.fld2 = !430848794259051096_u64;
place!(Field::<(i16,)>(Variant(_12, 0), 0)).0 = -_17.fld2.fld4;
_17.fld3 = 68_i8 - (-19_i8);
SetDiscriminant(_12, 1);
_20 = [52828_u16,12144_u16,5431_u16,53023_u16];
_13.fld1 = _17.fld0 as i16;
_12 = Adt41::Variant1 { fld0: _17.fld3,fld1: _17.fld0 };
_11 = _18;
_17.fld2.fld1 = RET;
_15 = !_17.fld2.fld0;
_6 = _5;
_13 = Adt51 { fld0: _14,fld1: _17.fld2.fld4 };
_17.fld2.fld5 = 1261508310_u32 & 4020537783_u32;
Goto(bb9)
}
bb9 = {
_6 = _16;
RET = _17.fld2.fld1;
_25 = [_17.fld2.fld1,RET,RET,_17.fld2.fld1,_17.fld2.fld1,RET];
_21 = _1 | _9;
_3 = _21;
_23 = _25;
_7 = _5 != _6;
_8 = !_9;
SetDiscriminant(_12, 0);
_13.fld1 = _17.fld2.fld4 >> _3;
_26 = [45810_u16,49426_u16,40653_u16,8382_u16];
_6 = _5 & _5;
_17.fld2.fld0 = _15;
match _17.fld0 {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb4,
4 => bb7,
7143830897968949125 => bb11,
_ => bb10
}
}
bb10 = {
_10 = _17.fld3 as isize;
_17.fld2.fld5 = 7917789629443403301_u64 as u32;
_4 = !_16;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
RET = '\u{63b75}';
_17.fld2.fld5 = _15 as u32;
Goto(bb4)
}
bb11 = {
place!(Field::<char>(Variant(_12, 0), 1)) = RET;
RET = Field::<char>(Variant(_12, 0), 1);
_21 = _1 >> _13.fld1;
_12 = Adt41::Variant1 { fld0: _17.fld3,fld1: _17.fld0 };
_17.fld2.fld4 = _19 as i16;
_13.fld1 = !_17.fld2.fld4;
RET = _17.fld2.fld1;
_24 = 19_u8 as isize;
_5 = !_4;
match _17.fld0 {
0 => bb1,
1 => bb12,
7143830897968949125 => bb14,
_ => bb13
}
}
bb12 = {
_15 = _5 as u128;
_17.fld2.fld5 = !4022040645_u32;
_4 = _6;
_1 = -_3;
_15 = 98908711087834481417390406827960963278_u128;
_16 = _5;
SetDiscriminant(_12, 1);
_19 = -(-149178528014607619150386088460381993326_i128);
place!(Field::<i64>(Variant(_12, 1), 1)) = (-4203580094046207815_i64) | 9024783399533637469_i64;
_1 = 1339172864_i32 as isize;
_5 = _7;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
(*_2) = [RET,RET,RET,RET,RET,RET];
_15 = 239850493284266738658828080589073817523_u128 + 237911504113922677500851345462877199477_u128;
place!(Field::<i64>(Variant(_12, 1), 1)) = -(-3892395139181224321_i64);
_17.fld3 = !65_i8;
_3 = _17.fld2.fld5 as isize;
_17.fld1 = _11 as i32;
_5 = _6;
Call(RET = fn9(_16, _13, _8, _13.fld1, _13, _8, _8, _6, _4, _13.fld1), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_2 = _17.fld2.fld3;
_20 = [26791_u16,27691_u16,53913_u16,20220_u16];
_16 = _7;
_14 = [_19,_19,_19];
_17.fld2.fld4 = !_13.fld1;
_12 = Adt41::Variant1 { fld0: _17.fld3,fld1: (-2980198408350824222_i64) };
_17.fld2.fld4 = -_13.fld1;
_17.fld3 = Field::<i8>(Variant(_12, 1), 0);
_17.fld2.fld1 = RET;
place!(Field::<i64>(Variant(_12, 1), 1)) = (-5065426571881398504_i64) - 8563354578007018845_i64;
SetDiscriminant(_12, 0);
_17.fld2.fld0 = !_15;
_17.fld2.fld3 = core::ptr::addr_of_mut!((*_2));
_14 = [_19,_19,_19];
_3 = -_8;
RET = _17.fld2.fld1;
_2 = _17.fld2.fld3;
_9 = _17.fld2.fld4 as isize;
(*_2) = [_17.fld2.fld1,_17.fld2.fld1,RET,_17.fld2.fld1,RET,_17.fld2.fld1];
_13 = Adt51 { fld0: _14,fld1: _17.fld2.fld4 };
_18 = _11 * _11;
_5 = _6;
_1 = _3 - _3;
_17.fld2.fld4 = _17.fld2.fld0 as i16;
_8 = 128_u8 as isize;
_14 = [_19,_19,_19];
_17.fld2.fld3 = _2;
_1 = _3;
Goto(bb6)
}
bb14 = {
_24 = 160_u8 as isize;
_6 = !_4;
_17.fld2.fld3 = _2;
_20 = [55402_u16,4239_u16,7309_u16,62225_u16];
_13.fld1 = Field::<i8>(Variant(_12, 1), 0) as i16;
(*_2) = [RET,RET,_17.fld2.fld1,_17.fld2.fld1,RET,_17.fld2.fld1];
place!(Field::<i64>(Variant(_12, 1), 1)) = -_17.fld0;
_14 = [_19,_19,_19];
_20 = [6188_u16,17143_u16,23227_u16,63377_u16];
_10 = -_9;
_26 = [43819_u16,15111_u16,39622_u16,40450_u16];
_1 = -_9;
_17.fld2.fld4 = _17.fld2.fld5 as i16;
_13.fld0 = [_19,_19,_19];
_6 = _4;
SetDiscriminant(_12, 1);
_11 = _18 + _18;
_29.fld0 = _15;
RET = _17.fld2.fld1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(4_usize, 19_usize, Move(_19), 16_usize, Move(_16), 25_usize, Move(_25), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(4_usize, 8_usize, Move(_8), 6_usize, Move(_6), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(4_usize, 26_usize, Move(_26), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: Adt51,mut _2: isize,mut _3: isize,mut _4: Adt51,mut _5: bool,mut _6: isize,mut _7: i16,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: bool,mut _12: isize) -> Adt41 {
mir! {
type RET = Adt41;
let _13: bool;
let _14: u128;
let _15: bool;
let _16: [i64; 1];
let _17: char;
let _18: Adt35;
let _19: isize;
let _20: f64;
let _21: (isize, usize, u16, &'static isize, (i16,), u128, (i16,));
let _22: *const *mut [char; 6];
let _23: Adt51;
let _24: u32;
let _25: char;
let _26: ();
let _27: ();
{
_10 = _2 & _6;
_10 = _12;
_1 = Adt51 { fld0: _4.fld0,fld1: _7 };
_4.fld0 = _1.fld0;
_13 = _12 >= _3;
Goto(bb1)
}
bb1 = {
_7 = !_4.fld1;
_12 = -_2;
_4 = Adt51 { fld0: _1.fld0,fld1: _1.fld1 };
_4.fld0 = _1.fld0;
_4 = Adt51 { fld0: _1.fld0,fld1: _7 };
_7 = 388142396_i32 as i16;
_3 = 1472511769_u32 as isize;
_10 = !_6;
_1.fld1 = 3142710842629255712_i64 as i16;
_4 = Adt51 { fld0: _1.fld0,fld1: _7 };
_9 = !_5;
_1 = Adt51 { fld0: _4.fld0,fld1: _7 };
RET = Adt41::Variant1 { fld0: (-37_i8),fld1: (-2631174421145709588_i64) };
_1.fld1 = _7;
_14 = 8548177090231750595_i64 as u128;
place!(Field::<i8>(Variant(RET, 1), 0)) = 0_usize as i8;
_1.fld0 = [43971258132076271170561599778251577019_i128,(-39782311159842329345106151353248335128_i128),(-1362640965932044328442263468604505525_i128)];
Call(_16 = fn6(_11, _6, _12, _10, _6, _12, _11, _9, _11, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = Adt41::Variant1 { fld0: 87_i8,fld1: (-9171181644317557842_i64) };
place!(Field::<i8>(Variant(RET, 1), 0)) = 76_i8 * 55_i8;
_14 = !223519314530263066276750102468547989003_u128;
_6 = _10 ^ _12;
_4.fld0 = [6318279044745390957574948683094144678_i128,164726815744347919084741773806783528866_i128,88733661156872749033858088065962031735_i128];
RET = Adt41::Variant1 { fld0: 46_i8,fld1: (-6059271027614852448_i64) };
_4.fld1 = _7;
_18.fld3 = 5764084435269638658_u64 & 9360677269853825981_u64;
_13 = !_5;
_18.fld6.0 = -_4.fld1;
_9 = _12 >= _12;
place!(Field::<i64>(Variant(RET, 1), 1)) = 50253866241090482944055061284274754982_i128 as i64;
_5 = _18.fld6.0 <= _18.fld6.0;
_18.fld0 = _9;
_17 = '\u{e1669}';
_13 = _12 != _6;
_14 = _18.fld3 as u128;
Call(_18.fld0 = fn8(_8, _13, _12, _11, _2, _13, _13, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _11 >= _11;
_18.fld1 = _17;
_13 = _9;
_18.fld5 = [_17,_18.fld1,_17,_18.fld1,_17,_17];
_18.fld2 = !_14;
RET = Adt41::Variant1 { fld0: (-34_i8),fld1: (-7540245122484297191_i64) };
_5 = _18.fld0;
_2 = !_6;
place!(Field::<i8>(Variant(RET, 1), 0)) = !(-79_i8);
_18.fld6 = (_1.fld1,);
_4.fld1 = _1.fld1 - _7;
RET = Adt41::Variant0 { fld0: _18.fld6,fld1: _18.fld1 };
_18.fld7 = 3_usize << _12;
RET = Adt41::Variant0 { fld0: _18.fld6,fld1: _18.fld1 };
_2 = -_12;
_18.fld7 = !5_usize;
_18.fld3 = 164_u8 as u64;
RET = Adt41::Variant0 { fld0: _18.fld6,fld1: _18.fld1 };
_21.2 = 11607_u16 * 29667_u16;
_21.6 = (Field::<(i16,)>(Variant(RET, 0), 0).0,);
_20 = _21.2 as f64;
place!(Field::<char>(Variant(RET, 0), 1)) = _17;
SetDiscriminant(RET, 1);
_12 = !_2;
_4.fld1 = (-10806139486474081444533335597506424068_i128) as i16;
_18.fld6 = _21.6;
_21.4.0 = -_4.fld1;
Goto(bb4)
}
bb4 = {
_18.fld6 = (_1.fld1,);
_16 = [(-8462046803293133210_i64)];
_7 = _18.fld6.0 >> _12;
RET = Adt41::Variant1 { fld0: (-42_i8),fld1: 530004725652942622_i64 };
RET = Adt41::Variant0 { fld0: _21.4,fld1: _17 };
_18.fld6 = _21.4;
place!(Field::<(i16,)>(Variant(RET, 0), 0)) = (_7,);
_21.6.0 = Field::<(i16,)>(Variant(RET, 0), 0).0 - _7;
_6 = _12 - _2;
_21.0 = _10;
_21.1 = _18.fld7;
_18.fld0 = _11;
place!(Field::<(i16,)>(Variant(RET, 0), 0)) = (_7,);
_17 = _18.fld1;
_8 = _5 >= _11;
_23.fld1 = _20 as i16;
_15 = !_8;
_4 = _1;
place!(Field::<char>(Variant(RET, 0), 1)) = _18.fld1;
_23.fld1 = -Field::<(i16,)>(Variant(RET, 0), 0).0;
_19 = _12;
_18.fld6.0 = -_23.fld1;
_18.fld1 = _17;
_18.fld0 = _9;
place!(Field::<(i16,)>(Variant(RET, 0), 0)).0 = _7;
Goto(bb5)
}
bb5 = {
Call(_26 = dump_var(5_usize, 10_usize, Move(_10), 19_usize, Move(_19), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_26 = dump_var(5_usize, 2_usize, Move(_2), 7_usize, Move(_7), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: isize) -> [i64; 1] {
mir! {
type RET = [i64; 1];
let _12: [u16; 4];
let _13: Adt36;
let _14: Adt36;
let _15: i64;
let _16: isize;
let _17: Adt41;
let _18: Adt51;
let _19: [i128; 3];
let _20: u8;
let _21: u128;
let _22: *mut [char; 6];
let _23: ();
let _24: ();
{
_5 = 59072186386266092040479702334283757326_u128 as isize;
RET = [5401454976227788005_i64];
RET = [236107492862513910_i64];
_2 = -_11;
RET = [(-482968626146306283_i64)];
_3 = 94_i8 as isize;
_4 = _11;
RET = [6883537620086782724_i64];
_11 = 11228_u16 as isize;
_1 = _9 > _9;
_2 = -_6;
_6 = -_10;
_4 = (-259_i16) as isize;
_7 = _1 > _9;
_2 = !_10;
_2 = _10;
_6 = _2 - _10;
_9 = _8 == _1;
_6 = !_10;
_6 = _2;
RET = [6846239393618656000_i64];
_7 = _6 <= _2;
_4 = 0_usize as isize;
_9 = _1 & _8;
_11 = _2;
_12 = [53784_u16,24446_u16,64429_u16,6515_u16];
_6 = 35672720275777546916957512184463764355_u128 as isize;
_3 = !_10;
Call(_5 = fn7(_7, _2, _9, _1, _3, _7, _10, _1, _1, _10, _8, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _5 ^ _10;
_10 = 47851_u16 as isize;
_13.fld5 = 1232362412_u32 >> _5;
_14.fld5 = (-83167532032969099287896068587118030597_i128) as u32;
_14.fld1 = '\u{3456b}';
_13.fld4 = (-15523_i16);
_14.fld5 = _13.fld5;
_13.fld0 = 160801094202368510014357739466103231226_u128;
_14.fld4 = 22877_u16 as i16;
_14.fld2 = _7 as u64;
RET = [(-8465501526121990880_i64)];
_14.fld0 = _13.fld0;
_14.fld2 = _8 as u64;
_17 = Adt41::Variant1 { fld0: (-120_i8),fld1: (-6824396925391743895_i64) };
_2 = _3 << _14.fld5;
_15 = 61_i8 as i64;
_12 = [45080_u16,23520_u16,58804_u16,54807_u16];
_12 = [52639_u16,60908_u16,214_u16,5683_u16];
_10 = _13.fld0 as isize;
Goto(bb2)
}
bb2 = {
_13.fld4 = 143_u8 as i16;
_17 = Adt41::Variant1 { fld0: (-48_i8),fld1: _15 };
_3 = _2 * _2;
_2 = _11 * _5;
_16 = _5 * _3;
place!(Field::<i64>(Variant(_17, 1), 1)) = _15;
place!(Field::<i8>(Variant(_17, 1), 0)) = !(-8_i8);
SetDiscriminant(_17, 1);
_13.fld2 = _14.fld2 + _14.fld2;
_14.fld4 = (-27868858236694785154006719818104150156_i128) as i16;
_5 = _16;
place!(Field::<i64>(Variant(_17, 1), 1)) = _15;
RET = [_15];
place!(Field::<i64>(Variant(_17, 1), 1)) = _15 - _15;
_7 = _3 > _5;
_18.fld1 = (-71_i8) as i16;
place!(Field::<i8>(Variant(_17, 1), 0)) = (-111_i8);
_1 = _14.fld2 <= _14.fld2;
_14.fld1 = '\u{1e452}';
Goto(bb3)
}
bb3 = {
_16 = !_2;
_14.fld2 = _13.fld2;
_13.fld1 = _14.fld1;
_11 = _2 * _16;
place!(Field::<i8>(Variant(_17, 1), 0)) = (-13_i8) - (-116_i8);
SetDiscriminant(_17, 1);
_13.fld5 = _14.fld5 * _14.fld5;
_12 = [37522_u16,5108_u16,42693_u16,36777_u16];
_5 = _16 + _16;
_14.fld5 = !_13.fld5;
_15 = _14.fld5 as i64;
_13.fld5 = _14.fld5;
Call(place!(Field::<i64>(Variant(_17, 1), 1)) = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13.fld4 = _18.fld1;
_6 = _5;
_9 = _14.fld2 != _13.fld2;
_19 = [(-97003194226461991394671291674269113808_i128),(-40659945562977426233642115398034785255_i128),(-168178933975196178819420205375415884612_i128)];
match _13.fld0 {
0 => bb2,
1 => bb5,
2 => bb6,
160801094202368510014357739466103231226 => bb8,
_ => bb7
}
}
bb5 = {
_16 = !_2;
_14.fld2 = _13.fld2;
_13.fld1 = _14.fld1;
_11 = _2 * _16;
place!(Field::<i8>(Variant(_17, 1), 0)) = (-13_i8) - (-116_i8);
SetDiscriminant(_17, 1);
_13.fld5 = _14.fld5 * _14.fld5;
_12 = [37522_u16,5108_u16,42693_u16,36777_u16];
_5 = _16 + _16;
_14.fld5 = !_13.fld5;
_15 = _14.fld5 as i64;
_13.fld5 = _14.fld5;
Call(place!(Field::<i64>(Variant(_17, 1), 1)) = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_13.fld4 = 143_u8 as i16;
_17 = Adt41::Variant1 { fld0: (-48_i8),fld1: _15 };
_3 = _2 * _2;
_2 = _11 * _5;
_16 = _5 * _3;
place!(Field::<i64>(Variant(_17, 1), 1)) = _15;
place!(Field::<i8>(Variant(_17, 1), 0)) = !(-8_i8);
SetDiscriminant(_17, 1);
_13.fld2 = _14.fld2 + _14.fld2;
_14.fld4 = (-27868858236694785154006719818104150156_i128) as i16;
_5 = _16;
place!(Field::<i64>(Variant(_17, 1), 1)) = _15;
RET = [_15];
place!(Field::<i64>(Variant(_17, 1), 1)) = _15 - _15;
_7 = _3 > _5;
_18.fld1 = (-71_i8) as i16;
place!(Field::<i8>(Variant(_17, 1), 0)) = (-111_i8);
_1 = _14.fld2 <= _14.fld2;
_14.fld1 = '\u{1e452}';
Goto(bb3)
}
bb7 = {
_2 = _5 ^ _10;
_10 = 47851_u16 as isize;
_13.fld5 = 1232362412_u32 >> _5;
_14.fld5 = (-83167532032969099287896068587118030597_i128) as u32;
_14.fld1 = '\u{3456b}';
_13.fld4 = (-15523_i16);
_14.fld5 = _13.fld5;
_13.fld0 = 160801094202368510014357739466103231226_u128;
_14.fld4 = 22877_u16 as i16;
_14.fld2 = _7 as u64;
RET = [(-8465501526121990880_i64)];
_14.fld0 = _13.fld0;
_14.fld2 = _8 as u64;
_17 = Adt41::Variant1 { fld0: (-120_i8),fld1: (-6824396925391743895_i64) };
_2 = _3 << _14.fld5;
_15 = 61_i8 as i64;
_12 = [45080_u16,23520_u16,58804_u16,54807_u16];
_12 = [52639_u16,60908_u16,214_u16,5683_u16];
_10 = _13.fld0 as isize;
Goto(bb2)
}
bb8 = {
_14.fld2 = !_13.fld2;
_1 = _6 <= _16;
_14.fld1 = _13.fld1;
_13.fld1 = _14.fld1;
_12 = [27769_u16,26141_u16,29218_u16,53473_u16];
_13.fld2 = _14.fld2 | _14.fld2;
_20 = _13.fld5 as u8;
_16 = _13.fld2 as isize;
_16 = !_3;
_14.fld1 = _13.fld1;
_21 = _14.fld0;
_18.fld1 = _14.fld1 as i16;
_18 = Adt51 { fld0: _19,fld1: _14.fld4 };
_17 = Adt41::Variant1 { fld0: 56_i8,fld1: _15 };
RET = [Field::<i64>(Variant(_17, 1), 1)];
_3 = _14.fld0 as isize;
Goto(bb9)
}
bb9 = {
Call(_23 = dump_var(6_usize, 9_usize, Move(_9), 10_usize, Move(_10), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_23 = dump_var(6_usize, 8_usize, Move(_8), 21_usize, Move(_21), 20_usize, Move(_20), 4_usize, Move(_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: isize,mut _6: bool,mut _7: isize,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: bool,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: i8;
let _14: ();
let _15: ();
{
_9 = _3;
RET = 50874_u16 as isize;
_12 = 6_usize as isize;
_5 = _7;
_3 = _6;
_11 = !_6;
_3 = _9;
_10 = 2378752002918659448_u64 as isize;
RET = _7;
_4 = !_9;
RET = _7 ^ _7;
_3 = _4;
_2 = 54383369818616451648212502395482574081_i128 as isize;
_9 = _3;
_10 = 243771792487727718516223599667404645355_u128 as isize;
_2 = !_7;
_2 = (-76276349343891881566250158532565000518_i128) as isize;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(7_usize, 8_usize, Move(_8), 3_usize, Move(_3), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(7_usize, 9_usize, Move(_9), 4_usize, Move(_4), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: bool,mut _3: isize,mut _4: bool,mut _5: isize,mut _6: bool,mut _7: bool,mut _8: isize) -> bool {
mir! {
type RET = bool;
let _9: [char; 6];
let _10: ();
let _11: ();
{
_5 = (-1954479729_i32) as isize;
_4 = _1;
_1 = _6;
RET = !_1;
_4 = _7 & RET;
_1 = _7 != RET;
_1 = _2;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(8_usize, 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: Adt51,mut _3: isize,mut _4: i16,mut _5: Adt51,mut _6: isize,mut _7: isize,mut _8: bool,mut _9: bool,mut _10: i16) -> char {
mir! {
type RET = char;
let _11: f64;
let _12: Adt42;
let _13: u16;
let _14: (i16,);
let _15: *const *mut [char; 6];
let _16: [i128; 3];
let _17: Adt43;
let _18: Adt51;
let _19: isize;
let _20: i128;
let _21: isize;
let _22: f32;
let _23: i128;
let _24: f32;
let _25: [i64; 1];
let _26: f32;
let _27: [u16; 4];
let _28: i8;
let _29: isize;
let _30: Adt51;
let _31: [char; 6];
let _32: f64;
let _33: i16;
let _34: i32;
let _35: f32;
let _36: i64;
let _37: isize;
let _38: char;
let _39: i32;
let _40: u64;
let _41: f32;
let _42: u128;
let _43: i32;
let _44: *mut ((isize, usize, u16, &'static isize, (i16,), u128, (i16,)), usize, &'static isize);
let _45: i16;
let _46: [i128; 3];
let _47: u16;
let _48: i32;
let _49: Adt41;
let _50: i32;
let _51: i128;
let _52: ();
let _53: ();
{
_9 = _1;
_10 = _2.fld1;
_8 = !_1;
RET = '\u{e066b}';
_1 = !_8;
_3 = _6;
_10 = _5.fld1;
_8 = !_1;
_5 = Adt51 { fld0: _2.fld0,fld1: _4 };
_2 = Adt51 { fld0: _5.fld0,fld1: _10 };
_2 = Adt51 { fld0: _5.fld0,fld1: _10 };
_8 = !_9;
_4 = -_2.fld1;
_4 = (-14966663677356796236055288949632279739_i128) as i16;
_8 = _9;
RET = '\u{813ee}';
_2.fld0 = [(-29968975704768739273460162188980354975_i128),22351592686469484692497593648960753769_i128,35618755896238293259375873893289431937_i128];
RET = '\u{86a49}';
_11 = 7432869950659525089_i64 as f64;
_8 = _1 == _9;
_11 = 4170580026528462914_i64 as f64;
_6 = !_3;
Call(_10 = fn10(_3, _2.fld1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _8 == _9;
_13 = !10460_u16;
_14 = (_2.fld1,);
Goto(bb2)
}
bb2 = {
_6 = _2.fld1 as isize;
_18.fld0 = _5.fld0;
_5 = _2;
_14 = (_2.fld1,);
_16 = [(-160563241825540891683471216837120564979_i128),(-20294811916164089206685167337344753150_i128),(-37199523159360953844662126798584963642_i128)];
_10 = (-9168471720947598593_i64) as i16;
_18 = Adt51 { fld0: _2.fld0,fld1: _5.fld1 };
_14 = (_2.fld1,);
_9 = _1;
_18.fld1 = 114358517989326052757402677178996980245_u128 as i16;
_4 = -_5.fld1;
_5.fld1 = 6850107561473678277_usize as i16;
_5.fld0 = [(-60122310270015260870890102374886418052_i128),(-137241363895615572507269096219623619227_i128),97995442842596600358871134821383421445_i128];
Call(_19 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14.0 = (-4_i8) as i16;
RET = '\u{6d4db}';
_16 = [(-126178187189522117505548962387287032231_i128),88837919684104929516407263374883051324_i128,(-14463454659285205306813857876299243628_i128)];
Call(_7 = core::intrinsics::bswap(_19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = !_1;
_14.0 = _4 << _6;
_2 = Adt51 { fld0: _16,fld1: _4 };
_18.fld1 = !_2.fld1;
_18 = _2;
_18.fld0 = [29122122901663476219591717694497469759_i128,114030975003978328023344403261821179268_i128,21970779403444474961512260017081585977_i128];
_20 = (-25314448907403945107003793951017744115_i128);
match _20 {
0 => bb1,
314967918013534518356370813480750467341 => bb6,
_ => bb5
}
}
bb5 = {
_1 = _8 == _9;
_13 = !10460_u16;
_14 = (_2.fld1,);
Goto(bb2)
}
bb6 = {
_14.0 = _4 | _4;
_7 = -_3;
_18.fld0 = _2.fld0;
_5 = Adt51 { fld0: _2.fld0,fld1: _14.0 };
_22 = _20 as f32;
_21 = 1491595527_u32 as isize;
_13 = 16340_u16;
_14 = (_2.fld1,);
_11 = 15394368846543831088_u64 as f64;
_2 = _5;
RET = '\u{b271c}';
_9 = _1 < _8;
_23 = _20;
_18 = Adt51 { fld0: _16,fld1: _5.fld1 };
_20 = !_23;
_13 = !62531_u16;
_16 = [_20,_23,_23];
_14.0 = 170_u8 as i16;
_21 = _20 as isize;
_23 = -_20;
_11 = _18.fld1 as f64;
_16 = [_20,_20,_23];
Goto(bb7)
}
bb7 = {
_28 = 84_i8;
_2.fld1 = _5.fld1 | _5.fld1;
_8 = _9;
Goto(bb8)
}
bb8 = {
_29 = _19;
_20 = _23 + _23;
_5.fld0 = [_20,_23,_20];
_9 = _7 != _3;
match _28 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
84 => bb9,
_ => bb7
}
}
bb9 = {
_19 = _29;
_18.fld0 = [_20,_20,_23];
_27 = [_13,_13,_13,_13];
_21 = _29 << _5.fld1;
_6 = !_3;
_28 = _4 as i8;
_27 = [_13,_13,_13,_13];
_24 = _22 * _22;
_8 = !_1;
_33 = _5.fld1;
Goto(bb10)
}
bb10 = {
_25 = [(-2172418961242951874_i64)];
_24 = 511156978_u32 as f32;
_14.0 = !_18.fld1;
_35 = _22;
_30.fld0 = [_20,_23,_20];
_32 = -_11;
_34 = -(-567308586_i32);
_30.fld1 = _32 as i16;
_1 = _30.fld1 < _4;
_2.fld1 = _4;
_29 = !_3;
_36 = !4139637059433003449_i64;
_3 = _32 as isize;
_14 = (_33,);
_30 = Adt51 { fld0: _5.fld0,fld1: _4 };
_25 = [_36];
_10 = _5.fld1 | _14.0;
_7 = _29 - _19;
_37 = _19 | _21;
Goto(bb11)
}
bb11 = {
_28 = _1 as i8;
_38 = RET;
_7 = _21 + _37;
_26 = _35 - _35;
_25 = [_36];
_32 = -_11;
_6 = 6873080984116938566_u64 as isize;
_13 = 11025_u16 & 51238_u16;
_9 = _1;
_9 = _10 < _2.fld1;
Goto(bb12)
}
bb12 = {
_26 = -_24;
_10 = -_14.0;
_39 = _34;
_31 = [RET,RET,RET,RET,RET,_38];
_2 = Adt51 { fld0: _18.fld0,fld1: _18.fld1 };
_6 = _8 as isize;
_2 = Adt51 { fld0: _16,fld1: _14.0 };
_37 = _6;
_2.fld1 = _18.fld1 + _4;
_41 = -_22;
_19 = !_7;
_5 = Adt51 { fld0: _30.fld0,fld1: _18.fld1 };
Goto(bb13)
}
bb13 = {
_37 = -_3;
_33 = -_2.fld1;
_13 = 178677071894278921872755031285578595027_u128 as u16;
RET = _38;
_30.fld1 = !_2.fld1;
Goto(bb14)
}
bb14 = {
_46 = _2.fld0;
_20 = _22 as i128;
_45 = 36893695717794543604645879394533656181_u128 as i16;
_4 = -_10;
_18.fld0 = _30.fld0;
_46 = [_20,_23,_23];
_18 = _2;
_43 = !_39;
_13 = !63700_u16;
_30.fld1 = _4 - _10;
_24 = _19 as f32;
_40 = 7774763365812852079_u64;
_29 = _7 ^ _21;
_29 = !_7;
_37 = -_19;
_30 = Adt51 { fld0: _5.fld0,fld1: _18.fld1 };
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(9_usize, 34_usize, Move(_34), 27_usize, Move(_27), 37_usize, Move(_37), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(9_usize, 38_usize, Move(_38), 40_usize, Move(_40), 36_usize, Move(_36), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_52 = dump_var(9_usize, 4_usize, Move(_4), 20_usize, Move(_20), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_52 = dump_var(9_usize, 31_usize, Move(_31), 7_usize, Move(_7), 33_usize, Move(_33), 53_usize, _53), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: i16,mut _3: bool) -> i16 {
mir! {
type RET = i16;
let _4: char;
let _5: i32;
let _6: Adt41;
let _7: (i16,);
let _8: i128;
let _9: i64;
let _10: i32;
let _11: f32;
let _12: u64;
let _13: [char; 6];
let _14: f64;
let _15: f32;
let _16: Adt51;
let _17: u8;
let _18: [u16; 4];
let _19: ((isize, usize, u16, &'static isize, (i16,), u128, (i16,)), usize, &'static isize);
let _20: f32;
let _21: ();
let _22: ();
{
RET = _2 + _2;
_1 = (-9223372036854775808_isize);
_1 = (-9223372036854775808_isize) | 75_isize;
_1 = (-9223372036854775808_isize);
_2 = !RET;
_4 = '\u{101942}';
_2 = 144886668919098797343584796680228265473_i128 as i16;
_3 = true;
RET = _2 >> _1;
_1 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_7 = (RET,);
_7 = (RET,);
_2 = !RET;
_6 = Adt41::Variant1 { fld0: 76_i8,fld1: (-3667256448172004369_i64) };
_2 = !_7.0;
_6 = Adt41::Variant0 { fld0: _7,fld1: _4 };
Call(_8 = fn11(_1, _4, _3, _7, _1, Field::<(i16,)>(Variant(_6, 0), 0), Field::<(i16,)>(Variant(_6, 0), 0).0, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_5 = (-2110240196_i32) + 1783930918_i32;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_1 = 3172319748436768392_usize as isize;
_2 = (-1303249791483364105_i64) as i16;
_10 = _5;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_7 = Field::<(i16,)>(Variant(_6, 0), 0);
_6 = Adt41::Variant1 { fld0: 37_i8,fld1: 6715200922770873357_i64 };
_5 = _10 | _10;
_11 = 314510420891746563945094821321875569104_u128 as f32;
_6 = Adt41::Variant0 { fld0: _7,fld1: _4 };
_2 = RET;
_7.0 = _2;
_13 = [Field::<char>(Variant(_6, 0), 1),_4,Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),_4,_4];
_13 = [Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),_4,Field::<char>(Variant(_6, 0), 1)];
_7 = Field::<(i16,)>(Variant(_6, 0), 0);
_6 = Adt41::Variant1 { fld0: 119_i8,fld1: (-1682933429088350025_i64) };
_3 = _5 <= _5;
Goto(bb2)
}
bb2 = {
_2 = !RET;
_13 = [_4,_4,_4,_4,_4,_4];
_11 = 65383_u16 as f32;
place!(Field::<i8>(Variant(_6, 1), 0)) = (-126_i8);
_12 = 5544571650673964564_u64;
_7.0 = -_2;
_5 = _10;
_10 = _5;
_2 = !_7.0;
_14 = _12 as f64;
_5 = _8 as i32;
_6 = Adt41::Variant1 { fld0: 40_i8,fld1: (-1896015599722465250_i64) };
place!(Field::<i64>(Variant(_6, 1), 1)) = (-4968288736089676479_i64) ^ 4311300209035191767_i64;
_5 = _12 as i32;
_7.0 = RET;
_10 = -_5;
_6 = Adt41::Variant1 { fld0: (-72_i8),fld1: (-7164187111246675300_i64) };
_3 = false | true;
_9 = (-2613767286361140352_i64) ^ 1541178272942206237_i64;
_10 = _5 >> RET;
_7.0 = !RET;
place!(Field::<i64>(Variant(_6, 1), 1)) = RET as i64;
_5 = _3 as i32;
_7.0 = 15_i8 as i16;
_2 = _4 as i16;
_8 = 2059771426_u32 as i128;
match _12 {
0 => bb3,
1 => bb4,
2 => bb5,
5544571650673964564 => bb7,
_ => bb6
}
}
bb3 = {
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_5 = (-2110240196_i32) + 1783930918_i32;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_1 = 3172319748436768392_usize as isize;
_2 = (-1303249791483364105_i64) as i16;
_10 = _5;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
_7 = Field::<(i16,)>(Variant(_6, 0), 0);
_6 = Adt41::Variant1 { fld0: 37_i8,fld1: 6715200922770873357_i64 };
_5 = _10 | _10;
_11 = 314510420891746563945094821321875569104_u128 as f32;
_6 = Adt41::Variant0 { fld0: _7,fld1: _4 };
_2 = RET;
_7.0 = _2;
_13 = [Field::<char>(Variant(_6, 0), 1),_4,Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),_4,_4];
_13 = [Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1),_4,Field::<char>(Variant(_6, 0), 1)];
_7 = Field::<(i16,)>(Variant(_6, 0), 0);
_6 = Adt41::Variant1 { fld0: 119_i8,fld1: (-1682933429088350025_i64) };
_3 = _5 <= _5;
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
RET = _9 as i16;
place!(Field::<i64>(Variant(_6, 1), 1)) = !_9;
_2 = _14 as i16;
_9 = _3 as i64;
_2 = _7.0;
_7.0 = _2;
_6 = Adt41::Variant0 { fld0: _7,fld1: _4 };
RET = Field::<(i16,)>(Variant(_6, 0), 0).0 | _2;
_15 = _11 - _11;
Goto(bb8)
}
bb8 = {
_7.0 = -RET;
_4 = Field::<char>(Variant(_6, 0), 1);
Goto(bb9)
}
bb9 = {
RET = _7.0 >> _12;
_2 = RET;
_15 = -_11;
_4 = Field::<char>(Variant(_6, 0), 1);
_13 = [_4,_4,Field::<char>(Variant(_6, 0), 1),_4,Field::<char>(Variant(_6, 0), 1),Field::<char>(Variant(_6, 0), 1)];
_16.fld1 = !_7.0;
RET = _7.0;
_18 = [14678_u16,15084_u16,27828_u16,65468_u16];
_3 = false & false;
_4 = Field::<char>(Variant(_6, 0), 1);
_2 = !RET;
_17 = !203_u8;
_7 = (_16.fld1,);
RET = _2;
_8 = 20639119573319391927562906846483113994_i128;
_14 = _7.0 as f64;
_15 = -_11;
_15 = _11 - _11;
_16.fld0 = [_8,_8,_8];
_19.0.4.0 = _10 as i16;
SetDiscriminant(_6, 0);
_19.0.6.0 = _4 as i16;
_17 = 141_u8;
_2 = _16.fld1;
_19.0.3 = &_19.0.0;
_19.0.4 = (_2,);
_2 = _19.0.4.0 * _19.0.4.0;
place!(Field::<char>(Variant(_6, 0), 1)) = _4;
match _8 {
0 => bb10,
1 => bb11,
20639119573319391927562906846483113994 => bb13,
_ => bb12
}
}
bb10 = {
_7.0 = -RET;
_4 = Field::<char>(Variant(_6, 0), 1);
Goto(bb9)
}
bb11 = {
_2 = !RET;
_13 = [_4,_4,_4,_4,_4,_4];
_11 = 65383_u16 as f32;
place!(Field::<i8>(Variant(_6, 1), 0)) = (-126_i8);
_12 = 5544571650673964564_u64;
_7.0 = -_2;
_5 = _10;
_10 = _5;
_2 = !_7.0;
_14 = _12 as f64;
_5 = _8 as i32;
_6 = Adt41::Variant1 { fld0: 40_i8,fld1: (-1896015599722465250_i64) };
place!(Field::<i64>(Variant(_6, 1), 1)) = (-4968288736089676479_i64) ^ 4311300209035191767_i64;
_5 = _12 as i32;
_7.0 = RET;
_10 = -_5;
_6 = Adt41::Variant1 { fld0: (-72_i8),fld1: (-7164187111246675300_i64) };
_3 = false | true;
_9 = (-2613767286361140352_i64) ^ 1541178272942206237_i64;
_10 = _5 >> RET;
_7.0 = !RET;
place!(Field::<i64>(Variant(_6, 1), 1)) = RET as i64;
_5 = _3 as i32;
_7.0 = 15_i8 as i16;
_2 = _4 as i16;
_8 = 2059771426_u32 as i128;
match _12 {
0 => bb3,
1 => bb4,
2 => bb5,
5544571650673964564 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
_9 = 629657199475135465_i64;
_3 = !false;
_6 = Adt41::Variant1 { fld0: 40_i8,fld1: _9 };
_13 = [_4,_4,_4,_4,_4,_4];
_19.1 = 14578936202479679534_usize ^ 15504498820310429178_usize;
_19.2 = &_19.0.0;
_4 = '\u{3bd50}';
_19.0.5 = 42037088528994767226951702288250323160_u128;
_19.0.6 = (_2,);
Goto(bb14)
}
bb14 = {
RET = -_19.0.6.0;
_19.0.1 = _19.1 - _19.1;
_7 = (_16.fld1,);
_6 = Adt41::Variant1 { fld0: (-94_i8),fld1: _9 };
_7 = (_19.0.6.0,);
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(10_usize, 12_usize, Move(_12), 2_usize, Move(_2), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(10_usize, 10_usize, Move(_10), 9_usize, Move(_9), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: char,mut _3: bool,mut _4: (i16,),mut _5: isize,mut _6: (i16,),mut _7: i16,mut _8: Adt41,mut _9: bool) -> i128 {
mir! {
type RET = i128;
let _10: Adt51;
let _11: [u16; 4];
let _12: u16;
let _13: [i128; 3];
let _14: bool;
let _15: u128;
let _16: u16;
let _17: char;
let _18: f64;
let _19: Adt41;
let _20: char;
let _21: i32;
let _22: Adt41;
let _23: Adt41;
let _24: u8;
let _25: isize;
let _26: [i128; 3];
let _27: i8;
let _28: (i16,);
let _29: Adt43;
let _30: isize;
let _31: u8;
let _32: (i128, (&'static isize, isize, *mut usize, i32), i64);
let _33: f32;
let _34: bool;
let _35: ();
let _36: ();
{
_9 = _3 ^ _3;
_1 = _5;
_1 = 7843012875423189383_u64 as isize;
_4.0 = (-1655350559_i32) as i16;
RET = !45581209563201873260462623221699459186_i128;
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (_6.0,);
_7 = Field::<(i16,)>(Variant(_8, 0), 0).0 ^ _6.0;
_7 = _6.0;
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
_10.fld0 = [RET,RET,RET];
_4 = (Field::<(i16,)>(Variant(_8, 0), 0).0,);
_5 = _1;
_5 = _1 << _7;
Goto(bb1)
}
bb1 = {
place!(Field::<(i16,)>(Variant(_8, 0), 0)).0 = _4.0;
_4 = (_7,);
_1 = !_5;
_10.fld0 = [RET,RET,RET];
_6 = (_4.0,);
_4.0 = Field::<(i16,)>(Variant(_8, 0), 0).0;
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (_4.0,);
_7 = 125_u8 as i16;
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
_6 = (Field::<(i16,)>(Variant(_8, 0), 0).0,);
_1 = 48_i8 as isize;
_12 = 43356_u16;
_11 = [_12,_12,_12,_12];
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (_6.0,);
_6.0 = _4.0 ^ _4.0;
_3 = !_9;
place!(Field::<(i16,)>(Variant(_8, 0), 0)).0 = RET as i16;
_4.0 = _6.0;
_13 = [RET,RET,RET];
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
_4 = _6;
_1 = _5;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
43356 => bb9,
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
_4.0 = -Field::<(i16,)>(Variant(_8, 0), 0).0;
_10 = Adt51 { fld0: _13,fld1: Field::<(i16,)>(Variant(_8, 0), 0).0 };
_7 = -_10.fld1;
_14 = _10.fld1 >= Field::<(i16,)>(Variant(_8, 0), 0).0;
_11 = [_12,_12,_12,_12];
_7 = -_6.0;
_7 = _4.0 + _4.0;
RET = Field::<(i16,)>(Variant(_8, 0), 0).0 as i128;
_9 = _3;
_12 = 63126_u16 & 14506_u16;
RET = 2994071860_u32 as i128;
_14 = !_9;
Goto(bb10)
}
bb10 = {
place!(Field::<(i16,)>(Variant(_8, 0), 0)).0 = _6.0 >> _7;
_17 = Field::<char>(Variant(_8, 0), 1);
_16 = _12 * _12;
_18 = 171_u8 as f64;
_11 = [_16,_16,_16,_16];
RET = -(-109849898747687780858188542981248611121_i128);
_6 = (Field::<(i16,)>(Variant(_8, 0), 0).0,);
place!(Field::<(i16,)>(Variant(_8, 0), 0)).0 = !_6.0;
_2 = Field::<char>(Variant(_8, 0), 1);
_10.fld1 = -Field::<(i16,)>(Variant(_8, 0), 0).0;
_17 = Field::<char>(Variant(_8, 0), 1);
_3 = !_9;
_10.fld0 = [RET,RET,RET];
_12 = _16 ^ _16;
RET = -(-110257243636175655062460070158463549991_i128);
Goto(bb11)
}
bb11 = {
_20 = Field::<char>(Variant(_8, 0), 1);
SetDiscriminant(_8, 0);
place!(Field::<char>(Variant(_8, 0), 1)) = _20;
_11 = [_12,_12,_12,_12];
_19 = Adt41::Variant1 { fld0: (-55_i8),fld1: (-2154375393679030145_i64) };
_1 = _5;
_7 = RET as i16;
_20 = _2;
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (_10.fld1,);
_4 = (_10.fld1,);
_19 = Adt41::Variant0 { fld0: _4,fld1: _17 };
_16 = !_12;
place!(Field::<(i16,)>(Variant(_8, 0), 0)).0 = _5 as i16;
_17 = Field::<char>(Variant(_19, 0), 1);
Call(_9 = fn12(_12, _19, Field::<char>(Variant(_19, 0), 1), _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_21 = -(-1174833510_i32);
place!(Field::<(i16,)>(Variant(_19, 0), 0)) = _4;
_24 = 83_u8 - 115_u8;
place!(Field::<(i16,)>(Variant(_19, 0), 0)).0 = _24 as i16;
_6 = Field::<(i16,)>(Variant(_8, 0), 0);
_6 = (_10.fld1,);
_25 = _1;
Goto(bb13)
}
bb13 = {
SetDiscriminant(_19, 0);
SetDiscriminant(_8, 0);
RET = (-158933239326014131103325119313668883003_i128) * (-113066423902160790085565675635344488866_i128);
_5 = -_1;
_24 = 19_u8 + 221_u8;
_8 = Adt41::Variant0 { fld0: _6,fld1: _17 };
SetDiscriminant(_8, 0);
place!(Field::<(i16,)>(Variant(_19, 0), 0)).0 = _18 as i16;
_6.0 = _7;
_28.0 = -_10.fld1;
_15 = 233159560044732589032958441664364934198_u128 & 148567620932454582959269963846693151782_u128;
_11 = [_16,_16,_12,_12];
_6.0 = 12132681255974185600_u64 as i16;
place!(Field::<char>(Variant(_8, 0), 1)) = _2;
_27 = 112_i8 >> _12;
_4 = (_7,);
RET = 108734601939342381581135664958936000115_i128 - (-104582308832571669828130512652189237544_i128);
_22 = Adt41::Variant0 { fld0: _28,fld1: _17 };
Goto(bb14)
}
bb14 = {
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (Field::<(i16,)>(Variant(_22, 0), 0).0,);
_1 = -_5;
_26 = [RET,RET,RET];
SetDiscriminant(_8, 0);
place!(Field::<(i16,)>(Variant(_8, 0), 0)) = (_6.0,);
place!(Field::<(i16,)>(Variant(_22, 0), 0)).0 = (-404103875337168099_i64) as i16;
place!(Field::<char>(Variant(_22, 0), 1)) = _2;
_9 = !_3;
_22 = Adt41::Variant0 { fld0: Field::<(i16,)>(Variant(_8, 0), 0),fld1: _2 };
_13 = [RET,RET,RET];
_16 = 2_usize as u16;
_25 = 15993727883826551326_u64 as isize;
_12 = _16;
_5 = -_1;
_10.fld1 = Field::<(i16,)>(Variant(_8, 0), 0).0 ^ _28.0;
_2 = _17;
_4 = Field::<(i16,)>(Variant(_22, 0), 0);
_32.0 = RET - RET;
_15 = 164318279390778978499282969909193290148_u128 << Field::<(i16,)>(Variant(_8, 0), 0).0;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(11_usize, 16_usize, Move(_16), 26_usize, Move(_26), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(11_usize, 25_usize, Move(_25), 14_usize, Move(_14), 9_usize, Move(_9), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(11_usize, 21_usize, Move(_21), 3_usize, Move(_3), 27_usize, Move(_27), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u16,mut _2: Adt41,mut _3: char,mut _4: u16) -> bool {
mir! {
type RET = bool;
let _5: isize;
let _6: isize;
let _7: Adt51;
let _8: isize;
let _9: [u16; 4];
let _10: usize;
let _11: char;
let _12: isize;
let _13: bool;
let _14: usize;
let _15: Adt41;
let _16: f64;
let _17: Adt37;
let _18: [u16; 4];
let _19: i8;
let _20: bool;
let _21: isize;
let _22: Adt51;
let _23: [u16; 4];
let _24: Adt37;
let _25: (isize, usize, u16, &'static isize, (i16,), u128, (i16,));
let _26: isize;
let _27: *const *mut [char; 6];
let _28: f32;
let _29: bool;
let _30: Adt41;
let _31: u64;
let _32: f32;
let _33: u16;
let _34: f64;
let _35: ();
let _36: ();
{
place!(Field::<(i16,)>(Variant(_2, 0), 0)).0 = false as i16;
_3 = Field::<char>(Variant(_2, 0), 1);
_1 = _4;
RET = !false;
place!(Field::<char>(Variant(_2, 0), 1)) = _3;
_1 = _4;
place!(Field::<char>(Variant(_2, 0), 1)) = _3;
RET = false;
place!(Field::<char>(Variant(_2, 0), 1)) = _3;
_3 = Field::<char>(Variant(_2, 0), 1);
_2 = Adt41::Variant1 { fld0: (-125_i8),fld1: 1197602127567616117_i64 };
place!(Field::<i8>(Variant(_2, 1), 0)) = (-4_i8) & 119_i8;
RET = false;
RET = _1 >= _4;
RET = _1 > _1;
_6 = 9223372036854775807_isize * (-9223372036854775808_isize);
RET = !true;
_1 = _4;
_2 = Adt41::Variant1 { fld0: (-99_i8),fld1: 6722484147058852423_i64 };
place!(Field::<i8>(Variant(_2, 1), 0)) = 6_i8 ^ 35_i8;
Goto(bb1)
}
bb1 = {
RET = false ^ false;
place!(Field::<i8>(Variant(_2, 1), 0)) = 44_i8;
_6 = (-96_isize);
_5 = _6 >> _4;
_7.fld1 = -(-14874_i16);
_4 = _1;
_6 = _5;
_1 = _4 | _4;
_7.fld1 = 11161_i16 | 29440_i16;
_4 = _1 & _1;
_4 = Field::<i8>(Variant(_2, 1), 0) as u16;
_7.fld1 = Field::<i8>(Variant(_2, 1), 0) as i16;
_7.fld1 = (-557626747_i32) as i16;
_2 = Adt41::Variant1 { fld0: (-108_i8),fld1: 5758707417440902563_i64 };
_4 = _1;
_7.fld1 = 7719_i16 - 17365_i16;
Goto(bb2)
}
bb2 = {
place!(Field::<i64>(Variant(_2, 1), 1)) = -8072774821596383515_i64;
place!(Field::<i64>(Variant(_2, 1), 1)) = (-6713976341681378580_i64) * (-8404627626823446874_i64);
_2 = Adt41::Variant1 { fld0: 35_i8,fld1: 5564990948281316275_i64 };
_3 = '\u{d1c2d}';
_1 = _4 << _5;
_10 = !12166418916734097087_usize;
_7.fld0 = [(-156548480496824388606028470585349350916_i128),32008216395057512542325240353162646199_i128,(-166559081813064397436180548840580449473_i128)];
_9 = [_4,_4,_1,_1];
_8 = !_5;
RET = true ^ false;
_1 = !_4;
_8 = _7.fld1 as isize;
_12 = _3 as isize;
_6 = _5;
place!(Field::<i8>(Variant(_2, 1), 0)) = (-88_i8) * 35_i8;
_1 = !_4;
Goto(bb3)
}
bb3 = {
_9 = [_4,_1,_1,_1];
place!(Field::<i64>(Variant(_2, 1), 1)) = (-2625824185464685860_i64);
_5 = !_6;
_13 = !RET;
_12 = _5 << _4;
_1 = _4 << _6;
_7.fld0 = [57748068752454660249543291970195737930_i128,25307006990716241971085339003533513553_i128,145922896463578780905799420032494923681_i128];
_6 = _7.fld1 as isize;
_9 = [_4,_1,_4,_4];
place!(Field::<i8>(Variant(_2, 1), 0)) = (-111_i8);
_11 = _3;
_1 = Field::<i8>(Variant(_2, 1), 0) as u16;
_14 = Field::<i8>(Variant(_2, 1), 0) as usize;
_7.fld1 = Field::<i8>(Variant(_2, 1), 0) as i16;
_4 = Field::<i64>(Variant(_2, 1), 1) as u16;
_2 = Adt41::Variant1 { fld0: 94_i8,fld1: (-5945902605315435424_i64) };
_11 = _3;
_10 = (-126127840218758754868356365133711848490_i128) as usize;
_7.fld0 = [(-132569779943699453220827685080036543003_i128),64549071522894600416938967611720073340_i128,(-36631643004981434948320931823796002136_i128)];
_15 = Adt41::Variant1 { fld0: 47_i8,fld1: (-4549637090197968291_i64) };
place!(Field::<i64>(Variant(_15, 1), 1)) = (-3384269141121116142_i64) - (-712630535098682556_i64);
place!(Field::<i8>(Variant(_15, 1), 0)) = 28_i8 + 68_i8;
_11 = _3;
_4 = _1;
_13 = RET;
_4 = _1;
RET = _13;
Goto(bb4)
}
bb4 = {
SetDiscriminant(_15, 1);
_4 = _1;
RET = !_13;
_13 = !RET;
_11 = _3;
_15 = Adt41::Variant1 { fld0: (-87_i8),fld1: (-1368087352064571457_i64) };
_4 = !_1;
_12 = _5 >> _5;
place!(Field::<i8>(Variant(_15, 1), 0)) = _11 as i8;
_1 = 213953728397275997026265011654028740100_u128 as u16;
Goto(bb5)
}
bb5 = {
_16 = Field::<i8>(Variant(_15, 1), 0) as f64;
place!(Field::<i64>(Variant(_15, 1), 1)) = (-1362831714633874619_i64) | (-7346550176444570330_i64);
Goto(bb6)
}
bb6 = {
place!(Field::<i64>(Variant(_2, 1), 1)) = 87873719742843216169290044300421361728_i128 as i64;
Call(_4 = fn13(_7, _12, _5, _9, _12, _5, _7.fld1, _5, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = _12;
_1 = _4;
_17.fld3 = _16 as i8;
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
place!(Field::<i8>(Variant(_15, 1), 0)) = _7.fld1 as i8;
_17.fld2.fld1 = _3;
_17.fld0 = -Field::<i64>(Variant(_15, 1), 1);
_9 = [_1,_4,_1,_1];
_21 = -_12;
_7.fld0 = [(-148035203641985660280214243853515184256_i128),88014488069182131306525761804308717519_i128,(-64617121673908044574921531900970434195_i128)];
_17.fld2.fld0 = 128689147031261148602300868476166274177_u128;
_1 = _4 >> _12;
_17.fld3 = Field::<i8>(Variant(_15, 1), 0) - Field::<i8>(Variant(_15, 1), 0);
Goto(bb10)
}
bb10 = {
_3 = _17.fld2.fld1;
_14 = _4 as usize;
_12 = _21;
_24.fld2.fld2 = 4476710028518443438_u64 >> _4;
SetDiscriminant(_15, 1);
_24.fld2.fld5 = 3530548553_u32 ^ 228548374_u32;
_4 = _1;
_10 = _17.fld3 as usize;
_17.fld2.fld2 = _24.fld2.fld2;
_17.fld2.fld4 = _7.fld1;
_17.fld1 = (-368611483_i32) & (-1922476274_i32);
_3 = _17.fld2.fld1;
_11 = _17.fld2.fld1;
_17.fld2.fld1 = _3;
_13 = !RET;
_9 = [_4,_4,_1,_4];
_6 = _17.fld2.fld2 as isize;
_24.fld2.fld6 = core::ptr::addr_of_mut!(_10);
_7.fld0 = [(-44760983794478020193857072394732355796_i128),(-85542407784492798771454748934138663589_i128),98454936759559138448459913401416880687_i128];
_11 = _3;
_21 = _6 * _6;
_19 = _17.fld3 >> _17.fld2.fld2;
_27 = core::ptr::addr_of!(_24.fld2.fld3);
_17.fld2.fld4 = _7.fld1 & _7.fld1;
_22.fld0 = [56596292899022558328343907105372933724_i128,(-79350146079008617472382587739513162258_i128),53934642739709868908371558025255808394_i128];
_3 = _17.fld2.fld1;
_17.fld2.fld0 = !304968495424085874888276286676770639743_u128;
Goto(bb11)
}
bb11 = {
_15 = Adt41::Variant1 { fld0: _19,fld1: _17.fld0 };
place!(Field::<i8>(Variant(_2, 1), 0)) = -_19;
_25.0 = _17.fld2.fld2 as isize;
place!(Field::<i8>(Variant(_15, 1), 0)) = -Field::<i8>(Variant(_2, 1), 0);
_22.fld0 = [(-104178878770671655188302048218469849091_i128),2485719141319440288477084667153930001_i128,15721592858065415973263114392765739758_i128];
Goto(bb12)
}
bb12 = {
_25.2 = _1 - _4;
_24.fld0 = _16 as i64;
_24.fld2.fld1 = _11;
_20 = !RET;
_24.fld2.fld0 = _24.fld0 as u128;
_25.6 = (_17.fld2.fld4,);
_24.fld1 = _25.6.0 as i32;
_24.fld2.fld6 = core::ptr::addr_of_mut!(_14);
_24.fld2.fld4 = -_17.fld2.fld4;
_24.fld3 = _24.fld2.fld5 as i8;
_17.fld2.fld5 = !_24.fld2.fld5;
SetDiscriminant(_2, 1);
_17.fld0 = Field::<i64>(Variant(_15, 1), 1) - Field::<i64>(Variant(_15, 1), 1);
_17.fld2.fld6 = _24.fld2.fld6;
_27 = core::ptr::addr_of!(_24.fld2.fld3);
_25.5 = _24.fld2.fld0 | _24.fld2.fld0;
Call(_24.fld1 = fn17(_25.0, _21, _4, _24.fld2.fld6), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
SetDiscriminant(_15, 1);
_15 = Adt41::Variant0 { fld0: _25.6,fld1: _3 };
_25.6.0 = _7.fld1 - _24.fld2.fld4;
_27 = core::ptr::addr_of!(_17.fld2.fld3);
_18 = [_4,_1,_4,_25.2];
_23 = _9;
SetDiscriminant(_15, 0);
_17.fld3 = -_19;
_15 = Adt41::Variant1 { fld0: _19,fld1: _17.fld0 };
_21 = _25.0;
_17.fld2.fld4 = _17.fld1 as i16;
Goto(bb14)
}
bb14 = {
_32 = (-18892677009645267256535877568581891860_i128) as f32;
_25.1 = !_14;
_8 = -_6;
SetDiscriminant(_15, 0);
_21 = _17.fld2.fld2 as isize;
_23 = _18;
place!(Field::<i64>(Variant(_2, 1), 1)) = _17.fld0 | _17.fld0;
_8 = _6 << _17.fld2.fld2;
_20 = !RET;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(12_usize, 5_usize, Move(_5), 9_usize, Move(_9), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(12_usize, 11_usize, Move(_11), 4_usize, Move(_4), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: Adt51,mut _2: isize,mut _3: isize,mut _4: [u16; 4],mut _5: isize,mut _6: isize,mut _7: i16,mut _8: isize,mut _9: [u16; 4]) -> u16 {
mir! {
type RET = u16;
let _10: isize;
let _11: char;
let _12: f32;
let _13: Adt50;
let _14: (i16,);
let _15: isize;
let _16: isize;
let _17: u64;
let _18: usize;
let _19: [char; 6];
let _20: f32;
let _21: (((isize, usize, u16, &'static isize, (i16,), u128, (i16,)), usize, &'static isize), *mut (i16,), (&'static isize, isize, *mut usize, i32));
let _22: Adt51;
let _23: i8;
let _24: Adt41;
let _25: u16;
let _26: f64;
let _27: f64;
let _28: (i128, (&'static isize, isize, *mut usize, i32), i64);
let _29: i8;
let _30: [i128; 3];
let _31: isize;
let _32: ();
let _33: ();
{
RET = 616203390_u32 as u16;
_7 = _1.fld1 << _5;
Goto(bb1)
}
bb1 = {
_3 = _7 as isize;
_5 = 106_u8 as isize;
_1.fld1 = 9773754374253376784_u64 as i16;
_11 = '\u{8dcb9}';
_1.fld1 = _7;
_3 = 15891784263959378287_u64 as isize;
RET = 19875_u16 << _6;
_1.fld1 = _7 + _7;
_1.fld1 = _7;
RET = !52588_u16;
_1.fld1 = _7 | _7;
_7 = 209888353125767011662323213719526545188_u128 as i16;
_2 = _8 - _6;
_9 = [RET,RET,RET,RET];
_10 = _2;
Call(_4 = fn14(_10, _10, _1.fld1, _1, _1, _6, _6, _2, _1, _2, _10, _2, _6, _1.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _10 | _6;
_9 = [RET,RET,RET,RET];
Goto(bb3)
}
bb3 = {
RET = 8435_u16;
RET = 26292_u16;
_8 = _2;
_3 = !_6;
_9 = _4;
_13 = Adt50::Variant2 { fld0: (-76935936810038632_i64) };
_2 = _8;
_6 = _8 & _8;
_10 = _6 + _6;
_5 = (-73385107658200145035991389147465066815_i128) as isize;
_3 = _2 - _6;
_11 = '\u{96a2b}';
_14 = (_7,);
place!(Field::<i64>(Variant(_13, 2), 0)) = 5_usize as i64;
RET = 62990_u16;
_8 = _10;
_5 = _3 * _3;
RET = (-163544975325098083606914282922130353098_i128) as u16;
_5 = !_10;
_3 = Field::<i64>(Variant(_13, 2), 0) as isize;
_3 = false as isize;
_12 = Field::<i64>(Variant(_13, 2), 0) as f32;
_10 = Field::<i64>(Variant(_13, 2), 0) as isize;
RET = 48333_u16;
Goto(bb4)
}
bb4 = {
_11 = '\u{cf384}';
RET = 15941_u16;
_12 = (-76_i8) as f32;
_3 = _8;
_1.fld0 = [(-168795539551498508024653024784116343016_i128),(-101531399860085115317139546144696505228_i128),60565149752491076996618056504638291067_i128];
RET = 10341350092117952456_u64 as u16;
SetDiscriminant(_13, 0);
place!(Field::<(i16,)>(Variant(_13, 0), 0)) = (_1.fld1,);
place!(Field::<(i16,)>(Variant(_13, 0), 0)).0 = _1.fld1;
_3 = !_8;
_14.0 = _12 as i16;
_14 = Field::<(i16,)>(Variant(_13, 0), 0);
Goto(bb5)
}
bb5 = {
_16 = _8 - _8;
place!(Field::<(i16,)>(Variant(_13, 0), 0)) = (_1.fld1,);
_14 = Field::<(i16,)>(Variant(_13, 0), 0);
RET = !49262_u16;
RET = _11 as u16;
_19 = [_11,_11,_11,_11,_11,_11];
_15 = !_5;
_17 = 194621553329220852_u64;
_14.0 = Field::<(i16,)>(Variant(_13, 0), 0).0;
_9 = _4;
_18 = !5_usize;
_7 = _1.fld1 >> _16;
place!(Field::<(i16,)>(Variant(_13, 0), 0)) = (_14.0,);
RET = _12 as u16;
place!(Field::<(i16,)>(Variant(_13, 0), 0)).0 = -_7;
_14.0 = -Field::<(i16,)>(Variant(_13, 0), 0).0;
_20 = (-39_i8) as f32;
place!(Field::<(i16,)>(Variant(_13, 0), 0)).0 = -_7;
_6 = _12 as isize;
Goto(bb6)
}
bb6 = {
_21.0.0.3 = &_21.2.1;
_21.1 = core::ptr::addr_of_mut!(place!(Field::<(i16,)>(Variant(_13, 0), 0)));
_21.2.2 = core::ptr::addr_of_mut!(_21.0.0.1);
_21.0.0.0 = _11 as isize;
_3 = !_8;
_7 = _1.fld1 >> _3;
_21.0.0.5 = 228426047809030193052828295265954294485_u128 & 56769559214153906781185868417829529876_u128;
_12 = _20 * _20;
Call(_22.fld1 = core::intrinsics::bswap(Field::<(i16,)>(Variant(_13, 0), 0).0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_21.0.2 = &_2;
_16 = _3 + _8;
_21.0.0.4.0 = !Field::<(i16,)>(Variant(_13, 0), 0).0;
_22.fld1 = -_14.0;
_21.0.0.3 = &_2;
RET = !28159_u16;
_20 = -_12;
_21.0.0.6.0 = !_21.0.0.4.0;
_5 = -_3;
_13 = Adt50::Variant1 { fld0: _4 };
_8 = _11 as isize;
_22.fld1 = _21.0.0.4.0;
_21.0.1 = _18 * _18;
_21.0.0 = (_2, _18, RET, Move(_21.0.2), _14, 213284360116317738845282669583613003315_u128, _14);
_1.fld0 = [34725686710090599321432143964086827751_i128,(-151311456687357602063105705346353748878_i128),(-35155350099901567414120911096248130032_i128)];
_3 = -_15;
_20 = -_12;
_28.1.2 = core::ptr::addr_of_mut!(_21.0.1);
_21.0.0.6.0 = _7;
_10 = _5 >> _15;
place!(Field::<[u16; 4]>(Variant(_13, 1), 0)) = [_21.0.0.2,_21.0.0.2,RET,_21.0.0.2];
_28.1.3 = (-760782635_i32) + 392942135_i32;
_21.0.0.4.0 = !_21.0.0.6.0;
_28.1.3 = 697810442_i32;
_29 = _20 as i8;
Goto(bb8)
}
bb8 = {
_7 = _22.fld1;
_24 = Adt41::Variant1 { fld0: _29,fld1: (-2823692416224384629_i64) };
_18 = _21.0.1;
_17 = 14060391259800221695_u64 & 18341483598389839531_u64;
_21.0.0.4.0 = _21.0.0.6.0 - _22.fld1;
_12 = -_20;
_18 = _17 as usize;
RET = _21.0.0.2 >> _22.fld1;
_22 = Adt51 { fld0: _1.fld0,fld1: _7 };
Goto(bb9)
}
bb9 = {
Call(_32 = dump_var(13_usize, 18_usize, Move(_18), 29_usize, Move(_29), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_32 = dump_var(13_usize, 10_usize, Move(_10), 16_usize, Move(_16), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: isize,mut _3: i16,mut _4: Adt51,mut _5: Adt51,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: Adt51,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: i16) -> [u16; 4] {
mir! {
type RET = [u16; 4];
let _15: *const &'static isize;
let _16: i8;
let _17: i128;
let _18: f32;
let _19: f32;
let _20: (isize, usize, u16, &'static isize, (i16,), u128, (i16,));
let _21: [i128; 3];
let _22: (u32, &'static isize, *const &'static isize, i32, f64);
let _23: [i64; 1];
let _24: Adt38;
let _25: char;
let _26: [i128; 3];
let _27: Adt51;
let _28: Adt35;
let _29: Adt36;
let _30: Adt41;
let _31: i8;
let _32: Adt41;
let _33: f64;
let _34: i16;
let _35: Adt41;
let _36: bool;
let _37: char;
let _38: isize;
let _39: u64;
let _40: f32;
let _41: (&'static isize, isize, *mut usize, i32);
let _42: f32;
let _43: Adt45;
let _44: ();
let _45: ();
{
_1 = -_7;
_4.fld0 = [(-47126843269008103776284908842611539423_i128),(-106509211887707401510952460106810799813_i128),(-11439493003979090152191818782369904524_i128)];
_4 = Adt51 { fld0: _5.fld0,fld1: _9.fld1 };
_4.fld1 = _14;
_4.fld0 = [(-127352970670836727837720220986013102809_i128),(-67702193341749837853018532664918887794_i128),(-156612804466003335093163365101417157467_i128)];
_1 = _2 >> _11;
_7 = _6;
_9.fld0 = [(-153019614537401490777790796840252175438_i128),75202023169910115086596185023730849729_i128,(-116999255985599163791059321521714493105_i128)];
_5.fld1 = _14 | _14;
_5 = Adt51 { fld0: _4.fld0,fld1: _9.fld1 };
_5.fld0 = [40887676215557772535624672695203199871_i128,(-63651410127632164091372891053328224708_i128),111913592589277216035815745141760616067_i128];
_5.fld0 = [1820815201847279379803134222682404286_i128,27313798437299099238441831197193543642_i128,14081105226566341613075531892345695168_i128];
_9.fld0 = [113869869850064810848176313625571239802_i128,(-140977338503022843391021986932211596007_i128),(-12901319539460943069393591321445997157_i128)];
_9.fld0 = _5.fld0;
_14 = _5.fld1;
RET = [4121_u16,55772_u16,1320_u16,55043_u16];
_9 = _5;
_16 = (-78_i8) >> _7;
RET = [18086_u16,18583_u16,39613_u16,3715_u16];
_7 = -_2;
_9 = Adt51 { fld0: _5.fld0,fld1: _3 };
_7 = _11 >> _11;
_17 = 242_u8 as i128;
RET = [11041_u16,14580_u16,13383_u16,48953_u16];
Goto(bb1)
}
bb1 = {
_2 = _7 | _1;
_5.fld1 = !_14;
_13 = !_2;
_1 = 93_u8 as isize;
_8 = (-632649600_i32) as isize;
_12 = _11 - _13;
_4 = Adt51 { fld0: _5.fld0,fld1: _14 };
_7 = _2;
_9 = _4;
_9.fld1 = _3;
RET = [18820_u16,56928_u16,1908_u16,48876_u16];
_19 = 8940453099062233485_u64 as f32;
_11 = _7;
_3 = 8004352672818554482_i64 as i16;
_18 = 7_usize as f32;
_1 = _2 >> _11;
_5.fld1 = 43737_u16 as i16;
_8 = 36690_u16 as isize;
_12 = _13 | _10;
_20.3 = &_12;
_20.4 = (_14,);
_20.6.0 = _20.4.0 | _9.fld1;
Goto(bb2)
}
bb2 = {
_20.2 = 1754124408_u32 as u16;
_9.fld1 = _19 as i16;
_20.4 = _20.6;
_22.1 = &_10;
_9.fld1 = _14 ^ _20.6.0;
_9 = Adt51 { fld0: _4.fld0,fld1: _14 };
_17 = _4.fld1 as i128;
_20.0 = !_13;
RET = [_20.2,_20.2,_20.2,_20.2];
_25 = '\u{18d87}';
_8 = _12 + _10;
_7 = _8;
_22.3 = 50992345_i32 - 1129407816_i32;
_5 = _4;
_18 = -_19;
_22.3 = _20.2 as i32;
_20.1 = 10736204738579042396_usize >> _14;
_23 = [(-5922526411541493726_i64)];
_4.fld0 = [_17,_17,_17];
Call(_6 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_25 = '\u{105696}';
_9.fld0 = [_17,_17,_17];
_21 = [_17,_17,_17];
_20.6 = _20.4;
_4.fld0 = [_17,_17,_17];
_27.fld1 = _20.4.0;
_3 = -_27.fld1;
_22.0 = 100402163_u32;
_5.fld0 = [_17,_17,_17];
_5 = Adt51 { fld0: _9.fld0,fld1: _3 };
_16 = !86_i8;
_13 = !_1;
_26 = _5.fld0;
_28.fld5 = [_25,_25,_25,_25,_25,_25];
_28.fld1 = _25;
_4 = Adt51 { fld0: _5.fld0,fld1: _27.fld1 };
_28.fld6.0 = _3 << _7;
_6 = -_12;
_13 = _17 as isize;
_4.fld0 = [_17,_17,_17];
_26 = _4.fld0;
_22.3 = 1778667081_i32;
Goto(bb4)
}
bb4 = {
_22.2 = core::ptr::addr_of!(_20.3);
_9.fld1 = _20.4.0 >> _3;
_28.fld4 = core::ptr::addr_of!(_29.fld3);
_8 = _6;
_22.2 = core::ptr::addr_of!(_20.3);
_26 = _5.fld0;
_28.fld6.0 = false as i16;
_28.fld6 = (_5.fld1,);
_14 = _3;
_27 = _5;
_13 = _6 - _2;
_29.fld4 = _28.fld6.0;
_13 = !_11;
_20 = (_11, 12032943979029713052_usize, 19168_u16, Move(_22.1), _28.fld6, 247256399306690374904203729955913997140_u128, _28.fld6);
_29.fld0 = !_20.5;
_12 = _16 as isize;
_28.fld6.0 = -_29.fld4;
_19 = _16 as f32;
Goto(bb5)
}
bb5 = {
_29.fld3 = core::ptr::addr_of_mut!(_28.fld5);
_24 = Adt38::Variant1 { fld0: _20.2,fld1: 8596134879126377580_u64,fld2: 5636248126426117215_i64,fld3: _20.6,fld4: 10_u8,fld5: _20.1 };
_29.fld6 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_24, 1), 5)));
_29.fld0 = !_20.5;
_22.1 = &_10;
_30 = Adt41::Variant0 { fld0: _20.4,fld1: _28.fld1 };
_9.fld1 = !Field::<(i16,)>(Variant(_30, 0), 0).0;
_26 = [_17,_17,_17];
_10 = -_6;
_20.0 = _11;
_28.fld6.0 = _9.fld1;
_29.fld5 = _22.0;
_7 = -_1;
_28.fld0 = !false;
_9.fld1 = _3 | Field::<(i16,)>(Variant(_30, 0), 0).0;
place!(Field::<(i16,)>(Variant(_30, 0), 0)).0 = -_29.fld4;
SetDiscriminant(_30, 1);
_29.fld4 = !_4.fld1;
place!(Field::<i64>(Variant(_30, 1), 1)) = -(-457728977625533435_i64);
_2 = -_13;
_13 = !_8;
_28.fld6.0 = !_3;
Goto(bb6)
}
bb6 = {
_34 = 118_u8 as i16;
_6 = _22.0 as isize;
_28.fld1 = _25;
_4.fld1 = _25 as i16;
place!(Field::<(i16,)>(Variant(_24, 1), 3)) = _20.6;
_32 = Adt41::Variant0 { fld0: Field::<(i16,)>(Variant(_24, 1), 3),fld1: _28.fld1 };
_1 = _13;
_5.fld0 = _9.fld0;
_29.fld5 = _22.0 ^ _22.0;
_22.4 = 237_u8 as f64;
_28.fld0 = true;
_11 = !_10;
_31 = !_16;
_28.fld7 = Field::<usize>(Variant(_24, 1), 5) >> _14;
_33 = _22.4;
_36 = !_28.fld0;
_29.fld3 = core::ptr::addr_of_mut!(_28.fld5);
_23 = [Field::<i64>(Variant(_30, 1), 1)];
_15 = core::ptr::addr_of!(_20.3);
SetDiscriminant(_32, 1);
_35 = Adt41::Variant1 { fld0: _16,fld1: Field::<i64>(Variant(_30, 1), 1) };
Call(place!(Field::<u64>(Variant(_24, 1), 1)) = fn15(_11, _20.2, _28.fld6, _3, _20.2, _27.fld0, Field::<usize>(Variant(_24, 1), 5), _10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_19 = _18;
_22.0 = _29.fld5 - _29.fld5;
_20.1 = _17 as usize;
place!(Field::<i8>(Variant(_30, 1), 0)) = _25 as i8;
Call(RET = fn16(_27, _20.6.0, Field::<u16>(Variant(_24, 1), 0), _30, _29.fld6, _14, _11, _3, _36, _20.5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_41.2 = core::ptr::addr_of_mut!(_28.fld7);
_4.fld1 = -_9.fld1;
_3 = -_9.fld1;
_3 = _28.fld0 as i16;
place!(Field::<(i16,)>(Variant(_24, 1), 3)).0 = _29.fld4 >> _14;
_29.fld2 = Field::<u64>(Variant(_24, 1), 1) * Field::<u64>(Variant(_24, 1), 1);
SetDiscriminant(_30, 0);
_4.fld0 = [_17,_17,_17];
_39 = Field::<u64>(Variant(_24, 1), 1);
_33 = -_22.4;
_20.1 = !Field::<usize>(Variant(_24, 1), 5);
_39 = _22.3 as u64;
Goto(bb9)
}
bb9 = {
Call(_44 = dump_var(14_usize, 10_usize, Move(_10), 25_usize, Move(_25), 14_usize, Move(_14), 39_usize, Move(_39)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_44 = dump_var(14_usize, 11_usize, Move(_11), 36_usize, Move(_36), 23_usize, Move(_23), 3_usize, Move(_3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_44 = dump_var(14_usize, 7_usize, Move(_7), 6_usize, Move(_6), 45_usize, _45, 45_usize, _45), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: u16,mut _3: (i16,),mut _4: i16,mut _5: u16,mut _6: [i128; 3],mut _7: usize,mut _8: isize) -> u64 {
mir! {
type RET = u64;
let _9: Adt51;
let _10: f32;
let _11: f64;
let _12: isize;
let _13: Adt49;
let _14: bool;
let _15: f32;
let _16: f32;
let _17: isize;
let _18: f32;
let _19: [char; 6];
let _20: ();
let _21: ();
{
_5 = _2;
_5 = _2;
_9.fld0 = _6;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
19168 => bb5,
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
_8 = _1 ^ _1;
RET = !10505023673369795309_u64;
_2 = _5 + _5;
_4 = !_3.0;
_7 = 9630007220645838909_usize;
_9.fld1 = !_4;
_11 = _2 as f64;
_12 = _8 & _1;
_11 = 319903181307339387984570292095428118028_u128 as f64;
_3.0 = _4;
_6 = _9.fld0;
_9.fld1 = !_3.0;
_9.fld1 = _4;
_6 = _9.fld0;
_8 = '\u{2ad73}' as isize;
Goto(bb6)
}
bb6 = {
_15 = _11 as f32;
RET = 8357871096802941634_u64 << _5;
_6 = _9.fld0;
_8 = !_12;
_1 = _8;
_8 = _12;
_4 = _2 as i16;
_3.0 = !_4;
_6 = _9.fld0;
_5 = 133_u8 as u16;
_14 = !false;
_3.0 = -_4;
Goto(bb7)
}
bb7 = {
Call(_20 = dump_var(15_usize, 14_usize, Move(_14), 3_usize, Move(_3), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_20 = dump_var(15_usize, 2_usize, Move(_2), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: Adt51,mut _2: i16,mut _3: u16,mut _4: Adt41,mut _5: *mut usize,mut _6: i16,mut _7: isize,mut _8: i16,mut _9: bool,mut _10: u128) -> [u16; 4] {
mir! {
type RET = [u16; 4];
let _11: isize;
let _12: Adt47;
let _13: u8;
let _14: u128;
let _15: Adt38;
let _16: i32;
let _17: Adt51;
let _18: u16;
let _19: [char; 6];
let _20: isize;
let _21: i64;
let _22: ();
let _23: ();
{
_10 = 280114182671496640576649991440428483929_u128;
_8 = -_6;
_8 = _2 << _6;
place!(Field::<i8>(Variant(_4, 1), 0)) = _3 as i8;
_3 = _10 as u16;
RET = [_3,_3,_3,_3];
_6 = _8;
RET = [_3,_3,_3,_3];
RET = [_3,_3,_3,_3];
place!(Field::<i8>(Variant(_4, 1), 0)) = !51_i8;
match (*_5) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
12032943979029713052 => bb8,
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
place!(Field::<i8>(Variant(_4, 1), 0)) = _9 as i8;
_9 = !false;
RET = [_3,_3,_3,_3];
match (*_5) {
0 => bb9,
12032943979029713052 => bb11,
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
_3 = _1.fld1 as u16;
_17.fld0 = _1.fld0;
_18 = !_3;
_18 = _3 | _3;
_18 = _3;
SetDiscriminant(_4, 1);
_1 = Adt51 { fld0: _17.fld0,fld1: _2 };
_4 = Adt41::Variant1 { fld0: 22_i8,fld1: (-5560181307484587075_i64) };
_17.fld0 = [(-114215360508818567166265249565660835779_i128),(-47851047743967348570923120397426235326_i128),(-91132648290707282873943667924249441803_i128)];
place!(Field::<i8>(Variant(_4, 1), 0)) = (-49_i8) & (-65_i8);
_17 = _1;
_16 = (-1459001760_i32) * 1010601432_i32;
_19 = ['\u{b3cb8}','\u{d43fc}','\u{1f424}','\u{30554}','\u{2d5e}','\u{a1003}'];
match (*_5) {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
12032943979029713052 => bb16,
_ => bb15
}
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
place!(Field::<i8>(Variant(_4, 1), 0)) = -64_i8;
_13 = 145776754177754502094085673269201645790_i128 as u8;
_1.fld0 = [(-43738827231879082871304341684727693557_i128),51104289473930887585941298870431986194_i128,62450781906128727387360844920563819588_i128];
_9 = true;
_3 = _18 - _18;
_9 = true;
_17 = Adt51 { fld0: _1.fld0,fld1: _8 };
RET = [_3,_3,_18,_3];
Goto(bb17)
}
bb17 = {
Call(_22 = dump_var(16_usize, 7_usize, Move(_7), 2_usize, Move(_2), 10_usize, Move(_10), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_22 = dump_var(16_usize, 13_usize, Move(_13), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: u16,mut _4: *mut usize) -> i32 {
mir! {
type RET = i32;
let _5: Adt51;
let _6: (i16,);
let _7: u64;
let _8: [u16; 4];
let _9: [i64; 1];
let _10: [u16; 4];
let _11: (i16,);
let _12: (u32, &'static isize, *const &'static isize, i32, f64);
let _13: [char; 6];
let _14: char;
let _15: u8;
let _16: char;
let _17: i32;
let _18: bool;
let _19: Adt41;
let _20: *mut usize;
let _21: f64;
let _22: Adt41;
let _23: Adt38;
let _24: [u16; 4];
let _25: Adt46;
let _26: u128;
let _27: ();
let _28: ();
{
_1 = _2 >> _2;
_4 = core::ptr::addr_of_mut!((*_4));
RET = 1705195493_i32 - (-2018232781_i32);
Goto(bb1)
}
bb1 = {
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = !9973419991355536123_usize;
_4 = core::ptr::addr_of_mut!((*_4));
_2 = _1;
_2 = _1 >> _3;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = 4_usize & 13133315894820750777_usize;
_1 = _2 * _2;
RET = (-45474024041924335167931060205982827374_i128) as i32;
_5.fld0 = [(-165762653482257237860155824471268302024_i128),104896228717587675105099080757482168226_i128,(-69257032935952873854603296429440990098_i128)];
_3 = 26720_u16;
_2 = _1 * _1;
_3 = (*_4) as u16;
_3 = !60758_u16;
(*_4) = !9975082020153643515_usize;
_1 = !_2;
_1 = _2 * _2;
(*_4) = 10498022665577604128_usize + 10743823939265187332_usize;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = 3_usize * 6_usize;
_1 = _2;
RET = 1622078234_i32 * (-689428111_i32);
_1 = -_2;
_2 = _1 + _1;
_4 = core::ptr::addr_of_mut!((*_4));
Call((*_4) = core::intrinsics::bswap(4_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = core::ptr::addr_of_mut!((*_4));
_5.fld1 = _2 as i16;
_5.fld1 = -(-15953_i16);
RET = (-2111187353_i32) * (-176914281_i32);
_7 = true as u64;
_5.fld1 = !4316_i16;
_6 = (_5.fld1,);
(*_4) = 3210811091_u32 as usize;
_7 = 10587122615347812189_u64 ^ 17889356561070993510_u64;
_4 = core::ptr::addr_of_mut!((*_4));
RET = _3 as i32;
_1 = _2 ^ _2;
_4 = core::ptr::addr_of_mut!((*_4));
_6.0 = _5.fld1 & _5.fld1;
_9 = [(-5211915428149201390_i64)];
Goto(bb3)
}
bb3 = {
_6.0 = _5.fld1 & _5.fld1;
RET = 485014569_i32 << _2;
_5.fld0 = [(-42552563402743151077933624621656920898_i128),114558815100231294264021211151699467111_i128,151352974350022976062739102478430081869_i128];
(*_4) = 3_usize ^ 2_usize;
_6 = (_5.fld1,);
_2 = -_1;
_7 = 5929318220900234953_u64;
_11.0 = '\u{293bd}' as i16;
_8 = [_3,_3,_3,_3];
_10 = [_3,_3,_3,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_11 = _6;
_5.fld0 = [(-95613741558620969829592814640225965851_i128),(-3981337372582675299557997099707247559_i128),124136575744143320824227417162886546655_i128];
_5.fld1 = _11.0 << _2;
_6.0 = -_5.fld1;
_2 = _5.fld1 as isize;
_2 = _1 | _1;
(*_4) = 15717510702587713864_usize;
_4 = core::ptr::addr_of_mut!((*_4));
_11 = (_6.0,);
_6.0 = _7 as i16;
RET = -(-208435339_i32);
_1 = _2;
_8 = [_3,_3,_3,_3];
_12.3 = -RET;
_5.fld0 = [111339363488242921419431876202898777357_i128,(-123212607298393956217471023397435425448_i128),158125211666900355867986864264649623485_i128];
_12.4 = _5.fld1 as f64;
Goto(bb4)
}
bb4 = {
_4 = core::ptr::addr_of_mut!((*_4));
_14 = '\u{37669}';
_4 = core::ptr::addr_of_mut!((*_4));
_13 = [_14,_14,_14,_14,_14,_14];
(*_4) = 0_usize - 3_usize;
_12.0 = 4049804010_u32;
_1 = RET as isize;
_15 = _12.4 as u8;
_12.2 = core::ptr::addr_of!(_12.1);
match _12.0 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
4049804010 => bb10,
_ => bb9
}
}
bb5 = {
_6.0 = _5.fld1 & _5.fld1;
RET = 485014569_i32 << _2;
_5.fld0 = [(-42552563402743151077933624621656920898_i128),114558815100231294264021211151699467111_i128,151352974350022976062739102478430081869_i128];
(*_4) = 3_usize ^ 2_usize;
_6 = (_5.fld1,);
_2 = -_1;
_7 = 5929318220900234953_u64;
_11.0 = '\u{293bd}' as i16;
_8 = [_3,_3,_3,_3];
_10 = [_3,_3,_3,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_11 = _6;
_5.fld0 = [(-95613741558620969829592814640225965851_i128),(-3981337372582675299557997099707247559_i128),124136575744143320824227417162886546655_i128];
_5.fld1 = _11.0 << _2;
_6.0 = -_5.fld1;
_2 = _5.fld1 as isize;
_2 = _1 | _1;
(*_4) = 15717510702587713864_usize;
_4 = core::ptr::addr_of_mut!((*_4));
_11 = (_6.0,);
_6.0 = _7 as i16;
RET = -(-208435339_i32);
_1 = _2;
_8 = [_3,_3,_3,_3];
_12.3 = -RET;
_5.fld0 = [111339363488242921419431876202898777357_i128,(-123212607298393956217471023397435425448_i128),158125211666900355867986864264649623485_i128];
_12.4 = _5.fld1 as f64;
Goto(bb4)
}
bb6 = {
_4 = core::ptr::addr_of_mut!((*_4));
_5.fld1 = _2 as i16;
_5.fld1 = -(-15953_i16);
RET = (-2111187353_i32) * (-176914281_i32);
_7 = true as u64;
_5.fld1 = !4316_i16;
_6 = (_5.fld1,);
(*_4) = 3210811091_u32 as usize;
_7 = 10587122615347812189_u64 ^ 17889356561070993510_u64;
_4 = core::ptr::addr_of_mut!((*_4));
RET = _3 as i32;
_1 = _2 ^ _2;
_4 = core::ptr::addr_of_mut!((*_4));
_6.0 = _5.fld1 & _5.fld1;
_9 = [(-5211915428149201390_i64)];
Goto(bb3)
}
bb7 = {
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = !9973419991355536123_usize;
_4 = core::ptr::addr_of_mut!((*_4));
_2 = _1;
_2 = _1 >> _3;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = 4_usize & 13133315894820750777_usize;
_1 = _2 * _2;
RET = (-45474024041924335167931060205982827374_i128) as i32;
_5.fld0 = [(-165762653482257237860155824471268302024_i128),104896228717587675105099080757482168226_i128,(-69257032935952873854603296429440990098_i128)];
_3 = 26720_u16;
_2 = _1 * _1;
_3 = (*_4) as u16;
_3 = !60758_u16;
(*_4) = !9975082020153643515_usize;
_1 = !_2;
_1 = _2 * _2;
(*_4) = 10498022665577604128_usize + 10743823939265187332_usize;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = 3_usize * 6_usize;
_1 = _2;
RET = 1622078234_i32 * (-689428111_i32);
_1 = -_2;
_2 = _1 + _1;
_4 = core::ptr::addr_of_mut!((*_4));
Call((*_4) = core::intrinsics::bswap(4_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_7 = !74785933632738855_u64;
_6 = _11;
_11.0 = _6.0 + _6.0;
_3 = 61999_u16;
_6.0 = !_5.fld1;
_11.0 = _14 as i16;
Goto(bb11)
}
bb11 = {
_12.3 = RET;
_12.2 = core::ptr::addr_of!(_12.1);
_15 = _2 as u8;
_5.fld1 = _6.0;
_12.3 = !RET;
_4 = core::ptr::addr_of_mut!((*_4));
_16 = _14;
_14 = _16;
_4 = core::ptr::addr_of_mut!((*_4));
_3 = 4729_u16 + 26938_u16;
_9 = [1904234195924782933_i64];
_13 = [_14,_16,_16,_16,_16,_14];
_3 = _15 as u16;
_9 = [(-8010751098800664748_i64)];
_3 = 57259_u16 - 17716_u16;
_19 = Adt41::Variant1 { fld0: 4_i8,fld1: 3883016038887684086_i64 };
_12.4 = 5670663275125570081_i64 as f64;
Call(place!(Field::<i64>(Variant(_19, 1), 1)) = fn18(_6, _2, _5.fld1, _6, _6.0, _2, _15, _10, _5.fld1, _6.0, _4, _2, _2, _5.fld0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_16 = _14;
place!(Field::<i8>(Variant(_19, 1), 0)) = (-36_i8) & 27_i8;
SetDiscriminant(_19, 1);
place!(Field::<i64>(Variant(_19, 1), 1)) = (-6098046499124350154_i64);
_13 = [_16,_14,_16,_16,_14,_16];
_9 = [Field::<i64>(Variant(_19, 1), 1)];
_17 = _12.3 >> _5.fld1;
_18 = true;
place!(Field::<i8>(Variant(_19, 1), 0)) = 71_i8 >> _17;
_17 = _12.3 - RET;
_9 = [Field::<i64>(Variant(_19, 1), 1)];
_12.0 = 301900355_u32 * 2236868749_u32;
_12.0 = !3466401973_u32;
(*_4) = 12849231642782384866_usize;
_20 = _4;
_5.fld0 = [120217883195974378082374467313458900634_i128,64658573437058125822183998177634477177_i128,(-144609292277538422651027628355350019187_i128)];
_14 = _16;
_21 = _12.4;
Goto(bb13)
}
bb13 = {
SetDiscriminant(_19, 1);
_2 = -_1;
_6 = (_5.fld1,);
_12.3 = _14 as i32;
_1 = -_2;
place!(Field::<i64>(Variant(_19, 1), 1)) = 90_i8 as i64;
_8 = [_3,_3,_3,_3];
_11.0 = !_6.0;
_11 = (_5.fld1,);
_15 = 323272062399311180706228385125823640770_u128 as u8;
place!(Field::<i64>(Variant(_19, 1), 1)) = _17 as i64;
_12.2 = core::ptr::addr_of!(_12.1);
_1 = _2 >> _6.0;
_5.fld0 = [(-2462083814117682351430565826014657702_i128),(-127242105793524862515091963070048332263_i128),151670149456591643386316541603620397853_i128];
_5.fld1 = -_11.0;
Goto(bb14)
}
bb14 = {
_19 = Adt41::Variant0 { fld0: _11,fld1: _14 };
_24 = _8;
_11 = (_5.fld1,);
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(17_usize, 8_usize, Move(_8), 13_usize, Move(_13), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(17_usize, 14_usize, Move(_14), 17_usize, Move(_17), 1_usize, Move(_1), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: (i16,),mut _2: isize,mut _3: i16,mut _4: (i16,),mut _5: i16,mut _6: isize,mut _7: u8,mut _8: [u16; 4],mut _9: i16,mut _10: i16,mut _11: *mut usize,mut _12: isize,mut _13: isize,mut _14: [i128; 3]) -> i64 {
mir! {
type RET = i64;
let _15: *mut usize;
let _16: [i64; 1];
let _17: [char; 6];
let _18: f64;
let _19: [char; 6];
let _20: Adt41;
let _21: i8;
let _22: f64;
let _23: Adt50;
let _24: f64;
let _25: char;
let _26: Adt41;
let _27: i32;
let _28: Adt44;
let _29: Adt36;
let _30: *mut usize;
let _31: isize;
let _32: u64;
let _33: usize;
let _34: &'static isize;
let _35: u32;
let _36: ();
let _37: ();
{
_2 = _12;
RET = 8603228032187321802_i64;
_12 = _6 + _13;
_12 = !_6;
_10 = '\u{f24be}' as i16;
_2 = 36044_u16 as isize;
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = _11;
(*_15) = !10506037451655605571_usize;
_2 = -_13;
(*_15) = 7182800111645420496_usize & 4296201038827572589_usize;
_1 = _4;
_6 = _13;
_4.0 = _9 + _3;
RET = -6062284610130343980_i64;
_12 = _2;
Goto(bb2)
}
bb2 = {
_15 = _11;
_6 = _13;
Goto(bb3)
}
bb3 = {
(*_11) = 6_usize - 17845961529694034131_usize;
_15 = _11;
_8 = [41071_u16,30653_u16,24576_u16,54400_u16];
_10 = _5;
_13 = RET as isize;
_17 = ['\u{10b530}','\u{10586d}','\u{244b9}','\u{7f111}','\u{a0547}','\u{aed4a}'];
(*_11) = !1_usize;
_9 = _10;
_2 = _5 as isize;
(*_11) = 5649874856141157443_usize;
(*_15) = 6105268806426910365_usize;
_5 = 37352_u16 as i16;
(*_11) = !7_usize;
(*_11) = 62752_u16 as usize;
Goto(bb4)
}
bb4 = {
_15 = core::ptr::addr_of_mut!((*_11));
_15 = core::ptr::addr_of_mut!((*_15));
(*_11) = !8159652359578322597_usize;
RET = !(-63554796215438710_i64);
Goto(bb5)
}
bb5 = {
_18 = (-84_i8) as f64;
_4 = _1;
_1.0 = _4.0;
(*_15) = 3_usize;
_16 = [RET];
(*_15) = 4574489970984124940_usize;
_19 = ['\u{bc90c}','\u{b3287}','\u{ce73c}','\u{6c199}','\u{e1df0}','\u{6a81e}'];
_5 = _7 as i16;
_2 = 248264622748168355802791234598618983948_u128 as isize;
(*_15) = !1_usize;
_16 = [RET];
_4 = (_3,);
(*_11) = 5_usize;
Call(_4.0 = core::intrinsics::bswap(_3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = (-87625263521510645_i64);
_2 = -_12;
(*_15) = 2_usize & 2436990740105518045_usize;
_21 = (-101_i8);
_1.0 = !_5;
_12 = !_6;
_12 = _6 >> _2;
_14 = [(-169022009539283926018862917188142613786_i128),(-126508288720049238777345816699358890128_i128),21145940829447675858977300828516621406_i128];
_18 = _21 as f64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463286982168246700811 => bb11,
_ => bb10
}
}
bb7 = {
_15 = _11;
(*_15) = !10506037451655605571_usize;
_2 = -_13;
(*_15) = 7182800111645420496_usize & 4296201038827572589_usize;
_1 = _4;
_6 = _13;
_4.0 = _9 + _3;
RET = -6062284610130343980_i64;
_12 = _2;
Goto(bb2)
}
bb8 = {
_15 = core::ptr::addr_of_mut!((*_11));
_15 = core::ptr::addr_of_mut!((*_15));
(*_11) = !8159652359578322597_usize;
RET = !(-63554796215438710_i64);
Goto(bb5)
}
bb9 = {
(*_11) = 6_usize - 17845961529694034131_usize;
_15 = _11;
_8 = [41071_u16,30653_u16,24576_u16,54400_u16];
_10 = _5;
_13 = RET as isize;
_17 = ['\u{10b530}','\u{10586d}','\u{244b9}','\u{7f111}','\u{a0547}','\u{aed4a}'];
(*_11) = !1_usize;
_9 = _10;
_2 = _5 as isize;
(*_11) = 5649874856141157443_usize;
(*_15) = 6105268806426910365_usize;
_5 = 37352_u16 as i16;
(*_11) = !7_usize;
(*_11) = 62752_u16 as usize;
Goto(bb4)
}
bb10 = {
_15 = _11;
_6 = _13;
Goto(bb3)
}
bb11 = {
_21 = (-65_i8);
_4.0 = _10;
_1.0 = _9 & _9;
_10 = !_1.0;
_24 = _18;
_26 = Adt41::Variant1 { fld0: _21,fld1: RET };
_21 = -Field::<i8>(Variant(_26, 1), 0);
_2 = -_6;
SetDiscriminant(_26, 1);
(*_15) = 296152958661232802286841567107272732253_u128 as usize;
(*_15) = 8528513927927848034_usize | 0_usize;
_1.0 = !_3;
_2 = _6;
_28.fld4 = Adt41::Variant1 { fld0: _21,fld1: RET };
RET = !Field::<i64>(Variant(_28.fld4, 1), 1);
_28.fld3.fld1 = '\u{1024f8}';
_28.fld1 = _14;
_13 = (-122313471151190367411830857466116138845_i128) as isize;
_15 = _11;
(*_11) = 7126_u16 as usize;
_29.fld3 = core::ptr::addr_of_mut!(_17);
_28.fld3.fld4 = _3;
_30 = _15;
_28.fld3.fld3 = _29.fld3;
match Field::<i64>(Variant(_28.fld4, 1), 1) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb9,
340282366920938463463286982168246700811 => bb13,
_ => bb12
}
}
bb12 = {
_15 = core::ptr::addr_of_mut!((*_11));
_15 = core::ptr::addr_of_mut!((*_15));
(*_11) = !8159652359578322597_usize;
RET = !(-63554796215438710_i64);
Goto(bb5)
}
bb13 = {
match Field::<i64>(Variant(_28.fld4, 1), 1) {
0 => bb5,
1 => bb14,
340282366920938463463286982168246700811 => bb16,
_ => bb15
}
}
bb14 = {
RET = (-87625263521510645_i64);
_2 = -_12;
(*_15) = 2_usize & 2436990740105518045_usize;
_21 = (-101_i8);
_1.0 = !_5;
_12 = !_6;
_12 = _6 >> _2;
_14 = [(-169022009539283926018862917188142613786_i128),(-126508288720049238777345816699358890128_i128),21145940829447675858977300828516621406_i128];
_18 = _21 as f64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463286982168246700811 => bb11,
_ => bb10
}
}
bb15 = {
_15 = _11;
(*_15) = !10506037451655605571_usize;
_2 = -_13;
(*_15) = 7182800111645420496_usize & 4296201038827572589_usize;
_1 = _4;
_6 = _13;
_4.0 = _9 + _3;
RET = -6062284610130343980_i64;
_12 = _2;
Goto(bb2)
}
bb16 = {
_4 = (_10,);
_12 = -_6;
_28.fld3.fld4 = !_9;
place!(Field::<i8>(Variant(_26, 1), 0)) = _21;
_15 = _30;
_1.0 = _7 as i16;
_28.fld2 = 112798090198181605047188658087443475918_i128 as isize;
SetDiscriminant(_28.fld4, 0);
_23 = Adt50::Variant2 { fld0: RET };
_29.fld5 = 1253957908_u32;
_29.fld1 = _28.fld3.fld1;
_31 = _2;
_28.fld3.fld4 = false as i16;
_30 = core::ptr::addr_of_mut!((*_15));
_28.fld3.fld0 = 325137832006048252229459159109014758646_u128;
_28.fld0 = _28.fld3.fld0 >> _9;
SetDiscriminant(_23, 2);
_29.fld2 = 17613649425760237916_u64 - 7581990226648245635_u64;
_29.fld6 = _30;
(*_11) = 1050436983862137087_usize | 1_usize;
place!(Field::<i64>(Variant(_26, 1), 1)) = -RET;
(*_15) = 3_usize - 1_usize;
_28.fld6 = 1483613172_i32 as i64;
_28.fld3 = Adt36 { fld0: _28.fld0,fld1: _29.fld1,fld2: _29.fld2,fld3: _29.fld3,fld4: _4.0,fld5: _29.fld5,fld6: _29.fld6 };
_32 = _29.fld2;
RET = _28.fld6 * _28.fld6;
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(18_usize, 14_usize, Move(_14), 4_usize, Move(_4), 31_usize, Move(_31), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(18_usize, 32_usize, Move(_32), 6_usize, Move(_6), 21_usize, Move(_21), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(18_usize, 7_usize, Move(_7), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: isize,mut _10: bool,mut _11: isize,mut _12: isize,mut _13: isize) -> bool {
mir! {
type RET = bool;
let _14: u8;
let _15: f32;
let _16: ();
let _17: ();
{
_4 = _8;
_11 = 16_i8 as isize;
_6 = _10 < _3;
_8 = !_7;
_2 = 4002847459_u32 as isize;
_6 = _7 ^ _10;
_9 = -_13;
RET = _6;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(19_usize, 11_usize, Move(_11), 3_usize, Move(_3), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(19_usize, 9_usize, Move(_9), 6_usize, Move(_6), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{dcff7}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(73_i8), std::hint::black_box((-15095_i16)), std::hint::black_box(1346495836_i32), std::hint::black_box((-4891609222006932810_i64)), std::hint::black_box((-38501078702348160931981335302126977993_i128)), std::hint::black_box(291282814832150377576316379700540646145_u128), std::hint::black_box(117_u8), std::hint::black_box(43317_u16), std::hint::black_box(2775499777_u32));
                
            }
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt35{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt35 {
fld0: bool,
fld1: char,
fld2: u128,
fld3: u64,
fld4: *const *mut [char; 6],
fld5: [char; 6],
fld6: (i16,),
fld7: usize,
}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt36{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt36 {
fld0: u128,
fld1: char,
fld2: u64,
fld3: *mut [char; 6],
fld4: i16,
fld5: u32,
fld6: *mut usize,
}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt37{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt37 {
fld0: i64,
fld1: i32,
fld2: Adt36,
fld3: i8,
}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt38::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: u64,
fld1: u8,
fld2: *mut (i16,),
fld3: i8,
fld4: f32,
fld5: *const *mut [char; 6],

},
Variant1{
fld0: u16,
fld1: u64,
fld2: i64,
fld3: (i16,),
fld4: u8,
fld5: usize,

},
Variant2{
fld0: bool,
fld1: Adt35,
fld2: u128,
fld3: usize,
fld4: Adt36,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: u128,
fld1: *const *mut [char; 6],
fld2: [char; 6],
fld3: *mut [char; 6],
fld4: f64,
fld5: i32,

},
Variant1{
fld0: u64,
fld1: Adt35,
fld2: u16,
fld3: i8,
fld4: *mut usize,
fld5: f32,
fld6: [char; 6],

},
Variant2{
fld0: *const *mut [char; 6],
fld1: f64,

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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: Adt37,
fld1: [i64; 1],
fld2: isize,
fld3: Adt36,
fld4: i64,
fld5: i32,

},
Variant1{
fld0: u64,
fld1: *const *mut [char; 6],
fld2: Adt39,
fld3: *mut usize,
fld4: *mut (i16,),

},
Variant2{
fld0: [char; 6],
fld1: f32,
fld2: isize,
fld3: i8,
fld4: Adt35,
fld5: u64,
fld6: i64,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: (i16,),
fld1: char,

},
Variant1{
fld0: i8,
fld1: i64,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u64,
fld1: Adt36,

},
Variant1{
fld0: Adt35,
fld1: u16,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: Adt35,
fld1: [u16; 4],
fld2: Adt41,
fld3: i8,
fld4: u8,
fld5: Adt36,

},
Variant1{
fld0: bool,
fld1: i64,
fld2: u64,
fld3: Adt39,
fld4: (i16,),

},
Variant2{
fld0: *mut usize,
fld1: char,
fld2: [i128; 3],
fld3: [u16; 4],
fld4: u16,
fld5: Adt36,
fld6: *const *mut [char; 6],
fld7: i128,

},
Variant3{
fld0: Adt38,
fld1: u8,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: u128,
fld1: [i128; 3],
fld2: isize,
fld3: Adt36,
fld4: Adt41,
fld5: Adt40,
fld6: i64,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u16,
fld1: Adt37,
fld2: isize,
fld3: Adt40,
fld4: usize,
fld5: i32,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: [u16; 4],
fld1: char,
fld2: *mut usize,
fld3: usize,
fld4: u128,
fld5: i32,

},
Variant2{
fld0: Adt39,

},
Variant3{
fld0: bool,
fld1: usize,
fld2: f64,
fld3: *mut [char; 6],
fld4: i16,
fld5: i32,
fld6: i64,
fld7: [u16; 4],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: *mut (i16,),
fld2: Adt35,
fld3: i8,
fld4: i16,
fld5: *const *mut [char; 6],
fld6: u16,
fld7: i128,

},
Variant1{
fld0: Adt35,
fld1: [i128; 3],
fld2: u32,
fld3: i8,
fld4: i16,
fld5: u8,

},
Variant2{
fld0: (i16,),
fld1: u32,
fld2: *const *mut [char; 6],
fld3: usize,
fld4: Adt44,
fld5: i32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i8,

},
Variant1{
fld0: Adt37,
fld1: Adt40,
fld2: *mut (i16,),
fld3: u16,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt44,
fld1: char,

},
Variant1{
fld0: Adt35,
fld1: char,
fld2: Adt45,
fld3: [i64; 1],
fld4: i16,
fld5: f64,
fld6: i64,
fld7: Adt46,

},
Variant2{
fld0: i32,
fld1: i8,

},
Variant3{
fld0: Adt41,
fld1: Adt47,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt37,
fld1: Adt39,
fld2: isize,
fld3: Adt45,
fld4: f32,

},
Variant1{
fld0: Adt48,
fld1: Adt35,
fld2: isize,
fld3: Adt46,
fld4: Adt44,
fld5: [char; 6],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (i16,),
fld1: i16,
fld2: Adt45,
fld3: Adt38,

},
Variant1{
fld0: [u16; 4],

},
Variant2{
fld0: i64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [i128; 3],
fld1: i16,
}

