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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> isize {
mir! {
type RET = isize;
let _13: char;
let _14: isize;
let _15: usize;
let _16: [i128; 4];
let _17: ((isize,), (i8,), u64);
let _18: u32;
let _19: [u128; 7];
let _20: [i128; 4];
let _21: ((isize,),);
let _22: Adt53;
let _23: *const (i8,);
let _24: isize;
let _25: bool;
let _26: (u64, *mut (i8,), u8);
let _27: isize;
let _28: (usize, usize, u16);
let _29: (i8,);
let _30: [i128; 7];
let _31: ([u128; 7], [char; 4], i128);
let _32: u32;
let _33: ();
let _34: ();
{
_5 = 26792_i16 + (-18975_i16);
RET = 100_isize * 88_isize;
_5 = !(-8745_i16);
Call(_5 = fn1(RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-5_isize) + (-9223372036854775808_isize);
_11 = 44634_u16;
_8 = 39505373903089793584603230637955920809_u128;
_8 = 118367576732165505634574909786200839243_u128;
_5 = 3303779446_u32 as i16;
_12 = !3972359924_u32;
_14 = false as isize;
_9 = !3_usize;
_1 = !true;
RET = -_14;
_2 = '\u{1b8a2}';
_1 = _12 < _12;
_16 = [119052712025646052391541117916449231009_i128,10067723920539324117028271138987500607_i128,(-113964313224594885753095427519347754461_i128),57252931217863573896042623817448501440_i128];
_4 = (-68_i8) | 71_i8;
_17.1 = (_4,);
_4 = -_17.1.0;
_8 = 162985561244195465060727961059144662729_u128;
_9 = !5_usize;
match _11 {
0 => bb2,
1 => bb3,
44634 => bb5,
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
_18 = _12;
_14 = !RET;
_6 = 9035782227120068155_i64 as i32;
_3 = RET << _11;
RET = !_3;
_1 = true | false;
_18 = _5 as u32;
RET = _3 * _14;
_7 = 2859165898788084717_i64;
_15 = _1 as usize;
_16 = [31993736123214734649060393632013455231_i128,(-69258001076270770053790396261715776906_i128),(-75491695002799271231627295027471852039_i128),(-67535773112600205952057165363755369150_i128)];
_8 = !319845770726846177040693849026336977733_u128;
_4 = _17.1.0;
_2 = '\u{44c87}';
_14 = _5 as isize;
_6 = 259532272_i32;
_23 = core::ptr::addr_of!(_17.1);
_15 = _14 as usize;
_16 = [(-113098047965221183135664601442148369882_i128),(-441347622116000192082467691831196801_i128),70424444156908757291097745496461183194_i128,(-100444418523535740418667806238228548203_i128)];
(*_23) = (_4,);
(*_23).0 = 6302075511633545182_u64 as i8;
_11 = 54590_u16 - 56434_u16;
Call(_17.0.0 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13 = _2;
_12 = _18 ^ _18;
_24 = 6429388074910756337_u64 as isize;
_8 = _15 as u128;
_8 = 246769036863930527035485498427100701031_u128;
_17.0 = (RET,);
_6 = !2012905822_i32;
(*_23).0 = _4;
_25 = _1;
_7 = (-2767905942821730287_i64) << _17.0.0;
_13 = _2;
_17.0 = (RET,);
_17.0.0 = _3;
_21.0.0 = !_3;
_25 = !_1;
_26.2 = 15592800470679527804853649047567618778_i128 as u8;
_17.0.0 = !RET;
_19 = [_8,_8,_8,_8,_8,_8,_8];
(*_23).0 = _4;
_20 = _16;
_18 = !_12;
_13 = _2;
_26.0 = 13257790806301125115_u64 - 6590600480020013409_u64;
Goto(bb7)
}
bb7 = {
_26.1 = core::ptr::addr_of_mut!((*_23));
_11 = 20240_u16 - 22247_u16;
_16 = [163774529872948371198404406163510453360_i128,1178205023120568254500820177673268626_i128,(-31190933146984661528683183498662044210_i128),75869747370276777748255871634026761288_i128];
_1 = _3 > _3;
_21 = (_17.0,);
_8 = 146959273258432658061058312117564664436_u128;
RET = _25 as isize;
_1 = _25;
_21.0 = (_17.0.0,);
_28 = (_9, _15, _11);
_24 = !_17.0.0;
_17.2 = _26.0;
_6 = 1354392617_i32;
_17.0.0 = !RET;
Goto(bb8)
}
bb8 = {
_26.1 = core::ptr::addr_of_mut!(_29);
match _8 {
0 => bb9,
1 => bb10,
146959273258432658061058312117564664436 => bb12,
_ => bb11
}
}
bb9 = {
_26.1 = core::ptr::addr_of_mut!((*_23));
_11 = 20240_u16 - 22247_u16;
_16 = [163774529872948371198404406163510453360_i128,1178205023120568254500820177673268626_i128,(-31190933146984661528683183498662044210_i128),75869747370276777748255871634026761288_i128];
_1 = _3 > _3;
_21 = (_17.0,);
_8 = 146959273258432658061058312117564664436_u128;
RET = _25 as isize;
_1 = _25;
_21.0 = (_17.0.0,);
_28 = (_9, _15, _11);
_24 = !_17.0.0;
_17.2 = _26.0;
_6 = 1354392617_i32;
_17.0.0 = !RET;
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
RET = (-5_isize) + (-9223372036854775808_isize);
_11 = 44634_u16;
_8 = 39505373903089793584603230637955920809_u128;
_8 = 118367576732165505634574909786200839243_u128;
_5 = 3303779446_u32 as i16;
_12 = !3972359924_u32;
_14 = false as isize;
_9 = !3_usize;
_1 = !true;
RET = -_14;
_2 = '\u{1b8a2}';
_1 = _12 < _12;
_16 = [119052712025646052391541117916449231009_i128,10067723920539324117028271138987500607_i128,(-113964313224594885753095427519347754461_i128),57252931217863573896042623817448501440_i128];
_4 = (-68_i8) | 71_i8;
_17.1 = (_4,);
_4 = -_17.1.0;
_8 = 162985561244195465060727961059144662729_u128;
_9 = !5_usize;
match _11 {
0 => bb2,
1 => bb3,
44634 => bb5,
_ => bb4
}
}
bb12 = {
_15 = _9 * _9;
RET = _3 ^ _24;
_22 = Adt53::Variant1 { fld0: (*_23).0 };
match _8 {
146959273258432658061058312117564664436 => bb14,
_ => bb13
}
}
bb13 = {
_26.1 = core::ptr::addr_of_mut!((*_23));
_11 = 20240_u16 - 22247_u16;
_16 = [163774529872948371198404406163510453360_i128,1178205023120568254500820177673268626_i128,(-31190933146984661528683183498662044210_i128),75869747370276777748255871634026761288_i128];
_1 = _3 > _3;
_21 = (_17.0,);
_8 = 146959273258432658061058312117564664436_u128;
RET = _25 as isize;
_1 = _25;
_21.0 = (_17.0.0,);
_28 = (_9, _15, _11);
_24 = !_17.0.0;
_17.2 = _26.0;
_6 = 1354392617_i32;
_17.0.0 = !RET;
Goto(bb8)
}
bb14 = {
_32 = _18;
_30 = [118838267564196932115177369654288349089_i128,(-104389889098654174184051765251006390751_i128),(-84908895028596698729198634472375963089_i128),(-39548607079811934831203992790239545823_i128),130472341645227761715356907616638188687_i128,44403274407539417056815167118455302319_i128,32752582478842284844974018513558376884_i128];
(*_23) = (Field::<i8>(Variant(_22, 1), 0),);
_21.0 = (_3,);
_18 = !_32;
_3 = _21.0.0 + RET;
_15 = !_9;
_29 = _17.1;
(*_23) = (Field::<i8>(Variant(_22, 1), 0),);
_23 = core::ptr::addr_of!(_29);
_26.1 = core::ptr::addr_of_mut!((*_23));
_7 = (-6056110076370862061_i64);
_4 = _17.1.0 | (*_23).0;
RET = _3 & _3;
_7 = -3279792506804116209_i64;
_24 = !_3;
_17.0.0 = RET;
_25 = _1;
_25 = _1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 6_usize, Move(_6), 28_usize, Move(_28), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 25_usize, Move(_25), 8_usize, Move(_8), 5_usize, Move(_5), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 24_usize, Move(_24), 29_usize, Move(_29), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(0_usize, 17_usize, Move(_17), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> i16 {
mir! {
type RET = i16;
let _5: bool;
let _6: f64;
let _7: i16;
let _8: f32;
let _9: [u32; 3];
let _10: u16;
let _11: f32;
let _12: u16;
let _13: f64;
let _14: ((isize,),);
let _15: Adt60;
let _16: [i8; 2];
let _17: Adt50;
let _18: Adt57;
let _19: [i128; 4];
let _20: [i128; 4];
let _21: i8;
let _22: *const [i8; 6];
let _23: [char; 3];
let _24: (usize, [u64; 1], *mut (i8,));
let _25: f64;
let _26: i32;
let _27: [u32; 3];
let _28: (usize, usize, u16);
let _29: Adt47;
let _30: bool;
let _31: u8;
let _32: (i8,);
let _33: [u128; 7];
let _34: usize;
let _35: ((isize,), (i8,), u64);
let _36: bool;
let _37: u32;
let _38: ();
let _39: ();
{
RET = 515807159_u32 as i16;
_3 = _4;
_3 = -_2;
_4 = _3;
RET = (-96_i8) as i16;
_1 = _3;
_1 = -_2;
_3 = -_4;
_2 = _4;
_5 = _2 < _3;
_1 = !_4;
_4 = _3;
_5 = !true;
_2 = _3 * _1;
_5 = true;
RET = 20888_i16;
_2 = !_3;
_2 = -_3;
match RET {
0 => bb1,
20888 => bb3,
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
_7 = 13175676489900805832_usize as i16;
_1 = 15394612692099396358_u64 as isize;
_6 = 3824848509_u32 as f64;
_6 = 6097497705672044592_i64 as f64;
_1 = _3 & _3;
_5 = false;
_8 = RET as f32;
_4 = _6 as isize;
_9 = [3156938174_u32,2514640494_u32,2035203825_u32];
Goto(bb4)
}
bb4 = {
RET = _6 as i16;
_5 = _6 >= _6;
_8 = 21355_u16 as f32;
_1 = !_2;
_10 = 269_u16 & 26268_u16;
_3 = !_1;
_4 = _1 ^ _2;
_11 = _8 * _8;
_12 = !_10;
_8 = 1061005425_u32 as f32;
RET = _7;
_4 = 0_usize as isize;
_3 = _1;
RET = 3735471951_u32 as i16;
_2 = -_1;
RET = _7 + _7;
_6 = 98_u8 as f64;
_4 = _3 * _2;
_14.0.0 = 17137871476944560907_u64 as isize;
_13 = _6 - _6;
_10 = _12;
_3 = -_2;
Call(_14.0 = fn2(_2, _9, _4, _5, _4, _1, _1, _4, _4, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = [2546625200_u32,895026140_u32,1839232260_u32];
_13 = _6 + _6;
_16 = [(-105_i8),13_i8];
_10 = !_12;
_14.0.0 = _1 ^ _1;
RET = _7 | _7;
RET = _7 & _7;
Call(_8 = fn18(_14.0, _2, _14, _14.0, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14.0 = (_3,);
_16 = [56_i8,108_i8];
_2 = _3 | _14.0.0;
_13 = 109829649600342187646995647557472143983_i128 as f64;
_7 = RET;
_16 = [(-24_i8),22_i8];
_5 = !true;
_7 = RET;
_14.0 = (_2,);
_4 = !_1;
Goto(bb7)
}
bb7 = {
_3 = -_14.0.0;
_5 = false;
_9 = [2395032330_u32,3145479871_u32,313673071_u32];
_12 = !_10;
_11 = _8 + _8;
_17.fld3 = _9;
_10 = !_12;
_3 = !_2;
_1 = -_14.0.0;
_4 = _2 & _1;
_14.0.0 = -_3;
Call(_17.fld5 = core::intrinsics::bswap(1254942081_i32), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = (-128868628468539703521402258186888275868_i128) as i16;
_14.0.0 = _3 | _4;
_3 = 2_usize as isize;
_9 = _17.fld3;
_3 = 3875613598_u32 as isize;
_5 = !false;
RET = _7;
_14.0.0 = 312705828972861457945783219523346497775_u128 as isize;
_3 = _4;
_2 = -_3;
_17.fld2 = core::ptr::addr_of_mut!(_18.fld3);
_18.fld3.0 = !4180234427167358832_usize;
_13 = _6 - _6;
_9 = _17.fld3;
_18.fld3.0 = !1568639694149444099_usize;
_8 = -_11;
_3 = _2;
_11 = _8;
_18.fld1 = '\u{b711c}';
_17.fld1 = [43818154969212462023897228314558274462_i128,(-67471506495359994125616001010544956921_i128),83485393684065191825347001006495093208_i128,(-79192269504732164316609480357043109510_i128)];
_18.fld3.2 = _7 as u16;
_18.fld2 = 300806240740791054181713539092989082296_u128 as isize;
_13 = -_6;
_18.fld3.1 = !_18.fld3.0;
Goto(bb9)
}
bb9 = {
RET = 4460524776006866508_i64 as i16;
_18.fld3 = (12955398217212539779_usize, 5_usize, _12);
_18.fld1 = '\u{c2be1}';
RET = (-6721774077222053014_i64) as i16;
_20 = _17.fld1;
_17.fld1 = [169847863473121496738683461840827523037_i128,126250499048898817354947847555737038803_i128,146090754394917140840289112850432583247_i128,148466540013043211411483053472481390299_i128];
_11 = _8;
_17.fld1 = [29992693288697110218046243190499118630_i128,109240863183524105343799372062358162214_i128,12515644914188786500226616760625680468_i128,(-115235134017118470257274977551906096374_i128)];
_18.fld0 = ((-92_i8),);
_18.fld0.0 = 30_i8;
_12 = _5 as u16;
_23 = [_18.fld1,_18.fld1,_18.fld1];
_14.0 = (_4,);
_18.fld3.1 = _18.fld3.0 ^ _18.fld3.0;
_19 = _17.fld1;
_18.fld3.1 = _18.fld0.0 as usize;
_17.fld4 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_18.fld0 = (105_i8,);
_17.fld1 = _19;
_5 = true;
_17.fld5 = !1790260614_i32;
_18.fld2 = _7 as isize;
_24.2 = core::ptr::addr_of_mut!(_18.fld0);
Goto(bb10)
}
bb10 = {
_24.0 = _18.fld3.0;
_18.fld2 = _18.fld3.2 as isize;
_11 = _18.fld3.2 as f32;
_28 = (_24.0, _24.0, _10);
_6 = _13 * _13;
_18.fld3.1 = _28.1 / _28.1;
Call(_4 = fn19(_14.0.0, _14, _18.fld3, _24.2, _28.2, _18.fld2, _14, _19, _14.0, _5, _28.1, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24.2 = core::ptr::addr_of_mut!(_18.fld0);
_24.1 = [6901407714268657712_u64];
_17.fld2 = core::ptr::addr_of_mut!(_28);
_10 = 10563550024173060861_u64 as u16;
_11 = 1831563954_u32 as f32;
RET = _7 & _7;
_25 = (-125664515196313339555614084993628336588_i128) as f64;
RET = _24.0 as i16;
_9 = _17.fld3;
Goto(bb12)
}
bb12 = {
_23 = [_18.fld1,_18.fld1,_18.fld1];
_14.0 = (_4,);
_24.2 = core::ptr::addr_of_mut!(_18.fld0);
_18.fld3.0 = 3825243472_u32 as usize;
_2 = _4;
_32.0 = !_18.fld0.0;
_28.0 = _28.1 - _18.fld3.1;
RET = -_7;
match _24.0 {
12955398217212539779 => bb13,
_ => bb10
}
}
bb13 = {
_28.0 = _28.2 as usize;
RET = _7 | _7;
_21 = _32.0;
_24.0 = _18.fld3.1 << _2;
_19 = [(-23773234385465264149236046656766004900_i128),(-124558791533101813857624384265739305021_i128),99375751315522964543142090446379062857_i128,(-6062073527400050091690598858551040499_i128)];
_28.0 = _24.0;
_21 = _32.0;
_7 = _5 as i16;
_17.fld5 = 17054639378664056491_u64 as i32;
_24.1 = [15065296626022037088_u64];
RET = !_7;
_2 = _4 * _14.0.0;
_23 = [_18.fld1,_18.fld1,_18.fld1];
_24.0 = !_28.0;
_33 = [81006471928753226295265310327465604560_u128,56221689929178334161861357367607380654_u128,208785497729889198981500431389409844470_u128,147872659155636656453285315695943206663_u128,46202510858768958991725338699087672566_u128,198682554990756451294043147955204154861_u128,173438300358905779220948972610785291709_u128];
_3 = -_2;
_13 = -_6;
_25 = _8 as f64;
_35 = (_14.0, _18.fld0, 7457915724580321616_u64);
_30 = _5;
_27 = _17.fld3;
_32.0 = _18.fld0.0 * _21;
_4 = _2;
Goto(bb14)
}
bb14 = {
_7 = 216508459567150894148774996514327171790_u128 as i16;
_33 = [79868235416065652868914189796745250813_u128,168343009820978682060783767875376463520_u128,137481356481317548545794138356995498862_u128,237215060471459667239838558759272403021_u128,270414578555891456311589075703146557478_u128,271944057556474210075921120683624684301_u128,204048308496970454851928968383941958267_u128];
_26 = -_17.fld5;
_18.fld0.0 = _18.fld1 as i8;
_28 = (_24.0, _24.0, _12);
_18.fld1 = '\u{d58b2}';
_16 = [_35.1.0,_32.0];
_18.fld3.2 = !_28.2;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(1_usize, 32_usize, Move(_32), 2_usize, Move(_2), 30_usize, Move(_30), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(1_usize, 33_usize, Move(_33), 35_usize, Move(_35), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(1_usize, 20_usize, Move(_20), 23_usize, Move(_23), 21_usize, Move(_21), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: [u32; 3],mut _3: isize,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> (isize,) {
mir! {
type RET = (isize,);
let _11: i8;
let _12: isize;
let _13: *const (i8,);
let _14: Adt51;
let _15: isize;
let _16: char;
let _17: char;
let _18: Adt57;
let _19: [u32; 3];
let _20: ();
let _21: ();
{
RET.0 = _5 & _7;
RET = (_10,);
RET.0 = _1;
_2 = [121347662_u32,207570797_u32,3404173374_u32];
_10 = _7 & _9;
_10 = _1 >> _5;
_8 = _7;
RET.0 = _3 | _6;
Goto(bb1)
}
bb1 = {
_10 = 5388785678711885097_usize as isize;
_1 = RET.0 & _3;
_1 = _3 - RET.0;
RET = (_1,);
_4 = false & true;
_4 = !true;
RET = (_5,);
_2 = [722942772_u32,2797456046_u32,2571918826_u32];
_11 = 121_i8 ^ 10_i8;
RET.0 = !_5;
Call(RET.0 = fn3(_3, _1, _9, _9, _1, _9, _2, _8, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = _4 as isize;
Goto(bb3)
}
bb3 = {
_1 = !_3;
Call(_12 = fn17(_9, _8, _5, _7, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = [4216006187_u32,3987956026_u32,3137932352_u32];
_7 = _1;
RET = (_9,);
_10 = _3 * _9;
RET = (_1,);
RET = (_12,);
_9 = _5 * _10;
RET.0 = 1902478436_u32 as isize;
_8 = _3;
_9 = 2428310278_u32 as isize;
_2 = [395710684_u32,407765973_u32,2030681911_u32];
RET.0 = !_10;
_6 = RET.0;
_9 = _5 >> _8;
_11 = !(-46_i8);
RET = (_6,);
RET = (_1,);
_7 = _8;
Goto(bb5)
}
bb5 = {
_11 = !(-52_i8);
RET.0 = !_6;
_5 = 1241050437_u32 as isize;
_3 = _11 as isize;
_2 = [4291862956_u32,2539402923_u32,3147369249_u32];
RET = (_9,);
_15 = _7 | _3;
_16 = '\u{ffdb}';
_15 = _7 ^ _1;
_18.fld3 = (3518894465559599146_usize, 7_usize, 3377_u16);
_18.fld1 = _16;
_15 = RET.0 & _7;
match _18.fld3.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
3377 => bb11,
_ => bb10
}
}
bb6 = {
_2 = [4216006187_u32,3987956026_u32,3137932352_u32];
_7 = _1;
RET = (_9,);
_10 = _3 * _9;
RET = (_1,);
RET = (_12,);
_9 = _5 * _10;
RET.0 = 1902478436_u32 as isize;
_8 = _3;
_9 = 2428310278_u32 as isize;
_2 = [395710684_u32,407765973_u32,2030681911_u32];
RET.0 = !_10;
_6 = RET.0;
_9 = _5 >> _8;
_11 = !(-46_i8);
RET = (_6,);
RET = (_1,);
_7 = _8;
Goto(bb5)
}
bb7 = {
_1 = !_3;
Call(_12 = fn17(_9, _8, _5, _7, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET.0 = _4 as isize;
Goto(bb3)
}
bb9 = {
_10 = 5388785678711885097_usize as isize;
_1 = RET.0 & _3;
_1 = _3 - RET.0;
RET = (_1,);
_4 = false & true;
_4 = !true;
RET = (_5,);
_2 = [722942772_u32,2797456046_u32,2571918826_u32];
_11 = 121_i8 ^ 10_i8;
RET.0 = !_5;
Call(RET.0 = fn3(_3, _1, _9, _9, _1, _9, _2, _8, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_13 = core::ptr::addr_of!(_18.fld0);
_18.fld2 = !_7;
_19 = [2040719921_u32,2340499491_u32,643085408_u32];
_11 = _18.fld3.0 as i8;
(*_13).0 = (-127258894953106985325571699783669666998_i128) as i8;
_18.fld3.1 = !_18.fld3.0;
_16 = _18.fld1;
RET.0 = -_15;
Goto(bb12)
}
bb12 = {
Call(_20 = dump_var(2_usize, 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_20 = dump_var(2_usize, 6_usize, Move(_6), 8_usize, Move(_8), 19_usize, Move(_19), 21_usize, _21), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: [u32; 3],mut _8: isize,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: Adt55;
let _11: *const ((isize,),);
let _12: f32;
let _13: i8;
let _14: u32;
let _15: Adt49;
let _16: ();
let _17: ();
{
RET = !_3;
_4 = 9289397660439276569_u64 as isize;
_7 = [2047509334_u32,753878806_u32,3341910695_u32];
_5 = _2;
Call(_11 = fn4(_3, _2, _2, _3, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = 64937_u16 as f32;
_5 = (-26_i8) as isize;
_10 = Adt55::Variant0 { fld0: 2820376595044915849_usize };
_2 = RET << _3;
_6 = 105894670072395880439955736994351985458_i128 as isize;
_4 = 38426_u16 as isize;
_6 = 517964860_i32 as isize;
_10 = Adt55::Variant0 { fld0: 0_usize };
_12 = (-103511486149262860022375939315405016854_i128) as f32;
RET = _2 >> _3;
_4 = 107_u8 as isize;
_4 = !RET;
_3 = -RET;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(3_usize, 1_usize, Move(_1), 3_usize, Move(_3), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> *const ((isize,),) {
mir! {
type RET = *const ((isize,),);
let _7: u32;
let _8: *const [i8; 6];
let _9: Adt52;
let _10: [i128; 7];
let _11: *const u32;
let _12: [u64; 1];
let _13: bool;
let _14: [i128; 4];
let _15: &'static i8;
let _16: f32;
let _17: f32;
let _18: Adt47;
let _19: Adt60;
let _20: f32;
let _21: Adt59;
let _22: f64;
let _23: usize;
let _24: f32;
let _25: f64;
let _26: bool;
let _27: *const ((isize,),);
let _28: i32;
let _29: f64;
let _30: f64;
let _31: Adt59;
let _32: f32;
let _33: (isize,);
let _34: ((isize,), (i8,), u64);
let _35: [i128; 7];
let _36: [i8; 2];
let _37: [u128; 7];
let _38: i16;
let _39: u16;
let _40: Adt54;
let _41: (usize, [u64; 1], *mut (i8,));
let _42: i8;
let _43: isize;
let _44: bool;
let _45: Adt57;
let _46: i16;
let _47: Adt56;
let _48: u16;
let _49: [i128; 7];
let _50: ((isize,),);
let _51: ();
let _52: ();
{
_5 = -_3;
_2 = -_5;
_4 = 64_i8 as isize;
_5 = 237_u8 as isize;
_2 = !_3;
_5 = !_2;
_2 = -_3;
_7 = !3161005388_u32;
_6 = !_1;
_7 = 2117206063_u32;
_1 = 32789_u16 as isize;
_1 = 761014532_i32 as isize;
_7 = 2580881634_u32 << _2;
_1 = _2;
_2 = _3 & _1;
_2 = -_1;
_2 = _1;
_4 = !_3;
_1 = (-3636110066286616704_i64) as isize;
_5 = -_2;
_6 = -_4;
Goto(bb1)
}
bb1 = {
_6 = _4;
_4 = _5 & _2;
_7 = 1749705452_u32;
_1 = -_5;
_1 = _3 >> _5;
_4 = 107638986_i32 as isize;
_7 = !3001918945_u32;
_3 = _1;
_3 = '\u{fd1e0}' as isize;
_6 = _2 & _4;
_4 = _2;
_10 = [(-67663561209946059827328784754983137426_i128),(-38917874648679716564898974358742748015_i128),(-93069868052141144733510837283833497028_i128),117673680654076900374547682433835610649_i128,132366190365642814931200090811127377015_i128,61299422300943972792789735912725178797_i128,43565143552831993774221572732237696692_i128];
_3 = 5469596620834290018_u64 as isize;
_1 = _6 * _4;
_7 = 1237834647_u32;
_5 = 18250809011944277508_u64 as isize;
_5 = true as isize;
Goto(bb2)
}
bb2 = {
_5 = !_2;
_11 = core::ptr::addr_of!(_7);
_1 = _4 ^ _5;
_11 = core::ptr::addr_of!(_7);
_11 = core::ptr::addr_of!((*_11));
_5 = _6;
_2 = _4 - _6;
(*_11) = !978084714_u32;
_3 = -_4;
(*_11) = !957966874_u32;
_11 = core::ptr::addr_of!((*_11));
_5 = true as isize;
_14 = [(-24716555107407579465936728160179919007_i128),67217858241328462767484412385820126334_i128,111789682897780135419638121544036016646_i128,7638661118777266608501622776634413582_i128];
_1 = _6;
_13 = _5 <= _6;
_16 = (-72963591237279431156367716417398506951_i128) as f32;
_2 = _13 as isize;
_4 = _3 + _1;
_5 = 12673_u16 as isize;
_3 = _2;
Goto(bb3)
}
bb3 = {
(*_11) = 2075530293_u32 - 1664847573_u32;
_12 = [1508360843508518701_u64];
_2 = _1 ^ _4;
_14 = [(-57109793749969999195954271599156465668_i128),159101425859652647681114568488422190331_i128,63349569400553857828186058638343371938_i128,73091836314782285716257443115285044417_i128];
_10 = [(-10232342917131347948926092748848443242_i128),(-168718988376990144661212604030005216391_i128),(-92452418899966531098324111905500377938_i128),19344888009227201887360747735693509805_i128,(-104442577210696624792424193264674770850_i128),(-11212882626230478088818158247135691856_i128),(-17510960710352457701168942039553918312_i128)];
_17 = (-135379651140580435611026434985480405456_i128) as f32;
_11 = core::ptr::addr_of!(_7);
_3 = _4;
_13 = !true;
_13 = !true;
_1 = 2_usize as isize;
_16 = 50879_u16 as f32;
_7 = _13 as u32;
_4 = _5 - _6;
_10 = [70630347508085036035441326894144904136_i128,(-102953917011965681413494079073522104677_i128),(-24938117728606691884899760062236495635_i128),(-150281726400787619669733329483382467589_i128),45217425863463160310933044340411200446_i128,66781436731199526845207258369467603212_i128,108470125722863965038345226411017144374_i128];
(*_11) = 3855655570_u32;
Goto(bb4)
}
bb4 = {
_13 = true | true;
(*_11) = 3766770548_u32;
_20 = -_17;
_5 = _4 << _4;
_5 = !_4;
_5 = -_3;
_17 = _16;
_14 = [350084026342934918281783106021223644_i128,(-46399035084562528116743529072439511958_i128),(-156601592892533729036900134377727519117_i128),111404507835057743205977665400483560969_i128];
_7 = 690782865_u32;
_13 = !false;
_10 = [(-119816404265602978354150666987536971636_i128),49292604811019680346739711535079461385_i128,112611018931791484035411314610999021659_i128,(-166572184927807878559698466363997646836_i128),146072222272581268648706184839393343922_i128,119150737778009633744053853720022523982_i128,51145039441467641349730023038571962806_i128];
(*_11) = 184_u8 as u32;
Goto(bb5)
}
bb5 = {
_1 = _4 + _3;
_10 = [105310983525691777571791972427549018347_i128,158249312426377158466668207565681151617_i128,54977848001513542646346393065977565381_i128,116966201681685070881514110418219834114_i128,(-59089391786735871263960615172231048684_i128),18329106118082025614375926560897843280_i128,(-40998664945550866419176843666574922692_i128)];
_13 = true;
_16 = (-3769937751192586491_i64) as f32;
Goto(bb6)
}
bb6 = {
_5 = 6815024299817892905_i64 as isize;
_14 = [7544654998128007523308164831053773613_i128,103753964283553922532606273559530784513_i128,26360402203886090976289650102768940087_i128,(-121909788866620371082089301388690752001_i128)];
_5 = !_2;
_6 = -_3;
_17 = _20 - _16;
_12 = [7619980543844047958_u64];
_13 = !true;
_11 = core::ptr::addr_of!((*_11));
_22 = (-29083_i16) as f64;
Call((*_11) = fn5(_6, _3, _2, _1, _4, _5, _3, _5, _4, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = _16 - _16;
(*_11) = !3250572414_u32;
_23 = 5_usize;
match _10[_23] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
18329106118082025614375926560897843280 => bb10,
_ => bb9
}
}
bb8 = {
_5 = 6815024299817892905_i64 as isize;
_14 = [7544654998128007523308164831053773613_i128,103753964283553922532606273559530784513_i128,26360402203886090976289650102768940087_i128,(-121909788866620371082089301388690752001_i128)];
_5 = !_2;
_6 = -_3;
_17 = _20 - _16;
_12 = [7619980543844047958_u64];
_13 = !true;
_11 = core::ptr::addr_of!((*_11));
_22 = (-29083_i16) as f64;
Call((*_11) = fn5(_6, _3, _2, _1, _4, _5, _3, _5, _4, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_5 = !_2;
_11 = core::ptr::addr_of!(_7);
_1 = _4 ^ _5;
_11 = core::ptr::addr_of!(_7);
_11 = core::ptr::addr_of!((*_11));
_5 = _6;
_2 = _4 - _6;
(*_11) = !978084714_u32;
_3 = -_4;
(*_11) = !957966874_u32;
_11 = core::ptr::addr_of!((*_11));
_5 = true as isize;
_14 = [(-24716555107407579465936728160179919007_i128),67217858241328462767484412385820126334_i128,111789682897780135419638121544036016646_i128,7638661118777266608501622776634413582_i128];
_1 = _6;
_13 = _5 <= _6;
_16 = (-72963591237279431156367716417398506951_i128) as f32;
_2 = _13 as isize;
_4 = _3 + _1;
_5 = 12673_u16 as isize;
_3 = _2;
Goto(bb3)
}
bb10 = {
_7 = !1525272691_u32;
_2 = !_5;
_23 = 2_usize - 8502813574234227947_usize;
_10 = [101140480562750471139415997983396992585_i128,(-154569436366817185846178819108380530711_i128),(-4082106728503583792263684707355304149_i128),144552021572244412387002443801028814366_i128,(-54110747205029737161751472964359573390_i128),50551725923787843309098706950193042060_i128,(-161079591113772575504082334224005258962_i128)];
_17 = _16 + _20;
_10 = [(-156195532786925143362034859861776709125_i128),(-84282717763326337739218100206799552266_i128),169549307014285259608596785709478100337_i128,114010406001293586017673848320399102255_i128,78678915426730064230363667093609672405_i128,(-61182104812040083609908109422766648119_i128),107745596090325443815211323038052512996_i128];
_10 = [(-112185416791166702584514524822639265654_i128),90042898472184553102689154172090201841_i128,(-19201770700017710981571797700543508448_i128),(-113979526821679421866664237672916655225_i128),(-50583059361820249627540754196080404195_i128),4176557918566865018969507598615015501_i128,146886889880489852549053035611835353148_i128];
_25 = _22;
_2 = _1 & _6;
_23 = !16226570807798370576_usize;
_5 = !_1;
_23 = 5182933774745373877_usize;
_4 = 45686010638584272931055055714778608704_u128 as isize;
(*_11) = !285591987_u32;
_3 = _2 >> _2;
_26 = _3 <= _3;
_7 = !4171715294_u32;
_23 = (-82_i8) as usize;
_2 = !_6;
_13 = _5 != _1;
_13 = _26;
_7 = 1041674240_u32 | 3149399471_u32;
_10 = [22776110306787594598326518445765148929_i128,(-16130585325826637818907429845864373571_i128),(-17652011975435374963793308637715104122_i128),110254573878051475561693649228826603530_i128,(-20785734478318821112033712164214351707_i128),(-41847141194182322822691116409328034580_i128),92848840340977591016440263785307804222_i128];
_14 = [143645982921957672880550046671531133711_i128,14401423627380120356396406881065438824_i128,36415805929131076586167696575557616818_i128,9695200596840218645351725251446789823_i128];
_24 = -_17;
Goto(bb11)
}
bb11 = {
_2 = _3;
_30 = _25;
_29 = _22 * _25;
_13 = _3 == _3;
_20 = _16 + _24;
_4 = _20 as isize;
_13 = !_26;
(*_11) = (-106842804783129471592002415466225702184_i128) as u32;
_26 = !_13;
_13 = _3 <= _3;
_12 = [3150926801049594852_u64];
_16 = _20;
_12 = [8786621544393247359_u64];
(*_11) = 3615140718_u32;
_3 = 3416_i16 as isize;
_26 = !_13;
_32 = _16 * _16;
_4 = 35525_u16 as isize;
_1 = -_2;
_12 = [15596445293572385555_u64];
_24 = _32;
_26 = !_13;
_22 = 4706710581729540720_i64 as f64;
_26 = !_13;
_5 = _2 << _2;
_23 = 7208828811196923378_usize;
_22 = _30;
Call(_5 = fn13(_2, _20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22 = _29;
_11 = core::ptr::addr_of!((*_11));
_22 = (-72_i8) as f64;
_32 = _16;
_1 = -_6;
Goto(bb13)
}
bb13 = {
_33.0 = _13 as isize;
_16 = _20 - _32;
_4 = 55439_u16 as isize;
_2 = _33.0;
_33.0 = _6 | _2;
_34.0 = (_1,);
(*_11) = 103352022_u32 | 824772034_u32;
_35 = [105672380657554531950888740347531060655_i128,56319332686172564935828282925997537578_i128,(-64766311104234410406001715043635830149_i128),148425080492234809758740496496969145686_i128,(-159909364527619842663142378362861579246_i128),101031679419047929623802110604918504880_i128,(-32220549539926575557459834572846551102_i128)];
_34.0.0 = (*_11) as isize;
(*_11) = 482114905_u32 * 1725687516_u32;
_30 = 207567020_i32 as f64;
_2 = !_1;
_33.0 = _5 * _6;
_28 = 164871466750640261007500484256399828550_u128 as i32;
_4 = _1;
match _23 {
0 => bb1,
1 => bb5,
2 => bb14,
7208828811196923378 => bb16,
_ => bb15
}
}
bb14 = {
_5 = !_2;
_11 = core::ptr::addr_of!(_7);
_1 = _4 ^ _5;
_11 = core::ptr::addr_of!(_7);
_11 = core::ptr::addr_of!((*_11));
_5 = _6;
_2 = _4 - _6;
(*_11) = !978084714_u32;
_3 = -_4;
(*_11) = !957966874_u32;
_11 = core::ptr::addr_of!((*_11));
_5 = true as isize;
_14 = [(-24716555107407579465936728160179919007_i128),67217858241328462767484412385820126334_i128,111789682897780135419638121544036016646_i128,7638661118777266608501622776634413582_i128];
_1 = _6;
_13 = _5 <= _6;
_16 = (-72963591237279431156367716417398506951_i128) as f32;
_2 = _13 as isize;
_4 = _3 + _1;
_5 = 12673_u16 as isize;
_3 = _2;
Goto(bb3)
}
bb15 = {
(*_11) = 2075530293_u32 - 1664847573_u32;
_12 = [1508360843508518701_u64];
_2 = _1 ^ _4;
_14 = [(-57109793749969999195954271599156465668_i128),159101425859652647681114568488422190331_i128,63349569400553857828186058638343371938_i128,73091836314782285716257443115285044417_i128];
_10 = [(-10232342917131347948926092748848443242_i128),(-168718988376990144661212604030005216391_i128),(-92452418899966531098324111905500377938_i128),19344888009227201887360747735693509805_i128,(-104442577210696624792424193264674770850_i128),(-11212882626230478088818158247135691856_i128),(-17510960710352457701168942039553918312_i128)];
_17 = (-135379651140580435611026434985480405456_i128) as f32;
_11 = core::ptr::addr_of!(_7);
_3 = _4;
_13 = !true;
_13 = !true;
_1 = 2_usize as isize;
_16 = 50879_u16 as f32;
_7 = _13 as u32;
_4 = _5 - _6;
_10 = [70630347508085036035441326894144904136_i128,(-102953917011965681413494079073522104677_i128),(-24938117728606691884899760062236495635_i128),(-150281726400787619669733329483382467589_i128),45217425863463160310933044340411200446_i128,66781436731199526845207258369467603212_i128,108470125722863965038345226411017144374_i128];
(*_11) = 3855655570_u32;
Goto(bb4)
}
bb16 = {
_33.0 = !_6;
_34.1.0 = 101_i8 - (-46_i8);
_22 = _25;
_34.0.0 = 1742509146754116401_i64 as isize;
_23 = 0_usize;
_35 = [_10[_23],_14[_23],_10[_23],_14[_23],_10[_23],_10[_23],_10[_23]];
_33.0 = 57_u8 as isize;
_12[_23] = 8332446093085186718_u64;
_15 = &_36[_23];
_12 = [17368293091243536970_u64];
_2 = _34.1.0 as isize;
_26 = !_13;
_22 = -_29;
_13 = _26 == _26;
Call(_35[_23] = fn14(_32, _13, _10, _1, _26, _13, _6, _5, _26, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_34.0 = _33;
_37 = [82583983729485858432376317560929251217_u128,335950632245275984152905799308460104176_u128,77536544005573192887714977732350150578_u128,124852693268219619442570239994161323351_u128,330920781832629632757334980005652239072_u128,29824721451687325810344770788875342863_u128,215533024521284836333549573265600736228_u128];
_34.0.0 = _4;
_32 = _24;
_11 = core::ptr::addr_of!((*_11));
_39 = 136605042073081541795449131825098623971_u128 as u16;
Call(_34.2 = core::intrinsics::transmute(_34.0.0), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_10 = _35;
_34.1 = ((-55_i8),);
_33.0 = _13 as isize;
_34.1 = ((-85_i8),);
_17 = _34.1.0 as f32;
_23 = 3_usize & 6_usize;
_43 = _39 as isize;
_12 = [_34.2];
_35 = _10;
_34.1 = ((-121_i8),);
_45.fld3.1 = !_23;
_15 = &(*_15);
_45.fld3.2 = _39;
_38 = 667_i16;
(*_11) = 1935369185_u32;
_34.0 = (_33.0,);
_45.fld3 = (_23, _23, _39);
_41.1 = _12;
_41.0 = _23 | _23;
_41.2 = core::ptr::addr_of_mut!(_45.fld0);
_38 = _28 as i16;
_42 = 92_u8 as i8;
_39 = _45.fld3.2 | _45.fld3.2;
_45.fld1 = '\u{c13ec}';
match (*_11) {
0 => bb19,
1 => bb20,
1935369185 => bb22,
_ => bb21
}
}
bb19 = {
_6 = _4;
_4 = _5 & _2;
_7 = 1749705452_u32;
_1 = -_5;
_1 = _3 >> _5;
_4 = 107638986_i32 as isize;
_7 = !3001918945_u32;
_3 = _1;
_3 = '\u{fd1e0}' as isize;
_6 = _2 & _4;
_4 = _2;
_10 = [(-67663561209946059827328784754983137426_i128),(-38917874648679716564898974358742748015_i128),(-93069868052141144733510837283833497028_i128),117673680654076900374547682433835610649_i128,132366190365642814931200090811127377015_i128,61299422300943972792789735912725178797_i128,43565143552831993774221572732237696692_i128];
_3 = 5469596620834290018_u64 as isize;
_1 = _6 * _4;
_7 = 1237834647_u32;
_5 = 18250809011944277508_u64 as isize;
_5 = true as isize;
Goto(bb2)
}
bb20 = {
_7 = !1525272691_u32;
_2 = !_5;
_23 = 2_usize - 8502813574234227947_usize;
_10 = [101140480562750471139415997983396992585_i128,(-154569436366817185846178819108380530711_i128),(-4082106728503583792263684707355304149_i128),144552021572244412387002443801028814366_i128,(-54110747205029737161751472964359573390_i128),50551725923787843309098706950193042060_i128,(-161079591113772575504082334224005258962_i128)];
_17 = _16 + _20;
_10 = [(-156195532786925143362034859861776709125_i128),(-84282717763326337739218100206799552266_i128),169549307014285259608596785709478100337_i128,114010406001293586017673848320399102255_i128,78678915426730064230363667093609672405_i128,(-61182104812040083609908109422766648119_i128),107745596090325443815211323038052512996_i128];
_10 = [(-112185416791166702584514524822639265654_i128),90042898472184553102689154172090201841_i128,(-19201770700017710981571797700543508448_i128),(-113979526821679421866664237672916655225_i128),(-50583059361820249627540754196080404195_i128),4176557918566865018969507598615015501_i128,146886889880489852549053035611835353148_i128];
_25 = _22;
_2 = _1 & _6;
_23 = !16226570807798370576_usize;
_5 = !_1;
_23 = 5182933774745373877_usize;
_4 = 45686010638584272931055055714778608704_u128 as isize;
(*_11) = !285591987_u32;
_3 = _2 >> _2;
_26 = _3 <= _3;
_7 = !4171715294_u32;
_23 = (-82_i8) as usize;
_2 = !_6;
_13 = _5 != _1;
_13 = _26;
_7 = 1041674240_u32 | 3149399471_u32;
_10 = [22776110306787594598326518445765148929_i128,(-16130585325826637818907429845864373571_i128),(-17652011975435374963793308637715104122_i128),110254573878051475561693649228826603530_i128,(-20785734478318821112033712164214351707_i128),(-41847141194182322822691116409328034580_i128),92848840340977591016440263785307804222_i128];
_14 = [143645982921957672880550046671531133711_i128,14401423627380120356396406881065438824_i128,36415805929131076586167696575557616818_i128,9695200596840218645351725251446789823_i128];
_24 = -_17;
Goto(bb11)
}
bb21 = {
(*_11) = 2075530293_u32 - 1664847573_u32;
_12 = [1508360843508518701_u64];
_2 = _1 ^ _4;
_14 = [(-57109793749969999195954271599156465668_i128),159101425859652647681114568488422190331_i128,63349569400553857828186058638343371938_i128,73091836314782285716257443115285044417_i128];
_10 = [(-10232342917131347948926092748848443242_i128),(-168718988376990144661212604030005216391_i128),(-92452418899966531098324111905500377938_i128),19344888009227201887360747735693509805_i128,(-104442577210696624792424193264674770850_i128),(-11212882626230478088818158247135691856_i128),(-17510960710352457701168942039553918312_i128)];
_17 = (-135379651140580435611026434985480405456_i128) as f32;
_11 = core::ptr::addr_of!(_7);
_3 = _4;
_13 = !true;
_13 = !true;
_1 = 2_usize as isize;
_16 = 50879_u16 as f32;
_7 = _13 as u32;
_4 = _5 - _6;
_10 = [70630347508085036035441326894144904136_i128,(-102953917011965681413494079073522104677_i128),(-24938117728606691884899760062236495635_i128),(-150281726400787619669733329483382467589_i128),45217425863463160310933044340411200446_i128,66781436731199526845207258369467603212_i128,108470125722863965038345226411017144374_i128];
(*_11) = 3855655570_u32;
Goto(bb4)
}
bb22 = {
_34.1.0 = _45.fld1 as i8;
_46 = 229622718807791662148700643256538258825_u128 as i16;
_6 = _45.fld3.2 as isize;
_45.fld3.1 = _41.0;
_32 = -_24;
_45.fld2 = -_33.0;
RET = core::ptr::addr_of!(_50);
(*RET).0 = _34.0;
(*RET).0.0 = -_1;
Goto(bb23)
}
bb23 = {
Call(_51 = dump_var(4_usize, 13_usize, Move(_13), 10_usize, Move(_10), 1_usize, Move(_1), 34_usize, Move(_34)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_51 = dump_var(4_usize, 33_usize, Move(_33), 14_usize, Move(_14), 26_usize, Move(_26), 23_usize, Move(_23)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_51 = dump_var(4_usize, 2_usize, Move(_2), 39_usize, Move(_39), 28_usize, Move(_28), 43_usize, Move(_43)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> u32 {
mir! {
type RET = u32;
let _11: f64;
let _12: *mut bool;
let _13: ((isize,),);
let _14: (isize,);
let _15: isize;
let _16: Adt49;
let _17: Adt52;
let _18: Adt48;
let _19: isize;
let _20: i16;
let _21: [i128; 4];
let _22: char;
let _23: Adt55;
let _24: ([u128; 7], [char; 4], i128);
let _25: [u32; 3];
let _26: i16;
let _27: bool;
let _28: isize;
let _29: *mut (usize, usize, u16);
let _30: usize;
let _31: f64;
let _32: usize;
let _33: [u32; 3];
let _34: *mut bool;
let _35: ();
let _36: ();
{
_10 = 5659322602074839121_usize as isize;
_8 = !_3;
_8 = -_2;
_3 = 114178895092780617864648455961559786330_i128 as isize;
RET = !1037586249_u32;
_4 = -_8;
_8 = '\u{951c2}' as isize;
_4 = _1;
_9 = 1571529546127693896_i64 as isize;
_4 = '\u{3b8ce}' as isize;
_11 = 9939373797199169543_u64 as f64;
_7 = _2;
_6 = 111135351500260384456580304417318024217_i128 as isize;
_6 = _1;
Call(_11 = fn6(_7, _5, _6, _6, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 41_i8 as u32;
_9 = -_6;
_1 = _2 + _7;
_2 = (-6328610376330202501_i64) as isize;
_9 = _1;
Call(_12 = fn7(_6, _6, _1, _1, _7, _6, _5, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = !_7;
_9 = _8;
_3 = false as isize;
_8 = '\u{c673c}' as isize;
_11 = 158_u8 as f64;
_2 = _7 * _1;
_14.0 = 298102792486309835524332335991314489014_u128 as isize;
_3 = (-27709_i16) as isize;
Goto(bb3)
}
bb3 = {
_2 = _11 as isize;
_6 = 108_u8 as isize;
_13.0.0 = !_1;
_8 = !_5;
_8 = _1;
_15 = _1 * _7;
_14 = (_1,);
_2 = _14.0;
RET = 31222_u16 as u32;
_19 = RET as isize;
_1 = -_13.0.0;
_14.0 = _6;
_13.0.0 = _5;
RET = 3867574497_u32 + 2731485325_u32;
_7 = -_1;
_14.0 = _2;
_22 = '\u{5d1c8}';
_2 = !_14.0;
Goto(bb4)
}
bb4 = {
_14.0 = _15;
_14 = (_1,);
_4 = -_7;
_9 = _14.0 + _2;
_13.0 = _14;
Call(_11 = fn11(_7, _13.0, _13.0.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = (_14,);
_7 = false as isize;
_24.2 = 135638753606855197816052307731988307043_i128 - 138628280437735081552966814102526899477_i128;
_13.0.0 = !_5;
_21 = [_24.2,_24.2,_24.2,_24.2];
_24.0 = [189749780660435083891899026038009737396_u128,335447024009889605282389518843209229381_u128,44942134677993490492530309526864652878_u128,190179062061053061262152428442901415189_u128,80431811869001874466889331306349156204_u128,8985155331428289807743402634876869570_u128,316254568196247702395478688966817134394_u128];
_20 = (-27000_i16);
_18 = Adt48::Variant2 { fld0: _20,fld1: _12 };
_19 = _14.0;
_28 = _4;
_24.0 = [302842678999377074157449750984387138464_u128,256925458286441958667296295772042679686_u128,126276947155598123453438492943365439047_u128,173727072414022306757096375960294849822_u128,165547723510605810655101677999022856389_u128,279689491007421495047919254451008018977_u128,11532203732858571619767466934586853691_u128];
place!(Field::<*mut bool>(Variant(_18, 2), 1)) = _12;
Goto(bb6)
}
bb6 = {
_12 = core::ptr::addr_of_mut!(_27);
SetDiscriminant(_18, 1);
(*_12) = !false;
place!(Field::<usize>(Variant(_18, 1), 1)) = !969012944351883656_usize;
_4 = _19;
_13.0 = (_1,);
_22 = '\u{9e333}';
RET = _9 as u32;
_25 = [RET,RET,RET];
_31 = Field::<usize>(Variant(_18, 1), 1) as f64;
(*_12) = false ^ true;
_13 = (_14,);
place!(Field::<isize>(Variant(_18, 1), 2)) = -_8;
place!(Field::<isize>(Variant(_18, 1), 2)) = -_13.0.0;
_12 = core::ptr::addr_of_mut!((*_12));
_31 = _11 - _11;
Goto(bb7)
}
bb7 = {
Call(_35 = dump_var(5_usize, 7_usize, Move(_7), 14_usize, Move(_14), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_35 = dump_var(5_usize, 9_usize, Move(_9), 22_usize, Move(_22), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_35 = dump_var(5_usize, 20_usize, Move(_20), 28_usize, Move(_28), 36_usize, _36, 36_usize, _36), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize) -> f64 {
mir! {
type RET = f64;
let _6: ();
let _7: ();
{
_5 = _4 << _2;
RET = _2 as f64;
_1 = '\u{b21d6}' as isize;
_3 = !_5;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(6_usize, 5_usize, Move(_5), 1_usize, Move(_1), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> *mut bool {
mir! {
type RET = *mut bool;
let _9: [u128; 7];
let _10: u16;
let _11: *mut (i8,);
let _12: i16;
let _13: [i64; 8];
let _14: i8;
let _15: bool;
let _16: (isize,);
let _17: u32;
let _18: [u32; 3];
let _19: [i8; 6];
let _20: [u32; 3];
let _21: f32;
let _22: u8;
let _23: Adt56;
let _24: (u64, *mut (i8,), u8);
let _25: isize;
let _26: [i8; 2];
let _27: [u32; 3];
let _28: ();
let _29: ();
{
_9 = [15800869267946482631764697164685127109_u128,200862128557776499050582087556084771582_u128,197847611286082041671526618084198896929_u128,17678449336253905560427193776158003505_u128,266386746428542848015204507013735657179_u128,40864247493791706471193673669443165159_u128,44574413443010329187527160087807147094_u128];
_8 = 3157_u16 as isize;
_7 = -_2;
_3 = -_6;
_12 = 8806_i16;
_2 = !_3;
_5 = -_4;
_6 = _4 - _1;
_10 = 9391813677917953463_usize as u16;
_4 = _6 - _1;
_5 = _4 * _6;
_8 = 806618240_u32 as isize;
_3 = 6406888655204102831_i64 as isize;
_3 = !_7;
_13 = [4531884759783036452_i64,(-865113581386640759_i64),(-8606287154472717116_i64),2694919544189629272_i64,130684430521485484_i64,(-1961786932387675056_i64),(-4905045383377581013_i64),(-6135465158614276075_i64)];
_7 = -_1;
_13 = [(-9157752080831741129_i64),8262523949966318244_i64,3231868044127828688_i64,8188858659749086989_i64,7366689708807535109_i64,(-9142068145275940697_i64),5143410519662530917_i64,(-3109320726229359722_i64)];
_5 = -_6;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
8806 => bb5,
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
_3 = _7 & _6;
_16.0 = -_6;
_15 = false;
_8 = !_1;
_14 = (-11_i8) ^ 17_i8;
_2 = 1747850674_u32 as isize;
_16.0 = _4 + _3;
_13 = [(-5210439870073986593_i64),(-1853503241343201583_i64),(-2276910478662191916_i64),2782836413201102479_i64,157058607642696447_i64,2128297050749979858_i64,1832545868485485287_i64,(-1449432671158788607_i64)];
_8 = _3 * _7;
_19 = [_14,_14,_14,_14,_14,_14];
_13 = [2952479852934962724_i64,2239213514081846184_i64,1681401519538011858_i64,(-3576374143163908361_i64),8951290197547778476_i64,5359066374295654705_i64,4486821051608265808_i64,(-3408280784231780589_i64)];
_14 = -(-40_i8);
_17 = 908895733_u32;
_3 = _16.0;
_13 = [7053709995794463794_i64,6535567724085609711_i64,6657815074646362951_i64,5609079779213624129_i64,934963302053069979_i64,(-3706225027282890700_i64),973715951315510313_i64,5300745439054325733_i64];
_5 = 787279940_i32 as isize;
_19 = [_14,_14,_14,_14,_14,_14];
_2 = _6 ^ _6;
_5 = _16.0 ^ _7;
_1 = _2 * _6;
Call(_19 = fn8(_6, _16.0, _16.0, _5, _8, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20 = [_17,_17,_17];
_22 = 38_u8 * 11_u8;
_21 = _3 as f32;
_1 = _6 | _2;
RET = core::ptr::addr_of_mut!(_15);
RET = core::ptr::addr_of_mut!((*RET));
_5 = _8 ^ _2;
_20 = [_17,_17,_17];
(*RET) = false & false;
(*RET) = _1 < _8;
_17 = 381128136_u32;
Goto(bb7)
}
bb7 = {
_14 = !(-7_i8);
_5 = -_8;
_8 = '\u{93017}' as isize;
_7 = _22 as isize;
_2 = _16.0;
_18 = _20;
_5 = _16.0;
_1 = _4 | _6;
(*RET) = _2 <= _5;
_20 = [_17,_17,_17];
_19 = [_14,_14,_14,_14,_14,_14];
_19 = [_14,_14,_14,_14,_14,_14];
(*RET) = _21 > _21;
_16.0 = _5 | _2;
_8 = !_4;
_1 = _3 + _3;
_3 = -_6;
(*RET) = false;
(*RET) = true;
_3 = 51557835272033974_i64 as isize;
_8 = _17 as isize;
RET = core::ptr::addr_of_mut!((*RET));
_1 = _2;
_4 = _5;
Call(_7 = fn10(_2, _21, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = [_17,_17,_17];
_15 = false;
_12 = !12127_i16;
(*RET) = _14 == _14;
_26 = [_14,_14];
_26 = [_14,_14];
_25 = (-2062522531255847889_i64) as isize;
_4 = _10 as isize;
_20 = _18;
_2 = -_1;
_24.2 = 81104849366157702634207197367650443832_u128 as u8;
_18 = _20;
_7 = _2 >> _1;
(*RET) = _2 <= _7;
RET = core::ptr::addr_of_mut!((*RET));
Goto(bb9)
}
bb9 = {
Call(_28 = dump_var(7_usize, 20_usize, Move(_20), 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_28 = dump_var(7_usize, 22_usize, Move(_22), 6_usize, Move(_6), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_28 = dump_var(7_usize, 15_usize, Move(_15), 4_usize, Move(_4), 7_usize, Move(_7), 29_usize, _29), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> [i8; 6] {
mir! {
type RET = [i8; 6];
let _7: u8;
let _8: *const u32;
let _9: u128;
let _10: Adt49;
let _11: (i8,);
let _12: Adt57;
let _13: [char; 4];
let _14: ([u128; 7], [char; 4], i128);
let _15: i8;
let _16: f64;
let _17: ();
let _18: ();
{
_2 = _1 + _5;
RET = [(-73_i8),(-2_i8),74_i8,24_i8,125_i8,105_i8];
RET = [(-84_i8),(-37_i8),9_i8,(-37_i8),96_i8,(-87_i8)];
_3 = _4 & _6;
_5 = _3;
_9 = 147403231188079325461255967715375162372_u128 + 58509420898340058367984981192467722134_u128;
_6 = _3 << _3;
_7 = 18425389432515312784_u64 as u8;
_9 = 109101813519999454396599420258359440831_u128;
_5 = 1462727932724056388_i64 as isize;
_4 = _3 >> _1;
match _9 {
0 => bb1,
109101813519999454396599420258359440831 => bb3,
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
_11 = ((-122_i8),);
_4 = _6 ^ _1;
_12.fld3.2 = 10321_u16 & 45127_u16;
_11.0 = 15_i8 + (-39_i8);
_12.fld1 = '\u{4e870}';
RET = [_11.0,_11.0,_11.0,_11.0,_11.0,_11.0];
_5 = !_2;
_12.fld3.0 = 7_usize;
_14.1 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_12.fld3.0 = _2 as usize;
_14.0 = [_9,_9,_9,_9,_9,_9,_9];
_12.fld3.1 = (-2121867504619754955_i64) as usize;
_12.fld2 = _12.fld3.0 as isize;
_12.fld0 = (_11.0,);
_7 = 176_u8 >> _2;
_12.fld3.2 = 33094_u16 << _6;
_14.2 = !99029453782869660002667402050584020242_i128;
_5 = _6 - _2;
_16 = _12.fld0.0 as f64;
_5 = -_4;
_3 = _6 >> _12.fld3.2;
_9 = 245570683107532520371453336318118827389_u128 + 271867683143223948172489504173043439049_u128;
_11 = (_12.fld0.0,);
_15 = -_11.0;
_11.0 = -_12.fld0.0;
Call(RET = fn9(_12.fld2, _3, _4, _12.fld3, _5, _2, _12.fld0.0, _7, _12.fld2, _2, _12, _5, _5, _5, _4, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = (_15,);
_4 = _1;
_11.0 = _12.fld0.0;
_12.fld2 = _3;
_13 = _14.1;
_9 = !136035049825532242875768982831182406937_u128;
_4 = -_12.fld2;
_12.fld0 = _11;
Goto(bb5)
}
bb5 = {
Call(_17 = dump_var(8_usize, 7_usize, Move(_7), 11_usize, Move(_11), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_17 = dump_var(8_usize, 5_usize, Move(_5), 14_usize, Move(_14), 18_usize, _18, 18_usize, _18), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: (usize, usize, u16),mut _5: isize,mut _6: isize,mut _7: i8,mut _8: u8,mut _9: isize,mut _10: isize,mut _11: Adt57,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> [i8; 6] {
mir! {
type RET = [i8; 6];
let _17: f32;
let _18: Adt54;
let _19: f64;
let _20: [u64; 1];
let _21: [u128; 7];
let _22: Adt47;
let _23: u128;
let _24: *const ((isize,),);
let _25: ([u128; 7], [char; 4], i128);
let _26: i8;
let _27: ();
let _28: ();
{
_14 = _16;
_4.2 = !_11.fld3.2;
_12 = -_1;
_2 = _16;
RET = [_11.fld0.0,_11.fld0.0,_7,_11.fld0.0,_11.fld0.0,_11.fld0.0];
RET = [_11.fld0.0,_11.fld0.0,_11.fld0.0,_11.fld0.0,_7,_11.fld0.0];
_3 = _10;
_6 = (-1224547294_i32) as isize;
_11.fld3.1 = _8 as usize;
_12 = -_14;
_16 = -_13;
_6 = -_9;
_11.fld3 = (_4.0, _4.0, _4.2);
RET = [_7,_11.fld0.0,_7,_7,_11.fld0.0,_11.fld0.0];
_14 = !_1;
_5 = _16;
_4.2 = _11.fld3.2 * _11.fld3.2;
_11.fld2 = _10;
_8 = !191_u8;
_11.fld0.0 = _7 >> _11.fld3.0;
_4.0 = !_11.fld3.1;
_7 = _11.fld0.0;
_11.fld2 = _16 & _12;
Goto(bb1)
}
bb1 = {
_19 = 322329880638161217931406767083058871575_u128 as f64;
_4.2 = _11.fld3.2 << _11.fld0.0;
_1 = 35789014973610934321667369604874610317_i128 as isize;
_4.2 = _11.fld3.2 - _11.fld3.2;
_5 = _14;
_1 = _2;
_11.fld3.2 = !_4.2;
_20 = [12252141833859404458_u64];
_17 = _11.fld0.0 as f32;
Goto(bb2)
}
bb2 = {
RET = [_11.fld0.0,_7,_11.fld0.0,_7,_7,_7];
_12 = _15;
_8 = 138_u8;
_5 = _2 ^ _10;
_2 = _16;
_7 = !_11.fld0.0;
_15 = _16 ^ _6;
_4 = (_11.fld3.1, _11.fld3.1, _11.fld3.2);
_11.fld1 = '\u{ace82}';
_4.2 = _11.fld3.2;
_13 = (-2082671967_i32) as isize;
_6 = _2;
_4 = (_11.fld3.1, _11.fld3.0, _11.fld3.2);
_8 = 151_u8 << _11.fld3.2;
_7 = -_11.fld0.0;
_21 = [75297800338039491112765100195337809112_u128,267371373621228965960353998402644312423_u128,30155275960260416930367172717859618153_u128,4081235583552772753228985172547100739_u128,238774555407571051933773724029487351930_u128,140099216966294388618718756879449332109_u128,101584949262804605444773518507532496191_u128];
_4.1 = false as usize;
_7 = !_11.fld0.0;
_20 = [18228596570743189388_u64];
_11.fld1 = '\u{b4142}';
_25.1 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1];
_4 = _11.fld3;
_25.2 = !(-84179239502666953186853989833014659519_i128);
Goto(bb3)
}
bb3 = {
Call(_27 = dump_var(9_usize, 6_usize, Move(_6), 14_usize, Move(_14), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_27 = dump_var(9_usize, 12_usize, Move(_12), 20_usize, Move(_20), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: f32,mut _3: (isize,)) -> isize {
mir! {
type RET = isize;
let _4: [i128; 4];
let _5: f64;
let _6: ();
let _7: ();
{
_1 = !_3.0;
_2 = (-26266_i16) as f32;
_3.0 = _1 >> _1;
RET = 25_i8 as isize;
_3.0 = 29144_i16 as isize;
RET = _1;
RET = _1 * _1;
RET = _1 >> _1;
_3 = (_1,);
RET = !_3.0;
_2 = 113_u8 as f32;
_1 = -_3.0;
_1 = RET;
RET = !_1;
_3.0 = RET;
RET = _3.0 << _3.0;
_1 = _3.0;
_4 = [(-81416370075142239731375380110202374216_i128),67218199645269604584507178922977691232_i128,(-108447290504582704330983611812245411565_i128),81849672167658496065948810886089280936_i128];
_3.0 = !_1;
_3.0 = _1;
RET = _3.0 << _3.0;
_3.0 = 637861279_i32 as isize;
_1 = RET;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(10_usize, 3_usize, Move(_3), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: (isize,),mut _3: isize) -> f64 {
mir! {
type RET = f64;
let _4: bool;
let _5: f64;
let _6: *mut bool;
let _7: bool;
let _8: [i8; 2];
let _9: char;
let _10: bool;
let _11: Adt45;
let _12: u64;
let _13: [u32; 3];
let _14: [u32; 3];
let _15: i32;
let _16: ();
let _17: ();
{
_2.0 = _1;
RET = (-118_i8) as f64;
_2.0 = _1;
Call(_4 = fn12(_3, _2, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _3;
_3 = _2.0 - _2.0;
_2 = (_3,);
_4 = !true;
_4 = !false;
_1 = !_2.0;
_2 = (_3,);
_3 = _4 as isize;
_5 = -RET;
_5 = RET;
_5 = -RET;
_3 = 44912823861718398674055314086125474474_i128 as isize;
_4 = false;
RET = -_5;
_1 = !_2.0;
_3 = _2.0;
Goto(bb2)
}
bb2 = {
RET = _5;
RET = -_5;
RET = -_5;
_1 = _3;
_1 = 2085330212_i32 as isize;
RET = 13093802115618430604_u64 as f64;
RET = _5 + _5;
_2 = (_3,);
_6 = core::ptr::addr_of_mut!(_4);
_7 = (*_6);
_8 = [(-123_i8),97_i8];
_1 = !_3;
_1 = _3;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = core::ptr::addr_of_mut!((*_6));
_6 = core::ptr::addr_of_mut!(_4);
(*_6) = !_7;
_3 = (-473258989_i32) as isize;
_10 = (*_6);
_5 = RET;
_3 = 8373108413576389429_i64 as isize;
_2.0 = _1;
_4 = _10 | _10;
(*_6) = !_7;
_8 = [(-53_i8),116_i8];
(*_6) = !_7;
_12 = (-12127_i16) as u64;
(*_6) = !_7;
_3 = _1;
_11 = Adt45::Variant0 { fld0: _6 };
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _3 as f64;
_5 = 296473040_u32 as f64;
_1 = (-2822849386945973580_i64) as isize;
_1 = -_2.0;
_1 = _2.0;
_12 = (-5_i8) as u64;
_4 = !_7;
_4 = _10;
_9 = '\u{8cf3c}';
_12 = _3 as u64;
(*_6) = !_7;
_12 = !7271009959077945643_u64;
_9 = '\u{ae582}';
_12 = 155_u8 as u64;
SetDiscriminant(_11, 2);
place!(Field::<isize>(Variant(_11, 2), 2)) = _1;
(*_6) = !_7;
_7 = !_10;
_1 = _3 & _2.0;
_4 = !_7;
_10 = RET != RET;
(*_6) = _10;
_12 = !12059644943857049871_u64;
Goto(bb5)
}
bb5 = {
Call(_16 = dump_var(11_usize, 2_usize, Move(_2), 7_usize, Move(_7), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: isize) -> bool {
mir! {
type RET = bool;
let _6: f64;
let _7: [char; 3];
let _8: ();
let _9: ();
{
RET = !false;
_2 = (_5,);
_2.0 = 1253283506_i32 as isize;
RET = _1 > _1;
_3 = _4;
_2 = (_3,);
_2 = (_5,);
RET = _3 > _1;
_5 = !_1;
_2 = (_1,);
RET = !true;
_3 = _2.0;
_3 = -_4;
_3 = (-26816_i16) as isize;
Goto(bb1)
}
bb1 = {
RET = !true;
_3 = 75_u8 as isize;
RET = false;
RET = _5 < _5;
_7 = ['\u{a4a1c}','\u{44835}','\u{9ff4a}'];
_3 = 12595_i16 as isize;
_6 = _1 as f64;
_7 = ['\u{fdd3e}','\u{37b70}','\u{7b656}'];
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(12_usize, 3_usize, Move(_3), 4_usize, Move(_4), 5_usize, Move(_5), 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: f32) -> isize {
mir! {
type RET = isize;
let _3: char;
let _4: [i8; 2];
let _5: f64;
let _6: ();
let _7: ();
{
RET = _1 << _1;
_1 = _2 as isize;
_2 = 193097552508604260786100786870814704083_u128 as f32;
RET = (-640499792_i32) as isize;
_3 = '\u{d124}';
_3 = '\u{18455}';
_2 = (-4415939501013824372_i64) as f32;
RET = _1;
_3 = '\u{485b0}';
_1 = !RET;
RET = !_1;
_4 = [(-8_i8),21_i8];
_5 = _2 as f64;
_4 = [(-11_i8),120_i8];
_2 = 2431966_u32 as f32;
_2 = 38729_u16 as f32;
_4 = [17_i8,21_i8];
_4 = [4_i8,(-73_i8)];
_2 = 11701_u16 as f32;
Goto(bb1)
}
bb1 = {
_2 = 48921_u16 as f32;
RET = _1 + _1;
_3 = '\u{36dec}';
_3 = '\u{51473}';
_1 = RET;
_2 = RET as f32;
RET = _1;
RET = _1 & _1;
_2 = 4016531066475289251_i64 as f32;
_5 = (-71851948644652847_i64) as f64;
Goto(bb2)
}
bb2 = {
Call(_6 = dump_var(13_usize, 4_usize, Move(_4), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: f32,mut _2: bool,mut _3: [i128; 7],mut _4: isize,mut _5: bool,mut _6: bool,mut _7: isize,mut _8: isize,mut _9: bool,mut _10: bool) -> i128 {
mir! {
type RET = i128;
let _11: [i8; 6];
let _12: i64;
let _13: (usize, usize, u16);
let _14: i64;
let _15: *mut (i8,);
let _16: i16;
let _17: *const u32;
let _18: (u64, *mut (i8,), u8);
let _19: (*const (i8,), i64, i8);
let _20: [u32; 3];
let _21: u32;
let _22: Adt57;
let _23: Adt58;
let _24: [i64; 8];
let _25: i128;
let _26: u32;
let _27: i64;
let _28: [u64; 1];
let _29: f32;
let _30: ([u128; 7], [char; 4], i128);
let _31: ([u128; 7], [char; 4], i128);
let _32: [char; 4];
let _33: (u64, *mut (i8,), u8);
let _34: ([u128; 7], [char; 4], i128);
let _35: ();
let _36: ();
{
_10 = _6;
_4 = _10 as isize;
_8 = _4 | _4;
_3 = [(-167453128832939634359163848784435676506_i128),84291125003247592684471590990455944392_i128,167193696256304726193804751057213081511_i128,(-28792340343906859410455405023347402803_i128),162826492052441495579698174812908089435_i128,(-7012320207267952676707505990036892906_i128),(-138587280032446298485008983788740789448_i128)];
_4 = (-350330383_i32) as isize;
_7 = _8;
RET = !154207699276824130429152518017600791889_i128;
_12 = !6230770170073325731_i64;
_11 = [(-46_i8),102_i8,14_i8,44_i8,118_i8,67_i8];
_13.1 = !7_usize;
_13 = (6_usize, 18198202935253170953_usize, 65028_u16);
_9 = !_5;
_13.1 = '\u{b8140}' as usize;
_1 = 212_u8 as f32;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_12 = (-6773404783279872592_i64);
_13 = (4_usize, 6173822020664915632_usize, 43679_u16);
_7 = _8;
_13.2 = 393021363297008691_u64 as u16;
_12 = _1 as i64;
_14 = (-59_i8) as i64;
_13.2 = 10705_u16;
_14 = 193_u8 as i64;
_8 = _4 << _13.1;
_13.1 = _13.0;
Call(_7 = core::intrinsics::bswap(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.2 = 19849_u16;
_10 = _6 <= _5;
_13.0 = _13.1 / _13.1;
_13.2 = !64947_u16;
_13 = (0_usize, 6_usize, 48207_u16);
_4 = _7 - _7;
Goto(bb2)
}
bb2 = {
_9 = _7 != _7;
_6 = _7 != _4;
_18.2 = !14_u8;
_10 = _5;
_18.0 = !9227778643671601893_u64;
_9 = !_6;
_11 = [6_i8,53_i8,122_i8,(-49_i8),6_i8,90_i8];
_7 = _4 - _4;
_18.0 = (-117934870_i32) as u64;
RET = 345407619_u32 as i128;
_16 = !19205_i16;
_2 = !_10;
_18.0 = 16918879923651406340_u64 ^ 16015287610390241035_u64;
_10 = _2 > _5;
Call(RET = fn15(_6, _4, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = !_2;
_13 = (6_usize, 7_usize, 38340_u16);
RET = (-37025672107999513061867134010460543764_i128);
_21 = 1799345601_u32;
RET = 73992495172435063647477413735913403332_i128;
_19.1 = -_12;
_20 = [_21,_21,_21];
_18.0 = !5701177374448064110_u64;
_22.fld0 = ((-122_i8),);
_15 = core::ptr::addr_of_mut!(_22.fld0);
_20 = [_21,_21,_21];
_22.fld3.1 = _13.0 + _13.0;
_10 = _9;
_17 = core::ptr::addr_of!(_21);
_7 = !_4;
_22.fld2 = _7 + _4;
_8 = _18.2 as isize;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_22.fld3.2 = _13.2 + _13.2;
_22.fld2 = _16 as isize;
_22.fld2 = _16 as isize;
_1 = 95210592879740271040163002193011075702_u128 as f32;
Call(_18.1 = fn16(_5, _2, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = '\u{a4a92}' as isize;
_13 = (_22.fld3.1, _22.fld3.1, _22.fld3.2);
_22.fld0 = (10_i8,);
_8 = -_4;
_22.fld3 = (_13.1, _13.1, _13.2);
_18 = (10792636909034830412_u64, _15, 164_u8);
_19.1 = -_14;
_22.fld3.1 = !_13.1;
(*_15) = ((-87_i8),);
_22.fld3.2 = _13.2;
_1 = 262833820261053047468760464918812424706_u128 as f32;
_22.fld3.1 = !_13.0;
_3 = [RET,RET,RET,RET,RET,RET,RET];
RET = (-3132570360781082643116874474335930083_i128) - (-30748112535053353540198870385314252091_i128);
_13 = (_22.fld3.1, _22.fld3.0, _22.fld3.2);
_19.1 = _14 ^ _12;
_19.1 = _14;
_18.2 = 57_u8;
_14 = _18.2 as i64;
_22.fld3.0 = !_22.fld3.1;
_17 = core::ptr::addr_of!((*_17));
_14 = '\u{9a9e7}' as i64;
_15 = core::ptr::addr_of_mut!(_22.fld0);
_9 = _5;
(*_17) = 2419885773_u32;
_22.fld3 = (_13.1, _13.1, _13.2);
(*_15).0 = _6 as i8;
(*_15) = (92_i8,);
_18 = (446252898847383901_u64, _15, 10_u8);
_14 = _12 - _12;
Goto(bb5)
}
bb5 = {
_24 = [_14,_12,_12,_19.1,_19.1,_14,_19.1,_19.1];
_17 = core::ptr::addr_of!((*_17));
(*_17) = _9 as u32;
_18.1 = _15;
_9 = _10;
_16 = !(-12467_i16);
_25 = RET & RET;
_13 = _22.fld3;
match _18.0 {
446252898847383901 => bb6,
_ => bb2
}
}
bb6 = {
_22.fld3.0 = 209643898661355983839129572102204301336_u128 as usize;
(*_15).0 = (-2_i8);
_19.2 = _4 as i8;
RET = _25 | _25;
_4 = _8;
_22.fld1 = '\u{c80ac}';
_6 = !_2;
_12 = _14 - _14;
_14 = RET as i64;
(*_15) = (_19.2,);
_6 = !_5;
_13.0 = _22.fld3.1;
_25 = _18.2 as i128;
_27 = _14 | _14;
_20 = [(*_17),(*_17),_21];
_31.1 = [_22.fld1,_22.fld1,_22.fld1,_22.fld1];
_12 = _14;
(*_17) = 876776396_u32 << (*_15).0;
_29 = _16 as f32;
_31.2 = _18.0 as i128;
_29 = _1 + _1;
_31.2 = RET;
match _18.0 {
0 => bb1,
1 => bb7,
2 => bb8,
446252898847383901 => bb10,
_ => bb9
}
}
bb7 = {
_9 = _7 != _7;
_6 = _7 != _4;
_18.2 = !14_u8;
_10 = _5;
_18.0 = !9227778643671601893_u64;
_9 = !_6;
_11 = [6_i8,53_i8,122_i8,(-49_i8),6_i8,90_i8];
_7 = _4 - _4;
_18.0 = (-117934870_i32) as u64;
RET = 345407619_u32 as i128;
_16 = !19205_i16;
_2 = !_10;
_18.0 = 16918879923651406340_u64 ^ 16015287610390241035_u64;
_10 = _2 > _5;
Call(RET = fn15(_6, _4, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_7 = '\u{a4a92}' as isize;
_13 = (_22.fld3.1, _22.fld3.1, _22.fld3.2);
_22.fld0 = (10_i8,);
_8 = -_4;
_22.fld3 = (_13.1, _13.1, _13.2);
_18 = (10792636909034830412_u64, _15, 164_u8);
_19.1 = -_14;
_22.fld3.1 = !_13.1;
(*_15) = ((-87_i8),);
_22.fld3.2 = _13.2;
_1 = 262833820261053047468760464918812424706_u128 as f32;
_22.fld3.1 = !_13.0;
_3 = [RET,RET,RET,RET,RET,RET,RET];
RET = (-3132570360781082643116874474335930083_i128) - (-30748112535053353540198870385314252091_i128);
_13 = (_22.fld3.1, _22.fld3.0, _22.fld3.2);
_19.1 = _14 ^ _12;
_19.1 = _14;
_18.2 = 57_u8;
_14 = _18.2 as i64;
_22.fld3.0 = !_22.fld3.1;
_17 = core::ptr::addr_of!((*_17));
_14 = '\u{9a9e7}' as i64;
_15 = core::ptr::addr_of_mut!(_22.fld0);
_9 = _5;
(*_17) = 2419885773_u32;
_22.fld3 = (_13.1, _13.1, _13.2);
(*_15).0 = _6 as i8;
(*_15) = (92_i8,);
_18 = (446252898847383901_u64, _15, 10_u8);
_14 = _12 - _12;
Goto(bb5)
}
bb9 = {
_10 = !_2;
_13 = (6_usize, 7_usize, 38340_u16);
RET = (-37025672107999513061867134010460543764_i128);
_21 = 1799345601_u32;
RET = 73992495172435063647477413735913403332_i128;
_19.1 = -_12;
_20 = [_21,_21,_21];
_18.0 = !5701177374448064110_u64;
_22.fld0 = ((-122_i8),);
_15 = core::ptr::addr_of_mut!(_22.fld0);
_20 = [_21,_21,_21];
_22.fld3.1 = _13.0 + _13.0;
_10 = _9;
_17 = core::ptr::addr_of!(_21);
_7 = !_4;
_22.fld2 = _7 + _4;
_8 = _18.2 as isize;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_22.fld3.2 = _13.2 + _13.2;
_22.fld2 = _16 as isize;
_22.fld2 = _16 as isize;
_1 = 95210592879740271040163002193011075702_u128 as f32;
Call(_18.1 = fn16(_5, _2, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_17 = core::ptr::addr_of!(_21);
_22.fld2 = _4 - _4;
_30.0 = [214209140531393900546150392128023053129_u128,234122651315225581654515399746620028390_u128,79788700517358370268209522308739692853_u128,122687379880667628372173789387365651626_u128,139139039778985673559826095588464666288_u128,114278451184173449423926111327194349944_u128,23524514839723021841854795234156981254_u128];
_32 = [_22.fld1,_22.fld1,_22.fld1,_22.fld1];
_21 = !3003917809_u32;
_22.fld3.0 = _29 as usize;
_9 = _2 | _6;
_19.1 = -_12;
_10 = _9;
_31.2 = _25 + RET;
_19.1 = _27 ^ _27;
(*_15) = (_19.2,);
_33 = (_18.0, _18.1, _18.2);
_33 = (_18.0, _18.1, _18.2);
RET = _31.2;
_26 = _19.1 as u32;
(*_17) = _26 & _26;
_11 = [_22.fld0.0,_22.fld0.0,_19.2,(*_15).0,(*_15).0,_19.2];
_24 = [_14,_19.1,_12,_19.1,_19.1,_19.1,_19.1,_27];
_19.1 = _27;
(*_15) = (_19.2,);
Goto(bb11)
}
bb11 = {
Call(_35 = dump_var(14_usize, 9_usize, Move(_9), 2_usize, Move(_2), 24_usize, Move(_24), 21_usize, Move(_21)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_35 = dump_var(14_usize, 25_usize, Move(_25), 7_usize, Move(_7), 32_usize, Move(_32), 5_usize, Move(_5)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_35 = dump_var(14_usize, 27_usize, Move(_27), 12_usize, Move(_12), 36_usize, _36, 36_usize, _36), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: isize,mut _3: isize) -> i128 {
mir! {
type RET = i128;
let _4: ((isize,), (i8,), u64);
let _5: bool;
let _6: isize;
let _7: [i128; 4];
let _8: f64;
let _9: ();
let _10: ();
{
_4.1.0 = 108_i8 ^ 0_i8;
_4.1 = (60_i8,);
_4.2 = _1 as u64;
_4.0 = (_3,);
_4.1.0 = 17960_i16 as i8;
RET = (-91761895049179975128267878240075679921_i128);
match RET {
248520471871758488335106729191692531535 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = (-1405790834237519102013271676233696597_i128) - (-117853022137237446186814659970189633983_i128);
_5 = _1;
RET = _5 as i128;
_4.1.0 = -(-48_i8);
_4.1.0 = !(-47_i8);
RET = (-74274158635431785917828258745859056755_i128);
_2 = RET as isize;
RET = !81827991034986572626642030075689884589_i128;
_6 = _4.0.0 ^ _3;
_2 = _4.0.0 & _4.0.0;
RET = (-45764894551389690010182964247889924249_i128);
_1 = _5;
_7 = [RET,RET,RET,RET];
_4.0.0 = _6 + _3;
_4.2 = 10186380214114560479_u64 << _3;
_4.2 = !6563350873570078692_u64;
_4.0.0 = _3;
_1 = !_5;
RET = 160724756468193653403607978398886306181_i128 << _3;
_8 = 17767958697320661363_usize as f64;
_3 = _6 + _2;
_5 = !_1;
_6 = _2 - _4.0.0;
Goto(bb3)
}
bb3 = {
Call(_9 = dump_var(15_usize, 7_usize, Move(_7), 6_usize, Move(_6), 1_usize, Move(_1), 10_usize, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: bool,mut _3: isize) -> *mut (i8,) {
mir! {
type RET = *mut (i8,);
let _4: Adt52;
let _5: Adt59;
let _6: [u32; 3];
let _7: [i128; 4];
let _8: Adt58;
let _9: char;
let _10: f32;
let _11: [u32; 3];
let _12: f64;
let _13: [u128; 7];
let _14: [char; 3];
let _15: ((isize,),);
let _16: u8;
let _17: u128;
let _18: (isize,);
let _19: (i8,);
let _20: Adt47;
let _21: ();
let _22: ();
{
_1 = _3 > _3;
_3 = -9223372036854775807_isize;
_2 = !_1;
_2 = !_1;
_2 = _1 < _1;
_1 = _2 ^ _2;
_3 = (-2542_i16) as isize;
_3 = 9223372036854775807_isize;
_1 = _2 > _2;
_1 = _2 > _2;
_1 = _2 != _2;
_3 = (-107_isize);
_1 = _2 <= _2;
_3 = 24_isize & (-32_isize);
_1 = !_2;
_3 = (-9223372036854775808_isize);
_6 = [2574404649_u32,1323749695_u32,2325309126_u32];
_6 = [677126409_u32,2214035461_u32,156450728_u32];
_1 = !_2;
_7 = [(-139885211404354495174621219127017277295_i128),(-153650533678936128002795458065528582150_i128),44523266406537686115709329014200477543_i128,(-150003827323819296572459988363155649433_i128)];
_7 = [96189456432069544733980481282965696301_i128,(-100365547452515056952474823117744579523_i128),(-156364975338993484894969835751289242782_i128),151388033239171539481846345333787924241_i128];
_1 = _2 ^ _2;
_1 = !_2;
Goto(bb1)
}
bb1 = {
_3 = 9223372036854775807_isize;
_6 = [2518306381_u32,372651129_u32,2417108214_u32];
_3 = 242705027_u32 as isize;
_2 = !_1;
_2 = _1;
_9 = '\u{baa25}';
_3 = 122_isize;
_7 = [(-51715296676343609309117932136293586008_i128),(-79001609391088846043265198893776851248_i128),31591700411455835492273168423934469808_i128,(-166422955080724529915989717584608841_i128)];
_10 = 51_u8 as f32;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
122 => bb6,
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
_7 = [26304038797319503857793862018677557687_i128,29994970408566658997791402616641888705_i128,23006961812548369354188220104497773265_i128,162753197560123007129101688746889553395_i128];
_7 = [(-67285955078367394472910249230533363467_i128),118177618583596469101832118644755119672_i128,(-10817323670201874714668911615415823678_i128),140726695540993488287445397077584395407_i128];
_6 = [1635217955_u32,1022832256_u32,2250686921_u32];
_11 = [2177232710_u32,3002902034_u32,3180657622_u32];
_9 = '\u{bd54e}';
_9 = '\u{64380}';
_7 = [(-20630502363524029289363839088909163588_i128),(-79229448671740762868454875100445984196_i128),(-77587365590439653409648240230911909107_i128),160622276244213578436548881151573170050_i128];
_6 = [1667441579_u32,742287148_u32,1715698999_u32];
_12 = 6_u8 as f64;
_1 = !_2;
_2 = _1 ^ _1;
_3 = 14516_u16 as isize;
_11 = [2546730512_u32,3623998908_u32,3570447426_u32];
_3 = 9223372036854775807_isize + 9223372036854775807_isize;
_7 = [100125558042110541034987221026911091298_i128,(-164208474333725261821061398036381545390_i128),71685305905952496758025784142648439402_i128,(-105216523447600728058372341621644153_i128)];
_3 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_2 = _1 <= _1;
_7 = [(-148859759189031457864664263954320602879_i128),(-31812307905071618225127824074348146191_i128),118893181375361445759618277911644017297_i128,(-45113507203656338615973326579744390405_i128)];
_3 = 62_isize;
match _3 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
62 => bb12,
_ => bb11
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
Return()
}
bb11 = {
_3 = 9223372036854775807_isize;
_6 = [2518306381_u32,372651129_u32,2417108214_u32];
_3 = 242705027_u32 as isize;
_2 = !_1;
_2 = _1;
_9 = '\u{baa25}';
_3 = 122_isize;
_7 = [(-51715296676343609309117932136293586008_i128),(-79001609391088846043265198893776851248_i128),31591700411455835492273168423934469808_i128,(-166422955080724529915989717584608841_i128)];
_10 = 51_u8 as f32;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
122 => bb6,
_ => bb5
}
}
bb12 = {
_9 = '\u{ec378}';
_10 = 123_u8 as f32;
_11 = [3824060598_u32,2834436658_u32,1993077911_u32];
_12 = 40041_u16 as f64;
_7 = [(-165005031075777947376614079495897401924_i128),57294870963210022413830574911346603721_i128,(-81352069947043006708450190798800916584_i128),(-68013183068730176121114106044567151223_i128)];
_12 = (-1563413269_i32) as f64;
_10 = 14323169766370596755_u64 as f32;
_13 = [259521917042314648262670318225466960_u128,38450151395584338056336211992068400936_u128,15380596036784036378504923174809198178_u128,242176752656953860469789673268078563848_u128,316767421618444434279137859269280168255_u128,205117419690530823841414937459906240025_u128,331339576702283004794587915530594459134_u128];
_12 = 15720189162685384790313206962763113319_i128 as f64;
_7 = [(-7386439743130990313812228771699929743_i128),(-57239143146239677458393234884632545455_i128),130430823165709331053956083800854108970_i128,157292440612442822015718766030180530257_i128];
_3 = (-9223372036854775808_isize);
_11 = [1900633815_u32,4248911479_u32,2148611069_u32];
_15.0 = (_3,);
_11 = [909180861_u32,3328720136_u32,2431523876_u32];
_6 = [971272240_u32,444050332_u32,900354856_u32];
_2 = _1 <= _1;
_12 = _10 as f64;
_2 = _1;
_14 = [_9,_9,_9];
_9 = '\u{482cf}';
_1 = _2;
_13 = [212322490253730976757336250284163940519_u128,222865891850406932458245892674605071410_u128,257018135895023596976153338525557662710_u128,311594835176904341341344618391005320423_u128,134473936428587280652174187422836323430_u128,298438001171436533789342807225150675453_u128,213605688730821233971183509883157725509_u128];
_11 = _6;
_14 = [_9,_9,_9];
_1 = _2 > _2;
_10 = 1262348456_i32 as f32;
_15.0 = (_3,);
_11 = _6;
Goto(bb13)
}
bb13 = {
_18.0 = _3 - _15.0.0;
_12 = 286332743976065204125998333271103331000_u128 as f64;
_9 = '\u{3cf54}';
_13 = [1405056591734757458233934589517204895_u128,1577140321596303667965033620577389345_u128,160226042164149826804244928867113413107_u128,125410309631387221512919348753931064489_u128,312548883768600779270619229981193754434_u128,337513413094825486842253747231284605489_u128,322156907846495574867567940954387999007_u128];
_12 = 9657753648239157064_u64 as f64;
_17 = !80206908873349940618844760686673085024_u128;
_18 = (_3,);
_15 = (_18,);
_6 = [3773050835_u32,3024895821_u32,1372760374_u32];
_18 = _15.0;
_17 = 322926673815890300238510553352039492304_u128;
_3 = _15.0.0 | _15.0.0;
_9 = '\u{10ab4a}';
_12 = (-22580_i16) as f64;
_16 = 88_u8 >> _17;
RET = core::ptr::addr_of_mut!(_19);
_7 = [(-113892027584241119798207128878854359687_i128),126459358105446861014059671070451951719_i128,144231197179259733053344389401632713614_i128,(-147697770035369281805067694945386887695_i128)];
_14 = [_9,_9,_9];
_19 = ((-121_i8),);
_7 = [(-111322359882605164777632193019573670257_i128),159886165368855726939245761874891813534_i128,110348890844966964428421945832697636916_i128,(-49514817568915251803413057921837257437_i128)];
_12 = 44764_u16 as f64;
(*RET).0 = _10 as i8;
_13 = [_17,_17,_17,_17,_17,_17,_17];
_16 = 82_u8;
RET = core::ptr::addr_of_mut!(_19);
_10 = _19.0 as f32;
RET = core::ptr::addr_of_mut!(_19);
(*RET) = (91_i8,);
_13 = [_17,_17,_17,_17,_17,_17,_17];
Call(_16 = core::intrinsics::bswap(49_u8), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET).0 = -(-56_i8);
_15 = (_18,);
_12 = 30836_i16 as f64;
_6 = [3146280190_u32,4292662120_u32,3717379528_u32];
_15.0.0 = _3;
(*RET).0 = !90_i8;
(*RET) = ((-74_i8),);
(*RET) = (114_i8,);
(*RET).0 = (-1822654997524819466_i64) as i8;
(*RET) = (75_i8,);
_16 = 58_u8 * 51_u8;
_12 = _15.0.0 as f64;
_7 = [26455750800811979753137659783641991146_i128,(-147042468887554561663610994397887750135_i128),(-85447901475373740572085949864037511673_i128),(-148468718672697177630725980027521514591_i128)];
(*RET) = ((-64_i8),);
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(16_usize, 18_usize, Move(_18), 13_usize, Move(_13), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(16_usize, 16_usize, Move(_16), 17_usize, Move(_17), 3_usize, Move(_3), 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [u32; 3],mut _6: isize) -> isize {
mir! {
type RET = isize;
let _7: isize;
let _8: Adt57;
let _9: *mut (usize, usize, u16);
let _10: i64;
let _11: (usize, [u64; 1], *mut (i8,));
let _12: isize;
let _13: [u64; 1];
let _14: Adt59;
let _15: f32;
let _16: i32;
let _17: [u64; 1];
let _18: isize;
let _19: (usize, usize, u16);
let _20: (isize,);
let _21: ();
let _22: ();
{
_5 = [3811325231_u32,3205682066_u32,4105694271_u32];
_5 = [3507225081_u32,1177201821_u32,2246808205_u32];
_5 = [3034419291_u32,1516824256_u32,1843215066_u32];
_3 = 2505143718893062663_usize as isize;
RET = -_1;
_5 = [2394245779_u32,974433516_u32,4134364654_u32];
_4 = (-118682246163971670111408907559942446645_i128) as isize;
Goto(bb1)
}
bb1 = {
_8.fld3.1 = !2_usize;
_8.fld3.2 = 15855_u16 << _6;
_8.fld3.0 = _8.fld3.1 - _8.fld3.1;
_8.fld3.2 = 1033493031_u32 as u16;
_4 = _2 ^ RET;
RET = -_2;
_8.fld0 = ((-11_i8),);
_5 = [779080129_u32,2936788068_u32,109100371_u32];
_5 = [442953605_u32,4172891077_u32,1254826107_u32];
_6 = (-6634_i16) as isize;
_6 = 180515826_u32 as isize;
match _8.fld0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211445 => bb7,
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
_8.fld3 = (2_usize, 1175235920217938802_usize, 62275_u16);
_5 = [3414431649_u32,449027837_u32,1543508352_u32];
_8.fld2 = 8180257509218125303_i64 as isize;
_8.fld3.0 = _8.fld3.1;
_8.fld1 = '\u{9b173}';
_8.fld3.1 = _8.fld3.0 / _8.fld3.0;
_8.fld0.0 = _8.fld1 as i8;
_8.fld3.2 = 51700_u16 * 55018_u16;
_11.0 = !_8.fld3.0;
_11.2 = core::ptr::addr_of_mut!(_8.fld0);
_13 = [5279332993220376891_u64];
_7 = _3 * _2;
_8.fld3 = (_11.0, _11.0, 49315_u16);
_1 = 113670476839535586074150603087964923786_i128 as isize;
_9 = core::ptr::addr_of_mut!(_8.fld3);
_11.1 = [9537062152023137943_u64];
_8.fld0.0 = 79_i8 << _8.fld3.1;
(*_9).2 = 32687_u16;
(*_9).0 = (*_9).1 >> (*_9).1;
_13 = [5811745688690133911_u64];
Goto(bb8)
}
bb8 = {
_1 = 4933972234286049293808670193040215391_i128 as isize;
_4 = (*_9).2 as isize;
(*_9).2 = !5622_u16;
_5 = [3647733854_u32,4266069558_u32,4233676563_u32];
(*_9).0 = (*_9).1;
_8.fld3.0 = _8.fld0.0 as usize;
(*_9) = (_11.0, _11.0, 51769_u16);
_5 = [3814345373_u32,3840450914_u32,1425397850_u32];
_9 = core::ptr::addr_of_mut!(_8.fld3);
_8.fld0.0 = 17_i8 | (-96_i8);
_1 = _3;
_9 = core::ptr::addr_of_mut!((*_9));
match (*_9).2 {
0 => bb3,
1 => bb9,
51769 => bb11,
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
_16 = 565686382_i32;
match _16 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb12,
5 => bb13,
565686382 => bb15,
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
_8.fld3.1 = !2_usize;
_8.fld3.2 = 15855_u16 << _6;
_8.fld3.0 = _8.fld3.1 - _8.fld3.1;
_8.fld3.2 = 1033493031_u32 as u16;
_4 = _2 ^ RET;
RET = -_2;
_8.fld0 = ((-11_i8),);
_5 = [779080129_u32,2936788068_u32,109100371_u32];
_5 = [442953605_u32,4172891077_u32,1254826107_u32];
_6 = (-6634_i16) as isize;
_6 = 180515826_u32 as isize;
match _8.fld0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211445 => bb7,
_ => bb6
}
}
bb15 = {
_3 = true as isize;
(*_9).2 = 37485_u16;
(*_9).1 = _8.fld3.0;
_8.fld0 = ((-12_i8),);
(*_9).0 = (-6942518312207441333_i64) as usize;
RET = _3;
_8.fld3.2 = _8.fld1 as u16;
_8.fld3.0 = !_8.fld3.1;
(*_9).2 = 60100_u16 - 2902_u16;
_20 = (RET,);
_8.fld3.1 = _8.fld3.0 * (*_9).0;
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(17_usize, 1_usize, Move(_1), 3_usize, Move(_3), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(17_usize, 16_usize, Move(_16), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: (isize,),mut _2: isize,mut _3: ((isize,),),mut _4: (isize,),mut _5: f64) -> f32 {
mir! {
type RET = f32;
let _6: [i128; 7];
let _7: (usize, usize, u16);
let _8: Adt47;
let _9: isize;
let _10: i16;
let _11: Adt55;
let _12: f32;
let _13: [u32; 3];
let _14: (i8,);
let _15: Adt57;
let _16: [i8; 6];
let _17: [u128; 7];
let _18: (isize,);
let _19: [i8; 6];
let _20: Adt59;
let _21: Adt53;
let _22: u64;
let _23: (isize,);
let _24: isize;
let _25: ((isize,), (i8,), u64);
let _26: Adt51;
let _27: (usize, [u64; 1], *mut (i8,));
let _28: i32;
let _29: Adt47;
let _30: isize;
let _31: bool;
let _32: [i128; 7];
let _33: u16;
let _34: isize;
let _35: i128;
let _36: [i128; 7];
let _37: isize;
let _38: [char; 3];
let _39: &'static i8;
let _40: u16;
let _41: ();
let _42: ();
{
_6 = [107606086456288523142881705180540101079_i128,(-147070814035937463089239479874073437821_i128),(-24537884607590169985581748327638858786_i128),(-16855582577228002172282428996730705568_i128),(-65178165481245109949573798827231073171_i128),(-101481750640799563929598654366183986726_i128),(-103119028685375882185460738572450445116_i128)];
_7.1 = 8482_u16 as usize;
RET = (-35867189430033715039997864319583261601_i128) as f32;
_3 = (_1,);
_3 = (_4,);
_9 = (-14393874506493491636946428928115339962_i128) as isize;
_3.0 = (_9,);
_3 = (_1,);
_10 = 22299_i16;
_3.0.0 = _10 as isize;
_5 = 25_u8 as f64;
_6 = [(-152447203727927916813466568296806277074_i128),97268559028288710416348818944975860653_i128,(-59677881675544016017229660946168190837_i128),(-33351944993025633746424058319779943976_i128),(-118760508136370256785963065511199432277_i128),111303068496073730078540030597699878890_i128,146427484189311026624751714798810513613_i128];
_7 = (2_usize, 4048662354853232469_usize, 42506_u16);
RET = 838053185_i32 as f32;
_6 = [80618560216202873540098059415162923924_i128,157428829236008596447326715379337067037_i128,132254279061371289612439697637226672397_i128,149194904546757674784305997704364427846_i128,139766634305984761290376460875105267212_i128,15723735031605376809930226552265012940_i128,(-116329053330787032645280318321637106839_i128)];
_5 = _7.2 as f64;
RET = 158430371131821092152283684059282840407_u128 as f32;
_3 = (_1,);
RET = _4.0 as f32;
Goto(bb1)
}
bb1 = {
_4.0 = _3.0.0 << _1.0;
_7.0 = !_7.1;
_5 = 1837262113_i32 as f64;
Call(_1.0 = core::intrinsics::transmute(_3.0.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.fld3.2 = _7.2 << _4.0;
RET = _5 as f32;
_3 = (_1,);
_14 = ((-112_i8),);
Goto(bb3)
}
bb3 = {
_3 = (_4,);
_12 = RET;
_7.2 = !_15.fld3.2;
_15 = Adt57 { fld0: _14,fld1: '\u{c72fc}',fld2: _9,fld3: _7 };
_1 = (_3.0.0,);
_5 = _15.fld0.0 as f64;
_4.0 = _1.0;
_15.fld0.0 = _14.0;
_16 = [_14.0,_15.fld0.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_16 = [_15.fld0.0,_14.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_3 = (_1,);
_17 = [245474541428768206828112504550551255272_u128,235920067247439107190347529000847925317_u128,29236907659704207008042221554122195397_u128,214376649058299464853158025774246459042_u128,62273403624695090221846440678845313299_u128,294770530619435286351335778467323002036_u128,150445697201813389789859771006880819972_u128];
_3 = (_1,);
_17 = [114971990138193099284663671147982053566_u128,321522470976488668756998112661282610636_u128,232002989866964871398738567734546169923_u128,239797675367926864350751697477365134240_u128,210270903815736196313660329661504310370_u128,185432799969144383685892002175695723767_u128,327102560771309254208882100413946400604_u128];
RET = _12 * _12;
_4.0 = 2216702925_u32 as isize;
_5 = _14.0 as f64;
_11 = Adt55::Variant0 { fld0: _7.0 };
_15.fld1 = '\u{90fd2}';
_14 = (_15.fld0.0,);
_7.0 = _15.fld3.0 ^ Field::<usize>(Variant(_11, 0), 0);
Call(_15.fld3.2 = core::intrinsics::transmute(_10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.0 = _3.0.0;
_15 = Adt57 { fld0: _14,fld1: '\u{58030}',fld2: _2,fld3: _7 };
match _10 {
0 => bb3,
1 => bb5,
22299 => bb7,
_ => bb6
}
}
bb5 = {
_3 = (_4,);
_12 = RET;
_7.2 = !_15.fld3.2;
_15 = Adt57 { fld0: _14,fld1: '\u{c72fc}',fld2: _9,fld3: _7 };
_1 = (_3.0.0,);
_5 = _15.fld0.0 as f64;
_4.0 = _1.0;
_15.fld0.0 = _14.0;
_16 = [_14.0,_15.fld0.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_16 = [_15.fld0.0,_14.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_3 = (_1,);
_17 = [245474541428768206828112504550551255272_u128,235920067247439107190347529000847925317_u128,29236907659704207008042221554122195397_u128,214376649058299464853158025774246459042_u128,62273403624695090221846440678845313299_u128,294770530619435286351335778467323002036_u128,150445697201813389789859771006880819972_u128];
_3 = (_1,);
_17 = [114971990138193099284663671147982053566_u128,321522470976488668756998112661282610636_u128,232002989866964871398738567734546169923_u128,239797675367926864350751697477365134240_u128,210270903815736196313660329661504310370_u128,185432799969144383685892002175695723767_u128,327102560771309254208882100413946400604_u128];
RET = _12 * _12;
_4.0 = 2216702925_u32 as isize;
_5 = _14.0 as f64;
_11 = Adt55::Variant0 { fld0: _7.0 };
_15.fld1 = '\u{90fd2}';
_14 = (_15.fld0.0,);
_7.0 = _15.fld3.0 ^ Field::<usize>(Variant(_11, 0), 0);
Call(_15.fld3.2 = core::intrinsics::transmute(_10), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_15.fld3.2 = _7.2 << _4.0;
RET = _5 as f32;
_3 = (_1,);
_14 = ((-112_i8),);
Goto(bb3)
}
bb7 = {
_18 = (_1.0,);
_7.2 = _15.fld3.2;
_3 = (_18,);
_7.1 = _14.0 as usize;
_7.0 = !_15.fld3.1;
_15.fld0.0 = -_14.0;
_7.1 = !_7.0;
_7.1 = !_7.0;
_7.1 = !_15.fld3.0;
_7 = (Field::<usize>(Variant(_11, 0), 0), _15.fld3.0, _15.fld3.2);
_16 = [_14.0,_14.0,_15.fld0.0,_14.0,_14.0,_14.0];
_22 = 1345553969397651771_u64 >> _15.fld3.2;
_2 = _18.0 >> _7.2;
RET = _12 - _12;
_24 = _18.0 - _3.0.0;
_23 = (_3.0.0,);
_1.0 = -_23.0;
_4 = _3.0;
RET = _12;
_19 = [_14.0,_14.0,_14.0,_15.fld0.0,_14.0,_14.0];
Goto(bb8)
}
bb8 = {
_22 = 12796554147375123681_u64;
SetDiscriminant(_11, 0);
_13 = [3916273362_u32,1889083477_u32,26283826_u32];
_25.2 = _22 + _22;
_16 = [_14.0,_15.fld0.0,_14.0,_15.fld0.0,_15.fld0.0,_14.0];
_25.0 = (_23.0,);
_25.2 = !_22;
_27.0 = _7.0;
_25.1.0 = true as i8;
_16 = [_15.fld0.0,_15.fld0.0,_15.fld0.0,_25.1.0,_15.fld0.0,_15.fld0.0];
_2 = -_24;
_25.0 = (_4.0,);
_27.0 = _15.fld3.0;
_7.2 = !_15.fld3.2;
_25.1.0 = _14.0 + _15.fld0.0;
_19 = [_14.0,_15.fld0.0,_25.1.0,_25.1.0,_25.1.0,_25.1.0];
_22 = _25.2;
place!(Field::<usize>(Variant(_11, 0), 0)) = !_27.0;
_4.0 = -_24;
_7.0 = _7.1;
_27.2 = core::ptr::addr_of_mut!(_15.fld0);
_23.0 = !_4.0;
_9 = _24 << _15.fld3.1;
Goto(bb9)
}
bb9 = {
_12 = RET * RET;
_4 = (_18.0,);
RET = _7.0 as f32;
_14.0 = _25.1.0 + _25.1.0;
_7.2 = _15.fld3.2 | _15.fld3.2;
_7.0 = _7.1 / _15.fld3.1;
_13 = [3482071555_u32,692453765_u32,856671866_u32];
_7.1 = _27.0 + _7.0;
_5 = _22 as f64;
_27.1 = [_25.2];
_2 = 1743834971_i32 as isize;
_7.1 = _7.2 as usize;
_3 = (_23,);
_14.0 = _10 as i8;
_3.0.0 = (-2095029632_i32) as isize;
_7.0 = _7.1 & _7.1;
_31 = !false;
_25.2 = !_22;
_14.0 = _25.1.0 * _15.fld0.0;
_15.fld0 = (_25.1.0,);
_15 = Adt57 { fld0: _25.1,fld1: '\u{1cc49}',fld2: _2,fld3: _7 };
place!(Field::<usize>(Variant(_11, 0), 0)) = !_15.fld3.0;
_7.0 = Field::<usize>(Variant(_11, 0), 0) ^ _15.fld3.0;
Goto(bb10)
}
bb10 = {
_25.0.0 = 631783843_u32 as isize;
_25.1 = _14;
_14 = _15.fld0;
_15.fld0.0 = !_25.1.0;
_25 = (_23, _14, _22);
_25 = (_1, _15.fld0, _22);
_6 = [151655801290677274458098948219445396952_i128,(-61178858449954880038540503682638812745_i128),26163428414451573562225185722261868305_i128,132603047121673473851601882679686534017_i128,(-144023911746016098258972936445477170714_i128),(-147406680739158866343574401661324790639_i128),(-94853971763339981359253859281362192004_i128)];
_24 = _23.0;
RET = _12 * _12;
_25.2 = _22;
match _10 {
0 => bb11,
1 => bb12,
2 => bb13,
22299 => bb15,
_ => bb14
}
}
bb11 = {
_3 = (_4,);
_12 = RET;
_7.2 = !_15.fld3.2;
_15 = Adt57 { fld0: _14,fld1: '\u{c72fc}',fld2: _9,fld3: _7 };
_1 = (_3.0.0,);
_5 = _15.fld0.0 as f64;
_4.0 = _1.0;
_15.fld0.0 = _14.0;
_16 = [_14.0,_15.fld0.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_16 = [_15.fld0.0,_14.0,_15.fld0.0,_14.0,_15.fld0.0,_14.0];
_3 = (_1,);
_17 = [245474541428768206828112504550551255272_u128,235920067247439107190347529000847925317_u128,29236907659704207008042221554122195397_u128,214376649058299464853158025774246459042_u128,62273403624695090221846440678845313299_u128,294770530619435286351335778467323002036_u128,150445697201813389789859771006880819972_u128];
_3 = (_1,);
_17 = [114971990138193099284663671147982053566_u128,321522470976488668756998112661282610636_u128,232002989866964871398738567734546169923_u128,239797675367926864350751697477365134240_u128,210270903815736196313660329661504310370_u128,185432799969144383685892002175695723767_u128,327102560771309254208882100413946400604_u128];
RET = _12 * _12;
_4.0 = 2216702925_u32 as isize;
_5 = _14.0 as f64;
_11 = Adt55::Variant0 { fld0: _7.0 };
_15.fld1 = '\u{90fd2}';
_14 = (_15.fld0.0,);
_7.0 = _15.fld3.0 ^ Field::<usize>(Variant(_11, 0), 0);
Call(_15.fld3.2 = core::intrinsics::transmute(_10), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_15.fld3.2 = _7.2 << _4.0;
RET = _5 as f32;
_3 = (_1,);
_14 = ((-112_i8),);
Goto(bb3)
}
bb13 = {
_18 = (_1.0,);
_7.2 = _15.fld3.2;
_3 = (_18,);
_7.1 = _14.0 as usize;
_7.0 = !_15.fld3.1;
_15.fld0.0 = -_14.0;
_7.1 = !_7.0;
_7.1 = !_7.0;
_7.1 = !_15.fld3.0;
_7 = (Field::<usize>(Variant(_11, 0), 0), _15.fld3.0, _15.fld3.2);
_16 = [_14.0,_14.0,_15.fld0.0,_14.0,_14.0,_14.0];
_22 = 1345553969397651771_u64 >> _15.fld3.2;
_2 = _18.0 >> _7.2;
RET = _12 - _12;
_24 = _18.0 - _3.0.0;
_23 = (_3.0.0,);
_1.0 = -_23.0;
_4 = _3.0;
RET = _12;
_19 = [_14.0,_14.0,_14.0,_15.fld0.0,_14.0,_14.0];
Goto(bb8)
}
bb14 = {
_15.fld3.2 = _7.2 << _4.0;
RET = _5 as f32;
_3 = (_1,);
_14 = ((-112_i8),);
Goto(bb3)
}
bb15 = {
_30 = _9;
_33 = _15.fld3.2 | _7.2;
SetDiscriminant(_11, 0);
_19 = _16;
_27.0 = _7.1;
_19 = [_14.0,_25.1.0,_14.0,_25.1.0,_25.1.0,_15.fld0.0];
_18.0 = -_9;
_5 = _4.0 as f64;
_25 = (_1, _14, _22);
_15.fld3.2 = _33;
_25.1.0 = -_14.0;
_39 = &_25.1.0;
Goto(bb16)
}
bb16 = {
Call(_41 = dump_var(18_usize, 19_usize, Move(_19), 4_usize, Move(_4), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(18_usize, 9_usize, Move(_9), 23_usize, Move(_23), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(18_usize, 7_usize, Move(_7), 18_usize, Move(_18), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: ((isize,),),mut _3: (usize, usize, u16),mut _4: *mut (i8,),mut _5: u16,mut _6: isize,mut _7: ((isize,),),mut _8: [i128; 4],mut _9: (isize,),mut _10: bool,mut _11: usize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: [i8; 6];
let _14: Adt51;
let _15: isize;
let _16: char;
let _17: [i128; 7];
let _18: i128;
let _19: [i64; 8];
let _20: Adt57;
let _21: [i128; 7];
let _22: Adt48;
let _23: i16;
let _24: ();
let _25: ();
{
RET = -_12;
_7.0.0 = 3262767173_u32 as isize;
_2 = (_9,);
(*_4).0 = !(-90_i8);
(*_4).0 = (-24146_i16) as i8;
_2.0 = (_9.0,);
(*_4).0 = -(-13_i8);
_1 = _2.0.0 * _9.0;
_2 = _7;
_5 = !_3.2;
_7 = (_9,);
(*_4) = ((-46_i8),);
_5 = _3.2;
match _3.0 {
0 => bb1,
1 => bb2,
12955398217212539779 => bb4,
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
_9 = _7.0;
_12 = -_9.0;
_7.0 = (RET,);
_5 = 198_u8 as u16;
_5 = _3.2 + _3.2;
_10 = _1 < _9.0;
_3.2 = !_5;
_6 = _7.0.0 & _1;
_3.2 = (-1410340780607702957_i64) as u16;
_13 = [(*_4).0,(*_4).0,(*_4).0,(*_4).0,(*_4).0,(*_4).0];
_7 = (_9,);
_15 = 681390441_u32 as isize;
(*_4) = ((-120_i8),);
_7 = _2;
(*_4).0 = 5709697221533980188_u64 as i8;
_14 = Adt51::Variant1 { fld0: _4 };
_12 = _6;
_16 = '\u{be766}';
_8 = [80840348765491046666707754214668675334_i128,25008006144672739106253677386325612045_i128,(-131015878555256315400195929840786591860_i128),99839685539744154392923185083661579008_i128];
(*_4) = (94_i8,);
_1 = _12 + _6;
(*_4) = (19_i8,);
_7.0 = (_12,);
_18 = 62111162668727254360852919452573645373_i128 << _3.1;
_2.0 = (_1,);
match _3.0 {
0 => bb3,
1 => bb2,
2 => bb5,
12955398217212539779 => bb7,
_ => bb6
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_1 = _2.0.0 | _12;
_11 = _3.1 + _3.0;
_15 = !_1;
_7.0.0 = !_15;
_17 = [_18,_18,_18,_18,_18,_18,_18];
_20.fld3.0 = _18 as usize;
_8 = [_18,_18,_18,_18];
_3.1 = _18 as usize;
_20.fld3.2 = _5 * _5;
SetDiscriminant(_14, 3);
_7 = (_2.0,);
_6 = _1 - _15;
_20.fld3.1 = _18 as usize;
_20.fld2 = _18 as isize;
place!(Field::<((isize,), (i8,), u64)>(Variant(_14, 3), 2)).0.0 = _11 as isize;
_3.2 = 154895885_u32 as u16;
_6 = !_7.0.0;
Call(_11 = core::intrinsics::bswap(_20.fld3.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<((isize,), (i8,), u64)>(Variant(_14, 3), 2)).0.0 = 134441026120762712287605078548894793258_u128 as isize;
_11 = _20.fld3.0;
_7.0.0 = -_2.0.0;
place!(Field::<(isize,)>(Variant(_14, 3), 4)) = _9;
_20.fld2 = _15;
match _3.0 {
0 => bb1,
1 => bb7,
2 => bb3,
12955398217212539779 => bb9,
_ => bb4
}
}
bb9 = {
_3.2 = !_20.fld3.2;
place!(Field::<((isize,), (i8,), u64)>(Variant(_14, 3), 2)).0.0 = _7.0.0 * _1;
_11 = !_20.fld3.0;
place!(Field::<(usize, [u64; 1], *mut (i8,))>(Variant(_14, 3), 5)).1 = [17993762721743069539_u64];
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
12955398217212539779 => bb10,
_ => bb8
}
}
bb10 = {
_13 = [(*_4).0,(*_4).0,(*_4).0,(*_4).0,(*_4).0,(*_4).0];
_7.0 = (_6,);
_16 = '\u{446f5}';
place!(Field::<[char; 3]>(Variant(_14, 3), 0)) = [_16,_16,_16];
_21 = [_18,_18,_18,_18,_18,_18,_18];
RET = _15 << _7.0.0;
Goto(bb11)
}
bb11 = {
Call(_24 = dump_var(19_usize, 16_usize, Move(_16), 10_usize, Move(_10), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_24 = dump_var(19_usize, 2_usize, Move(_2), 6_usize, Move(_6), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{4e50f}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(122_i8), std::hint::black_box(14716_i16), std::hint::black_box(1449438322_i32), std::hint::black_box((-1457767712986217890_i64)), std::hint::black_box(324119951646736525134431136139622151586_u128), std::hint::black_box(14792300861940640144_usize), std::hint::black_box(59_u8), std::hint::black_box(32961_u16), std::hint::black_box(3785836592_u32));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *const (i8,),
fld1: *const u32,
fld2: [char; 4],
fld3: (usize, [u64; 1], *mut (i8,)),

},
Variant1{
fld0: *const (i8,),
fld1: *mut (i8,),
fld2: isize,
fld3: (u64, *mut (i8,), u8),
fld4: u64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut bool,

},
Variant1{
fld0: *const [i8; 6],
fld1: i64,
fld2: (*const (i8,), i64, i8),
fld3: [i128; 4],

},
Variant2{
fld0: bool,
fld1: *mut (usize, usize, u16),
fld2: isize,

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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: i32,
fld1: [u32; 3],

},
Variant1{
fld0: ((isize,),),
fld1: (i8,),
fld2: isize,
fld3: [u64; 1],
fld4: u8,
fld5: *const u32,

},
Variant2{
fld0: (isize,),
fld1: [char; 3],
fld2: *mut (i8,),
fld3: u8,
fld4: [char; 4],
fld5: (i8,),
fld6: [i8; 6],
fld7: (*const (i8,), i64, i8),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt46,
fld1: [u64; 1],

},
Variant1{
fld0: [char; 4],
fld1: (isize,),
fld2: [u64; 1],
fld3: *const u32,
fld4: i16,
fld5: *mut bool,
fld6: ((isize,),),

},
Variant2{
fld0: [i8; 2],
fld1: usize,
fld2: u32,

},
Variant3{
fld0: (*const (i8,), i64, i8),
fld1: *mut bool,
fld2: Adt45,
fld3: [i64; 8],
fld4: (usize, [u64; 1], *mut (i8,)),
fld5: [char; 3],
fld6: [u128; 7],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: i128,
fld1: u16,
fld2: (usize, usize, u16),
fld3: ((isize,),),

},
Variant1{
fld0: u64,
fld1: usize,
fld2: isize,
fld3: *mut (i8,),
fld4: [u128; 7],

},
Variant2{
fld0: i16,
fld1: *mut bool,

},
Variant3{
fld0: [char; 4],

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u128,
fld1: ((isize,),),
fld2: usize,
fld3: i8,
fld4: i16,
fld5: [u128; 7],

},
Variant1{
fld0: *const ((isize,),),
fld1: char,
fld2: (usize, usize, u16),
fld3: u32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: *const u32,
fld1: [i128; 4],
fld2: *mut (usize, usize, u16),
fld3: [u32; 3],
fld4: [char; 4],
fld5: i32,
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [i128; 7],
fld1: i8,
fld2: ([u128; 7], [char; 4], i128),

},
Variant1{
fld0: *mut (i8,),

},
Variant2{
fld0: *mut (i8,),
fld1: [char; 4],

},
Variant3{
fld0: [char; 3],
fld1: (*const (i8,), i64, i8),
fld2: ((isize,), (i8,), u64),
fld3: usize,
fld4: (isize,),
fld5: (usize, [u64; 1], *mut (i8,)),
fld6: [i128; 4],
fld7: Adt49,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt44,
fld1: u16,

},
Variant1{
fld0: i8,
fld1: *const ((isize,),),
fld2: Adt45,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const u32,
fld1: (u64, *mut (i8,), u8),
fld2: Adt49,
fld3: [i8; 6],
fld4: (usize, [u64; 1], *mut (i8,)),

},
Variant1{
fld0: i8,

},
Variant2{
fld0: *const u32,
fld1: char,
fld2: (u64, *mut (i8,), u8),
fld3: (*const (i8,), i64, i8),
fld4: *mut (i8,),

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
fld0: Adt45,
fld1: char,
fld2: u32,

},
Variant1{
fld0: Adt52,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: usize,

},
Variant1{
fld0: *mut (i8,),
fld1: char,

},
Variant2{
fld0: [char; 3],
fld1: char,
fld2: u8,
fld3: ((isize,), (i8,), u64),
fld4: Adt46,
fld5: *const (i8,),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
fld0: *mut (i8,),
fld1: char,
fld2: ((isize,), (i8,), u64),

},
Variant1{
fld0: (isize,),
fld1: u8,
fld2: isize,
fld3: *const ((isize,),),
fld4: Adt48,
fld5: (u64, *mut (i8,), u8),
fld6: Adt51,

},
Variant2{
fld0: Adt54,
fld1: [u64; 1],
fld2: (usize, usize, u16),
fld3: u128,
fld4: [char; 3],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: (i8,),
fld1: char,
fld2: isize,
fld3: (usize, usize, u16),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [i128; 4],
fld1: Adt44,
fld2: (usize, usize, u16),
fld3: Adt49,
fld4: Adt53,
fld5: Adt47,

},
Variant1{
fld0: *mut (i8,),
fld1: Adt46,
fld2: [u128; 7],
fld3: u32,
fld4: [i8; 2],
fld5: Adt51,

},
Variant2{
fld0: [char; 4],
fld1: Adt57,
fld2: [u64; 1],
fld3: *mut (usize, usize, u16),
fld4: *const ((isize,),),
fld5: Adt48,

},
Variant3{
fld0: u16,
fld1: u128,
fld2: f32,
fld3: i8,
fld4: i16,
fld5: Adt44,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: Adt53,
fld1: *const u32,
fld2: [i8; 2],
fld3: *const ((isize,),),

},
Variant1{
fld0: ([u128; 7], [char; 4], i128),

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: *mut (usize, usize, u16),
fld1: [i128; 7],
fld2: Adt44,
fld3: Adt52,
fld4: [u64; 1],

},
Variant1{
fld0: Adt55,

},
Variant2{
fld0: Adt56,
fld1: u128,
fld2: Adt53,
fld3: Adt58,
fld4: *mut (i8,),

},
Variant3{
fld0: f32,
fld1: ((isize,),),
fld2: Adt58,
fld3: [u128; 7],

}}

