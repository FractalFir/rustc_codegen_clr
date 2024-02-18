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
pub fn fn0(mut _1: i64,mut _2: i32,mut _3: u32) -> f64 {
mir! {
type RET = f64;
let _4: f64;
let _5: [u32; 2];
let _6: isize;
let _7: [u64; 4];
let _8: i16;
let _9: Adt38;
let _10: f32;
let _11: f64;
let _12: char;
let _13: bool;
let _14: Adt44;
let _15: isize;
let _16: i128;
let _17: &'static &'static u16;
let _18: &'static f32;
let _19: u8;
let _20: (&'static *const f64,);
let _21: [usize; 7];
let _22: Adt57;
let _23: [i128; 3];
let _24: bool;
let _25: ();
let _26: ();
{
_1 = !(-198388721026969357_i64);
RET = 2945109384413397315_usize as f64;
_3 = 2199126206_u32 - 3244055682_u32;
RET = (-14997197336824639060192194616347684366_i128) as f64;
_4 = -RET;
_5 = [_3,_3];
_6 = !92_isize;
_1 = 99_i8 as i64;
RET = -_4;
_5 = [_3,_3];
RET = _4;
_7 = [10463266045484231041_u64,13520853735105180812_u64,11718848076523972133_u64,2223852436920153046_u64];
_9.fld0 = [(-10_i8),85_i8,(-114_i8)];
_9.fld1 = [10600929655402837390_u64,12640173853615066101_u64,6718830886179043275_u64,12166439339391862264_u64,3399656816975023352_u64];
_11 = 268808445572390735077151586457099887968_u128 as f64;
RET = _4 * _4;
_12 = '\u{265fd}';
_10 = _6 as f32;
_2 = (-299285594_i32) << _1;
RET = _4 - _11;
Call(_12 = fn1(_7, _9.fld1, _11, _7, _9.fld0, _9.fld0, _9.fld1, _9.fld1, _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _4 + _11;
_2 = _10 as i32;
_3 = _12 as u32;
_5 = [_3,_3];
RET = _2 as f64;
_12 = '\u{26476}';
_5 = [_3,_3];
_6 = 9223372036854775807_isize | (-9223372036854775808_isize);
_5 = [_3,_3];
_13 = false | true;
_5 = [_3,_3];
_7 = [18328621189791460868_u64,14624577665035575454_u64,8448740884779112185_u64,2609232518723357526_u64];
_2 = 292988883_i32 >> _6;
_5 = [_3,_3];
_8 = 30851_i16;
_16 = (-92447266735980746111378393872099833361_i128) & (-10171261415878156914246033537026892865_i128);
_14 = Adt44::Variant2 { fld0: _9.fld1 };
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
30851 => bb9,
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
_11 = -RET;
_6 = (-43_isize) | (-9223372036854775808_isize);
_11 = _6 as f64;
RET = _11 + _4;
RET = _4 - _11;
RET = _8 as f64;
_9.fld0 = [(-9_i8),(-100_i8),(-57_i8)];
_18 = &_10;
_3 = _11 as u32;
_16 = (-141754842385873366598072025651954664627_i128) | (-101582827559398559603919570781825287777_i128);
_11 = RET;
_13 = (*_18) >= (*_18);
_15 = _6;
_6 = -_15;
_18 = &(*_18);
_1 = _2 as i64;
Call(_9.fld1 = core::intrinsics::transmute(Field::<[u64; 5]>(Variant(_14, 2), 0)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = _11 - _4;
_5 = [_3,_3];
_9.fld1 = [15811930258957683491_u64,16589639507723020010_u64,6393248077738768519_u64,8544968816235814429_u64,9472158993254249330_u64];
place!(Field::<[u64; 5]>(Variant(_14, 2), 0)) = _9.fld1;
_11 = _4 - RET;
_12 = '\u{e20b3}';
RET = _4;
_11 = RET + _4;
_8 = 29642_i16 ^ (-8405_i16);
_13 = _8 != _8;
_8 = (-11925_i16);
_10 = (-111_i8) as f32;
_9.fld0 = [28_i8,(-35_i8),92_i8];
_6 = _15 + _15;
_2 = -659684006_i32;
_2 = 5018169644694996550_u64 as i32;
_15 = !_6;
RET = _11;
_21 = [0_usize,7_usize,1_usize,4_usize,4_usize,3819356813432882776_usize,2_usize];
_15 = _6;
_14 = Adt44::Variant2 { fld0: _9.fld1 };
_10 = _3 as f32;
_11 = _2 as f64;
_18 = &_10;
_9.fld0 = [(-83_i8),18_i8,(-9_i8)];
_23 = [_16,_16,_16];
_13 = _6 < _15;
place!(Field::<[u64; 5]>(Variant(_14, 2), 0)) = [7056072254815833692_u64,7138380088817991253_u64,13653964374099948083_u64,14451945227935430330_u64,2632098472994289361_u64];
Goto(bb11)
}
bb11 = {
_22.fld0 = core::ptr::addr_of_mut!(_13);
match _8 {
0 => bb1,
1 => bb3,
2 => bb12,
3 => bb13,
4 => bb14,
340282366920938463463374607431768199531 => bb16,
_ => bb15
}
}
bb12 = {
RET = _11 - _4;
_5 = [_3,_3];
_9.fld1 = [15811930258957683491_u64,16589639507723020010_u64,6393248077738768519_u64,8544968816235814429_u64,9472158993254249330_u64];
place!(Field::<[u64; 5]>(Variant(_14, 2), 0)) = _9.fld1;
_11 = _4 - RET;
_12 = '\u{e20b3}';
RET = _4;
_11 = RET + _4;
_8 = 29642_i16 ^ (-8405_i16);
_13 = _8 != _8;
_8 = (-11925_i16);
_10 = (-111_i8) as f32;
_9.fld0 = [28_i8,(-35_i8),92_i8];
_6 = _15 + _15;
_2 = -659684006_i32;
_2 = 5018169644694996550_u64 as i32;
_15 = !_6;
RET = _11;
_21 = [0_usize,7_usize,1_usize,4_usize,4_usize,3819356813432882776_usize,2_usize];
_15 = _6;
_14 = Adt44::Variant2 { fld0: _9.fld1 };
_10 = _3 as f32;
_11 = _2 as f64;
_18 = &_10;
_9.fld0 = [(-83_i8),18_i8,(-9_i8)];
_23 = [_16,_16,_16];
_13 = _6 < _15;
place!(Field::<[u64; 5]>(Variant(_14, 2), 0)) = [7056072254815833692_u64,7138380088817991253_u64,13653964374099948083_u64,14451945227935430330_u64,2632098472994289361_u64];
Goto(bb11)
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
_3 = (*_18) as u32;
_19 = !146_u8;
_18 = &(*_18);
_9.fld0 = [107_i8,46_i8,(-57_i8)];
_23 = [_16,_16,_16];
_12 = '\u{d242f}';
_19 = _6 as u8;
_18 = &_10;
_4 = -RET;
_21 = [5_usize,6211640385164821602_usize,6_usize,15989910815819849832_usize,9687666747088960149_usize,4_usize,1116672301412259625_usize];
_10 = _16 as f32;
_12 = '\u{fa1f0}';
_23 = [_16,_16,_16];
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(0_usize, 6_usize, Move(_6), 19_usize, Move(_19), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(0_usize, 13_usize, Move(_13), 7_usize, Move(_7), 23_usize, Move(_23), 26_usize, _26), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [u64; 4],mut _2: [u64; 5],mut _3: f64,mut _4: [u64; 4],mut _5: [i8; 3],mut _6: [i8; 3],mut _7: [u64; 5],mut _8: [u64; 5],mut _9: isize,mut _10: [u64; 4]) -> char {
mir! {
type RET = char;
let _11: i16;
let _12: bool;
let _13: isize;
let _14: [i64; 8];
let _15: isize;
let _16: [i8; 2];
let _17: [u64; 6];
let _18: [u64; 4];
let _19: f64;
let _20: &'static *mut [isize; 5];
let _21: char;
let _22: f64;
let _23: isize;
let _24: Adt42;
let _25: u32;
let _26: char;
let _27: isize;
let _28: [u32; 4];
let _29: &'static *const (&'static *const f64,);
let _30: (Adt38, *mut [u32; 4]);
let _31: f32;
let _32: isize;
let _33: &'static &'static u16;
let _34: &'static &'static u16;
let _35: (Adt38, *mut [u32; 4]);
let _36: *const Adt25;
let _37: &'static isize;
let _38: isize;
let _39: u64;
let _40: &'static u8;
let _41: *mut Adt25;
let _42: u8;
let _43: i8;
let _44: *mut [isize; 5];
let _45: isize;
let _46: Adt35;
let _47: ();
let _48: ();
{
_8 = [1145366862280443892_u64,16653135574465104599_u64,2141297744633394264_u64,11598357628724777886_u64,3568562531991089625_u64];
_6 = _5;
RET = '\u{17702}';
_4 = [9026659398480177757_u64,14216755832875562670_u64,468484293150858105_u64,14854061541365514255_u64];
_2 = [14821701415793957306_u64,6363184344488953333_u64,12890826159281959886_u64,16594279944311814438_u64,11380999587172685247_u64];
RET = '\u{ad593}';
RET = '\u{2ad25}';
Call(_3 = fn2(_7, _1, _7, _6, _10, _4, _10, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [106_i8,(-15_i8),(-59_i8)];
_10 = _4;
_6 = [90_i8,(-27_i8),(-56_i8)];
_6 = _5;
_11 = 25342_i16;
_11 = 23863_i16 >> _9;
RET = '\u{50453}';
_11 = 39167_u16 as i16;
_10 = _4;
_3 = _11 as f64;
RET = '\u{4c6bd}';
_8 = _7;
_6 = [38_i8,68_i8,27_i8];
Goto(bb2)
}
bb2 = {
_7 = [459922536104494183_u64,3431689091605402467_u64,6270168113912488884_u64,5496708030840676728_u64,2844893359185070126_u64];
RET = '\u{3894b}';
_9 = -9223372036854775807_isize;
_12 = _9 != _9;
_11 = !(-12087_i16);
RET = '\u{87f32}';
_2 = [1683309063752459834_u64,9615923333024416648_u64,13061228060030684753_u64,860280379813363682_u64,7022169585674507229_u64];
_7 = [5264875990225001361_u64,9881852147105462035_u64,14577058205118728434_u64,1133693757443086270_u64,18290024784580179075_u64];
_7 = [2859500327028930303_u64,15690965709431751988_u64,4821230586910501198_u64,13837736546867913789_u64,9473071204414335688_u64];
RET = '\u{f19d5}';
_3 = _11 as f64;
_14 = [(-3895946870793292723_i64),6497068673156077999_i64,2977210362049878061_i64,7004515483095391972_i64,(-6443678035676805140_i64),2345997153574267924_i64,(-2952269031669774742_i64),(-341145029351566031_i64)];
RET = '\u{e4325}';
Goto(bb3)
}
bb3 = {
_1 = _10;
_8 = [4422199865357726766_u64,14738764487074862102_u64,8896481490129126718_u64,1789639811426483996_u64,5787021143024728926_u64];
_5 = [(-57_i8),(-37_i8),126_i8];
_1 = [8678851119748713858_u64,662023136964471807_u64,11360678547714534399_u64,17157958487738937580_u64];
_17 = [199950859667103798_u64,14122412860502456162_u64,7386290266022489639_u64,2198843552817154042_u64,13131414648001663288_u64,9665518299197576920_u64];
_10 = [5354262365825189195_u64,6643982257568490019_u64,4027014657880271349_u64,6214730848607707972_u64];
_16 = [(-62_i8),(-82_i8)];
RET = '\u{94f56}';
_8 = [17534178960179633426_u64,9056292451771607084_u64,11428172947340224506_u64,15828274502016016532_u64,4295312292670587726_u64];
_16 = [113_i8,109_i8];
_11 = !(-7658_i16);
Call(_1 = fn18(_14, _12, _9, _8, _5, _17, _14, _10, _14, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12 = !false;
Goto(bb5)
}
bb5 = {
_17 = [11514260373296791558_u64,13108592441027249255_u64,12397530491425387459_u64,11465463130696168300_u64,1928520318156104090_u64,6131370521854365920_u64];
_15 = _9 ^ _9;
_12 = false;
_5 = [30_i8,101_i8,111_i8];
_12 = true;
_7 = _2;
_7 = [17510994792605517170_u64,14700631895753832637_u64,11690705839878637179_u64,2315540313827326519_u64,4191272158662546417_u64];
_15 = 147_u8 as isize;
_10 = [7140391677378851999_u64,12590498684136603701_u64,7458069971177611472_u64,12522798772289345427_u64];
_18 = _1;
Goto(bb6)
}
bb6 = {
_4 = _18;
_18 = [5148977911027212738_u64,15062745603632053772_u64,17559734506186639937_u64,14564081613849246323_u64];
_8 = _2;
_10 = _4;
_6 = _5;
_5 = [76_i8,(-41_i8),(-123_i8)];
_9 = _15 & _15;
_5 = _6;
_22 = _3 * _3;
_9 = _15;
_5 = _6;
_18 = [15655790803662221169_u64,13626556262079541751_u64,13284361813291173697_u64,1283827475631838245_u64];
_8 = [13008720399252863924_u64,8933044186332477858_u64,13468068781404536989_u64,18288159628715176947_u64,15519856125137877398_u64];
_21 = RET;
_13 = _15 * _9;
_9 = (-148252043770482705391187103081815283755_i128) as isize;
_7 = [11264848072617642769_u64,3321128835614193067_u64,9468432506150376211_u64,970518563325446351_u64,3930258194665629968_u64];
_5 = _6;
RET = _21;
_7 = [5406176037422057567_u64,11808087337392274205_u64,4612704650920502855_u64,16373928842073304846_u64,17906765805378464356_u64];
_12 = false;
_22 = -_3;
_17 = [10426562232422306641_u64,6514840146404768130_u64,2084976780046281205_u64,4811130374707618237_u64,14008759237930553619_u64,7169282050665645003_u64];
_10 = [16465651672931000601_u64,16223352491121456810_u64,5910031523978589089_u64,5129786084339099432_u64];
_23 = _9;
_13 = _15;
_22 = _3;
_3 = _22 - _22;
_14 = [(-4681378657440304156_i64),7519863556601777634_i64,8761484548496448747_i64,966703321215723296_i64,(-3613885675036828931_i64),2527912607701318886_i64,534596177281564090_i64,119910046826736488_i64];
Call(_15 = fn19(_1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = _13 & _9;
_12 = _3 >= _3;
_4 = [14211702937074014369_u64,12637492646242912877_u64,9167925900542378962_u64,13843947909359179882_u64];
_18 = [4755008596091404658_u64,5156954211585534953_u64,5647135435975048613_u64,685003304251647487_u64];
_14 = [6834306005870424125_i64,(-189507791012952046_i64),(-7590929369884609124_i64),5039325033440158523_i64,1257710760188084713_i64,(-7061836423577895490_i64),(-7264935292281436404_i64),(-6773045475748663959_i64)];
_10 = [43143837168942811_u64,6784719046917145061_u64,7996991037360533454_u64,6647904820989012987_u64];
_5 = [(-61_i8),(-52_i8),64_i8];
RET = _21;
_2 = _8;
_9 = (-1844767733_i32) as isize;
Goto(bb8)
}
bb8 = {
_5 = [47_i8,27_i8,(-34_i8)];
_25 = 2323349166_u32 - 2700250287_u32;
_25 = 2839110341_u32 - 3334432262_u32;
_12 = false;
_5 = [31_i8,1_i8,(-9_i8)];
_5 = [27_i8,12_i8,42_i8];
_11 = (-2662_i16);
_19 = _3 - _3;
RET = _21;
_12 = !false;
_21 = RET;
_1 = [15004141699956001031_u64,10536830838059323715_u64,7970894364764854135_u64,15074485138551554974_u64];
_30.0 = Adt38 { fld0: _5,fld1: _8 };
_5 = [(-88_i8),49_i8,106_i8];
_26 = _21;
_1 = _4;
_30.1 = core::ptr::addr_of_mut!(_28);
_25 = 3744288721_u32 * 1661905213_u32;
_4 = [4770759962083626550_u64,4876499731546779337_u64,8010325737871845607_u64,3372570394847612835_u64];
RET = _21;
_22 = -_19;
_28 = [_25,_25,_25,_25];
_19 = _3 - _3;
match _11 {
0 => bb6,
340282366920938463463374607431768208794 => bb10,
_ => bb9
}
}
bb9 = {
_12 = !false;
Goto(bb5)
}
bb10 = {
_27 = _23 | _13;
_7 = _2;
_2 = _8;
_14 = [(-4802287492971837879_i64),4139443851372957257_i64,622215729543225506_i64,1862126237998133047_i64,4296735918752016002_i64,63658614792321568_i64,(-5273499374306993829_i64),3763835880715899635_i64];
_2 = [5666480474743077222_u64,9440820668871091642_u64,16277958643721253185_u64,11964424782810765919_u64,5528525539285702775_u64];
_1 = _4;
_30.0.fld1 = [11986033080200086199_u64,14932890283659798016_u64,12283875460747358465_u64,11536676692023641398_u64,6055545798580461356_u64];
_27 = _15 & _23;
_9 = _15 - _13;
_10 = [10159913386631569440_u64,3839696583134175289_u64,2748407686714584782_u64,4730395783954155803_u64];
_35.0 = Adt38 { fld0: _5,fld1: _2 };
_11 = !(-32091_i16);
Goto(bb11)
}
bb11 = {
_35.1 = Move(_30.1);
_9 = _25 as isize;
_31 = _11 as f32;
_30 = (_35.0, Move(_35.1));
_35.1 = Move(_30.1);
_30.1 = core::ptr::addr_of_mut!(_28);
_28 = [_25,_25,_25,_25];
Goto(bb12)
}
bb12 = {
_31 = 13_u8 as f32;
_13 = _25 as isize;
_38 = !_27;
_31 = _19 as f32;
_28 = [_25,_25,_25,_25];
_35 = (_30.0, Move(_30.1));
_26 = _21;
_35.0.fld0 = [(-35_i8),70_i8,115_i8];
_18 = _4;
_27 = !_38;
Goto(bb13)
}
bb13 = {
_32 = _15;
_37 = &_23;
_4 = [14318296696795475779_u64,3446797336783676878_u64,14739831110767846560_u64,17137520966012623941_u64];
_23 = _38 >> _32;
_1 = [3301159801866663888_u64,5056606313290475292_u64,18122375614138418061_u64,9429682032214783148_u64];
_16 = [40_i8,120_i8];
_30.0 = _35.0;
_30.0.fld1 = [6919286061289124745_u64,13639967891787593423_u64,4426670381880323125_u64,3900567306900514404_u64,3700930842283744267_u64];
_30 = (_35.0, Move(_35.1));
_22 = -_19;
_12 = _27 <= _32;
_32 = _38 ^ _15;
_17 = [2605657532549605904_u64,3904228189010606001_u64,14780432245993116271_u64,11415378648675026668_u64,15357555851342629410_u64,13167312469597974503_u64];
_8 = [3969499090718484952_u64,2427782931766435485_u64,8331455790343596314_u64,8445634116519638098_u64,12930884560420442963_u64];
RET = _21;
_35.1 = core::ptr::addr_of_mut!(_28);
_40 = &_42;
_35.0.fld0 = [(-52_i8),40_i8,(-107_i8)];
_40 = &(*_40);
_16 = [(-93_i8),(-52_i8)];
_43 = _32 as i8;
_30 = (_35.0, Move(_35.1));
_19 = 46822_u16 as f64;
_43 = !41_i8;
_35.1 = core::ptr::addr_of_mut!(_28);
Goto(bb14)
}
bb14 = {
_39 = 4113377101582949070_u64;
_18 = _1;
_14 = [(-7598287972057474015_i64),3055414015439657378_i64,7425446797238119113_i64,(-6058646898559960804_i64),(-7938408817277683093_i64),(-8806633031809320387_i64),7922469743779191825_i64,(-8945851256492063001_i64)];
_39 = !17392927182578855688_u64;
_10 = [_39,_39,_39,_39];
_2 = [_39,_39,_39,_39,_39];
_11 = 5596_i16;
_30.1 = core::ptr::addr_of_mut!(_28);
_45 = !_9;
_1 = [_39,_39,_39,_39];
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(1_usize, 1_usize, Move(_1), 26_usize, Move(_26), 39_usize, Move(_39), 45_usize, Move(_45)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(1_usize, 28_usize, Move(_28), 10_usize, Move(_10), 15_usize, Move(_15), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(1_usize, 2_usize, Move(_2), 25_usize, Move(_25), 17_usize, Move(_17), 32_usize, Move(_32)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(1_usize, 16_usize, Move(_16), 7_usize, Move(_7), 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u64; 5],mut _2: [u64; 4],mut _3: [u64; 5],mut _4: [i8; 3],mut _5: [u64; 4],mut _6: [u64; 4],mut _7: [u64; 4],mut _8: [u64; 5]) -> f64 {
mir! {
type RET = f64;
let _9: [i128; 3];
let _10: *const u16;
let _11: *mut [u32; 4];
let _12: isize;
let _13: u16;
let _14: isize;
let _15: *mut bool;
let _16: [isize; 5];
let _17: [i128; 3];
let _18: *const (&'static *const f64,);
let _19: isize;
let _20: u64;
let _21: bool;
let _22: *mut [isize; 5];
let _23: Adt38;
let _24: (*const f64, *mut &'static u8);
let _25: f64;
let _26: bool;
let _27: char;
let _28: f64;
let _29: bool;
let _30: f32;
let _31: [u128; 7];
let _32: [u16; 8];
let _33: (Adt31,);
let _34: char;
let _35: *const f64;
let _36: Adt42;
let _37: &'static u8;
let _38: Adt57;
let _39: u32;
let _40: i16;
let _41: f32;
let _42: f32;
let _43: [u32; 7];
let _44: u32;
let _45: [u64; 6];
let _46: [u64; 4];
let _47: [u64; 6];
let _48: char;
let _49: usize;
let _50: [u32; 4];
let _51: ();
let _52: ();
{
Call(_8 = fn3(_6, _7, _1, _5, _1, _7, _5, _2, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [10521488307579760327_u64,6922363777350379632_u64,14949322929403584670_u64,829164772080415767_u64,4903103979263691029_u64];
RET = 2883933056619685773_u64 as f64;
_2 = _6;
_5 = [7327789451101961317_u64,10368784794335772605_u64,5184995131142359449_u64,7617929886158757901_u64];
_3 = _1;
_4 = [76_i8,51_i8,(-42_i8)];
_5 = [14109297401475518243_u64,10182015164310880334_u64,3441373495840839103_u64,3844112184498463813_u64];
_2 = [15499773831517401021_u64,11615692102652562794_u64,8495466304872312617_u64,15804972503537484574_u64];
_6 = _2;
_9 = [169191470532722229985354956308836266798_i128,4080620004838368375044431474118264501_i128,(-118256731023889474442736035151946906999_i128)];
RET = 9981957968223688947_u64 as f64;
_2 = [16167766772761503917_u64,4268498265004021645_u64,3459765916581709226_u64,14839748991301170567_u64];
_6 = [10406756553925034445_u64,12095122520383288548_u64,14447278922324720641_u64,11602539761035924066_u64];
_4 = [89_i8,101_i8,(-75_i8)];
Call(_8 = fn17(_3, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = [155831439536526575943294616264003043751_i128,(-19521178276865075743346973170552321400_i128),(-81996746846414173226151451248918104637_i128)];
_4 = [66_i8,(-109_i8),68_i8];
_1 = [13581898838708379289_u64,10739764612920334408_u64,13443078489862418148_u64,9102263254428970590_u64,9387188454916169092_u64];
_8 = [8872571351743720475_u64,17916808417198514434_u64,6064230943523767983_u64,9236139472702073754_u64,5116660271673682339_u64];
_9 = [16424134521207015667822062774149658382_i128,97258649232693519821220263478195913500_i128,(-147755599876150958127787837269433533245_i128)];
Goto(bb3)
}
bb3 = {
_5 = [4677263866962101696_u64,6952676616796179073_u64,12107443635204292178_u64,5580142579362941885_u64];
RET = (-6134716462686864552_i64) as f64;
RET = 43552_u16 as f64;
_8 = _1;
_2 = [9165146616826346102_u64,5852455703524735169_u64,1494952297181745594_u64,9693535413663233482_u64];
_3 = _1;
Goto(bb4)
}
bb4 = {
RET = 3_usize as f64;
_12 = 83_isize - (-9223372036854775808_isize);
_2 = [15551801774405146880_u64,9580313324135811058_u64,332928061702614582_u64,9321693985415065690_u64];
_12 = 6895339773136670712_u64 as isize;
_8 = [8927893031056372034_u64,16011839397780312644_u64,12757199972314731119_u64,3920085288501249677_u64,15645839618107494554_u64];
_9 = [(-148805632605597283031443675746313903956_i128),36369951028627803524318872729858863171_i128,(-57553345523654252459481072001525740489_i128)];
_6 = _7;
_13 = !27454_u16;
_3 = [11107498045368037225_u64,6784898500195196097_u64,18255457624579959824_u64,15457876701508923919_u64,16799307349999273387_u64];
_10 = core::ptr::addr_of!(_13);
_3 = _8;
_13 = (-6951_i16) as u16;
_5 = _2;
RET = (-388355138_i32) as f64;
_3 = [10885544349482602855_u64,8200276198056894827_u64,4533402593574692504_u64,1750393891387359647_u64,14124193644510794018_u64];
_3 = _8;
RET = (-51_i8) as f64;
_3 = [14409399820150378997_u64,11957609121418150326_u64,5166513257986649302_u64,16471453599422084809_u64,10203022625751540255_u64];
RET = (-53_i8) as f64;
_9 = [139400039430250966211415621954354758747_i128,(-137278254066890682603999901003280553732_i128),(-157622809470234740661181566733764847333_i128)];
Goto(bb5)
}
bb5 = {
_2 = [9038289695331705900_u64,12032462971464733505_u64,6137967130916567141_u64,1835348600313895677_u64];
RET = 134568369263842337032362278644989582531_u128 as f64;
RET = 99_u8 as f64;
_16 = [_12,_12,_12,_12,_12];
_3 = [18011201098002933607_u64,16852044725166405459_u64,16179413256408695352_u64,7137408321897109818_u64,14926860068323181803_u64];
_17 = _9;
(*_10) = 17875_u16;
_4 = [120_i8,39_i8,(-48_i8)];
_10 = core::ptr::addr_of!(_13);
_14 = _12;
_8 = _1;
_1 = [9505520112885218016_u64,15343065895631187154_u64,928177634790649224_u64,1474534172570362449_u64,9565402437755994698_u64];
_7 = _5;
_13 = 2637_u16 * 62220_u16;
RET = 5132003892938206627_usize as f64;
_19 = _12 << _13;
_12 = _19;
_2 = [5989665910200543790_u64,14074225060217907605_u64,14012411742711674921_u64,13825334201914980959_u64];
_1 = _3;
_17 = [125322293824663211558220109181849718739_i128,12719046696221986962767712153756721988_i128,(-61240392697136711312287842724802025358_i128)];
_14 = 30_u8 as isize;
_12 = -_19;
(*_10) = !45110_u16;
Goto(bb6)
}
bb6 = {
(*_10) = false as u16;
_21 = !false;
_17 = [(-76378509289049161721549373675900118705_i128),144750197850899360104176004100452483220_i128,15083513759393950248415579829690577697_i128];
_20 = 11329722159225283225_u64;
RET = (*_10) as f64;
_7 = [_20,_20,_20,_20];
_3 = [_20,_20,_20,_20,_20];
_9 = [(-54927158513588364655161991963101527192_i128),(-80314327574069227858096878814335504475_i128),(-127032146177079188628329449788682747261_i128)];
_21 = !false;
_22 = core::ptr::addr_of_mut!(_16);
Goto(bb7)
}
bb7 = {
_7 = [_20,_20,_20,_20];
_17 = _9;
_21 = !false;
_9 = [(-15628053869445672523570065359736168795_i128),98116886820223603073678858168883659968_i128,131694475625373433632035307343550769279_i128];
_8 = [_20,_20,_20,_20,_20];
_23.fld1 = [_20,_20,_20,_20,_20];
_1 = [_20,_20,_20,_20,_20];
_23 = Adt38 { fld0: _4,fld1: _3 };
_3 = [_20,_20,_20,_20,_20];
_7 = [_20,_20,_20,_20];
_13 = 3077673673_u32 as u16;
match _20 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
11329722159225283225 => bb9,
_ => bb8
}
}
bb8 = {
(*_10) = false as u16;
_21 = !false;
_17 = [(-76378509289049161721549373675900118705_i128),144750197850899360104176004100452483220_i128,15083513759393950248415579829690577697_i128];
_20 = 11329722159225283225_u64;
RET = (*_10) as f64;
_7 = [_20,_20,_20,_20];
_3 = [_20,_20,_20,_20,_20];
_9 = [(-54927158513588364655161991963101527192_i128),(-80314327574069227858096878814335504475_i128),(-127032146177079188628329449788682747261_i128)];
_21 = !false;
_22 = core::ptr::addr_of_mut!(_16);
Goto(bb7)
}
bb9 = {
_8 = _1;
(*_22) = [_12,_19,_12,_19,_12];
_1 = _23.fld1;
(*_22) = [_14,_12,_14,_19,_12];
(*_22) = [_19,_14,_19,_12,_12];
_23 = Adt38 { fld0: _4,fld1: _1 };
_21 = true;
RET = _13 as f64;
_19 = -_12;
(*_22) = [_12,_12,_12,_12,_19];
(*_10) = 26246_u16 * 44692_u16;
_28 = RET * RET;
_27 = '\u{d3c35}';
_27 = '\u{f4a74}';
_24.0 = core::ptr::addr_of!(_25);
_30 = 701677612592095240_i64 as f32;
_3 = [_20,_20,_20,_20,_20];
_26 = !_21;
_5 = [_20,_20,_20,_20];
_23 = Adt38 { fld0: _4,fld1: _1 };
(*_10) = 30319_u16;
_30 = 9121338313160499288_i64 as f32;
(*_22) = [_12,_14,_19,_12,_19];
RET = _28 + _28;
_15 = core::ptr::addr_of_mut!(_21);
Goto(bb10)
}
bb10 = {
_24.0 = core::ptr::addr_of!(_25);
_26 = _21;
_10 = core::ptr::addr_of!((*_10));
_15 = core::ptr::addr_of_mut!(_29);
_9 = [25067644419797698596123681512534481920_i128,(-122518241462264822734590115978538361967_i128),(-127492551627131432520325140179064764333_i128)];
_22 = core::ptr::addr_of_mut!((*_22));
_29 = _26;
_1 = _3;
(*_10) = !45747_u16;
_34 = _27;
_28 = RET;
_27 = _34;
_25 = 87_i8 as f64;
_23 = Adt38 { fld0: _4,fld1: _3 };
_32 = [(*_10),(*_10),_13,(*_10),(*_10),_13,(*_10),_13];
_24.0 = core::ptr::addr_of!(RET);
_1 = [_20,_20,_20,_20,_20];
_4 = [99_i8,(-6_i8),(-91_i8)];
_23.fld0 = _4;
_12 = _19 | _19;
match _20 {
0 => bb9,
1 => bb8,
11329722159225283225 => bb12,
_ => bb11
}
}
bb11 = {
_2 = [9038289695331705900_u64,12032462971464733505_u64,6137967130916567141_u64,1835348600313895677_u64];
RET = 134568369263842337032362278644989582531_u128 as f64;
RET = 99_u8 as f64;
_16 = [_12,_12,_12,_12,_12];
_3 = [18011201098002933607_u64,16852044725166405459_u64,16179413256408695352_u64,7137408321897109818_u64,14926860068323181803_u64];
_17 = _9;
(*_10) = 17875_u16;
_4 = [120_i8,39_i8,(-48_i8)];
_10 = core::ptr::addr_of!(_13);
_14 = _12;
_8 = _1;
_1 = [9505520112885218016_u64,15343065895631187154_u64,928177634790649224_u64,1474534172570362449_u64,9565402437755994698_u64];
_7 = _5;
_13 = 2637_u16 * 62220_u16;
RET = 5132003892938206627_usize as f64;
_19 = _12 << _13;
_12 = _19;
_2 = [5989665910200543790_u64,14074225060217907605_u64,14012411742711674921_u64,13825334201914980959_u64];
_1 = _3;
_17 = [125322293824663211558220109181849718739_i128,12719046696221986962767712153756721988_i128,(-61240392697136711312287842724802025358_i128)];
_14 = 30_u8 as isize;
_12 = -_19;
(*_10) = !45110_u16;
Goto(bb6)
}
bb12 = {
_21 = _12 > _19;
_17 = _9;
_6 = [_20,_20,_20,_20];
_38 = Adt57 { fld0: Move(_15) };
_23 = Adt38 { fld0: _4,fld1: _8 };
_35 = core::ptr::addr_of!(RET);
_34 = _27;
_6 = [_20,_20,_20,_20];
_5 = [_20,_20,_20,_20];
_8 = [_20,_20,_20,_20,_20];
_38.fld0 = core::ptr::addr_of_mut!(_21);
_19 = _12 ^ _12;
_20 = !6760247017095977551_u64;
_39 = 3056315290_u32;
_5 = [_20,_20,_20,_20];
_24.1 = core::ptr::addr_of_mut!(_37);
_16 = [_12,_14,_12,_14,_12];
Goto(bb13)
}
bb13 = {
_23 = Adt38 { fld0: _4,fld1: _8 };
_32 = [_13,_13,(*_10),(*_10),(*_10),_13,_13,(*_10)];
_15 = Move(_38.fld0);
_40 = (-7236_i16) & 21558_i16;
_22 = core::ptr::addr_of_mut!((*_22));
_7 = _2;
(*_22) = [_19,_19,_12,_19,_19];
_38.fld0 = core::ptr::addr_of_mut!(_21);
_31 = [286923358882257990771550866395597837799_u128,55920751744414932031879645198472352734_u128,256398480889738381428747456941684060157_u128,190842001233300008897029276900704042925_u128,242192688763495886420177389224890321103_u128,86032534251128614053468090180073465033_u128,266685025870708486984281412348455005227_u128];
_27 = _34;
_42 = _30;
_43 = [_39,_39,_39,_39,_39,_39,_39];
_2 = [_20,_20,_20,_20];
_4 = [(-127_i8),61_i8,(-45_i8)];
_22 = core::ptr::addr_of_mut!(_16);
_19 = _12 | _12;
_12 = _19;
_35 = core::ptr::addr_of!((*_35));
_6 = _7;
RET = (-23_i8) as f64;
Call(_30 = core::intrinsics::transmute(_39), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_41 = _19 as f32;
_8 = [_20,_20,_20,_20,_20];
_26 = _19 != _19;
_38 = Adt57 { fld0: Move(_15) };
RET = _28 * _28;
_20 = 13127768016092208676_u64;
_17 = [145606037986889859865357249141692391405_i128,(-89180606025739054272196095790344920940_i128),34398366291434123633804395918309942938_i128];
_43 = [_39,_39,_39,_39,_39,_39,_39];
_17 = [60549002178083148349303300764981085203_i128,(-59983069048502951226825349264627324764_i128),90738281995573253159045013785719931961_i128];
(*_35) = _28;
_46 = _6;
(*_35) = _28;
_24.1 = core::ptr::addr_of_mut!(_37);
_22 = core::ptr::addr_of_mut!(_16);
_33.0 = Adt31::Variant2 { fld0: _26,fld1: 6_usize,fld2: _12 };
_40 = 24131_i16 << Field::<isize>(Variant(_33.0, 2), 2);
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(2_usize, 26_usize, Move(_26), 29_usize, Move(_29), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(2_usize, 32_usize, Move(_32), 4_usize, Move(_4), 8_usize, Move(_8), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(2_usize, 39_usize, Move(_39), 27_usize, Move(_27), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(2_usize, 5_usize, Move(_5), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [u64; 4],mut _2: [u64; 4],mut _3: [u64; 5],mut _4: [u64; 4],mut _5: [u64; 5],mut _6: [u64; 4],mut _7: [u64; 4],mut _8: [u64; 4],mut _9: [u64; 4]) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _10: *const Adt31;
let _11: isize;
let _12: isize;
let _13: bool;
let _14: *mut [u32; 4];
let _15: bool;
let _16: u128;
let _17: *mut [u32; 4];
let _18: [u32; 3];
let _19: isize;
let _20: f32;
let _21: *const *const Adt25;
let _22: isize;
let _23: (*const f64, *mut &'static u8);
let _24: i32;
let _25: *mut [u32; 4];
let _26: char;
let _27: i8;
let _28: Adt35;
let _29: ([u32; 4],);
let _30: isize;
let _31: Adt44;
let _32: f32;
let _33: i16;
let _34: *mut [i32; 7];
let _35: *mut bool;
let _36: ();
let _37: ();
{
_2 = _9;
_8 = [2684890870143780734_u64,16106150419281820093_u64,4015012648053750919_u64,11985320821136728405_u64];
_7 = [15346230391646476272_u64,13342232806945830592_u64,10563736475667129484_u64,9344229395660243847_u64];
RET = _3;
_5 = _3;
_2 = [4738247836318156027_u64,11385708465091581617_u64,17583567241820637450_u64,8801860117902450788_u64];
_2 = [798348675466702453_u64,13661940060220294617_u64,6688940048353317512_u64,1597888545893209893_u64];
Goto(bb1)
}
bb1 = {
_4 = [10622059471682436621_u64,13685354849329549682_u64,12057758284109657068_u64,15256966799804692897_u64];
_12 = 53_isize >> 531217618_u32;
_9 = _6;
RET = [3630048488135011851_u64,1696870939345685400_u64,15623914811750776939_u64,6232426453957221289_u64,12328057792861553681_u64];
_11 = (-80_i8) as isize;
Goto(bb2)
}
bb2 = {
_12 = 641855878_i32 as isize;
_9 = [5813282842519273140_u64,16162791875493933220_u64,4409024273760435858_u64,16791631774119703766_u64];
_12 = -_11;
_1 = [16660637930800004086_u64,17099463372438375805_u64,15162021640295801556_u64,3657119954049571783_u64];
_12 = _11;
_1 = [17870826341044035845_u64,6943449039236275159_u64,16655893190986177087_u64,1397451112694085566_u64];
_3 = [13330761715642728605_u64,13654830719092997611_u64,8539359153789626189_u64,13702672599814678241_u64,8884603185786874473_u64];
_7 = [2039210894615229845_u64,166358542531045762_u64,11205865317499722946_u64,610761866508202217_u64];
_4 = [16192814080572436627_u64,12548773836176002155_u64,925670731866296651_u64,15429168650616174186_u64];
_6 = [7084426064123613294_u64,14359927442526566359_u64,4996724653304198402_u64,17817867084943391434_u64];
_9 = _2;
Goto(bb3)
}
bb3 = {
_13 = !false;
RET = [15753865518991238698_u64,2556983764613315559_u64,6445388199434171785_u64,6202990225608347096_u64,4303935069902454430_u64];
_2 = [10256046210745372815_u64,7018503423908885287_u64,15823694715950189142_u64,17814524489466569422_u64];
_7 = [17591377291240193536_u64,4611646241700350061_u64,13004003042767364351_u64,9661572872975373975_u64];
_6 = [12694410165464648619_u64,16948958669044628795_u64,2061364790950155021_u64,608280440285280156_u64];
_4 = [14903016802841620352_u64,7562363594960925803_u64,13192788894112696173_u64,333318321847457745_u64];
_1 = [10409910276614420672_u64,10408485899592333168_u64,4255696207673503538_u64,11013367465810138748_u64];
_13 = true;
_13 = false;
_4 = _7;
RET = [5365061624784888851_u64,3526820767793525545_u64,5036905425573201361_u64,14072276682428633950_u64,2844828876976580191_u64];
_1 = _4;
_13 = false;
_1 = [17754020945533971777_u64,8143130897653878883_u64,16251987294117006643_u64,13094727228837415910_u64];
_15 = _13;
_5 = RET;
_5 = [12708245553697188037_u64,644600452126362215_u64,8409707972232574103_u64,10072824791598963853_u64,11908961209337668403_u64];
Call(_5 = fn4(_15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = [704615005_u32,4162306268_u32,1441831038_u32];
_16 = 32558064439877414020652948499920730619_u128;
_6 = [3482161300844782337_u64,4329693590716230781_u64,8591692758708740728_u64,5989331374354961869_u64];
_5 = _3;
_4 = _1;
_16 = 77084377428298549102112889294983544212_u128;
_16 = '\u{722f7}' as u128;
_8 = _1;
_7 = _2;
RET = [1266545510197459781_u64,13828564247281036375_u64,3959308863526412723_u64,1540250072490884531_u64,3776473461647434945_u64];
_6 = _7;
Goto(bb5)
}
bb5 = {
_19 = _11 - _12;
_15 = !_13;
_9 = [7348128914433314294_u64,5627918030336783129_u64,5741018557918445013_u64,5287479690631421956_u64];
_8 = _4;
_18 = [418248177_u32,1288196411_u32,2868316441_u32];
Goto(bb6)
}
bb6 = {
_9 = [18378383929848013649_u64,13073462192789104956_u64,10307282302693675632_u64,258759920441847647_u64];
_3 = [16282829651623851611_u64,3932012177964478038_u64,14957497801195633462_u64,7495736938967578165_u64,15791722174377023784_u64];
_20 = _16 as f32;
_13 = _15 & _15;
Goto(bb7)
}
bb7 = {
_13 = _15 | _15;
_11 = _19 - _19;
_5 = [14911743929134833069_u64,9360603818777378103_u64,10152986181711809287_u64,4067084194483314829_u64,13361099605582709507_u64];
_6 = _2;
_5 = [3557240972569587250_u64,13259341777220640881_u64,17310642566481487783_u64,17031543819387047482_u64,12847944748018783026_u64];
_18 = [18629085_u32,1715792533_u32,3800860499_u32];
_3 = [12931356075713382735_u64,9514937949152997330_u64,13131017338089338539_u64,728554540270796359_u64,13158643406217914290_u64];
_20 = 2747931317888886963_i64 as f32;
_16 = 176400455541158387650907912755236453473_u128 + 60601482687994361909321091761593529121_u128;
_9 = _7;
_6 = _9;
_12 = !_19;
_9 = [5766860558837618563_u64,5009252815632393134_u64,638297945038250169_u64,3989516140968713646_u64];
_12 = _11;
_22 = !_19;
_22 = _13 as isize;
_19 = 1_usize as isize;
_12 = _19;
_15 = _13;
_7 = _1;
Goto(bb8)
}
bb8 = {
_7 = [11411457243364377758_u64,6797886070995230150_u64,8155764775824362754_u64,17090989738028735409_u64];
_22 = '\u{2c65f}' as isize;
_3 = RET;
RET = [10621958851798299574_u64,7484821347805521894_u64,8671373708932458501_u64,17108936040293954425_u64,5391672683144209784_u64];
_15 = !_13;
RET = [4935050024770290149_u64,3163740976118017927_u64,3113821025777446653_u64,13894120607812099697_u64,3797892040437675644_u64];
_16 = 308682980210953920139119543687268073352_u128;
_3 = [7157643525451456283_u64,9500237068341062930_u64,5467208678774209686_u64,879801326796397707_u64,17322283429979107881_u64];
_13 = !_15;
_9 = [9846041019572000441_u64,9253559243303900064_u64,1087307984341732411_u64,8739636581914968295_u64];
RET = [2001694949368009830_u64,503874596205935478_u64,9044357516127353196_u64,7161321163711061644_u64,18132432408848405441_u64];
_22 = !_11;
_18 = [1693658696_u32,2300197723_u32,1620183219_u32];
RET = _3;
_24 = 1591856513_i32 << _22;
_3 = _5;
_2 = [11611221128079203254_u64,4950037927499191815_u64,3867582664793501517_u64,4216251254634128241_u64];
match _16 {
0 => bb1,
1 => bb5,
2 => bb3,
308682980210953920139119543687268073352 => bb9,
_ => bb4
}
}
bb9 = {
_3 = RET;
_7 = [9638927431515464304_u64,15808134243335187092_u64,11837739222210402239_u64,16684889101589625945_u64];
_24 = (-417134410_i32);
RET = [8799674204830282779_u64,12094380179082332671_u64,17397505332028644950_u64,3937331350259395157_u64,17211569376365410851_u64];
_4 = [14430277577649586595_u64,14876129131891390233_u64,13372632734255398377_u64,9202649206735572742_u64];
_26 = '\u{4915}';
_9 = _1;
_22 = (-475498703875016827_i64) as isize;
_16 = 10855784529780476574211813907709944848_u128 ^ 160239966957768311600003758305245786509_u128;
_16 = 3357010001018866498215354497362736756_u128 << _19;
_27 = -(-59_i8);
RET = [13432492040381931181_u64,14150498630447168890_u64,1695848652134931563_u64,17811270940170395694_u64,14786808823952395369_u64];
_18 = [2157620148_u32,4148831090_u32,197127382_u32];
_13 = _15;
_30 = _22 - _11;
_2 = [17169670560918099356_u64,18048715460863779891_u64,5439529176754356686_u64,3973795459875858912_u64];
_20 = _27 as f32;
_3 = _5;
_9 = _8;
_25 = core::ptr::addr_of_mut!(_29.0);
_9 = _4;
_4 = [14087186430797863159_u64,10838145283751375701_u64,6850822481566907961_u64,5025867149303415719_u64];
_18 = [998138295_u32,2602216933_u32,2206285889_u32];
(*_25) = [917127893_u32,558024266_u32,4229371991_u32,2329372246_u32];
_4 = _6;
Call(_11 = core::intrinsics::transmute(_30), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = Move(_25);
_24 = -628991867_i32;
Goto(bb11)
}
bb11 = {
_16 = 239_u8 as u128;
Goto(bb12)
}
bb12 = {
_4 = [1927175754956473664_u64,9239626190591031455_u64,4761063334408027893_u64,1367216406484030998_u64];
_9 = [7390934928176702878_u64,2289967016295488374_u64,13307934672148543686_u64,14087602577319404564_u64];
_27 = (-1750930417326499483_i64) as i8;
_17 = core::ptr::addr_of_mut!(_29.0);
_20 = _12 as f32;
_8 = _2;
_12 = _11;
_14 = Move(_17);
_25 = core::ptr::addr_of_mut!(_29.0);
_14 = Move(_25);
_22 = -_12;
_2 = _8;
_3 = [15624324616923813885_u64,18153250778764475506_u64,2418315471768860977_u64,12629757624526048124_u64,6505350606888408109_u64];
_22 = _30;
_4 = _1;
_6 = [10281038175661339371_u64,8362799471075394014_u64,16602650391879635617_u64,17178354668936279267_u64];
_29.0 = [1923697672_u32,2004997058_u32,1032659893_u32,2335961688_u32];
Goto(bb13)
}
bb13 = {
_9 = _7;
_12 = !_22;
_5 = [8020048085178529858_u64,11155152086620478187_u64,5919601024451192332_u64,12091937205969750068_u64,5812841822267981141_u64];
_9 = [7006378843268390195_u64,3574281192900113254_u64,8668948082287443615_u64,10916116427285757674_u64];
_24 = _26 as i32;
_4 = [15406600613623575990_u64,4888021624692792544_u64,7704389404426659100_u64,681426108929437307_u64];
_14 = core::ptr::addr_of_mut!(_29.0);
_29.0 = [2978406165_u32,3581249633_u32,2269739942_u32,1545886815_u32];
RET = [13545194301378656272_u64,6044798105788160348_u64,10724590682713929046_u64,8275380089815754626_u64,16613043110598408754_u64];
_25 = core::ptr::addr_of_mut!((*_14));
_20 = 33_u8 as f32;
(*_25) = [353461528_u32,3343216399_u32,1847673841_u32,2880891339_u32];
_32 = _20;
_27 = (-99_i8);
(*_25) = [1418861155_u32,1795201617_u32,1021347455_u32,3811829652_u32];
_25 = Move(_14);
_6 = [12963639486488069444_u64,13825175829084901864_u64,9560608681554640906_u64,13242032405768549070_u64];
_22 = -_12;
match _27 {
0 => bb8,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb14,
340282366920938463463374607431768211357 => bb16,
_ => bb15
}
}
bb14 = {
_4 = [1927175754956473664_u64,9239626190591031455_u64,4761063334408027893_u64,1367216406484030998_u64];
_9 = [7390934928176702878_u64,2289967016295488374_u64,13307934672148543686_u64,14087602577319404564_u64];
_27 = (-1750930417326499483_i64) as i8;
_17 = core::ptr::addr_of_mut!(_29.0);
_20 = _12 as f32;
_8 = _2;
_12 = _11;
_14 = Move(_17);
_25 = core::ptr::addr_of_mut!(_29.0);
_14 = Move(_25);
_22 = -_12;
_2 = _8;
_3 = [15624324616923813885_u64,18153250778764475506_u64,2418315471768860977_u64,12629757624526048124_u64,6505350606888408109_u64];
_22 = _30;
_4 = _1;
_6 = [10281038175661339371_u64,8362799471075394014_u64,16602650391879635617_u64,17178354668936279267_u64];
_29.0 = [1923697672_u32,2004997058_u32,1032659893_u32,2335961688_u32];
Goto(bb13)
}
bb15 = {
_13 = _15 | _15;
_11 = _19 - _19;
_5 = [14911743929134833069_u64,9360603818777378103_u64,10152986181711809287_u64,4067084194483314829_u64,13361099605582709507_u64];
_6 = _2;
_5 = [3557240972569587250_u64,13259341777220640881_u64,17310642566481487783_u64,17031543819387047482_u64,12847944748018783026_u64];
_18 = [18629085_u32,1715792533_u32,3800860499_u32];
_3 = [12931356075713382735_u64,9514937949152997330_u64,13131017338089338539_u64,728554540270796359_u64,13158643406217914290_u64];
_20 = 2747931317888886963_i64 as f32;
_16 = 176400455541158387650907912755236453473_u128 + 60601482687994361909321091761593529121_u128;
_9 = _7;
_6 = _9;
_12 = !_19;
_9 = [5766860558837618563_u64,5009252815632393134_u64,638297945038250169_u64,3989516140968713646_u64];
_12 = _11;
_22 = !_19;
_22 = _13 as isize;
_19 = 1_usize as isize;
_12 = _19;
_15 = _13;
_7 = _1;
Goto(bb8)
}
bb16 = {
_19 = !_12;
_12 = _11;
_32 = _20;
_11 = -_12;
_1 = [4187308528347944950_u64,10917180818815169716_u64,4538333622863933466_u64,370244916900429003_u64];
_29.0 = [1443713712_u32,1732798204_u32,3183764942_u32,3877997269_u32];
_25 = core::ptr::addr_of_mut!(_29.0);
_33 = 29809_i16 | (-3117_i16);
_24 = 1311173565_i32;
_7 = [12902546881361130586_u64,7694285781612409291_u64,4335262437192006505_u64,6455249050493464716_u64];
_6 = [18213794757883779104_u64,13435735007786321684_u64,13028779744934979286_u64,4749470749430969149_u64];
_26 = '\u{c3e7d}';
_26 = '\u{18e49}';
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(3_usize, 24_usize, Move(_24), 13_usize, Move(_13), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(3_usize, 4_usize, Move(_4), 33_usize, Move(_33), 30_usize, Move(_30), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(3_usize, 3_usize, Move(_3), 2_usize, Move(_2), 22_usize, Move(_22), 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _2: *mut bool;
let _3: (&'static *const f64,);
let _4: Adt44;
let _5: isize;
let _6: isize;
let _7: &'static &'static u16;
let _8: char;
let _9: i16;
let _10: f32;
let _11: u16;
let _12: [u64; 4];
let _13: f32;
let _14: u128;
let _15: isize;
let _16: *mut usize;
let _17: Adt25;
let _18: *mut usize;
let _19: [u128; 7];
let _20: [u64; 5];
let _21: i64;
let _22: isize;
let _23: i32;
let _24: char;
let _25: &'static f32;
let _26: u32;
let _27: f32;
let _28: Adt38;
let _29: ();
let _30: ();
{
_1 = !false;
RET = [8546093876010557103_u64,13108495609281565996_u64,4807525816120879691_u64,11592138395292953019_u64,12157194830794119652_u64];
RET = [14777311314513327568_u64,16798127535610935601_u64,18100824687958566006_u64,18243638963742465193_u64,18156501875429229847_u64];
_1 = !false;
_2 = core::ptr::addr_of_mut!(_1);
RET = [14684207344754760239_u64,7589849435177298752_u64,15489600067489053525_u64,2711703089580940208_u64,2206946918088358998_u64];
RET = [4563352065117004114_u64,3516765746631055121_u64,4577378120486009287_u64,5797493302919286037_u64,12712866554202592425_u64];
(*_2) = true;
(*_2) = false;
(*_2) = !true;
_4 = Adt44::Variant2 { fld0: RET };
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = RET;
_4 = Adt44::Variant2 { fld0: RET };
_5 = (-9223372036854775808_isize) >> 9223372036854775807_isize;
RET = [1212357542223306847_u64,2161905674586210868_u64,15194590255721554019_u64,4098059972216590042_u64,5869464448578136790_u64];
_2 = core::ptr::addr_of_mut!(_1);
RET = [7539130976177591244_u64,2339115355640775781_u64,10856641821032590717_u64,17982863130543045288_u64,11917148655572727963_u64];
(*_2) = false;
RET = [7183998994422374622_u64,25842990159322055_u64,10433623894200193315_u64,194729901517825204_u64,13766715995790090099_u64];
_6 = _5;
_2 = core::ptr::addr_of_mut!((*_2));
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = RET;
Goto(bb1)
}
bb1 = {
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [6633482400288939955_u64,3685412856342825741_u64,1921776967478317017_u64,6092828289899181743_u64,18100612998092839139_u64];
_4 = Adt44::Variant2 { fld0: RET };
RET = [7205737211487381704_u64,4050940049567500689_u64,3445051727444623707_u64,8190614433421875847_u64,17119642734434384987_u64];
_10 = 4138398645_u32 as f32;
_8 = '\u{f8334}';
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [14279064407798762753_u64,18446153079082812628_u64,7855286248413198241_u64,3754575185669571476_u64,5538525274220175597_u64];
RET = [3780570805430006689_u64,16402695084224175_u64,14709352014246341719_u64,10118099458857774537_u64,10369912648016201452_u64];
(*_2) = !true;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
_10 = 94896049355607646117103625131808872333_u128 as f32;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
_2 = core::ptr::addr_of_mut!((*_2));
_5 = _6 >> _6;
_9 = 215448542477224637321306929397570779945_u128 as i16;
_8 = '\u{9f042}';
_11 = 33721_u16;
_1 = false ^ false;
_1 = _8 <= _8;
(*_2) = true & true;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
33721 => bb7,
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
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [14818045654307126550_u64,13781749585895234575_u64,16827830842720699660_u64,17337961504513242069_u64,3109386336733046087_u64];
RET = [9794531413449951638_u64,14725064846017479409_u64,13689707397952086814_u64,8887662486037086465_u64,3348210372303505333_u64];
_13 = _10 + _10;
_6 = _5;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
_6 = _5;
_14 = 42429641659927640778596641707325312613_u128;
(*_2) = false;
_15 = _6;
_9 = 22179_i16;
_1 = !false;
_9 = (-21537_i16);
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
_14 = !65262133465447164217081595519522262089_u128;
(*_2) = _5 >= _6;
Goto(bb8)
}
bb8 = {
_2 = core::ptr::addr_of_mut!((*_2));
_9 = 168_u8 as i16;
_1 = _8 < _8;
_10 = 3213363468_u32 as f32;
_9 = (-12279_i16) ^ 12811_i16;
_8 = '\u{1014d5}';
_15 = _6;
(*_2) = false;
_14 = !300490192013643464393714775835603314685_u128;
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = RET;
RET = [16954197181552908548_u64,10685614678840243803_u64,1735939626956368112_u64,9512954207078635647_u64,14275017983509424526_u64];
_19 = [_14,_14,_14,_14,_14,_14,_14];
SetDiscriminant(_4, 1);
RET = [17799982661110748256_u64,15892837112195558771_u64,9405211910065443004_u64,4819136546286309959_u64,1782560189031091477_u64];
_14 = !139944263919609421575728368949476587089_u128;
_21 = 6254111962318330072_i64;
_9 = 171_u8 as i16;
_9 = 672_i16 ^ (-3430_i16);
Goto(bb9)
}
bb9 = {
RET = [10316150823238062880_u64,14916261519069113329_u64,13808978948581921081_u64,4739193241898470024_u64,15314759855302833171_u64];
_9 = 16103_i16;
Call(_19 = fn5(_14, _6, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = _6;
_12 = [578007922170044833_u64,2625793656135248615_u64,12071321819165547697_u64,13843295522353504992_u64];
_5 = 38_u8 as isize;
_1 = !false;
_14 = 307464827353678219195063247215604709593_u128 * 14397896462147305800607233613190485655_u128;
_11 = 1281026681_u32 as u16;
_1 = false;
RET = [17674446061211105849_u64,5474687835510459107_u64,9480066969093372939_u64,1143359807566657806_u64,2003093987028361107_u64];
_20 = [12164988407573621705_u64,10373999152912962246_u64,10344504966083899398_u64,971577501860650631_u64,12577004475076510243_u64];
_12 = [8188931386410629364_u64,14745003204808868499_u64,8502653584740977802_u64,11270978865822984442_u64];
_1 = _15 != _15;
_8 = '\u{a49d5}';
_1 = !true;
_8 = '\u{ed53f}';
_11 = 54536_u16 ^ 10990_u16;
_22 = _6 - _6;
_8 = '\u{e1f0d}';
RET = [16825762440256897073_u64,8482350340589742823_u64,6988574679297685953_u64,4063410700043034152_u64,15147266741007001924_u64];
RET = [7254191956189970252_u64,4185799067132414195_u64,12183474554020735862_u64,15170412686469687949_u64,5173743758276443376_u64];
(*_2) = false;
RET = _20;
_25 = &_13;
_26 = 2820745431_u32 >> _22;
_1 = _13 > _13;
_26 = !3724028354_u32;
(*_2) = false;
_27 = _13;
Goto(bb11)
}
bb11 = {
(*_2) = true;
RET = _20;
_10 = (*_25);
_4 = Adt44::Variant2 { fld0: _20 };
_19 = [_14,_14,_14,_14,_14,_14,_14];
RET = [15393332813613287373_u64,3916420589699552541_u64,17684358028206716397_u64,5775435157511498764_u64,1228446035149535557_u64];
match _21 {
0 => bb7,
1 => bb9,
2 => bb5,
3 => bb12,
4 => bb13,
6254111962318330072 => bb15,
_ => bb14
}
}
bb12 = {
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [6633482400288939955_u64,3685412856342825741_u64,1921776967478317017_u64,6092828289899181743_u64,18100612998092839139_u64];
_4 = Adt44::Variant2 { fld0: RET };
RET = [7205737211487381704_u64,4050940049567500689_u64,3445051727444623707_u64,8190614433421875847_u64,17119642734434384987_u64];
_10 = 4138398645_u32 as f32;
_8 = '\u{f8334}';
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [14279064407798762753_u64,18446153079082812628_u64,7855286248413198241_u64,3754575185669571476_u64,5538525274220175597_u64];
RET = [3780570805430006689_u64,16402695084224175_u64,14709352014246341719_u64,10118099458857774537_u64,10369912648016201452_u64];
(*_2) = !true;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
_10 = 94896049355607646117103625131808872333_u128 as f32;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
_2 = core::ptr::addr_of_mut!((*_2));
_5 = _6 >> _6;
_9 = 215448542477224637321306929397570779945_u128 as i16;
_8 = '\u{9f042}';
_11 = 33721_u16;
_1 = false ^ false;
_1 = _8 <= _8;
(*_2) = true & true;
RET = Field::<[u64; 5]>(Variant(_4, 2), 0);
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
33721 => bb7,
_ => bb6
}
}
bb13 = {
Return()
}
bb14 = {
_2 = core::ptr::addr_of_mut!((*_2));
_9 = 168_u8 as i16;
_1 = _8 < _8;
_10 = 3213363468_u32 as f32;
_9 = (-12279_i16) ^ 12811_i16;
_8 = '\u{1014d5}';
_15 = _6;
(*_2) = false;
_14 = !300490192013643464393714775835603314685_u128;
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = RET;
RET = [16954197181552908548_u64,10685614678840243803_u64,1735939626956368112_u64,9512954207078635647_u64,14275017983509424526_u64];
_19 = [_14,_14,_14,_14,_14,_14,_14];
SetDiscriminant(_4, 1);
RET = [17799982661110748256_u64,15892837112195558771_u64,9405211910065443004_u64,4819136546286309959_u64,1782560189031091477_u64];
_14 = !139944263919609421575728368949476587089_u128;
_21 = 6254111962318330072_i64;
_9 = 171_u8 as i16;
_9 = 672_i16 ^ (-3430_i16);
Goto(bb9)
}
bb15 = {
_5 = _15;
_11 = 51170_u16;
_24 = _8;
_9 = (-22391_i16) >> _6;
RET = _20;
_26 = 2722184163_u32;
_9 = 22038_i16 * 31603_i16;
_12 = [772288726141665769_u64,15605389705155142548_u64,6768483422536262922_u64,8619560859919605004_u64];
place!(Field::<[u64; 5]>(Variant(_4, 2), 0)) = [10109235487539166975_u64,11525075422391604893_u64,14453403494666774238_u64,8567834573422968319_u64,7116703081708280811_u64];
_21 = (-3536290378533646278_i64) + 6001053144379997261_i64;
_28.fld0 = [56_i8,(-43_i8),41_i8];
_14 = 431172594_i32 as u128;
SetDiscriminant(_4, 0);
place!(Field::<u16>(Variant(_4, 0), 0)) = !_11;
_23 = _21 as i32;
_14 = 304588418209963019700983427191789634873_u128;
(*_2) = false & false;
place!(Field::<[u32; 2]>(Variant(_4, 0), 2)) = [_26,_26];
_25 = &_13;
_2 = core::ptr::addr_of_mut!((*_2));
place!(Field::<u32>(Variant(_4, 0), 1)) = !_26;
place!(Field::<[u32; 2]>(Variant(_4, 0), 2)) = [_26,_26];
place!(Field::<[u32; 4]>(Variant(_4, 0), 3)) = [_26,Field::<u32>(Variant(_4, 0), 1),Field::<u32>(Variant(_4, 0), 1),Field::<u32>(Variant(_4, 0), 1)];
(*_2) = true;
_27 = _23 as f32;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(4_usize, 23_usize, Move(_23), 6_usize, Move(_6), 12_usize, Move(_12), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(4_usize, 8_usize, Move(_8), 20_usize, Move(_20), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u128,mut _2: isize,mut _3: isize) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _4: i128;
let _5: i64;
let _6: [u16; 4];
let _7: *mut Adt25;
let _8: u16;
let _9: isize;
let _10: u8;
let _11: (*const f64, *mut &'static u8);
let _12: [i32; 7];
let _13: isize;
let _14: &'static (Adt31,);
let _15: u64;
let _16: i16;
let _17: &'static *const f64;
let _18: bool;
let _19: f32;
let _20: *const Adt25;
let _21: u8;
let _22: [u32; 4];
let _23: (Adt31,);
let _24: isize;
let _25: [u32; 7];
let _26: f64;
let _27: *mut [u32; 4];
let _28: [u32; 7];
let _29: ();
let _30: ();
{
_2 = _3 & _3;
RET = [_1,_1,_1,_1,_1,_1,_1];
_2 = -_3;
_2 = -_3;
RET = [_1,_1,_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1,_1,_1];
Call(_2 = fn6(_1, RET, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_1,_1,_1,_1,_1,_1,_1];
_1 = !261533822005855048566920646290196087116_u128;
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = 709265018533000181_i64;
RET = [_1,_1,_1,_1,_1,_1,_1];
_1 = 197984920596818841142655929339865791491_u128 - 214985454556676650971733836137934725285_u128;
_5 = 508759828178480163_i64 - 927640281435991045_i64;
_2 = (-80_i8) as isize;
_4 = 5111694301272523417890009680884732963_i128;
_4 = -(-29115134576218824656996452334882523803_i128);
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = _3 as i128;
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = (-9277612078919283818901893910564997975_i128);
_5 = !(-7458416653068191371_i64);
Call(RET = fn7(_3, _3, _3, _3, _3, _1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = -(-82425351380351403490117159488126821680_i128);
RET = [_1,_1,_1,_1,_1,_1,_1];
_3 = _2;
_3 = _2 >> _1;
_6 = [20617_u16,62359_u16,16431_u16,3133_u16];
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = -(-2460760484162553585_i64);
_6 = [49693_u16,4226_u16,33327_u16,37413_u16];
RET = [_1,_1,_1,_1,_1,_1,_1];
_2 = 57_u8 as isize;
RET = [_1,_1,_1,_1,_1,_1,_1];
_2 = !_3;
_6 = [43151_u16,2505_u16,41016_u16,48396_u16];
_3 = (-895855075_i32) as isize;
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = 1538788832_u32 as i64;
_8 = 54365_u16;
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = (-123141763298211090611861344502847730122_i128);
_5 = 2056977694817945271_i64 + 160989208817756682_i64;
Goto(bb3)
}
bb3 = {
_9 = _2 ^ _2;
_9 = -_3;
_2 = _3;
RET = [_1,_1,_1,_1,_1,_1,_1];
_1 = 239068899819526545542526669337695913885_u128;
_12 = [2072575939_i32,966773708_i32,(-227492124_i32),1190871580_i32,1540251058_i32,102483531_i32,(-176762364_i32)];
_6 = [_8,_8,_8,_8];
_5 = (-1173893542424536431_i64);
RET = [_1,_1,_1,_1,_1,_1,_1];
_10 = 250_u8 - 147_u8;
_10 = !182_u8;
RET = [_1,_1,_1,_1,_1,_1,_1];
_12 = [949786423_i32,1375624599_i32,(-1817462650_i32),(-1022931354_i32),246998641_i32,229809876_i32,143717378_i32];
RET = [_1,_1,_1,_1,_1,_1,_1];
_6 = [_8,_8,_8,_8];
_13 = _3 + _2;
_1 = (-7344_i16) as u128;
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = (-14272951741223376274647349364069471716_i128) << _2;
_9 = _10 as isize;
RET = [_1,_1,_1,_1,_1,_1,_1];
_8 = (-1025499866_i32) as u16;
Goto(bb4)
}
bb4 = {
_6 = [_8,_8,_8,_8];
RET = [_1,_1,_1,_1,_1,_1,_1];
_12 = [(-2124195523_i32),1451246148_i32,716463591_i32,(-2021893286_i32),1897035531_i32,932006465_i32,(-2030830404_i32)];
_17 = &_11.0;
_18 = !true;
_8 = !60572_u16;
_12 = [1432228620_i32,318209414_i32,(-335296675_i32),(-709466853_i32),(-337824074_i32),2066179120_i32,1224790400_i32];
Goto(bb5)
}
bb5 = {
_8 = 62046_u16;
_16 = 402595201_u32 as i16;
_17 = &(*_17);
_6 = [_8,_8,_8,_8];
_3 = _2 - _2;
_2 = _13 + _9;
_12 = [348455025_i32,612142456_i32,664441949_i32,1030189438_i32,978523109_i32,2017671827_i32,1923422664_i32];
_17 = &(*_17);
_13 = _3;
_4 = !153902532988434998440605692122753126343_i128;
_5 = (-4177696015443932842_i64) | (-813787079329346668_i64);
_9 = -_2;
_17 = &(*_17);
_4 = 61360433092143590581389804201442413802_i128 | 38121786665493087493905204044868665271_i128;
_19 = 2661820431_u32 as f32;
Call(_2 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _9 << _9;
match _8 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb9,
62046 => bb11,
_ => bb10
}
}
bb7 = {
_8 = 62046_u16;
_16 = 402595201_u32 as i16;
_17 = &(*_17);
_6 = [_8,_8,_8,_8];
_3 = _2 - _2;
_2 = _13 + _9;
_12 = [348455025_i32,612142456_i32,664441949_i32,1030189438_i32,978523109_i32,2017671827_i32,1923422664_i32];
_17 = &(*_17);
_13 = _3;
_4 = !153902532988434998440605692122753126343_i128;
_5 = (-4177696015443932842_i64) | (-813787079329346668_i64);
_9 = -_2;
_17 = &(*_17);
_4 = 61360433092143590581389804201442413802_i128 | 38121786665493087493905204044868665271_i128;
_19 = 2661820431_u32 as f32;
Call(_2 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_6 = [_8,_8,_8,_8];
RET = [_1,_1,_1,_1,_1,_1,_1];
_12 = [(-2124195523_i32),1451246148_i32,716463591_i32,(-2021893286_i32),1897035531_i32,932006465_i32,(-2030830404_i32)];
_17 = &_11.0;
_18 = !true;
_8 = !60572_u16;
_12 = [1432228620_i32,318209414_i32,(-335296675_i32),(-709466853_i32),(-337824074_i32),2066179120_i32,1224790400_i32];
Goto(bb5)
}
bb9 = {
RET = [_1,_1,_1,_1,_1,_1,_1];
_1 = !261533822005855048566920646290196087116_u128;
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = 709265018533000181_i64;
RET = [_1,_1,_1,_1,_1,_1,_1];
_1 = 197984920596818841142655929339865791491_u128 - 214985454556676650971733836137934725285_u128;
_5 = 508759828178480163_i64 - 927640281435991045_i64;
_2 = (-80_i8) as isize;
_4 = 5111694301272523417890009680884732963_i128;
_4 = -(-29115134576218824656996452334882523803_i128);
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = _3 as i128;
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = (-9277612078919283818901893910564997975_i128);
_5 = !(-7458416653068191371_i64);
Call(RET = fn7(_3, _3, _3, _3, _3, _1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_4 = -(-82425351380351403490117159488126821680_i128);
RET = [_1,_1,_1,_1,_1,_1,_1];
_3 = _2;
_3 = _2 >> _1;
_6 = [20617_u16,62359_u16,16431_u16,3133_u16];
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = -(-2460760484162553585_i64);
_6 = [49693_u16,4226_u16,33327_u16,37413_u16];
RET = [_1,_1,_1,_1,_1,_1,_1];
_2 = 57_u8 as isize;
RET = [_1,_1,_1,_1,_1,_1,_1];
_2 = !_3;
_6 = [43151_u16,2505_u16,41016_u16,48396_u16];
_3 = (-895855075_i32) as isize;
RET = [_1,_1,_1,_1,_1,_1,_1];
_5 = 1538788832_u32 as i64;
_8 = 54365_u16;
RET = [_1,_1,_1,_1,_1,_1,_1];
_4 = (-123141763298211090611861344502847730122_i128);
_5 = 2056977694817945271_i64 + 160989208817756682_i64;
Goto(bb3)
}
bb11 = {
_6 = [_8,_8,_8,_8];
_17 = &(*_17);
_17 = &(*_17);
_1 = 18834109042819454711446122448876926578_u128 & 122951458679016820710874109872050200890_u128;
_8 = 13_i8 as u16;
Goto(bb12)
}
bb12 = {
_1 = _10 as u128;
_13 = (-52_i8) as isize;
_15 = 678333454_i32 as u64;
_18 = !false;
_4 = !35064436390676357515012554926611066048_i128;
_17 = &_11.0;
_3 = -_2;
_12 = [1397663532_i32,(-1935381217_i32),1543014385_i32,(-346840209_i32),987348896_i32,(-154856013_i32),(-984306886_i32)];
_12 = [108429658_i32,784773661_i32,451795748_i32,804640866_i32,(-1302714512_i32),1090836616_i32,1928202946_i32];
_18 = true;
_21 = !_10;
_8 = '\u{7b884}' as u16;
_5 = -(-8489147797882305720_i64);
_10 = !_21;
_15 = !351099164373075256_u64;
_14 = &_23;
_10 = !_21;
_14 = &(*_14);
_5 = -(-5276989259454836349_i64);
_23.0 = Adt31::Variant2 { fld0: _18,fld1: 2_usize,fld2: _3 };
_5 = _4 as i64;
_3 = _2;
_3 = _2;
_24 = _3;
_14 = &_23;
_15 = !14839840525429874952_u64;
_8 = 9520964085160731008_usize as u16;
Goto(bb13)
}
bb13 = {
_8 = !65340_u16;
_22 = [153568566_u32,3267048994_u32,1921993728_u32,2499741992_u32];
_17 = &_11.0;
_17 = &(*_17);
_2 = !Field::<isize>(Variant((*_14).0, 2), 2);
_18 = Field::<bool>(Variant((*_14).0, 2), 0);
_17 = &(*_17);
_24 = Field::<isize>(Variant(_23.0, 2), 2) & _9;
_25 = [894389991_u32,2907138461_u32,2570773307_u32,3362442813_u32,1918925798_u32,2110133656_u32,820558561_u32];
_13 = !_9;
Call(_1 = core::intrinsics::transmute(_22), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<isize>(Variant(_23.0, 2), 2)) = 1_usize as isize;
_23.0 = Adt31::Variant2 { fld0: _18,fld1: 2_usize,fld2: _13 };
place!(Field::<bool>(Variant(_23.0, 2), 0)) = _18;
_2 = _5 as isize;
_2 = _9;
_18 = !Field::<bool>(Variant(_23.0, 2), 0);
_22 = [173022647_u32,2308476716_u32,496471291_u32,3859415522_u32];
RET = [_1,_1,_1,_1,_1,_1,_1];
_3 = _24;
_12 = [1271714055_i32,1749709082_i32,343996568_i32,(-1547131334_i32),995600007_i32,598513696_i32,1481403642_i32];
_13 = _3 & Field::<isize>(Variant(_23.0, 2), 2);
_14 = &_23;
_13 = _3 >> _3;
_8 = (-1158149384_i32) as u16;
_25 = [2637105151_u32,2505528777_u32,4253981059_u32,3091320776_u32,1449223076_u32,1537655103_u32,3306253749_u32];
_25 = [1099829676_u32,2933168080_u32,2172411690_u32,2771235547_u32,4033653840_u32,2125406743_u32,1073431209_u32];
_27 = core::ptr::addr_of_mut!(_22);
_3 = !Field::<isize>(Variant(_23.0, 2), 2);
place!(Field::<isize>(Variant(_23.0, 2), 2)) = _2;
_14 = &_23;
_1 = !190808966625601549350621147112750172009_u128;
(*_27) = [1013218384_u32,3636404318_u32,3942537999_u32,659276041_u32];
_1 = 216796538521922221563981482537508504671_u128 | 138326173423770162888452070975527167455_u128;
_16 = !9060_i16;
_8 = 44521_u16;
_9 = _13 | _13;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(5_usize, 18_usize, Move(_18), 12_usize, Move(_12), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(5_usize, 10_usize, Move(_10), 22_usize, Move(_22), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 9_usize, Move(_9), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: u128,mut _2: [u128; 7],mut _3: isize,mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: i8;
let _7: *mut &'static f32;
let _8: ();
let _9: ();
{
_5 = -_4;
RET = false as isize;
_2 = [_1,_1,_1,_1,_1,_1,_1];
_6 = (-1_i8) ^ 99_i8;
_4 = !_5;
_3 = _4 >> _4;
_4 = !_3;
_3 = _4 ^ _5;
_5 = !_4;
_3 = _5 | _4;
_6 = '\u{e8f94}' as i8;
_6 = !115_i8;
RET = 15185_i16 as isize;
_4 = _5 - _3;
_2 = [_1,_1,_1,_1,_1,_1,_1];
_1 = 219461481660453465022864415792523233005_u128 & 280387279190005664406942142504186236749_u128;
_4 = _3;
RET = -_4;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(6_usize, 1_usize, Move(_1), 6_usize, Move(_6), 5_usize, Move(_5), 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u128,mut _7: isize) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _8: (([u32; 4],), bool, (&'static *const f64,));
let _9: (Adt38, *mut [u32; 4]);
let _10: char;
let _11: *mut i32;
let _12: ([u32; 4],);
let _13: bool;
let _14: f32;
let _15: *mut Adt25;
let _16: char;
let _17: *mut bool;
let _18: &'static *const (&'static *const f64,);
let _19: &'static u16;
let _20: bool;
let _21: &'static (u8,);
let _22: [i32; 7];
let _23: f32;
let _24: isize;
let _25: isize;
let _26: i16;
let _27: f32;
let _28: ();
let _29: ();
{
_6 = !112241516198055902619434817688868526231_u128;
Goto(bb1)
}
bb1 = {
_4 = _5;
_7 = 41895_u16 as isize;
_4 = _5;
RET = [_6,_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6,_6];
_2 = _4 + _4;
_6 = (-6121520784039369064_i64) as u128;
_5 = !_3;
_7 = '\u{d2e48}' as isize;
RET = [_6,_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6,_6];
_3 = _5 & _5;
_1 = _4;
_7 = 17918243201904171620_usize as isize;
Goto(bb2)
}
bb2 = {
_5 = _3;
_2 = true as isize;
_2 = _5 >> _3;
_4 = _1;
Goto(bb3)
}
bb3 = {
_2 = -_3;
RET = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb4)
}
bb4 = {
_9.0.fld1 = [5425036626125239851_u64,8486590938610400610_u64,17989867993055288704_u64,1590770378031092658_u64,16175241378267401004_u64];
_1 = 4_usize as isize;
_8.1 = false ^ false;
_7 = 47042090284505910595000833274083024219_i128 as isize;
Goto(bb5)
}
bb5 = {
_9.0.fld0 = [(-69_i8),105_i8,(-106_i8)];
_9.1 = core::ptr::addr_of_mut!(_8.0.0);
_8.0.0 = [1103380_u32,4237151827_u32,817861897_u32,635360648_u32];
_9.1 = core::ptr::addr_of_mut!(_8.0.0);
_10 = '\u{cb737}';
_8.0.0 = [1175824138_u32,1277658337_u32,571092930_u32,4047515412_u32];
_12.0 = _8.0.0;
_7 = 9279404083915680347_u64 as isize;
_6 = 118676643265140681868809976284645289994_u128;
Goto(bb6)
}
bb6 = {
_9.0.fld0 = [(-111_i8),(-37_i8),3_i8];
_9.0.fld1 = [12537324262514820430_u64,14500799462847729603_u64,213263487241566530_u64,8482395403988534783_u64,2378581585045167344_u64];
_10 = '\u{99bf1}';
_9.0.fld0 = [(-43_i8),(-41_i8),13_i8];
_12.0 = [402884299_u32,1102878013_u32,1823519176_u32,2052484700_u32];
RET = [_6,_6,_6,_6,_6,_6,_6];
_5 = _4 << _3;
RET = [_6,_6,_6,_6,_6,_6,_6];
_6 = !214383204939982910663232671379933864356_u128;
_12.0 = [1581565817_u32,1343302816_u32,1570144058_u32,1282746126_u32];
_7 = _5;
_12.0 = [32299403_u32,40602006_u32,2748386882_u32,834527184_u32];
RET = [_6,_6,_6,_6,_6,_6,_6];
_8.0.0 = [329970845_u32,3374195082_u32,1003295817_u32,1423952620_u32];
_9.1 = core::ptr::addr_of_mut!(_8.0.0);
_13 = _4 > _7;
_10 = '\u{469d7}';
_6 = 208993946108313128591403669072828988876_u128;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9.0.fld0 = [(-63_i8),82_i8,45_i8];
_14 = (-935712586_i32) as f32;
_13 = _7 < _5;
_8.1 = !_13;
_10 = '\u{3574}';
RET = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb7)
}
bb7 = {
_16 = _10;
match _6 {
0 => bb6,
208993946108313128591403669072828988876 => bb8,
_ => bb4
}
}
bb8 = {
_9.0.fld0 = [(-42_i8),115_i8,75_i8];
RET = [_6,_6,_6,_6,_6,_6,_6];
_16 = _10;
_8.0.0 = [4271066184_u32,1704906328_u32,3999682054_u32,2186107719_u32];
_4 = !_2;
_10 = _16;
_12.0 = [3643185042_u32,745853576_u32,80192437_u32,1897572082_u32];
_1 = _10 as isize;
_12.0 = [1197717956_u32,3630877263_u32,3058761301_u32,1375102791_u32];
_8.1 = _13 & _13;
_10 = _16;
_13 = !_8.1;
_20 = _13;
_2 = _14 as isize;
Call(_4 = fn8(), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = _16;
_8.0.0 = [4223571741_u32,2901510386_u32,1218698105_u32,3583299259_u32];
_9.0.fld1 = [12188611334693266114_u64,3709362763791047418_u64,11902963651151733242_u64,15790130514723988634_u64,2660773787401785392_u64];
_16 = _10;
_23 = 37936_u16 as f32;
_7 = _5;
_6 = _8.1 as u128;
_3 = _20 as isize;
_1 = _5;
_12.0 = _8.0.0;
_25 = -_3;
_14 = _23 * _23;
_9.0.fld1 = [1492420072589859775_u64,13871463579819269003_u64,4888854580758879246_u64,12291055136466401987_u64,35263084808451391_u64];
_8.0.0 = [498792023_u32,1653638849_u32,2358234788_u32,2452131384_u32];
_8.0.0 = [2109459401_u32,756664333_u32,2082953600_u32,62410327_u32];
_3 = _25;
_16 = _10;
_24 = _2;
_9.0.fld1 = [9014353060245996810_u64,4528989472924377974_u64,17976534148809763973_u64,9286289028395486049_u64,6128275746758726486_u64];
RET = [_6,_6,_6,_6,_6,_6,_6];
_2 = _25 & _25;
_26 = -22249_i16;
RET = [_6,_6,_6,_6,_6,_6,_6];
_22 = [1685472681_i32,(-1574387778_i32),1861397711_i32,1564115423_i32,1288182468_i32,670812775_i32,(-74061195_i32)];
_8.1 = !_20;
Goto(bb10)
}
bb10 = {
Call(_28 = dump_var(7_usize, 4_usize, Move(_4), 25_usize, Move(_25), 13_usize, Move(_13), 26_usize, Move(_26)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_28 = dump_var(7_usize, 20_usize, Move(_20), 10_usize, Move(_10), 24_usize, Move(_24), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8() -> isize {
mir! {
type RET = isize;
let _1: [u16; 4];
let _2: char;
let _3: isize;
let _4: Adt25;
let _5: f64;
let _6: u128;
let _7: &'static u16;
let _8: u64;
let _9: (&'static u8,);
let _10: [i8; 2];
let _11: (&'static *const f64,);
let _12: *const f64;
let _13: char;
let _14: i16;
let _15: i64;
let _16: *const u16;
let _17: [i8; 3];
let _18: f32;
let _19: *mut bool;
let _20: bool;
let _21: [i64; 8];
let _22: i64;
let _23: i32;
let _24: Adt31;
let _25: i16;
let _26: isize;
let _27: ();
let _28: ();
{
RET = !9223372036854775807_isize;
RET = 102_i8 as isize;
RET = (-9223372036854775808_isize) << 50_u8;
RET = !(-9223372036854775808_isize);
RET = 60_isize;
RET = -9223372036854775807_isize;
RET = 5439957818281673785_i64 as isize;
RET = (-88898291079256803036916918750073746045_i128) as isize;
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
RET = 105_isize + (-9223372036854775808_isize);
RET = 11_isize;
RET = 9223372036854775807_isize;
_1 = [37493_u16,36833_u16,31929_u16,49720_u16];
_1 = [64798_u16,4292_u16,54990_u16,33035_u16];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9223372036854775807 => bb8,
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
RET = 9223372036854775807_isize << (-9223372036854775808_isize);
_2 = '\u{e989f}';
_1 = [55895_u16,55082_u16,12875_u16,34182_u16];
_2 = '\u{f065b}';
RET = (-9223372036854775808_isize);
RET = (-9223372036854775808_isize) + (-23_isize);
RET = !(-111_isize);
_1 = [51101_u16,47728_u16,4282_u16,52045_u16];
_2 = '\u{10550b}';
_3 = -RET;
_6 = 324317238246828541007571844153027579792_u128 & 55432465032165547754472177806068985788_u128;
_5 = _6 as f64;
_3 = (-1788497190_i32) as isize;
RET = !_3;
_1 = [64822_u16,18307_u16,64455_u16,30772_u16];
RET = 0_usize as isize;
_5 = 6212699707759582861_u64 as f64;
_3 = 8359_i16 as isize;
_3 = 23_i8 as isize;
_5 = 10_i8 as f64;
Goto(bb9)
}
bb9 = {
_1 = [373_u16,55163_u16,63198_u16,4722_u16];
_6 = !80714202866534250977008735814328577747_u128;
_6 = 106_u8 as u128;
_1 = [29310_u16,18662_u16,40954_u16,55445_u16];
_3 = RET & RET;
RET = !_3;
_6 = 172816840938724063599766562216783791333_u128 - 180991642998705582347792910961576993115_u128;
_5 = (-137991922_i32) as f64;
_10 = [(-99_i8),(-121_i8)];
RET = 2773291727_u32 as isize;
Goto(bb10)
}
bb10 = {
_10 = [(-24_i8),(-56_i8)];
_5 = 629135181220692817_i64 as f64;
_8 = 4874397476540754461_u64 - 3429161809664060788_u64;
_12 = core::ptr::addr_of!(_5);
_11.0 = &_12;
_8 = 217_u8 as u64;
_11.0 = &_12;
RET = 160097913660038086957756530334691744058_i128 as isize;
_13 = _2;
(*_12) = (-10527_i16) as f64;
_14 = 14596_i16 + 17030_i16;
_1 = [45791_u16,39282_u16,52282_u16,47293_u16];
(*_12) = _8 as f64;
_1 = [51453_u16,11171_u16,54833_u16,60815_u16];
_5 = 17517030838087766873_usize as f64;
_15 = (-2332401334627786036_i64) ^ (-7198431119071917647_i64);
_2 = _13;
_10 = [(-60_i8),56_i8];
Goto(bb11)
}
bb11 = {
_12 = core::ptr::addr_of!(_5);
RET = 1177032428_i32 as isize;
_1 = [63519_u16,249_u16,39596_u16,30900_u16];
_8 = true as u64;
(*_12) = 9449_u16 as f64;
_1 = [32256_u16,57461_u16,24553_u16,34150_u16];
RET = _3 + _3;
RET = !_3;
_11.0 = &_12;
_5 = (-135747412294332574414097349049330070889_i128) as f64;
_13 = _2;
RET = -_3;
_14 = 4617076107954162294_usize as i16;
_5 = 8_u8 as f64;
_10 = [(-88_i8),1_i8];
_2 = _13;
_3 = RET ^ RET;
_5 = 65092_u16 as f64;
_10 = [110_i8,(-95_i8)];
_15 = 1220123324165158543_i64 + (-1447269075813625813_i64);
_2 = _13;
_19 = core::ptr::addr_of_mut!(_20);
_23 = 2031254884_i32 + (-272859025_i32);
_15 = 8069712669239236064_i64 >> _6;
Call(_20 = fn9(Move(_11.0)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_12 = core::ptr::addr_of!((*_12));
RET = _2 as isize;
_5 = 169479584416753877039017669628293433109_i128 as f64;
(*_19) = true;
_22 = _13 as i64;
_10 = [(-55_i8),(-76_i8)];
_21 = [_15,_15,_15,_15,_15,_22,_15,_15];
RET = !_3;
_17 = [71_i8,38_i8,(-67_i8)];
_1 = [47004_u16,28320_u16,48802_u16,39150_u16];
_11.0 = &_12;
_13 = _2;
_19 = core::ptr::addr_of_mut!(_20);
_21 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17 = [(-28_i8),74_i8,(-78_i8)];
(*_12) = 18_i8 as f64;
RET = _2 as isize;
_22 = 2339276103_u32 as i64;
RET = (-125125552009291502480966253443367181977_i128) as isize;
_11.0 = &_12;
(*_19) = !false;
_17 = [26_i8,54_i8,65_i8];
Goto(bb13)
}
bb13 = {
_1 = [60878_u16,55062_u16,9054_u16,4164_u16];
(*_19) = false;
_11.0 = &_12;
_13 = _2;
_3 = RET + RET;
Call(_25 = core::intrinsics::transmute(_10), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18 = 4092605616_u32 as f32;
_18 = _23 as f32;
_26 = _3 * _3;
_2 = _13;
_13 = _2;
_6 = 117101289624274222045261929959316014028_u128;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(8_usize, 3_usize, Move(_3), 10_usize, Move(_10), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(8_usize, 25_usize, Move(_25), 1_usize, Move(_1), 20_usize, Move(_20), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: &'static *const f64) -> bool {
mir! {
type RET = bool;
let _2: &'static &'static (Adt31,);
let _3: isize;
let _4: ([u128; 7],);
let _5: isize;
let _6: char;
let _7: isize;
let _8: ([u32; 4],);
let _9: char;
let _10: [u64; 5];
let _11: isize;
let _12: &'static isize;
let _13: isize;
let _14: &'static [i64; 8];
let _15: [u32; 2];
let _16: [u64; 4];
let _17: isize;
let _18: *mut &'static u8;
let _19: (&'static *const f64,);
let _20: Adt35;
let _21: *mut [isize; 5];
let _22: bool;
let _23: (u8,);
let _24: *mut [i32; 7];
let _25: Adt38;
let _26: &'static &'static u16;
let _27: ();
let _28: ();
{
RET = !false;
RET = false;
RET = !false;
RET = false;
RET = 243_u8 < 201_u8;
_3 = 9223372036854775807_isize;
RET = true;
_3 = 234757487461544817175843867478957717057_u128 as isize;
RET = _3 == _3;
RET = true;
RET = !true;
RET = !false;
_4.0 = [12548315574647695585949927563801332653_u128,84139259227966970204908533460014640289_u128,222397040541753741473557089678403617677_u128,240832424653809564367260427336803097157_u128,277057012034233317752387252908150253615_u128,221796633010624598504325567437536893914_u128,115677106747255331511654376536451425631_u128];
_4.0 = [92221972021042144427140274821296378615_u128,187216637285534434454028110033869076093_u128,279650649726944057931078680684595368668_u128,240053882674963659946216913553313734475_u128,277518140567186932616568407697289199301_u128,302434442282774064945075302449245435353_u128,65147709620875063378747065074776237784_u128];
_4.0 = [41600557226685825561966750497136644138_u128,62736819946363184821869123908310711240_u128,246515108825270798861118745903447504614_u128,13547359633994358951273896374858057650_u128,23985326803296671351183041949451475930_u128,219750800303350365858407609500505535478_u128,126871130816743643866876470707117044373_u128];
_4.0 = [296500037415954177589533305852647707340_u128,150797791591720899720804720203632075403_u128,250776381172968083105535951132319208213_u128,178838792386180728676359921903956058251_u128,235935444583527832892628635702649371513_u128,52770896203422521480769629636269673384_u128,119367907194999747404751786541111927043_u128];
RET = false | false;
_4.0 = [240091202236379589238263113769302129441_u128,124995634341727847872162120359230508048_u128,332385200159966125075162235723907391926_u128,136735540503675631422960897971163185499_u128,55375580302623212736593104953021049484_u128,35852755903961716830676841421376895969_u128,50057655486737136994492137178630559055_u128];
_5 = _3;
_5 = _3;
_5 = 556270247735568929_i64 as isize;
_5 = -_3;
_3 = _5 >> _5;
Goto(bb1)
}
bb1 = {
RET = _5 != _5;
RET = !false;
_5 = -_3;
_4.0 = [26453230838452527110359505460522945698_u128,264411687190348601532073970438361509914_u128,30500720229224227814689578404699609208_u128,283267139543441718450734823320476701052_u128,53619356564646856596586847447176158112_u128,274307871986197928621585570568988036581_u128,271439415860843782763251822662250777529_u128];
_7 = _3 - _5;
_3 = 84128630543184153854341304724574323900_i128 as isize;
_5 = (-893278994_i32) as isize;
_6 = '\u{10a252}';
_5 = 243885810112007973436425516507898232786_u128 as isize;
_8.0 = [2361728675_u32,3819484939_u32,1509758902_u32,2773282297_u32];
RET = false;
_6 = '\u{4536}';
RET = _3 <= _7;
_3 = 60147_u16 as isize;
_4.0 = [237663470231600354654940177188499666956_u128,335756152074960430827457901205939031636_u128,162334422241759222518481102493021138238_u128,265895482813958386073778601816620686483_u128,148789051455544956564363344079139469310_u128,235446564604755714506968779193502333147_u128,223230003558570426553524745702590344804_u128];
RET = !false;
_7 = _5;
_9 = _6;
_5 = !_7;
_5 = -_7;
_3 = _5;
RET = _9 != _6;
RET = !true;
RET = !false;
Call(_4.0 = fn10(_8, _8, _8.0, _8, _8.0, _6, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _6;
_3 = -_5;
_7 = _3;
_7 = (-747236648_i32) as isize;
_7 = -_5;
_5 = _3;
Goto(bb3)
}
bb3 = {
_9 = _6;
_6 = _9;
_3 = (-137015356981827645267881764743631993821_i128) as isize;
_6 = _9;
_3 = !_7;
_10 = [11102567009934505181_u64,4091403195609971944_u64,17672748738177402020_u64,11056045767941677462_u64,16373956757940686819_u64];
RET = true;
_6 = _9;
_9 = _6;
_6 = _9;
_11 = _5;
RET = !true;
_8.0 = [3067073054_u32,147543440_u32,2418727560_u32,4193462609_u32];
_11 = 7_usize as isize;
_7 = _11;
_3 = RET as isize;
_5 = 1231646940_u32 as isize;
_11 = _7 | _5;
_13 = 6971072156870406888_u64 as isize;
_12 = &_11;
_13 = 17_i8 as isize;
_12 = &_7;
_12 = &_3;
_8.0 = [3853652594_u32,2252050700_u32,3461666030_u32,645115737_u32];
_3 = _13 + _11;
RET = !false;
Goto(bb4)
}
bb4 = {
_6 = _9;
RET = false ^ false;
_8.0 = [1287698337_u32,154198557_u32,1431331401_u32,1856825853_u32];
_5 = 330846727074708774801535766255529873477_u128 as isize;
_13 = _5;
_12 = &_3;
_7 = RET as isize;
RET = !false;
_13 = _11 * _7;
_17 = _13;
_8.0 = [1394942759_u32,1721174354_u32,2299918427_u32,441944938_u32];
Goto(bb5)
}
bb5 = {
_4.0 = [263237272370980033492999032939065517189_u128,148258010652165011497438958385006521517_u128,268402756314005231070779168297386073775_u128,118729776442451454229897870710621893432_u128,142059750673370174809799067847667176549_u128,95785512193462302955463218859432561536_u128,140826909985595650044797007398271550506_u128];
Goto(bb6)
}
bb6 = {
RET = !false;
_5 = 22107_u16 as isize;
_16 = [13673095061150872976_u64,12153851639815033197_u64,11416400878261657264_u64,17160779393120644775_u64];
_8.0 = [174260219_u32,2567913966_u32,3959172740_u32,1812314199_u32];
RET = true;
_10 = [10389473239484989045_u64,10963956084537859576_u64,12200484475437167313_u64,8035870915112668784_u64,3966168620858521259_u64];
_12 = &_7;
_12 = &_3;
Call(RET = fn14(_17, _4.0, _4.0, _16, _13, _17, _8.0, _8.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = [875184074_u32,112979295_u32];
RET = !false;
_17 = _11 << _7;
_11 = _5;
_7 = _17 ^ _13;
_10 = [11814749904394741251_u64,6648433383881299419_u64,8778566305623137357_u64,10937375429057987993_u64,2030286211127043292_u64];
RET = _7 >= _3;
_11 = _3 & (*_12);
_16 = [9111850187029876841_u64,9633188743247082510_u64,4488518884653399044_u64,5899638014255548152_u64];
_5 = !(*_12);
_4.0 = [199016012298425869910255297491368418557_u128,82528393016396152048068593021651695405_u128,339640409543103706966258772578798965655_u128,278561460231225871381429313344358982216_u128,244844602891122030396817805161766943846_u128,301620484450839496921573756833395912132_u128,233439043640518209506601802863054678919_u128];
RET = false;
_8.0 = [1585708696_u32,3314330596_u32,3729789814_u32,247972144_u32];
Goto(bb8)
}
bb8 = {
_20 = Adt35::Variant1 { fld0: 154721622720436622475604120361574528250_i128 };
_8.0 = [40095417_u32,521304750_u32,3386701936_u32,2641534744_u32];
Goto(bb9)
}
bb9 = {
_22 = _3 > _7;
_15 = [3595443586_u32,1104527092_u32];
_8.0 = [873730124_u32,4080572620_u32,2797246613_u32,2437138805_u32];
_9 = _6;
_4.0 = [110664574593957334606984645731048844133_u128,207873744062313599890027920827624146330_u128,45625455956787990391244910735204362492_u128,229765094129616613183934385529241920919_u128,17880975575529702133737591235242138667_u128,295054655896391998096134498024312623383_u128,50291192232190142379972142127133216522_u128];
place!(Field::<i128>(Variant(_20, 1), 0)) = (-1144730344_i32) as i128;
Goto(bb10)
}
bb10 = {
_20 = Adt35::Variant1 { fld0: 140176477496339071245127992478017048003_i128 };
_23.0 = !215_u8;
RET = _5 < _7;
_10 = [12040729241038233419_u64,5851548113984701182_u64,8363905109968381064_u64,3765952879664639630_u64,2778436651532712086_u64];
_12 = &_7;
_17 = _7;
_22 = RET;
_16 = [2364255930429636303_u64,1175898842625621112_u64,14940833532986945351_u64,7063755278256922480_u64];
place!(Field::<i128>(Variant(_20, 1), 0)) = (*_12) as i128;
_10 = [8422971757009253925_u64,1803540210842276321_u64,2974597039250700801_u64,1499713059452262560_u64,13189479985103782889_u64];
_25.fld1 = _10;
_15 = [479301162_u32,2800809821_u32];
_22 = RET;
RET = !_22;
place!(Field::<i128>(Variant(_20, 1), 0)) = (-139303703377323050634111311495430653104_i128) & 147941575983016029028837164460934209077_i128;
_23.0 = _17 as u8;
Goto(bb11)
}
bb11 = {
Call(_27 = dump_var(9_usize, 22_usize, Move(_22), 10_usize, Move(_10), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_27 = dump_var(9_usize, 7_usize, Move(_7), 3_usize, Move(_3), 5_usize, Move(_5), 28_usize, _28), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: ([u32; 4],),mut _2: ([u32; 4],),mut _3: [u32; 4],mut _4: ([u32; 4],),mut _5: [u32; 4],mut _6: char,mut _7: ([u32; 4],)) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _8: i64;
let _9: i16;
let _10: ([u128; 7],);
let _11: &'static &'static (Adt31,);
let _12: bool;
let _13: &'static [i64; 8];
let _14: &'static Adt35;
let _15: bool;
let _16: char;
let _17: *mut Adt25;
let _18: Adt56;
let _19: (*mut usize,);
let _20: &'static *const f64;
let _21: [i64; 8];
let _22: *mut i32;
let _23: u32;
let _24: u32;
let _25: isize;
let _26: u16;
let _27: ([u128; 7],);
let _28: [i8; 2];
let _29: isize;
let _30: isize;
let _31: (Adt31,);
let _32: [isize; 5];
let _33: &'static u16;
let _34: usize;
let _35: [i8; 3];
let _36: [i128; 3];
let _37: [i64; 1];
let _38: [u32; 3];
let _39: (&'static *const f64,);
let _40: char;
let _41: f64;
let _42: Adt31;
let _43: (([u32; 4],), bool, (&'static *const f64,));
let _44: char;
let _45: *const (&'static *const f64,);
let _46: [u64; 6];
let _47: ((Adt38, *mut [u32; 4]), *const f64, *const *const Adt25, *const Adt25);
let _48: [u32; 3];
let _49: *mut [isize; 5];
let _50: char;
let _51: isize;
let _52: u128;
let _53: i64;
let _54: ((Adt38, *mut [u32; 4]), *const f64, *const *const Adt25, *const Adt25);
let _55: [u32; 2];
let _56: ();
let _57: ();
{
RET = [136388894972632126087353306153884741846_u128,188393660236899600984973021612882153200_u128,138746026289022499919446834732914913619_u128,248713054871452928171567149033542760497_u128,339527864653071817651629261222960513863_u128,62212069826287022500619115831930033774_u128,5878146358021404394606862457563666912_u128];
_7 = (_2.0,);
_8 = 32_i8 as i64;
_9 = (-427_i16);
_6 = '\u{df0a1}';
_5 = [4206971116_u32,2214965156_u32,3302331425_u32,372123097_u32];
Call(_4 = fn11(_3, _7.0, _5, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = '\u{1699f}';
_1.0 = [1766837036_u32,3925547868_u32,105672_u32,1239524309_u32];
_1 = (_4.0,);
_6 = '\u{83195}';
_9 = (-2924_i16);
_4.0 = [3588301976_u32,2695483258_u32,3548688966_u32,233860557_u32];
RET = [228143326341292642403657686322848331169_u128,294354817720855144218904169899774008772_u128,65131609016575401072942266966315120125_u128,234639436678953039968204652465081855858_u128,59007166046141753778268285114754539525_u128,287960506207420271305670235655141205308_u128,17505969227308701179928826591474314046_u128];
_9 = -(-2859_i16);
_5 = [3147272512_u32,3058539026_u32,3518302920_u32,3855281276_u32];
_3 = [580497957_u32,623352542_u32,2580947368_u32,3589915036_u32];
_10 = (RET,);
_1 = _4;
_1 = (_4.0,);
_1 = (_3,);
_7 = _4;
_1 = (_2.0,);
_7.0 = [695090435_u32,1964927300_u32,3695098560_u32,3915155742_u32];
_10.0 = [332555044054716351337314600865276859498_u128,223250402741190921855487511937588272568_u128,58094526936695840669064623181645070862_u128,115802768706731852630725332514183594052_u128,107211939189856376556814818954265061912_u128,9337479474443200800454066031602420912_u128,15921785828610877846822342983460318484_u128];
_1 = (_7.0,);
_10.0 = [261978557571776193018851005471395584083_u128,101609007647533840022501646204293720899_u128,234063631362873173265958580375256475676_u128,54836197822550421244094666895278436785_u128,72677040098347777105500514205473254790_u128,300131019319086906048475391418034637095_u128,110102878303429042316175736015092754294_u128];
_1 = (_7.0,);
Goto(bb2)
}
bb2 = {
_12 = false;
_7 = _4;
_3 = _7.0;
_2.0 = [4222938097_u32,2707526825_u32,843820489_u32,822962125_u32];
_15 = !_12;
RET = [75732628983312232806098150356516515048_u128,184843825158119083179153643077836395280_u128,336368382989946006304313577417656659100_u128,215115897468146822769890356731321275484_u128,90621178160121863464353518945029470195_u128,287816864789718905097650893025569019279_u128,38516446419248109296427392476320541993_u128];
_10.0 = [139709142418993215943560673024143128984_u128,189437773637878771700184424679522305018_u128,50058454339542449410357836506250181940_u128,184375995011684532885500965729564460216_u128,210830312946361306368776175350101632107_u128,41555902170754222446506244981495189797_u128,219041758139595856218130627597839598401_u128];
_15 = !_12;
_5 = _4.0;
_8 = 8901521590204935212_i64 ^ (-2890203802166601639_i64);
_1.0 = _7.0;
Goto(bb3)
}
bb3 = {
_1.0 = [3286554390_u32,2315571146_u32,1053985861_u32,1080421971_u32];
_16 = _6;
_2 = (_7.0,);
_10 = (RET,);
_3 = _7.0;
_12 = _15;
_18.fld2 = (_7.0,);
_18 = Adt56 { fld0: _3,fld1: RET,fld2: _1 };
_15 = _9 > _9;
_8 = 8184024318536170495_i64 ^ 8717606415879460325_i64;
_10.0 = [337953581970374022343548725277576075198_u128,95380065856720699188621309635407972900_u128,209996738836222302623516639165627944843_u128,65851143969688992708602427746527036871_u128,173717867443725499693293607704789136938_u128,53097511648269723107227005584951071385_u128,238027424082695985754827676411815332353_u128];
_4 = _18.fld2;
RET = _18.fld1;
_7 = (_2.0,);
RET = [142277748136801843395719151456856801728_u128,1193574177414702011812853478553627412_u128,2677790474523778249597724670316516371_u128,187273739303138059385330776828412401692_u128,31235053042820547671538887540405370718_u128,285689198152523379340850240370477718439_u128,137186284529446192559433439566064766191_u128];
_12 = !_15;
_1.0 = [4219708882_u32,1479135514_u32,899495290_u32,874090568_u32];
_10.0 = [303624286026456141567876965995214955506_u128,38180058760474321672544347232428470457_u128,83361434665910349834673768056960858778_u128,131825135102762032211362300895433165601_u128,273532291992335344074502622847444077530_u128,225521569277410884508426731536117453934_u128,24276390491599380367361550814659365736_u128];
_18.fld1 = _10.0;
_18.fld2.0 = [2526470807_u32,4061191124_u32,3306868133_u32,1504959816_u32];
_8 = (-1887376605716961252_i64);
_7.0 = [1876561000_u32,730766215_u32,1113900914_u32,2765351946_u32];
_8 = -8569533019324896204_i64;
_1.0 = [3490941987_u32,3594618351_u32,1568652085_u32,170308311_u32];
_1 = (_5,);
Call(RET = fn13(_1.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = &_21;
_1.0 = [3050633360_u32,1883265738_u32,1796065695_u32,4258483136_u32];
_1.0 = [211431671_u32,2092705686_u32,3623076673_u32,2795175618_u32];
_7 = (_1.0,);
_13 = &(*_13);
_18.fld0 = _5;
_1.0 = [2261371816_u32,365424043_u32,2684906549_u32,2421464721_u32];
_13 = &_21;
_8 = !(-5835105356196276477_i64);
_2 = (_1.0,);
_7.0 = [2378223947_u32,4003612091_u32,373948451_u32,1841416972_u32];
_10 = (RET,);
_10.0 = [198634810679585665633497618639942198459_u128,164243925187954874979627628800926243256_u128,314806435286469735372766545905907401335_u128,145613749218458156064462491555357581847_u128,106924583619170272596668269021974572565_u128,62071728466431354977998973964484800719_u128,204319110131448617760128646206088787384_u128];
_5 = _4.0;
_13 = &(*_13);
_1.0 = [1738788946_u32,233684813_u32,1387117702_u32,2030384232_u32];
_18.fld0 = [1473306690_u32,3943041465_u32,2504661285_u32,6466184_u32];
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_1.0 = [3592723787_u32,1696047962_u32,3931738679_u32,308348132_u32];
_1 = (_18.fld2.0,);
_18.fld1 = [29463378633529349468054074144307514595_u128,96646389460238687958399022155204791436_u128,183664724013920379613737724054257227108_u128,210927198193609645333070652668785382051_u128,211620382758898067802789401376397291491_u128,81038657556419661699166629198162651786_u128,200720574976281491267873081490071058368_u128];
Goto(bb5)
}
bb5 = {
_15 = _12;
_24 = 9223372036854775807_isize as u32;
_26 = _6 as u16;
_10.0 = [179529877688817688305642738590397177782_u128,181704270803326033397503910392682640571_u128,91450020124863378021881310222317450572_u128,330333737806483036614550805521628458460_u128,47748085231291454321071510359632997080_u128,69016263799625019105824988091190308638_u128,99076346350987189725042837391776173192_u128];
Goto(bb6)
}
bb6 = {
_26 = !33394_u16;
_18.fld0 = _2.0;
_2.0 = [_24,_24,_24,_24];
_27 = (RET,);
_25 = !12_isize;
_2 = (_3,);
_12 = !_15;
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_27 = (_10.0,);
_2 = (_5,);
_10 = _27;
_10 = (_18.fld1,);
_18.fld0 = [_24,_24,_24,_24];
_1.0 = [_24,_24,_24,_24];
_18 = Adt56 { fld0: _2.0,fld1: _10.0,fld2: _4 };
_2 = (_4.0,);
_4.0 = [_24,_24,_24,_24];
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_23 = _24;
_23 = 65084716765678013923990055646715373777_i128 as u32;
RET = _10.0;
Goto(bb7)
}
bb7 = {
_6 = _16;
Call(_2.0 = core::intrinsics::transmute(_7.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = [19425369210996499711288849926145641005_u128,54618044203282958918033946559263819165_u128,158671252796360460802507922930548417944_u128,242167239581258896004979308923226322374_u128,138902675815246953297148993052113668631_u128,99993075400817330021503151138479076020_u128,51432797822593177247583467909964220036_u128];
_16 = _6;
_10.0 = RET;
_4 = _2;
RET = _27.0;
_10 = (_18.fld1,);
_6 = _16;
_29 = -_25;
RET = [193944422592482102296079780839534676612_u128,147094689574777270361139134102259639771_u128,306483102848429086207325695476486011722_u128,102631436413515212774015816065112654916_u128,180234526629806241982010514756318549821_u128,233877067025699889973404184406202723628_u128,89065776984979974509030853629181999509_u128];
_18.fld2.0 = [_24,_24,_23,_24];
_10 = _27;
_18 = Adt56 { fld0: _3,fld1: RET,fld2: _4 };
_18 = Adt56 { fld0: _5,fld1: _10.0,fld2: _4 };
_18.fld2 = _4;
_27.0 = [167604735736585491035212361992138291334_u128,131581227820505311681232290525231864626_u128,177205621770531788633522989117235696286_u128,12100857898728143165741390099539213046_u128,109633562369872028009033498339078985300_u128,285206862811808315872660879263949827379_u128,240423628840406686013565117767567855459_u128];
_7.0 = [_23,_23,_24,_23];
_28 = [3_i8,7_i8];
_16 = _6;
_10 = (_18.fld1,);
_35 = [56_i8,3_i8,(-33_i8)];
_23 = !_24;
Goto(bb9)
}
bb9 = {
_18.fld1 = _27.0;
_15 = _12 ^ _12;
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_19.0 = core::ptr::addr_of_mut!(_34);
_30 = -_29;
_18.fld2.0 = [_24,_24,_23,_24];
_26 = 39186_u16 - 3330_u16;
_13 = &_21;
_2.0 = [_23,_23,_23,_23];
_19.0 = core::ptr::addr_of_mut!(_34);
RET = [265445531544525275002201626210831003310_u128,36157680260179138381878909584113376747_u128,287344330886188141819805593582640129282_u128,277707222222665537755648701567655106564_u128,43526484986839765327944629342733407523_u128,339913103119085410468772984617618864125_u128,7169409115788263350347762859984818807_u128];
_26 = _6 as u16;
_3 = [_23,_24,_24,_24];
_26 = !63880_u16;
_27 = (_10.0,);
_40 = _6;
_18.fld1 = [293539350272375440773891296348031215187_u128,97427697470687623489268896297305299271_u128,271831648080101479605656859034446713894_u128,101506686555849198155248213974877048410_u128,11665123459739792770831763308838113668_u128,37575493824695598616881166447155632196_u128,24166831363296763840022699303677812374_u128];
_27 = (_10.0,);
_33 = &_26;
_34 = !3_usize;
_13 = &(*_13);
Goto(bb10)
}
bb10 = {
_26 = _29 as u16;
_18 = Adt56 { fld0: _1.0,fld1: _10.0,fld2: _4 };
_12 = !_15;
_40 = _6;
_34 = !6_usize;
_4.0 = _1.0;
_7 = (_18.fld2.0,);
_41 = 123369430995639282167286634259468472822_u128 as f64;
_24 = _29 as u32;
_33 = &_26;
_34 = 6_usize | 14864789041163650312_usize;
_8 = -5107937979678968706_i64;
_26 = !48388_u16;
_7 = _1;
_1 = (_18.fld2.0,);
_29 = _9 as isize;
_5 = [_24,_24,_23,_24];
_42 = Adt31::Variant3 { fld0: _41,fld1: Move(_19.0),fld2: _24,fld3: (-2_i8),fld4: _9,fld5: 1716050011_i32,fld6: 17560229855487529576_u64,fld7: (-75570364604731540439798890391738200324_i128) };
_32 = [_25,_25,_29,_30,_30];
_18 = Adt56 { fld0: _5,fld1: RET,fld2: _1 };
_24 = _15 as u32;
_36 = [(-53916530718950009622054655800970797998_i128),(-135584902523958029958845493563461194192_i128),154840737456898856845796447969542208279_i128];
place!(Field::<i32>(Variant(_42, 3), 5)) = (-1552014382_i32);
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_18.fld1 = [259714432122585860824344650540043749375_u128,324366257674542625454237265378004833402_u128,115706268223469665833218659003202922058_u128,247886697820446403197154428297152862195_u128,44130409996565599566346904139024712417_u128,244494583807381826455568494637895400463_u128,195211165947214864440361396509335008163_u128];
_16 = _40;
Goto(bb11)
}
bb11 = {
_10.0 = [29938725566164762403659071633926897085_u128,54441760043219009596719095358579532537_u128,174712715154171862661494058715965490385_u128,6015137330305171581176501349721007880_u128,57027657715585838865782594530633637253_u128,318379057829117692113648439242380529809_u128,306561035714978724360834235122351840021_u128];
_18 = Adt56 { fld0: _1.0,fld1: _27.0,fld2: _4 };
_7 = (_1.0,);
_18.fld2 = (_18.fld0,);
place!(Field::<i128>(Variant(_42, 3), 7)) = !(-85655799596085468099240202246003927870_i128);
_12 = !_15;
_30 = _34 as isize;
_13 = &_21;
_18.fld2.0 = [_24,_24,_23,_24];
_15 = _12;
place!(Field::<*mut usize>(Variant(_42, 3), 1)) = core::ptr::addr_of_mut!(_34);
_37 = [_8];
_43.2.0 = &_47.1;
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_32 = [_30,_30,_29,_25,_25];
_20 = &_47.1;
match Field::<i32>(Variant(_42, 3), 5) {
0 => bb12,
1 => bb13,
340282366920938463463374607430216197074 => bb15,
_ => bb14
}
}
bb12 = {
_12 = false;
_7 = _4;
_3 = _7.0;
_2.0 = [4222938097_u32,2707526825_u32,843820489_u32,822962125_u32];
_15 = !_12;
RET = [75732628983312232806098150356516515048_u128,184843825158119083179153643077836395280_u128,336368382989946006304313577417656659100_u128,215115897468146822769890356731321275484_u128,90621178160121863464353518945029470195_u128,287816864789718905097650893025569019279_u128,38516446419248109296427392476320541993_u128];
_10.0 = [139709142418993215943560673024143128984_u128,189437773637878771700184424679522305018_u128,50058454339542449410357836506250181940_u128,184375995011684532885500965729564460216_u128,210830312946361306368776175350101632107_u128,41555902170754222446506244981495189797_u128,219041758139595856218130627597839598401_u128];
_15 = !_12;
_5 = _4.0;
_8 = 8901521590204935212_i64 ^ (-2890203802166601639_i64);
_1.0 = _7.0;
Goto(bb3)
}
bb13 = {
_18.fld1 = _27.0;
_15 = _12 ^ _12;
_21 = [_8,_8,_8,_8,_8,_8,_8,_8];
_19.0 = core::ptr::addr_of_mut!(_34);
_30 = -_29;
_18.fld2.0 = [_24,_24,_23,_24];
_26 = 39186_u16 - 3330_u16;
_13 = &_21;
_2.0 = [_23,_23,_23,_23];
_19.0 = core::ptr::addr_of_mut!(_34);
RET = [265445531544525275002201626210831003310_u128,36157680260179138381878909584113376747_u128,287344330886188141819805593582640129282_u128,277707222222665537755648701567655106564_u128,43526484986839765327944629342733407523_u128,339913103119085410468772984617618864125_u128,7169409115788263350347762859984818807_u128];
_26 = _6 as u16;
_3 = [_23,_24,_24,_24];
_26 = !63880_u16;
_27 = (_10.0,);
_40 = _6;
_18.fld1 = [293539350272375440773891296348031215187_u128,97427697470687623489268896297305299271_u128,271831648080101479605656859034446713894_u128,101506686555849198155248213974877048410_u128,11665123459739792770831763308838113668_u128,37575493824695598616881166447155632196_u128,24166831363296763840022699303677812374_u128];
_27 = (_10.0,);
_33 = &_26;
_34 = !3_usize;
_13 = &(*_13);
Goto(bb10)
}
bb14 = {
_6 = '\u{1699f}';
_1.0 = [1766837036_u32,3925547868_u32,105672_u32,1239524309_u32];
_1 = (_4.0,);
_6 = '\u{83195}';
_9 = (-2924_i16);
_4.0 = [3588301976_u32,2695483258_u32,3548688966_u32,233860557_u32];
RET = [228143326341292642403657686322848331169_u128,294354817720855144218904169899774008772_u128,65131609016575401072942266966315120125_u128,234639436678953039968204652465081855858_u128,59007166046141753778268285114754539525_u128,287960506207420271305670235655141205308_u128,17505969227308701179928826591474314046_u128];
_9 = -(-2859_i16);
_5 = [3147272512_u32,3058539026_u32,3518302920_u32,3855281276_u32];
_3 = [580497957_u32,623352542_u32,2580947368_u32,3589915036_u32];
_10 = (RET,);
_1 = _4;
_1 = (_4.0,);
_1 = (_3,);
_7 = _4;
_1 = (_2.0,);
_7.0 = [695090435_u32,1964927300_u32,3695098560_u32,3915155742_u32];
_10.0 = [332555044054716351337314600865276859498_u128,223250402741190921855487511937588272568_u128,58094526936695840669064623181645070862_u128,115802768706731852630725332514183594052_u128,107211939189856376556814818954265061912_u128,9337479474443200800454066031602420912_u128,15921785828610877846822342983460318484_u128];
_1 = (_7.0,);
_10.0 = [261978557571776193018851005471395584083_u128,101609007647533840022501646204293720899_u128,234063631362873173265958580375256475676_u128,54836197822550421244094666895278436785_u128,72677040098347777105500514205473254790_u128,300131019319086906048475391418034637095_u128,110102878303429042316175736015092754294_u128];
_1 = (_7.0,);
Goto(bb2)
}
bb15 = {
_24 = !_23;
_25 = _26 as isize;
_36 = [Field::<i128>(Variant(_42, 3), 7),Field::<i128>(Variant(_42, 3), 7),Field::<i128>(Variant(_42, 3), 7)];
_41 = Field::<i32>(Variant(_42, 3), 5) as f64;
_9 = _15 as i16;
_31.0 = Adt31::Variant2 { fld0: _12,fld1: _34,fld2: _25 };
_45 = core::ptr::addr_of!(_39);
_20 = &(*_20);
_47.2 = core::ptr::addr_of!(_54.3);
_48 = [_24,Field::<u32>(Variant(_42, 3), 2),Field::<u32>(Variant(_42, 3), 2)];
place!(Field::<i8>(Variant(_42, 3), 3)) = (-125_i8);
_42 = Move(_31.0);
_47.0.0.fld0 = _35;
_18.fld2.0 = [_24,_23,_24,_24];
_13 = &_21;
_41 = 70_u8 as f64;
_54.0.1 = core::ptr::addr_of_mut!(_3);
_49 = core::ptr::addr_of_mut!(_32);
_31.0 = Adt31::Variant2 { fld0: Field::<bool>(Variant(_42, 2), 0),fld1: _34,fld2: _30 };
SetDiscriminant(_42, 0);
_34 = !Field::<usize>(Variant(_31.0, 2), 1);
_36 = [(-169670575681601406358999945156974995136_i128),(-61521162605069955613978090929483693246_i128),55146242720163647585978777389028784887_i128];
_43.1 = _12 & _12;
place!(Field::<bool>(Variant(_31.0, 2), 0)) = _43.1 & _43.1;
_42 = Move(_31.0);
_19.0 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_42, 2), 1)));
Goto(bb16)
}
bb16 = {
Call(_56 = dump_var(10_usize, 29_usize, Move(_29), 9_usize, Move(_9), 6_usize, Move(_6), 48_usize, Move(_48)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(10_usize, 10_usize, Move(_10), 2_usize, Move(_2), 24_usize, Move(_24), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(10_usize, 37_usize, Move(_37), 36_usize, Move(_36), 26_usize, Move(_26), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(10_usize, 30_usize, Move(_30), 4_usize, Move(_4), 57_usize, _57, 57_usize, _57), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [u32; 4],mut _2: [u32; 4],mut _3: [u32; 4],mut _4: i16) -> ([u32; 4],) {
mir! {
type RET = ([u32; 4],);
let _5: [u16; 4];
let _6: ([u32; 4],);
let _7: f64;
let _8: f32;
let _9: &'static *const f64;
let _10: isize;
let _11: [isize; 5];
let _12: u32;
let _13: usize;
let _14: f32;
let _15: (*const f64, *mut &'static u8);
let _16: *mut &'static f32;
let _17: [u32; 4];
let _18: u128;
let _19: usize;
let _20: (u32, *mut bool, i128, i16);
let _21: *mut &'static f32;
let _22: bool;
let _23: ([u32; 4],);
let _24: u8;
let _25: (*const f64, *mut &'static u8);
let _26: Adt44;
let _27: i32;
let _28: (*mut usize,);
let _29: u64;
let _30: [u64; 5];
let _31: Adt38;
let _32: ([u128; 7],);
let _33: *mut usize;
let _34: i8;
let _35: ([u128; 7],);
let _36: [i8; 3];
let _37: ();
let _38: ();
{
RET.0 = [415981803_u32,3056059893_u32,3036206754_u32,1034210177_u32];
RET.0 = [3883447764_u32,4172119580_u32,857025839_u32,3681371228_u32];
RET.0 = [3893892514_u32,2932646858_u32,3925602588_u32,4103852509_u32];
_4 = (-28318_i16);
RET = (_3,);
_4 = 9493_i16;
RET = (_1,);
_3 = [492105710_u32,2479706752_u32,340324637_u32,804110368_u32];
_4 = 9157_i16 + 7253_i16;
_7 = 65898698474885810681401874517750478844_i128 as f64;
_6 = (_1,);
_5 = [47087_u16,42465_u16,11609_u16,49785_u16];
_8 = 11597859196998652313_u64 as f32;
_2 = [1558131748_u32,2106646152_u32,3466723157_u32,2466304036_u32];
_1 = [1712086161_u32,164765900_u32,853830473_u32,2674682875_u32];
_6 = (_2,);
_1 = [1309319959_u32,312620599_u32,1445325033_u32,929540631_u32];
_3 = [3675392276_u32,3541071067_u32,914256383_u32,3454783662_u32];
_3 = [2431841679_u32,2324326659_u32,3957155807_u32,3677151139_u32];
_6 = RET;
RET.0 = [1240492040_u32,3954485718_u32,3067737358_u32,1082697762_u32];
Call(RET = fn12(_2, _3, _3, _8, _1, _2, _6, _1, _2, _3, _2, _6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _6;
_4 = (-30319_i16) ^ 1344_i16;
RET.0 = [799843483_u32,1381536563_u32,2920176012_u32,1608481005_u32];
_1 = _2;
RET = (_2,);
_5 = [28143_u16,48604_u16,6688_u16,36404_u16];
_6.0 = [932491208_u32,697323277_u32,2999255489_u32,3044212210_u32];
_7 = 4708899496572618542_u64 as f64;
_6 = RET;
_10 = 8100_u16 as isize;
_1 = _6.0;
_6 = (RET.0,);
_2 = [1839529798_u32,3268980765_u32,3550000154_u32,946552322_u32];
_7 = 18985_u16 as f64;
_12 = !3752824491_u32;
_13 = !4282822724417796102_usize;
Goto(bb2)
}
bb2 = {
RET.0 = [_12,_12,_12,_12];
_12 = 230847375545187940412372227078181266409_u128 as u32;
RET = (_1,);
_14 = _8;
Goto(bb3)
}
bb3 = {
RET = (_6.0,);
RET.0 = [_12,_12,_12,_12];
_3 = _2;
_11 = [_10,_10,_10,_10,_10];
Goto(bb4)
}
bb4 = {
_7 = _8 as f64;
_18 = 293383996541690490826874738495569553489_u128;
_17 = _1;
_20.2 = 28341352948658295896603388658121993891_i128 * (-168512490307840884921406612586300625112_i128);
_20.0 = _12 + _12;
_9 = &_15.0;
_14 = 4015144882297880155_u64 as f32;
_19 = _13;
RET.0 = [_20.0,_20.0,_20.0,_20.0];
_20.3 = !_4;
_8 = -_14;
_3 = _2;
Goto(bb5)
}
bb5 = {
_5 = [26128_u16,64132_u16,1403_u16,7867_u16];
_9 = &(*_9);
RET = _6;
_20.1 = core::ptr::addr_of_mut!(_22);
match _18 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
293383996541690490826874738495569553489 => bb13,
_ => bb12
}
}
bb6 = {
_7 = _8 as f64;
_18 = 293383996541690490826874738495569553489_u128;
_17 = _1;
_20.2 = 28341352948658295896603388658121993891_i128 * (-168512490307840884921406612586300625112_i128);
_20.0 = _12 + _12;
_9 = &_15.0;
_14 = 4015144882297880155_u64 as f32;
_19 = _13;
RET.0 = [_20.0,_20.0,_20.0,_20.0];
_20.3 = !_4;
_8 = -_14;
_3 = _2;
Goto(bb5)
}
bb7 = {
RET = (_6.0,);
RET.0 = [_12,_12,_12,_12];
_3 = _2;
_11 = [_10,_10,_10,_10,_10];
Goto(bb4)
}
bb8 = {
RET.0 = [_12,_12,_12,_12];
_12 = 230847375545187940412372227078181266409_u128 as u32;
RET = (_1,);
_14 = _8;
Goto(bb3)
}
bb9 = {
RET = _6;
_4 = (-30319_i16) ^ 1344_i16;
RET.0 = [799843483_u32,1381536563_u32,2920176012_u32,1608481005_u32];
_1 = _2;
RET = (_2,);
_5 = [28143_u16,48604_u16,6688_u16,36404_u16];
_6.0 = [932491208_u32,697323277_u32,2999255489_u32,3044212210_u32];
_7 = 4708899496572618542_u64 as f64;
_6 = RET;
_10 = 8100_u16 as isize;
_1 = _6.0;
_6 = (RET.0,);
_2 = [1839529798_u32,3268980765_u32,3550000154_u32,946552322_u32];
_7 = 18985_u16 as f64;
_12 = !3752824491_u32;
_13 = !4282822724417796102_usize;
Goto(bb2)
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
_23.0 = RET.0;
_8 = _14;
_22 = true;
RET.0 = _2;
_11 = [_10,_10,_10,_10,_10];
_1 = _2;
_23.0 = _1;
_19 = !_13;
RET.0 = [_20.0,_20.0,_20.0,_20.0];
_10 = 9223372036854775807_isize;
RET = (_23.0,);
_23.0 = [_12,_20.0,_20.0,_20.0];
_18 = 123767765616300675584767353601720328196_u128 >> _19;
_4 = _20.3;
match _10 {
0 => bb1,
1 => bb2,
9223372036854775807 => bb15,
_ => bb14
}
}
bb14 = {
RET = _6;
_4 = (-30319_i16) ^ 1344_i16;
RET.0 = [799843483_u32,1381536563_u32,2920176012_u32,1608481005_u32];
_1 = _2;
RET = (_2,);
_5 = [28143_u16,48604_u16,6688_u16,36404_u16];
_6.0 = [932491208_u32,697323277_u32,2999255489_u32,3044212210_u32];
_7 = 4708899496572618542_u64 as f64;
_6 = RET;
_10 = 8100_u16 as isize;
_1 = _6.0;
_6 = (RET.0,);
_2 = [1839529798_u32,3268980765_u32,3550000154_u32,946552322_u32];
_7 = 18985_u16 as f64;
_12 = !3752824491_u32;
_13 = !4282822724417796102_usize;
Goto(bb2)
}
bb15 = {
_10 = !10_isize;
_20.1 = core::ptr::addr_of_mut!(_22);
_18 = 21503002172872306491223578055650086067_u128 << _10;
_23.0 = [_20.0,_20.0,_20.0,_12];
RET = (_1,);
_3 = [_20.0,_20.0,_12,_20.0];
_9 = &(*_9);
_31.fld1 = [5197853304054009978_u64,10885993819949745046_u64,15953743134369667756_u64,9179946534976902979_u64,16091409708704815916_u64];
_3 = _2;
_9 = &(*_9);
_15.0 = core::ptr::addr_of!(_7);
_27 = (-602887194_i32);
_13 = !_19;
_33 = core::ptr::addr_of_mut!(_19);
_24 = 75_u8 ^ 63_u8;
_32.0 = [_18,_18,_18,_18,_18,_18,_18];
_35 = (_32.0,);
_3 = [_20.0,_12,_12,_20.0];
_23.0 = [_20.0,_12,_12,_20.0];
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(11_usize, 24_usize, Move(_24), 5_usize, Move(_5), 32_usize, Move(_32), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(11_usize, 22_usize, Move(_22), 10_usize, Move(_10), 3_usize, Move(_3), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(11_usize, 35_usize, Move(_35), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [u32; 4],mut _2: [u32; 4],mut _3: [u32; 4],mut _4: f32,mut _5: [u32; 4],mut _6: [u32; 4],mut _7: ([u32; 4],),mut _8: [u32; 4],mut _9: [u32; 4],mut _10: [u32; 4],mut _11: [u32; 4],mut _12: [u32; 4]) -> ([u32; 4],) {
mir! {
type RET = ([u32; 4],);
let _13: &'static (u8,);
let _14: f64;
let _15: f64;
let _16: isize;
let _17: &'static (Adt31,);
let _18: [i64; 1];
let _19: *mut &'static u8;
let _20: f32;
let _21: i128;
let _22: [u32; 7];
let _23: *mut [i32; 7];
let _24: &'static i128;
let _25: Adt57;
let _26: [u32; 4];
let _27: i16;
let _28: *const (&'static *const f64,);
let _29: &'static i128;
let _30: *mut [i32; 7];
let _31: Adt38;
let _32: *mut bool;
let _33: ([u32; 4],);
let _34: *const u16;
let _35: ();
let _36: ();
{
_4 = (-1137310055109780572_i64) as f32;
_4 = 7679485733223441861_i64 as f32;
RET = _7;
RET = _7;
RET.0 = [3878050922_u32,1287899755_u32,164883356_u32,1760685550_u32];
RET.0 = [3619434019_u32,4009533566_u32,4226628195_u32,1686716270_u32];
_8 = _10;
_7 = (_11,);
_7 = (_10,);
_8 = [1052656003_u32,2087756697_u32,2278052889_u32,2516518016_u32];
_4 = 164548181486692870040253195177860071035_u128 as f32;
_6 = [1100831773_u32,596671115_u32,1471716588_u32,3340877251_u32];
_7.0 = [1627980701_u32,70370690_u32,1582911420_u32,251567736_u32];
_9 = [4276503416_u32,1327681448_u32,1989384576_u32,1935890854_u32];
RET.0 = _12;
_8 = [3849010264_u32,422062309_u32,3015523381_u32,4094393357_u32];
RET = (_8,);
_8 = [1588031807_u32,575071597_u32,48705534_u32,2773482839_u32];
_2 = [3119691943_u32,3558661434_u32,3936594323_u32,1026537786_u32];
Goto(bb1)
}
bb1 = {
_11 = _1;
RET.0 = _7.0;
RET.0 = _3;
_7 = RET;
_4 = 9048928572916519657_u64 as f32;
_12 = [2145676651_u32,1379126105_u32,1663042100_u32,1351952690_u32];
_7 = (_8,);
_7 = (_12,);
_5 = _11;
_9 = _3;
Goto(bb2)
}
bb2 = {
RET.0 = [4052690745_u32,1297551992_u32,500389095_u32,1022634736_u32];
_10 = [56303824_u32,2415049005_u32,1189480745_u32,2003568363_u32];
_4 = 2738570053_u32 as f32;
_2 = [3691562185_u32,2721207752_u32,352605504_u32,1590603649_u32];
_16 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_6 = [2887503683_u32,1572143760_u32,1824231828_u32,473585930_u32];
_7.0 = [1542052226_u32,3177048305_u32,54210578_u32,27680409_u32];
_9 = [2288668532_u32,304463671_u32,4197916923_u32,2426586549_u32];
_14 = 15231160318105507601_usize as f64;
_7.0 = _3;
_12 = _11;
RET.0 = _9;
_16 = (-6994850805399469674_i64) as isize;
_6 = _7.0;
_1 = [898218501_u32,2225674662_u32,3569492795_u32,3813812785_u32];
_12 = [3459847720_u32,2936862122_u32,2682715816_u32,3717098045_u32];
RET = (_10,);
Goto(bb3)
}
bb3 = {
_12 = [335222057_u32,3985191160_u32,958940904_u32,3928416514_u32];
_8 = [1400768047_u32,616482470_u32,1780304714_u32,3399807352_u32];
RET = (_12,);
_7 = (_12,);
_5 = [1792793329_u32,3053705183_u32,3002066896_u32,2035117240_u32];
_10 = [3084000156_u32,1789529664_u32,1355271634_u32,1248109635_u32];
_2 = _8;
RET.0 = [800324550_u32,1164658925_u32,2545029382_u32,89652997_u32];
RET.0 = _5;
_10 = [1450395674_u32,2592283373_u32,200125884_u32,3655051234_u32];
_2 = _8;
_2 = [3542128960_u32,4120197684_u32,1808524195_u32,397303570_u32];
_10 = _11;
Goto(bb4)
}
bb4 = {
RET = (_5,);
_21 = 5570719086139032086_i64 as i128;
_7.0 = _8;
_16 = 9223372036854775807_isize;
_12 = [3239477378_u32,563160813_u32,2930539142_u32,3911876629_u32];
_15 = _14;
_10 = _3;
_12 = [2754397732_u32,2632852672_u32,3150754115_u32,2233402148_u32];
_21 = (-165382190019499936528901789528177178631_i128) & 89595863882762582635104525527813895608_i128;
Goto(bb5)
}
bb5 = {
_5 = _9;
_5 = [3862482695_u32,1284495130_u32,2070380809_u32,3255397748_u32];
RET = (_10,);
_11 = _9;
_4 = 24294_i16 as f32;
_3 = [2617102210_u32,1245380346_u32,1292730231_u32,1442844970_u32];
_8 = [4290884948_u32,3857654111_u32,2918739970_u32,1841693342_u32];
_8 = [4198896156_u32,1916449490_u32,3140222492_u32,398929129_u32];
_9 = [656962361_u32,1955384762_u32,1583588740_u32,111148323_u32];
_22 = [1407592413_u32,1171400907_u32,2116090906_u32,3644338975_u32,3582453196_u32,3875364026_u32,699124773_u32];
_4 = (-7037_i16) as f32;
_18 = [(-8746481106303771430_i64)];
_9 = _12;
_10 = [100177057_u32,4084018968_u32,96366691_u32,850745846_u32];
_3 = [13813459_u32,2852418473_u32,2249143731_u32,478864746_u32];
_2 = [2632475946_u32,3555002232_u32,4011128992_u32,2138249888_u32];
_18 = [(-1669543081186319917_i64)];
_20 = -_4;
_9 = _3;
_24 = &_21;
_8 = _2;
_11 = _6;
_2 = _11;
RET = _7;
_1 = _2;
match _16 {
0 => bb1,
9223372036854775807 => bb7,
_ => bb6
}
}
bb6 = {
RET = (_5,);
_21 = 5570719086139032086_i64 as i128;
_7.0 = _8;
_16 = 9223372036854775807_isize;
_12 = [3239477378_u32,563160813_u32,2930539142_u32,3911876629_u32];
_15 = _14;
_10 = _3;
_12 = [2754397732_u32,2632852672_u32,3150754115_u32,2233402148_u32];
_21 = (-165382190019499936528901789528177178631_i128) & 89595863882762582635104525527813895608_i128;
Goto(bb5)
}
bb7 = {
_4 = -_20;
_12 = [4215872035_u32,2254328800_u32,1814559100_u32,308300191_u32];
_7.0 = [3147270686_u32,2847353862_u32,4193363150_u32,741694844_u32];
_14 = -_15;
_6 = [526290020_u32,3560569994_u32,2794990497_u32,3535793933_u32];
RET = (_8,);
_1 = [3949704422_u32,364090931_u32,2222451820_u32,1188472660_u32];
RET = (_1,);
_3 = [1953125921_u32,4203495456_u32,1443977948_u32,3450137554_u32];
_20 = _4;
_26 = _8;
_5 = _9;
_22 = [3141211326_u32,3178884676_u32,423050064_u32,2806659101_u32,4128988279_u32,1254944715_u32,3337625719_u32];
match _16 {
0 => bb4,
1 => bb2,
2 => bb8,
3 => bb9,
9223372036854775807 => bb11,
_ => bb10
}
}
bb8 = {
_11 = _1;
RET.0 = _7.0;
RET.0 = _3;
_7 = RET;
_4 = 9048928572916519657_u64 as f32;
_12 = [2145676651_u32,1379126105_u32,1663042100_u32,1351952690_u32];
_7 = (_8,);
_7 = (_12,);
_5 = _11;
_9 = _3;
Goto(bb2)
}
bb9 = {
_12 = [335222057_u32,3985191160_u32,958940904_u32,3928416514_u32];
_8 = [1400768047_u32,616482470_u32,1780304714_u32,3399807352_u32];
RET = (_12,);
_7 = (_12,);
_5 = [1792793329_u32,3053705183_u32,3002066896_u32,2035117240_u32];
_10 = [3084000156_u32,1789529664_u32,1355271634_u32,1248109635_u32];
_2 = _8;
RET.0 = [800324550_u32,1164658925_u32,2545029382_u32,89652997_u32];
RET.0 = _5;
_10 = [1450395674_u32,2592283373_u32,200125884_u32,3655051234_u32];
_2 = _8;
_2 = [3542128960_u32,4120197684_u32,1808524195_u32,397303570_u32];
_10 = _11;
Goto(bb4)
}
bb10 = {
RET.0 = [4052690745_u32,1297551992_u32,500389095_u32,1022634736_u32];
_10 = [56303824_u32,2415049005_u32,1189480745_u32,2003568363_u32];
_4 = 2738570053_u32 as f32;
_2 = [3691562185_u32,2721207752_u32,352605504_u32,1590603649_u32];
_16 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_6 = [2887503683_u32,1572143760_u32,1824231828_u32,473585930_u32];
_7.0 = [1542052226_u32,3177048305_u32,54210578_u32,27680409_u32];
_9 = [2288668532_u32,304463671_u32,4197916923_u32,2426586549_u32];
_14 = 15231160318105507601_usize as f64;
_7.0 = _3;
_12 = _11;
RET.0 = _9;
_16 = (-6994850805399469674_i64) as isize;
_6 = _7.0;
_1 = [898218501_u32,2225674662_u32,3569492795_u32,3813812785_u32];
_12 = [3459847720_u32,2936862122_u32,2682715816_u32,3717098045_u32];
RET = (_10,);
Goto(bb3)
}
bb11 = {
RET = (_26,);
_24 = &(*_24);
_9 = [1909290348_u32,2899253168_u32,3350825988_u32,1345356166_u32];
RET.0 = _26;
match _16 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb5,
9223372036854775807 => bb13,
_ => bb12
}
}
bb12 = {
RET.0 = [4052690745_u32,1297551992_u32,500389095_u32,1022634736_u32];
_10 = [56303824_u32,2415049005_u32,1189480745_u32,2003568363_u32];
_4 = 2738570053_u32 as f32;
_2 = [3691562185_u32,2721207752_u32,352605504_u32,1590603649_u32];
_16 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_6 = [2887503683_u32,1572143760_u32,1824231828_u32,473585930_u32];
_7.0 = [1542052226_u32,3177048305_u32,54210578_u32,27680409_u32];
_9 = [2288668532_u32,304463671_u32,4197916923_u32,2426586549_u32];
_14 = 15231160318105507601_usize as f64;
_7.0 = _3;
_12 = _11;
RET.0 = _9;
_16 = (-6994850805399469674_i64) as isize;
_6 = _7.0;
_1 = [898218501_u32,2225674662_u32,3569492795_u32,3813812785_u32];
_12 = [3459847720_u32,2936862122_u32,2682715816_u32,3717098045_u32];
RET = (_10,);
Goto(bb3)
}
bb13 = {
_27 = 4_usize as i16;
_11 = [1017299679_u32,3640242522_u32,3966965426_u32,4203313255_u32];
RET.0 = [2694518982_u32,2589917226_u32,3647496873_u32,4224752463_u32];
_21 = (-88980704002593857554271938894200500279_i128) >> _27;
_1 = [307488398_u32,606305068_u32,167107162_u32,717679223_u32];
_9 = RET.0;
_9 = [1514936079_u32,771244028_u32,1761918930_u32,1760438259_u32];
_26 = [1260028822_u32,276105899_u32,2873916418_u32,1792800140_u32];
_18 = [3136177908029917226_i64];
_27 = 9881_i16;
_1 = [875976577_u32,1784723127_u32,2480138507_u32,4288598354_u32];
_15 = -_14;
_15 = -_14;
Goto(bb14)
}
bb14 = {
_1 = [3433109348_u32,3904473804_u32,1486334445_u32,3871430686_u32];
RET.0 = _7.0;
_26 = _11;
_26 = _1;
_24 = &_21;
_9 = _6;
_24 = &(*_24);
_31.fld0 = [(-113_i8),(-82_i8),110_i8];
_21 = 90_i8 as i128;
_9 = [2123978113_u32,1617055454_u32,2998130744_u32,3522582095_u32];
RET = (_6,);
_14 = _15;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(12_usize, 6_usize, Move(_6), 7_usize, Move(_7), 21_usize, Move(_21), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(12_usize, 12_usize, Move(_12), 8_usize, Move(_8), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [u32; 4]) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _2: [u32; 7];
let _3: [i8; 3];
let _4: char;
let _5: bool;
let _6: &'static f32;
let _7: &'static *mut [isize; 5];
let _8: char;
let _9: *mut [u32; 4];
let _10: f32;
let _11: f64;
let _12: isize;
let _13: [u32; 4];
let _14: *const f64;
let _15: f64;
let _16: (u32, *mut bool, i128, i16);
let _17: usize;
let _18: [u32; 3];
let _19: isize;
let _20: [u16; 4];
let _21: [i32; 7];
let _22: (([u32; 4],), bool, (&'static *const f64,));
let _23: char;
let _24: isize;
let _25: [u32; 7];
let _26: &'static u16;
let _27: *mut &'static f32;
let _28: (&'static u8,);
let _29: i16;
let _30: Adt44;
let _31: f64;
let _32: f32;
let _33: ();
let _34: ();
{
RET = [200051381469897523202349296642772377757_u128,105390752950597299410637329937214985309_u128,97854914516085412687499042738478825030_u128,39405904358513416753433458621146096910_u128,241316805649296147754477681431303800744_u128,295417014328776662597111102850998620681_u128,314982824838116875978103455200995396567_u128];
RET = [141006675282257540618293388226212864936_u128,182828344238874295653042316670804790284_u128,158766705375064190711109217430228770236_u128,166730156499814687507406576246456939680_u128,86690322851612699945426942139211419221_u128,61243761575930816196885453129896307586_u128,248534886484363388881538509180841415613_u128];
RET = [288047143618212124429972220900480289033_u128,10150511505710330784344159402480353097_u128,1381483714441562482351865316865789332_u128,44351173644528316312522000684055650266_u128,55738797230671801975406787828130656492_u128,138254193902975222437814440818724861472_u128,103726388743423666859193121411621671509_u128];
RET = [242929270206128843430220804597304636220_u128,329263080598156143709974932607687159762_u128,42224978811223971715423803314239666947_u128,263024765950371094537678766810710657739_u128,256741173198205258018106797279705031034_u128,107242722575942576552327741288905831228_u128,174167438124435331153104644627451801543_u128];
_1 = [4027763856_u32,915292312_u32,1880414038_u32,2572982354_u32];
RET = [238978037545505114025919108626816696135_u128,217942319620138217168289339777810513611_u128,83652982871101456083710424809275428062_u128,179999131818599448846780258432126854862_u128,242743925589008920237652384018745834835_u128,324046480031605411369415966231138477640_u128,163065956597608334760958945775431533866_u128];
_1 = [2702587361_u32,2528034914_u32,684784373_u32,1643878941_u32];
_2 = [3716745861_u32,392545828_u32,4177002732_u32,3422356231_u32,2160930424_u32,667405399_u32,1152414158_u32];
RET = [319259239006867706183128555912144954159_u128,178322497063534178102020098078995416143_u128,110992826894903782700411210289830014984_u128,225491664272816217914943900351442660033_u128,93789089921887826988222599112205001298_u128,154847546201756234594983958439408780378_u128,83759588892767369929074138923835660312_u128];
RET = [153401926636028866350168523417994271820_u128,292429032246456715918373694159893316398_u128,282171597904516323872676842256515957956_u128,202643720598785378346984842459319451083_u128,259511544981553183834529828442194815972_u128,168452690461081027495120459763009220801_u128,274075508545124038369398002767093344405_u128];
RET = [145673119039786425040107594636879985894_u128,110659625858235766731692262002255352054_u128,156189258943703006468876609482616683082_u128,181669633631351715881399787516018629097_u128,12130781627080147349464541785740422872_u128,93007548630883626414907306965138711909_u128,296264786961273213540891179833659918809_u128];
_2 = [4176564267_u32,4196688707_u32,2558736735_u32,131568097_u32,2027691257_u32,2553362871_u32,4050693376_u32];
_2 = [1257708206_u32,1633864152_u32,2562862548_u32,1587991775_u32,768634795_u32,3189804685_u32,1246200625_u32];
_1 = [436552124_u32,2712480177_u32,820868018_u32,4057837144_u32];
_2 = [3564453755_u32,190368595_u32,3561091467_u32,990845122_u32,2593725039_u32,101543635_u32,4139403507_u32];
Goto(bb1)
}
bb1 = {
_3 = [55_i8,(-112_i8),(-17_i8)];
RET = [24978875415980421593449113343636572513_u128,217089348137532322977715071587416036572_u128,316099903960672248052840459979270832112_u128,207440479386687763360383717490285911108_u128,314465756392269196359234819171829858293_u128,68315035868742161914244392514418289724_u128,169739586061254206045980170940968110981_u128];
RET = [228280111313489699924546139873137431212_u128,152013955927132569842498106251762331444_u128,108097516871628874287823603431810000055_u128,118747260139934019978263767582404925226_u128,63791105676725305567975283609447874078_u128,80465839261196620722696050812637999410_u128,190905663413417167116977199966493150882_u128];
_2 = [685413654_u32,3802968398_u32,4271724291_u32,2722815490_u32,1534251779_u32,2983461366_u32,4064422591_u32];
RET = [116132036259979390862005899130090701989_u128,85344911672963424185931925080984479437_u128,329790772518077428057058967717584436779_u128,183379630716900506569195857937056477151_u128,186567228889165170165511562471988555460_u128,67163121344853556866689599066253721583_u128,271410899096454410918542086503710284681_u128];
_3 = [32_i8,(-10_i8),(-79_i8)];
RET = [297966298755920895209357903231179354312_u128,242985825405101668806776849661941350767_u128,163685176306582099541148939163242854971_u128,228363440852901453275767820769897050262_u128,314091986269737536563148947129047517689_u128,65679155746299902641127252283678748767_u128,240858778972106527381085900570653315738_u128];
_2 = [3968025768_u32,1678953627_u32,3608730027_u32,3710474418_u32,2956660434_u32,3883906968_u32,1867903626_u32];
_4 = '\u{7eae0}';
Goto(bb2)
}
bb2 = {
_3 = [126_i8,65_i8,(-92_i8)];
_2 = [876501363_u32,2054573669_u32,1373071910_u32,2128176387_u32,3081329317_u32,3879389909_u32,410208325_u32];
_3 = [(-86_i8),98_i8,(-126_i8)];
_2 = [2771334936_u32,3421319587_u32,1664476524_u32,1145951067_u32,4247485667_u32,1089104195_u32,681462250_u32];
Goto(bb3)
}
bb3 = {
RET = [46303970841589037153321634866887596735_u128,338208530985227897141835062531007561114_u128,210677154702144010953283503112401848478_u128,315623329151789525539040333815801239844_u128,112709767264141770383326276537146751704_u128,26469679375987514729588962636153137472_u128,82879618639418325841804996146937031242_u128];
_3 = [81_i8,85_i8,(-68_i8)];
_5 = !true;
RET = [258052059237605468042530697052827462356_u128,39621932392126543792594825558090763802_u128,40239737161784199924831466334247577329_u128,334989752795471042847462363486480965561_u128,250719084500561472090202861440448203741_u128,53543675289085663721748583112918353837_u128,131777898830355218122773867301840326066_u128];
_1 = [874250561_u32,1521960810_u32,1163827156_u32,4044192749_u32];
_5 = true;
_5 = true;
_5 = true;
_1 = [1235758918_u32,886477651_u32,437047986_u32,3945314919_u32];
_2 = [4176884412_u32,3538257183_u32,568994229_u32,1510762653_u32,1092794248_u32,3869980110_u32,3695328568_u32];
_1 = [1571830869_u32,1344546244_u32,3126733686_u32,2127653974_u32];
RET = [168888679637545121571032691588602542993_u128,250119158576181852580975428038117149211_u128,197055599978832089562332241498226114228_u128,243498942517663743891810726451547529894_u128,213331002158004939526523978544638599175_u128,322072391686576115674873336134707292771_u128,172713381715048904445627118445629883873_u128];
_5 = false;
_4 = '\u{d9da0}';
_2 = [4180689877_u32,3938001583_u32,2730920407_u32,292631553_u32,3073773210_u32,3658904480_u32,1260197795_u32];
_5 = _4 < _4;
_2 = [938044580_u32,2907497086_u32,1963490852_u32,1648551863_u32,3626886894_u32,2573293387_u32,1162267612_u32];
RET = [60976424432365501484405226389367480705_u128,96520478313877699966242141634028055638_u128,198153411276992537042233584944700790889_u128,46808097936006020888191173783884223260_u128,39860349363758283240085726625827370180_u128,64378952785259688384834284503643133946_u128,131881440161961776385108137694612051492_u128];
_1 = [1138407092_u32,3416550818_u32,2268052204_u32,685871897_u32];
_5 = _4 > _4;
_1 = [1037429739_u32,2752659153_u32,3164389010_u32,2433872580_u32];
_3 = [1_i8,(-16_i8),(-77_i8)];
_3 = [70_i8,(-46_i8),119_i8];
RET = [54521050200672146868770328396774656864_u128,82721711503102190502109080236918716584_u128,73052762732183496735788510968848477279_u128,50920922532470727256475206956957377550_u128,62930407904894822633262348001472427041_u128,84866514952556583814246951224888545407_u128,247306653406436649184495047400542536298_u128];
_5 = _4 <= _4;
_2 = [3334730241_u32,2083992437_u32,1188070721_u32,2118341169_u32,3399637277_u32,1905788260_u32,1426516073_u32];
Goto(bb4)
}
bb4 = {
RET = [234664580389328058217657035157624062356_u128,264985079841468098360201663587986594409_u128,308503060382396109591341415483079772790_u128,316504287055025270397415984760813470647_u128,13972756723746048716067741092114807209_u128,70322740345975083725936239139097157395_u128,192044417890053507578084940962140079812_u128];
_1 = [3166523374_u32,3841056477_u32,1271711512_u32,1591192763_u32];
_8 = _4;
Goto(bb5)
}
bb5 = {
RET = [151370999936240097197815486342869632937_u128,18505481484576211410180983564027688235_u128,198309986568371657537209192400003577179_u128,94567097877202470897394515696422639222_u128,56396224847596519328908706253412920412_u128,28743738484071014665783153970176539349_u128,187675504745192383882218517051062215925_u128];
_2 = [4006476696_u32,1640804043_u32,796282680_u32,3006721004_u32,3751629337_u32,193353243_u32,2039761980_u32];
_6 = &_10;
_10 = 119_u8 as f32;
RET = [245587816188964770038018471465571719605_u128,69253557966433301623864286219603493920_u128,106587177446733828559766368045546739412_u128,264696468547093044030869102217332643575_u128,136579122997732499608083849661293892382_u128,160084479246924096605711908070049737738_u128,252563734762059365762743118453847113740_u128];
_9 = core::ptr::addr_of_mut!(_1);
(*_9) = [317302788_u32,2556486383_u32,99160726_u32,3878297107_u32];
Goto(bb6)
}
bb6 = {
_13 = (*_9);
_11 = 2823223411_u32 as f64;
_11 = _10 as f64;
_11 = 16355853697184289657_u64 as f64;
(*_9) = _13;
_2 = [4024858725_u32,3054318101_u32,339116590_u32,1112638347_u32,81588267_u32,1102628880_u32,4140834792_u32];
_10 = _11 as f32;
_1 = [3606984788_u32,3948062752_u32,216204094_u32,285182888_u32];
_8 = _4;
_4 = _8;
(*_9) = [2724000535_u32,1524701488_u32,3499153531_u32,3249233163_u32];
_4 = _8;
_15 = -_11;
_16.0 = _10 as u32;
_16.2 = 102307866879517746217841431876551061637_i128 - 75268837934527525868965073544099548788_i128;
_4 = _8;
_12 = 9223372036854775807_isize;
_16.0 = 3218998620_u32 ^ 3621474761_u32;
_6 = &_10;
_8 = _4;
match _12 {
0 => bb1,
1 => bb2,
2 => bb5,
9223372036854775807 => bb7,
_ => bb4
}
}
bb7 = {
_5 = !false;
_16.1 = core::ptr::addr_of_mut!(_5);
_16.1 = core::ptr::addr_of_mut!(_5);
RET = [334408029759783974942598329090305513967_u128,298672165874871640094947394854771789953_u128,117014951354791970390836640470578873554_u128,171442063529938649361327241592823778489_u128,119921942061496904287886723440898963829_u128,320904579407295189710600399424391559751_u128,52165550368134124993852383939939751403_u128];
_16.0 = 2136709909_u32 ^ 1941204237_u32;
_16.3 = 50_i8 as i16;
_15 = _11 + _11;
_3 = [80_i8,(-64_i8),48_i8];
_16.1 = core::ptr::addr_of_mut!(_5);
_14 = core::ptr::addr_of!(_15);
_8 = _4;
_12 = 9223372036854775807_isize;
RET = [218035578885504798236290414011286864598_u128,31283387962962670576954418456477354707_u128,25055924943751977829859814973436367727_u128,304798136676277699839340172012128772829_u128,289366602641266060362586530407971170235_u128,333267166468666100375211929785011436753_u128,197240571267525345590935557032316593483_u128];
_13 = (*_9);
_16.1 = core::ptr::addr_of_mut!(_5);
_16.3 = !(-24731_i16);
Goto(bb8)
}
bb8 = {
_9 = core::ptr::addr_of_mut!(_1);
_4 = _8;
_10 = 13558_u16 as f32;
_16.3 = 28657_i16 * 187_i16;
RET = [329823145711321383844383354323801294438_u128,648504381970156573587088592283416681_u128,320929739622594140187103969512139866651_u128,92487864053862335579849162410731226616_u128,203462679709248007651928426341750597354_u128,333333973943947810198754978964970801675_u128,177710033446854614000394483277479540267_u128];
RET = [326035320079460347724613115933131241406_u128,263999199306696483259771407594102443282_u128,249044909765422930043701789052414762078_u128,333556253890850594445786122662966659487_u128,76890325450905132060992019587464442472_u128,177782302178651436491337424459523425283_u128,223295390816681995351538454449774110950_u128];
_15 = _11;
_3 = [89_i8,18_i8,(-69_i8)];
Goto(bb9)
}
bb9 = {
_16.3 = (-6013_i16) * (-31092_i16);
_19 = _5 as isize;
_17 = 6_usize >> _16.0;
match _12 {
0 => bb5,
1 => bb2,
2 => bb10,
3 => bb11,
9223372036854775807 => bb13,
_ => bb12
}
}
bb10 = {
_3 = [55_i8,(-112_i8),(-17_i8)];
RET = [24978875415980421593449113343636572513_u128,217089348137532322977715071587416036572_u128,316099903960672248052840459979270832112_u128,207440479386687763360383717490285911108_u128,314465756392269196359234819171829858293_u128,68315035868742161914244392514418289724_u128,169739586061254206045980170940968110981_u128];
RET = [228280111313489699924546139873137431212_u128,152013955927132569842498106251762331444_u128,108097516871628874287823603431810000055_u128,118747260139934019978263767582404925226_u128,63791105676725305567975283609447874078_u128,80465839261196620722696050812637999410_u128,190905663413417167116977199966493150882_u128];
_2 = [685413654_u32,3802968398_u32,4271724291_u32,2722815490_u32,1534251779_u32,2983461366_u32,4064422591_u32];
RET = [116132036259979390862005899130090701989_u128,85344911672963424185931925080984479437_u128,329790772518077428057058967717584436779_u128,183379630716900506569195857937056477151_u128,186567228889165170165511562471988555460_u128,67163121344853556866689599066253721583_u128,271410899096454410918542086503710284681_u128];
_3 = [32_i8,(-10_i8),(-79_i8)];
RET = [297966298755920895209357903231179354312_u128,242985825405101668806776849661941350767_u128,163685176306582099541148939163242854971_u128,228363440852901453275767820769897050262_u128,314091986269737536563148947129047517689_u128,65679155746299902641127252283678748767_u128,240858778972106527381085900570653315738_u128];
_2 = [3968025768_u32,1678953627_u32,3608730027_u32,3710474418_u32,2956660434_u32,3883906968_u32,1867903626_u32];
_4 = '\u{7eae0}';
Goto(bb2)
}
bb11 = {
RET = [46303970841589037153321634866887596735_u128,338208530985227897141835062531007561114_u128,210677154702144010953283503112401848478_u128,315623329151789525539040333815801239844_u128,112709767264141770383326276537146751704_u128,26469679375987514729588962636153137472_u128,82879618639418325841804996146937031242_u128];
_3 = [81_i8,85_i8,(-68_i8)];
_5 = !true;
RET = [258052059237605468042530697052827462356_u128,39621932392126543792594825558090763802_u128,40239737161784199924831466334247577329_u128,334989752795471042847462363486480965561_u128,250719084500561472090202861440448203741_u128,53543675289085663721748583112918353837_u128,131777898830355218122773867301840326066_u128];
_1 = [874250561_u32,1521960810_u32,1163827156_u32,4044192749_u32];
_5 = true;
_5 = true;
_5 = true;
_1 = [1235758918_u32,886477651_u32,437047986_u32,3945314919_u32];
_2 = [4176884412_u32,3538257183_u32,568994229_u32,1510762653_u32,1092794248_u32,3869980110_u32,3695328568_u32];
_1 = [1571830869_u32,1344546244_u32,3126733686_u32,2127653974_u32];
RET = [168888679637545121571032691588602542993_u128,250119158576181852580975428038117149211_u128,197055599978832089562332241498226114228_u128,243498942517663743891810726451547529894_u128,213331002158004939526523978544638599175_u128,322072391686576115674873336134707292771_u128,172713381715048904445627118445629883873_u128];
_5 = false;
_4 = '\u{d9da0}';
_2 = [4180689877_u32,3938001583_u32,2730920407_u32,292631553_u32,3073773210_u32,3658904480_u32,1260197795_u32];
_5 = _4 < _4;
_2 = [938044580_u32,2907497086_u32,1963490852_u32,1648551863_u32,3626886894_u32,2573293387_u32,1162267612_u32];
RET = [60976424432365501484405226389367480705_u128,96520478313877699966242141634028055638_u128,198153411276992537042233584944700790889_u128,46808097936006020888191173783884223260_u128,39860349363758283240085726625827370180_u128,64378952785259688384834284503643133946_u128,131881440161961776385108137694612051492_u128];
_1 = [1138407092_u32,3416550818_u32,2268052204_u32,685871897_u32];
_5 = _4 > _4;
_1 = [1037429739_u32,2752659153_u32,3164389010_u32,2433872580_u32];
_3 = [1_i8,(-16_i8),(-77_i8)];
_3 = [70_i8,(-46_i8),119_i8];
RET = [54521050200672146868770328396774656864_u128,82721711503102190502109080236918716584_u128,73052762732183496735788510968848477279_u128,50920922532470727256475206956957377550_u128,62930407904894822633262348001472427041_u128,84866514952556583814246951224888545407_u128,247306653406436649184495047400542536298_u128];
_5 = _4 <= _4;
_2 = [3334730241_u32,2083992437_u32,1188070721_u32,2118341169_u32,3399637277_u32,1905788260_u32,1426516073_u32];
Goto(bb4)
}
bb12 = {
_3 = [126_i8,65_i8,(-92_i8)];
_2 = [876501363_u32,2054573669_u32,1373071910_u32,2128176387_u32,3081329317_u32,3879389909_u32,410208325_u32];
_3 = [(-86_i8),98_i8,(-126_i8)];
_2 = [2771334936_u32,3421319587_u32,1664476524_u32,1145951067_u32,4247485667_u32,1089104195_u32,681462250_u32];
Goto(bb3)
}
bb13 = {
_22.2.0 = &_14;
_12 = _19 << _17;
_2 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
(*_14) = _11 * _11;
RET = [173001324953107132788035658930801902376_u128,200962150482672477331880009330481068886_u128,244629422526363868018782964695463425786_u128,57237674520680812389320885605417162214_u128,157189344737019543047381110990408105018_u128,179928924610956422916562812057512179009_u128,166917222903179914003006528580859042470_u128];
_22.1 = _5;
_22.0 = ((*_9),);
_24 = _12;
_20 = [17666_u16,45034_u16,9027_u16,14215_u16];
_16.2 = _5 as i128;
_22.1 = (*_14) > _11;
_1 = [_16.0,_16.0,_16.0,_16.0];
_10 = (-1097045310_i32) as f32;
_6 = &_10;
_19 = _16.0 as isize;
_22.1 = !_5;
Call((*_9) = core::intrinsics::transmute(_22.0.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_9 = core::ptr::addr_of_mut!(_1);
_22.2.0 = &_14;
(*_9) = [_16.0,_16.0,_16.0,_16.0];
_22.0 = ((*_9),);
_27 = core::ptr::addr_of_mut!(_6);
(*_9) = _13;
_4 = _8;
_4 = _8;
_18 = [_16.0,_16.0,_16.0];
_22.0 = ((*_9),);
_16.3 = !8673_i16;
_22.1 = _5;
_3 = [47_i8,42_i8,78_i8];
_5 = (*_14) == (*_14);
_16.3 = 4851_i16 | 10754_i16;
_29 = 146_u8 as i16;
_22.2.0 = &_14;
_8 = _4;
_22.1 = !_5;
_22.2.0 = &_14;
_20 = [1442_u16,45218_u16,11611_u16,47916_u16];
_2 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_6 = &_10;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(13_usize, 2_usize, Move(_2), 13_usize, Move(_13), 17_usize, Move(_17), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(13_usize, 3_usize, Move(_3), 20_usize, Move(_20), 8_usize, Move(_8), 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: [u128; 7],mut _3: [u128; 7],mut _4: [u64; 4],mut _5: isize,mut _6: isize,mut _7: [u32; 4],mut _8: [u32; 4]) -> bool {
mir! {
type RET = bool;
let _9: f64;
let _10: (u32, *mut bool, i128, i16);
let _11: isize;
let _12: [u32; 2];
let _13: &'static (u8,);
let _14: isize;
let _15: u64;
let _16: &'static (u8,);
let _17: i8;
let _18: isize;
let _19: isize;
let _20: *const Adt31;
let _21: bool;
let _22: f32;
let _23: isize;
let _24: [u32; 7];
let _25: *mut [isize; 5];
let _26: char;
let _27: Adt42;
let _28: *const (&'static *const f64,);
let _29: char;
let _30: (&'static u8,);
let _31: f32;
let _32: (*const u16, *const Adt31, [i32; 7]);
let _33: Adt56;
let _34: i16;
let _35: *const *const Adt25;
let _36: i128;
let _37: ([u128; 7],);
let _38: *mut [i32; 7];
let _39: Adt42;
let _40: (&'static u8,);
let _41: ();
let _42: ();
{
_9 = 37146806039313500842035078635424356234_i128 as f64;
_9 = (-115_i8) as f64;
_3 = [50211748786707481887079015948770296646_u128,317335452655871354725102921326370538850_u128,16198977796498537099659946645608868693_u128,173819262401583075498739786124779769282_u128,340265213213098793329751962669532561908_u128,179378910267887059925281625098933809157_u128,94933992661697840271180291923089625741_u128];
RET = _5 >= _5;
_8 = _7;
_9 = 8992_i16 as f64;
_3 = [175471921145090071593607804873659582723_u128,119585377168345343652096870696125267583_u128,148936588659585332108353775972414685105_u128,260051948530955616239113236336883085406_u128,77592350766431707248885471685117316352_u128,263134794574270906330199213745235059793_u128,309307729824896779223267248819271619182_u128];
_6 = _1;
_10.2 = 7_usize as i128;
Call(_10.3 = fn15(_1, _6, _6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _1 != _1;
_2 = [222734062121981721081127306611016395625_u128,137666105562514294653613003703598578748_u128,158222511227312928561362735686773774425_u128,239101431802513324954551417185444243574_u128,176803230446602224614775029765230499303_u128,134346531152166440467997751930305543475_u128,95030247253343005913707287426083958368_u128];
_4 = [17378110863080782507_u64,6727425170375878502_u64,9834307649025101171_u64,13636636294469396876_u64];
_10.0 = !61392994_u32;
_9 = 15927444156419487491_u64 as f64;
_5 = (-78_i8) as isize;
RET = false;
_8 = [_10.0,_10.0,_10.0,_10.0];
_1 = _6;
_11 = !_1;
_11 = _6;
_10.0 = _10.2 as u32;
_10.2 = -(-96101137472464409965417960281354985876_i128);
_3 = _2;
RET = false;
_10.3 = (-17697_i16) ^ (-758_i16);
_9 = 29103_u16 as f64;
_8 = _7;
_3 = [224761726697776504731393153013203902086_u128,141285992786634527275092012757615399828_u128,333886563621429679081155130765801963168_u128,112127525325880211217410062334041406093_u128,15599612743074385172942101149946426343_u128,139815005897851609165263921556221885271_u128,127171424241067112029356810356579865554_u128];
Goto(bb2)
}
bb2 = {
_12 = [_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_10.3 = 28249_i16;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.2 = -110960244101146279765364646843708732804_i128;
_8 = [_10.0,_10.0,_10.0,_10.0];
RET = !false;
_5 = _11;
_8 = [_10.0,_10.0,_10.0,_10.0];
_7 = _8;
_15 = 193_u8 as u64;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_15 = 11581852362890770783_u64 >> _6;
_6 = _10.0 as isize;
_1 = _11 | _6;
_10.2 = 237046877202236615318084001141334315329_u128 as i128;
_10.3 = -11592_i16;
_17 = 483743477263012779_usize as i8;
_18 = -_5;
Goto(bb3)
}
bb3 = {
_8 = [_10.0,_10.0,_10.0,_10.0];
_3 = _2;
_6 = _18 - _11;
_10.1 = core::ptr::addr_of_mut!(RET);
_8 = [_10.0,_10.0,_10.0,_10.0];
_10.2 = 55129943379691488997618876770310921502_i128;
_17 = (-34_i8) >> _6;
_2 = [6163242442233354683060830659912247829_u128,254806078929790892828542030871465248339_u128,126127730928651679969997864101380930464_u128,321358746509127110000872542404853737578_u128,11342616003736632134339530565320121355_u128,184378016582530963400539191159453232306_u128,84156706174882905735142027497741738313_u128];
_10.1 = core::ptr::addr_of_mut!(RET);
_3 = [311953923916702419352977588232793294367_u128,277095158280161081021853866052510960584_u128,239290832424276497173986556506542691478_u128,184176504404585221816254462057200515804_u128,74750341516024422279041751752931189444_u128,57748478588571823611402032584894376556_u128,254453087277836089088980545667350574010_u128];
_3 = [119142333286394725352055988482854585178_u128,196651538034637758413356956560969579798_u128,55427012856969907255745316598507600623_u128,209677713422141811381182159625179004829_u128,329860033083928477112480179087362201726_u128,118650480416494561187293598028104665088_u128,60761363613056850289237217214752780097_u128];
_2 = [183310918584017306638858049785632048261_u128,34357351226255111376470798513950242709_u128,236392040020217193000270011999912388267_u128,1622705761347223403710298332454381887_u128,75677617545297594351717832127353349364_u128,86153050890287834531705479354771090717_u128,100955245027290841422974501141864337166_u128];
_7 = [_10.0,_10.0,_10.0,_10.0];
_15 = 8433283856645701994_u64 + 13966956619916561131_u64;
_2 = [252875731439040886668076745817882367918_u128,32686487928226668764929534117095925720_u128,330866122018925407537632588658050888238_u128,321789231732312098796567733751203848079_u128,27698108722477402140621975643035975103_u128,91606621453778897009364291544082969739_u128,11968782624040921147115664751876737069_u128];
_17 = 82_i8 + 117_i8;
_12 = [_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_2 = _3;
RET = !true;
_8 = [_10.0,_10.0,_10.0,_10.0];
_10.3 = 26935_i16;
_11 = _15 as isize;
_14 = !_1;
_12 = [_10.0,_10.0];
Goto(bb4)
}
bb4 = {
_1 = _5 & _6;
RET = false;
_8 = [_10.0,_10.0,_10.0,_10.0];
Goto(bb5)
}
bb5 = {
_22 = _10.0 as f32;
_10.2 = 4796612993408989125_i64 as i128;
RET = !false;
Goto(bb6)
}
bb6 = {
_5 = _18 & _1;
_23 = _6;
_22 = 48496_u16 as f32;
_10.2 = 16_u8 as i128;
_9 = _17 as f64;
_10.2 = _22 as i128;
_9 = _10.0 as f64;
_15 = !6583655779388772263_u64;
_24 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_9 = 47913627773498054935073422344766994825_u128 as f64;
_17 = (-35_i8) - 16_i8;
_18 = _10.3 as isize;
_23 = !_5;
_8 = _7;
_23 = _1 >> _6;
RET = !false;
_10.0 = 5875915486662521257_i64 as u32;
match _10.3 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
26935 => bb12,
_ => bb11
}
}
bb7 = {
_22 = _10.0 as f32;
_10.2 = 4796612993408989125_i64 as i128;
RET = !false;
Goto(bb6)
}
bb8 = {
_1 = _5 & _6;
RET = false;
_8 = [_10.0,_10.0,_10.0,_10.0];
Goto(bb5)
}
bb9 = {
_8 = [_10.0,_10.0,_10.0,_10.0];
_3 = _2;
_6 = _18 - _11;
_10.1 = core::ptr::addr_of_mut!(RET);
_8 = [_10.0,_10.0,_10.0,_10.0];
_10.2 = 55129943379691488997618876770310921502_i128;
_17 = (-34_i8) >> _6;
_2 = [6163242442233354683060830659912247829_u128,254806078929790892828542030871465248339_u128,126127730928651679969997864101380930464_u128,321358746509127110000872542404853737578_u128,11342616003736632134339530565320121355_u128,184378016582530963400539191159453232306_u128,84156706174882905735142027497741738313_u128];
_10.1 = core::ptr::addr_of_mut!(RET);
_3 = [311953923916702419352977588232793294367_u128,277095158280161081021853866052510960584_u128,239290832424276497173986556506542691478_u128,184176504404585221816254462057200515804_u128,74750341516024422279041751752931189444_u128,57748478588571823611402032584894376556_u128,254453087277836089088980545667350574010_u128];
_3 = [119142333286394725352055988482854585178_u128,196651538034637758413356956560969579798_u128,55427012856969907255745316598507600623_u128,209677713422141811381182159625179004829_u128,329860033083928477112480179087362201726_u128,118650480416494561187293598028104665088_u128,60761363613056850289237217214752780097_u128];
_2 = [183310918584017306638858049785632048261_u128,34357351226255111376470798513950242709_u128,236392040020217193000270011999912388267_u128,1622705761347223403710298332454381887_u128,75677617545297594351717832127353349364_u128,86153050890287834531705479354771090717_u128,100955245027290841422974501141864337166_u128];
_7 = [_10.0,_10.0,_10.0,_10.0];
_15 = 8433283856645701994_u64 + 13966956619916561131_u64;
_2 = [252875731439040886668076745817882367918_u128,32686487928226668764929534117095925720_u128,330866122018925407537632588658050888238_u128,321789231732312098796567733751203848079_u128,27698108722477402140621975643035975103_u128,91606621453778897009364291544082969739_u128,11968782624040921147115664751876737069_u128];
_17 = 82_i8 + 117_i8;
_12 = [_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_2 = _3;
RET = !true;
_8 = [_10.0,_10.0,_10.0,_10.0];
_10.3 = 26935_i16;
_11 = _15 as isize;
_14 = !_1;
_12 = [_10.0,_10.0];
Goto(bb4)
}
bb10 = {
_12 = [_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_10.3 = 28249_i16;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.2 = -110960244101146279765364646843708732804_i128;
_8 = [_10.0,_10.0,_10.0,_10.0];
RET = !false;
_5 = _11;
_8 = [_10.0,_10.0,_10.0,_10.0];
_7 = _8;
_15 = 193_u8 as u64;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_15 = 11581852362890770783_u64 >> _6;
_6 = _10.0 as isize;
_1 = _11 | _6;
_10.2 = 237046877202236615318084001141334315329_u128 as i128;
_10.3 = -11592_i16;
_17 = 483743477263012779_usize as i8;
_18 = -_5;
Goto(bb3)
}
bb11 = {
RET = _1 != _1;
_2 = [222734062121981721081127306611016395625_u128,137666105562514294653613003703598578748_u128,158222511227312928561362735686773774425_u128,239101431802513324954551417185444243574_u128,176803230446602224614775029765230499303_u128,134346531152166440467997751930305543475_u128,95030247253343005913707287426083958368_u128];
_4 = [17378110863080782507_u64,6727425170375878502_u64,9834307649025101171_u64,13636636294469396876_u64];
_10.0 = !61392994_u32;
_9 = 15927444156419487491_u64 as f64;
_5 = (-78_i8) as isize;
RET = false;
_8 = [_10.0,_10.0,_10.0,_10.0];
_1 = _6;
_11 = !_1;
_11 = _6;
_10.0 = _10.2 as u32;
_10.2 = -(-96101137472464409965417960281354985876_i128);
_3 = _2;
RET = false;
_10.3 = (-17697_i16) ^ (-758_i16);
_9 = 29103_u16 as f64;
_8 = _7;
_3 = [224761726697776504731393153013203902086_u128,141285992786634527275092012757615399828_u128,333886563621429679081155130765801963168_u128,112127525325880211217410062334041406093_u128,15599612743074385172942101149946426343_u128,139815005897851609165263921556221885271_u128,127171424241067112029356810356579865554_u128];
Goto(bb2)
}
bb12 = {
_29 = '\u{1874a}';
_15 = 2941941229327047403_u64 & 3690324381812612828_u64;
_10.0 = 1442540096_u32 + 1426778162_u32;
_17 = -35_i8;
_9 = 198_u8 as f64;
_2 = [287307085684704558408222032909647903838_u128,133130473153499670649151002323329528067_u128,315470447514337253937014775844689460069_u128,242902946102154897753929642240075423691_u128,326461705888138404785889996763163742469_u128,136241305013335070321077213444900019440_u128,15427060175945842685854323364886805137_u128];
_4 = [_15,_15,_15,_15];
_31 = -_22;
_4 = [_15,_15,_15,_15];
_19 = _23;
_10.2 = 136885083504183684509614873864473482099_i128 + 48127894699866000007225127994480673268_i128;
_24 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_31 = -_22;
_18 = _23;
_23 = 1529724890964812877_usize as isize;
_18 = _10.0 as isize;
_26 = _29;
_32.2 = [(-1973397487_i32),(-1017639192_i32),1680808211_i32,(-298620550_i32),890080593_i32,(-1131572374_i32),872319839_i32];
_10.2 = !(-98039055098293359952089029593268578093_i128);
_10.1 = core::ptr::addr_of_mut!(RET);
_9 = _10.0 as f64;
_15 = !949076744652797512_u64;
_10.0 = 3177298923_u32;
RET = !true;
Goto(bb13)
}
bb13 = {
_5 = 1649354025_i32 as isize;
_33.fld0 = _8;
RET = !true;
_19 = 7145538669701883180_usize as isize;
_21 = RET;
_33.fld1 = _2;
_9 = _15 as f64;
_27 = Adt42::Variant1 { fld0: _2 };
_1 = _19;
_33.fld1 = [16174161612190654241629291860013106757_u128,52218691387060756866327606503448972406_u128,112089370213805264197345792644747894914_u128,254466678905850250430208282634651656794_u128,184317813333648215150855911918649187249_u128,121920814419297496893568549696764589799_u128,11046999574791261993622060331194640756_u128];
_5 = -_11;
_24 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
RET = _21;
_32.2 = [(-238645077_i32),1179568253_i32,1589762024_i32,(-1977492395_i32),1624661676_i32,739996479_i32,(-742552332_i32)];
_4 = [_15,_15,_15,_15];
_33.fld2.0 = [_10.0,_10.0,_10.0,_10.0];
SetDiscriminant(_27, 3);
_19 = _5 + _6;
_6 = _1 ^ _19;
place!(Field::<[u32; 4]>(Variant(_27, 3), 0)) = [_10.0,_10.0,_10.0,_10.0];
_23 = !_18;
_36 = _10.2;
place!(Field::<*mut i32>(Variant(_27, 3), 7)) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_27, 3), 5)));
_3 = _2;
match _10.3 {
0 => bb14,
26935 => bb16,
_ => bb15
}
}
bb14 = {
_12 = [_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_10.3 = 28249_i16;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.2 = -110960244101146279765364646843708732804_i128;
_8 = [_10.0,_10.0,_10.0,_10.0];
RET = !false;
_5 = _11;
_8 = [_10.0,_10.0,_10.0,_10.0];
_7 = _8;
_15 = 193_u8 as u64;
_7 = [_10.0,_10.0,_10.0,_10.0];
_10.1 = core::ptr::addr_of_mut!(RET);
_15 = 11581852362890770783_u64 >> _6;
_6 = _10.0 as isize;
_1 = _11 | _6;
_10.2 = 237046877202236615318084001141334315329_u128 as i128;
_10.3 = -11592_i16;
_17 = 483743477263012779_usize as i8;
_18 = -_5;
Goto(bb3)
}
bb15 = {
RET = _1 != _1;
_2 = [222734062121981721081127306611016395625_u128,137666105562514294653613003703598578748_u128,158222511227312928561362735686773774425_u128,239101431802513324954551417185444243574_u128,176803230446602224614775029765230499303_u128,134346531152166440467997751930305543475_u128,95030247253343005913707287426083958368_u128];
_4 = [17378110863080782507_u64,6727425170375878502_u64,9834307649025101171_u64,13636636294469396876_u64];
_10.0 = !61392994_u32;
_9 = 15927444156419487491_u64 as f64;
_5 = (-78_i8) as isize;
RET = false;
_8 = [_10.0,_10.0,_10.0,_10.0];
_1 = _6;
_11 = !_1;
_11 = _6;
_10.0 = _10.2 as u32;
_10.2 = -(-96101137472464409965417960281354985876_i128);
_3 = _2;
RET = false;
_10.3 = (-17697_i16) ^ (-758_i16);
_9 = 29103_u16 as f64;
_8 = _7;
_3 = [224761726697776504731393153013203902086_u128,141285992786634527275092012757615399828_u128,333886563621429679081155130765801963168_u128,112127525325880211217410062334041406093_u128,15599612743074385172942101149946426343_u128,139815005897851609165263921556221885271_u128,127171424241067112029356810356579865554_u128];
Goto(bb2)
}
bb16 = {
_15 = 1079284359400205072_u64;
_23 = -_6;
_1 = _6;
_10.3 = !(-24316_i16);
_37 = (_3,);
place!(Field::<[u32; 4]>(Variant(_27, 3), 0)) = [_10.0,_10.0,_10.0,_10.0];
place!(Field::<u128>(Variant(_27, 3), 3)) = 234968377888539185503660417438490161824_u128;
_8 = [_10.0,_10.0,_10.0,_10.0];
_2 = _37.0;
_23 = _6;
RET = !_21;
_10.3 = 51_u8 as i16;
_18 = _19 ^ _6;
place!(Field::<[u32; 4]>(Variant(_27, 3), 0)) = [_10.0,_10.0,_10.0,_10.0];
place!(Field::<i32>(Variant(_27, 3), 5)) = 2029585723_i32 * 280758604_i32;
_2 = [Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3),Field::<u128>(Variant(_27, 3), 3)];
_33.fld1 = _37.0;
_5 = _6;
_32.2 = [Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5),Field::<i32>(Variant(_27, 3), 5)];
_6 = _5;
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(14_usize, 29_usize, Move(_29), 12_usize, Move(_12), 37_usize, Move(_37), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(14_usize, 3_usize, Move(_3), 24_usize, Move(_24), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(14_usize, 18_usize, Move(_18), 5_usize, Move(_5), 17_usize, Move(_17), 42_usize, _42), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> i16 {
mir! {
type RET = i16;
let _5: Adt25;
let _6: [u32; 7];
let _7: u128;
let _8: &'static f32;
let _9: f32;
let _10: [i64; 1];
let _11: isize;
let _12: (*const u16, *const Adt31, [i32; 7]);
let _13: Adt25;
let _14: *mut Adt25;
let _15: Adt38;
let _16: i64;
let _17: char;
let _18: (&'static *const f64,);
let _19: (Adt38, *mut [u32; 4]);
let _20: bool;
let _21: f32;
let _22: *const u16;
let _23: [i64; 8];
let _24: u32;
let _25: i32;
let _26: (*const u16, *const Adt31, [i32; 7]);
let _27: *mut i32;
let _28: [u128; 7];
let _29: Adt35;
let _30: ();
let _31: ();
{
_3 = _1;
RET = -30197_i16;
_3 = -_4;
_4 = _2 + _1;
_3 = 54014180359694326796675438114649108199_u128 as isize;
RET = 32705_i16;
_1 = !_4;
_1 = -_4;
_1 = -_2;
RET = -20281_i16;
RET = '\u{43ae5}' as i16;
RET = (-3142_i16) & 19723_i16;
Call(_3 = fn16(_4, _2, _4, _4, _2, _2, _2, _1, RET, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = -_1;
RET = (-17819_i16) & 18877_i16;
RET = 14871766217084134289_usize as i16;
RET = !(-19439_i16);
_3 = !_1;
RET = -7962_i16;
_2 = _1;
RET = !(-922_i16);
_7 = !55979882407373952696748813691307576600_u128;
_1 = _2;
RET = 15_i8 as i16;
_7 = !323821077989606792458833979252708713134_u128;
_4 = _7 as isize;
RET = 2641_i16;
_3 = -_2;
_1 = _3;
RET = -8480_i16;
Goto(bb2)
}
bb2 = {
_6 = [4122509876_u32,2439843135_u32,842730762_u32,3273690914_u32,2480670961_u32,75341430_u32,2068434284_u32];
_6 = [1002647099_u32,2544482377_u32,1928741887_u32,1438257206_u32,4173154921_u32,1342552308_u32,2896045788_u32];
_2 = _7 as isize;
_2 = (-25_i8) as isize;
_4 = _3;
_6 = [1172341269_u32,4179311723_u32,3369868096_u32,2192360071_u32,547068266_u32,124485193_u32,2079698452_u32];
_3 = _1 * _4;
_3 = 981798283754351168_i64 as isize;
_7 = '\u{7792}' as u128;
_2 = 13189_u16 as isize;
RET = (-30271_i16) ^ 5446_i16;
_7 = false as u128;
RET = (-11491_i16) * 19634_i16;
RET = 25555_i16;
_8 = &_9;
_9 = (-147374261540454389784492408298974263927_i128) as f32;
_4 = _3;
_9 = 0_usize as f32;
_4 = 15860671811273638771_u64 as isize;
_9 = (-1144792675_i32) as f32;
_10 = [5196512539373443965_i64];
_6 = [178024321_u32,3377164855_u32,655042486_u32,39246493_u32,3064139838_u32,1425958141_u32,1957960541_u32];
_4 = !_1;
_1 = _7 as isize;
RET = _1 as i16;
RET = (-4363_i16);
_1 = _3 & _3;
_9 = 116_u8 as f32;
match RET {
0 => bb1,
1 => bb3,
340282366920938463463374607431768207093 => bb5,
_ => bb4
}
}
bb3 = {
_4 = -_1;
RET = (-17819_i16) & 18877_i16;
RET = 14871766217084134289_usize as i16;
RET = !(-19439_i16);
_3 = !_1;
RET = -7962_i16;
_2 = _1;
RET = !(-922_i16);
_7 = !55979882407373952696748813691307576600_u128;
_1 = _2;
RET = 15_i8 as i16;
_7 = !323821077989606792458833979252708713134_u128;
_4 = _7 as isize;
RET = 2641_i16;
_3 = -_2;
_1 = _3;
RET = -8480_i16;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_3 = -_2;
_11 = _4 >> _4;
RET = (-28192_i16);
_10 = [(-2449853064349023805_i64)];
_9 = 2728307508_u32 as f32;
_2 = -_4;
_12.2 = [1473229352_i32,1858616060_i32,1661192724_i32,(-229433819_i32),(-2029000577_i32),238124211_i32,1455848919_i32];
_15.fld0 = [123_i8,1_i8,87_i8];
_15.fld1 = [9563666233425679126_u64,1210134128369887214_u64,16022974370007340099_u64,5464936061263102197_u64,1690208812643981847_u64];
_15.fld0 = [125_i8,120_i8,54_i8];
_3 = _2;
_6 = [148628749_u32,2065498345_u32,86319456_u32,2882932391_u32,1700781979_u32,2876514769_u32,2493367971_u32];
_14 = core::ptr::addr_of_mut!(_13);
_12.2 = [(-143895324_i32),6347151_i32,2074167480_i32,(-1587633105_i32),541986183_i32,(-180960329_i32),(-94138389_i32)];
RET = 4178_i16 | (-18028_i16);
RET = 26_i8 as i16;
_10 = [228193186231486647_i64];
_7 = !46628366311359718105687658396008408703_u128;
_3 = _7 as isize;
_1 = _11;
Goto(bb6)
}
bb6 = {
RET = 4635_u16 as i16;
_7 = 195290578946686616721985406241945519566_u128 | 311759279640318134848692681307651522138_u128;
_17 = '\u{5376a}';
RET = (-4672_i16);
_12.2 = [1809485614_i32,1675535472_i32,(-1286050855_i32),891131959_i32,(-1968621245_i32),(-1987901820_i32),106062357_i32];
_15.fld0 = [49_i8,6_i8,83_i8];
_15.fld1 = [1631172572589520542_u64,14129592735688279989_u64,9851070002437216052_u64,7425685434825863400_u64,11682100595083926794_u64];
_3 = _4 * _11;
_16 = 8334808377475880803_i64 << _11;
_2 = _1;
RET = 22791_i16;
_1 = -_3;
RET = 26642_i16;
_6 = [3426487835_u32,2273049299_u32,3225305085_u32,2987561189_u32,285655401_u32,3481274312_u32,3651865991_u32];
Goto(bb7)
}
bb7 = {
_15.fld1 = [5494082931999182270_u64,15518477538395696076_u64,8744083810741004030_u64,14194297095391085182_u64,12468396740331308550_u64];
_11 = -_3;
_20 = _3 <= _11;
_19.0.fld0 = _15.fld0;
_19.0 = Adt38 { fld0: _15.fld0,fld1: _15.fld1 };
_8 = &_9;
_19.0.fld0 = [25_i8,12_i8,95_i8];
_15.fld1 = [12759580668353585334_u64,928125271497662078_u64,14219891014283868393_u64,8717171771922546656_u64,1077776966903529938_u64];
_15.fld0 = _19.0.fld0;
_16 = 9157594822668117538_i64;
RET = -7405_i16;
_4 = !_11;
_19.0.fld1 = _15.fld1;
_9 = (-128_i8) as f32;
_25 = !(-546358654_i32);
_15.fld0 = [(-75_i8),(-39_i8),0_i8];
Call(_15.fld1 = core::intrinsics::transmute(_19.0.fld1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6 = [1531822310_u32,2671056589_u32,906579549_u32,434582213_u32,2681665414_u32,3982269606_u32,3164134967_u32];
_15.fld0 = [(-47_i8),45_i8,(-92_i8)];
_24 = 1655707638_u32;
_20 = false & false;
_1 = _2;
_6 = [_24,_24,_24,_24,_24,_24,_24];
RET = 18931_i16;
_3 = _4 + _4;
_20 = _3 <= _3;
_10 = [_16];
_23 = [_16,_16,_16,_16,_16,_16,_16,_16];
_21 = -_9;
_1 = _2 ^ _4;
_3 = !_2;
match RET {
0 => bb5,
1 => bb6,
2 => bb9,
3 => bb10,
18931 => bb12,
_ => bb11
}
}
bb9 = {
_6 = [4122509876_u32,2439843135_u32,842730762_u32,3273690914_u32,2480670961_u32,75341430_u32,2068434284_u32];
_6 = [1002647099_u32,2544482377_u32,1928741887_u32,1438257206_u32,4173154921_u32,1342552308_u32,2896045788_u32];
_2 = _7 as isize;
_2 = (-25_i8) as isize;
_4 = _3;
_6 = [1172341269_u32,4179311723_u32,3369868096_u32,2192360071_u32,547068266_u32,124485193_u32,2079698452_u32];
_3 = _1 * _4;
_3 = 981798283754351168_i64 as isize;
_7 = '\u{7792}' as u128;
_2 = 13189_u16 as isize;
RET = (-30271_i16) ^ 5446_i16;
_7 = false as u128;
RET = (-11491_i16) * 19634_i16;
RET = 25555_i16;
_8 = &_9;
_9 = (-147374261540454389784492408298974263927_i128) as f32;
_4 = _3;
_9 = 0_usize as f32;
_4 = 15860671811273638771_u64 as isize;
_9 = (-1144792675_i32) as f32;
_10 = [5196512539373443965_i64];
_6 = [178024321_u32,3377164855_u32,655042486_u32,39246493_u32,3064139838_u32,1425958141_u32,1957960541_u32];
_4 = !_1;
_1 = _7 as isize;
RET = _1 as i16;
RET = (-4363_i16);
_1 = _3 & _3;
_9 = 116_u8 as f32;
match RET {
0 => bb1,
1 => bb3,
340282366920938463463374607431768207093 => bb5,
_ => bb4
}
}
bb10 = {
RET = 4635_u16 as i16;
_7 = 195290578946686616721985406241945519566_u128 | 311759279640318134848692681307651522138_u128;
_17 = '\u{5376a}';
RET = (-4672_i16);
_12.2 = [1809485614_i32,1675535472_i32,(-1286050855_i32),891131959_i32,(-1968621245_i32),(-1987901820_i32),106062357_i32];
_15.fld0 = [49_i8,6_i8,83_i8];
_15.fld1 = [1631172572589520542_u64,14129592735688279989_u64,9851070002437216052_u64,7425685434825863400_u64,11682100595083926794_u64];
_3 = _4 * _11;
_16 = 8334808377475880803_i64 << _11;
_2 = _1;
RET = 22791_i16;
_1 = -_3;
RET = 26642_i16;
_6 = [3426487835_u32,2273049299_u32,3225305085_u32,2987561189_u32,285655401_u32,3481274312_u32,3651865991_u32];
Goto(bb7)
}
bb11 = {
_3 = -_2;
_11 = _4 >> _4;
RET = (-28192_i16);
_10 = [(-2449853064349023805_i64)];
_9 = 2728307508_u32 as f32;
_2 = -_4;
_12.2 = [1473229352_i32,1858616060_i32,1661192724_i32,(-229433819_i32),(-2029000577_i32),238124211_i32,1455848919_i32];
_15.fld0 = [123_i8,1_i8,87_i8];
_15.fld1 = [9563666233425679126_u64,1210134128369887214_u64,16022974370007340099_u64,5464936061263102197_u64,1690208812643981847_u64];
_15.fld0 = [125_i8,120_i8,54_i8];
_3 = _2;
_6 = [148628749_u32,2065498345_u32,86319456_u32,2882932391_u32,1700781979_u32,2876514769_u32,2493367971_u32];
_14 = core::ptr::addr_of_mut!(_13);
_12.2 = [(-143895324_i32),6347151_i32,2074167480_i32,(-1587633105_i32),541986183_i32,(-180960329_i32),(-94138389_i32)];
RET = 4178_i16 | (-18028_i16);
RET = 26_i8 as i16;
_10 = [228193186231486647_i64];
_7 = !46628366311359718105687658396008408703_u128;
_3 = _7 as isize;
_1 = _11;
Goto(bb6)
}
bb12 = {
_9 = _21 - _21;
RET = _24 as i16;
_3 = !_1;
_24 = _25 as u32;
_27 = core::ptr::addr_of_mut!(_25);
Goto(bb13)
}
bb13 = {
RET = !(-21940_i16);
_21 = -_9;
_14 = core::ptr::addr_of_mut!(_5);
_26.2 = _12.2;
_1 = _2;
_19.0 = Adt38 { fld0: _15.fld0,fld1: _15.fld1 };
_16 = !5490588399259181388_i64;
_12.2 = [(*_27),(*_27),(*_27),(*_27),_25,(*_27),(*_27)];
_26.2 = [(*_27),_25,(*_27),(*_27),(*_27),_25,_25];
Goto(bb14)
}
bb14 = {
(*_27) = _3 as i32;
_7 = 107643914036667539103020368225110208325_u128;
_15.fld0 = [79_i8,(-53_i8),(-28_i8)];
_10 = [_16];
_11 = _3 ^ _3;
_6 = [_24,_24,_24,_24,_24,_24,_24];
_15.fld0 = _19.0.fld0;
_26.2 = _12.2;
(*_27) = RET as i32;
RET = _24 as i16;
_16 = (-6342569940898353821_i64);
_19.0.fld0 = _15.fld0;
_17 = '\u{104c8e}';
_8 = &_9;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(15_usize, 2_usize, Move(_2), 1_usize, Move(_1), 23_usize, Move(_23), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(15_usize, 10_usize, Move(_10), 6_usize, Move(_6), 16_usize, Move(_16), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: i16,mut _10: isize) -> isize {
mir! {
type RET = isize;
let _11: i128;
let _12: isize;
let _13: [u64; 5];
let _14: ();
let _15: ();
{
RET = 70_u8 as isize;
Goto(bb1)
}
bb1 = {
_8 = 17739660670923360956_usize as isize;
Goto(bb2)
}
bb2 = {
RET = _1 * _7;
_5 = _1 >> _10;
_11 = 26756092137347733892339159884897608084_i128;
_12 = _3;
_10 = _4;
RET = _7 * _3;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(16_usize, 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_14 = dump_var(16_usize, 1_usize, Move(_1), 10_usize, Move(_10), 15_usize, _15, 15_usize, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: [u64; 5],mut _2: [u64; 4]) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _3: (u8,);
let _4: bool;
let _5: *const Adt31;
let _6: i32;
let _7: f32;
let _8: [usize; 7];
let _9: &'static &'static (Adt31,);
let _10: char;
let _11: u8;
let _12: bool;
let _13: Adt35;
let _14: &'static isize;
let _15: usize;
let _16: u32;
let _17: [i32; 7];
let _18: &'static &'static (Adt31,);
let _19: u32;
let _20: i128;
let _21: i64;
let _22: Adt44;
let _23: [u16; 4];
let _24: bool;
let _25: Adt42;
let _26: isize;
let _27: char;
let _28: u8;
let _29: &'static u16;
let _30: &'static *const (&'static *const f64,);
let _31: isize;
let _32: (&'static u8,);
let _33: ();
let _34: ();
{
_1 = [2987291839892285332_u64,13484634115799403004_u64,15617829839557438671_u64,9838311831875161544_u64,16860101494337118952_u64];
_1 = [2948874312136839716_u64,4578938723099076049_u64,4378912892165943758_u64,10942071926000327505_u64,13618164833705862758_u64];
_2 = [4370452014142107319_u64,10955593916578931614_u64,3081477295841048548_u64,15877481989788996882_u64];
_3 = (87_u8,);
_2 = [14908209361185662639_u64,15143830453812581741_u64,13672183276680502874_u64,6180004998361434203_u64];
RET = [6401717767039414603_u64,10524568745207555049_u64,17552253620368754570_u64,4678101151869834137_u64,11160091665593499090_u64];
_1 = [14545650624670638916_u64,2919622666776898192_u64,7291367593356432847_u64,4694736544573492155_u64,13108999384153265832_u64];
RET = [2512929486508952685_u64,9614900165679584238_u64,15137904295097464589_u64,16216263297323455343_u64,1239424226536556589_u64];
RET = [14326985396119021560_u64,5566963287425358751_u64,9874521283563538086_u64,5552913532422027804_u64,17157277680847600345_u64];
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
87 => bb9,
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
_3 = (19_u8,);
_3.0 = 140_u8 & 231_u8;
_1 = [5674577857948073189_u64,17824146752468951278_u64,18317790847399987184_u64,5858789480363641626_u64,17092365583331191820_u64];
_6 = 1681486370_i32 + 706417772_i32;
_4 = !true;
RET = [9486429685977296761_u64,17539212016547532875_u64,8970256381599386136_u64,5347564412600924999_u64,12273253104882635680_u64];
_4 = !false;
RET = [9760770663887829141_u64,8182548231968501037_u64,16773919527301282496_u64,8418452321664213697_u64,10965290030670198343_u64];
_7 = 160542239101171369418816951638462010941_u128 as f32;
_2 = [667672468574013752_u64,8864110264100748483_u64,6697063975668820822_u64,16723666252041453073_u64];
_8 = [3_usize,706378447930622354_usize,1_usize,11166352094388577935_usize,10383133812671350011_usize,6357998494968477969_usize,2_usize];
_7 = 1683431856647837240_u64 as f32;
_3.0 = 375510957_u32 as u8;
_4 = true;
_1 = [13085377873936356520_u64,20693620465929109_u64,17921473837869241750_u64,5987989812790849430_u64,14702454387974461485_u64];
_2 = [4915727901808037991_u64,5519222515317172095_u64,17389954212597094398_u64,15949722107333935865_u64];
Goto(bb10)
}
bb10 = {
_12 = !_4;
RET = _1;
_4 = !_12;
_1 = [8747135074022635457_u64,839979690895911797_u64,4050642627244527213_u64,9009973457053206118_u64,16549571597320337422_u64];
_13 = Adt35::Variant1 { fld0: 1574169672947134428938362621635812081_i128 };
place!(Field::<i128>(Variant(_13, 1), 0)) = 94539467114856539310929167107853088838_i128 + (-81818895787554158221148993980031461882_i128);
_1 = [9187501312837617560_u64,15527964524925682275_u64,13670720831761341404_u64,18405463180126933130_u64,14207480801623664727_u64];
_10 = '\u{5c717}';
_11 = _3.0 << Field::<i128>(Variant(_13, 1), 0);
_12 = _11 >= _3.0;
_3.0 = _11;
_2 = [6103892546869118996_u64,912535536296874450_u64,13514378064935016646_u64,1610649752573896859_u64];
_15 = !0_usize;
_8 = [_15,_15,_15,_15,_15,_15,_15];
_12 = _4;
_11 = _3.0 >> _3.0;
place!(Field::<i128>(Variant(_13, 1), 0)) = !41537149475536405622140507101194744898_i128;
_3.0 = _11;
_12 = _4 ^ _4;
_3.0 = _11 | _11;
Goto(bb11)
}
bb11 = {
_10 = '\u{979e4}';
_7 = _15 as f32;
place!(Field::<i128>(Variant(_13, 1), 0)) = _11 as i128;
SetDiscriminant(_13, 3);
place!(Field::<i8>(Variant(_13, 3), 3)) = -(-29_i8);
_19 = 1446011219_u32 - 3830076089_u32;
place!(Field::<i16>(Variant(_13, 3), 4)) = (-18799_i16);
place!(Field::<*const f64>(Variant(_13, 3), 2)) = core::ptr::addr_of!(place!(Field::<f64>(Variant(_13, 3), 5)));
place!(Field::<f64>(Variant(_13, 3), 5)) = (-2175439276423155644_i64) as f64;
_20 = (-140754741859054048029756637986661179247_i128) * 60859352873328500795704932529374992447_i128;
_13 = Adt35::Variant1 { fld0: _20 };
_11 = _3.0;
_8 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb12)
}
bb12 = {
_3 = (_11,);
_11 = _3.0;
_1 = RET;
RET = [5433720536668419227_u64,13751351124843141533_u64,621618390744563625_u64,12464870021872641160_u64,8276719239747565741_u64];
_23 = [21583_u16,37561_u16,7748_u16,63915_u16];
SetDiscriminant(_13, 2);
_3 = (_11,);
_17 = [_6,_6,_6,_6,_6,_6,_6];
_21 = 1022682682295630664_i64 | (-3996942164487312042_i64);
Call(place!(Field::<[u32; 2]>(Variant(_13, 2), 5)) = core::intrinsics::transmute(_15), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16 = _19 ^ _19;
_15 = 75995289877111522517211687085892039062_u128 as usize;
_23 = [59536_u16,37474_u16,13714_u16,44720_u16];
_14 = &_26;
_23 = [55980_u16,55219_u16,7876_u16,47180_u16];
place!(Field::<bool>(Variant(_13, 2), 0)) = _12;
_15 = _20 as usize;
_3.0 = _11 - _11;
_2 = [16080303409026167035_u64,1974321437960505430_u64,659470640363365234_u64,5446568563733626445_u64];
_16 = _19 >> _21;
_1 = [61763237709850117_u64,4200945795310917673_u64,17210536601524523266_u64,5274839960165123417_u64,1804430960977077612_u64];
place!(Field::<i16>(Variant(_13, 2), 4)) = 243270965574573342779520182461656327850_u128 as i16;
_26 = Field::<i16>(Variant(_13, 2), 4) as isize;
_8 = [_15,_15,_15,_15,_15,_15,_15];
_23 = [36213_u16,14970_u16,51257_u16,15197_u16];
_11 = !_3.0;
_20 = !133740904310149167406810119246284917086_i128;
_11 = _3.0;
RET = [9643633861311947205_u64,3170826481880627488_u64,9004755786699619362_u64,17523811507939658762_u64,5239426146837126255_u64];
_3 = (_11,);
_14 = &_26;
_4 = _12 ^ _12;
_23 = [28786_u16,22290_u16,1760_u16,27592_u16];
_24 = _3.0 <= _11;
_12 = !_24;
RET = _1;
_15 = Field::<i16>(Variant(_13, 2), 4) as usize;
_19 = _16;
Goto(bb14)
}
bb14 = {
place!(Field::<u128>(Variant(_13, 2), 1)) = !312958555927576875088081981895474995855_u128;
_6 = Field::<i16>(Variant(_13, 2), 4) as i32;
_3 = (_11,);
_26 = 9223372036854775807_isize;
_21 = 4190_u16 as i64;
_16 = _15 as u32;
_21 = -3858907249711626274_i64;
_2 = [10960078354118439477_u64,7242294968729020379_u64,3081504268549841389_u64,11506021670633030485_u64];
_1 = [17050512766947077927_u64,446891968822589394_u64,3441221576590343038_u64,12823326684610302090_u64,15766842438299263614_u64];
_20 = (-112925817383457338629860664791318716644_i128);
_27 = _10;
_21 = 3120785952845959797_i64;
_20 = 69229181833407820241413600040308121305_i128 & (-85479497047600607700615320166441214584_i128);
_28 = _11;
_2 = [13644239501583442080_u64,13123920219425096695_u64,3158756131692868554_u64,3249135323473061315_u64];
_17 = [_6,_6,_6,_6,_6,_6,_6];
RET = [16842241016520888594_u64,3031027689864393319_u64,2448044533764175744_u64,6322686073976172926_u64,15551074896117182018_u64];
place!(Field::<bool>(Variant(_13, 2), 0)) = !_24;
_16 = _19 << _11;
_23 = [34709_u16,26822_u16,42395_u16,36526_u16];
_22 = Adt44::Variant2 { fld0: _1 };
_3.0 = _28 ^ _11;
_12 = !Field::<bool>(Variant(_13, 2), 0);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(17_usize, 10_usize, Move(_10), 8_usize, Move(_8), 15_usize, Move(_15), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(17_usize, 12_usize, Move(_12), 21_usize, Move(_21), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(17_usize, 1_usize, Move(_1), 17_usize, Move(_17), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [i64; 8],mut _2: bool,mut _3: isize,mut _4: [u64; 5],mut _5: [i8; 3],mut _6: [u64; 6],mut _7: [i64; 8],mut _8: [u64; 4],mut _9: [i64; 8],mut _10: [i64; 8]) -> [u64; 4] {
mir! {
type RET = [u64; 4];
let _11: (&'static *const f64,);
let _12: &'static *const (&'static *const f64,);
let _13: i128;
let _14: f64;
let _15: isize;
let _16: [u64; 6];
let _17: i128;
let _18: i16;
let _19: (*mut usize,);
let _20: i64;
let _21: (Adt38, *mut [u32; 4]);
let _22: Adt57;
let _23: usize;
let _24: (&'static u8,);
let _25: [u16; 8];
let _26: ([u32; 4],);
let _27: [u64; 6];
let _28: &'static (Adt31,);
let _29: (([u32; 4],), bool, (&'static *const f64,));
let _30: [i64; 8];
let _31: isize;
let _32: [u32; 3];
let _33: isize;
let _34: ();
let _35: ();
{
RET = [693029064452716805_u64,2343061398499993914_u64,6306196097624137636_u64,4228947236296593504_u64];
_7 = _10;
_8 = RET;
RET = [11724022715028505729_u64,16509389034756203079_u64,6747301311887689341_u64,1267551946581011337_u64];
_8 = [3848323370108720144_u64,324357874003860291_u64,16888634245790551833_u64,16898275393706834132_u64];
_13 = 81367984724694406851051688906098813864_i128;
_10 = [(-3786795399823596754_i64),131486729767140149_i64,(-5602750260546069246_i64),3374271940304511040_i64,3704578557853490307_i64,(-7667920482014685216_i64),(-3252053016705033692_i64),(-141305416162714796_i64)];
_2 = false | true;
_14 = 63003_u16 as f64;
_3 = !(-38_isize);
_7 = _1;
_10 = _7;
RET = [2548107264995778530_u64,1304212563514259340_u64,5596264597282484915_u64,3114020645247306033_u64];
_5 = [68_i8,75_i8,(-120_i8)];
_6 = [15472970457837631159_u64,301922924205244123_u64,17680499758046105439_u64,2664093831697822113_u64,11564589794469480699_u64,13702254230385460681_u64];
_9 = [(-1870563335789438294_i64),(-6911598405410278181_i64),8547585014264606053_i64,(-3949511405964849213_i64),(-5048015521700965353_i64),6583684478671535005_i64,4736952508645065171_i64,5344747046185224983_i64];
_3 = 21_isize | (-9223372036854775808_isize);
_10 = [(-7281209732804454573_i64),410137505523487907_i64,4824267090160850341_i64,4945361755836200935_i64,(-6276821432865749011_i64),(-5870539407942944520_i64),(-1363926348512204520_i64),(-7311733279000633526_i64)];
_1 = [(-725045216495335431_i64),(-5728613905698775886_i64),(-3057511759284144031_i64),5404896719878987301_i64,8172094967847318376_i64,1601061215939927297_i64,(-8926111605583978833_i64),(-2413803419057964078_i64)];
_6 = [2209747282456589270_u64,7759152596890787956_u64,5337453598148047957_u64,17858231613468578589_u64,227256873509907788_u64,13218345279739423635_u64];
_8 = [9708649608894746546_u64,16693199683862680474_u64,4266483084649762190_u64,17104700926677045136_u64];
_6 = [12172866806292520689_u64,12123709695057967095_u64,4386621214363869682_u64,7427245738626098068_u64,2436791640795799892_u64,17852074655218564511_u64];
_6 = [15515611519684613700_u64,5503073095811458779_u64,4533030655325206177_u64,2666232994394653543_u64,17880006449012039914_u64,17140446033415641169_u64];
_3 = 114_isize - 9223372036854775807_isize;
_10 = [(-984997936476497537_i64),(-3694064881031349701_i64),8757974058574188929_i64,6044897139654995995_i64,(-4806694616372463469_i64),(-8895287358548884616_i64),(-6538955569335710306_i64),(-4039611730570745696_i64)];
_7 = _10;
_4 = [8411414283451487405_u64,3682800095320973204_u64,17650513622334562369_u64,11144886929280481853_u64,9510405066209775922_u64];
match _13 {
0 => bb1,
81367984724694406851051688906098813864 => bb3,
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
_1 = _7;
_6 = [11461128340138430402_u64,8231591698863798963_u64,12995713704188249775_u64,8740341501517343281_u64,8325506050111165722_u64,17965052297568427969_u64];
_14 = 236_u8 as f64;
RET = [3165248559250844312_u64,11525768227916674107_u64,247184718734601527_u64,4396597734149405436_u64];
_14 = 293964668416083316953295479008123622775_u128 as f64;
_3 = -89_isize;
RET = [7095295932407267768_u64,7354630288837462168_u64,17283367158884892029_u64,3330659063581930672_u64];
_15 = _3;
_2 = false & true;
_6 = [8162646326240619292_u64,3949500177205280441_u64,15973783837997505262_u64,7114097606563321293_u64,12359803456143501230_u64,18167452191064886520_u64];
_2 = !true;
Goto(bb4)
}
bb4 = {
_15 = 14265688850200031181_u64 as isize;
_9 = [2906941607261108783_i64,2820298823431915470_i64,(-5374351856390762887_i64),(-6909304188572940645_i64),(-4342731826835782363_i64),(-3687301573262500107_i64),(-7523228941287553541_i64),8317731458649463591_i64];
match _13 {
0 => bb5,
1 => bb6,
2 => bb7,
81367984724694406851051688906098813864 => bb9,
_ => bb8
}
}
bb5 = {
_1 = _7;
_6 = [11461128340138430402_u64,8231591698863798963_u64,12995713704188249775_u64,8740341501517343281_u64,8325506050111165722_u64,17965052297568427969_u64];
_14 = 236_u8 as f64;
RET = [3165248559250844312_u64,11525768227916674107_u64,247184718734601527_u64,4396597734149405436_u64];
_14 = 293964668416083316953295479008123622775_u128 as f64;
_3 = -89_isize;
RET = [7095295932407267768_u64,7354630288837462168_u64,17283367158884892029_u64,3330659063581930672_u64];
_15 = _3;
_2 = false & true;
_6 = [8162646326240619292_u64,3949500177205280441_u64,15973783837997505262_u64,7114097606563321293_u64,12359803456143501230_u64,18167452191064886520_u64];
_2 = !true;
Goto(bb4)
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
_6 = [1649879526516884092_u64,9655488565648885883_u64,8783878331785285627_u64,5566332396853327066_u64,5593438488171999447_u64,16642422588109900726_u64];
RET = [13470154660603492445_u64,3726068329737563492_u64,18212722965719578862_u64,13164201978461931218_u64];
_13 = 47300101601071372878730817033953782036_i128;
_8 = RET;
_3 = (-7214231693450082210_i64) as isize;
_8 = [3426493864003295794_u64,17597208381181303963_u64,13438187796349611995_u64,10190696820355356847_u64];
_14 = 1755567068_u32 as f64;
_15 = -_3;
RET = _8;
_3 = _13 as isize;
_3 = !_15;
RET = [9149898266599168121_u64,14221660991949414450_u64,15105041138038142494_u64,16056176222393050017_u64];
_5 = [(-41_i8),(-2_i8),(-12_i8)];
_9 = _7;
_9 = [(-3726843359708623118_i64),(-8558157799539346494_i64),(-3250044599059868598_i64),3050484109614656800_i64,5810535619718147221_i64,8167954082987295527_i64,(-4585157249318183177_i64),8168402302615963813_i64];
_6 = [249411816058353863_u64,18232836141899341321_u64,4003845902037256462_u64,880858205282179784_u64,7173604288674132934_u64,12038446060065200300_u64];
_9 = _10;
_6 = [8210436528510829309_u64,472082378653122501_u64,18370040071033179910_u64,10352278127415653765_u64,12081092127275570614_u64,5227413315610528332_u64];
_17 = !_13;
_2 = !false;
_16 = [1179257161533698846_u64,4379982918464924553_u64,16624770934422900992_u64,8577342667773176290_u64,6559773268759229306_u64,2672393872726306559_u64];
_13 = 186_u8 as i128;
_4 = [4157199522158042096_u64,10302682563650266140_u64,9936126607798246740_u64,3819252160595350155_u64,16661608718768230503_u64];
RET = _8;
_15 = _3;
_16 = [5699540320419031889_u64,16912163378580456152_u64,13221216203787305777_u64,9035139024129075781_u64,6382205784647974776_u64,16987577725593231467_u64];
_8 = RET;
Call(_9 = core::intrinsics::transmute(_1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = _6;
_8 = [1971913690028267823_u64,4645955954057315199_u64,13580719261693450964_u64,12645622164939360881_u64];
_1 = [(-3127880407252613950_i64),(-5150256375958691605_i64),(-327901661620446593_i64),3362510462760423204_i64,5681886815374506098_i64,6784935700954707148_i64,(-2847133312600337639_i64),1202853101091865573_i64];
_15 = 6079153973658492842_i64 as isize;
_4 = [6170595193449671801_u64,10069337351013745155_u64,2563678240368711512_u64,2066082231530998843_u64,5746154194877997750_u64];
RET = [16230189958411520336_u64,13582313860104255742_u64,1660156040212786134_u64,1217413200877773082_u64];
_14 = _3 as f64;
_3 = _15 * _15;
_18 = 5055_i16;
_9 = [3189905703598465240_i64,(-4721378990155444187_i64),(-548942320325952439_i64),890577781470164103_i64,1168034980731813715_i64,6446176215702674494_i64,1940539141963081093_i64,(-8595482193413234506_i64)];
_13 = _17 & _17;
RET = [14590669679343301911_u64,15986303430238628884_u64,11819909962144093665_u64,14080799838641598717_u64];
Goto(bb11)
}
bb11 = {
_8 = RET;
_8 = [7286869299250408078_u64,7070085346390170908_u64,15674078919913197496_u64,9891094803854271496_u64];
Goto(bb12)
}
bb12 = {
RET = [6509689932378301659_u64,8812104561946099599_u64,10079155886784553188_u64,968517915760927111_u64];
_15 = _3 ^ _3;
_21.0.fld0 = _5;
_21.0 = Adt38 { fld0: _5,fld1: _4 };
_1 = [(-4730893616048756733_i64),(-7568681478356172843_i64),(-3491134841843867406_i64),5026758765816795423_i64,(-4370097597863984014_i64),9049016362985288577_i64,4902651981072911326_i64,6419676061586331006_i64];
RET = [2741615203681019223_u64,11362608603575102985_u64,7971046819672694179_u64,8273192928226974283_u64];
_18 = !(-5096_i16);
_2 = !true;
_3 = _15 & _15;
_23 = 7532780996489644505_usize;
_21.0.fld1 = [6606021803811575460_u64,2671733434036853578_u64,9358499770812786191_u64,5793989924022126791_u64,16171233745550369095_u64];
_1 = [(-3972964061156376480_i64),3290769920465986545_i64,2938057084864770751_i64,8160823154832785546_i64,(-1033047482691082748_i64),(-7272579966823274404_i64),(-5392405339026620320_i64),(-3315325674962271581_i64)];
_1 = [(-8327610592066242481_i64),7004953508805449614_i64,7309244392194287670_i64,(-9162912305292840166_i64),2016497829035343168_i64,(-5797646447862421841_i64),8556893562529327391_i64,(-3621504803532231145_i64)];
_17 = _13;
_18 = _14 as i16;
_8 = [6706024996400888548_u64,7065573956048104067_u64,9568317010055420105_u64,10332383509642864642_u64];
_21.0 = Adt38 { fld0: _5,fld1: _4 };
_23 = !9433660059182335706_usize;
_21.0.fld1 = [15276791353186435646_u64,13171673841760837814_u64,11263964890243129557_u64,14775921455310114622_u64,2542669250948650400_u64];
_17 = 110754967841522223354719596525516461752_u128 as i128;
_14 = 744511558115858361_i64 as f64;
_21.0 = Adt38 { fld0: _5,fld1: _4 };
_7 = [(-680956283544549373_i64),3149325123985718967_i64,1522620672478791001_i64,(-3522413000717551203_i64),(-6283583057019241641_i64),(-6195935523174220916_i64),1836432124293156629_i64,2533605098989102787_i64];
_10 = [(-5740022121215497500_i64),5292862729977861870_i64,(-449609956179461298_i64),(-1162443202817505406_i64),168604545817787580_i64,2144494273357927413_i64,8007223928650128194_i64,(-3333335781782444744_i64)];
_5 = _21.0.fld0;
_21.0 = Adt38 { fld0: _5,fld1: _4 };
Goto(bb13)
}
bb13 = {
_21.0 = Adt38 { fld0: _5,fld1: _4 };
_4 = [1161710634379788345_u64,16200729284819621611_u64,7062957928751576873_u64,11739177036313574695_u64,5046767446286639207_u64];
RET = _8;
_5 = [(-52_i8),121_i8,(-26_i8)];
_14 = _18 as f64;
_3 = _15 ^ _15;
_18 = -(-11705_i16);
_21.0.fld0 = [76_i8,(-27_i8),107_i8];
_5 = [72_i8,(-85_i8),42_i8];
_21.0.fld1 = _4;
_21.0.fld0 = [(-19_i8),(-76_i8),(-62_i8)];
_18 = _15 as i16;
_21.1 = core::ptr::addr_of_mut!(_26.0);
_21.0.fld1 = [3331708795837045905_u64,12732409989468480982_u64,8120485386415002532_u64,3237076254626017901_u64,2186240526972679072_u64];
_21.1 = core::ptr::addr_of_mut!(_26.0);
_20 = 8383364467869824450_i64;
_29.1 = _2;
_15 = _3;
_9 = [_20,_20,_20,_20,_20,_20,_20,_20];
_3 = !_15;
Goto(bb14)
}
bb14 = {
_21.1 = core::ptr::addr_of_mut!(_26.0);
_29.1 = !_2;
_26.0 = [1240514283_u32,1774062394_u32,1998694313_u32,955179863_u32];
_10 = _7;
_17 = -_13;
_9 = _1;
_27 = [10116979685289823206_u64,7679830803611452034_u64,13988073803031887825_u64,6844721972342958975_u64,10208363357730509492_u64,2402262423975405856_u64];
_13 = _17 >> _17;
_22.fld0 = core::ptr::addr_of_mut!(_29.1);
_21.0.fld1 = _4;
_31 = _15 * _15;
_29.0.0 = _26.0;
_23 = 135_u8 as usize;
_10 = _7;
_25 = [38921_u16,3612_u16,22366_u16,43454_u16,39457_u16,13936_u16,4787_u16,21726_u16];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(18_usize, 23_usize, Move(_23), 8_usize, Move(_8), 13_usize, Move(_13), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(18_usize, 7_usize, Move(_7), 27_usize, Move(_27), 26_usize, Move(_26), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(18_usize, 5_usize, Move(_5), 6_usize, Move(_6), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [u64; 4]) -> isize {
mir! {
type RET = isize;
let _2: (*mut usize,);
let _3: isize;
let _4: *mut &'static u8;
let _5: i64;
let _6: Adt38;
let _7: [u128; 7];
let _8: f32;
let _9: usize;
let _10: f64;
let _11: [u64; 4];
let _12: (&'static *const f64,);
let _13: char;
let _14: usize;
let _15: u16;
let _16: f32;
let _17: i128;
let _18: f32;
let _19: [u64; 6];
let _20: i128;
let _21: ();
let _22: ();
{
RET = 53_isize & 110_isize;
RET = 16310203116383352622_u64 as isize;
_3 = -RET;
_3 = RET >> RET;
RET = _3 & _3;
_3 = 5480519158427687811_u64 as isize;
RET = _3 << _3;
RET = !_3;
_1 = [1184517147246806246_u64,6327477361748526040_u64,35979704486752906_u64,12658616980244862408_u64];
_1 = [15398018764116723077_u64,5736884677249564526_u64,995707757855127205_u64,1962584776429689403_u64];
RET = _3;
_3 = RET;
Goto(bb1)
}
bb1 = {
_1 = [7307505781990185295_u64,16847378530057666236_u64,6243051078479057610_u64,4802413618033145496_u64];
_3 = RET - RET;
RET = (-8598521469192988545_i64) as isize;
_3 = false as isize;
RET = _3 << _3;
Goto(bb2)
}
bb2 = {
_1 = [16211090992792328373_u64,9828047269900725565_u64,17183854364118213580_u64,15654058485492210914_u64];
_3 = RET;
_5 = 5799147506432274541_i64 & 4206293816035955540_i64;
_5 = -(-1991968902001929494_i64);
_5 = (-3283281438667466149_i64) - (-6311988668285650949_i64);
RET = _3;
_1 = [8704292669431704457_u64,16601862243185495514_u64,2106100989156505477_u64,14107238046987681828_u64];
_3 = RET;
Goto(bb3)
}
bb3 = {
_1 = [15445476694967437548_u64,2519341969831813305_u64,3407487745524987368_u64,5442356666685943080_u64];
_5 = 1310409969758502489_i64;
_1 = [13063083208270308273_u64,12450581195396899797_u64,3420911749947412587_u64,12675719457050813660_u64];
_3 = RET - RET;
RET = !_3;
_3 = RET & RET;
RET = (-91_i8) as isize;
_6.fld1 = [12439124796344670692_u64,13191088096777144501_u64,2979110613318087617_u64,1176366744875645943_u64,4355423895566020036_u64];
_7 = [230106601534635279915802060314576432464_u128,18079842089835432017992283836836653869_u128,50110098488320437521461182252344467392_u128,252716648263502682255869487588352095271_u128,297945708290199074180033711849092141049_u128,254132586253784054861093450958839851640_u128,266773819413522691930907005206169309242_u128];
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1310409969758502489 => bb10,
_ => bb9
}
}
bb4 = {
_1 = [16211090992792328373_u64,9828047269900725565_u64,17183854364118213580_u64,15654058485492210914_u64];
_3 = RET;
_5 = 5799147506432274541_i64 & 4206293816035955540_i64;
_5 = -(-1991968902001929494_i64);
_5 = (-3283281438667466149_i64) - (-6311988668285650949_i64);
RET = _3;
_1 = [8704292669431704457_u64,16601862243185495514_u64,2106100989156505477_u64,14107238046987681828_u64];
_3 = RET;
Goto(bb3)
}
bb5 = {
_1 = [7307505781990185295_u64,16847378530057666236_u64,6243051078479057610_u64,4802413618033145496_u64];
_3 = RET - RET;
RET = (-8598521469192988545_i64) as isize;
_3 = false as isize;
RET = _3 << _3;
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
_8 = 63_u8 as f32;
RET = (-23717_i16) as isize;
_6.fld1 = [10999015257750137986_u64,13302526485375203092_u64,5248434433364690603_u64,4286926704048688610_u64,6461891486088394830_u64];
RET = -_3;
_8 = 81_i8 as f32;
_5 = (-3143_i16) as i64;
_3 = -RET;
_6.fld0 = [(-20_i8),(-78_i8),62_i8];
_6.fld0 = [92_i8,(-21_i8),94_i8];
_1 = [2202160906354515579_u64,13779468744713562525_u64,6986572242735432430_u64,17860541101689559361_u64];
_9 = 1_usize >> RET;
_2.0 = core::ptr::addr_of_mut!(_9);
_1 = [1742988881751879103_u64,17893103574319494388_u64,3726683174357559102_u64,13376920662015497495_u64];
_5 = (-7117635409866427712_i64);
_9 = !5_usize;
_2.0 = core::ptr::addr_of_mut!(_9);
_6.fld0 = [(-27_i8),24_i8,(-108_i8)];
_5 = _8 as i64;
_5 = 6817706783875896131_i64 & 6522021394295550411_i64;
_10 = (-94_i8) as f64;
_5 = _9 as i64;
_7 = [180374887616030713139680617107459709630_u128,99214814310551779575846839822536167947_u128,145363356979261608713850679393071480415_u128,195937413712819284535586933825292155485_u128,135555025914816004696182372679296615467_u128,84798331287211739328469929974646524925_u128,234197025125677460094963209983375774459_u128];
_6.fld1 = [7476350499413050602_u64,13116485882506648686_u64,11104908469271231701_u64,15278842912715475921_u64,12881020988211266219_u64];
RET = (-118056222815925680276662784127928645769_i128) as isize;
Goto(bb11)
}
bb11 = {
_7 = [183376770626587292176694303882179088954_u128,148540430901222097538038582068908227438_u128,178475642479167105432974219976383448747_u128,201641814587828074976877698823746434296_u128,204816030462625407904890637972838017266_u128,240921668052771747352944162171818662197_u128,162578541238290242805081183581841599552_u128];
_8 = 43120_u16 as f32;
_6.fld0 = [116_i8,56_i8,(-81_i8)];
_8 = 12517449951621229875_u64 as f32;
RET = 308635054204488170301682490883858948487_u128 as isize;
RET = !_3;
_5 = (-4501115945353987403_i64);
_6.fld0 = [49_i8,0_i8,(-81_i8)];
_6.fld1 = [13923501357205451152_u64,4631016615047805923_u64,15561092806842679268_u64,3412150093224407012_u64,17824103032074058562_u64];
RET = -_3;
RET = _3 ^ _3;
_6.fld0 = [111_i8,(-31_i8),18_i8];
_6.fld1 = [3380201921328089303_u64,2190571073336278382_u64,1845460742339174222_u64,12582173858424673358_u64,6677808366750327008_u64];
_10 = _3 as f64;
_14 = _9 ^ _9;
_1 = [7848092528718835528_u64,4085823889672034842_u64,895295607903447033_u64,11947233325048884952_u64];
RET = -_3;
Goto(bb12)
}
bb12 = {
_10 = 21377117347257422468339527893023218236_u128 as f64;
_1 = [11830566108495881858_u64,15903867832693515707_u64,15292508510561301256_u64,11725159309560399567_u64];
_8 = _5 as f32;
_10 = (-17909_i16) as f64;
Call(_15 = core::intrinsics::bswap(28209_u16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_1 = [5262803288418135409_u64,5184243700122898339_u64,3556789298461767411_u64,17429409903058478261_u64];
Call(_9 = core::intrinsics::bswap(_14), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11 = [18101122961037156943_u64,13460529368763618311_u64,902069631312868565_u64,3745088026523903048_u64];
_3 = 1111522719685820128784125223861993934_u128 as isize;
_14 = 324887553569448790319238054840660700098_u128 as usize;
_17 = 105788262379698073776930553247716203700_i128;
_10 = 165_u8 as f64;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(19_usize, 14_usize, Move(_14), 9_usize, Move(_9), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(7984474744106073618_i64), std::hint::black_box((-1425537266_i32)), std::hint::black_box(3511081200_u32));
                
            }
impl PrintFDebug for Adt25{
	unsafe fn printf_debug(&self){unsafe{printf("Adt25::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
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
#[derive(Copy,Clone)]pub enum Adt25 {
Variant0{
fld0: bool,
fld1: [u32; 4],
fld2: *mut bool,
fld3: u8,
fld4: f64,

},
Variant1{
fld0: *mut usize,
fld1: *const f64,
fld2: isize,
fld3: u128,
fld4: usize,

},
Variant2{
fld0: (*mut usize,),
fld1: char,
fld2: i64,
fld3: u32,
fld4: [u32; 4],
fld5: i32,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: u16,
fld1: u8,
fld2: u128,
fld3: *mut i32,
fld4: *const f64,
fld5: *mut bool,

},
Variant1{
fld0: *mut usize,
fld1: u128,
fld2: i64,

},
Variant2{
fld0: bool,
fld1: usize,
fld2: isize,

},
Variant3{
fld0: f64,
fld1: *mut usize,
fld2: u32,
fld3: i8,
fld4: i16,
fld5: i32,
fld6: u64,
fld7: i128,

}}
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
fld0: bool,
fld1: usize,
fld2: [u32; 2],
fld3: u8,
fld4: i16,
fld5: u16,
fld6: [u128; 7],
fld7: *mut usize,

},
Variant1{
fld0: i128,

},
Variant2{
fld0: bool,
fld1: u128,
fld2: Adt25,
fld3: *const f64,
fld4: i16,
fld5: [u32; 2],

},
Variant3{
fld0: u32,
fld1: [u128; 7],
fld2: *const f64,
fld3: i8,
fld4: i16,
fld5: f64,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: [i8; 3],
fld1: [u64; 5],
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u8,
fld1: i128,
fld2: [u32; 2],
fld3: i8,
fld4: u64,
fld5: Adt31,
fld6: i64,

},
Variant1{
fld0: [u128; 7],

},
Variant2{
fld0: usize,
fld1: (*mut usize,),
fld2: *const u16,
fld3: i8,
fld4: *mut usize,
fld5: Adt25,

},
Variant3{
fld0: [u32; 4],
fld1: char,
fld2: [u32; 2],
fld3: u128,
fld4: *mut bool,
fld5: i32,
fld6: [u64; 5],
fld7: *mut i32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u16,
fld1: u32,
fld2: [u32; 2],
fld3: [u32; 4],

},
Variant1{
fld0: *mut usize,

},
Variant2{
fld0: [u64; 5],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: [u32; 4],
fld1: [u128; 7],
fld2: ([u32; 4],),
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: *mut bool,
}

