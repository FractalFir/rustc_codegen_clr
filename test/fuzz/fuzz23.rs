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
pub fn fn0(mut _1: u64,mut _2: u8,mut _3: u32,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u128,mut _8: u16) -> u32 {
mir! {
type RET = u32;
let _9: bool;
let _10: i64;
let _11: [i128; 1];
let _12: (u64, [i128; 1], isize);
let _13: isize;
let _14: Adt45;
let _15: (usize,);
let _16: bool;
let _17: char;
let _18: (usize,);
let _19: (u64, bool);
let _20: Adt48;
let _21: Adt52;
let _22: f64;
let _23: [i128; 1];
let _24: Adt48;
let _25: (bool,);
let _26: *mut isize;
let _27: (bool,);
let _28: f64;
let _29: u64;
let _30: char;
let _31: ();
let _32: ();
{
_5 = (-3369_i16) ^ 11620_i16;
_6 = 344730160_i32;
Call(_3 = fn1(_6, _6, _5, _5, _5, _6, _5, _6, _6, _5, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 39628_u16 + 43213_u16;
_1 = _8 as u64;
Goto(bb2)
}
bb2 = {
_1 = 17300546120933332111_u64;
_6 = -(-1784810760_i32);
_10 = 2302295878679299275_i64;
_9 = false ^ false;
_12.2 = _8 as isize;
_11 = [70527642740735104618362704083421854893_i128];
_7 = 139591347577165530092038846885873876116_u128 << _1;
_13 = _12.2;
_9 = true;
_15.0 = !2435562881859987198_usize;
_16 = _9;
_2 = 98_u8;
_4 = (-6_i8);
_18.0 = _15.0;
_2 = 70_u8 - 113_u8;
_3 = 898523365_u32 >> _15.0;
_18.0 = !_15.0;
_19.0 = !_1;
Goto(bb3)
}
bb3 = {
_7 = 153482828835016035879411567941821838228_i128 as u128;
_13 = _12.2 >> _15.0;
_18 = _15;
_1 = _19.0;
_12 = (_1, _11, _13);
_12 = (_19.0, _11, _13);
_22 = _4 as f64;
_12.2 = _13 * _13;
_23 = [(-58749880414461109225183188012266017199_i128)];
Goto(bb4)
}
bb4 = {
_1 = !_19.0;
_3 = !3301998240_u32;
_13 = 34654306214886682552413862555586259778_i128 as isize;
_15.0 = (-82310766889979318490858070918941047085_i128) as usize;
_15 = (_18.0,);
_5 = _3 as i16;
_11 = _12.1;
_2 = _4 as u8;
_9 = _12.2 > _12.2;
_9 = _12.2 != _12.2;
match _10 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
2302295878679299275 => bb12,
_ => bb11
}
}
bb5 = {
_7 = 153482828835016035879411567941821838228_i128 as u128;
_13 = _12.2 >> _15.0;
_18 = _15;
_1 = _19.0;
_12 = (_1, _11, _13);
_12 = (_19.0, _11, _13);
_22 = _4 as f64;
_12.2 = _13 * _13;
_23 = [(-58749880414461109225183188012266017199_i128)];
Goto(bb4)
}
bb6 = {
_1 = 17300546120933332111_u64;
_6 = -(-1784810760_i32);
_10 = 2302295878679299275_i64;
_9 = false ^ false;
_12.2 = _8 as isize;
_11 = [70527642740735104618362704083421854893_i128];
_7 = 139591347577165530092038846885873876116_u128 << _1;
_13 = _12.2;
_9 = true;
_15.0 = !2435562881859987198_usize;
_16 = _9;
_2 = 98_u8;
_4 = (-6_i8);
_18.0 = _15.0;
_2 = 70_u8 - 113_u8;
_3 = 898523365_u32 >> _15.0;
_18.0 = !_15.0;
_19.0 = !_1;
Goto(bb3)
}
bb7 = {
_8 = 39628_u16 + 43213_u16;
_1 = _8 as u64;
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
_26 = core::ptr::addr_of_mut!(_12.2);
_19 = Checked(_1 * _1);
(*_26) = _2 as isize;
_16 = _9 >= _9;
Goto(bb13)
}
bb13 = {
_25 = (_9,);
RET = _3;
_17 = '\u{38231}';
_18.0 = _15.0 + _15.0;
RET = (-100994849047397888887746203326254970835_i128) as u32;
_1 = _19.0 + _19.0;
_22 = _5 as f64;
_12.0 = _9 as u64;
_3 = RET >> _12.0;
_12.1 = [(-100181438250382036534126503883556726534_i128)];
_12.0 = _3 as u64;
_22 = _4 as f64;
_12.1 = [(-133449529256090098577535250538565971860_i128)];
_26 = core::ptr::addr_of_mut!(_12.2);
_19.1 = !_9;
(*_26) = _13 >> RET;
(*_26) = !_13;
_19.0 = !_12.0;
_19.1 = !_9;
_27.0 = _9;
_12.1 = _11;
_18.0 = _15.0;
_3 = _16 as u32;
_18 = (_15.0,);
_9 = _25.0;
_28 = 158359576249502183509474058692924706148_i128 as f64;
(*_26) = -_13;
_19.1 = _9;
(*_26) = _13;
match _10 {
0 => bb4,
2302295878679299275 => bb14,
_ => bb8
}
}
bb14 = {
_5 = (-3033_i16);
_11 = [20130754436539819088965933383520302461_i128];
_3 = !RET;
_26 = core::ptr::addr_of_mut!((*_26));
_17 = '\u{f3f99}';
_11 = [(-69072561598575542093676701980084161323_i128)];
_19.0 = _12.0 ^ _1;
_12.1 = [133380218070531895121827629233503661608_i128];
_12.2 = !_13;
_4 = 57_i8;
_28 = _5 as f64;
_26 = core::ptr::addr_of_mut!(_12.2);
_2 = !18_u8;
_27 = _25;
_12.0 = _19.0 ^ _19.0;
_30 = _17;
_6 = (-1225067172_i32) >> _3;
_12.1 = [(-165994784623505112566472621394461171212_i128)];
_2 = 75_u8 & 90_u8;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 10_usize, Move(_10), 7_usize, Move(_7), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 19_usize, Move(_19), 8_usize, Move(_8), 25_usize, Move(_25), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 6_usize, Move(_6), 4_usize, Move(_4), 11_usize, Move(_11), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i32,mut _2: i32,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i32,mut _7: i16,mut _8: i32,mut _9: i32,mut _10: i16,mut _11: i32) -> u32 {
mir! {
type RET = u32;
let _12: isize;
let _13: [i128; 1];
let _14: u16;
let _15: u8;
let _16: char;
let _17: (f64, f64);
let _18: u32;
let _19: [i128; 1];
let _20: *mut char;
let _21: char;
let _22: *const (u64, bool);
let _23: f32;
let _24: isize;
let _25: u32;
let _26: (u64, [i128; 1], isize);
let _27: (f64, f64);
let _28: (i128, usize);
let _29: ();
let _30: ();
{
_1 = _11;
_4 = _10 >> _6;
_2 = _1 >> _3;
_3 = !_4;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
344730160 => bb5,
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
RET = 997783539_u32;
_11 = _9 & _8;
_11 = _2;
_12 = (-9223372036854775808_isize);
_9 = _6 * _11;
_1 = _11;
_5 = _7 << RET;
_2 = _1;
_2 = !_11;
RET = 2473536734_u32;
_13 = [9814309225125368187513186103216274130_i128];
_3 = _4 << _9;
_4 = _5 & _3;
match _8 {
344730160 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_1 = _9;
_8 = _2;
_17.0 = 175_u8 as f64;
_8 = !_1;
_17.1 = -_17.0;
_16 = '\u{c55de}';
_1 = 24215_u16 as i32;
_5 = -_3;
_3 = -_4;
_2 = 544521610592588892_i64 as i32;
_15 = RET as u8;
_3 = _4 - _5;
_16 = '\u{f6f60}';
_9 = _11 & _8;
_7 = 5790045913290441048_u64 as i16;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
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
_10 = _4 & _3;
_8 = _9 + _2;
_5 = _10 * _10;
_6 = _15 as i32;
_5 = _10;
_15 = _12 as u8;
_2 = _6 | _11;
_13 = [(-53286685737993181581456964319575259022_i128)];
_12 = 38354_u16 as isize;
_21 = _16;
_1 = _2 & _2;
_14 = !65367_u16;
_6 = _15 as i32;
_11 = !_9;
_19 = _13;
_10 = -_3;
_17.1 = _17.0;
_21 = _16;
Call(_18 = fn2(_3, _4, _8, _11, _1, _4, _10, _3, _10, _5, _9, _2, _3, _5, _2, _5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20 = core::ptr::addr_of_mut!(_16);
_1 = _8 & _8;
_13 = [(-23658747277013448428966231280541054321_i128)];
_11 = 286493059365318775251678027979419846241_u128 as i32;
_9 = _1 << _5;
_11 = _16 as i32;
_9 = _1 << _5;
_21 = (*_20);
_3 = !_10;
_1 = _9 & _9;
_1 = _17.0 as i32;
_24 = _12;
_1 = _14 as i32;
_21 = _16;
_15 = 2_u8 >> _10;
_3 = (-43264387428546102739828243433272199904_i128) as i16;
_23 = (-124_i8) as f32;
_20 = core::ptr::addr_of_mut!((*_20));
_17.0 = _17.1;
_25 = RET;
_23 = _24 as f32;
RET = _25;
Goto(bb13)
}
bb13 = {
(*_20) = _21;
_25 = _18 | RET;
_24 = true as isize;
_8 = !_9;
_11 = _8;
_19 = [(-95699719256831924721210940971907792711_i128)];
_6 = 167761397924498945516054589147479706662_u128 as i32;
_1 = _11 << _9;
_20 = core::ptr::addr_of_mut!((*_20));
_12 = _24;
_10 = !_4;
(*_20) = _21;
Goto(bb14)
}
bb14 = {
_23 = _15 as f32;
_6 = _23 as i32;
_11 = _9;
_9 = !_1;
_7 = _5 * _5;
_4 = 78_i8 as i16;
_27 = (_17.0, _17.0);
_4 = 3_usize as i16;
_28.1 = 11991581494667759679_usize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(1_usize, 2_usize, Move(_2), 25_usize, Move(_25), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(1_usize, 7_usize, Move(_7), 13_usize, Move(_13), 15_usize, Move(_15), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(1_usize, 16_usize, Move(_16), 5_usize, Move(_5), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i16,mut _2: i16,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i16,mut _7: i16,mut _8: i16,mut _9: i16,mut _10: i16,mut _11: i32,mut _12: i32,mut _13: i16,mut _14: i16,mut _15: i32,mut _16: i16) -> u32 {
mir! {
type RET = u32;
let _17: *mut (f64, f64);
let _18: (bool,);
let _19: [i128; 2];
let _20: (f64, f64);
let _21: i32;
let _22: isize;
let _23: (f64, f64);
let _24: (u64, bool);
let _25: (i128, usize);
let _26: (*mut [i128; 1], *const *const (u64, bool));
let _27: (u128, usize, i64, u128);
let _28: (usize,);
let _29: u64;
let _30: i8;
let _31: i128;
let _32: [u32; 1];
let _33: u8;
let _34: *const *const (u64, bool);
let _35: ((f64, f64), [i128; 2], (f64, f64));
let _36: char;
let _37: i32;
let _38: char;
let _39: ();
let _40: ();
{
_8 = _4 as i16;
_4 = !_15;
_7 = _10;
RET = !493180190_u32;
Call(_6 = fn3(_16, _13, _5, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _16 >> _2;
_7 = _8 >> _16;
_15 = _12;
_18 = (false,);
_5 = _12;
_16 = -_2;
_6 = (-114_i8) as i16;
_10 = !_8;
_8 = !_14;
_11 = -_12;
_12 = _15;
RET = 727078758_u32 * 1508803229_u32;
_4 = -_5;
_15 = -_11;
Call(_3 = fn13(_15, _1, _10, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 3625349915_u32 - 1549162206_u32;
RET = 9223372036854775807_isize as u32;
_13 = 251_u8 as i16;
_8 = _7;
_15 = _12;
_11 = 61444515276302080_u64 as i32;
_12 = _3;
_13 = _14 | _8;
_9 = _7;
_3 = _4 & _11;
_16 = _13;
_1 = RET as i16;
_1 = _13 & _7;
_14 = _7 + _16;
_16 = _14;
_11 = _12 & _5;
_11 = _12 * _12;
_8 = _1;
_17 = core::ptr::addr_of_mut!(_20);
_15 = _12;
(*_17).1 = 109_u8 as f64;
_18.0 = !true;
_19 = [66701735378364781141225433743008765951_i128,(-7404488504134711358388260705426395703_i128)];
_5 = -_11;
Call((*_17).0 = fn14(_14, _16, _9, _9, _13, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = !_10;
_15 = 265020757368741285313518514926521509240_u128 as i32;
Goto(bb4)
}
bb4 = {
(*_17).1 = (-84049849439380949643102728358180425237_i128) as f64;
(*_17).1 = (*_17).0;
_22 = 52_isize;
_12 = (-121776903471334895358593012276900492679_i128) as i32;
_4 = _11;
(*_17).1 = (*_17).0;
_18 = (false,);
_24.0 = '\u{f2219}' as u64;
_23.0 = -(*_17).0;
(*_17).0 = -_23.0;
_8 = _10;
_7 = !_16;
_13 = _16 + _9;
_18.0 = false;
(*_17) = (_23.0, _23.0);
_25.0 = '\u{1e345}' as i128;
Goto(bb5)
}
bb5 = {
(*_17).1 = -_20.0;
(*_17).1 = 79274612086787068920138686579613362420_u128 as f64;
_8 = -_7;
Goto(bb6)
}
bb6 = {
_16 = _8 + _8;
_3 = _11;
_6 = _24.0 as i16;
_23 = _20;
(*_17).0 = (*_17).1;
_23.1 = (*_17).0;
_18 = (true,);
_20 = _23;
(*_17).1 = _23.1;
_27.3 = !284669330396203599918494955779967537740_u128;
(*_17).0 = _23.1 + _23.0;
(*_17).0 = -_20.1;
_27.0 = _27.3 >> _8;
_18.0 = _14 >= _13;
_18.0 = !true;
match _22 {
0 => bb4,
52 => bb8,
_ => bb7
}
}
bb7 = {
(*_17).1 = (-84049849439380949643102728358180425237_i128) as f64;
(*_17).1 = (*_17).0;
_22 = 52_isize;
_12 = (-121776903471334895358593012276900492679_i128) as i32;
_4 = _11;
(*_17).1 = (*_17).0;
_18 = (false,);
_24.0 = '\u{f2219}' as u64;
_23.0 = -(*_17).0;
(*_17).0 = -_23.0;
_8 = _10;
_7 = !_16;
_13 = _16 + _9;
_18.0 = false;
(*_17) = (_23.0, _23.0);
_25.0 = '\u{1e345}' as i128;
Goto(bb5)
}
bb8 = {
_12 = _3 >> _16;
_25.0 = -(-132959837917375123769571369531245927039_i128);
_30 = -10_i8;
(*_17).0 = -_23.1;
_23 = (*_17);
_28 = (17079077992357156684_usize,);
_23 = ((*_17).0, (*_17).0);
_2 = !_1;
_14 = _7;
_28 = (8758200002279128488_usize,);
_27.0 = _28.0 as u128;
(*_17) = _23;
_19 = [_25.0,_25.0];
_22 = 9223372036854775807_isize - (-32_isize);
_8 = -_10;
_2 = _24.0 as i16;
_15 = _5 * _5;
_22 = 9223372036854775807_isize;
_15 = _12;
_21 = _8 as i32;
_27.1 = RET as usize;
Goto(bb9)
}
bb9 = {
_6 = _16 << _3;
(*_17).0 = -_23.0;
_27.1 = 2_u8 as usize;
_11 = _15 << _14;
_27 = (2951363725333538768552315172462116741_u128, _28.0, (-2159901696772632845_i64), 24606943473387674584287462063037521816_u128);
_7 = _13 >> _10;
_29 = !_24.0;
_27 = (33156389687791241037742546879299537160_u128, _28.0, 5739476692682657067_i64, 187981880823489377757969844051916865387_u128);
_20.1 = (*_17).0;
_25.0 = !(-51245091008037656721778642433965506018_i128);
_24.1 = _18.0 ^ _18.0;
_16 = !_10;
Call(_29 = core::intrinsics::bswap(_24.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_12 = _15;
_17 = core::ptr::addr_of_mut!((*_17));
_33 = !98_u8;
_7 = _14 * _14;
_13 = _14 >> _1;
_16 = _10 ^ _6;
_35.1 = [_25.0,_25.0];
_33 = _27.0 as u8;
(*_17).0 = _23.0 - _23.1;
_11 = _27.2 as i32;
_21 = _12;
match _27.2 {
0 => bb9,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
5739476692682657067 => bb18,
_ => bb17
}
}
bb11 = {
RET = 3625349915_u32 - 1549162206_u32;
RET = 9223372036854775807_isize as u32;
_13 = 251_u8 as i16;
_8 = _7;
_15 = _12;
_11 = 61444515276302080_u64 as i32;
_12 = _3;
_13 = _14 | _8;
_9 = _7;
_3 = _4 & _11;
_16 = _13;
_1 = RET as i16;
_1 = _13 & _7;
_14 = _7 + _16;
_16 = _14;
_11 = _12 & _5;
_11 = _12 * _12;
_8 = _1;
_17 = core::ptr::addr_of_mut!(_20);
_15 = _12;
(*_17).1 = 109_u8 as f64;
_18.0 = !true;
_19 = [66701735378364781141225433743008765951_i128,(-7404488504134711358388260705426395703_i128)];
_5 = -_11;
Call((*_17).0 = fn14(_14, _16, _9, _9, _13, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_12 = _3 >> _16;
_25.0 = -(-132959837917375123769571369531245927039_i128);
_30 = -10_i8;
(*_17).0 = -_23.1;
_23 = (*_17);
_28 = (17079077992357156684_usize,);
_23 = ((*_17).0, (*_17).0);
_2 = !_1;
_14 = _7;
_28 = (8758200002279128488_usize,);
_27.0 = _28.0 as u128;
(*_17) = _23;
_19 = [_25.0,_25.0];
_22 = 9223372036854775807_isize - (-32_isize);
_8 = -_10;
_2 = _24.0 as i16;
_15 = _5 * _5;
_22 = 9223372036854775807_isize;
_15 = _12;
_21 = _8 as i32;
_27.1 = RET as usize;
Goto(bb9)
}
bb13 = {
(*_17).1 = (-84049849439380949643102728358180425237_i128) as f64;
(*_17).1 = (*_17).0;
_22 = 52_isize;
_12 = (-121776903471334895358593012276900492679_i128) as i32;
_4 = _11;
(*_17).1 = (*_17).0;
_18 = (false,);
_24.0 = '\u{f2219}' as u64;
_23.0 = -(*_17).0;
(*_17).0 = -_23.0;
_8 = _10;
_7 = !_16;
_13 = _16 + _9;
_18.0 = false;
(*_17) = (_23.0, _23.0);
_25.0 = '\u{1e345}' as i128;
Goto(bb5)
}
bb14 = {
_16 = _8 + _8;
_3 = _11;
_6 = _24.0 as i16;
_23 = _20;
(*_17).0 = (*_17).1;
_23.1 = (*_17).0;
_18 = (true,);
_20 = _23;
(*_17).1 = _23.1;
_27.3 = !284669330396203599918494955779967537740_u128;
(*_17).0 = _23.1 + _23.0;
(*_17).0 = -_20.1;
_27.0 = _27.3 >> _8;
_18.0 = _14 >= _13;
_18.0 = !true;
match _22 {
0 => bb4,
52 => bb8,
_ => bb7
}
}
bb15 = {
(*_17).1 = -_20.0;
(*_17).1 = 79274612086787068920138686579613362420_u128 as f64;
_8 = -_7;
Goto(bb6)
}
bb16 = {
(*_17).1 = (-84049849439380949643102728358180425237_i128) as f64;
(*_17).1 = (*_17).0;
_22 = 52_isize;
_12 = (-121776903471334895358593012276900492679_i128) as i32;
_4 = _11;
(*_17).1 = (*_17).0;
_18 = (false,);
_24.0 = '\u{f2219}' as u64;
_23.0 = -(*_17).0;
(*_17).0 = -_23.0;
_8 = _10;
_7 = !_16;
_13 = _16 + _9;
_18.0 = false;
(*_17) = (_23.0, _23.0);
_25.0 = '\u{1e345}' as i128;
Goto(bb5)
}
bb17 = {
_8 = _16 >> _2;
_7 = _8 >> _16;
_15 = _12;
_18 = (false,);
_5 = _12;
_16 = -_2;
_6 = (-114_i8) as i16;
_10 = !_8;
_8 = !_14;
_11 = -_12;
_12 = _15;
RET = 727078758_u32 * 1508803229_u32;
_4 = -_5;
_15 = -_11;
Call(_3 = fn13(_15, _1, _10, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
(*_17) = (_23.0, _23.1);
_17 = core::ptr::addr_of_mut!(_23);
_30 = -84_i8;
_28.0 = !_27.1;
(*_17) = (_20.1, _20.0);
_25 = (156109223827259396604169047987443173612_i128, _28.0);
_35.0.0 = _23.0;
_27.2 = -(-1801610546756823572_i64);
_18 = (_24.1,);
_1 = !_13;
_15 = _21 & _12;
_28 = (_25.1,);
_18 = (_24.1,);
_28.0 = _27.1;
_16 = _1 << _7;
_35.2 = _20;
(*_17).0 = _35.2.1 - _35.0.0;
_35.2.0 = _3 as f64;
_19 = [_25.0,_25.0];
_35.2.1 = -_35.2.0;
_28.0 = _27.1 / _27.1;
Goto(bb19)
}
bb19 = {
Call(_39 = dump_var(2_usize, 25_usize, Move(_25), 15_usize, Move(_15), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(2_usize, 10_usize, Move(_10), 29_usize, Move(_29), 3_usize, Move(_3), 22_usize, Move(_22)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(2_usize, 27_usize, Move(_27), 18_usize, Move(_18), 19_usize, Move(_19), 30_usize, Move(_30)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(2_usize, 13_usize, Move(_13), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i16,mut _2: i16,mut _3: i32,mut _4: i16) -> i16 {
mir! {
type RET = i16;
let _5: char;
let _6: (u128, usize, i64, u128);
let _7: (i128, usize);
let _8: Adt54;
let _9: Adt51;
let _10: i8;
let _11: (i128, usize);
let _12: *mut bool;
let _13: isize;
let _14: Adt39;
let _15: Adt40;
let _16: (u64, [i128; 1], isize);
let _17: *const (u64, bool);
let _18: bool;
let _19: Adt51;
let _20: isize;
let _21: (u128, usize, i64, u128);
let _22: (u64, bool);
let _23: ();
let _24: ();
{
_1 = (-84_i8) as i16;
RET = 3_usize as i16;
_3 = 226_u8 as i32;
_5 = '\u{14bd3}';
Call(_6.1 = fn4(_4, _2, _2, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.1 = _6.1;
_7 = ((-76644459192749140070259868529301171471_i128), _6.1);
Goto(bb2)
}
bb2 = {
_6.1 = !_7.1;
_6.0 = 206557968253281404490805837207807852705_u128;
_5 = '\u{20426}';
_8.fld0 = 51762_u16 as i32;
_9.fld1.fld1 = _2 as f64;
_6.3 = _6.0 << _4;
_9.fld3 = 354670830_u32;
_9.fld1.fld2 = 2673262556671030653_u64 << _6.3;
_2 = _4 + _4;
match _6.0 {
0 => bb1,
206557968253281404490805837207807852705 => bb4,
_ => bb3
}
}
bb3 = {
_7.1 = _6.1;
_7 = ((-76644459192749140070259868529301171471_i128), _6.1);
Goto(bb2)
}
bb4 = {
_6 = (294628126275112913978592988659138305124_u128, _7.1, (-7444851657218090558_i64), 325846615989183040604831790459407734336_u128);
Call(_9.fld1.fld7 = core::intrinsics::bswap(_7.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11.0 = _7.0 | _7.0;
_9.fld1.fld4 = _7.1;
_6 = (117001211483378661293600529179275724617_u128, _9.fld1.fld4, 7311723012288480131_i64, 224845063270673441654658061659428777403_u128);
_8.fld0 = _3;
Call(_9.fld1.fld0 = fn5(_2, _9.fld1.fld1, _6.3, _2, _6.2, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = !_4;
_10 = !(-96_i8);
_4 = _1;
_11.1 = _6.3 as usize;
_10 = _11.1 as i8;
_6.1 = _11.1 - _11.1;
_11.0 = _7.0 - _7.0;
_6.0 = 9223372036854775807_isize as u128;
_6 = (56127455595707904228226449664166563703_u128, _11.1, 224265874647234444_i64, 154112097012490329017570196325528124815_u128);
_2 = _10 as i16;
_9.fld3 = _6.0 as u32;
_15.fld7 = _11.0;
_15.fld5 = _9.fld1.fld2 as i32;
_13 = (-9223372036854775808_isize);
_6.0 = _6.3 % _6.3;
_16.0 = _9.fld1.fld2 - _9.fld1.fld2;
_15.fld6 = _6.3 & _6.0;
_9.fld1.fld5 = _15.fld5;
_8.fld1.0 = core::ptr::addr_of_mut!(_16.1);
_9.fld1.fld3 = core::ptr::addr_of_mut!(_16.1);
_11.1 = _6.1;
_18 = _16.0 >= _9.fld1.fld2;
_16.1 = [_7.0];
Goto(bb7)
}
bb7 = {
_15.fld4.0 = _18;
_19.fld3 = _9.fld3;
_15.fld1 = _9.fld3;
_15.fld3 = [_15.fld7,_11.0];
_19.fld1.fld7 = _15.fld7 | _15.fld7;
_8.fld1.1 = core::ptr::addr_of!(_17);
_15.fld2.1 = _9.fld1.fld1;
RET = _9.fld1.fld1 as i16;
_11.1 = _6.1;
_19.fld1.fld1 = _19.fld1.fld7 as f64;
_9.fld1.fld3 = _8.fld1.0;
_8.fld1.0 = core::ptr::addr_of_mut!(_16.1);
_8.fld1.0 = _9.fld1.fld3;
_9.fld1.fld6 = 33_u8;
_9.fld1.fld7 = _15.fld7;
_19.fld1.fld5 = _9.fld1.fld5;
Goto(bb8)
}
bb8 = {
Call(_23 = dump_var(3_usize, 6_usize, Move(_6), 10_usize, Move(_10), 18_usize, Move(_18), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_23 = dump_var(3_usize, 11_usize, Move(_11), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16) -> usize {
mir! {
type RET = usize;
let _5: f32;
let _6: [i8; 5];
let _7: f64;
let _8: bool;
let _9: i128;
let _10: (u64, bool);
let _11: isize;
let _12: u64;
let _13: Adt38;
let _14: bool;
let _15: bool;
let _16: [i128; 2];
let _17: (f64, f64);
let _18: *mut [i128; 1];
let _19: isize;
let _20: Adt40;
let _21: (f64, f64);
let _22: (u64, bool);
let _23: i128;
let _24: (usize,);
let _25: [i128; 1];
let _26: Adt46;
let _27: u8;
let _28: [i128; 1];
let _29: (u64, [i128; 1], isize);
let _30: ();
let _31: ();
{
RET = !6_usize;
_4 = '\u{8ae07}' as i16;
_6 = [(-78_i8),(-98_i8),(-43_i8),92_i8,(-59_i8)];
_5 = (-154710505224620651590080234845048223472_i128) as f32;
_3 = !_2;
_1 = _3 ^ _3;
RET = 5_usize | 1631706539151143221_usize;
_2 = !_3;
_4 = 2688931757_u32 as i16;
RET = !4_usize;
_4 = -_3;
_2 = _1 & _1;
_3 = _4;
_7 = _4 as f64;
_5 = 184_u8 as f32;
Goto(bb1)
}
bb1 = {
_1 = !_3;
_2 = -_4;
_4 = _3 & _3;
_6 = [(-105_i8),2_i8,(-17_i8),(-5_i8),70_i8];
_8 = !false;
_4 = _1;
Goto(bb2)
}
bb2 = {
RET = 1493845314_i32 as usize;
_10.1 = !_8;
_10.0 = 2193759482193555284_u64 + 8397194368312065669_u64;
RET = 5741017859137255307_usize & 3_usize;
RET = 1_usize * 1059675597865691570_usize;
_4 = _1 - _1;
_11 = _7 as isize;
RET = !4_usize;
_9 = (-48623342536249804296071798408644485969_i128) | (-141430603085757714058473760470673843265_i128);
_3 = _2 >> _11;
_8 = _10.1;
_3 = _2;
_4 = !_1;
_9 = -(-66145064441605730163162987137267105023_i128);
_10 = Checked(16901275993308161500_u64 + 2763895112640802840_u64);
_6 = [54_i8,65_i8,(-11_i8),20_i8,47_i8];
_2 = _1;
_1 = _3 * _3;
_3 = _4;
RET = 2_usize & 12297818950230823219_usize;
_17 = (_7, _7);
Goto(bb3)
}
bb3 = {
_4 = _11 as i16;
_15 = !_10.1;
RET = _5 as usize;
_14 = _15;
_20.fld4.0 = _1 < _1;
_14 = _20.fld4.0;
_20.fld6 = 99366444184852999089233232643172776176_u128 << _1;
_15 = _20.fld4.0 | _20.fld4.0;
_10 = (6270346065288256252_u64, _14);
match _10.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
6270346065288256252 => bb11,
_ => bb10
}
}
bb4 = {
RET = 1493845314_i32 as usize;
_10.1 = !_8;
_10.0 = 2193759482193555284_u64 + 8397194368312065669_u64;
RET = 5741017859137255307_usize & 3_usize;
RET = 1_usize * 1059675597865691570_usize;
_4 = _1 - _1;
_11 = _7 as isize;
RET = !4_usize;
_9 = (-48623342536249804296071798408644485969_i128) | (-141430603085757714058473760470673843265_i128);
_3 = _2 >> _11;
_8 = _10.1;
_3 = _2;
_4 = !_1;
_9 = -(-66145064441605730163162987137267105023_i128);
_10 = Checked(16901275993308161500_u64 + 2763895112640802840_u64);
_6 = [54_i8,65_i8,(-11_i8),20_i8,47_i8];
_2 = _1;
_1 = _3 * _3;
_3 = _4;
RET = 2_usize & 12297818950230823219_usize;
_17 = (_7, _7);
Goto(bb3)
}
bb5 = {
_1 = !_3;
_2 = -_4;
_4 = _3 & _3;
_6 = [(-105_i8),2_i8,(-17_i8),(-5_i8),70_i8];
_8 = !false;
_4 = _1;
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
_12 = _10.0 << _11;
_20.fld3 = [_9,_9];
_22 = Checked(_10.0 * _12);
_22.1 = _15;
_20.fld2.1 = -_17.0;
_4 = _1;
_16 = [_9,_9];
_22.0 = _12;
_21.1 = _17.0;
_20.fld2.1 = 62704_u16 as f64;
_20.fld4.0 = !_15;
_17 = (_7, _21.1);
_14 = !_22.1;
_4 = _14 as i16;
_10.0 = 16121_u16 as u64;
_20.fld2.0 = _22.0 as f64;
_5 = RET as f32;
_20.fld6 = 41_u8 as u128;
_7 = _20.fld2.0 + _21.1;
_12 = _22.0 >> _1;
_20.fld2.0 = -_17.1;
_15 = _20.fld4.0;
_21.1 = -_17.1;
_3 = _2 * _4;
_9 = 165156260716127561753702995071734416430_i128 & 165630787023617591638003623764296322407_i128;
Goto(bb12)
}
bb12 = {
_20.fld3 = [_9,_9];
Goto(bb13)
}
bb13 = {
_23 = !_9;
_16 = _20.fld3;
_17.0 = _7 * _7;
_22 = _10;
_11 = !(-95_isize);
_12 = !_10.0;
_21 = _17;
_20.fld6 = 163455266361778353885849393232241234073_u128 + 124419946765964074821315814664148975939_u128;
_20.fld4 = (_10.1,);
_20.fld2 = (_7, _7);
_1 = -_3;
_18 = core::ptr::addr_of_mut!(_25);
_19 = -_11;
_10.0 = _12 | _12;
_6 = [(-27_i8),63_i8,(-12_i8),112_i8,13_i8];
_20.fld7 = _10.1 as i128;
_24.0 = RET;
_18 = core::ptr::addr_of_mut!((*_18));
_26 = Adt46::Variant0 { fld0: _22.0,fld1: _24.0 };
_15 = _22.1;
_11 = _19;
_27 = 201_u8 - 241_u8;
Goto(bb14)
}
bb14 = {
(*_18) = [_20.fld7];
_25 = [_20.fld7];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(4_usize, 12_usize, Move(_12), 4_usize, Move(_4), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(4_usize, 10_usize, Move(_10), 27_usize, Move(_27), 6_usize, Move(_6), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(4_usize, 9_usize, Move(_9), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i16,mut _2: f64,mut _3: u128,mut _4: i16,mut _5: i64,mut _6: i16,mut _7: i128) -> *mut (f64, f64) {
mir! {
type RET = *mut (f64, f64);
let _8: i8;
let _9: f32;
let _10: f32;
let _11: (usize,);
let _12: Adt40;
let _13: Adt52;
let _14: Adt48;
let _15: f32;
let _16: i128;
let _17: bool;
let _18: u128;
let _19: i32;
let _20: u128;
let _21: (f64, f64);
let _22: u128;
let _23: [u32; 1];
let _24: Adt53;
let _25: (i128, usize);
let _26: Adt49;
let _27: (f64, f64);
let _28: Adt46;
let _29: bool;
let _30: (i128, usize);
let _31: [i128; 1];
let _32: Adt46;
let _33: ();
let _34: ();
{
_7 = !135327551024136887688669406000621072780_i128;
_3 = 1393969194_i32 as u128;
Call(_6 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _6 ^ _6;
_7 = 149804747809005010821618335342562558627_i128 + 78027330134549705522456648470200803596_i128;
_8 = (-117_i8);
_4 = _1;
_6 = '\u{9f0fc}' as i16;
_4 = _1 ^ _1;
_1 = _4;
_7 = 163959305102280519046737418335526451344_i128;
_10 = _7 as f32;
_3 = 28993403723671968795244051896384727547_u128;
_6 = !_4;
_5 = -(-2909032140862327971_i64);
_11 = (6_usize,);
_9 = _10 + _10;
_5 = (-6425281081908868890_i64);
_8 = !10_i8;
_4 = _6 >> _6;
_11.0 = !7_usize;
_7 = !144992240381541112271100156441324518270_i128;
_11.0 = false as usize;
_8 = 124_i8;
_10 = 59383939485656716_u64 as f32;
_8 = (-71_i8);
_3 = !10877940390553213900132803276173930088_u128;
_2 = 86877565_i32 as f64;
_12.fld3 = [_7,_7];
_12.fld4.0 = false;
Goto(bb2)
}
bb2 = {
_6 = !_1;
_11.0 = !3269948680344479995_usize;
_12.fld2.1 = _2;
_7 = 50150965761647849992053701850527551549_i128 >> _6;
_12.fld2.1 = 35545_u16 as f64;
_12.fld2 = (_2, _2);
_12.fld4.0 = !false;
_4 = !_6;
_4 = _1;
RET = core::ptr::addr_of_mut!(_12.fld2);
(*RET).1 = 1286330758_u32 as f64;
_5 = 98912737_u32 as i64;
_10 = _9;
Goto(bb3)
}
bb3 = {
_15 = _5 as f32;
Call(_4 = fn6(_1, _6, _1, _6, _7, _6, _1, _7, _7, _7, _3, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET).1 = (*RET).0;
_12.fld1 = 3947301674_u32 ^ 4291004987_u32;
_12.fld5 = (-160413069_i32) * (-342961453_i32);
(*RET) = (_2, _2);
_1 = -_6;
_8 = _7 as i8;
_18 = _9 as u128;
_11 = (1_usize,);
_4 = _6;
(*RET).1 = -(*RET).0;
_12.fld7 = _7 - _7;
(*RET).1 = _2;
_12.fld2 = (_2, _2);
(*RET) = (_2, _2);
_7 = !_12.fld7;
(*RET).0 = (*RET).1 + _2;
_1 = _12.fld4.0 as i16;
(*RET).0 = _11.0 as f64;
Goto(bb5)
}
bb5 = {
(*RET).0 = (*RET).1 - _12.fld2.1;
_12.fld1 = !3148330683_u32;
_2 = (*RET).0 - (*RET).0;
Call(_12.fld4.0 = fn7(_6, _8, (*RET).1, _4, _12.fld7, _7, _6, _8, _7, _12.fld7, _7, _8, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = !_12.fld7;
_4 = _6;
RET = core::ptr::addr_of_mut!((*RET));
_19 = _12.fld5 >> _18;
RET = core::ptr::addr_of_mut!((*RET));
_2 = (*RET).1;
(*RET).0 = (*RET).1 * _12.fld2.1;
_10 = _15 + _15;
match _11.0 {
0 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
1 => bb12,
_ => bb11
}
}
bb7 = {
(*RET).0 = (*RET).1 - _12.fld2.1;
_12.fld1 = !3148330683_u32;
_2 = (*RET).0 - (*RET).0;
Call(_12.fld4.0 = fn7(_6, _8, (*RET).1, _4, _12.fld7, _7, _6, _8, _7, _12.fld7, _7, _8, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET).1 = (*RET).0;
_12.fld1 = 3947301674_u32 ^ 4291004987_u32;
_12.fld5 = (-160413069_i32) * (-342961453_i32);
(*RET) = (_2, _2);
_1 = -_6;
_8 = _7 as i8;
_18 = _9 as u128;
_11 = (1_usize,);
_4 = _6;
(*RET).1 = -(*RET).0;
_12.fld7 = _7 - _7;
(*RET).1 = _2;
_12.fld2 = (_2, _2);
(*RET) = (_2, _2);
_7 = !_12.fld7;
(*RET).0 = (*RET).1 + _2;
_1 = _12.fld4.0 as i16;
(*RET).0 = _11.0 as f64;
Goto(bb5)
}
bb9 = {
_15 = _5 as f32;
Call(_4 = fn6(_1, _6, _1, _6, _7, _6, _1, _7, _7, _7, _3, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_6 = !_1;
_11.0 = !3269948680344479995_usize;
_12.fld2.1 = _2;
_7 = 50150965761647849992053701850527551549_i128 >> _6;
_12.fld2.1 = 35545_u16 as f64;
_12.fld2 = (_2, _2);
_12.fld4.0 = !false;
_4 = !_6;
_4 = _1;
RET = core::ptr::addr_of_mut!(_12.fld2);
(*RET).1 = 1286330758_u32 as f64;
_5 = 98912737_u32 as i64;
_10 = _9;
Goto(bb3)
}
bb11 = {
_4 = _6 ^ _6;
_7 = 149804747809005010821618335342562558627_i128 + 78027330134549705522456648470200803596_i128;
_8 = (-117_i8);
_4 = _1;
_6 = '\u{9f0fc}' as i16;
_4 = _1 ^ _1;
_1 = _4;
_7 = 163959305102280519046737418335526451344_i128;
_10 = _7 as f32;
_3 = 28993403723671968795244051896384727547_u128;
_6 = !_4;
_5 = -(-2909032140862327971_i64);
_11 = (6_usize,);
_9 = _10 + _10;
_5 = (-6425281081908868890_i64);
_8 = !10_i8;
_4 = _6 >> _6;
_11.0 = !7_usize;
_7 = !144992240381541112271100156441324518270_i128;
_11.0 = false as usize;
_8 = 124_i8;
_10 = 59383939485656716_u64 as f32;
_8 = (-71_i8);
_3 = !10877940390553213900132803276173930088_u128;
_2 = 86877565_i32 as f64;
_12.fld3 = [_7,_7];
_12.fld4.0 = false;
Goto(bb2)
}
bb12 = {
_23 = [_12.fld1];
_12.fld2.0 = _11.0 as f64;
_21.1 = -(*RET).1;
(*RET) = (_21.1, _21.1);
(*RET).1 = (*RET).0;
_22 = _18 + _18;
_20 = '\u{c62db}' as u128;
_12.fld6 = _22;
_18 = 193_u8 as u128;
_21.0 = _12.fld2.0 * _21.1;
(*RET).0 = (*RET).1 + _21.1;
_26.fld0.0 = !8165713905465441346_u64;
match _11.0 {
0 => bb13,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
1 => bb20,
_ => bb19
}
}
bb13 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET).1 = (*RET).0;
_12.fld1 = 3947301674_u32 ^ 4291004987_u32;
_12.fld5 = (-160413069_i32) * (-342961453_i32);
(*RET) = (_2, _2);
_1 = -_6;
_8 = _7 as i8;
_18 = _9 as u128;
_11 = (1_usize,);
_4 = _6;
(*RET).1 = -(*RET).0;
_12.fld7 = _7 - _7;
(*RET).1 = _2;
_12.fld2 = (_2, _2);
(*RET) = (_2, _2);
_7 = !_12.fld7;
(*RET).0 = (*RET).1 + _2;
_1 = _12.fld4.0 as i16;
(*RET).0 = _11.0 as f64;
Goto(bb5)
}
bb14 = {
_6 = !_1;
_11.0 = !3269948680344479995_usize;
_12.fld2.1 = _2;
_7 = 50150965761647849992053701850527551549_i128 >> _6;
_12.fld2.1 = 35545_u16 as f64;
_12.fld2 = (_2, _2);
_12.fld4.0 = !false;
_4 = !_6;
_4 = _1;
RET = core::ptr::addr_of_mut!(_12.fld2);
(*RET).1 = 1286330758_u32 as f64;
_5 = 98912737_u32 as i64;
_10 = _9;
Goto(bb3)
}
bb15 = {
_15 = _5 as f32;
Call(_4 = fn6(_1, _6, _1, _6, _7, _6, _1, _7, _7, _7, _3, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET).1 = (*RET).0;
_12.fld1 = 3947301674_u32 ^ 4291004987_u32;
_12.fld5 = (-160413069_i32) * (-342961453_i32);
(*RET) = (_2, _2);
_1 = -_6;
_8 = _7 as i8;
_18 = _9 as u128;
_11 = (1_usize,);
_4 = _6;
(*RET).1 = -(*RET).0;
_12.fld7 = _7 - _7;
(*RET).1 = _2;
_12.fld2 = (_2, _2);
(*RET) = (_2, _2);
_7 = !_12.fld7;
(*RET).0 = (*RET).1 + _2;
_1 = _12.fld4.0 as i16;
(*RET).0 = _11.0 as f64;
Goto(bb5)
}
bb17 = {
(*RET).0 = (*RET).1 - _12.fld2.1;
_12.fld1 = !3148330683_u32;
_2 = (*RET).0 - (*RET).0;
Call(_12.fld4.0 = fn7(_6, _8, (*RET).1, _4, _12.fld7, _7, _6, _8, _7, _12.fld7, _7, _8, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb18 = {
_4 = _6 ^ _6;
_7 = 149804747809005010821618335342562558627_i128 + 78027330134549705522456648470200803596_i128;
_8 = (-117_i8);
_4 = _1;
_6 = '\u{9f0fc}' as i16;
_4 = _1 ^ _1;
_1 = _4;
_7 = 163959305102280519046737418335526451344_i128;
_10 = _7 as f32;
_3 = 28993403723671968795244051896384727547_u128;
_6 = !_4;
_5 = -(-2909032140862327971_i64);
_11 = (6_usize,);
_9 = _10 + _10;
_5 = (-6425281081908868890_i64);
_8 = !10_i8;
_4 = _6 >> _6;
_11.0 = !7_usize;
_7 = !144992240381541112271100156441324518270_i128;
_11.0 = false as usize;
_8 = 124_i8;
_10 = 59383939485656716_u64 as f32;
_8 = (-71_i8);
_3 = !10877940390553213900132803276173930088_u128;
_2 = 86877565_i32 as f64;
_12.fld3 = [_7,_7];
_12.fld4.0 = false;
Goto(bb2)
}
bb19 = {
(*RET).0 = (*RET).1 - _12.fld2.1;
_12.fld1 = !3148330683_u32;
_2 = (*RET).0 - (*RET).0;
Call(_12.fld4.0 = fn7(_6, _8, (*RET).1, _4, _12.fld7, _7, _6, _8, _7, _12.fld7, _7, _8, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb20 = {
_29 = !_12.fld4.0;
_17 = (*RET).0 < _12.fld2.1;
_12.fld3 = [_12.fld7,_7];
_12.fld2 = _21;
_12.fld4 = (_29,);
_27 = ((*RET).0, (*RET).1);
_26.fld2 = Adt38::Variant0 { fld0: _9,fld1: _11 };
place!(Field::<(usize,)>(Variant(_26.fld2, 0), 1)).0 = !_11.0;
Goto(bb21)
}
bb21 = {
Call(_33 = dump_var(5_usize, 1_usize, Move(_1), 22_usize, Move(_22), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(5_usize, 8_usize, Move(_8), 23_usize, Move(_23), 4_usize, Move(_4), 29_usize, Move(_29)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i128,mut _6: i16,mut _7: i16,mut _8: i128,mut _9: i128,mut _10: i128,mut _11: u128,mut _12: i16) -> i16 {
mir! {
type RET = i16;
let _13: u16;
let _14: Adt44;
let _15: Adt48;
let _16: ();
let _17: ();
{
_12 = _7 * _6;
RET = _4 << _3;
_12 = _3 ^ _1;
_7 = 18_isize as i16;
_11 = '\u{679e8}' as u128;
_3 = -_7;
_7 = 11398547029722044469_usize as i16;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(6_usize, 2_usize, Move(_2), 10_usize, Move(_10), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(6_usize, 5_usize, Move(_5), 3_usize, Move(_3), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i16,mut _2: i8,mut _3: f64,mut _4: i16,mut _5: i128,mut _6: i128,mut _7: i16,mut _8: i8,mut _9: i128,mut _10: i128,mut _11: i128,mut _12: i8,mut _13: i8) -> bool {
mir! {
type RET = bool;
let _14: u32;
let _15: (u64, [i128; 1], isize);
let _16: u64;
let _17: *mut bool;
let _18: [i8; 5];
let _19: ();
let _20: ();
{
_10 = false as i128;
RET = _6 >= _6;
_8 = _13;
_11 = _6;
_5 = _11 >> _4;
_5 = 13347135080340064192_usize as i128;
RET = false;
_1 = !_7;
_5 = _6 | _9;
_14 = 1316434709_u32;
_15.0 = 2640142719772295103_u64;
_7 = _1;
_12 = _8 + _8;
_3 = _4 as f64;
_6 = _9 | _5;
_16 = !_15.0;
_17 = core::ptr::addr_of_mut!(RET);
_7 = _3 as i16;
_9 = _5;
_12 = -_8;
_16 = _15.0;
_15.1 = [_6];
_16 = _15.0;
Goto(bb1)
}
bb1 = {
_1 = _4;
Call(_15.0 = fn8(_3, _15.1, _3, _15.1, _12, _2, _5, _13, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _1 ^ _7;
_13 = _12;
_11 = _5;
RET = true;
_13 = _4 as i8;
_6 = _9 - _9;
_8 = RET as i8;
_8 = _12;
_12 = _2 - _13;
_6 = _3 as i128;
(*_17) = _7 < _4;
_15.2 = 55_isize | 116_isize;
_18 = [_13,_12,_8,_2,_13];
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(7_usize, 5_usize, Move(_5), 10_usize, Move(_10), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(7_usize, 15_usize, Move(_15), 1_usize, Move(_1), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: f64,mut _2: [i128; 1],mut _3: f64,mut _4: [i128; 1],mut _5: i8,mut _6: i8,mut _7: i128,mut _8: i8,mut _9: i16) -> u64 {
mir! {
type RET = u64;
let _10: i8;
let _11: (u64, [i128; 1], isize);
let _12: u8;
let _13: Adt49;
let _14: isize;
let _15: (u64, [i128; 1], isize);
let _16: *mut isize;
let _17: char;
let _18: &'static u16;
let _19: [i128; 2];
let _20: ();
let _21: ();
{
_8 = _5;
_6 = !_8;
_2 = [_7];
RET = !6267784983586675800_u64;
_8 = -_6;
_9 = !13143_i16;
_1 = _3;
_4 = _2;
_10 = !_5;
RET = 276163038_u32 as u64;
_6 = !_5;
_6 = _8 >> _10;
_11.1 = [_7];
_11.0 = _1 as u64;
_11.1 = [_7];
_11 = (RET, _4, 9223372036854775807_isize);
_3 = _1;
RET = 1809017755_u32 as u64;
_9 = 5_usize as i16;
_7 = (-17168215084482626399752771384501151086_i128) & 69551762290176049279610336704536105804_i128;
RET = !_11.0;
Goto(bb1)
}
bb1 = {
_3 = _7 as f64;
_12 = 152_u8 >> _8;
_3 = _1;
_5 = _12 as i8;
_9 = 201695629432895780417859097203324913081_u128 as i16;
Call(_4 = fn9(_8, _11, _11.1, _5, _11.2, _5, _11.2, _2, _11, _2, _8, _3, _8, _8, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13.fld0.1 = false;
_13.fld0 = (RET, false);
_13.fld0.0 = _11.2 as u64;
_11.0 = _13.fld0.0;
Goto(bb3)
}
bb3 = {
_2 = _11.1;
_13.fld1 = 0_usize - 7_usize;
_11.0 = _13.fld0.0 >> _13.fld0.0;
_14 = _1 as isize;
RET = !_13.fld0.0;
_13.fld0.1 = _6 <= _10;
_4 = _11.1;
RET = !_13.fld0.0;
_13.fld0.1 = !false;
_11.1 = [_7];
_14 = _11.2 & _11.2;
_14 = -_11.2;
_11.2 = _14 * _14;
_15.0 = _11.0;
_15 = _11;
_9 = RET as i16;
_15.2 = _11.2;
_6 = !_10;
Goto(bb4)
}
bb4 = {
Call(_20 = dump_var(8_usize, 12_usize, Move(_12), 4_usize, Move(_4), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_20 = dump_var(8_usize, 7_usize, Move(_7), 5_usize, Move(_5), 21_usize, _21, 21_usize, _21), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i8,mut _2: (u64, [i128; 1], isize),mut _3: [i128; 1],mut _4: i8,mut _5: isize,mut _6: i8,mut _7: isize,mut _8: [i128; 1],mut _9: (u64, [i128; 1], isize),mut _10: [i128; 1],mut _11: i8,mut _12: f64,mut _13: i8,mut _14: i8,mut _15: i8) -> [i128; 1] {
mir! {
type RET = [i128; 1];
let _16: u64;
let _17: [i128; 1];
let _18: f32;
let _19: i64;
let _20: char;
let _21: (usize,);
let _22: u8;
let _23: (u64, bool);
let _24: Adt38;
let _25: f64;
let _26: i8;
let _27: bool;
let _28: Adt50;
let _29: ();
let _30: ();
{
_2.0 = _9.0;
RET = _3;
_2.2 = _9.2;
_9.1 = [(-116396293520393303144844447837549571346_i128)];
_9.2 = _7;
_15 = -_13;
_7 = _9.2 << _15;
_9.2 = _7 + _7;
RET = [2218719504658727942917014954093275291_i128];
_13 = -_15;
_9.2 = _2.2;
_3 = _10;
_14 = _4 | _4;
RET = [(-117432033501945849032135050885839340172_i128)];
_11 = !_1;
_3 = [146638575733505628156409966217472350488_i128];
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9223372036854775807 => bb6,
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
_2.0 = _9.0 << _9.2;
_5 = _12 as isize;
_16 = 1789654753_i32 as u64;
_14 = _4 - _15;
RET = [115106830007565893134920096373174729003_i128];
_9.0 = _2.0 | _2.0;
_2.2 = _9.2;
_2.1 = [91258448483143221304420249071262694749_i128];
RET = [90282377854942873660294009236471503728_i128];
_14 = _15;
_10 = [101335800146115319444081411974860063047_i128];
_2.0 = 113781275631081292387453755372880672363_i128 as u64;
_9.2 = _5 << _2.2;
_7 = (-1843050633798446736_i64) as isize;
_1 = _14;
_18 = _12 as f32;
_5 = _9.2;
_17 = _8;
_2.1 = [(-11003117766779997205473885534956342168_i128)];
_9.1 = [137554379148746785037034827373562682546_i128];
_2.0 = _9.0 << _13;
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
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
_5 = 10700140685952261950_usize as isize;
RET = [(-134365205796362340154409741209599582590_i128)];
_5 = _2.2 << _9.2;
_15 = !_14;
_17 = _8;
_19 = -3174923572240757499_i64;
_14 = 23341_u16 as i8;
Call(_21 = fn10(_12, _2.2, _12, _2, _6, _17), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4 = _2.2 as i8;
_8 = [6881405703833000080421654802472897807_i128];
_16 = _9.0 >> _6;
Call(RET = fn11(_9.2, _13), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = [(-68232789499316618011082352598389925586_i128)];
_9 = (_2.0, _17, _5);
_15 = -_13;
RET = [41868911623635004732618152874416715411_i128];
_12 = 178_u8 as f64;
_23 = (_2.0, false);
_14 = !_6;
_4 = _13 + _14;
_9.2 = !_5;
_19 = (-6235758357775586349_i64) ^ 4479018073305401493_i64;
Goto(bb13)
}
bb13 = {
_10 = _17;
_14 = _1 + _6;
_12 = 114432269421539928796438476394076175996_i128 as f64;
_6 = _23.1 as i8;
_20 = '\u{a8063}';
_2.0 = _16 << _13;
_23.0 = _16 * _2.0;
_5 = _2.2;
_6 = !_4;
_21.0 = !3945973508689761176_usize;
_2.1 = [113396053681274722692077483189488057319_i128];
_8 = [(-4729142232426813116985443131023418878_i128)];
_23.1 = !false;
_11 = _14;
_8 = [46289610695238407177953331889765943135_i128];
_26 = -_6;
_25 = 1078455039_u32 as f64;
_2 = (_23.0, _10, _5);
_1 = _18 as i8;
_16 = _21.0 as u64;
_21.0 = 16357341734470411009_usize >> _1;
_3 = [(-36390965000816336296017163290355940494_i128)];
_9 = (_23.0, _17, _5);
_18 = 22872_u16 as f32;
_19 = 1733487820_i32 as i64;
Call(_9.2 = fn12(_9.1, _2, _13, _2.2, _26, _1, _10, _5, _6, _23.0, _9.1, _21.0, _21.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_16 = !_2.0;
_2.1 = _9.1;
_11 = _15 | _6;
_9 = _2;
_7 = 212919931890617125639941562028747292608_u128 as isize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(9_usize, 23_usize, Move(_23), 8_usize, Move(_8), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(9_usize, 20_usize, Move(_20), 1_usize, Move(_1), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(9_usize, 10_usize, Move(_10), 14_usize, Move(_14), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f64,mut _2: isize,mut _3: f64,mut _4: (u64, [i128; 1], isize),mut _5: i8,mut _6: [i128; 1]) -> (usize,) {
mir! {
type RET = (usize,);
let _7: i8;
let _8: [i128; 2];
let _9: isize;
let _10: (f64, f64);
let _11: ();
let _12: ();
{
RET.0 = 17569854887613212212_usize - 6_usize;
_4.0 = 83549593885150913_u64 << _5;
_4.0 = (-106601857_i32) as u64;
_6 = [151159717243651351194233693421908752466_i128];
_6 = [(-36603734106523298305022953799284454179_i128)];
match _2 {
0 => bb1,
9223372036854775807 => bb3,
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
_3 = _1 + _1;
_8 = [(-39979886098395695322762966470686246580_i128),(-57174966130393707046877131982727006524_i128)];
_2 = -_4.2;
_3 = _1 + _1;
_2 = -_4.2;
RET = (1775940863018076237_usize,);
match _4.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
9223372036854775807 => bb9,
_ => bb8
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
Return()
}
bb9 = {
RET.0 = !13652787008811531053_usize;
RET.0 = 12268664605590365454_usize >> _5;
_2 = 108625130473627197651505958972545137325_u128 as isize;
_10.1 = _5 as f64;
_2 = _4.2;
RET.0 = _2 as usize;
Goto(bb10)
}
bb10 = {
Call(_11 = dump_var(10_usize, 2_usize, Move(_2), 6_usize, Move(_6), 12_usize, _12, 12_usize, _12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: i8) -> [i128; 1] {
mir! {
type RET = [i128; 1];
let _3: bool;
let _4: (f64, f64);
let _5: Adt44;
let _6: u32;
let _7: (f64, f64);
let _8: Adt41;
let _9: f32;
let _10: (usize,);
let _11: (usize,);
let _12: Adt45;
let _13: &'static u16;
let _14: [i128; 2];
let _15: (u128, usize, i64, u128);
let _16: i32;
let _17: f32;
let _18: (u128, usize, i64, u128);
let _19: (usize,);
let _20: f64;
let _21: f64;
let _22: &'static u16;
let _23: [i128; 2];
let _24: u16;
let _25: usize;
let _26: ((f64, f64), [i128; 2], (f64, f64));
let _27: (u64, [i128; 1], isize);
let _28: u64;
let _29: ();
let _30: ();
{
RET = [160536984513987462580715227395047408220_i128];
RET = [(-37759382968444756622279182688362301364_i128)];
_2 = 12_i8 - 114_i8;
RET = [(-49650122406664561337800654616772010716_i128)];
RET = [(-157779780583157852651776830841415867736_i128)];
_2 = (-39_i8);
RET = [92032134335438218412142627421312095665_i128];
RET = [(-91585930180506812798290837693232017394_i128)];
RET = [165853073289109856841045198397069755705_i128];
_1 = !9223372036854775807_isize;
_2 = (-32_i8);
_2 = !19_i8;
_2 = 117_i8;
RET = [(-95574119943531717548651596201435776048_i128)];
RET = [121456525545023721276828190982044729577_i128];
match _2 {
0 => bb1,
117 => bb3,
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
RET = [(-95382276858109823317680265426177824082_i128)];
_3 = !true;
_3 = _2 > _2;
_2 = -(-102_i8);
RET = [75541084616411530927772969094855285348_i128];
RET = [102837887846521792147922803316223169472_i128];
RET = [(-71381586480428553831505001208111529773_i128)];
_3 = !false;
RET = [44194814835472871476967106162035616949_i128];
_2 = 66_i8;
_2 = (-120_i8) + 97_i8;
_1 = !9223372036854775807_isize;
_1 = !9223372036854775807_isize;
RET = [61382573777325573919571636859570310596_i128];
_2 = (-2_i8) << _1;
_4.0 = _1 as f64;
_2 = 114_i8 & 0_i8;
RET = [37365047970681752100565187683633622559_i128];
_4.0 = 3_usize as f64;
_2 = (-71_i8) & 30_i8;
Call(_1 = core::intrinsics::bswap(21_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4.1 = _4.0 - _4.0;
_2 = 101_i8 - (-13_i8);
_6 = !4146103460_u32;
_4.1 = _4.0 * _4.0;
_6 = !3481861195_u32;
_7.1 = _4.1 * _4.1;
RET = [22397350941277018924416108065118260178_i128];
_7.1 = -_4.1;
_4.1 = _4.0;
_3 = false;
Goto(bb5)
}
bb5 = {
_7.0 = _7.1;
_7.0 = _4.1 - _7.1;
_8.fld1 = 5_usize as f64;
_8.fld7 = -(-46328451479452347551909204377773509783_i128);
_9 = _8.fld7 as f32;
_8.fld2 = 4203897619721147735_u64;
_2 = (-40_i8) >> _8.fld2;
_8.fld1 = -_4.0;
_4 = (_7.1, _7.0);
_8.fld7 = (-80752131286291018810526785784880412023_i128) * 78090292334249902173238481698304492042_i128;
_8.fld3 = core::ptr::addr_of_mut!(RET);
_4 = (_7.0, _7.1);
_11.0 = _4.1 as usize;
_2 = _9 as i8;
_4.0 = _7.0 - _7.0;
_8.fld4 = !_11.0;
match _8.fld2 {
0 => bb4,
4203897619721147735 => bb6,
_ => bb2
}
}
bb6 = {
_4 = (_7.0, _7.0);
_7 = (_4.0, _4.1);
_4 = (_7.1, _7.1);
_1 = _11.0 as isize;
_8.fld5 = !(-199312012_i32);
_11 = (_8.fld4,);
_1 = (-9223372036854775808_isize);
_8.fld5 = _8.fld7 as i32;
_10 = _11;
_8.fld1 = _8.fld7 as f64;
_8.fld4 = _8.fld7 as usize;
_8.fld7 = 151821848052303063529198322206808840399_i128;
_6 = !3299554358_u32;
_7 = _4;
_15 = (198749913819188630405172324308470926957_u128, _10.0, 8231674843881662637_i64, 92563843280695279136433893135138203630_u128);
RET = [_8.fld7];
_8.fld0 = core::ptr::addr_of_mut!(_7);
_10.0 = !_11.0;
_15.0 = _15.3 << _10.0;
_8.fld6 = _6 as u8;
_15.1 = !_8.fld4;
_5 = Adt44::Variant0 { fld0: RET };
SetDiscriminant(_5, 1);
_4 = (_7.1, _7.1);
Call(_8.fld1 = core::intrinsics::fmaf64(_4.0, _7.1, _7.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = [_8.fld7,_8.fld7];
_8.fld2 = !14077146936548925417_u64;
_8.fld3 = core::ptr::addr_of_mut!(RET);
_4 = (_7.0, _8.fld1);
_8.fld4 = '\u{10aa0d}' as usize;
_17 = _9;
_2 = 88_i8 << _10.0;
Goto(bb8)
}
bb8 = {
_20 = _7.0;
_18.0 = _15.3 - _15.0;
_15.0 = _18.0 << _18.0;
_18 = (_15.0, _10.0, _15.2, _15.0);
_8.fld4 = _18.3 as usize;
_3 = false;
_18.0 = _15.3 + _18.3;
_18.3 = _9 as u128;
_18.0 = !_15.0;
_11.0 = _8.fld4 ^ _8.fld4;
_21 = _7.0 * _20;
_18.0 = _15.0;
RET = [_8.fld7];
match _15.2 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
8231674843881662637 => bb13,
_ => bb12
}
}
bb9 = {
Return()
}
bb10 = {
_4 = (_7.0, _7.0);
_7 = (_4.0, _4.1);
_4 = (_7.1, _7.1);
_1 = _11.0 as isize;
_8.fld5 = !(-199312012_i32);
_11 = (_8.fld4,);
_1 = (-9223372036854775808_isize);
_8.fld5 = _8.fld7 as i32;
_10 = _11;
_8.fld1 = _8.fld7 as f64;
_8.fld4 = _8.fld7 as usize;
_8.fld7 = 151821848052303063529198322206808840399_i128;
_6 = !3299554358_u32;
_7 = _4;
_15 = (198749913819188630405172324308470926957_u128, _10.0, 8231674843881662637_i64, 92563843280695279136433893135138203630_u128);
RET = [_8.fld7];
_8.fld0 = core::ptr::addr_of_mut!(_7);
_10.0 = !_11.0;
_15.0 = _15.3 << _10.0;
_8.fld6 = _6 as u8;
_15.1 = !_8.fld4;
_5 = Adt44::Variant0 { fld0: RET };
SetDiscriminant(_5, 1);
_4 = (_7.1, _7.1);
Call(_8.fld1 = core::intrinsics::fmaf64(_4.0, _7.1, _7.0), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_7.0 = _7.1;
_7.0 = _4.1 - _7.1;
_8.fld1 = 5_usize as f64;
_8.fld7 = -(-46328451479452347551909204377773509783_i128);
_9 = _8.fld7 as f32;
_8.fld2 = 4203897619721147735_u64;
_2 = (-40_i8) >> _8.fld2;
_8.fld1 = -_4.0;
_4 = (_7.1, _7.0);
_8.fld7 = (-80752131286291018810526785784880412023_i128) * 78090292334249902173238481698304492042_i128;
_8.fld3 = core::ptr::addr_of_mut!(RET);
_4 = (_7.0, _7.1);
_11.0 = _4.1 as usize;
_2 = _9 as i8;
_4.0 = _7.0 - _7.0;
_8.fld4 = !_11.0;
match _8.fld2 {
0 => bb4,
4203897619721147735 => bb6,
_ => bb2
}
}
bb12 = {
_4.1 = _4.0 - _4.0;
_2 = 101_i8 - (-13_i8);
_6 = !4146103460_u32;
_4.1 = _4.0 * _4.0;
_6 = !3481861195_u32;
_7.1 = _4.1 * _4.1;
RET = [22397350941277018924416108065118260178_i128];
_7.1 = -_4.1;
_4.1 = _4.0;
_3 = false;
Goto(bb5)
}
bb13 = {
Call(_7.0 = core::intrinsics::transmute(_18.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_4 = (_21, _21);
_17 = _9 + _9;
_15.3 = _18.0;
_22 = &_24;
_18.0 = _15.3;
_18 = (_15.0, _8.fld4, _15.2, _15.3);
_4 = (_20, _7.0);
_8.fld2 = !5894063709222562376_u64;
RET = [_8.fld7];
_19 = (_11.0,);
_13 = &_24;
_26.2.1 = _21;
_11.0 = _19.0;
_15 = _18;
_25 = _18.1 - _18.1;
_14 = [_8.fld7,_8.fld7];
_26.0.0 = -_7.0;
_27.2 = !_1;
_26.2.1 = -_21;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(11_usize, 25_usize, Move(_25), 11_usize, Move(_11), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(11_usize, 14_usize, Move(_14), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [i128; 1],mut _2: (u64, [i128; 1], isize),mut _3: i8,mut _4: isize,mut _5: i8,mut _6: i8,mut _7: [i128; 1],mut _8: isize,mut _9: i8,mut _10: u64,mut _11: [i128; 1],mut _12: usize,mut _13: usize) -> isize {
mir! {
type RET = isize;
let _14: *mut (f64, f64);
let _15: (i128, usize);
let _16: ();
let _17: ();
{
_7 = [(-48344897794803667883976675980391496234_i128)];
_10 = _2.0 * _2.0;
_13 = 672454177_i32 as usize;
_2.1 = _11;
_7 = _2.1;
RET = _2.2 - _8;
_11 = [159443451757365954762320477951383277993_i128];
_2.0 = !_10;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(12_usize, 12_usize, Move(_12), 11_usize, Move(_11), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(12_usize, 10_usize, Move(_10), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i32,mut _2: i16,mut _3: i16,mut _4: i32) -> i32 {
mir! {
type RET = i32;
let _5: [i128; 1];
let _6: (usize,);
let _7: bool;
let _8: (u64, bool);
let _9: [i8; 5];
let _10: (f64, f64);
let _11: *mut char;
let _12: f32;
let _13: *mut (f64, f64);
let _14: (f64, f64);
let _15: ();
let _16: ();
{
_2 = 9108587538156962340_u64 as i16;
RET = _4;
_2 = 1462_u16 as i16;
RET = 3964887685_u32 as i32;
RET = -_4;
_4 = RET;
_5 = [87308784660906928706709809233410063623_i128];
_3 = -_2;
RET = _4;
_3 = _2 >> _4;
_4 = -RET;
Goto(bb1)
}
bb1 = {
_3 = 1368047385_u32 as i16;
_6 = (2_usize,);
RET = _4 * _1;
_8 = (2288363631638586019_u64, true);
_1 = _4 - RET;
_9 = [109_i8,(-103_i8),(-6_i8),37_i8,(-91_i8)];
_6.0 = 0_usize;
_8.0 = !12228070221362161696_u64;
Goto(bb2)
}
bb2 = {
_4 = _1;
_10.1 = _8.0 as f64;
Call(_3 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.1 = (-135996782700295344122963329891394187538_i128) as f64;
_6 = (0_usize,);
_4 = _1;
RET = -_1;
_8 = (6186529855993933091_u64, false);
_9 = [79_i8,(-95_i8),106_i8,(-42_i8),45_i8];
_10.0 = 5_i8 as f64;
_10.1 = _10.0 * _10.0;
_4 = RET >> _6.0;
_6 = (2_usize,);
_9 = [(-97_i8),53_i8,(-40_i8),53_i8,(-76_i8)];
_6.0 = !3_usize;
_3 = _2 >> _1;
_2 = !_3;
_14.1 = _10.0;
_4 = _1 * _1;
_6 = (3123880378733706716_usize,);
_9 = [122_i8,124_i8,107_i8,118_i8,(-125_i8)];
_1 = _4 * RET;
_6 = (1_usize,);
Goto(bb4)
}
bb4 = {
Call(_15 = dump_var(13_usize, 6_usize, Move(_6), 1_usize, Move(_1), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16) -> f64 {
mir! {
type RET = f64;
let _7: Adt46;
let _8: char;
let _9: [u32; 1];
let _10: Adt41;
let _11: Adt54;
let _12: f64;
let _13: i64;
let _14: (usize,);
let _15: f64;
let _16: [i128; 1];
let _17: ((f64, f64), [i128; 2], (f64, f64));
let _18: (usize,);
let _19: (bool,);
let _20: Adt45;
let _21: ();
let _22: ();
{
_1 = -_2;
_4 = _1;
_4 = -_1;
_4 = _1 & _2;
Goto(bb1)
}
bb1 = {
_8 = '\u{55ce0}';
_2 = _4;
RET = 47722293972878341072659091818428247888_u128 as f64;
_8 = '\u{b9f45}';
_3 = 209_u8 as i16;
Goto(bb2)
}
bb2 = {
_9 = [515740110_u32];
_2 = _4 - _1;
_6 = _5 & _4;
_5 = !_1;
_5 = _6 * _2;
_7 = Adt46::Variant0 { fld0: 18211277595569982173_u64,fld1: 3_usize };
_6 = !_4;
_10.fld6 = 120_u8;
_10.fld5 = 97520680_i32;
_10.fld6 = !53_u8;
match _10.fld5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
97520680 => bb8,
_ => bb7
}
}
bb3 = {
_8 = '\u{55ce0}';
_2 = _4;
RET = 47722293972878341072659091818428247888_u128 as f64;
_8 = '\u{b9f45}';
_3 = 209_u8 as i16;
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
_10.fld7 = (-162739760676646235760904210701968516782_i128) | 112971443449439767949445992045602981525_i128;
_10.fld4 = 2438218481109779803_usize;
_3 = true as i16;
place!(Field::<u64>(Variant(_7, 0), 0)) = 11074991108328050203_u64;
_10.fld2 = 289284712421127373217677319356144500861_u128 as u64;
place!(Field::<u64>(Variant(_7, 0), 0)) = _10.fld2 & _10.fld2;
_5 = 7269591804410898453_i64 as i16;
_11.fld0 = _10.fld5;
place!(Field::<usize>(Variant(_7, 0), 1)) = _10.fld7 as usize;
_2 = 3424305826_u32 as i16;
_10.fld4 = Field::<usize>(Variant(_7, 0), 1);
_4 = _10.fld2 as i16;
_7 = Adt46::Variant0 { fld0: _10.fld2,fld1: _10.fld4 };
_2 = 179092677468347553650898478394440309724_u128 as i16;
Call(_12 = core::intrinsics::fmaf64(RET, RET, RET), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10.fld4 = Field::<usize>(Variant(_7, 0), 1) << _1;
_10.fld4 = Field::<usize>(Variant(_7, 0), 1) + Field::<usize>(Variant(_7, 0), 1);
_5 = !_6;
_13 = _10.fld4 as i64;
Goto(bb10)
}
bb10 = {
_4 = _1 - _6;
_15 = _10.fld2 as f64;
_13 = -(-3211690371245519087_i64);
_10.fld2 = Field::<u64>(Variant(_7, 0), 0);
_3 = _10.fld6 as i16;
_1 = !_5;
_9 = [2036311641_u32];
_1 = _6;
_16 = [_10.fld7];
_9 = [370036284_u32];
_13 = -4813362610291050905_i64;
_10.fld1 = RET * _15;
_10.fld4 = !Field::<usize>(Variant(_7, 0), 1);
_8 = '\u{69cd7}';
_17.0.1 = -_15;
_17.2.0 = _10.fld1;
Goto(bb11)
}
bb11 = {
SetDiscriminant(_7, 1);
place!(Field::<Adt41>(Variant(_7, 1), 4)).fld2 = _10.fld2;
place!(Field::<(bool,)>(Variant(_7, 1), 2)).0 = true;
place!(Field::<Adt40>(Variant(_7, 1), 1)).fld0 = core::ptr::addr_of_mut!(_8);
_7 = Adt46::Variant0 { fld0: _10.fld2,fld1: _10.fld4 };
SetDiscriminant(_7, 1);
place!(Field::<(bool,)>(Variant(_7, 1), 2)) = (false,);
place!(Field::<Adt41>(Variant(_7, 1), 4)).fld5 = _17.0.1 as i32;
_10.fld6 = !111_u8;
_18 = (_10.fld4,);
_8 = '\u{6e195}';
_11.fld1.0 = core::ptr::addr_of_mut!(_16);
place!(Field::<Adt41>(Variant(_7, 1), 4)).fld6 = _10.fld6;
Call(place!(Field::<Adt40>(Variant(_7, 1), 1)).fld7 = core::intrinsics::transmute(_10.fld7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_17.0 = (RET, _17.2.0);
Goto(bb13)
}
bb13 = {
RET = _17.2.0 * _15;
place!(Field::<Adt40>(Variant(_7, 1), 1)).fld1 = _4 as u32;
RET = -_17.0.0;
match _11.fld0 {
0 => bb1,
1 => bb7,
2 => bb10,
3 => bb12,
4 => bb8,
5 => bb6,
6 => bb14,
97520680 => bb16,
_ => bb15
}
}
bb14 = {
_17.0 = (RET, _17.2.0);
Goto(bb13)
}
bb15 = {
_9 = [515740110_u32];
_2 = _4 - _1;
_6 = _5 & _4;
_5 = !_1;
_5 = _6 * _2;
_7 = Adt46::Variant0 { fld0: 18211277595569982173_u64,fld1: 3_usize };
_6 = !_4;
_10.fld6 = 120_u8;
_10.fld5 = 97520680_i32;
_10.fld6 = !53_u8;
match _10.fld5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
97520680 => bb8,
_ => bb7
}
}
bb16 = {
_10.fld3 = _11.fld1.0;
place!(Field::<Adt41>(Variant(_7, 1), 4)).fld1 = _17.0.0 * _17.0.1;
_17.0 = (Field::<Adt41>(Variant(_7, 1), 4).fld1, _10.fld1);
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(14_usize, 3_usize, Move(_3), 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_21 = dump_var(14_usize, 2_usize, Move(_2), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(10294032018165841329_u64), std::hint::black_box(190_u8), std::hint::black_box(2361436309_u32), std::hint::black_box((-25_i8)), std::hint::black_box(24505_i16), std::hint::black_box((-1984954924_i32)), std::hint::black_box(316884975908534550166952766284006026133_u128), std::hint::black_box(24595_u16));
                
            }
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: f32,
fld1: (usize,),

},
Variant1{
fld0: (bool,),
fld1: (u64, bool),
fld2: *mut char,
fld3: i8,
fld4: usize,
fld5: [u32; 1],
fld6: [i128; 2],
fld7: u64,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: [u32; 1],
fld1: *const (u64, bool),

},
Variant1{
fld0: *const *const (u64, bool),

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: *mut char,
fld1: u32,
fld2: (f64, f64),
fld3: [i128; 2],
fld4: (bool,),
fld5: i32,
fld6: u128,
fld7: i128,
}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: *mut (f64, f64),
fld1: f64,
fld2: u64,
fld3: *mut [i128; 1],
fld4: usize,
fld5: i32,
fld6: u8,
fld7: i128,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ((f64, f64), [i128; 2], (f64, f64)),
fld1: [i128; 1],
fld2: *mut char,
fld3: *mut isize,

},
Variant1{
fld0: [i128; 2],
fld1: (f64, f64),
fld2: Adt40,

},
Variant2{
fld0: bool,
fld1: [i128; 2],
fld2: u32,
fld3: *mut [i128; 1],
fld4: (i128, usize),
fld5: (bool,),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: (bool,),
fld2: u32,
fld3: i128,
fld4: [i8; 5],
fld5: i32,
fld6: Adt40,

},
Variant1{
fld0: i64,
fld1: (u128, usize, i64, u128),
fld2: f64,
fld3: *mut char,
fld4: [i8; 5],
fld5: *const *const (u64, bool),

},
Variant2{
fld0: *mut char,
fld1: *const (u64, bool),

}}
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i128; 1],

},
Variant1{
fld0: *mut isize,
fld1: Adt43,

},
Variant2{
fld0: (u64, bool),
fld1: (i128, usize),

},
Variant3{
fld0: [u32; 1],
fld1: (u128, usize, i64, u128),
fld2: [i8; 5],
fld3: (u64, bool),
fld4: i16,
fld5: [i128; 1],
fld6: u8,
fld7: Adt38,

}}
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: f64,
fld1: *mut char,
fld2: (u128, usize, i64, u128),
fld3: (usize,),
fld4: i16,
fld5: i32,
fld6: i128,

},
Variant1{
fld0: (u64, [i128; 1], isize),
fld1: (usize,),

},
Variant2{
fld0: *const (u64, bool),
fld1: (u64, [i128; 1], isize),
fld2: Adt43,
fld3: (*mut [i128; 1], *const *const (u64, bool)),
fld4: u64,
fld5: u128,
fld6: (u128, usize, i64, u128),
fld7: [u32; 1],

},
Variant3{
fld0: [i128; 2],
fld1: *mut [i128; 1],
fld2: [i128; 1],
fld3: usize,
fld4: *mut (f64, f64),

}}
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u64,
fld1: usize,

},
Variant1{
fld0: Adt42,
fld1: Adt40,
fld2: (bool,),
fld3: *mut bool,
fld4: Adt41,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u32; 1],
fld1: Adt44,
fld2: *const *const (u64, bool),
fld3: Adt46,
fld4: (*mut [i128; 1], *const *const (u64, bool)),
fld5: *mut bool,
fld6: usize,

},
Variant1{
fld0: u8,
fld1: u16,
fld2: Adt41,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (i128, usize),
fld1: usize,
fld2: *mut isize,
fld3: (*mut [i128; 1], *const *const (u64, bool)),
fld4: (f64, f64),
fld5: i32,
fld6: Adt40,

},
Variant1{
fld0: Adt38,
fld1: char,
fld2: Adt43,
fld3: *mut char,
fld4: *const *const (u64, bool),
fld5: *const (u64, bool),
fld6: i64,
fld7: f32,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (u64, bool),
fld1: usize,
fld2: Adt38,
}
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
fld0: (usize,),
fld1: *mut char,
fld2: *const *const (u64, bool),

},
Variant1{
fld0: (bool,),
fld1: Adt48,
fld2: *mut (f64, f64),
fld3: i8,
fld4: *mut isize,
fld5: Adt46,
fld6: i64,
fld7: [i128; 2],

},
Variant2{
fld0: Adt44,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt48,
fld1: Adt41,
fld2: *mut isize,
fld3: u32,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (*mut [i128; 1], *const *const (u64, bool)),
fld1: f32,
fld2: [i128; 2],
fld3: (u64, [i128; 1], isize),
fld4: *mut bool,
fld5: [u32; 1],

},
Variant1{
fld0: Adt48,
fld1: [i128; 1],
fld2: (bool,),
fld3: Adt42,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u32; 1],

},
Variant1{
fld0: Adt38,

},
Variant2{
fld0: (u64, [i128; 1], isize),
fld1: (u128, usize, i64, u128),
fld2: [u32; 1],
fld3: [i128; 1],
fld4: u64,

},
Variant3{
fld0: Adt42,
fld1: u16,
fld2: Adt50,
fld3: [i128; 1],
fld4: (bool,),
fld5: Adt51,
fld6: Adt46,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: i32,
fld1: (*mut [i128; 1], *const *const (u64, bool)),
fld2: Adt38,
fld3: Adt42,
}

