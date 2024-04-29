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
pub fn fn0(mut _1: bool,mut _2: u32,mut _3: isize,mut _4: u128,mut _5: u64,mut _6: i32,mut _7: i64,mut _8: i128) -> Adt56 {
mir! {
type RET = Adt56;
let _9: Adt47;
let _10: *mut i32;
let _11: u128;
let _12: Adt57;
let _13: isize;
let _14: f64;
let _15: usize;
let _16: f64;
let _17: ([u64; 8],);
let _18: ([u64; 8], *const bool, ([usize; 4], u8, isize));
let _19: isize;
let _20: i64;
let _21: f64;
let _22: u128;
let _23: i128;
let _24: bool;
let _25: [i64; 2];
let _26: ();
let _27: ();
{
_3 = 9223372036854775807_isize >> 19703_u16;
_2 = 7726965141463392809_i64 as u32;
_3 = 9223372036854775807_isize;
_6 = 55071_u16 as i32;
_2 = 1775941716_u32;
_8 = (-71400195098866409566001708926320460171_i128);
_4 = !186640856930190699159736872637305442012_u128;
_8 = 26578759424175525323240891288030156135_i128;
_8 = (-164695789132741788776462141170702477716_i128) << _6;
_6 = (-1112961424_i32) + (-923564773_i32);
_2 = 1221516248_u32 & 4119546935_u32;
_1 = _2 != _2;
_10 = core::ptr::addr_of_mut!(_6);
_1 = true;
_6 = (-85_i8) as i32;
_8 = 30306226847119830216438700529336051832_i128 << (*_10);
_10 = core::ptr::addr_of_mut!((*_10));
_3 = (-9223372036854775808_isize);
_4 = 111770773156923802554919558173425176157_u128 >> _3;
_2 = 188299968_u32;
_7 = 946204195144521012_i64;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
188299968 => bb9,
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
Goto(bb10)
}
bb10 = {
_5 = !14273670765692302717_u64;
_2 = 1391398790_u32;
_4 = 57_u8 as u128;
_2 = 2874428011_u32;
_13 = _3;
_5 = 60449_u16 as u64;
_1 = !false;
_5 = !2925751142631975186_u64;
(*_10) = _4 as i32;
_8 = -(-139852496628523627158070038728992481824_i128);
_6 = _1 as i32;
_11 = _8 as u128;
_11 = 16174582881553441516_usize as u128;
(*_10) = 983430579_i32;
_6 = (-1123053873_i32) & 1331699371_i32;
_1 = (*_10) <= _6;
_13 = !_3;
_11 = (*_10) as u128;
_1 = _7 >= _7;
_16 = 49_i8 as f64;
_14 = _16;
_15 = _8 as usize;
_8 = _5 as i128;
_10 = core::ptr::addr_of_mut!((*_10));
Call(_14 = fn1((*_10), _6, _13, _6, (*_10), _10, (*_10), _5, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = -_14;
_4 = _11;
_10 = core::ptr::addr_of_mut!(_6);
match _7 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
946204195144521012 => bb19,
_ => bb18
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
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_19 = _13;
_18.2.2 = _3;
(*_10) = 1472780702_i32 | 1438384727_i32;
(*_10) = !1795305248_i32;
_8 = 10041328476779273448657765781386280032_i128 - (-92909384629136209929644975989989888216_i128);
_17.0 = [_5,_5,_5,_5,_5,_5,_5,_5];
_18.1 = core::ptr::addr_of!(_1);
_13 = -_18.2.2;
_18.0 = [_5,_5,_5,_5,_5,_5,_5,_5];
_13 = _3 >> _6;
match _3 {
0 => bb1,
1 => bb17,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb20,
_ => bb12
}
}
bb20 = {
_15 = !4542206190951666322_usize;
_21 = _16 - _16;
_10 = core::ptr::addr_of_mut!((*_10));
_18.1 = core::ptr::addr_of!(_1);
_14 = _2 as f64;
_18.1 = core::ptr::addr_of!(_1);
_20 = _7 >> _2;
_4 = !_11;
_18.2.1 = !245_u8;
_8 = 40216852286032056810000109388091283990_i128;
_18.2.0 = [_15,_15,_15,_15];
_11 = _4;
_23 = (*_10) as i128;
_11 = _4;
_16 = -_21;
_16 = -_21;
RET = Adt56::Variant2 { fld0: _18.1 };
_18.0 = _17.0;
_13 = _18.2.2 | _3;
_21 = 45170_u16 as f64;
_23 = _8;
_25 = [_20,_20];
_14 = 33359_u16 as f64;
Goto(bb21)
}
bb21 = {
Call(_26 = dump_var(0_usize, 11_usize, Move(_11), 13_usize, Move(_13), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_26 = dump_var(0_usize, 6_usize, Move(_6), 5_usize, Move(_5), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i32,mut _2: i32,mut _3: isize,mut _4: i32,mut _5: i32,mut _6: *mut i32,mut _7: i32,mut _8: u64,mut _9: *mut i32) -> f64 {
mir! {
type RET = f64;
let _10: char;
let _11: i128;
let _12: isize;
let _13: i16;
let _14: [i16; 6];
let _15: [u8; 8];
let _16: u16;
let _17: (usize, i64);
let _18: Adt54;
let _19: Adt42;
let _20: char;
let _21: ([u64; 8],);
let _22: [i64; 2];
let _23: (usize, i64);
let _24: f32;
let _25: isize;
let _26: ([usize; 4], u8, isize);
let _27: char;
let _28: f32;
let _29: f64;
let _30: f32;
let _31: Adt43;
let _32: ();
let _33: ();
{
_6 = core::ptr::addr_of_mut!(_5);
(*_9) = -(*_6);
_4 = !_5;
_5 = _7 >> _2;
RET = _1 as f64;
_2 = (*_6) - (*_6);
_8 = !3381693115106224594_u64;
_2 = '\u{b7933}' as i32;
(*_6) = _4;
_3 = 85_isize;
(*_9) = true as i32;
(*_9) = (*_6) << _1;
_1 = _5 ^ _4;
_1 = (*_9);
(*_9) = (*_6);
_3 = 9223372036854775807_isize << _1;
_1 = !(*_6);
(*_6) = 40884_u16 as i32;
_3 = (-8_isize) ^ 48_isize;
_7 = 2606388394397555353_usize as i32;
_8 = 7863555008215967848_u64 | 436304026658848402_u64;
_4 = _1;
_10 = '\u{65fdc}';
_11 = 56630_u16 as i128;
_11 = 29686514721960729773870259985788206313_i128 >> _1;
_3 = (-55_isize);
(*_6) = -(*_9);
Call(_3 = fn2(_4, _6, _6, _10, _5, _6, (*_6), (*_9), _9, (*_9), (*_6), (*_9), (*_9)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_9) = (-122_i8) as i32;
(*_6) = _7;
(*_9) = !_4;
(*_9) = _4 + (*_6);
_9 = core::ptr::addr_of_mut!(_2);
_7 = _1;
_8 = 8777929371158395416_u64 << _4;
(*_6) = _7 ^ _2;
RET = 82_i8 as f64;
_10 = '\u{124e1}';
RET = (-5175270254739683253_i64) as f64;
_3 = 9223372036854775807_isize;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
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
_13 = _3 as i16;
(*_6) = -(*_9);
_14 = [_13,_13,_13,_13,_13,_13];
(*_9) = !_4;
_8 = (-60_i8) as u64;
_7 = -(*_9);
match _3 {
0 => bb6,
9223372036854775807 => bb11,
_ => bb3
}
}
bb11 = {
_14 = [_13,_13,_13,_13,_13,_13];
RET = 154_u8 as f64;
RET = (-1684433743434769055_i64) as f64;
(*_6) = _13 as i32;
_8 = 3049782717168126858_u64;
(*_9) = !_1;
_17.1 = 8465538777278845809_i64 * (-3177812272588461309_i64);
_2 = _1 << _8;
_17.0 = 15390631477356139843_usize << _4;
RET = _17.1 as f64;
_15 = [173_u8,104_u8,152_u8,103_u8,251_u8,10_u8,189_u8,208_u8];
(*_6) = !_4;
(*_9) = (-36_i8) as i32;
(*_9) = (*_6) | _1;
Goto(bb12)
}
bb12 = {
_23.0 = _17.0;
(*_9) = -_4;
(*_6) = (*_9);
_17.1 = -8362700005671683645_i64;
_6 = _9;
_5 = (*_6);
_6 = _9;
_15 = [154_u8,146_u8,87_u8,206_u8,207_u8,92_u8,7_u8,222_u8];
(*_6) = !_7;
_24 = 88199907190770757494692416864081811710_u128 as f32;
_5 = _4;
_25 = -_3;
_13 = _11 as i16;
_12 = _3;
(*_6) = _1 << _25;
RET = _23.0 as f64;
_7 = -_4;
RET = _3 as f64;
Call(_20 = fn18(_1, _11, _15, _8, (*_9), _7), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_13 = (-21745_i16);
_8 = _11 as u64;
_16 = 19729_u16;
_28 = _24 - _24;
(*_6) = _5;
_21.0 = [_8,_8,_8,_8,_8,_8,_8,_8];
_25 = _12 ^ _3;
_22 = [_17.1,_17.1];
_26.1 = !151_u8;
_12 = -_25;
_26.0 = [_17.0,_23.0,_23.0,_23.0];
_23 = _17;
(*_9) = _1;
_26.2 = _25 << _17.1;
_26.1 = 103_u8;
_8 = !807315695372732894_u64;
_26.1 = 201_u8 | 165_u8;
(*_6) = _7;
_23.0 = _17.0 - _17.0;
_7 = _1;
_24 = -_28;
_7 = (*_6) ^ (*_9);
_24 = -_28;
_12 = -_26.2;
Call(_29 = core::intrinsics::transmute(_17.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_6 = _9;
RET = _29 + _29;
_20 = _10;
_17.0 = !_23.0;
_1 = -_5;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 10_usize, Move(_10), 4_usize, Move(_4), 1_usize, Move(_1), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 17_usize, Move(_17), 8_usize, Move(_8), 22_usize, Move(_22), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(1_usize, 2_usize, Move(_2), 21_usize, Move(_21), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i32,mut _2: *mut i32,mut _3: *mut i32,mut _4: char,mut _5: i32,mut _6: *mut i32,mut _7: i32,mut _8: i32,mut _9: *mut i32,mut _10: i32,mut _11: i32,mut _12: i32,mut _13: i32) -> isize {
mir! {
type RET = isize;
let _14: f64;
let _15: u8;
let _16: i64;
let _17: *const bool;
let _18: f32;
let _19: ([usize; 4], u8, isize);
let _20: f64;
let _21: ([usize; 4], u8, isize);
let _22: u8;
let _23: [usize; 4];
let _24: [bool; 6];
let _25: char;
let _26: Adt46;
let _27: Adt52;
let _28: bool;
let _29: f32;
let _30: [usize; 4];
let _31: isize;
let _32: isize;
let _33: Adt50;
let _34: [u64; 8];
let _35: ();
let _36: ();
{
_1 = (*_6);
(*_6) = -_11;
_16 = 4382738894327262944_i64 >> _1;
(*_3) = 313782513_u32 as i32;
_6 = _3;
(*_3) = -_5;
RET = _12 as isize;
(*_6) = _5;
(*_6) = -_1;
_15 = 108_u8;
(*_6) = 2577_i16 as i32;
_9 = core::ptr::addr_of_mut!((*_6));
_7 = (*_2) * (*_9);
(*_2) = _8;
_10 = -(*_2);
_4 = '\u{dba29}';
_3 = _6;
_1 = true as i32;
_12 = !(*_2);
(*_3) = _5;
_13 = _12;
(*_2) = RET as i32;
_15 = 16_u8 & 198_u8;
(*_9) = 150223436196082405277748353534982548510_u128 as i32;
_11 = !_13;
_16 = 5119203218890358124_i64 & 4806483719391699020_i64;
_6 = _2;
_5 = 41_i8 as i32;
Goto(bb1)
}
bb1 = {
(*_3) = !_10;
(*_9) = _12 | _8;
_6 = core::ptr::addr_of_mut!((*_2));
(*_6) = _13;
_2 = core::ptr::addr_of_mut!(_11);
_11 = (*_6);
(*_9) = 22718978846911808590213545095486105169_u128 as i32;
RET = 9223372036854775807_isize;
_9 = _6;
_11 = (*_9);
(*_2) = _8 & _13;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
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
_15 = 48_u8;
_14 = RET as f64;
RET = 9223372036854775807_isize;
_12 = (*_2);
_15 = _16 as u8;
_13 = _11;
(*_2) = _4 as i32;
_16 = 88_i8 as i64;
(*_6) = _8;
_5 = (*_6);
(*_2) = _12 ^ (*_3);
_7 = (*_9) >> (*_9);
(*_3) = -(*_2);
RET = (-31_i8) as isize;
Call(_2 = fn3(_3, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_18 = _12 as f32;
_16 = 669732500_u32 as i64;
_19.1 = _15 << _12;
_13 = _8;
_21.1 = !_19.1;
_13 = _5 + _5;
_21.2 = RET + RET;
_12 = !(*_6);
_14 = 220861300206483209113482017738434534151_u128 as f64;
_11 = _1;
_6 = _2;
(*_9) = _7 ^ _13;
(*_3) = -_10;
_19.2 = _21.2 >> _19.1;
Goto(bb12)
}
bb12 = {
_16 = (-6824123589176645889_i64);
_9 = core::ptr::addr_of_mut!(_5);
_22 = true as u8;
_7 = 4_usize as i32;
_2 = core::ptr::addr_of_mut!(_7);
_20 = 30_i8 as f64;
_4 = '\u{83515}';
_23 = [17732718163216994213_usize,2_usize,1_usize,16951282038201621163_usize];
_11 = -(*_2);
_10 = 169564621004478689962577298460916804469_u128 as i32;
_20 = _16 as f64;
_25 = _4;
_9 = core::ptr::addr_of_mut!(_1);
_21.1 = !_15;
_21.0 = [3614467730342695046_usize,6_usize,3926300136788826768_usize,2_usize];
_25 = _4;
_23 = [4_usize,1_usize,6_usize,6_usize];
(*_9) = _20 as i32;
RET = _19.2 >> _5;
_13 = _16 as i32;
_19 = (_23, _21.1, RET);
_22 = _15;
_12 = _5 << _19.2;
Goto(bb13)
}
bb13 = {
_23 = _19.0;
_21.1 = _22;
_22 = (-29142770276501261005289754387087869500_i128) as u8;
_4 = _25;
_2 = core::ptr::addr_of_mut!(_12);
_21 = (_19.0, _15, _19.2);
_22 = !_21.1;
_19.1 = !_15;
(*_3) = _12 & (*_2);
_21 = _19;
(*_3) = !_5;
_13 = _12;
_16 = 2526063433889842244_i64 << (*_2);
_22 = !_19.1;
_13 = _10 & (*_2);
RET = 93620723369015413950436526521406719247_u128 as isize;
_33.fld0.0 = [17102472782998644884_u64,11067635089938025656_u64,12257781875509950510_u64,17578826361479660806_u64,15279641534954111781_u64,3527686897362357615_u64,15318266681699731347_u64,3400009020790474411_u64];
_6 = core::ptr::addr_of_mut!(_5);
(*_6) = (*_2);
_21 = (_19.0, _22, _19.2);
_5 = 63585_u16 as i32;
_33.fld0.0 = [10968515319874338072_u64,8435090460932868810_u64,15544620537571277352_u64,17227505459929169421_u64,5196956353619846811_u64,5456743445118252032_u64,10581644387844544912_u64,12421440924280815526_u64];
_34 = [3831961062763791551_u64,14385435157346356638_u64,18208319459619328480_u64,18405762027556814578_u64,17559417661642928198_u64,13190929979220923745_u64,17259776372266765539_u64,18276974898669191262_u64];
_33.fld0.2.1 = _15 ^ _15;
_12 = _1 + (*_3);
Goto(bb14)
}
bb14 = {
_29 = 3672869372_u32 as f32;
_2 = _3;
_14 = _20;
_33.fld0.1 = core::ptr::addr_of!(_28);
(*_2) = !_13;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(2_usize, 12_usize, Move(_12), 8_usize, Move(_8), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(2_usize, 25_usize, Move(_25), 16_usize, Move(_16), 34_usize, Move(_34), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: *mut i32,mut _2: *mut i32) -> *mut i32 {
mir! {
type RET = *mut i32;
let _3: i32;
let _4: i8;
let _5: [u8; 8];
let _6: isize;
let _7: &'static [i64; 4];
let _8: ([usize; 4], u8, isize);
let _9: Adt53;
let _10: isize;
let _11: [i64; 4];
let _12: char;
let _13: bool;
let _14: ();
let _15: ();
{
(*_2) = 110680043_i32 & (-1936821337_i32);
(*_2) = 2_usize as i32;
(*_2) = 29651_i16 as i32;
RET = core::ptr::addr_of_mut!((*_1));
_3 = (*_1) * (*RET);
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = (*RET);
_1 = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_2));
RET = _2;
_2 = _1;
(*RET) = !(-2009415041_i32);
_4 = (-111_i8);
_2 = core::ptr::addr_of_mut!((*_2));
_5 = [187_u8,173_u8,202_u8,104_u8,76_u8,183_u8,70_u8,77_u8];
_4 = (-93_i8) >> (*_2);
(*RET) = (-40157096014555366645344179669029553984_i128) as i32;
(*RET) = 0_usize as i32;
(*RET) = 689505950_i32;
_2 = core::ptr::addr_of_mut!((*_1));
RET = core::ptr::addr_of_mut!((*_1));
_6 = _4 as isize;
(*RET) = 1915212922_i32;
_3 = !338979909_i32;
Call(RET = fn4(_6, _5, _5, _5, _5, _6, _3, _5, _6, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 25_isize;
_8.2 = _6;
_10 = _6 << (*_2);
(*_1) = _4 as i32;
_2 = core::ptr::addr_of_mut!((*_1));
_8.0 = [4546379174759959350_usize,3_usize,1497352935699753137_usize,4_usize];
_8.2 = 0_usize as isize;
_6 = -_8.2;
(*_2) = 1526177425_i32;
match (*_1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1526177425 => bb9,
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
_1 = _2;
_2 = core::ptr::addr_of_mut!(_3);
_9 = Adt53 { fld0: _4 };
_9.fld0 = -_4;
_10 = _6;
match (*_1) {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb4,
1526177425 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_1 = RET;
(*_2) = (-2038699555_i32) >> _9.fld0;
_3 = 2078668382_i32;
_8.1 = 106_u8;
_8.1 = 52_u8 >> _10;
_9 = Adt53 { fld0: _4 };
_1 = RET;
_2 = core::ptr::addr_of_mut!((*_2));
_4 = _9.fld0 << _3;
Goto(bb12)
}
bb12 = {
_6 = !_8.2;
_8.1 = !214_u8;
_7 = &_11;
_6 = _10 + _8.2;
Call(_10 = core::intrinsics::transmute(_5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = core::ptr::addr_of_mut!((*_2));
_4 = 58315754401795421560814595360650185001_i128 as i8;
(*_2) = (-1952304065_i32);
_11 = [(-8921504636328230469_i64),(-915783480749350456_i64),(-4735518177349423744_i64),(-6583711179334514974_i64)];
_2 = _1;
match (*RET) {
0 => bb11,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607429815907391 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
_1 = RET;
(*_2) = (-2038699555_i32) >> _9.fld0;
_3 = 2078668382_i32;
_8.1 = 106_u8;
_8.1 = 52_u8 >> _10;
_9 = Adt53 { fld0: _4 };
_1 = RET;
_2 = core::ptr::addr_of_mut!((*_2));
_4 = _9.fld0 << _3;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_10 = _6 - _6;
RET = _1;
_1 = core::ptr::addr_of_mut!(_3);
_5 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_2 = core::ptr::addr_of_mut!((*_1));
_9.fld0 = !_4;
_12 = '\u{dc5fc}';
_3 = 1828339805_i32;
_7 = &_11;
_9.fld0 = _4 << _8.2;
_8.1 = !113_u8;
_10 = _6 >> _9.fld0;
_9.fld0 = _4 & _4;
_12 = '\u{54c0e}';
_8.0 = [4_usize,17105802878457345584_usize,3_usize,3_usize];
_13 = !true;
_9.fld0 = !_4;
(*_2) = -(-1141971942_i32);
_3 = (-1302735980_i32) & (-466531075_i32);
RET = _2;
(*RET) = 748201647_i32 ^ (-1935995289_i32);
RET = _1;
_13 = !false;
(*_2) = (-680008647_i32);
_10 = -_8.2;
(*RET) = 16213_u16 as i32;
RET = _1;
RET = _2;
Goto(bb19)
}
bb19 = {
Call(_14 = dump_var(3_usize, 4_usize, Move(_4), 5_usize, Move(_5), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: [u8; 8],mut _3: [u8; 8],mut _4: [u8; 8],mut _5: [u8; 8],mut _6: isize,mut _7: i32,mut _8: [u8; 8],mut _9: isize,mut _10: [u8; 8]) -> *mut i32 {
mir! {
type RET = *mut i32;
let _11: isize;
let _12: u8;
let _13: f64;
let _14: [u16; 3];
let _15: [i64; 4];
let _16: f64;
let _17: u8;
let _18: i16;
let _19: [u16; 1];
let _20: [u16; 6];
let _21: Adt51;
let _22: *mut i32;
let _23: Adt55;
let _24: *mut [i64; 4];
let _25: isize;
let _26: [u16; 1];
let _27: isize;
let _28: Adt56;
let _29: Adt44;
let _30: &'static [i64; 4];
let _31: Adt43;
let _32: [u16; 6];
let _33: isize;
let _34: (u64, [bool; 6]);
let _35: Adt56;
let _36: ();
let _37: ();
{
RET = core::ptr::addr_of_mut!(_7);
_5 = [195_u8,143_u8,226_u8,145_u8,228_u8,177_u8,107_u8,126_u8];
_4 = _3;
(*RET) = false as i32;
(*RET) = 1453371434_i32 ^ 179266941_i32;
RET = core::ptr::addr_of_mut!((*RET));
_2 = [163_u8,97_u8,44_u8,152_u8,215_u8,92_u8,135_u8,36_u8];
_3 = [220_u8,8_u8,234_u8,52_u8,20_u8,104_u8,131_u8,249_u8];
RET = core::ptr::addr_of_mut!(_7);
_4 = [130_u8,219_u8,225_u8,11_u8,52_u8,97_u8,241_u8,44_u8];
_10 = [254_u8,165_u8,80_u8,37_u8,114_u8,189_u8,44_u8,129_u8];
_3 = [62_u8,93_u8,197_u8,168_u8,43_u8,33_u8,189_u8,50_u8];
(*RET) = (-1640566420_i32);
_1 = -_9;
RET = core::ptr::addr_of_mut!(_7);
_5 = [202_u8,90_u8,106_u8,115_u8,50_u8,235_u8,136_u8,157_u8];
(*RET) = (-6747_i16) as i32;
_1 = _6 ^ _6;
Goto(bb1)
}
bb1 = {
_2 = [91_u8,82_u8,217_u8,85_u8,174_u8,147_u8,57_u8,20_u8];
(*RET) = 5505769121668698131_i64 as i32;
_11 = 10429198091625476477_usize as isize;
_1 = _6;
(*RET) = !2050854613_i32;
_9 = _6;
_10 = [76_u8,208_u8,239_u8,30_u8,80_u8,66_u8,197_u8,231_u8];
(*RET) = -(-1271745646_i32);
(*RET) = 7912769343104908124_u64 as i32;
_1 = _11;
(*RET) = _9 as i32;
_12 = 116_u8 | 95_u8;
Goto(bb2)
}
bb2 = {
_10 = [_12,_12,_12,_12,_12,_12,_12,_12];
_13 = (*RET) as f64;
_8 = [_12,_12,_12,_12,_12,_12,_12,_12];
_13 = 2_usize as f64;
Goto(bb3)
}
bb3 = {
(*RET) = 539878712_i32 - (-2064145731_i32);
_14 = [33411_u16,16346_u16,32502_u16];
(*RET) = 1371455498_i32 + 2119791457_i32;
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
(*RET) = (-1769113042_i32) | 1605957370_i32;
RET = core::ptr::addr_of_mut!((*RET));
_15 = [(-7708883034053511834_i64),3120508283075772459_i64,8339063117823045825_i64,4989061120659276757_i64];
_14 = [855_u16,18246_u16,33485_u16];
_8 = [_12,_12,_12,_12,_12,_12,_12,_12];
_7 = 665847075_i32 & 1290621069_i32;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
(*RET) = (-877116131_i32);
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = core::ptr::addr_of_mut!(_7);
_14 = [46482_u16,22853_u16,32080_u16];
RET = core::ptr::addr_of_mut!((*RET));
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
_6 = _9 + _1;
_13 = 206561072471784833531064383775302075545_u128 as f64;
(*RET) = 620221156_i32;
_19 = [10240_u16];
_9 = -_1;
_3 = _8;
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = core::ptr::addr_of_mut!(_7);
_18 = (-19689_i16);
Goto(bb4)
}
bb4 = {
_17 = _12;
(*RET) = (-1271764857_i32) + (-2080550597_i32);
_9 = _18 as isize;
_19 = [11555_u16];
_13 = _12 as f64;
_9 = -_6;
_15 = [7720502235677258014_i64,5045722814390569050_i64,1904545482082438896_i64,1101645896471443677_i64];
_7 = (-363152488_i32);
_2 = _4;
(*RET) = (-735476456_i32);
Call((*RET) = fn5(_10, _4, _9, RET, _17, _17, _4, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = _6 >> _9;
_2 = [_17,_12,_12,_12,_17,_17,_12,_12];
_1 = !_11;
_9 = _11;
_15 = [5136298049387710239_i64,(-4725837717367269090_i64),3049500382164825459_i64,(-7529720129892527963_i64)];
_18 = !25105_i16;
_4 = _8;
_9 = 135219989079883005427669886547099477192_i128 as isize;
_19 = [32205_u16];
_12 = !_17;
_20 = [39598_u16,247_u16,46118_u16,57632_u16,50447_u16,49655_u16];
_13 = 2480984152979250341_u64 as f64;
_7 = 1978549298_u32 as i32;
_16 = -_13;
(*RET) = (-508065717_i32) << _11;
_11 = _1;
RET = core::ptr::addr_of_mut!((*RET));
_3 = [_12,_17,_12,_12,_17,_12,_17,_17];
_2 = [_12,_12,_12,_17,_17,_17,_17,_17];
_10 = _3;
_15 = [5130605098636776654_i64,(-936800687629061333_i64),6503040724281431949_i64,(-3323286348716511693_i64)];
_18 = !10367_i16;
_20 = [61321_u16,61456_u16,64378_u16,27165_u16,41248_u16,60543_u16];
(*RET) = _11 as i32;
Call((*RET) = core::intrinsics::bswap((-1692785888_i32)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = core::ptr::addr_of_mut!(_7);
_20 = [32066_u16,3673_u16,2474_u16,11379_u16,32652_u16,40804_u16];
(*RET) = 839071269_i32;
_1 = _11 & _11;
_8 = _10;
_7 = !(-1220932658_i32);
_15 = [360725184061796585_i64,650670022943652539_i64,(-5428217865496450294_i64),(-197918140274342585_i64)];
_6 = _11 << _1;
_17 = _12;
_24 = core::ptr::addr_of_mut!(_15);
_3 = [_17,_12,_12,_17,_17,_12,_17,_17];
(*_24) = [(-4139345585740595871_i64),(-3102662515738938171_i64),1846916649619282170_i64,(-4492219607622019048_i64)];
_6 = 15608820574734529646_u64 as isize;
_4 = [_12,_17,_12,_12,_17,_17,_12,_12];
_4 = [_17,_12,_17,_12,_17,_17,_12,_17];
_20 = [18826_u16,11503_u16,46583_u16,15095_u16,46358_u16,46271_u16];
_16 = 599404122_u32 as f64;
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = core::ptr::addr_of_mut!((*RET));
_9 = -_1;
_12 = _17 | _17;
_22 = RET;
RET = core::ptr::addr_of_mut!((*RET));
_27 = _1 & _9;
_16 = _13;
(*_22) = '\u{a49ff}' as i32;
_6 = _1;
(*RET) = '\u{61486}' as i32;
_12 = !_17;
_29.fld0 = [false,true,false,false,true,false,false,false];
(*RET) = !2077222372_i32;
_29.fld2 = _9 | _27;
_25 = !_6;
Goto(bb8)
}
bb8 = {
_13 = -_16;
Goto(bb9)
}
bb9 = {
_2 = [_12,_12,_17,_17,_17,_12,_17,_17];
_29.fld1 = [45098_u16];
_29.fld0 = [false,false,false,true,true,true,false,false];
_16 = _13 * _13;
_19 = [11296_u16];
_1 = _29.fld2 | _29.fld2;
_7 = false as i32;
_26 = [58656_u16];
_30 = &_15;
_9 = _16 as isize;
_24 = core::ptr::addr_of_mut!((*_24));
_1 = _29.fld2 & _6;
RET = _22;
RET = core::ptr::addr_of_mut!((*_22));
_29.fld3 = !2031606338_u32;
_20 = [19804_u16,39621_u16,47686_u16,9512_u16,25152_u16,40023_u16];
_32 = [44735_u16,55397_u16,5970_u16,47448_u16,37778_u16,55667_u16];
Goto(bb10)
}
bb10 = {
_16 = (-40_i8) as f64;
_14 = [36966_u16,59966_u16,35559_u16];
_29.fld4 = [(-1390733367776625176_i64),14008697365576056_i64];
_29.fld2 = _9 * _1;
_29.fld1 = _19;
RET = _22;
_29.fld1 = _19;
_15 = [7696453219006226612_i64,(-421965746385100300_i64),4911028044727515327_i64,4096374671875797291_i64];
_18 = -28459_i16;
_29.fld0 = [false,false,true,true,true,false,true,false];
_2 = [_12,_17,_17,_12,_17,_17,_17,_12];
_34.0 = 14550318673401051431_u64 << _27;
_8 = [_12,_12,_12,_12,_12,_17,_17,_12];
Goto(bb11)
}
bb11 = {
_26 = [15385_u16];
(*RET) = -2112377933_i32;
_34.1 = [true,false,true,false,true,true];
_30 = &_15;
_11 = _29.fld3 as isize;
_17 = 52671_u16 as u8;
_22 = RET;
_11 = _1;
(*RET) = 1141235975_i32;
Goto(bb12)
}
bb12 = {
_34.1 = [false,false,false,false,true,false];
_33 = -_29.fld2;
_25 = -_1;
_17 = _12;
match (*_22) {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
1141235975 => bb21,
_ => bb20
}
}
bb13 = {
_26 = [15385_u16];
(*RET) = -2112377933_i32;
_34.1 = [true,false,true,false,true,true];
_30 = &_15;
_11 = _29.fld3 as isize;
_17 = 52671_u16 as u8;
_22 = RET;
_11 = _1;
(*RET) = 1141235975_i32;
Goto(bb12)
}
bb14 = {
_16 = (-40_i8) as f64;
_14 = [36966_u16,59966_u16,35559_u16];
_29.fld4 = [(-1390733367776625176_i64),14008697365576056_i64];
_29.fld2 = _9 * _1;
_29.fld1 = _19;
RET = _22;
_29.fld1 = _19;
_15 = [7696453219006226612_i64,(-421965746385100300_i64),4911028044727515327_i64,4096374671875797291_i64];
_18 = -28459_i16;
_29.fld0 = [false,false,true,true,true,false,true,false];
_2 = [_12,_17,_17,_12,_17,_17,_17,_12];
_34.0 = 14550318673401051431_u64 << _27;
_8 = [_12,_12,_12,_12,_12,_17,_17,_12];
Goto(bb11)
}
bb15 = {
(*RET) = 539878712_i32 - (-2064145731_i32);
_14 = [33411_u16,16346_u16,32502_u16];
(*RET) = 1371455498_i32 + 2119791457_i32;
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
(*RET) = (-1769113042_i32) | 1605957370_i32;
RET = core::ptr::addr_of_mut!((*RET));
_15 = [(-7708883034053511834_i64),3120508283075772459_i64,8339063117823045825_i64,4989061120659276757_i64];
_14 = [855_u16,18246_u16,33485_u16];
_8 = [_12,_12,_12,_12,_12,_12,_12,_12];
_7 = 665847075_i32 & 1290621069_i32;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
(*RET) = (-877116131_i32);
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = core::ptr::addr_of_mut!(_7);
_14 = [46482_u16,22853_u16,32080_u16];
RET = core::ptr::addr_of_mut!((*RET));
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
_6 = _9 + _1;
_13 = 206561072471784833531064383775302075545_u128 as f64;
(*RET) = 620221156_i32;
_19 = [10240_u16];
_9 = -_1;
_3 = _8;
_3 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = core::ptr::addr_of_mut!(_7);
_18 = (-19689_i16);
Goto(bb4)
}
bb16 = {
_13 = -_16;
Goto(bb9)
}
bb17 = {
RET = core::ptr::addr_of_mut!((*RET));
_9 = -_1;
_12 = _17 | _17;
_22 = RET;
RET = core::ptr::addr_of_mut!((*RET));
_27 = _1 & _9;
_16 = _13;
(*_22) = '\u{a49ff}' as i32;
_6 = _1;
(*RET) = '\u{61486}' as i32;
_12 = !_17;
_29.fld0 = [false,true,false,false,true,false,false,false];
(*RET) = !2077222372_i32;
_29.fld2 = _9 | _27;
_25 = !_6;
Goto(bb8)
}
bb18 = {
RET = core::ptr::addr_of_mut!(_7);
_20 = [32066_u16,3673_u16,2474_u16,11379_u16,32652_u16,40804_u16];
(*RET) = 839071269_i32;
_1 = _11 & _11;
_8 = _10;
_7 = !(-1220932658_i32);
_15 = [360725184061796585_i64,650670022943652539_i64,(-5428217865496450294_i64),(-197918140274342585_i64)];
_6 = _11 << _1;
_17 = _12;
_24 = core::ptr::addr_of_mut!(_15);
_3 = [_17,_12,_12,_17,_17,_12,_17,_17];
(*_24) = [(-4139345585740595871_i64),(-3102662515738938171_i64),1846916649619282170_i64,(-4492219607622019048_i64)];
_6 = 15608820574734529646_u64 as isize;
_4 = [_12,_17,_12,_12,_17,_17,_12,_12];
_4 = [_17,_12,_17,_12,_17,_17,_12,_17];
_20 = [18826_u16,11503_u16,46583_u16,15095_u16,46358_u16,46271_u16];
_16 = 599404122_u32 as f64;
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb7), UnwindUnreachable())
}
bb19 = {
_11 = _6 >> _9;
_2 = [_17,_12,_12,_12,_17,_17,_12,_12];
_1 = !_11;
_9 = _11;
_15 = [5136298049387710239_i64,(-4725837717367269090_i64),3049500382164825459_i64,(-7529720129892527963_i64)];
_18 = !25105_i16;
_4 = _8;
_9 = 135219989079883005427669886547099477192_i128 as isize;
_19 = [32205_u16];
_12 = !_17;
_20 = [39598_u16,247_u16,46118_u16,57632_u16,50447_u16,49655_u16];
_13 = 2480984152979250341_u64 as f64;
_7 = 1978549298_u32 as i32;
_16 = -_13;
(*RET) = (-508065717_i32) << _11;
_11 = _1;
RET = core::ptr::addr_of_mut!((*RET));
_3 = [_12,_17,_12,_12,_17,_12,_17,_17];
_2 = [_12,_12,_12,_17,_17,_17,_17,_17];
_10 = _3;
_15 = [5130605098636776654_i64,(-936800687629061333_i64),6503040724281431949_i64,(-3323286348716511693_i64)];
_18 = !10367_i16;
_20 = [61321_u16,61456_u16,64378_u16,27165_u16,41248_u16,60543_u16];
(*RET) = _11 as i32;
Call((*RET) = core::intrinsics::bswap((-1692785888_i32)), ReturnTo(bb6), UnwindUnreachable())
}
bb20 = {
_17 = _12;
(*RET) = (-1271764857_i32) + (-2080550597_i32);
_9 = _18 as isize;
_19 = [11555_u16];
_13 = _12 as f64;
_9 = -_6;
_15 = [7720502235677258014_i64,5045722814390569050_i64,1904545482082438896_i64,1101645896471443677_i64];
_7 = (-363152488_i32);
_2 = _4;
(*RET) = (-735476456_i32);
Call((*RET) = fn5(_10, _4, _9, RET, _17, _17, _4, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb21 = {
_18 = '\u{100ec9}' as i16;
_29.fld2 = _29.fld3 as isize;
Goto(bb22)
}
bb22 = {
Call(_36 = dump_var(4_usize, 26_usize, Move(_26), 19_usize, Move(_19), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_36 = dump_var(4_usize, 11_usize, Move(_11), 27_usize, Move(_27), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_36 = dump_var(4_usize, 14_usize, Move(_14), 18_usize, Move(_18), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u8; 8],mut _2: [u8; 8],mut _3: isize,mut _4: *mut i32,mut _5: u8,mut _6: u8,mut _7: [u8; 8],mut _8: [u8; 8]) -> i32 {
mir! {
type RET = i32;
let _9: i128;
let _10: ([i16; 6], i32, u16, i64, i16);
let _11: f32;
let _12: isize;
let _13: f64;
let _14: f32;
let _15: i128;
let _16: [u16; 1];
let _17: Adt50;
let _18: [i64; 4];
let _19: Adt42;
let _20: f32;
let _21: isize;
let _22: i64;
let _23: Adt58;
let _24: [usize; 3];
let _25: u8;
let _26: [bool; 6];
let _27: f64;
let _28: *mut [i64; 4];
let _29: ();
let _30: ();
{
RET = 1380402657_i32;
RET = 3355974037648195342_i64 as i32;
_7 = [_5,_5,_5,_5,_6,_6,_6,_6];
RET = 76469824_i32 * 1335953698_i32;
_2 = [_6,_6,_5,_6,_6,_5,_6,_6];
_7 = _8;
_8 = [_6,_6,_5,_6,_5,_6,_5,_6];
_4 = core::ptr::addr_of_mut!(RET);
Goto(bb1)
}
bb1 = {
RET = (-1640056635_i32);
(*_4) = 995510826_i32 | 395831159_i32;
_10.0 = [12756_i16,(-8627_i16),31773_i16,(-26120_i16),(-4949_i16),(-1941_i16)];
_11 = (-6604222681901160624_i64) as f32;
_9 = !23408176841831346929893432266034677242_i128;
_10.1 = -RET;
(*_4) = _10.1;
_10.1 = (*_4);
_8 = [_5,_6,_6,_5,_5,_5,_6,_6];
_10.1 = (*_4) * (*_4);
_8 = [_5,_6,_6,_5,_6,_6,_6,_6];
_12 = 4_usize as isize;
_10.4 = (-18526_i16) << _10.1;
_10.4 = (-74_i8) as i16;
_1 = _2;
_6 = 464620340599475020_u64 as u8;
_10.3 = (-6927292269676336558_i64) + (-8409502703796087357_i64);
_2 = [_5,_5,_5,_5,_6,_5,_5,_5];
_14 = -_11;
_13 = _5 as f64;
_2 = [_5,_5,_5,_5,_5,_5,_6,_5];
_10.0 = [_10.4,_10.4,_10.4,_10.4,_10.4,_10.4];
Call(_5 = fn6(_4, _10.1, _13, _10.1, _12, _7, _4, _10.1, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = [_5,_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_10.1;
Goto(bb3)
}
bb3 = {
_13 = 3045058150_u32 as f64;
_7 = _8;
_9 = (-58542795915451373197354656939341604486_i128) | 52546080157994461849415522419719921462_i128;
_10.2 = _12 as u16;
RET = _10.1;
_10.4 = 6827_i16 | (-8292_i16);
Goto(bb4)
}
bb4 = {
(*_4) = _10.1;
_17.fld0.0 = [7819601206496111402_u64,16349588211238725936_u64,9755175865269450467_u64,4644558058950616497_u64,9583064847752267774_u64,7217117651411642676_u64,16395288582946691100_u64,1898214499312994876_u64];
_10.1 = _10.4 as i32;
_17.fld0.2.2 = _12;
_17.fld0.0 = [955304388274342441_u64,14685855067342690443_u64,15958270029300923989_u64,5891114925616863765_u64,1400889517771718965_u64,11717600554696997782_u64,7408319925005312656_u64,14873721766741328928_u64];
Goto(bb5)
}
bb5 = {
_17.fld0.2.0 = [7_usize,12664488076941530381_usize,2748670860413913194_usize,2212521181495315483_usize];
_10.2 = 54615_u16;
_11 = _5 as f32;
_7 = _1;
_12 = -_17.fld0.2.2;
_5 = _6;
(*_4) = _10.3 as i32;
(*_4) = !_10.1;
_16 = [_10.2];
_14 = -_11;
_14 = (*_4) as f32;
_17.fld0.2.1 = !_5;
_5 = _6 ^ _6;
_17.fld0.2.0 = [0_usize,1_usize,5_usize,7_usize];
_10.2 = 53901_u16;
_1 = [_5,_5,_17.fld0.2.1,_17.fld0.2.1,_5,_5,_6,_5];
_17.fld0.2.1 = _5;
_10.2 = !51030_u16;
_13 = (-77_i8) as f64;
_4 = core::ptr::addr_of_mut!((*_4));
_2 = [_17.fld0.2.1,_5,_5,_17.fld0.2.1,_17.fld0.2.1,_5,_6,_17.fld0.2.1];
_17.fld0.2.0 = [0_usize,5_usize,5_usize,872995626280817635_usize];
(*_4) = _10.1 << _10.4;
Call(_17.fld0.2 = fn9(_2, _9, _14, _16, _3, _10.4, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.2 = _10.4 as u16;
_2 = _1;
_15 = _9 << _17.fld0.2.2;
_13 = _11 as f64;
_10.2 = 17299_u16 & 42944_u16;
_10.0 = [_10.4,_10.4,_10.4,_10.4,_10.4,_10.4];
_5 = !_17.fld0.2.1;
_7 = _8;
_18 = [_10.3,_10.3,_10.3,_10.3];
_11 = _14 - _14;
_10.2 = 157_u16;
_10.3 = 5192204458753907812_i64;
_17.fld0.2.0 = [6_usize,137428175700464597_usize,3489007973537696172_usize,2705103942423637799_usize];
_10.2 = !9619_u16;
_5 = false as u8;
(*_4) = _10.1 * _10.1;
_8 = [_6,_6,_17.fld0.2.1,_5,_5,_5,_6,_17.fld0.2.1];
_20 = _11;
_7 = [_17.fld0.2.1,_5,_17.fld0.2.1,_6,_6,_17.fld0.2.1,_17.fld0.2.1,_17.fld0.2.1];
_7 = [_17.fld0.2.1,_17.fld0.2.1,_6,_17.fld0.2.1,_6,_17.fld0.2.1,_6,_17.fld0.2.1];
_21 = _17.fld0.2.2;
Call(_9 = fn15(_17.fld0.2.2, _17.fld0.2.2, _21, _17.fld0.2.2, (*_4), _17.fld0.2, _17.fld0.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_18 = [_10.3,_10.3,_10.3,_10.3];
_9 = _15;
_8 = _2;
_12 = _13 as isize;
_12 = 9228765921142711542_usize as isize;
_22 = _10.2 as i64;
Call(_25 = core::intrinsics::bswap(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.fld0.2.0 = [1_usize,3_usize,8303504142950961700_usize,9555642011765841003_usize];
_24 = [1_usize,6898242264350793871_usize,11404122331206211477_usize];
(*_4) = _10.1;
_17.fld0.2.0 = [16035300365213535080_usize,16139736616358980967_usize,1_usize,3_usize];
(*_4) = -_10.1;
_3 = _10.2 as isize;
_10.0 = [_10.4,_10.4,_10.4,_10.4,_10.4,_10.4];
_17.fld0.2.2 = _5 as isize;
_2 = [_5,_6,_6,_17.fld0.2.1,_17.fld0.2.1,_17.fld0.2.1,_17.fld0.2.1,_17.fld0.2.1];
_11 = 292430491604629212093505485520038287749_u128 as f32;
_14 = _21 as f32;
_28 = core::ptr::addr_of_mut!(_18);
_17.fld0.2.0 = [2_usize,4_usize,9841901818971682142_usize,4_usize];
match _17.fld0.2.1 {
0 => bb7,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
254 => bb15,
_ => bb14
}
}
bb9 = {
_18 = [_10.3,_10.3,_10.3,_10.3];
_9 = _15;
_8 = _2;
_12 = _13 as isize;
_12 = 9228765921142711542_usize as isize;
_22 = _10.2 as i64;
Call(_25 = core::intrinsics::bswap(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_10.2 = _10.4 as u16;
_2 = _1;
_15 = _9 << _17.fld0.2.2;
_13 = _11 as f64;
_10.2 = 17299_u16 & 42944_u16;
_10.0 = [_10.4,_10.4,_10.4,_10.4,_10.4,_10.4];
_5 = !_17.fld0.2.1;
_7 = _8;
_18 = [_10.3,_10.3,_10.3,_10.3];
_11 = _14 - _14;
_10.2 = 157_u16;
_10.3 = 5192204458753907812_i64;
_17.fld0.2.0 = [6_usize,137428175700464597_usize,3489007973537696172_usize,2705103942423637799_usize];
_10.2 = !9619_u16;
_5 = false as u8;
(*_4) = _10.1 * _10.1;
_8 = [_6,_6,_17.fld0.2.1,_5,_5,_5,_6,_17.fld0.2.1];
_20 = _11;
_7 = [_17.fld0.2.1,_5,_17.fld0.2.1,_6,_6,_17.fld0.2.1,_17.fld0.2.1,_17.fld0.2.1];
_7 = [_17.fld0.2.1,_17.fld0.2.1,_6,_17.fld0.2.1,_6,_17.fld0.2.1,_6,_17.fld0.2.1];
_21 = _17.fld0.2.2;
Call(_9 = fn15(_17.fld0.2.2, _17.fld0.2.2, _21, _17.fld0.2.2, (*_4), _17.fld0.2, _17.fld0.2), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_17.fld0.2.0 = [7_usize,12664488076941530381_usize,2748670860413913194_usize,2212521181495315483_usize];
_10.2 = 54615_u16;
_11 = _5 as f32;
_7 = _1;
_12 = -_17.fld0.2.2;
_5 = _6;
(*_4) = _10.3 as i32;
(*_4) = !_10.1;
_16 = [_10.2];
_14 = -_11;
_14 = (*_4) as f32;
_17.fld0.2.1 = !_5;
_5 = _6 ^ _6;
_17.fld0.2.0 = [0_usize,1_usize,5_usize,7_usize];
_10.2 = 53901_u16;
_1 = [_5,_5,_17.fld0.2.1,_17.fld0.2.1,_5,_5,_6,_5];
_17.fld0.2.1 = _5;
_10.2 = !51030_u16;
_13 = (-77_i8) as f64;
_4 = core::ptr::addr_of_mut!((*_4));
_2 = [_17.fld0.2.1,_5,_5,_17.fld0.2.1,_17.fld0.2.1,_5,_6,_17.fld0.2.1];
_17.fld0.2.0 = [0_usize,5_usize,5_usize,872995626280817635_usize];
(*_4) = _10.1 << _10.4;
Call(_17.fld0.2 = fn9(_2, _9, _14, _16, _3, _10.4, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
RET = (-1640056635_i32);
(*_4) = 995510826_i32 | 395831159_i32;
_10.0 = [12756_i16,(-8627_i16),31773_i16,(-26120_i16),(-4949_i16),(-1941_i16)];
_11 = (-6604222681901160624_i64) as f32;
_9 = !23408176841831346929893432266034677242_i128;
_10.1 = -RET;
(*_4) = _10.1;
_10.1 = (*_4);
_8 = [_5,_6,_6,_5,_5,_5,_6,_6];
_10.1 = (*_4) * (*_4);
_8 = [_5,_6,_6,_5,_6,_6,_6,_6];
_12 = 4_usize as isize;
_10.4 = (-18526_i16) << _10.1;
_10.4 = (-74_i8) as i16;
_1 = _2;
_6 = 464620340599475020_u64 as u8;
_10.3 = (-6927292269676336558_i64) + (-8409502703796087357_i64);
_2 = [_5,_5,_5,_5,_6,_5,_5,_5];
_14 = -_11;
_13 = _5 as f64;
_2 = [_5,_5,_5,_5,_5,_5,_6,_5];
_10.0 = [_10.4,_10.4,_10.4,_10.4,_10.4,_10.4];
Call(_5 = fn6(_4, _10.1, _13, _10.1, _12, _7, _4, _10.1, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_13 = 3045058150_u32 as f64;
_7 = _8;
_9 = (-58542795915451373197354656939341604486_i128) | 52546080157994461849415522419719921462_i128;
_10.2 = _12 as u16;
RET = _10.1;
_10.4 = 6827_i16 | (-8292_i16);
Goto(bb4)
}
bb14 = {
_7 = [_5,_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_10.1;
Goto(bb3)
}
bb15 = {
_24 = [2_usize,5734130008241585262_usize,10270685101192203941_usize];
_13 = _14 as f64;
RET = -_10.1;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(5_usize, 5_usize, Move(_5), 25_usize, Move(_25), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 21_usize, Move(_21), 12_usize, Move(_12), 24_usize, Move(_24), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut i32,mut _2: i32,mut _3: f64,mut _4: i32,mut _5: isize,mut _6: [u8; 8],mut _7: *mut i32,mut _8: i32,mut _9: f64) -> u8 {
mir! {
type RET = u8;
let _10: [i128; 5];
let _11: ([i16; 6], i32, u16, i64, i16);
let _12: char;
let _13: u64;
let _14: *mut i32;
let _15: [u16; 3];
let _16: [bool; 4];
let _17: isize;
let _18: u8;
let _19: [bool; 6];
let _20: isize;
let _21: [i16; 6];
let _22: ([usize; 4], u8, isize);
let _23: isize;
let _24: [u16; 1];
let _25: ([u64; 8], *const bool, ([usize; 4], u8, isize));
let _26: [bool; 8];
let _27: [i64; 4];
let _28: isize;
let _29: Adt53;
let _30: ();
let _31: ();
{
_7 = _1;
_9 = -_3;
_8 = _4 ^ (*_1);
_10 = [123716150586050989324787598005046226883_i128,164030132382208541031476962177636474490_i128,90401091060755955710913035168912994373_i128,149151924201754315647946470094110073959_i128,(-143392419033940667392101901935098487946_i128)];
_6 = [157_u8,123_u8,131_u8,234_u8,111_u8,211_u8,38_u8,219_u8];
(*_7) = _4 ^ _2;
_11.1 = (*_1);
(*_7) = (-20_i8) as i32;
_5 = !(-20_isize);
_4 = _8;
_6 = [59_u8,241_u8,199_u8,96_u8,40_u8,216_u8,1_u8,141_u8];
(*_7) = _2 * _2;
(*_1) = _11.1;
(*_1) = -_4;
_11.2 = 44871_u16 + 52116_u16;
(*_1) = _4 + _11.1;
_11.4 = _5 as i16;
_5 = 9223372036854775807_isize;
_10 = [98246454616816585105495075995225605612_i128,87330282518567484487486165042798757858_i128,155214485921628162038540042508666625459_i128,(-29873099852200343001579945173236640131_i128),(-60661365302831589330846257877280762090_i128)];
_15 = [_11.2,_11.2,_11.2];
_7 = core::ptr::addr_of_mut!((*_7));
_2 = !(*_1);
RET = !113_u8;
_16 = [true,true,false,false];
Call(RET = core::intrinsics::bswap(115_u8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.3 = 179327987756082544406725980889796481478_u128 as i64;
_15 = [_11.2,_11.2,_11.2];
_9 = _3;
_5 = 101_isize;
_12 = '\u{7f4fa}';
_16 = [false,true,true,false];
(*_7) = _8;
(*_7) = _8;
(*_7) = _11.4 as i32;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_14 = core::ptr::addr_of_mut!((*_1));
Call(_11.3 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_1) = _8 << _8;
_17 = (-51_i8) as isize;
_18 = !RET;
_13 = !16503454372025570885_u64;
(*_1) = !_8;
_16 = [false,true,true,true];
(*_14) = _8;
(*_14) = _2;
_11.3 = 1804985770413413007_i64;
_16 = [false,true,true,false];
(*_7) = _4 << _8;
(*_7) = _2 << RET;
_19 = [true,false,false,true,true,true];
_11.4 = (-22365_i16) << _13;
_4 = (*_14);
RET = _18 | _18;
_15 = [_11.2,_11.2,_11.2];
(*_1) = !_11.1;
_11.0 = [_11.4,_11.4,_11.4,_11.4,_11.4,_11.4];
_13 = 9342479168560023432_u64;
_9 = _13 as f64;
_22.0 = [4941053376794256407_usize,10038211352914969122_usize,4_usize,0_usize];
_14 = _7;
(*_14) = -_4;
_11.0 = [_11.4,_11.4,_11.4,_11.4,_11.4,_11.4];
_23 = _17;
_23 = (-1_i8) as isize;
Goto(bb3)
}
bb3 = {
_14 = core::ptr::addr_of_mut!(_2);
Call(_22.2 = fn7(_10, _14, _13, (*_1), _2, (*_7), (*_1), (*_14), _22.0, (*_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = _7;
_1 = _14;
_16 = [false,false,true,false];
_25.2 = (_22.0, RET, _22.2);
_24 = [_11.2];
_13 = 265491372474996296032392424198986324511_u128 as u64;
_19 = [false,false,false,true,false,true];
_24 = [_11.2];
_12 = '\u{56abf}';
_25.0 = [_13,_13,_13,_13,_13,_13,_13,_13];
RET = (*_14) as u8;
_5 = -_25.2.2;
_15 = [_11.2,_11.2,_11.2];
(*_7) = 8097530619046359239956907683927751487_u128 as i32;
(*_14) = 15089476591488030833_usize as i32;
_7 = _1;
_25.2.1 = RET;
_25.2 = (_22.0, RET, _22.2);
_9 = _3;
_24 = [_11.2];
_21 = [_11.4,_11.4,_11.4,_11.4,_11.4,_11.4];
_14 = core::ptr::addr_of_mut!(_4);
Goto(bb5)
}
bb5 = {
Call(_30 = dump_var(6_usize, 17_usize, Move(_17), 18_usize, Move(_18), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_30 = dump_var(6_usize, 4_usize, Move(_4), 19_usize, Move(_19), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [i128; 5],mut _2: *mut i32,mut _3: u64,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: [usize; 4],mut _10: i32) -> isize {
mir! {
type RET = isize;
let _11: isize;
let _12: Adt58;
let _13: u128;
let _14: char;
let _15: usize;
let _16: [i128; 5];
let _17: Adt55;
let _18: &'static [i64; 4];
let _19: bool;
let _20: ();
let _21: ();
{
_2 = core::ptr::addr_of_mut!((*_2));
RET = (-63_isize) ^ (-46_isize);
_9 = [13294445994089147318_usize,5_usize,15916430498717963635_usize,3_usize];
_11 = RET;
_5 = -_4;
_9 = [2_usize,2_usize,5_usize,3_usize];
_4 = _10 & _5;
(*_2) = _5;
RET = -_11;
_6 = !_4;
_9 = [4_usize,5722555739289466617_usize,10967503258474169354_usize,1_usize];
_8 = -_7;
_10 = _4;
_10 = 139_u8 as i32;
_5 = false as i32;
Goto(bb1)
}
bb1 = {
_3 = _11 as u64;
_8 = (*_2) << (*_2);
_6 = _8;
_2 = core::ptr::addr_of_mut!(_8);
RET = (-7955_i16) as isize;
_10 = _7 >> (*_2);
_4 = 8373150780593317045896770851388117594_u128 as i32;
_9 = [3_usize,13472067920909269691_usize,3280917104149642803_usize,3_usize];
_4 = (-25892_i16) as i32;
_9 = [12966649178284537768_usize,3_usize,3_usize,2151459684745092976_usize];
Call(_11 = fn8(_10, RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = !10006153493419030169_u64;
_14 = '\u{db70a}';
_2 = core::ptr::addr_of_mut!(_7);
_3 = 9786462467916296996_u64 + 16692655446799657337_u64;
_4 = 41584_u16 as i32;
_15 = 9239142800403012463_usize + 5_usize;
_3 = !7331326343503591071_u64;
_10 = _6 - _7;
_14 = '\u{ef0cc}';
(*_2) = !_6;
_13 = !12738983736790614680876548941584458377_u128;
_3 = 18355085040525007219_u64 >> _10;
_13 = 46080448306092639457797867017721155676_u128 - 173953210482770545164985240981718360912_u128;
_8 = (-51829747147972384989340500242649885450_i128) as i32;
Goto(bb3)
}
bb3 = {
RET = (*_2) as isize;
_5 = (*_2);
_6 = false as i32;
_16 = [100622459269489716456367844951420796238_i128,88362694552543876551763779607595536082_i128,(-23560101699486262786281867858509400913_i128),(-161158810157888807326714951135755202660_i128),(-6884644584590045378300809733417442150_i128)];
_6 = _5 * _10;
_3 = !7329663382848819807_u64;
_4 = (*_2) & _6;
_9 = [_15,_15,_15,_15];
_8 = -_10;
(*_2) = -_4;
_8 = !(*_2);
_7 = _8 + _4;
_1 = [25235924000074407085063597065946523172_i128,(-126616841472449616199848569079300958770_i128),4472637759890093473197860641838125400_i128,169118603232520616212197619140862283218_i128,24088654570538827082200907653071490374_i128];
_3 = 9198333194672734240_u64;
_11 = !RET;
RET = _11 & _11;
_13 = _14 as u128;
Goto(bb4)
}
bb4 = {
Call(_20 = dump_var(7_usize, 11_usize, Move(_11), 1_usize, Move(_1), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_20 = dump_var(7_usize, 10_usize, Move(_10), 8_usize, Move(_8), 16_usize, Move(_16), 21_usize, _21), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i32,mut _2: isize,mut _3: [i128; 5]) -> isize {
mir! {
type RET = isize;
let _4: [usize; 4];
let _5: [usize; 4];
let _6: ([i16; 6], i32, u16, i64, i16);
let _7: Adt51;
let _8: isize;
let _9: f32;
let _10: i128;
let _11: *const bool;
let _12: Adt43;
let _13: f64;
let _14: u8;
let _15: bool;
let _16: [bool; 6];
let _17: ([i16; 6], i32, u16, i64, i16);
let _18: ([u64; 8], *const bool, ([usize; 4], u8, isize));
let _19: f64;
let _20: bool;
let _21: [bool; 8];
let _22: char;
let _23: ([usize; 4], u8, isize);
let _24: *mut i32;
let _25: [i128; 5];
let _26: *const bool;
let _27: u16;
let _28: char;
let _29: (usize, i64);
let _30: u32;
let _31: isize;
let _32: bool;
let _33: ([usize; 4], u8, isize);
let _34: isize;
let _35: ();
let _36: ();
{
_2 = (-75_isize) >> _1;
RET = _2;
RET = _2;
_1 = 127_i8 as i32;
_2 = 13602810325815643899_usize as isize;
_2 = 42991_u16 as isize;
RET = _2;
_4 = [2_usize,12010600581904055556_usize,2_usize,2_usize];
_5 = _4;
RET = 1092334164_u32 as isize;
_8 = _2 - _2;
_2 = RET | RET;
_6.3 = 70_i8 as i64;
_6.4 = 21607_i16;
_1 = 9696616262766496062_u64 as i32;
_2 = _6.3 as isize;
Goto(bb1)
}
bb1 = {
RET = _2 ^ _8;
_1 = 16095508426027040966722907690835499948_i128 as i32;
_6.1 = _1;
_9 = 103062551152453809638449889078656148945_i128 as f32;
_6.2 = 46020_u16;
_6.1 = _6.4 as i32;
_8 = (-170089213023740843625787955011684991422_i128) as isize;
_10 = _6.2 as i128;
_10 = 37616127980279179616191397030400319443_i128;
RET = _9 as isize;
_4 = [2_usize,9499312053377847065_usize,11501601258638837283_usize,15329561007825603149_usize];
_8 = RET << _2;
_6.1 = !_1;
_6.4 = -(-20132_i16);
_6.0 = [_6.4,_6.4,_6.4,_6.4,_6.4,_6.4];
_2 = _8;
_10 = 124705762043186388063130782484680328543_i128;
_13 = 119_u8 as f64;
_2 = _8 >> _10;
match _6.2 {
0 => bb2,
1 => bb3,
46020 => bb5,
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
_14 = !143_u8;
RET = _8;
_13 = RET as f64;
_8 = 59182821899631649120834293082416089958_u128 as isize;
_14 = 166_u8;
_15 = !true;
_15 = false;
_4 = [1_usize,7_usize,7061212186864702849_usize,8717817665471479194_usize];
_6.4 = _9 as i16;
_11 = core::ptr::addr_of!(_15);
_6.4 = -(-24935_i16);
_18.2.0 = [7_usize,5_usize,0_usize,0_usize];
match _6.2 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
46020 => bb8,
_ => bb7
}
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
RET = _2 ^ _2;
_17.4 = _6.4 * _6.4;
_18.0 = [4667861393713167257_u64,4930357380551661176_u64,8430280061898564187_u64,179588682265016338_u64,18205827177806527630_u64,6466936583063483959_u64,2623465469894590131_u64,4484329597631341276_u64];
_17.0 = [_6.4,_6.4,_17.4,_17.4,_17.4,_17.4];
_19 = _10 as f64;
_18.2.1 = '\u{4f9f1}' as u8;
(*_11) = _14 > _18.2.1;
_18.1 = core::ptr::addr_of!((*_11));
_18.0 = [2319218571659087336_u64,34072559854199247_u64,8623277523402419728_u64,347492633028654660_u64,6461894932140056751_u64,4065594992299964879_u64,5796458121914592258_u64,17109276690596930468_u64];
(*_11) = RET <= RET;
Goto(bb9)
}
bb9 = {
_10 = 3_usize as i128;
_18.0 = [1186279774690779375_u64,14101671176347394611_u64,12228622075920448236_u64,14343460323554291493_u64,5490655286101917671_u64,5173677569077455531_u64,17565468107837696778_u64,17849087589251550054_u64];
_19 = _13 - _13;
_18.2.2 = -RET;
_17 = _6;
_6.1 = -_1;
_18.2.2 = RET ^ RET;
_6 = _17;
_2 = _8;
_18.2.1 = _14 ^ _14;
Goto(bb10)
}
bb10 = {
_20 = (*_11) | (*_11);
_16 = [_20,(*_11),_20,_20,_20,_20];
_18.2.0 = _5;
_17.2 = _6.2;
_8 = _9 as isize;
_3 = [_10,_10,_10,_10,_10];
_20 = _18.2.2 >= RET;
_21 = [(*_11),_20,(*_11),(*_11),(*_11),_20,_15,(*_11)];
_9 = 36011017236275314328778662519486377334_u128 as f32;
(*_11) = _20;
Call(_17.4 = core::intrinsics::transmute(_17.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _17;
_18.2 = (_4, _14, RET);
_6.1 = 5230066700993321566477783434173775884_u128 as i32;
_13 = 6145789345159554268_usize as f64;
_17.2 = !_6.2;
_17 = (_6.0, _1, _6.2, _6.3, _6.4);
Goto(bb12)
}
bb12 = {
_5 = [13536306003663620348_usize,3_usize,7_usize,11472376322196721175_usize];
_16 = [(*_11),_15,_15,(*_11),(*_11),_20];
_6.3 = _17.3 + _17.3;
_17 = (_6.0, _1, _6.2, _6.3, _6.4);
_23 = _18.2;
_9 = 800184506733566739_u64 as f32;
_24 = core::ptr::addr_of_mut!(_1);
_18.2 = _23;
_4 = _18.2.0;
_13 = -_19;
Goto(bb13)
}
bb13 = {
_5 = [3_usize,7214534095056164380_usize,4_usize,26330236185182614_usize];
_22 = '\u{1f384}';
_29.1 = _6.3 + _17.3;
(*_24) = _17.1 | _17.1;
_28 = _22;
_18.2.1 = !_23.1;
(*_11) = _20 & _20;
RET = _23.2;
_18.2.1 = _14 >> _17.2;
_17.0 = _6.0;
_3 = [_10,_10,_10,_10,_10];
_17 = _6;
_30 = !2137927449_u32;
_17.3 = _29.1;
_18.2.0 = [6_usize,7_usize,1_usize,1_usize];
Goto(bb14)
}
bb14 = {
_24 = core::ptr::addr_of_mut!((*_24));
_18.2.1 = _14;
_23.1 = !_14;
_6.2 = _17.2;
_6 = (_17.0, _1, _17.2, _29.1, _17.4);
_5 = [3_usize,2_usize,17256845786052509343_usize,3_usize];
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(8_usize, 17_usize, Move(_17), 22_usize, Move(_22), 21_usize, Move(_21), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(8_usize, 6_usize, Move(_6), 30_usize, Move(_30), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(8_usize, 3_usize, Move(_3), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [u8; 8],mut _2: i128,mut _3: f32,mut _4: [u16; 1],mut _5: isize,mut _6: i16,mut _7: [u8; 8]) -> ([usize; 4], u8, isize) {
mir! {
type RET = ([usize; 4], u8, isize);
let _8: Adt53;
let _9: bool;
let _10: Adt53;
let _11: [u16; 1];
let _12: isize;
let _13: Adt42;
let _14: Adt53;
let _15: *const bool;
let _16: *mut [i64; 4];
let _17: Adt45;
let _18: ([i16; 6], i32, u16, i64, i16);
let _19: *const bool;
let _20: Adt42;
let _21: (u64, [bool; 6]);
let _22: Adt56;
let _23: i32;
let _24: [bool; 6];
let _25: *mut [i64; 4];
let _26: f32;
let _27: char;
let _28: [i64; 2];
let _29: ();
let _30: ();
{
RET.2 = _5;
_6 = (-15054_i16) * (-12139_i16);
RET.2 = -_5;
_2 = -166055895261091925033092568760273210125_i128;
RET.2 = 323557830073614692066847737198143191313_u128 as isize;
_4 = [42354_u16];
_8.fld0 = 7093744314219127951_i64 as i8;
RET.2 = _2 as isize;
_5 = 16179312454359776113_usize as isize;
_3 = 3289083298426867937_i64 as f32;
RET.2 = '\u{9d06a}' as isize;
_4 = [13982_u16];
RET.1 = 64_u8;
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET.0 = [13755400733584411934_usize,7_usize,3899038447329197459_usize,4_usize];
_7 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_7 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET.0 = [16967445989628353055_usize,5_usize,17610092571128513136_usize,8482945334815575118_usize];
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
64 => bb7,
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
_6 = (-3301_i16) << _2;
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_8.fld0 = 268209034357111595381472957843701440851_u128 as i8;
match RET.1 {
0 => bb8,
1 => bb9,
64 => bb11,
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
_8.fld0 = 77_i8 + 6_i8;
_9 = _8.fld0 == _8.fld0;
_4 = [35279_u16];
RET.2 = _5;
RET.0 = [7411577559980856459_usize,4_usize,4802112094025116045_usize,4393491207945497577_usize];
_5 = RET.2 | RET.2;
RET.2 = _5 | _5;
RET.1 = 170_u8;
RET.1 = !113_u8;
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_2 = -(-26256511705954912505192123869130395868_i128);
_8 = Adt53 { fld0: 19_i8 };
RET.0 = [8026027221580923508_usize,15670296523460007499_usize,4_usize,6_usize];
match _8.fld0 {
0 => bb3,
19 => bb12,
_ => bb2
}
}
bb12 = {
_10.fld0 = 3_usize as i8;
_2 = (-72819533177647995163041318363582644564_i128) << RET.2;
RET.0 = [7_usize,0_usize,13206804426272469004_usize,8242907441346985323_usize];
RET.0 = [4_usize,5_usize,0_usize,2_usize];
_4 = [21191_u16];
RET.0 = [2_usize,1_usize,899955094685050290_usize,8516382591884837316_usize];
_7 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_6 = 2601_i16;
_8 = Adt53 { fld0: _10.fld0 };
_7 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_7 = _1;
RET.2 = !_5;
_5 = !RET.2;
_7 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET.2 = -_5;
_8 = Move(_10);
RET.0 = [3_usize,8006442263390684192_usize,3900791016522854734_usize,24582043922678225_usize];
RET.0 = [2_usize,6948901004741044502_usize,8400375530773460760_usize,10543197513795726009_usize];
_8.fld0 = (-14_i8);
_11 = _4;
RET.0 = [15369624080266468047_usize,6685828856593786206_usize,5_usize,0_usize];
_2 = _6 as i128;
_8.fld0 = 2691413827_u32 as i8;
Call(_5 = fn10(RET, _6, RET, _1, _2, _2, RET.0, RET, _4, RET.1, RET.0, RET.0, RET.0, Move(_8), RET, RET), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET.0 = [7248056807192590339_usize,18306194705000370690_usize,12536862327901575113_usize,4_usize];
_14 = Adt53 { fld0: (-51_i8) };
_8.fld0 = _14.fld0 * _14.fld0;
RET.1 = 254_u8;
_2 = 11563557275014108579_u64 as i128;
_18.2 = 44705_u16 | 22689_u16;
_4 = [_18.2];
_14 = Adt53 { fld0: _8.fld0 };
_18.3 = _3 as i64;
_14 = Adt53 { fld0: _8.fld0 };
_2 = -(-81136925517726861620257447734360060934_i128);
_12 = _5 - _5;
_15 = core::ptr::addr_of!(_9);
_11 = _4;
_19 = core::ptr::addr_of!((*_15));
_19 = core::ptr::addr_of!((*_15));
_24 = [(*_15),(*_19),(*_19),(*_19),(*_19),_9];
(*_19) = !false;
_8 = Adt53 { fld0: _14.fld0 };
_9 = !false;
_2 = _18.3 as i128;
_1 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
match _6 {
0 => bb5,
1 => bb8,
2 => bb4,
3 => bb14,
4 => bb15,
2601 => bb17,
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
Return()
}
bb17 = {
_11 = [_18.2];
_22 = Adt56::Variant2 { fld0: _19 };
_5 = !RET.2;
SetDiscriminant(_22, 2);
_18.1 = -1172440596_i32;
_23 = 9649934243136008220_u64 as i32;
RET.0 = [3_usize,7172224812739456161_usize,4987566681545638655_usize,9331500143071524901_usize];
_11 = [_18.2];
_23 = _18.1 - _18.1;
RET.2 = _12 ^ _5;
_18.4 = -_6;
_11 = [_18.2];
_14 = Adt53 { fld0: _8.fld0 };
_8.fld0 = _14.fld0 * _14.fld0;
_18.0 = [_18.4,_18.4,_18.4,_18.4,_18.4,_6];
_23 = '\u{21427}' as i32;
_21 = (350836554233111609_u64, _24);
_8 = Adt53 { fld0: _14.fld0 };
_2 = RET.1 as i128;
_19 = core::ptr::addr_of!((*_15));
Goto(bb18)
}
bb18 = {
Call(_29 = dump_var(9_usize, 7_usize, Move(_7), 5_usize, Move(_5), 12_usize, Move(_12), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(9_usize, 6_usize, Move(_6), 1_usize, Move(_1), 30_usize, _30, 30_usize, _30), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: ([usize; 4], u8, isize),mut _2: i16,mut _3: ([usize; 4], u8, isize),mut _4: [u8; 8],mut _5: i128,mut _6: i128,mut _7: [usize; 4],mut _8: ([usize; 4], u8, isize),mut _9: [u16; 1],mut _10: u8,mut _11: [usize; 4],mut _12: [usize; 4],mut _13: [usize; 4],mut _14: Adt53,mut _15: ([usize; 4], u8, isize),mut _16: ([usize; 4], u8, isize)) -> isize {
mir! {
type RET = isize;
let _17: char;
let _18: *mut [i64; 4];
let _19: i64;
let _20: f64;
let _21: i16;
let _22: usize;
let _23: f32;
let _24: *const bool;
let _25: char;
let _26: Adt56;
let _27: i16;
let _28: isize;
let _29: char;
let _30: char;
let _31: [usize; 4];
let _32: &'static [i64; 4];
let _33: ();
let _34: ();
{
_2 = -(-13274_i16);
_3.0 = _16.0;
_17 = '\u{1e368}';
_3.2 = -_15.2;
_3 = _15;
_4 = [_1.1,_10,_15.1,_1.1,_10,_10,_8.1,_16.1];
_17 = '\u{682a9}';
_13 = [1_usize,0_usize,9598066238149635551_usize,1243960893320407563_usize];
_8 = _15;
_1.2 = _17 as isize;
_1.1 = !_3.1;
_16.1 = _3.1;
RET = _3.2 | _15.2;
_20 = 14202262075091600001_u64 as f64;
_1.1 = !_8.1;
_14 = Adt53 { fld0: 6_i8 };
_5 = _6 << _16.2;
_3.1 = _14.fld0 as u8;
_8.2 = _3.2 ^ RET;
_14 = Adt53 { fld0: (-93_i8) };
_22 = !8936998307741199222_usize;
_11 = [_22,_22,_22,_22];
_15.2 = _2 as isize;
_10 = !_1.1;
_15.1 = _8.1 << _14.fld0;
_19 = _20 as i64;
match _14.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211363 => bb9,
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
_1.0 = [_22,_22,_22,_22];
_16.1 = _10;
_7 = _1.0;
_11 = [_22,_22,_22,_22];
_13 = _16.0;
_1 = (_3.0, _15.1, _15.2);
_14 = Adt53 { fld0: (-124_i8) };
_6 = _22 as i128;
_14 = Adt53 { fld0: (-81_i8) };
_21 = _1.1 as i16;
_17 = '\u{a0dfa}';
_14 = Adt53 { fld0: (-21_i8) };
_19 = (-8150031467713762576_i64);
_8.2 = -RET;
RET = _21 as isize;
_16 = (_3.0, _3.1, _3.2);
_1 = (_15.0, _10, _16.2);
_15.1 = 1622661641_u32 as u8;
Goto(bb10)
}
bb10 = {
_1 = _3;
_1.1 = _10;
_3.1 = _10;
_21 = _2 << _2;
_17 = '\u{80c73}';
_1 = _3;
Call(_24 = fn11(_19, _15.0, _1, _17, _15, _3, _1.0, _16, _16.0, _1.2, _3, _21, _11, _1, _12, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = [_22,_22,_22,_22];
_20 = 10395_u16 as f64;
_3 = (_12, _15.1, _8.2);
_1.0 = _3.0;
RET = _15.2 | _8.2;
Goto(bb12)
}
bb12 = {
_8 = _3;
_1 = (_16.0, _16.1, _3.2);
_1.1 = !_8.1;
_17 = '\u{74c20}';
_9 = [15965_u16];
_21 = _2;
_1.0 = [_22,_22,_22,_22];
_7 = [_22,_22,_22,_22];
_4 = [_1.1,_3.1,_15.1,_15.1,_1.1,_3.1,_1.1,_1.1];
_4 = [_10,_15.1,_16.1,_16.1,_15.1,_15.1,_3.1,_10];
_10 = !_15.1;
_8.2 = _20 as isize;
_4 = [_8.1,_15.1,_3.1,_10,_1.1,_8.1,_8.1,_15.1];
_25 = _17;
_15.2 = !RET;
_3.2 = RET - _15.2;
_11 = [_22,_22,_22,_22];
RET = _3.2;
_27 = true as i16;
_4 = [_1.1,_10,_1.1,_1.1,_16.1,_16.1,_3.1,_1.1];
_21 = !_2;
_1.1 = !_15.1;
_9 = [6738_u16];
match _14.fld0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
340282366920938463463374607431768211435 => bb21,
_ => bb20
}
}
bb13 = {
_11 = [_22,_22,_22,_22];
_20 = 10395_u16 as f64;
_3 = (_12, _15.1, _8.2);
_1.0 = _3.0;
RET = _15.2 | _8.2;
Goto(bb12)
}
bb14 = {
_1 = _3;
_1.1 = _10;
_3.1 = _10;
_21 = _2 << _2;
_17 = '\u{80c73}';
_1 = _3;
Call(_24 = fn11(_19, _15.0, _1, _17, _15, _3, _1.0, _16, _16.0, _1.2, _3, _21, _11, _1, _12, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_1.0 = [_22,_22,_22,_22];
_16.1 = _10;
_7 = _1.0;
_11 = [_22,_22,_22,_22];
_13 = _16.0;
_1 = (_3.0, _15.1, _15.2);
_14 = Adt53 { fld0: (-124_i8) };
_6 = _22 as i128;
_14 = Adt53 { fld0: (-81_i8) };
_21 = _1.1 as i16;
_17 = '\u{a0dfa}';
_14 = Adt53 { fld0: (-21_i8) };
_19 = (-8150031467713762576_i64);
_8.2 = -RET;
RET = _21 as isize;
_16 = (_3.0, _3.1, _3.2);
_1 = (_15.0, _10, _16.2);
_15.1 = 1622661641_u32 as u8;
Goto(bb10)
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
Return()
}
bb20 = {
Return()
}
bb21 = {
_1.2 = !RET;
_3.0 = [_22,_22,_22,_22];
_9 = [33958_u16];
_28 = RET - _3.2;
_10 = _22 as u8;
_14.fld0 = (-47_i8);
_5 = _6;
_1.0 = [_22,_22,_22,_22];
_8 = _1;
_10 = 18774_u16 as u8;
_17 = _25;
_23 = _5 as f32;
_20 = 1699268920376216122_u64 as f64;
_16.1 = !_1.1;
_15 = _1;
_10 = _3.1 - _15.1;
_8.1 = _17 as u8;
_28 = _14.fld0 as isize;
_3.2 = _15.2;
_4 = [_1.1,_16.1,_15.1,_3.1,_1.1,_10,_16.1,_15.1];
Goto(bb22)
}
bb22 = {
Call(_33 = dump_var(10_usize, 8_usize, Move(_8), 21_usize, Move(_21), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_33 = dump_var(10_usize, 9_usize, Move(_9), 2_usize, Move(_2), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_33 = dump_var(10_usize, 4_usize, Move(_4), 6_usize, Move(_6), 7_usize, Move(_7), 34_usize, _34), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i64,mut _2: [usize; 4],mut _3: ([usize; 4], u8, isize),mut _4: char,mut _5: ([usize; 4], u8, isize),mut _6: ([usize; 4], u8, isize),mut _7: [usize; 4],mut _8: ([usize; 4], u8, isize),mut _9: [usize; 4],mut _10: isize,mut _11: ([usize; 4], u8, isize),mut _12: i16,mut _13: [usize; 4],mut _14: ([usize; 4], u8, isize),mut _15: [usize; 4],mut _16: ([usize; 4], u8, isize)) -> *const bool {
mir! {
type RET = *const bool;
let _17: [i32; 4];
let _18: isize;
let _19: isize;
let _20: Adt50;
let _21: isize;
let _22: f64;
let _23: i16;
let _24: isize;
let _25: char;
let _26: Adt58;
let _27: Adt42;
let _28: bool;
let _29: bool;
let _30: bool;
let _31: f64;
let _32: [bool; 4];
let _33: Adt54;
let _34: isize;
let _35: i128;
let _36: bool;
let _37: isize;
let _38: Adt54;
let _39: ();
let _40: ();
{
_2 = _6.0;
_5.0 = [12234904224158927601_usize,9326117853337764974_usize,2_usize,9839352553070655014_usize];
_14 = (_3.0, _16.1, _10);
_3 = (_14.0, _8.1, _8.2);
_5.0 = [2475419866095329470_usize,12858731072127072771_usize,5_usize,15289380852094735839_usize];
_3.0 = [13142312653010866983_usize,2956942665439685531_usize,10584266723011680769_usize,4_usize];
_3.1 = _12 as u8;
_15 = [15606558032851792955_usize,9611736807051095653_usize,3_usize,147121465775237399_usize];
_14 = (_7, _6.1, _8.2);
_4 = '\u{cf870}';
_18 = _14.2 << _3.1;
_17 = [1006301917_i32,709577940_i32,(-1866479046_i32),(-1990366664_i32)];
_8.1 = (-806856566_i32) as u8;
Call(_21 = core::intrinsics::transmute(_6.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.0 = [16530634367156175498_usize,5_usize,4_usize,6651310496987424594_usize];
_11.2 = -_14.2;
_11 = (_7, _16.1, _6.2);
_20.fld0.2.1 = _11.1 * _3.1;
_20.fld0.0 = [4606568681710669964_u64,6446298125284668936_u64,4675861965507643362_u64,15019441315360152491_u64,3696969050003366617_u64,15774384860064996811_u64,4369489983428610040_u64,4307175571639529850_u64];
_5.0 = [11777642311911000627_usize,3772316706101584078_usize,0_usize,9471427879532317491_usize];
_15 = _5.0;
_19 = !_14.2;
_6.1 = _4 as u8;
_6 = _5;
_3.2 = (-1973006935_i32) as isize;
_20.fld0.2 = _8;
_4 = '\u{d2b87}';
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463455224575964054448880 => bb10,
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
_19 = -_8.2;
_21 = (-1518010572_i32) as isize;
_19 = _10 & _10;
_15 = _9;
_15 = [1_usize,2_usize,6_usize,2_usize];
_14.1 = _20.fld0.2.1;
_9 = [7_usize,2_usize,17753285042804044218_usize,15416224819701330483_usize];
_8 = (_11.0, _14.1, _19);
_8.2 = _16.2;
_16 = (_9, _8.1, _19);
_12 = (-28562_i16);
_15 = [3_usize,2_usize,6_usize,15540788295562977450_usize];
_1 = 3278193987502353590_i64;
_5.2 = _3.2;
_6 = (_13, _8.1, _3.2);
match _12 {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb4,
340282366920938463463374607431768182894 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_22 = 5375926048708006668_u64 as f64;
_3.0 = [9399409593464780331_usize,3048500752440814085_usize,1290214164824982535_usize,5_usize];
_16 = (_5.0, _3.1, _8.2);
_22 = _19 as f64;
_11 = (_5.0, _3.1, _8.2);
_8.0 = [7_usize,4_usize,2_usize,3_usize];
_24 = _11.2 | _21;
_24 = 9183186137929433488_u64 as isize;
_19 = _4 as isize;
_14.2 = _10;
_16 = (_2, _11.1, _24);
_15 = [2_usize,2_usize,3646445107528482680_usize,3_usize];
RET = core::ptr::addr_of!(_28);
_5 = (_11.0, _6.1, _11.2);
_11.0 = _13;
(*RET) = true;
RET = core::ptr::addr_of!((*RET));
_3.0 = [365857815099678689_usize,11564781166360240383_usize,4485223256294801242_usize,3024176573889916808_usize];
RET = core::ptr::addr_of!((*RET));
_26 = Adt58::Variant1 { fld0: 142414250010295363665820302930121304530_i128 };
_7 = _9;
_11.2 = _18 & _8.2;
Call(_10 = fn12(_11.2, _21, _3, _5.0, _20.fld0.2, _16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_2 = [6_usize,1185348557817719347_usize,4700881770441334574_usize,6_usize];
_30 = !(*RET);
_13 = [9040894007846328379_usize,5_usize,0_usize,15062343976281116937_usize];
_29 = (*RET);
_30 = _10 == _18;
_3.1 = _16.1;
_4 = '\u{cc40c}';
_20.fld0.2.0 = [7539663733496450662_usize,14166929718701691528_usize,17716324163260818705_usize,7672342529735048318_usize];
_14 = (_6.0, _11.1, _10);
_6 = (_11.0, _20.fld0.2.1, _14.2);
RET = core::ptr::addr_of!((*RET));
_20.fld0.2.0 = _3.0;
_26 = Adt58::Variant1 { fld0: (-103832066261654887925104191132601611501_i128) };
_22 = 1249947377_u32 as f64;
RET = core::ptr::addr_of!((*RET));
_20.fld0.1 = RET;
_5.2 = !_6.2;
_20.fld0.0 = [16175636589439981447_u64,13163180048419001733_u64,12605443630646843058_u64,17538189375848151648_u64,6214735326515531884_u64,17217549882962077924_u64,16344940348841414485_u64,7582934263773812059_u64];
_9 = [2_usize,12559915075591011518_usize,14177915271990773775_usize,1863533757236492034_usize];
_12 = 10564_i16 << _8.2;
_28 = _30;
Call(_33 = fn13(_14, Move(_20), _15, _5, _5.2, _14.2, _10, _16), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<i16>(Variant(place!(Field::<Adt52>(Variant(_33, 1), 1)), 0), 3)) = 552621683_i32 as i16;
_20.fld0.2 = (Field::<[usize; 4]>(Variant(Field::<Adt52>(Variant(_33, 1), 1), 0), 2), _16.1, _5.2);
_3.2 = Field::<isize>(Variant(_33, 1), 0);
_20.fld0.2.2 = Field::<isize>(Variant(_33, 1), 0) * _19;
_14 = (_20.fld0.2.0, _20.fld0.2.1, _3.2);
RET = core::ptr::addr_of!((*RET));
_26 = Adt58::Variant1 { fld0: (-92450460001128374235352496576450878016_i128) };
SetDiscriminant(_33, 2);
_16 = _6;
place!(Field::<u64>(Variant(_33, 2), 1)) = !11495350038142308542_u64;
_3.1 = _5.1;
_3.2 = _8.1 as isize;
_3.0 = _14.0;
_1 = !1357884197450636844_i64;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(11_usize, 13_usize, Move(_13), 7_usize, Move(_7), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(11_usize, 8_usize, Move(_8), 21_usize, Move(_21), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(11_usize, 5_usize, Move(_5), 1_usize, Move(_1), 28_usize, Move(_28), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: ([usize; 4], u8, isize),mut _4: [usize; 4],mut _5: ([usize; 4], u8, isize),mut _6: ([usize; 4], u8, isize)) -> isize {
mir! {
type RET = isize;
let _7: [u16; 3];
let _8: [i64; 2];
let _9: (u64, [bool; 6]);
let _10: ([u64; 8],);
let _11: ();
let _12: ();
{
_3.2 = _1 >> _5.1;
RET = _1;
_3.0 = [2_usize,8696045230937615235_usize,5292414764835199299_usize,2_usize];
_5.0 = [5662752656092940906_usize,3_usize,17294044384353758486_usize,0_usize];
_3.0 = [5_usize,2973726861181910156_usize,5233853735899416740_usize,6155758048341115956_usize];
_6 = (_5.0, _3.1, _1);
_7 = [37931_u16,27457_u16,17757_u16];
_3.0 = [5_usize,4_usize,9302685025436379271_usize,1_usize];
_6.1 = _5.1;
RET = !_5.2;
_2 = _3.2 << _3.2;
_5.2 = !_2;
_3.0 = [3_usize,4106236598134348178_usize,0_usize,2_usize];
_8 = [5505004375432063223_i64,3931925032178682236_i64];
_1 = -_6.2;
_3.1 = _5.1 - _6.1;
_2 = !_5.2;
_8 = [4032956155229248542_i64,5124372825403277992_i64];
RET = _3.2 << _1;
_6.1 = _5.1 + _3.1;
_5.2 = _6.2;
_6.2 = 414464642_u32 as isize;
_6.2 = !_3.2;
_5.2 = true as isize;
_4 = _3.0;
RET = _2 - _2;
_3 = _6;
_8 = [(-8633457363330762151_i64),1766790425587124791_i64];
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(12_usize, 4_usize, Move(_4), 5_usize, Move(_5), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: ([usize; 4], u8, isize),mut _2: Adt50,mut _3: [usize; 4],mut _4: ([usize; 4], u8, isize),mut _5: isize,mut _6: isize,mut _7: isize,mut _8: ([usize; 4], u8, isize)) -> Adt54 {
mir! {
type RET = Adt54;
let _9: ([u64; 8],);
let _10: u64;
let _11: bool;
let _12: i64;
let _13: usize;
let _14: (usize, i64);
let _15: u16;
let _16: Adt48;
let _17: [u16; 1];
let _18: Adt47;
let _19: ([i16; 6], i32, u16, i64, i16);
let _20: i8;
let _21: i128;
let _22: [i64; 4];
let _23: char;
let _24: char;
let _25: Adt53;
let _26: [usize; 3];
let _27: f32;
let _28: Adt46;
let _29: Adt51;
let _30: Adt42;
let _31: (usize, i64);
let _32: u8;
let _33: bool;
let _34: &'static [i64; 4];
let _35: Adt52;
let _36: [u16; 6];
let _37: [u64; 8];
let _38: isize;
let _39: *mut [i64; 4];
let _40: u64;
let _41: [u8; 8];
let _42: f32;
let _43: char;
let _44: bool;
let _45: isize;
let _46: isize;
let _47: isize;
let _48: u128;
let _49: &'static [i64; 4];
let _50: [usize; 3];
let _51: i32;
let _52: u16;
let _53: [i32; 4];
let _54: [bool; 4];
let _55: isize;
let _56: ([u64; 8],);
let _57: ([u64; 8], *const bool, ([usize; 4], u8, isize));
let _58: bool;
let _59: i8;
let _60: Adt55;
let _61: [u16; 1];
let _62: Adt54;
let _63: [bool; 4];
let _64: isize;
let _65: ();
let _66: ();
{
_8.2 = -_7;
_2.fld0.0 = [5881523066325885162_u64,3175155901758297079_u64,4644686259434871354_u64,15422370240229274185_u64,10475769417023411031_u64,12706347342138075834_u64,13256937041943623228_u64,11709291860435709840_u64];
_8.2 = 1_usize as isize;
_5 = _1.2 - _6;
_4.2 = _1.2 & _5;
_2.fld0.2.2 = 2512307705_u32 as isize;
_2.fld0.2.0 = [1_usize,2199858929410171419_usize,14012262530224049414_usize,3_usize];
_8.0 = _2.fld0.2.0;
_10 = !1551603420398417029_u64;
_5 = _6 - _4.2;
_7 = _4.2;
Goto(bb1)
}
bb1 = {
_1.1 = _8.1;
_2.fld0.2.2 = _7 - _4.2;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1.1 = 46680_u16 as u8;
_2.fld0.2.0 = _3;
_9.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_2.fld0.2 = _4;
_4.2 = _2.fld0.2.2;
_3 = [10384216012393621627_usize,9735478759233837781_usize,7_usize,2_usize];
_10 = 13730559521430100894476967400913477569_u128 as u64;
_6 = _8.1 as isize;
_2.fld0.2.1 = _4.1 - _8.1;
_4.2 = _4.1 as isize;
_9 = (_2.fld0.0,);
_4.1 = !_2.fld0.2.1;
_10 = 12307697506498438767_u64 * 12484068754978738372_u64;
Goto(bb2)
}
bb2 = {
_8.0 = [7_usize,2_usize,4_usize,18305629852336376480_usize];
_1.0 = [2_usize,4_usize,3292665518324704546_usize,7637143693138124814_usize];
_1.0 = [17950550307119954820_usize,11822251093719384213_usize,3698498298171406203_usize,2_usize];
_3 = _4.0;
_2.fld0.2.1 = !_1.1;
_8 = (_4.0, _4.1, _2.fld0.2.2);
_4.0 = [6403795365016325235_usize,6_usize,6_usize,0_usize];
_4.0 = [0_usize,10934658970049895160_usize,6_usize,6_usize];
_4 = (_3, _1.1, _1.2);
_1.0 = [3_usize,3_usize,9034294467361525701_usize,5_usize];
_2.fld0.2.0 = [2_usize,3797836240159686809_usize,4_usize,7_usize];
_2.fld0.2 = (_4.0, _1.1, _5);
_5 = -_2.fld0.2.2;
_1 = (_3, _8.1, _5);
_7 = _5 + _1.2;
_4 = (_1.0, _8.1, _1.2);
Call(_2.fld0.2.1 = core::intrinsics::bswap(_1.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = -_4.2;
_11 = true;
_10 = 98_i8 as u64;
_1.0 = [10748352173095747183_usize,16151735152007208500_usize,5_usize,3530562363326325303_usize];
_13 = !9112604393626267156_usize;
_8.0 = _3;
_2.fld0.2.1 = _1.1 & _4.1;
_1.2 = _4.2;
_12 = _10 as i64;
_8.2 = -_6;
_14.0 = _13 | _13;
_14.0 = _13;
_8.1 = _4.1;
_11 = !false;
_14.0 = _11 as usize;
_8.2 = 2104314302_i32 as isize;
_15 = 58746_u16;
_14.1 = !_12;
_14.0 = _13 >> _2.fld0.2.2;
Call(_4 = fn14(Move(_2), _1, _14, _5, _5, _1.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.0 = [_14.0,_14.0,_14.0,_14.0];
_6 = _1.2;
_1.0 = [_14.0,_14.0,_14.0,_14.0];
_9.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_4.2 = (-121958240689140760434037008117798417630_i128) as isize;
_4.0 = [_14.0,_14.0,_14.0,_14.0];
_2.fld0.1 = core::ptr::addr_of!(_11);
_2.fld0.2.1 = !_8.1;
_2.fld0.2.0 = [_14.0,_14.0,_14.0,_14.0];
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_9.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
match _15 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
58746 => bb12,
_ => bb11
}
}
bb5 = {
_6 = -_4.2;
_11 = true;
_10 = 98_i8 as u64;
_1.0 = [10748352173095747183_usize,16151735152007208500_usize,5_usize,3530562363326325303_usize];
_13 = !9112604393626267156_usize;
_8.0 = _3;
_2.fld0.2.1 = _1.1 & _4.1;
_1.2 = _4.2;
_12 = _10 as i64;
_8.2 = -_6;
_14.0 = _13 | _13;
_14.0 = _13;
_8.1 = _4.1;
_11 = !false;
_14.0 = _11 as usize;
_8.2 = 2104314302_i32 as isize;
_15 = 58746_u16;
_14.1 = !_12;
_14.0 = _13 >> _2.fld0.2.2;
Call(_4 = fn14(Move(_2), _1, _14, _5, _5, _1.2), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_8.0 = [7_usize,2_usize,4_usize,18305629852336376480_usize];
_1.0 = [2_usize,4_usize,3292665518324704546_usize,7637143693138124814_usize];
_1.0 = [17950550307119954820_usize,11822251093719384213_usize,3698498298171406203_usize,2_usize];
_3 = _4.0;
_2.fld0.2.1 = !_1.1;
_8 = (_4.0, _4.1, _2.fld0.2.2);
_4.0 = [6403795365016325235_usize,6_usize,6_usize,0_usize];
_4.0 = [0_usize,10934658970049895160_usize,6_usize,6_usize];
_4 = (_3, _1.1, _1.2);
_1.0 = [3_usize,3_usize,9034294467361525701_usize,5_usize];
_2.fld0.2.0 = [2_usize,3797836240159686809_usize,4_usize,7_usize];
_2.fld0.2 = (_4.0, _1.1, _5);
_5 = -_2.fld0.2.2;
_1 = (_3, _8.1, _5);
_7 = _5 + _1.2;
_4 = (_1.0, _8.1, _1.2);
Call(_2.fld0.2.1 = core::intrinsics::bswap(_1.1), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_1.1 = _8.1;
_2.fld0.2.2 = _7 - _4.2;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1.1 = 46680_u16 as u8;
_2.fld0.2.0 = _3;
_9.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_2.fld0.2 = _4;
_4.2 = _2.fld0.2.2;
_3 = [10384216012393621627_usize,9735478759233837781_usize,7_usize,2_usize];
_10 = 13730559521430100894476967400913477569_u128 as u64;
_6 = _8.1 as isize;
_2.fld0.2.1 = _4.1 - _8.1;
_4.2 = _4.1 as isize;
_9 = (_2.fld0.0,);
_4.1 = !_2.fld0.2.1;
_10 = 12307697506498438767_u64 * 12484068754978738372_u64;
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
_2.fld0.1 = core::ptr::addr_of!(_11);
_2.fld0.2.2 = _6;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_12 = _14.1;
_2.fld0.2.2 = _1.2 << _14.0;
_6 = _7 - _1.2;
_12 = -_14.1;
_8.0 = _1.0;
_8 = (_4.0, _4.1, _6);
_4.2 = _7 & _2.fld0.2.2;
_9 = (_2.fld0.0,);
_3 = _8.0;
_1.0 = [_14.0,_14.0,_14.0,_14.0];
_1 = _2.fld0.2;
_2.fld0.2.2 = -_6;
_8.2 = _7 | _4.2;
_8.0 = [_14.0,_14.0,_14.0,_14.0];
_10 = 10877314185963250208_u64 - 14647186838372046756_u64;
_19.2 = !_15;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
match _15 {
0 => bb1,
58746 => bb13,
_ => bb10
}
}
bb13 = {
_8.0 = [_14.0,_14.0,_14.0,_14.0];
_19.3 = _12 & _14.1;
_9.0 = _2.fld0.0;
_3 = _2.fld0.2.0;
_1.0 = _4.0;
_19.2 = _15 << _7;
_8.2 = (-143764295311273047431281268063525494776_i128) as isize;
_14.1 = !_19.3;
_22 = [_12,_19.3,_14.1,_14.1];
_1 = (_4.0, _8.1, _2.fld0.2.2);
_1.1 = !_4.1;
_15 = 5_i8 as u16;
_14.1 = -_19.3;
_2.fld0.2 = _4;
_20 = !53_i8;
_2.fld0.1 = core::ptr::addr_of!(_11);
_16 = Adt48::Variant0 { fld0: _2.fld0.1,fld1: _9.0,fld2: _5,fld3: _20,fld4: _14,fld5: 2995401291_u32 };
_1.1 = _4.1;
_22 = [_14.1,_19.3,Field::<(usize, i64)>(Variant(_16, 0), 4).1,_19.3];
Goto(bb14)
}
bb14 = {
_14.1 = _19.3 | Field::<(usize, i64)>(Variant(_16, 0), 4).1;
_1.0 = [_14.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,_14.0];
_26 = [_14.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,Field::<(usize, i64)>(Variant(_16, 0), 4).0];
Call(_19.4 = core::intrinsics::bswap((-11226_i16)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_21 = 96816532224572350428703372543580195734_i128;
_25 = Adt53 { fld0: _20 };
_17 = [_19.2];
_3 = [_14.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,Field::<(usize, i64)>(Variant(_16, 0), 4).0];
_2.fld0.1 = Field::<*const bool>(Variant(_16, 0), 0);
_7 = -_1.2;
_2.fld0.2.2 = Field::<isize>(Variant(_16, 0), 2) & _6;
_22 = [_12,_19.3,Field::<(usize, i64)>(Variant(_16, 0), 4).1,_19.3];
_27 = 7607_i16 as f32;
_20 = Field::<i8>(Variant(_16, 0), 3);
match _21 {
0 => bb2,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
96816532224572350428703372543580195734 => bb21,
_ => bb20
}
}
bb16 = {
_8.0 = [7_usize,2_usize,4_usize,18305629852336376480_usize];
_1.0 = [2_usize,4_usize,3292665518324704546_usize,7637143693138124814_usize];
_1.0 = [17950550307119954820_usize,11822251093719384213_usize,3698498298171406203_usize,2_usize];
_3 = _4.0;
_2.fld0.2.1 = !_1.1;
_8 = (_4.0, _4.1, _2.fld0.2.2);
_4.0 = [6403795365016325235_usize,6_usize,6_usize,0_usize];
_4.0 = [0_usize,10934658970049895160_usize,6_usize,6_usize];
_4 = (_3, _1.1, _1.2);
_1.0 = [3_usize,3_usize,9034294467361525701_usize,5_usize];
_2.fld0.2.0 = [2_usize,3797836240159686809_usize,4_usize,7_usize];
_2.fld0.2 = (_4.0, _1.1, _5);
_5 = -_2.fld0.2.2;
_1 = (_3, _8.1, _5);
_7 = _5 + _1.2;
_4 = (_1.0, _8.1, _1.2);
Call(_2.fld0.2.1 = core::intrinsics::bswap(_1.1), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_8.0 = [_14.0,_14.0,_14.0,_14.0];
_19.3 = _12 & _14.1;
_9.0 = _2.fld0.0;
_3 = _2.fld0.2.0;
_1.0 = _4.0;
_19.2 = _15 << _7;
_8.2 = (-143764295311273047431281268063525494776_i128) as isize;
_14.1 = !_19.3;
_22 = [_12,_19.3,_14.1,_14.1];
_1 = (_4.0, _8.1, _2.fld0.2.2);
_1.1 = !_4.1;
_15 = 5_i8 as u16;
_14.1 = -_19.3;
_2.fld0.2 = _4;
_20 = !53_i8;
_2.fld0.1 = core::ptr::addr_of!(_11);
_16 = Adt48::Variant0 { fld0: _2.fld0.1,fld1: _9.0,fld2: _5,fld3: _20,fld4: _14,fld5: 2995401291_u32 };
_1.1 = _4.1;
_22 = [_14.1,_19.3,Field::<(usize, i64)>(Variant(_16, 0), 4).1,_19.3];
Goto(bb14)
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_2.fld0.2.1 = !_4.1;
place!(Field::<i8>(Variant(_16, 0), 3)) = _25.fld0;
_12 = Field::<(usize, i64)>(Variant(_16, 0), 4).1;
_19.1 = -1864795790_i32;
_17 = [_19.2];
_2.fld0.2.1 = !_4.1;
_22 = [_19.3,Field::<(usize, i64)>(Variant(_16, 0), 4).1,_19.3,_14.1];
place!(Field::<i8>(Variant(_16, 0), 3)) = Field::<isize>(Variant(_16, 0), 2) as i8;
_8 = (_3, _2.fld0.2.1, _5);
_4.2 = !_2.fld0.2.2;
_1 = (_8.0, _2.fld0.2.1, _4.2);
_14 = (Field::<(usize, i64)>(Variant(_16, 0), 4).0, _12);
_33 = _11;
_19.4 = (-31173_i16);
_2.fld0.2.0 = [_14.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,_14.0];
_8 = (_1.0, _2.fld0.2.1, _4.2);
_19.0 = [_19.4,_19.4,_19.4,_19.4,_19.4,_19.4];
_19.1 = 967116025_i32 >> Field::<(usize, i64)>(Variant(_16, 0), 4).0;
_17 = [_19.2];
_31 = (Field::<(usize, i64)>(Variant(_16, 0), 4).0, Field::<(usize, i64)>(Variant(_16, 0), 4).1);
_19.1 = (-623347060_i32) ^ 1146118649_i32;
_34 = &_22;
_4 = (_1.0, _8.1, _2.fld0.2.2);
_22 = [_31.1,_14.1,_14.1,_31.1];
match _19.4 {
0 => bb8,
1 => bb2,
2 => bb9,
3 => bb5,
4 => bb22,
340282366920938463463374607431768180283 => bb24,
_ => bb23
}
}
bb22 = {
_6 = -_4.2;
_11 = true;
_10 = 98_i8 as u64;
_1.0 = [10748352173095747183_usize,16151735152007208500_usize,5_usize,3530562363326325303_usize];
_13 = !9112604393626267156_usize;
_8.0 = _3;
_2.fld0.2.1 = _1.1 & _4.1;
_1.2 = _4.2;
_12 = _10 as i64;
_8.2 = -_6;
_14.0 = _13 | _13;
_14.0 = _13;
_8.1 = _4.1;
_11 = !false;
_14.0 = _11 as usize;
_8.2 = 2104314302_i32 as isize;
_15 = 58746_u16;
_14.1 = !_12;
_14.0 = _13 >> _2.fld0.2.2;
Call(_4 = fn14(Move(_2), _1, _14, _5, _5, _1.2), ReturnTo(bb4), UnwindUnreachable())
}
bb23 = {
Return()
}
bb24 = {
_24 = '\u{91e62}';
_19.3 = _14.1;
_31 = (_14.0, _19.3);
_9 = (Field::<[u64; 8]>(Variant(_16, 0), 1),);
_31.1 = _19.3;
_4.2 = _6 + _5;
_25.fld0 = Field::<i8>(Variant(_16, 0), 3) ^ Field::<i8>(Variant(_16, 0), 3);
_1 = (_8.0, _4.1, _8.2);
_8.0 = [_31.0,_14.0,_31.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0];
_8 = _1;
_22 = [_12,_19.3,_31.1,_14.1];
_2.fld0.2.1 = _27 as u8;
_31.0 = !Field::<(usize, i64)>(Variant(_16, 0), 4).0;
_8.1 = _1.1 >> _5;
_31.1 = _12 & _19.3;
_36 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_5 = _10 as isize;
_37 = [_10,_10,_10,_10,_10,_10,_10,_10];
Goto(bb25)
}
bb25 = {
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_17 = [_19.2];
_13 = Field::<(usize, i64)>(Variant(_16, 0), 4).0;
_39 = core::ptr::addr_of_mut!(_22);
_2.fld0.2.0 = [Field::<(usize, i64)>(Variant(_16, 0), 4).0,_13,_13,_13];
_2.fld0.2.2 = _24 as isize;
_15 = !_19.2;
_8.2 = Field::<isize>(Variant(_16, 0), 2);
_14.0 = !Field::<(usize, i64)>(Variant(_16, 0), 4).0;
_1.1 = _8.1;
_31.1 = _19.2 as i64;
_5 = !_1.2;
_28 = Adt46::Variant1 { fld0: _31.1 };
_41 = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_34 = &_22;
_32 = _8.1 & _8.1;
Goto(bb26)
}
bb26 = {
_43 = _24;
_14.1 = _31.1 | Field::<i64>(Variant(_28, 1), 0);
_29 = Adt51::Variant0 { fld0: _19.0,fld1: Field::<i8>(Variant(_16, 0), 3),fld2: _41 };
_13 = _31.0 >> _14.0;
_13 = _31.0 ^ Field::<(usize, i64)>(Variant(_16, 0), 4).0;
place!(Field::<isize>(Variant(_16, 0), 2)) = _1.2;
place!(Field::<i8>(Variant(_29, 0), 1)) = !_25.fld0;
_19.4 = 25766_i16;
_45 = Field::<isize>(Variant(_16, 0), 2);
_19.3 = Field::<i64>(Variant(_28, 1), 0);
_46 = _45;
_32 = _1.1 << _31.0;
_16 = Adt48::Variant0 { fld0: _2.fld0.1,fld1: _9.0,fld2: _7,fld3: _25.fld0,fld4: _31,fld5: 4039729223_u32 };
place!(Field::<i8>(Variant(_29, 0), 1)) = _7 as i8;
_22 = [_14.1,_19.3,Field::<i64>(Variant(_28, 1), 0),_19.3];
match _21 {
0 => bb21,
1 => bb2,
2 => bb19,
3 => bb27,
4 => bb28,
96816532224572350428703372543580195734 => bb30,
_ => bb29
}
}
bb27 = {
_8.0 = [_14.0,_14.0,_14.0,_14.0];
_19.3 = _12 & _14.1;
_9.0 = _2.fld0.0;
_3 = _2.fld0.2.0;
_1.0 = _4.0;
_19.2 = _15 << _7;
_8.2 = (-143764295311273047431281268063525494776_i128) as isize;
_14.1 = !_19.3;
_22 = [_12,_19.3,_14.1,_14.1];
_1 = (_4.0, _8.1, _2.fld0.2.2);
_1.1 = !_4.1;
_15 = 5_i8 as u16;
_14.1 = -_19.3;
_2.fld0.2 = _4;
_20 = !53_i8;
_2.fld0.1 = core::ptr::addr_of!(_11);
_16 = Adt48::Variant0 { fld0: _2.fld0.1,fld1: _9.0,fld2: _5,fld3: _20,fld4: _14,fld5: 2995401291_u32 };
_1.1 = _4.1;
_22 = [_14.1,_19.3,Field::<(usize, i64)>(Variant(_16, 0), 4).1,_19.3];
Goto(bb14)
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
place!(Field::<[u8; 8]>(Variant(_29, 0), 2)) = _41;
_27 = _46 as f32;
_38 = 285665469549571931024330637879402665075_u128 as isize;
match _21 {
0 => bb1,
1 => bb31,
2 => bb32,
3 => bb33,
96816532224572350428703372543580195734 => bb35,
_ => bb34
}
}
bb31 = {
Return()
}
bb32 = {
_6 = -_4.2;
_11 = true;
_10 = 98_i8 as u64;
_1.0 = [10748352173095747183_usize,16151735152007208500_usize,5_usize,3530562363326325303_usize];
_13 = !9112604393626267156_usize;
_8.0 = _3;
_2.fld0.2.1 = _1.1 & _4.1;
_1.2 = _4.2;
_12 = _10 as i64;
_8.2 = -_6;
_14.0 = _13 | _13;
_14.0 = _13;
_8.1 = _4.1;
_11 = !false;
_14.0 = _11 as usize;
_8.2 = 2104314302_i32 as isize;
_15 = 58746_u16;
_14.1 = !_12;
_14.0 = _13 >> _2.fld0.2.2;
Call(_4 = fn14(Move(_2), _1, _14, _5, _5, _1.2), ReturnTo(bb4), UnwindUnreachable())
}
bb33 = {
_8.0 = [7_usize,2_usize,4_usize,18305629852336376480_usize];
_1.0 = [2_usize,4_usize,3292665518324704546_usize,7637143693138124814_usize];
_1.0 = [17950550307119954820_usize,11822251093719384213_usize,3698498298171406203_usize,2_usize];
_3 = _4.0;
_2.fld0.2.1 = !_1.1;
_8 = (_4.0, _4.1, _2.fld0.2.2);
_4.0 = [6403795365016325235_usize,6_usize,6_usize,0_usize];
_4.0 = [0_usize,10934658970049895160_usize,6_usize,6_usize];
_4 = (_3, _1.1, _1.2);
_1.0 = [3_usize,3_usize,9034294467361525701_usize,5_usize];
_2.fld0.2.0 = [2_usize,3797836240159686809_usize,4_usize,7_usize];
_2.fld0.2 = (_4.0, _1.1, _5);
_5 = -_2.fld0.2.2;
_1 = (_3, _8.1, _5);
_7 = _5 + _1.2;
_4 = (_1.0, _8.1, _1.2);
Call(_2.fld0.2.1 = core::intrinsics::bswap(_1.1), ReturnTo(bb3), UnwindUnreachable())
}
bb34 = {
_2.fld0.1 = core::ptr::addr_of!(_11);
_2.fld0.2.2 = _6;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_12 = _14.1;
_2.fld0.2.2 = _1.2 << _14.0;
_6 = _7 - _1.2;
_12 = -_14.1;
_8.0 = _1.0;
_8 = (_4.0, _4.1, _6);
_4.2 = _7 & _2.fld0.2.2;
_9 = (_2.fld0.0,);
_3 = _8.0;
_1.0 = [_14.0,_14.0,_14.0,_14.0];
_1 = _2.fld0.2;
_2.fld0.2.2 = -_6;
_8.2 = _7 | _4.2;
_8.0 = [_14.0,_14.0,_14.0,_14.0];
_10 = 10877314185963250208_u64 - 14647186838372046756_u64;
_19.2 = !_15;
_2.fld0.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
match _15 {
0 => bb1,
58746 => bb13,
_ => bb10
}
}
bb35 = {
_19 = (Field::<[i16; 6]>(Variant(_29, 0), 0), (-131243182_i32), _15, _14.1, 30807_i16);
_2.fld0 = (Field::<[u64; 8]>(Variant(_16, 0), 1), Field::<*const bool>(Variant(_16, 0), 0), _1);
_31 = (_14.0, Field::<(usize, i64)>(Variant(_16, 0), 4).1);
_8.1 = _32 << _6;
_50 = [_14.0,_13,_13];
_7 = _4.2;
_25.fld0 = Field::<i8>(Variant(_16, 0), 3);
_36 = [_15,_15,_15,_19.2,_19.2,_15];
_53 = [_19.1,_19.1,_19.1,_19.1];
_35 = Adt52::Variant0 { fld0: _53,fld1: Move(_29),fld2: _2.fld0.2.0,fld3: _19.4 };
Goto(bb36)
}
bb36 = {
_20 = _21 as i8;
_14.0 = _31.0;
_50 = [_14.0,Field::<(usize, i64)>(Variant(_16, 0), 4).0,_31.0];
_27 = _19.2 as f32;
_54 = [_33,_11,_33,_11];
_1.0 = [_31.0,_13,_14.0,_14.0];
_5 = _45;
_14.1 = -_31.1;
_40 = _4.2 as u64;
_45 = _24 as isize;
place!(Field::<(usize, i64)>(Variant(_16, 0), 4)) = (_14.0, _19.3);
place!(Field::<i8>(Variant(place!(Field::<Adt51>(Variant(_35, 0), 1)), 0), 1)) = Field::<i8>(Variant(_16, 0), 3) * _25.fld0;
place!(Field::<[i16; 6]>(Variant(place!(Field::<Adt51>(Variant(_35, 0), 1)), 0), 0)) = [_19.4,Field::<i16>(Variant(_35, 0), 3),_19.4,Field::<i16>(Variant(_35, 0), 3),_19.4,Field::<i16>(Variant(_35, 0), 3)];
_26 = _50;
_57.2.1 = !_32;
_25.fld0 = -Field::<i8>(Variant(_16, 0), 3);
match _19.4 {
0 => bb3,
30807 => bb38,
_ => bb37
}
}
bb37 = {
Return()
}
bb38 = {
place!(Field::<[i32; 4]>(Variant(_35, 0), 0)) = _53;
_25.fld0 = Field::<i8>(Variant(_16, 0), 3);
place!(Field::<(usize, i64)>(Variant(_16, 0), 4)).0 = 16933474026909891904476129460697840693_u128 as usize;
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(_35, 0), 1)), 0), 2)) = [_32,_2.fld0.2.1,_57.2.1,_1.1,_2.fld0.2.1,_1.1,_57.2.1,_1.1];
_1.2 = 128498935786748211322261099093653246473_u128 as isize;
_33 = !_11;
_25.fld0 = _21 as i8;
_51 = _19.1;
_2.fld0.2.0 = [_31.0,_14.0,_14.0,_31.0];
_13 = !_31.0;
_59 = _27 as i8;
_4.1 = _2.fld0.2.1;
_1 = (_8.0, _8.1, _7);
_34 = &_22;
RET = Adt54::Variant1 { fld0: _4.2,fld1: Move(_35) };
_18 = Adt47::Variant3 { fld0: Field::<(usize, i64)>(Variant(_16, 0), 4) };
_9.0 = [_40,_40,_40,_40,_40,_40,_40,_40];
_51 = -_19.1;
_2.fld0.2.0 = _4.0;
_42 = _27 + _27;
Goto(bb39)
}
bb39 = {
Call(_65 = dump_var(13_usize, 24_usize, Move(_24), 41_usize, Move(_41), 6_usize, Move(_6), 36_usize, Move(_36)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_65 = dump_var(13_usize, 9_usize, Move(_9), 43_usize, Move(_43), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_65 = dump_var(13_usize, 33_usize, Move(_33), 14_usize, Move(_14), 4_usize, Move(_4), 51_usize, Move(_51)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_65 = dump_var(13_usize, 40_usize, Move(_40), 54_usize, Move(_54), 31_usize, Move(_31), 22_usize, Move(_22)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_65 = dump_var(13_usize, 12_usize, Move(_12), 50_usize, Move(_50), 66_usize, _66, 66_usize, _66), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: Adt50,mut _2: ([usize; 4], u8, isize),mut _3: (usize, i64),mut _4: isize,mut _5: isize,mut _6: isize) -> ([usize; 4], u8, isize) {
mir! {
type RET = ([usize; 4], u8, isize);
let _7: Adt43;
let _8: usize;
let _9: isize;
let _10: char;
let _11: ([usize; 4], u8, isize);
let _12: ();
let _13: ();
{
RET.0 = _1.fld0.2.0;
_1.fld0.2.2 = (-32713_i16) as isize;
_2.0 = [_3.0,_3.0,_3.0,_3.0];
RET.1 = _1.fld0.2.1;
_2.0 = [_3.0,_3.0,_3.0,_3.0];
RET = (_1.fld0.2.0, _1.fld0.2.1, _2.2);
_1.fld0.0 = [12623241119594987696_u64,9748561203956554557_u64,16415374698930644543_u64,2423618658156707126_u64,3888511979227153491_u64,1805426108819925138_u64,5863556273123482462_u64,3195518057745326584_u64];
_8 = _3.0;
RET.2 = _2.2 << _2.2;
RET.0 = _2.0;
_5 = _4 + _6;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(14_usize, 8_usize, Move(_8), 5_usize, Move(_5), 3_usize, Move(_3), 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: i32,mut _6: ([usize; 4], u8, isize),mut _7: ([usize; 4], u8, isize)) -> i128 {
mir! {
type RET = i128;
let _8: Adt46;
let _9: u8;
let _10: bool;
let _11: u64;
let _12: [usize; 3];
let _13: [i32; 4];
let _14: Adt45;
let _15: [bool; 6];
let _16: Adt43;
let _17: char;
let _18: (usize, i64);
let _19: Adt56;
let _20: i32;
let _21: u64;
let _22: char;
let _23: i16;
let _24: f32;
let _25: Adt56;
let _26: u128;
let _27: ();
let _28: ();
{
_6.1 = !_7.1;
_6.2 = _7.2 << _1;
_5 = !661575861_i32;
_7 = (_6.0, _6.1, _6.2);
_7.0 = [10625256098762705696_usize,958601674345742568_usize,3_usize,7_usize];
_5 = !606001656_i32;
RET = 4401887119010072710_u64 as i128;
_7.2 = _2;
_9 = 47704_u16 as u8;
_6.2 = _3;
Call(_7.1 = fn16(_6.2, _2, _2, _2, _3, _7.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (_6.0, _6.1, _6.2);
RET = !(-47564629713670751740993029145807284560_i128);
_8 = Adt46::Variant1 { fld0: 4571183059896652823_i64 };
_7.2 = _6.2;
_9 = _6.1;
_7.2 = _1 + _3;
_7.2 = 6650739334938493008_i64 as isize;
_6.2 = _3;
place!(Field::<i64>(Variant(_8, 1), 0)) = -(-5551863934614327039_i64);
_4 = '\u{247e1}' as isize;
_7 = (_6.0, _6.1, _6.2);
_2 = _1;
_6 = (_7.0, _7.1, _2);
RET = 148399112804183359811430527163512263510_i128 & (-3649408888291648538248599725578654364_i128);
_7.1 = _9 | _6.1;
_11 = !13898518336082683629_u64;
RET = !37471791285529411891102032791013335561_i128;
_13 = [_5,_5,_5,_5];
_8 = Adt46::Variant1 { fld0: (-2193697387970982250_i64) };
place!(Field::<i64>(Variant(_8, 1), 0)) = (-5_i8) as i64;
_7 = (_6.0, _9, _3);
_5 = 350336635_i32;
_4 = _7.2 - _1;
_4 = _1;
Goto(bb2)
}
bb2 = {
_7.1 = !_9;
_7.2 = -_2;
Goto(bb3)
}
bb3 = {
_7.1 = Field::<i64>(Variant(_8, 1), 0) as u8;
_3 = 24675_u16 as isize;
_7.2 = _1;
match _5 {
0 => bb2,
350336635 => bb5,
_ => bb4
}
}
bb4 = {
_7 = (_6.0, _6.1, _6.2);
RET = !(-47564629713670751740993029145807284560_i128);
_8 = Adt46::Variant1 { fld0: 4571183059896652823_i64 };
_7.2 = _6.2;
_9 = _6.1;
_7.2 = _1 + _3;
_7.2 = 6650739334938493008_i64 as isize;
_6.2 = _3;
place!(Field::<i64>(Variant(_8, 1), 0)) = -(-5551863934614327039_i64);
_4 = '\u{247e1}' as isize;
_7 = (_6.0, _6.1, _6.2);
_2 = _1;
_6 = (_7.0, _7.1, _2);
RET = 148399112804183359811430527163512263510_i128 & (-3649408888291648538248599725578654364_i128);
_7.1 = _9 | _6.1;
_11 = !13898518336082683629_u64;
RET = !37471791285529411891102032791013335561_i128;
_13 = [_5,_5,_5,_5];
_8 = Adt46::Variant1 { fld0: (-2193697387970982250_i64) };
place!(Field::<i64>(Variant(_8, 1), 0)) = (-5_i8) as i64;
_7 = (_6.0, _9, _3);
_5 = 350336635_i32;
_4 = _7.2 - _1;
_4 = _1;
Goto(bb2)
}
bb5 = {
_6 = (_7.0, _7.1, _2);
_11 = 15867948656248044064_u64;
SetDiscriminant(_8, 1);
_15 = [true,true,false,true,false,true];
_6.0 = _7.0;
_9 = _7.1;
_12 = [0_usize,6_usize,6_usize];
_4 = !_6.2;
_7.1 = _11 as u8;
_11 = !6990176408128598784_u64;
_3 = _1 ^ _4;
_11 = !16498447185038494267_u64;
_13 = [_5,_5,_5,_5];
match _5 {
0 => bb1,
1 => bb3,
2 => bb6,
350336635 => bb8,
_ => bb7
}
}
bb6 = {
_7 = (_6.0, _6.1, _6.2);
RET = !(-47564629713670751740993029145807284560_i128);
_8 = Adt46::Variant1 { fld0: 4571183059896652823_i64 };
_7.2 = _6.2;
_9 = _6.1;
_7.2 = _1 + _3;
_7.2 = 6650739334938493008_i64 as isize;
_6.2 = _3;
place!(Field::<i64>(Variant(_8, 1), 0)) = -(-5551863934614327039_i64);
_4 = '\u{247e1}' as isize;
_7 = (_6.0, _6.1, _6.2);
_2 = _1;
_6 = (_7.0, _7.1, _2);
RET = 148399112804183359811430527163512263510_i128 & (-3649408888291648538248599725578654364_i128);
_7.1 = _9 | _6.1;
_11 = !13898518336082683629_u64;
RET = !37471791285529411891102032791013335561_i128;
_13 = [_5,_5,_5,_5];
_8 = Adt46::Variant1 { fld0: (-2193697387970982250_i64) };
place!(Field::<i64>(Variant(_8, 1), 0)) = (-5_i8) as i64;
_7 = (_6.0, _9, _3);
_5 = 350336635_i32;
_4 = _7.2 - _1;
_4 = _1;
Goto(bb2)
}
bb7 = {
_7.1 = !_9;
_7.2 = -_2;
Goto(bb3)
}
bb8 = {
_18 = (8850416756177778697_usize, (-7285056007925403566_i64));
_18.0 = 13141991539617674620_usize;
_7.1 = _9;
_6.1 = !_7.1;
_6 = (_7.0, _9, _1);
_12 = [_18.0,_18.0,_18.0];
_18 = (2_usize, 8482256755051614615_i64);
_7.2 = -_1;
_18.1 = (-7613936596542601163_i64);
_7.2 = -_6.2;
_10 = false & false;
_18.0 = 2_usize;
_18.1 = -4584018312800255163_i64;
RET = !17393071974970000873864482067032534402_i128;
_17 = '\u{8ad27}';
_7.2 = _6.2 - _3;
_13 = [_5,_5,_5,_5];
_10 = !true;
_20 = _5;
_11 = !8128877670982344685_u64;
place!(Field::<i64>(Variant(_8, 1), 0)) = _18.1 | _18.1;
_18.0 = 14604197306790752191_usize;
_7.1 = 7694_u16 as u8;
_15 = [_10,_10,_10,_10,_10,_10];
_18 = (1_usize, Field::<i64>(Variant(_8, 1), 0));
_17 = '\u{8b5c7}';
_6 = (_7.0, _9, _4);
_7.2 = _1 ^ _6.2;
match _5 {
0 => bb1,
1 => bb7,
2 => bb5,
350336635 => bb9,
_ => bb4
}
}
bb9 = {
_1 = 2838042044_u32 as isize;
_21 = _10 as u64;
_6.1 = _17 as u8;
_17 = '\u{49c2}';
_7 = (_6.0, _6.1, _4);
_7.0 = [_18.0,_18.0,_18.0,_18.0];
RET = 152503011948825587550207767854374223963_i128;
_22 = _17;
_7.1 = !_6.1;
_12 = [_18.0,_18.0,_18.0];
_11 = RET as u64;
_11 = !_21;
_7.1 = _6.1;
_20 = (-57_i8) as i32;
_7.0 = [_18.0,_18.0,_18.0,_18.0];
_18.0 = 1_usize;
_6 = (_7.0, _9, _4);
_1 = _7.2;
_18.1 = Field::<i64>(Variant(_8, 1), 0) & Field::<i64>(Variant(_8, 1), 0);
_18.1 = 32776_u16 as i64;
_15 = [_10,_10,_10,_10,_10,_10];
_18.0 = _5 as usize;
RET = !144964681961717327976192235040008467527_i128;
RET = (-60_i8) as i128;
_8 = Adt46::Variant1 { fld0: _18.1 };
_3 = _6.2;
_23 = -(-29716_i16);
_7 = (_6.0, _9, _1);
_8 = Adt46::Variant1 { fld0: _18.1 };
_4 = RET as isize;
match _5 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb6,
5 => bb10,
6 => bb11,
350336635 => bb13,
_ => bb12
}
}
bb10 = {
_18 = (8850416756177778697_usize, (-7285056007925403566_i64));
_18.0 = 13141991539617674620_usize;
_7.1 = _9;
_6.1 = !_7.1;
_6 = (_7.0, _9, _1);
_12 = [_18.0,_18.0,_18.0];
_18 = (2_usize, 8482256755051614615_i64);
_7.2 = -_1;
_18.1 = (-7613936596542601163_i64);
_7.2 = -_6.2;
_10 = false & false;
_18.0 = 2_usize;
_18.1 = -4584018312800255163_i64;
RET = !17393071974970000873864482067032534402_i128;
_17 = '\u{8ad27}';
_7.2 = _6.2 - _3;
_13 = [_5,_5,_5,_5];
_10 = !true;
_20 = _5;
_11 = !8128877670982344685_u64;
place!(Field::<i64>(Variant(_8, 1), 0)) = _18.1 | _18.1;
_18.0 = 14604197306790752191_usize;
_7.1 = 7694_u16 as u8;
_15 = [_10,_10,_10,_10,_10,_10];
_18 = (1_usize, Field::<i64>(Variant(_8, 1), 0));
_17 = '\u{8b5c7}';
_6 = (_7.0, _9, _4);
_7.2 = _1 ^ _6.2;
match _5 {
0 => bb1,
1 => bb7,
2 => bb5,
350336635 => bb9,
_ => bb4
}
}
bb11 = {
_7.1 = !_9;
_7.2 = -_2;
Goto(bb3)
}
bb12 = {
_7.1 = Field::<i64>(Variant(_8, 1), 0) as u8;
_3 = 24675_u16 as isize;
_7.2 = _1;
match _5 {
0 => bb2,
350336635 => bb5,
_ => bb4
}
}
bb13 = {
_2 = -_7.2;
_22 = _17;
_1 = _23 as isize;
_1 = 47256_u16 as isize;
_6 = (_7.0, _7.1, _7.2);
_6.1 = _9 * _7.1;
_10 = _2 != _3;
place!(Field::<i64>(Variant(_8, 1), 0)) = _18.1;
_21 = _11 * _11;
_7.2 = _2;
match _5 {
0 => bb14,
350336635 => bb16,
_ => bb15
}
}
bb14 = {
_18 = (8850416756177778697_usize, (-7285056007925403566_i64));
_18.0 = 13141991539617674620_usize;
_7.1 = _9;
_6.1 = !_7.1;
_6 = (_7.0, _9, _1);
_12 = [_18.0,_18.0,_18.0];
_18 = (2_usize, 8482256755051614615_i64);
_7.2 = -_1;
_18.1 = (-7613936596542601163_i64);
_7.2 = -_6.2;
_10 = false & false;
_18.0 = 2_usize;
_18.1 = -4584018312800255163_i64;
RET = !17393071974970000873864482067032534402_i128;
_17 = '\u{8ad27}';
_7.2 = _6.2 - _3;
_13 = [_5,_5,_5,_5];
_10 = !true;
_20 = _5;
_11 = !8128877670982344685_u64;
place!(Field::<i64>(Variant(_8, 1), 0)) = _18.1 | _18.1;
_18.0 = 14604197306790752191_usize;
_7.1 = 7694_u16 as u8;
_15 = [_10,_10,_10,_10,_10,_10];
_18 = (1_usize, Field::<i64>(Variant(_8, 1), 0));
_17 = '\u{8b5c7}';
_6 = (_7.0, _9, _4);
_7.2 = _1 ^ _6.2;
match _5 {
0 => bb1,
1 => bb7,
2 => bb5,
350336635 => bb9,
_ => bb4
}
}
bb15 = {
_6 = (_7.0, _7.1, _2);
_11 = 15867948656248044064_u64;
SetDiscriminant(_8, 1);
_15 = [true,true,false,true,false,true];
_6.0 = _7.0;
_9 = _7.1;
_12 = [0_usize,6_usize,6_usize];
_4 = !_6.2;
_7.1 = _11 as u8;
_11 = !6990176408128598784_u64;
_3 = _1 ^ _4;
_11 = !16498447185038494267_u64;
_13 = [_5,_5,_5,_5];
match _5 {
0 => bb1,
1 => bb3,
2 => bb6,
350336635 => bb8,
_ => bb7
}
}
bb16 = {
_13 = [_5,_20,_5,_20];
_7 = _6;
RET = 161266399103543804499506677677568665975_i128;
_6.2 = _2;
_6.2 = _3;
RET = (-117_i8) as i128;
_26 = !128439941925679701129126212521352012870_u128;
_7 = _6;
_15 = [_10,_10,_10,_10,_10,_10];
RET = 112750617504625666509988281901118135575_i128;
_12 = [_18.0,_18.0,_18.0];
_18 = (2_usize, Field::<i64>(Variant(_8, 1), 0));
_12 = [_18.0,_18.0,_18.0];
_2 = _3;
_13 = [_5,_20,_20,_20];
_11 = !_21;
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(15_usize, 26_usize, Move(_26), 12_usize, Move(_12), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(15_usize, 6_usize, Move(_6), 10_usize, Move(_10), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(15_usize, 2_usize, Move(_2), 1_usize, Move(_1), 28_usize, _28, 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> u8 {
mir! {
type RET = u8;
let _7: char;
let _8: &'static [i64; 4];
let _9: [usize; 4];
let _10: ();
let _11: ();
{
RET = !229_u8;
_2 = !_5;
RET = 143_u8 * 20_u8;
_2 = _4 | _6;
_3 = _1 | _6;
_2 = 284801809183602858380031634505129616169_u128 as isize;
_1 = !_6;
RET = 4223569895302603277_i64 as u8;
_5 = 3141103563_u32 as isize;
_1 = -_6;
_6 = _3 | _4;
_3 = -_4;
_4 = (-98413506996262733104222663408550729683_i128) as isize;
_3 = _6 - _1;
_3 = 57025_u16 as isize;
RET = 7596111854500071348_u64 as u8;
_7 = '\u{e5bf5}';
Goto(bb1)
}
bb1 = {
_4 = _1;
RET = !169_u8;
_3 = _6;
Goto(bb2)
}
bb2 = {
RET = 41_u8;
_7 = '\u{6e382}';
_6 = _1;
_5 = -_3;
RET = 435788581_i32 as u8;
_4 = _5 * _1;
_4 = _5;
Call(_1 = fn17(_3, _4, _5, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = !84_u8;
RET = 68_u8;
RET = !143_u8;
_4 = _3;
_3 = _4 & _5;
_6 = _1 | _3;
_4 = _1;
_1 = !_4;
_7 = '\u{10de1b}';
_9 = [6895258467625462232_usize,1_usize,9817653700163947430_usize,11743612404921752213_usize];
RET = 43_u8 >> _1;
_4 = (-3735253947275486263_i64) as isize;
_6 = !_3;
_7 = '\u{2cc52}';
_3 = _5 + _6;
_5 = _6 | _3;
_9 = [0_usize,1_usize,1_usize,17421051774667915468_usize];
_4 = 1210579596_u32 as isize;
Goto(bb4)
}
bb4 = {
Call(_10 = dump_var(16_usize, 9_usize, Move(_9), 7_usize, Move(_7), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: [u16; 3];
let _6: u32;
let _7: ();
let _8: ();
{
_1 = _3;
_3 = _4 & _2;
_4 = (-2042654601_i32) as isize;
_3 = _2;
_2 = !_1;
RET = _2;
_5 = [49389_u16,17307_u16,48446_u16];
RET = _3 ^ _1;
RET = (-69755267787263326929404141873153012983_i128) as isize;
_2 = 977419101_i32 as isize;
RET = 126_u8 as isize;
_1 = _3;
RET = _3;
_4 = -RET;
_6 = 2409522914516698049_usize as u32;
_1 = _4;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(17_usize, 4_usize, Move(_4), 6_usize, Move(_6), 5_usize, Move(_5), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: i32,mut _2: i128,mut _3: [u8; 8],mut _4: u64,mut _5: i32,mut _6: i32) -> char {
mir! {
type RET = char;
let _7: isize;
let _8: [i16; 6];
let _9: Adt53;
let _10: char;
let _11: bool;
let _12: Adt56;
let _13: Adt48;
let _14: [i64; 4];
let _15: u32;
let _16: isize;
let _17: i16;
let _18: i128;
let _19: (u64, [bool; 6]);
let _20: i64;
let _21: Adt45;
let _22: [u16; 1];
let _23: u128;
let _24: bool;
let _25: i32;
let _26: [u16; 6];
let _27: [usize; 3];
let _28: bool;
let _29: isize;
let _30: [usize; 3];
let _31: ();
let _32: ();
{
RET = '\u{417a6}';
RET = '\u{b827}';
RET = '\u{5aab5}';
_1 = _4 as i32;
_3 = [182_u8,143_u8,250_u8,121_u8,156_u8,22_u8,180_u8,150_u8];
_1 = !_5;
_2 = 110_i8 as i128;
_3 = [236_u8,34_u8,106_u8,150_u8,75_u8,125_u8,197_u8,172_u8];
match _4 {
0 => bb1,
1 => bb2,
3049782717168126858 => bb4,
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
_7 = (-3_isize) + (-9223372036854775808_isize);
_10 = RET;
RET = _10;
_10 = RET;
_2 = (-22142147023432370957208871965542579028_i128);
_1 = 3835794021_u32 as i32;
Goto(bb5)
}
bb5 = {
_6 = !_5;
_11 = _7 < _7;
_8 = [23606_i16,(-9318_i16),(-11557_i16),13620_i16,(-28995_i16),(-10752_i16)];
_9.fld0 = _5 as i8;
_11 = !true;
_1 = _5 << _7;
_10 = RET;
_1 = _5;
_11 = true;
_6 = _1 << _2;
_10 = RET;
_11 = !false;
_8 = [2012_i16,3567_i16,(-10963_i16),(-21168_i16),(-23997_i16),28858_i16];
Goto(bb6)
}
bb6 = {
RET = _10;
_10 = RET;
_6 = 22528_u16 as i32;
_8 = [11423_i16,27900_i16,(-23291_i16),(-15355_i16),26_i16,(-724_i16)];
_4 = !16671663388533429549_u64;
_9 = Adt53 { fld0: 43_i8 };
_3 = [71_u8,83_u8,83_u8,39_u8,85_u8,79_u8,4_u8,42_u8];
RET = _10;
_6 = _1;
_10 = RET;
_6 = 14358_u16 as i32;
RET = _10;
_3 = [24_u8,198_u8,168_u8,151_u8,198_u8,186_u8,153_u8,36_u8];
_6 = _1;
_7 = (-62_isize);
_8 = [1253_i16,(-8697_i16),(-27282_i16),(-15414_i16),19703_i16,14603_i16];
_9 = Adt53 { fld0: (-3_i8) };
RET = _10;
_15 = 3785940170_u32;
_14 = [8170206630975722883_i64,(-4653854190428225778_i64),(-5090895623306004383_i64),8145243336930219063_i64];
_17 = (-308821489874045777_i64) as i16;
_2 = 151648420863216815856725920408667245895_i128;
_6 = !_1;
_17 = 2213_i16 << _1;
_16 = -_7;
_1 = _15 as i32;
match _7 {
0 => bb7,
340282366920938463463374607431768211394 => bb9,
_ => bb8
}
}
bb7 = {
Return()
}
bb8 = {
_7 = (-3_isize) + (-9223372036854775808_isize);
_10 = RET;
RET = _10;
_10 = RET;
_2 = (-22142147023432370957208871965542579028_i128);
_1 = 3835794021_u32 as i32;
Goto(bb5)
}
bb9 = {
_7 = -_16;
_3 = [105_u8,55_u8,140_u8,107_u8,207_u8,164_u8,72_u8,247_u8];
_3 = [250_u8,159_u8,92_u8,167_u8,107_u8,159_u8,221_u8,49_u8];
RET = _10;
_14 = [3122153905084313216_i64,6292774281960634608_i64,(-981102221541174193_i64),(-6238224937136022186_i64)];
_4 = 6819383452140550242_u64 | 8648681655935096777_u64;
_6 = _2 as i32;
_18 = _2 & _2;
_2 = RET as i128;
_11 = _5 == _6;
_1 = 216_u8 as i32;
_11 = _5 < _5;
_1 = _5 >> _5;
_3 = [172_u8,58_u8,55_u8,147_u8,9_u8,26_u8,154_u8,171_u8];
_10 = RET;
_9 = Adt53 { fld0: 71_i8 };
_3 = [188_u8,209_u8,176_u8,197_u8,26_u8,42_u8,92_u8,5_u8];
_6 = _2 as i32;
_8 = [_17,_17,_17,_17,_17,_17];
_7 = _9.fld0 as isize;
Goto(bb10)
}
bb10 = {
_6 = _5 * _1;
RET = _10;
_22 = [25643_u16];
_19.1 = [_11,_11,_11,_11,_11,_11];
_3 = [113_u8,69_u8,129_u8,35_u8,188_u8,117_u8,229_u8,16_u8];
_10 = RET;
_23 = !310864674414784593060424234664181824068_u128;
_10 = RET;
_19.0 = _11 as u64;
_15 = !3168481252_u32;
_18 = _2 ^ _2;
_1 = RET as i32;
match _9.fld0 {
0 => bb9,
71 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_25 = 38117_u16 as i32;
_11 = !true;
_26 = [11518_u16,54609_u16,61076_u16,43104_u16,64357_u16,38217_u16];
match _9.fld0 {
0 => bb7,
1 => bb2,
2 => bb6,
3 => bb9,
71 => bb13,
_ => bb5
}
}
bb13 = {
RET = _10;
_19.1 = [_11,_11,_11,_11,_11,_11];
_27 = [4571018277587244957_usize,9463326824081172613_usize,7_usize];
_5 = _6 | _6;
_24 = _11;
_28 = !_24;
_9.fld0 = -(-41_i8);
Goto(bb14)
}
bb14 = {
_19.0 = _15 as u64;
_7 = _9.fld0 as isize;
_2 = _18 - _18;
_27 = [5_usize,1_usize,6_usize];
_9 = Adt53 { fld0: (-49_i8) };
RET = _10;
_22 = [51686_u16];
_15 = 1635931347_u32 - 3289458693_u32;
_28 = _11;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(18_usize, 24_usize, Move(_24), 22_usize, Move(_22), 19_usize, Move(_19), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(18_usize, 14_usize, Move(_14), 28_usize, Move(_28), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(18_usize, 17_usize, Move(_17), 8_usize, Move(_8), 15_usize, Move(_15), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(2673301532_u32), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(173434566433667984809898191852845397236_u128), std::hint::black_box(10534190097486921564_u64), std::hint::black_box(177341733_i32), std::hint::black_box(2122452967951064868_i64), std::hint::black_box(40141913620916493900735644977075604374_i128));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: [i64; 2],
fld2: [bool; 6],
fld3: ([u64; 8],),
fld4: [u16; 6],

},
Variant1{
fld0: [u16; 3],
fld1: [u64; 8],
fld2: [bool; 6],

},
Variant2{
fld0: [u64; 8],
fld1: u64,
fld2: *mut i32,
fld3: [bool; 6],
fld4: u32,
fld5: [bool; 8],
fld6: ([i16; 6], i32, u16, i64, i16),
fld7: ([u64; 8], *const bool, ([usize; 4], u8, isize)),

},
Variant3{
fld0: [i128; 5],
fld1: [bool; 8],
fld2: u128,
fld3: *mut [i64; 4],
fld4: ([u64; 8],),
fld5: *const bool,
fld6: (u64, [bool; 6]),
fld7: ([usize; 4], u8, isize),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u16; 6],
fld1: [bool; 6],
fld2: f32,
fld3: [bool; 4],

},
Variant1{
fld0: ([i16; 6], i32, u16, i64, i16),
fld1: *mut i32,
fld2: [u64; 8],
fld3: u8,
fld4: [u16; 1],

},
Variant2{
fld0: usize,
fld1: *mut i32,
fld2: ([u64; 8], *const bool, ([usize; 4], u8, isize)),
fld3: ([u64; 8],),
fld4: [usize; 4],
fld5: i32,
fld6: i64,

},
Variant3{
fld0: bool,
fld1: [u64; 8],
fld2: *mut i32,
fld3: [i32; 4],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: [bool; 8],
fld1: [u16; 1],
fld2: isize,
fld3: u32,
fld4: [i64; 2],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (u64, [bool; 6]),

},
Variant1{
fld0: [i64; 4],
fld1: ([i16; 6], i32, u16, i64, i16),
fld2: [u16; 1],
fld3: [u16; 3],
fld4: Adt43,
fld5: i32,
fld6: u32,
fld7: i128,

},
Variant2{
fld0: [i128; 5],
fld1: char,
fld2: ([i16; 6], i32, u16, i64, i16),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: ([usize; 4], u8, isize),
fld1: [usize; 4],
fld2: isize,
fld3: u128,
fld4: i16,
fld5: u8,
fld6: f64,
fld7: [u8; 8],

},
Variant1{
fld0: i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt43,
fld1: char,
fld2: Adt44,
fld3: *mut [i64; 4],
fld4: [usize; 4],

},
Variant1{
fld0: [i16; 6],
fld1: [i64; 2],
fld2: u32,
fld3: [u16; 1],

},
Variant2{
fld0: [u8; 8],
fld1: u64,
fld2: u16,
fld3: [u64; 8],

},
Variant3{
fld0: (usize, i64),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const bool,
fld1: [u64; 8],
fld2: isize,
fld3: i8,
fld4: (usize, i64),
fld5: u32,

},
Variant1{
fld0: [bool; 6],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: i64,
fld1: (u64, [bool; 6]),
fld2: [i32; 4],
fld3: ([u64; 8], *const bool, ([usize; 4], u8, isize)),

},
Variant1{
fld0: Adt47,

},
Variant2{
fld0: ([u64; 8], *const bool, ([usize; 4], u8, isize)),
fld1: i64,

},
Variant3{
fld0: Adt44,
fld1: [i16; 6],
fld2: i128,
fld3: i8,
fld4: u32,
fld5: i32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: ([u64; 8], *const bool, ([usize; 4], u8, isize)),
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [i16; 6],
fld1: i8,
fld2: [u8; 8],

},
Variant1{
fld0: (usize, i64),
fld1: char,
fld2: [i64; 2],
fld3: *mut i32,

},
Variant2{
fld0: [u16; 3],
fld1: ([usize; 4], u8, isize),
fld2: *const bool,
fld3: usize,
fld4: Adt47,
fld5: i32,
fld6: Adt50,
fld7: Adt43,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [i32; 4],
fld1: Adt51,
fld2: [usize; 4],
fld3: i16,

},
Variant1{
fld0: Adt47,
fld1: [bool; 6],
fld2: Adt51,

},
Variant2{
fld0: u128,
fld1: Adt51,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: i8,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u16; 1],
fld1: *mut [i64; 4],
fld2: [usize; 4],
fld3: [i64; 4],
fld4: Adt47,
fld5: [u16; 3],
fld6: (usize, i64),

},
Variant1{
fld0: isize,
fld1: Adt52,

},
Variant2{
fld0: (usize, i64),
fld1: u64,
fld2: f64,
fld3: [usize; 3],
fld4: [u16; 1],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ([usize; 4], u8, isize),
fld1: u8,
fld2: [usize; 3],
fld3: usize,
fld4: Adt50,
fld5: Adt46,

},
Variant1{
fld0: [i32; 4],
fld1: [u8; 8],
fld2: [bool; 6],
fld3: Adt50,
fld4: f64,
fld5: u32,
fld6: i64,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (usize, i64),
fld1: Adt48,
fld2: [i128; 5],
fld3: Adt44,
fld4: i64,

},
Variant1{
fld0: Adt42,
fld1: u128,

},
Variant2{
fld0: *const bool,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [i128; 5],
fld1: [bool; 6],
fld2: Adt54,
fld3: i8,
fld4: Adt50,
fld5: [i16; 6],
fld6: *mut [i64; 4],

},
Variant1{
fld0: Adt44,
fld1: char,
fld2: [u8; 8],
fld3: Adt42,
fld4: u32,
fld5: [usize; 4],
fld6: f32,

},
Variant2{
fld0: Adt42,
fld1: u8,
fld2: ([u64; 8], *const bool, ([usize; 4], u8, isize)),
fld3: *mut i32,
fld4: [u64; 8],
fld5: u128,
fld6: i64,
fld7: [i32; 4],

},
Variant3{
fld0: [i32; 4],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [u16; 6],
fld1: [u8; 8],
fld2: f64,
fld3: ([u64; 8], *const bool, ([usize; 4], u8, isize)),

},
Variant1{
fld0: i128,

},
Variant2{
fld0: [i128; 5],
fld1: [bool; 8],
fld2: i64,

}}

