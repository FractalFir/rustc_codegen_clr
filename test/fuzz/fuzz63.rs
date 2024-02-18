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
pub fn fn0() -> *const u64 {
mir! {
type RET = *const u64;
let _1: char;
let _2: u16;
let _3: [i64; 4];
let _4: Adt44;
let _5: *mut (i64, i8, *const u64, [char; 8]);
let _6: f32;
let _7: char;
let _8: *mut u16;
let _9: bool;
let _10: [i128; 3];
let _11: ((i32, u128, &'static bool), [i8; 8], *const *mut (i64, i8, *const u64, [char; 8]));
let _12: ([i128; 4], ([i128; 3], bool, [char; 7], isize));
let _13: [i16; 5];
let _14: *mut (i64, i8, *const u64, [char; 8]);
let _15: u16;
let _16: (u32, [u8; 8]);
let _17: isize;
let _18: &'static u32;
let _19: isize;
let _20: isize;
let _21: char;
let _22: *mut i128;
let _23: Adt55;
let _24: isize;
let _25: f64;
let _26: (i64, u64);
let _27: bool;
let _28: *const *const *mut (i64, i8, *const u64, [char; 8]);
let _29: [i128; 4];
let _30: u128;
let _31: *mut *const char;
let _32: char;
let _33: f32;
let _34: u64;
let _35: bool;
let _36: u8;
let _37: f64;
let _38: i64;
let _39: ();
let _40: ();
{
_1 = '\u{69f20}';
_1 = '\u{8a4e2}';
_1 = '\u{3105}';
_2 = 64560_u16 << 232_u8;
_2 = 43095_u16 * 62041_u16;
_1 = '\u{664d8}';
_1 = '\u{6d5db}';
_1 = '\u{a0518}';
_1 = '\u{a68d8}';
_1 = '\u{536c0}';
_3 = [(-9198029215261011486_i64),7727964577723715832_i64,(-4932092883024690301_i64),6175466793512900882_i64];
_3 = [(-6678758197146095593_i64),6721055457358798284_i64,5203349272190165438_i64,(-7927914286140992637_i64)];
_1 = '\u{5bddd}';
_1 = '\u{72871}';
_2 = !1369_u16;
_2 = 52688_u16;
_1 = '\u{7b186}';
_3 = [2436010186044280197_i64,(-3481305911919942264_i64),(-6765690929636093228_i64),(-5219838031117741154_i64)];
_1 = '\u{697f6}';
_1 = '\u{af3c3}';
Call(RET = fn1(_3, _2, _2, _3, _2, _1, _2, _3, _3, _3, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [4295814293207721451_i64,(-8434250528861400061_i64),2202715206014738896_i64,(-7528768940687019222_i64)];
_2 = 2_usize as u16;
_6 = 69_u8 as f32;
_1 = '\u{d366f}';
_1 = '\u{f34f3}';
_6 = 17099611849794403210_usize as f32;
_1 = '\u{6ca44}';
_2 = 53420_u16;
_1 = '\u{6abc}';
_3 = [3827673449252475389_i64,2901945449900682745_i64,1791454155946049253_i64,(-8151772008366743018_i64)];
_6 = 251_u8 as f32;
_7 = _1;
_2 = !31612_u16;
_1 = _7;
_1 = _7;
_1 = _7;
Goto(bb2)
}
bb2 = {
_6 = 15423653038155847140_u64 as f32;
_9 = false;
_2 = 30240_u16 | 5348_u16;
_3 = [(-7549390153439058379_i64),(-993032875064872903_i64),3624083803993613237_i64,1099138289731607717_i64];
_10 = [45787244510194315917751362803181806995_i128,(-23834400885202026609430213801995116872_i128),(-39758134365959584238024648138871749707_i128)];
_11.2 = core::ptr::addr_of!(_5);
_7 = _1;
_8 = core::ptr::addr_of_mut!(_2);
_1 = _7;
_11.0.1 = !55901781937870277423547034335911680657_u128;
_12.1.0 = _10;
Goto(bb3)
}
bb3 = {
_12.1.2 = [_7,_7,_1,_7,_7,_1,_7];
_12.1.1 = _9;
_12.1.3 = (-9223372036854775808_isize) ^ (-13_isize);
_12.1.3 = (-9223372036854775808_isize);
_12.1.1 = _9 < _9;
_6 = (-129547607861909964520286560519251315291_i128) as f32;
_11.0.1 = 207786043898448074285516957940370624625_u128 + 88503205792006724412973927608182338926_u128;
_8 = core::ptr::addr_of_mut!(_2);
_11.0.1 = 56559842693117451128099056317463454064_u128 << (*_8);
_12.1.1 = _9;
_11.1 = [99_i8,91_i8,119_i8,122_i8,105_i8,62_i8,29_i8,102_i8];
_10 = _12.1.0;
_13 = [9954_i16,(-26950_i16),(-21272_i16),(-5500_i16),(-15654_i16)];
_12.0 = [56653771164790835134154777147356419443_i128,131193280069829253251789856065394119084_i128,(-101655211227665726700995855450302744718_i128),(-79448335765550346551345108735690176110_i128)];
_12.1.0 = [157236772032388629604356731799190434249_i128,(-10621110471948748909882273619675590682_i128),(-113924386305262504865545070214866428270_i128)];
_11.1 = [8_i8,87_i8,(-93_i8),(-23_i8),(-85_i8),(-77_i8),96_i8,(-110_i8)];
Goto(bb4)
}
bb4 = {
_11.0.2 = &_9;
_2 = (-76_i8) as u16;
_12.1.3 = (-66_isize) >> _11.0.1;
_11.2 = core::ptr::addr_of!(_5);
_11.2 = core::ptr::addr_of!(_5);
_11.0.1 = 1389202692_i32 as u128;
_9 = _12.1.1 & _12.1.1;
_12.0 = [(-141774819401053925587884976966387442867_i128),(-93737852678721547413394347301561597021_i128),142464742692246095466623963497655191153_i128,(-54006208260940416505679732433995702976_i128)];
_11.0.1 = !153832325820783355354572098400400035061_u128;
_9 = _12.1.1;
(*_8) = 27152_u16 - 33048_u16;
_9 = _12.1.1;
_11.0.0 = -1441629204_i32;
Goto(bb5)
}
bb5 = {
_10 = [(-39407508928129387328136632697548696319_i128),57108177169386747007061785975767205257_i128,82992222067128017861970988396811570218_i128];
_12.1.3 = (-9223372036854775808_isize);
_11.0.2 = &_12.1.1;
_13 = [4682_i16,(-7234_i16),(-14809_i16),17003_i16,(-18608_i16)];
_11.0.0 = 1197064840_i32 + 2027713481_i32;
_11.2 = core::ptr::addr_of!(_5);
(*_8) = 18336_u16;
(*_8) = 3579_u16 - 51882_u16;
_9 = !_12.1.1;
Goto(bb6)
}
bb6 = {
_12.1.1 = _6 == _6;
_11.0.1 = !103731555606692959075233548134052112224_u128;
_17 = !_12.1.3;
_3 = [644433646135673578_i64,5071640958005756679_i64,6978057332124139952_i64,(-1888948931129404776_i64)];
_10 = [88742480487761293300122516699773061013_i128,(-166504403546930456258585176460982866452_i128),(-132092536324448482719359760540405253331_i128)];
_11.1 = [63_i8,6_i8,(-101_i8),(-32_i8),(-110_i8),(-106_i8),(-88_i8),116_i8];
_8 = core::ptr::addr_of_mut!((*_8));
_3 = [(-8559919868489758976_i64),5946177643410654844_i64,(-706690890950422705_i64),1489435529335540489_i64];
_17 = (-105547734961329384988043294197507398055_i128) as isize;
_12.1.2 = [_1,_7,_7,_1,_7,_7,_7];
_21 = _7;
(*_8) = 12565_u16;
_11.0.2 = &_12.1.1;
_6 = 118517232679177541914865062413390284196_i128 as f32;
_16.0 = !646340258_u32;
_9 = _12.1.1 | _12.1.1;
_10 = [(-37735911697569899928539696045069945155_i128),121122602944019474851447425065022434028_i128,107367638949344144874252295554926599439_i128];
_12.1.0 = _10;
_20 = 29149_i16 as isize;
_18 = &_16.0;
_9 = !_12.1.1;
_11.2 = core::ptr::addr_of!(_14);
_1 = _7;
_21 = _1;
Goto(bb7)
}
bb7 = {
RET = core::ptr::addr_of!(_26.1);
_16.0 = 134_u8 as u32;
RET = core::ptr::addr_of!((*RET));
(*RET) = !8636934839091118228_u64;
_11.0.1 = 110460180439356426309542033101130880124_u128 << (*_8);
_27 = _12.1.1;
_8 = core::ptr::addr_of_mut!(_2);
_25 = (-29803_i16) as f64;
_12.1.1 = _9;
_15 = (*_8) - (*_8);
_16.1 = [135_u8,9_u8,9_u8,222_u8,189_u8,29_u8,112_u8,83_u8];
Goto(bb8)
}
bb8 = {
_18 = &_16.0;
_26.0 = 7132668214287995319_i64;
_12.0 = [(-20988831626265893651612030132133991857_i128),85743542460857811008778015529398685113_i128,92247885688186852747360957782325244604_i128,53836538417666095244565556073215297369_i128];
Goto(bb9)
}
bb9 = {
_28 = core::ptr::addr_of!(_11.2);
_21 = _1;
_2 = !_15;
_19 = _17;
RET = core::ptr::addr_of!((*RET));
_26.1 = !9210035068784988305_u64;
(*RET) = _26.0 as u64;
_13 = [13575_i16,24284_i16,31581_i16,(-12397_i16),(-19656_i16)];
(*_28) = core::ptr::addr_of!(_14);
(*_8) = _15 << _16.0;
_11.1 = [(-111_i8),(-121_i8),21_i8,4_i8,(-61_i8),(-35_i8),61_i8,4_i8];
_29 = [(-90647388637214819378699795562056285148_i128),(-117161948557397150614587042241461348430_i128),155747194870214069748129096113255247030_i128,(-13822662083183494305660337758805565950_i128)];
_26.1 = 16790568736994627732_u64;
_17 = _19 << _2;
_20 = _12.1.3 ^ _17;
(*_28) = core::ptr::addr_of!(_14);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*_28) = core::ptr::addr_of!(_5);
_11.0.2 = &_27;
_7 = _21;
_11.0.1 = 16453351826768550488449131372163531992_u128 >> (*RET);
_9 = !_27;
_4 = Adt44::Variant1 { fld0: _12.1.1,fld1: _12.1.2,fld2: _26,fld3: _11.0.1,fld4: _25,fld5: _13,fld6: _26.0,fld7: 80153890188413056143555606095720577073_i128 };
Goto(bb10)
}
bb10 = {
_22 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_4, 1), 7)));
_30 = _9 as u128;
_8 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i64>(Variant(_4, 1), 6)) = _26.0;
_12.1.1 = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<(i64, u64)>(Variant(_4, 1), 2)) = (Field::<i64>(Variant(_4, 1), 6), (*RET));
_26.1 = Field::<(i64, u64)>(Variant(_4, 1), 2).1;
_18 = &_16.0;
_15 = !_2;
_7 = _1;
_11.1 = [(-35_i8),(-49_i8),3_i8,(-41_i8),52_i8,118_i8,83_i8,(-20_i8)];
place!(Field::<i64>(Variant(_4, 1), 6)) = (*_8) as i64;
place!(Field::<(i64, u64)>(Variant(_4, 1), 2)).0 = Field::<i64>(Variant(_4, 1), 6) + Field::<i64>(Variant(_4, 1), 6);
_13 = Field::<[i16; 5]>(Variant(_4, 1), 5);
_25 = Field::<f64>(Variant(_4, 1), 4) * Field::<f64>(Variant(_4, 1), 4);
_20 = _17 | _17;
_28 = core::ptr::addr_of!((*_28));
match _12.1.3 {
0 => bb1,
1 => bb4,
2 => bb8,
340282366920938463454151235394913435648 => bb12,
_ => bb11
}
}
bb11 = {
_28 = core::ptr::addr_of!(_11.2);
_21 = _1;
_2 = !_15;
_19 = _17;
RET = core::ptr::addr_of!((*RET));
_26.1 = !9210035068784988305_u64;
(*RET) = _26.0 as u64;
_13 = [13575_i16,24284_i16,31581_i16,(-12397_i16),(-19656_i16)];
(*_28) = core::ptr::addr_of!(_14);
(*_8) = _15 << _16.0;
_11.1 = [(-111_i8),(-121_i8),21_i8,4_i8,(-61_i8),(-35_i8),61_i8,4_i8];
_29 = [(-90647388637214819378699795562056285148_i128),(-117161948557397150614587042241461348430_i128),155747194870214069748129096113255247030_i128,(-13822662083183494305660337758805565950_i128)];
_26.1 = 16790568736994627732_u64;
_17 = _19 << _2;
_20 = _12.1.3 ^ _17;
(*_28) = core::ptr::addr_of!(_14);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*_28) = core::ptr::addr_of!(_5);
_11.0.2 = &_27;
_7 = _21;
_11.0.1 = 16453351826768550488449131372163531992_u128 >> (*RET);
_9 = !_27;
_4 = Adt44::Variant1 { fld0: _12.1.1,fld1: _12.1.2,fld2: _26,fld3: _11.0.1,fld4: _25,fld5: _13,fld6: _26.0,fld7: 80153890188413056143555606095720577073_i128 };
Goto(bb10)
}
bb12 = {
_3 = [Field::<i64>(Variant(_4, 1), 6),Field::<(i64, u64)>(Variant(_4, 1), 2).0,Field::<(i64, u64)>(Variant(_4, 1), 2).0,Field::<(i64, u64)>(Variant(_4, 1), 2).0];
match _26.0 {
0 => bb13,
1 => bb14,
7132668214287995319 => bb16,
_ => bb15
}
}
bb13 = {
_28 = core::ptr::addr_of!(_11.2);
_21 = _1;
_2 = !_15;
_19 = _17;
RET = core::ptr::addr_of!((*RET));
_26.1 = !9210035068784988305_u64;
(*RET) = _26.0 as u64;
_13 = [13575_i16,24284_i16,31581_i16,(-12397_i16),(-19656_i16)];
(*_28) = core::ptr::addr_of!(_14);
(*_8) = _15 << _16.0;
_11.1 = [(-111_i8),(-121_i8),21_i8,4_i8,(-61_i8),(-35_i8),61_i8,4_i8];
_29 = [(-90647388637214819378699795562056285148_i128),(-117161948557397150614587042241461348430_i128),155747194870214069748129096113255247030_i128,(-13822662083183494305660337758805565950_i128)];
_26.1 = 16790568736994627732_u64;
_17 = _19 << _2;
_20 = _12.1.3 ^ _17;
(*_28) = core::ptr::addr_of!(_14);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*_28) = core::ptr::addr_of!(_5);
_11.0.2 = &_27;
_7 = _21;
_11.0.1 = 16453351826768550488449131372163531992_u128 >> (*RET);
_9 = !_27;
_4 = Adt44::Variant1 { fld0: _12.1.1,fld1: _12.1.2,fld2: _26,fld3: _11.0.1,fld4: _25,fld5: _13,fld6: _26.0,fld7: 80153890188413056143555606095720577073_i128 };
Goto(bb10)
}
bb14 = {
_18 = &_16.0;
_26.0 = 7132668214287995319_i64;
_12.0 = [(-20988831626265893651612030132133991857_i128),85743542460857811008778015529398685113_i128,92247885688186852747360957782325244604_i128,53836538417666095244565556073215297369_i128];
Goto(bb9)
}
bb15 = {
_11.0.2 = &_9;
_2 = (-76_i8) as u16;
_12.1.3 = (-66_isize) >> _11.0.1;
_11.2 = core::ptr::addr_of!(_5);
_11.2 = core::ptr::addr_of!(_5);
_11.0.1 = 1389202692_i32 as u128;
_9 = _12.1.1 & _12.1.1;
_12.0 = [(-141774819401053925587884976966387442867_i128),(-93737852678721547413394347301561597021_i128),142464742692246095466623963497655191153_i128,(-54006208260940416505679732433995702976_i128)];
_11.0.1 = !153832325820783355354572098400400035061_u128;
_9 = _12.1.1;
(*_8) = 27152_u16 - 33048_u16;
_9 = _12.1.1;
_11.0.0 = -1441629204_i32;
Goto(bb5)
}
bb16 = {
_26.1 = Field::<(i64, u64)>(Variant(_4, 1), 2).1 << Field::<(i64, u64)>(Variant(_4, 1), 2).1;
_24 = !_20;
_26 = (Field::<i64>(Variant(_4, 1), 6), Field::<(i64, u64)>(Variant(_4, 1), 2).1);
(*RET) = Field::<(i64, u64)>(Variant(_4, 1), 2).1;
(*_22) = _6 as i128;
(*_22) = _11.0.0 as i128;
_1 = _21;
_18 = &(*_18);
_34 = !(*RET);
_12.1 = (_10, Field::<bool>(Variant(_4, 1), 0), Field::<[char; 7]>(Variant(_4, 1), 1), _24);
(*_8) = _30 as u16;
_34 = (*_18) as u64;
_35 = !_12.1.1;
_15 = !(*_8);
_11.0.0 = -1207575013_i32;
_3 = [Field::<i64>(Variant(_4, 1), 6),Field::<(i64, u64)>(Variant(_4, 1), 2).0,Field::<(i64, u64)>(Variant(_4, 1), 2).0,_26.0];
Goto(bb17)
}
bb17 = {
Call(_39 = dump_var(0_usize, 10_usize, Move(_10), 7_usize, Move(_7), 29_usize, Move(_29), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(0_usize, 16_usize, Move(_16), 35_usize, Move(_35), 3_usize, Move(_3), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(0_usize, 24_usize, Move(_24), 19_usize, Move(_19), 40_usize, _40, 40_usize, _40), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i64; 4],mut _2: u16,mut _3: u16,mut _4: [i64; 4],mut _5: u16,mut _6: char,mut _7: u16,mut _8: [i64; 4],mut _9: [i64; 4],mut _10: [i64; 4],mut _11: [i64; 4],mut _12: char) -> *const u64 {
mir! {
type RET = *const u64;
let _13: char;
let _14: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _15: &'static *mut i128;
let _16: u32;
let _17: &'static (i64, u64);
let _18: *const *mut (i64, i8, *const u64, [char; 8]);
let _19: Adt51;
let _20: &'static u32;
let _21: f64;
let _22: i32;
let _23: u64;
let _24: char;
let _25: char;
let _26: f32;
let _27: u16;
let _28: bool;
let _29: [char; 8];
let _30: isize;
let _31: u64;
let _32: isize;
let _33: f32;
let _34: [char; 8];
let _35: u128;
let _36: ([i128; 3], bool, [char; 7], isize);
let _37: char;
let _38: *mut u16;
let _39: (char,);
let _40: u64;
let _41: [usize; 7];
let _42: &'static usize;
let _43: (f64,);
let _44: Adt49;
let _45: i32;
let _46: (*mut *const char, &'static [char; 7], [isize; 4], &'static i32);
let _47: i64;
let _48: [char; 7];
let _49: i64;
let _50: [i128; 3];
let _51: isize;
let _52: (*const (i32, u128, &'static bool), [char; 8], u64);
let _53: char;
let _54: u64;
let _55: *mut (i32, u128, &'static bool);
let _56: ();
let _57: ();
{
_13 = _6;
_4 = [(-2867950183429204755_i64),(-8584401163606227348_i64),(-1110252694586216816_i64),(-4961496938378402592_i64)];
_5 = !_7;
_2 = !_3;
_10 = [(-3623713201471995427_i64),(-3096932665530066348_i64),2617420310025015757_i64,779751812589722248_i64];
_13 = _6;
_3 = !_2;
_5 = !_7;
_11 = [(-3688734988228547341_i64),(-3889398967972935004_i64),1704367358493566087_i64,(-8364588632183342216_i64)];
_13 = _6;
_7 = (-29_isize) as u16;
_1 = [(-1608747880912942277_i64),(-2231463938266056248_i64),(-3650608376451586394_i64),(-3900272558160174724_i64)];
_1 = _9;
_2 = _5;
_16 = 963398546379804943_i64 as u32;
_10 = [4097174346878706578_i64,(-7838563642676441917_i64),5226052071105710061_i64,(-1165210216909450140_i64)];
_9 = [8956574640918568803_i64,4931655356533244130_i64,(-261206462489704089_i64),4067669099084969463_i64];
_2 = _7;
_2 = 7981361530973714823_i64 as u16;
_6 = _12;
_9 = [(-3760767698836639102_i64),(-2357599196538486204_i64),2460838153550392265_i64,(-4483798290242846705_i64)];
_5 = _7 & _2;
_11 = [1262606923612276969_i64,(-8157420811115204185_i64),(-9140738080216667489_i64),1475406901876427294_i64];
_16 = 3273337049_u32 + 563465181_u32;
_7 = _3 >> _3;
_7 = !_3;
_3 = 1523148596910599324_u64 as u16;
Goto(bb1)
}
bb1 = {
_12 = _13;
_8 = [(-3290316791742982871_i64),(-670737179744042470_i64),(-5490776387902569447_i64),(-6754378493833221268_i64)];
_2 = _3 + _5;
_7 = !_2;
_5 = !_2;
_10 = [(-121959329748645599_i64),(-1805154610355800163_i64),5499711744571987510_i64,(-1922591497685749499_i64)];
_19.fld0 = [22_i8,(-107_i8)];
_13 = _12;
Goto(bb2)
}
bb2 = {
RET = core::ptr::addr_of!(_23);
_10 = [1295580855017981448_i64,(-1764291486088215752_i64),5417319477551624460_i64,1956946523633560014_i64];
_21 = _16 as f64;
_19.fld0 = [(-38_i8),104_i8];
_22 = (-1137359797_i32);
_14 = core::ptr::addr_of_mut!(_18);
(*RET) = 15026733955365364028_u64 | 12457082192508712584_u64;
_9 = [(-7941026465207775940_i64),756612864273965114_i64,6968524569354636017_i64,3666588742362755196_i64];
_10 = [(-6527998269225724883_i64),8094146410252962711_i64,(-3101801094355546056_i64),4397041478489662693_i64];
Goto(bb3)
}
bb3 = {
_22 = _7 as i32;
_5 = _7;
_20 = &_16;
_21 = 14316_i16 as f64;
_12 = _13;
_7 = !_2;
_20 = &(*_20);
(*RET) = !11154836403715148748_u64;
_9 = _8;
(*RET) = 16021056371129822843_u64;
_7 = 1771053197407081958_usize as u16;
RET = core::ptr::addr_of!(_23);
_7 = _2;
_26 = 107688307301988281358729864111337837737_i128 as f32;
_8 = _4;
_25 = _6;
_2 = !_7;
(*RET) = 8544041408173318137_u64 & 6419551813433438038_u64;
_14 = core::ptr::addr_of_mut!((*_14));
_4 = [(-7154902497950141748_i64),(-5761569682325008987_i64),(-5653936231283666577_i64),(-8061550957004040359_i64)];
_3 = _5;
Goto(bb4)
}
bb4 = {
_6 = _25;
_28 = _7 >= _7;
_19.fld1 = [_12,_25,_6,_25,_25,_25,_12];
_13 = _25;
_11 = [7238983642599269828_i64,(-1636097198509090238_i64),6629129604360277876_i64,(-442538183068778743_i64)];
_26 = _22 as f32;
_19.fld1 = [_25,_13,_12,_25,_6,_13,_6];
_12 = _6;
_23 = 5086800210064765060_u64 & 16682076369059949300_u64;
_11 = [(-5136799906241849459_i64),(-4893911599185314691_i64),(-7543712276293091326_i64),8238921151761973719_i64];
_23 = 72_i8 as u64;
_27 = 718_i16 as u16;
(*RET) = 4083391195497072647_u64;
_26 = _22 as f32;
_32 = -9223372036854775807_isize;
_16 = 1958705956_u32;
_5 = _2 + _7;
_21 = 63494542387588287644765767149523579039_u128 as f64;
_29 = [_13,_6,_13,_13,_25,_25,_25,_13];
_20 = &_16;
RET = core::ptr::addr_of!((*RET));
_31 = _6 as u64;
_24 = _6;
match _16 {
0 => bb5,
1958705956 => bb7,
_ => bb6
}
}
bb5 = {
_12 = _13;
_8 = [(-3290316791742982871_i64),(-670737179744042470_i64),(-5490776387902569447_i64),(-6754378493833221268_i64)];
_2 = _3 + _5;
_7 = !_2;
_5 = !_2;
_10 = [(-121959329748645599_i64),(-1805154610355800163_i64),5499711744571987510_i64,(-1922591497685749499_i64)];
_19.fld0 = [22_i8,(-107_i8)];
_13 = _12;
Goto(bb2)
}
bb6 = {
RET = core::ptr::addr_of!(_23);
_10 = [1295580855017981448_i64,(-1764291486088215752_i64),5417319477551624460_i64,1956946523633560014_i64];
_21 = _16 as f64;
_19.fld0 = [(-38_i8),104_i8];
_22 = (-1137359797_i32);
_14 = core::ptr::addr_of_mut!(_18);
(*RET) = 15026733955365364028_u64 | 12457082192508712584_u64;
_9 = [(-7941026465207775940_i64),756612864273965114_i64,6968524569354636017_i64,3666588742362755196_i64];
_10 = [(-6527998269225724883_i64),8094146410252962711_i64,(-3101801094355546056_i64),4397041478489662693_i64];
Goto(bb3)
}
bb7 = {
_8 = _1;
_4 = _11;
_30 = _32 & _32;
_26 = 91549589753615243493429916528338549719_i128 as f32;
_4 = [(-33218485497651172_i64),(-3947985309401016393_i64),2480512414716841723_i64,8886635906647933916_i64];
_10 = [494492093768006021_i64,4379905754349936040_i64,(-5446276682740623752_i64),8633455337130984547_i64];
(*RET) = (-2981676896031444831_i64) as u64;
_5 = _6 as u16;
RET = core::ptr::addr_of!(_23);
_21 = _22 as f64;
_3 = _5;
_3 = (-8135108950974274857_i64) as u16;
_4 = _10;
_8 = _11;
_24 = _13;
_3 = !_2;
_11 = _1;
_19.fld1 = [_12,_24,_6,_25,_24,_6,_6];
_22 = 655457093_i32;
_10 = _11;
_16 = !2736856299_u32;
(*RET) = 18018233647302550181_usize as u64;
_37 = _25;
RET = core::ptr::addr_of!((*RET));
_22 = -(-235544806_i32);
_34 = _29;
_29 = _34;
_2 = _3 ^ _7;
Call(_3 = fn2(_28, _24, Move(_19), _30, _2, _30), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_36.1 = (*RET) != (*RET);
_36.0 = [(-132361366527539876327058356736613618493_i128),92957579851441927202965359814370091954_i128,43105619926371639117140173352591640088_i128];
_20 = &_16;
_12 = _24;
_3 = _2;
_8 = _11;
_19.fld0 = [8_i8,35_i8];
_5 = _7 << _30;
_39 = (_13,);
_27 = (-6097487498585506722_i64) as u16;
_12 = _37;
_12 = _6;
_2 = 15139_i16 as u16;
_26 = (*RET) as f32;
_40 = _23 & (*RET);
_20 = &_16;
_21 = _30 as f64;
(*RET) = _31;
_43 = (_21,);
_38 = core::ptr::addr_of_mut!(_5);
Goto(bb9)
}
bb9 = {
_12 = _25;
_26 = 16777975731786566630_usize as f32;
_31 = (-4366832072977181268_i64) as u64;
_32 = _30;
_41 = [3_usize,15873152655081163084_usize,3_usize,5933392147024920342_usize,0_usize,7_usize,3869499344787505102_usize];
RET = core::ptr::addr_of!(_31);
_21 = -_43.0;
_21 = -_43.0;
_11 = [6815180041799118758_i64,5618706966502151876_i64,5976404719861336333_i64,(-2582658591732024420_i64)];
_3 = _5 * _5;
Goto(bb10)
}
bb10 = {
_37 = _24;
_7 = (*_38);
_31 = _40;
_37 = _12;
_33 = 48_u8 as f32;
_35 = 133435944342553725786263235450267520974_u128;
_43.0 = _21 * _21;
_33 = -_26;
_36.0 = [(-149283872901039064399980730613818759664_i128),62107112232972353847537543114258071437_i128,70921950683875851863621876968450728534_i128];
_25 = _37;
_43 = (_21,);
_39.0 = _25;
_45 = !_22;
(*RET) = _40;
_45 = !_22;
_46.3 = &_45;
_29 = [_39.0,_24,_13,_25,_39.0,_25,_13,_37];
(*RET) = _40;
_19.fld1 = [_24,_39.0,_37,_6,_37,_37,_25];
_5 = !_3;
_19.fld1 = [_37,_39.0,_25,_24,_37,_12,_6];
_36.2 = [_6,_39.0,_12,_24,_39.0,_37,_24];
match _35 {
0 => bb4,
1 => bb6,
133435944342553725786263235450267520974 => bb12,
_ => bb11
}
}
bb11 = {
_36.1 = (*RET) != (*RET);
_36.0 = [(-132361366527539876327058356736613618493_i128),92957579851441927202965359814370091954_i128,43105619926371639117140173352591640088_i128];
_20 = &_16;
_12 = _24;
_3 = _2;
_8 = _11;
_19.fld0 = [8_i8,35_i8];
_5 = _7 << _30;
_39 = (_13,);
_27 = (-6097487498585506722_i64) as u16;
_12 = _37;
_12 = _6;
_2 = 15139_i16 as u16;
_26 = (*RET) as f32;
_40 = _23 & (*RET);
_20 = &_16;
_21 = _30 as f64;
(*RET) = _31;
_43 = (_21,);
_38 = core::ptr::addr_of_mut!(_5);
Goto(bb9)
}
bb12 = {
_36.3 = _30;
_6 = _12;
(*RET) = !_40;
_49 = _28 as i64;
_48 = [_37,_24,_12,_39.0,_13,_12,_24];
_39.0 = _12;
_46.1 = &_48;
_33 = _26;
_27 = !(*_38);
_22 = _43.0 as i32;
_46.2 = [_36.3,_30,_36.3,_36.3];
_26 = -_33;
_9 = _8;
_3 = (*_38);
_49 = _22 as i64;
_2 = _36.3 as u16;
(*RET) = 1293877830113854514_usize as u64;
_37 = _39.0;
_37 = _12;
_25 = _6;
_34 = _29;
_37 = _13;
_43 = (_21,);
(*RET) = _40;
_36.2 = [_12,_6,_12,_13,_24,_24,_6];
_33 = -_26;
_36.0 = [17502154831015855560112519569531764995_i128,135723321149054639325418511239815119822_i128,104968551228786897003098361704626365726_i128];
match _35 {
0 => bb3,
1 => bb4,
2 => bb13,
3 => bb14,
4 => bb15,
133435944342553725786263235450267520974 => bb17,
_ => bb16
}
}
bb13 = {
_36.1 = (*RET) != (*RET);
_36.0 = [(-132361366527539876327058356736613618493_i128),92957579851441927202965359814370091954_i128,43105619926371639117140173352591640088_i128];
_20 = &_16;
_12 = _24;
_3 = _2;
_8 = _11;
_19.fld0 = [8_i8,35_i8];
_5 = _7 << _30;
_39 = (_13,);
_27 = (-6097487498585506722_i64) as u16;
_12 = _37;
_12 = _6;
_2 = 15139_i16 as u16;
_26 = (*RET) as f32;
_40 = _23 & (*RET);
_20 = &_16;
_21 = _30 as f64;
(*RET) = _31;
_43 = (_21,);
_38 = core::ptr::addr_of_mut!(_5);
Goto(bb9)
}
bb14 = {
RET = core::ptr::addr_of!(_23);
_10 = [1295580855017981448_i64,(-1764291486088215752_i64),5417319477551624460_i64,1956946523633560014_i64];
_21 = _16 as f64;
_19.fld0 = [(-38_i8),104_i8];
_22 = (-1137359797_i32);
_14 = core::ptr::addr_of_mut!(_18);
(*RET) = 15026733955365364028_u64 | 12457082192508712584_u64;
_9 = [(-7941026465207775940_i64),756612864273965114_i64,6968524569354636017_i64,3666588742362755196_i64];
_10 = [(-6527998269225724883_i64),8094146410252962711_i64,(-3101801094355546056_i64),4397041478489662693_i64];
Goto(bb3)
}
bb15 = {
RET = core::ptr::addr_of!(_23);
_10 = [1295580855017981448_i64,(-1764291486088215752_i64),5417319477551624460_i64,1956946523633560014_i64];
_21 = _16 as f64;
_19.fld0 = [(-38_i8),104_i8];
_22 = (-1137359797_i32);
_14 = core::ptr::addr_of_mut!(_18);
(*RET) = 15026733955365364028_u64 | 12457082192508712584_u64;
_9 = [(-7941026465207775940_i64),756612864273965114_i64,6968524569354636017_i64,3666588742362755196_i64];
_10 = [(-6527998269225724883_i64),8094146410252962711_i64,(-3101801094355546056_i64),4397041478489662693_i64];
Goto(bb3)
}
bb16 = {
_36.1 = (*RET) != (*RET);
_36.0 = [(-132361366527539876327058356736613618493_i128),92957579851441927202965359814370091954_i128,43105619926371639117140173352591640088_i128];
_20 = &_16;
_12 = _24;
_3 = _2;
_8 = _11;
_19.fld0 = [8_i8,35_i8];
_5 = _7 << _30;
_39 = (_13,);
_27 = (-6097487498585506722_i64) as u16;
_12 = _37;
_12 = _6;
_2 = 15139_i16 as u16;
_26 = (*RET) as f32;
_40 = _23 & (*RET);
_20 = &_16;
_21 = _30 as f64;
(*RET) = _31;
_43 = (_21,);
_38 = core::ptr::addr_of_mut!(_5);
Goto(bb9)
}
bb17 = {
_10 = [_49,_49,_49,_49];
_40 = _23 >> _3;
_23 = (*RET) + _40;
_52.1 = _34;
_12 = _6;
Goto(bb18)
}
bb18 = {
Call(_56 = dump_var(1_usize, 31_usize, Move(_31), 12_usize, Move(_12), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(1_usize, 4_usize, Move(_4), 5_usize, Move(_5), 7_usize, Move(_7), 49_usize, Move(_49)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_56 = dump_var(1_usize, 41_usize, Move(_41), 24_usize, Move(_24), 36_usize, Move(_36), 28_usize, Move(_28)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_56 = dump_var(1_usize, 16_usize, Move(_16), 48_usize, Move(_48), 6_usize, Move(_6), 37_usize, Move(_37)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_56 = dump_var(1_usize, 11_usize, Move(_11), 57_usize, _57, 57_usize, _57, 57_usize, _57), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: char,mut _3: Adt51,mut _4: isize,mut _5: u16,mut _6: isize) -> u16 {
mir! {
type RET = u16;
let _7: char;
let _8: [u32; 7];
let _9: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _10: *mut (i32, u128, &'static bool);
let _11: usize;
let _12: bool;
let _13: [char; 8];
let _14: (i32, u128, &'static bool);
let _15: f64;
let _16: char;
let _17: (i64, i8, *const u64, [char; 8]);
let _18: Adt44;
let _19: usize;
let _20: f64;
let _21: u32;
let _22: *mut (i32, u128, &'static bool);
let _23: &'static u32;
let _24: u64;
let _25: isize;
let _26: Adt49;
let _27: f64;
let _28: f64;
let _29: (*const (i32, u128, &'static bool), [char; 8], u64);
let _30: ();
let _31: ();
{
_1 = false;
RET = _2 as u16;
_7 = _2;
_3.fld0 = [47_i8,(-40_i8)];
_1 = _5 != _5;
_4 = -_6;
_3.fld1 = [_7,_7,_7,_2,_2,_2,_7];
_4 = 94_u8 as isize;
_8 = [4217975469_u32,1835717634_u32,4193393098_u32,2303160466_u32,2376492589_u32,2457529044_u32,3916027115_u32];
_7 = _2;
_3.fld1 = [_2,_7,_2,_2,_2,_2,_7];
_6 = -_4;
Call(_2 = fn3(Move(_3), _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _2;
_6 = 69_u8 as isize;
_3.fld1 = [_7,_2,_2,_2,_7,_7,_2];
_8 = [1600359589_u32,2877533635_u32,1424556775_u32,3451978133_u32,2053179484_u32,587756847_u32,55004728_u32];
_7 = _2;
RET = !_5;
RET = 2606130955_u32 as u16;
_2 = _7;
_4 = _6;
_5 = _1 as u16;
_5 = RET << _4;
RET = !_5;
_2 = _7;
_4 = !_6;
_5 = 2027097856_i32 as u16;
_6 = (-7948257321413856418_i64) as isize;
_2 = _7;
_4 = 4383300526955440468_usize as isize;
Goto(bb2)
}
bb2 = {
RET = _5 & _5;
_2 = _7;
_3.fld0 = [(-115_i8),(-65_i8)];
_6 = -_4;
_3.fld0 = [81_i8,6_i8];
_8 = [3037751881_u32,4020642770_u32,4074410939_u32,2196762995_u32,2347582229_u32,786660416_u32,1151095294_u32];
_1 = _6 == _4;
_7 = _2;
_8 = [2595507334_u32,1124525041_u32,1506174813_u32,136031209_u32,4059824692_u32,2674167072_u32,1351252183_u32];
_14.0 = 633165396_i32;
_14.1 = 133165601343550116584525033411400918513_u128 & 255728902997831825269934373707250408331_u128;
_13 = [_7,_2,_7,_2,_2,_7,_2,_2];
_14.2 = &_12;
_8 = [3289415167_u32,973363353_u32,2161832702_u32,296051558_u32,1807736293_u32,2087399785_u32,3631460317_u32];
_14.2 = &_1;
_12 = !_1;
_11 = !2_usize;
_10 = core::ptr::addr_of_mut!(_14);
_13 = [_2,_2,_7,_2,_2,_2,_7,_7];
(*_10).2 = &_12;
_7 = _2;
_2 = _7;
match _14.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
633165396 => bb10,
_ => bb9
}
}
bb3 = {
_7 = _2;
_6 = 69_u8 as isize;
_3.fld1 = [_7,_2,_2,_2,_7,_7,_2];
_8 = [1600359589_u32,2877533635_u32,1424556775_u32,3451978133_u32,2053179484_u32,587756847_u32,55004728_u32];
_7 = _2;
RET = !_5;
RET = 2606130955_u32 as u16;
_2 = _7;
_4 = _6;
_5 = _1 as u16;
_5 = RET << _4;
RET = !_5;
_2 = _7;
_4 = !_6;
_5 = 2027097856_i32 as u16;
_6 = (-7948257321413856418_i64) as isize;
_2 = _7;
_4 = 4383300526955440468_usize as isize;
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
_4 = !_6;
_3.fld0 = [(-14_i8),(-51_i8)];
_3.fld0 = [(-116_i8),49_i8];
_7 = _2;
Goto(bb11)
}
bb11 = {
(*_10).2 = &_1;
RET = _14.0 as u16;
_4 = _14.1 as isize;
_14.2 = &_12;
(*_10).1 = !220371394396787852972245142608840895643_u128;
(*_10).1 = 191576432643132058562286266185852091661_u128;
_8 = [514844818_u32,2480078626_u32,1179808640_u32,1004494528_u32,1950897096_u32,2257696336_u32,3363792116_u32];
Goto(bb12)
}
bb12 = {
_17.1 = 3_u8 as i8;
_2 = _7;
(*_10).0 = _5 as i32;
_15 = 987181987512951741_i64 as f64;
_4 = _6 | _6;
(*_10).0 = 1275302094_i32 + (-1883313220_i32);
_11 = (*_10).1 as usize;
_5 = RET;
_15 = 11688332644596320188_u64 as f64;
_14.2 = &_12;
_1 = !_12;
(*_10).2 = &_12;
_16 = _7;
_20 = 195_u8 as f64;
(*_10).0 = 45552975_i32;
_21 = 2065108009_u32;
(*_10).2 = &_1;
_24 = _7 as u64;
match (*_10).0 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb13,
4 => bb14,
45552975 => bb16,
_ => bb15
}
}
bb13 = {
RET = _5 & _5;
_2 = _7;
_3.fld0 = [(-115_i8),(-65_i8)];
_6 = -_4;
_3.fld0 = [81_i8,6_i8];
_8 = [3037751881_u32,4020642770_u32,4074410939_u32,2196762995_u32,2347582229_u32,786660416_u32,1151095294_u32];
_1 = _6 == _4;
_7 = _2;
_8 = [2595507334_u32,1124525041_u32,1506174813_u32,136031209_u32,4059824692_u32,2674167072_u32,1351252183_u32];
_14.0 = 633165396_i32;
_14.1 = 133165601343550116584525033411400918513_u128 & 255728902997831825269934373707250408331_u128;
_13 = [_7,_2,_7,_2,_2,_7,_2,_2];
_14.2 = &_12;
_8 = [3289415167_u32,973363353_u32,2161832702_u32,296051558_u32,1807736293_u32,2087399785_u32,3631460317_u32];
_14.2 = &_1;
_12 = !_1;
_11 = !2_usize;
_10 = core::ptr::addr_of_mut!(_14);
_13 = [_2,_2,_7,_2,_2,_2,_7,_7];
(*_10).2 = &_12;
_7 = _2;
_2 = _7;
match _14.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
633165396 => bb10,
_ => bb9
}
}
bb14 = {
_7 = _2;
_6 = 69_u8 as isize;
_3.fld1 = [_7,_2,_2,_2,_7,_7,_2];
_8 = [1600359589_u32,2877533635_u32,1424556775_u32,3451978133_u32,2053179484_u32,587756847_u32,55004728_u32];
_7 = _2;
RET = !_5;
RET = 2606130955_u32 as u16;
_2 = _7;
_4 = _6;
_5 = _1 as u16;
_5 = RET << _4;
RET = !_5;
_2 = _7;
_4 = !_6;
_5 = 2027097856_i32 as u16;
_6 = (-7948257321413856418_i64) as isize;
_2 = _7;
_4 = 4383300526955440468_usize as isize;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_17.0 = -(-6280962736149250632_i64);
_2 = _16;
_21 = 4179018025_u32 - 3286183776_u32;
_17.1 = _20 as i8;
_24 = 14741137131431565314_u64;
(*_10).0 = !755536412_i32;
_3.fld1 = [_16,_2,_16,_2,_7,_16,_2];
(*_10).2 = &_12;
(*_10).2 = &_12;
_5 = RET;
RET = _5;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(2_usize, 21_usize, Move(_21), 11_usize, Move(_11), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(2_usize, 2_usize, Move(_2), 13_usize, Move(_13), 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: Adt51,mut _2: isize,mut _3: char) -> char {
mir! {
type RET = char;
let _4: [usize; 6];
let _5: char;
let _6: (*const &'static u32, [char; 3], [i128; 4], &'static u16);
let _7: char;
let _8: f64;
let _9: i128;
let _10: Adt73;
let _11: u32;
let _12: u128;
let _13: f32;
let _14: &'static *const (i32, u128, &'static bool);
let _15: &'static i16;
let _16: u64;
let _17: i64;
let _18: &'static (i64, u64);
let _19: &'static [char; 7];
let _20: Adt49;
let _21: *const *mut (i64, i8, *const u64, [char; 8]);
let _22: &'static i32;
let _23: isize;
let _24: bool;
let _25: bool;
let _26: u16;
let _27: usize;
let _28: f64;
let _29: ();
let _30: ();
{
_1.fld1 = [_3,_3,_3,_3,_3,_3,_3];
_1.fld1 = [_3,_3,_3,_3,_3,_3,_3];
RET = _3;
_3 = RET;
_3 = RET;
_1.fld0 = [(-74_i8),123_i8];
_3 = RET;
_3 = RET;
_3 = RET;
_5 = RET;
RET = _3;
_6.2 = [42806732952891831629245025896521133402_i128,27165852709636860565016902827417001328_i128,(-132249574935681922011184998895205886033_i128),(-62395873736076762717164020109470145550_i128)];
_3 = RET;
Call(_4 = fn4(_3, _1.fld0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.fld0 = [39_i8,(-26_i8)];
_6.1 = [_3,_5,RET];
RET = _5;
_6.1 = [_5,_5,_5];
_6.1 = [_3,_3,_3];
_2 = 64_isize;
_6.1 = [RET,RET,_5];
_1.fld1 = [RET,_3,_3,_3,_5,RET,_5];
_1.fld1 = [RET,_3,_3,RET,_3,RET,RET];
_5 = _3;
_7 = _3;
RET = _3;
RET = _3;
_6.1 = [RET,_7,_3];
_4 = [0_usize,1_usize,10131703517157877924_usize,2_usize,2721897524994553268_usize,607012332852867215_usize];
RET = _5;
_5 = _7;
_7 = _3;
_2 = (-125_i8) as isize;
_4 = [1764500872275659479_usize,8630579579427049477_usize,4835937546559384247_usize,11935710518314395981_usize,0_usize,2938100252718769683_usize];
_5 = _7;
_2 = !(-91_isize);
_1.fld0 = [(-7_i8),123_i8];
_9 = !(-76075437170403449871816622243037992066_i128);
_5 = RET;
_8 = 1542060513_i32 as f64;
RET = _7;
Goto(bb2)
}
bb2 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb3 = {
RET = _7;
match _9 {
0 => bb2,
1 => bb4,
206244444889315933976880575122055421715 => bb6,
_ => bb5
}
}
bb4 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb5 = {
_1.fld0 = [39_i8,(-26_i8)];
_6.1 = [_3,_5,RET];
RET = _5;
_6.1 = [_5,_5,_5];
_6.1 = [_3,_3,_3];
_2 = 64_isize;
_6.1 = [RET,RET,_5];
_1.fld1 = [RET,_3,_3,_3,_5,RET,_5];
_1.fld1 = [RET,_3,_3,RET,_3,RET,RET];
_5 = _3;
_7 = _3;
RET = _3;
RET = _3;
_6.1 = [RET,_7,_3];
_4 = [0_usize,1_usize,10131703517157877924_usize,2_usize,2721897524994553268_usize,607012332852867215_usize];
RET = _5;
_5 = _7;
_7 = _3;
_2 = (-125_i8) as isize;
_4 = [1764500872275659479_usize,8630579579427049477_usize,4835937546559384247_usize,11935710518314395981_usize,0_usize,2938100252718769683_usize];
_5 = _7;
_2 = !(-91_isize);
_1.fld0 = [(-7_i8),123_i8];
_9 = !(-76075437170403449871816622243037992066_i128);
_5 = RET;
_8 = 1542060513_i32 as f64;
RET = _7;
Goto(bb2)
}
bb6 = {
_6.2 = [_9,_9,_9,_9];
_6.2 = [_9,_9,_9,_9];
_1.fld1 = [_7,_3,_7,RET,RET,RET,_3];
_4 = [491399320109861797_usize,7268257579879734951_usize,5_usize,15704088531428549351_usize,2588682914089542650_usize,4_usize];
_6.1 = [_7,_3,_5];
_1.fld0 = [(-46_i8),(-21_i8)];
RET = _3;
_1.fld1 = [_3,_5,_5,RET,_7,_3,RET];
_6.2 = [_9,_9,_9,_9];
_2 = -(-30_isize);
_1.fld1 = [RET,_5,_5,_7,_7,_7,_7];
RET = _5;
_5 = RET;
_2 = (-9223372036854775808_isize) ^ 112_isize;
_2 = (-69_isize);
_4 = [4_usize,6_usize,695054446494600944_usize,0_usize,6774523968093706114_usize,5_usize];
_7 = _5;
_2 = (-35_i8) as isize;
_11 = 3194960851_u32;
_2 = 101_isize & (-9223372036854775808_isize);
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_1.fld0 = [(-17_i8),99_i8];
_7 = _5;
_6.2 = [_9,_9,_9,_9];
RET = _7;
_8 = 13220849460833100923_usize as f64;
_5 = _7;
_6.2 = [_9,_9,_9,_9];
Goto(bb7)
}
bb7 = {
_12 = 49690_u16 as u128;
_5 = _3;
_12 = 56114910569755093029611643652582352163_u128;
_1.fld1 = [_5,_5,RET,_5,_5,_3,_7];
place!(Field::<f64>(Variant(_10, 0), 0)) = 5988444194647357168_u64 as f64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [105_i8,(-108_i8)];
_1.fld1 = [RET,RET,_5,_7,_5,_7,_5];
_5 = _7;
_6.1 = [RET,_5,_3];
_3 = RET;
_13 = Field::<f64>(Variant(_10, 0), 0) as f32;
_9 = 16439067240727464861911206737606887588_i128;
_3 = _5;
_4 = [17083877591427288043_usize,7_usize,1_usize,2_usize,0_usize,7_usize];
_5 = _3;
_1.fld0 = [0_i8,(-43_i8)];
_3 = _5;
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_2 = -(-9223372036854775808_isize);
_1.fld1 = [RET,RET,RET,_7,RET,_7,_3];
place!(Field::<f64>(Variant(_10, 0), 0)) = (-110_i8) as f64;
_8 = 54002_u16 as f64;
_17 = -4450240046133269727_i64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [92_i8,119_i8];
RET = _3;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [(-4_i8),117_i8];
_8 = -Field::<f64>(Variant(_10, 0), 0);
Goto(bb8)
}
bb8 = {
_6.2 = [_9,_9,_9,_9];
_16 = !1808770374103272185_u64;
_8 = 3049_i16 as f64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [87_i8,(-78_i8)];
_2 = 9223372036854775807_isize | 35_isize;
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_13 = _8 as f32;
_13 = _9 as f32;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [41_i8,(-14_i8)];
_19 = &_1.fld1;
_5 = _7;
_1.fld0 = [(-4_i8),53_i8];
_17 = _11 as i64;
_1.fld1 = [RET,_5,_5,_3,_5,_7,_5];
_13 = 11083_i16 as f32;
_2 = 9223372036854775807_isize << _9;
_9 = -(-107240519420754955040396424597999438364_i128);
place!(Field::<f64>(Variant(_10, 0), 0)) = _8 + _8;
_11 = _8 as u32;
place!(Field::<f64>(Variant(_10, 0), 0)) = -_8;
_19 = &_1.fld1;
_17 = (-11948_i16) as i64;
RET = _7;
match _12 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb9,
5 => bb10,
6 => bb11,
56114910569755093029611643652582352163 => bb13,
_ => bb12
}
}
bb9 = {
_12 = 49690_u16 as u128;
_5 = _3;
_12 = 56114910569755093029611643652582352163_u128;
_1.fld1 = [_5,_5,RET,_5,_5,_3,_7];
place!(Field::<f64>(Variant(_10, 0), 0)) = 5988444194647357168_u64 as f64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [105_i8,(-108_i8)];
_1.fld1 = [RET,RET,_5,_7,_5,_7,_5];
_5 = _7;
_6.1 = [RET,_5,_3];
_3 = RET;
_13 = Field::<f64>(Variant(_10, 0), 0) as f32;
_9 = 16439067240727464861911206737606887588_i128;
_3 = _5;
_4 = [17083877591427288043_usize,7_usize,1_usize,2_usize,0_usize,7_usize];
_5 = _3;
_1.fld0 = [0_i8,(-43_i8)];
_3 = _5;
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_2 = -(-9223372036854775808_isize);
_1.fld1 = [RET,RET,RET,_7,RET,_7,_3];
place!(Field::<f64>(Variant(_10, 0), 0)) = (-110_i8) as f64;
_8 = 54002_u16 as f64;
_17 = -4450240046133269727_i64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [92_i8,119_i8];
RET = _3;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [(-4_i8),117_i8];
_8 = -Field::<f64>(Variant(_10, 0), 0);
Goto(bb8)
}
bb10 = {
_6.2 = [_9,_9,_9,_9];
_6.2 = [_9,_9,_9,_9];
_1.fld1 = [_7,_3,_7,RET,RET,RET,_3];
_4 = [491399320109861797_usize,7268257579879734951_usize,5_usize,15704088531428549351_usize,2588682914089542650_usize,4_usize];
_6.1 = [_7,_3,_5];
_1.fld0 = [(-46_i8),(-21_i8)];
RET = _3;
_1.fld1 = [_3,_5,_5,RET,_7,_3,RET];
_6.2 = [_9,_9,_9,_9];
_2 = -(-30_isize);
_1.fld1 = [RET,_5,_5,_7,_7,_7,_7];
RET = _5;
_5 = RET;
_2 = (-9223372036854775808_isize) ^ 112_isize;
_2 = (-69_isize);
_4 = [4_usize,6_usize,695054446494600944_usize,0_usize,6774523968093706114_usize,5_usize];
_7 = _5;
_2 = (-35_i8) as isize;
_11 = 3194960851_u32;
_2 = 101_isize & (-9223372036854775808_isize);
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_1.fld0 = [(-17_i8),99_i8];
_7 = _5;
_6.2 = [_9,_9,_9,_9];
RET = _7;
_8 = 13220849460833100923_usize as f64;
_5 = _7;
_6.2 = [_9,_9,_9,_9];
Goto(bb7)
}
bb11 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb12 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb13 = {
_23 = 26357_i16 as isize;
_11 = 4006758483_u32;
_2 = -_23;
_9 = 71292229252168547210142200076090810562_i128;
RET = _3;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [(-72_i8),(-8_i8)];
RET = _3;
_16 = 17791582725984499567_u64;
_17 = 5981546353119636441_i64;
_11 = 2922587733_u32;
_4 = [7090475828117387357_usize,5_usize,18188461042917136900_usize,7_usize,1_usize,4_usize];
_4 = [16137949414091009452_usize,1_usize,5110240525261356755_usize,17184329546634120657_usize,5893895848800342626_usize,13034254376679738872_usize];
_5 = _3;
_4 = [4722348239184585553_usize,6_usize,1_usize,2_usize,5_usize,17851049443753835556_usize];
_3 = RET;
_7 = _5;
_7 = RET;
RET = _5;
_23 = _2;
_4 = [2_usize,2_usize,5241458410445089793_usize,4_usize,6_usize,3_usize];
_2 = -_23;
_23 = _9 as isize;
_19 = &_1.fld1;
_16 = 8503143135421557434_u64 - 3284056750370316417_u64;
_19 = &(*_19);
_1.fld1 = [_3,_7,RET,_3,_5,_5,RET];
match _11 {
0 => bb1,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
2922587733 => bb19,
_ => bb18
}
}
bb14 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb15 = {
_6.2 = [_9,_9,_9,_9];
_1.fld0 = [119_i8,(-71_i8)];
_3 = _5;
_7 = _5;
_6.1 = [RET,_5,RET];
_1.fld0 = [(-35_i8),40_i8];
_4 = [2416092084885788051_usize,14255022600456668135_usize,13340331363400657271_usize,7781395573788582267_usize,6_usize,6_usize];
_4 = [9930966972785515502_usize,7_usize,6_usize,0_usize,3_usize,6_usize];
_3 = _7;
_6.2 = [_9,_9,_9,_9];
_9 = (-117589327329575466373866722080777369956_i128) ^ 85403535378739160440219026926216933785_i128;
_8 = _2 as f64;
RET = _3;
_9 = (-134037922031622529486494032309712789741_i128);
_8 = 200330735_u32 as f64;
Goto(bb3)
}
bb16 = {
RET = _7;
match _9 {
0 => bb2,
1 => bb4,
206244444889315933976880575122055421715 => bb6,
_ => bb5
}
}
bb17 = {
_12 = 49690_u16 as u128;
_5 = _3;
_12 = 56114910569755093029611643652582352163_u128;
_1.fld1 = [_5,_5,RET,_5,_5,_3,_7];
place!(Field::<f64>(Variant(_10, 0), 0)) = 5988444194647357168_u64 as f64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [105_i8,(-108_i8)];
_1.fld1 = [RET,RET,_5,_7,_5,_7,_5];
_5 = _7;
_6.1 = [RET,_5,_3];
_3 = RET;
_13 = Field::<f64>(Variant(_10, 0), 0) as f32;
_9 = 16439067240727464861911206737606887588_i128;
_3 = _5;
_4 = [17083877591427288043_usize,7_usize,1_usize,2_usize,0_usize,7_usize];
_5 = _3;
_1.fld0 = [0_i8,(-43_i8)];
_3 = _5;
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_2 = -(-9223372036854775808_isize);
_1.fld1 = [RET,RET,RET,_7,RET,_7,_3];
place!(Field::<f64>(Variant(_10, 0), 0)) = (-110_i8) as f64;
_8 = 54002_u16 as f64;
_17 = -4450240046133269727_i64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [92_i8,119_i8];
RET = _3;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [(-4_i8),117_i8];
_8 = -Field::<f64>(Variant(_10, 0), 0);
Goto(bb8)
}
bb18 = {
_6.2 = [_9,_9,_9,_9];
_16 = !1808770374103272185_u64;
_8 = 3049_i16 as f64;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [87_i8,(-78_i8)];
_2 = 9223372036854775807_isize | 35_isize;
_10 = Adt73::Variant0 { fld0: _8,fld1: _1.fld0 };
_13 = _8 as f32;
_13 = _9 as f32;
place!(Field::<[i8; 2]>(Variant(_10, 0), 1)) = [41_i8,(-14_i8)];
_19 = &_1.fld1;
_5 = _7;
_1.fld0 = [(-4_i8),53_i8];
_17 = _11 as i64;
_1.fld1 = [RET,_5,_5,_3,_5,_7,_5];
_13 = 11083_i16 as f32;
_2 = 9223372036854775807_isize << _9;
_9 = -(-107240519420754955040396424597999438364_i128);
place!(Field::<f64>(Variant(_10, 0), 0)) = _8 + _8;
_11 = _8 as u32;
place!(Field::<f64>(Variant(_10, 0), 0)) = -_8;
_19 = &_1.fld1;
_17 = (-11948_i16) as i64;
RET = _7;
match _12 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb9,
5 => bb10,
6 => bb11,
56114910569755093029611643652582352163 => bb13,
_ => bb12
}
}
bb19 = {
_5 = RET;
_19 = &_1.fld1;
RET = _5;
_24 = !true;
_7 = _3;
_16 = 18419644733040218442_u64;
_6.1 = [_5,RET,_7];
_4 = [7_usize,7_usize,14635628935709300778_usize,1_usize,10988723207614544682_usize,1_usize];
_9 = _11 as i128;
Goto(bb20)
}
bb20 = {
Call(_29 = dump_var(3_usize, 11_usize, Move(_11), 7_usize, Move(_7), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_29 = dump_var(3_usize, 5_usize, Move(_5), 9_usize, Move(_9), 30_usize, _30, 30_usize, _30), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: [i8; 2]) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _3: isize;
let _4: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _5: &'static Adt49;
let _6: isize;
let _7: isize;
let _8: isize;
let _9: &'static (i64, u64);
let _10: (u32, [u8; 8]);
let _11: *const i32;
let _12: &'static i32;
let _13: &'static i32;
let _14: Adt49;
let _15: (char,);
let _16: Adt44;
let _17: f32;
let _18: i64;
let _19: &'static *const (i32, u128, &'static bool);
let _20: Adt79;
let _21: i32;
let _22: u64;
let _23: u128;
let _24: f32;
let _25: *mut (i32, u128, &'static bool);
let _26: isize;
let _27: u32;
let _28: isize;
let _29: u32;
let _30: f32;
let _31: ();
let _32: ();
{
_2 = [109_i8,91_i8];
_1 = '\u{64cfc}';
_1 = '\u{d63e7}';
_3 = -(-9223372036854775808_isize);
Call(_2 = fn5(_1, _1, _1, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [2420194804929668945_usize,5159881260758591249_usize,2400472415915993089_usize,11211415949260964316_usize,6_usize,6_usize];
_1 = '\u{2ac1b}';
RET = [1_usize,13112167338783738641_usize,0_usize,8014999390782023676_usize,0_usize,7358664220959222099_usize];
RET = [7876696342347448625_usize,3507731516172997211_usize,3_usize,1054873241762394762_usize,11576604371186417535_usize,13141369433594467963_usize];
RET = [2_usize,6_usize,2_usize,2_usize,8402713329904714231_usize,9669089802671131369_usize];
_1 = '\u{7ebc8}';
_1 = '\u{c27a9}';
_1 = '\u{5e7e6}';
RET = [5_usize,2_usize,6_usize,15231346693211783240_usize,5_usize,1236923846520522300_usize];
_1 = '\u{3fd2a}';
RET = [14505232782280664294_usize,2562060870829943707_usize,3_usize,6_usize,2_usize,8995649516101859120_usize];
_1 = '\u{e9ad}';
_2 = [3_i8,(-106_i8)];
_3 = 30024_u16 as isize;
RET = [14725744353290275690_usize,8541267899870191088_usize,15738361134920633181_usize,17949495318524610126_usize,13367599649695348630_usize,6_usize];
RET = [7_usize,2_usize,5_usize,12596525562446314706_usize,10809389241727510188_usize,0_usize];
_2 = [(-39_i8),(-13_i8)];
_3 = 9223372036854775807_isize + (-9223372036854775808_isize);
_1 = '\u{e795f}';
RET = [0_usize,5_usize,15253512796953612165_usize,1_usize,4301623450961225740_usize,13728517415200419160_usize];
RET = [7489736376128174797_usize,1_usize,0_usize,17023334937489586414_usize,5192888355565016648_usize,0_usize];
_1 = '\u{32864}';
_2 = [8_i8,(-105_i8)];
RET = [3_usize,4_usize,4_usize,5_usize,3_usize,13817294677298221658_usize];
RET = [3_usize,0_usize,13357034786675725172_usize,3_usize,10852296539690562637_usize,2_usize];
_6 = 1791657608_u32 as isize;
Goto(bb2)
}
bb2 = {
RET = [3135492931018420381_usize,6754021085640523781_usize,3_usize,3_usize,1428581980096749801_usize,5_usize];
RET = [12051636101888321647_usize,6884277218373005524_usize,4_usize,7666695109230912612_usize,17390801215825245463_usize,7377180039448015597_usize];
_3 = !_6;
_3 = !_6;
_3 = _6;
_2 = [(-98_i8),(-21_i8)];
_2 = [28_i8,(-85_i8)];
_3 = 3800976100064638164_u64 as isize;
_6 = _3;
_1 = '\u{ceb00}';
_6 = 18318939920964996818_u64 as isize;
RET = [17232730441667641470_usize,6_usize,3285820574705891169_usize,3_usize,8452186917030338600_usize,16943691768375003810_usize];
_3 = _6;
_1 = '\u{9c05e}';
_3 = _6 & _6;
_3 = _6 + _6;
_2 = [(-11_i8),(-116_i8)];
_7 = _3 ^ _6;
_7 = -_3;
_3 = false as isize;
_6 = 690246699031923328_u64 as isize;
_3 = _7 | _6;
_1 = '\u{82ca8}';
Goto(bb3)
}
bb3 = {
_1 = '\u{e8d7f}';
_3 = _7;
_1 = '\u{80827}';
_1 = '\u{789d8}';
_10.0 = 4583024569273472643_i64 as u32;
RET = [1691965198728566433_usize,7_usize,12884770227550291217_usize,2_usize,5_usize,5_usize];
_10.1 = [147_u8,58_u8,189_u8,15_u8,135_u8,37_u8,94_u8,220_u8];
_2 = [(-107_i8),73_i8];
_10.1 = [188_u8,226_u8,35_u8,228_u8,207_u8,104_u8,135_u8,22_u8];
_3 = _7;
_2 = [(-52_i8),86_i8];
_2 = [74_i8,(-70_i8)];
_1 = '\u{8a4e4}';
_8 = _6;
_2 = [(-36_i8),(-104_i8)];
_2 = [40_i8,(-44_i8)];
RET = [3_usize,4202583278293687811_usize,7_usize,289598147794745635_usize,2_usize,3_usize];
RET = [0_usize,4_usize,3_usize,4_usize,4_usize,5819608185111152023_usize];
RET = [3_usize,3_usize,5_usize,6_usize,3_usize,9109638706199552113_usize];
_7 = _3;
_2 = [60_i8,(-20_i8)];
_6 = 63837_u16 as isize;
_5 = &_14;
_7 = -_6;
_6 = _8 ^ _8;
_14 = Adt49::Variant1 { fld0: (-8144709905051733076_i64) };
Goto(bb4)
}
bb4 = {
_7 = -_3;
place!(Field::<i64>(Variant(_14, 1), 0)) = 6641121021179502037_i64;
place!(Field::<i64>(Variant(_14, 1), 0)) = 6378141638605171033_i64 - 1894252371910855305_i64;
SetDiscriminant(_14, 1);
Goto(bb5)
}
bb5 = {
_10.0 = 1691339262_u32 - 2648928944_u32;
RET = [2_usize,4_usize,4_usize,6_usize,4_usize,7_usize];
_5 = &_14;
_10.1 = [33_u8,38_u8,226_u8,127_u8,113_u8,171_u8,237_u8,124_u8];
place!(Field::<i64>(Variant(_14, 1), 0)) = _10.0 as i64;
_1 = '\u{16a73}';
_10.0 = !3170468455_u32;
place!(Field::<i64>(Variant(_14, 1), 0)) = _10.0 as i64;
RET = [14914256289558166234_usize,8668890315850212555_usize,3_usize,8612514247621660602_usize,3_usize,7116384649644335860_usize];
RET = [1_usize,9282471214554068748_usize,6_usize,6_usize,6_usize,4_usize];
_15 = (_1,);
_6 = true as isize;
_3 = !_8;
RET = [12245071565610659730_usize,7_usize,15244863009274707756_usize,9084868307714052619_usize,1_usize,1_usize];
_1 = _15.0;
RET = [6_usize,5_usize,2640945471449413584_usize,2_usize,3_usize,7_usize];
_8 = _7 & _7;
_14 = Adt49::Variant1 { fld0: (-2661454851789702902_i64) };
_1 = _15.0;
_7 = -_8;
_15.0 = _1;
_1 = _15.0;
_14 = Adt49::Variant1 { fld0: 2331577128289824999_i64 };
_14 = Adt49::Variant1 { fld0: (-5978494775591457822_i64) };
_6 = _7;
place!(Field::<i64>(Variant(_14, 1), 0)) = !4438799791350497619_i64;
Goto(bb6)
}
bb6 = {
_20.fld6.1 = true;
_1 = _15.0;
_15.0 = _1;
_6 = _7;
_20.fld5 = [_15.0,_1,_15.0,_15.0,_1,_1,_15.0];
_21 = 1310557656_i32;
_11 = core::ptr::addr_of!(_21);
_20.fld6.3 = _10.0 as isize;
_22 = !9644728450330035221_u64;
_12 = &(*_11);
_20.fld1 = _1;
_20.fld2 = Move(_11);
_23 = 75155195523907201166336324010866948843_u128;
_2 = [24_i8,127_i8];
_15.0 = _1;
RET = [80087838790774193_usize,17753613599933819922_usize,6_usize,8553206967724320811_usize,16904380557321899681_usize,15739598493752037508_usize];
_21 = !(-1772526625_i32);
Goto(bb7)
}
bb7 = {
_15.0 = _1;
_10.0 = 2200818326_u32 << Field::<i64>(Variant(_14, 1), 0);
place!(Field::<i64>(Variant(_14, 1), 0)) = (-8195146785135699380_i64);
_5 = &_14;
RET = [5_usize,3_usize,11940226956094472601_usize,15363289508374852719_usize,4_usize,7_usize];
_7 = _10.0 as isize;
_18 = Field::<i64>(Variant(_14, 1), 0);
_24 = (-18_i8) as f32;
_2 = [(-11_i8),21_i8];
Goto(bb8)
}
bb8 = {
_18 = 129_u8 as i64;
_10.0 = _21 as u32;
_26 = !_7;
_24 = _23 as f32;
_24 = (-122464848327812603706124322120082216127_i128) as f32;
_20.fld6.2 = [_1,_1,_1,_20.fld1,_1,_1,_20.fld1];
_20.fld5 = _20.fld6.2;
_27 = _10.0;
_29 = _27;
_20.fld0 = [12_i8,(-53_i8),105_i8,(-16_i8),31_i8,(-127_i8),81_i8,(-68_i8)];
_13 = &_21;
_20.fld1 = _1;
place!(Field::<i64>(Variant(_14, 1), 0)) = !_18;
match _23 {
0 => bb9,
1 => bb10,
2 => bb11,
75155195523907201166336324010866948843 => bb13,
_ => bb12
}
}
bb9 = {
_15.0 = _1;
_10.0 = 2200818326_u32 << Field::<i64>(Variant(_14, 1), 0);
place!(Field::<i64>(Variant(_14, 1), 0)) = (-8195146785135699380_i64);
_5 = &_14;
RET = [5_usize,3_usize,11940226956094472601_usize,15363289508374852719_usize,4_usize,7_usize];
_7 = _10.0 as isize;
_18 = Field::<i64>(Variant(_14, 1), 0);
_24 = (-18_i8) as f32;
_2 = [(-11_i8),21_i8];
Goto(bb8)
}
bb10 = {
_1 = '\u{e8d7f}';
_3 = _7;
_1 = '\u{80827}';
_1 = '\u{789d8}';
_10.0 = 4583024569273472643_i64 as u32;
RET = [1691965198728566433_usize,7_usize,12884770227550291217_usize,2_usize,5_usize,5_usize];
_10.1 = [147_u8,58_u8,189_u8,15_u8,135_u8,37_u8,94_u8,220_u8];
_2 = [(-107_i8),73_i8];
_10.1 = [188_u8,226_u8,35_u8,228_u8,207_u8,104_u8,135_u8,22_u8];
_3 = _7;
_2 = [(-52_i8),86_i8];
_2 = [74_i8,(-70_i8)];
_1 = '\u{8a4e4}';
_8 = _6;
_2 = [(-36_i8),(-104_i8)];
_2 = [40_i8,(-44_i8)];
RET = [3_usize,4202583278293687811_usize,7_usize,289598147794745635_usize,2_usize,3_usize];
RET = [0_usize,4_usize,3_usize,4_usize,4_usize,5819608185111152023_usize];
RET = [3_usize,3_usize,5_usize,6_usize,3_usize,9109638706199552113_usize];
_7 = _3;
_2 = [60_i8,(-20_i8)];
_6 = 63837_u16 as isize;
_5 = &_14;
_7 = -_6;
_6 = _8 ^ _8;
_14 = Adt49::Variant1 { fld0: (-8144709905051733076_i64) };
Goto(bb4)
}
bb11 = {
_10.0 = 1691339262_u32 - 2648928944_u32;
RET = [2_usize,4_usize,4_usize,6_usize,4_usize,7_usize];
_5 = &_14;
_10.1 = [33_u8,38_u8,226_u8,127_u8,113_u8,171_u8,237_u8,124_u8];
place!(Field::<i64>(Variant(_14, 1), 0)) = _10.0 as i64;
_1 = '\u{16a73}';
_10.0 = !3170468455_u32;
place!(Field::<i64>(Variant(_14, 1), 0)) = _10.0 as i64;
RET = [14914256289558166234_usize,8668890315850212555_usize,3_usize,8612514247621660602_usize,3_usize,7116384649644335860_usize];
RET = [1_usize,9282471214554068748_usize,6_usize,6_usize,6_usize,4_usize];
_15 = (_1,);
_6 = true as isize;
_3 = !_8;
RET = [12245071565610659730_usize,7_usize,15244863009274707756_usize,9084868307714052619_usize,1_usize,1_usize];
_1 = _15.0;
RET = [6_usize,5_usize,2640945471449413584_usize,2_usize,3_usize,7_usize];
_8 = _7 & _7;
_14 = Adt49::Variant1 { fld0: (-2661454851789702902_i64) };
_1 = _15.0;
_7 = -_8;
_15.0 = _1;
_1 = _15.0;
_14 = Adt49::Variant1 { fld0: 2331577128289824999_i64 };
_14 = Adt49::Variant1 { fld0: (-5978494775591457822_i64) };
_6 = _7;
place!(Field::<i64>(Variant(_14, 1), 0)) = !4438799791350497619_i64;
Goto(bb6)
}
bb12 = {
RET = [2420194804929668945_usize,5159881260758591249_usize,2400472415915993089_usize,11211415949260964316_usize,6_usize,6_usize];
_1 = '\u{2ac1b}';
RET = [1_usize,13112167338783738641_usize,0_usize,8014999390782023676_usize,0_usize,7358664220959222099_usize];
RET = [7876696342347448625_usize,3507731516172997211_usize,3_usize,1054873241762394762_usize,11576604371186417535_usize,13141369433594467963_usize];
RET = [2_usize,6_usize,2_usize,2_usize,8402713329904714231_usize,9669089802671131369_usize];
_1 = '\u{7ebc8}';
_1 = '\u{c27a9}';
_1 = '\u{5e7e6}';
RET = [5_usize,2_usize,6_usize,15231346693211783240_usize,5_usize,1236923846520522300_usize];
_1 = '\u{3fd2a}';
RET = [14505232782280664294_usize,2562060870829943707_usize,3_usize,6_usize,2_usize,8995649516101859120_usize];
_1 = '\u{e9ad}';
_2 = [3_i8,(-106_i8)];
_3 = 30024_u16 as isize;
RET = [14725744353290275690_usize,8541267899870191088_usize,15738361134920633181_usize,17949495318524610126_usize,13367599649695348630_usize,6_usize];
RET = [7_usize,2_usize,5_usize,12596525562446314706_usize,10809389241727510188_usize,0_usize];
_2 = [(-39_i8),(-13_i8)];
_3 = 9223372036854775807_isize + (-9223372036854775808_isize);
_1 = '\u{e795f}';
RET = [0_usize,5_usize,15253512796953612165_usize,1_usize,4301623450961225740_usize,13728517415200419160_usize];
RET = [7489736376128174797_usize,1_usize,0_usize,17023334937489586414_usize,5192888355565016648_usize,0_usize];
_1 = '\u{32864}';
_2 = [8_i8,(-105_i8)];
RET = [3_usize,4_usize,4_usize,5_usize,3_usize,13817294677298221658_usize];
RET = [3_usize,0_usize,13357034786675725172_usize,3_usize,10852296539690562637_usize,2_usize];
_6 = 1791657608_u32 as isize;
Goto(bb2)
}
bb13 = {
SetDiscriminant(_14, 1);
_20.fld4 = [_1,_15.0,_1,_15.0,_1];
place!(Field::<i64>(Variant(_14, 1), 0)) = _18;
_5 = &_14;
_15 = (_1,);
place!(Field::<i64>(Variant(_14, 1), 0)) = -_18;
Goto(bb14)
}
bb14 = {
_6 = _3 << _27;
_12 = &(*_13);
RET = [7_usize,2734510927493943782_usize,4622271277629460996_usize,15314583117869189850_usize,16480112820682428982_usize,7884305161099239909_usize];
_26 = _7 >> _10.0;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(4_usize, 27_usize, Move(_27), 10_usize, Move(_10), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 21_usize, Move(_21), 22_usize, Move(_22), 29_usize, Move(_29), 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char,mut _2: char,mut _3: char,mut _4: isize,mut _5: isize) -> [i8; 2] {
mir! {
type RET = [i8; 2];
let _6: isize;
let _7: &'static bool;
let _8: [i8; 8];
let _9: char;
let _10: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _11: i64;
let _12: [bool; 3];
let _13: [usize; 7];
let _14: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _15: *const &'static u32;
let _16: i128;
let _17: [isize; 4];
let _18: usize;
let _19: *const i32;
let _20: f64;
let _21: [char; 5];
let _22: [char; 5];
let _23: ([i128; 3], bool, [char; 7], isize);
let _24: (i64, u64);
let _25: (u32, [u8; 8]);
let _26: (*const (i32, u128, &'static bool), &'static (f64,), i32);
let _27: *mut i128;
let _28: f64;
let _29: [i8; 2];
let _30: char;
let _31: *mut u64;
let _32: char;
let _33: (i32, u128, &'static bool);
let _34: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _35: &'static Adt49;
let _36: isize;
let _37: isize;
let _38: [u32; 7];
let _39: &'static *mut i128;
let _40: ();
let _41: ();
{
_4 = true as isize;
_1 = _3;
_4 = (-123_i8) as isize;
RET = [27_i8,3_i8];
RET = [86_i8,92_i8];
_4 = _5 & _5;
_1 = _2;
_2 = _3;
_4 = _5;
RET = [(-74_i8),(-53_i8)];
_3 = _2;
_2 = _3;
Goto(bb1)
}
bb1 = {
_6 = -_4;
_1 = _2;
_1 = _2;
_6 = 38506_u16 as isize;
_6 = 181_u8 as isize;
_3 = _1;
_8 = [64_i8,(-26_i8),41_i8,(-49_i8),(-1_i8),(-43_i8),(-77_i8),47_i8];
_2 = _1;
_1 = _2;
_5 = _4 & _4;
_3 = _2;
_4 = _5 - _5;
RET = [21_i8,49_i8];
_3 = _2;
_4 = _6 - _5;
_9 = _3;
Goto(bb2)
}
bb2 = {
_6 = _5 + _4;
_2 = _3;
_4 = _5;
_3 = _9;
_8 = [(-77_i8),(-37_i8),93_i8,63_i8,124_i8,(-13_i8),109_i8,(-39_i8)];
Call(_4 = fn6(_6, RET, _8, _6, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [112_i8,44_i8];
_5 = -_4;
_3 = _9;
_5 = _6;
RET = [4_i8,3_i8];
RET = [(-17_i8),9_i8];
_4 = (-35_i8) as isize;
_4 = _6 >> _5;
_8 = [(-111_i8),(-31_i8),79_i8,15_i8,(-104_i8),(-4_i8),(-5_i8),85_i8];
_6 = _4;
RET = [122_i8,(-110_i8)];
_2 = _3;
_8 = [(-78_i8),19_i8,(-43_i8),3_i8,(-109_i8),62_i8,52_i8,(-10_i8)];
_1 = _9;
Goto(bb4)
}
bb4 = {
_3 = _2;
_4 = _5;
RET = [(-57_i8),(-118_i8)];
_8 = [120_i8,95_i8,(-20_i8),17_i8,115_i8,103_i8,(-77_i8),(-31_i8)];
_6 = -_4;
RET = [85_i8,(-46_i8)];
_2 = _3;
_5 = _6 ^ _4;
_4 = _6;
_4 = (-1137341520_i32) as isize;
_6 = _5;
Goto(bb5)
}
bb5 = {
_12 = [false,true,true];
_9 = _1;
_5 = _1 as isize;
_2 = _3;
RET = [69_i8,(-81_i8)];
_5 = !_6;
_2 = _9;
_3 = _2;
_11 = 432794847_u32 as i64;
_13 = [3_usize,13122818266455486327_usize,7681667778435941656_usize,10190738076133265131_usize,9829317656437790666_usize,9913209847771437099_usize,0_usize];
_2 = _9;
_9 = _2;
_8 = [(-75_i8),(-57_i8),83_i8,44_i8,(-9_i8),(-121_i8),87_i8,85_i8];
RET = [98_i8,(-117_i8)];
RET = [97_i8,(-103_i8)];
_4 = _5 | _6;
RET = [84_i8,111_i8];
_2 = _3;
RET = [(-6_i8),2_i8];
_16 = -115110477510904967081426908030972227490_i128;
RET = [(-124_i8),(-94_i8)];
_8 = [102_i8,85_i8,6_i8,(-27_i8),68_i8,(-117_i8),(-97_i8),(-124_i8)];
_8 = [(-34_i8),34_i8,51_i8,37_i8,(-122_i8),31_i8,103_i8,51_i8];
RET = [(-77_i8),50_i8];
Goto(bb6)
}
bb6 = {
_6 = _4;
_8 = [68_i8,69_i8,120_i8,(-93_i8),119_i8,75_i8,22_i8,32_i8];
Goto(bb7)
}
bb7 = {
_5 = _6 << _11;
_18 = !748190310822416293_usize;
_16 = (-124184368670932292051875464550337453043_i128) + 130618005542590946681400514894712225376_i128;
_23.2 = [_3,_2,_2,_3,_1,_3,_2];
_20 = 203_u8 as f64;
_22 = [_9,_1,_9,_2,_3];
_23.2 = [_1,_1,_1,_3,_2,_9,_3];
_1 = _9;
Goto(bb8)
}
bb8 = {
_23.3 = _4 + _4;
_5 = -_23.3;
Goto(bb9)
}
bb9 = {
_23.3 = _4 | _5;
_13 = [_18,_18,_18,_18,_18,_18,_18];
_24 = (_11, 5127431287882863859_u64);
_6 = !_23.3;
RET = [(-100_i8),124_i8];
_2 = _1;
_16 = 110712955047992944158038193462757709029_i128;
_2 = _3;
_21 = [_3,_3,_1,_3,_1];
_18 = 5312725722214231328_usize;
_19 = core::ptr::addr_of!(_26.2);
_24.0 = _23.3 as i64;
_26.2 = (-120_i8) as i32;
_13 = [_18,_18,_18,_18,_18,_18,_18];
_23.3 = !_4;
_27 = core::ptr::addr_of_mut!(_16);
_18 = 18335419247786349727_usize;
_22 = _21;
_28 = _20;
_23.3 = _6;
match _16 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb10,
110712955047992944158038193462757709029 => bb12,
_ => bb11
}
}
bb10 = {
RET = [112_i8,44_i8];
_5 = -_4;
_3 = _9;
_5 = _6;
RET = [4_i8,3_i8];
RET = [(-17_i8),9_i8];
_4 = (-35_i8) as isize;
_4 = _6 >> _5;
_8 = [(-111_i8),(-31_i8),79_i8,15_i8,(-104_i8),(-4_i8),(-5_i8),85_i8];
_6 = _4;
RET = [122_i8,(-110_i8)];
_2 = _3;
_8 = [(-78_i8),19_i8,(-43_i8),3_i8,(-109_i8),62_i8,52_i8,(-10_i8)];
_1 = _9;
Goto(bb4)
}
bb11 = {
_6 = _4;
_8 = [68_i8,69_i8,120_i8,(-93_i8),119_i8,75_i8,22_i8,32_i8];
Goto(bb7)
}
bb12 = {
(*_27) = _24.1 as i128;
(*_19) = !683746816_i32;
_24.0 = _11;
_6 = _23.3;
RET = [16_i8,(-56_i8)];
_20 = _28;
_23.0 = [(*_27),(*_27),_16];
_29 = [118_i8,73_i8];
_7 = &_23.1;
_25.1 = [111_u8,133_u8,57_u8,184_u8,191_u8,24_u8,211_u8,105_u8];
_23.1 = !false;
_7 = &_23.1;
_25.1 = [100_u8,156_u8,146_u8,78_u8,200_u8,129_u8,175_u8,158_u8];
_6 = -_23.3;
_25.0 = !73775262_u32;
_18 = _24.1 as usize;
_9 = _1;
_5 = _4;
_28 = _26.2 as f64;
_29 = [(-4_i8),14_i8];
_17 = [_6,_23.3,_4,_6];
_19 = core::ptr::addr_of!((*_19));
_30 = _9;
(*_27) = _24.1 as i128;
_7 = &(*_7);
_33.0 = (*_19) << _6;
Call((*_19) = core::intrinsics::bswap(_33.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = [(-124_i8),(-41_i8)];
(*_19) = _33.0 - _33.0;
_5 = (*_7) as isize;
_23.0 = [_16,(*_27),_16];
_4 = 30001_i16 as isize;
_26.2 = _33.0;
_5 = _23.3;
_25.0 = 435446856_u32;
_11 = -_24.0;
_33.2 = &_23.1;
_36 = 10923_u16 as isize;
_24.0 = -_11;
(*_19) = _33.0;
match _25.0 {
0 => bb1,
1 => bb2,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb6,
435446856 => bb14,
_ => bb11
}
}
bb14 = {
_3 = _2;
(*_27) = -144619148232739368305444374991452035117_i128;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(5_usize, 13_usize, Move(_13), 3_usize, Move(_3), 18_usize, Move(_18), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(5_usize, 30_usize, Move(_30), 36_usize, Move(_36), 23_usize, Move(_23), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(5_usize, 12_usize, Move(_12), 17_usize, Move(_17), 5_usize, Move(_5), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: [i8; 2],mut _3: [i8; 8],mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: &'static (i64, u64);
let _7: *mut u16;
let _8: f64;
let _9: [char; 7];
let _10: &'static *const (i32, u128, &'static bool);
let _11: char;
let _12: isize;
let _13: u128;
let _14: u128;
let _15: f32;
let _16: f64;
let _17: bool;
let _18: usize;
let _19: [i16; 5];
let _20: bool;
let _21: u16;
let _22: i16;
let _23: &'static usize;
let _24: i8;
let _25: usize;
let _26: i64;
let _27: bool;
let _28: bool;
let _29: ();
let _30: ();
{
_5 = _4 - _1;
RET = 22206_i16 as isize;
RET = _5;
_1 = RET;
_3 = [41_i8,(-40_i8),114_i8,(-49_i8),(-87_i8),82_i8,(-105_i8),116_i8];
_5 = -RET;
_2 = [12_i8,86_i8];
RET = _5;
_1 = 96_u8 as isize;
_5 = true as isize;
_4 = RET;
_4 = RET & RET;
_4 = (-51_i8) as isize;
_2 = [(-86_i8),106_i8];
_2 = [(-48_i8),(-86_i8)];
_8 = 159124337612424375098250326561105232166_u128 as f64;
_5 = _1 & _4;
_4 = _5;
_9 = ['\u{493a4}','\u{10e72e}','\u{ee474}','\u{22dcc}','\u{b7356}','\u{106e10}','\u{9fc1d}'];
_4 = RET;
Goto(bb1)
}
bb1 = {
_5 = 248913395_i32 as isize;
_3 = [(-15_i8),(-99_i8),(-39_i8),63_i8,59_i8,109_i8,81_i8,82_i8];
RET = _4;
RET = -_5;
RET = _4 << _4;
RET = _1;
_1 = true as isize;
RET = 15344956948860166400_usize as isize;
_3 = [56_i8,(-7_i8),110_i8,(-29_i8),98_i8,108_i8,(-89_i8),35_i8];
_13 = 148965474669133715766627121645647784458_u128 & 293651754487817782694971250093680906254_u128;
_11 = '\u{9a6b}';
_4 = true as isize;
_14 = true as u128;
_9 = [_11,_11,_11,_11,_11,_11,_11];
_9 = [_11,_11,_11,_11,_11,_11,_11];
_8 = 5375999614238673657509390919058461631_i128 as f64;
_2 = [19_i8,(-113_i8)];
_2 = [(-31_i8),123_i8];
RET = (-4376_i16) as isize;
_11 = '\u{86676}';
_1 = 48416_u16 as isize;
RET = -_5;
_1 = RET & _5;
_2 = [34_i8,18_i8];
_15 = 3716461295_u32 as f32;
Goto(bb2)
}
bb2 = {
_15 = (-71543444965372412115260196635659606384_i128) as f32;
_8 = (-7567220475697383200_i64) as f64;
_13 = !_14;
_9 = [_11,_11,_11,_11,_11,_11,_11];
_15 = 1970417442020539946_u64 as f32;
_1 = -RET;
RET = _4;
_14 = _13 * _13;
RET = _14 as isize;
_12 = -_1;
_1 = _12;
_11 = '\u{357a}';
_3 = [56_i8,(-51_i8),(-100_i8),20_i8,17_i8,89_i8,122_i8,(-128_i8)];
Goto(bb3)
}
bb3 = {
_17 = RET < _5;
_3 = [(-8_i8),(-125_i8),(-111_i8),103_i8,(-99_i8),76_i8,(-44_i8),72_i8];
_11 = '\u{e0d76}';
_13 = 92_u8 as u128;
_16 = 62635373_u32 as f64;
RET = _15 as isize;
RET = _4 & _1;
_9 = [_11,_11,_11,_11,_11,_11,_11];
_16 = _8 * _8;
_13 = !_14;
_4 = _12;
_16 = _8;
_3 = [(-84_i8),91_i8,(-98_i8),(-6_i8),112_i8,(-119_i8),82_i8,(-54_i8)];
RET = _12 >> _4;
_14 = !_13;
_18 = 1899198822_i32 as usize;
_17 = true;
_1 = _5 ^ _4;
_11 = '\u{e45a8}';
_17 = true;
_9 = [_11,_11,_11,_11,_11,_11,_11];
_12 = _1 >> _1;
_7 = core::ptr::addr_of_mut!(_21);
_20 = _17;
_19 = [24113_i16,(-12408_i16),(-4106_i16),13830_i16,(-157_i16)];
Call(_4 = fn7(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_7) = 12689_u16 + 26545_u16;
_8 = -_16;
_17 = _4 != _12;
_9 = [_11,_11,_11,_11,_11,_11,_11];
_9 = [_11,_11,_11,_11,_11,_11,_11];
_1 = _4 * _4;
_3 = [(-53_i8),90_i8,11_i8,(-77_i8),(-105_i8),97_i8,78_i8,(-93_i8)];
_3 = [30_i8,110_i8,(-66_i8),53_i8,59_i8,122_i8,69_i8,75_i8];
_21 = _11 as u16;
(*_7) = (-7412515998773597864_i64) as u16;
_12 = 178_u8 as isize;
_4 = -_1;
_21 = 61447_u16;
(*_7) = _18 as u16;
_24 = -(-11_i8);
_9 = [_11,_11,_11,_11,_11,_11,_11];
RET = !_1;
_16 = 2120015133_i32 as f64;
_22 = _11 as i16;
_20 = !_17;
_14 = !_13;
_17 = _20;
_4 = -_1;
_27 = !_20;
Goto(bb5)
}
bb5 = {
Call(_29 = dump_var(6_usize, 2_usize, Move(_2), 3_usize, Move(_3), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_29 = dump_var(6_usize, 18_usize, Move(_18), 11_usize, Move(_11), 19_usize, Move(_19), 21_usize, Move(_21)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_29 = dump_var(6_usize, 24_usize, Move(_24), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7() -> isize {
mir! {
type RET = isize;
let _1: isize;
let _2: ([i128; 3], bool, [char; 7], isize);
let _3: f32;
let _4: [u32; 7];
let _5: [bool; 3];
let _6: i8;
let _7: ();
let _8: ();
{
RET = (-13_isize);
RET = '\u{10161f}' as isize;
RET = '\u{cfe58}' as isize;
RET = (-9223372036854775808_isize) << 60_isize;
RET = 9223372036854775807_isize & 9223372036854775807_isize;
RET = (-100_isize);
RET = '\u{68b08}' as isize;
RET = -(-9223372036854775808_isize);
RET = -9223372036854775807_isize;
RET = (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
RET = !9223372036854775807_isize;
RET = (-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
_2.1 = !true;
_2.1 = false | false;
_2.1 = !true;
_2.0 = [(-146141011564860435477788636107009554564_i128),140830458797989909437541386977679269204_i128,(-94156427117383451353265996078537780626_i128)];
_2.3 = -RET;
_2.0 = [(-167031738997913879079657702252060121483_i128),(-140731523663674511234082918168941677690_i128),(-72571071745113174285828836363570874638_i128)];
Call(_1 = fn8(RET, RET, _2.0, _2.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = 34230_u16 as f32;
_2.2 = ['\u{6ea1}','\u{9ab7c}','\u{ce55b}','\u{b4fb}','\u{d0597}','\u{714ae}','\u{1f673}'];
_2.2 = ['\u{a3e4d}','\u{44a0a}','\u{53ede}','\u{a5d4}','\u{46ecd}','\u{9a8df}','\u{1075ba}'];
_2.3 = -_1;
_4 = [200882537_u32,1688686888_u32,3928307294_u32,4201986200_u32,4268233504_u32,2809052066_u32,2474812749_u32];
_2.2 = ['\u{8d764}','\u{2022e}','\u{e6a61}','\u{346d7}','\u{2723}','\u{c8fc5}','\u{539f3}'];
_2.2 = ['\u{c8343}','\u{d5e65}','\u{cc23a}','\u{33f14}','\u{f39a7}','\u{4340f}','\u{5617d}'];
_4 = [615026056_u32,1179748008_u32,1424352066_u32,3897448924_u32,183540025_u32,3500252650_u32,3206629654_u32];
_4 = [962268408_u32,349495359_u32,874542667_u32,2728166107_u32,3043563545_u32,664648215_u32,2509277021_u32];
_2.3 = -_1;
_1 = 17786_u16 as isize;
_3 = 201827912014597109650034080150581696748_u128 as f32;
RET = -_2.3;
_5 = [_2.1,_2.1,_2.1];
Goto(bb3)
}
bb3 = {
Call(_7 = dump_var(7_usize, 1_usize, Move(_1), 5_usize, Move(_5), 8_usize, _8, 8_usize, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: [i128; 3],mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: i32;
let _6: f64;
let _7: *const *mut (i64, i8, *const u64, [char; 8]);
let _8: [u32; 7];
let _9: isize;
let _10: bool;
let _11: i64;
let _12: (*mut *const char, &'static [char; 7], [isize; 4], &'static i32);
let _13: (*const &'static u32, [char; 3], [i128; 4], &'static u16);
let _14: *const (i32, u128, &'static bool);
let _15: [u8; 8];
let _16: i8;
let _17: [usize; 7];
let _18: *const &'static u32;
let _19: i16;
let _20: char;
let _21: usize;
let _22: isize;
let _23: *mut u64;
let _24: isize;
let _25: (u32, [u8; 8]);
let _26: *mut u16;
let _27: ();
let _28: ();
{
_4 = _1;
_3 = [(-4724812438036198696894813582801227303_i128),(-168163372491975486709453169280146136534_i128),147886041253880658630306000573387084418_i128];
_3 = [70226220479433277206152024126641191931_i128,(-37708784807184874198743149630511540515_i128),(-82548457502294269941601620672183562717_i128)];
_5 = 54936_u16 as i32;
RET = _2;
_2 = !RET;
_2 = RET;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_1 = -_2;
_5 = (-1861866457_i32);
_5 = -(-217326303_i32);
_5 = (-1902085403_i32);
_3 = [140579498147288950146402365535696170933_i128,7785510045292711867871607782996251153_i128,67505837024477795365909257089502485745_i128];
RET = _4 << _1;
_3 = [(-79578291388619481341123934775572291687_i128),(-4852682145701930124183190949473048127_i128),(-142985389904247006051624599381338211043_i128)];
_3 = [(-87311748712426657940639337329022746630_i128),167750525818819058638917115478846544956_i128,(-77583521604849267395516206215931035517_i128)];
_3 = [66739374905696783468526164246293397022_i128,54506822150688011896735283535616511182_i128,(-161475937211775235914304353827119196787_i128)];
_6 = 4780157511363207949_usize as f64;
_2 = (-63_i8) as isize;
_6 = 7_usize as f64;
RET = -_4;
_3 = [130812893555513788394658851941294165063_i128,(-65785274667370494960817951069309244074_i128),64018940109065395036860045188546402949_i128];
_3 = [(-69239128410093746615665450024091171170_i128),(-115039708207512689869765946054826895868_i128),(-89610921269004943036711538039082269554_i128)];
_2 = RET << _1;
Goto(bb8)
}
bb8 = {
_8 = [1640935566_u32,3564799832_u32,3596485826_u32,224898299_u32,1514247867_u32,2989103668_u32,2746329760_u32];
Goto(bb9)
}
bb9 = {
_1 = RET;
_6 = 9154_u16 as f64;
_1 = !RET;
_2 = -_4;
_4 = _1;
_8 = [324232153_u32,3652005586_u32,4269240235_u32,2637173520_u32,225510211_u32,515919106_u32,664221788_u32];
RET = _2;
_5 = (-525642057_i32) << RET;
Goto(bb10)
}
bb10 = {
_2 = -_1;
_13.2 = [167202030129998316238308617859859359954_i128,(-86322672570246546124304087376564404358_i128),(-83595488692674048000273555035904604761_i128),160083644488564711684018236910032588150_i128];
_12.2 = [RET,RET,RET,_2];
Goto(bb11)
}
bb11 = {
_2 = _1 * _4;
_13.1 = ['\u{18c2e}','\u{a67d3}','\u{81fa3}'];
_11 = '\u{440dc}' as i64;
RET = _2 * _2;
_12.3 = &_5;
_11 = !(-3194997406629952727_i64);
Call(_6 = fn9(Move(_12.3), _2, _11, RET), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_4 = _1;
_4 = _2 | _2;
_9 = 1732662879009551991_u64 as isize;
_9 = _4 - _2;
_12.2 = [_9,_9,_9,_9];
RET = _4;
_10 = false;
_13.2 = [(-133376247837451312786208978708787262477_i128),(-55747801704618211661828501651232954281_i128),(-5796284918800639449112340924332104149_i128),(-23816722336722089003698371789860857310_i128)];
_12.3 = &_5;
_5 = (-1127181120_i32);
_13.2 = [(-38971304799397933114801073185629101391_i128),151088290111097809886872275907018723639_i128,(-82033917927960545589887291961755989967_i128),(-25313501277143469462691968357153859041_i128)];
RET = 2058823566_u32 as isize;
_17 = [12763140842473870176_usize,0_usize,1_usize,4343175956118364078_usize,3_usize,11635135061925125922_usize,7526907516135056336_usize];
_16 = 18546665390970851116803956518525479312_i128 as i8;
_9 = !_4;
RET = !_2;
_17 = [3_usize,7_usize,5932773565008194123_usize,8893359834620869653_usize,2_usize,6_usize,17334538922411430098_usize];
_4 = _1 ^ _9;
_13.1 = ['\u{249e6}','\u{10c46}','\u{c91eb}'];
_3 = [(-130488089404663617121509122615898726043_i128),(-109694582481574176432183637428045739568_i128),(-135392266554944993502159524553756398301_i128)];
_5 = 4_usize as i32;
_13.1 = ['\u{79699}','\u{b960e}','\u{cd82e}'];
_16 = _11 as i8;
_9 = _10 as isize;
RET = _1;
_8 = [1927696782_u32,3198986669_u32,2127697342_u32,543175586_u32,907181976_u32,2798141685_u32,2756529034_u32];
Goto(bb13)
}
bb13 = {
_19 = !15587_i16;
_20 = '\u{57973}';
_13.1 = [_20,_20,_20];
RET = _5 as isize;
_9 = 215417063208879014176317590220711793990_u128 as isize;
_4 = _2 - _1;
_1 = !_2;
_1 = _4 ^ _2;
_19 = _20 as i16;
_15 = [153_u8,29_u8,42_u8,61_u8,121_u8,226_u8,90_u8,163_u8];
_11 = (-8266999788894319627_i64) & (-2770780858538234724_i64);
_17 = [7_usize,7_usize,7_usize,9749555329259753776_usize,2_usize,1468067416779509268_usize,4_usize];
_8 = [2140912426_u32,738989885_u32,1399108456_u32,1750387101_u32,2036679533_u32,3778589158_u32,431637282_u32];
_2 = 27536882477582618418676926364283457744_i128 as isize;
_15 = [205_u8,88_u8,183_u8,233_u8,148_u8,145_u8,239_u8,181_u8];
_20 = '\u{10cfc4}';
_20 = '\u{9b342}';
_16 = 3783820564421829606_u64 as i8;
_2 = _1 << _11;
_22 = _1;
_15 = [206_u8,33_u8,87_u8,123_u8,138_u8,148_u8,164_u8,234_u8];
_12.3 = &_5;
_6 = _1 as f64;
_5 = !(-1706179491_i32);
Goto(bb14)
}
bb14 = {
_22 = _1;
_15 = [187_u8,231_u8,189_u8,242_u8,141_u8,126_u8,95_u8,255_u8];
_12.3 = &_5;
_20 = '\u{108183}';
_4 = -_1;
_1 = _6 as isize;
_22 = _2 << _2;
_25 = (2536910299_u32, _15);
_17 = [1051418337819264758_usize,14373729804776126155_usize,2_usize,4_usize,16689513870664022470_usize,14500579670199894689_usize,3_usize];
RET = !_22;
RET = _2 | _1;
_12.3 = &_5;
_21 = 6_usize ^ 1081210606829723793_usize;
_11 = 3602110985475192076_i64 + 7509298845801615086_i64;
_25 = (2739781640_u32, _15);
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(8_usize, 20_usize, Move(_20), 4_usize, Move(_4), 22_usize, Move(_22), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(8_usize, 16_usize, Move(_16), 2_usize, Move(_2), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: &'static i32,mut _2: isize,mut _3: i64,mut _4: isize) -> f64 {
mir! {
type RET = f64;
let _5: isize;
let _6: f64;
let _7: usize;
let _8: isize;
let _9: [i8; 2];
let _10: bool;
let _11: i8;
let _12: &'static i32;
let _13: char;
let _14: [bool; 2];
let _15: isize;
let _16: i128;
let _17: f64;
let _18: u64;
let _19: &'static ([i128; 3], bool, [char; 7], isize);
let _20: isize;
let _21: *mut (i32, u128, &'static bool);
let _22: ();
let _23: ();
{
_5 = _2 << _4;
RET = 28577_u16 as f64;
_2 = _4;
_2 = 29198_i16 as isize;
_3 = 4416613990113355497_usize as i64;
RET = 250555317415297845703294545056186451212_u128 as f64;
_4 = RET as isize;
_3 = !(-1301386553677712369_i64);
RET = 80118392409862083023051257915437189454_i128 as f64;
RET = 565469584_i32 as f64;
RET = 9088_i16 as f64;
_4 = _5 ^ _5;
_5 = '\u{d83d4}' as isize;
_2 = -_4;
Call(_3 = fn10(_4, _4, _4, RET, _2, _2, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
Goto(bb2)
}
bb2 = {
_6 = 128_u8 as f64;
RET = _3 as f64;
_7 = 622416738416260797_usize ^ 3_usize;
RET = _6;
_4 = 1731971775_u32 as isize;
_8 = _2 & _2;
_8 = -_2;
_8 = _5 | _2;
_9 = [(-108_i8),100_i8];
_8 = -_2;
_7 = 4992382143722649122_usize | 14587893616929400506_usize;
_3 = !(-494216149819963527_i64);
_11 = -(-57_i8);
_8 = !_2;
_13 = '\u{ce1a2}';
_10 = true;
_5 = -_8;
_7 = 6_usize >> _5;
RET = -_6;
_4 = _2 * _8;
RET = _6;
_3 = !(-5863612390208439334_i64);
_9 = [_11,_11];
Call(_7 = fn13(_4, _4, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _3 as isize;
_7 = 11459293052958684767_usize | 13348218635340777054_usize;
_5 = !_4;
_7 = !8915394064759877863_usize;
_3 = (-3786075286470681595_i64);
_9 = [_11,_11];
_15 = _4 << _5;
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
340282366920938463459588532145297529861 => bb6,
_ => bb5
}
}
bb4 = {
_6 = 128_u8 as f64;
RET = _3 as f64;
_7 = 622416738416260797_usize ^ 3_usize;
RET = _6;
_4 = 1731971775_u32 as isize;
_8 = _2 & _2;
_8 = -_2;
_8 = _5 | _2;
_9 = [(-108_i8),100_i8];
_8 = -_2;
_7 = 4992382143722649122_usize | 14587893616929400506_usize;
_3 = !(-494216149819963527_i64);
_11 = -(-57_i8);
_8 = !_2;
_13 = '\u{ce1a2}';
_10 = true;
_5 = -_8;
_7 = 6_usize >> _5;
RET = -_6;
_4 = _2 * _8;
RET = _6;
_3 = !(-5863612390208439334_i64);
_9 = [_11,_11];
Call(_7 = fn13(_4, _4, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
Goto(bb2)
}
bb6 = {
_2 = _10 as isize;
Goto(bb7)
}
bb7 = {
_16 = -166613803882885015822289142056696251791_i128;
_3 = (-7736362945915881581_i64);
_10 = !false;
_7 = 3_usize;
_14 = [_10,_10];
_16 = -(-75593197514975277300833705114184873157_i128);
_13 = '\u{5fafa}';
_2 = _15;
RET = _6;
Goto(bb8)
}
bb8 = {
_13 = '\u{d6a2}';
_2 = !_15;
_17 = -RET;
_14 = [_10,_10];
_14 = [_10,_10];
RET = _6 * _17;
_3 = (-4140816371502907447_i64);
_5 = _17 as isize;
_15 = _2 & _4;
_18 = 17422787073863895134_u64;
_14 = [_10,_10];
_10 = false;
RET = _17 + _17;
_7 = 4121290741548942329_usize >> _2;
_20 = _4;
_2 = _4;
_3 = _6 as i64;
RET = _15 as f64;
_20 = _3 as isize;
_16 = !77318845539369192695958438432544931832_i128;
_17 = -RET;
_10 = _17 == RET;
_5 = _15;
_5 = _2 >> _7;
_4 = _5 | _15;
_14 = [_10,_10];
Goto(bb9)
}
bb9 = {
Call(_22 = dump_var(9_usize, 20_usize, Move(_20), 4_usize, Move(_4), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_22 = dump_var(9_usize, 13_usize, Move(_13), 15_usize, Move(_15), 10_usize, Move(_10), 23_usize, _23), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: f64,mut _5: isize,mut _6: isize,mut _7: isize) -> i64 {
mir! {
type RET = i64;
let _8: [char; 5];
let _9: u128;
let _10: &'static usize;
let _11: [char; 7];
let _12: isize;
let _13: char;
let _14: [char; 5];
let _15: [bool; 3];
let _16: Adt73;
let _17: isize;
let _18: u128;
let _19: bool;
let _20: ();
let _21: ();
{
RET = (-7896168951454099777_i64) ^ (-2253642402573116266_i64);
_7 = _2 * _1;
_3 = !_2;
RET = 1874730943_i32 as i64;
_4 = 347993417_i32 as f64;
_2 = -_7;
_2 = -_3;
Call(_7 = core::intrinsics::bswap(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _3;
_4 = 11979503603759500967896812888228248499_u128 as f64;
_3 = !_6;
_9 = !87591980806759814633211281642187360110_u128;
_1 = -_7;
_1 = 201_u8 as isize;
_9 = !39291338699150008962119645655914653281_u128;
_1 = _3;
_5 = -_7;
_7 = _1 & _2;
_11 = ['\u{4e389}','\u{a892d}','\u{e1b9d}','\u{194d3}','\u{eafd8}','\u{69a9}','\u{fdf25}'];
_4 = RET as f64;
_3 = -_1;
_12 = _7;
_9 = 199684065281199590497650739687454378080_u128 >> _7;
Goto(bb2)
}
bb2 = {
RET = 13559636410050678057_u64 as i64;
_4 = 8929364587895622346_usize as f64;
_8 = ['\u{57800}','\u{4070e}','\u{211e0}','\u{f064d}','\u{bbd53}'];
_11 = ['\u{f9412}','\u{2d47a}','\u{bb9d1}','\u{8834c}','\u{688b7}','\u{f3c88}','\u{8a07e}'];
_3 = _9 as isize;
RET = -(-3125142957680212496_i64);
RET = 60790_u16 as i64;
_9 = 335101909100055776418913537098913168314_u128 >> _3;
_5 = _12 << _9;
RET = !6606019313159507881_i64;
_12 = !_7;
_8 = ['\u{c464f}','\u{a4d53}','\u{f7486}','\u{4d62b}','\u{7efff}'];
_4 = RET as f64;
_13 = '\u{6b615}';
_8 = [_13,_13,_13,_13,_13];
_12 = -_1;
_11 = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb3)
}
bb3 = {
_15 = [true,true,false];
_6 = !_7;
RET = (-4013819349092767904_i64);
RET = 5602_i16 as i64;
_2 = _3 << _5;
_3 = _6 - _7;
_15 = [false,true,false];
_14 = [_13,_13,_13,_13,_13];
_15 = [false,true,true];
_15 = [true,true,true];
_1 = !_3;
_3 = -_7;
_11 = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb4)
}
bb4 = {
_6 = _1 - _2;
_3 = _6;
_14 = [_13,_13,_13,_13,_13];
_12 = _13 as isize;
_15 = [false,false,true];
_2 = _5;
_5 = _3;
_17 = 36521_u16 as isize;
_1 = -_5;
_11 = [_13,_13,_13,_13,_13,_13,_13];
_11 = [_13,_13,_13,_13,_13,_13,_13];
_9 = !263702609270941118834915119595290752009_u128;
_12 = _5 * _1;
_3 = !_12;
_15 = [true,true,false];
_4 = _9 as f64;
_3 = _2 - _5;
_9 = (-113_i8) as u128;
_19 = false;
_12 = _6 >> _5;
_7 = 1087246753_i32 as isize;
_3 = !_1;
RET = 32047_i16 as i64;
RET = (-5483956637597113625_i64);
Call(_18 = fn11(_12, _6, _1, _5, _6, _5, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = _1 & _1;
_5 = _3;
_2 = !_5;
_11 = [_13,_13,_13,_13,_13,_13,_13];
RET = (-8331705396329989749_i64) >> _12;
_19 = _1 <= _1;
_13 = '\u{55557}';
_17 = 28332_i16 as isize;
_1 = _2 + _3;
_13 = '\u{5a4c4}';
_12 = -_6;
_7 = -_6;
Goto(bb6)
}
bb6 = {
Call(_20 = dump_var(10_usize, 18_usize, Move(_18), 3_usize, Move(_3), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_20 = dump_var(10_usize, 12_usize, Move(_12), 2_usize, Move(_2), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize) -> u128 {
mir! {
type RET = u128;
let _8: i32;
let _9: [u8; 8];
let _10: isize;
let _11: Adt55;
let _12: &'static (i64, u64);
let _13: Adt79;
let _14: u128;
let _15: (u32, [u8; 8]);
let _16: i16;
let _17: Adt51;
let _18: [char; 8];
let _19: &'static (f64,);
let _20: isize;
let _21: char;
let _22: (*mut *const char, &'static [char; 7], [isize; 4], &'static i32);
let _23: [i8; 2];
let _24: f32;
let _25: i128;
let _26: (f64,);
let _27: isize;
let _28: bool;
let _29: isize;
let _30: &'static i32;
let _31: &'static *const (i32, u128, &'static bool);
let _32: f64;
let _33: ([i128; 4], ([i128; 3], bool, [char; 7], isize));
let _34: [usize; 7];
let _35: &'static *mut i128;
let _36: u32;
let _37: [i128; 3];
let _38: i128;
let _39: &'static usize;
let _40: ();
let _41: ();
{
_8 = 48659_u16 as i32;
_2 = _8 as isize;
_3 = _7 << _6;
_9 = [197_u8,70_u8,186_u8,105_u8,71_u8,206_u8,36_u8,194_u8];
RET = 25895_u16 as u128;
_1 = -_3;
_10 = _4;
_7 = _4;
_4 = !_10;
RET = 55170052531530532591014334312903539589_u128;
_3 = -_1;
_1 = -_5;
_13.fld1 = '\u{36bd3}';
RET = 102689454360596990863192204614218894017_u128;
RET = !149204411307816719273048737392221780523_u128;
_13.fld6.0 = [163584767670067780870581856714722595140_i128,(-12430232426602045576530550755216047350_i128),123822208841874824611731692167744209514_i128];
_6 = !_3;
_13.fld1 = '\u{30ea2}';
Call(_15.0 = fn12(_1, _6, _4, _3, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.fld2 = core::ptr::addr_of!(_8);
_3 = (-2998242771395053278_i64) as isize;
_13.fld6.3 = _10 - _7;
_13.fld6.1 = true;
RET = _13.fld1 as u128;
_13.fld6.1 = _15.0 <= _15.0;
_8 = !(-1672409059_i32);
RET = 73611060268490565956267599685498676805_u128;
_13.fld0 = [94_i8,7_i8,(-121_i8),20_i8,110_i8,35_i8,105_i8,(-13_i8)];
_13.fld6.0 = [(-53215737880870733339670673732295811323_i128),(-113829956376456747196527798788444182876_i128),(-104470596521939075484901515045858761454_i128)];
_13.fld2 = core::ptr::addr_of!(_8);
_14 = RET;
_2 = _10 * _4;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
73611060268490565956267599685498676805 => bb7,
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
_13.fld4 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_5 = _7;
_17.fld1 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_3 = _5 | _10;
_6 = !_3;
_10 = !_13.fld6.3;
_8 = 781907051_i32;
_13.fld2 = core::ptr::addr_of!(_8);
_15.1 = _9;
_13.fld2 = core::ptr::addr_of!(_8);
_20 = _13.fld6.3 - _4;
_15 = (3497008082_u32, _9);
_3 = _4 + _7;
_18 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_17.fld1 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_21 = _13.fld1;
_25 = 152_u8 as i128;
match _15.0 {
0 => bb1,
1 => bb6,
2 => bb8,
3497008082 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_22.1 = &_13.fld6.2;
_23 = [96_i8,(-90_i8)];
_13.fld5 = _17.fld1;
_15 = (1602591821_u32, _9);
_10 = _3 >> _5;
_13.fld6.1 = _10 <= _10;
_13.fld2 = core::ptr::addr_of!(_8);
_13.fld6.2 = [_21,_13.fld1,_21,_21,_13.fld1,_13.fld1,_13.fld1];
_22.3 = &_8;
_7 = _4;
_4 = !_6;
_5 = _15.0 as isize;
_17.fld1 = [_13.fld1,_13.fld1,_21,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_16 = 6751_i16;
_13.fld2 = core::ptr::addr_of!(_8);
_9 = [197_u8,31_u8,100_u8,244_u8,206_u8,143_u8,230_u8,93_u8];
_17.fld1 = [_21,_21,_21,_21,_13.fld1,_21,_21];
_22.2 = [_3,_13.fld6.3,_7,_3];
_2 = _4 - _4;
_26.0 = 38115_u16 as f64;
_18 = [_13.fld1,_13.fld1,_21,_21,_13.fld1,_21,_21,_21];
_28 = _13.fld6.1;
_27 = _13.fld6.3;
_24 = _15.0 as f32;
_29 = -_7;
_24 = 2105352147559217220_u64 as f32;
_22.3 = &_8;
_15.1 = _9;
Goto(bb11)
}
bb11 = {
_2 = _8 as isize;
_17 = Adt51 { fld0: _23,fld1: _13.fld5 };
_18 = [_21,_13.fld1,_13.fld1,_21,_13.fld1,_21,_21,_13.fld1];
_13.fld4 = [_13.fld1,_21,_13.fld1,_13.fld1,_13.fld1];
_17.fld0 = _23;
_17 = Adt51 { fld0: _23,fld1: _13.fld5 };
_26.0 = 2472917705279364755_u64 as f64;
_15 = (2741774219_u32, _9);
_26.0 = 16505225859560301360_u64 as f64;
_6 = _13.fld6.1 as isize;
_33.1.3 = _6;
Call(_13.fld6.3 = core::intrinsics::bswap(_20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = RET << _20;
_1 = 7_usize as isize;
_10 = _13.fld6.3 + _27;
_32 = 21_u8 as f64;
_13.fld6.1 = !_28;
_13.fld6.3 = -_3;
_2 = !_6;
_33.1 = (_13.fld6.0, _13.fld6.1, _13.fld6.2, _2);
_1 = _4 + _20;
_22.2 = [_1,_7,_20,_20];
_17.fld1 = [_21,_13.fld1,_13.fld1,_21,_13.fld1,_13.fld1,_13.fld1];
_13.fld6.0 = _33.1.0;
_22.1 = &_17.fld1;
_15.0 = _16 as u32;
_30 = &_8;
_10 = !_1;
_26 = (_32,);
match _8 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb7,
5 => bb8,
781907051 => bb14,
_ => bb13
}
}
bb13 = {
_13.fld4 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_5 = _7;
_17.fld1 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_3 = _5 | _10;
_6 = !_3;
_10 = !_13.fld6.3;
_8 = 781907051_i32;
_13.fld2 = core::ptr::addr_of!(_8);
_15.1 = _9;
_13.fld2 = core::ptr::addr_of!(_8);
_20 = _13.fld6.3 - _4;
_15 = (3497008082_u32, _9);
_3 = _4 + _7;
_18 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_17.fld1 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_21 = _13.fld1;
_25 = 152_u8 as i128;
match _15.0 {
0 => bb1,
1 => bb6,
2 => bb8,
3497008082 => bb10,
_ => bb9
}
}
bb14 = {
_20 = !_29;
_33.0 = [_25,_25,_25,_25];
_6 = _10;
_33.1.1 = _13.fld6.1;
_33.1.3 = _3;
_8 = !2090068501_i32;
_37 = _13.fld6.0;
_36 = _15.0 + _15.0;
_13.fld6.3 = _4 * _2;
_37 = _33.1.0;
_2 = -_7;
_15.1 = [226_u8,169_u8,189_u8,28_u8,232_u8,193_u8,136_u8,193_u8];
_13.fld6.0 = [_25,_25,_25];
_13.fld1 = _21;
_32 = _26.0;
_17.fld0 = [(-36_i8),(-30_i8)];
_18 = [_13.fld1,_13.fld1,_13.fld1,_21,_13.fld1,_21,_21,_13.fld1];
_38 = _25 | _25;
_33.1.3 = _4 + _6;
_13.fld4 = [_21,_13.fld1,_21,_21,_21];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(11_usize, 38_usize, Move(_38), 20_usize, Move(_20), 28_usize, Move(_28), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(11_usize, 27_usize, Move(_27), 6_usize, Move(_6), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(11_usize, 15_usize, Move(_15), 36_usize, Move(_36), 25_usize, Move(_25), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize) -> u32 {
mir! {
type RET = u32;
let _6: char;
let _7: &'static [char; 7];
let _8: ();
let _9: ();
{
RET = 3111343470_u32;
_5 = _2;
_5 = 6641638602425378076_usize as isize;
RET = 397933365_u32 & 3568688047_u32;
_4 = -_3;
_4 = _3;
_5 = _2 << _3;
_2 = _4 | _3;
_1 = _5;
RET = !2608216607_u32;
RET = 2541625623_u32;
_3 = _2 & _5;
_3 = 59408_u16 as isize;
RET = 1797109778_u32 << _5;
_1 = 13508849555345751050_u64 as isize;
_2 = -_5;
RET = !144580824_u32;
_2 = _5;
_1 = !_2;
RET = 3026447673_u32 << _2;
_5 = _4 << _1;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(12_usize, 3_usize, Move(_3), 1_usize, Move(_1), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: isize) -> usize {
mir! {
type RET = usize;
let _4: (f64,);
let _5: ([i128; 4], ([i128; 3], bool, [char; 7], isize));
let _6: char;
let _7: ((i32, u128, &'static bool), [i8; 8], *const *mut (i64, i8, *const u64, [char; 8]));
let _8: isize;
let _9: f32;
let _10: [bool; 3];
let _11: &'static usize;
let _12: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _13: i64;
let _14: &'static ([i128; 3], bool, [char; 7], isize);
let _15: u16;
let _16: *const u64;
let _17: Adt79;
let _18: [usize; 7];
let _19: (*const (i32, u128, &'static bool), [char; 8], u64);
let _20: [usize; 7];
let _21: &'static [char; 7];
let _22: u32;
let _23: [isize; 4];
let _24: bool;
let _25: Adt46;
let _26: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _27: *const &'static u32;
let _28: i32;
let _29: &'static usize;
let _30: Adt79;
let _31: ([i128; 3], bool, [char; 7], isize);
let _32: Adt44;
let _33: [char; 8];
let _34: ();
let _35: ();
{
_1 = 9583145149984280278_u64 as isize;
RET = 15219168947660892782_usize >> _2;
RET = 15453586535021480749_usize;
_3 = _2 | _2;
RET = 5_usize ^ 5137917053585922115_usize;
_1 = _2;
_1 = _3;
RET = 12900564136625242488_usize;
_2 = 18169506405345227469_u64 as isize;
RET = !4862086774290820215_usize;
RET = 25977_i16 as usize;
_4.0 = RET as f64;
_2 = _3;
RET = 2770726692260419299_usize;
RET = 87571230316920341410308063048814539130_i128 as usize;
RET = 7_usize;
RET = 30466_u16 as usize;
_5.1.1 = _2 > _3;
_5.0 = [60209121896817072027437198240333993361_i128,(-52056055990654743831721670649101239323_i128),44347211961363408744584498885491385666_i128,(-87333686479769616606264804055728153194_i128)];
_5.1.3 = _3 >> _2;
RET = !14970870498367493338_usize;
_1 = _3;
_4.0 = (-1858987670_i32) as f64;
_5.1.2 = ['\u{1059a6}','\u{ef720}','\u{51ea5}','\u{e37e2}','\u{c3e2b}','\u{83f84}','\u{396b4}'];
_3 = RET as isize;
_5.1.0 = [141177307457027092313243397384039384072_i128,144739501833396951956601347495323471080_i128,(-48725849090905184918565266137550892205_i128)];
_5.1.1 = true;
Goto(bb1)
}
bb1 = {
RET = 17782973148572274155_usize;
_6 = '\u{d7d8e}';
_8 = !_5.1.3;
_5.1.2 = [_6,_6,_6,_6,_6,_6,_6];
_5.1.0 = [(-96212423821004883682982789014073146811_i128),(-22950253133326964905966300620271953584_i128),(-32929588318741850313424012558762467989_i128)];
_4.0 = 44_u8 as f64;
_6 = '\u{3689d}';
_2 = _1 & _8;
_3 = RET as isize;
_7.0.2 = &_5.1.1;
_7.0.1 = _6 as u128;
_9 = (-103120835199645094436624282288997941681_i128) as f32;
_5.1.3 = _8 & _8;
RET = 6_usize;
_4.0 = _9 as f64;
_7.1[RET] = (-122_i8) & 118_i8;
RET = 2172805802596785656_usize;
_7.0.2 = &_5.1.1;
_7.0.2 = &_5.1.1;
_7.0.1 = 160098393304025375213529324852462785744_u128;
_7.0.2 = &_5.1.1;
_12 = core::ptr::addr_of_mut!(_7.2);
_7.0.1 = 311656738316349174487427731745620820948_u128 & 208741330684204161289619375010145987082_u128;
Goto(bb2)
}
bb2 = {
_2 = _5.1.3 & _5.1.3;
RET = 1416386259_u32 as usize;
_4.0 = _9 as f64;
RET = 30_u8 as usize;
_5.0 = [(-7562936253689569586514149218473925369_i128),(-44785951315461143788395274209997338380_i128),160146136513194175706125194261261310580_i128,(-23353764671155431055021841604198451535_i128)];
_5.0 = [23558090098229347698239610002087281181_i128,74556532275039114766922135753086820141_i128,75772375136754159715430312309160368910_i128,(-6871592068060447083697325991744559398_i128)];
_5.1.1 = !false;
_5.1.0 = [30327726020248784087236598951058159198_i128,91998138282222495492807764638790506616_i128,106352467275946257739642260415585804557_i128];
RET = 10060311100336682915_usize;
_7.0.2 = &_5.1.1;
_5.1.0 = [26209035330399352922869804655508703884_i128,53981729479839124537778867804298261192_i128,(-37273683636243089725199747194738732828_i128)];
_12 = core::ptr::addr_of_mut!((*_12));
_5.1.1 = !false;
_11 = &RET;
_4.0 = 28261_i16 as f64;
RET = !4_usize;
_11 = &RET;
_5.1.3 = !_1;
_7.1 = [(-28_i8),44_i8,52_i8,69_i8,(-10_i8),4_i8,42_i8,98_i8];
_4.0 = 10850_i16 as f64;
_14 = &_5.1;
_5.1.1 = _8 <= _8;
Goto(bb3)
}
bb3 = {
_2 = _5.1.3;
_13 = !4827876716383558510_i64;
_2 = _5.1.3 - (*_14).3;
_5.1.3 = _5.1.1 as isize;
_6 = '\u{7f54b}';
_7.0.1 = 84913352788293523463442510993239747326_u128;
RET = 13899204019014246456_usize;
_2 = -_5.1.3;
_10 = [_5.1.1,_5.1.1,_5.1.1];
_3 = _8 << _5.1.3;
_17.fld6.1 = _5.1.1;
_5.0 = [104271984924451330099964433024625814783_i128,(-107162357681628074129359330669803654028_i128),(-16859478800411884075937102038173990356_i128),153031158313839664521588267618996915082_i128];
_7.0.1 = 48402_u16 as u128;
_17.fld6 = ((*_14).0, _5.1.1, _5.1.2, _2);
_7.0.0 = -(-1857101365_i32);
_5.1.1 = !_17.fld6.1;
_5.0 = [98589755665927820133622854493566844962_i128,(-159889294288276809140472191629322368878_i128),(-58234434918743159494152692905488038566_i128),40554046855118826975372843856570612795_i128];
_8 = -_17.fld6.3;
_17.fld1 = _6;
_13 = 2106986959387502431_i64;
_17.fld2 = core::ptr::addr_of!(_7.0.0);
_4.0 = _13 as f64;
_10 = [_5.1.1,_5.1.1,_17.fld6.1];
_7.0.2 = &_17.fld6.1;
_5.0 = [(-77744409025070423118567036141985025875_i128),(-41478807795202029663925295604195811659_i128),(-148196848481305071948501425064635915841_i128),(-156277987972634637023104015365246779906_i128)];
_9 = _17.fld6.3 as f32;
_13 = 406712572417519317_i64 * 3931039811990031234_i64;
_7.0.2 = &_17.fld6.1;
match RET {
0 => bb4,
1 => bb5,
13899204019014246456 => bb7,
_ => bb6
}
}
bb4 = {
_2 = _5.1.3 & _5.1.3;
RET = 1416386259_u32 as usize;
_4.0 = _9 as f64;
RET = 30_u8 as usize;
_5.0 = [(-7562936253689569586514149218473925369_i128),(-44785951315461143788395274209997338380_i128),160146136513194175706125194261261310580_i128,(-23353764671155431055021841604198451535_i128)];
_5.0 = [23558090098229347698239610002087281181_i128,74556532275039114766922135753086820141_i128,75772375136754159715430312309160368910_i128,(-6871592068060447083697325991744559398_i128)];
_5.1.1 = !false;
_5.1.0 = [30327726020248784087236598951058159198_i128,91998138282222495492807764638790506616_i128,106352467275946257739642260415585804557_i128];
RET = 10060311100336682915_usize;
_7.0.2 = &_5.1.1;
_5.1.0 = [26209035330399352922869804655508703884_i128,53981729479839124537778867804298261192_i128,(-37273683636243089725199747194738732828_i128)];
_12 = core::ptr::addr_of_mut!((*_12));
_5.1.1 = !false;
_11 = &RET;
_4.0 = 28261_i16 as f64;
RET = !4_usize;
_11 = &RET;
_5.1.3 = !_1;
_7.1 = [(-28_i8),44_i8,52_i8,69_i8,(-10_i8),4_i8,42_i8,98_i8];
_4.0 = 10850_i16 as f64;
_14 = &_5.1;
_5.1.1 = _8 <= _8;
Goto(bb3)
}
bb5 = {
RET = 17782973148572274155_usize;
_6 = '\u{d7d8e}';
_8 = !_5.1.3;
_5.1.2 = [_6,_6,_6,_6,_6,_6,_6];
_5.1.0 = [(-96212423821004883682982789014073146811_i128),(-22950253133326964905966300620271953584_i128),(-32929588318741850313424012558762467989_i128)];
_4.0 = 44_u8 as f64;
_6 = '\u{3689d}';
_2 = _1 & _8;
_3 = RET as isize;
_7.0.2 = &_5.1.1;
_7.0.1 = _6 as u128;
_9 = (-103120835199645094436624282288997941681_i128) as f32;
_5.1.3 = _8 & _8;
RET = 6_usize;
_4.0 = _9 as f64;
_7.1[RET] = (-122_i8) & 118_i8;
RET = 2172805802596785656_usize;
_7.0.2 = &_5.1.1;
_7.0.2 = &_5.1.1;
_7.0.1 = 160098393304025375213529324852462785744_u128;
_7.0.2 = &_5.1.1;
_12 = core::ptr::addr_of_mut!(_7.2);
_7.0.1 = 311656738316349174487427731745620820948_u128 & 208741330684204161289619375010145987082_u128;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_10 = [_17.fld6.1,_5.1.1,_5.1.1];
RET = !5_usize;
_5.1.1 = !_17.fld6.1;
_19.1 = [_17.fld1,_6,_6,_17.fld1,_6,_6,_17.fld1,_6];
_15 = _17.fld6.1 as u16;
_23 = [_2,_17.fld6.3,_8,_17.fld6.3];
_22 = _7.0.1 as u32;
_19.0 = core::ptr::addr_of!(_7.0);
_17.fld4 = [_6,_6,_6,_17.fld1,_17.fld1];
_26 = core::ptr::addr_of_mut!(_7.2);
_12 = core::ptr::addr_of_mut!((*_26));
_7.1 = [127_i8,(-91_i8),23_i8,(-59_i8),(-90_i8),(-2_i8),(-62_i8),54_i8];
_17.fld3 = core::ptr::addr_of_mut!((*_26));
_17.fld6.2 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1,_6,_17.fld1];
Goto(bb8)
}
bb8 = {
_10 = [_17.fld6.1,_5.1.1,_5.1.1];
_17.fld2 = core::ptr::addr_of!(_7.0.0);
_7.0.2 = &_5.1.1;
_17.fld0 = _7.1;
_17.fld1 = _6;
_17.fld6 = (_5.1.0, _5.1.1, (*_14).2, _5.1.3);
_5.1.2 = [_17.fld1,_6,_17.fld1,_17.fld1,_6,_6,_6];
_11 = &RET;
_21 = &_5.1.2;
_5.1.0 = _17.fld6.0;
_1 = !_2;
_16 = core::ptr::addr_of!(_19.2);
_18 = [RET,RET,RET,(*_11),(*_11),RET,(*_11)];
_28 = _7.0.0 * _7.0.0;
_26 = core::ptr::addr_of_mut!((*_12));
_5.0 = [(-162799907588978542236058220586104241735_i128),(-104114553097729894073953960911898307173_i128),(-85076918171976355417856814227701403795_i128),(-72675990707586315840467626580092843634_i128)];
_17.fld5 = (*_21);
_14 = &_5.1;
_17.fld4 = [_6,_6,_6,_17.fld1,_6];
_17.fld6.3 = -_1;
_17.fld1 = _6;
_5.0 = [168415232382020112674442878879608555809_i128,82469920934963241381320207809727107249_i128,(-139794362603980559729827206097063535318_i128),96996168283011321065818396203263123059_i128];
_7.0.2 = &(*_14).1;
_29 = Move(_11);
_16 = core::ptr::addr_of!(_19.2);
_5.0 = [38927231786559261516337690803996713975_i128,163872482117058808508068067347149561345_i128,(-123386523958392348999327857502716485695_i128),(-64102994326383263912933304808661734647_i128)];
_17.fld1 = _6;
Goto(bb9)
}
bb9 = {
_11 = &RET;
_19.0 = core::ptr::addr_of!(_7.0);
_15 = (-103_i8) as u16;
_29 = &(*_11);
_19.0 = core::ptr::addr_of!(_7.0);
_12 = core::ptr::addr_of_mut!((*_12));
_30.fld6.0 = (*_14).0;
_19.1 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_6,_17.fld1,_6,_6];
_17.fld6 = (_5.1.0, _5.1.1, (*_14).2, (*_14).3);
_8 = _5.1.3 ^ _5.1.3;
_15 = 19731_u16;
Goto(bb10)
}
bb10 = {
_25 = Adt46::Variant2 { fld0: _5.1.2 };
_30.fld6 = (*_14);
_3 = _17.fld6.3;
_30.fld4 = _17.fld4;
_30.fld5 = (*_14).2;
_1 = _30.fld6.3 * _5.1.3;
Call(_31.1 = fn14(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = core::ptr::addr_of!(_19.2);
_31.1 = !_5.1.1;
_29 = Move(_11);
_24 = _30.fld6.1;
place!(Field::<[char; 7]>(Variant(_25, 2), 0)) = _30.fld5;
_19.2 = 10206944768101175301_u64 * 13240365693689744576_u64;
_31.2 = [_17.fld1,_6,_17.fld1,_17.fld1,_17.fld1,_6,_6];
_31.3 = _30.fld6.3;
_2 = _24 as isize;
_18 = [RET,RET,RET,RET,RET,RET,RET];
_20 = [RET,RET,RET,RET,RET,RET,RET];
_7.0.2 = &_31.1;
_30.fld0 = _7.1;
_17.fld6.3 = _8 & _8;
SetDiscriminant(_25, 1);
_29 = &RET;
_7.0.2 = &_30.fld6.1;
_5.1 = (_30.fld6.0, _30.fld6.1, _31.2, _8);
_5.1.0 = [(-98272983043706963571250161100252472701_i128),(-127973648679273408268608609763647539990_i128),130638386951173376244703441720918445160_i128];
_30.fld1 = _17.fld1;
_13 = 118_u8 as i64;
_13 = (-1940252243137529521_i64) * 2778548130102061688_i64;
match _15 {
19731 => bb13,
_ => bb12
}
}
bb12 = {
_11 = &RET;
_19.0 = core::ptr::addr_of!(_7.0);
_15 = (-103_i8) as u16;
_29 = &(*_11);
_19.0 = core::ptr::addr_of!(_7.0);
_12 = core::ptr::addr_of_mut!((*_12));
_30.fld6.0 = (*_14).0;
_19.1 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_6,_17.fld1,_6,_6];
_17.fld6 = (_5.1.0, _5.1.1, (*_14).2, (*_14).3);
_8 = _5.1.3 ^ _5.1.3;
_15 = 19731_u16;
Goto(bb10)
}
bb13 = {
(*_16) = !17139363739607813007_u64;
_5.1.1 = _17.fld6.3 > _1;
_30.fld2 = core::ptr::addr_of!(_7.0.0);
RET = 8809976807864065658_usize ^ 6412355594811671782_usize;
_31.0 = [(-41952940314774849639885059631773683214_i128),(-145790759602781497263938340900226504005_i128),101051383806988078535845692961681388308_i128];
_17.fld0 = [(-79_i8),72_i8,76_i8,112_i8,94_i8,(-127_i8),71_i8,(-43_i8)];
_11 = &RET;
_22 = 4208666222_u32;
_31.0 = [11344799357627336592202608408104392954_i128,(-58346722126223513936778533362950224595_i128),(-143155796641292043407282078679052372051_i128)];
_7.0.2 = &_5.1.1;
RET = 17739607996246489541_usize;
_5.1.2 = [_30.fld1,_6,_6,_30.fld1,_17.fld1,_17.fld1,_30.fld1];
(*_16) = !1301131537014068071_u64;
_14 = &_5.1;
_30.fld6.3 = -(*_14).3;
place!(Field::<[u8; 8]>(Variant(_25, 1), 0)) = [152_u8,164_u8,169_u8,109_u8,24_u8,107_u8,18_u8,209_u8];
_24 = (*_14).1;
_17.fld6.1 = _30.fld6.1;
_31 = _30.fld6;
_31.2 = [_30.fld1,_17.fld1,_6,_6,_30.fld1,_30.fld1,_6];
Goto(bb14)
}
bb14 = {
_17.fld6.1 = (*_14).1;
place!(Field::<i8>(Variant(_25, 1), 1)) = (-58_i8) - (-119_i8);
_30.fld6.1 = (*_14).1;
SetDiscriminant(_25, 2);
_31.1 = !_30.fld6.1;
_17.fld6.1 = _5.1.1 < _5.1.1;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(13_usize, 3_usize, Move(_3), 2_usize, Move(_2), 10_usize, Move(_10), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(13_usize, 13_usize, Move(_13), 23_usize, Move(_23), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14() -> bool {
mir! {
type RET = bool;
let _1: bool;
let _2: Adt51;
let _3: (i32, u128, &'static bool);
let _4: *const i32;
let _5: u32;
let _6: i128;
let _7: [i16; 5];
let _8: isize;
let _9: char;
let _10: f64;
let _11: u8;
let _12: u64;
let _13: [i128; 3];
let _14: char;
let _15: &'static *const (i32, u128, &'static bool);
let _16: f64;
let _17: i32;
let _18: &'static i32;
let _19: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _20: bool;
let _21: f32;
let _22: [usize; 7];
let _23: f32;
let _24: isize;
let _25: *const *mut (i64, i8, *const u64, [char; 8]);
let _26: Adt44;
let _27: isize;
let _28: isize;
let _29: f64;
let _30: char;
let _31: Adt51;
let _32: usize;
let _33: u128;
let _34: [char; 7];
let _35: f64;
let _36: ();
let _37: ();
{
RET = true;
RET = 9338_i16 != 24072_i16;
RET = true;
Goto(bb1)
}
bb1 = {
RET = !true;
RET = 124927707258256101157698259197863788629_u128 <= 46180993072800802586129817568562252619_u128;
_1 = RET & RET;
RET = _1 | _1;
_2.fld0 = [(-82_i8),58_i8];
RET = !_1;
_3.0 = (-605652807_i32);
_2.fld0 = [107_i8,99_i8];
_2.fld1 = ['\u{e2f6e}','\u{d119c}','\u{3708c}','\u{2e0a3}','\u{66bf6}','\u{6aa4e}','\u{29a14}'];
_3.2 = &_1;
RET = _3.0 <= _3.0;
_4 = core::ptr::addr_of!(_3.0);
Call(_3.0 = fn15(Move(_3.2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.2 = &RET;
_2.fld0 = [(-91_i8),73_i8];
_2.fld0 = [35_i8,2_i8];
_3.0 = (-152151432562756383048496366262402454783_i128) as i32;
_4 = core::ptr::addr_of!((*_4));
_5 = !672310616_u32;
_2.fld0 = [(-128_i8),102_i8];
_2.fld0 = [76_i8,122_i8];
RET = _1 != _1;
_2.fld0 = [(-56_i8),(-39_i8)];
_6 = -(-116077498278652764681502801511532051031_i128);
_3.2 = &RET;
_3.2 = &RET;
_3.1 = 63723707099176464148905197602176102874_u128;
_3.0 = (-278425702_i32) - (-510108486_i32);
(*_4) = 1518467515_i32 & 507471153_i32;
_1 = RET ^ RET;
_3.2 = &_1;
_6 = 107368855425388973104338229236487900452_i128;
_7 = [(-29448_i16),30984_i16,23127_i16,22654_i16,(-1281_i16)];
_2.fld0 = [(-1_i8),(-71_i8)];
RET = (*_4) != (*_4);
_9 = '\u{f2270}';
_3.2 = &RET;
_5 = 100570022_u32;
_10 = 96_u8 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb3 = {
_1 = RET;
_11 = 57_u8 >> (*_4);
_4 = core::ptr::addr_of!(_3.0);
_5 = !3851062103_u32;
_9 = '\u{939d0}';
_4 = core::ptr::addr_of!((*_4));
_8 = !9223372036854775807_isize;
_3.2 = &_1;
_3.2 = &RET;
_12 = 8132600838249985099_usize as u64;
_6 = (-89556069570718413702292283391675210542_i128);
_14 = _9;
_2.fld0 = [(-16_i8),(-62_i8)];
_3.0 = (-1009480089_i32);
_6 = 119453356529905425555611511156958030035_i128;
match (*_4) {
0 => bb1,
340282366920938463463374607430758731367 => bb4,
_ => bb2
}
}
bb4 = {
_7 = [(-18814_i16),11785_i16,6902_i16,15983_i16,16066_i16];
_17 = 2_usize as i32;
_7 = [(-28037_i16),7664_i16,26122_i16,(-27932_i16),(-4512_i16)];
_17 = (*_4);
_3.2 = &_1;
_2.fld1 = [_9,_9,_14,_14,_14,_14,_14];
_3.2 = &RET;
_13 = [_6,_6,_6];
_3.0 = _17 + _17;
_12 = !1392774112749840788_u64;
_6 = (-21832153941048698742500883753704262167_i128) ^ (-81919371307823953074338225290388855912_i128);
_11 = _14 as u8;
(*_4) = _8 as i32;
_17 = (*_4);
_6 = 96020147057741247513225856313516327788_i128 + 115665739965597539185137011133232076911_i128;
_1 = !RET;
_6 = !50246654486030616320638977365302242906_i128;
_16 = _10 + _10;
_3.2 = &RET;
_11 = 54_u8 | 61_u8;
(*_4) = _11 as i32;
_3.1 = 5756265329579681638_i64 as u128;
_3.2 = &_1;
_8 = (-9223372036854775808_isize);
_14 = _9;
_9 = _14;
match _8 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb5 = {
_1 = RET;
_11 = 57_u8 >> (*_4);
_4 = core::ptr::addr_of!(_3.0);
_5 = !3851062103_u32;
_9 = '\u{939d0}';
_4 = core::ptr::addr_of!((*_4));
_8 = !9223372036854775807_isize;
_3.2 = &_1;
_3.2 = &RET;
_12 = 8132600838249985099_usize as u64;
_6 = (-89556069570718413702292283391675210542_i128);
_14 = _9;
_2.fld0 = [(-16_i8),(-62_i8)];
_3.0 = (-1009480089_i32);
_6 = 119453356529905425555611511156958030035_i128;
match (*_4) {
0 => bb1,
340282366920938463463374607430758731367 => bb4,
_ => bb2
}
}
bb6 = {
_3.2 = &RET;
_2.fld0 = [(-91_i8),73_i8];
_2.fld0 = [35_i8,2_i8];
_3.0 = (-152151432562756383048496366262402454783_i128) as i32;
_4 = core::ptr::addr_of!((*_4));
_5 = !672310616_u32;
_2.fld0 = [(-128_i8),102_i8];
_2.fld0 = [76_i8,122_i8];
RET = _1 != _1;
_2.fld0 = [(-56_i8),(-39_i8)];
_6 = -(-116077498278652764681502801511532051031_i128);
_3.2 = &RET;
_3.2 = &RET;
_3.1 = 63723707099176464148905197602176102874_u128;
_3.0 = (-278425702_i32) - (-510108486_i32);
(*_4) = 1518467515_i32 & 507471153_i32;
_1 = RET ^ RET;
_3.2 = &_1;
_6 = 107368855425388973104338229236487900452_i128;
_7 = [(-29448_i16),30984_i16,23127_i16,22654_i16,(-1281_i16)];
_2.fld0 = [(-1_i8),(-71_i8)];
RET = (*_4) != (*_4);
_9 = '\u{f2270}';
_3.2 = &RET;
_5 = 100570022_u32;
_10 = 96_u8 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb7 = {
RET = !true;
RET = 124927707258256101157698259197863788629_u128 <= 46180993072800802586129817568562252619_u128;
_1 = RET & RET;
RET = _1 | _1;
_2.fld0 = [(-82_i8),58_i8];
RET = !_1;
_3.0 = (-605652807_i32);
_2.fld0 = [107_i8,99_i8];
_2.fld1 = ['\u{e2f6e}','\u{d119c}','\u{3708c}','\u{2e0a3}','\u{66bf6}','\u{6aa4e}','\u{29a14}'];
_3.2 = &_1;
RET = _3.0 <= _3.0;
_4 = core::ptr::addr_of!(_3.0);
Call(_3.0 = fn15(Move(_3.2)), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_10 = -_16;
_4 = core::ptr::addr_of!(_3.0);
_22 = [6_usize,2_usize,16919698742598392012_usize,5_usize,7_usize,2825377584381417324_usize,0_usize];
_2.fld0 = [(-42_i8),27_i8];
_24 = _8;
_7 = [(-8248_i16),4688_i16,11009_i16,(-13182_i16),9239_i16];
_19 = core::ptr::addr_of_mut!(_25);
_2.fld0 = [64_i8,38_i8];
_19 = core::ptr::addr_of_mut!((*_19));
_21 = _12 as f32;
_14 = _9;
_4 = core::ptr::addr_of!((*_4));
_9 = _14;
_21 = (*_4) as f32;
_28 = _24;
_29 = _16;
Goto(bb10)
}
bb10 = {
RET = _11 == _11;
_2.fld0 = [(-79_i8),(-17_i8)];
_27 = _8 ^ _8;
_21 = _3.1 as f32;
_1 = (*_4) > (*_4);
_29 = _16 - _16;
_17 = (*_4);
_24 = -_8;
_31.fld1 = [_14,_9,_9,_14,_9,_9,_14];
_3.1 = 294633197144106366998410322979159252093_u128;
_6 = (-155849228398162153496504680653368746039_i128) * (-44771360128454611786823821654984425649_i128);
_22 = [15116195540407549670_usize,3_usize,5_usize,2_usize,14820669145608540448_usize,389733283873806233_usize,0_usize];
_7 = [(-5282_i16),(-10122_i16),25682_i16,15042_i16,(-2557_i16)];
_30 = _14;
_11 = !203_u8;
_3.0 = _17 << _5;
_13 = [_6,_6,_6];
RET = _6 <= _6;
_20 = !RET;
RET = _1;
_32 = 17660383821948545727_usize + 977739926048397220_usize;
_3.2 = &_1;
_8 = !_27;
_14 = _30;
_31.fld1 = [_9,_30,_9,_30,_30,_9,_14];
_28 = _8 | _24;
_29 = -_10;
match _3.1 {
0 => bb2,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
294633197144106366998410322979159252093 => bb17,
_ => bb16
}
}
bb11 = {
_10 = -_16;
_4 = core::ptr::addr_of!(_3.0);
_22 = [6_usize,2_usize,16919698742598392012_usize,5_usize,7_usize,2825377584381417324_usize,0_usize];
_2.fld0 = [(-42_i8),27_i8];
_24 = _8;
_7 = [(-8248_i16),4688_i16,11009_i16,(-13182_i16),9239_i16];
_19 = core::ptr::addr_of_mut!(_25);
_2.fld0 = [64_i8,38_i8];
_19 = core::ptr::addr_of_mut!((*_19));
_21 = _12 as f32;
_14 = _9;
_4 = core::ptr::addr_of!((*_4));
_9 = _14;
_21 = (*_4) as f32;
_28 = _24;
_29 = _16;
Goto(bb10)
}
bb12 = {
RET = !true;
RET = 124927707258256101157698259197863788629_u128 <= 46180993072800802586129817568562252619_u128;
_1 = RET & RET;
RET = _1 | _1;
_2.fld0 = [(-82_i8),58_i8];
RET = !_1;
_3.0 = (-605652807_i32);
_2.fld0 = [107_i8,99_i8];
_2.fld1 = ['\u{e2f6e}','\u{d119c}','\u{3708c}','\u{2e0a3}','\u{66bf6}','\u{6aa4e}','\u{29a14}'];
_3.2 = &_1;
RET = _3.0 <= _3.0;
_4 = core::ptr::addr_of!(_3.0);
Call(_3.0 = fn15(Move(_3.2)), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_3.2 = &RET;
_2.fld0 = [(-91_i8),73_i8];
_2.fld0 = [35_i8,2_i8];
_3.0 = (-152151432562756383048496366262402454783_i128) as i32;
_4 = core::ptr::addr_of!((*_4));
_5 = !672310616_u32;
_2.fld0 = [(-128_i8),102_i8];
_2.fld0 = [76_i8,122_i8];
RET = _1 != _1;
_2.fld0 = [(-56_i8),(-39_i8)];
_6 = -(-116077498278652764681502801511532051031_i128);
_3.2 = &RET;
_3.2 = &RET;
_3.1 = 63723707099176464148905197602176102874_u128;
_3.0 = (-278425702_i32) - (-510108486_i32);
(*_4) = 1518467515_i32 & 507471153_i32;
_1 = RET ^ RET;
_3.2 = &_1;
_6 = 107368855425388973104338229236487900452_i128;
_7 = [(-29448_i16),30984_i16,23127_i16,22654_i16,(-1281_i16)];
_2.fld0 = [(-1_i8),(-71_i8)];
RET = (*_4) != (*_4);
_9 = '\u{f2270}';
_3.2 = &RET;
_5 = 100570022_u32;
_10 = 96_u8 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb14 = {
_3.2 = &RET;
_2.fld0 = [(-91_i8),73_i8];
_2.fld0 = [35_i8,2_i8];
_3.0 = (-152151432562756383048496366262402454783_i128) as i32;
_4 = core::ptr::addr_of!((*_4));
_5 = !672310616_u32;
_2.fld0 = [(-128_i8),102_i8];
_2.fld0 = [76_i8,122_i8];
RET = _1 != _1;
_2.fld0 = [(-56_i8),(-39_i8)];
_6 = -(-116077498278652764681502801511532051031_i128);
_3.2 = &RET;
_3.2 = &RET;
_3.1 = 63723707099176464148905197602176102874_u128;
_3.0 = (-278425702_i32) - (-510108486_i32);
(*_4) = 1518467515_i32 & 507471153_i32;
_1 = RET ^ RET;
_3.2 = &_1;
_6 = 107368855425388973104338229236487900452_i128;
_7 = [(-29448_i16),30984_i16,23127_i16,22654_i16,(-1281_i16)];
_2.fld0 = [(-1_i8),(-71_i8)];
RET = (*_4) != (*_4);
_9 = '\u{f2270}';
_3.2 = &RET;
_5 = 100570022_u32;
_10 = 96_u8 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb15 = {
_1 = RET;
_11 = 57_u8 >> (*_4);
_4 = core::ptr::addr_of!(_3.0);
_5 = !3851062103_u32;
_9 = '\u{939d0}';
_4 = core::ptr::addr_of!((*_4));
_8 = !9223372036854775807_isize;
_3.2 = &_1;
_3.2 = &RET;
_12 = 8132600838249985099_usize as u64;
_6 = (-89556069570718413702292283391675210542_i128);
_14 = _9;
_2.fld0 = [(-16_i8),(-62_i8)];
_3.0 = (-1009480089_i32);
_6 = 119453356529905425555611511156958030035_i128;
match (*_4) {
0 => bb1,
340282366920938463463374607430758731367 => bb4,
_ => bb2
}
}
bb16 = {
_1 = RET;
_11 = 57_u8 >> (*_4);
_4 = core::ptr::addr_of!(_3.0);
_5 = !3851062103_u32;
_9 = '\u{939d0}';
_4 = core::ptr::addr_of!((*_4));
_8 = !9223372036854775807_isize;
_3.2 = &_1;
_3.2 = &RET;
_12 = 8132600838249985099_usize as u64;
_6 = (-89556069570718413702292283391675210542_i128);
_14 = _9;
_2.fld0 = [(-16_i8),(-62_i8)];
_3.0 = (-1009480089_i32);
_6 = 119453356529905425555611511156958030035_i128;
match (*_4) {
0 => bb1,
340282366920938463463374607430758731367 => bb4,
_ => bb2
}
}
bb17 = {
_30 = _9;
_12 = 3678288647746559042_u64 + 11913043649860588839_u64;
_1 = !RET;
_9 = _30;
_28 = _8 >> _32;
_13 = [_6,_6,_6];
_27 = _21 as isize;
_2.fld1 = [_14,_30,_14,_9,_14,_14,_9];
_7 = [24344_i16,25405_i16,16076_i16,30065_i16,(-10784_i16)];
_20 = !RET;
_5 = _10 as u32;
_31 = Adt51 { fld0: _2.fld0,fld1: _2.fld1 };
Goto(bb18)
}
bb18 = {
Call(_36 = dump_var(14_usize, 22_usize, Move(_22), 20_usize, Move(_20), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(14_usize, 28_usize, Move(_28), 6_usize, Move(_6), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_36 = dump_var(14_usize, 14_usize, Move(_14), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static bool) -> i32 {
mir! {
type RET = i32;
let _2: [i64; 4];
let _3: *const *mut (i64, i8, *const u64, [char; 8]);
let _4: [i8; 2];
let _5: Adt73;
let _6: Adt74;
let _7: [char; 5];
let _8: &'static i32;
let _9: (u32, [u8; 8]);
let _10: [i128; 3];
let _11: isize;
let _12: f32;
let _13: Adt74;
let _14: isize;
let _15: [i128; 3];
let _16: Adt46;
let _17: f64;
let _18: i128;
let _19: [i16; 5];
let _20: (*mut *const char, &'static [char; 7], [isize; 4], &'static i32);
let _21: *mut *const char;
let _22: (*const &'static u32, [char; 3], [i128; 4], &'static u16);
let _23: i8;
let _24: [isize; 4];
let _25: Adt73;
let _26: f32;
let _27: *const *mut (i64, i8, *const u64, [char; 8]);
let _28: [char; 5];
let _29: u64;
let _30: bool;
let _31: f32;
let _32: f64;
let _33: u128;
let _34: *const i32;
let _35: *mut *const *mut (i64, i8, *const u64, [char; 8]);
let _36: (u32, [u8; 8]);
let _37: bool;
let _38: *const *mut i128;
let _39: char;
let _40: f32;
let _41: bool;
let _42: ();
let _43: ();
{
RET = 1156381127_i32;
RET = (-905935289_i32) & (-1609534502_i32);
RET = '\u{66a08}' as i32;
RET = 3559_i16 as i32;
RET = 14297703244787079472_u64 as i32;
_2 = [416828577937476708_i64,65652860143725224_i64,8899274948402394634_i64,(-7726343811190645978_i64)];
_4 = [11_i8,(-20_i8)];
_2 = [(-2756475277272633522_i64),6936088984776859918_i64,8147692729706809214_i64,7897247747104337497_i64];
_2 = [5306305728124165032_i64,3481344079816672431_i64,(-186524550258777593_i64),(-2749776724475702588_i64)];
_6 = Adt74::Variant0 { fld0: RET };
RET = 162_u8 as i32;
_7 = ['\u{ef408}','\u{4041c}','\u{c7693}','\u{615ba}','\u{6c131}'];
RET = -Field::<i32>(Variant(_6, 0), 0);
RET = Field::<i32>(Variant(_6, 0), 0);
_2 = [(-6998127594882138780_i64),926352586299433315_i64,(-5801861996429570487_i64),(-1290307454246660595_i64)];
_4 = [35_i8,(-7_i8)];
place!(Field::<i32>(Variant(_6, 0), 0)) = -RET;
RET = !Field::<i32>(Variant(_6, 0), 0);
place!(Field::<i32>(Variant(_6, 0), 0)) = RET - RET;
Goto(bb1)
}
bb1 = {
_7 = ['\u{bfce1}','\u{a2d2e}','\u{cdb00}','\u{7d245}','\u{373c5}'];
_4 = [47_i8,123_i8];
RET = -Field::<i32>(Variant(_6, 0), 0);
place!(Field::<i32>(Variant(_6, 0), 0)) = !RET;
_4 = [39_i8,(-111_i8)];
Goto(bb2)
}
bb2 = {
place!(Field::<i32>(Variant(_6, 0), 0)) = -RET;
_7 = ['\u{fc9e1}','\u{367e}','\u{34daa}','\u{d68b2}','\u{528e0}'];
_8 = &RET;
_9.1 = [50_u8,87_u8,145_u8,134_u8,76_u8,190_u8,44_u8,65_u8];
_12 = 8354_i16 as f32;
SetDiscriminant(_6, 2);
_8 = &(*_8);
_10 = [(-29053929293861532951862815651284744765_i128),(-82460900620356046445160554817272909248_i128),84107209443422189054382238465506724193_i128];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = (-92_i8);
_7 = ['\u{44715}','\u{6b040}','\u{b3a1f}','\u{750ce}','\u{100022}'];
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld0 = [Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1];
Goto(bb3)
}
bb3 = {
RET = 194509189600148874507062323079439282357_u128 as i32;
_7 = ['\u{5c6f8}','\u{2a66e}','\u{a5c04}','\u{7e0d0}','\u{f3e85}'];
place!(Field::<[usize; 7]>(Variant(_6, 2), 3)) = [5_usize,2_usize,15985703874689551498_usize,13256987014439653628_usize,16384402102257282362_usize,1_usize,2_usize];
_12 = 195864324472550493377692531354010585694_u128 as f32;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = 3880916168829686765_i64 + (-2920497117018797181_i64);
_8 = &RET;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = (-49_i8);
_9.0 = 2887161922_u32 ^ 2912127486_u32;
RET = 9223372036854775807_isize as i32;
place!(Field::<bool>(Variant(_6, 2), 0)) = _9.0 > _9.0;
RET = -1092251320_i32;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{1eafa}','\u{168bb}','\u{6143d}','\u{1081e}','\u{36ccc}','\u{a094f}','\u{d6a77}','\u{6e557}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = (-8247471520915400667_i64) >> RET;
_7 = ['\u{a2341}','\u{803a}','\u{c5f36}','\u{1a7b7}','\u{d26f1}'];
_11 = !(-9223372036854775808_isize);
_10 = [(-168372092141552057371573185487810788842_i128),106764077174321346787990009557707795601_i128,(-128683568023435514355037859916119517938_i128)];
RET = !(-1742829871_i32);
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1 = ['\u{3ab71}','\u{a943f}','\u{a460b}','\u{5dd91}','\u{ec1f3}','\u{eece7}','\u{100f2f}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{1d0fd}','\u{1061d2}','\u{a67e4}','\u{edb75}','\u{4a8c4}','\u{924f4}','\u{10c169}','\u{e891c}'];
_15 = [(-133383323896129587583470852585336261379_i128),60934349421920334096199514004831083104_i128,(-82178155796636741401078243743014819439_i128)];
_8 = &RET;
Call(_2 = core::intrinsics::transmute(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = 7_usize as i128;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
Call(place!(Field::<[usize; 7]>(Variant(_6, 2), 3)) = fn16(Move(_8), Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3, _9.1, _10, Field::<bool>(Variant(_6, 2), 0), Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3, _9.1, _9, _2, _9.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17 = 14471614311885722823_usize as f64;
_9.0 = _11 as u32;
_11 = (-89_isize);
_16 = Adt46::Variant1 { fld0: _9.1,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 };
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{470f3}','\u{10007a}','\u{213b1}','\u{b2958}','\u{25724}','\u{3b1c0}','\u{7437}','\u{945b5}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{80113}','\u{2f51c}','\u{abbe8}','\u{2f53b}','\u{4c8be}','\u{600bd}','\u{4c1e8}','\u{f5e50}'];
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_10 = [_18,_18,_18];
_5 = Adt73::Variant1 { fld0: RET,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_15 = [_18,_18,_18];
place!(Field::<i8>(Variant(_16, 1), 1)) = Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 ^ Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1;
_14 = _11 - _11;
_11 = '\u{37db0}' as isize;
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld0 = _4;
_20.3 = &RET;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_11 = !_14;
_7 = ['\u{f5d3d}','\u{d2237}','\u{27f9b}','\u{23b80}','\u{2384b}'];
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
place!(Field::<i8>(Variant(_16, 1), 1)) = Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1;
_18 = -(-33743364059269495979298041180965157244_i128);
_10 = _15;
_9.0 = 415051397_u32 | 1578704276_u32;
_1 = &place!(Field::<bool>(Variant(_6, 2), 0));
place!(Field::<bool>(Variant(_6, 2), 0)) = true;
RET = Field::<i32>(Variant(_5, 1), 0) | Field::<i32>(Variant(_5, 1), 0);
_14 = _11 - _11;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
place!(Field::<[usize; 7]>(Variant(_6, 2), 3)) = [11474005440834053341_usize,10620787159139171291_usize,6_usize,311065904385524349_usize,6_usize,10976544256001112300_usize,3_usize];
match Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 {
340282366920938463463374607431768211407 => bb6,
_ => bb3
}
}
bb6 = {
RET = '\u{2e2a6}' as i32;
_10 = [_18,_18,_18];
_1 = &place!(Field::<bool>(Variant(_6, 2), 0));
SetDiscriminant(_16, 0);
_20.2 = [_11,_11,_14,_14];
place!(Field::<i8>(Variant(_16, 0), 3)) = Field::<i32>(Variant(_5, 1), 0) as i8;
_2 = [Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0];
place!(Field::<bool>(Variant(_6, 2), 0)) = true ^ false;
place!(Field::<i8>(Variant(_16, 0), 3)) = (-25546_i16) as i8;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = -Field::<i8>(Variant(_16, 0), 3);
SetDiscriminant(_5, 0);
_20.2 = [_14,_14,_11,_14];
_11 = -_14;
_8 = &RET;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_20.3 = &RET;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = -4258105219759389597_i64;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
place!(Field::<[i16; 5]>(Variant(_16, 0), 0)) = [8035_i16,14892_i16,26816_i16,24616_i16,24063_i16];
_19 = [(-12959_i16),356_i16,(-863_i16),32672_i16,(-27857_i16)];
Goto(bb7)
}
bb7 = {
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1 = ['\u{b6596}','\u{c3f53}','\u{2aaea}','\u{1bb86}','\u{f1233}','\u{35038}','\u{ecf03}'];
_2 = [Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0];
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
_25 = Adt73::Variant1 { fld0: (*_8),fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_5 = Adt73::Variant1 { fld0: RET,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_22.2 = [_18,_18,_18,_18];
_26 = _12 - _12;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
_23 = Field::<i8>(Variant(_16, 0), 3) & Field::<i8>(Variant(_16, 0), 3);
_28 = _7;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).2 = core::ptr::addr_of!(_29);
SetDiscriminant(_6, 0);
_14 = _11 ^ _11;
SetDiscriminant(_25, 0);
_10 = [_18,_18,_18];
Goto(bb8)
}
bb8 = {
_20.3 = Move(_8);
place!(Field::<u128>(Variant(_16, 0), 2)) = _18 as u128;
_4 = [_23,Field::<i8>(Variant(_16, 0), 3)];
Goto(bb9)
}
bb9 = {
RET = false as i32;
place!(Field::<[i8; 2]>(Variant(_25, 0), 1)) = _4;
_30 = false;
RET = Field::<i32>(Variant(_5, 1), 0) << _18;
_18 = (-133767233378775600336428514090399815274_i128);
_18 = (-118131861159390182198671145405363303498_i128);
_14 = '\u{9acb}' as isize;
place!(Field::<[i8; 2]>(Variant(_25, 0), 1)) = [_23,_23];
_20.2 = [_11,_11,_11,_14];
Goto(bb10)
}
bb10 = {
SetDiscriminant(_5, 1);
place!(Field::<char>(Variant(_16, 0), 1)) = '\u{100ced}';
_22.1 = [Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1)];
_32 = 11806_i16 as f64;
_32 = 78_u8 as f64;
_31 = RET as f32;
_31 = 34401_u16 as f32;
_12 = _26;
_9.0 = 3428101994_u32 * 517207738_u32;
place!(Field::<i32>(Variant(_6, 0), 0)) = RET + RET;
_7 = [Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1)];
match _18 {
0 => bb4,
1 => bb7,
2 => bb9,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
222150505761548281264703462026404907958 => bb16,
_ => bb15
}
}
bb11 = {
_7 = ['\u{bfce1}','\u{a2d2e}','\u{cdb00}','\u{7d245}','\u{373c5}'];
_4 = [47_i8,123_i8];
RET = -Field::<i32>(Variant(_6, 0), 0);
place!(Field::<i32>(Variant(_6, 0), 0)) = !RET;
_4 = [39_i8,(-111_i8)];
Goto(bb2)
}
bb12 = {
RET = 194509189600148874507062323079439282357_u128 as i32;
_7 = ['\u{5c6f8}','\u{2a66e}','\u{a5c04}','\u{7e0d0}','\u{f3e85}'];
place!(Field::<[usize; 7]>(Variant(_6, 2), 3)) = [5_usize,2_usize,15985703874689551498_usize,13256987014439653628_usize,16384402102257282362_usize,1_usize,2_usize];
_12 = 195864324472550493377692531354010585694_u128 as f32;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = 3880916168829686765_i64 + (-2920497117018797181_i64);
_8 = &RET;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = (-49_i8);
_9.0 = 2887161922_u32 ^ 2912127486_u32;
RET = 9223372036854775807_isize as i32;
place!(Field::<bool>(Variant(_6, 2), 0)) = _9.0 > _9.0;
RET = -1092251320_i32;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{1eafa}','\u{168bb}','\u{6143d}','\u{1081e}','\u{36ccc}','\u{a094f}','\u{d6a77}','\u{6e557}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = (-8247471520915400667_i64) >> RET;
_7 = ['\u{a2341}','\u{803a}','\u{c5f36}','\u{1a7b7}','\u{d26f1}'];
_11 = !(-9223372036854775808_isize);
_10 = [(-168372092141552057371573185487810788842_i128),106764077174321346787990009557707795601_i128,(-128683568023435514355037859916119517938_i128)];
RET = !(-1742829871_i32);
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1 = ['\u{3ab71}','\u{a943f}','\u{a460b}','\u{5dd91}','\u{ec1f3}','\u{eece7}','\u{100f2f}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{1d0fd}','\u{1061d2}','\u{a67e4}','\u{edb75}','\u{4a8c4}','\u{924f4}','\u{10c169}','\u{e891c}'];
_15 = [(-133383323896129587583470852585336261379_i128),60934349421920334096199514004831083104_i128,(-82178155796636741401078243743014819439_i128)];
_8 = &RET;
Call(_2 = core::intrinsics::transmute(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1 = ['\u{b6596}','\u{c3f53}','\u{2aaea}','\u{1bb86}','\u{f1233}','\u{35038}','\u{ecf03}'];
_2 = [Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0];
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
_25 = Adt73::Variant1 { fld0: (*_8),fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_5 = Adt73::Variant1 { fld0: RET,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_22.2 = [_18,_18,_18,_18];
_26 = _12 - _12;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
_23 = Field::<i8>(Variant(_16, 0), 3) & Field::<i8>(Variant(_16, 0), 3);
_28 = _7;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).2 = core::ptr::addr_of!(_29);
SetDiscriminant(_6, 0);
_14 = _11 ^ _11;
SetDiscriminant(_25, 0);
_10 = [_18,_18,_18];
Goto(bb8)
}
bb14 = {
RET = '\u{2e2a6}' as i32;
_10 = [_18,_18,_18];
_1 = &place!(Field::<bool>(Variant(_6, 2), 0));
SetDiscriminant(_16, 0);
_20.2 = [_11,_11,_14,_14];
place!(Field::<i8>(Variant(_16, 0), 3)) = Field::<i32>(Variant(_5, 1), 0) as i8;
_2 = [Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0,Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).0];
place!(Field::<bool>(Variant(_6, 2), 0)) = true ^ false;
place!(Field::<i8>(Variant(_16, 0), 3)) = (-25546_i16) as i8;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = -Field::<i8>(Variant(_16, 0), 3);
SetDiscriminant(_5, 0);
_20.2 = [_14,_14,_11,_14];
_11 = -_14;
_8 = &RET;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_20.3 = &RET;
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).0 = -4258105219759389597_i64;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
place!(Field::<[i16; 5]>(Variant(_16, 0), 0)) = [8035_i16,14892_i16,26816_i16,24616_i16,24063_i16];
_19 = [(-12959_i16),356_i16,(-863_i16),32672_i16,(-27857_i16)];
Goto(bb7)
}
bb15 = {
_17 = 14471614311885722823_usize as f64;
_9.0 = _11 as u32;
_11 = (-89_isize);
_16 = Adt46::Variant1 { fld0: _9.1,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 };
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{470f3}','\u{10007a}','\u{213b1}','\u{b2958}','\u{25724}','\u{3b1c0}','\u{7437}','\u{945b5}'];
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).3 = ['\u{80113}','\u{2f51c}','\u{abbe8}','\u{2f53b}','\u{4c8be}','\u{600bd}','\u{4c1e8}','\u{f5e50}'];
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_10 = [_18,_18,_18];
_5 = Adt73::Variant1 { fld0: RET,fld1: Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).3 };
_15 = [_18,_18,_18];
place!(Field::<i8>(Variant(_16, 1), 1)) = Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 ^ Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1;
_14 = _11 - _11;
_11 = '\u{37db0}' as isize;
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld0 = _4;
_20.3 = &RET;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_11 = !_14;
_7 = ['\u{f5d3d}','\u{d2237}','\u{27f9b}','\u{23b80}','\u{2384b}'];
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
place!(Field::<i8>(Variant(_16, 1), 1)) = Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1;
_18 = -(-33743364059269495979298041180965157244_i128);
_10 = _15;
_9.0 = 415051397_u32 | 1578704276_u32;
_1 = &place!(Field::<bool>(Variant(_6, 2), 0));
place!(Field::<bool>(Variant(_6, 2), 0)) = true;
RET = Field::<i32>(Variant(_5, 1), 0) | Field::<i32>(Variant(_5, 1), 0);
_14 = _11 - _11;
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
place!(Field::<[usize; 7]>(Variant(_6, 2), 3)) = [11474005440834053341_usize,10620787159139171291_usize,6_usize,311065904385524349_usize,6_usize,10976544256001112300_usize,3_usize];
match Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2).1 {
340282366920938463463374607431768211407 => bb6,
_ => bb3
}
}
bb16 = {
_7 = _28;
_36 = (_9.0, _9.1);
_24 = [_11,_11,_11,_14];
place!(Field::<[i16; 5]>(Variant(_16, 0), 0)) = [(-7922_i16),(-3230_i16),(-16866_i16),32138_i16,(-11499_i16)];
SetDiscriminant(_6, 2);
place!(Field::<i32>(Variant(_5, 1), 0)) = -RET;
_18 = RET as i128;
place!(Field::<[u32; 7]>(Variant(_6, 2), 1)) = [_9.0,_9.0,_36.0,_36.0,_36.0,_9.0,_36.0];
_37 = !_30;
_35 = core::ptr::addr_of_mut!(_27);
place!(Field::<(i64, i8, *const u64, [char; 8])>(Variant(_6, 2), 2)).1 = !_23;
_8 = &place!(Field::<i32>(Variant(_5, 1), 0));
_20.1 = &place!(Field::<Adt51>(Variant(_6, 2), 4)).fld1;
place!(Field::<char>(Variant(_16, 0), 1)) = '\u{332d6}';
_20.3 = &(*_8);
_36.0 = _9.0 | _9.0;
_39 = Field::<char>(Variant(_16, 0), 1);
_33 = Field::<u128>(Variant(_16, 0), 2) + Field::<u128>(Variant(_16, 0), 2);
place!(Field::<Adt51>(Variant(_6, 2), 4)).fld0 = _4;
_24 = [_11,_11,_11,_14];
_22.1 = [Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1)];
place!(Field::<[char; 8]>(Variant(_5, 1), 1)) = [Field::<char>(Variant(_16, 0), 1),_39,_39,Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),_39,_39];
_20.2 = [_11,_11,_11,_11];
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(15_usize, 19_usize, Move(_19), 7_usize, Move(_7), 24_usize, Move(_24), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(15_usize, 11_usize, Move(_11), 4_usize, Move(_4), 15_usize, Move(_15), 37_usize, Move(_37)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(15_usize, 30_usize, Move(_30), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: &'static i32,mut _2: [char; 8],mut _3: [u8; 8],mut _4: [i128; 3],mut _5: bool,mut _6: [char; 8],mut _7: [u8; 8],mut _8: (u32, [u8; 8]),mut _9: [i64; 4],mut _10: [u8; 8]) -> [usize; 7] {
mir! {
type RET = [usize; 7];
let _11: [bool; 3];
let _12: &'static *mut i128;
let _13: [i16; 5];
let _14: isize;
let _15: [bool; 3];
let _16: isize;
let _17: i8;
let _18: [char; 5];
let _19: char;
let _20: [char; 8];
let _21: *mut (i64, i8, *const u64, [char; 8]);
let _22: Adt74;
let _23: [bool; 3];
let _24: *mut (i32, u128, &'static bool);
let _25: char;
let _26: Adt55;
let _27: *mut u16;
let _28: &'static *mut i128;
let _29: [usize; 7];
let _30: f64;
let _31: *const char;
let _32: [u32; 7];
let _33: &'static (f64,);
let _34: char;
let _35: bool;
let _36: *mut (i64, i8, *const u64, [char; 8]);
let _37: u8;
let _38: f32;
let _39: ();
let _40: ();
{
RET = [17630928760584228512_usize,8310677834058456177_usize,5_usize,7_usize,5_usize,1_usize,2119080339125914870_usize];
_8.1 = [11_u8,73_u8,250_u8,14_u8,180_u8,91_u8,170_u8,26_u8];
_4 = [1222685803917258602003628710809049205_i128,(-6053112878715268005326623647163589702_i128),63380647209821945106819139112257784831_i128];
_5 = !false;
_8 = (718454058_u32, _3);
RET = [5_usize,1_usize,17698291572349296330_usize,2_usize,2_usize,16300519127540097693_usize,3377728019039830996_usize];
_7 = [50_u8,41_u8,153_u8,251_u8,189_u8,211_u8,39_u8,70_u8];
_13 = [277_i16,(-23844_i16),(-22931_i16),(-17582_i16),(-30989_i16)];
_8 = (3145801817_u32, _7);
_8.1 = _7;
_10 = [254_u8,170_u8,69_u8,56_u8,52_u8,153_u8,73_u8,86_u8];
_6 = ['\u{e22fc}','\u{4312c}','\u{77141}','\u{cc8df}','\u{42112}','\u{6b01f}','\u{1ee71}','\u{dd9f7}'];
_9 = [(-5976768806487725867_i64),(-7729513453558695650_i64),(-8408164509890981411_i64),5763678850464618544_i64];
_8.0 = !2957056544_u32;
_7 = [70_u8,102_u8,244_u8,81_u8,129_u8,38_u8,35_u8,61_u8];
_3 = [129_u8,56_u8,181_u8,147_u8,53_u8,167_u8,121_u8,250_u8];
RET = [14071103284270266228_usize,7_usize,12316603266363346813_usize,7_usize,8629083159961327940_usize,3_usize,2_usize];
_3 = _10;
_3 = _7;
_6 = ['\u{d98d2}','\u{f3382}','\u{7e5f7}','\u{42ddc}','\u{b07d9}','\u{e2354}','\u{a4516}','\u{5310}'];
_7 = _3;
Goto(bb1)
}
bb1 = {
_8.1 = _10;
_5 = !false;
_2 = ['\u{5d78}','\u{67bd5}','\u{4c432}','\u{32762}','\u{2595b}','\u{84276}','\u{3963e}','\u{98f51}'];
_11 = [_5,_5,_5];
_6 = _2;
_9 = [(-7159317414658774912_i64),(-4750205116024871981_i64),(-1815481115617875139_i64),4022025693448490739_i64];
RET = [2305152267633667882_usize,10843573373358671236_usize,1_usize,16766214069082424169_usize,9696284659071539640_usize,3_usize,15093283749370008224_usize];
_8.1 = _10;
_10 = [154_u8,215_u8,95_u8,31_u8,35_u8,75_u8,51_u8,98_u8];
_8.1 = [168_u8,121_u8,78_u8,36_u8,160_u8,170_u8,41_u8,63_u8];
_5 = !true;
_7 = [148_u8,54_u8,175_u8,59_u8,128_u8,116_u8,105_u8,208_u8];
_16 = -(-9223372036854775808_isize);
_15 = [_5,_5,_5];
_4 = [(-16546238577635885292941496227967383498_i128),(-64030024726175670550129892511580631591_i128),21254475927192842133572901226221183900_i128];
_6 = ['\u{b266c}','\u{a7b8}','\u{5af53}','\u{8b2ba}','\u{c2392}','\u{6054d}','\u{ceeaa}','\u{cecf3}'];
_8 = (1561816408_u32, _10);
_9 = [(-2231954433586322225_i64),(-7161402612024132452_i64),(-4372860431505091540_i64),9170326406014497657_i64];
_14 = 139_u8 as isize;
_16 = -_14;
_8.0 = 3502270571_u32;
_8.1 = [251_u8,255_u8,200_u8,131_u8,135_u8,97_u8,180_u8,37_u8];
_14 = _16 * _16;
_15 = [_5,_5,_5];
Call(_5 = fn17(_3, _14, _10, _8, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = _11;
_9 = [2528852829617078167_i64,(-1212820828672126592_i64),8975877928405136333_i64,1059434174742155722_i64];
_5 = _14 > _16;
_8.0 = 405039234_u32;
_8.1 = [146_u8,210_u8,183_u8,190_u8,120_u8,243_u8,11_u8,129_u8];
_17 = -(-114_i8);
_19 = '\u{f5f7e}';
_14 = _16 * _16;
match _8.0 {
0 => bb1,
1 => bb3,
405039234 => bb5,
_ => bb4
}
}
bb3 = {
_8.1 = _10;
_5 = !false;
_2 = ['\u{5d78}','\u{67bd5}','\u{4c432}','\u{32762}','\u{2595b}','\u{84276}','\u{3963e}','\u{98f51}'];
_11 = [_5,_5,_5];
_6 = _2;
_9 = [(-7159317414658774912_i64),(-4750205116024871981_i64),(-1815481115617875139_i64),4022025693448490739_i64];
RET = [2305152267633667882_usize,10843573373358671236_usize,1_usize,16766214069082424169_usize,9696284659071539640_usize,3_usize,15093283749370008224_usize];
_8.1 = _10;
_10 = [154_u8,215_u8,95_u8,31_u8,35_u8,75_u8,51_u8,98_u8];
_8.1 = [168_u8,121_u8,78_u8,36_u8,160_u8,170_u8,41_u8,63_u8];
_5 = !true;
_7 = [148_u8,54_u8,175_u8,59_u8,128_u8,116_u8,105_u8,208_u8];
_16 = -(-9223372036854775808_isize);
_15 = [_5,_5,_5];
_4 = [(-16546238577635885292941496227967383498_i128),(-64030024726175670550129892511580631591_i128),21254475927192842133572901226221183900_i128];
_6 = ['\u{b266c}','\u{a7b8}','\u{5af53}','\u{8b2ba}','\u{c2392}','\u{6054d}','\u{ceeaa}','\u{cecf3}'];
_8 = (1561816408_u32, _10);
_9 = [(-2231954433586322225_i64),(-7161402612024132452_i64),(-4372860431505091540_i64),9170326406014497657_i64];
_14 = 139_u8 as isize;
_16 = -_14;
_8.0 = 3502270571_u32;
_8.1 = [251_u8,255_u8,200_u8,131_u8,135_u8,97_u8,180_u8,37_u8];
_14 = _16 * _16;
_15 = [_5,_5,_5];
Call(_5 = fn17(_3, _14, _10, _8, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_18 = [_19,_19,_19,_19,_19];
_4 = [4727396700122301132923326884132485653_i128,148696137008335020035996702485101620092_i128,(-144354301529908773914219393557942491950_i128)];
_13 = [8354_i16,(-15416_i16),11580_i16,7171_i16,(-6507_i16)];
_17 = _5 as i8;
_7 = _8.1;
_17 = 2132359606944827968_u64 as i8;
_9 = [1700983185063673170_i64,958960851769093394_i64,(-3391325304656514780_i64),(-1418616563456372726_i64)];
_8 = (3551887898_u32, _7);
_4 = [151840450436007747179768468271961614132_i128,(-686500538721077040393569115690357614_i128),159899496640016482560713463167437388962_i128];
_16 = (-470317891_i32) as isize;
match _8.0 {
0 => bb1,
1 => bb2,
3551887898 => bb6,
_ => bb4
}
}
bb6 = {
_16 = _14;
_6 = [_19,_19,_19,_19,_19,_19,_19,_19];
_18 = [_19,_19,_19,_19,_19];
_10 = [149_u8,174_u8,164_u8,75_u8,110_u8,118_u8,134_u8,253_u8];
_4 = [(-118499496093374904746036993834238279511_i128),119622347901162320275493998266818813577_i128,(-27806950908950458079688313971351300231_i128)];
_9 = [(-7779969878812651963_i64),(-3663332630905414377_i64),1499935571259489873_i64,(-605494593961537158_i64)];
_4 = [(-51787827287359292982137268726925903219_i128),(-154919230603645758538587339405255354037_i128),161413457883440098005009658002878350824_i128];
_20 = [_19,_19,_19,_19,_19,_19,_19,_19];
RET = [6_usize,11386506272406704024_usize,10156413820045806575_usize,6_usize,4362534693142288196_usize,4_usize,7_usize];
match _8.0 {
0 => bb4,
1 => bb5,
2 => bb3,
3 => bb7,
3551887898 => bb9,
_ => bb8
}
}
bb7 = {
_18 = [_19,_19,_19,_19,_19];
_4 = [4727396700122301132923326884132485653_i128,148696137008335020035996702485101620092_i128,(-144354301529908773914219393557942491950_i128)];
_13 = [8354_i16,(-15416_i16),11580_i16,7171_i16,(-6507_i16)];
_17 = _5 as i8;
_7 = _8.1;
_17 = 2132359606944827968_u64 as i8;
_9 = [1700983185063673170_i64,958960851769093394_i64,(-3391325304656514780_i64),(-1418616563456372726_i64)];
_8 = (3551887898_u32, _7);
_4 = [151840450436007747179768468271961614132_i128,(-686500538721077040393569115690357614_i128),159899496640016482560713463167437388962_i128];
_16 = (-470317891_i32) as isize;
match _8.0 {
0 => bb1,
1 => bb2,
3551887898 => bb6,
_ => bb4
}
}
bb8 = {
_8.1 = _10;
_5 = !false;
_2 = ['\u{5d78}','\u{67bd5}','\u{4c432}','\u{32762}','\u{2595b}','\u{84276}','\u{3963e}','\u{98f51}'];
_11 = [_5,_5,_5];
_6 = _2;
_9 = [(-7159317414658774912_i64),(-4750205116024871981_i64),(-1815481115617875139_i64),4022025693448490739_i64];
RET = [2305152267633667882_usize,10843573373358671236_usize,1_usize,16766214069082424169_usize,9696284659071539640_usize,3_usize,15093283749370008224_usize];
_8.1 = _10;
_10 = [154_u8,215_u8,95_u8,31_u8,35_u8,75_u8,51_u8,98_u8];
_8.1 = [168_u8,121_u8,78_u8,36_u8,160_u8,170_u8,41_u8,63_u8];
_5 = !true;
_7 = [148_u8,54_u8,175_u8,59_u8,128_u8,116_u8,105_u8,208_u8];
_16 = -(-9223372036854775808_isize);
_15 = [_5,_5,_5];
_4 = [(-16546238577635885292941496227967383498_i128),(-64030024726175670550129892511580631591_i128),21254475927192842133572901226221183900_i128];
_6 = ['\u{b266c}','\u{a7b8}','\u{5af53}','\u{8b2ba}','\u{c2392}','\u{6054d}','\u{ceeaa}','\u{cecf3}'];
_8 = (1561816408_u32, _10);
_9 = [(-2231954433586322225_i64),(-7161402612024132452_i64),(-4372860431505091540_i64),9170326406014497657_i64];
_14 = 139_u8 as isize;
_16 = -_14;
_8.0 = 3502270571_u32;
_8.1 = [251_u8,255_u8,200_u8,131_u8,135_u8,97_u8,180_u8,37_u8];
_14 = _16 * _16;
_15 = [_5,_5,_5];
Call(_5 = fn17(_3, _14, _10, _8, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_9 = [(-5788941062825982527_i64),(-2732751081321731759_i64),(-1284312736063370683_i64),(-101275644333872654_i64)];
_9 = [(-3993827071312061505_i64),5691749446627201123_i64,(-6716524435863473543_i64),(-2766696754357679575_i64)];
_9 = [7315711629383141462_i64,(-1237611667125382849_i64),7202432704036720962_i64,5708007947040014468_i64];
_10 = [253_u8,44_u8,105_u8,225_u8,18_u8,31_u8,8_u8,228_u8];
_23 = [_5,_5,_5];
_14 = 39059_u16 as isize;
_9 = [5074915075633488130_i64,(-2028730855989240390_i64),3252963298953159054_i64,8110664140586018117_i64];
_8.0 = 3771410922_u32;
_8.0 = !3447279027_u32;
Goto(bb10)
}
bb10 = {
_15 = [_5,_5,_5];
_11 = [_5,_5,_5];
Goto(bb11)
}
bb11 = {
RET = [6_usize,1475463802483504986_usize,5_usize,7_usize,4_usize,13674809830280119518_usize,7_usize];
RET = [5_usize,6_usize,2_usize,694136778941674433_usize,9253433834643594583_usize,12684544926586779696_usize,5669973961366357213_usize];
_9 = [1772966261047691990_i64,(-7934125125911337464_i64),(-356361643639138981_i64),4340413798291121862_i64];
_15 = [_5,_5,_5];
_20 = _2;
RET = [5_usize,0_usize,174045571539264872_usize,7_usize,4_usize,6_usize,3_usize];
_8.1 = [40_u8,226_u8,202_u8,149_u8,193_u8,28_u8,168_u8,21_u8];
_5 = !true;
_18 = [_19,_19,_19,_19,_19];
RET = [18195824585999367760_usize,10213940678585087028_usize,3008563798822119842_usize,5412113531424741009_usize,1_usize,5_usize,6241193250427386455_usize];
RET = [9524986265462317868_usize,8713447984679726730_usize,18110422358258017717_usize,2_usize,12338646448747374299_usize,17044260302680424246_usize,6_usize];
Goto(bb12)
}
bb12 = {
_19 = '\u{fc813}';
_25 = _19;
_6 = [_19,_19,_19,_25,_19,_19,_19,_25];
_8 = (559422349_u32, _3);
_14 = _25 as isize;
_2 = [_25,_25,_25,_25,_19,_25,_25,_25];
_5 = true;
_9 = [2513431801394674507_i64,(-7781330243057218522_i64),(-2237807106771894755_i64),9168156029806033420_i64];
_29 = [10850009088254822159_usize,2_usize,3_usize,1_usize,5_usize,0_usize,4767378579687113450_usize];
_14 = _16 * _16;
_30 = _17 as f64;
Goto(bb13)
}
bb13 = {
_4 = [(-11649485960724219848043943385723429487_i128),155862855931202904841584831460790988734_i128,(-117057748039512604779519462521881768160_i128)];
_13 = [757_i16,(-29430_i16),(-4232_i16),(-22521_i16),11955_i16];
_22 = Adt74::Variant0 { fld0: 1873996048_i32 };
_30 = 52_u8 as f64;
_6 = [_25,_25,_19,_19,_19,_25,_25,_25];
RET = [1_usize,1_usize,15582300496669766962_usize,4_usize,10689436749258119345_usize,5_usize,7_usize];
_23 = _15;
_7 = [13_u8,150_u8,36_u8,46_u8,47_u8,249_u8,205_u8,16_u8];
_14 = _16 - _16;
RET = [4_usize,0_usize,3763589250532685052_usize,4_usize,1014466407722485577_usize,3_usize,6978838782115585949_usize];
_18 = [_25,_19,_19,_19,_19];
_13 = [18068_i16,(-4584_i16),(-31183_i16),13004_i16,(-26076_i16)];
_8.1 = [66_u8,202_u8,217_u8,108_u8,183_u8,130_u8,106_u8,35_u8];
_19 = _25;
_32 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_29 = [7_usize,16770847767696065201_usize,4_usize,12337703341053736563_usize,3646206701124324334_usize,6_usize,10738806277873910333_usize];
RET = [7_usize,5024132616286668654_usize,17429049732386961478_usize,8723919917268897139_usize,4530620444469537517_usize,18430115285188585390_usize,12364450789817774329_usize];
RET = _29;
place!(Field::<i32>(Variant(_22, 0), 0)) = _16 as i32;
_9 = [(-3211736423694607409_i64),(-1912358105664150243_i64),(-17150753742656118_i64),(-8972745828949095054_i64)];
Goto(bb14)
}
bb14 = {
SetDiscriminant(_22, 0);
_7 = _3;
place!(Field::<i32>(Variant(_22, 0), 0)) = 1597076452_i32 + 696727669_i32;
_25 = _19;
_1 = &place!(Field::<i32>(Variant(_22, 0), 0));
RET = [7_usize,11191829943210753390_usize,8092174959961854289_usize,2312955741049354632_usize,2_usize,17655157355697422926_usize,4_usize];
_3 = [169_u8,109_u8,177_u8,102_u8,142_u8,35_u8,22_u8,199_u8];
_31 = core::ptr::addr_of!(_25);
_8.0 = !2147543778_u32;
_10 = _3;
_23 = [_5,_5,_5];
_6 = [(*_31),_19,(*_31),(*_31),_19,_25,_25,(*_31)];
_3 = [50_u8,25_u8,144_u8,172_u8,70_u8,3_u8,1_u8,129_u8];
_13 = [13933_i16,(-4916_i16),29593_i16,(-29841_i16),14743_i16];
_2 = [_19,_25,_25,_25,_25,(*_31),(*_31),_19];
RET = [8437330201472093205_usize,618743004487369569_usize,3_usize,0_usize,7314166476406169316_usize,1_usize,2005094788866964404_usize];
_8.0 = !2432683059_u32;
_11 = [_5,_5,_5];
_7 = _10;
_14 = -_16;
_23 = _11;
_29 = [3462313627478774865_usize,17908451030112553836_usize,606434317607033291_usize,3_usize,6_usize,14320352873331540368_usize,0_usize];
_30 = (*_1) as f64;
_35 = _5;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(16_usize, 7_usize, Move(_7), 16_usize, Move(_16), 20_usize, Move(_20), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(16_usize, 6_usize, Move(_6), 32_usize, Move(_32), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(16_usize, 15_usize, Move(_15), 10_usize, Move(_10), 17_usize, Move(_17), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [u8; 8],mut _2: isize,mut _3: [u8; 8],mut _4: (u32, [u8; 8]),mut _5: [char; 8]) -> bool {
mir! {
type RET = bool;
let _6: ([i128; 4], ([i128; 3], bool, [char; 7], isize));
let _7: *mut (i32, u128, &'static bool);
let _8: char;
let _9: &'static u32;
let _10: bool;
let _11: f64;
let _12: f64;
let _13: [char; 5];
let _14: (i64, u64);
let _15: [isize; 4];
let _16: isize;
let _17: f32;
let _18: &'static *const (i32, u128, &'static bool);
let _19: i64;
let _20: char;
let _21: u32;
let _22: ();
let _23: ();
{
_1 = [215_u8,219_u8,47_u8,233_u8,6_u8,129_u8,53_u8,98_u8];
_4 = (3405387075_u32, _3);
_5 = ['\u{24d43}','\u{4659d}','\u{c90e3}','\u{eabe8}','\u{570ce}','\u{95fb1}','\u{bc83b}','\u{223f2}'];
RET = true;
_4.0 = (-5822008077333357622_i64) as u32;
_4.0 = 268506346_u32;
_1 = _4.1;
_3 = [171_u8,124_u8,186_u8,239_u8,226_u8,82_u8,157_u8,100_u8];
_3 = [103_u8,106_u8,228_u8,88_u8,155_u8,252_u8,50_u8,152_u8];
_6.1.2 = ['\u{f9c05}','\u{f6cee}','\u{90ab6}','\u{1b45f}','\u{2c211}','\u{86389}','\u{b0229}'];
_4.1 = [31_u8,199_u8,72_u8,59_u8,250_u8,13_u8,34_u8,82_u8];
_6.1.1 = !RET;
Goto(bb1)
}
bb1 = {
RET = _6.1.1;
_4.0 = !2941116652_u32;
_4.0 = 3685596299_u32 - 1830052014_u32;
_9 = &_4.0;
_6.0 = [146794544840189411791527706296589213446_i128,(-118281737690715903753529182790365925802_i128),87949039813049676285564503553579258951_i128,(-139587942915307428803165098933017542083_i128)];
_6.1.0 = [30357202816763378314680299768394733465_i128,168928579307179301123928257755412311101_i128,142549419602086939421169684213744928458_i128];
_6.0 = [41094529290517325091691682405847460178_i128,(-57701280742558056632465832995568139108_i128),(-164957419678321162777573938935977623271_i128),(-19776622248359453046014646785441381832_i128)];
_6.1.3 = _2;
_3 = [225_u8,76_u8,29_u8,66_u8,230_u8,124_u8,126_u8,216_u8];
_4.1 = [255_u8,192_u8,50_u8,70_u8,160_u8,248_u8,111_u8,9_u8];
_3 = _1;
Call(_6.1.3 = fn18(Move(_9), _4, _6.1.0, RET, _3, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = '\u{58269}';
RET = !_6.1.1;
_10 = _6.1.1;
_6.1.3 = _2;
_9 = &_4.0;
RET = _2 > _2;
_1 = [83_u8,162_u8,52_u8,141_u8,105_u8,5_u8,15_u8,46_u8];
_6.1.2 = [_8,_8,_8,_8,_8,_8,_8];
_11 = 846932169433738548_i64 as f64;
_4 = (896753503_u32, _1);
_6.1.2 = [_8,_8,_8,_8,_8,_8,_8];
_2 = _6.1.3;
_10 = RET;
_4 = (4191268553_u32, _1);
_3 = _4.1;
match _4.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
4191268553 => bb10,
_ => bb9
}
}
bb3 = {
RET = _6.1.1;
_4.0 = !2941116652_u32;
_4.0 = 3685596299_u32 - 1830052014_u32;
_9 = &_4.0;
_6.0 = [146794544840189411791527706296589213446_i128,(-118281737690715903753529182790365925802_i128),87949039813049676285564503553579258951_i128,(-139587942915307428803165098933017542083_i128)];
_6.1.0 = [30357202816763378314680299768394733465_i128,168928579307179301123928257755412311101_i128,142549419602086939421169684213744928458_i128];
_6.0 = [41094529290517325091691682405847460178_i128,(-57701280742558056632465832995568139108_i128),(-164957419678321162777573938935977623271_i128),(-19776622248359453046014646785441381832_i128)];
_6.1.3 = _2;
_3 = [225_u8,76_u8,29_u8,66_u8,230_u8,124_u8,126_u8,216_u8];
_4.1 = [255_u8,192_u8,50_u8,70_u8,160_u8,248_u8,111_u8,9_u8];
_3 = _1;
Call(_6.1.3 = fn18(Move(_9), _4, _6.1.0, RET, _3, _3, _3), ReturnTo(bb2), UnwindUnreachable())
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
_6.0 = [(-25707429470752778751472666518318475488_i128),(-51433908128271269537652167437790157683_i128),(-167350928036052794835129221775356810191_i128),72653421045955945892833291245115477954_i128];
_6.0 = [(-122494751191955754837545485493563576731_i128),(-147969205173514646457954350504804335523_i128),109234961984186471356238318717376587087_i128,(-144652418471742751400156376280119395665_i128)];
_4 = (2724939474_u32, _1);
_8 = '\u{732e5}';
_6.0 = [(-139958911693948303002489593422911014733_i128),(-92046352631841118063339541435675224358_i128),(-42302271509438759095186747809285843857_i128),(-4714035732850646431114633302965873721_i128)];
_2 = _6.1.3 + _6.1.3;
_8 = '\u{35865}';
_15 = [_2,_2,_2,_2];
_1 = [107_u8,74_u8,170_u8,198_u8,138_u8,124_u8,251_u8,165_u8];
_3 = [148_u8,206_u8,166_u8,118_u8,73_u8,7_u8,133_u8,65_u8];
_14.0 = (-7657594696366125677_i64) ^ 4342091032599904081_i64;
_6.1.0 = [(-40857607949432535708890543929725339663_i128),71755331119922580083121772353579384623_i128,(-140932453377065057264558030876972806229_i128)];
_15 = [_2,_6.1.3,_6.1.3,_2];
_8 = '\u{dc30c}';
_14.1 = RET as u64;
_11 = _14.1 as f64;
match _4.0 {
2724939474 => bb11,
_ => bb5
}
}
bb11 = {
_6.1.3 = _2 * _2;
_9 = &_4.0;
_11 = 22988_u16 as f64;
_16 = _2 << _4.0;
_14 = ((-292813908909374266_i64), 1624620989925847410_u64);
_5 = [_8,_8,_8,_8,_8,_8,_8,_8];
_2 = _16;
_17 = 68_u8 as f32;
_12 = _11 * _11;
_4 = (3187712664_u32, _1);
_14 = ((-7768685822920340223_i64), 13216615495079662030_u64);
_6.0 = [118588331829447145944528487852925392963_i128,(-84086257563713669710156028578871777581_i128),164003083606508934499705130170488926141_i128,(-120747324569484299636187766510194129230_i128)];
_11 = 27592_u16 as f64;
_20 = _8;
_4.1 = _3;
_14 = (7922194094262351176_i64, 16587732831397088269_u64);
RET = _4.0 >= _4.0;
_6.0 = [105214269223748025757332109618706905537_i128,62972863075835533575198254035761345007_i128,(-104400255891593472506783488482887544430_i128),68220557046480279396291506810005443406_i128];
Goto(bb12)
}
bb12 = {
Call(_22 = dump_var(17_usize, 14_usize, Move(_14), 8_usize, Move(_8), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_22 = dump_var(17_usize, 10_usize, Move(_10), 1_usize, Move(_1), 23_usize, _23, 23_usize, _23), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static u32,mut _2: (u32, [u8; 8]),mut _3: [i128; 3],mut _4: bool,mut _5: [u8; 8],mut _6: [u8; 8],mut _7: [u8; 8]) -> isize {
mir! {
type RET = isize;
let _8: f64;
let _9: Adt73;
let _10: [usize; 6];
let _11: f64;
let _12: ((i32, u128, &'static bool), [i8; 8], *const *mut (i64, i8, *const u64, [char; 8]));
let _13: i128;
let _14: usize;
let _15: [i128; 4];
let _16: &'static u32;
let _17: u8;
let _18: isize;
let _19: ([i16; 5], &'static i32);
let _20: u8;
let _21: isize;
let _22: f32;
let _23: *mut *const char;
let _24: u16;
let _25: i128;
let _26: f64;
let _27: *const (i32, u128, &'static bool);
let _28: [usize; 7];
let _29: *const u64;
let _30: *const i32;
let _31: isize;
let _32: isize;
let _33: (char,);
let _34: u16;
let _35: Adt73;
let _36: f64;
let _37: char;
let _38: f32;
let _39: (u32, [u8; 8]);
let _40: ();
let _41: ();
{
RET = -27_isize;
_2.1 = [111_u8,69_u8,241_u8,214_u8,31_u8,204_u8,155_u8,8_u8];
_2 = (1481721978_u32, _5);
_2.1 = _5;
_2.0 = 3132343362_u32;
RET = !(-9223372036854775808_isize);
_2.1 = [69_u8,53_u8,69_u8,122_u8,148_u8,199_u8,54_u8,44_u8];
_3 = [(-159383027370990430106345029029131168833_i128),(-139669113239123398085038007449918234904_i128),(-71212185584259021933978033534556668912_i128)];
Goto(bb1)
}
bb1 = {
_2.0 = !702678871_u32;
_2.1 = [130_u8,227_u8,74_u8,162_u8,174_u8,98_u8,105_u8,206_u8];
_2.1 = [18_u8,48_u8,123_u8,146_u8,122_u8,84_u8,240_u8,50_u8];
RET = (-1523046696_i32) as isize;
_7 = [95_u8,170_u8,236_u8,184_u8,107_u8,68_u8,93_u8,197_u8];
_2.0 = 429987966_u32;
_2 = (3207026055_u32, _7);
_6 = [97_u8,159_u8,6_u8,127_u8,48_u8,2_u8,120_u8,152_u8];
_1 = &_2.0;
RET = -113_isize;
_6 = [145_u8,167_u8,38_u8,125_u8,182_u8,58_u8,162_u8,213_u8];
_2.1 = [255_u8,62_u8,156_u8,156_u8,70_u8,121_u8,78_u8,75_u8];
RET = (-123_isize);
_2.1 = [0_u8,108_u8,229_u8,64_u8,51_u8,47_u8,213_u8,201_u8];
_2.0 = 3963540669_u32 * 2780979199_u32;
_8 = 24771_u16 as f64;
_8 = 97_i8 as f64;
_5 = _2.1;
_7 = [103_u8,66_u8,178_u8,129_u8,190_u8,188_u8,14_u8,232_u8];
_7 = _5;
_3 = [(-59596414137514805229204316207326895822_i128),(-51309284292386743478182624612308468811_i128),(-112092633624634168900779438967174396747_i128)];
_1 = &_2.0;
RET = (-9223372036854775808_isize) + (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_10 = [5_usize,16456143706573135181_usize,1_usize,9546055340578409618_usize,10741398232658550311_usize,0_usize];
_3 = [403934894944263162400729864758967092_i128,153987688223576111420948219568218981872_i128,61240129820848444922431813915551067631_i128];
_2.0 = !199522671_u32;
_3 = [123741348446775220489528691091410539682_i128,127365920399976122442782478529921921230_i128,(-52840457876198492006873748985214231098_i128)];
_2 = (694122051_u32, _7);
_2 = (165431461_u32, _7);
_12.1 = [41_i8,(-80_i8),(-25_i8),125_i8,(-86_i8),(-21_i8),76_i8,94_i8];
_2.0 = 2020333716_u32;
_7 = _6;
_1 = &_2.0;
_2.0 = 2486053813_u32;
_11 = _8;
_3 = [(-57215584106855882011487620488111440849_i128),124976533553258481924605635373848254302_i128,70425352229558775829234658190005040252_i128];
RET = (-16_isize);
_12.0.1 = 321606828679978109516488362942036898771_u128 ^ 327820571785482447976889198501686342244_u128;
_12.0.2 = &_4;
RET = (-5521598477492360161_i64) as isize;
_12.0.0 = (-294497440_i32) * (-1967552424_i32);
_8 = (-32693_i16) as f64;
_1 = &_2.0;
match _2.0 {
2486053813 => bb3,
_ => bb1
}
}
bb3 = {
_2.1 = _7;
Goto(bb4)
}
bb4 = {
_2 = (2412618672_u32, _5);
_7 = [170_u8,106_u8,185_u8,214_u8,6_u8,153_u8,242_u8,207_u8];
_7 = [236_u8,20_u8,19_u8,97_u8,219_u8,16_u8,113_u8,83_u8];
_15 = [111198359851479096844928018177766573333_i128,(-108627432471576289096589311774665028665_i128),60671231071738467245848225670742250474_i128,96348669109833919485245531901893277064_i128];
_17 = !77_u8;
_12.0.2 = &_4;
_12.0.1 = 29919_i16 as u128;
_16 = &_2.0;
_6 = [_17,_17,_17,_17,_17,_17,_17,_17];
_13 = 3_usize as i128;
_17 = 13_u8;
_6 = [_17,_17,_17,_17,_17,_17,_17,_17];
_12.0.1 = 224639448703062397652108979500436443770_u128;
RET = 23998_i16 as isize;
_7 = _5;
_12.0.2 = &_4;
_12.0.2 = &_4;
_12.0.0 = (-307590333_i32) | 1889422567_i32;
_12.0.0 = _13 as i32;
_11 = -_8;
_17 = !111_u8;
_12.0.1 = 171111293245878326535569475389887167449_u128;
Goto(bb5)
}
bb5 = {
_15 = [_13,_13,_13,_13];
_18 = !RET;
_15 = [_13,_13,_13,_13];
_1 = Move(_16);
_11 = _8;
_11 = _13 as f64;
_7 = [_17,_17,_17,_17,_17,_17,_17,_17];
_16 = &_2.0;
_12.0.2 = &_4;
_18 = RET & RET;
_20 = !_17;
_12.0.0 = (-115_i8) as i32;
_6 = [_20,_17,_17,_20,_17,_20,_17,_17];
_6 = [_17,_17,_17,_17,_17,_20,_17,_20];
_4 = true;
_3 = [_13,_13,_13];
Call(_12.1 = fn19(Move(_16), _2.0, _2.0, _2.0, (*_16), _15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19.1 = &_12.0.0;
_16 = &_2.0;
_20 = 16042286705914193810_u64 as u8;
_7 = [_20,_20,_17,_17,_20,_20,_17,_17];
_13 = 16050_i16 as i128;
RET = _18;
_19.0 = [19452_i16,31873_i16,(-13570_i16),(-20943_i16),7993_i16];
_8 = -_11;
_3 = [_13,_13,_13];
_21 = !RET;
RET = _21;
_14 = 3_usize & 2_usize;
_12.1 = [(-117_i8),(-118_i8),68_i8,(-6_i8),97_i8,(-30_i8),96_i8,(-120_i8)];
_3 = [_13,_13,_13];
_12.0.0 = _4 as i32;
RET = _20 as isize;
_17 = _12.0.0 as u8;
_19.1 = &_12.0.0;
_18 = !_21;
_6 = [_20,_20,_20,_20,_20,_17,_17,_17];
_21 = _12.0.1 as isize;
_1 = Move(_16);
_5 = [_17,_17,_20,_20,_17,_17,_20,_17];
_24 = 448_u16 & 13775_u16;
_26 = _14 as f64;
_24 = _13 as u16;
_25 = 8878597044457353947_i64 as i128;
match _2.0 {
0 => bb5,
1 => bb2,
2412618672 => bb7,
_ => bb3
}
}
bb7 = {
_27 = core::ptr::addr_of!(_12.0);
(*_27).1 = _18 as u128;
_19.0 = [31037_i16,26126_i16,3065_i16,375_i16,25522_i16];
(*_27).2 = &_4;
_12.0.1 = 197002347571795090438404842148854896128_u128;
_19.1 = &_12.0.0;
Goto(bb8)
}
bb8 = {
_6 = _2.1;
_5 = [_20,_20,_20,_20,_20,_17,_17,_17];
(*_27).1 = 69841462958774512869865900487914054241_u128 ^ 9274419631889216486425317769642219105_u128;
_30 = core::ptr::addr_of!((*_27).0);
_6 = [_17,_17,_20,_17,_17,_17,_17,_17];
_22 = (*_27).1 as f32;
_15 = [_13,_25,_25,_25];
_11 = -_8;
_12.0.0 = 279620832_i32;
_7 = _2.1;
(*_27).1 = 293706130098362085098767302437665833717_u128 - 233567290268887537334136027250285054922_u128;
_12.1 = [(-15_i8),106_i8,(-51_i8),71_i8,(-100_i8),2_i8,(-121_i8),100_i8];
_18 = _21;
_3 = [_13,_13,_13];
_13 = -_25;
_26 = _11 * _8;
_3 = [_25,_25,_13];
_31 = _18 >> _17;
_24 = 7915_u16 << _17;
_22 = _21 as f32;
_30 = core::ptr::addr_of!(_12.0.0);
(*_27).2 = &_4;
Goto(bb9)
}
bb9 = {
_32 = 7753169638639444154_i64 as isize;
_1 = &_2.0;
_32 = _4 as isize;
_4 = true;
_13 = !_25;
RET = _31 ^ _18;
_12.0.1 = !182691789366527752068002415836025055171_u128;
_12.0.2 = &_4;
_3 = [_25,_13,_13];
_3 = [_25,_25,_13];
RET = -_18;
_8 = -_26;
_3 = [_13,_25,_13];
(*_27).1 = 6082983492487839710_i64 as u128;
_5 = _7;
_18 = _31 * _31;
_19.0 = [(-13793_i16),15277_i16,13951_i16,6906_i16,(-9851_i16)];
_26 = _13 as f64;
_33 = ('\u{bcdc3}',);
match (*_1) {
0 => bb1,
2412618672 => bb11,
_ => bb10
}
}
bb10 = {
_2 = (2412618672_u32, _5);
_7 = [170_u8,106_u8,185_u8,214_u8,6_u8,153_u8,242_u8,207_u8];
_7 = [236_u8,20_u8,19_u8,97_u8,219_u8,16_u8,113_u8,83_u8];
_15 = [111198359851479096844928018177766573333_i128,(-108627432471576289096589311774665028665_i128),60671231071738467245848225670742250474_i128,96348669109833919485245531901893277064_i128];
_17 = !77_u8;
_12.0.2 = &_4;
_12.0.1 = 29919_i16 as u128;
_16 = &_2.0;
_6 = [_17,_17,_17,_17,_17,_17,_17,_17];
_13 = 3_usize as i128;
_17 = 13_u8;
_6 = [_17,_17,_17,_17,_17,_17,_17,_17];
_12.0.1 = 224639448703062397652108979500436443770_u128;
RET = 23998_i16 as isize;
_7 = _5;
_12.0.2 = &_4;
_12.0.2 = &_4;
_12.0.0 = (-307590333_i32) | 1889422567_i32;
_12.0.0 = _13 as i32;
_11 = -_8;
_17 = !111_u8;
_12.0.1 = 171111293245878326535569475389887167449_u128;
Goto(bb5)
}
bb11 = {
_19.1 = &(*_30);
_13 = -_25;
RET = _18 << _2.0;
_2.0 = !3641842401_u32;
(*_30) = 1100662482_i32;
(*_27).2 = &_4;
(*_27).2 = &_4;
RET = _14 as isize;
_24 = _22 as u16;
_18 = !_31;
_34 = !_24;
_20 = _17;
_27 = core::ptr::addr_of!(_12.0);
_12.0.2 = &_4;
(*_27).2 = &_4;
(*_27).1 = 153741374856995763576340377052249921461_u128 & 276302740513425325138048232084156273076_u128;
_34 = (*_27).1 as u16;
_19.0 = [(-12022_i16),(-2980_i16),(-32162_i16),27734_i16,26045_i16];
_6 = [_20,_17,_17,_20,_20,_17,_17,_17];
(*_30) = -(-1100321389_i32);
_33.0 = '\u{5cf93}';
RET = _18 * _21;
_15 = [_13,_25,_13,_13];
Goto(bb12)
}
bb12 = {
(*_30) = !(-278874696_i32);
_27 = core::ptr::addr_of!((*_27));
_16 = &_2.0;
_31 = -RET;
_12.0.2 = &_4;
RET = _22 as isize;
_31 = _32;
_19.1 = &(*_27).0;
_17 = _12.0.0 as u8;
_25 = -_13;
_24 = !_34;
_14 = 14776457271663493842_usize;
_33.0 = '\u{c4196}';
(*_27).1 = _18 as u128;
_20 = _17 - _17;
_28 = [_14,_14,_14,_14,_14,_14,_14];
_5 = [_20,_20,_17,_17,_20,_20,_20,_20];
_30 = core::ptr::addr_of!((*_30));
_19.1 = &(*_27).0;
match _14 {
0 => bb11,
1 => bb2,
2 => bb8,
3 => bb4,
14776457271663493842 => bb14,
_ => bb13
}
}
bb13 = {
_6 = _2.1;
_5 = [_20,_20,_20,_20,_20,_17,_17,_17];
(*_27).1 = 69841462958774512869865900487914054241_u128 ^ 9274419631889216486425317769642219105_u128;
_30 = core::ptr::addr_of!((*_27).0);
_6 = [_17,_17,_20,_17,_17,_17,_17,_17];
_22 = (*_27).1 as f32;
_15 = [_13,_25,_25,_25];
_11 = -_8;
_12.0.0 = 279620832_i32;
_7 = _2.1;
(*_27).1 = 293706130098362085098767302437665833717_u128 - 233567290268887537334136027250285054922_u128;
_12.1 = [(-15_i8),106_i8,(-51_i8),71_i8,(-100_i8),2_i8,(-121_i8),100_i8];
_18 = _21;
_3 = [_13,_13,_13];
_13 = -_25;
_26 = _11 * _8;
_3 = [_25,_25,_13];
_31 = _18 >> _17;
_24 = 7915_u16 << _17;
_22 = _21 as f32;
_30 = core::ptr::addr_of!(_12.0.0);
(*_27).2 = &_4;
Goto(bb9)
}
bb14 = {
_12.1 = [(-25_i8),(-31_i8),(-127_i8),47_i8,(-96_i8),94_i8,105_i8,15_i8];
_19.1 = &(*_30);
_15 = [_13,_13,_25,_25];
_7 = _2.1;
(*_27).0 = 1061754025_i32 >> _32;
_38 = _22;
_16 = &_2.0;
_36 = _11;
_28 = [_14,_14,_14,_14,_14,_14,_14];
_8 = _11 - _11;
_19.1 = &(*_27).0;
_13 = !_25;
(*_27).2 = &_4;
_7 = [_20,_20,_20,_17,_17,_20,_17,_17];
_28 = [_14,_14,_14,_14,_14,_14,_14];
_15 = [_25,_25,_25,_25];
_1 = &(*_16);
_2.0 = !2720045017_u32;
_2.1 = [_17,_20,_17,_20,_17,_20,_20,_20];
_13 = (*_27).0 as i128;
_2 = (162404607_u32, _5);
_5 = [_20,_20,_17,_17,_17,_20,_17,_17];
_33.0 = '\u{cf971}';
_8 = -_26;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(18_usize, 31_usize, Move(_31), 32_usize, Move(_32), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(18_usize, 25_usize, Move(_25), 28_usize, Move(_28), 33_usize, Move(_33), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(18_usize, 20_usize, Move(_20), 10_usize, Move(_10), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static u32,mut _2: u32,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: [i128; 4]) -> [i8; 8] {
mir! {
type RET = [i8; 8];
let _7: f32;
let _8: i8;
let _9: [i64; 4];
let _10: bool;
let _11: [i64; 4];
let _12: f64;
let _13: i8;
let _14: f32;
let _15: ();
let _16: ();
{
_4 = !_2;
_5 = !_3;
RET = [68_i8,(-66_i8),98_i8,121_i8,(-96_i8),(-100_i8),100_i8,116_i8];
_4 = '\u{8f544}' as u32;
_1 = &_2;
_6 = [5975044481981028818363722487799585735_i128,(-119312207513653977018069436053936700165_i128),(-117765815891136496379905281563984672949_i128),111081896721723946486703133820703075903_i128];
RET = [(-54_i8),118_i8,9_i8,(-32_i8),(-118_i8),69_i8,(-1_i8),(-56_i8)];
_4 = (*_1);
_7 = (-18652_i16) as f32;
_6 = [(-7326730465923961044131927637621924166_i128),(-56212494399507169965260012252750432159_i128),161553359907096983009598524842983230584_i128,(-10285489928162916161076461942252624631_i128)];
_6 = [(-155733199749723196665679754799504654581_i128),(-39879994073008421092422763172773210295_i128),(-130726744928844598581946373333096937592_i128),16428108881128736720756123201716363003_i128];
_5 = _4 >> _3;
_8 = 5383249028305747845_u64 as i8;
_6 = [101257654326868040133750681630990115533_i128,(-37401494995874632725823939735065171510_i128),98046332510883685976119273065831159098_i128,(-94036610980011168094186567163680400998_i128)];
_4 = !_2;
Goto(bb1)
}
bb1 = {
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_1 = &_3;
_7 = _4 as f32;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_6 = [137499782192138366391228301491405496525_i128,13324924823236484580111159445656764939_i128,28751955324785466140288769118968739191_i128,(-150877259921035578356705653038088786400_i128)];
_4 = (*_1);
_3 = !_2;
Goto(bb2)
}
bb2 = {
_3 = _5;
_3 = _2 ^ _2;
_1 = &_4;
_7 = 3_usize as f32;
_7 = 45225203890746690736131815243151471850_i128 as f32;
_9 = [5978409778056827363_i64,2124389125161586115_i64,(-3554059544499210404_i64),6346659933743113750_i64];
_2 = !(*_1);
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = (*_1);
_7 = 272220828168159508938690659584999901293_u128 as f32;
Goto(bb3)
}
bb3 = {
_4 = 9223372036854775807_isize as u32;
_3 = 13575_u16 as u32;
_5 = _2 * _4;
_4 = _5 >> _2;
_1 = &_4;
_11 = [(-8418036367579396881_i64),(-8968632884138376957_i64),(-409105476010370346_i64),5700146838362170111_i64];
_8 = (-939243278_i32) as i8;
_10 = true & false;
_13 = -_8;
Goto(bb4)
}
bb4 = {
_9 = _11;
_13 = (*_1) as i8;
_1 = &(*_1);
_1 = &_4;
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_12 = 28729_i16 as f64;
_2 = _4 & (*_1);
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_7 = 6420603486096484014437003683040615007_u128 as f32;
_8 = _13 >> _5;
RET = [_8,_8,_8,_13,_8,_8,_8,_13];
RET = [_8,_13,_8,_13,_8,_13,_8,_8];
Goto(bb5)
}
bb5 = {
Call(_15 = dump_var(19_usize, 9_usize, Move(_9), 13_usize, Move(_13), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_15 = dump_var(19_usize, 10_usize, Move(_10), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (i64, i8, *const u64, [char; 8]),
fld1: [i128; 3],
fld2: *const char,
fld3: u8,
fld4: i16,
fld5: *const *mut i128,

},
Variant1{
fld0: bool,
fld1: [char; 7],
fld2: (i64, u64),
fld3: u128,
fld4: f64,
fld5: [i16; 5],
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [i16; 5],
fld1: char,
fld2: u128,
fld3: i8,
fld4: *const *mut i128,

},
Variant1{
fld0: [u8; 8],
fld1: i8,

},
Variant2{
fld0: [char; 7],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: (i64, i8, *const u64, [char; 8]),

},
Variant1{
fld0: i64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [i8; 2],
fld1: [char; 7],
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *const *mut i128,
fld1: char,
fld2: [i64; 4],
fld3: *const *mut (i64, i8, *const u64, [char; 8]),
fld4: i64,

},
Variant1{
fld0: *const *mut (i64, i8, *const u64, [char; 8]),
fld1: char,
fld2: isize,
fld3: Adt51,
fld4: u16,
fld5: u8,
fld6: [u8; 8],

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: f64,
fld1: [i8; 2],

},
Variant1{
fld0: i32,
fld1: [char; 8],

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){unsafe{printf("Adt74::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: i32,

},
Variant1{
fld0: bool,
fld1: [u8; 8],
fld2: [i128; 3],
fld3: *const *mut (i64, i8, *const u64, [char; 8]),
fld4: [i128; 4],
fld5: [char; 8],
fld6: [isize; 4],
fld7: [u32; 7],

},
Variant2{
fld0: bool,
fld1: [u32; 7],
fld2: (i64, i8, *const u64, [char; 8]),
fld3: [usize; 7],
fld4: Adt51,

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt79{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt79 {
fld0: [i8; 8],
fld1: char,
fld2: *const i32,
fld3: *mut *const *mut (i64, i8, *const u64, [char; 8]),
fld4: [char; 5],
fld5: [char; 7],
fld6: ([i128; 3], bool, [char; 7], isize),
}

