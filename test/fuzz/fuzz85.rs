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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u32,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64) -> ((*mut f32, isize, i64), u128, [i8; 4]) {
mir! {
type RET = ((*mut f32, isize, i64), u128, [i8; 4]);
let _8: i8;
let _9: u32;
let _10: isize;
let _11: (*mut f32, isize, i64);
let _12: isize;
let _13: *const u8;
let _14: [u16; 7];
let _15: [u16; 7];
let _16: *mut f32;
let _17: [u128; 6];
let _18: isize;
let _19: f32;
let _20: [i16; 1];
let _21: u64;
let _22: Adt55;
let _23: [u128; 6];
let _24: i64;
let _25: Adt44;
let _26: char;
let _27: [isize; 8];
let _28: isize;
let _29: [u128; 6];
let _30: bool;
let _31: Adt50;
let _32: i16;
let _33: ();
let _34: ();
{
RET.2 = [(-4_i8),(-89_i8),125_i8,(-25_i8)];
RET.1 = 330048348275046301634134011559590456431_u128 ^ 163788118241393265171969379418780840086_u128;
_4 = 6_i8 + (-120_i8);
RET.0.2 = !6663444821875189598_i64;
RET.0.2 = (-2152366617107144532_i64);
_7 = RET.0.2 >> _4;
RET.2 = [_4,_4,_4,_4];
_8 = _4;
RET.0.1 = !9223372036854775807_isize;
_5 = (-16184_i16);
RET.1 = !35416579182772230754792224710164325981_u128;
_6 = (-889557199_i32) & (-844342035_i32);
RET.1 = '\u{e3713}' as u128;
_4 = _8 & _8;
_7 = RET.1 as i64;
RET.0.1 = 133325045545751475557535394570578459443_i128 as isize;
RET.2 = [_4,_8,_4,_8];
_2 = '\u{bd06}';
_4 = 2269826171_u32 as i8;
_1 = !false;
RET.1 = 13712837365502762326340904102231215940_u128 - 11360494672972109770388105190721027406_u128;
_6 = (-825914351_i32);
RET.0.1 = !(-117_isize);
_9 = !806043222_u32;
_7 = RET.0.2 + RET.0.2;
_3 = 29671485018125973297513584212112507183_i128 as u32;
RET.1 = !283377747361645773822313549636462853014_u128;
match RET.0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463461222240814661066924 => bb6,
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
_1 = !true;
RET.2 = [_8,_8,_4,_8];
_11.1 = !RET.0.1;
RET.2 = [_8,_8,_8,_8];
RET.0.2 = RET.0.1 as i64;
RET.0.1 = _11.1;
_8 = _4;
Call(_10 = fn1(_2, _2, _5, RET.0.1, RET.1, _5, _5, RET.1, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11.2 = _2 as i64;
_2 = '\u{ea68}';
RET.1 = _9 as u128;
_9 = !_3;
_5 = -21886_i16;
_3 = _9 * _9;
_7 = RET.0.2 & _11.2;
RET.1 = 245101396538247019775733875559671796073_u128 >> _11.2;
_7 = RET.0.2 >> RET.0.1;
_6 = 1608794807_i32 | (-1348201348_i32);
_9 = !_3;
RET.0.2 = _11.2 << _6;
RET.1 = !48294405630534756049134850546483158741_u128;
RET.0.2 = !_7;
_4 = _8;
RET.0.2 = 10581947413617150766_u64 as i64;
Goto(bb8)
}
bb8 = {
_9 = _3;
_8 = -_4;
RET.1 = 172431431572798604563543974657522606586_u128 ^ 190128913984237892438518259109400356467_u128;
RET.2 = [_4,_8,_8,_4];
_2 = '\u{b1cff}';
_7 = RET.0.1 as i64;
_3 = _9;
_3 = _9;
RET.0.1 = _10;
RET.1 = _2 as u128;
_6 = (-1447182343_i32) ^ (-2066769037_i32);
RET.0.1 = _10 & _11.1;
Goto(bb9)
}
bb9 = {
_1 = RET.1 <= RET.1;
_14 = [21100_u16,40995_u16,25075_u16,6570_u16,65177_u16,43643_u16,10982_u16];
_5 = _1 as i16;
RET.0.2 = -_11.2;
_11.2 = -RET.0.2;
_15 = _14;
_9 = _3;
_11.1 = !RET.0.1;
_15 = [52515_u16,54229_u16,27999_u16,33720_u16,2450_u16,13716_u16,8532_u16];
_17 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_18 = RET.0.1;
RET.0.2 = _7 | _11.2;
RET.2 = [_8,_4,_4,_4];
_11.2 = -RET.0.2;
RET.0.2 = !_11.2;
_11.2 = !_7;
_9 = !_3;
_15 = [64830_u16,43010_u16,50596_u16,32601_u16,38508_u16,46448_u16,60385_u16];
_15 = [3093_u16,12608_u16,26946_u16,31010_u16,16932_u16,21769_u16,10151_u16];
_7 = 2899624594209406695_usize as i64;
_15 = [47256_u16,7168_u16,47512_u16,64605_u16,59500_u16,36194_u16,60521_u16];
_14 = [44137_u16,9129_u16,51018_u16,52717_u16,43129_u16,58137_u16,40683_u16];
Goto(bb10)
}
bb10 = {
_5 = 30167_i16;
RET.2 = [_4,_4,_8,_4];
RET.0.0 = core::ptr::addr_of_mut!(_19);
_5 = -(-1366_i16);
_8 = !_4;
RET.1 = !218519477063524187478887644483578361729_u128;
_12 = !_11.1;
_11.0 = core::ptr::addr_of_mut!(_19);
_9 = 2286792890710915119_usize as u32;
_6 = 1306644453_i32;
RET.1 = _3 as u128;
_18 = !_11.1;
RET.0.0 = core::ptr::addr_of_mut!(_19);
_16 = core::ptr::addr_of_mut!(_19);
_5 = (-10606_i16) >> _18;
_11 = RET.0;
(*_16) = 107_u8 as f32;
_11.1 = _8 as isize;
_9 = _3;
RET.0.1 = 51594_u16 as isize;
_10 = _18;
RET.0 = _11;
(*_16) = _6 as f32;
_11 = (_16, _18, RET.0.2);
_11.1 = _10;
_6 = (-1757070981_i32) * 1668930188_i32;
RET.0 = _11;
Goto(bb11)
}
bb11 = {
RET.0.1 = _11.1;
_8 = 16429749589106367733_u64 as i8;
_19 = 4671939950352107488_usize as f32;
_24 = RET.0.2 * _11.2;
_5 = -14790_i16;
RET.0.1 = _12;
_2 = '\u{9682}';
RET.0.0 = _16;
_25.fld1.fld1 = 11165180485744729883_u64 - 12583030040773440052_u64;
_25.fld6 = [_1,_1,_1,_1];
_14 = [44966_u16,10255_u16,45257_u16,44064_u16,9292_u16,64720_u16,52738_u16];
RET.2 = [_4,_8,_8,_8];
Goto(bb12)
}
bb12 = {
RET.2 = [_4,_4,_8,_4];
_21 = 5923580564395962050_usize as u64;
_11 = RET.0;
Goto(bb13)
}
bb13 = {
_25.fld2 = -109547785029597034943860332396242592669_i128;
_17 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_25.fld1.fld1 = !_21;
_25.fld1.fld5.0 = core::ptr::addr_of_mut!(_25.fld1.fld6);
_25.fld1.fld6 = _9 ^ _9;
_25.fld1.fld5.1 = RET.0.2;
RET.0 = (_11.0, _11.1, _24);
_25.fld6 = [_1,_1,_1,_1];
_25.fld1.fld5.0 = core::ptr::addr_of_mut!(_3);
(*_16) = _8 as f32;
_23 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET.1 = 118450660375181740115076262899239245467_u128;
_9 = _8 as u32;
RET.1 = 166915500928037801230554070098403889171_u128;
_11.1 = -_18;
_19 = 5522943830340049485_usize as f32;
match RET.1 {
0 => bb11,
1 => bb9,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb6,
6 => bb7,
166915500928037801230554070098403889171 => bb14,
_ => bb10
}
}
bb14 = {
_17 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_9 = 55725_u16 as u32;
_25.fld1.fld6 = !_3;
_25.fld1.fld5.0 = core::ptr::addr_of_mut!(_3);
_11.1 = _25.fld2 as isize;
RET.0.0 = core::ptr::addr_of_mut!((*_16));
_28 = RET.0.1;
RET.0 = (_11.0, _18, _7);
_25.fld6 = [_1,_1,_1,_1];
RET.0.0 = _11.0;
RET.0.1 = !_28;
_25.fld1.fld2 = [_1,_1,_1,_1];
_25.fld1.fld1 = _21;
_17 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_25.fld3 = core::ptr::addr_of_mut!(_9);
_20 = [_5];
RET.0.0 = _16;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 8_usize, Move(_8), 18_usize, Move(_18), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 3_usize, Move(_3), 10_usize, Move(_10), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 21_usize, Move(_21), 1_usize, Move(_1), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: char,mut _3: i16,mut _4: isize,mut _5: u128,mut _6: i16,mut _7: i16,mut _8: u128,mut _9: i64) -> isize {
mir! {
type RET = isize;
let _10: [u128; 6];
let _11: f32;
let _12: Adt49;
let _13: i16;
let _14: u8;
let _15: f32;
let _16: [isize; 8];
let _17: isize;
let _18: (*mut f32, isize, i64);
let _19: isize;
let _20: isize;
let _21: (u128, isize, [u64; 2]);
let _22: char;
let _23: (u128, isize, [u64; 2]);
let _24: *const u8;
let _25: [isize; 8];
let _26: bool;
let _27: f64;
let _28: [i16; 4];
let _29: char;
let _30: f64;
let _31: ();
let _32: ();
{
_7 = _3;
_1 = _2;
_9 = 394276412936306322_i64;
RET = _4 ^ _4;
RET = _4;
RET = !_4;
_10 = [_5,_5,_8,_8,_8,_8];
_3 = _7 & _6;
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_10 = [_5,_5,_8,_8,_5,_8];
_11 = 725710913_u32 as f32;
RET = _4 - _4;
_10 = [_5,_8,_5,_8,_5,_8];
_7 = _3;
_14 = 52_u8 + 78_u8;
Goto(bb1)
}
bb1 = {
_3 = _7;
_12.fld0.0 = _5;
RET = !_4;
RET = -_4;
_12.fld0.1 = _4;
_12.fld0.1 = 4140438791_u32 as isize;
_2 = _1;
_2 = _1;
_5 = _8;
_3 = _7;
_12.fld0.1 = RET << _6;
_12.fld0.2 = [2605733688228767532_u64,5603206603929741350_u64];
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_11 = 7167_u16 as f32;
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
Call(_14 = core::intrinsics::bswap(12_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.fld2 = 7497080141968427788_usize as isize;
_12.fld0.0 = _2 as u128;
SetDiscriminant(_12.fld3, 1);
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).1 = 16852957919827763742_u64 as isize;
_11 = 87874204435785516623813415241020396891_i128 as f32;
_10 = [_5,_8,_5,_12.fld0.0,_8,_5];
_3 = -_7;
_10 = [_8,_12.fld0.0,_12.fld0.0,_8,_5,_12.fld0.0];
_17 = 477836884_i32 as isize;
RET = true as isize;
Call(_12.fld2 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = [_12.fld0.1,_12.fld2,_17,_12.fld0.1,_17,_17,_12.fld2,_17];
_15 = (-49_i8) as f32;
_18.2 = _9 + _9;
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)) = (_12.fld0.0, _12.fld0.1, _12.fld0.2);
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
_6 = _3;
_4 = 2_usize as isize;
_18.0 = core::ptr::addr_of_mut!(_15);
_14 = 2507168535054388970_usize as u8;
_2 = _1;
_12.fld0.0 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0 ^ _5;
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_19 = 28981_u16 as isize;
Goto(bb4)
}
bb4 = {
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).0 = !_8;
_4 = !_12.fld0.1;
_6 = 14471055447158587966_u64 as i16;
_16 = [Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).1,_19,_4,_12.fld0.1,_12.fld2,_12.fld0.1,_19,RET];
_12.fld1.0 = core::ptr::addr_of_mut!(place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).2);
_12.fld0 = (Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0, _4, Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).2);
_12.fld2 = -RET;
_13 = _14 as i16;
_12.fld0 = (_8, _17, Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).2);
_4 = _12.fld2 ^ _17;
_12.fld2 = _17 | _4;
_2 = _1;
SetDiscriminant(_12.fld3, 1);
_12.fld0.2 = [14902039966392131122_u64,18443525960690304621_u64];
_21.2 = _12.fld0.2;
_22 = _2;
_16 = [_12.fld2,_12.fld2,_4,_12.fld2,_17,_12.fld0.1,_4,_17];
_8 = 3449125088_u32 as u128;
_20 = _4 << _12.fld2;
_21.1 = _4 >> _20;
_24 = core::ptr::addr_of!(_14);
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
394276412936306322 => bb8,
_ => bb7
}
}
bb5 = {
_16 = [_12.fld0.1,_12.fld2,_17,_12.fld0.1,_17,_17,_12.fld2,_17];
_15 = (-49_i8) as f32;
_18.2 = _9 + _9;
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)) = (_12.fld0.0, _12.fld0.1, _12.fld0.2);
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
_6 = _3;
_4 = 2_usize as isize;
_18.0 = core::ptr::addr_of_mut!(_15);
_14 = 2507168535054388970_usize as u8;
_2 = _1;
_12.fld0.0 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0 ^ _5;
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_19 = 28981_u16 as isize;
Goto(bb4)
}
bb6 = {
_12.fld2 = 7497080141968427788_usize as isize;
_12.fld0.0 = _2 as u128;
SetDiscriminant(_12.fld3, 1);
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).1 = 16852957919827763742_u64 as isize;
_11 = 87874204435785516623813415241020396891_i128 as f32;
_10 = [_5,_8,_5,_12.fld0.0,_8,_5];
_3 = -_7;
_10 = [_8,_12.fld0.0,_12.fld0.0,_8,_5,_12.fld0.0];
_17 = 477836884_i32 as isize;
RET = true as isize;
Call(_12.fld2 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = _7;
_12.fld0.0 = _5;
RET = !_4;
RET = -_4;
_12.fld0.1 = _4;
_12.fld0.1 = 4140438791_u32 as isize;
_2 = _1;
_2 = _1;
_5 = _8;
_3 = _7;
_12.fld0.1 = RET << _6;
_12.fld0.2 = [2605733688228767532_u64,5603206603929741350_u64];
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_11 = 7167_u16 as f32;
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
Call(_14 = core::intrinsics::bswap(12_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_20 = !_4;
_10 = [_8,_8,_5,_5,_5,_12.fld0.0];
Call(place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).1 = fn2(_20, _12.fld0.1, _21.2, _21.1, _12.fld1, _17, _12.fld1.0, RET, _2, _21.1, _12.fld0.1, _1, _12.fld2, (*_24), _12.fld1.0, _12.fld2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21.0 = !_5;
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)).0 = _8 - _21.0;
_26 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).1 == _4;
_7 = _13;
_23.0 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0 << _19;
_23.2 = [1901716114183838748_u64,15623493774138218584_u64];
match _9 {
0 => bb5,
1 => bb10,
394276412936306322 => bb12,
_ => bb11
}
}
bb10 = {
_16 = [_12.fld0.1,_12.fld2,_17,_12.fld0.1,_17,_17,_12.fld2,_17];
_15 = (-49_i8) as f32;
_18.2 = _9 + _9;
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)) = (_12.fld0.0, _12.fld0.1, _12.fld0.2);
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
_6 = _3;
_4 = 2_usize as isize;
_18.0 = core::ptr::addr_of_mut!(_15);
_14 = 2507168535054388970_usize as u8;
_2 = _1;
_12.fld0.0 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0 ^ _5;
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_19 = 28981_u16 as isize;
Goto(bb4)
}
bb11 = {
_3 = _7;
_12.fld0.0 = _5;
RET = !_4;
RET = -_4;
_12.fld0.1 = _4;
_12.fld0.1 = 4140438791_u32 as isize;
_2 = _1;
_2 = _1;
_5 = _8;
_3 = _7;
_12.fld0.1 = RET << _6;
_12.fld0.2 = [2605733688228767532_u64,5603206603929741350_u64];
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_11 = 7167_u16 as f32;
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
Call(_14 = core::intrinsics::bswap(12_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_21.2 = [7854598599234276760_u64,16827009886069588623_u64];
RET = _6 as isize;
(*_24) = 42450_u16 as u8;
_1 = _2;
_21 = (_23.0, Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).1, _23.2);
_23.0 = _5 << Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0;
_17 = !Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).1;
_12.fld0.1 = _21.1;
_27 = 893332258_u32 as f64;
place!(Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0)) = (_23.0, _12.fld0.1, _12.fld0.2);
_28 = [_7,_3,_3,_3];
_12.fld0 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0);
_12.fld2 = _17;
_18.2 = -_9;
_26 = true & false;
_25 = [_21.1,_12.fld2,Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).1,_12.fld0.1,_17,_17,_12.fld0.1,_21.1];
_28 = [_13,_3,_3,_3];
(*_24) = (-114808987990829977883351992187983745704_i128) as u8;
_26 = !false;
_10 = [_21.0,_12.fld0.0,_12.fld0.0,_12.fld0.0,_23.0,_23.0];
_18.2 = _20 as i64;
_21.2 = _12.fld0.2;
_8 = _5 + _23.0;
_21 = (Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).0, _17, Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).2);
_23 = (_12.fld0.0, _21.1, _12.fld0.2);
_16 = [_21.1,_21.1,_20,_12.fld2,_17,_12.fld2,_21.1,_12.fld2];
_12.fld0.2 = _21.2;
_24 = core::ptr::addr_of!(_14);
_24 = core::ptr::addr_of!(_14);
_18.1 = _23.1 + _23.1;
match _9 {
394276412936306322 => bb14,
_ => bb13
}
}
bb13 = {
_3 = _7;
_12.fld0.0 = _5;
RET = !_4;
RET = -_4;
_12.fld0.1 = _4;
_12.fld0.1 = 4140438791_u32 as isize;
_2 = _1;
_2 = _1;
_5 = _8;
_3 = _7;
_12.fld0.1 = RET << _6;
_12.fld0.2 = [2605733688228767532_u64,5603206603929741350_u64];
_12.fld1.0 = core::ptr::addr_of_mut!(_12.fld0.2);
_11 = 7167_u16 as f32;
_12.fld3 = Adt48::Variant1 { fld0: _12.fld0 };
Call(_14 = core::intrinsics::bswap(12_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_21.2 = Field::<(u128, isize, [u64; 2])>(Variant(_12.fld3, 1), 0).2;
(*_24) = _20 as u8;
_21.0 = !_23.0;
_25 = _16;
_21.2 = [18425141076232680588_u64,1893132830900588633_u64];
_24 = core::ptr::addr_of!(_14);
_20 = _15 as isize;
SetDiscriminant(_12.fld3, 0);
_22 = _1;
_11 = _15;
_12.fld2 = _12.fld0.1 + _23.1;
_23 = _21;
_13 = _6;
_12.fld2 = -_12.fld0.1;
place!(Field::<i16>(Variant(_12.fld3, 0), 3)) = _3;
_12.fld0 = (_8, _18.1, _23.2);
_19 = _17;
_18.2 = _9 & _9;
RET = _27 as isize;
_28 = [_3,Field::<i16>(Variant(_12.fld3, 0), 3),_6,_6];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 8_usize, Move(_8), 6_usize, Move(_6), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 25_usize, Move(_25), 19_usize, Move(_19), 28_usize, Move(_28), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(1_usize, 20_usize, Move(_20), 22_usize, Move(_22), 13_usize, Move(_13), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: [u64; 2],mut _4: isize,mut _5: (*mut [u64; 2],),mut _6: isize,mut _7: *mut [u64; 2],mut _8: isize,mut _9: char,mut _10: isize,mut _11: isize,mut _12: char,mut _13: isize,mut _14: u8,mut _15: *mut [u64; 2],mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: f64;
let _18: (char, i16, char);
let _19: isize;
let _20: *const u8;
let _21: [i16; 2];
let _22: u128;
let _23: u16;
let _24: (u128, isize, [u64; 2]);
let _25: Adt53;
let _26: ();
let _27: ();
{
_12 = _9;
RET = _4 | _16;
RET = 54_i8 as isize;
_5.0 = _15;
_6 = !_4;
Call(_17 = fn3(RET, _6, _6, _5.0, _10, _7, _9, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = core::ptr::addr_of_mut!(_3);
_16 = 11007512826326154770_usize as isize;
_17 = 29450879737146260071072630282084102212_i128 as f64;
_15 = core::ptr::addr_of_mut!((*_15));
_10 = !_16;
_5.0 = core::ptr::addr_of_mut!((*_15));
_1 = !_2;
RET = _2;
_3 = [123502119782217427_u64,1614919539917542870_u64];
(*_15) = [14718930575672947357_u64,6635128138564298647_u64];
_15 = _5.0;
_5 = (_7,);
_1 = _10;
_4 = -_13;
_3 = [3749379791560462324_u64,15178968915513015578_u64];
(*_15) = [5805979478865480644_u64,16744409181859683085_u64];
_3 = [3549904056560164683_u64,5751632371982550719_u64];
_11 = _13 * _6;
_7 = core::ptr::addr_of_mut!(_3);
_18 = (_12, (-3981_i16), _9);
_13 = 335161512058521627241550106251546264969_u128 as isize;
(*_15) = [17684645260051656182_u64,14256458166884543308_u64];
_2 = _6;
Goto(bb2)
}
bb2 = {
_18.2 = _18.0;
_8 = RET;
_5 = (_15,);
(*_7) = [16891389418280198461_u64,384090906889440285_u64];
_14 = (-29452551019104666849381682151136235387_i128) as u8;
(*_7) = [9750016409455550622_u64,12582361453601820637_u64];
RET = _2;
_4 = _11 ^ _11;
_20 = core::ptr::addr_of!(_14);
Goto(bb3)
}
bb3 = {
_18.0 = _18.2;
_17 = 5451460652574653135_usize as f64;
_15 = _7;
RET = _2 << _6;
(*_20) = 158_u8;
_22 = 11583761934491679431_u64 as u128;
_21 = [_18.1,_18.1];
(*_20) = (-6819580580349343921999702940991058876_i128) as u8;
_24.2 = [1209798245472311643_u64,350390061992184796_u64];
_4 = _6 << _2;
_16 = _4 & _4;
_23 = (*_20) as u16;
_6 = !_16;
_24.0 = 9521050984996600440_usize as u128;
_10 = _16;
_24 = (_22, _16, (*_7));
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(2_usize, 23_usize, Move(_23), 12_usize, Move(_12), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(2_usize, 14_usize, Move(_14), 22_usize, Move(_22), 24_usize, Move(_24), 13_usize, Move(_13)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_26 = dump_var(2_usize, 1_usize, Move(_1), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: *mut [u64; 2],mut _5: isize,mut _6: *mut [u64; 2],mut _7: char,mut _8: isize) -> f64 {
mir! {
type RET = f64;
let _9: *mut u32;
let _10: bool;
let _11: *const u8;
let _12: Adt51;
let _13: [i16; 4];
let _14: [u16; 7];
let _15: u128;
let _16: [u64; 2];
let _17: f64;
let _18: u16;
let _19: isize;
let _20: char;
let _21: u64;
let _22: ();
let _23: ();
{
_5 = _2;
RET = 2318047470_u32 as f64;
_7 = '\u{f6358}';
_1 = -_5;
_5 = _8 << _8;
_4 = _6;
_10 = false & false;
_4 = _6;
_3 = _1 - _2;
_5 = RET as isize;
_3 = _8 * _8;
RET = 76_i8 as f64;
_1 = (-2928157263708660278_i64) as isize;
_4 = _6;
_1 = -_8;
_10 = false | false;
_8 = 3563507593_u32 as isize;
RET = 116_i8 as f64;
_14 = [13938_u16,1618_u16,43828_u16,35912_u16,31840_u16,41918_u16,34888_u16];
_6 = _4;
Call(_9 = fn4(RET, _8, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-677212371_i32) as f64;
_10 = true;
_4 = _6;
_3 = _2 & _2;
_13 = [(-16963_i16),(-19116_i16),608_i16,(-24619_i16)];
_15 = RET as u128;
_2 = _1;
_13 = [(-26271_i16),(-9634_i16),(-20237_i16),(-25793_i16)];
_6 = _4;
_3 = 4155_i16 as isize;
_17 = RET - RET;
RET = 5583558589438610505_i64 as f64;
_3 = _2 * _2;
_17 = 6960224609678439967_i64 as f64;
_6 = _4;
_14 = [57948_u16,5466_u16,590_u16,47104_u16,60495_u16,61811_u16,122_u16];
_2 = _3;
_16 = [2351965846684467058_u64,5210945002345748412_u64];
_17 = _3 as f64;
_4 = _6;
Goto(bb2)
}
bb2 = {
_7 = '\u{dca28}';
_6 = core::ptr::addr_of_mut!(_16);
_7 = '\u{9c2a8}';
_16 = [18285337297248792345_u64,7813917213327878801_u64];
_1 = _15 as isize;
_15 = 10973217989117897972576444540085110725_u128 - 116382649951354284949528808016891341386_u128;
_14 = [24133_u16,25366_u16,17981_u16,59653_u16,34988_u16,40906_u16,5680_u16];
_16 = [5457597830493750023_u64,18440837403126004789_u64];
_1 = _2 * _3;
_6 = core::ptr::addr_of_mut!(_16);
_6 = _4;
RET = 2202999587_u32 as f64;
_3 = (-96302453_i32) as isize;
Goto(bb3)
}
bb3 = {
_4 = core::ptr::addr_of_mut!(_16);
_15 = !28477846074685354239347628365652794316_u128;
_18 = 195_u8 as u16;
_15 = !215072547536388630890915077002997945321_u128;
_8 = 4_i8 as isize;
RET = -_17;
_15 = 156643450388741274458402783264061690988_u128;
(*_4) = [8520187969295899160_u64,2061052924024099670_u64];
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(3_usize, 16_usize, Move(_16), 8_usize, Move(_8), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(3_usize, 13_usize, Move(_13), 18_usize, Move(_18), 23_usize, _23, 23_usize, _23), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f64,mut _2: isize,mut _3: *mut [u64; 2],mut _4: isize) -> *mut u32 {
mir! {
type RET = *mut u32;
let _5: f64;
let _6: [i8; 4];
let _7: u64;
let _8: &'static [isize; 1];
let _9: i64;
let _10: [i8; 4];
let _11: u8;
let _12: i128;
let _13: i16;
let _14: i32;
let _15: f32;
let _16: isize;
let _17: [i16; 1];
let _18: i128;
let _19: [i16; 1];
let _20: i32;
let _21: (u128, isize, [u64; 2]);
let _22: f64;
let _23: isize;
let _24: f64;
let _25: u32;
let _26: isize;
let _27: isize;
let _28: Adt55;
let _29: ();
let _30: ();
{
_4 = _2;
_4 = !_2;
_1 = 15_u8 as f64;
_4 = -_2;
_4 = _2;
_4 = -_2;
_6 = [110_i8,73_i8,118_i8,126_i8];
_5 = _1;
_6 = [(-44_i8),(-30_i8),113_i8,(-80_i8)];
_4 = (-1634875934856478967_i64) as isize;
_2 = 3365649691_u32 as isize;
_2 = _4;
_6 = [(-111_i8),(-128_i8),(-32_i8),(-126_i8)];
_4 = _2 - _2;
_5 = _1 - _1;
_9 = !(-7209423542098472354_i64);
_6 = [(-110_i8),102_i8,(-77_i8),(-11_i8)];
_7 = 14920654427868868961_u64 | 14443748635204143546_u64;
_7 = !5506827357060039273_u64;
_2 = true as isize;
_6 = [(-48_i8),(-110_i8),(-120_i8),(-69_i8)];
_4 = 1525231746_i32 as isize;
_1 = _5 + _5;
_7 = !15199710034203063754_u64;
Goto(bb1)
}
bb1 = {
_4 = -_2;
_7 = 309192157804634553498880365740118341874_u128 as u64;
_7 = 10899680848259436111_u64 + 12136100017049385900_u64;
_4 = 18142_i16 as isize;
_1 = -_5;
_6 = [(-45_i8),(-46_i8),(-11_i8),14_i8];
_1 = 119418161952004837183809695826865401664_u128 as f64;
_6 = [126_i8,(-113_i8),(-77_i8),56_i8];
_4 = _2;
_7 = 1128297804275190481_u64 >> _9;
_6 = [80_i8,31_i8,(-124_i8),(-72_i8)];
_7 = _5 as u64;
_7 = !15834013323623738462_u64;
_11 = 192_u8;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
192 => bb9,
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
_1 = _5;
_7 = _11 as u64;
_1 = _5;
Goto(bb10)
}
bb10 = {
_12 = _9 as i128;
Call(_16 = fn5(_3, _1, _1, _1, _5, _11, _6, _9, _6, _5, _4, _3, _11, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13 = 146_i16;
_11 = 225_u8;
_16 = !_4;
_4 = _2;
_15 = (-100_i8) as f32;
_10 = [18_i8,94_i8,49_i8,(-43_i8)];
_11 = 242_u8 ^ 84_u8;
_17 = [_13];
_10 = [72_i8,119_i8,71_i8,54_i8];
_14 = -(-1493018520_i32);
_16 = _15 as isize;
_5 = _1;
_2 = -_4;
_12 = 66694548074593095618989682433165174502_i128;
_10 = _6;
_16 = _4;
_2 = _16;
_17 = [_13];
_10 = [(-41_i8),76_i8,(-6_i8),(-108_i8)];
_9 = _7 as i64;
_16 = _4 + _4;
_2 = _4 - _4;
_18 = _12 ^ _12;
Goto(bb12)
}
bb12 = {
_10 = _6;
_9 = 5530540605063407240_i64;
_10 = [127_i8,(-51_i8),22_i8,73_i8];
_16 = _2 * _2;
Call(_1 = core::intrinsics::fmaf64(_5, _5, _5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_19 = _17;
_15 = 1_usize as f32;
_21.0 = !35575816735875315121766438951407408177_u128;
_21.2 = [_7,_7];
_21.1 = _16 * _16;
_21.0 = !44289187694551814397372558449934494615_u128;
Goto(bb14)
}
bb14 = {
_17 = _19;
Goto(bb15)
}
bb15 = {
_18 = (-57_i8) as i128;
_11 = 90_u8;
_7 = 5765468666123011944_u64 >> _21.1;
_12 = -_18;
_20 = _14 << _21.0;
_2 = 18171_u16 as isize;
_21.2 = [_7,_7];
_17 = _19;
_21.0 = 280093430721351782263809204712941449232_u128;
_20 = _14 ^ _14;
_10 = [27_i8,(-29_i8),22_i8,(-80_i8)];
_15 = _16 as f32;
_20 = _14 ^ _14;
_7 = !16753688857610498282_u64;
_15 = _13 as f32;
_1 = _5;
_1 = _5;
_24 = _1;
Goto(bb16)
}
bb16 = {
_17 = [_13];
_7 = !14748192855555401762_u64;
Goto(bb17)
}
bb17 = {
_2 = -_21.1;
_12 = _18 - _18;
_9 = (-8552900840218016569_i64);
_22 = _11 as f64;
_27 = _24 as isize;
RET = core::ptr::addr_of_mut!(_25);
Goto(bb18)
}
bb18 = {
Call(_29 = dump_var(4_usize, 13_usize, Move(_13), 6_usize, Move(_6), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(4_usize, 10_usize, Move(_10), 9_usize, Move(_9), 20_usize, Move(_20), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut [u64; 2],mut _2: f64,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: u8,mut _7: [i8; 4],mut _8: i64,mut _9: [i8; 4],mut _10: f64,mut _11: isize,mut _12: *mut [u64; 2],mut _13: u8,mut _14: [i8; 4]) -> isize {
mir! {
type RET = isize;
let _15: [i16; 2];
let _16: [u64; 2];
let _17: [bool; 4];
let _18: char;
let _19: [isize; 8];
let _20: Adt45;
let _21: [u16; 7];
let _22: [i16; 1];
let _23: [u64; 2];
let _24: char;
let _25: [u128; 6];
let _26: *mut [u64; 2];
let _27: isize;
let _28: (*mut f32, isize, i64);
let _29: f32;
let _30: (*mut [u64; 2],);
let _31: isize;
let _32: isize;
let _33: u32;
let _34: i8;
let _35: f64;
let _36: [isize; 1];
let _37: i16;
let _38: bool;
let _39: u64;
let _40: f64;
let _41: [isize; 8];
let _42: [isize; 1];
let _43: [i16; 1];
let _44: [bool; 4];
let _45: ();
let _46: ();
{
_5 = 16990_i16 as f64;
_14 = _7;
_14 = _9;
RET = _11;
Call(_14 = core::intrinsics::transmute(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !_13;
_11 = RET;
_4 = _2;
Goto(bb2)
}
bb2 = {
_13 = 1196212046_u32 as u8;
_5 = -_2;
Call(_1 = fn6(_9, _4, _13, _7, _14, _12, _12, _12, _13, _10, _3, _14, RET, _14, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = RET - RET;
_4 = -_2;
_3 = _5;
_15 = [(-30594_i16),18245_i16];
Goto(bb4)
}
bb4 = {
RET = _13 as isize;
_11 = 2_usize as isize;
RET = _11 + _11;
_16 = [18238052646915760730_u64,10162746815766560880_u64];
_9 = [(-75_i8),(-127_i8),(-17_i8),75_i8];
_17 = [true,false,true,false];
_10 = _4;
_17 = [false,true,false,false];
_12 = _1;
Goto(bb5)
}
bb5 = {
_6 = !_13;
RET = 3063252263_u32 as isize;
_7 = _14;
_16 = [16991735000342746662_u64,3275381394837289609_u64];
_14 = _7;
_5 = 4_i8 as f64;
_5 = _2;
_4 = _2;
Call(_6 = core::intrinsics::bswap(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = _3;
_4 = -_2;
RET = _8 as isize;
_12 = _1;
_2 = _4 - _5;
_16 = [1769853156182173535_u64,8887816425086102531_u64];
_21 = [20980_u16,23616_u16,29163_u16,5381_u16,4297_u16,37382_u16,15225_u16];
_9 = [(-27_i8),(-62_i8),18_i8,(-93_i8)];
Call(_22 = fn19(_3, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = [(-19268_i16),15713_i16];
_6 = '\u{b2b1a}' as u8;
_19 = [_11,RET,_11,RET,_11,RET,_11,RET];
_14 = [45_i8,72_i8,19_i8,82_i8];
_13 = !_6;
RET = _11;
RET = 2106960001_u32 as isize;
_8 = 3084103477805666105_i64 >> RET;
_12 = _1;
_1 = _12;
_8 = 2600736743277144551_i64;
_15 = [(-23051_i16),(-24306_i16)];
_9 = [(-115_i8),(-67_i8),(-52_i8),5_i8];
_13 = _6 - _6;
_22 = [(-8349_i16)];
_17 = [true,false,true,false];
_24 = '\u{77609}';
_15 = [(-3356_i16),208_i16];
_23 = [7497562104400490443_u64,8170328517097332609_u64];
_9 = _14;
_11 = RET;
_24 = '\u{fa4e4}';
_23 = _16;
_17 = [false,false,false,true];
_17 = [true,true,true,false];
_5 = _2 * _2;
match _8 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb8,
2600736743277144551 => bb10,
_ => bb9
}
}
bb8 = {
_2 = _3;
_4 = -_2;
RET = _8 as isize;
_12 = _1;
_2 = _4 - _5;
_16 = [1769853156182173535_u64,8887816425086102531_u64];
_21 = [20980_u16,23616_u16,29163_u16,5381_u16,4297_u16,37382_u16,15225_u16];
_9 = [(-27_i8),(-62_i8),18_i8,(-93_i8)];
Call(_22 = fn19(_3, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_13 = 1196212046_u32 as u8;
_5 = -_2;
Call(_1 = fn6(_9, _4, _13, _7, _14, _12, _12, _12, _13, _10, _3, _14, RET, _14, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_19 = [RET,_11,RET,_11,_11,RET,RET,_11];
_24 = '\u{84737}';
_11 = _4 as isize;
_1 = _12;
_15 = [(-9333_i16),(-16739_i16)];
_4 = -_5;
_12 = core::ptr::addr_of_mut!(_23);
_4 = _5;
_25 = [99468322594350587934773591196540915567_u128,9134449520772501491823785796497405205_u128,254524559669625652489980964708935103463_u128,279315996389970689660793510749296031416_u128,280897148875743482456344326846623281777_u128,246367666339371405281388305707168833440_u128];
_16 = [11270343667593686357_u64,17140624634645886991_u64];
_26 = core::ptr::addr_of_mut!(_16);
_2 = -_3;
_12 = _1;
_5 = -_3;
(*_26) = [5808741992834772046_u64,6460235827531553459_u64];
_18 = _24;
_7 = [117_i8,(-75_i8),(-32_i8),(-127_i8)];
_27 = _11 ^ _11;
_8 = 75_i8 as i64;
_23 = (*_26);
RET = _27;
_29 = (-169797461405290787849918001203181087190_i128) as f32;
_22 = [(-18962_i16)];
_19 = [_27,RET,_11,RET,_27,_11,_27,_27];
Goto(bb11)
}
bb11 = {
_25 = [61258785365776014211818504564524173218_u128,314594424490724541954851020909598738708_u128,10825185631491667465098886648227778761_u128,204805474445813327294303119668770016541_u128,229385440119711644968570670118936986148_u128,42949043514302722262931612124248140774_u128];
_1 = _26;
_11 = _27 ^ _27;
_16 = [3702061204766336836_u64,1190689454040907713_u64];
RET = 51143_u16 as isize;
(*_26) = [11142173852228063994_u64,14279744387876985583_u64];
_14 = [111_i8,(-101_i8),(-98_i8),102_i8];
_28.2 = -_8;
_27 = _11;
_25 = [109816480734463582273763175452591645355_u128,305358037786464910050933899239792802157_u128,325409812607783750329129835856547911997_u128,160490508996507691420523481503884049664_u128,311803178082581633480829829309636601285_u128,60184228288603450420136291347520231299_u128];
(*_1) = [15239094185681755319_u64,295389715807340260_u64];
_28.0 = core::ptr::addr_of_mut!(_29);
_12 = core::ptr::addr_of_mut!((*_26));
_13 = _6;
_31 = -_27;
_3 = (-32256_i16) as f64;
_30 = (_1,);
_3 = _10;
_6 = _18 as u8;
_1 = core::ptr::addr_of_mut!((*_12));
_16 = [14798493232865697160_u64,10698650111509443925_u64];
_13 = _6;
_17 = [true,false,true,true];
Goto(bb12)
}
bb12 = {
_12 = _1;
(*_1) = _23;
_23 = (*_1);
_12 = core::ptr::addr_of_mut!((*_1));
_15 = [1072_i16,19446_i16];
RET = !_31;
_15 = [(-23622_i16),15886_i16];
_11 = false as isize;
_37 = 41370_u16 as i16;
_8 = -_28.2;
_11 = _27;
_1 = core::ptr::addr_of_mut!((*_1));
Call(_11 = core::intrinsics::transmute(RET), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4 = _5 + _10;
_27 = _11 ^ _11;
(*_12) = [11612360340527702529_u64,2595026166835871478_u64];
_12 = core::ptr::addr_of_mut!((*_12));
_4 = -_3;
RET = _2 as isize;
_17 = [false,true,false,false];
_2 = 3897906988_u32 as f64;
_28.1 = -_31;
_38 = !true;
_20 = Adt45::Variant0 { fld0: _1,fld1: _28.0,fld2: _22,fld3: 3377359320429130862_u64,fld4: 3919024088_u32 };
_28.2 = _8;
_29 = 6656739824763010072_usize as f32;
_18 = _24;
Goto(bb14)
}
bb14 = {
_38 = _27 != _11;
_43 = [_37];
place!(Field::<u64>(Variant(_20, 0), 3)) = 9607944040736893368_u64 - 2890507676183634316_u64;
_23 = [Field::<u64>(Variant(_20, 0), 3),Field::<u64>(Variant(_20, 0), 3)];
_42 = [_31];
_28.1 = (-323801345036208211056102524819607428_i128) as isize;
place!(Field::<u64>(Variant(_20, 0), 3)) = 1852835754825448451_u64 - 7812896555144934620_u64;
_30 = (Field::<*mut [u64; 2]>(Variant(_20, 0), 0),);
_3 = _5;
_14 = _9;
RET = 6_usize as isize;
_19 = [_27,_27,RET,_27,_27,_27,_27,_31];
_32 = _27;
RET = -_27;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(5_usize, 24_usize, Move(_24), 6_usize, Move(_6), 14_usize, Move(_14), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(5_usize, 38_usize, Move(_38), 25_usize, Move(_25), 31_usize, Move(_31), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(5_usize, 17_usize, Move(_17), 15_usize, Move(_15), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i8; 4],mut _2: f64,mut _3: u8,mut _4: [i8; 4],mut _5: [i8; 4],mut _6: *mut [u64; 2],mut _7: *mut [u64; 2],mut _8: *mut [u64; 2],mut _9: u8,mut _10: f64,mut _11: f64,mut _12: [i8; 4],mut _13: isize,mut _14: [i8; 4],mut _15: *mut [u64; 2]) -> *mut [u64; 2] {
mir! {
type RET = *mut [u64; 2];
let _16: i128;
let _17: u128;
let _18: i128;
let _19: [bool; 4];
let _20: [i16; 2];
let _21: [i16; 1];
let _22: f32;
let _23: f64;
let _24: i32;
let _25: isize;
let _26: Adt49;
let _27: Adt44;
let _28: f32;
let _29: f32;
let _30: *const u8;
let _31: Adt49;
let _32: bool;
let _33: [i8; 4];
let _34: bool;
let _35: u32;
let _36: [u128; 6];
let _37: (u128, isize, [u64; 2]);
let _38: bool;
let _39: ();
let _40: ();
{
_4 = [64_i8,(-95_i8),57_i8,66_i8];
_7 = _8;
RET = _7;
_8 = _7;
_14 = [(-12_i8),30_i8,90_i8,31_i8];
_16 = !155205110923705392883797090127063987445_i128;
RET = _6;
_13 = (-9223372036854775808_isize);
_2 = _10 * _11;
RET = _6;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463454151235394913435648 => bb5,
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
_15 = _7;
_18 = _16;
_7 = _15;
_2 = _11;
RET = _15;
_9 = _3;
_10 = _2;
_17 = 129284937430068887308890905325054716364_u128;
_7 = _8;
RET = _15;
_13 = -(-74_isize);
_20 = [11675_i16,(-25707_i16)];
_4 = _1;
RET = _15;
_20 = [17498_i16,25861_i16];
_15 = _8;
_9 = _3;
_15 = _7;
_6 = _7;
_21 = [(-14302_i16)];
_19 = [true,true,false,false];
_6 = _15;
_12 = [(-20_i8),(-57_i8),95_i8,(-104_i8)];
_13 = 9223372036854775807_isize;
Goto(bb6)
}
bb6 = {
_14 = [38_i8,22_i8,(-91_i8),(-88_i8)];
RET = _7;
_8 = _15;
_22 = (-868725454560075260_i64) as f32;
_8 = _15;
_20 = [13735_i16,17507_i16];
_20 = [6024_i16,781_i16];
_4 = _12;
_5 = [(-3_i8),(-87_i8),119_i8,42_i8];
_13 = !(-9223372036854775808_isize);
Call(_1 = fn7(_8, _8, _12, _20, _2, _8, _6, _4, _9, _8, _19, _4, _6, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = _8;
_23 = (-127_i8) as f64;
_5 = [(-120_i8),(-128_i8),24_i8,(-56_i8)];
_24 = !729140217_i32;
match _17 {
0 => bb1,
1 => bb4,
2 => bb8,
3 => bb9,
129284937430068887308890905325054716364 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
_15 = _7;
_18 = _16;
_7 = _15;
_2 = _11;
RET = _15;
_9 = _3;
_10 = _2;
_17 = 129284937430068887308890905325054716364_u128;
_7 = _8;
RET = _15;
_13 = -(-74_isize);
_20 = [11675_i16,(-25707_i16)];
_4 = _1;
RET = _15;
_20 = [17498_i16,25861_i16];
_15 = _8;
_9 = _3;
_15 = _7;
_6 = _7;
_21 = [(-14302_i16)];
_19 = [true,true,false,false];
_6 = _15;
_12 = [(-20_i8),(-57_i8),95_i8,(-104_i8)];
_13 = 9223372036854775807_isize;
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_1 = _12;
_22 = _10 as f32;
_10 = -_2;
_6 = _7;
_17 = _22 as u128;
_25 = _13 * _13;
_13 = _25;
Goto(bb12)
}
bb12 = {
_20 = [18022_i16,(-25312_i16)];
_27.fld0 = _17 >> _3;
_27.fld6 = [false,true,false,true];
_27.fld3 = core::ptr::addr_of_mut!(_27.fld1.fld6);
_3 = 116_i8 as u8;
_7 = _6;
_11 = 15819_u16 as f64;
_11 = -_2;
_29 = -_22;
_27.fld1.fld4 = core::ptr::addr_of!(_9);
Goto(bb13)
}
bb13 = {
_4 = [(-116_i8),(-101_i8),(-53_i8),59_i8];
_27.fld2 = _18 | _16;
_27.fld1.fld1 = 8776268484191148338_u64;
_31.fld2 = _9 as isize;
_27.fld1.fld5.0 = core::ptr::addr_of_mut!(_27.fld1.fld6);
_11 = _10;
_27.fld1.fld5.1 = 4436902215988893050_i64 + (-5658379793408675567_i64);
_26.fld0.2 = [_27.fld1.fld1,_27.fld1.fld1];
_27.fld1.fld5.0 = core::ptr::addr_of_mut!(_27.fld1.fld6);
_26.fld0.0 = !_17;
RET = _8;
_31.fld0 = (_26.fld0.0, _25, _26.fld0.2);
_20 = [9527_i16,(-25483_i16)];
_27.fld1.fld3 = 1_usize & 1_usize;
_26.fld1.0 = core::ptr::addr_of_mut!(_26.fld0.2);
_26.fld0.0 = 17605_i16 as u128;
_28 = _29 * _22;
_27.fld1.fld2 = [true,false,true,true];
_1 = [124_i8,(-83_i8),73_i8,(-37_i8)];
_26.fld0.1 = -_13;
_27.fld1.fld1 = 18434921525065528827_u64;
_25 = _13;
_27.fld1.fld2 = [true,true,false,true];
_26.fld0 = (_27.fld0, _31.fld2, _31.fld0.2);
match _27.fld1.fld1 {
0 => bb5,
1 => bb7,
2 => bb12,
3 => bb14,
4 => bb15,
18434921525065528827 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
_15 = _7;
_18 = _16;
_7 = _15;
_2 = _11;
RET = _15;
_9 = _3;
_10 = _2;
_17 = 129284937430068887308890905325054716364_u128;
_7 = _8;
RET = _15;
_13 = -(-74_isize);
_20 = [11675_i16,(-25707_i16)];
_4 = _1;
RET = _15;
_20 = [17498_i16,25861_i16];
_15 = _8;
_9 = _3;
_15 = _7;
_6 = _7;
_21 = [(-14302_i16)];
_19 = [true,true,false,false];
_6 = _15;
_12 = [(-20_i8),(-57_i8),95_i8,(-104_i8)];
_13 = 9223372036854775807_isize;
Goto(bb6)
}
bb16 = {
Return()
}
bb17 = {
_34 = !false;
_31.fld1 = (_7,);
_35 = !2917965605_u32;
_32 = _25 > _26.fld0.1;
_27.fld1.fld5.1 = (-9097114856759800111_i64) * 1976171316440938528_i64;
_27.fld2 = _16;
_1 = _5;
_27.fld1.fld0 = [_31.fld0.0,_17,_26.fld0.0,_27.fld0,_27.fld0,_26.fld0.0];
_16 = _18;
_18 = -_16;
_26.fld1 = _31.fld1;
_26.fld0.1 = _25;
_26.fld3 = Adt48::Variant1 { fld0: _26.fld0 };
_31 = Adt49 { fld0: _26.fld0,fld1: _26.fld1,fld2: Field::<(u128, isize, [u64; 2])>(Variant(_26.fld3, 1), 0).1,fld3: Move(_26.fld3) };
_9 = _3 & _3;
_37.0 = _16 as u128;
_8 = core::ptr::addr_of_mut!(_31.fld0.2);
_27.fld1.fld5.1 = !624125306021296621_i64;
_10 = _11 + _23;
_38 = _34;
Goto(bb18)
}
bb18 = {
Call(_39 = dump_var(6_usize, 16_usize, Move(_16), 1_usize, Move(_1), 38_usize, Move(_38), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(6_usize, 20_usize, Move(_20), 4_usize, Move(_4), 17_usize, Move(_17), 32_usize, Move(_32)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(6_usize, 14_usize, Move(_14), 34_usize, Move(_34), 40_usize, _40, 40_usize, _40), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *mut [u64; 2],mut _2: *mut [u64; 2],mut _3: [i8; 4],mut _4: [i16; 2],mut _5: f64,mut _6: *mut [u64; 2],mut _7: *mut [u64; 2],mut _8: [i8; 4],mut _9: u8,mut _10: *mut [u64; 2],mut _11: [bool; 4],mut _12: [i8; 4],mut _13: *mut [u64; 2],mut _14: *mut [u64; 2]) -> [i8; 4] {
mir! {
type RET = [i8; 4];
let _15: f32;
let _16: char;
let _17: f64;
let _18: [bool; 4];
let _19: *mut u32;
let _20: char;
let _21: Adt42;
let _22: [i16; 1];
let _23: Adt47;
let _24: [u128; 6];
let _25: i64;
let _26: u32;
let _27: bool;
let _28: [isize; 8];
let _29: Adt55;
let _30: [isize; 8];
let _31: [i8; 4];
let _32: [u16; 7];
let _33: ();
let _34: ();
{
_11 = [true,false,false,false];
_11 = [false,false,false,true];
RET = _3;
_8 = [(-32_i8),(-114_i8),110_i8,(-96_i8)];
_10 = _7;
_13 = _1;
_13 = _1;
_4 = [(-1170_i16),(-27572_i16)];
_5 = 30876_u16 as f64;
_8 = [(-60_i8),94_i8,71_i8,89_i8];
_15 = 42121_u16 as f32;
_6 = _2;
_16 = '\u{c518d}';
_6 = _1;
_2 = _13;
_10 = _6;
Call(_8 = fn8(_1, _11, _4, _12, _6, _14, _6, _3, _3, _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _10;
_15 = 78_i8 as f32;
_11 = [true,true,false,true];
_5 = 953047214_u32 as f64;
_9 = 223_u8 >> (-3766153807556447364_i64);
Goto(bb2)
}
bb2 = {
_12 = _8;
_6 = _7;
_10 = _14;
_18 = [true,true,true,false];
_6 = _10;
_15 = (-95601173038639414010150079429390039941_i128) as f32;
_11 = [false,false,true,false];
_15 = _9 as f32;
Goto(bb3)
}
bb3 = {
_8 = [91_i8,11_i8,98_i8,(-69_i8)];
_21.fld3 = 319961131835790898_usize >> _9;
_21.fld0 = [21836147039088605520247425086370102058_u128,208059845610294860364591867565407647273_u128,172701693609069077019695524793765208366_u128,269943565494278666864693414726849679338_u128,231882892758260579633919046188799789765_u128,127169368191318233931866106783077697056_u128];
_22 = [(-19223_i16)];
_16 = '\u{a637b}';
_21.fld5.1 = 1256674123502437032_i64;
_22 = [31756_i16];
_21.fld1 = 7941564379891828942_u64 & 10058992326851362349_u64;
_6 = _10;
_21.fld1 = (-19559_i16) as u64;
_13 = _10;
_2 = _7;
_21.fld6 = 1125914363_u32 << _21.fld1;
_2 = _6;
_21.fld4 = core::ptr::addr_of!(_9);
_21.fld0 = [324940311238310211986730009560663069575_u128,273201660463059727976517062109153633522_u128,130619901892332592011473036401757910800_u128,201184612940166685287996929320346084641_u128,3600665489696384388746702319052470064_u128,128144449714336921070991364788153770191_u128];
match _21.fld5.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1256674123502437032 => bb9,
_ => bb8
}
}
bb4 = {
_12 = _8;
_6 = _7;
_10 = _14;
_18 = [true,true,true,false];
_6 = _10;
_15 = (-95601173038639414010150079429390039941_i128) as f32;
_11 = [false,false,true,false];
_15 = _9 as f32;
Goto(bb3)
}
bb5 = {
_6 = _10;
_15 = 78_i8 as f32;
_11 = [true,true,false,true];
_5 = 953047214_u32 as f64;
_9 = 223_u8 >> (-3766153807556447364_i64);
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
_12 = _8;
_21.fld2 = [true,true,false,false];
match _21.fld5.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb7,
1256674123502437032 => bb10,
_ => bb6
}
}
bb10 = {
_17 = -_5;
_16 = '\u{f6c21}';
_10 = _14;
_19 = core::ptr::addr_of_mut!(_21.fld6);
_21.fld5.1 = (-52472335479430247_i64);
_20 = _16;
_22 = [(-29090_i16)];
_2 = _10;
_21.fld5 = (_19, 6365670459437913561_i64);
_14 = _1;
_8 = [(-103_i8),(-69_i8),(-76_i8),(-4_i8)];
_17 = -_5;
_24 = [318376429009953841935850309122898634512_u128,155435248139213631936838873837488113428_u128,239524521380051986117344019515198850504_u128,191625087679690763464460388460485577228_u128,246662009505523067663347493591505994837_u128,170636543598998081448740607369601468667_u128];
_12 = _3;
_25 = _21.fld5.1;
_26 = 15405_u16 as u32;
_12 = _8;
_11 = [false,false,true,true];
_21.fld0 = [277729087878885298496368978587685993467_u128,81358368314059184778480087227431270176_u128,281446983464220757361841198092962507206_u128,332167691497812157995727617360530271662_u128,136059602005374243198660223127205989758_u128,31102459463803860174245431294735691076_u128];
_21.fld3 = 2_usize - 4_usize;
match _21.fld5.1 {
0 => bb1,
1 => bb11,
6365670459437913561 => bb13,
_ => bb12
}
}
bb11 = {
_12 = _8;
_6 = _7;
_10 = _14;
_18 = [true,true,true,false];
_6 = _10;
_15 = (-95601173038639414010150079429390039941_i128) as f32;
_11 = [false,false,true,false];
_15 = _9 as f32;
Goto(bb3)
}
bb12 = {
Return()
}
bb13 = {
_21.fld5 = (_19, _25);
_16 = _20;
(*_19) = !_26;
Call(_9 = core::intrinsics::bswap(205_u8), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_21.fld0 = [182923350310736601987715764444989813137_u128,296565209737729696216098955325193720549_u128,335019466448762509660906828163319611857_u128,186908794603282338366607522944566126587_u128,256940756136800372124067384443324345580_u128,319816287608927923179228726955585591616_u128];
_6 = _7;
_28 = [(-123_isize),0_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_21.fld5 = (_19, _25);
RET = [65_i8,(-44_i8),116_i8,(-113_i8)];
_22 = [20411_i16];
_18 = _21.fld2;
_6 = _14;
_20 = _16;
_27 = !true;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(7_usize, 12_usize, Move(_12), 8_usize, Move(_8), 27_usize, Move(_27), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(7_usize, 3_usize, Move(_3), 28_usize, Move(_28), 16_usize, Move(_16), 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *mut [u64; 2],mut _2: [bool; 4],mut _3: [i16; 2],mut _4: [i8; 4],mut _5: *mut [u64; 2],mut _6: *mut [u64; 2],mut _7: *mut [u64; 2],mut _8: [i8; 4],mut _9: [i8; 4],mut _10: *mut [u64; 2],mut _11: *mut [u64; 2]) -> [i8; 4] {
mir! {
type RET = [i8; 4];
let _12: u64;
let _13: [i16; 1];
let _14: bool;
let _15: f32;
let _16: [isize; 8];
let _17: u128;
let _18: &'static [isize; 1];
let _19: Adt58;
let _20: &'static [isize; 1];
let _21: *mut u32;
let _22: (((*mut f32, isize, i64), u128, [i8; 4]), [isize; 8], &'static [isize; 1], f64);
let _23: [bool; 4];
let _24: f64;
let _25: [i16; 1];
let _26: f64;
let _27: [isize; 8];
let _28: char;
let _29: bool;
let _30: bool;
let _31: isize;
let _32: usize;
let _33: [bool; 4];
let _34: ();
let _35: ();
{
_7 = _11;
RET = _8;
Call(_2 = fn9(_11, _7, _5, _6, _4, RET, _1, _8, RET, RET, _11, _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _4;
_9 = [78_i8,117_i8,102_i8,(-37_i8)];
_2 = [false,false,true,false];
_11 = _10;
Goto(bb2)
}
bb2 = {
_15 = (-116394349322899667190845760860738451372_i128) as f32;
_14 = !true;
_15 = 234313011_u32 as f32;
_4 = [78_i8,63_i8,(-89_i8),(-21_i8)];
_9 = _4;
_14 = !false;
_7 = _10;
_13 = [19841_i16];
_15 = 4_usize as f32;
_7 = _1;
_5 = _1;
_10 = _7;
_11 = _1;
_19.fld0.fld1.fld5.1 = 1170853222243936552_i64;
Goto(bb3)
}
bb3 = {
_19.fld0.fld1.fld6 = !1484365182_u32;
_10 = _11;
RET = [78_i8,(-91_i8),(-38_i8),126_i8];
_19.fld5.fld6 = _19.fld0.fld1.fld6 & _19.fld0.fld1.fld6;
_19.fld0.fld1.fld3 = !1_usize;
_19.fld0.fld1.fld0 = [201500836627859991251333007668325716451_u128,76286755570059321592676780221619511924_u128,303846724375515547666656434761209483515_u128,60964750834448632788575604560981396792_u128,301532392794454220271720168698482480201_u128,13546204320669519277339903772943834990_u128];
_25 = [(-30248_i16)];
_12 = 11394877596940141564_u64;
_19.fld0.fld1.fld1 = !_12;
_7 = _5;
match _19.fld0.fld1.fld5.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
1170853222243936552 => bb9,
_ => bb8
}
}
bb4 = {
_15 = (-116394349322899667190845760860738451372_i128) as f32;
_14 = !true;
_15 = 234313011_u32 as f32;
_4 = [78_i8,63_i8,(-89_i8),(-21_i8)];
_9 = _4;
_14 = !false;
_7 = _10;
_13 = [19841_i16];
_15 = 4_usize as f32;
_7 = _1;
_5 = _1;
_10 = _7;
_11 = _1;
_19.fld0.fld1.fld5.1 = 1170853222243936552_i64;
Goto(bb3)
}
bb5 = {
_8 = _4;
_9 = [78_i8,117_i8,102_i8,(-37_i8)];
_2 = [false,false,true,false];
_11 = _10;
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
_19.fld4.0 = core::ptr::addr_of_mut!(_19.fld5.fld6);
_19.fld5.fld1 = _12;
Call(_22.0.0.0 = fn17(RET, _19.fld0.fld1.fld5.1, RET, _19.fld4.0, _11, _19.fld4.0, _19.fld5.fld1, _7, _19.fld5.fld1, _19.fld4.0, _11, _19.fld4.0, _2, _19.fld0.fld1.fld0, _8, RET), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.fld4.0 = core::ptr::addr_of_mut!(_19.fld0.fld1.fld6);
_19.fld0.fld0 = _19.fld0.fld1.fld5.1 as u128;
_22.0.2 = _9;
_19.fld1.0 = _11;
_8 = [26_i8,(-121_i8),81_i8,80_i8];
_11 = _7;
_13 = [(-4360_i16)];
_19.fld5.fld3 = _19.fld0.fld1.fld3;
_14 = false;
_19.fld4.0 = core::ptr::addr_of_mut!(_19.fld5.fld6);
_14 = true ^ false;
_19.fld0.fld1.fld2 = [_14,_14,_14,_14];
_19.fld0.fld0 = 89189394800635052040397090989813853450_u128 >> _19.fld0.fld1.fld3;
_8 = [14_i8,(-78_i8),8_i8,122_i8];
_22.3 = 20_u8 as f64;
_13 = [(-31211_i16)];
_22.0.1 = !_19.fld0.fld0;
_15 = (-1969503737_i32) as f32;
_19.fld5.fld0 = [_22.0.1,_19.fld0.fld0,_19.fld0.fld0,_19.fld0.fld0,_22.0.1,_22.0.1];
_22.0.0.2 = _22.3 as i64;
_19.fld0.fld1.fld0 = [_22.0.1,_22.0.1,_22.0.1,_22.0.1,_19.fld0.fld0,_22.0.1];
_27 = [(-88_isize),(-30_isize),9223372036854775807_isize,(-61_isize),(-91_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_33 = _19.fld0.fld1.fld2;
_22.1 = [(-9223372036854775808_isize),(-97_isize),12_isize,(-9223372036854775808_isize),9223372036854775807_isize,95_isize,9223372036854775807_isize,91_isize];
match _19.fld0.fld1.fld5.1 {
0 => bb1,
1 => bb3,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
1170853222243936552 => bb16,
_ => bb15
}
}
bb11 = {
_19.fld4.0 = core::ptr::addr_of_mut!(_19.fld5.fld6);
_19.fld5.fld1 = _12;
Call(_22.0.0.0 = fn17(RET, _19.fld0.fld1.fld5.1, RET, _19.fld4.0, _11, _19.fld4.0, _19.fld5.fld1, _7, _19.fld5.fld1, _19.fld4.0, _11, _19.fld4.0, _2, _19.fld0.fld1.fld0, _8, RET), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_15 = (-116394349322899667190845760860738451372_i128) as f32;
_14 = !true;
_15 = 234313011_u32 as f32;
_4 = [78_i8,63_i8,(-89_i8),(-21_i8)];
_9 = _4;
_14 = !false;
_7 = _10;
_13 = [19841_i16];
_15 = 4_usize as f32;
_7 = _1;
_5 = _1;
_10 = _7;
_11 = _1;
_19.fld0.fld1.fld5.1 = 1170853222243936552_i64;
Goto(bb3)
}
bb14 = {
Return()
}
bb15 = {
_15 = (-116394349322899667190845760860738451372_i128) as f32;
_14 = !true;
_15 = 234313011_u32 as f32;
_4 = [78_i8,63_i8,(-89_i8),(-21_i8)];
_9 = _4;
_14 = !false;
_7 = _10;
_13 = [19841_i16];
_15 = 4_usize as f32;
_7 = _1;
_5 = _1;
_10 = _7;
_11 = _1;
_19.fld0.fld1.fld5.1 = 1170853222243936552_i64;
Goto(bb3)
}
bb16 = {
_19.fld5.fld2 = _2;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(8_usize, 13_usize, Move(_13), 14_usize, Move(_14), 33_usize, Move(_33), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(8_usize, 3_usize, Move(_3), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut [u64; 2],mut _2: *mut [u64; 2],mut _3: *mut [u64; 2],mut _4: *mut [u64; 2],mut _5: [i8; 4],mut _6: [i8; 4],mut _7: *mut [u64; 2],mut _8: [i8; 4],mut _9: [i8; 4],mut _10: [i8; 4],mut _11: *mut [u64; 2],mut _12: *mut [u64; 2],mut _13: *mut [u64; 2]) -> [bool; 4] {
mir! {
type RET = [bool; 4];
let _14: usize;
let _15: char;
let _16: (*mut f32, isize, i64);
let _17: bool;
let _18: char;
let _19: u64;
let _20: u32;
let _21: Adt44;
let _22: Adt45;
let _23: (char, i16, char);
let _24: [i16; 2];
let _25: isize;
let _26: (*mut u32, i64);
let _27: char;
let _28: char;
let _29: u64;
let _30: Adt57;
let _31: char;
let _32: (*mut f32, isize, i64);
let _33: [i8; 4];
let _34: Adt50;
let _35: (*mut u8,);
let _36: f64;
let _37: Adt58;
let _38: isize;
let _39: ();
let _40: ();
{
_8 = _5;
_3 = _4;
_8 = _5;
_10 = _9;
_12 = _7;
_2 = _3;
_7 = _13;
_2 = _7;
_8 = [79_i8,(-116_i8),(-21_i8),(-9_i8)];
_1 = _7;
_1 = _2;
_9 = _10;
_8 = [4_i8,72_i8,125_i8,(-124_i8)];
_8 = _9;
_3 = _1;
_7 = _13;
_1 = _12;
Call(_7 = fn10(_3, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [7_i8,(-20_i8),35_i8,68_i8];
_1 = _13;
_2 = _7;
Goto(bb2)
}
bb2 = {
_13 = _4;
_13 = _3;
RET = [true,false,true,true];
_1 = _12;
_12 = _3;
_5 = [63_i8,52_i8,72_i8,75_i8];
_9 = [72_i8,(-87_i8),(-48_i8),(-104_i8)];
Goto(bb3)
}
bb3 = {
_6 = [28_i8,62_i8,(-62_i8),(-80_i8)];
_1 = _13;
_6 = [(-64_i8),(-13_i8),(-36_i8),(-19_i8)];
_8 = _5;
_2 = _11;
_15 = '\u{679c4}';
_8 = _10;
_11 = _4;
_8 = _5;
_2 = _7;
_5 = _10;
_1 = _7;
_3 = _13;
_3 = _13;
_14 = 281844721896178216_usize + 1_usize;
_3 = _7;
_15 = '\u{17cb2}';
_2 = _11;
RET = [true,true,false,false];
_11 = _13;
_12 = _4;
_12 = _13;
RET = [false,true,false,true];
_16.1 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_18 = _15;
_1 = _4;
Goto(bb4)
}
bb4 = {
_7 = _12;
_14 = !4_usize;
_3 = _2;
_8 = [97_i8,4_i8,61_i8,35_i8];
_3 = _13;
_10 = [105_i8,7_i8,20_i8,(-116_i8)];
_16.2 = !(-4128213300994451675_i64);
_9 = _5;
_15 = _18;
Goto(bb5)
}
bb5 = {
_9 = _5;
_16.2 = 5729041231600964250_i64;
match _16.2 {
0 => bb1,
1 => bb2,
2 => bb3,
5729041231600964250 => bb6,
_ => bb4
}
}
bb6 = {
_21.fld3 = core::ptr::addr_of_mut!(_20);
_21.fld1.fld0 = [9837531787595307927751142678255224505_u128,328567805041027808621231421973735601833_u128,242008342052800654932521776706833540594_u128,33585052331927030607306256943510050945_u128,262747732438053738639934380209814143016_u128,60456203639737736464843067872843897704_u128];
_14 = 6630125264635052126_usize;
_21.fld1.fld6 = !2734164329_u32;
_19 = 17502439992707937763_u64 & 5630777516340148808_u64;
_21.fld1.fld1 = _19 << _16.1;
_21.fld1.fld1 = _19 & _19;
_21.fld5 = !173144391_i32;
_19 = _21.fld1.fld1;
_23.1 = 171_u8 as i16;
_4 = _1;
_17 = false;
_21.fld1.fld5.1 = 50369_u16 as i64;
_23.0 = _18;
_18 = _23.0;
_25 = _16.1 | _16.1;
_4 = _1;
_14 = !7_usize;
_12 = _7;
_21.fld4 = _14 as u64;
_21.fld1.fld3 = _14 << _21.fld1.fld1;
_24 = [_23.1,_23.1];
_26.1 = _16.2 - _21.fld1.fld5.1;
Call(_21.fld2 = fn14(_21.fld1.fld1, _5, _25, _15, _1, _5, _16.1, _3, _18, _21.fld1.fld0, _13, _11, _15, _17, _23.0, _26.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_26.1 = _21.fld1.fld5.1;
_21.fld1.fld2 = [_17,_17,_17,_17];
_26.0 = core::ptr::addr_of_mut!(_20);
_21.fld4 = _21.fld1.fld1;
_21.fld1.fld6 = 107_u8 as u32;
_27 = _18;
RET = [_17,_17,_17,_17];
_7 = _1;
_14 = _21.fld1.fld3;
_15 = _18;
_1 = _3;
_2 = _11;
_11 = _1;
_5 = [101_i8,83_i8,(-36_i8),71_i8];
_28 = _23.0;
RET = [_17,_17,_17,_17];
_27 = _23.0;
_26.1 = _21.fld1.fld5.1 & _16.2;
Goto(bb8)
}
bb8 = {
_17 = !true;
_29 = !_21.fld4;
_23.2 = _27;
_18 = _23.0;
_9 = [49_i8,59_i8,(-31_i8),93_i8];
_21.fld5 = -(-180899129_i32);
_25 = _16.1;
_21.fld6 = [_17,_17,_17,_17];
_21.fld1.fld1 = _29 - _19;
_21.fld1.fld5.0 = core::ptr::addr_of_mut!(_20);
_26 = _21.fld1.fld5;
_8 = [80_i8,7_i8,23_i8,(-67_i8)];
_3 = _13;
_18 = _28;
_4 = _12;
_11 = _3;
_5 = [10_i8,(-68_i8),(-97_i8),37_i8];
_21.fld1.fld3 = !_14;
_9 = _8;
_16.1 = _25 << _21.fld1.fld3;
_20 = !_21.fld1.fld6;
_20 = !_21.fld1.fld6;
RET = [_17,_17,_17,_17];
_18 = _28;
_26 = (_21.fld1.fld5.0, _16.2);
_21.fld1.fld6 = _20 >> _14;
_10 = [(-57_i8),38_i8,(-84_i8),(-50_i8)];
_21.fld1.fld0 = [301735629069978450100815099260643052320_u128,67914992009697889334189729204594663136_u128,234404743098996153342409604295688632593_u128,105868644724946498165123844411598786719_u128,234290720501788084044024144852323890510_u128,115349306616073780262601839781030705801_u128];
match _16.2 {
0 => bb9,
1 => bb10,
2 => bb11,
5729041231600964250 => bb13,
_ => bb12
}
}
bb9 = {
_13 = _4;
_13 = _3;
RET = [true,false,true,true];
_1 = _12;
_12 = _3;
_5 = [63_i8,52_i8,72_i8,75_i8];
_9 = [72_i8,(-87_i8),(-48_i8),(-104_i8)];
Goto(bb3)
}
bb10 = {
_21.fld3 = core::ptr::addr_of_mut!(_20);
_21.fld1.fld0 = [9837531787595307927751142678255224505_u128,328567805041027808621231421973735601833_u128,242008342052800654932521776706833540594_u128,33585052331927030607306256943510050945_u128,262747732438053738639934380209814143016_u128,60456203639737736464843067872843897704_u128];
_14 = 6630125264635052126_usize;
_21.fld1.fld6 = !2734164329_u32;
_19 = 17502439992707937763_u64 & 5630777516340148808_u64;
_21.fld1.fld1 = _19 << _16.1;
_21.fld1.fld1 = _19 & _19;
_21.fld5 = !173144391_i32;
_19 = _21.fld1.fld1;
_23.1 = 171_u8 as i16;
_4 = _1;
_17 = false;
_21.fld1.fld5.1 = 50369_u16 as i64;
_23.0 = _18;
_18 = _23.0;
_25 = _16.1 | _16.1;
_4 = _1;
_14 = !7_usize;
_12 = _7;
_21.fld4 = _14 as u64;
_21.fld1.fld3 = _14 << _21.fld1.fld1;
_24 = [_23.1,_23.1];
_26.1 = _16.2 - _21.fld1.fld5.1;
Call(_21.fld2 = fn14(_21.fld1.fld1, _5, _25, _15, _1, _5, _16.1, _3, _18, _21.fld1.fld0, _13, _11, _15, _17, _23.0, _26.1), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_8 = [7_i8,(-20_i8),35_i8,68_i8];
_1 = _13;
_2 = _7;
Goto(bb2)
}
bb12 = {
_6 = [28_i8,62_i8,(-62_i8),(-80_i8)];
_1 = _13;
_6 = [(-64_i8),(-13_i8),(-36_i8),(-19_i8)];
_8 = _5;
_2 = _11;
_15 = '\u{679c4}';
_8 = _10;
_11 = _4;
_8 = _5;
_2 = _7;
_5 = _10;
_1 = _7;
_3 = _13;
_3 = _13;
_14 = 281844721896178216_usize + 1_usize;
_3 = _7;
_15 = '\u{17cb2}';
_2 = _11;
RET = [true,true,false,false];
_11 = _13;
_12 = _4;
_12 = _13;
RET = [false,true,false,true];
_16.1 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_18 = _15;
_1 = _4;
Goto(bb4)
}
bb13 = {
_3 = _1;
_19 = !_21.fld1.fld1;
_21.fld1.fld5.0 = core::ptr::addr_of_mut!(_21.fld1.fld6);
_27 = _28;
_5 = _10;
_12 = _3;
_21.fld0 = 150419237270074450961108832582290006641_u128 - 234576698197600405480540719453219778005_u128;
_21.fld1.fld5 = (_21.fld3, _26.1);
_32.2 = _21.fld1.fld5.1 >> _19;
_21.fld1.fld6 = _23.1 as u32;
_24 = [_23.1,_23.1];
_21.fld1.fld1 = !_19;
_6 = [(-60_i8),(-52_i8),(-14_i8),(-100_i8)];
_21.fld1.fld3 = !_14;
_17 = !true;
_15 = _23.2;
_18 = _23.2;
_21.fld2 = _21.fld0 as i128;
_21.fld3 = _26.0;
_13 = _4;
_23.2 = _27;
_16.1 = -_25;
_21.fld1.fld6 = _20;
_27 = _15;
match _16.2 {
0 => bb1,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb14,
5 => bb15,
6 => bb16,
5729041231600964250 => bb18,
_ => bb17
}
}
bb14 = {
_8 = [7_i8,(-20_i8),35_i8,68_i8];
_1 = _13;
_2 = _7;
Goto(bb2)
}
bb15 = {
_8 = [7_i8,(-20_i8),35_i8,68_i8];
_1 = _13;
_2 = _7;
Goto(bb2)
}
bb16 = {
_13 = _4;
_13 = _3;
RET = [true,false,true,true];
_1 = _12;
_12 = _3;
_5 = [63_i8,52_i8,72_i8,75_i8];
_9 = [72_i8,(-87_i8),(-48_i8),(-104_i8)];
Goto(bb3)
}
bb17 = {
_6 = [28_i8,62_i8,(-62_i8),(-80_i8)];
_1 = _13;
_6 = [(-64_i8),(-13_i8),(-36_i8),(-19_i8)];
_8 = _5;
_2 = _11;
_15 = '\u{679c4}';
_8 = _10;
_11 = _4;
_8 = _5;
_2 = _7;
_5 = _10;
_1 = _7;
_3 = _13;
_3 = _13;
_14 = 281844721896178216_usize + 1_usize;
_3 = _7;
_15 = '\u{17cb2}';
_2 = _11;
RET = [true,true,false,false];
_11 = _13;
_12 = _4;
_12 = _13;
RET = [false,true,false,true];
_16.1 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_18 = _15;
_1 = _4;
Goto(bb4)
}
bb18 = {
_15 = _28;
_10 = [84_i8,(-8_i8),(-34_i8),(-80_i8)];
_21.fld0 = _21.fld5 as u128;
_24 = [_23.1,_23.1];
_37.fld0.fld5 = _21.fld1.fld3 as i32;
_37.fld0.fld2 = !_21.fld2;
_37.fld5.fld5.0 = _21.fld1.fld5.0;
_37.fld5.fld2 = [_17,_17,_17,_17];
_37.fld0.fld1.fld1 = _19 & _19;
_21.fld4 = !_19;
_11 = _2;
_32.1 = -_16.1;
_32.2 = _26.1 << _21.fld1.fld3;
_33 = [119_i8,25_i8,32_i8,(-128_i8)];
_37.fld0.fld1.fld5.1 = _32.2;
_6 = _10;
_37.fld1 = (_1,);
_37.fld0.fld1.fld6 = _20 * _21.fld1.fld6;
_21.fld5 = 45305_u16 as i32;
_37.fld0.fld1.fld0 = _21.fld1.fld0;
Goto(bb19)
}
bb19 = {
Call(_39 = dump_var(9_usize, 5_usize, Move(_5), 14_usize, Move(_14), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(9_usize, 27_usize, Move(_27), 19_usize, Move(_19), 9_usize, Move(_9), 29_usize, Move(_29)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(9_usize, 18_usize, Move(_18), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *mut [u64; 2],mut _2: *mut [u64; 2]) -> *mut [u64; 2] {
mir! {
type RET = *mut [u64; 2];
let _3: isize;
let _4: (*mut f32, isize, i64);
let _5: Adt57;
let _6: char;
let _7: i16;
let _8: u64;
let _9: [i16; 4];
let _10: [i16; 4];
let _11: i16;
let _12: [i16; 1];
let _13: [i16; 1];
let _14: f32;
let _15: [isize; 1];
let _16: f32;
let _17: bool;
let _18: i128;
let _19: i8;
let _20: [i16; 1];
let _21: bool;
let _22: i128;
let _23: (char, i16, char);
let _24: *const u8;
let _25: i8;
let _26: [isize; 8];
let _27: usize;
let _28: ();
let _29: ();
{
_1 = _2;
RET = _1;
_1 = _2;
RET = _2;
_3 = -9223372036854775807_isize;
_2 = _1;
RET = _1;
RET = _2;
_1 = _2;
_1 = _2;
RET = _2;
_3 = 5_usize as isize;
_1 = RET;
_4.1 = (-68_i8) as isize;
RET = _2;
_2 = RET;
_2 = _1;
_1 = _2;
_4.1 = _3 ^ _3;
RET = _2;
RET = _1;
_2 = RET;
_4.1 = _3;
_2 = RET;
RET = _2;
RET = _1;
RET = _1;
Goto(bb1)
}
bb1 = {
RET = _2;
RET = _1;
_1 = _2;
_1 = RET;
_1 = _2;
_6 = '\u{c0a8b}';
_2 = RET;
_4.2 = (-8044036196417657374_i64);
_6 = '\u{7747e}';
_1 = _2;
_4.1 = !_3;
_2 = _1;
RET = _1;
_6 = '\u{1a659}';
_4.1 = (-858295312_i32) as isize;
_4.2 = 168_u8 as i64;
_1 = _2;
Call(_1 = fn11(_3, _2, _4.2, _6, _6, _4.2, RET, _2, RET, _6, _6, _3, RET, _4.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.1 = _6 as isize;
_7 = 105_i8 as i16;
_6 = '\u{b88e8}';
_7 = 11769_i16 ^ 22457_i16;
_4.1 = -_3;
_1 = _2;
_2 = _1;
RET = _1;
_7 = 4_usize as i16;
_4.2 = 6_usize as i64;
_7 = -25894_i16;
RET = _2;
_7 = !25800_i16;
_4.1 = (-1371069691_i32) as isize;
_1 = _2;
_3 = _4.1 | _4.1;
_9 = [_7,_7,_7,_7];
_4.1 = -_3;
Goto(bb3)
}
bb3 = {
_8 = 17980509273594388109_u64;
_3 = _4.2 as isize;
_2 = _1;
_4.2 = (-7019391888695453537_i64);
_4.2 = (-5864438476103393786_i64) << _3;
_9 = [_7,_7,_7,_7];
RET = _1;
_3 = 44_u8 as isize;
RET = _2;
_8 = !7922021360632190952_u64;
_3 = _4.1 | _4.1;
_10 = _9;
_7 = 18330_i16;
_11 = _4.2 as i16;
_3 = -_4.1;
match _7 {
0 => bb2,
18330 => bb5,
_ => bb4
}
}
bb4 = {
RET = _2;
RET = _1;
_1 = _2;
_1 = RET;
_1 = _2;
_6 = '\u{c0a8b}';
_2 = RET;
_4.2 = (-8044036196417657374_i64);
_6 = '\u{7747e}';
_1 = _2;
_4.1 = !_3;
_2 = _1;
RET = _1;
_6 = '\u{1a659}';
_4.1 = (-858295312_i32) as isize;
_4.2 = 168_u8 as i64;
_1 = _2;
Call(_1 = fn11(_3, _2, _4.2, _6, _6, _4.2, RET, _2, RET, _6, _6, _3, RET, _4.2), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_4.1 = _3 + _3;
_10 = _9;
Goto(bb6)
}
bb6 = {
_11 = _4.1 as i16;
_1 = _2;
_9 = _10;
_11 = _7;
_2 = _1;
RET = _2;
_3 = _4.1;
_6 = '\u{19a68}';
_4.1 = -_3;
_11 = -_7;
_11 = !_7;
_3 = !_4.1;
_9 = _10;
_10 = [_11,_7,_7,_11];
_7 = !_11;
_10 = [_7,_11,_7,_11];
_9 = [_7,_7,_7,_7];
_8 = 1351304958_i32 as u64;
_4.1 = _3 >> _4.2;
_9 = [_7,_7,_7,_7];
_11 = _7 >> _4.2;
RET = _1;
_9 = [_11,_11,_11,_7];
_11 = _8 as i16;
RET = _1;
RET = _2;
_4.1 = _3 << _7;
Call(_8 = core::intrinsics::transmute(_4.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = _9;
_12 = [_11];
_8 = 7750069625583827364_u64;
_12 = [_11];
_9 = [_7,_7,_7,_11];
RET = _2;
_1 = _2;
_2 = _1;
_3 = _8 as isize;
RET = _2;
match _8 {
0 => bb3,
7750069625583827364 => bb8,
_ => bb5
}
}
bb8 = {
_12 = [_11];
_11 = 70473031094784703418740782377015814771_i128 as i16;
_7 = -_11;
_6 = '\u{55380}';
RET = _1;
RET = _2;
_4.2 = !(-4095215388720596647_i64);
_14 = 179_u8 as f32;
_15 = [_4.1];
_9 = [_7,_11,_11,_7];
_2 = _1;
_4.0 = core::ptr::addr_of_mut!(_16);
Call(_7 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = '\u{75981}';
_17 = !false;
_13 = [_7];
_14 = 144216042101709889746740584991734723701_u128 as f32;
_16 = 15419935645470041300_usize as f32;
_7 = _11 >> _3;
_6 = '\u{5607d}';
_11 = _7;
_4.2 = _16 as i64;
_10 = _9;
_17 = _11 == _11;
_4.1 = _11 as isize;
_16 = -_14;
RET = _1;
_17 = false;
_18 = !(-108238996341855071169325070496038956471_i128);
_17 = false;
_14 = _16;
_11 = 295073192883028834100380587789321786451_u128 as i16;
match _8 {
0 => bb3,
1 => bb10,
2 => bb11,
7750069625583827364 => bb13,
_ => bb12
}
}
bb10 = {
_12 = [_11];
_11 = 70473031094784703418740782377015814771_i128 as i16;
_7 = -_11;
_6 = '\u{55380}';
RET = _1;
RET = _2;
_4.2 = !(-4095215388720596647_i64);
_14 = 179_u8 as f32;
_15 = [_4.1];
_9 = [_7,_11,_11,_7];
_2 = _1;
_4.0 = core::ptr::addr_of_mut!(_16);
Call(_7 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
RET = _2;
RET = _1;
_1 = _2;
_1 = RET;
_1 = _2;
_6 = '\u{c0a8b}';
_2 = RET;
_4.2 = (-8044036196417657374_i64);
_6 = '\u{7747e}';
_1 = _2;
_4.1 = !_3;
_2 = _1;
RET = _1;
_6 = '\u{1a659}';
_4.1 = (-858295312_i32) as isize;
_4.2 = 168_u8 as i64;
_1 = _2;
Call(_1 = fn11(_3, _2, _4.2, _6, _6, _4.2, RET, _2, RET, _6, _6, _3, RET, _4.2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_8 = 17980509273594388109_u64;
_3 = _4.2 as isize;
_2 = _1;
_4.2 = (-7019391888695453537_i64);
_4.2 = (-5864438476103393786_i64) << _3;
_9 = [_7,_7,_7,_7];
RET = _1;
_3 = 44_u8 as isize;
RET = _2;
_8 = !7922021360632190952_u64;
_3 = _4.1 | _4.1;
_10 = _9;
_7 = 18330_i16;
_11 = _4.2 as i16;
_3 = -_4.1;
match _7 {
0 => bb2,
18330 => bb5,
_ => bb4
}
}
bb13 = {
RET = _1;
_20 = _13;
_4.1 = _3 >> _3;
_11 = !_7;
_19 = 35_i8;
_18 = !(-109209426737336292484687761252249620948_i128);
_9 = [_11,_11,_7,_7];
_17 = false;
_10 = _9;
_20 = [_7];
_10 = [_7,_11,_11,_11];
_15 = [_4.1];
_3 = -_4.1;
_21 = _17;
_12 = [_7];
_20 = [_11];
match _8 {
0 => bb2,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
7750069625583827364 => bb19,
_ => bb18
}
}
bb14 = {
RET = _2;
RET = _1;
_1 = _2;
_1 = RET;
_1 = _2;
_6 = '\u{c0a8b}';
_2 = RET;
_4.2 = (-8044036196417657374_i64);
_6 = '\u{7747e}';
_1 = _2;
_4.1 = !_3;
_2 = _1;
RET = _1;
_6 = '\u{1a659}';
_4.1 = (-858295312_i32) as isize;
_4.2 = 168_u8 as i64;
_1 = _2;
Call(_1 = fn11(_3, _2, _4.2, _6, _6, _4.2, RET, _2, RET, _6, _6, _3, RET, _4.2), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
RET = _2;
RET = _1;
_1 = _2;
_1 = RET;
_1 = _2;
_6 = '\u{c0a8b}';
_2 = RET;
_4.2 = (-8044036196417657374_i64);
_6 = '\u{7747e}';
_1 = _2;
_4.1 = !_3;
_2 = _1;
RET = _1;
_6 = '\u{1a659}';
_4.1 = (-858295312_i32) as isize;
_4.2 = 168_u8 as i64;
_1 = _2;
Call(_1 = fn11(_3, _2, _4.2, _6, _6, _4.2, RET, _2, RET, _6, _6, _3, RET, _4.2), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_4.1 = _3 + _3;
_10 = _9;
Goto(bb6)
}
bb17 = {
_11 = _4.1 as i16;
_1 = _2;
_9 = _10;
_11 = _7;
_2 = _1;
RET = _2;
_3 = _4.1;
_6 = '\u{19a68}';
_4.1 = -_3;
_11 = -_7;
_11 = !_7;
_3 = !_4.1;
_9 = _10;
_10 = [_11,_7,_7,_11];
_7 = !_11;
_10 = [_7,_11,_7,_11];
_9 = [_7,_7,_7,_7];
_8 = 1351304958_i32 as u64;
_4.1 = _3 >> _4.2;
_9 = [_7,_7,_7,_7];
_11 = _7 >> _4.2;
RET = _1;
_9 = [_11,_11,_11,_7];
_11 = _8 as i16;
RET = _1;
RET = _2;
_4.1 = _3 << _7;
Call(_8 = core::intrinsics::transmute(_4.1), ReturnTo(bb7), UnwindUnreachable())
}
bb18 = {
_12 = [_11];
_11 = 70473031094784703418740782377015814771_i128 as i16;
_7 = -_11;
_6 = '\u{55380}';
RET = _1;
RET = _2;
_4.2 = !(-4095215388720596647_i64);
_14 = 179_u8 as f32;
_15 = [_4.1];
_9 = [_7,_11,_11,_7];
_2 = _1;
_4.0 = core::ptr::addr_of_mut!(_16);
Call(_7 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb19 = {
_23 = (_6, _7, _6);
_23.1 = _19 as i16;
_2 = RET;
_22 = _18 << _4.1;
_15 = [_4.1];
_13 = _12;
_19 = 21_i8 * (-70_i8);
Goto(bb20)
}
bb20 = {
Call(_28 = dump_var(10_usize, 9_usize, Move(_9), 15_usize, Move(_15), 12_usize, Move(_12), 22_usize, Move(_22)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_28 = dump_var(10_usize, 13_usize, Move(_13), 19_usize, Move(_19), 8_usize, Move(_8), 18_usize, Move(_18)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: *mut [u64; 2],mut _3: i64,mut _4: char,mut _5: char,mut _6: i64,mut _7: *mut [u64; 2],mut _8: *mut [u64; 2],mut _9: *mut [u64; 2],mut _10: char,mut _11: char,mut _12: isize,mut _13: *mut [u64; 2],mut _14: i64) -> *mut [u64; 2] {
mir! {
type RET = *mut [u64; 2];
let _15: bool;
let _16: *mut [u64; 2];
let _17: [bool; 4];
let _18: Adt52;
let _19: char;
let _20: ();
let _21: ();
{
_14 = (-27792_i16) as i64;
_1 = _12;
_1 = _12 - _12;
_6 = _3 << _14;
RET = _2;
_9 = _13;
_2 = _9;
Call(_6 = fn12(_11, RET, _5, _2, _2, _7, _8, _1, _11, _10, RET, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !_12;
_10 = _5;
match _6 {
0 => bb2,
340282366920938463460784086950141795224 => bb4,
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
_4 = _11;
_2 = _7;
_7 = RET;
_7 = _13;
_5 = _4;
_11 = _4;
_12 = _1;
Goto(bb5)
}
bb5 = {
_2 = _7;
_7 = _13;
_1 = !_12;
RET = _9;
_12 = !_1;
match _6 {
0 => bb3,
1 => bb2,
2 => bb6,
340282366920938463460784086950141795224 => bb8,
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
_8 = _9;
match _6 {
0 => bb6,
1 => bb9,
340282366920938463460784086950141795224 => bb11,
_ => bb10
}
}
bb9 = {
_2 = _7;
_7 = _13;
_1 = !_12;
RET = _9;
_12 = !_1;
match _6 {
0 => bb3,
1 => bb2,
2 => bb6,
340282366920938463460784086950141795224 => bb8,
_ => bb7
}
}
bb10 = {
Return()
}
bb11 = {
_14 = _6;
_11 = _4;
_4 = _10;
_7 = _13;
_1 = _12 | _12;
_7 = _8;
Call(_12 = core::intrinsics::transmute(_14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7 = RET;
_7 = RET;
_9 = RET;
_15 = false;
RET = _9;
_2 = _8;
_8 = _13;
_2 = _7;
_7 = _8;
_10 = _11;
Goto(bb13)
}
bb13 = {
_16 = _8;
RET = _16;
_2 = _9;
_4 = _11;
_6 = _14;
Call(_12 = core::intrinsics::bswap(_1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = _11;
_8 = RET;
_11 = _10;
_3 = 8559474982364932216_usize as i64;
_7 = _8;
_12 = _1;
RET = _9;
_12 = _1 & _1;
_18.fld1.1 = 111_i8 as isize;
_18.fld0 = (-152564710491653872943026575703831289185_i128);
_18.fld5.fld2 = [_15,_15,_15,_15];
_18.fld5.fld6 = 2625790506_u32;
_18.fld5.fld3 = 214_u8 as usize;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(11_usize, 10_usize, Move(_10), 5_usize, Move(_5), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(11_usize, 11_usize, Move(_11), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: char,mut _2: *mut [u64; 2],mut _3: char,mut _4: *mut [u64; 2],mut _5: *mut [u64; 2],mut _6: *mut [u64; 2],mut _7: *mut [u64; 2],mut _8: isize,mut _9: char,mut _10: char,mut _11: *mut [u64; 2],mut _12: isize) -> i64 {
mir! {
type RET = i64;
let _13: &'static [isize; 1];
let _14: (char, i16, char);
let _15: [isize; 8];
let _16: [u16; 7];
let _17: [isize; 8];
let _18: [u16; 7];
let _19: [i16; 4];
let _20: u8;
let _21: isize;
let _22: u8;
let _23: Adt44;
let _24: u32;
let _25: Adt58;
let _26: isize;
let _27: Adt46;
let _28: f32;
let _29: isize;
let _30: char;
let _31: isize;
let _32: [i8; 4];
let _33: [isize; 1];
let _34: usize;
let _35: [bool; 4];
let _36: [bool; 4];
let _37: [i16; 1];
let _38: ();
let _39: ();
{
_7 = _2;
_1 = _3;
RET = 8171869006818775679_i64;
_9 = _1;
_11 = _7;
_4 = _5;
_11 = _2;
_14.0 = _9;
_3 = _9;
_16 = [58697_u16,21636_u16,5176_u16,28383_u16,13149_u16,35644_u16,22170_u16];
_18 = [12490_u16,11760_u16,37161_u16,58902_u16,25521_u16,12707_u16,2784_u16];
_14 = (_3, 8692_i16, _1);
_12 = -_8;
RET = (-2590520481626416232_i64);
_14.0 = _1;
_14.0 = _1;
_9 = _1;
_5 = _7;
_14.2 = _10;
Call(_21 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_19 = [_14.1,_14.1,_14.1,_14.1];
_8 = _12 * _12;
_11 = _7;
_20 = 168_u8 << _14.1;
_14.1 = 232285162_i32 as i16;
_15 = [_21,_12,_8,_8,_12,_8,_8,_8];
_16 = [56572_u16,14549_u16,42255_u16,58702_u16,17639_u16,35101_u16,64928_u16];
_14.0 = _10;
_10 = _14.0;
Call(_6 = fn13(_15, _16, _8, _11, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = [_8,_12,_21,_12,_8,_21,_8,_12];
_23.fld1.fld0 = [29331912034912035636515998984929319785_u128,54601349490083010210839809609008933926_u128,144247199526013457512800844762010383108_u128,334899802274992451801118746028257098271_u128,132497790120783712451765191133162305343_u128,269550062723144564936866816035514146255_u128];
_12 = _14.1 as isize;
_22 = _20;
_25.fld5.fld3 = 6_usize << _20;
_25.fld5.fld5.0 = core::ptr::addr_of_mut!(_25.fld0.fld1.fld6);
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463460784086950141795224 => bb6,
_ => bb5
}
}
bb3 = {
_19 = [_14.1,_14.1,_14.1,_14.1];
_8 = _12 * _12;
_11 = _7;
_20 = 168_u8 << _14.1;
_14.1 = 232285162_i32 as i16;
_15 = [_21,_12,_8,_8,_12,_8,_8,_8];
_16 = [56572_u16,14549_u16,42255_u16,58702_u16,17639_u16,35101_u16,64928_u16];
_14.0 = _10;
_10 = _14.0;
Call(_6 = fn13(_15, _16, _8, _11, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_15 = _17;
_25.fld2 = _25.fld5.fld3;
_25.fld0.fld5 = 1285789422_i32 * 1448156230_i32;
_25.fld0.fld1.fld6 = 2878917261_u32 + 3591456998_u32;
_25.fld0.fld1.fld5 = (_25.fld5.fld5.0, RET);
_25.fld0.fld0 = 173088362793528637989947810499254867598_u128 * 252948579633301376976110977095825062707_u128;
_25.fld0.fld3 = _25.fld0.fld1.fld5.0;
_25.fld0.fld2 = (-57972745348015546915564706571899986598_i128) * (-66068988093016752128407489735241585618_i128);
_25.fld5.fld2 = [true,false,false,true];
_25.fld0.fld1.fld3 = !_25.fld2;
_23.fld5 = _25.fld0.fld5;
_25.fld0.fld1.fld5.1 = RET & RET;
_25.fld0.fld1.fld1 = true as u64;
_25.fld0.fld0 = _25.fld0.fld1.fld5.1 as u128;
_25.fld4.0 = _25.fld0.fld3;
_23.fld6 = [true,false,false,true];
_23.fld3 = _25.fld5.fld5.0;
_25.fld0.fld1.fld2 = [false,false,true,false];
_25.fld5.fld3 = _25.fld0.fld1.fld3 * _25.fld2;
_25.fld0.fld1.fld6 = !3367870150_u32;
_23.fld1.fld5.1 = _25.fld0.fld1.fld5.1;
_14.2 = _10;
_23.fld1.fld5 = (_25.fld4.0, _25.fld0.fld1.fld5.1);
match RET {
0 => bb4,
1 => bb3,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463460784086950141795224 => bb12,
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
_19 = [_14.1,_14.1,_14.1,_14.1];
_8 = _12 * _12;
_11 = _7;
_20 = 168_u8 << _14.1;
_14.1 = 232285162_i32 as i16;
_15 = [_21,_12,_8,_8,_12,_8,_8,_8];
_16 = [56572_u16,14549_u16,42255_u16,58702_u16,17639_u16,35101_u16,64928_u16];
_14.0 = _10;
_10 = _14.0;
Call(_6 = fn13(_15, _16, _8, _11, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_17 = [_8,_12,_21,_12,_8,_21,_8,_12];
_23.fld1.fld0 = [29331912034912035636515998984929319785_u128,54601349490083010210839809609008933926_u128,144247199526013457512800844762010383108_u128,334899802274992451801118746028257098271_u128,132497790120783712451765191133162305343_u128,269550062723144564936866816035514146255_u128];
_12 = _14.1 as isize;
_22 = _20;
_25.fld5.fld3 = 6_usize << _20;
_25.fld5.fld5.0 = core::ptr::addr_of_mut!(_25.fld0.fld1.fld6);
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463460784086950141795224 => bb6,
_ => bb5
}
}
bb11 = {
_19 = [_14.1,_14.1,_14.1,_14.1];
_8 = _12 * _12;
_11 = _7;
_20 = 168_u8 << _14.1;
_14.1 = 232285162_i32 as i16;
_15 = [_21,_12,_8,_8,_12,_8,_8,_8];
_16 = [56572_u16,14549_u16,42255_u16,58702_u16,17639_u16,35101_u16,64928_u16];
_14.0 = _10;
_10 = _14.0;
Call(_6 = fn13(_15, _16, _8, _11, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_14.1 = 7830_i16;
_28 = _25.fld0.fld1.fld6 as f32;
_25.fld0.fld6 = _25.fld5.fld2;
_25.fld1.0 = _2;
_25.fld1 = (_4,);
_15 = [_21,_8,_12,_12,_12,_21,_8,_8];
_25.fld0.fld1.fld2 = [true,true,true,true];
_25.fld4.1 = -RET;
_25.fld5.fld5.1 = -_25.fld4.1;
_23.fld1.fld5 = (_25.fld4.0, _25.fld0.fld1.fld5.1);
_25.fld5.fld5.1 = RET >> _8;
_23.fld1.fld6 = _8 as u32;
_15 = _17;
_25.fld0.fld1.fld4 = core::ptr::addr_of!(_20);
_25.fld4 = (_23.fld3, _25.fld5.fld5.1);
_1 = _9;
_30 = _9;
_25.fld3 = _25.fld0.fld1.fld1 as i8;
_25.fld5 = Adt42 { fld0: _23.fld1.fld0,fld1: _25.fld0.fld1.fld1,fld2: _25.fld0.fld6,fld3: _25.fld2,fld4: _25.fld0.fld1.fld4,fld5: _25.fld4,fld6: _23.fld1.fld6 };
_25.fld0.fld1.fld5.0 = core::ptr::addr_of_mut!(_23.fld1.fld6);
_3 = _9;
_19 = [_14.1,_14.1,_14.1,_14.1];
Goto(bb13)
}
bb13 = {
_25.fld0.fld3 = _25.fld4.0;
_25.fld0.fld1.fld5 = _25.fld5.fld5;
_19 = [_14.1,_14.1,_14.1,_14.1];
_12 = -_8;
_25.fld0.fld3 = _25.fld0.fld1.fld5.0;
_20 = _22;
_25.fld5.fld3 = !_25.fld2;
_3 = _9;
_14 = (_30, (-9459_i16), _1);
_25.fld0.fld1 = Adt42 { fld0: _25.fld5.fld0,fld1: _25.fld5.fld1,fld2: _23.fld6,fld3: _25.fld2,fld4: _25.fld5.fld4,fld5: _23.fld1.fld5,fld6: _23.fld1.fld6 };
_25.fld0.fld1.fld2 = [false,true,true,true];
_31 = _25.fld3 as isize;
_25.fld4.0 = core::ptr::addr_of_mut!(_24);
Goto(bb14)
}
bb14 = {
_3 = _9;
_10 = _14.0;
_23.fld1.fld2 = [false,true,true,false];
_23.fld1.fld4 = _25.fld0.fld1.fld4;
_25.fld0.fld1.fld2 = _25.fld0.fld6;
_24 = _25.fld0.fld1.fld6 >> _25.fld5.fld3;
_5 = _7;
_33 = [_8];
_25.fld2 = _25.fld0.fld1.fld3;
_23.fld1.fld5.0 = _25.fld0.fld1.fld5.0;
_5 = _4;
_25.fld5.fld2 = _25.fld0.fld1.fld2;
_34 = _25.fld5.fld5.1 as usize;
_20 = _23.fld5 as u8;
_25.fld0.fld3 = _25.fld4.0;
_23.fld1.fld1 = _14.1 as u64;
_17 = [_8,_12,_8,_8,_8,_12,_12,_8];
_23 = Adt44 { fld0: _25.fld0.fld0,fld1: Move(_25.fld0.fld1),fld2: _25.fld0.fld2,fld3: _25.fld4.0,fld4: _25.fld0.fld1.fld1,fld5: _25.fld0.fld5,fld6: _25.fld0.fld1.fld2 };
_25.fld0.fld1.fld2 = [false,true,true,true];
_31 = _1 as isize;
_35 = [true,false,false,true];
_23.fld1 = Adt42 { fld0: _25.fld5.fld0,fld1: _23.fld4,fld2: _25.fld0.fld1.fld2,fld3: _25.fld2,fld4: _25.fld5.fld4,fld5: _25.fld4,fld6: _25.fld5.fld6 };
_18 = [2121_u16,2475_u16,53720_u16,3274_u16,58023_u16,48265_u16,52341_u16];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(12_usize, 19_usize, Move(_19), 8_usize, Move(_8), 17_usize, Move(_17), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(12_usize, 14_usize, Move(_14), 34_usize, Move(_34), 9_usize, Move(_9), 35_usize, Move(_35)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(12_usize, 10_usize, Move(_10), 22_usize, Move(_22), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [isize; 8],mut _2: [u16; 7],mut _3: isize,mut _4: *mut [u64; 2],mut _5: isize) -> *mut [u64; 2] {
mir! {
type RET = *mut [u64; 2];
let _6: [i16; 1];
let _7: &'static [isize; 1];
let _8: [i16; 1];
let _9: Adt42;
let _10: [bool; 4];
let _11: i32;
let _12: [isize; 8];
let _13: f32;
let _14: isize;
let _15: i128;
let _16: u32;
let _17: (*mut u32, i64);
let _18: [u16; 7];
let _19: u128;
let _20: isize;
let _21: Adt52;
let _22: u32;
let _23: (char, i16, char);
let _24: [isize; 8];
let _25: Adt51;
let _26: [u16; 7];
let _27: [i16; 4];
let _28: [u64; 2];
let _29: f64;
let _30: (*mut f32, isize, i64);
let _31: [i8; 4];
let _32: usize;
let _33: ();
let _34: ();
{
RET = _4;
RET = _4;
RET = _4;
_3 = _5 >> _5;
_6 = [25067_i16];
RET = _4;
_9.fld5.1 = 8824574297824401428_i64;
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_8 = _6;
_2 = [11198_u16,61387_u16,48614_u16,58643_u16,51363_u16,4537_u16,57267_u16];
_9.fld0 = [327343991789332075682022177059917486842_u128,74773166319079365485811548007773326004_u128,79800899336697344459694084530098766753_u128,74901359491948512945627071526187705737_u128,165102400783676367861481833509771705651_u128,17450175028222333822378713605513011021_u128];
_10 = [false,false,true,false];
_4 = RET;
RET = _4;
_9.fld0 = [43306967181890760500119917339524559671_u128,79797988730969939160866410751892266685_u128,153367191547476292754292418035111572847_u128,68897968502628819176629056219541330017_u128,223849204664598462477984021909582065235_u128,48356615451966162457024824144359515995_u128];
_6 = _8;
_10 = [false,false,false,true];
_9.fld0 = [189895799714301537951153265858653631371_u128,240496426446431391224708723416727578751_u128,301157084693660294352923396885162850102_u128,150665221757686378537179476060490820719_u128,203163761433713134145977299104839573049_u128,153154789939196060651586434686630013623_u128];
_10 = [true,true,false,false];
_4 = RET;
_8 = _6;
_6 = [(-7949_i16)];
_10 = [false,true,false,false];
_1 = [_5,_5,_5,_5,_3,_5,_3,_3];
match _9.fld5.1 {
0 => bb1,
1 => bb2,
8824574297824401428 => bb4,
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
_9.fld2 = [false,false,false,false];
_11 = '\u{740ea}' as i32;
_9.fld3 = 7_usize ^ 8084951912009034110_usize;
_2 = [39635_u16,44176_u16,63473_u16,62663_u16,58514_u16,20331_u16,54938_u16];
_8 = _6;
_2 = [10232_u16,44285_u16,37017_u16,13837_u16,26973_u16,3588_u16,32245_u16];
_4 = RET;
_6 = [9093_i16];
_5 = false as isize;
_9.fld2 = [false,true,false,true];
_9.fld6 = 3161440091_u32 & 1787330147_u32;
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_9.fld1 = true as u64;
_13 = (-125593711934936859003603695227689706765_i128) as f32;
_1 = [_3,_3,_3,_3,_3,_5,_3,_3];
_11 = 1850736745_i32;
Goto(bb5)
}
bb5 = {
_9.fld5.1 = 134_u8 as i64;
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_6 = _8;
_9.fld5.1 = -3552209247838246250_i64;
_6 = [(-18150_i16)];
RET = _4;
_9.fld0 = [101008227040625786107260940694649136772_u128,112125592698296030653835613532241061971_u128,120861588889401179078762791675633198771_u128,264889447080147250815304598890392901102_u128,219413115764108969773290941353564558713_u128,137936447894083911794047335844061966286_u128];
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_9.fld2 = [true,false,false,false];
match _11 {
0 => bb6,
1850736745 => bb8,
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
_10 = [false,false,false,true];
match _11 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
1850736745 => bb9,
_ => bb6
}
}
bb9 = {
_6 = [(-30349_i16)];
_9.fld1 = 148_u8 as u64;
_17 = (_9.fld5.0, _9.fld5.1);
_15 = -80612191254700115226779308090424719298_i128;
_9.fld3 = 391_i16 as usize;
_17.1 = _9.fld5.1 - _9.fld5.1;
_9.fld3 = !14649402637271931980_usize;
_9.fld0 = [317400562358200398838050260658865402209_u128,58651894978739043176926576565683253049_u128,284522705138294376788808566737868198894_u128,190769275737395228565979424448646084922_u128,154922948414579959059010955481250972730_u128,588350197580330619286121586661666784_u128];
_3 = 153_u8 as isize;
_13 = 127_i8 as f32;
_9.fld1 = (-11_i8) as u64;
_9.fld2 = [false,false,true,true];
_9.fld1 = 7702680799698180894_u64 | 7432211531701878406_u64;
_9.fld5.0 = core::ptr::addr_of_mut!(_16);
_9.fld1 = 2918365321081087348_u64 * 11035645173840886679_u64;
_16 = !_9.fld6;
_8 = [31396_i16];
_12 = _1;
_9.fld1 = !5593499058540587947_u64;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19 = 69435038755430714430797723579537065739_u128;
_9.fld0 = [_19,_19,_19,_19,_19,_19];
_15 = 44363783784521645705255579573531109049_i128 - (-85951722547288496442152221717655868338_i128);
_9.fld5.0 = _17.0;
_15 = _9.fld1 as i128;
_11 = 683967392_i32;
_21.fld0 = !_15;
_9.fld3 = 2085457012396060529_usize;
_9.fld5.1 = _17.1 ^ _17.1;
_21.fld5.fld0 = [_19,_19,_19,_19,_19,_19];
_21.fld1.2 = _17.1 << _17.1;
_17.1 = _9.fld5.1 ^ _21.fld1.2;
_9.fld6 = _16 * _16;
match _19 {
0 => bb11,
69435038755430714430797723579537065739 => bb13,
_ => bb12
}
}
bb11 = {
_6 = [(-30349_i16)];
_9.fld1 = 148_u8 as u64;
_17 = (_9.fld5.0, _9.fld5.1);
_15 = -80612191254700115226779308090424719298_i128;
_9.fld3 = 391_i16 as usize;
_17.1 = _9.fld5.1 - _9.fld5.1;
_9.fld3 = !14649402637271931980_usize;
_9.fld0 = [317400562358200398838050260658865402209_u128,58651894978739043176926576565683253049_u128,284522705138294376788808566737868198894_u128,190769275737395228565979424448646084922_u128,154922948414579959059010955481250972730_u128,588350197580330619286121586661666784_u128];
_3 = 153_u8 as isize;
_13 = 127_i8 as f32;
_9.fld1 = (-11_i8) as u64;
_9.fld2 = [false,false,true,true];
_9.fld1 = 7702680799698180894_u64 | 7432211531701878406_u64;
_9.fld5.0 = core::ptr::addr_of_mut!(_16);
_9.fld1 = 2918365321081087348_u64 * 11035645173840886679_u64;
_16 = !_9.fld6;
_8 = [31396_i16];
_12 = _1;
_9.fld1 = !5593499058540587947_u64;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_8 = [13911_i16];
_21.fld2 = RET;
_21.fld0 = 6361_i16 as i128;
_21.fld4 = -8843_i16;
_23.2 = '\u{10e30f}';
_21.fld5.fld6 = _3 as u32;
_21.fld5.fld5.0 = core::ptr::addr_of_mut!(_21.fld5.fld6);
match _19 {
0 => bb3,
1 => bb8,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
69435038755430714430797723579537065739 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
_9.fld5.1 = 134_u8 as i64;
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_6 = _8;
_9.fld5.1 = -3552209247838246250_i64;
_6 = [(-18150_i16)];
RET = _4;
_9.fld0 = [101008227040625786107260940694649136772_u128,112125592698296030653835613532241061971_u128,120861588889401179078762791675633198771_u128,264889447080147250815304598890392901102_u128,219413115764108969773290941353564558713_u128,137936447894083911794047335844061966286_u128];
_9.fld5.0 = core::ptr::addr_of_mut!(_9.fld6);
_9.fld2 = [true,false,false,false];
match _11 {
0 => bb6,
1850736745 => bb8,
_ => bb7
}
}
bb16 = {
_19 = 69435038755430714430797723579537065739_u128;
_9.fld0 = [_19,_19,_19,_19,_19,_19];
_15 = 44363783784521645705255579573531109049_i128 - (-85951722547288496442152221717655868338_i128);
_9.fld5.0 = _17.0;
_15 = _9.fld1 as i128;
_11 = 683967392_i32;
_21.fld0 = !_15;
_9.fld3 = 2085457012396060529_usize;
_9.fld5.1 = _17.1 ^ _17.1;
_21.fld5.fld0 = [_19,_19,_19,_19,_19,_19];
_21.fld1.2 = _17.1 << _17.1;
_17.1 = _9.fld5.1 ^ _21.fld1.2;
_9.fld6 = _16 * _16;
match _19 {
0 => bb11,
69435038755430714430797723579537065739 => bb13,
_ => bb12
}
}
bb17 = {
_6 = [(-30349_i16)];
_9.fld1 = 148_u8 as u64;
_17 = (_9.fld5.0, _9.fld5.1);
_15 = -80612191254700115226779308090424719298_i128;
_9.fld3 = 391_i16 as usize;
_17.1 = _9.fld5.1 - _9.fld5.1;
_9.fld3 = !14649402637271931980_usize;
_9.fld0 = [317400562358200398838050260658865402209_u128,58651894978739043176926576565683253049_u128,284522705138294376788808566737868198894_u128,190769275737395228565979424448646084922_u128,154922948414579959059010955481250972730_u128,588350197580330619286121586661666784_u128];
_3 = 153_u8 as isize;
_13 = 127_i8 as f32;
_9.fld1 = (-11_i8) as u64;
_9.fld2 = [false,false,true,true];
_9.fld1 = 7702680799698180894_u64 | 7432211531701878406_u64;
_9.fld5.0 = core::ptr::addr_of_mut!(_16);
_9.fld1 = 2918365321081087348_u64 * 11035645173840886679_u64;
_16 = !_9.fld6;
_8 = [31396_i16];
_12 = _1;
_9.fld1 = !5593499058540587947_u64;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
_10 = [false,false,false,true];
match _11 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
1850736745 => bb9,
_ => bb6
}
}
bb19 = {
Return()
}
bb20 = {
_21.fld5.fld5.1 = 25_i8 as i64;
_20 = _15 as isize;
_23.1 = !_21.fld4;
_21.fld5.fld1 = !_9.fld1;
_24 = _12;
_29 = _3 as f64;
_10 = [false,false,false,false];
_9.fld3 = 15931298716834523458_usize;
_10 = [false,true,false,false];
_21.fld5.fld1 = _13 as u64;
_21.fld0 = !_15;
_9.fld5.1 = _17.1;
_19 = _15 as u128;
_2 = [55972_u16,35891_u16,47675_u16,39158_u16,9352_u16,62134_u16,38452_u16];
RET = core::ptr::addr_of_mut!(_28);
_11 = (-1235877735_i32);
_30.1 = _5;
Goto(bb21)
}
bb21 = {
Call(_33 = dump_var(13_usize, 6_usize, Move(_6), 1_usize, Move(_1), 20_usize, Move(_20), 24_usize, Move(_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(13_usize, 5_usize, Move(_5), 10_usize, Move(_10), 3_usize, Move(_3), 34_usize, _34), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u64,mut _2: [i8; 4],mut _3: isize,mut _4: char,mut _5: *mut [u64; 2],mut _6: [i8; 4],mut _7: isize,mut _8: *mut [u64; 2],mut _9: char,mut _10: [u128; 6],mut _11: *mut [u64; 2],mut _12: *mut [u64; 2],mut _13: char,mut _14: bool,mut _15: char,mut _16: i64) -> i128 {
mir! {
type RET = i128;
let _17: [u128; 6];
let _18: u8;
let _19: char;
let _20: isize;
let _21: (*mut u32, i64);
let _22: &'static [isize; 1];
let _23: *const u8;
let _24: u8;
let _25: bool;
let _26: u8;
let _27: Adt48;
let _28: (*mut u32, i64);
let _29: [bool; 4];
let _30: i64;
let _31: Adt55;
let _32: [bool; 4];
let _33: i8;
let _34: f32;
let _35: [isize; 1];
let _36: [u16; 7];
let _37: Adt44;
let _38: bool;
let _39: isize;
let _40: [isize; 8];
let _41: char;
let _42: bool;
let _43: char;
let _44: [u16; 7];
let _45: u64;
let _46: char;
let _47: char;
let _48: ();
let _49: ();
{
RET = (-153597417284019085328145391344607762450_i128);
_6 = [41_i8,126_i8,91_i8,29_i8];
RET = 154_u8 as i128;
_17 = _10;
_5 = _12;
_1 = 6_usize as u64;
_6 = [84_i8,20_i8,(-95_i8),(-80_i8)];
_17 = [317403996866102395233023150539714582586_u128,163750730721860620120852465530514889544_u128,98309243804144101000254519528957945388_u128,22014113922863984276084742938988277654_u128,63148329798182851845320725334199239401_u128,80395638749315671266424988652551994629_u128];
_15 = _4;
RET = (-2714311907841462772573767688872441650_i128);
_17 = _10;
_18 = 80_u8 & 157_u8;
_8 = _5;
_8 = _11;
Goto(bb1)
}
bb1 = {
_14 = !true;
RET = (-45976133199943325385085425287131029174_i128) - (-60154546324349617100779189221822724552_i128);
_13 = _4;
_13 = _9;
_17 = _10;
_7 = 27504_i16 as isize;
Goto(bb2)
}
bb2 = {
_3 = _7;
_7 = _3;
_16 = 4826036410790150862_i64 << _3;
_10 = _17;
_6 = [(-63_i8),13_i8,26_i8,113_i8];
_15 = _9;
_11 = _5;
_7 = !_3;
RET = _14 as i128;
_5 = _8;
_17 = [2730151196570587548244507758102511196_u128,48436136820044323303851621960761134795_u128,8494965793512595388849391005232122640_u128,121036320288319671007051790947415150667_u128,229965934307929220970724786022802507240_u128,221584200706430755047188691181020271426_u128];
_17 = _10;
_10 = [324420512353478085432736999899963459138_u128,17254492824277536983713372701440581099_u128,30430440719809284845474700876074611758_u128,25549821159259520086122039450789395955_u128,87245109207671310730285560333000321394_u128,253717568176177391990264148492738192516_u128];
_21.1 = _16 * _16;
_21.1 = _16;
_20 = _9 as isize;
_12 = _11;
_12 = _8;
_3 = (-1110189221_i32) as isize;
_4 = _9;
_14 = false;
_11 = _8;
_5 = _12;
RET = 90872328977716775659776778577535603785_i128;
_23 = core::ptr::addr_of!(_24);
Goto(bb3)
}
bb3 = {
_9 = _15;
(*_23) = _13 as u8;
RET = -111309560340516623073781474752409629582_i128;
_7 = _20;
_19 = _13;
_11 = _5;
_5 = _12;
RET = !48329976239281327900192889608281106565_i128;
_25 = _14 > _14;
_3 = _20 * _7;
_21.1 = _3 as i64;
_10 = [125180603415816249069721537761289232184_u128,3792483241441131040445847929643317556_u128,236811720314215326651824837147040977348_u128,267293679598066956958480475015476515091_u128,236433781512392928872412426884272115368_u128,54318783676925695411830883967631871236_u128];
_1 = 3776244562688995716_u64 * 9646659973681253668_u64;
_9 = _19;
_8 = _5;
_20 = 13805401602982702159_usize as isize;
_9 = _19;
(*_23) = _1 as u8;
_6 = [102_i8,82_i8,113_i8,37_i8];
_18 = (*_23) - (*_23);
Goto(bb4)
}
bb4 = {
_11 = _12;
Goto(bb5)
}
bb5 = {
RET = !(-122042474613705150851255601507830227051_i128);
_25 = _14 & _14;
_28.1 = _16 * _16;
_8 = _12;
_5 = _8;
_4 = _13;
_29 = [_14,_25,_25,_25];
_9 = _19;
_25 = !_14;
_9 = _19;
_24 = _18 | _18;
_24 = !_18;
_9 = _19;
_5 = _12;
_1 = 13226195360175696311_usize as u64;
_9 = _15;
_19 = _15;
_26 = _18 | (*_23);
_9 = _19;
_28.1 = _26 as i64;
_30 = -_21.1;
_3 = _20 << _24;
Goto(bb6)
}
bb6 = {
_13 = _15;
_32 = [_25,_14,_14,_25];
_5 = _12;
(*_23) = _26 | _26;
_30 = _28.1;
_30 = _28.1;
_3 = _7 << _28.1;
_3 = -_7;
_5 = _8;
_7 = _3 & _20;
_18 = (-185606518_i32) as u8;
_20 = _13 as isize;
_29 = [_25,_25,_25,_14];
Goto(bb7)
}
bb7 = {
_10 = [26583314143259230833556791983598422242_u128,233427433368603031877147075493246059_u128,291773404333322256523965448985829206424_u128,158867562393091987312855636790657929426_u128,294974374429628450540308599562051680898_u128,75587471598414665523836994191754370633_u128];
(*_23) = _26 ^ _18;
_3 = _20 & _20;
_37.fld3 = core::ptr::addr_of_mut!(_37.fld1.fld6);
_38 = _14 & _25;
_37.fld0 = 247530143731719966241630117626747689037_u128;
_7 = 2781341099_u32 as isize;
Call(_33 = fn15(_17, _24, (*_23), _38, _23, _37.fld0, _10, _38, _24, (*_23), _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_32 = _29;
_37.fld5 = -16198050_i32;
_35 = [_3];
_37.fld0 = 46419780761171548025976850014534922659_u128;
_25 = _38;
_37.fld1.fld0 = _17;
_15 = _9;
_6 = [_33,_33,_33,_33];
_22 = &_35;
_28.1 = _30 & _30;
_11 = _5;
_24 = !_26;
(*_23) = !_26;
_36 = [34329_u16,64997_u16,57622_u16,5022_u16,6689_u16,57161_u16,48278_u16];
_21.0 = core::ptr::addr_of_mut!(_37.fld1.fld6);
_13 = _9;
_16 = _30;
_37.fld6 = [_14,_38,_25,_14];
_37.fld1.fld6 = !2476605414_u32;
_37.fld1.fld2 = [_25,_25,_38,_14];
_30 = _28.1;
_37.fld1.fld6 = 643999015_u32;
_33 = 58_i8;
_33 = !127_i8;
_37.fld1.fld4 = core::ptr::addr_of!((*_23));
_37.fld1.fld1 = !_1;
_29 = [_38,_25,_25,_38];
_32 = [_38,_14,_25,_38];
Goto(bb9)
}
bb9 = {
_37.fld1.fld5.0 = _37.fld3;
_37.fld1.fld1 = _1;
Call(_39 = core::intrinsics::bswap(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_36 = [19054_u16,48746_u16,53968_u16,50939_u16,57883_u16,47581_u16,23972_u16];
_4 = _13;
_8 = _5;
_35 = [_3];
_21.0 = core::ptr::addr_of_mut!(_37.fld1.fld6);
match _37.fld1.fld6 {
643999015 => bb12,
_ => bb11
}
}
bb11 = {
_11 = _12;
Goto(bb5)
}
bb12 = {
_25 = !_38;
_37.fld1.fld1 = !_1;
_22 = &_35;
_41 = _15;
_28.0 = _37.fld1.fld5.0;
_20 = _37.fld0 as isize;
_13 = _4;
_42 = _25;
_37.fld1.fld5.1 = _16 + _30;
_28.0 = _37.fld3;
_34 = _1 as f32;
_37.fld1.fld5.1 = _28.1 | _30;
_30 = -_37.fld1.fld5.1;
_37.fld2 = _25 as i128;
_25 = !_42;
_11 = _8;
_43 = _4;
_37.fld1.fld3 = 2975217734411172554_usize ^ 4_usize;
_34 = _37.fld5 as f32;
_29 = _37.fld1.fld2;
_37.fld1.fld5 = _21;
_21 = _28;
_46 = _4;
_8 = _5;
_37.fld1.fld5 = _21;
_22 = &(*_22);
_17 = _10;
match _37.fld0 {
0 => bb11,
1 => bb4,
2 => bb13,
46419780761171548025976850014534922659 => bb15,
_ => bb14
}
}
bb13 = {
_10 = [26583314143259230833556791983598422242_u128,233427433368603031877147075493246059_u128,291773404333322256523965448985829206424_u128,158867562393091987312855636790657929426_u128,294974374429628450540308599562051680898_u128,75587471598414665523836994191754370633_u128];
(*_23) = _26 ^ _18;
_3 = _20 & _20;
_37.fld3 = core::ptr::addr_of_mut!(_37.fld1.fld6);
_38 = _14 & _25;
_37.fld0 = 247530143731719966241630117626747689037_u128;
_7 = 2781341099_u32 as isize;
Call(_33 = fn15(_17, _24, (*_23), _38, _23, _37.fld0, _10, _38, _24, (*_23), _16), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_14 = !true;
RET = (-45976133199943325385085425287131029174_i128) - (-60154546324349617100779189221822724552_i128);
_13 = _4;
_13 = _9;
_17 = _10;
_7 = 27504_i16 as isize;
Goto(bb2)
}
bb15 = {
_20 = _3;
_37.fld1.fld5 = _28;
_37.fld1.fld4 = core::ptr::addr_of!((*_23));
_21.1 = _16 << _37.fld1.fld5.1;
_30 = -_37.fld1.fld5.1;
_36 = [48426_u16,32502_u16,40427_u16,49590_u16,37637_u16,35899_u16,6428_u16];
RET = _37.fld2;
_6 = _2;
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(14_usize, 19_usize, Move(_19), 4_usize, Move(_4), 7_usize, Move(_7), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(14_usize, 6_usize, Move(_6), 29_usize, Move(_29), 24_usize, Move(_24), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(14_usize, 26_usize, Move(_26), 9_usize, Move(_9), 41_usize, Move(_41), 43_usize, Move(_43)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(14_usize, 20_usize, Move(_20), 10_usize, Move(_10), 15_usize, Move(_15), 49_usize, _49), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u128; 6],mut _2: u8,mut _3: u8,mut _4: bool,mut _5: *const u8,mut _6: u128,mut _7: [u128; 6],mut _8: bool,mut _9: u8,mut _10: u8,mut _11: i64) -> i8 {
mir! {
type RET = i8;
let _12: [i16; 4];
let _13: [i16; 1];
let _14: i16;
let _15: i32;
let _16: Adt58;
let _17: (u128, isize, [u64; 2]);
let _18: u8;
let _19: f32;
let _20: i8;
let _21: [u64; 2];
let _22: [u128; 6];
let _23: [i16; 4];
let _24: i8;
let _25: isize;
let _26: f64;
let _27: f32;
let _28: bool;
let _29: [i16; 4];
let _30: u64;
let _31: *mut u8;
let _32: i32;
let _33: Adt43;
let _34: ();
let _35: ();
{
_6 = 37644352053444980491389257933242206616_u128 + 248541795111758374600261328240659758498_u128;
_2 = _3 | _3;
_8 = _4 & _4;
_5 = core::ptr::addr_of!(_3);
RET = 71_i8;
Call(_3 = fn16(_6, _9, _7, _9, _4, _2, _2, _5, _5, _2, _5, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_2;
(*_5) = _2 & _10;
_12 = [28752_i16,(-2943_i16),20776_i16,23410_i16];
RET = (-46010888219307219867481970310437347505_i128) as i8;
_12 = [14577_i16,5764_i16,30310_i16,(-20671_i16)];
_7 = [_6,_6,_6,_6,_6,_6];
_1 = [_6,_6,_6,_6,_6,_6];
(*_5) = _2 - _9;
(*_5) = _2 & _2;
_3 = _10 - _2;
(*_5) = 8152847799644780113711757577800144679_i128 as u8;
RET = _6 as i8;
(*_5) = !_2;
_5 = core::ptr::addr_of!(_3);
_7 = _1;
_8 = _9 < _9;
_2 = _10;
_3 = !_10;
_3 = (-22868_i16) as u8;
_5 = core::ptr::addr_of!(_3);
Call(_16.fld0.fld0 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _1;
_2 = !_9;
Goto(bb3)
}
bb3 = {
_16.fld0.fld3 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld2 = !4043534284071883773_usize;
_6 = !139554333455968882753097605687499247862_u128;
_16.fld5.fld0 = _7;
Goto(bb4)
}
bb4 = {
_17.2 = [12154721296028334151_u64,7971121739275490157_u64];
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld5.fld6);
_16.fld5.fld1 = 687659641824761328_u64 >> _2;
_16.fld0.fld1.fld3 = _16.fld2 >> (*_5);
_9 = _3 >> _16.fld5.fld1;
_14 = !25496_i16;
_16.fld5.fld4 = core::ptr::addr_of!(_3);
_17.2 = [_16.fld5.fld1,_16.fld5.fld1];
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_17.1 = 9223372036854775807_isize ^ 66_isize;
_16.fld5.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_15 = !(-1625009423_i32);
_16.fld0.fld1.fld1 = _16.fld5.fld1;
_16.fld0.fld1.fld2 = [_8,_4,_8,_8];
_16.fld5.fld5.1 = _11;
_16.fld0.fld1.fld2 = [_8,_8,_8,_8];
_16.fld2 = !_16.fld0.fld1.fld3;
RET = (-77_i8);
_17.2 = [_16.fld0.fld1.fld1,_16.fld0.fld1.fld1];
_15 = -(-859585248_i32);
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_16.fld4.1 = _16.fld5.fld5.1 ^ _11;
_16.fld5.fld4 = _5;
_13 = [_14];
Goto(bb5)
}
bb5 = {
_15 = (-2123364876_i32) - 229267561_i32;
_11 = _16.fld5.fld5.1 & _16.fld4.1;
RET = (-31_i8);
_17.0 = !_6;
_16.fld5 = Adt42 { fld0: _7,fld1: _16.fld0.fld1.fld1,fld2: _16.fld0.fld1.fld2,fld3: _16.fld2,fld4: _5,fld5: _16.fld4,fld6: 940333760_u32 };
_16.fld0.fld1.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld5.fld6 = !301520455_u32;
_16.fld0.fld1.fld3 = _16.fld5.fld3 ^ _16.fld5.fld3;
_16.fld0.fld1.fld2 = [_8,_8,_4,_8];
_16.fld0.fld6 = _16.fld5.fld2;
_2 = '\u{fd951}' as u8;
_16.fld0.fld1.fld4 = _5;
_16.fld4 = _16.fld5.fld5;
match RET {
0 => bb6,
1 => bb7,
340282366920938463463374607431768211425 => bb9,
_ => bb8
}
}
bb6 = {
_17.2 = [12154721296028334151_u64,7971121739275490157_u64];
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld5.fld6);
_16.fld5.fld1 = 687659641824761328_u64 >> _2;
_16.fld0.fld1.fld3 = _16.fld2 >> (*_5);
_9 = _3 >> _16.fld5.fld1;
_14 = !25496_i16;
_16.fld5.fld4 = core::ptr::addr_of!(_3);
_17.2 = [_16.fld5.fld1,_16.fld5.fld1];
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_17.1 = 9223372036854775807_isize ^ 66_isize;
_16.fld5.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_15 = !(-1625009423_i32);
_16.fld0.fld1.fld1 = _16.fld5.fld1;
_16.fld0.fld1.fld2 = [_8,_4,_8,_8];
_16.fld5.fld5.1 = _11;
_16.fld0.fld1.fld2 = [_8,_8,_8,_8];
_16.fld2 = !_16.fld0.fld1.fld3;
RET = (-77_i8);
_17.2 = [_16.fld0.fld1.fld1,_16.fld0.fld1.fld1];
_15 = -(-859585248_i32);
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_16.fld4.1 = _16.fld5.fld5.1 ^ _11;
_16.fld5.fld4 = _5;
_13 = [_14];
Goto(bb5)
}
bb7 = {
_16.fld0.fld3 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld2 = !4043534284071883773_usize;
_6 = !139554333455968882753097605687499247862_u128;
_16.fld5.fld0 = _7;
Goto(bb4)
}
bb8 = {
_7 = _1;
_2 = !_9;
Goto(bb3)
}
bb9 = {
_21 = [_16.fld5.fld1,_16.fld5.fld1];
_16.fld0.fld1.fld5 = _16.fld4;
_16.fld0.fld1.fld4 = _5;
_5 = core::ptr::addr_of!((*_5));
_6 = !_17.0;
_22 = [_6,_6,_17.0,_6,_6,_6];
_16.fld0.fld3 = core::ptr::addr_of_mut!(_16.fld5.fld6);
_16.fld5 = Adt42 { fld0: _1,fld1: _16.fld0.fld1.fld1,fld2: _16.fld0.fld1.fld2,fld3: _16.fld0.fld1.fld3,fld4: _5,fld5: _16.fld0.fld1.fld5,fld6: 3125713139_u32 };
_16.fld0.fld1 = Adt42 { fld0: _16.fld5.fld0,fld1: _16.fld5.fld1,fld2: _16.fld5.fld2,fld3: _16.fld5.fld3,fld4: _16.fld5.fld4,fld5: _16.fld5.fld5,fld6: _16.fld5.fld6 };
_16.fld1.0 = core::ptr::addr_of_mut!(_21);
_16.fld0.fld1.fld5.1 = _16.fld5.fld5.1;
_16.fld0.fld1.fld0 = [_17.0,_17.0,_6,_17.0,_17.0,_17.0];
Goto(bb10)
}
bb10 = {
_16.fld0.fld1 = Adt42 { fld0: _1,fld1: _16.fld5.fld1,fld2: _16.fld5.fld2,fld3: _16.fld2,fld4: _16.fld5.fld4,fld5: _16.fld5.fld5,fld6: _16.fld5.fld6 };
_19 = _17.1 as f32;
_6 = _17.0;
_10 = _9 - _9;
_17.2 = [_16.fld5.fld1,_16.fld0.fld1.fld1];
_8 = _4;
_16.fld5.fld1 = !_16.fld0.fld1.fld1;
_12 = [_14,_14,_14,_14];
_24 = -RET;
_16.fld0.fld1.fld6 = _11 as u32;
_16.fld4.0 = _16.fld0.fld3;
_16.fld5.fld6 = _16.fld5.fld3 as u32;
Goto(bb11)
}
bb11 = {
_28 = _4;
_16.fld0.fld1.fld1 = !_16.fld5.fld1;
_27 = _16.fld0.fld1.fld1 as f32;
_26 = 56616_u16 as f64;
_16.fld0.fld2 = !(-164512850859019901595440401749244455887_i128);
_16.fld0.fld5 = -_15;
_16.fld1.0 = core::ptr::addr_of_mut!(_21);
_29 = _12;
_1 = _16.fld5.fld0;
_13 = [_14];
_17.1 = 9223372036854775807_isize;
RET = _24 & _24;
_29 = [_14,_14,_14,_14];
_8 = _28;
_2 = _10 << _16.fld0.fld1.fld6;
_16.fld4 = (_16.fld5.fld5.0, _16.fld5.fld5.1);
_16.fld4.1 = _26 as i64;
_22 = [_17.0,_17.0,_6,_6,_6,_6];
_18 = _6 as u8;
match _17.1 {
0 => bb12,
1 => bb13,
2 => bb14,
9223372036854775807 => bb16,
_ => bb15
}
}
bb12 = {
_17.2 = [12154721296028334151_u64,7971121739275490157_u64];
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld5.fld6);
_16.fld5.fld1 = 687659641824761328_u64 >> _2;
_16.fld0.fld1.fld3 = _16.fld2 >> (*_5);
_9 = _3 >> _16.fld5.fld1;
_14 = !25496_i16;
_16.fld5.fld4 = core::ptr::addr_of!(_3);
_17.2 = [_16.fld5.fld1,_16.fld5.fld1];
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_17.1 = 9223372036854775807_isize ^ 66_isize;
_16.fld5.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_15 = !(-1625009423_i32);
_16.fld0.fld1.fld1 = _16.fld5.fld1;
_16.fld0.fld1.fld2 = [_8,_4,_8,_8];
_16.fld5.fld5.1 = _11;
_16.fld0.fld1.fld2 = [_8,_8,_8,_8];
_16.fld2 = !_16.fld0.fld1.fld3;
RET = (-77_i8);
_17.2 = [_16.fld0.fld1.fld1,_16.fld0.fld1.fld1];
_15 = -(-859585248_i32);
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_16.fld4.1 = _16.fld5.fld5.1 ^ _11;
_16.fld5.fld4 = _5;
_13 = [_14];
Goto(bb5)
}
bb13 = {
_15 = (-2123364876_i32) - 229267561_i32;
_11 = _16.fld5.fld5.1 & _16.fld4.1;
RET = (-31_i8);
_17.0 = !_6;
_16.fld5 = Adt42 { fld0: _7,fld1: _16.fld0.fld1.fld1,fld2: _16.fld0.fld1.fld2,fld3: _16.fld2,fld4: _5,fld5: _16.fld4,fld6: 940333760_u32 };
_16.fld0.fld1.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld5.fld6 = !301520455_u32;
_16.fld0.fld1.fld3 = _16.fld5.fld3 ^ _16.fld5.fld3;
_16.fld0.fld1.fld2 = [_8,_8,_4,_8];
_16.fld0.fld6 = _16.fld5.fld2;
_2 = '\u{fd951}' as u8;
_16.fld0.fld1.fld4 = _5;
_16.fld4 = _16.fld5.fld5;
match RET {
0 => bb6,
1 => bb7,
340282366920938463463374607431768211425 => bb9,
_ => bb8
}
}
bb14 = {
_17.2 = [12154721296028334151_u64,7971121739275490157_u64];
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld5.fld6);
_16.fld5.fld1 = 687659641824761328_u64 >> _2;
_16.fld0.fld1.fld3 = _16.fld2 >> (*_5);
_9 = _3 >> _16.fld5.fld1;
_14 = !25496_i16;
_16.fld5.fld4 = core::ptr::addr_of!(_3);
_17.2 = [_16.fld5.fld1,_16.fld5.fld1];
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_17.1 = 9223372036854775807_isize ^ 66_isize;
_16.fld5.fld5.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_15 = !(-1625009423_i32);
_16.fld0.fld1.fld1 = _16.fld5.fld1;
_16.fld0.fld1.fld2 = [_8,_4,_8,_8];
_16.fld5.fld5.1 = _11;
_16.fld0.fld1.fld2 = [_8,_8,_8,_8];
_16.fld2 = !_16.fld0.fld1.fld3;
RET = (-77_i8);
_17.2 = [_16.fld0.fld1.fld1,_16.fld0.fld1.fld1];
_15 = -(-859585248_i32);
_16.fld4.0 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld0.fld1.fld4 = core::ptr::addr_of!((*_5));
_16.fld4.1 = _16.fld5.fld5.1 ^ _11;
_16.fld5.fld4 = _5;
_13 = [_14];
Goto(bb5)
}
bb15 = {
_16.fld0.fld3 = core::ptr::addr_of_mut!(_16.fld0.fld1.fld6);
_16.fld2 = !4043534284071883773_usize;
_6 = !139554333455968882753097605687499247862_u128;
_16.fld5.fld0 = _7;
Goto(bb4)
}
bb16 = {
_18 = _2 & _9;
_13 = [_14];
_20 = !RET;
_16.fld0.fld4 = _16.fld5.fld1 + _16.fld0.fld1.fld1;
_16.fld5 = Adt42 { fld0: _7,fld1: _16.fld0.fld1.fld1,fld2: _16.fld0.fld1.fld2,fld3: _16.fld0.fld1.fld3,fld4: _16.fld0.fld1.fld4,fld5: _16.fld0.fld1.fld5,fld6: _16.fld0.fld1.fld6 };
_25 = -_17.1;
_32 = -_16.fld0.fld5;
_27 = _19;
_16.fld4.1 = _16.fld0.fld1.fld5.1 * _11;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(15_usize, 8_usize, Move(_8), 7_usize, Move(_7), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(15_usize, 4_usize, Move(_4), 25_usize, Move(_25), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(15_usize, 29_usize, Move(_29), 32_usize, Move(_32), 20_usize, Move(_20), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u128,mut _2: u8,mut _3: [u128; 6],mut _4: u8,mut _5: bool,mut _6: u8,mut _7: u8,mut _8: *const u8,mut _9: *const u8,mut _10: u8,mut _11: *const u8,mut _12: u8) -> u8 {
mir! {
type RET = u8;
let _13: i128;
let _14: u64;
let _15: isize;
let _16: [i8; 4];
let _17: ();
let _18: ();
{
RET = _2;
_7 = !_6;
RET = _2 | _6;
RET = _12;
_12 = 1800213856_u32 as u8;
_4 = _7 - _6;
_1 = 120586909994313467738674307205824131907_u128;
RET = _4 ^ _10;
_9 = _8;
_11 = _8;
_2 = _6;
_13 = 16_i8 as i128;
_4 = RET ^ RET;
_5 = true;
RET = _4 | _4;
_1 = !331140114369428459011883136442520053001_u128;
_4 = RET;
_2 = _6;
_13 = _1 as i128;
_9 = _11;
RET = !_4;
_16 = [60_i8,53_i8,111_i8,(-106_i8)];
_8 = core::ptr::addr_of!(_10);
_10 = !_6;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(16_usize, 16_usize, Move(_16), 7_usize, Move(_7), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(16_usize, 2_usize, Move(_2), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [i8; 4],mut _2: i64,mut _3: [i8; 4],mut _4: *mut u32,mut _5: *mut [u64; 2],mut _6: *mut u32,mut _7: u64,mut _8: *mut [u64; 2],mut _9: u64,mut _10: *mut u32,mut _11: *mut [u64; 2],mut _12: *mut u32,mut _13: [bool; 4],mut _14: [u128; 6],mut _15: [i8; 4],mut _16: [i8; 4]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _17: f32;
let _18: f32;
let _19: [u16; 7];
let _20: bool;
let _21: *mut u32;
let _22: (*mut f32, isize, i64);
let _23: f64;
let _24: Adt50;
let _25: [isize; 1];
let _26: [i8; 4];
let _27: [isize; 1];
let _28: isize;
let _29: u32;
let _30: u64;
let _31: i16;
let _32: (*mut [u64; 2],);
let _33: [i16; 4];
let _34: f32;
let _35: [bool; 4];
let _36: [u16; 7];
let _37: char;
let _38: u16;
let _39: [i16; 2];
let _40: isize;
let _41: u8;
let _42: f64;
let _43: Adt58;
let _44: [u16; 7];
let _45: Adt46;
let _46: [i16; 4];
let _47: Adt47;
let _48: u16;
let _49: bool;
let _50: Adt52;
let _51: u64;
let _52: Adt53;
let _53: (*mut u8,);
let _54: ();
let _55: ();
{
_13 = [false,false,false,true];
_8 = _11;
_2 = (-5745548848794877904_i64);
_7 = !_9;
(*_4) = 3312581805_u32;
(*_10) = 14933684184249183828_usize as u32;
_16 = _3;
_18 = (*_12) as f32;
_17 = -_18;
_1 = _16;
(*_4) = 4243658045_u32;
_1 = _3;
RET = core::ptr::addr_of_mut!(_18);
(*_4) = 4059379505_u32;
_20 = !false;
_5 = _11;
_4 = _10;
(*RET) = _17 - _17;
_16 = [56_i8,45_i8,33_i8,49_i8];
_19 = [34349_u16,27542_u16,40130_u16,39607_u16,7103_u16,23094_u16,13072_u16];
_20 = !true;
_21 = _10;
_1 = [(-4_i8),27_i8,(-48_i8),125_i8];
Goto(bb1)
}
bb1 = {
(*_6) = 1603337621_u32 >> _7;
(*_21) = !1760795723_u32;
_15 = [99_i8,114_i8,(-27_i8),(-86_i8)];
(*RET) = _17 - _17;
_11 = _8;
(*_12) = 28_i8 as u32;
_15 = [97_i8,(-82_i8),(-58_i8),(-56_i8)];
_22.1 = (-13667_i16) as isize;
_9 = _7 ^ _7;
(*_6) = !3469264969_u32;
_2 = (-1728737139512595193_i64);
(*RET) = _17 - _17;
_18 = _17;
_22.2 = (*RET) as i64;
Goto(bb2)
}
bb2 = {
(*RET) = _17 * _17;
_21 = core::ptr::addr_of_mut!((*_6));
_19 = [58927_u16,2911_u16,63548_u16,7058_u16,36074_u16,43592_u16,40561_u16];
_15 = [127_i8,113_i8,59_i8,114_i8];
_16 = _3;
_22.0 = RET;
(*_21) = 4256362141_u32;
_1 = _3;
_22.1 = 9223372036854775807_isize;
(*RET) = _17;
(*RET) = -_17;
_4 = core::ptr::addr_of_mut!((*_4));
_23 = 41_u8 as f64;
_15 = _1;
(*_21) = 2952512599_u32 - 2714605683_u32;
match _2 {
0 => bb1,
1 => bb3,
340282366920938463461645870292255616263 => bb5,
_ => bb4
}
}
bb3 = {
(*_6) = 1603337621_u32 >> _7;
(*_21) = !1760795723_u32;
_15 = [99_i8,114_i8,(-27_i8),(-86_i8)];
(*RET) = _17 - _17;
_11 = _8;
(*_12) = 28_i8 as u32;
_15 = [97_i8,(-82_i8),(-58_i8),(-56_i8)];
_22.1 = (-13667_i16) as isize;
_9 = _7 ^ _7;
(*_6) = !3469264969_u32;
_2 = (-1728737139512595193_i64);
(*RET) = _17 - _17;
_18 = _17;
_22.2 = (*RET) as i64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*_6) = !1461585008_u32;
(*_12) = _22.2 as u32;
(*RET) = _17;
(*_10) = 3707382534_u32 & 3740114434_u32;
(*RET) = -_17;
_8 = _5;
RET = core::ptr::addr_of_mut!(_17);
_16 = [93_i8,(-117_i8),29_i8,61_i8];
match _22.1 {
0 => bb1,
9223372036854775807 => bb6,
_ => bb2
}
}
bb6 = {
RET = core::ptr::addr_of_mut!((*RET));
_19 = [54874_u16,19718_u16,46653_u16,26449_u16,45479_u16,20002_u16,58359_u16];
_11 = _8;
(*_12) = 1791113831_u32 | 221773452_u32;
(*_6) = _18 as u32;
_23 = 5293_i16 as f64;
_27 = [_22.1];
_22.1 = -9223372036854775807_isize;
Goto(bb7)
}
bb7 = {
_18 = (*RET) + (*RET);
_25 = _27;
(*_4) = 96_u8 as u32;
(*_21) = !3760195235_u32;
_22 = (RET, 9223372036854775807_isize, _2);
_22.0 = core::ptr::addr_of_mut!(_18);
(*_6) = 3132678083_u32 * 2785857276_u32;
_4 = core::ptr::addr_of_mut!((*_21));
_8 = _5;
(*_12) = !627024706_u32;
_4 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3065956328_u32 >> _9;
_28 = 62670_u16 as isize;
_18 = (-17344_i16) as f32;
_13 = [_20,_20,_20,_20];
_31 = -(-30213_i16);
(*_10) = 2323231609_u32 | 2916674776_u32;
RET = _22.0;
(*_12) = 2525717116_u32 & 1200512164_u32;
_29 = (*_10) ^ (*_6);
(*RET) = _17;
_12 = core::ptr::addr_of_mut!(_29);
_12 = core::ptr::addr_of_mut!((*_10));
(*RET) = 208_u8 as f32;
Goto(bb8)
}
bb8 = {
_21 = core::ptr::addr_of_mut!((*_10));
_14 = [11809216182135485709839328140768018777_u128,235105238895926640144699160847488693493_u128,323992240377780856240027265954595382830_u128,83012537204025063000846772949992660485_u128,175982292121017218398355556014960032699_u128,20428160604332403529556662829759440606_u128];
_9 = _7 - _7;
_31 = -31824_i16;
_29 = !(*_21);
(*_4) = _29;
_2 = _23 as i64;
(*_6) = 62680992607389033842858408211859066334_u128 as u32;
_16 = [55_i8,62_i8,(-125_i8),(-114_i8)];
RET = core::ptr::addr_of_mut!(_18);
(*RET) = _17 * _17;
_32.0 = _11;
_18 = _2 as f32;
_22.2 = _2 ^ _2;
RET = core::ptr::addr_of_mut!(_34);
_12 = core::ptr::addr_of_mut!((*_6));
(*_4) = !_29;
(*RET) = _18;
(*RET) = _18;
_21 = core::ptr::addr_of_mut!((*_4));
_38 = !31942_u16;
_22.2 = _2 << _9;
Call(_30 = fn18(_32.0, _6, _4, _22.1, _19, _21, _6, _11, _21, _22, _9, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = [91_i8,(-92_i8),(-70_i8),(-97_i8)];
_7 = !_30;
(*RET) = _7 as f32;
_37 = '\u{34cf6}';
_1 = [(-26_i8),22_i8,97_i8,(-108_i8)];
_14 = [337333387234292188648413078035006196218_u128,217618330107916556108799733537788172025_u128,109555845254744423824017608778020833741_u128,64596310433338536021712704047990074477_u128,95826311296377010820101884090690014149_u128,236823742491842339048677011475140270879_u128];
_9 = _7;
_15 = _1;
(*_10) = _29;
_13 = [_20,_20,_20,_20];
_22.0 = core::ptr::addr_of_mut!(_18);
_5 = _11;
_27 = [_22.1];
_36 = _19;
(*_10) = !_29;
(*RET) = _18 + _17;
_34 = _2 as f32;
_4 = _21;
(*_12) = _29 ^ _29;
_13 = [_20,_20,_20,_20];
(*_21) = _29 | _29;
_8 = _11;
_23 = _29 as f64;
RET = core::ptr::addr_of_mut!(_17);
_43.fld4.0 = _6;
_22.2 = _2 ^ _2;
match _22.1 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb5,
5 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb10 = {
_21 = core::ptr::addr_of_mut!((*_10));
_14 = [11809216182135485709839328140768018777_u128,235105238895926640144699160847488693493_u128,323992240377780856240027265954595382830_u128,83012537204025063000846772949992660485_u128,175982292121017218398355556014960032699_u128,20428160604332403529556662829759440606_u128];
_9 = _7 - _7;
_31 = -31824_i16;
_29 = !(*_21);
(*_4) = _29;
_2 = _23 as i64;
(*_6) = 62680992607389033842858408211859066334_u128 as u32;
_16 = [55_i8,62_i8,(-125_i8),(-114_i8)];
RET = core::ptr::addr_of_mut!(_18);
(*RET) = _17 * _17;
_32.0 = _11;
_18 = _2 as f32;
_22.2 = _2 ^ _2;
RET = core::ptr::addr_of_mut!(_34);
_12 = core::ptr::addr_of_mut!((*_6));
(*_4) = !_29;
(*RET) = _18;
(*RET) = _18;
_21 = core::ptr::addr_of_mut!((*_4));
_38 = !31942_u16;
_22.2 = _2 << _9;
Call(_30 = fn18(_32.0, _6, _4, _22.1, _19, _21, _6, _11, _21, _22, _9, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
(*_6) = !1461585008_u32;
(*_12) = _22.2 as u32;
(*RET) = _17;
(*_10) = 3707382534_u32 & 3740114434_u32;
(*RET) = -_17;
_8 = _5;
RET = core::ptr::addr_of_mut!(_17);
_16 = [93_i8,(-117_i8),29_i8,61_i8];
match _22.1 {
0 => bb1,
9223372036854775807 => bb6,
_ => bb2
}
}
bb12 = {
_43.fld0.fld1.fld6 = _37 as u32;
_43.fld5.fld4 = core::ptr::addr_of!(_41);
_43.fld0.fld5 = _20 as i32;
_15 = [(-66_i8),23_i8,103_i8,49_i8];
_40 = !_22.1;
_12 = core::ptr::addr_of_mut!((*_4));
_15 = [(-55_i8),83_i8,(-11_i8),87_i8];
(*_21) = _40 as u32;
_43.fld3 = (-58_i8) << _22.1;
_30 = _7 + _9;
_16 = _3;
(*RET) = -_18;
_22 = (RET, _40, _2);
_43.fld0.fld1.fld4 = core::ptr::addr_of!(_41);
(*RET) = _18;
_43.fld0.fld1.fld3 = 6483257932631259147_usize >> (*_6);
_21 = core::ptr::addr_of_mut!((*_4));
_6 = core::ptr::addr_of_mut!(_29);
(*_12) = (*_6);
RET = _22.0;
Call(_43.fld0.fld4 = core::intrinsics::transmute(_25), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_35 = [_20,_20,_20,_20];
(*_6) = (*_12) - (*_12);
_3 = [_43.fld3,_43.fld3,_43.fld3,_43.fld3];
_38 = 45150_u16 << _43.fld0.fld1.fld3;
_22.1 = _40 | _28;
_43.fld2 = _43.fld0.fld1.fld3 + _43.fld0.fld1.fld3;
_3 = [_43.fld3,_43.fld3,_43.fld3,_43.fld3];
_11 = _5;
_43.fld0.fld1.fld2 = _13;
_3 = [_43.fld3,_43.fld3,_43.fld3,_43.fld3];
_43.fld0.fld1.fld5.0 = _21;
_43.fld1.0 = _8;
_7 = !_30;
_12 = core::ptr::addr_of_mut!(_43.fld0.fld1.fld6);
_30 = _7 | _7;
_48 = _38 & _38;
_32 = (_8,);
_43.fld5.fld6 = (*_10) << _43.fld2;
_43.fld2 = _43.fld0.fld1.fld3;
_5 = _8;
_43.fld0.fld5 = -(-1207012749_i32);
_43.fld0.fld1.fld0 = [324202652825044743371995021378105786415_u128,321080967874129209366423412076742629200_u128,278172083144596359885171439671747317047_u128,169609874001238052448334336947986070914_u128,138031459556270874258123917266833664910_u128,318798324487054220016737587521453012578_u128];
Goto(bb14)
}
bb14 = {
(*_21) = (*_6) + _29;
_36 = [_48,_48,_38,_48,_38,_48,_48];
_37 = '\u{eb44f}';
_43.fld5.fld0 = [127624040893793424828994253327439260839_u128,288456727021173536770459772355988633027_u128,301892217974213814693946882338962447437_u128,320598451470721108062895573848888750578_u128,190600167415052796763805189178613354411_u128,112154368048768305142584260863150670260_u128];
(*_10) = _2 as u32;
_10 = _4;
_43.fld5.fld3 = _43.fld0.fld1.fld3;
_16 = _3;
(*RET) = 159_u8 as f32;
RET = core::ptr::addr_of_mut!(_17);
_18 = _31 as f32;
_6 = core::ptr::addr_of_mut!(_43.fld0.fld1.fld6);
_38 = !_48;
_43.fld4 = (_10, _2);
_43.fld0.fld0 = !147881054909534314060517929835285675561_u128;
_43.fld3 = !(-14_i8);
_43.fld1 = _32;
_3 = _15;
_43.fld1.0 = _5;
(*_6) = !_29;
_46 = [_31,_31,_31,_31];
_50.fld3 = core::ptr::addr_of_mut!(_41);
_43.fld0.fld1.fld0 = [_43.fld0.fld0,_43.fld0.fld0,_43.fld0.fld0,_43.fld0.fld0,_43.fld0.fld0,_43.fld0.fld0];
_44 = [_38,_38,_48,_48,_48,_48,_48];
_43.fld5.fld1 = !_7;
_32.0 = _43.fld1.0;
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(17_usize, 29_usize, Move(_29), 19_usize, Move(_19), 28_usize, Move(_28), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(17_usize, 25_usize, Move(_25), 38_usize, Move(_38), 7_usize, Move(_7), 35_usize, Move(_35)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(17_usize, 30_usize, Move(_30), 15_usize, Move(_15), 20_usize, Move(_20), 40_usize, Move(_40)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *mut [u64; 2],mut _2: *mut u32,mut _3: *mut u32,mut _4: isize,mut _5: [u16; 7],mut _6: *mut u32,mut _7: *mut u32,mut _8: *mut [u64; 2],mut _9: *mut u32,mut _10: (*mut f32, isize, i64),mut _11: u64,mut _12: *mut u32) -> u64 {
mir! {
type RET = u64;
let _13: *mut u8;
let _14: f32;
let _15: (char, i16, char);
let _16: isize;
let _17: [bool; 4];
let _18: [isize; 1];
let _19: u8;
let _20: f64;
let _21: char;
let _22: i64;
let _23: Adt57;
let _24: isize;
let _25: f64;
let _26: f32;
let _27: u8;
let _28: char;
let _29: ();
let _30: ();
{
_9 = _6;
(*_2) = 1600141950_u32 * 595370249_u32;
(*_12) = !2344079002_u32;
(*_6) = 1637905430_u32 | 2692446216_u32;
_10.2 = (-1276288148984770740_i64) << (*_7);
RET = _11 >> (*_9);
RET = _11;
_1 = _8;
_10.1 = '\u{99fef}' as isize;
_16 = _4;
_4 = '\u{939ea}' as isize;
_4 = '\u{5372d}' as isize;
_15.0 = '\u{101bcb}';
(*_6) = (-1866946183_i32) as u32;
(*_2) = RET as u32;
_15.2 = _15.0;
RET = _11 >> (*_12);
(*_6) = 4208201499_u32 | 3357290329_u32;
_17 = [false,true,true,true];
_9 = _7;
_14 = (*_7) as f32;
_12 = core::ptr::addr_of_mut!((*_7));
RET = 4298_i16 as u64;
Goto(bb1)
}
bb1 = {
(*_7) = _10.2 as u32;
_15.2 = _15.0;
_15.2 = _15.0;
_8 = _1;
_11 = RET | RET;
_15 = ('\u{6c2b9}', (-16201_i16), '\u{10ce4}');
_10.1 = !_16;
_11 = RET << _10.1;
RET = _11;
_3 = _7;
_11 = RET;
_22 = true as i64;
_12 = _9;
(*_12) = 4080824022_u32 & 2259981062_u32;
_19 = 151_u8;
_20 = (*_12) as f64;
_5 = [54493_u16,18706_u16,61721_u16,61595_u16,1474_u16,22009_u16,31391_u16];
(*_9) = 1677997918_u32;
(*_9) = _20 as u32;
match _16 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_13 = core::ptr::addr_of_mut!(_19);
_16 = -_4;
_2 = _7;
_3 = core::ptr::addr_of_mut!((*_3));
(*_6) = 4066295626_u32 + 2227307735_u32;
_10.1 = _4 | _4;
_10.2 = _22 - _22;
_11 = _20 as u64;
_1 = _8;
_22 = _20 as i64;
_24 = _4 >> (*_6);
_13 = core::ptr::addr_of_mut!((*_13));
(*_12) = 817206551_u32;
_17 = [true,true,false,false];
(*_9) = 2434567328_u32;
_21 = _15.2;
_8 = _1;
_3 = _9;
_7 = core::ptr::addr_of_mut!((*_6));
_1 = _8;
_15.0 = _15.2;
(*_12) = 1058735042_u32;
(*_3) = RET as u32;
_26 = -_14;
_5 = [39335_u16,9728_u16,9748_u16,39037_u16,36824_u16,10860_u16,49068_u16];
(*_2) = 1331105461_u32 & 246146462_u32;
_20 = _14 as f64;
(*_7) = 2248860573_u32;
Goto(bb4)
}
bb4 = {
_1 = _8;
_24 = _11 as isize;
_22 = _10.2 | _10.2;
match _15.1 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768195255 => bb10,
_ => bb9
}
}
bb5 = {
_13 = core::ptr::addr_of_mut!(_19);
_16 = -_4;
_2 = _7;
_3 = core::ptr::addr_of_mut!((*_3));
(*_6) = 4066295626_u32 + 2227307735_u32;
_10.1 = _4 | _4;
_10.2 = _22 - _22;
_11 = _20 as u64;
_1 = _8;
_22 = _20 as i64;
_24 = _4 >> (*_6);
_13 = core::ptr::addr_of_mut!((*_13));
(*_12) = 817206551_u32;
_17 = [true,true,false,false];
(*_9) = 2434567328_u32;
_21 = _15.2;
_8 = _1;
_3 = _9;
_7 = core::ptr::addr_of_mut!((*_6));
_1 = _8;
_15.0 = _15.2;
(*_12) = 1058735042_u32;
(*_3) = RET as u32;
_26 = -_14;
_5 = [39335_u16,9728_u16,9748_u16,39037_u16,36824_u16,10860_u16,49068_u16];
(*_2) = 1331105461_u32 & 246146462_u32;
_20 = _14 as f64;
(*_7) = 2248860573_u32;
Goto(bb4)
}
bb6 = {
Return()
}
bb7 = {
(*_7) = _10.2 as u32;
_15.2 = _15.0;
_15.2 = _15.0;
_8 = _1;
_11 = RET | RET;
_15 = ('\u{6c2b9}', (-16201_i16), '\u{10ce4}');
_10.1 = !_16;
_11 = RET << _10.1;
RET = _11;
_3 = _7;
_11 = RET;
_22 = true as i64;
_12 = _9;
(*_12) = 4080824022_u32 & 2259981062_u32;
_19 = 151_u8;
_20 = (*_12) as f64;
_5 = [54493_u16,18706_u16,61721_u16,61595_u16,1474_u16,22009_u16,31391_u16];
(*_9) = 1677997918_u32;
(*_9) = _20 as u32;
match _16 {
9223372036854775807 => bb3,
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
(*_2) = !190607595_u32;
(*_6) = 1927546800_u32 >> _11;
match _15.1 {
0 => bb8,
1 => bb4,
2 => bb11,
3 => bb12,
340282366920938463463374607431768195255 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
(*_7) = _10.2 as u32;
_15.2 = _15.0;
_15.2 = _15.0;
_8 = _1;
_11 = RET | RET;
_15 = ('\u{6c2b9}', (-16201_i16), '\u{10ce4}');
_10.1 = !_16;
_11 = RET << _10.1;
RET = _11;
_3 = _7;
_11 = RET;
_22 = true as i64;
_12 = _9;
(*_12) = 4080824022_u32 & 2259981062_u32;
_19 = 151_u8;
_20 = (*_12) as f64;
_5 = [54493_u16,18706_u16,61721_u16,61595_u16,1474_u16,22009_u16,31391_u16];
(*_9) = 1677997918_u32;
(*_9) = _20 as u32;
match _16 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb13 = {
_13 = core::ptr::addr_of_mut!(_19);
_16 = -_4;
_2 = _7;
_3 = core::ptr::addr_of_mut!((*_3));
(*_6) = 4066295626_u32 + 2227307735_u32;
_10.1 = _4 | _4;
_10.2 = _22 - _22;
_11 = _20 as u64;
_1 = _8;
_22 = _20 as i64;
_24 = _4 >> (*_6);
_13 = core::ptr::addr_of_mut!((*_13));
(*_12) = 817206551_u32;
_17 = [true,true,false,false];
(*_9) = 2434567328_u32;
_21 = _15.2;
_8 = _1;
_3 = _9;
_7 = core::ptr::addr_of_mut!((*_6));
_1 = _8;
_15.0 = _15.2;
(*_12) = 1058735042_u32;
(*_3) = RET as u32;
_26 = -_14;
_5 = [39335_u16,9728_u16,9748_u16,39037_u16,36824_u16,10860_u16,49068_u16];
(*_2) = 1331105461_u32 & 246146462_u32;
_20 = _14 as f64;
(*_7) = 2248860573_u32;
Goto(bb4)
}
bb14 = {
_15 = (_21, 25001_i16, _21);
_24 = _10.1 >> _22;
_15.2 = _21;
_12 = core::ptr::addr_of_mut!((*_2));
_15.2 = _15.0;
_2 = _3;
_27 = (*_13);
_4 = -_24;
_10.2 = _22;
(*_3) = !3283605166_u32;
_24 = !_4;
_15 = (_21, (-18576_i16), _21);
_13 = core::ptr::addr_of_mut!((*_13));
_28 = _15.0;
(*_12) = 3249394739_u32;
_20 = _24 as f64;
RET = _11;
_11 = RET;
_1 = _8;
(*_9) = 2517056989_u32 | 2665058132_u32;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(18_usize, 19_usize, Move(_19), 17_usize, Move(_17), 27_usize, Move(_27), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(18_usize, 4_usize, Move(_4), 5_usize, Move(_5), 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: f64,mut _2: [u16; 7]) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _3: i128;
let _4: (u128, isize, [u64; 2]);
let _5: char;
let _6: [u16; 7];
let _7: isize;
let _8: f32;
let _9: f32;
let _10: (*mut f32, isize, i64);
let _11: isize;
let _12: Adt47;
let _13: i128;
let _14: [u64; 2];
let _15: (u128, isize, [u64; 2]);
let _16: (*mut f32, isize, i64);
let _17: *mut u8;
let _18: bool;
let _19: [i16; 4];
let _20: u32;
let _21: f32;
let _22: [isize; 1];
let _23: (char, i16, char);
let _24: u32;
let _25: ();
let _26: ();
{
RET = [(-21580_i16)];
RET = [1055_i16];
_2 = [11406_u16,407_u16,5141_u16,5660_u16,57250_u16,39569_u16,8560_u16];
_1 = 32687_i16 as f64;
_2 = [42688_u16,57195_u16,43909_u16,12373_u16,2607_u16,12102_u16,31360_u16];
RET = [8059_i16];
Goto(bb1)
}
bb1 = {
_2 = [52654_u16,41766_u16,52310_u16,6946_u16,44946_u16,17563_u16,38267_u16];
RET = [(-30361_i16)];
RET = [10595_i16];
RET = [(-5148_i16)];
_1 = 36272455377334281915230061996656554734_i128 as f64;
RET = [20593_i16];
RET = [(-5439_i16)];
RET = [21763_i16];
RET = [9362_i16];
_1 = 11740951096538769668_usize as f64;
RET = [15808_i16];
_2 = [34294_u16,61769_u16,22336_u16,54836_u16,32817_u16,49282_u16,15471_u16];
_1 = 19983433726979455491232533796257323226_u128 as f64;
RET = [24530_i16];
RET = [(-28444_i16)];
Goto(bb2)
}
bb2 = {
_4.1 = _1 as isize;
RET = [24311_i16];
RET = [(-25775_i16)];
RET = [(-10061_i16)];
RET = [(-31364_i16)];
_3 = (-9381513238441492661801339172972594_i128) & 70062031552187742503968860001198548921_i128;
_4.2 = [11279289934785975969_u64,9507693823513381697_u64];
_6 = [27911_u16,7955_u16,1778_u16,54299_u16,5167_u16,22395_u16,29151_u16];
RET = [31443_i16];
_10.2 = -(-6252383980530356128_i64);
_11 = 70665416698570302040754705111527207153_u128 as isize;
_7 = -_4.1;
_8 = _3 as f32;
_1 = 78640145408372835443048696993756891837_u128 as f64;
Goto(bb3)
}
bb3 = {
_2 = [53167_u16,63278_u16,50547_u16,51279_u16,57816_u16,31281_u16,39526_u16];
_4.1 = _3 as isize;
RET = [14310_i16];
_9 = _8 + _8;
_8 = -_9;
_9 = -_8;
_7 = _4.1 | _11;
_10.1 = -_7;
RET = [18249_i16];
_10.1 = 15614_u16 as isize;
RET = [897_i16];
_5 = '\u{74b14}';
Goto(bb4)
}
bb4 = {
_15 = (250022650011791733585853988236047007198_u128, _11, _4.2);
_15.2 = [4600064562918789134_u64,11561947094865722001_u64];
_16.0 = core::ptr::addr_of_mut!(_8);
_15.2 = [4678088303897500076_u64,11908403710207520195_u64];
_15.0 = 320475242782219828483232762520793787539_u128;
_15 = (213776912980894190160282184123867894693_u128, _7, _4.2);
_4.0 = 18461_i16 as u128;
_13 = _8 as i128;
_6 = _2;
RET = [(-19116_i16)];
_14 = _15.2;
_3 = _13;
RET = [18755_i16];
_10.1 = _3 as isize;
_1 = _10.2 as f64;
_16.1 = _5 as isize;
_16.1 = _7;
_3 = !_13;
_10.2 = (-7586844339434934399_i64);
_16.2 = _13 as i64;
_16.1 = _11 << _15.1;
match _15.0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
213776912980894190160282184123867894693 => bb10,
_ => bb9
}
}
bb5 = {
_2 = [53167_u16,63278_u16,50547_u16,51279_u16,57816_u16,31281_u16,39526_u16];
_4.1 = _3 as isize;
RET = [14310_i16];
_9 = _8 + _8;
_8 = -_9;
_9 = -_8;
_7 = _4.1 | _11;
_10.1 = -_7;
RET = [18249_i16];
_10.1 = 15614_u16 as isize;
RET = [897_i16];
_5 = '\u{74b14}';
Goto(bb4)
}
bb6 = {
_4.1 = _1 as isize;
RET = [24311_i16];
RET = [(-25775_i16)];
RET = [(-10061_i16)];
RET = [(-31364_i16)];
_3 = (-9381513238441492661801339172972594_i128) & 70062031552187742503968860001198548921_i128;
_4.2 = [11279289934785975969_u64,9507693823513381697_u64];
_6 = [27911_u16,7955_u16,1778_u16,54299_u16,5167_u16,22395_u16,29151_u16];
RET = [31443_i16];
_10.2 = -(-6252383980530356128_i64);
_11 = 70665416698570302040754705111527207153_u128 as isize;
_7 = -_4.1;
_8 = _3 as f32;
_1 = 78640145408372835443048696993756891837_u128 as f64;
Goto(bb3)
}
bb7 = {
_2 = [52654_u16,41766_u16,52310_u16,6946_u16,44946_u16,17563_u16,38267_u16];
RET = [(-30361_i16)];
RET = [10595_i16];
RET = [(-5148_i16)];
_1 = 36272455377334281915230061996656554734_i128 as f64;
RET = [20593_i16];
RET = [(-5439_i16)];
RET = [21763_i16];
RET = [9362_i16];
_1 = 11740951096538769668_usize as f64;
RET = [15808_i16];
_2 = [34294_u16,61769_u16,22336_u16,54836_u16,32817_u16,49282_u16,15471_u16];
_1 = 19983433726979455491232533796257323226_u128 as f64;
RET = [24530_i16];
RET = [(-28444_i16)];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_3 = _13;
_10 = (_16.0, _7, _16.2);
_13 = _16.2 as i128;
_6 = [33184_u16,50845_u16,31916_u16,50536_u16,22743_u16,27494_u16,13295_u16];
_4.2 = [6477755669035746243_u64,16869608072979828982_u64];
_15.2 = [16752610194288018313_u64,6096314102433026553_u64];
_11 = _16.1;
_4.0 = !_15.0;
Goto(bb11)
}
bb11 = {
_16 = (_10.0, _15.1, _10.2);
_1 = 6289467421936133492_u64 as f64;
_4.2 = _14;
_3 = _15.0 as i128;
match _15.0 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb6,
213776912980894190160282184123867894693 => bb13,
_ => bb12
}
}
bb12 = {
_2 = [52654_u16,41766_u16,52310_u16,6946_u16,44946_u16,17563_u16,38267_u16];
RET = [(-30361_i16)];
RET = [10595_i16];
RET = [(-5148_i16)];
_1 = 36272455377334281915230061996656554734_i128 as f64;
RET = [20593_i16];
RET = [(-5439_i16)];
RET = [21763_i16];
RET = [9362_i16];
_1 = 11740951096538769668_usize as f64;
RET = [15808_i16];
_2 = [34294_u16,61769_u16,22336_u16,54836_u16,32817_u16,49282_u16,15471_u16];
_1 = 19983433726979455491232533796257323226_u128 as f64;
RET = [24530_i16];
RET = [(-28444_i16)];
Goto(bb2)
}
bb13 = {
_16.0 = _10.0;
_16.0 = core::ptr::addr_of_mut!(_9);
_16.2 = _10.2;
_7 = _16.1 & _11;
_10 = _16;
_4 = (_15.0, _7, _14);
_4.0 = _15.0 >> _15.1;
_1 = _8 as f64;
_11 = _4.1 + _4.1;
_4.0 = 6101584905443061759_u64 as u128;
_4.2 = _15.2;
_14 = _15.2;
_4 = (_15.0, _7, _14);
_10 = _16;
_19 = [16179_i16,(-21925_i16),(-27781_i16),19552_i16];
RET = [(-26409_i16)];
_16 = (_10.0, _7, _10.2);
_15.1 = _16.1;
_15.2 = [14715421859415312441_u64,12377820645710655970_u64];
_4.2 = [11021467185861124231_u64,7805743770167124529_u64];
_15 = (_4.0, _11, _14);
match _4.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
213776912980894190160282184123867894693 => bb22,
_ => bb21
}
}
bb14 = {
_2 = [52654_u16,41766_u16,52310_u16,6946_u16,44946_u16,17563_u16,38267_u16];
RET = [(-30361_i16)];
RET = [10595_i16];
RET = [(-5148_i16)];
_1 = 36272455377334281915230061996656554734_i128 as f64;
RET = [20593_i16];
RET = [(-5439_i16)];
RET = [21763_i16];
RET = [9362_i16];
_1 = 11740951096538769668_usize as f64;
RET = [15808_i16];
_2 = [34294_u16,61769_u16,22336_u16,54836_u16,32817_u16,49282_u16,15471_u16];
_1 = 19983433726979455491232533796257323226_u128 as f64;
RET = [24530_i16];
RET = [(-28444_i16)];
Goto(bb2)
}
bb15 = {
_16 = (_10.0, _15.1, _10.2);
_1 = 6289467421936133492_u64 as f64;
_4.2 = _14;
_3 = _15.0 as i128;
match _15.0 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb6,
213776912980894190160282184123867894693 => bb13,
_ => bb12
}
}
bb16 = {
_3 = _13;
_10 = (_16.0, _7, _16.2);
_13 = _16.2 as i128;
_6 = [33184_u16,50845_u16,31916_u16,50536_u16,22743_u16,27494_u16,13295_u16];
_4.2 = [6477755669035746243_u64,16869608072979828982_u64];
_15.2 = [16752610194288018313_u64,6096314102433026553_u64];
_11 = _16.1;
_4.0 = !_15.0;
Goto(bb11)
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_2 = [53167_u16,63278_u16,50547_u16,51279_u16,57816_u16,31281_u16,39526_u16];
_4.1 = _3 as isize;
RET = [14310_i16];
_9 = _8 + _8;
_8 = -_9;
_9 = -_8;
_7 = _4.1 | _11;
_10.1 = -_7;
RET = [18249_i16];
_10.1 = 15614_u16 as isize;
RET = [897_i16];
_5 = '\u{74b14}';
Goto(bb4)
}
bb20 = {
_4.1 = _1 as isize;
RET = [24311_i16];
RET = [(-25775_i16)];
RET = [(-10061_i16)];
RET = [(-31364_i16)];
_3 = (-9381513238441492661801339172972594_i128) & 70062031552187742503968860001198548921_i128;
_4.2 = [11279289934785975969_u64,9507693823513381697_u64];
_6 = [27911_u16,7955_u16,1778_u16,54299_u16,5167_u16,22395_u16,29151_u16];
RET = [31443_i16];
_10.2 = -(-6252383980530356128_i64);
_11 = 70665416698570302040754705111527207153_u128 as isize;
_7 = -_4.1;
_8 = _3 as f32;
_1 = 78640145408372835443048696993756891837_u128 as f64;
Goto(bb3)
}
bb21 = {
_15 = (250022650011791733585853988236047007198_u128, _11, _4.2);
_15.2 = [4600064562918789134_u64,11561947094865722001_u64];
_16.0 = core::ptr::addr_of_mut!(_8);
_15.2 = [4678088303897500076_u64,11908403710207520195_u64];
_15.0 = 320475242782219828483232762520793787539_u128;
_15 = (213776912980894190160282184123867894693_u128, _7, _4.2);
_4.0 = 18461_i16 as u128;
_13 = _8 as i128;
_6 = _2;
RET = [(-19116_i16)];
_14 = _15.2;
_3 = _13;
RET = [18755_i16];
_10.1 = _3 as isize;
_1 = _10.2 as f64;
_16.1 = _5 as isize;
_16.1 = _7;
_3 = !_13;
_10.2 = (-7586844339434934399_i64);
_16.2 = _13 as i64;
_16.1 = _11 << _15.1;
match _15.0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
213776912980894190160282184123867894693 => bb10,
_ => bb9
}
}
bb22 = {
_20 = 1541177163_u32 | 3206265655_u32;
_5 = '\u{750dd}';
_2 = _6;
_15 = (_4.0, _7, _14);
_4 = (_15.0, _11, _14);
_19 = [(-1120_i16),856_i16,15710_i16,25152_i16];
_10.0 = core::ptr::addr_of_mut!(_8);
_16.2 = (-1202225101_i32) as i64;
_10.0 = core::ptr::addr_of_mut!(_9);
_8 = -_9;
RET = [26145_i16];
_10 = (_16.0, _11, _16.2);
_7 = _8 as isize;
_15.0 = _4.0;
_18 = !true;
_18 = !true;
_4.2 = [4046019574469153737_u64,1036642488324475447_u64];
RET = [(-9894_i16)];
_15.2 = [9159830034579577495_u64,2521206837049212310_u64];
_4.0 = _15.0 ^ _15.0;
_16.2 = _3 as i64;
_10.2 = _16.2 | _16.2;
_21 = _8;
_3 = 138_u8 as i128;
_8 = _21 + _21;
_24 = _20 ^ _20;
_16.0 = core::ptr::addr_of_mut!(_8);
_22 = [_15.1];
Goto(bb23)
}
bb23 = {
Call(_25 = dump_var(19_usize, 14_usize, Move(_14), 4_usize, Move(_4), 22_usize, Move(_22), 20_usize, Move(_20)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_25 = dump_var(19_usize, 3_usize, Move(_3), 5_usize, Move(_5), 18_usize, Move(_18), 26_usize, _26), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{28fa}'), std::hint::black_box(2430381809_u32), std::hint::black_box(98_i8), std::hint::black_box(13676_i16), std::hint::black_box((-1461133657_i32)), std::hint::black_box(4621408813206075250_i64));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: [u128; 6],
fld1: u64,
fld2: [bool; 4],
fld3: usize,
fld4: *const u8,
fld5: (*mut u32, i64),
fld6: u32,
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
fld0: *mut u32,
fld1: char,
fld2: isize,
fld3: [u128; 6],
fld4: ((*mut f32, isize, i64), u128, [i8; 4]),
fld5: [isize; 1],
fld6: i64,
fld7: *const u8,

},
Variant1{
fld0: [i16; 1],
fld1: char,
fld2: [u128; 6],
fld3: i8,

},
Variant2{
fld0: [i8; 4],
fld1: *mut [u64; 2],
fld2: usize,
fld3: [i16; 1],
fld4: [u128; 6],
fld5: (*mut u8,),
fld6: *mut u32,

},
Variant3{
fld0: i32,
fld1: [u128; 6],
fld2: [u64; 2],
fld3: [bool; 4],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: u128,
fld1: Adt42,
fld2: i128,
fld3: *mut u32,
fld4: u64,
fld5: i32,
fld6: [bool; 4],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut [u64; 2],
fld1: *mut f32,
fld2: [i16; 1],
fld3: u64,
fld4: u32,

},
Variant1{
fld0: bool,
fld1: u8,
fld2: (char, i16, char),
fld3: *const u8,
fld4: *mut u32,
fld5: [u64; 2],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: ((*mut f32, isize, i64), u128, [i8; 4]),
fld2: u128,
fld3: f32,
fld4: i16,
fld5: (*mut [u64; 2],),

},
Variant1{
fld0: u32,
fld1: [u16; 7],
fld2: [u64; 2],
fld3: *const u8,
fld4: *mut [u64; 2],
fld5: i32,

},
Variant2{
fld0: bool,
fld1: i32,
fld2: i128,
fld3: usize,

},
Variant3{
fld0: f64,
fld1: [isize; 1],
fld2: ([i16; 1], u64, *mut [u64; 2], (u128, isize, [u64; 2]), i16),
fld3: i64,
fld4: *mut f32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: usize,
fld1: char,
fld2: isize,
fld3: (char, i16, char),
fld4: u32,
fld5: [i16; 1],
fld6: *mut u8,
fld7: i128,

},
Variant1{
fld0: f64,
fld1: Adt42,
fld2: (*mut f32, isize, i64),
fld3: *mut u8,
fld4: Adt45,
fld5: i32,
fld6: [i16; 4],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: i128,
fld1: Adt46,
fld2: *mut u32,
fld3: i16,

},
Variant1{
fld0: (u128, isize, [u64; 2]),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (u128, isize, [u64; 2]),
fld1: (*mut [u64; 2],),
fld2: isize,
fld3: Adt48,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [u16; 7],
fld1: char,
fld2: *mut f32,
fld3: usize,
fld4: Adt42,
fld5: [i16; 1],
fld6: u64,
fld7: [isize; 8],

},
Variant1{
fld0: [u128; 6],
fld1: [i16; 4],
fld2: [u16; 7],
fld3: Adt45,

},
Variant2{
fld0: Adt44,
fld1: *mut f32,
fld2: u64,
fld3: i8,
fld4: usize,
fld5: (*mut u8,),
fld6: Adt47,
fld7: [u64; 2],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: (u128, isize, [u64; 2]),
fld2: Adt42,
fld3: (*mut u8,),
fld4: [isize; 8],
fld5: [u128; 6],

},
Variant1{
fld0: [i16; 2],
fld1: ([i16; 1], u64, *mut [u64; 2], (u128, isize, [u64; 2]), i16),
fld2: isize,
fld3: i8,
fld4: (*mut u8,),
fld5: *mut u32,
fld6: i64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: i128,
fld1: (*mut f32, isize, i64),
fld2: *mut [u64; 2],
fld3: *mut u8,
fld4: i16,
fld5: Adt42,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i16; 2],
fld1: *const u8,
fld2: (*mut u8,),
fld3: Adt48,
fld4: f32,
fld5: Adt45,
fld6: (char, i16, char),
fld7: u128,

},
Variant1{
fld0: [isize; 1],
fld1: u16,
fld2: (*mut [u64; 2],),
fld3: i8,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
fld0: bool,
fld1: ((*mut f32, isize, i64), u128, [i8; 4]),
fld2: u8,
fld3: Adt52,
fld4: (char, i16, char),
fld5: [u128; 6],
fld6: (*mut f32, isize, i64),
fld7: u32,

},
Variant1{
fld0: (*mut [u64; 2],),
fld1: i8,

},
Variant2{
fld0: (*mut f32, isize, i64),
fld1: char,
fld2: Adt50,
fld3: Adt46,
fld4: u64,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt54,
fld1: [isize; 8],
fld2: (*mut u32, i64),
fld3: i8,
fld4: (*mut u8,),

},
Variant1{
fld0: bool,
fld1: Adt49,

},
Variant2{
fld0: (u128, isize, [u64; 2]),
fld1: u16,
fld2: Adt43,
fld3: Adt50,
fld4: [i16; 2],
fld5: Adt46,
fld6: Adt54,
fld7: *mut u32,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt43,
fld1: ((*mut f32, isize, i64), u128, [i8; 4]),
fld2: Adt44,
fld3: [u16; 7],
fld4: f64,
fld5: Adt55,
fld6: [bool; 4],

},
Variant1{
fld0: Adt54,
fld1: ([i16; 1], u64, *mut [u64; 2], (u128, isize, [u64; 2]), i16),
fld2: [i16; 1],
fld3: Adt51,
fld4: *const u8,
fld5: [isize; 1],
fld6: [i8; 4],
fld7: Adt52,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *mut u8,
fld1: Adt56,

},
Variant1{
fld0: *mut [u64; 2],
fld1: char,
fld2: [bool; 4],
fld3: i8,
fld4: i16,
fld5: u16,

},
Variant2{
fld0: [u64; 2],
fld1: ([i16; 1], u64, *mut [u64; 2], (u128, isize, [u64; 2]), i16),
fld2: Adt52,
fld3: *mut u32,
fld4: [i16; 2],

},
Variant3{
fld0: Adt50,
fld1: Adt42,
fld2: Adt43,
fld3: i8,
fld4: [i16; 4],
fld5: f64,
fld6: Adt56,
fld7: (*mut u32, i64),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: Adt44,
fld1: (*mut [u64; 2],),
fld2: usize,
fld3: i8,
fld4: (*mut u32, i64),
fld5: Adt42,
}

