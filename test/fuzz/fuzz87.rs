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
pub fn fn0(mut _1: usize,mut _2: u128,mut _3: u8,mut _4: i32,mut _5: i16) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _6: f32;
let _7: i128;
let _8: isize;
let _9: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _10: (char, [i16; 3]);
let _11: f32;
let _12: f64;
let _13: *mut i8;
let _14: u128;
let _15: &'static u64;
let _16: i128;
let _17: isize;
let _18: ();
let _19: ();
{
_1 = 283413520836390904_i64 as usize;
_3 = _1 as u8;
_7 = (-131556270758984120233160974702070363885_i128) << _1;
RET = [(-729187880755885812_i64),7823012568846587007_i64,(-2243438050494756300_i64),(-2684048982225940066_i64),(-7943251097062426410_i64),(-6980719604457098688_i64),572694306702059691_i64];
_2 = !103972925955648403348370299679917707798_u128;
_4 = (-1178550193_i32) & 774434696_i32;
RET = [(-2330615517409211927_i64),(-8878103618187955270_i64),(-5331920247710391317_i64),(-7346855166699278554_i64),1577115455062894119_i64,(-6954239352432340780_i64),(-5111194684223800238_i64)];
_9.2.2 = !9223372036854775807_isize;
RET = [(-8810888107737175201_i64),7755595564297872601_i64,6825984239443337554_i64,107900450093797283_i64,(-2232901968305928481_i64),3572045303236014445_i64,2835547469712089698_i64];
_9.2.0 = (36051_u16, _9.2.2);
_9.2.1 = _1 as f32;
_9.2.0 = (9736_u16, _9.2.2);
_9.1.1 = [5913_i16,(-18454_i16),(-13884_i16)];
_9.2.0.0 = 12589_u16;
_9.2.2 = _9.2.0.1 + _9.2.0.1;
_1 = !4_usize;
_10.1 = _9.1.1;
Goto(bb1)
}
bb1 = {
_3 = 50_u8;
_5 = 20249_i16;
_9.2.3 = _4 as u16;
_8 = !_9.2.2;
_10.0 = '\u{1aaf9}';
_9.2.3 = _9.2.0.0 >> _9.2.2;
_9.2.1 = 16848768308368915539_u64 as f32;
_6 = _9.2.1 - _9.2.1;
_9.1.1 = [_5,_5,_5];
_7 = 120902955325291725200871703632669965642_i128;
_9.2.0.0 = _7 as u16;
_9.1 = (_10.0, _10.1);
_10.0 = _9.1.0;
_9.2.0.1 = _8;
_9.2.3 = _9.2.0.0;
_9.0 = _5 as u32;
_10 = (_9.1.0, _9.1.1);
_12 = _5 as f64;
match _5 {
20249 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_10.0 = _9.1.0;
_9.2.0.0 = _6 as u16;
RET = [(-7547803646590088292_i64),5315595403189964830_i64,4631245729382606030_i64,3667125633109011567_i64,8157145851823767022_i64,(-5930004773076847557_i64),4115506779207481169_i64];
RET = [5169873272947773028_i64,7662852752205049358_i64,128381165268950788_i64,(-7684951603802957858_i64),5364320437463046260_i64,(-4836943485696791444_i64),(-6529433303400197197_i64)];
_2 = !281011614377540178155533002336530613438_u128;
Goto(bb4)
}
bb4 = {
_2 = _10.0 as u128;
_2 = _9.2.0.0 as u128;
_9.1 = (_10.0, _10.1);
RET = [(-7157881874399803784_i64),(-8466011951194501045_i64),(-4853909805516279921_i64),(-4819255081549901163_i64),2955631438423531704_i64,8322924095649939331_i64,(-3002207237381905193_i64)];
_12 = _4 as f64;
Call(_9.1.1 = fn1(_8, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = [5632827665808184572_i64,(-4785360238705822755_i64),(-2642509665438114564_i64),(-1179643706160905213_i64),438206019420874704_i64,(-2658493561528166323_i64),(-944045347867337896_i64)];
_1 = !0_usize;
_12 = 8114859641768503224_i64 as f64;
_7 = (-129408671227284208858764766841049595505_i128);
_9.2.0.0 = _9.2.3;
match _3 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
50 => bb12,
_ => bb11
}
}
bb6 = {
_2 = _10.0 as u128;
_2 = _9.2.0.0 as u128;
_9.1 = (_10.0, _10.1);
RET = [(-7157881874399803784_i64),(-8466011951194501045_i64),(-4853909805516279921_i64),(-4819255081549901163_i64),2955631438423531704_i64,8322924095649939331_i64,(-3002207237381905193_i64)];
_12 = _4 as f64;
Call(_9.1.1 = fn1(_8, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_10.0 = _9.1.0;
_9.2.0.0 = _6 as u16;
RET = [(-7547803646590088292_i64),5315595403189964830_i64,4631245729382606030_i64,3667125633109011567_i64,8157145851823767022_i64,(-5930004773076847557_i64),4115506779207481169_i64];
RET = [5169873272947773028_i64,7662852752205049358_i64,128381165268950788_i64,(-7684951603802957858_i64),5364320437463046260_i64,(-4836943485696791444_i64),(-6529433303400197197_i64)];
_2 = !281011614377540178155533002336530613438_u128;
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_3 = 50_u8;
_5 = 20249_i16;
_9.2.3 = _4 as u16;
_8 = !_9.2.2;
_10.0 = '\u{1aaf9}';
_9.2.3 = _9.2.0.0 >> _9.2.2;
_9.2.1 = 16848768308368915539_u64 as f32;
_6 = _9.2.1 - _9.2.1;
_9.1.1 = [_5,_5,_5];
_7 = 120902955325291725200871703632669965642_i128;
_9.2.0.0 = _7 as u16;
_9.1 = (_10.0, _10.1);
_10.0 = _9.1.0;
_9.2.0.1 = _8;
_9.2.3 = _9.2.0.0;
_9.0 = _5 as u32;
_10 = (_9.1.0, _9.1.1);
_12 = _5 as f64;
match _5 {
20249 => bb3,
_ => bb2
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_11 = _12 as f32;
RET = [8375997743804564885_i64,(-9182030107733655764_i64),(-8783521327920667172_i64),(-4466433634127399794_i64),(-5255842045578732966_i64),(-3973915728196591990_i64),(-771072451945092513_i64)];
_9.1.0 = _10.0;
_9.2.1 = _6 * _6;
_11 = -_9.2.1;
_2 = 233140562619156661543636637719211269895_u128;
_7 = (-125038932901290773270435142775970065484_i128) + 142199628374005716171900165042871595328_i128;
_9.2.2 = !_8;
_9.1.0 = _10.0;
_14 = _2;
_9.1 = (_10.0, _10.1);
_9.1 = _10;
_6 = _11;
_16 = -_7;
_3 = 59_u8 - 187_u8;
_10.0 = _9.1.0;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb7,
5 => bb13,
20249 => bb15,
_ => bb14
}
}
bb13 = {
_2 = _10.0 as u128;
_2 = _9.2.0.0 as u128;
_9.1 = (_10.0, _10.1);
RET = [(-7157881874399803784_i64),(-8466011951194501045_i64),(-4853909805516279921_i64),(-4819255081549901163_i64),2955631438423531704_i64,8322924095649939331_i64,(-3002207237381905193_i64)];
_12 = _4 as f64;
Call(_9.1.1 = fn1(_8, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb14 = {
_2 = _10.0 as u128;
_2 = _9.2.0.0 as u128;
_9.1 = (_10.0, _10.1);
RET = [(-7157881874399803784_i64),(-8466011951194501045_i64),(-4853909805516279921_i64),(-4819255081549901163_i64),2955631438423531704_i64,8322924095649939331_i64,(-3002207237381905193_i64)];
_12 = _4 as f64;
Call(_9.1.1 = fn1(_8, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_1 = 6_usize;
_2 = !_14;
_12 = _14 as f64;
Goto(bb16)
}
bb16 = {
Call(_18 = dump_var(0_usize, 8_usize, Move(_8), 16_usize, Move(_16), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_18 = dump_var(0_usize, 10_usize, Move(_10), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: f64) -> [i16; 3] {
mir! {
type RET = [i16; 3];
let _3: [i128; 5];
let _4: [u64; 8];
let _5: u16;
let _6: isize;
let _7: (char, [i16; 3]);
let _8: f64;
let _9: *const [i128; 4];
let _10: (usize, i32);
let _11: i16;
let _12: [u64; 2];
let _13: (usize, i32);
let _14: usize;
let _15: f64;
let _16: bool;
let _17: isize;
let _18: (bool, u64, f64, u64);
let _19: [usize; 8];
let _20: f32;
let _21: [u64; 2];
let _22: i8;
let _23: isize;
let _24: (Adt32, (u128, f64), *const Adt36);
let _25: (Adt26, i16, *const usize, &'static Adt17);
let _26: &'static Adt17;
let _27: ();
let _28: ();
{
RET = [(-6036_i16),32373_i16,(-1271_i16)];
_3 = [(-157094008450744861362076200233808196991_i128),(-131355392797043384036070601513821793243_i128),(-139460063985318878490896118282876753335_i128),125215214244477819669030643699911409899_i128,(-75948128357398573227881127538527207025_i128)];
_4 = [8913772283717666058_u64,16804589290053377832_u64,2523355695989186050_u64,17256484066531765225_u64,10367523887561256060_u64,7387552805600955370_u64,14947507256590799386_u64,6563461319352502722_u64];
_3 = [167375754134647283345628061264253596830_i128,113981031368803502600322616926451212131_i128,(-16124808444051697419758051100126789296_i128),(-90845343627053071845496586754591548259_i128),125046886414794706003432641863710887666_i128];
RET = [3922_i16,11118_i16,2564_i16];
RET = [19596_i16,(-24528_i16),4129_i16];
_1 = (-55_isize) * 9223372036854775807_isize;
_4 = [16445618220748368028_u64,3264592259651179393_u64,402759305675529562_u64,10037177339360071133_u64,11904544635659343545_u64,16603078372734404362_u64,13165042307024411682_u64,4112435669142259949_u64];
_3 = [(-79379874161079536240610212587725276872_i128),(-85154230522324613628280063372450542687_i128),(-157226989970575357471866616197588513766_i128),150872773869969731548239614412849483730_i128,(-126795344261536033546442764822007362198_i128)];
_1 = !(-9223372036854775808_isize);
_1 = 9223372036854775807_isize * (-9223372036854775808_isize);
_4 = [13677516514807395738_u64,8235858417790799687_u64,17246249732811589926_u64,9800529436058534542_u64,17588825029150891285_u64,9585458460117769735_u64,14892797193415620746_u64,10308484546050283932_u64];
_1 = 9223372036854775807_isize;
_2 = (-57_i8) as f64;
_1 = (-9223372036854775808_isize);
_1 = (-9223372036854775808_isize);
_4 = [12484722188216754968_u64,17544903826640118019_u64,6866648634955385008_u64,16373127876137204225_u64,13687616949646767181_u64,13216308535507791748_u64,14041966891731822297_u64,6223503053175884423_u64];
_7 = ('\u{adfc7}', RET);
_5 = 631402991578970515_i64 as u16;
RET = [25979_i16,(-12658_i16),28571_i16];
_5 = !37793_u16;
RET = _7.1;
Call(_7.1 = fn2(_1, _2, _3, _7.0, _4, _7.0, _3, _4, _4, _3, _3, _7.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [10042148002735968526_u64,11149916505918133459_u64,17891209288055519044_u64,10873999631889361618_u64,9273798314050210493_u64,18184366972104362389_u64,8847502304228428693_u64,18332035276764268529_u64];
RET = [14605_i16,15891_i16,(-27410_i16)];
_4 = [18390514591072828494_u64,3420503744151867487_u64,8084959044745340526_u64,8157274993072633318_u64,9284060656899832844_u64,753506224463491117_u64,17571126994951729077_u64,3882741737740843123_u64];
_7.1 = [25355_i16,20634_i16,(-18799_i16)];
_5 = 61746_u16 + 30756_u16;
_7.1 = [8151_i16,22843_i16,3762_i16];
_6 = _1 | _1;
_8 = -_2;
_1 = !_6;
_7.0 = '\u{ccc68}';
_7 = ('\u{d3b36}', RET);
_6 = 2146449345_u32 as isize;
RET = [(-28364_i16),23374_i16,(-17667_i16)];
_8 = _2 - _2;
RET = [24715_i16,(-7917_i16),(-3058_i16)];
_10.1 = -631935982_i32;
_2 = _6 as f64;
_11 = 1850953699_u32 as i16;
_10 = (7792280974576104213_usize, 302608875_i32);
_13 = _10;
_10 = (_13.0, _13.1);
_1 = _6;
_6 = -_1;
_13.1 = -_10.1;
_11 = (-12634_i16);
_10 = (_13.0, _13.1);
match _10.0 {
0 => bb2,
1 => bb3,
7792280974576104213 => bb5,
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
_5 = 18838_u16;
_10 = (_13.0, _13.1);
_13.1 = (-63_i8) as i32;
_7.1 = [_11,_11,_11];
_13.0 = !_10.0;
_7 = ('\u{6a181}', RET);
_4 = [2373799490428101684_u64,4382723781974496254_u64,7535276633309169883_u64,7973596248550772891_u64,15872825200146242746_u64,562147836206976491_u64,5324183112066312738_u64,14928878055678779782_u64];
_14 = 320482616191789246269346700805551612448_u128 as usize;
_11 = -31898_i16;
_8 = _1 as f64;
_10.1 = _13.1 & _13.1;
_13 = (_14, _10.1);
_14 = _10.0 >> _13.1;
_15 = _8;
_10.0 = _13.0 | _14;
_13 = _10;
_7.1 = [_11,_11,_11];
_13.1 = _10.1 >> _14;
_16 = false;
_10 = _13;
Goto(bb6)
}
bb6 = {
_16 = false;
_15 = _5 as f64;
_13 = (_10.0, _10.1);
_7 = ('\u{9cbb7}', RET);
_12 = [12449683166263950811_u64,6473937526688008843_u64];
_2 = -_8;
_5 = 33294_u16;
_4 = [3374867862621229272_u64,9682236427674835450_u64,2988332569887195917_u64,6632525761191422764_u64,4251199898454529207_u64,15098833032165241613_u64,8493071720242552867_u64,1861645529556679955_u64];
RET = [_11,_11,_11];
_10.1 = -_13.1;
RET = [_11,_11,_11];
_13 = (_10.0, _10.1);
_14 = !_13.0;
_17 = _6;
_3 = [160961790755453311288659478529439895500_i128,68497915545082897216223191608349211551_i128,21307463143114504443162315168987627469_i128,(-15061449618582374040836333447993405100_i128),15095337294010633108792965032069941770_i128];
_17 = _11 as isize;
_7.1 = RET;
_17 = !_1;
_11 = 25715_i16 - 26968_i16;
_7 = ('\u{8d860}', RET);
_14 = _13.0 << _10.1;
_16 = false;
_3 = [(-96253155503559536908439325464090681622_i128),(-71741906843856064135815138787647118449_i128),(-39776542765108633533954490700630292369_i128),(-43757928618504262785951508415893743172_i128),(-87060361443110898840632621996595514356_i128)];
match _5 {
0 => bb3,
33294 => bb7,
_ => bb4
}
}
bb7 = {
_3 = [(-76130115327468238086879144060688676712_i128),(-141036435638976891725364278882439626445_i128),47473877797718899943271522352449545970_i128,49832267062444677810227618289997574746_i128,159539733764886533487515658017109611417_i128];
_16 = true;
_16 = true;
_10.0 = !_14;
_10 = _13;
_7 = ('\u{3e3b7}', RET);
_8 = (-62_i8) as f64;
_7.0 = '\u{b0306}';
_6 = !_17;
Goto(bb8)
}
bb8 = {
RET = [_11,_11,_11];
_13.1 = _10.1;
_7.1 = [_11,_11,_11];
RET = [_11,_11,_11];
_7 = ('\u{c52ea}', RET);
_7 = ('\u{100b3c}', RET);
_15 = _8 * _2;
_3 = [32153924068026653656994754627427303413_i128,63415788373227475640297001175414406887_i128,117028042859219923722574691900527571336_i128,(-35433642365425596247350446071010381233_i128),119341220141744941171974180163947460718_i128];
_14 = _13.0;
_14 = _10.0;
_1 = _17;
_17 = _6;
_13.0 = !_10.0;
_6 = _1 >> _10.0;
RET = _7.1;
_10 = (_14, _13.1);
_12 = [4271833811245965290_u64,5896930345634296099_u64];
_8 = _15 * _2;
_17 = !_6;
_7 = ('\u{bc844}', RET);
Goto(bb9)
}
bb9 = {
_10.0 = !_13.0;
_6 = _17 | _17;
_7.1 = [_11,_11,_11];
_13.1 = _10.1 << _17;
_2 = _15 - _8;
_5 = _17 as u16;
_7.0 = '\u{c424c}';
_13.1 = -_10.1;
_3 = [(-115246595479299659156264481666478908533_i128),(-115310958138750618408084031796576456279_i128),167088993644703013194048056317678130229_i128,150357500194671436838361295448693264412_i128,25990932313499198417676975674161824853_i128];
_13.1 = _10.1 + _10.1;
RET = _7.1;
_6 = (-6185543851349988303_i64) as isize;
_18.0 = _17 <= _17;
_16 = _18.0 > _18.0;
_18 = (_16, 8901692643091274813_u64, _15, 1599161864501289134_u64);
_16 = _18.0 | _18.0;
_2 = _10.1 as f64;
Goto(bb10)
}
bb10 = {
RET = [_11,_11,_11];
Goto(bb11)
}
bb11 = {
_14 = 94070800415324123848525974969224992712_u128 as usize;
_7.1 = RET;
_18.3 = _18.1;
_20 = _2 as f32;
_10.1 = !_13.1;
_6 = _10.1 as isize;
_7.0 = '\u{2d89}';
_16 = _18.0;
_7.1 = RET;
_13 = (_10.0, _10.1);
_12 = [_18.3,_18.1];
_16 = !_18.0;
_19 = [_13.0,_13.0,_13.0,_10.0,_13.0,_10.0,_10.0,_10.0];
_19 = [_10.0,_10.0,_10.0,_10.0,_13.0,_10.0,_10.0,_10.0];
_15 = -_8;
_15 = _8;
_4 = [_18.3,_18.3,_18.3,_18.3,_18.3,_18.1,_18.3,_18.1];
_13 = _10;
_13.0 = _14 >> _6;
_15 = (-6359210814270715683_i64) as f64;
RET = [_11,_11,_11];
match _18.3 {
0 => bb7,
1 => bb10,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
8901692643091274813 => bb18,
_ => bb17
}
}
bb12 = {
Return()
}
bb13 = {
_10.0 = !_13.0;
_6 = _17 | _17;
_7.1 = [_11,_11,_11];
_13.1 = _10.1 << _17;
_2 = _15 - _8;
_5 = _17 as u16;
_7.0 = '\u{c424c}';
_13.1 = -_10.1;
_3 = [(-115246595479299659156264481666478908533_i128),(-115310958138750618408084031796576456279_i128),167088993644703013194048056317678130229_i128,150357500194671436838361295448693264412_i128,25990932313499198417676975674161824853_i128];
_13.1 = _10.1 + _10.1;
RET = _7.1;
_6 = (-6185543851349988303_i64) as isize;
_18.0 = _17 <= _17;
_16 = _18.0 > _18.0;
_18 = (_16, 8901692643091274813_u64, _15, 1599161864501289134_u64);
_16 = _18.0 | _18.0;
_2 = _10.1 as f64;
Goto(bb10)
}
bb14 = {
RET = [_11,_11,_11];
_13.1 = _10.1;
_7.1 = [_11,_11,_11];
RET = [_11,_11,_11];
_7 = ('\u{c52ea}', RET);
_7 = ('\u{100b3c}', RET);
_15 = _8 * _2;
_3 = [32153924068026653656994754627427303413_i128,63415788373227475640297001175414406887_i128,117028042859219923722574691900527571336_i128,(-35433642365425596247350446071010381233_i128),119341220141744941171974180163947460718_i128];
_14 = _13.0;
_14 = _10.0;
_1 = _17;
_17 = _6;
_13.0 = !_10.0;
_6 = _1 >> _10.0;
RET = _7.1;
_10 = (_14, _13.1);
_12 = [4271833811245965290_u64,5896930345634296099_u64];
_8 = _15 * _2;
_17 = !_6;
_7 = ('\u{bc844}', RET);
Goto(bb9)
}
bb15 = {
_3 = [(-76130115327468238086879144060688676712_i128),(-141036435638976891725364278882439626445_i128),47473877797718899943271522352449545970_i128,49832267062444677810227618289997574746_i128,159539733764886533487515658017109611417_i128];
_16 = true;
_16 = true;
_10.0 = !_14;
_10 = _13;
_7 = ('\u{3e3b7}', RET);
_8 = (-62_i8) as f64;
_7.0 = '\u{b0306}';
_6 = !_17;
Goto(bb8)
}
bb16 = {
_4 = [10042148002735968526_u64,11149916505918133459_u64,17891209288055519044_u64,10873999631889361618_u64,9273798314050210493_u64,18184366972104362389_u64,8847502304228428693_u64,18332035276764268529_u64];
RET = [14605_i16,15891_i16,(-27410_i16)];
_4 = [18390514591072828494_u64,3420503744151867487_u64,8084959044745340526_u64,8157274993072633318_u64,9284060656899832844_u64,753506224463491117_u64,17571126994951729077_u64,3882741737740843123_u64];
_7.1 = [25355_i16,20634_i16,(-18799_i16)];
_5 = 61746_u16 + 30756_u16;
_7.1 = [8151_i16,22843_i16,3762_i16];
_6 = _1 | _1;
_8 = -_2;
_1 = !_6;
_7.0 = '\u{ccc68}';
_7 = ('\u{d3b36}', RET);
_6 = 2146449345_u32 as isize;
RET = [(-28364_i16),23374_i16,(-17667_i16)];
_8 = _2 - _2;
RET = [24715_i16,(-7917_i16),(-3058_i16)];
_10.1 = -631935982_i32;
_2 = _6 as f64;
_11 = 1850953699_u32 as i16;
_10 = (7792280974576104213_usize, 302608875_i32);
_13 = _10;
_10 = (_13.0, _13.1);
_1 = _6;
_6 = -_1;
_13.1 = -_10.1;
_11 = (-12634_i16);
_10 = (_13.0, _13.1);
match _10.0 {
0 => bb2,
1 => bb3,
7792280974576104213 => bb5,
_ => bb4
}
}
bb17 = {
Return()
}
bb18 = {
_21 = [_18.1,_18.3];
_5 = 3827289687_u32 as u16;
_24.1.1 = _2;
_8 = (-47_i8) as f64;
_17 = _6 << _18.1;
_24.1 = (106765964837332312731578110254575415225_u128, _2);
_22 = 80_i8 * 119_i8;
_8 = _2;
_10 = (_13.0, _13.1);
_22 = (-75_i8) >> _17;
_10 = _13;
_13.0 = _10.0 ^ _10.0;
_24.1.1 = -_8;
_19 = [_10.0,_10.0,_13.0,_13.0,_10.0,_10.0,_13.0,_13.0];
_22 = -(-55_i8);
_24.1 = (53372326745862529525039790301116427216_u128, _2);
_18 = (_16, 3774379710336013707_u64, _2, 15476995122059724155_u64);
Goto(bb19)
}
bb19 = {
Call(_27 = dump_var(1_usize, 19_usize, Move(_19), 11_usize, Move(_11), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_27 = dump_var(1_usize, 22_usize, Move(_22), 13_usize, Move(_13), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: f64,mut _3: [i128; 5],mut _4: char,mut _5: [u64; 8],mut _6: char,mut _7: [i128; 5],mut _8: [u64; 8],mut _9: [u64; 8],mut _10: [i128; 5],mut _11: [i128; 5],mut _12: char) -> [i16; 3] {
mir! {
type RET = [i16; 3];
let _13: [u16; 2];
let _14: usize;
let _15: u128;
let _16: [u16; 2];
let _17: f64;
let _18: char;
let _19: isize;
let _20: i16;
let _21: isize;
let _22: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _23: [usize; 8];
let _24: u32;
let _25: char;
let _26: &'static u64;
let _27: *const u128;
let _28: char;
let _29: i32;
let _30: isize;
let _31: ();
let _32: ();
{
_2 = 45_i8 as f64;
_7 = [(-46217249261889800725855206892971840834_i128),(-78284574209645354987012046304931358706_i128),119883198677256169853267884826581728866_i128,(-102326976661767651022671771713174192671_i128),(-4715067276692130658751002021910922661_i128)];
RET = [(-28383_i16),18362_i16,21040_i16];
_6 = _4;
_2 = (-219263700_i32) as f64;
_13 = [11667_u16,64005_u16];
_10 = _3;
_10 = _3;
_10 = [160696593967107194661067545553075084644_i128,78456805701794652975602323980271750147_i128,(-21940073554402142077711568508904978018_i128),14982721875460800880327743310861498022_i128,(-49576126983936337205216165403995892916_i128)];
_8 = [7766808842791084080_u64,4183742503651148532_u64,703540567987137966_u64,10505964195708753805_u64,6478235115463544594_u64,10681529457234886992_u64,1585206085115308605_u64,4131519092156999076_u64];
_6 = _4;
RET = [(-17313_i16),14020_i16,(-27681_i16)];
_7 = [(-13206729045029848689511554041709386650_i128),(-82784298732171107433618992621690857107_i128),134474446060908030207886267434622813766_i128,61486531559754853247649202495879981315_i128,24164197420583956812182682547787627006_i128];
_7 = _11;
_13 = [28137_u16,25499_u16];
_11 = [68889777457114075244652905573822748634_i128,33460839750661092325337416748466315841_i128,(-61702439791817331559614490491765288514_i128),(-78916108543419858177786720064157484032_i128),(-134683494117259774274288761931143754402_i128)];
_9 = _5;
_13 = [6069_u16,1333_u16];
_7 = _11;
_10 = [(-163343620168891889312204709243015052922_i128),71981460999360313450868535914638399527_i128,59091539830568765264401453440601699402_i128,104182573331432210540708304092749012257_i128,(-11705791885751254392990367615550186154_i128)];
_10 = [(-38649220093298569885833133593699266642_i128),34380315207138906805831496473162296890_i128,(-96551640738994141898019460716926551812_i128),35892710037420451215502759481056984620_i128,30730569959068313866992796865344263077_i128];
Goto(bb1)
}
bb1 = {
_11 = _7;
_14 = 1542701169202221356_usize;
_15 = _1 as u128;
_10 = [150730753172546037501000011076344400258_i128,(-37020523170521656241651079437423305303_i128),(-123386475472090585042779273660849572547_i128),(-91009102286612634847813665899497538233_i128),39024725852520220068125958080869819804_i128];
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
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
RET = [9090_i16,144_i16,(-21256_i16)];
_12 = _6;
RET = [(-5827_i16),29811_i16,32713_i16];
_6 = _4;
_11 = [(-165989771067958067158137717572388442491_i128),(-67756764816926310905616792562803815420_i128),164012266218144367601836346910404170761_i128,(-164731664503705812332147611569438691461_i128),93756871731337447862050245443288740662_i128];
_4 = _12;
_7 = [7972328389511309922247783938713498157_i128,78627102992890259451460225834135049533_i128,(-119263491976788648132955631442839782912_i128),162984248981450585053522820751707829440_i128,(-29018464778855335009550750387798105391_i128)];
_9 = [8102574261484330296_u64,9518808451550542347_u64,10788261975903324247_u64,18335550085558789763_u64,4131741447949698922_u64,244876853858221725_u64,3645230890598943842_u64,17885621976465494459_u64];
RET = [5111_i16,(-17041_i16),(-12987_i16)];
_17 = _2;
_15 = 238474281763632420692891283744213332381_u128;
_18 = _4;
_8 = [6624673769334468108_u64,4096188071098372111_u64,5192292281351340418_u64,4138813708851436637_u64,9601599263551329277_u64,7505519296452380349_u64,17049725941257430063_u64,11703170834925394894_u64];
RET = [1826_i16,(-27634_i16),9449_i16];
_6 = _4;
_11 = [82521063741851931409950429072821151444_i128,(-123669213803141594053201546047787564870_i128),99108532274092612241262439190134467965_i128,69127348825384104286751404450094586262_i128,133034367503752657111475124597221653574_i128];
_16 = [24722_u16,18094_u16];
_19 = _1 + _1;
_19 = 1980125108_i32 as isize;
_17 = _2 + _2;
_5 = [11050816564926823102_u64,6927216070152897678_u64,6316337604609836200_u64,16959394461767937798_u64,12864582889531980585_u64,10835114110004093985_u64,438169278261071526_u64,17268177037059080783_u64];
RET = [(-15885_i16),(-14477_i16),25697_i16];
Call(_12 = fn3(_9, _8, _4, _9, _4, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = [71554120569006191170216459623825827347_i128,(-26651179982667513465452504826769220914_i128),(-42095209199755173310850572461201188161_i128),100852770601705069562063406791880982914_i128,128758049034961324152643390255231905609_i128];
_22.2.1 = (-8571090922139426610_i64) as f32;
_22.2.0 = (52654_u16, _1);
_19 = _1 + _1;
_7 = _10;
_13 = [_22.2.0.0,_22.2.0.0];
_22.1 = (_12, RET);
_22.2.3 = _22.2.0.0 + _22.2.0.0;
_21 = -_19;
match _15 {
0 => bb3,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
238474281763632420692891283744213332381 => bb17,
_ => bb16
}
}
bb10 = {
RET = [9090_i16,144_i16,(-21256_i16)];
_12 = _6;
RET = [(-5827_i16),29811_i16,32713_i16];
_6 = _4;
_11 = [(-165989771067958067158137717572388442491_i128),(-67756764816926310905616792562803815420_i128),164012266218144367601836346910404170761_i128,(-164731664503705812332147611569438691461_i128),93756871731337447862050245443288740662_i128];
_4 = _12;
_7 = [7972328389511309922247783938713498157_i128,78627102992890259451460225834135049533_i128,(-119263491976788648132955631442839782912_i128),162984248981450585053522820751707829440_i128,(-29018464778855335009550750387798105391_i128)];
_9 = [8102574261484330296_u64,9518808451550542347_u64,10788261975903324247_u64,18335550085558789763_u64,4131741447949698922_u64,244876853858221725_u64,3645230890598943842_u64,17885621976465494459_u64];
RET = [5111_i16,(-17041_i16),(-12987_i16)];
_17 = _2;
_15 = 238474281763632420692891283744213332381_u128;
_18 = _4;
_8 = [6624673769334468108_u64,4096188071098372111_u64,5192292281351340418_u64,4138813708851436637_u64,9601599263551329277_u64,7505519296452380349_u64,17049725941257430063_u64,11703170834925394894_u64];
RET = [1826_i16,(-27634_i16),9449_i16];
_6 = _4;
_11 = [82521063741851931409950429072821151444_i128,(-123669213803141594053201546047787564870_i128),99108532274092612241262439190134467965_i128,69127348825384104286751404450094586262_i128,133034367503752657111475124597221653574_i128];
_16 = [24722_u16,18094_u16];
_19 = _1 + _1;
_19 = 1980125108_i32 as isize;
_17 = _2 + _2;
_5 = [11050816564926823102_u64,6927216070152897678_u64,6316337604609836200_u64,16959394461767937798_u64,12864582889531980585_u64,10835114110004093985_u64,438169278261071526_u64,17268177037059080783_u64];
RET = [(-15885_i16),(-14477_i16),25697_i16];
Call(_12 = fn3(_9, _8, _4, _9, _4, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
_11 = _7;
_14 = 1542701169202221356_usize;
_15 = _1 as u128;
_10 = [150730753172546037501000011076344400258_i128,(-37020523170521656241651079437423305303_i128),(-123386475472090585042779273660849572547_i128),(-91009102286612634847813665899497538233_i128),39024725852520220068125958080869819804_i128];
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
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
_17 = _2;
_22.2.0.1 = _1 << _21;
_15 = !117021410589147961694352967819575997640_u128;
RET = [9046_i16,28492_i16,8882_i16];
_20 = (-23736_i16) - 7302_i16;
_23 = [_14,_14,_14,_14,_14,_14,_14,_14];
_21 = !_19;
_24 = 1811564877_u32 & 2865560112_u32;
_9 = [15763155314631076971_u64,18042946741801867431_u64,6278703506715896198_u64,2609799306708766167_u64,4516802046892109455_u64,16001675926052410879_u64,8451526164185621204_u64,1447008182444967733_u64];
_1 = -_19;
_27 = core::ptr::addr_of!(_15);
_22.1.0 = _18;
_13 = [_22.2.3,_22.2.3];
_22.2.3 = !_22.2.0.0;
_5 = [8107515973497152845_u64,14522935619337553623_u64,12467870395996700955_u64,15813918783773848700_u64,16328872186532005085_u64,8080570613279125787_u64,17072606114887799433_u64,7604577459143058513_u64];
_22.0 = 206_u8 as u32;
_22.2.2 = _19 * _22.2.0.1;
(*_27) = 23415284553634710919181139660128974614_u128 >> _22.2.2;
_27 = core::ptr::addr_of!((*_27));
_22.1 = (_6, RET);
_8 = [702437730840431945_u64,17514492613748248997_u64,17985820217218303549_u64,5351983038881608970_u64,8027320519806131117_u64,10665319593297517119_u64,13694084149054638529_u64,14855766177065169407_u64];
_25 = _18;
_1 = _20 as isize;
(*_27) = 168945870988331038538412936708861668676_u128;
_10 = _7;
_9 = [2131250270543316026_u64,14713674124526900518_u64,9683377797184655102_u64,6333508263063945107_u64,12240567836724843403_u64,15223395805016779903_u64,6906875610186545594_u64,15020261253054521823_u64];
RET = _22.1.1;
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(2_usize, 8_usize, Move(_8), 4_usize, Move(_4), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(2_usize, 20_usize, Move(_20), 24_usize, Move(_24), 18_usize, Move(_18), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_31 = dump_var(2_usize, 1_usize, Move(_1), 19_usize, Move(_19), 5_usize, Move(_5), 32_usize, _32), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u64; 8],mut _2: [u64; 8],mut _3: char,mut _4: [u64; 8],mut _5: char,mut _6: f64) -> char {
mir! {
type RET = char;
let _7: f32;
let _8: i8;
let _9: f64;
let _10: [i16; 3];
let _11: [i128; 4];
let _12: isize;
let _13: u128;
let _14: Adt61;
let _15: char;
let _16: i32;
let _17: &'static [u16; 6];
let _18: bool;
let _19: [u64; 8];
let _20: i32;
let _21: [u64; 5];
let _22: bool;
let _23: f64;
let _24: Adt26;
let _25: [u8; 2];
let _26: f32;
let _27: bool;
let _28: Adt32;
let _29: [u64; 8];
let _30: bool;
let _31: isize;
let _32: [u16; 2];
let _33: [i128; 4];
let _34: isize;
let _35: [i16; 3];
let _36: i64;
let _37: f32;
let _38: &'static [u16; 6];
let _39: *mut [i128; 2];
let _40: ();
let _41: ();
{
_8 = -(-95_i8);
RET = _3;
RET = _3;
_2 = [9572124112304503458_u64,14929718281720266250_u64,13597038480314267946_u64,4979128482340589215_u64,5956710024760544888_u64,8346158656526409318_u64,12960569842451044597_u64,3625777321835635820_u64];
_5 = _3;
_4 = [6981811639923054515_u64,2461044707070477712_u64,17691521182201256649_u64,9070899501497185189_u64,8474993572909712864_u64,6084549792921233531_u64,13704260555002288349_u64,3023102707635993600_u64];
Call(_4 = fn4(_3, RET, _2, _2, _2, RET, _3, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _6;
_6 = _9;
_1 = [12832618423316704092_u64,2598888343897207692_u64,17551499773703198306_u64,14363094029222832254_u64,3803488775450821658_u64,11107886112727151911_u64,9518955525926897820_u64,4149720782227468313_u64];
RET = _3;
_3 = RET;
_3 = _5;
_5 = _3;
_11 = [49159339497558882064954344125737353740_i128,(-41840149859621776547422549936091611684_i128),73107471515693264219756230977419701159_i128,5267320582517702049431928130085676145_i128];
_9 = 9223372036854775807_isize as f64;
_11 = [(-96380070581579320666724426715007674977_i128),(-145212171314887963820155229649971228681_i128),(-140401825203020641098689367388982748373_i128),97232588403218757683743026052459118104_i128];
_9 = _6 + _6;
RET = _5;
RET = _5;
_3 = RET;
RET = _5;
_1 = [2574720103561959644_u64,8261712628903773643_u64,16894818151753645557_u64,15410965001822916596_u64,6505585919024204273_u64,4373810856088617390_u64,17498488837497582365_u64,317884000001203578_u64];
_3 = RET;
_6 = _8 as f64;
_7 = _8 as f32;
RET = _3;
RET = _5;
RET = _3;
_9 = _8 as f64;
_9 = -_6;
_12 = -(-9223372036854775808_isize);
_6 = -_9;
Goto(bb2)
}
bb2 = {
_8 = -110_i8;
_10 = [28390_i16,(-10726_i16),(-5193_i16)];
RET = _3;
_2 = _1;
_7 = 10337893168532109448113779752931363246_u128 as f32;
_3 = RET;
_5 = RET;
_10 = [(-9056_i16),(-18307_i16),(-12416_i16)];
_1 = _2;
_12 = 1667077932_u32 as isize;
_13 = !84472036510318621279348129468192404031_u128;
_7 = 1621810240_u32 as f32;
_13 = 287785111445584342272193277978269807991_u128 | 226587371524241833306156718132997959788_u128;
_12 = _8 as isize;
_18 = !true;
_9 = -_6;
_16 = 325013903_i32;
RET = _5;
_12 = !9223372036854775807_isize;
_1 = [16652810020685791651_u64,6456449668542814644_u64,6168940262899593090_u64,14562834759830658495_u64,10446185637739415716_u64,5163910756500032536_u64,10685470379622340082_u64,3670642155412596763_u64];
match _16 {
325013903 => bb3,
_ => bb1
}
}
bb3 = {
_11 = [1829037874567120315597900907134429649_i128,(-140685424821662994002462308894747452948_i128),5174677023821093693189805306623303650_i128,136411531856788878803030415163988456438_i128];
_7 = 111212660505906682185884158332894686102_i128 as f32;
_18 = !false;
_4 = [9516539720560146520_u64,15877438008393679194_u64,11218166315675423650_u64,8227259107972447811_u64,7387231664139995006_u64,16695070011684819547_u64,14855417204484561079_u64,9640606670088241137_u64];
_18 = !true;
_10 = [30910_i16,(-9415_i16),2718_i16];
RET = _5;
_18 = true;
_15 = RET;
_4 = [512372136223569719_u64,4245159207685141307_u64,15441087502327217922_u64,13277898780594717699_u64,10505777706147276274_u64,2849893713856893814_u64,14347592740332436554_u64,6608895411794097008_u64];
_19 = [10870944203128842080_u64,127608771125317163_u64,503334360996753362_u64,9636304006958814657_u64,982096669126158612_u64,8054166506175770831_u64,12803140389657191981_u64,8509249260203810092_u64];
_5 = RET;
_9 = _6;
_8 = (-84_i8);
_6 = _7 as f64;
_13 = 217147143464592399906158900599540881376_u128 >> _16;
_3 = _15;
_5 = _15;
RET = _3;
_20 = _16;
_12 = _7 as isize;
_18 = _8 != _8;
_9 = -_6;
_5 = _15;
RET = _3;
_3 = _15;
match _20 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
325013903 => bb8,
_ => bb7
}
}
bb4 = {
_8 = -110_i8;
_10 = [28390_i16,(-10726_i16),(-5193_i16)];
RET = _3;
_2 = _1;
_7 = 10337893168532109448113779752931363246_u128 as f32;
_3 = RET;
_5 = RET;
_10 = [(-9056_i16),(-18307_i16),(-12416_i16)];
_1 = _2;
_12 = 1667077932_u32 as isize;
_13 = !84472036510318621279348129468192404031_u128;
_7 = 1621810240_u32 as f32;
_13 = 287785111445584342272193277978269807991_u128 | 226587371524241833306156718132997959788_u128;
_12 = _8 as isize;
_18 = !true;
_9 = -_6;
_16 = 325013903_i32;
RET = _5;
_12 = !9223372036854775807_isize;
_1 = [16652810020685791651_u64,6456449668542814644_u64,6168940262899593090_u64,14562834759830658495_u64,10446185637739415716_u64,5163910756500032536_u64,10685470379622340082_u64,3670642155412596763_u64];
match _16 {
325013903 => bb3,
_ => bb1
}
}
bb5 = {
_9 = _6;
_6 = _9;
_1 = [12832618423316704092_u64,2598888343897207692_u64,17551499773703198306_u64,14363094029222832254_u64,3803488775450821658_u64,11107886112727151911_u64,9518955525926897820_u64,4149720782227468313_u64];
RET = _3;
_3 = RET;
_3 = _5;
_5 = _3;
_11 = [49159339497558882064954344125737353740_i128,(-41840149859621776547422549936091611684_i128),73107471515693264219756230977419701159_i128,5267320582517702049431928130085676145_i128];
_9 = 9223372036854775807_isize as f64;
_11 = [(-96380070581579320666724426715007674977_i128),(-145212171314887963820155229649971228681_i128),(-140401825203020641098689367388982748373_i128),97232588403218757683743026052459118104_i128];
_9 = _6 + _6;
RET = _5;
RET = _5;
_3 = RET;
RET = _5;
_1 = [2574720103561959644_u64,8261712628903773643_u64,16894818151753645557_u64,15410965001822916596_u64,6505585919024204273_u64,4373810856088617390_u64,17498488837497582365_u64,317884000001203578_u64];
_3 = RET;
_6 = _8 as f64;
_7 = _8 as f32;
RET = _3;
RET = _5;
RET = _3;
_9 = _8 as f64;
_9 = -_6;
_12 = -(-9223372036854775808_isize);
_6 = -_9;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_2 = [6497657372655602746_u64,8725470149297550495_u64,14724239290355498218_u64,16413675226117590425_u64,6920684562177449563_u64,10125726047499231604_u64,3234287514229813644_u64,10205176281739363720_u64];
_4 = _19;
_9 = (-59564729382271378439455808738296106932_i128) as f64;
_19 = _1;
_8 = 15003_u16 as i8;
_7 = 7_usize as f32;
_1 = [3723014279419980462_u64,9371176855396126960_u64,5623244141930517410_u64,406101921200328260_u64,2778228890784023960_u64,3300411799215463443_u64,18195362196063255353_u64,8765992131996400215_u64];
_9 = _6 * _6;
_19 = [2972543856969599312_u64,957398333394232039_u64,9234165497266938738_u64,7968498005356595912_u64,5637577832713771335_u64,16215334563933098287_u64,10309073834570814279_u64,5729823054267314817_u64];
_16 = _20;
_21 = [13273241189345854288_u64,1484454738258320160_u64,14834776426036959488_u64,13670362859060467408_u64,12476346544663937475_u64];
_6 = -_9;
_5 = RET;
_13 = !338759560790510652418436157666620374368_u128;
_4 = [18054726842782490175_u64,5795662458742787320_u64,2141539053790582163_u64,3933395473888257559_u64,4321233324594421339_u64,16195974138364215060_u64,16481940422511810396_u64,6846746641969590751_u64];
RET = _3;
_9 = _8 as f64;
_21 = [9768111354731527529_u64,11096263911764435016_u64,15742853429010609199_u64,18346181130198207027_u64,6908796998967401083_u64];
_12 = -9223372036854775807_isize;
_18 = true;
_1 = [15159276087047388407_u64,7568297026706884315_u64,6029669864353482044_u64,527757020245381976_u64,7391465851692187208_u64,13813233904343140985_u64,18214952064167838148_u64,11776980318161775486_u64];
_1 = [2377700319445244857_u64,14399255625957884044_u64,433281740700687815_u64,467007176723082062_u64,429423477636768428_u64,10430317323731579478_u64,761645014869082383_u64,5962024032890878444_u64];
_19 = [1799716558900236107_u64,13710567145591083173_u64,9984752685139285928_u64,14326372760305087090_u64,6208651868386701550_u64,8610371395219097999_u64,16691337362786948988_u64,3789898316196201668_u64];
Goto(bb9)
}
bb9 = {
_9 = -_6;
_3 = _15;
_10 = [19225_i16,(-32744_i16),(-13560_i16)];
_20 = _16;
_16 = -_20;
_15 = RET;
_16 = !_20;
RET = _5;
_21 = [16316701555600468237_u64,2244006527807992288_u64,12333869132610651884_u64,13107862103033728210_u64,14416830109882691769_u64];
_4 = _1;
_18 = false;
_4 = _2;
_15 = _5;
_12 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_4 = _1;
_15 = RET;
_10 = [(-9510_i16),8307_i16,14822_i16];
_5 = RET;
_1 = _19;
_8 = 65_i8;
_22 = _18 & _18;
_4 = [4013666570828488953_u64,8999256170692508282_u64,12784785257525739604_u64,6659773193299332866_u64,9350911020294014719_u64,15489212806585367012_u64,5583634926287685026_u64,5435281143783217034_u64];
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
65 => bb10,
_ => bb6
}
}
bb10 = {
_11 = [30965946270506783025719246580350106737_i128,(-108424045482618959581320203370134849043_i128),(-4447379347780044881379388132074392314_i128),11983812679098408350784173478953837128_i128];
_21 = [8688768468775142055_u64,3848946667573183180_u64,1448462759904237889_u64,7833121705309278011_u64,9563862287070485431_u64];
_15 = RET;
_23 = _6;
_11 = [(-151946502558421822869113533253523970670_i128),86443816352996015059843877427114032650_i128,34409002616030674835258988335480461552_i128,(-141336252662027110147521093075997728353_i128)];
_8 = !60_i8;
_18 = _22;
Goto(bb11)
}
bb11 = {
_15 = _3;
_27 = _18;
_11 = [150521428085839090505555364442399029398_i128,(-25272760207711349915158118286150456539_i128),87827527649832445115731664354127666942_i128,(-29979542225661992427834938411479939240_i128)];
_4 = _2;
_6 = _23;
_20 = -_16;
_27 = _18 ^ _22;
_1 = [10723115391272676416_u64,15180324779884771508_u64,1922559435295592148_u64,7384095901609234471_u64,8793383896147457656_u64,17337877304443909579_u64,8216738093135316859_u64,5478864265306607653_u64];
_25 = [248_u8,161_u8];
_26 = _7;
_3 = RET;
_7 = _26;
_26 = 4170350743_u32 as f32;
_8 = (-86_i8);
_26 = _7;
_12 = (-8925579464142426549_i64) as isize;
_9 = _6;
_27 = _22 ^ _18;
_27 = _18;
_22 = _18 | _27;
_22 = !_27;
_29 = [1726942778229174239_u64,6052476263651559472_u64,3773195080115838771_u64,7414246156791315460_u64,13099028936873434299_u64,9496532997994640474_u64,18181604181169159094_u64,14821528299383658742_u64];
_19 = [8052386373601711476_u64,9032903972020131003_u64,10840035299593340636_u64,7726930033901321522_u64,7135824395216242153_u64,4704318546215818110_u64,1052995054558073128_u64,4395654710165643268_u64];
_22 = _27;
_9 = _6;
_29 = [15767430296637180259_u64,8173370259090767912_u64,7880135851521103368_u64,10335672779454380406_u64,8663838944478944_u64,17808405833407949040_u64,14587213440374268469_u64,8173299263599147286_u64];
Goto(bb12)
}
bb12 = {
Call(_8 = core::intrinsics::bswap((-71_i8)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = [65453_u16,5777_u16];
_18 = !_27;
_6 = 11200413856806926494_u64 as f64;
_32 = [56570_u16,55513_u16];
_20 = _16;
_19 = [4363939309599703423_u64,968520714042484544_u64,3874955502208010389_u64,10842636372470532063_u64,54283812991369005_u64,9895724461507085989_u64,17577914211292248272_u64,3867720381125511614_u64];
_15 = _5;
_20 = _16;
_4 = [7849075224978918399_u64,13476641157111164805_u64,3751966761146985293_u64,11580349472920000842_u64,13959656906031595754_u64,9555325947146471368_u64,6405219295298921396_u64,13042051176980763323_u64];
_31 = _7 as isize;
_2 = [11079186652923615038_u64,18195694063955243631_u64,6232297333419777563_u64,2477411285544477471_u64,8577816389315157409_u64,10200824475545112982_u64,10549256061245701168_u64,647066424385666606_u64];
_12 = _31;
_3 = _5;
_9 = _16 as f64;
RET = _5;
_5 = _3;
_27 = _18;
_5 = RET;
_30 = _22;
_1 = [6762401247958702404_u64,1465567936355678581_u64,17138076065296244962_u64,11381111847409078320_u64,3463013009991937826_u64,9148304735024099353_u64,8113500626422976593_u64,6600160976707078079_u64];
RET = _5;
Goto(bb14)
}
bb14 = {
_35 = _10;
_32 = [57054_u16,45550_u16];
_31 = 11819101100097174482_usize as isize;
_5 = _3;
_15 = RET;
_33 = _11;
_26 = _8 as f32;
_37 = _26 - _7;
_23 = _6 + _6;
_11 = [79382270911518950922497540565939956083_i128,162051422667623043395957722107208855087_i128,8227595092318167209552277311273547499_i128,129932458695070360629516947736102149432_i128];
_31 = !_12;
_36 = -(-6822476451171462898_i64);
_16 = -_20;
_10 = [(-19825_i16),4597_i16,(-19749_i16)];
_34 = _12 & _12;
_5 = _15;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(3_usize, 25_usize, Move(_25), 18_usize, Move(_18), 27_usize, Move(_27), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(3_usize, 29_usize, Move(_29), 2_usize, Move(_2), 10_usize, Move(_10), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(3_usize, 5_usize, Move(_5), 8_usize, Move(_8), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(3_usize, 32_usize, Move(_32), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: char,mut _3: [u64; 8],mut _4: [u64; 8],mut _5: [u64; 8],mut _6: char,mut _7: char,mut _8: [u64; 8],mut _9: [u64; 8]) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _10: [i128; 2];
let _11: f32;
let _12: [u16; 2];
let _13: [u16; 2];
let _14: *mut u16;
let _15: Adt36;
let _16: [usize; 8];
let _17: usize;
let _18: i32;
let _19: u8;
let _20: Adt26;
let _21: *mut i8;
let _22: (*const i64, Adt47);
let _23: i16;
let _24: f64;
let _25: i16;
let _26: (u16,);
let _27: usize;
let _28: ();
let _29: ();
{
_1 = _6;
RET = _4;
_6 = _7;
RET = _3;
_5 = RET;
_10 = [30553945238599349475819964829013616096_i128,(-150422177422875664047906809795424170744_i128)];
_5 = [17125811455206111679_u64,9961031086889019534_u64,13773282531569679248_u64,15824609061059745893_u64,8824536572397123012_u64,4727250909523959531_u64,3943751184203112164_u64,970880146346980305_u64];
RET = _9;
_3 = RET;
RET = [10931817243895643936_u64,2044219666224881691_u64,12606197536180568912_u64,10668135701678556448_u64,16969136809845004505_u64,10171517415470011001_u64,11127084587020155151_u64,7493191778420737143_u64];
_3 = [7162180479465193752_u64,47692570635536205_u64,7938354854392743940_u64,10733169453454489869_u64,15195992211858915118_u64,429288539875902701_u64,6067031099877941701_u64,4888844671154362434_u64];
_4 = _8;
RET = [6393849294330285200_u64,2058141518827544389_u64,4201541489157750781_u64,11368467626703282377_u64,17152457513324835858_u64,3243882948415328229_u64,5761220024626124396_u64,5094325550898577671_u64];
_7 = _2;
_6 = _1;
_8 = RET;
_9 = [9328678134685240718_u64,865320190206559305_u64,9924765254137150980_u64,16790860451836802697_u64,7157676360405369731_u64,8915908774617937480_u64,4826259423235653555_u64,93913266684017335_u64];
_9 = [12055967589373292926_u64,7755033291908709735_u64,15740906797002150069_u64,6025888314928741259_u64,1831427441024322773_u64,10768859238338515047_u64,11345114817023718088_u64,7869057542062277797_u64];
_9 = [11283616858960201529_u64,14278675763027278211_u64,11997699672942608070_u64,2730922691407990661_u64,6475480040451418776_u64,7581104490837558238_u64,7407088390214792907_u64,6237217826807576938_u64];
_9 = [3624673779130895762_u64,12653636904271523045_u64,1986244230809060780_u64,9747765723982027816_u64,15103790657747557966_u64,4739137906185182808_u64,8169677293515560904_u64,16652023319368310909_u64];
_2 = _7;
Call(_9 = fn5(_2, _1, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 1401746394_i32 as f32;
_5 = _3;
_8 = [7559952632069165320_u64,9602975135762771921_u64,9249244450980053325_u64,10474255870280764101_u64,1423997522336234325_u64,2243636986261234357_u64,8133999503654871824_u64,16018818762339842468_u64];
_12 = [29969_u16,20966_u16];
Goto(bb2)
}
bb2 = {
_8 = [17567042585119189765_u64,10957960575467270965_u64,8023831048901091909_u64,17568336385369929645_u64,13933156698329484048_u64,9941329483407850491_u64,5436899496809225478_u64,13676048536951289160_u64];
_7 = _1;
_8 = _9;
_13 = [43741_u16,4929_u16];
_7 = _6;
_6 = _2;
_8 = [10284012457316807136_u64,16667774301341948604_u64,1368743562891987244_u64,6772404467067709339_u64,4244822590312873446_u64,17351229834820932051_u64,8671115497873324776_u64,9912755947227428345_u64];
RET = [6573457434589850796_u64,10168056611475952448_u64,8915587599759547744_u64,653411467242179014_u64,141680079360565114_u64,12437711406717867982_u64,15240520060946982001_u64,3284633009179394144_u64];
_8 = _9;
RET = [4266899929514359366_u64,17194716370594337109_u64,6198841654722918347_u64,17088723362068352637_u64,8855232551899378305_u64,4598133778248230754_u64,11053701413692310484_u64,7601683161485993809_u64];
_2 = _6;
Goto(bb3)
}
bb3 = {
_11 = (-7780166610350906701_i64) as f32;
_2 = _1;
_3 = [12638708749501306105_u64,14105506551706756955_u64,11515919439053282167_u64,17378502526370317803_u64,662288770510999832_u64,1538176139874720595_u64,13816263800621466786_u64,63791999751393141_u64];
_11 = 12604149608238259858_u64 as f32;
_9 = [5798453390634441841_u64,2621985313236105444_u64,14362916625575518105_u64,12349761619396018022_u64,2382245029410794471_u64,3930826845162476518_u64,16481768558909928538_u64,8102370814811483785_u64];
_6 = _1;
_12 = _13;
_10 = [(-152178016093502973083128316004196095286_i128),(-161726575788547340289892141017487054860_i128)];
_3 = [14499144290924863402_u64,400641591185687785_u64,8215470655503498090_u64,7713606184130060647_u64,7939351541737156600_u64,2829796250015474068_u64,6341081153210920986_u64,12388294643831288050_u64];
_5 = _4;
Goto(bb4)
}
bb4 = {
_3 = [4058895004961024073_u64,269653236626668027_u64,14448453554404884632_u64,5415848693955615658_u64,16914488263282901687_u64,3813650937564299616_u64,1252600370137411701_u64,14877458452194264815_u64];
RET = [14115355629229304789_u64,4140757399517800303_u64,6648512090220484843_u64,16506631877832118343_u64,10433980379344954293_u64,13880313340373898448_u64,5750722639393650111_u64,11204892894654043582_u64];
_3 = [17122414547500869290_u64,2729273284852073042_u64,18301340094622308639_u64,18243218808795237290_u64,12896056861534450482_u64,2223861572802934175_u64,6077719799678855079_u64,13341271823855305899_u64];
RET = _3;
_9 = [6943080784172532267_u64,6995661237285343964_u64,15781805947904839226_u64,3636078047088862021_u64,16296285685261500256_u64,40538420060976728_u64,3950033393784006411_u64,14880568085050731071_u64];
_8 = [6960144739193420453_u64,14924623312828020535_u64,10778353194573097151_u64,17031785721784111440_u64,7860771455494392188_u64,2152894175630666014_u64,12674749307405215770_u64,6822611210109189994_u64];
RET = [707447867841843372_u64,7303962686029541542_u64,11095360619017856565_u64,120101042218137400_u64,15960092684311450091_u64,10594792785733834734_u64,13635946586118758480_u64,8403740230718895473_u64];
_16 = [2_usize,7_usize,14125645012673671485_usize,10819075081533673681_usize,3_usize,5_usize,1_usize,5502452400842384648_usize];
_10 = [6935175095231669582158512363324084383_i128,(-153001197655455410825732417278506023554_i128)];
_8 = [16298153626718935288_u64,14713003326949760599_u64,9644200729541819815_u64,9490269710030894789_u64,6841939557634257770_u64,4086871566359218961_u64,2169044021496540846_u64,8029034757080937846_u64];
_16 = [16442546673446887164_usize,13419152552894884420_usize,0_usize,5338864293784249072_usize,2_usize,9697807533418577520_usize,3360495733876111051_usize,3_usize];
_10 = [(-107229402431287076373539879054865880842_i128),(-152282939436361389486108683002191375700_i128)];
_16 = [3_usize,16405528174812599988_usize,2_usize,8828141155072869943_usize,0_usize,16667406743119140644_usize,9774594678712011826_usize,16893279569840740680_usize];
_18 = (-1230296168_i32) & 205801672_i32;
_10 = [(-19014757503826489313886877488180578546_i128),154960332514585639448048604565470531162_i128];
_4 = [143615888156858431_u64,5890999105446986457_u64,15228180706241887190_u64,7035885181387979936_u64,9981956986794126944_u64,16753924318363201015_u64,2036485075596779967_u64,9317416581443313383_u64];
_12 = _13;
_12 = [40598_u16,20501_u16];
RET = [7075415829259790206_u64,3129438929255631295_u64,2079645646117355896_u64,15660046116984198962_u64,1800160945847373956_u64,18180905108158898184_u64,11899508525253862677_u64,9701316029449022957_u64];
RET = [606277531955718807_u64,10735001339247513508_u64,1704914389396617585_u64,17291746137052817509_u64,14757116664458265987_u64,2571493469269491505_u64,4423366889601510928_u64,8682220511536447961_u64];
_1 = _2;
_6 = _1;
_18 = (-100363249990112632254939607999465045122_i128) as i32;
_5 = RET;
_9 = [3823434200833516836_u64,16258310056029433437_u64,12318498533397553913_u64,17359566528310808632_u64,15697784619123671949_u64,620483253967099441_u64,3952209008695218913_u64,17737218187019364790_u64];
_12 = [6114_u16,54857_u16];
_3 = _9;
Goto(bb5)
}
bb5 = {
_20 = Adt26::Variant2 { fld0: 6581305186536630556_u64,fld1: 245721398_u32,fld2: 9223372036854775807_isize,fld3: _16,fld4: 155_u8,fld5: _11,fld6: (-146979117362129263_i64),fld7: 154444004133573643798475478505063429270_u128 };
place!(Field::<u64>(Variant(_20, 2), 0)) = 1535591392622889042_u64;
place!(Field::<[usize; 8]>(Variant(_20, 2), 3)) = _16;
place!(Field::<u128>(Variant(_20, 2), 7)) = 3736488490_u32 as u128;
RET = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
place!(Field::<isize>(Variant(_20, 2), 2)) = (-9223372036854775808_isize);
_4 = _3;
place!(Field::<[usize; 8]>(Variant(_20, 2), 3)) = [7_usize,709920363820009102_usize,3_usize,4157958639849967560_usize,6_usize,2_usize,10362697125483066831_usize,7157506118128889619_usize];
_19 = 2896202635753331500_usize as u8;
place!(Field::<u64>(Variant(_20, 2), 0)) = (-166578257221009422026967593319840169399_i128) as u64;
place!(Field::<f32>(Variant(_20, 2), 5)) = 3896415691_u32 as f32;
_8 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
_8 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
RET = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
_22.1.fld0.0 = !13474165072856381714_usize;
_2 = _7;
_22.1.fld0.0 = 18378436844073789467_usize;
_3 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
_22.1.fld1 = (_1,);
match _22.1.fld0.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
18378436844073789467 => bb13,
_ => bb12
}
}
bb6 = {
_3 = [4058895004961024073_u64,269653236626668027_u64,14448453554404884632_u64,5415848693955615658_u64,16914488263282901687_u64,3813650937564299616_u64,1252600370137411701_u64,14877458452194264815_u64];
RET = [14115355629229304789_u64,4140757399517800303_u64,6648512090220484843_u64,16506631877832118343_u64,10433980379344954293_u64,13880313340373898448_u64,5750722639393650111_u64,11204892894654043582_u64];
_3 = [17122414547500869290_u64,2729273284852073042_u64,18301340094622308639_u64,18243218808795237290_u64,12896056861534450482_u64,2223861572802934175_u64,6077719799678855079_u64,13341271823855305899_u64];
RET = _3;
_9 = [6943080784172532267_u64,6995661237285343964_u64,15781805947904839226_u64,3636078047088862021_u64,16296285685261500256_u64,40538420060976728_u64,3950033393784006411_u64,14880568085050731071_u64];
_8 = [6960144739193420453_u64,14924623312828020535_u64,10778353194573097151_u64,17031785721784111440_u64,7860771455494392188_u64,2152894175630666014_u64,12674749307405215770_u64,6822611210109189994_u64];
RET = [707447867841843372_u64,7303962686029541542_u64,11095360619017856565_u64,120101042218137400_u64,15960092684311450091_u64,10594792785733834734_u64,13635946586118758480_u64,8403740230718895473_u64];
_16 = [2_usize,7_usize,14125645012673671485_usize,10819075081533673681_usize,3_usize,5_usize,1_usize,5502452400842384648_usize];
_10 = [6935175095231669582158512363324084383_i128,(-153001197655455410825732417278506023554_i128)];
_8 = [16298153626718935288_u64,14713003326949760599_u64,9644200729541819815_u64,9490269710030894789_u64,6841939557634257770_u64,4086871566359218961_u64,2169044021496540846_u64,8029034757080937846_u64];
_16 = [16442546673446887164_usize,13419152552894884420_usize,0_usize,5338864293784249072_usize,2_usize,9697807533418577520_usize,3360495733876111051_usize,3_usize];
_10 = [(-107229402431287076373539879054865880842_i128),(-152282939436361389486108683002191375700_i128)];
_16 = [3_usize,16405528174812599988_usize,2_usize,8828141155072869943_usize,0_usize,16667406743119140644_usize,9774594678712011826_usize,16893279569840740680_usize];
_18 = (-1230296168_i32) & 205801672_i32;
_10 = [(-19014757503826489313886877488180578546_i128),154960332514585639448048604565470531162_i128];
_4 = [143615888156858431_u64,5890999105446986457_u64,15228180706241887190_u64,7035885181387979936_u64,9981956986794126944_u64,16753924318363201015_u64,2036485075596779967_u64,9317416581443313383_u64];
_12 = _13;
_12 = [40598_u16,20501_u16];
RET = [7075415829259790206_u64,3129438929255631295_u64,2079645646117355896_u64,15660046116984198962_u64,1800160945847373956_u64,18180905108158898184_u64,11899508525253862677_u64,9701316029449022957_u64];
RET = [606277531955718807_u64,10735001339247513508_u64,1704914389396617585_u64,17291746137052817509_u64,14757116664458265987_u64,2571493469269491505_u64,4423366889601510928_u64,8682220511536447961_u64];
_1 = _2;
_6 = _1;
_18 = (-100363249990112632254939607999465045122_i128) as i32;
_5 = RET;
_9 = [3823434200833516836_u64,16258310056029433437_u64,12318498533397553913_u64,17359566528310808632_u64,15697784619123671949_u64,620483253967099441_u64,3952209008695218913_u64,17737218187019364790_u64];
_12 = [6114_u16,54857_u16];
_3 = _9;
Goto(bb5)
}
bb7 = {
_11 = (-7780166610350906701_i64) as f32;
_2 = _1;
_3 = [12638708749501306105_u64,14105506551706756955_u64,11515919439053282167_u64,17378502526370317803_u64,662288770510999832_u64,1538176139874720595_u64,13816263800621466786_u64,63791999751393141_u64];
_11 = 12604149608238259858_u64 as f32;
_9 = [5798453390634441841_u64,2621985313236105444_u64,14362916625575518105_u64,12349761619396018022_u64,2382245029410794471_u64,3930826845162476518_u64,16481768558909928538_u64,8102370814811483785_u64];
_6 = _1;
_12 = _13;
_10 = [(-152178016093502973083128316004196095286_i128),(-161726575788547340289892141017487054860_i128)];
_3 = [14499144290924863402_u64,400641591185687785_u64,8215470655503498090_u64,7713606184130060647_u64,7939351541737156600_u64,2829796250015474068_u64,6341081153210920986_u64,12388294643831288050_u64];
_5 = _4;
Goto(bb4)
}
bb8 = {
_8 = [17567042585119189765_u64,10957960575467270965_u64,8023831048901091909_u64,17568336385369929645_u64,13933156698329484048_u64,9941329483407850491_u64,5436899496809225478_u64,13676048536951289160_u64];
_7 = _1;
_8 = _9;
_13 = [43741_u16,4929_u16];
_7 = _6;
_6 = _2;
_8 = [10284012457316807136_u64,16667774301341948604_u64,1368743562891987244_u64,6772404467067709339_u64,4244822590312873446_u64,17351229834820932051_u64,8671115497873324776_u64,9912755947227428345_u64];
RET = [6573457434589850796_u64,10168056611475952448_u64,8915587599759547744_u64,653411467242179014_u64,141680079360565114_u64,12437711406717867982_u64,15240520060946982001_u64,3284633009179394144_u64];
_8 = _9;
RET = [4266899929514359366_u64,17194716370594337109_u64,6198841654722918347_u64,17088723362068352637_u64,8855232551899378305_u64,4598133778248230754_u64,11053701413692310484_u64,7601683161485993809_u64];
_2 = _6;
Goto(bb3)
}
bb9 = {
_11 = 1401746394_i32 as f32;
_5 = _3;
_8 = [7559952632069165320_u64,9602975135762771921_u64,9249244450980053325_u64,10474255870280764101_u64,1423997522336234325_u64,2243636986261234357_u64,8133999503654871824_u64,16018818762339842468_u64];
_12 = [29969_u16,20966_u16];
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
_3 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
_9 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
place!(Field::<u32>(Variant(_20, 2), 1)) = 544614507_u32 << Field::<u64>(Variant(_20, 2), 0);
_17 = !_22.1.fld0.0;
_17 = !_22.1.fld0.0;
place!(Field::<i64>(Variant(_20, 2), 6)) = Field::<isize>(Variant(_20, 2), 2) as i64;
RET = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
place!(Field::<[usize; 8]>(Variant(_20, 2), 3)) = _16;
match Field::<isize>(Variant(_20, 2), 2) {
340282366920938463454151235394913435648 => bb14,
_ => bb10
}
}
bb14 = {
_22.1.fld0.1 = -_18;
_8 = _4;
_10 = [112575054258758286714350730009456622505_i128,120425575956761018449944706759323234958_i128];
_13 = [29219_u16,13136_u16];
place!(Field::<f32>(Variant(_20, 2), 5)) = 64743_u16 as f32;
_25 = (-15925_i16) | 10938_i16;
_4 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
_3 = [Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0),Field::<u64>(Variant(_20, 2), 0)];
place!(Field::<u128>(Variant(_20, 2), 7)) = 57477189247154016536366263476005988303_u128 >> _22.1.fld0.1;
_22.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_20, 2), 6)));
_18 = _22.1.fld0.0 as i32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(4_usize, 6_usize, Move(_6), 3_usize, Move(_3), 16_usize, Move(_16), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(4_usize, 7_usize, Move(_7), 17_usize, Move(_17), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char,mut _2: char,mut _3: [i128; 2]) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _4: isize;
let _5: [u8; 2];
let _6: f32;
let _7: bool;
let _8: bool;
let _9: *mut &'static i16;
let _10: *const *mut i8;
let _11: Adt17;
let _12: f64;
let _13: &'static [u16; 6];
let _14: isize;
let _15: [u32; 1];
let _16: (u16,);
let _17: &'static [u16; 6];
let _18: [usize; 8];
let _19: [i128; 2];
let _20: *const i64;
let _21: *const i64;
let _22: isize;
let _23: isize;
let _24: i64;
let _25: (*const Adt36,);
let _26: i128;
let _27: [u64; 2];
let _28: isize;
let _29: [u128; 3];
let _30: *const u128;
let _31: f64;
let _32: i32;
let _33: i16;
let _34: [i128; 4];
let _35: [u64; 8];
let _36: i32;
let _37: [u128; 3];
let _38: [i128; 2];
let _39: &'static Adt17;
let _40: char;
let _41: [i128; 2];
let _42: i32;
let _43: (Adt32, (u128, f64), *const Adt36);
let _44: u16;
let _45: isize;
let _46: ();
let _47: ();
{
_2 = _1;
_1 = _2;
Call(RET = fn6(_1, _2, _3, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-6_isize) >> 186118964304137473428060868475417170139_u128;
RET = [8384372006921641892_u64,4356228657690813328_u64,11947106711982018081_u64,18115362321484643740_u64,13062849052178903044_u64,9194875958251371377_u64,15220439513738512326_u64,15440479197031402912_u64];
RET = [2477942104891135861_u64,3691895484683212857_u64,17168504668134687903_u64,18308475865277101994_u64,13839641272218361470_u64,10276892209854201453_u64,15088227245494854079_u64,66194704326204394_u64];
_1 = _2;
_7 = true;
_3 = [(-109235617714466178782778039871898016424_i128),(-130373067366085004344053376031739194912_i128)];
_8 = !_7;
RET = [2089327947206542967_u64,1299470366524045842_u64,7422617753480424687_u64,2233486169749575236_u64,8520970498383215944_u64,18262305525274891267_u64,5222129092075427930_u64,18191442900278439465_u64];
_7 = !_8;
_1 = _2;
RET = [16089534750034792423_u64,11265840558470603490_u64,716437422113862142_u64,1985984030200627472_u64,15212403222249878391_u64,17206344338274508902_u64,10786864651160545257_u64,4354646256838734831_u64];
Goto(bb2)
}
bb2 = {
_5 = [12_u8,168_u8];
_4 = 4254694845886839521_i64 as isize;
_7 = _8;
_1 = _2;
_7 = _8;
_8 = _7;
RET = [15404640219324407152_u64,15137082160802620201_u64,18253239993914221000_u64,8973068486159410147_u64,12165699239720685731_u64,2053361890787879637_u64,2547524175293015685_u64,17618285160567974466_u64];
RET = [3221829433207404886_u64,10486567542848040781_u64,14711124729083134319_u64,7788016632540820142_u64,14041424140072692923_u64,17687274326014136646_u64,6741665107554897767_u64,6190088442301191489_u64];
_11.fld0 = !(-1916384808_i32);
RET = [4371888553404285067_u64,2762810990997674229_u64,2227448251491851219_u64,11393381229348901673_u64,12793580454962377837_u64,4826684031952315957_u64,5979297463031824934_u64,8544137356117234882_u64];
_12 = 16345831691916960734_usize as f64;
_11 = Adt17 { fld0: (-1300472791_i32) };
_2 = _1;
_1 = _2;
_6 = 56806_u16 as f32;
Goto(bb3)
}
bb3 = {
_14 = 4081717511_u32 as isize;
_5 = [205_u8,132_u8];
RET = [12652444612576387457_u64,10380255662594472297_u64,6630451332182846027_u64,5716187037435666169_u64,10598053032985805031_u64,17981431131577302474_u64,5385250598338249481_u64,14072433966468366916_u64];
_14 = _4 & _4;
_12 = 505701825_u32 as f64;
_16 = (61018_u16,);
_16.0 = (-1400309249622105038_i64) as u16;
RET = [13646298451289040378_u64,10488311428953230315_u64,2125895890039776072_u64,678985907377171307_u64,18022546872584681561_u64,15947187848598208193_u64,8358336012797000792_u64,5472395590448148836_u64];
_19 = [(-100022870947521003462761537865082222100_i128),(-59483119312351914056313954707626110155_i128)];
_15 = [860833181_u32];
RET = [16546874867926449733_u64,1611599792797550855_u64,5274980517904362346_u64,10028210944582024204_u64,9080378624963277600_u64,913443175572193523_u64,6071558395497208176_u64,18262820829687141263_u64];
RET = [13309489416508883029_u64,9706028789574898199_u64,15432739556163150160_u64,5940325082620187704_u64,17157249069697746185_u64,16193228203908496547_u64,4188741087161465541_u64,7334912652896919770_u64];
match _11.fld0 {
340282366920938463463374607430467738665 => bb4,
_ => bb1
}
}
bb4 = {
_3 = _19;
_3 = [165732877765993409400723689956513346958_i128,49202420122574288698365655557629681127_i128];
_11.fld0 = _12 as i32;
_5 = [140_u8,224_u8];
_4 = 18394768532814349500_u64 as isize;
_6 = 2497697664_u32 as f32;
_1 = _2;
_23 = (-64_i8) as isize;
_1 = _2;
_22 = _14 | _14;
_21 = core::ptr::addr_of!(_24);
_21 = core::ptr::addr_of!((*_21));
_7 = _23 >= _4;
_19 = [(-92004496676634788587498141498684285630_i128),(-168383043932915417367754454925325466256_i128)];
_12 = _6 as f64;
_18 = [8465483299803036394_usize,5_usize,10120638385465677220_usize,4_usize,8552352532805297118_usize,2_usize,6084584706026943741_usize,1_usize];
RET = [7388436608253229095_u64,2216463356097564856_u64,16307935684657234145_u64,10710565646776256120_u64,14183557888775926056_u64,581359679244395527_u64,9603038049143374777_u64,6736458032591930386_u64];
(*_21) = 6507840487602293885_i64;
RET = [14512707122905099881_u64,8658687288326629584_u64,18240340895296124337_u64,11516787780752436310_u64,17786069502197262742_u64,16628725260175800218_u64,11338999625198804673_u64,8264675836277157870_u64];
_5 = [29_u8,105_u8];
RET = [10534856805329465378_u64,10523241802672570709_u64,12798254062424731739_u64,8967693706518221374_u64,6432727047321636353_u64,7675631566618492847_u64,16314150233240130411_u64,6121918858819271724_u64];
_22 = -_23;
_24 = -(-5046274953688205850_i64);
_21 = core::ptr::addr_of!((*_21));
Goto(bb5)
}
bb5 = {
_16 = (61582_u16,);
(*_21) = -(-7189023443724593211_i64);
_27 = [280024891412415205_u64,13732655845858880519_u64];
_4 = !_22;
Goto(bb6)
}
bb6 = {
_3 = [4080436096229117027740621606216934362_i128,(-87065308873437496613537041250805216316_i128)];
_24 = 4227661300712991128_i64;
_11 = Adt17 { fld0: 501129679_i32 };
_28 = (-61_i8) as isize;
_21 = core::ptr::addr_of!(_24);
RET = [17111226637126626454_u64,4401014010459815860_u64,8328290339413855866_u64,717187732245933187_u64,7475052627959154600_u64,12648828652606028369_u64,8786577511826859133_u64,14717253771844009199_u64];
_32 = _11.fld0;
_22 = _14 >> _23;
_31 = _12 * _12;
_23 = -_22;
_8 = _7 & _7;
Goto(bb7)
}
bb7 = {
_20 = Move(_21);
_27 = [5357475944101744028_u64,103133019659032624_u64];
_21 = Move(_20);
_28 = _4 * _22;
_24 = -2719153132174434066_i64;
_11 = Adt17 { fld0: _32 };
_27 = [15647302528873717734_u64,3526491805598710060_u64];
_24 = (-1760308059934801935_i64) * 7022386282774616043_i64;
_22 = !_28;
_18 = [7_usize,1_usize,1_usize,4_usize,8974785157934314030_usize,7884636190741218035_usize,0_usize,5779673320433137656_usize];
_4 = _14;
_12 = 123_i8 as f64;
_21 = core::ptr::addr_of!(_24);
(*_21) = (-8826911894897367121_i64);
_18 = [2_usize,2_usize,5_usize,8774973336450207791_usize,5_usize,3_usize,7228387923533360784_usize,13815174216652763867_usize];
_12 = _31;
RET = [550319032532181119_u64,1621522992351684677_u64,6455026077445354455_u64,9212005316660568440_u64,12283529516360234086_u64,13145646715764291041_u64,6996668035769196023_u64,8338869874846791495_u64];
RET = [15109733012290866853_u64,11590425148936932890_u64,15398847239354976601_u64,5147074129255484785_u64,18044914919996328761_u64,7862214227511433111_u64,17123755193795916936_u64,17164513938062404577_u64];
_12 = 1753496304_u32 as f64;
_6 = _16.0 as f32;
_15 = [2906526513_u32];
_27 = [16391703092052858891_u64,17413839103700336698_u64];
_20 = core::ptr::addr_of!((*_21));
(*_21) = (-3285426664717385974_i64) & 5137159935903514634_i64;
RET = [7112480833750025886_u64,14072363585215320996_u64,17528328806941474591_u64,1982764724253968208_u64,10506150504004549641_u64,5109578321531228630_u64,18043050527205869680_u64,7279823097137004379_u64];
match _11.fld0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
501129679 => bb9,
_ => bb8
}
}
bb8 = {
_3 = [4080436096229117027740621606216934362_i128,(-87065308873437496613537041250805216316_i128)];
_24 = 4227661300712991128_i64;
_11 = Adt17 { fld0: 501129679_i32 };
_28 = (-61_i8) as isize;
_21 = core::ptr::addr_of!(_24);
RET = [17111226637126626454_u64,4401014010459815860_u64,8328290339413855866_u64,717187732245933187_u64,7475052627959154600_u64,12648828652606028369_u64,8786577511826859133_u64,14717253771844009199_u64];
_32 = _11.fld0;
_22 = _14 >> _23;
_31 = _12 * _12;
_23 = -_22;
_8 = _7 & _7;
Goto(bb7)
}
bb9 = {
_26 = (-169718712705946936170552644400501476878_i128);
_21 = Move(_20);
_20 = Move(_21);
RET = [1338819529870439899_u64,1348240394580386331_u64,204905235142848125_u64,11694517542588632985_u64,6820586841294997757_u64,14096789884043151955_u64,9404941208274958977_u64,13017731425426621644_u64];
_11 = Adt17 { fld0: _32 };
_1 = _2;
_6 = 3_usize as f32;
_20 = core::ptr::addr_of!(_24);
(*_20) = 5724655550914958310_i64 + (-5731883213855958889_i64);
_21 = core::ptr::addr_of!(_24);
_5 = [28_u8,235_u8];
_27 = [15059157595419836953_u64,7175598686472666908_u64];
_14 = !_22;
Goto(bb10)
}
bb10 = {
_28 = _14;
_16 = (59488_u16,);
_16.0 = 7083_u16;
(*_21) = -(-2459692507336391221_i64);
_34 = [_26,_26,_26,_26];
RET = [14786087907193367889_u64,7673124266039130194_u64,505173496277011913_u64,18110771061560984316_u64,810466967223148766_u64,11156812895749224261_u64,6619533258159238076_u64,3566368935653507356_u64];
_28 = _22 + _22;
_33 = 25925_i16 >> _22;
(*_20) = -(-1073058793431314262_i64);
match _32 {
0 => bb6,
1 => bb3,
2 => bb11,
3 => bb12,
4 => bb13,
501129679 => bb15,
_ => bb14
}
}
bb11 = {
_5 = [12_u8,168_u8];
_4 = 4254694845886839521_i64 as isize;
_7 = _8;
_1 = _2;
_7 = _8;
_8 = _7;
RET = [15404640219324407152_u64,15137082160802620201_u64,18253239993914221000_u64,8973068486159410147_u64,12165699239720685731_u64,2053361890787879637_u64,2547524175293015685_u64,17618285160567974466_u64];
RET = [3221829433207404886_u64,10486567542848040781_u64,14711124729083134319_u64,7788016632540820142_u64,14041424140072692923_u64,17687274326014136646_u64,6741665107554897767_u64,6190088442301191489_u64];
_11.fld0 = !(-1916384808_i32);
RET = [4371888553404285067_u64,2762810990997674229_u64,2227448251491851219_u64,11393381229348901673_u64,12793580454962377837_u64,4826684031952315957_u64,5979297463031824934_u64,8544137356117234882_u64];
_12 = 16345831691916960734_usize as f64;
_11 = Adt17 { fld0: (-1300472791_i32) };
_2 = _1;
_1 = _2;
_6 = 56806_u16 as f32;
Goto(bb3)
}
bb12 = {
_14 = 4081717511_u32 as isize;
_5 = [205_u8,132_u8];
RET = [12652444612576387457_u64,10380255662594472297_u64,6630451332182846027_u64,5716187037435666169_u64,10598053032985805031_u64,17981431131577302474_u64,5385250598338249481_u64,14072433966468366916_u64];
_14 = _4 & _4;
_12 = 505701825_u32 as f64;
_16 = (61018_u16,);
_16.0 = (-1400309249622105038_i64) as u16;
RET = [13646298451289040378_u64,10488311428953230315_u64,2125895890039776072_u64,678985907377171307_u64,18022546872584681561_u64,15947187848598208193_u64,8358336012797000792_u64,5472395590448148836_u64];
_19 = [(-100022870947521003462761537865082222100_i128),(-59483119312351914056313954707626110155_i128)];
_15 = [860833181_u32];
RET = [16546874867926449733_u64,1611599792797550855_u64,5274980517904362346_u64,10028210944582024204_u64,9080378624963277600_u64,913443175572193523_u64,6071558395497208176_u64,18262820829687141263_u64];
RET = [13309489416508883029_u64,9706028789574898199_u64,15432739556163150160_u64,5940325082620187704_u64,17157249069697746185_u64,16193228203908496547_u64,4188741087161465541_u64,7334912652896919770_u64];
match _11.fld0 {
340282366920938463463374607430467738665 => bb4,
_ => bb1
}
}
bb13 = {
_20 = Move(_21);
_27 = [5357475944101744028_u64,103133019659032624_u64];
_21 = Move(_20);
_28 = _4 * _22;
_24 = -2719153132174434066_i64;
_11 = Adt17 { fld0: _32 };
_27 = [15647302528873717734_u64,3526491805598710060_u64];
_24 = (-1760308059934801935_i64) * 7022386282774616043_i64;
_22 = !_28;
_18 = [7_usize,1_usize,1_usize,4_usize,8974785157934314030_usize,7884636190741218035_usize,0_usize,5779673320433137656_usize];
_4 = _14;
_12 = 123_i8 as f64;
_21 = core::ptr::addr_of!(_24);
(*_21) = (-8826911894897367121_i64);
_18 = [2_usize,2_usize,5_usize,8774973336450207791_usize,5_usize,3_usize,7228387923533360784_usize,13815174216652763867_usize];
_12 = _31;
RET = [550319032532181119_u64,1621522992351684677_u64,6455026077445354455_u64,9212005316660568440_u64,12283529516360234086_u64,13145646715764291041_u64,6996668035769196023_u64,8338869874846791495_u64];
RET = [15109733012290866853_u64,11590425148936932890_u64,15398847239354976601_u64,5147074129255484785_u64,18044914919996328761_u64,7862214227511433111_u64,17123755193795916936_u64,17164513938062404577_u64];
_12 = 1753496304_u32 as f64;
_6 = _16.0 as f32;
_15 = [2906526513_u32];
_27 = [16391703092052858891_u64,17413839103700336698_u64];
_20 = core::ptr::addr_of!((*_21));
(*_21) = (-3285426664717385974_i64) & 5137159935903514634_i64;
RET = [7112480833750025886_u64,14072363585215320996_u64,17528328806941474591_u64,1982764724253968208_u64,10506150504004549641_u64,5109578321531228630_u64,18043050527205869680_u64,7279823097137004379_u64];
match _11.fld0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
501129679 => bb9,
_ => bb8
}
}
bb14 = {
_3 = [4080436096229117027740621606216934362_i128,(-87065308873437496613537041250805216316_i128)];
_24 = 4227661300712991128_i64;
_11 = Adt17 { fld0: 501129679_i32 };
_28 = (-61_i8) as isize;
_21 = core::ptr::addr_of!(_24);
RET = [17111226637126626454_u64,4401014010459815860_u64,8328290339413855866_u64,717187732245933187_u64,7475052627959154600_u64,12648828652606028369_u64,8786577511826859133_u64,14717253771844009199_u64];
_32 = _11.fld0;
_22 = _14 >> _23;
_31 = _12 * _12;
_23 = -_22;
_8 = _7 & _7;
Goto(bb7)
}
bb15 = {
_5 = [107_u8,155_u8];
(*_20) = _33 as i64;
_31 = _32 as f64;
_11.fld0 = _32;
_38 = _19;
RET = [18400375744876130870_u64,8628847160573303814_u64,497832041846364248_u64,15322516661214262373_u64,10787207429507147912_u64,16990954106424489662_u64,10308135283022258011_u64,9696703499758152961_u64];
_36 = _11.fld0;
_16.0 = _8 as u16;
_37 = [245766111641395202719027934742534916017_u128,220906251659614091118954609035174849274_u128,228487222430073116161039925973947597964_u128];
_24 = _31 as i64;
_7 = _8;
_29 = [14824825692820273627280409810377493732_u128,333363459562669641907453162891989691972_u128,254044163562217033770418722859462858936_u128];
_11.fld0 = _32 & _36;
_5 = [179_u8,229_u8];
_39 = &_11;
_19 = [_26,_26];
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(5_usize, 15_usize, Move(_15), 22_usize, Move(_22), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(5_usize, 7_usize, Move(_7), 33_usize, Move(_33), 32_usize, Move(_32), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(5_usize, 4_usize, Move(_4), 36_usize, Move(_36), 5_usize, Move(_5), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: char,mut _2: char,mut _3: [i128; 2],mut _4: char,mut _5: char) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _6: usize;
let _7: i64;
let _8: bool;
let _9: *const Adt61;
let _10: char;
let _11: &'static i16;
let _12: Adt47;
let _13: *const u32;
let _14: &'static [i16; 3];
let _15: ((char, [i16; 3]), [u64; 8], i32, f32);
let _16: *const (u16, isize);
let _17: u64;
let _18: f32;
let _19: (u16, isize);
let _20: f64;
let _21: [u16; 6];
let _22: u8;
let _23: [i128; 4];
let _24: &'static [usize; 8];
let _25: *const Adt61;
let _26: f32;
let _27: f64;
let _28: [u16; 6];
let _29: (char, [i16; 3]);
let _30: Adt26;
let _31: *const usize;
let _32: [u16; 6];
let _33: [i128; 4];
let _34: u64;
let _35: &'static i16;
let _36: [u64; 8];
let _37: u128;
let _38: char;
let _39: (char,);
let _40: *mut &'static Adt17;
let _41: ();
let _42: ();
{
_1 = _4;
RET = [14046979643882428169_u64,9399480406631364537_u64,4854137411353027570_u64,12457163511922437142_u64,17432481417351649341_u64,11641286491910748602_u64,9022457496640585763_u64,14488365577289683389_u64];
_2 = _4;
_5 = _4;
_5 = _4;
_7 = 5817595035881774695_i64 * (-641827542626838423_i64);
_7 = !5993630081652127472_i64;
_6 = 5_usize;
_1 = _5;
_4 = _1;
_2 = _5;
_7 = !(-1727712340266527679_i64);
Call(_7 = fn7(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _1;
_1 = _5;
_5 = _4;
_6 = 1_usize;
RET = [15330563278604629181_u64,9065285345136651067_u64,10082584051743241214_u64,7479395119449815032_u64,7615931247376279594_u64,6438677552253068197_u64,11214907075403352698_u64,9339193384626956508_u64];
RET[_6] = 3508132702466615746_u64 - 14407908129989743658_u64;
RET = [17449858165194951180_u64,17253374873911860156_u64,11781204256300546620_u64,18110920030593886330_u64,3377667803573174475_u64,17165805724434269747_u64,14696401866097348646_u64,2258696537671342855_u64];
_4 = _5;
_6 = 2236643414_u32 as usize;
_8 = _4 != _4;
_5 = _2;
RET = [3612886971071876503_u64,5214968988386171927_u64,1616874930718057443_u64,1438120136950845869_u64,436634840877352039_u64,6931816165191895659_u64,572936353216537931_u64,17169294987217868944_u64];
_7 = -(-1915707786877293148_i64);
_1 = _4;
_1 = _2;
_5 = _1;
RET = [14566667713538525551_u64,14414190740126782736_u64,12123392683824280992_u64,10081760305920068875_u64,16504112847256057837_u64,1741217724400583261_u64,1843538350582168915_u64,17023461400591634637_u64];
Goto(bb2)
}
bb2 = {
_4 = _1;
_2 = _5;
_6 = !5_usize;
_4 = _1;
Goto(bb3)
}
bb3 = {
_2 = _1;
RET = [12786609531696386060_u64,5683185388899425050_u64,5336929597326008607_u64,3461622773432338164_u64,16136220506155145584_u64,5097161397161538134_u64,7742210292795443802_u64,17444113726809836615_u64];
_7 = _2 as i64;
_12.fld0.0 = _6;
_12.fld0 = (_6, 964850104_i32);
_12.fld0.0 = _6;
_12.fld1 = (_2,);
_8 = true | true;
_12.fld0.0 = !_6;
_6 = _12.fld0.0;
RET = [12651716023978398696_u64,13822531831831870614_u64,438265977489311505_u64,8824991260129404138_u64,4458779415741276252_u64,15123928305952578451_u64,14899382503263487294_u64,4156034689108523806_u64];
_12.fld0 = (_6, (-1839225495_i32));
_12.fld0.0 = _6 + _6;
_12.fld1 = (_2,);
_6 = _12.fld0.0 ^ _12.fld0.0;
_12.fld0 = (_6, 824072493_i32);
_12.fld0 = (_6, 397101455_i32);
_1 = _12.fld1.0;
_12.fld0.0 = _6 << _7;
_7 = 91_i8 as i64;
_10 = _4;
_12.fld0 = (_6, 1953740177_i32);
_4 = _12.fld1.0;
match _12.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
1953740177 => bb7,
_ => bb6
}
}
bb4 = {
_4 = _1;
_2 = _5;
_6 = !5_usize;
_4 = _1;
Goto(bb3)
}
bb5 = {
_5 = _1;
_1 = _5;
_5 = _4;
_6 = 1_usize;
RET = [15330563278604629181_u64,9065285345136651067_u64,10082584051743241214_u64,7479395119449815032_u64,7615931247376279594_u64,6438677552253068197_u64,11214907075403352698_u64,9339193384626956508_u64];
RET[_6] = 3508132702466615746_u64 - 14407908129989743658_u64;
RET = [17449858165194951180_u64,17253374873911860156_u64,11781204256300546620_u64,18110920030593886330_u64,3377667803573174475_u64,17165805724434269747_u64,14696401866097348646_u64,2258696537671342855_u64];
_4 = _5;
_6 = 2236643414_u32 as usize;
_8 = _4 != _4;
_5 = _2;
RET = [3612886971071876503_u64,5214968988386171927_u64,1616874930718057443_u64,1438120136950845869_u64,436634840877352039_u64,6931816165191895659_u64,572936353216537931_u64,17169294987217868944_u64];
_7 = -(-1915707786877293148_i64);
_1 = _4;
_1 = _2;
_5 = _1;
RET = [14566667713538525551_u64,14414190740126782736_u64,12123392683824280992_u64,10081760305920068875_u64,16504112847256057837_u64,1741217724400583261_u64,1843538350582168915_u64,17023461400591634637_u64];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_10 = _1;
_3 = [(-158976789045697480974619582312672540938_i128),169322883052599827404859442600628152233_i128];
_5 = _1;
_7 = 17_u8 as i64;
_8 = !false;
_7 = 7_u8 as i64;
_12.fld1 = (_5,);
_7 = 2982040812519725085_i64;
_15.0.0 = _4;
_15.3 = _12.fld0.1 as f32;
_5 = _4;
_12.fld1 = (_10,);
_4 = _15.0.0;
_12.fld0 = (_6, 1107967195_i32);
_15.1 = [12266409002208171891_u64,7977550290011465037_u64,6964395084968350448_u64,9846479800671497009_u64,3903299543982342594_u64,15639540083420246585_u64,2759351767964128763_u64,15732253038088886083_u64];
_4 = _1;
RET = [5251800631261983574_u64,9924222591382842271_u64,4669121612384402648_u64,14730227996495059687_u64,1758905145468238174_u64,17490976420719904834_u64,8873401174940852209_u64,12681093207896876214_u64];
_12.fld0.1 = _4 as i32;
_6 = _12.fld0.0 << _12.fld0.1;
_12.fld0.1 = 330613601_i32 + (-1329365653_i32);
_12.fld0 = (_6, (-1705247837_i32));
_2 = _15.0.0;
_19 = (1546_u16, 9223372036854775807_isize);
_16 = core::ptr::addr_of!(_19);
(*_16).1 = 119_isize & 126_isize;
RET = _15.1;
_19.0 = !30139_u16;
Goto(bb8)
}
bb8 = {
_7 = (-6944_i16) as i64;
(*_16).0 = 39652_u16 << _12.fld0.0;
_19.0 = _12.fld0.1 as u16;
_19.0 = 1777542705_u32 as u16;
RET = [9812195575377890445_u64,17924546162389725884_u64,2994028888818268757_u64,9733576765270817017_u64,16259544243390218313_u64,9882597949999482444_u64,10102162930101956113_u64,16826414534646848849_u64];
_14 = &_15.0.1;
_15.0.1 = [32400_i16,(-29026_i16),(-1307_i16)];
_17 = 15878356157712208431_u64;
_20 = _12.fld0.1 as f64;
_12.fld1 = (_10,);
_7 = (-2333950291777729794_i64);
_15.3 = _12.fld0.1 as f32;
_19 = (13790_u16, 9223372036854775807_isize);
_3 = [(-49662685704513136911777074275830148928_i128),(-36965387011274414718966150587578272359_i128)];
_6 = !_12.fld0.0;
(*_16).0 = 1944_u16;
_21 = [(*_16).0,_19.0,(*_16).0,_19.0,_19.0,_19.0];
Goto(bb9)
}
bb9 = {
_15.0.1 = [(-20630_i16),26156_i16,(-30193_i16)];
_1 = _4;
(*_16) = (5222_u16, (-43_isize));
_12.fld1.0 = _15.0.0;
_22 = 18_u8 & 107_u8;
_12.fld0.0 = !_6;
_15.0.0 = _10;
_18 = -_15.3;
(*_16).1 = 285021671857334709467919198468691971523_u128 as isize;
_12.fld2 = Move(_16);
_15.2 = !_12.fld0.1;
_19.0 = 10520_i16 as u16;
_20 = 1196441651270524757022283502251973512_i128 as f64;
_15.0.0 = _5;
_10 = _4;
_29.0 = _4;
_26 = _15.3 - _18;
_29.1 = [1910_i16,(-11549_i16),12599_i16];
_14 = &_15.0.1;
_21 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_16 = core::ptr::addr_of!(_19);
_6 = _12.fld0.0 | _12.fld0.0;
(*_16) = (28190_u16, (-9223372036854775808_isize));
(*_16).0 = 43841_u16;
_16 = core::ptr::addr_of!((*_16));
Goto(bb10)
}
bb10 = {
_12.fld0 = (_6, _15.2);
RET = _15.1;
_15.1 = [_17,_17,_17,_17,_17,_17,_17,_17];
_15 = (_29, RET, _12.fld0.1, _26);
_2 = _12.fld1.0;
_20 = _7 as f64;
_14 = &_29.1;
_15.0 = _29;
(*_16).1 = !30_isize;
_15.0.1 = [3458_i16,(-9050_i16),9638_i16];
_2 = _15.0.0;
_15.0.0 = _4;
_19.1 = 5652691611787295418305989173067450990_i128 as isize;
_17 = 1711338074109801936_u64;
_15.2 = _12.fld0.1 << _12.fld0.0;
_12.fld0.1 = _15.2 << _12.fld0.0;
(*_16).0 = !52840_u16;
_23 = [160074085308191605798804444455598752509_i128,(-166175166113373909327203893264083434370_i128),122116329152464295319982985121822022679_i128,(-152110222969910132206367885911536867989_i128)];
match _7 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
340282366920938463461040657139990481662 => bb12,
_ => bb11
}
}
bb11 = {
_4 = _1;
_2 = _5;
_6 = !5_usize;
_4 = _1;
Goto(bb3)
}
bb12 = {
_1 = _15.0.0;
_3 = [(-52523257044939523932167896626042883169_i128),(-146734655375432245277222044605460912864_i128)];
_6 = _12.fld0.0;
_5 = _10;
_16 = core::ptr::addr_of!(_19);
_26 = _19.1 as f32;
_21 = [(*_16).0,(*_16).0,(*_16).0,(*_16).0,(*_16).0,(*_16).0];
Goto(bb13)
}
bb13 = {
_7 = (-2948446163132109519_i64) ^ (-4264639343284370899_i64);
_12.fld2 = core::ptr::addr_of!((*_16));
RET = _15.1;
_27 = _17 as f64;
_31 = core::ptr::addr_of!(_12.fld0.0);
_14 = &_15.0.1;
match _17 {
0 => bb11,
1 => bb2,
2 => bb6,
1711338074109801936 => bb15,
_ => bb14
}
}
bb14 = {
_2 = _1;
RET = [12786609531696386060_u64,5683185388899425050_u64,5336929597326008607_u64,3461622773432338164_u64,16136220506155145584_u64,5097161397161538134_u64,7742210292795443802_u64,17444113726809836615_u64];
_7 = _2 as i64;
_12.fld0.0 = _6;
_12.fld0 = (_6, 964850104_i32);
_12.fld0.0 = _6;
_12.fld1 = (_2,);
_8 = true | true;
_12.fld0.0 = !_6;
_6 = _12.fld0.0;
RET = [12651716023978398696_u64,13822531831831870614_u64,438265977489311505_u64,8824991260129404138_u64,4458779415741276252_u64,15123928305952578451_u64,14899382503263487294_u64,4156034689108523806_u64];
_12.fld0 = (_6, (-1839225495_i32));
_12.fld0.0 = _6 + _6;
_12.fld1 = (_2,);
_6 = _12.fld0.0 ^ _12.fld0.0;
_12.fld0 = (_6, 824072493_i32);
_12.fld0 = (_6, 397101455_i32);
_1 = _12.fld1.0;
_12.fld0.0 = _6 << _7;
_7 = 91_i8 as i64;
_10 = _4;
_12.fld0 = (_6, 1953740177_i32);
_4 = _12.fld1.0;
match _12.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
1953740177 => bb7,
_ => bb6
}
}
bb15 = {
_37 = !93236425485392898147928474528687685194_u128;
RET = _15.1;
_29.0 = _12.fld1.0;
_37 = !182604044388343814346500036061188738977_u128;
_39 = (_1,);
_39.0 = _2;
_37 = _7 as u128;
_7 = (-1577032321737364744_i64) ^ 5286241656077268782_i64;
_22 = !10_u8;
_34 = 2744206011_u32 as u64;
_36 = [_17,_34,_34,_34,_34,_17,_17,_34];
_31 = core::ptr::addr_of!((*_31));
_12.fld1 = (_4,);
Goto(bb16)
}
bb16 = {
Call(_41 = dump_var(6_usize, 10_usize, Move(_10), 23_usize, Move(_23), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(6_usize, 22_usize, Move(_22), 34_usize, Move(_34), 21_usize, Move(_21), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(6_usize, 36_usize, Move(_36), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> i64 {
mir! {
type RET = i64;
let _1: *const i16;
let _2: (Adt26, i16, *const usize, &'static Adt17);
let _3: char;
let _4: f64;
let _5: u64;
let _6: [u16; 6];
let _7: *const (usize, i32);
let _8: Adt36;
let _9: u128;
let _10: bool;
let _11: u16;
let _12: [u64; 2];
let _13: &'static [i16; 3];
let _14: &'static [i16; 3];
let _15: [i128; 5];
let _16: [i16; 3];
let _17: [u16; 6];
let _18: isize;
let _19: u128;
let _20: Adt47;
let _21: ((char, [i16; 3]), [u64; 8], i32, f32);
let _22: &'static [u16; 6];
let _23: &'static [i16; 3];
let _24: [u64; 2];
let _25: Adt79;
let _26: [u16; 2];
let _27: usize;
let _28: [u32; 1];
let _29: f64;
let _30: [i64; 7];
let _31: isize;
let _32: *const usize;
let _33: (bool, u64, f64, u64);
let _34: f32;
let _35: bool;
let _36: char;
let _37: [u16; 2];
let _38: &'static i8;
let _39: ();
let _40: ();
{
RET = 582737255151871908_i64 >> 31210193893039491759690335411863322736_i128;
RET = !(-7537468515916930621_i64);
RET = (-6766835775301953701_i64);
RET = 2652744945585002669_i64;
RET = (-5617585958682751451_i64) & (-2286338368253705325_i64);
RET = 5090850567524168578_i64;
RET = 1808_i16 as i64;
RET = 14876997082426293629_u64 as i64;
RET = -(-8839564117846585089_i64);
RET = 8342521363054385067_i64 & (-3862796530526441591_i64);
RET = !4760964285902603181_i64;
RET = 11204_i16 as i64;
_3 = '\u{10d2ff}';
_1 = core::ptr::addr_of!(_2.1);
Call((*_1) = fn8(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 8714412337906952097_usize as f64;
_4 = 25_i8 as f64;
(*_1) = (-6951_i16);
_2.1 = (-9742_i16) | (-11101_i16);
_3 = '\u{1ada3}';
(*_1) = (-19494_i16);
(*_1) = 25903_i16 * (-26669_i16);
(*_1) = 2192586984_u32 as i16;
(*_1) = !(-20079_i16);
_5 = !10175650906207648303_u64;
RET = !5471438679532214635_i64;
_5 = !197917653487359484_u64;
RET = -5256551635042899030_i64;
_2.1 = (-14008_i16) << _5;
_1 = core::ptr::addr_of!(_2.1);
_2.1 = !12055_i16;
Call(_4 = fn16(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = '\u{61097}';
(*_1) = -20740_i16;
_1 = core::ptr::addr_of!(_2.1);
RET = !4596002831013096199_i64;
Goto(bb3)
}
bb3 = {
_4 = 3882078361_u32 as f64;
RET = !(-6145140540714680810_i64);
_11 = 9437_u16;
_12 = [_5,_5];
_11 = 36479_u16;
_9 = !122994792340430148956528285581116928710_u128;
_10 = !false;
match _11 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
36479 => bb9,
_ => bb8
}
}
bb4 = {
_3 = '\u{61097}';
(*_1) = -20740_i16;
_1 = core::ptr::addr_of!(_2.1);
RET = !4596002831013096199_i64;
Goto(bb3)
}
bb5 = {
_4 = 8714412337906952097_usize as f64;
_4 = 25_i8 as f64;
(*_1) = (-6951_i16);
_2.1 = (-9742_i16) | (-11101_i16);
_3 = '\u{1ada3}';
(*_1) = (-19494_i16);
(*_1) = 25903_i16 * (-26669_i16);
(*_1) = 2192586984_u32 as i16;
(*_1) = !(-20079_i16);
_5 = !10175650906207648303_u64;
RET = !5471438679532214635_i64;
_5 = !197917653487359484_u64;
RET = -5256551635042899030_i64;
_2.1 = (-14008_i16) << _5;
_1 = core::ptr::addr_of!(_2.1);
_2.1 = !12055_i16;
Call(_4 = fn16(), ReturnTo(bb2), UnwindUnreachable())
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
_6 = [_11,_11,_11,_11,_11,_11];
_2.1 = -29990_i16;
_2.1 = !16163_i16;
_2.1 = -15255_i16;
_2.1 = 15337_i16;
_11 = 20318_u16 << (*_1);
_10 = (*_1) < (*_1);
_15 = [18994087904024035718492827140409585967_i128,(-8233436429321883301286160101711422384_i128),(-150759109153647785406821548341881950057_i128),(-62541532758832856369922721875021749466_i128),(-71403503393393420485390714543215328624_i128)];
_13 = &_16;
_15 = [117772028290101143653139561102189035839_i128,(-58540006356914513800625925889381113468_i128),(-8363297601111969117813067031774416737_i128),164828344876020934199854269811672752242_i128,134784888261463788623171950174879390436_i128];
match _2.1 {
0 => bb5,
1 => bb2,
15337 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_14 = &(*_13);
_19 = !_9;
_1 = core::ptr::addr_of!((*_1));
_17 = [_11,_11,_11,_11,_11,_11];
_16 = [(*_1),_2.1,(*_1)];
_3 = '\u{5d35}';
_20.fld1.0 = _3;
_20.fld0 = (2_usize, (-463156073_i32));
_20.fld1 = (_3,);
_4 = _19 as f64;
_10 = !true;
_2.2 = core::ptr::addr_of!(_20.fld0.0);
RET = 62117887984695667_i64 * 1996090595743812507_i64;
_7 = core::ptr::addr_of!(_20.fld0);
_19 = _9;
_3 = _20.fld1.0;
_21.3 = _11 as f32;
(*_7) = (0_usize, (-2757632_i32));
(*_7).1 = _10 as i32;
_20.fld0 = (2_usize, (-400013299_i32));
(*_7).1 = 89127304_i32 >> _20.fld0.0;
_21.0.1 = [(*_1),_2.1,_2.1];
_5 = _10 as u64;
_14 = &_21.0.1;
_21.0 = (_3, _16);
_9 = _19;
_21.0 = (_20.fld1.0, _16);
Goto(bb12)
}
bb12 = {
_2.1 = 10225_i16 * (-14350_i16);
_24 = [_5,_5];
_10 = true;
(*_7) = (7438390563958981025_usize, (-600320137_i32));
RET = (-8595345386292447260_i64) >> (*_7).0;
_23 = &_16;
_4 = 2126095609_u32 as f64;
_20.fld0.1 = 277426816_i32 & 108119922_i32;
_13 = &(*_23);
_20.fld1.0 = _21.0.0;
_21.2 = (*_7).1;
_16 = [(*_1),_2.1,(*_1)];
_14 = &_16;
Goto(bb13)
}
bb13 = {
(*_1) = 27938_i16 + (-23623_i16);
_21.3 = 29_isize as f32;
(*_7).0 = 6_usize & 11232714496182528016_usize;
_10 = (*_1) < _2.1;
_27 = _21.3 as usize;
_21.0.1 = (*_14);
(*_7).1 = _19 as i32;
_20.fld1 = (_21.0.0,);
_1 = core::ptr::addr_of!(_2.1);
_2.2 = core::ptr::addr_of!((*_7).0);
_4 = 112_isize as f64;
_20.fld1.0 = _3;
_2.1 = 23564_i16;
_3 = _20.fld1.0;
(*_7) = (_27, _21.2);
_2.1 = (-32_i16) + 4146_i16;
_20.fld0.0 = !_27;
(*_7) = (_27, _21.2);
_21.1 = [_5,_5,_5,_5,_5,_5,_5,_5];
_20.fld0 = (_27, _21.2);
_10 = !true;
_20.fld1.0 = _21.0.0;
_9 = !_19;
(*_1) = (-11565_i16);
(*_7).1 = _21.2 ^ _21.2;
_20.fld0.0 = _27 << (*_7).1;
RET = (-65_i8) as i64;
(*_7) = (_27, _21.2);
(*_7) = (_27, _21.2);
_3 = _21.0.0;
Goto(bb14)
}
bb14 = {
(*_7).0 = !_27;
_17 = [_11,_11,_11,_11,_11,_11];
_20.fld0.1 = !_21.2;
_13 = Move(_14);
_26 = [_11,_11];
_2.1 = (-123_i16);
RET = 6559289588650883055_i64 | (-5743948427807928381_i64);
_18 = 21_isize | (-9223372036854775808_isize);
_7 = core::ptr::addr_of!(_20.fld0);
_21.0 = (_3, _16);
(*_7).1 = (*_1) as i32;
_20.fld1.0 = _3;
(*_1) = !(-16630_i16);
_33.2 = _4 * _4;
_32 = Move(_2.2);
_28 = [1562504502_u32];
_20.fld1.0 = _3;
_14 = &_21.0.1;
_31 = !_18;
_29 = RET as f64;
_13 = Move(_14);
_23 = &_16;
_18 = _5 as isize;
_37 = _26;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(7_usize, 17_usize, Move(_17), 26_usize, Move(_26), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(7_usize, 6_usize, Move(_6), 11_usize, Move(_11), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(7_usize, 15_usize, Move(_15), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8() -> i16 {
mir! {
type RET = i16;
let _1: usize;
let _2: char;
let _3: u16;
let _4: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _5: ([u128; 3], &'static i16, i32, &'static [i16; 3]);
let _6: [u64; 2];
let _7: [u16; 2];
let _8: (usize, i32);
let _9: char;
let _10: (Adt32, (u128, f64), *const Adt36);
let _11: (char,);
let _12: [usize; 8];
let _13: char;
let _14: u32;
let _15: [u8; 2];
let _16: i16;
let _17: u32;
let _18: *mut Adt47;
let _19: char;
let _20: Adt26;
let _21: isize;
let _22: ((u16, isize), f32, isize, u16);
let _23: isize;
let _24: *const (usize, i32);
let _25: char;
let _26: [u64; 8];
let _27: bool;
let _28: u64;
let _29: &'static *const u128;
let _30: Adt17;
let _31: *mut [i128; 2];
let _32: ();
let _33: ();
{
RET = false as i16;
RET = (-58_i8) as i16;
RET = 3633_i16 | 17466_i16;
RET = 64_u8 as i16;
RET = 14174_u16 as i16;
RET = (-12445_i16);
RET = 148_u8 as i16;
Goto(bb1)
}
bb1 = {
RET = 8934_i16 ^ (-30488_i16);
_1 = !7_usize;
RET = (-127286549925186412327809701443054071516_i128) as i16;
RET = 29383_i16 | (-9160_i16);
_1 = !11361720902694830082_usize;
Call(RET = fn9(_1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = '\u{c1c18}';
_1 = !8585222558712489669_usize;
_2 = '\u{f7bd0}';
_2 = '\u{254e8}';
_2 = '\u{40185}';
RET = 20479_u16 as i16;
_1 = !5325992719556324696_usize;
RET = 2627600317_u32 as i16;
RET = (-7078_i16);
RET = (-15739_i16) + 26441_i16;
_1 = !11284760088215574948_usize;
RET = (-14391_i16);
_3 = 35049_u16;
_5.0 = [142987588399616055031053472937016561680_u128,294108689062367720029041390631447464383_u128,28715775070780382069796632184291496792_u128];
_2 = '\u{a02df}';
_4.2.3 = !_3;
_3 = _4.2.3 | _4.2.3;
_7 = [_4.2.3,_4.2.3];
_4.2.1 = (-454651121_i32) as f32;
_4.2.2 = RET as isize;
_6 = [13902891673222524315_u64,15823619170685593840_u64];
_4.2.0.1 = _4.2.2 * _4.2.2;
_4.1.0 = _2;
RET = -27329_i16;
Call(_4.2.0.0 = core::intrinsics::bswap(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4.2.2 = _4.2.0.1;
_4.1.1 = [RET,RET,RET];
_4.2.0 = (_4.2.3, _4.2.2);
_5.3 = &_4.1.1;
_8.1 = _4.2.1 as i32;
_5.0 = [19311089582904362669769379125196500530_u128,10435469763280796720492051212717949770_u128,321432572155165357098694230851844791552_u128];
_4.2.2 = -_4.2.0.1;
_4.2.1 = 122_u8 as f32;
_4.2.0 = (_3, _4.2.2);
_5.1 = &RET;
_4.2.1 = (-6_i8) as f32;
_1 = 11513514718661374636_usize;
_8.0 = _1;
_5.3 = &_4.1.1;
_4.2.1 = 178576156626359900221908195111694808268_u128 as f32;
_4.1.1 = [RET,RET,RET];
_8 = (_1, 1700034271_i32);
RET = (-19427_i16) - (-30465_i16);
_8.0 = 24153126937188051136353092753411083791_u128 as usize;
_4.1.1 = [RET,RET,RET];
_4.2.0 = (_3, _4.2.2);
_4.2.0.0 = !_4.2.3;
RET = 31017_i16 ^ (-17614_i16);
RET = (-13222_i16) - 23403_i16;
_9 = _4.1.0;
_5.0 = [219370573056504617508118679969207926762_u128,178468118564101065906493351014302223633_u128,317211284802575882839742442483534677322_u128];
RET = 31813_i16 | 30369_i16;
Goto(bb4)
}
bb4 = {
_4.1.0 = _2;
_5.0 = [273561863590025352287776605112918978180_u128,195738215974053575403600588790753906963_u128,3548306562526928640531952133328931998_u128];
_5.2 = _8.1 ^ _8.1;
_5.1 = &RET;
_4.2.2 = !_4.2.0.1;
_9 = _4.1.0;
_6 = [14937719207452006643_u64,9985362362197657286_u64];
_8 = (_1, _5.2);
_7 = [_3,_3];
_10.1.0 = !49403549867156920078686633789593487706_u128;
_5.3 = &_4.1.1;
_6 = [17843139625713294713_u64,8869687811582371307_u64];
_15 = [68_u8,54_u8];
_16 = RET;
_13 = _4.1.0;
_14 = 4083666255_u32;
_5.3 = &_4.1.1;
_4.2.3 = !_3;
Goto(bb5)
}
bb5 = {
_10.1.1 = _4.2.0.1 as f64;
_4.2.0 = (_4.2.3, _4.2.2);
_4.1.0 = _13;
_4.0 = !_14;
RET = _4.2.0.0 as i16;
_5.3 = &_4.1.1;
_7 = [_4.2.0.0,_3];
_10.1.1 = 16005507316620054724_u64 as f64;
_16 = RET;
_5.1 = &RET;
Goto(bb6)
}
bb6 = {
_16 = (-70_i8) as i16;
_15 = [143_u8,190_u8];
_1 = _5.2 as usize;
_4.2.0.1 = -_4.2.2;
_4.1.1 = [RET,RET,_16];
_16 = RET ^ RET;
RET = _16 + _16;
_5.3 = &_4.1.1;
_3 = 76_u8 as u16;
_11 = (_13,);
_4.2.2 = -_4.2.0.1;
_8.1 = -_5.2;
_6 = [10950641559376277626_u64,13050582980427874983_u64];
_5.3 = &_4.1.1;
_10.0 = Adt32::Variant2 { fld0: _10.1.0 };
_9 = _13;
SetDiscriminant(_10.0, 0);
match _8.0 {
0 => bb7,
1 => bb8,
2 => bb9,
11513514718661374636 => bb11,
_ => bb10
}
}
bb7 = {
RET = 8934_i16 ^ (-30488_i16);
_1 = !7_usize;
RET = (-127286549925186412327809701443054071516_i128) as i16;
RET = 29383_i16 | (-9160_i16);
_1 = !11361720902694830082_usize;
Call(RET = fn9(_1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_4.1.0 = _2;
_5.0 = [273561863590025352287776605112918978180_u128,195738215974053575403600588790753906963_u128,3548306562526928640531952133328931998_u128];
_5.2 = _8.1 ^ _8.1;
_5.1 = &RET;
_4.2.2 = !_4.2.0.1;
_9 = _4.1.0;
_6 = [14937719207452006643_u64,9985362362197657286_u64];
_8 = (_1, _5.2);
_7 = [_3,_3];
_10.1.0 = !49403549867156920078686633789593487706_u128;
_5.3 = &_4.1.1;
_6 = [17843139625713294713_u64,8869687811582371307_u64];
_15 = [68_u8,54_u8];
_16 = RET;
_13 = _4.1.0;
_14 = 4083666255_u32;
_5.3 = &_4.1.1;
_4.2.3 = !_3;
Goto(bb5)
}
bb9 = {
_4.2.2 = _4.2.0.1;
_4.1.1 = [RET,RET,RET];
_4.2.0 = (_4.2.3, _4.2.2);
_5.3 = &_4.1.1;
_8.1 = _4.2.1 as i32;
_5.0 = [19311089582904362669769379125196500530_u128,10435469763280796720492051212717949770_u128,321432572155165357098694230851844791552_u128];
_4.2.2 = -_4.2.0.1;
_4.2.1 = 122_u8 as f32;
_4.2.0 = (_3, _4.2.2);
_5.1 = &RET;
_4.2.1 = (-6_i8) as f32;
_1 = 11513514718661374636_usize;
_8.0 = _1;
_5.3 = &_4.1.1;
_4.2.1 = 178576156626359900221908195111694808268_u128 as f32;
_4.1.1 = [RET,RET,RET];
_8 = (_1, 1700034271_i32);
RET = (-19427_i16) - (-30465_i16);
_8.0 = 24153126937188051136353092753411083791_u128 as usize;
_4.1.1 = [RET,RET,RET];
_4.2.0 = (_3, _4.2.2);
_4.2.0.0 = !_4.2.3;
RET = 31017_i16 ^ (-17614_i16);
RET = (-13222_i16) - 23403_i16;
_9 = _4.1.0;
_5.0 = [219370573056504617508118679969207926762_u128,178468118564101065906493351014302223633_u128,317211284802575882839742442483534677322_u128];
RET = 31813_i16 | 30369_i16;
Goto(bb4)
}
bb10 = {
_2 = '\u{c1c18}';
_1 = !8585222558712489669_usize;
_2 = '\u{f7bd0}';
_2 = '\u{254e8}';
_2 = '\u{40185}';
RET = 20479_u16 as i16;
_1 = !5325992719556324696_usize;
RET = 2627600317_u32 as i16;
RET = (-7078_i16);
RET = (-15739_i16) + 26441_i16;
_1 = !11284760088215574948_usize;
RET = (-14391_i16);
_3 = 35049_u16;
_5.0 = [142987588399616055031053472937016561680_u128,294108689062367720029041390631447464383_u128,28715775070780382069796632184291496792_u128];
_2 = '\u{a02df}';
_4.2.3 = !_3;
_3 = _4.2.3 | _4.2.3;
_7 = [_4.2.3,_4.2.3];
_4.2.1 = (-454651121_i32) as f32;
_4.2.2 = RET as isize;
_6 = [13902891673222524315_u64,15823619170685593840_u64];
_4.2.0.1 = _4.2.2 * _4.2.2;
_4.1.0 = _2;
RET = -27329_i16;
Call(_4.2.0.0 = core::intrinsics::bswap(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5.3 = &place!(Field::<[i16; 3]>(Variant(_10.0, 0), 2));
_4.2.0.0 = _3;
RET = _16 & _16;
_7 = [_4.2.3,_4.2.3];
_19 = _13;
_3 = true as u16;
place!(Field::<i64>(Variant(_10.0, 0), 6)) = _4.2.3 as i64;
_14 = _4.0 << _1;
place!(Field::<Adt17>(Variant(_10.0, 0), 7)).fld0 = _5.2;
_17 = !_14;
_22.0 = (_4.2.3, _4.2.2);
place!(Field::<i8>(Variant(_10.0, 0), 3)) = !(-106_i8);
_5.1 = &place!(Field::<i16>(Variant(_10.0, 0), 4));
_11 = (_9,);
_23 = _14 as isize;
_4.2.1 = Field::<i64>(Variant(_10.0, 0), 6) as f32;
_5.2 = _13 as i32;
Goto(bb12)
}
bb12 = {
_1 = _8.0;
place!(Field::<[i16; 3]>(Variant(_10.0, 0), 2)) = [_16,_16,RET];
_16 = -RET;
match _8.0 {
0 => bb10,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
11513514718661374636 => bb13,
_ => bb9
}
}
bb13 = {
RET = !_16;
_17 = _10.1.0 as u32;
_4.2.2 = !_23;
_22 = (_4.2.0, _4.2.1, _4.2.2, _3);
_22 = _4.2;
_4.1.1 = [RET,RET,RET];
_11 = (_13,);
_5.2 = -Field::<Adt17>(Variant(_10.0, 0), 7).fld0;
_22.3 = _3;
match _1 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb14,
4 => bb15,
11513514718661374636 => bb17,
_ => bb16
}
}
bb14 = {
_1 = _8.0;
place!(Field::<[i16; 3]>(Variant(_10.0, 0), 2)) = [_16,_16,RET];
_16 = -RET;
match _8.0 {
0 => bb10,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
11513514718661374636 => bb13,
_ => bb9
}
}
bb15 = {
RET = 8934_i16 ^ (-30488_i16);
_1 = !7_usize;
RET = (-127286549925186412327809701443054071516_i128) as i16;
RET = 29383_i16 | (-9160_i16);
_1 = !11361720902694830082_usize;
Call(RET = fn9(_1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
RET = 8934_i16 ^ (-30488_i16);
_1 = !7_usize;
RET = (-127286549925186412327809701443054071516_i128) as i16;
RET = 29383_i16 | (-9160_i16);
_1 = !11361720902694830082_usize;
Call(RET = fn9(_1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_14 = _11.0 as u32;
_9 = _4.1.0;
place!(Field::<i16>(Variant(_10.0, 0), 4)) = RET + _16;
_22.2 = _23 + _23;
place!(Field::<i8>(Variant(_10.0, 0), 3)) = 6_i8;
_8.1 = !Field::<Adt17>(Variant(_10.0, 0), 7).fld0;
_4.1 = (_2, Field::<[i16; 3]>(Variant(_10.0, 0), 2));
_10.0 = Adt32::Variant2 { fld0: _10.1.0 };
_6 = [2963431295585337650_u64,1375751967688882655_u64];
_23 = -_4.2.2;
_19 = _2;
RET = _16 + _16;
_1 = _8.0;
_4.2.2 = -_23;
_22.1 = _4.2.1 * _4.2.1;
_4.2 = (_22.0, _22.1, _22.0.1, _3);
_22.0.0 = _4.2.0.0 | _22.3;
_4.2.0.1 = _22.2;
_22.0.1 = 10_u8 as isize;
Goto(bb18)
}
bb18 = {
Call(_32 = dump_var(8_usize, 9_usize, Move(_9), 11_usize, Move(_11), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(8_usize, 17_usize, Move(_17), 2_usize, Move(_2), 6_usize, Move(_6), 33_usize, _33), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize) -> i16 {
mir! {
type RET = i16;
let _5: *const [i128; 4];
let _6: bool;
let _7: f64;
let _8: *const i16;
let _9: bool;
let _10: isize;
let _11: i128;
let _12: f32;
let _13: *const u128;
let _14: bool;
let _15: i128;
let _16: (u16, isize);
let _17: *mut Adt47;
let _18: Adt26;
let _19: isize;
let _20: i8;
let _21: (u128, f64);
let _22: i64;
let _23: u16;
let _24: ();
let _25: ();
{
RET = _4 as i16;
RET = 2748526885_u32 as i16;
RET = (-25201_i16);
_4 = '\u{eaf9b}' as usize;
RET = 2362_i16 + 14956_i16;
_4 = _1;
_2 = _4;
RET = (-22_i8) as i16;
_4 = _2;
RET = 37_u8 as i16;
Goto(bb1)
}
bb1 = {
_1 = !_2;
Goto(bb2)
}
bb2 = {
_3 = !_4;
RET = 28400_i16;
_3 = _2 >> RET;
RET = 25886_i16 >> _3;
_6 = true;
RET = 12597_i16;
Goto(bb3)
}
bb3 = {
_2 = _3 << _3;
_2 = !_4;
_6 = true;
_1 = _3 | _3;
Goto(bb4)
}
bb4 = {
_2 = !_3;
_2 = _1 << RET;
Goto(bb5)
}
bb5 = {
_8 = core::ptr::addr_of!(RET);
_9 = _6 ^ _6;
_3 = !_2;
_4 = _1;
(*_8) = (-7100_i16);
(*_8) = -3110_i16;
_8 = core::ptr::addr_of!((*_8));
_9 = !_6;
Goto(bb6)
}
bb6 = {
_8 = core::ptr::addr_of!(RET);
_1 = _2;
_9 = _2 < _1;
Goto(bb7)
}
bb7 = {
(*_8) = !32489_i16;
RET = 17435_i16 << _4;
(*_8) = (-9802_i16) | 14985_i16;
_4 = !_1;
_2 = (-113845858231251497522895283231571566060_i128) as usize;
_8 = core::ptr::addr_of!(RET);
_4 = _1;
_9 = RET != (*_8);
(*_8) = 26461_i16;
_2 = _3 | _3;
_4 = (-23430101758373411876555430681728623554_i128) as usize;
_10 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
RET = !31939_i16;
Call(_2 = core::intrinsics::transmute(_4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7 = _1 as f64;
_3 = _4 ^ _1;
_3 = _4 & _4;
_1 = _4 & _4;
(*_8) = 8481_i16;
_12 = 129964932965613710_u64 as f32;
_11 = _10 as i128;
_1 = !_3;
_11 = (-158695370059142558355565791222591046609_i128) * 119089794234340774654733429878567395639_i128;
_9 = _6;
RET = !(-31459_i16);
RET = _11 as i16;
(*_8) = (-10854_i16);
_2 = _3 * _3;
RET = 18737_i16;
_11 = (-69158266807895166172671197251702599839_i128);
_4 = _2;
_2 = _7 as usize;
_2 = _3 + _4;
_4 = _2;
(*_8) = 15638_i16 << _1;
_6 = !_9;
Goto(bb9)
}
bb9 = {
RET = _11 as i16;
_1 = _2;
_6 = !_9;
_12 = 11958936725600748104_u64 as f32;
_10 = 17_isize & 9223372036854775807_isize;
_11 = !(-53847137611337254557862676498015176310_i128);
_4 = _7 as usize;
Call(RET = fn10(_10, _10, Move(_8), _3, _7, _10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8 = core::ptr::addr_of!(RET);
_9 = _6;
_2 = 177974013106097061529149453836210248109_u128 as usize;
_15 = _9 as i128;
_14 = !_9;
RET = (-11422_i16);
_4 = !_1;
Goto(bb11)
}
bb11 = {
_1 = _4;
_6 = !_9;
_16.0 = 13120_u16 | 23210_u16;
_16.1 = _10 - _10;
_16 = (15422_u16, _10);
_14 = _10 != _16.1;
_12 = 1268412647_u32 as f32;
_1 = _4 * _4;
_1 = _4;
match RET {
0 => bb5,
340282366920938463463374607431768200034 => bb13,
_ => bb12
}
}
bb12 = {
_8 = core::ptr::addr_of!(RET);
_9 = _6;
_2 = 177974013106097061529149453836210248109_u128 as usize;
_15 = _9 as i128;
_14 = !_9;
RET = (-11422_i16);
_4 = !_1;
Goto(bb11)
}
bb13 = {
_19 = _10;
_10 = _11 as isize;
_7 = 155_u8 as f64;
_3 = _1 | _4;
_6 = _14;
_16 = (57452_u16, _10);
_21.0 = 192743672191799102318960593007550319847_u128 * 303032176984234492284488669099369748169_u128;
_13 = core::ptr::addr_of!(_21.0);
_13 = core::ptr::addr_of!((*_13));
_14 = _6;
_19 = (-109_i8) as isize;
_16 = (249_u16, _19);
_4 = !_1;
_18 = Adt26::Variant1 { fld0: Move(_13),fld1: '\u{64818}',fld2: _16.0,fld3: (-3915182322642778491_i64) };
_16 = (Field::<u16>(Variant(_18, 1), 2), _19);
Call((*_8) = core::intrinsics::transmute(_16.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_14 = _6 ^ _6;
_13 = core::ptr::addr_of!(_21.0);
_16.1 = 5339192881048938243_u64 as isize;
_8 = core::ptr::addr_of!(RET);
_9 = _6;
_11 = _15 - _15;
_16.0 = !Field::<u16>(Variant(_18, 1), 2);
_7 = (-8680597569030566792_i64) as f64;
RET = 18249_i16;
_6 = _3 >= _1;
_21 = (303767679164360120790633533957966273807_u128, _7);
_16 = (Field::<u16>(Variant(_18, 1), 2), _10);
_22 = 261703963424848203_i64;
_4 = _1;
_7 = _21.1;
_21.0 = !142438973299736247673879413138632986839_u128;
place!(Field::<i64>(Variant(_18, 1), 3)) = _22 * _22;
_21.0 = !156397767484620422046395942258212443435_u128;
_21 = (101015960833956000518081286590843471714_u128, _7);
_16 = (Field::<u16>(Variant(_18, 1), 2), _10);
_8 = core::ptr::addr_of!((*_8));
(*_8) = -16352_i16;
_21.0 = !317825000465099543489775246891885032791_u128;
_23 = _11 as u16;
_6 = !_14;
place!(Field::<i64>(Variant(_18, 1), 3)) = _11 as i64;
_18 = Adt26::Variant1 { fld0: Move(_13),fld1: '\u{13d34}',fld2: _23,fld3: _22 };
_13 = core::ptr::addr_of!(_21.0);
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(9_usize, 2_usize, Move(_2), 19_usize, Move(_19), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(9_usize, 15_usize, Move(_15), 6_usize, Move(_6), 22_usize, Move(_22), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: *const i16,mut _4: usize,mut _5: f64,mut _6: isize) -> i16 {
mir! {
type RET = i16;
let _7: i8;
let _8: f64;
let _9: usize;
let _10: (char, [i16; 3]);
let _11: char;
let _12: bool;
let _13: u32;
let _14: &'static [i16; 3];
let _15: u64;
let _16: f64;
let _17: i128;
let _18: *const i16;
let _19: f32;
let _20: Adt79;
let _21: *const [usize; 8];
let _22: usize;
let _23: bool;
let _24: ((u16, isize), f32, isize, u16);
let _25: [u64; 2];
let _26: [i64; 7];
let _27: *const (usize, i32);
let _28: [i128; 5];
let _29: Adt17;
let _30: [u16; 6];
let _31: Adt36;
let _32: [i128; 4];
let _33: u8;
let _34: Adt32;
let _35: ();
let _36: ();
{
_4 = 7383153923382366666_usize;
_1 = 106010818706710851099896367862547224230_i128 as isize;
_4 = 4_usize * 6_usize;
RET = 29_i8 as i16;
RET = 32155_i16 * (-28427_i16);
_5 = _6 as f64;
RET = (-1086_i16) << _2;
RET = (-9510_i16);
_5 = 188_u8 as f64;
_3 = core::ptr::addr_of!(RET);
(*_3) = '\u{55e3d}' as i16;
_6 = _2;
_6 = -_1;
_4 = _5 as usize;
Goto(bb1)
}
bb1 = {
_5 = 591866780_i32 as f64;
_3 = core::ptr::addr_of!((*_3));
_5 = (-2611140402678611970_i64) as f64;
_9 = 30926749941634110_u64 as usize;
Goto(bb2)
}
bb2 = {
_2 = (-10796463952798550066239806222018729823_i128) as isize;
_3 = core::ptr::addr_of!((*_3));
_9 = _4 & _4;
_10.1 = [RET,(*_3),RET];
_4 = _9;
_8 = -_5;
(*_3) = 115050427040520972952430741815718565800_i128 as i16;
_2 = _6;
Goto(bb3)
}
bb3 = {
_10.1 = [(*_3),(*_3),(*_3)];
(*_3) = !(-6722_i16);
_10.0 = '\u{68f6a}';
_3 = core::ptr::addr_of!(RET);
_4 = _9;
_3 = core::ptr::addr_of!(RET);
(*_3) = -12277_i16;
_7 = 59_i8;
_1 = 6447_u16 as isize;
_2 = _9 as isize;
_9 = !_4;
_4 = _9;
Call(_7 = core::intrinsics::bswap((-80_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = 1262316160_i32 as isize;
_9 = _4;
_3 = core::ptr::addr_of!((*_3));
_7 = _2 as i8;
_10.0 = '\u{102031}';
_13 = _2 as u32;
_6 = _2;
_9 = _4 + _4;
_1 = _2;
_12 = false;
_5 = _8;
_8 = _13 as f64;
_3 = core::ptr::addr_of!(RET);
_10.1 = [(*_3),RET,(*_3)];
_12 = true;
_2 = _6 << _4;
_11 = _10.0;
Goto(bb5)
}
bb5 = {
RET = (-17638_i16);
_15 = 3689602490063116110_u64 >> _2;
_6 = -_2;
_10.1 = [RET,RET,(*_3)];
_4 = _9 ^ _9;
_3 = core::ptr::addr_of!((*_3));
_1 = 139452186709629113411290016100303867562_u128 as isize;
_15 = 8678952500131465639_u64 << _9;
_17 = (-71073107622148888161856544868857256822_i128) & 49533643656490879996752038431963563290_i128;
_3 = core::ptr::addr_of!((*_3));
_2 = _6 - _6;
_7 = (-38_i8) * 86_i8;
_11 = _10.0;
_19 = _15 as f32;
_15 = 16373733535391834393_u64 * 8088585114053148372_u64;
_14 = &_10.1;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = 151976583657496357382539941827655409937_u128 as f64;
_6 = _2 & _2;
_14 = &_10.1;
_13 = 1946818968_u32;
_14 = &(*_14);
_19 = _17 as f32;
RET = _13 as i16;
_6 = _1;
RET = 25810_i16;
_18 = Move(_3);
_7 = (-50_i8);
_8 = _5 + _16;
Call(_6 = fn11(_2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = Move(_18);
_12 = true & false;
_5 = _16 - _16;
_23 = _2 != _2;
_14 = &(*_14);
_1 = -_2;
RET = -(-11713_i16);
_14 = &(*_14);
_7 = -(-73_i8);
_10.1 = [RET,RET,RET];
_24.0 = (39669_u16, _1);
_5 = _9 as f64;
_24.3 = !_24.0.0;
_10.0 = _11;
_8 = -_5;
match _24.0.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
39669 => bb13,
_ => bb12
}
}
bb8 = {
_5 = 591866780_i32 as f64;
_3 = core::ptr::addr_of!((*_3));
_5 = (-2611140402678611970_i64) as f64;
_9 = 30926749941634110_u64 as usize;
Goto(bb2)
}
bb9 = {
RET = (-17638_i16);
_15 = 3689602490063116110_u64 >> _2;
_6 = -_2;
_10.1 = [RET,RET,(*_3)];
_4 = _9 ^ _9;
_3 = core::ptr::addr_of!((*_3));
_1 = 139452186709629113411290016100303867562_u128 as isize;
_15 = 8678952500131465639_u64 << _9;
_17 = (-71073107622148888161856544868857256822_i128) & 49533643656490879996752038431963563290_i128;
_3 = core::ptr::addr_of!((*_3));
_2 = _6 - _6;
_7 = (-38_i8) * 86_i8;
_11 = _10.0;
_19 = _15 as f32;
_15 = 16373733535391834393_u64 * 8088585114053148372_u64;
_14 = &_10.1;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_1 = 1262316160_i32 as isize;
_9 = _4;
_3 = core::ptr::addr_of!((*_3));
_7 = _2 as i8;
_10.0 = '\u{102031}';
_13 = _2 as u32;
_6 = _2;
_9 = _4 + _4;
_1 = _2;
_12 = false;
_5 = _8;
_8 = _13 as f64;
_3 = core::ptr::addr_of!(RET);
_10.1 = [(*_3),RET,(*_3)];
_12 = true;
_2 = _6 << _4;
_11 = _10.0;
Goto(bb5)
}
bb11 = {
_10.1 = [(*_3),(*_3),(*_3)];
(*_3) = !(-6722_i16);
_10.0 = '\u{68f6a}';
_3 = core::ptr::addr_of!(RET);
_4 = _9;
_3 = core::ptr::addr_of!(RET);
(*_3) = -12277_i16;
_7 = 59_i8;
_1 = 6447_u16 as isize;
_2 = _9 as isize;
_9 = !_4;
_4 = _9;
Call(_7 = core::intrinsics::bswap((-80_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_2 = (-10796463952798550066239806222018729823_i128) as isize;
_3 = core::ptr::addr_of!((*_3));
_9 = _4 & _4;
_10.1 = [RET,(*_3),RET];
_4 = _9;
_8 = -_5;
(*_3) = 115050427040520972952430741815718565800_i128 as i16;
_2 = _6;
Goto(bb3)
}
bb13 = {
_1 = 8910573036939239312_i64 as isize;
_3 = core::ptr::addr_of!(RET);
RET = !24115_i16;
_3 = core::ptr::addr_of!(RET);
_25 = [_15,_15];
_26 = [5304842990996122306_i64,6555623386007435966_i64,(-1042711524791068672_i64),4083722612445207659_i64,5225111154101517905_i64,3403703342109648408_i64,(-7536030505684637036_i64)];
_8 = _5;
_24.2 = _2 << _24.3;
_8 = _5;
_3 = core::ptr::addr_of!((*_3));
_8 = 231_u8 as f64;
_24.1 = -_19;
_7 = !(-118_i8);
_22 = !_9;
_2 = _24.2 - _24.2;
_24.2 = _24.0.1 >> _4;
_32 = [_17,_17,_17,_17];
_18 = core::ptr::addr_of!(RET);
_29.fld0 = _17 as i32;
_30 = [_24.0.0,_24.0.0,_24.3,_24.0.0,_24.3,_24.0.0];
(*_3) = _17 as i16;
Goto(bb14)
}
bb14 = {
_7 = !(-122_i8);
_8 = _5 + _5;
_24.3 = 335349684302628174599879747834286398511_u128 as u16;
_32 = [_17,_17,_17,_17];
_8 = _16 * _5;
_10.0 = _11;
_6 = _17 as isize;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(10_usize, 15_usize, Move(_15), 9_usize, Move(_9), 11_usize, Move(_11), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(10_usize, 23_usize, Move(_23), 6_usize, Move(_6), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(10_usize, 7_usize, Move(_7), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize) -> isize {
mir! {
type RET = isize;
let _2: f64;
let _3: *const u128;
let _4: isize;
let _5: char;
let _6: i16;
let _7: *mut i8;
let _8: ([u128; 3], &'static i16, i32, &'static [i16; 3]);
let _9: [u64; 2];
let _10: u64;
let _11: usize;
let _12: i16;
let _13: bool;
let _14: isize;
let _15: i32;
let _16: u16;
let _17: bool;
let _18: (u16,);
let _19: f64;
let _20: *mut [u16; 2];
let _21: &'static Adt17;
let _22: u32;
let _23: u8;
let _24: [u32; 1];
let _25: [i128; 5];
let _26: [u8; 2];
let _27: (Adt32, (u128, f64), *const Adt36);
let _28: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _29: (u128, f64);
let _30: isize;
let _31: ();
let _32: ();
{
RET = _1 * _1;
RET = _1;
RET = -_1;
RET = _1 << _1;
_2 = (-4858320290353903929_i64) as f64;
RET = _1;
RET = !_1;
RET = !_1;
RET = !_1;
RET = _1;
RET = _1 & _1;
RET = 671447452_i32 as isize;
_1 = RET;
RET = _1;
RET = _1;
_6 = 18961_i16 + (-29557_i16);
RET = 134169751622517431750622262289471340151_u128 as isize;
_5 = '\u{4c4ec}';
_4 = RET;
RET = _4;
_2 = _6 as f64;
RET = _1 & _4;
_6 = 24819_i16;
Call(_6 = fn12(_1, RET, _5, RET, RET, _4, _1, _4, _1, RET, _5, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 15412_i16 & 24854_i16;
_8.0 = [250696437716572474512750053479570052594_u128,125129053320362984663493185030857225805_u128,217562261867430462515103273512216229764_u128];
_8.2 = _1 as i32;
_4 = RET ^ RET;
RET = _4 & _4;
RET = _4;
_8.0 = [231426544316377078882325539405838458520_u128,191325692850331490304115875662262134274_u128,330324717866941775902144468486242988306_u128];
Goto(bb2)
}
bb2 = {
_8.1 = &_6;
_8.0 = [110217763753831003820128394415781315167_u128,45354689435886186088917624027251049549_u128,109781618826388490545077462512046906603_u128];
RET = (-4313311773006299899_i64) as isize;
_4 = RET >> _6;
_2 = _4 as f64;
_8.0 = [211708026622881168196878240683438760765_u128,89664289138029372574379730471903280056_u128,322696898648078952497925551969179814043_u128];
_4 = _1 ^ _1;
RET = _4 >> _8.2;
_8.0 = [9837987980750738403977898861662927835_u128,283606024588156065192503639544415457099_u128,255170402699269063160401760038997132595_u128];
_5 = '\u{98dee}';
Goto(bb3)
}
bb3 = {
_11 = !0_usize;
_11 = !87439264872440385_usize;
_10 = 6634297610260898327_u64 << RET;
_8.1 = &_6;
_8.0 = [233138626920272154212973612486998218452_u128,260088764937270034048807963767746566411_u128,60971964216117287146772493525990701636_u128];
Call(_8.2 = fn15(RET), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _4;
_8.2 = _11 as i32;
_6 = _10 as i16;
_8.1 = &_6;
RET = _1;
_2 = 19762270114598842667824784594546385187_i128 as f64;
_10 = 121785515585632278213649980445718135331_u128 as u64;
_8.2 = (-1591290045_i32) | 40966975_i32;
_4 = RET;
RET = _1;
RET = _4 - _4;
RET = _4;
_6 = _4 as i16;
_12 = _6;
_8.1 = &_12;
_13 = true;
Goto(bb5)
}
bb5 = {
_8.0 = [276009062971548973300225370877268718356_u128,203015681326075005982344894584376398770_u128,263817200267343176181359150699173657708_u128];
_11 = !16120034144399437102_usize;
_2 = 6525253686371763460_i64 as f64;
_9 = [_10,_10];
_1 = !_4;
_15 = 3275_u16 as i32;
_15 = RET as i32;
_10 = !11113136879024093166_u64;
RET = _8.2 as isize;
_14 = _1;
_6 = !_12;
_16 = !50938_u16;
_1 = 196881315485264265751888219350865750943_u128 as isize;
_12 = !_6;
Goto(bb6)
}
bb6 = {
_12 = _2 as i16;
Goto(bb7)
}
bb7 = {
_16 = _2 as u16;
_14 = _15 as isize;
_8.1 = &_6;
_8.1 = &_6;
_8.1 = &_6;
Goto(bb8)
}
bb8 = {
_13 = false;
_6 = _12 ^ _12;
_14 = !RET;
_14 = _6 as isize;
_19 = _2 - _2;
_17 = _13;
RET = _4;
RET = _14;
_4 = _1;
_13 = _17;
_18 = (_16,);
_6 = _12 + _12;
_15 = _8.2;
_11 = 17610908877065997059_usize;
_18.0 = !_16;
_17 = _13;
_11 = 171_u8 as usize;
_10 = 10134556799579366594_u64;
Goto(bb9)
}
bb9 = {
_8.0 = [287029894807435195068076331883051734737_u128,154062154077412140488941577913646246279_u128,4028585020129616165789724898167734463_u128];
_16 = _18.0 | _18.0;
_18 = (_16,);
_6 = _12;
RET = _15 as isize;
_22 = _5 as u32;
_8.2 = _15 - _15;
_8.1 = &_12;
RET = _4 >> _16;
Goto(bb10)
}
bb10 = {
_12 = _6;
_16 = !_18.0;
_8.2 = !_15;
_1 = 297398590484604382772841786748520351948_u128 as isize;
_2 = -_19;
_11 = !1713876894716300559_usize;
RET = _14;
_8.1 = &_6;
_16 = 4942319507192440364_i64 as u16;
_24 = [_22];
_5 = '\u{53013}';
match _10 {
0 => bb11,
1 => bb12,
10134556799579366594 => bb14,
_ => bb13
}
}
bb11 = {
_8.0 = [287029894807435195068076331883051734737_u128,154062154077412140488941577913646246279_u128,4028585020129616165789724898167734463_u128];
_16 = _18.0 | _18.0;
_18 = (_16,);
_6 = _12;
RET = _15 as isize;
_22 = _5 as u32;
_8.2 = _15 - _15;
_8.1 = &_12;
RET = _4 >> _16;
Goto(bb10)
}
bb12 = {
RET = _4;
_8.2 = _11 as i32;
_6 = _10 as i16;
_8.1 = &_6;
RET = _1;
_2 = 19762270114598842667824784594546385187_i128 as f64;
_10 = 121785515585632278213649980445718135331_u128 as u64;
_8.2 = (-1591290045_i32) | 40966975_i32;
_4 = RET;
RET = _1;
RET = _4 - _4;
RET = _4;
_6 = _4 as i16;
_12 = _6;
_8.1 = &_12;
_13 = true;
Goto(bb5)
}
bb13 = {
_16 = _2 as u16;
_14 = _15 as isize;
_8.1 = &_6;
_8.1 = &_6;
_8.1 = &_6;
Goto(bb8)
}
bb14 = {
_8.0 = [143021868002671459123390614677193569006_u128,289910705503283990494853484761902415990_u128,226635898419409731186860480658120998263_u128];
_18.0 = !_16;
_8.0 = [66955694393878896569916771199356368722_u128,118729775909175801421138427625290449255_u128,235612376522305274825509426464395696833_u128];
_18.0 = !_16;
_8.3 = &_28.1.1;
_28.2.0.0 = _16;
_13 = _12 >= _6;
_23 = !189_u8;
_3 = core::ptr::addr_of!(_29.0);
_22 = (-2602308950646068218_i64) as u32;
_28.2.0 = (_16, _1);
(*_3) = _5 as u128;
_26 = [_23,_23];
_5 = '\u{ec0a3}';
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(11_usize, 11_usize, Move(_11), 1_usize, Move(_1), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(11_usize, 10_usize, Move(_10), 24_usize, Move(_24), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(11_usize, 14_usize, Move(_14), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: char,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: char,mut _12: isize) -> i16 {
mir! {
type RET = i16;
let _13: (Adt32, (u128, f64), *const Adt36);
let _14: char;
let _15: isize;
let _16: f64;
let _17: u128;
let _18: [u64; 5];
let _19: (Adt32, (u128, f64), *const Adt36);
let _20: isize;
let _21: isize;
let _22: i8;
let _23: usize;
let _24: *const usize;
let _25: [i128; 5];
let _26: Adt36;
let _27: Adt79;
let _28: isize;
let _29: [i128; 5];
let _30: *const u32;
let _31: Adt72;
let _32: [usize; 8];
let _33: (bool, u64, f64, u64);
let _34: ();
let _35: ();
{
RET = (-188_i16);
RET = (-19091_i16) >> _2;
Goto(bb1)
}
bb1 = {
RET = !(-1430_i16);
_9 = -_2;
_8 = !_9;
_8 = 192568893796236898486473079853278898225_u128 as isize;
_2 = _5;
_2 = _5;
_11 = _3;
_9 = -_7;
_3 = _11;
RET = !7536_i16;
_7 = _8 * _4;
_13.0 = Adt32::Variant2 { fld0: 81452954796100840287206144932603492527_u128 };
_8 = _4;
Goto(bb2)
}
bb2 = {
_13.0 = Adt32::Variant2 { fld0: 157050043868032166581015559007919076621_u128 };
RET = (-3042_i16) << _1;
_13.1.1 = 16317204066666337248_usize as f64;
place!(Field::<u128>(Variant(_13.0, 2), 0)) = 284896555521619402540943727744025275637_u128;
_13.1.0 = Field::<u128>(Variant(_13.0, 2), 0) + Field::<u128>(Variant(_13.0, 2), 0);
_5 = -_2;
_13.0 = Adt32::Variant2 { fld0: _13.1.0 };
_10 = _3 as isize;
place!(Field::<u128>(Variant(_13.0, 2), 0)) = !_13.1.0;
_13.0 = Adt32::Variant2 { fld0: _13.1.0 };
SetDiscriminant(_13.0, 1);
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [(-1873524140637362434_i64),9017903510149754434_i64,9031231605336293458_i64,5619166445426878154_i64,6154202590249380954_i64,(-4907704786811231714_i64),2950413407129952379_i64];
RET = _13.1.0 as i16;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = !_9;
place!(Field::<u128>(Variant(_13.0, 1), 1)) = (-6209917210791070520_i64) as u128;
_14 = _11;
_13.1.1 = 9372408994032980152_u64 as f64;
place!(Field::<u128>(Variant(_13.0, 1), 1)) = !_13.1.0;
place!(Field::<f64>(Variant(_13.0, 1), 0)) = _13.1.1 * _13.1.1;
place!(Field::<f64>(Variant(_13.0, 1), 0)) = _13.1.1;
_5 = Field::<isize>(Variant(_13.0, 1), 2) + _8;
Call(_5 = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _14;
_13.1.1 = Field::<f64>(Variant(_13.0, 1), 0);
_11 = _3;
_13.1.0 = Field::<u128>(Variant(_13.0, 1), 1);
place!(Field::<u128>(Variant(_13.0, 1), 1)) = _13.1.0 * _13.1.0;
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [(-3326485620418005491_i64),6844264613351258489_i64,4132788286439378422_i64,(-3691496349878236671_i64),(-2306032494364302881_i64),2407686726770185745_i64,6452174388904020312_i64];
_13.1 = (Field::<u128>(Variant(_13.0, 1), 1), Field::<f64>(Variant(_13.0, 1), 0));
_16 = -Field::<f64>(Variant(_13.0, 1), 0);
_13.1.0 = Field::<u128>(Variant(_13.0, 1), 1) | Field::<u128>(Variant(_13.0, 1), 1);
place!(Field::<i128>(Variant(_13.0, 1), 3)) = true as i128;
_10 = _7;
_1 = 8255241965565375688_i64 as isize;
RET = 5691_i16;
_10 = 5455462689624865362_i64 as isize;
_5 = 822580421_i32 as isize;
_15 = _2 * _7;
_3 = _11;
match RET {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
5691 => bb10,
_ => bb9
}
}
bb4 = {
_13.0 = Adt32::Variant2 { fld0: 157050043868032166581015559007919076621_u128 };
RET = (-3042_i16) << _1;
_13.1.1 = 16317204066666337248_usize as f64;
place!(Field::<u128>(Variant(_13.0, 2), 0)) = 284896555521619402540943727744025275637_u128;
_13.1.0 = Field::<u128>(Variant(_13.0, 2), 0) + Field::<u128>(Variant(_13.0, 2), 0);
_5 = -_2;
_13.0 = Adt32::Variant2 { fld0: _13.1.0 };
_10 = _3 as isize;
place!(Field::<u128>(Variant(_13.0, 2), 0)) = !_13.1.0;
_13.0 = Adt32::Variant2 { fld0: _13.1.0 };
SetDiscriminant(_13.0, 1);
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [(-1873524140637362434_i64),9017903510149754434_i64,9031231605336293458_i64,5619166445426878154_i64,6154202590249380954_i64,(-4907704786811231714_i64),2950413407129952379_i64];
RET = _13.1.0 as i16;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = !_9;
place!(Field::<u128>(Variant(_13.0, 1), 1)) = (-6209917210791070520_i64) as u128;
_14 = _11;
_13.1.1 = 9372408994032980152_u64 as f64;
place!(Field::<u128>(Variant(_13.0, 1), 1)) = !_13.1.0;
place!(Field::<f64>(Variant(_13.0, 1), 0)) = _13.1.1 * _13.1.1;
place!(Field::<f64>(Variant(_13.0, 1), 0)) = _13.1.1;
_5 = Field::<isize>(Variant(_13.0, 1), 2) + _8;
Call(_5 = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = !(-1430_i16);
_9 = -_2;
_8 = !_9;
_8 = 192568893796236898486473079853278898225_u128 as isize;
_2 = _5;
_2 = _5;
_11 = _3;
_9 = -_7;
_3 = _11;
RET = !7536_i16;
_7 = _8 * _4;
_13.0 = Adt32::Variant2 { fld0: 81452954796100840287206144932603492527_u128 };
_8 = _4;
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
_15 = -_2;
_13.1 = (Field::<u128>(Variant(_13.0, 1), 1), _16);
_2 = _10 & _7;
_6 = _15 * _2;
place!(Field::<[usize; 8]>(Variant(_13.0, 1), 4)) = [5_usize,10249918595917440329_usize,2_usize,5_usize,15247826242644899576_usize,14707531427753411652_usize,6223526647832999603_usize,0_usize];
place!(Field::<i128>(Variant(_13.0, 1), 3)) = 43866862719237472686581645205782497723_i128;
_6 = _5 * _12;
_3 = _11;
_18 = [13469610075762819287_u64,53319971104366547_u64,17672713919581022563_u64,17595521906258390944_u64,16953725862113437022_u64];
_16 = 3423712164275065546_i64 as f64;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = _15 | _5;
_9 = Field::<isize>(Variant(_13.0, 1), 2) ^ _2;
place!(Field::<i128>(Variant(_13.0, 1), 3)) = -55323707644041677458502281634896882845_i128;
_18 = [9869199623335547621_u64,5783365015043602431_u64,10899989011149856893_u64,77828392024743933_u64,7638997511498362901_u64];
_6 = _9 - _9;
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [(-3703643552558880313_i64),4852292109650727327_i64,7715624249638677373_i64,3278553534263576700_i64,8413304894671434639_i64,6900903537043295028_i64,(-8350117057589493745_i64)];
_11 = _14;
_9 = _10;
_10 = _6 >> Field::<i128>(Variant(_13.0, 1), 3);
Call(RET = fn13(Field::<[usize; 8]>(Variant(_13.0, 1), 4), _7, _6, _10, Field::<isize>(Variant(_13.0, 1), 2), Field::<u128>(Variant(_13.0, 1), 1), _2, _9, _10, _10, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = _11;
_3 = _11;
_6 = _7;
_6 = _10;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = _9 << _15;
_13.1.0 = (-909728437_i32) as u128;
RET = -32175_i16;
_13.1 = (Field::<u128>(Variant(_13.0, 1), 1), _16);
_6 = -_7;
_5 = !_2;
_20 = 20_i8 as isize;
RET = (-12380_i16);
_17 = !_13.1.0;
match RET {
340282366920938463463374607431768199076 => bb12,
_ => bb5
}
}
bb12 = {
_24 = core::ptr::addr_of!(_23);
place!(Field::<f64>(Variant(_13.0, 1), 0)) = 8938_u16 as f64;
(*_24) = 2_usize;
_19.1.1 = -Field::<f64>(Variant(_13.0, 1), 0);
_5 = 59316_u16 as isize;
_21 = _10;
_25 = [Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3)];
place!(Field::<*const i64>(Variant(_13.0, 1), 6)) = core::ptr::addr_of!(place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5))[_23]);
_4 = _2;
SetDiscriminant(_13.0, 1);
_25[_23] = (-66084386601907121846573626713794244320_i128) << _21;
_24 = core::ptr::addr_of!(_23);
_12 = 52_u8 as isize;
_4 = _21 << _13.1.0;
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [1265590604273523355_i64,(-6038242732264043341_i64),(-3857887405629707003_i64),(-609622199012394313_i64),(-779703113413279135_i64),(-3133118383878398732_i64),(-4717576429878504697_i64)];
_11 = _14;
_11 = _14;
_8 = _2;
place!(Field::<*const i64>(Variant(_13.0, 1), 6)) = core::ptr::addr_of!(place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5))[_23]);
_25[_23] = 6544025200041594853015469987301046610_i128;
Call(_23 = fn14(Field::<[i64; 7]>(Variant(_13.0, 1), 5)[_23], _8, _18, _25[_23]), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9 = !_2;
_19.1 = (_17, _13.1.1);
_7 = _2 + _4;
_8 = !_10;
_1 = _7;
place!(Field::<[i64; 7]>(Variant(_13.0, 1), 5)) = [(-5083267134577184380_i64),(-8759110202287239520_i64),1185616175840320761_i64,(-4691488913787027868_i64),7998161156699999313_i64,7429381475367488664_i64,(-371387785433783116_i64)];
_19.1.1 = -_16;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = -_2;
_1 = -_8;
_15 = 362593353_i32 as isize;
place!(Field::<u128>(Variant(_13.0, 1), 1)) = !_17;
place!(Field::<i128>(Variant(_13.0, 1), 3)) = (*_24) as i128;
_12 = false as isize;
_13.1.1 = 21_i8 as f64;
_13.2 = core::ptr::addr_of!(_26);
_7 = _4 >> _2;
place!(Field::<isize>(Variant(_13.0, 1), 2)) = _19.1.0 as isize;
Goto(bb14)
}
bb14 = {
_29 = [Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3)];
_25 = [Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3),Field::<i128>(Variant(_13.0, 1), 3)];
_23 = 126_u8 as usize;
place!(Field::<i128>(Variant(_13.0, 1), 3)) = (*_24) as i128;
place!(Field::<[usize; 8]>(Variant(_13.0, 1), 4)) = [(*_24),(*_24),_23,(*_24),(*_24),_23,_23,_23];
_5 = 13949_u16 as isize;
_18 = [12214404797011529219_u64,8665418309321005233_u64,12805993426906320087_u64,7532521795118533774_u64,9727477675781419117_u64];
_22 = !77_i8;
_31.fld2.0 = (*_24);
_28 = _4;
_31.fld0.0 = _14;
_19.2 = core::ptr::addr_of!(_26);
_7 = -_1;
_15 = _9;
_31.fld0.0 = _14;
RET = 26067_i16;
_23 = _31.fld2.0 - _31.fld2.0;
_8 = _7;
_18 = [13806994431527867147_u64,1237600803050286995_u64,14363901937797002463_u64,8319725959093955347_u64,1880943190636490051_u64];
_19.0 = Adt32::Variant1 { fld0: _16,fld1: _19.1.0,fld2: _28,fld3: Field::<i128>(Variant(_13.0, 1), 3),fld4: Field::<[usize; 8]>(Variant(_13.0, 1), 4),fld5: Field::<[i64; 7]>(Variant(_13.0, 1), 5),fld6: Move(Field::<*const i64>(Variant(_13.0, 1), 6)) };
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(12_usize, 29_usize, Move(_29), 2_usize, Move(_2), 21_usize, Move(_21), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(12_usize, 3_usize, Move(_3), 12_usize, Move(_12), 28_usize, Move(_28), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(12_usize, 7_usize, Move(_7), 10_usize, Move(_10), 5_usize, Move(_5), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [usize; 8],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u128,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> i16 {
mir! {
type RET = i16;
let _12: usize;
let _13: bool;
let _14: i8;
let _15: ();
let _16: ();
{
_6 = 9343891057461561667_u64 as u128;
_9 = 13141109862550961552_usize as isize;
_9 = (-1_i8) as isize;
RET = 11699_i16;
_3 = _6 as isize;
RET = -22293_i16;
RET = -6567_i16;
_5 = -_4;
_12 = 6_usize << _5;
_11 = _5;
_1 = [_12,_12,_12,_12,_12,_12,_12,_12];
_5 = _11;
_6 = _4 as u128;
_2 = 489430637_u32 as isize;
_5 = _4 - _11;
RET = 12068_i16 << _4;
_2 = -_5;
_7 = false as isize;
_6 = !263468858239079227301446208171013438614_u128;
_8 = _11;
_9 = '\u{c48c2}' as isize;
_12 = 1829772092138348101_usize ^ 3456770297109622372_usize;
_8 = 20101_u16 as isize;
_11 = 1298081447_i32 as isize;
_12 = !12076416600401281755_usize;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(13_usize, 3_usize, Move(_3), 10_usize, Move(_10), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(13_usize, 12_usize, Move(_12), 6_usize, Move(_6), 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i64,mut _2: isize,mut _3: [u64; 5],mut _4: i128) -> usize {
mir! {
type RET = usize;
let _5: f64;
let _6: &'static *const u128;
let _7: &'static [usize; 8];
let _8: char;
let _9: f64;
let _10: *const usize;
let _11: [u64; 2];
let _12: u8;
let _13: isize;
let _14: *const u128;
let _15: i8;
let _16: isize;
let _17: *mut Adt47;
let _18: isize;
let _19: u16;
let _20: f32;
let _21: *mut u32;
let _22: *const usize;
let _23: f32;
let _24: ();
let _25: ();
{
_3 = [17938664257304042760_u64,17196180886129102455_u64,6127900526171198525_u64,11936909385857576608_u64,15266836153214419734_u64];
RET = !6_usize;
RET = !2340133784115722821_usize;
RET = !15721873824398744339_usize;
_3 = [8701423397792127903_u64,3044658202170985375_u64,17167182360483555424_u64,12960491018665899140_u64,15648820221786515784_u64];
_1 = '\u{ccad}' as i64;
_1 = (-278724026126537336_i64);
RET = 1483500012131860960_usize;
_4 = -(-159860862172522645463105051531509687024_i128);
RET = _4 as usize;
RET = 18030327799935758155_usize;
_2 = 15763475124805834075_u64 as isize;
_2 = 6472772979048685660_u64 as isize;
_4 = 156914737369578930101923840095229307345_i128;
_3 = [7310576010149298459_u64,10465971587814519311_u64,53470045791699524_u64,16697649589077362377_u64,17273265599209073545_u64];
RET = 6951715700912817298_u64 as usize;
_1 = 2504342687_u32 as i64;
_4 = -(-125539447146489945377345710502444899716_i128);
_3 = [3696402584720272603_u64,11140456979174857962_u64,3398928597110114311_u64,11946481797967862845_u64,1421578591158818670_u64];
RET = true as usize;
_2 = !(-32_isize);
RET = 14188161555815501532_usize | 2_usize;
_2 = !9223372036854775807_isize;
_2 = -9223372036854775807_isize;
RET = 13986322074261811318_usize;
_1 = 9198765154666736350_u64 as i64;
_1 = (-6561583980513502608_i64) + (-3596548439766517039_i64);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
13986322074261811318 => bb7,
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
RET = 8695484610470222022_usize;
RET = 13979724141330904390_usize >> _1;
_3 = [15147476008655826880_u64,17034988396566128615_u64,5852375808487982554_u64,3103199998211771634_u64,14835096573744883052_u64];
_1 = (-123_i8) as i64;
_1 = 52281_u16 as i64;
_4 = (-38816385195483521771749163788998120512_i128);
_2 = 9223372036854775807_isize & (-9223372036854775808_isize);
_2 = (-93_isize);
_3 = [13244165063803452369_u64,16168373222863499675_u64,9028546347569793410_u64,7667825733920042599_u64,5441898319755009112_u64];
_2 = 9223372036854775807_isize;
RET = !1814740561794245031_usize;
_2 = false as isize;
_3 = [10778837804838284271_u64,2477541797642882221_u64,12205459807990651894_u64,14628402828563626046_u64,8351398195578380309_u64];
RET = !7_usize;
_5 = _1 as f64;
_3 = [1161154101145732278_u64,3663018504744386904_u64,9582762365989264077_u64,2733094011077979011_u64,11652941160682284097_u64];
_1 = 220633950614402470_i64 + (-842670031637223334_i64);
_1 = (-2352305643177838912_i64);
RET = !8572365691035979149_usize;
RET = 2440608086266269304_usize;
_3 = [7057398556659813567_u64,16851364558985299475_u64,4265282733832953050_u64,11664443219164650898_u64,31354627689531712_u64];
RET = 4_usize & 4_usize;
_3 = [8395555598323932600_u64,7908089623954898099_u64,1038204939039602185_u64,3924169898795234895_u64,548588406899113054_u64];
Goto(bb8)
}
bb8 = {
_5 = _1 as f64;
RET = !0_usize;
_1 = !2865760979829174029_i64;
RET = 11887_i16 as usize;
_1 = !6493848424213569991_i64;
_1 = -727514462066707967_i64;
_4 = !(-103769789169016884139525433071413806535_i128);
_5 = 405483765_u32 as f64;
RET = 17335546490749199715_u64 as usize;
RET = !12791067962059228860_usize;
_5 = RET as f64;
_5 = 10205_u16 as f64;
_1 = true as i64;
RET = !15769790019492873081_usize;
RET = 15188412467134744043_usize << _4;
_3 = [3895223841709155101_u64,2088009115414349364_u64,17895275583284211299_u64,2575710694020864082_u64,6383442946185522250_u64];
_5 = _1 as f64;
RET = '\u{d132d}' as usize;
_2 = (-78_isize);
_8 = '\u{a0541}';
_1 = 62169_u16 as i64;
_3 = [10559636693122997944_u64,115724386455182621_u64,1325507059710660721_u64,4541706413807328723_u64,11059221785623623192_u64];
RET = 5_usize;
_1 = !(-8231520070801580352_i64);
_9 = -_5;
RET = !15547303220515076341_usize;
Goto(bb9)
}
bb9 = {
_2 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = 119_i8 as isize;
_9 = _5 - _5;
Goto(bb10)
}
bb10 = {
_9 = 1150755711_i32 as f64;
_3 = [14329855510966874876_u64,10580769240609051641_u64,2634046462001524884_u64,15241143663913172032_u64,10611567317614362788_u64];
_1 = 4496446579340592301_i64;
_1 = 3334740651579643400_i64;
_4 = !111136245146669395451604602766078079670_i128;
_9 = -_5;
_5 = 193_u8 as f64;
_1 = !(-6735129948441964158_i64);
_12 = 52_u8 & 100_u8;
_11 = [8618127816145125784_u64,1083306726355191171_u64];
_1 = !8526048032643247847_i64;
_1 = 1850906460776920479_i64 - 5238835558377482379_i64;
_2 = 9223372036854775807_isize;
_10 = core::ptr::addr_of!(RET);
_12 = 1814653117_u32 as u8;
_9 = _5 * _5;
_12 = 63820_u16 as u8;
_12 = 41_u8;
_1 = _12 as i64;
_8 = '\u{75b28}';
Call(RET = core::intrinsics::transmute(_2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = 0_usize;
_9 = _5;
_8 = '\u{1082f}';
RET = 5_usize;
_3 = [7030090845291421843_u64,548347558409281139_u64,17640814931586480163_u64,1156027068230579382_u64,17778851304106386759_u64];
(*_10) = _8 as usize;
RET = 46869327867448346664655099461025812058_u128 as usize;
_5 = _9 - _9;
_11 = [15344984943898850595_u64,2083163939246660512_u64];
_9 = _1 as f64;
_4 = (-107703351427538385600630782489096315383_i128) ^ 20786180870895308746704828266089357892_i128;
_15 = 604119960802988949_u64 as i8;
_11 = [3064753184058637231_u64,10899071490581179186_u64];
_2 = _4 as isize;
(*_10) = !4_usize;
_2 = 9223372036854775807_isize;
_16 = _2;
RET = 6693611313127567912_usize + 7138532313054983652_usize;
_8 = '\u{f71a4}';
(*_10) = 1_usize;
_19 = (-1018258720_i32) as u16;
_19 = 60670_u16;
_11 = [_3[RET],_3[RET]];
_5 = _15 as f64;
Goto(bb12)
}
bb12 = {
_13 = !_2;
(*_10) = 15765068813129805202_usize >> _2;
match _19 {
0 => bb8,
1 => bb13,
2 => bb14,
3 => bb15,
60670 => bb17,
_ => bb16
}
}
bb13 = {
RET = 0_usize;
_9 = _5;
_8 = '\u{1082f}';
RET = 5_usize;
_3 = [7030090845291421843_u64,548347558409281139_u64,17640814931586480163_u64,1156027068230579382_u64,17778851304106386759_u64];
(*_10) = _8 as usize;
RET = 46869327867448346664655099461025812058_u128 as usize;
_5 = _9 - _9;
_11 = [15344984943898850595_u64,2083163939246660512_u64];
_9 = _1 as f64;
_4 = (-107703351427538385600630782489096315383_i128) ^ 20786180870895308746704828266089357892_i128;
_15 = 604119960802988949_u64 as i8;
_11 = [3064753184058637231_u64,10899071490581179186_u64];
_2 = _4 as isize;
(*_10) = !4_usize;
_2 = 9223372036854775807_isize;
_16 = _2;
RET = 6693611313127567912_usize + 7138532313054983652_usize;
_8 = '\u{f71a4}';
(*_10) = 1_usize;
_19 = (-1018258720_i32) as u16;
_19 = 60670_u16;
_11 = [_3[RET],_3[RET]];
_5 = _15 as f64;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_2 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = 119_i8 as isize;
_9 = _5 - _5;
Goto(bb10)
}
bb16 = {
Return()
}
bb17 = {
_5 = _9;
_18 = _16 * _16;
_12 = !228_u8;
(*_10) = 4519090183109408820_usize;
_10 = core::ptr::addr_of!(RET);
_11 = [11917110866478141733_u64,9435229449246208612_u64];
RET = _12 as usize;
_10 = core::ptr::addr_of!((*_10));
_20 = 750800653_i32 as f32;
_16 = 1022693291_u32 as isize;
(*_10) = !4_usize;
_8 = '\u{6de9a}';
(*_10) = 122771815658306566545930371592601849519_u128 as usize;
_4 = 84447443125130792930797591656985789227_i128 << _15;
_5 = 12369624523398331577_u64 as f64;
_4 = -(-151038676123974026035118523743949017858_i128);
_2 = _18 | _18;
Goto(bb18)
}
bb18 = {
Call(_24 = dump_var(14_usize, 11_usize, Move(_11), 8_usize, Move(_8), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_24 = dump_var(14_usize, 18_usize, Move(_18), 19_usize, Move(_19), 25_usize, _25, 25_usize, _25), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize) -> i32 {
mir! {
type RET = i32;
let _2: *const usize;
let _3: f64;
let _4: Adt32;
let _5: u128;
let _6: i16;
let _7: bool;
let _8: *mut [u16; 2];
let _9: isize;
let _10: (Adt32, (u128, f64), *const Adt36);
let _11: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _12: [u8; 2];
let _13: f64;
let _14: *const usize;
let _15: (char,);
let _16: [i128; 5];
let _17: &'static Adt17;
let _18: u8;
let _19: char;
let _20: [u64; 2];
let _21: [u8; 2];
let _22: [usize; 8];
let _23: u32;
let _24: *const [i128; 4];
let _25: ([u128; 3], &'static i16, i32, &'static [i16; 3]);
let _26: char;
let _27: bool;
let _28: char;
let _29: f64;
let _30: (u16, isize);
let _31: [u16; 6];
let _32: f64;
let _33: (u16,);
let _34: isize;
let _35: [u64; 5];
let _36: &'static [u16; 6];
let _37: char;
let _38: bool;
let _39: char;
let _40: ();
let _41: ();
{
RET = (-2041257349_i32);
RET = 678135865_i32;
_1 = 9223372036854775807_isize;
match _1 {
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
_3 = 69753811811458879972376873083757449782_u128 as f64;
_1 = 9223372036854775807_isize & 48_isize;
_3 = _1 as f64;
_6 = (-5907_i16);
_6 = 15823607813558658923_u64 as i16;
_5 = 190891444733699353344607240615726886125_u128;
_6 = !25496_i16;
_1 = (-9223372036854775808_isize);
_5 = !33923986561088185638243702327837618927_u128;
_4 = Adt32::Variant2 { fld0: _5 };
place!(Field::<u128>(Variant(_4, 2), 0)) = _5;
_4 = Adt32::Variant2 { fld0: _5 };
_3 = _6 as f64;
_1 = (-110_isize) >> Field::<u128>(Variant(_4, 2), 0);
RET = 930612417_i32 & (-1396114219_i32);
place!(Field::<u128>(Variant(_4, 2), 0)) = !_5;
_5 = Field::<u128>(Variant(_4, 2), 0);
_7 = !false;
_3 = 45105_u16 as f64;
_7 = false;
RET = !2116165836_i32;
Goto(bb4)
}
bb4 = {
place!(Field::<u128>(Variant(_4, 2), 0)) = !_5;
place!(Field::<u128>(Variant(_4, 2), 0)) = 1611317682_u32 as u128;
_3 = _1 as f64;
_11.1.1 = [_6,_6,_6];
RET = (-88735438_i32);
match RET {
340282366920938463463374607431679476018 => bb5,
_ => bb2
}
}
bb5 = {
SetDiscriminant(_4, 2);
_11.2.2 = 3091483606_u32 as isize;
_12 = [180_u8,201_u8];
_13 = _3 + _3;
_11.2.0.0 = 2109_u16;
_11.1.1 = [_6,_6,_6];
_9 = -_1;
_10.0 = Adt32::Variant2 { fld0: _5 };
_15 = ('\u{b168a}',);
RET = 710558580_i32 >> _6;
_13 = -_3;
_11.2.1 = (-64_i8) as f32;
_11.2.1 = 107_i8 as f32;
_11.2.0.1 = _1;
Goto(bb6)
}
bb6 = {
_13 = _3;
_15.0 = '\u{b016}';
_4 = Adt32::Variant2 { fld0: Field::<u128>(Variant(_10.0, 2), 0) };
_4 = Move(_10.0);
_10.1 = (_5, _13);
_15 = ('\u{9d579}',);
match _11.2.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
2109 => bb10,
_ => bb9
}
}
bb7 = {
SetDiscriminant(_4, 2);
_11.2.2 = 3091483606_u32 as isize;
_12 = [180_u8,201_u8];
_13 = _3 + _3;
_11.2.0.0 = 2109_u16;
_11.1.1 = [_6,_6,_6];
_9 = -_1;
_10.0 = Adt32::Variant2 { fld0: _5 };
_15 = ('\u{b168a}',);
RET = 710558580_i32 >> _6;
_13 = -_3;
_11.2.1 = (-64_i8) as f32;
_11.2.1 = 107_i8 as f32;
_11.2.0.1 = _1;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
_3 = 69753811811458879972376873083757449782_u128 as f64;
_1 = 9223372036854775807_isize & 48_isize;
_3 = _1 as f64;
_6 = (-5907_i16);
_6 = 15823607813558658923_u64 as i16;
_5 = 190891444733699353344607240615726886125_u128;
_6 = !25496_i16;
_1 = (-9223372036854775808_isize);
_5 = !33923986561088185638243702327837618927_u128;
_4 = Adt32::Variant2 { fld0: _5 };
place!(Field::<u128>(Variant(_4, 2), 0)) = _5;
_4 = Adt32::Variant2 { fld0: _5 };
_3 = _6 as f64;
_1 = (-110_isize) >> Field::<u128>(Variant(_4, 2), 0);
RET = 930612417_i32 & (-1396114219_i32);
place!(Field::<u128>(Variant(_4, 2), 0)) = !_5;
_5 = Field::<u128>(Variant(_4, 2), 0);
_7 = !false;
_3 = 45105_u16 as f64;
_7 = false;
RET = !2116165836_i32;
Goto(bb4)
}
bb10 = {
SetDiscriminant(_4, 1);
_7 = !false;
_10.1.0 = !_5;
_20 = [2233484773163455685_u64,3601465859092238008_u64];
place!(Field::<isize>(Variant(_4, 1), 2)) = _1;
place!(Field::<u128>(Variant(_4, 1), 1)) = _3 as u128;
_5 = 161886671046447101203223296527674008342_i128 as u128;
place!(Field::<isize>(Variant(_4, 1), 2)) = _11.2.0.1;
_3 = -_13;
_21 = [97_u8,121_u8];
_18 = 224_u8;
_11.2.0.0 = !2314_u16;
_3 = _10.1.1;
_11.1.1 = [_6,_6,_6];
place!(Field::<i128>(Variant(_4, 1), 3)) = !18678250432340888599984466526034923959_i128;
_25.1 = &_6;
_22 = [1_usize,12749369926385146094_usize,801565029715961895_usize,15932900382109697903_usize,4_usize,1_usize,3_usize,15112398400933979324_usize];
_11.0 = _18 as u32;
place!(Field::<[i64; 7]>(Variant(_4, 1), 5)) = [(-3032654346694487825_i64),(-523782391533199739_i64),(-9018189757363797823_i64),(-2185515847222353567_i64),(-9151275559094286938_i64),(-3109750649807030942_i64),(-7292287981810964011_i64)];
_16 = [Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3)];
_25.2 = !RET;
_22 = [2574502882689899993_usize,4_usize,16956125014065484543_usize,1_usize,0_usize,0_usize,16304032589614765033_usize,6_usize];
place!(Field::<[i64; 7]>(Variant(_4, 1), 5)) = [7728159222628714610_i64,1612700176388813328_i64,(-7553434527910621321_i64),(-7689164707644616845_i64),4115899758633299057_i64,(-6204860364758307033_i64),2482677639681069245_i64];
place!(Field::<u128>(Variant(_4, 1), 1)) = _10.1.0 & _10.1.0;
_11.2.3 = _11.2.0.0;
Goto(bb11)
}
bb11 = {
_11.1.0 = _15.0;
place!(Field::<[usize; 8]>(Variant(_4, 1), 4)) = [7_usize,7069591333331262241_usize,4222495606057669671_usize,16630217324595871853_usize,2_usize,7040946250719249846_usize,12639976035890638907_usize,0_usize];
_25.3 = &_11.1.1;
_23 = _11.0 & _11.0;
_19 = _15.0;
place!(Field::<u128>(Variant(_4, 1), 1)) = _10.1.0;
place!(Field::<[usize; 8]>(Variant(_4, 1), 4)) = [5_usize,12459734781068322756_usize,384518002075220206_usize,3015087181723876846_usize,15064757121435887248_usize,0_usize,0_usize,16841500616075334806_usize];
_11.2.3 = _11.2.0.0;
_19 = _15.0;
_11.2.3 = 4253399105056391199_u64 as u16;
_25.0 = [_5,Field::<u128>(Variant(_4, 1), 1),_10.1.0];
place!(Field::<u128>(Variant(_4, 1), 1)) = _10.1.0;
place!(Field::<f64>(Variant(_4, 1), 0)) = _10.1.1 + _10.1.1;
_15.0 = _11.1.0;
_11.2.1 = _11.2.0.1 as f32;
_20 = [1690131018801961501_u64,8032266368832443835_u64];
_20 = [5498068933433807092_u64,17610580315042064042_u64];
_10.1.0 = !Field::<u128>(Variant(_4, 1), 1);
_11.2.2 = _11.2.3 as isize;
_3 = -Field::<f64>(Variant(_4, 1), 0);
_11.2.0 = (_11.2.3, Field::<isize>(Variant(_4, 1), 2));
Goto(bb12)
}
bb12 = {
_11.2.0 = (_11.2.3, _1);
_19 = _11.1.0;
place!(Field::<u128>(Variant(_4, 1), 1)) = !_5;
_13 = Field::<f64>(Variant(_4, 1), 0) * _3;
_22 = Field::<[usize; 8]>(Variant(_4, 1), 4);
_18 = 27_u8;
place!(Field::<isize>(Variant(_4, 1), 2)) = _1;
_16 = [Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3)];
_11.2.0.1 = _9;
_16 = [Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3)];
_16 = [Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3),Field::<i128>(Variant(_4, 1), 3)];
RET = _25.2 >> _11.2.0.1;
_11.1.0 = _15.0;
_26 = _19;
_10.1.1 = -Field::<f64>(Variant(_4, 1), 0);
place!(Field::<[usize; 8]>(Variant(_4, 1), 4)) = [3_usize,3_usize,6_usize,0_usize,2_usize,10583199175041884713_usize,14080929094295536758_usize,0_usize];
_29 = _6 as f64;
_30.1 = !_11.2.0.1;
_15.0 = _26;
_30.1 = _13 as isize;
_25.2 = RET & RET;
_32 = -_10.1.1;
Goto(bb13)
}
bb13 = {
place!(Field::<[i64; 7]>(Variant(_4, 1), 5)) = [4937914470367141929_i64,(-7950296221435628506_i64),1937202954613909842_i64,(-7472623878122442025_i64),5307244160962942825_i64,2407475322910895928_i64,8825437145619607692_i64];
_33.0 = _11.2.0.0;
_30 = (_33.0, _11.2.2);
_9 = !_11.2.2;
_12 = [_18,_18];
_21 = [_18,_18];
_11.2.2 = 142127778641344440_i64 as isize;
_11.2.0 = _30;
_11.1.0 = _26;
_12 = _21;
_11.1.1 = [_6,_6,_6];
_29 = _32 - _3;
_10.1.0 = Field::<u128>(Variant(_4, 1), 1);
_25.1 = &_6;
_25.1 = &_6;
_18 = !132_u8;
_12 = [_18,_18];
_33.0 = _25.2 as u16;
_12 = _21;
Goto(bb14)
}
bb14 = {
_21 = [_18,_18];
_19 = _11.1.0;
_3 = _10.1.1 + Field::<f64>(Variant(_4, 1), 0);
_22 = Field::<[usize; 8]>(Variant(_4, 1), 4);
_27 = !_7;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(15_usize, 27_usize, Move(_27), 18_usize, Move(_18), 1_usize, Move(_1), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(15_usize, 20_usize, Move(_20), 23_usize, Move(_23), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(15_usize, 21_usize, Move(_21), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16() -> f64 {
mir! {
type RET = f64;
let _1: i8;
let _2: isize;
let _3: isize;
let _4: f32;
let _5: [u16; 6];
let _6: (u32, (char, [i16; 3]), ((u16, isize), f32, isize, u16));
let _7: isize;
let _8: *const [i128; 4];
let _9: *const usize;
let _10: [i128; 4];
let _11: (bool, u64, f64, u64);
let _12: i64;
let _13: *mut u16;
let _14: f64;
let _15: (*const i64, Adt47);
let _16: i8;
let _17: [usize; 8];
let _18: i128;
let _19: bool;
let _20: ();
let _21: ();
{
RET = 70_u8 as f64;
RET = 134_u8 as f64;
RET = 12453654357217215908_u64 as f64;
RET = 4209127695_u32 as f64;
RET = 3_usize as f64;
RET = 9223372036854775807_isize as f64;
_1 = -(-79_i8);
Goto(bb1)
}
bb1 = {
RET = 1422734190_i32 as f64;
RET = 120_u8 as f64;
_1 = (-28_i8) * (-89_i8);
RET = 72241217771781887514800210011784372784_i128 as f64;
_1 = 115159444438246404409203329904855356178_u128 as i8;
_2 = !(-9223372036854775808_isize);
_2 = -(-9223372036854775808_isize);
Call(_1 = fn17(_2, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-18313_i16) as f64;
_2 = !9223372036854775807_isize;
Call(_2 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = (-120_i8) >> _2;
_3 = 259807637889144315106875771741037292185_u128 as isize;
RET = 164_u8 as f64;
_2 = !_3;
_1 = 3_i8;
RET = 245_u8 as f64;
RET = 2_usize as f64;
RET = _1 as f64;
_2 = _3;
RET = 6711_u16 as f64;
_3 = (-546589028_i32) as isize;
_3 = true as isize;
RET = 19612_u16 as f64;
RET = 29882_u16 as f64;
RET = _2 as f64;
RET = 1966304859_i32 as f64;
_4 = _3 as f32;
RET = (-156299686456420948987250282145998605793_i128) as f64;
_3 = _4 as isize;
_5 = [9686_u16,36697_u16,56179_u16,53471_u16,49892_u16,37721_u16];
_3 = _2 + _2;
_6.2.2 = _3 << _1;
_6.2.0.1 = _6.2.2 * _2;
_1 = (-26_i8) << _2;
_7 = 27_u8 as isize;
Call(_9 = fn18(_5, _5, _6.2.2, _7, _5, _4, _6.2.2, _2, RET, _6.2.0.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.2 = RET + RET;
_7 = _6.2.2 - _6.2.0.1;
_10 = [(-146117103938812779814602302775369332295_i128),74061600701830875617790391031301077123_i128,54658774564911293157328508490046047070_i128,89631043839175792764213035846664710307_i128];
_1 = !(-43_i8);
_11.0 = false;
_6.2.0.0 = !4263_u16;
_4 = (-3847912768116542886_i64) as f32;
_11.0 = !true;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
_6.2.0.1 = !_7;
Goto(bb5)
}
bb5 = {
_6.2.0.0 = _6.2.2 as u16;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
_15.1.fld0 = (3_usize, 482733828_i32);
_6.2.0.0 = 39512_u16;
_8 = core::ptr::addr_of!(_10);
_15.1.fld1 = ('\u{eee88}',);
_6.1.1 = [(-89_i16),(-32049_i16),17438_i16];
_6.2.0 = (27524_u16, _6.2.2);
_6.2.1 = 2658810976377354443_u64 as f32;
_6.0 = !3795832233_u32;
_6.2.1 = _4;
_8 = core::ptr::addr_of!(_10);
Call(_6.2.0.0 = core::intrinsics::bswap(34560_u16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6.2.3 = !_6.2.0.0;
_16 = _1;
(*_13) = !_6.2.3;
_6.2.2 = _6.2.0.1 | _3;
_15.1.fld0 = (3_usize, 1882608921_i32);
_11.3 = 11052121951393325592_u64 >> (*_13);
_6.2.0 = (_6.2.3, _7);
_13 = core::ptr::addr_of_mut!((*_13));
_15.1.fld0.1 = (-1545355658_i32);
RET = _6.2.3 as f64;
_15.1.fld0 = (6_usize, (-1171093771_i32));
_16 = _4 as i8;
_6.2.0.0 = _6.2.3 | _6.2.3;
_11.3 = 5454674020934818856_u64 + 6900890328041528853_u64;
_3 = _11.3 as isize;
_11.2 = 10997_i16 as f64;
_5 = [(*_13),(*_13),(*_13),(*_13),(*_13),(*_13)];
_11.1 = _15.1.fld0.1 as u64;
_5 = [_6.2.0.0,(*_13),_6.2.0.0,(*_13),(*_13),_6.2.3];
_11.2 = RET;
(*_13) = 51_u8 as u16;
_3 = !_6.2.0.1;
match _15.1.fld0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
6 => bb7,
_ => bb4
}
}
bb7 = {
_12 = 497945717213836601_i64;
_6.0 = 514688879_u32;
_17 = [_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0];
_6.1.0 = _15.1.fld1.0;
_10 = [(-101904989616783747065259719184070139354_i128),97500788036747216290048439445586714426_i128,125043223999053896805900955382859414295_i128,66915360513126532993452702253793259710_i128];
_15.1.fld0.0 = 1_usize;
_1 = _16 << _3;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
match _15.1.fld0.1 {
0 => bb4,
1 => bb2,
340282366920938463463374607430597117685 => bb8,
_ => bb6
}
}
bb8 = {
_6.2.0.0 = !_6.2.3;
_8 = core::ptr::addr_of!((*_8));
_15.1.fld0.0 = 3_usize;
_6.2.0.1 = -_7;
_16 = _1;
_6.2.0.0 = _6.2.3 << _6.2.2;
match _15.1.fld0.1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607430597117685 => bb15,
_ => bb14
}
}
bb9 = {
_12 = 497945717213836601_i64;
_6.0 = 514688879_u32;
_17 = [_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0,_15.1.fld0.0];
_6.1.0 = _15.1.fld1.0;
_10 = [(-101904989616783747065259719184070139354_i128),97500788036747216290048439445586714426_i128,125043223999053896805900955382859414295_i128,66915360513126532993452702253793259710_i128];
_15.1.fld0.0 = 1_usize;
_1 = _16 << _3;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
match _15.1.fld0.1 {
0 => bb4,
1 => bb2,
340282366920938463463374607430597117685 => bb8,
_ => bb6
}
}
bb10 = {
RET = 1422734190_i32 as f64;
RET = 120_u8 as f64;
_1 = (-28_i8) * (-89_i8);
RET = 72241217771781887514800210011784372784_i128 as f64;
_1 = 115159444438246404409203329904855356178_u128 as i8;
_2 = !(-9223372036854775808_isize);
_2 = -(-9223372036854775808_isize);
Call(_1 = fn17(_2, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_6.2.0.0 = _6.2.2 as u16;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
_15.1.fld0 = (3_usize, 482733828_i32);
_6.2.0.0 = 39512_u16;
_8 = core::ptr::addr_of!(_10);
_15.1.fld1 = ('\u{eee88}',);
_6.1.1 = [(-89_i16),(-32049_i16),17438_i16];
_6.2.0 = (27524_u16, _6.2.2);
_6.2.1 = 2658810976377354443_u64 as f32;
_6.0 = !3795832233_u32;
_6.2.1 = _4;
_8 = core::ptr::addr_of!(_10);
Call(_6.2.0.0 = core::intrinsics::bswap(34560_u16), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_11.2 = RET + RET;
_7 = _6.2.2 - _6.2.0.1;
_10 = [(-146117103938812779814602302775369332295_i128),74061600701830875617790391031301077123_i128,54658774564911293157328508490046047070_i128,89631043839175792764213035846664710307_i128];
_1 = !(-43_i8);
_11.0 = false;
_6.2.0.0 = !4263_u16;
_4 = (-3847912768116542886_i64) as f32;
_11.0 = !true;
_13 = core::ptr::addr_of_mut!(_6.2.0.0);
_6.2.0.1 = !_7;
Goto(bb5)
}
bb13 = {
_1 = (-120_i8) >> _2;
_3 = 259807637889144315106875771741037292185_u128 as isize;
RET = 164_u8 as f64;
_2 = !_3;
_1 = 3_i8;
RET = 245_u8 as f64;
RET = 2_usize as f64;
RET = _1 as f64;
_2 = _3;
RET = 6711_u16 as f64;
_3 = (-546589028_i32) as isize;
_3 = true as isize;
RET = 19612_u16 as f64;
RET = 29882_u16 as f64;
RET = _2 as f64;
RET = 1966304859_i32 as f64;
_4 = _3 as f32;
RET = (-156299686456420948987250282145998605793_i128) as f64;
_3 = _4 as isize;
_5 = [9686_u16,36697_u16,56179_u16,53471_u16,49892_u16,37721_u16];
_3 = _2 + _2;
_6.2.2 = _3 << _1;
_6.2.0.1 = _6.2.2 * _2;
_1 = (-26_i8) << _2;
_7 = 27_u8 as isize;
Call(_9 = fn18(_5, _5, _6.2.2, _7, _5, _4, _6.2.2, _2, RET, _6.2.0.1), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
RET = (-18313_i16) as f64;
_2 = !9223372036854775807_isize;
Call(_2 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
RET = -_11.2;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(16_usize, 3_usize, Move(_3), 7_usize, Move(_7), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> i8 {
mir! {
type RET = i8;
let _5: i8;
let _6: *const u128;
let _7: &'static i8;
let _8: (usize, i32);
let _9: f32;
let _10: isize;
let _11: [u64; 8];
let _12: &'static (usize, i32);
let _13: *const Adt61;
let _14: [i128; 5];
let _15: [i16; 3];
let _16: f32;
let _17: *const i16;
let _18: &'static [u16; 6];
let _19: i64;
let _20: i8;
let _21: ([u128; 3], &'static i16, i32, &'static [i16; 3]);
let _22: Adt17;
let _23: f64;
let _24: (*const Adt36,);
let _25: bool;
let _26: [u64; 5];
let _27: isize;
let _28: bool;
let _29: f64;
let _30: ();
let _31: ();
{
_1 = (-8906130379952460305_i64) as isize;
_4 = 14924_i16 as isize;
RET = -7_i8;
RET = 62_i8 + 39_i8;
RET = 169319224949864437072561236006368254037_i128 as i8;
Goto(bb1)
}
bb1 = {
_1 = _2 * _4;
_5 = 47995_u16 as i8;
RET = 26528_u16 as i8;
_2 = _1;
RET = -_5;
_4 = '\u{5124b}' as isize;
_5 = RET << _3;
_4 = -_1;
Goto(bb2)
}
bb2 = {
_8.0 = 3_u8 as usize;
_8.1 = 71129713919176968889961904430084681808_u128 as i32;
_3 = _1;
_1 = 4235322024_u32 as isize;
RET = -_5;
_1 = !_4;
_11 = [15467231171333092041_u64,2648707477852484949_u64,13092406425582245464_u64,9914265891320750503_u64,7307054365350535535_u64,4271001151422409919_u64,10484787573135619738_u64,15878053881710731043_u64];
_7 = &RET;
_4 = 126934989476453343184328160008327141232_i128 as isize;
Goto(bb3)
}
bb3 = {
_10 = _1;
_3 = _1;
_7 = &(*_7);
_8.1 = 7758520988686201502_i64 as i32;
_10 = (-21096327086926107059986005907722734388_i128) as isize;
_4 = _2;
_3 = _1 >> (*_7);
_10 = _8.1 as isize;
_12 = &_8;
_5 = (*_7) ^ RET;
_1 = !_4;
_7 = &(*_7);
_3 = _8.0 as isize;
_5 = 37428077116518295862187621379682540791_i128 as i8;
_3 = _1 ^ _10;
RET = true as i8;
_1 = (*_12).1 as isize;
_8 = (2782695727468471538_usize, (-1880099558_i32));
_7 = &_5;
_8 = (11523174157650080520_usize, 860653091_i32);
_4 = _10 & _3;
RET = _5 ^ (*_7);
_2 = _4;
match _8.0 {
0 => bb4,
1 => bb5,
2 => bb6,
11523174157650080520 => bb8,
_ => bb7
}
}
bb4 = {
_8.0 = 3_u8 as usize;
_8.1 = 71129713919176968889961904430084681808_u128 as i32;
_3 = _1;
_1 = 4235322024_u32 as isize;
RET = -_5;
_1 = !_4;
_11 = [15467231171333092041_u64,2648707477852484949_u64,13092406425582245464_u64,9914265891320750503_u64,7307054365350535535_u64,4271001151422409919_u64,10484787573135619738_u64,15878053881710731043_u64];
_7 = &RET;
_4 = 126934989476453343184328160008327141232_i128 as isize;
Goto(bb3)
}
bb5 = {
_1 = _2 * _4;
_5 = 47995_u16 as i8;
RET = 26528_u16 as i8;
_2 = _1;
RET = -_5;
_4 = '\u{5124b}' as isize;
_5 = RET << _3;
_4 = -_1;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_16 = 26363_i16 as f32;
_15 = [20608_i16,(-30015_i16),3667_i16];
_12 = &_8;
_5 = RET ^ RET;
_2 = _3;
_12 = &(*_12);
_1 = _2 >> _5;
_3 = _1;
_8.1 = (-6545117022201501811_i64) as i32;
_9 = _16;
_12 = &_8;
_2 = 13282_i16 as isize;
_2 = '\u{ebf7e}' as isize;
_7 = &RET;
_9 = -_16;
_10 = _1;
_2 = _1;
RET = _5 & _5;
_12 = &(*_12);
_19 = _10 as i64;
RET = _5 * _5;
Call(_2 = core::intrinsics::bswap(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9 = _16;
RET = true as i8;
_19 = 6573353047965933457_i64 + (-342442872230822333_i64);
_20 = _5;
Goto(bb10)
}
bb10 = {
_12 = &(*_12);
_21.2 = (*_12).1 >> _8.1;
_21.0 = [90137477585638332507123934852545544193_u128,303596571632004793278535639689219621332_u128,272744125651943952087888601030026337742_u128];
RET = _21.2 as i8;
_4 = -_1;
_8 = (14439193473214836397_usize, _21.2);
_21.2 = _8.1;
_12 = &_8;
_7 = &_20;
_9 = _16;
_4 = _1;
_22.fld0 = -_21.2;
_21.3 = &_15;
_5 = (*_7);
Goto(bb11)
}
bb11 = {
_21.3 = &_15;
_19 = (*_12).0 as i64;
_23 = 1521321813_u32 as f64;
_10 = _1;
_14 = [110270110606226446053357822623644111950_i128,(-98934323027242627118017065891528040215_i128),67880678941922527125863521008675725781_i128,137631066475408730725042096563663241889_i128,81589669041872501208637820183396825077_i128];
_21.3 = &_15;
_21.2 = -(*_12).1;
_11 = [2440181098249935912_u64,12941704550451598656_u64,9825395382728776576_u64,7723061204895315064_u64,4481400170364818913_u64,12116010845770109333_u64,10603219053904004618_u64,1120503265675475700_u64];
_12 = &(*_12);
_21.3 = &_15;
_11 = [3755291047334717837_u64,12579664967944173056_u64,13903807208429467348_u64,7229939185621905051_u64,1527151907741900822_u64,18047723935426183296_u64,13138947126917810832_u64,4865993143123080517_u64];
Goto(bb12)
}
bb12 = {
_7 = &(*_7);
_21.0 = [279877628389133977240972453693150867764_u128,103752751303431581121073392009667691489_u128,182545914036028183934128494982294476557_u128];
_12 = &_8;
_22.fld0 = !(*_12).1;
RET = _5;
_21.3 = &_15;
RET = -_20;
_12 = &(*_12);
match _8.0 {
0 => bb7,
1 => bb6,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
14439193473214836397 => bb19,
_ => bb18
}
}
bb13 = {
_21.3 = &_15;
_19 = (*_12).0 as i64;
_23 = 1521321813_u32 as f64;
_10 = _1;
_14 = [110270110606226446053357822623644111950_i128,(-98934323027242627118017065891528040215_i128),67880678941922527125863521008675725781_i128,137631066475408730725042096563663241889_i128,81589669041872501208637820183396825077_i128];
_21.3 = &_15;
_21.2 = -(*_12).1;
_11 = [2440181098249935912_u64,12941704550451598656_u64,9825395382728776576_u64,7723061204895315064_u64,4481400170364818913_u64,12116010845770109333_u64,10603219053904004618_u64,1120503265675475700_u64];
_12 = &(*_12);
_21.3 = &_15;
_11 = [3755291047334717837_u64,12579664967944173056_u64,13903807208429467348_u64,7229939185621905051_u64,1527151907741900822_u64,18047723935426183296_u64,13138947126917810832_u64,4865993143123080517_u64];
Goto(bb12)
}
bb14 = {
_8.0 = 3_u8 as usize;
_8.1 = 71129713919176968889961904430084681808_u128 as i32;
_3 = _1;
_1 = 4235322024_u32 as isize;
RET = -_5;
_1 = !_4;
_11 = [15467231171333092041_u64,2648707477852484949_u64,13092406425582245464_u64,9914265891320750503_u64,7307054365350535535_u64,4271001151422409919_u64,10484787573135619738_u64,15878053881710731043_u64];
_7 = &RET;
_4 = 126934989476453343184328160008327141232_i128 as isize;
Goto(bb3)
}
bb15 = {
_9 = _16;
RET = true as i8;
_19 = 6573353047965933457_i64 + (-342442872230822333_i64);
_20 = _5;
Goto(bb10)
}
bb16 = {
_10 = _1;
_3 = _1;
_7 = &(*_7);
_8.1 = 7758520988686201502_i64 as i32;
_10 = (-21096327086926107059986005907722734388_i128) as isize;
_4 = _2;
_3 = _1 >> (*_7);
_10 = _8.1 as isize;
_12 = &_8;
_5 = (*_7) ^ RET;
_1 = !_4;
_7 = &(*_7);
_3 = _8.0 as isize;
_5 = 37428077116518295862187621379682540791_i128 as i8;
_3 = _1 ^ _10;
RET = true as i8;
_1 = (*_12).1 as isize;
_8 = (2782695727468471538_usize, (-1880099558_i32));
_7 = &_5;
_8 = (11523174157650080520_usize, 860653091_i32);
_4 = _10 & _3;
RET = _5 ^ (*_7);
_2 = _4;
match _8.0 {
0 => bb4,
1 => bb5,
2 => bb6,
11523174157650080520 => bb8,
_ => bb7
}
}
bb17 = {
_8.0 = 3_u8 as usize;
_8.1 = 71129713919176968889961904430084681808_u128 as i32;
_3 = _1;
_1 = 4235322024_u32 as isize;
RET = -_5;
_1 = !_4;
_11 = [15467231171333092041_u64,2648707477852484949_u64,13092406425582245464_u64,9914265891320750503_u64,7307054365350535535_u64,4271001151422409919_u64,10484787573135619738_u64,15878053881710731043_u64];
_7 = &RET;
_4 = 126934989476453343184328160008327141232_i128 as isize;
Goto(bb3)
}
bb18 = {
_1 = _2 * _4;
_5 = 47995_u16 as i8;
RET = 26528_u16 as i8;
_2 = _1;
RET = -_5;
_4 = '\u{5124b}' as isize;
_5 = RET << _3;
_4 = -_1;
Goto(bb2)
}
bb19 = {
_28 = _20 >= (*_7);
_12 = &_8;
_21.0 = [288327709117362360303064947320409157421_u128,252773501134079386736850687285149304899_u128,333110726536583230652976331801957126010_u128];
_7 = &(*_7);
_26 = [4146928417817767010_u64,12590004169334366431_u64,1431936120332865272_u64,16009165000147071420_u64,15407695360549792834_u64];
_8.1 = _21.2;
_2 = 14000155781668508252_u64 as isize;
_1 = 5177734319054527301_u64 as isize;
_8.1 = _8.0 as i32;
_7 = &(*_7);
_12 = &_8;
_21.3 = &_15;
Goto(bb20)
}
bb20 = {
Call(_30 = dump_var(17_usize, 4_usize, Move(_4), 5_usize, Move(_5), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_30 = dump_var(17_usize, 26_usize, Move(_26), 1_usize, Move(_1), 10_usize, Move(_10), 31_usize, _31), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [u16; 6],mut _2: [u16; 6],mut _3: isize,mut _4: isize,mut _5: [u16; 6],mut _6: f32,mut _7: isize,mut _8: isize,mut _9: f64,mut _10: isize) -> *const usize {
mir! {
type RET = *const usize;
let _11: usize;
let _12: isize;
let _13: isize;
let _14: isize;
let _15: *const u32;
let _16: *mut Adt47;
let _17: *const *mut i8;
let _18: i64;
let _19: bool;
let _20: &'static Adt17;
let _21: ((u16, isize), f32, isize, u16);
let _22: bool;
let _23: *const Adt36;
let _24: *const Adt36;
let _25: *mut &'static i16;
let _26: &'static i8;
let _27: bool;
let _28: u8;
let _29: (*const i64, Adt47);
let _30: ();
let _31: ();
{
_4 = _7 | _10;
_7 = !_4;
_6 = 136_u8 as f32;
_2 = [31960_u16,40668_u16,42725_u16,11430_u16,16080_u16,63153_u16];
_7 = true as isize;
_8 = !_4;
_3 = 176_u8 as isize;
_3 = _10;
_9 = 6789681801739643044_i64 as f64;
_10 = 12_i8 as isize;
_2 = [15541_u16,41552_u16,1388_u16,11114_u16,6425_u16,57657_u16];
_1 = [60573_u16,6795_u16,4357_u16,50887_u16,34384_u16,44572_u16];
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of!(_11);
_5 = _1;
(*RET) = !12837406228355996263_usize;
_4 = 123_u8 as isize;
Goto(bb2)
}
bb2 = {
(*RET) = 5_usize - 15644244860544557117_usize;
_11 = !10910307463758792544_usize;
_1 = [15633_u16,25361_u16,50825_u16,7168_u16,61275_u16,60992_u16];
_1 = _2;
RET = core::ptr::addr_of!((*RET));
Goto(bb3)
}
bb3 = {
_11 = 17615726111982132076_usize;
RET = core::ptr::addr_of!((*RET));
_5 = [29061_u16,42794_u16,31338_u16,25025_u16,32523_u16,11772_u16];
_9 = _11 as f64;
_11 = !10080835834439351188_usize;
RET = core::ptr::addr_of!((*RET));
_11 = (-1193276572_i32) as usize;
_4 = _3 - _8;
_3 = _4;
(*RET) = 8471662648610400372_usize >> _8;
_1 = _5;
_11 = (-29715989635958245564702916579993842805_i128) as usize;
_12 = (-1288494571_i32) as isize;
(*RET) = _12 as usize;
_12 = _3;
_12 = _4 << _7;
RET = core::ptr::addr_of!((*RET));
(*RET) = 14170935228578793417_u64 as usize;
_10 = _3;
_7 = 58965_u16 as isize;
_13 = 35_u8 as isize;
RET = core::ptr::addr_of!(_11);
_14 = _3 * _12;
Goto(bb4)
}
bb4 = {
_4 = 647095845520184948_u64 as isize;
_13 = !_14;
Goto(bb5)
}
bb5 = {
(*RET) = 489383939_i32 as usize;
_9 = _6 as f64;
RET = core::ptr::addr_of!(_11);
RET = core::ptr::addr_of!(_11);
_3 = _14 - _13;
_21.1 = _6;
_7 = _3;
_2 = [15584_u16,36144_u16,63560_u16,33092_u16,4635_u16,967_u16];
Goto(bb6)
}
bb6 = {
_21.3 = !52911_u16;
_21.0 = (_21.3, _14);
_11 = !3_usize;
(*RET) = 10282921289333460985_usize;
_21.2 = _3;
_1 = _2;
_21.2 = !_10;
_21.1 = -_6;
_19 = _14 != _21.0.1;
_19 = !true;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_9 = 3849354907492190440_i64 as f64;
_22 = _8 <= _3;
_19 = _22;
_12 = _14;
_21.0.0 = (-8650_i16) as u16;
_13 = !_14;
_21.0.0 = _21.3 & _21.3;
_12 = !_4;
_21.0 = (_21.3, _12);
_8 = 272474907927648578294738533840362781032_u128 as isize;
_21.0.0 = 1290588529640729729_u64 as u16;
match (*RET) {
10282921289333460985 => bb7,
_ => bb3
}
}
bb7 = {
_21.0.0 = 99_u8 as u16;
_7 = -_3;
(*RET) = '\u{93e35}' as usize;
_19 = _22 ^ _22;
RET = core::ptr::addr_of!((*RET));
_18 = 8538567483820055053_i64;
_13 = (-131771473324890523729059122525561025637_i128) as isize;
_21.3 = _21.0.0 << _14;
_21.2 = 146_u8 as isize;
_21.3 = !_21.0.0;
_21.0 = (_21.3, _3);
_3 = -_14;
_28 = 164_u8;
_21.0.1 = _7;
_12 = _4;
_21.0.1 = _7;
_3 = _21.0.0 as isize;
_21.1 = -_6;
match _18 {
0 => bb1,
1 => bb6,
8538567483820055053 => bb9,
_ => bb8
}
}
bb8 = {
RET = core::ptr::addr_of!(_11);
_5 = _1;
(*RET) = !12837406228355996263_usize;
_4 = 123_u8 as isize;
Goto(bb2)
}
bb9 = {
(*RET) = 1534094687885293499_usize;
match _28 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb10,
164 => bb12,
_ => bb11
}
}
bb10 = {
(*RET) = 5_usize - 15644244860544557117_usize;
_11 = !10910307463758792544_usize;
_1 = [15633_u16,25361_u16,50825_u16,7168_u16,61275_u16,60992_u16];
_1 = _2;
RET = core::ptr::addr_of!((*RET));
Goto(bb3)
}
bb11 = {
_4 = 647095845520184948_u64 as isize;
_13 = !_14;
Goto(bb5)
}
bb12 = {
_10 = _21.0.1 << _7;
Goto(bb13)
}
bb13 = {
_4 = -_7;
(*RET) = 4_usize << _7;
_13 = -_10;
Goto(bb14)
}
bb14 = {
_29.1.fld0.1 = 2050641873_i32 >> (*RET);
_27 = _19;
_9 = _28 as f64;
_5 = [_21.0.0,_21.0.0,_21.3,_21.0.0,_21.3,_21.3];
_8 = -_13;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(18_usize, 10_usize, Move(_10), 28_usize, Move(_28), 18_usize, Move(_18), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(18_usize, 22_usize, Move(_22), 19_usize, Move(_19), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(4_usize), std::hint::black_box(187663838488580835379453792920133497326_u128), std::hint::black_box(42_u8), std::hint::black_box(28715425_i32), std::hint::black_box((-27876_i16)));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt17{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt17 {
fld0: i32,
}
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: (bool, u64, f64, u64),
fld1: Adt17,
fld2: isize,
fld3: u16,
fld4: [u64; 2],
fld5: i32,

},
Variant1{
fld0: *const u128,
fld1: char,
fld2: u16,
fld3: i64,

},
Variant2{
fld0: u64,
fld1: u32,
fld2: isize,
fld3: [usize; 8],
fld4: u8,
fld5: f32,
fld6: i64,
fld7: u128,

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: Adt26,
fld1: char,
fld2: [i16; 3],
fld3: i8,
fld4: i16,
fld5: [i64; 7],
fld6: i64,
fld7: Adt17,

},
Variant1{
fld0: f64,
fld1: u128,
fld2: isize,
fld3: i128,
fld4: [usize; 8],
fld5: [i64; 7],
fld6: *const i64,

},
Variant2{
fld0: u128,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: [i16; 3],
fld1: [usize; 8],

},
Variant1{
fld0: f64,
fld1: i64,
fld2: [i16; 3],
fld3: f32,
fld4: i16,

},
Variant2{
fld0: bool,
fld1: i128,
fld2: [i64; 7],
fld3: *const usize,
fld4: (char,),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: (usize, i32),
fld1: (char,),
fld2: *const (u16, isize),
}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: bool,
fld1: Adt26,
fld2: *const (u16, isize),
fld3: [u64; 2],
fld4: u8,
fld5: [u128; 3],

},
Variant1{
fld0: *const (usize, i32),
fld1: f64,
fld2: u32,
fld3: Adt47,
fld4: i16,

}}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt72{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt72 {
fld0: (char, [i16; 3]),
fld1: *const Adt61,
fld2: (usize, i32),
fld3: [i16; 3],
}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: *const Adt36,
fld1: (char, [i16; 3]),
fld2: [i128; 2],
fld3: (usize, i32),
fld4: i64,

},
Variant1{
fld0: *mut [i128; 2],
fld1: *const i64,
fld2: [i128; 2],
fld3: (u16, isize),
fld4: (*const i64, Adt47),
fld5: (u128, f64),

},
Variant2{
fld0: u64,
fld1: [i16; 3],
fld2: *const [i128; 4],

},
Variant3{
fld0: *const *mut i8,

}}

