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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u128,mut _13: u64) -> *const isize {
mir! {
type RET = *const isize;
let _14: [isize; 2];
let _15: Adt44;
let _16: ([i128; 2], [u16; 7], u64);
let _17: Adt44;
let _18: *mut char;
let _19: [i8; 3];
let _20: [i32; 8];
let _21: isize;
let _22: u8;
let _23: [i8; 3];
let _24: isize;
let _25: Adt60;
let _26: [u16; 7];
let _27: [u16; 7];
let _28: isize;
let _29: [i128; 7];
let _30: [char; 3];
let _31: [i8; 3];
let _32: i16;
let _33: Adt58;
let _34: (bool, [i32; 8], bool, i32, ((*mut u8, &'static char, bool), char, [i8; 4]), i64, usize);
let _35: [isize; 2];
let _36: Adt45;
let _37: ([i128; 2], [u16; 7], u64);
let _38: ();
let _39: ();
{
RET = core::ptr::addr_of!(_3);
_6 = 1244169152093907733_u64 as i32;
(*RET) = 15810162973135245911_u64 as isize;
_5 = 8670_i16 ^ (-25114_i16);
Call(_12 = fn1(_6, RET, RET, (*RET), _6, (*RET)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = (-78_i8) as i16;
Goto(bb2)
}
bb2 = {
_13 = 45161_u16 as u64;
_10 = _13 as u8;
(*RET) = !(-9223372036854775808_isize);
_15.fld0 = _5;
_1 = true;
(*RET) = -9223372036854775807_isize;
_6 = (-1659868157_i32) >> (*RET);
_16.1 = [14283_u16,49518_u16,45391_u16,1085_u16,48174_u16,41120_u16,36611_u16];
_4 = (-13_i8);
_16.2 = _13 | _13;
_6 = (-1712350785361954430_i64) as i32;
_8 = !(-106598809749481827384236818742161430317_i128);
_17.fld1 = [_8,_8,_8,_8,_8,_8,_8];
_6 = (-302222986_i32);
Goto(bb3)
}
bb3 = {
_14 = [(*RET),(*RET)];
(*RET) = _13 as isize;
_17.fld1 = [_8,_8,_8,_8,_8,_8,_8];
_18 = core::ptr::addr_of_mut!(_2);
_17.fld0 = _5;
_17.fld0 = _16.2 as i16;
_15.fld1 = _17.fld1;
_21 = 6_usize as isize;
_18 = core::ptr::addr_of_mut!((*_18));
_18 = core::ptr::addr_of_mut!((*_18));
_15 = Adt44 { fld0: _5,fld1: _17.fld1 };
_22 = _10;
_16.0 = [_8,_8];
Goto(bb4)
}
bb4 = {
_11 = !36813_u16;
_20 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16.2 = _13 >> _12;
_7 = !(-6716612202030607675_i64);
_13 = !_16.2;
_9 = _4 as usize;
_3 = _12 as isize;
(*_18) = '\u{64974}';
_12 = 195054941022208251705749983609280193649_u128;
_8 = !(-124354512411676628779953800687517775603_i128);
_17.fld0 = _15.fld0 | _5;
_15 = Adt44 { fld0: _5,fld1: _17.fld1 };
_8 = (-2504446708162891760854709716985387179_i128);
_5 = _11 as i16;
_12 = _1 as u128;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
337777920212775571702519897714782824277 => bb9,
_ => bb8
}
}
bb5 = {
_14 = [(*RET),(*RET)];
(*RET) = _13 as isize;
_17.fld1 = [_8,_8,_8,_8,_8,_8,_8];
_18 = core::ptr::addr_of_mut!(_2);
_17.fld0 = _5;
_17.fld0 = _16.2 as i16;
_15.fld1 = _17.fld1;
_21 = 6_usize as isize;
_18 = core::ptr::addr_of_mut!((*_18));
_18 = core::ptr::addr_of_mut!((*_18));
_15 = Adt44 { fld0: _5,fld1: _17.fld1 };
_22 = _10;
_16.0 = [_8,_8];
Goto(bb4)
}
bb6 = {
_13 = 45161_u16 as u64;
_10 = _13 as u8;
(*RET) = !(-9223372036854775808_isize);
_15.fld0 = _5;
_1 = true;
(*RET) = -9223372036854775807_isize;
_6 = (-1659868157_i32) >> (*RET);
_16.1 = [14283_u16,49518_u16,45391_u16,1085_u16,48174_u16,41120_u16,36611_u16];
_4 = (-13_i8);
_16.2 = _13 | _13;
_6 = (-1712350785361954430_i64) as i32;
_8 = !(-106598809749481827384236818742161430317_i128);
_17.fld1 = [_8,_8,_8,_8,_8,_8,_8];
_6 = (-302222986_i32);
Goto(bb3)
}
bb7 = {
_5 = (-78_i8) as i16;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_15 = Adt44 { fld0: _17.fld0,fld1: _17.fld1 };
_5 = _15.fld0 | _15.fld0;
_26 = [_11,_11,_11,_11,_11,_11,_11];
_18 = core::ptr::addr_of_mut!((*_18));
_15.fld0 = _5 ^ _17.fld0;
_14 = [(*RET),_3];
_11 = _4 as u16;
(*RET) = _21 ^ _21;
_3 = _21 & _21;
_6 = !425600489_i32;
_1 = false;
_23 = [_4,_4,_4];
(*_18) = '\u{c1d40}';
_6 = (-1782068838_i32);
_27 = [_11,_11,_11,_11,_11,_11,_11];
(*RET) = !_21;
_15 = Move(_17);
_22 = _10 + _10;
RET = core::ptr::addr_of!(_28);
_7 = 3307057475952104740_i64 - (-6467275434984246260_i64);
_5 = !_15.fld0;
_17 = Move(_15);
(*RET) = _3;
_19 = [_4,_4,_4];
(*_18) = '\u{98dde}';
_16.2 = !_13;
Call(_6 = core::intrinsics::transmute((*_18)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16.0 = [_8,_8];
_9 = 1_usize;
_29[_9] = _16.0[_9];
_16.1 = _27;
_26 = [_16.1[_9],_27[_9],_11,_27[_9],_27[_9],_27[_9],_27[_9]];
_23 = [_19[_9],_4,_19[_9]];
_15 = Adt44 { fld0: _5,fld1: _17.fld1 };
_15.fld0 = !_5;
_14 = [_3,_28];
_13 = _16.2;
_2 = '\u{803b3}';
_15.fld1 = [_17.fld1[_9],_16.0[_9],_8,_17.fld1[_9],_16.0[_9],_8,_17.fld1[_9]];
_22 = !_10;
_26[_9] = !_11;
(*RET) = _14[_9] << _15.fld0;
(*RET) = _3 - _3;
_20 = [_6,_6,_6,_6,_6,_6,_6,_6];
_27 = [_16.1[_9],_26[_9],_26[_9],_26[_9],_26[_9],_16.1[_9],_11];
_1 = false & true;
Goto(bb11)
}
bb11 = {
_34.4.0.0 = core::ptr::addr_of_mut!(_10);
_34.4.1 = (*_18);
_34.0 = !_1;
_5 = _28 as i16;
_33.fld4 = Adt45 { fld0: RET,fld1: _23[_9],fld2: _12 };
_35 = [_21,(*RET)];
_17.fld1 = _15.fld1;
Goto(bb12)
}
bb12 = {
_24 = _15.fld1[_9] as isize;
_34.4.0.2 = _34.0;
_7 = (-4679016835925589212_i64) + 3812016255000608329_i64;
_30[_9] = (*_18);
_33.fld2 = -_35[_9];
_18 = core::ptr::addr_of_mut!((*_18));
_21 = _34.0 as isize;
_15 = Move(_17);
_13 = _28 as u64;
_16.1[_9] = _7 as u16;
_17.fld1 = _15.fld1;
_23[_9] = _33.fld4.fld1 << _7;
Call(_17.fld0 = core::intrinsics::transmute(_27[_9]), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_33.fld2 = !_14[_9];
_33.fld0 = _35;
_37.1[_9] = _27[_9];
_33.fld0 = _14;
_23 = _19;
_30 = [(*_18),(*_18),(*_18)];
_13 = !_16.2;
_4 = _12 as i8;
Goto(bb14)
}
bb14 = {
_26[_9] = _15.fld0 as u16;
_36.fld0 = core::ptr::addr_of!((*RET));
_26 = [_16.1[_9],_11,_16.1[_9],_16.1[_9],_27[_9],_37.1[_9],_27[_9]];
_34.3 = _20[_9] + _20[_9];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(0_usize, 2_usize, Move(_2), 14_usize, Move(_14), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(0_usize, 3_usize, Move(_3), 4_usize, Move(_4), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(0_usize, 26_usize, Move(_26), 20_usize, Move(_20), 8_usize, Move(_8), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(0_usize, 23_usize, Move(_23), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i32,mut _2: *const isize,mut _3: *const isize,mut _4: isize,mut _5: i32,mut _6: isize) -> u128 {
mir! {
type RET = u128;
let _7: Adt59;
let _8: [i128; 2];
let _9: [i128; 2];
let _10: f32;
let _11: [char; 3];
let _12: *mut char;
let _13: f32;
let _14: Adt49;
let _15: char;
let _16: *const i8;
let _17: [i8; 3];
let _18: [isize; 2];
let _19: [i8; 4];
let _20: isize;
let _21: ([i128; 7], *const f64, ([i128; 2], [u16; 7], u64), [i8; 3], [i8; 4], *const i64, u32);
let _22: Adt47;
let _23: isize;
let _24: isize;
let _25: [usize; 7];
let _26: usize;
let _27: *const i16;
let _28: Adt53;
let _29: f32;
let _30: isize;
let _31: [i128; 7];
let _32: Adt51;
let _33: *const isize;
let _34: [i8; 3];
let _35: usize;
let _36: f64;
let _37: *const i8;
let _38: [char; 3];
let _39: Adt44;
let _40: u32;
let _41: f32;
let _42: isize;
let _43: ();
let _44: ();
{
_4 = -(*_3);
RET = 318336194068490966448582636223956445494_u128 - 160124940035205681999819992949485829069_u128;
_5 = !_1;
(*_2) = _4 << _5;
_4 = 1305670969_u32 as isize;
(*_3) = _6 | _6;
(*_2) = _4 >> RET;
_7.fld3.fld4 = [_4,(*_3)];
(*_2) = _6;
Goto(bb1)
}
bb1 = {
_7.fld3.fld2 = [12239131553905869679_usize,7561293817381674747_usize,7_usize,16856734616406382925_usize,17132189076257351707_usize,16563913808990653233_usize,57688777868880152_usize];
_7.fld2 = [14181_u16,7673_u16,30761_u16,39329_u16,62002_u16,20430_u16,54471_u16];
_7.fld3.fld1 = core::ptr::addr_of!(_7.fld6);
_7.fld3.fld1 = core::ptr::addr_of!(_7.fld6);
RET = 48239104445832038785687982388146706973_u128 & 231659814402590013246251445155170677170_u128;
Call(_7.fld3.fld5 = fn2((*_2), (*_2), (*_3), _7.fld3.fld4, _3, _7.fld3.fld4, (*_2), _7.fld3.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.fld6 = 30866_u16 as f64;
_7.fld3.fld1 = core::ptr::addr_of!(_7.fld6);
RET = 292025230206655021182568291514744191306_u128 & 164085244726869985443543988458650976694_u128;
_7.fld6 = 2570141460_u32 as f64;
_2 = _3;
_1 = _5;
(*_3) = 8923188376002640098_u64 as isize;
(*_3) = 3355367338_u32 as isize;
_8 = [(-153573126605543821716755209759340591792_i128),99473649662793572486484389687450427775_i128];
_7.fld3.fld2 = [2_usize,2_usize,14282007393403178562_usize,5263825268799814025_usize,5_usize,3_usize,3349009074015615779_usize];
_7.fld4 = 15064_u16 as u64;
(*_3) = -_4;
_5 = -_1;
_6 = _4;
_7.fld3.fld4 = [(*_3),(*_3)];
_7.fld4 = !5338432748129466208_u64;
_7.fld3.fld3.0 = [85132753802750014256801046476598531668_i128,81984434320742739943768284106831238886_i128,132531840283944694196423025007611074522_i128,(-48893912097589462073411524040657068907_i128),22724624357859985924089881689716454967_i128,(-127462900521968593647520990042725974618_i128),(-168278303088688602883835354221393610563_i128)];
_3 = _2;
(*_3) = _6 - _4;
_2 = _3;
_4 = (*_2) + (*_2);
(*_2) = _6 + _4;
_7.fld0 = [(*_2),_4];
RET = 329712233065525147443203316592176129650_u128 ^ 180244638438167800863453387071966228284_u128;
_2 = core::ptr::addr_of!((*_3));
_10 = _7.fld6 as f32;
Goto(bb3)
}
bb3 = {
_4 = (*_2);
_7.fld3.fld2 = [6_usize,6587034422801769048_usize,6742237645179017430_usize,15258409706947972497_usize,13654889849465399102_usize,11967474144606838293_usize,4_usize];
_1 = 74_i8 as i32;
_2 = core::ptr::addr_of!((*_3));
_9 = [(-46417365079266910177265443631213481947_i128),121888738842032353587726477665811776833_i128];
_7.fld3.fld0 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
_7.fld3.fld3.0 = [52303149410689452919342785322398312129_i128,5871494909401932518723772044524361129_i128,105349717520871899023755699764542894059_i128,137447051948702218036850027878959616347_i128,(-67147479086143360054373721772776378269_i128),(-27457112321397153345851293404693306745_i128),67859841183147117402512596808393899372_i128];
_8 = [(-3059471009714012350805733847533746574_i128),26852284880825738505852967158326303951_i128];
_7.fld2 = [317_u16,3275_u16,40564_u16,41882_u16,34388_u16,6848_u16,32372_u16];
_2 = core::ptr::addr_of!((*_2));
(*_3) = false as isize;
_7.fld3.fld0 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
RET = !279664224430619370257702527000632139662_u128;
_11 = ['\u{9f23b}','\u{538da}','\u{91433}'];
_8 = [(-75987400783682895017273076438753106563_i128),37709131711474171225427381807958004741_i128];
_7.fld3.fld2 = [1_usize,2_usize,2_usize,12535930306533754352_usize,7156252739737798709_usize,3488900391020862096_usize,3669416511851896962_usize];
_3 = core::ptr::addr_of!((*_3));
_1 = _5;
_7.fld6 = _7.fld4 as f64;
_5 = _1;
_7.fld3.fld4 = [_4,_4];
_2 = core::ptr::addr_of!((*_3));
RET = 53397081178222791814858358703717013589_u128 + 198260431733822588004192802408976790242_u128;
_9 = [117818229555851641713092403719578506280_i128,(-165665259488940867686662318968512645102_i128)];
RET = !86027641534786606347460788770795603095_u128;
_7.fld2 = [8052_u16,50299_u16,46866_u16,27824_u16,2194_u16,60247_u16,11160_u16];
(*_2) = !_4;
_13 = _10;
RET = 265539784747428900011090176033653369283_u128;
_7.fld2 = [52730_u16,11199_u16,13372_u16,9816_u16,65059_u16,9976_u16,26353_u16];
Goto(bb4)
}
bb4 = {
(*_3) = !_4;
(*_2) = _6;
_7.fld0 = _7.fld3.fld4;
_5 = _1 << _4;
Goto(bb5)
}
bb5 = {
_10 = 9056187644664341815_i64 as f32;
(*_2) = _4 | _4;
_13 = _10 - _10;
_7.fld3.fld4 = _7.fld0;
_1 = 17_u8 as i32;
_6 = !(*_3);
_15 = '\u{cac6d}';
_7.fld3.fld4 = _7.fld0;
_17 = [49_i8,(-87_i8),(-117_i8)];
_12 = core::ptr::addr_of_mut!(_15);
_11 = [(*_12),(*_12),(*_12)];
_1 = _5 - _5;
match RET {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
265539784747428900011090176033653369283 => bb9,
_ => bb8
}
}
bb6 = {
(*_3) = !_4;
(*_2) = _6;
_7.fld0 = _7.fld3.fld4;
_5 = _1 << _4;
Goto(bb5)
}
bb7 = {
_4 = (*_2);
_7.fld3.fld2 = [6_usize,6587034422801769048_usize,6742237645179017430_usize,15258409706947972497_usize,13654889849465399102_usize,11967474144606838293_usize,4_usize];
_1 = 74_i8 as i32;
_2 = core::ptr::addr_of!((*_3));
_9 = [(-46417365079266910177265443631213481947_i128),121888738842032353587726477665811776833_i128];
_7.fld3.fld0 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
_7.fld3.fld3.0 = [52303149410689452919342785322398312129_i128,5871494909401932518723772044524361129_i128,105349717520871899023755699764542894059_i128,137447051948702218036850027878959616347_i128,(-67147479086143360054373721772776378269_i128),(-27457112321397153345851293404693306745_i128),67859841183147117402512596808393899372_i128];
_8 = [(-3059471009714012350805733847533746574_i128),26852284880825738505852967158326303951_i128];
_7.fld2 = [317_u16,3275_u16,40564_u16,41882_u16,34388_u16,6848_u16,32372_u16];
_2 = core::ptr::addr_of!((*_2));
(*_3) = false as isize;
_7.fld3.fld0 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
RET = !279664224430619370257702527000632139662_u128;
_11 = ['\u{9f23b}','\u{538da}','\u{91433}'];
_8 = [(-75987400783682895017273076438753106563_i128),37709131711474171225427381807958004741_i128];
_7.fld3.fld2 = [1_usize,2_usize,2_usize,12535930306533754352_usize,7156252739737798709_usize,3488900391020862096_usize,3669416511851896962_usize];
_3 = core::ptr::addr_of!((*_3));
_1 = _5;
_7.fld6 = _7.fld4 as f64;
_5 = _1;
_7.fld3.fld4 = [_4,_4];
_2 = core::ptr::addr_of!((*_3));
RET = 53397081178222791814858358703717013589_u128 + 198260431733822588004192802408976790242_u128;
_9 = [117818229555851641713092403719578506280_i128,(-165665259488940867686662318968512645102_i128)];
RET = !86027641534786606347460788770795603095_u128;
_7.fld2 = [8052_u16,50299_u16,46866_u16,27824_u16,2194_u16,60247_u16,11160_u16];
(*_2) = !_4;
_13 = _10;
RET = 265539784747428900011090176033653369283_u128;
_7.fld2 = [52730_u16,11199_u16,13372_u16,9816_u16,65059_u16,9976_u16,26353_u16];
Goto(bb4)
}
bb8 = {
_7.fld6 = 30866_u16 as f64;
_7.fld3.fld1 = core::ptr::addr_of!(_7.fld6);
RET = 292025230206655021182568291514744191306_u128 & 164085244726869985443543988458650976694_u128;
_7.fld6 = 2570141460_u32 as f64;
_2 = _3;
_1 = _5;
(*_3) = 8923188376002640098_u64 as isize;
(*_3) = 3355367338_u32 as isize;
_8 = [(-153573126605543821716755209759340591792_i128),99473649662793572486484389687450427775_i128];
_7.fld3.fld2 = [2_usize,2_usize,14282007393403178562_usize,5263825268799814025_usize,5_usize,3_usize,3349009074015615779_usize];
_7.fld4 = 15064_u16 as u64;
(*_3) = -_4;
_5 = -_1;
_6 = _4;
_7.fld3.fld4 = [(*_3),(*_3)];
_7.fld4 = !5338432748129466208_u64;
_7.fld3.fld3.0 = [85132753802750014256801046476598531668_i128,81984434320742739943768284106831238886_i128,132531840283944694196423025007611074522_i128,(-48893912097589462073411524040657068907_i128),22724624357859985924089881689716454967_i128,(-127462900521968593647520990042725974618_i128),(-168278303088688602883835354221393610563_i128)];
_3 = _2;
(*_3) = _6 - _4;
_2 = _3;
_4 = (*_2) + (*_2);
(*_2) = _6 + _4;
_7.fld0 = [(*_2),_4];
RET = 329712233065525147443203316592176129650_u128 ^ 180244638438167800863453387071966228284_u128;
_2 = core::ptr::addr_of!((*_3));
_10 = _7.fld6 as f32;
Goto(bb3)
}
bb9 = {
_7.fld6 = _7.fld4 as f64;
_7.fld2 = [63796_u16,65461_u16,25610_u16,57991_u16,50825_u16,56312_u16,38481_u16];
_11 = [(*_12),_15,(*_12)];
_6 = -(*_3);
_11 = [_15,_15,_15];
_7.fld4 = 12579435009032149479_u64 | 9027472434675849183_u64;
_6 = !(*_2);
_7.fld3.fld4 = [_6,(*_2)];
_8 = [(-66432226202164783017381036208354333115_i128),83180702409278701942144833582078156824_i128];
_18 = [(*_2),(*_2)];
_15 = '\u{9ec2c}';
_3 = core::ptr::addr_of!(_6);
_12 = core::ptr::addr_of_mut!(_15);
_19 = [(-84_i8),(-54_i8),(-108_i8),40_i8];
_18 = [(*_2),_6];
_1 = _5;
_7.fld3.fld1 = core::ptr::addr_of!(_7.fld6);
(*_3) = (*_2) - (*_2);
_7.fld3.fld3.0 = [111574769507458469114403239461214097533_i128,26930355351492360871272902506522594171_i128,(-42031184321504368504549322311709817973_i128),30078903193555697751400476967064014411_i128,64660785146073789689262352368898552074_i128,(-68570570649367834794741791759803977849_i128),(-69528907448543620992297860408722534457_i128)];
_12 = core::ptr::addr_of_mut!((*_12));
_5 = _1 | _1;
_7.fld3.fld4 = [(*_2),(*_2)];
_2 = _3;
_21.4 = _19;
Goto(bb10)
}
bb10 = {
_21.0 = [(-95109279213297229809495963365029093316_i128),27238105104510686333175274396126332389_i128,(-78224205365932375315007637089226407770_i128),(-137860518743819880747555197499766921233_i128),(-143705207950436009400560500814175463082_i128),(-70652445884586385511598048835188976493_i128),7914425214049541170335940311163787992_i128];
_7.fld3.fld2 = [2_usize,0_usize,9988264283329323991_usize,6_usize,1_usize,5_usize,3_usize];
_21.2.2 = _7.fld4 >> _6;
_17 = [(-124_i8),38_i8,106_i8];
_9 = [(-165101858363544820266589873390139230360_i128),122939713030114542538530469012530297153_i128];
_3 = core::ptr::addr_of!((*_3));
(*_2) = -_4;
_21.2.0 = [45036552845728494653776532140508605087_i128,(-17067390590694003812611681595493845658_i128)];
_9 = [128882439965682025273338272025176794913_i128,(-106934277066530351669860151225919044087_i128)];
_4 = (-48160332565261138861241543804745225814_i128) as isize;
_7.fld3.fld0 = [_21.2.2,_21.2.2,_21.2.2,_21.2.2,_21.2.2,_21.2.2];
_7.fld4 = _21.2.2 >> _1;
_24 = !_6;
(*_2) = _24 ^ _24;
Goto(bb11)
}
bb11 = {
_21.2 = (_8, _7.fld2, _7.fld4);
_32.fld3 = (_21.0,);
(*_3) = !_24;
_29 = (*_2) as f32;
_31 = [(-80173402517386503327143051598443212698_i128),(-130949962836343914265635966354464760027_i128),1178186456671812519668509437637963273_i128,64082310759369048819905625077466471533_i128,120696846849797072733298429545941558042_i128,(-98502924359609705033253879836270319253_i128),(-46240917616881513816163413860428247997_i128)];
_32.fld1 = core::ptr::addr_of!(_7.fld6);
_4 = _7.fld4 as isize;
_8 = _21.2.0;
_30 = !_4;
_21.2.1 = [10156_u16,14157_u16,28541_u16,42433_u16,35042_u16,33646_u16,16338_u16];
_6 = _4;
_23 = (*_2);
_9 = [46331381670209309958764716015627052978_i128,(-120404893446746908399084996401794860681_i128)];
_21.6 = (-8605318133086024612_i64) as u32;
_21.1 = _32.fld1;
(*_3) = _30 * _4;
_32.fld1 = core::ptr::addr_of!(_7.fld6);
_9 = [76145576482255097440036137682828953011_i128,68776259137963697324759193254292621494_i128];
_21.6 = 3766369511_u32 + 1122609777_u32;
_7.fld4 = _21.6 as u64;
_21.6 = _29 as u32;
_21.6 = !3945909879_u32;
Call(_27 = core::intrinsics::arith_offset(_7.fld3.fld5, (-9223372036854775808_isize)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24 = _6;
_25 = [13805529467482523562_usize,4_usize,6_usize,0_usize,1_usize,3_usize,6_usize];
_21.2.0 = _8;
(*_12) = '\u{b5235}';
_32.fld0 = [_21.2.2,_7.fld4,_21.2.2,_21.2.2,_21.2.2,_21.2.2];
_21.3 = _17;
_7.fld3 = Adt51 { fld0: _32.fld0,fld1: _32.fld1,fld2: _25,fld3: _32.fld3,fld4: _7.fld0,fld5: _27 };
_32.fld0 = _7.fld3.fld0;
_32 = Adt51 { fld0: _7.fld3.fld0,fld1: _7.fld3.fld1,fld2: _7.fld3.fld2,fld3: _7.fld3.fld3,fld4: _7.fld3.fld4,fld5: _27 };
_23 = !(*_2);
_8 = [99280923848936945560158243404772217566_i128,(-52029761566372210864075700432421801599_i128)];
_17 = [(-5_i8),30_i8,119_i8];
(*_12) = '\u{b237a}';
_3 = core::ptr::addr_of!((*_3));
_13 = -_10;
_4 = RET as isize;
_22 = Adt47::Variant0 { fld0: _21.3,fld1: _2,fld2: _4,fld3: _5,fld4: RET };
Goto(bb13)
}
bb13 = {
_12 = core::ptr::addr_of_mut!((*_12));
_26 = 8774_i16 as usize;
_2 = _3;
_21.0 = _31;
place!(Field::<i32>(Variant(_22, 0), 3)) = _1 - _1;
_10 = -_29;
_32.fld4 = [(*_3),_24];
_7.fld4 = !_21.2.2;
place!(Field::<i32>(Variant(_22, 0), 3)) = (-11737_i16) as i32;
place!(Field::<i32>(Variant(_22, 0), 3)) = _1 - _1;
_21.2.1 = _7.fld2;
_21.4 = _19;
_35 = !_26;
Goto(bb14)
}
bb14 = {
_7.fld3.fld0 = [_21.2.2,_7.fld4,_21.2.2,_7.fld4,_21.2.2,_7.fld4];
_7.fld3.fld0 = [_21.2.2,_7.fld4,_7.fld4,_21.2.2,_7.fld4,_7.fld4];
_32.fld1 = _7.fld3.fld1;
_32.fld4 = [_23,(*_3)];
_17 = _21.3;
_12 = core::ptr::addr_of_mut!((*_12));
_7.fld3 = Move(_32);
_2 = core::ptr::addr_of!(_30);
_32 = Move(_7.fld3);
_40 = _21.6;
_21.6 = _15 as u32;
_21.2 = (_9, _7.fld2, _7.fld4);
SetDiscriminant(_22, 0);
_38 = [(*_12),(*_12),(*_12)];
_21.3 = [52_i8,(-106_i8),36_i8];
place!(Field::<*const isize>(Variant(_22, 0), 1)) = core::ptr::addr_of!(_24);
_7.fld6 = (-3151216214445662691_i64) as f64;
_21.2.0 = _9;
_7.fld3.fld2 = [_26,_26,_26,_26,_35,_26,_26];
_21.1 = core::ptr::addr_of!(_7.fld6);
_21.2.2 = _7.fld4;
_35 = _26;
RET = 121226852367249960300602934824833194465_u128 * 95604674359456768498726750588220854993_u128;
RET = !35700148789956003409932247752799443336_u128;
_6 = 38_u8 as isize;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(1_usize, 30_usize, Move(_30), 38_usize, Move(_38), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(1_usize, 5_usize, Move(_5), 4_usize, Move(_4), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(1_usize, 23_usize, Move(_23), 24_usize, Move(_24), 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 2],mut _5: *const isize,mut _6: [isize; 2],mut _7: isize,mut _8: [usize; 7]) -> *const i16 {
mir! {
type RET = *const i16;
let _9: [i128; 7];
let _10: isize;
let _11: [i32; 8];
let _12: bool;
let _13: *const i16;
let _14: [u16; 7];
let _15: isize;
let _16: char;
let _17: f32;
let _18: [i8; 4];
let _19: (bool, [i32; 8], bool, i32, ((*mut u8, &'static char, bool), char, [i8; 4]), i64, usize);
let _20: ([i128; 2], [u16; 7], u64);
let _21: [u16; 7];
let _22: isize;
let _23: ([i128; 2], [u16; 7], u64);
let _24: &'static char;
let _25: [i128; 7];
let _26: *const f64;
let _27: isize;
let _28: Adt44;
let _29: bool;
let _30: isize;
let _31: f64;
let _32: i32;
let _33: Adt44;
let _34: Adt44;
let _35: u64;
let _36: bool;
let _37: i8;
let _38: i128;
let _39: f32;
let _40: &'static char;
let _41: i16;
let _42: ([i128; 7],);
let _43: Adt45;
let _44: [char; 3];
let _45: [i128; 7];
let _46: bool;
let _47: Adt48;
let _48: u128;
let _49: Adt51;
let _50: isize;
let _51: bool;
let _52: f64;
let _53: u128;
let _54: char;
let _55: Adt57;
let _56: ();
let _57: ();
{
_1 = (*_5);
_7 = (*_5);
_9 = [(-49374172288181264344993962288622593669_i128),(-135446435291952591555933298022463435017_i128),(-168762169563508019247135125794253486438_i128),(-103392865723242291205246651514394897985_i128),127435997747288920483490122702847666701_i128,134417888897806433717507847505119900438_i128,(-46082768656262488344904634758708130227_i128)];
_3 = (*_5) >> _7;
(*_5) = 3240_i16 as isize;
(*_5) = !_2;
(*_5) = _3 & _3;
(*_5) = _3 & _2;
_8 = [16430271686828526346_usize,7174537259108509238_usize,12041001826798643771_usize,8157997619055814177_usize,0_usize,10310747426667032191_usize,7_usize];
_11 = [1176706496_i32,(-921473706_i32),690907216_i32,(-414767302_i32),971046064_i32,(-87629262_i32),254988161_i32,1761771141_i32];
_5 = core::ptr::addr_of!(_3);
_10 = (*_5);
_3 = _10;
_3 = _7;
_5 = core::ptr::addr_of!((*_5));
_10 = _7;
_1 = !_2;
(*_5) = _7 - _2;
_1 = !_3;
_10 = (*_5) + _1;
_12 = (*_5) <= _3;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_5) = -_1;
_4 = [_1,_1];
(*_5) = !_10;
_4 = _6;
_3 = 4_usize as isize;
_5 = core::ptr::addr_of!(_1);
_9 = [116440749770491667454416353381588957403_i128,78707268897648949840893669264999841490_i128,(-140438197125212471154773702072370965894_i128),(-32897323926304237606924887720675854706_i128),150548173415665200522095595249425627756_i128,(-103235895349146191723009247183197321273_i128),148627346134334662460097799535967155872_i128];
_6 = [_10,_10];
_3 = _7;
_2 = _1;
_10 = _1 & _3;
_4 = _6;
_14 = [18235_u16,33861_u16,62520_u16,25218_u16,31466_u16,9695_u16,63345_u16];
_5 = core::ptr::addr_of!(_1);
_10 = 1969682700_i32 as isize;
_3 = -_10;
_7 = _10;
_3 = 0_usize as isize;
_10 = _1 | (*_5);
_15 = (*_5) + (*_5);
(*_5) = _15;
_16 = '\u{275a0}';
_6 = _4;
(*_5) = 95856848185242322630823230333142314762_u128 as isize;
_3 = !_15;
Call(_6 = fn3(_8, _9, _10, _3, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = 16544682002207161788_usize as f32;
_3 = _10;
_7 = 284210783967830400358451391758576807943_u128 as isize;
_12 = !false;
_19.6 = !14001815726512239398_usize;
_12 = !true;
_19.4.1 = _16;
_15 = _3 & _1;
_19.4.1 = _16;
_19.4.0.2 = _12;
_3 = !(*_5);
_18 = [61_i8,(-98_i8),54_i8,106_i8];
_3 = -_10;
_18 = [12_i8,(-92_i8),13_i8,(-15_i8)];
_19.3 = -1062394614_i32;
_2 = (*_5);
_19.0 = _3 == _3;
_20.2 = _10 as u64;
_20.1 = [18499_u16,14577_u16,60574_u16,25701_u16,57324_u16,27221_u16,52897_u16];
_19.2 = !_12;
_19.0 = _12 | _12;
_1 = _3;
Goto(bb3)
}
bb3 = {
_10 = _15 - _1;
_19.4.2 = [(-104_i8),(-38_i8),2_i8,48_i8];
_8 = [_19.6,_19.6,_19.6,_19.6,_19.6,_19.6,_19.6];
_5 = core::ptr::addr_of!(_15);
_19.4.0.1 = &_19.4.1;
_19.4.1 = _16;
_20.0 = [118961031668539652922863075240767679634_i128,(-133256370085986590868458891788224569711_i128)];
_19.4.0.1 = &_16;
_3 = !_15;
_11 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_19.4.0.1 = &_19.4.1;
_14 = [53098_u16,3127_u16,3104_u16,5876_u16,55039_u16,13333_u16,33584_u16];
_7 = _19.2 as isize;
_19.6 = 1_usize >> _20.2;
_21 = [40371_u16,32846_u16,60395_u16,25961_u16,62988_u16,9326_u16,13803_u16];
Goto(bb4)
}
bb4 = {
_6 = [(*_5),(*_5)];
_18 = _19.4.2;
_11 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_19.3 = (-1317502699_i32) & (-1252089082_i32);
_9 = [140736950699763377730238881206064022679_i128,(-120118793659937197077263434037850214177_i128),(-168805142723414699490277397576071627021_i128),61665253109491097076247681897347521642_i128,(-47667381259921057336466930572249471769_i128),(-74558534249243006881908710789946942464_i128),17661567125527933074498571753330860291_i128];
_19.0 = _19.2;
_12 = _19.4.0.2;
_20.1 = [52116_u16,15280_u16,27482_u16,39134_u16,5247_u16,17849_u16,1632_u16];
_4 = [(*_5),_15];
_11 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_19.5 = -(-305660147687922531_i64);
_9 = [140197411104715088910636291248505555519_i128,(-125208844703618418264112114280199676088_i128),(-146300932434808082040541595232975706296_i128),(-143661693531398950863353518938712556652_i128),135231408012826858499455182656072485917_i128,(-122085923695664977305999079867412483692_i128),23772879681620106530830042953611527828_i128];
_19.0 = !_19.2;
RET = core::ptr::addr_of!(_28.fld0);
_23.0 = [(-14491500292782472884453573761249955652_i128),111164169258847236784479570054370492017_i128];
_29 = _10 > _3;
Goto(bb5)
}
bb5 = {
_23 = _20;
_19.1 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_19.0 = _29;
(*RET) = 30089_i16;
_1 = _19.0 as isize;
_23.2 = !_20.2;
_19.4.0.1 = &_16;
_20.1 = _21;
_24 = &_19.4.1;
_23 = (_20.0, _21, _20.2);
RET = core::ptr::addr_of!(_28.fld0);
_15 = _1;
_16 = (*_24);
_9 = [(-143002328203746203130638223482084895556_i128),78048753944645617042559801537123747840_i128,(-141665887589622900462860728137969974710_i128),52868254413748326830819366177404860994_i128,63913903512958576028346315095214242493_i128,131108311425781737314647889983682414133_i128,89191504447023856859812100338748541361_i128];
_9 = [(-28921523761607767590753641848197620148_i128),(-127282980489497969798124653205704827871_i128),110931076911546346925322009406807281606_i128,(-140657277738204506998650892449469309036_i128),(-46492132332167573437792977193600988707_i128),133591280572017260802748052483633210580_i128,(-119834205922543036784121240056427221220_i128)];
RET = core::ptr::addr_of!(_28.fld0);
_21 = _14;
_28.fld0 = 17333_i16 + 5597_i16;
_23.1 = [8172_u16,43088_u16,61944_u16,39255_u16,20924_u16,50474_u16,47978_u16];
_21 = [21695_u16,49524_u16,38960_u16,60567_u16,31916_u16,46098_u16,61822_u16];
_15 = !_10;
Goto(bb6)
}
bb6 = {
(*RET) = _10 as i16;
_20.1 = [49052_u16,2035_u16,2908_u16,8292_u16,53663_u16,11159_u16,12698_u16];
_19.6 = 3_usize ^ 1_usize;
(*RET) = 15860_i16 - (-22836_i16);
_2 = !_1;
_10 = (*_5);
(*RET) = _19.3 as i16;
(*RET) = !27824_i16;
_28.fld0 = _29 as i16;
_22 = _2;
_22 = (*_24) as isize;
_19.6 = 4_usize - 1_usize;
_20.0 = [(-120535673921159534476107604374961138498_i128),86360600943523228726086942266534123755_i128];
_2 = (*_5);
RET = core::ptr::addr_of!((*RET));
_14 = [27379_u16,58824_u16,48519_u16,54469_u16,7061_u16,60396_u16,4256_u16];
_13 = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_28.fld0);
_19.4.0.2 = _12;
(*_13) = 26_i8 as i16;
_19.1 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_7 = _15 + (*_5);
RET = core::ptr::addr_of!((*_13));
_34.fld0 = (*_13) * (*RET);
_28 = Adt44 { fld0: _34.fld0,fld1: _9 };
Goto(bb7)
}
bb7 = {
_14 = [23097_u16,58542_u16,25920_u16,42536_u16,55590_u16,31276_u16,25912_u16];
_31 = (-91365048485496616646067985323743554271_i128) as f64;
_19.1 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_25 = [129129379214004153564124919721393321902_i128,(-100193990970167212518380770931412454035_i128),(-64332110232305000442605367256891873369_i128),(-19885364172933437945416323164290015581_i128),(-41828996776440917497604374350559242432_i128),(-45865604496962044896241218017886971099_i128),(-15037505911500262338412251037574070203_i128)];
_7 = (*_5);
_33.fld0 = (*_13);
_34.fld1 = _9;
_19.2 = _19.0 & _19.0;
_33.fld1 = [(-63965923005110457184563756228059250672_i128),25453094740909248474968280093754262408_i128,(-168605966372028862359846127355153452870_i128),101330560697667191580880736367887633308_i128,41536881386735505479962011330822684834_i128,166511268739517876755529005383060231468_i128,133005969449814203394688961003053795906_i128];
_21 = [14764_u16,25598_u16,10131_u16,20111_u16,3523_u16,19118_u16,23290_u16];
_28.fld1 = _34.fld1;
_7 = _1 + (*_5);
_11 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_29 = _19.0;
_23.0 = [(-81516910066430251873662612308464196648_i128),(-53505375095400232759805538513777677429_i128)];
_20.1 = [2936_u16,57316_u16,44987_u16,12493_u16,63541_u16,17979_u16,31144_u16];
_32 = _19.3 * _19.3;
_19.4.0.2 = _19.2 == _19.0;
(*RET) = _34.fld0 & _34.fld0;
(*_13) = _34.fld0;
Goto(bb8)
}
bb8 = {
_36 = _19.2;
_37 = 105246233681607486329631076155289123772_i128 as i8;
(*RET) = -_33.fld0;
_24 = &_19.4.1;
_43.fld1 = _37 + _37;
_44 = [_19.4.1,(*_24),(*_24)];
_44 = [(*_24),(*_24),_19.4.1];
_11 = [_32,_19.3,_32,_32,_32,_32,_32,_19.3];
_39 = -_17;
_6 = [_1,_1];
_30 = _7 & (*_5);
_15 = _39 as isize;
Goto(bb9)
}
bb9 = {
_33 = Move(_28);
(*_5) = _7 | _7;
_43.fld2 = 132685187028133279329660395692554931839_u128 + 74058004973201275519343174868082623518_u128;
_38 = (-99406720101532081640364004171753578423_i128);
_5 = core::ptr::addr_of!(_15);
(*_13) = _33.fld0;
_23.0 = [_38,_38];
_30 = (*_5) >> (*RET);
_19.3 = _43.fld2 as i32;
_19.4.0.1 = &(*_24);
(*RET) = -_34.fld0;
(*_5) = !_7;
_11 = _19.1;
_34 = Adt44 { fld0: (*_13),fld1: _33.fld1 };
_17 = -_39;
_47 = Adt48::Variant1 { fld0: 2469890033_u32,fld1: _32,fld2: _44,fld3: _43.fld2,fld4: (*RET) };
_22 = (*_24) as isize;
_38 = 141576614818625164440870342337378907829_i128;
_7 = _2 << _30;
_24 = Move(_19.4.0.1);
_19.6 = _37 as usize;
_19.5 = 2524246090324898503_i64;
_49.fld3.0 = [_38,_38,_38,_38,_38,_38,_38];
Goto(bb10)
}
bb10 = {
_43 = Adt45 { fld0: _5,fld1: _37,fld2: Field::<u128>(Variant(_47, 1), 3) };
_45 = [_38,_38,_38,_38,_38,_38,_38];
(*RET) = _34.fld0;
_40 = &_16;
_49.fld4 = _4;
_23 = (_20.0, _14, _20.2);
_23.2 = !_20.2;
_42 = (_34.fld1,);
_35 = _20.2;
_19.1 = [_32,Field::<i32>(Variant(_47, 1), 1),_32,Field::<i32>(Variant(_47, 1), 1),Field::<i32>(Variant(_47, 1), 1),Field::<i32>(Variant(_47, 1), 1),_32,_32];
_49.fld2 = [_19.6,_19.6,_19.6,_19.6,_19.6,_19.6,_19.6];
_12 = _19.4.0.2;
_53 = _7 as u128;
(*RET) = Field::<i32>(Variant(_47, 1), 1) as i16;
_26 = core::ptr::addr_of!(_52);
_20 = _23;
(*RET) = _33.fld0 & Field::<i16>(Variant(_47, 1), 4);
_12 = _19.4.0.2 == _36;
_49.fld4 = [_7,(*_5)];
_7 = !_2;
_34.fld1 = [_38,_38,_38,_38,_38,_38,_38];
_8 = [_19.6,_19.6,_19.6,_19.6,_19.6,_19.6,_19.6];
_46 = _19.4.0.2;
_48 = !_53;
_33 = Adt44 { fld0: (*RET),fld1: _42.0 };
(*_5) = -_7;
(*_13) = Field::<i16>(Variant(_47, 1), 4);
match _38 {
0 => bb2,
1 => bb11,
2 => bb12,
141576614818625164440870342337378907829 => bb14,
_ => bb13
}
}
bb11 = {
_10 = _15 - _1;
_19.4.2 = [(-104_i8),(-38_i8),2_i8,48_i8];
_8 = [_19.6,_19.6,_19.6,_19.6,_19.6,_19.6,_19.6];
_5 = core::ptr::addr_of!(_15);
_19.4.0.1 = &_19.4.1;
_19.4.1 = _16;
_20.0 = [118961031668539652922863075240767679634_i128,(-133256370085986590868458891788224569711_i128)];
_19.4.0.1 = &_16;
_3 = !_15;
_11 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_19.4.0.1 = &_19.4.1;
_14 = [53098_u16,3127_u16,3104_u16,5876_u16,55039_u16,13333_u16,33584_u16];
_7 = _19.2 as isize;
_19.6 = 1_usize >> _20.2;
_21 = [40371_u16,32846_u16,60395_u16,25961_u16,62988_u16,9326_u16,13803_u16];
Goto(bb4)
}
bb12 = {
_17 = 16544682002207161788_usize as f32;
_3 = _10;
_7 = 284210783967830400358451391758576807943_u128 as isize;
_12 = !false;
_19.6 = !14001815726512239398_usize;
_12 = !true;
_19.4.1 = _16;
_15 = _3 & _1;
_19.4.1 = _16;
_19.4.0.2 = _12;
_3 = !(*_5);
_18 = [61_i8,(-98_i8),54_i8,106_i8];
_3 = -_10;
_18 = [12_i8,(-92_i8),13_i8,(-15_i8)];
_19.3 = -1062394614_i32;
_2 = (*_5);
_19.0 = _3 == _3;
_20.2 = _10 as u64;
_20.1 = [18499_u16,14577_u16,60574_u16,25701_u16,57324_u16,27221_u16,52897_u16];
_19.2 = !_12;
_19.0 = _12 | _12;
_1 = _3;
Goto(bb3)
}
bb13 = {
(*RET) = _10 as i16;
_20.1 = [49052_u16,2035_u16,2908_u16,8292_u16,53663_u16,11159_u16,12698_u16];
_19.6 = 3_usize ^ 1_usize;
(*RET) = 15860_i16 - (-22836_i16);
_2 = !_1;
_10 = (*_5);
(*RET) = _19.3 as i16;
(*RET) = !27824_i16;
_28.fld0 = _29 as i16;
_22 = _2;
_22 = (*_24) as isize;
_19.6 = 4_usize - 1_usize;
_20.0 = [(-120535673921159534476107604374961138498_i128),86360600943523228726086942266534123755_i128];
_2 = (*_5);
RET = core::ptr::addr_of!((*RET));
_14 = [27379_u16,58824_u16,48519_u16,54469_u16,7061_u16,60396_u16,4256_u16];
_13 = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_28.fld0);
_19.4.0.2 = _12;
(*_13) = 26_i8 as i16;
_19.1 = [_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3,_19.3];
_7 = _15 + (*_5);
RET = core::ptr::addr_of!((*_13));
_34.fld0 = (*_13) * (*RET);
_28 = Adt44 { fld0: _34.fld0,fld1: _9 };
Goto(bb7)
}
bb14 = {
_28.fld1 = [_38,_38,_38,_38,_38,_38,_38];
place!(Field::<u32>(Variant(_47, 1), 0)) = 1646102376_u32;
_20.1 = [17423_u16,15002_u16,10595_u16,10124_u16,37539_u16,51542_u16,27029_u16];
_23.0 = [_38,_38];
place!(Field::<[char; 3]>(Variant(_47, 1), 2)) = [_16,(*_24),(*_24)];
_19.0 = _12;
_50 = (*_5);
_19.4.0.2 = !_46;
_47 = Adt48::Variant1 { fld0: 274519303_u32,fld1: _32,fld2: _44,fld3: _48,fld4: (*_13) };
_28.fld0 = _33.fld0 >> _53;
_36 = _19.0 | _12;
_52 = _31 * _31;
_43.fld0 = _5;
(*_5) = (*_26) as isize;
_23.1 = [65438_u16,34192_u16,47951_u16,37185_u16,47821_u16,36356_u16,3919_u16];
Goto(bb15)
}
bb15 = {
Call(_56 = dump_var(2_usize, 32_usize, Move(_32), 11_usize, Move(_11), 38_usize, Move(_38), 48_usize, Move(_48)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_56 = dump_var(2_usize, 45_usize, Move(_45), 7_usize, Move(_7), 16_usize, Move(_16), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(2_usize, 10_usize, Move(_10), 12_usize, Move(_12), 36_usize, Move(_36), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(2_usize, 3_usize, Move(_3), 15_usize, Move(_15), 21_usize, Move(_21), 46_usize, Move(_46)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(2_usize, 20_usize, Move(_20), 57_usize, _57, 57_usize, _57, 57_usize, _57), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [usize; 7],mut _2: [i128; 7],mut _3: isize,mut _4: isize,mut _5: isize) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _6: char;
let _7: Adt60;
let _8: *const f32;
let _9: isize;
let _10: ();
let _11: ();
{
RET = [_4,_3];
_6 = '\u{bb554}';
Call(_2 = fn4(_5, RET, _5, RET, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _3 + _5;
_6 = '\u{3f8b2}';
_3 = _5;
_6 = '\u{2cc12}';
_3 = _4 * _5;
_2 = [156510617297980029635074936181652170119_i128,142896007846965025608623136731557976461_i128,(-113457243501230956861339977242536911476_i128),(-101614253556020150506459311983149514281_i128),(-66671726626194071482260531960073160576_i128),66703845207004854319388865516733100998_i128,(-158263393169916066644323506321717577730_i128)];
RET = [_5,_3];
_9 = _4 & _4;
RET = [_9,_9];
RET = [_3,_4];
_1 = [6_usize,5805584568876493220_usize,4_usize,1_usize,6761955096092575632_usize,14736437672882743782_usize,5_usize];
RET = [_5,_3];
_6 = '\u{4f9da}';
RET = [_3,_5];
_1 = [1_usize,2_usize,2932867959990434959_usize,17134191346185412238_usize,2_usize,17553274387928677749_usize,17825343872858758126_usize];
RET = [_9,_9];
_4 = (-4861368895213286731139097694046278732_i128) as isize;
_3 = 169425714997889811357748787764640831250_i128 as isize;
_1 = [1_usize,14371185207115731572_usize,3_usize,7_usize,4_usize,1_usize,3694613579950998808_usize];
RET = [_9,_9];
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(3_usize, 1_usize, Move(_1), 6_usize, Move(_6), 4_usize, Move(_4), 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: [isize; 2],mut _3: isize,mut _4: [isize; 2],mut _5: isize,mut _6: [usize; 7]) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _7: [usize; 7];
let _8: i16;
let _9: u16;
let _10: ([i128; 2], [u16; 7], u64);
let _11: Adt44;
let _12: f32;
let _13: isize;
let _14: [isize; 2];
let _15: ([i128; 2], [u16; 7], u64);
let _16: isize;
let _17: [i128; 7];
let _18: usize;
let _19: (*mut u8, &'static char, bool);
let _20: bool;
let _21: f32;
let _22: i64;
let _23: Adt54;
let _24: [i8; 4];
let _25: bool;
let _26: u128;
let _27: ();
let _28: ();
{
_2 = [_5,_1];
_6 = [1_usize,7_usize,6_usize,1_usize,7_usize,7519275050093024722_usize,2102411371598949172_usize];
Call(_4 = fn5(_2, _2, _6, _2, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [0_usize,6_usize,4_usize,14140985088420505575_usize,18356676897043177425_usize,1008538624609214506_usize,4_usize];
_4 = [_3,_3];
_3 = !_5;
_1 = 119_i8 as isize;
_2 = _4;
_4 = _2;
_8 = false as i16;
Goto(bb2)
}
bb2 = {
RET = [(-85560969074742735791222942497494477048_i128),(-69474274973370847010665811526873737819_i128),(-16981361604355195006062933455312447348_i128),94805657676285038070104161025478635029_i128,(-110164649152344587798915143722919528148_i128),(-108688364535905941338989253394104110917_i128),(-29405529109107940442737201900899379250_i128)];
RET = [51657828921565150125813299299005443140_i128,(-129096512406517732732528321274148570776_i128),(-106151919637326573704006045608243193089_i128),8052477225176940168784133077683092140_i128,(-107841097282536387998550386433283153964_i128),(-152899020935396884512331691332206061438_i128),143520489816458290781394459039238397862_i128];
_8 = 135246205461768546634280300792864753397_u128 as i16;
_6 = [7_usize,6_usize,8317016387639470887_usize,15831170909010552492_usize,1066240569079632248_usize,10382251232046579849_usize,1976518882853385174_usize];
_7 = [7_usize,1_usize,2211429621252437150_usize,4_usize,4_usize,3_usize,4_usize];
Goto(bb3)
}
bb3 = {
_9 = 49753_u16;
_2 = [_1,_3];
_5 = _1 + _3;
_4 = [_3,_3];
_1 = 4006813005253862045_i64 as isize;
_4 = [_3,_5];
RET = [(-121365154014104556698474898849394959990_i128),(-15850345201600934709274530355683096823_i128),(-152334956437367406851021892124025656194_i128),(-71779847201953570115845757620909064356_i128),(-163310602036827379448367283727357829208_i128),3885048684723243675370773492992593239_i128,2656189487560515935915035017273583626_i128];
RET = [68011714574150135377640459563073754499_i128,34057233776439448234055999027125817646_i128,(-63963418105977888228107963806668365140_i128),(-24804182358344801918721029523491190768_i128),72237363106524080810086080782505760610_i128,(-113852379351833331468299126025813132136_i128),126112105889316615983786910980497209304_i128];
_3 = _5;
_11.fld0 = _8;
_2 = _4;
_11 = Adt44 { fld0: _8,fld1: RET };
_2 = [_3,_3];
_12 = 9837733432110306819_usize as f32;
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
_1 = 97_u8 as isize;
_10.2 = 9652810295923284661_u64;
Goto(bb4)
}
bb4 = {
RET = _11.fld1;
Goto(bb5)
}
bb5 = {
_10.0 = [(-166184267482539194956535181955613252971_i128),(-89544670934708380045399924954738574544_i128)];
_3 = _1;
_1 = !_5;
_13 = _5;
_7 = _6;
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_11 = Adt44 { fld0: _8,fld1: RET };
_5 = _13 << _9;
Goto(bb6)
}
bb6 = {
_4 = _2;
_11.fld1 = [17083384509820048414698450292766095537_i128,132139481346591311989728268136769996618_i128,122720027443480131836316332053474113613_i128,137489002430351227036193313486936000994_i128,26504233128521715542358333006792995597_i128,(-149700363823227090527816063323255645325_i128),(-68061278454674941674081017838287063077_i128)];
RET = [95363559314178121057173290247481729119_i128,(-155433687938822529256090219513460114227_i128),(-34480168044267845289367629070978855811_i128),(-167752403460453583127392282656947462573_i128),(-141302338740043867683970274922924119732_i128),113507614906512958555746914938391668524_i128,(-131241057624418805503582346579693142371_i128)];
_4 = [_5,_13];
_15.2 = _10.2;
_7 = [4_usize,11176914336467896261_usize,1_usize,2_usize,0_usize,1_usize,8780257772647522181_usize];
_16 = _1;
_3 = _5;
_15 = (_10.0, _10.1, _10.2);
_15 = (_10.0, _10.1, _10.2);
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_15.0 = [(-58003020404877992878652444031104329994_i128),150515083887092397764404520235037047945_i128];
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
_9 = _11.fld0 as u16;
_14 = [_1,_3];
_8 = _11.fld0 * _11.fld0;
RET = [(-107314309302439094000442165773435241624_i128),139357809743372264847233920523054131044_i128,98693359156635152963818889834281073191_i128,123409988672202591941327012785563834185_i128,(-121673179119058195003708031805711970834_i128),(-92244552361064012488946515904908877677_i128),91321403259950050825238431829912034813_i128];
match _15.2 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
9652810295923284661 => bb11,
_ => bb10
}
}
bb7 = {
_10.0 = [(-166184267482539194956535181955613252971_i128),(-89544670934708380045399924954738574544_i128)];
_3 = _1;
_1 = !_5;
_13 = _5;
_7 = _6;
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_11 = Adt44 { fld0: _8,fld1: RET };
_5 = _13 << _9;
Goto(bb6)
}
bb8 = {
RET = _11.fld1;
Goto(bb5)
}
bb9 = {
_9 = 49753_u16;
_2 = [_1,_3];
_5 = _1 + _3;
_4 = [_3,_3];
_1 = 4006813005253862045_i64 as isize;
_4 = [_3,_5];
RET = [(-121365154014104556698474898849394959990_i128),(-15850345201600934709274530355683096823_i128),(-152334956437367406851021892124025656194_i128),(-71779847201953570115845757620909064356_i128),(-163310602036827379448367283727357829208_i128),3885048684723243675370773492992593239_i128,2656189487560515935915035017273583626_i128];
RET = [68011714574150135377640459563073754499_i128,34057233776439448234055999027125817646_i128,(-63963418105977888228107963806668365140_i128),(-24804182358344801918721029523491190768_i128),72237363106524080810086080782505760610_i128,(-113852379351833331468299126025813132136_i128),126112105889316615983786910980497209304_i128];
_3 = _5;
_11.fld0 = _8;
_2 = _4;
_11 = Adt44 { fld0: _8,fld1: RET };
_2 = [_3,_3];
_12 = 9837733432110306819_usize as f32;
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
_1 = 97_u8 as isize;
_10.2 = 9652810295923284661_u64;
Goto(bb4)
}
bb10 = {
_6 = [0_usize,6_usize,4_usize,14140985088420505575_usize,18356676897043177425_usize,1008538624609214506_usize,4_usize];
_4 = [_3,_3];
_3 = !_5;
_1 = 119_i8 as isize;
_2 = _4;
_4 = _2;
_8 = false as i16;
Goto(bb2)
}
bb11 = {
_13 = _3;
_11.fld1 = [102539082590021416508786136259862962630_i128,145002151220878831426251815803222487951_i128,(-128032130333604406880394084798267272456_i128),41007615117150358101386111314908584571_i128,1620089812334765013709359072735672990_i128,67913612880562281425567295257529332387_i128,2483282621201197769103537927796076080_i128];
_1 = _3;
_5 = !_16;
_15.0 = [(-91382914466828437635756091998646537685_i128),(-4299867829550476228360138042823837469_i128)];
_15.1 = _10.1;
_16 = _5;
_4 = _2;
_9 = 31891_u16;
RET = [(-116450067831946007927979968872327529354_i128),(-112248887314967607306729401706812161080_i128),(-8530557777887175605891386497924030531_i128),(-102939125663573067015743558559039085703_i128),140449480349067714607597158736738984953_i128,26903482457389011922828598093243559436_i128,6948334470715514071771807086977610917_i128];
match _10.2 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
9652810295923284661 => bb17,
_ => bb16
}
}
bb12 = {
_6 = [0_usize,6_usize,4_usize,14140985088420505575_usize,18356676897043177425_usize,1008538624609214506_usize,4_usize];
_4 = [_3,_3];
_3 = !_5;
_1 = 119_i8 as isize;
_2 = _4;
_4 = _2;
_8 = false as i16;
Goto(bb2)
}
bb13 = {
_10.0 = [(-166184267482539194956535181955613252971_i128),(-89544670934708380045399924954738574544_i128)];
_3 = _1;
_1 = !_5;
_13 = _5;
_7 = _6;
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_11 = Adt44 { fld0: _8,fld1: RET };
_5 = _13 << _9;
Goto(bb6)
}
bb14 = {
_9 = 49753_u16;
_2 = [_1,_3];
_5 = _1 + _3;
_4 = [_3,_3];
_1 = 4006813005253862045_i64 as isize;
_4 = [_3,_5];
RET = [(-121365154014104556698474898849394959990_i128),(-15850345201600934709274530355683096823_i128),(-152334956437367406851021892124025656194_i128),(-71779847201953570115845757620909064356_i128),(-163310602036827379448367283727357829208_i128),3885048684723243675370773492992593239_i128,2656189487560515935915035017273583626_i128];
RET = [68011714574150135377640459563073754499_i128,34057233776439448234055999027125817646_i128,(-63963418105977888228107963806668365140_i128),(-24804182358344801918721029523491190768_i128),72237363106524080810086080782505760610_i128,(-113852379351833331468299126025813132136_i128),126112105889316615983786910980497209304_i128];
_3 = _5;
_11.fld0 = _8;
_2 = _4;
_11 = Adt44 { fld0: _8,fld1: RET };
_2 = [_3,_3];
_12 = 9837733432110306819_usize as f32;
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
_1 = 97_u8 as isize;
_10.2 = 9652810295923284661_u64;
Goto(bb4)
}
bb15 = {
_10.0 = [(-166184267482539194956535181955613252971_i128),(-89544670934708380045399924954738574544_i128)];
_3 = _1;
_1 = !_5;
_13 = _5;
_7 = _6;
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_11 = Adt44 { fld0: _8,fld1: RET };
_5 = _13 << _9;
Goto(bb6)
}
bb16 = {
RET = [(-85560969074742735791222942497494477048_i128),(-69474274973370847010665811526873737819_i128),(-16981361604355195006062933455312447348_i128),94805657676285038070104161025478635029_i128,(-110164649152344587798915143722919528148_i128),(-108688364535905941338989253394104110917_i128),(-29405529109107940442737201900899379250_i128)];
RET = [51657828921565150125813299299005443140_i128,(-129096512406517732732528321274148570776_i128),(-106151919637326573704006045608243193089_i128),8052477225176940168784133077683092140_i128,(-107841097282536387998550386433283153964_i128),(-152899020935396884512331691332206061438_i128),143520489816458290781394459039238397862_i128];
_8 = 135246205461768546634280300792864753397_u128 as i16;
_6 = [7_usize,6_usize,8317016387639470887_usize,15831170909010552492_usize,1066240569079632248_usize,10382251232046579849_usize,1976518882853385174_usize];
_7 = [7_usize,1_usize,2211429621252437150_usize,4_usize,4_usize,3_usize,4_usize];
Goto(bb3)
}
bb17 = {
RET = [117669970661820060655670798266780327025_i128,(-5374864305824705317022097535108256647_i128),(-34631549999219611456990504477837614975_i128),(-155683177734537109514133865236168699744_i128),41105189335499005318110802956816731083_i128,112264742615707598980057852381836353400_i128,(-166066138006596354361444396745243007792_i128)];
_17 = [37083404772412762813012938621270653829_i128,8105754434149372406016468747099989854_i128,47583699193264834917152320607662055493_i128,93885866591116945687286312229086321751_i128,(-169504855718783314401307735214957218273_i128),115701041510897169746205054912900820763_i128,139512173552214858139488817473716181803_i128];
_12 = 89174809256266620716724414421063946442_i128 as f32;
_1 = -_3;
_19.2 = _1 >= _13;
_13 = _16;
_11 = Adt44 { fld0: _8,fld1: _17 };
_15 = (_10.0, _10.1, _10.2);
_15.1 = [_9,_9,_9,_9,_9,_9,_9];
_18 = 9492256286656515847_usize ^ 1510691923061377063_usize;
_20 = _19.2;
_21 = _12;
_6 = [_18,_18,_18,_18,_18,_18,_18];
_2 = [_1,_16];
_13 = 427521974_u32 as isize;
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
RET = [(-151906947041782170017797618116827690450_i128),(-151574469951139004241296089001937246216_i128),84817222909330144772645579168805768622_i128,(-115850242513047666000868037088935947494_i128),125896073033000027750946872733546201034_i128,(-148409998078596741735438548752504891157_i128),46671336351230830113241030314556271553_i128];
_10.0 = _15.0;
_10.1 = [_9,_9,_9,_9,_9,_9,_9];
_7 = _6;
_16 = _1;
_4 = _2;
_24 = [(-86_i8),(-34_i8),4_i8,(-14_i8)];
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(4_usize, 14_usize, Move(_14), 9_usize, Move(_9), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(4_usize, 8_usize, Move(_8), 3_usize, Move(_3), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_27 = dump_var(4_usize, 6_usize, Move(_6), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: [usize; 7],mut _4: [isize; 2],mut _5: [usize; 7],mut _6: isize) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _7: isize;
let _8: [i8; 3];
let _9: [u16; 7];
let _10: char;
let _11: u64;
let _12: i64;
let _13: isize;
let _14: [i128; 7];
let _15: [i128; 7];
let _16: bool;
let _17: isize;
let _18: Adt49;
let _19: isize;
let _20: bool;
let _21: bool;
let _22: i32;
let _23: i64;
let _24: f64;
let _25: i32;
let _26: [isize; 2];
let _27: [u16; 7];
let _28: [char; 3];
let _29: u8;
let _30: Adt48;
let _31: Adt55;
let _32: *const i64;
let _33: Adt60;
let _34: u128;
let _35: isize;
let _36: Adt44;
let _37: ();
let _38: ();
{
_7 = _6;
_8 = [29_i8,(-106_i8),(-6_i8)];
Goto(bb1)
}
bb1 = {
RET = _4;
_9 = [60772_u16,65219_u16,16898_u16,8968_u16,24912_u16,46940_u16,22350_u16];
_7 = 14750245326205358804_usize as isize;
RET = [_6,_6];
RET = [_6,_7];
_1 = [_6,_6];
_5 = [6_usize,12463515698157376309_usize,10151243176990099269_usize,18294172796938746433_usize,13521705962733669441_usize,6055077474257855108_usize,12916341548768470514_usize];
_4 = _2;
_4 = [_6,_6];
_5 = _3;
_3 = _5;
_2 = [_6,_6];
_10 = '\u{fcc32}';
_5 = [7922671851822481780_usize,1_usize,12610428472628078604_usize,7_usize,4_usize,4_usize,17784199058663285459_usize];
_9 = [63861_u16,1556_u16,46098_u16,16825_u16,27182_u16,3715_u16,12752_u16];
_2 = _1;
_1 = _4;
_3 = [3_usize,7_usize,2102779349627941919_usize,5_usize,2228783329356090575_usize,10601228393100671891_usize,2406769535638024321_usize];
_2 = [_6,_6];
RET = [_7,_7];
_6 = _7 >> _7;
_8 = [(-68_i8),29_i8,28_i8];
Goto(bb2)
}
bb2 = {
_11 = 328331211159805808711185465262212412826_u128 as u64;
_4 = [_6,_6];
_9 = [28090_u16,6714_u16,54890_u16,5322_u16,17377_u16,46711_u16,16696_u16];
_1 = [_7,_7];
RET = _4;
_11 = 285043773553415763521596347566989090983_u128 as u64;
_1 = _4;
Call(_12 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _12 as isize;
_2 = [_7,_6];
RET = [_7,_7];
_7 = 120_u8 as isize;
_3 = [12054471865397802754_usize,6_usize,18237616804020707967_usize,5_usize,0_usize,18075687112158870312_usize,16553474169349723710_usize];
RET = [_6,_7];
_9 = [3071_u16,3264_u16,49523_u16,53711_u16,28283_u16,26647_u16,58374_u16];
_10 = '\u{b6448}';
_12 = (-355211732333587812_i64);
_4 = _2;
_6 = (-303383576_i32) as isize;
_13 = _7;
_13 = _7 << _11;
_3 = _5;
_9 = [52982_u16,54738_u16,43633_u16,39077_u16,52647_u16,64037_u16,33354_u16];
_3 = [1_usize,10425509400057496625_usize,4_usize,18112847142354446415_usize,7_usize,13729922442499111781_usize,0_usize];
match _12 {
0 => bb2,
1 => bb4,
340282366920938463463019395699434623644 => bb6,
_ => bb5
}
}
bb4 = {
_11 = 328331211159805808711185465262212412826_u128 as u64;
_4 = [_6,_6];
_9 = [28090_u16,6714_u16,54890_u16,5322_u16,17377_u16,46711_u16,16696_u16];
_1 = [_7,_7];
RET = _4;
_11 = 285043773553415763521596347566989090983_u128 as u64;
_1 = _4;
Call(_12 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = _4;
_9 = [60772_u16,65219_u16,16898_u16,8968_u16,24912_u16,46940_u16,22350_u16];
_7 = 14750245326205358804_usize as isize;
RET = [_6,_6];
RET = [_6,_7];
_1 = [_6,_6];
_5 = [6_usize,12463515698157376309_usize,10151243176990099269_usize,18294172796938746433_usize,13521705962733669441_usize,6055077474257855108_usize,12916341548768470514_usize];
_4 = _2;
_4 = [_6,_6];
_5 = _3;
_3 = _5;
_2 = [_6,_6];
_10 = '\u{fcc32}';
_5 = [7922671851822481780_usize,1_usize,12610428472628078604_usize,7_usize,4_usize,4_usize,17784199058663285459_usize];
_9 = [63861_u16,1556_u16,46098_u16,16825_u16,27182_u16,3715_u16,12752_u16];
_2 = _1;
_1 = _4;
_3 = [3_usize,7_usize,2102779349627941919_usize,5_usize,2228783329356090575_usize,10601228393100671891_usize,2406769535638024321_usize];
_2 = [_6,_6];
RET = [_7,_7];
_6 = _7 >> _7;
_8 = [(-68_i8),29_i8,28_i8];
Goto(bb2)
}
bb6 = {
_5 = [1_usize,9722850737646779102_usize,7_usize,9194923482143982833_usize,15469067560418790393_usize,3_usize,7_usize];
_8 = [89_i8,88_i8,(-8_i8)];
RET = [_13,_13];
_10 = '\u{29459}';
_14 = [156446060884171763753694969476251013773_i128,(-165205460419495493446640494389335295925_i128),120513514255701175857963588221612049020_i128,11295901673141770476810145160683970108_i128,(-53544509573855047270109136313563776253_i128),78750087962504674044494835807304785499_i128,98381162376199234555822008384170422605_i128];
_15 = _14;
_17 = -_13;
Call(_17 = fn6(_14, _15, _3, _2, _14, _5, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7 = _13 & _13;
_9 = [58546_u16,40929_u16,40216_u16,9106_u16,53639_u16,47327_u16,22192_u16];
_13 = _17 >> _17;
_5 = [1_usize,6_usize,9712837946487086947_usize,0_usize,4_usize,7118297254339507588_usize,17089442229129146361_usize];
_6 = !_17;
_10 = '\u{7705f}';
_5 = [5_usize,3_usize,1_usize,9950771395223365301_usize,12407933404762498542_usize,7906218264770093048_usize,5_usize];
_16 = false;
match _12 {
0 => bb1,
1 => bb6,
2 => bb3,
340282366920938463463019395699434623644 => bb8,
_ => bb5
}
}
bb8 = {
_1 = [_6,_7];
_16 = _13 <= _7;
_7 = -_13;
_15 = _14;
match _12 {
340282366920938463463019395699434623644 => bb10,
_ => bb9
}
}
bb9 = {
_11 = 328331211159805808711185465262212412826_u128 as u64;
_4 = [_6,_6];
_9 = [28090_u16,6714_u16,54890_u16,5322_u16,17377_u16,46711_u16,16696_u16];
_1 = [_7,_7];
RET = _4;
_11 = 285043773553415763521596347566989090983_u128 as u64;
_1 = _4;
Call(_12 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_4 = _1;
_17 = 62677_u16 as isize;
_8 = [20_i8,(-122_i8),79_i8];
_16 = !false;
_22 = 231748264_i32;
RET = [_6,_13];
_4 = [_6,_17];
_2 = [_13,_6];
_19 = -_17;
_21 = _16;
_20 = _7 <= _19;
_1 = [_17,_19];
_10 = '\u{ef52d}';
_22 = (-1410303168_i32);
_12 = _10 as i64;
_24 = 3666997768_u32 as f64;
RET = [_6,_13];
_15 = _14;
_13 = !_6;
_15 = [(-93963227032964352028959753290747469121_i128),115238287662206827109775508081345510349_i128,(-137285428716667675272875734176226172016_i128),161779270723743892135242909544451263964_i128,26097859891519512984894819204069864981_i128,48082362177779538363078442750104269696_i128,127970587505286324379814081300985702043_i128];
_6 = _7 | _13;
_5 = _3;
_5 = _3;
_14 = [43902694972420615572138849886252005883_i128,84663113398415709218308681225564029897_i128,(-167294865781581120471338826739434169040_i128),(-96491063003056829420853576962906080280_i128),6069112227269593657281277822366368744_i128,(-4591159310352744365140561859535845060_i128),(-103557403536685612592899399354607449999_i128)];
Goto(bb11)
}
bb11 = {
_12 = (-4979595544514876520_i64);
_17 = _10 as isize;
_9 = [19692_u16,49342_u16,64348_u16,35110_u16,1920_u16,6762_u16,16096_u16];
_9 = [814_u16,29117_u16,28362_u16,14151_u16,46790_u16,28246_u16,5219_u16];
_10 = '\u{ae0c3}';
_19 = _6 - _6;
RET = [_19,_19];
_15 = _14;
match _22 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
340282366920938463463374607430357908288 => bb13,
_ => bb12
}
}
bb12 = {
_6 = _12 as isize;
_2 = [_7,_6];
RET = [_7,_7];
_7 = 120_u8 as isize;
_3 = [12054471865397802754_usize,6_usize,18237616804020707967_usize,5_usize,0_usize,18075687112158870312_usize,16553474169349723710_usize];
RET = [_6,_7];
_9 = [3071_u16,3264_u16,49523_u16,53711_u16,28283_u16,26647_u16,58374_u16];
_10 = '\u{b6448}';
_12 = (-355211732333587812_i64);
_4 = _2;
_6 = (-303383576_i32) as isize;
_13 = _7;
_13 = _7 << _11;
_3 = _5;
_9 = [52982_u16,54738_u16,43633_u16,39077_u16,52647_u16,64037_u16,33354_u16];
_3 = [1_usize,10425509400057496625_usize,4_usize,18112847142354446415_usize,7_usize,13729922442499111781_usize,0_usize];
match _12 {
0 => bb2,
1 => bb4,
340282366920938463463019395699434623644 => bb6,
_ => bb5
}
}
bb13 = {
_12 = (-6639158755173062004_i64) << _7;
_6 = _19;
_6 = !_19;
_26 = [_19,_19];
_3 = [3749781036575286864_usize,0_usize,11856302078663844963_usize,8997760049635502333_usize,1027967348871770007_usize,908231491222914172_usize,2_usize];
_15 = _14;
_20 = !_21;
_14 = [(-44848698447285987241752652701085763849_i128),39615768446019613236804995493098670708_i128,(-153676326284271176261319624371575560950_i128),(-99547461088476699981365702777304410011_i128),(-6703171574092222556272048315552271482_i128),132577104513939083250285537066439651634_i128,64397811868925580378797071121515981733_i128];
_21 = _20;
_11 = !6320782333009631905_u64;
_10 = '\u{17050}';
_27 = [49028_u16,1674_u16,36600_u16,44476_u16,53701_u16,29840_u16,56969_u16];
_3 = _5;
_10 = '\u{186ac}';
_28 = [_10,_10,_10];
_11 = 7005897710428594803_u64 - 10581388614317498853_u64;
_23 = 164982977423892847843360125238255250563_i128 as i64;
_4 = [_6,_6];
_10 = '\u{5906f}';
_10 = '\u{452ad}';
_10 = '\u{c4acc}';
_13 = -_7;
Goto(bb14)
}
bb14 = {
_19 = !_6;
_5 = [13651529621246627778_usize,6_usize,10251247263238996337_usize,3_usize,5_usize,17874274193551541897_usize,15066939952959485126_usize];
_17 = _6 - _6;
_22 = 528668807_i32 | (-1724654261_i32);
_21 = _12 > _12;
_32 = core::ptr::addr_of!(_23);
(*_32) = !_12;
_23 = _12;
_29 = !205_u8;
RET = _26;
_4 = [_17,_17];
_26 = [_19,_6];
_35 = _19;
_36.fld0 = !(-32026_i16);
_20 = _7 > _7;
_28 = [_10,_10,_10];
_2 = RET;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(5_usize, 28_usize, Move(_28), 7_usize, Move(_7), 11_usize, Move(_11), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(5_usize, 10_usize, Move(_10), 17_usize, Move(_17), 27_usize, Move(_27), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(5_usize, 23_usize, Move(_23), 14_usize, Move(_14), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(5_usize, 2_usize, Move(_2), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i128; 7],mut _2: [i128; 7],mut _3: [usize; 7],mut _4: [isize; 2],mut _5: [i128; 7],mut _6: [usize; 7],mut _7: [usize; 7]) -> isize {
mir! {
type RET = isize;
let _8: u64;
let _9: ([i128; 2], [u16; 7], u64);
let _10: isize;
let _11: &'static char;
let _12: [i128; 7];
let _13: Adt50;
let _14: [i128; 2];
let _15: [i32; 8];
let _16: bool;
let _17: ([i128; 2], [u16; 7], u64);
let _18: char;
let _19: u32;
let _20: [u16; 7];
let _21: Adt49;
let _22: u8;
let _23: isize;
let _24: u128;
let _25: [i128; 7];
let _26: f32;
let _27: *const isize;
let _28: char;
let _29: [isize; 2];
let _30: Adt51;
let _31: ([i128; 2], [u16; 7], u64);
let _32: i32;
let _33: ();
let _34: ();
{
_1 = _5;
_6 = [2_usize,4_usize,1_usize,6310725912473714694_usize,8737535689567559836_usize,13206293616763071557_usize,1_usize];
RET = (-9223372036854775808_isize) + 11_isize;
_4 = [RET,RET];
_3 = [8360936177387620697_usize,2_usize,0_usize,4978094774096271858_usize,4_usize,6_usize,0_usize];
_8 = 2_usize as u64;
_9.0 = [2592824893028031955666782059209894460_i128,73488405756214266987515189729157087368_i128];
_2 = [(-21085240627043980839672019029140916918_i128),31248633118188943512634617730151986610_i128,66696012569143680333749269880211754043_i128,(-74163011443594783709944943663263660263_i128),3864983280600657703820832230178020650_i128,(-97646830321606236935337125273073640605_i128),150711609830635484614360743778199096525_i128];
_12 = [(-23924644113710301428214761212661380261_i128),30851658827087040682476171463095588450_i128,(-138022288603212441406795673125423629677_i128),(-25403248945987942545908746757655034752_i128),(-519808048228048090649621766438471008_i128),(-89369887179945705317440824032143019835_i128),(-31032198528852009892906949918406637749_i128)];
_6 = [6_usize,14864778533843078421_usize,1_usize,5_usize,18334759994960034887_usize,11156979485049812761_usize,0_usize];
_13.fld0.6 = 3457265696_u32;
_13.fld0.4 = [86_i8,(-14_i8),46_i8,34_i8];
Goto(bb1)
}
bb1 = {
_13.fld4 = _13.fld0.4;
_13.fld0.4 = [(-115_i8),31_i8,45_i8,4_i8];
match _13.fld0.6 {
0 => bb2,
1 => bb3,
3457265696 => bb5,
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
_13.fld0.2.0 = _9.0;
_13.fld0.3 = [76_i8,0_i8,(-58_i8)];
_10 = false as isize;
_9.2 = !_8;
_12 = [(-168904984345508148101762224080153588418_i128),164014055693503906213446359549577539139_i128,56780758576012451401363125123182059171_i128,(-16851875723733797733430839909529543132_i128),(-75069846254210555242152959706804379114_i128),(-99329978942047293147819109964741161190_i128),90877836510619436613447053524837098190_i128];
_13.fld7 = [(-47_i8),(-28_i8),(-78_i8)];
RET = !_10;
_13.fld4 = _13.fld0.4;
Call(_5 = fn7(_4, _2, _3, _13.fld0.6, _12, _3, _12, _3, _13.fld4, _1, _3, RET, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.fld0.0 = [(-39540204014925594383741974149722538139_i128),57940092445755251398673759856057770370_i128,159339028583109446971336860564808766088_i128,155374813478511379584929259123925969568_i128,(-102303979967304728000761227175968858645_i128),(-32285970430541603198851382139487942126_i128),104682823021889350138780544038658512031_i128];
_13.fld1 = 5_usize << _13.fld0.6;
match _13.fld0.6 {
3457265696 => bb7,
_ => bb1
}
}
bb7 = {
_4 = [RET,RET];
_13.fld0.2.2 = true as u64;
RET = _10;
_8 = _13.fld1 as u64;
_13.fld1 = 7_usize | 3_usize;
_9.2 = _13.fld1 as u64;
_13.fld0.2.2 = _9.2 ^ _8;
_5 = [(-11171843590014704170654577109339899958_i128),10712998877379819295350387495738157692_i128,28755473089140127605383356732107019979_i128,138967947106130817171184782224819199872_i128,(-41942005499331996572488746884676595314_i128),56497926661587338624612908515387314087_i128,(-50270798234279788879646148135424094491_i128)];
_13.fld0.6 = !3187300296_u32;
_10 = -RET;
_8 = 36942_u16 as u64;
_3 = _6;
_13.fld0.4 = _13.fld4;
_8 = !_13.fld0.2.2;
_13.fld5 = ['\u{dd812}','\u{8d8d9}','\u{3c802}'];
_9.0 = _13.fld0.2.0;
_13.fld0.2.0 = [(-196166158035555353249217708298552001_i128),(-159598825093602577772031725353618118347_i128)];
_13.fld0.0 = [(-52815958945898138808213800335257680186_i128),(-162833318150257649644795288687367877112_i128),(-37434628112102181759625608362382678707_i128),86261570027742129617479468878385303841_i128,41273306092612219059145718812680180027_i128,(-18771884412056165195569748593330994925_i128),(-100731775084566346948916359092732776358_i128)];
Goto(bb8)
}
bb8 = {
_16 = true;
_17.2 = _8 - _9.2;
_8 = _13.fld0.2.2;
_13.fld4 = [21_i8,6_i8,121_i8,(-65_i8)];
_13.fld0.0 = [10612772924596126662845153984803273485_i128,(-98189864124288586066701139724752245886_i128),(-70411368833882584543973413120265644369_i128),(-103764398413487362378060070645301516126_i128),(-43944769154051439199368944231020489454_i128),(-75596153503584008715316619992713246207_i128),30081630916210004053691463484646049445_i128];
RET = -_10;
_17.1 = [29482_u16,47965_u16,49455_u16,10725_u16,28734_u16,42557_u16,41897_u16];
_3 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
RET = _10;
_15 = [(-131928885_i32),2020970460_i32,(-82970966_i32),(-1259373677_i32),722666904_i32,487651575_i32,(-201523424_i32),(-1002361992_i32)];
_1 = [12143380444993052437321781798166164199_i128,95212223138837806155847992005763510486_i128,144771985737725527831686268816704673139_i128,107178345006387662671904418673805487209_i128,(-81776267730563519931171114928589600014_i128),30545425880276410218331125745199600547_i128,111113319166629184630575160706759643623_i128];
Goto(bb9)
}
bb9 = {
_14 = [(-72651414207841466635493939701674367961_i128),66167741027234312618738194698459402969_i128];
_5 = [(-27156589206667602372292681229942893808_i128),(-53753103831104398443068770578970589346_i128),135945496135698347477880393243300327710_i128,(-71001187124990806205697966039943565899_i128),146085221869695579398966013289881393429_i128,111171630880763931206391818067567984180_i128,119659394365256408055943960829552726154_i128];
_13.fld4 = _13.fld0.4;
_16 = !false;
_19 = _13.fld0.6;
_23 = 194_u8 as isize;
_13.fld7 = _13.fld0.3;
_24 = 282125569872788103592046064368164246985_u128 | 182225847081051101782281640673651860179_u128;
_18 = '\u{7e990}';
Goto(bb10)
}
bb10 = {
_17.0 = [(-157722483951111134542711204168226812027_i128),(-127333626677982155209582532081197193346_i128)];
_17.0 = [(-102319728794447192678462683004275620031_i128),(-123563782045081588077904246313885007346_i128)];
_18 = '\u{2e343}';
_9 = (_17.0, _17.1, _17.2);
_20 = [65425_u16,42216_u16,22477_u16,29776_u16,2605_u16,37041_u16,40530_u16];
_9 = (_13.fld0.2.0, _20, _8);
_17.2 = _8 - _13.fld0.2.2;
_7 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_13.fld7 = _13.fld0.3;
_5 = [151187383793590776385526446758369081988_i128,(-59267322852937145797844582046995949178_i128),(-129174697061537890543939986599090056003_i128),(-147603834518553209507100485736758419533_i128),(-49135605926115590806061917796269355113_i128),(-136625298409707184105064223399119537194_i128),(-152263026175779401859759151211772080562_i128)];
_13.fld0.3 = [(-25_i8),(-126_i8),71_i8];
_18 = '\u{233e6}';
_13.fld0.2.1 = [11781_u16,15896_u16,5956_u16,65414_u16,65464_u16,16481_u16,12042_u16];
_13.fld0.0 = [(-155149053578076644779405468097397890273_i128),(-67293624028118444129183284044489182511_i128),(-5821546863884609477232240898711485214_i128),154903611769197020481848680109508106635_i128,(-47336749430555080896565299590545986859_i128),130557309455901243029564430600610976510_i128,31976732415829818962760013674564350667_i128];
_8 = _17.2 - _17.2;
_9 = _13.fld0.2;
_13.fld0.2.1 = _20;
RET = _10 >> _24;
Goto(bb11)
}
bb11 = {
_13.fld0.2.1 = [5669_u16,2682_u16,51430_u16,56266_u16,32399_u16,15461_u16,11372_u16];
RET = _23;
RET = _24 as isize;
_9.0 = [(-2751253403923295577695885186783829350_i128),116619444159996581763602647974296243863_i128];
_23 = !_10;
Goto(bb12)
}
bb12 = {
_17.1 = [39566_u16,30097_u16,59173_u16,27038_u16,61017_u16,19944_u16,39888_u16];
_13.fld0.2.2 = _17.2 * _8;
Goto(bb13)
}
bb13 = {
_15 = [1066070040_i32,(-1309119832_i32),(-1822900154_i32),(-1694816508_i32),1150605315_i32,1004760228_i32,(-163412890_i32),121956135_i32];
_12 = [123600280861916739217462747978203237036_i128,40904740313663767198145392881095558950_i128,(-149014552480467995556918037895244309216_i128),20908978603672470008265349395119464037_i128,(-10456722514064327490814277546021864769_i128),153021017001239255412658973675985206731_i128,28808039929092562327777171471285840726_i128];
_13.fld0.2 = (_9.0, _17.1, _8);
_5 = [81978507596732381984045523825060733581_i128,(-112792057514305286034458840305568301017_i128),51415247307009609549237049420594477663_i128,(-16674023723246644886331513215732695126_i128),(-100717674273576180068416298235467288381_i128),(-146410859551482802422701033658185884809_i128),(-21720173268700639088037382543713199293_i128)];
_13.fld0.2 = (_9.0, _9.1, _17.2);
_16 = !false;
_26 = 154720837367983175472232843900301542022_i128 as f32;
_9 = _17;
_29 = _4;
_19 = !_13.fld0.6;
RET = 568251353_i32 as isize;
_8 = !_17.2;
_3 = _6;
Goto(bb14)
}
bb14 = {
_30.fld3 = (_12,);
_6 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_30.fld0 = [_8,_9.2,_17.2,_8,_13.fld0.2.2,_17.2];
RET = !_10;
_30.fld4 = _29;
_27 = core::ptr::addr_of!(_23);
_20 = [8784_u16,51869_u16,56511_u16,41449_u16,61319_u16,26538_u16,35718_u16];
_17 = _13.fld0.2;
_31.0 = [78802540693856913592101083004523432404_i128,21423126913432044300732949091964453357_i128];
_13.fld3 = Adt47::Variant0 { fld0: _13.fld0.3,fld1: _27,fld2: _23,fld3: 1088988242_i32,fld4: _24 };
_13.fld0.0 = _5;
_31 = _13.fld0.2;
_26 = _24 as f32;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(6_usize, 12_usize, Move(_12), 7_usize, Move(_7), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(6_usize, 29_usize, Move(_29), 19_usize, Move(_19), 15_usize, Move(_15), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(6_usize, 16_usize, Move(_16), 5_usize, Move(_5), 2_usize, Move(_2), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [isize; 2],mut _2: [i128; 7],mut _3: [usize; 7],mut _4: u32,mut _5: [i128; 7],mut _6: [usize; 7],mut _7: [i128; 7],mut _8: [usize; 7],mut _9: [i8; 4],mut _10: [i128; 7],mut _11: [usize; 7],mut _12: isize,mut _13: [usize; 7]) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _14: [u64; 6];
let _15: *const i8;
let _16: isize;
let _17: u128;
let _18: [isize; 2];
let _19: i128;
let _20: [char; 3];
let _21: [i8; 3];
let _22: i8;
let _23: isize;
let _24: Adt51;
let _25: u128;
let _26: f32;
let _27: isize;
let _28: *const f32;
let _29: u128;
let _30: [isize; 2];
let _31: ([i128; 2], [u16; 7], u64);
let _32: [i128; 7];
let _33: ([i128; 7],);
let _34: [i32; 8];
let _35: [i8; 4];
let _36: [i128; 7];
let _37: Adt49;
let _38: isize;
let _39: char;
let _40: ();
let _41: ();
{
RET = _10;
_10 = _2;
_8 = _13;
_6 = [4948558631561642619_usize,2_usize,0_usize,5_usize,1_usize,3_usize,14688484151945353634_usize];
_8 = [1_usize,3516843077554914057_usize,0_usize,2_usize,1_usize,1_usize,13190438209343049037_usize];
_9 = [67_i8,117_i8,(-68_i8),20_i8];
_3 = [3380263071573287785_usize,3_usize,1_usize,16887287570469499595_usize,9265844473523429760_usize,13763841724173785872_usize,4029016198754888736_usize];
RET = _7;
_5 = [69014662237807932337065099977944565542_i128,49869087507227884963355963157716675136_i128,154028848198457608173077209033625134638_i128,(-117819893070412727887672847894723354358_i128),(-58678243708387687880419651740960946929_i128),(-72953109405187405615381361321465900461_i128),4184679622629478422128286494405918168_i128];
_3 = _6;
_9 = [(-48_i8),32_i8,121_i8,69_i8];
_2 = _5;
_16 = _12;
RET = [(-102355383357738545810529886357512783787_i128),(-26169058952198617647525073068132247424_i128),(-10180663622620259722546594551898708102_i128),31453636468426194550447501642260283183_i128,109884048688935419142608789295732977287_i128,(-106365499148293353152982653868095691917_i128),126832862634790146250327889500036411383_i128];
RET = [(-120390128771031819517462571850660288077_i128),154918441698630169071285008195729655409_i128,(-135170209868234633211682071261235056016_i128),(-100247416813829793850109392034816113499_i128),(-16169123948459607383829337211721932646_i128),29004007966814229923018629715975966830_i128,(-13851354427615461053545054636331670289_i128)];
_16 = _12;
_18 = [_16,_16];
Goto(bb1)
}
bb1 = {
_10 = [21914102346598399479351883470214790290_i128,(-104929113745484468553834947560505254157_i128),(-124447701633788303357028792580748615210_i128),(-167685016408806253364682614689215324595_i128),80418714101723767076512100663456869053_i128,(-50719728126802593333100221853589912535_i128),(-117013103526484806327769911667363234762_i128)];
RET = [160819969447441278457287644568693849410_i128,22546675104619827575838195160010988479_i128,(-16568182829028386276537381856152817702_i128),(-119407253645903140240274905010856590839_i128),167015337512326842649954791875076544346_i128,(-104636177605440946012811373790034845431_i128),(-118009770067154977635997807170842147606_i128)];
_2 = [58296960845946265959183210935631940167_i128,127547245339528823870426278307107489808_i128,(-84097827088939275120058847695256419004_i128),(-90301578929861394922926160772116461065_i128),(-136768166126020083819943672595860680115_i128),18694159142632664672951391732328470618_i128,(-125923058551431546077781502115349480156_i128)];
_3 = _13;
_16 = 5209977872547131387_u64 as isize;
_12 = _16 << _4;
_14 = [8593325071866518625_u64,6590683180824083915_u64,14067582145532846047_u64,10416370699202326585_u64,3768977177206725187_u64,14124367104758958312_u64];
_9 = [120_i8,(-95_i8),(-62_i8),20_i8];
_17 = !335465389023284673413313891133579632825_u128;
_17 = 19172576527201272910220637188093329101_u128;
RET = [(-119186447284540291046148000912638568348_i128),(-26693344787560278240007932407720715497_i128),26740427633447207711980124922398880433_i128,(-141982163880322052844154224076245523116_i128),59422893879596453137733233752787989588_i128,24035358797978668539127541752639582710_i128,(-12627869048589034388789281716243085105_i128)];
_13 = _6;
_18 = [_12,_16];
_17 = !204315199231491327651519582622816549800_u128;
_5 = [(-14010971851459055898818103969203728405_i128),124828097143575536577899145939362285174_i128,(-100793398448787649283941851035296313318_i128),164517224693422018660166940596143846430_i128,(-75892542021175056139432645788734773976_i128),(-5040102534707150973989109953584196764_i128),(-153696910364324270151663441409422480091_i128)];
_12 = _16 << _16;
_10 = _2;
_13 = _11;
_18 = [_16,_12];
Call(_3 = core::intrinsics::transmute(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = [1_usize,2_usize,18429418746024119251_usize,0_usize,7_usize,3_usize,5_usize];
_2 = [(-122797365098922770144246485228910396588_i128),(-156951618540186805267877675183742461674_i128),86571944783130994448980116091584157737_i128,131160950437734867946176115906203175267_i128,(-149039412217877901814780296685360921068_i128),39474988527349717941858365912240409245_i128,(-9671604334887174829222584297525423912_i128)];
_17 = 8_u8 as u128;
RET = [(-38090166010765199178701386842363065408_i128),(-13933584402808557906867303859690035476_i128),163453992909301827940586019814950054825_i128,86559722139119851587012565213285144864_i128,(-86223535185480515870660491899796590900_i128),121509293700437674757144406149490084146_i128,92272467118477150776297696477895585082_i128];
_1 = _18;
_7 = [(-18369431896014944636023807136530555768_i128),(-30553965284481486676748554334530492970_i128),(-164912110876538214199004743902991904135_i128),(-36501313452755100747881204357516406665_i128),151576317624450769537052710441440942611_i128,12258327742096215591602975398126350365_i128,56451795353397978828319779403015040344_i128];
_18 = [_12,_12];
_12 = 14608793927064926384_u64 as isize;
_14 = [7765202694257420192_u64,6491025794773768465_u64,165061443638366766_u64,10122079276953041034_u64,2349812490466823668_u64,17422095522793822595_u64];
_13 = [3_usize,7_usize,4_usize,3220735644497821030_usize,9237146129308576944_usize,1290316633014381929_usize,6493891520622127587_usize];
match _4 {
0 => bb1,
1 => bb3,
3457265696 => bb5,
_ => bb4
}
}
bb3 = {
_10 = [21914102346598399479351883470214790290_i128,(-104929113745484468553834947560505254157_i128),(-124447701633788303357028792580748615210_i128),(-167685016408806253364682614689215324595_i128),80418714101723767076512100663456869053_i128,(-50719728126802593333100221853589912535_i128),(-117013103526484806327769911667363234762_i128)];
RET = [160819969447441278457287644568693849410_i128,22546675104619827575838195160010988479_i128,(-16568182829028386276537381856152817702_i128),(-119407253645903140240274905010856590839_i128),167015337512326842649954791875076544346_i128,(-104636177605440946012811373790034845431_i128),(-118009770067154977635997807170842147606_i128)];
_2 = [58296960845946265959183210935631940167_i128,127547245339528823870426278307107489808_i128,(-84097827088939275120058847695256419004_i128),(-90301578929861394922926160772116461065_i128),(-136768166126020083819943672595860680115_i128),18694159142632664672951391732328470618_i128,(-125923058551431546077781502115349480156_i128)];
_3 = _13;
_16 = 5209977872547131387_u64 as isize;
_12 = _16 << _4;
_14 = [8593325071866518625_u64,6590683180824083915_u64,14067582145532846047_u64,10416370699202326585_u64,3768977177206725187_u64,14124367104758958312_u64];
_9 = [120_i8,(-95_i8),(-62_i8),20_i8];
_17 = !335465389023284673413313891133579632825_u128;
_17 = 19172576527201272910220637188093329101_u128;
RET = [(-119186447284540291046148000912638568348_i128),(-26693344787560278240007932407720715497_i128),26740427633447207711980124922398880433_i128,(-141982163880322052844154224076245523116_i128),59422893879596453137733233752787989588_i128,24035358797978668539127541752639582710_i128,(-12627869048589034388789281716243085105_i128)];
_13 = _6;
_18 = [_12,_16];
_17 = !204315199231491327651519582622816549800_u128;
_5 = [(-14010971851459055898818103969203728405_i128),124828097143575536577899145939362285174_i128,(-100793398448787649283941851035296313318_i128),164517224693422018660166940596143846430_i128,(-75892542021175056139432645788734773976_i128),(-5040102534707150973989109953584196764_i128),(-153696910364324270151663441409422480091_i128)];
_12 = _16 << _16;
_10 = _2;
_13 = _11;
_18 = [_16,_12];
Call(_3 = core::intrinsics::transmute(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_19 = 50159930862811598128116664995681123267_i128;
_21 = [(-88_i8),(-99_i8),(-109_i8)];
_11 = _3;
_13 = [3302952123028469656_usize,2_usize,7_usize,15772787916039187397_usize,12314970449162682556_usize,7364529486676589804_usize,1_usize];
_8 = [7218236008456400931_usize,3_usize,6243946486510091161_usize,7_usize,5571264711848154300_usize,6_usize,15547334202790052676_usize];
_8 = [18312472475054383980_usize,1822895196412071325_usize,7884765297424335883_usize,11832326441409738371_usize,4_usize,3_usize,6629000541107803836_usize];
_6 = [5_usize,2_usize,2_usize,4_usize,2381336983776764049_usize,3503928666958576461_usize,2_usize];
RET = _5;
_25 = _17;
_16 = _12;
_1 = _18;
_11 = [8858564149791522227_usize,11489023332588915543_usize,4_usize,3_usize,5_usize,7_usize,1_usize];
_23 = 2104713858703443268_i64 as isize;
_16 = !_12;
_24.fld3 = (_7,);
_17 = !_25;
_23 = !_12;
_15 = core::ptr::addr_of!(_22);
(*_15) = (-127_i8);
_24.fld3 = (RET,);
_22 = -(-113_i8);
match _19 {
50159930862811598128116664995681123267 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_25 = !_17;
_18 = [_16,_12];
RET = [_19,_19,_19,_19,_19,_19,_19];
_26 = 5463015715838617489_u64 as f32;
_25 = _17 ^ _17;
_14 = [5940440657066576091_u64,7214092146960246409_u64,6295612342147281899_u64,16320052970556164836_u64,18021728000867012283_u64,8279415047583003880_u64];
_20 = ['\u{f3111}','\u{40218}','\u{fb262}'];
_24.fld3 = (_10,);
_10 = _5;
_12 = _16 >> (*_15);
_15 = core::ptr::addr_of!((*_15));
_1 = _18;
_27 = (*_15) as isize;
_15 = core::ptr::addr_of!((*_15));
_28 = core::ptr::addr_of!(_26);
(*_28) = (-5752_i16) as f32;
_18 = [_16,_12];
Goto(bb8)
}
bb8 = {
_24.fld3.0 = [_19,_19,_19,_19,_19,_19,_19];
_2 = [_19,_19,_19,_19,_19,_19,_19];
_28 = core::ptr::addr_of!((*_28));
_28 = core::ptr::addr_of!((*_28));
Goto(bb9)
}
bb9 = {
_24.fld4 = [_16,_12];
_14 = [10309913078324825503_u64,14297600782935844336_u64,13440267181963195690_u64,2174558559894512570_u64,17664103227202104904_u64,8263253205066442468_u64];
_29 = _25 | _25;
_2 = [_19,_19,_19,_19,_19,_19,_19];
_24.fld2 = _6;
_22 = 7_usize as i8;
_30 = [_12,_16];
_24.fld0 = [8654963814192365201_u64,2424910133764874093_u64,16015439832998642417_u64,9367310711089687396_u64,5371769601840586228_u64,8302313580609206002_u64];
Goto(bb10)
}
bb10 = {
_31.1 = [39081_u16,23268_u16,47854_u16,50481_u16,49190_u16,57979_u16,36289_u16];
_30 = [_27,_27];
RET = [_19,_19,_19,_19,_19,_19,_19];
(*_15) = -103_i8;
_32 = _10;
RET = [_19,_19,_19,_19,_19,_19,_19];
_14 = [13661380099540812864_u64,5000775822910930913_u64,2381228256513953298_u64,9873764597969925306_u64,282452202214336609_u64,5599586422551327740_u64];
_16 = _12;
(*_15) = -29_i8;
_31.1 = [2151_u16,23479_u16,57727_u16,48748_u16,7293_u16,7897_u16,37582_u16];
_1 = _18;
_7 = [_19,_19,_19,_19,_19,_19,_19];
Goto(bb11)
}
bb11 = {
_3 = [4_usize,16623126047237616001_usize,3_usize,18087699539158584844_usize,7_usize,2_usize,2444250036095943729_usize];
_4 = 4111441856_u32 * 1150522971_u32;
_26 = _19 as f32;
_31.2 = _12 as u64;
(*_28) = _31.2 as f32;
_6 = [2_usize,4_usize,1_usize,16147542003362005517_usize,1_usize,4847157059795583687_usize,9131024398752939465_usize];
_24.fld0 = [_31.2,_31.2,_31.2,_31.2,_31.2,_31.2];
_24.fld3.0 = _10;
_18 = [_16,_16];
_24.fld0 = _14;
_24.fld3.0 = [_19,_19,_19,_19,_19,_19,_19];
_4 = 4054146099_u32 ^ 2707101588_u32;
_7 = _2;
_24.fld4 = _18;
_8 = [4_usize,5_usize,0_usize,2_usize,2322879469260355368_usize,3439282979176254361_usize,1_usize];
_2 = RET;
_20 = ['\u{f68c1}','\u{87f0f}','\u{e8275}'];
_19 = (-2964749186284313420_i64) as i128;
_3 = [1897107963663243848_usize,3985463438114172741_usize,2393569476823633506_usize,7117547738681456406_usize,2_usize,1_usize,17909636509089806962_usize];
_12 = _16 << _29;
_24.fld3.0 = [_19,_19,_19,_19,_19,_19,_19];
_33.0 = _32;
_19 = _26 as i128;
_2 = _10;
_24.fld0 = _14;
Goto(bb12)
}
bb12 = {
_24.fld2 = [3_usize,14034681760591128956_usize,14797019715353897514_usize,5279562938826219102_usize,3624350192899533956_usize,3_usize,13364025731562167161_usize];
_9 = [(*_15),(*_15),(*_15),(*_15)];
_25 = !_29;
Goto(bb13)
}
bb13 = {
_28 = core::ptr::addr_of!(_26);
_11 = [1_usize,1241093462164926707_usize,7_usize,1_usize,6_usize,4163459869286069382_usize,4788640856495788970_usize];
_3 = [2_usize,3550041640784882273_usize,7_usize,14832370140018640078_usize,953397369363345408_usize,10531848315265387007_usize,1_usize];
_31.0 = [_19,_19];
_5 = [_19,_19,_19,_19,_19,_19,_19];
(*_15) = 54_i8 | (-30_i8);
_20 = ['\u{41ad9}','\u{a4a35}','\u{7fcca}'];
_24.fld0 = [_31.2,_31.2,_31.2,_31.2,_31.2,_31.2];
_27 = _12;
(*_28) = (-762424750143979304_i64) as f32;
_25 = _19 as u128;
(*_15) = 104_i8;
_24.fld4 = [_12,_27];
_24.fld4 = [_16,_27];
_8 = [3_usize,15553379552730498913_usize,5_usize,11012683660463613209_usize,0_usize,5111909060665082351_usize,5610069579717279834_usize];
_2 = [_19,_19,_19,_19,_19,_19,_19];
_17 = _31.2 as u128;
_7 = RET;
_31.2 = 6788477226370094562_u64 * 16089481455628515683_u64;
_23 = _19 as isize;
_33.0 = [_19,_19,_19,_19,_19,_19,_19];
_19 = 162786559023472019529570255892674235202_i128;
_20 = ['\u{f9ddb}','\u{9dcb7}','\u{e7f3b}'];
_20 = ['\u{7a8dd}','\u{1db1c}','\u{ebb12}'];
_38 = !_12;
RET = [_19,_19,_19,_19,_19,_19,_19];
Call(_33 = fn8(_8, _10, _12, _18, _11, _6, _20, _15, _16, _30, _29, _24.fld4, _38, RET, _5), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7 = [_19,_19,_19,_19,_19,_19,_19];
_1 = [_12,_27];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(7_usize, 16_usize, Move(_16), 13_usize, Move(_13), 22_usize, Move(_22), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(7_usize, 7_usize, Move(_7), 30_usize, Move(_30), 31_usize, Move(_31), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(7_usize, 18_usize, Move(_18), 1_usize, Move(_1), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(7_usize, 9_usize, Move(_9), 38_usize, Move(_38), 25_usize, Move(_25), 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [usize; 7],mut _2: [i128; 7],mut _3: isize,mut _4: [isize; 2],mut _5: [usize; 7],mut _6: [usize; 7],mut _7: [char; 3],mut _8: *const i8,mut _9: isize,mut _10: [isize; 2],mut _11: u128,mut _12: [isize; 2],mut _13: isize,mut _14: [i128; 7],mut _15: [i128; 7]) -> ([i128; 7],) {
mir! {
type RET = ([i128; 7],);
let _16: [usize; 7];
let _17: f32;
let _18: f64;
let _19: [i128; 2];
let _20: i16;
let _21: isize;
let _22: [usize; 7];
let _23: bool;
let _24: [i32; 8];
let _25: Adt45;
let _26: isize;
let _27: *const i8;
let _28: [i128; 2];
let _29: i64;
let _30: bool;
let _31: f64;
let _32: char;
let _33: [i128; 7];
let _34: Adt45;
let _35: i128;
let _36: u64;
let _37: f32;
let _38: f64;
let _39: ([i128; 7],);
let _40: i8;
let _41: char;
let _42: Adt55;
let _43: isize;
let _44: [usize; 7];
let _45: f64;
let _46: f64;
let _47: *const f32;
let _48: ();
let _49: ();
{
_5 = [8837002777780998137_usize,2_usize,7_usize,6_usize,2705016400054250376_usize,7_usize,5_usize];
RET.0 = _2;
_5 = [15730411827193775599_usize,15522386598177516205_usize,1170628301429513988_usize,2_usize,8359858103824529655_usize,2_usize,1_usize];
RET.0 = _2;
RET = (_2,);
_2 = RET.0;
_18 = _13 as f64;
RET = (_2,);
_12 = [_3,_13];
_9 = _3;
_14 = [(-134727328534859600439541601379349123805_i128),112495638544485366023706164334087914053_i128,126436553136640873000035148092960500849_i128,(-24508564786773999296891057138176046260_i128),(-93382622185312425849962496991192367169_i128),(-64896295408641008434584789237332142642_i128),(-103387764485260504889939918812288843723_i128)];
_20 = 23327_i16;
_11 = !93317684533187452917733894822483768671_u128;
_6 = _5;
RET = (_2,);
_17 = _18 as f32;
_1 = [4_usize,14238836448989405833_usize,6_usize,5558029095670102667_usize,3_usize,0_usize,13060591377093697196_usize];
_1 = _5;
Goto(bb1)
}
bb1 = {
_3 = -_9;
_2 = [(-12769319753839272627130374859922362332_i128),(-76428273587410644658168351350888152844_i128),98061298170868849418515074335876714424_i128,(-70871961701091102694738487042135286289_i128),66551534972131732111804166975432649437_i128,61777986004441223005354048216663440167_i128,(-169370099008370724662232396853913161788_i128)];
_16 = _5;
_13 = _9;
_18 = 231922676_u32 as f64;
RET.0 = [(-61782736545648239664120828056188550465_i128),(-80336151112554839986580586988944971771_i128),2132873872159304276844940654906551645_i128,(-123857873319147093074478418026413381839_i128),(-151009758867021346347639256656331067824_i128),(-162629696986515355227042967356559187272_i128),143380490077403581466031737474665578365_i128];
_14 = _2;
Call(_16 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _11 as isize;
_16 = _1;
_19 = [125691264826998726374808826402002076413_i128,(-162415840803110091889728979008412034321_i128)];
_18 = (-654728314_i32) as f64;
_17 = 118_u8 as f32;
_10 = [_9,_13];
_10 = [_13,_13];
_6 = [3_usize,13022710161492294288_usize,14886784999230743733_usize,2_usize,15863145896054774906_usize,5_usize,573678539221705881_usize];
_2 = [103433497860136929908457388504524535350_i128,142694926532651023444758208647375528304_i128,(-50858382497067587379384756460379973062_i128),52684132449992507879368007134122203751_i128,(-105679806400872047555243221648387012265_i128),(-156455951212609097514516598429624664442_i128),109610930933815780105548526833920874422_i128];
_11 = 258643056793319953555558456878888564980_u128 ^ 96400481074109050694811625623448734883_u128;
(*_8) = (-97_i8) << _13;
(*_8) = 6088_u16 as i8;
RET = (_14,);
_18 = 2122752831_u32 as f64;
_7 = ['\u{7d056}','\u{3811e}','\u{90b00}'];
Goto(bb3)
}
bb3 = {
_7 = ['\u{5d32}','\u{abf4}','\u{c0a6d}'];
_2 = [(-78499427124760971408896765688044170026_i128),74542086864657088249395156959290196047_i128,109330906893464406579026073179681431616_i128,149441349369266757187476607738266700474_i128,(-135218892121817507040099221929309925178_i128),(-43681589404517564838750655091103657996_i128),(-103082007378199910456292478929939362723_i128)];
RET = (_2,);
_21 = _13 * _3;
RET.0 = [129973560322654316838008474663125530776_i128,100261181599724464914887386276283077811_i128,108991756003002707610507961698935318193_i128,17463770157967425918277001922854673741_i128,(-48695681870229329146513517276121807893_i128),(-48694532288535561106588834802720097918_i128),92443479709382609749050402531173586019_i128];
_5 = [15444032522870631683_usize,5691494501853180666_usize,2008357611826776729_usize,965185173805498344_usize,9043802636028744582_usize,12084814208149957657_usize,16562713020111405380_usize];
_21 = -_13;
_4 = [_21,_13];
_3 = _18 as isize;
_6 = _5;
_12 = _10;
RET = (_15,);
RET = (_14,);
_10 = [_3,_13];
_8 = core::ptr::addr_of!((*_8));
_22 = [12016914303979805441_usize,5_usize,2_usize,7_usize,2_usize,0_usize,7_usize];
_17 = (-1079459732079288242_i64) as f32;
_11 = 104487762978806264775661495566725909642_u128;
_9 = _21 >> _13;
_13 = -_21;
_9 = 11436613050685385931_u64 as isize;
_25.fld0 = core::ptr::addr_of!(_9);
_5 = [2_usize,4028098306582171462_usize,5_usize,17658649363308875627_usize,3_usize,17118292657338237802_usize,8777402989538482312_usize];
(*_8) = _9 as i8;
_3 = !_13;
_13 = 64389_u16 as isize;
_26 = -_13;
match _11 {
104487762978806264775661495566725909642 => bb5,
_ => bb4
}
}
bb4 = {
_3 = -_9;
_2 = [(-12769319753839272627130374859922362332_i128),(-76428273587410644658168351350888152844_i128),98061298170868849418515074335876714424_i128,(-70871961701091102694738487042135286289_i128),66551534972131732111804166975432649437_i128,61777986004441223005354048216663440167_i128,(-169370099008370724662232396853913161788_i128)];
_16 = _5;
_13 = _9;
_18 = 231922676_u32 as f64;
RET.0 = [(-61782736545648239664120828056188550465_i128),(-80336151112554839986580586988944971771_i128),2132873872159304276844940654906551645_i128,(-123857873319147093074478418026413381839_i128),(-151009758867021346347639256656331067824_i128),(-162629696986515355227042967356559187272_i128),143380490077403581466031737474665578365_i128];
_14 = _2;
Call(_16 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_20 = -(-22519_i16);
(*_8) = !49_i8;
_15 = [166375499884587348367018575338344830461_i128,(-73058290392818480689043027065753742236_i128),40840353643538827297246671258213087443_i128,92207541698957766985485476227277723597_i128,100856123962607436593879984767237387210_i128,(-113509355950880414391992052433747638899_i128),24577317501857091508182445596625071120_i128];
_13 = !_3;
_23 = true;
_25.fld1 = _21 as i8;
_4 = _10;
_17 = 1374155379_u32 as f32;
_25.fld1 = (*_8) ^ (*_8);
_4 = _10;
_11 = !14163244732065522389519768267923740116_u128;
_20 = 7843_i16;
_15 = _14;
(*_8) = _25.fld1 + _25.fld1;
_19 = [(-40599605821460135590263317122012063172_i128),(-131880203037503266482414947270795270527_i128)];
_28 = [(-91605408783929337082302389979876960124_i128),140962984030302711647152077732120772942_i128];
_25.fld1 = !(*_8);
_15 = [154221941925669513732359657261533366823_i128,(-119631435952827605894778498534316873501_i128),(-51654081512509615331502081651137130473_i128),73266437292895039094894566656061017453_i128,108089799760357047713853521531999923019_i128,141952963727719994595538746887957182252_i128,93615000912606056600463518110673548743_i128];
_8 = core::ptr::addr_of!((*_8));
RET = (_15,);
_9 = _13;
_15 = [144577917822243934418588067413482457851_i128,119699369705605620490525204032496095034_i128,(-44925714518279585520636512419358775074_i128),(-137089449854396992405993221491688435201_i128),20460360110250920652718100327668319124_i128,(-18468166695669006030385762221946734747_i128),119301590588216811516253490427408038377_i128];
_7 = ['\u{2060}','\u{29184}','\u{9ece5}'];
_10 = _12;
_24 = [1529660827_i32,311149163_i32,1631042553_i32,136122750_i32,1346555721_i32,1295625387_i32,(-1284133576_i32),1281186588_i32];
_1 = [2335167796168506126_usize,4_usize,0_usize,14624966671001899521_usize,90423578611295243_usize,3845121547272884190_usize,6635006314327271176_usize];
RET = (_2,);
Goto(bb6)
}
bb6 = {
_15 = RET.0;
_27 = _8;
_10 = [_13,_13];
_1 = _6;
_2 = [(-97625518975595377860199619833802975237_i128),(-115591560098306624966648358279789353253_i128),(-78072639339369376007640310845811024634_i128),(-129122477713139210880352361485352122260_i128),(-125859571698319307612975920709931324230_i128),63800771049654907589422488221567033833_i128,27840261995061655695400678928887083356_i128];
(*_27) = (-6883219299589626765_i64) as i8;
_12 = _10;
RET.0 = [140628160944910566182449286062946797514_i128,12571819494901146651928813514429642984_i128,45302012557084158757926774001290834188_i128,(-95718986028041232536889775808758514981_i128),154901989722606300562201831195474817023_i128,20150102502287070106580627054836361689_i128,134732322114685520162530016644109778087_i128];
_10 = [_9,_21];
Call(_25.fld2 = fn9(_25.fld1, _13, _4, _21, _24, _26, _2, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = [69073507200744781371727553873298774842_i128,(-154010701035130772013607339113469329726_i128),160451856304605719278663825665029377019_i128,50482816295272342087253507350719796040_i128,760222556074216066495972849488631059_i128,(-161528120301158289729862312914494103592_i128),148764093232212044300139280984549704650_i128];
_27 = core::ptr::addr_of!((*_27));
RET.0 = [(-63258098809758179573652222191509513270_i128),(-104458605677186090600952913450227953757_i128),159144917777097301698716570541110521749_i128,(-144819286076375607255668545634827997467_i128),(-40818447597984946397929451739464205815_i128),78308434613498883571043946431181578093_i128,(-33495853170154554707609695301401656807_i128)];
_9 = -_3;
_7 = ['\u{65698}','\u{e48c4}','\u{a987d}'];
_30 = _23;
_25.fld2 = !_11;
_11 = !_25.fld2;
_11 = !_25.fld2;
_21 = _3;
_29 = 654234078_u32 as i64;
RET = (_14,);
(*_27) = 183_u8 as i8;
_20 = (-21240_i16) >> _3;
_27 = core::ptr::addr_of!((*_8));
(*_27) = _25.fld1 << _3;
_24 = [1655160012_i32,1902485098_i32,(-1559334575_i32),1606400362_i32,1964358057_i32,1218972359_i32,(-1344623798_i32),(-1241125703_i32)];
_33 = [131457567253389333751542779623206805790_i128,52265331467981878069075861412940378717_i128,96584609839707591836373799039467888166_i128,(-69695909927067768767710560099075566526_i128),(-8670473450188366260069379373028867221_i128),60250194169256354223283981633861027937_i128,(-81289270531416444618098381757857384040_i128)];
_33 = [132597519420301509092119743797254157736_i128,(-23795198818560319480250031239275303137_i128),(-150177008728294136887947401756017579367_i128),(-70728194937209216434933709616655730791_i128),127312520987117876320643447039644010517_i128,56187204760948080741077547506041582432_i128,(-109149956208399868988929656388766203744_i128)];
_8 = core::ptr::addr_of!(_25.fld1);
_2 = [(-25557813487624587499705407213224277636_i128),(-20686450502867773615566771440565630229_i128),101799177452673047565513038337680665609_i128,(-143276923040934956805566232040837822350_i128),133023454624115902082982251370777856181_i128,(-686376417619800121468391755586338849_i128),(-38371390766056775957397404568828599972_i128)];
RET = (_14,);
_7 = ['\u{38dbd}','\u{f6580}','\u{d85b1}'];
Goto(bb8)
}
bb8 = {
_34.fld0 = _25.fld0;
_15 = [87450453271796693633873313616664498265_i128,96753480835611646445802676048029546926_i128,56459524807211963757287975803819529662_i128,17995499371630247283585723621925804563_i128,111590155067115968592997125185255937845_i128,(-144793353340653055279789023774583697059_i128),105467129689007946441844790535810024675_i128];
Goto(bb9)
}
bb9 = {
_28 = [105724727163482884759995107448527190248_i128,(-3445485269211654821218758652509247850_i128)];
_19 = [(-55769812629975718301305036026041280274_i128),(-6610530712153694484677509439824665121_i128)];
_29 = 11713264339890351201_u64 as i64;
(*_8) = -(*_27);
_32 = '\u{103e48}';
Goto(bb10)
}
bb10 = {
(*_8) = (*_27) * (*_27);
_33 = [94450369170151538917063627824219113944_i128,122694164155340045667258554536207990956_i128,(-52604956791921822786521641220736646741_i128),124062465822687523604071516649705245202_i128,(-92331472531587551373468436905365536978_i128),106688317064811116932565486986413850163_i128,(-159363118159369258640121263416461067855_i128)];
_9 = _3 & _21;
_14 = _33;
_25.fld1 = _3 as i8;
_34 = Adt45 { fld0: _25.fld0,fld1: (*_27),fld2: _25.fld2 };
_36 = 15284494446580582646_u64;
_24 = [1330924225_i32,(-1392574362_i32),(-331247353_i32),(-53994101_i32),(-291686510_i32),(-287607357_i32),2030719039_i32,(-1009463985_i32)];
_11 = _17 as u128;
_23 = _30;
_29 = _20 as i64;
_27 = _8;
_15 = _14;
RET.0 = [14166628798339025094269518180718899618_i128,(-55633143112298743737465007281938586170_i128),(-154257010196906399156577406672720880490_i128),(-120260315379719409600523991125000076652_i128),(-126850017229967783028552298284238720343_i128),(-97915491322830357477945552645847319770_i128),25894324742512478987614226593895679558_i128];
_25.fld1 = _34.fld1 + _34.fld1;
Goto(bb11)
}
bb11 = {
RET.0 = _15;
_4 = _10;
_14 = [115177674663487896253275906191041336366_i128,62247077840106172901180791285561727176_i128,101127299094157752547933358238321076471_i128,98535812130570036720129340012610015396_i128,(-97078779274197586508591020466144959872_i128),(-41577677219768119491717756225654329676_i128),(-139762500347520585553655268940262768418_i128)];
_22 = _16;
_29 = -(-7195385445932257789_i64);
_16 = _1;
_22 = [2_usize,2_usize,3_usize,3_usize,2036982055104793086_usize,3_usize,1_usize];
_36 = 18008516683449083508_u64;
_5 = _1;
_21 = _23 as isize;
_9 = -_13;
_36 = 13116796082727061544_u64 | 5826448307072285250_u64;
RET = (_15,);
_37 = _17;
_34.fld1 = _29 as i8;
_3 = 7_usize as isize;
_5 = [10482519518380276350_usize,2_usize,0_usize,1207381589634565780_usize,4_usize,0_usize,3_usize];
_31 = -_18;
Goto(bb12)
}
bb12 = {
_15 = RET.0;
(*_27) = !_34.fld1;
_17 = 100_u8 as f32;
_28 = [152846212211035936419882935049485050789_i128,(-116756193712245534824615142181118012228_i128)];
_25.fld1 = _34.fld1 - _34.fld1;
_6 = [13646770073222688307_usize,15073663570024386015_usize,13065494720961253882_usize,0_usize,1671839909572320460_usize,12409361664232680773_usize,13858042537150665197_usize];
_29 = (-6393783304288532811_i64);
_4 = [_3,_13];
_27 = core::ptr::addr_of!(_34.fld1);
_29 = 5471827259066191408_i64 & 6663580113548609356_i64;
_5 = [2_usize,16581870048322348785_usize,16181225456167522093_usize,4860226536458229827_usize,14719712367938009022_usize,6_usize,2_usize];
_28 = [79391347630242308131545117010209122220_i128,(-26048469826287975118223161678573922081_i128)];
RET.0 = _15;
_39 = (_14,);
_38 = -_31;
_36 = 130271386764752851109493280326342171898_i128 as u64;
_17 = _37 - _37;
_34 = Move(_25);
_34.fld0 = core::ptr::addr_of!(_13);
_9 = _3;
_34.fld2 = _11;
_10 = [_9,_3];
_22 = _1;
(*_8) = (*_27) | _34.fld1;
_5 = [0_usize,7_usize,17418191832594171368_usize,2_usize,11261959266410866273_usize,12318480982591632576_usize,1_usize];
_39.0 = [(-76277246022643512686455635679657562696_i128),44605297381332031772875167938238142216_i128,(-118112494585384577278684593561005602359_i128),122900332539550491864441642202318585898_i128,(-72759588187912295986624045200825015464_i128),7926097107775523792946449627373089202_i128,164277822806709748311177600268474085677_i128];
Call(_34.fld2 = core::intrinsics::transmute(_4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = '\u{30645}';
_33 = _15;
_38 = _31 + _31;
_25 = Adt45 { fld0: _34.fld0,fld1: _34.fld1,fld2: _34.fld2 };
(*_27) = !(*_8);
(*_8) = !_34.fld1;
_32 = '\u{816fb}';
_7 = [_32,_32,_32];
_18 = -_38;
Goto(bb14)
}
bb14 = {
_43 = _13 << _29;
_28 = [33866784842321817292496355583436688581_i128,(-50732518394123389186364252174468572329_i128)];
_25.fld0 = core::ptr::addr_of!(_43);
_44 = _16;
_23 = _34.fld2 > _34.fld2;
_12 = _4;
_34 = Adt45 { fld0: _25.fld0,fld1: (*_8),fld2: _25.fld2 };
_41 = _32;
_6 = _5;
_6 = [3_usize,6_usize,3_usize,13422081133428865881_usize,1585988379136687591_usize,1_usize,10050933701395208953_usize];
_4 = _12;
_26 = !_43;
_33 = [96632298175600255407824172735283694706_i128,(-151249948339143121631135841526939008270_i128),82487590636528373585184627552759740053_i128,98374910762001114506137298323290592122_i128,25772414018861618684566854377195403125_i128,(-124601674509701376206379216604279003571_i128),121017134968581118476169971792496606762_i128];
_38 = 48064_u16 as f64;
_25.fld0 = core::ptr::addr_of!(_26);
(*_27) = -_25.fld1;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(8_usize, 20_usize, Move(_20), 21_usize, Move(_21), 36_usize, Move(_36), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(8_usize, 6_usize, Move(_6), 3_usize, Move(_3), 41_usize, Move(_41), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(8_usize, 39_usize, Move(_39), 7_usize, Move(_7), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(8_usize, 14_usize, Move(_14), 16_usize, Move(_16), 29_usize, Move(_29), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i8,mut _2: isize,mut _3: [isize; 2],mut _4: isize,mut _5: [i32; 8],mut _6: isize,mut _7: [i128; 7],mut _8: [usize; 7]) -> u128 {
mir! {
type RET = u128;
let _9: char;
let _10: i16;
let _11: f32;
let _12: Adt51;
let _13: Adt54;
let _14: f64;
let _15: f32;
let _16: [isize; 2];
let _17: ([i128; 7],);
let _18: u8;
let _19: u16;
let _20: ();
let _21: ();
{
_6 = _4;
RET = !79584838458258736545378714985813944910_u128;
Call(_1 = fn10(_4, _4, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [(-31751067889580866498883076107380899701_i128),97956638409145904254607916827046999223_i128,(-103503277908415060528498117392741728241_i128),(-29660389529991982718377305545969145892_i128),22954752595217523936794986359169259660_i128,168237162284223161719828766872841630849_i128,(-26300217363813438621081869236647094318_i128)];
_5 = [1745545177_i32,(-1146322108_i32),(-2131203723_i32),1581122433_i32,(-522036198_i32),819254121_i32,154641358_i32,1181584762_i32];
_8 = [1955184782073717960_usize,18218509397061879142_usize,14634896908173172986_usize,18063497075043889429_usize,2_usize,4_usize,0_usize];
RET = _2 as u128;
_1 = (-52_i8) | (-105_i8);
_8 = [8214512196239541211_usize,1_usize,6_usize,2_usize,3_usize,13707014414318412129_usize,2_usize];
_4 = _2 + _2;
_1 = (-115_i8);
_7 = [(-17645273378297722042865640682516480105_i128),35349638666506070670231414806365688374_i128,(-30313252964161338530779862901801918393_i128),52724113866987799076171041640802187117_i128,154850551738825784348795235093986046999_i128,110722514312079603920233974636987665853_i128,99159015269199867908082695597647452997_i128];
Call(RET = fn11(_3, _2, _4, _7, _4, _3, _2, _8, _8, _3, _3, _7, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [_4,_6];
_4 = RET as isize;
RET = 207520262977273868376335249690452790909_u128 ^ 129869488672479475076644377436349663411_u128;
_6 = _4 * _2;
_3 = [_2,_2];
RET = 2259010242_u32 as u128;
_2 = !_4;
_7 = [123360900546051715277001038993021900950_i128,(-90636105064283765036043502276830276827_i128),22616263301838775121415150113339801799_i128,20634803065699699157363448252058358461_i128,(-152139638302933884316816217088820871836_i128),97772224380881952191619027614391416025_i128,(-118747388634415249171828188944836712542_i128)];
_7 = [(-723623636348125468316705995794649993_i128),(-29951592345703015516371535102813621068_i128),135804345578356145581210985032079292645_i128,112250756362013097538461363986348981291_i128,(-59963735332842744198410519207824638298_i128),(-15788991222617588542708294170928666619_i128),139990502731700694373732865046848313709_i128];
_7 = [22969981041850392061696174270672822063_i128,(-109074592670489762360184648025113876997_i128),(-35524533519315885805181450587068858415_i128),29363004497579546865144064321474454715_i128,105979180843299569030579432782962325084_i128,(-128642873993159273671440783337175158996_i128),88198184969988390160142703596060438036_i128];
_1 = RET as i8;
RET = !329916130133648507419079414180968275417_u128;
_6 = _2;
_8 = [6_usize,1_usize,5_usize,4_usize,11474798457750693571_usize,0_usize,7_usize];
_8 = [7_usize,5_usize,339857085025260643_usize,7_usize,0_usize,11523023863562219185_usize,7_usize];
_2 = _4;
_2 = _6 - _4;
RET = 7742125900621134900_u64 as u128;
_9 = '\u{46cb6}';
_9 = '\u{178c1}';
_2 = _4 + _4;
_5 = [1200670199_i32,2086899823_i32,(-1882659360_i32),(-837460744_i32),(-2110033631_i32),218049808_i32,(-1036064380_i32),280139685_i32];
_9 = '\u{94b16}';
_6 = -_2;
Goto(bb3)
}
bb3 = {
_9 = '\u{426e1}';
_9 = '\u{b3aee}';
RET = 237993431494040160227350948662933509612_u128;
_9 = '\u{f7048}';
_2 = !_6;
_8 = [6140074246472366231_usize,17988143802095852174_usize,11256825566051781945_usize,12651953075053388595_usize,306348158005494680_usize,1_usize,1_usize];
RET = 228599505064221216126553199860195331897_u128;
_5 = [998921915_i32,(-282041844_i32),1782567550_i32,(-91987844_i32),853550328_i32,(-2084230825_i32),(-1512130019_i32),1493638455_i32];
_4 = 1_usize as isize;
match RET {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
228599505064221216126553199860195331897 => bb9,
_ => bb8
}
}
bb4 = {
_3 = [_4,_6];
_4 = RET as isize;
RET = 207520262977273868376335249690452790909_u128 ^ 129869488672479475076644377436349663411_u128;
_6 = _4 * _2;
_3 = [_2,_2];
RET = 2259010242_u32 as u128;
_2 = !_4;
_7 = [123360900546051715277001038993021900950_i128,(-90636105064283765036043502276830276827_i128),22616263301838775121415150113339801799_i128,20634803065699699157363448252058358461_i128,(-152139638302933884316816217088820871836_i128),97772224380881952191619027614391416025_i128,(-118747388634415249171828188944836712542_i128)];
_7 = [(-723623636348125468316705995794649993_i128),(-29951592345703015516371535102813621068_i128),135804345578356145581210985032079292645_i128,112250756362013097538461363986348981291_i128,(-59963735332842744198410519207824638298_i128),(-15788991222617588542708294170928666619_i128),139990502731700694373732865046848313709_i128];
_7 = [22969981041850392061696174270672822063_i128,(-109074592670489762360184648025113876997_i128),(-35524533519315885805181450587068858415_i128),29363004497579546865144064321474454715_i128,105979180843299569030579432782962325084_i128,(-128642873993159273671440783337175158996_i128),88198184969988390160142703596060438036_i128];
_1 = RET as i8;
RET = !329916130133648507419079414180968275417_u128;
_6 = _2;
_8 = [6_usize,1_usize,5_usize,4_usize,11474798457750693571_usize,0_usize,7_usize];
_8 = [7_usize,5_usize,339857085025260643_usize,7_usize,0_usize,11523023863562219185_usize,7_usize];
_2 = _4;
_2 = _6 - _4;
RET = 7742125900621134900_u64 as u128;
_9 = '\u{46cb6}';
_9 = '\u{178c1}';
_2 = _4 + _4;
_5 = [1200670199_i32,2086899823_i32,(-1882659360_i32),(-837460744_i32),(-2110033631_i32),218049808_i32,(-1036064380_i32),280139685_i32];
_9 = '\u{94b16}';
_6 = -_2;
Goto(bb3)
}
bb5 = {
_7 = [(-31751067889580866498883076107380899701_i128),97956638409145904254607916827046999223_i128,(-103503277908415060528498117392741728241_i128),(-29660389529991982718377305545969145892_i128),22954752595217523936794986359169259660_i128,168237162284223161719828766872841630849_i128,(-26300217363813438621081869236647094318_i128)];
_5 = [1745545177_i32,(-1146322108_i32),(-2131203723_i32),1581122433_i32,(-522036198_i32),819254121_i32,154641358_i32,1181584762_i32];
_8 = [1955184782073717960_usize,18218509397061879142_usize,14634896908173172986_usize,18063497075043889429_usize,2_usize,4_usize,0_usize];
RET = _2 as u128;
_1 = (-52_i8) | (-105_i8);
_8 = [8214512196239541211_usize,1_usize,6_usize,2_usize,3_usize,13707014414318412129_usize,2_usize];
_4 = _2 + _2;
_1 = (-115_i8);
_7 = [(-17645273378297722042865640682516480105_i128),35349638666506070670231414806365688374_i128,(-30313252964161338530779862901801918393_i128),52724113866987799076171041640802187117_i128,154850551738825784348795235093986046999_i128,110722514312079603920233974636987665853_i128,99159015269199867908082695597647452997_i128];
Call(RET = fn11(_3, _2, _4, _7, _4, _3, _2, _8, _8, _3, _3, _7, _7, _7), ReturnTo(bb2), UnwindUnreachable())
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
_10 = _9 as i16;
_8 = [2_usize,7_usize,13229723484837598790_usize,12136972693946099270_usize,5_usize,5_usize,6582574780895200381_usize];
RET = !244286709057344728843334898170769775411_u128;
RET = 10271546622001594250963842222635242286_u128 * 100124978401157099534441420576731735603_u128;
_11 = 32602465162113892710884300554321210100_i128 as f32;
_5 = [(-652505729_i32),(-453737975_i32),(-779338371_i32),(-45461629_i32),1348222648_i32,1875207328_i32,(-1277585920_i32),(-341650152_i32)];
_7 = [152690496057203759757675081054035867491_i128,(-95055312965690492702726321077887895015_i128),(-78683807874778785963769001923917724524_i128),(-69687371121747355142043391922507228450_i128),134192216692590124782510362448509929000_i128,103908525570589129258452833054216916437_i128,(-36246414358124571316538873318363381855_i128)];
_4 = _6 & _6;
_7 = [(-114613603148047286159629311869632098206_i128),75760354898193236927092604988525506633_i128,(-5720153403188736215720483179956213306_i128),(-88012845608532648553356839130058372185_i128),(-83852660534378451190045985531273349294_i128),(-107621775998599954239087745687191994079_i128),(-21155271177025502489555324558048542466_i128)];
_6 = _4 ^ _4;
_11 = 803363638_i32 as f32;
_4 = 14268_u16 as isize;
_10 = 132529295532932382040348238261440509244_i128 as i16;
_12.fld5 = core::ptr::addr_of!(_10);
_8 = [4265040670531826457_usize,1_usize,5_usize,13580773027096932557_usize,12567653898442221403_usize,10110479973326462790_usize,14209394120637839284_usize];
_12.fld0 = [17466174127423985587_u64,8174336293628879890_u64,15274128141781024922_u64,12351215325405856889_u64,8056004400199115330_u64,16939994422449908617_u64];
_12.fld3 = (_7,);
_12.fld4 = _3;
_1 = 74_i8 ^ (-96_i8);
_1 = (-89_i8) - (-118_i8);
RET = 325339344771399495988114698395573498075_u128;
_3 = [_6,_6];
_7 = [(-68660341709904587748365634765736084783_i128),168742096181381677431649460027572221341_i128,80967680203540341913412279643009841339_i128,83618063606373513001997204956938856468_i128,(-43833921798537365180510154199276261427_i128),123624126270853585815401038533315661792_i128,68477025384360978537440281166315304155_i128];
_12.fld4 = [_6,_2];
_12.fld0 = [2718447642956832251_u64,452700823122475925_u64,15237105121904947645_u64,15699389871980630487_u64,16996315321144715313_u64,13831303107618447308_u64];
_8 = [4434313639488763317_usize,0_usize,6_usize,2_usize,2129834616027577951_usize,0_usize,6_usize];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
325339344771399495988114698395573498075 => bb11,
_ => bb10
}
}
bb10 = {
_7 = [(-31751067889580866498883076107380899701_i128),97956638409145904254607916827046999223_i128,(-103503277908415060528498117392741728241_i128),(-29660389529991982718377305545969145892_i128),22954752595217523936794986359169259660_i128,168237162284223161719828766872841630849_i128,(-26300217363813438621081869236647094318_i128)];
_5 = [1745545177_i32,(-1146322108_i32),(-2131203723_i32),1581122433_i32,(-522036198_i32),819254121_i32,154641358_i32,1181584762_i32];
_8 = [1955184782073717960_usize,18218509397061879142_usize,14634896908173172986_usize,18063497075043889429_usize,2_usize,4_usize,0_usize];
RET = _2 as u128;
_1 = (-52_i8) | (-105_i8);
_8 = [8214512196239541211_usize,1_usize,6_usize,2_usize,3_usize,13707014414318412129_usize,2_usize];
_4 = _2 + _2;
_1 = (-115_i8);
_7 = [(-17645273378297722042865640682516480105_i128),35349638666506070670231414806365688374_i128,(-30313252964161338530779862901801918393_i128),52724113866987799076171041640802187117_i128,154850551738825784348795235093986046999_i128,110722514312079603920233974636987665853_i128,99159015269199867908082695597647452997_i128];
Call(RET = fn11(_3, _2, _4, _7, _4, _3, _2, _8, _8, _3, _3, _7, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_9 = '\u{827ee}';
_12.fld5 = core::ptr::addr_of!(_10);
_12.fld3 = (_7,);
_2 = -_4;
_2 = _6 << _6;
_2 = _10 as isize;
_9 = '\u{e2ab8}';
_5 = [(-1380262044_i32),(-1769741628_i32),(-1044469125_i32),774922998_i32,(-683095866_i32),471713501_i32,434523015_i32,(-357458755_i32)];
_12.fld1 = core::ptr::addr_of!(_14);
_15 = _11;
_12.fld3.0 = _7;
_12.fld1 = core::ptr::addr_of!(_14);
_16 = [_6,_6];
_14 = _11 as f64;
_12.fld5 = core::ptr::addr_of!(_10);
_17 = (_7,);
_6 = _2;
match RET {
0 => bb4,
1 => bb12,
325339344771399495988114698395573498075 => bb14,
_ => bb13
}
}
bb12 = {
_7 = [(-31751067889580866498883076107380899701_i128),97956638409145904254607916827046999223_i128,(-103503277908415060528498117392741728241_i128),(-29660389529991982718377305545969145892_i128),22954752595217523936794986359169259660_i128,168237162284223161719828766872841630849_i128,(-26300217363813438621081869236647094318_i128)];
_5 = [1745545177_i32,(-1146322108_i32),(-2131203723_i32),1581122433_i32,(-522036198_i32),819254121_i32,154641358_i32,1181584762_i32];
_8 = [1955184782073717960_usize,18218509397061879142_usize,14634896908173172986_usize,18063497075043889429_usize,2_usize,4_usize,0_usize];
RET = _2 as u128;
_1 = (-52_i8) | (-105_i8);
_8 = [8214512196239541211_usize,1_usize,6_usize,2_usize,3_usize,13707014414318412129_usize,2_usize];
_4 = _2 + _2;
_1 = (-115_i8);
_7 = [(-17645273378297722042865640682516480105_i128),35349638666506070670231414806365688374_i128,(-30313252964161338530779862901801918393_i128),52724113866987799076171041640802187117_i128,154850551738825784348795235093986046999_i128,110722514312079603920233974636987665853_i128,99159015269199867908082695597647452997_i128];
Call(RET = fn11(_3, _2, _4, _7, _4, _3, _2, _8, _8, _3, _3, _7, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_12.fld3 = (_7,);
_3 = [_4,_6];
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(9_usize, 16_usize, Move(_16), 7_usize, Move(_7), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(9_usize, 2_usize, Move(_2), 6_usize, Move(_6), 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: [usize; 7]) -> i8 {
mir! {
type RET = i8;
let _4: f64;
let _5: Adt56;
let _6: Adt54;
let _7: ();
let _8: ();
{
RET = 48_i8;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
48 => bb9,
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
RET = (-8_i8);
_3 = [2_usize,1_usize,1374783469768157131_usize,3_usize,8900432806929461220_usize,1_usize,5_usize];
RET = (-97_i8);
RET = !(-115_i8);
RET = (-128_i8);
_1 = _2;
RET = 29_i8;
match RET {
0 => bb1,
1 => bb8,
2 => bb7,
3 => bb10,
4 => bb11,
5 => bb12,
29 => bb14,
_ => bb13
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
Return()
}
bb14 = {
_1 = 1400976657_u32 as isize;
RET = 309325997043473276551251264062833845596_u128 as i8;
_3 = [16151063269516890996_usize,0_usize,3604601856600415597_usize,5159022044386973645_usize,3_usize,6_usize,17839953940439494245_usize];
_1 = _2 << _2;
_1 = 0_usize as isize;
RET = 26_u8 as i8;
_1 = _2;
RET = false as i8;
_2 = _1;
_3 = [14047852776047441554_usize,11555570297609856308_usize,7498794539629428785_usize,5_usize,16996606449977361770_usize,4_usize,16727457106261981542_usize];
_2 = 9608887235641088171_usize as isize;
_2 = _1 - _1;
RET = 3_i8 << _2;
_2 = _1;
_4 = 104_u8 as f64;
RET = (-38_i8);
RET = 114_i8;
_3 = [0_usize,1_usize,10757056853945959498_usize,2_usize,1_usize,14794041514982839562_usize,3_usize];
Goto(bb15)
}
bb15 = {
Call(_7 = dump_var(10_usize, 1_usize, Move(_1), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [isize; 2],mut _2: isize,mut _3: isize,mut _4: [i128; 7],mut _5: isize,mut _6: [isize; 2],mut _7: isize,mut _8: [usize; 7],mut _9: [usize; 7],mut _10: [isize; 2],mut _11: [isize; 2],mut _12: [i128; 7],mut _13: [i128; 7],mut _14: [i128; 7]) -> u128 {
mir! {
type RET = u128;
let _15: Adt47;
let _16: [i128; 7];
let _17: [i8; 4];
let _18: [usize; 7];
let _19: [i8; 3];
let _20: i64;
let _21: isize;
let _22: f64;
let _23: u64;
let _24: ([i128; 7],);
let _25: [u64; 6];
let _26: char;
let _27: [i128; 2];
let _28: isize;
let _29: usize;
let _30: u8;
let _31: i128;
let _32: isize;
let _33: f64;
let _34: *mut char;
let _35: [u64; 6];
let _36: &'static char;
let _37: Adt57;
let _38: u32;
let _39: isize;
let _40: f32;
let _41: ([i128; 7],);
let _42: usize;
let _43: ([i128; 2], [u16; 7], u64);
let _44: [char; 3];
let _45: ([i128; 2], [u16; 7], u64);
let _46: [i128; 2];
let _47: ();
let _48: ();
{
_12 = _14;
_12 = [11078502233687073714384856811406743426_i128,38754770604991299171108400297394890768_i128,51227461385199425552372685620003940708_i128,117647930168125313637241620647170024249_i128,(-104505772051353976067071268015754256464_i128),5144467264263428228352606920362289735_i128,52705102134857699601277369669670187435_i128];
_1 = [_7,_5];
_6 = _11;
_10 = _1;
_8 = [6_usize,3_usize,7586813579951432981_usize,0_usize,15536377009085140506_usize,12279381117182038546_usize,14407779147099271874_usize];
_4 = _14;
_11 = _10;
_6 = [_5,_2];
RET = !305934692025975388783610029655081214469_u128;
_5 = !_3;
RET = 56479_u16 as u128;
_8 = [14188192523647679576_usize,5_usize,6_usize,3_usize,5_usize,11733072121073177040_usize,7959278502474719713_usize];
_5 = _7 * _3;
_12 = [(-82740207452693710921146987547035121769_i128),107327602545161014351648093840078125482_i128,(-46967499687946596328865252157306645980_i128),(-27496579199913856526367877265336187585_i128),(-75607097705409938595320573277740900070_i128),(-159443479691052729604312232475504878886_i128),78935743684414417929155173079404742636_i128];
RET = 80402480775029706807714408933041336237_u128 ^ 60460440226711947710620590819660676664_u128;
_8 = [1_usize,6_usize,0_usize,2589747685764593819_usize,7_usize,0_usize,2_usize];
_10 = _1;
_9 = [134654568391357728_usize,7342446011602518509_usize,3_usize,5412436481323195831_usize,1_usize,1_usize,12743526020297283042_usize];
_8 = [7644258064084166450_usize,17004102546975097586_usize,1_usize,4677103152953574578_usize,4_usize,2_usize,6671618127870246156_usize];
_16 = _4;
_7 = _5;
Goto(bb1)
}
bb1 = {
_9 = [18144358922152313311_usize,2_usize,12049787955355053071_usize,6416524605615286074_usize,1_usize,8674684089833482649_usize,5_usize];
_14 = [31271744998446843820418987512063135090_i128,46401575683097498851554141539932656736_i128,73442328862074791865616596908418442952_i128,130354493412384066328255830343364198885_i128,(-25225248014234531502788919061752919410_i128),116993692365841335059733804562572224041_i128,64831921980012355573975030430652325682_i128];
_4 = _16;
_5 = -_7;
_12 = [108598097951560478030489070391856197330_i128,87526871249807677200512457709400300781_i128,79608194410658285319957445003747449474_i128,(-40379935228908852421919325265916679799_i128),84140517065924815662165863509638280758_i128,45133874364856601598328976222121729053_i128,(-69840114192605818107964706667882130773_i128)];
_12 = [40325802347461745956028499468424467369_i128,(-2044844875792563351434602596410914577_i128),129537253978902399245737224890806674602_i128,(-156948912362366265655643525222037135799_i128),(-53860797091234656613230288124064074671_i128),160216705585383759491624482146449254466_i128,114828620842539878846047010011037404267_i128];
_17 = [42_i8,(-55_i8),53_i8,(-8_i8)];
_5 = 4251821045718065157_i64 as isize;
_17 = [99_i8,92_i8,(-76_i8),(-119_i8)];
_17 = [(-108_i8),60_i8,(-121_i8),(-120_i8)];
_18 = [6_usize,3_usize,0_usize,814141319561310639_usize,3_usize,4_usize,4_usize];
_3 = _2 >> _7;
_18 = [6708373427907328045_usize,7_usize,10897560036890176851_usize,15658944526486827571_usize,17643794876372621610_usize,7584732858332206740_usize,8023874904755610918_usize];
_5 = _7 - _3;
_8 = [4396347557924816348_usize,0_usize,10076729550840402148_usize,7404266155493218761_usize,6526308380570225713_usize,15213812872014250840_usize,3934337766555716748_usize];
_18 = [0_usize,8521199910719912940_usize,0_usize,1_usize,2_usize,12852627815024500999_usize,1_usize];
Call(_20 = fn12(_3, _8, _3, _5, _4, _3, _6, _5, _3, _5, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = _11;
_16 = [2632818901348618557939969679909580202_i128,(-86561533921733023301004510180002530349_i128),88006616532767693640510121643800474665_i128,(-48409086635197833887346713445635489842_i128),65313254061342322589312369868575824605_i128,50090031477316253014076847899649650777_i128,(-69285954293993988753352270223880085553_i128)];
_6 = [_7,_3];
RET = !277499342241084255570743985122686294677_u128;
_22 = _3 as f64;
Call(_3 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_23 = !7175832569410249416_u64;
Goto(bb4)
}
bb4 = {
_4 = [99224088597530255287053956433111712964_i128,(-29525170588427649221138813116124199691_i128),(-131209078086443627444203913636528105815_i128),118869182103661008385762188485374722019_i128,(-164089920260529881906120043921030934098_i128),167208768185602928271493253240037891242_i128,127018199191691700243560996719506201187_i128];
_23 = 11389083289522113420_u64 | 4908326323018930848_u64;
_4 = [134284585775655982993350813033690057785_i128,99854922456089362795397821571082992937_i128,140598691022847393650913687057863064648_i128,(-83300955633995055417421503803232219700_i128),(-129266708587175505449153102556039766834_i128),(-92106382472690954475870338945250812559_i128),(-155159122907529450853894952434074897742_i128)];
_24.0 = _4;
_17 = [114_i8,(-60_i8),(-127_i8),115_i8];
_17 = [(-60_i8),(-78_i8),98_i8,(-13_i8)];
_14 = [128540597898218384548305034408867709481_i128,(-49946049197052976503026983414866184840_i128),26188520597219663174426742607245767303_i128,14167569318807994604563176161702247828_i128,(-56586724551299689127743970200285604437_i128),80747520098717590520005240983297723743_i128,113578448451204588178199351567303006688_i128];
_16 = _14;
_24 = (_12,);
_14 = [(-56422892593769901100802947191099548744_i128),8936615105358743380136332130866695171_i128,100452124952377171234848661994598843229_i128,(-149243120556577699760343204054729936411_i128),137198086749037695698615249542229786197_i128,(-38288483539579247809355066248685744142_i128),169402257607032328394189213270821147384_i128];
_12 = [166483632676087021728357327955204114110_i128,(-80153383558361902761048465738189461373_i128),(-14476664790948501789274660544520765986_i128),(-118730402102293084841190208293465920947_i128),(-102373982718351531012443583376451139786_i128),(-99427683576310434452483474844685877593_i128),70381653807243913644448375608715326841_i128];
_7 = !_3;
_11 = _6;
_21 = _5;
_14 = [131483854766234441757735573074940225861_i128,(-84763872656948890267444286294906078262_i128),144314200273433727554932549368454881550_i128,23837283125149454505643146812619787549_i128,93677572569625137557129064770619986304_i128,(-167625365195786856327255587212117987060_i128),(-156484731922849925041566142012115765512_i128)];
_29 = !12787592247076514855_usize;
_2 = _21 ^ _7;
_7 = _2;
_18 = _9;
_25 = [_23,_23,_23,_23,_23,_23];
_20 = 6044912934525228374_i64 << _7;
_9 = [_29,_29,_29,_29,_29,_29,_29];
_4 = [(-72396486946603501474726128681096386976_i128),(-124832645947193002026063079077671881288_i128),81808067108138998632341576432970966296_i128,(-157033938343420036627561414841056652252_i128),(-94376804115143385758566991449833840949_i128),(-109820652553304523796136629035053323484_i128),112601030468979178682107849899939268215_i128];
_1 = [_7,_5];
Goto(bb5)
}
bb5 = {
RET = 166557167698670658063799344653592955524_u128;
_26 = '\u{d3523}';
_10 = [_7,_2];
_16 = [157267567883469806033780431250814597692_i128,168268727128111673693151357942949452983_i128,158605539239925155354423936988319752818_i128,89949955113544645502903620396706225640_i128,25568751926528797036171633688260305953_i128,(-142116449222295059204209115681370173032_i128),3842543260825911834097450635384556193_i128];
_7 = _2 | _5;
_16 = [142889017306346296949435899680543125976_i128,47352693862794816698198996327706904061_i128,40483857957951500520583609846641822141_i128,43324447776455325873110053262440260186_i128,(-116105723580008379848609350939850714234_i128),78966282443578838235382588632870963306_i128,13696592126978894287308064171395635718_i128];
RET = !74137617392684354322269442842070747551_u128;
_9 = _18;
_4 = [24495763642804755247655131477373138007_i128,127769169445680316216510791409529962072_i128,(-127404430851454052067372943675753218123_i128),148231362960804491956749331574514529513_i128,153596137018591259477595476316546629970_i128,(-66962653168374447222609126194493228064_i128),(-33441271790698365648045468103835573051_i128)];
_5 = -_2;
_16 = [94660653124515843266032756566968834799_i128,9701266434834532267411472592594951229_i128,28585291569842682327063966106011657055_i128,78942601380597983630695147697672349089_i128,157159194990370694522601286808231777198_i128,111420737494852168829524329801723546255_i128,34332410732889375363904080282401696625_i128];
_27 = [43236726609903170119525423186182062374_i128,(-32585936115789352313285429325043818115_i128)];
_21 = _3 | _2;
_24.0 = _4;
_13 = _14;
_1 = _6;
Call(_28 = core::intrinsics::bswap(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_11 = [_2,_21];
_13 = [(-54908245844707852113835445769544764881_i128),(-133990174132701334186340142255312311572_i128),(-18734828150916693413534371539051569556_i128),113140750207352465572142265225663120094_i128,(-95178619560755737021459585662522668464_i128),(-1206113493379392376185318365483732034_i128),(-96722835085940417291524899704945599313_i128)];
Call(_13 = core::intrinsics::transmute(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = [_3,_7];
_31 = (-160225251609861457791808156991914476531_i128);
_11 = [_21,_5];
_32 = _21;
_29 = 3956066151860658249_usize;
_25 = [_23,_23,_23,_23,_23,_23];
_30 = 114_u8;
_34 = core::ptr::addr_of_mut!(_26);
_5 = _7;
_24.0 = [_31,_31,_31,_31,_31,_31,_31];
_3 = _7 >> _2;
_31 = 11095_u16 as i128;
_4 = _13;
_20 = 3989524860320585620_i64;
_24 = (_16,);
_38 = !2944344852_u32;
_6 = _11;
_7 = !_2;
_33 = _22 * _22;
_9 = [_29,_29,_29,_29,_29,_29,_29];
_30 = _21 as u8;
match _20 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb9,
3989524860320585620 => bb11,
_ => bb10
}
}
bb8 = {
_11 = [_2,_21];
_13 = [(-54908245844707852113835445769544764881_i128),(-133990174132701334186340142255312311572_i128),(-18734828150916693413534371539051569556_i128),113140750207352465572142265225663120094_i128,(-95178619560755737021459585662522668464_i128),(-1206113493379392376185318365483732034_i128),(-96722835085940417291524899704945599313_i128)];
Call(_13 = core::intrinsics::transmute(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
RET = 166557167698670658063799344653592955524_u128;
_26 = '\u{d3523}';
_10 = [_7,_2];
_16 = [157267567883469806033780431250814597692_i128,168268727128111673693151357942949452983_i128,158605539239925155354423936988319752818_i128,89949955113544645502903620396706225640_i128,25568751926528797036171633688260305953_i128,(-142116449222295059204209115681370173032_i128),3842543260825911834097450635384556193_i128];
_7 = _2 | _5;
_16 = [142889017306346296949435899680543125976_i128,47352693862794816698198996327706904061_i128,40483857957951500520583609846641822141_i128,43324447776455325873110053262440260186_i128,(-116105723580008379848609350939850714234_i128),78966282443578838235382588632870963306_i128,13696592126978894287308064171395635718_i128];
RET = !74137617392684354322269442842070747551_u128;
_9 = _18;
_4 = [24495763642804755247655131477373138007_i128,127769169445680316216510791409529962072_i128,(-127404430851454052067372943675753218123_i128),148231362960804491956749331574514529513_i128,153596137018591259477595476316546629970_i128,(-66962653168374447222609126194493228064_i128),(-33441271790698365648045468103835573051_i128)];
_5 = -_2;
_16 = [94660653124515843266032756566968834799_i128,9701266434834532267411472592594951229_i128,28585291569842682327063966106011657055_i128,78942601380597983630695147697672349089_i128,157159194990370694522601286808231777198_i128,111420737494852168829524329801723546255_i128,34332410732889375363904080282401696625_i128];
_27 = [43236726609903170119525423186182062374_i128,(-32585936115789352313285429325043818115_i128)];
_21 = _3 | _2;
_24.0 = _4;
_13 = _14;
_1 = _6;
Call(_28 = core::intrinsics::bswap(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_10 = _11;
_16 = [2632818901348618557939969679909580202_i128,(-86561533921733023301004510180002530349_i128),88006616532767693640510121643800474665_i128,(-48409086635197833887346713445635489842_i128),65313254061342322589312369868575824605_i128,50090031477316253014076847899649650777_i128,(-69285954293993988753352270223880085553_i128)];
_6 = [_7,_3];
RET = !277499342241084255570743985122686294677_u128;
_22 = _3 as f64;
Call(_3 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_10 = [_21,_32];
_20 = 2755813332826458499_i64;
_27 = [_31,_31];
_4 = [_31,_31,_31,_31,_31,_31,_31];
_10 = [_21,_21];
_23 = !9652697919756908556_u64;
_35 = _25;
_27 = [_31,_31];
_28 = !_32;
_5 = _32 << _21;
_9 = [_29,_29,_29,_29,_29,_29,_29];
_18 = [_29,_29,_29,_29,_29,_29,_29];
_2 = _5 << _5;
_36 = &(*_34);
Goto(bb12)
}
bb12 = {
_20 = -(-1332398850575629748_i64);
Goto(bb13)
}
bb13 = {
(*_34) = '\u{cf76f}';
_45.2 = !_23;
_10 = [_32,_32];
_28 = _5;
_12 = _16;
Call(_29 = core::intrinsics::transmute(_5), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_12 = _14;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(11_usize, 29_usize, Move(_29), 38_usize, Move(_38), 24_usize, Move(_24), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(11_usize, 25_usize, Move(_25), 21_usize, Move(_21), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(11_usize, 20_usize, Move(_20), 4_usize, Move(_4), 8_usize, Move(_8), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(11_usize, 5_usize, Move(_5), 16_usize, Move(_16), 17_usize, Move(_17), 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: [usize; 7],mut _3: isize,mut _4: isize,mut _5: [i128; 7],mut _6: isize,mut _7: [isize; 2],mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> i64 {
mir! {
type RET = i64;
let _12: u128;
let _13: bool;
let _14: bool;
let _15: [i128; 7];
let _16: i128;
let _17: f32;
let _18: i8;
let _19: u32;
let _20: [i8; 3];
let _21: isize;
let _22: [i8; 4];
let _23: [i128; 2];
let _24: *mut char;
let _25: isize;
let _26: bool;
let _27: *const f32;
let _28: bool;
let _29: Adt53;
let _30: (*mut u8, &'static char, bool);
let _31: usize;
let _32: [i8; 3];
let _33: Adt51;
let _34: f64;
let _35: ([i128; 7], *const f64, ([i128; 2], [u16; 7], u64), [i8; 3], [i8; 4], *const i64, u32);
let _36: [u16; 7];
let _37: u128;
let _38: isize;
let _39: [i32; 8];
let _40: f32;
let _41: u32;
let _42: Adt47;
let _43: *mut f32;
let _44: [i128; 7];
let _45: u8;
let _46: f64;
let _47: ([i128; 2], [u16; 7], u64);
let _48: ([i128; 7],);
let _49: f64;
let _50: [i32; 8];
let _51: ();
let _52: ();
{
_6 = -_4;
RET = 213651500043411835765516099673427051441_u128 as i64;
_2 = [14877498059759917694_usize,1730980442446949964_usize,6_usize,5_usize,373624602945040359_usize,5811431850652725954_usize,6_usize];
RET = (-6614975395450984623_i64) | (-2744773958557157258_i64);
_4 = _8 - _3;
_5 = [(-134239169713783859838141076154176426915_i128),(-100984287626370601018274810636366062234_i128),34446834060839137315757151712439614045_i128,(-30967620584024125297061809688702036166_i128),(-88639963663861473659137965047098933018_i128),94127301183436210116562466735767239498_i128,(-77646241609347312386225732498515903601_i128)];
RET = 7645_i16 as i64;
_9 = _6;
_12 = 3973281651364908539_u64 as u128;
_2 = [3_usize,18244847810554291496_usize,5_usize,7846274469533042855_usize,1979369575747796379_usize,938013877497516819_usize,13347829296720486859_usize];
_5 = [24936125413133985661013465895310377246_i128,75329222512282409679532194432019568682_i128,130555668261754922903134774904007618997_i128,(-148966147330422612170756154551316018066_i128),(-144437425744195140243867380414485419648_i128),85575065714438027138446869232438057096_i128,98722337972834957186583682769117845966_i128];
RET = (-9121817707964606886_i64) ^ (-3595517018102016738_i64);
_8 = 1_usize as isize;
_2 = [12198629071428882900_usize,6214407703882592942_usize,12335149583270656045_usize,12702820388430790369_usize,8808731914826364069_usize,6316266418296008827_usize,2_usize];
_13 = true;
_13 = !false;
_9 = (-1749177844_i32) as isize;
_11 = -_6;
_7 = [_3,_4];
_12 = 188155737257252662326999342628152808100_u128;
Goto(bb1)
}
bb1 = {
_13 = !false;
_16 = 95315574039585878638401814644603717646_i128 ^ (-16474079126803137699305598627833112607_i128);
_7 = [_11,_4];
_9 = _16 as isize;
Call(_5 = fn13(_3, _4, _4, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _12 as i64;
_17 = 37171_u16 as f32;
_9 = _4;
_7 = [_9,_1];
Call(_19 = core::intrinsics::bswap(3896244437_u32), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_18 = (-60_i8);
_20 = [_18,_18,_18];
_7 = [_9,_4];
_2 = [1353678683707022769_usize,7_usize,17875233417557866328_usize,14381988639694933080_usize,16832608103485849774_usize,15019698343690638484_usize,7_usize];
_11 = !_6;
_10 = -_4;
_18 = (-92_i8) | (-71_i8);
_15 = [_16,_16,_16,_16,_16,_16,_16];
_22 = [_18,_18,_18,_18];
_11 = _1 << _3;
_6 = _4;
_22 = [_18,_18,_18,_18];
_13 = false;
_22 = [_18,_18,_18,_18];
RET = (-6338626141946682974_i64);
_15 = [_16,_16,_16,_16,_16,_16,_16];
_18 = -30_i8;
_21 = _3 << _10;
Goto(bb4)
}
bb4 = {
_16 = -36958148106321560490960699582586232217_i128;
_20 = [_18,_18,_18];
RET = (-77917714002242158_i64);
_12 = 4453836096571759487_usize as u128;
_1 = _4;
_22 = [_18,_18,_18,_18];
_13 = _21 == _6;
_21 = _6 | _11;
_14 = _13;
_10 = _11;
_17 = 361017552_i32 as f32;
_17 = 2700_u16 as f32;
_6 = _3 - _4;
_18 = 65_i8 * 124_i8;
_10 = _21 - _21;
_2 = [7197780363699766571_usize,4157337171570197554_usize,14847086066104643869_usize,16953102137837160102_usize,2_usize,7_usize,4_usize];
_9 = -_6;
_12 = 25858948896840556406567331048289368060_u128 | 12064202022182909934398169212439777062_u128;
RET = -3188549698580857085_i64;
Goto(bb5)
}
bb5 = {
_23 = [_16,_16];
_15 = _5;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_13 = !_14;
_23 = [_16,_16];
_30.2 = _6 == _9;
_29 = Adt53::Variant0 { fld0: 9039622480952864602_u64 };
place!(Field::<u64>(Variant(_29, 0), 0)) = 2046166629245656453_u64 & 7027096255695609717_u64;
_22 = [_18,_18,_18,_18];
_4 = !_1;
_11 = -_9;
_23 = [_16,_16];
_9 = _10 + _21;
_23 = [_16,_16];
_31 = _18 as usize;
Goto(bb6)
}
bb6 = {
RET = -(-4638705838735655428_i64);
_1 = -_11;
_8 = _11 >> _9;
_11 = _8 << _21;
_26 = _13;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_9 = _8 ^ _6;
_11 = _16 as isize;
_10 = _12 as isize;
_19 = !1151630999_u32;
_4 = _31 as isize;
_12 = 302396779820732928335752437456330947573_u128 ^ 241571913510022963467015527345067936304_u128;
_31 = !10464585174137919288_usize;
_20 = [_18,_18,_18];
_30.2 = _6 == _8;
_15 = [_16,_16,_16,_16,_16,_16,_16];
_12 = 243384992449621400042655405952867848717_u128;
_25 = _6;
_14 = _26 ^ _26;
_3 = _1 + _9;
_12 = !190246295539896492405148349349455073098_u128;
_28 = !_14;
_10 = _6 - _21;
Goto(bb7)
}
bb7 = {
_33.fld4 = _7;
_33.fld4 = [_21,_21];
Call(_5 = core::intrinsics::transmute(_15), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_28 = _10 >= _6;
_30.2 = _26;
_35.0 = _15;
_26 = _28 & _30.2;
_33.fld1 = core::ptr::addr_of!(_34);
_33.fld0 = [Field::<u64>(Variant(_29, 0), 0),Field::<u64>(Variant(_29, 0), 0),Field::<u64>(Variant(_29, 0), 0),Field::<u64>(Variant(_29, 0), 0),Field::<u64>(Variant(_29, 0), 0),Field::<u64>(Variant(_29, 0), 0)];
_33.fld3.0 = [_16,_16,_16,_16,_16,_16,_16];
_35.3 = [_18,_18,_18];
SetDiscriminant(_29, 1);
_30.2 = _28 | _14;
Goto(bb9)
}
bb9 = {
_35.1 = core::ptr::addr_of!(_34);
place!(Field::<([i128; 2], [u16; 7], u64)>(Variant(_29, 1), 0)).1 = [65506_u16,393_u16,19328_u16,39199_u16,3463_u16,53526_u16,40515_u16];
_35.6 = _19;
_33.fld1 = core::ptr::addr_of!(_34);
_30.2 = _9 == _10;
place!(Field::<Adt44>(Variant(_29, 1), 1)) = Adt44 { fld0: (-12117_i16),fld1: _15 };
_14 = _21 < _10;
_32 = [_18,_18,_18];
_33.fld4 = [_3,_6];
_36 = Field::<([i128; 2], [u16; 7], u64)>(Variant(_29, 1), 0).1;
_36 = Field::<([i128; 2], [u16; 7], u64)>(Variant(_29, 1), 0).1;
place!(Field::<([i128; 2], [u16; 7], u64)>(Variant(_29, 1), 0)).0 = [_16,_16];
_35.2.0 = [_16,_16];
place!(Field::<[i32; 8]>(Variant(_29, 1), 4)) = [(-1526248076_i32),2088681854_i32,(-1915598499_i32),(-690232524_i32),(-1812053142_i32),865657317_i32,(-805641909_i32),(-894150508_i32)];
_20 = [_18,_18,_18];
_14 = _30.2 & _28;
_14 = !_26;
_4 = Field::<Adt44>(Variant(_29, 1), 1).fld0 as isize;
_27 = core::ptr::addr_of!(_17);
_33.fld3.0 = [_16,_16,_16,_16,_16,_16,_16];
_29 = Adt53::Variant0 { fld0: 13282275601145313696_u64 };
_35.2.2 = _12 as u64;
_35.2.1 = _36;
_35.5 = core::ptr::addr_of!(RET);
place!(Field::<u64>(Variant(_29, 0), 0)) = _35.2.2 * _35.2.2;
Goto(bb10)
}
bb10 = {
_33.fld3.0 = [_16,_16,_16,_16,_16,_16,_16];
_33.fld3.0 = _35.0;
(*_27) = 3_u8 as f32;
_23 = [_16,_16];
_35.5 = core::ptr::addr_of!(RET);
_32 = [_18,_18,_18];
_28 = _30.2 | _13;
_1 = _10;
_38 = !_1;
_39 = [(-1959601715_i32),(-1801266169_i32),(-578038028_i32),(-817444282_i32),(-1678972769_i32),282667313_i32,1394545594_i32,214004816_i32];
_35.6 = (*_27) as u32;
_28 = _13;
_14 = _13 ^ _30.2;
Goto(bb11)
}
bb11 = {
_42 = Adt47::Variant2 { fld0: _35.5,fld1: _17 };
_35.0 = [_16,_16,_16,_16,_16,_16,_16];
_33.fld1 = core::ptr::addr_of!(_34);
_35.2.0 = _23;
_3 = _13 as isize;
_35.2.2 = !Field::<u64>(Variant(_29, 0), 0);
_37 = _12;
_7 = [_21,_25];
_21 = -_25;
_30.0 = core::ptr::addr_of_mut!(_45);
_33.fld2 = [_31,_31,_31,_31,_31,_31,_31];
_17 = Field::<f32>(Variant(_42, 2), 1) - Field::<f32>(Variant(_42, 2), 1);
_43 = core::ptr::addr_of_mut!(_40);
RET = !(-6420498856869545651_i64);
_33.fld2 = [_31,_31,_31,_31,_31,_31,_31];
_33.fld1 = core::ptr::addr_of!(_46);
_33.fld2 = [_31,_31,_31,_31,_31,_31,_31];
_8 = _21 << _1;
_47.2 = _35.2.2;
_34 = 204_u8 as f64;
place!(Field::<*const i64>(Variant(_42, 2), 0)) = core::ptr::addr_of!(RET);
SetDiscriminant(_42, 1);
_35.2.0 = [_16,_16];
_35.5 = core::ptr::addr_of!(RET);
Goto(bb12)
}
bb12 = {
_16 = (-25099375948997024926427638483228655515_i128);
_40 = -(*_27);
_18 = (-16_i8) ^ (-103_i8);
(*_27) = -(*_43);
_2 = [_31,_31,_31,_31,_31,_31,_31];
_35.3 = _32;
place!(Field::<i64>(Variant(_42, 1), 3)) = RET + RET;
_33.fld1 = _35.1;
match _16 {
0 => bb10,
1 => bb7,
2 => bb5,
315182990971941438536946968948539555941 => bb14,
_ => bb13
}
}
bb13 = {
_23 = [_16,_16];
_15 = _5;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_13 = !_14;
_23 = [_16,_16];
_30.2 = _6 == _9;
_29 = Adt53::Variant0 { fld0: 9039622480952864602_u64 };
place!(Field::<u64>(Variant(_29, 0), 0)) = 2046166629245656453_u64 & 7027096255695609717_u64;
_22 = [_18,_18,_18,_18];
_4 = !_1;
_11 = -_9;
_23 = [_16,_16];
_9 = _10 + _21;
_23 = [_16,_16];
_31 = _18 as usize;
Goto(bb6)
}
bb14 = {
_45 = 6_u8;
_33.fld3.0 = [_16,_16,_16,_16,_16,_16,_16];
(*_27) = -(*_43);
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(12_usize, 7_usize, Move(_7), 18_usize, Move(_18), 25_usize, Move(_25), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(12_usize, 3_usize, Move(_3), 6_usize, Move(_6), 31_usize, Move(_31), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(12_usize, 36_usize, Move(_36), 1_usize, Move(_1), 20_usize, Move(_20), 45_usize, Move(_45)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(12_usize, 12_usize, Move(_12), 16_usize, Move(_16), 9_usize, Move(_9), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _5: ([i128; 2], [u16; 7], u64);
let _6: [usize; 7];
let _7: u128;
let _8: isize;
let _9: i16;
let _10: bool;
let _11: ([i128; 2], [u16; 7], u64);
let _12: [i128; 2];
let _13: f32;
let _14: *const i16;
let _15: isize;
let _16: [isize; 2];
let _17: [u64; 6];
let _18: isize;
let _19: Adt44;
let _20: [i128; 2];
let _21: [usize; 7];
let _22: [char; 3];
let _23: u64;
let _24: u8;
let _25: [i8; 4];
let _26: char;
let _27: i16;
let _28: isize;
let _29: ([i128; 2], [u16; 7], u64);
let _30: bool;
let _31: [i32; 8];
let _32: Adt48;
let _33: (*mut u8, &'static char, bool);
let _34: [isize; 2];
let _35: [usize; 7];
let _36: isize;
let _37: usize;
let _38: ([i128; 7],);
let _39: Adt56;
let _40: [i8; 3];
let _41: Adt55;
let _42: *mut char;
let _43: [isize; 2];
let _44: ();
let _45: ();
{
RET = [81227680270785442834447297245148945780_i128,131993023702241027416919575784468401621_i128,(-54654246781186374948316681741122501792_i128),129694923324459014286890495790041230795_i128,(-86473237095195639439919059890887533834_i128),(-89717053564661155258299277918901680561_i128),(-129270001214304346010769217913905554045_i128)];
_3 = 741324232_u32 as isize;
_4 = _2;
_3 = _2 << _4;
_3 = _4;
_5.0 = [(-143559374642083495595264142262139935276_i128),115192346976286955845835111470364822650_i128];
_5.1 = [7726_u16,55111_u16,2606_u16,651_u16,29142_u16,26460_u16,17615_u16];
_3 = 33698_u16 as isize;
_5.0 = [81258490204478284344194653038680721200_i128,(-153602543244891493997125175825028850348_i128)];
_5.2 = !14470718166439316063_u64;
_2 = -_4;
RET = [673248029585417589630784920947349614_i128,66995166776424131408036125565393844283_i128,(-169177580067465274653511134627365583240_i128),(-9102587581156946090939196598464011621_i128),(-30392091545515502321460500168092742059_i128),123235133027322455937372689670869200798_i128,(-157411637278210075601083790662775636857_i128)];
RET = [98171116568209883037224090623088035360_i128,(-135357824914223660119259211523266923576_i128),(-129932343030250375534016897223418159834_i128),120341165700658918168893992522306389634_i128,(-81597032351387066379066890556686161872_i128),15463651111051420917560003443749490639_i128,(-62946388022655152255326703020293267417_i128)];
_6 = [7_usize,7_usize,7979604959264831340_usize,16014397225437337228_usize,1933518585239477117_usize,7_usize,10361386453142954567_usize];
_1 = (-1883442986402517934_i64) as isize;
Goto(bb1)
}
bb1 = {
_9 = (-21139_i16);
_11 = _5;
_11.1 = [3716_u16,54494_u16,53397_u16,6095_u16,32318_u16,54913_u16,20364_u16];
_5 = _11;
_7 = !216476727392795551861294218209378361005_u128;
match _9 {
340282366920938463463374607431768190317 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_5 = _11;
_5.2 = !_11.2;
RET = [110505423276688895572017395856491614439_i128,(-14061609283217993378936066749022320825_i128),(-4417729515013521852469387094789254768_i128),(-43521195462208660511819905829295383340_i128),15356345594844837043659662768758099597_i128,101791221366311457100147470554867914969_i128,55206473061501170465359218388271794552_i128];
_11.1 = [64515_u16,11672_u16,3600_u16,39384_u16,60692_u16,54864_u16,36051_u16];
_9 = (-508_i16) << _4;
_3 = _2;
_1 = _9 as isize;
_11.1 = [59390_u16,58352_u16,7245_u16,39784_u16,9956_u16,57632_u16,30718_u16];
_7 = 271938491053481257181062468191897504479_u128;
_5 = (_11.0, _11.1, _11.2);
RET = [83829533743631009012980380339199071274_i128,92555756456738827310336344047828130034_i128,28618727517642094744407519123468771827_i128,(-69078008011469220880783602114596181184_i128),57520013546889331323277078023392172475_i128,39547078637463838452055788808049872103_i128,(-135883560714316578518333887386124609378_i128)];
_3 = 7_usize as isize;
_9 = (-7129_i16);
RET = [37174716058180601335020676766462696350_i128,79872448624630536356536101546019384249_i128,(-72774167110126212283839890825063734601_i128),(-72233911953723900308833538074663546203_i128),(-47672075952003308352574177044301994941_i128),109265269983373864366709621991010713944_i128,(-7801063997767512805307758498497213136_i128)];
_11 = _5;
_11 = (_5.0, _5.1, _5.2);
RET = [(-50215303208112308847945240041473382708_i128),161638046260271149366762931062850641030_i128,(-69461668399373403866461755651872767937_i128),123192799972912613253761170080532041254_i128,(-20862792947606556258896980061726726632_i128),115678341389013707918993690175386987825_i128,(-68575342729087617297926979138880060512_i128)];
_5 = _11;
_10 = !true;
_16 = [_1,_4];
_15 = '\u{324e1}' as isize;
_11.0 = [(-124156707460443383251191571506114150738_i128),2373104542215091920197129588145587607_i128];
_8 = _1;
_17 = [_5.2,_11.2,_11.2,_5.2,_11.2,_5.2];
_9 = (-29419_i16) | (-8356_i16);
_15 = _1 & _8;
Call(_11.0 = core::intrinsics::transmute(_5.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = _15 + _1;
_5.0 = _11.0;
_12 = _5.0;
_7 = !273324195523043000444610658440203599266_u128;
_3 = 92_i8 as isize;
_8 = -_15;
_3 = _1 | _15;
_12 = _5.0;
_11.0 = [(-76747921969928314436090009576794975508_i128),(-168265211650702354407528616454288575851_i128)];
_18 = !_3;
_5.2 = _11.2 << _3;
_7 = !69285745354254386220324717501738377424_u128;
_6 = [7_usize,0_usize,6_usize,4_usize,2_usize,1859536668587613865_usize,18400010381724435530_usize];
_19.fld0 = !_9;
_14 = core::ptr::addr_of!(_19.fld0);
_11 = _5;
_5.0 = _12;
_5.2 = !_11.2;
_11.2 = !_5.2;
RET = [(-15395536097713889791098966004745840957_i128),141121459750641707324311577889905660330_i128,84876856533551548596263506170051104481_i128,(-148223088094285996859577218783177941883_i128),(-39619200949554170801905257080924386912_i128),(-96409848990286600314741633401583685441_i128),160372431479338201447086435124327321892_i128];
_13 = 151009544026106300111198747038388749992_i128 as f32;
Goto(bb5)
}
bb5 = {
(*_14) = -_9;
_12 = [49567892867060666040482355054895826050_i128,(-159335392561975455497784656643932338043_i128)];
Call(_19.fld1 = core::intrinsics::transmute(RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = [_8,_3];
_9 = -(*_14);
_21 = _6;
RET = [(-103950550348916004171652801363961154971_i128),(-27426764208286493339736704719477378635_i128),112744551546848789011198425377925149699_i128,(-23711253033281181267036707131659018740_i128),155980272986906429769380076082688761018_i128,111575851911573653885862524528038396454_i128,73045535300809740262982691648692541831_i128];
_11.1 = _5.1;
_15 = _8;
_11.2 = _5.2;
_17 = [_5.2,_5.2,_5.2,_11.2,_5.2,_11.2];
_3 = !_18;
_20 = _5.0;
_11 = _5;
_5.0 = [2977376839907681865314446717729979593_i128,(-39363727438839226568899531868063018097_i128)];
(*_14) = -_9;
_19.fld0 = !_9;
_11.2 = _5.2;
_11.0 = _20;
_11.2 = (-1983438101_i32) as u64;
_19.fld1 = RET;
_5.1 = _11.1;
_6 = [2_usize,6645700107501639435_usize,6_usize,3_usize,16484003284273579111_usize,4_usize,15205405261498277292_usize];
_23 = 34598_u16 as u64;
_5.0 = [79696526558626496333298628000124127509_i128,(-74294352028540961319477661667182627709_i128)];
_5 = _11;
_24 = !118_u8;
_19.fld0 = -_9;
_15 = _2;
_5.1 = _11.1;
(*_14) = _9 ^ _9;
_8 = !_18;
Goto(bb7)
}
bb7 = {
_17 = [_23,_11.2,_5.2,_11.2,_5.2,_5.2];
_23 = !_5.2;
_17 = [_23,_11.2,_23,_5.2,_11.2,_11.2];
_18 = !_2;
_19.fld1 = [(-113686967218697270414111045064629018160_i128),56990192968947995903919819711750376014_i128,107014832562476124353972723961101513067_i128,61475087947281932013996430090984644137_i128,(-62953512562773802627193112366251279084_i128),(-94115937493918177211226986315566180594_i128),(-118802716994881271590655968306421048686_i128)];
_11.2 = _23;
_4 = _1 | _3;
_10 = false;
RET = [86401014649821165747150462162641227565_i128,(-88362732938959529371706062319103198937_i128),59631475157025453418976110880800666612_i128,(-112350691337232893384491922803150836266_i128),(-76669918344039722694851559744906331732_i128),(-163065669597690048743504522057960445424_i128),(-117585257041542459959547227677613112390_i128)];
_15 = _2;
_13 = 59252_u16 as f32;
_20 = [(-23971741231962382523371039794388726143_i128),(-98187186572093715632847875663102603710_i128)];
RET = [(-147877007219861831923873923534546459126_i128),83505153882280139618397615761875857680_i128,20212002044650675544750713372475268961_i128,164321732347613530755228745635979535636_i128,(-58141196123908252175228566484151415953_i128),(-66178354180067002150396188843541119004_i128),(-129374227785093917896551917410905987396_i128)];
_7 = 143635985378970491087838866108395008677_u128 ^ 274239849514885427153781603151308478507_u128;
Goto(bb8)
}
bb8 = {
_23 = _11.2;
_19.fld1 = [37490494073219252249168086488989068105_i128,(-159505835999000609705801889304889322260_i128),(-90325968772840079989970685489229831074_i128),(-113523930170371115113898891789232767295_i128),(-11694550799773519815064443106977956953_i128),(-136766886948417673708223855202943680899_i128),(-115563479084483275814410652442026708211_i128)];
_10 = !true;
_24 = !151_u8;
Call(_20 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21 = _6;
_12 = [129098813044302747390115374673058489920_i128,154384132376497678309428583618160026412_i128];
_12 = [(-167957974697720329150897376804077823987_i128),(-36712543887012029176023871050769894374_i128)];
_13 = 1678415657_u32 as f32;
_11 = (_20, _5.1, _5.2);
_12 = [100933378959394921170263076228318171262_i128,161711336779825725857189778351494413319_i128];
(*_14) = _9 * _9;
_11 = (_20, _5.1, _23);
_14 = core::ptr::addr_of!(_9);
_5.2 = _11.2 << _3;
_11.0 = [152295623962170754149756060453167539034_i128,(-437376706780392333662298530750152896_i128)];
_13 = _24 as f32;
_12 = [(-108813279659883171729303962693756600929_i128),105529113444815901976594966193122129259_i128];
_19.fld1 = [(-154299270944281253604643460262898919936_i128),151512313988577797911112777042844197081_i128,(-148669818526903065479935847295979927845_i128),167516220844459334436580205301437411623_i128,(-3397535691768264868278637016011082266_i128),117006719689471275359148913882110595354_i128,(-82634044079024981652891807395940216645_i128)];
(*_14) = _19.fld0 << _2;
RET = _19.fld1;
_4 = _2;
Call(_23 = fn14(_1, _18, _2, _20, _3, (*_14), _4, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = _2;
_3 = 91834448512145295722635071341472889208_i128 as isize;
_5.2 = _11.2;
_22 = ['\u{b16c5}','\u{d9003}','\u{5d15b}'];
_3 = _2 * _8;
_21 = [3_usize,16206836431197790443_usize,16571650744896831360_usize,6536959182491882890_usize,4_usize,14333571098916990280_usize,4340994373540681941_usize];
_11.2 = _23 >> _15;
_16 = [_3,_2];
RET = [101162366684129550115063660858188564056_i128,151558615708421777743848321984949804940_i128,(-18737772987806038922181727849167274495_i128),128460311044139119236484951263977007314_i128,30731380848311912980348739784225663422_i128,(-84846932629324100425365588411781982466_i128),(-159297027342946343250340985666802653629_i128)];
Goto(bb11)
}
bb11 = {
_10 = _15 != _8;
_11.1 = [24725_u16,27166_u16,54307_u16,30207_u16,64389_u16,6035_u16,60860_u16];
(*_14) = _19.fld0 + _19.fld0;
_5.1 = [37529_u16,2430_u16,33547_u16,29378_u16,27167_u16,5332_u16,5479_u16];
_19.fld0 = 102_i8 as i16;
_15 = (-59923235237920342450601285926990925422_i128) as isize;
RET = _19.fld1;
_19 = Adt44 { fld0: (*_14),fld1: RET };
(*_14) = _19.fld0;
_6 = [1155576823038677728_usize,1_usize,7_usize,4236005878308276145_usize,12332040179758290249_usize,1128569820073765586_usize,13470983265299759144_usize];
_6 = [0_usize,12204223313569511613_usize,3_usize,15068891115425736417_usize,1_usize,3175689113103979630_usize,8559948913207314085_usize];
_24 = 13_u8 - 125_u8;
_5.0 = [157696488105769945806486516289597523830_i128,(-98416562931307508710582943662572666270_i128)];
_25 = [(-8_i8),(-123_i8),73_i8,(-77_i8)];
_6 = [2_usize,3_usize,2_usize,6_usize,14108368660939746668_usize,1_usize,4_usize];
RET = _19.fld1;
_5.2 = _11.2 << _3;
_19 = Adt44 { fld0: (*_14),fld1: RET };
_5.2 = (-7705754732786952034_i64) as u64;
_17 = [_11.2,_11.2,_11.2,_11.2,_11.2,_11.2];
_29.1 = _11.1;
_28 = _2;
_20 = [113123936712878413659533994823319452648_i128,58395399311527015809254886992042186278_i128];
Goto(bb12)
}
bb12 = {
_26 = '\u{d6a1a}';
_27 = _9;
_1 = _8;
_10 = _1 == _4;
_21 = _6;
_5.0 = [141209809482188311453379132569999639834_i128,50661535474494380374191896571745702482_i128];
_25 = [(-74_i8),92_i8,(-84_i8),(-26_i8)];
_1 = -_8;
(*_14) = 1_usize as i16;
_24 = !78_u8;
_5 = (_11.0, _29.1, _11.2);
_22 = [_26,_26,_26];
_16 = [_28,_8];
_11.1 = _29.1;
_29 = _11;
(*_14) = _27 * _19.fld0;
_5.0 = _29.0;
_29.2 = (-121_i8) as u64;
_9 = _19.fld0 ^ _27;
_33.2 = _10 | _10;
Call(_31 = fn15(_10, _8, _28, _4, _16, _18, _16, _17, _16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10 = _33.2 & _33.2;
_30 = !_10;
_19.fld1 = RET;
_22 = [_26,_26,_26];
_30 = !_33.2;
_9 = _27;
_34 = _16;
_28 = _24 as isize;
_7 = !145038818506378469981273732000473315953_u128;
_13 = (-68_i8) as f32;
_16 = [_4,_2];
_13 = _7 as f32;
(*_14) = !_27;
_18 = -_4;
_5.0 = [22456908090605305325278004238596320240_i128,93470925736905860640335334076871123859_i128];
_13 = 18373972896070339764_usize as f32;
_14 = core::ptr::addr_of!(_27);
_29.2 = _5.2;
_1 = _26 as isize;
_33.0 = core::ptr::addr_of_mut!(_24);
_33.1 = &_26;
_19 = Adt44 { fld0: (*_14),fld1: RET };
_27 = -_9;
Goto(bb14)
}
bb14 = {
_17 = [_23,_5.2,_11.2,_29.2,_29.2,_5.2];
_20 = _11.0;
(*_14) = _26 as i16;
_25 = [(-70_i8),50_i8,(-127_i8),(-18_i8)];
_38.0 = [152391630042588985822841234566138895295_i128,133237975436888862019648111556695309996_i128,109670039920605589232110384837606443339_i128,39498284469510180393051394831131832862_i128,(-159700127493141142677621283173382720_i128),(-27745631607415197669581763097074068443_i128),36319182357266637100466858547199381482_i128];
RET = [(-31811143327088490520367855079318036337_i128),(-78939920871172747991390009417041224379_i128),22140421172315788269107524584284056333_i128,65499124006292675396553535964341004783_i128,(-5319543835514662195932443572450583634_i128),134070639984586293666289003429898308769_i128,(-82294276178991293168867325359702941140_i128)];
_29.0 = [(-31154195569251631775121997562514232370_i128),105238159338532398991258073415535623750_i128];
_13 = 4055_u16 as f32;
_36 = _2;
_11.2 = (-89_i8) as u64;
_33.1 = &_26;
_8 = (-1386759732_i32) as isize;
_21 = _6;
_33.0 = core::ptr::addr_of_mut!(_24);
_12 = _29.0;
_5.1 = _11.1;
_37 = 13214254214090305891_usize;
_26 = '\u{81691}';
_19.fld0 = _36 as i16;
_23 = _29.2;
_7 = 135288999954889788381677331871572220985_u128 + 325104721716254423540660005130339465497_u128;
_19.fld1 = [(-23961919180311772194014361934551576183_i128),27516446690445838780719100426163087892_i128,(-19804421987086636789871271882695845346_i128),(-166482799322823753418108435189032852476_i128),93645081245820520706472811793526736772_i128,(-40243464279234685683698375180141183139_i128),(-102019236517282797918753985431753740819_i128)];
_22 = [_26,_26,_26];
_30 = !_33.2;
_15 = -_18;
_3 = _7 as isize;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(13_usize, 1_usize, Move(_1), 22_usize, Move(_22), 38_usize, Move(_38), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(13_usize, 29_usize, Move(_29), 20_usize, Move(_20), 37_usize, Move(_37), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(13_usize, 8_usize, Move(_8), 4_usize, Move(_4), 36_usize, Move(_36), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(13_usize, 26_usize, Move(_26), 30_usize, Move(_30), 23_usize, Move(_23), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [i128; 2],mut _5: isize,mut _6: i16,mut _7: isize,mut _8: isize) -> u64 {
mir! {
type RET = u64;
let _9: isize;
let _10: bool;
let _11: Adt45;
let _12: i8;
let _13: ([i128; 2], [u16; 7], u64);
let _14: Adt52;
let _15: isize;
let _16: f64;
let _17: [i8; 3];
let _18: f64;
let _19: Adt60;
let _20: char;
let _21: u32;
let _22: [i8; 4];
let _23: u32;
let _24: f32;
let _25: (bool, [i32; 8], bool, i32, ((*mut u8, &'static char, bool), char, [i8; 4]), i64, usize);
let _26: i16;
let _27: f32;
let _28: isize;
let _29: [i32; 8];
let _30: u32;
let _31: i128;
let _32: [u64; 6];
let _33: i16;
let _34: ();
let _35: ();
{
RET = 157577614953873808_u64 * 5039964715512981543_u64;
RET = 1689849607434498967_u64 * 10760173979193677404_u64;
_7 = !_3;
_5 = _8;
Goto(bb1)
}
bb1 = {
RET = 14893821046692696297_u64;
_8 = -_2;
_1 = 3636663435_u32 as isize;
_7 = _5;
RET = '\u{9e85a}' as u64;
Goto(bb2)
}
bb2 = {
_2 = -_5;
RET = !10739543441797731656_u64;
_11.fld1 = 93026913006232856713593837364482765152_u128 as i8;
_2 = _3;
_11.fld1 = (-32_i8) & (-88_i8);
_7 = !_3;
_11.fld0 = core::ptr::addr_of!(_9);
_2 = !_5;
_11.fld0 = core::ptr::addr_of!(_7);
_6 = 19014_i16;
_1 = (-4697020416026024648_i64) as isize;
_9 = _2 - _2;
_10 = false;
_11.fld0 = core::ptr::addr_of!(_3);
_1 = _3 >> _2;
_12 = -_11.fld1;
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
19014 => bb8,
_ => bb7
}
}
bb3 = {
RET = 14893821046692696297_u64;
_8 = -_2;
_1 = 3636663435_u32 as isize;
_7 = _5;
RET = '\u{9e85a}' as u64;
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
_8 = _2;
_13.1 = [36130_u16,54665_u16,30042_u16,23062_u16,59722_u16,51649_u16,16807_u16];
_11.fld0 = core::ptr::addr_of!(_5);
_11.fld1 = _12 << _2;
_13.1 = [37841_u16,53207_u16,7411_u16,49107_u16,58858_u16,53382_u16,38764_u16];
_11.fld0 = core::ptr::addr_of!(_9);
_7 = 4_usize as isize;
_7 = _1 ^ _8;
_2 = -_9;
_13.0 = _4;
_11.fld2 = 46764_u16 as u128;
_11.fld2 = !256794298152806539304901903203615943766_u128;
_15 = _11.fld2 as isize;
_16 = _11.fld2 as f64;
Goto(bb9)
}
bb9 = {
_8 = _11.fld1 as isize;
_4 = _13.0;
_11.fld1 = !_12;
_12 = _11.fld1 - _11.fld1;
_9 = !_1;
_13.2 = !RET;
_3 = 2646163945_u32 as isize;
_13.2 = 1324012831_u32 as u64;
_5 = -_7;
_7 = _5;
_20 = '\u{4236d}';
_9 = _16 as isize;
_1 = _2 & _7;
match _6 {
0 => bb5,
1 => bb7,
2 => bb6,
19014 => bb10,
_ => bb4
}
}
bb10 = {
_23 = 3117109374_u32;
_20 = '\u{c60f3}';
_18 = -_16;
match _6 {
0 => bb1,
1 => bb5,
19014 => bb11,
_ => bb3
}
}
bb11 = {
_5 = _7;
RET = 1065161333_i32 as u64;
_13.0 = [3931927887476926952664214958660261398_i128,(-135572632229358492598721405591712981672_i128)];
_25.5 = !9070090704177352845_i64;
_15 = !_5;
_11.fld1 = _23 as i8;
_11.fld1 = !_12;
_17 = [_11.fld1,_11.fld1,_12];
_22 = [_12,_12,_12,_12];
Goto(bb12)
}
bb12 = {
_21 = _18 as u32;
_3 = _8 << _2;
RET = _13.2;
RET = !_13.2;
RET = _13.2;
_25.6 = !11003923735467190664_usize;
_3 = -_1;
_25.0 = _1 <= _15;
_25.4.2 = _22;
_10 = _7 < _2;
_16 = -_18;
_22 = _25.4.2;
_3 = _5;
_2 = _11.fld2 as isize;
_25.2 = _25.0;
_25.4.0.1 = &_25.4.1;
RET = _13.2 & _13.2;
_17 = [_11.fld1,_11.fld1,_11.fld1];
_25.1 = [315431056_i32,(-771770037_i32),458962822_i32,1944234045_i32,1426280745_i32,1145223358_i32,234239024_i32,(-1476003882_i32)];
_25.4.0.1 = &_20;
_25.4.1 = _20;
_25.5 = 1097164694864240030_i64;
_12 = _11.fld1 + _11.fld1;
_25.4.1 = _20;
match _25.5 {
0 => bb9,
1097164694864240030 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_32 = [RET,RET,RET,RET,RET,RET];
_26 = _6 | _6;
_27 = _25.6 as f32;
_23 = _21;
_26 = -_6;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(14_usize, 13_usize, Move(_13), 32_usize, Move(_32), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(14_usize, 20_usize, Move(_20), 26_usize, Move(_26), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(14_usize, 6_usize, Move(_6), 9_usize, Move(_9), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 2],mut _6: isize,mut _7: [isize; 2],mut _8: [u64; 6],mut _9: [isize; 2]) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _10: i8;
let _11: i16;
let _12: [i8; 3];
let _13: [i8; 3];
let _14: Adt57;
let _15: usize;
let _16: [i8; 3];
let _17: ([i128; 7],);
let _18: [char; 3];
let _19: [usize; 7];
let _20: Adt47;
let _21: u128;
let _22: [i128; 2];
let _23: u128;
let _24: [usize; 7];
let _25: i128;
let _26: [i8; 3];
let _27: [i128; 7];
let _28: Adt46;
let _29: f32;
let _30: u8;
let _31: *mut u8;
let _32: [i128; 7];
let _33: *mut f32;
let _34: u32;
let _35: [i8; 3];
let _36: [i8; 3];
let _37: Adt44;
let _38: [i8; 3];
let _39: (bool, [i32; 8], bool, i32, ((*mut u8, &'static char, bool), char, [i8; 4]), i64, usize);
let _40: Adt58;
let _41: isize;
let _42: ();
let _43: ();
{
_3 = _2 | _4;
_9 = _5;
_6 = _4;
_9 = [_4,_4];
RET = [1833303080_i32,2078557712_i32,284873139_i32,(-1313703552_i32),1755508875_i32,1230119767_i32,1119999563_i32,(-1125117411_i32)];
_5 = _9;
_10 = 335147689782757099370454709519251559711_u128 as i8;
_2 = !_6;
_2 = 239_u8 as isize;
_7 = _9;
_9 = _5;
_5 = [_4,_4];
RET = [44013596_i32,(-959121178_i32),1073952770_i32,1133720212_i32,1083312871_i32,238769864_i32,1871901652_i32,438915032_i32];
_8 = [16764212309352192016_u64,5955637105524639090_u64,10060913785042547512_u64,5014681565659960488_u64,8050848620736603104_u64,2444417584093016051_u64];
_10 = (-76_i8) >> _3;
_6 = _4 << _3;
_1 = true;
RET = [1424433899_i32,(-894400499_i32),1200918833_i32,1742391421_i32,555812153_i32,(-457414719_i32),1104304257_i32,(-132728320_i32)];
RET = [(-463078050_i32),(-1110364407_i32),217659037_i32,(-1032744286_i32),(-107694737_i32),1375508136_i32,(-520633095_i32),(-1873966394_i32)];
RET = [(-328484750_i32),(-1915159251_i32),(-1773584040_i32),(-765353755_i32),(-1233232754_i32),424944323_i32,(-577418935_i32),(-576570701_i32)];
RET = [478964357_i32,(-1798928558_i32),1327202611_i32,(-735738628_i32),549576054_i32,901762140_i32,917902658_i32,819808959_i32];
_6 = _4 ^ _4;
_7 = [_6,_3];
_2 = 43_u8 as isize;
Goto(bb1)
}
bb1 = {
RET = [(-336142279_i32),(-1518913453_i32),(-797174605_i32),(-1737062569_i32),(-1700641271_i32),775892656_i32,(-1557966584_i32),231607111_i32];
_6 = -_3;
_9 = [_3,_6];
_1 = _4 < _3;
_3 = _4;
_11 = 12806_i16;
_11 = 4637_i16;
_9 = _5;
_7 = _5;
_1 = _4 <= _6;
_11 = !26961_i16;
_7 = [_4,_6];
RET = [(-1421513480_i32),696611487_i32,(-56166539_i32),(-728473562_i32),(-33507934_i32),548698127_i32,1026208649_i32,(-1614305296_i32)];
_11 = 10786_i16;
_9 = _5;
Goto(bb2)
}
bb2 = {
_5 = [_4,_3];
_8 = [12403872561525718082_u64,18104831470766867979_u64,3718199498794092309_u64,11577210349970444402_u64,13258999489258416942_u64,5581264273152462040_u64];
_9 = [_3,_6];
_11 = (-18280_i16) ^ 12064_i16;
RET = [(-838317692_i32),938690829_i32,(-1403564126_i32),(-628100374_i32),1687000952_i32,(-175787814_i32),480649162_i32,(-1082056256_i32)];
_2 = _4 & _6;
RET = [637516006_i32,1517177092_i32,2063269021_i32,(-454563726_i32),1675395435_i32,(-929984451_i32),(-421137397_i32),(-36774510_i32)];
_7 = [_2,_3];
RET = [(-1973971244_i32),804282576_i32,1499039786_i32,(-1664358471_i32),377965264_i32,(-26963333_i32),(-1817273360_i32),85629092_i32];
_9 = [_3,_4];
_5 = [_3,_4];
_3 = _4;
Call(_11 = fn16(_5, _1, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Goto(bb4)
}
bb4 = {
_12 = [_10,_10,_10];
_6 = _10 as isize;
Goto(bb5)
}
bb5 = {
_15 = !4_usize;
_8 = [5607339753020126186_u64,16716784627129567589_u64,16585070803408947793_u64,6475666065893297427_u64,16137651830599674945_u64,6481255762490565861_u64];
RET = [1795530191_i32,795875153_i32,(-2023599687_i32),(-546696061_i32),(-1460208792_i32),(-979183048_i32),(-86198596_i32),(-1166019883_i32)];
_5 = [_3,_3];
_18 = ['\u{ecd73}','\u{bdf4a}','\u{a18ac}'];
_17.0 = [51812437283331509536193446135767216155_i128,85524576771894922090698088696614329643_i128,94154847503685563000839685900539473192_i128,(-37616432153489305380862326737356075819_i128),(-124026355140142010733339730821347610826_i128),128223247868465664149003882554593377447_i128,148886878706974163271244580370057277759_i128];
RET = [(-265237824_i32),507646392_i32,1327897501_i32,(-1462404092_i32),(-1190070654_i32),757076437_i32,1361023116_i32,490512894_i32];
_19 = [_15,_15,_15,_15,_15,_15,_15];
_1 = false ^ false;
_1 = true ^ true;
_2 = -_3;
_16 = [_10,_10,_10];
_17.0 = [(-167128249129656794960038882154964292504_i128),(-35946015856612441108993029542435581926_i128),(-18404851261238522499317336156205973533_i128),(-39404959741281842786275268103932349672_i128),10935596676999583984564920970495550886_i128,39122877726345544148383793576588971951_i128,63576827186161432601126218227398238638_i128];
_8 = [9860784864829529876_u64,6028613710532104120_u64,8876011222069643919_u64,3862600327180880908_u64,15261895699938166849_u64,1696615849245376550_u64];
_2 = 17301223354748128385_u64 as isize;
_12 = [_10,_10,_10];
_2 = _15 as isize;
_6 = _3;
_5 = [_4,_4];
_1 = true | true;
_13 = [_10,_10,_10];
_11 = -(-32392_i16);
Goto(bb6)
}
bb6 = {
_21 = !227937396425399123030539794667494242002_u128;
_3 = -_6;
Goto(bb7)
}
bb7 = {
_4 = 1259687697_i32 as isize;
_11 = 20183_i16 * 15976_i16;
_15 = 5395538194221973690_usize;
Goto(bb8)
}
bb8 = {
_21 = _11 as u128;
_15 = !17738029476035127042_usize;
_7 = [_6,_6];
_21 = '\u{9341f}' as u128;
_21 = 232995784736606283099411391088997421951_u128;
_25 = 104575201371536239740317285507705172605_i128;
_3 = _6 << _10;
_17.0 = [_25,_25,_25,_25,_25,_25,_25];
_25 = (-70407389319695285129927406236837870996_i128) + 147802038951585508760845984219696164419_i128;
_22 = [_25,_25];
_17.0 = [_25,_25,_25,_25,_25,_25,_25];
_25 = '\u{e7899}' as i128;
_8 = [6090045237237280507_u64,2765280905130323868_u64,11905617450988729582_u64,8683283434551267715_u64,2965331810112874252_u64,5278116379546967876_u64];
_23 = !_21;
_26 = _12;
_24 = [_15,_15,_15,_15,_15,_15,_15];
_6 = _3 | _3;
_13 = _12;
_7 = [_6,_3];
Goto(bb9)
}
bb9 = {
_9 = [_3,_6];
_1 = false | false;
_8 = [5961454025959215839_u64,12463900000760783008_u64,8191739214684269803_u64,3413987758005150795_u64,961836990028049143_u64,286815714049905294_u64];
_18 = ['\u{db277}','\u{907c8}','\u{1de01}'];
_2 = _3;
RET = [338418455_i32,(-1478885613_i32),666927967_i32,716928037_i32,388338004_i32,(-1348453185_i32),(-1712586227_i32),762474890_i32];
_1 = !false;
_26 = _13;
_32 = [_25,_25,_25,_25,_25,_25,_25];
RET = [(-1420873545_i32),(-856931366_i32),2094866741_i32,340377903_i32,37392792_i32,(-1881774321_i32),(-1365180657_i32),1864612318_i32];
_11 = !(-19546_i16);
_33 = core::ptr::addr_of_mut!(_29);
Goto(bb10)
}
bb10 = {
_34 = 3835921959_u32;
_36 = [_10,_10,_10];
_17 = (_32,);
_16 = _36;
RET = [(-2053053766_i32),(-633711821_i32),1743895044_i32,(-712841936_i32),(-302052762_i32),(-1946929001_i32),(-284035339_i32),(-2033011829_i32)];
_12 = [_10,_10,_10];
_13 = [_10,_10,_10];
_7 = _5;
_16 = _12;
_30 = 52_u8;
match _21 {
0 => bb8,
232995784736606283099411391088997421951 => bb12,
_ => bb11
}
}
bb11 = {
_21 = !227937396425399123030539794667494242002_u128;
_3 = -_6;
Goto(bb7)
}
bb12 = {
_32 = [_25,_25,_25,_25,_25,_25,_25];
_29 = _30 as f32;
_10 = !70_i8;
_30 = 47106_u16 as u8;
_23 = _21 ^ _21;
RET = [634076889_i32,2116949697_i32,(-1963983134_i32),577918337_i32,(-1286592762_i32),(-2090407683_i32),(-1584901761_i32),761986077_i32];
_13 = [_10,_10,_10];
_11 = !31272_i16;
_37 = Adt44 { fld0: _11,fld1: _32 };
_19 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb13)
}
bb13 = {
match _21 {
0 => bb7,
1 => bb8,
232995784736606283099411391088997421951 => bb14,
_ => bb12
}
}
bb14 = {
_19 = [_15,_15,_15,_15,_15,_15,_15];
_15 = !10600013954852591761_usize;
_13 = [_10,_10,_10];
_29 = _15 as f32;
_32 = _37.fld1;
_40.fld3 = [_10,_10,_10];
_15 = 3554628119624838112_usize;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(15_usize, 36_usize, Move(_36), 17_usize, Move(_17), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(15_usize, 5_usize, Move(_5), 15_usize, Move(_15), 16_usize, Move(_16), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(15_usize, 22_usize, Move(_22), 10_usize, Move(_10), 32_usize, Move(_32), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(15_usize, 12_usize, Move(_12), 21_usize, Move(_21), 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [isize; 2],mut _2: bool,mut _3: [isize; 2]) -> i16 {
mir! {
type RET = i16;
let _4: Adt56;
let _5: u32;
let _6: f64;
let _7: bool;
let _8: [i128; 7];
let _9: isize;
let _10: [i128; 7];
let _11: *mut f32;
let _12: ([i128; 2], [u16; 7], u64);
let _13: [i8; 4];
let _14: f64;
let _15: Adt59;
let _16: f64;
let _17: Adt46;
let _18: f64;
let _19: [i8; 4];
let _20: usize;
let _21: [isize; 2];
let _22: char;
let _23: [usize; 7];
let _24: Adt60;
let _25: Adt48;
let _26: i128;
let _27: [i128; 2];
let _28: [u64; 6];
let _29: [usize; 7];
let _30: Adt57;
let _31: [isize; 2];
let _32: Adt56;
let _33: isize;
let _34: *const i8;
let _35: isize;
let _36: (bool, [i32; 8], bool, i32, ((*mut u8, &'static char, bool), char, [i8; 4]), i64, usize);
let _37: char;
let _38: *const i8;
let _39: [char; 3];
let _40: [isize; 2];
let _41: bool;
let _42: [i128; 7];
let _43: [usize; 7];
let _44: isize;
let _45: char;
let _46: [isize; 2];
let _47: ([i128; 7],);
let _48: ([i128; 7],);
let _49: ();
let _50: ();
{
_1 = _3;
_3 = _1;
RET = 1660184698_i32 as i16;
RET = 2763_i16 ^ 11414_i16;
RET = (-8520_i16) + 13228_i16;
RET = (-9223372036854775808_isize) as i16;
Goto(bb1)
}
bb1 = {
RET = '\u{f1b76}' as i16;
RET = (-23170338743367938178584509513106955389_i128) as i16;
RET = 12828_i16 + 2422_i16;
_5 = 149997529_u32 ^ 2578495303_u32;
RET = -25375_i16;
_1 = _3;
RET = _2 as i16;
RET = (-20228_i16) - (-9603_i16);
RET = 2459_i16 >> _5;
_5 = 3841075847_u32;
Goto(bb2)
}
bb2 = {
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !false;
RET = 2075823961244438848_usize as i16;
_3 = _1;
RET = 127_i8 as i16;
_2 = !false;
RET = (-29560112890685543770413241992525462336_i128) as i16;
_1 = [(-91_isize),(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
RET = (-15497_i16);
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = (-30772_i16) + 1469_i16;
_6 = 9219461004913751594_u64 as f64;
_8 = [(-136602363726813340603794653611171527165_i128),103432586125062772327710801748651790822_i128,32613971977001524715727624860229058426_i128,(-117425267078913878109911329770519343963_i128),(-54226511303597484228441005291793096034_i128),11980036434620833682199797988226434406_i128,(-156551448738210744381586406903524158333_i128)];
_7 = !_2;
_1 = _3;
_1 = _3;
_6 = 104982626882248719008012728635517232012_u128 as f64;
_5 = _6 as u32;
Goto(bb3)
}
bb3 = {
_1 = _3;
_3 = [9223372036854775807_isize,(-74_isize)];
_7 = _2;
_1 = _3;
_8 = [126139791870204388397885055831601812620_i128,(-99318477158879044605251679675102441001_i128),58382673725582585228894702925952411373_i128,(-71880121413155706547424696873239741074_i128),62375000280035778418206239496539307888_i128,162828275934552623655030681511464286869_i128,54768091799117203489389641917292559163_i128];
_2 = _7 & _7;
RET = 17122032319031869479_u64 as i16;
RET = 2621_i16;
_6 = 11729969256449021642_u64 as f64;
RET = -(-28938_i16);
_10 = _8;
_3 = [9223372036854775807_isize,(-19_isize)];
RET = 21878_i16 >> _5;
_2 = !_7;
_6 = 31525577829283268737376232410422650050_u128 as f64;
_9 = 13_isize;
_13 = [73_i8,29_i8,61_i8,(-5_i8)];
RET = !29721_i16;
Call(_14 = fn17(_1, _10, _10, _8, _13, _1, _6, _10, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = (-18054_i16);
_3 = [_9,_9];
_15.fld3.fld1 = core::ptr::addr_of!(_6);
_15.fld3.fld0 = [16311262340186874802_u64,4442774906397036477_u64,10536780905943610259_u64,8771175986924267911_u64,3396225140091427166_u64,17718917012199535802_u64];
_13 = [(-3_i8),18_i8,78_i8,(-94_i8)];
_8 = [(-133176985133368267204517014928203938090_i128),49045246042008675609528431545877635904_i128,101751876826298267192174842206687069575_i128,118122338260140595718863107736615040197_i128,(-12301360896501543392097744669375360633_i128),112390190369571396861542034417479109682_i128,12912441058957092391124372023740717977_i128];
_15.fld3.fld1 = core::ptr::addr_of!(_18);
RET = _5 as i16;
_15.fld3.fld2 = [989072224777049859_usize,4549538739588750479_usize,9663610842461206826_usize,5_usize,5_usize,2152563618521442852_usize,3_usize];
_12.2 = _7 as u64;
_12.0 = [(-82792797955443863982470004611877508890_i128),88194556689058489622394028712902235969_i128];
_12.1 = [27858_u16,19125_u16,5907_u16,63515_u16,32003_u16,4142_u16,32964_u16];
_15.fld3.fld3 = (_8,);
_1 = [_9,_9];
_18 = _6;
_14 = -_18;
_7 = _2;
_3 = _1;
RET = !(-22960_i16);
match _9 {
0 => bb5,
1 => bb6,
2 => bb7,
13 => bb9,
_ => bb8
}
}
bb5 = {
_1 = _3;
_3 = [9223372036854775807_isize,(-74_isize)];
_7 = _2;
_1 = _3;
_8 = [126139791870204388397885055831601812620_i128,(-99318477158879044605251679675102441001_i128),58382673725582585228894702925952411373_i128,(-71880121413155706547424696873239741074_i128),62375000280035778418206239496539307888_i128,162828275934552623655030681511464286869_i128,54768091799117203489389641917292559163_i128];
_2 = _7 & _7;
RET = 17122032319031869479_u64 as i16;
RET = 2621_i16;
_6 = 11729969256449021642_u64 as f64;
RET = -(-28938_i16);
_10 = _8;
_3 = [9223372036854775807_isize,(-19_isize)];
RET = 21878_i16 >> _5;
_2 = !_7;
_6 = 31525577829283268737376232410422650050_u128 as f64;
_9 = 13_isize;
_13 = [73_i8,29_i8,61_i8,(-5_i8)];
RET = !29721_i16;
Call(_14 = fn17(_1, _10, _10, _8, _13, _1, _6, _10, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !false;
RET = 2075823961244438848_usize as i16;
_3 = _1;
RET = 127_i8 as i16;
_2 = !false;
RET = (-29560112890685543770413241992525462336_i128) as i16;
_1 = [(-91_isize),(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
RET = (-15497_i16);
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = (-30772_i16) + 1469_i16;
_6 = 9219461004913751594_u64 as f64;
_8 = [(-136602363726813340603794653611171527165_i128),103432586125062772327710801748651790822_i128,32613971977001524715727624860229058426_i128,(-117425267078913878109911329770519343963_i128),(-54226511303597484228441005291793096034_i128),11980036434620833682199797988226434406_i128,(-156551448738210744381586406903524158333_i128)];
_7 = !_2;
_1 = _3;
_1 = _3;
_6 = 104982626882248719008012728635517232012_u128 as f64;
_5 = _6 as u32;
Goto(bb3)
}
bb7 = {
RET = '\u{f1b76}' as i16;
RET = (-23170338743367938178584509513106955389_i128) as i16;
RET = 12828_i16 + 2422_i16;
_5 = 149997529_u32 ^ 2578495303_u32;
RET = -25375_i16;
_1 = _3;
RET = _2 as i16;
RET = (-20228_i16) - (-9603_i16);
RET = 2459_i16 >> _5;
_5 = 3841075847_u32;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_15.fld3.fld3.0 = [87163086321687568005731338240590659738_i128,(-88180132647652771063053050676526353595_i128),(-128289610567595155537658427454542587834_i128),7881562873909325696761832780756741962_i128,104365682038099313380769713724710846453_i128,(-59407548582231739580036207986389925669_i128),(-30266334991611089384399323826831041778_i128)];
_2 = !_7;
_15.fld4 = _12.2 >> _12.2;
_15.fld3.fld3 = (_8,);
_15.fld3.fld4 = [_9,_9];
_15.fld0 = [_9,_9];
RET = 10407_i16 ^ 19380_i16;
_15.fld3.fld0 = [_15.fld4,_12.2,_15.fld4,_12.2,_15.fld4,_15.fld4];
_15.fld3.fld5 = core::ptr::addr_of!(RET);
_14 = (-121390919433314733338685984072518005356_i128) as f64;
_10 = _15.fld3.fld3.0;
_15.fld2 = [23875_u16,8994_u16,51856_u16,2451_u16,50680_u16,19567_u16,25289_u16];
RET = -15412_i16;
match _9 {
13 => bb11,
_ => bb10
}
}
bb10 = {
_1 = _3;
_3 = [9223372036854775807_isize,(-74_isize)];
_7 = _2;
_1 = _3;
_8 = [126139791870204388397885055831601812620_i128,(-99318477158879044605251679675102441001_i128),58382673725582585228894702925952411373_i128,(-71880121413155706547424696873239741074_i128),62375000280035778418206239496539307888_i128,162828275934552623655030681511464286869_i128,54768091799117203489389641917292559163_i128];
_2 = _7 & _7;
RET = 17122032319031869479_u64 as i16;
RET = 2621_i16;
_6 = 11729969256449021642_u64 as f64;
RET = -(-28938_i16);
_10 = _8;
_3 = [9223372036854775807_isize,(-19_isize)];
RET = 21878_i16 >> _5;
_2 = !_7;
_6 = 31525577829283268737376232410422650050_u128 as f64;
_9 = 13_isize;
_13 = [73_i8,29_i8,61_i8,(-5_i8)];
RET = !29721_i16;
Call(_14 = fn17(_1, _10, _10, _8, _13, _1, _6, _10, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_14 = _6 * _18;
_19 = _13;
_15.fld3.fld0 = [_15.fld4,_15.fld4,_15.fld4,_15.fld4,_15.fld4,_15.fld4];
_15.fld3.fld3 = (_8,);
_9 = 9223372036854775807_isize * 9223372036854775807_isize;
_23 = _15.fld3.fld2;
_15.fld4 = _12.2 * _12.2;
_15.fld3.fld4 = [_9,_9];
_12.0 = [(-130517675461657466825453646470190418590_i128),(-140244197304239691265986341040824838744_i128)];
_3 = [_9,_9];
_22 = '\u{8274e}';
_15.fld2 = [45357_u16,33407_u16,8229_u16,33594_u16,8147_u16,10365_u16,42795_u16];
_16 = -_14;
_21 = [_9,_9];
_8 = [(-9531514813862778056320615123207260183_i128),145847247888011473788875650208060076407_i128,36685620382544270408417957871292107627_i128,49283460472052727998857459546721249140_i128,(-130424659018796277892425075949363433753_i128),36013723728406957804932215659308261936_i128,167896572198918141695751209784485211789_i128];
_12.1 = [60530_u16,9290_u16,15682_u16,33140_u16,7237_u16,8615_u16,40217_u16];
_20 = !1_usize;
_28 = [_15.fld4,_15.fld4,_15.fld4,_15.fld4,_12.2,_15.fld4];
_23 = [_20,_20,_20,_20,_20,_20,_20];
_28 = _15.fld3.fld0;
_15.fld3.fld0 = [_12.2,_15.fld4,_15.fld4,_15.fld4,_15.fld4,_15.fld4];
_15.fld3.fld2 = [_20,_20,_20,_20,_20,_20,_20];
_7 = _2;
Call(_13 = fn18(_15.fld2, _15.fld3.fld3.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20 = 432988918031789526_usize;
_8 = [117382938816643396349742046287437474177_i128,19375790358293369404844478729131264797_i128,(-140584271640451757495028507611546266704_i128),153868873087096696069414675197121143705_i128,71855507678733251686669290630306991764_i128,169981452612965139304344717796829912245_i128,(-124404268681402209677276557959440697758_i128)];
_15.fld3.fld4 = [_9,_9];
_15.fld3.fld5 = core::ptr::addr_of!(RET);
_9 = (-95_isize) << RET;
_4 = Adt56::Variant1 { fld0: _12.1,fld1: _23 };
_9 = !107_isize;
SetDiscriminant(_4, 0);
_23 = _15.fld3.fld2;
place!(Field::<isize>(Variant(_4, 0), 2)) = _9;
_28 = [_15.fld4,_15.fld4,_12.2,_15.fld4,_12.2,_15.fld4];
RET = _9 as i16;
_16 = (-3_i8) as f64;
_15.fld3.fld4 = [Field::<isize>(Variant(_4, 0), 2),_9];
place!(Field::<[isize; 2]>(Variant(_4, 0), 4)) = [Field::<isize>(Variant(_4, 0), 2),_9];
_3 = [Field::<isize>(Variant(_4, 0), 2),Field::<isize>(Variant(_4, 0), 2)];
Goto(bb13)
}
bb13 = {
_15.fld3.fld3 = (_10,);
_15.fld4 = _5 as u64;
_29 = [_20,_20,_20,_20,_20,_20,_20];
_15.fld2 = [47523_u16,63512_u16,9414_u16,59578_u16,35979_u16,21563_u16,52372_u16];
_36.1 = [357579425_i32,900978163_i32,1672836624_i32,(-240706792_i32),(-938173249_i32),1324678123_i32,1954459020_i32,(-80628410_i32)];
_27 = [(-10418426228729491255988304095293108864_i128),(-36903645023624519349489765698209241062_i128)];
_18 = RET as f64;
_22 = '\u{267d}';
_35 = !Field::<isize>(Variant(_4, 0), 2);
_12.1 = _15.fld2;
_15.fld3.fld2 = [_20,_20,_20,_20,_20,_20,_20];
_23 = [_20,_20,_20,_20,_20,_20,_20];
_36.0 = !_7;
_36.4.1 = _22;
_12.1 = [37846_u16,42874_u16,48330_u16,42822_u16,29037_u16,6775_u16,63501_u16];
_36.3 = !(-1131801076_i32);
place!(Field::<u64>(Variant(_4, 0), 5)) = 53532419651372312964027496164265976666_i128 as u64;
_33 = -_9;
RET = !(-15939_i16);
_37 = _22;
_36.6 = _20;
match _20 {
0 => bb1,
1 => bb11,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
432988918031789526 => bb20,
_ => bb19
}
}
bb14 = {
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !false;
RET = 2075823961244438848_usize as i16;
_3 = _1;
RET = 127_i8 as i16;
_2 = !false;
RET = (-29560112890685543770413241992525462336_i128) as i16;
_1 = [(-91_isize),(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
RET = (-15497_i16);
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = (-30772_i16) + 1469_i16;
_6 = 9219461004913751594_u64 as f64;
_8 = [(-136602363726813340603794653611171527165_i128),103432586125062772327710801748651790822_i128,32613971977001524715727624860229058426_i128,(-117425267078913878109911329770519343963_i128),(-54226511303597484228441005291793096034_i128),11980036434620833682199797988226434406_i128,(-156551448738210744381586406903524158333_i128)];
_7 = !_2;
_1 = _3;
_1 = _3;
_6 = 104982626882248719008012728635517232012_u128 as f64;
_5 = _6 as u32;
Goto(bb3)
}
bb15 = {
RET = (-18054_i16);
_3 = [_9,_9];
_15.fld3.fld1 = core::ptr::addr_of!(_6);
_15.fld3.fld0 = [16311262340186874802_u64,4442774906397036477_u64,10536780905943610259_u64,8771175986924267911_u64,3396225140091427166_u64,17718917012199535802_u64];
_13 = [(-3_i8),18_i8,78_i8,(-94_i8)];
_8 = [(-133176985133368267204517014928203938090_i128),49045246042008675609528431545877635904_i128,101751876826298267192174842206687069575_i128,118122338260140595718863107736615040197_i128,(-12301360896501543392097744669375360633_i128),112390190369571396861542034417479109682_i128,12912441058957092391124372023740717977_i128];
_15.fld3.fld1 = core::ptr::addr_of!(_18);
RET = _5 as i16;
_15.fld3.fld2 = [989072224777049859_usize,4549538739588750479_usize,9663610842461206826_usize,5_usize,5_usize,2152563618521442852_usize,3_usize];
_12.2 = _7 as u64;
_12.0 = [(-82792797955443863982470004611877508890_i128),88194556689058489622394028712902235969_i128];
_12.1 = [27858_u16,19125_u16,5907_u16,63515_u16,32003_u16,4142_u16,32964_u16];
_15.fld3.fld3 = (_8,);
_1 = [_9,_9];
_18 = _6;
_14 = -_18;
_7 = _2;
_3 = _1;
RET = !(-22960_i16);
match _9 {
0 => bb5,
1 => bb6,
2 => bb7,
13 => bb9,
_ => bb8
}
}
bb16 = {
RET = '\u{f1b76}' as i16;
RET = (-23170338743367938178584509513106955389_i128) as i16;
RET = 12828_i16 + 2422_i16;
_5 = 149997529_u32 ^ 2578495303_u32;
RET = -25375_i16;
_1 = _3;
RET = _2 as i16;
RET = (-20228_i16) - (-9603_i16);
RET = 2459_i16 >> _5;
_5 = 3841075847_u32;
Goto(bb2)
}
bb17 = {
_1 = _3;
_3 = [9223372036854775807_isize,(-74_isize)];
_7 = _2;
_1 = _3;
_8 = [126139791870204388397885055831601812620_i128,(-99318477158879044605251679675102441001_i128),58382673725582585228894702925952411373_i128,(-71880121413155706547424696873239741074_i128),62375000280035778418206239496539307888_i128,162828275934552623655030681511464286869_i128,54768091799117203489389641917292559163_i128];
_2 = _7 & _7;
RET = 17122032319031869479_u64 as i16;
RET = 2621_i16;
_6 = 11729969256449021642_u64 as f64;
RET = -(-28938_i16);
_10 = _8;
_3 = [9223372036854775807_isize,(-19_isize)];
RET = 21878_i16 >> _5;
_2 = !_7;
_6 = 31525577829283268737376232410422650050_u128 as f64;
_9 = 13_isize;
_13 = [73_i8,29_i8,61_i8,(-5_i8)];
RET = !29721_i16;
Call(_14 = fn17(_1, _10, _10, _8, _13, _1, _6, _10, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
RET = '\u{f1b76}' as i16;
RET = (-23170338743367938178584509513106955389_i128) as i16;
RET = 12828_i16 + 2422_i16;
_5 = 149997529_u32 ^ 2578495303_u32;
RET = -25375_i16;
_1 = _3;
RET = _2 as i16;
RET = (-20228_i16) - (-9603_i16);
RET = 2459_i16 >> _5;
_5 = 3841075847_u32;
Goto(bb2)
}
bb20 = {
place!(Field::<i64>(Variant(_4, 0), 6)) = 45524_u16 as i64;
_15.fld4 = Field::<u64>(Variant(_4, 0), 5);
_1 = [Field::<isize>(Variant(_4, 0), 2),_35];
_15.fld5 = core::ptr::addr_of!(_36.5);
_15.fld3.fld1 = core::ptr::addr_of!(_18);
_15.fld3.fld0 = _28;
_7 = _36.6 > _36.6;
_31 = _3;
_27 = [(-126356327692222019306622615288794480806_i128),4860074657248474196315205907432004729_i128];
_37 = _22;
_15.fld2 = _12.1;
_3 = [Field::<isize>(Variant(_4, 0), 2),_9];
_36.5 = Field::<i64>(Variant(_4, 0), 6) * Field::<i64>(Variant(_4, 0), 6);
_35 = (-84_i8) as isize;
_43 = [_36.6,_36.6,_20,_20,_20,_36.6,_20];
_42 = [13491397239732907958722056863873579836_i128,146849376892973945724492898502092789285_i128,99217838982035883068211578611050888109_i128,(-103411735908704626592410257856089833189_i128),(-135186607885560198479863243549281420389_i128),124657126762377749841443283379615140373_i128,(-132244040598392817677609195276195624855_i128)];
_15.fld3.fld4 = Field::<[isize; 2]>(Variant(_4, 0), 4);
_32 = Adt56::Variant1 { fld0: _15.fld2,fld1: _29 };
place!(Field::<[u16; 7]>(Variant(_32, 1), 0)) = _12.1;
_15.fld5 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_4, 0), 6)));
Goto(bb21)
}
bb21 = {
Call(_49 = dump_var(16_usize, 27_usize, Move(_27), 31_usize, Move(_31), 7_usize, Move(_7), 35_usize, Move(_35)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_49 = dump_var(16_usize, 19_usize, Move(_19), 43_usize, Move(_43), 5_usize, Move(_5), 20_usize, Move(_20)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_49 = dump_var(16_usize, 13_usize, Move(_13), 3_usize, Move(_3), 8_usize, Move(_8), 23_usize, Move(_23)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: [isize; 2],mut _2: [i128; 7],mut _3: [i128; 7],mut _4: [i128; 7],mut _5: [i8; 4],mut _6: [isize; 2],mut _7: f64,mut _8: [i128; 7],mut _9: [i128; 7]) -> f64 {
mir! {
type RET = f64;
let _10: [i32; 8];
let _11: [i8; 4];
let _12: Adt60;
let _13: *mut char;
let _14: [usize; 7];
let _15: i64;
let _16: *mut char;
let _17: i64;
let _18: u16;
let _19: Adt55;
let _20: isize;
let _21: u16;
let _22: f32;
let _23: (*mut u8, &'static char, bool);
let _24: Adt44;
let _25: f64;
let _26: *const isize;
let _27: isize;
let _28: i16;
let _29: bool;
let _30: *const i16;
let _31: f32;
let _32: f64;
let _33: isize;
let _34: i128;
let _35: [i8; 4];
let _36: ([i128; 7],);
let _37: [i8; 4];
let _38: i16;
let _39: isize;
let _40: (*mut u8, &'static char, bool);
let _41: [u16; 7];
let _42: Adt57;
let _43: usize;
let _44: u64;
let _45: [isize; 2];
let _46: f32;
let _47: Adt59;
let _48: u8;
let _49: f32;
let _50: ([i128; 2], [u16; 7], u64);
let _51: [i8; 4];
let _52: ();
let _53: ();
{
RET = _7 + _7;
_4 = [114232809042183142417308382868272399495_i128,15712967453101301300136686162916716106_i128,(-91876841340111583244496374203712442000_i128),(-24902631300777523736160761729067844245_i128),60871614394044892080157132965375798183_i128,111588550111327333535708404747759613041_i128,(-38941008864157794876445678884460050264_i128)];
_7 = RET - RET;
RET = -_7;
RET = 269946678988354086456130802889990058674_u128 as f64;
_10 = [1894624724_i32,1632404125_i32,1443775366_i32,(-35713152_i32),1580322545_i32,664032200_i32,1608096496_i32,2093932353_i32];
_10 = [(-1213442671_i32),1126875999_i32,520238616_i32,1666631291_i32,(-243830619_i32),(-887639869_i32),934066758_i32,(-1572427365_i32)];
_11 = [58_i8,(-18_i8),(-85_i8),(-2_i8)];
_9 = [(-95313219293881870073337251667540068824_i128),2839795469728831984839885619011059668_i128,36978154940218438820124661210023789989_i128,(-106497974750316585296390702793634347654_i128),72381411852188056716837278343870076423_i128,56578710964710357363304007779725726239_i128,33025595875128597760594235242132708258_i128];
_6 = [(-9223372036854775808_isize),9223372036854775807_isize];
_6 = _1;
_2 = [10606653710712122819424060988211586046_i128,85785330502291587536381850032272615170_i128,2790986047456112639775587288125454328_i128,(-35751030042728455162144868439679230950_i128),64106631543567380712645798845217294914_i128,(-168798428961754658560202565301509348241_i128),17919719719999707770560320846758877266_i128];
_10 = [455928732_i32,1514369701_i32,2066326452_i32,1012970529_i32,1869196143_i32,(-548394170_i32),(-67214011_i32),(-1365183823_i32)];
_3 = _8;
_2 = _9;
_2 = _8;
_8 = [(-69473182889701621479646918825817939213_i128),31277536209555124277460389689640994086_i128,(-165706829869852931988911778992168724465_i128),107746282443177492787721322457271922708_i128,5819838091745665795544048011739981630_i128,70388332419992132116915227391445247506_i128,48985023103989823879353927032409076850_i128];
_8 = [(-136259306190647193721492241486396762601_i128),(-83254192326981421626308285912142539476_i128),(-39912564369223770619170149472405169517_i128),(-90230888330534967471887664992911402220_i128),120351038111505522534841725757979838876_i128,(-13559573621950397748477767090086474246_i128),49529133565867203475848108089066540434_i128];
RET = _7;
RET = _7;
_7 = RET + RET;
RET = 61063_u16 as f64;
_2 = [(-110853228450905304523785642953625621819_i128),(-20898188047157349104574784330899371065_i128),(-142402646420138340838747331154102825705_i128),(-162628156165820513645580511840607505186_i128),66157700467429146424979218129611949590_i128,(-142960934635439416745876928564790348188_i128),85889766104955424849629159060765273843_i128];
_8 = _9;
RET = -_7;
_14 = [2_usize,9559490776382349765_usize,5850066776756343185_usize,15798137362204814391_usize,4_usize,16224658304473284794_usize,1041377040546819899_usize];
Goto(bb1)
}
bb1 = {
RET = -_7;
_9 = [(-8319008582007598172367916643116461819_i128),(-89020909431633731487020465271871949496_i128),(-44661604309056714907623967295542090191_i128),75850363394416919744945865838647981296_i128,62971612652340398990122948871401421795_i128,(-102648513452890460216924558690274189059_i128),99234307644671373031963322890245065661_i128];
_5 = [79_i8,(-103_i8),36_i8,113_i8];
_1 = [(-9223372036854775808_isize),24_isize];
Goto(bb2)
}
bb2 = {
RET = -_7;
RET = _7 - _7;
_11 = _5;
_9 = [29107658266370663204424038808460047020_i128,(-140280047034209041745381897121434592879_i128),(-73027408193943507282732530398689698364_i128),(-122331884622660809586692088281936302042_i128),(-55842596647411361916683207443826210197_i128),50091331725321921724366438649660140350_i128,(-123646373389108752817605072171992924403_i128)];
_10 = [(-1419424592_i32),1973756861_i32,(-2143713256_i32),1496621138_i32,548930755_i32,1046241486_i32,(-770387597_i32),(-892555718_i32)];
RET = 1650750464_i32 as f64;
_15 = (-2185909066937947119_i64);
Call(_11 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = [(-98625593893952566753417640260515003636_i128),(-88683557423006955901654592993406532813_i128),95605292923823146051857707532684663385_i128,107318455970836955633300675326220203918_i128,87821946745878348855698173560967255034_i128,(-169871154296518009675239448047586088152_i128),(-9809921937287024432081085793636857884_i128)];
Goto(bb4)
}
bb4 = {
_8 = _2;
_15 = (-4540706270709518455_i64) - 6722864184532099170_i64;
_9 = [1326867958315694688543765443441927977_i128,1932272021892773879201164427903418299_i128,37637875735820129149586482610557133669_i128,79727207894720012896592269587611653365_i128,144431576638941173800347466211234441769_i128,(-58740077982981399293633158909143614979_i128),155600081162647512740825661476404781908_i128];
_6 = _1;
_8 = [(-112201069714159186805647873868990617309_i128),167781743927641517073430202143668678775_i128,(-1062676372096213356006797547079694148_i128),(-163813874707581724842433957007042662751_i128),143737498566337060659321226498956979486_i128,(-72076213808574653054339581975303398181_i128),(-152354006012340828354784107467949700525_i128)];
_11 = [51_i8,112_i8,26_i8,22_i8];
RET = -_7;
_2 = [128128382461379113047888317815224713475_i128,111502937059130292560456133111879013748_i128,(-157292508138827471473313829693006301872_i128),(-156886007869314688095917159440638448067_i128),57228297668221891920458461057702268592_i128,(-148407785490824316170342339030420886311_i128),133112594686571828849190880769681909393_i128];
_14 = [13402882811155918476_usize,485559687217703012_usize,8374985510142386843_usize,7640840936406028007_usize,4_usize,7793780361421751848_usize,7053667951915185556_usize];
_5 = _11;
RET = 28083_i16 as f64;
_1 = _6;
_5 = [26_i8,22_i8,(-104_i8),106_i8];
_6 = _1;
_11 = _5;
_9 = _4;
Goto(bb5)
}
bb5 = {
_15 = 89995661372870912594557662902321630161_u128 as i64;
_2 = [(-92589261835789837556563830792893429340_i128),159615171380364812568623810233980161528_i128,(-67506491050692746045901988168136175776_i128),(-47225191811520008916629051919685301543_i128),(-75124439252232850148529237712597094496_i128),(-122049688050142915856138097203087768025_i128),64329990956394435749693464516711805065_i128];
_5 = [(-117_i8),75_i8,(-117_i8),11_i8];
RET = 485451597_i32 as f64;
_2 = _3;
_15 = !4398345815489161596_i64;
_14 = [2_usize,0_usize,16716946379781873276_usize,15617331649363198898_usize,3541227082886529359_usize,6159322730937034208_usize,13495611534768118569_usize];
RET = 9819688209666679837_u64 as f64;
_9 = [(-7312114724213438671943039622948701531_i128),23309656109475303174552086450250316110_i128,(-159470228317046085461738343750028252932_i128),(-32242034130823541269802040072588118874_i128),168192454503497409983297325818074059530_i128,(-61008453272715519740157690673161221404_i128),(-44581264069065748488135037409200341539_i128)];
_4 = _3;
_4 = _8;
_15 = _7 as i64;
_4 = [(-50207583571814150067167052850321879777_i128),97577260625808576191934321651049613570_i128,(-145694149908821414607929513021309266107_i128),(-38956466838789825843318794394946905009_i128),(-32560442719800510302983804826171918982_i128),151336487762787416323623420356514879791_i128,(-5631740075770029439942987411640265713_i128)];
_17 = !_15;
Goto(bb6)
}
bb6 = {
_11 = _5;
_20 = -(-109_isize);
_9 = _3;
_21 = _20 as u16;
_4 = [94439365067521072154606582113918178486_i128,(-12611438857961663785673522994369676928_i128),139287134185119434139041346686841532757_i128,(-24648188578482579840908515359434874359_i128),66732975902032950189294884027146446373_i128,(-5131044672031988371195446293871382603_i128),126286804555153811911978171879597881717_i128];
_3 = [(-38642148755863390446073386108132962642_i128),70280916503607942513505402939691181842_i128,(-83562132705236283749581642992627646533_i128),(-152402217774363881048562230655774281555_i128),(-168856478351089580478078131357001359476_i128),(-106770533558105654591503740185605052535_i128),(-54999684697345405637744153290940842119_i128)];
Goto(bb7)
}
bb7 = {
_23.2 = true;
RET = _20 as f64;
_21 = 25233_u16 * 10350_u16;
_5 = [(-11_i8),21_i8,108_i8,(-18_i8)];
_7 = RET + RET;
_10 = [1266631730_i32,(-838928085_i32),(-217068616_i32),(-1057737909_i32),1514327670_i32,(-294960464_i32),(-1510857066_i32),1530822351_i32];
_6 = [_20,_20];
_20 = 125_isize ^ 54_isize;
_11 = [(-40_i8),72_i8,116_i8,42_i8];
_24 = Adt44 { fld0: (-14149_i16),fld1: _2 };
_7 = -RET;
_1 = [_20,_20];
_5 = [(-32_i8),(-76_i8),50_i8,(-43_i8)];
_3 = [100963386230897377932902888379821726989_i128,(-3957848699705392074576228009795624700_i128),(-87829741078782101494192461034747945248_i128),77170927323945124560489650965903361016_i128,(-101361067543659521881054868067660489312_i128),(-125569031274980649538690695220547752675_i128),(-11231337458195699744397295233068512693_i128)];
RET = -_7;
Call(_22 = core::intrinsics::transmute(_5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_24 = Adt44 { fld0: 32651_i16,fld1: _9 };
_22 = 184785025861515941981218203948420046782_u128 as f32;
_24.fld1 = [9564466543279107343006221276186068990_i128,(-159705401405693153565983118700125509036_i128),86801607229284588523078977213357213010_i128,47225094312313016445361880219547930168_i128,151195314454724599969853969086183204554_i128,(-119185054178616599896860241848310887302_i128),119755618570105942014065943581440047985_i128];
_24.fld1 = [122408736817047220748400829175167853639_i128,(-109647819158744536032647457639314060875_i128),(-137294847946057628530752392979582421854_i128),20662789181333292235643117130946789671_i128,44638928614011286426085427567540773326_i128,22242446786095200392249259763921125239_i128,(-69774371583106882950518602989664892858_i128)];
_1 = _6;
_18 = _24.fld0 as u16;
_6 = [_20,_20];
RET = _7;
_25 = RET - RET;
RET = _7 + _25;
_24 = Adt44 { fld0: 25177_i16,fld1: _4 };
_26 = core::ptr::addr_of!(_20);
_25 = -_7;
(*_26) = 9223372036854775807_isize >> _21;
_8 = [159446174399573728786643018767263589279_i128,5552051957250502194588642974495325655_i128,29556458089433171203645323999171418895_i128,(-26884116177175234251825004729779018738_i128),(-44348305445434051592647023847343961485_i128),105500354244760478000965636873890096519_i128,(-59425111123765062195250558784412291533_i128)];
_21 = 541254090_i32 as u16;
_10 = [(-1084920267_i32),812735220_i32,149517937_i32,(-556586144_i32),1849814577_i32,(-194218863_i32),(-1821135647_i32),1300325004_i32];
_26 = core::ptr::addr_of!((*_26));
_11 = _5;
_10 = [1397586120_i32,1867685728_i32,(-878985559_i32),583071262_i32,280962204_i32,1642704175_i32,755937032_i32,(-1509543621_i32)];
_11 = [53_i8,19_i8,109_i8,59_i8];
RET = -_7;
_2 = [(-150304767757946774001009373464359459768_i128),(-95073314179595109706736073345967660201_i128),(-148965953710221961493372858065490879422_i128),5284876694013579235491453671409568519_i128,27496940068801129790825109619748267607_i128,80407855473653707106663311309633750584_i128,120813289755446707882700153470882633949_i128];
Goto(bb9)
}
bb9 = {
_6 = [(*_26),(*_26)];
_9 = [8650971596325120109237339717253696563_i128,(-70464412058088030066708412748876973383_i128),(-12769303253598557966995266285275941735_i128),157599165586592592316554165476259522701_i128,(-118568115305367699241351671785865470724_i128),89204676596811922211927675691058096785_i128,(-69377162314692981165878942019692078142_i128)];
_28 = 15238686173713142964_u64 as i16;
_2 = [(-149382121497836166009733856884336485120_i128),(-40330050972123170058289761005539132131_i128),113364069737576064952574829572249085719_i128,(-41909822038503242751832445692929727669_i128),(-64814592182859113281561283208894630977_i128),(-86797329476387389141072647909317091913_i128),36764365557500408925447814683465348067_i128];
_2 = [26851507908118922244177197482755300358_i128,(-17113680539139456769146871814248470289_i128),(-18293942429910197774604137842045532310_i128),59515627194000826882509850920498922874_i128,(-157148624414009821947658255711061259005_i128),4877616825228755890867629640996441928_i128,118231116043151137303324359614363287640_i128];
(*_26) = _25 as isize;
_27 = -_20;
_6 = [_27,(*_26)];
_23.2 = false;
_15 = _22 as i64;
_24.fld1 = [(-75397740197297298146595060096935792538_i128),140884824103048685728179755461756382619_i128,(-118349199081293546410447650761054741649_i128),160206058312344787199665465357964332954_i128,(-106596488381982812278851328956522914305_i128),(-97380505582603225579002209082180507905_i128),(-136153333924265028450756650249700186774_i128)];
_8 = [162253254137610212064952102182819078196_i128,(-39882947520676171092139609080112492309_i128),(-148895131960473002400681909413555644634_i128),(-144582302643001263184026927182462845288_i128),(-107493598736591974746521760137827235358_i128),21617820425190094671410460259763834598_i128,134776731154860748651971827099142876642_i128];
Call(_33 = core::intrinsics::bswap(_20), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_33 = -_27;
_31 = _22 * _22;
_24.fld1 = _9;
_10 = [(-1413373641_i32),(-213987020_i32),(-1527977993_i32),1836574663_i32,(-1075840294_i32),(-1455486607_i32),(-1724782340_i32),1048501479_i32];
_2 = [149299813578179959236994858802502627158_i128,156147639375310758991890060308373256361_i128,81973048384650761098045299820666180400_i128,(-111676248807030145338866923613955252902_i128),11437628333515907420960247774637832768_i128,120289445660680170829866912458103966712_i128,(-47525137096695362103853594500479812752_i128)];
_11 = [78_i8,(-27_i8),(-35_i8),(-42_i8)];
RET = _17 as f64;
_37 = [36_i8,56_i8,90_i8,35_i8];
_22 = -_31;
_27 = 75_i8 as isize;
_6 = _1;
_11 = _37;
_1 = [(*_26),(*_26)];
_24.fld0 = _17 as i16;
_15 = -_17;
_33 = (*_26);
_10 = [(-842250765_i32),(-298851203_i32),1501968424_i32,1068948337_i32,(-2040330077_i32),455886918_i32,(-636464661_i32),2060452964_i32];
_15 = 78_u8 as i64;
RET = _7 + _7;
Call(_20 = core::intrinsics::transmute(_17), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = (-1940847955_i32) as f64;
_35 = [5_i8,(-116_i8),(-56_i8),(-79_i8)];
_8 = _2;
_5 = [102_i8,(-40_i8),(-67_i8),65_i8];
_11 = _37;
(*_26) = _33 - _33;
_7 = RET * RET;
_3 = _4;
_23.2 = _18 != _18;
_18 = _21 ^ _21;
_41 = [_18,_21,_18,_21,_21,_18,_18];
_20 = 2480356366_u32 as isize;
_26 = core::ptr::addr_of!(_39);
_7 = -RET;
_28 = 8956313786218169497_usize as i16;
_4 = [116318775253140153090188496651011914332_i128,(-137847189594961281299073679631850633554_i128),(-114154646492589499053577349720483558908_i128),136351014450719377296265290524160635755_i128,27106063826956954400315697930158555996_i128,(-62489952256517012635059988403108078734_i128),93119478145217078649262058129937655208_i128];
_22 = _31;
_44 = 4283630464179632959_u64 + 7436366216935359457_u64;
_43 = 7_usize;
_20 = _33 + _27;
_24.fld1 = _8;
Call(RET = core::intrinsics::fmaf64(_7, _7, _25), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_26) = _20;
_26 = core::ptr::addr_of!((*_26));
match _10[_43] {
0 => bb5,
1 => bb2,
2060452964 => bb13,
_ => bb6
}
}
bb13 = {
_3 = [48354515713058619318917914531168363536_i128,(-31830116418692631783402193110502546179_i128),(-123513891920042811765058895983412769570_i128),(-88264208921991652647878483551214760117_i128),(-113433532170953738109034570046067808991_i128),(-71093186970788596119197560408853215192_i128),(-78951018191931781088609979470610564252_i128)];
_37 = [92_i8,6_i8,(-1_i8),(-120_i8)];
_9 = [168455632655935416849434010384460986388_i128,(-63331003070692798606674629719808766617_i128),(-130289599701226679177514910828841633623_i128),50329914155370223093908372562176798224_i128,(-53727650048519360088202072213973939318_i128),(-4422157324775730732685687190256459426_i128),(-142853286197001184145160261666533518946_i128)];
RET = _25;
_17 = -_15;
_41 = [_18,_18,_18,_18,_18,_18,_18];
_36 = (_9,);
_20 = 103_u8 as isize;
_47.fld3.fld1 = core::ptr::addr_of!(_47.fld6);
_10[_43] = (-86_i8) as i32;
_49 = -_22;
_47.fld5 = core::ptr::addr_of!(_15);
_20 = (*_26) >> _21;
_10 = [(-1212912879_i32),(-1694350784_i32),1138769347_i32,(-1066407109_i32),1077312744_i32,(-1648560825_i32),1293796036_i32,1631967363_i32];
(*_26) = _31 as isize;
_47.fld0 = [(*_26),_27];
_47.fld3.fld3 = (_36.0,);
_4 = [76541917495641696001814932563562513625_i128,(-114692419359147711799043558988174643278_i128),110445531999004676739102181503049057872_i128,29079184648607637037183281665908813513_i128,(-26638744357829731872255381751653834280_i128),62488812805417880886412756226026524881_i128,(-11881163976207694474268278866198209824_i128)];
_22 = _18 as f32;
_4 = _9;
Goto(bb14)
}
bb14 = {
_23.0 = core::ptr::addr_of_mut!(_48);
_3 = [143355978303020440609672425639139511722_i128,167159207486715683500997950552165644074_i128,66987748660867557591339970802412312517_i128,102000213067977685143687967208833870348_i128,(-73260932294861462616055606482354405849_i128),(-109991528053881078840859924404548968297_i128),67526341211203558549240402960349539514_i128];
_23.0 = core::ptr::addr_of_mut!(_48);
_34 = -60700452552155585370069259163474400617_i128;
_40.2 = _23.2;
_48 = 49_u8 * 229_u8;
_30 = core::ptr::addr_of!(_24.fld0);
_41 = [_18,_18,_18,_18,_18,_18,_18];
_50.0 = [_34,_34];
(*_26) = '\u{10d0ed}' as isize;
_46 = _22 + _49;
(*_26) = !_33;
_10 = [(-51361556_i32),998559023_i32,1687676604_i32,(-458676016_i32),528923788_i32,(-1382934504_i32),(-1540050762_i32),1763312120_i32];
_47.fld6 = _7 + RET;
_47.fld3.fld5 = _30;
_51 = [93_i8,(-123_i8),33_i8,(-92_i8)];
_50.2 = _44;
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(17_usize, 27_usize, Move(_27), 17_usize, Move(_17), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(17_usize, 44_usize, Move(_44), 15_usize, Move(_15), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_52 = dump_var(17_usize, 10_usize, Move(_10), 39_usize, Move(_39), 48_usize, Move(_48), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_52 = dump_var(17_usize, 35_usize, Move(_35), 1_usize, Move(_1), 53_usize, _53, 53_usize, _53), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [u16; 7],mut _2: [i128; 7]) -> [i8; 4] {
mir! {
type RET = [i8; 4];
let _3: ([i128; 7],);
let _4: *const i64;
let _5: i32;
let _6: i8;
let _7: f32;
let _8: u16;
let _9: [i8; 3];
let _10: f32;
let _11: u64;
let _12: bool;
let _13: bool;
let _14: ([i128; 7],);
let _15: *const isize;
let _16: Adt53;
let _17: [char; 3];
let _18: i128;
let _19: Adt52;
let _20: isize;
let _21: Adt46;
let _22: ();
let _23: ();
{
RET = [(-110_i8),15_i8,(-112_i8),24_i8];
RET = [(-49_i8),46_i8,10_i8,0_i8];
RET = [(-120_i8),(-17_i8),106_i8,(-24_i8)];
_2 = [141573119121798636122151560173505727615_i128,(-129869559981190552118679639271497473187_i128),47055898286066974017289953426808062670_i128,147285115004627628635497987585534813283_i128,85887084547685045831494442493264707747_i128,(-129727664689151770389303126963967743265_i128),(-19130332342688027474369567286397493555_i128)];
RET = [15_i8,(-93_i8),(-78_i8),(-20_i8)];
RET = [(-15_i8),(-35_i8),(-58_i8),(-47_i8)];
RET = [(-62_i8),(-84_i8),125_i8,92_i8];
RET = [68_i8,92_i8,52_i8,3_i8];
_1 = [47464_u16,43277_u16,39903_u16,49678_u16,371_u16,29982_u16,57902_u16];
_1 = [852_u16,37642_u16,931_u16,62620_u16,35446_u16,39879_u16,32092_u16];
_3.0 = [(-122388765971840651405686599635071218945_i128),(-35415988080073702057024532906081387381_i128),25894146048423432748468177975931934666_i128,(-42434975938560895257617772515437749379_i128),(-1585679273375031550053279345955621488_i128),135512069421095823123992729704747183541_i128,(-2809320614109682873437658566805979131_i128)];
RET = [(-128_i8),(-22_i8),89_i8,47_i8];
RET = [(-27_i8),62_i8,117_i8,(-93_i8)];
RET = [96_i8,57_i8,44_i8,17_i8];
Call(_3 = fn19(_1, RET, RET, RET, _2, _1, _2, RET, _2, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-17_i8),(-113_i8),(-65_i8),(-92_i8)];
_1 = [39382_u16,39600_u16,3568_u16,16803_u16,59653_u16,28316_u16,55670_u16];
RET = [(-57_i8),(-35_i8),51_i8,53_i8];
_1 = [46295_u16,39597_u16,61612_u16,290_u16,60896_u16,49083_u16,48280_u16];
RET = [90_i8,59_i8,56_i8,48_i8];
_3.0 = [91078878765107179174351704508330273528_i128,(-40879514152711399653799354951941791292_i128),141266027395153478034206714890934508930_i128,(-109771929240425859615719201303377259248_i128),(-39227750513920527490211588769477493719_i128),(-165057034416147684417440245290068007263_i128),133642473495603065583861905381633373871_i128];
_2 = [(-29576634620563372992631362061045990331_i128),(-54545981097385019166456443152327153749_i128),4359490934029584382399217468736310173_i128,7465743101633765947237471057091477572_i128,(-11004802194841337374257055037280538578_i128),7177830565206718180152940061249057520_i128,122275527118604192461850986240132500817_i128];
_2 = [22268403005585781226328185228972326956_i128,24077169034731100901999838446816100881_i128,47349639873517184675206740126127690008_i128,127061464142481163897598614636008414390_i128,26706413443909616289612605906289801336_i128,(-37140836170895417512014893615848370896_i128),144262545091499199383514765901077865734_i128];
_3.0 = _2;
_2 = [(-72114478174897376751880677853443526810_i128),133723864498456310907521907558868442369_i128,(-20624355949958962932161659354635802276_i128),48017634990795423789373256921933824408_i128,11913007945501870222641909691165691104_i128,(-124129718569795487883949834250398771700_i128),62299014239530146999713326314219351008_i128];
_3 = (_2,);
_3.0 = [38339192543344019567889131740869773035_i128,112906612799759547966547900595143672178_i128,(-30121508111382126270553833361400108955_i128),(-158981983074140308509109267800335841665_i128),(-43698064379604001045712153805222352791_i128),20237699483785366339835365341195255011_i128,(-150735380574677882081259886546204201487_i128)];
_2 = [(-143574423968896623080709900345513708177_i128),(-8891297480803963934264571367153506404_i128),137998396703228990320115723794970337323_i128,110533333397560797807692197993864009311_i128,(-10082904556215146771320110318201737835_i128),169326656581844600665230798959242966072_i128,(-107724410915781352499404895661116391545_i128)];
_5 = (-7649693_i32) & 1234540985_i32;
Call(_2 = core::intrinsics::transmute(_3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = [17734_u16,25181_u16,42469_u16,30999_u16,33916_u16,40645_u16,24552_u16];
_3.0 = _2;
_3 = (_2,);
RET = [(-84_i8),75_i8,93_i8,6_i8];
_1 = [6572_u16,54893_u16,64999_u16,7785_u16,24363_u16,2977_u16,55704_u16];
_2 = _3.0;
_1 = [55401_u16,37402_u16,39975_u16,54122_u16,52422_u16,1823_u16,52763_u16];
Goto(bb3)
}
bb3 = {
RET = [40_i8,(-15_i8),105_i8,(-36_i8)];
_7 = 2950383215717910770491104903208581662_i128 as f32;
RET = [33_i8,(-45_i8),(-7_i8),4_i8];
_3.0 = [28828830029086641002470074544573920860_i128,(-91132700071189103726756359821161969532_i128),74304421416107728653826874032846290867_i128,(-14329011150076981836974312246807173525_i128),163149218853788213001821257872849661516_i128,(-5061448244360474036265495111893626921_i128),(-105944647831741351205366836401946435363_i128)];
_6 = !109_i8;
_8 = 65032_u16;
_2 = _3.0;
_8 = !46490_u16;
_9 = [_6,_6,_6];
_3 = (_2,);
_3 = (_2,);
_5 = 1408010237_i32 ^ (-765741307_i32);
_1 = [_8,_8,_8,_8,_8,_8,_8];
_3 = (_2,);
Call(RET = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = (-1432007830_i32);
_10 = _7 + _7;
_8 = 61064_u16;
_6 = (-93_i8);
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211363 => bb9,
_ => bb8
}
}
bb5 = {
RET = [40_i8,(-15_i8),105_i8,(-36_i8)];
_7 = 2950383215717910770491104903208581662_i128 as f32;
RET = [33_i8,(-45_i8),(-7_i8),4_i8];
_3.0 = [28828830029086641002470074544573920860_i128,(-91132700071189103726756359821161969532_i128),74304421416107728653826874032846290867_i128,(-14329011150076981836974312246807173525_i128),163149218853788213001821257872849661516_i128,(-5061448244360474036265495111893626921_i128),(-105944647831741351205366836401946435363_i128)];
_6 = !109_i8;
_8 = 65032_u16;
_2 = _3.0;
_8 = !46490_u16;
_9 = [_6,_6,_6];
_3 = (_2,);
_3 = (_2,);
_5 = 1408010237_i32 ^ (-765741307_i32);
_1 = [_8,_8,_8,_8,_8,_8,_8];
_3 = (_2,);
Call(RET = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_1 = [17734_u16,25181_u16,42469_u16,30999_u16,33916_u16,40645_u16,24552_u16];
_3.0 = _2;
_3 = (_2,);
RET = [(-84_i8),75_i8,93_i8,6_i8];
_1 = [6572_u16,54893_u16,64999_u16,7785_u16,24363_u16,2977_u16,55704_u16];
_2 = _3.0;
_1 = [55401_u16,37402_u16,39975_u16,54122_u16,52422_u16,1823_u16,52763_u16];
Goto(bb3)
}
bb7 = {
RET = [(-17_i8),(-113_i8),(-65_i8),(-92_i8)];
_1 = [39382_u16,39600_u16,3568_u16,16803_u16,59653_u16,28316_u16,55670_u16];
RET = [(-57_i8),(-35_i8),51_i8,53_i8];
_1 = [46295_u16,39597_u16,61612_u16,290_u16,60896_u16,49083_u16,48280_u16];
RET = [90_i8,59_i8,56_i8,48_i8];
_3.0 = [91078878765107179174351704508330273528_i128,(-40879514152711399653799354951941791292_i128),141266027395153478034206714890934508930_i128,(-109771929240425859615719201303377259248_i128),(-39227750513920527490211588769477493719_i128),(-165057034416147684417440245290068007263_i128),133642473495603065583861905381633373871_i128];
_2 = [(-29576634620563372992631362061045990331_i128),(-54545981097385019166456443152327153749_i128),4359490934029584382399217468736310173_i128,7465743101633765947237471057091477572_i128,(-11004802194841337374257055037280538578_i128),7177830565206718180152940061249057520_i128,122275527118604192461850986240132500817_i128];
_2 = [22268403005585781226328185228972326956_i128,24077169034731100901999838446816100881_i128,47349639873517184675206740126127690008_i128,127061464142481163897598614636008414390_i128,26706413443909616289612605906289801336_i128,(-37140836170895417512014893615848370896_i128),144262545091499199383514765901077865734_i128];
_3.0 = _2;
_2 = [(-72114478174897376751880677853443526810_i128),133723864498456310907521907558868442369_i128,(-20624355949958962932161659354635802276_i128),48017634990795423789373256921933824408_i128,11913007945501870222641909691165691104_i128,(-124129718569795487883949834250398771700_i128),62299014239530146999713326314219351008_i128];
_3 = (_2,);
_3.0 = [38339192543344019567889131740869773035_i128,112906612799759547966547900595143672178_i128,(-30121508111382126270553833361400108955_i128),(-158981983074140308509109267800335841665_i128),(-43698064379604001045712153805222352791_i128),20237699483785366339835365341195255011_i128,(-150735380574677882081259886546204201487_i128)];
_2 = [(-143574423968896623080709900345513708177_i128),(-8891297480803963934264571367153506404_i128),137998396703228990320115723794970337323_i128,110533333397560797807692197993864009311_i128,(-10082904556215146771320110318201737835_i128),169326656581844600665230798959242966072_i128,(-107724410915781352499404895661116391545_i128)];
_5 = (-7649693_i32) & 1234540985_i32;
Call(_2 = core::intrinsics::transmute(_3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_3.0 = [(-57109817577514396346026731894395866564_i128),(-103124214166603298636230575022224527267_i128),(-113486679870417904973096422447057898074_i128),98491330790094484683977918855634447412_i128,(-104115143906969852898716712551552493383_i128),(-85978395745843653661845661173860865247_i128),(-36207415521532036936111008103275771578_i128)];
_10 = _5 as f32;
_9 = [_6,_6,_6];
_8 = 28263_u16 * 27829_u16;
_3.0 = [74981438933885724683223546184183262138_i128,65977638678980590624833256375065357473_i128,(-115276837028833473359009403102584819430_i128),123784430348731976517612304227464159276_i128,(-37286668422372179935550956480144834601_i128),(-59748757883265805977844643667747361026_i128),64321040616625307434694926464003018778_i128];
_10 = 199_u8 as f32;
Goto(bb10)
}
bb10 = {
_5 = (-1783403787_i32) * 1193899873_i32;
_1 = [_8,_8,_8,_8,_8,_8,_8];
RET = [_6,_6,_6,_6];
_3 = (_2,);
_7 = _5 as f32;
_3.0 = [9555871643026817802844482312602512674_i128,137690765502516361299226284300041308354_i128,165530537620886204441987822904148649612_i128,(-43206280555166087272577910263330775504_i128),117403872371656459510403381997550629665_i128,(-59437185143752504231995992724196020506_i128),106798627812705788122366698595006123575_i128];
_9 = [_6,_6,_6];
Goto(bb11)
}
bb11 = {
_12 = true;
_13 = _12;
_13 = _12;
_10 = _7 + _7;
_5 = 272939168_i32 >> _8;
_3 = (_2,);
_12 = _8 <= _8;
_3.0 = [(-116862809495045585407641106072317170417_i128),155422359604883174112223958629276680464_i128,(-139842617685909137842156163497843325743_i128),(-17413457498551104551214108032674499340_i128),150762284441115683816701346596782308411_i128,(-53651927617837621177636804301019806711_i128),42062041881775285081525795967452570059_i128];
_16 = Adt53::Variant0 { fld0: 8582450367084866547_u64 };
_7 = -_10;
_1 = [_8,_8,_8,_8,_8,_8,_8];
_14.0 = _3.0;
_5 = -2090061397_i32;
_5 = _8 as i32;
_3 = (_2,);
_11 = 440669328766089256_u64 + 5090211896157213326_u64;
_1 = [_8,_8,_8,_8,_8,_8,_8];
match _6 {
0 => bb12,
1 => bb13,
340282366920938463463374607431768211363 => bb15,
_ => bb14
}
}
bb12 = {
RET = [40_i8,(-15_i8),105_i8,(-36_i8)];
_7 = 2950383215717910770491104903208581662_i128 as f32;
RET = [33_i8,(-45_i8),(-7_i8),4_i8];
_3.0 = [28828830029086641002470074544573920860_i128,(-91132700071189103726756359821161969532_i128),74304421416107728653826874032846290867_i128,(-14329011150076981836974312246807173525_i128),163149218853788213001821257872849661516_i128,(-5061448244360474036265495111893626921_i128),(-105944647831741351205366836401946435363_i128)];
_6 = !109_i8;
_8 = 65032_u16;
_2 = _3.0;
_8 = !46490_u16;
_9 = [_6,_6,_6];
_3 = (_2,);
_3 = (_2,);
_5 = 1408010237_i32 ^ (-765741307_i32);
_1 = [_8,_8,_8,_8,_8,_8,_8];
_3 = (_2,);
Call(RET = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_3.0 = [(-57109817577514396346026731894395866564_i128),(-103124214166603298636230575022224527267_i128),(-113486679870417904973096422447057898074_i128),98491330790094484683977918855634447412_i128,(-104115143906969852898716712551552493383_i128),(-85978395745843653661845661173860865247_i128),(-36207415521532036936111008103275771578_i128)];
_10 = _5 as f32;
_9 = [_6,_6,_6];
_8 = 28263_u16 * 27829_u16;
_3.0 = [74981438933885724683223546184183262138_i128,65977638678980590624833256375065357473_i128,(-115276837028833473359009403102584819430_i128),123784430348731976517612304227464159276_i128,(-37286668422372179935550956480144834601_i128),(-59748757883265805977844643667747361026_i128),64321040616625307434694926464003018778_i128];
_10 = 199_u8 as f32;
Goto(bb10)
}
bb14 = {
RET = [(-17_i8),(-113_i8),(-65_i8),(-92_i8)];
_1 = [39382_u16,39600_u16,3568_u16,16803_u16,59653_u16,28316_u16,55670_u16];
RET = [(-57_i8),(-35_i8),51_i8,53_i8];
_1 = [46295_u16,39597_u16,61612_u16,290_u16,60896_u16,49083_u16,48280_u16];
RET = [90_i8,59_i8,56_i8,48_i8];
_3.0 = [91078878765107179174351704508330273528_i128,(-40879514152711399653799354951941791292_i128),141266027395153478034206714890934508930_i128,(-109771929240425859615719201303377259248_i128),(-39227750513920527490211588769477493719_i128),(-165057034416147684417440245290068007263_i128),133642473495603065583861905381633373871_i128];
_2 = [(-29576634620563372992631362061045990331_i128),(-54545981097385019166456443152327153749_i128),4359490934029584382399217468736310173_i128,7465743101633765947237471057091477572_i128,(-11004802194841337374257055037280538578_i128),7177830565206718180152940061249057520_i128,122275527118604192461850986240132500817_i128];
_2 = [22268403005585781226328185228972326956_i128,24077169034731100901999838446816100881_i128,47349639873517184675206740126127690008_i128,127061464142481163897598614636008414390_i128,26706413443909616289612605906289801336_i128,(-37140836170895417512014893615848370896_i128),144262545091499199383514765901077865734_i128];
_3.0 = _2;
_2 = [(-72114478174897376751880677853443526810_i128),133723864498456310907521907558868442369_i128,(-20624355949958962932161659354635802276_i128),48017634990795423789373256921933824408_i128,11913007945501870222641909691165691104_i128,(-124129718569795487883949834250398771700_i128),62299014239530146999713326314219351008_i128];
_3 = (_2,);
_3.0 = [38339192543344019567889131740869773035_i128,112906612799759547966547900595143672178_i128,(-30121508111382126270553833361400108955_i128),(-158981983074140308509109267800335841665_i128),(-43698064379604001045712153805222352791_i128),20237699483785366339835365341195255011_i128,(-150735380574677882081259886546204201487_i128)];
_2 = [(-143574423968896623080709900345513708177_i128),(-8891297480803963934264571367153506404_i128),137998396703228990320115723794970337323_i128,110533333397560797807692197993864009311_i128,(-10082904556215146771320110318201737835_i128),169326656581844600665230798959242966072_i128,(-107724410915781352499404895661116391545_i128)];
_5 = (-7649693_i32) & 1234540985_i32;
Call(_2 = core::intrinsics::transmute(_3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_3 = (_14.0,);
_3.0 = [57654842903156772750376973741374851281_i128,(-96243079570226598387264664813177637511_i128),(-63919484763064970290525834274404652133_i128),(-147677094725200052901915682460924802789_i128),(-169961483408257837745302316418590308805_i128),(-78526103338678897215438583937266843860_i128),(-158993282106602548646632536782733774033_i128)];
_14 = _3;
_9 = [_6,_6,_6];
_10 = 48_u8 as f32;
_5 = 381995719_i32 << _8;
_18 = !89312448343616805292830386614412097610_i128;
_3 = (_14.0,);
_12 = _13;
_2 = _3.0;
_16 = Adt53::Variant0 { fld0: _11 };
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(18_usize, 5_usize, Move(_5), 3_usize, Move(_3), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(18_usize, 9_usize, Move(_9), 1_usize, Move(_1), 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [u16; 7],mut _2: [i8; 4],mut _3: [i8; 4],mut _4: [i8; 4],mut _5: [i128; 7],mut _6: [u16; 7],mut _7: [i128; 7],mut _8: [i8; 4],mut _9: [i128; 7],mut _10: [u16; 7],mut _11: [i128; 7]) -> ([i128; 7],) {
mir! {
type RET = ([i128; 7],);
let _12: f32;
let _13: f64;
let _14: [i128; 2];
let _15: isize;
let _16: (*mut u8, &'static char, bool);
let _17: f32;
let _18: Adt45;
let _19: f32;
let _20: u64;
let _21: i8;
let _22: i16;
let _23: *mut u8;
let _24: isize;
let _25: Adt52;
let _26: u128;
let _27: Adt55;
let _28: ();
let _29: ();
{
_8 = [(-94_i8),112_i8,83_i8,(-95_i8)];
RET.0 = _9;
RET.0 = [106149080573941441431525611638007628939_i128,(-50021836913241512339061420268072703011_i128),(-32578239837370195009341231604432977813_i128),37839526470398960213832113988307951062_i128,592799710300659028407949669804431386_i128,(-126869635348789573672918658639091437577_i128),(-104620342695474116354503871635531629156_i128)];
_6 = [24921_u16,37239_u16,27809_u16,8822_u16,64624_u16,60897_u16,24718_u16];
_1 = [17269_u16,59507_u16,7135_u16,52812_u16,44761_u16,44647_u16,11742_u16];
RET.0 = _9;
_6 = [44289_u16,29302_u16,11231_u16,27129_u16,4845_u16,50317_u16,40388_u16];
_8 = [114_i8,61_i8,50_i8,(-109_i8)];
Goto(bb1)
}
bb1 = {
RET.0 = [(-164520071265464749403631702846688373693_i128),(-19538359940788226141892829930280573151_i128),169834251019165334583347747977295129601_i128,158458618150669513973643862854319908737_i128,30920096640118202658200588640211865388_i128,87732850665817991599701531595233533597_i128,(-72508458159640293511663358435388087836_i128)];
_8 = [(-3_i8),(-37_i8),(-107_i8),2_i8];
_10 = _6;
_9 = _5;
_1 = [29265_u16,21826_u16,55328_u16,33701_u16,33776_u16,56311_u16,24846_u16];
_12 = 72_i8 as f32;
_10 = [20737_u16,52066_u16,40788_u16,27024_u16,19108_u16,27505_u16,40036_u16];
_11 = RET.0;
_5 = [(-110235426403607303736874340650807484866_i128),50858402289190191382296410813740643685_i128,(-159757118950757862301991858562194765040_i128),(-14083953425177929746417464258781016595_i128),127481989940318451191241214006876611631_i128,(-58514514989849265093826544062646855482_i128),(-17714629139791706436244194292638386237_i128)];
RET.0 = [103642198186465560543413281236971625204_i128,(-22543532597609592760124179095554737973_i128),168644903396889059933966779330237067025_i128,70446610465558804582609951198935658037_i128,(-2165868593618458839730481247499220447_i128),19276812225435049259968720860409196468_i128,(-35813085715576992435438391354255555340_i128)];
_3 = _4;
RET = (_11,);
_4 = [32_i8,109_i8,64_i8,59_i8];
_5 = [154969976514989392772976000049635957512_i128,(-21420144417980241450004647043080371073_i128),(-5918506652464968199692817886419322356_i128),156748793526667819635396521474500124152_i128,(-70597507589095359208931459030613677726_i128),(-10499830490631888913430032650388925717_i128),(-66030987511294669654976336774559825866_i128)];
_9 = _11;
Goto(bb2)
}
bb2 = {
RET.0 = _5;
_6 = [53406_u16,3659_u16,25448_u16,26783_u16,65522_u16,29904_u16,38546_u16];
_10 = [13934_u16,51803_u16,28702_u16,8008_u16,27049_u16,18851_u16,31004_u16];
_7 = _9;
_13 = _12 as f64;
RET.0 = [5434699445513721885700487329691488252_i128,(-155394152454016733356515731792581047485_i128),(-111814162279467009449502154888958711239_i128),(-95563200354801729946593860575422006586_i128),123359665625823365291904042067683926740_i128,130121816111785767544210398331085904987_i128,131740047720493126424431229117114676182_i128];
_3 = [26_i8,(-16_i8),58_i8,(-35_i8)];
Call(_5 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _7;
RET = (_11,);
_1 = _10;
_3 = [57_i8,(-83_i8),(-37_i8),(-50_i8)];
RET.0 = [124007124448484475223646215304846534429_i128,(-158173171273044929948689967014842014829_i128),26761783421119540865895967559434978026_i128,113633135414714425135501056337090717142_i128,(-68146379219367594113242051771713590283_i128),132103564420035188968128436786927122424_i128,(-96833934426776473423501985828675939335_i128)];
RET = (_9,);
_11 = [(-168403488237643245248797621955027987450_i128),89709273681439192192709176405294802496_i128,6843647410306335334424843138800556341_i128,(-41612788181050594535037855762390859877_i128),71981901961943932239021479401329329882_i128,(-147492136555136436291769687581727249475_i128),32760021110425440739691693559779006652_i128];
_3 = [99_i8,(-36_i8),99_i8,72_i8];
_13 = 56156_u16 as f64;
_10 = [64288_u16,64325_u16,64165_u16,61789_u16,30760_u16,38849_u16,22101_u16];
_6 = [23692_u16,1838_u16,9950_u16,60352_u16,42121_u16,62334_u16,60341_u16];
_6 = _10;
_12 = 5_usize as f32;
RET = (_5,);
_12 = (-6039_i16) as f32;
RET = (_5,);
RET = (_7,);
_2 = _4;
_13 = 80_i8 as f64;
_7 = [(-119523328184919066758605762297585823645_i128),89859544892864903404555107272393274076_i128,(-81609132196144815047450511262207138292_i128),16142029072082076525495957936894688236_i128,155860325782127550277736379493181107532_i128,(-31640944125424564429684949827313335906_i128),78877231816454137974793060852451875176_i128];
_4 = [(-66_i8),(-62_i8),114_i8,20_i8];
Goto(bb4)
}
bb4 = {
RET = (_9,);
_14 = [(-24481146686905549379240644093548383645_i128),(-159879473805334871161829011068856396396_i128)];
_11 = [(-62505068779104513911763299188562599559_i128),(-141389501056644188181406963835207502433_i128),(-127883951735794133830194092102298929456_i128),(-23162137264590277060913147909680415169_i128),(-20839824836807978244014329206831301612_i128),(-64108398920178454585418388522040356016_i128),87764084335579612806706687946058723140_i128];
_9 = [(-13417840643054741082724200683731021992_i128),40181340659802390212005421003026608042_i128,(-148444849288461060821141513112847949514_i128),(-128699981882222501750766721347199219019_i128),115432977779046408109408024839117551717_i128,92247978072834696988761313424591952431_i128,(-169196677505134007823429443143984010086_i128)];
_12 = 9223372036854775807_isize as f32;
_15 = 9223372036854775807_isize;
RET.0 = [(-52599040211064552949379397401618500626_i128),80820067247198520241659767835123874119_i128,(-59160915175955363452916897510479795769_i128),113277958696957647608800768780654562157_i128,(-157414653013932243944199399004733819064_i128),(-139125017348820991796034535810321598385_i128),53868230298658553956528170227947943644_i128];
_18.fld2 = !269214938498057604356141339432720406217_u128;
_19 = _12;
_19 = _12;
_2 = _4;
_15 = 9223372036854775807_isize;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb5 = {
_11 = _7;
RET = (_11,);
_1 = _10;
_3 = [57_i8,(-83_i8),(-37_i8),(-50_i8)];
RET.0 = [124007124448484475223646215304846534429_i128,(-158173171273044929948689967014842014829_i128),26761783421119540865895967559434978026_i128,113633135414714425135501056337090717142_i128,(-68146379219367594113242051771713590283_i128),132103564420035188968128436786927122424_i128,(-96833934426776473423501985828675939335_i128)];
RET = (_9,);
_11 = [(-168403488237643245248797621955027987450_i128),89709273681439192192709176405294802496_i128,6843647410306335334424843138800556341_i128,(-41612788181050594535037855762390859877_i128),71981901961943932239021479401329329882_i128,(-147492136555136436291769687581727249475_i128),32760021110425440739691693559779006652_i128];
_3 = [99_i8,(-36_i8),99_i8,72_i8];
_13 = 56156_u16 as f64;
_10 = [64288_u16,64325_u16,64165_u16,61789_u16,30760_u16,38849_u16,22101_u16];
_6 = [23692_u16,1838_u16,9950_u16,60352_u16,42121_u16,62334_u16,60341_u16];
_6 = _10;
_12 = 5_usize as f32;
RET = (_5,);
_12 = (-6039_i16) as f32;
RET = (_5,);
RET = (_7,);
_2 = _4;
_13 = 80_i8 as f64;
_7 = [(-119523328184919066758605762297585823645_i128),89859544892864903404555107272393274076_i128,(-81609132196144815047450511262207138292_i128),16142029072082076525495957936894688236_i128,155860325782127550277736379493181107532_i128,(-31640944125424564429684949827313335906_i128),78877231816454137974793060852451875176_i128];
_4 = [(-66_i8),(-62_i8),114_i8,20_i8];
Goto(bb4)
}
bb6 = {
RET.0 = _5;
_6 = [53406_u16,3659_u16,25448_u16,26783_u16,65522_u16,29904_u16,38546_u16];
_10 = [13934_u16,51803_u16,28702_u16,8008_u16,27049_u16,18851_u16,31004_u16];
_7 = _9;
_13 = _12 as f64;
RET.0 = [5434699445513721885700487329691488252_i128,(-155394152454016733356515731792581047485_i128),(-111814162279467009449502154888958711239_i128),(-95563200354801729946593860575422006586_i128),123359665625823365291904042067683926740_i128,130121816111785767544210398331085904987_i128,131740047720493126424431229117114676182_i128];
_3 = [26_i8,(-16_i8),58_i8,(-35_i8)];
Call(_5 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET.0 = [(-164520071265464749403631702846688373693_i128),(-19538359940788226141892829930280573151_i128),169834251019165334583347747977295129601_i128,158458618150669513973643862854319908737_i128,30920096640118202658200588640211865388_i128,87732850665817991599701531595233533597_i128,(-72508458159640293511663358435388087836_i128)];
_8 = [(-3_i8),(-37_i8),(-107_i8),2_i8];
_10 = _6;
_9 = _5;
_1 = [29265_u16,21826_u16,55328_u16,33701_u16,33776_u16,56311_u16,24846_u16];
_12 = 72_i8 as f32;
_10 = [20737_u16,52066_u16,40788_u16,27024_u16,19108_u16,27505_u16,40036_u16];
_11 = RET.0;
_5 = [(-110235426403607303736874340650807484866_i128),50858402289190191382296410813740643685_i128,(-159757118950757862301991858562194765040_i128),(-14083953425177929746417464258781016595_i128),127481989940318451191241214006876611631_i128,(-58514514989849265093826544062646855482_i128),(-17714629139791706436244194292638386237_i128)];
RET.0 = [103642198186465560543413281236971625204_i128,(-22543532597609592760124179095554737973_i128),168644903396889059933966779330237067025_i128,70446610465558804582609951198935658037_i128,(-2165868593618458839730481247499220447_i128),19276812225435049259968720860409196468_i128,(-35813085715576992435438391354255555340_i128)];
_3 = _4;
RET = (_11,);
_4 = [32_i8,109_i8,64_i8,59_i8];
_5 = [154969976514989392772976000049635957512_i128,(-21420144417980241450004647043080371073_i128),(-5918506652464968199692817886419322356_i128),156748793526667819635396521474500124152_i128,(-70597507589095359208931459030613677726_i128),(-10499830490631888913430032650388925717_i128),(-66030987511294669654976336774559825866_i128)];
_9 = _11;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_18.fld1 = 106_i8 << _18.fld2;
RET.0 = _9;
_18.fld1 = !75_i8;
_10 = _1;
Goto(bb11)
}
bb11 = {
_17 = (-1628573985_i32) as f32;
_22 = 16997_i16 | 29286_i16;
_21 = _18.fld1 + _18.fld1;
_11 = [23020583808545041874058621865114935173_i128,(-126553901018089137623857766188387243522_i128),17312835682843717135811082426777920540_i128,89816197516686992078739615259814123940_i128,(-3974763188484290341824571921656329856_i128),(-79800213217796355088959152579404323878_i128),31673120051967180803159589812324924587_i128];
RET.0 = [64545065335477323368854699285212265090_i128,74591498409365147476619675513049164710_i128,146163289570760446428933142415469065445_i128,(-59178637071848764938184550896264662592_i128),98172604558333017639196539104829806597_i128,169814204240564208855248108535270855380_i128,(-7533347899734946030579052631871930595_i128)];
_20 = !16784079964948905085_u64;
_13 = _18.fld2 as f64;
_8 = [_21,_21,_21,_21];
_12 = -_17;
_11 = [(-107473470133881379049230121764970195578_i128),76998720903594020236129899156048247170_i128,152330047801444090783146181982257952310_i128,(-145310751096544861334804348031188508620_i128),(-31949411195243094158811434314693736483_i128),(-36145104024486966635653623016867580295_i128),(-12221233367393808186526595276685727480_i128)];
_14 = [148808794281344651448100959658302463795_i128,(-53920007831861459564302041525033447759_i128)];
_1 = _10;
_6 = [49467_u16,55085_u16,31215_u16,52277_u16,48537_u16,42250_u16,1379_u16];
_21 = _18.fld1 * _18.fld1;
_5 = [(-119719897624851900575466682581990943647_i128),(-148653596222915363322197696083376114082_i128),(-124085264923683020207590441072394604400_i128),1058129467411030774717490551784425512_i128,(-31052143819894043976092828769390005626_i128),93823070829543726952453351235575803716_i128,(-123683323853771436224849695482434855796_i128)];
_18.fld0 = core::ptr::addr_of!(_15);
_19 = -_17;
RET = (_7,);
_16.2 = true;
_11 = [163892802816720398665731636871675894728_i128,97027998542220104044394204004841915534_i128,153488444080737686883694930249010228858_i128,(-146797453259383790514891853982767287483_i128),(-71191048990622975907577841660887712397_i128),65881553818020685350276199992756225257_i128,(-25200022498139914172301029366811941845_i128)];
_6 = [63986_u16,10026_u16,17286_u16,10706_u16,55232_u16,60254_u16,45304_u16];
_3 = [_21,_18.fld1,_18.fld1,_21];
_17 = -_12;
_16.2 = !true;
_10 = [4411_u16,19065_u16,32996_u16,27230_u16,5629_u16,58978_u16,31890_u16];
_13 = 3064288620_u32 as f64;
RET.0 = [(-126648850976402346597813287149703182505_i128),(-98358501024024189286952336112344167292_i128),134412694353343209614112858542252370565_i128,86914193087359282971144011920035046665_i128,166915120040189679184915209626167528925_i128,(-125963489774066402606544781873060334908_i128),(-28504876498895238196697407341594725118_i128)];
_21 = _18.fld1 | _18.fld1;
_18.fld2 = 273098611156229480841205047593324655389_u128;
match _15 {
0 => bb12,
9223372036854775807 => bb14,
_ => bb13
}
}
bb12 = {
_11 = _7;
RET = (_11,);
_1 = _10;
_3 = [57_i8,(-83_i8),(-37_i8),(-50_i8)];
RET.0 = [124007124448484475223646215304846534429_i128,(-158173171273044929948689967014842014829_i128),26761783421119540865895967559434978026_i128,113633135414714425135501056337090717142_i128,(-68146379219367594113242051771713590283_i128),132103564420035188968128436786927122424_i128,(-96833934426776473423501985828675939335_i128)];
RET = (_9,);
_11 = [(-168403488237643245248797621955027987450_i128),89709273681439192192709176405294802496_i128,6843647410306335334424843138800556341_i128,(-41612788181050594535037855762390859877_i128),71981901961943932239021479401329329882_i128,(-147492136555136436291769687581727249475_i128),32760021110425440739691693559779006652_i128];
_3 = [99_i8,(-36_i8),99_i8,72_i8];
_13 = 56156_u16 as f64;
_10 = [64288_u16,64325_u16,64165_u16,61789_u16,30760_u16,38849_u16,22101_u16];
_6 = [23692_u16,1838_u16,9950_u16,60352_u16,42121_u16,62334_u16,60341_u16];
_6 = _10;
_12 = 5_usize as f32;
RET = (_5,);
_12 = (-6039_i16) as f32;
RET = (_5,);
RET = (_7,);
_2 = _4;
_13 = 80_i8 as f64;
_7 = [(-119523328184919066758605762297585823645_i128),89859544892864903404555107272393274076_i128,(-81609132196144815047450511262207138292_i128),16142029072082076525495957936894688236_i128,155860325782127550277736379493181107532_i128,(-31640944125424564429684949827313335906_i128),78877231816454137974793060852451875176_i128];
_4 = [(-66_i8),(-62_i8),114_i8,20_i8];
Goto(bb4)
}
bb13 = {
Return()
}
bb14 = {
_22 = -(-16101_i16);
_13 = 8054241132693474776_i64 as f64;
RET.0 = _11;
_6 = [65284_u16,28272_u16,21663_u16,58428_u16,12651_u16,43242_u16,28291_u16];
_18.fld2 = !253835166248557247964329089496650727624_u128;
_3 = _8;
_18.fld0 = core::ptr::addr_of!(_24);
_10 = [27785_u16,21233_u16,35380_u16,51567_u16,1677_u16,28401_u16,12525_u16];
RET = (_11,);
_15 = !(-16_isize);
_6 = [9306_u16,36531_u16,59766_u16,17728_u16,13610_u16,756_u16,48830_u16];
_2 = _4;
_19 = -_17;
_16.2 = !true;
_8 = [_21,_21,_21,_21];
_15 = (-3676145274177518206_i64) as isize;
_14 = [(-145137868841882134894275247962172479334_i128),147552546457797328618302625647193276538_i128];
_21 = _18.fld1;
_9 = [(-86357284747794575059562304996789981241_i128),102598209424507075312688543566065255639_i128,(-167479836202917829258881465995406838813_i128),(-67402637128813584102725699932504823444_i128),152902963102128037706514440025161582845_i128,121985067965608060284354068819719574547_i128,(-95071610607356460430105930317859196473_i128)];
_24 = _15 + _15;
_15 = !_24;
_26 = _18.fld2;
_1 = [58078_u16,19446_u16,7358_u16,13432_u16,9146_u16,50720_u16,32836_u16];
_18.fld2 = _26 << _21;
_20 = !8376100966625103771_u64;
_22 = (-30228_i16);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(19_usize, 24_usize, Move(_24), 22_usize, Move(_22), 15_usize, Move(_15), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(19_usize, 8_usize, Move(_8), 21_usize, Move(_21), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(19_usize, 26_usize, Move(_26), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{7e801}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-6_i8)), std::hint::black_box((-804_i16)), std::hint::black_box((-2135210389_i32)), std::hint::black_box((-93041168882558821_i64)), std::hint::black_box(60286548190329481982926048186990045835_i128), std::hint::black_box(14217252550686723415_usize), std::hint::black_box(71_u8), std::hint::black_box(10794_u16), std::hint::black_box(97595341648099305368508600826568158288_u128), std::hint::black_box(17756097862438786646_u64));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: i16,
fld1: [i128; 7],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *const isize,
fld1: i8,
fld2: u128,
}
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *const i8,
fld1: char,
fld2: *const isize,

},
Variant1{
fld0: u8,
fld1: [usize; 7],
fld2: *const i8,
fld3: i8,
fld4: *const f64,

},
Variant2{
fld0: bool,
fld1: *const f32,
fld2: [i128; 7],
fld3: i8,
fld4: *mut char,
fld5: u128,

},
Variant3{
fld0: bool,
fld1: f32,
fld2: u8,
fld3: *const i8,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [i8; 3],
fld1: *const isize,
fld2: isize,
fld3: i32,
fld4: u128,

},
Variant1{
fld0: i128,
fld1: char,
fld2: [char; 3],
fld3: i64,
fld4: Adt45,
fld5: f64,

},
Variant2{
fld0: *const i64,
fld1: f32,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u32,
fld1: char,
fld2: isize,
fld3: [usize; 7],
fld4: u16,
fld5: f32,

},
Variant1{
fld0: u32,
fld1: i32,
fld2: [char; 3],
fld3: u128,
fld4: i16,

},
Variant2{
fld0: f32,
fld1: [i32; 8],
fld2: [isize; 2],
fld3: ([i128; 7],),
fld4: [i128; 2],

},
Variant3{
fld0: [i8; 4],
fld1: [char; 3],
fld2: Adt45,
fld3: f32,
fld4: Adt44,

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: bool,
fld1: [i128; 7],
fld2: [i128; 2],
fld3: Adt46,
fld4: Adt48,

},
Variant1{
fld0: bool,
fld1: usize,
fld2: ([i128; 7], *const f64, ([i128; 2], [u16; 7], u64), [i8; 3], [i8; 4], *const i64, u32),
fld3: *const i16,
fld4: Adt48,
fld5: *const i8,
fld6: *mut char,
fld7: u32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: ([i128; 7], *const f64, ([i128; 2], [u16; 7], u64), [i8; 3], [i8; 4], *const i64, u32),
fld1: usize,
fld2: *const f32,
fld3: Adt47,
fld4: [i8; 4],
fld5: [char; 3],
fld6: *const i64,
fld7: [i8; 3],
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [u64; 6],
fld1: *const f64,
fld2: [usize; 7],
fld3: ([i128; 7],),
fld4: [isize; 2],
fld5: *const i16,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const isize,
fld1: char,
fld2: u32,
fld3: u16,
fld4: i128,
fld5: [i8; 4],
fld6: [i128; 7],

},
Variant1{
fld0: Adt46,
fld1: *const i16,
fld2: u128,
fld3: *mut u8,
fld4: usize,
fld5: i32,
fld6: *const f32,
fld7: u32,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u64,

},
Variant1{
fld0: ([i128; 2], [u16; 7], u64),
fld1: Adt44,
fld2: Adt52,
fld3: Adt49,
fld4: [i32; 8],
fld5: Adt46,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: *const f32,
fld1: u16,
fld2: i32,
fld3: Adt48,
fld4: Adt52,

},
Variant1{
fld0: bool,
fld1: [i128; 7],
fld2: Adt45,
fld3: Adt46,
fld4: f32,
fld5: *mut f32,

},
Variant2{
fld0: *const i16,
fld1: char,
fld2: [i8; 3],
fld3: *const f32,
fld4: Adt48,
fld5: Adt44,
fld6: i64,

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *const f64,
fld1: Adt52,
fld2: [usize; 7],
fld3: i16,

},
Variant1{
fld0: Adt46,
fld1: Adt48,
fld2: [i128; 2],
fld3: Adt47,
fld4: *const isize,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt48,
fld1: [i8; 3],
fld2: isize,
fld3: Adt49,
fld4: [isize; 2],
fld5: u64,
fld6: i64,

},
Variant1{
fld0: [u16; 7],
fld1: [usize; 7],

},
Variant2{
fld0: i64,
fld1: i32,
fld2: *const f64,
fld3: Adt47,
fld4: [char; 3],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: bool,
fld1: *mut char,

},
Variant1{
fld0: Adt56,
fld1: [u16; 7],
fld2: Adt45,
fld3: Adt49,
fld4: u32,
fld5: Adt44,
fld6: *const f64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: [isize; 2],
fld1: Adt52,
fld2: isize,
fld3: [i8; 3],
fld4: Adt45,
fld5: u64,
fld6: Adt46,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: [isize; 2],
fld1: Adt54,
fld2: [u16; 7],
fld3: Adt51,
fld4: u64,
fld5: *const i64,
fld6: f64,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: u8,
fld1: *mut f32,
fld2: *const isize,
fld3: ([i128; 2], [u16; 7], u64),

},
Variant1{
fld0: f64,
fld1: Adt48,
fld2: [usize; 7],
fld3: u8,

}}

