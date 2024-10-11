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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: i128,mut _4: usize,mut _5: i16) -> *const i128 {
mir! {
type RET = *const i128;
let _6: [i128; 2];
let _7: [i32; 5];
let _8: bool;
let _9: i8;
let _10: Adt53;
let _11: u128;
let _12: *mut i32;
let _13: [bool; 2];
let _14: u128;
let _15: [i128; 7];
let _16: &'static f32;
let _17: i32;
let _18: Adt54;
let _19: i128;
let _20: (i16, [i8; 1], char, *mut i32);
let _21: [i32; 5];
let _22: (usize, ((f64, *mut i32), *mut bool, *mut i32), i64);
let _23: *mut u32;
let _24: (i16, [i8; 1], char, *mut i32);
let _25: i64;
let _26: [i8; 1];
let _27: Adt51;
let _28: ((f64, *mut i32), *mut bool, *mut i32);
let _29: i64;
let _30: bool;
let _31: f64;
let _32: ();
let _33: ();
{
_2 = '\u{6978a}';
_4 = 116_i8 as usize;
RET = core::ptr::addr_of!(_3);
_4 = 6_usize;
_1 = _4 <= _4;
_4 = !4_usize;
Call((*RET) = fn1(_1, RET, _1, _1, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 159338754284424984324233001911639767841_u128 as usize;
_3 = -(-98121645312014191383934653290535161044_i128);
_4 = 4_usize;
(*RET) = 26217926129725889738553191158329893452_i128 * (-33184115838558884453085276178952077370_i128);
_2 = '\u{4747a}';
_7 = [(-1257516744_i32),(-1691404349_i32),1503868495_i32,1995769284_i32,726494628_i32];
(*RET) = !(-67481793219789282969692733580297770557_i128);
_6 = [(*RET),(*RET)];
_6 = [_3,_3];
_4 = 739883289_u32 as usize;
_8 = _1;
_2 = '\u{57cf0}';
_5 = (-27821_i16) ^ (-9406_i16);
_9 = (-124_i8) - (-6_i8);
_1 = _2 <= _2;
(*RET) = (-8544785928173552341707444839571130917_i128);
_6 = [(*RET),_3];
_1 = _3 > (*RET);
_8 = _1 | _1;
(*RET) = -115805975141910612732144795604497100061_i128;
_5 = 41299_u16 as i16;
_1 = !_8;
_2 = '\u{218cc}';
(*RET) = 126967332411523969166474544677555394783_i128;
RET = core::ptr::addr_of!((*RET));
Goto(bb2)
}
bb2 = {
(*RET) = _5 as i128;
(*RET) = 167903162396862650455890971480273561982_i128 * (-100673332965711349786784045373216505568_i128);
(*RET) = 3554603420_u32 as i128;
RET = core::ptr::addr_of!(_3);
_5 = -(-21534_i16);
_1 = !_8;
Goto(bb3)
}
bb3 = {
(*RET) = 81488927455940336119749836316604790856_i128;
_1 = _4 != _4;
(*RET) = 75908763317497938673148911104431455960_i128;
_2 = '\u{9593e}';
_3 = 97343189497405501184916597203878836575_i128 >> _9;
Goto(bb4)
}
bb4 = {
_3 = (-141805021722050475371374117612768077955_i128);
_5 = 19859_i16;
_11 = !70209972122300065059073099645640976169_u128;
_2 = '\u{aed62}';
_11 = 14830008625013406482_u64 as u128;
_11 = !148875921501031314677588383377199691498_u128;
_13 = [_1,_8];
_11 = 292082450522122871635973605916704630715_u128 * 60868509220870031901923108821330573685_u128;
_5 = (-26562_i16) & (-25360_i16);
(*RET) = (-36476606427739220437048221798505374417_i128);
_7 = [(-975954197_i32),1311433577_i32,(-630186554_i32),(-889764463_i32),(-540584491_i32)];
(*RET) = 9655982341310727621_u64 as i128;
_15 = [_3,_3,_3,_3,(*RET),_3,(*RET)];
(*RET) = (-112190914681717144938720335084704491044_i128);
_13 = [_1,_1];
_13 = [_8,_1];
_6 = [(*RET),_3];
(*RET) = (-149528932275919613782344630186655776988_i128) << _11;
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of!((*RET));
_15 = [_3,(*RET),_3,(*RET),(*RET),(*RET),_3];
(*RET) = 11065392374260061477_u64 as i128;
_2 = '\u{3222e}';
(*RET) = -(-22313877165191328736257996516284038593_i128);
_6 = [(*RET),(*RET)];
_5 = -27254_i16;
_17 = !(-1336259246_i32);
_1 = !_8;
_2 = '\u{2398c}';
RET = core::ptr::addr_of!(_19);
_15 = [_3,_3,_3,_3,_3,_3,_3];
(*RET) = _5 as i128;
_20.1 = [_9];
(*RET) = -_3;
_14 = !_11;
_20.2 = _2;
_4 = _11 as usize;
_17 = -(-791255179_i32);
Goto(bb6)
}
bb6 = {
_4 = 1_u8 as usize;
_21 = [_17,_17,_17,_17,_17];
_20.3 = core::ptr::addr_of_mut!(_17);
_20.0 = _5;
_6 = [(*RET),_19];
_4 = 6_usize;
_5 = _4 as i16;
_22.1.0.1 = core::ptr::addr_of_mut!(_17);
_22.1.1 = core::ptr::addr_of_mut!(_1);
_6 = [(*RET),(*RET)];
(*RET) = _4 as i128;
(*RET) = _9 as i128;
_19 = _15[_4] & _15[_4];
_22.1.0.0 = _17 as f64;
_22.1.0.0 = 45172_u16 as f64;
_5 = _20.0 ^ _20.0;
_22.1.0.0 = _14 as f64;
_15[_4] = !(*RET);
_22.1.1 = core::ptr::addr_of_mut!(_1);
(*RET) = _15[_4] & _15[_4];
_13 = [_1,_1];
RET = core::ptr::addr_of!(_3);
_7 = _21;
_21 = [_17,_17,_17,_17,_17];
match _4 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb13,
_ => bb12
}
}
bb7 = {
RET = core::ptr::addr_of!((*RET));
_15 = [_3,(*RET),_3,(*RET),(*RET),(*RET),_3];
(*RET) = 11065392374260061477_u64 as i128;
_2 = '\u{3222e}';
(*RET) = -(-22313877165191328736257996516284038593_i128);
_6 = [(*RET),(*RET)];
_5 = -27254_i16;
_17 = !(-1336259246_i32);
_1 = !_8;
_2 = '\u{2398c}';
RET = core::ptr::addr_of!(_19);
_15 = [_3,_3,_3,_3,_3,_3,_3];
(*RET) = _5 as i128;
_20.1 = [_9];
(*RET) = -_3;
_14 = !_11;
_20.2 = _2;
_4 = _11 as usize;
_17 = -(-791255179_i32);
Goto(bb6)
}
bb8 = {
_3 = (-141805021722050475371374117612768077955_i128);
_5 = 19859_i16;
_11 = !70209972122300065059073099645640976169_u128;
_2 = '\u{aed62}';
_11 = 14830008625013406482_u64 as u128;
_11 = !148875921501031314677588383377199691498_u128;
_13 = [_1,_8];
_11 = 292082450522122871635973605916704630715_u128 * 60868509220870031901923108821330573685_u128;
_5 = (-26562_i16) & (-25360_i16);
(*RET) = (-36476606427739220437048221798505374417_i128);
_7 = [(-975954197_i32),1311433577_i32,(-630186554_i32),(-889764463_i32),(-540584491_i32)];
(*RET) = 9655982341310727621_u64 as i128;
_15 = [_3,_3,_3,_3,(*RET),_3,(*RET)];
(*RET) = (-112190914681717144938720335084704491044_i128);
_13 = [_1,_1];
_13 = [_8,_1];
_6 = [(*RET),_3];
(*RET) = (-149528932275919613782344630186655776988_i128) << _11;
Goto(bb5)
}
bb9 = {
(*RET) = 81488927455940336119749836316604790856_i128;
_1 = _4 != _4;
(*RET) = 75908763317497938673148911104431455960_i128;
_2 = '\u{9593e}';
_3 = 97343189497405501184916597203878836575_i128 >> _9;
Goto(bb4)
}
bb10 = {
(*RET) = _5 as i128;
(*RET) = 167903162396862650455890971480273561982_i128 * (-100673332965711349786784045373216505568_i128);
(*RET) = 3554603420_u32 as i128;
RET = core::ptr::addr_of!(_3);
_5 = -(-21534_i16);
_1 = !_8;
Goto(bb3)
}
bb11 = {
_4 = 159338754284424984324233001911639767841_u128 as usize;
_3 = -(-98121645312014191383934653290535161044_i128);
_4 = 4_usize;
(*RET) = 26217926129725889738553191158329893452_i128 * (-33184115838558884453085276178952077370_i128);
_2 = '\u{4747a}';
_7 = [(-1257516744_i32),(-1691404349_i32),1503868495_i32,1995769284_i32,726494628_i32];
(*RET) = !(-67481793219789282969692733580297770557_i128);
_6 = [(*RET),(*RET)];
_6 = [_3,_3];
_4 = 739883289_u32 as usize;
_8 = _1;
_2 = '\u{57cf0}';
_5 = (-27821_i16) ^ (-9406_i16);
_9 = (-124_i8) - (-6_i8);
_1 = _2 <= _2;
(*RET) = (-8544785928173552341707444839571130917_i128);
_6 = [(*RET),_3];
_1 = _3 > (*RET);
_8 = _1 | _1;
(*RET) = -115805975141910612732144795604497100061_i128;
_5 = 41299_u16 as i16;
_1 = !_8;
_2 = '\u{218cc}';
(*RET) = 126967332411523969166474544677555394783_i128;
RET = core::ptr::addr_of!((*RET));
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_22.1.1 = core::ptr::addr_of_mut!(_1);
_22.1.2 = _22.1.0.1;
_11 = _22.1.0.0 as u128;
_12 = core::ptr::addr_of_mut!(_17);
_20.3 = _12;
(*_12) = _5 as i32;
_22.0 = !_4;
_22.1.0.0 = (-47_isize) as f64;
_11 = _14 - _14;
_22.2 = (-3287731711171730835_i64);
_24.1 = [_9];
(*RET) = _22.0 as i128;
_22.2 = _9 as i64;
_22.0 = _4 & _4;
_15[_4] = _19 & (*RET);
(*RET) = _19 << _14;
_22.1.1 = core::ptr::addr_of_mut!(_1);
_20.3 = core::ptr::addr_of_mut!(_17);
_24.2 = _2;
_5 = _20.0 - _20.0;
_22.1.0.0 = _11 as f64;
_20.3 = core::ptr::addr_of_mut!((*_12));
_24.1 = [_9];
(*_12) = (-1359935877_i32) + (-322421311_i32);
_1 = _8 & _8;
_7 = _21;
_28 = (_22.1.0, _22.1.1, _12);
Goto(bb14)
}
bb14 = {
(*RET) = -_19;
_28 = _22.1;
_2 = _20.2;
_28.0.0 = -_22.1.0.0;
_28.0 = (_22.1.0.0, _22.1.2);
_24.3 = _12;
_12 = _20.3;
RET = core::ptr::addr_of!(_3);
_21 = _7;
_1 = _19 == (*RET);
_9 = 73_u8 as i8;
_8 = _1;
_22.1.0.0 = _28.0.0 + _28.0.0;
_17 = (-1649452826_i32) | (-1197001597_i32);
_14 = _11;
_24 = (_5, _20.1, _20.2, _22.1.0.1);
_24 = (_5, _20.1, _2, _22.1.2);
_29 = _22.2 ^ _22.2;
_11 = _9 as u128;
_28.1 = _22.1.1;
_19 = 3401358132_u32 as i128;
_21 = [(*_12),(*_12),_17,(*_12),(*_12)];
_22.1.0.1 = _24.3;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(0_usize, 29_usize, Move(_29), 5_usize, Move(_5), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(0_usize, 4_usize, Move(_4), 19_usize, Move(_19), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: *const i128,mut _3: bool,mut _4: bool,mut _5: *const i128,mut _6: *const i128,mut _7: *const i128,mut _8: *const i128) -> i128 {
mir! {
type RET = i128;
let _9: f64;
let _10: [isize; 7];
let _11: bool;
let _12: (u8, i8, f64, (u8, i32, bool), i16);
let _13: i128;
let _14: [i8; 1];
let _15: f64;
let _16: [isize; 7];
let _17: f64;
let _18: Adt45;
let _19: (u8, i32, bool);
let _20: u64;
let _21: i64;
let _22: i64;
let _23: *const i128;
let _24: (f64, *mut i32);
let _25: &'static f32;
let _26: f32;
let _27: (u8, i8, f64, (u8, i32, bool), i16);
let _28: isize;
let _29: i16;
let _30: f32;
let _31: bool;
let _32: isize;
let _33: [i128; 7];
let _34: [isize; 7];
let _35: (i32, i128);
let _36: char;
let _37: [i128; 7];
let _38: i128;
let _39: i32;
let _40: ();
let _41: ();
{
_6 = _7;
RET = !(-129142253879672787411781132398124503979_i128);
_8 = core::ptr::addr_of!(RET);
_5 = _7;
_6 = _2;
(*_8) = -(-124593199120165126276258953288514256299_i128);
_8 = _6;
_1 = _3;
_4 = _1;
_8 = core::ptr::addr_of!(RET);
RET = (-806081424_i32) as i128;
RET = 4594129522651027184525089706893483957_i128;
_9 = 597243440_u32 as f64;
_3 = !_1;
_6 = _5;
(*_8) = 38055_u16 as i128;
(*_8) = -(-123905069123840408297248069970665925998_i128);
_8 = _7;
_8 = core::ptr::addr_of!(RET);
Goto(bb1)
}
bb1 = {
_5 = core::ptr::addr_of!(RET);
_3 = _4 > _4;
(*_8) = (-47912835896432153899428357675932520757_i128) | (-46026213013848838196863928822800702809_i128);
_1 = !_3;
_10 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,27_isize];
(*_8) = !132902624793838188726042464238656052521_i128;
(*_5) = -(-79009917057561142680331652854875827185_i128);
_11 = _3;
_6 = _7;
_12.1 = 195_u8 as i8;
_5 = _2;
_12.1 = (-128_i8) & (-5_i8);
Call(_12.3 = fn2(_2, _2, _6, _7, _6, _5, _2, _8, _7, _10, _4, _11, _10, _3, _4, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.2 = 9223372036854775807_isize as f64;
_13 = (*_8) ^ RET;
_13 = !(*_8);
_2 = _6;
_13 = _12.2 as i128;
_14 = [_12.1];
_12.3.0 = 210_u8;
_7 = _2;
_5 = core::ptr::addr_of!((*_8));
_12.3 = (41_u8, 1845295484_i32, _1);
_15 = -_12.2;
RET = 11151050105417493850_u64 as i128;
_5 = core::ptr::addr_of!((*_8));
_2 = core::ptr::addr_of!((*_5));
(*_2) = _13 >> _12.3.0;
Goto(bb3)
}
bb3 = {
RET = -_13;
(*_2) = _13 + _13;
_9 = 14350_i16 as f64;
_9 = -_12.2;
(*_8) = _13;
(*_8) = _13;
_15 = _12.2;
_12.4 = 15849_i16 << (*_2);
_4 = !_3;
_9 = -_15;
(*_5) = _13 << _12.1;
_12.0 = _12.3.1 as u8;
_10 = [110_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,15_isize,9223372036854775807_isize,111_isize];
_8 = _6;
_12.0 = _12.3.0 >> (*_2);
_15 = 566150848376528232_i64 as f64;
(*_2) = 46858_u16 as i128;
_2 = core::ptr::addr_of!(RET);
Goto(bb4)
}
bb4 = {
RET = -_13;
_20 = 8097246499247569171_u64 + 8223809287944379554_u64;
_19.1 = _13 as i32;
(*_2) = _13;
RET = _12.1 as i128;
_19 = (_12.3.0, _12.3.1, _1);
(*_5) = (-9223372036854775808_isize) as i128;
_23 = core::ptr::addr_of!((*_2));
_25 = &_26;
Goto(bb5)
}
bb5 = {
_26 = _9 as f32;
_27.0 = 11652_u16 as u8;
_24.0 = 43572467523497234630941022268953684740_u128 as f64;
_27.3.2 = !_3;
_27.1 = _12.1;
_27.3 = (_12.0, _12.3.1, _1);
_14 = [_12.1];
(*_2) = _12.3.0 as i128;
_12.1 = _27.1;
_17 = _12.2;
_21 = 8622804230616399884_i64;
RET = 45403_u16 as i128;
_27.2 = _12.2;
_12 = (_19.0, _27.1, _24.0, _19, 404_i16);
_3 = _27.3.2;
_12.0 = !_12.3.0;
_19.1 = !_12.3.1;
_3 = _27.3.1 != _27.3.1;
_27.3.0 = _12.0;
_23 = core::ptr::addr_of!((*_23));
_15 = 135080361215573548998173532330953108166_u128 as f64;
_12.4 = 15508_i16;
_12.3 = _19;
_24.1 = core::ptr::addr_of_mut!(_19.1);
_16 = _10;
_31 = _3;
match _12.3.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
41 => bb11,
_ => bb10
}
}
bb6 = {
RET = -_13;
_20 = 8097246499247569171_u64 + 8223809287944379554_u64;
_19.1 = _13 as i32;
(*_2) = _13;
RET = _12.1 as i128;
_19 = (_12.3.0, _12.3.1, _1);
(*_5) = (-9223372036854775808_isize) as i128;
_23 = core::ptr::addr_of!((*_2));
_25 = &_26;
Goto(bb5)
}
bb7 = {
RET = -_13;
(*_2) = _13 + _13;
_9 = 14350_i16 as f64;
_9 = -_12.2;
(*_8) = _13;
(*_8) = _13;
_15 = _12.2;
_12.4 = 15849_i16 << (*_2);
_4 = !_3;
_9 = -_15;
(*_5) = _13 << _12.1;
_12.0 = _12.3.1 as u8;
_10 = [110_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,15_isize,9223372036854775807_isize,111_isize];
_8 = _6;
_12.0 = _12.3.0 >> (*_2);
_15 = 566150848376528232_i64 as f64;
(*_2) = 46858_u16 as i128;
_2 = core::ptr::addr_of!(RET);
Goto(bb4)
}
bb8 = {
_12.2 = 9223372036854775807_isize as f64;
_13 = (*_8) ^ RET;
_13 = !(*_8);
_2 = _6;
_13 = _12.2 as i128;
_14 = [_12.1];
_12.3.0 = 210_u8;
_7 = _2;
_5 = core::ptr::addr_of!((*_8));
_12.3 = (41_u8, 1845295484_i32, _1);
_15 = -_12.2;
RET = 11151050105417493850_u64 as i128;
_5 = core::ptr::addr_of!((*_8));
_2 = core::ptr::addr_of!((*_5));
(*_2) = _13 >> _12.3.0;
Goto(bb3)
}
bb9 = {
_5 = core::ptr::addr_of!(RET);
_3 = _4 > _4;
(*_8) = (-47912835896432153899428357675932520757_i128) | (-46026213013848838196863928822800702809_i128);
_1 = !_3;
_10 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,27_isize];
(*_8) = !132902624793838188726042464238656052521_i128;
(*_5) = -(-79009917057561142680331652854875827185_i128);
_11 = _3;
_6 = _7;
_12.1 = 195_u8 as i8;
_5 = _2;
_12.1 = (-128_i8) & (-5_i8);
Call(_12.3 = fn2(_2, _2, _6, _7, _6, _5, _2, _8, _7, _10, _4, _11, _10, _3, _4, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_22 = _21 << _19.0;
_20 = 1680466789798099684_u64;
_15 = -_24.0;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = core::ptr::addr_of!((*_23));
_10 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-2_isize),(-9223372036854775808_isize),53_isize,(-9223372036854775808_isize)];
_24.0 = _13 as f64;
_29 = _12.4 * _12.4;
_27.0 = _27.3.0 & _12.0;
Call((*_2) = fn3(_10, _27.3.0, _19.0, _27.2, _19.1, _7, _9, _27.3.0, _24, _1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_2) = _13;
(*_8) = _29 as i128;
_27.1 = -_12.1;
_12.4 = -_29;
RET = _13 & _13;
_12.0 = !_27.3.0;
_18 = Adt45::Variant0 { fld0: 2951727012_u32,fld1: _24 };
_17 = _27.1 as f64;
_3 = _31 | _19.2;
_27.3.0 = _27.0;
_6 = core::ptr::addr_of!(_13);
_24.0 = _15 * _27.2;
_27.3.0 = 595424936_u32 as u8;
(*_6) = !RET;
_12.3 = (_19.0, _27.3.1, _27.3.2);
_27 = (_19.0, _12.1, _12.2, _19, _29);
_10 = _16;
place!(Field::<u32>(Variant(_18, 0), 0)) = !3358347450_u32;
_27.3.1 = _27.2 as i32;
_12.0 = _27.0 - _27.3.0;
_12.4 = _29;
SetDiscriminant(_18, 0);
_32 = -58_isize;
_16 = _10;
_2 = _7;
_32 = (-25_isize);
_27.1 = 54856_u16 as i8;
_27.4 = _29 + _29;
_7 = core::ptr::addr_of!((*_6));
_12.2 = _20 as f64;
match _19.0 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb7,
41 => bb13,
_ => bb8
}
}
bb13 = {
_19.0 = _27.3.0;
_12.4 = -_27.4;
_12.3 = (_27.0, _27.3.1, _3);
place!(Field::<(f64, *mut i32)>(Variant(_18, 0), 1)).0 = _15;
(*_7) = (*_23);
place!(Field::<(f64, *mut i32)>(Variant(_18, 0), 1)) = (_17, _24.1);
_12.3.2 = _31;
_16 = _10;
_6 = core::ptr::addr_of!((*_23));
_13 = (*_8) & (*_6);
(*_23) = _13 * _13;
_19.0 = _12.0 / _27.3.0;
Goto(bb14)
}
bb14 = {
place!(Field::<u32>(Variant(_18, 0), 0)) = !31947614_u32;
_33 = [(*_6),(*_6),(*_5),(*_6),(*_6),RET,(*_7)];
(*_8) = _13;
_27.3.0 = _19.0 - _12.0;
_28 = _32 * _32;
_35 = (_27.3.1, (*_23));
_27.4 = _12.4 | _12.4;
_27.4 = _29;
_11 = !_12.3.2;
_27.3 = (_12.0, _35.0, _31);
_12.3 = (_19.0, _27.3.1, _4);
_19 = _27.3;
_29 = !_12.4;
(*_23) = -(*_7);
_27.4 = !_29;
place!(Field::<(f64, *mut i32)>(Variant(_18, 0), 1)).0 = _12.2;
_24.1 = core::ptr::addr_of_mut!(_35.0);
_22 = !_21;
_39 = _35.0 ^ _19.1;
_12.1 = _27.1;
_12.2 = Field::<(f64, *mut i32)>(Variant(_18, 0), 1).0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(1_usize, 11_usize, Move(_11), 4_usize, Move(_4), 39_usize, Move(_39), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 20_usize, Move(_20), 28_usize, Move(_28), 14_usize, Move(_14), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(1_usize, 1_usize, Move(_1), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: *const i128,mut _2: *const i128,mut _3: *const i128,mut _4: *const i128,mut _5: *const i128,mut _6: *const i128,mut _7: *const i128,mut _8: *const i128,mut _9: *const i128,mut _10: [isize; 7],mut _11: bool,mut _12: bool,mut _13: [isize; 7],mut _14: bool,mut _15: bool,mut _16: bool) -> (u8, i32, bool) {
mir! {
type RET = (u8, i32, bool);
let _17: bool;
let _18: isize;
let _19: ((f64, *mut i32),);
let _20: (u8, i32, bool);
let _21: isize;
let _22: i128;
let _23: isize;
let _24: [i32; 5];
let _25: [isize; 7];
let _26: (u8, i32, bool);
let _27: (u8, i32, bool);
let _28: i128;
let _29: i16;
let _30: i16;
let _31: isize;
let _32: ();
let _33: ();
{
_3 = _4;
RET.0 = 127_u8 >> (*_8);
_8 = _2;
_5 = _2;
_8 = _1;
_5 = _1;
_8 = _7;
_5 = _8;
RET.2 = !_14;
RET.1 = 6338401557285900458_i64 as i32;
_14 = _12;
RET = (252_u8, (-1578546548_i32), _14);
_16 = _11 < RET.2;
_4 = _1;
_1 = _2;
_1 = _3;
_6 = _9;
RET = (251_u8, 2009553013_i32, _16);
_7 = _8;
_10 = _13;
_15 = _16;
_13 = [9223372036854775807_isize,(-116_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-35_isize),96_isize,(-41_isize)];
_8 = _4;
_10 = _13;
_9 = _6;
RET.1 = 1384168770_i32;
RET.1 = 9790609468656261416_usize as i32;
_18 = 12089_i16 as isize;
RET.2 = _14 == _14;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
251 => bb9,
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
RET.2 = _12;
_2 = _3;
_20 = RET;
_12 = _18 > _18;
_20 = (RET.0, RET.1, RET.2);
RET.0 = 9983_u16 as u8;
_6 = _9;
_6 = _9;
match _20.0 {
0 => bb5,
1 => bb4,
251 => bb10,
_ => bb8
}
}
bb10 = {
_13 = _10;
_13 = _10;
_16 = _11;
_7 = core::ptr::addr_of!(_22);
_14 = _15;
_8 = core::ptr::addr_of!((*_7));
_6 = _3;
_19.0.1 = core::ptr::addr_of_mut!(_20.1);
_14 = _20.2 != RET.2;
RET.2 = _15 | _12;
_1 = _9;
_3 = _2;
(*_7) = 6277622308427195185722707754219272438_i128 >> _20.0;
_19.0.0 = 109_i8 as f64;
_18 = (-9223372036854775808_isize);
_16 = !_15;
_18 = (-120_isize);
_26 = (_20.0, RET.1, RET.2);
_24 = [RET.1,_20.1,_20.1,_26.1,_20.1];
_1 = core::ptr::addr_of!(_22);
_26 = (_20.0, RET.1, RET.2);
(*_1) = (-32396_i16) as i128;
(*_7) = (-3223709516137558490_i64) as i128;
_27.1 = !_26.1;
_12 = RET.2 | _20.2;
_18 = 9223372036854775807_isize * (-9223372036854775808_isize);
match _20.0 {
0 => bb11,
1 => bb12,
2 => bb13,
251 => bb15,
_ => bb14
}
}
bb11 = {
RET.2 = _12;
_2 = _3;
_20 = RET;
_12 = _18 > _18;
_20 = (RET.0, RET.1, RET.2);
RET.0 = 9983_u16 as u8;
_6 = _9;
_6 = _9;
match _20.0 {
0 => bb5,
1 => bb4,
251 => bb10,
_ => bb8
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
_26.1 = !RET.1;
RET.0 = _20.0 & _20.0;
_27.0 = _20.0;
_21 = _18 - _18;
_3 = _4;
(*_1) = 20897536705783973884692833558259024638_i128 * (-133134638714411907542556771775443549585_i128);
RET.1 = '\u{6648f}' as i32;
_26.0 = _27.0 ^ _20.0;
_19.0.0 = 17710046608848785488_u64 as f64;
_25 = _10;
_20.0 = !_26.0;
_27.1 = _20.1 - RET.1;
_26.2 = _20.0 > _20.0;
RET.1 = 23294_i16 as i32;
_26.2 = !RET.2;
_19.0.1 = core::ptr::addr_of_mut!(_26.1);
_20.1 = -_27.1;
_29 = (-9700_i16);
_30 = !_29;
_27.2 = !_15;
_8 = _5;
_27 = (RET.0, _26.1, _16);
_3 = core::ptr::addr_of!((*_7));
_10 = _13;
_19.0.1 = core::ptr::addr_of_mut!(_26.1);
(*_3) = (-111185549474069249891066376041588440445_i128);
_23 = _18;
_17 = _16;
RET.1 = -_20.1;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(2_usize, 29_usize, Move(_29), 20_usize, Move(_20), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(2_usize, 26_usize, Move(_26), 17_usize, Move(_17), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(2_usize, 16_usize, Move(_16), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [isize; 7],mut _2: u8,mut _3: u8,mut _4: f64,mut _5: i32,mut _6: *const i128,mut _7: f64,mut _8: u8,mut _9: (f64, *mut i32),mut _10: bool) -> i128 {
mir! {
type RET = i128;
let _11: (((f64, *mut i32),),);
let _12: i64;
let _13: char;
let _14: f32;
let _15: Adt57;
let _16: [i8; 1];
let _17: i64;
let _18: isize;
let _19: ((f64, *mut i32),);
let _20: ();
let _21: ();
{
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = (-122_isize) as i32;
_1 = [(-9223372036854775808_isize),56_isize,(-20_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-33_isize)];
_5 = 43_i8 as i32;
_11.0.0 = _9;
_7 = -_11.0.0.0;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
41 => bb6,
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
_1 = [(-9223372036854775808_isize),28_isize,(-74_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-66_isize),(-83_isize)];
_4 = _7;
RET = (-119744276339284189248615054501552544131_i128) | 22433423560664435873931837821320249051_i128;
_14 = 181283750936608043397968945266697632142_u128 as f32;
_14 = _5 as f32;
Call(_9.1 = fn4(_6, _2, _2, _11.0, _5, _8, _3, _10, _6, _10, _11, _6, _3, _11.0.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = !1267721191_i32;
_11.0.0.1 = core::ptr::addr_of_mut!(_5);
_9.0 = _5 as f64;
_1 = [26_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-18_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_14 = (-858_i16) as f32;
RET = !(-9306990772996851835209344004343015894_i128);
_8 = _2 & _2;
_6 = core::ptr::addr_of!(RET);
_11.0.0.0 = _4 * _7;
(*_6) = 97994964301161431691404503355324818790_i128;
_16 = [(-41_i8)];
_11.0.0 = _9;
_12 = -3516539850965549592_i64;
match (*_6) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
97994964301161431691404503355324818790 => bb16,
_ => bb15
}
}
bb8 = {
_1 = [(-9223372036854775808_isize),28_isize,(-74_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-66_isize),(-83_isize)];
_4 = _7;
RET = (-119744276339284189248615054501552544131_i128) | 22433423560664435873931837821320249051_i128;
_14 = 181283750936608043397968945266697632142_u128 as f32;
_14 = _5 as f32;
Call(_9.1 = fn4(_6, _2, _2, _11.0, _5, _8, _3, _10, _6, _10, _11, _6, _3, _11.0.0), ReturnTo(bb7), UnwindUnreachable())
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
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
(*_6) = -(-165698206629110993299669331905806705028_i128);
_5 = (-968120991_i32);
_5 = 611822367_i32 + 1547817190_i32;
_3 = _8 >> _2;
_16 = [49_i8];
_9.1 = core::ptr::addr_of_mut!(_5);
_19 = _11.0;
Goto(bb17)
}
bb17 = {
Call(_20 = dump_var(3_usize, 2_usize, Move(_2), 8_usize, Move(_8), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: *const i128,mut _2: u8,mut _3: u8,mut _4: ((f64, *mut i32),),mut _5: i32,mut _6: u8,mut _7: u8,mut _8: bool,mut _9: *const i128,mut _10: bool,mut _11: (((f64, *mut i32),),),mut _12: *const i128,mut _13: u8,mut _14: (f64, *mut i32)) -> *mut i32 {
mir! {
type RET = *mut i32;
let _15: ([i128; 7],);
let _16: f32;
let _17: (u8, i32, bool);
let _18: (i16, [i8; 1], char, *mut i32);
let _19: isize;
let _20: Adt53;
let _21: bool;
let _22: [i128; 7];
let _23: bool;
let _24: isize;
let _25: [i32; 5];
let _26: [i128; 7];
let _27: isize;
let _28: isize;
let _29: u8;
let _30: f32;
let _31: (i32, i128);
let _32: isize;
let _33: Adt53;
let _34: *mut u32;
let _35: f64;
let _36: (f64, *mut i32);
let _37: Adt56;
let _38: isize;
let _39: bool;
let _40: u64;
let _41: ((i32, i128), *const i32);
let _42: isize;
let _43: i16;
let _44: [isize; 7];
let _45: f64;
let _46: Adt43;
let _47: (u8, i8, f64, (u8, i32, bool), i16);
let _48: *const i32;
let _49: u128;
let _50: i16;
let _51: ([i128; 7],);
let _52: *const usize;
let _53: i64;
let _54: ([i128; 7],);
let _55: [i128; 2];
let _56: ((i32, i128), *const i32);
let _57: u8;
let _58: [isize; 7];
let _59: ();
let _60: ();
{
_14.0 = _4.0.0 + _4.0.0;
_14.1 = core::ptr::addr_of_mut!(_5);
_4.0 = (_14.0, _11.0.0.1);
_14.1 = _4.0.1;
_4.0.0 = _14.0;
_8 = !_10;
_2 = _8 as u8;
_3 = _13 * _6;
_11.0.0 = (_14.0, _4.0.1);
_11 = (_4,);
RET = _14.1;
_11.0.0 = _14;
_15.0 = [27632412234036320738771952272239110396_i128,48968581998430651152262011281172717900_i128,89103993079895999053068663651881806749_i128,2041432954381849550639667437386165347_i128,(-38479930686523808586752825616925932480_i128),(-99119518132226266964316685839475179958_i128),(-106376177649669847808576667431154465950_i128)];
_14.1 = _4.0.1;
_16 = 122_i8 as f32;
Goto(bb1)
}
bb1 = {
_10 = (*RET) == (*RET);
_11 = (_4,);
_13 = 5630119912633332852_usize as u8;
(*RET) = _5 >> _3;
(*RET) = 1153319807_u32 as i32;
_15.0 = [135990336847167325113849289198828297731_i128,(-117955007766062643023700681798608279985_i128),(-124657604245462412008476704916906833731_i128),54750948171466495965282037596734582968_i128,(-37366925950094087458529394797555415556_i128),(-20150956197498548193544347800203241521_i128),(-114096219418848152836291076485789765986_i128)];
(*RET) = _5;
RET = _11.0.0.1;
_1 = _9;
_4.0.0 = _11.0.0.0 * _11.0.0.0;
(*RET) = _5 + _5;
_8 = !_10;
_11.0 = (_14,);
_6 = 308415835359013438424981629696112361291_u128 as u8;
_11.0.0 = (_14.0, _14.1);
_9 = _1;
_19 = (-119_isize) >> _2;
_14.0 = -_4.0.0;
_4.0.0 = (-5359_i16) as f64;
_18.0 = -19802_i16;
_4 = (_11.0.0,);
_22 = [(-104047751766344089504195888334849839197_i128),(-162693647335154322446628792409018310042_i128),161747697383308852199889429128076172577_i128,25253985361943811411775328468239627383_i128,45475956293071071009582537257144374335_i128,(-47808498865703602293961798258462211749_i128),149109482934480255848373079857912299268_i128];
Call(_15.0 = fn5(_7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _3;
Goto(bb3)
}
bb3 = {
_9 = _12;
_1 = _12;
_17.2 = !_10;
_17.1 = !(*RET);
_7 = (-116103588745321867083774541892656675643_i128) as u8;
_18.0 = -(-751_i16);
_11.0.0.0 = _14.0;
_8 = _17.2;
_17 = (_3, _5, _8);
_15.0 = [84410066939637305842517036031244506361_i128,(-96075688049776096553690285082718886108_i128),(-34455125595180841431158192147485828001_i128),91463724470549000535053378366787053123_i128,(-28706692411585125344479734849874677158_i128),20410201143706460769438283876102450142_i128,(-76355805081581905466912855254150277178_i128)];
_24 = _19 - _19;
_18.1 = [48_i8];
_4.0.0 = 222185289983489820_u64 as f64;
(*RET) = -_17.1;
_4.0.1 = RET;
Call((*RET) = fn18(_15.0, _14, _12, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = _17.0 | _17.0;
RET = core::ptr::addr_of_mut!(_5);
_10 = (*RET) > _5;
(*RET) = 949860375_u32 as i32;
_15.0 = [144100638939273431807600090688993715191_i128,8288719992126239397534644648393221137_i128,93908842068179494228340291584000855856_i128,(-82716608265066822396164590512589403636_i128),(-61792394035535560009028194573872629445_i128),59805930421369714789066841441148804201_i128,80231720630424394572006293985044508013_i128];
_15 = (_22,);
_22 = _15.0;
(*RET) = _17.1;
_4.0.0 = 159206696683733430101280596555212829693_i128 as f64;
_12 = _9;
_4.0 = (_14.0, RET);
_2 = !_17.0;
_13 = _5 as u8;
_9 = _1;
_5 = _17.1 * _17.1;
_5 = (-88_i8) as i32;
_11 = (_4,);
_13 = _17.0;
(*RET) = _17.1;
_16 = _5 as f32;
_28 = _24 | _24;
_11.0 = (_4.0,);
_10 = _17.2;
_18.3 = core::ptr::addr_of_mut!((*RET));
_15.0 = _22;
_4.0.0 = 128059797263649904558869981897559964102_i128 as f64;
RET = _18.3;
_15 = (_22,);
Goto(bb5)
}
bb5 = {
_26 = [(-15513086521907920426476087236960723341_i128),(-164951430254272998171848850074018503272_i128),(-133837674334101128764210246637726237664_i128),95354017221104753650437899642049708614_i128,101263208253571306008740315616353697712_i128,(-136449101602294098687674376702738209667_i128),(-31755432772055357943917036754068990486_i128)];
_21 = !_17.2;
_27 = _24 >> _3;
_18.1 = [(-33_i8)];
_22 = [88506194859357225412640155975535491229_i128,107088223869102609504440667176180438775_i128,77563003459463331955107236406512944782_i128,(-28630446293713625513459935355692682487_i128),163382572534637087107159273483701193438_i128,100075650474309826013024181144537789523_i128,(-161906539197098401380096936969988797776_i128)];
_31 = ((*RET), 115901405394201726596416537613388686559_i128);
_5 = -_17.1;
_14.0 = _11.0.0.0;
_23 = _27 < _27;
_26 = [_31.1,_31.1,_31.1,_31.1,_31.1,_31.1,_31.1];
_15 = (_26,);
_15.0 = _22;
_23 = _28 != _27;
_4.0 = _14;
_30 = _16;
_31.0 = _17.1 + (*RET);
_10 = _23 > _23;
_31.1 = (-146947811283100360522628207024780382576_i128) + (-71357768222337818912663143940394794932_i128);
_14 = (_11.0.0.0, _4.0.1);
_24 = _27;
_10 = _23;
_4.0.0 = _14.0 - _11.0.0.0;
_17 = (_3, (*RET), _10);
Goto(bb6)
}
bb6 = {
_19 = _28;
_3 = _17.0;
_26 = [_31.1,_31.1,_31.1,_31.1,_31.1,_31.1,_31.1];
Goto(bb7)
}
bb7 = {
_32 = _27;
_4.0.0 = _11.0.0.0;
(*RET) = '\u{81947}' as i32;
_7 = !_17.0;
_4.0.1 = core::ptr::addr_of_mut!(_31.0);
_18.1 = [44_i8];
_23 = _21;
_28 = -_32;
_10 = !_17.2;
_18.1 = [102_i8];
_6 = _17.1 as u8;
_11.0.0.1 = _14.1;
_36.0 = -_4.0.0;
RET = _14.1;
_6 = !_3;
_28 = !_27;
_11.0 = (_14,);
_11.0.0 = (_14.0, _14.1);
_9 = _1;
_38 = _28 + _27;
_3 = 12881_u16 as u8;
_35 = _11.0.0.0 - _36.0;
_10 = !_17.2;
_40 = _11.0.0.0 as u64;
Goto(bb8)
}
bb8 = {
_7 = 1211442576_u32 as u8;
_41.0.0 = -_17.1;
_18.3 = core::ptr::addr_of_mut!(_5);
_7 = _17.0;
_41.0 = _31;
_36 = (_35, _14.1);
_39 = _28 > _32;
_41.0.0 = _31.0;
_17.0 = !_6;
_42 = !_27;
_8 = _39;
_17 = (_7, (*RET), _8);
_32 = _38;
_18.3 = core::ptr::addr_of_mut!((*RET));
_11.0.0.1 = core::ptr::addr_of_mut!((*RET));
_45 = 43952_u16 as f64;
_18.0 = 28429_i16 << _27;
_47.4 = _17.2 as i16;
RET = _4.0.1;
_42 = _32;
_8 = !_21;
Goto(bb9)
}
bb9 = {
_41.0.1 = _31.1;
_15.0 = _22;
_28 = _47.4 as isize;
_30 = _36.0 as f32;
_3 = !_2;
_44 = [_38,_24,_27,_27,_28,_32,_27];
_14.0 = _36.0 + _11.0.0.0;
_43 = _18.0 + _18.0;
_47.2 = _14.0 + _35;
_1 = core::ptr::addr_of!(_41.0.1);
_4.0 = _36;
_49 = _47.2 as u128;
_7 = _47.2 as u8;
_38 = (-110_i8) as isize;
_4.0 = _14;
_47.1 = (*RET) as i8;
Goto(bb10)
}
bb10 = {
_48 = core::ptr::addr_of!((*RET));
_13 = _17.0 - _7;
(*_1) = -_31.1;
_4 = (_36,);
_15.0 = [_41.0.1,_41.0.1,_31.1,_41.0.1,_31.1,_31.1,_41.0.1];
(*_48) = _41.0.0 << _43;
(*RET) = _5;
(*RET) = _5 >> _17.0;
_12 = core::ptr::addr_of!(_31.1);
(*_1) = -(*_12);
_4.0 = _14;
_47.1 = _42 as i8;
_11.0.0.1 = RET;
_6 = _13 * _7;
_36.0 = _47.2 * _14.0;
_36.1 = core::ptr::addr_of_mut!(_17.1);
Call(_45 = core::intrinsics::transmute(_19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1 = _9;
Goto(bb12)
}
bb12 = {
_17.0 = _32 as u8;
_47.3.2 = !_10;
_19 = _27 << _49;
_18.2 = '\u{37f83}';
_29 = _17.0;
_50 = _43;
_51.0 = [_41.0.1,(*_12),(*_12),_41.0.1,_31.1,(*_12),(*_12)];
_47.3.1 = _47.1 as i32;
_9 = _1;
_11.0 = _4;
RET = core::ptr::addr_of_mut!((*_48));
_2 = 3556589393145651792_i64 as u8;
_56.0 = (_47.3.1, (*_12));
_4 = (_14,);
_25 = [_47.3.1,_47.3.1,_56.0.0,_56.0.0,_56.0.0];
_53 = -8284955901083492573_i64;
_12 = core::ptr::addr_of!((*_12));
_11.0 = (_36,);
_57 = !_6;
Goto(bb13)
}
bb13 = {
Call(_59 = dump_var(4_usize, 40_usize, Move(_40), 10_usize, Move(_10), 26_usize, Move(_26), 31_usize, Move(_31)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_59 = dump_var(4_usize, 19_usize, Move(_19), 39_usize, Move(_39), 51_usize, Move(_51), 6_usize, Move(_6)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_59 = dump_var(4_usize, 29_usize, Move(_29), 28_usize, Move(_28), 22_usize, Move(_22), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_59 = dump_var(4_usize, 42_usize, Move(_42), 43_usize, Move(_43), 27_usize, Move(_27), 50_usize, Move(_50)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u8,mut _2: *const i128) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _3: i32;
let _4: char;
let _5: Adt56;
let _6: *mut *const usize;
let _7: *mut *const usize;
let _8: ((f64, *mut i32), *mut bool, *mut i32);
let _9: i32;
let _10: f32;
let _11: i16;
let _12: i32;
let _13: isize;
let _14: Adt47;
let _15: Adt52;
let _16: ([i128; 7],);
let _17: bool;
let _18: Adt55;
let _19: usize;
let _20: isize;
let _21: ([i128; 7],);
let _22: isize;
let _23: (((f64, *mut i32),),);
let _24: (u8, i32, bool);
let _25: i128;
let _26: [isize; 7];
let _27: bool;
let _28: ();
let _29: ();
{
RET = [166604150682151387488847953880631674181_i128,116025670875833254057764337942230905528_i128,63316287746691082957611009574342578810_i128,21772739862638979661764868535070241000_i128,66213669348816447306954075782964057420_i128,77628285197972752506228724151846620668_i128,(-95898840699595318147509409193793026221_i128)];
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
41 => bb6,
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
_3 = 842998053_i32;
_1 = !241_u8;
RET = [103941284410567447268149051291221758148_i128,(-5849929558636102925406939230076146569_i128),155266107917324500631305573919985933712_i128,(-121026269675774444291521528274876340220_i128),(-3177495286153779329636374390895328852_i128),(-165930696648256179598610874470382323025_i128),80766030213227527984712648480261933334_i128];
RET = [142996852347675130905234598377442133256_i128,(-151366035844051207232789729240345386325_i128),(-23953750728285571252599784029440826632_i128),(-170009526259269499310229813228230188056_i128),(-59694404850093692272192323730810752420_i128),102492448944077666587667000424112941549_i128,111035586243765220486566511227148298854_i128];
RET = [96910001134636279761342660427867509546_i128,(-98852676746955797693077236143926694772_i128),(-166280133426157850602024629987578890974_i128),41818890028757455788840438804497560615_i128,(-64508986242863087038734474878012847171_i128),(-21631179000450867680064988457720046591_i128),(-44581724795583827083419108416046705118_i128)];
_1 = 40_u8 + 249_u8;
_4 = '\u{3824e}';
_4 = '\u{dbd1b}';
RET = [93745395632916988840312332463296620224_i128,(-29126533842345464336267610906992093531_i128),(-67847763326575867875088716756165298258_i128),162843316192504511226071513272525371256_i128,17867038265187647620209354000040526349_i128,(-110748617518048090459389232703620207434_i128),(-99993200608685792067265086797088191990_i128)];
_1 = 161_u8 & 102_u8;
_4 = '\u{13c08}';
RET = [(-141420976704407837760122322890250489665_i128),93068440140187928052201473809793253706_i128,130689534272443641016282741295674461745_i128,83421453452478612603450786257789129609_i128,90155761745356403323939642359666397312_i128,130881490981142023554242101816507860605_i128,70647780670377455903310803061684364652_i128];
_3 = -(-714477681_i32);
RET = [80927725348720574154077990246470138632_i128,8613863538586260381842683804190188097_i128,(-164236404746296036734294890864216108207_i128),33090374722188782482577583430322424835_i128,157304921466770789778648093939512244225_i128,139293789789497627411064448450895134744_i128,9439331507259539478912820492374592214_i128];
RET = [(-136520434567250601805829734242349667644_i128),(-75550121541582129997013860045086795583_i128),(-122304207682353041120461652081647807191_i128),65608199407068841869052493397505759708_i128,154657548731081820696357849476694015207_i128,121485057724057739892341444451382197850_i128,92047675710448074972719429699517859798_i128];
RET = [(-10804994011435867959387607998848568651_i128),(-70346866783185611250473974803621518858_i128),49642106477541773926885244428326478586_i128,111008614093725108045162589927267573097_i128,(-149386536709835016537066760528971407368_i128),(-114290704351929627085385958844256140613_i128),93234985077834908814005564653202749755_i128];
_4 = '\u{e39b}';
Goto(bb7)
}
bb7 = {
RET = [(-92592379215396180607150063477782756654_i128),(-120965780342755934866904069055412144934_i128),(-166439216470354915733298436523263665262_i128),14953039283877966987865959716583547806_i128,22641862596978998256223439840222250085_i128,146743982997770172831567458523996240880_i128,(-137578467463581322803012497468414763964_i128)];
RET = [(-137761083982599833018680293448821166869_i128),16214669798182520149120464292891722766_i128,127252655383326769631056279289918289115_i128,(-132134993851434775666212836431704699151_i128),103484345106262552853662941642200921270_i128,49059393663773627059876768332784496167_i128,10807661135309143078144865397335051184_i128];
_8.0.1 = core::ptr::addr_of_mut!(_3);
_3 = 487102927_i32;
RET = [(-26657864911557504245187073155913302122_i128),(-126998887024180332936368333242722393647_i128),(-85028559456750886903748648788989178197_i128),(-164426932433735666798619828321743750532_i128),48795319488867824193629225349897473846_i128,75136191297829363804133319274190641161_i128,(-93893490642120474475943132947040055637_i128)];
_10 = 11569183760930338943_usize as f32;
_8.0.1 = core::ptr::addr_of_mut!(_9);
_13 = !9223372036854775807_isize;
_8.2 = core::ptr::addr_of_mut!(_12);
_8.0.1 = core::ptr::addr_of_mut!(_12);
_8.0.0 = 20279_u16 as f64;
_11 = 3355_i16 ^ (-4038_i16);
_10 = 234881824_u32 as f32;
_9 = 28446_u16 as i32;
_10 = 5_usize as f32;
_12 = _3 << _9;
_11 = 6_usize as i16;
_3 = !_12;
_12 = _3;
Call(_15 = fn6(_13, _2, RET, _10, RET, _2, _8.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
SetDiscriminant(_15, 0);
_8.1 = core::ptr::addr_of_mut!(_17);
_11 = (-29528_i16);
_17 = false;
_13 = 53_isize;
_4 = '\u{66117}';
_4 = '\u{f22e9}';
_17 = true;
_1 = !115_u8;
_8.1 = core::ptr::addr_of_mut!(_17);
_8.2 = _8.0.1;
_16 = (RET,);
RET = _16.0;
_3 = _12;
RET = [(-95541863865169671021674528986013415325_i128),(-38422880570430915914278928501978229739_i128),(-139018314519201414880270011886549092707_i128),10063526466294773179908390782125652525_i128,11104489433294318193751031663267865761_i128,(-1282920446038995840583950166964779420_i128),147801469065902971853982793150142864678_i128];
_12 = -_3;
_8.2 = core::ptr::addr_of_mut!(_3);
_3 = _12;
_16 = (RET,);
_3 = _12;
_8.2 = core::ptr::addr_of_mut!(_12);
_8.0.1 = core::ptr::addr_of_mut!(_12);
_8.0.1 = core::ptr::addr_of_mut!(_3);
Goto(bb9)
}
bb9 = {
_19 = 4_usize;
_8.2 = core::ptr::addr_of_mut!(_9);
_13 = 9223372036854775807_isize & 9223372036854775807_isize;
_16 = (RET,);
_8.0.0 = 223348857539701551_u64 as f64;
_2 = core::ptr::addr_of!(RET[_19]);
_20 = _13;
_4 = '\u{10fdab}';
RET = _16.0;
_17 = _3 != _3;
_16.0[_19] = -(*_2);
_4 = '\u{d7773}';
_12 = _3;
match RET[_19] {
0 => bb7,
1 => bb2,
11104489433294318193751031663267865761 => bb10,
_ => bb3
}
}
bb10 = {
_1 = 129_u8;
_17 = true;
_19 = !6_usize;
_8.0.1 = _8.2;
_21 = (_16.0,);
_16 = _21;
_22 = _20 >> _12;
_8.0.0 = (*_2) as f64;
_17 = !false;
_13 = -_22;
_11 = _4 as i16;
_8.2 = _8.0.1;
_13 = _22;
_22 = _11 as isize;
_24.0 = _1 | _1;
_20 = _24.0 as isize;
_10 = _8.0.0 as f32;
_24.2 = !_17;
_24.0 = _1 >> _13;
_16 = (RET,);
Goto(bb11)
}
bb11 = {
_10 = _11 as f32;
_4 = '\u{84516}';
_16.0 = [(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2)];
_24 = (_1, _12, _17);
_22 = _19 as isize;
_3 = _24.1;
_16.0 = [(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2)];
RET = [152465622648554678998460169606736597653_i128,2128124885394113886715070730042807654_i128,87866495886956672068801180382489928973_i128,(-65918423167845918281690909206113789385_i128),(-25402800536596269355265214900421104900_i128),153600519803243258755225601973029392651_i128,(-33335697268357488153521447664415497587_i128)];
_20 = _13 ^ _13;
_16.0 = [(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2)];
_20 = _13;
_25 = (*_2);
_17 = _24.2;
_24 = (_1, _3, _17);
_23.0.0.0 = _8.0.0 * _8.0.0;
_2 = core::ptr::addr_of!((*_2));
_8.2 = core::ptr::addr_of_mut!(_12);
_1 = _24.0;
_23.0.0.1 = core::ptr::addr_of_mut!(_12);
_4 = '\u{20cdc}';
_24 = (_1, _3, _17);
_16 = (_21.0,);
RET = [_25,_25,_25,_25,_25,_25,_25];
_21 = (_16.0,);
(*_2) = _25 & _25;
_23.0.0 = (_8.0.0, _8.2);
Goto(bb12)
}
bb12 = {
Call(_28 = dump_var(5_usize, 22_usize, Move(_22), 19_usize, Move(_19), 24_usize, Move(_24), 17_usize, Move(_17)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_28 = dump_var(5_usize, 1_usize, Move(_1), 16_usize, Move(_16), 21_usize, Move(_21), 29_usize, _29), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: *const i128,mut _3: [i128; 7],mut _4: f32,mut _5: [i128; 7],mut _6: *const i128,mut _7: *mut i32) -> Adt52 {
mir! {
type RET = Adt52;
let _8: f64;
let _9: [i8; 1];
let _10: ((f64, *mut i32), *mut bool, *mut i32);
let _11: Adt44;
let _12: (i32, i128);
let _13: [i32; 5];
let _14: (i32, i128);
let _15: u64;
let _16: [bool; 2];
let _17: isize;
let _18: u128;
let _19: ([i128; 7],);
let _20: [i128; 2];
let _21: Adt54;
let _22: Adt53;
let _23: isize;
let _24: [i32; 5];
let _25: i32;
let _26: *const (i32, i128);
let _27: [i8; 1];
let _28: ((f64, *mut i32),);
let _29: u32;
let _30: u8;
let _31: [i32; 5];
let _32: (i32, i128);
let _33: Adt46;
let _34: *const i128;
let _35: [i128; 2];
let _36: u8;
let _37: *mut u32;
let _38: u128;
let _39: [isize; 7];
let _40: Adt43;
let _41: (usize, ((f64, *mut i32), *mut bool, *mut i32), i64);
let _42: i128;
let _43: &'static f32;
let _44: Adt55;
let _45: Adt51;
let _46: [isize; 7];
let _47: Adt45;
let _48: u32;
let _49: [isize; 7];
let _50: u16;
let _51: [bool; 2];
let _52: i128;
let _53: ([i128; 7],);
let _54: [bool; 2];
let _55: [bool; 2];
let _56: Adt58;
let _57: (i32, i128);
let _58: f32;
let _59: Adt50;
let _60: f32;
let _61: [bool; 2];
let _62: [i128; 2];
let _63: u8;
let _64: Adt46;
let _65: i8;
let _66: bool;
let _67: (u8, i32, bool);
let _68: f32;
let _69: f32;
let _70: [i8; 1];
let _71: [isize; 7];
let _72: [isize; 7];
let _73: Adt59;
let _74: char;
let _75: (usize, ((f64, *mut i32), *mut bool, *mut i32), i64);
let _76: (u8, i32, bool);
let _77: (u8, i32, bool);
let _78: u16;
let _79: usize;
let _80: (i32, i128);
let _81: f64;
let _82: [i128; 2];
let _83: [i32; 5];
let _84: [bool; 2];
let _85: [i128; 2];
let _86: [i128; 2];
let _87: Adt43;
let _88: [i32; 5];
let _89: ();
let _90: ();
{
_2 = _6;
(*_7) = !(-474858665_i32);
_5 = [3358897786954651328141185779582643122_i128,112224220189840838450860323564652894454_i128,(-98430957820513963712303742363179136093_i128),(-66619664605360461994344463546656592551_i128),(-11896946898605224307194968096175518870_i128),(-42640249327476295562669053873033921395_i128),27626923779042219739073991127084321521_i128];
_3 = [117688338547809152633639543996093101497_i128,85214598378077799035931652996272300432_i128,(-140516630540757554265528137731755006134_i128),(-55659382565295858039051621089922190920_i128),87925635148064559165948150865882537650_i128,(-71864982349946791729148324890022203139_i128),38554565962407250193139715333699797950_i128];
_9 = [63_i8];
_10.0.1 = core::ptr::addr_of_mut!((*_7));
_4 = 41385_u16 as f32;
_7 = core::ptr::addr_of_mut!((*_7));
_3 = [(-82856588728786089484662201238178745387_i128),43969823191828635075223298103585605188_i128,168685108702928500221342351814724927737_i128,8630982210896528370655813793769383208_i128,99180694665599742423766752023319249364_i128,72058932435248754985773940026929997117_i128,(-107245416548965013882170939914360820092_i128)];
_3 = [94796338453868758853385625870325565232_i128,(-11932071918634686975213232839257361695_i128),(-87478157217381177946575593181874985807_i128),69901100141052193359972218199942448451_i128,125860218170161113280048511390927587498_i128,24367062577889734285384041291854912428_i128,(-152316365893983409647825415456460002021_i128)];
_9 = [31_i8];
_1 = 49014435543170971620594775941536230613_u128 as isize;
_6 = _2;
_10.2 = core::ptr::addr_of_mut!((*_7));
_9 = [(-74_i8)];
Goto(bb1)
}
bb1 = {
_10.0.0 = 56_u8 as f64;
(*_7) = 328472031290152214721414178366326388519_u128 as i32;
_12 = ((*_7), (-61016412002527913141214136643762446548_i128));
_5 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_5 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_4 = (-3_i8) as f32;
_12.1 = !29395424855465137872214057759349616939_i128;
_10.2 = _7;
_12.0 = -(*_7);
(*_7) = _12.0 | _12.0;
_11 = Adt44::Variant2 { fld0: 890472891145168419_i64,fld1: (-2293_i16),fld2: _12.0,fld3: _10.0 };
(*_7) = Field::<i32>(Variant(_11, 2), 2) << _12.1;
place!(Field::<i32>(Variant(_11, 2), 2)) = 17319931625030177629_usize as i32;
place!(Field::<i64>(Variant(_11, 2), 0)) = _1 as i64;
_10.0 = (Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0, _10.2);
_9 = [(-42_i8)];
_12 = ((*_7), 76815153937616648228830077446431310830_i128);
_10.0 = (Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0, _10.2);
_1 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_11, 2), 1)) = !(-15927_i16);
_12.0 = Field::<i32>(Variant(_11, 2), 2) & Field::<i32>(Variant(_11, 2), 2);
_15 = !2212966120095595745_u64;
_10.0.0 = Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0;
place!(Field::<i16>(Variant(_11, 2), 1)) = 9177_i16;
_14.1 = !_12.1;
Goto(bb2)
}
bb2 = {
_17 = _1 << _1;
_16 = [true,false];
_13 = [_12.0,(*_7),(*_7),(*_7),(*_7)];
_12.0 = (*_7) - Field::<i32>(Variant(_11, 2), 2);
_18 = 167195334715274242350608362749682780321_u128;
_14.0 = 143_u8 as i32;
_2 = core::ptr::addr_of!(_14.1);
_8 = _10.0.0 * _10.0.0;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = core::ptr::addr_of_mut!(_14.0);
match Field::<i16>(Variant(_11, 2), 1) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9177 => bb9,
_ => bb8
}
}
bb3 = {
_10.0.0 = 56_u8 as f64;
(*_7) = 328472031290152214721414178366326388519_u128 as i32;
_12 = ((*_7), (-61016412002527913141214136643762446548_i128));
_5 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_5 = [_12.1,_12.1,_12.1,_12.1,_12.1,_12.1,_12.1];
_4 = (-3_i8) as f32;
_12.1 = !29395424855465137872214057759349616939_i128;
_10.2 = _7;
_12.0 = -(*_7);
(*_7) = _12.0 | _12.0;
_11 = Adt44::Variant2 { fld0: 890472891145168419_i64,fld1: (-2293_i16),fld2: _12.0,fld3: _10.0 };
(*_7) = Field::<i32>(Variant(_11, 2), 2) << _12.1;
place!(Field::<i32>(Variant(_11, 2), 2)) = 17319931625030177629_usize as i32;
place!(Field::<i64>(Variant(_11, 2), 0)) = _1 as i64;
_10.0 = (Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0, _10.2);
_9 = [(-42_i8)];
_12 = ((*_7), 76815153937616648228830077446431310830_i128);
_10.0 = (Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0, _10.2);
_1 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_11, 2), 1)) = !(-15927_i16);
_12.0 = Field::<i32>(Variant(_11, 2), 2) & Field::<i32>(Variant(_11, 2), 2);
_15 = !2212966120095595745_u64;
_10.0.0 = Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0;
place!(Field::<i16>(Variant(_11, 2), 1)) = 9177_i16;
_14.1 = !_12.1;
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
(*_2) = _12.1;
_12.0 = (*_7) & _14.0;
place!(Field::<i32>(Variant(_11, 2), 2)) = _12.0 * (*_7);
_20 = [(*_2),(*_2)];
_19.0 = [_12.1,(*_2),(*_2),(*_2),(*_2),_12.1,_12.1];
_14.0 = Field::<i32>(Variant(_11, 2), 2) >> _14.1;
_10.0.1 = core::ptr::addr_of_mut!((*_7));
_12.1 = (*_2);
_10.0.1 = core::ptr::addr_of_mut!((*_7));
_13 = [(*_7),_14.0,Field::<i32>(Variant(_11, 2), 2),Field::<i32>(Variant(_11, 2), 2),(*_7)];
_19.0 = [_14.1,(*_2),_12.1,_14.1,_12.1,_12.1,_14.1];
_16 = [true,false];
_2 = core::ptr::addr_of!(_14.1);
_12 = (Field::<i32>(Variant(_11, 2), 2), _14.1);
_14.1 = _12.1 - _12.1;
_6 = _2;
match _18 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
167195334715274242350608362749682780321 => bb10,
_ => bb8
}
}
bb10 = {
_3 = [(*_2),(*_2),(*_2),(*_2),_14.1,_12.1,_12.1];
_19 = (_5,);
_16 = [false,false];
_23 = _1;
_19 = (_5,);
_14 = ((*_7), _12.1);
_6 = core::ptr::addr_of!((*_2));
_15 = 6306137470147531355_u64 * 564996987807608228_u64;
_3 = _19.0;
_6 = _2;
_7 = core::ptr::addr_of_mut!(_14.0);
_24 = [(*_7),_14.0,_12.0,(*_7),_12.0];
(*_2) = _12.1 | _12.1;
_10.0.0 = 46547_u16 as f64;
(*_7) = _12.0;
(*_6) = -_12.1;
Goto(bb11)
}
bb11 = {
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = core::ptr::addr_of_mut!((*_7));
place!(Field::<i64>(Variant(_11, 2), 0)) = 256_u16 as i64;
_7 = _10.2;
_10.2 = Field::<(f64, *mut i32)>(Variant(_11, 2), 3).1;
place!(Field::<i64>(Variant(_11, 2), 0)) = 124_u8 as i64;
match _18 {
0 => bb9,
167195334715274242350608362749682780321 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_6 = core::ptr::addr_of!((*_2));
_15 = 6743059505025975986_u64 & 1129264057194968240_u64;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)) = (_10.0.0, _10.2);
_10.0.1 = _7;
SetDiscriminant(_11, 2);
_15 = !2316994832311489367_u64;
_19.0 = _3;
Call(place!(Field::<i64>(Variant(_11, 2), 0)) = core::intrinsics::transmute(_17), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_7) = _12.0 >> (*_6);
(*_7) = _14.0 & _14.0;
(*_2) = (*_7) as i128;
_15 = 8227830610837179464_u64;
_28 = (_10.0,);
_14 = _12;
_23 = _17;
_17 = _1 & _23;
_17 = _23 << (*_7);
_14.1 = _12.1;
_28.0.0 = _8;
(*_2) = '\u{34e6}' as i128;
_32.0 = !(*_7);
_28.0.0 = _8 * _10.0.0;
_1 = false as isize;
Call(_30 = fn7(_12.0, _7, _12.1, _28, _12.0, _28, _28.0.1, _14.0, Field::<i64>(Variant(_11, 2), 0), _17, (*_7), _28.0.1, _10.0.1, _7, _17, _12), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_14.0 = (*_7) & (*_7);
place!(Field::<i32>(Variant(_11, 2), 2)) = _4 as i32;
_37 = core::ptr::addr_of_mut!(_29);
_14.0 = (*_7);
_7 = _28.0.1;
Call(place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = fn12((*_7), _30), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_28.0.1 = _10.0.1;
(*_37) = _30 as u32;
(*_7) = _14.0;
_24 = _13;
_35 = [_14.1,_12.1];
_32 = (_14.0, (*_6));
_28.0.0 = _10.0.0 + _8;
_27 = [(-52_i8)];
_12.0 = _14.0 * (*_7);
_40 = Adt43::Variant0 { fld0: false,fld1: _18,fld2: _35 };
_12.0 = (*_7) - (*_7);
_41.1.0.1 = core::ptr::addr_of_mut!(_14.0);
(*_2) = _12.1 * _12.1;
_41.2 = Field::<i64>(Variant(_11, 2), 0);
_42 = -(*_6);
_39 = [_17,_23,_23,_17,_17,_1,_23];
_13 = _24;
_12 = (_14.0, (*_6));
Goto(bb17)
}
bb17 = {
_31 = [_12.0,(*_7),_32.0,_14.0,_14.0];
(*_2) = _18 as i128;
(*_2) = _28.0.0 as i128;
place!(Field::<bool>(Variant(_40, 0), 0)) = !false;
_30 = 169_u8 - 104_u8;
_41.2 = -Field::<i64>(Variant(_11, 2), 0);
_25 = !_32.0;
_34 = _2;
(*_2) = !_12.1;
_36 = !_30;
SetDiscriminant(_40, 0);
(*_34) = _42;
(*_37) = 1921212129_u32 << (*_7);
_27 = _9;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).0 = _28.0.0;
_12 = _32;
_40 = Adt43::Variant1 { fld0: _28.0.0,fld1: _37,fld2: _4,fld3: _34,fld4: _12,fld5: _12.0,fld6: _18,fld7: _31 };
_14 = ((*_7), _42);
_47 = Adt45::Variant0 { fld0: _29,fld1: _28.0 };
_35 = [(*_2),(*_34)];
_14.1 = _42 << Field::<(i32, i128)>(Variant(_40, 1), 4).0;
_41.1.0.0 = _8 * _8;
_41.1.2 = _41.1.0.1;
match Field::<u128>(Variant(_40, 1), 6) {
0 => bb10,
1 => bb9,
2 => bb13,
3 => bb14,
4 => bb16,
5 => bb18,
167195334715274242350608362749682780321 => bb20,
_ => bb19
}
}
bb18 = {
_17 = _1 << _1;
_16 = [true,false];
_13 = [_12.0,(*_7),(*_7),(*_7),(*_7)];
_12.0 = (*_7) - Field::<i32>(Variant(_11, 2), 2);
_18 = 167195334715274242350608362749682780321_u128;
_14.0 = 143_u8 as i32;
_2 = core::ptr::addr_of!(_14.1);
_8 = _10.0.0 * _10.0.0;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = core::ptr::addr_of_mut!(_14.0);
match Field::<i16>(Variant(_11, 2), 1) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9177 => bb9,
_ => bb8
}
}
bb19 = {
(*_7) = _12.0 >> (*_6);
(*_7) = _14.0 & _14.0;
(*_2) = (*_7) as i128;
_15 = 8227830610837179464_u64;
_28 = (_10.0,);
_14 = _12;
_23 = _17;
_17 = _1 & _23;
_17 = _23 << (*_7);
_14.1 = _12.1;
_28.0.0 = _8;
(*_2) = '\u{34e6}' as i128;
_32.0 = !(*_7);
_28.0.0 = _8 * _10.0.0;
_1 = false as isize;
Call(_30 = fn7(_12.0, _7, _12.1, _28, _12.0, _28, _28.0.1, _14.0, Field::<i64>(Variant(_11, 2), 0), _17, (*_7), _28.0.1, _10.0.1, _7, _17, _12), ReturnTo(bb15), UnwindUnreachable())
}
bb20 = {
(*_2) = _42;
SetDiscriminant(_47, 1);
place!(Field::<i32>(Variant(_11, 2), 2)) = -_12.0;
_32.1 = _17 as i128;
_15 = 11922595685063452290_u64 & 1520355659930323030_u64;
_3 = [_14.1,_32.1,_42,(*_6),_32.1,_32.1,(*_6)];
_24 = [_12.0,(*_7),_32.0,_32.0,_14.0];
_20 = _35;
Call(place!(Field::<i8>(Variant(_47, 1), 3)) = core::intrinsics::transmute(_9), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
SetDiscriminant(_40, 1);
_3 = [_42,_32.1,(*_34),(*_6),(*_6),_32.1,_14.1];
place!(Field::<i16>(Variant(_11, 2), 1)) = 5236_i16;
_41.0 = !4020018597809333808_usize;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).0 = -_8;
(*_37) = 3905197290_u32;
place!(Field::<i16>(Variant(_11, 2), 1)) = -28215_i16;
place!(Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1)).0.0 = _15 as i32;
place!(Field::<i32>(Variant(_40, 1), 5)) = _32.0 | _25;
place!(Field::<*mut u32>(Variant(_40, 1), 1)) = core::ptr::addr_of_mut!(_29);
Goto(bb22)
}
bb22 = {
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = core::ptr::addr_of_mut!(place!(Field::<(i32, i128)>(Variant(_40, 1), 4)).0);
place!(Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1)).1 = core::ptr::addr_of!((*_7));
_41.1.0.0 = -_28.0.0;
place!(Field::<*const i128>(Variant(_40, 1), 3)) = core::ptr::addr_of!((*_6));
_33 = Adt46::Variant0 { fld0: _15,fld1: _39,fld2: Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1).1 };
_40 = Adt43::Variant0 { fld0: true,fld1: _18,fld2: _20 };
place!(Field::<i16>(Variant(_11, 2), 1)) = (-22412_i16) << Field::<i32>(Variant(_11, 2), 2);
SetDiscriminant(_33, 2);
(*_6) = _32.1 ^ _32.1;
_40 = Adt43::Variant1 { fld0: _28.0.0,fld1: _37,fld2: _4,fld3: _6,fld4: _32,fld5: _12.0,fld6: _18,fld7: _24 };
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).0 = _28.0.0;
SetDiscriminant(_40, 0);
match _29 {
0 => bb10,
3905197290 => bb23,
_ => bb19
}
}
bb23 = {
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)) = (_28.0.0, _41.1.0.1);
_14.0 = -_32.0;
_29 = !2262618179_u32;
_55 = [true,false];
_16 = _55;
_7 = _10.2;
place!(Field::<[isize; 7]>(Variant(_33, 2), 0)) = [_23,_17,_17,_17,_17,_23,_23];
_50 = 46435_u16 << (*_6);
_52 = (*_37) as i128;
_40 = Adt43::Variant0 { fld0: false,fld1: _18,fld2: _35 };
_40 = Adt43::Variant1 { fld0: Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0,fld1: _37,fld2: _4,fld3: _2,fld4: _14,fld5: _32.0,fld6: _18,fld7: _24 };
_6 = _2;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)) = _41.1.0;
place!(Field::<(i32, i128)>(Variant(_40, 1), 4)) = _14;
match Field::<u128>(Variant(_40, 1), 6) {
0 => bb8,
1 => bb2,
2 => bb21,
3 => bb17,
4 => bb10,
167195334715274242350608362749682780321 => bb24,
_ => bb6
}
}
bb24 = {
place!(Field::<i64>(Variant(_11, 2), 0)) = _41.2 + _41.2;
_10.2 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11, 2), 2)));
_39 = Field::<[isize; 7]>(Variant(_33, 2), 0);
place!(Field::<*const i128>(Variant(_40, 1), 3)) = core::ptr::addr_of!(_52);
_12 = (_14.0, _14.1);
_53.0 = _3;
_54 = _55;
_32 = (_25, (*_2));
_41.1.0.0 = (*_37) as f64;
_14.1 = Field::<(i32, i128)>(Variant(_40, 1), 4).1 >> Field::<i32>(Variant(_11, 2), 2);
_59.fld2 = core::ptr::addr_of_mut!((*_7));
_48 = (*_37) >> (*_7);
_28.0.1 = Field::<(f64, *mut i32)>(Variant(_11, 2), 3).1;
Call(_6 = fn17(_24, Move(_11), _10.2, _41.1.2, (*_34), _24, _12.0, (*_7), Move(_40), (*_6), _24, _14, _59.fld2, _59.fld2, _2), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_30 = _36;
_57.1 = _18 as i128;
_49 = Field::<[isize; 7]>(Variant(_33, 2), 0);
_55 = [false,false];
place!(Field::<Adt45>(Variant(_33, 2), 1)) = Adt45::Variant0 { fld0: _48,fld1: _41.1.0 };
_25 = _14.0;
place!(Field::<f32>(Variant(_47, 1), 0)) = _4;
_24 = [_25,_32.0,(*_7),_14.0,_12.0];
_17 = !_1;
_4 = Field::<f32>(Variant(_47, 1), 0) - Field::<f32>(Variant(_47, 1), 0);
_62 = _35;
_3 = [(*_6),(*_6),_14.1,(*_34),(*_6),(*_34),(*_6)];
_4 = Field::<f32>(Variant(_47, 1), 0) * Field::<f32>(Variant(_47, 1), 0);
SetDiscriminant(Field::<Adt45>(Variant(_33, 2), 1), 1);
_41.1.0.0 = _10.0.0 + _28.0.0;
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).0.0 = _28.0.0 as i32;
_59.fld0 = -Field::<i8>(Variant(_47, 1), 3);
place!(Field::<f32>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 0)) = -_4;
_7 = _59.fld2;
Goto(bb26)
}
bb26 = {
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).1 = Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1).1;
_28.0.0 = _41.0 as f64;
_11 = Adt44::Variant2 { fld0: _41.2,fld1: (-6686_i16),fld2: _32.0,fld3: _28.0 };
_49 = [_23,_23,_23,_23,_1,_17,_1];
Goto(bb27)
}
bb27 = {
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).0 = _32;
_28.0 = (Field::<(f64, *mut i32)>(Variant(_11, 2), 3).0, _41.1.2);
_16 = _54;
_59 = Adt50 { fld0: Field::<i8>(Variant(_47, 1), 3),fld1: '\u{376ae}',fld2: _41.1.0.1 };
_35 = _20;
place!(Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1)).0.1 = (*_2) * _14.1;
_53.0 = [(*_2),(*_6),_12.1,Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1).0.1,_14.1,(*_6),(*_2)];
_57 = ((*_7), (*_2));
_61 = _54;
match _18 {
0 => bb15,
1 => bb2,
2 => bb28,
167195334715274242350608362749682780321 => bb30,
_ => bb29
}
}
bb28 = {
(*_7) = _12.0 >> (*_6);
(*_7) = _14.0 & _14.0;
(*_2) = (*_7) as i128;
_15 = 8227830610837179464_u64;
_28 = (_10.0,);
_14 = _12;
_23 = _17;
_17 = _1 & _23;
_17 = _23 << (*_7);
_14.1 = _12.1;
_28.0.0 = _8;
(*_2) = '\u{34e6}' as i128;
_32.0 = !(*_7);
_28.0.0 = _8 * _10.0.0;
_1 = false as isize;
Call(_30 = fn7(_12.0, _7, _12.1, _28, _12.0, _28, _28.0.1, _14.0, Field::<i64>(Variant(_11, 2), 0), _17, (*_7), _28.0.1, _10.0.1, _7, _17, _12), ReturnTo(bb15), UnwindUnreachable())
}
bb29 = {
_14.0 = (*_7) & (*_7);
place!(Field::<i32>(Variant(_11, 2), 2)) = _4 as i32;
_37 = core::ptr::addr_of_mut!(_29);
_14.0 = (*_7);
_7 = _28.0.1;
Call(place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = fn12((*_7), _30), ReturnTo(bb16), UnwindUnreachable())
}
bb30 = {
_44 = Adt55::Variant1 { fld0: _57.1 };
_55 = [true,false];
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).0 = (_32.0, (*_2));
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).0 = (_14.0, (*_34));
place!(Field::<f32>(Variant(_47, 1), 0)) = -Field::<f32>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 0);
_60 = Field::<f32>(Variant(_47, 1), 0) - Field::<f32>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 0);
_13 = [_32.0,_57.0,(*_7),_57.0,Field::<i32>(Variant(_11, 2), 2)];
_64 = Adt46::Variant1 { fld0: _48 };
_14 = (Field::<i32>(Variant(_11, 2), 2), _57.1);
_48 = Field::<u32>(Variant(_64, 1), 0) & Field::<u32>(Variant(_64, 1), 0);
_59.fld1 = '\u{d699c}';
_19.0 = _3;
_41.1.1 = core::ptr::addr_of_mut!(_67.2);
_19.0 = [(*_6),(*_6),(*_34),(*_2),_14.1,_14.1,Field::<i128>(Variant(_44, 1), 0)];
_53 = _19;
_7 = _41.1.2;
_39 = Field::<[isize; 7]>(Variant(_33, 2), 0);
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)) = (_10.0.0, _41.1.0.1);
_10.2 = core::ptr::addr_of_mut!((*_7));
(*_34) = _41.0 as i128;
match _18 {
0 => bb4,
1 => bb11,
167195334715274242350608362749682780321 => bb32,
_ => bb31
}
}
bb31 = {
_17 = _1 << _1;
_16 = [true,false];
_13 = [_12.0,(*_7),(*_7),(*_7),(*_7)];
_12.0 = (*_7) - Field::<i32>(Variant(_11, 2), 2);
_18 = 167195334715274242350608362749682780321_u128;
_14.0 = 143_u8 as i32;
_2 = core::ptr::addr_of!(_14.1);
_8 = _10.0.0 * _10.0.0;
place!(Field::<(f64, *mut i32)>(Variant(_11, 2), 3)).1 = core::ptr::addr_of_mut!(_14.0);
match Field::<i16>(Variant(_11, 2), 1) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9177 => bb9,
_ => bb8
}
}
bb32 = {
(*_2) = -Field::<i128>(Variant(_44, 1), 0);
_57 = (_32.0, (*_6));
_75.0 = _41.0 >> _48;
_18 = 48215048435862862240857389038365719654_u128 >> _32.0;
_26 = core::ptr::addr_of!(_12);
_10.0.1 = core::ptr::addr_of_mut!((*_7));
_67.1 = _36 as i32;
place!(Field::<u32>(Variant(_64, 1), 0)) = _18 as u32;
_68 = -Field::<f32>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 0);
_67.2 = false | true;
_49 = [_23,_23,_23,_23,_23,_17,_23];
_47 = Adt45::Variant1 { fld0: Field::<f32>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 0),fld1: Field::<((i32, i128), *const i32)>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 1),fld2: Field::<((i32, i128), *const i32)>(Variant(Field::<Adt45>(Variant(_33, 2), 1), 1), 1).1,fld3: _59.fld0 };
_36 = _67.2 as u8;
_32.1 = !Field::<i128>(Variant(_44, 1), 0);
place!(Field::<((i32, i128), *const i32)>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 1), 1)).0.1 = !Field::<i128>(Variant(_44, 1), 0);
_41.1.1 = core::ptr::addr_of_mut!(_66);
_67.0 = !_30;
_11 = Adt44::Variant3 { fld0: _9,fld1: Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1),fld2: _62,fld3: _41.1,fld4: Field::<[isize; 7]>(Variant(_33, 2), 0) };
_10.1 = core::ptr::addr_of_mut!(_66);
_13 = _24;
_59.fld0 = -Field::<i8>(Variant(_47, 1), 3);
_10 = _41.1;
(*_37) = _48 + Field::<u32>(Variant(_64, 1), 0);
_59.fld1 = '\u{10203d}';
place!(Field::<((i32, i128), *const i32)>(Variant(_11, 3), 1)).0.1 = !(*_34);
place!(Field::<Adt45>(Variant(_33, 2), 1)) = Adt45::Variant0 { fld0: _29,fld1: _10.0 };
Goto(bb33)
}
bb33 = {
(*_37) = !_48;
_14.1 = _18 as i128;
_59.fld1 = '\u{2031a}';
_41 = (_75.0, _10, 4194155029866618617_i64);
_79 = !_75.0;
SetDiscriminant(_44, 3);
_41 = (_79, Field::<((f64, *mut i32), *mut bool, *mut i32)>(Variant(_11, 3), 3), (-3903830944212661664_i64));
_24 = _31;
place!(Field::<i8>(Variant(_47, 1), 3)) = _59.fld0;
_25 = !(*_26).0;
SetDiscriminant(_64, 0);
_23 = _1;
place!(Field::<(((f64, *mut i32),),)>(Variant(_44, 3), 1)).0.0.0 = -_10.0.0;
_38 = _18;
Goto(bb34)
}
bb34 = {
_77.1 = (*_7) - _25;
place!(Field::<u32>(Variant(place!(Field::<Adt45>(Variant(_33, 2), 1)), 0), 0)) = _29;
_14.0 = _25 >> (*_37);
(*_26) = (_14.0, Field::<((i32, i128), *const i32)>(Variant(_11, 3), 1).0.1);
_39 = [_23,_23,_17,_1,_17,_1,_17];
_81 = _15 as f64;
_59.fld0 = !Field::<i8>(Variant(_47, 1), 3);
_43 = &place!(Field::<f32>(Variant(_33, 2), 2));
place!(Field::<((f64, *mut i32), *mut bool, *mut i32)>(Variant(_44, 3), 0)).2 = Field::<((f64, *mut i32), *mut bool, *mut i32)>(Variant(_11, 3), 3).0.1;
RET = Adt52::Variant2 { fld0: Field::<[isize; 7]>(Variant(_33, 2), 0),fld1: _36,fld2: _20,fld3: _3 };
place!(Field::<((i32, i128), *const i32)>(Variant(_47, 1), 1)) = Field::<((i32, i128), *const i32)>(Variant(_11, 3), 1);
_12.0 = _14.0 << _12.1;
_15 = !8433716355689679022_u64;
_53 = (_19.0,);
_67.1 = 12618_i16 as i32;
_67 = (_36, _57.0, true);
_41 = (_79, _10, 7866584172461486350_i64);
_36 = _30 << (*_7);
place!(Field::<(((f64, *mut i32),),)>(Variant(_44, 3), 1)).0.0.0 = _8 * _8;
_76.1 = (*_37) as i32;
Goto(bb35)
}
bb35 = {
Call(_89 = dump_var(6_usize, 25_usize, Move(_25), 18_usize, Move(_18), 23_usize, Move(_23), 17_usize, Move(_17)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_89 = dump_var(6_usize, 12_usize, Move(_12), 49_usize, Move(_49), 5_usize, Move(_5), 30_usize, Move(_30)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_89 = dump_var(6_usize, 39_usize, Move(_39), 61_usize, Move(_61), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_89 = dump_var(6_usize, 35_usize, Move(_35), 62_usize, Move(_62), 38_usize, Move(_38), 55_usize, Move(_55)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_89 = dump_var(6_usize, 36_usize, Move(_36), 79_usize, Move(_79), 15_usize, Move(_15), 90_usize, _90), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i32,mut _2: *mut i32,mut _3: i128,mut _4: ((f64, *mut i32),),mut _5: i32,mut _6: ((f64, *mut i32),),mut _7: *mut i32,mut _8: i32,mut _9: i64,mut _10: isize,mut _11: i32,mut _12: *mut i32,mut _13: *mut i32,mut _14: *mut i32,mut _15: isize,mut _16: (i32, i128)) -> u8 {
mir! {
type RET = u8;
let _17: [bool; 2];
let _18: [isize; 7];
let _19: u8;
let _20: [i8; 1];
let _21: Adt48;
let _22: (i32, i128);
let _23: f64;
let _24: isize;
let _25: ([i128; 7],);
let _26: isize;
let _27: char;
let _28: (u8, i8, f64, (u8, i32, bool), i16);
let _29: [i128; 2];
let _30: (i16, [i8; 1], char, *mut i32);
let _31: char;
let _32: [i128; 2];
let _33: f32;
let _34: [i32; 5];
let _35: (usize, ((f64, *mut i32), *mut bool, *mut i32), i64);
let _36: Adt47;
let _37: i32;
let _38: u8;
let _39: ();
let _40: ();
{
_1 = (*_13);
(*_7) = _1 * _11;
_16.1 = !_3;
RET = !130_u8;
Goto(bb1)
}
bb1 = {
_14 = _4.0.1;
_1 = (*_2);
_14 = _6.0.1;
(*_13) = 5358255373169205529_u64 as i32;
_16.0 = _6.0.0 as i32;
_9 = 567998493458685739_i64 * 747530400529148011_i64;
RET = _9 as u8;
_6.0.1 = core::ptr::addr_of_mut!((*_2));
_13 = core::ptr::addr_of_mut!((*_12));
_12 = core::ptr::addr_of_mut!((*_14));
_6.0 = (_4.0.0, _4.0.1);
_22.1 = _10 as i128;
_22 = (_11, _3);
_4.0.1 = _7;
_4.0.1 = _13;
_22.0 = _15 as i32;
_22 = (_8, _3);
_12 = core::ptr::addr_of_mut!((*_7));
_26 = -_15;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
76815153937616648228830077446431310830 => bb7,
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
(*_7) = _1 * _1;
_8 = (*_12) + (*_7);
RET = !239_u8;
_22.1 = -_16.1;
(*_2) = _8;
(*_2) = -_8;
(*_2) = _11;
_27 = '\u{35972}';
_17 = [true,false];
_25.0 = [_16.1,_3,_3,_3,_16.1,_16.1,_16.1];
_28.3.0 = !RET;
_9 = 2300937163721041905_i64 | (-4382462232458777707_i64);
_28.3.2 = !false;
_1 = (*_13);
Call(_28.0 = fn8((*_14), _6.0.1, _28.3.2, (*_14), _6.0.1, _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_28.1 = 34472010249320128255700062221172307211_u128 as i8;
_28.3.2 = _10 != _10;
_28.3 = (_28.0, _1, false);
(*_14) = _1;
Goto(bb9)
}
bb9 = {
_29 = [_3,_22.1];
_6 = (_4.0,);
_19 = !_28.3.0;
(*_14) = _3 as i32;
_28.2 = -_6.0.0;
(*_12) = -_8;
_6.0 = (_28.2, _4.0.1);
_5 = _1;
_10 = -_26;
_28.0 = !_28.3.0;
_8 = (*_14) + (*_13);
_4.0.0 = -_28.2;
(*_12) = _5;
_7 = core::ptr::addr_of_mut!(_22.0);
_19 = _9 as u8;
(*_14) = _8 * _1;
_28.3.2 = !false;
(*_7) = _28.0 as i32;
_6 = (_4.0,);
_3 = _22.1 ^ _22.1;
_18 = [_15,_26,_15,_26,_10,_26,_15];
_22.0 = !(*_2);
_28.3.0 = _28.0;
_28.3 = (_28.0, (*_12), true);
(*_14) = -_5;
_7 = _2;
_25.0 = [_3,_16.1,_3,_3,_16.1,_3,_3];
_20 = [_28.1];
_4 = (_6.0,);
Call(_24 = fn9(_13, _28.3, _6.0, _28.3.1, _6.0, _28.3.1, _8, _6.0.1, _6.0, _28.3.2, _18, _14, _22.0, _6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_14) = _22.0;
_25.0 = [_3,_3,_16.1,_3,_3,_22.1,_16.1];
_6.0.1 = _4.0.1;
(*_12) = _8 - _8;
_18 = [_15,_24,_15,_15,_24,_24,_24];
_22 = _16;
_25.0 = [_16.1,_16.1,_16.1,_3,_16.1,_22.1,_16.1];
_8 = _28.1 as i32;
_22.1 = !_3;
_4 = (_6.0,);
Goto(bb11)
}
bb11 = {
_4.0.1 = _14;
_24 = _15 << (*_14);
_1 = (*_12);
_28.4 = -29060_i16;
_30.1 = _20;
_30.2 = _27;
_6.0.1 = _13;
_6.0.0 = -_28.2;
_28.3.1 = _30.2 as i32;
_23 = _28.2 - _28.2;
_30.0 = -_28.4;
_3 = -_16.1;
_6.0.0 = _23;
_6.0.0 = -_4.0.0;
_12 = core::ptr::addr_of_mut!(_16.0);
_16 = _22;
(*_7) = -_1;
_22.0 = (*_13) ^ (*_2);
_4.0.0 = -_23;
_14 = _13;
_26 = _24;
_1 = (*_14) << (*_2);
_18 = [_24,_26,_24,_26,_24,_26,_24];
(*_2) = !_22.0;
_19 = _28.3.2 as u8;
_30.0 = (*_2) as i16;
_4 = (_6.0,);
_23 = -_28.2;
Call(_28.4 = fn11(_13, _6.0, _4.0, (*_2), (*_7), _24, _6, _22, _14, _18, (*_14), _30.0, _22.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _3 as u8;
_26 = !_15;
_23 = _4.0.0 - _4.0.0;
_14 = _7;
_28.3.2 = false;
_19 = _28.3.0 | _28.0;
_8 = (*_7);
_30.3 = _7;
_29 = [_16.1,_16.1];
_28.4 = !_30.0;
_16 = _22;
_6 = _4;
_11 = _22.0 - (*_7);
_31 = _27;
_12 = _30.3;
_38 = _28.0;
_18 = [_24,_26,_24,_15,_15,_24,_15];
_28.0 = _19 * _38;
RET = _19;
_1 = (*_2) ^ (*_13);
Goto(bb13)
}
bb13 = {
Call(_39 = dump_var(7_usize, 17_usize, Move(_17), 25_usize, Move(_25), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_39 = dump_var(7_usize, 3_usize, Move(_3), 8_usize, Move(_8), 26_usize, Move(_26), 19_usize, Move(_19)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_39 = dump_var(7_usize, 27_usize, Move(_27), 31_usize, Move(_31), 40_usize, _40, 40_usize, _40), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i32,mut _2: *mut i32,mut _3: bool,mut _4: i32,mut _5: *mut i32,mut _6: *mut i32) -> u8 {
mir! {
type RET = u8;
let _7: u8;
let _8: bool;
let _9: i16;
let _10: (u8, i32, bool);
let _11: [i128; 2];
let _12: i128;
let _13: [bool; 2];
let _14: Adt51;
let _15: ([i128; 7],);
let _16: u32;
let _17: *const (i32, i128);
let _18: i16;
let _19: f64;
let _20: i32;
let _21: Adt50;
let _22: (u8, i8, f64, (u8, i32, bool), i16);
let _23: f64;
let _24: ();
let _25: ();
{
_1 = (*_6) + (*_5);
(*_5) = !_1;
_7 = !225_u8;
(*_2) = _4 >> _4;
_1 = 1917947227_u32 as i32;
(*_6) = 1118849224646390179_i64 as i32;
_2 = _5;
_6 = core::ptr::addr_of_mut!((*_2));
(*_5) = 24927_u16 as i32;
_8 = !_3;
_1 = (*_2) << _4;
(*_5) = !_1;
(*_5) = _4;
(*_6) = -_1;
_7 = 3_u8;
(*_2) = _1 >> _4;
_2 = _5;
(*_6) = _1 & _1;
_2 = core::ptr::addr_of_mut!((*_2));
(*_5) = 1853337514_u32 as i32;
(*_2) = _1 * _1;
_10 = (_7, (*_5), _3);
_10.0 = 1671789769_u32 as u8;
Goto(bb1)
}
bb1 = {
(*_2) = _1 ^ _4;
_9 = 19755_i16 * (-23352_i16);
(*_6) = 78047680146057225608475871842906440910_u128 as i32;
_6 = core::ptr::addr_of_mut!(_4);
_9 = (-6471_i16);
_11 = [(-17467066867367879953750103993250768575_i128),(-78179966730867350261418240938018771739_i128)];
_10 = (_7, (*_6), _3);
_1 = -(*_6);
(*_2) = _8 as i32;
_5 = _6;
_2 = _5;
_10.1 = _1 >> (*_6);
(*_2) = !_1;
RET = _3 as u8;
(*_2) = !_1;
_2 = _6;
(*_6) = !_10.1;
_10 = (_7, (*_5), _3);
_8 = !_3;
_3 = _10.2;
Call(_10.1 = core::intrinsics::bswap((*_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _6;
_10.2 = !_3;
_13 = [_3,_8];
_16 = _10.0 as u32;
(*_5) = 8641466996949049128_i64 as i32;
_10 = (_7, _1, _8);
_15.0 = [141933344990251070494514031883610367075_i128,77211955523670863021221762816958834576_i128,(-106491334333378158743611095574193076469_i128),123814337164317444007935331289816540571_i128,(-70418867142178997963492749084593327622_i128),(-33543513558358216412529238455440798973_i128),(-125393140572069435168830942717952904587_i128)];
_6 = core::ptr::addr_of_mut!(_4);
_4 = _10.1;
_2 = core::ptr::addr_of_mut!((*_2));
RET = _10.0 ^ _10.0;
_2 = core::ptr::addr_of_mut!((*_5));
_15.0 = [156402124964874178217463061926564283882_i128,41465407138472642154413179386320412735_i128,13381725556773046311344950052149272356_i128,(-75491154159489249527122390445686361105_i128),170034092480456400841957660883048277968_i128,55376157311351671731022497705738829107_i128,9868794077252998745807384049920503143_i128];
_10 = (RET, (*_6), _8);
_12 = 56480_u16 as i128;
_7 = _10.0 ^ RET;
_6 = _2;
(*_5) = _1 & _1;
(*_6) = _1;
RET = _7 & _7;
(*_2) = _10.1 | _10.1;
_18 = -_9;
_10.0 = RET << (*_6);
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768204985 => bb7,
_ => bb6
}
}
bb3 = {
(*_2) = _1 ^ _4;
_9 = 19755_i16 * (-23352_i16);
(*_6) = 78047680146057225608475871842906440910_u128 as i32;
_6 = core::ptr::addr_of_mut!(_4);
_9 = (-6471_i16);
_11 = [(-17467066867367879953750103993250768575_i128),(-78179966730867350261418240938018771739_i128)];
_10 = (_7, (*_6), _3);
_1 = -(*_6);
(*_2) = _8 as i32;
_5 = _6;
_2 = _5;
_10.1 = _1 >> (*_6);
(*_2) = !_1;
RET = _3 as u8;
(*_2) = !_1;
_2 = _6;
(*_6) = !_10.1;
_10 = (_7, (*_5), _3);
_8 = !_3;
_3 = _10.2;
Call(_10.1 = core::intrinsics::bswap((*_2)), ReturnTo(bb2), UnwindUnreachable())
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
RET = _10.0 & _7;
(*_6) = _10.1 ^ _1;
_10.2 = !_3;
_9 = -_18;
_10.2 = !_3;
RET = !_10.0;
(*_6) = -_1;
RET = !_10.0;
_19 = 4184000625482492921_i64 as f64;
_21.fld2 = core::ptr::addr_of_mut!(_4);
_15.0 = [_12,_12,_12,_12,_12,_12,_12];
_9 = _18;
_21.fld1 = '\u{9d876}';
_1 = (*_6);
RET = _10.0 ^ _10.0;
_15.0 = [_12,_12,_12,_12,_12,_12,_12];
(*_5) = _21.fld1 as i32;
_22.4 = 113_i8 as i16;
_21.fld0 = _21.fld1 as i8;
_14 = Adt51::Variant3 { fld0: _8,fld1: _10.0 };
_13 = [_8,_8];
_16 = !3514244174_u32;
_21 = Adt50 { fld0: (-61_i8),fld1: '\u{d0989}',fld2: _5 };
SetDiscriminant(_14, 0);
_22 = (RET, _21.fld0, _19, _10, _9);
(*_2) = _22.3.1 * _22.3.1;
Goto(bb8)
}
bb8 = {
Call(_24 = dump_var(8_usize, 16_usize, Move(_16), 10_usize, Move(_10), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_24 = dump_var(8_usize, 3_usize, Move(_3), 7_usize, Move(_7), 25_usize, _25, 25_usize, _25), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut i32,mut _2: (u8, i32, bool),mut _3: (f64, *mut i32),mut _4: i32,mut _5: (f64, *mut i32),mut _6: i32,mut _7: i32,mut _8: *mut i32,mut _9: (f64, *mut i32),mut _10: bool,mut _11: [isize; 7],mut _12: *mut i32,mut _13: i32,mut _14: ((f64, *mut i32),)) -> isize {
mir! {
type RET = isize;
let _15: char;
let _16: isize;
let _17: (u8, i32, bool);
let _18: *mut i32;
let _19: [i8; 1];
let _20: *const i128;
let _21: f64;
let _22: [i128; 2];
let _23: [i128; 7];
let _24: isize;
let _25: Adt57;
let _26: u8;
let _27: i32;
let _28: [i32; 5];
let _29: ();
let _30: ();
{
_3.0 = _9.0;
_15 = '\u{2de50}';
_2.2 = _10;
_9.0 = -_3.0;
RET = 9223372036854775807_isize;
_9.1 = _8;
_16 = !RET;
_10 = !_2.2;
_14.0 = _5;
_12 = core::ptr::addr_of_mut!((*_1));
_6 = -_13;
_7 = _3.0 as i32;
_9 = _5;
_14.0.1 = core::ptr::addr_of_mut!((*_12));
_12 = _1;
_12 = _14.0.1;
Call(_14.0 = fn10(_11, _2.2, _2.1, _2.2, _2.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_8) = _4;
_5.0 = _9.0;
_5.0 = _9.0 - _9.0;
_8 = core::ptr::addr_of_mut!((*_12));
_3.0 = -_5.0;
_2.1 = (*_1) * _4;
_13 = !_4;
_17.0 = !_2.0;
_11 = [RET,RET,_16,RET,RET,RET,RET];
_5 = (_3.0, _8);
_10 = !_2.2;
_2.2 = _10 <= _10;
_17.2 = _10;
_17.1 = _4 & (*_12);
(*_12) = -_17.1;
(*_8) = _17.1;
_3.0 = -_5.0;
_7 = (*_1) >> (*_1);
(*_8) = -_6;
_17.1 = _2.1 >> _6;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9223372036854775807 => bb9,
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
_9.0 = (*_8) as f64;
_17.1 = 55502_u16 as i32;
_9.1 = _1;
_14.0.0 = _9.0 + _9.0;
_11 = [_16,_16,RET,_16,_16,_16,_16];
_3 = (_14.0.0, _12);
_14.0.0 = _3.0;
(*_12) = -_4;
_2.2 = _17.2;
_2 = _17;
(*_1) = -_4;
RET = _16 ^ _16;
_17 = (_2.0, (*_12), _2.2);
(*_1) = !_17.1;
_17.2 = _2.2;
_16 = RET;
_17.0 = 49506_u16 as u8;
_5.0 = -_3.0;
_11 = [RET,RET,_16,_16,_16,_16,RET];
_22 = [28713636393130198876168396787907577399_i128,(-90341744160441154110158586118724392859_i128)];
_8 = _1;
_2.0 = !_17.0;
_17.1 = -(*_8);
Call(_3.0 = core::intrinsics::transmute(_16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = (_9.0, _1);
_3.1 = core::ptr::addr_of_mut!(_2.1);
_2.1 = !(*_8);
_14.0.1 = _12;
_3.0 = _9.0 * _14.0.0;
_14 = (_9,);
_17.0 = !_2.0;
_10 = !_17.2;
_8 = _9.1;
_2.2 = _10;
_9.0 = _14.0.0;
RET = _16 >> (*_12);
_21 = -_14.0.0;
_3.1 = core::ptr::addr_of_mut!((*_12));
_5.0 = (-63_i8) as f64;
_10 = _2.2;
_14.0.1 = _3.1;
_21 = -_3.0;
_28 = [(*_8),_7,(*_1),(*_8),_2.1];
_12 = _3.1;
Goto(bb11)
}
bb11 = {
Call(_29 = dump_var(9_usize, 10_usize, Move(_10), 2_usize, Move(_2), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_29 = dump_var(9_usize, 7_usize, Move(_7), 16_usize, Move(_16), 30_usize, _30, 30_usize, _30), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [isize; 7],mut _2: bool,mut _3: i32,mut _4: bool,mut _5: bool) -> (f64, *mut i32) {
mir! {
type RET = (f64, *mut i32);
let _6: ();
let _7: ();
{
_2 = _4 ^ _4;
RET.0 = 66116092861793757336264816349434570705_u128 as f64;
RET.1 = core::ptr::addr_of_mut!(_3);
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,95_isize];
_3 = (-1113429850_i32);
RET.0 = 79389568291655874090652327381417609402_u128 as f64;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(10_usize, 3_usize, Move(_3), 2_usize, Move(_2), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut i32,mut _2: (f64, *mut i32),mut _3: (f64, *mut i32),mut _4: i32,mut _5: i32,mut _6: isize,mut _7: ((f64, *mut i32),),mut _8: (i32, i128),mut _9: *mut i32,mut _10: [isize; 7],mut _11: i32,mut _12: i16,mut _13: i32) -> i16 {
mir! {
type RET = i16;
let _14: f32;
let _15: u8;
let _16: ();
let _17: ();
{
_1 = _7.0.1;
_7 = (_2,);
RET = _12 * _12;
_7.0 = _2;
_2.1 = core::ptr::addr_of_mut!((*_9));
_14 = _12 as f32;
_8.0 = (*_9);
_3.0 = _7.0.0;
_14 = 5_usize as f32;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(11_usize, 8_usize, Move(_8), 6_usize, Move(_6), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32,mut _2: u8) -> *mut i32 {
mir! {
type RET = *mut i32;
let _3: [i32; 5];
let _4: [i8; 1];
let _5: [bool; 2];
let _6: [bool; 2];
let _7: i8;
let _8: (u8, i8, f64, (u8, i32, bool), i16);
let _9: [bool; 2];
let _10: Adt44;
let _11: usize;
let _12: char;
let _13: i16;
let _14: i16;
let _15: *mut u32;
let _16: *const (i32, i128);
let _17: ((f64, *mut i32),);
let _18: *const usize;
let _19: (u8, i8, f64, (u8, i32, bool), i16);
let _20: Adt54;
let _21: [i32; 5];
let _22: *mut *const usize;
let _23: [isize; 7];
let _24: f32;
let _25: [isize; 7];
let _26: i128;
let _27: isize;
let _28: isize;
let _29: u32;
let _30: isize;
let _31: (f64, *mut i32);
let _32: u128;
let _33: ();
let _34: ();
{
RET = core::ptr::addr_of_mut!(_1);
_1 = 318451644320372129257500494088746697858_u128 as i32;
_2 = 197_u8 << (*RET);
(*RET) = -295722942_i32;
(*RET) = (-904650861_i32) << _2;
RET = core::ptr::addr_of_mut!(_1);
(*RET) = (-1091041362_i32) + 946660040_i32;
(*RET) = 1832752772_i32;
(*RET) = 3549500030_u32 as i32;
_2 = 92_u8 << (*RET);
_1 = (-1409052662_i32) | (-1569799638_i32);
(*RET) = !(-2126430204_i32);
(*RET) = 85805941_i32 - 427922320_i32;
_2 = !151_u8;
(*RET) = 47804395_i32;
RET = core::ptr::addr_of_mut!(_1);
Goto(bb1)
}
bb1 = {
_1 = 1262459062_i32;
(*RET) = 3785668133618090116_u64 as i32;
_4 = [55_i8];
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!(_1);
_1 = 2010032066_i32;
(*RET) = !(-1417117811_i32);
(*RET) = -(-2068659449_i32);
_2 = !28_u8;
Goto(bb2)
}
bb2 = {
(*RET) = (-1087519754_i32) << _2;
_1 = !(-2003357989_i32);
_5 = [true,false];
_6 = [false,false];
(*RET) = !1469013496_i32;
_3 = [_1,_1,(*RET),(*RET),_1];
_5 = _6;
(*RET) = 1341700797_i32 << _2;
_2 = 10033723788780529924081445880613958189_i128 as u8;
_5 = _6;
_3 = [(*RET),(*RET),(*RET),(*RET),_1];
(*RET) = 8160902008040662942_u64 as i32;
RET = core::ptr::addr_of_mut!(_1);
_1 = 1483102336_i32;
_1 = 97389717_i32;
RET = core::ptr::addr_of_mut!(_1);
(*RET) = -493684351_i32;
(*RET) = -1762824281_i32;
_5 = [false,false];
Goto(bb3)
}
bb3 = {
_7 = !70_i8;
_1 = (-1276480194_i32);
_6 = [true,true];
_7 = (-63_i8);
_1 = 928160465_i32 >> _2;
_2 = 32_u8 * 41_u8;
(*RET) = (-1134220863_i32) * (-984784313_i32);
_8.3.0 = 1461160056973525906_u64 as u8;
_8.0 = !_2;
_8.3.1 = (*RET) ^ (*RET);
RET = core::ptr::addr_of_mut!(_1);
_4 = [_7];
_7 = !34_i8;
_2 = _8.0;
RET = core::ptr::addr_of_mut!((*RET));
_8.3 = (_2, (*RET), false);
RET = core::ptr::addr_of_mut!((*RET));
_3 = [_1,(*RET),(*RET),_1,(*RET)];
(*RET) = _8.3.1;
_8.4 = _8.3.2 as i16;
(*RET) = _8.3.1;
Goto(bb4)
}
bb4 = {
_2 = _8.3.0;
_8.0 = _2 - _2;
RET = core::ptr::addr_of_mut!(_8.3.1);
_8.4 = (-145368138900178653375534067622706528398_i128) as i16;
_8.1 = _7 + _7;
(*RET) = -_1;
_5 = [_8.3.2,_8.3.2];
_9 = [_8.3.2,_8.3.2];
_2 = !_8.0;
_6 = _9;
_5 = [_8.3.2,_8.3.2];
_4 = [_8.1];
(*RET) = _1;
RET = core::ptr::addr_of_mut!((*RET));
_9 = _5;
_9 = [_8.3.2,_8.3.2];
_1 = 4_usize as i32;
RET = core::ptr::addr_of_mut!((*RET));
_8.3.2 = true;
_8.1 = _7;
_2 = 3962973588014085317_u64 as u8;
_5 = [_8.3.2,_8.3.2];
_13 = 147699935449541600553609147328232570638_u128 as i16;
_4 = [_8.1];
_12 = '\u{55044}';
RET = core::ptr::addr_of_mut!(_8.3.1);
Goto(bb5)
}
bb5 = {
(*RET) = !_1;
_8.2 = (-9223372036854775808_isize) as f64;
_8.0 = 4131267546_u32 as u8;
_8.2 = _7 as f64;
(*RET) = 142521129180186245304876765717712070623_u128 as i32;
_12 = '\u{26b91}';
_8.3.1 = _8.2 as i32;
_11 = !6_usize;
_5 = [_8.3.2,_8.3.2];
_8.3.0 = _8.0;
_1 = (*RET);
RET = core::ptr::addr_of_mut!((*RET));
_2 = (-122479970306477894089872950841461897096_i128) as u8;
Goto(bb6)
}
bb6 = {
Goto(bb7)
}
bb7 = {
_4 = [_7];
_8.3 = (_2, _1, false);
_8.3.1 = (-7823802023677153658_i64) as i32;
Goto(bb8)
}
bb8 = {
RET = core::ptr::addr_of_mut!(_1);
_11 = _8.3.1 as usize;
Goto(bb9)
}
bb9 = {
_17.0.1 = RET;
(*RET) = 14997997319569388501_u64 as i32;
_14 = -_8.4;
_11 = !12516967679222279185_usize;
_17.0.0 = _8.2 - _8.2;
_8.0 = _8.3.0 | _2;
_8.3.2 = true | true;
_6 = [_8.3.2,_8.3.2];
_17.0 = (_8.2, RET);
_3 = [(*RET),_1,_1,_1,_1];
(*RET) = _8.3.1 + _8.3.1;
(*RET) = -_8.3.1;
_8.1 = _7 & _7;
_2 = _8.3.0 - _8.0;
_8.4 = _14 << _8.0;
(*RET) = !_8.3.1;
(*RET) = _8.3.1;
(*RET) = _8.3.1 * _8.3.1;
_6 = [_8.3.2,_8.3.2];
_19.0 = !_8.0;
RET = _17.0.1;
_19.1 = !_8.1;
_19.3 = _8.3;
_13 = _8.4;
_5 = _6;
Goto(bb10)
}
bb10 = {
_19.3.0 = !_8.0;
_8.3.0 = _2;
_5 = [_19.3.2,_8.3.2];
_19.0 = !_2;
_8.2 = -_17.0.0;
_17.0.1 = RET;
_12 = '\u{9b6e2}';
(*RET) = _8.3.1 - _19.3.1;
_8.3 = (_2, _19.3.1, _19.3.2);
_8.0 = _2;
_4 = [_19.1];
_4 = [_19.1];
Goto(bb11)
}
bb11 = {
_19.3.1 = (*RET);
(*RET) = 4279971723_u32 as i32;
_12 = '\u{8dbe5}';
_19.3.1 = _8.3.1;
_8 = (_19.3.0, _7, _17.0.0, _19.3, _13);
_14 = _8.3.1 as i16;
_11 = !10999512411463527458_usize;
_17.0 = (_8.2, RET);
(*RET) = _8.3.1 | _8.3.1;
_12 = '\u{8593d}';
RET = _17.0.1;
_8.3.2 = !_19.3.2;
_8.3.1 = (-50982403205492101888285638073721334813_i128) as i32;
_19.3.2 = _8.3.2;
_7 = -_19.1;
_8.3.2 = !_19.3.2;
_17.0 = (_8.2, RET);
_7 = _19.0 as i8;
_8.4 = _14;
_6 = [_19.3.2,_19.3.2];
_12 = '\u{804a7}';
_19.0 = _19.3.0;
(*RET) = _8.3.1 >> _7;
_13 = -_14;
_24 = 57419_u16 as f32;
_17.0.1 = RET;
Call(_19.4 = fn13(_7, _17, _6, _17.0, _9, _17.0.1, _19.3.2, _8.4, _17.0, RET, _17.0.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24 = 1099605998958083442_u64 as f32;
_12 = '\u{184d9}';
_18 = core::ptr::addr_of!(_11);
(*RET) = !_19.3.1;
_8.1 = -_19.1;
_21 = [_19.3.1,(*RET),_8.3.1,_1,(*RET)];
RET = core::ptr::addr_of_mut!(_1);
_26 = _19.1 as i128;
_19.2 = _8.2 * _8.2;
_7 = !_8.1;
(*RET) = _8.1 as i32;
_5 = [_8.3.2,_19.3.2];
_8.0 = _2 ^ _8.3.0;
_19.3.2 = _8.3.2 ^ _8.3.2;
_8.2 = (-2140729712749579716_i64) as f64;
(*_18) = !1_usize;
_27 = !9223372036854775807_isize;
(*RET) = -_8.3.1;
_30 = _27 + _27;
_8.3.2 = _19.3.2;
_29 = !61544267_u32;
Goto(bb13)
}
bb13 = {
_32 = 168225803774187031029980267118450490357_u128 >> _19.0;
(*RET) = _19.3.1;
_27 = !_30;
_31.1 = _17.0.1;
_28 = _30 + _30;
_23 = [_27,_27,_28,_28,_30,_28,_28];
RET = _31.1;
_25 = _23;
_28 = _27 ^ _30;
_26 = -(-25590092057977874584557371627879467234_i128);
(*RET) = !_19.3.1;
Call(RET = fn14(_8.3.1, _8, _28, _5, _17.0.1, _27, _25, _9, _25, _8.3.2, _8.4, _4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_8.1 = _32 as i8;
_8.3.0 = _24 as u8;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(12_usize, 9_usize, Move(_9), 13_usize, Move(_13), 26_usize, Move(_26), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(12_usize, 6_usize, Move(_6), 12_usize, Move(_12), 2_usize, Move(_2), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(12_usize, 5_usize, Move(_5), 30_usize, Move(_30), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i8,mut _2: ((f64, *mut i32),),mut _3: [bool; 2],mut _4: (f64, *mut i32),mut _5: [bool; 2],mut _6: *mut i32,mut _7: bool,mut _8: i16,mut _9: (f64, *mut i32),mut _10: *mut i32,mut _11: *mut i32) -> i16 {
mir! {
type RET = i16;
let _12: usize;
let _13: i128;
let _14: [bool; 2];
let _15: u32;
let _16: f64;
let _17: u32;
let _18: ([i128; 7],);
let _19: [i32; 5];
let _20: char;
let _21: f64;
let _22: isize;
let _23: f32;
let _24: Adt47;
let _25: [i32; 5];
let _26: isize;
let _27: ((f64, *mut i32), *mut bool, *mut i32);
let _28: (i16, [i8; 1], char, *mut i32);
let _29: u128;
let _30: [i8; 1];
let _31: *const i128;
let _32: (i32, i128);
let _33: ();
let _34: ();
{
(*_6) = (-89661872_i32);
_9 = _4;
_6 = _10;
RET = _8 << _1;
_14 = [_7,_7];
_12 = 17049399457294117172_usize + 2381637681590922589_usize;
_17 = 310522301_u32 << RET;
_10 = _6;
RET = _8 << _1;
(*_11) = (-1057914020_i32) + (-478595546_i32);
_16 = _2.0.0;
(*_11) = 1832885036_i32;
_2.0.0 = _16;
match (*_6) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1832885036 => bb7,
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
(*_10) = (-1915512446_i32) & 915092066_i32;
_9 = (_16, _10);
RET = -_8;
_9.0 = _4.0;
_4.1 = _6;
RET = !_8;
_12 = 2000976184990183725_usize << RET;
Goto(bb8)
}
bb8 = {
_4.1 = core::ptr::addr_of_mut!((*_11));
(*_10) = (-1629331596_i32) ^ (-1562278193_i32);
_4 = (_16, _2.0.1);
_14 = [_7,_7];
_9 = _2.0;
_16 = _2.0.0;
RET = -_8;
_17 = (-9223372036854775808_isize) as u32;
_19 = [(*_6),(*_10),(*_6),(*_11),(*_6)];
_20 = '\u{7949}';
_4.0 = _2.0.0;
_18.0 = [150742580904636192217027772643945374635_i128,151149808078837888264788717349373406614_i128,157665918237409388019810114952314542724_i128,113840566769877145999182021435055839722_i128,8807024957007284303586934644681288727_i128,8951927370304700219770247637501547210_i128,4368395458174912052056127980214473808_i128];
_14 = [_7,_7];
_13 = 160410240432791077543573727140483730503_i128;
_2 = (_9,);
_8 = -RET;
_4 = _9;
_21 = _4.0;
_5 = [_7,_7];
_4.0 = 60319_u16 as f64;
_10 = core::ptr::addr_of_mut!((*_6));
_25 = [(*_11),(*_6),(*_6),(*_10),(*_6)];
_28.1 = [_1];
_8 = !RET;
match _13 {
0 => bb6,
1 => bb2,
160410240432791077543573727140483730503 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_23 = _13 as f32;
_8 = -RET;
_2 = (_9,);
_27.2 = _10;
_30 = [_1];
(*_10) = !1173581538_i32;
_30 = [_1];
match _13 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
160410240432791077543573727140483730503 => bb19,
_ => bb18
}
}
bb11 = {
Return()
}
bb12 = {
_4.1 = core::ptr::addr_of_mut!((*_11));
(*_10) = (-1629331596_i32) ^ (-1562278193_i32);
_4 = (_16, _2.0.1);
_14 = [_7,_7];
_9 = _2.0;
_16 = _2.0.0;
RET = -_8;
_17 = (-9223372036854775808_isize) as u32;
_19 = [(*_6),(*_10),(*_6),(*_11),(*_6)];
_20 = '\u{7949}';
_4.0 = _2.0.0;
_18.0 = [150742580904636192217027772643945374635_i128,151149808078837888264788717349373406614_i128,157665918237409388019810114952314542724_i128,113840566769877145999182021435055839722_i128,8807024957007284303586934644681288727_i128,8951927370304700219770247637501547210_i128,4368395458174912052056127980214473808_i128];
_14 = [_7,_7];
_13 = 160410240432791077543573727140483730503_i128;
_2 = (_9,);
_8 = -RET;
_4 = _9;
_21 = _4.0;
_5 = [_7,_7];
_4.0 = 60319_u16 as f64;
_10 = core::ptr::addr_of_mut!((*_6));
_25 = [(*_11),(*_6),(*_6),(*_10),(*_6)];
_28.1 = [_1];
_8 = !RET;
match _13 {
0 => bb6,
1 => bb2,
160410240432791077543573727140483730503 => bb10,
_ => bb9
}
}
bb13 = {
(*_10) = (-1915512446_i32) & 915092066_i32;
_9 = (_16, _10);
RET = -_8;
_9.0 = _4.0;
_4.1 = _6;
RET = !_8;
_12 = 2000976184990183725_usize << RET;
Goto(bb8)
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
Return()
}
bb18 = {
Return()
}
bb19 = {
_17 = 238_u8 as u32;
_22 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_26 = _4.0 as isize;
_19 = [(*_11),(*_11),(*_6),(*_10),(*_11)];
_4.1 = core::ptr::addr_of_mut!((*_6));
_26 = _16 as isize;
_28 = (_8, _30, _20, _6);
_6 = _4.1;
_27.0 = _2.0;
_2.0 = (_9.0, _11);
(*_10) = _22 as i32;
_2.0.1 = core::ptr::addr_of_mut!((*_11));
_27.0.0 = -_2.0.0;
_19 = [(*_6),(*_10),(*_6),(*_10),(*_11)];
_31 = core::ptr::addr_of!(_13);
_22 = _26 & _26;
_28.3 = _6;
_10 = _27.2;
_17 = 2289062719_u32;
_28.3 = _27.2;
_4.0 = _9.0;
_5 = [_7,_7];
RET = 83_u8 as i16;
Goto(bb20)
}
bb20 = {
Call(_33 = dump_var(13_usize, 19_usize, Move(_19), 12_usize, Move(_12), 7_usize, Move(_7), 30_usize, Move(_30)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_33 = dump_var(13_usize, 3_usize, Move(_3), 20_usize, Move(_20), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i32,mut _2: (u8, i8, f64, (u8, i32, bool), i16),mut _3: isize,mut _4: [bool; 2],mut _5: *mut i32,mut _6: isize,mut _7: [isize; 7],mut _8: [bool; 2],mut _9: [isize; 7],mut _10: bool,mut _11: i16,mut _12: [i8; 1]) -> *mut i32 {
mir! {
type RET = *mut i32;
let _13: (i32, i128);
let _14: [isize; 7];
let _15: char;
let _16: Adt49;
let _17: (u8, i32, bool);
let _18: [isize; 7];
let _19: (u8, i8, f64, (u8, i32, bool), i16);
let _20: u64;
let _21: ([i128; 7],);
let _22: Adt56;
let _23: [i32; 5];
let _24: [i128; 7];
let _25: Adt46;
let _26: (((f64, *mut i32),),);
let _27: f64;
let _28: Adt53;
let _29: char;
let _30: [i32; 5];
let _31: (i32, i128);
let _32: Adt56;
let _33: ();
let _34: ();
{
_11 = _2.4 - _2.4;
_2.0 = !_2.3.0;
RET = core::ptr::addr_of_mut!((*_5));
_11 = !_2.4;
_2.3.2 = _2.4 > _11;
_2.3 = (_2.0, (*RET), _10);
_13.0 = (*_5);
_3 = !_6;
_2.1 = 87_i8;
_2.3 = (_2.0, (*RET), _10);
_4 = [_10,_2.3.2];
_13.1 = -(-120241354003400367660813043823471398808_i128);
(*RET) = _1 >> _6;
(*RET) = _2.3.1 ^ _13.0;
(*_5) = _1;
Goto(bb1)
}
bb1 = {
_2.3.1 = (*RET) ^ _1;
_13.1 = 62163774824694831005853503968905406220_i128;
_13.1 = (-94855614104473192704329729038884219048_i128) - (-53399192898874258092327468092890584893_i128);
_2.3.1 = _1;
_7 = [_3,_3,_3,_6,_6,_6,_3];
_11 = _2.4 ^ _2.4;
_11 = 2771108796_u32 as i16;
_2.3.0 = !_2.0;
_2.0 = !_2.3.0;
_14 = _7;
RET = core::ptr::addr_of_mut!(_2.3.1);
_3 = -_6;
_2.3 = (_2.0, _13.0, _10);
_12 = [_2.1];
(*_5) = _1 >> _13.0;
RET = core::ptr::addr_of_mut!(_2.3.1);
_2.3.2 = !_10;
_3 = !_6;
_13.0 = (*RET) & (*_5);
_15 = '\u{da41a}';
_2.4 = !_11;
_2.3 = (_2.0, (*_5), _10);
_1 = (*_5) | _13.0;
_14 = _9;
_14 = [_6,_6,_6,_3,_3,_6,_6];
Goto(bb2)
}
bb2 = {
(*RET) = 5775230305522835167_i64 as i32;
_2.3.1 = _13.0;
RET = core::ptr::addr_of_mut!(_16.fld3.0);
(*RET) = _1;
_2.4 = _2.0 as i16;
_8 = [_10,_10];
_16.fld5.1 = _12;
_16.fld5.2 = _15;
_8 = _4;
_16.fld0.0.0 = _2.2;
(*RET) = !_2.3.1;
_16.fld6 = _2.0 as i64;
_17.1 = -(*_5);
match _2.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
87 => bb10,
_ => bb9
}
}
bb3 = {
_2.3.1 = (*RET) ^ _1;
_13.1 = 62163774824694831005853503968905406220_i128;
_13.1 = (-94855614104473192704329729038884219048_i128) - (-53399192898874258092327468092890584893_i128);
_2.3.1 = _1;
_7 = [_3,_3,_3,_6,_6,_6,_3];
_11 = _2.4 ^ _2.4;
_11 = 2771108796_u32 as i16;
_2.3.0 = !_2.0;
_2.0 = !_2.3.0;
_14 = _7;
RET = core::ptr::addr_of_mut!(_2.3.1);
_3 = -_6;
_2.3 = (_2.0, _13.0, _10);
_12 = [_2.1];
(*_5) = _1 >> _13.0;
RET = core::ptr::addr_of_mut!(_2.3.1);
_2.3.2 = !_10;
_3 = !_6;
_13.0 = (*RET) & (*_5);
_15 = '\u{da41a}';
_2.4 = !_11;
_2.3 = (_2.0, (*_5), _10);
_1 = (*_5) | _13.0;
_14 = _9;
_14 = [_6,_6,_6,_3,_3,_6,_6];
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
_16.fld5.2 = _15;
_19.2 = _16.fld0.0.0 * _16.fld0.0.0;
Call(_19 = fn15(_13.0, _8, _2.3.1, _2.0, _2.3.1, _1, RET, RET, _9, _2.3.1, _2.3.2, _16.fld6, _17.1, _2.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19 = (_2.0, _2.1, _2.2, _2.3, _2.4);
_16.fld6 = _6 as i64;
_2.3 = _19.3;
_16.fld5 = (_19.4, _12, _15, RET);
_19 = (_2.0, _2.1, _16.fld0.0.0, _2.3, _2.4);
_16.fld5.1 = [_19.1];
_9 = _14;
_16.fld0.0.1 = _16.fld5.3;
_19.2 = (*RET) as f64;
_16.fld5.0 = !_19.4;
_16.fld3 = ((*_5), _13.1);
_1 = _13.0 + _2.3.1;
_2.3.0 = _2.1 as u8;
_13.1 = _16.fld3.1 & _16.fld3.1;
_21.0 = [_13.1,_13.1,_13.1,_16.fld3.1,_16.fld3.1,_13.1,_16.fld3.1];
_2.1 = _19.1;
_2.3 = (_19.0, _1, _19.3.2);
_23 = [_2.3.1,_17.1,_1,_19.3.1,_2.3.1];
_16.fld5.0 = 23990540513749984079859609978736343282_u128 as i16;
(*RET) = -_1;
_16.fld0.0.0 = 46602_u16 as f64;
_17.2 = _10 ^ _2.3.2;
_16.fld3.0 = _2.3.1 | _13.0;
_20 = 46189_u16 as u64;
Call(_17.1 = core::intrinsics::bswap(_2.3.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = [_6,_6,_6,_3,_3,_6,_3];
_17.0 = !_2.0;
_2.0 = _2.3.0;
_16.fld5.2 = _15;
_2.3.0 = !_2.0;
_8 = [_2.3.2,_19.3.2];
_4 = _8;
_16.fld0.0.1 = core::ptr::addr_of_mut!(_19.3.1);
_2.0 = !_17.0;
_24 = _21.0;
_19.1 = _2.1;
_26.0.0.0 = 19209_u16 as f64;
RET = _16.fld5.3;
_16.fld3.1 = _13.1;
_19 = (_17.0, _2.1, _2.2, _2.3, _16.fld5.0);
_17.2 = _19.3.2;
_16.fld3.1 = 26877925878034426639847652255052877705_u128 as i128;
_2.3.0 = _19.0;
_16.fld3.1 = _13.1;
(*_5) = -_19.3.1;
_24 = _21.0;
Call((*RET) = core::intrinsics::bswap((*_5)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = core::ptr::addr_of_mut!(_16.fld3.0);
_8 = [_10,_19.3.2];
Goto(bb14)
}
bb14 = {
_19.4 = !_2.4;
_1 = -_16.fld3.0;
_10 = _16.fld5.2 == _16.fld5.2;
_31 = _16.fld3;
_17 = (_2.0, (*_5), _10);
_13 = (_19.3.1, _16.fld3.1);
_2.2 = _13.0 as f64;
_19.3.0 = _17.0;
(*RET) = _1 + _31.0;
_30 = [_2.3.1,(*RET),_1,_13.0,(*_5)];
_13.0 = (*RET) * (*_5);
_2 = (_19.3.0, _19.1, _16.fld0.0.0, _19.3, _11);
_16.fld1 = [_16.fld3.1,_13.1,_13.1,_31.1,_16.fld3.1,_16.fld3.1,_31.1];
_17.2 = !_2.3.2;
_2.0 = _19.3.0 ^ _2.3.0;
_31.0 = (*RET) * (*RET);
_16.fld4 = _26.0.0.0;
_2.3 = (_2.0, _13.0, _19.3.2);
_29 = _15;
_16.fld5.3 = core::ptr::addr_of_mut!((*RET));
_26.0 = _16.fld0;
(*RET) = (*_5);
_19.4 = _11;
_16.fld5.3 = _16.fld0.0.1;
_26 = (_16.fld0,);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(14_usize, 1_usize, Move(_1), 14_usize, Move(_14), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(14_usize, 13_usize, Move(_13), 23_usize, Move(_23), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(14_usize, 31_usize, Move(_31), 29_usize, Move(_29), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i32,mut _2: [bool; 2],mut _3: i32,mut _4: u8,mut _5: i32,mut _6: i32,mut _7: *mut i32,mut _8: *mut i32,mut _9: [isize; 7],mut _10: i32,mut _11: bool,mut _12: i64,mut _13: i32,mut _14: f64) -> (u8, i8, f64, (u8, i32, bool), i16) {
mir! {
type RET = (u8, i8, f64, (u8, i32, bool), i16);
let _15: u32;
let _16: Adt48;
let _17: [i32; 5];
let _18: i8;
let _19: u128;
let _20: isize;
let _21: char;
let _22: [i128; 2];
let _23: [isize; 7];
let _24: i32;
let _25: f64;
let _26: ([i128; 7],);
let _27: (i32, i128);
let _28: ();
let _29: ();
{
RET.1 = _12 as i8;
RET.3.2 = _11;
RET.3.0 = !_4;
RET.4 = 24246_i16 + (-23651_i16);
(*_7) = _10;
RET.2 = _14;
_8 = core::ptr::addr_of_mut!((*_7));
RET.3.1 = (*_8);
_7 = _8;
RET.0 = !RET.3.0;
RET.3 = (_4, _1, _11);
RET.3 = (RET.0, (*_7), _11);
(*_7) = RET.1 as i32;
_5 = _13;
_1 = RET.2 as i32;
RET.4 = 25297_i16 - 26353_i16;
_11 = !RET.3.2;
Goto(bb1)
}
bb1 = {
(*_7) = '\u{76c80}' as i32;
RET.3.0 = RET.0;
_12 = 120107781448060421256045597683534155320_u128 as i64;
RET.3.1 = _3;
_9 = [74_isize,9223372036854775807_isize,100_isize,(-124_isize),9223372036854775807_isize,117_isize,(-9223372036854775808_isize)];
RET.3 = (RET.0, _10, _11);
RET.2 = RET.0 as f64;
RET.0 = _4 & _4;
(*_7) = _1;
RET.3.1 = _3;
RET.0 = RET.3.0;
RET.3.0 = _12 as u8;
_4 = RET.3.0;
_14 = RET.2 * RET.2;
_2 = [_11,RET.3.2];
Call(RET.3.1 = fn16(_8, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_7) = !_10;
(*_8) = _1;
RET.3.0 = _4;
RET.3 = (RET.0, _3, _11);
RET.3.1 = RET.3.0 as i32;
(*_7) = _10;
RET.1 = (-56_i8);
(*_8) = !_6;
_13 = _3;
_2 = [_11,_11];
_14 = -RET.2;
RET.0 = RET.3.0 * RET.3.0;
RET.3.2 = RET.0 != RET.3.0;
RET.3.1 = _1 | _13;
_8 = _7;
RET.1 = RET.3.2 as i8;
_11 = !RET.3.2;
Goto(bb3)
}
bb3 = {
_13 = -_3;
RET.3 = (RET.0, (*_8), _11);
Goto(bb4)
}
bb4 = {
_13 = _12 as i32;
_11 = !RET.3.2;
_2 = [RET.3.2,_11];
RET.4 = 4120_i16 >> (*_7);
_12 = (-7002715248578486590_i64) ^ (-3262254477068770577_i64);
_5 = _3;
RET.3.1 = _10;
RET.3.0 = RET.0;
_3 = _6;
_2 = [RET.3.2,RET.3.2];
RET.2 = -_14;
_14 = RET.2 * RET.2;
_4 = !RET.0;
(*_7) = 29707301725798105906891023910607291036_u128 as i32;
RET.4 = (-19980_i16);
RET.3.1 = (*_8) - (*_7);
RET.2 = _14 - _14;
RET.2 = -_14;
_7 = _8;
_12 = (-311099452863251244_i64);
RET.3.1 = 10095_u16 as i32;
_8 = _7;
Goto(bb5)
}
bb5 = {
_11 = !RET.3.2;
_10 = (*_7) ^ _3;
_6 = _10;
_5 = _10 - _6;
RET.4 = 9457_i16;
RET.1 = !(-81_i8);
match RET.4 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
9457 => bb14,
_ => bb13
}
}
bb6 = {
_13 = _12 as i32;
_11 = !RET.3.2;
_2 = [RET.3.2,_11];
RET.4 = 4120_i16 >> (*_7);
_12 = (-7002715248578486590_i64) ^ (-3262254477068770577_i64);
_5 = _3;
RET.3.1 = _10;
RET.3.0 = RET.0;
_3 = _6;
_2 = [RET.3.2,RET.3.2];
RET.2 = -_14;
_14 = RET.2 * RET.2;
_4 = !RET.0;
(*_7) = 29707301725798105906891023910607291036_u128 as i32;
RET.4 = (-19980_i16);
RET.3.1 = (*_8) - (*_7);
RET.2 = _14 - _14;
RET.2 = -_14;
_7 = _8;
_12 = (-311099452863251244_i64);
RET.3.1 = 10095_u16 as i32;
_8 = _7;
Goto(bb5)
}
bb7 = {
_13 = -_3;
RET.3 = (RET.0, (*_8), _11);
Goto(bb4)
}
bb8 = {
(*_7) = !_10;
(*_8) = _1;
RET.3.0 = _4;
RET.3 = (RET.0, _3, _11);
RET.3.1 = RET.3.0 as i32;
(*_7) = _10;
RET.1 = (-56_i8);
(*_8) = !_6;
_13 = _3;
_2 = [_11,_11];
_14 = -RET.2;
RET.0 = RET.3.0 * RET.3.0;
RET.3.2 = RET.0 != RET.3.0;
RET.3.1 = _1 | _13;
_8 = _7;
RET.1 = RET.3.2 as i8;
_11 = !RET.3.2;
Goto(bb3)
}
bb9 = {
(*_7) = '\u{76c80}' as i32;
RET.3.0 = RET.0;
_12 = 120107781448060421256045597683534155320_u128 as i64;
RET.3.1 = _3;
_9 = [74_isize,9223372036854775807_isize,100_isize,(-124_isize),9223372036854775807_isize,117_isize,(-9223372036854775808_isize)];
RET.3 = (RET.0, _10, _11);
RET.2 = RET.0 as f64;
RET.0 = _4 & _4;
(*_7) = _1;
RET.3.1 = _3;
RET.0 = RET.3.0;
RET.3.0 = _12 as u8;
_4 = RET.3.0;
_14 = RET.2 * RET.2;
_2 = [_11,RET.3.2];
Call(RET.3.1 = fn16(_8, _9), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb14 = {
_20 = !32_isize;
_23 = [_20,_20,_20,_20,_20,_20,_20];
_21 = '\u{9d29f}';
_6 = _3;
_1 = 1743440096_u32 as i32;
_14 = -RET.2;
_4 = RET.2 as u8;
(*_8) = _5 + _5;
_23 = [_20,_20,_20,_20,_20,_20,_20];
RET.4 = !(-12643_i16);
_11 = !RET.3.2;
_19 = 98815853585747047681408743243767655798_u128;
_9 = _23;
(*_8) = _5 ^ _5;
_5 = (*_7) << RET.0;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(15_usize, 19_usize, Move(_19), 9_usize, Move(_9), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(15_usize, 12_usize, Move(_12), 13_usize, Move(_13), 11_usize, Move(_11), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *mut i32,mut _2: [isize; 7]) -> i32 {
mir! {
type RET = i32;
let _3: bool;
let _4: bool;
let _5: (u8, i32, bool);
let _6: [i32; 5];
let _7: (i32, i128);
let _8: [i32; 5];
let _9: [i8; 1];
let _10: [i128; 7];
let _11: *const (i32, i128);
let _12: [i128; 2];
let _13: f32;
let _14: [isize; 7];
let _15: i32;
let _16: bool;
let _17: ();
let _18: ();
{
RET = '\u{7a36d}' as i32;
_2 = [(-111_isize),117_isize,(-9223372036854775808_isize),(-37_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
(*_1) = RET & RET;
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-74_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = !RET;
_2 = [(-9223372036854775808_isize),(-101_isize),95_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
(*_1) = RET;
(*_1) = !RET;
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 27070187_i32;
RET = (-1974547530678602288_i64) as i32;
_3 = RET >= (*_1);
RET = -1321072104_i32;
_1 = core::ptr::addr_of_mut!(RET);
Goto(bb2)
}
bb2 = {
(*_1) = 570467759_i32;
_4 = !_3;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = 20386_i16 as i32;
(*_1) = _5.1;
_4 = (*_1) < _5.1;
Goto(bb3)
}
bb3 = {
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,29_isize,(-3_isize),(-9223372036854775808_isize)];
RET = 88_isize as i32;
_1 = core::ptr::addr_of_mut!((*_1));
_7 = ((*_1), (-47516896867026888744439939147516284340_i128));
_5.2 = !_3;
_6 = [RET,_7.0,RET,(*_1),(*_1)];
_5.2 = _7.0 < (*_1);
(*_1) = !_7.0;
(*_1) = 12195268079449342396072770078577657722_u128 as i32;
_5.1 = !RET;
_7.1 = (-165285870164856212304816402414475337291_i128);
match _7.1 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
174996496756082251158558205017292874165 => bb9,
_ => bb8
}
}
bb4 = {
(*_1) = 570467759_i32;
_4 = !_3;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = 20386_i16 as i32;
(*_1) = _5.1;
_4 = (*_1) < _5.1;
Goto(bb3)
}
bb5 = {
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 27070187_i32;
RET = (-1974547530678602288_i64) as i32;
_3 = RET >= (*_1);
RET = -1321072104_i32;
_1 = core::ptr::addr_of_mut!(RET);
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
_10 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_2 = [9223372036854775807_isize,(-45_isize),(-9223372036854775808_isize),62_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = [125_i8];
_5.0 = !164_u8;
Goto(bb10)
}
bb10 = {
_3 = _4;
_7.1 = 84213291771743158244082279499322122001_i128 << (*_1);
(*_1) = !_5.1;
_5 = (248_u8, RET, _4);
(*_1) = _5.1;
_8 = [RET,(*_1),(*_1),_5.1,_7.0];
_7.0 = _5.1;
RET = _3 as i32;
_5.0 = !233_u8;
_7.0 = -(*_1);
_5.1 = -(*_1);
RET = !_7.0;
_6 = [RET,RET,RET,(*_1),RET];
_8 = _6;
_7 = ((*_1), 40717395507042937972945468701362224495_i128);
_11 = core::ptr::addr_of!(_7);
(*_11).0 = (*_1) ^ _5.1;
_1 = core::ptr::addr_of_mut!(_5.1);
_8 = [RET,_7.0,(*_1),(*_11).0,(*_11).0];
(*_11).1 = 79456829190043365571060443481714598981_i128 ^ 130048815129750751974668553134510720880_i128;
(*_11).0 = !RET;
Call((*_11).0 = core::intrinsics::bswap((*_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4 = _5.2 & _5.2;
Goto(bb12)
}
bb12 = {
_10 = [_7.1,(*_11).1,_7.1,(*_11).1,(*_11).1,(*_11).1,_7.1];
(*_11).0 = (*_1);
(*_11).0 = !(*_1);
(*_11).1 = 53396_u16 as i128;
_14 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),75_isize,(-25_isize)];
_6 = [(*_1),(*_1),RET,(*_1),RET];
_9 = [(-27_i8)];
RET = 4271037188_u32 as i32;
(*_11) = ((*_1), 8218202747159845734627091691914599314_i128);
_12 = [_7.1,(*_11).1];
Goto(bb13)
}
bb13 = {
_13 = (-49_isize) as f32;
_4 = _5.2;
_12 = [(*_11).1,(*_11).1];
_1 = core::ptr::addr_of_mut!((*_11).0);
_12 = [(*_11).1,(*_11).1];
_13 = _5.0 as f32;
_11 = core::ptr::addr_of!(_7);
_2 = [(-9223372036854775808_isize),(-57_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = 13869167739712682476_u64 as i32;
_7.1 = '\u{3695b}' as i128;
(*_11) = (_5.1, (-120861313902164586207969250643432264330_i128));
(*_1) = _5.1;
_13 = 3_usize as f32;
_7.1 = !121248476383817339114024263801597591637_i128;
_5 = (255_u8, _7.0, _4);
_15 = (*_1) << (*_11).1;
_7.1 = (-21812520262294325045317701838136207411_i128) | 75203814836611614129895464089497389370_i128;
_7 = (_15, (-19775771296491202907041009518825477398_i128));
match _7.1 {
320506595624447260556333597912942734058 => bb15,
_ => bb14
}
}
bb14 = {
(*_1) = 570467759_i32;
_4 = !_3;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = 20386_i16 as i32;
(*_1) = _5.1;
_4 = (*_1) < _5.1;
Goto(bb3)
}
bb15 = {
_16 = _5.2 ^ _5.2;
_13 = 121_i8 as f32;
_5.0 = 119_u8 | 209_u8;
_9 = [44_i8];
_5 = (11_u8, _15, _4);
Goto(bb16)
}
bb16 = {
Call(_17 = dump_var(16_usize, 12_usize, Move(_12), 3_usize, Move(_3), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_17 = dump_var(16_usize, 16_usize, Move(_16), 9_usize, Move(_9), 18_usize, _18, 18_usize, _18), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: [i32; 5],mut _2: Adt44,mut _3: *mut i32,mut _4: *mut i32,mut _5: i128,mut _6: [i32; 5],mut _7: i32,mut _8: i32,mut _9: Adt43,mut _10: i128,mut _11: [i32; 5],mut _12: (i32, i128),mut _13: *mut i32,mut _14: *mut i32,mut _15: *const i128) -> *const i128 {
mir! {
type RET = *const i128;
let _16: u16;
let _17: u8;
let _18: isize;
let _19: i32;
let _20: f64;
let _21: ();
let _22: ();
{
_8 = (*_4);
_12.0 = -(*_13);
_6 = [Field::<i32>(Variant(_9, 1), 5),_12.0,(*_14),(*_13),Field::<(i32, i128)>(Variant(_9, 1), 4).0];
RET = core::ptr::addr_of!((*_15));
place!(Field::<(f64, *mut i32)>(Variant(_2, 2), 3)).1 = _14;
(*_14) = _7;
_1 = [_12.0,(*_13),_7,(*_13),Field::<(i32, i128)>(Variant(_9, 1), 4).0];
_14 = core::ptr::addr_of_mut!(_8);
_10 = (*RET) - _12.1;
_8 = Field::<(f64, *mut i32)>(Variant(_2, 2), 3).0 as i32;
_10 = (*RET);
place!(Field::<i64>(Variant(_2, 2), 0)) = -9183353979355682925_i64;
(*_14) = 155_u8 as i32;
_1 = _6;
_15 = core::ptr::addr_of!((*RET));
(*_13) = !Field::<i32>(Variant(_9, 1), 5);
place!(Field::<i16>(Variant(_2, 2), 1)) = !26714_i16;
_12.0 = 3174920099_u32 as i32;
(*_4) = Field::<i32>(Variant(_2, 2), 2) + _7;
_18 = (-16_isize) ^ (-9223372036854775808_isize);
_8 = Field::<(i32, i128)>(Variant(_9, 1), 4).0;
_3 = core::ptr::addr_of_mut!((*_14));
_19 = (*_3) << Field::<i32>(Variant(_2, 2), 2);
_6 = [Field::<(i32, i128)>(Variant(_9, 1), 4).0,(*_3),(*_14),(*_13),(*_14)];
place!(Field::<i16>(Variant(_2, 2), 1)) = '\u{10f318}' as i16;
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(17_usize, 1_usize, Move(_1), 10_usize, Move(_10), 18_usize, Move(_18), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_21 = dump_var(17_usize, 11_usize, Move(_11), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [i128; 7],mut _2: (f64, *mut i32),mut _3: *const i128,mut _4: u8) -> i32 {
mir! {
type RET = i32;
let _5: isize;
let _6: ([i128; 7],);
let _7: Adt46;
let _8: (i32, i128);
let _9: isize;
let _10: [i32; 5];
let _11: isize;
let _12: *mut bool;
let _13: bool;
let _14: [i8; 1];
let _15: bool;
let _16: f64;
let _17: isize;
let _18: Adt53;
let _19: bool;
let _20: [i8; 1];
let _21: isize;
let _22: i128;
let _23: i128;
let _24: *const usize;
let _25: *mut i32;
let _26: Adt50;
let _27: [i128; 2];
let _28: u16;
let _29: u64;
let _30: Adt59;
let _31: [bool; 2];
let _32: Adt58;
let _33: Adt52;
let _34: isize;
let _35: [i128; 7];
let _36: f64;
let _37: ();
let _38: ();
{
RET = -2022586229_i32;
_4 = 10925_u16 as u8;
_2.0 = 1987548822_u32 as f64;
_2.1 = core::ptr::addr_of_mut!(RET);
_2.0 = (-160122338597820888163655305185077603677_i128) as f64;
_2.1 = core::ptr::addr_of_mut!(RET);
_4 = !224_u8;
RET = (-1219342848_i32);
_5 = '\u{4aecd}' as isize;
_2.0 = 1006831398_u32 as f64;
_2.0 = 2945319170_u32 as f64;
_6.0 = [24162926650238234005762109057990468883_i128,(-86271704853472112830056621759335277365_i128),10134558218692789348169365029990133411_i128,143818008728855826379876323740255901554_i128,89514771614173226993294796922649561486_i128,66719464407405840243013302772542404815_i128,(-13449668598000113499814826039931932119_i128)];
Goto(bb1)
}
bb1 = {
_4 = !207_u8;
_4 = 131_u8 << _5;
_6 = (_1,);
_5 = 16563895975739485607_u64 as isize;
_1 = [(-13002805803751963883548911262628501780_i128),(-74556228128197324078549881457724124874_i128),(-59968119338522379930126975509603953124_i128),(-77253413023230890839931236306542092643_i128),(-124897371960530253951162934361914870907_i128),15345302259737599505416535493979530705_i128,(-56136440184282052767843477696119070438_i128)];
_8.0 = _5 as i32;
_2.1 = core::ptr::addr_of_mut!(RET);
_9 = -_5;
_6.0 = _1;
_10 = [_8.0,_8.0,RET,_8.0,RET];
_8 = (RET, (-97290309208976200235579023314832726358_i128));
_11 = _9 - _5;
_2.0 = 14709148692504301201_u64 as f64;
Goto(bb2)
}
bb2 = {
_8 = (RET, 41996950508151238391468882888101426420_i128);
_7 = Adt46::Variant1 { fld0: 3887936905_u32 };
_10 = [RET,_8.0,RET,RET,RET];
_8 = (RET, (-112242245983840279002101710217126192991_i128));
_7 = Adt46::Variant1 { fld0: 3822925232_u32 };
place!(Field::<u32>(Variant(_7, 1), 0)) = 2002731410_u32 | 397492247_u32;
_8.1 = 54973614767879746335218880137746800293_i128;
_8.1 = !149917635055603103485470885039489759437_i128;
_8.0 = !RET;
place!(Field::<u32>(Variant(_7, 1), 0)) = 3943850998_u32;
place!(Field::<u32>(Variant(_7, 1), 0)) = !1141523934_u32;
_2.0 = _11 as f64;
_10 = [RET,_8.0,RET,RET,RET];
_8.1 = 97445702951358492555528354033676585058_i128 * 25779650988784167614609048216138869016_i128;
SetDiscriminant(_7, 1);
_8.0 = RET;
_11 = _9;
Goto(bb3)
}
bb3 = {
_13 = true;
RET = _13 as i32;
_11 = _9;
_8.0 = RET * RET;
RET = !_8.0;
_1 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_6 = (_1,);
_12 = core::ptr::addr_of_mut!(_13);
place!(Field::<u32>(Variant(_7, 1), 0)) = 1583716104_u32;
_8.1 = !(-18260776329637253834678948609400402669_i128);
_15 = (*_12) | (*_12);
_8.0 = !RET;
_8.1 = !152407664449365615885584857865456903278_i128;
(*_12) = !_15;
_6.0 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
(*_12) = _15;
SetDiscriminant(_7, 0);
_1 = _6.0;
_5 = !_9;
_13 = _15;
place!(Field::<[isize; 7]>(Variant(_7, 0), 1)) = [_9,_9,_5,_9,_11,_9,_5];
_6 = (_1,);
_16 = _2.0 + _2.0;
_12 = core::ptr::addr_of_mut!((*_12));
_6 = (_1,);
_8.0 = RET >> RET;
_2.1 = core::ptr::addr_of_mut!(_8.0);
Goto(bb4)
}
bb4 = {
_16 = _2.0 - _2.0;
_2.0 = (-603_i16) as f64;
_6 = (_1,);
_5 = _11 & _9;
_13 = !_15;
place!(Field::<*const i32>(Variant(_7, 0), 2)) = core::ptr::addr_of!(_8.0);
_16 = _2.0 + _2.0;
_1 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_11 = _9 >> _8.0;
_20 = [29_i8];
_4 = 2815175778_u32 as u8;
_19 = !_13;
_2.1 = core::ptr::addr_of_mut!(_8.0);
_5 = _11;
_8.1 = (*_12) as i128;
(*_12) = _19;
_19 = (*_12);
_16 = (-4950789314406052314_i64) as f64;
_11 = -_5;
_8 = (RET, 7023807620736563262683854850791665547_i128);
_14 = [(-12_i8)];
_1 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
match _8.1 {
7023807620736563262683854850791665547 => bb5,
_ => bb3
}
}
bb5 = {
_21 = _11;
_16 = _2.0;
_6.0 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_2.1 = core::ptr::addr_of_mut!(_8.0);
_13 = _4 != _4;
_4 = 222_u8 & 209_u8;
_12 = core::ptr::addr_of_mut!((*_12));
_26 = Adt50 { fld0: 117_i8,fld1: '\u{a49ad}',fld2: _2.1 };
_17 = _21;
(*_12) = RET <= _8.0;
Goto(bb6)
}
bb6 = {
_27 = [_8.1,_8.1];
_17 = -_21;
_29 = !18068900008804265132_u64;
place!(Field::<*const i32>(Variant(_7, 0), 2)) = core::ptr::addr_of!(RET);
Goto(bb7)
}
bb7 = {
_12 = core::ptr::addr_of_mut!((*_12));
_28 = 55144_u16;
_23 = _8.1 * _8.1;
_26 = Adt50 { fld0: (-24_i8),fld1: '\u{fcc05}',fld2: _2.1 };
_22 = -_23;
_19 = _13 & (*_12);
_26 = Adt50 { fld0: (-122_i8),fld1: '\u{344c5}',fld2: _2.1 };
_14 = _20;
_4 = 111_u8 & 51_u8;
match _26.fld0 {
0 => bb6,
1 => bb8,
2 => bb9,
340282366920938463463374607431768211334 => bb11,
_ => bb10
}
}
bb8 = {
_4 = !207_u8;
_4 = 131_u8 << _5;
_6 = (_1,);
_5 = 16563895975739485607_u64 as isize;
_1 = [(-13002805803751963883548911262628501780_i128),(-74556228128197324078549881457724124874_i128),(-59968119338522379930126975509603953124_i128),(-77253413023230890839931236306542092643_i128),(-124897371960530253951162934361914870907_i128),15345302259737599505416535493979530705_i128,(-56136440184282052767843477696119070438_i128)];
_8.0 = _5 as i32;
_2.1 = core::ptr::addr_of_mut!(RET);
_9 = -_5;
_6.0 = _1;
_10 = [_8.0,_8.0,RET,_8.0,RET];
_8 = (RET, (-97290309208976200235579023314832726358_i128));
_11 = _9 - _5;
_2.0 = 14709148692504301201_u64 as f64;
Goto(bb2)
}
bb9 = {
_21 = _11;
_16 = _2.0;
_6.0 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_2.1 = core::ptr::addr_of_mut!(_8.0);
_13 = _4 != _4;
_4 = 222_u8 & 209_u8;
_12 = core::ptr::addr_of_mut!((*_12));
_26 = Adt50 { fld0: 117_i8,fld1: '\u{a49ad}',fld2: _2.1 };
_17 = _21;
(*_12) = RET <= _8.0;
Goto(bb6)
}
bb10 = {
_16 = _2.0 - _2.0;
_2.0 = (-603_i16) as f64;
_6 = (_1,);
_5 = _11 & _9;
_13 = !_15;
place!(Field::<*const i32>(Variant(_7, 0), 2)) = core::ptr::addr_of!(_8.0);
_16 = _2.0 + _2.0;
_1 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_11 = _9 >> _8.0;
_20 = [29_i8];
_4 = 2815175778_u32 as u8;
_19 = !_13;
_2.1 = core::ptr::addr_of_mut!(_8.0);
_5 = _11;
_8.1 = (*_12) as i128;
(*_12) = _19;
_19 = (*_12);
_16 = (-4950789314406052314_i64) as f64;
_11 = -_5;
_8 = (RET, 7023807620736563262683854850791665547_i128);
_14 = [(-12_i8)];
_1 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
match _8.1 {
7023807620736563262683854850791665547 => bb5,
_ => bb3
}
}
bb11 = {
_2.1 = core::ptr::addr_of_mut!(RET);
_8 = (RET, _23);
_6 = (_1,);
place!(Field::<[isize; 7]>(Variant(_7, 0), 1)) = [_21,_11,_17,_17,_11,_21,_17];
_20 = [_26.fld0];
_10 = [_8.0,_8.0,RET,RET,RET];
place!(Field::<u64>(Variant(_7, 0), 0)) = !_29;
_5 = _17 >> _21;
_8.1 = _22 | _23;
_26.fld1 = '\u{2c7aa}';
_2.0 = _16;
RET = _8.0;
_26.fld2 = core::ptr::addr_of_mut!(_8.0);
_11 = _21 - _17;
_25 = core::ptr::addr_of_mut!(RET);
SetDiscriminant(_7, 0);
_13 = _19;
_5 = _29 as isize;
_2 = (_16, _25);
place!(Field::<[isize; 7]>(Variant(_7, 0), 1)) = [_17,_11,_21,_21,_5,_21,_11];
_8 = ((*_25), _23);
Goto(bb12)
}
bb12 = {
_5 = _21 + _21;
_20 = [_26.fld0];
_19 = _13;
_12 = core::ptr::addr_of_mut!((*_12));
_21 = _11 * _5;
_21 = _17 | _11;
_17 = _8.1 as isize;
Goto(bb13)
}
bb13 = {
_7 = Adt46::Variant1 { fld0: 2428414524_u32 };
RET = _8.0 + _8.0;
_20 = _14;
_26.fld1 = '\u{12bda}';
_28 = 16024_u16 * 53421_u16;
_8.1 = -_23;
_1 = [_23,_22,_8.1,_23,_23,_8.1,_22];
_19 = (*_12) != (*_12);
_11 = _21 ^ _21;
_26.fld0 = 146843819977723520754576897706939011384_u128 as i8;
_26.fld2 = _2.1;
_8.0 = _26.fld1 as i32;
_8.1 = !_23;
_26 = Adt50 { fld0: (-55_i8),fld1: '\u{f761b}',fld2: _2.1 };
_2 = (_16, _26.fld2);
_21 = _11;
_1 = [_8.1,_22,_22,_22,_22,_22,_8.1];
_13 = _19 | _19;
_26.fld0 = (-110_i8);
_15 = (*_25) < RET;
_5 = _29 as isize;
Goto(bb14)
}
bb14 = {
_7 = Adt46::Variant1 { fld0: 3155442455_u32 };
_6.0 = [_22,_23,_8.1,_22,_22,_23,_23];
RET = _28 as i32;
_26.fld1 = '\u{1f818}';
_16 = -_2.0;
_1 = [_8.1,_22,_8.1,_22,_22,_8.1,_23];
_1 = [_23,_22,_8.1,_22,_8.1,_8.1,_22];
_14 = _20;
RET = _29 as i32;
_6 = (_1,);
place!(Field::<u32>(Variant(_7, 1), 0)) = 1137815541895776130_usize as u32;
_34 = _26.fld1 as isize;
_29 = 12878139882225720082_u64;
_31 = [(*_12),(*_12)];
_23 = _26.fld0 as i128;
_11 = _4 as isize;
(*_25) = 6067778271305521190_usize as i32;
_17 = _26.fld0 as isize;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(18_usize, 21_usize, Move(_21), 23_usize, Move(_23), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(18_usize, 1_usize, Move(_1), 11_usize, Move(_11), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(18_usize, 8_usize, Move(_8), 22_usize, Move(_22), 20_usize, Move(_20), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{434be}'), std::hint::black_box(117149857466892143738098951519483213018_i128), std::hint::black_box(13905609174287409677_usize), std::hint::black_box(1173_i16));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: u128,
fld2: [i128; 2],

},
Variant1{
fld0: f64,
fld1: *mut u32,
fld2: f32,
fld3: *const i128,
fld4: (i32, i128),
fld5: i32,
fld6: u128,
fld7: [i32; 5],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *const i32,
fld1: ((f64, *mut i32), *mut bool, *mut i32),

},
Variant1{
fld0: *mut u32,
fld1: *const usize,
fld2: u128,
fld3: (u8, i8, f64, (u8, i32, bool), i16),
fld4: [bool; 2],
fld5: Adt43,
fld6: i64,
fld7: (i16, [i8; 1], char, *mut i32),

},
Variant2{
fld0: i64,
fld1: i16,
fld2: i32,
fld3: (f64, *mut i32),

},
Variant3{
fld0: [i8; 1],
fld1: ((i32, i128), *const i32),
fld2: [i128; 2],
fld3: ((f64, *mut i32), *mut bool, *mut i32),
fld4: [isize; 7],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u32,
fld1: (f64, *mut i32),

},
Variant1{
fld0: f32,
fld1: ((i32, i128), *const i32),
fld2: *const i32,
fld3: i8,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u64,
fld1: [isize; 7],
fld2: *const i32,

},
Variant1{
fld0: u32,

},
Variant2{
fld0: [isize; 7],
fld1: Adt45,
fld2: f32,

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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt46,
fld1: ((f64, *mut i32), *mut bool, *mut i32),
fld2: u32,
fld3: (f64, *mut i32),

},
Variant1{
fld0: [bool; 2],
fld1: (f64, *mut i32),
fld2: u64,
fld3: *const usize,
fld4: (u8, i8, f64, (u8, i32, bool), i16),
fld5: *const i32,
fld6: *mut bool,
fld7: (u8, i32, bool),

},
Variant2{
fld0: [i32; 5],
fld1: *const (i32, i128),
fld2: ((f64, *mut i32), *mut bool, *mut i32),
fld3: u8,
fld4: (u8, i8, f64, (u8, i32, bool), i16),
fld5: *mut i32,
fld6: *const i128,
fld7: f64,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u8,
fld1: char,
fld2: ((f64, *mut i32), *mut bool, *mut i32),
fld3: f32,
fld4: Adt45,

},
Variant1{
fld0: *mut *const usize,
fld1: char,
fld2: ((i32, i128), *const i32),
fld3: i16,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: ((f64, *mut i32),),
fld1: [i128; 7],
fld2: *const usize,
fld3: (i32, i128),
fld4: f64,
fld5: (i16, [i8; 1], char, *mut i32),
fld6: i64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: i8,
fld1: char,
fld2: *mut i32,
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: f64,
fld1: u64,
fld2: [i8; 1],

},
Variant1{
fld0: ((f64, *mut i32), *mut bool, *mut i32),
fld1: ((f64, *mut i32),),
fld2: u128,

},
Variant2{
fld0: (i32, i128),
fld1: char,
fld2: (f64, *mut i32),
fld3: usize,
fld4: i16,
fld5: *const usize,
fld6: [isize; 7],
fld7: u8,

},
Variant3{
fld0: bool,
fld1: u8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const (i32, i128),

},
Variant1{
fld0: f64,
fld1: Adt51,
fld2: (u8, i8, f64, (u8, i32, bool), i16),
fld3: i64,
fld4: ((f64, *mut i32), *mut bool, *mut i32),

},
Variant2{
fld0: [isize; 7],
fld1: u8,
fld2: [i128; 2],
fld3: [i128; 7],

},
Variant3{
fld0: *mut *const usize,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i128; 7],
fld1: *mut bool,
fld2: Adt44,
fld3: (i32, i128),

},
Variant1{
fld0: [i32; 5],
fld1: Adt47,
fld2: [i8; 1],
fld3: Adt45,
fld4: u32,
fld5: *mut *const usize,
fld6: ([i128; 7],),
fld7: u64,

},
Variant2{
fld0: Adt48,
fld1: u128,
fld2: isize,

},
Variant3{
fld0: f64,
fld1: char,
fld2: (u8, i8, f64, (u8, i32, bool), i16),
fld3: i8,
fld4: Adt51,
fld5: [i128; 7],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i128; 7],
fld1: ((i32, i128), *const i32),
fld2: Adt50,
fld3: (i16, [i8; 1], char, *mut i32),
fld4: ((f64, *mut i32), *mut bool, *mut i32),
fld5: i32,
fld6: *const i128,

},
Variant1{
fld0: ((i32, i128), *const i32),
fld1: *mut u32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *mut i32,
fld1: u32,
fld2: isize,
fld3: f64,
fld4: ((f64, *mut i32),),
fld5: Adt48,
fld6: (f64, *mut i32),

},
Variant1{
fld0: i128,

},
Variant2{
fld0: (usize, ((f64, *mut i32), *mut bool, *mut i32), i64),
fld1: u128,
fld2: Adt53,
fld3: Adt50,

},
Variant3{
fld0: ((f64, *mut i32), *mut bool, *mut i32),
fld1: (((f64, *mut i32),),),
fld2: Adt53,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (((f64, *mut i32),),),

},
Variant1{
fld0: Adt52,
fld1: [i8; 1],
fld2: [i32; 5],

},
Variant2{
fld0: Adt50,
fld1: *const i128,
fld2: isize,
fld3: *const (i32, i128),
fld4: Adt47,
fld5: i32,
fld6: Adt44,

},
Variant3{
fld0: Adt49,
fld1: u16,
fld2: *mut u32,
fld3: Adt53,
fld4: f64,
fld5: *const i32,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *mut u32,

},
Variant1{
fld0: *const (i32, i128),
fld1: Adt43,
fld2: usize,
fld3: ([i128; 7],),
fld4: Adt46,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: (u8, i32, bool),
fld1: f32,
fld2: *const (i32, i128),

},
Variant1{
fld0: *mut *const usize,
fld1: *mut i32,
fld2: u32,
fld3: [isize; 7],
fld4: u64,
fld5: i128,
fld6: Adt54,

},
Variant2{
fld0: Adt57,
fld1: char,
fld2: (u8, i8, f64, (u8, i32, bool), i16),
fld3: u64,
fld4: f32,
fld5: Adt50,
fld6: [i128; 2],
fld7: *mut *const usize,

},
Variant3{
fld0: ((f64, *mut i32),),
fld1: char,
fld2: f32,
fld3: Adt49,
fld4: Adt46,
fld5: f64,
fld6: *mut bool,
fld7: [i32; 5],

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u128,
fld1: *mut u32,
fld2: [i128; 7],

},
Variant1{
fld0: [i128; 2],
fld1: i128,
fld2: i64,

},
Variant2{
fld0: usize,
fld1: [i128; 7],
fld2: *const i128,
fld3: i8,
fld4: u16,
fld5: u32,
fld6: Adt43,

},
Variant3{
fld0: (i16, [i8; 1], char, *mut i32),
fld1: Adt43,
fld2: ((i32, i128), *const i32),
fld3: *const (i32, i128),
fld4: *mut u32,
fld5: i64,

}}

