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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: u128,mut _4: u16,mut _5: i128,mut _6: i32) -> isize {
mir! {
type RET = isize;
let _7: bool;
let _8: [i64; 8];
let _9: f32;
let _10: usize;
let _11: i32;
let _12: isize;
let _13: f32;
let _14: isize;
let _15: i32;
let _16: usize;
let _17: [i32; 8];
let _18: *const i8;
let _19: Adt41;
let _20: bool;
let _21: char;
let _22: isize;
let _23: usize;
let _24: i16;
let _25: Adt41;
let _26: [i32; 3];
let _27: f64;
let _28: *mut *const &'static u32;
let _29: *mut f32;
let _30: char;
let _31: &'static [char; 5];
let _32: Adt72;
let _33: &'static &'static *const i8;
let _34: i64;
let _35: isize;
let _36: f32;
let _37: &'static &'static u32;
let _38: &'static [char; 5];
let _39: [u8; 8];
let _40: &'static *const i8;
let _41: &'static &'static *const i8;
let _42: *const &'static f32;
let _43: i16;
let _44: i64;
let _45: i64;
let _46: f64;
let _47: f64;
let _48: ();
let _49: ();
{
RET = (-9223372036854775808_isize);
_4 = 51396_u16 ^ 24760_u16;
_5 = -(-39969788571806972717076043133283694040_i128);
_7 = true;
_5 = !137187055217371521420269782868376075966_i128;
RET = (-9223372036854775808_isize);
_6 = 612008789_i32 << _4;
_7 = !false;
_8 = [8094458228952209220_i64,(-4753716250933796770_i64),(-2217627811895764759_i64),6739791316838317071_i64,990932458180168461_i64,2159130644136048447_i64,1747559329155963429_i64,1740249547856755441_i64];
_7 = !false;
_8 = [3121478844190819871_i64,3335928735951752911_i64,5760560879407586289_i64,(-8250174484441326272_i64),3547041133253959512_i64,(-3195997222705967421_i64),6104882350288492803_i64,355933799309882761_i64];
RET = -(-9223372036854775808_isize);
_3 = 255462756004765876164744522403265169766_u128;
_4 = RET as u16;
RET = 9223372036854775807_isize;
_9 = _6 as f32;
_4 = 58893_u16 & 614_u16;
_13 = -_9;
_2 = 127_u8 >> RET;
_9 = 2745424951229448138_i64 as f32;
_3 = !188829737868646620688618648374316720457_u128;
Goto(bb1)
}
bb1 = {
_4 = 9902009863135418709_u64 as u16;
_11 = (-5673286239233836452_i64) as i32;
_16 = 5_usize << _4;
_10 = _16;
_6 = -_11;
_14 = RET ^ RET;
RET = _9 as isize;
_9 = _13 - _13;
_16 = !_10;
_22 = _14 ^ RET;
RET = !_22;
_17 = [_11,_11,_6,_6,_6,_11,_6,_6];
_21 = '\u{51a6d}';
_21 = '\u{a9023}';
Goto(bb2)
}
bb2 = {
Call(_7 = fn1(_17, _9, _14, _22, _10, _8, _22, _13, _21, _14, _8, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _13;
_1 = _7;
RET = !_14;
Goto(bb4)
}
bb4 = {
_11 = _6;
_7 = _1;
_13 = -_9;
_20 = RET > RET;
RET = -_22;
RET = !_14;
_23 = !_10;
_3 = 264694938066280376172760818700289573651_u128;
_23 = _10 << _14;
_9 = -_13;
_6 = _11;
_3 = 263158555687385589855043204962307812659_u128 | 292719229962480606774733606265093913603_u128;
_24 = 3615_i16;
_13 = _9 - _9;
RET = _14;
_11 = !_6;
_14 = !RET;
_23 = !_10;
Call(_5 = core::intrinsics::bswap((-19153087948164980734915721681511285916_i128)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17 = [_11,_6,_11,_6,_11,_6,_11,_6];
_27 = 6_i8 as f64;
_2 = !222_u8;
_10 = !_23;
_26 = [_6,_11,_11];
_12 = (-6877660093403402686_i64) as isize;
_2 = 69_u8 + 199_u8;
RET = _22;
_16 = _24 as usize;
_15 = _27 as i32;
_21 = '\u{10fffa}';
_11 = !_15;
_10 = _23;
Goto(bb6)
}
bb6 = {
_14 = -_22;
_14 = !RET;
_1 = _7;
_21 = '\u{26796}';
Goto(bb7)
}
bb7 = {
_30 = _21;
_16 = _24 as usize;
_3 = 261091310641764855038200719669701691429_u128 << _4;
RET = _14 | _22;
_3 = 5062919118484396241159122932491503598_u128 - 29735847889066459915356098235713027637_u128;
_21 = _30;
RET = _22;
_21 = _30;
_14 = 341097120_u32 as isize;
_6 = _15;
_16 = _23 >> _22;
_29 = core::ptr::addr_of_mut!(_13);
_29 = core::ptr::addr_of_mut!(_9);
_9 = _13;
_8 = [1252541049642349703_i64,1101026373659050555_i64,(-7660896151034280674_i64),(-927527527302035729_i64),(-5451449898684695019_i64),1756320403911547503_i64,(-6357791679911742787_i64),7376740994219507477_i64];
_29 = core::ptr::addr_of_mut!((*_29));
_6 = _15;
_6 = -_15;
_5 = !(-68689667189009294689645288334130093831_i128);
match _24 {
0 => bb8,
1 => bb9,
3615 => bb11,
_ => bb10
}
}
bb8 = {
Call(_7 = fn1(_17, _9, _14, _22, _10, _8, _22, _13, _21, _14, _8, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_17 = [_11,_6,_11,_6,_11,_6,_11,_6];
_27 = 6_i8 as f64;
_2 = !222_u8;
_10 = !_23;
_26 = [_6,_11,_11];
_12 = (-6877660093403402686_i64) as isize;
_2 = 69_u8 + 199_u8;
RET = _22;
_16 = _24 as usize;
_15 = _27 as i32;
_21 = '\u{10fffa}';
_11 = !_15;
_10 = _23;
Goto(bb6)
}
bb10 = {
_11 = _6;
_7 = _1;
_13 = -_9;
_20 = RET > RET;
RET = -_22;
RET = !_14;
_23 = !_10;
_3 = 264694938066280376172760818700289573651_u128;
_23 = _10 << _14;
_9 = -_13;
_6 = _11;
_3 = 263158555687385589855043204962307812659_u128 | 292719229962480606774733606265093913603_u128;
_24 = 3615_i16;
_13 = _9 - _9;
RET = _14;
_11 = !_6;
_14 = !RET;
_23 = !_10;
Call(_5 = core::intrinsics::bswap((-19153087948164980734915721681511285916_i128)), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
(*_29) = _13 * _13;
_13 = (*_29) * (*_29);
_2 = 169_u8 << _14;
_12 = -_22;
_26 = [_11,_15,_15];
Call((*_29) = core::intrinsics::transmute(_30), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_36 = _13 - (*_29);
_16 = !_23;
RET = _12;
(*_29) = _10 as f32;
_13 = _36;
RET = !_12;
_3 = _15 as u128;
_10 = _16 << _12;
_40 = &_18;
_34 = 5654929491214758778_i64;
_22 = -_12;
_41 = &_40;
_34 = _4 as i64;
(*_29) = _13;
_16 = _10;
_20 = !_1;
Goto(bb13)
}
bb13 = {
_10 = 4076672707_u32 as usize;
(*_29) = _3 as f32;
_27 = _15 as f64;
_9 = _36 * _13;
_22 = !_14;
_23 = _16 - _16;
(*_29) = -_13;
_1 = _23 != _16;
Goto(bb14)
}
bb14 = {
_45 = _34 & _34;
_23 = _10;
(*_29) = 2366624599904151674_u64 as f32;
_43 = _7 as i16;
_34 = -_45;
_21 = _30;
_40 = &_18;
_14 = RET;
_15 = _11 ^ _11;
_3 = 175658043130972396661204468440364691167_u128;
_17 = [_11,_11,_11,_11,_15,_11,_6,_15];
_40 = &_18;
_6 = !_15;
_26 = [_6,_15,_15];
_35 = _3 as isize;
_30 = _21;
_35 = 2689485439_u32 as isize;
_47 = -_27;
_16 = !_23;
_11 = _6 | _6;
_40 = &(*_40);
_30 = _21;
_17 = [_11,_11,_15,_15,_11,_11,_11,_15];
_9 = _13;
_9 = _11 as f32;
_39 = [_2,_2,_2,_2,_2,_2,_2,_2];
_4 = !37469_u16;
_12 = _14;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(0_usize, 6_usize, Move(_6), 21_usize, Move(_21), 39_usize, Move(_39), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(0_usize, 26_usize, Move(_26), 16_usize, Move(_16), 5_usize, Move(_5), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(0_usize, 7_usize, Move(_7), 8_usize, Move(_8), 43_usize, Move(_43), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(0_usize, 20_usize, Move(_20), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i32; 8],mut _2: f32,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: [i64; 8],mut _7: isize,mut _8: f32,mut _9: char,mut _10: isize,mut _11: [i64; 8],mut _12: usize) -> bool {
mir! {
type RET = bool;
let _13: [i128; 1];
let _14: &'static &'static i8;
let _15: f32;
let _16: f32;
let _17: u8;
let _18: ((char, *const i8),);
let _19: [bool; 6];
let _20: bool;
let _21: isize;
let _22: *mut f32;
let _23: i64;
let _24: isize;
let _25: f32;
let _26: [char; 5];
let _27: ();
let _28: ();
{
RET = true & true;
_1 = [1293635658_i32,(-1456108928_i32),(-391251144_i32),116033653_i32,(-1884820783_i32),672381959_i32,(-1573902368_i32),(-203473892_i32)];
_10 = _4;
_12 = _5;
_10 = -_3;
Goto(bb1)
}
bb1 = {
_11 = [9095200134913419421_i64,4299610883861858582_i64,8610841982215428077_i64,(-8085583911968264447_i64),(-7998922944035944218_i64),(-7995170308033421551_i64),1063354169879781876_i64,3638325673550486088_i64];
_3 = _10 + _10;
_9 = '\u{54d18}';
RET = false;
_2 = _8;
_7 = (-92960123236434742460852480933573539143_i128) as isize;
_3 = 64483_u16 as isize;
_10 = 132_u8 as isize;
RET = false ^ false;
_9 = '\u{8279e}';
_6 = _11;
_12 = _5;
_4 = _10 & _7;
_10 = _9 as isize;
_8 = -_2;
RET = !true;
Goto(bb2)
}
bb2 = {
_8 = _5 as f32;
_3 = _7;
_8 = _2 * _2;
_10 = 846699895_u32 as isize;
_3 = RET as isize;
_1 = [(-626234071_i32),(-1537747854_i32),(-1641545254_i32),(-646063402_i32),899866957_i32,(-404119945_i32),(-1036494055_i32),2059579823_i32];
RET = true;
RET = _4 > _10;
_9 = '\u{34102}';
_5 = 42046858817925404949099690268528493023_i128 as usize;
_12 = _5 ^ _5;
_8 = -_2;
RET = !true;
_13 = [(-4414630609010614998819830290661357397_i128)];
_10 = (-4129329143968199244_i64) as isize;
_10 = _4;
_13 = [160955428815735485747783876173628026157_i128];
_13 = [139072390477547506985102223101584351509_i128];
_6 = [8054428445743404755_i64,(-7992237493935235196_i64),(-299135579525471339_i64),1941638022621745784_i64,3936679978455331551_i64,5150985595407106600_i64,(-2155933896749531912_i64),6701789311509232937_i64];
_7 = _4;
_7 = _4 ^ _3;
_11 = _6;
_2 = -_8;
_1 = [972005998_i32,1618414525_i32,1537773241_i32,1113991665_i32,1997046023_i32,(-704728783_i32),(-1212136916_i32),(-846187010_i32)];
_9 = '\u{57295}';
_13 = [166059215562385592102219453051785283124_i128];
_5 = !_12;
Goto(bb3)
}
bb3 = {
_3 = _4;
_4 = _5 as isize;
RET = true;
RET = !true;
_3 = -_7;
_7 = -_3;
RET = true;
_6 = [5359220389029010775_i64,920087381731461789_i64,(-4717821502489993099_i64),8211198301500889300_i64,(-6481769279352452514_i64),(-7638650349361668473_i64),2231567719877403908_i64,765690701285015024_i64];
_3 = _9 as isize;
RET = !true;
_5 = !_12;
RET = true | true;
_5 = _12;
Call(_5 = fn2(_4, _6, _10, _6, _4, _11, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17 = 40_u8;
_2 = 42_i8 as f32;
_2 = _8;
_3 = _4 * _7;
_9 = '\u{38c9f}';
RET = !true;
Goto(bb5)
}
bb5 = {
_15 = _8;
_9 = '\u{3a695}';
_20 = !RET;
_3 = (-30250_i16) as isize;
_7 = _3 << _5;
_3 = _10 + _7;
RET = _20 ^ _20;
_20 = _7 > _7;
_18.0.0 = _9;
_8 = _2 + _2;
_8 = -_2;
_4 = _3 ^ _3;
_10 = _4 << _3;
_19 = [_20,_20,_20,_20,_20,_20];
_5 = _12 ^ _12;
_13 = [115257733476116098550579437374694399407_i128];
_16 = _2 + _2;
_19 = [_20,_20,_20,_20,_20,_20];
_19 = [_20,_20,_20,_20,_20,_20];
match _17 {
0 => bb6,
1 => bb7,
40 => bb9,
_ => bb8
}
}
bb6 = {
_17 = 40_u8;
_2 = 42_i8 as f32;
_2 = _8;
_3 = _4 * _7;
_9 = '\u{38c9f}';
RET = !true;
Goto(bb5)
}
bb7 = {
_3 = _4;
_4 = _5 as isize;
RET = true;
RET = !true;
_3 = -_7;
_7 = -_3;
RET = true;
_6 = [5359220389029010775_i64,920087381731461789_i64,(-4717821502489993099_i64),8211198301500889300_i64,(-6481769279352452514_i64),(-7638650349361668473_i64),2231567719877403908_i64,765690701285015024_i64];
_3 = _9 as isize;
RET = !true;
_5 = !_12;
RET = true | true;
_5 = _12;
Call(_5 = fn2(_4, _6, _10, _6, _4, _11, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_8 = _5 as f32;
_3 = _7;
_8 = _2 * _2;
_10 = 846699895_u32 as isize;
_3 = RET as isize;
_1 = [(-626234071_i32),(-1537747854_i32),(-1641545254_i32),(-646063402_i32),899866957_i32,(-404119945_i32),(-1036494055_i32),2059579823_i32];
RET = true;
RET = _4 > _10;
_9 = '\u{34102}';
_5 = 42046858817925404949099690268528493023_i128 as usize;
_12 = _5 ^ _5;
_8 = -_2;
RET = !true;
_13 = [(-4414630609010614998819830290661357397_i128)];
_10 = (-4129329143968199244_i64) as isize;
_10 = _4;
_13 = [160955428815735485747783876173628026157_i128];
_13 = [139072390477547506985102223101584351509_i128];
_6 = [8054428445743404755_i64,(-7992237493935235196_i64),(-299135579525471339_i64),1941638022621745784_i64,3936679978455331551_i64,5150985595407106600_i64,(-2155933896749531912_i64),6701789311509232937_i64];
_7 = _4;
_7 = _4 ^ _3;
_11 = _6;
_2 = -_8;
_1 = [972005998_i32,1618414525_i32,1537773241_i32,1113991665_i32,1997046023_i32,(-704728783_i32),(-1212136916_i32),(-846187010_i32)];
_9 = '\u{57295}';
_13 = [166059215562385592102219453051785283124_i128];
_5 = !_12;
Goto(bb3)
}
bb9 = {
_10 = 11457024483427101611_u64 as isize;
_5 = !_12;
_5 = 76636810_i32 as usize;
_1 = [2058614685_i32,1567935149_i32,(-935194095_i32),(-233136272_i32),(-493291257_i32),1347341212_i32,2050745069_i32,(-430316425_i32)];
_18.0.0 = _9;
_2 = -_15;
_1 = [648355450_i32,(-1631385621_i32),(-1559281003_i32),(-955922038_i32),(-1507048096_i32),(-925175283_i32),1607374546_i32,1516787395_i32];
Goto(bb10)
}
bb10 = {
RET = !_20;
_19 = [_20,RET,_20,_20,RET,_20];
_7 = _3;
RET = !_20;
RET = !_20;
_23 = 13976_u16 as i64;
_21 = _3;
_23 = 1772357000753747835_i64 * (-1404579947870738255_i64);
_18.0.0 = _9;
_6 = _11;
_1 = [(-1092744170_i32),1570231667_i32,(-1498475677_i32),(-1484418844_i32),(-1907047834_i32),(-1410818705_i32),1210395900_i32,(-427181728_i32)];
_4 = _21 | _7;
_16 = _3 as f32;
_7 = _4;
_22 = core::ptr::addr_of_mut!(_16);
_3 = _7;
_10 = _7;
_2 = (*_22) * _16;
_19 = [_20,_20,_20,RET,_20,RET];
_4 = 63260839622823243268352875810277141552_u128 as isize;
_1 = [470440748_i32,733863835_i32,(-1318066376_i32),(-797152287_i32),2015590725_i32,1411624267_i32,(-240365853_i32),(-749314755_i32)];
_26 = [_9,_18.0.0,_18.0.0,_9,_18.0.0];
_23 = (-8602424969525626288_i64);
_17 = 4_u8;
_21 = -_3;
_8 = -_2;
_16 = _17 as f32;
_18.0.0 = _9;
Goto(bb11)
}
bb11 = {
Call(_27 = dump_var(1_usize, 21_usize, Move(_21), 9_usize, Move(_9), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_27 = dump_var(1_usize, 5_usize, Move(_5), 1_usize, Move(_1), 19_usize, Move(_19), 23_usize, Move(_23)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: [i64; 8],mut _3: isize,mut _4: [i64; 8],mut _5: isize,mut _6: [i64; 8],mut _7: [i64; 8]) -> usize {
mir! {
type RET = usize;
let _8: [i128; 1];
let _9: u8;
let _10: [u64; 2];
let _11: f32;
let _12: char;
let _13: char;
let _14: f64;
let _15: isize;
let _16: ();
let _17: ();
{
_3 = 41970116454441878120773036099773817858_i128 as isize;
RET = !0_usize;
_6 = [(-9062161507179567492_i64),6377290714732703909_i64,1896534746797902227_i64,(-761832932260374911_i64),(-8061025727306826939_i64),3965052991546410031_i64,5452219693620352707_i64,2017882946804164853_i64];
_6 = [(-6262678836210043488_i64),2808088840685765873_i64,(-1951318345498385353_i64),7964576716541916681_i64,1527020018795598153_i64,(-8741096319512265480_i64),(-3035668179923134400_i64),(-853963008603600068_i64)];
_3 = !_1;
RET = !7_usize;
_8 = [(-118590958317720357032065321441811321273_i128)];
_4 = _7;
_9 = 57_u8;
_2 = [7162183277652482908_i64,(-3448607449958833580_i64),(-2744048634910610707_i64),2088474386731104105_i64,104967884432915273_i64,3733935029731855073_i64,8915935854199303086_i64,8669518268540349857_i64];
RET = true as usize;
_5 = -_1;
_10 = [5312879610048530754_u64,14485187155598826681_u64];
_5 = _1;
_2 = _4;
_9 = true as u8;
_10 = [4133519285908318317_u64,12615582932451173275_u64];
RET = 14198307206891319793_usize;
_7 = [7540913826034704909_i64,641351633401832774_i64,(-3701772195522132956_i64),(-4752141314108661380_i64),(-3729219191771769888_i64),973251768997265723_i64,7825874289308507582_i64,2989116967972551375_i64];
_1 = -_5;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
14198307206891319793 => bb9,
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
_5 = !_3;
_1 = _3;
_6 = [2088527858592662222_i64,(-2409894798937377301_i64),2869216038585987839_i64,833688810451878177_i64,(-8843869724249978849_i64),9036759898496931512_i64,(-5743334930729750612_i64),(-2551125423266828063_i64)];
Call(_6 = core::intrinsics::transmute(_7), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = _1 << _1;
_11 = (-831973199697564740_i64) as f32;
_5 = _3;
_4 = [(-6232597766424961041_i64),2660501085704707987_i64,4147668130174441570_i64,3133269852600507514_i64,3360726849877483132_i64,(-3074385246422007104_i64),(-7966218786108479354_i64),1713079977553866910_i64];
RET = 15760793412898894118_usize;
_4 = [1805647731651972293_i64,2072235469222136480_i64,(-4359539390609931660_i64),(-9027217054677841311_i64),(-518751353905637219_i64),1840284965219847287_i64,(-8685008017702919044_i64),6675756334696780896_i64];
RET = !12483856985137492003_usize;
_9 = !203_u8;
_4 = [357915800537812447_i64,(-7251139708171123143_i64),4097026333391689632_i64,(-402157540699867645_i64),(-5324594894354749844_i64),6552152533476675995_i64,6287772272391902194_i64,(-8368586068478783914_i64)];
_11 = 22225_u16 as f32;
_11 = 336722442808623782247362401085469284039_u128 as f32;
_12 = '\u{afed7}';
_8 = [165021144779616451248606932738121114018_i128];
_1 = -_5;
_7 = _2;
_1 = _3 - _5;
_9 = !229_u8;
_7 = _6;
Call(_11 = fn3(_7, _2, _7, _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = [(-9161398562148142139_i64),3272518496582484897_i64,(-86448417905632553_i64),4104586550817275587_i64,(-3009089482611909487_i64),345202930981673801_i64,(-8546767388153460822_i64),(-4231567934743602535_i64)];
_6 = [(-5301692415243665296_i64),454283229769256070_i64,1451560877291394942_i64,(-2311731005083814994_i64),8002862627834278876_i64,(-7967332288598924525_i64),(-6930051587996779495_i64),641433632932891231_i64];
Goto(bb12)
}
bb12 = {
_4 = [(-3036406775084195599_i64),(-6887577340147391111_i64),(-2472979620964152498_i64),(-8105959197475331622_i64),(-3843041771489295433_i64),8633959819086798709_i64,(-1065517174208840127_i64),7199253459748091971_i64];
Call(_2 = fn16(_4, _1, _3, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_8 = [42622938703673755379687039547881096693_i128];
_14 = RET as f64;
_3 = _1 * _1;
_13 = _12;
_5 = _1 | _1;
_1 = _5 + _5;
_8 = [15121384025855450453192533925505920274_i128];
Goto(bb14)
}
bb14 = {
_5 = 62889027685684952282508642900208470380_u128 as isize;
_4 = [(-81175110583085979_i64),640893584256634276_i64,7194753854410657583_i64,3202364034876192085_i64,(-7356755863519137656_i64),(-581620135797232033_i64),(-887689146028920612_i64),(-3908254089365808255_i64)];
_12 = _13;
_2 = _7;
_8 = [141819900246592245011375748309614960106_i128];
_5 = !_1;
_14 = (-1717580920698552603_i64) as f64;
RET = 6484109896243046816_usize << _5;
_6 = [(-7523712375235272596_i64),(-1871302203116661995_i64),2270402447623445188_i64,6549009858749665795_i64,(-502621723109404801_i64),9102572132261542379_i64,(-8975374095301113934_i64),(-4981026674870765022_i64)];
_14 = 1353876088_i32 as f64;
_11 = 60765_u16 as f32;
_15 = 189206020_i32 as isize;
_13 = _12;
_2 = [(-7058864467412494983_i64),(-8078283284370034791_i64),7953080519183471492_i64,(-3334857021275384694_i64),(-789474328387420472_i64),5541884859805687910_i64,(-3657572949280269702_i64),(-5919869101646619712_i64)];
_8 = [(-138639204940380516253772746206842220577_i128)];
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(2_usize, 3_usize, Move(_3), 9_usize, Move(_9), 7_usize, Move(_7), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_16 = dump_var(2_usize, 15_usize, Move(_15), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i64; 8],mut _2: [i64; 8],mut _3: [i64; 8],mut _4: [i64; 8]) -> f32 {
mir! {
type RET = f32;
let _5: Adt69;
let _6: isize;
let _7: i64;
let _8: &'static u32;
let _9: f64;
let _10: [char; 5];
let _11: [u32; 3];
let _12: usize;
let _13: i128;
let _14: f64;
let _15: u64;
let _16: u16;
let _17: (u8, [u128; 6], [u128; 6], *mut *const &'static u32);
let _18: u64;
let _19: f64;
let _20: bool;
let _21: [u64; 2];
let _22: usize;
let _23: isize;
let _24: [u16; 6];
let _25: u8;
let _26: ();
let _27: ();
{
_4 = [(-4613192374687572772_i64),8101445588134266836_i64,(-2837085433090679102_i64),4105260212251694336_i64,6271778133216777754_i64,(-3903378559582822531_i64),2571456531832377295_i64,(-5732282631410315875_i64)];
_2 = _1;
_5.fld1 = 2_usize;
_5.fld2 = 1854754360_u32 as i64;
_5.fld2 = -(-4190693383744038064_i64);
_2 = [_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2];
_1 = _4;
_5.fld3 = [(-1440750115_i32),(-1807033169_i32),35108019_i32];
RET = 16129_i16 as f32;
_5.fld0 = 16362128092903981341_u64;
_5.fld3 = [(-394721102_i32),(-1582619903_i32),(-966082722_i32)];
_6 = 1199_i16 as isize;
_5.fld0 = 2154473424056300181_u64 * 3650069694211028916_u64;
_5.fld1 = !11088182852727808989_usize;
_2 = [_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2];
_5.fld1 = !5_usize;
_6 = !9223372036854775807_isize;
_4 = _3;
RET = 280903229378688561332744443036554882271_u128 as f32;
_7 = _5.fld2 << _6;
_5.fld1 = 10206184240405814962_usize;
_4 = [_7,_5.fld2,_5.fld2,_5.fld2,_7,_7,_7,_7];
RET = _5.fld0 as f32;
_6 = (-44_isize);
_5.fld3 = [(-157334844_i32),37278187_i32,(-1303523673_i32)];
_5.fld0 = _6 as u64;
Goto(bb1)
}
bb1 = {
_9 = _5.fld0 as f64;
_5.fld1 = 0_usize;
_1 = [_7,_7,_7,_5.fld2,_7,_5.fld2,_7,_7];
RET = _5.fld0 as f32;
RET = 337553815020655971023622526350657069122_u128 as f32;
RET = _5.fld1 as f32;
match _6 {
0 => bb2,
1 => bb3,
340282366920938463463374607431768211412 => bb5,
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
_5.fld0 = (-578570106_i32) as u64;
_2 = [_5.fld2,_7,_7,_7,_7,_7,_7,_5.fld2];
_4 = [_5.fld2,_5.fld2,_7,_5.fld2,_5.fld2,_7,_5.fld2,_7];
_6 = 65_isize;
_6 = 34387796492764257025015760799862085173_u128 as isize;
_5.fld2 = _7;
RET = 81_i8 as f32;
_10 = ['\u{27d03}','\u{7e1aa}','\u{5bca2}','\u{67baf}','\u{c5e9e}'];
Goto(bb6)
}
bb6 = {
RET = (-1038867149_i32) as f32;
_5.fld2 = !_7;
RET = _9 as f32;
_11 = [849682062_u32,1634351107_u32,1773275980_u32];
_4 = [_5.fld2,_7,_7,_7,_7,_5.fld2,_7,_5.fld2];
_5.fld2 = 2638898799_u32 as i64;
_12 = _5.fld1;
RET = _12 as f32;
_5.fld2 = !_2[_12];
_1[_12] = _5.fld2 & _3[_12];
_9 = _5.fld3[_12] as f64;
_2 = [_7,_1[_12],_4[_12],_7,_1[_12],_3[_12],_1[_12],_1[_12]];
_5.fld2 = 15749170987377011277462913963868796751_u128 as i64;
_5.fld0 = 9162332646759174588_u64;
_6 = 9223372036854775807_isize;
_9 = _5.fld3[_12] as f64;
_8 = &_11[_12];
_5.fld2 = _3[_12];
_10 = ['\u{2c3e2}','\u{2ed92}','\u{c7bb1}','\u{cca50}','\u{256a4}'];
_10[_12] = '\u{b27a8}';
_5.fld3[_12] = 273286540_i32 & (-398597157_i32);
_4 = [_5.fld2,_5.fld2,_3[_12],_2[_12],_1[_12],_1[_12],_2[_12],_1[_12]];
_10 = ['\u{24a8d}','\u{f4ff3}','\u{4eb5}','\u{86a0d}','\u{8ac75}'];
_15 = _10[_12] as u64;
_12 = _5.fld1;
match _12 {
1 => bb2,
2 => bb3,
3 => bb4,
0 => bb7,
_ => bb5
}
}
bb7 = {
_5.fld0 = RET as u64;
_16 = _5.fld1 as u16;
_17.1 = [294986677208142243867910592711866966426_u128,227075237173700010582782769181232616370_u128,97667523411828307565531696849777149853_u128,299268400379944104790042810530868116704_u128,243777012672635143540149684655887154619_u128,212959017135725970567292073798536834989_u128];
_3[_12] = !_1[_12];
_5.fld0 = _15;
_5.fld1 = _12;
_1[_12] = _7;
_5.fld3[_12] = 752948608_i32;
_3 = [_7,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_5.fld2,_4[_12]];
_12 = !_5.fld1;
match (*_8) {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb9,
6 => bb10,
849682062 => bb12,
_ => bb11
}
}
bb8 = {
_9 = _5.fld0 as f64;
_5.fld1 = 0_usize;
_1 = [_7,_7,_7,_5.fld2,_7,_5.fld2,_7,_7];
RET = _5.fld0 as f32;
RET = 337553815020655971023622526350657069122_u128 as f32;
RET = _5.fld1 as f32;
match _6 {
0 => bb2,
1 => bb3,
340282366920938463463374607431768211412 => bb5,
_ => bb4
}
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
_11 = [3049746474_u32,1667321419_u32,2424420309_u32];
RET = _12 as f32;
_17.2 = [265185630948209854426674220205394732216_u128,191000769140677049400551075639379942511_u128,260763146483925106953065793796874578368_u128,123033194391940232697903841369912727591_u128,268587963542733241086675871519863179325_u128,313398349509149611840412026977269754481_u128];
_5.fld3 = [1387191181_i32,1008191294_i32,314399857_i32];
_7 = _5.fld2;
_19 = 8794_i16 as f64;
_10 = ['\u{c850e}','\u{45616}','\u{48155}','\u{1a482}','\u{917df}'];
RET = 3279789436_u32 as f32;
_7 = _5.fld2;
_5.fld3 = [(-1050820572_i32),(-162972809_i32),(-66926536_i32)];
_18 = _5.fld0;
_5.fld1 = !_12;
_21 = [_15,_5.fld0];
_15 = 144662776_u32 as u64;
_4 = _2;
_17.1 = _17.2;
Call(_13 = fn4(_18, _17.1, _5.fld3, _17.1, _16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = _5.fld1 as f32;
_20 = !true;
_14 = -_9;
_14 = _9;
_17.1 = _17.2;
RET = _16 as f32;
_21 = [_15,_5.fld0];
_22 = RET as usize;
_1 = _2;
RET = _13 as f32;
_1 = [_5.fld2,_5.fld2,_7,_5.fld2,_7,_5.fld2,_7,_7];
match _6 {
9223372036854775807 => bb15,
_ => bb14
}
}
bb14 = {
RET = (-1038867149_i32) as f32;
_5.fld2 = !_7;
RET = _9 as f32;
_11 = [849682062_u32,1634351107_u32,1773275980_u32];
_4 = [_5.fld2,_7,_7,_7,_7,_5.fld2,_7,_5.fld2];
_5.fld2 = 2638898799_u32 as i64;
_12 = _5.fld1;
RET = _12 as f32;
_5.fld2 = !_2[_12];
_1[_12] = _5.fld2 & _3[_12];
_9 = _5.fld3[_12] as f64;
_2 = [_7,_1[_12],_4[_12],_7,_1[_12],_3[_12],_1[_12],_1[_12]];
_5.fld2 = 15749170987377011277462913963868796751_u128 as i64;
_5.fld0 = 9162332646759174588_u64;
_6 = 9223372036854775807_isize;
_9 = _5.fld3[_12] as f64;
_8 = &_11[_12];
_5.fld2 = _3[_12];
_10 = ['\u{2c3e2}','\u{2ed92}','\u{c7bb1}','\u{cca50}','\u{256a4}'];
_10[_12] = '\u{b27a8}';
_5.fld3[_12] = 273286540_i32 & (-398597157_i32);
_4 = [_5.fld2,_5.fld2,_3[_12],_2[_12],_1[_12],_1[_12],_2[_12],_1[_12]];
_10 = ['\u{24a8d}','\u{f4ff3}','\u{4eb5}','\u{86a0d}','\u{8ac75}'];
_15 = _10[_12] as u64;
_12 = _5.fld1;
match _12 {
1 => bb2,
2 => bb3,
3 => bb4,
0 => bb7,
_ => bb5
}
}
bb15 = {
_16 = 54101_u16;
_17.0 = 163231507108890192448503007189210485734_u128 as u8;
_25 = _17.0;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(3_usize, 4_usize, Move(_4), 18_usize, Move(_18), 7_usize, Move(_7), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(3_usize, 25_usize, Move(_25), 12_usize, Move(_12), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u64,mut _2: [u128; 6],mut _3: [i32; 3],mut _4: [u128; 6],mut _5: u16) -> i128 {
mir! {
type RET = i128;
let _6: *const f64;
let _7: [u8; 8];
let _8: u8;
let _9: u64;
let _10: [u8; 8];
let _11: [i8; 2];
let _12: i64;
let _13: u128;
let _14: isize;
let _15: *mut i64;
let _16: char;
let _17: *mut usize;
let _18: [i8; 6];
let _19: &'static &'static &'static i8;
let _20: isize;
let _21: char;
let _22: f32;
let _23: [u64; 2];
let _24: *const *mut u128;
let _25: char;
let _26: &'static [u16; 5];
let _27: bool;
let _28: ();
let _29: ();
{
_3 = [1107015393_i32,1551798317_i32,(-875456343_i32)];
RET = 131689177886957724469180609753401668740_i128 | 21051346773464986057185964437087239187_i128;
Call(RET = fn5(_2, _2, _3, _1, _2, _2, _3, _1, _4, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 134042343732999573164859179608578911997_u128 as u16;
_3 = [(-1211746738_i32),142580211_i32,48356154_i32];
_9 = !_1;
_7 = [240_u8,85_u8,236_u8,216_u8,77_u8,112_u8,17_u8,69_u8];
RET = !(-156850386201676419104648513734636850380_i128);
_8 = 253_u8;
_8 = 215_u8;
RET = 66222945821925487822024105296062359792_i128 * (-68700021202941555669659089763972727812_i128);
_2 = [165777162344861233301579588734477684547_u128,106727800454888248530704698903046637942_u128,76777152320458936441466306687121609616_u128,16431547639030983798416479722544351152_u128,120516744520283227312837792873355293559_u128,237969297111478634832362334708431981815_u128];
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1507700192_i32),(-1560467837_i32),(-225405236_i32)];
_1 = !_9;
_3 = [1032801890_i32,(-1762379128_i32),(-1424302616_i32)];
_12 = -(-7213692408984795849_i64);
_12 = 2195889810_u32 as i64;
match _8 {
0 => bb2,
215 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_12 = (-1992442252429557496_i64);
_11 = [43_i8,(-78_i8)];
_4 = [309298164967022205977033672342711788760_u128,190497237112010313718138785251223715684_u128,289338997657253436627742403613514373980_u128,152305568872868526913395215280186200358_u128,63678216023764800797888742247557008559_u128,187279754312140804462834862132273657307_u128];
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1254744221_i32),1395269148_i32,56469299_i32];
RET = !(-60254199424546422447781595264354949926_i128);
_5 = RET as u16;
_9 = _1 * _1;
_11 = [8_i8,115_i8];
_3 = [(-301823393_i32),(-1980504607_i32),(-1888231410_i32)];
_16 = '\u{d3c37}';
_2 = [130947637580642646767532647860506005460_u128,1340536225090634973858241595753205165_u128,259143600783734583062000899648096779334_u128,163341292643667178246960692043058806774_u128,19963899584787513394412668663230703264_u128,99341902671744848557224774531916325440_u128];
_5 = RET as u16;
_11 = [116_i8,(-119_i8)];
_18 = [104_i8,33_i8,(-37_i8),125_i8,106_i8,49_i8];
RET = -92821328733757369596678739262935308875_i128;
_15 = core::ptr::addr_of_mut!(_12);
match _12 {
0 => bb1,
1 => bb5,
340282366920938463461382165179338653960 => bb7,
_ => bb6
}
}
bb5 = {
Return()
}
bb6 = {
_5 = 134042343732999573164859179608578911997_u128 as u16;
_3 = [(-1211746738_i32),142580211_i32,48356154_i32];
_9 = !_1;
_7 = [240_u8,85_u8,236_u8,216_u8,77_u8,112_u8,17_u8,69_u8];
RET = !(-156850386201676419104648513734636850380_i128);
_8 = 253_u8;
_8 = 215_u8;
RET = 66222945821925487822024105296062359792_i128 * (-68700021202941555669659089763972727812_i128);
_2 = [165777162344861233301579588734477684547_u128,106727800454888248530704698903046637942_u128,76777152320458936441466306687121609616_u128,16431547639030983798416479722544351152_u128,120516744520283227312837792873355293559_u128,237969297111478634832362334708431981815_u128];
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1507700192_i32),(-1560467837_i32),(-225405236_i32)];
_1 = !_9;
_3 = [1032801890_i32,(-1762379128_i32),(-1424302616_i32)];
_12 = -(-7213692408984795849_i64);
_12 = 2195889810_u32 as i64;
match _8 {
0 => bb2,
215 => bb4,
_ => bb3
}
}
bb7 = {
_3 = [(-1908069174_i32),332071486_i32,(-138876691_i32)];
(*_15) = _8 as i64;
_13 = 74224782634975825833857181729396858758_u128;
_2 = [_13,_13,_13,_13,_13,_13];
_12 = 6763172961165124249_i64;
_20 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_5 = !36417_u16;
RET = 98_i8 as i128;
(*_15) = _8 as i64;
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_14 = _20;
match _13 {
74224782634975825833857181729396858758 => bb8,
_ => bb4
}
}
bb8 = {
_3 = [1294690132_i32,1110966062_i32,(-224575515_i32)];
_16 = '\u{76d6e}';
_21 = _16;
_9 = _1;
(*_15) = 1313926670905758703_i64;
_4 = _2;
_23 = [_1,_9];
_11 = [(-88_i8),(-11_i8)];
_25 = _21;
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_21 = _16;
_13 = 318407964332050696897710072847378284318_u128;
match _13 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
318407964332050696897710072847378284318 => bb14,
_ => bb13
}
}
bb9 = {
Return()
}
bb10 = {
_5 = 134042343732999573164859179608578911997_u128 as u16;
_3 = [(-1211746738_i32),142580211_i32,48356154_i32];
_9 = !_1;
_7 = [240_u8,85_u8,236_u8,216_u8,77_u8,112_u8,17_u8,69_u8];
RET = !(-156850386201676419104648513734636850380_i128);
_8 = 253_u8;
_8 = 215_u8;
RET = 66222945821925487822024105296062359792_i128 * (-68700021202941555669659089763972727812_i128);
_2 = [165777162344861233301579588734477684547_u128,106727800454888248530704698903046637942_u128,76777152320458936441466306687121609616_u128,16431547639030983798416479722544351152_u128,120516744520283227312837792873355293559_u128,237969297111478634832362334708431981815_u128];
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1507700192_i32),(-1560467837_i32),(-225405236_i32)];
_1 = !_9;
_3 = [1032801890_i32,(-1762379128_i32),(-1424302616_i32)];
_12 = -(-7213692408984795849_i64);
_12 = 2195889810_u32 as i64;
match _8 {
0 => bb2,
215 => bb4,
_ => bb3
}
}
bb11 = {
_5 = 134042343732999573164859179608578911997_u128 as u16;
_3 = [(-1211746738_i32),142580211_i32,48356154_i32];
_9 = !_1;
_7 = [240_u8,85_u8,236_u8,216_u8,77_u8,112_u8,17_u8,69_u8];
RET = !(-156850386201676419104648513734636850380_i128);
_8 = 253_u8;
_8 = 215_u8;
RET = 66222945821925487822024105296062359792_i128 * (-68700021202941555669659089763972727812_i128);
_2 = [165777162344861233301579588734477684547_u128,106727800454888248530704698903046637942_u128,76777152320458936441466306687121609616_u128,16431547639030983798416479722544351152_u128,120516744520283227312837792873355293559_u128,237969297111478634832362334708431981815_u128];
_10 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1507700192_i32),(-1560467837_i32),(-225405236_i32)];
_1 = !_9;
_3 = [1032801890_i32,(-1762379128_i32),(-1424302616_i32)];
_12 = -(-7213692408984795849_i64);
_12 = 2195889810_u32 as i64;
match _8 {
0 => bb2,
215 => bb4,
_ => bb3
}
}
bb12 = {
_12 = (-1992442252429557496_i64);
_11 = [43_i8,(-78_i8)];
_4 = [309298164967022205977033672342711788760_u128,190497237112010313718138785251223715684_u128,289338997657253436627742403613514373980_u128,152305568872868526913395215280186200358_u128,63678216023764800797888742247557008559_u128,187279754312140804462834862132273657307_u128];
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [(-1254744221_i32),1395269148_i32,56469299_i32];
RET = !(-60254199424546422447781595264354949926_i128);
_5 = RET as u16;
_9 = _1 * _1;
_11 = [8_i8,115_i8];
_3 = [(-301823393_i32),(-1980504607_i32),(-1888231410_i32)];
_16 = '\u{d3c37}';
_2 = [130947637580642646767532647860506005460_u128,1340536225090634973858241595753205165_u128,259143600783734583062000899648096779334_u128,163341292643667178246960692043058806774_u128,19963899584787513394412668663230703264_u128,99341902671744848557224774531916325440_u128];
_5 = RET as u16;
_11 = [116_i8,(-119_i8)];
_18 = [104_i8,33_i8,(-37_i8),125_i8,106_i8,49_i8];
RET = -92821328733757369596678739262935308875_i128;
_15 = core::ptr::addr_of_mut!(_12);
match _12 {
0 => bb1,
1 => bb5,
340282366920938463461382165179338653960 => bb7,
_ => bb6
}
}
bb13 = {
Return()
}
bb14 = {
_18 = [(-26_i8),(-34_i8),96_i8,97_i8,79_i8,(-124_i8)];
_3 = [(-39025736_i32),(-1751510505_i32),234847970_i32];
_8 = !166_u8;
RET = !(-42914406228756144256481493723782358020_i128);
_27 = true | true;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(4_usize, 3_usize, Move(_3), 2_usize, Move(_2), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(4_usize, 20_usize, Move(_20), 8_usize, Move(_8), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(4_usize, 21_usize, Move(_21), 23_usize, Move(_23), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u128; 6],mut _2: [u128; 6],mut _3: [i32; 3],mut _4: u64,mut _5: [u128; 6],mut _6: [u128; 6],mut _7: [i32; 3],mut _8: u64,mut _9: [u128; 6],mut _10: u64) -> i128 {
mir! {
type RET = i128;
let _11: char;
let _12: isize;
let _13: &'static &'static u32;
let _14: *mut f32;
let _15: i8;
let _16: isize;
let _17: (*mut usize, (i64, i16, &'static &'static i8, u16));
let _18: [char; 2];
let _19: bool;
let _20: ([u32; 3], char);
let _21: [u8; 8];
let _22: &'static [u16; 5];
let _23: Adt35;
let _24: isize;
let _25: bool;
let _26: f64;
let _27: Adt63;
let _28: *const f64;
let _29: &'static [u128; 6];
let _30: f64;
let _31: ();
let _32: ();
{
RET = -(-101320623187851273570596874116438246574_i128);
_1 = [134287275609452645531424503567456786356_u128,252623061925637218719165198142668302092_u128,267849662946900252330638052103973490480_u128,326757085485689884772100265943875490995_u128,169984829237921485918938761095869236791_u128,276431249980168223008529841426504964130_u128];
_5 = [4766108239336929544696254170419540740_u128,138121626183731026324642212625541817833_u128,320712636677490029738987999565695649864_u128,102197013322978597703111524065715146174_u128,82443357521216439522329089841588707048_u128,169969751015247324094803679976940482336_u128];
_1 = [60523111708803716503038455565833663641_u128,252590135763251832969700856436779635069_u128,207575336363057461905719523275104880085_u128,15166070125885237405766281199185341965_u128,6411234246702097084899515161903450059_u128,315192787895860438488460719257359112276_u128];
_2 = _6;
_11 = '\u{db0a2}';
RET = (-136047796726874205140365171439592085374_i128);
_1 = [307104190123390707417121583069815381051_u128,126370948186916289492465396591632951895_u128,181564799692009483074213043322165157891_u128,254812635241170535496618126112569396541_u128,265216454756633358708502049758081681788_u128,67888282118560406318889678429155729532_u128];
_8 = !_10;
_3 = [751134556_i32,1288116651_i32,(-118479619_i32)];
_4 = _8 - _8;
Goto(bb1)
}
bb1 = {
_8 = _10 + _10;
_5 = _9;
_15 = -(-14_i8);
_2 = [13421675846346701807654576717652689449_u128,162239660820593426537603337697468741151_u128,178154747320330274001202440423272910318_u128,160751778075607919809134511386189135671_u128,204337457566927641589469738715265420746_u128,200911655370726037184451786326280825528_u128];
RET = -(-69303211444582870677157188300887220261_i128);
_15 = 92681885167886567280327969271388338026_u128 as i8;
_6 = _9;
_9 = [47205916821819124041175843366611534104_u128,41074874209085365396604433327025698995_u128,306393792518379247399736909231464875022_u128,82419520404539890037375863389011696260_u128,238204589709281921164476530014563687127_u128,252866442410685335565895130293457238548_u128];
_16 = 14933689328043807816117143561087077099_u128 as isize;
_10 = _4 - _8;
_10 = 3426846377171966748_i64 as u64;
_16 = (-17513_i16) as isize;
_7 = [(-979637566_i32),(-1614939166_i32),1477007823_i32];
_5 = [32271479627505672159520362313716887767_u128,55847941301447324036489825397969891790_u128,290717278835349410113272695374401434002_u128,323294024846249207219805036416686892627_u128,290974683744857444458065157894723586058_u128,108042333733186918998737137339299554070_u128];
_11 = '\u{87103}';
_17.1.0 = RET as i64;
RET = (-63484633832711157808639332972008800696_i128);
_9 = _2;
Call(RET = fn6(_1, _8, _7, _4, _9, _4, _8, _5, _6, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.1.1 = (-12139_i16);
_6 = [329080119984769968055321597240128730677_u128,58125689539264140728882669335464260248_u128,87131380202262631382146672191169689204_u128,107130340661552455669626683435180612790_u128,176768611113435410414253194143379130780_u128,272960610709454559478767068800040585428_u128];
_17.1.3 = 59992_u16 | 23072_u16;
_17.1.0 = (-946466486701602133_i64) ^ 2768913354158193970_i64;
_19 = !true;
_17.1.0 = RET as i64;
RET = 161754235944519247220787930712557085045_i128;
_18 = [_11,_11];
_1 = [248839680761601923796694108484307280092_u128,18048550794365134944406640247877391677_u128,79782416522951884886955994785360093976_u128,80387403336104962393314983795634092819_u128,74062500266954918310002343619346413982_u128,193095822452156119293018661065041240358_u128];
Goto(bb3)
}
bb3 = {
RET = -157910325455404367680786574817921066459_i128;
_12 = _16 | _16;
_20.0 = [3934580910_u32,1090392846_u32,494694333_u32];
_11 = '\u{d75e0}';
_21 = [229_u8,51_u8,102_u8,104_u8,134_u8,156_u8,155_u8,86_u8];
_19 = _11 != _11;
_6 = [199859165128266799275409645845346388455_u128,234167262515903021716504756349339013226_u128,74890572691241678542364810713913768194_u128,286543401704704946977483153001078657931_u128,137607093808000185579282501315626088494_u128,110455294354999885829952988943967635772_u128];
_9 = _5;
_21 = [182_u8,138_u8,36_u8,152_u8,51_u8,83_u8,159_u8,71_u8];
_6 = [107984017340784959320839877626363469815_u128,265968993862367161141754159601756448759_u128,306747382010068967297643534535249364492_u128,91985644746777818375368199903539926949_u128,238714406638925796179778651847000247151_u128,132942291963684733457194979716821231935_u128];
_18 = [_11,_11];
match _17.1.1 {
340282366920938463463374607431768199317 => bb4,
_ => bb2
}
}
bb4 = {
_15 = 47_i8;
match _15 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
47 => bb11,
_ => bb10
}
}
bb5 = {
RET = -157910325455404367680786574817921066459_i128;
_12 = _16 | _16;
_20.0 = [3934580910_u32,1090392846_u32,494694333_u32];
_11 = '\u{d75e0}';
_21 = [229_u8,51_u8,102_u8,104_u8,134_u8,156_u8,155_u8,86_u8];
_19 = _11 != _11;
_6 = [199859165128266799275409645845346388455_u128,234167262515903021716504756349339013226_u128,74890572691241678542364810713913768194_u128,286543401704704946977483153001078657931_u128,137607093808000185579282501315626088494_u128,110455294354999885829952988943967635772_u128];
_9 = _5;
_21 = [182_u8,138_u8,36_u8,152_u8,51_u8,83_u8,159_u8,71_u8];
_6 = [107984017340784959320839877626363469815_u128,265968993862367161141754159601756448759_u128,306747382010068967297643534535249364492_u128,91985644746777818375368199903539926949_u128,238714406638925796179778651847000247151_u128,132942291963684733457194979716821231935_u128];
_18 = [_11,_11];
match _17.1.1 {
340282366920938463463374607431768199317 => bb4,
_ => bb2
}
}
bb6 = {
_17.1.1 = (-12139_i16);
_6 = [329080119984769968055321597240128730677_u128,58125689539264140728882669335464260248_u128,87131380202262631382146672191169689204_u128,107130340661552455669626683435180612790_u128,176768611113435410414253194143379130780_u128,272960610709454559478767068800040585428_u128];
_17.1.3 = 59992_u16 | 23072_u16;
_17.1.0 = (-946466486701602133_i64) ^ 2768913354158193970_i64;
_19 = !true;
_17.1.0 = RET as i64;
RET = 161754235944519247220787930712557085045_i128;
_18 = [_11,_11];
_1 = [248839680761601923796694108484307280092_u128,18048550794365134944406640247877391677_u128,79782416522951884886955994785360093976_u128,80387403336104962393314983795634092819_u128,74062500266954918310002343619346413982_u128,193095822452156119293018661065041240358_u128];
Goto(bb3)
}
bb7 = {
_8 = _10 + _10;
_5 = _9;
_15 = -(-14_i8);
_2 = [13421675846346701807654576717652689449_u128,162239660820593426537603337697468741151_u128,178154747320330274001202440423272910318_u128,160751778075607919809134511386189135671_u128,204337457566927641589469738715265420746_u128,200911655370726037184451786326280825528_u128];
RET = -(-69303211444582870677157188300887220261_i128);
_15 = 92681885167886567280327969271388338026_u128 as i8;
_6 = _9;
_9 = [47205916821819124041175843366611534104_u128,41074874209085365396604433327025698995_u128,306393792518379247399736909231464875022_u128,82419520404539890037375863389011696260_u128,238204589709281921164476530014563687127_u128,252866442410685335565895130293457238548_u128];
_16 = 14933689328043807816117143561087077099_u128 as isize;
_10 = _4 - _8;
_10 = 3426846377171966748_i64 as u64;
_16 = (-17513_i16) as isize;
_7 = [(-979637566_i32),(-1614939166_i32),1477007823_i32];
_5 = [32271479627505672159520362313716887767_u128,55847941301447324036489825397969891790_u128,290717278835349410113272695374401434002_u128,323294024846249207219805036416686892627_u128,290974683744857444458065157894723586058_u128,108042333733186918998737137339299554070_u128];
_11 = '\u{87103}';
_17.1.0 = RET as i64;
RET = (-63484633832711157808639332972008800696_i128);
_9 = _2;
Call(RET = fn6(_1, _8, _7, _4, _9, _4, _8, _5, _6, _5), ReturnTo(bb2), UnwindUnreachable())
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
_8 = _4 * _4;
_7 = [(-2012274491_i32),(-83120798_i32),2101631369_i32];
_5 = [310130383666189499682516722115701069269_u128,327358095285150744111591282082263475350_u128,296784672107422349078464669096030839514_u128,190322172240878176062786441424594604302_u128,203736431121850811334528624710641819301_u128,136535663107553081447334808566263971511_u128];
_9 = _6;
Goto(bb12)
}
bb12 = {
_21 = [7_u8,118_u8,49_u8,221_u8,203_u8,24_u8,253_u8,54_u8];
_11 = '\u{109323}';
_2 = _1;
_9 = [73311888434514220393731072450063269903_u128,231984830038800746179360579468358505017_u128,118294004634174900084506699688514441190_u128,224520468022201679383461935333891107383_u128,182980166289788569385866541026818049136_u128,323183206103861776293831875510357723640_u128];
RET = (-132195634653912094733836846235026374021_i128);
_20.0 = [171413882_u32,3623801624_u32,2223110592_u32];
_20.1 = _11;
_2 = [180885128392184642301843290678652109927_u128,293442686079086295211601840434388966616_u128,122267573134371208720117002688206330652_u128,180379285521913346011672594485707771620_u128,231981300325296760250677384893636282154_u128,278911755374022575358071789617034968237_u128];
_20.0 = [3955580676_u32,2282080025_u32,2535903686_u32];
_2 = [317173941086814596879737875575635975846_u128,310003436195081410495745415717569490811_u128,242748349206113004516063079600128679474_u128,78712202074217091064371147896854529178_u128,122927498072649970730797091980495734093_u128,94365993623267477704663369874063471269_u128];
match _17.1.1 {
0 => bb4,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463463374607431768199317 => bb19,
_ => bb18
}
}
bb13 = {
_8 = _4 * _4;
_7 = [(-2012274491_i32),(-83120798_i32),2101631369_i32];
_5 = [310130383666189499682516722115701069269_u128,327358095285150744111591282082263475350_u128,296784672107422349078464669096030839514_u128,190322172240878176062786441424594604302_u128,203736431121850811334528624710641819301_u128,136535663107553081447334808566263971511_u128];
_9 = _6;
Goto(bb12)
}
bb14 = {
RET = -157910325455404367680786574817921066459_i128;
_12 = _16 | _16;
_20.0 = [3934580910_u32,1090392846_u32,494694333_u32];
_11 = '\u{d75e0}';
_21 = [229_u8,51_u8,102_u8,104_u8,134_u8,156_u8,155_u8,86_u8];
_19 = _11 != _11;
_6 = [199859165128266799275409645845346388455_u128,234167262515903021716504756349339013226_u128,74890572691241678542364810713913768194_u128,286543401704704946977483153001078657931_u128,137607093808000185579282501315626088494_u128,110455294354999885829952988943967635772_u128];
_9 = _5;
_21 = [182_u8,138_u8,36_u8,152_u8,51_u8,83_u8,159_u8,71_u8];
_6 = [107984017340784959320839877626363469815_u128,265968993862367161141754159601756448759_u128,306747382010068967297643534535249364492_u128,91985644746777818375368199903539926949_u128,238714406638925796179778651847000247151_u128,132942291963684733457194979716821231935_u128];
_18 = [_11,_11];
match _17.1.1 {
340282366920938463463374607431768199317 => bb4,
_ => bb2
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_8 = _10 + _10;
_5 = _9;
_15 = -(-14_i8);
_2 = [13421675846346701807654576717652689449_u128,162239660820593426537603337697468741151_u128,178154747320330274001202440423272910318_u128,160751778075607919809134511386189135671_u128,204337457566927641589469738715265420746_u128,200911655370726037184451786326280825528_u128];
RET = -(-69303211444582870677157188300887220261_i128);
_15 = 92681885167886567280327969271388338026_u128 as i8;
_6 = _9;
_9 = [47205916821819124041175843366611534104_u128,41074874209085365396604433327025698995_u128,306393792518379247399736909231464875022_u128,82419520404539890037375863389011696260_u128,238204589709281921164476530014563687127_u128,252866442410685335565895130293457238548_u128];
_16 = 14933689328043807816117143561087077099_u128 as isize;
_10 = _4 - _8;
_10 = 3426846377171966748_i64 as u64;
_16 = (-17513_i16) as isize;
_7 = [(-979637566_i32),(-1614939166_i32),1477007823_i32];
_5 = [32271479627505672159520362313716887767_u128,55847941301447324036489825397969891790_u128,290717278835349410113272695374401434002_u128,323294024846249207219805036416686892627_u128,290974683744857444458065157894723586058_u128,108042333733186918998737137339299554070_u128];
_11 = '\u{87103}';
_17.1.0 = RET as i64;
RET = (-63484633832711157808639332972008800696_i128);
_9 = _2;
Call(RET = fn6(_1, _8, _7, _4, _9, _4, _8, _5, _6, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_15 = 47_i8;
match _15 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
47 => bb11,
_ => bb10
}
}
bb19 = {
_9 = [77031116064012798348235201774559543948_u128,144158038801076000019427047699088347765_u128,33144066924043315781888795400244739058_u128,312259203526927897669848917270312638769_u128,24769015477462778604026579029461562873_u128,290494492554896589355475379568242493317_u128];
_20.1 = _11;
_21 = [211_u8,248_u8,110_u8,78_u8,68_u8,225_u8,130_u8,16_u8];
_12 = _16 << _4;
_26 = 6_usize as f64;
_17.1.0 = 7423313405326134061_i64;
_5 = [166853970193230822422888635517345164003_u128,175881245843331748768274281726281494196_u128,201141434451895150186645959882625148058_u128,273414386559313441081575865156517077597_u128,13491686148555938251831764476425073390_u128,326575625167143297641419328357039157488_u128];
_25 = _8 >= _8;
_3 = [(-1488959819_i32),(-890256206_i32),2077432501_i32];
_1 = [317829153329956629026705772788841803683_u128,237780111731908624048767908946408431586_u128,221975454994722192038083986259616642437_u128,31621502915166864659875456836650752587_u128,40736199183228643400557768678775315341_u128,291855583445258612909984531645983591610_u128];
RET = _17.1.0 as i128;
_20.1 = _11;
_28 = core::ptr::addr_of!(_26);
RET = 77846177375584716867366350569245176236_i128 | 62447320522738017764128633999126673512_i128;
_6 = [250206542501934654584629830371496005601_u128,309578727115902396538697056571914216617_u128,295753917622927568236555488483182775715_u128,14609549835043447466682340880971053254_u128,15854345854355480594142789382797873577_u128,309526432782037447869730690981557353699_u128];
Goto(bb20)
}
bb20 = {
Call(_31 = dump_var(5_usize, 4_usize, Move(_4), 19_usize, Move(_19), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_31 = dump_var(5_usize, 21_usize, Move(_21), 6_usize, Move(_6), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_31 = dump_var(5_usize, 5_usize, Move(_5), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u128; 6],mut _2: u64,mut _3: [i32; 3],mut _4: u64,mut _5: [u128; 6],mut _6: u64,mut _7: u64,mut _8: [u128; 6],mut _9: [u128; 6],mut _10: [u128; 6]) -> i128 {
mir! {
type RET = i128;
let _11: f32;
let _12: [i32; 3];
let _13: [isize; 3];
let _14: Adt70;
let _15: &'static f32;
let _16: u32;
let _17: [i32; 3];
let _18: isize;
let _19: *mut u128;
let _20: f64;
let _21: isize;
let _22: u16;
let _23: [i64; 8];
let _24: &'static u32;
let _25: isize;
let _26: i32;
let _27: bool;
let _28: [char; 2];
let _29: Adt69;
let _30: Adt74;
let _31: bool;
let _32: f64;
let _33: &'static u32;
let _34: isize;
let _35: f32;
let _36: &'static u32;
let _37: isize;
let _38: (char, *const i8);
let _39: &'static i8;
let _40: ();
let _41: ();
{
RET = (-26_i8) as i128;
RET = -(-63085815371134285338016310149530673529_i128);
_7 = !_6;
_5 = _1;
_7 = !_4;
_11 = (-766037290_i32) as f32;
_9 = [197116596157880596005201881262062160648_u128,38915702045457695037653274387143842408_u128,136243351627862951593916489336760569750_u128,83136199802691120130595681272398683252_u128,110743750981897707406006184716877738678_u128,172037606422479535145057942170455107864_u128];
_1 = [319294129770582463467613133548090167987_u128,196014510902848420993584375812817412547_u128,204429705521147776129091518011124043844_u128,21080310567925870049435163185958789795_u128,39930421951814910767289489744061697511_u128,288449191276499367522452793967680973762_u128];
_10 = _9;
_4 = !_2;
RET = (-8993951984797243262228839836655008627_i128) << _7;
_8 = [270696993001666182475710682337248235016_u128,200649527806305618112310448977088583531_u128,224059380145194945168847690678835898988_u128,160691309810473877556362366408723929363_u128,335246218171512443531150119204430687171_u128,195760538823926000227465405706941408313_u128];
_4 = _7;
_4 = _2;
_6 = !_2;
_2 = !_4;
RET = !(-100672040722451216238907571333295575726_i128);
Goto(bb1)
}
bb1 = {
_9 = [163996848390928818392164235705392376075_u128,132070305533433865088739814573169724008_u128,37016971762424667391124644555660728592_u128,297545170449769139860665874587833905428_u128,229153647036498100914719233807614067129_u128,169436077933915084330785906456079756130_u128];
RET = 12668096641416218205_usize as i128;
_6 = 236_u8 as u64;
_6 = !_4;
_12 = [437363968_i32,1645980789_i32,(-1315920329_i32)];
_9 = [125225734244605796729410951143420100080_u128,148503201852056068989862569634746071410_u128,30288556142295745982269048128607116536_u128,132446690018877060244280874067586457812_u128,301682568814330501615648161219464104621_u128,284186439664224096036356120184139823922_u128];
RET = 156149267150140955072586551913767602340_i128;
_3 = [(-2086899718_i32),(-1924893513_i32),330709429_i32];
_4 = (-10938_i16) as u64;
_1 = [209446416926171166881393718020264307951_u128,148002310983770102433021526064123569129_u128,251042433155840492092801844485910936495_u128,83050819957958664060541936622918844564_u128,188191264041057240520285443238098494062_u128,180950107621909916164260381782542873923_u128];
_10 = [66730449700222541306116544005667447465_u128,31525436896834066380285797894032968988_u128,151404258764000420351032162394097646985_u128,89132725381014047697316167875024753531_u128,103440551531627564082572007136931267568_u128,130588153251166214437904529917373899790_u128];
_11 = 2176_u16 as f32;
RET = !(-95368657615771478138173695037233634796_i128);
_11 = RET as f32;
_4 = _6 << _7;
_15 = &_11;
_3 = _12;
_6 = _4 * _4;
_3 = _12;
_6 = _2 + _4;
_11 = 0_u8 as f32;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = [21180424118777859432159119020137595618_u128,121920521967768228358466464191864429042_u128,219797909862385115747351791602975211108_u128,23458398581748592596995982171329601866_u128,256148303394777861969781622394837477032_u128,83872931428716078881604202417877611574_u128];
Call(_10 = fn7(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_20 = 201_u8 as f64;
_18 = RET as isize;
RET = 125634462596512776426138886356128909839_i128 ^ (-141197298649377004934764256212949268914_i128);
_11 = (-2565866003975070014_i64) as f32;
_20 = 17440_i16 as f64;
_6 = _2;
_15 = &_11;
_1 = [61960952681122456140564385564694503315_u128,222213478159996313184955706561766837753_u128,293163308818809229239461972070307920025_u128,163546651904404481338555721446479023573_u128,125677266902207510017380550234460627432_u128,63151586689190730340286023930097907512_u128];
_1 = [101388625067866032439321942825169510523_u128,209325861931543095344296930977427971469_u128,275756301626452134213825982940581572241_u128,247382577462793356432459404400517565813_u128,251000408006616623186203027313962322781_u128,45619345779398667574591286034950441910_u128];
_5 = [71761693260540672860800842234602754789_u128,123320284545962907423361432262342490799_u128,330189690300147711889278006280340089040_u128,303406489089694932532645357951563996003_u128,38052040163254857576000318570229624335_u128,33467764177933736254138846645117599929_u128];
_9 = [262203135439134273708427311624863599045_u128,79324361125824356670302594656582769991_u128,84018710066408223199077196328194276966_u128,295819520885049441601907520396916023162_u128,217492487457837884276441865484966117408_u128,24403021461832629551545140087420351797_u128];
Goto(bb3)
}
bb3 = {
_6 = _2 << _7;
_8 = _5;
_21 = -_18;
_2 = _4;
_17 = [(-1740502258_i32),2049783038_i32,(-894233053_i32)];
_17 = _12;
_3 = [(-1044640383_i32),967253023_i32,(-1980612643_i32)];
_2 = _7 * _7;
_1 = _8;
_3 = [2011636058_i32,(-407895268_i32),(-1491816801_i32)];
_12 = [906641054_i32,(-1257270643_i32),(-93100059_i32)];
_2 = _4;
_25 = !_21;
_25 = _21;
_5 = _1;
_24 = &_16;
Goto(bb4)
}
bb4 = {
_20 = (-14_i8) as f64;
_16 = !223458137_u32;
_26 = !1838839236_i32;
_27 = true;
_22 = 47020_u16;
_24 = &_16;
_11 = _22 as f32;
_16 = 3666389915_u32;
_16 = 80824902_u32 | 2927965126_u32;
_6 = _7 - _4;
_15 = &_11;
_2 = !_6;
_6 = _4 - _2;
_7 = _6;
_4 = !_2;
Goto(bb5)
}
bb5 = {
_22 = '\u{8da15}' as u16;
_9 = _8;
Call(_22 = fn9(Move(_15), _7, _7, _1, _7, _7, _7, _2, _2, _7, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_25 = 7439786242965332958_usize as isize;
_23 = [5426475651119164933_i64,(-2046851252013283549_i64),(-3430984204638954602_i64),7280296086214191196_i64,1132469947164725799_i64,(-1676531013442209903_i64),7691809201416298312_i64,5300438492719555234_i64];
_3 = [_26,_26,_26];
_6 = _16 as u64;
_24 = &_16;
_16 = 3500899725_u32 + 791536572_u32;
_26 = 132_u8 as i32;
_24 = &_16;
_15 = &_11;
_28 = ['\u{1bd17}','\u{ede44}'];
_22 = 2904_u16 ^ 51286_u16;
_22 = (-94_i8) as u16;
Call(_29.fld1 = fn15(Move(_24), Move(_15), _7, _25, _2, _28, _4, _5, _7, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = [313721107714239361453714986194758400403_u128,273498035244279789783127512769743559439_u128,50813324488100906496857945347229686616_u128,112232350727522473887252279695364947921_u128,47270158068878612407454636145071147978_u128,140871563807085454363326391548376303843_u128];
_17 = _3;
_28 = ['\u{7d761}','\u{4164}'];
_9 = _5;
_8 = [68450304771898557247544291984625541103_u128,35238908171649036717458878273115188775_u128,147761656547324719035258943864160602249_u128,55262286335545512254165638016079844765_u128,119369042723914923437461571252948803189_u128,98211125383809759760833491216722346220_u128];
_9 = [254858864479114240598635296731859924365_u128,107894168224168400594295261936402913632_u128,221683018436005562719843917625974593057_u128,168524187123834795253075929233686009302_u128,78763011521958495503306188150181537694_u128,72471677103144409337911553567138563550_u128];
_29.fld3 = _12;
_29.fld2 = -3535560380791226812_i64;
_32 = 91_u8 as f64;
Goto(bb8)
}
bb8 = {
_11 = _29.fld1 as f32;
_24 = &_16;
_16 = 910647038_u32;
_12 = _17;
_20 = -_32;
_10 = [335131440800077837192765573652431644442_u128,323070105279415740672526966431983779997_u128,170032905934115200168537272952717715964_u128,56352332888577567284146674577950212348_u128,214625671710174663627356507775493809568_u128,197763402466966731562249176327122751030_u128];
_17 = [_26,_26,_26];
_15 = &_11;
_24 = &_16;
_2 = _4 + _7;
_23 = [_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2];
_31 = !_27;
_34 = _18;
_35 = -_11;
_4 = _22 as u64;
_5 = _10;
match _16 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
910647038 => bb16,
_ => bb15
}
}
bb9 = {
_1 = [313721107714239361453714986194758400403_u128,273498035244279789783127512769743559439_u128,50813324488100906496857945347229686616_u128,112232350727522473887252279695364947921_u128,47270158068878612407454636145071147978_u128,140871563807085454363326391548376303843_u128];
_17 = _3;
_28 = ['\u{7d761}','\u{4164}'];
_9 = _5;
_8 = [68450304771898557247544291984625541103_u128,35238908171649036717458878273115188775_u128,147761656547324719035258943864160602249_u128,55262286335545512254165638016079844765_u128,119369042723914923437461571252948803189_u128,98211125383809759760833491216722346220_u128];
_9 = [254858864479114240598635296731859924365_u128,107894168224168400594295261936402913632_u128,221683018436005562719843917625974593057_u128,168524187123834795253075929233686009302_u128,78763011521958495503306188150181537694_u128,72471677103144409337911553567138563550_u128];
_29.fld3 = _12;
_29.fld2 = -3535560380791226812_i64;
_32 = 91_u8 as f64;
Goto(bb8)
}
bb10 = {
_25 = 7439786242965332958_usize as isize;
_23 = [5426475651119164933_i64,(-2046851252013283549_i64),(-3430984204638954602_i64),7280296086214191196_i64,1132469947164725799_i64,(-1676531013442209903_i64),7691809201416298312_i64,5300438492719555234_i64];
_3 = [_26,_26,_26];
_6 = _16 as u64;
_24 = &_16;
_16 = 3500899725_u32 + 791536572_u32;
_26 = 132_u8 as i32;
_24 = &_16;
_15 = &_11;
_28 = ['\u{1bd17}','\u{ede44}'];
_22 = 2904_u16 ^ 51286_u16;
_22 = (-94_i8) as u16;
Call(_29.fld1 = fn15(Move(_24), Move(_15), _7, _25, _2, _28, _4, _5, _7, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_22 = '\u{8da15}' as u16;
_9 = _8;
Call(_22 = fn9(Move(_15), _7, _7, _1, _7, _7, _7, _2, _2, _7, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_20 = (-14_i8) as f64;
_16 = !223458137_u32;
_26 = !1838839236_i32;
_27 = true;
_22 = 47020_u16;
_24 = &_16;
_11 = _22 as f32;
_16 = 3666389915_u32;
_16 = 80824902_u32 | 2927965126_u32;
_6 = _7 - _4;
_15 = &_11;
_2 = !_6;
_6 = _4 - _2;
_7 = _6;
_4 = !_2;
Goto(bb5)
}
bb13 = {
_6 = _2 << _7;
_8 = _5;
_21 = -_18;
_2 = _4;
_17 = [(-1740502258_i32),2049783038_i32,(-894233053_i32)];
_17 = _12;
_3 = [(-1044640383_i32),967253023_i32,(-1980612643_i32)];
_2 = _7 * _7;
_1 = _8;
_3 = [2011636058_i32,(-407895268_i32),(-1491816801_i32)];
_12 = [906641054_i32,(-1257270643_i32),(-93100059_i32)];
_2 = _4;
_25 = !_21;
_25 = _21;
_5 = _1;
_24 = &_16;
Goto(bb4)
}
bb14 = {
_20 = 201_u8 as f64;
_18 = RET as isize;
RET = 125634462596512776426138886356128909839_i128 ^ (-141197298649377004934764256212949268914_i128);
_11 = (-2565866003975070014_i64) as f32;
_20 = 17440_i16 as f64;
_6 = _2;
_15 = &_11;
_1 = [61960952681122456140564385564694503315_u128,222213478159996313184955706561766837753_u128,293163308818809229239461972070307920025_u128,163546651904404481338555721446479023573_u128,125677266902207510017380550234460627432_u128,63151586689190730340286023930097907512_u128];
_1 = [101388625067866032439321942825169510523_u128,209325861931543095344296930977427971469_u128,275756301626452134213825982940581572241_u128,247382577462793356432459404400517565813_u128,251000408006616623186203027313962322781_u128,45619345779398667574591286034950441910_u128];
_5 = [71761693260540672860800842234602754789_u128,123320284545962907423361432262342490799_u128,330189690300147711889278006280340089040_u128,303406489089694932532645357951563996003_u128,38052040163254857576000318570229624335_u128,33467764177933736254138846645117599929_u128];
_9 = [262203135439134273708427311624863599045_u128,79324361125824356670302594656582769991_u128,84018710066408223199077196328194276966_u128,295819520885049441601907520396916023162_u128,217492487457837884276441865484966117408_u128,24403021461832629551545140087420351797_u128];
Goto(bb3)
}
bb15 = {
_9 = [163996848390928818392164235705392376075_u128,132070305533433865088739814573169724008_u128,37016971762424667391124644555660728592_u128,297545170449769139860665874587833905428_u128,229153647036498100914719233807614067129_u128,169436077933915084330785906456079756130_u128];
RET = 12668096641416218205_usize as i128;
_6 = 236_u8 as u64;
_6 = !_4;
_12 = [437363968_i32,1645980789_i32,(-1315920329_i32)];
_9 = [125225734244605796729410951143420100080_u128,148503201852056068989862569634746071410_u128,30288556142295745982269048128607116536_u128,132446690018877060244280874067586457812_u128,301682568814330501615648161219464104621_u128,284186439664224096036356120184139823922_u128];
RET = 156149267150140955072586551913767602340_i128;
_3 = [(-2086899718_i32),(-1924893513_i32),330709429_i32];
_4 = (-10938_i16) as u64;
_1 = [209446416926171166881393718020264307951_u128,148002310983770102433021526064123569129_u128,251042433155840492092801844485910936495_u128,83050819957958664060541936622918844564_u128,188191264041057240520285443238098494062_u128,180950107621909916164260381782542873923_u128];
_10 = [66730449700222541306116544005667447465_u128,31525436896834066380285797894032968988_u128,151404258764000420351032162394097646985_u128,89132725381014047697316167875024753531_u128,103440551531627564082572007136931267568_u128,130588153251166214437904529917373899790_u128];
_11 = 2176_u16 as f32;
RET = !(-95368657615771478138173695037233634796_i128);
_11 = RET as f32;
_4 = _6 << _7;
_15 = &_11;
_3 = _12;
_6 = _4 * _4;
_3 = _12;
_6 = _2 + _4;
_11 = 0_u8 as f32;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = [21180424118777859432159119020137595618_u128,121920521967768228358466464191864429042_u128,219797909862385115747351791602975211108_u128,23458398581748592596995982171329601866_u128,256148303394777861969781622394837477032_u128,83872931428716078881604202417877611574_u128];
Call(_10 = fn7(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_29.fld0 = 287773929189416455619024397071735672752_u128 as u64;
_24 = &_16;
_33 = &(*_24);
_27 = _31;
_34 = _18;
_12 = _17;
_26 = _7 as i32;
_3 = [_26,_26,_26];
_12 = [_26,_26,_26];
_22 = !5955_u16;
_16 = _29.fld2 as u32;
RET = 163192965798699989889480309816056274199_i128 & (-10354771612031030955689037617114739287_i128);
_22 = 56459_u16 + 2979_u16;
_29.fld0 = _4;
_26 = -2075827604_i32;
_2 = _7;
_23 = [_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2];
_23 = [_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2,_29.fld2];
_21 = _34;
_24 = &_16;
_25 = _29.fld1 as isize;
_9 = _1;
_7 = _2 + _2;
_13 = [_18,_21,_34];
_25 = -_21;
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(6_usize, 12_usize, Move(_12), 21_usize, Move(_21), 31_usize, Move(_31), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(6_usize, 9_usize, Move(_9), 17_usize, Move(_17), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(6_usize, 18_usize, Move(_18), 3_usize, Move(_3), 23_usize, Move(_23), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [u128; 6]) -> [u128; 6] {
mir! {
type RET = [u128; 6];
let _2: u16;
let _3: char;
let _4: Adt63;
let _5: &'static u32;
let _6: bool;
let _7: i32;
let _8: i32;
let _9: f64;
let _10: &'static i32;
let _11: &'static [u16; 5];
let _12: u128;
let _13: f64;
let _14: isize;
let _15: f64;
let _16: &'static (char, *const i8);
let _17: isize;
let _18: bool;
let _19: &'static [u16; 5];
let _20: [isize; 3];
let _21: char;
let _22: [i32; 3];
let _23: [u8; 8];
let _24: i16;
let _25: ([u32; 3], char);
let _26: f64;
let _27: &'static &'static &'static i8;
let _28: ();
let _29: ();
{
RET = [329186871896131655903395556128266725387_u128,127432692577004298469205170942947676387_u128,93870751852680084255593639140844556822_u128,305570116372818146059853922935596399958_u128,116498260979759720930804315298428415320_u128,224907080327837236021203054983171983155_u128];
RET = [82375404074984286628832319401007196641_u128,73537110049334025512625702036076592722_u128,161522638344495516330743803509144814267_u128,322847296611061070356063639162060946363_u128,127775549726241103730263207329262404999_u128,67229821325050304276485190932525981069_u128];
_1 = RET;
_1 = RET;
_1 = RET;
_2 = 33961_u16 >> 2358748077_u32;
_3 = '\u{2ce88}';
_2 = 58191_u16 >> 31724_i16;
_3 = '\u{3cc5e}';
RET = [203543768751139554312859374974983818098_u128,302118000856027082348187887976024052372_u128,179889792765300970625219146566993482873_u128,117445768761981618794674707965325041986_u128,223165815953061213043604594272268920668_u128,324526606600779744630642486051993954780_u128];
_2 = !61701_u16;
RET = _1;
_2 = !55713_u16;
_3 = '\u{fbdd9}';
_2 = _3 as u16;
_3 = '\u{d68e9}';
Goto(bb1)
}
bb1 = {
_2 = !57932_u16;
RET = [166198714177725785150479806453121941489_u128,325250082915463673503697376905350230575_u128,330379690574816020041820270640462517006_u128,137774819975742693964305306042207652468_u128,9056369455220416724264259190172389362_u128,164747023042729012340244990103326498282_u128];
_2 = 28580_u16 * 29827_u16;
_2 = 64408_u16 - 49579_u16;
_2 = 49126_u16 >> (-15_i8);
_2 = 1992_u16;
RET = [222283780846209384320072594451864189584_u128,336063759293519954749169519108662951034_u128,218792163196224698273782702810879986770_u128,250840015871050646731579484443858521676_u128,34868540213493573396109270672921246956_u128,105831052091913220208877010375985119373_u128];
_1 = [258846883940271055547056993419090670198_u128,103829625034001479445019286545511031636_u128,241606578577819545191760804005336005471_u128,7192987520154391342748660471165012604_u128,25315718159811290352370096350861834858_u128,158191424465067140969993862154614607785_u128];
_1 = [220028825758729475721721263962027131824_u128,233592948244316012099683206593296976656_u128,303424117811844460097060528636267908319_u128,254200521890419669116336790793005911079_u128,176442831384955574512220855121852891677_u128,99535466666076644438729311501219221801_u128];
Goto(bb2)
}
bb2 = {
_3 = '\u{46332}';
RET = [143008862497689724894562328126022628956_u128,338602377787455972106779520996759446712_u128,100406721493453018192253263257277405838_u128,219750219257992297835250185371071134171_u128,46917603766534557718564724791624294670_u128,96400587555747720320391714756522316295_u128];
_2 = 30831_u16;
Call(_7 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = RET;
_1 = [125625806277219785363944846043943903979_u128,258202880232637462066071991452083402093_u128,154402222040641870591707322534408791009_u128,114066755285107923098266538469719606410_u128,250744859771025256750579022222159914175_u128,237637353643492731496172560230609245222_u128];
_8 = _7;
_6 = _2 > _2;
_9 = 273782487490402721816093479524008336843_u128 as f64;
_6 = _7 == _8;
_8 = 754687078208476400_usize as i32;
_2 = 13402_u16;
_3 = '\u{9021e}';
_8 = _7 & _7;
_6 = _2 != _2;
_7 = -_8;
Goto(bb4)
}
bb4 = {
_7 = _8 | _8;
RET = [85292292345550878949126490965032199137_u128,195750319873204297378059657526733427066_u128,316097059690042704143121874013636570176_u128,64666351300108713393369828911631515764_u128,52055504893627402606761561165318940905_u128,261856288404598013658975511856463080737_u128];
_3 = '\u{aacc8}';
_2 = 30298_u16 | 46245_u16;
RET = [288320095359111136429996080083826071717_u128,12559731443596139829690416705083372718_u128,288655380135956125796870693652384467046_u128,336414742798545571403045339197941531474_u128,245349564536750036606580453567314809147_u128,325397971148656521135206646068439340331_u128];
_3 = '\u{e4b70}';
RET = _1;
_9 = 3_usize as f64;
_10 = &_7;
_12 = 193360952895711500610382725224989030879_u128;
_3 = '\u{4e788}';
_10 = &(*_10);
_7 = _8;
_9 = _2 as f64;
_7 = 7_usize as i32;
_3 = '\u{a2e3b}';
Call(_7 = fn8(RET, RET, _1, _8, _6, _2, _12, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = 3439003841969250389_usize as f64;
_15 = _9 * _9;
RET = [_12,_12,_12,_12,_12,_12];
_13 = -_15;
RET = [_12,_12,_12,_12,_12,_12];
_8 = 4686061109238439053_usize as i32;
RET = [_12,_12,_12,_12,_12,_12];
_15 = 20508_i16 as f64;
RET = [_12,_12,_12,_12,_12,_12];
_6 = true;
match _12 {
0 => bb2,
193360952895711500610382725224989030879 => bb7,
_ => bb6
}
}
bb6 = {
_1 = RET;
_1 = [125625806277219785363944846043943903979_u128,258202880232637462066071991452083402093_u128,154402222040641870591707322534408791009_u128,114066755285107923098266538469719606410_u128,250744859771025256750579022222159914175_u128,237637353643492731496172560230609245222_u128];
_8 = _7;
_6 = _2 > _2;
_9 = 273782487490402721816093479524008336843_u128 as f64;
_6 = _7 == _8;
_8 = 754687078208476400_usize as i32;
_2 = 13402_u16;
_3 = '\u{9021e}';
_8 = _7 & _7;
_6 = _2 != _2;
_7 = -_8;
Goto(bb4)
}
bb7 = {
_6 = true;
_14 = -67_isize;
_8 = !_7;
match _12 {
0 => bb3,
1 => bb8,
2 => bb9,
3 => bb10,
193360952895711500610382725224989030879 => bb12,
_ => bb11
}
}
bb8 = {
_1 = RET;
_1 = [125625806277219785363944846043943903979_u128,258202880232637462066071991452083402093_u128,154402222040641870591707322534408791009_u128,114066755285107923098266538469719606410_u128,250744859771025256750579022222159914175_u128,237637353643492731496172560230609245222_u128];
_8 = _7;
_6 = _2 > _2;
_9 = 273782487490402721816093479524008336843_u128 as f64;
_6 = _7 == _8;
_8 = 754687078208476400_usize as i32;
_2 = 13402_u16;
_3 = '\u{9021e}';
_8 = _7 & _7;
_6 = _2 != _2;
_7 = -_8;
Goto(bb4)
}
bb9 = {
_2 = !57932_u16;
RET = [166198714177725785150479806453121941489_u128,325250082915463673503697376905350230575_u128,330379690574816020041820270640462517006_u128,137774819975742693964305306042207652468_u128,9056369455220416724264259190172389362_u128,164747023042729012340244990103326498282_u128];
_2 = 28580_u16 * 29827_u16;
_2 = 64408_u16 - 49579_u16;
_2 = 49126_u16 >> (-15_i8);
_2 = 1992_u16;
RET = [222283780846209384320072594451864189584_u128,336063759293519954749169519108662951034_u128,218792163196224698273782702810879986770_u128,250840015871050646731579484443858521676_u128,34868540213493573396109270672921246956_u128,105831052091913220208877010375985119373_u128];
_1 = [258846883940271055547056993419090670198_u128,103829625034001479445019286545511031636_u128,241606578577819545191760804005336005471_u128,7192987520154391342748660471165012604_u128,25315718159811290352370096350861834858_u128,158191424465067140969993862154614607785_u128];
_1 = [220028825758729475721721263962027131824_u128,233592948244316012099683206593296976656_u128,303424117811844460097060528636267908319_u128,254200521890419669116336790793005911079_u128,176442831384955574512220855121852891677_u128,99535466666076644438729311501219221801_u128];
Goto(bb2)
}
bb10 = {
_7 = _8 | _8;
RET = [85292292345550878949126490965032199137_u128,195750319873204297378059657526733427066_u128,316097059690042704143121874013636570176_u128,64666351300108713393369828911631515764_u128,52055504893627402606761561165318940905_u128,261856288404598013658975511856463080737_u128];
_3 = '\u{aacc8}';
_2 = 30298_u16 | 46245_u16;
RET = [288320095359111136429996080083826071717_u128,12559731443596139829690416705083372718_u128,288655380135956125796870693652384467046_u128,336414742798545571403045339197941531474_u128,245349564536750036606580453567314809147_u128,325397971148656521135206646068439340331_u128];
_3 = '\u{e4b70}';
RET = _1;
_9 = 3_usize as f64;
_10 = &_7;
_12 = 193360952895711500610382725224989030879_u128;
_3 = '\u{4e788}';
_10 = &(*_10);
_7 = _8;
_9 = _2 as f64;
_7 = 7_usize as i32;
_3 = '\u{a2e3b}';
Call(_7 = fn8(RET, RET, _1, _8, _6, _2, _12, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_1 = RET;
_1 = [125625806277219785363944846043943903979_u128,258202880232637462066071991452083402093_u128,154402222040641870591707322534408791009_u128,114066755285107923098266538469719606410_u128,250744859771025256750579022222159914175_u128,237637353643492731496172560230609245222_u128];
_8 = _7;
_6 = _2 > _2;
_9 = 273782487490402721816093479524008336843_u128 as f64;
_6 = _7 == _8;
_8 = 754687078208476400_usize as i32;
_2 = 13402_u16;
_3 = '\u{9021e}';
_8 = _7 & _7;
_6 = _2 != _2;
_7 = -_8;
Goto(bb4)
}
bb12 = {
_10 = &_7;
_6 = true & false;
_2 = 27302_u16;
_18 = _6 | _6;
RET = [_12,_12,_12,_12,_12,_12];
_7 = (-28805_i16) as i32;
_6 = _18 >= _18;
_17 = _3 as isize;
_21 = _3;
_12 = !21444338940900960223717804696468387384_u128;
_20 = [_14,_14,_14];
_13 = -_9;
_10 = &_7;
RET = _1;
_22 = [_8,(*_10),_8];
_20 = [_14,_17,_14];
_18 = !_6;
_25.0 = [3965431631_u32,2668844104_u32,3400143024_u32];
_22 = [_8,_8,(*_10)];
_1 = [_12,_12,_12,_12,_12,_12];
_21 = _3;
_17 = _14;
RET = [_12,_12,_12,_12,_12,_12];
match _2 {
0 => bb1,
1 => bb10,
27302 => bb13,
_ => bb5
}
}
bb13 = {
_10 = &(*_10);
_7 = _8;
_8 = !_7;
match _2 {
0 => bb1,
1 => bb9,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb12,
27302 => bb14,
_ => bb7
}
}
bb14 = {
_23 = [185_u8,83_u8,142_u8,239_u8,54_u8,31_u8,132_u8,184_u8];
_7 = (-23536_i16) as i32;
_12 = 246668090836894527857209641691077325760_u128 ^ 283647831774855616768675173035968116488_u128;
_15 = _13;
_10 = &_7;
_24 = -958_i16;
RET = _1;
_8 = (*_10);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(7_usize, 1_usize, Move(_1), 23_usize, Move(_23), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(7_usize, 22_usize, Move(_22), 12_usize, Move(_12), 6_usize, Move(_6), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [u128; 6],mut _2: [u128; 6],mut _3: [u128; 6],mut _4: i32,mut _5: bool,mut _6: u16,mut _7: u128,mut _8: bool) -> i32 {
mir! {
type RET = i32;
let _9: isize;
let _10: [u128; 6];
let _11: char;
let _12: bool;
let _13: [u32; 3];
let _14: u16;
let _15: f64;
let _16: *mut Adt45;
let _17: &'static *const i8;
let _18: &'static f32;
let _19: ();
let _20: ();
{
RET = 2637770403_u32 as i32;
_4 = 8723299102362679843_u64 as i32;
RET = _4;
_1 = [_7,_7,_7,_7,_7,_7];
_8 = _5;
_1 = [_7,_7,_7,_7,_7,_7];
RET = 269676452144740232_u64 as i32;
_7 = 224361917547305394724561308498102288782_u128 ^ 280975399950050490248388830258184732470_u128;
Call(_9 = core::intrinsics::bswap((-113_isize)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = (-88005255684322275392468659800633002560_i128) as u16;
_1 = [_7,_7,_7,_7,_7,_7];
_7 = !82000753153962841733365271690388410888_u128;
_6 = 22564_u16 * 43029_u16;
_1 = [_7,_7,_7,_7,_7,_7];
_4 = RET >> _7;
RET = _4 + _4;
_3 = [_7,_7,_7,_7,_7,_7];
_7 = 275994704864567071202436044673396998440_u128;
_4 = RET;
RET = !_4;
_1 = [_7,_7,_7,_7,_7,_7];
_7 = 330987593888902159357307412338677758725_u128;
_1 = _2;
RET = !_4;
_1 = [_7,_7,_7,_7,_7,_7];
Goto(bb2)
}
bb2 = {
_9 = (-9223372036854775808_isize);
_9 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_8 = !_5;
_1 = _3;
_9 = 15_i8 as isize;
_4 = RET;
_8 = !_5;
_1 = _2;
RET = _4 - _4;
RET = _4;
_5 = _6 > _6;
Goto(bb3)
}
bb3 = {
_4 = RET - RET;
_9 = (-9223372036854775808_isize);
_6 = (-8270_i16) as u16;
_3 = [_7,_7,_7,_7,_7,_7];
_8 = RET <= RET;
_8 = _5 >= _5;
_4 = 2_usize as i32;
_1 = [_7,_7,_7,_7,_7,_7];
_6 = 38416_u16 - 57218_u16;
_1 = [_7,_7,_7,_7,_7,_7];
_9 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_9 = (-16_isize);
RET = _4 * _4;
_3 = [_7,_7,_7,_7,_7,_7];
match _9 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211440 => bb9,
_ => bb8
}
}
bb4 = {
_9 = (-9223372036854775808_isize);
_9 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_8 = !_5;
_1 = _3;
_9 = 15_i8 as isize;
_4 = RET;
_8 = !_5;
_1 = _2;
RET = _4 - _4;
RET = _4;
_5 = _6 > _6;
Goto(bb3)
}
bb5 = {
_6 = (-88005255684322275392468659800633002560_i128) as u16;
_1 = [_7,_7,_7,_7,_7,_7];
_7 = !82000753153962841733365271690388410888_u128;
_6 = 22564_u16 * 43029_u16;
_1 = [_7,_7,_7,_7,_7,_7];
_4 = RET >> _7;
RET = _4 + _4;
_3 = [_7,_7,_7,_7,_7,_7];
_7 = 275994704864567071202436044673396998440_u128;
_4 = RET;
RET = !_4;
_1 = [_7,_7,_7,_7,_7,_7];
_7 = 330987593888902159357307412338677758725_u128;
_1 = _2;
RET = !_4;
_1 = [_7,_7,_7,_7,_7,_7];
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
RET = _4;
_2 = [_7,_7,_7,_7,_7,_7];
_12 = _6 >= _6;
_9 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_2 = [_7,_7,_7,_7,_7,_7];
_9 = 9223372036854775807_isize >> _6;
_14 = RET as u16;
match _7 {
330987593888902159357307412338677758725 => bb10,
_ => bb6
}
}
bb10 = {
_10 = _3;
_4 = -RET;
Goto(bb11)
}
bb11 = {
_12 = !_5;
_6 = (-8772691418555550983_i64) as u16;
_13 = [3235593501_u32,3479096335_u32,3751389720_u32];
_11 = '\u{af599}';
match _7 {
0 => bb12,
1 => bb13,
330987593888902159357307412338677758725 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
RET = _4;
_2 = [_7,_7,_7,_7,_7,_7];
_12 = _6 >= _6;
_9 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_2 = [_7,_7,_7,_7,_7,_7];
_9 = 9223372036854775807_isize >> _6;
_14 = RET as u16;
match _7 {
330987593888902159357307412338677758725 => bb10,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_3 = _1;
_2 = [_7,_7,_7,_7,_7,_7];
_1 = [_7,_7,_7,_7,_7,_7];
_15 = 7259919849709068824_u64 as f64;
_2 = _10;
RET = 29218_i16 as i32;
_1 = [_7,_7,_7,_7,_7,_7];
_3 = [_7,_7,_7,_7,_7,_7];
_4 = RET << _9;
_12 = _4 >= _4;
_12 = _8;
RET = -_4;
_5 = _12;
_11 = '\u{35cc4}';
_5 = !_8;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(8_usize, 4_usize, Move(_4), 6_usize, Move(_6), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(8_usize, 12_usize, Move(_12), 8_usize, Move(_8), 3_usize, Move(_3), 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: &'static f32,mut _2: u64,mut _3: u64,mut _4: [u128; 6],mut _5: u64,mut _6: u64,mut _7: u64,mut _8: u64,mut _9: u64,mut _10: u64,mut _11: [u128; 6]) -> u16 {
mir! {
type RET = u16;
let _12: f64;
let _13: (isize,);
let _14: i8;
let _15: u8;
let _16: [i64; 8];
let _17: [u8; 8];
let _18: &'static i8;
let _19: *const f64;
let _20: [char; 4];
let _21: &'static isize;
let _22: f64;
let _23: i16;
let _24: bool;
let _25: &'static (i64, i16, &'static &'static i8, u16);
let _26: (Adt41, *mut u128, &'static isize, u64);
let _27: (Adt41, *mut u128, &'static isize, u64);
let _28: [i64; 8];
let _29: [u16; 6];
let _30: ();
let _31: ();
{
RET = 25455_u16 & 12087_u16;
_2 = (-9223372036854775808_isize) as u64;
_11 = [237272720448370707825184969623021200817_u128,176193069468629601121294704378057548861_u128,107675809404515684125164286458727809250_u128,216996338237740911420851051142873995184_u128,148840638342024816751278606171497049416_u128,69704175814449508094543272286784057092_u128];
_8 = !_10;
_2 = 119_u8 as u64;
_6 = !_9;
_8 = _3 * _10;
RET = 240206475910965866523701456186062124382_u128 as u16;
_5 = _10 + _9;
_5 = _7 ^ _8;
RET = 62552_u16 | 54578_u16;
_4 = [272784791982229969038661194253101487573_u128,89517496763654774398617647186373053045_u128,204990451312463200081033755506111903223_u128,1253004159937832661983637734994736267_u128,279207759325308827565500586724121250897_u128,179711247883566329278576809253525853957_u128];
_12 = _8 as f64;
RET = 30243_u16 + 43956_u16;
_2 = _10 + _9;
_6 = !_7;
_3 = _7;
_3 = 2761680180040897112_i64 as u64;
RET = 5655_u16 | 35664_u16;
Call(RET = core::intrinsics::bswap(59111_u16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 19_i8 as u64;
_2 = _10 - _9;
RET = 17126_u16 ^ 20984_u16;
_9 = _12 as u64;
_8 = !_6;
_7 = _10 << _9;
_3 = _2;
_11 = _4;
_4 = _11;
_13 = (9223372036854775807_isize,);
_8 = (-104421533916647815397991133425084200375_i128) as u64;
_3 = _12 as u64;
_11 = _4;
RET = 30959_i16 as u16;
_4 = [121609072623287540915658566343963728900_u128,236409522492730908137860320210348719306_u128,329987451573911048626850131055571210288_u128,160627288483758028839900435962999867493_u128,81603478104228399786207389922513076389_u128,154288607528333160656751517610510271877_u128];
Goto(bb2)
}
bb2 = {
_15 = (-1152152550_i32) as u8;
RET = !24189_u16;
_7 = !_2;
_16 = [2261484116882241296_i64,7952199019605723378_i64,(-8224442763371828635_i64),8904959185459486351_i64,3602410187722719968_i64,(-3493061649770903468_i64),3442798239043310363_i64,7482144100363520921_i64];
_9 = _3;
RET = 32415_u16;
_8 = _3 | _9;
_7 = !_3;
_10 = _9 + _3;
_6 = 60_i8 as u64;
_11 = [202106350396970455510084178926735655811_u128,115637788567599864379905587654754523669_u128,327392787259039360056104718902628591633_u128,105371377183286716668289837577870987879_u128,155340961585947354557959265842129040398_u128,8508371829120533255361824389349041071_u128];
_9 = _10;
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _3;
_14 = (-33_i8) >> _10;
_13.0 = (-9223372036854775808_isize);
RET = 63407_u16;
_14 = 0_i8;
_19 = core::ptr::addr_of!(_12);
Goto(bb3)
}
bb3 = {
_15 = (*_19) as u8;
_2 = _10;
_2 = !_10;
_3 = _8;
_13 = ((-9223372036854775808_isize),);
_12 = (-1691_i16) as f64;
(*_19) = _13.0 as f64;
_6 = _8 - _9;
_16 = [4754608508765482627_i64,8383264142535790821_i64,(-1715019209994110910_i64),(-774164157459717006_i64),7146659799376357148_i64,(-224366264865063251_i64),6257930951149015782_i64,(-2891763790125235414_i64)];
_2 = RET as u64;
_13.0 = 9223372036854775807_isize * (-9223372036854775808_isize);
_10 = _9;
_11 = _4;
_19 = core::ptr::addr_of!(_12);
_5 = _6 << _9;
_13.0 = (-9223372036854775808_isize) + (-3_isize);
_21 = &_13.0;
_4 = [94567801839213592704716232280055742708_u128,34998465560146544538324562836424578887_u128,241613157164324423756832403514288282304_u128,149380560757774993821104778922068333943_u128,79023486207865871135633381132628340827_u128,281768472507599274897579123015056069984_u128];
_14 = (-2810_i16) as i8;
(*_19) = (-5743579794931178179_i64) as f64;
_12 = _13.0 as f64;
_9 = !_10;
_6 = !_9;
_13.0 = 80_isize;
_19 = core::ptr::addr_of!((*_19));
Goto(bb4)
}
bb4 = {
_12 = RET as f64;
_4 = _11;
Call(RET = fn10(_10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_21 = &_13.0;
_9 = !_6;
_18 = &_14;
_14 = -68_i8;
RET = (-120708826958915870359521045405745789466_i128) as u16;
_12 = (*_21) as f64;
_4 = [77137346706232527963611111206860903593_u128,29323445572249950004849494904193383614_u128,262336049456528284932546578715396846371_u128,228359975268760805876584015781022739599_u128,50958332592944986509043072789629597741_u128,254126540572534832469480022691685678374_u128];
match (*_21) {
0 => bb1,
1 => bb6,
80 => bb8,
_ => bb7
}
}
bb6 = {
_15 = (-1152152550_i32) as u8;
RET = !24189_u16;
_7 = !_2;
_16 = [2261484116882241296_i64,7952199019605723378_i64,(-8224442763371828635_i64),8904959185459486351_i64,3602410187722719968_i64,(-3493061649770903468_i64),3442798239043310363_i64,7482144100363520921_i64];
_9 = _3;
RET = 32415_u16;
_8 = _3 | _9;
_7 = !_3;
_10 = _9 + _3;
_6 = 60_i8 as u64;
_11 = [202106350396970455510084178926735655811_u128,115637788567599864379905587654754523669_u128,327392787259039360056104718902628591633_u128,105371377183286716668289837577870987879_u128,155340961585947354557959265842129040398_u128,8508371829120533255361824389349041071_u128];
_9 = _10;
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _3;
_14 = (-33_i8) >> _10;
_13.0 = (-9223372036854775808_isize);
RET = 63407_u16;
_14 = 0_i8;
_19 = core::ptr::addr_of!(_12);
Goto(bb3)
}
bb7 = {
_5 = 19_i8 as u64;
_2 = _10 - _9;
RET = 17126_u16 ^ 20984_u16;
_9 = _12 as u64;
_8 = !_6;
_7 = _10 << _9;
_3 = _2;
_11 = _4;
_4 = _11;
_13 = (9223372036854775807_isize,);
_8 = (-104421533916647815397991133425084200375_i128) as u64;
_3 = _12 as u64;
_11 = _4;
RET = 30959_i16 as u16;
_4 = [121609072623287540915658566343963728900_u128,236409522492730908137860320210348719306_u128,329987451573911048626850131055571210288_u128,160627288483758028839900435962999867493_u128,81603478104228399786207389922513076389_u128,154288607528333160656751517610510271877_u128];
Goto(bb2)
}
bb8 = {
_9 = 76467139991945326689810777092527188254_u128 as u64;
_27.2 = Move(_21);
_27.2 = &_13.0;
_20 = ['\u{20037}','\u{cad71}','\u{e469d}','\u{124a8}'];
_8 = _10 & _5;
_10 = _7 + _7;
_11 = [339980928328410722705976802555584109567_u128,77305608747369129872790156912534185496_u128,24728563009380596890327353559297637563_u128,50908015088341008404353570738661499024_u128,81004281315065528039390670397853074607_u128,115382608817935784095092941795525097972_u128];
_8 = _6 * _5;
_13.0 = -15_isize;
_13 = ((-9223372036854775808_isize),);
_12 = _15 as f64;
RET = 21020_u16;
_27.2 = &_13.0;
_28 = _16;
_26.2 = &_13.0;
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
match RET {
0 => bb4,
1 => bb5,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
21020 => bb15,
_ => bb14
}
}
bb9 = {
_5 = 19_i8 as u64;
_2 = _10 - _9;
RET = 17126_u16 ^ 20984_u16;
_9 = _12 as u64;
_8 = !_6;
_7 = _10 << _9;
_3 = _2;
_11 = _4;
_4 = _11;
_13 = (9223372036854775807_isize,);
_8 = (-104421533916647815397991133425084200375_i128) as u64;
_3 = _12 as u64;
_11 = _4;
RET = 30959_i16 as u16;
_4 = [121609072623287540915658566343963728900_u128,236409522492730908137860320210348719306_u128,329987451573911048626850131055571210288_u128,160627288483758028839900435962999867493_u128,81603478104228399786207389922513076389_u128,154288607528333160656751517610510271877_u128];
Goto(bb2)
}
bb10 = {
_5 = 19_i8 as u64;
_2 = _10 - _9;
RET = 17126_u16 ^ 20984_u16;
_9 = _12 as u64;
_8 = !_6;
_7 = _10 << _9;
_3 = _2;
_11 = _4;
_4 = _11;
_13 = (9223372036854775807_isize,);
_8 = (-104421533916647815397991133425084200375_i128) as u64;
_3 = _12 as u64;
_11 = _4;
RET = 30959_i16 as u16;
_4 = [121609072623287540915658566343963728900_u128,236409522492730908137860320210348719306_u128,329987451573911048626850131055571210288_u128,160627288483758028839900435962999867493_u128,81603478104228399786207389922513076389_u128,154288607528333160656751517610510271877_u128];
Goto(bb2)
}
bb11 = {
_21 = &_13.0;
_9 = !_6;
_18 = &_14;
_14 = -68_i8;
RET = (-120708826958915870359521045405745789466_i128) as u16;
_12 = (*_21) as f64;
_4 = [77137346706232527963611111206860903593_u128,29323445572249950004849494904193383614_u128,262336049456528284932546578715396846371_u128,228359975268760805876584015781022739599_u128,50958332592944986509043072789629597741_u128,254126540572534832469480022691685678374_u128];
match (*_21) {
0 => bb1,
1 => bb6,
80 => bb8,
_ => bb7
}
}
bb12 = {
_12 = RET as f64;
_4 = _11;
Call(RET = fn10(_10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_15 = (*_19) as u8;
_2 = _10;
_2 = !_10;
_3 = _8;
_13 = ((-9223372036854775808_isize),);
_12 = (-1691_i16) as f64;
(*_19) = _13.0 as f64;
_6 = _8 - _9;
_16 = [4754608508765482627_i64,8383264142535790821_i64,(-1715019209994110910_i64),(-774164157459717006_i64),7146659799376357148_i64,(-224366264865063251_i64),6257930951149015782_i64,(-2891763790125235414_i64)];
_2 = RET as u64;
_13.0 = 9223372036854775807_isize * (-9223372036854775808_isize);
_10 = _9;
_11 = _4;
_19 = core::ptr::addr_of!(_12);
_5 = _6 << _9;
_13.0 = (-9223372036854775808_isize) + (-3_isize);
_21 = &_13.0;
_4 = [94567801839213592704716232280055742708_u128,34998465560146544538324562836424578887_u128,241613157164324423756832403514288282304_u128,149380560757774993821104778922068333943_u128,79023486207865871135633381132628340827_u128,281768472507599274897579123015056069984_u128];
_14 = (-2810_i16) as i8;
(*_19) = (-5743579794931178179_i64) as f64;
_12 = _13.0 as f64;
_9 = !_10;
_6 = !_9;
_13.0 = 80_isize;
_19 = core::ptr::addr_of!((*_19));
Goto(bb4)
}
bb14 = {
_15 = (-1152152550_i32) as u8;
RET = !24189_u16;
_7 = !_2;
_16 = [2261484116882241296_i64,7952199019605723378_i64,(-8224442763371828635_i64),8904959185459486351_i64,3602410187722719968_i64,(-3493061649770903468_i64),3442798239043310363_i64,7482144100363520921_i64];
_9 = _3;
RET = 32415_u16;
_8 = _3 | _9;
_7 = !_3;
_10 = _9 + _3;
_6 = 60_i8 as u64;
_11 = [202106350396970455510084178926735655811_u128,115637788567599864379905587654754523669_u128,327392787259039360056104718902628591633_u128,105371377183286716668289837577870987879_u128,155340961585947354557959265842129040398_u128,8508371829120533255361824389349041071_u128];
_9 = _10;
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _3;
_14 = (-33_i8) >> _10;
_13.0 = (-9223372036854775808_isize);
RET = 63407_u16;
_14 = 0_i8;
_19 = core::ptr::addr_of!(_12);
Goto(bb3)
}
bb15 = {
_14 = !(-74_i8);
_26.3 = _6;
(*_19) = _15 as f64;
_29 = [RET,RET,RET,RET,RET,RET];
_14 = 68_i8 + (-84_i8);
_22 = -(*_19);
_14 = -(-62_i8);
RET = !36472_u16;
_28 = [(-4491805312574058750_i64),(-3598326176175519168_i64),(-3638880026973864697_i64),1663888980831564159_i64,(-4884511451666959195_i64),62717158096022050_i64,(-7867514125258243891_i64),(-3923651786855082009_i64)];
_15 = 11984863178403120956546442338105587057_i128 as u8;
_11 = [274702998018990184092683006766707076648_u128,280675911517568059952296877233163506164_u128,265936560492731644368974642338815927037_u128,46420263545614659224455939443399308566_u128,26479794724745968912215049011974672777_u128,39811642766990991116969887916422288595_u128];
_7 = _5;
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(9_usize, 17_usize, Move(_17), 13_usize, Move(_13), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(9_usize, 28_usize, Move(_28), 14_usize, Move(_14), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(9_usize, 5_usize, Move(_5), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u64,mut _2: u64) -> u16 {
mir! {
type RET = u16;
let _3: *const &'static f32;
let _4: &'static &'static u32;
let _5: isize;
let _6: [i8; 8];
let _7: (isize,);
let _8: &'static &'static *const i8;
let _9: u16;
let _10: bool;
let _11: [char; 5];
let _12: usize;
let _13: bool;
let _14: Adt41;
let _15: char;
let _16: [char; 2];
let _17: [u16; 5];
let _18: &'static i32;
let _19: bool;
let _20: f64;
let _21: Adt63;
let _22: *mut f32;
let _23: u16;
let _24: &'static (i64, i16, &'static &'static i8, u16);
let _25: *const &'static u32;
let _26: &'static isize;
let _27: f32;
let _28: &'static i8;
let _29: [bool; 6];
let _30: isize;
let _31: [u32; 3];
let _32: isize;
let _33: char;
let _34: Adt69;
let _35: [u16; 6];
let _36: [i8; 8];
let _37: isize;
let _38: ((char, *const i8),);
let _39: Adt63;
let _40: f64;
let _41: f32;
let _42: [char; 5];
let _43: ();
let _44: ();
{
_1 = _2;
RET = 62395_u16;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 35764_u16;
_2 = RET as u64;
RET = 46050_u16;
RET = 63489_u16;
_1 = !_2;
RET = 102_u8 as u16;
RET = 38100_u16 << _1;
_1 = !_2;
RET = !58956_u16;
_2 = 59_i8 as u64;
_2 = !_1;
RET = 64757_u16 | 41037_u16;
_2 = !_1;
RET = 47427_u16;
_2 = !_1;
_1 = 100_u8 as u64;
_2 = '\u{5f9cb}' as u64;
RET = 213620957291360985937526261772335923906_u128 as u16;
RET = !39261_u16;
_2 = _1 & _1;
_1 = _2 - _2;
Goto(bb2)
}
bb2 = {
_2 = _1;
_6 = [90_i8,(-81_i8),30_i8,(-65_i8),33_i8,(-11_i8),(-4_i8),(-57_i8)];
_1 = _2;
_2 = !_1;
_2 = _1;
_6 = [(-110_i8),(-116_i8),98_i8,121_i8,(-125_i8),15_i8,49_i8,30_i8];
_5 = !(-36_isize);
_6 = [(-8_i8),(-100_i8),(-65_i8),3_i8,(-77_i8),(-42_i8),40_i8,60_i8];
RET = 60824_u16;
_2 = RET as u64;
_5 = 9223372036854775807_isize | (-53_isize);
Goto(bb3)
}
bb3 = {
_7 = (_5,);
RET = !52540_u16;
_2 = _1;
_9 = RET;
Call(_10 = fn11(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = !_1;
RET = 1697367735_u32 as u16;
_5 = _7.0;
_5 = '\u{cf530}' as isize;
_6 = [106_i8,(-67_i8),(-10_i8),84_i8,73_i8,(-45_i8),84_i8,(-74_i8)];
_5 = _7.0;
_7.0 = _5 - _5;
_7.0 = -_5;
_7 = (_5,);
_1 = (-39561616274258802473803298204555509307_i128) as u64;
RET = 4184411513_u32 as u16;
_10 = _5 == _5;
_1 = _2;
_12 = 1_usize ^ 12104044242879765431_usize;
Goto(bb5)
}
bb5 = {
_2 = _1;
_11 = ['\u{a2c15}','\u{91ee9}','\u{98be4}','\u{a46cd}','\u{b3b81}'];
_15 = '\u{98fbd}';
_16 = [_15,_15];
_2 = _1 >> RET;
_9 = RET;
_5 = !_7.0;
_6 = [80_i8,8_i8,(-91_i8),(-109_i8),(-72_i8),115_i8,24_i8,0_i8];
_15 = '\u{442a3}';
_9 = RET;
RET = _9;
RET = 109617021882833047782367541784275811059_u128 as u16;
RET = _9 | _9;
RET = !_9;
_7 = (_5,);
_7.0 = -_5;
_6 = [28_i8,111_i8,47_i8,109_i8,(-15_i8),7_i8,(-99_i8),(-126_i8)];
_7 = (_5,);
_2 = _1;
_10 = !false;
RET = !_9;
_5 = 5233151800494207954_i64 as isize;
RET = _9 << _2;
_13 = RET >= _9;
RET = _9;
_7 = (_5,);
_1 = _2;
_12 = _15 as usize;
_6 = [34_i8,114_i8,54_i8,87_i8,(-89_i8),(-23_i8),0_i8,49_i8];
Goto(bb6)
}
bb6 = {
RET = !_9;
_7 = (_5,);
_15 = '\u{1f6c4}';
RET = 19277_i16 as u16;
_17 = [_9,_9,RET,RET,RET];
_2 = _1 ^ _1;
_5 = _7.0 | _7.0;
RET = _9;
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _1 };
place!(Field::<[u16; 5]>(Variant(_14, 2), 0)) = [_9,RET,_9,_9,_9];
_6 = [94_i8,106_i8,(-60_i8),2_i8,(-97_i8),98_i8,(-96_i8),61_i8];
_7.0 = _5 + _5;
_11 = [Field::<char>(Variant(_14, 2), 1),_15,Field::<char>(Variant(_14, 2), 1),_15,_15];
_1 = !_2;
_7 = (_5,);
_19 = _2 < _2;
_6 = [83_i8,(-38_i8),(-31_i8),(-64_i8),93_i8,(-77_i8),(-31_i8),43_i8];
SetDiscriminant(_14, 1);
_1 = (-632214595_i32) as u64;
Goto(bb7)
}
bb7 = {
_12 = !7548694333768622946_usize;
place!(Field::<i128>(Variant(_14, 1), 1)) = (-33119682232293921200685221706976469003_i128) ^ (-143495050053562228576899044870439290288_i128);
RET = _9 * _9;
_2 = !_1;
_2 = (-70_i8) as u64;
place!(Field::<f32>(Variant(_14, 1), 3)) = 98_u8 as f32;
_9 = RET;
_9 = Field::<f32>(Variant(_14, 1), 3) as u16;
place!(Field::<*const f64>(Variant(_14, 1), 0)) = core::ptr::addr_of!(_20);
_15 = '\u{bc077}';
_16 = [_15,_15];
_1 = _13 as u64;
_17 = [RET,RET,_9,RET,RET];
_15 = '\u{71ab4}';
_22 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_14, 1), 3)));
_20 = (*_22) as f64;
_11 = [_15,_15,_15,_15,_15];
_20 = (-1802_i16) as f64;
_23 = !RET;
(*_22) = 47_u8 as f32;
_1 = !_2;
_1 = _2;
place!(Field::<i128>(Variant(_14, 1), 1)) = RET as i128;
(*_22) = _7.0 as f32;
place!(Field::<*const f64>(Variant(_14, 1), 0)) = core::ptr::addr_of!(_20);
_22 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_14, 1), 3)));
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _1 };
Goto(bb8)
}
bb8 = {
_16 = [_15,Field::<char>(Variant(_14, 2), 1)];
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _2 };
_27 = 207_u8 as f32;
_16 = [Field::<char>(Variant(_14, 2), 1),Field::<char>(Variant(_14, 2), 1)];
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _2 };
_2 = _1;
_9 = _23;
place!(Field::<[u16; 5]>(Variant(_14, 2), 0)) = _17;
_9 = !RET;
_23 = !RET;
_29 = [_19,_19,_19,_19,_19,_19];
_26 = &_5;
_26 = &_5;
_7.0 = _19 as isize;
_27 = 17820_i16 as f32;
Goto(bb9)
}
bb9 = {
_1 = !Field::<u64>(Variant(_14, 2), 2);
_10 = !_19;
place!(Field::<[u16; 5]>(Variant(_14, 2), 0)) = _17;
_31 = [712971357_u32,1834196942_u32,1260754160_u32];
_30 = (*_26) & (*_26);
_23 = 41_u8 as u16;
Call(_20 = core::intrinsics::transmute(Field::<u64>(Variant(_14, 2), 2)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_13 = _19 ^ _19;
_33 = _15;
_31 = [1560500263_u32,343518255_u32,817429385_u32];
place!(Field::<char>(Variant(_14, 2), 1)) = _15;
_29 = [_19,_10,_13,_13,_10,_10];
_26 = &_7.0;
_14 = Adt41::Variant2 { fld0: _17,fld1: _33,fld2: _1 };
RET = _20 as u16;
_5 = -(*_26);
_23 = _33 as u16;
_9 = !RET;
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _2 };
_16 = [_15,_33];
_15 = _33;
_15 = Field::<char>(Variant(_14, 2), 1);
_31 = [779097280_u32,236320468_u32,4081586095_u32];
RET = _23 & _23;
_33 = Field::<char>(Variant(_14, 2), 1);
_5 = _7.0;
_34.fld2 = !(-8891517777041001022_i64);
_26 = &_7.0;
_30 = (*_26) - _5;
_34.fld3 = [1050514168_i32,(-1125232570_i32),2127665441_i32];
place!(Field::<[u16; 5]>(Variant(_14, 2), 0)) = _17;
SetDiscriminant(_14, 0);
place!(Field::<*const [i8; 8]>(Variant(_14, 0), 0)) = core::ptr::addr_of!(_6);
place!(Field::<[u32; 3]>(Variant(_14, 0), 1)) = [3370762482_u32,3160776228_u32,1340061158_u32];
_32 = _30 - _7.0;
_11 = [_33,_33,_15,_33,_33];
Goto(bb11)
}
bb11 = {
_28 = &place!(Field::<i8>(Variant(_14, 0), 3));
_9 = !RET;
_32 = (*_26);
_26 = &_5;
_27 = _20 as f32;
_11 = [_33,_33,_15,_15,_15];
_34.fld0 = _2 ^ _1;
_35 = [_9,RET,RET,_23,_9,_9];
RET = _23;
_19 = !_13;
_23 = !RET;
_10 = _19;
_26 = &_32;
_20 = (-18996_i16) as f64;
place!(Field::<i8>(Variant(_14, 0), 3)) = !27_i8;
_34.fld2 = (-121907141410591244_i64);
_22 = core::ptr::addr_of_mut!(_27);
_11 = [_15,_15,_15,_33,_15];
_31 = [4286476435_u32,750367207_u32,923709878_u32];
match _34.fld2 {
0 => bb12,
1 => bb13,
340282366920938463463252700290357620212 => bb15,
_ => bb14
}
}
bb12 = {
_12 = !7548694333768622946_usize;
place!(Field::<i128>(Variant(_14, 1), 1)) = (-33119682232293921200685221706976469003_i128) ^ (-143495050053562228576899044870439290288_i128);
RET = _9 * _9;
_2 = !_1;
_2 = (-70_i8) as u64;
place!(Field::<f32>(Variant(_14, 1), 3)) = 98_u8 as f32;
_9 = RET;
_9 = Field::<f32>(Variant(_14, 1), 3) as u16;
place!(Field::<*const f64>(Variant(_14, 1), 0)) = core::ptr::addr_of!(_20);
_15 = '\u{bc077}';
_16 = [_15,_15];
_1 = _13 as u64;
_17 = [RET,RET,_9,RET,RET];
_15 = '\u{71ab4}';
_22 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_14, 1), 3)));
_20 = (*_22) as f64;
_11 = [_15,_15,_15,_15,_15];
_20 = (-1802_i16) as f64;
_23 = !RET;
(*_22) = 47_u8 as f32;
_1 = !_2;
_1 = _2;
place!(Field::<i128>(Variant(_14, 1), 1)) = RET as i128;
(*_22) = _7.0 as f32;
place!(Field::<*const f64>(Variant(_14, 1), 0)) = core::ptr::addr_of!(_20);
_22 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_14, 1), 3)));
_14 = Adt41::Variant2 { fld0: _17,fld1: _15,fld2: _1 };
Goto(bb8)
}
bb13 = {
_2 = !_1;
RET = 1697367735_u32 as u16;
_5 = _7.0;
_5 = '\u{cf530}' as isize;
_6 = [106_i8,(-67_i8),(-10_i8),84_i8,73_i8,(-45_i8),84_i8,(-74_i8)];
_5 = _7.0;
_7.0 = _5 - _5;
_7.0 = -_5;
_7 = (_5,);
_1 = (-39561616274258802473803298204555509307_i128) as u64;
RET = 4184411513_u32 as u16;
_10 = _5 == _5;
_1 = _2;
_12 = 1_usize ^ 12104044242879765431_usize;
Goto(bb5)
}
bb14 = {
RET = 35764_u16;
_2 = RET as u64;
RET = 46050_u16;
RET = 63489_u16;
_1 = !_2;
RET = 102_u8 as u16;
RET = 38100_u16 << _1;
_1 = !_2;
RET = !58956_u16;
_2 = 59_i8 as u64;
_2 = !_1;
RET = 64757_u16 | 41037_u16;
_2 = !_1;
RET = 47427_u16;
_2 = !_1;
_1 = 100_u8 as u64;
_2 = '\u{5f9cb}' as u64;
RET = 213620957291360985937526261772335923906_u128 as u16;
RET = !39261_u16;
_2 = _1 & _1;
_1 = _2 - _2;
Goto(bb2)
}
bb15 = {
_7 = (_30,);
_33 = _15;
_28 = &place!(Field::<i8>(Variant(_14, 0), 3));
RET = _9 >> _32;
_42 = _11;
_20 = (*_26) as f64;
_36 = [(*_28),(*_28),(*_28),(*_28),(*_28),(*_28),Field::<i8>(Variant(_14, 0), 3),(*_28)];
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(10_usize, 36_usize, Move(_36), 6_usize, Move(_6), 42_usize, Move(_42), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 30_usize, Move(_30), 16_usize, Move(_16), 1_usize, Move(_1), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(10_usize, 9_usize, Move(_9), 5_usize, Move(_5), 10_usize, Move(_10), 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11() -> bool {
mir! {
type RET = bool;
let _1: [i32; 8];
let _2: bool;
let _3: f64;
let _4: *const [i8; 8];
let _5: [char; 5];
let _6: &'static u16;
let _7: isize;
let _8: &'static i32;
let _9: f32;
let _10: i128;
let _11: u128;
let _12: i8;
let _13: [i8; 6];
let _14: ();
let _15: ();
{
RET = 107_i8 > 114_i8;
_1 = [49972525_i32,(-1820545308_i32),(-563145497_i32),(-958957138_i32),908784835_i32,1362740562_i32,(-1712168517_i32),827448211_i32];
RET = true ^ false;
RET = false ^ true;
RET = !true;
RET = 197817206492285638159057944117406764943_u128 != 98510553683730667733171596298471153322_u128;
_1 = [(-1531345751_i32),(-175744075_i32),256309583_i32,(-384737321_i32),1346625991_i32,(-1220270162_i32),327447539_i32,(-531958897_i32)];
_1 = [1901301010_i32,343046961_i32,(-205523966_i32),(-435977534_i32),1679959444_i32,(-855527558_i32),(-1388957954_i32),(-212523424_i32)];
_1 = [(-1771552091_i32),1892501548_i32,(-587555717_i32),1756620723_i32,673105728_i32,(-1161275590_i32),716927514_i32,(-1890178020_i32)];
RET = true == true;
RET = 8665003994157974943_i64 <= (-5852189423687089593_i64);
RET = !false;
_2 = !RET;
RET = _2;
_1 = [1043275665_i32,(-486966324_i32),(-248996952_i32),(-406502752_i32),68773945_i32,1070519687_i32,2068085148_i32,1758409174_i32];
_3 = 1303802225_i32 as f64;
_2 = RET;
_3 = (-17_i8) as f64;
RET = !_2;
_1 = [1560368560_i32,(-784361099_i32),727923428_i32,204598560_i32,(-144743698_i32),1602670584_i32,1465611406_i32,(-669215645_i32)];
RET = _3 == _3;
_5 = ['\u{51dbc}','\u{1e3c}','\u{360b0}','\u{92da6}','\u{e32fb}'];
_5 = ['\u{d800f}','\u{b7562}','\u{b620b}','\u{100b26}','\u{3f588}'];
Goto(bb1)
}
bb1 = {
RET = !_2;
_1 = [(-315436661_i32),(-214277324_i32),1721931508_i32,1118002332_i32,(-640910807_i32),(-477211467_i32),(-1852616159_i32),902752387_i32];
_7 = (-158700082847081548792726455086544051046_i128) as isize;
_3 = 126861903438743140560255256763039732261_i128 as f64;
RET = _3 != _3;
_5 = ['\u{94e04}','\u{b8ce1}','\u{64167}','\u{aec06}','\u{df53e}'];
RET = _2;
RET = !_2;
_7 = _2 as isize;
_5 = ['\u{e912d}','\u{8eaf3}','\u{41407}','\u{7b889}','\u{eb1a2}'];
_2 = !RET;
_2 = RET ^ RET;
Call(_5 = fn12(_2, _7, _1, RET, RET, _1, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = [(-1114567790_i32),940172583_i32,1886026840_i32,942276197_i32,(-1350902862_i32),1152495276_i32,1571639742_i32,917504620_i32];
_7 = 9223372036854775807_isize * 23_isize;
_3 = _7 as f64;
_7 = !(-9223372036854775808_isize);
_7 = 18_isize >> 376842369_u32;
RET = _3 < _3;
_5 = ['\u{e7d2a}','\u{9365a}','\u{5ec7f}','\u{add35}','\u{1049e7}'];
_5 = ['\u{a9364}','\u{e4a66}','\u{2e4b5}','\u{fe6a2}','\u{3bb21}'];
_1 = [(-1031976727_i32),(-2085193239_i32),1005165847_i32,(-1298633043_i32),(-1430225447_i32),1299008928_i32,(-343120863_i32),(-1628514748_i32)];
RET = _2 < _2;
_3 = 14074_u16 as f64;
_7 = -9223372036854775807_isize;
Call(_2 = fn13(_1, _5, _1, _5, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = ['\u{1069fa}','\u{facb1}','\u{cc791}','\u{823d1}','\u{4acad}'];
_7 = -(-9223372036854775808_isize);
_7 = 9223372036854775807_isize;
_7 = (-1188875229_i32) as isize;
_9 = 62165_u16 as f32;
_9 = (-113_i8) as f32;
_7 = 9223372036854775807_isize * (-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
RET = _2;
_1 = [(-1606137794_i32),(-1586051862_i32),(-6178915_i32),2071517696_i32,1351639088_i32,(-2115568574_i32),(-1756582164_i32),1847315517_i32];
_10 = (-6662769478273658313393162143414041169_i128);
_9 = _3 as f32;
Goto(bb5)
}
bb5 = {
Call(_14 = dump_var(11_usize, 1_usize, Move(_1), 5_usize, Move(_5), 15_usize, _15, 15_usize, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: bool,mut _2: isize,mut _3: [i32; 8],mut _4: bool,mut _5: bool,mut _6: [i32; 8],mut _7: bool,mut _8: [i32; 8]) -> [char; 5] {
mir! {
type RET = [char; 5];
let _9: bool;
let _10: *mut f32;
let _11: &'static *const *mut u128;
let _12: isize;
let _13: isize;
let _14: *const [i8; 8];
let _15: *mut Adt45;
let _16: i8;
let _17: f64;
let _18: bool;
let _19: isize;
let _20: [u32; 3];
let _21: ([u32; 3], char);
let _22: [u128; 6];
let _23: [u8; 8];
let _24: u8;
let _25: u128;
let _26: u128;
let _27: [u64; 2];
let _28: (i64, i16, &'static &'static i8, u16);
let _29: bool;
let _30: Adt63;
let _31: Adt69;
let _32: ();
let _33: ();
{
RET = ['\u{8aaf1}','\u{dd902}','\u{3b84f}','\u{9244e}','\u{b869a}'];
_3 = [220332261_i32,398293205_i32,1421310811_i32,1051750088_i32,1143916454_i32,367141717_i32,205133799_i32,635746788_i32];
_5 = _7 | _1;
_6 = _8;
RET = ['\u{bac47}','\u{a0756}','\u{46174}','\u{d40f5}','\u{5eef1}'];
_6 = _8;
_2 = 9223372036854775807_isize | (-4_isize);
_3 = [(-635125763_i32),(-1110764955_i32),(-1909954888_i32),815454013_i32,(-1598751077_i32),(-14384000_i32),1829057557_i32,115724923_i32];
_3 = [1569502374_i32,600242815_i32,279964242_i32,(-543365604_i32),1990006657_i32,11979425_i32,(-1413609431_i32),(-1023938736_i32)];
RET = ['\u{b10af}','\u{3d673}','\u{1051e3}','\u{51cc2}','\u{df4e6}'];
RET = ['\u{537f1}','\u{7ae92}','\u{4ef85}','\u{854e3}','\u{7d7fc}'];
_8 = [215540645_i32,(-478022045_i32),109685079_i32,466465725_i32,(-1272966792_i32),(-2120797825_i32),357018594_i32,2118341727_i32];
_12 = (-1047453945539499296_i64) as isize;
_3 = _8;
Goto(bb1)
}
bb1 = {
_4 = _5;
_6 = [1471366329_i32,(-775241688_i32),(-1746275041_i32),(-739153378_i32),901620886_i32,(-2144950807_i32),215589056_i32,1630647509_i32];
RET = ['\u{14119}','\u{1880a}','\u{fd6f7}','\u{75afb}','\u{e88a6}'];
_12 = _2;
_9 = !_4;
_7 = _9;
_9 = _5;
_2 = _12;
RET = ['\u{7935a}','\u{f51a}','\u{f046d}','\u{6a912}','\u{7c43e}'];
_5 = !_7;
_6 = [32966028_i32,341714649_i32,1130803443_i32,(-1207563920_i32),1946726486_i32,1294895882_i32,(-1494420280_i32),(-1779933044_i32)];
_7 = !_9;
_6 = _8;
_6 = [(-852548240_i32),1398588936_i32,(-1511498489_i32),990707512_i32,1325984356_i32,(-1447383834_i32),1721612665_i32,(-1912023546_i32)];
_7 = !_9;
_12 = _2;
_13 = !_12;
_9 = !_4;
_13 = (-126036298752657005116145584165634925197_i128) as isize;
_8 = _3;
Goto(bb2)
}
bb2 = {
_17 = 850311011_u32 as f64;
_16 = (-95_i8);
match _16 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211361 => bb8,
_ => bb7
}
}
bb3 = {
_4 = _5;
_6 = [1471366329_i32,(-775241688_i32),(-1746275041_i32),(-739153378_i32),901620886_i32,(-2144950807_i32),215589056_i32,1630647509_i32];
RET = ['\u{14119}','\u{1880a}','\u{fd6f7}','\u{75afb}','\u{e88a6}'];
_12 = _2;
_9 = !_4;
_7 = _9;
_9 = _5;
_2 = _12;
RET = ['\u{7935a}','\u{f51a}','\u{f046d}','\u{6a912}','\u{7c43e}'];
_5 = !_7;
_6 = [32966028_i32,341714649_i32,1130803443_i32,(-1207563920_i32),1946726486_i32,1294895882_i32,(-1494420280_i32),(-1779933044_i32)];
_7 = !_9;
_6 = _8;
_6 = [(-852548240_i32),1398588936_i32,(-1511498489_i32),990707512_i32,1325984356_i32,(-1447383834_i32),1721612665_i32,(-1912023546_i32)];
_7 = !_9;
_12 = _2;
_13 = !_12;
_9 = !_4;
_13 = (-126036298752657005116145584165634925197_i128) as isize;
_8 = _3;
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
_16 = (-111_i8);
Goto(bb9)
}
bb9 = {
_3 = _8;
_18 = !_5;
_13 = _12;
_6 = _3;
_6 = _3;
_8 = [355779402_i32,1667995533_i32,(-805653434_i32),1523418675_i32,(-1910821804_i32),912213684_i32,(-1491069770_i32),(-318115620_i32)];
_9 = _5 & _7;
Goto(bb10)
}
bb10 = {
_16 = -(-16_i8);
_2 = -_13;
_19 = _13;
_2 = !_19;
RET = ['\u{bf276}','\u{5265f}','\u{10f4a}','\u{388a1}','\u{cbc95}'];
_3 = [(-1768796758_i32),1878452319_i32,(-594661598_i32),(-154369320_i32),(-646646228_i32),49908525_i32,643239993_i32,242093667_i32];
_9 = _7 | _7;
Goto(bb11)
}
bb11 = {
_17 = 25294_i16 as f64;
_16 = (-33_i8);
_12 = _2 >> _16;
_13 = _12 | _12;
RET = ['\u{f3c6}','\u{5490e}','\u{8d088}','\u{11aaf}','\u{ff396}'];
_12 = _13;
_18 = _7 ^ _7;
_12 = _13 >> _2;
_17 = (-48659594554940670704312649959678403091_i128) as f64;
_13 = _2;
_16 = (-69_i8);
_6 = [420170358_i32,1341503963_i32,(-83721793_i32),708245836_i32,(-1763409587_i32),1382259077_i32,(-1648267377_i32),(-757083483_i32)];
_4 = !_1;
_20 = [2469170554_u32,1412636492_u32,3268031068_u32];
_4 = _9;
_7 = _4 & _5;
_12 = _2;
_20 = [2389923184_u32,3427905781_u32,799826747_u32];
_18 = _7;
_1 = _7 | _7;
_4 = !_1;
_12 = _19;
_12 = _2;
_21.1 = '\u{b2533}';
match _16 {
0 => bb8,
1 => bb3,
340282366920938463463374607431768211387 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_8 = _6;
_4 = _7 > _7;
_17 = (-138870207621893368244445380915797870464_i128) as f64;
_23 = [244_u8,91_u8,8_u8,89_u8,75_u8,101_u8,107_u8,137_u8];
_4 = _9;
_21.1 = '\u{358c}';
_24 = 228_u8;
_8 = _3;
_21.1 = '\u{c1dbb}';
_28.3 = _16 as u16;
_19 = _13;
_3 = _6;
match _24 {
228 => bb15,
_ => bb14
}
}
bb14 = {
_16 = -(-16_i8);
_2 = -_13;
_19 = _13;
_2 = !_19;
RET = ['\u{bf276}','\u{5265f}','\u{10f4a}','\u{388a1}','\u{cbc95}'];
_3 = [(-1768796758_i32),1878452319_i32,(-594661598_i32),(-154369320_i32),(-646646228_i32),49908525_i32,643239993_i32,242093667_i32];
_9 = _7 | _7;
Goto(bb11)
}
bb15 = {
_31.fld1 = 1339483184_i32 as usize;
_29 = _18;
_22 = [21580652795831134484622109249564069289_u128,13602197733899363698534506261975273465_u128,157974658547950970033790225816590247314_u128,118515684003742750051212767600439785841_u128,320515965844224661441012898717292060994_u128,196714823258825880355248081828350205520_u128];
_19 = !_13;
_27 = [17514358636068598367_u64,17177171042608981137_u64];
_26 = 312046377896008872758826230783173667785_u128;
_7 = !_18;
_4 = !_29;
_5 = _31.fld1 <= _31.fld1;
RET = [_21.1,_21.1,_21.1,_21.1,_21.1];
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(12_usize, 26_usize, Move(_26), 12_usize, Move(_12), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(12_usize, 4_usize, Move(_4), 2_usize, Move(_2), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(12_usize, 7_usize, Move(_7), 22_usize, Move(_22), 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i32; 8],mut _2: [char; 5],mut _3: [i32; 8],mut _4: [char; 5],mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: char;
let _7: [u64; 2];
let _8: isize;
let _9: i16;
let _10: &'static [u16; 6];
let _11: u8;
let _12: [u8; 8];
let _13: char;
let _14: ();
let _15: ();
{
_4 = ['\u{cf2f6}','\u{62bfb}','\u{9b1de}','\u{9692d}','\u{1002d3}'];
_2 = ['\u{b8d38}','\u{aaf0}','\u{6d36d}','\u{fb5e5}','\u{f78e5}'];
Goto(bb1)
}
bb1 = {
RET = !_5;
_4 = ['\u{104c7e}','\u{907f9}','\u{5886c}','\u{42a9b}','\u{ce288}'];
_5 = RET;
_4 = _2;
RET = _5 | _5;
_5 = !RET;
_6 = '\u{2abe4}';
_1 = [(-579202560_i32),175657096_i32,(-440730587_i32),1014158461_i32,(-566260447_i32),1840794201_i32,1974041591_i32,(-15843766_i32)];
RET = !_5;
Goto(bb2)
}
bb2 = {
RET = _5;
Call(RET = fn14(_4, _1, _1, _3, _5, _3, _5, _5, _2, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = !_5;
_4 = [_6,_6,_6,_6,_6];
_6 = '\u{8b548}';
_6 = '\u{f8484}';
_6 = '\u{414f4}';
RET = _5 >= _5;
_2 = [_6,_6,_6,_6,_6];
_7 = [9513766075367922006_u64,6662919079004016160_u64];
_9 = 106362319811256162130966606806328237890_u128 as i16;
_3 = [866762481_i32,484011547_i32,95156195_i32,(-1678709809_i32),724030992_i32,(-572834242_i32),206258514_i32,1992063909_i32];
_4 = [_6,_6,_6,_6,_6];
_8 = (-9223372036854775808_isize) - 96_isize;
_1 = _3;
_1 = _3;
_5 = RET > RET;
RET = _5 >= _5;
RET = _5;
_4 = [_6,_6,_6,_6,_6];
_11 = 103_u8;
_8 = !9223372036854775807_isize;
_1 = [(-1553511610_i32),(-148290681_i32),(-914911556_i32),(-787543493_i32),1510038593_i32,33282339_i32,1427225222_i32,(-827000109_i32)];
_8 = 9223372036854775807_isize;
_1 = [725894386_i32,1062992082_i32,(-327458710_i32),1223326425_i32,647337234_i32,(-1773589620_i32),189613948_i32,2093231824_i32];
_11 = 29_u8;
_2 = _4;
Goto(bb4)
}
bb4 = {
Call(_14 = dump_var(13_usize, 6_usize, Move(_6), 3_usize, Move(_3), 4_usize, Move(_4), 11_usize, Move(_11)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_14 = dump_var(13_usize, 7_usize, Move(_7), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [char; 5],mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8],mut _5: bool,mut _6: [i32; 8],mut _7: bool,mut _8: bool,mut _9: [char; 5],mut _10: [i32; 8],mut _11: [i32; 8]) -> bool {
mir! {
type RET = bool;
let _12: [u16; 6];
let _13: isize;
let _14: ();
let _15: ();
{
_7 = _5 != _8;
_3 = [1754397932_i32,418617071_i32,(-1363483182_i32),(-1039907258_i32),1835071754_i32,(-1845039556_i32),391474881_i32,1463320750_i32];
RET = !_8;
_4 = _11;
_3 = [1296780522_i32,1321132191_i32,413468741_i32,1408365605_i32,(-850541033_i32),699998379_i32,(-1820260409_i32),(-80240179_i32)];
_11 = [(-30572431_i32),(-683765337_i32),415152761_i32,629456612_i32,(-143912863_i32),(-1919189976_i32),(-924809760_i32),396609675_i32];
_7 = _8 == RET;
_4 = _2;
_9 = ['\u{10ef57}','\u{24c94}','\u{81d8b}','\u{d4849}','\u{f5786}'];
_4 = _6;
_2 = [1042708477_i32,(-1570168836_i32),646988583_i32,(-1821998064_i32),(-1580109173_i32),(-728647814_i32),2121927380_i32,(-2126233042_i32)];
_4 = _11;
_4 = [1922785638_i32,(-585524630_i32),(-111885138_i32),(-444463353_i32),240982383_i32,(-1535420028_i32),821701775_i32,(-411119928_i32)];
_3 = _2;
_8 = _5;
RET = _7 <= _7;
_8 = _7;
_5 = _8;
_13 = 3_u8 as isize;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(14_usize, 11_usize, Move(_11), 1_usize, Move(_1), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(14_usize, 8_usize, Move(_8), 10_usize, Move(_10), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static u32,mut _2: &'static f32,mut _3: u64,mut _4: isize,mut _5: u64,mut _6: [char; 2],mut _7: u64,mut _8: [u128; 6],mut _9: u64,mut _10: [i64; 8]) -> usize {
mir! {
type RET = usize;
let _11: [i32; 3];
let _12: u8;
let _13: [bool; 6];
let _14: f64;
let _15: &'static [u16; 6];
let _16: [isize; 3];
let _17: Adt69;
let _18: f64;
let _19: bool;
let _20: [i32; 8];
let _21: ();
let _22: ();
{
_4 = 125_isize ^ (-127_isize);
RET = 5_usize << _9;
_4 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_7 = _3 << _9;
RET = 2_usize;
match _10[RET] {
340282366920938463459943623227129256854 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3 = '\u{273fe}' as u64;
_10 = [5274907543648424455_i64,6496564529768584405_i64,(-4171590691785537927_i64),213746798095784764_i64,6100051109622465626_i64,(-2057859844425499230_i64),4655649334285446629_i64,6022012994380262735_i64];
RET = 7_usize + 4_usize;
_6 = ['\u{573e2}','\u{30f70}'];
_3 = 3206617331_u32 as u64;
_7 = 48_u8 as u64;
_7 = _5;
_6 = ['\u{9d9b2}','\u{b0239}'];
_7 = true as u64;
_7 = 24319_u16 as u64;
_5 = _9;
RET = 0_usize;
_6[RET] = '\u{ee8db}';
_11 = [(-1520956362_i32),(-324908607_i32),(-1533192288_i32)];
_12 = 65_u8 - 154_u8;
RET = 11812270678532438414_usize;
match RET {
0 => bb3,
1 => bb4,
11812270678532438414 => bb6,
_ => bb5
}
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
_7 = _5 + _5;
_6 = ['\u{eeac6}','\u{2209e}'];
_8 = [160072361872067140719577849366784814613_u128,285423046109252871714781837488988341248_u128,85398629233865708381215701006336339006_u128,168755889573314980997041785953998727363_u128,170365519625894018481429364453738817003_u128,309718880633830217375317798789557260173_u128];
_9 = _7;
_3 = _9 - _7;
RET = _7 as usize;
_4 = (-9223372036854775808_isize) + (-83_isize);
RET = 339761348267688237760201678605352292386_u128 as usize;
_5 = _9;
_3 = 36_i8 as u64;
_11 = [1579671373_i32,(-1266502130_i32),987013128_i32];
_12 = 131_u8;
_9 = _5 * _5;
RET = _4 as usize;
_13 = [false,true,false,false,true,false];
_3 = !_7;
_13 = [true,false,true,true,true,false];
_14 = _7 as f64;
_8 = [319322001896539932928015032251720866856_u128,88292264807053284345539426709282051422_u128,299982140810460334676374633751628967243_u128,212944091203773191304078964145231186803_u128,198519721166916951484918508899186598843_u128,236979144361844006121691989649467473352_u128];
_10 = [9215204804125490819_i64,(-8171836761219397100_i64),(-5488584422441163598_i64),1689827284567957921_i64,2470067091043069763_i64,8393390741714374051_i64,6751488154973200422_i64,8251590267376699125_i64];
RET = !6_usize;
_9 = _3;
_8 = [173880278054840098962312906654865028136_u128,111306394425866704585902102395968030371_u128,64640664985010763110282545661085316102_u128,315943226246043021599612583922802395134_u128,240556019680146095948693018985851213584_u128,287281792401891696494235296850231437745_u128];
_9 = !_3;
_10 = [460005319744231964_i64,(-2986251684853530962_i64),2179039344179559738_i64,1906803572037470661_i64,(-7502072947917555723_i64),(-3376529475674166219_i64),(-2492727272044784641_i64),(-1577822234176515406_i64)];
match _12 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
131 => bb12,
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
_3 = '\u{273fe}' as u64;
_10 = [5274907543648424455_i64,6496564529768584405_i64,(-4171590691785537927_i64),213746798095784764_i64,6100051109622465626_i64,(-2057859844425499230_i64),4655649334285446629_i64,6022012994380262735_i64];
RET = 7_usize + 4_usize;
_6 = ['\u{573e2}','\u{30f70}'];
_3 = 3206617331_u32 as u64;
_7 = 48_u8 as u64;
_7 = _5;
_6 = ['\u{9d9b2}','\u{b0239}'];
_7 = true as u64;
_7 = 24319_u16 as u64;
_5 = _9;
RET = 0_usize;
_6[RET] = '\u{ee8db}';
_11 = [(-1520956362_i32),(-324908607_i32),(-1533192288_i32)];
_12 = 65_u8 - 154_u8;
RET = 11812270678532438414_usize;
match RET {
0 => bb3,
1 => bb4,
11812270678532438414 => bb6,
_ => bb5
}
}
bb11 = {
Return()
}
bb12 = {
RET = 1909280451144678954_usize >> _7;
_7 = 1151984408_i32 as u64;
_6 = ['\u{10c6cc}','\u{55c35}'];
_8 = [103614607709205591238710738589464161657_u128,70486308259386749226502275837643151544_u128,318289380550571071586777339168395508116_u128,273343940316829783518079876597378354407_u128,98835981324247269922481914162687839483_u128,148390853197828274722639085824268183679_u128];
_8 = [204698078845570412267581250949054555177_u128,53508283706555028146254616892316460087_u128,241932758931837587121891039668134789416_u128,290549548116529875708612053533574387017_u128,122205381623596750509769585759112933401_u128,125479913032205558686905989527898405200_u128];
_11 = [(-1456286593_i32),(-1007395779_i32),49256847_i32];
_11 = [2032900623_i32,(-1228654108_i32),(-217602038_i32)];
RET = 11898700503518802601_usize >> _3;
_6 = ['\u{7c218}','\u{4001e}'];
RET = 2659139950287016243_usize | 9904103804159822777_usize;
_5 = _3;
_16 = [_4,_4,_4];
_10 = [7550191753062676842_i64,(-526842112737656146_i64),(-7620732948837157134_i64),3363895821350685213_i64,2457297506734972425_i64,(-5811064907284171137_i64),126937618435002170_i64,7524768622545372691_i64];
_5 = !_9;
_9 = !_5;
_13 = [true,false,true,false,true,false];
_3 = 34032_u16 as u64;
_11 = [61273557_i32,(-1760841543_i32),819692677_i32];
_16 = [_4,_4,_4];
_5 = !_9;
_17 = Adt69 { fld0: _9,fld1: RET,fld2: (-1413720859858291762_i64),fld3: _11 };
RET = !_17.fld1;
_17.fld0 = 13407_i16 as u64;
_19 = true;
_4 = 9223372036854775807_isize;
match _17.fld2 {
0 => bb9,
340282366920938463461960886571909919694 => bb13,
_ => bb8
}
}
bb13 = {
_10 = [_17.fld2,_17.fld2,_17.fld2,_17.fld2,_17.fld2,_17.fld2,_17.fld2,_17.fld2];
RET = !_17.fld1;
Goto(bb14)
}
bb14 = {
_14 = 3278822954_u32 as f64;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(15_usize, 16_usize, Move(_16), 13_usize, Move(_13), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(15_usize, 9_usize, Move(_9), 11_usize, Move(_11), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i64; 8],mut _2: isize,mut _3: isize,mut _4: isize) -> [i64; 8] {
mir! {
type RET = [i64; 8];
let _5: usize;
let _6: f32;
let _7: [isize; 3];
let _8: [i32; 8];
let _9: *const f64;
let _10: [u128; 6];
let _11: f64;
let _12: Adt63;
let _13: isize;
let _14: bool;
let _15: [i8; 2];
let _16: [i32; 8];
let _17: &'static (char, *const i8);
let _18: (*mut usize, (i64, i16, &'static &'static i8, u16));
let _19: Adt74;
let _20: i8;
let _21: isize;
let _22: isize;
let _23: f64;
let _24: u128;
let _25: char;
let _26: [bool; 6];
let _27: &'static *const i8;
let _28: isize;
let _29: Adt74;
let _30: [bool; 6];
let _31: &'static [u16; 6];
let _32: [u16; 6];
let _33: bool;
let _34: &'static &'static i8;
let _35: [u8; 8];
let _36: &'static [u16; 6];
let _37: [i32; 3];
let _38: f32;
let _39: ();
let _40: ();
{
_1 = [(-5585690348591688961_i64),2302008060772661940_i64,3487859597251488856_i64,6298613213609864910_i64,2677701640220197947_i64,6388074583606609628_i64,(-8494590090375347904_i64),8381276083367744441_i64];
RET = [3021018531946190130_i64,6460254159463819390_i64,(-6496388339986788756_i64),41636018621265374_i64,4269905991107778942_i64,(-6796268336392626025_i64),(-8868662390517990678_i64),9066051734962321569_i64];
_3 = _2;
_4 = _3;
_1 = RET;
RET = _1;
_4 = 16031_u16 as isize;
RET = [2716542970446741109_i64,7956473438988853496_i64,(-8436447181627500254_i64),6914321369909624214_i64,(-2986385116774039766_i64),(-6299371675988222625_i64),(-309355466967661016_i64),(-3320115984031122119_i64)];
RET = [1495992438789635730_i64,6891253944596454168_i64,(-1934268724650644831_i64),(-6998784481669768733_i64),(-1319371712643606199_i64),(-3078270888109588012_i64),9179353634903656929_i64,(-8405948326006567451_i64)];
_4 = !_3;
RET = _1;
_5 = (-5849127250011680902556342569128084470_i128) as usize;
RET = _1;
_3 = -_4;
_10 = [338346342331211532865069207526913465099_u128,214087218200770500580210716371778217412_u128,177065808458926891853674734190009025817_u128,308576486570841149927250107969769441547_u128,73369355191614000595201920447840479526_u128,123634463858214358245724989600959442316_u128];
RET = _1;
_9 = core::ptr::addr_of!(_11);
(*_9) = 31146_u16 as f64;
_8 = [(-366078990_i32),(-1020281743_i32),(-1041007306_i32),(-1853875636_i32),1994166682_i32,(-1835916351_i32),511105252_i32,574577322_i32];
Goto(bb1)
}
bb1 = {
_3 = !_4;
_6 = 24470013933006852928169004062288519106_u128 as f32;
_8 = [1114623215_i32,1484158962_i32,179014881_i32,1068865814_i32,1943123875_i32,809929224_i32,(-58849326_i32),1322395940_i32];
_13 = 98505981370083545317378511192501512568_i128 as isize;
_8 = [(-1228502779_i32),(-1707607114_i32),961049654_i32,69010763_i32,(-536991785_i32),(-448106838_i32),1731593335_i32,189738287_i32];
(*_9) = (-71726689791802198655303424788902806020_i128) as f64;
_6 = 77747137145880383287448887934244147227_u128 as f32;
_5 = !9576342155158037694_usize;
_15 = [106_i8,72_i8];
_4 = _3;
_16 = [1667943312_i32,(-1400882776_i32),2120752474_i32,1374270948_i32,1644508895_i32,(-2088046166_i32),(-266554411_i32),(-276106695_i32)];
_2 = 34_u8 as isize;
_3 = _13;
_15 = [(-34_i8),(-56_i8)];
_8 = _16;
_15 = [103_i8,(-59_i8)];
_16 = [(-1455112938_i32),(-1180639044_i32),1133600138_i32,1839045038_i32,(-1344617830_i32),(-385493361_i32),266400058_i32,1014679464_i32];
_8 = _16;
_16 = [(-1072507967_i32),(-848686536_i32),194551068_i32,336462484_i32,1226311556_i32,(-265910716_i32),(-1166850327_i32),(-887655651_i32)];
_14 = false;
(*_9) = 271938738_u32 as f64;
_8 = _16;
_1 = [3254475224995377623_i64,4348100409497665850_i64,4795739283362198956_i64,(-4804404311080882710_i64),230658373638262490_i64,1593627167758633932_i64,(-3179162327326711085_i64),(-585567628415761380_i64)];
_9 = core::ptr::addr_of!((*_9));
_2 = _3 << _4;
_4 = _2;
Goto(bb2)
}
bb2 = {
_2 = !_13;
(*_9) = 13662924151225166046_u64 as f64;
(*_9) = (-113636630954495444075690156951540806917_i128) as f64;
_5 = 7_usize;
_4 = !_13;
_13 = _4 & _2;
_5 = 54_i8 as usize;
RET = [(-5118027561373642862_i64),(-3233038326920651955_i64),(-5829490634873286912_i64),(-2122389619590119406_i64),(-5916704034533745568_i64),(-8742003711753277258_i64),(-4358100944240014005_i64),1401103796657258761_i64];
_18.1.1 = 14165_i16 - (-29641_i16);
_8 = [(-335494612_i32),1675597667_i32,395101644_i32,950470074_i32,(-429946619_i32),1341358446_i32,(-389380233_i32),(-1051236798_i32)];
Goto(bb3)
}
bb3 = {
_14 = !false;
_20 = -(-89_i8);
_10 = [222994959458854335049714257461493083187_u128,109608298853836793356180978472938211167_u128,45501807054229489113468678542507792350_u128,5609428458659572140407000179465191849_u128,134552052863918736278057786616583382744_u128,113507438382275055188235302169870971809_u128];
(*_9) = 107739479110461759546118245723633495156_i128 as f64;
Goto(bb4)
}
bb4 = {
_4 = 45999_u16 as isize;
_18.1.0 = 4698956393961314345_i64 + (-1792740676196881861_i64);
_4 = 1121839681_i32 as isize;
_14 = false;
_1 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_11 = 3482590584_u32 as f64;
Call(_20 = core::intrinsics::transmute(_14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_9) = _20 as f64;
_16 = _8;
_19 = Adt74::Variant2 { fld0: 98652970888477634795087564508729484815_u128 };
_4 = _13;
_15 = [_20,_20];
Call(_13 = fn17(RET, _10, _5, _4, _10, _16, _8, _4, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = 20314_u16 as isize;
Call(_3 = fn18(_13, _18.1.0, _13, _18.1.0, _13, _13, _13, _13, _13, _4, _16), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = (*_9) as f32;
_10 = [224066923934503120142171216987009218015_u128,285793102239358162630033639744684572337_u128,47458264103593023769692543744415153151_u128,172437580837872299546362993893650947380_u128,31974471849403960055942469983017946687_u128,209026118840425401321955650987490164420_u128];
_14 = true;
(*_9) = _18.1.0 as f64;
_14 = false;
_4 = _20 as isize;
_22 = 254751103536494348305318887345488988866_u128 as isize;
_1 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
place!(Field::<u128>(Variant(_19, 2), 0)) = !181877731306306458979379343592748724086_u128;
(*_9) = Field::<u128>(Variant(_19, 2), 0) as f64;
_18.0 = core::ptr::addr_of_mut!(_5);
(*_9) = (-1689720498_i32) as f64;
_11 = _18.1.0 as f64;
_8 = _16;
RET = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_6 = _3 as f32;
_6 = _18.1.0 as f32;
_9 = core::ptr::addr_of!(_11);
(*_9) = Field::<u128>(Variant(_19, 2), 0) as f64;
_22 = (-94361661600539773244674804239057623255_i128) as isize;
_20 = (-60_i8) + 28_i8;
_16 = _8;
_5 = !8290838253541518567_usize;
SetDiscriminant(_19, 1);
_23 = (*_9) * _11;
place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0.1 = core::ptr::addr_of!(_20);
_18.1.0 = 1855665648365438935_i64 ^ 1312225931135461122_i64;
Goto(bb8)
}
bb8 = {
_17 = &place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0;
_8 = [(-1342654701_i32),(-1580264245_i32),602222577_i32,821813390_i32,(-1127606283_i32),(-1557205320_i32),1549632912_i32,(-1770405166_i32)];
_23 = _11 * (*_9);
Goto(bb9)
}
bb9 = {
_22 = _3;
_25 = '\u{ff0da}';
_24 = !63684149434912788137142752109309700396_u128;
_26 = [_14,_14,_14,_14,_14,_14];
_9 = core::ptr::addr_of!((*_9));
_5 = !14390311754877185820_usize;
_23 = -_11;
_16 = _8;
_18.1.3 = 33167_u16;
place!(Field::<i64>(Variant(_19, 1), 6)) = _24 as i64;
_20 = !(-120_i8);
_2 = -_22;
_18.0 = core::ptr::addr_of_mut!(_5);
place!(Field::<i64>(Variant(_19, 1), 6)) = _18.1.0 & _18.1.0;
_9 = core::ptr::addr_of!(_23);
_22 = _3;
place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0.1 = core::ptr::addr_of!(_20);
place!(Field::<Adt35>(Variant(_19, 1), 4)) = Adt35::Variant0 { fld0: _5,fld1: 177_u8,fld2: _18.1.3,fld3: (*_9),fld4: _16,fld5: 1527992950_u32,fld6: _18.1.0,fld7: (-118965576648566432216817796326049219024_i128) };
place!(Field::<Adt35>(Variant(_19, 1), 4)) = Adt35::Variant0 { fld0: _5,fld1: 138_u8,fld2: _18.1.3,fld3: _11,fld4: _16,fld5: 4113778110_u32,fld6: Field::<i64>(Variant(_19, 1), 6),fld7: 101327371796670761273743042351338061101_i128 };
Goto(bb10)
}
bb10 = {
place!(Field::<i64>(Variant(_19, 1), 6)) = -_18.1.0;
place!(Field::<u32>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 5)) = 2667410729_u32;
place!(Field::<i64>(Variant(_19, 1), 6)) = _6 as i64;
_2 = !_3;
place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0.0 = _25;
_27 = &place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0.1;
place!(Field::<[i32; 8]>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 4)) = [(-415673185_i32),64864868_i32,(-1754289886_i32),(-309663557_i32),45329735_i32,1865773174_i32,(-1985348942_i32),(-1451658861_i32)];
Goto(bb11)
}
bb11 = {
(*_9) = _11 - Field::<f64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 3);
place!(Field::<*const f64>(Variant(_19, 1), 3)) = Move(_9);
_26 = [_14,_14,_14,_14,_14,_14];
RET = _1;
_1 = [Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(_19, 1), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(_19, 1), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6)];
_30 = [_14,_14,_14,_14,_14,_14];
RET = _1;
_16 = Field::<[i32; 8]>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 4);
_28 = _22 ^ _3;
_17 = &place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0;
_24 = 810608087725637198559274990237530307_u128;
place!(Field::<u32>(Variant(_19, 1), 0)) = _24 as u32;
_22 = _28 * _3;
_4 = _22 ^ _28;
_23 = _11;
_22 = !_3;
_5 = Field::<usize>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 0);
_20 = -(-40_i8);
Goto(bb12)
}
bb12 = {
place!(Field::<f64>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 3)) = -_23;
place!(Field::<f64>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 3)) = _23 - _23;
match Field::<u16>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 2) {
0 => bb4,
1 => bb13,
33167 => bb15,
_ => bb14
}
}
bb13 = {
_22 = _3;
_25 = '\u{ff0da}';
_24 = !63684149434912788137142752109309700396_u128;
_26 = [_14,_14,_14,_14,_14,_14];
_9 = core::ptr::addr_of!((*_9));
_5 = !14390311754877185820_usize;
_23 = -_11;
_16 = _8;
_18.1.3 = 33167_u16;
place!(Field::<i64>(Variant(_19, 1), 6)) = _24 as i64;
_20 = !(-120_i8);
_2 = -_22;
_18.0 = core::ptr::addr_of_mut!(_5);
place!(Field::<i64>(Variant(_19, 1), 6)) = _18.1.0 & _18.1.0;
_9 = core::ptr::addr_of!(_23);
_22 = _3;
place!(Field::<((char, *const i8),)>(Variant(_19, 1), 5)).0.1 = core::ptr::addr_of!(_20);
place!(Field::<Adt35>(Variant(_19, 1), 4)) = Adt35::Variant0 { fld0: _5,fld1: 177_u8,fld2: _18.1.3,fld3: (*_9),fld4: _16,fld5: 1527992950_u32,fld6: _18.1.0,fld7: (-118965576648566432216817796326049219024_i128) };
place!(Field::<Adt35>(Variant(_19, 1), 4)) = Adt35::Variant0 { fld0: _5,fld1: 138_u8,fld2: _18.1.3,fld3: _11,fld4: _16,fld5: 4113778110_u32,fld6: Field::<i64>(Variant(_19, 1), 6),fld7: 101327371796670761273743042351338061101_i128 };
Goto(bb10)
}
bb14 = {
_4 = 20314_u16 as isize;
Call(_3 = fn18(_13, _18.1.0, _13, _18.1.0, _13, _13, _13, _13, _13, _4, _16), ReturnTo(bb7), UnwindUnreachable())
}
bb15 = {
place!(Field::<i128>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 7)) = -107477304635696897045989298463254705057_i128;
_31 = &_32;
RET = [Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),_18.1.0,Field::<i64>(Variant(_19, 1), 6),Field::<i64>(Variant(_19, 1), 6),_18.1.0,Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6),Field::<i64>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 6)];
_24 = 9999733985120089627_u64 as u128;
_33 = !_14;
_3 = _28;
place!(Field::<[i32; 8]>(Variant(place!(Field::<Adt35>(Variant(_19, 1), 4)), 0), 4)) = [1711442920_i32,234551991_i32,386961573_i32,(-1609158389_i32),1607670213_i32,(-907996933_i32),833126694_i32,1584796756_i32];
_28 = _4;
_14 = _33;
_7 = [_4,_28,_3];
_23 = Field::<usize>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 0) as f64;
place!(Field::<[i8; 8]>(Variant(_19, 1), 1)) = [_20,_20,_20,_20,_20,_20,_20,_20];
_18.1.3 = !Field::<u16>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 2);
_18.0 = core::ptr::addr_of_mut!(_5);
_35 = [110_u8,103_u8,24_u8,20_u8,222_u8,157_u8,149_u8,1_u8];
place!(Field::<[u128; 6]>(Variant(_19, 1), 2)) = [_24,_24,_24,_24,_24,_24];
_6 = Field::<i128>(Variant(Field::<Adt35>(Variant(_19, 1), 4), 0), 7) as f32;
_3 = _22;
_7 = [_22,_3,_4];
_30 = [_14,_14,_33,_14,_14,_14];
_27 = &(*_27);
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(16_usize, 3_usize, Move(_3), 1_usize, Move(_1), 20_usize, Move(_20), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(16_usize, 13_usize, Move(_13), 5_usize, Move(_5), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(16_usize, 2_usize, Move(_2), 16_usize, Move(_16), 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [i64; 8],mut _2: [u128; 6],mut _3: usize,mut _4: isize,mut _5: [u128; 6],mut _6: [i32; 8],mut _7: [i32; 8],mut _8: isize,mut _9: [i64; 8]) -> isize {
mir! {
type RET = isize;
let _10: u16;
let _11: u32;
let _12: f64;
let _13: f32;
let _14: [char; 5];
let _15: &'static (i64, i16, &'static &'static i8, u16);
let _16: bool;
let _17: *const &'static u32;
let _18: &'static isize;
let _19: ([u32; 3], char);
let _20: ();
let _21: ();
{
_2 = [328290126811577187001485816505996687888_u128,94402590564661176225459498066871817515_u128,192218604604910487887545159353772850430_u128,100990360864634888388080689716005912892_u128,272296107617498652809883685673458274850_u128,98084614710959351719367793756235549968_u128];
_3 = 2_usize;
_7[_3] = !_6[_3];
_9 = _1;
_9 = _1;
RET = -_8;
_7 = [_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3]];
RET = (-41735331417529826203719355497121272157_i128) as isize;
_7[_3] = _6[_3];
_4 = _1[_3] as isize;
_13 = 104_i8 as f32;
_10 = !17081_u16;
_13 = _10 as f32;
_8 = _4 ^ _4;
_1[_3] = _10 as i64;
_2[_3] = 2507155014_u32 as u128;
_5 = [_2[_3],_2[_3],_2[_3],_2[_3],_2[_3],_2[_3]];
_7[_3] = (-169453460096771812531540281493488258026_i128) as i32;
_12 = _13 as f64;
_10 = 16253_u16 & 4044_u16;
_2 = [_5[_3],_5[_3],_5[_3],_5[_3],_5[_3],_5[_3]];
_1[_3] = _9[_3] + _9[_3];
_4 = _8 + _8;
_6[_3] = _3 as i32;
Call(_8 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _8 | _8;
_4 = '\u{1697a}' as isize;
RET = _8;
_6[_3] = _7[_3];
_14 = ['\u{fceac}','\u{fcceb}','\u{9bdf1}','\u{54729}','\u{f1e25}'];
_12 = 1623376768_u32 as f64;
_1[_3] = -_9[_3];
_7 = [_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3],_6[_3]];
_4 = _8 >> _5[_3];
_18 = &RET;
RET = _4 - _8;
_18 = &_4;
_1[_3] = _9[_3] + _9[_3];
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(17_usize, 14_usize, Move(_14), 10_usize, Move(_10), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(17_usize, 9_usize, Move(_9), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: i64,mut _3: isize,mut _4: i64,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: [i32; 8]) -> isize {
mir! {
type RET = isize;
let _12: Adt35;
let _13: ();
let _14: ();
{
_1 = -_3;
_3 = 4361609199150506063125574198493986189_i128 as isize;
_4 = _2;
Goto(bb1)
}
bb1 = {
_1 = _7 - _8;
_8 = _9;
_1 = _7;
_10 = _9 - _9;
RET = _6 + _1;
_3 = _1;
_6 = -_1;
_10 = _9 | _8;
_9 = (-22990_i16) as isize;
_7 = -RET;
_4 = -_2;
_3 = 5434395983061133723_u64 as isize;
_4 = _2;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(18_usize, 5_usize, Move(_5), 4_usize, Move(_4), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(18_usize, 9_usize, Move(_9), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(205_u8), std::hint::black_box(5013139131327679940841908553268655299_u128), std::hint::black_box(16309_u16), std::hint::black_box(159772611696298636630072829736005051653_i128), std::hint::black_box(1935235027_i32));
                
            }
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: usize,
fld1: u8,
fld2: u16,
fld3: f64,
fld4: [i32; 8],
fld5: u32,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: usize,
fld1: *mut i64,
fld2: i32,
fld3: u32,
fld4: i64,

},
Variant2{
fld0: *const f64,
fld1: (char, *const i8),
fld2: isize,
fld3: *mut u128,
fld4: u16,
fld5: f32,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *const [i8; 8],
fld1: [u32; 3],
fld2: isize,
fld3: i8,

},
Variant1{
fld0: *const f64,
fld1: i128,
fld2: *mut i64,
fld3: f32,
fld4: *const i8,

},
Variant2{
fld0: [u16; 5],
fld1: char,
fld2: u64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *mut usize,
fld1: f32,
fld2: u128,
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: f32,
fld1: *const [i8; 8],
fld2: *const f64,
fld3: u32,
fld4: [char; 5],
fld5: *const *mut u128,
fld6: [i32; 8],
fld7: u64,

},
Variant1{
fld0: [i32; 3],
fld1: *const i8,

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt69{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt69 {
fld0: u64,
fld1: usize,
fld2: i64,
fld3: [i32; 3],
}
impl PrintFDebug for Adt70{
	unsafe fn printf_debug(&self){unsafe{printf("Adt70::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt70 {
Variant0{
fld0: [i8; 2],
fld1: [i32; 8],
fld2: [u32; 3],
fld3: Adt69,

},
Variant1{
fld0: *const i8,
fld1: char,
fld2: Adt63,
fld3: u64,
fld4: [u16; 6],

}}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){unsafe{printf("Adt72::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt72 {
Variant0{
fld0: [i32; 8],
fld1: *mut i64,

},
Variant1{
fld0: [char; 3],

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){unsafe{printf("Adt74::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: *mut Adt45,
fld1: u64,
fld2: usize,
fld3: *const f64,
fld4: *mut f32,
fld5: [u128; 6],

},
Variant1{
fld0: u32,
fld1: [i8; 8],
fld2: [u128; 6],
fld3: *const f64,
fld4: Adt35,
fld5: ((char, *const i8),),
fld6: i64,

},
Variant2{
fld0: u128,

}}

