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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: isize,mut _4: usize,mut _5: i128) -> f64 {
mir! {
type RET = f64;
let _6: (i64, f64, usize, *const isize, isize, *const f64);
let _7: *const [char; 4];
let _8: u16;
let _9: Adt48;
let _10: i32;
let _11: f64;
let _12: bool;
let _13: Adt48;
let _14: [u32; 1];
let _15: [u32; 7];
let _16: bool;
let _17: (u32,);
let _18: isize;
let _19: [i64; 8];
let _20: Adt53;
let _21: bool;
let _22: u16;
let _23: i16;
let _24: [char; 4];
let _25: char;
let _26: Adt52;
let _27: [u32; 1];
let _28: f32;
let _29: Adt56;
let _30: char;
let _31: ();
let _32: ();
{
_4 = 4_usize;
_1 = !4794839840262617689_u64;
_5 = !11287561566868886144963737116153510244_i128;
RET = _4 as f64;
_3 = (-9223372036854775808_isize) >> _5;
_5 = '\u{16a02}' as i128;
_2 = '\u{29f07}';
_5 = 83410397444700285039279662286027199416_u128 as i128;
_2 = '\u{98281}';
_1 = !15118360204893276191_u64;
_1 = 11678608533328257270_u64;
_2 = '\u{ab8fe}';
RET = (-617567096_i32) as f64;
_1 = !12822809524365223847_u64;
Goto(bb1)
}
bb1 = {
_6.0 = !(-5558156856571614000_i64);
_6.3 = core::ptr::addr_of!(_3);
_6.4 = !_3;
_6.2 = 32309_u16 as usize;
RET = 51760_u16 as f64;
_2 = '\u{f7098}';
_3 = _6.4;
_2 = '\u{c250f}';
_5 = (-46819243736597977936303777079204911399_i128) ^ 83615410524363684780021735684721864433_i128;
_6.4 = !_3;
_6.2 = _4 | _4;
RET = 62532120296226642701938662180891567394_u128 as f64;
_2 = '\u{2a72e}';
_3 = _6.4;
_3 = -_6.4;
_2 = '\u{553de}';
RET = 28959_u16 as f64;
RET = _5 as f64;
_5 = (-11457772005395785756536706880349584066_i128);
_6.1 = 14_i8 as f64;
RET = _6.1 - _6.1;
RET = _6.1 - _6.1;
_5 = 1580329121_i32 as i128;
_6.5 = core::ptr::addr_of!(_6.1);
_5 = 68_i8 as i128;
RET = -_6.1;
_1 = 4291112278023420410_u64;
_6.0 = (-3642799220732073112_i64);
Call(_6.1 = fn1(_4, _6.0, _6.3, _1, _5, _6.2, _6.4, _6.5, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6.4 = _3;
_2 = '\u{17624}';
_3 = _6.4 * _6.4;
_6.4 = _3;
_3 = _6.4 ^ _6.4;
_4 = _6.2 >> _6.4;
_8 = _4 as u16;
_6.4 = _3;
_3 = -_6.4;
_6.5 = core::ptr::addr_of!(_6.1);
_5 = -(-134120653953337375892617865782799204277_i128);
RET = -_6.1;
_3 = (-1509941934_i32) as isize;
RET = _6.1 * _6.1;
_6.4 = -_3;
_6.5 = core::ptr::addr_of!(RET);
RET = _6.1 * _6.1;
_1 = 17902412756541484947_u64;
_6.0 = _6.1 as i64;
_6.0 = 4456149601190814561_i64;
Call(_10 = core::intrinsics::bswap(1858786030_i32), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = 14164_u16 | 64843_u16;
_3 = _6.4;
_6.5 = core::ptr::addr_of!(RET);
_6.1 = _1 as f64;
_5 = 3152753809_u32 as i128;
_2 = '\u{37a1e}';
_6.4 = _3;
_8 = 44609_u16 * 24558_u16;
_6.5 = core::ptr::addr_of!(_6.1);
_6.1 = -RET;
_6.2 = !_4;
_6.0 = -(-8139369305401484681_i64);
_11 = RET + _6.1;
_1 = 5584191464106149882_u64;
_11 = -_6.1;
_5 = -148002622896936262473778249737467882838_i128;
_12 = !false;
_6.5 = core::ptr::addr_of!(RET);
_12 = true;
_6.4 = _3 | _3;
_10 = _6.0 as i32;
RET = -_11;
_6.5 = core::ptr::addr_of!(_6.1);
_11 = (-54_i8) as f64;
_14 = [673124803_u32];
_3 = _6.4 + _6.4;
Goto(bb4)
}
bb4 = {
RET = _6.1;
_6.2 = _4 + _4;
_15 = [191042117_u32,602271895_u32,1053528010_u32,1558524313_u32,2009262108_u32,2409523482_u32,2309028928_u32];
_6.5 = core::ptr::addr_of!(_6.1);
Goto(bb5)
}
bb5 = {
RET = _11 * _6.1;
_6.0 = (-5110236270825043048_i64) * (-366275488617945776_i64);
_6.4 = _8 as isize;
_6.3 = core::ptr::addr_of!(_6.4);
_6.2 = _4 + _4;
_4 = _6.2 + _6.2;
_17 = (3053356377_u32,);
RET = _6.1;
_12 = !false;
Goto(bb6)
}
bb6 = {
_6.5 = core::ptr::addr_of!(_11);
_6.5 = core::ptr::addr_of!(RET);
_3 = _10 as isize;
_10 = !(-1478394221_i32);
_16 = !_12;
_6.2 = _3 as usize;
_10 = 1123391299_i32 + (-2084875789_i32);
_8 = 31136_u16 & 53196_u16;
_6.1 = -RET;
_6.1 = _6.0 as f64;
_9 = Adt48::Variant2 { fld0: _10 };
_13 = Adt48::Variant2 { fld0: Field::<i32>(Variant(_9, 2), 0) };
RET = -_11;
_9 = Move(_13);
SetDiscriminant(_9, 0);
place!(Field::<u128>(Variant(_9, 0), 0)) = 65273980635409457224724536619896385661_u128;
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant0 { fld0: _3 };
_6.5 = core::ptr::addr_of!(RET);
place!(Field::<i64>(Variant(_9, 0), 6)) = _6.0 ^ _6.0;
SetDiscriminant(Field::<Adt44>(Variant(_9, 0), 3), 0);
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant0 { fld0: _6.4 };
_18 = _6.4;
match Field::<u128>(Variant(_9, 0), 0) {
0 => bb2,
1 => bb7,
2 => bb8,
65273980635409457224724536619896385661 => bb10,
_ => bb9
}
}
bb7 = {
_6.4 = _3;
_2 = '\u{17624}';
_3 = _6.4 * _6.4;
_6.4 = _3;
_3 = _6.4 ^ _6.4;
_4 = _6.2 >> _6.4;
_8 = _4 as u16;
_6.4 = _3;
_3 = -_6.4;
_6.5 = core::ptr::addr_of!(_6.1);
_5 = -(-134120653953337375892617865782799204277_i128);
RET = -_6.1;
_3 = (-1509941934_i32) as isize;
RET = _6.1 * _6.1;
_6.4 = -_3;
_6.5 = core::ptr::addr_of!(RET);
RET = _6.1 * _6.1;
_1 = 17902412756541484947_u64;
_6.0 = _6.1 as i64;
_6.0 = 4456149601190814561_i64;
Call(_10 = core::intrinsics::bswap(1858786030_i32), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
RET = _6.1;
_6.2 = _4 + _4;
_15 = [191042117_u32,602271895_u32,1053528010_u32,1558524313_u32,2009262108_u32,2409523482_u32,2309028928_u32];
_6.5 = core::ptr::addr_of!(_6.1);
Goto(bb5)
}
bb9 = {
_6.0 = !(-5558156856571614000_i64);
_6.3 = core::ptr::addr_of!(_3);
_6.4 = !_3;
_6.2 = 32309_u16 as usize;
RET = 51760_u16 as f64;
_2 = '\u{f7098}';
_3 = _6.4;
_2 = '\u{c250f}';
_5 = (-46819243736597977936303777079204911399_i128) ^ 83615410524363684780021735684721864433_i128;
_6.4 = !_3;
_6.2 = _4 | _4;
RET = 62532120296226642701938662180891567394_u128 as f64;
_2 = '\u{2a72e}';
_3 = _6.4;
_3 = -_6.4;
_2 = '\u{553de}';
RET = 28959_u16 as f64;
RET = _5 as f64;
_5 = (-11457772005395785756536706880349584066_i128);
_6.1 = 14_i8 as f64;
RET = _6.1 - _6.1;
RET = _6.1 - _6.1;
_5 = 1580329121_i32 as i128;
_6.5 = core::ptr::addr_of!(_6.1);
_5 = 68_i8 as i128;
RET = -_6.1;
_1 = 4291112278023420410_u64;
_6.0 = (-3642799220732073112_i64);
Call(_6.1 = fn1(_4, _6.0, _6.3, _1, _5, _6.2, _6.4, _6.5, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_17 = (3752985192_u32,);
_6.0 = Field::<u128>(Variant(_9, 0), 0) as i64;
_6.5 = core::ptr::addr_of!(_6.1);
_9 = Adt48::Variant2 { fld0: _10 };
place!(Field::<i32>(Variant(_9, 2), 0)) = !_10;
SetDiscriminant(_9, 0);
_17 = (1222818438_u32,);
place!(Field::<(usize,)>(Variant(_9, 0), 1)).0 = _4;
place!(Field::<i64>(Variant(_9, 0), 6)) = _6.0;
_19 = [Field::<i64>(Variant(_9, 0), 6),Field::<i64>(Variant(_9, 0), 6),Field::<i64>(Variant(_9, 0), 6),Field::<i64>(Variant(_9, 0), 6),Field::<i64>(Variant(_9, 0), 6),_6.0,Field::<i64>(Variant(_9, 0), 6),Field::<i64>(Variant(_9, 0), 6)];
place!(Field::<u128>(Variant(_9, 0), 0)) = !313030512121152871423582076316253808593_u128;
_5 = _8 as i128;
_18 = _2 as isize;
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant0 { fld0: _6.4 };
place!(Field::<u128>(Variant(_9, 0), 0)) = 24380476398190248933406996564218167456_u128;
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant0 { fld0: _6.4 };
place!(Field::<u128>(Variant(_9, 0), 0)) = !111692753335066016839993032836746807991_u128;
RET = 218_u8 as f64;
Goto(bb11)
}
bb11 = {
_14 = [_17.0];
place!(Field::<u128>(Variant(_9, 0), 0)) = 1845550003774311631109202319179460235_u128 - 93220336534884230359756075640081265537_u128;
_23 = 4821_i16;
SetDiscriminant(Field::<Adt44>(Variant(_9, 0), 3), 1);
_10 = 3048270_i32 & 605367175_i32;
_6.2 = _18 as usize;
place!(Field::<u32>(Variant(_9, 0), 4)) = !_17.0;
place!(Field::<u128>(Variant(_9, 0), 0)) = 77939390168693630572211638319785821085_u128 << Field::<(usize,)>(Variant(_9, 0), 1).0;
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant1 { fld0: _1,fld1: _6.5 };
_17.0 = Field::<u32>(Variant(_9, 0), 4);
_1 = Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 0), 3), 1), 0);
SetDiscriminant(Field::<Adt44>(Variant(_9, 0), 3), 0);
_16 = Field::<u128>(Variant(_9, 0), 0) <= Field::<u128>(Variant(_9, 0), 0);
_17.0 = Field::<u32>(Variant(_9, 0), 4) ^ Field::<u32>(Variant(_9, 0), 4);
_22 = _23 as u16;
place!(Field::<isize>(Variant(place!(Field::<Adt44>(Variant(_9, 0), 3)), 0), 0)) = _2 as isize;
_15 = [_17.0,Field::<u32>(Variant(_9, 0), 4),_17.0,_17.0,_17.0,_17.0,_17.0];
_6.5 = core::ptr::addr_of!(_11);
place!(Field::<Adt44>(Variant(_9, 0), 3)) = Adt44::Variant1 { fld0: _1,fld1: _6.5 };
_22 = !_8;
_24 = [_2,_2,_2,_2];
place!(Field::<u128>(Variant(_9, 0), 0)) = 125480338450595378239534749809838609532_u128 - 248886388674432241882615119769847765246_u128;
place!(Field::<(usize,)>(Variant(_9, 0), 1)) = (_4,);
Goto(bb12)
}
bb12 = {
SetDiscriminant(Field::<Adt44>(Variant(_9, 0), 3), 1);
_27 = [_17.0];
_14 = _27;
_17.0 = Field::<u32>(Variant(_9, 0), 4);
_6.4 = _3;
_10 = !(-327454143_i32);
_18 = _3 + _3;
place!(Field::<f32>(Variant(_9, 0), 5)) = 81_u8 as f32;
_12 = _16;
_28 = _23 as f32;
place!(Field::<*const f64>(Variant(place!(Field::<Adt44>(Variant(_9, 0), 3)), 1), 1)) = core::ptr::addr_of!(_29.fld0.fld4.fld0.1);
_29.fld0.fld6.2.1 = !_5;
place!(Field::<*mut (i128, [char; 4])>(Variant(_9, 0), 2)) = core::ptr::addr_of_mut!(_29.fld0.fld6.0);
_29.fld0.fld6.2.2 = (-107_i8) as f32;
_11 = RET + _6.1;
_29.fld0.fld1 = (_5, _24);
_29.fld3 = _6.5;
_29.fld0.fld2 = Adt45::Variant3 { fld0: _12,fld1: _19,fld2: _6.4,fld3: _14,fld4: _23,fld5: 163_u8,fld6: _1 };
_29.fld0.fld6.3 = 206_u8 as i8;
_29.fld5 = (Field::<u32>(Variant(_9, 0), 4),);
match _1 {
0 => bb6,
1 => bb11,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb13,
5584191464106149882 => bb15,
_ => bb14
}
}
bb13 = {
_6.0 = !(-5558156856571614000_i64);
_6.3 = core::ptr::addr_of!(_3);
_6.4 = !_3;
_6.2 = 32309_u16 as usize;
RET = 51760_u16 as f64;
_2 = '\u{f7098}';
_3 = _6.4;
_2 = '\u{c250f}';
_5 = (-46819243736597977936303777079204911399_i128) ^ 83615410524363684780021735684721864433_i128;
_6.4 = !_3;
_6.2 = _4 | _4;
RET = 62532120296226642701938662180891567394_u128 as f64;
_2 = '\u{2a72e}';
_3 = _6.4;
_3 = -_6.4;
_2 = '\u{553de}';
RET = 28959_u16 as f64;
RET = _5 as f64;
_5 = (-11457772005395785756536706880349584066_i128);
_6.1 = 14_i8 as f64;
RET = _6.1 - _6.1;
RET = _6.1 - _6.1;
_5 = 1580329121_i32 as i128;
_6.5 = core::ptr::addr_of!(_6.1);
_5 = 68_i8 as i128;
RET = -_6.1;
_1 = 4291112278023420410_u64;
_6.0 = (-3642799220732073112_i64);
Call(_6.1 = fn1(_4, _6.0, _6.3, _1, _5, _6.2, _6.4, _6.5, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
RET = _11 * _6.1;
_6.0 = (-5110236270825043048_i64) * (-366275488617945776_i64);
_6.4 = _8 as isize;
_6.3 = core::ptr::addr_of!(_6.4);
_6.2 = _4 + _4;
_4 = _6.2 + _6.2;
_17 = (3053356377_u32,);
RET = _6.1;
_12 = !false;
Goto(bb6)
}
bb15 = {
_29.fld0.fld6.5 = _29.fld0.fld6.3;
_8 = _22 + _22;
_29.fld0.fld6.2.0 = _23;
_29.fld4 = core::ptr::addr_of!(_29.fld0.fld4.fld0.4);
_29.fld0.fld4.fld0.4 = -_18;
_17 = (_29.fld5.0,);
_29.fld3 = _6.5;
_10 = Field::<(usize,)>(Variant(_9, 0), 1).0 as i32;
_18 = _6.4 * _3;
place!(Field::<i16>(Variant(_29.fld0.fld2, 3), 4)) = _29.fld0.fld6.5 as i16;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(0_usize, 4_usize, Move(_4), 16_usize, Move(_16), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 22_usize, Move(_22), 17_usize, Move(_17), 10_usize, Move(_10), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(0_usize, 23_usize, Move(_23), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: usize,mut _2: i64,mut _3: *const isize,mut _4: u64,mut _5: i128,mut _6: usize,mut _7: isize,mut _8: *const f64,mut _9: char,mut _10: char) -> f64 {
mir! {
type RET = f64;
let _11: Adt55;
let _12: Adt60;
let _13: char;
let _14: u8;
let _15: [char; 4];
let _16: (u32,);
let _17: u128;
let _18: (u32,);
let _19: char;
let _20: char;
let _21: i8;
let _22: f32;
let _23: (i128, [char; 4]);
let _24: Adt57;
let _25: isize;
let _26: [u32; 1];
let _27: i8;
let _28: isize;
let _29: (i16, i128, f32);
let _30: *const isize;
let _31: ();
let _32: ();
{
_11.fld0 = !true;
_11.fld6.4 = (-1348305010_i32) as u16;
_11.fld5 = (_6,);
RET = (-7_i8) as f64;
_11.fld6.2.0 = (-26888_i16);
_1 = !_11.fld5.0;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4291112278023420410 => bb6,
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
_11.fld6.2.0 = 25112_i16;
_11.fld5 = (_1,);
_11.fld4.fld0 = (_2, RET, _11.fld5.0, _3, (*_3), _8);
_11.fld4.fld0.4 = _11.fld6.2.0 as isize;
_11.fld4.fld0.0 = _2 * _2;
_11.fld6.2.1 = _5;
_11.fld4.fld0.3 = _3;
_11.fld6.0.1 = [_9,_10,_9,_9];
_11.fld6.1 = core::ptr::addr_of!(_11.fld5);
_4 = _11.fld4.fld0.1 as u64;
_11.fld4.fld0.3 = _3;
_11.fld4.fld0.2 = !_11.fld5.0;
_11.fld4.fld0.1 = _11.fld4.fld0.0 as f64;
_11.fld6.5 = !(-77_i8);
_11.fld4.fld0.3 = core::ptr::addr_of!(_7);
_11.fld5 = (_1,);
_11.fld4.fld0.0 = _2 + _2;
_11.fld6.4 = 43730_u16 + 48492_u16;
_11.fld6.2.1 = _4 as i128;
_11.fld6.1 = core::ptr::addr_of!(_11.fld5);
_11.fld1.1 = [_9,_10,_9,_10];
_11.fld1.1 = [_9,_9,_10,_9];
_11.fld6.2.1 = _11.fld4.fld0.1 as i128;
Call(_11.fld6.4 = core::intrinsics::transmute(_11.fld6.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = [_9,_9,_10,_10];
_8 = core::ptr::addr_of!(RET);
_14 = 203_u8;
RET = _4 as f64;
_11.fld4.fld0.0 = _2;
_11.fld1.1 = [_9,_10,_9,_10];
_2 = _11.fld6.5 as i64;
RET = -_11.fld4.fld0.1;
_11.fld4.fld0.1 = _11.fld6.4 as f64;
_11.fld1 = (_11.fld6.2.1, _11.fld6.0.1);
_11.fld6.0 = (_11.fld1.0, _15);
_4 = !15529047919673499114_u64;
(*_8) = _11.fld4.fld0.0 as f64;
_18 = (3136014324_u32,);
_17 = !26540901025231428966739241500946962829_u128;
Goto(bb8)
}
bb8 = {
_11.fld6.2.1 = -_11.fld6.0.0;
_11.fld1 = _11.fld6.0;
_7 = (*_3) | (*_3);
_11.fld6.3 = _11.fld6.5 ^ _11.fld6.5;
(*_8) = _18.0 as f64;
(*_3) = _7;
_4 = 3581225321235947222_u64;
_22 = _2 as f32;
(*_8) = _17 as f64;
_18.0 = !1376737061_u32;
_3 = _11.fld4.fld0.3;
_8 = _11.fld4.fld0.5;
RET = -_11.fld4.fld0.1;
_5 = _11.fld6.2.1;
_13 = _9;
_8 = _11.fld4.fld0.5;
(*_3) = _11.fld4.fld0.4 | _11.fld4.fld0.4;
_7 = -_11.fld4.fld0.4;
_2 = _4 as i64;
_11.fld4.fld0.1 = RET - RET;
_11.fld4.fld0.2 = _1 ^ _6;
_11.fld6.2 = ((-10237_i16), _11.fld6.0.0, _22);
_17 = !220373467887486730143459669075712639572_u128;
match _14 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb9,
4 => bb10,
5 => bb11,
203 => bb13,
_ => bb12
}
}
bb9 = {
_15 = [_9,_9,_10,_10];
_8 = core::ptr::addr_of!(RET);
_14 = 203_u8;
RET = _4 as f64;
_11.fld4.fld0.0 = _2;
_11.fld1.1 = [_9,_10,_9,_10];
_2 = _11.fld6.5 as i64;
RET = -_11.fld4.fld0.1;
_11.fld4.fld0.1 = _11.fld6.4 as f64;
_11.fld1 = (_11.fld6.2.1, _11.fld6.0.1);
_11.fld6.0 = (_11.fld1.0, _15);
_4 = !15529047919673499114_u64;
(*_8) = _11.fld4.fld0.0 as f64;
_18 = (3136014324_u32,);
_17 = !26540901025231428966739241500946962829_u128;
Goto(bb8)
}
bb10 = {
_11.fld6.2.0 = 25112_i16;
_11.fld5 = (_1,);
_11.fld4.fld0 = (_2, RET, _11.fld5.0, _3, (*_3), _8);
_11.fld4.fld0.4 = _11.fld6.2.0 as isize;
_11.fld4.fld0.0 = _2 * _2;
_11.fld6.2.1 = _5;
_11.fld4.fld0.3 = _3;
_11.fld6.0.1 = [_9,_10,_9,_9];
_11.fld6.1 = core::ptr::addr_of!(_11.fld5);
_4 = _11.fld4.fld0.1 as u64;
_11.fld4.fld0.3 = _3;
_11.fld4.fld0.2 = !_11.fld5.0;
_11.fld4.fld0.1 = _11.fld4.fld0.0 as f64;
_11.fld6.5 = !(-77_i8);
_11.fld4.fld0.3 = core::ptr::addr_of!(_7);
_11.fld5 = (_1,);
_11.fld4.fld0.0 = _2 + _2;
_11.fld6.4 = 43730_u16 + 48492_u16;
_11.fld6.2.1 = _4 as i128;
_11.fld6.1 = core::ptr::addr_of!(_11.fld5);
_11.fld1.1 = [_9,_10,_9,_10];
_11.fld1.1 = [_9,_9,_10,_9];
_11.fld6.2.1 = _11.fld4.fld0.1 as i128;
Call(_11.fld6.4 = core::intrinsics::transmute(_11.fld6.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_24.fld5.fld0.fld6.2 = (_11.fld6.2.0, _11.fld6.0.0, _22);
_23.0 = _11.fld6.3 as i128;
_19 = _10;
_24.fld5.fld0.fld4 = Move(_11.fld4);
_24.fld5.fld0.fld4.fld0.0 = !_2;
_24.fld5.fld5 = _18;
_24.fld5.fld0.fld6.2.2 = _11.fld6.2.2 + _11.fld6.2.2;
_16.0 = _23.0 as u32;
_16 = (_18.0,);
_11.fld4.fld0.2 = _6;
_24.fld5.fld0.fld6.2.2 = -_22;
Call(_11.fld2 = fn2(_11.fld6, _24.fld5.fld0.fld4.fld0, _11.fld4.fld0.2, _11.fld6.2, _3, _24.fld5.fld0.fld4.fld0, _11.fld5, _24.fld5.fld5.0, _8, _9, _24.fld5.fld0.fld4.fld0.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15 = [_9,_10,_19,_9];
_24.fld5.fld0.fld4.fld0.3 = core::ptr::addr_of!((*_3));
_24.fld5.fld3 = core::ptr::addr_of!(RET);
_22 = _24.fld5.fld5.0 as f32;
_11.fld6.4 = !19067_u16;
_11.fld6.5 = !_11.fld6.3;
_24.fld5.fld0.fld6.2.2 = _22;
_11.fld1.1 = [_19,_9,_19,_9];
_11.fld4.fld0.5 = core::ptr::addr_of!(_11.fld4.fld0.1);
_11.fld6.5 = _11.fld6.3;
_24.fld5.fld0.fld6 = (_11.fld6.0, _11.fld6.1, _11.fld6.2, _11.fld6.5, _11.fld6.4, _11.fld6.5);
_24.fld0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_19 = _10;
_8 = core::ptr::addr_of!(RET);
_24.fld5.fld0.fld0 = _11.fld0 | _11.fld0;
_24.fld5.fld0.fld4.fld0.4 = (*_3);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 10_usize, Move(_10), 7_usize, Move(_7), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 9_usize, Move(_9), 16_usize, Move(_16), 5_usize, Move(_5), 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8),mut _2: (i64, f64, usize, *const isize, isize, *const f64),mut _3: usize,mut _4: (i16, i128, f32),mut _5: *const isize,mut _6: (i64, f64, usize, *const isize, isize, *const f64),mut _7: (usize,),mut _8: u32,mut _9: *const f64,mut _10: char,mut _11: usize) -> Adt45 {
mir! {
type RET = Adt45;
let _12: isize;
let _13: Adt46;
let _14: i128;
let _15: i8;
let _16: [char; 4];
let _17: usize;
let _18: Adt55;
let _19: bool;
let _20: Adt57;
let _21: f64;
let _22: &'static i64;
let _23: [u32; 1];
let _24: char;
let _25: f32;
let _26: [char; 6];
let _27: *mut (i128, [char; 4]);
let _28: bool;
let _29: [char; 6];
let _30: [u32; 1];
let _31: Adt51;
let _32: (i16, i128, f32);
let _33: (i16, i128, f32);
let _34: bool;
let _35: char;
let _36: [u32; 2];
let _37: u64;
let _38: isize;
let _39: isize;
let _40: (&'static i64, i16, bool, i8);
let _41: [u32; 7];
let _42: f64;
let _43: Adt55;
let _44: isize;
let _45: isize;
let _46: isize;
let _47: &'static i64;
let _48: ();
let _49: ();
{
_1.2.2 = _4.2 + _4.2;
_13.fld0.2 = !_11;
(*_5) = _2.4 - _2.4;
_13.fld0 = (_2.0, _2.1, _2.2, _5, (*_5), _6.5);
_1.0.0 = -_4.1;
_6.5 = _9;
_1.2.1 = 11_u8 as i128;
Goto(bb1)
}
bb1 = {
_7.0 = _3 & _3;
Goto(bb2)
}
bb2 = {
_4.2 = _1.2.2 - _1.2.2;
_2.0 = _2.1 as i64;
_1.2 = (_4.0, _4.1, _4.2);
(*_5) = _2.4 ^ _2.4;
_2.5 = _13.fld0.5;
_1.4 = _4.2 as u16;
_3 = 212_u8 as usize;
_4.2 = _1.2.0 as f32;
_12 = (*_5) ^ _13.fld0.4;
_3 = _1.4 as usize;
_2.5 = _13.fld0.5;
_10 = '\u{e993c}';
_6.3 = core::ptr::addr_of!((*_5));
(*_5) = -_13.fld0.4;
_4 = _1.2;
_1.3 = _1.4 as i8;
_6.4 = _13.fld0.4;
_7 = (_13.fld0.2,);
_13.fld0.4 = _12 & _2.4;
_6 = (_2.0, _13.fld0.1, _13.fld0.2, _13.fld0.3, _13.fld0.4, _2.5);
_2.0 = _2.1 as i64;
_15 = 108_u8 as i8;
Call(_6 = fn3(_1.2.0, _1.0, _5, _4.0, _13.fld0, _11, _2, _1.3, _9, _4.0, _1.3, Move(_13), _1.4, _1.2.0, (*_5), _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1.1 = core::ptr::addr_of!(_7);
(*_5) = _1.3 as isize;
_6 = (_2.0, _2.1, _11, _5, (*_5), _2.5);
_2.2 = _10 as usize;
_2.1 = (*_5) as f64;
_1.5 = _1.3 | _1.3;
_13.fld0.2 = _11 * _11;
_13.fld0.3 = _5;
_2.1 = -_6.1;
_13.fld0 = (_6.0, _2.1, _6.2, _6.3, _6.4, _2.5);
_9 = _6.5;
_6.2 = _7.0 >> _2.4;
_3 = 3844911775330619005_u64 as usize;
_2 = (_6.0, _13.fld0.1, _13.fld0.2, _13.fld0.3, _13.fld0.4, _6.5);
_13.fld0.1 = _6.1 + _6.1;
_13.fld0.5 = _9;
_17 = _7.0 >> _6.2;
_16 = _1.0.1;
_13.fld0.1 = _6.1;
_2.2 = 112_u8 as usize;
_18.fld6.5 = _12 as i8;
_18.fld6.4 = !_1.4;
_2.1 = _4.0 as f64;
_18.fld0 = !false;
match _1.2.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768201219 => bb12,
_ => bb11
}
}
bb4 = {
_4.2 = _1.2.2 - _1.2.2;
_2.0 = _2.1 as i64;
_1.2 = (_4.0, _4.1, _4.2);
(*_5) = _2.4 ^ _2.4;
_2.5 = _13.fld0.5;
_1.4 = _4.2 as u16;
_3 = 212_u8 as usize;
_4.2 = _1.2.0 as f32;
_12 = (*_5) ^ _13.fld0.4;
_3 = _1.4 as usize;
_2.5 = _13.fld0.5;
_10 = '\u{e993c}';
_6.3 = core::ptr::addr_of!((*_5));
(*_5) = -_13.fld0.4;
_4 = _1.2;
_1.3 = _1.4 as i8;
_6.4 = _13.fld0.4;
_7 = (_13.fld0.2,);
_13.fld0.4 = _12 & _2.4;
_6 = (_2.0, _13.fld0.1, _13.fld0.2, _13.fld0.3, _13.fld0.4, _2.5);
_2.0 = _2.1 as i64;
_15 = 108_u8 as i8;
Call(_6 = fn3(_1.2.0, _1.0, _5, _4.0, _13.fld0, _11, _2, _1.3, _9, _4.0, _1.3, Move(_13), _1.4, _1.2.0, (*_5), _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7.0 = _3 & _3;
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
Return()
}
bb12 = {
_1.2.1 = _4.1;
_20.fld5.fld0.fld1.1 = _1.0.1;
_6.4 = !_12;
_20.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_6.1);
_20.fld5.fld0.fld0 = _13.fld0.2 != _17;
_20.fld5.fld0.fld4.fld0.3 = _6.3;
_20.fld3 = [_10,_10,_10,_10];
_20.fld5.fld0.fld6.2.0 = _17 as i16;
match _4.0 {
0 => bb7,
1 => bb9,
2 => bb13,
340282366920938463463374607431768201219 => bb15,
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
_20.fld5.fld0.fld6.1 = core::ptr::addr_of!(_18.fld5);
_20.fld5.fld0.fld6.2.1 = _4.1;
_18.fld5.0 = !_13.fld0.2;
_20.fld5.fld0.fld6.2.2 = _4.2 - _1.2.2;
_6.2 = _17 * _17;
_20.fld5.fld0.fld4.fld0.4 = _1.0.0 as isize;
_20.fld1 = _10;
_18.fld4 = Adt46 { fld0: _6 };
_9 = _18.fld4.fld0.5;
_18.fld1 = _1.0;
_20.fld5.fld0.fld5 = _7;
_1.2.0 = !_20.fld5.fld0.fld6.2.0;
_18.fld4.fld0 = (_6.0, _6.1, _6.2, _5, _13.fld0.4, _20.fld5.fld0.fld4.fld0.5);
_14 = _20.fld5.fld0.fld6.2.1 >> _20.fld5.fld0.fld5.0;
_20.fld5.fld0.fld6 = (_18.fld1, _1.1, _1.2, _1.3, _18.fld6.4, _15);
_1.4 = _20.fld5.fld0.fld6.4 ^ _20.fld5.fld0.fld6.4;
_20.fld3 = _20.fld5.fld0.fld1.1;
_18.fld4.fld0.1 = 117108385225098595928280388265177650884_u128 as f64;
_18.fld6.2.1 = -_1.0.0;
Goto(bb16)
}
bb16 = {
_20.fld5.fld0.fld6.4 = _1.4 ^ _1.4;
_18.fld6.0.1 = _20.fld5.fld0.fld1.1;
_4.1 = !_1.2.1;
_20.fld5.fld0.fld6.0.0 = _20.fld5.fld0.fld6.2.1;
_20.fld5.fld5 = (_8,);
_7.0 = !_11;
_26 = [_10,_20.fld1,_10,_10,_10,_20.fld1];
(*_5) = _6.0 as isize;
_20.fld5.fld0.fld6.4 = 227287538914099652205948632487625114253_u128 as u16;
_1.0.1 = [_20.fld1,_20.fld1,_20.fld1,_20.fld1];
_2.0 = _20.fld5.fld0.fld0 as i64;
_6.0 = _2.0 + _2.0;
_21 = _13.fld0.1 + _2.1;
_18.fld6.2.2 = _20.fld5.fld0.fld6.2.2;
_8 = _20.fld5.fld5.0;
_20.fld5.fld0.fld6.2 = _1.2;
_18.fld6 = _20.fld5.fld0.fld6;
_20.fld5.fld0.fld4.fld0.0 = !_6.0;
_1.4 = _20.fld5.fld0.fld6.4;
_22 = &_13.fld0.0;
_1.1 = _20.fld5.fld0.fld6.1;
_33 = (_20.fld5.fld0.fld6.2.0, _18.fld6.0.0, _1.2.2);
_18.fld3 = _20.fld5.fld0.fld0 as u128;
_20.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_6.1);
_18.fld4.fld0 = (_6.0, _21, _17, _13.fld0.3, _13.fld0.4, _2.5);
_15 = _2.1 as i8;
Goto(bb17)
}
bb17 = {
_20.fld5.fld0.fld4.fld0 = (_2.0, _2.1, _18.fld4.fld0.2, _2.3, _2.4, _18.fld4.fld0.5);
_20.fld5.fld0.fld3 = !_18.fld3;
_17 = _18.fld4.fld0.2 | _6.2;
(*_5) = _4.2 as isize;
_20.fld5.fld0.fld1.0 = _14 + _20.fld5.fld0.fld6.0.0;
_2.5 = _6.5;
_20.fld5.fld5.0 = _8 * _8;
_2.3 = core::ptr::addr_of!(_2.4);
_18.fld6.2.2 = -_33.2;
_18.fld6.2.1 = _18.fld3 as i128;
_20.fld5.fld4 = _18.fld4.fld0.3;
_18.fld6.0.0 = _18.fld6.2.1;
_20.fld5.fld3 = core::ptr::addr_of!(_6.1);
_13.fld0.2 = _1.2.2 as usize;
_20.fld5.fld6 = core::ptr::addr_of!(_18.fld1.1);
_1.0.1 = [_20.fld1,_20.fld1,_10,_10];
_20.fld5.fld0.fld6.1 = core::ptr::addr_of!(_18.fld5);
_18.fld6.4 = _1.4;
match _4.0 {
0 => bb13,
1 => bb10,
2 => bb15,
3 => bb12,
4 => bb16,
5 => bb18,
6 => bb19,
340282366920938463463374607431768201219 => bb21,
_ => bb20
}
}
bb18 = {
Return()
}
bb19 = {
_7.0 = _3 & _3;
Goto(bb2)
}
bb20 = {
_4.2 = _1.2.2 - _1.2.2;
_2.0 = _2.1 as i64;
_1.2 = (_4.0, _4.1, _4.2);
(*_5) = _2.4 ^ _2.4;
_2.5 = _13.fld0.5;
_1.4 = _4.2 as u16;
_3 = 212_u8 as usize;
_4.2 = _1.2.0 as f32;
_12 = (*_5) ^ _13.fld0.4;
_3 = _1.4 as usize;
_2.5 = _13.fld0.5;
_10 = '\u{e993c}';
_6.3 = core::ptr::addr_of!((*_5));
(*_5) = -_13.fld0.4;
_4 = _1.2;
_1.3 = _1.4 as i8;
_6.4 = _13.fld0.4;
_7 = (_13.fld0.2,);
_13.fld0.4 = _12 & _2.4;
_6 = (_2.0, _13.fld0.1, _13.fld0.2, _13.fld0.3, _13.fld0.4, _2.5);
_2.0 = _2.1 as i64;
_15 = 108_u8 as i8;
Call(_6 = fn3(_1.2.0, _1.0, _5, _4.0, _13.fld0, _11, _2, _1.3, _9, _4.0, _1.3, Move(_13), _1.4, _1.2.0, (*_5), _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb21 = {
_20.fld4 = core::ptr::addr_of!(_23);
_20.fld0 = [_20.fld5.fld0.fld0,_20.fld5.fld0.fld0,_20.fld5.fld0.fld0,_20.fld5.fld0.fld0,_20.fld5.fld0.fld0,_20.fld5.fld0.fld0,_20.fld5.fld0.fld0];
_32.0 = _33.0 << _17;
_20.fld5.fld0.fld6.0 = _18.fld6.0;
_30 = [_20.fld5.fld5.0];
_20.fld5.fld0.fld6.5 = _20.fld5.fld0.fld6.3;
_18.fld4.fld0.0 = _6.0;
_2.1 = _6.1 * _18.fld4.fld0.1;
(*_5) = _18.fld4.fld0.4 >> _13.fld0.4;
_25 = _1.2.2;
_40.1 = _4.0 >> _18.fld4.fld0.2;
_18.fld6.2 = (_1.2.0, _14, _25);
_20.fld5.fld0.fld6.2.2 = _20.fld5.fld5.0 as f32;
Goto(bb22)
}
bb22 = {
_37 = 11972775195178817232_u64 | 3247251807667263645_u64;
_20.fld5.fld3 = _9;
_43.fld6.4 = _18.fld0 as u16;
_13 = Adt46 { fld0: _18.fld4.fld0 };
_23 = _30;
_19 = !_18.fld0;
_20.fld5.fld0.fld1 = (_18.fld6.2.1, _20.fld5.fld0.fld6.0.1);
_36 = [_8,_20.fld5.fld5.0];
_20.fld5.fld0.fld4.fld0.5 = _13.fld0.5;
_28 = _18.fld6.0.0 >= _14;
_45 = _20.fld5.fld0.fld3 as isize;
_18.fld0 = _28;
_43.fld0 = _18.fld0;
_20.fld5.fld0.fld4 = Adt46 { fld0: _2 };
_1.2 = (_33.0, _20.fld5.fld0.fld1.0, _18.fld6.2.2);
_20.fld2 = _5;
_32.1 = -_20.fld5.fld0.fld6.0.0;
_40.3 = _18.fld6.3;
_20.fld5.fld0.fld2 = Adt45::Variant1 { fld0: _30,fld1: _2.0 };
RET = Adt45::Variant1 { fld0: _30,fld1: _13.fld0.0 };
_40.0 = &_6.0;
_43.fld6.2.1 = _33.2 as i128;
Goto(bb23)
}
bb23 = {
Call(_48 = dump_var(2_usize, 17_usize, Move(_17), 8_usize, Move(_8), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_48 = dump_var(2_usize, 45_usize, Move(_45), 19_usize, Move(_19), 14_usize, Move(_14), 30_usize, Move(_30)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_48 = dump_var(2_usize, 37_usize, Move(_37), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i16,mut _2: (i128, [char; 4]),mut _3: *const isize,mut _4: i16,mut _5: (i64, f64, usize, *const isize, isize, *const f64),mut _6: usize,mut _7: (i64, f64, usize, *const isize, isize, *const f64),mut _8: i8,mut _9: *const f64,mut _10: i16,mut _11: i8,mut _12: Adt46,mut _13: u16,mut _14: i16,mut _15: isize,mut _16: i64) -> (i64, f64, usize, *const isize, isize, *const f64) {
mir! {
type RET = (i64, f64, usize, *const isize, isize, *const f64);
let _17: u64;
let _18: (i128, [char; 4]);
let _19: bool;
let _20: bool;
let _21: f64;
let _22: (usize,);
let _23: *mut (i128, [char; 4]);
let _24: isize;
let _25: u64;
let _26: bool;
let _27: (u32,);
let _28: isize;
let _29: Adt60;
let _30: u64;
let _31: f64;
let _32: Adt57;
let _33: [u32; 2];
let _34: Adt59;
let _35: i64;
let _36: f64;
let _37: [u32; 7];
let _38: char;
let _39: isize;
let _40: isize;
let _41: *const f64;
let _42: char;
let _43: Adt54;
let _44: *mut (i128, [char; 4]);
let _45: [i64; 8];
let _46: Adt56;
let _47: u64;
let _48: f32;
let _49: i64;
let _50: u64;
let _51: ();
let _52: ();
{
_12.fld0.4 = true as isize;
_5.2 = _7.2;
_15 = _5.4;
RET = _12.fld0;
_5.1 = RET.1 + _12.fld0.1;
RET = (_5.0, _5.1, _5.2, _12.fld0.3, (*_3), _7.5);
RET.4 = _7.4;
_5.0 = _16 & _16;
_7.0 = _5.0 >> _12.fld0.0;
RET = _7;
RET.0 = _5.0 - _5.0;
_7.3 = core::ptr::addr_of!((*_3));
RET.2 = _5.2;
_7.5 = _5.5;
_12.fld0 = (_7.0, RET.1, _5.2, _7.3, _15, RET.5);
Goto(bb1)
}
bb1 = {
RET = (_7.0, _5.1, _12.fld0.2, _3, _5.4, _7.5);
_2.1 = ['\u{4e31f}','\u{9018a}','\u{595c2}','\u{61dd3}'];
RET.2 = _6 & _6;
RET.3 = _12.fld0.3;
_12.fld0.2 = _7.2;
_7.4 = !_15;
RET.1 = -_12.fld0.1;
_18.0 = _2.0;
RET.5 = core::ptr::addr_of!(_12.fld0.1);
RET.2 = !_7.2;
RET.3 = core::ptr::addr_of!((*_3));
_18 = _2;
_12.fld0 = (RET.0, _7.1, _5.2, _5.3, _15, _7.5);
_4 = 638046904_u32 as i16;
_9 = _5.5;
_7.5 = core::ptr::addr_of!(RET.1);
_13 = 53767_u16;
_23 = core::ptr::addr_of_mut!(_2);
RET.3 = core::ptr::addr_of!(_12.fld0.4);
_12.fld0.5 = core::ptr::addr_of!(_12.fld0.1);
_7.5 = _5.5;
_24 = !(*_3);
_17 = 5019596439967361323_u64;
(*_23) = _18;
_19 = !false;
RET.4 = !_15;
RET.2 = _7.2 * _6;
(*_23).0 = 3721344584_u32 as i128;
Goto(bb2)
}
bb2 = {
(*_23).1 = ['\u{8eb5e}','\u{95153}','\u{d98f8}','\u{87c5d}'];
RET.0 = !_12.fld0.0;
_12.fld0.5 = _7.5;
RET.1 = _7.0 as f64;
_7.2 = _2.0 as usize;
_20 = !_19;
RET.5 = _7.5;
_12 = Adt46 { fld0: RET };
_22 = (_5.2,);
RET.0 = _17 as i64;
RET = (_5.0, _12.fld0.1, _22.0, _12.fld0.3, _12.fld0.4, _12.fld0.5);
(*_3) = -_5.4;
RET = (_12.fld0.0, _12.fld0.1, _22.0, _5.3, (*_3), _12.fld0.5);
(*_3) = 3089300283_u32 as isize;
_12.fld0 = RET;
_25 = _17;
_7.4 = _5.4 & (*_3);
(*_23).0 = !_18.0;
RET = (_7.0, _7.1, _6, _5.3, _15, _12.fld0.5);
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768201219 => bb8,
_ => bb7
}
}
bb3 = {
RET = (_7.0, _5.1, _12.fld0.2, _3, _5.4, _7.5);
_2.1 = ['\u{4e31f}','\u{9018a}','\u{595c2}','\u{61dd3}'];
RET.2 = _6 & _6;
RET.3 = _12.fld0.3;
_12.fld0.2 = _7.2;
_7.4 = !_15;
RET.1 = -_12.fld0.1;
_18.0 = _2.0;
RET.5 = core::ptr::addr_of!(_12.fld0.1);
RET.2 = !_7.2;
RET.3 = core::ptr::addr_of!((*_3));
_18 = _2;
_12.fld0 = (RET.0, _7.1, _5.2, _5.3, _15, _7.5);
_4 = 638046904_u32 as i16;
_9 = _5.5;
_7.5 = core::ptr::addr_of!(RET.1);
_13 = 53767_u16;
_23 = core::ptr::addr_of_mut!(_2);
RET.3 = core::ptr::addr_of!(_12.fld0.4);
_12.fld0.5 = core::ptr::addr_of!(_12.fld0.1);
_7.5 = _5.5;
_24 = !(*_3);
_17 = 5019596439967361323_u64;
(*_23) = _18;
_19 = !false;
RET.4 = !_15;
RET.2 = _7.2 * _6;
(*_23).0 = 3721344584_u32 as i128;
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
_5.5 = RET.5;
_7.2 = _5.2 << _15;
_7.2 = _8 as usize;
_12 = Adt46 { fld0: _7 };
RET.0 = !_12.fld0.0;
_21 = RET.1 - _7.1;
Call(RET.0 = fn4(_5.4, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_23).1 = _18.1;
(*_23).0 = _18.0 & _18.0;
_2.1 = ['\u{fe24}','\u{b3d12}','\u{9ce6}','\u{9b737}'];
RET.2 = !_5.2;
_2 = (_18.0, _18.1);
_26 = _19;
_7.2 = !RET.2;
match _10 {
0 => bb6,
340282366920938463463374607431768201219 => bb10,
_ => bb5
}
}
bb10 = {
_7 = _12.fld0;
RET.2 = _14 as usize;
_11 = !_8;
_12.fld0.1 = -RET.1;
_32.fld5.fld0.fld6.0.0 = (*_23).0 + (*_23).0;
_18 = _2;
RET.5 = core::ptr::addr_of!(_21);
_10 = _18.0 as i16;
_32.fld5.fld0.fld1.0 = -(*_23).0;
_32.fld5.fld0.fld0 = _19 | _26;
_32.fld5.fld0.fld6.0 = (_18.0, (*_23).1);
_32.fld5.fld0.fld1.1 = _2.1;
_32.fld5.fld0.fld6.0 = ((*_23).0, (*_23).1);
_32.fld5.fld6 = core::ptr::addr_of!(_2.1);
_32.fld5.fld0.fld6.2.2 = 5_u8 as f32;
_21 = _12.fld0.1 + _5.1;
_32.fld5.fld0.fld6.2.2 = (*_23).0 as f32;
_7.1 = _17 as f64;
_32.fld5.fld0.fld6.2.2 = _17 as f32;
_18.1 = ['\u{d2188}','\u{b7b9f}','\u{f0a51}','\u{104abd}'];
_1 = !_14;
_11 = _8 | _8;
_27.0 = 3293342716_u32 << RET.2;
_5 = (_12.fld0.0, _21, _12.fld0.2, _3, _15, RET.5);
_36 = _5.1 - RET.1;
Goto(bb11)
}
bb11 = {
_32.fld5.fld0.fld4.fld0.3 = _12.fld0.3;
_18.0 = -(*_23).0;
_32.fld5.fld0.fld4.fld0.1 = _1 as f64;
_2.0 = -_32.fld5.fld0.fld1.0;
_31 = -_21;
_32.fld5.fld0.fld6.2.0 = !_14;
_5.2 = _6 ^ _6;
_29 = Adt60::Variant0 { fld0: _27 };
_2 = (_18.0, _18.1);
match _14 {
0 => bb1,
1 => bb8,
2 => bb9,
3 => bb4,
4 => bb7,
5 => bb6,
340282366920938463463374607431768201219 => bb13,
_ => bb12
}
}
bb12 = {
RET = (_7.0, _5.1, _12.fld0.2, _3, _5.4, _7.5);
_2.1 = ['\u{4e31f}','\u{9018a}','\u{595c2}','\u{61dd3}'];
RET.2 = _6 & _6;
RET.3 = _12.fld0.3;
_12.fld0.2 = _7.2;
_7.4 = !_15;
RET.1 = -_12.fld0.1;
_18.0 = _2.0;
RET.5 = core::ptr::addr_of!(_12.fld0.1);
RET.2 = !_7.2;
RET.3 = core::ptr::addr_of!((*_3));
_18 = _2;
_12.fld0 = (RET.0, _7.1, _5.2, _5.3, _15, _7.5);
_4 = 638046904_u32 as i16;
_9 = _5.5;
_7.5 = core::ptr::addr_of!(RET.1);
_13 = 53767_u16;
_23 = core::ptr::addr_of_mut!(_2);
RET.3 = core::ptr::addr_of!(_12.fld0.4);
_12.fld0.5 = core::ptr::addr_of!(_12.fld0.1);
_7.5 = _5.5;
_24 = !(*_3);
_17 = 5019596439967361323_u64;
(*_23) = _18;
_19 = !false;
RET.4 = !_15;
RET.2 = _7.2 * _6;
(*_23).0 = 3721344584_u32 as i128;
Goto(bb2)
}
bb13 = {
_32.fld5.fld0.fld6.4 = _32.fld5.fld0.fld6.2.2 as u16;
_32.fld0 = [_19,_26,_32.fld5.fld0.fld0,_19,_32.fld5.fld0.fld0,_32.fld5.fld0.fld0,_32.fld5.fld0.fld0];
_5.3 = core::ptr::addr_of!((*_3));
(*_3) = _15 & _15;
_24 = !_12.fld0.4;
_32.fld5.fld0.fld1.1 = (*_23).1;
RET.4 = -(*_3);
_39 = _8 as isize;
_13 = _32.fld5.fld0.fld6.4 ^ _32.fld5.fld0.fld6.4;
(*_23).0 = _32.fld5.fld0.fld6.2.0 as i128;
_32.fld5.fld0.fld4.fld0.4 = _39;
_9 = _5.5;
_31 = -RET.1;
_32.fld5.fld0.fld6.1 = core::ptr::addr_of!(_22);
_32.fld5.fld5 = (Field::<(u32,)>(Variant(_29, 0), 0).0,);
_12.fld0 = (_5.0, (*_9), _6, _5.3, (*_3), _9);
_36 = _27.0 as f64;
_12.fld0.0 = _32.fld5.fld0.fld6.2.2 as i64;
_20 = _26 ^ _19;
SetDiscriminant(_29, 0);
_7.0 = _2.0 as i64;
Goto(bb14)
}
bb14 = {
_45 = [_5.0,RET.0,RET.0,_7.0,RET.0,_16,_7.0,_5.0];
_32.fld5.fld0.fld4.fld0.1 = _21;
_32.fld5.fld0.fld5.0 = _32.fld5.fld0.fld0 as usize;
_4 = _1 + _14;
_32.fld5.fld5.0 = _27.0;
_22 = (_7.2,);
_39 = _12.fld0.4 ^ RET.4;
_46.fld0.fld4.fld0.0 = _7.0;
_46.fld0.fld4.fld0.5 = core::ptr::addr_of!(_46.fld0.fld4.fld0.1);
_5.5 = core::ptr::addr_of!(RET.1);
_20 = _32.fld5.fld0.fld0;
_43 = Adt54::Variant1 { fld0: 235007951443003937214572240692660601633_u128 };
_46.fld0.fld6.0.0 = _32.fld5.fld0.fld1.0 + (*_23).0;
_32.fld5.fld0.fld4.fld0.3 = _12.fld0.3;
_46.fld5 = (_27.0,);
_28 = !_39;
_17 = _25 >> _46.fld0.fld6.0.0;
_30 = _17 << RET.4;
_37 = [_32.fld5.fld5.0,_46.fld5.0,_32.fld5.fld5.0,_32.fld5.fld5.0,_27.0,_27.0,_46.fld5.0];
_18 = (_2.0, _2.1);
_5.5 = core::ptr::addr_of!(_46.fld0.fld4.fld0.1);
(*_23) = (_32.fld5.fld0.fld1.0, _32.fld5.fld0.fld6.0.1);
_32.fld5.fld0.fld4 = Adt46 { fld0: _7 };
_32.fld5.fld0.fld1 = (*_23);
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(3_usize, 18_usize, Move(_18), 10_usize, Move(_10), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(3_usize, 13_usize, Move(_13), 8_usize, Move(_8), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(3_usize, 11_usize, Move(_11), 28_usize, Move(_28), 20_usize, Move(_20), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: i16) -> i64 {
mir! {
type RET = i64;
let _3: Adt45;
let _4: u8;
let _5: f32;
let _6: Adt50;
let _7: u8;
let _8: ([char; 4], bool, i16);
let _9: *mut u32;
let _10: [bool; 7];
let _11: [char; 4];
let _12: isize;
let _13: f32;
let _14: char;
let _15: bool;
let _16: isize;
let _17: Adt60;
let _18: *const isize;
let _19: char;
let _20: f32;
let _21: [char; 4];
let _22: *mut (i128, [char; 4]);
let _23: u32;
let _24: usize;
let _25: [i64; 8];
let _26: isize;
let _27: [i64; 8];
let _28: u128;
let _29: ((&'static i64, i16, bool, i8), i16, (usize,));
let _30: (i128, [char; 4]);
let _31: isize;
let _32: i32;
let _33: *mut isize;
let _34: Adt49;
let _35: isize;
let _36: ();
let _37: ();
{
RET = _2 as i64;
RET = false as i64;
RET = 115_u8 as i64;
RET = (-2226508946842770716_i64) | 2357441834631903030_i64;
_2 = 18152_i16;
_2 = -(-13965_i16);
RET = 2613245889419384943_i64;
_2 = 1149_i16;
_2 = -(-6995_i16);
_4 = 238_u8;
_1 = !9223372036854775807_isize;
RET = -6940830694015090704_i64;
RET = -(-3667340904258214176_i64);
_4 = 201_u8;
_1 = 316609692955162947008684185907978421542_u128 as isize;
_4 = !76_u8;
RET = 3489901685652638067_i64 + 840193650414525981_i64;
RET = -2906139325101857902_i64;
_1 = -5_isize;
_4 = 80_u8 * 148_u8;
_2 = 17917_i16;
RET = (-7992696498166478265_i64) | 800004056482955419_i64;
_2 = (-30825_i16) + 21944_i16;
_2 = (-19633_i16);
_2 = (-22635_i16) ^ 17373_i16;
Goto(bb1)
}
bb1 = {
_5 = RET as f32;
RET = 34605335550104175_u64 as i64;
RET = false as i64;
RET = (-4595726214387473916_i64);
RET = _2 as i64;
_5 = (-100_i8) as f32;
RET = _1 as i64;
_1 = -91_isize;
Goto(bb2)
}
bb2 = {
RET = _5 as i64;
_7 = !_4;
_5 = 5_usize as f32;
_2 = '\u{f0b15}' as i16;
_2 = 24555_i16 & 6214_i16;
RET = -(-2095343789743122266_i64);
_4 = _7;
Call(_8.1 = fn5(_4, _2, RET, _7, _4, _1, _4, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8.2 = (-113_i8) as i16;
_8.1 = _1 > _1;
_1 = (-20_isize);
_8.2 = !_2;
RET = 13383633495585289653_usize as i64;
_5 = 17334621206178184148_u64 as f32;
_1 = -(-9223372036854775808_isize);
RET = 5454942950432453673_i64 ^ (-4827986146952052070_i64);
_8.1 = true | true;
_8.2 = !_2;
_7 = _4;
_7 = _4;
_7 = RET as u8;
Goto(bb4)
}
bb4 = {
RET = _5 as i64;
_8.0 = ['\u{32dac}','\u{9ef83}','\u{15a64}','\u{100b18}'];
_8.0 = ['\u{c3fc0}','\u{5c2}','\u{5b42e}','\u{96d59}'];
_8.1 = true;
_7 = !_4;
_1 = -9223372036854775807_isize;
_7 = !_4;
_1 = 9223372036854775807_isize * (-9223372036854775808_isize);
_8.2 = _2 << _1;
_8.2 = _2 - _2;
RET = -(-4473946427195949818_i64);
_1 = !9223372036854775807_isize;
_8.2 = -_2;
_8.2 = 210056552833670554306543982587957084828_u128 as i16;
_2 = '\u{fd0b8}' as i16;
_5 = RET as f32;
_11 = _8.0;
_7 = _4;
_8 = (_11, false, _2);
_2 = _8.2 & _8.2;
_13 = 2345671778825488750_u64 as f32;
_12 = _1 - _1;
_5 = -_13;
_4 = (-113283951467113942915339883793400085944_i128) as u8;
_8.1 = false;
_7 = !_4;
Goto(bb5)
}
bb5 = {
_4 = _8.2 as u8;
_4 = 50982518936648401518591043906440558824_i128 as u8;
_2 = _8.2;
_1 = -_12;
_7 = _4;
_15 = _12 > _1;
_15 = _8.2 != _2;
_14 = '\u{20eda}';
_5 = -_13;
_12 = !_1;
_10 = [_15,_8.1,_8.1,_15,_8.1,_8.1,_15];
_16 = _1;
_2 = !_8.2;
_7 = _4;
RET = 9138857501892214885_i64 >> _16;
Call(_12 = core::intrinsics::bswap(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = !(-4775278404271542220_i64);
_11 = [_14,_14,_14,_14];
_7 = _2 as u8;
_7 = _8.2 as u8;
_8.0 = [_14,_14,_14,_14];
_11 = _8.0;
_14 = '\u{4fb9f}';
_4 = _7 | _7;
_7 = _4;
_4 = !_7;
_16 = _1;
Goto(bb7)
}
bb7 = {
RET = (-4406591490885973405_i64);
_8 = (_11, _15, _2);
_8.0 = [_14,_14,_14,_14];
_8.0 = _11;
_8 = (_11, _15, _2);
_8.1 = _15 | _15;
_1 = _12 * _12;
_11 = [_14,_14,_14,_14];
_12 = _8.2 as isize;
_5 = _13 * _13;
_16 = _1 - _1;
_18 = core::ptr::addr_of!(_16);
_2 = !_8.2;
RET = _14 as i64;
_13 = _1 as f32;
_12 = -(*_18);
_20 = -_13;
_12 = (*_18);
_1 = _13 as isize;
RET = 2346714550568658082_i64;
_12 = _14 as isize;
Goto(bb8)
}
bb8 = {
_14 = '\u{cc72b}';
_2 = !_8.2;
_4 = !_7;
_5 = _20;
_19 = _14;
_18 = core::ptr::addr_of!((*_18));
_13 = _20 * _20;
_16 = _14 as isize;
_7 = _8.1 as u8;
_4 = 1708435887_i32 as u8;
_4 = _7 | _7;
(*_18) = _1;
_21 = [_14,_19,_19,_14];
_2 = 321211038_u32 as i16;
(*_18) = _8.1 as isize;
(*_18) = _12;
RET = -(-2116079108553935368_i64);
_5 = _4 as f32;
_13 = (-1189508146_i32) as f32;
_5 = -_20;
_18 = core::ptr::addr_of!(_16);
_12 = _1;
_14 = _19;
_14 = _19;
_14 = _19;
_15 = _8.1;
Call(_9 = fn18(_1, _8.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8.1 = _14 == _19;
_19 = _14;
_2 = 30018_u16 as i16;
Goto(bb10)
}
bb10 = {
_11 = _8.0;
_8.1 = !_15;
_4 = _7;
_14 = _19;
_27 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = (_11, _15, _2);
_1 = _12;
_2 = !_8.2;
_8 = (_11, _15, _2);
_24 = 15679517591901953645_usize | 5_usize;
_25 = _27;
_8.2 = _2;
_26 = _12 + _1;
_8 = (_11, _15, _2);
_4 = 66_i8 as u8;
Goto(bb11)
}
bb11 = {
_29.1 = _8.2;
RET = !7416981211898027014_i64;
_26 = _20 as isize;
_11 = [_19,_19,_19,_19];
_28 = 270348623025649902191647867523565061364_u128 ^ 224659947660637477763750310564734656114_u128;
_8.0 = _21;
_29.2.0 = 2459764991_u32 as usize;
_4 = !_7;
_23 = _4 as u32;
RET = (-2706282873350815449_i64) << _7;
_1 = RET as isize;
Goto(bb12)
}
bb12 = {
_7 = _4 - _4;
(*_18) = _26;
_20 = _5;
_11 = _21;
_8.1 = _15;
_8 = (_21, _15, _2);
_11 = [_14,_14,_19,_14];
_29.0.2 = _8.1;
RET = _23 as i64;
_30.0 = 10086570524147878766_u64 as i128;
_11 = [_14,_14,_14,_14];
_2 = _29.1;
_29.1 = -_8.2;
Goto(bb13)
}
bb13 = {
_27 = _25;
_32 = !(-79988488_i32);
_29.0.3 = _29.2.0 as i8;
Call(_12 = core::intrinsics::bswap(_1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = core::ptr::addr_of_mut!(_30);
(*_22).1 = [_14,_14,_19,_19];
_11 = [_19,_14,_19,_19];
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(4_usize, 25_usize, Move(_25), 30_usize, Move(_30), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(4_usize, 19_usize, Move(_19), 8_usize, Move(_8), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(4_usize, 7_usize, Move(_7), 27_usize, Move(_27), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: u8,mut _2: i16,mut _3: i64,mut _4: u8,mut _5: u8,mut _6: isize,mut _7: u8,mut _8: u8) -> bool {
mir! {
type RET = bool;
let _9: bool;
let _10: (i128, [char; 4]);
let _11: u16;
let _12: (u32,);
let _13: isize;
let _14: ([char; 4], bool, i16);
let _15: f32;
let _16: usize;
let _17: i8;
let _18: Adt50;
let _19: u128;
let _20: [bool; 7];
let _21: i128;
let _22: f32;
let _23: u8;
let _24: (i128, [char; 4]);
let _25: [u32; 7];
let _26: f32;
let _27: Adt54;
let _28: u128;
let _29: [u32; 2];
let _30: i64;
let _31: i32;
let _32: Adt47;
let _33: Adt59;
let _34: (i128, [char; 4]);
let _35: ([char; 4], bool, i16);
let _36: f64;
let _37: ([char; 4], bool, i16);
let _38: *const (usize,);
let _39: Adt46;
let _40: bool;
let _41: Adt56;
let _42: [u32; 1];
let _43: bool;
let _44: *const [u32; 1];
let _45: i8;
let _46: bool;
let _47: f32;
let _48: (usize,);
let _49: u8;
let _50: usize;
let _51: isize;
let _52: ();
let _53: ();
{
RET = _2 > _2;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _2 as u8;
_4 = _8;
_8 = _5 - _7;
_9 = RET;
_4 = 3643301322_u32 as u8;
_5 = 9475_u16 as u8;
RET = !_9;
RET = _9;
_10.0 = (-64777928523808584000557229254245773782_i128) | 19238892173964362734131938136897707945_i128;
_8 = !_7;
RET = _9;
_10.1 = ['\u{a9673}','\u{a0d19}','\u{46f71}','\u{10934a}'];
RET = _9;
_10.1 = ['\u{74bc2}','\u{7424c}','\u{88b9b}','\u{ac636}'];
_6 = 9223372036854775807_isize | 115_isize;
_8 = !_1;
Goto(bb2)
}
bb2 = {
_1 = _10.0 as u8;
RET = _9 ^ _9;
_3 = 2493161628851118514_i64;
_5 = !_8;
RET = _3 > _3;
RET = _9 ^ _9;
_3 = (-434995886358306464_i64) << _7;
_2 = !(-11629_i16);
_12 = (1166785392_u32,);
_8 = _7 << _5;
_11 = !45365_u16;
_6 = 49_isize;
_13 = _6 - _6;
_2 = -18763_i16;
_11 = 18550_u16;
_10.0 = !146534608395239794340125880030184994786_i128;
_2 = (-15975_i16);
_10.0 = (-149469895380545662051632780277432345637_i128) * 95470031631862516960999450338910659368_i128;
_14 = (_10.1, _9, _2);
_4 = _14.2 as u8;
RET = _14.1 <= _9;
RET = _9 ^ _9;
_5 = !_7;
_14.0 = ['\u{b72d2}','\u{308b3}','\u{e3891}','\u{1a4d2}'];
_3 = 58_i8 as i64;
Call(_17 = fn6(_10, _10.1, RET, RET, _14.2, RET, _9, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = 15132010764746547289_u64 as isize;
_15 = _11 as f32;
_2 = _14.2;
_16 = 11668336596261633689_usize << _4;
_16 = _15 as usize;
_10.0 = (-12308228876669046987213134763954537177_i128);
_6 = _9 as isize;
RET = _9;
_3 = (-8048582093062053567_i64) >> _1;
_9 = _4 > _5;
_14.0 = ['\u{69593}','\u{68140}','\u{e1759}','\u{e4521}'];
_19 = !158604837980745228281106282337954507455_u128;
_6 = _2 as isize;
Goto(bb4)
}
bb4 = {
_19 = 111497545873295135364497104855680279133_u128;
_14.1 = _9;
_21 = RET as i128;
_8 = _1 - _1;
_13 = -_6;
_17 = (-102_i8);
_23 = _6 as u8;
RET = !_14.1;
_14.1 = RET;
_10.1 = ['\u{f62ea}','\u{fa4bc}','\u{eac1c}','\u{100485}'];
_21 = _10.0 * _10.0;
_11 = _6 as u16;
_20 = [_9,_14.1,RET,_9,_14.1,_9,RET];
_12.0 = 821397504_u32;
_5 = !_8;
_21 = !_10.0;
_8 = _1 ^ _4;
_17 = _16 as i8;
_10.0 = _21 >> _14.2;
_1 = 12605585842027178032_u64 as u8;
_20 = [RET,_9,_14.1,_14.1,_9,_14.1,_9];
_22 = _15;
_12 = (3758908809_u32,);
_1 = '\u{b43f3}' as u8;
_7 = _8;
RET = _14.1;
_5 = !_7;
_19 = (-19676674_i32) as u128;
Goto(bb5)
}
bb5 = {
_14.0 = _10.1;
_24.1 = ['\u{5c217}','\u{d1696}','\u{a2427}','\u{4c577}'];
_14.1 = _9 ^ _9;
RET = _14.1;
_21 = _10.0 * _10.0;
_20 = [RET,RET,RET,_14.1,RET,_14.1,_14.1];
_25 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_10 = (_21, _24.1);
Goto(bb6)
}
bb6 = {
RET = _10.0 <= _21;
_8 = _7;
_3 = (-4861944259591005122_i64);
_13 = _6 - _6;
_20 = [RET,_14.1,RET,RET,_14.1,_14.1,RET];
_16 = 4_usize;
_17 = 987958359_i32 as i8;
_12 = (_25[_16],);
_26 = _22;
_11 = !44237_u16;
_27 = Adt54::Variant1 { fld0: _19 };
_25 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_21 = -_10.0;
_19 = _9 as u128;
_26 = _15;
_30 = -_3;
_24.1 = ['\u{5150d}','\u{1339d}','\u{a8ed7}','\u{f851}'];
_28 = _19 - _19;
_14.2 = -_2;
_2 = _16 as i16;
_1 = !_8;
_14 = (_24.1, _9, _2);
_21 = _28 as i128;
_25[_16] = _12.0;
_6 = _17 as isize;
match _25[_16] {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
3758908809 => bb9,
_ => bb8
}
}
bb7 = {
_1 = _10.0 as u8;
RET = _9 ^ _9;
_3 = 2493161628851118514_i64;
_5 = !_8;
RET = _3 > _3;
RET = _9 ^ _9;
_3 = (-434995886358306464_i64) << _7;
_2 = !(-11629_i16);
_12 = (1166785392_u32,);
_8 = _7 << _5;
_11 = !45365_u16;
_6 = 49_isize;
_13 = _6 - _6;
_2 = -18763_i16;
_11 = 18550_u16;
_10.0 = !146534608395239794340125880030184994786_i128;
_2 = (-15975_i16);
_10.0 = (-149469895380545662051632780277432345637_i128) * 95470031631862516960999450338910659368_i128;
_14 = (_10.1, _9, _2);
_4 = _14.2 as u8;
RET = _14.1 <= _9;
RET = _9 ^ _9;
_5 = !_7;
_14.0 = ['\u{b72d2}','\u{308b3}','\u{e3891}','\u{1a4d2}'];
_3 = 58_i8 as i64;
Call(_17 = fn6(_10, _10.1, RET, RET, _14.2, RET, _9, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_19 = 111497545873295135364497104855680279133_u128;
_14.1 = _9;
_21 = RET as i128;
_8 = _1 - _1;
_13 = -_6;
_17 = (-102_i8);
_23 = _6 as u8;
RET = !_14.1;
_14.1 = RET;
_10.1 = ['\u{f62ea}','\u{fa4bc}','\u{eac1c}','\u{100485}'];
_21 = _10.0 * _10.0;
_11 = _6 as u16;
_20 = [_9,_14.1,RET,_9,_14.1,_9,RET];
_12.0 = 821397504_u32;
_5 = !_8;
_21 = !_10.0;
_8 = _1 ^ _4;
_17 = _16 as i8;
_10.0 = _21 >> _14.2;
_1 = 12605585842027178032_u64 as u8;
_20 = [RET,_9,_14.1,_14.1,_9,_14.1,_9];
_22 = _15;
_12 = (3758908809_u32,);
_1 = '\u{b43f3}' as u8;
_7 = _8;
RET = _14.1;
_5 = !_7;
_19 = (-19676674_i32) as u128;
Goto(bb5)
}
bb9 = {
_14 = (_10.1, _9, _2);
_7 = _16 as u8;
_9 = _14.1;
_23 = !_8;
_8 = _5;
SetDiscriminant(_27, 0);
_10.0 = _21;
_22 = 283627610_i32 as f32;
_8 = !_1;
_14.1 = _20[_16];
_30 = _3;
RET = !_14.1;
_19 = 12434703903383153521_u64 as u128;
Goto(bb10)
}
bb10 = {
_31 = (-633515392_i32) << _30;
_8 = !_23;
_25 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_33 = Adt59::Variant0 { fld0: _12.0 };
Goto(bb11)
}
bb11 = {
_11 = !10645_u16;
_14.2 = -_2;
_8 = _23;
_27 = Adt54::Variant0 { fld0: _16 };
_10.0 = _21 - _21;
_34.0 = _10.0;
_24.0 = _10.0;
_29 = [Field::<u32>(Variant(_33, 0), 0),_25[_16]];
_15 = _31 as f32;
_10.1 = ['\u{6463}','\u{bf513}','\u{f40d2}','\u{c29b5}'];
_24 = _10;
_35.2 = _2;
_14 = (_10.1, RET, _2);
_14.1 = _20[_16];
_35.1 = Field::<usize>(Variant(_27, 0), 0) == Field::<usize>(Variant(_27, 0), 0);
_37.1 = !_9;
_20[_16] = !_35.1;
Goto(bb12)
}
bb12 = {
_10.0 = !_24.0;
_14.2 = _2;
place!(Field::<u32>(Variant(_33, 0), 0)) = !_25[_16];
_41.fld0.fld6.5 = -_17;
_39.fld0.2 = _16;
_41.fld0.fld4.fld0.5 = core::ptr::addr_of!(_39.fld0.1);
_41.fld0.fld1.1 = ['\u{4f29f}','\u{7caf8}','\u{c26be}','\u{c5132}'];
_19 = _28 | _28;
SetDiscriminant(_33, 1);
_34 = _10;
_39.fld0.4 = -_13;
_41.fld0.fld6.5 = -_17;
_31 = '\u{1f1a0}' as i32;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld1.1 = _41.fld0.fld1.1;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld2[_16] = !_14.1;
_10 = _24;
_41.fld3 = core::ptr::addr_of!(place!(Field::<Adt46>(Variant(_33, 1), 3)).fld0.1);
_31 = !1791800003_i32;
_41.fld2[_16] = RET;
_41.fld0.fld6.2.1 = -_34.0;
_39.fld0.1 = _13 as f64;
_34.0 = _41.fld0.fld6.2.1 << _4;
_17 = _25[_16] as i8;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld2[_16] = _41.fld2[_16];
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld6.2.2 = -_26;
place!(Field::<Adt46>(Variant(_33, 1), 3)).fld0.0 = _30;
_5 = _7 | _1;
Goto(bb13)
}
bb13 = {
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld4 = core::ptr::addr_of!(_13);
place!(Field::<u64>(Variant(_33, 1), 4)) = !10232978393923239803_u64;
SetDiscriminant(_27, 0);
_44 = core::ptr::addr_of!(_42);
match _16 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb9,
5 => bb15,
6 => bb16,
4 => bb18,
_ => bb17
}
}
bb14 = {
_10.0 = !_24.0;
_14.2 = _2;
place!(Field::<u32>(Variant(_33, 0), 0)) = !_25[_16];
_41.fld0.fld6.5 = -_17;
_39.fld0.2 = _16;
_41.fld0.fld4.fld0.5 = core::ptr::addr_of!(_39.fld0.1);
_41.fld0.fld1.1 = ['\u{4f29f}','\u{7caf8}','\u{c26be}','\u{c5132}'];
_19 = _28 | _28;
SetDiscriminant(_33, 1);
_34 = _10;
_39.fld0.4 = -_13;
_41.fld0.fld6.5 = -_17;
_31 = '\u{1f1a0}' as i32;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld1.1 = _41.fld0.fld1.1;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld2[_16] = !_14.1;
_10 = _24;
_41.fld3 = core::ptr::addr_of!(place!(Field::<Adt46>(Variant(_33, 1), 3)).fld0.1);
_31 = !1791800003_i32;
_41.fld2[_16] = RET;
_41.fld0.fld6.2.1 = -_34.0;
_39.fld0.1 = _13 as f64;
_34.0 = _41.fld0.fld6.2.1 << _4;
_17 = _25[_16] as i8;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld2[_16] = _41.fld2[_16];
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld6.2.2 = -_26;
place!(Field::<Adt46>(Variant(_33, 1), 3)).fld0.0 = _30;
_5 = _7 | _1;
Goto(bb13)
}
bb15 = {
_11 = !10645_u16;
_14.2 = -_2;
_8 = _23;
_27 = Adt54::Variant0 { fld0: _16 };
_10.0 = _21 - _21;
_34.0 = _10.0;
_24.0 = _10.0;
_29 = [Field::<u32>(Variant(_33, 0), 0),_25[_16]];
_15 = _31 as f32;
_10.1 = ['\u{6463}','\u{bf513}','\u{f40d2}','\u{c29b5}'];
_24 = _10;
_35.2 = _2;
_14 = (_10.1, RET, _2);
_14.1 = _20[_16];
_35.1 = Field::<usize>(Variant(_27, 0), 0) == Field::<usize>(Variant(_27, 0), 0);
_37.1 = !_9;
_20[_16] = !_35.1;
Goto(bb12)
}
bb16 = {
_13 = 15132010764746547289_u64 as isize;
_15 = _11 as f32;
_2 = _14.2;
_16 = 11668336596261633689_usize << _4;
_16 = _15 as usize;
_10.0 = (-12308228876669046987213134763954537177_i128);
_6 = _9 as isize;
RET = _9;
_3 = (-8048582093062053567_i64) >> _1;
_9 = _4 > _5;
_14.0 = ['\u{69593}','\u{68140}','\u{e1759}','\u{e4521}'];
_19 = !158604837980745228281106282337954507455_u128;
_6 = _2 as isize;
Goto(bb4)
}
bb17 = {
_14 = (_10.1, _9, _2);
_7 = _16 as u8;
_9 = _14.1;
_23 = !_8;
_8 = _5;
SetDiscriminant(_27, 0);
_10.0 = _21;
_22 = 283627610_i32 as f32;
_8 = !_1;
_14.1 = _20[_16];
_30 = _3;
RET = !_14.1;
_19 = 12434703903383153521_u64 as u128;
Goto(bb10)
}
bb18 = {
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld5.0 = _39.fld0.2 / _39.fld0.2;
_28 = !_19;
_37.0 = _14.0;
_41.fld0.fld5 = (Field::<Adt56>(Variant(_33, 1), 1).fld0.fld5.0,);
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld4.fld0.3 = core::ptr::addr_of!(_6);
_38 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld5);
_24 = _34;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld0 = !_9;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld1.0 = !_34.0;
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld6.1 = core::ptr::addr_of!((*_38));
place!(Field::<Adt46>(Variant(_33, 1), 3)).fld0.5 = core::ptr::addr_of!(_36);
place!(Field::<Adt56>(Variant(_33, 1), 1)).fld6 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_33, 1), 1)).fld0.fld6.0.1);
_25[_16] = _12.0 << _28;
(*_44) = [_25[_16]];
Goto(bb19)
}
bb19 = {
Call(_52 = dump_var(5_usize, 34_usize, Move(_34), 6_usize, Move(_6), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_52 = dump_var(5_usize, 24_usize, Move(_24), 7_usize, Move(_7), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_52 = dump_var(5_usize, 5_usize, Move(_5), 21_usize, Move(_21), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_52 = dump_var(5_usize, 29_usize, Move(_29), 11_usize, Move(_11), 53_usize, _53, 53_usize, _53), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: (i128, [char; 4]),mut _2: [char; 4],mut _3: bool,mut _4: bool,mut _5: i16,mut _6: bool,mut _7: bool,mut _8: u8) -> i8 {
mir! {
type RET = i8;
let _9: u128;
let _10: *mut u32;
let _11: bool;
let _12: isize;
let _13: [char; 4];
let _14: [u32; 7];
let _15: Adt54;
let _16: f32;
let _17: f32;
let _18: u128;
let _19: i32;
let _20: Adt48;
let _21: u32;
let _22: f32;
let _23: *const f64;
let _24: f32;
let _25: bool;
let _26: isize;
let _27: [u32; 1];
let _28: (i16, i128, f32);
let _29: Adt59;
let _30: [char; 6];
let _31: isize;
let _32: f64;
let _33: Adt50;
let _34: Adt50;
let _35: (i16, i128, f32);
let _36: i32;
let _37: (i16, i128, f32);
let _38: Adt54;
let _39: i128;
let _40: isize;
let _41: [char; 6];
let _42: i16;
let _43: Adt48;
let _44: (&'static i64, i16, bool, i8);
let _45: i8;
let _46: u64;
let _47: ();
let _48: ();
{
_1 = (36020176578957133145420141029741809254_i128, _2);
RET = (-85_i8);
_8 = 133_u8;
_9 = 2044809281_u32 as u128;
_4 = _6;
_7 = _3;
_8 = _5 as u8;
_13 = _2;
_13 = _1.1;
RET = (-38_i8) << _1.0;
_12 = -93_isize;
_9 = 16556586184812451869_u64 as u128;
_8 = !130_u8;
_13 = ['\u{47327}','\u{412e7}','\u{161b8}','\u{b87bc}'];
_6 = _7;
_3 = _6 > _6;
_1.1 = _13;
_6 = !_7;
_7 = _3 ^ _6;
Goto(bb1)
}
bb1 = {
_15 = Adt54::Variant0 { fld0: 11963525472954863909_usize };
_4 = RET != RET;
_15 = Adt54::Variant0 { fld0: 690113129870725442_usize };
_1 = (147526638628655261184887789205501265390_i128, _2);
_13 = _2;
_17 = _12 as f32;
RET = 16_i8;
_16 = _5 as f32;
_15 = Adt54::Variant0 { fld0: 0_usize };
RET = '\u{30e16}' as i8;
_16 = _17 + _17;
place!(Field::<usize>(Variant(_15, 0), 0)) = !17107966091285011380_usize;
_3 = _7;
_18 = _12 as u128;
_3 = !_4;
_17 = _16;
_5 = Field::<usize>(Variant(_15, 0), 0) as i16;
Call(_5 = fn7(_8, _3, _7, _3, _8, _7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = _9 | _9;
RET = !63_i8;
_19 = (-2031220619_i32);
_16 = -_17;
_11 = _7 > _3;
_4 = !_7;
_22 = -_16;
_7 = !_11;
_3 = _7 == _7;
_17 = _22 + _16;
match _1.0 {
147526638628655261184887789205501265390 => bb3,
_ => bb1
}
}
bb3 = {
_1.1 = ['\u{a015d}','\u{5f5ce}','\u{efae6}','\u{4845c}'];
_5 = 19209_i16;
_16 = _17 - _17;
_15 = Adt54::Variant1 { fld0: _18 };
_21 = !4249615393_u32;
_16 = _22 - _22;
_5 = -25538_i16;
_4 = !_7;
_6 = _4 != _4;
_10 = core::ptr::addr_of_mut!(_21);
_18 = _9 >> _1.0;
(*_10) = _18 as u32;
_13 = ['\u{8675f}','\u{bba1f}','\u{34945}','\u{89fef}'];
(*_10) = 3910226473_u32;
_8 = 34_u8;
_14 = [(*_10),(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
_2 = ['\u{5c139}','\u{dea58}','\u{9cd3e}','\u{dc41c}'];
RET = 26_i8 >> _19;
match _1.0 {
147526638628655261184887789205501265390 => bb5,
_ => bb4
}
}
bb4 = {
_18 = _9 | _9;
RET = !63_i8;
_19 = (-2031220619_i32);
_16 = -_17;
_11 = _7 > _3;
_4 = !_7;
_22 = -_16;
_7 = !_11;
_3 = _7 == _7;
_17 = _22 + _16;
match _1.0 {
147526638628655261184887789205501265390 => bb3,
_ => bb1
}
}
bb5 = {
_22 = RET as f32;
_15 = Adt54::Variant0 { fld0: 7023775679948616481_usize };
_19 = 1350927718_i32;
_22 = _17 * _17;
_19 = !2088889848_i32;
_16 = _22;
_24 = -_16;
RET = 65_i8;
_4 = _6 != _7;
Call(_4 = fn8(_6, _7, _1.0, _17, _3, _11, _11, _6, _6, _7, _7, _22, _11, _3, _1.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12 = (-9223372036854775808_isize);
_12 = (-9223372036854775808_isize) >> _18;
(*_10) = 3652776452_u32 | 620541312_u32;
_12 = 9223372036854775807_isize;
_15 = Adt54::Variant0 { fld0: 5_usize };
_17 = -_24;
_19 = -(-2093065622_i32);
_22 = -_24;
_25 = _6;
_20 = Adt48::Variant2 { fld0: _19 };
_25 = _7;
_12 = _19 as isize;
_4 = !_3;
Goto(bb7)
}
bb7 = {
(*_10) = _8 as u32;
_9 = !_18;
_16 = _22;
_6 = _4 > _4;
_9 = _18;
place!(Field::<i32>(Variant(_20, 2), 0)) = 16885135645547083135_u64 as i32;
SetDiscriminant(_20, 1);
_22 = _24 + _17;
_25 = _6 == _11;
place!(Field::<i64>(Variant(_20, 1), 3)) = !(-9018960147766839791_i64);
_2 = ['\u{8603a}','\u{29237}','\u{ef118}','\u{d7a43}'];
_22 = -_24;
_14 = [_21,_21,(*_10),(*_10),(*_10),(*_10),(*_10)];
_1 = ((-35085839132081286884100831119279997204_i128), _13);
place!(Field::<i64>(Variant(_20, 1), 3)) = 2840194812359438219_i64;
_28.2 = _17 - _24;
place!(Field::<i64>(Variant(_20, 1), 3)) = 3147730117853690982_i64;
(*_10) = !958652925_u32;
Goto(bb8)
}
bb8 = {
_28 = (_5, _1.0, _17);
(*_10) = 3774923030_u32 & 2951342101_u32;
place!(Field::<usize>(Variant(_15, 0), 0)) = !3_usize;
_1 = (_28.1, _2);
RET = _8 as i8;
_26 = (*_10) as isize;
_12 = _26;
_17 = _28.2 + _22;
_1.0 = _28.1;
_7 = _25 >= _11;
_14 = [_21,_21,_21,_21,(*_10),(*_10),_21];
Goto(bb9)
}
bb9 = {
place!(Field::<[u32; 1]>(Variant(_20, 1), 1)) = [(*_10)];
_28.2 = _22 + _17;
_21 = 1685209057_u32;
_21 = 3214484977_u32;
_12 = _26;
_25 = !_6;
place!(Field::<usize>(Variant(_15, 0), 0)) = !0_usize;
RET = 126_i8;
_1 = (_28.1, _2);
_12 = _26 | _26;
_7 = !_4;
_1 = (_28.1, _13);
_1.1 = _2;
_28 = (_5, _1.0, _17);
_11 = _24 > _17;
SetDiscriminant(_15, 1);
_14 = [(*_10),_21,(*_10),_21,(*_10),_21,(*_10)];
_28.0 = _19 as i16;
_5 = 6051895910587606320_usize as i16;
_9 = _18;
_32 = _18 as f64;
_31 = -_26;
RET = -3_i8;
(*_10) = !3153042585_u32;
Call(place!(Field::<*const f64>(Variant(_20, 1), 2)) = fn9(_24, _4, _6, _25, _7, _12, _5, _11, _4, _6, _22, _28), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<*const f64>(Variant(_20, 1), 2)) = core::ptr::addr_of!(_32);
place!(Field::<*const f64>(Variant(_20, 1), 2)) = core::ptr::addr_of!(_32);
_28.2 = -_24;
_1.1 = ['\u{1040c7}','\u{104943}','\u{87e9c}','\u{78b32}'];
_13 = ['\u{70d39}','\u{4c2ca}','\u{10847d}','\u{adbb}'];
_7 = _6 ^ _6;
_35.1 = _1.0 * _1.0;
place!(Field::<u128>(Variant(_15, 1), 0)) = '\u{2c4ac}' as u128;
_35.1 = !_28.1;
_35 = (_28.0, _28.1, _17);
_23 = Field::<*const f64>(Variant(_20, 1), 2);
place!(Field::<i64>(Variant(_20, 1), 3)) = -(-4967413697580133001_i64);
(*_23) = 16229091689560573721_usize as f64;
_36 = !_19;
SetDiscriminant(_15, 1);
_4 = _6;
match _28.1 {
305196527788857176579273776312488214252 => bb12,
_ => bb11
}
}
bb11 = {
_22 = RET as f32;
_15 = Adt54::Variant0 { fld0: 7023775679948616481_usize };
_19 = 1350927718_i32;
_22 = _17 * _17;
_19 = !2088889848_i32;
_16 = _22;
_24 = -_16;
RET = 65_i8;
_4 = _6 != _7;
Call(_4 = fn8(_6, _7, _1.0, _17, _3, _11, _11, _6, _6, _7, _7, _22, _11, _3, _1.1), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_35.2 = _24 - _28.2;
_1.1 = ['\u{afefd}','\u{b30c1}','\u{2eedf}','\u{31b28}'];
_12 = _31;
_1 = (_35.1, _2);
RET = (-115_i8) + 113_i8;
_28 = (_35.0, _35.1, _24);
place!(Field::<*const f64>(Variant(_20, 1), 2)) = core::ptr::addr_of!((*_23));
_15 = Adt54::Variant1 { fld0: _9 };
SetDiscriminant(_15, 0);
_1.1 = _13;
_37.0 = _5;
_24 = Field::<i64>(Variant(_20, 1), 3) as f32;
_1.0 = _28.1;
(*_23) = _36 as f64;
match _1.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb5,
305196527788857176579273776312488214252 => bb13,
_ => bb8
}
}
bb13 = {
place!(Field::<[char; 6]>(Variant(_20, 1), 0)) = ['\u{5f246}','\u{b3c73}','\u{e4695}','\u{80540}','\u{f2b78}','\u{10aa44}'];
_8 = !229_u8;
place!(Field::<[char; 6]>(Variant(_20, 1), 0)) = ['\u{d4104}','\u{31f35}','\u{6e885}','\u{d438c}','\u{80db6}','\u{ef28d}'];
(*_10) = _35.2 as u32;
place!(Field::<i64>(Variant(_20, 1), 3)) = (-7166575314039830464_i64);
_28 = _35;
place!(Field::<[char; 6]>(Variant(_20, 1), 0)) = ['\u{6543c}','\u{23696}','\u{d025c}','\u{983d3}','\u{9e408}','\u{fcf43}'];
_15 = Adt54::Variant0 { fld0: 5368109839496281238_usize };
_14 = [(*_10),(*_10),_21,(*_10),(*_10),_21,(*_10)];
SetDiscriminant(_20, 2);
_37.2 = _16;
_35.0 = _9 as i16;
_27 = [(*_10)];
_40 = _31;
_3 = _6;
_32 = 55644_u16 as f64;
_10 = core::ptr::addr_of_mut!((*_10));
_41 = ['\u{95aed}','\u{9a786}','\u{6b3a8}','\u{eabc7}','\u{72ffa}','\u{8cef9}'];
RET = _32 as i8;
_21 = 4232728832_u32;
place!(Field::<i32>(Variant(_20, 2), 0)) = _36 >> _9;
_8 = 94_u8;
_1.0 = _17 as i128;
match _8 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb11,
6 => bb12,
94 => bb14,
_ => bb8
}
}
bb14 = {
_2 = ['\u{683c4}','\u{ce390}','\u{bc559}','\u{34a58}'];
SetDiscriminant(_20, 0);
_28 = _35;
_35.0 = 15452949061224935393_u64 as i16;
place!(Field::<(usize,)>(Variant(_20, 0), 1)).0 = 2703_u16 as usize;
_42 = Field::<(usize,)>(Variant(_20, 0), 1).0 as i16;
_22 = _17 - _28.2;
place!(Field::<usize>(Variant(_15, 0), 0)) = Field::<(usize,)>(Variant(_20, 0), 1).0;
place!(Field::<Adt44>(Variant(_20, 0), 3)) = Adt44::Variant0 { fld0: _12 };
_1.1 = _2;
place!(Field::<u128>(Variant(_20, 0), 0)) = !_9;
(*_10) = _1.0 as u32;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(6_usize, 40_usize, Move(_40), 21_usize, Move(_21), 6_usize, Move(_6), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(6_usize, 9_usize, Move(_9), 4_usize, Move(_4), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(6_usize, 19_usize, Move(_19), 27_usize, Move(_27), 14_usize, Move(_14), 41_usize, Move(_41)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u8,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: u8,mut _6: bool,mut _7: [char; 4]) -> i16 {
mir! {
type RET = i16;
let _8: Adt46;
let _9: *mut isize;
let _10: u32;
let _11: isize;
let _12: isize;
let _13: [char; 4];
let _14: i8;
let _15: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8);
let _16: bool;
let _17: u32;
let _18: [bool; 7];
let _19: f64;
let _20: u64;
let _21: isize;
let _22: isize;
let _23: *const f64;
let _24: ();
let _25: ();
{
_7 = ['\u{dc494}','\u{ebccf}','\u{7bb32}','\u{3c28e}'];
_1 = _5;
RET = 29024_i16;
_1 = 55630_u16 as u8;
_4 = _3;
_5 = !_1;
RET = 44_i8 as i16;
_3 = _4 | _4;
_5 = _1;
_3 = !_6;
_7 = ['\u{75092}','\u{1754f}','\u{7ef96}','\u{fdc5d}'];
_6 = _4;
_7 = ['\u{4defb}','\u{41b0c}','\u{f4ce8}','\u{902d8}'];
_6 = _4;
_3 = _2 <= _6;
_6 = _3 | _3;
_8.fld0.2 = 7_usize + 1_usize;
_8.fld0.0 = 9185846024377858778_i64;
_8.fld0.2 = 7641744734867782556_usize;
RET = (-12582_i16);
_8.fld0.2 = 1_usize & 8593693369367042138_usize;
_9 = core::ptr::addr_of_mut!(_8.fld0.4);
_8.fld0.3 = core::ptr::addr_of!((*_9));
_8.fld0.2 = !7_usize;
_4 = !_3;
Goto(bb1)
}
bb1 = {
_8.fld0.2 = (-9223372036854775808_isize) as usize;
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
_8.fld0.4 = (-9223372036854775808_isize) + 63_isize;
_8.fld0.1 = _8.fld0.0 as f64;
_8.fld0.3 = core::ptr::addr_of!((*_9));
_3 = !_6;
_8.fld0.0 = RET as i64;
_8.fld0.4 = !(-9223372036854775808_isize);
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
RET = 5102_i16;
_10 = 3187293750_u32 << _5;
_5 = _1;
RET = 27248_i16;
_12 = (*_9) - _8.fld0.4;
_4 = _3;
(*_9) = -_12;
_15.5 = 125_i8;
_15.2.2 = 1923963605_i32 as f32;
_14 = _8.fld0.1 as i8;
RET = _12 as i16;
_15.0 = ((-48990453727405434219373324865502538498_i128), _7);
RET = (-9088_i16) << _15.0.0;
_8.fld0.1 = (*_9) as f64;
Goto(bb2)
}
bb2 = {
_15.3 = -_14;
match _15.0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
291291913193533029244001282566265672958 => bb8,
_ => bb7
}
}
bb3 = {
_8.fld0.2 = (-9223372036854775808_isize) as usize;
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
_8.fld0.4 = (-9223372036854775808_isize) + 63_isize;
_8.fld0.1 = _8.fld0.0 as f64;
_8.fld0.3 = core::ptr::addr_of!((*_9));
_3 = !_6;
_8.fld0.0 = RET as i64;
_8.fld0.4 = !(-9223372036854775808_isize);
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
_8.fld0.5 = core::ptr::addr_of!(_8.fld0.1);
RET = 5102_i16;
_10 = 3187293750_u32 << _5;
_5 = _1;
RET = 27248_i16;
_12 = (*_9) - _8.fld0.4;
_4 = _3;
(*_9) = -_12;
_15.5 = 125_i8;
_15.2.2 = 1923963605_i32 as f32;
_14 = _8.fld0.1 as i8;
RET = _12 as i16;
_15.0 = ((-48990453727405434219373324865502538498_i128), _7);
RET = (-9088_i16) << _15.0.0;
_8.fld0.1 = (*_9) as f64;
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
_13 = _15.0.1;
_15.0 = ((-124079763745349701164194103990058409365_i128), _7);
_15.4 = 52476_u16 + 26116_u16;
_15.3 = _14;
_15.2.1 = -_15.0.0;
_8.fld0.0 = (-4663908257676561622_i64) + 1407198215511381624_i64;
_15.2.1 = -_15.0.0;
_4 = _3;
_15.4 = 14483_u16 << _15.2.1;
_8.fld0.4 = _12;
_15.4 = _8.fld0.1 as u16;
_15.2.1 = 9555987477709179190_u64 as i128;
_15.0.0 = _10 as i128;
Goto(bb9)
}
bb9 = {
_4 = !_2;
_8.fld0.0 = (-6748583743176744647_i64);
_2 = !_3;
_14 = _15.5 + _15.5;
_16 = _6 != _3;
_15.0.0 = '\u{70ac}' as i128;
_8.fld0.3 = core::ptr::addr_of!(_12);
_1 = !_5;
_17 = !_10;
_15.2.1 = _15.0.0 & _15.0.0;
_18 = [_6,_6,_16,_16,_6,_16,_3];
_8.fld0.1 = 996303951_i32 as f64;
_15.4 = 18254_u16 * 6323_u16;
match _15.5 {
0 => bb10,
1 => bb11,
2 => bb12,
125 => bb14,
_ => bb13
}
}
bb10 = {
_13 = _15.0.1;
_15.0 = ((-124079763745349701164194103990058409365_i128), _7);
_15.4 = 52476_u16 + 26116_u16;
_15.3 = _14;
_15.2.1 = -_15.0.0;
_8.fld0.0 = (-4663908257676561622_i64) + 1407198215511381624_i64;
_15.2.1 = -_15.0.0;
_4 = _3;
_15.4 = 14483_u16 << _15.2.1;
_8.fld0.4 = _12;
_15.4 = _8.fld0.1 as u16;
_15.2.1 = 9555987477709179190_u64 as i128;
_15.0.0 = _10 as i128;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_11 = _15.2.2 as isize;
_8.fld0.2 = 18076401740464087541_usize;
_15.2.1 = _8.fld0.0 as i128;
_15.5 = 2772674871585253323_u64 as i8;
(*_9) = !_11;
_8.fld0.0 = 6786831876579766782_i64 ^ (-970376841807684377_i64);
_5 = _1;
_15.2.2 = _8.fld0.0 as f32;
_4 = _6 | _6;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(7_usize, 18_usize, Move(_18), 1_usize, Move(_1), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(7_usize, 10_usize, Move(_10), 4_usize, Move(_4), 14_usize, Move(_14), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: i128,mut _4: f32,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: f32,mut _13: bool,mut _14: bool,mut _15: [char; 4]) -> bool {
mir! {
type RET = bool;
let _16: *const u8;
let _17: u64;
let _18: [u32; 7];
let _19: *const (usize,);
let _20: isize;
let _21: ();
let _22: ();
{
_9 = _8 ^ _7;
_1 = _5 < _2;
_13 = _5 <= _7;
RET = _9;
_18 = [3919055585_u32,1129520351_u32,1814300674_u32,1918154157_u32,596038404_u32,3349103026_u32,1344582971_u32];
_14 = !_7;
_2 = !_1;
_5 = _7;
_17 = (-5_i8) as u64;
_15 = ['\u{51b5d}','\u{2331e}','\u{14d1d}','\u{8976b}'];
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(8_usize, 2_usize, Move(_2), 3_usize, Move(_3), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_21 = dump_var(8_usize, 17_usize, Move(_17), 9_usize, Move(_9), 1_usize, Move(_1), 22_usize, _22), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: f32,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: isize,mut _7: i16,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: f32,mut _12: (i16, i128, f32)) -> *const f64 {
mir! {
type RET = *const f64;
let _13: isize;
let _14: isize;
let _15: *mut u32;
let _16: char;
let _17: (i128, [char; 4]);
let _18: Adt48;
let _19: (u32,);
let _20: (i16, i128, f32);
let _21: u64;
let _22: u16;
let _23: [bool; 7];
let _24: [char; 4];
let _25: u8;
let _26: char;
let _27: [char; 4];
let _28: (u32,);
let _29: i128;
let _30: f64;
let _31: f64;
let _32: Adt54;
let _33: (i16, i128, f32);
let _34: f64;
let _35: i128;
let _36: i128;
let _37: ([char; 4], bool, i16);
let _38: *const (usize,);
let _39: (i16, i128, f32);
let _40: u32;
let _41: u8;
let _42: [u32; 2];
let _43: *const isize;
let _44: [u32; 7];
let _45: [u32; 7];
let _46: [i64; 8];
let _47: bool;
let _48: [u32; 2];
let _49: ();
let _50: ();
{
_11 = _12.1 as f32;
_10 = _9;
_6 = 7369601101422189291_i64 as isize;
_7 = _6 as i16;
_9 = !_4;
_10 = !_4;
_12 = (_7, 165058202489638780521301073666340703681_i128, _1);
_6 = (-9223372036854775808_isize);
_12 = (_7, 34427930055043244742831265649488925889_i128, _11);
_12 = (_7, 93884515207579928458924100999147352988_i128, _1);
_8 = _9 != _9;
_13 = _6 >> _6;
_1 = 2_usize as f32;
Call(_7 = core::intrinsics::bswap(_12.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 550294460_u32 as f32;
_14 = !_13;
_1 = -_12.2;
_3 = _1 > _12.2;
_14 = _6 + _13;
_12.1 = (-62_i8) as i128;
_12.2 = _1;
_14 = _5 as isize;
_12.1 = 20166228838169163927569089348178249287_i128 >> _14;
_12.2 = _1;
_2 = !_10;
_1 = -_12.2;
_8 = _10;
_14 = 106_i8 as isize;
_6 = _14;
_1 = _12.2 - _12.2;
_3 = _8;
_14 = _6;
_17.1 = ['\u{100650}','\u{c36d7}','\u{d435f}','\u{48860}'];
Call(_20.1 = fn10(_9, _8, _9, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _5 == _8;
_21 = 3_usize as u64;
_15 = core::ptr::addr_of_mut!(_19.0);
_3 = _5;
(*_15) = 2169623570_u32;
_5 = _2 | _8;
_20 = (_7, _12.1, _11);
_1 = -_12.2;
_4 = _5;
_7 = _20.0;
_20.0 = '\u{931fb}' as i16;
(*_15) = 3709098318_u32;
_9 = _3 != _3;
_9 = _8;
_12.0 = _12.1 as i16;
(*_15) = 2858965050_u32 * 2745325214_u32;
_10 = _9;
_4 = !_2;
Goto(bb3)
}
bb3 = {
_1 = _20.2;
_23 = [_10,_9,_4,_5,_4,_2,_4];
_9 = !_2;
_12.0 = _20.0 | _20.0;
_20.1 = _12.1 + _12.1;
_14 = 204496470101192906032617459639532682950_u128 as isize;
_19.0 = !1373522588_u32;
_14 = !_13;
_19 = (33990458_u32,);
_12.1 = _20.1 >> _20.1;
_6 = _14;
_6 = _14 ^ _14;
_5 = _10;
_21 = 12433311558644082781_usize as u64;
_8 = _5 >= _5;
_2 = _10;
_11 = _12.2 * _12.2;
_25 = 12_u8 + 138_u8;
_12.0 = -_7;
_26 = '\u{1f612}';
_5 = _9;
_24 = _17.1;
_18 = Adt48::Variant2 { fld0: (-1567376134_i32) };
_28 = _19;
_22 = 56936_u16 * 17845_u16;
Goto(bb4)
}
bb4 = {
(*_15) = _28.0 ^ _28.0;
_8 = !_3;
_1 = _11;
_26 = '\u{20aff}';
_4 = _20.1 <= _20.1;
place!(Field::<i32>(Variant(_18, 2), 0)) = _6 as i32;
_1 = _25 as f32;
_14 = _6 ^ _6;
_21 = _14 as u64;
_26 = '\u{1c7d4}';
RET = core::ptr::addr_of!(_30);
_27 = [_26,_26,_26,_26];
_31 = _12.1 as f64;
(*RET) = _31 * _31;
_30 = Field::<i32>(Variant(_18, 2), 0) as f64;
_17 = (_20.1, _24);
_7 = _20.0 >> _17.0;
_13 = _14;
_8 = _10;
_15 = core::ptr::addr_of_mut!((*_15));
_17 = (_20.1, _24);
_16 = _26;
_12.2 = -_11;
_7 = !_12.0;
_14 = _13;
RET = core::ptr::addr_of!(_30);
match _28.0 {
0 => bb2,
1 => bb5,
33990458 => bb7,
_ => bb6
}
}
bb5 = {
_1 = _20.2;
_23 = [_10,_9,_4,_5,_4,_2,_4];
_9 = !_2;
_12.0 = _20.0 | _20.0;
_20.1 = _12.1 + _12.1;
_14 = 204496470101192906032617459639532682950_u128 as isize;
_19.0 = !1373522588_u32;
_14 = !_13;
_19 = (33990458_u32,);
_12.1 = _20.1 >> _20.1;
_6 = _14;
_6 = _14 ^ _14;
_5 = _10;
_21 = 12433311558644082781_usize as u64;
_8 = _5 >= _5;
_2 = _10;
_11 = _12.2 * _12.2;
_25 = 12_u8 + 138_u8;
_12.0 = -_7;
_26 = '\u{1f612}';
_5 = _9;
_24 = _17.1;
_18 = Adt48::Variant2 { fld0: (-1567376134_i32) };
_28 = _19;
_22 = 56936_u16 * 17845_u16;
Goto(bb4)
}
bb6 = {
_11 = 550294460_u32 as f32;
_14 = !_13;
_1 = -_12.2;
_3 = _1 > _12.2;
_14 = _6 + _13;
_12.1 = (-62_i8) as i128;
_12.2 = _1;
_14 = _5 as isize;
_12.1 = 20166228838169163927569089348178249287_i128 >> _14;
_12.2 = _1;
_2 = !_10;
_1 = -_12.2;
_8 = _10;
_14 = 106_i8 as isize;
_6 = _14;
_1 = _12.2 - _12.2;
_3 = _8;
_14 = _6;
_17.1 = ['\u{100650}','\u{c36d7}','\u{d435f}','\u{48860}'];
Call(_20.1 = fn10(_9, _8, _9, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_4 = !_9;
_31 = _25 as f64;
_16 = _26;
_33.0 = 7495137614659200435_usize as i16;
_12 = (_7, _20.1, _11);
_25 = 1202697989411713800_i64 as u8;
_20.0 = _33.0;
_23 = [_5,_8,_8,_4,_3,_2,_4];
(*_15) = _28.0;
_32 = Adt54::Variant1 { fld0: 173575065804023829934932593868090515378_u128 };
_5 = _9 & _9;
_25 = 129_u8;
(*RET) = _31 + _31;
Call(_37.0 = fn11(_23, _5, _4, _4, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
SetDiscriminant(_18, 1);
place!(Field::<*const f64>(Variant(_18, 1), 2)) = RET;
_24 = [_16,_26,_16,_26];
_7 = !_12.0;
place!(Field::<i64>(Variant(_18, 1), 3)) = (*RET) as i64;
_39 = (_12.0, _20.1, _12.2);
_12.0 = !_7;
_36 = -_17.0;
Call(_19.0 = fn15(_17.0, _9, _12, _20.1, _39, RET, _3, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_15) = !_28.0;
_3 = _9 < _5;
_37.2 = _7;
_39.2 = _12.2 * _11;
_20.0 = _12.0;
_3 = _2;
_17.0 = _12.1;
_20 = (_7, _36, _39.2);
place!(Field::<u128>(Variant(_32, 1), 0)) = 313306866668615043398327496778133145694_u128 << _17.0;
_17 = (_20.1, _37.0);
SetDiscriminant(_32, 1);
place!(Field::<[u32; 1]>(Variant(_18, 1), 1)) = [_19.0];
_23 = [_3,_4,_9,_2,_5,_4,_9];
match _25 {
129 => bb11,
_ => bb10
}
}
bb10 = {
_1 = _20.2;
_23 = [_10,_9,_4,_5,_4,_2,_4];
_9 = !_2;
_12.0 = _20.0 | _20.0;
_20.1 = _12.1 + _12.1;
_14 = 204496470101192906032617459639532682950_u128 as isize;
_19.0 = !1373522588_u32;
_14 = !_13;
_19 = (33990458_u32,);
_12.1 = _20.1 >> _20.1;
_6 = _14;
_6 = _14 ^ _14;
_5 = _10;
_21 = 12433311558644082781_usize as u64;
_8 = _5 >= _5;
_2 = _10;
_11 = _12.2 * _12.2;
_25 = 12_u8 + 138_u8;
_12.0 = -_7;
_26 = '\u{1f612}';
_5 = _9;
_24 = _17.1;
_18 = Adt48::Variant2 { fld0: (-1567376134_i32) };
_28 = _19;
_22 = 56936_u16 * 17845_u16;
Goto(bb4)
}
bb11 = {
_3 = _2;
_32 = Adt54::Variant0 { fld0: 0_usize };
_5 = _3;
_9 = _4;
(*RET) = _31;
_33.2 = _12.2;
_33 = (_12.0, _12.1, _20.2);
(*RET) = _31;
_29 = !_36;
_19.0 = !_28.0;
match _28.0 {
0 => bb12,
1 => bb13,
33990458 => bb15,
_ => bb14
}
}
bb12 = {
_1 = _20.2;
_23 = [_10,_9,_4,_5,_4,_2,_4];
_9 = !_2;
_12.0 = _20.0 | _20.0;
_20.1 = _12.1 + _12.1;
_14 = 204496470101192906032617459639532682950_u128 as isize;
_19.0 = !1373522588_u32;
_14 = !_13;
_19 = (33990458_u32,);
_12.1 = _20.1 >> _20.1;
_6 = _14;
_6 = _14 ^ _14;
_5 = _10;
_21 = 12433311558644082781_usize as u64;
_8 = _5 >= _5;
_2 = _10;
_11 = _12.2 * _12.2;
_25 = 12_u8 + 138_u8;
_12.0 = -_7;
_26 = '\u{1f612}';
_5 = _9;
_24 = _17.1;
_18 = Adt48::Variant2 { fld0: (-1567376134_i32) };
_28 = _19;
_22 = 56936_u16 * 17845_u16;
Goto(bb4)
}
bb13 = {
(*_15) = !_28.0;
_3 = _9 < _5;
_37.2 = _7;
_39.2 = _12.2 * _11;
_20.0 = _12.0;
_3 = _2;
_17.0 = _12.1;
_20 = (_7, _36, _39.2);
place!(Field::<u128>(Variant(_32, 1), 0)) = 313306866668615043398327496778133145694_u128 << _17.0;
_17 = (_20.1, _37.0);
SetDiscriminant(_32, 1);
place!(Field::<[u32; 1]>(Variant(_18, 1), 1)) = [_19.0];
_23 = [_3,_4,_9,_2,_5,_4,_9];
match _25 {
129 => bb11,
_ => bb10
}
}
bb14 = {
_4 = !_9;
_31 = _25 as f64;
_16 = _26;
_33.0 = 7495137614659200435_usize as i16;
_12 = (_7, _20.1, _11);
_25 = 1202697989411713800_i64 as u8;
_20.0 = _33.0;
_23 = [_5,_8,_8,_4,_3,_2,_4];
(*_15) = _28.0;
_32 = Adt54::Variant1 { fld0: 173575065804023829934932593868090515378_u128 };
_5 = _9 & _9;
_25 = 129_u8;
(*RET) = _31 + _31;
Call(_37.0 = fn11(_23, _5, _4, _4, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb15 = {
_17.1 = [_16,_16,_16,_26];
_6 = _13;
_21 = 11215338664972977465_u64;
_23 = [_10,_4,_2,_9,_2,_5,_3];
_11 = _33.2 * _20.2;
_30 = _31;
_19 = (_28.0,);
_19 = _28;
_11 = _12.2;
_30 = _31;
_24 = _27;
(*RET) = _31 + _31;
_31 = -(*RET);
_12.2 = -_20.2;
_37.1 = !_9;
_20 = _33;
_43 = core::ptr::addr_of!(_13);
_25 = !121_u8;
_35 = _12.1 & _29;
(*RET) = -_31;
_44 = [_19.0,(*_15),(*_15),_19.0,(*_15),(*_15),_28.0];
_34 = (*RET) + _31;
_16 = _26;
_43 = core::ptr::addr_of!(_14);
_10 = !_3;
_48 = [_28.0,(*_15)];
Goto(bb16)
}
bb16 = {
Call(_49 = dump_var(9_usize, 16_usize, Move(_16), 44_usize, Move(_44), 48_usize, Move(_48), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(9_usize, 27_usize, Move(_27), 23_usize, Move(_23), 22_usize, Move(_22), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(9_usize, 21_usize, Move(_21), 10_usize, Move(_10), 25_usize, Move(_25), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_49 = dump_var(9_usize, 8_usize, Move(_8), 17_usize, Move(_17), 50_usize, _50, 50_usize, _50), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool) -> i128 {
mir! {
type RET = i128;
let _5: Adt57;
let _6: bool;
let _7: [char; 6];
let _8: u8;
let _9: *const [char; 4];
let _10: u128;
let _11: [i64; 8];
let _12: Adt54;
let _13: [u32; 7];
let _14: [bool; 7];
let _15: f64;
let _16: u8;
let _17: u16;
let _18: isize;
let _19: *const [u32; 1];
let _20: (i16, i128, f32);
let _21: f32;
let _22: isize;
let _23: [u32; 2];
let _24: f64;
let _25: isize;
let _26: Adt46;
let _27: u128;
let _28: bool;
let _29: *mut (i128, [char; 4]);
let _30: isize;
let _31: Adt45;
let _32: f64;
let _33: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8);
let _34: (i128, [char; 4]);
let _35: isize;
let _36: char;
let _37: ();
let _38: ();
{
Goto(bb1)
}
bb1 = {
_5.fld5.fld0.fld5.0 = 0_usize + 7_usize;
_5.fld5.fld0.fld6.4 = !19288_u16;
RET = (-2101029493246004863708200278293906829_i128) & (-94318162766515679758249869742788080568_i128);
_5.fld5.fld3 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_4 = !_2;
_9 = core::ptr::addr_of!(_5.fld5.fld0.fld1.1);
_5.fld5.fld0.fld1.1 = ['\u{f8707}','\u{4be00}','\u{7a87d}','\u{8d24f}'];
_5.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_5.fld5.fld0.fld6.0.0 = !RET;
(*_9) = ['\u{94f91}','\u{9ded6}','\u{2631d}','\u{9d932}'];
_8 = !32_u8;
_5.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_5.fld5.fld0.fld6.4 = 12199_u16;
_5.fld5.fld2 = [_3,_1,_1,_3,_2,_1,_1];
_4 = _3;
_5.fld2 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_11 = [(-2862782675996000697_i64),317441547721025980_i64,3770147411891610408_i64,(-1915218704188649048_i64),(-535593128297980910_i64),(-611401726800453268_i64),6140173169875803438_i64,3488168808115753087_i64];
_5.fld5.fld6 = core::ptr::addr_of!((*_9));
_5.fld5.fld0.fld6.2.1 = _5.fld5.fld0.fld6.0.0 & RET;
_5.fld5.fld0.fld6.2.0 = -5011_i16;
(*_9) = ['\u{daf8f}','\u{21447}','\u{354e4}','\u{75762}'];
_5.fld5.fld0.fld6.0.0 = _8 as i128;
match _5.fld5.fld0.fld6.4 {
12199 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_5.fld5.fld0.fld6.5 = 123_i8 >> _5.fld5.fld0.fld6.2.0;
_5.fld5.fld0.fld3 = 254308008337290786073600378610287751611_u128;
_5.fld5.fld2 = [_4,_4,_3,_3,_3,_4,_2];
_12 = Adt54::Variant1 { fld0: _5.fld5.fld0.fld3 };
_5.fld5.fld0.fld1.0 = -RET;
_5.fld5.fld0.fld4.fld0.2 = _5.fld5.fld0.fld5.0;
_7 = ['\u{7d2c1}','\u{b0415}','\u{ceba3}','\u{c51cc}','\u{3dac4}','\u{546d9}'];
_5.fld5.fld0.fld6.2.0 = !32534_i16;
SetDiscriminant(_12, 0);
_5.fld5.fld0.fld4.fld0.0 = -8007322403321483374_i64;
_5.fld5.fld5.0 = 908730393_u32 - 2085765809_u32;
_5.fld6 = core::ptr::addr_of!(_5.fld5.fld0.fld5);
_5.fld5.fld0.fld6.4 = 25331_u16 >> _5.fld5.fld0.fld6.5;
_5.fld5.fld4 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_5.fld5.fld0.fld6.0.0 = _5.fld5.fld0.fld1.0;
_5.fld5.fld0.fld4.fld0.4 = (-101_isize);
_5.fld5.fld0.fld5 = (_5.fld5.fld0.fld4.fld0.2,);
_14 = [_4,_2,_3,_2,_1,_1,_1];
_5.fld5.fld0.fld3 = _5.fld5.fld0.fld4.fld0.0 as u128;
_5.fld5.fld0.fld6.3 = -_5.fld5.fld0.fld6.5;
_5.fld5.fld0.fld6.0 = (_5.fld5.fld0.fld1.0, (*_9));
_5.fld3 = _5.fld5.fld0.fld1.1;
_5.fld3 = ['\u{75beb}','\u{d61a6}','\u{1bf39}','\u{b78e3}'];
_5.fld5.fld0.fld6.0 = (_5.fld5.fld0.fld1.0, _5.fld5.fld0.fld1.1);
(*_9) = _5.fld3;
_5.fld5.fld0.fld4.fld0.4 = -9223372036854775807_isize;
_5.fld5.fld0.fld4.fld0.1 = _8 as f64;
Goto(bb4)
}
bb4 = {
_5.fld5.fld0.fld5 = (_5.fld5.fld0.fld4.fld0.2,);
_12 = Adt54::Variant1 { fld0: _5.fld5.fld0.fld3 };
_5.fld5.fld0.fld4.fld0.4 = 94_isize;
_5.fld5.fld0.fld6.0.1 = ['\u{cb56e}','\u{10d334}','\u{dce2e}','\u{afdc6}'];
_5.fld2 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_5.fld5.fld4 = _5.fld2;
_5.fld3 = (*_9);
_16 = _8;
_5.fld5.fld0.fld6.0.1 = ['\u{bdb84}','\u{f5d15}','\u{f971a}','\u{63008}'];
_5.fld6 = core::ptr::addr_of!(_5.fld5.fld0.fld5);
_18 = _5.fld5.fld0.fld4.fld0.4;
_2 = _3 <= _3;
_7 = ['\u{5f818}','\u{8acc4}','\u{777c2}','\u{3c1f5}','\u{b0fb1}','\u{97b0c}'];
_10 = Field::<u128>(Variant(_12, 1), 0);
RET = !_5.fld5.fld0.fld6.2.1;
_5.fld5.fld0.fld5.0 = _5.fld5.fld0.fld4.fld0.2 ^ _5.fld5.fld0.fld4.fld0.2;
_5.fld5.fld0.fld0 = _2 & _3;
_12 = Adt54::Variant1 { fld0: _10 };
_5.fld5.fld0.fld6.2.1 = _5.fld5.fld0.fld1.0 ^ RET;
RET = !_5.fld5.fld0.fld6.0.0;
match _5.fld5.fld0.fld4.fld0.4 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
94 => bb10,
_ => bb9
}
}
bb5 = {
_5.fld5.fld0.fld6.5 = 123_i8 >> _5.fld5.fld0.fld6.2.0;
_5.fld5.fld0.fld3 = 254308008337290786073600378610287751611_u128;
_5.fld5.fld2 = [_4,_4,_3,_3,_3,_4,_2];
_12 = Adt54::Variant1 { fld0: _5.fld5.fld0.fld3 };
_5.fld5.fld0.fld1.0 = -RET;
_5.fld5.fld0.fld4.fld0.2 = _5.fld5.fld0.fld5.0;
_7 = ['\u{7d2c1}','\u{b0415}','\u{ceba3}','\u{c51cc}','\u{3dac4}','\u{546d9}'];
_5.fld5.fld0.fld6.2.0 = !32534_i16;
SetDiscriminant(_12, 0);
_5.fld5.fld0.fld4.fld0.0 = -8007322403321483374_i64;
_5.fld5.fld5.0 = 908730393_u32 - 2085765809_u32;
_5.fld6 = core::ptr::addr_of!(_5.fld5.fld0.fld5);
_5.fld5.fld0.fld6.4 = 25331_u16 >> _5.fld5.fld0.fld6.5;
_5.fld5.fld4 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_5.fld5.fld0.fld6.0.0 = _5.fld5.fld0.fld1.0;
_5.fld5.fld0.fld4.fld0.4 = (-101_isize);
_5.fld5.fld0.fld5 = (_5.fld5.fld0.fld4.fld0.2,);
_14 = [_4,_2,_3,_2,_1,_1,_1];
_5.fld5.fld0.fld3 = _5.fld5.fld0.fld4.fld0.0 as u128;
_5.fld5.fld0.fld6.3 = -_5.fld5.fld0.fld6.5;
_5.fld5.fld0.fld6.0 = (_5.fld5.fld0.fld1.0, (*_9));
_5.fld3 = _5.fld5.fld0.fld1.1;
_5.fld3 = ['\u{75beb}','\u{d61a6}','\u{1bf39}','\u{b78e3}'];
_5.fld5.fld0.fld6.0 = (_5.fld5.fld0.fld1.0, _5.fld5.fld0.fld1.1);
(*_9) = _5.fld3;
_5.fld5.fld0.fld4.fld0.4 = -9223372036854775807_isize;
_5.fld5.fld0.fld4.fld0.1 = _8 as f64;
Goto(bb4)
}
bb6 = {
Return()
}
bb7 = {
_5.fld5.fld0.fld5.0 = 0_usize + 7_usize;
_5.fld5.fld0.fld6.4 = !19288_u16;
RET = (-2101029493246004863708200278293906829_i128) & (-94318162766515679758249869742788080568_i128);
_5.fld5.fld3 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_4 = !_2;
_9 = core::ptr::addr_of!(_5.fld5.fld0.fld1.1);
_5.fld5.fld0.fld1.1 = ['\u{f8707}','\u{4be00}','\u{7a87d}','\u{8d24f}'];
_5.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_5.fld5.fld0.fld6.0.0 = !RET;
(*_9) = ['\u{94f91}','\u{9ded6}','\u{2631d}','\u{9d932}'];
_8 = !32_u8;
_5.fld5.fld0.fld4.fld0.5 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.1);
_5.fld5.fld0.fld6.4 = 12199_u16;
_5.fld5.fld2 = [_3,_1,_1,_3,_2,_1,_1];
_4 = _3;
_5.fld2 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_11 = [(-2862782675996000697_i64),317441547721025980_i64,3770147411891610408_i64,(-1915218704188649048_i64),(-535593128297980910_i64),(-611401726800453268_i64),6140173169875803438_i64,3488168808115753087_i64];
_5.fld5.fld6 = core::ptr::addr_of!((*_9));
_5.fld5.fld0.fld6.2.1 = _5.fld5.fld0.fld6.0.0 & RET;
_5.fld5.fld0.fld6.2.0 = -5011_i16;
(*_9) = ['\u{daf8f}','\u{21447}','\u{354e4}','\u{75762}'];
_5.fld5.fld0.fld6.0.0 = _8 as i128;
match _5.fld5.fld0.fld6.4 {
12199 => bb3,
_ => bb2
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_5.fld1 = '\u{87c64}';
_5.fld5.fld6 = _9;
_21 = _5.fld5.fld0.fld6.4 as f32;
_5.fld5.fld0.fld6.4 = 49526_u16 | 61072_u16;
_5.fld5.fld0.fld6.0 = (_5.fld5.fld0.fld1.0, (*_9));
SetDiscriminant(_12, 0);
_5.fld5.fld0.fld6.2.1 = _5.fld5.fld0.fld6.0.0;
_20.0 = -_5.fld5.fld0.fld6.2.0;
_5.fld5.fld5 = (1770847274_u32,);
_13 = [_5.fld5.fld5.0,_5.fld5.fld5.0,_5.fld5.fld5.0,_5.fld5.fld5.0,_5.fld5.fld5.0,_5.fld5.fld5.0,_5.fld5.fld5.0];
_5.fld5.fld0.fld1 = (_5.fld5.fld0.fld6.0.0, _5.fld5.fld0.fld6.0.1);
_12 = Adt54::Variant1 { fld0: _10 };
_25 = -_18;
_20.1 = _5.fld5.fld0.fld6.2.1;
_5.fld2 = _5.fld5.fld4;
_5.fld5.fld0.fld5 = (_5.fld5.fld0.fld4.fld0.2,);
SetDiscriminant(_12, 0);
_26.fld0.1 = -_5.fld5.fld0.fld4.fld0.1;
_5.fld5.fld2 = [_5.fld5.fld0.fld0,_1,_1,_4,_5.fld5.fld0.fld0,_1,_1];
_5.fld5.fld5.0 = 1389501521_u32 & 3776324621_u32;
_9 = _5.fld5.fld6;
match _18 {
0 => bb4,
94 => bb11,
_ => bb3
}
}
bb11 = {
_4 = !_3;
_27 = _20.1 as u128;
_20.0 = -_5.fld5.fld0.fld6.2.0;
RET = _5.fld5.fld0.fld6.0.0 << _18;
_5.fld5.fld0.fld5.0 = _5.fld5.fld0.fld4.fld0.2;
_15 = _8 as f64;
_5.fld5.fld0.fld6.2.0 = _20.0 * _20.0;
_20.1 = _5.fld5.fld0.fld1.0 & _5.fld5.fld0.fld1.0;
_26.fld0.0 = -_5.fld5.fld0.fld4.fld0.0;
_22 = _5.fld5.fld0.fld4.fld0.4;
_5.fld5.fld5.0 = 2226567888_u32;
_27 = _2 as u128;
_8 = _16;
Goto(bb12)
}
bb12 = {
_5.fld5.fld0.fld4.fld0 = (_26.fld0.0, _26.fld0.1, _5.fld5.fld0.fld5.0, _5.fld5.fld4, _18, _5.fld5.fld3);
_7 = [_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1];
_5.fld5.fld0.fld6.0 = (_20.1, (*_9));
_26.fld0.5 = _5.fld5.fld0.fld4.fld0.5;
_5.fld5.fld0.fld4.fld0.0 = 1134052210_i32 as i64;
Goto(bb13)
}
bb13 = {
_5.fld5.fld0.fld4.fld0.3 = core::ptr::addr_of!(_5.fld5.fld0.fld4.fld0.4);
_18 = _5.fld5.fld0.fld4.fld0.4 ^ _5.fld5.fld0.fld4.fld0.4;
_20.2 = -_21;
_5.fld0 = _5.fld5.fld2;
Goto(bb14)
}
bb14 = {
_5.fld5.fld0.fld6.2 = _20;
_5.fld5.fld6 = _9;
_23 = [_5.fld5.fld5.0,_5.fld5.fld5.0];
_20.0 = _27 as i16;
_5.fld5.fld0.fld1 = (_5.fld5.fld0.fld6.2.1, _5.fld5.fld0.fld6.0.1);
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(10_usize, 18_usize, Move(_18), 11_usize, Move(_11), 25_usize, Move(_25), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(10_usize, 7_usize, Move(_7), 3_usize, Move(_3), 13_usize, Move(_13), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [bool; 7],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> [char; 4] {
mir! {
type RET = [char; 4];
let _6: isize;
let _7: [i64; 8];
let _8: f64;
let _9: u32;
let _10: Adt58;
let _11: [i64; 8];
let _12: (i16, i128, f32);
let _13: isize;
let _14: u8;
let _15: char;
let _16: ([char; 4], bool, i16);
let _17: f32;
let _18: i32;
let _19: Adt45;
let _20: f64;
let _21: Adt45;
let _22: i128;
let _23: f64;
let _24: i8;
let _25: [u32; 2];
let _26: (i128, [char; 4]);
let _27: usize;
let _28: (&'static i64, i16, bool, i8);
let _29: Adt54;
let _30: i16;
let _31: (i16, i128, f32);
let _32: f64;
let _33: [u32; 2];
let _34: ();
let _35: ();
{
RET = ['\u{73c17}','\u{3e0f1}','\u{8211e}','\u{c082f}'];
_5 = !_2;
RET = ['\u{ac6e2}','\u{c38d8}','\u{4c3e5}','\u{f9746}'];
_5 = _2;
_5 = _4;
_8 = 8823693288055897933_i64 as f64;
_4 = _2;
_4 = !_5;
_7 = [9101963676077879523_i64,1192402420250181869_i64,5779137083341893462_i64,946848150799642356_i64,(-7013730730105509335_i64),(-4923917376109207469_i64),(-1319062844911673976_i64),3244949482989150749_i64];
_9 = 2549314210_u32 & 2958801313_u32;
Goto(bb1)
}
bb1 = {
_9 = 2185702425_u32 >> 7_usize;
_7 = [(-6556281253935402506_i64),3907801640605820013_i64,8861443554733414514_i64,(-6795734207239844585_i64),527479103256578140_i64,9011286066647653779_i64,7813760983771294090_i64,2810736921537413258_i64];
RET = ['\u{deea1}','\u{d2cc7}','\u{c71fb}','\u{a13d4}'];
_3 = !_5;
_6 = 9223372036854775807_isize;
_5 = !_4;
_5 = _4 < _3;
_5 = _2 | _4;
_7 = [(-7243436140580569138_i64),4523268861997348382_i64,7540426907809024901_i64,8404064883444185535_i64,6691903883368370475_i64,(-8640272068040460241_i64),(-907295656921703964_i64),(-4202037488627324987_i64)];
_6 = !23_isize;
_4 = _5 < _2;
Goto(bb2)
}
bb2 = {
_8 = _9 as f64;
_4 = _3;
_7 = [(-4599673999987832290_i64),2926158059761746919_i64,4929879586516413789_i64,(-8466985590647869308_i64),7153577677084049906_i64,2532183972596799177_i64,5020105324082995349_i64,(-9128613277639306557_i64)];
_4 = _3;
_2 = _4 ^ _4;
_12.0 = 26685_i16;
_8 = 96767633649821990452143436626813257900_i128 as f64;
_2 = !_4;
_12.2 = 7513_u16 as f32;
RET = ['\u{60bd4}','\u{1080a}','\u{d698b}','\u{f8b8c}'];
_11 = _7;
_7 = _11;
RET = ['\u{7579}','\u{eb9fa}','\u{2d5b4}','\u{43ac4}'];
_2 = _4 == _4;
_11 = _7;
_1 = [_5,_4,_5,_5,_5,_5,_4];
_8 = 56868_u16 as f64;
_11 = [(-3752102856127054251_i64),2021989925882878179_i64,8254655108712871448_i64,4798645733214141616_i64,(-6891886063058963379_i64),8792353011668790108_i64,(-4427386780659007458_i64),2890225498639929624_i64];
_4 = !_2;
_15 = '\u{64377}';
RET = [_15,_15,_15,_15];
_13 = -_6;
_3 = _4 == _5;
_3 = _2 | _5;
RET = [_15,_15,_15,_15];
Call(_9 = core::intrinsics::transmute(_15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.0 = _12.2 as i16;
_12.1 = -81354927033380276727246226008417810672_i128;
RET = [_15,_15,_15,_15];
_2 = _3;
_12.2 = 1673360527_i32 as f32;
_14 = 91_u8;
_1 = [_3,_5,_4,_3,_3,_4,_5];
_12.1 = 7_i8 as i128;
_16.0 = [_15,_15,_15,_15];
Goto(bb4)
}
bb4 = {
_5 = _4 != _3;
_8 = 11431180721182748334_u64 as f64;
_12.0 = (-9699_i16);
_6 = !_13;
_16 = (RET, _4, _12.0);
_4 = !_16.1;
_9 = 4267042349_u32;
_6 = -_13;
_9 = 2580533974_u32;
_17 = _12.1 as f32;
RET = [_15,_15,_15,_15];
_12 = (_16.2, (-106492143109688274316108396187840168934_i128), _17);
_12.2 = _17 + _17;
_16.0 = RET;
_12.0 = _16.2;
_11 = _7;
_16 = (RET, _2, _12.0);
match _12.1 {
0 => bb1,
233790223811250189147266211243928042522 => bb6,
_ => bb5
}
}
bb5 = {
_8 = _9 as f64;
_4 = _3;
_7 = [(-4599673999987832290_i64),2926158059761746919_i64,4929879586516413789_i64,(-8466985590647869308_i64),7153577677084049906_i64,2532183972596799177_i64,5020105324082995349_i64,(-9128613277639306557_i64)];
_4 = _3;
_2 = _4 ^ _4;
_12.0 = 26685_i16;
_8 = 96767633649821990452143436626813257900_i128 as f64;
_2 = !_4;
_12.2 = 7513_u16 as f32;
RET = ['\u{60bd4}','\u{1080a}','\u{d698b}','\u{f8b8c}'];
_11 = _7;
_7 = _11;
RET = ['\u{7579}','\u{eb9fa}','\u{2d5b4}','\u{43ac4}'];
_2 = _4 == _4;
_11 = _7;
_1 = [_5,_4,_5,_5,_5,_5,_4];
_8 = 56868_u16 as f64;
_11 = [(-3752102856127054251_i64),2021989925882878179_i64,8254655108712871448_i64,4798645733214141616_i64,(-6891886063058963379_i64),8792353011668790108_i64,(-4427386780659007458_i64),2890225498639929624_i64];
_4 = !_2;
_15 = '\u{64377}';
RET = [_15,_15,_15,_15];
_13 = -_6;
_3 = _4 == _5;
_3 = _2 | _5;
RET = [_15,_15,_15,_15];
Call(_9 = core::intrinsics::transmute(_15), ReturnTo(bb3), UnwindUnreachable())
}
bb6 = {
_13 = _12.1 as isize;
_6 = _13;
_1 = [_5,_16.1,_3,_16.1,_2,_2,_2];
_6 = _15 as isize;
_3 = !_4;
_5 = !_16.1;
_17 = -_12.2;
_2 = !_4;
_18 = !1264632392_i32;
Call(_4 = fn12(_16, _3, _3, _3, _16.1, _16, _16.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = [_3,_4,_4,_16.1,_5,_16.1,_2];
_12.0 = !_16.2;
_12.0 = 237031319796538347896608377756158116197_u128 as i16;
_12.1 = (-131924172734615781527592006619946964491_i128);
_6 = -_13;
_14 = 148_u8 * 71_u8;
RET = [_15,_15,_15,_15];
_2 = _5 >= _4;
_12.1 = 32062669511764827995571548585689973296_i128 & (-163618310865493595631239471049613010143_i128);
RET = [_15,_15,_15,_15];
_16.2 = _12.0 >> _18;
_3 = _2;
_1 = [_3,_2,_2,_16.1,_3,_3,_2];
_20 = _8 + _8;
_16.1 = _4;
_15 = '\u{e71e2}';
_9 = 3242730808_u32;
_25 = [_9,_9];
_11 = [(-8364058740410213030_i64),7380479786310570270_i64,547194650772818709_i64,3026393964199662702_i64,2641351315718265728_i64,(-751612086212265775_i64),1527805967525343253_i64,(-7207184417388441123_i64)];
match _9 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb6,
4 => bb8,
3242730808 => bb10,
_ => bb9
}
}
bb8 = {
_5 = _4 != _3;
_8 = 11431180721182748334_u64 as f64;
_12.0 = (-9699_i16);
_6 = !_13;
_16 = (RET, _4, _12.0);
_4 = !_16.1;
_9 = 4267042349_u32;
_6 = -_13;
_9 = 2580533974_u32;
_17 = _12.1 as f32;
RET = [_15,_15,_15,_15];
_12 = (_16.2, (-106492143109688274316108396187840168934_i128), _17);
_12.2 = _17 + _17;
_16.0 = RET;
_12.0 = _16.2;
_11 = _7;
_16 = (RET, _2, _12.0);
match _12.1 {
0 => bb1,
233790223811250189147266211243928042522 => bb6,
_ => bb5
}
}
bb9 = {
_12.0 = _12.2 as i16;
_12.1 = -81354927033380276727246226008417810672_i128;
RET = [_15,_15,_15,_15];
_2 = _3;
_12.2 = 1673360527_i32 as f32;
_14 = 91_u8;
_1 = [_3,_5,_4,_3,_3,_4,_5];
_12.1 = 7_i8 as i128;
_16.0 = [_15,_15,_15,_15];
Goto(bb4)
}
bb10 = {
_2 = !_3;
_16.1 = _4;
RET = [_15,_15,_15,_15];
_12.0 = _16.2 + _16.2;
_17 = -_12.2;
_26.1 = RET;
_11 = [(-7396328554137935963_i64),(-664321080348481296_i64),7291893831066663584_i64,(-8372736595067785769_i64),8057678054003613123_i64,3959684521185869624_i64,(-3594580191229115276_i64),45452183664584854_i64];
_16.0 = [_15,_15,_15,_15];
_3 = _4;
_23 = _20 * _20;
_16 = (_26.1, _4, _12.0);
_6 = !_13;
Goto(bb11)
}
bb11 = {
_14 = _23 as u8;
_12.1 = -(-26456100982908113156880443109558129571_i128);
_26.0 = -_12.1;
_20 = 48_i8 as f64;
_22 = _26.0;
_4 = _2 <= _3;
_28.3 = _18 as i8;
_26.0 = -_22;
_14 = 12_u8;
_16.2 = _12.0;
_31.0 = _12.0 >> _16.2;
_26.0 = !_22;
_29 = Adt54::Variant1 { fld0: 253600131657362419820676631023970698255_u128 };
_24 = _28.3 + _28.3;
_30 = _2 as i16;
_28.1 = _30;
Call(_27 = fn13(_16.2, _26.1, _1, _5, _2, _4, _5, _5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_16.0 = [_15,_15,_15,_15];
_13 = _9 as isize;
match _9 {
0 => bb10,
1 => bb13,
3242730808 => bb15,
_ => bb14
}
}
bb13 = {
_5 = _4 != _3;
_8 = 11431180721182748334_u64 as f64;
_12.0 = (-9699_i16);
_6 = !_13;
_16 = (RET, _4, _12.0);
_4 = !_16.1;
_9 = 4267042349_u32;
_6 = -_13;
_9 = 2580533974_u32;
_17 = _12.1 as f32;
RET = [_15,_15,_15,_15];
_12 = (_16.2, (-106492143109688274316108396187840168934_i128), _17);
_12.2 = _17 + _17;
_16.0 = RET;
_12.0 = _16.2;
_11 = _7;
_16 = (RET, _2, _12.0);
match _12.1 {
0 => bb1,
233790223811250189147266211243928042522 => bb6,
_ => bb5
}
}
bb14 = {
_13 = _12.1 as isize;
_6 = _13;
_1 = [_5,_16.1,_3,_16.1,_2,_2,_2];
_6 = _15 as isize;
_3 = !_4;
_5 = !_16.1;
_17 = -_12.2;
_2 = !_4;
_18 = !1264632392_i32;
Call(_4 = fn12(_16, _3, _3, _3, _16.1, _16, _16.1), ReturnTo(bb7), UnwindUnreachable())
}
bb15 = {
place!(Field::<u128>(Variant(_29, 1), 0)) = !77969929663911904065210674482289321644_u128;
_16 = (_26.1, _5, _28.1);
_24 = _28.3;
_20 = _23 + _23;
_26.1 = [_15,_15,_15,_15];
_32 = -_23;
_4 = !_2;
_20 = -_32;
place!(Field::<u128>(Variant(_29, 1), 0)) = 170282212704047364128788730260888656645_u128;
_12.0 = _14 as i16;
_17 = _23 as f32;
_31.2 = _17;
_13 = _6 | _6;
SetDiscriminant(_29, 1);
_12.0 = _30 ^ _16.2;
place!(Field::<u128>(Variant(_29, 1), 0)) = _9 as u128;
_31.1 = _12.0 as i128;
_23 = _32;
_14 = 204_u8;
RET = [_15,_15,_15,_15];
RET = _16.0;
_6 = _13;
_26.1 = [_15,_15,_15,_15];
_28.2 = _3;
SetDiscriminant(_29, 0);
RET = [_15,_15,_15,_15];
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(11_usize, 7_usize, Move(_7), 25_usize, Move(_25), 27_usize, Move(_27), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(11_usize, 2_usize, Move(_2), 5_usize, Move(_5), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(11_usize, 9_usize, Move(_9), 15_usize, Move(_15), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([char; 4], bool, i16),mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: ([char; 4], bool, i16),mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: [u32; 1];
let _9: u32;
let _10: ((&'static i64, i16, bool, i8), i16, (usize,));
let _11: char;
let _12: isize;
let _13: ();
let _14: ();
{
RET = _6.1;
_6.0 = _1.0;
_1.2 = !_6.2;
_6.0 = _1.0;
_3 = _6.1 != _4;
_2 = _4 != _7;
_10.2 = (3_usize,);
_10.0.2 = _4 >= _3;
_10.0.2 = !_7;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(12_usize, 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i16,mut _2: [char; 4],mut _3: [bool; 7],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> usize {
mir! {
type RET = usize;
let _9: isize;
let _10: usize;
let _11: Adt54;
let _12: i8;
let _13: (i128, [char; 4]);
let _14: isize;
let _15: i16;
let _16: u128;
let _17: *const [u32; 1];
let _18: bool;
let _19: i32;
let _20: Adt45;
let _21: [i64; 8];
let _22: i16;
let _23: bool;
let _24: *const f64;
let _25: i128;
let _26: isize;
let _27: Adt46;
let _28: bool;
let _29: i128;
let _30: ();
let _31: ();
{
_4 = !_5;
RET = 4_usize;
_4 = _6;
_8 = _3[RET];
Goto(bb1)
}
bb1 = {
_3 = [_8,_8,_7,_5,_4,_7,_4];
_3[RET] = _7 | _7;
_3[RET] = !_8;
_7 = _6 ^ _4;
_9 = 9223372036854775807_isize - (-9223372036854775808_isize);
_5 = _3[RET] >= _3[RET];
_3[RET] = _6;
_6 = _7 <= _8;
RET = 0_usize & 17008585308870432205_usize;
RET = 23837833579782563_usize;
RET = 3589975270427346767_usize;
_10 = (-35_i8) as usize;
_14 = _9 >> _9;
_13.1 = _2;
_2 = ['\u{a4cc0}','\u{1318}','\u{2a91b}','\u{dfd56}'];
_15 = -_1;
_13 = ((-149994341370799793602521390827130504104_i128), _2);
_5 = _14 == _9;
_3 = [_7,_6,_7,_7,_6,_8,_8];
_12 = (-106_i8);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
3589975270427346767 => bb9,
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
_11 = Adt54::Variant1 { fld0: 168638413452966137707861954459888462489_u128 };
_14 = _9;
_1 = _15;
_13.0 = 6782881973720227300_i64 as i128;
_11 = Adt54::Variant1 { fld0: 85201537222790243807848590144906126679_u128 };
_3 = [_6,_8,_4,_6,_7,_7,_6];
_4 = !_7;
_6 = _8;
_14 = _9;
_11 = Adt54::Variant1 { fld0: 109092220403570731143804525751078942576_u128 };
_10 = !RET;
place!(Field::<u128>(Variant(_11, 1), 0)) = 99655071946666925083799398734290887487_u128 + 301728436302816877850881061624591105769_u128;
_6 = _8;
SetDiscriminant(_11, 0);
_13.1 = _2;
RET = 149234161409141961583064487342354008957_u128 as usize;
_18 = !_4;
match _12 {
0 => bb1,
1 => bb6,
2 => bb3,
340282366920938463463374607431768211350 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_11 = Adt54::Variant1 { fld0: 69827891063010901439012581165659807276_u128 };
match _12 {
340282366920938463463374607431768211350 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_19 = -876650925_i32;
RET = 8060616331098613302_u64 as usize;
place!(Field::<u128>(Variant(_11, 1), 0)) = 9099361321316052281_i64 as u128;
place!(Field::<u128>(Variant(_11, 1), 0)) = !57387963617140817795182113927261479205_u128;
RET = _10;
_21 = [(-155347663358508070_i64),(-1213955666538984185_i64),(-361664248958748414_i64),8996905295105887682_i64,(-9191862093192950799_i64),(-275739295231817828_i64),8427969297898627255_i64,755069882040452112_i64];
_23 = _18 & _7;
Call(_14 = fn14(_18, _7, _8, _23, _3, _23, _3, _6, _3, _6, _4, _18), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_9 = !_14;
_5 = _23;
_13.1 = ['\u{ad62e}','\u{b844f}','\u{109296}','\u{33fd3}'];
_3 = [_23,_8,_7,_5,_4,_18,_6];
_9 = !_14;
_13.0 = 128057542713057091790562355393394854473_i128 - (-122677222882964295447029846300178266877_i128);
_25 = _13.0;
place!(Field::<u128>(Variant(_11, 1), 0)) = _25 as u128;
_5 = _4;
RET = !_10;
_25 = _8 as i128;
_10 = !RET;
_15 = -_1;
RET = !_10;
_14 = -_9;
_25 = _13.0 - _13.0;
_22 = !_1;
_25 = _13.0;
_27.fld0.5 = core::ptr::addr_of!(_27.fld0.1);
_11 = Adt54::Variant1 { fld0: 235722068839385741380220322282567007855_u128 };
_27.fld0.3 = core::ptr::addr_of!(_27.fld0.4);
_27.fld0.5 = core::ptr::addr_of!(_27.fld0.1);
_11 = Adt54::Variant0 { fld0: _10 };
_3 = [_8,_7,_18,_8,_8,_8,_23];
RET = !_10;
_2 = _13.1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(13_usize, 25_usize, Move(_25), 22_usize, Move(_22), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(13_usize, 23_usize, Move(_23), 3_usize, Move(_3), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(13_usize, 10_usize, Move(_10), 6_usize, Move(_6), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: [bool; 7],mut _6: bool,mut _7: [bool; 7],mut _8: bool,mut _9: [bool; 7],mut _10: bool,mut _11: bool,mut _12: bool) -> isize {
mir! {
type RET = isize;
let _13: f32;
let _14: isize;
let _15: (i128, [char; 4]);
let _16: *mut u32;
let _17: *const u8;
let _18: isize;
let _19: ();
let _20: ();
{
_2 = _1 & _4;
_7 = [_6,_6,_11,_4,_1,_8,_12];
_10 = _1;
_12 = !_11;
_6 = _4;
_8 = _4 >= _10;
_7 = [_2,_12,_8,_8,_4,_8,_11];
RET = 9223372036854775807_isize - 2_isize;
_3 = _8;
RET = (-115_isize);
_12 = !_4;
_6 = _10 ^ _4;
_5 = [_10,_6,_2,_4,_10,_12,_10];
_10 = !_2;
_5 = _9;
RET = -9223372036854775807_isize;
_2 = _11 | _3;
_3 = _11 < _4;
_12 = _6 > _10;
_14 = _6 as isize;
_8 = !_6;
_1 = _11 != _11;
_15.0 = 14463672860413086645536953273329910818_i128 << _14;
_10 = _6;
_9 = [_6,_11,_6,_4,_10,_6,_12];
Goto(bb1)
}
bb1 = {
_6 = _1;
_11 = _1;
_1 = _2 > _4;
Goto(bb2)
}
bb2 = {
RET = !_14;
_12 = _3;
_4 = !_10;
_14 = -RET;
_14 = '\u{6f42b}' as isize;
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(14_usize, 14_usize, Move(_14), 3_usize, Move(_3), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(14_usize, 10_usize, Move(_10), 8_usize, Move(_8), 20_usize, _20, 20_usize, _20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i128,mut _2: bool,mut _3: (i16, i128, f32),mut _4: i128,mut _5: (i16, i128, f32),mut _6: *const f64,mut _7: bool,mut _8: bool) -> u32 {
mir! {
type RET = u32;
let _9: u128;
let _10: [bool; 7];
let _11: (i16, i128, f32);
let _12: i8;
let _13: *const [u32; 1];
let _14: [u32; 7];
let _15: (i128, [char; 4]);
let _16: (i128, [char; 4]);
let _17: i128;
let _18: f32;
let _19: *mut isize;
let _20: [char; 6];
let _21: char;
let _22: u128;
let _23: u32;
let _24: (u32,);
let _25: [char; 4];
let _26: Adt44;
let _27: Adt56;
let _28: (u32,);
let _29: [u32; 7];
let _30: [char; 6];
let _31: Adt46;
let _32: i32;
let _33: i128;
let _34: isize;
let _35: f32;
let _36: i16;
let _37: f32;
let _38: bool;
let _39: u8;
let _40: ();
let _41: ();
{
RET = 211_u8 as u32;
_3.2 = _5.2;
(*_6) = (-115_i8) as f64;
_5.2 = -_3.2;
Goto(bb1)
}
bb1 = {
_5.0 = _3.0;
Call(_6 = fn16(_5, _2, _4, _4, _5, _3.1, _4, _4, _7, _5.1, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.0 = -_3.0;
_4 = -_5.1;
_7 = _2;
_5 = (_3.0, _3.1, _3.2);
_3.1 = 78_isize as i128;
_1 = _3.2 as i128;
_9 = !243638215935292192067046714599593505778_u128;
_5.0 = _3.0 & _3.0;
_11.1 = 16030223373526328761_usize as i128;
_5.2 = -_3.2;
_3.0 = (-1974553449_i32) as i16;
_3 = _5;
RET = !856635277_u32;
_3.2 = _5.2;
Goto(bb3)
}
bb3 = {
_11 = (_3.0, _5.1, _3.2);
RET = 1528280271_u32 | 770308442_u32;
_11.2 = -_5.2;
_5.1 = _3.1 >> _3.1;
_8 = _5.1 > _3.1;
_11 = (_5.0, _3.1, _5.2);
_14 = [RET,RET,RET,RET,RET,RET,RET];
_10 = [_7,_2,_8,_8,_2,_7,_7];
_1 = -_11.1;
_5 = _11;
RET = 2310474572_u32 & 2537729349_u32;
_4 = _5.1 ^ _1;
_11.0 = 88_isize as i16;
_15.0 = _4;
_3.1 = !_11.1;
_15.1 = ['\u{f45c8}','\u{827f6}','\u{72797}','\u{9b51}'];
_14 = [RET,RET,RET,RET,RET,RET,RET];
_11.0 = 8831094929221567287_i64 as i16;
_16.0 = _3.1;
Call(_11.2 = fn17(_1, _16.0, _5.1, _10, _16.0, _3, _1, _3, _3, _16.0, _15, _4, _4, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = _15;
_5.0 = !_3.0;
_15.0 = _4 ^ _5.1;
_14 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _5;
_3.2 = -_11.2;
_5.1 = -_1;
_11 = _3;
_16.1 = ['\u{ecdd1}','\u{e5c07}','\u{bbc90}','\u{9391f}'];
_1 = _15.0;
RET = 2153607346_u32;
_11.0 = !_3.0;
_3.1 = _11.1;
_5.0 = _3.0;
_18 = -_11.2;
match RET {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
2153607346 => bb9,
_ => bb8
}
}
bb5 = {
_11 = (_3.0, _5.1, _3.2);
RET = 1528280271_u32 | 770308442_u32;
_11.2 = -_5.2;
_5.1 = _3.1 >> _3.1;
_8 = _5.1 > _3.1;
_11 = (_5.0, _3.1, _5.2);
_14 = [RET,RET,RET,RET,RET,RET,RET];
_10 = [_7,_2,_8,_8,_2,_7,_7];
_1 = -_11.1;
_5 = _11;
RET = 2310474572_u32 & 2537729349_u32;
_4 = _5.1 ^ _1;
_11.0 = 88_isize as i16;
_15.0 = _4;
_3.1 = !_11.1;
_15.1 = ['\u{f45c8}','\u{827f6}','\u{72797}','\u{9b51}'];
_14 = [RET,RET,RET,RET,RET,RET,RET];
_11.0 = 8831094929221567287_i64 as i16;
_16.0 = _3.1;
Call(_11.2 = fn17(_1, _16.0, _5.1, _10, _16.0, _3, _1, _3, _3, _16.0, _15, _4, _4, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_5.0 = -_3.0;
_4 = -_5.1;
_7 = _2;
_5 = (_3.0, _3.1, _3.2);
_3.1 = 78_isize as i128;
_1 = _3.2 as i128;
_9 = !243638215935292192067046714599593505778_u128;
_5.0 = _3.0 & _3.0;
_11.1 = 16030223373526328761_usize as i128;
_5.2 = -_3.2;
_3.0 = (-1974553449_i32) as i16;
_3 = _5;
RET = !856635277_u32;
_3.2 = _5.2;
Goto(bb3)
}
bb7 = {
_5.0 = _3.0;
Call(_6 = fn16(_5, _2, _4, _4, _5, _3.1, _4, _4, _7, _5.1, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_17 = _5.1 - _4;
_15.1 = ['\u{10305d}','\u{7cd89}','\u{e111f}','\u{25a8d}'];
_2 = _1 > _11.1;
_11.1 = _5.1 | _17;
_5 = (_11.0, _1, _3.2);
_4 = _15.0 << _1;
_11.0 = !_3.0;
_16.0 = -_4;
_15 = (_5.1, _16.1);
_10 = [_8,_7,_7,_7,_7,_2,_2];
_5.1 = -_11.1;
_15.0 = 8611778511996859053_usize as i128;
_12 = (-69_i8);
_7 = !_8;
_15.0 = -_3.1;
_11.0 = !_5.0;
_12 = 15301668023568267948_u64 as i8;
_5.0 = _3.0 ^ _11.0;
_16 = (_17, _15.1);
_5.2 = _18;
_11.1 = _3.1;
RET = _12 as u32;
RET = 3899941815_u32;
_12 = !116_i8;
_3.1 = -_17;
_7 = !_2;
match RET {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb10,
3899941815 => bb12,
_ => bb11
}
}
bb10 = {
_11 = (_3.0, _5.1, _3.2);
RET = 1528280271_u32 | 770308442_u32;
_11.2 = -_5.2;
_5.1 = _3.1 >> _3.1;
_8 = _5.1 > _3.1;
_11 = (_5.0, _3.1, _5.2);
_14 = [RET,RET,RET,RET,RET,RET,RET];
_10 = [_7,_2,_8,_8,_2,_7,_7];
_1 = -_11.1;
_5 = _11;
RET = 2310474572_u32 & 2537729349_u32;
_4 = _5.1 ^ _1;
_11.0 = 88_isize as i16;
_15.0 = _4;
_3.1 = !_11.1;
_15.1 = ['\u{f45c8}','\u{827f6}','\u{72797}','\u{9b51}'];
_14 = [RET,RET,RET,RET,RET,RET,RET];
_11.0 = 8831094929221567287_i64 as i16;
_16.0 = _3.1;
Call(_11.2 = fn17(_1, _16.0, _5.1, _10, _16.0, _3, _1, _3, _3, _16.0, _15, _4, _4, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_5.0 = _3.0;
Call(_6 = fn16(_5, _2, _4, _4, _5, _3.1, _4, _4, _7, _5.1, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_20 = ['\u{fd58b}','\u{1ace5}','\u{c57f2}','\u{6df68}','\u{91a2f}','\u{b75c7}'];
_2 = _7 & _8;
_9 = !141329959864951664762398290668155911014_u128;
_3 = (_5.0, _1, _11.2);
_3.0 = _5.0;
_11.0 = _5.2 as i16;
_4 = 189_u8 as i128;
_5 = (_11.0, _1, _11.2);
_3 = (_11.0, _1, _18);
_23 = !RET;
_20 = ['\u{cd791}','\u{27375}','\u{4af0}','\u{5a1d3}','\u{844d1}','\u{81aa7}'];
_3 = _11;
_15.1 = ['\u{8f202}','\u{d66e0}','\u{d34a2}','\u{3a52}'];
_4 = _1;
_9 = 131809420353382455126709159624542454467_u128 + 46457614804024547645290775077509100620_u128;
_8 = _7;
_18 = _11.0 as f32;
_27.fld0.fld5.0 = 6_usize & 0_usize;
Call(_4 = core::intrinsics::transmute(_5.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_14 = [_23,RET,_23,_23,_23,RET,_23];
_27.fld0.fld4.fld0.0 = (-8832082690032917184_i64);
_27.fld0.fld6.1 = core::ptr::addr_of!(_27.fld0.fld5);
_18 = _11.2 + _5.2;
_18 = RET as f32;
_10 = [_8,_8,_2,_2,_2,_7,_8];
_25 = ['\u{e40ee}','\u{f855c}','\u{10372f}','\u{8d84f}'];
_12 = 176_u8 as i8;
_27.fld0.fld5 = (5217911067878941314_usize,);
_27.fld0.fld6.2.0 = !_3.0;
_27.fld3 = core::ptr::addr_of!(_27.fld0.fld4.fld0.1);
_17 = 9388422209384098009_u64 as i128;
_27.fld0.fld6.2.2 = _5.2;
_20 = ['\u{8db9}','\u{e8343}','\u{6c67b}','\u{39c28}','\u{4919a}','\u{faec}'];
_27.fld0.fld4.fld0.1 = (-9223372036854775808_isize) as f64;
_11 = (_27.fld0.fld6.2.0, _1, _5.2);
_27.fld5.0 = RET;
_27.fld0.fld4.fld0.4 = 9223372036854775807_isize;
_27.fld0.fld6.1 = core::ptr::addr_of!(_27.fld0.fld5);
_28.0 = _12 as u32;
_27.fld0.fld6.2.0 = _11.0;
_17 = _1 - _1;
Goto(bb14)
}
bb14 = {
_12 = _27.fld0.fld4.fld0.1 as i8;
_27.fld0.fld6.3 = _9 as i8;
_27.fld4 = core::ptr::addr_of!(_34);
_27.fld0.fld6.1 = core::ptr::addr_of!(_27.fld0.fld5);
_15.1 = ['\u{ae75f}','\u{923a7}','\u{dc921}','\u{810f7}'];
RET = !_23;
_27.fld0.fld6.2 = (_3.0, _16.0, _3.2);
_28 = (_27.fld5.0,);
_19 = core::ptr::addr_of_mut!(_34);
_27.fld0.fld6.2.1 = _4;
_12 = -_27.fld0.fld6.3;
_27.fld0.fld1.0 = _5.1;
_3.2 = _11.2;
_27.fld2 = [_2,_2,_2,_2,_7,_8,_8];
_27.fld0.fld6.0.0 = -_5.1;
_3.0 = _27.fld0.fld6.2.0 + _11.0;
_25 = ['\u{d5299}','\u{1be2a}','\u{9694f}','\u{4224f}'];
_27.fld0.fld6.2.0 = _3.0;
_11.0 = -_27.fld0.fld6.2.0;
_35 = -_27.fld0.fld6.2.2;
_29 = _14;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(15_usize, 7_usize, Move(_7), 12_usize, Move(_12), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(15_usize, 28_usize, Move(_28), 4_usize, Move(_4), 1_usize, Move(_1), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: (i16, i128, f32),mut _2: bool,mut _3: i128,mut _4: i128,mut _5: (i16, i128, f32),mut _6: i128,mut _7: i128,mut _8: i128,mut _9: bool,mut _10: i128,mut _11: i128,mut _12: i128) -> *const f64 {
mir! {
type RET = *const f64;
let _13: (i128, [char; 4]);
let _14: *const f64;
let _15: [u32; 7];
let _16: [u32; 2];
let _17: [bool; 7];
let _18: *const [u32; 1];
let _19: i64;
let _20: isize;
let _21: *const (usize,);
let _22: u16;
let _23: [char; 4];
let _24: i128;
let _25: (i16, i128, f32);
let _26: [bool; 7];
let _27: f64;
let _28: Adt52;
let _29: *const isize;
let _30: &'static i64;
let _31: char;
let _32: i64;
let _33: i32;
let _34: isize;
let _35: i32;
let _36: (u32,);
let _37: ();
let _38: ();
{
_1.2 = (-71_i8) as f32;
_1.2 = _5.0 as f32;
_1.1 = -_10;
_5.2 = _1.2;
_1.0 = _5.0;
_5.1 = _1.2 as i128;
_1.2 = _5.2;
_13.0 = _8;
_5.0 = !_1.0;
_11 = 4145136322705655060_u64 as i128;
_9 = !_2;
_1.0 = _5.0;
_7 = _8;
_1 = (_5.0, _12, _5.2);
_6 = _7;
_17 = [_2,_2,_9,_2,_2,_2,_9];
_1 = (_5.0, _10, _5.2);
Goto(bb1)
}
bb1 = {
_7 = !_8;
_3 = !_12;
_3 = (-56_i8) as i128;
_5 = (_1.0, _6, _1.2);
_3 = _10 * _5.1;
_5 = (_1.0, _6, _1.2);
_13.1 = ['\u{100c14}','\u{882eb}','\u{66a52}','\u{85475}'];
_12 = _6 >> _6;
_9 = _2;
_9 = !_2;
Goto(bb2)
}
bb2 = {
_10 = _3 - _7;
_5.1 = _6 - _12;
_12 = _4 * _6;
_1 = (_5.0, _7, _5.2);
_7 = _5.1 * _5.1;
_9 = !_2;
_15 = [212728694_u32,815006414_u32,1085643416_u32,461868354_u32,3216264927_u32,2009697578_u32,1869028145_u32];
_12 = _13.0 * _6;
_9 = _2;
_7 = !_3;
_9 = !_2;
_9 = _2;
_17 = [_2,_9,_2,_9,_9,_2,_9];
_7 = _10 * _5.1;
_16 = [1167466170_u32,1388528332_u32];
_7 = 113_u8 as i128;
_1.1 = _12;
_5.2 = 17475719109792010164_usize as f32;
_13.0 = 9318_u16 as i128;
_7 = -_10;
_5.2 = _1.2;
_5.2 = -_1.2;
Goto(bb3)
}
bb3 = {
_1.0 = 126_u8 as i16;
_19 = (-5810177585320884910_i64);
_5.1 = _7;
_23 = _13.1;
_1.2 = 3_usize as f32;
_5 = (_1.0, _1.1, _1.2);
_5 = (_1.0, _8, _1.2);
match _19 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463457564429846447326546 => bb9,
_ => bb8
}
}
bb4 = {
_10 = _3 - _7;
_5.1 = _6 - _12;
_12 = _4 * _6;
_1 = (_5.0, _7, _5.2);
_7 = _5.1 * _5.1;
_9 = !_2;
_15 = [212728694_u32,815006414_u32,1085643416_u32,461868354_u32,3216264927_u32,2009697578_u32,1869028145_u32];
_12 = _13.0 * _6;
_9 = _2;
_7 = !_3;
_9 = !_2;
_9 = _2;
_17 = [_2,_9,_2,_9,_9,_2,_9];
_7 = _10 * _5.1;
_16 = [1167466170_u32,1388528332_u32];
_7 = 113_u8 as i128;
_1.1 = _12;
_5.2 = 17475719109792010164_usize as f32;
_13.0 = 9318_u16 as i128;
_7 = -_10;
_5.2 = _1.2;
_5.2 = -_1.2;
Goto(bb3)
}
bb5 = {
_7 = !_8;
_3 = !_12;
_3 = (-56_i8) as i128;
_5 = (_1.0, _6, _1.2);
_3 = _10 * _5.1;
_5 = (_1.0, _6, _1.2);
_13.1 = ['\u{100c14}','\u{882eb}','\u{66a52}','\u{85475}'];
_12 = _6 >> _6;
_9 = _2;
_9 = !_2;
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
_17 = [_2,_2,_2,_2,_2,_9,_9];
_24 = _9 as i128;
_13 = (_6, _23);
_16 = [1647149602_u32,3269750274_u32];
_25.2 = _5.2;
_24 = !_5.1;
_4 = _5.1;
_25.2 = 10118206493472664707_usize as f32;
_12 = _13.0;
_19 = 155947098539399106_i64 - 132527847169926080_i64;
_5.0 = _1.0;
_9 = !_2;
Goto(bb10)
}
bb10 = {
_14 = core::ptr::addr_of!(_27);
_14 = core::ptr::addr_of!(_27);
_27 = 9_u8 as f64;
_6 = _5.0 as i128;
_25 = (_5.0, _12, _1.2);
_14 = core::ptr::addr_of!((*_14));
_1.0 = -_25.0;
RET = core::ptr::addr_of!((*_14));
_28 = Adt52::Variant2 { fld0: _5.2,fld1: '\u{50bdf}',fld2: (-9223372036854775808_isize),fld3: 537_u16 };
_25 = (_5.0, _10, _5.2);
_9 = _2 ^ _2;
_6 = _10;
(*_14) = _5.1 as f64;
_22 = 24131_u16 + 3028_u16;
_17 = [_2,_9,_9,_2,_2,_2,_9];
_27 = _19 as f64;
_2 = _9;
_25.2 = Field::<f32>(Variant(_28, 2), 0);
_19 = 57462026080197961998939093906863463190_u128 as i64;
(*RET) = _1.2 as f64;
_20 = 12_isize;
Goto(bb11)
}
bb11 = {
RET = _14;
_5.1 = -_8;
_28 = Adt52::Variant2 { fld0: _1.2,fld1: '\u{f00bf}',fld2: _20,fld3: _22 };
(*RET) = _22 as f64;
_7 = _27 as i128;
_13.0 = 17077710990094517158_usize as i128;
_33 = -1379558295_i32;
_23 = _13.1;
_32 = _19 ^ _19;
_34 = _20;
match _34 {
0 => bb12,
1 => bb13,
2 => bb14,
12 => bb16,
_ => bb15
}
}
bb12 = {
_7 = !_8;
_3 = !_12;
_3 = (-56_i8) as i128;
_5 = (_1.0, _6, _1.2);
_3 = _10 * _5.1;
_5 = (_1.0, _6, _1.2);
_13.1 = ['\u{100c14}','\u{882eb}','\u{66a52}','\u{85475}'];
_12 = _6 >> _6;
_9 = _2;
_9 = !_2;
Goto(bb2)
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
_25.2 = _5.2;
place!(Field::<u16>(Variant(_28, 2), 3)) = !_22;
_13.1 = ['\u{29d0e}','\u{26a60}','\u{93b65}','\u{370d7}'];
_3 = 4313464518771128538_usize as i128;
place!(Field::<f32>(Variant(_28, 2), 0)) = -_5.2;
_13.0 = !_24;
_1.0 = !_25.0;
_10 = _6;
place!(Field::<char>(Variant(_28, 2), 1)) = '\u{2326f}';
_23 = _13.1;
_20 = _34 ^ Field::<isize>(Variant(_28, 2), 2);
_25.1 = _12;
_13.1 = [Field::<char>(Variant(_28, 2), 1),Field::<char>(Variant(_28, 2), 1),Field::<char>(Variant(_28, 2), 1),Field::<char>(Variant(_28, 2), 1)];
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(16_usize, 4_usize, Move(_4), 9_usize, Move(_9), 19_usize, Move(_19), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(16_usize, 32_usize, Move(_32), 3_usize, Move(_3), 24_usize, Move(_24), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(16_usize, 13_usize, Move(_13), 34_usize, Move(_34), 6_usize, Move(_6), 38_usize, _38), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i128,mut _2: i128,mut _3: i128,mut _4: [bool; 7],mut _5: i128,mut _6: (i16, i128, f32),mut _7: i128,mut _8: (i16, i128, f32),mut _9: (i16, i128, f32),mut _10: i128,mut _11: (i128, [char; 4]),mut _12: i128,mut _13: i128,mut _14: [bool; 7]) -> f32 {
mir! {
type RET = f32;
let _15: i8;
let _16: f32;
let _17: ((&'static i64, i16, bool, i8), i16, (usize,));
let _18: u64;
let _19: Adt51;
let _20: isize;
let _21: isize;
let _22: (usize,);
let _23: ([char; 4], bool, i16);
let _24: f32;
let _25: *const f64;
let _26: u32;
let _27: ();
let _28: ();
{
_8 = (_9.0, _7, _9.2);
_12 = 16407647159223808776_u64 as i128;
_12 = -_5;
_11.1 = ['\u{4db09}','\u{106c60}','\u{d98bf}','\u{e7b9}'];
_6 = (_8.0, _10, _8.2);
_6.2 = _9.2;
_5 = _12;
_16 = _8.2;
_8 = (_9.0, _2, _6.2);
_9.0 = _8.0 - _8.0;
_3 = _8.1 >> _13;
Goto(bb1)
}
bb1 = {
_4 = [true,false,false,true,true,true,false];
_9.1 = _2 + _11.0;
_17.1 = 6_usize as i16;
_9.0 = -_6.0;
_4 = _14;
_12 = _1;
_11.0 = 9223372036854775807_isize as i128;
_9 = (_6.0, _8.1, _8.2);
_17.2.0 = (-535547087_i32) as usize;
_14 = [true,true,true,false,false,true,false];
_8.2 = -_16;
_6.0 = _8.0 ^ _17.1;
_6 = _8;
_10 = _7;
_6.1 = 31522_u16 as i128;
_9.1 = 7636107122443573178_u64 as i128;
_11.0 = _1 >> _8.1;
_10 = !_3;
_6.0 = _17.1;
_17.1 = -_8.0;
_9 = (_8.0, _8.1, _8.2);
_8.0 = _17.1 | _6.0;
_20 = 3554117687_u32 as isize;
Goto(bb2)
}
bb2 = {
_2 = _13 ^ _11.0;
_17.1 = -_8.0;
_2 = _10;
_8.2 = _6.2;
_23.1 = false | false;
_8.1 = !_13;
_1 = _3;
RET = _9.2 - _16;
_17.2.0 = 6988939827851665553_usize;
_9 = (_17.1, _8.1, _8.2);
_9 = _8;
_17.0.3 = -(-87_i8);
RET = _17.0.3 as f32;
_15 = _17.0.3;
_8.1 = _3 & _10;
_18 = 9338923729740240876_u64 - 4201903549816253398_u64;
_8 = (_17.1, _3, _9.2);
_12 = 3923807991_u32 as i128;
Goto(bb3)
}
bb3 = {
_24 = _6.2 + _8.2;
_6.1 = _13 >> _9.1;
_2 = _9.1 | _10;
Goto(bb4)
}
bb4 = {
_7 = -_6.1;
_17.2.0 = 63640_u16 as usize;
_18 = 1911226156_u32 as u64;
_7 = _1 - _9.1;
_2 = _18 as i128;
_22 = (_17.2.0,);
_6.2 = _24 * _9.2;
_17.0.1 = !_8.0;
_6.1 = !_8.1;
_6.1 = !_3;
_23.1 = false;
_12 = 83324431421342204753133332048797364482_u128 as i128;
_3 = _1;
_23 = (_11.1, true, _17.0.1);
RET = _7 as f32;
Goto(bb5)
}
bb5 = {
Call(_27 = dump_var(17_usize, 14_usize, Move(_14), 3_usize, Move(_3), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_27 = dump_var(17_usize, 18_usize, Move(_18), 23_usize, Move(_23), 20_usize, Move(_20), 5_usize, Move(_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: bool) -> *mut u32 {
mir! {
type RET = *mut u32;
let _3: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8);
let _4: i32;
let _5: [u32; 1];
let _6: ([char; 4], bool, i16);
let _7: [char; 4];
let _8: bool;
let _9: [char; 6];
let _10: f32;
let _11: Adt51;
let _12: u16;
let _13: [i64; 8];
let _14: bool;
let _15: *const [char; 4];
let _16: f32;
let _17: char;
let _18: *const [char; 4];
let _19: bool;
let _20: isize;
let _21: Adt56;
let _22: i8;
let _23: ();
let _24: ();
{
_1 = !61_isize;
_2 = _1 >= _1;
_1 = (-2081987550864633475_i64) as isize;
_3.0.0 = !148716953783502884848009978969003027704_i128;
_3.2.0 = (-23318_i16);
_3.2.0 = 21984_i16;
_3.2.2 = 10213300842114591068_usize as f32;
_3.0.1 = ['\u{45b32}','\u{66f84}','\u{bba49}','\u{1d1ea}'];
_3.0.0 = 30156131042539983463005401809154656399_i128;
_2 = true;
_1 = 9223372036854775807_isize;
_3.0.1 = ['\u{369f2}','\u{28d51}','\u{6b681}','\u{6b502}'];
match _3.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
30156131042539983463005401809154656399 => bb6,
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
_5 = [1842572760_u32];
Goto(bb7)
}
bb7 = {
_3.4 = 3135727011247830147_i64 as u16;
_4 = 1904776371_i32 | 1350341484_i32;
_3.0.0 = 111372955333556204207214725442147446567_i128 - 4931000246470282204922484192650182852_i128;
_3.5 = 89_i8;
Goto(bb8)
}
bb8 = {
_3.2.2 = _4 as f32;
_3.2.0 = (-29866_i16);
_3.3 = _2 as i8;
_5 = [1663516781_u32];
_3.2.0 = !(-21721_i16);
_3.2.1 = _3.0.0;
_2 = false;
_3.2.2 = 16440807185076126368_u64 as f32;
_3.4 = 60902_u16 >> _3.0.0;
match _1 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb9,
4 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb9 = {
_3.4 = 3135727011247830147_i64 as u16;
_4 = 1904776371_i32 | 1350341484_i32;
_3.0.0 = 111372955333556204207214725442147446567_i128 - 4931000246470282204922484192650182852_i128;
_3.5 = 89_i8;
Goto(bb8)
}
bb10 = {
_5 = [1842572760_u32];
Goto(bb7)
}
bb11 = {
Return()
}
bb12 = {
_3.4 = !43429_u16;
_3.2.2 = _3.2.0 as f32;
_3.2.1 = _3.2.2 as i128;
_3.4 = !65092_u16;
_5 = [1439313614_u32];
_3.2.2 = 24473810050787282465394481878663414771_u128 as f32;
_6.0 = _3.0.1;
_6 = (_3.0.1, _2, _3.2.0);
Goto(bb13)
}
bb13 = {
_3.4 = 2846_u16 * 4045_u16;
_5 = [101997044_u32];
_3.0 = (_3.2.1, _6.0);
_6.2 = 2614044042715348691_i64 as i16;
_3.5 = _3.3;
_7 = ['\u{3016}','\u{7ae79}','\u{439aa}','\u{5919c}'];
_3.4 = !40027_u16;
_5 = [865494317_u32];
_6.0 = ['\u{6aef1}','\u{ee148}','\u{976fc}','\u{2f988}'];
_6 = (_7, _2, _3.2.0);
_1 = 15_isize * 101_isize;
_6.1 = _2;
_6.0 = ['\u{3a5da}','\u{6f007}','\u{3559e}','\u{d0dc2}'];
_1 = 9223372036854775807_isize;
_3.5 = _3.3 + _3.3;
_3.0 = (_3.2.1, _6.0);
_6.0 = _7;
_2 = _6.1;
_5 = [2117363599_u32];
_3.4 = 15506_u16 ^ 62662_u16;
Goto(bb14)
}
bb14 = {
_5 = [2197071496_u32];
_6 = (_3.0.1, _2, _3.2.0);
_6 = (_7, _2, _3.2.0);
_6.2 = -_3.2.0;
_3.3 = _3.5;
_3.0.1 = ['\u{324fc}','\u{ef65f}','\u{58876}','\u{f0289}'];
_3.5 = _3.3 >> _4;
_3.5 = _3.3 & _3.3;
_4 = 10894624999210039058_usize as i32;
_4 = !718954496_i32;
_3.4 = !40254_u16;
_3.3 = _3.5;
_6.0 = _7;
_3.2.1 = '\u{31cb0}' as i128;
_6 = (_3.0.1, _2, _3.2.0);
_3.2.1 = !_3.0.0;
_5 = [3486673961_u32];
_3.0 = (_3.2.1, _7);
_7 = ['\u{5e104}','\u{cde17}','\u{6f1c}','\u{d658c}'];
_9 = ['\u{bc36e}','\u{c87f8}','\u{42e0b}','\u{71321}','\u{93a83}','\u{b6dde}'];
match _1 {
0 => bb15,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
5 => bb20,
6 => bb21,
9223372036854775807 => bb23,
_ => bb22
}
}
bb15 = {
_3.4 = 2846_u16 * 4045_u16;
_5 = [101997044_u32];
_3.0 = (_3.2.1, _6.0);
_6.2 = 2614044042715348691_i64 as i16;
_3.5 = _3.3;
_7 = ['\u{3016}','\u{7ae79}','\u{439aa}','\u{5919c}'];
_3.4 = !40027_u16;
_5 = [865494317_u32];
_6.0 = ['\u{6aef1}','\u{ee148}','\u{976fc}','\u{2f988}'];
_6 = (_7, _2, _3.2.0);
_1 = 15_isize * 101_isize;
_6.1 = _2;
_6.0 = ['\u{3a5da}','\u{6f007}','\u{3559e}','\u{d0dc2}'];
_1 = 9223372036854775807_isize;
_3.5 = _3.3 + _3.3;
_3.0 = (_3.2.1, _6.0);
_6.0 = _7;
_2 = _6.1;
_5 = [2117363599_u32];
_3.4 = 15506_u16 ^ 62662_u16;
Goto(bb14)
}
bb16 = {
_3.4 = !43429_u16;
_3.2.2 = _3.2.0 as f32;
_3.2.1 = _3.2.2 as i128;
_3.4 = !65092_u16;
_5 = [1439313614_u32];
_3.2.2 = 24473810050787282465394481878663414771_u128 as f32;
_6.0 = _3.0.1;
_6 = (_3.0.1, _2, _3.2.0);
Goto(bb13)
}
bb17 = {
Return()
}
bb18 = {
_5 = [1842572760_u32];
Goto(bb7)
}
bb19 = {
Return()
}
bb20 = {
_3.2.2 = _4 as f32;
_3.2.0 = (-29866_i16);
_3.3 = _2 as i8;
_5 = [1663516781_u32];
_3.2.0 = !(-21721_i16);
_3.2.1 = _3.0.0;
_2 = false;
_3.2.2 = 16440807185076126368_u64 as f32;
_3.4 = 60902_u16 >> _3.0.0;
match _1 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb9,
4 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb21 = {
_3.4 = 3135727011247830147_i64 as u16;
_4 = 1904776371_i32 | 1350341484_i32;
_3.0.0 = 111372955333556204207214725442147446567_i128 - 4931000246470282204922484192650182852_i128;
_3.5 = 89_i8;
Goto(bb8)
}
bb22 = {
Return()
}
bb23 = {
_3.0 = (_3.2.1, _6.0);
_3.4 = 23322_u16;
_5 = [2353820695_u32];
_3.5 = _3.3 << _3.3;
_3.2.0 = _6.2;
_3.3 = _3.5 * _3.5;
_9 = ['\u{6d74f}','\u{1fa03}','\u{52c9c}','\u{b85f7}','\u{e5148}','\u{e5541}'];
_3.4 = 3626146405260252552_usize as u16;
_6.0 = ['\u{103}','\u{cc7f2}','\u{951c7}','\u{1fd7d}'];
_3.2.0 = !_6.2;
_5 = [2767612374_u32];
_2 = _6.1;
_3.2.1 = _3.0.0;
_6.2 = _3.2.0;
_12 = _4 as u16;
_3.4 = _6.2 as u16;
_8 = _6.1;
_6.2 = _3.2.0 ^ _3.2.0;
_3.0.1 = ['\u{92c90}','\u{b95a3}','\u{9b4a1}','\u{43d35}'];
_6 = (_7, _2, _3.2.0);
match _1 {
0 => bb9,
1 => bb2,
2 => bb20,
3 => bb7,
4 => bb5,
5 => bb24,
6 => bb25,
9223372036854775807 => bb27,
_ => bb26
}
}
bb24 = {
_5 = [2197071496_u32];
_6 = (_3.0.1, _2, _3.2.0);
_6 = (_7, _2, _3.2.0);
_6.2 = -_3.2.0;
_3.3 = _3.5;
_3.0.1 = ['\u{324fc}','\u{ef65f}','\u{58876}','\u{f0289}'];
_3.5 = _3.3 >> _4;
_3.5 = _3.3 & _3.3;
_4 = 10894624999210039058_usize as i32;
_4 = !718954496_i32;
_3.4 = !40254_u16;
_3.3 = _3.5;
_6.0 = _7;
_3.2.1 = '\u{31cb0}' as i128;
_6 = (_3.0.1, _2, _3.2.0);
_3.2.1 = !_3.0.0;
_5 = [3486673961_u32];
_3.0 = (_3.2.1, _7);
_7 = ['\u{5e104}','\u{cde17}','\u{6f1c}','\u{d658c}'];
_9 = ['\u{bc36e}','\u{c87f8}','\u{42e0b}','\u{71321}','\u{93a83}','\u{b6dde}'];
match _1 {
0 => bb15,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
5 => bb20,
6 => bb21,
9223372036854775807 => bb23,
_ => bb22
}
}
bb25 = {
_3.4 = 3135727011247830147_i64 as u16;
_4 = 1904776371_i32 | 1350341484_i32;
_3.0.0 = 111372955333556204207214725442147446567_i128 - 4931000246470282204922484192650182852_i128;
_3.5 = 89_i8;
Goto(bb8)
}
bb26 = {
_3.4 = 2846_u16 * 4045_u16;
_5 = [101997044_u32];
_3.0 = (_3.2.1, _6.0);
_6.2 = 2614044042715348691_i64 as i16;
_3.5 = _3.3;
_7 = ['\u{3016}','\u{7ae79}','\u{439aa}','\u{5919c}'];
_3.4 = !40027_u16;
_5 = [865494317_u32];
_6.0 = ['\u{6aef1}','\u{ee148}','\u{976fc}','\u{2f988}'];
_6 = (_7, _2, _3.2.0);
_1 = 15_isize * 101_isize;
_6.1 = _2;
_6.0 = ['\u{3a5da}','\u{6f007}','\u{3559e}','\u{d0dc2}'];
_1 = 9223372036854775807_isize;
_3.5 = _3.3 + _3.3;
_3.0 = (_3.2.1, _6.0);
_6.0 = _7;
_2 = _6.1;
_5 = [2117363599_u32];
_3.4 = 15506_u16 ^ 62662_u16;
Goto(bb14)
}
bb27 = {
_6.1 = _3.3 >= _3.5;
_3.3 = _3.5 + _3.5;
_6 = (_7, _8, _3.2.0);
_2 = !_8;
_8 = !_6.1;
_4 = _3.2.2 as i32;
_3.2.2 = (-7403887873252080758_i64) as f32;
_10 = _3.2.2 * _3.2.2;
_8 = _6.1;
_6.1 = !_2;
_3.0 = (_3.2.1, _6.0);
_2 = _10 != _3.2.2;
_3.4 = '\u{282d2}' as u16;
_10 = _3.0.0 as f32;
_3.3 = _3.5;
_13 = [(-9143810358458447831_i64),(-7407206096300519449_i64),(-6551967438686009889_i64),6692166186452980189_i64,(-4960963121989437718_i64),3896913257738296232_i64,(-4103159876406185626_i64),5489951088705954819_i64];
_8 = !_2;
_14 = !_2;
_15 = core::ptr::addr_of!(_7);
_6.1 = !_2;
_3.0 = (_3.2.1, (*_15));
(*_15) = _6.0;
_1 = '\u{1811f}' as isize;
_15 = core::ptr::addr_of!(_7);
_4 = (-1058019898_i32) >> _3.3;
_9 = ['\u{87156}','\u{7e2c7}','\u{b6939}','\u{12872}','\u{10b14d}','\u{d0792}'];
_3.2 = (_6.2, _3.0.0, _10);
Call(_3.5 = core::intrinsics::transmute(_6.1), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_5 = [3728462589_u32];
_16 = _3.2.2;
_3.0.1 = _7;
_3.4 = _12 & _12;
_3.3 = -_3.5;
_9 = ['\u{a74a7}','\u{9390f}','\u{6a633}','\u{c361f}','\u{1ba8c}','\u{8b55a}'];
_3.0.0 = _3.2.1;
Call(RET = fn19(_6.2, _15, _10, _9, _13, _14, _3.3, _6.0, _4, _9, _13, _6.1), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
_2 = _8 == _8;
_3.0.1 = ['\u{10b5d9}','\u{8285c}','\u{6c567}','\u{41016}'];
_3.0.1 = _6.0;
_17 = '\u{86291}';
_14 = _8;
_12 = _3.4 >> _4;
(*_15) = [_17,_17,_17,_17];
_16 = _10;
_6.2 = _3.0.0 as i16;
_13 = [(-3094651377089095106_i64),(-1137884865694179736_i64),(-5790215853903248027_i64),625953116348836378_i64,(-6646306685933744278_i64),7389561809642869052_i64,8712287594351881304_i64,8242351236916272891_i64];
_7 = _6.0;
_21.fld0.fld6.5 = 4048613798_u32 as i8;
_21.fld0.fld6.1 = core::ptr::addr_of!(_21.fld0.fld5);
_21.fld0.fld2 = Adt45::Variant1 { fld0: _5,fld1: (-8744127035315596342_i64) };
_21.fld4 = core::ptr::addr_of!(_21.fld0.fld4.fld0.4);
_6 = (_7, _2, _3.2.0);
_6 = ((*_15), _8, _3.2.0);
_3.0.1 = [_17,_17,_17,_17];
_21.fld0.fld6.3 = !_3.3;
RET = core::ptr::addr_of_mut!(_21.fld5.0);
Goto(bb30)
}
bb30 = {
Call(_23 = dump_var(18_usize, 12_usize, Move(_12), 8_usize, Move(_8), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_23 = dump_var(18_usize, 1_usize, Move(_1), 14_usize, Move(_14), 24_usize, _24, 24_usize, _24), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i16,mut _2: *const [char; 4],mut _3: f32,mut _4: [char; 6],mut _5: [i64; 8],mut _6: bool,mut _7: i8,mut _8: [char; 4],mut _9: i32,mut _10: [char; 6],mut _11: [i64; 8],mut _12: bool) -> *mut u32 {
mir! {
type RET = *mut u32;
let _13: char;
let _14: [char; 4];
let _15: u16;
let _16: ([char; 4], bool, i16);
let _17: isize;
let _18: *mut (i128, [char; 4]);
let _19: isize;
let _20: [char; 4];
let _21: i128;
let _22: [char; 6];
let _23: usize;
let _24: char;
let _25: (i128, [char; 4]);
let _26: u16;
let _27: ([char; 4], bool, i16);
let _28: f64;
let _29: char;
let _30: i64;
let _31: Adt60;
let _32: [u32; 2];
let _33: i8;
let _34: [u32; 7];
let _35: [u32; 1];
let _36: isize;
let _37: ([char; 4], bool, i16);
let _38: (usize,);
let _39: (i16, i128, f32);
let _40: [i64; 8];
let _41: [u32; 7];
let _42: isize;
let _43: Adt44;
let _44: (u32,);
let _45: ();
let _46: ();
{
_9 = 2019161876_i32 - (-1774875065_i32);
_1 = _3 as i16;
_7 = -(-86_i8);
(*_2) = _8;
_2 = core::ptr::addr_of!((*_2));
_1 = -(-13356_i16);
_1 = 9481_i16;
_7 = !10_i8;
_17 = (-9223372036854775808_isize);
_15 = '\u{a6f78}' as u16;
_16.2 = _1 >> _7;
_16 = ((*_2), _6, _1);
_5 = [(-6061393643172790324_i64),5993577781610735205_i64,1934704530022316572_i64,7470064736049842232_i64,(-3250505800006938295_i64),(-8654665639674296472_i64),5568003085127551993_i64,(-334451809456524382_i64)];
_13 = '\u{d15ae}';
_2 = core::ptr::addr_of!((*_2));
_17 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_20 = [_13,_13,_13,_13];
_3 = (-5548210977116808879_i64) as f32;
_14 = [_13,_13,_13,_13];
_12 = _16.1;
_16.0 = [_13,_13,_13,_13];
_17 = -9223372036854775807_isize;
match _16.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
9481 => bb7,
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
_12 = _9 != _9;
Goto(bb8)
}
bb8 = {
_16.0 = _8;
_16 = (_8, _12, _1);
_12 = _6;
_11 = _5;
_9 = !775898684_i32;
_16.1 = _9 != _9;
Goto(bb9)
}
bb9 = {
_19 = _17 + _17;
_16.1 = _12 & _12;
_6 = _16.1;
_9 = !(-1521036254_i32);
_8 = (*_2);
_9 = (-671182315_i32);
(*_2) = _16.0;
_21 = 16477612277754171443866760796987651162_i128;
_16 = ((*_2), _6, _1);
_1 = _16.2;
_16.1 = !_12;
_8 = [_13,_13,_13,_13];
_8 = (*_2);
match _21 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb8,
6 => bb10,
16477612277754171443866760796987651162 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_12 = _9 != _9;
Goto(bb8)
}
bb12 = {
(*_2) = [_13,_13,_13,_13];
(*_2) = _20;
_5 = [(-913385400432832505_i64),(-8843884591245482522_i64),(-163692408452963464_i64),(-4257300080087286735_i64),(-7605128496730016999_i64),(-4569881413085889189_i64),(-7236485304652687196_i64),3586616489152116902_i64];
_5 = [9076210354806075027_i64,284227389048665249_i64,(-7312397067566074659_i64),1461734043505236868_i64,8389709629015977961_i64,(-1862901816206936036_i64),5894661961382063179_i64,3849092790833281165_i64];
_4 = [_13,_13,_13,_13,_13,_13];
_3 = _15 as f32;
_5 = [1442010403994531706_i64,(-8779796379903611537_i64),8188752986203370006_i64,(-8179275772559417216_i64),(-2106903691655636426_i64),8218124391985820012_i64,1149947367727630695_i64,8783159316348128834_i64];
_11 = [7654230317420239961_i64,5313072547041718517_i64,3474680859852811882_i64,2014351795789931129_i64,(-730257782641334909_i64),7218842917889013759_i64,2813910526163173200_i64,(-2225664617416298970_i64)];
_22 = [_13,_13,_13,_13,_13,_13];
_6 = !_16.1;
_12 = _16.1 ^ _16.1;
_20 = [_13,_13,_13,_13];
_5 = [(-7912798421911031179_i64),2100555985612352809_i64,7624636196457629504_i64,4167796289699269516_i64,5056540898728819273_i64,(-2395315323094037122_i64),(-6640978987919501037_i64),8150182382498920240_i64];
_12 = !_16.1;
_16.1 = !_12;
_13 = '\u{74fb5}';
_16.1 = !_6;
_10 = [_13,_13,_13,_13,_13,_13];
_16.0 = (*_2);
_16 = ((*_2), _6, _1);
_14 = [_13,_13,_13,_13];
_16 = ((*_2), _6, _1);
_6 = !_12;
_12 = !_6;
_9 = !(-1505473444_i32);
Goto(bb13)
}
bb13 = {
_7 = -103_i8;
_16.2 = _1 << _15;
_22 = [_13,_13,_13,_13,_13,_13];
_1 = !_16.2;
_17 = _15 as isize;
(*_2) = _8;
_9 = 386560000_i32 ^ (-428974821_i32);
match _21 {
16477612277754171443866760796987651162 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_15 = 25690_u16 ^ 10430_u16;
_19 = _17 << _16.2;
_16.1 = _13 >= _13;
_7 = !58_i8;
_4 = [_13,_13,_13,_13,_13,_13];
_23 = 4052767280_u32 as usize;
_1 = _16.2 * _16.2;
match _21 {
0 => bb6,
1 => bb13,
2 => bb10,
3 => bb16,
16477612277754171443866760796987651162 => bb18,
_ => bb17
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_20 = [_13,_13,_13,_13];
_16.0 = [_13,_13,_13,_13];
_17 = _19 | _19;
_21 = 122970555900268900291431651602616093994_i128;
_17 = _19;
_27.1 = _3 >= _3;
_13 = '\u{a4214}';
_26 = _15;
_11 = [7175320988803435977_i64,780031682911891554_i64,7439062125656991810_i64,8923239122690224446_i64,672881640567192521_i64,(-8747003645201519123_i64),(-1053367968086579858_i64),808108326766273587_i64];
_17 = _19 >> _16.2;
_7 = (-14_i8) << _17;
(*_2) = [_13,_13,_13,_13];
_18 = core::ptr::addr_of_mut!(_25);
_27 = (_16.0, _12, _1);
_21 = -161789316373936287823778323971190802092_i128;
(*_18).0 = _21;
(*_18).1 = [_13,_13,_13,_13];
_24 = _13;
_25.0 = _21 & _21;
_16 = _27;
_16.0 = [_24,_24,_13,_24];
_25.1 = _8;
_20 = (*_18).1;
Goto(bb19)
}
bb19 = {
Goto(bb20)
}
bb20 = {
_16.1 = !_12;
_15 = (-6527769731181126123_i64) as u16;
_5 = [(-7799967615982220747_i64),(-7219663729488356422_i64),4226424035244266159_i64,(-6563919762746380391_i64),7270207653211088537_i64,(-3376118763450556403_i64),792195793203639905_i64,3520702956553654860_i64];
_1 = !_16.2;
_24 = _13;
(*_18) = (_21, _27.0);
_12 = _6;
_25.0 = _21;
_30 = _7 as i64;
(*_18).1 = [_24,_13,_24,_24];
_25.1 = _14;
Goto(bb21)
}
bb21 = {
_10 = [_13,_13,_24,_24,_24,_24];
_7 = 116_i8;
_27 = (_8, _6, _16.2);
(*_18).0 = _21 * _21;
_25.1 = _27.0;
_25.1 = [_24,_24,_13,_13];
_29 = _13;
_34 = [3053741302_u32,515809499_u32,1667272307_u32,2831986551_u32,3331451188_u32,1727266331_u32,3779065317_u32];
match _7 {
0 => bb20,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb22,
5 => bb23,
116 => bb25,
_ => bb24
}
}
bb22 = {
_7 = -103_i8;
_16.2 = _1 << _15;
_22 = [_13,_13,_13,_13,_13,_13];
_1 = !_16.2;
_17 = _15 as isize;
(*_2) = _8;
_9 = 386560000_i32 ^ (-428974821_i32);
match _21 {
16477612277754171443866760796987651162 => bb15,
_ => bb14
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
(*_18) = (_21, _14);
(*_18).0 = _21;
_16.0 = _8;
_10 = _4;
_16.1 = !_12;
_14 = [_29,_24,_29,_13];
_34 = [3725638956_u32,1139073248_u32,2095286620_u32,3646408382_u32,2827688098_u32,3705350121_u32,1570855991_u32];
_19 = !_17;
(*_18) = (_21, _20);
_16 = _27;
_14 = [_24,_29,_29,_24];
_34 = [3479842330_u32,393766434_u32,1501981965_u32,2310739903_u32,3190361430_u32,2974534672_u32,1376807927_u32];
_29 = _13;
_9 = -1411250447_i32;
_26 = _15;
_4 = _10;
(*_18).0 = _1 as i128;
_8 = [_24,_13,_13,_13];
_21 = _13 as i128;
_27.2 = _16.2 - _1;
_21 = _25.0 | (*_18).0;
_27.0 = (*_18).1;
_10 = _4;
_25.1 = [_24,_13,_13,_29];
_7 = (-121_i8) | (-5_i8);
_12 = _6;
_5 = [_30,_30,_30,_30,_30,_30,_30,_30];
Goto(bb26)
}
bb26 = {
_8 = _20;
_32 = [2573840906_u32,2935398514_u32];
_37.2 = _30 as i16;
_11 = _5;
_15 = _26 - _26;
_26 = !_15;
_16.2 = _37.2;
_41 = [30294664_u32,400234354_u32,144383327_u32,794238790_u32,1894036351_u32,268219548_u32,3711737955_u32];
_25 = (_21, _8);
_15 = _26 + _26;
_16.2 = _30 as i16;
_23 = 113458038495146900306578681518689145264_u128 as usize;
_39.0 = !_16.2;
Goto(bb27)
}
bb27 = {
_40 = [_30,_30,_30,_30,_30,_30,_30,_30];
_36 = _19;
_36 = _19;
_37.1 = _27.1;
_17 = !_19;
_1 = !_27.2;
_1 = _16.2 | _39.0;
_14 = [_24,_24,_24,_13];
_27 = _16;
_16.1 = _6;
_34 = _41;
_37.0 = [_13,_29,_13,_24];
_38.0 = _15 as usize;
_26 = _15;
_30 = 8602985939032232709_i64;
_3 = 752085241733498295_u64 as f32;
_39.2 = _15 as f32;
_44 = (1514249489_u32,);
RET = core::ptr::addr_of_mut!(_44.0);
_13 = _24;
RET = core::ptr::addr_of_mut!((*RET));
_40 = [_30,_30,_30,_30,_30,_30,_30,_30];
(*_18).0 = -_21;
_34 = [(*RET),_44.0,(*RET),(*RET),_44.0,(*RET),_44.0];
_44.0 = 193_u8 as u32;
_17 = _6 as isize;
_4 = [_29,_29,_29,_24,_24,_13];
Goto(bb28)
}
bb28 = {
Call(_45 = dump_var(19_usize, 5_usize, Move(_5), 41_usize, Move(_41), 22_usize, Move(_22), 16_usize, Move(_16)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_45 = dump_var(19_usize, 23_usize, Move(_23), 27_usize, Move(_27), 15_usize, Move(_15), 44_usize, Move(_44)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_45 = dump_var(19_usize, 12_usize, Move(_12), 40_usize, Move(_40), 37_usize, Move(_37), 29_usize, Move(_29)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_45 = dump_var(19_usize, 32_usize, Move(_32), 14_usize, Move(_14), 17_usize, Move(_17), 1_usize, Move(_1)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_45 = dump_var(19_usize, 26_usize, Move(_26), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(14643779630727943882_u64), std::hint::black_box('\u{787bf}'), std::hint::black_box((-22_isize)), std::hint::black_box(5125217912287785853_usize), std::hint::black_box((-88236428774110204286449923879684031302_i128)));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: isize,

},
Variant1{
fld0: u64,
fld1: *const f64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (i128, [char; 4]),
fld1: *const [u32; 1],
fld2: isize,

},
Variant1{
fld0: [u32; 1],
fld1: i64,

},
Variant2{
fld0: (usize,),
fld1: *mut isize,
fld2: i16,
fld3: *const [u32; 1],

},
Variant3{
fld0: bool,
fld1: [i64; 8],
fld2: isize,
fld3: [u32; 1],
fld4: i16,
fld5: u8,
fld6: u64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (i64, f64, usize, *const isize, isize, *const f64),
}
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
fld0: bool,
fld1: char,
fld2: u128,
fld3: *const u8,
fld4: [u32; 1],
fld5: *mut (i128, [char; 4]),
fld6: *mut u16,
fld7: i128,

},
Variant1{
fld0: *const isize,
fld1: Adt45,
fld2: i32,
fld3: (u32,),

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u128,
fld1: (usize,),
fld2: *mut (i128, [char; 4]),
fld3: Adt44,
fld4: u32,
fld5: f32,
fld6: i64,

},
Variant1{
fld0: [char; 6],
fld1: [u32; 1],
fld2: *const f64,
fld3: i64,

},
Variant2{
fld0: i32,

},
Variant3{
fld0: bool,
fld1: char,
fld2: (usize,),
fld3: i8,
fld4: *const (usize,),
fld5: u16,
fld6: (i64, f64, usize, *const isize, isize, *const f64),
fld7: Adt47,

}}
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i64; 8],
fld1: *mut isize,
fld2: isize,
fld3: i128,
fld4: i16,
fld5: [char; 4],

},
Variant1{
fld0: bool,
fld1: i128,
fld2: Adt46,
fld3: (u32,),
fld4: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8),

},
Variant2{
fld0: f32,
fld1: u8,
fld2: (usize,),
fld3: (u32,),
fld4: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8),
fld5: u16,
fld6: *mut isize,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [char; 6],
fld1: *mut u32,
fld2: [u32; 7],
fld3: (i16, i128, f32),
fld4: [bool; 7],

},
Variant1{
fld0: bool,
fld1: Adt45,
fld2: [u32; 2],
fld3: u16,

}}
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt46,
fld1: (i16, i128, f32),
fld2: *mut isize,

},
Variant1{
fld0: *mut isize,
fld1: (i128, [char; 4]),

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
fld0: i16,
fld1: (usize,),
fld2: isize,
fld3: i8,

},
Variant1{
fld0: bool,
fld1: ([char; 4], bool, i16),
fld2: u8,
fld3: *mut (i128, [char; 4]),
fld4: [u32; 1],
fld5: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8),
fld6: *const f64,

},
Variant2{
fld0: f32,
fld1: char,
fld2: isize,
fld3: u16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: Adt50,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: usize,

},
Variant1{
fld0: u128,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: bool,
fld1: (i128, [char; 4]),
fld2: Adt45,
fld3: u128,
fld4: Adt46,
fld5: (usize,),
fld6: ((i128, [char; 4]), *const (usize,), (i16, i128, f32), i8, u16, i8),
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: Adt55,
fld1: Adt49,
fld2: [bool; 7],
fld3: *const f64,
fld4: *const isize,
fld5: (u32,),
fld6: *const [char; 4],
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: [bool; 7],
fld1: char,
fld2: *const isize,
fld3: [char; 4],
fld4: *const [u32; 1],
fld5: Adt56,
fld6: *const (usize,),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt47,
fld1: i32,

},
Variant1{
fld0: *mut u16,
fld1: ([char; 4], bool, i16),
fld2: Adt55,
fld3: Adt53,

},
Variant2{
fld0: Adt50,
fld1: (usize,),
fld2: [u32; 7],
fld3: u8,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u32,

},
Variant1{
fld0: Adt54,
fld1: Adt56,
fld2: *const isize,
fld3: Adt46,
fld4: u64,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: (u32,),

},
Variant1{
fld0: ([char; 4], bool, i16),
fld1: Adt49,
fld2: Adt48,
fld3: (u32,),

},
Variant2{
fld0: (u32,),
fld1: Adt56,

}}

