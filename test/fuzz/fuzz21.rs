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
pub fn fn0(mut _1: u8,mut _2: i128,mut _3: isize,mut _4: u64) -> isize {
mir! {
type RET = isize;
let _5: bool;
let _6: [u32; 7];
let _7: char;
let _8: ([u8; 4],);
let _9: &'static &'static u128;
let _10: i8;
let _11: isize;
let _12: i32;
let _13: i16;
let _14: i8;
let _15: char;
let _16: &'static &'static isize;
let _17: (&'static Adt50,);
let _18: *const isize;
let _19: i16;
let _20: char;
let _21: [isize; 4];
let _22: Adt33;
let _23: Adt50;
let _24: [i16; 5];
let _25: *const &'static i8;
let _26: f32;
let _27: bool;
let _28: &'static f64;
let _29: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _30: f32;
let _31: isize;
let _32: ([u8; 4],);
let _33: [i8; 2];
let _34: ();
let _35: ();
{
_3 = 61068_u16 as isize;
RET = _3 - _3;
_2 = (-68856810223475680209424449985334862908_i128) + 17228127761777603094329786761327212979_i128;
RET = _3;
_2 = 22666_u16 as i128;
RET = !_3;
_1 = 157_u8;
RET = _3;
_1 = !158_u8;
_4 = 10785247465414705000_u64 << _1;
_4 = 125511274_u32 as u64;
RET = !_3;
Goto(bb1)
}
bb1 = {
RET = _3 | _3;
RET = _3;
_2 = (-90574318880843295467297950350820603000_i128) ^ 109150716193330400911127819886251146959_i128;
_2 = !(-76092413700347159377138578320269045297_i128);
_1 = !237_u8;
_3 = RET;
_2 = (-145284684337721235904566759807116360236_i128) + 78564417369846882966561372810273252965_i128;
_3 = RET >> _1;
_4 = 2446770795704077080_u64 | 4577672525525696602_u64;
_4 = 12263742296051220093_usize as u64;
_5 = true & true;
RET = 4154958649627564996_usize as isize;
Goto(bb2)
}
bb2 = {
_6 = [1756430877_u32,3316641063_u32,3129682519_u32,2928474334_u32,2744356880_u32,3651679547_u32,1874734025_u32];
_6 = [3665660252_u32,597537407_u32,2223082697_u32,1046925892_u32,1392801186_u32,2079401698_u32,266753821_u32];
_6 = [1993620401_u32,2106926173_u32,3663527764_u32,3002156193_u32,2824635902_u32,4153210968_u32,198958762_u32];
RET = _3 * _3;
_1 = 169_u8 ^ 140_u8;
_1 = 182_u8 ^ 200_u8;
_2 = (-108798396277570605327782292013715578280_i128);
_8.0 = [_1,_1,_1,_1];
_6 = [2061025415_u32,2877148199_u32,144422988_u32,1566305869_u32,1567573176_u32,3239465659_u32,665653522_u32];
_4 = 5237302238302005468_u64;
_2 = 39330747522918282864226242765211011927_i128;
_10 = (-3022107183563895359_i64) as i8;
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5237302238302005468 => bb9,
_ => bb8
}
}
bb3 = {
RET = _3 | _3;
RET = _3;
_2 = (-90574318880843295467297950350820603000_i128) ^ 109150716193330400911127819886251146959_i128;
_2 = !(-76092413700347159377138578320269045297_i128);
_1 = !237_u8;
_3 = RET;
_2 = (-145284684337721235904566759807116360236_i128) + 78564417369846882966561372810273252965_i128;
_3 = RET >> _1;
_4 = 2446770795704077080_u64 | 4577672525525696602_u64;
_4 = 12263742296051220093_usize as u64;
_5 = true & true;
RET = 4154958649627564996_usize as isize;
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
_3 = RET ^ RET;
_8.0 = [_1,_1,_1,_1];
_3 = _10 as isize;
_1 = 338769048_u32 as u8;
_7 = '\u{99589}';
_8.0 = [_1,_1,_1,_1];
_3 = -RET;
_7 = '\u{142d9}';
_5 = true;
_8.0 = [_1,_1,_1,_1];
_12 = -202017438_i32;
_2 = -108736766838992862177409051756963957239_i128;
RET = _3 + _3;
_5 = !false;
_6 = [139616598_u32,1227469954_u32,324541546_u32,4177161034_u32,714162420_u32,3513946540_u32,872664570_u32];
_10 = 84_i8 * 4_i8;
_12 = -(-235779443_i32);
_4 = !8994003340627920515_u64;
Call(RET = fn1(_7, _8, _8.0, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = _3;
_13 = 2083138438_u32 as i16;
_1 = !25_u8;
_15 = _7;
_10 = 5368032479188624858_i64 as i8;
_4 = 6794_u16 as u64;
_3 = !RET;
_13 = (-820_i16) | 30159_i16;
_6 = [714642477_u32,1075742403_u32,362666161_u32,1536095711_u32,208997995_u32,349936908_u32,198398219_u32];
_8.0 = [_1,_1,_1,_1];
_11 = _4 as isize;
_13 = 17343_i16;
Goto(bb11)
}
bb11 = {
_2 = 48821140749681084146767650702092115834_i128 << _11;
_1 = 175_u8 + 242_u8;
_10 = (-117_i8);
RET = _3 | _3;
RET = _11 & _3;
_18 = core::ptr::addr_of!(RET);
_20 = _7;
_11 = -(*_18);
_4 = _2 as u64;
RET = -_11;
_19 = 8361180642535158262_usize as i16;
_22.fld0 = !_5;
_22.fld0 = _5;
RET = _11 - _11;
_21 = [RET,RET,(*_18),(*_18)];
_22.fld3 = _10;
Goto(bb12)
}
bb12 = {
_2 = (-49719944726227289012251887693867932327_i128);
_23.fld1 = _8;
_23.fld0.0 = _1 as f32;
_24 = [_13,_13,_19,_13,_19];
_21 = [(*_18),RET,_11,(*_18)];
_23.fld1.0 = [_1,_1,_1,_1];
match _22.fld3 {
0 => bb1,
1 => bb11,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211339 => bb13,
_ => bb9
}
}
bb13 = {
_23.fld0.2.3 = !_2;
_3 = (*_18) >> _1;
_22.fld1 = !_23.fld0.2.3;
_15 = _7;
_19 = _23.fld0.0 as i16;
_21 = [_3,_3,(*_18),RET];
_23.fld0.2.0.0 = _20 as u32;
_22.fld4 = Adt17::Variant1 { fld0: _22.fld0,fld1: _15,fld2: 8663949883739884962_i64,fld3: _4,fld4: _2 };
_19 = _13 | _13;
_12 = 3198655731106642978_usize as i32;
Goto(bb14)
}
bb14 = {
place!(Field::<u64>(Variant(_22.fld4, 1), 3)) = _4 + _4;
_23.fld0.2.1 = (-9193678514279247913_i64) as f64;
(*_18) = _12 as isize;
_6 = [_23.fld0.2.0.0,_23.fld0.2.0.0,_23.fld0.2.0.0,_23.fld0.2.0.0,_23.fld0.2.0.0,_23.fld0.2.0.0,_23.fld0.2.0.0];
_12 = Field::<bool>(Variant(_22.fld4, 1), 0) as i32;
_29.0.0 = (_23.fld0.2.0.0, _23.fld0.2.1);
_23.fld0.0 = 52091_u16 as f32;
_5 = !Field::<bool>(Variant(_22.fld4, 1), 0);
_23.fld0.0 = 6_usize as f32;
_26 = _4 as f32;
place!(Field::<u64>(Variant(_22.fld4, 1), 3)) = !_4;
_4 = Field::<u64>(Variant(_22.fld4, 1), 3) & Field::<u64>(Variant(_22.fld4, 1), 3);
_22.fld2 = 4596137711993733503_i64 << _11;
_7 = _15;
_4 = _26 as u64;
_30 = -_26;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(0_usize, 10_usize, Move(_10), 15_usize, Move(_15), 24_usize, Move(_24), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(0_usize, 19_usize, Move(_19), 5_usize, Move(_5), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: ([u8; 4],),mut _3: [u8; 4],mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: u128;
let _6: [u16; 5];
let _7: [u16; 4];
let _8: (*const &'static i8, Adt17, u128);
let _9: f32;
let _10: Adt17;
let _11: (bool, (char, [u16; 5], u8, &'static i8), usize);
let _12: isize;
let _13: [u32; 7];
let _14: ([u8; 4],);
let _15: f32;
let _16: &'static [u16; 5];
let _17: f32;
let _18: isize;
let _19: f64;
let _20: f32;
let _21: *mut usize;
let _22: u64;
let _23: isize;
let _24: i16;
let _25: [i8; 2];
let _26: Adt50;
let _27: *const *mut bool;
let _28: [u8; 4];
let _29: *const *const i8;
let _30: [i8; 5];
let _31: *mut f32;
let _32: i128;
let _33: [u32; 7];
let _34: &'static isize;
let _35: (i8,);
let _36: Adt17;
let _37: i8;
let _38: Adt17;
let _39: u32;
let _40: ();
let _41: ();
{
RET = _4;
_3 = [46_u8,53_u8,122_u8,217_u8];
Call(_4 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = RET;
_2 = (_3,);
RET = -_4;
_1 = '\u{5b2ec}';
_2.0 = _3;
_3 = _2.0;
RET = false as isize;
_4 = (-91_i8) as isize;
_4 = (-29665_i16) as isize;
_2.0 = [202_u8,108_u8,255_u8,47_u8];
Call(_2.0 = fn2(RET, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = (_3,);
_1 = '\u{5c6f1}';
_4 = RET;
_2.0 = [147_u8,189_u8,224_u8,41_u8];
_5 = !146475301028553180661220154678974637846_u128;
_4 = false as isize;
_5 = !246713318088626344247403616104875693185_u128;
RET = _1 as isize;
_2 = (_3,);
_2.0 = _3;
RET = _5 as isize;
_5 = true as u128;
_6 = [42481_u16,48964_u16,6544_u16,16500_u16,19076_u16];
_1 = '\u{21c75}';
RET = _4 & _4;
Goto(bb3)
}
bb3 = {
_6 = [57835_u16,999_u16,29180_u16,61279_u16,28611_u16];
_3 = [254_u8,151_u8,88_u8,24_u8];
RET = _4;
_2 = (_3,);
_5 = !68840423653348032416948898897888250886_u128;
_8.1 = Adt17::Variant0 { fld0: 1057941199_i32 };
RET = -_4;
_7 = [56562_u16,20444_u16,52544_u16,12508_u16];
_3 = [216_u8,83_u8,61_u8,189_u8];
_8.2 = _5 & _5;
place!(Field::<i32>(Variant(_8.1, 0), 0)) = 1376371749_i32;
RET = -_4;
_8.2 = !_5;
RET = -_4;
_10 = Move(_8.1);
_7 = [1268_u16,19125_u16,64505_u16,51089_u16];
_8.2 = 162_u8 as u128;
Goto(bb4)
}
bb4 = {
_11.0 = !false;
_11.2 = 12012_i16 as usize;
_6 = [53779_u16,21265_u16,61205_u16,13391_u16,6829_u16];
_8.0 = core::ptr::addr_of!(_11.1.3);
_8.1 = Adt17::Variant1 { fld0: _11.0,fld1: _1,fld2: 3546625796733256007_i64,fld3: 12495667313118665151_u64,fld4: 56867440846942220853516902190204776729_i128 };
_11.1.0 = Field::<char>(Variant(_8.1, 1), 1);
_11.2 = 3900978503_u32 as usize;
_12 = !_4;
place!(Field::<char>(Variant(_8.1, 1), 1)) = _11.1.0;
_12 = 43685161849657030058349571797023272504_i128 as isize;
place!(Field::<i32>(Variant(_10, 0), 0)) = !1362027811_i32;
RET = _12 * _12;
_9 = _11.2 as f32;
_8.1 = Adt17::Variant0 { fld0: Field::<i32>(Variant(_10, 0), 0) };
_10 = Adt17::Variant2 { fld0: 51_u8,fld1: 11280787529556192435_u64,fld2: (-51_i8) };
_2 = (_3,);
_8.2 = _5;
Call(place!(Field::<u64>(Variant(_10, 2), 1)) = fn18(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<u8>(Variant(_10, 2), 0)) = 101_u8 + 62_u8;
SetDiscriminant(_8.1, 1);
_8.1 = Adt17::Variant0 { fld0: (-1768751084_i32) };
_10 = Adt17::Variant2 { fld0: 113_u8,fld1: 13918208479537616756_u64,fld2: (-67_i8) };
place!(Field::<i8>(Variant(_10, 2), 2)) = 16_i8;
place!(Field::<i32>(Variant(_8.1, 0), 0)) = RET as i32;
_8.2 = 8464979022739501874_u64 as u128;
_11.1.2 = _9 as u8;
_11.1.0 = _1;
_4 = _12;
place!(Field::<i8>(Variant(_10, 2), 2)) = 68_i8;
place!(Field::<i32>(Variant(_8.1, 0), 0)) = _11.0 as i32;
place!(Field::<u64>(Variant(_10, 2), 1)) = _12 as u64;
_11.1.1 = [51894_u16,39512_u16,15389_u16,59730_u16,48491_u16];
_18 = _12 * RET;
_11.1.2 = !207_u8;
RET = _11.1.2 as isize;
RET = -_18;
_10 = Adt17::Variant2 { fld0: _11.1.2,fld1: 18223691101489760011_u64,fld2: 106_i8 };
_8.2 = !_5;
place!(Field::<u64>(Variant(_10, 2), 1)) = 7354198578569255927_u64;
_6 = [50368_u16,2830_u16,50384_u16,43689_u16,4565_u16];
_22 = !Field::<u64>(Variant(_10, 2), 1);
_20 = -_9;
Goto(bb6)
}
bb6 = {
SetDiscriminant(_8.1, 0);
RET = _18 - _4;
_8.1 = Adt17::Variant1 { fld0: _11.0,fld1: _1,fld2: 4604312019886457697_i64,fld3: _22,fld4: (-98047378370738783585468269766084288403_i128) };
_8.0 = core::ptr::addr_of!(_11.1.3);
_7 = [21762_u16,15872_u16,39813_u16,63319_u16];
place!(Field::<i8>(Variant(_10, 2), 2)) = 11_i8;
_11.1.3 = &place!(Field::<i8>(Variant(_10, 2), 2));
RET = 163195870949975372535116654958217437570_i128 as isize;
_13 = [3390253573_u32,2570758978_u32,1253613117_u32,2748350284_u32,3764542877_u32,3471591131_u32,3035657916_u32];
_15 = _18 as f32;
_2 = (_3,);
place!(Field::<i8>(Variant(_10, 2), 2)) = (-22_i8) | 2_i8;
place!(Field::<i128>(Variant(_8.1, 1), 4)) = (-35829474872121034397066908589599557862_i128);
_17 = -_15;
SetDiscriminant(_10, 2);
_11.0 = _15 != _15;
_8.1 = Adt17::Variant1 { fld0: _11.0,fld1: _11.1.0,fld2: (-2871469609778952646_i64),fld3: _22,fld4: (-134961266302034187921982379871721762179_i128) };
_16 = &_11.1.1;
place!(Field::<u64>(Variant(_8.1, 1), 3)) = !_22;
Goto(bb7)
}
bb7 = {
place!(Field::<char>(Variant(_8.1, 1), 1)) = _11.1.0;
_4 = -_18;
_10 = Adt17::Variant1 { fld0: Field::<bool>(Variant(_8.1, 1), 0),fld1: Field::<char>(Variant(_8.1, 1), 1),fld2: 5423444317578785139_i64,fld3: Field::<u64>(Variant(_8.1, 1), 3),fld4: (-137277279329472709596162191443388928809_i128) };
_14.0 = _2.0;
place!(Field::<u64>(Variant(_8.1, 1), 3)) = _22 + Field::<u64>(Variant(_10, 1), 3);
_11.1.1 = [13114_u16,9734_u16,49777_u16,1046_u16,37169_u16];
_8.2 = _11.1.2 as u128;
_22 = !Field::<u64>(Variant(_8.1, 1), 3);
_14 = _2;
_23 = _4;
place!(Field::<i128>(Variant(_8.1, 1), 4)) = (-145619045866635908084048247421704173450_i128) + (-100761076482757985243552099063220908913_i128);
_7 = [43046_u16,30989_u16,24246_u16,6161_u16];
place!(Field::<i64>(Variant(_8.1, 1), 2)) = (-5181130639932830968_i64) ^ (-3262318611721006108_i64);
_26.fld0.1 = 62094_u16 << _5;
_17 = -_15;
_26.fld0.2.2 = _26.fld0.1;
_26.fld0.2.0.1 = _11.2 as f64;
Call(RET = fn19(Move(_8), _23, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<i128>(Variant(_10, 1), 4)) = 2856572260_u32 as i128;
_16 = &_6;
_30 = [67_i8,50_i8,(-21_i8),(-25_i8),(-6_i8)];
_25 = [122_i8,120_i8];
_16 = &_6;
_26.fld1 = (_3,);
_12 = _23;
_11.1.2 = !39_u8;
_22 = Field::<u64>(Variant(_10, 1), 3);
_22 = Field::<u64>(Variant(_10, 1), 3) << _11.2;
_28 = _2.0;
_22 = Field::<u64>(Variant(_10, 1), 3) << _23;
_2 = (_28,);
_14.0 = [_11.1.2,_11.1.2,_11.1.2,_11.1.2];
RET = _23;
_28 = [_11.1.2,_11.1.2,_11.1.2,_11.1.2];
_14.0 = [_11.1.2,_11.1.2,_11.1.2,_11.1.2];
_5 = !8623208656971255439509535816627414682_u128;
_31 = core::ptr::addr_of_mut!(_26.fld0.0);
(*_31) = _22 as f32;
Call(_9 = core::intrinsics::transmute(_28), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_26.fld2 = [10604_i16,14981_i16,13816_i16,26409_i16,20352_i16];
_21 = core::ptr::addr_of_mut!(_11.2);
_7 = [_26.fld0.2.2,_26.fld0.1,_26.fld0.1,_26.fld0.2.2];
_26.fld1.0 = [_11.1.2,_11.1.2,_11.1.2,_11.1.2];
(*_31) = _17 * _20;
_26.fld0.2.3 = Field::<i128>(Variant(_10, 1), 4) << (*_21);
Goto(bb10)
}
bb10 = {
_11.1.2 = 10690_i16 as u8;
_33 = [2700077399_u32,2359674580_u32,631086307_u32,3145994088_u32,1665491009_u32,2108992914_u32,2241655592_u32];
RET = _23;
_2 = _26.fld1;
_34 = &_23;
_8.0 = core::ptr::addr_of!(_11.1.3);
_20 = (*_31);
_10 = Adt17::Variant2 { fld0: _11.1.2,fld1: _22,fld2: (-119_i8) };
_26.fld0.0 = -_15;
_26.fld0.2.3 = _11.1.0 as i128;
Goto(bb11)
}
bb11 = {
_11.2 = 4_usize - 15034206416855730114_usize;
_11.2 = !1_usize;
_5 = _11.1.2 as u128;
_12 = !RET;
_26.fld0.0 = _26.fld0.2.2 as f32;
_10 = Adt17::Variant0 { fld0: (-1603906871_i32) };
_26.fld0.2.3 = (-18931038_i32) as i128;
_11.2 = 4005994023646152547_usize;
Goto(bb12)
}
bb12 = {
_10 = Adt17::Variant2 { fld0: _11.1.2,fld1: _22,fld2: (-70_i8) };
_33 = _13;
_9 = _20;
_26.fld0.1 = _26.fld0.2.2;
place!(Field::<i8>(Variant(_10, 2), 2)) = 5_i8;
_14 = (_26.fld1.0,);
_26.fld0.2.0.0 = !4112840457_u32;
_17 = _15;
(*_21) = 0_usize ^ 3_usize;
place!(Field::<i8>(Variant(_10, 2), 2)) = !94_i8;
_36 = Adt17::Variant2 { fld0: Field::<u8>(Variant(_10, 2), 0),fld1: Field::<u64>(Variant(_10, 2), 1),fld2: Field::<i8>(Variant(_10, 2), 2) };
_11.1.3 = &place!(Field::<i8>(Variant(_10, 2), 2));
_8.0 = core::ptr::addr_of!(_11.1.3);
_23 = _4;
_26.fld0.2.3 = (-26293464715182183859039929859471757739_i128) - 77105358706516521104114953229167487380_i128;
_11.1.1 = [_26.fld0.2.2,_26.fld0.2.2,_26.fld0.1,_26.fld0.1,_26.fld0.1];
_1 = _11.1.0;
_19 = _26.fld0.2.0.1;
place!(Field::<u8>(Variant(_36, 2), 0)) = !Field::<u8>(Variant(_10, 2), 0);
Goto(bb13)
}
bb13 = {
_1 = _11.1.0;
_26.fld1.0 = _3;
_12 = Field::<u64>(Variant(_10, 2), 1) as isize;
Goto(bb14)
}
bb14 = {
(*_21) = _22 as usize;
_24 = (-3044_i16);
_11.1.0 = _1;
_26.fld0.2.3 = (-135221226604389136805114712659110074046_i128) ^ (-159266313981406042440226648136101144156_i128);
RET = _23 - _12;
_26.fld1 = (_3,);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(1_usize, 5_usize, Move(_5), 33_usize, Move(_33), 24_usize, Move(_24), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 28_usize, Move(_28), 22_usize, Move(_22), 12_usize, Move(_12), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(1_usize, 18_usize, Move(_18), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: [u8; 4],mut _3: isize,mut _4: [u8; 4]) -> [u8; 4] {
mir! {
type RET = [u8; 4];
let _5: &'static *const &'static i8;
let _6: f64;
let _7: (Adt33, &'static [i16; 4]);
let _8: *const *const i8;
let _9: u8;
let _10: Adt50;
let _11: usize;
let _12: isize;
let _13: &'static u32;
let _14: i128;
let _15: f32;
let _16: u128;
let _17: ();
let _18: ();
{
RET = _4;
_3 = 236821866439383884827194131333375083286_u128 as isize;
_4 = [158_u8,152_u8,103_u8,99_u8];
RET = _2;
_4 = RET;
_1 = _3 ^ _3;
_1 = -_3;
Goto(bb1)
}
bb1 = {
_1 = _3 >> _3;
RET = [165_u8,34_u8,139_u8,93_u8];
_7.0.fld0 = _3 <= _3;
_6 = 8540807827932409489_usize as f64;
_7.0.fld0 = _6 <= _6;
_7.0.fld0 = true;
RET = _4;
_1 = !_3;
_7.0.fld2 = 115303020009877654105236453668344013769_i128 as i64;
_7.0.fld3 = 31_i8;
_3 = _1 - _1;
_2 = RET;
_6 = _3 as f64;
_7.0.fld2 = 9206179682811103249_i64 << _7.0.fld3;
match _7.0.fld3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
31 => bb9,
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
_4 = RET;
_7.0.fld0 = !true;
_2 = _4;
_7.0.fld2 = (-7403454366911439560_i64);
_9 = 110_u8 & 252_u8;
_1 = _3;
_10.fld0.2.0 = (2970019191_u32, _6);
_6 = _10.fld0.2.0.1;
_10.fld0.2.3 = -(-29386758159522711421772483183977337621_i128);
match _7.0.fld2 {
0 => bb10,
1 => bb11,
340282366920938463455971153064856771896 => bb13,
_ => bb12
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
_7.0.fld4 = Adt17::Variant1 { fld0: _7.0.fld0,fld1: '\u{1007a2}',fld2: _7.0.fld2,fld3: 4590137336884652275_u64,fld4: _10.fld0.2.3 };
_10.fld0.2.0.1 = _6;
place!(Field::<i64>(Variant(_7.0.fld4, 1), 2)) = _7.0.fld2 - _7.0.fld2;
Call(_10.fld0.2.3 = fn3(), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10.fld0.0 = 1848450455_i32 as f32;
_10.fld1 = (_4,);
_6 = 8885422402513239668_u64 as f64;
RET = [_9,_9,_9,_9];
_11 = 6_usize;
_6 = -_10.fld0.2.0.1;
_10.fld0.2.3 = _11 as i128;
_3 = -_1;
_1 = _3 << Field::<i128>(Variant(_7.0.fld4, 1), 4);
_12 = !_1;
_10.fld0.2.2 = !43719_u16;
_10.fld0.2.1 = _6;
_10.fld0.2.3 = -Field::<i128>(Variant(_7.0.fld4, 1), 4);
place!(Field::<i64>(Variant(_7.0.fld4, 1), 2)) = _7.0.fld2 << _1;
_16 = 295197160654180192811126269802871151477_u128 & 316976398580903374241303174599092404592_u128;
_10.fld0.2.0.1 = _16 as f64;
_4 = [_9,_9,_9,_9];
_7.0.fld1 = -Field::<i128>(Variant(_7.0.fld4, 1), 4);
_10.fld0.1 = !_10.fld0.2.2;
place!(Field::<char>(Variant(_7.0.fld4, 1), 1)) = '\u{f0422}';
_7.0.fld3 = 47_i8 ^ 84_i8;
_3 = _12 + _1;
_10.fld1.0 = [_9,_9,_9,_9];
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(2_usize, 4_usize, Move(_4), 11_usize, Move(_11), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3() -> i128 {
mir! {
type RET = i128;
let _1: i64;
let _2: (bool, (char, [u16; 5], u8, &'static i8), usize);
let _3: usize;
let _4: &'static (f32, u16, ((u32, f64), f64, u16, i128));
let _5: &'static *const &'static i8;
let _6: f32;
let _7: *mut f32;
let _8: &'static (Adt33, &'static [i16; 4]);
let _9: (f32, u16, ((u32, f64), f64, u16, i128));
let _10: Adt50;
let _11: bool;
let _12: *const *mut bool;
let _13: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _14: i16;
let _15: &'static i8;
let _16: &'static (Adt33, &'static [i16; 4]);
let _17: (*mut bool, (*const &'static i8, Adt17, u128), [i16; 5], i64);
let _18: char;
let _19: [u8; 1];
let _20: &'static *const &'static i8;
let _21: i32;
let _22: f64;
let _23: &'static *const &'static i8;
let _24: f64;
let _25: ();
let _26: ();
{
RET = (-9223372036854775808_isize) as i128;
RET = 45524163134772992186242477514949276559_i128;
RET = 92896175697658520673959159939335765607_i128 ^ 86189851359465073523728966953636581371_i128;
RET = !40559156537427009908061598589046338950_i128;
RET = -76280357064887472143297909017646410333_i128;
RET = !(-82088606619821350834002703631599406514_i128);
RET = !(-24742831216690824900866293241110103565_i128);
RET = -96187635565226671659619481577202888729_i128;
RET = 112285571193611969714306994183362975164_i128;
RET = true as i128;
_2.2 = 7_usize >> RET;
RET = 135470674688449779966887280861825793391_i128 + 119988852679097330567790786748426259982_i128;
_2.2 = 27565_i16 as usize;
_3 = !_2.2;
_3 = 78_i8 as usize;
_1 = (-8265478473295215816_i64) & 3085001495767863100_i64;
Goto(bb1)
}
bb1 = {
_2.2 = !_3;
RET = !152101323641750745262672501409577214490_i128;
_2.1.1 = [36804_u16,23640_u16,39763_u16,62609_u16,26283_u16];
_2.0 = true;
_2.1.2 = (-113_i8) as u8;
RET = !(-89116642876170994537246061843758664560_i128);
_2.1.0 = '\u{398d8}';
_6 = 541784387_i32 as f32;
_2.1.2 = 197_u8 ^ 204_u8;
_3 = _2.2 ^ _2.2;
_2.1.2 = 178_u8;
_2.1.1 = [47042_u16,27924_u16,45527_u16,55299_u16,48813_u16];
_1 = 7857049292996069128_i64;
RET = _2.1.0 as i128;
_2.2 = _3;
_2.2 = !_3;
RET = !73269938004599852967634860489676298892_i128;
_3 = !_2.2;
RET = !82416438630110320231743315913186386297_i128;
_2.2 = _3 & _3;
_3 = _2.2 * _2.2;
RET = _2.0 as i128;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 9223372036854775807_isize as f32;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 10385_i16 as f32;
_2.1.2 = _1 as u8;
_2.0 = false;
Goto(bb2)
}
bb2 = {
_7 = core::ptr::addr_of_mut!(_9.0);
_9.0 = _6 * _6;
_9.2.0.1 = RET as f64;
_2.1.0 = '\u{6dd7d}';
_9.2.3 = RET >> _2.2;
_9.2.1 = _9.2.0.1 - _9.2.0.1;
_2.0 = true & false;
_4 = &_9;
RET = (*_4).2.3 - (*_4).2.3;
_10.fld0.2.1 = (*_4).2.1 * _9.2.0.1;
Goto(bb3)
}
bb3 = {
_10.fld0.0 = -_6;
_9.2.2 = !25414_u16;
_9.2.2 = 46508_u16;
_9.2.0.0 = 1827826215_u32 >> (*_4).2.3;
_10.fld0.2.0.1 = _9.0 as f64;
_10.fld0.2.1 = (*_4).2.1 - (*_4).2.1;
_11 = !_2.0;
Goto(bb4)
}
bb4 = {
_4 = &_9;
_6 = (*_4).0;
_4 = &_9;
_7 = core::ptr::addr_of_mut!((*_4).0);
_10.fld0 = ((*_4).0, (*_4).2.2, (*_4).2);
_10.fld0.2.0.0 = (*_4).2.0.0 << _3;
_10.fld2 = [30824_i16,(-18691_i16),(-20053_i16),(-31412_i16),20235_i16];
_4 = &_10.fld0;
_1 = 1473584976913084166_i64;
_13.0 = (_10.fld0.2.0, (*_4).2.1, _10.fld0.1, (*_4).2.3);
_13.0.0 = (*_4).2.0;
_13.0.3 = _9.2.3 - (*_4).2.3;
_10.fld0.2.1 = -_13.0.0.1;
_10.fld0.2.0 = (_13.0.0.0, _9.2.1);
_13.2.0 = _2.1.0;
_13.2.0 = _2.1.0;
_13.0.1 = _10.fld0.2.0.1;
_6 = _13.0.3 as f32;
match _9.2.2 {
0 => bb5,
1 => bb6,
2 => bb7,
46508 => bb9,
_ => bb8
}
}
bb5 = {
_10.fld0.0 = -_6;
_9.2.2 = !25414_u16;
_9.2.2 = 46508_u16;
_9.2.0.0 = 1827826215_u32 >> (*_4).2.3;
_10.fld0.2.0.1 = _9.0 as f64;
_10.fld0.2.1 = (*_4).2.1 - (*_4).2.1;
_11 = !_2.0;
Goto(bb4)
}
bb6 = {
_7 = core::ptr::addr_of_mut!(_9.0);
_9.0 = _6 * _6;
_9.2.0.1 = RET as f64;
_2.1.0 = '\u{6dd7d}';
_9.2.3 = RET >> _2.2;
_9.2.1 = _9.2.0.1 - _9.2.0.1;
_2.0 = true & false;
_4 = &_9;
RET = (*_4).2.3 - (*_4).2.3;
_10.fld0.2.1 = (*_4).2.1 * _9.2.0.1;
Goto(bb3)
}
bb7 = {
_2.2 = !_3;
RET = !152101323641750745262672501409577214490_i128;
_2.1.1 = [36804_u16,23640_u16,39763_u16,62609_u16,26283_u16];
_2.0 = true;
_2.1.2 = (-113_i8) as u8;
RET = !(-89116642876170994537246061843758664560_i128);
_2.1.0 = '\u{398d8}';
_6 = 541784387_i32 as f32;
_2.1.2 = 197_u8 ^ 204_u8;
_3 = _2.2 ^ _2.2;
_2.1.2 = 178_u8;
_2.1.1 = [47042_u16,27924_u16,45527_u16,55299_u16,48813_u16];
_1 = 7857049292996069128_i64;
RET = _2.1.0 as i128;
_2.2 = _3;
_2.2 = !_3;
RET = !73269938004599852967634860489676298892_i128;
_3 = !_2.2;
RET = !82416438630110320231743315913186386297_i128;
_2.2 = _3 & _3;
_3 = _2.2 * _2.2;
RET = _2.0 as i128;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 9223372036854775807_isize as f32;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 10385_i16 as f32;
_2.1.2 = _1 as u8;
_2.0 = false;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_14 = 13426_i16 & 1755_i16;
_14 = (-241_i16) | (-19922_i16);
_13.2.1 = [(*_4).2.2,_13.0.2,(*_4).1,_10.fld0.1,_10.fld0.2.2];
_9.2.0 = _10.fld0.2.0;
_10.fld0.2 = (_9.2.0, _13.0.0.1, _10.fld0.1, _13.0.3);
_10.fld0 = (_6, _13.0.2, _13.0);
RET = !_13.0.3;
_9.2 = (_10.fld0.2.0, _13.0.1, _10.fld0.2.2, RET);
_10.fld0.2.1 = _13.0.1 + _13.0.0.1;
_13.2.0 = _2.1.0;
_3 = _2.2 & _2.2;
_13.0 = _9.2;
_2.1.1 = _13.2.1;
_10.fld0.2.0 = (_13.0.0.0, _9.2.1);
_2.2 = _3;
Goto(bb10)
}
bb10 = {
_9 = _10.fld0;
_10.fld1.0 = [_2.1.2,_2.1.2,_2.1.2,_2.1.2];
Call(_10.fld0.2.1 = fn4(_6, _10.fld0.2.0, _6, _10.fld0.2.3, _13.0, _10.fld0.2.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9.1 = _13.0.2 >> RET;
_10.fld0.2.1 = _1 as f64;
_10.fld0.2.2 = _9.1 * _9.1;
_9.0 = -_6;
_5 = &_17.1.0;
_13.0.0 = (_9.2.0.0, _9.2.1);
_13.0.1 = _10.fld0.2.0.1;
Goto(bb12)
}
bb12 = {
_13.0.0.0 = _9.2.0.0 * _9.2.0.0;
_9.2.2 = _10.fld0.2.2;
_10.fld0.2.0.1 = _14 as f64;
_17.1.2 = _14 as u128;
_9.2.0 = (_13.0.0.0, _13.0.0.1);
_13.0.0.0 = 1070242740_i32 as u32;
_10.fld0.2.3 = _13.0.3 * _13.0.3;
_9.2.0.1 = 71_i8 as f64;
_20 = &(*_5);
_17.3 = _17.1.2 as i64;
_13.1 = _1 as isize;
_13.0.0.1 = -_9.2.1;
_2.1.0 = _13.2.0;
_13.2.2 = _2.1.2;
_10.fld0.2.0.0 = !_9.2.0.0;
_10.fld0.2 = _13.0;
_17.1.2 = _2.1.2 as u128;
_17.1.1 = Adt17::Variant0 { fld0: (-70296573_i32) };
_10.fld0.2.0.0 = _13.1 as u32;
_20 = &(*_5);
_10.fld0.2.3 = _13.0.3;
_17.1.1 = Adt17::Variant0 { fld0: (-1720109821_i32) };
_10.fld0.2.0.0 = _9.2.0.0;
_10.fld0.2.0.0 = _9.2.0.0;
_23 = &(*_5);
match _10.fld0.2.2 {
0 => bb13,
1 => bb14,
46508 => bb16,
_ => bb15
}
}
bb13 = {
_2.2 = !_3;
RET = !152101323641750745262672501409577214490_i128;
_2.1.1 = [36804_u16,23640_u16,39763_u16,62609_u16,26283_u16];
_2.0 = true;
_2.1.2 = (-113_i8) as u8;
RET = !(-89116642876170994537246061843758664560_i128);
_2.1.0 = '\u{398d8}';
_6 = 541784387_i32 as f32;
_2.1.2 = 197_u8 ^ 204_u8;
_3 = _2.2 ^ _2.2;
_2.1.2 = 178_u8;
_2.1.1 = [47042_u16,27924_u16,45527_u16,55299_u16,48813_u16];
_1 = 7857049292996069128_i64;
RET = _2.1.0 as i128;
_2.2 = _3;
_2.2 = !_3;
RET = !73269938004599852967634860489676298892_i128;
_3 = !_2.2;
RET = !82416438630110320231743315913186386297_i128;
_2.2 = _3 & _3;
_3 = _2.2 * _2.2;
RET = _2.0 as i128;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 9223372036854775807_isize as f32;
_7 = core::ptr::addr_of_mut!(_6);
_6 = 10385_i16 as f32;
_2.1.2 = _1 as u8;
_2.0 = false;
Goto(bb2)
}
bb14 = {
_7 = core::ptr::addr_of_mut!(_9.0);
_9.0 = _6 * _6;
_9.2.0.1 = RET as f64;
_2.1.0 = '\u{6dd7d}';
_9.2.3 = RET >> _2.2;
_9.2.1 = _9.2.0.1 - _9.2.0.1;
_2.0 = true & false;
_4 = &_9;
RET = (*_4).2.3 - (*_4).2.3;
_10.fld0.2.1 = (*_4).2.1 * _9.2.0.1;
Goto(bb3)
}
bb15 = {
_7 = core::ptr::addr_of_mut!(_9.0);
_9.0 = _6 * _6;
_9.2.0.1 = RET as f64;
_2.1.0 = '\u{6dd7d}';
_9.2.3 = RET >> _2.2;
_9.2.1 = _9.2.0.1 - _9.2.0.1;
_2.0 = true & false;
_4 = &_9;
RET = (*_4).2.3 - (*_4).2.3;
_10.fld0.2.1 = (*_4).2.1 * _9.2.0.1;
Goto(bb3)
}
bb16 = {
_9.2.0.1 = _10.fld0.2.0.1 - _13.0.0.1;
_22 = -_13.0.1;
_10.fld0.2 = (_9.2.0, _13.0.1, _9.2.2, _9.2.3);
_9.1 = _9.2.2 * _9.2.2;
_18 = _13.2.0;
_10.fld0 = (_6, _9.2.2, _9.2);
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(3_usize, 18_usize, Move(_18), 1_usize, Move(_1), 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f32,mut _2: (u32, f64),mut _3: f32,mut _4: i128,mut _5: ((u32, f64), f64, u16, i128),mut _6: (u32, f64)) -> f64 {
mir! {
type RET = f64;
let _7: f32;
let _8: (i128, [isize; 4], &'static *const &'static i8);
let _9: [i16; 5];
let _10: i64;
let _11: *const &'static u128;
let _12: &'static (Adt33, &'static [i16; 4]);
let _13: i64;
let _14: bool;
let _15: isize;
let _16: u32;
let _17: &'static (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _18: isize;
let _19: i8;
let _20: u16;
let _21: [i16; 5];
let _22: isize;
let _23: [isize; 7];
let _24: char;
let _25: char;
let _26: *mut bool;
let _27: f32;
let _28: *mut f32;
let _29: &'static (f32, u16, ((u32, f64), f64, u16, i128));
let _30: bool;
let _31: &'static [i16; 4];
let _32: Adt50;
let _33: u32;
let _34: i8;
let _35: &'static u128;
let _36: bool;
let _37: [u16; 5];
let _38: ((f32, u16, ((u32, f64), f64, u16, i128)), &'static i8);
let _39: *mut bool;
let _40: &'static i8;
let _41: ();
let _42: ();
{
_3 = _1 + _1;
_6.1 = _5.1;
RET = _2.1 * _5.1;
_5.2 = false as u16;
Goto(bb1)
}
bb1 = {
_2.0 = _5.0.0 - _6.0;
RET = _5.1 * _5.1;
_2.0 = _6.0;
_3 = RET as f32;
_5.0.1 = _2.1 - _5.1;
_5 = (_2, _6.1, 21522_u16, _4);
_7 = _1;
RET = _6.1 * _6.1;
_5 = (_2, RET, 52376_u16, _4);
_5.0.1 = 2003905164_i32 as f64;
_2.0 = 9223372036854775807_isize as u32;
_5.1 = _6.1;
_5.3 = (-715667063287212858_i64) as i128;
_5.3 = _5.0.0 as i128;
_1 = _7 * _7;
_5.2 = true as u16;
RET = _5.2 as f64;
_6 = _5.0;
_1 = _7 - _7;
Goto(bb2)
}
bb2 = {
RET = _5.0.1 - _2.1;
_2 = _6;
_6.1 = -RET;
_5.1 = -_5.0.1;
_8.0 = _4;
_8.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5.0.1 = _5.3 as f64;
_1 = 121_i8 as f32;
Call(_5.0 = fn5(_6.0, _8.0, _2.0, _7, _8.1, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = (-2670127775212910151_i64) * (-8941434883530670460_i64);
_5.3 = _8.0;
Call(_6.0 = fn6(_5, _2, _2.0, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5.0.0 = !_2.0;
_8.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.0 = (_6.0, _5.1);
_13 = _10;
RET = _7 as f64;
Goto(bb5)
}
bb5 = {
RET = _6.1 + _6.1;
_5.1 = RET - RET;
_8.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,123_isize];
_6 = (_2.0, RET);
_4 = 728701649472323437_u64 as i128;
_6 = (_5.0.0, _5.1);
_8.0 = _5.3 * _5.3;
_16 = _2.0;
_3 = _7;
_8.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_13 = _10;
_7 = 14464054347390524443_u64 as f32;
_7 = _1 - _3;
Goto(bb6)
}
bb6 = {
_5.0.0 = _5.2 as u32;
_13 = _10;
Goto(bb7)
}
bb7 = {
_5.0 = (_6.0, RET);
_20 = _5.2 ^ _5.2;
_15 = 9223372036854775807_isize + 9223372036854775807_isize;
_2.0 = !_6.0;
_19 = 122_i8 * (-11_i8);
_22 = -_15;
_5.2 = _20 * _20;
_14 = false;
_2.1 = 12021346655881631891_u64 as f64;
_5.0.1 = 8534326680127143288_u64 as f64;
_9 = [3914_i16,19162_i16,31200_i16,10897_i16,5037_i16];
_19 = (-20_i8) >> _2.0;
_5.1 = RET * _5.0.1;
Call(_13 = fn7(_9, _5.0.0, _5.0, _2, _6, _5.0.0, _5.3, _6.0, _2.0, _19, _6.0, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19 = (-95_i8);
_6.0 = _2.0;
_18 = _22;
_5.0.0 = !_16;
_6.0 = 526_i16 as u32;
_9 = [(-28895_i16),(-10676_i16),(-26968_i16),(-15096_i16),(-576_i16)];
_5.0.0 = _2.0;
_24 = '\u{eca5f}';
_26 = core::ptr::addr_of_mut!(_14);
_27 = -_3;
_7 = -_27;
_28 = core::ptr::addr_of_mut!(_27);
_8.1 = [_15,_22,_15,_18];
_7 = -(*_28);
_7 = _3;
match _19 {
0 => bb9,
340282366920938463463374607431768211361 => bb11,
_ => bb10
}
}
bb9 = {
_10 = (-2670127775212910151_i64) * (-8941434883530670460_i64);
_5.3 = _8.0;
Call(_6.0 = fn6(_5, _2, _2.0, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_5.0.0 = !_2.0;
_8.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.0 = (_6.0, _5.1);
_13 = _10;
RET = _7 as f64;
Goto(bb5)
}
bb11 = {
RET = _5.1;
_8.1 = [_18,_22,_15,_15];
_22 = !_15;
_8.1 = [_15,_22,_18,_15];
_2.1 = RET;
_16 = 18046894641438535006_u64 as u32;
_5.3 = _8.0 & _8.0;
_19 = 126_i8 * 62_i8;
_8.0 = _19 as i128;
_21 = [22393_i16,(-15411_i16),(-9398_i16),(-25678_i16),23335_i16];
_32.fld0 = (_7, _20, _5);
_8.1 = [_18,_18,_22,_15];
_16 = _2.0 - _2.0;
_27 = _7 + _7;
_32.fld2 = [27263_i16,(-24777_i16),29994_i16,12963_i16,(-2882_i16)];
_32.fld1.0 = [97_u8,93_u8,224_u8,202_u8];
_32.fld0.2.2 = _5.2 * _5.2;
_32.fld0 = ((*_28), _20, _5);
_20 = _32.fld0.1;
Call(_15 = fn9(_6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6.0 = !_32.fld0.2.0.0;
Goto(bb13)
}
bb13 = {
_22 = _18 & _18;
_5.0.1 = _5.2 as f64;
_7 = _6.1 as f32;
_20 = _16 as u16;
Call(_6.0 = core::intrinsics::bswap(_16), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_5.0.1 = _32.fld0.2.1 * _6.1;
_32.fld0.2.0.1 = _32.fld0.2.1;
_24 = '\u{e24bd}';
_38.0.2.1 = _32.fld0.2.1;
_38.0 = ((*_28), _20, _5);
_22 = _15 + _15;
_2.0 = !_6.0;
_34 = !_19;
_32.fld2 = [14069_i16,(-28710_i16),(-2372_i16),(-31038_i16),28281_i16];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(4_usize, 4_usize, Move(_4), 22_usize, Move(_22), 20_usize, Move(_20), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(4_usize, 34_usize, Move(_34), 24_usize, Move(_24), 13_usize, Move(_13), 42_usize, _42), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u32,mut _2: i128,mut _3: u32,mut _4: f32,mut _5: [isize; 4],mut _6: f64) -> (u32, f64) {
mir! {
type RET = (u32, f64);
let _7: &'static (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _8: isize;
let _9: ();
let _10: ();
{
_6 = _4 as f64;
RET.0 = !_3;
RET.1 = _6 + _6;
_2 = 35249297629133136679978402503512782548_i128;
_1 = !RET.0;
_4 = (-9223372036854775808_isize) as f32;
_1 = RET.0;
_3 = !RET.0;
RET = (_1, _6);
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(5_usize, 1_usize, Move(_1), 2_usize, Move(_2), 10_usize, _10, 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: ((u32, f64), f64, u16, i128),mut _2: (u32, f64),mut _3: u32,mut _4: ((u32, f64), f64, u16, i128)) -> u32 {
mir! {
type RET = u32;
let _5: &'static u128;
let _6: i8;
let _7: i64;
let _8: ();
let _9: ();
{
RET = 1112662233781638598_i64 as u32;
_4.1 = (-12_i8) as f64;
_1 = (_2, _4.0.1, _4.2, _4.3);
_4 = (_2, _1.1, _1.2, _1.3);
_4.1 = _1.1;
_1.1 = 226_u8 as f64;
_1 = (_4.0, _4.0.1, _4.2, _4.3);
_1.0.1 = _4.1;
RET = _3 | _1.0.0;
_1 = _4;
_6 = -(-1_i8);
_4.0.1 = _4.1;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(6_usize, 3_usize, Move(_3), 9_usize, _9, 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i16; 5],mut _2: u32,mut _3: (u32, f64),mut _4: (u32, f64),mut _5: (u32, f64),mut _6: u32,mut _7: i128,mut _8: u32,mut _9: u32,mut _10: i8,mut _11: u32,mut _12: u32) -> i64 {
mir! {
type RET = i64;
let _13: [u8; 1];
let _14: ((u32, f64), f64, u16, i128);
let _15: *mut Adt50;
let _16: [i8; 5];
let _17: u16;
let _18: bool;
let _19: i128;
let _20: bool;
let _21: i128;
let _22: u16;
let _23: &'static Adt50;
let _24: i32;
let _25: char;
let _26: *mut f32;
let _27: (bool, (char, [u16; 5], u8, &'static i8), usize);
let _28: *const *const i8;
let _29: *const Adt17;
let _30: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _31: isize;
let _32: isize;
let _33: i8;
let _34: bool;
let _35: isize;
let _36: f32;
let _37: u64;
let _38: isize;
let _39: f32;
let _40: ();
let _41: ();
{
_12 = 47989_u16 as u32;
_4 = (_9, _5.1);
_1 = [27217_i16,(-4769_i16),(-1836_i16),(-5451_i16),(-22408_i16)];
_5.1 = _4.1 + _4.1;
RET = -8065508988401992148_i64;
_4 = (_6, _5.1);
RET = (-551898598_i32) as i64;
_5.1 = _4.1;
_5 = (_9, _4.1);
_14 = (_3, _5.1, 64485_u16, _7);
_14.2 = '\u{6fe22}' as u16;
_2 = !_3.0;
_13 = [138_u8];
_5 = (_6, _4.1);
_6 = _8;
_6 = _4.0 | _5.0;
_14.1 = _5.1 + _5.1;
Goto(bb1)
}
bb1 = {
_5.1 = -_14.1;
_3.0 = RET as u32;
_8 = _9 & _4.0;
_8 = 1937662553_i32 as u32;
_2 = !_6;
_6 = !_4.0;
_14.0 = _4;
_14.0 = _5;
_3 = _5;
_4 = (_14.0.0, _14.1);
_3 = (_2, _5.1);
Goto(bb2)
}
bb2 = {
_4.1 = 70962712081178673407501893028454760306_u128 as f64;
_9 = _2 - _6;
_11 = !_6;
_14.0.0 = _2 << _9;
_14 = (_3, _5.1, 43647_u16, _7);
_7 = 247_u8 as i128;
_4.0 = _14.0.0 & _3.0;
_14.2 = 63701_u16;
_4 = (_5.0, _3.1);
_14.2 = _14.1 as u16;
_8 = 152_u8 as u32;
_16 = [_10,_10,_10,_10,_10];
_14.0.1 = _3.1 + _4.1;
_2 = _10 as u32;
_3 = (_14.0.0, _14.0.1);
_13 = [197_u8];
_14.2 = !59552_u16;
_14.2 = !59624_u16;
_9 = _5.0;
_12 = !_3.0;
_14.0.1 = 1421393997438430373_usize as f64;
RET = -(-6151661171754974877_i64);
_14.0.1 = _3.1;
_3.0 = 625709769083677907_u64 as u32;
Call(_5.0 = fn8(_14, _14.0, _14, _3, _3.1, _2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = (_4, _3.1, 8674_u16, _7);
_14.0.0 = !_11;
_11 = 252_i16 as u32;
_9 = (-9785_i16) as u32;
_18 = _5.0 != _5.0;
_3.1 = _4.1 - _5.1;
_4 = _3;
_10 = _7 as i8;
_3.1 = (-9223372036854775808_isize) as f64;
_5.1 = _14.1 * _14.1;
_14.3 = _7;
_5.1 = (-21838_i16) as f64;
_13 = [15_u8];
_13 = [191_u8];
_14.3 = _7 & _7;
_20 = _18;
RET = 1719944775161253818_i64 | (-1302718398453408841_i64);
Call(_5.0 = core::intrinsics::bswap(_2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_20 = !_18;
_14 = (_4, _4.1, 49948_u16, _7);
_2 = !_5.0;
_4.0 = _5.0 - _5.0;
_19 = _18 as i128;
_3.0 = _2;
_17 = _14.2;
_14.1 = _4.1;
_3.0 = !_5.0;
_25 = '\u{8ab1d}';
_27.2 = 12397762001766298097_usize ^ 0_usize;
_14.0.1 = _14.1;
_3.0 = _2 << _4.0;
_14.3 = _19 & _19;
_2 = _4.0 << _14.3;
_10 = 29_i8 & (-11_i8);
_20 = _4.1 != _14.0.1;
_14.3 = !_19;
_5.0 = RET as u32;
_19 = _14.3;
_27.1.3 = &_10;
_8 = _25 as u32;
match _14.2 {
0 => bb1,
1 => bb5,
2 => bb6,
49948 => bb8,
_ => bb7
}
}
bb5 = {
_14 = (_4, _3.1, 8674_u16, _7);
_14.0.0 = !_11;
_11 = 252_i16 as u32;
_9 = (-9785_i16) as u32;
_18 = _5.0 != _5.0;
_3.1 = _4.1 - _5.1;
_4 = _3;
_10 = _7 as i8;
_3.1 = (-9223372036854775808_isize) as f64;
_5.1 = _14.1 * _14.1;
_14.3 = _7;
_5.1 = (-21838_i16) as f64;
_13 = [15_u8];
_13 = [191_u8];
_14.3 = _7 & _7;
_20 = _18;
RET = 1719944775161253818_i64 | (-1302718398453408841_i64);
Call(_5.0 = core::intrinsics::bswap(_2), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_4.1 = 70962712081178673407501893028454760306_u128 as f64;
_9 = _2 - _6;
_11 = !_6;
_14.0.0 = _2 << _9;
_14 = (_3, _5.1, 43647_u16, _7);
_7 = 247_u8 as i128;
_4.0 = _14.0.0 & _3.0;
_14.2 = 63701_u16;
_4 = (_5.0, _3.1);
_14.2 = _14.1 as u16;
_8 = 152_u8 as u32;
_16 = [_10,_10,_10,_10,_10];
_14.0.1 = _3.1 + _4.1;
_2 = _10 as u32;
_3 = (_14.0.0, _14.0.1);
_13 = [197_u8];
_14.2 = !59552_u16;
_14.2 = !59624_u16;
_9 = _5.0;
_12 = !_3.0;
_14.0.1 = 1421393997438430373_usize as f64;
RET = -(-6151661171754974877_i64);
_14.0.1 = _3.1;
_3.0 = 625709769083677907_u64 as u32;
Call(_5.0 = fn8(_14, _14.0, _14, _3, _3.1, _2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_5.1 = -_14.1;
_3.0 = RET as u32;
_8 = _9 & _4.0;
_8 = 1937662553_i32 as u32;
_2 = !_6;
_6 = !_4.0;
_14.0 = _4;
_14.0 = _5;
_3 = _5;
_4 = (_14.0.0, _14.1);
_3 = (_2, _5.1);
Goto(bb2)
}
bb8 = {
_1 = [(-30047_i16),(-14837_i16),(-24171_i16),(-9980_i16),8263_i16];
_30.0.0 = _3;
_14.0.0 = _30.0.0.0;
_14.1 = 101007961815366036_u64 as f64;
_12 = RET as u32;
_14.2 = _17;
_30.2.0 = _25;
_30.0 = (_3, _14.0.1, _14.2, _19);
_27.1.3 = &_10;
_14.0.0 = !_2;
_30.1 = -(-85_isize);
_32 = _30.0.2 as isize;
_14.0 = _4;
_14.0.1 = _4.1 * _30.0.1;
_30.2.0 = _25;
RET = (-7764886636789734383_i64) * (-7575240039001861855_i64);
_30.0.0.0 = _3.0;
_31 = -_32;
_31 = _32;
_34 = _30.0.2 < _14.2;
_16 = [_10,_10,_10,_10,_10];
_30.0.3 = _14.3 | _14.3;
Goto(bb9)
}
bb9 = {
_12 = _14.0.0;
_3.1 = -_4.1;
_30.2.0 = _25;
_30.0 = (_3, _4.1, _14.2, _19);
_21 = _14.3;
_1 = [(-12026_i16),7862_i16,(-10437_i16),19469_i16,562_i16];
_35 = _32 & _32;
_8 = _30.0.0.0;
_5.0 = _4.0;
match _14.2 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb12,
49948 => bb14,
_ => bb13
}
}
bb10 = {
_1 = [(-30047_i16),(-14837_i16),(-24171_i16),(-9980_i16),8263_i16];
_30.0.0 = _3;
_14.0.0 = _30.0.0.0;
_14.1 = 101007961815366036_u64 as f64;
_12 = RET as u32;
_14.2 = _17;
_30.2.0 = _25;
_30.0 = (_3, _14.0.1, _14.2, _19);
_27.1.3 = &_10;
_14.0.0 = !_2;
_30.1 = -(-85_isize);
_32 = _30.0.2 as isize;
_14.0 = _4;
_14.0.1 = _4.1 * _30.0.1;
_30.2.0 = _25;
RET = (-7764886636789734383_i64) * (-7575240039001861855_i64);
_30.0.0.0 = _3.0;
_31 = -_32;
_31 = _32;
_34 = _30.0.2 < _14.2;
_16 = [_10,_10,_10,_10,_10];
_30.0.3 = _14.3 | _14.3;
Goto(bb9)
}
bb11 = {
_5.1 = -_14.1;
_3.0 = RET as u32;
_8 = _9 & _4.0;
_8 = 1937662553_i32 as u32;
_2 = !_6;
_6 = !_4.0;
_14.0 = _4;
_14.0 = _5;
_3 = _5;
_4 = (_14.0.0, _14.1);
_3 = (_2, _5.1);
Goto(bb2)
}
bb12 = {
_4.1 = 70962712081178673407501893028454760306_u128 as f64;
_9 = _2 - _6;
_11 = !_6;
_14.0.0 = _2 << _9;
_14 = (_3, _5.1, 43647_u16, _7);
_7 = 247_u8 as i128;
_4.0 = _14.0.0 & _3.0;
_14.2 = 63701_u16;
_4 = (_5.0, _3.1);
_14.2 = _14.1 as u16;
_8 = 152_u8 as u32;
_16 = [_10,_10,_10,_10,_10];
_14.0.1 = _3.1 + _4.1;
_2 = _10 as u32;
_3 = (_14.0.0, _14.0.1);
_13 = [197_u8];
_14.2 = !59552_u16;
_14.2 = !59624_u16;
_9 = _5.0;
_12 = !_3.0;
_14.0.1 = 1421393997438430373_usize as f64;
RET = -(-6151661171754974877_i64);
_14.0.1 = _3.1;
_3.0 = 625709769083677907_u64 as u32;
Call(_5.0 = fn8(_14, _14.0, _14, _3, _3.1, _2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_14 = (_4, _3.1, 8674_u16, _7);
_14.0.0 = !_11;
_11 = 252_i16 as u32;
_9 = (-9785_i16) as u32;
_18 = _5.0 != _5.0;
_3.1 = _4.1 - _5.1;
_4 = _3;
_10 = _7 as i8;
_3.1 = (-9223372036854775808_isize) as f64;
_5.1 = _14.1 * _14.1;
_14.3 = _7;
_5.1 = (-21838_i16) as f64;
_13 = [15_u8];
_13 = [191_u8];
_14.3 = _7 & _7;
_20 = _18;
RET = 1719944775161253818_i64 | (-1302718398453408841_i64);
Call(_5.0 = core::intrinsics::bswap(_2), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_30.1 = _4.1 as isize;
_4.0 = _6;
_27.1.0 = _30.2.0;
_30.2.3 = &_10;
_5.0 = _30.0.0.0;
_27.1.3 = Move(_30.2.3);
_26 = core::ptr::addr_of_mut!(_39);
_36 = _30.0.3 as f32;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(7_usize, 8_usize, Move(_8), 1_usize, Move(_1), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(7_usize, 12_usize, Move(_12), 11_usize, Move(_11), 31_usize, Move(_31), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(7_usize, 21_usize, Move(_21), 34_usize, Move(_34), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: ((u32, f64), f64, u16, i128),mut _2: (u32, f64),mut _3: ((u32, f64), f64, u16, i128),mut _4: (u32, f64),mut _5: f64,mut _6: u32,mut _7: u32) -> u32 {
mir! {
type RET = u32;
let _8: u64;
let _9: *mut Adt50;
let _10: (f32, u16, ((u32, f64), f64, u16, i128));
let _11: bool;
let _12: ([u8; 4],);
let _13: [i8; 1];
let _14: ();
let _15: ();
{
_3.2 = !_1.2;
_3.0.0 = _7 | _1.0.0;
_1.1 = _5 * _2.1;
_1.0.1 = 269155519355342071_u64 as f64;
_5 = _4.1 - _2.1;
_2 = _1.0;
RET = _3.0.0 >> _2.0;
_3 = (_2, _1.1, _1.2, _1.3);
_10.0 = (-1860_i16) as f32;
_8 = 15029681596088904276_u64 - 142790339771758034_u64;
_8 = !2367398554936673107_u64;
RET = !_1.0.0;
_10.2.2 = !_3.2;
_3.1 = _1.1 * _4.1;
_6 = _10.0 as u32;
_10.2.3 = !_3.3;
_10.0 = _3.0.0 as f32;
_12.0 = [116_u8,254_u8,4_u8,48_u8];
_10.1 = (-2_i8) as u16;
_6 = RET & _2.0;
_11 = _3.1 <= _3.1;
_10.2.2 = _10.1 ^ _10.1;
_1.0 = (_6, _3.1);
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(8_usize, 11_usize, Move(_11), 12_usize, Move(_12), 15_usize, _15, 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (u32, f64)) -> isize {
mir! {
type RET = isize;
let _2: (i128, [isize; 4], &'static *const &'static i8);
let _3: Adt83;
let _4: char;
let _5: i64;
let _6: (bool, (char, [u16; 5], u8, &'static i8), usize);
let _7: bool;
let _8: isize;
let _9: [i16; 5];
let _10: [u16; 5];
let _11: i128;
let _12: *const *mut bool;
let _13: *const Adt17;
let _14: i32;
let _15: &'static *const &'static i8;
let _16: &'static u32;
let _17: *const &'static i8;
let _18: &'static [u16; 5];
let _19: i32;
let _20: [i8; 5];
let _21: *const isize;
let _22: usize;
let _23: *mut f32;
let _24: isize;
let _25: (*const &'static i8, Adt17, u128);
let _26: [i8; 5];
let _27: f32;
let _28: (char, [u16; 5], u8, &'static i8);
let _29: &'static &'static i8;
let _30: [i16; 2];
let _31: [i16; 5];
let _32: &'static (Adt33, &'static [i16; 4]);
let _33: *const *mut bool;
let _34: (&'static u128,);
let _35: ((u32, f64), f64, u16, i128);
let _36: [i16; 2];
let _37: isize;
let _38: *const isize;
let _39: [i16; 4];
let _40: *const *const i8;
let _41: &'static (f32, u16, ((u32, f64), f64, u16, i128));
let _42: [u32; 2];
let _43: ();
let _44: ();
{
RET = (-40_isize);
RET = 2274_i16 as isize;
_2.1 = [RET,RET,RET,RET];
_2.1 = [RET,RET,RET,RET];
_2.0 = 13760149563724196017508186450739610547_i128 * 93491840095429090726412819859889491028_i128;
RET = !(-70_isize);
_2.0 = !(-3250563459227041016639983792039296668_i128);
_1.1 = _1.0 as f64;
_2.0 = (-138055967203192321392757654233097485891_i128);
RET = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_2.0 = (-96_i8) as i128;
_1.0 = !3827866786_u32;
Goto(bb1)
}
bb1 = {
_1.1 = 5997413390630085841_usize as f64;
_1.1 = (-1066655775_i32) as f64;
_1.0 = 3016028041_u32 >> RET;
_1.1 = 12693_i16 as f64;
_4 = '\u{dfb73}';
_1.0 = 1921798313_u32;
RET = 9223372036854775807_isize << _1.0;
_2.0 = -(-54355154716086571288281130314204098958_i128);
_1.1 = 10_i8 as f64;
_4 = '\u{66ffd}';
RET = (-9223372036854775808_isize);
_2.1 = [RET,RET,RET,RET];
_1.1 = 1756_u16 as f64;
_2.0 = 1645329957_i32 as i128;
_2.1 = [RET,RET,RET,RET];
RET = (-9223372036854775808_isize) >> _2.0;
RET = _1.0 as isize;
_6.1.1 = [51398_u16,2856_u16,23103_u16,33897_u16,29832_u16];
RET = _1.0 as isize;
_6.2 = !9479937036210070380_usize;
_7 = false;
RET = -(-9223372036854775808_isize);
RET = -(-9223372036854775808_isize);
_5 = 1_u8 as i64;
Call(_8 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.0 = 241078438_u32 ^ 822958321_u32;
_6.0 = !_7;
_6.0 = _7;
_6.1.1 = [10454_u16,47286_u16,47087_u16,15016_u16,30205_u16];
_6.1.2 = 179_u8;
_1.0 = 819811992_u32;
_7 = _6.0;
_4 = '\u{5a4ff}';
_6.1.0 = _4;
_8 = RET << _6.1.2;
Goto(bb3)
}
bb3 = {
_3 = Adt83::Variant2 { fld0: 231831407231237073944095124405759919161_u128 };
_5 = (-52_i8) as i64;
_2.1 = [_8,_8,_8,_8];
_6.1.1 = [56887_u16,58700_u16,62596_u16,17187_u16,15214_u16];
_8 = 3_i8 as isize;
_7 = !_6.0;
_6.1.1 = [31504_u16,17138_u16,26137_u16,46370_u16,55811_u16];
_1.0 = 24928_u16 as u32;
_10 = [63859_u16,46693_u16,14988_u16,34438_u16,40734_u16];
place!(Field::<u128>(Variant(_3, 2), 0)) = !337216138560086471034478243222362514854_u128;
_1.0 = !3748454066_u32;
_2.1 = [_8,_8,RET,_8];
_10 = _6.1.1;
_8 = RET ^ RET;
Call(_9 = core::intrinsics::transmute(_6.1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = 63369_u16 as isize;
_6.2 = 0_usize;
_6.1.1 = [23869_u16,13628_u16,1209_u16,15446_u16,53991_u16];
_4 = _6.1.0;
_1.1 = 270492859_i32 as f64;
_2.0 = 14031533712447090342713030096476605072_i128 << RET;
SetDiscriminant(_3, 2);
_2.0 = (-118992818876390407211786464745913234319_i128) ^ 37875754682110280537894252692467608620_i128;
_4 = _6.1.0;
place!(Field::<u128>(Variant(_3, 2), 0)) = _1.1 as u128;
_9 = [(-6322_i16),7198_i16,9326_i16,20461_i16,(-11763_i16)];
_6.2 = 7765871637414725251_u64 as usize;
_4 = _6.1.0;
_6.1.1 = _10;
_8 = !RET;
_1.1 = _6.1.2 as f64;
_6.1.1 = [9859_u16,23124_u16,4718_u16,23447_u16,62377_u16];
_5 = 7199244133760960505_i64 - (-6237055861014988183_i64);
_1.1 = _2.0 as f64;
_18 = &_6.1.1;
RET = _8 - _8;
_2.1 = [RET,_8,RET,RET];
Goto(bb5)
}
bb5 = {
_2.2 = &_17;
_7 = !_6.0;
_1.1 = (-1389628783_i32) as f64;
place!(Field::<u128>(Variant(_3, 2), 0)) = _6.0 as u128;
_2.2 = &_17;
_4 = _6.1.0;
_19 = !893564493_i32;
_10 = (*_18);
match _6.1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
179 => bb6,
_ => bb4
}
}
bb6 = {
_2.1 = [RET,RET,_8,_8];
_21 = core::ptr::addr_of!(_8);
_14 = _19 ^ _19;
_9 = [15937_i16,(-29092_i16),14188_i16,21683_i16,(-32243_i16)];
RET = (*_21);
(*_21) = !RET;
_7 = (*_21) >= RET;
SetDiscriminant(_3, 2);
_18 = &_6.1.1;
_2.0 = (-151526666038705703423767110155234950593_i128) | (-76968077540269887398594468726940336157_i128);
_6.0 = _8 <= (*_21);
Goto(bb7)
}
bb7 = {
_11 = _2.0;
_20 = [126_i8,(-7_i8),83_i8,39_i8,(-71_i8)];
_22 = _5 as usize;
_7 = _6.0 ^ _6.0;
_21 = core::ptr::addr_of!((*_21));
_7 = _6.0;
_1.0 = !690945047_u32;
_9 = [5422_i16,(-19685_i16),(-21595_i16),(-18916_i16),(-30336_i16)];
_15 = &_17;
_22 = _2.0 as usize;
_18 = &_10;
place!(Field::<u128>(Variant(_3, 2), 0)) = _1.0 as u128;
_11 = _1.0 as i128;
(*_21) = RET;
(*_21) = RET >> _1.0;
SetDiscriminant(_3, 1);
place!(Field::<i128>(Variant(_3, 1), 7)) = _2.0;
_6.1.3 = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
_16 = &_1.0;
_1.1 = _2.0 as f64;
place!(Field::<i64>(Variant(_3, 1), 6)) = 3846474452181102116_u64 as i64;
_2.2 = &_17;
_2.2 = &_25.0;
_1.1 = _14 as f64;
_5 = !Field::<i64>(Variant(_3, 1), 6);
_2.0 = Field::<i128>(Variant(_3, 1), 7);
Goto(bb8)
}
bb8 = {
place!(Field::<([u8; 4],)>(Variant(_3, 1), 4)).0 = [_6.1.2,_6.1.2,_6.1.2,_6.1.2];
place!(Field::<i64>(Variant(_3, 1), 6)) = 24638_i16 as i64;
_2.2 = &_25.0;
_17 = core::ptr::addr_of!(_6.1.3);
_27 = _14 as f32;
_2.0 = Field::<i128>(Variant(_3, 1), 7);
_6.1.3 = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
(*_17) = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
_24 = _8 * (*_21);
_28.0 = _4;
Call(_23 = fn10(Move(_17), Move(_16), _9, (*_18), _20, Move(_21), _2.1, _24, _24, _24, _2.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2.2 = &_25.0;
place!(Field::<(i8,)>(Variant(_3, 1), 2)) = ((-30_i8),);
place!(Field::<*const isize>(Variant(_3, 1), 1)) = core::ptr::addr_of!(_24);
_14 = _11 as i32;
place!(Field::<(i8,)>(Variant(_3, 1), 2)) = ((-31_i8),);
_4 = _6.1.0;
_11 = Field::<i128>(Variant(_3, 1), 7);
place!(Field::<([u8; 4],)>(Variant(_3, 1), 4)).0 = [_6.1.2,_6.1.2,_6.1.2,_6.1.2];
_28.2 = !_6.1.2;
_28.1 = (*_18);
place!(Field::<f32>(Variant(_3, 1), 3)) = _27;
_14 = _19 >> _11;
_22 = 35936_u16 as usize;
_5 = -Field::<i64>(Variant(_3, 1), 6);
_26 = [Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0];
_24 = _8;
_15 = &_25.0;
_1.0 = !104210046_u32;
_3 = Adt83::Variant2 { fld0: 279556605388268204428020088273781617186_u128 };
RET = 38692_u16 as isize;
_28.1 = [58247_u16,10913_u16,56026_u16,47104_u16,30510_u16];
_30 = [(-26699_i16),23908_i16];
_23 = core::ptr::addr_of_mut!(_27);
_25.1 = Adt17::Variant1 { fld0: _6.0,fld1: _28.0,fld2: _5,fld3: 1588410103179928065_u64,fld4: _11 };
(*_23) = 12466_u16 as f32;
_25.2 = _6.1.0 as u128;
place!(Field::<char>(Variant(_25.1, 1), 1)) = _4;
_8 = _24 * _24;
place!(Field::<u128>(Variant(_3, 2), 0)) = !_25.2;
Goto(bb10)
}
bb10 = {
_15 = &_25.0;
(*_23) = _19 as f32;
_25.1 = Adt17::Variant0 { fld0: _14 };
_18 = &_6.1.1;
(*_23) = _25.2 as f32;
_28.0 = _6.1.0;
_25.1 = Adt17::Variant0 { fld0: _14 };
_14 = _1.1 as i32;
_17 = core::ptr::addr_of!(_6.1.3);
_22 = (-22522_i16) as usize;
_21 = core::ptr::addr_of!(RET);
_10 = [5865_u16,64754_u16,52032_u16,30234_u16,25991_u16];
SetDiscriminant(_3, 1);
_2.2 = &_17;
Goto(bb11)
}
bb11 = {
SetDiscriminant(_25.1, 0);
_29 = &_28.3;
_27 = _1.1 as f32;
place!(Field::<(i8,)>(Variant(_3, 1), 2)) = (98_i8,);
_6.1.2 = _28.2 * _28.2;
Goto(bb12)
}
bb12 = {
_6.0 = _7;
_30 = [(-25853_i16),(-6794_i16)];
_13 = core::ptr::addr_of!(_25.1);
place!(Field::<bool>(Variant(_3, 1), 0)) = !_7;
_6.2 = _22 ^ _22;
_27 = _1.1 as f32;
_16 = &_1.0;
_5 = !2523086955327180488_i64;
RET = !_8;
place!(Field::<bool>(Variant(_3, 1), 0)) = _7;
(*_13) = Adt17::Variant0 { fld0: _14 };
_26 = [Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0,Field::<(i8,)>(Variant(_3, 1), 2).0];
(*_23) = _25.2 as f32;
place!(Field::<i32>(Variant(_25.1, 0), 0)) = _14;
place!(Field::<bool>(Variant(_3, 1), 0)) = Field::<(i8,)>(Variant(_3, 1), 2).0 == Field::<(i8,)>(Variant(_3, 1), 2).0;
_8 = _25.2 as isize;
_18 = &_28.1;
_6.2 = _22 ^ _22;
_10 = [43374_u16,54205_u16,2034_u16,51134_u16,11876_u16];
_28.0 = _6.1.0;
_28.2 = (*_16) as u8;
_29 = &(*_29);
_13 = core::ptr::addr_of!((*_13));
_18 = &(*_18);
_20 = _26;
(*_17) = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
(*_21) = !_8;
match Field::<(i8,)>(Variant(_3, 1), 2).0 {
0 => bb4,
1 => bb9,
2 => bb13,
98 => bb15,
_ => bb14
}
}
bb13 = {
_11 = _2.0;
_20 = [126_i8,(-7_i8),83_i8,39_i8,(-71_i8)];
_22 = _5 as usize;
_7 = _6.0 ^ _6.0;
_21 = core::ptr::addr_of!((*_21));
_7 = _6.0;
_1.0 = !690945047_u32;
_9 = [5422_i16,(-19685_i16),(-21595_i16),(-18916_i16),(-30336_i16)];
_15 = &_17;
_22 = _2.0 as usize;
_18 = &_10;
place!(Field::<u128>(Variant(_3, 2), 0)) = _1.0 as u128;
_11 = _1.0 as i128;
(*_21) = RET;
(*_21) = RET >> _1.0;
SetDiscriminant(_3, 1);
place!(Field::<i128>(Variant(_3, 1), 7)) = _2.0;
_6.1.3 = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
_16 = &_1.0;
_1.1 = _2.0 as f64;
place!(Field::<i64>(Variant(_3, 1), 6)) = 3846474452181102116_u64 as i64;
_2.2 = &_17;
_2.2 = &_25.0;
_1.1 = _14 as f64;
_5 = !Field::<i64>(Variant(_3, 1), 6);
_2.0 = Field::<i128>(Variant(_3, 1), 7);
Goto(bb8)
}
bb14 = {
RET = 63369_u16 as isize;
_6.2 = 0_usize;
_6.1.1 = [23869_u16,13628_u16,1209_u16,15446_u16,53991_u16];
_4 = _6.1.0;
_1.1 = 270492859_i32 as f64;
_2.0 = 14031533712447090342713030096476605072_i128 << RET;
SetDiscriminant(_3, 2);
_2.0 = (-118992818876390407211786464745913234319_i128) ^ 37875754682110280537894252692467608620_i128;
_4 = _6.1.0;
place!(Field::<u128>(Variant(_3, 2), 0)) = _1.1 as u128;
_9 = [(-6322_i16),7198_i16,9326_i16,20461_i16,(-11763_i16)];
_6.2 = 7765871637414725251_u64 as usize;
_4 = _6.1.0;
_6.1.1 = _10;
_8 = !RET;
_1.1 = _6.1.2 as f64;
_6.1.1 = [9859_u16,23124_u16,4718_u16,23447_u16,62377_u16];
_5 = 7199244133760960505_i64 - (-6237055861014988183_i64);
_1.1 = _2.0 as f64;
_18 = &_6.1.1;
RET = _8 - _8;
_2.1 = [RET,_8,RET,RET];
Goto(bb5)
}
bb15 = {
_34.0 = &_25.2;
_27 = (*_16) as f32;
_17 = core::ptr::addr_of!(_6.1.3);
_16 = &(*_16);
(*_13) = Adt17::Variant2 { fld0: _28.2,fld1: 445840518610564850_u64,fld2: Field::<(i8,)>(Variant(_3, 1), 2).0 };
_6.1.0 = _4;
_28.3 = Move(_6.1.3);
_6.1.0 = _28.0;
RET = -_24;
(*_21) = _24;
_19 = _11 as i32;
place!(Field::<([u8; 4],)>(Variant(_3, 1), 4)).0 = [_6.1.2,Field::<u8>(Variant((*_13), 2), 0),Field::<u8>(Variant((*_13), 2), 0),_6.1.2];
_28.2 = _6.1.2;
(*_13) = Adt17::Variant1 { fld0: _6.0,fld1: _28.0,fld2: _5,fld3: 1660058220887247294_u64,fld4: _11 };
(*_21) = 6340_u16 as isize;
(*_17) = &place!(Field::<(i8,)>(Variant(_3, 1), 2)).0;
_25.0 = Move(_17);
_24 = _27 as isize;
(*_23) = (*_16) as f32;
_19 = -_14;
_14 = -_19;
place!(Field::<char>(Variant((*_13), 1), 1)) = _28.0;
_36 = _30;
_2.1 = [(*_21),(*_21),RET,_24];
place!(Field::<i128>(Variant(_3, 1), 7)) = !_2.0;
(*_13) = Adt17::Variant0 { fld0: _19 };
_6.1.2 = _28.2 | _28.2;
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(9_usize, 10_usize, Move(_10), 30_usize, Move(_30), 11_usize, Move(_11), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(9_usize, 24_usize, Move(_24), 7_usize, Move(_7), 36_usize, Move(_36), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const &'static i8,mut _2: &'static u32,mut _3: [i16; 5],mut _4: [u16; 5],mut _5: [i8; 5],mut _6: *const isize,mut _7: [isize; 4],mut _8: isize,mut _9: isize,mut _10: isize,mut _11: [isize; 4]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _12: [i16; 4];
let _13: [u8; 4];
let _14: i32;
let _15: i128;
let _16: i8;
let _17: &'static (f32, u16, ((u32, f64), f64, u16, i128));
let _18: [i8; 2];
let _19: [u32; 2];
let _20: *mut f32;
let _21: ((i8,), *mut bool, ((f32, u16, ((u32, f64), f64, u16, i128)), &'static i8), (i8,));
let _22: &'static &'static u128;
let _23: *const Adt17;
let _24: (u32, f64);
let _25: &'static &'static i8;
let _26: *const &'static i8;
let _27: u32;
let _28: ((i8,), *mut bool, ((f32, u16, ((u32, f64), f64, u16, i128)), &'static i8), (i8,));
let _29: Adt64;
let _30: *mut f32;
let _31: *mut f32;
let _32: f64;
let _33: i128;
let _34: i8;
let _35: f64;
let _36: &'static (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _37: f32;
let _38: ((f32, u16, ((u32, f64), f64, u16, i128)), *mut Adt50, [i16; 4], &'static &'static i8);
let _39: &'static u32;
let _40: [i16; 4];
let _41: (bool, (char, [u16; 5], u8, &'static i8), usize);
let _42: isize;
let _43: isize;
let _44: &'static f64;
let _45: Adt64;
let _46: ([u8; 4],);
let _47: *mut f32;
let _48: [isize; 5];
let _49: bool;
let _50: (f32, u16, ((u32, f64), f64, u16, i128));
let _51: bool;
let _52: f32;
let _53: ();
let _54: ();
{
_6 = core::ptr::addr_of!(_10);
_4 = [42026_u16,24019_u16,39400_u16,45109_u16,45390_u16];
_4 = [28787_u16,25240_u16,63086_u16,58761_u16,12714_u16];
_4 = [28025_u16,1862_u16,49993_u16,13701_u16,31621_u16];
Goto(bb1)
}
bb1 = {
_4 = [41340_u16,42736_u16,14168_u16,15080_u16,45923_u16];
Goto(bb2)
}
bb2 = {
_14 = false as i32;
_9 = (*_6);
(*_6) = _8;
_8 = (-20584_i16) as isize;
_12 = [(-31586_i16),7405_i16,(-8572_i16),(-4316_i16)];
_4 = [52706_u16,11749_u16,50192_u16,24235_u16,57985_u16];
_14 = !(-334266735_i32);
_10 = true as isize;
(*_6) = 6_usize as isize;
_15 = -(-107325503211378831216777041916429188695_i128);
_9 = !_8;
_12 = [(-29040_i16),(-10673_i16),(-19807_i16),(-30332_i16)];
_4 = [19796_u16,59962_u16,38088_u16,6061_u16,59563_u16];
(*_6) = -_8;
_12 = [(-30322_i16),2573_i16,(-5326_i16),19743_i16];
_6 = core::ptr::addr_of!((*_6));
_9 = '\u{bbaad}' as isize;
_13 = [199_u8,198_u8,188_u8,251_u8];
_13 = [238_u8,79_u8,71_u8,131_u8];
_12 = [8623_i16,(-10013_i16),(-32684_i16),(-19393_i16)];
_6 = core::ptr::addr_of!((*_6));
_15 = (-39774986291976744751416582936049453533_i128);
_7 = [_9,_8,(*_6),_10];
_5 = [93_i8,78_i8,31_i8,54_i8,(-46_i8)];
match _15 {
0 => bb3,
300507380628961718711958024495718757923 => bb5,
_ => bb4
}
}
bb3 = {
_4 = [41340_u16,42736_u16,14168_u16,15080_u16,45923_u16];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*_6) = _9;
_9 = _14 as isize;
_9 = (*_6) + (*_6);
_14 = !(-190403912_i32);
_18 = [120_i8,(-30_i8)];
_10 = -_9;
_13 = [55_u8,128_u8,136_u8,47_u8];
_8 = (*_6);
_11 = [_8,(*_6),_10,_9];
_15 = (-75978037505175799448293062706581621331_i128);
_15 = (-32904296629467517038495191216476297327_i128) ^ 10983026025293993207427985647145535967_i128;
_4 = [33340_u16,17847_u16,20578_u16,22380_u16,12607_u16];
_19 = [1146151944_u32,1390368466_u32];
_9 = 7594_u16 as isize;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12 = [3210_i16,(-29342_i16),(-906_i16),(-5077_i16)];
_9 = -(*_6);
_4 = [14788_u16,64748_u16,62500_u16,6302_u16,45196_u16];
(*_6) = _9 >> _15;
_11 = [(*_6),(*_6),(*_6),(*_6)];
_6 = core::ptr::addr_of!((*_6));
_8 = _9 * (*_6);
_21.2.0.2.0.1 = 27633_i16 as f64;
RET = core::ptr::addr_of_mut!(_21.2.0.0);
_21.2.0.0 = 8293267829844326983_i64 as f32;
_21.3.0 = 14_i8;
_16 = _21.3.0;
_21.2.1 = &_16;
_18 = [_16,_21.3.0];
_2 = &_21.2.0.2.0.0;
Goto(bb7)
}
bb7 = {
_13 = [32_u8,193_u8,221_u8,143_u8];
_20 = Move(RET);
_21.2.0.2.1 = _21.2.0.2.0.1;
_21.0 = (_16,);
_24.1 = _21.2.0.2.1 - _21.2.0.2.1;
_21.2.0.2.2 = 23209_u16 >> _14;
_21.2.0.1 = 172_u8 as u16;
_21.0 = (_16,);
_21.2.0.2.3 = '\u{921fc}' as i128;
_1 = core::ptr::addr_of!(_21.2.1);
_21.3 = _21.0;
_2 = &(*_2);
_25 = &(*_1);
_21.0 = (_21.3.0,);
_21.2.0.2.0 = (3989802571_u32, _24.1);
_21.2.0.2.2 = _21.3.0 as u16;
_21.2.0.2.3 = _15 & _15;
(*_1) = &_21.3.0;
(*_1) = &_16;
(*_1) = &_16;
_26 = core::ptr::addr_of!(_21.2.1);
_16 = _21.3.0;
(*_6) = -_8;
match _16 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
14 => bb9,
_ => bb8
}
}
bb8 = {
(*_6) = _9;
_9 = _14 as isize;
_9 = (*_6) + (*_6);
_14 = !(-190403912_i32);
_18 = [120_i8,(-30_i8)];
_10 = -_9;
_13 = [55_u8,128_u8,136_u8,47_u8];
_8 = (*_6);
_11 = [_8,(*_6),_10,_9];
_15 = (-75978037505175799448293062706581621331_i128);
_15 = (-32904296629467517038495191216476297327_i128) ^ 10983026025293993207427985647145535967_i128;
_4 = [33340_u16,17847_u16,20578_u16,22380_u16,12607_u16];
_19 = [1146151944_u32,1390368466_u32];
_9 = 7594_u16 as isize;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb9 = {
_14 = 1639945895_i32;
_25 = &(*_26);
_28.0 = _21.3;
_28.2.0.2.0.0 = _21.2.0.2.0.0 % _21.2.0.2.0.0;
_21.2.0.2.0.1 = _14 as f64;
_24.0 = 103_u8 as u32;
_9 = (*_6) | (*_6);
_28.0 = _21.0;
_15 = _21.2.0.2.3;
_28.3.0 = !_28.0.0;
_28.2.0.2 = (_24, _21.2.0.2.1, _21.2.0.2.2, _15);
_28.2.1 = &_28.3.0;
(*_6) = 99_u8 as isize;
match _28.0.0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
14 => bb16,
_ => bb15
}
}
bb10 = {
(*_6) = _9;
_9 = _14 as isize;
_9 = (*_6) + (*_6);
_14 = !(-190403912_i32);
_18 = [120_i8,(-30_i8)];
_10 = -_9;
_13 = [55_u8,128_u8,136_u8,47_u8];
_8 = (*_6);
_11 = [_8,(*_6),_10,_9];
_15 = (-75978037505175799448293062706581621331_i128);
_15 = (-32904296629467517038495191216476297327_i128) ^ 10983026025293993207427985647145535967_i128;
_4 = [33340_u16,17847_u16,20578_u16,22380_u16,12607_u16];
_19 = [1146151944_u32,1390368466_u32];
_9 = 7594_u16 as isize;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
_13 = [32_u8,193_u8,221_u8,143_u8];
_20 = Move(RET);
_21.2.0.2.1 = _21.2.0.2.0.1;
_21.0 = (_16,);
_24.1 = _21.2.0.2.1 - _21.2.0.2.1;
_21.2.0.2.2 = 23209_u16 >> _14;
_21.2.0.1 = 172_u8 as u16;
_21.0 = (_16,);
_21.2.0.2.3 = '\u{921fc}' as i128;
_1 = core::ptr::addr_of!(_21.2.1);
_21.3 = _21.0;
_2 = &(*_2);
_25 = &(*_1);
_21.0 = (_21.3.0,);
_21.2.0.2.0 = (3989802571_u32, _24.1);
_21.2.0.2.2 = _21.3.0 as u16;
_21.2.0.2.3 = _15 & _15;
(*_1) = &_21.3.0;
(*_1) = &_16;
(*_1) = &_16;
_26 = core::ptr::addr_of!(_21.2.1);
_16 = _21.3.0;
(*_6) = -_8;
match _16 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
14 => bb9,
_ => bb8
}
}
bb12 = {
_14 = false as i32;
_9 = (*_6);
(*_6) = _8;
_8 = (-20584_i16) as isize;
_12 = [(-31586_i16),7405_i16,(-8572_i16),(-4316_i16)];
_4 = [52706_u16,11749_u16,50192_u16,24235_u16,57985_u16];
_14 = !(-334266735_i32);
_10 = true as isize;
(*_6) = 6_usize as isize;
_15 = -(-107325503211378831216777041916429188695_i128);
_9 = !_8;
_12 = [(-29040_i16),(-10673_i16),(-19807_i16),(-30332_i16)];
_4 = [19796_u16,59962_u16,38088_u16,6061_u16,59563_u16];
(*_6) = -_8;
_12 = [(-30322_i16),2573_i16,(-5326_i16),19743_i16];
_6 = core::ptr::addr_of!((*_6));
_9 = '\u{bbaad}' as isize;
_13 = [199_u8,198_u8,188_u8,251_u8];
_13 = [238_u8,79_u8,71_u8,131_u8];
_12 = [8623_i16,(-10013_i16),(-32684_i16),(-19393_i16)];
_6 = core::ptr::addr_of!((*_6));
_15 = (-39774986291976744751416582936049453533_i128);
_7 = [_9,_8,(*_6),_10];
_5 = [93_i8,78_i8,31_i8,54_i8,(-46_i8)];
match _15 {
0 => bb3,
300507380628961718711958024495718757923 => bb5,
_ => bb4
}
}
bb13 = {
(*_6) = _9;
_9 = _14 as isize;
_9 = (*_6) + (*_6);
_14 = !(-190403912_i32);
_18 = [120_i8,(-30_i8)];
_10 = -_9;
_13 = [55_u8,128_u8,136_u8,47_u8];
_8 = (*_6);
_11 = [_8,(*_6),_10,_9];
_15 = (-75978037505175799448293062706581621331_i128);
_15 = (-32904296629467517038495191216476297327_i128) ^ 10983026025293993207427985647145535967_i128;
_4 = [33340_u16,17847_u16,20578_u16,22380_u16,12607_u16];
_19 = [1146151944_u32,1390368466_u32];
_9 = 7594_u16 as isize;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_4 = [41340_u16,42736_u16,14168_u16,15080_u16,45923_u16];
Goto(bb2)
}
bb16 = {
(*_26) = Move(_28.2.1);
_28.0 = (_21.0.0,);
_21.3 = (_28.0.0,);
(*_1) = &_21.0.0;
(*_6) = _9;
_14 = (-265438162_i32) + 1981631072_i32;
_21.2.0.2.0.0 = !_24.0;
match _28.0.0 {
14 => bb18,
_ => bb17
}
}
bb17 = {
_4 = [41340_u16,42736_u16,14168_u16,15080_u16,45923_u16];
Goto(bb2)
}
bb18 = {
_28.2.0 = (_21.2.0.0, _21.2.0.1, _21.2.0.2);
_21.2.0.0 = _28.2.0.0;
_21.3.0 = _21.0.0 << _8;
_28.0 = (_21.0.0,);
_28.3 = (_21.3.0,);
_17 = &_28.2.0;
_19 = [_21.2.0.2.0.0,_21.2.0.2.0.0];
_21.2.0.2.1 = (*_17).2.1;
_27 = (*_17).2.0.0;
_33 = !(*_17).2.3;
_21.0 = _21.3;
_24 = _28.2.0.2.0;
Goto(bb19)
}
bb19 = {
RET = Move(_20);
_28.0.0 = _21.0.0;
_18 = [_21.0.0,_28.0.0];
_28.2.0.2.0 = (_24.0, _28.2.0.2.1);
_16 = _21.0.0 * _21.0.0;
_28.0.0 = 185_u8 as i8;
_34 = _21.2.0.2.0.1 as i8;
_35 = _24.1 - _24.1;
_28.2.0.2.1 = _35 + _28.2.0.2.0.1;
_32 = -_28.2.0.2.1;
_21.2.0 = (_28.2.0.0, (*_17).1, _28.2.0.2);
(*_26) = &_16;
_28.2.0.2.1 = _35 * _21.2.0.2.1;
_20 = Move(RET);
Goto(bb20)
}
bb20 = {
_17 = &_21.2.0;
_21.0.0 = _16;
_28.2.0 = _21.2.0;
_17 = &(*_17);
_38.0.2.2 = (*_17).2.2 - (*_17).2.2;
_21.2.0.2 = (_24, _32, (*_17).1, _33);
(*_26) = &_16;
_38.0.2.2 = _28.2.0.1;
_37 = _28.2.0.0 - _21.2.0.0;
_30 = core::ptr::addr_of_mut!((*_17).0);
_14 = '\u{e694b}' as i32;
_28.2.1 = &_28.3.0;
_10 = _8 & _9;
_8 = (*_6);
_7 = _11;
_7 = [_8,_8,_8,(*_6)];
_21.3.0 = _21.0.0 * _28.3.0;
(*_1) = &_16;
_28.2.0.2.2 = _28.2.0.1 >> _10;
_21.2.0.1 = '\u{44828}' as u16;
(*_1) = Move(_28.2.1);
_38.0.2.0 = (_28.2.0.2.0.0, _21.2.0.2.0.1);
_28.2.0.2.1 = _32;
_21.2.0.2.3 = _28.2.0.2.3;
_41.2 = '\u{cf78e}' as usize;
_41.1.2 = 18_u8;
_38.0 = _28.2.0;
_38.2 = [(-24716_i16),(-24743_i16),(-987_i16),26231_i16];
_28.2.0.2.0.0 = _27;
Call((*_6) = fn11(Move(_26), Move(_1), _7), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_28.2.0 = ((*_17).0, _21.2.0.1, _21.2.0.2);
_21.1 = core::ptr::addr_of_mut!(_41.0);
_6 = core::ptr::addr_of!(_8);
_5 = [_21.3.0,_28.3.0,_28.0.0,_21.3.0,_21.3.0];
_28.2.0.2.0.0 = _38.0.2.0.0;
match _41.1.2 {
0 => bb12,
1 => bb20,
2 => bb6,
3 => bb13,
4 => bb22,
18 => bb24,
_ => bb23
}
}
bb22 = {
(*_6) = _9;
_9 = _14 as isize;
_9 = (*_6) + (*_6);
_14 = !(-190403912_i32);
_18 = [120_i8,(-30_i8)];
_10 = -_9;
_13 = [55_u8,128_u8,136_u8,47_u8];
_8 = (*_6);
_11 = [_8,(*_6),_10,_9];
_15 = (-75978037505175799448293062706581621331_i128);
_15 = (-32904296629467517038495191216476297327_i128) ^ 10983026025293993207427985647145535967_i128;
_4 = [33340_u16,17847_u16,20578_u16,22380_u16,12607_u16];
_19 = [1146151944_u32,1390368466_u32];
_9 = 7594_u16 as isize;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb23 = {
_14 = false as i32;
_9 = (*_6);
(*_6) = _8;
_8 = (-20584_i16) as isize;
_12 = [(-31586_i16),7405_i16,(-8572_i16),(-4316_i16)];
_4 = [52706_u16,11749_u16,50192_u16,24235_u16,57985_u16];
_14 = !(-334266735_i32);
_10 = true as isize;
(*_6) = 6_usize as isize;
_15 = -(-107325503211378831216777041916429188695_i128);
_9 = !_8;
_12 = [(-29040_i16),(-10673_i16),(-19807_i16),(-30332_i16)];
_4 = [19796_u16,59962_u16,38088_u16,6061_u16,59563_u16];
(*_6) = -_8;
_12 = [(-30322_i16),2573_i16,(-5326_i16),19743_i16];
_6 = core::ptr::addr_of!((*_6));
_9 = '\u{bbaad}' as isize;
_13 = [199_u8,198_u8,188_u8,251_u8];
_13 = [238_u8,79_u8,71_u8,131_u8];
_12 = [8623_i16,(-10013_i16),(-32684_i16),(-19393_i16)];
_6 = core::ptr::addr_of!((*_6));
_15 = (-39774986291976744751416582936049453533_i128);
_7 = [_9,_8,(*_6),_10];
_5 = [93_i8,78_i8,31_i8,54_i8,(-46_i8)];
match _15 {
0 => bb3,
300507380628961718711958024495718757923 => bb5,
_ => bb4
}
}
bb24 = {
_7 = _11;
_21.2.0 = _38.0;
_38.0.2 = _21.2.0.2;
(*_6) = _9 | _9;
_16 = !_21.3.0;
_11 = [(*_6),(*_6),(*_6),_9];
_39 = &_21.2.0.2.0.0;
_21.2.0.1 = _38.0.2.2;
_3 = [2370_i16,10316_i16,(-21683_i16),23953_i16,(-121_i16)];
_38.0.0 = -_21.2.0.0;
_25 = &_28.2.1;
_38.3 = Move(_25);
_21.2.0.0 = -_37;
_39 = &_28.2.0.2.0.0;
_39 = &_24.0;
_30 = core::ptr::addr_of_mut!(_28.2.0.0);
RET = core::ptr::addr_of_mut!(_37);
Goto(bb25)
}
bb25 = {
Call(_53 = dump_var(10_usize, 3_usize, Move(_3), 34_usize, Move(_34), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_53 = dump_var(10_usize, 4_usize, Move(_4), 7_usize, Move(_7), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_53 = dump_var(10_usize, 15_usize, Move(_15), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: *const &'static i8,mut _2: *const &'static i8,mut _3: [isize; 4]) -> isize {
mir! {
type RET = isize;
let _4: *const Adt17;
let _5: (Adt33, &'static [i16; 4]);
let _6: bool;
let _7: char;
let _8: Adt33;
let _9: &'static u128;
let _10: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _11: (&'static u128,);
let _12: u16;
let _13: [i16; 5];
let _14: i8;
let _15: u128;
let _16: Adt64;
let _17: f32;
let _18: i32;
let _19: [u32; 7];
let _20: isize;
let _21: i32;
let _22: [i16; 2];
let _23: ((f32, u16, ((u32, f64), f64, u16, i128)), *mut Adt50, [i16; 4], &'static &'static i8);
let _24: char;
let _25: bool;
let _26: *const isize;
let _27: char;
let _28: &'static *const &'static i8;
let _29: ((u32, f64), f64, u16, i128);
let _30: *const *const i8;
let _31: usize;
let _32: usize;
let _33: (&'static u128,);
let _34: u16;
let _35: isize;
let _36: i64;
let _37: isize;
let _38: isize;
let _39: &'static [u16; 5];
let _40: *mut f32;
let _41: u16;
let _42: &'static isize;
let _43: u64;
let _44: f32;
let _45: ((u32, f64), f64, u16, i128);
let _46: &'static *const &'static i8;
let _47: isize;
let _48: [u8; 4];
let _49: ();
let _50: ();
{
_2 = Move(_1);
_1 = Move(_2);
RET = !103_isize;
_5.0.fld1 = 168494575738591203155456761567894497836_i128 & 162191997125346690880134901292667390742_i128;
_5.0.fld1 = 9598772450505060949866092459538034466_i128 - (-21382807823198347180389901806395331089_i128);
_4 = core::ptr::addr_of!(_5.0.fld4);
_3 = [RET,RET,RET,RET];
(*_4) = Adt17::Variant2 { fld0: 213_u8,fld1: 9375594793436394517_u64,fld2: 116_i8 };
(*_4) = Adt17::Variant1 { fld0: true,fld1: '\u{88b2e}',fld2: (-6397345429114002259_i64),fld3: 9788441666562348019_u64,fld4: _5.0.fld1 };
_5.0.fld2 = 4542747441278627689_i64;
place!(Field::<bool>(Variant((*_4), 1), 0)) = false;
_2 = Move(_1);
_5.0.fld3 = 6_usize as i8;
place!(Field::<bool>(Variant((*_4), 1), 0)) = !false;
_5.0.fld0 = !Field::<bool>(Variant((*_4), 1), 0);
place!(Field::<i128>(Variant((*_4), 1), 4)) = _5.0.fld1 << _5.0.fld1;
place!(Field::<bool>(Variant((*_4), 1), 0)) = !_5.0.fld0;
_5.0.fld1 = RET as i128;
RET = Field::<i128>(Variant((*_4), 1), 4) as isize;
place!(Field::<u64>(Variant((*_4), 1), 3)) = 12892622938443533923_u64;
place!(Field::<i64>(Variant((*_4), 1), 2)) = Field::<bool>(Variant((*_4), 1), 0) as i64;
_5.0.fld3 = (-64_i8);
(*_4) = Adt17::Variant1 { fld0: _5.0.fld0,fld1: '\u{53b49}',fld2: _5.0.fld2,fld3: 16612796806669550472_u64,fld4: _5.0.fld1 };
place!(Field::<u64>(Variant((*_4), 1), 3)) = 10411596927579221075_u64;
(*_4) = Adt17::Variant2 { fld0: 106_u8,fld1: 14031465428882769882_u64,fld2: _5.0.fld3 };
Goto(bb1)
}
bb1 = {
place!(Field::<u8>(Variant((*_4), 2), 0)) = 249_u8 | 12_u8;
_4 = core::ptr::addr_of!(_5.0.fld4);
_3 = [RET,RET,RET,RET];
place!(Field::<i8>(Variant((*_4), 2), 2)) = _5.0.fld3;
place!(Field::<u64>(Variant(_5.0.fld4, 2), 1)) = !2492458217658513272_u64;
place!(Field::<u64>(Variant((*_4), 2), 1)) = _5.0.fld1 as u64;
place!(Field::<u8>(Variant((*_4), 2), 0)) = !111_u8;
place!(Field::<u8>(Variant(_5.0.fld4, 2), 0)) = _5.0.fld1 as u8;
_7 = '\u{f613f}';
place!(Field::<i8>(Variant((*_4), 2), 2)) = 11626379533021023295_usize as i8;
_6 = _5.0.fld0;
place!(Field::<i8>(Variant(_5.0.fld4, 2), 2)) = _5.0.fld3;
place!(Field::<u8>(Variant((*_4), 2), 0)) = 185_u8 << RET;
place!(Field::<u8>(Variant(_5.0.fld4, 2), 0)) = !23_u8;
_5.0.fld4 = Adt17::Variant0 { fld0: (-1201147096_i32) };
(*_4) = Adt17::Variant0 { fld0: 1702580325_i32 };
_8.fld3 = 312211706681692135804032254464023126958_u128 as i8;
_1 = Move(_2);
Goto(bb2)
}
bb2 = {
(*_4) = Adt17::Variant2 { fld0: 218_u8,fld1: 3570984529269605044_u64,fld2: _5.0.fld3 };
_1 = core::ptr::addr_of!(_10.2.3);
_8.fld3 = -Field::<i8>(Variant(_5.0.fld4, 2), 2);
_10.2.0 = _7;
_5.0.fld1 = !40774720068191352457114953892915697529_i128;
place!(Field::<u64>(Variant((*_4), 2), 1)) = 11703528392821082355_u64;
_2 = core::ptr::addr_of!((*_1));
_6 = _5.0.fld0;
_8.fld0 = _6;
Call((*_4) = fn12(_5.0.fld0, Move(_4), _7, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_1) = &_8.fld3;
(*_2) = &_5.0.fld3;
_8.fld1 = !_5.0.fld1;
_8.fld4 = Adt17::Variant2 { fld0: 238_u8,fld1: 9624245212593055081_u64,fld2: _5.0.fld3 };
_10.0.2 = !49444_u16;
_10.0.1 = 236_u8 as f64;
_8.fld2 = !_5.0.fld2;
_6 = _5.0.fld0 & _5.0.fld0;
SetDiscriminant(_5.0.fld4, 2);
place!(Field::<u64>(Variant(_5.0.fld4, 2), 1)) = 9614297522439832630_u64;
Goto(bb4)
}
bb4 = {
_5.0.fld4 = Adt17::Variant0 { fld0: (-1074656857_i32) };
_5.0.fld2 = _8.fld2;
_2 = core::ptr::addr_of!((*_1));
match Field::<i8>(Variant(_8.fld4, 2), 2) {
0 => bb3,
340282366920938463463374607431768211392 => bb5,
_ => bb2
}
}
bb5 = {
(*_1) = &_5.0.fld3;
_10.2.2 = !71_u8;
_14 = _8.fld3;
_15 = 277141900218708107868312559968500798010_u128 + 333752906774124022725110805206869789317_u128;
_2 = Move(_1);
_18 = -1198202486_i32;
_7 = _10.2.0;
_5.0.fld1 = _14 as i128;
_4 = core::ptr::addr_of!(_8.fld4);
_19 = [1373785732_u32,333967732_u32,1477363875_u32,2048571453_u32,179237506_u32,2249520219_u32,2493638632_u32];
place!(Field::<u64>(Variant(_8.fld4, 2), 1)) = !400584523545156441_u64;
(*_4) = Adt17::Variant0 { fld0: _18 };
_5.0 = Adt33 { fld0: _6,fld1: _8.fld1,fld2: _8.fld2,fld3: _8.fld3,fld4: Move((*_4)) };
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 9649423300848844461_u64,fld2: _8.fld3 };
place!(Field::<u64>(Variant(_8.fld4, 2), 1)) = 8746664137170586794_u64 | 2688689933698274688_u64;
Goto(bb6)
}
bb6 = {
_1 = core::ptr::addr_of!(_10.2.3);
place!(Field::<i8>(Variant(_8.fld4, 2), 2)) = _8.fld3;
_10.2.3 = &_14;
RET = -(-9223372036854775808_isize);
_23.0.2.2 = _8.fld0 as u16;
_7 = _10.2.0;
_8.fld3 = _8.fld0 as i8;
_2 = Move(_1);
_23.0.2.3 = -_5.0.fld1;
_5.0 = Adt33 { fld0: _6,fld1: _8.fld1,fld2: _8.fld2,fld3: Field::<i8>(Variant((*_4), 2), 2),fld4: Move((*_4)) };
_5.0.fld2 = _18 as i64;
_8.fld2 = Field::<u64>(Variant(_5.0.fld4, 2), 1) as i64;
_23.2 = [5704_i16,(-2081_i16),(-31531_i16),(-6972_i16)];
(*_4) = Adt17::Variant0 { fld0: _18 };
_7 = _10.2.0;
place!(Field::<u64>(Variant(_5.0.fld4, 2), 1)) = 6800467252758602035_u64;
_5.1 = &_23.2;
_10.1 = RET;
_24 = _10.2.0;
RET = _10.1 << Field::<i8>(Variant(_5.0.fld4, 2), 2);
_8 = Move(_5.0);
_5.0.fld1 = _8.fld1 * _8.fld1;
_10.0.0.1 = _10.0.1;
_21 = _18;
_20 = _10.1 & RET;
Call(_1 = fn13(Move(_2), Move(_5.1), Move(_10.2.3), Move(_4), Move(_8), _3, _23.0.2.2, _3, _20, _19, _24), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_12 = _10.0.2 << RET;
_3 = [_20,RET,_20,RET];
_23.0.1 = RET as u16;
_5.0.fld2 = !210185700834454468_i64;
_10.2.1 = [_12,_23.0.1,_12,_23.0.1,_12];
_23.0.0 = (-32043_i16) as f32;
_25 = _23.0.1 != _10.0.2;
_5.1 = &_23.2;
_5.0.fld3 = _14 | _14;
_10.0.0 = (4264871722_u32, _10.0.1);
_8.fld1 = -_5.0.fld1;
_14 = _5.0.fld3;
_8.fld0 = !_25;
_10.2.0 = _7;
_23.0.2 = (_10.0.0, _10.0.0.1, _23.0.1, _5.0.fld1);
_5.0.fld4 = Adt17::Variant1 { fld0: _8.fld0,fld1: _10.2.0,fld2: _5.0.fld2,fld3: 3825110597137800449_u64,fld4: _23.0.2.3 };
_29 = (_23.0.2.0, _23.0.2.1, _23.0.2.2, _5.0.fld1);
_29.2 = !_12;
place!(Field::<i128>(Variant(_5.0.fld4, 1), 4)) = 4994041350694370379_u64 as i128;
_17 = _29.0.0 as f32;
_24 = _7;
_9 = &_15;
_23.3 = &_10.2.3;
Goto(bb8)
}
bb8 = {
_18 = _21;
_23.3 = &_10.2.3;
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 17656990989390366530_u64,fld2: _14 };
_10.0.0.1 = _23.0.2.1 * _23.0.2.1;
_6 = _25 >= _25;
_20 = -RET;
place!(Field::<i8>(Variant(_8.fld4, 2), 2)) = _14 & _14;
_10.0.3 = _6 as i128;
_23.0.2.2 = _29.2 + _29.2;
_12 = !_29.2;
_10.2.3 = &_14;
_23.0.1 = !_29.2;
_12 = _29.1 as u16;
_8.fld0 = Field::<i8>(Variant(_8.fld4, 2), 2) < Field::<i8>(Variant(_8.fld4, 2), 2);
_26 = core::ptr::addr_of!(_10.1);
place!(Field::<i64>(Variant(_5.0.fld4, 1), 2)) = 307388548354375431_u64 as i64;
_27 = _7;
_26 = core::ptr::addr_of!((*_26));
_22 = [16497_i16,21022_i16];
_8.fld2 = !Field::<i64>(Variant(_5.0.fld4, 1), 2);
_10.0.0 = (_29.0.0, _10.0.1);
(*_26) = _20;
_29.0 = _23.0.2.0;
_4 = core::ptr::addr_of!(_8.fld4);
(*_4) = Adt17::Variant1 { fld0: _6,fld1: _10.2.0,fld2: _8.fld2,fld3: 9525144143723752203_u64,fld4: _5.0.fld1 };
_33 = (Move(_9),);
match _23.0.2.0.0 {
0 => bb1,
1 => bb7,
2 => bb9,
4264871722 => bb11,
_ => bb10
}
}
bb9 = {
_5.0.fld4 = Adt17::Variant0 { fld0: (-1074656857_i32) };
_5.0.fld2 = _8.fld2;
_2 = core::ptr::addr_of!((*_1));
match Field::<i8>(Variant(_8.fld4, 2), 2) {
0 => bb3,
340282366920938463463374607431768211392 => bb5,
_ => bb2
}
}
bb10 = {
_1 = core::ptr::addr_of!(_10.2.3);
place!(Field::<i8>(Variant(_8.fld4, 2), 2)) = _8.fld3;
_10.2.3 = &_14;
RET = -(-9223372036854775808_isize);
_23.0.2.2 = _8.fld0 as u16;
_7 = _10.2.0;
_8.fld3 = _8.fld0 as i8;
_2 = Move(_1);
_23.0.2.3 = -_5.0.fld1;
_5.0 = Adt33 { fld0: _6,fld1: _8.fld1,fld2: _8.fld2,fld3: Field::<i8>(Variant((*_4), 2), 2),fld4: Move((*_4)) };
_5.0.fld2 = _18 as i64;
_8.fld2 = Field::<u64>(Variant(_5.0.fld4, 2), 1) as i64;
_23.2 = [5704_i16,(-2081_i16),(-31531_i16),(-6972_i16)];
(*_4) = Adt17::Variant0 { fld0: _18 };
_7 = _10.2.0;
place!(Field::<u64>(Variant(_5.0.fld4, 2), 1)) = 6800467252758602035_u64;
_5.1 = &_23.2;
_10.1 = RET;
_24 = _10.2.0;
RET = _10.1 << Field::<i8>(Variant(_5.0.fld4, 2), 2);
_8 = Move(_5.0);
_5.0.fld1 = _8.fld1 * _8.fld1;
_10.0.0.1 = _10.0.1;
_21 = _18;
_20 = _10.1 & RET;
Call(_1 = fn13(Move(_2), Move(_5.1), Move(_10.2.3), Move(_4), Move(_8), _3, _23.0.2.2, _3, _20, _19, _24), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_23.3 = &_10.2.3;
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 14318593643618923221_u64,fld2: _14 };
_10.0.0.0 = _29.0.0 % _23.0.2.0.0;
_7 = _10.2.0;
_2 = Move(_1);
_8.fld2 = Field::<i64>(Variant(_5.0.fld4, 1), 2);
_3 = [_10.1,(*_26),(*_26),_10.1];
_33.0 = &_15;
_23.3 = &_10.2.3;
_12 = 11944753619560733170_usize as u16;
_29.3 = _20 as i128;
_34 = _29.2 ^ _12;
_10.0.0 = (_23.0.2.0.0, _23.0.2.1);
_11.0 = &_15;
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 12760976741061303351_u64,fld2: _14 };
_32 = Field::<i64>(Variant(_5.0.fld4, 1), 2) as usize;
RET = Field::<bool>(Variant(_5.0.fld4, 1), 0) as isize;
place!(Field::<u64>(Variant((*_4), 2), 1)) = 1456024704666377494_u64;
Goto(bb12)
}
bb12 = {
_37 = _10.1 * RET;
_31 = _32 * _32;
SetDiscriminant(_8.fld4, 0);
(*_4) = Adt17::Variant0 { fld0: _18 };
RET = 4226613300149043342_u64 as isize;
_29.0 = (_23.0.2.0.0, _10.0.0.1);
SetDiscriminant((*_4), 1);
place!(Field::<char>(Variant(_8.fld4, 1), 1)) = _24;
place!(Field::<u64>(Variant(_5.0.fld4, 1), 3)) = !13962575795019384482_u64;
_10.0.3 = _8.fld1 | _5.0.fld1;
(*_4) = Move(_5.0.fld4);
_11.0 = &_15;
_10.0.0 = (_23.0.2.0.0, _23.0.2.1);
_15 = 10996608093197159101084460295775219744_u128 ^ 7409225609753562680432738118583833332_u128;
_29.1 = -_29.0.1;
_45.3 = _15 as i128;
match _29.0.0 {
0 => bb1,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
4264871722 => bb19,
_ => bb18
}
}
bb13 = {
(*_1) = &_5.0.fld3;
_10.2.2 = !71_u8;
_14 = _8.fld3;
_15 = 277141900218708107868312559968500798010_u128 + 333752906774124022725110805206869789317_u128;
_2 = Move(_1);
_18 = -1198202486_i32;
_7 = _10.2.0;
_5.0.fld1 = _14 as i128;
_4 = core::ptr::addr_of!(_8.fld4);
_19 = [1373785732_u32,333967732_u32,1477363875_u32,2048571453_u32,179237506_u32,2249520219_u32,2493638632_u32];
place!(Field::<u64>(Variant(_8.fld4, 2), 1)) = !400584523545156441_u64;
(*_4) = Adt17::Variant0 { fld0: _18 };
_5.0 = Adt33 { fld0: _6,fld1: _8.fld1,fld2: _8.fld2,fld3: _8.fld3,fld4: Move((*_4)) };
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 9649423300848844461_u64,fld2: _8.fld3 };
place!(Field::<u64>(Variant(_8.fld4, 2), 1)) = 8746664137170586794_u64 | 2688689933698274688_u64;
Goto(bb6)
}
bb14 = {
_5.0.fld4 = Adt17::Variant0 { fld0: (-1074656857_i32) };
_5.0.fld2 = _8.fld2;
_2 = core::ptr::addr_of!((*_1));
match Field::<i8>(Variant(_8.fld4, 2), 2) {
0 => bb3,
340282366920938463463374607431768211392 => bb5,
_ => bb2
}
}
bb15 = {
_5.0.fld4 = Adt17::Variant0 { fld0: (-1074656857_i32) };
_5.0.fld2 = _8.fld2;
_2 = core::ptr::addr_of!((*_1));
match Field::<i8>(Variant(_8.fld4, 2), 2) {
0 => bb3,
340282366920938463463374607431768211392 => bb5,
_ => bb2
}
}
bb16 = {
_18 = _21;
_23.3 = &_10.2.3;
_8.fld4 = Adt17::Variant2 { fld0: _10.2.2,fld1: 17656990989390366530_u64,fld2: _14 };
_10.0.0.1 = _23.0.2.1 * _23.0.2.1;
_6 = _25 >= _25;
_20 = -RET;
place!(Field::<i8>(Variant(_8.fld4, 2), 2)) = _14 & _14;
_10.0.3 = _6 as i128;
_23.0.2.2 = _29.2 + _29.2;
_12 = !_29.2;
_10.2.3 = &_14;
_23.0.1 = !_29.2;
_12 = _29.1 as u16;
_8.fld0 = Field::<i8>(Variant(_8.fld4, 2), 2) < Field::<i8>(Variant(_8.fld4, 2), 2);
_26 = core::ptr::addr_of!(_10.1);
place!(Field::<i64>(Variant(_5.0.fld4, 1), 2)) = 307388548354375431_u64 as i64;
_27 = _7;
_26 = core::ptr::addr_of!((*_26));
_22 = [16497_i16,21022_i16];
_8.fld2 = !Field::<i64>(Variant(_5.0.fld4, 1), 2);
_10.0.0 = (_29.0.0, _10.0.1);
(*_26) = _20;
_29.0 = _23.0.2.0;
_4 = core::ptr::addr_of!(_8.fld4);
(*_4) = Adt17::Variant1 { fld0: _6,fld1: _10.2.0,fld2: _8.fld2,fld3: 9525144143723752203_u64,fld4: _5.0.fld1 };
_33 = (Move(_9),);
match _23.0.2.0.0 {
0 => bb1,
1 => bb7,
2 => bb9,
4264871722 => bb11,
_ => bb10
}
}
bb17 = {
_12 = _10.0.2 << RET;
_3 = [_20,RET,_20,RET];
_23.0.1 = RET as u16;
_5.0.fld2 = !210185700834454468_i64;
_10.2.1 = [_12,_23.0.1,_12,_23.0.1,_12];
_23.0.0 = (-32043_i16) as f32;
_25 = _23.0.1 != _10.0.2;
_5.1 = &_23.2;
_5.0.fld3 = _14 | _14;
_10.0.0 = (4264871722_u32, _10.0.1);
_8.fld1 = -_5.0.fld1;
_14 = _5.0.fld3;
_8.fld0 = !_25;
_10.2.0 = _7;
_23.0.2 = (_10.0.0, _10.0.0.1, _23.0.1, _5.0.fld1);
_5.0.fld4 = Adt17::Variant1 { fld0: _8.fld0,fld1: _10.2.0,fld2: _5.0.fld2,fld3: 3825110597137800449_u64,fld4: _23.0.2.3 };
_29 = (_23.0.2.0, _23.0.2.1, _23.0.2.2, _5.0.fld1);
_29.2 = !_12;
place!(Field::<i128>(Variant(_5.0.fld4, 1), 4)) = 4994041350694370379_u64 as i128;
_17 = _29.0.0 as f32;
_24 = _7;
_9 = &_15;
_23.3 = &_10.2.3;
Goto(bb8)
}
bb18 = {
(*_1) = &_8.fld3;
(*_2) = &_5.0.fld3;
_8.fld1 = !_5.0.fld1;
_8.fld4 = Adt17::Variant2 { fld0: 238_u8,fld1: 9624245212593055081_u64,fld2: _5.0.fld3 };
_10.0.2 = !49444_u16;
_10.0.1 = 236_u8 as f64;
_8.fld2 = !_5.0.fld2;
_6 = _5.0.fld0 & _5.0.fld0;
SetDiscriminant(_5.0.fld4, 2);
place!(Field::<u64>(Variant(_5.0.fld4, 2), 1)) = 9614297522439832630_u64;
Goto(bb4)
}
bb19 = {
_45.3 = _5.0.fld1;
place!(Field::<i128>(Variant((*_4), 1), 4)) = _45.3;
_8.fld3 = _14 + _5.0.fld3;
_40 = core::ptr::addr_of_mut!(_44);
_13 = [(-28812_i16),(-10708_i16),21410_i16,15535_i16,(-16126_i16)];
_38 = Field::<char>(Variant((*_4), 1), 1) as isize;
_42 = &RET;
_15 = 268627580310713022704649672235925291103_u128;
_9 = &_15;
_45.3 = Field::<i128>(Variant(_8.fld4, 1), 4) & _23.0.2.3;
_10.2.0 = _27;
place!(Field::<i64>(Variant(_8.fld4, 1), 2)) = _31 as i64;
(*_4) = Adt17::Variant2 { fld0: _10.2.2,fld1: 11204238317910937745_u64,fld2: _14 };
_5.0.fld3 = !_14;
_8.fld4 = Adt17::Variant0 { fld0: _21 };
_46 = &_1;
_29.0 = (_23.0.2.0.0, _23.0.2.0.1);
_41 = _34 * _23.0.2.2;
_24 = _10.2.0;
_43 = 17943584599927031027_u64;
_48 = [_10.2.2,_10.2.2,_10.2.2,_10.2.2];
_28 = Move(_46);
RET = (*_9) as isize;
_45.2 = _8.fld0 as u16;
_33.0 = &(*_9);
SetDiscriminant(_8.fld4, 2);
_10.0.0 = (_29.0.0, _23.0.2.1);
Goto(bb20)
}
bb20 = {
Call(_49 = dump_var(11_usize, 21_usize, Move(_21), 13_usize, Move(_13), 48_usize, Move(_48), 22_usize, Move(_22)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_49 = dump_var(11_usize, 6_usize, Move(_6), 41_usize, Move(_41), 43_usize, Move(_43), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_49 = dump_var(11_usize, 15_usize, Move(_15), 37_usize, Move(_37), 3_usize, Move(_3), 50_usize, _50), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: *const Adt17,mut _3: char,mut _4: isize) -> Adt17 {
mir! {
type RET = Adt17;
let _5: isize;
let _6: Adt50;
let _7: *const isize;
let _8: &'static u128;
let _9: ((i8,), *mut bool, ((f32, u16, ((u32, f64), f64, u16, i128)), &'static i8), (i8,));
let _10: isize;
let _11: Adt83;
let _12: &'static i8;
let _13: char;
let _14: [u8; 4];
let _15: f64;
let _16: bool;
let _17: bool;
let _18: char;
let _19: *mut f32;
let _20: f32;
let _21: ([u8; 4],);
let _22: &'static &'static isize;
let _23: ();
let _24: ();
{
RET = Adt17::Variant0 { fld0: 1468007384_i32 };
_2 = core::ptr::addr_of!(RET);
(*_2) = Adt17::Variant2 { fld0: 188_u8,fld1: 6793389627338079905_u64,fld2: 124_i8 };
place!(Field::<i8>(Variant(RET, 2), 2)) = 109_i8;
place!(Field::<u64>(Variant((*_2), 2), 1)) = 6825903326834227401_u64 ^ 17926509472302928898_u64;
place!(Field::<u64>(Variant((*_2), 2), 1)) = 16670797717630517153_u64;
place!(Field::<i8>(Variant(RET, 2), 2)) = 119_i8;
(*_2) = Adt17::Variant2 { fld0: 183_u8,fld1: 6904504714106263196_u64,fld2: (-28_i8) };
(*_2) = Adt17::Variant1 { fld0: _1,fld1: _3,fld2: 5883315931606237635_i64,fld3: 5989311516934866834_u64,fld4: (-137858113039881547319371301820363718805_i128) };
_6.fld0.2.0.0 = 811751593_u32 - 2843504694_u32;
_6.fld0.2.0.0 = 11687577770550330210_usize as u32;
_6.fld0.1 = _4 as u16;
place!(Field::<i128>(Variant((*_2), 1), 4)) = -(-128583268830483503316609689261350998232_i128);
place!(Field::<i64>(Variant(RET, 1), 2)) = 6948813471012069302_i64;
place!(Field::<u64>(Variant(RET, 1), 3)) = !16753586085722444119_u64;
place!(Field::<u64>(Variant((*_2), 1), 3)) = 5266419143289300031_u64 - 18130852198564709555_u64;
_6.fld0.2.0.1 = _6.fld0.2.0.0 as f64;
_1 = Field::<bool>(Variant(RET, 1), 0) & Field::<bool>(Variant((*_2), 1), 0);
_6.fld0.2.1 = _6.fld0.2.0.1;
(*_2) = Adt17::Variant0 { fld0: 1564058834_i32 };
(*_2) = Adt17::Variant0 { fld0: 50963055_i32 };
_6.fld0.0 = 276625534541122381609207816330158380117_u128 as f32;
_1 = _6.fld0.1 >= _6.fld0.1;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_6.fld0.2.2 = !_6.fld0.1;
_9.2.0.2 = (_6.fld0.2.0, _6.fld0.2.0.1, _6.fld0.2.2, 150345338409567971330586195374775515443_i128);
_6.fld0.2 = (_9.2.0.2.0, _9.2.0.2.1, _9.2.0.2.2, _9.2.0.2.3);
_5 = _9.2.0.2.2 as isize;
_6.fld0.2 = _9.2.0.2;
place!(Field::<i32>(Variant(RET, 0), 0)) = (-1713521652_i32) & (-204704053_i32);
_9.2.1 = &_9.0.0;
_9.2.1 = &_9.0.0;
_10 = _5;
_9.3 = (68_i8,);
_9.2.0 = _6.fld0;
place!(Field::<i32>(Variant((*_2), 0), 0)) = _6.fld0.2.0.0 as i32;
place!(Field::<i32>(Variant((*_2), 0), 0)) = (-931587882_i32);
_6.fld0.0 = -_9.2.0.0;
_6.fld2 = [(-27813_i16),(-28027_i16),31555_i16,(-21651_i16),25611_i16];
_1 = true ^ true;
_3 = '\u{b3b15}';
_9.2.1 = &_9.0.0;
_6.fld0.2.0.0 = !_9.2.0.2.0.0;
_6.fld0.2.3 = _6.fld0.0 as i128;
_9.2.0.2.2 = _6.fld0.2.1 as u16;
Goto(bb3)
}
bb3 = {
_7 = core::ptr::addr_of!(_4);
_6.fld1.0 = [6_u8,175_u8,183_u8,39_u8];
_9.2.0.2.0 = _6.fld0.2.0;
_9.2.1 = &_9.3.0;
SetDiscriminant((*_2), 0);
_6.fld0.1 = !_9.2.0.1;
_6.fld0.2 = _9.2.0.2;
_9.0 = (_9.3.0,);
_9.2.0.2.0.1 = -_9.2.0.2.1;
_11 = Adt83::Variant2 { fld0: 234414321675951860792137200679124452243_u128 };
place!(Field::<i32>(Variant(RET, 0), 0)) = _9.3.0 as i32;
_9.2.0.0 = -_6.fld0.0;
_9.1 = core::ptr::addr_of_mut!(_1);
_6.fld0 = (_9.2.0.0, _9.2.0.2.2, _9.2.0.2);
_1 = false;
_6.fld0.2.0.1 = -_9.2.0.2.0.1;
_9.2.0.1 = _6.fld0.1;
_9.2.0.2.0.0 = _6.fld0.2.0.0;
match _6.fld0.2.3 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
150345338409567971330586195374775515443 => bb9,
_ => bb8
}
}
bb4 = {
_6.fld0.2.2 = !_6.fld0.1;
_9.2.0.2 = (_6.fld0.2.0, _6.fld0.2.0.1, _6.fld0.2.2, 150345338409567971330586195374775515443_i128);
_6.fld0.2 = (_9.2.0.2.0, _9.2.0.2.1, _9.2.0.2.2, _9.2.0.2.3);
_5 = _9.2.0.2.2 as isize;
_6.fld0.2 = _9.2.0.2;
place!(Field::<i32>(Variant(RET, 0), 0)) = (-1713521652_i32) & (-204704053_i32);
_9.2.1 = &_9.0.0;
_9.2.1 = &_9.0.0;
_10 = _5;
_9.3 = (68_i8,);
_9.2.0 = _6.fld0;
place!(Field::<i32>(Variant((*_2), 0), 0)) = _6.fld0.2.0.0 as i32;
place!(Field::<i32>(Variant((*_2), 0), 0)) = (-931587882_i32);
_6.fld0.0 = -_9.2.0.0;
_6.fld2 = [(-27813_i16),(-28027_i16),31555_i16,(-21651_i16),25611_i16];
_1 = true ^ true;
_3 = '\u{b3b15}';
_9.2.1 = &_9.0.0;
_6.fld0.2.0.0 = !_9.2.0.2.0.0;
_6.fld0.2.3 = _6.fld0.0 as i128;
_9.2.0.2.2 = _6.fld0.2.1 as u16;
Goto(bb3)
}
bb5 = {
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
(*_7) = _10 - _10;
_9.2.1 = &_9.3.0;
_9.1 = core::ptr::addr_of_mut!(_1);
_6.fld2 = [23153_i16,(-19806_i16),(-5698_i16),(-6003_i16),(-25479_i16)];
_6.fld0.2.1 = _9.2.0.2.0.1 + _6.fld0.2.0.1;
_9.0 = _9.3;
_4 = _10;
_2 = core::ptr::addr_of!(RET);
place!(Field::<i32>(Variant((*_2), 0), 0)) = (-7984_i16) as i32;
_9.2.0.2.2 = !_9.2.0.1;
_9.2.0.0 = _6.fld0.0 * _6.fld0.0;
_1 = true | false;
_6.fld0.2.0.1 = _6.fld0.2.1;
_8 = &place!(Field::<u128>(Variant(_11, 2), 0));
_9.2.1 = &_9.3.0;
SetDiscriminant((*_2), 1);
_12 = &_9.0.0;
place!(Field::<i64>(Variant((*_2), 1), 2)) = 8947750886958136024_i64;
place!(Field::<bool>(Variant(RET, 1), 0)) = (*_7) == (*_7);
_9.2.0.2.3 = 39023828395814743084773095098126898573_u128 as i128;
place!(Field::<bool>(Variant(RET, 1), 0)) = _1 & _1;
_7 = core::ptr::addr_of!((*_7));
place!(Field::<char>(Variant((*_2), 1), 1)) = _3;
place!(Field::<bool>(Variant(RET, 1), 0)) = _1;
place!(Field::<bool>(Variant(RET, 1), 0)) = !_1;
_9.2.0.2.3 = -_6.fld0.2.3;
_9.2.0.2.3 = -_6.fld0.2.3;
match _6.fld0.2.3 {
0 => bb6,
150345338409567971330586195374775515443 => bb10,
_ => bb5
}
}
bb10 = {
place!(Field::<u64>(Variant(RET, 1), 3)) = !8664701433186725627_u64;
_9.1 = core::ptr::addr_of_mut!(_17);
_6.fld1.0 = [6_u8,197_u8,74_u8,48_u8];
Goto(bb11)
}
bb11 = {
_12 = &(*_12);
place!(Field::<u64>(Variant(RET, 1), 3)) = !14301023460769548307_u64;
place!(Field::<i128>(Variant((*_2), 1), 4)) = _9.2.0.2.3;
(*_7) = _5;
_14 = [106_u8,176_u8,228_u8,133_u8];
_14 = [227_u8,165_u8,38_u8,156_u8];
match Field::<i64>(Variant(RET, 1), 2) {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
8947750886958136024 => bb20,
_ => bb19
}
}
bb12 = {
place!(Field::<u64>(Variant(RET, 1), 3)) = !8664701433186725627_u64;
_9.1 = core::ptr::addr_of_mut!(_17);
_6.fld1.0 = [6_u8,197_u8,74_u8,48_u8];
Goto(bb11)
}
bb13 = {
_6.fld0.2.2 = !_6.fld0.1;
_9.2.0.2 = (_6.fld0.2.0, _6.fld0.2.0.1, _6.fld0.2.2, 150345338409567971330586195374775515443_i128);
_6.fld0.2 = (_9.2.0.2.0, _9.2.0.2.1, _9.2.0.2.2, _9.2.0.2.3);
_5 = _9.2.0.2.2 as isize;
_6.fld0.2 = _9.2.0.2;
place!(Field::<i32>(Variant(RET, 0), 0)) = (-1713521652_i32) & (-204704053_i32);
_9.2.1 = &_9.0.0;
_9.2.1 = &_9.0.0;
_10 = _5;
_9.3 = (68_i8,);
_9.2.0 = _6.fld0;
place!(Field::<i32>(Variant((*_2), 0), 0)) = _6.fld0.2.0.0 as i32;
place!(Field::<i32>(Variant((*_2), 0), 0)) = (-931587882_i32);
_6.fld0.0 = -_9.2.0.0;
_6.fld2 = [(-27813_i16),(-28027_i16),31555_i16,(-21651_i16),25611_i16];
_1 = true ^ true;
_3 = '\u{b3b15}';
_9.2.1 = &_9.0.0;
_6.fld0.2.0.0 = !_9.2.0.2.0.0;
_6.fld0.2.3 = _6.fld0.0 as i128;
_9.2.0.2.2 = _6.fld0.2.1 as u16;
Goto(bb3)
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
Goto(bb2)
}
bb18 = {
_6.fld0.2.2 = !_6.fld0.1;
_9.2.0.2 = (_6.fld0.2.0, _6.fld0.2.0.1, _6.fld0.2.2, 150345338409567971330586195374775515443_i128);
_6.fld0.2 = (_9.2.0.2.0, _9.2.0.2.1, _9.2.0.2.2, _9.2.0.2.3);
_5 = _9.2.0.2.2 as isize;
_6.fld0.2 = _9.2.0.2;
place!(Field::<i32>(Variant(RET, 0), 0)) = (-1713521652_i32) & (-204704053_i32);
_9.2.1 = &_9.0.0;
_9.2.1 = &_9.0.0;
_10 = _5;
_9.3 = (68_i8,);
_9.2.0 = _6.fld0;
place!(Field::<i32>(Variant((*_2), 0), 0)) = _6.fld0.2.0.0 as i32;
place!(Field::<i32>(Variant((*_2), 0), 0)) = (-931587882_i32);
_6.fld0.0 = -_9.2.0.0;
_6.fld2 = [(-27813_i16),(-28027_i16),31555_i16,(-21651_i16),25611_i16];
_1 = true ^ true;
_3 = '\u{b3b15}';
_9.2.1 = &_9.0.0;
_6.fld0.2.0.0 = !_9.2.0.2.0.0;
_6.fld0.2.3 = _6.fld0.0 as i128;
_9.2.0.2.2 = _6.fld0.2.1 as u16;
Goto(bb3)
}
bb19 = {
Goto(bb2)
}
bb20 = {
_13 = _3;
_9.2.0.2.2 = _6.fld0.2.2;
_9.2.0.2.0 = (_6.fld0.2.0.0, _6.fld0.2.0.1);
_6.fld0.2.3 = -Field::<i128>(Variant((*_2), 1), 4);
place!(Field::<char>(Variant(RET, 1), 1)) = _3;
_9.2.0.2.1 = -_6.fld0.2.0.1;
place!(Field::<char>(Variant(RET, 1), 1)) = _13;
_9.2.0.2.2 = !_9.2.0.1;
_18 = _13;
_9.2.0.1 = _6.fld0.2.2;
_21 = (_14,);
SetDiscriminant((*_2), 1);
_7 = core::ptr::addr_of!((*_7));
place!(Field::<char>(Variant(RET, 1), 1)) = _18;
_3 = _18;
_20 = _4 as f32;
RET = Adt17::Variant1 { fld0: _1,fld1: _13,fld2: (-7150923536119900740_i64),fld3: 15837851347471946318_u64,fld4: _9.2.0.2.3 };
_16 = !Field::<bool>(Variant(RET, 1), 0);
_6.fld0.2.0 = (_9.2.0.2.0.0, _6.fld0.2.1);
_9.2.1 = &(*_12);
_9.2.0.1 = !_6.fld0.1;
_21 = _6.fld1;
_6.fld0.2.0 = (_9.2.0.2.0.0, _6.fld0.2.1);
match (*_12) {
0 => bb17,
1 => bb7,
2 => bb21,
3 => bb22,
68 => bb24,
_ => bb23
}
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
Goto(bb2)
}
bb24 = {
_9.2.0.0 = _20;
place!(Field::<bool>(Variant(RET, 1), 0)) = _16;
RET = Adt17::Variant0 { fld0: (-1757165035_i32) };
_6.fld0.0 = -_9.2.0.0;
_18 = _3;
_9.2.0 = _6.fld0;
RET = Adt17::Variant1 { fld0: _1,fld1: _13,fld2: (-2517887886080887072_i64),fld3: 8763083646992660250_u64,fld4: _6.fld0.2.3 };
(*_2) = Adt17::Variant0 { fld0: (-47886600_i32) };
_6.fld1 = _21;
_9.2.0.2.3 = _6.fld0.2.3;
place!(Field::<i32>(Variant((*_2), 0), 0)) = 1842058866_i32;
Goto(bb25)
}
bb25 = {
Call(_23 = dump_var(12_usize, 21_usize, Move(_21), 14_usize, Move(_14), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_23 = dump_var(12_usize, 18_usize, Move(_18), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *const &'static i8,mut _2: &'static [i16; 4],mut _3: &'static i8,mut _4: *const Adt17,mut _5: Adt33,mut _6: [isize; 4],mut _7: u16,mut _8: [isize; 4],mut _9: isize,mut _10: [u32; 7],mut _11: char) -> *const &'static i8 {
mir! {
type RET = *const &'static i8;
let _12: i64;
let _13: i64;
let _14: isize;
let _15: (usize,);
let _16: &'static &'static u128;
let _17: (Adt33, &'static [i16; 4]);
let _18: isize;
let _19: u16;
let _20: isize;
let _21: (f32, u16, ((u32, f64), f64, u16, i128));
let _22: [i16; 4];
let _23: ((u32, f64), f64, u16, i128);
let _24: usize;
let _25: bool;
let _26: [u16; 5];
let _27: usize;
let _28: bool;
let _29: *const i8;
let _30: f32;
let _31: *const Adt17;
let _32: char;
let _33: i32;
let _34: f32;
let _35: u64;
let _36: bool;
let _37: *const i32;
let _38: ();
let _39: ();
{
RET = Move(_1);
_3 = &_5.fld3;
RET = core::ptr::addr_of!(_3);
RET = core::ptr::addr_of!(_3);
RET = core::ptr::addr_of!((*RET));
_3 = &place!(Field::<i8>(Variant(_5.fld4, 2), 2));
_5.fld4 = Adt17::Variant0 { fld0: (-382033096_i32) };
_1 = Move(RET);
_14 = 29839_i16 as isize;
_12 = _5.fld2;
Goto(bb1)
}
bb1 = {
_6 = [_9,_14,_9,_9];
RET = core::ptr::addr_of!(_3);
(*RET) = &_5.fld3;
_4 = core::ptr::addr_of!(_5.fld4);
place!(Field::<i32>(Variant((*_4), 0), 0)) = 3876253375_u32 as i32;
place!(Field::<i32>(Variant((*_4), 0), 0)) = _5.fld2 as i32;
_7 = !29851_u16;
_3 = &(*_3);
(*RET) = &(*_3);
_5.fld3 = 18_u8 as i8;
_13 = _5.fld2 & _12;
_5.fld4 = Adt17::Variant1 { fld0: _5.fld0,fld1: _11,fld2: _12,fld3: 3582241751429822904_u64,fld4: _5.fld1 };
(*RET) = &_5.fld3;
_5.fld4 = Adt17::Variant0 { fld0: 1787417733_i32 };
_10 = [254958082_u32,1957749942_u32,1241118743_u32,3863391300_u32,1719468848_u32,1624591430_u32,2212519758_u32];
_5.fld4 = Adt17::Variant1 { fld0: _5.fld0,fld1: _11,fld2: _12,fld3: 4763497928448548934_u64,fld4: _5.fld1 };
_14 = 29_u8 as isize;
_5.fld3 = 4039593412_u32 as i8;
_15 = (15319014165775915310_usize,);
place!(Field::<i128>(Variant(_5.fld4, 1), 4)) = _5.fld1 ^ _5.fld1;
(*RET) = &_5.fld3;
place!(Field::<i64>(Variant(_5.fld4, 1), 2)) = _15.0 as i64;
_6 = [_9,_9,_9,_9];
(*RET) = &_5.fld3;
_7 = !39310_u16;
match _15.0 {
0 => bb2,
1 => bb3,
15319014165775915310 => bb5,
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
place!(Field::<i128>(Variant((*_4), 1), 4)) = 7173705051816820198_u64 as i128;
(*RET) = &_17.0.fld3;
_5.fld3 = (-13_i8);
_5.fld3 = (-7_i8);
_9 = !_14;
_12 = !_5.fld2;
_18 = _9 & _9;
_17.0.fld2 = -Field::<i64>(Variant((*_4), 1), 2);
_17.0.fld0 = Field::<bool>(Variant((*_4), 1), 0);
(*RET) = &_5.fld3;
(*RET) = &(*_3);
(*RET) = &_5.fld3;
(*RET) = &_17.0.fld3;
place!(Field::<bool>(Variant((*_4), 1), 0)) = _17.0.fld0 ^ _17.0.fld0;
Goto(bb6)
}
bb6 = {
_21.2.0.0 = !789260273_u32;
place!(Field::<i64>(Variant((*_4), 1), 2)) = (-25118_i16) as i64;
(*RET) = &(*_3);
_21.2.0.1 = _7 as f64;
_8 = [_14,_9,_18,_18];
_21.2.1 = _21.2.0.1 - _21.2.0.1;
_21.2.0.1 = _21.2.1;
(*RET) = &(*_3);
(*_4) = Adt17::Variant0 { fld0: (-1815150593_i32) };
_5.fld2 = _7 as i64;
Call(_21.2.3 = fn14(Move(_1), Move(_4), _18, _17.0.fld0, _17.0.fld2, _5.fld1, _6, _6, _10, _12, _15.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_19 = 94534122106110299782998770603051525024_u128 as u16;
_5.fld3 = 58_i8 ^ (-119_i8);
_17.0.fld3 = _5.fld3;
(*RET) = &_5.fld3;
_15.0 = !925997171925783922_usize;
_17.0.fld3 = _21.2.3 as i8;
_21.2.0.1 = _7 as f64;
_11 = '\u{a0600}';
place!(Field::<i32>(Variant(_5.fld4, 0), 0)) = -1815810905_i32;
_21.0 = 5341529758251060749_u64 as f32;
(*RET) = &(*_3);
_21.1 = _21.2.3 as u16;
_21.0 = _17.0.fld3 as f32;
_23.2 = _17.0.fld2 as u16;
_11 = '\u{fece2}';
_10 = [_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0];
_24 = _15.0;
_21.0 = Field::<i32>(Variant(_5.fld4, 0), 0) as f32;
_22 = [(-29617_i16),(-21638_i16),6853_i16,30584_i16];
(*RET) = &_5.fld3;
_5.fld3 = _21.2.0.0 as i8;
_17.0 = Move(_5);
(*RET) = &_17.0.fld3;
_21.1 = Field::<i32>(Variant(_17.0.fld4, 0), 0) as u16;
_18 = _21.2.0.0 as isize;
_1 = core::ptr::addr_of!((*RET));
_23.0.1 = _21.2.0.0 as f64;
(*RET) = &_17.0.fld3;
Goto(bb8)
}
bb8 = {
SetDiscriminant(_17.0.fld4, 0);
Goto(bb9)
}
bb9 = {
_10 = [_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0,_21.2.0.0];
_5.fld2 = _13;
_3 = &_17.0.fld3;
_23.0.0 = _21.2.0.0;
_4 = core::ptr::addr_of!(_5.fld4);
_17.0.fld4 = Adt17::Variant1 { fld0: _17.0.fld0,fld1: _11,fld2: _13,fld3: 17482806272587860178_u64,fld4: _21.2.3 };
_21.2.2 = !_21.1;
_23.3 = Field::<i128>(Variant(_17.0.fld4, 1), 4);
(*RET) = &_5.fld3;
_26 = [_21.1,_19,_21.1,_23.2,_21.2.2];
place!(Field::<u64>(Variant(_17.0.fld4, 1), 3)) = !15622313682516912089_u64;
(*RET) = &_17.0.fld3;
_21.2.2 = _19;
_23.0.0 = !_21.2.0.0;
_2 = &_22;
_26 = [_23.2,_19,_21.1,_7,_19];
_5.fld4 = Move(_17.0.fld4);
SetDiscriminant((*_4), 0);
(*RET) = &_5.fld3;
_23.2 = !_19;
_6 = [_18,_18,_18,_14];
(*RET) = &(*_3);
(*RET) = &(*_3);
_13 = -_17.0.fld2;
_23.0.0 = _17.0.fld3 as u32;
_21.0 = _17.0.fld3 as f32;
_15 = (_24,);
Goto(bb10)
}
bb10 = {
place!(Field::<i32>(Variant((*_4), 0), 0)) = 332109585_i32;
(*_1) = &(*_3);
_21.2.2 = _7 >> _23.3;
(*_1) = &_5.fld3;
_11 = '\u{e75df}';
_6 = _8;
(*RET) = &(*_3);
SetDiscriminant((*_4), 1);
_2 = &_22;
place!(Field::<bool>(Variant(_5.fld4, 1), 0)) = _21.2.3 != _23.3;
(*_1) = &_17.0.fld3;
_21.2.0.0 = _23.0.0 | _23.0.0;
(*RET) = &_5.fld3;
_17.0.fld1 = _21.2.3;
RET = Move(_1);
_5.fld4 = Adt17::Variant0 { fld0: (-1761483967_i32) };
_23 = (_21.2.0, _21.2.0.1, _21.2.2, _21.2.3);
_13 = _12 - _17.0.fld2;
place!(Field::<i32>(Variant((*_4), 0), 0)) = (-261684894_i32) ^ 1009120679_i32;
_23.2 = !_21.2.2;
_21.0 = 19_u8 as f32;
SetDiscriminant((*_4), 2);
_17.0.fld1 = -_21.2.3;
_5.fld1 = _17.0.fld1 | _21.2.3;
place!(Field::<u64>(Variant((*_4), 2), 1)) = !3599236193473208908_u64;
Goto(bb11)
}
bb11 = {
_1 = core::ptr::addr_of!(_3);
(*_4) = Adt17::Variant2 { fld0: 105_u8,fld1: 1515640440391391685_u64,fld2: _17.0.fld3 };
_9 = _14;
(*_4) = Adt17::Variant1 { fld0: _17.0.fld0,fld1: _11,fld2: _17.0.fld2,fld3: 5649760563266597396_u64,fld4: _5.fld1 };
(*_1) = &_17.0.fld3;
_4 = core::ptr::addr_of!((*_4));
Goto(bb12)
}
bb12 = {
_13 = -Field::<i64>(Variant((*_4), 1), 2);
_17.0.fld4 = Adt17::Variant0 { fld0: (-1260251723_i32) };
_28 = Field::<bool>(Variant((*_4), 1), 0) | _17.0.fld0;
place!(Field::<i128>(Variant((*_4), 1), 4)) = !_5.fld1;
(*_4) = Adt17::Variant0 { fld0: 1143798455_i32 };
_3 = &(*_3);
_17.0.fld1 = -_5.fld1;
_23.0.0 = !_21.2.0.0;
Goto(bb13)
}
bb13 = {
place!(Field::<i32>(Variant(_17.0.fld4, 0), 0)) = (-1753572552_i32);
_23.3 = -_5.fld1;
_15 = (_24,);
_33 = -Field::<i32>(Variant(_17.0.fld4, 0), 0);
_20 = _5.fld2 as isize;
(*_4) = Move(_17.0.fld4);
_30 = _21.0;
_33 = !Field::<i32>(Variant((*_4), 0), 0);
_17.0 = Adt33 { fld0: _28,fld1: _5.fld1,fld2: _5.fld2,fld3: (-75_i8),fld4: Move((*_4)) };
_17.0.fld0 = !_28;
_21.2.0.1 = _23.1;
_17.1 = &(*_2);
_35 = _11 as u64;
_21 = (_30, _23.2, _23);
_34 = _21.0;
_14 = !_9;
_5.fld0 = !_17.0.fld0;
_21.2.0 = _23.0;
_28 = _5.fld1 != _21.2.3;
_17.0.fld1 = _23.3 >> _5.fld1;
_1 = core::ptr::addr_of!((*_1));
_4 = core::ptr::addr_of!((*_4));
_27 = _18 as usize;
_5.fld4 = Adt17::Variant1 { fld0: _28,fld1: _11,fld2: _5.fld2,fld3: _35,fld4: _17.0.fld1 };
_24 = _27 * _27;
_6 = [_20,_9,_9,_9];
_5.fld3 = -_17.0.fld3;
_28 = !Field::<bool>(Variant((*_4), 1), 0);
Call(_21.2.1 = core::intrinsics::transmute((*_2)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15.0 = !_24;
(*_4) = Move(_17.0.fld4);
_25 = _28;
_17.0.fld4 = Move((*_4));
_2 = &(*_2);
_3 = &_5.fld3;
_8 = _6;
_21.1 = _23.2;
_2 = Move(_17.1);
_3 = &(*_3);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(13_usize, 13_usize, Move(_13), 9_usize, Move(_9), 18_usize, Move(_18), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(13_usize, 6_usize, Move(_6), 11_usize, Move(_11), 28_usize, Move(_28), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(13_usize, 12_usize, Move(_12), 8_usize, Move(_8), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *const &'static i8,mut _2: *const Adt17,mut _3: isize,mut _4: bool,mut _5: i64,mut _6: i128,mut _7: [isize; 4],mut _8: [isize; 4],mut _9: [u32; 7],mut _10: i64,mut _11: usize) -> i128 {
mir! {
type RET = i128;
let _12: f32;
let _13: &'static [i16; 4];
let _14: *mut f32;
let _15: u8;
let _16: &'static u128;
let _17: &'static &'static u128;
let _18: bool;
let _19: f64;
let _20: (f32, u16, ((u32, f64), f64, u16, i128));
let _21: (i128, [isize; 4], &'static *const &'static i8);
let _22: (usize,);
let _23: [u32; 7];
let _24: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _25: isize;
let _26: *const (bool, (char, [u16; 5], u8, &'static i8), usize);
let _27: f32;
let _28: f32;
let _29: f32;
let _30: ((u32, f64), f64, u16, i128);
let _31: i64;
let _32: isize;
let _33: char;
let _34: i32;
let _35: char;
let _36: [u16; 5];
let _37: [i8; 5];
let _38: isize;
let _39: isize;
let _40: isize;
let _41: f64;
let _42: i128;
let _43: f32;
let _44: [u8; 1];
let _45: [u8; 1];
let _46: ();
let _47: ();
{
_9 = [787417828_u32,4031028560_u32,150090675_u32,149574599_u32,769064962_u32,1464693651_u32,4186124124_u32];
_10 = !_5;
RET = _6;
_9 = [1713636380_u32,1596986791_u32,1778639170_u32,3636340697_u32,3661297939_u32,3576054554_u32,3262557464_u32];
_6 = RET | RET;
_7 = _8;
_9 = [352257410_u32,734635296_u32,754043909_u32,4030832381_u32,1962119443_u32,3361071961_u32,328088320_u32];
RET = _6;
_11 = 5_usize;
_11 = 7_usize & 4_usize;
RET = -_6;
_15 = 115_i8 as u8;
_3 = (-45_isize);
_7 = _8;
RET = !_6;
_17 = &_16;
Goto(bb1)
}
bb1 = {
_11 = 11347139958886503273_usize - 9824590827484804936_usize;
_6 = -RET;
_3 = 9223372036854775807_isize >> _6;
RET = _11 as i128;
_6 = _5 as i128;
_14 = core::ptr::addr_of_mut!(_12);
Goto(bb2)
}
bb2 = {
_11 = _4 as usize;
(*_14) = 7296631411943695955_u64 as f32;
(*_14) = (-14374_i16) as f32;
(*_14) = 865836164_i32 as f32;
(*_14) = _5 as f32;
_8 = [_3,_3,_3,_3];
RET = (-83_i8) as i128;
_3 = (-90_isize) - 9223372036854775807_isize;
_3 = (-114_isize) * (-9223372036854775808_isize);
_11 = 7_usize;
RET = -_6;
_14 = core::ptr::addr_of_mut!(_12);
_11 = _15 as usize;
_4 = !true;
_12 = 1317595396_i32 as f32;
_9 = [3079185280_u32,3289523938_u32,2068845336_u32,679892381_u32,994284694_u32,1668571457_u32,782046628_u32];
_3 = 103_isize * 10_isize;
RET = -_6;
_4 = true ^ true;
_12 = 3178669810_u32 as f32;
RET = -_6;
(*_14) = _11 as f32;
RET = _6 + _6;
_17 = &(*_17);
_6 = -RET;
_18 = !_4;
_9 = [3935389171_u32,3560516377_u32,4199331594_u32,588690136_u32,1316576464_u32,4273522125_u32,3577808582_u32];
_4 = !_18;
_19 = _15 as f64;
(*_14) = 54_i8 as f32;
_17 = &(*_17);
Goto(bb3)
}
bb3 = {
_17 = &(*_17);
_4 = _18;
_9 = [3170206006_u32,76753860_u32,3948212965_u32,626479511_u32,2845867147_u32,4261444479_u32,2186656061_u32];
_20.2.0.0 = 2357349638_u32 << _11;
_20.2.2 = !39959_u16;
RET = _6;
_11 = !16931031629563450266_usize;
_5 = _10 + _10;
_18 = _4;
Goto(bb4)
}
bb4 = {
_22.0 = _11;
_20.2.1 = _19;
_21.2 = &_1;
_20.2.1 = _19 * _19;
_14 = core::ptr::addr_of_mut!(_12);
_21.2 = &_1;
_20.2.0.0 = !123697251_u32;
_5 = 3846494035813460783_u64 as i64;
_20.0 = -(*_14);
_20.2.0.0 = !4280046974_u32;
_15 = !57_u8;
_5 = -_10;
_23 = [_20.2.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0];
_20.2.3 = 20309_i16 as i128;
_20.2.3 = RET;
Goto(bb5)
}
bb5 = {
_20.2.0.0 = 3412173285_u32;
_14 = core::ptr::addr_of_mut!((*_14));
_24.0.3 = -_6;
_6 = _20.2.2 as i128;
_24.0.0.1 = _19;
_3 = !43_isize;
_24.2.1 = [_20.2.2,_20.2.2,_20.2.2,_20.2.2,_20.2.2];
_1 = core::ptr::addr_of!(_24.2.3);
_24.0.0 = (_20.2.0.0, _20.2.1);
_12 = _20.0;
_17 = &(*_17);
_18 = !_4;
_20.2.0 = _24.0.0;
_20.2.0.1 = -_24.0.0.1;
_28 = (*_14) + (*_14);
RET = _3 as i128;
_12 = _28 - _28;
_20.1 = _20.2.2 & _20.2.2;
_24.0.1 = 2564598307783202688_u64 as f64;
_20.2.1 = -_19;
_18 = _4 | _4;
_24.0.1 = _20.2.0.1;
_24.1 = _3 ^ _3;
_24.2.2 = (-105256929_i32) as u8;
Goto(bb6)
}
bb6 = {
_24.2.0 = '\u{bb8e1}';
_24.2.0 = '\u{b2ebb}';
_21.1 = [_3,_24.1,_24.1,_24.1];
_20.2.3 = -_24.0.3;
_10 = 45_i8 as i64;
_24.0.2 = _20.2.2;
_17 = &_16;
_24.2.2 = !_15;
_18 = !_4;
_24.0.0.1 = _19 - _20.2.1;
_24.0 = _20.2;
_7 = [_24.1,_24.1,_24.1,_3];
_30.0 = (_20.2.0.0, _19);
_21.0 = !_20.2.3;
_24.0.1 = _19;
_18 = !_4;
_30.1 = _20.2.0.1;
Call(_30.3 = fn15(_24.0.0.0, _21.0, _24.0.1, (*_14), RET, _20.2, Move(_14), RET), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_30.1 = _24.0.1;
Call(_30.2 = fn16(_21.1, _24.0.0, _9, _20.2.0.0, Move(_2), _30.1, _9, _30.3, _20.2.0.1, _12, _24.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10 = _5 >> _30.3;
_24.0.1 = -_30.0.1;
_30.2 = _24.0.2;
_21.2 = &_1;
_23 = [_24.0.0.0,_20.2.0.0,_30.0.0,_24.0.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0];
_20.2.0 = _24.0.0;
_9 = [_30.0.0,_30.0.0,_20.2.0.0,_24.0.0.0,_24.0.0.0,_20.2.0.0,_20.2.0.0];
_29 = _12;
Goto(bb9)
}
bb9 = {
_21.1 = [_24.1,_3,_24.1,_3];
_20.2.1 = -_19;
_24.0 = (_20.2.0, _30.1, _20.2.2, _30.3);
_14 = core::ptr::addr_of_mut!(_29);
_20.2 = (_24.0.0, _30.1, _20.1, _24.0.3);
_21.2 = &_1;
Goto(bb10)
}
bb10 = {
_15 = _24.2.2;
_15 = _24.1 as u8;
_24.0.0.0 = _22.0 as u32;
_31 = _10 & _10;
_24.0.0 = (_20.2.0.0, _19);
_4 = _18;
_20 = ((*_14), _30.2, _24.0);
_24.2.1 = [_30.2,_30.2,_20.2.2,_30.2,_20.2.2];
Goto(bb11)
}
bb11 = {
_14 = core::ptr::addr_of_mut!((*_14));
_30 = _24.0;
_30.0 = (_20.2.0.0, _30.1);
_20.2.0.0 = _24.0.0.0 % _30.0.0;
_24.0.0.0 = _30.0.0 << _24.1;
_30.3 = _24.2.0 as i128;
_21.1 = [_24.1,_24.1,_24.1,_3];
_5 = _10 | _31;
Goto(bb12)
}
bb12 = {
_38 = _24.1 | _24.1;
_25 = _38;
_20.1 = _30.2 * _20.2.2;
_12 = -(*_14);
_1 = core::ptr::addr_of!((*_1));
_20 = (_12, _24.0.2, _24.0);
_30.3 = !_24.0.3;
_24.0.3 = _30.3;
_21.2 = &_1;
_24.2.0 = '\u{fcacc}';
_24.0.0.1 = _25 as f64;
_17 = &(*_17);
_41 = _3 as f64;
_30.0.1 = _24.0.0.1 - _20.2.0.1;
_3 = _24.1 ^ _24.1;
_24.0 = (_30.0, _41, _20.1, _20.2.3);
_42 = _24.0.3 - _30.3;
_22.0 = !_11;
_5 = -_31;
_40 = _24.0.0.1 as isize;
_22 = (_11,);
_20.0 = (*_14);
_30.0.0 = _24.0.0.0;
_31 = !_10;
_17 = &(*_17);
_9 = [_20.2.0.0,_20.2.0.0,_30.0.0,_20.2.0.0,_20.2.0.0,_20.2.0.0,_24.0.0.0];
Goto(bb13)
}
bb13 = {
_12 = (-108_i8) as f32;
_30.0.1 = -_24.0.0.1;
_31 = _5 * _5;
_33 = _24.2.0;
_6 = !_42;
_20.2.0.0 = _30.0.0;
_24.0.3 = _6 - _20.2.3;
_30 = _24.0;
RET = _24.0.3;
_27 = (*_14) * (*_14);
_21.0 = (-116_i8) as i128;
_10 = _5;
_20.2.3 = !_24.0.3;
_24.2.1 = [_20.1,_20.2.2,_30.2,_24.0.2,_20.2.2];
_22.0 = _11;
_1 = core::ptr::addr_of!((*_1));
_24.2.1 = [_30.2,_30.2,_24.0.2,_30.2,_20.1];
Goto(bb14)
}
bb14 = {
Call(_46 = dump_var(14_usize, 6_usize, Move(_6), 42_usize, Move(_42), 25_usize, Move(_25), 31_usize, Move(_31)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_46 = dump_var(14_usize, 7_usize, Move(_7), 10_usize, Move(_10), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(14_usize, 40_usize, Move(_40), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u32,mut _2: i128,mut _3: f64,mut _4: f32,mut _5: i128,mut _6: ((u32, f64), f64, u16, i128),mut _7: *mut f32,mut _8: i128) -> i128 {
mir! {
type RET = i128;
let _9: ([u8; 4],);
let _10: bool;
let _11: (f32, u16, ((u32, f64), f64, u16, i128));
let _12: u64;
let _13: bool;
let _14: u32;
let _15: [u16; 4];
let _16: ((f32, u16, ((u32, f64), f64, u16, i128)), *mut Adt50, [i16; 4], &'static &'static i8);
let _17: bool;
let _18: isize;
let _19: *const &'static u128;
let _20: *const isize;
let _21: *const i8;
let _22: char;
let _23: (u32, f64);
let _24: [u8; 4];
let _25: f32;
let _26: ();
let _27: ();
{
RET = _8 | _6.3;
_6.1 = -_6.0.1;
_3 = _6.1 * _6.0.1;
_6.0.1 = 12304748965868767992_u64 as f64;
_6.0 = (_1, _3);
_6.2 = !22246_u16;
_1 = _6.0.0;
_2 = _6.0.1 as i128;
_6.2 = 32525_u16;
_6.0.0 = 1048736823_i32 as u32;
_3 = _6.0.1 + _6.0.1;
_2 = (-9223372036854775808_isize) as i128;
_5 = _4 as i128;
_6.0.0 = !_1;
_6.0 = (_1, _3);
_6.0 = (_1, _6.1);
_2 = -_5;
_2 = (-9223372036854775808_isize) as i128;
_1 = _6.0.0;
_7 = core::ptr::addr_of_mut!(_4);
_2 = _6.3;
_3 = _6.1;
_6.0.0 = _1;
_6.2 = !31994_u16;
Call(_6.3 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.0 = [249_u8,151_u8,117_u8,94_u8];
_2 = !_6.3;
_4 = 32129911059356174022590956687067116695_u128 as f32;
_1 = _6.0.0 - _6.0.0;
_6.3 = !_2;
_6.1 = _6.0.1;
_2 = RET;
_6.0.0 = _1;
_4 = 370781597_i32 as f32;
_4 = _6.0.0 as f32;
_6.3 = _2;
_4 = _6.0.0 as f32;
_1 = !_6.0.0;
_6.0 = (_1, _3);
_10 = !false;
_11.2.0.1 = 263983880198856235326231183875858695226_u128 as f64;
_6.1 = _6.0.1 - _11.2.0.1;
RET = !_2;
_6.0 = (_1, _6.1);
_11.2.0 = _6.0;
_12 = (-1268_i16) as u64;
_6 = (_11.2.0, _3, 50592_u16, RET);
_6.0.1 = _11.2.0.1 - _6.1;
_13 = _10 | _10;
match _6.2 {
50592 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = !_6.3;
_11 = ((*_7), _6.2, _6);
(*_7) = -_11.0;
_15 = [_6.2,_11.2.2,_6.2,_11.1];
_6.3 = _5 >> _6.2;
_14 = '\u{83600}' as u32;
_5 = _6.3 >> _11.1;
_11.2.2 = _6.1 as u16;
_4 = _11.0;
_7 = core::ptr::addr_of_mut!((*_7));
_16.0 = _11;
_16.0.2.0 = (_1, _6.0.1);
_16.0.2.0.0 = _1 | _14;
_6.3 = 4_usize as i128;
_13 = _6.3 >= _5;
_20 = core::ptr::addr_of!(_18);
_2 = _16.0.2.3;
Goto(bb4)
}
bb4 = {
_2 = _5 << _6.0.0;
Goto(bb5)
}
bb5 = {
_11.2 = _6;
_11.2.0.1 = _4 as f64;
_10 = _13;
_20 = core::ptr::addr_of!((*_20));
_14 = _16.0.2.2 as u32;
_12 = !17165709595960302538_u64;
_16.0.2.1 = _6.1 - _3;
_13 = _6.2 < _16.0.1;
_11.2.1 = 131150522533716110017788760311103026358_u128 as f64;
_15 = [_11.2.2,_16.0.1,_11.2.2,_16.0.1];
_16.0.1 = !_11.1;
_3 = _6.1;
_23 = (_16.0.2.0.0, _6.1);
_16.2 = [28122_i16,28324_i16,26448_i16,(-3174_i16)];
_16.0.1 = 586624424678612508_i64 as u16;
RET = !_2;
_16.0.2.0.1 = 124_u8 as f64;
(*_20) = 9223372036854775807_isize;
_16.0.2.3 = !RET;
_13 = !_10;
_6.1 = _16.0.2.1;
_9.0 = [24_u8,194_u8,97_u8,50_u8];
Goto(bb6)
}
bb6 = {
Call(_26 = dump_var(15_usize, 15_usize, Move(_15), 5_usize, Move(_5), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_26 = dump_var(15_usize, 12_usize, Move(_12), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [isize; 4],mut _2: (u32, f64),mut _3: [u32; 7],mut _4: u32,mut _5: *const Adt17,mut _6: f64,mut _7: [u32; 7],mut _8: i128,mut _9: f64,mut _10: f32,mut _11: ((u32, f64), f64, u16, i128)) -> u16 {
mir! {
type RET = u16;
let _12: (i8,);
let _13: usize;
let _14: [isize; 7];
let _15: u64;
let _16: (&'static u128,);
let _17: &'static &'static i8;
let _18: Adt50;
let _19: (&'static u128,);
let _20: isize;
let _21: &'static Adt50;
let _22: &'static isize;
let _23: isize;
let _24: u64;
let _25: Adt36;
let _26: &'static f64;
let _27: &'static [u16; 5];
let _28: &'static &'static isize;
let _29: Adt85;
let _30: f32;
let _31: ([u8; 4],);
let _32: isize;
let _33: ();
let _34: ();
{
RET = _11.2;
RET = _11.2 + _11.2;
_11 = (_2, _2.1, RET, _8);
_11.3 = !_8;
_11.1 = _11.0.1;
_11.0.1 = _6 + _9;
_11.0.1 = _9;
_9 = -_2.1;
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,41_isize];
_7 = [_11.0.0,_11.0.0,_2.0,_11.0.0,_11.0.0,_11.0.0,_2.0];
_8 = _11.3 - _11.3;
_3 = [_4,_4,_4,_4,_11.0.0,_11.0.0,_2.0];
RET = _11.2 - _11.2;
_2 = (_11.0.0, _11.1);
_11.0.0 = _4 / _4;
_3 = [_11.0.0,_11.0.0,_4,_11.0.0,_2.0,_11.0.0,_2.0];
_11.1 = 8902316841010606325_i64 as f64;
_12 = ((-74_i8),);
_2.1 = _9;
_11.0.0 = _2.0 & _2.0;
_11 = (_2, _6, RET, _8);
_7 = [_2.0,_11.0.0,_2.0,_11.0.0,_11.0.0,_4,_4];
_4 = _11.0.0;
_12 = (82_i8,);
_11.3 = _8 | _8;
_2 = _11.0;
_9 = _11.1;
_10 = 4227215116297329319_i64 as f32;
Goto(bb1)
}
bb1 = {
_14 = [126_isize,9223372036854775807_isize,9223372036854775807_isize,(-117_isize),35_isize,115_isize,64_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-103_isize)];
_11.0.1 = _6;
_11.2 = RET << _11.3;
_8 = _11.3 - _11.3;
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
3412173285 => bb7,
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
_15 = 13853948508130087243_u64;
_13 = 4_usize;
_2.0 = !_11.0.0;
_14[_13] = (-9223372036854775808_isize) * (-98_isize);
_7[_13] = _3[_13];
_11 = (_2, _6, RET, _8);
_8 = _11.3 >> _11.2;
_7[_13] = !_3[_13];
_6 = 185634883449915253155247449426210395118_u128 as f64;
_12.0 = RET as i8;
_4 = _13 as u32;
_13 = 4_usize & 3900337093962169422_usize;
_13 = 394315840_i32 as usize;
_13 = _11.2 as usize;
_11.1 = -_6;
_14 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-65_isize),9223372036854775807_isize,53_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_11.0 = _2;
RET = _11.2 ^ _11.2;
_13 = 14377969798755036545_usize * 0_usize;
_18.fld0.2.1 = _2.1 - _2.1;
_11.0.1 = -_11.1;
_18.fld0.2.0.0 = !_11.0.0;
Call(_18.fld2 = fn17(_11, _8, _11.3, _18.fld0.2.1, _11.3, _8, _11, _1, _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = 1_usize;
_21 = &_18;
Call(_7[_13] = core::intrinsics::transmute(_18.fld0.2.0.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18.fld0.0 = _10;
_7 = [_4,_4,(*_21).fld0.2.0.0,(*_21).fld0.2.0.0,(*_21).fld0.2.0.0,_4,_18.fld0.2.0.0];
_10 = _15 as f32;
_7 = [_3[_13],(*_21).fld0.2.0.0,(*_21).fld0.2.0.0,_11.0.0,_3[_13],_11.0.0,_3[_13]];
_11.1 = _14[_13] as f64;
_21 = &_18;
_18.fld0.2.3 = _11.3;
_3[_13] = (-1814582170_i32) as u32;
_11.0 = _2;
RET = _11.2 * _11.2;
_18.fld0.2.0.0 = _4 * _4;
_18.fld2[_13] = -(-23515_i16);
match _14[_13] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb10 = {
_14 = [126_isize,9223372036854775807_isize,9223372036854775807_isize,(-117_isize),35_isize,115_isize,64_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-103_isize)];
_11.0.1 = _6;
_11.2 = RET << _11.3;
_8 = _11.3 - _11.3;
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
3412173285 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_14[_13] = _1[_13];
_22 = &_20;
_15 = 10171808716993623760_u64;
_11.0.1 = _18.fld0.2.1 - _11.1;
_12 = ((-2_i8),);
_20 = true as isize;
_18.fld0.2.3 = _8 & _8;
_2.1 = _11.1;
_25 = Adt36::Variant0 { fld0: false };
_22 = &_23;
_3 = [_2.0,_11.0.0,_7[_13],_7[_13],_18.fld0.2.0.0,_11.0.0,_18.fld0.2.0.0];
_18.fld0.2.0.0 = _18.fld2[_13] as u32;
_18.fld2[_13] = 209762554102604065_i64 as i16;
_12 = ((-48_i8),);
_7 = [_18.fld0.2.0.0,_18.fld0.2.0.0,_11.0.0,_11.0.0,_2.0,_4,_11.0.0];
_6 = -_11.0.1;
_12 = (19_i8,);
_14[_13] = _10 as isize;
_18.fld0.2 = _11;
_18.fld0.2.1 = _11.1;
_18.fld0.2.0.1 = -_11.0.1;
_21 = &_18;
_11.3 = (*_21).fld0.2.3 & (*_21).fld0.2.3;
Goto(bb14)
}
bb14 = {
_24 = _15 & _15;
_1 = [_20,_14[_13],_20,_20];
_3[_13] = _2.0;
_18.fld0.2.0 = (_3[_13], _6);
_15 = _18.fld0.2.1 as u64;
_18.fld0.2.2 = 223_u8 as u16;
_11 = (_2, (*_21).fld0.2.1, RET, _8);
_15 = !_24;
_24 = _14[_13] as u64;
_2.0 = _3[_13] * _7[_13];
_26 = &_2.1;
_9 = -_11.1;
_18.fld0.2.0.1 = -_6;
_1 = [_20,_20,_20,_14[_13]];
_23 = _10 as isize;
_11 = (_18.fld0.2.0, (*_26), RET, (*_21).fld0.2.3);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(16_usize, 15_usize, Move(_15), 12_usize, Move(_12), 14_usize, Move(_14), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(16_usize, 23_usize, Move(_23), 8_usize, Move(_8), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: ((u32, f64), f64, u16, i128),mut _2: i128,mut _3: i128,mut _4: f64,mut _5: i128,mut _6: i128,mut _7: ((u32, f64), f64, u16, i128),mut _8: [isize; 4],mut _9: ((u32, f64), f64, u16, i128)) -> [i16; 5] {
mir! {
type RET = [i16; 5];
let _10: *mut f32;
let _11: char;
let _12: isize;
let _13: *const Adt17;
let _14: *const &'static i8;
let _15: &'static Adt50;
let _16: [isize; 7];
let _17: bool;
let _18: f32;
let _19: [u16; 5];
let _20: [isize; 7];
let _21: &'static [i16; 4];
let _22: Adt73;
let _23: Adt33;
let _24: (*mut bool, (*const &'static i8, Adt17, u128), [i16; 5], i64);
let _25: f64;
let _26: isize;
let _27: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _28: ((f32, u16, ((u32, f64), f64, u16, i128)), *mut Adt50, [i16; 4], &'static &'static i8);
let _29: ([u8; 4],);
let _30: *const Adt17;
let _31: f32;
let _32: &'static Adt50;
let _33: &'static i8;
let _34: &'static u32;
let _35: f64;
let _36: ();
let _37: ();
{
_9.0 = (_1.0.0, _4);
_1.0 = _9.0;
_9 = (_1.0, _1.0.1, _7.2, _6);
_7.0.1 = -_9.1;
RET = [(-30038_i16),(-17748_i16),(-1174_i16),(-6912_i16),(-9569_i16)];
_1 = _9;
_9.3 = !_5;
_7 = (_9.0, _9.0.1, _9.2, _5);
_7.0.0 = _9.0.0 + _9.0.0;
_8 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-100_isize)];
_9.2 = _7.2 - _7.2;
_9.0 = _7.0;
_11 = '\u{4616b}';
_8 = [(-9223372036854775808_isize),(-16_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9.0.0 = !_1.0.0;
_7.3 = -_9.3;
_2 = -_5;
_7 = (_9.0, _9.1, _9.2, _1.3);
_6 = _5;
Goto(bb1)
}
bb1 = {
_7.2 = !_9.2;
_7.0.0 = _9.0.0 << _7.2;
_5 = _3 + _2;
_1 = (_7.0, _7.1, _9.2, _5);
_9.3 = !_5;
_1.0.1 = _7.1 + _9.1;
RET = [(-12557_i16),(-28455_i16),(-7519_i16),28880_i16,29562_i16];
_4 = 191_u8 as f64;
_7.0.1 = _1.0.1;
_1.0 = (_7.0.0, _7.0.1);
_9.0.0 = !_1.0.0;
_1 = (_9.0, _7.0.1, _9.2, _7.3);
_5 = _1.3;
_7.0 = _1.0;
_7.2 = (-7603506029320466195_i64) as u16;
_11 = '\u{10f7c1}';
_9 = _7;
RET = [31220_i16,(-20692_i16),24264_i16,(-13000_i16),1968_i16];
_1 = (_7.0, _7.1, _9.2, _9.3);
_8 = [(-91_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = [(-9223372036854775808_isize),(-116_isize),(-9223372036854775808_isize),(-48_isize)];
Goto(bb2)
}
bb2 = {
_11 = '\u{7636a}';
Goto(bb3)
}
bb3 = {
_3 = _2 | _7.3;
_7 = (_1.0, _1.1, _9.2, _5);
_6 = _2 * _3;
_8 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_9.3 = _3 & _1.3;
_9 = (_1.0, _1.0.1, _7.2, _2);
_7.3 = 3581818127737002981_usize as i128;
_6 = _1.3;
_1.1 = -_9.0.1;
RET = [22389_i16,(-25534_i16),9205_i16,(-12660_i16),5161_i16];
_7.0.0 = _1.0.0 ^ _9.0.0;
_9.0 = _7.0;
_1.0.1 = -_9.1;
Goto(bb4)
}
bb4 = {
_1 = (_7.0, _7.0.1, _7.2, _6);
_1.0.1 = 68_i8 as f64;
_1.1 = (-187931057_i32) as f64;
_7.1 = _4 * _9.0.1;
_1.0.1 = _7.0.1;
RET = [27679_i16,6908_i16,(-9306_i16),(-30571_i16),(-26053_i16)];
_1 = (_7.0, _9.1, _9.2, _3);
Goto(bb5)
}
bb5 = {
_9.3 = -_5;
RET = [17717_i16,15854_i16,(-12492_i16),18512_i16,13306_i16];
_1.3 = _9.3 + _5;
_1.3 = !_3;
_1 = (_7.0, _7.1, _9.2, _2);
_17 = _2 >= _3;
_1.0.1 = _9.0.1;
_16 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,95_isize,9223372036854775807_isize];
_4 = _1.1;
RET = [30850_i16,21732_i16,(-7832_i16),9114_i16,(-25582_i16)];
_17 = !false;
RET = [20111_i16,32174_i16,31770_i16,7576_i16,2948_i16];
_9.0 = (_7.0.0, _4);
_18 = 1_usize as f32;
_9.0.1 = 3555087636658206663_u64 as f64;
_1.0.1 = _7.0.1;
RET = [5688_i16,8641_i16,1499_i16,(-16511_i16),(-30587_i16)];
Goto(bb6)
}
bb6 = {
_9.3 = _2;
_12 = (-9223372036854775808_isize) + 9223372036854775807_isize;
RET = [(-2850_i16),(-28467_i16),6890_i16,(-26666_i16),3493_i16];
_7 = _1;
_16 = [_12,_12,_12,_12,_12,_12,_12];
_7.0 = _1.0;
_7.1 = _9.0.1;
_7.2 = !_1.2;
_9.0.1 = -_7.1;
_1.0 = _9.0;
_1.3 = _9.3 + _6;
_7.0.0 = !_9.0.0;
_9.0 = _1.0;
_7.0 = _9.0;
_22.fld3 = _1.3 & _3;
_23.fld4 = Adt17::Variant2 { fld0: 170_u8,fld1: 9404142308524469534_u64,fld2: 62_i8 };
_23.fld2 = _3 as i64;
place!(Field::<u8>(Variant(_23.fld4, 2), 0)) = !4_u8;
_22.fld6 = (_9.0, _4, _7.2, _5);
_10 = core::ptr::addr_of_mut!(_18);
_9.0.0 = _7.0.0 * _1.0.0;
_5 = _9.0.0 as i128;
place!(Field::<u64>(Variant(_23.fld4, 2), 1)) = 10986042153081674660_u64;
_4 = _7.0.1;
_22.fld6.3 = _9.3 >> _3;
_6 = _22.fld6.3 - _22.fld6.3;
Goto(bb7)
}
bb7 = {
_22.fld5 = Field::<u8>(Variant(_23.fld4, 2), 0);
_11 = '\u{f2566}';
place!(Field::<u8>(Variant(_23.fld4, 2), 0)) = !_22.fld5;
match Field::<u64>(Variant(_23.fld4, 2), 1) {
0 => bb6,
1 => bb2,
2 => bb5,
10986042153081674660 => bb9,
_ => bb8
}
}
bb8 = {
_3 = _2 | _7.3;
_7 = (_1.0, _1.1, _9.2, _5);
_6 = _2 * _3;
_8 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_9.3 = _3 & _1.3;
_9 = (_1.0, _1.0.1, _7.2, _2);
_7.3 = 3581818127737002981_usize as i128;
_6 = _1.3;
_1.1 = -_9.0.1;
RET = [22389_i16,(-25534_i16),9205_i16,(-12660_i16),5161_i16];
_7.0.0 = _1.0.0 ^ _9.0.0;
_9.0 = _7.0;
_1.0.1 = -_9.1;
Goto(bb4)
}
bb9 = {
_27.2.1 = [_22.fld6.2,_22.fld6.2,_1.2,_1.2,_1.2];
_27.2.0 = _11;
(*_10) = _7.0.1 as f32;
(*_10) = 924426153481862151_usize as f32;
_27.0.0.0 = _9.0.0;
_24.3 = _23.fld2;
_22.fld0 = [_9.0.0,_9.0.0];
Goto(bb10)
}
bb10 = {
_1.3 = -_3;
_24.3 = -_23.fld2;
_14 = core::ptr::addr_of!(_27.2.3);
_28.0 = ((*_10), _1.2, _7);
_29.0 = [_22.fld5,_22.fld5,Field::<u8>(Variant(_23.fld4, 2), 0),Field::<u8>(Variant(_23.fld4, 2), 0)];
_24.1.2 = 312626328850400241418127935512219043266_u128;
_27.2.1 = [_28.0.2.2,_9.2,_22.fld6.2,_7.2,_28.0.2.2];
_24.1.0 = core::ptr::addr_of!((*_14));
_9.1 = -_22.fld6.1;
_17 = true ^ false;
_1 = _28.0.2;
_9.1 = _24.1.2 as f64;
_28.0.1 = _9.2 << _2;
_12 = 9223372036854775807_isize;
_27.1 = _12;
_3 = !_7.3;
_24.1.1 = Adt17::Variant0 { fld0: 2010689709_i32 };
(*_14) = &place!(Field::<i8>(Variant(_23.fld4, 2), 2));
_1 = (_7.0, _22.fld6.1, _28.0.1, _22.fld3);
_24.1.1 = Adt17::Variant0 { fld0: 512994360_i32 };
(*_10) = _1.2 as f32;
(*_10) = _22.fld3 as f32;
_9.0.1 = -_9.1;
_7.3 = _5;
match Field::<u64>(Variant(_23.fld4, 2), 1) {
0 => bb4,
1 => bb11,
10986042153081674660 => bb13,
_ => bb12
}
}
bb11 = {
_27.2.1 = [_22.fld6.2,_22.fld6.2,_1.2,_1.2,_1.2];
_27.2.0 = _11;
(*_10) = _7.0.1 as f32;
(*_10) = 924426153481862151_usize as f32;
_27.0.0.0 = _9.0.0;
_24.3 = _23.fld2;
_22.fld0 = [_9.0.0,_9.0.0];
Goto(bb10)
}
bb12 = {
_11 = '\u{7636a}';
Goto(bb3)
}
bb13 = {
_28.0.2.0.1 = _9.1 * _9.1;
_21 = &_28.2;
_22.fld6.0.1 = -_1.1;
_24.2 = RET;
_23.fld1 = !_2;
_27.0.0.0 = _22.fld6.0.0;
_26 = _2 as isize;
_7.3 = _6 << _23.fld2;
_24.1.1 = Adt17::Variant0 { fld0: (-209193828_i32) };
_1.3 = _7.3 * _9.3;
_9.3 = Field::<u8>(Variant(_23.fld4, 2), 0) as i128;
_27.0.0 = (_28.0.2.0.0, _22.fld6.0.1);
_23.fld2 = !_24.3;
_24.1.2 = !203262809330487322336929276989738193260_u128;
_28.0.2.3 = _9.1 as i128;
_1.0.0 = _24.3 as u32;
_27.2.1 = [_1.2,_28.0.1,_1.2,_28.0.1,_28.0.1];
_25 = -_1.1;
_28.3 = &_33;
_24.0 = core::ptr::addr_of_mut!(_23.fld0);
_1.0.0 = _27.0.0.0;
_19 = [_1.2,_28.0.1,_28.0.1,_1.2,_28.0.1];
Goto(bb14)
}
bb14 = {
_1.0.0 = _9.0.0 + _28.0.2.0.0;
_11 = _27.2.0;
_1.0.0 = _28.0.2.0.0;
_20 = _16;
_9.1 = _24.1.2 as f64;
_21 = &_28.2;
_9.0 = (_28.0.2.0.0, _25);
_7.3 = _6;
_34 = &_27.0.0.0;
place!(Field::<i8>(Variant(_23.fld4, 2), 2)) = !47_i8;
_24.1 = (Move(_14), Move(_23.fld4), 128172040741834196847218080366409824710_u128);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(17_usize, 19_usize, Move(_19), 29_usize, Move(_29), 6_usize, Move(_6), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(17_usize, 26_usize, Move(_26), 8_usize, Move(_8), 37_usize, _37, 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> u64 {
mir! {
type RET = u64;
let _1: u8;
let _2: i16;
let _3: *const Adt36;
let _4: u64;
let _5: [isize; 7];
let _6: isize;
let _7: i32;
let _8: [i16; 2];
let _9: i8;
let _10: i16;
let _11: ((f32, u16, ((u32, f64), f64, u16, i128)), *mut Adt50, [i16; 4], &'static &'static i8);
let _12: &'static (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _13: f32;
let _14: *const (bool, (char, [u16; 5], u8, &'static i8), usize);
let _15: Adt64;
let _16: isize;
let _17: (char, [u16; 5], u8, &'static i8);
let _18: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _19: ();
let _20: ();
{
_1 = 175_u8;
RET = 16884204697009616135_u64 * 5448511529341082043_u64;
_1 = 41_u8 - 210_u8;
RET = 15181190339485726232_u64;
_2 = RET as i16;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
15181190339485726232 => bb8,
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
RET = 13778544741000077090_u64;
RET = 5466824992799938313_u64 * 3983638408751614825_u64;
_2 = 27201_i16 + (-27302_i16);
_1 = 147_u8;
_5 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,106_isize,27_isize];
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = true as u8;
Goto(bb9)
}
bb9 = {
_1 = (-9223372036854775808_isize) as u8;
_2 = 4596_i16;
_2 = (-15169_i16);
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),55_isize,(-4_isize),(-62_isize),6_isize,9223372036854775807_isize];
RET = 10454496250713012274_u64;
_4 = RET - RET;
_5 = [(-49_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),6_isize,9223372036854775807_isize];
match _2 {
340282366920938463463374607431768196287 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_2 = RET as i16;
RET = !_4;
_5 = [(-69_isize),(-9223372036854775808_isize),9223372036854775807_isize,7_isize,9223372036854775807_isize,23_isize,(-37_isize)];
_4 = RET & RET;
_2 = (-24992_i16) | 31199_i16;
_2 = !(-28967_i16);
_4 = RET;
_5 = [(-9223372036854775808_isize),9_isize,(-90_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-21_isize)];
_2 = 4240_i16 << RET;
RET = 2944_u16 as u64;
_2 = !(-13367_i16);
Goto(bb12)
}
bb12 = {
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-109_isize),(-9223372036854775808_isize),(-19_isize),(-9223372036854775808_isize)];
_1 = _4 as u8;
_4 = !RET;
RET = 1700764648_i32 as u64;
_1 = !176_u8;
_1 = '\u{2ac84}' as u8;
_2 = _1 as i16;
_1 = (-997627541_i32) as u8;
_6 = 9223372036854775807_isize;
_6 = !(-126_isize);
_1 = !183_u8;
RET = _4;
RET = !_4;
_1 = 131_u8;
_5 = [_6,_6,_6,_6,_6,_6,_6];
RET = _4 * _4;
_1 = 201_u8 | 98_u8;
_7 = !1140548807_i32;
_5 = [_6,_6,_6,_6,_6,_6,_6];
_1 = 172_u8 * 238_u8;
RET = _4 >> _7;
_2 = 21797_i16 + 20540_i16;
RET = 329886276228816814780674139518326845949_u128 as u64;
_1 = 116_u8;
_2 = -(-11422_i16);
Goto(bb13)
}
bb13 = {
_11.2 = [_2,_2,_2,_2];
_8 = [_2,_2];
RET = _4 + _4;
_7 = 51010_u16 as i32;
match _1 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
116 => bb20,
_ => bb19
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
_1 = (-9223372036854775808_isize) as u8;
_2 = 4596_i16;
_2 = (-15169_i16);
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),55_isize,(-4_isize),(-62_isize),6_isize,9223372036854775807_isize];
RET = 10454496250713012274_u64;
_4 = RET - RET;
_5 = [(-49_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),6_isize,9223372036854775807_isize];
match _2 {
340282366920938463463374607431768196287 => bb11,
_ => bb10
}
}
bb18 = {
RET = 13778544741000077090_u64;
RET = 5466824992799938313_u64 * 3983638408751614825_u64;
_2 = 27201_i16 + (-27302_i16);
_1 = 147_u8;
_5 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,106_isize,27_isize];
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = true as u8;
Goto(bb9)
}
bb19 = {
Return()
}
bb20 = {
_9 = '\u{ca989}' as i8;
_4 = RET;
_11.0.0 = 2560882187203713497_i64 as f32;
_11.0.0 = 130214378_u32 as f32;
_11.2 = [_2,_2,_2,_2];
_6 = false as isize;
_11.0.2.2 = !52677_u16;
_11.0.0 = 0_usize as f32;
_10 = _2;
_11.0.2.1 = _11.0.2.2 as f64;
_11.0.2.0.1 = _11.0.2.1;
_11.0.2.0 = (3698916474_u32, _11.0.2.1);
_11.0.2.1 = _1 as f64;
_18.2.1 = [_11.0.2.2,_11.0.2.2,_11.0.2.2,_11.0.2.2,_11.0.2.2];
_11.3 = &_17.3;
_18.0 = (_11.0.2.0, _11.0.2.0.1, _11.0.2.2, 59805921627243646549201492301054574462_i128);
_9 = (-89_i8);
_17.3 = &_9;
_18.2.2 = false as u8;
Goto(bb21)
}
bb21 = {
Call(_19 = dump_var(18_usize, 5_usize, Move(_5), 1_usize, Move(_1), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: (*const &'static i8, Adt17, u128),mut _2: isize,mut _3: [u16; 5]) -> isize {
mir! {
type RET = isize;
let _4: bool;
let _5: *const &'static u128;
let _6: usize;
let _7: Adt36;
let _8: *const isize;
let _9: usize;
let _10: u16;
let _11: i64;
let _12: [i16; 2];
let _13: (*const &'static i8, Adt17, u128);
let _14: [u16; 4];
let _15: [isize; 4];
let _16: (((u32, f64), f64, u16, i128), isize, (char, [u16; 5], u8, &'static i8));
let _17: (f32, u16, ((u32, f64), f64, u16, i128));
let _18: [u32; 7];
let _19: (char, [u16; 5], u8, &'static i8);
let _20: [u8; 4];
let _21: i64;
let _22: i128;
let _23: [u32; 2];
let _24: i128;
let _25: &'static isize;
let _26: f32;
let _27: char;
let _28: ();
let _29: ();
{
_3 = [65204_u16,39532_u16,39314_u16,62910_u16,13812_u16];
place!(Field::<i128>(Variant(_1.1, 1), 4)) = _1.2 as i128;
_2 = (-27_isize) - (-9223372036854775808_isize);
_4 = Field::<u64>(Variant(_1.1, 1), 3) >= Field::<u64>(Variant(_1.1, 1), 3);
place!(Field::<char>(Variant(_1.1, 1), 1)) = '\u{bacc2}';
RET = _2;
_1.1 = Adt17::Variant0 { fld0: 695469258_i32 };
_6 = !12152225508268364044_usize;
_7 = Adt36::Variant2 { fld0: _4 };
RET = -_2;
place!(Field::<i32>(Variant(_1.1, 0), 0)) = 1844233826_i32;
_7 = Adt36::Variant0 { fld0: _4 };
_3 = [26636_u16,5240_u16,9505_u16,50876_u16,20022_u16];
RET = -_2;
_1.1 = Adt17::Variant1 { fld0: Field::<bool>(Variant(_7, 0), 0),fld1: '\u{d186a}',fld2: 2683495344917482391_i64,fld3: 13009205639357388233_u64,fld4: (-52713287509346551223546962923591266122_i128) };
place!(Field::<i128>(Variant(_1.1, 1), 4)) = 91658885347603833429095473754533568440_i128;
RET = _2;
RET = (-7706_i16) as isize;
_2 = RET;
match Field::<i128>(Variant(_1.1, 1), 4) {
0 => bb1,
91658885347603833429095473754533568440 => bb3,
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
_9 = _1.2 as usize;
SetDiscriminant(_7, 3);
_13.2 = !_1.2;
place!(Field::<f32>(Variant(_7, 3), 0)) = 2301376565968432910_u64 as f32;
_8 = core::ptr::addr_of!(_2);
_15 = [(*_8),(*_8),RET,_2];
RET = !_2;
place!(Field::<i64>(Variant(_7, 3), 1)) = -1167066216109531230_i64;
Goto(bb4)
}
bb4 = {
_1.1 = Adt17::Variant0 { fld0: (-578782978_i32) };
_1.1 = Adt17::Variant1 { fld0: _4,fld1: '\u{572bc}',fld2: Field::<i64>(Variant(_7, 3), 1),fld3: 4855267511303281812_u64,fld4: 144038829019916051855468745260742641299_i128 };
_4 = Field::<bool>(Variant(_1.1, 1), 0);
place!(Field::<u64>(Variant(_1.1, 1), 3)) = !13777287641480757764_u64;
_16.0.0.1 = Field::<i64>(Variant(_7, 3), 1) as f64;
Goto(bb5)
}
bb5 = {
_8 = core::ptr::addr_of!(_2);
_10 = Field::<u64>(Variant(_1.1, 1), 3) as u16;
Goto(bb6)
}
bb6 = {
_16.1 = '\u{d6fa2}' as isize;
_14 = [_10,_10,_10,_10];
_16.0.3 = 52012314776607659580869070047085003578_i128;
_6 = _10 as usize;
_3 = [_10,_10,_10,_10,_10];
_11 = _16.0.3 as i64;
Goto(bb7)
}
bb7 = {
_16.2.0 = '\u{97c0b}';
_10 = 17161_u16;
Goto(bb8)
}
bb8 = {
_19.1 = [_10,_10,_10,_10,_10];
_17.2.0.0 = 3617540817_u32;
match _10 {
0 => bb9,
1 => bb10,
17161 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
_16.1 = '\u{d6fa2}' as isize;
_14 = [_10,_10,_10,_10];
_16.0.3 = 52012314776607659580869070047085003578_i128;
_6 = _10 as usize;
_3 = [_10,_10,_10,_10,_10];
_11 = _16.0.3 as i64;
Goto(bb7)
}
bb11 = {
_8 = core::ptr::addr_of!(_2);
_10 = Field::<u64>(Variant(_1.1, 1), 3) as u16;
Goto(bb6)
}
bb12 = {
_1.0 = core::ptr::addr_of!(_16.2.3);
place!(Field::<[u16; 5]>(Variant(_7, 3), 2)) = _3;
place!(Field::<i64>(Variant(_7, 3), 1)) = _11;
_17.2.0.1 = -_16.0.0.1;
_6 = _9 & _9;
_16.0.0.0 = !_17.2.0.0;
Goto(bb13)
}
bb13 = {
_15 = [RET,(*_8),RET,RET];
_14 = [_10,_10,_10,_10];
_16.0.1 = Field::<u64>(Variant(_1.1, 1), 3) as f64;
_16.0.1 = Field::<i64>(Variant(_7, 3), 1) as f64;
_21 = Field::<i64>(Variant(_1.1, 1), 2);
_19.0 = _16.2.0;
_17.2.1 = _16.0.1;
_16.0.0 = (_17.2.0.0, _17.2.1);
_19.2 = 57_u8 & 243_u8;
_17.2.3 = _16.0.3 ^ _16.0.3;
_17.2.3 = _16.0.3 << _16.1;
_17.2.0 = (_16.0.0.0, _16.0.0.1);
_17.2 = (_16.0.0, _16.0.0.1, _10, _16.0.3);
_16.1 = 101_i8 as isize;
place!(Field::<u32>(Variant(_7, 3), 3)) = _17.2.0.0;
_19.0 = _16.2.0;
_17.0 = Field::<f32>(Variant(_7, 3), 0);
_16.2.2 = !_19.2;
_22 = _17.2.3;
_20 = [_19.2,_19.2,_19.2,_16.2.2];
SetDiscriminant(_7, 1);
_17.1 = _17.2.2 % _10;
place!(Field::<isize>(Variant(_7, 1), 2)) = (*_8);
_23 = [_17.2.0.0,_17.2.0.0];
place!(Field::<char>(Variant(_1.1, 1), 1)) = _16.2.0;
_21 = _17.0 as i64;
Goto(bb14)
}
bb14 = {
_1.1 = Adt17::Variant2 { fld0: _16.2.2,fld1: 11515252848586229191_u64,fld2: 108_i8 };
_17.2.1 = _17.2.0.1;
_16.0 = _17.2;
_12 = [(-30746_i16),(-5572_i16)];
place!(Field::<[u16; 4]>(Variant(_7, 1), 0)) = [_17.2.2,_10,_17.1,_16.0.2];
_17.0 = _17.1 as f32;
place!(Field::<*const Adt17>(Variant(_7, 1), 3)) = core::ptr::addr_of!(_1.1);
_16.0 = (_17.2.0, _17.2.1, _17.1, _22);
_16.0.0 = (_17.2.0.0, _17.2.1);
_7 = Adt36::Variant2 { fld0: _4 };
_1.0 = core::ptr::addr_of!(_16.2.3);
_21 = _11 - _11;
_26 = -_17.0;
SetDiscriminant(_7, 3);
_19.0 = _16.2.0;
place!(Field::<u8>(Variant(_1.1, 2), 0)) = _19.2 & _16.2.2;
_22 = 724654539_i32 as i128;
place!(Field::<u8>(Variant(_1.1, 2), 0)) = !_16.2.2;
_13.0 = core::ptr::addr_of!(_19.3);
_16.1 = !RET;
_1.1 = Adt17::Variant1 { fld0: _4,fld1: _19.0,fld2: _21,fld3: 1415754401979086907_u64,fld4: _17.2.3 };
_25 = &_16.1;
_16.0.2 = _17.1 << _9;
(*_8) = !(*_25);
place!(Field::<i64>(Variant(_7, 3), 1)) = _11 * _21;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(19_usize, 22_usize, Move(_22), 10_usize, Move(_10), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(19_usize, 14_usize, Move(_14), 2_usize, Move(_2), 15_usize, Move(_15), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(99_u8), std::hint::black_box((-51369069786412824637097867436779656_i128)), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(3374344310424732767_u64));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){unsafe{printf("Adt17::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt17 {
Variant0{
fld0: i32,

},
Variant1{
fld0: bool,
fld1: char,
fld2: i64,
fld3: u64,
fld4: i128,

},
Variant2{
fld0: u8,
fld1: u64,
fld2: i8,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: bool,
fld1: i128,
fld2: i64,
fld3: i8,
fld4: Adt17,
}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: bool,

},
Variant1{
fld0: [u16; 4],
fld1: f32,
fld2: isize,
fld3: *const Adt17,
fld4: i16,
fld5: ((u32, f64), f64, u16, i128),
fld6: u16,

},
Variant2{
fld0: bool,

},
Variant3{
fld0: f32,
fld1: i64,
fld2: [u16; 5],
fld3: u32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: (f32, u16, ((u32, f64), f64, u16, i128)),
fld1: ([u8; 4],),
fld2: [i16; 5],
}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){unsafe{printf("Adt64::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: (usize,),
fld1: (u32, f64),
fld2: isize,
fld3: [i16; 4],
fld4: u64,
fld5: [u32; 2],
fld6: usize,
fld7: i128,

},
Variant1{
fld0: ([u8; 4],),
fld1: char,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt73{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt73 {
fld0: [u32; 2],
fld1: [i8; 1],
fld2: f32,
fld3: i128,
fld4: u16,
fld5: u8,
fld6: ((u32, f64), f64, u16, i128),
}
impl PrintFDebug for Adt83{
	unsafe fn printf_debug(&self){unsafe{printf("Adt83::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt83 {
Variant0{
fld0: [u16; 4],
fld1: u8,
fld2: [i8; 5],
fld3: [isize; 5],
fld4: [u32; 2],
fld5: [i16; 5],
fld6: u128,

},
Variant1{
fld0: bool,
fld1: *const isize,
fld2: (i8,),
fld3: f32,
fld4: ([u8; 4],),
fld5: *mut Adt50,
fld6: i64,
fld7: i128,

},
Variant2{
fld0: u128,

},
Variant3{
fld0: [i16; 2],
fld1: (i8,),
fld2: [isize; 5],
fld3: (f32, u16, ((u32, f64), f64, u16, i128)),

}}
impl PrintFDebug for Adt85{
	unsafe fn printf_debug(&self){unsafe{printf("Adt85::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt85 {
Variant0{
fld0: *const i8,
fld1: *mut Adt50,
fld2: [i8; 2],
fld3: i8,
fld4: [u8; 4],

},
Variant1{
fld0: (usize,),
fld1: Adt83,
fld2: u64,
fld3: *mut usize,

},
Variant2{
fld0: ([u8; 4],),
fld1: [u8; 4],
fld2: Adt17,

}}

