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
pub fn fn0(mut _1: u16,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: u64,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u8) -> [i64; 4] {
mir! {
type RET = [i64; 4];
let _10: &'static i16;
let _11: isize;
let _12: i8;
let _13: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _14: Adt54;
let _15: [char; 8];
let _16: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _17: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _18: u8;
let _19: [u16; 2];
let _20: (isize, i32, u64, f64);
let _21: f32;
let _22: [u32; 6];
let _23: Adt57;
let _24: f64;
let _25: bool;
let _26: Adt56;
let _27: *const u16;
let _28: (f32, (isize, i32, u64, f64));
let _29: isize;
let _30: i8;
let _31: Adt49;
let _32: (u32,);
let _33: ();
let _34: ();
{
RET = [6782856748701258141_i64,(-8365739572722402736_i64),(-6037463083244706052_i64),(-6964967316487465596_i64)];
_2 = 155060381542031842070771681091992561997_u128;
_4 = !(-29_i8);
_5 = !17351763382771884952_u64;
_11 = 16456902067625252884_usize as isize;
_11 = (-126_isize) - (-25_isize);
_3 = 2769920377_u32 as isize;
_8 = (-27936_i16) as i128;
_9 = !178_u8;
_11 = 720991039_i32 as isize;
_1 = !4194_u16;
RET = [(-5997693117391194672_i64),(-6648586805796319696_i64),(-8678386943661317166_i64),6992199636353405682_i64];
_8 = 87639955877934135081688050640676308142_i128 * 13032073720946494087970381117558211362_i128;
_1 = true as u16;
_13.0 = _1;
_13.2.1.1 = (-1962731977_i32) - (-1686979659_i32);
_6 = !_13.2.1.1;
_1 = _13.0;
_9 = 133_u8 - 124_u8;
RET = [(-3792765200877782039_i64),(-6366439016296305140_i64),8275509445337925373_i64,(-5253035650069623717_i64)];
RET = [4084107487767489135_i64,(-830656674102367483_i64),(-8964701012741713456_i64),(-1452861027893689911_i64)];
_13.3 = -7151891463740581196_i64;
Goto(bb1)
}
bb1 = {
_8 = 155006216964962257544971831733700871436_i128 << _4;
_7 = _8 as i64;
_6 = _13.2.1.1;
_13.2.2 = [(-11063_i16),25063_i16,(-9686_i16),28932_i16,(-5082_i16),(-18293_i16),24690_i16,17240_i16];
_13.2.0 = [_13.2.1.1,_13.2.1.1,_6,_13.2.1.1,_6,_13.2.1.1];
_13.3 = _7;
_11 = _3;
_13.2.1.1 = _6;
_17.2 = _13.2.2;
_15 = ['\u{22666}','\u{3e9ce}','\u{db0ed}','\u{c9b4a}','\u{e8e5c}','\u{b27f9}','\u{78f3c}','\u{24073}'];
_11 = -_3;
_16.2 = [(-5414_i16),16254_i16,(-24483_i16),29477_i16,18653_i16,286_i16,(-31121_i16),5658_i16];
Goto(bb2)
}
bb2 = {
_13.2.1.3 = 192245888_u32 as f64;
_17.1.1 = !_6;
_16.1.3 = _13.2.1.3;
_16.1.0 = !_11;
_17.1.3 = _9 as f64;
_13.2.1.2 = '\u{10addf}' as u64;
_12 = '\u{f6925}' as i8;
_18 = false as u8;
_13.2.1.2 = _17.1.1 as u64;
_13.3 = 3823413804_u32 as i64;
Call(_16.1.1 = fn1(_13.2.1.3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = -_13.3;
_16.1.2 = _5 << _12;
_17.1.1 = -_13.2.1.1;
_17.2 = [(-3581_i16),(-23248_i16),(-16487_i16),(-31416_i16),(-14076_i16),(-19153_i16),17620_i16,(-11665_i16)];
_20.1 = _16.1.1 * _17.1.1;
_8 = 140037741974015046154269541016264042950_i128 * 150796112140351988676948392296076603176_i128;
_17 = (_13.2.0, _16.1, _16.2);
_17.2 = _13.2.2;
RET = [_13.3,_7,_13.3,_13.3];
_13.2.1.2 = _16.1.2 << _11;
match _2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
155060381542031842070771681091992561997 => bb11,
_ => bb10
}
}
bb4 = {
_13.2.1.3 = 192245888_u32 as f64;
_17.1.1 = !_6;
_16.1.3 = _13.2.1.3;
_16.1.0 = !_11;
_17.1.3 = _9 as f64;
_13.2.1.2 = '\u{10addf}' as u64;
_12 = '\u{f6925}' as i8;
_18 = false as u8;
_13.2.1.2 = _17.1.1 as u64;
_13.3 = 3823413804_u32 as i64;
Call(_16.1.1 = fn1(_13.2.1.3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_8 = 155006216964962257544971831733700871436_i128 << _4;
_7 = _8 as i64;
_6 = _13.2.1.1;
_13.2.2 = [(-11063_i16),25063_i16,(-9686_i16),28932_i16,(-5082_i16),(-18293_i16),24690_i16,17240_i16];
_13.2.0 = [_13.2.1.1,_13.2.1.1,_6,_13.2.1.1,_6,_13.2.1.1];
_13.3 = _7;
_11 = _3;
_13.2.1.1 = _6;
_17.2 = _13.2.2;
_15 = ['\u{22666}','\u{3e9ce}','\u{db0ed}','\u{c9b4a}','\u{e8e5c}','\u{b27f9}','\u{78f3c}','\u{24073}'];
_11 = -_3;
_16.2 = [(-5414_i16),16254_i16,(-24483_i16),29477_i16,18653_i16,286_i16,(-31121_i16),5658_i16];
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
_20.0 = _2 as isize;
_13.2 = (_17.0, _16.1, _17.2);
_16.0 = _17.0;
_9 = _18;
_13.2 = (_16.0, _17.1, _17.2);
_4 = _12;
_12 = _4 << _20.1;
_11 = false as isize;
_18 = _16.1.2 as u8;
_2 = 66794729210724554922314829789226915830_u128 >> _18;
_22 = [436922345_u32,1813638100_u32,2750111576_u32,2573592596_u32,1448969422_u32,1022138916_u32];
_13 = (_1, _1, _16, _7);
_13.2.1.1 = _20.1 * _20.1;
_13.1 = !_1;
_11 = _16.1.0;
_13.3 = _7;
_20.0 = -_13.2.1.0;
_9 = !_18;
_20 = (_13.2.1.0, _13.2.1.1, _13.2.1.2, _17.1.3);
_18 = !_9;
_17 = (_13.2.0, _20, _13.2.2);
_20 = _13.2.1;
_18 = _17.1.2 as u8;
_19 = [_13.0,_1];
_17.1 = (_11, _16.1.1, _20.2, _16.1.3);
_17.1.2 = _13.2.1.2 + _13.2.1.2;
Call(_12 = core::intrinsics::bswap(_4), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_17.1 = (_11, _20.1, _13.2.1.2, _20.3);
_13.2.1.3 = -_16.1.3;
_13.2.1.3 = _17.1.3;
_13.0 = !_13.1;
_13.2.1.1 = 2144720689_u32 as i32;
_17.1.2 = _16.1.2;
_25 = false & true;
_26.fld6 = [_1,_1];
_13.3 = _7;
_9 = _18;
_26.fld3 = _18 as i8;
_26.fld0 = core::ptr::addr_of!(_13.1);
_24 = _8 as f64;
_1 = _13.0;
_20 = (_3, _16.1.1, _5, _17.1.3);
_13.2.1.0 = -_16.1.0;
_26.fld6 = [_13.0,_13.1];
_26.fld0 = core::ptr::addr_of!(_1);
_17.2 = _16.2;
Call(_7 = core::intrinsics::transmute(_13.2.1.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26.fld5 = !_9;
Goto(bb14)
}
bb14 = {
_28.0 = _2 as f32;
_26.fld3 = _24 as i8;
_13.2.1.2 = _17.1.2;
_13.1 = '\u{7de66}' as u16;
_28.1.1 = !_17.1.1;
_17.1.0 = _20.0;
_28.1.0 = 30733_i16 as isize;
_5 = !_17.1.2;
_16.2 = [(-18837_i16),(-7187_i16),22182_i16,(-17384_i16),5516_i16,5424_i16,(-11095_i16),3728_i16];
_13.2.1.1 = _17.1.1;
_28.1.2 = _16.1.2 ^ _13.2.1.2;
_28.1.3 = _17.1.3;
_20.0 = _3 - _17.1.0;
_20.3 = _16.1.3;
_13.2.1.1 = _28.1.1 >> _17.1.1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 8_usize, Move(_8), 4_usize, Move(_4), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 7_usize, Move(_7), 12_usize, Move(_12), 22_usize, Move(_22), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f64,mut _2: u128) -> i32 {
mir! {
type RET = i32;
let _3: u8;
let _4: [i128; 6];
let _5: f32;
let _6: *mut (isize, *mut bool);
let _7: char;
let _8: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _9: u64;
let _10: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _11: Adt58;
let _12: [u32; 6];
let _13: *const i128;
let _14: char;
let _15: [u16; 2];
let _16: [char; 8];
let _17: [i64; 4];
let _18: bool;
let _19: ();
let _20: ();
{
RET = '\u{d3c65}' as i32;
_4 = [(-115783716482153505043243841086971757815_i128),(-146935475328608091015128277272464519864_i128),16654799202723166635405977475586976082_i128,(-39991360823447944106258487801907806235_i128),(-142825702563603573975719103225455251606_i128),(-102592296316106014873582803208087549337_i128)];
_1 = 10_u8 as f64;
match _2 {
0 => bb1,
1 => bb2,
155060381542031842070771681091992561997 => bb4,
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
RET = 332348957_i32 * (-1700013079_i32);
Call(_1 = fn2(_4, _4, _2, RET, _2, _4, _2, _2, _4, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = 3153583341163506477_u64 as f64;
_3 = 2866236791227715896_u64 as u8;
_1 = (-64616718023647085315956826633610950426_i128) as f64;
RET = 1091567440_i32;
_1 = (-85_isize) as f64;
_4 = [18019581887438610879723832116473578767_i128,89103110250285996328229805237185387478_i128,158270618455657982649060485409487661407_i128,(-155610027209333659929496798816658823793_i128),(-98227134757957731306689917721349140591_i128),(-2268730736188176865542795077437308846_i128)];
_8.1.2 = 23214_u16 as u64;
_8.1.1 = !RET;
_8.2 = [18600_i16,(-12425_i16),6143_i16,(-14904_i16),15637_i16,16337_i16,7981_i16,(-30641_i16)];
match RET {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1091567440 => bb13,
_ => bb12
}
}
bb6 = {
RET = 332348957_i32 * (-1700013079_i32);
Call(_1 = fn2(_4, _4, _2, RET, _2, _4, _2, _2, _4, _4), ReturnTo(bb5), UnwindUnreachable())
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
Return()
}
bb12 = {
Return()
}
bb13 = {
_3 = 9_u8 << _8.1.1;
_8.1.3 = _1;
_8.1.0 = 68_isize << _8.1.1;
_8.1.3 = _1;
_8.1.3 = -_1;
_7 = '\u{c0a5d}';
_10.1 = (_8.1.0, RET, _8.1.2, _8.1.3);
_10.1 = (_8.1.0, RET, _8.1.2, _8.1.3);
_10.1.0 = _8.1.0;
_4 = [23020875567563395567471855929100812960_i128,(-7409971776154674275021656368877912178_i128),52780790349144187420829478746008531212_i128,163063127492762668709776030052615853829_i128,(-154642486304174064131879558159902033002_i128),(-21919108012169657770576873480851571685_i128)];
_10.2 = [6359_i16,(-8840_i16),17874_i16,(-22141_i16),(-32662_i16),30416_i16,(-19045_i16),20477_i16];
_8.1.1 = RET ^ RET;
RET = true as i32;
_8.1.0 = _10.1.0;
_9 = _10.1.0 as u64;
_12 = [2323964365_u32,2929994198_u32,1271454559_u32,857560007_u32,3875749593_u32,1084918071_u32];
_12 = [2018067083_u32,3008549503_u32,225250063_u32,2985086310_u32,2716903528_u32,4280874522_u32];
_4 = [(-105429325735406967893074487749630154856_i128),54509759007969042849923065619812460336_i128,(-130672726440963402548255031281117593683_i128),133642384482995569951132840619208880919_i128,(-77310604217477725715474063954876453214_i128),(-59101532622670559515614439188334241668_i128)];
_12 = [3328898577_u32,1355787747_u32,3528530937_u32,16218376_u32,1593056832_u32,3880325652_u32];
_8.1.2 = _9;
_10.1.1 = -_8.1.1;
RET = 2557248814_u32 as i32;
_10.1 = (_8.1.0, RET, _8.1.2, _1);
_7 = '\u{7a332}';
match _2 {
0 => bb6,
1 => bb10,
155060381542031842070771681091992561997 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_10.1.0 = !_8.1.0;
_8.2 = [23515_i16,(-23709_i16),5090_i16,(-8066_i16),29283_i16,(-29100_i16),(-5288_i16),14873_i16];
_8.1.0 = 20291_i16 as isize;
_7 = '\u{79cc6}';
_4 = [(-73160106473426779679361789158856865773_i128),28076483840008374188349694789071520730_i128,(-62140061771805438857439973568276416298_i128),(-120481543646041107954130076362767423020_i128),(-96406861552437557282523442751583571866_i128),(-164820966081884094527926912073171172354_i128)];
_10.1.2 = !_8.1.2;
_14 = _7;
_14 = _7;
_8.1.3 = -_10.1.3;
_16 = [_14,_14,_7,_14,_14,_14,_14,_14];
_4 = [(-95466279682670209464407695411421899924_i128),35190307014239839970715032736065456311_i128,25438559418036408210630325684533179761_i128,(-54887003364310950431594587991560364693_i128),49362016029770515941960482470467481351_i128,(-33872483411608447006370988176875369664_i128)];
_2 = _8.1.1 as u128;
_5 = _8.1.0 as f32;
_12 = [4206607919_u32,2394543452_u32,4291928066_u32,167164577_u32,147918181_u32,1618146800_u32];
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(1_usize, 3_usize, Move(_3), 14_usize, Move(_14), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: u128,mut _4: i32,mut _5: u128,mut _6: [i128; 6],mut _7: u128,mut _8: u128,mut _9: [i128; 6],mut _10: [i128; 6]) -> f64 {
mir! {
type RET = f64;
let _11: [i32; 6];
let _12: *mut *mut *const (isize, *mut bool);
let _13: Adt51;
let _14: [i128; 6];
let _15: (isize, i32, u64, f64);
let _16: *const i128;
let _17: (f32, (isize, i32, u64, f64));
let _18: i64;
let _19: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _20: Adt58;
let _21: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _22: i32;
let _23: f32;
let _24: isize;
let _25: (f32, (isize, i32, u64, f64));
let _26: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _27: i128;
let _28: u16;
let _29: ();
let _30: ();
{
_4 = !(-911842517_i32);
_2 = _1;
_3 = _4 as u128;
RET = (-9223372036854775808_isize) as f64;
_9 = [(-110809912160051712762712638859266013756_i128),(-106275408503739244948607220125760927447_i128),90835938557374152280202137472859087372_i128,(-22398745255555831169440541544292937779_i128),133872377152456007698915747320258343226_i128,(-17268227520016725767252263617516806365_i128)];
_7 = !_5;
Goto(bb1)
}
bb1 = {
_1 = [71969385185587073447408307130746212512_i128,81014043803384625739714683785919570597_i128,(-12446930504284902489342383766282840360_i128),9654702521464009926430942340129537972_i128,90716220407753864571221525863021105228_i128,88733225474694976531877944916262856399_i128];
_3 = 95_i8 as u128;
_14 = _6;
RET = 719641705_u32 as f64;
_15.3 = RET;
_11 = [_4,_4,_4,_4,_4,_4];
_11 = [_4,_4,_4,_4,_4,_4];
_15.1 = false as i32;
_4 = _15.3 as i32;
RET = _15.3 + _15.3;
_8 = !_5;
Goto(bb2)
}
bb2 = {
_17.1.2 = 10601894093538776517_u64;
_17.1.0 = (-9223372036854775808_isize);
_11 = [_15.1,_15.1,_4,_4,_15.1,_4];
_9 = _2;
_17.1.0 = -39_isize;
_3 = _8 ^ _5;
_19.1.1 = 24656_u16 as i32;
_15.0 = _17.1.0;
Goto(bb3)
}
bb3 = {
_19.1.2 = _17.1.2;
Goto(bb4)
}
bb4 = {
_19.2 = [(-30491_i16),(-22529_i16),18094_i16,(-22577_i16),4317_i16,(-17302_i16),16339_i16,11903_i16];
_18 = (-8692705191646899595_i64) << _19.1.1;
_17.1.0 = _8 as isize;
_19.2 = [2290_i16,28464_i16,(-32288_i16),(-20354_i16),6127_i16,(-20853_i16),(-16682_i16),7680_i16];
match _17.1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
10601894093538776517 => bb9,
_ => bb8
}
}
bb5 = {
_19.1.2 = _17.1.2;
Goto(bb4)
}
bb6 = {
_17.1.2 = 10601894093538776517_u64;
_17.1.0 = (-9223372036854775808_isize);
_11 = [_15.1,_15.1,_4,_4,_15.1,_4];
_9 = _2;
_17.1.0 = -39_isize;
_3 = _8 ^ _5;
_19.1.1 = 24656_u16 as i32;
_15.0 = _17.1.0;
Goto(bb3)
}
bb7 = {
_1 = [71969385185587073447408307130746212512_i128,81014043803384625739714683785919570597_i128,(-12446930504284902489342383766282840360_i128),9654702521464009926430942340129537972_i128,90716220407753864571221525863021105228_i128,88733225474694976531877944916262856399_i128];
_3 = 95_i8 as u128;
_14 = _6;
RET = 719641705_u32 as f64;
_15.3 = RET;
_11 = [_4,_4,_4,_4,_4,_4];
_11 = [_4,_4,_4,_4,_4,_4];
_15.1 = false as i32;
_4 = _15.3 as i32;
RET = _15.3 + _15.3;
_8 = !_5;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_21.0 = [_19.1.1,_4,_19.1.1,_4,_19.1.1,_19.1.1];
_21.1.0 = 2455109918_u32 as isize;
_15 = (_21.1.0, _19.1.1, _17.1.2, RET);
_15.2 = _17.1.2;
_11 = _21.0;
_17.1.1 = -_4;
_21.0 = _11;
_4 = !_19.1.1;
_20 = Adt58::Variant2 { fld0: _17.1.2 };
_15.2 = _17.1.2;
_21.1.3 = _15.3 - _15.3;
Call(_17.1.3 = fn3(_14, _10, _8, Move(_20)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.1.3 = RET - RET;
_21.0 = [_15.1,_19.1.1,_17.1.1,_19.1.1,_4,_4];
_5 = _3;
_21.1.2 = _15.2;
_19.1.1 = _15.1 & _4;
Goto(bb11)
}
bb11 = {
_22 = _19.1.1 - _17.1.1;
_17.1.3 = -_15.3;
_17.1.3 = -_21.1.3;
_8 = !_3;
_4 = _22;
_15.2 = _17.1.2 % _21.1.2;
_2 = [(-59063460022783071238637051829901205306_i128),146633498951461140737899811897540877106_i128,(-15013404382853514302467552648374644598_i128),5860549454521527186581043234119794744_i128,8469538495931318789881497343696760447_i128,43679224963865501290624174161126101467_i128];
_17.1 = _15;
_21.0 = [_22,_22,_22,_4,_4,_17.1.1];
_4 = 9685_u16 as i32;
_24 = 86_i8 as isize;
_2 = [68024216026557158103023415473218079414_i128,(-6157802700078724376526514598438614826_i128),(-6905880210464733957131722199094746632_i128),148187132558653291724592768813784528156_i128,136764374181533121662776654130313069741_i128,(-84162674328133567831909845666527911886_i128)];
_21.2 = [26834_i16,(-28996_i16),(-8920_i16),(-22511_i16),(-12852_i16),(-300_i16),27987_i16,29213_i16];
_2 = [(-149876132525651847420322850074953680515_i128),(-100742892887894756100384217642630039411_i128),169818676062185195822660158860076302771_i128,45173095502584058775538773871208125267_i128,98865810071727716118702599112381062579_i128,(-18681547346474735735344571188510615114_i128)];
_25.0 = _18 as f32;
_17.0 = RET as f32;
_21 = (_11, _15, _19.2);
_17.1.1 = -_19.1.1;
_17 = (_25.0, _21.1);
RET = 22121_i16 as f64;
_21 = (_11, _15, _19.2);
_15.1 = -_19.1.1;
match _19.1.2 {
10601894093538776517 => bb13,
_ => bb12
}
}
bb12 = {
_1 = [71969385185587073447408307130746212512_i128,81014043803384625739714683785919570597_i128,(-12446930504284902489342383766282840360_i128),9654702521464009926430942340129537972_i128,90716220407753864571221525863021105228_i128,88733225474694976531877944916262856399_i128];
_3 = 95_i8 as u128;
_14 = _6;
RET = 719641705_u32 as f64;
_15.3 = RET;
_11 = [_4,_4,_4,_4,_4,_4];
_11 = [_4,_4,_4,_4,_4,_4];
_15.1 = false as i32;
_4 = _15.3 as i32;
RET = _15.3 + _15.3;
_8 = !_5;
Goto(bb2)
}
bb13 = {
_21.1.3 = 27_u8 as f64;
_19.1 = _21.1;
_25 = (_17.0, _19.1);
_10 = [5179135806414427862220694420817157028_i128,138573399748888117359783425232233328013_i128,(-35163114036452565434091245574507523045_i128),131195284743908890300399861325695780244_i128,(-35502570070921104042568022822214730667_i128),(-85276699840859135293936723684896149368_i128)];
_23 = _25.0 - _17.0;
_11 = _21.0;
_15.2 = _25.1.2 - _17.1.2;
_21.1.3 = -RET;
_26 = (_11, _15, _19.2);
_21 = (_26.0, _15, _26.2);
_15 = (_21.1.0, _26.1.1, _26.1.2, _21.1.3);
Call(_21.2 = core::intrinsics::transmute(_19.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_27 = (-60794974840717122999672259078485630798_i128);
_17 = (_25.0, _15);
_15.2 = _21.1.2;
RET = _19.1.3 * _19.1.3;
_21.1 = (_17.1.0, _22, _17.1.2, _15.3);
_26.1.0 = -_17.1.0;
_10 = [_27,_27,_27,_27,_27,_27];
_28 = 2572_u16;
_19.0 = [_22,_22,_15.1,_26.1.1,_15.1,_25.1.1];
_21.1 = (_24, _19.1.1, _17.1.2, _25.1.3);
_19.1 = (_15.0, _4, _21.1.2, _17.1.3);
_25.1 = (_21.1.0, _21.1.1, _26.1.2, _21.1.3);
_2 = _6;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 3_usize, Move(_3), 11_usize, Move(_11), 1_usize, Move(_1), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 24_usize, Move(_24), 2_usize, Move(_2), 22_usize, Move(_22), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: u128,mut _4: Adt58) -> f64 {
mir! {
type RET = f64;
let _5: isize;
let _6: *const u16;
let _7: [u16; 2];
let _8: f64;
let _9: [u32; 6];
let _10: [u16; 2];
let _11: isize;
let _12: [u16; 2];
let _13: [char; 8];
let _14: f32;
let _15: [i32; 6];
let _16: isize;
let _17: bool;
let _18: *const u16;
let _19: f64;
let _20: isize;
let _21: ();
let _22: ();
{
RET = Field::<u64>(Variant(_4, 2), 0) as f64;
_2 = [(-149492619724737398133535274717666085468_i128),3087886467273755570631902611751700600_i128,116974830597845847355642398335842948898_i128,98667442841059713701339900491833863763_i128,(-94181065292714236998599353421962286272_i128),(-4080600213390879558964420441059023291_i128)];
_2 = [(-101356584423634426259000758459039714089_i128),(-161779493921037904728340584185916889678_i128),(-55978077100535949019761742684074953224_i128),150872578188026052338193814271228593750_i128,67319593759517780638495991497037641052_i128,(-89404090860095749073095539665529081723_i128)];
RET = 59525_u16 as f64;
RET = (-33_i8) as f64;
_1 = [20186437103918179585196760149435676637_i128,66624052355652003300547476938547785696_i128,(-68254537184256089159388431558502177466_i128),(-100908585549552111387086090006022689793_i128),(-6339977782567853161356553192034573486_i128),(-8456136769696638588651444157262359423_i128)];
RET = 135592626_u32 as f64;
RET = 2958766104_u32 as f64;
_3 = !275691667440612042930258394853590284035_u128;
RET = _3 as f64;
SetDiscriminant(_4, 0);
_2 = _1;
_1 = [72590314265507652463146284882964986508_i128,(-41215337190293416973976523128589928132_i128),(-110575740128257900803791990607490499143_i128),127247138491895960497806008885370856137_i128,92118631860512601487299146060424676834_i128,136085457017422543113820259204327537619_i128];
_2 = _1;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).1 = 2617964550_u32 - 3068271462_u32;
_1 = [(-86193770583975736181472037725131035421_i128),122079989094011983210717217391711807897_i128,(-94532767129005094939945401514479324041_i128),(-27221949948633625726744665018112192792_i128),(-99643095801053344579723747040902970103_i128),54876192641182528007765044239998475474_i128];
Call(RET = fn4(_1, _1, _1, _1, Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2).1, _1, Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2).1, _2, _2, _1, _2, _3, _1, _1, Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2).1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [42505560470958292807258327124437732626_i128,120650070993019938008990025267983454701_i128,(-47525643526677255746256725333201594301_i128),128555280930405624854640950118624244456_i128,157055184125041079871124138234767243267_i128,(-111999328587207072411681966010472948427_i128)];
place!(Field::<i8>(Variant(_4, 0), 3)) = 109_i8 + 20_i8;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).1 = 3056511239_u32 + 3863852786_u32;
RET = 29740_i16 as f64;
_2 = _1;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{ee107}';
_2 = [124963702543742354238700667462674629069_i128,78461458556687997576735730923831231326_i128,(-96177553005064308337378616169865469184_i128),145336237938070365869550092759020024920_i128,(-155795704549856652481677002113807549494_i128),(-121082857394396112824596753058913572767_i128)];
_3 = !119434744125454281875129978403392759520_u128;
_2 = [72346338142047143419555851663469662741_i128,(-40847735659073164524812405304702140812_i128),51946629595858283526148090332535857232_i128,(-168333322629641451288240061273937705304_i128),114697840675396605493796534565508665137_i128,105916021286679282297293982397597147757_i128];
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).1 = !3077121234_u32;
_2 = _1;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{45ee4}';
RET = (-1213341656_i32) as f64;
_5 = 9223372036854775807_isize << _3;
place!(Field::<i8>(Variant(_4, 0), 3)) = (-16_i8) >> _5;
RET = 1096794688_i32 as f64;
_3 = 306727680648670642675837103644107328634_u128;
RET = 1939_u16 as f64;
_1 = _2;
_5 = -9223372036854775807_isize;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{a7848}';
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{84532}';
RET = 3686519769980837976_u64 as f64;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{88f75}';
place!(Field::<i8>(Variant(_4, 0), 3)) = 108855894420704347425590358869671216541_i128 as i8;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 0), 2)).3 = '\u{58932}';
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
306727680648670642675837103644107328634 => bb10,
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
place!(Field::<i8>(Variant(_4, 0), 3)) = !79_i8;
_4 = Adt58::Variant2 { fld0: 9267387893490157982_u64 };
RET = 41850_u16 as f64;
place!(Field::<u64>(Variant(_4, 2), 0)) = true as u64;
RET = 642895926362093943_usize as f64;
place!(Field::<u64>(Variant(_4, 2), 0)) = 15771942400298506182_u64;
SetDiscriminant(_4, 3);
place!(Field::<i128>(Variant(_4, 3), 7)) = (-119997623760757671006980624214884620867_i128);
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{49746}';
RET = (-496038005_i32) as f64;
_3 = Field::<i128>(Variant(_4, 3), 7) as u128;
place!(Field::<*mut [u32; 6]>(Variant(_4, 3), 4)) = core::ptr::addr_of_mut!(_9);
_8 = -RET;
place!(Field::<*mut [u32; 6]>(Variant(_4, 3), 4)) = core::ptr::addr_of_mut!(_9);
_5 = (-9223372036854775808_isize);
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{2f8f4}';
place!(Field::<[i128; 6]>(Variant(_4, 3), 3)) = _1;
_10 = [43706_u16,154_u16];
match Field::<i128>(Variant(_4, 3), 7) {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb11,
220284743160180792456393983216883590589 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
place!(Field::<usize>(Variant(_4, 3), 5)) = !0_usize;
_11 = _5 + _5;
_1 = _2;
_8 = 1384_i16 as f64;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).1 = _3 as u32;
_7 = [47752_u16,3147_u16];
_9 = [Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).1];
place!(Field::<u128>(Variant(_4, 3), 1)) = _3 & _3;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{ed68e}';
_12 = _10;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).2 = core::ptr::addr_of_mut!(_17);
place!(Field::<[i128; 6]>(Variant(_4, 3), 3)) = [Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7)];
_5 = (-20724_i16) as isize;
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{d5e46}';
_1 = [Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7)];
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).2 = core::ptr::addr_of_mut!(_17);
_2 = [Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7)];
place!(Field::<*mut [u32; 6]>(Variant(_4, 3), 4)) = core::ptr::addr_of_mut!(_9);
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{ddc44}';
match Field::<i128>(Variant(_4, 3), 7) {
0 => bb3,
220284743160180792456393983216883590589 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_13 = [Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3,Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0).3];
_3 = !Field::<u128>(Variant(_4, 3), 1);
place!(Field::<u128>(Variant(_4, 3), 1)) = !_3;
place!(Field::<i128>(Variant(_4, 3), 7)) = 173_u8 as i128;
_12 = [16512_u16,27230_u16];
_10 = [35818_u16,35956_u16];
_16 = _5 << Field::<usize>(Variant(_4, 3), 5);
_19 = 3376123511111183628_i64 as f64;
_7 = [9405_u16,40863_u16];
place!(Field::<[i128; 6]>(Variant(_4, 3), 3)) = [Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7),Field::<i128>(Variant(_4, 3), 7)];
place!(Field::<(*mut bool, u32, *mut bool, char)>(Variant(_4, 3), 0)).3 = '\u{acd93}';
_20 = -_11;
_10 = _7;
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(3_usize, 7_usize, Move(_7), 10_usize, Move(_10), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(3_usize, 20_usize, Move(_20), 16_usize, Move(_16), 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: u32,mut _6: [i128; 6],mut _7: u32,mut _8: [i128; 6],mut _9: [i128; 6],mut _10: [i128; 6],mut _11: [i128; 6],mut _12: u128,mut _13: [i128; 6],mut _14: [i128; 6],mut _15: u32,mut _16: [i128; 6]) -> f64 {
mir! {
type RET = f64;
let _17: char;
let _18: char;
let _19: isize;
let _20: f64;
let _21: char;
let _22: isize;
let _23: Adt47;
let _24: Adt47;
let _25: f64;
let _26: [i32; 6];
let _27: (i64,);
let _28: [i16; 8];
let _29: isize;
let _30: i8;
let _31: Adt44;
let _32: f32;
let _33: isize;
let _34: (i64,);
let _35: f64;
let _36: bool;
let _37: [char; 8];
let _38: isize;
let _39: f32;
let _40: u16;
let _41: [u32; 6];
let _42: (u32,);
let _43: u16;
let _44: u64;
let _45: char;
let _46: isize;
let _47: char;
let _48: *const i128;
let _49: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _50: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _51: ();
let _52: ();
{
_11 = [(-30517783880234872229745065212502919322_i128),(-17913851149860536819126489579276642626_i128),(-109894861685881890055311871406931751644_i128),70391987651297243715773063274685710088_i128,148327071581097365359164574769429979031_i128,(-69410155592026945335624360470727830361_i128)];
_8 = [8594704843092893892767296474838593668_i128,(-78547520062862702050171397599698758672_i128),9125937869776975737191709163975483585_i128,31210982400929407775087681330707927580_i128,(-92521202822125372178788947740182672894_i128),(-51010294995361188789692954487522797836_i128)];
_2 = [(-78400090245440550115155860264935336403_i128),(-38771147204783376637461933254696471927_i128),(-159931429698767946214477167455944217964_i128),169840033084728532576036592466629254089_i128,30499744282416062276419055982304034957_i128,(-123423627898287951105035723801043292136_i128)];
_14 = _8;
_11 = _10;
_11 = [4401851336967266826089190851474109899_i128,(-64474003059172685898651514683130641240_i128),141715368890248821321347710182205570740_i128,43665003684199597084111816110796656937_i128,130032115014412486289897291990111706319_i128,(-73227083944999950368790411155119829352_i128)];
_4 = [(-77301025322021995732708793034706988384_i128),76317745914914372125900088647869370755_i128,(-139393633911081280162054149791045818049_i128),(-118061982497315395848290176386822698316_i128),(-31198101866598028204860693378392656772_i128),66091365814451919615197571821088687221_i128];
RET = 9223372036854775807_isize as f64;
_4 = [(-49204379239994294836099093403869507783_i128),60231445542984376942747676737692136677_i128,5012207126088534604227967392273603045_i128,(-44002113391703729276669401062697799651_i128),101067251009902777910267132454906616764_i128,90466781871314430302096315196102737169_i128];
_17 = '\u{58640}';
_1 = _4;
_13 = [169791554058564308030397968809469073810_i128,(-72046510399423482682119794945043854016_i128),(-104391683006708349461385343076355940323_i128),(-43480554356265011520266856699218716619_i128),113501860401991902679451109396320771021_i128,67312735205508687528191216496563376819_i128];
_8 = _1;
_14 = [17794452201682087932022192996669481646_i128,(-72729561004424402323675156342179851346_i128),(-43504741852099807344202384841874911923_i128),22408369267408058924494015530026760178_i128,157518574939311558783950197801495091417_i128,(-68665855061793986233980020091701356550_i128)];
_13 = [130377806653247702721781557870799941627_i128,(-97232734924481734423558835528607469714_i128),(-10667384604984816152707829282911449598_i128),79329774057555392849945163462221864856_i128,53777043963945892852242945738338980676_i128,(-45869261188986348086956433809545023119_i128)];
RET = (-30429_i16) as f64;
Goto(bb1)
}
bb1 = {
_2 = [116586784831322965715493095922772623765_i128,(-144427561537052073211055984082815338420_i128),54343844539340065277741005715011637191_i128,154207048825610878940416950826366773763_i128,90252380976351728827520522157397655547_i128,21428430595052433539334317949493960466_i128];
_18 = _17;
_9 = [(-116030221750926614699533516390187939182_i128),(-8362975127639956832072402494343978932_i128),(-132103736418110994233002918820153640950_i128),(-98956086557178495407301234528726683041_i128),150374735854778761357134081035485177106_i128,40938365113911793402704323439023372458_i128];
_8 = _1;
_9 = [37537816960809500417490740278985037464_i128,(-18499253837265487534241688371263550880_i128),169026057012456538137372838948378092970_i128,70075451263263793652456875378841776655_i128,27791034554369358870694557748226479134_i128,150582085326859796878313908677182549462_i128];
_13 = _6;
_10 = [(-53449571768788670947643268755483546403_i128),(-109185177018588157396956601537617803791_i128),(-22220276882942268389417798749378588675_i128),72334349330405449993057157313951981708_i128,95761737001119176194365997991619763032_i128,52654679211340547774102542006433862664_i128];
_2 = [61012349110375666117596848189166483243_i128,(-19076933111065998760898156460361165379_i128),150592809122669812240472351553198621878_i128,(-140527797186394954945633803616769013720_i128),(-41204446410548125036593988751826255618_i128),(-128251356469048715823626807086654051344_i128)];
_5 = _7 & _7;
RET = 7773963432767659867_usize as f64;
_6 = [46217245750984432229400180328335171632_i128,(-158746962251265379057818201573002227469_i128),1197472735225755193943307470878998959_i128,97908923743868037731832808341248656782_i128,(-131527356914663275080864558983000610608_i128),(-59871640544624969695033861351871041703_i128)];
_13 = [28723531430217575390727254930462561833_i128,99017572226139403111457838477615279181_i128,19812051697809336205465316526133406914_i128,(-59047656709678989202583554244703368975_i128),(-129909082339018081505531778194236907704_i128),(-29074721278473807550623462634601271941_i128)];
Goto(bb2)
}
bb2 = {
_6 = [(-109499120831611980550614128229529505318_i128),12228167901552066456177731923087034041_i128,158153992647673852209851931412618038271_i128,45642531622452776280903801794236654492_i128,19427956298496676354737623648369644189_i128,125395007287761797252559766381993255579_i128];
_9 = [30532210465771355225646836039910848570_i128,(-106160956551738570680095543730843570738_i128),36698262670616040687048499316114924077_i128,162059778884420720074546974385802849644_i128,(-30218085910684289703128526503263342196_i128),(-41413029428699056982338919591330949901_i128)];
_14 = [(-112403291771100054584179400342707878270_i128),(-103131902733259686639175013903293344874_i128),95492124370117467083555428388358700631_i128,(-7401635067484058091333248384148363645_i128),132074892435171301805955032326712995767_i128,(-95775150351353537288141600760794268992_i128)];
_18 = _17;
_5 = _7;
_12 = !128164839563540861177260304004756350099_u128;
RET = (-1154_i16) as f64;
_7 = _17 as u32;
RET = _12 as f64;
_17 = _18;
_17 = _18;
_12 = 256837317615502413163588969532815547897_u128;
_15 = !_7;
_16 = [(-23022364227416356507666088389435062741_i128),153058551398254515670342972619180238824_i128,(-60408967653059599686034269117073598014_i128),167767107152095503981202156057309933274_i128,2676655918194986160296846420681457104_i128,(-136989973285759129891768218028576958898_i128)];
_21 = _17;
RET = _7 as f64;
_21 = _17;
_19 = (-9223372036854775808_isize);
_20 = -RET;
_6 = _11;
RET = _20;
match _12 {
0 => bb1,
1 => bb3,
256837317615502413163588969532815547897 => bb5,
_ => bb4
}
}
bb3 = {
_2 = [116586784831322965715493095922772623765_i128,(-144427561537052073211055984082815338420_i128),54343844539340065277741005715011637191_i128,154207048825610878940416950826366773763_i128,90252380976351728827520522157397655547_i128,21428430595052433539334317949493960466_i128];
_18 = _17;
_9 = [(-116030221750926614699533516390187939182_i128),(-8362975127639956832072402494343978932_i128),(-132103736418110994233002918820153640950_i128),(-98956086557178495407301234528726683041_i128),150374735854778761357134081035485177106_i128,40938365113911793402704323439023372458_i128];
_8 = _1;
_9 = [37537816960809500417490740278985037464_i128,(-18499253837265487534241688371263550880_i128),169026057012456538137372838948378092970_i128,70075451263263793652456875378841776655_i128,27791034554369358870694557748226479134_i128,150582085326859796878313908677182549462_i128];
_13 = _6;
_10 = [(-53449571768788670947643268755483546403_i128),(-109185177018588157396956601537617803791_i128),(-22220276882942268389417798749378588675_i128),72334349330405449993057157313951981708_i128,95761737001119176194365997991619763032_i128,52654679211340547774102542006433862664_i128];
_2 = [61012349110375666117596848189166483243_i128,(-19076933111065998760898156460361165379_i128),150592809122669812240472351553198621878_i128,(-140527797186394954945633803616769013720_i128),(-41204446410548125036593988751826255618_i128),(-128251356469048715823626807086654051344_i128)];
_5 = _7 & _7;
RET = 7773963432767659867_usize as f64;
_6 = [46217245750984432229400180328335171632_i128,(-158746962251265379057818201573002227469_i128),1197472735225755193943307470878998959_i128,97908923743868037731832808341248656782_i128,(-131527356914663275080864558983000610608_i128),(-59871640544624969695033861351871041703_i128)];
_13 = [28723531430217575390727254930462561833_i128,99017572226139403111457838477615279181_i128,19812051697809336205465316526133406914_i128,(-59047656709678989202583554244703368975_i128),(-129909082339018081505531778194236907704_i128),(-29074721278473807550623462634601271941_i128)];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_3 = [107755807930255829343706113347766460992_i128,48937882380001790902573845287516130999_i128,(-57368562207002858308884952243955068023_i128),(-56140584337485539612759964671257772718_i128),(-160097610055423658341393054960952259407_i128),(-91954918492773606027689736336222840288_i128)];
_18 = _21;
RET = -_20;
_20 = RET - RET;
_15 = _7;
_12 = !235885953468357165090247629287460954064_u128;
Goto(bb6)
}
bb6 = {
_11 = [(-21209193265180694368693422379647210218_i128),(-34472861522029427605001566850602370136_i128),114087797866789619637234642916499021144_i128,96753044770219414565851292191388915683_i128,104929949148570222433824901870777456677_i128,29922650848075956590509585858495008836_i128];
RET = _20 + _20;
_21 = _17;
_18 = _17;
_8 = _9;
_18 = _21;
_17 = _21;
RET = -_20;
_19 = !115_isize;
_9 = [(-39078703474345270834446349590175811608_i128),6215201958430055993684959543322095476_i128,(-13346764663549486970703973059190524655_i128),(-113579228306402013445070861424860048438_i128),(-70318322658363525721362436590857153518_i128),(-19495656262277359686792620696408651627_i128)];
_22 = 68666462513092201269671486563316960903_i128 as isize;
_17 = _18;
_7 = _12 as u32;
_5 = 158_u8 as u32;
_16 = [(-86946966203622941279095442326552824090_i128),16554724764475259040997153722950584268_i128,166233174201615013697455600840033320551_i128,21351394111819078476618723931909210356_i128,(-114326000066870690154672817060304969948_i128),53349236908122058616213648702128432958_i128];
_9 = [(-132371588295157797845814942077422110200_i128),164144181306352786778364645804239183939_i128,115574111178787179089176605122969494812_i128,53141439593684076147161030307332236060_i128,34869592473310963858017503070033534590_i128,(-66267186621415169481162451203035111617_i128)];
_20 = 16877336183631905421_u64 as f64;
RET = (-3269363936901015678_i64) as f64;
Call(_27.0 = fn5(_11, _6, _2, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = [21970798192274774971575838423858948216_i128,(-116422349952061206388921155366355770568_i128),(-89214855654131382531985040665260280982_i128),(-69432994180257971116844787004813056250_i128),(-43897986214918239885664993875901450350_i128),(-100024810693471629871878975230565753695_i128)];
_1 = [71794985127100369070196271881678106703_i128,7176752036487384201593943236611437982_i128,55405784433954769633582787127366612376_i128,(-77894985835536238897911266499190996319_i128),(-25212173184250982378748970309405354987_i128),161889028950358008287093176793818684526_i128];
_6 = _13;
_14 = _9;
_22 = RET as isize;
_19 = _22;
_3 = _8;
_26 = [1636292642_i32,(-342051542_i32),87663647_i32,(-1675797072_i32),(-1447494595_i32),(-1973588973_i32)];
_20 = RET + RET;
_6 = [(-120992423443765117021218700873778408856_i128),21236762501648056595150297599197451143_i128,(-138428896114479045332688283977797827128_i128),82499693568749992494014103811752812308_i128,18146092615775718220061467824165428421_i128,(-138928615559166274562510863946498888284_i128)];
_7 = _5;
_7 = _15;
_1 = [38104549955961138401664998613979317717_i128,(-87908245088270233997613436490239513839_i128),75306397600177157531513067134416290490_i128,(-47059729201492009147881854291851795243_i128),18796638866905837407088264716493358699_i128,89204400643628511039541526572047168711_i128];
_11 = [(-143936952347424236827513594206188617468_i128),125613894278197991010486900354179751590_i128,87538560134049554434714567546963890933_i128,45656068666072408503306694295278418747_i128,136135611885098532018469395797452460885_i128,13390672827446565750864975912076267626_i128];
_14 = _1;
_7 = _22 as u32;
RET = _20 + _20;
_29 = _19 ^ _19;
_11 = _4;
Goto(bb8)
}
bb8 = {
RET = _20;
_18 = _21;
_10 = _16;
_2 = [(-141048501664569998059288953429860580065_i128),66497914772327807916020576116405849600_i128,(-91622042211747864809586915793920186793_i128),134789454188026811573251655343189559844_i128,166061329852709565273199413957594206539_i128,(-128860541811561464002989337499397568221_i128)];
_19 = _29;
_15 = _5 & _5;
_2 = [116324857933531116551155319744037544656_i128,(-95903702122716103685744909334015257008_i128),137995993259877806127968584973162022523_i128,(-74854721108655410964684055704812272518_i128),18354360821304618722463690268654398599_i128,(-54855741351547161778352294274667669272_i128)];
_4 = [(-101195847697731069169609646133231887516_i128),129721684873233708627010431960948950896_i128,130761295804853162469256767219535803190_i128,18762978751198838553539186315558387710_i128,(-12580765312041791609207675236617281659_i128),(-46584500843954215742818562374691848555_i128)];
_11 = [(-96294147389884874852464736358029417639_i128),(-77699435999731003496425190457959753221_i128),39777574507421795599596115796650479189_i128,(-138764340282199378046220656609177059431_i128),77041989695904048249907422465386632791_i128,(-118345831510364166942071211271488536763_i128)];
_26 = [692005448_i32,527639487_i32,502978071_i32,960960056_i32,(-662377433_i32),296883935_i32];
_22 = !_29;
_15 = _5;
RET = _20 * _20;
_25 = -RET;
_28 = [(-4322_i16),(-27658_i16),(-6901_i16),893_i16,21892_i16,(-11506_i16),17564_i16,2890_i16];
_6 = [20100576330015837865214862084872727819_i128,70719105728756991756956479770620403173_i128,151106124849448130742419213850694680119_i128,(-159012657408855634453785036095726655305_i128),(-99170336111468203216378615771519518375_i128),(-40172511568011906568356425568442611491_i128)];
_28 = [(-3719_i16),(-30603_i16),(-6640_i16),(-20597_i16),(-21719_i16),26676_i16,11439_i16,(-19423_i16)];
_16 = _1;
_22 = (-66500669362537649475045828288891877359_i128) as isize;
_8 = [31324345752509412639792020291407919985_i128,6325631506081368217286417774224625603_i128,27266371678182334666509646336116431052_i128,123163695275387117378274780299858080082_i128,88923285945346190638381359263199972978_i128,(-98440249647145966265151405971832817726_i128)];
_30 = 81_i8 + 82_i8;
_33 = -_29;
_15 = _12 as u32;
Goto(bb9)
}
bb9 = {
_33 = _22 ^ _29;
_34 = (_27.0,);
_2 = _14;
_3 = _13;
RET = _25;
_21 = _17;
_9 = [(-81600635971281957922720217033928651082_i128),(-111880248024096822586331931971682728469_i128),(-71230561411289534657422926314605063969_i128),(-145429689194610579357183792535235236626_i128),(-143428706690708719859037247640735030970_i128),(-6333229122874575932541245177019602527_i128)];
_14 = [(-123410271711343405015118608847927950516_i128),(-67588742177811854550391711630966195666_i128),282908092037229312385932482894897185_i128,(-109459547791645413567762602754330084490_i128),(-1396750597001601978760094058800742520_i128),110999415748168006103679708194910311188_i128];
_27 = (_34.0,);
_37 = [_18,_21,_18,_18,_17,_18,_21,_18];
_12 = 211110726460821008066356326103731519905_u128 | 244552958797267312688234156389283722870_u128;
_21 = _18;
_17 = _21;
Goto(bb10)
}
bb10 = {
_36 = true;
_12 = 41634819662783611829827396493145282346_u128;
_21 = _17;
_32 = 3_usize as f32;
RET = (-24740_i16) as f64;
_16 = [(-72710157640150916740242775055417582735_i128),155495262437378074499741844542550089661_i128,27771357405341784290422067026529956921_i128,17633101790465390009483186628078705716_i128,(-49945521351073737510612830704701644968_i128),79177252834876791455467969740401864602_i128];
_4 = [(-64757565411717853789804723138098845314_i128),161473003549472404257485187767955779541_i128,42060206908818333183639518276210994273_i128,(-35168469325847433120426111600847046676_i128),(-35354994535293351539179805316247849243_i128),7637612518353588830104716555784324590_i128];
_30 = 84_i8;
_17 = _18;
_39 = _32 + _32;
_17 = _21;
_43 = !14550_u16;
match _12 {
41634819662783611829827396493145282346 => bb12,
_ => bb11
}
}
bb11 = {
_11 = [(-21209193265180694368693422379647210218_i128),(-34472861522029427605001566850602370136_i128),114087797866789619637234642916499021144_i128,96753044770219414565851292191388915683_i128,104929949148570222433824901870777456677_i128,29922650848075956590509585858495008836_i128];
RET = _20 + _20;
_21 = _17;
_18 = _17;
_8 = _9;
_18 = _21;
_17 = _21;
RET = -_20;
_19 = !115_isize;
_9 = [(-39078703474345270834446349590175811608_i128),6215201958430055993684959543322095476_i128,(-13346764663549486970703973059190524655_i128),(-113579228306402013445070861424860048438_i128),(-70318322658363525721362436590857153518_i128),(-19495656262277359686792620696408651627_i128)];
_22 = 68666462513092201269671486563316960903_i128 as isize;
_17 = _18;
_7 = _12 as u32;
_5 = 158_u8 as u32;
_16 = [(-86946966203622941279095442326552824090_i128),16554724764475259040997153722950584268_i128,166233174201615013697455600840033320551_i128,21351394111819078476618723931909210356_i128,(-114326000066870690154672817060304969948_i128),53349236908122058616213648702128432958_i128];
_9 = [(-132371588295157797845814942077422110200_i128),164144181306352786778364645804239183939_i128,115574111178787179089176605122969494812_i128,53141439593684076147161030307332236060_i128,34869592473310963858017503070033534590_i128,(-66267186621415169481162451203035111617_i128)];
_20 = 16877336183631905421_u64 as f64;
RET = (-3269363936901015678_i64) as f64;
Call(_27.0 = fn5(_11, _6, _2, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb12 = {
_18 = _21;
_25 = _20 + _20;
_46 = _19;
match _30 {
0 => bb9,
84 => bb13,
_ => bb6
}
}
bb13 = {
_26 = [574528846_i32,1365966618_i32,(-1110728329_i32),(-875963013_i32),189813160_i32,531553227_i32];
_27 = (_34.0,);
_44 = 3194086684491430196_u64;
_6 = [57914267132670485372817805105420771574_i128,142896557568463571221951010564535681263_i128,145825758640752394049598031269432225857_i128,(-4077153475568632750587551636239714309_i128),(-148778873973157866772236007346578354422_i128),(-67073990909993029927456227486465732747_i128)];
_18 = _21;
_16 = [(-106724023870457774683856518305521485374_i128),8624838361161216983276863765988063479_i128,(-89413417159132958229742333297325350535_i128),97538493955428875396768722807039210593_i128,125425370048047861568057623877617183802_i128,50724590415435041052394373300883461047_i128];
_42 = (_5,);
RET = _25;
_32 = _46 as f32;
_26 = [1809164265_i32,(-1867717284_i32),662297518_i32,1344317017_i32,(-1515573405_i32),576901488_i32];
_22 = !_33;
_11 = _16;
_7 = _43 as u32;
_46 = _44 as isize;
_13 = _3;
_38 = _44 as isize;
_7 = _42.0 << _15;
_26 = [1978195445_i32,(-824902969_i32),(-864356376_i32),(-1390447020_i32),(-1313584962_i32),(-1272269248_i32)];
_36 = _18 == _18;
_4 = [(-142934498798091311872335345454961213018_i128),(-49425023276126721951932680243302452543_i128),28995651223276487604608440183685416992_i128,(-60932191930755791160923800785774578663_i128),57151662699479323681963346319763383322_i128,56630637184596006139285942514331944983_i128];
_34.0 = (-711426354_i32) as i64;
RET = _20;
_37 = [_21,_17,_21,_21,_17,_18,_18,_18];
_6 = [21479760602849334622046549540212168511_i128,163276128705723895517522183102379482843_i128,82738539302942156203386436050068104591_i128,(-70007939876322762973721073730418917549_i128),90033663598306037390356459919588987498_i128,24616516575571072123011590115826695806_i128];
Goto(bb14)
}
bb14 = {
_1 = [75410214621791723945349435401852599689_i128,100154112559114579283535110652674432157_i128,(-56522975047506498258880296557853817503_i128),133770498821958515458930266691522050419_i128,108762768580616394820720818392655464408_i128,(-102415227412934751320182074727592742854_i128)];
_8 = [(-143702760781139141293058291401820267706_i128),(-23949093668172925403414405842429504091_i128),24662938206590236605571091359523794773_i128,162389376045280734686237533609766830503_i128,11638493431731371637086525390954700447_i128,48291922878641579137303866798198225052_i128];
_15 = !_7;
_7 = _15 + _15;
_37 = [_21,_21,_21,_17,_18,_17,_18,_21];
_6 = [24267041338808312554554899817402903515_i128,138525478405662763417291153948246007630_i128,4337897006377809931131096909908316553_i128,(-113377486449532792236813497355155543448_i128),124246921124656197175129558932144347881_i128,(-112103330429734564843876175159697719244_i128)];
_13 = [(-106726029153978091856485350449450865271_i128),104895063134217949024211802556779006792_i128,(-94334216511547935856677375972874290119_i128),(-108576943712450592772194410131304436402_i128),149632431952096046834021459888119609672_i128,39769182003837050580752967640281952713_i128];
_21 = _18;
_43 = !53105_u16;
_3 = [8114086878914102459900304309072998487_i128,138527207441079212768100803886344704530_i128,72493659062625007034795215396683819064_i128,165640708687835234148026642606988516015_i128,101124195682630022713487379059143778038_i128,(-90139692083755059244104819393407012441_i128)];
_35 = _25;
_49.2.1.2 = _44 & _44;
_50.2.1.0 = _29 * _19;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(4_usize, 22_usize, Move(_22), 9_usize, Move(_9), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(4_usize, 29_usize, Move(_29), 3_usize, Move(_3), 17_usize, Move(_17), 42_usize, Move(_42)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(4_usize, 19_usize, Move(_19), 2_usize, Move(_2), 14_usize, Move(_14), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(4_usize, 44_usize, Move(_44), 30_usize, Move(_30), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(4_usize, 26_usize, Move(_26), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6]) -> i64 {
mir! {
type RET = i64;
let _5: i16;
let _6: (i64,);
let _7: f32;
let _8: bool;
let _9: [i128; 6];
let _10: *const ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _11: isize;
let _12: isize;
let _13: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _14: f64;
let _15: *mut bool;
let _16: char;
let _17: ();
let _18: ();
{
RET = 3848260945604470298_i64 & 4155272169481272589_i64;
_4 = _3;
RET = (-4208507219282493607_i64) ^ 7371598244961273636_i64;
_4 = _1;
Call(_4 = fn6(_1, _3, _3, _3, _1, _3, _2, _3, _3, _3, _1, _3, _3, RET, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _3;
RET = (-4236062160272947783_i64) & (-9110143181258898660_i64);
_5 = (-20161_i16);
_3 = [43255983372506662383413563872611843877_i128,(-166163851047699452854238151102105958133_i128),(-110045866062152214847416782162237102381_i128),(-103335599224744237919621562511286205919_i128),138971620482715108132596810824093478507_i128,(-169920628856305053331829145188579690229_i128)];
RET = 3693678669624136016_i64 - 3570738728205616240_i64;
_6 = (RET,);
_4 = [161843474511412080042343661147980259966_i128,22773082736929332215188462492718226885_i128,(-112031129407914585219727744088385716408_i128),163294206806969419229875269780668967385_i128,(-6818334985132479828837714437860278461_i128),(-12809231206663371992438847012073095078_i128)];
_6 = (RET,);
_2 = _1;
_6 = (RET,);
_4 = [25052616116609462837925923702588925089_i128,159452837149189610193860601269965151008_i128,54992490957416047695857761234600559534_i128,(-23254506680130371555921034865950064535_i128),(-48263779037972309152329522925758721540_i128),87730832203201346057284134513299392866_i128];
_1 = [(-5999217444260320091734741730473709947_i128),(-51562922464454435884826923287339499834_i128),(-37705975379617532600531220164758223200_i128),(-41615235089498927563205627464354088765_i128),42718271390405675167531019390328457984_i128,(-48197551905674391756702123667087876657_i128)];
_7 = 105_isize as f32;
_5 = (-32012_i16);
RET = _6.0;
_7 = (-1388885375_i32) as f32;
_8 = !false;
_5 = (-3074_i16);
_5 = RET as i16;
_7 = (-9223372036854775808_isize) as f32;
_3 = [(-98476224312140995411729741852281325410_i128),(-143821255461700156881873174329499306865_i128),(-93073769492183540142561187275315710393_i128),(-145097274475602304261546593305193254985_i128),132630518788033162237535893784691394875_i128,(-37487355827751589428219613732152973762_i128)];
_3 = _1;
_8 = !true;
RET = _6.0 & _6.0;
_5 = _7 as i16;
_8 = _7 == _7;
Goto(bb2)
}
bb2 = {
_12 = !(-9223372036854775808_isize);
_3 = [133208116389287794924382759451304092544_i128,(-42440253641822386149669653651611474560_i128),(-131517233231928331093937415008185621622_i128),12551338642355782905390134485051972337_i128,(-119966100460308746695008993059161773203_i128),(-33235368887255994776895945662057401562_i128)];
_4 = _3;
_7 = 30_u8 as f32;
_13.1.3 = 199_u8 as f64;
_9 = [10882580110708927319480748991185199404_i128,(-156027808710937942734735644032760636116_i128),145543796180542152807895339945387974194_i128,70588699570803805469557978087106351437_i128,28024144579342947534898764781125753134_i128,73713155905865314806422277147238847102_i128];
_13.1.2 = 12940244672520588051_u64;
_12 = (-84_isize);
RET = _6.0 * _6.0;
_10 = core::ptr::addr_of!(_13);
_10 = core::ptr::addr_of!(_13);
match (*_10).1.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
12940244672520588051 => bb9,
_ => bb8
}
}
bb3 = {
_4 = _3;
RET = (-4236062160272947783_i64) & (-9110143181258898660_i64);
_5 = (-20161_i16);
_3 = [43255983372506662383413563872611843877_i128,(-166163851047699452854238151102105958133_i128),(-110045866062152214847416782162237102381_i128),(-103335599224744237919621562511286205919_i128),138971620482715108132596810824093478507_i128,(-169920628856305053331829145188579690229_i128)];
RET = 3693678669624136016_i64 - 3570738728205616240_i64;
_6 = (RET,);
_4 = [161843474511412080042343661147980259966_i128,22773082736929332215188462492718226885_i128,(-112031129407914585219727744088385716408_i128),163294206806969419229875269780668967385_i128,(-6818334985132479828837714437860278461_i128),(-12809231206663371992438847012073095078_i128)];
_6 = (RET,);
_2 = _1;
_6 = (RET,);
_4 = [25052616116609462837925923702588925089_i128,159452837149189610193860601269965151008_i128,54992490957416047695857761234600559534_i128,(-23254506680130371555921034865950064535_i128),(-48263779037972309152329522925758721540_i128),87730832203201346057284134513299392866_i128];
_1 = [(-5999217444260320091734741730473709947_i128),(-51562922464454435884826923287339499834_i128),(-37705975379617532600531220164758223200_i128),(-41615235089498927563205627464354088765_i128),42718271390405675167531019390328457984_i128,(-48197551905674391756702123667087876657_i128)];
_7 = 105_isize as f32;
_5 = (-32012_i16);
RET = _6.0;
_7 = (-1388885375_i32) as f32;
_8 = !false;
_5 = (-3074_i16);
_5 = RET as i16;
_7 = (-9223372036854775808_isize) as f32;
_3 = [(-98476224312140995411729741852281325410_i128),(-143821255461700156881873174329499306865_i128),(-93073769492183540142561187275315710393_i128),(-145097274475602304261546593305193254985_i128),132630518788033162237535893784691394875_i128,(-37487355827751589428219613732152973762_i128)];
_3 = _1;
_8 = !true;
RET = _6.0 & _6.0;
_5 = _7 as i16;
_8 = _7 == _7;
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
RET = !_6.0;
_3 = [127573490022603538665822353715775224643_i128,(-95148133363594205625069934999058290787_i128),(-169936899286121983792299223690886008426_i128),(-43881244238686817106406694676074279592_i128),(-104543138336866304821361602547843023869_i128),135546065680971474208147225352500179222_i128];
(*_10).0 = [1103213443_i32,2644533_i32,226809929_i32,(-1084618267_i32),2054958149_i32,1430590223_i32];
_13.0 = [122969730_i32,2145796943_i32,259825474_i32,463268834_i32,2022008852_i32,(-93123824_i32)];
(*_10).0 = [(-80982449_i32),(-1636581995_i32),(-1456396189_i32),(-971247526_i32),(-236904547_i32),(-846001743_i32)];
match _12 {
0 => bb8,
1 => bb6,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463463374607431768211372 => bb14,
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
_14 = 2621492935_u32 as f64;
(*_10).1.0 = _8 as isize;
(*_10).1.1 = 266253997_i32;
(*_10).0 = [(*_10).1.1,(*_10).1.1,_13.1.1,(*_10).1.1,_13.1.1,(*_10).1.1];
(*_10).1.0 = !_12;
_13.1.3 = _14;
(*_10).1.1 = RET as i32;
_10 = core::ptr::addr_of!(_13);
_6 = (RET,);
(*_10).2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_15 = core::ptr::addr_of_mut!(_8);
(*_10).2 = [_5,_5,_5,_5,_5,_5,_5,_5];
(*_10).1.3 = _14;
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(5_usize, 6_usize, Move(_6), 4_usize, Move(_4), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: [i128; 6],mut _6: [i128; 6],mut _7: [i128; 6],mut _8: [i128; 6],mut _9: [i128; 6],mut _10: [i128; 6],mut _11: [i128; 6],mut _12: [i128; 6],mut _13: [i128; 6],mut _14: i64,mut _15: i64,mut _16: [i128; 6]) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _17: char;
let _18: [i32; 6];
let _19: bool;
let _20: Adt49;
let _21: [i128; 6];
let _22: i8;
let _23: Adt57;
let _24: *mut (isize, *mut bool);
let _25: u32;
let _26: Adt51;
let _27: f64;
let _28: *mut [u32; 6];
let _29: Adt44;
let _30: isize;
let _31: bool;
let _32: Adt50;
let _33: Adt56;
let _34: u16;
let _35: (isize, i32, u64, f64);
let _36: usize;
let _37: i8;
let _38: &'static i16;
let _39: isize;
let _40: (i64,);
let _41: bool;
let _42: *mut (isize, *mut bool);
let _43: char;
let _44: i128;
let _45: ();
let _46: ();
{
_4 = [(-107944000985909319432617797378581406729_i128),101352640718795074675548162828468278987_i128,3572515526582650813461118058562597031_i128,157767565151023936887521002227792594237_i128,(-145806116593612398370837230589062157520_i128),8689063240500400527655605974342943775_i128];
RET = _9;
_9 = [(-5075646584277113577866252994608130661_i128),73461961466444001623016906791029963002_i128,(-2752336504560287352328670821208475614_i128),101813211384747117317036170892370904039_i128,(-118551984129042875080422573685000740459_i128),128530386630779311792699307164149915993_i128];
Call(_14 = fn7(_6, _7, _13, _12, _9, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [78285119750846089992158632752997556563_i128,(-144728325971060598717430635178059903655_i128),93218004272009745018911185969503833535_i128,52447167191261629600425376505147264346_i128,(-144735438763120663376414375701588171184_i128),90857358105805868197058940492601516162_i128];
Goto(bb2)
}
bb2 = {
_7 = [128913368735358207584710727494159491014_i128,(-109761888056988276364647981201304693973_i128),(-29253958649008121175168047703964444315_i128),(-105291578635552498551932288862339001307_i128),60034143742787259356085221131608551976_i128,154645172702954142051542470393096323563_i128];
Goto(bb3)
}
bb3 = {
_5 = [151075193411490355610067212878354397768_i128,82849222815697953628097155496488447596_i128,(-42305785987419855819815163238125968266_i128),(-57563131343000948077667927651149529434_i128),(-102421202086849436284818252467082125032_i128),148368712546465703479328220480543948792_i128];
_3 = [(-117674387967141722804154488504652032206_i128),936067677876765557653773351184977532_i128,(-117883623319143218067437687939793558008_i128),(-73032220358049020329765594055980759251_i128),137418281021918659720618109740815352025_i128,(-107844862740354015078410290170897989204_i128)];
Call(_16 = fn15(_13, _2, _4, _5, _13, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = _2;
_3 = [(-108730594040900587793701586931683018104_i128),(-107665649264777686770829474219505349694_i128),(-130143956462104474446806245016936193348_i128),68821326294183712200520476571251897316_i128,89334504730230939510099819912467256558_i128,(-111362332111307502798187747928935506711_i128)];
_3 = _9;
_8 = [134803434617547659474136700853720757794_i128,(-138042446972113100326112463208612826816_i128),(-81531476010668862716114107903098598463_i128),(-60409789712757708962279142892149007863_i128),76811450137982428474273616569229813387_i128,(-78238169166971056204422000283026140986_i128)];
Call(_5 = fn16(_1, RET, _2, _4, _16, _10, _9, _3, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = _10;
Goto(bb6)
}
bb6 = {
_16 = [(-66653689038115139432368767239923315190_i128),93750941518353995342470732850703879850_i128,(-35443476194641182615397275866305711450_i128),(-42321582281339524580960329184326155016_i128),159803024890483977425956820887138114989_i128,45992478735186524882956556846703409339_i128];
_4 = _10;
_8 = _7;
_15 = _14 >> _14;
Call(_16 = core::intrinsics::transmute(_5), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16 = _6;
_15 = 96_isize as i64;
_9 = [55012036552672723974961142245997478691_i128,97190302267372720988528240758042950559_i128,(-106169079267856767412934851985864025804_i128),51809864411611760679919160900079376733_i128,(-35077386261542481309786077188837155700_i128),(-56826559969920994146314574466650885357_i128)];
_8 = [21518077660339729327794344330318418050_i128,(-73131270024520153987525889799545080189_i128),(-21363805683910802505078942336274492343_i128),(-138052671831485874607594521591153955963_i128),107761429749628104418733843761038434682_i128,107774704485975194487433856036444143968_i128];
_18 = [1643010966_i32,1759704433_i32,1209566153_i32,374404887_i32,2040026119_i32,(-1767572027_i32)];
_12 = _3;
_18 = [1049631199_i32,(-2130800427_i32),860723633_i32,1957159963_i32,(-1514955641_i32),1101262941_i32];
_5 = [28296916141105579562072019145310746953_i128,131599634882608615436602182571355065912_i128,(-103122597198228859214241763664336295385_i128),82497568284737413139507674621955759154_i128,12530156327543915620219230362962948377_i128,93712805348811409568278519943106902247_i128];
_11 = _13;
_17 = '\u{f8dae}';
RET = [(-149046649083179350852248751550964349030_i128),(-168453645569759691485048832656681822673_i128),(-11652359305503257673593529514584143700_i128),(-98145094671248093061186662182282034133_i128),(-73830637016291651973839624416239491136_i128),(-137739541964999980431054560858935797913_i128)];
Goto(bb8)
}
bb8 = {
_17 = '\u{232a1}';
_1 = _4;
_15 = _14;
_7 = _16;
_15 = _14 - _14;
_10 = [(-107944129727197694196708391641483592548_i128),(-81985814474326493070922174245299183934_i128),(-85101305533044402689319626220403434382_i128),(-116273110828743133786396238963357107730_i128),(-155875286774990946361964527172932651000_i128),(-78604511095778914050826588728823471539_i128)];
_10 = [(-138783467152816168036730785797144083551_i128),(-50091700103908447172669771624324983681_i128),3469156277730659426909024426704375103_i128,(-157035730989639697209010041750908394994_i128),101560237738983182324850229567478790356_i128,(-61434035627799637593998553721603727024_i128)];
_19 = true | true;
_1 = _7;
_12 = [(-117557993603275258737418218533703895601_i128),(-108218358679665672292871313257470576477_i128),63820441552029010318061522760289926139_i128,76928355099642046500708546755171998379_i128,(-43638656487619210673954755507228725645_i128),(-132853433857252094273818292830748899798_i128)];
_1 = _5;
_15 = !_14;
_21 = [43088527151559618935537825013519202964_i128,114771613434168147408910648477592689732_i128,39417870047077635052716586018386292766_i128,67031636173587076132570078468405299776_i128,155267546692199816376823850965929346158_i128,(-52065726470992965597699766279065484772_i128)];
_16 = [(-142273007948986250835933613961339086599_i128),(-107097565796942767389987782755514453246_i128),(-161380256150373055995773517468606411591_i128),142609921759026039903949216948295345926_i128,(-75980015697520740380585425825351015644_i128),(-85875277112806876228778287373433043374_i128)];
_9 = [21666191189123398993139869918147995841_i128,(-133904234401519911961218438954548054863_i128),24746589923959088667400837089695091769_i128,66249032267482063704783101259474125662_i128,(-22122921228014500790319664284122934314_i128),(-37722878420841824537082952859923399174_i128)];
_1 = [161201751470075545015359870632113832120_i128,(-105215742938063233102236024059341015701_i128),112129954065549296204277935408970275930_i128,77400212471186843959959058877982048528_i128,45841658179170715307670585763731353040_i128,144916555851268403654489131958352385362_i128];
_12 = [(-45631313839099698985551278995620050357_i128),(-68543880076197793149636948271522167178_i128),(-50189569737191432902867604567520176182_i128),(-148352969086410821551057270522151085371_i128),86727021513185618606169893537067136657_i128,(-140127567301431707935572296190623064135_i128)];
_4 = _9;
_11 = [106128750208408331289434689725458640690_i128,(-132985514092819599717223590453066641063_i128),40897029567889364469978455891687093431_i128,132368569984603527224335196495366117494_i128,168298773474046924615545522302099975424_i128,110370197425731894584962904519793467200_i128];
_12 = [(-74211983377627182115292486590541169394_i128),73433010743577738766156209876523279751_i128,(-100991919505830489939338231136891910622_i128),(-57306458752466017987814733359914978633_i128),138862978640308875392524781451193872155_i128,(-51990874921732490080020277916017668361_i128)];
RET = _7;
_11 = [(-8672337273022887673852293931201254323_i128),(-113793085537907156634071900858658982402_i128),154896058177203479313287504984452907088_i128,159152711152607839822591232366968481459_i128,97399844421486059701830288145477768527_i128,3529064683664608709114045347190820558_i128];
_16 = [72473843897894810949405859728548903733_i128,73710204813010985038461053379757958637_i128,(-114886603430952136900642463902504120130_i128),162800010874092273260065145049711704824_i128,74252586778238800014533572891938199633_i128,(-161148258879655379776649028850809007956_i128)];
Call(_10 = fn17(_12, _2, _1, _12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = [(-25006080585749739707754345273162880508_i128),145657249353307964433191127280839699797_i128,(-68133777257980000443863838993558466097_i128),160311884954851461458728316030065075124_i128,7136776699255810446051496749118588190_i128,(-161161868344129550727233122445033515501_i128)];
_16 = RET;
_18 = [460239748_i32,1597550884_i32,(-1592213845_i32),151294530_i32,(-1733774087_i32),1066255781_i32];
_8 = _13;
_19 = false;
_7 = [(-129622740249973780424629432060598503103_i128),117808445875398591687951128415882590662_i128,75495377183208967142246060852747720455_i128,(-23179920868694569386057405259232586659_i128),(-59312835880757832098075386260434490124_i128),(-71995678218190475902909330871817049792_i128)];
_14 = _15 & _15;
_7 = _4;
_5 = [(-123919662598165533366059509087510257058_i128),74633978196707318521990413450514952618_i128,(-149849953104345039205026696159692370226_i128),(-81617465168910275166366786764222838559_i128),(-121769187042536296898124711128180173233_i128),(-148192485440654548703006128825974845526_i128)];
_13 = [(-10228536436785868945052935317120699272_i128),66544549430176344295234700289587163755_i128,71023536486254380872403940579732499149_i128,107869896362071839764579561708127109121_i128,(-2613710034273890860372701365906529463_i128),(-37013552511367684802240111687130052346_i128)];
_18 = [(-59365916_i32),1034196501_i32,1448897996_i32,953763009_i32,(-91415483_i32),(-161672530_i32)];
RET = [(-151357882751915654395608655743114463940_i128),22973598516146850746938116887827588324_i128,27628099466461250867963051525894444749_i128,(-162690647326444007165452940606649227372_i128),(-116655956170635584567401337343075826081_i128),650421602444437782351882846145030053_i128];
_11 = [(-27852768690545191370979450556193001140_i128),75735953432796923631449890363140503934_i128,157151969208678002031162919735238236840_i128,(-135995510106074643328123844610149360180_i128),93640431575424409608020971476992332239_i128,29704721313293232266375130192941074629_i128];
_15 = _14 - _14;
_14 = _15;
_17 = '\u{9eef0}';
_2 = [119771987738843530314014890910535513966_i128,129641715251178882357171459988922945973_i128,113886528491314181979668400857403077554_i128,83917259897626079961707923029438179633_i128,150939707629392309494522324306468031984_i128,166226503243929203589650150174789971707_i128];
_6 = [74225660070484351419713680619664659789_i128,103336145383847182780405829581906647643_i128,(-169845878847149625428071830484897628942_i128),122327311654618865389900306095464839232_i128,(-68954468912547920375420910678265597028_i128),68204824673975357412689289750152809189_i128];
_7 = [72347882935409742322634364093161014054_i128,127743499662651625511847627071371357508_i128,129471145752350378430835438369834680981_i128,(-88407703697224493412477284142295904493_i128),(-27158000137881121125862611452025887278_i128),(-41914539702152106373192283498467351566_i128)];
RET = [11253421121497945878140584875301013889_i128,69891168878077583646251351519560952424_i128,82687124913604167920234003657182516428_i128,(-150186350514479896988962695502781148849_i128),(-109920950997025379985562504744590525355_i128),(-156907974280603098928415618556602978529_i128)];
_25 = 1510530012_u32 * 1795693914_u32;
_5 = [(-132468369324834063830650981834123934509_i128),(-8796437323981084342330844140860723368_i128),(-59792599220989221427059132761163714608_i128),70287404192283188675172814466191122316_i128,60136447889886682723961200318144977068_i128,89150636724208362039313453883158436784_i128];
Goto(bb10)
}
bb10 = {
_27 = (-9223372036854775808_isize) as f64;
_25 = 3967034643_u32 ^ 1397906063_u32;
_6 = [(-8810201743971622350675006881450435602_i128),136175406261848535321620402495720530165_i128,(-137178137746954535064761837157066411426_i128),161584193369107409818042264064141781506_i128,(-124097861354927405315518672815862009382_i128),156523521094325895759283666302790066390_i128];
_30 = _15 as isize;
Goto(bb11)
}
bb11 = {
_21 = [(-153822121772726494278784966899679999612_i128),(-74937068253327197750788892982614979332_i128),130955192231917038876257554258697012974_i128,133003839052863199077039466182783206968_i128,19418216230835251186233039042612079841_i128,(-12714028750164567311257449087241373537_i128)];
_2 = _4;
Goto(bb12)
}
bb12 = {
_22 = (-88_i8);
_30 = 215_u8 as isize;
_5 = [105852062891958256769923044594922796919_i128,98455672684293110394107708871281349562_i128,37047444464492016106070646465593859339_i128,(-106831288763622362555722981211124867600_i128),(-105437007892437635166943617644690425756_i128),(-29117746533180168960392307941756229663_i128)];
RET = [154584322907877419824010139159279540713_i128,58537431105858481185230745629745287889_i128,(-54111884390415405323430459701456084446_i128),84511658322913880239996272231737025591_i128,(-159782886948297865607080744327925707086_i128),67987350895957375385393331454349388092_i128];
_19 = !true;
_14 = _15;
_33.fld6 = [45312_u16,21682_u16];
_8 = _21;
_10 = _4;
_10 = [120577780741595425329867386143036340753_i128,78973025292972930405767356265351619805_i128,30519529903587698028639801297265416088_i128,(-108874433918825627408094661412417689698_i128),32967559752044973310118228913368160876_i128,151468847926928902939214790944224924330_i128];
_33.fld0 = core::ptr::addr_of!(_34);
_34 = 35983_u16 >> _15;
_18 = [(-12948173_i32),1598141211_i32,(-1770163044_i32),(-1226995751_i32),320101519_i32,(-1538511943_i32)];
_8 = [53700472674485205727227102153096580883_i128,91644630131752589808596079832595783434_i128,(-78617386453950629680456904206014340275_i128),98322411416926773261002133643335908189_i128,146674243744994145647165049764757543875_i128,(-88744881087386861286216939543567635399_i128)];
_7 = _1;
_16 = [16701192942238966149907961338437979132_i128,(-113066712050505182942527483883264299808_i128),70250493899137918883810550502374764933_i128,3184205223234515881141004544824116212_i128,(-149931444148807523759156670124243021541_i128),(-30978559859753123020045076115749552484_i128)];
_15 = _14 - _14;
_19 = true;
Goto(bb13)
}
bb13 = {
_22 = !(-63_i8);
_7 = _5;
_33.fld0 = core::ptr::addr_of!(_34);
_33.fld3 = _22 + _22;
_34 = 143415945528881622815014791688675336267_i128 as u16;
_35.0 = 16455014256113354602_usize as isize;
_35.0 = -_30;
_8 = [108729803656121741707323331485827466892_i128,(-60827428540949378457783174845480270582_i128),(-48269952353274656296783129515387441629_i128),87548687283342249017403442276778544997_i128,72134235106554548484563712929336622725_i128,(-44913626694684169731239441397726720707_i128)];
Call(_36 = core::intrinsics::bswap(6_usize), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_4 = [252937918053718900890703076496748753_i128,61493978871436223226125243438094350161_i128,17958979183341653019163009181754490573_i128,(-148507109099721593115738125901736695689_i128),42202801863977179989720106422694231699_i128,(-145561742650877008373776306672238516051_i128)];
_33.fld5 = !48_u8;
_10 = _4;
_18 = [1162327691_i32,913418909_i32,159647732_i32,374859987_i32,966956995_i32,(-235840460_i32)];
_35.1 = 0_usize as i32;
_19 = _15 != _14;
_35.1 = (-1379707040_i32);
_35.3 = -_27;
_17 = '\u{a7985}';
_18 = [_35.1,_35.1,_35.1,_35.1,_35.1,_35.1];
_39 = _30 & _35.0;
_9 = _5;
_17 = '\u{180f5}';
_41 = _19;
_36 = !175318415630661240_usize;
_37 = _33.fld3 ^ _22;
_1 = [(-61011067133866865624094660579243214669_i128),(-3085622459025207821683266025373746670_i128),48254691894273062391506283336077193718_i128,(-36877314988883873173054893827597325980_i128),(-151076184052339243329262361265407381626_i128),(-92031544725334267742377877737473576737_i128)];
_33.fld0 = core::ptr::addr_of!(_34);
_14 = -_15;
_6 = [(-23015082338584891221193079658251493239_i128),106353396655413300883163107507825461786_i128,(-156399479625283217533307717142418120323_i128),152737857431691646200890934164399110975_i128,10143304242430674841126267806347059338_i128,(-168100398175813600027884392960885077621_i128)];
_15 = _14;
_35 = (_39, 1066825340_i32, 3543321596848671806_u64, _27);
_21 = [56603539234880021323996531726875742651_i128,(-122788917317204302502333384748645390576_i128),58453917654392415296839494819285730935_i128,(-87361574605164241344153968461392562644_i128),(-6807435006573080067193309059325841217_i128),(-35410228424634489833875380944789063530_i128)];
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(6_usize, 9_usize, Move(_9), 10_usize, Move(_10), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(6_usize, 3_usize, Move(_3), 36_usize, Move(_36), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(6_usize, 41_usize, Move(_41), 16_usize, Move(_16), 34_usize, Move(_34), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(6_usize, 37_usize, Move(_37), 2_usize, Move(_2), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: [i128; 6],mut _6: [i128; 6]) -> i64 {
mir! {
type RET = i64;
let _7: (i64,);
let _8: u8;
let _9: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _10: i64;
let _11: f32;
let _12: [i16; 8];
let _13: isize;
let _14: *mut *mut *const (isize, *mut bool);
let _15: [u16; 2];
let _16: u8;
let _17: f32;
let _18: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _19: *const i128;
let _20: i64;
let _21: Adt49;
let _22: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _23: f32;
let _24: i32;
let _25: [u16; 2];
let _26: usize;
let _27: *const (isize, *mut bool);
let _28: (*mut bool, u32, *mut bool, char);
let _29: (*mut bool, u32, *mut bool, char);
let _30: u64;
let _31: [i32; 6];
let _32: *mut (isize, *mut bool);
let _33: isize;
let _34: (*const i128, (f32, (isize, i32, u64, f64)), *const i128);
let _35: isize;
let _36: (u32,);
let _37: f64;
let _38: [i32; 6];
let _39: f32;
let _40: isize;
let _41: ();
let _42: ();
{
RET = !(-3800315691440251500_i64);
_4 = [(-61673504410938081850240563871823254326_i128),71671784841315666763928158409730036629_i128,(-151269359069950660915943831128445111771_i128),161189367104415712852359550658250250970_i128,(-15834031862162575491507966515143137651_i128),30600201526266090289982027009645059121_i128];
_4 = [17433918352361339057244913195433978911_i128,168152845415648640496796283741956618723_i128,163121944338098652622802999987460958960_i128,(-112195902920415180935503231582332079212_i128),(-19501747928952263544463097965252256651_i128),149313201245636837544037827903252505370_i128];
_3 = [135655915939006791444603956018971345454_i128,(-89725707283346625996572649971562025330_i128),58095391666699448080734890975553303723_i128,11526572296793567064572835255193450496_i128,100954035375141251740681234942727301229_i128,37893478204380362085283525175793227694_i128];
RET = (-2983744635751741266_i64);
_7.0 = !RET;
_7.0 = '\u{ab03d}' as i64;
_5 = [43645517282679419602802703297960559360_i128,146705629033946478459010181450664576226_i128,(-53531437887862987524487282119897601571_i128),79281938155204142251824799121110494316_i128,(-3142393638034730284221584596048436995_i128),126453893976046952739040774551747335535_i128];
RET = _7.0;
_5 = [(-33237880115208802112994518426725734930_i128),(-77231029482032917400874411169103323753_i128),(-6341472661785881958214488281511727485_i128),(-34543795673917969005095586522670248904_i128),(-119115778715799481123669595558997832354_i128),(-71279064546410655458274620469220378753_i128)];
Goto(bb1)
}
bb1 = {
_7 = (RET,);
_4 = _3;
_8 = !137_u8;
_7.0 = RET * RET;
_9.1.1 = _7.0 as i32;
Goto(bb2)
}
bb2 = {
_3 = _4;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.1.3 = 3616006694_u32 as f64;
_9.1.0 = (-9223372036854775808_isize);
_6 = [96507427332729513956065047097720661198_i128,74340520243478773078770272640981277328_i128,(-87673692854307252679006501563685391669_i128),168365590084404051304901291707744720140_i128,(-7367304426030709374522298237519582228_i128),(-85544932340797970377117603098297431856_i128)];
_1 = [119695979185817821934926593361591966811_i128,26038431443314421954892808850721987745_i128,27716965702154289986502887582781798307_i128,155632076495680138824324128828681021546_i128,(-52997882154653310962376887259430031445_i128),(-77334851294736722282216489544385507394_i128)];
_7 = (RET,);
_9.1.1 = 1455425862_u32 as i32;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_5 = _3;
_1 = _2;
_2 = [160513121569327499644551819211532463003_i128,150967506635368452116259044477882019004_i128,8006496411002326542934975583574800784_i128,65599609009426371890149551081240567737_i128,(-6443566805665170154470541108552793387_i128),(-105915852529793997622971140308313526047_i128)];
Goto(bb3)
}
bb3 = {
_8 = (-62233008740311130546120794280824294524_i128) as u8;
_10 = _7.0;
_12 = [(-1770_i16),(-8006_i16),24205_i16,(-21416_i16),11755_i16,17755_i16,28225_i16,31204_i16];
_9.1.1 = (-976003704_i32) << _8;
_7.0 = _10 ^ RET;
_9.1.2 = !10341493968428081854_u64;
_8 = (-13478_i16) as u8;
_3 = _6;
_15 = [12724_u16,44127_u16];
_6 = _2;
_4 = _1;
_16 = !_8;
_9.2 = _12;
_16 = !_8;
Goto(bb4)
}
bb4 = {
_11 = 77_i8 as f32;
match _9.1.0 {
0 => bb5,
1 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb5 = {
_8 = (-62233008740311130546120794280824294524_i128) as u8;
_10 = _7.0;
_12 = [(-1770_i16),(-8006_i16),24205_i16,(-21416_i16),11755_i16,17755_i16,28225_i16,31204_i16];
_9.1.1 = (-976003704_i32) << _8;
_7.0 = _10 ^ RET;
_9.1.2 = !10341493968428081854_u64;
_8 = (-13478_i16) as u8;
_3 = _6;
_15 = [12724_u16,44127_u16];
_6 = _2;
_4 = _1;
_16 = !_8;
_9.2 = _12;
_16 = !_8;
Goto(bb4)
}
bb6 = {
_3 = _4;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.1.3 = 3616006694_u32 as f64;
_9.1.0 = (-9223372036854775808_isize);
_6 = [96507427332729513956065047097720661198_i128,74340520243478773078770272640981277328_i128,(-87673692854307252679006501563685391669_i128),168365590084404051304901291707744720140_i128,(-7367304426030709374522298237519582228_i128),(-85544932340797970377117603098297431856_i128)];
_1 = [119695979185817821934926593361591966811_i128,26038431443314421954892808850721987745_i128,27716965702154289986502887582781798307_i128,155632076495680138824324128828681021546_i128,(-52997882154653310962376887259430031445_i128),(-77334851294736722282216489544385507394_i128)];
_7 = (RET,);
_9.1.1 = 1455425862_u32 as i32;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_5 = _3;
_1 = _2;
_2 = [160513121569327499644551819211532463003_i128,150967506635368452116259044477882019004_i128,8006496411002326542934975583574800784_i128,65599609009426371890149551081240567737_i128,(-6443566805665170154470541108552793387_i128),(-105915852529793997622971140308313526047_i128)];
Goto(bb3)
}
bb7 = {
_7 = (RET,);
_4 = _3;
_8 = !137_u8;
_7.0 = RET * RET;
_9.1.1 = _7.0 as i32;
Goto(bb2)
}
bb8 = {
_9.1.0 = (-9223372036854775808_isize) << _16;
_13 = _9.1.0 * _9.1.0;
_3 = [117907139017898119896490942266193561753_i128,(-13482578701233208906451585352539900202_i128),105561380376732245935066382144008400914_i128,153837931652290530362223128384774877484_i128,(-88182468654765897924286641000255187094_i128),61100984490027623702512975236297569756_i128];
_18.2 = [17193_i16,30613_i16,(-1194_i16),(-14580_i16),(-29095_i16),(-9087_i16),10510_i16,(-9038_i16)];
_8 = 141177259754123322304662538692962435853_i128 as u8;
Goto(bb9)
}
bb9 = {
_1 = [92375118370329533995543495114304659330_i128,148511095286292577357715795031030625080_i128,(-102350132445275287951527894165365043089_i128),106336540888654619071513247879665512041_i128,161341674613017641758133438191216181297_i128,147634645674369084508453912172156586912_i128];
_18.1.1 = 69_i8 as i32;
_9.1.3 = (-126_i8) as f64;
_9.2 = _18.2;
_12 = [(-3520_i16),7706_i16,12760_i16,(-28976_i16),14225_i16,(-14799_i16),(-29801_i16),25822_i16];
_18.1.2 = _9.1.2;
_7.0 = RET & RET;
_22.3 = 22208_i16 as i64;
_18.1.2 = !_9.1.2;
_1 = [(-90082923555768957031769843948995899394_i128),(-146484606245214415980349969289933283121_i128),165543573602484235325475832045552225244_i128,(-106280893696133508578305736794391347874_i128),36067876796626702413539309758335513978_i128,(-137170846621703097378797572723948905806_i128)];
_17 = _11 + _11;
_18.1.0 = -_13;
_1 = [(-3163094414069179512612250979825241990_i128),(-110664161043751357776075622877374745569_i128),169472471536113880634997115918972370162_i128,81026342143758187825587844656462119771_i128,(-139820971246697359270123889893895326651_i128),73182075724663003309064336369054302982_i128];
_24 = _9.1.1;
RET = 55295_u16 as i64;
_18.2 = [13034_i16,(-18566_i16),(-7407_i16),(-13571_i16),9831_i16,8703_i16,(-612_i16),6017_i16];
_12 = [23226_i16,(-19962_i16),(-26656_i16),29383_i16,24718_i16,(-18639_i16),14360_i16,16653_i16];
_18.0 = _9.0;
_26 = 117869557539842431880310669215405036538_i128 as usize;
_3 = [68430244189520956250309080504510417482_i128,(-55800014113288778477584215426481332087_i128),39758021376557747549364975701508317572_i128,(-17990171575714689377337454250667404672_i128),(-1083348284988001683630162757458458325_i128),104021761505692702308319608707052382117_i128];
_22.2.0 = _18.0;
_15 = [51685_u16,45633_u16];
_9.1.3 = _18.1.2 as f64;
_25 = _15;
_18 = (_22.2.0, _9.1, _9.2);
Goto(bb10)
}
bb10 = {
_22.2 = _18;
_11 = _17 * _17;
RET = !_7.0;
_9.1 = (_13, _22.2.1.1, _18.1.2, _18.1.3);
_22.0 = 58678_u16;
_2 = [49574275927336811379788107799953228633_i128,32399848817267828881370626690738450093_i128,(-151182746474770331047477150444391013397_i128),31411552330952664544041712026272593151_i128,132226145567482519410461364948122534471_i128,12006429446476870233279563785938761552_i128];
_23 = -_11;
_9.1.0 = _22.2.1.0;
_18.1.3 = _9.1.3 - _22.2.1.3;
_18.1.0 = _13 >> RET;
RET = _22.3 - _7.0;
_12 = _9.2;
_15 = [_22.0,_22.0];
_3 = _2;
_20 = _22.3;
_22.2 = _9;
_18.1.3 = _22.2.1.3;
_28.3 = '\u{5e2c5}';
_9 = (_18.0, _22.2.1, _22.2.2);
_22.2.2 = _18.2;
_22.2.0 = [_24,_9.1.1,_24,_24,_9.1.1,_18.1.1];
RET = _7.0;
_29.1 = 408152764_u32 >> _7.0;
_3 = [(-90918779754813617811242414810598918459_i128),30796728813555384908692770328306123238_i128,(-15895596396731599069439660300467542022_i128),(-101508752109069005100794730871393104092_i128),(-69497266209027745220420477275879061792_i128),152808972593884250995485705425164217384_i128];
_28.1 = 60864477962219223542866484066461765241_u128 as u32;
_22.2.1 = (_18.1.0, _18.1.1, _9.1.2, _18.1.3);
Call(_29.1 = fn8(_18.2, _18.1, _13, _13, _6, _9.1.0, _18.1.0, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1 = _6;
Goto(bb12)
}
bb12 = {
_9.1.1 = _22.0 as i32;
_11 = _22.2.1.0 as f32;
_18.0 = [_22.2.1.1,_24,_24,_18.1.1,_9.1.1,_24];
_31 = [_24,_18.1.1,_24,_18.1.1,_22.2.1.1,_9.1.1];
_28.1 = !_29.1;
_1 = [(-103756362612117141716927074517852724455_i128),(-145150413938871285967540087690023972319_i128),(-4973066179322715262852348030215474367_i128),5423463492487169894703245628348920755_i128,74753374234031213978238197607349575081_i128,162383413521185441686442264674493081012_i128];
_29.1 = 94_i8 as u32;
_9.1.0 = _18.1.0 ^ _22.2.1.0;
_17 = _23 - _11;
_23 = _16 as f32;
_22.2.1 = (_9.1.0, _24, _18.1.2, _18.1.3);
_29.3 = _28.3;
_33 = _22.2.1.0;
_33 = _22.2.1.0;
_22 = (19233_u16, 43074_u16, _9, _7.0);
_24 = -_22.2.1.1;
_28.1 = _22.1 as u32;
Goto(bb13)
}
bb13 = {
_9.1.0 = _22.2.1.0 ^ _18.1.0;
_29.3 = _28.3;
_11 = _17;
_22.3 = _22.2.1.1 as i64;
_7 = (_22.3,);
_34.1.1.3 = _18.1.3 - _22.2.1.3;
_9.0 = [_18.1.1,_9.1.1,_18.1.1,_22.2.1.1,_18.1.1,_22.2.1.1];
_22.3 = RET + _20;
_16 = !_8;
_9.1.1 = _22.2.1.1 & _18.1.1;
_4 = [11695289842770982990614826961326144016_i128,(-19698357637756358149962543668943138239_i128),78206581515063864874088912730876208639_i128,(-52768639711638101205408361732730510039_i128),29315195917701812334585986990230902029_i128,3932827104449745695197707013122246163_i128];
_7 = (_22.3,);
_31 = [_24,_9.1.1,_18.1.1,_18.1.1,_9.1.1,_18.1.1];
_17 = _11;
_9.2 = _12;
_3 = [(-40338834984066440445773639293117578685_i128),(-129432311721049432582565993295100999409_i128),(-161426071388413836954556523386928660059_i128),(-168800841874258512628612573707903426365_i128),149807434552309777094961610185787699357_i128,52461077969656853252749231135604758532_i128];
RET = _22.3;
_9.1.3 = -_22.2.1.3;
_16 = _8 - _8;
_29.3 = _28.3;
_34.1.1 = (_33, _9.1.1, _18.1.2, _18.1.3);
_9.1.3 = _34.1.1.1 as f64;
_30 = RET as u64;
_22.2.1.3 = -_9.1.3;
_22.0 = (-46925071734397289373798549464592649468_i128) as u16;
_37 = _9.1.3;
_34.1.1.3 = _37 + _9.1.3;
_10 = _20 ^ RET;
match _22.1 {
0 => bb11,
1 => bb9,
2 => bb8,
3 => bb4,
4 => bb14,
43074 => bb16,
_ => bb15
}
}
bb14 = {
_22.2 = _18;
_11 = _17 * _17;
RET = !_7.0;
_9.1 = (_13, _22.2.1.1, _18.1.2, _18.1.3);
_22.0 = 58678_u16;
_2 = [49574275927336811379788107799953228633_i128,32399848817267828881370626690738450093_i128,(-151182746474770331047477150444391013397_i128),31411552330952664544041712026272593151_i128,132226145567482519410461364948122534471_i128,12006429446476870233279563785938761552_i128];
_23 = -_11;
_9.1.0 = _22.2.1.0;
_18.1.3 = _9.1.3 - _22.2.1.3;
_18.1.0 = _13 >> RET;
RET = _22.3 - _7.0;
_12 = _9.2;
_15 = [_22.0,_22.0];
_3 = _2;
_20 = _22.3;
_22.2 = _9;
_18.1.3 = _22.2.1.3;
_28.3 = '\u{5e2c5}';
_9 = (_18.0, _22.2.1, _22.2.2);
_22.2.2 = _18.2;
_22.2.0 = [_24,_9.1.1,_24,_24,_9.1.1,_18.1.1];
RET = _7.0;
_29.1 = 408152764_u32 >> _7.0;
_3 = [(-90918779754813617811242414810598918459_i128),30796728813555384908692770328306123238_i128,(-15895596396731599069439660300467542022_i128),(-101508752109069005100794730871393104092_i128),(-69497266209027745220420477275879061792_i128),152808972593884250995485705425164217384_i128];
_28.1 = 60864477962219223542866484066461765241_u128 as u32;
_22.2.1 = (_18.1.0, _18.1.1, _9.1.2, _18.1.3);
Call(_29.1 = fn8(_18.2, _18.1, _13, _13, _6, _9.1.0, _18.1.0, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_7 = (RET,);
_4 = _3;
_8 = !137_u8;
_7.0 = RET * RET;
_9.1.1 = _7.0 as i32;
Goto(bb2)
}
bb16 = {
_2 = [(-33161427393674956950809047562680225472_i128),54433145678879544582597703788759593123_i128,53773900631197598991199737143755716381_i128,154089264243787605725412236465779873819_i128,67429736795644567928833112965158920738_i128,(-1126967867739074880336719344776663966_i128)];
_18.0 = _31;
_9 = _22.2;
_18.2 = [18936_i16,(-13673_i16),(-30586_i16),29127_i16,6351_i16,(-22192_i16),(-29167_i16),(-31091_i16)];
_36 = (_28.1,);
_26 = 12842889373615490974_usize;
_17 = (-22119959868049469728810058092168321631_i128) as f32;
_20 = _10;
_35 = _33;
_5 = [152959462110862543401642835214541308393_i128,30315201568789225844499552552844391948_i128,19057324637191735597997413331337954662_i128,102037664751328844528799746003290255198_i128,69490038367029313733295579344610946926_i128,(-118741578112298785536876515424581833928_i128)];
_6 = [(-21083127523256376572380373312514430279_i128),(-169734813573397422457197185917027209618_i128),(-36790504332050813819746028404139486167_i128),(-60953753913140098121719624691462397930_i128),(-51562450975462099168522252812351709222_i128),101686122754021932694189220106658759216_i128];
_18 = (_31, _22.2.1, _22.2.2);
_26 = !9317675711784365482_usize;
_3 = [19311009486875180213814477111315145290_i128,80014822165656476739762496040046838305_i128,121005585601780474589172180881327741722_i128,(-92616916535405972832668018874702410500_i128),(-29459368801696206162648411344263178974_i128),5163547999614658502679938335705482850_i128];
_40 = _35;
_30 = !_22.2.1.2;
RET = _10 - _7.0;
_4 = _5;
_18.1.0 = _35 | _35;
_4 = _2;
_18.1 = (_22.2.1.0, _22.2.1.1, _22.2.1.2, _34.1.1.3);
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(7_usize, 31_usize, Move(_31), 26_usize, Move(_26), 30_usize, Move(_30), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(7_usize, 35_usize, Move(_35), 15_usize, Move(_15), 12_usize, Move(_12), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(7_usize, 10_usize, Move(_10), 2_usize, Move(_2), 13_usize, Move(_13), 42_usize, _42), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i16; 8],mut _2: (isize, i32, u64, f64),mut _3: isize,mut _4: isize,mut _5: [i128; 6],mut _6: isize,mut _7: isize,mut _8: [i128; 6]) -> u32 {
mir! {
type RET = u32;
let _9: [i32; 6];
let _10: (isize, i32, u64, f64);
let _11: char;
let _12: [i64; 4];
let _13: [u32; 6];
let _14: char;
let _15: Adt44;
let _16: Adt50;
let _17: f64;
let _18: [i32; 6];
let _19: *const (isize, *mut bool);
let _20: &'static i16;
let _21: (f32, (isize, i32, u64, f64));
let _22: i128;
let _23: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _24: u64;
let _25: (isize, *mut bool);
let _26: f32;
let _27: [i64; 4];
let _28: (isize, i32, u64, f64);
let _29: isize;
let _30: Adt48;
let _31: u128;
let _32: i8;
let _33: f64;
let _34: [i16; 8];
let _35: Adt47;
let _36: f64;
let _37: isize;
let _38: Adt54;
let _39: ();
let _40: ();
{
RET = !313250213_u32;
_4 = _3;
_2.0 = 5260009359640566036_i64 as isize;
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_5 = [45731422304945730395937185370114050497_i128,(-33395731644386775868407134464852665606_i128),(-102321264114782298545876602763971214589_i128),3005821440448480322637123685044860723_i128,103196211251150260652025736559985468087_i128,(-157791895564911293460916829562938072511_i128)];
_2.0 = -_3;
_4 = !_2.0;
Goto(bb1)
}
bb1 = {
_10.3 = _2.3;
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_10.0 = _7 | _6;
_2.2 = true as u64;
_2.1 = (-1471365861_i32) - (-643158249_i32);
_8 = _5;
_10.0 = -_3;
_1 = [(-28289_i16),(-22765_i16),(-19325_i16),17617_i16,2293_i16,(-23087_i16),(-10332_i16),30762_i16];
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_2.2 = 7478295627728384348_u64 ^ 9364036408572934408_u64;
_2.3 = 19_u8 as f64;
_5 = [161200594133789446576661625984420759388_i128,85092276281001901462187411010522780205_i128,(-154790849829772792609418760365735533674_i128),55820211581824542861159260934290420795_i128,(-64017344693928739690576111603122683007_i128),108332446160319615121399809883934118761_i128];
_10 = _2;
_2.3 = RET as f64;
Goto(bb2)
}
bb2 = {
_12 = [(-5712759924102420551_i64),(-569317144307438883_i64),(-6563325453452842306_i64),7119084878613102093_i64];
_12 = [1714611862641898946_i64,(-8538481358122594646_i64),4959142688602708629_i64,7078831444996595779_i64];
_10.3 = _6 as f64;
_2.0 = !_6;
_2 = _10;
Call(_2.1 = fn9(_5, _10.1, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _10.1 as isize;
_10.2 = 124_u8 as u64;
_13 = [RET,RET,RET,RET,RET,RET];
Call(_7 = core::intrinsics::bswap(_2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = -_7;
_10.3 = -_2.3;
_9 = [_10.1,_10.1,_2.1,_2.1,_2.1,_10.1];
_8 = [48913988302070985855333854635521640315_i128,16764410592517945961923220092156018467_i128,(-107452002359347450381180664211322187841_i128),(-89918087892773141670232880407636696092_i128),(-18789803164019102184995719132177031044_i128),(-130860453612562855653013919812772357313_i128)];
_13 = [RET,RET,RET,RET,RET,RET];
_8 = [54396110056434678465367904143695872524_i128,(-59912541979540640712558642048215811483_i128),(-123471923882298932486899445027625799877_i128),50972276748958132027735715095838960888_i128,(-20775455630817866422692641540445056787_i128),117509315809698842809541290997312049974_i128];
RET = 3928793701_u32;
_10 = (_7, _2.1, _2.2, _2.3);
_2.1 = !_10.1;
_12 = [(-2433420127347574341_i64),5114567949844569729_i64,5490182083349069794_i64,4463754190400787412_i64];
_8 = [63299408897926339797111280770575346516_i128,43113314236369258063132732573311591583_i128,(-132099853025095178762087995196203401359_i128),(-48330036599288371957411998502374096900_i128),5771511475290436676445553005014461591_i128,123365673204394139940486014262756243516_i128];
_2.3 = _10.3;
_10 = (_2.0, _2.1, _2.2, _2.3);
_1 = [(-7874_i16),(-20630_i16),11696_i16,20347_i16,18705_i16,(-32551_i16),12471_i16,13109_i16];
_3 = -_7;
_10 = (_2.0, _2.1, _2.2, _2.3);
_6 = -_3;
_10.2 = _2.2;
_9 = [_2.1,_10.1,_10.1,_2.1,_2.1,_2.1];
_2.3 = _10.3 * _10.3;
_14 = '\u{d83e0}';
_7 = !_6;
_2.2 = _10.2 * _10.2;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
3928793701 => bb7,
_ => bb6
}
}
bb5 = {
_4 = _10.1 as isize;
_10.2 = 124_u8 as u64;
_13 = [RET,RET,RET,RET,RET,RET];
Call(_7 = core::intrinsics::bswap(_2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_10.3 = _2.3;
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_10.0 = _7 | _6;
_2.2 = true as u64;
_2.1 = (-1471365861_i32) - (-643158249_i32);
_8 = _5;
_10.0 = -_3;
_1 = [(-28289_i16),(-22765_i16),(-19325_i16),17617_i16,2293_i16,(-23087_i16),(-10332_i16),30762_i16];
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_2.2 = 7478295627728384348_u64 ^ 9364036408572934408_u64;
_2.3 = 19_u8 as f64;
_5 = [161200594133789446576661625984420759388_i128,85092276281001901462187411010522780205_i128,(-154790849829772792609418760365735533674_i128),55820211581824542861159260934290420795_i128,(-64017344693928739690576111603122683007_i128),108332446160319615121399809883934118761_i128];
_10 = _2;
_2.3 = RET as f64;
Goto(bb2)
}
bb7 = {
_17 = _2.3 + _2.3;
_10.3 = _17 - _17;
_2 = (_6, _10.1, _10.2, _10.3);
_10.0 = 288003204111515143122830940546345953474_u128 as isize;
Goto(bb8)
}
bb8 = {
_8 = _5;
_10.2 = _2.2 - _2.2;
_6 = _7 & _2.0;
_11 = _14;
_10.0 = -_2.0;
_4 = 6697546135829863962_usize as isize;
_2.1 = 29214_u16 as i32;
_1 = [(-3797_i16),(-12993_i16),(-21972_i16),(-5141_i16),(-16082_i16),14849_i16,20910_i16,(-8302_i16)];
_3 = -_6;
_10 = (_7, _2.1, _2.2, _2.3);
_10.2 = _2.2 * _2.2;
_10 = (_6, _2.1, _2.2, _2.3);
_21.1.1 = _7 as i32;
_23.1 = (_6, _21.1.1, _2.2, _2.3);
_23 = (_9, _2, _1);
Call(_21.1.3 = fn13(_1, _10.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21.1.0 = _10.0 >> _21.1.1;
_2.3 = -_21.1.3;
_13 = [RET,RET,RET,RET,RET,RET];
_21.1.0 = _6;
_10.3 = _21.1.3;
_10.2 = _23.1.2 << _10.0;
_28 = (_6, _21.1.1, _10.2, _17);
_26 = 148484714098208243388824722148697113152_i128 as f32;
_5 = [(-44596440803760195226384688282320555045_i128),(-132127164970691022782680678207462264182_i128),6890595842643527301173121489582286566_i128,(-114906054107033598244623472872376769314_i128),(-76389062835317489942729098251578911593_i128),(-30079654130195158126894534605349156983_i128)];
_10.3 = _2.3;
_7 = -_2.0;
_1 = [(-11593_i16),(-20525_i16),16574_i16,(-6565_i16),23607_i16,11235_i16,20891_i16,20929_i16];
_21.0 = -_26;
_21.1.2 = 38713_u16 as u64;
_21.1 = _10;
_10.3 = _2.3 * _2.3;
_2.0 = _3;
_21.1.0 = !_3;
RET = 1789704046_u32 | 3172255124_u32;
_10.1 = _21.1.1;
_10.0 = _2.0 ^ _6;
_23.1.1 = 7119650616547545289_i64 as i32;
_3 = _6 ^ _10.0;
Goto(bb10)
}
bb10 = {
_21.1.3 = 222_u8 as f64;
_2.3 = 5255_i16 as f64;
_29 = !_10.0;
_25.0 = 26_u8 as isize;
_28.1 = _21.1.1;
_21.1.1 = _10.1 << _10.0;
_31 = 337838980066290024318808984078216140437_u128;
_28 = _10;
RET = _26 as u32;
_28 = (_29, _21.1.1, _21.1.2, _10.3);
_28.1 = 6648664605912934762_i64 as i32;
_26 = 1108169944558778455_i64 as f32;
_22 = !(-134299156588569934606216019837277833120_i128);
_21.1.1 = _21.0 as i32;
_7 = !_3;
RET = 2013863457_u32;
_21.1.0 = 121_i8 as isize;
RET = 1727433369_u32;
_2.2 = !_10.2;
_21.1.2 = _2.0 as u64;
match _31 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb8,
4 => bb11,
5 => bb12,
337838980066290024318808984078216140437 => bb14,
_ => bb13
}
}
bb11 = {
_4 = _10.1 as isize;
_10.2 = 124_u8 as u64;
_13 = [RET,RET,RET,RET,RET,RET];
Call(_7 = core::intrinsics::bswap(_2.0), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_10.3 = _2.3;
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_10.0 = _7 | _6;
_2.2 = true as u64;
_2.1 = (-1471365861_i32) - (-643158249_i32);
_8 = _5;
_10.0 = -_3;
_1 = [(-28289_i16),(-22765_i16),(-19325_i16),17617_i16,2293_i16,(-23087_i16),(-10332_i16),30762_i16];
_9 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_2.2 = 7478295627728384348_u64 ^ 9364036408572934408_u64;
_2.3 = 19_u8 as f64;
_5 = [161200594133789446576661625984420759388_i128,85092276281001901462187411010522780205_i128,(-154790849829772792609418760365735533674_i128),55820211581824542861159260934290420795_i128,(-64017344693928739690576111603122683007_i128),108332446160319615121399809883934118761_i128];
_10 = _2;
_2.3 = RET as f64;
Goto(bb2)
}
bb13 = {
_12 = [(-5712759924102420551_i64),(-569317144307438883_i64),(-6563325453452842306_i64),7119084878613102093_i64];
_12 = [1714611862641898946_i64,(-8538481358122594646_i64),4959142688602708629_i64,7078831444996595779_i64];
_10.3 = _6 as f64;
_2.0 = !_6;
_2 = _10;
Call(_2.1 = fn9(_5, _10.1, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_11 = _14;
_10.0 = _7 + _7;
_23.1.3 = _10.3;
_2.3 = _28.3 + _23.1.3;
_5 = [_22,_22,_22,_22,_22,_22];
_8 = [_22,_22,_22,_22,_22,_22];
_25.0 = _28.0;
_10.0 = RET as isize;
_27 = [2051059124247138170_i64,7863885372507618469_i64,(-177922630247698177_i64),(-3069127612651584066_i64)];
_22 = _14 as i128;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(8_usize, 1_usize, Move(_1), 11_usize, Move(_11), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(8_usize, 27_usize, Move(_27), 29_usize, Move(_29), 31_usize, Move(_31), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i128; 6],mut _2: i32,mut _3: [i16; 8],mut _4: isize) -> i32 {
mir! {
type RET = i32;
let _5: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _6: u16;
let _7: i32;
let _8: u64;
let _9: u8;
let _10: Adt53;
let _11: Adt54;
let _12: bool;
let _13: *const u16;
let _14: (u32,);
let _15: char;
let _16: Adt57;
let _17: bool;
let _18: (*mut bool, u32, *mut bool, char);
let _19: Adt46;
let _20: [char; 8];
let _21: [u32; 6];
let _22: (isize, *mut bool);
let _23: Adt44;
let _24: (u32,);
let _25: [u16; 2];
let _26: ();
let _27: ();
{
_5.2.1.2 = 11373667903198641241_u64;
_5.2.1.1 = _2 >> _4;
_1 = [100213111726241816161522134252794617814_i128,162356892701200301404541516451451241594_i128,(-44063584958764010052525417956675122280_i128),(-153930592586530270701795224497613578694_i128),(-168846933721836768718437891959577528751_i128),139725487759345550281940654724384260767_i128];
RET = 282648521702794272718020059835883569775_u128 as i32;
_5.2.1.2 = 13603302305931925861_u64 >> _5.2.1.1;
_5.1 = 40645_u16 ^ 49867_u16;
_7 = 167339116123667687083591463318495825317_i128 as i32;
Goto(bb1)
}
bb1 = {
_5.0 = _5.1 - _5.1;
_5.3 = _4 as i64;
_5.3 = !(-1748379907846449911_i64);
_5.1 = !_5.0;
_7 = 15496810846430097939_usize as i32;
_5.2.1.2 = 4017610303019521709_u64 >> _2;
Call(_6 = fn10(RET, RET, _4, _5.2.1.2, _5.1, _5.3, _5.1, _3, _5.2.1.2, _5.1, _5.2.1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.2.1.3 = _5.2.1.2 as f64;
_5.2.1.3 = _5.3 as f64;
_5.2.1.0 = !_4;
_5.0 = _5.2.1.1 as u16;
_9 = 57_u8 - 108_u8;
_5.2.2 = [(-12067_i16),16559_i16,28331_i16,(-28594_i16),22379_i16,(-23820_i16),4947_i16,7824_i16];
_6 = _5.2.1.1 as u16;
_5.2.1.2 = !5427029249675817666_u64;
Call(_6 = fn11(_5.2.1, _5.2.1.3, _3, _7, _5.2.1, _5.2.2, _4, _5.2.1.0, _5.2.1.0, _5.2.1, _5.2.1, RET, _1, _5.1, _5.2.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.2.1.0 = _4;
_12 = true;
_12 = !false;
_5.2.1.1 = -_7;
RET = '\u{91cf0}' as i32;
RET = _2;
_2 = !RET;
_8 = !_5.2.1.2;
_14 = (1960953503_u32,);
_3 = [(-23382_i16),15627_i16,9630_i16,(-12527_i16),(-25894_i16),3994_i16,7271_i16,(-28037_i16)];
_4 = -_5.2.1.0;
_5.2.0 = [RET,_2,RET,_2,_5.2.1.1,RET];
_8 = _5.2.1.2;
_13 = core::ptr::addr_of!(_5.0);
_6 = _5.1;
_6 = !(*_13);
_14 = (3826053172_u32,);
_15 = '\u{76f7c}';
_15 = '\u{1466a}';
_7 = -_2;
RET = _7;
_7 = _5.2.1.1 - RET;
Goto(bb4)
}
bb4 = {
_17 = _5.3 <= _5.3;
_6 = (*_13) + (*_13);
_6 = _5.1;
_3 = [(-12029_i16),32113_i16,(-32187_i16),(-25565_i16),24687_i16,(-3576_i16),(-617_i16),9727_i16];
_19 = Adt46::Variant1 { fld0: _5.3 };
_18.3 = _15;
_5.2.1.3 = (-11896_i16) as f64;
_18.1 = _14.0 - _14.0;
_18.2 = core::ptr::addr_of_mut!(_17);
SetDiscriminant(_19, 1);
_5.2.1.3 = _5.2.1.0 as f64;
_18.1 = !_14.0;
_20 = [_18.3,_15,_18.3,_18.3,_18.3,_15,_15,_15];
_5.1 = _5.2.1.0 as u16;
(*_13) = _5.3 as u16;
_21 = [_14.0,_18.1,_18.1,_18.1,_18.1,_18.1];
_18.2 = core::ptr::addr_of_mut!(_17);
_4 = _8 as isize;
_13 = core::ptr::addr_of!(_6);
_18.3 = _15;
_18.1 = _14.0;
_5.3 = _17 as i64;
_22.1 = _18.2;
match _14.0 {
3826053172 => bb6,
_ => bb5
}
}
bb5 = {
_5.0 = _5.1 - _5.1;
_5.3 = _4 as i64;
_5.3 = !(-1748379907846449911_i64);
_5.1 = !_5.0;
_7 = 15496810846430097939_usize as i32;
_5.2.1.2 = 4017610303019521709_u64 >> _2;
Call(_6 = fn10(RET, RET, _4, _5.2.1.2, _5.1, _5.3, _5.1, _3, _5.2.1.2, _5.1, _5.2.1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_7 = _5.2.1.3 as i32;
_18 = (_22.1, _14.0, _22.1, _15);
_15 = _18.3;
_17 = _12;
_22.1 = core::ptr::addr_of_mut!(_17);
match _18.1 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
3826053172 => bb13,
_ => bb12
}
}
bb7 = {
_5.0 = _5.1 - _5.1;
_5.3 = _4 as i64;
_5.3 = !(-1748379907846449911_i64);
_5.1 = !_5.0;
_7 = 15496810846430097939_usize as i32;
_5.2.1.2 = 4017610303019521709_u64 >> _2;
Call(_6 = fn10(RET, RET, _4, _5.2.1.2, _5.1, _5.3, _5.1, _3, _5.2.1.2, _5.1, _5.2.1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_17 = _5.3 <= _5.3;
_6 = (*_13) + (*_13);
_6 = _5.1;
_3 = [(-12029_i16),32113_i16,(-32187_i16),(-25565_i16),24687_i16,(-3576_i16),(-617_i16),9727_i16];
_19 = Adt46::Variant1 { fld0: _5.3 };
_18.3 = _15;
_5.2.1.3 = (-11896_i16) as f64;
_18.1 = _14.0 - _14.0;
_18.2 = core::ptr::addr_of_mut!(_17);
SetDiscriminant(_19, 1);
_5.2.1.3 = _5.2.1.0 as f64;
_18.1 = !_14.0;
_20 = [_18.3,_15,_18.3,_18.3,_18.3,_15,_15,_15];
_5.1 = _5.2.1.0 as u16;
(*_13) = _5.3 as u16;
_21 = [_14.0,_18.1,_18.1,_18.1,_18.1,_18.1];
_18.2 = core::ptr::addr_of_mut!(_17);
_4 = _8 as isize;
_13 = core::ptr::addr_of!(_6);
_18.3 = _15;
_18.1 = _14.0;
_5.3 = _17 as i64;
_22.1 = _18.2;
match _14.0 {
3826053172 => bb6,
_ => bb5
}
}
bb9 = {
_5.2.1.0 = _4;
_12 = true;
_12 = !false;
_5.2.1.1 = -_7;
RET = '\u{91cf0}' as i32;
RET = _2;
_2 = !RET;
_8 = !_5.2.1.2;
_14 = (1960953503_u32,);
_3 = [(-23382_i16),15627_i16,9630_i16,(-12527_i16),(-25894_i16),3994_i16,7271_i16,(-28037_i16)];
_4 = -_5.2.1.0;
_5.2.0 = [RET,_2,RET,_2,_5.2.1.1,RET];
_8 = _5.2.1.2;
_13 = core::ptr::addr_of!(_5.0);
_6 = _5.1;
_6 = !(*_13);
_14 = (3826053172_u32,);
_15 = '\u{76f7c}';
_15 = '\u{1466a}';
_7 = -_2;
RET = _7;
_7 = _5.2.1.1 - RET;
Goto(bb4)
}
bb10 = {
_5.2.1.3 = _5.2.1.2 as f64;
_5.2.1.3 = _5.3 as f64;
_5.2.1.0 = !_4;
_5.0 = _5.2.1.1 as u16;
_9 = 57_u8 - 108_u8;
_5.2.2 = [(-12067_i16),16559_i16,28331_i16,(-28594_i16),22379_i16,(-23820_i16),4947_i16,7824_i16];
_6 = _5.2.1.1 as u16;
_5.2.1.2 = !5427029249675817666_u64;
Call(_6 = fn11(_5.2.1, _5.2.1.3, _3, _7, _5.2.1, _5.2.2, _4, _5.2.1.0, _5.2.1.0, _5.2.1, _5.2.1, RET, _1, _5.1, _5.2.1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5.0 = _5.1 - _5.1;
_5.3 = _4 as i64;
_5.3 = !(-1748379907846449911_i64);
_5.1 = !_5.0;
_7 = 15496810846430097939_usize as i32;
_5.2.1.2 = 4017610303019521709_u64 >> _2;
Call(_6 = fn10(RET, RET, _4, _5.2.1.2, _5.1, _5.3, _5.1, _3, _5.2.1.2, _5.1, _5.2.1.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_5.2.1.0 = _7 as isize;
_17 = _12;
_3 = [23259_i16,(-27746_i16),9892_i16,(-3337_i16),17514_i16,(-21583_i16),10083_i16,19459_i16];
_5.3 = 8555933729829707144_i64 & (-8159994052125700299_i64);
place!(Field::<i64>(Variant(_19, 1), 0)) = !_5.3;
_5.0 = !(*_13);
_18.0 = _18.2;
SetDiscriminant(_19, 1);
_22.0 = -_5.2.1.0;
_7 = RET;
_18.3 = _15;
_5.0 = 92_i8 as u16;
_18 = (_22.1, _14.0, _22.1, _15);
_5.2.2 = [22444_i16,(-17851_i16),16121_i16,11686_i16,20726_i16,20717_i16,27441_i16,(-23153_i16)];
_17 = _12;
_14 = (_18.1,);
_2 = !_5.2.1.1;
Goto(bb14)
}
bb14 = {
_5.2.0 = [RET,_2,_5.2.1.1,RET,RET,_7];
_5.2.1.0 = !_4;
_3 = [(-5076_i16),26936_i16,(-31382_i16),(-2577_i16),(-23039_i16),29824_i16,(-17445_i16),15524_i16];
(*_13) = _5.1;
_22.1 = _18.2;
_20 = [_18.3,_15,_15,_15,_18.3,_18.3,_15,_15];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(9_usize, 6_usize, Move(_6), 21_usize, Move(_21), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(9_usize, 8_usize, Move(_8), 14_usize, Move(_14), 1_usize, Move(_1), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i32,mut _2: i32,mut _3: isize,mut _4: u64,mut _5: u16,mut _6: i64,mut _7: u16,mut _8: [i16; 8],mut _9: u64,mut _10: u16,mut _11: i32,mut _12: isize) -> u16 {
mir! {
type RET = u16;
let _13: usize;
let _14: isize;
let _15: [i32; 6];
let _16: Adt53;
let _17: [i64; 4];
let _18: ();
let _19: ();
{
RET = _6 as u16;
_7 = _5 - _5;
_9 = _7 as u64;
_11 = _1 + _1;
_12 = 285565673248420616716700266878370505966_u128 as isize;
_4 = !_9;
Goto(bb1)
}
bb1 = {
_3 = _12;
_7 = _10 & _5;
_7 = _4 as u16;
_13 = _2 as usize;
_10 = '\u{d4e3a}' as u16;
Goto(bb2)
}
bb2 = {
_10 = _7;
_11 = _2 & _1;
_2 = _11 << _11;
_13 = 13409254722108786986_usize;
_3 = !_12;
_11 = _13 as i32;
_8 = [30208_i16,(-13755_i16),(-31416_i16),6412_i16,(-13306_i16),(-5616_i16),20491_i16,(-6013_i16)];
_8 = [136_i16,22420_i16,(-32440_i16),2837_i16,848_i16,4752_i16,29592_i16,(-16064_i16)];
_9 = _4;
_13 = 5_usize;
_3 = (-106901946860321351611955055871308619318_i128) as isize;
_8[_13] = 3921231629_u32 as i16;
_15 = [_2,_1,_1,_2,_2,_11];
_10 = !_7;
RET = _9 as u16;
_1 = _15[_13] - _2;
RET = _10;
_8[_13] = -7778_i16;
_8[_13] = false as i16;
_6 = (-47388383502149407219189628307977312332_i128) as i64;
_9 = _4 + _4;
_17 = [_6,_6,_6,_6];
_8 = [32588_i16,(-6369_i16),18299_i16,(-11102_i16),(-8425_i16),(-74_i16),(-28250_i16),18467_i16];
_17 = [_6,_6,_6,_6];
_6 = 6691193780135606211_i64;
_13 = !6_usize;
RET = _7 << _10;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(10_usize, 8_usize, Move(_8), 10_usize, Move(_10), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(10_usize, 5_usize, Move(_5), 15_usize, Move(_15), 12_usize, Move(_12), 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (isize, i32, u64, f64),mut _2: f64,mut _3: [i16; 8],mut _4: i32,mut _5: (isize, i32, u64, f64),mut _6: [i16; 8],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: (isize, i32, u64, f64),mut _11: (isize, i32, u64, f64),mut _12: i32,mut _13: [i128; 6],mut _14: u16,mut _15: (isize, i32, u64, f64)) -> u16 {
mir! {
type RET = u16;
let _16: f32;
let _17: u8;
let _18: i16;
let _19: isize;
let _20: isize;
let _21: isize;
let _22: [i16; 8];
let _23: isize;
let _24: char;
let _25: (i64,);
let _26: Adt53;
let _27: (i64,);
let _28: &'static i16;
let _29: (f32, (isize, i32, u64, f64));
let _30: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _31: *mut bool;
let _32: char;
let _33: [i16; 8];
let _34: [i16; 8];
let _35: usize;
let _36: Adt45;
let _37: [i16; 8];
let _38: isize;
let _39: [u16; 2];
let _40: i16;
let _41: u128;
let _42: u16;
let _43: [i16; 8];
let _44: isize;
let _45: [u32; 6];
let _46: bool;
let _47: Adt45;
let _48: usize;
let _49: [u16; 2];
let _50: char;
let _51: ();
let _52: ();
{
_4 = 20312_i16 as i32;
_8 = !_15.0;
_11 = _1;
_15 = (_9, _1.1, _11.2, _11.3);
_8 = !_11.0;
_15.2 = _5.2;
_10.0 = _11.0;
_8 = -_5.0;
_5.3 = _2;
_8 = _10.0;
_15.0 = -_11.0;
_17 = 82_u8 | 97_u8;
_13 = [87065584581880681774326635453408108546_i128,(-100077227934863360109787920111252621333_i128),14100971069815943438230746880298761138_i128,(-140982699553436393758496330255032017234_i128),(-26702649076331054042167299159204691297_i128),100532032235140019940022569467103857587_i128];
RET = _14;
_9 = _15.0;
_11.0 = _17 as isize;
_11.2 = !_5.2;
_6 = _3;
_5 = _10;
_11.2 = _15.2 >> _15.0;
_11 = (_5.0, _5.1, _10.2, _1.3);
Goto(bb1)
}
bb1 = {
_15.0 = _10.2 as isize;
RET = _11.3 as u16;
_10.0 = _1.0 * _7;
_10.0 = (-51248811851893048904750333416173438884_i128) as isize;
_5.0 = _10.3 as isize;
_10.0 = (-37703472217767465821063879590967170218_i128) as isize;
_5.1 = _5.0 as i32;
_15 = (_7, _1.1, _1.2, _2);
_4 = 332952308577172850785509064908294093491_u128 as i32;
_11.0 = _15.0 ^ _15.0;
_11.2 = !_5.2;
_22 = [509_i16,(-25324_i16),17588_i16,(-8619_i16),9202_i16,(-1531_i16),(-12041_i16),(-19088_i16)];
_25.0 = 2559257950991081756_i64 + 6309046609155786274_i64;
_25 = ((-4303232162017823818_i64),);
_5 = (_15.0, _15.1, _11.2, _2);
_13 = [(-137960186630985005914700487792617405295_i128),(-77566610561251228092097921728113166183_i128),(-96384506800412166301061936355265205308_i128),122640670603079358732706210737182449529_i128,(-152070585111413634503172370320182259923_i128),(-110394865323024299816932615795005119886_i128)];
_16 = _15.0 as f32;
_11.0 = -_1.0;
_1.2 = _7 as u64;
_17 = 176_u8 ^ 172_u8;
_10.2 = _11.2 << _11.0;
_11.1 = _10.1 + _1.1;
Goto(bb2)
}
bb2 = {
_15 = _5;
_13 = [137137276192573543171063925128004847631_i128,(-75908007742871161036019307472320385560_i128),(-47905335429791940701257287116405924506_i128),(-47975194102134386664269072552922794805_i128),(-153869201860942087346054492035162205359_i128),128392831054789308289932408913046095142_i128];
_10.2 = !_11.2;
_15.3 = _5.3 + _11.3;
_23 = '\u{ce53f}' as isize;
_11.3 = _25.0 as f64;
_5 = _10;
_29.1.0 = _9 ^ _8;
_21 = !_1.0;
_20 = _15.0;
_22 = [(-24112_i16),(-6789_i16),(-4614_i16),(-26100_i16),13893_i16,(-125_i16),(-7975_i16),12118_i16];
Goto(bb3)
}
bb3 = {
_1.3 = _11.3;
_1.0 = -_15.0;
_27.0 = -_25.0;
_27.0 = _25.0;
_10 = (_29.1.0, _11.1, _1.2, _1.3);
_1.0 = _21;
_10.2 = !_1.2;
_30.1.3 = _11.3;
_1 = _11;
_18 = 30801_i16 >> _5.1;
_18 = _14 as i16;
_5.1 = _14 as i32;
_5.1 = _10.1;
_16 = 13536428448389161381_usize as f32;
_27 = (_25.0,);
RET = !_14;
_29.1.1 = (-160600370964757691497812674098945097229_i128) as i32;
_1.3 = -_15.3;
_30.2 = _3;
_15 = _11;
_19 = _10.0;
_2 = _1.3 - _11.3;
_29.1.0 = _19;
_25 = (_27.0,);
_29.0 = 1_usize as f32;
_30.1.1 = -_11.1;
_30.1 = _11;
_25.0 = !_27.0;
_29.1.3 = _2;
match _27.0 {
0 => bb1,
340282366920938463459071375269750387638 => bb4,
_ => bb2
}
}
bb4 = {
_30.2 = [_18,_18,_18,_18,_18,_18,_18,_18];
_29.1.2 = _10.2;
_28 = &_18;
RET = _14 * _14;
_9 = 117863585880596785953598410291239415001_u128 as isize;
_10.0 = _19 | _20;
_27.0 = _25.0 | _25.0;
_13 = [(-111591733008943801096671228122209194699_i128),(-94451332537481555451627980860967275759_i128),(-5745622184603613768545350786567397083_i128),74140332521615692133988965544610902166_i128,113327396631906126240435766951041699352_i128,(-41067466516566769636890004372162267852_i128)];
_5 = (_19, _1.1, _30.1.2, _15.3);
_12 = _1.0 as i32;
_16 = _29.0 * _29.0;
Goto(bb5)
}
bb5 = {
_5.1 = -_11.1;
_30.1.2 = _29.1.2 << _11.0;
_7 = _15.0 & _29.1.0;
_15 = (_29.1.0, _1.1, _10.2, _2);
_10.2 = !_5.2;
_33 = [_18,(*_28),_18,(*_28),(*_28),(*_28),(*_28),_18];
RET = _14 * _14;
_11.0 = -_7;
_35 = 1_usize;
_21 = -_30.1.0;
_11 = _30.1;
_11.3 = -_5.3;
_30.0 = [_10.1,_11.1,_1.1,_10.1,_10.1,_11.1];
_35 = _6[_35] as usize;
_30.0 = [_1.1,_1.1,_10.1,_30.1.1,_1.1,_4];
_18 = 5522_i16 & 19680_i16;
_7 = !_30.1.0;
_37 = [_18,_18,_18,_18,_18,_18,_18,_18];
Goto(bb6)
}
bb6 = {
_11.3 = -_29.1.3;
_15 = (_21, _30.1.1, _30.1.2, _2);
_32 = '\u{c69e7}';
_33 = [_18,_18,_18,_18,_18,_18,_18,_18];
_14 = !RET;
_15.2 = _29.1.2 << _11.1;
_29.1.2 = !_30.1.2;
_42 = !RET;
_43 = [_18,_18,_18,_18,_18,_18,_18,_18];
_11 = (_5.0, _15.1, _29.1.2, _30.1.3);
_13 = [(-73717561501288111380602988787157578306_i128),12581522791308333666156782055177943010_i128,49031698950849497480139283311908013840_i128,(-115821231491923958641820364769286466775_i128),114394646065148468749818989691948475743_i128,96827439297762445145832666675787348685_i128];
_30.1.1 = !_1.1;
_11.0 = -_19;
_15.1 = !_11.1;
_39 = [RET,_14];
_29.1.1 = !_11.1;
_23 = _5.0;
_31 = core::ptr::addr_of_mut!(_46);
_30.1.2 = false as u64;
_3 = _6;
_11.0 = _1.0 & _10.0;
_39 = [_14,_42];
_10.1 = true as i32;
_1.0 = -_11.0;
_30.1.2 = !_29.1.2;
Call(_5.0 = fn12(_15, _29, _30, _11.0, _10.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = _1;
_38 = _20 | _1.0;
_44 = _10.0 ^ _5.0;
_11.3 = _1.3 + _29.1.3;
_30.1 = _5;
_39 = [_14,_14];
_41 = 313906739322970518543287582717512946697_u128 ^ 27203254677157899609324967056917446513_u128;
_10.1 = _32 as i32;
_12 = _14 as i32;
_17 = 165_u8 * 195_u8;
RET = _14;
_11.3 = _15.3;
_48 = !_35;
_34 = [_18,_18,_18,_18,_18,_18,_18,_18];
_30.1.0 = _44 - _5.0;
_49 = [_14,RET];
_29.1.1 = _17 as i32;
_15.0 = 1800316185_u32 as isize;
_11.0 = _42 as isize;
_16 = _29.0;
_15.0 = !_30.1.0;
_17 = 3_u8 - 251_u8;
_23 = -_15.0;
_39 = [_14,_14];
RET = _14 << _38;
_1.3 = _25.0 as f64;
_34 = [_18,_18,_18,_18,_18,_18,_18,_18];
_11.0 = _41 as isize;
_11.3 = _10.3;
Goto(bb8)
}
bb8 = {
Call(_51 = dump_var(11_usize, 43_usize, Move(_43), 21_usize, Move(_21), 38_usize, Move(_38), 20_usize, Move(_20)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_51 = dump_var(11_usize, 22_usize, Move(_22), 6_usize, Move(_6), 49_usize, Move(_49), 7_usize, Move(_7)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_51 = dump_var(11_usize, 12_usize, Move(_12), 17_usize, Move(_17), 14_usize, Move(_14), 19_usize, Move(_19)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_51 = dump_var(11_usize, 8_usize, Move(_8), 25_usize, Move(_25), 18_usize, Move(_18), 52_usize, _52), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (isize, i32, u64, f64),mut _2: (f32, (isize, i32, u64, f64)),mut _3: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: i8;
let _7: char;
let _8: *const (isize, *mut bool);
let _9: isize;
let _10: f32;
let _11: ();
let _12: ();
{
_2.1.1 = 134563216816492358550879269374679333069_u128 as i32;
_2.1.1 = (-6796727564975718625_i64) as i32;
_3.2 = [(-25257_i16),(-30930_i16),(-28129_i16),(-23009_i16),5015_i16,(-24577_i16),28924_i16,(-16411_i16)];
_1.3 = _2.1.3;
_2.1.3 = _1.3;
_1.1 = _3.1.2 as i32;
_6 = _2.0 as i8;
_1 = _2.1;
_3.1.0 = _4;
_1.3 = 3_usize as f64;
_3.1.1 = _2.1.1 + _1.1;
RET = _4 * _5;
_9 = _2.1.0 ^ _2.1.0;
_2.1.0 = RET;
_4 = !_9;
_5 = _4 & _1.0;
_3.2 = [(-13402_i16),2347_i16,22177_i16,(-26781_i16),(-20786_i16),(-21566_i16),1594_i16,17651_i16];
_3.1.0 = (-95296731563736966201773108717926054830_i128) as isize;
_3.1.3 = -_1.3;
_2.1.3 = _2.0 as f64;
_7 = '\u{b6f47}';
_1.1 = _3.1.1;
_3.1.0 = 3182394183_u32 as isize;
_2.1 = (_1.0, _3.1.1, _3.1.2, _1.3);
_10 = _2.0 + _2.0;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(12_usize, 7_usize, Move(_7), 9_usize, Move(_9), 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i16; 8],mut _2: isize) -> f64 {
mir! {
type RET = f64;
let _3: [u16; 2];
let _4: [i64; 4];
let _5: [i64; 4];
let _6: char;
let _7: i128;
let _8: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _9: [u32; 6];
let _10: Adt55;
let _11: Adt47;
let _12: Adt55;
let _13: bool;
let _14: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _15: usize;
let _16: *mut *mut *const (isize, *mut bool);
let _17: f32;
let _18: u128;
let _19: isize;
let _20: isize;
let _21: i16;
let _22: ();
let _23: ();
{
RET = 2091025505702438878_i64 as f64;
_1 = [(-13795_i16),19270_i16,(-20156_i16),24347_i16,(-13839_i16),(-9881_i16),(-23844_i16),12037_i16];
RET = (-2560_i16) as f64;
RET = 130_u8 as f64;
RET = 124_i8 as f64;
RET = (-9040686656887327128_i64) as f64;
_2 = !(-9223372036854775808_isize);
_3 = [4973_u16,22455_u16];
_2 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_4 = [2114863617811345121_i64,(-6644891528469543875_i64),(-2801585554458243135_i64),(-5313055637097892720_i64)];
RET = (-24912_i16) as f64;
_2 = 18398585746385899483_usize as isize;
_3 = [54809_u16,14223_u16];
_2 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_2 = 9223372036854775807_isize;
_1 = [(-21081_i16),21222_i16,(-29634_i16),(-2264_i16),(-13758_i16),(-8255_i16),18780_i16,29256_i16];
_5 = [(-4793353574155662133_i64),2413828235226127320_i64,(-5220317295889482332_i64),(-5911644155350288037_i64)];
_3 = [40711_u16,38456_u16];
_2 = !(-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_1 = [1668_i16,(-29158_i16),(-3237_i16),(-19415_i16),2924_i16,7654_i16,(-22552_i16),25626_i16];
Goto(bb2)
}
bb2 = {
_2 = -(-55_isize);
_3 = [30175_u16,30378_u16];
_2 = !9223372036854775807_isize;
_3 = [27483_u16,64234_u16];
_5 = [2886163006008010584_i64,3090299029568270962_i64,4290914705808150444_i64,(-536180425839900192_i64)];
_5 = [3320624400328022802_i64,2907273560791906167_i64,(-7112713179637355752_i64),2499274812702242144_i64];
_5 = [8234558028297936643_i64,1379982346666734945_i64,725504201259704726_i64,6139034856315808827_i64];
_6 = '\u{70bf8}';
_3 = [2582_u16,60690_u16];
_5 = [2085879494435382975_i64,4198662817495126649_i64,(-6385841755402678504_i64),(-5633568585855587138_i64)];
RET = 1222732940_i32 as f64;
_8.2.1.2 = 4226955287_u32 as u64;
_8.3 = 4772288886965307818_i64 - 3789516708110061990_i64;
_8.2.0 = [516509365_i32,(-1759075805_i32),(-850046182_i32),1296543923_i32,2131492686_i32,(-1577404758_i32)];
_8.0 = (-32331_i16) as u16;
_2 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_9 = [2210801156_u32,3769216462_u32,3068999256_u32,4274233044_u32,2250451772_u32,992451419_u32];
RET = 3118961318_u32 as f64;
_8.2.0 = [(-478454092_i32),(-1079302561_i32),118034455_i32,(-484177444_i32),(-1922652676_i32),(-71559920_i32)];
_8.3 = (-6239621951194635800_i64) | (-6234595936839645474_i64);
RET = (-128069701_i32) as f64;
_8.2.1.1 = !(-1697320362_i32);
Goto(bb3)
}
bb3 = {
_6 = '\u{b31f1}';
_8.2.1 = (_2, (-688897831_i32), 15817694330859532542_u64, RET);
_7 = (-66145593039907072285688430864340965934_i128) | 150206743906826887228575338220135017354_i128;
_7 = (-74575761911299561213548063171915146232_i128);
_8.2.1 = (_2, 1584823694_i32, 12372610340989502611_u64, RET);
_9 = [4232467701_u32,3753506030_u32,2022748608_u32,2634958005_u32,3727756418_u32,299946896_u32];
_8.3 = 193585250135459907_i64;
_8.1 = true as u16;
match _8.2.1.2 {
0 => bb1,
1 => bb4,
2 => bb5,
12372610340989502611 => bb7,
_ => bb6
}
}
bb4 = {
_2 = -(-55_isize);
_3 = [30175_u16,30378_u16];
_2 = !9223372036854775807_isize;
_3 = [27483_u16,64234_u16];
_5 = [2886163006008010584_i64,3090299029568270962_i64,4290914705808150444_i64,(-536180425839900192_i64)];
_5 = [3320624400328022802_i64,2907273560791906167_i64,(-7112713179637355752_i64),2499274812702242144_i64];
_5 = [8234558028297936643_i64,1379982346666734945_i64,725504201259704726_i64,6139034856315808827_i64];
_6 = '\u{70bf8}';
_3 = [2582_u16,60690_u16];
_5 = [2085879494435382975_i64,4198662817495126649_i64,(-6385841755402678504_i64),(-5633568585855587138_i64)];
RET = 1222732940_i32 as f64;
_8.2.1.2 = 4226955287_u32 as u64;
_8.3 = 4772288886965307818_i64 - 3789516708110061990_i64;
_8.2.0 = [516509365_i32,(-1759075805_i32),(-850046182_i32),1296543923_i32,2131492686_i32,(-1577404758_i32)];
_8.0 = (-32331_i16) as u16;
_2 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_9 = [2210801156_u32,3769216462_u32,3068999256_u32,4274233044_u32,2250451772_u32,992451419_u32];
RET = 3118961318_u32 as f64;
_8.2.0 = [(-478454092_i32),(-1079302561_i32),118034455_i32,(-484177444_i32),(-1922652676_i32),(-71559920_i32)];
_8.3 = (-6239621951194635800_i64) | (-6234595936839645474_i64);
RET = (-128069701_i32) as f64;
_8.2.1.1 = !(-1697320362_i32);
Goto(bb3)
}
bb5 = {
_1 = [1668_i16,(-29158_i16),(-3237_i16),(-19415_i16),2924_i16,7654_i16,(-22552_i16),25626_i16];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_8.2.0 = [_8.2.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1];
_14.1 = (_8.2.1.0, _8.2.1.1, _8.2.1.2, _8.2.1.3);
_8.2.1.0 = !_14.1.0;
_13 = !false;
_12 = Adt55::Variant3 { fld0: _14.1.1 };
_8.3 = -8313716156807837060_i64;
_10 = Move(_12);
_3 = [_8.1,_8.1];
_8.0 = _8.1 >> _14.1.2;
_14.2 = _1;
_4 = _5;
_6 = '\u{fc42a}';
Goto(bb8)
}
bb8 = {
_8.2.1.3 = _14.1.3;
_4 = _5;
_3 = [_8.0,_8.0];
_4 = _5;
_8.2.1.2 = _8.2.1.1 as u64;
_14.2 = [(-5738_i16),19958_i16,12000_i16,6022_i16,25141_i16,28828_i16,16232_i16,(-6035_i16)];
_14 = (_8.2.0, _8.2.1, _1);
_8.2.1.2 = !_14.1.2;
_12 = Move(_10);
_15 = _14.1.2 as usize;
_14.0 = [_8.2.1.1,_14.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1];
_14.1 = (_2, Field::<i32>(Variant(_12, 3), 0), _8.2.1.2, RET);
_14.0 = _8.2.0;
_8.2.0 = [_8.2.1.1,_8.2.1.1,Field::<i32>(Variant(_12, 3), 0),_8.2.1.1,Field::<i32>(Variant(_12, 3), 0),_14.1.1];
_8.2.1.1 = !Field::<i32>(Variant(_12, 3), 0);
_8.0 = _8.1 - _8.1;
_5 = [_8.3,_8.3,_8.3,_8.3];
SetDiscriminant(_12, 2);
RET = _8.3 as f64;
_8.2.1 = (_2, _14.1.1, _14.1.2, RET);
Goto(bb9)
}
bb9 = {
_15 = 4847_i16 as usize;
_1 = [(-17122_i16),(-29257_i16),(-32236_i16),(-2674_i16),(-18325_i16),17080_i16,9988_i16,(-20403_i16)];
_8.2.1.2 = _15 as u64;
_8.2.1.0 = -_2;
_14.1.1 = _8.2.1.1;
Call(RET = fn14(_8.2.1.0, _14.2, _14.2, _14.1.1, _14.1.2, _8.2.1, _14, _14.1.2, _2, _8.0, _8.2.1.2, _14, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11 = Adt47::Variant0 { fld0: _14.1,fld1: _14,fld2: (-3521_i16),fld3: _1 };
_8.2.2 = _1;
_3 = [_8.1,_8.0];
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1)).2 = _14.2;
place!(Field::<[i16; 8]>(Variant(_11, 0), 3)) = [25554_i16,(-11486_i16),(-15055_i16),6797_i16,14388_i16,(-10719_i16),12518_i16,(-14845_i16)];
_8 = (50694_u16, 44403_u16, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1), 4345643863423015130_i64);
RET = _7 as f64;
_16 = core::ptr::addr_of_mut!(place!(Field::<*mut *const (isize, *mut bool)>(Variant(_12, 2), 0)));
_14.1.2 = !Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0).2;
_17 = _8.3 as f32;
_14 = (_8.2.0, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1).1, _1);
_8.2.1.3 = _8.3 as f64;
_3 = [_8.0,_8.1];
_9 = [3542593696_u32,801292616_u32,3625413117_u32,1522193895_u32,523829923_u32,2877260467_u32];
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1)).1.3 = _8.2.1.3 - _8.2.1.3;
_14.1.3 = Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1).1.3 * _8.2.1.3;
_8.3 = (-3202815085614136187_i64);
_17 = _14.1.0 as f32;
place!(Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0)).1 = (-81_i8) as i32;
_8.0 = _8.3 as u16;
_2 = _8.3 as isize;
match Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1).1.1 {
0 => bb4,
1 => bb11,
2 => bb12,
1584823694 => bb14,
_ => bb13
}
}
bb11 = {
_15 = 4847_i16 as usize;
_1 = [(-17122_i16),(-29257_i16),(-32236_i16),(-2674_i16),(-18325_i16),17080_i16,9988_i16,(-20403_i16)];
_8.2.1.2 = _15 as u64;
_8.2.1.0 = -_2;
_14.1.1 = _8.2.1.1;
Call(RET = fn14(_8.2.1.0, _14.2, _14.2, _14.1.1, _14.1.2, _8.2.1, _14, _14.1.2, _2, _8.0, _8.2.1.2, _14, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_8.2.1.3 = _14.1.3;
_4 = _5;
_3 = [_8.0,_8.0];
_4 = _5;
_8.2.1.2 = _8.2.1.1 as u64;
_14.2 = [(-5738_i16),19958_i16,12000_i16,6022_i16,25141_i16,28828_i16,16232_i16,(-6035_i16)];
_14 = (_8.2.0, _8.2.1, _1);
_8.2.1.2 = !_14.1.2;
_12 = Move(_10);
_15 = _14.1.2 as usize;
_14.0 = [_8.2.1.1,_14.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1,_8.2.1.1];
_14.1 = (_2, Field::<i32>(Variant(_12, 3), 0), _8.2.1.2, RET);
_14.0 = _8.2.0;
_8.2.0 = [_8.2.1.1,_8.2.1.1,Field::<i32>(Variant(_12, 3), 0),_8.2.1.1,Field::<i32>(Variant(_12, 3), 0),_14.1.1];
_8.2.1.1 = !Field::<i32>(Variant(_12, 3), 0);
_8.0 = _8.1 - _8.1;
_5 = [_8.3,_8.3,_8.3,_8.3];
SetDiscriminant(_12, 2);
RET = _8.3 as f64;
_8.2.1 = (_2, _14.1.1, _14.1.2, RET);
Goto(bb9)
}
bb13 = {
_1 = [1668_i16,(-29158_i16),(-3237_i16),(-19415_i16),2924_i16,7654_i16,(-22552_i16),25626_i16];
Goto(bb2)
}
bb14 = {
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1)).2 = [(-15749_i16),12407_i16,15354_i16,(-10098_i16),23779_i16,12124_i16,21286_i16,(-23446_i16)];
place!(Field::<i16>(Variant(_11, 0), 2)) = _8.2.1.1 as i16;
_4 = _5;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1)).1.3 = _8.2.1.3 + _8.2.1.3;
_9 = [604993847_u32,2176842964_u32,1735042824_u32,1494783518_u32,2527003782_u32,2827216605_u32];
_14.1.1 = _8.2.1.1 << Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1).1.1;
_14.1.2 = Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0).2;
_8.3 = 1781122563618847828_i64;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_11, 0), 1)).1 = (_14.1.0, Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0).1, Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0).2, _14.1.3);
_8.2.2 = Field::<[i16; 8]>(Variant(_11, 0), 3);
_17 = 43_u8 as f32;
RET = -_14.1.3;
place!(Field::<[i16; 8]>(Variant(_11, 0), 3)) = _8.2.2;
_5 = _4;
_8.0 = _8.1 % _8.1;
_8.2.1.0 = -_2;
_7 = 39061192388347144686345768245295732728_u128 as i128;
_14.2 = _8.2.2;
place!(Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0)).3 = _17 as f64;
_4 = _5;
_1 = [Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2),Field::<i16>(Variant(_11, 0), 2)];
_7 = 114633355417940571724888834247387088239_i128 & (-23029994512856364311253003400733460926_i128);
place!(Field::<i16>(Variant(_11, 0), 2)) = !16322_i16;
place!(Field::<(isize, i32, u64, f64)>(Variant(_11, 0), 0)).1 = _8.2.1.1 ^ _14.1.1;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(13_usize, 6_usize, Move(_6), 1_usize, Move(_1), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(13_usize, 5_usize, Move(_5), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: [i16; 8],mut _3: [i16; 8],mut _4: i32,mut _5: u64,mut _6: (isize, i32, u64, f64),mut _7: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),mut _8: u64,mut _9: isize,mut _10: u16,mut _11: u64,mut _12: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),mut _13: ([i32; 6], (isize, i32, u64, f64), [i16; 8])) -> f64 {
mir! {
type RET = f64;
let _14: f32;
let _15: char;
let _16: Adt47;
let _17: (f32, (isize, i32, u64, f64));
let _18: [i16; 8];
let _19: u64;
let _20: Adt48;
let _21: bool;
let _22: u128;
let _23: (isize, *mut bool);
let _24: [u16; 2];
let _25: [i32; 6];
let _26: isize;
let _27: ();
let _28: ();
{
_6.1 = 29669_i16 as i32;
_13.1.2 = true as u64;
RET = -_7.1.3;
_7.0 = [_4,_4,_13.1.1,_4,_13.1.1,_4];
_2 = [9372_i16,(-2018_i16),(-6803_i16),26067_i16,(-12702_i16),17752_i16,(-11817_i16),(-25955_i16)];
_13.1.2 = _7.1.2;
_6.2 = 24_i8 as u64;
_6.0 = _13.1.0;
_12.1 = (_1, _6.1, _5, RET);
_1 = _13.1.0;
_12.1.0 = 36_u8 as isize;
_13.1 = (_12.1.0, _12.1.1, _7.1.2, _7.1.3);
_12.1.2 = !_7.1.2;
_6.3 = -_12.1.3;
_7.1.3 = -RET;
_13.1.0 = _9 + _1;
_12.1.2 = _13.1.2 - _8;
_10 = 20760_u16 - 3393_u16;
_17.1 = _7.1;
_7.0 = [_17.1.1,_7.1.1,_4,_13.1.1,_7.1.1,_17.1.1];
_6.2 = !_13.1.2;
_10 = 44868_u16;
match _7.1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
1584823694 => bb5,
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
_13.1 = (_7.1.0, _6.1, _8, _7.1.3);
_18 = _13.2;
_7.1.1 = _12.1.2 as i32;
_13.0 = [_4,_7.1.1,_17.1.1,_7.1.1,_17.1.1,_17.1.1];
_6 = (_12.1.0, _12.1.1, _17.1.2, _17.1.3);
_6.3 = _17.1.3 * RET;
RET = _17.1.0 as f64;
_17.1.3 = _12.1.3 * _6.3;
_12.1.0 = _1;
_1 = -_13.1.0;
_12.1 = _6;
_13.1.0 = _1;
_3 = _18;
_12.2 = [10695_i16,24710_i16,23253_i16,18355_i16,17013_i16,(-4106_i16),(-1835_i16),5069_i16];
_17.1 = (_9, _4, _6.2, _7.1.3);
_7.1 = (_13.1.0, _17.1.1, _8, _17.1.3);
RET = _7.1.3;
_13.1.2 = !_7.1.2;
_6 = _7.1;
_6.0 = _9 | _13.1.0;
_16 = Adt47::Variant0 { fld0: _12.1,fld1: _7,fld2: 27396_i16,fld3: _13.2 };
_7.1.0 = _1;
match _7.1.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
1584823694 => bb14,
_ => bb13
}
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
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)).2 = [6565_i16,(-2958_i16),(-22510_i16),(-18201_i16),14935_i16,13548_i16,(-25722_i16),21603_i16];
_11 = _13.1.2 | _13.1.2;
_17.1.0 = !Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1).1.0;
_9 = _10 as isize;
_7.1.2 = _12.1.2;
_12.1 = _6;
_7.1.1 = _4;
_6.3 = 1882593207920166067953318544027145580_i128 as f64;
_12 = (_13.0, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1).1, _3);
_23.1 = core::ptr::addr_of_mut!(_21);
_7.1.2 = _13.1.3 as u64;
_3 = [(-27369_i16),32348_i16,(-21041_i16),21144_i16,1579_i16,(-10720_i16),(-654_i16),(-9291_i16)];
_23.0 = 15_i8 as isize;
place!(Field::<(isize, i32, u64, f64)>(Variant(_16, 0), 0)).2 = Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1).1.1 as u64;
_12 = (_13.0, _13.1, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1).2);
_6 = _17.1;
_13.1.2 = 2_usize as u64;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)).1.2 = !_11;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)) = (_13.0, _6, _13.2);
_17.1.2 = !_12.1.2;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)).2 = _3;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)).2 = [14456_i16,14550_i16,(-24605_i16),23009_i16,(-27925_i16),14791_i16,(-22893_i16),(-14219_i16)];
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_16, 0), 1)).1.0 = _6.0;
RET = _17.1.3 + _13.1.3;
place!(Field::<i16>(Variant(_16, 0), 2)) = 93939578937349046338427634210147461395_i128 as i16;
_12 = (_13.0, Field::<(isize, i32, u64, f64)>(Variant(_16, 0), 0), _2);
_13.1.2 = _11 << _5;
_7.1.0 = _23.0;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 5_usize, Move(_5), 8_usize, Move(_8), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 4_usize, Move(_4), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: [i128; 6],mut _6: i64) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _7: [i64; 4];
let _8: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _9: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _10: isize;
let _11: Adt50;
let _12: [i64; 4];
let _13: [i32; 6];
let _14: &'static i16;
let _15: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _16: i64;
let _17: i16;
let _18: *const (isize, *mut bool);
let _19: f32;
let _20: Adt44;
let _21: Adt53;
let _22: [i128; 6];
let _23: isize;
let _24: Adt47;
let _25: *const (isize, *mut bool);
let _26: (*mut bool, u32, *mut bool, char);
let _27: [char; 8];
let _28: Adt45;
let _29: (isize, i32, u64, f64);
let _30: (isize, i32, u64, f64);
let _31: char;
let _32: ();
let _33: ();
{
RET = [150166666582065422277855039318056356218_i128,109361635633053891050985565748033404420_i128,(-62806909707437342688542439828969513032_i128),7077043835638688907071260440679969441_i128,(-104198079688232663619304047928628570271_i128),(-147742441220292218299089532774832965533_i128)];
_4 = RET;
_3 = [(-138443643622074784598471832502323729148_i128),125644122501764504696660331322007746603_i128,125215632092357586699455376978783972815_i128,(-19284078938680991592660618257876414418_i128),(-88591617821634306882231324247155351337_i128),(-48233964691349149935164678807250239865_i128)];
RET = _1;
_7 = [_6,_6,_6,_6];
_4 = _3;
RET = [125633355702776766325379449050302242534_i128,(-19942448484531714304140546230243466473_i128),(-49978343238656891684160740627784697271_i128),97230002729410898750990581157844677628_i128,(-74307824006925686490148040815048744626_i128),(-146318200187463830078022468822815625350_i128)];
_2 = _4;
_3 = [(-80524376722953489680167094251067712017_i128),125876477307245049503826921121570798326_i128,(-151022852243618784686310511043438682400_i128),(-85569717280893650371032313170312410714_i128),(-161412750979185872778197045819092806656_i128),76882085280006225399932785207627385728_i128];
_8.0 = [1512970740_i32,2102919754_i32,(-1735441017_i32),457495337_i32,2061792458_i32,1704807615_i32];
RET = [3780511822865202425530208482423796443_i128,10729553573980788865582549247072961217_i128,100465443638523058578041584979225989716_i128,(-149780406233900962560656549615874228472_i128),(-10434136647746436799143132319392103991_i128),(-146816750073346848497558195088740854279_i128)];
_8.1.3 = 9223372036854775807_isize as f64;
_8.1.0 = -(-89_isize);
_5 = [(-30952882404960117811632587823068337655_i128),157394180174358008793069261646712448563_i128,7427223801591411719689042395861310705_i128,45916439193983733332636970223201755117_i128,65542407244733689996289018950079505090_i128,(-67130378238427344960412938784738468542_i128)];
_1 = _5;
_9.2 = [15949_i16,(-26943_i16),1631_i16,(-7898_i16),24667_i16,20844_i16,17761_i16,21862_i16];
_9.1.1 = !1238525224_i32;
RET = [(-32614642187382978486861454705589569863_i128),(-117028996625523158293282884725560966948_i128),(-106012748764617606386578618714816411947_i128),66337892688247093433398196116996183261_i128,(-159425330239910899521446535017884576434_i128),(-135242375814585769784044057747892348375_i128)];
_8.2 = [13004_i16,(-19598_i16),22423_i16,(-27054_i16),3314_i16,(-18158_i16),(-31577_i16),11766_i16];
_8.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
Goto(bb1)
}
bb1 = {
_8.1.3 = 1043005577_u32 as f64;
_9.1.2 = 4979328608625425788_u64;
_10 = _8.1.0;
_8.1.2 = _9.1.2;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.1.0 = _10;
_8.2 = [15914_i16,(-25532_i16),(-4530_i16),(-29507_i16),8457_i16,5131_i16,9221_i16,1327_i16];
Goto(bb2)
}
bb2 = {
_2 = [(-20316747973817245792003154802536099349_i128),(-88441111271926949796919758568815514741_i128),(-99096024612046398185297732172149679318_i128),19645195090345967279956037365480133391_i128,(-7153627141141070964187920872952321462_i128),27775368006239718485812698976736759003_i128];
_9.1.0 = _8.1.0 ^ _10;
_9.1 = (_10, (-1316887128_i32), _8.1.2, _8.1.3);
_1 = _2;
_8.1.1 = 98_u8 as i32;
Goto(bb3)
}
bb3 = {
_8.1.3 = -_9.1.3;
Goto(bb4)
}
bb4 = {
_9.2 = _8.2;
_9.1.1 = _8.1.1;
_8.1.1 = _9.1.1 | _9.1.1;
_7 = [_6,_6,_6,_6];
_2 = [27346120666662630223448625966583707005_i128,(-97009763815906423791471853479162937903_i128),(-150440763505958808752556794854086658850_i128),(-99239374990130139250025586519333570527_i128),(-80074185508956573000129729084656127525_i128),12212403960477070258093520333759301112_i128];
_8.1.3 = 254650928197730703874172309420034318403_u128 as f64;
_8.2 = _9.2;
_13 = [_8.1.1,_9.1.1,_8.1.1,_8.1.1,_8.1.1,_9.1.1];
_2 = _1;
_15.2.0 = _9.0;
match _8.1.2 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
4979328608625425788 => bb12,
_ => bb11
}
}
bb5 = {
_8.1.3 = -_9.1.3;
Goto(bb4)
}
bb6 = {
_2 = [(-20316747973817245792003154802536099349_i128),(-88441111271926949796919758568815514741_i128),(-99096024612046398185297732172149679318_i128),19645195090345967279956037365480133391_i128,(-7153627141141070964187920872952321462_i128),27775368006239718485812698976736759003_i128];
_9.1.0 = _8.1.0 ^ _10;
_9.1 = (_10, (-1316887128_i32), _8.1.2, _8.1.3);
_1 = _2;
_8.1.1 = 98_u8 as i32;
Goto(bb3)
}
bb7 = {
_8.1.3 = 1043005577_u32 as f64;
_9.1.2 = 4979328608625425788_u64;
_10 = _8.1.0;
_8.1.2 = _9.1.2;
_9.0 = [_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1,_9.1.1];
_9.1.0 = _10;
_8.2 = [15914_i16,(-25532_i16),(-4530_i16),(-29507_i16),8457_i16,5131_i16,9221_i16,1327_i16];
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
_15.3 = -_6;
_12 = [_6,_15.3,_6,_15.3];
RET = [128691164295357784212389949753848970746_i128,50896240536976702279828050460368393377_i128,42782154284164293539768933078044415699_i128,(-95984588004356679665128344878384428032_i128),103912938015449389946138784322011762581_i128,121546775825292985877342677488592983435_i128];
_8.1 = _9.1;
_15.2.1.2 = 134497274433581777408695876546062231673_i128 as u64;
_8.1.2 = _9.1.2 - _15.2.1.2;
_15 = (47018_u16, 40998_u16, _9, _6);
_15.2.1.1 = _8.1.1;
_16 = 5_usize as i64;
_4 = [91222682545328053664930094733803734873_i128,137728526038185627831880574624158896163_i128,18447678066280841409113455775473689646_i128,151915066015698415352389360521184437931_i128,(-136949650017962480208269878599032330177_i128),88509361568777285716049611194502264680_i128];
_10 = _8.1.0 ^ _8.1.0;
_15.2.1.3 = _8.1.3;
_15.2.0 = [_9.1.1,_9.1.1,_9.1.1,_15.2.1.1,_15.2.1.1,_8.1.1];
_14 = &_17;
_15.2.1.2 = _9.1.2 % _9.1.2;
_6 = _15.1 as i64;
_8.0 = _13;
_6 = -_15.3;
Goto(bb13)
}
bb13 = {
_15.1 = _15.0 ^ _15.0;
_22 = [(-155220732008733258548357372836012489300_i128),(-8093363819851380992573380817155888475_i128),(-98403319060049580992259921612520613600_i128),(-149760890314256008405884392700003109068_i128),(-76951211722658168314797681890977755200_i128),(-38335790592586978797204555079585738839_i128)];
RET = [117800197681922448938411712768176269352_i128,100028170713745040117691412132317934602_i128,(-981481129225455503788860693700226577_i128),(-57646976065085835770861779805699917747_i128),(-165354389687911522174991721811059815077_i128),(-32907059101086382711373355723069675847_i128)];
_13 = _9.0;
Goto(bb14)
}
bb14 = {
_12 = [_16,_6,_6,_6];
_27 = ['\u{815e0}','\u{69984}','\u{7e440}','\u{6fd6e}','\u{b7421}','\u{2e374}','\u{64419}','\u{62606}'];
_8.1.3 = -_15.2.1.3;
_9.1.1 = _15.2.1.1;
_26.1 = 3_usize as u32;
_15.2.0 = [_15.2.1.1,_8.1.1,_8.1.1,_8.1.1,_9.1.1,_9.1.1];
_15.3 = _6 ^ _6;
_30.2 = _8.1.2;
_3 = RET;
RET = _22;
_8.1.3 = _9.1.3;
RET = [163585506123768428791948174754245150333_i128,47325327286065781760245662399520032334_i128,128768602743364640849979054512224802773_i128,(-138339130327367313264661564207444183239_i128),(-165702894408091892084371541342175243096_i128),(-98929950501730525828993299051974074488_i128)];
_6 = -_15.3;
_30.1 = _8.1.1;
_6 = !_15.3;
_8.1.0 = !_10;
_12 = _7;
_15.2.1.3 = (-30608_i16) as f64;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(15_usize, 16_usize, Move(_16), 27_usize, Move(_27), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(15_usize, 22_usize, Move(_22), 1_usize, Move(_1), 33_usize, _33, 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: [i128; 6],mut _6: [i128; 6],mut _7: [i128; 6],mut _8: [i128; 6],mut _9: [i128; 6]) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _10: char;
let _11: i32;
let _12: i16;
let _13: *const (isize, *mut bool);
let _14: char;
let _15: f32;
let _16: Adt48;
let _17: u32;
let _18: f64;
let _19: Adt47;
let _20: char;
let _21: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _22: isize;
let _23: char;
let _24: f32;
let _25: f64;
let _26: (*const i128, (f32, (isize, i32, u64, f64)), *const i128);
let _27: bool;
let _28: Adt56;
let _29: Adt55;
let _30: [u16; 2];
let _31: bool;
let _32: (u32,);
let _33: Adt46;
let _34: Adt55;
let _35: u128;
let _36: ();
let _37: ();
{
_6 = [140237542496769031803104246476074139884_i128,(-169434259033021185076054894735810610716_i128),(-49476446573492100804054239211061468233_i128),(-158394214232244580631055131816766706265_i128),(-67058705838897598478479916895711930385_i128),(-166138417689268255612904985184901887371_i128)];
_12 = (-2811_i16) << 3756957048967540127_i64;
RET = [56215791269165400433729031927613621327_i128,117601322414448454903950183999949136508_i128,67666150620430463628030417460973160221_i128,143855517902579466275818947594420801904_i128,142873058236568720208215570037563946954_i128,77822672864766021937719586634881925208_i128];
_12 = -27998_i16;
_7 = [(-50757132974831796192958175824842527985_i128),(-35094936973670153143415604510468911708_i128),75685910593107428344615878345340074565_i128,(-18001782727434313408790421836747300275_i128),(-65255696645143457922791620247232907280_i128),83783040343655586106076229529587914421_i128];
RET = [(-8137417923552031025282789467941942053_i128),28399430342028072828444409647762735909_i128,(-71347092889219670739583928549404385541_i128),(-134931400075437139228815836171958817501_i128),82311728887513658261512794329476424469_i128,158433260261304057272486458135661637734_i128];
_5 = _6;
Goto(bb1)
}
bb1 = {
_5 = [64054268772499184012022475011288610917_i128,66944914984176386422144816601886624841_i128,(-154604931124435165171218416061191275158_i128),17067801629772878575864945380950123255_i128,164995334256014578119061283245868842938_i128,(-109380037348603879123461838984018824565_i128)];
_4 = [31075734795479738745646765621607547494_i128,29613559757066381783120605326241717516_i128,51955737690275613858526394698788650721_i128,142462853643640700489556450992731199440_i128,(-4138656067049181061751082051973414093_i128),112810104366561719187511393966485790987_i128];
_1 = [(-91014724929418411788500134913731132763_i128),(-67635526381208945090818710641896050948_i128),(-2068737958991153689239646558178537661_i128),153270183632107502867120983461278973991_i128,117000700401322428618289631494167131608_i128,(-150935903603011110880208346420624335634_i128)];
_6 = [(-142887923266231406660414763020204826074_i128),78203662171936230209969680207064876916_i128,(-1089833692577047609247015494296714224_i128),(-83956609406381066984581087869427289440_i128),148505050591321852317637397193814315307_i128,(-137968100601267125213525439349101367189_i128)];
_11 = !(-1724957074_i32);
_11 = !1785644593_i32;
_14 = '\u{aad81}';
_14 = '\u{824b}';
_9 = [149410797522716167890493806867751218447_i128,127506676405271582081118677833857330924_i128,118209105278628482762322638165664203020_i128,(-60242783480427198904072378653810812730_i128),162115247721937122413658385259714116444_i128,(-36957303007599294632039839082265525566_i128)];
_12 = (-25271_i16);
RET = [(-151718077575057885963510441243717099480_i128),138034667269857053303215445901671224629_i128,23395347568388068445194324511206317096_i128,(-61965910389076340356403642468389969148_i128),(-85982186425103529754695441354930971195_i128),(-54238664010848962933708199780123534425_i128)];
_5 = _6;
_10 = _14;
_7 = [(-98351739245775047482190067329783033765_i128),42924774767317945357459667587444784843_i128,(-100288536341683626769866316245651832371_i128),62961702119573796628325883600855276474_i128,(-155488224048257908061272522335512227001_i128),(-59909311323919029530773504201362401394_i128)];
_11 = !(-1322079354_i32);
_12 = 313088611353511140700693578592011197162_u128 as i16;
_1 = [136471258006063102989342152800817965826_i128,74943930087931398784962967455506176146_i128,(-36234219580131466022401594788666709965_i128),39770437758083455283830668074811988830_i128,(-78925149793320175343673361929561101381_i128),115147986348796451407826401793626070432_i128];
_6 = [81304207701868283587920671169543448298_i128,(-138809905684887082007139005033228446395_i128),(-114829194919780194074570646525595848779_i128),(-100714609253672116820124446928516921431_i128),(-104766711033910430144930885947452827941_i128),(-132512360724579792901666624984404061603_i128)];
RET = _5;
_17 = 3802659373_u32 + 1937050528_u32;
_14 = _10;
_18 = _12 as f64;
Goto(bb2)
}
bb2 = {
_10 = _14;
_6 = [130973479749644924492708637709123938024_i128,56183151521501661963164707561584210309_i128,(-44604900285077049849679378209643593046_i128),(-104096678654893623259748848968035865484_i128),109293830581009294226262314659013379173_i128,164955050861611215598185878364092622743_i128];
_11 = 8457517919132036228_i64 as i32;
_14 = _10;
_18 = (-32_isize) as f64;
_14 = _10;
_9 = [15440436626986501738862870701287798922_i128,75876068797586505591905262362592805193_i128,119394103485964190651527439487452662899_i128,(-24897084279365130096261984994677692015_i128),169601886064505651669063353275755902538_i128,122576575542808950973560625719395125076_i128];
_2 = [114426659607067609923352103774643622663_i128,(-68481899982116629213088693443356869296_i128),(-137925648064214426795805199014238321114_i128),90511678199669127998013870834885734325_i128,(-156986008970424720869064192048742694992_i128),37748791634228855762386704910498310985_i128];
_2 = [(-94757251899840121294897506227080736066_i128),85980159301887531650217740999778820285_i128,1092128166310455366214008254286322204_i128,131610543213153328829628228993565025459_i128,(-97555982450985409700195233016173217618_i128),(-18101617768158557848244164731752232279_i128)];
_8 = [57163744032792563219299297727755884264_i128,(-128315821272670720340347126115267427246_i128),(-9525846938783923219392951615918302784_i128),(-16737155742232412677508707790069801942_i128),(-141313325661269293900254964618232580663_i128),41887523904296823628159434779139744780_i128];
_3 = [(-96935794061894106633719523850297851293_i128),34538469989671070528382992696371493759_i128,(-97836857308857436223720330750998125922_i128),(-87664103671322152703766319512609844610_i128),97562325373242354046787197791872372719_i128,(-122566679500385351601877831464257188913_i128)];
_6 = [53764412382347532182211985253906390169_i128,158033945356417472636927259327704853160_i128,54659942315043507283667530124552552969_i128,156520650692793511681899130257780819049_i128,(-145110397895934201448129565604330977614_i128),129042946084510299685961232475331385864_i128];
_3 = [(-96675207494787953417699902098672379124_i128),67385780250824973537078932619950001501_i128,(-76511927706434899691580509655835216514_i128),(-151297072510147329163016165205636105736_i128),105135460503671744485938386600745808641_i128,(-53010692127697120928428624984750049112_i128)];
_10 = _14;
_10 = _14;
_18 = _11 as f64;
_4 = [9128369528830778667602159241031848708_i128,(-44399527684343612826444545626318182489_i128),59328419341220104601723121135631708748_i128,25181645594272002181048257234329311261_i128,94978441609651031227041227726145285874_i128,(-114511447484971228809962757137476121390_i128)];
RET = [(-42212046322276794929278201232347232233_i128),138467569368205539753016711864535997673_i128,(-93120022979591253357048415506593247107_i128),79604649802259222080363397017593368080_i128,(-78028254393101891517410046397590399243_i128),(-151990736502055379067172724835370502272_i128)];
_11 = (-1253301484_i32) & 380451876_i32;
_15 = (-37170075255441025193695097182118432549_i128) as f32;
_3 = [(-26647734500527925540834551659446889962_i128),(-55331582520834683842230655790283940653_i128),145490032480268995206783219026632407744_i128,117005749665459019557995739721586501259_i128,30873069269347041171992972222065481093_i128,103525769031300136980990291480290488205_i128];
_8 = RET;
_10 = _14;
RET = [(-126035103061996068473263472351262420880_i128),(-12390255063435927105399882922921260128_i128),140919640893898222582064286005447350798_i128,(-38451224316656586643542193593090169374_i128),104355291664249611614061272434999738630_i128,45664189978777526787236993336632611880_i128];
_2 = _4;
_1 = _8;
_11 = 873879150_i32 + 844342365_i32;
Goto(bb3)
}
bb3 = {
_4 = _3;
_21.2.1.2 = 5412514048628502491_u64;
_7 = _9;
_21.2.1.0 = 16_isize;
_21.0 = 45581_u16;
_21.2.0 = [_11,_11,_11,_11,_11,_11];
_21.2.1.0 = (-9223372036854775808_isize) << _11;
_21.2.1 = (52_isize, _11, 12386097038537614052_u64, _18);
RET = [(-15596073438425151909347510094808697167_i128),(-98553459372456509592534838785463453011_i128),87095759322427778515180383069693317524_i128,(-113554499500844050226650242943215335392_i128),(-109622481033210116952459672256760437997_i128),(-147946115754612420898798340887400663956_i128)];
_21.2.0 = [_11,_21.2.1.1,_21.2.1.1,_11,_11,_11];
_21.3 = _17 as i64;
_22 = -_21.2.1.0;
_21.1 = _21.2.1.3 as u16;
_7 = [(-107385097276200231448048454494900215489_i128),103856856552199914071892475775701690690_i128,(-982846823905466150404623753109656707_i128),141950296145882014597394196050391995692_i128,(-144175672168428894062838038968339036963_i128),(-55011552292223292278934648283613226401_i128)];
_21.2.2 = [_12,_12,_12,_12,_12,_12,_12,_12];
_5 = _1;
_9 = [(-100958641965640659665501955257754453680_i128),38349202655544249278058807665505193553_i128,(-126001722302258800836983877249895807938_i128),(-45934670576915917463171917374103128322_i128),(-5137780095771121797474997409103664375_i128),(-144246905166831320099828535470897473973_i128)];
match _21.2.1.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
12386097038537614052 => bb9,
_ => bb8
}
}
bb4 = {
_10 = _14;
_6 = [130973479749644924492708637709123938024_i128,56183151521501661963164707561584210309_i128,(-44604900285077049849679378209643593046_i128),(-104096678654893623259748848968035865484_i128),109293830581009294226262314659013379173_i128,164955050861611215598185878364092622743_i128];
_11 = 8457517919132036228_i64 as i32;
_14 = _10;
_18 = (-32_isize) as f64;
_14 = _10;
_9 = [15440436626986501738862870701287798922_i128,75876068797586505591905262362592805193_i128,119394103485964190651527439487452662899_i128,(-24897084279365130096261984994677692015_i128),169601886064505651669063353275755902538_i128,122576575542808950973560625719395125076_i128];
_2 = [114426659607067609923352103774643622663_i128,(-68481899982116629213088693443356869296_i128),(-137925648064214426795805199014238321114_i128),90511678199669127998013870834885734325_i128,(-156986008970424720869064192048742694992_i128),37748791634228855762386704910498310985_i128];
_2 = [(-94757251899840121294897506227080736066_i128),85980159301887531650217740999778820285_i128,1092128166310455366214008254286322204_i128,131610543213153328829628228993565025459_i128,(-97555982450985409700195233016173217618_i128),(-18101617768158557848244164731752232279_i128)];
_8 = [57163744032792563219299297727755884264_i128,(-128315821272670720340347126115267427246_i128),(-9525846938783923219392951615918302784_i128),(-16737155742232412677508707790069801942_i128),(-141313325661269293900254964618232580663_i128),41887523904296823628159434779139744780_i128];
_3 = [(-96935794061894106633719523850297851293_i128),34538469989671070528382992696371493759_i128,(-97836857308857436223720330750998125922_i128),(-87664103671322152703766319512609844610_i128),97562325373242354046787197791872372719_i128,(-122566679500385351601877831464257188913_i128)];
_6 = [53764412382347532182211985253906390169_i128,158033945356417472636927259327704853160_i128,54659942315043507283667530124552552969_i128,156520650692793511681899130257780819049_i128,(-145110397895934201448129565604330977614_i128),129042946084510299685961232475331385864_i128];
_3 = [(-96675207494787953417699902098672379124_i128),67385780250824973537078932619950001501_i128,(-76511927706434899691580509655835216514_i128),(-151297072510147329163016165205636105736_i128),105135460503671744485938386600745808641_i128,(-53010692127697120928428624984750049112_i128)];
_10 = _14;
_10 = _14;
_18 = _11 as f64;
_4 = [9128369528830778667602159241031848708_i128,(-44399527684343612826444545626318182489_i128),59328419341220104601723121135631708748_i128,25181645594272002181048257234329311261_i128,94978441609651031227041227726145285874_i128,(-114511447484971228809962757137476121390_i128)];
RET = [(-42212046322276794929278201232347232233_i128),138467569368205539753016711864535997673_i128,(-93120022979591253357048415506593247107_i128),79604649802259222080363397017593368080_i128,(-78028254393101891517410046397590399243_i128),(-151990736502055379067172724835370502272_i128)];
_11 = (-1253301484_i32) & 380451876_i32;
_15 = (-37170075255441025193695097182118432549_i128) as f32;
_3 = [(-26647734500527925540834551659446889962_i128),(-55331582520834683842230655790283940653_i128),145490032480268995206783219026632407744_i128,117005749665459019557995739721586501259_i128,30873069269347041171992972222065481093_i128,103525769031300136980990291480290488205_i128];
_8 = RET;
_10 = _14;
RET = [(-126035103061996068473263472351262420880_i128),(-12390255063435927105399882922921260128_i128),140919640893898222582064286005447350798_i128,(-38451224316656586643542193593090169374_i128),104355291664249611614061272434999738630_i128,45664189978777526787236993336632611880_i128];
_2 = _4;
_1 = _8;
_11 = 873879150_i32 + 844342365_i32;
Goto(bb3)
}
bb5 = {
_5 = [64054268772499184012022475011288610917_i128,66944914984176386422144816601886624841_i128,(-154604931124435165171218416061191275158_i128),17067801629772878575864945380950123255_i128,164995334256014578119061283245868842938_i128,(-109380037348603879123461838984018824565_i128)];
_4 = [31075734795479738745646765621607547494_i128,29613559757066381783120605326241717516_i128,51955737690275613858526394698788650721_i128,142462853643640700489556450992731199440_i128,(-4138656067049181061751082051973414093_i128),112810104366561719187511393966485790987_i128];
_1 = [(-91014724929418411788500134913731132763_i128),(-67635526381208945090818710641896050948_i128),(-2068737958991153689239646558178537661_i128),153270183632107502867120983461278973991_i128,117000700401322428618289631494167131608_i128,(-150935903603011110880208346420624335634_i128)];
_6 = [(-142887923266231406660414763020204826074_i128),78203662171936230209969680207064876916_i128,(-1089833692577047609247015494296714224_i128),(-83956609406381066984581087869427289440_i128),148505050591321852317637397193814315307_i128,(-137968100601267125213525439349101367189_i128)];
_11 = !(-1724957074_i32);
_11 = !1785644593_i32;
_14 = '\u{aad81}';
_14 = '\u{824b}';
_9 = [149410797522716167890493806867751218447_i128,127506676405271582081118677833857330924_i128,118209105278628482762322638165664203020_i128,(-60242783480427198904072378653810812730_i128),162115247721937122413658385259714116444_i128,(-36957303007599294632039839082265525566_i128)];
_12 = (-25271_i16);
RET = [(-151718077575057885963510441243717099480_i128),138034667269857053303215445901671224629_i128,23395347568388068445194324511206317096_i128,(-61965910389076340356403642468389969148_i128),(-85982186425103529754695441354930971195_i128),(-54238664010848962933708199780123534425_i128)];
_5 = _6;
_10 = _14;
_7 = [(-98351739245775047482190067329783033765_i128),42924774767317945357459667587444784843_i128,(-100288536341683626769866316245651832371_i128),62961702119573796628325883600855276474_i128,(-155488224048257908061272522335512227001_i128),(-59909311323919029530773504201362401394_i128)];
_11 = !(-1322079354_i32);
_12 = 313088611353511140700693578592011197162_u128 as i16;
_1 = [136471258006063102989342152800817965826_i128,74943930087931398784962967455506176146_i128,(-36234219580131466022401594788666709965_i128),39770437758083455283830668074811988830_i128,(-78925149793320175343673361929561101381_i128),115147986348796451407826401793626070432_i128];
_6 = [81304207701868283587920671169543448298_i128,(-138809905684887082007139005033228446395_i128),(-114829194919780194074570646525595848779_i128),(-100714609253672116820124446928516921431_i128),(-104766711033910430144930885947452827941_i128),(-132512360724579792901666624984404061603_i128)];
RET = _5;
_17 = 3802659373_u32 + 1937050528_u32;
_14 = _10;
_18 = _12 as f64;
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
_20 = _14;
_3 = [(-154538546349137081839417372670549923568_i128),(-60971929203894970256366367693371958830_i128),22071362166053110003756605530028957425_i128,152330127872000926673504684550464803382_i128,116775309170533268987407767756924612475_i128,(-106333227916871109210586807912130267548_i128)];
_7 = _9;
_21.2.1.1 = _11;
_4 = RET;
_7 = [(-145125676641045976105176568333277968898_i128),(-163419543143315508236749335137068032016_i128),(-159784720369001996020751479117333929994_i128),102575008757377057249718279831022408368_i128,(-79445039672212415080349687576248934130_i128),(-37421423595350982050040872553523587583_i128)];
_11 = _21.2.1.1;
_9 = [(-149407208216759648442907080610529513796_i128),(-23230266611986707923544438526036589207_i128),134207501387297747182565015766895476832_i128,(-40707906513102645086843217208574272303_i128),51854171561985679676645101377682903727_i128,(-4458553665059883573620255255574541603_i128)];
_6 = [51899202945604854006803141418796098639_i128,(-13389292646299422007303131559019768813_i128),125288107883596471504682965175001090564_i128,71963302798323723519856234672104023824_i128,17470298420611597047347637932651097653_i128,58311503069889717070405390530633994701_i128];
_10 = _14;
_4 = [(-95331859598915583616703353601514602976_i128),88353844229400236267974084481701746425_i128,143862639198334434334887870877617373465_i128,71686332973322322992035191614556857288_i128,(-87219787672395071272346966954748852760_i128),115678938425372040753138644097179382791_i128];
_6 = [(-39064863536026270871360844059311488339_i128),(-22649710355452842214432058375559551524_i128),(-24687062786698234767804376872110800035_i128),(-107151700994326793589815737396486304478_i128),(-71790573915180862405278816094740849584_i128),130275948122730875346000404030765514969_i128];
_22 = false as isize;
_19 = Adt47::Variant0 { fld0: _21.2.1,fld1: _21.2,fld2: _12,fld3: _21.2.2 };
_3 = [46534475772806261820593134717009085567_i128,50200680100485708320856124313655976204_i128,26220988693495592448504833809349628891_i128,116692075325343746069983857221909033273_i128,(-122384607852896894974103197222131659290_i128),(-149557675020000072990668418351092570100_i128)];
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).1.3 = -_21.2.1.3;
_21 = (4183_u16, 799_u16, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1), 8237714882266285526_i64);
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).1.2 = Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).2 + Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).2;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).1.3 = Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).3;
_21.2.1.1 = _11 * Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1.1;
_21.2.1 = (_22, Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).1, Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).2, _18);
match _21.2.1.2 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb7,
12386097038537614052 => bb11,
_ => bb10
}
}
bb10 = {
_4 = _3;
_21.2.1.2 = 5412514048628502491_u64;
_7 = _9;
_21.2.1.0 = 16_isize;
_21.0 = 45581_u16;
_21.2.0 = [_11,_11,_11,_11,_11,_11];
_21.2.1.0 = (-9223372036854775808_isize) << _11;
_21.2.1 = (52_isize, _11, 12386097038537614052_u64, _18);
RET = [(-15596073438425151909347510094808697167_i128),(-98553459372456509592534838785463453011_i128),87095759322427778515180383069693317524_i128,(-113554499500844050226650242943215335392_i128),(-109622481033210116952459672256760437997_i128),(-147946115754612420898798340887400663956_i128)];
_21.2.0 = [_11,_21.2.1.1,_21.2.1.1,_11,_11,_11];
_21.3 = _17 as i64;
_22 = -_21.2.1.0;
_21.1 = _21.2.1.3 as u16;
_7 = [(-107385097276200231448048454494900215489_i128),103856856552199914071892475775701690690_i128,(-982846823905466150404623753109656707_i128),141950296145882014597394196050391995692_i128,(-144175672168428894062838038968339036963_i128),(-55011552292223292278934648283613226401_i128)];
_21.2.2 = [_12,_12,_12,_12,_12,_12,_12,_12];
_5 = _1;
_9 = [(-100958641965640659665501955257754453680_i128),38349202655544249278058807665505193553_i128,(-126001722302258800836983877249895807938_i128),(-45934670576915917463171917374103128322_i128),(-5137780095771121797474997409103664375_i128),(-144246905166831320099828535470897473973_i128)];
match _21.2.1.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
12386097038537614052 => bb9,
_ => bb8
}
}
bb11 = {
_21.2.1 = Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1;
_24 = _15 + _15;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).0 = _21.2.0;
_2 = [169060143660995911376330855603639058042_i128,10681858267037622750268905295885953043_i128,(-1268023357282714964683949978453282017_i128),58772008614997536151666608041209943277_i128,(-9939917587017133436737546831623386749_i128),(-153038152623158757988655668067403597374_i128)];
_25 = -_21.2.1.3;
_21.2.1.1 = !Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1.1;
_6 = [(-124728068376446766634795994038111649845_i128),31882304631599002433348327185910459847_i128,(-137098479865164492730929464620657474247_i128),21459919903005644740686041081108863488_i128,(-2327146100681732260854869563841325225_i128),(-66047095273020256765633602326229086740_i128)];
_21.2.1.1 = 104_u8 as i32;
place!(Field::<i16>(Variant(_19, 0), 2)) = _12 | _12;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).1.2 = !_21.2.1.2;
_21.2.2 = [Field::<i16>(Variant(_19, 0), 2),_12,_12,_12,_12,Field::<i16>(Variant(_19, 0), 2),_12,_12];
_7 = [134994660236991350716362144300049569031_i128,2854204581937759928130810192657091274_i128,(-120280117371184000781282020497105468031_i128),140310014379205864658119604634707362008_i128,(-64677351282183472268297661665764561253_i128),162769268919757987006855934976132764698_i128];
_21.3 = _11 as i64;
_26.1.1 = (Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).0, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1.1, _21.2.1.2, Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).3);
_4 = [(-96695444297212822056862018726536839889_i128),114947612491475354407242432384288330722_i128,111633130968155321676306852825482218378_i128,(-6010282848571677449148496270432413131_i128),42321859812747716787522467887098539579_i128,(-167623427426670225429861648762743448831_i128)];
_26.1 = (_15, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1);
_10 = _20;
_22 = _21.2.1.0;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).0 = [_26.1.1.1,_11,_26.1.1.1,_11,Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1.1,_26.1.1.1];
_18 = _26.1.1.3 - Field::<(isize, i32, u64, f64)>(Variant(_19, 0), 0).3;
_23 = _14;
place!(Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1)).2 = [Field::<i16>(Variant(_19, 0), 2),_12,Field::<i16>(Variant(_19, 0), 2),_12,Field::<i16>(Variant(_19, 0), 2),Field::<i16>(Variant(_19, 0), 2),_12,Field::<i16>(Variant(_19, 0), 2)];
_11 = Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).1.1 | _21.2.1.1;
_26.1.1.0 = _11 as isize;
_21.2.1.1 = -_26.1.1.1;
_28.fld1 = Adt49::Variant0 { fld0: Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(_19, 0), 1).0,fld1: Move(_19),fld2: 73676825164829858205874000058175836357_i128,fld3: _21.3 };
_21 = (32893_u16, 1114_u16, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(Field::<Adt47>(Variant(_28.fld1, 0), 1), 0), 1), Field::<i64>(Variant(_28.fld1, 0), 3));
Goto(bb12)
}
bb12 = {
_26.1.1 = (_22, _11, Field::<([i32; 6], (isize, i32, u64, f64), [i16; 8])>(Variant(Field::<Adt47>(Variant(_28.fld1, 0), 1), 0), 1).1.2, _25);
_1 = [(-20474149154304375229336577696364686851_i128),(-104780125288178027237869280689974502457_i128),109945454871909646470155244024015491573_i128,159216161889550781435812141055791327775_i128,54583776405753723434940228635004992006_i128,(-12858747533522189287620084633060017307_i128)];
_24 = _15;
_24 = _26.1.0 + _26.1.0;
_28.fld6 = [_21.0,_21.1];
_21.2.1.3 = -_25;
_8 = _7;
_26.1.1.1 = -_21.2.1.1;
_10 = _20;
_27 = _26.1.1.1 < Field::<(isize, i32, u64, f64)>(Variant(Field::<Adt47>(Variant(_28.fld1, 0), 1), 0), 0).1;
_28.fld0 = core::ptr::addr_of!(_21.1);
_26.1.0 = -_15;
RET = [(-3331094400628225978603615363422039130_i128),(-115234905134339816821009967961673177761_i128),27018514890527873591254203128426939859_i128,(-73621673213219734584176719162789030962_i128),(-60124731791215981799898578226870640113_i128),51147871968871206495317121160067352099_i128];
RET = [108963788896370717407091348997471931386_i128,29280283711425864930293109219526630920_i128,100609581155728435720656398538226887933_i128,34434614605707861661474494483547767005_i128,157875688416545690634469898810172280513_i128,(-4968581564831796915529038414020242771_i128)];
_6 = [(-147425562632906607075693330281192757543_i128),(-113704021493853919746557310824726901786_i128),131525992001823285688564210367596206122_i128,102540715534016949533979681100500306081_i128,(-31035768318214041204404747766215438098_i128),110249994974072742856875828441987284803_i128];
SetDiscriminant(Field::<Adt47>(Variant(_28.fld1, 0), 1), 0);
_10 = _20;
_21.2.1.1 = _11;
_26.1.0 = -_15;
_28.fld5 = !32_u8;
_21.2.1.3 = -_18;
RET = [47363839226186520703633545867358661370_i128,64407839400733696891929364284167662621_i128,(-150327431509841079702030878515569604682_i128),(-88532520998001880393805231411464516311_i128),(-36019772988928733380203042148502591402_i128),32650261940616757131925904672050712094_i128];
_21.2.2 = [_12,_12,_12,_12,_12,_12,_12,_12];
place!(Field::<i128>(Variant(_28.fld1, 0), 2)) = !16577923594629591064442526075483765705_i128;
Goto(bb13)
}
bb13 = {
_21.2.0 = [_21.2.1.1,_11,_11,_26.1.1.1,_11,_26.1.1.1];
_15 = -_24;
RET = _6;
_7 = _2;
_15 = -_24;
_30 = [_21.0,_21.1];
_6 = [Field::<i128>(Variant(_28.fld1, 0), 2),Field::<i128>(Variant(_28.fld1, 0), 2),Field::<i128>(Variant(_28.fld1, 0), 2),Field::<i128>(Variant(_28.fld1, 0), 2),Field::<i128>(Variant(_28.fld1, 0), 2),Field::<i128>(Variant(_28.fld1, 0), 2)];
Goto(bb14)
}
bb14 = {
place!(Field::<(isize, i32, u64, f64)>(Variant(place!(Field::<Adt47>(Variant(_28.fld1, 0), 1)), 0), 0)).1 = !_21.2.1.1;
place!(Field::<(isize, i32, u64, f64)>(Variant(place!(Field::<Adt47>(Variant(_28.fld1, 0), 1)), 0), 0)) = (_22, _21.2.1.1, _26.1.1.2, _18);
_15 = -_24;
_17 = 114478544_u32 << Field::<(isize, i32, u64, f64)>(Variant(Field::<Adt47>(Variant(_28.fld1, 0), 1), 0), 0).2;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(16_usize, 12_usize, Move(_12), 5_usize, Move(_5), 11_usize, Move(_11), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(16_usize, 30_usize, Move(_30), 6_usize, Move(_6), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(16_usize, 14_usize, Move(_14), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [i128; 6],mut _2: [i128; 6],mut _3: [i128; 6],mut _4: [i128; 6]) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _5: isize;
let _6: Adt49;
let _7: *const u16;
let _8: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _9: [i64; 4];
let _10: i16;
let _11: i16;
let _12: Adt43;
let _13: isize;
let _14: [i64; 4];
let _15: [i16; 8];
let _16: [i16; 8];
let _17: f64;
let _18: u64;
let _19: Adt58;
let _20: Adt56;
let _21: isize;
let _22: bool;
let _23: (*mut bool, u32, *mut bool, char);
let _24: f32;
let _25: Adt54;
let _26: i128;
let _27: Adt52;
let _28: [u32; 6];
let _29: f64;
let _30: Adt50;
let _31: [i64; 4];
let _32: ();
let _33: ();
{
RET = [25948858174694688532347522128214772593_i128,159978479302016882333265397234410314711_i128,(-166484499608068208240548064329446507907_i128),(-43404713732822249161070309452902347967_i128),154438572810356200804762375441737936837_i128,(-169940130064715283694143075855057009435_i128)];
_4 = [(-28541864351948768391785219851905967694_i128),(-98340080901699505040378164445446643951_i128),88538591873886385514463314917958065485_i128,(-3395385698803855869198808351174162998_i128),(-130054837260950736987355515588790248091_i128),154743747431849966451035642653091478910_i128];
RET = [(-47933389629878572378754150863528948674_i128),79673720519240800703200651328904076482_i128,(-27323379347508112167507337329559176695_i128),165799425995478666458998804199329298242_i128,43594448393250221314411191908897634147_i128,30707566045504079969446025945004438989_i128];
RET = [(-34201338862049472760736033108291232558_i128),(-119829240650849494061432131423785501635_i128),65010520679413359416647008611688311438_i128,(-2672023755266206984504483853252381951_i128),(-138593170615309027764779170915409407295_i128),(-119618033245935087687875049321523964239_i128)];
_1 = [(-131834601804399556852777521085919378171_i128),(-39057768643789676720597357015940630395_i128),130305973721914294965560446175686221246_i128,(-86734358154037638737077137032060508289_i128),(-28779587384561584815566464412144000445_i128),(-51883168051575100189343181774204706419_i128)];
RET = _2;
RET = [(-37975613558816407117273019449253151919_i128),(-111115561884257419965140064569880514488_i128),(-88431453298683342020236155154407845825_i128),(-75518607868597942671256027442299410978_i128),(-103099880472378107507364634650832547932_i128),13682117371249601227292422976427766219_i128];
_3 = RET;
_8.2.2 = [(-22056_i16),9548_i16,18861_i16,(-20630_i16),18849_i16,(-13782_i16),5849_i16,(-31129_i16)];
RET = [(-116276795454668670266169896949122190505_i128),(-26938509781877766113020795498124494155_i128),3094363097905902876982414479132239332_i128,(-169748250409374270963709573471837287603_i128),109836468545528390646557130390288545939_i128,(-93079032805731301495714657035203800302_i128)];
_8.2.1.3 = (-105_i8) as f64;
_1 = [(-112208093604993015567029331294487502616_i128),121064233544123618284119573476933334554_i128,(-102297384490133018160028383617069481086_i128),(-125156177067452581549356527270965486727_i128),(-14255300317570983833042337751528040756_i128),103921403254296257797488057757024144482_i128];
_2 = [158631282641431994397800403950929670169_i128,(-30231141678799445953516501981401769200_i128),(-87592576320796393453311535718311203802_i128),97127719506531710686415604100999207886_i128,32613450250998832162254603338021536159_i128,633952644357733443157116366176215096_i128];
Goto(bb1)
}
bb1 = {
_8.2.1.0 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_7 = core::ptr::addr_of!(_8.0);
_8.2.0 = [(-276849314_i32),2092902257_i32,(-584293681_i32),781585202_i32,(-898078069_i32),(-622960581_i32)];
_8.3 = -2903941009561288062_i64;
_8.0 = 26942_i16 as u16;
_1 = [(-97193082627287049832923008356579651314_i128),112225969278795519306018032071112282793_i128,(-34928866940925764817177444827550638914_i128),45319674037524842134742264822422103271_i128,(-24093317711495078046074377326938615602_i128),(-6572790936120966773126883735965025568_i128)];
_8.2.1.1 = 808000505_i32;
_7 = core::ptr::addr_of!((*_7));
_8.2.2 = [(-16711_i16),(-13695_i16),(-32076_i16),2163_i16,27451_i16,(-13522_i16),23860_i16,6035_i16];
_8.1 = !(*_7);
_5 = _8.2.1.1 as isize;
_8.2.1.3 = 102_u8 as f64;
RET = [106795578966597831715044316585741779639_i128,73849095682862383108663803020555157678_i128,24434085736100857921929547942625813943_i128,73178452976327518902043988708341155716_i128,95100762526156979078048682041002521448_i128,53750882477056797528593171236155170898_i128];
_10 = 21536_i16 ^ (-14419_i16);
_8.2.2 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1 = _3;
_11 = !_10;
(*_7) = _8.1 | _8.1;
RET = _1;
_8.2.1.0 = _5;
_9 = [_8.3,_8.3,_8.3,_8.3];
_8.2.1.2 = '\u{7f53e}' as u64;
_5 = _8.2.1.0;
_8.0 = _8.1 - _8.1;
_2 = [(-99724005363943412466524834540760725081_i128),(-81945084155009996938806391446252045000_i128),(-65381975389028989240448477075315765742_i128),(-165817877915063238581671735843959048575_i128),161558722147312837557628480060476784951_i128,74170056627487416854742197527725768139_i128];
Goto(bb2)
}
bb2 = {
_8.2.2 = [_10,_10,_11,_11,_10,_11,_11,_10];
(*_7) = _8.1;
_9 = [_8.3,_8.3,_8.3,_8.3];
_4 = RET;
RET = _3;
_8.2.2 = [_10,_11,_11,_10,_10,_11,_11,_11];
_3 = _4;
_12 = Adt43::Variant0 { fld0: _8 };
_10 = !_11;
_8.2.2 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.3 = -_8.2.1.3;
_4 = RET;
SetDiscriminant(_12, 0);
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.2 = !_8.2.1.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.0 = _8.2.0;
_14 = [_8.3,_8.3,_8.3,_8.3];
_13 = _8.2.1.0 - _5;
_8.2.2 = [_11,_11,_11,_10,_10,_11,_10,_11];
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2 = _8.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1 = (_13, _8.2.1.1, _8.2.1.2, _8.2.1.3);
match Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1 {
808000505 => bb3,
_ => bb1
}
}
bb3 = {
_9 = _14;
_11 = _8.2.1.0 as i16;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).0 = 79656524392505392633214349424906146599_u128 as u16;
_8.2.0 = [Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,_8.2.1.1,_8.2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1];
Call(place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.0 = fn18(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1, _13, (*_7), _1, _8.2, _3, _5, _1, (*_7), _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _1;
_16 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
_8.2.1 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1;
_3 = [92461595119881510850290215177474240296_i128,107900477727354201304659630303160954969_i128,(-51278028336922540083856916273837540070_i128),105963149492712627290569386371124043960_i128,104954109610273104998736860104017768589_i128,(-86305981434139609405784588801366651911_i128)];
_20.fld0 = _7;
_10 = _11 | _11;
_21 = _13;
_8.3 = -2396389227309413048_i64;
_8 = (Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2, (-853638644684830600_i64));
_20.fld6 = [Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0,(*_7)];
_18 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.2;
_17 = 6_usize as f64;
_18 = !Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.2;
Call(place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.0 = core::intrinsics::transmute(_8.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
_8.0 = _8.1;
_13 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.0;
RET = _4;
_8.2.1.0 = '\u{ba6bb}' as isize;
_8.2.1.2 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.2 >> (*_7);
_1 = [114296034663934264286523242443350712700_i128,(-127057940794159934848747705126051510329_i128),97925246189054438104423463631657474213_i128,(-50519093952639515669345290479608616181_i128),(-104788285278448332849102111216145127578_i128),22915709313607654065230677083562222435_i128];
_2 = [103090256468400648612961544673220917304_i128,(-16249577387066410003288173598001378180_i128),(-153949293150144851585169840489627199876_i128),67535951510798223584449601283135834682_i128,(-144581808554905465015640964784877886999_i128),69371145575718394363786768623461984105_i128];
_8 = (Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2, 2286914912172669426_i64);
_11 = !_10;
_22 = _21 < Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.0;
_8.2.2 = _16;
_8.2.1.0 = -Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.0;
_14 = _9;
match _8.3 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
2286914912172669426 => bb12,
_ => bb11
}
}
bb6 = {
_4 = _1;
_16 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
_8.2.1 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1;
_3 = [92461595119881510850290215177474240296_i128,107900477727354201304659630303160954969_i128,(-51278028336922540083856916273837540070_i128),105963149492712627290569386371124043960_i128,104954109610273104998736860104017768589_i128,(-86305981434139609405784588801366651911_i128)];
_20.fld0 = _7;
_10 = _11 | _11;
_21 = _13;
_8.3 = -2396389227309413048_i64;
_8 = (Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2, (-853638644684830600_i64));
_20.fld6 = [Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0,(*_7)];
_18 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.2;
_17 = 6_usize as f64;
_18 = !Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.2;
Call(place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.0 = core::intrinsics::transmute(_8.3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_9 = _14;
_11 = _8.2.1.0 as i16;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).0 = 79656524392505392633214349424906146599_u128 as u16;
_8.2.0 = [Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,_8.2.1.1,_8.2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1];
Call(place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.0 = fn18(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1, _13, (*_7), _1, _8.2, _3, _5, _1, (*_7), _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_8.2.2 = [_10,_10,_11,_11,_10,_11,_11,_10];
(*_7) = _8.1;
_9 = [_8.3,_8.3,_8.3,_8.3];
_4 = RET;
RET = _3;
_8.2.2 = [_10,_11,_11,_10,_10,_11,_11,_11];
_3 = _4;
_12 = Adt43::Variant0 { fld0: _8 };
_10 = !_11;
_8.2.2 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.3 = -_8.2.1.3;
_4 = RET;
SetDiscriminant(_12, 0);
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1.2 = !_8.2.1.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.0 = _8.2.0;
_14 = [_8.3,_8.3,_8.3,_8.3];
_13 = _8.2.1.0 - _5;
_8.2.2 = [_11,_11,_11,_10,_10,_11,_10,_11];
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2 = _8.2;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2.1 = (_13, _8.2.1.1, _8.2.1.2, _8.2.1.3);
match Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.1 {
808000505 => bb3,
_ => bb1
}
}
bb9 = {
_8.2.1.0 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_7 = core::ptr::addr_of!(_8.0);
_8.2.0 = [(-276849314_i32),2092902257_i32,(-584293681_i32),781585202_i32,(-898078069_i32),(-622960581_i32)];
_8.3 = -2903941009561288062_i64;
_8.0 = 26942_i16 as u16;
_1 = [(-97193082627287049832923008356579651314_i128),112225969278795519306018032071112282793_i128,(-34928866940925764817177444827550638914_i128),45319674037524842134742264822422103271_i128,(-24093317711495078046074377326938615602_i128),(-6572790936120966773126883735965025568_i128)];
_8.2.1.1 = 808000505_i32;
_7 = core::ptr::addr_of!((*_7));
_8.2.2 = [(-16711_i16),(-13695_i16),(-32076_i16),2163_i16,27451_i16,(-13522_i16),23860_i16,6035_i16];
_8.1 = !(*_7);
_5 = _8.2.1.1 as isize;
_8.2.1.3 = 102_u8 as f64;
RET = [106795578966597831715044316585741779639_i128,73849095682862383108663803020555157678_i128,24434085736100857921929547942625813943_i128,73178452976327518902043988708341155716_i128,95100762526156979078048682041002521448_i128,53750882477056797528593171236155170898_i128];
_10 = 21536_i16 ^ (-14419_i16);
_8.2.2 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1 = _3;
_11 = !_10;
(*_7) = _8.1 | _8.1;
RET = _1;
_8.2.1.0 = _5;
_9 = [_8.3,_8.3,_8.3,_8.3];
_8.2.1.2 = '\u{7f53e}' as u64;
_5 = _8.2.1.0;
_8.0 = _8.1 - _8.1;
_2 = [(-99724005363943412466524834540760725081_i128),(-81945084155009996938806391446252045000_i128),(-65381975389028989240448477075315765742_i128),(-165817877915063238581671735843959048575_i128),161558722147312837557628480060476784951_i128,74170056627487416854742197527725768139_i128];
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
(*_7) = !Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0;
_23.1 = (*_7) as u32;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).1 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0 * Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0;
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)) = ((*_7), (*_7), _8.2, _8.3);
_14 = [_8.3,Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).3,_8.3,_8.3];
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).2 = (_8.2.0, _8.2.1, _16);
_23.0 = core::ptr::addr_of_mut!(_22);
_8 = (Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).1, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).0, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).3);
place!(Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0)).3 = _8.3 >> Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.1.0;
_23.3 = '\u{91f76}';
_23.2 = _23.0;
_8.2 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2;
_8.2.1.2 = _8.3 as u64;
_8.2.1.3 = _13 as f64;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_8.3 = _23.1 as i64;
_8.2.1.2 = _18;
_20.fld5 = 208_u8 - 221_u8;
_8.2.2 = Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2.2;
_23.3 = '\u{81d0d}';
_8 = (Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).1, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).1, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).2, Field::<(u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64)>(Variant(_12, 0), 0).3);
SetDiscriminant(_12, 2);
_3 = [(-55718741210473188213197711204658172509_i128),128625820521022527035713129918380516316_i128,68181678517154175127984277463316163861_i128,83299688216006094324615891432793563546_i128,10674871913857383362749594654888189097_i128,67743631579439676734099108471062555648_i128];
Goto(bb14)
}
bb14 = {
_2 = [(-38089904756713722026212422145206870966_i128),(-83861511253510145895895066556177788692_i128),(-84347119005220584029763121830136319020_i128),158822353289453897453517465858831422166_i128,29667835022267913060314093354676955877_i128,21432056634204717916815869138311282080_i128];
_1 = _4;
_15 = [_11,_11,_10,_10,_10,_11,_11,_10];
_20.fld6 = [(*_7),(*_7)];
_23.2 = _23.0;
place!(Field::<(u32,)>(Variant(_12, 2), 4)) = (_23.1,);
place!(Field::<*const u16>(Variant(_12, 2), 5)) = _20.fld0;
_20.fld5 = 12_u8 | 199_u8;
RET = [(-102690226693709166222508047020782740959_i128),53151295504614449614932471175492634572_i128,153586493922806566074790759400769381732_i128,135680863346514211762193169710285740347_i128,151338601017199371903537592449797404884_i128,(-126172005022276134993727984899933610182_i128)];
_7 = Field::<*const u16>(Variant(_12, 2), 5);
_26 = (-151020428737196550978745486759600896244_i128) ^ (-23027965458237852424337489066165203330_i128);
place!(Field::<*const u16>(Variant(_12, 2), 5)) = _20.fld0;
_20.fld5 = 96_u8;
_11 = !_10;
RET = [_26,_26,_26,_26,_26,_26];
_14 = _9;
_28 = [Field::<(u32,)>(Variant(_12, 2), 4).0,Field::<(u32,)>(Variant(_12, 2), 4).0,_23.1,Field::<(u32,)>(Variant(_12, 2), 4).0,Field::<(u32,)>(Variant(_12, 2), 4).0,Field::<(u32,)>(Variant(_12, 2), 4).0];
_31 = _9;
_23.2 = core::ptr::addr_of_mut!(_22);
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(17_usize, 1_usize, Move(_1), 15_usize, Move(_15), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(17_usize, 13_usize, Move(_13), 14_usize, Move(_14), 21_usize, Move(_21), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(17_usize, 3_usize, Move(_3), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: (isize, i32, u64, f64),mut _2: isize,mut _3: u16,mut _4: [i128; 6],mut _5: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),mut _6: [i128; 6],mut _7: isize,mut _8: [i128; 6],mut _9: u16,mut _10: isize,mut _11: [i128; 6]) -> [i32; 6] {
mir! {
type RET = [i32; 6];
let _12: char;
let _13: [u32; 6];
let _14: u64;
let _15: f32;
let _16: *mut *const (isize, *mut bool);
let _17: i128;
let _18: i32;
let _19: isize;
let _20: i8;
let _21: isize;
let _22: bool;
let _23: u32;
let _24: (i64,);
let _25: f32;
let _26: (*const i128, (f32, (isize, i32, u64, f64)), *const i128);
let _27: ([i32; 6], (isize, i32, u64, f64), [i16; 8]);
let _28: isize;
let _29: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _30: isize;
let _31: isize;
let _32: i32;
let _33: f32;
let _34: *mut *mut *const (isize, *mut bool);
let _35: isize;
let _36: usize;
let _37: f32;
let _38: f32;
let _39: i8;
let _40: [i128; 6];
let _41: u128;
let _42: isize;
let _43: isize;
let _44: [char; 8];
let _45: [u16; 2];
let _46: i128;
let _47: (isize, i32, u64, f64);
let _48: [i64; 4];
let _49: i32;
let _50: f64;
let _51: u64;
let _52: Adt57;
let _53: bool;
let _54: *mut [u32; 6];
let _55: [i128; 6];
let _56: [char; 8];
let _57: (isize, i32, u64, f64);
let _58: isize;
let _59: Adt43;
let _60: [char; 8];
let _61: u16;
let _62: [i16; 8];
let _63: Adt50;
let _64: u16;
let _65: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64);
let _66: f64;
let _67: i16;
let _68: isize;
let _69: u16;
let _70: isize;
let _71: usize;
let _72: ();
let _73: ();
{
_7 = _9 as isize;
RET = _5.0;
_9 = _1.2 as u16;
_5.0 = RET;
_2 = '\u{ab0af}' as isize;
_5.1.2 = _1.2 ^ _1.2;
_5.1.1 = 95_i8 as i32;
_5.1.3 = (-5429285433548030409_i64) as f64;
_1 = (_2, _5.1.1, _5.1.2, _5.1.3);
RET = [_1.1,_5.1.1,_5.1.1,_1.1,_5.1.1,_1.1];
_5.1.2 = _1.2;
_4 = [94409595451949371808135911045302009785_i128,30337143242745624604098307380522677305_i128,(-121860936604440567808795172655988162545_i128),(-67226907319439168131299030970303731432_i128),27309233271755106448879280852204840078_i128,39278975236923007677693896541902220141_i128];
_5.0 = RET;
_1.1 = _3 as i32;
_6 = [(-38122167156000864869550476189768452631_i128),110573449079452258385605340808783506435_i128,46847365126160402889321517097558537775_i128,41803944897639919876757267172002615789_i128,(-118089395550595439243757998595159640675_i128),111953940881694174335004350586625068324_i128];
_5.1.0 = _1.3 as isize;
_5.1.1 = !_1.1;
_14 = _1.2 + _5.1.2;
RET = _5.0;
_13 = [576733470_u32,304669409_u32,2858957115_u32,3803702012_u32,2257176820_u32,3674524413_u32];
Goto(bb1)
}
bb1 = {
_1.0 = 152454933534681167301252052358620714158_i128 as isize;
_19 = !_5.1.0;
_18 = _1.1 | _1.1;
Goto(bb2)
}
bb2 = {
_1.2 = _5.1.2;
_1 = (_2, _18, _14, _5.1.3);
_1.2 = _5.1.2;
_5.1 = (_1.0, _1.1, _14, _1.3);
_5.1.2 = !_14;
_15 = _9 as f32;
_1.1 = _5.1.1 + _5.1.1;
Goto(bb3)
}
bb3 = {
_17 = -145383286570747991007740573157343976429_i128;
_10 = _19 << _5.1.0;
_20 = 10_i8 - 97_i8;
_15 = 2180891731765509130_usize as f32;
_4 = _8;
_5.1 = (_7, _18, _1.2, _1.3);
_25 = -_15;
_15 = _5.1.3 as f32;
_21 = _5.1.0 - _7;
_14 = !_5.1.2;
_24.0 = (-1484781121268138821_i64) - (-2785672614317798576_i64);
_5.2 = [10725_i16,(-26671_i16),(-11913_i16),(-19983_i16),15152_i16,(-492_i16),(-29944_i16),(-11973_i16)];
_27.0 = [_5.1.1,_18,_18,_1.1,_18,_1.1];
_9 = _3 * _3;
_5.1.2 = true as u64;
_29.2 = (RET, _1, _5.2);
_5 = (_27.0, _29.2.1, _29.2.2);
_28 = _5.1.1 as isize;
_5.1 = (_2, _1.1, _1.2, _29.2.1.3);
_5 = (RET, _1, _29.2.2);
_5.0 = _27.0;
_22 = _10 < _21;
Call(_26.1.1.1 = core::intrinsics::transmute(_1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_19 = _5.1.0 ^ _7;
_26.1.1 = (_21, _5.1.1, _1.2, _1.3);
_26.1.1.1 = _29.2.1.1;
_29.2.0 = [_18,_29.2.1.1,_29.2.1.1,_18,_26.1.1.1,_26.1.1.1];
Goto(bb5)
}
bb5 = {
_8 = [_17,_17,_17,_17,_17,_17];
_12 = '\u{932e}';
_27.1.0 = _7;
RET = [_18,_26.1.1.1,_18,_5.1.1,_1.1,_5.1.1];
_5.1 = (_28, _29.2.1.1, _14, _26.1.1.3);
_5.1.2 = !_26.1.1.2;
_15 = _25 + _25;
_32 = 321804704223685027675926340667980037703_u128 as i32;
_24.0 = _28 as i64;
_26.1.1.3 = _5.1.3;
_27.1.1 = !_29.2.1.1;
_26.1 = (_15, _29.2.1);
_26.0 = core::ptr::addr_of!(_17);
Goto(bb6)
}
bb6 = {
_26.2 = _26.0;
_15 = _26.1.1.1 as f32;
_23 = _20 as u32;
RET = _29.2.0;
_27.1 = (_5.1.0, _1.1, _26.1.1.2, _29.2.1.3);
_11 = _4;
_29.2.2 = [26019_i16,15546_i16,17720_i16,9295_i16,(-21272_i16),(-23351_i16),5133_i16,(-24109_i16)];
_29.2.1.2 = _27.1.2 + _26.1.1.2;
_5.1.3 = _27.1.3 + _29.2.1.3;
_1.1 = _5.1.1;
_29.2.1.0 = -_27.1.0;
_26.1.1.2 = 24706_i16 as u64;
_21 = _27.1.0;
_37 = _26.1.0 - _26.1.0;
_29.3 = _27.1.3 as i64;
_37 = _26.1.0 * _25;
_29.3 = _24.0;
_40 = _11;
RET = _27.0;
_5.1.1 = -_1.1;
_26.1.1.2 = _27.1.2 + _29.2.1.2;
Call(_8 = core::intrinsics::transmute(_40), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5.1.0 = _28;
_41 = _26.1.1.1 as u128;
_27 = _29.2;
_14 = !_26.1.1.2;
Goto(bb8)
}
bb8 = {
_26.2 = core::ptr::addr_of!(_17);
_26.2 = _26.0;
_29.2.2 = [(-30510_i16),(-6464_i16),(-12111_i16),5809_i16,(-8558_i16),(-25942_i16),29058_i16,18727_i16];
_33 = -_15;
_42 = _5.1.0;
_43 = _42 >> _21;
_27.0 = [_18,_18,_32,_1.1,_29.2.1.1,_29.2.1.1];
_39 = 32161_i16 as i8;
_6 = [_17,_17,_17,_17,_17,_17];
_9 = _12 as u16;
_29 = (_9, _3, _5, _24.0);
_38 = _26.1.0;
_47.3 = _5.1.3 + _5.1.3;
_40 = [_17,_17,_17,_17,_17,_17];
_18 = !_5.1.1;
Call(_37 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_27.1.1 = _5.1.1 + _5.1.1;
_1.2 = !_14;
_47.0 = _29.2.1.0 * _21;
_5.0 = [_5.1.1,_27.1.1,_27.1.1,_27.1.1,_5.1.1,_18];
_28 = _43;
RET = [_5.1.1,_5.1.1,_5.1.1,_27.1.1,_5.1.1,_27.1.1];
_22 = true;
_37 = _33 + _33;
_5.2 = _27.2;
_18 = _26.1.1.1;
_1.2 = !_26.1.1.2;
_29.2.1.2 = _14;
_46 = !_17;
_34 = core::ptr::addr_of_mut!(_16);
_26.1.1.3 = _20 as f64;
_29.2.2 = [(-20355_i16),(-13918_i16),16852_i16,22774_i16,(-28894_i16),25282_i16,21643_i16,22224_i16];
_29.2.2 = [17923_i16,(-13340_i16),(-5058_i16),(-26301_i16),(-2129_i16),928_i16,(-24031_i16),(-2582_i16)];
_31 = !_27.1.0;
_11 = _8;
_19 = _27.1.0;
Goto(bb10)
}
bb10 = {
_29.2.1.2 = _5.1.3 as u64;
_45 = [_9,_3];
_5 = (_29.2.0, _1, _29.2.2);
_5.2 = [(-10651_i16),18689_i16,(-27280_i16),(-22373_i16),(-10671_i16),24683_i16,24243_i16,(-31598_i16)];
_29.2.1.3 = _47.3;
_4 = [_46,_17,_46,_46,_46,_17];
_1.0 = _47.0 | _21;
RET = [_1.1,_27.1.1,_27.1.1,_18,_27.1.1,_18];
_29.3 = _24.0 << _28;
_29.2.1.3 = _1.3;
_36 = 7_usize;
_27.2 = _29.2.2;
_26.0 = core::ptr::addr_of!(_17);
_26.1.1.2 = _22 as u64;
_9 = _29.2.1.3 as u16;
_34 = core::ptr::addr_of_mut!((*_34));
_27.1.0 = -_1.0;
_29.2.1.2 = !_27.1.2;
_1 = _29.2.1;
_3 = _29.0 & _29.1;
_18 = _26.1.1.1 | _26.1.1.1;
_9 = !_29.1;
_11 = [_17,_17,_17,_17,_17,_46];
_20 = -_39;
Goto(bb11)
}
bb11 = {
_31 = _29.2.2[_36] as isize;
_27.1.0 = _47.0 ^ _47.0;
Goto(bb12)
}
bb12 = {
_14 = _5.1.2;
_27.2[_36] = _29.2.2[_36];
_29.2.1.2 = _5.1.1 as u64;
_5.1.0 = _26.1.1.1 as isize;
_49 = _18 + _18;
_5.1.1 = !_27.1.1;
_29.2.0 = [_49,_1.1,_49,_26.1.1.1,_5.1.1,_18];
_57.2 = _27.1.2;
_44[_36] = _12;
_44 = [_12,_12,_12,_12,_12,_12,_12,_12];
_26.1.1.0 = !_21;
_9 = _29.1;
_7 = _39 as isize;
_45 = [_9,_3];
_41 = 157024542124485857301132089808117829048_u128;
_40 = [_17,_17,_46,_46,_17,_46];
_29.2.0 = [_49,_18,_27.1.1,_5.1.1,_49,_27.1.1];
_57.1 = _9 as i32;
_30 = _43 | _5.1.0;
Goto(bb13)
}
bb13 = {
_6 = [_46,_17,_46,_17,_46,_46];
_61 = _27.2[_36] as u16;
_10 = _28 + _30;
_15 = _10 as f32;
_26.1.1.1 = _49 * _18;
_5.1 = _26.1.1;
_39 = _41 as i8;
_27.1 = (_30, _49, _29.2.1.2, _47.3);
_6 = [_17,_17,_17,_17,_46,_17];
_56 = [_44[_36],_12,_12,_44[_36],_12,_44[_36],_44[_36],_12];
_10 = _30 | _29.2.1.0;
_65.2.2 = [_27.2[_36],_29.2.2[_36],_27.2[_36],_27.2[_36],_5.2[_36],_29.2.2[_36],_5.2[_36],_29.2.2[_36]];
_65 = _29;
_44[_36] = _56[_36];
_65.2.2[_36] = _5.2[_36];
_40 = [_17,_46,_17,_17,_17,_46];
_60 = [_12,_44[_36],_12,_56[_36],_56[_36],_12,_12,_56[_36]];
_26.1.1.2 = _22 as u64;
_27.2[_36] = _36 as i16;
_5.1.1 = _65.2.1.1;
_26.2 = _26.0;
_27.1.1 = _18;
_36 = 8038437088515070860_usize;
Call(_27.1 = fn19(_26, _26.1.1, _15, _42, _26.0, _29, _8, _26.1.1, _31, _5, _26.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_5.1.3 = _65.2.1.3 - _65.2.1.3;
_1.1 = _26.1.1.1 + _27.1.1;
_57.2 = _14 | _14;
_47 = (_43, _26.1.1.1, _1.2, _5.1.3);
_65.1 = !_61;
_41 = 5611777971927536275678678784065678298_u128;
_64 = !_65.1;
_57 = (_43, _1.1, _65.2.1.2, _27.1.3);
_10 = _27.1.0;
_32 = _1.1;
_26.1.1.0 = -_27.1.0;
_8 = [_17,_46,_17,_17,_17,_17];
_68 = _10 >> _57.1;
Goto(bb15)
}
bb15 = {
Call(_72 = dump_var(18_usize, 36_usize, Move(_36), 14_usize, Move(_14), 45_usize, Move(_45), 61_usize, Move(_61)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_72 = dump_var(18_usize, 42_usize, Move(_42), 56_usize, Move(_56), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_72 = dump_var(18_usize, 12_usize, Move(_12), 39_usize, Move(_39), 10_usize, Move(_10), 46_usize, Move(_46)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_72 = dump_var(18_usize, 23_usize, Move(_23), 3_usize, Move(_3), 41_usize, Move(_41), 68_usize, Move(_68)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_72 = dump_var(18_usize, 21_usize, Move(_21), 32_usize, Move(_32), 30_usize, Move(_30), 73_usize, _73), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),mut _2: (isize, i32, u64, f64),mut _3: f32,mut _4: isize,mut _5: *const i128,mut _6: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64),mut _7: [i128; 6],mut _8: (isize, i32, u64, f64),mut _9: isize,mut _10: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),mut _11: (f32, (isize, i32, u64, f64))) -> (isize, i32, u64, f64) {
mir! {
type RET = (isize, i32, u64, f64);
let _12: [i128; 6];
let _13: usize;
let _14: isize;
let _15: i64;
let _16: ();
let _17: ();
{
RET.1 = 2787238267_u32 as i32;
_10.1 = (_8.0, _2.1, _6.2.1.2, _1.1.1.3);
_1.1.0 = _6.0 as f32;
_11.1.1 = _10.1.1 << _10.1.1;
_6.2.1.1 = !_8.1;
_2.0 = -_11.1.0;
_1.1.1 = (_4, _10.1.1, _6.2.1.2, _10.1.3);
_1.1.1.2 = 282703010889524993941096777014352292645_u128 as u64;
_4 = _1.1.1.0 << _1.1.1.1;
_10.0 = [_1.1.1.1,_11.1.1,_8.1,_6.2.1.1,_1.1.1.1,_2.1];
_6.0 = _6.1 >> _11.1.1;
_2.3 = 1_i8 as f64;
_6.2.1.2 = true as u64;
_11.1.0 = _4;
_10.1.2 = _8.2;
_10.1 = _2;
RET = (_4, _8.1, _6.2.1.2, _8.3);
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(19_usize, 9_usize, Move(_9), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(807_u16), std::hint::black_box(248699661012619531079636256814775005303_u128), std::hint::black_box((-119_isize)), std::hint::black_box(74_i8), std::hint::black_box(7937449176785789185_u64), std::hint::black_box(274236966_i32), std::hint::black_box((-1527109093063678313_i64)), std::hint::black_box(39242328114753855286337084110072968055_i128), std::hint::black_box(85_u8));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64),

},
Variant1{
fld0: u128,
fld1: [i64; 4],
fld2: isize,
fld3: (u32,),
fld4: [char; 8],
fld5: u8,
fld6: i64,

},
Variant2{
fld0: *const ([i32; 6], (isize, i32, u64, f64), [i16; 8]),
fld1: [i16; 8],
fld2: [u16; 2],
fld3: *mut *mut *const (isize, *mut bool),
fld4: (u32,),
fld5: *const u16,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut *mut *const (isize, *mut bool),
fld1: u128,

},
Variant1{
fld0: i128,
fld1: *mut (isize, *mut bool),
fld2: [i16; 8],
fld3: i32,
fld4: (*mut bool, u32, *mut bool, char),

},
Variant2{
fld0: Adt43,
fld1: f64,
fld2: u32,
fld3: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),
fld4: f32,
fld5: usize,

},
Variant3{
fld0: *mut *mut *const (isize, *mut bool),
fld1: f64,
fld2: *const ([i32; 6], (isize, i32, u64, f64), [i16; 8]),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut *mut *const (isize, *mut bool),
fld1: char,

},
Variant1{
fld0: u16,

},
Variant2{
fld0: i64,
fld1: f64,
fld2: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),
fld3: u16,
fld4: *mut (isize, *mut bool),
fld5: (u16, u16, ([i32; 6], (isize, i32, u64, f64), [i16; 8]), i64),

},
Variant3{
fld0: i32,
fld1: *mut [u32; 6],
fld2: Adt43,
fld3: u32,
fld4: Adt44,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
fld0: [i32; 6],
fld1: *const u16,
fld2: isize,
fld3: i8,
fld4: [i16; 8],

},
Variant1{
fld0: i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (isize, i32, u64, f64),
fld1: ([i32; 6], (isize, i32, u64, f64), [i16; 8]),
fld2: i16,
fld3: [i16; 8],

},
Variant1{
fld0: u128,
fld1: f32,
fld2: [i32; 6],
fld3: u8,
fld4: i16,
fld5: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),
fld6: *mut *mut *const (isize, *mut bool),

},
Variant2{
fld0: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),

},
Variant3{
fld0: usize,
fld1: u64,
fld2: [char; 8],
fld3: *mut *mut *const (isize, *mut bool),
fld4: [i16; 8],
fld5: *const i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (i64,),
fld1: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),
fld2: *mut [u32; 6],

},
Variant1{
fld0: [i32; 6],
fld1: char,
fld2: *mut (isize, *mut bool),

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i32; 6],
fld1: Adt47,
fld2: i128,
fld3: i64,

},
Variant1{
fld0: bool,
fld1: usize,
fld2: *mut *const (isize, *mut bool),
fld3: [i32; 6],
fld4: (isize, *mut bool),
fld5: *mut (isize, *mut bool),
fld6: (i64,),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [i64; 4],
fld1: u64,
fld2: isize,
fld3: f64,
fld4: Adt45,

},
Variant1{
fld0: bool,
fld1: Adt49,
fld2: [u16; 2],
fld3: f32,
fld4: [i64; 4],
fld5: i32,
fld6: i64,
fld7: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),

},
Variant2{
fld0: (i64,),
fld1: i32,
fld2: isize,
fld3: i8,
fld4: u32,

},
Variant3{
fld0: *const (isize, *mut bool),
fld1: *const i128,
fld2: Adt49,
fld3: [i128; 6],
fld4: u128,
fld5: f32,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (isize, i32, u64, f64),
fld1: (f32, (isize, i32, u64, f64)),

},
Variant1{
fld0: *const u16,
fld1: Adt48,
fld2: isize,
fld3: (f32, (isize, i32, u64, f64)),

},
Variant2{
fld0: i64,
fld1: u8,
fld2: (*mut bool, u32, *mut bool, char),
fld3: [u16; 2],
fld4: *mut *mut *const (isize, *mut bool),

},
Variant3{
fld0: Adt45,
fld1: (u32,),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt44,
fld1: Adt51,

},
Variant1{
fld0: Adt51,
fld1: [i128; 6],
fld2: [i16; 8],
fld3: Adt43,
fld4: (u32,),
fld5: u32,
fld6: f64,
fld7: Adt48,

},
Variant2{
fld0: (isize, i32, u64, f64),
fld1: i32,
fld2: u32,
fld3: f32,
fld4: [i128; 6],

},
Variant3{
fld0: f32,
fld1: Adt45,
fld2: i128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
fld0: Adt45,
fld1: *mut bool,

},
Variant1{
fld0: Adt44,
fld1: i32,
fld2: u16,
fld3: *mut bool,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt51,
fld1: (isize, *mut bool),
fld2: Adt53,

},
Variant1{
fld0: *const u16,
fld1: *mut bool,
fld2: Adt52,
fld3: (f32, (isize, i32, u64, f64)),
fld4: *mut *const (isize, *mut bool),

},
Variant2{
fld0: *const u16,
fld1: *const ([i32; 6], (isize, i32, u64, f64), [i16; 8]),
fld2: *mut [u32; 6],
fld3: Adt50,
fld4: i16,
fld5: (*const i128, (f32, (isize, i32, u64, f64)), *const i128),
fld6: u64,
fld7: (*mut bool, u32, *mut bool, char),

},
Variant3{
fld0: bool,
fld1: (f32, (isize, i32, u64, f64)),
fld2: isize,
fld3: i8,
fld4: Adt47,
fld5: [i64; 4],
fld6: [i16; 8],
fld7: *const ([i32; 6], (isize, i32, u64, f64), [i16; 8]),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: (i64,),

},
Variant1{
fld0: Adt44,

},
Variant2{
fld0: *mut *const (isize, *mut bool),
fld1: char,
fld2: *const u16,

},
Variant3{
fld0: i32,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: *const u16,
fld1: Adt49,
fld2: *mut (isize, *mut bool),
fld3: i8,
fld4: Adt50,
fld5: u8,
fld6: [u16; 2],
fld7: Adt54,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: Adt49,
}
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt45,
fld1: Adt57,
fld2: (*mut bool, u32, *mut bool, char),
fld3: i8,

},
Variant1{
fld0: Adt48,
fld1: Adt49,
fld2: Adt47,
fld3: (f32, (isize, i32, u64, f64)),

},
Variant2{
fld0: u64,

},
Variant3{
fld0: (*mut bool, u32, *mut bool, char),
fld1: u128,
fld2: Adt47,
fld3: [i128; 6],
fld4: *mut [u32; 6],
fld5: usize,
fld6: [i64; 4],
fld7: i128,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: [i128; 6],
fld1: i128,

},
Variant1{
fld0: Adt51,
fld1: u64,

},
Variant2{
fld0: Adt58,
fld1: Adt49,

}}

