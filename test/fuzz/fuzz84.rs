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
pub fn fn0(mut _1: u32,mut _2: char) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _3: i8;
let _4: [u32; 5];
let _5: u16;
let _6: [u8; 4];
let _7: *const &'static &'static (u32, f32);
let _8: bool;
let _9: u32;
let _10: char;
let _11: u128;
let _12: &'static Adt41;
let _13: isize;
let _14: [i128; 8];
let _15: Adt48;
let _16: i8;
let _17: [bool; 3];
let _18: [i64; 3];
let _19: &'static &'static i16;
let _20: isize;
let _21: char;
let _22: *mut &'static Adt41;
let _23: ((Adt38,), Adt38, &'static i16, isize);
let _24: isize;
let _25: i16;
let _26: *const *const i16;
let _27: u8;
let _28: ();
let _29: ();
{
RET = [1328311494_i32,(-993844252_i32),1308619950_i32,1005664471_i32,402107633_i32,(-2075046819_i32),(-1724027325_i32),1874356201_i32];
RET = [936583596_i32,1407674114_i32,(-340993880_i32),476936184_i32,670464829_i32,1544132662_i32,942645049_i32,1680960179_i32];
RET = [342207540_i32,(-1220696780_i32),1950594597_i32,(-844101731_i32),2027597675_i32,(-1950689109_i32),935713586_i32,224764520_i32];
RET = [(-610905887_i32),1408518406_i32,89573646_i32,1872546110_i32,1525340812_i32,(-1956599042_i32),(-1974592445_i32),784044103_i32];
RET = [2127890402_i32,(-543386701_i32),(-393732486_i32),1384187937_i32,681128490_i32,(-1011093265_i32),(-1543270476_i32),1685942889_i32];
_2 = '\u{5a299}';
Call(RET = fn1(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = '\u{dc21d}';
_2 = '\u{b3454}';
_1 = 4020794934_u32 * 2171104433_u32;
_3 = 58_i8;
_2 = '\u{2a5a8}';
_4 = [_1,_1,_1,_1,_1];
RET = [(-260362218_i32),(-1312538941_i32),(-1096075494_i32),(-1728071020_i32),(-972266010_i32),1992201557_i32,(-98243115_i32),(-1090363015_i32)];
Goto(bb2)
}
bb2 = {
_3 = (-27310_i16) as i8;
_3 = _2 as i8;
RET = [67311109_i32,189437602_i32,(-233537333_i32),2005659015_i32,(-1852341992_i32),1279040506_i32,1112829593_i32,(-2120798577_i32)];
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_3 = 18_i8;
_6 = [214_u8,65_u8,239_u8,230_u8];
_5 = 61312_u16;
RET = [(-1570750456_i32),(-775696267_i32),(-1052805147_i32),977483571_i32,1923152204_i32,928963986_i32,1313588701_i32,(-883543174_i32)];
RET = [1654146758_i32,(-1382822658_i32),654954692_i32,(-1474089528_i32),549551429_i32,855138749_i32,(-98654434_i32),(-1694406667_i32)];
_4 = [_1,_1,_1,_1,_1];
_5 = 34887_u16 | 28001_u16;
RET = [(-889860748_i32),(-442600499_i32),1720050883_i32,(-1779274886_i32),509454770_i32,(-195049762_i32),(-1583013106_i32),(-1842413465_i32)];
_2 = '\u{9522c}';
_6 = [84_u8,222_u8,187_u8,88_u8];
Goto(bb5)
}
bb5 = {
RET = [(-2011443113_i32),(-1604305175_i32),997029921_i32,(-2110882301_i32),(-1411178413_i32),(-263531910_i32),(-1240563928_i32),1618059426_i32];
Call(_1 = core::intrinsics::bswap(2933368884_u32), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = 4_i8;
_4 = [_1,_1,_1,_1,_1];
RET = [(-1895819370_i32),1483619648_i32,1917387065_i32,484061813_i32,1046584346_i32,598295597_i32,2061749724_i32,553518464_i32];
RET = [(-100821272_i32),(-1260643717_i32),(-1759785053_i32),406359152_i32,1680220419_i32,62327113_i32,493538273_i32,(-581859109_i32)];
_2 = '\u{5324a}';
Call(_1 = core::intrinsics::bswap(496275065_u32), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = !true;
RET = [962642037_i32,1096953085_i32,2505562_i32,2052529517_i32,(-178725256_i32),(-332846553_i32),1033405508_i32,1392846240_i32];
Goto(bb8)
}
bb8 = {
RET = [(-69692423_i32),2001280249_i32,1049138816_i32,451819108_i32,(-771580495_i32),(-143928045_i32),(-1979723779_i32),1405658626_i32];
_2 = '\u{daa17}';
RET = [1453087445_i32,(-1708741741_i32),64571510_i32,789040781_i32,(-169062079_i32),(-1918973554_i32),595735199_i32,1742631839_i32];
_4 = [_1,_1,_1,_1,_1];
RET = [(-210227245_i32),(-1211707246_i32),(-1932903493_i32),(-1287376435_i32),(-1980915375_i32),1177647294_i32,1268694011_i32,940139189_i32];
RET = [226673869_i32,(-1934653432_i32),(-486756385_i32),1294586291_i32,1765060007_i32,(-959722325_i32),(-640693144_i32),1298869933_i32];
_9 = (-862498530_i32) as u32;
_8 = !false;
_8 = !true;
_14 = [(-133386490825089598495091283226001311748_i128),169488213955269556267152735395606902716_i128,(-82094522376379840081090749245019430817_i128),(-89694423986858538467714926683001626009_i128),(-61501588873274071650192049204627317429_i128),41006936914385486712422984465286303387_i128,(-129747819801730988952279369152495193917_i128),114488565055183374575009003909534660654_i128];
_13 = (-9223372036854775808_isize);
_13 = 7011778959517119055_i64 as isize;
_10 = _2;
_2 = _10;
_2 = _10;
_10 = _2;
_9 = _1 & _1;
_2 = _10;
_3 = _2 as i8;
_6 = [77_u8,39_u8,105_u8,179_u8];
_1 = _8 as u32;
_13 = !9223372036854775807_isize;
_11 = 253460732796281710927382397471685295032_u128 & 150478485849604192465820666366551998630_u128;
Call(_15 = fn19(RET, RET), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_4 = [_9,_9,_1,_9,_9];
_10 = Field::<char>(Variant(_15, 3), 1);
_11 = _2 as u128;
_9 = !_1;
_14 = [89423514726932044371886617843024869771_i128,(-68300397159557133573880314395849305634_i128),95358289409835842504922235516137463152_i128,(-40255749335749512477044037939111409002_i128),(-59443714813915510750343877233781956045_i128),(-67886870382020299688955861842452367309_i128),(-128120920279255461717134911947893286062_i128),(-118313413413218705719227252819040242316_i128)];
_2 = _10;
place!(Field::<char>(Variant(_15, 3), 1)) = _2;
_8 = !Field::<bool>(Variant(_15, 3), 0);
RET = [1927953821_i32,764842495_i32,696053521_i32,(-1993731679_i32),(-399212011_i32),(-216599129_i32),(-437677145_i32),(-609166686_i32)];
_13 = 9223372036854775807_isize >> _5;
Goto(bb10)
}
bb10 = {
_1 = !_9;
SetDiscriminant(_15, 1);
_9 = _1;
_10 = _2;
place!(Field::<[i128; 8]>(Variant(_15, 1), 5)) = _14;
place!(Field::<i8>(Variant(_15, 1), 3)) = 17455381681468060830_u64 as i8;
_14 = [53947328763833551361725092659878115881_i128,(-147014970039920520445165812628182656350_i128),(-35567035405246794478244223169038346514_i128),(-115692008339594781653845004640717581271_i128),149984217307652905303061039509477954619_i128,(-121104486565787197589781730439816244151_i128),(-80671361105412632069646234332017982676_i128),47718860810309173375170293967898529450_i128];
place!(Field::<Adt47>(Variant(_15, 1), 2)) = Adt47::Variant0 { fld0: _6,fld1: _10,fld2: (-9185455048756762943_i64) };
_18 = [(-7873676909180059625_i64),2614371735459097225_i64,2181778088187678526_i64];
RET = [1893316925_i32,752764479_i32,(-100539468_i32),(-489202219_i32),(-812521959_i32),(-1862666784_i32),2009535491_i32,(-1547293971_i32)];
place!(Field::<u128>(Variant(_15, 1), 1)) = (-3691367205851157843_i64) as u128;
_20 = _13 << _3;
place!(Field::<u8>(Variant(_15, 1), 4)) = !131_u8;
_20 = _13;
_15 = Adt48::Variant3 { fld0: _8,fld1: _10 };
_5 = 35910_u16 << _1;
_16 = _3;
Goto(bb11)
}
bb11 = {
_6 = [119_u8,141_u8,103_u8,75_u8];
_2 = _10;
_18 = [(-5804382864698707107_i64),(-4321460581052774868_i64),7635487895414918155_i64];
_5 = _13 as u16;
_4 = [_1,_9,_9,_1,_1];
_5 = Field::<bool>(Variant(_15, 3), 0) as u16;
_23.1.fld2 = _11 as isize;
_1 = !_9;
_23.1.fld3 = 9016645_i32;
_3 = _16 ^ _16;
_8 = !Field::<bool>(Variant(_15, 3), 0);
_9 = !_1;
_17 = [_8,Field::<bool>(Variant(_15, 3), 0),_8];
_24 = 130111691843162634228410377877406247461_i128 as isize;
_4 = [_9,_1,_9,_1,_9];
_2 = _10;
_23.0.0.fld0 = core::ptr::addr_of!(_25);
_23.1.fld3 = _3 as i32;
_9 = _1;
_25 = (-12229_i16) << _3;
_3 = _1 as i8;
_27 = 96_u8;
_22 = core::ptr::addr_of_mut!(_12);
_23.3 = _20 & _23.1.fld2;
_24 = _20;
_23.0.0.fld0 = core::ptr::addr_of!(_25);
_24 = _5 as isize;
Goto(bb12)
}
bb12 = {
_18 = [(-2765497392403448067_i64),7050261989310763108_i64,(-6846079388946239656_i64)];
match _27 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb13,
96 => bb15,
_ => bb14
}
}
bb13 = {
_3 = (-27310_i16) as i8;
_3 = _2 as i8;
RET = [67311109_i32,189437602_i32,(-233537333_i32),2005659015_i32,(-1852341992_i32),1279040506_i32,1112829593_i32,(-2120798577_i32)];
Goto(bb3)
}
bb14 = {
RET = [(-2011443113_i32),(-1604305175_i32),997029921_i32,(-2110882301_i32),(-1411178413_i32),(-263531910_i32),(-1240563928_i32),1618059426_i32];
Call(_1 = core::intrinsics::bswap(2933368884_u32), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_23.0.0.fld2 = _5 as isize;
RET = [_23.1.fld3,_23.1.fld3,_23.1.fld3,_23.1.fld3,_23.1.fld3,_23.1.fld3,_23.1.fld3,_23.1.fld3];
_8 = Field::<bool>(Variant(_15, 3), 0);
_23.0.0.fld3 = _23.1.fld3;
_13 = _25 as isize;
_13 = _3 as isize;
SetDiscriminant(_15, 1);
_19 = &_23.2;
_6 = [_27,_27,_27,_27];
_23.1.fld1 = _10;
_10 = _2;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(0_usize, 14_usize, Move(_14), 5_usize, Move(_5), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(0_usize, 8_usize, Move(_8), 17_usize, Move(_17), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(0_usize, 16_usize, Move(_16), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _2: &'static [u16; 5];
let _3: isize;
let _4: f64;
let _5: i64;
let _6: i32;
let _7: &'static i16;
let _8: char;
let _9: (u32, f32);
let _10: [u8; 4];
let _11: [bool; 3];
let _12: bool;
let _13: i64;
let _14: (u32, f32);
let _15: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _16: i8;
let _17: usize;
let _18: *const i16;
let _19: ((Adt38,), (u32, f32), u128);
let _20: f32;
let _21: [i32; 8];
let _22: [u32; 5];
let _23: [u32; 7];
let _24: [i128; 1];
let _25: &'static bool;
let _26: f32;
let _27: ((u32, i8), u32, [u8; 4], [bool; 3]);
let _28: char;
let _29: u16;
let _30: (&'static Adt27,);
let _31: ();
let _32: ();
{
_1 = '\u{5af1e}';
RET = [736765163_i32,(-636698748_i32),1966067365_i32,523418189_i32,181969295_i32,(-1195933379_i32),(-184722431_i32),1671345840_i32];
_1 = '\u{d5f93}';
_1 = '\u{40d8c}';
RET = [(-1731212008_i32),1231575427_i32,(-1148555587_i32),(-1722634926_i32),1424538763_i32,1651225151_i32,(-1162891335_i32),(-813239217_i32)];
RET = [1503348709_i32,1697240828_i32,(-324415232_i32),(-2091033984_i32),1740247540_i32,(-2109709450_i32),1099102307_i32,1881589326_i32];
_3 = 9223372036854775807_isize;
RET = [2060244764_i32,(-1640255879_i32),1105951590_i32,(-520609433_i32),536829270_i32,30467603_i32,2116842351_i32,(-85185628_i32)];
_3 = 52_isize - 9223372036854775807_isize;
_4 = 33725_u16 as f64;
Call(_4 = fn2(_3, RET, RET, RET, RET, RET, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-60564479_i32),(-1460937765_i32),918541753_i32,(-2064795228_i32),(-1350955771_i32),1521378961_i32,(-328580879_i32),(-2033493925_i32)];
_4 = _3 as f64;
_5 = 18503_u16 as i64;
RET = [65576124_i32,2054820068_i32,(-749476803_i32),408300027_i32,706830203_i32,1537748567_i32,(-206123618_i32),701359745_i32];
RET = [593606940_i32,(-634744244_i32),560926655_i32,(-517519476_i32),(-1521619667_i32),(-2040416725_i32),(-1614971025_i32),1738430523_i32];
_5 = -(-6394782275670522982_i64);
RET = [1547235412_i32,(-516055964_i32),(-1773189772_i32),(-1443856697_i32),(-1247739999_i32),(-396379552_i32),(-1542805984_i32),1500472193_i32];
_1 = '\u{10aa2b}';
_5 = !(-5908436036481898119_i64);
RET = [2068217800_i32,(-840325578_i32),1493426421_i32,179317064_i32,(-942573778_i32),(-1479759025_i32),1294487503_i32,1602211045_i32];
_1 = '\u{d93}';
_5 = 361587507542569751_u64 as i64;
RET = [(-1990246644_i32),1868981315_i32,(-1377246524_i32),668307301_i32,(-1095033857_i32),1388964701_i32,333447110_i32,116242570_i32];
_1 = '\u{36c60}';
_1 = '\u{e33d6}';
_6 = 922555849_i32 >> _5;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_8 = _1;
_6 = 82_u8 as i32;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = _4 as i32;
Goto(bb2)
}
bb2 = {
_6 = (-279112872_i32);
_1 = _8;
_9.0 = !2629851735_u32;
_9.1 = _6 as f32;
_5 = (-1542610934896775531_i64) & 8692965924995758571_i64;
_3 = (-91_isize) - 84_isize;
_8 = _1;
_6 = 682648717_i32 + 534234308_i32;
_8 = _1;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = [151_u8,66_u8,56_u8,164_u8];
_1 = _8;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_11 = [false,true,false];
_1 = _8;
_8 = _1;
Goto(bb3)
}
bb3 = {
_14.0 = !_9.0;
_9.1 = 12270319844568215511_u64 as f32;
_3 = (-86_i8) as isize;
_15.0.3.3 = 9752264915554848955_usize;
_15.0.2.fld3 = (_14.0, _9.1);
_10 = [50_u8,89_u8,154_u8,158_u8];
_15.0.2.fld0 = _4;
_15.0.3.1 = [(-73_i8),(-73_i8),87_i8];
Goto(bb4)
}
bb4 = {
_15.0.2.fld3.0 = _14.0;
_7 = &_15.0.3.0;
_15.0.3.0 = (-20790_i16) - 12292_i16;
_7 = &_15.0.3.0;
_14.0 = _15.0.2.fld3.1 as u32;
_15.0.3.2 = [53482799262049560229961982379123052033_i128,(-91117752920693128577715100539379157148_i128),151343367672214249550495613385380777377_i128,6936613824588584716261954118255574425_i128,26069510814437139494163985409713003370_i128,85406838125414050138744728291168195459_i128,(-7449869402625908373988748364978367366_i128),(-22231654510157125709652033614392685873_i128)];
_15.0.0 = core::ptr::addr_of!(_15.0.3.0);
_15.0.0 = core::ptr::addr_of!((*_7));
match _15.0.3.3 {
9752264915554848955 => bb6,
_ => bb5
}
}
bb5 = {
_14.0 = !_9.0;
_9.1 = 12270319844568215511_u64 as f32;
_3 = (-86_i8) as isize;
_15.0.3.3 = 9752264915554848955_usize;
_15.0.2.fld3 = (_14.0, _9.1);
_10 = [50_u8,89_u8,154_u8,158_u8];
_15.0.2.fld0 = _4;
_15.0.3.1 = [(-73_i8),(-73_i8),87_i8];
Goto(bb4)
}
bb6 = {
_11 = [true,false,false];
_15.0.2 = Adt21 { fld0: _4,fld1: _1,fld2: 35482_u16,fld3: _9,fld4: 174_u8 };
_15.0.2.fld2 = _15.0.2.fld0 as u16;
_19.0.0.fld2 = !_3;
_14 = (_9.0, _15.0.2.fld3.1);
_4 = _15.0.2.fld0;
_13 = _5;
_16 = !1_i8;
_18 = core::ptr::addr_of!((*_7));
_14.1 = _15.0.3.3 as f32;
_15.0.3.1 = [_16,_16,_16];
_3 = false as isize;
_19.0.0.fld1 = _8;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_15.0.2.fld4 = 188_u8;
_19.0.0.fld0 = Move(_15.0.0);
_19.0.0.fld2 = _3;
_19.0.0.fld2 = -_3;
_22 = [_15.0.2.fld3.0,_9.0,_9.0,_9.0,_9.0];
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
match _15.0.3.3 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb7,
9752264915554848955 => bb9,
_ => bb8
}
}
bb7 = {
_14.0 = !_9.0;
_9.1 = 12270319844568215511_u64 as f32;
_3 = (-86_i8) as isize;
_15.0.3.3 = 9752264915554848955_usize;
_15.0.2.fld3 = (_14.0, _9.1);
_10 = [50_u8,89_u8,154_u8,158_u8];
_15.0.2.fld0 = _4;
_15.0.3.1 = [(-73_i8),(-73_i8),87_i8];
Goto(bb4)
}
bb8 = {
_6 = (-279112872_i32);
_1 = _8;
_9.0 = !2629851735_u32;
_9.1 = _6 as f32;
_5 = (-1542610934896775531_i64) & 8692965924995758571_i64;
_3 = (-91_isize) - 84_isize;
_8 = _1;
_6 = 682648717_i32 + 534234308_i32;
_8 = _1;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = [151_u8,66_u8,56_u8,164_u8];
_1 = _8;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_11 = [false,true,false];
_1 = _8;
_8 = _1;
Goto(bb3)
}
bb9 = {
_17 = _15.0.3.3;
_7 = &(*_7);
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_19.0.0 = Adt38 { fld0: Move(_18),fld1: _8,fld2: _3,fld3: _6 };
_15.0.0 = core::ptr::addr_of!((*_7));
_22 = [_14.0,_9.0,_15.0.2.fld3.0,_14.0,_14.0];
_19.1 = (_9.0, _9.1);
_14.0 = 17227035318683978979_u64 as u32;
_10 = [_15.0.2.fld4,_15.0.2.fld4,_15.0.2.fld4,_15.0.2.fld4];
_19.1 = (_14.0, _9.1);
_15.0.2.fld4 = _9.0 as u8;
_14.0 = _15.0.2.fld3.0 >> _3;
_27.2 = [_15.0.2.fld4,_15.0.2.fld4,_15.0.2.fld4,_15.0.2.fld4];
_5 = _19.0.0.fld1 as i64;
_15.0.3.1 = [_16,_16,_16];
match _15.0.3.3 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb12,
9752264915554848955 => bb14,
_ => bb13
}
}
bb10 = {
_6 = (-279112872_i32);
_1 = _8;
_9.0 = !2629851735_u32;
_9.1 = _6 as f32;
_5 = (-1542610934896775531_i64) & 8692965924995758571_i64;
_3 = (-91_isize) - 84_isize;
_8 = _1;
_6 = 682648717_i32 + 534234308_i32;
_8 = _1;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = [151_u8,66_u8,56_u8,164_u8];
_1 = _8;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_11 = [false,true,false];
_1 = _8;
_8 = _1;
Goto(bb3)
}
bb11 = {
_15.0.2.fld3.0 = _14.0;
_7 = &_15.0.3.0;
_15.0.3.0 = (-20790_i16) - 12292_i16;
_7 = &_15.0.3.0;
_14.0 = _15.0.2.fld3.1 as u32;
_15.0.3.2 = [53482799262049560229961982379123052033_i128,(-91117752920693128577715100539379157148_i128),151343367672214249550495613385380777377_i128,6936613824588584716261954118255574425_i128,26069510814437139494163985409713003370_i128,85406838125414050138744728291168195459_i128,(-7449869402625908373988748364978367366_i128),(-22231654510157125709652033614392685873_i128)];
_15.0.0 = core::ptr::addr_of!(_15.0.3.0);
_15.0.0 = core::ptr::addr_of!((*_7));
match _15.0.3.3 {
9752264915554848955 => bb6,
_ => bb5
}
}
bb12 = {
_11 = [true,false,false];
_15.0.2 = Adt21 { fld0: _4,fld1: _1,fld2: 35482_u16,fld3: _9,fld4: 174_u8 };
_15.0.2.fld2 = _15.0.2.fld0 as u16;
_19.0.0.fld2 = !_3;
_14 = (_9.0, _15.0.2.fld3.1);
_4 = _15.0.2.fld0;
_13 = _5;
_16 = !1_i8;
_18 = core::ptr::addr_of!((*_7));
_14.1 = _15.0.3.3 as f32;
_15.0.3.1 = [_16,_16,_16];
_3 = false as isize;
_19.0.0.fld1 = _8;
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
_15.0.2.fld4 = 188_u8;
_19.0.0.fld0 = Move(_15.0.0);
_19.0.0.fld2 = _3;
_19.0.0.fld2 = -_3;
_22 = [_15.0.2.fld3.0,_9.0,_9.0,_9.0,_9.0];
RET = [_6,_6,_6,_6,_6,_6,_6,_6];
match _15.0.3.3 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb7,
9752264915554848955 => bb9,
_ => bb8
}
}
bb13 = {
_14.0 = !_9.0;
_9.1 = 12270319844568215511_u64 as f32;
_3 = (-86_i8) as isize;
_15.0.3.3 = 9752264915554848955_usize;
_15.0.2.fld3 = (_14.0, _9.1);
_10 = [50_u8,89_u8,154_u8,158_u8];
_15.0.2.fld0 = _4;
_15.0.3.1 = [(-73_i8),(-73_i8),87_i8];
Goto(bb4)
}
bb14 = {
_19.0.0.fld1 = _15.0.2.fld1;
_6 = _19.0.0.fld3;
_26 = _14.1;
_24 = [22724148883266839853860195991335406151_i128];
_15.0.3.2 = [143197800929583519033640757105063304903_i128,(-105819298739391680741799766111098662615_i128),(-7463805247642887171764979289422975295_i128),(-141771249996245425627511991232583145061_i128),(-143845847816536584481321493425397846316_i128),(-120140118879001614074300394588128321810_i128),159621570790851891517620306477522069930_i128,(-82648158397386926333227370165556359104_i128)];
_14.0 = _19.1.0 >> _15.0.2.fld3.0;
RET = [_6,_19.0.0.fld3,_6,_19.0.0.fld3,_6,_19.0.0.fld3,_19.0.0.fld3,_19.0.0.fld3];
_19.0.0.fld1 = _15.0.2.fld1;
_15.0.3.0 = false as i16;
_17 = _15.0.2.fld4 as usize;
_14 = _9;
_15.0.2.fld3 = (_19.1.0, _19.1.1);
_22 = [_14.0,_9.0,_15.0.2.fld3.0,_9.0,_14.0];
_19.0.0.fld1 = _1;
_15.0.1 = &_7;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 11_usize, Move(_11), 17_usize, Move(_17), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 16_usize, Move(_16), 5_usize, Move(_5), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8],mut _5: [i32; 8],mut _6: [i32; 8],mut _7: char) -> f64 {
mir! {
type RET = f64;
let _8: &'static &'static i16;
let _9: &'static [u16; 5];
let _10: f32;
let _11: &'static Adt41;
let _12: f64;
let _13: u32;
let _14: [bool; 3];
let _15: u64;
let _16: &'static Adt41;
let _17: [i32; 4];
let _18: [i32; 8];
let _19: *const f32;
let _20: *const *const i16;
let _21: i32;
let _22: u128;
let _23: usize;
let _24: i16;
let _25: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _26: *const [u16; 2];
let _27: i64;
let _28: (u16,);
let _29: i32;
let _30: u64;
let _31: &'static &'static i16;
let _32: i8;
let _33: u64;
let _34: i32;
let _35: (&'static Adt27,);
let _36: u8;
let _37: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _38: char;
let _39: usize;
let _40: isize;
let _41: Adt58;
let _42: *mut Adt21;
let _43: (&'static Adt27,);
let _44: ();
let _45: ();
{
RET = (-136211512359491026682856416901750380773_i128) as f64;
_2 = _3;
_1 = !9223372036854775807_isize;
_7 = '\u{9f7ad}';
RET = 4110510090_u32 as f64;
RET = (-1265061829957939732_i64) as f64;
_5 = [(-1401702646_i32),453494443_i32,(-1738134790_i32),1762451493_i32,(-1670463712_i32),1631822524_i32,331772407_i32,(-370742861_i32)];
_2 = [(-1417492988_i32),1094137794_i32,(-65490857_i32),(-1976152829_i32),(-1250399381_i32),(-1144765163_i32),1417593233_i32,1684405251_i32];
RET = 3306098196847760907_u64 as f64;
_4 = _3;
_7 = '\u{3be95}';
RET = _1 as f64;
RET = 258368655910322455299737668595120063457_u128 as f64;
_3 = [(-332808665_i32),363290460_i32,(-2133064570_i32),159896926_i32,(-1287530265_i32),1922498741_i32,(-303825381_i32),(-2081797944_i32)];
_3 = [(-1901405154_i32),867495892_i32,1273676089_i32,783950081_i32,648855829_i32,(-1895881779_i32),208233697_i32,1021658261_i32];
_7 = '\u{f9faf}';
_5 = [1834785452_i32,389000253_i32,2081417211_i32,(-1062797014_i32),1286920943_i32,135955370_i32,1003116829_i32,(-944520334_i32)];
Goto(bb1)
}
bb1 = {
RET = 15066736156941648760_u64 as f64;
_5 = [(-245614017_i32),(-521142590_i32),152037854_i32,724603022_i32,(-1830365589_i32),(-1534029632_i32),2004927475_i32,(-181021902_i32)];
_10 = 189556524_u32 as f32;
_7 = '\u{1bf68}';
RET = 177_u8 as f64;
_5 = _4;
_1 = !9223372036854775807_isize;
_4 = [(-90182730_i32),(-1233023510_i32),1872060993_i32,(-1125774165_i32),404669540_i32,(-1739295716_i32),1263130583_i32,17395767_i32];
_7 = '\u{3a3f9}';
_2 = _6;
_4 = [1707589390_i32,(-1658708800_i32),2052710519_i32,1514378799_i32,1509433257_i32,(-138753038_i32),(-693439617_i32),(-2142110_i32)];
_10 = 3_usize as f32;
_3 = [1526013416_i32,1433621150_i32,808174039_i32,(-707191463_i32),1429216319_i32,1124713330_i32,(-1971279957_i32),1080645776_i32];
_3 = [619244159_i32,(-1002365894_i32),(-804498306_i32),924774931_i32,(-1097504527_i32),1879918882_i32,(-1683908155_i32),1017728427_i32];
Goto(bb2)
}
bb2 = {
RET = (-811807990_i32) as f64;
_4 = _6;
RET = 17417146988251405048_u64 as f64;
_14 = [true,false,true];
_15 = 14674231326446492063_u64 << _1;
_12 = RET;
_5 = [(-778973741_i32),(-1966352163_i32),1100909418_i32,(-708064660_i32),(-650615807_i32),(-1374193825_i32),(-1872718459_i32),(-1470178583_i32)];
_15 = 6925432698582522513_u64;
_15 = !5909384477959574849_u64;
_18 = _4;
_13 = 2451998932_u32;
_19 = core::ptr::addr_of!(_10);
RET = _1 as f64;
_13 = !723089879_u32;
_4 = [(-1047542825_i32),1916668148_i32,874992343_i32,(-275765685_i32),1743146237_i32,(-1926989520_i32),1639355316_i32,(-411315175_i32)];
_10 = (-2972839160437616277_i64) as f32;
_3 = _2;
_10 = RET as f32;
_3 = [(-2097430826_i32),394636986_i32,(-1543014303_i32),1903256702_i32,(-1467608337_i32),(-1815874250_i32),1396027035_i32,33694299_i32];
_3 = [1809958973_i32,1629169966_i32,(-171678659_i32),1618892490_i32,(-1467165656_i32),1790344216_i32,(-1001919569_i32),397031678_i32];
_13 = 17499212693251571719_usize as u32;
_7 = '\u{e5a34}';
Goto(bb3)
}
bb3 = {
_21 = false as i32;
_18 = _6;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_7 = '\u{993f6}';
_21 = (-1364197024_i32) * 1139198998_i32;
_4 = _18;
_22 = 281427211309854980177409207015081052928_u128 | 186091776340302375661676010959324134264_u128;
_7 = '\u{e2821}';
_13 = _21 as u32;
_23 = _1 as usize;
_10 = _22 as f32;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_15 = 1749212870220968059_u64 >> _13;
_18 = [_21,_21,_21,_21,_21,_21,_21,_21];
_14 = [true,false,true];
_22 = 196_u8 as u128;
_5 = _18;
_6 = [_21,_21,_21,_21,_21,_21,_21,_21];
_22 = 158489015688580134113587463242592571558_u128;
RET = _21 as f64;
_5 = _18;
RET = _12 + _12;
Goto(bb4)
}
bb4 = {
_15 = 4269561607303957926_u64 * 3291752487525246411_u64;
_10 = _23 as f32;
_24 = _15 as i16;
_25.2 = core::ptr::addr_of!(_17);
_23 = !6_usize;
_17 = [_21,_21,_21,_21];
_25.0.2.fld0 = _23 as f64;
_25.0.3.0 = _24;
_25.0.0 = core::ptr::addr_of!(_24);
_25.0.2.fld0 = _12;
_25.0.2.fld3.0 = !_13;
_1 = (-20_isize) | 59_isize;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25.1 = Move(_25.2);
_18 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25.0.2.fld3 = (_13, _10);
_1 = 58685_u16 as isize;
_17 = [_21,_21,_21,_21];
_25.0.2.fld2 = _23 as u16;
_25.1 = core::ptr::addr_of!(_17);
_1 = 9223372036854775807_isize;
_25.0.2.fld1 = _7;
_12 = _25.0.2.fld0 - _25.0.2.fld0;
match _1 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb5 = {
_21 = false as i32;
_18 = _6;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_7 = '\u{993f6}';
_21 = (-1364197024_i32) * 1139198998_i32;
_4 = _18;
_22 = 281427211309854980177409207015081052928_u128 | 186091776340302375661676010959324134264_u128;
_7 = '\u{e2821}';
_13 = _21 as u32;
_23 = _1 as usize;
_10 = _22 as f32;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_15 = 1749212870220968059_u64 >> _13;
_18 = [_21,_21,_21,_21,_21,_21,_21,_21];
_14 = [true,false,true];
_22 = 196_u8 as u128;
_5 = _18;
_6 = [_21,_21,_21,_21,_21,_21,_21,_21];
_22 = 158489015688580134113587463242592571558_u128;
RET = _21 as f64;
_5 = _18;
RET = _12 + _12;
Goto(bb4)
}
bb6 = {
RET = (-811807990_i32) as f64;
_4 = _6;
RET = 17417146988251405048_u64 as f64;
_14 = [true,false,true];
_15 = 14674231326446492063_u64 << _1;
_12 = RET;
_5 = [(-778973741_i32),(-1966352163_i32),1100909418_i32,(-708064660_i32),(-650615807_i32),(-1374193825_i32),(-1872718459_i32),(-1470178583_i32)];
_15 = 6925432698582522513_u64;
_15 = !5909384477959574849_u64;
_18 = _4;
_13 = 2451998932_u32;
_19 = core::ptr::addr_of!(_10);
RET = _1 as f64;
_13 = !723089879_u32;
_4 = [(-1047542825_i32),1916668148_i32,874992343_i32,(-275765685_i32),1743146237_i32,(-1926989520_i32),1639355316_i32,(-411315175_i32)];
_10 = (-2972839160437616277_i64) as f32;
_3 = _2;
_10 = RET as f32;
_3 = [(-2097430826_i32),394636986_i32,(-1543014303_i32),1903256702_i32,(-1467608337_i32),(-1815874250_i32),1396027035_i32,33694299_i32];
_3 = [1809958973_i32,1629169966_i32,(-171678659_i32),1618892490_i32,(-1467165656_i32),1790344216_i32,(-1001919569_i32),397031678_i32];
_13 = 17499212693251571719_usize as u32;
_7 = '\u{e5a34}';
Goto(bb3)
}
bb7 = {
RET = 15066736156941648760_u64 as f64;
_5 = [(-245614017_i32),(-521142590_i32),152037854_i32,724603022_i32,(-1830365589_i32),(-1534029632_i32),2004927475_i32,(-181021902_i32)];
_10 = 189556524_u32 as f32;
_7 = '\u{1bf68}';
RET = 177_u8 as f64;
_5 = _4;
_1 = !9223372036854775807_isize;
_4 = [(-90182730_i32),(-1233023510_i32),1872060993_i32,(-1125774165_i32),404669540_i32,(-1739295716_i32),1263130583_i32,17395767_i32];
_7 = '\u{3a3f9}';
_2 = _6;
_4 = [1707589390_i32,(-1658708800_i32),2052710519_i32,1514378799_i32,1509433257_i32,(-138753038_i32),(-693439617_i32),(-2142110_i32)];
_10 = 3_usize as f32;
_3 = [1526013416_i32,1433621150_i32,808174039_i32,(-707191463_i32),1429216319_i32,1124713330_i32,(-1971279957_i32),1080645776_i32];
_3 = [619244159_i32,(-1002365894_i32),(-804498306_i32),924774931_i32,(-1097504527_i32),1879918882_i32,(-1683908155_i32),1017728427_i32];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
(*_19) = _15 as f32;
_25.0.2.fld3 = (_13, _10);
_2 = [_21,_21,_21,_21,_21,_21,_21,_21];
_13 = _23 as u32;
_25.0.3.0 = _24 >> _25.0.2.fld2;
_25.2 = Move(_25.1);
_25.0.3.2 = [112284996479035459987314887757954108269_i128,144407855177691927081633054137013177261_i128,(-150269185063974344129990035621376930432_i128),(-47633225523880179919144485793379049430_i128),(-109795733900443463151415373593042003089_i128),(-25830126318166534290438195636941000855_i128),44421759572028806851499468814429307924_i128,116755886059912436992751512056459455892_i128];
_15 = 15869695697560866490_u64 + 5225751445575891342_u64;
_15 = 15762885754282293996_u64 - 16287935085395137489_u64;
_25.0.3.2 = [(-162834315786568465595912775294175464724_i128),120898532712951801580997519710730457327_i128,(-106023015597800625346090623501855722445_i128),(-152890489523204057281470845501824755230_i128),(-128955176798058301488043331346346050972_i128),152593806225632520415754933669859939351_i128,(-109481362151425714762342370126749042019_i128),(-63913814953673518315001461169581279100_i128)];
_25.0.3.2 = [(-21689295399565030991182044377067383960_i128),(-74072911814494147111225040974996633163_i128),(-18782157971369325128470912692070565536_i128),(-161315429129734934700956340305137774501_i128),16296613123447381876764288899787587237_i128,147231521976486318989542485750506561400_i128,(-37092240550275050422909854926426652571_i128),140752068490083885125261900486514147674_i128];
_23 = _25.0.3.0 as usize;
_25.0.2.fld4 = !213_u8;
_10 = _25.0.2.fld3.1 + _25.0.2.fld3.1;
_25.0.2.fld3.0 = _13;
_25.0.3.3 = _23 & _23;
_25.0.3.2 = [83366650308165307712330599485032654729_i128,150982500553132919933159836878634844324_i128,127740566412942760748900704740095777563_i128,140870621782373243483536415655175722700_i128,70541023711461719949564443143330654507_i128,124475130594519940988305012066698355621_i128,84721742722190013680148136463663844348_i128,27357348080579516456591081313347596624_i128];
_25.0.3.2 = [(-49137132914491804829935537124587738476_i128),65045677038173276439914630987459165324_i128,50460246482672236089505874644453933930_i128,(-68596618286412121027807566080748456140_i128),70060271936328974468425755852404341966_i128,(-61458866153587702297221949284337826919_i128),(-93125627524995753358985672311256527428_i128),(-7904736972447949335906174106508667692_i128)];
Call(_25.2 = fn3(_4, _25.0.3.2, Move(_19), _25.0.2.fld2, Move(_25.0.0), _25.0.3.0, _25.0.2.fld3.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_12 = -RET;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_17 = [_21,_21,_21,_21];
_25.1 = core::ptr::addr_of!(_17);
RET = _12 + _25.0.2.fld0;
_1 = (-9223372036854775808_isize);
_15 = 4104250832175562961_u64 << _25.0.3.3;
_25.0.3.3 = !_23;
_25.0.2.fld0 = RET + RET;
_25.0.2.fld3 = (_13, _10);
_28 = (_25.0.2.fld2,);
_19 = core::ptr::addr_of!(_25.0.2.fld3.1);
(*_19) = -_10;
_25.0.2.fld3.0 = 72751809381516775469958331224715570093_i128 as u32;
_25.0.2.fld2 = !_28.0;
_21 = false as i32;
_19 = core::ptr::addr_of!((*_19));
_25.0.3.2 = [(-118682767162893653347013788810011360298_i128),(-121615370321030863883038048057318820221_i128),(-122973303411878453274840364151155543931_i128),151245245854860998243999811409671500226_i128,116731427907932427009727808848790280239_i128,154008950637471217347136626700951167911_i128,(-39293354604290003874252347657516826068_i128),(-147815248330911510691015698875538269248_i128)];
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28.0 = _25.0.2.fld2;
_25.0.2.fld2 = !_28.0;
_7 = _25.0.2.fld1;
_7 = _25.0.2.fld1;
_25.0.2.fld3.1 = _13 as f32;
_27 = (-5232500093923094548_i64) ^ (-1312821754163127977_i64);
Goto(bb11)
}
bb11 = {
_12 = -_25.0.2.fld0;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_23 = _25.0.3.3 | _25.0.3.3;
RET = _25.0.2.fld0 - _12;
RET = -_12;
_20 = core::ptr::addr_of!(_25.0.0);
_1 = (-14_isize) | 9223372036854775807_isize;
_14 = [true,true,true];
_22 = _21 as u128;
_32 = -(-124_i8);
_30 = _15 & _15;
_1 = 68_isize;
(*_20) = core::ptr::addr_of!(_24);
_25.0.2.fld4 = !237_u8;
_25.0.3.3 = !_23;
_13 = (*_19) as u32;
_25.0.0 = core::ptr::addr_of!(_24);
_22 = 233895319854606309688622186707498854950_u128;
_25.0.2.fld3.0 = RET as u32;
_18 = _4;
_25.0.3.1 = [_32,_32,_32];
_25.0.3.2 = [63229761219671123384839769575602250716_i128,137394377255048516559484440181651569133_i128,123593204606916125651700639403723504532_i128,(-85787048910163198090501291293745485823_i128),134056636130628777342110747148592779468_i128,125851973262402847741890850836437486059_i128,(-47332827251141004214484787130114681540_i128),58711746846195482861950358043429352015_i128];
_37.0.0 = Move((*_20));
_20 = core::ptr::addr_of!(_25.0.0);
match _22 {
0 => bb8,
1 => bb12,
2 => bb13,
233895319854606309688622186707498854950 => bb15,
_ => bb14
}
}
bb12 = {
_15 = 4269561607303957926_u64 * 3291752487525246411_u64;
_10 = _23 as f32;
_24 = _15 as i16;
_25.2 = core::ptr::addr_of!(_17);
_23 = !6_usize;
_17 = [_21,_21,_21,_21];
_25.0.2.fld0 = _23 as f64;
_25.0.3.0 = _24;
_25.0.0 = core::ptr::addr_of!(_24);
_25.0.2.fld0 = _12;
_25.0.2.fld3.0 = !_13;
_1 = (-20_isize) | 59_isize;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25.1 = Move(_25.2);
_18 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25.0.2.fld3 = (_13, _10);
_1 = 58685_u16 as isize;
_17 = [_21,_21,_21,_21];
_25.0.2.fld2 = _23 as u16;
_25.1 = core::ptr::addr_of!(_17);
_1 = 9223372036854775807_isize;
_25.0.2.fld1 = _7;
_12 = _25.0.2.fld0 - _25.0.2.fld0;
match _1 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb13 = {
_21 = false as i32;
_18 = _6;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_7 = '\u{993f6}';
_21 = (-1364197024_i32) * 1139198998_i32;
_4 = _18;
_22 = 281427211309854980177409207015081052928_u128 | 186091776340302375661676010959324134264_u128;
_7 = '\u{e2821}';
_13 = _21 as u32;
_23 = _1 as usize;
_10 = _22 as f32;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_15 = 1749212870220968059_u64 >> _13;
_18 = [_21,_21,_21,_21,_21,_21,_21,_21];
_14 = [true,false,true];
_22 = 196_u8 as u128;
_5 = _18;
_6 = [_21,_21,_21,_21,_21,_21,_21,_21];
_22 = 158489015688580134113587463242592571558_u128;
RET = _21 as f64;
_5 = _18;
RET = _12 + _12;
Goto(bb4)
}
bb14 = {
RET = 15066736156941648760_u64 as f64;
_5 = [(-245614017_i32),(-521142590_i32),152037854_i32,724603022_i32,(-1830365589_i32),(-1534029632_i32),2004927475_i32,(-181021902_i32)];
_10 = 189556524_u32 as f32;
_7 = '\u{1bf68}';
RET = 177_u8 as f64;
_5 = _4;
_1 = !9223372036854775807_isize;
_4 = [(-90182730_i32),(-1233023510_i32),1872060993_i32,(-1125774165_i32),404669540_i32,(-1739295716_i32),1263130583_i32,17395767_i32];
_7 = '\u{3a3f9}';
_2 = _6;
_4 = [1707589390_i32,(-1658708800_i32),2052710519_i32,1514378799_i32,1509433257_i32,(-138753038_i32),(-693439617_i32),(-2142110_i32)];
_10 = 3_usize as f32;
_3 = [1526013416_i32,1433621150_i32,808174039_i32,(-707191463_i32),1429216319_i32,1124713330_i32,(-1971279957_i32),1080645776_i32];
_3 = [619244159_i32,(-1002365894_i32),(-804498306_i32),924774931_i32,(-1097504527_i32),1879918882_i32,(-1683908155_i32),1017728427_i32];
Goto(bb2)
}
bb15 = {
_37.0.2.fld0 = _30 as f64;
_25.1 = core::ptr::addr_of!(_17);
_25.0.2.fld2 = _28.0;
_38 = _25.0.2.fld1;
_28 = (_25.0.2.fld2,);
_25.0.3.0 = _24;
_25.0.2.fld0 = _12;
_25.0.2.fld3 = (_13, _10);
_37.0.2.fld3.0 = _25.0.2.fld3.0 >> _23;
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(2_usize, 30_usize, Move(_30), 28_usize, Move(_28), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(2_usize, 27_usize, Move(_27), 4_usize, Move(_4), 24_usize, Move(_24), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(2_usize, 18_usize, Move(_18), 38_usize, Move(_38), 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [i32; 8],mut _2: [i128; 8],mut _3: *const f32,mut _4: u16,mut _5: *const i16,mut _6: i16,mut _7: f32) -> *const [i32; 4] {
mir! {
type RET = *const [i32; 4];
let _8: [u16; 5];
let _9: ((Adt38,), (u32, f32), u128);
let _10: isize;
let _11: bool;
let _12: *mut Adt21;
let _13: ();
let _14: ();
{
_1 = [(-1373524384_i32),(-2089529565_i32),449271230_i32,(-300885036_i32),(-276406826_i32),(-545809605_i32),(-1906216718_i32),2146685854_i32];
_3 = core::ptr::addr_of!(_7);
_8 = [_4,_4,_4,_4,_4];
(*_3) = 99_u8 as f32;
_7 = _6 as f32;
_9.2 = 100570872626120096286918638319911452402_u128;
_10 = 9223372036854775807_isize;
_10 = (-9223372036854775808_isize);
_2 = [130787024387149787104124237696716190278_i128,119089101458904756632850905290660859331_i128,(-98329579293726118844130334211390369665_i128),27803231398551182416094471020400780677_i128,104344911849685489845588702754884404527_i128,(-38687246108584048455788573352684295114_i128),(-120281945080027080056708757864825327515_i128),(-25207881525138915156085522381691334718_i128)];
Call(RET = fn4(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.1.0 = 2751772728_u32 | 2071314369_u32;
_3 = core::ptr::addr_of!((*_3));
_11 = false;
_9.1.0 = 3709359150_u32 | 3193588725_u32;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(3_usize, 11_usize, Move(_11), 6_usize, Move(_6), 2_usize, Move(_2), 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4() -> *const [i32; 4] {
mir! {
type RET = *const [i32; 4];
let _1: u32;
let _2: &'static (u32, f32);
let _3: &'static &'static [u16; 5];
let _4: &'static Adt48;
let _5: ();
let _6: ();
{
_1 = !1113469177_u32;
_1 = !1139999493_u32;
_1 = 673208172_u32;
_1 = !2346541418_u32;
_1 = 6019566312820125455_i64 as u32;
_1 = 2720058896_u32 & 689258361_u32;
_1 = 2460770760_u32 >> (-80260861161657655482048328283403735054_i128);
_1 = !3422601727_u32;
_1 = !3779689619_u32;
_1 = !484525353_u32;
_1 = 38655_u16 as u32;
_1 = 2616939199_u32 >> 3703928804916591802_i64;
_1 = !236019726_u32;
_1 = (-1722255165046077566_i64) as u32;
_1 = 1578089009_i32 as u32;
_1 = 625046412719418358533065791122536246_i128 as u32;
_1 = !649166865_u32;
_1 = 229_u8 as u32;
_1 = '\u{30c4a}' as u32;
_1 = 1071734742_u32 ^ 416083345_u32;
_1 = 2988735016_u32 << 123_u8;
_1 = 1811949080_u32 ^ 1856304860_u32;
Goto(bb1)
}
bb1 = {
_1 = 1491543872_u32 - 4203803067_u32;
_1 = 19345_i16 as u32;
_1 = 2644340697_u32;
_1 = !1387759020_u32;
_1 = true as u32;
_1 = 1051152000_u32;
_1 = !1432876388_u32;
_1 = 10421908181638734340_u64 as u32;
_1 = 30527_u16 as u32;
_1 = 2428768896_u32 ^ 1442531768_u32;
_1 = !3672098519_u32;
Call(RET = fn5(_1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = true as u32;
_1 = !2333643137_u32;
_1 = 847365583_u32;
_1 = 1404830180_u32;
_1 = 693848193_u32;
_1 = 15199173423438511762_u64 as u32;
_1 = 3724424234_u32;
_1 = !2111424753_u32;
_1 = !404517995_u32;
Goto(bb3)
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u32,mut _2: u32,mut _3: u32,mut _4: u32) -> *const [i32; 4] {
mir! {
type RET = *const [i32; 4];
let _5: char;
let _6: *mut &'static Adt41;
let _7: bool;
let _8: [u8; 8];
let _9: i8;
let _10: char;
let _11: isize;
let _12: Adt65;
let _13: f64;
let _14: (u32, i8);
let _15: [u32; 7];
let _16: &'static (u32, f32);
let _17: isize;
let _18: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _19: i8;
let _20: [u8; 8];
let _21: Adt58;
let _22: [u32; 5];
let _23: u8;
let _24: ((Adt38,), (u32, f32), u128);
let _25: &'static i16;
let _26: Adt21;
let _27: usize;
let _28: &'static &'static [u16; 5];
let _29: Adt27;
let _30: isize;
let _31: &'static Adt41;
let _32: [isize; 5];
let _33: f64;
let _34: (u32, i8);
let _35: &'static Adt41;
let _36: [u32; 5];
let _37: *mut u128;
let _38: u16;
let _39: (f64,);
let _40: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _41: u32;
let _42: isize;
let _43: (u32, f32);
let _44: *mut u8;
let _45: (Adt38,);
let _46: [i128; 8];
let _47: *mut *mut Adt21;
let _48: isize;
let _49: bool;
let _50: i16;
let _51: (bool, isize, Adt47);
let _52: *mut Adt21;
let _53: bool;
let _54: f32;
let _55: u128;
let _56: (i16, [i8; 3], [i128; 8], usize);
let _57: usize;
let _58: [i64; 3];
let _59: [isize; 5];
let _60: *const [u16; 2];
let _61: Adt27;
let _62: [i32; 4];
let _63: Adt41;
let _64: f32;
let _65: u16;
let _66: i16;
let _67: [u8; 4];
let _68: ();
let _69: ();
{
_3 = 8557711641981329072_i64 as u32;
_4 = _2 | _1;
_3 = true as u32;
_3 = !_1;
_2 = _3 & _4;
_2 = _3 ^ _4;
_3 = 109_i8 as u32;
_3 = _4 >> _2;
_5 = '\u{aa1f0}';
_1 = _3;
_2 = _3;
_4 = _3 & _3;
_4 = (-106_i8) as u32;
_1 = 37701_u16 as u32;
_1 = !_2;
_2 = !_1;
_4 = !_3;
_4 = _3 ^ _1;
Goto(bb1)
}
bb1 = {
_1 = !_2;
_3 = _1;
_3 = _2;
_5 = '\u{4ecb2}';
_1 = _4;
_2 = 165331070729906574428659221820172976239_i128 as u32;
_8 = [205_u8,219_u8,194_u8,219_u8,213_u8,57_u8,123_u8,202_u8];
_5 = '\u{62962}';
_8 = [64_u8,242_u8,65_u8,238_u8,19_u8,189_u8,130_u8,220_u8];
_4 = _1 >> _3;
_7 = true;
_2 = _1;
_3 = 936_u16 as u32;
_5 = '\u{9f913}';
_8 = [68_u8,220_u8,201_u8,227_u8,185_u8,119_u8,80_u8,216_u8];
_5 = '\u{2f923}';
_1 = !_4;
_3 = 23317_u16 as u32;
_8 = [3_u8,91_u8,231_u8,165_u8,167_u8,17_u8,241_u8,223_u8];
_9 = (-122_i8) << _4;
_8 = [119_u8,139_u8,45_u8,57_u8,114_u8,207_u8,100_u8,72_u8];
_5 = '\u{6e979}';
_1 = _4;
_3 = !_2;
_2 = !_1;
Goto(bb2)
}
bb2 = {
_1 = !_3;
_3 = _1 ^ _4;
_9 = 96_i8;
_4 = _3 >> _3;
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
96 => bb9,
_ => bb8
}
}
bb3 = {
_1 = !_2;
_3 = _1;
_3 = _2;
_5 = '\u{4ecb2}';
_1 = _4;
_2 = 165331070729906574428659221820172976239_i128 as u32;
_8 = [205_u8,219_u8,194_u8,219_u8,213_u8,57_u8,123_u8,202_u8];
_5 = '\u{62962}';
_8 = [64_u8,242_u8,65_u8,238_u8,19_u8,189_u8,130_u8,220_u8];
_4 = _1 >> _3;
_7 = true;
_2 = _1;
_3 = 936_u16 as u32;
_5 = '\u{9f913}';
_8 = [68_u8,220_u8,201_u8,227_u8,185_u8,119_u8,80_u8,216_u8];
_5 = '\u{2f923}';
_1 = !_4;
_3 = 23317_u16 as u32;
_8 = [3_u8,91_u8,231_u8,165_u8,167_u8,17_u8,241_u8,223_u8];
_9 = (-122_i8) << _4;
_8 = [119_u8,139_u8,45_u8,57_u8,114_u8,207_u8,100_u8,72_u8];
_5 = '\u{6e979}';
_1 = _4;
_3 = !_2;
_2 = !_1;
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
_9 = -(-128_i8);
_2 = _4 + _3;
_2 = _3;
_4 = _2 * _2;
_4 = 124915821049449873732849668628792588651_u128 as u32;
_7 = true | true;
_4 = _3;
_10 = _5;
_10 = _5;
_10 = _5;
_5 = _10;
_12.fld1.fld0 = 17818480846805045943_usize as f64;
Goto(bb10)
}
bb10 = {
_11 = -(-9223372036854775808_isize);
_12.fld1.fld0 = 5479933742155610390_u64 as f64;
Call(_5 = fn6(_2, _2, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.fld1.fld2 = !61331_u16;
_12.fld1.fld0 = 5_usize as f64;
_12.fld1.fld3.1 = 166686657151743331956460888337842470914_u128 as f32;
_12.fld1.fld4 = !23_u8;
_3 = _2;
_12.fld1.fld4 = 176_u8 & 85_u8;
_12.fld1.fld1 = _10;
_12.fld1.fld0 = (-4152899950606130177_i64) as f64;
_12.fld1.fld3.0 = _3;
_12.fld0 = _9;
_13 = _12.fld1.fld0 - _12.fld1.fld0;
Goto(bb12)
}
bb12 = {
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _10 };
SetDiscriminant(_12.fld2, 1);
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _5 };
_11 = _12.fld1.fld2 as isize;
_1 = _4 << _3;
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _5 };
_5 = Field::<char>(Variant(_12.fld2, 3), 1);
_3 = !_4;
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_1 = _12.fld1.fld3.1 as u32;
_18.0.0 = core::ptr::addr_of!(_18.0.3.0);
_7 = _2 < _12.fld1.fld3.0;
place!(Field::<bool>(Variant(_12.fld2, 3), 0)) = !_7;
_18.0.2.fld3.0 = !_12.fld1.fld3.0;
_14.1 = -_12.fld0;
_16 = &_18.0.2.fld3;
_20 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
SetDiscriminant(_12.fld2, 2);
_18.0.2.fld3.0 = !_4;
_12.fld1.fld3.0 = _3 ^ _4;
Goto(bb13)
}
bb13 = {
_18.0.2.fld3 = (_2, _12.fld1.fld3.1);
_12.fld1.fld3 = (_2, _18.0.2.fld3.1);
_2 = _9 as u32;
place!(Field::<f64>(Variant(_12.fld2, 2), 1)) = _13 - _13;
_18.0.3.0 = 17597881210042939702_usize as i16;
_18.0.2.fld0 = _13;
_12.fld1.fld2 = _4 as u16;
_3 = !_18.0.2.fld3.0;
Goto(bb14)
}
bb14 = {
_18.0.2.fld3.0 = _7 as u32;
_7 = _12.fld1.fld1 == _10;
_24.1 = _18.0.2.fld3;
_22 = [_12.fld1.fld3.0,_4,_4,_3,_12.fld1.fld3.0];
Goto(bb15)
}
bb15 = {
_18.0.2 = Adt21 { fld0: Field::<f64>(Variant(_12.fld2, 2), 1),fld1: _10,fld2: _12.fld1.fld2,fld3: _24.1,fld4: _12.fld1.fld4 };
place!(Field::<i32>(Variant(_12.fld2, 2), 5)) = 1212047529_i32;
_13 = Field::<f64>(Variant(_12.fld2, 2), 1) + Field::<f64>(Variant(_12.fld2, 2), 1);
_18.0.3.0 = _18.0.2.fld2 as i16;
_19 = _7 as i8;
_4 = _24.1.0;
_20 = [_18.0.2.fld4,_18.0.2.fld4,_12.fld1.fld4,_18.0.2.fld4,_12.fld1.fld4,_18.0.2.fld4,_12.fld1.fld4,_18.0.2.fld4];
_24.0.0 = Adt38 { fld0: Move(_18.0.0),fld1: _12.fld1.fld1,fld2: _11,fld3: Field::<i32>(Variant(_12.fld2, 2), 5) };
_17 = _24.0.0.fld2;
_18.0.0 = core::ptr::addr_of!(_18.0.3.0);
_24.0.0.fld0 = core::ptr::addr_of!(_18.0.3.0);
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _24.0.0.fld1 };
_12.fld0 = _18.0.2.fld0 as i8;
_12.fld1.fld3 = _18.0.2.fld3;
_18.0.3.2 = [49984638557517869114911216532350441417_i128,(-135539442754404681916081571412478126675_i128),(-101864131222268352130494591326716397923_i128),(-2056307304492279440098956947019308689_i128),103608291071611712764733640550108695595_i128,(-32399881143752766647592038701498149282_i128),(-30619456860304764228267614958895725411_i128),(-16692179666923126762941992225688380502_i128)];
_12.fld1 = Move(_18.0.2);
_18.0.2 = Adt21 { fld0: _12.fld1.fld0,fld1: Field::<char>(Variant(_12.fld2, 3), 1),fld2: _12.fld1.fld2,fld3: _12.fld1.fld3,fld4: _12.fld1.fld4 };
_17 = -_11;
_18.0.2.fld2 = _12.fld1.fld2 * _12.fld1.fld2;
_12.fld1.fld0 = -_13;
_18.0.3.1 = [_19,_19,_12.fld0];
_12.fld1 = Move(_18.0.2);
_18.0.2.fld4 = _12.fld1.fld4 >> _4;
match _24.0.0.fld3 {
0 => bb1,
1 => bb13,
2 => bb16,
3 => bb17,
4 => bb18,
1212047529 => bb20,
_ => bb19
}
}
bb16 = {
_1 = !_3;
_3 = _1 ^ _4;
_9 = 96_i8;
_4 = _3 >> _3;
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
96 => bb9,
_ => bb8
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_9 = -(-128_i8);
_2 = _4 + _3;
_2 = _3;
_4 = _2 * _2;
_4 = 124915821049449873732849668628792588651_u128 as u32;
_7 = true | true;
_4 = _3;
_10 = _5;
_10 = _5;
_10 = _5;
_5 = _10;
_12.fld1.fld0 = 17818480846805045943_usize as f64;
Goto(bb10)
}
bb20 = {
_14.1 = _19;
_20 = [_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4,_18.0.2.fld4];
_4 = !_12.fld1.fld3.0;
match _24.0.0.fld3 {
0 => bb7,
1 => bb2,
2 => bb18,
3 => bb10,
4 => bb12,
5 => bb21,
6 => bb22,
1212047529 => bb24,
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
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _10 };
SetDiscriminant(_12.fld2, 1);
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _5 };
_11 = _12.fld1.fld2 as isize;
_1 = _4 << _3;
_12.fld2 = Adt48::Variant3 { fld0: _7,fld1: _5 };
_5 = Field::<char>(Variant(_12.fld2, 3), 1);
_3 = !_4;
_8 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
_1 = _12.fld1.fld3.1 as u32;
_18.0.0 = core::ptr::addr_of!(_18.0.3.0);
_7 = _2 < _12.fld1.fld3.0;
place!(Field::<bool>(Variant(_12.fld2, 3), 0)) = !_7;
_18.0.2.fld3.0 = !_12.fld1.fld3.0;
_14.1 = -_12.fld0;
_16 = &_18.0.2.fld3;
_20 = [_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4,_12.fld1.fld4];
SetDiscriminant(_12.fld2, 2);
_18.0.2.fld3.0 = !_4;
_12.fld1.fld3.0 = _3 ^ _4;
Goto(bb13)
}
bb24 = {
_24.0.0.fld3 = _12.fld1.fld1 as i32;
_18.0.3.3 = !8519869153990011090_usize;
_12.fld1.fld3.0 = !_4;
_26 = Adt21 { fld0: _13,fld1: _10,fld2: _12.fld1.fld2,fld3: _12.fld1.fld3,fld4: _18.0.2.fld4 };
_9 = _12.fld0;
_12.fld1.fld3 = (_4, _26.fld3.1);
_18.0.1 = &_25;
_22 = [_4,_26.fld3.0,_24.1.0,_26.fld3.0,_26.fld3.0];
_24.2 = _12.fld0 as u128;
_12.fld1.fld3.1 = _26.fld0 as f32;
_13 = _12.fld1.fld0;
_26.fld3.1 = -_24.1.1;
_14.0 = _26.fld3.0 * _12.fld1.fld3.0;
_14.0 = !_4;
_27 = _18.0.3.3;
_30 = _17 & _24.0.0.fld2;
_12.fld0 = 14820892970189997998_u64 as i8;
_18.0.2 = Adt21 { fld0: _13,fld1: Field::<char>(Variant(_12.fld2, 3), 1),fld2: _26.fld2,fld3: _12.fld1.fld3,fld4: _26.fld4 };
_14.0 = _30 as u32;
_1 = !_12.fld1.fld3.0;
place!(Field::<bool>(Variant(_12.fld2, 3), 0)) = _7;
_18.0.2 = Adt21 { fld0: _12.fld1.fld0,fld1: Field::<char>(Variant(_12.fld2, 3), 1),fld2: _26.fld2,fld3: _24.1,fld4: _26.fld4 };
Goto(bb25)
}
bb25 = {
_12.fld1 = Move(_18.0.2);
_18.0.2.fld4 = 12234099907857875060_u64 as u8;
_20 = [_12.fld1.fld4,_26.fld4,_26.fld4,_12.fld1.fld4,_12.fld1.fld4,_26.fld4,_26.fld4,_12.fld1.fld4];
_24.0.0.fld1 = Field::<char>(Variant(_12.fld2, 3), 1);
_24.0.0.fld3 = !(-324347739_i32);
_22 = [_24.1.0,_1,_1,_12.fld1.fld3.0,_12.fld1.fld3.0];
_18.0.2.fld3.1 = _24.1.1;
_36 = _22;
_25 = &_18.0.3.0;
Goto(bb26)
}
bb26 = {
_12.fld1.fld3 = (_26.fld3.0, _24.1.1);
_26.fld1 = _5;
Call(_30 = core::intrinsics::transmute(_20), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_39.0 = _13;
Call(_23 = core::intrinsics::transmute(_12.fld1.fld4), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_6 = core::ptr::addr_of_mut!(_35);
_40.0.0 = core::ptr::addr_of!(_18.0.3.0);
_24.0.0.fld1 = _5;
_18.0.2.fld0 = _12.fld1.fld0 - _26.fld0;
SetDiscriminant(_12.fld2, 1);
place!(Field::<i8>(Variant(_12.fld2, 1), 3)) = _7 as i8;
_18.0.1 = &_25;
_33 = _18.0.2.fld0 * _18.0.2.fld0;
_42 = _30 ^ _30;
_26.fld3 = (_24.1.0, _24.1.1);
_1 = 2552424348270154967_u64 as u32;
_18.0.2 = Move(_12.fld1);
Goto(bb29)
}
bb29 = {
_40.0.2.fld1 = _5;
_18.0.1 = &_25;
_12.fld1.fld1 = _26.fld1;
_41 = _24.1.0 << _18.0.2.fld2;
_24.0.0.fld2 = _33 as isize;
_18.0.1 = &_25;
_40.0.2.fld3.0 = !_41;
_40.0.2.fld4 = _23 >> _18.0.2.fld4;
_40.0.2 = Move(_26);
_19 = _24.2 as i8;
_18.0.0 = core::ptr::addr_of!(_40.0.3.0);
_43 = (_4, _40.0.2.fld3.1);
_12.fld1.fld3 = (_40.0.2.fld3.0, _40.0.2.fld3.1);
_20 = _8;
_24.0.0 = Adt38 { fld0: Move(_40.0.0),fld1: _40.0.2.fld1,fld2: _42,fld3: 583601297_i32 };
_18.0.2.fld1 = _40.0.2.fld1;
_3 = (*_25) as u32;
_40.0.3 = _18.0.3;
_43 = _12.fld1.fld3;
_24.0.0.fld2 = _30;
_45.0.fld0 = core::ptr::addr_of!((*_25));
_13 = (*_25) as f64;
_10 = _40.0.2.fld1;
_37 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_12.fld2, 1), 1)));
_37 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_12.fld2, 1), 1)));
_40.0.0 = Move(_45.0.fld0);
place!(Field::<[i128; 8]>(Variant(_12.fld2, 1), 5)) = _40.0.3.2;
match _24.0.0.fld3 {
0 => bb27,
1 => bb20,
583601297 => bb30,
_ => bb5
}
}
bb30 = {
_24.2 = 187091699340254776777913612202605897455_u128;
_8 = _20;
_40.0.2.fld3.1 = _43.1 - _24.1.1;
_12.fld1 = Adt21 { fld0: _39.0,fld1: _18.0.2.fld1,fld2: _18.0.2.fld2,fld3: _18.0.2.fld3,fld4: _23 };
_5 = _40.0.2.fld1;
_27 = _18.0.3.3 * _18.0.3.3;
_12.fld0 = _9 ^ _9;
_45.0.fld1 = _18.0.2.fld1;
_12.fld1.fld3.0 = !_43.0;
place!(Field::<u128>(Variant(_12.fld2, 1), 1)) = _24.2 + _24.2;
_42 = _30;
_18.0.2.fld3 = _43;
_11 = _42;
_40.0.1 = &_25;
_14.0 = _30 as u32;
place!(Field::<Adt47>(Variant(_12.fld2, 1), 2)) = Adt47::Variant1 { fld0: _12.fld1.fld3.0 };
_45.0.fld3 = _24.0.0.fld3;
_25 = &(*_25);
_38 = _40.0.2.fld2;
_5 = _10;
_45 = (Move(_24.0.0),);
_51.2 = Move(Field::<Adt47>(Variant(_12.fld2, 1), 2));
_18.0.3.1 = _40.0.3.1;
match _45.0.fld3 {
0 => bb31,
583601297 => bb33,
_ => bb32
}
}
bb31 = {
_18.0.2.fld3.0 = _7 as u32;
_7 = _12.fld1.fld1 == _10;
_24.1 = _18.0.2.fld3;
_22 = [_12.fld1.fld3.0,_4,_4,_3,_12.fld1.fld3.0];
Goto(bb15)
}
bb32 = {
Return()
}
bb33 = {
_33 = _13;
_15 = [_41,_43.0,_14.0,_14.0,_41,_4,_43.0];
_50 = _9 as i16;
_24.1.0 = _40.0.2.fld3.0 >> _40.0.2.fld4;
_18.0.2.fld2 = _45.0.fld3 as u16;
Goto(bb34)
}
bb34 = {
_39.0 = _33 * _33;
_45.0.fld0 = Move(_40.0.0);
_18.0.3.2 = [(-8954611002405380116177159759020345595_i128),106466987407565061009731191271537414243_i128,(-158545939965864746222488715101461978380_i128),(-143089284422291633019397561316712656084_i128),(-98162435234549820079029245024079969944_i128),(-104190989543633860995720051567937539801_i128),107061408777903530657949554950923594126_i128,26753948988016836485625483848930665041_i128];
(*_37) = _24.2;
place!(Field::<u128>(Variant(_12.fld2, 1), 1)) = _40.0.3.0 as u128;
_44 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12.fld2, 1), 4)));
_19 = _12.fld0 >> _12.fld1.fld4;
_26 = Adt21 { fld0: _12.fld1.fld0,fld1: _12.fld1.fld1,fld2: _40.0.2.fld2,fld3: _12.fld1.fld3,fld4: _40.0.2.fld4 };
_47 = core::ptr::addr_of_mut!(_52);
_37 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_12.fld2, 1), 1)));
_16 = &_26.fld3;
_9 = _19;
_49 = _7;
SetDiscriminant(_51.2, 1);
_36 = [_40.0.2.fld3.0,_24.1.0,_24.1.0,_24.1.0,_41];
(*_47) = core::ptr::addr_of_mut!(_40.0.2);
_18.0.3 = (_40.0.3.0, _40.0.3.1, Field::<[i128; 8]>(Variant(_12.fld2, 1), 5), _27);
_12.fld0 = (*_37) as i8;
Call(_55 = core::intrinsics::transmute(Field::<u128>(Variant(_12.fld2, 1), 1)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
_45.0.fld3 = (-2090164353_i32) + (-1289287862_i32);
_24.1.0 = _41;
match _24.2 {
0 => bb33,
1 => bb17,
2 => bb3,
3 => bb16,
4 => bb29,
5 => bb6,
6 => bb7,
187091699340254776777913612202605897455 => bb36,
_ => bb26
}
}
bb36 = {
_24.0.0.fld2 = -_45.0.fld2;
_24 = (Move(_45), _12.fld1.fld3, (*_37));
_40.0.3.3 = _9 as usize;
_51.1 = _11;
_45.0 = Adt38 { fld0: Move(_24.0.0.fld0),fld1: _10,fld2: _42,fld3: _24.0.0.fld3 };
_18.0.1 = &_25;
_24.0.0.fld0 = Move(_45.0.fld0);
_18.0.2.fld3.0 = (*_52).fld3.0;
place!(Field::<usize>(Variant(_12.fld2, 1), 0)) = _40.0.3.3;
(*_47) = core::ptr::addr_of_mut!(_18.0.2);
_24.1 = _26.fld3;
(*_52).fld3.0 = _24.0.0.fld2 as u32;
_48 = _42 * _24.0.0.fld2;
Goto(bb37)
}
bb37 = {
place!(Field::<i8>(Variant(_12.fld2, 1), 3)) = _18.0.2.fld3.1 as i8;
_45 = Move(_24.0);
_40.0.3.3 = _27 - Field::<usize>(Variant(_12.fld2, 1), 0);
_53 = _18.0.2.fld3.0 < _26.fld3.0;
_45.0.fld3 = _18.0.2.fld2 as i32;
_34.0 = (*_52).fld3.0;
_57 = Field::<usize>(Variant(_12.fld2, 1), 0);
_46 = [(-75228285066141971854274788592176213338_i128),(-122260273722211469895480583184237602653_i128),(-77207048004252609909425669641999589988_i128),50550702250572949192648076176607403159_i128,(-89085960781634769975846549003135777553_i128),110328389755962336825015271013730592417_i128,11021653786800243769795111067857255706_i128,(-4820119215250009165590838749596269270_i128)];
Goto(bb38)
}
bb38 = {
_26.fld0 = (*_52).fld2 as f64;
_30 = _11 & _48;
(*_52).fld1 = _5;
_24.0.0.fld0 = core::ptr::addr_of!(_40.0.3.0);
_38 = !(*_52).fld2;
_32 = [_30,_30,_30,_11,_51.1];
_58 = [5958920811161276840_i64,2509886920617552185_i64,1979881464002643649_i64];
_18.1 = core::ptr::addr_of!(_62);
_12.fld1.fld3.0 = _34.0 - _3;
_40.0 = Move(_18.0);
_56.1 = _40.0.3.1;
(*_52).fld0 = _39.0;
_40.0.3.1 = [_19,_12.fld0,_9];
place!(Field::<u128>(Variant(_12.fld2, 1), 1)) = !_24.2;
_2 = (*_16).0 + _14.0;
_18.0.2.fld3 = (_40.0.2.fld3.0, _12.fld1.fld3.1);
_40.0.2.fld3.1 = _43.1 - _26.fld3.1;
place!(Field::<u8>(Variant(_12.fld2, 1), 4)) = 14297801718415714067_u64 as u8;
Goto(bb39)
}
bb39 = {
(*_52).fld3.0 = _41;
_1 = !(*_52).fld3.0;
RET = core::ptr::addr_of!(_62);
(*RET) = [_45.0.fld3,_45.0.fld3,_45.0.fld3,_45.0.fld3];
_63 = Adt41::Variant0 { fld0: Move(_26),fld1: _40.0.3,fld2: Move(_37),fld3: _40.0.3.1 };
_56.2 = [80180806138549301201120102104247514493_i128,2098591354622311995467011668885937041_i128,147070399765702199661547864520697199777_i128,(-74345391913342786398022512602060069345_i128),85039675932897041884991989663358083584_i128,61670940914071711954706640147344087339_i128,(-108482354989676200851868022744912525207_i128),144553163542625222051320697955880847604_i128];
_51.0 = !_53;
place!(Field::<*mut u128>(Variant(_63, 0), 2)) = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_12.fld2, 1), 1)));
_46 = [149074281381677333428903154991392235465_i128,50032527942329265852996658684612312125_i128,(-156482363426783428162614913473005981005_i128),(-61576442370660213404758447811107712648_i128),(-151509492243666957813511819389854318952_i128),89960519922936357789720492121317383064_i128,6727031999947432919555082342881647488_i128,(-41831225184373546561922718212896940816_i128)];
_26.fld4 = !_12.fld1.fld4;
place!(Field::<u32>(Variant(_51.2, 1), 0)) = _2 + _4;
_18.0.1 = &_25;
_45.0 = Adt38 { fld0: Move(_40.0.0),fld1: _10,fld2: _11,fld3: 160368746_i32 };
_24.1.1 = _18.0.2.fld3.1 - _40.0.2.fld3.1;
Goto(bb40)
}
bb40 = {
Call(_68 = dump_var(5_usize, 2_usize, Move(_2), 23_usize, Move(_23), 48_usize, Move(_48), 11_usize, Move(_11)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_68 = dump_var(5_usize, 27_usize, Move(_27), 41_usize, Move(_41), 30_usize, Move(_30), 50_usize, Move(_50)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_68 = dump_var(5_usize, 8_usize, Move(_8), 62_usize, Move(_62), 36_usize, Move(_36), 5_usize, Move(_5)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_68 = dump_var(5_usize, 32_usize, Move(_32), 7_usize, Move(_7), 15_usize, Move(_15), 14_usize, Move(_14)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: u32,mut _2: u32,mut _3: u32) -> char {
mir! {
type RET = char;
let _4: (f64,);
let _5: bool;
let _6: u8;
let _7: isize;
let _8: [i64; 3];
let _9: u8;
let _10: &'static &'static i16;
let _11: *const [i32; 4];
let _12: [u8; 8];
let _13: bool;
let _14: (*mut u8, [i32; 4]);
let _15: f32;
let _16: &'static Adt27;
let _17: usize;
let _18: i16;
let _19: i8;
let _20: (u32, f32);
let _21: f64;
let _22: [i128; 1];
let _23: Adt27;
let _24: (Adt38,);
let _25: &'static (u32, f32);
let _26: &'static [u8; 4];
let _27: char;
let _28: f32;
let _29: f32;
let _30: f64;
let _31: isize;
let _32: usize;
let _33: u128;
let _34: char;
let _35: *mut *mut Adt21;
let _36: &'static i16;
let _37: *mut u8;
let _38: u64;
let _39: [i8; 3];
let _40: Adt21;
let _41: &'static bool;
let _42: (bool, isize, Adt47);
let _43: ();
let _44: ();
{
RET = '\u{36929}';
_2 = _3;
_4.0 = (-14_i8) as f64;
_4.0 = 2_usize as f64;
_5 = !false;
_2 = 13551379642893065887_usize as u32;
RET = '\u{a3ca6}';
_4.0 = (-9223372036854775808_isize) as f64;
_1 = !_3;
_5 = false;
_4.0 = (-102_i8) as f64;
_3 = 10310_u16 as u32;
_3 = !_1;
_5 = !true;
_6 = 245_u8;
RET = '\u{a13cb}';
Call(_2 = fn7(_1, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{d16a}';
_6 = 9223372036854775807_isize as u8;
_1 = !_2;
_5 = false;
_3 = 170398485887769061363091557374835139928_u128 as u32;
Goto(bb2)
}
bb2 = {
_1 = !_2;
_6 = _2 as u8;
_1 = _2;
_6 = 148_u8 << _2;
_8 = [1798869533310397086_i64,(-1927067288482135909_i64),(-6736433388143786324_i64)];
_5 = !false;
_1 = _2 - _2;
_1 = (-7333025300369522052_i64) as u32;
_9 = _6 & _6;
Goto(bb3)
}
bb3 = {
_8 = [(-663136241141637163_i64),4823653675862326182_i64,(-6564462374424047815_i64)];
_3 = _2 | _2;
RET = '\u{c3240}';
_1 = !_3;
_7 = _5 as isize;
_8 = [699359294417091644_i64,(-4080739079214607688_i64),(-2523780961820623612_i64)];
RET = '\u{d399c}';
_9 = _3 as u8;
_8 = [(-3630983906371180152_i64),(-8584373662071897275_i64),(-4699176049149982108_i64)];
_5 = false ^ false;
_6 = _9;
RET = '\u{b3c82}';
_5 = _9 >= _9;
_6 = !_9;
_8 = [(-1783611297577496527_i64),(-6330632403424725928_i64),1312630716190013893_i64];
_2 = !_1;
RET = '\u{65b73}';
_6 = _9;
_6 = _9;
_5 = false;
_5 = _9 != _6;
_5 = false;
_1 = _3;
_3 = _2 + _2;
_6 = !_9;
_6 = (-16887_i16) as u8;
_7 = (-2_isize);
_6 = _9 | _9;
_7 = -(-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
_8 = [6322325731864249696_i64,1760231073623135979_i64,(-6122577468245435752_i64)];
RET = '\u{f1395}';
_2 = _7 as u32;
_6 = _9;
Call(_11 = fn8(_3, _6, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = [(-2347778470633296570_i64),5954691912681298268_i64,(-3841787995013443134_i64)];
RET = '\u{412fd}';
_3 = _5 as u32;
RET = '\u{9dd4a}';
_8 = [2671605567047076489_i64,(-8894754768024353598_i64),(-7862521209816253797_i64)];
_4.0 = 203115486587552169745242039198478224884_u128 as f64;
_6 = !_9;
_2 = 3867729122842273898800569887891179083_i128 as u32;
_6 = (-30777_i16) as u8;
_13 = _5;
_14.1 = [70418194_i32,1290585620_i32,878281084_i32,1052910189_i32];
_12 = [_9,_9,_9,_9,_9,_9,_9,_9];
RET = '\u{10edbf}';
_15 = 11817197721290925301_usize as f32;
_2 = _1 + _1;
_14.0 = core::ptr::addr_of_mut!(_9);
_4.0 = (-1874416904_i32) as f64;
_19 = -17_i8;
_2 = _1 * _1;
_14.1 = [845028779_i32,772723756_i32,(-1677071230_i32),1533635924_i32];
Goto(bb6)
}
bb6 = {
_6 = _9;
Goto(bb7)
}
bb7 = {
_20.0 = _2 ^ _1;
_22 = [15174848437361954136969929726432357806_i128];
_1 = _20.0 * _2;
_22 = [(-166238643396987111790138030497965504557_i128)];
_15 = 75886810582432996175822977652336266163_i128 as f32;
_21 = _4.0 - _4.0;
_15 = 439_i16 as f32;
_14.0 = core::ptr::addr_of_mut!(_6);
_19 = (-35_i8);
_24.0.fld1 = RET;
_25 = &_20;
_24.0.fld3 = 249256556_i32 >> _6;
_12 = [_6,_6,_9,_9,_6,_6,_6,_6];
_27 = _24.0.fld1;
_8 = [(-4483226452115647768_i64),7585339882824722904_i64,(-3919555449184554888_i64)];
_4 = (_21,);
_5 = !_13;
RET = _24.0.fld1;
_15 = 99277306161544642120602603517110825168_i128 as f32;
_2 = 34951_u16 as u32;
match _19 {
0 => bb1,
1 => bb5,
2 => bb3,
340282366920938463463374607431768211421 => bb8,
_ => bb4
}
}
bb8 = {
_20.0 = !_1;
_24.0.fld2 = _7 | _7;
_28 = _15 - _15;
_13 = _5;
_20.1 = -_28;
_16 = &_23;
_12 = [_9,_9,_6,_9,_6,_9,_9,_9];
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = !17541_i16;
_4.0 = -_21;
_4.0 = _21 - _21;
match _19 {
0 => bb4,
1 => bb8,
2 => bb10,
3 => bb11,
340282366920938463463374607431768211421 => bb13,
_ => bb12
}
}
bb10 = {
RET = '\u{d16a}';
_6 = 9223372036854775807_isize as u8;
_1 = !_2;
_5 = false;
_3 = 170398485887769061363091557374835139928_u128 as u32;
Goto(bb2)
}
bb11 = {
_20.0 = _2 ^ _1;
_22 = [15174848437361954136969929726432357806_i128];
_1 = _20.0 * _2;
_22 = [(-166238643396987111790138030497965504557_i128)];
_15 = 75886810582432996175822977652336266163_i128 as f32;
_21 = _4.0 - _4.0;
_15 = 439_i16 as f32;
_14.0 = core::ptr::addr_of_mut!(_6);
_19 = (-35_i8);
_24.0.fld1 = RET;
_25 = &_20;
_24.0.fld3 = 249256556_i32 >> _6;
_12 = [_6,_6,_9,_9,_6,_6,_6,_6];
_27 = _24.0.fld1;
_8 = [(-4483226452115647768_i64),7585339882824722904_i64,(-3919555449184554888_i64)];
_4 = (_21,);
_5 = !_13;
RET = _24.0.fld1;
_15 = 99277306161544642120602603517110825168_i128 as f32;
_2 = 34951_u16 as u32;
match _19 {
0 => bb1,
1 => bb5,
2 => bb3,
340282366920938463463374607431768211421 => bb8,
_ => bb4
}
}
bb12 = {
_8 = [(-2347778470633296570_i64),5954691912681298268_i64,(-3841787995013443134_i64)];
RET = '\u{412fd}';
_3 = _5 as u32;
RET = '\u{9dd4a}';
_8 = [2671605567047076489_i64,(-8894754768024353598_i64),(-7862521209816253797_i64)];
_4.0 = 203115486587552169745242039198478224884_u128 as f64;
_6 = !_9;
_2 = 3867729122842273898800569887891179083_i128 as u32;
_6 = (-30777_i16) as u8;
_13 = _5;
_14.1 = [70418194_i32,1290585620_i32,878281084_i32,1052910189_i32];
_12 = [_9,_9,_9,_9,_9,_9,_9,_9];
RET = '\u{10edbf}';
_15 = 11817197721290925301_usize as f32;
_2 = _1 + _1;
_14.0 = core::ptr::addr_of_mut!(_9);
_4.0 = (-1874416904_i32) as f64;
_19 = -17_i8;
_2 = _1 * _1;
_14.1 = [845028779_i32,772723756_i32,(-1677071230_i32),1533635924_i32];
Goto(bb6)
}
bb13 = {
_11 = core::ptr::addr_of!(_14.1);
(*_11) = [_24.0.fld3,_24.0.fld3,_24.0.fld3,_24.0.fld3];
_14.0 = core::ptr::addr_of_mut!(_9);
(*_11) = [_24.0.fld3,_24.0.fld3,_24.0.fld3,_24.0.fld3];
_8 = [(-4219396525877928027_i64),5978903753623755477_i64,8184056259752464403_i64];
_15 = _18 as f32;
_25 = &_20;
_5 = _13;
_29 = _19 as f32;
_25 = &_20;
_3 = _18 as u32;
_17 = 2_usize ^ 3680160207293366086_usize;
_24.0.fld2 = RET as isize;
_12 = [_9,_9,_9,_9,_9,_9,_9,_9];
_2 = (*_25).0 & (*_25).0;
_15 = -_20.1;
RET = _27;
_9 = _19 as u8;
_21 = -_4.0;
_30 = -_4.0;
RET = _24.0.fld1;
_27 = RET;
_11 = core::ptr::addr_of!((*_11));
_1 = _18 as u32;
_31 = _7;
Goto(bb14)
}
bb14 = {
_20.0 = _2 >> _2;
_29 = 13907552295588721761_u64 as f32;
_21 = _4.0;
_15 = _28;
_36 = &_18;
_4.0 = 83963980225251961778642771687500216870_i128 as f64;
_1 = _20.0 >> _6;
_42.1 = _24.0.fld2;
_42.0 = _24.0.fld3 > _24.0.fld3;
_4 = (_30,);
_24.0.fld0 = core::ptr::addr_of!(_18);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(6_usize, 18_usize, Move(_18), 5_usize, Move(_5), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(6_usize, 12_usize, Move(_12), 8_usize, Move(_8), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u32,mut _2: u32,mut _3: u32) -> u32 {
mir! {
type RET = u32;
let _4: isize;
let _5: ();
let _6: ();
{
RET = _2 >> _3;
_3 = _2 << RET;
_3 = _1 * _2;
_3 = !RET;
_3 = 2254261754907706238_u64 as u32;
_1 = !_2;
RET = _1 & _1;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(7_usize, 1_usize, Move(_1), 6_usize, _6, 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u32,mut _2: u8,mut _3: u32) -> *const [i32; 4] {
mir! {
type RET = *const [i32; 4];
let _4: char;
let _5: *const [i32; 4];
let _6: bool;
let _7: *const [i32; 4];
let _8: u8;
let _9: isize;
let _10: bool;
let _11: &'static (u32, f32);
let _12: [u16; 2];
let _13: [i8; 3];
let _14: bool;
let _15: (&'static Adt27, ([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]));
let _16: u128;
let _17: f32;
let _18: u64;
let _19: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _20: isize;
let _21: Adt38;
let _22: u64;
let _23: u8;
let _24: isize;
let _25: isize;
let _26: usize;
let _27: [u16; 6];
let _28: *const f32;
let _29: isize;
let _30: [isize; 5];
let _31: bool;
let _32: (&'static Adt27, ([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]));
let _33: ();
let _34: ();
{
_2 = '\u{d3497}' as u8;
_1 = _3 << _3;
_2 = 51_u8;
_3 = _1 * _1;
_2 = 226_u8;
_2 = 230_u8 * 132_u8;
_2 = 162_u8;
_3 = !_1;
_1 = _3 + _3;
_2 = true as u8;
_1 = 49728_u16 as u32;
_1 = !_3;
_2 = !237_u8;
_2 = !79_u8;
_3 = _1;
_4 = '\u{82d56}';
Goto(bb1)
}
bb1 = {
_2 = !195_u8;
_1 = _3 & _3;
_3 = 18233939580299918929_usize as u32;
_2 = 36_isize as u8;
_1 = (-733042946178209100_i64) as u32;
_3 = _1 + _1;
_1 = _3 << _3;
_2 = !72_u8;
_4 = '\u{7f03}';
_1 = _3 + _3;
_3 = _1;
_2 = !136_u8;
_3 = !_1;
_3 = _1;
_1 = _3 << _3;
_4 = '\u{ea695}';
_3 = _1 - _1;
_4 = '\u{17aff}';
_2 = 214_u8;
_4 = '\u{4606e}';
_3 = _1 * _1;
_2 = 178_u8 >> _3;
_2 = !220_u8;
_1 = 274827488082095389850863873432882401682_u128 as u32;
_1 = _3 | _3;
_4 = '\u{36764}';
_1 = 6710239137243162896_i64 as u32;
_1 = 46092_u16 as u32;
_2 = 190_u8;
_4 = '\u{fae66}';
_1 = _3;
Call(_4 = fn9(_1, _3, _1, _1, _1, _3, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = '\u{619c8}';
_1 = _3 + _3;
_6 = _3 > _3;
_1 = _3;
_4 = '\u{c5d1b}';
Goto(bb3)
}
bb3 = {
_4 = '\u{e0ecd}';
_3 = _1;
_8 = !_2;
_4 = '\u{2fbbf}';
_4 = '\u{d06f4}';
_1 = _3 | _3;
_8 = _2 / _2;
_8 = _2;
_9 = (-51_i8) as isize;
_3 = !_1;
_6 = true;
_2 = _8 >> _3;
_4 = '\u{223ce}';
_6 = !false;
_9 = !(-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
_4 = '\u{12afb}';
_4 = '\u{3354d}';
_9 = !(-9223372036854775808_isize);
_2 = _8 % _8;
_9 = _4 as isize;
_3 = !_1;
_6 = true;
_1 = !_3;
Goto(bb5)
}
bb5 = {
_4 = '\u{afb17}';
_8 = _2 ^ _2;
_1 = _3;
_4 = '\u{40f6e}';
_6 = true;
_9 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1 = !_3;
_2 = _8;
_6 = _3 <= _3;
_10 = _1 != _3;
_1 = 15_i8 as u32;
_3 = _1 - _1;
_4 = '\u{11a7}';
_1 = !_3;
_3 = _1 | _1;
Goto(bb6)
}
bb6 = {
_1 = _3 | _3;
_10 = !_6;
_2 = !_8;
_1 = !_3;
_12 = [27603_u16,23264_u16];
_6 = _10;
_12 = [52471_u16,25654_u16];
_4 = '\u{8d9ca}';
_10 = _6 < _6;
_1 = (-546380700425359808_i64) as u32;
_14 = _6 >= _6;
_2 = _8;
_14 = _6 < _10;
_8 = (-1323241893_i32) as u8;
_8 = !_2;
_2 = _8 | _8;
_13 = [(-120_i8),(-91_i8),(-97_i8)];
_3 = _1;
_8 = _2 ^ _2;
_6 = !_10;
Goto(bb7)
}
bb7 = {
RET = core::ptr::addr_of!(_15.1.0);
(*RET) = [1839644787_i32,(-528728998_i32),480386741_i32,(-1503913000_i32)];
_17 = (-99_i8) as f32;
_15.1.0 = [1848401429_i32,(-1706088708_i32),(-1614507379_i32),1670132551_i32];
_4 = '\u{3b2ad}';
_4 = '\u{5d9ad}';
_16 = 107240841145404703605690699581640145854_u128 & 271556005039099848641785935323168579780_u128;
_19.0.2.fld4 = 6648676278734760321_i64 as u8;
_19.0.2.fld3.1 = _9 as f32;
Goto(bb8)
}
bb8 = {
_19.2 = core::ptr::addr_of!(_15.1.0);
_15.1.3 = [21913_i16,(-1541_i16),(-3457_i16),2802_i16,6835_i16,19140_i16,(-709_i16),17429_i16];
_19.0.2.fld1 = _4;
(*RET) = [269644271_i32,692426746_i32,1130908792_i32,2030511075_i32];
_17 = -_19.0.2.fld3.1;
_17 = _19.0.2.fld3.1 * _19.0.2.fld3.1;
_1 = _3;
(*RET) = [(-474849832_i32),1878753514_i32,(-1303427_i32),1634291777_i32];
(*RET) = [939025894_i32,(-1308608432_i32),582788693_i32,1187270704_i32];
_19.0.3.1 = [49_i8,(-116_i8),32_i8];
_19.0.2.fld3.1 = _16 as f32;
_7 = Move(RET);
_19.0.3.3 = 20199_u16 as usize;
_11 = &_19.0.2.fld3;
_1 = _3 << _8;
_18 = 15027383925210534360_u64 * 18267745703776723335_u64;
_5 = core::ptr::addr_of!(_15.1.0);
_19.0.3.0 = 14_i8 as i16;
_19.0.2.fld3.0 = _1;
Goto(bb9)
}
bb9 = {
_1 = (-105_i8) as u32;
_21.fld3 = (-2006533473_i32) ^ 1171578463_i32;
Goto(bb10)
}
bb10 = {
_10 = _6;
_19.0.2.fld0 = _21.fld3 as f64;
RET = core::ptr::addr_of!((*_5));
_11 = &_19.0.2.fld3;
_20 = _9 - _9;
(*RET) = [_21.fld3,_21.fld3,_21.fld3,_21.fld3];
_21.fld1 = _4;
_15.1.2 = [_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0];
_20 = _9;
_16 = !2476057336445084567781586631949508078_u128;
_19.2 = core::ptr::addr_of!((*RET));
Call(_21.fld2 = core::intrinsics::bswap(_20), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_23 = _8 * _2;
_21.fld0 = core::ptr::addr_of!(_19.0.3.0);
_19.0.2.fld4 = 10413_u16 as u8;
(*RET) = [_21.fld3,_21.fld3,_21.fld3,_21.fld3];
_15.1.2 = [_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0];
_14 = !_10;
_25 = _9;
_5 = core::ptr::addr_of!((*RET));
_11 = &(*_11);
(*_5) = [_21.fld3,_21.fld3,_21.fld3,_21.fld3];
_20 = _16 as isize;
_5 = core::ptr::addr_of!((*_5));
_21.fld3 = (-1743836495_i32) | 704924964_i32;
_7 = core::ptr::addr_of!(_15.1.0);
_16 = !284900828434336401644159494356182219205_u128;
_22 = _19.0.2.fld3.0 as u64;
_10 = _14 > _14;
_19.0.2.fld2 = _10 as u16;
_15.1.3 = [_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0,_19.0.3.0];
_2 = !_8;
_2 = _19.0.2.fld0 as u8;
_21.fld3 = _22 as i32;
_21.fld1 = _19.0.2.fld1;
Goto(bb12)
}
bb12 = {
(*_5) = [_21.fld3,_21.fld3,_21.fld3,_21.fld3];
_19.0.3.3 = _23 as usize;
_9 = _25 + _25;
_11 = &(*_11);
_13 = [(-16_i8),22_i8,3_i8];
_23 = !_8;
_20 = _9 - _9;
_20 = _9;
_19.1 = Move(RET);
_8 = !_19.0.2.fld4;
_21.fld0 = core::ptr::addr_of!(_19.0.3.0);
_3 = (-17_i8) as u32;
_1 = _19.0.2.fld3.0 ^ (*_11).0;
_23 = _21.fld1 as u8;
_24 = -_20;
(*_5) = [_21.fld3,_21.fld3,_21.fld3,_21.fld3];
RET = core::ptr::addr_of!((*_5));
_28 = core::ptr::addr_of!((*_11).1);
_19.0.2.fld3 = (_1, _17);
_28 = core::ptr::addr_of!(_19.0.2.fld3.1);
_18 = !_22;
_31 = !_10;
_19.0.3.1 = _13;
Goto(bb13)
}
bb13 = {
Call(_33 = dump_var(8_usize, 23_usize, Move(_23), 20_usize, Move(_20), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_33 = dump_var(8_usize, 24_usize, Move(_24), 25_usize, Move(_25), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_33 = dump_var(8_usize, 22_usize, Move(_22), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u32,mut _2: u32,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: u32,mut _7: u32,mut _8: u32) -> char {
mir! {
type RET = char;
let _9: &'static i16;
let _10: *const (Adt38,);
let _11: *mut *const i16;
let _12: isize;
let _13: f32;
let _14: Adt27;
let _15: u8;
let _16: char;
let _17: isize;
let _18: [i128; 8];
let _19: u32;
let _20: u64;
let _21: Adt58;
let _22: *mut u8;
let _23: bool;
let _24: *mut *const i16;
let _25: (*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize));
let _26: Adt58;
let _27: (([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]),);
let _28: char;
let _29: Adt58;
let _30: ((Adt38,), Adt38, &'static i16, isize);
let _31: *const (Adt38,);
let _32: u32;
let _33: [u16; 2];
let _34: f32;
let _35: &'static (u32, f32);
let _36: i16;
let _37: [i32; 4];
let _38: [u8; 8];
let _39: bool;
let _40: [u32; 5];
let _41: (([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]),);
let _42: [i64; 3];
let _43: [i128; 8];
let _44: *const f32;
let _45: (u16,);
let _46: *const i16;
let _47: u128;
let _48: *mut Adt21;
let _49: usize;
let _50: ();
let _51: ();
{
_7 = !_5;
RET = '\u{7822e}';
_4 = 47806_u16 as u32;
_6 = !_2;
_7 = !_8;
Goto(bb1)
}
bb1 = {
_4 = 7407645281180334003_u64 as u32;
_12 = -(-9223372036854775808_isize);
_7 = _1 - _1;
_7 = _3 - _8;
_2 = (-51035263437793696387818491268737843001_i128) as u32;
_2 = !_5;
_3 = !_2;
_4 = _8 * _3;
_3 = !_8;
_13 = 133_u8 as f32;
_4 = 1745134960_i32 as u32;
_4 = !_8;
_8 = _5 >> _2;
RET = '\u{33600}';
_13 = 115956058390394081254819050865690279409_u128 as f32;
_4 = _8;
_2 = !_5;
_7 = 591_u16 as u32;
_4 = _6;
_8 = _1;
_15 = 117_u8;
_15 = 8227899965047346056_i64 as u8;
_3 = _2;
Goto(bb2)
}
bb2 = {
_15 = (-1608518672_i32) as u8;
_4 = !_8;
_1 = 4_usize as u32;
_7 = !_6;
_6 = _7 * _5;
_8 = 3214750451626643286_i64 as u32;
RET = '\u{78f7c}';
RET = '\u{df629}';
Goto(bb3)
}
bb3 = {
_4 = !_3;
_15 = 91_u8 >> _4;
_17 = -_12;
_8 = !_2;
_17 = _12;
RET = '\u{33493}';
_4 = (-30703_i16) as u32;
_18 = [19774776105235285527294726121045103102_i128,(-148914381394654603468732964328071647597_i128),130539938034001696692350187665853684148_i128,169071058815614043231759091765930136014_i128,(-37877227124188275896463141341590163923_i128),(-49835320662621781369826606919021239595_i128),24909919620138694261189659011501530899_i128,144469431097738786890367668325310514895_i128];
_5 = _3 >> _6;
_16 = RET;
_18 = [7110663955783602482000358863953390981_i128,65457486869467513669616979826879461774_i128,78880106994851626312697712941988524654_i128,21646046658020443221226112858651770674_i128,45393366428295431594063626448902745160_i128,150649638350506216973261900572703589741_i128,36500974124873449360677176227486897392_i128,127675177923119997986516426162122874489_i128];
_6 = _7;
_19 = !_6;
_7 = _3;
_5 = false as u32;
_5 = !_3;
_2 = 2162_i16 as u32;
_2 = !_7;
_6 = _2 << _8;
_13 = 4_usize as f32;
_4 = _6;
_16 = RET;
Goto(bb4)
}
bb4 = {
_4 = _6;
_17 = -_12;
_20 = 17895268053915730051_u64 >> _3;
_2 = !_19;
_6 = _5;
_19 = _4 ^ _4;
_3 = _19;
_5 = !_19;
_8 = _4;
_1 = !_3;
_17 = -_12;
RET = _16;
_1 = _19 >> _3;
_13 = (-348057490_i32) as f32;
_13 = _19 as f32;
_1 = 7_usize as u32;
RET = _16;
_8 = _20 as u32;
Goto(bb5)
}
bb5 = {
_13 = 47817635982244676731782623780550768756_i128 as f32;
Call(RET = fn10(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15 = 18_i8 as u8;
_7 = _19 | _5;
_12 = _17;
_19 = !_1;
_19 = _4;
_15 = 83_u8 ^ 223_u8;
RET = _16;
_8 = _15 as u32;
_6 = 68_i8 as u32;
_19 = _3 - _7;
_12 = 59602280367887861550339242297058592695_u128 as isize;
_3 = _19 + _2;
_16 = RET;
_6 = _20 as u32;
RET = _16;
_6 = (-1781134744121843489_i64) as u32;
_12 = _17;
RET = _16;
RET = _16;
_7 = 53311_u16 as u32;
_15 = 90_u8 * 130_u8;
_20 = 10233032024099187332_u64 - 17226428017783060060_u64;
_19 = 135682018120572233228484699358670864363_u128 as u32;
_3 = !_2;
_20 = !15682302422535686187_u64;
_5 = _4;
_25.3.1 = [(-72_i8),(-41_i8),(-63_i8)];
Call(_17 = core::intrinsics::transmute(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_25.2.fld4 = true as u8;
_4 = _3;
_25.1 = &_9;
Call(_18 = fn18(_5, _7, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = _17 as f32;
_20 = !8143963269032693572_u64;
_25.2.fld0 = 3764_i16 as f64;
_30.0.0.fld3 = 94096986407757308807583192444094087203_u128 as i32;
_7 = RET as u32;
_3 = !_2;
_25.2.fld3 = (_2, _13);
_27.0.2 = [13773_i16,(-1641_i16),5384_i16,(-6129_i16),18413_i16,6095_i16,18482_i16,2487_i16];
_2 = _20 as u32;
_27.0.3 = _27.0.2;
_27.0.0 = [_30.0.0.fld3,_30.0.0.fld3,_30.0.0.fld3,_30.0.0.fld3];
_22 = core::ptr::addr_of_mut!(_15);
_25.3.3 = !7_usize;
_28 = RET;
_30.0.0.fld1 = RET;
_10 = core::ptr::addr_of!(_30.0);
_25.0 = core::ptr::addr_of!(_25.3.0);
_25.3.2 = [113933692313576953411808124927625171843_i128,137270392795707283510486642869995518720_i128,125065200944185778009473737207198059545_i128,106154671083109012179018103351738533368_i128,139430731777693026303550503537782481900_i128,(-35149006418008534752845941783656907531_i128),16444888895043320394979629826923782869_i128,93999781717810842856935712815008615558_i128];
_23 = true ^ true;
_12 = 99041313022486080490244359057062066957_u128 as isize;
_24 = core::ptr::addr_of_mut!(_30.0.0.fld0);
_25.3.2 = _18;
_12 = -_17;
(*_24) = core::ptr::addr_of!(_25.3.0);
Goto(bb9)
}
bb9 = {
_32 = _3;
(*_10).0.fld2 = !_12;
_25.3.3 = _13 as usize;
_30.1.fld3 = (*_10).0.fld3 & (*_10).0.fld3;
_30.1.fld0 = core::ptr::addr_of!(_25.3.0);
(*_10).0.fld1 = _28;
(*_10).0.fld1 = _28;
_5 = _25.2.fld3.0 * _3;
_8 = !_5;
_25.3.0 = _23 as i16;
_25.2.fld2 = !63222_u16;
_30.2 = &_25.3.0;
(*_10).0 = Adt38 { fld0: Move(_30.1.fld0),fld1: RET,fld2: _17,fld3: _30.1.fld3 };
_30.0.0.fld2 = _25.2.fld2 as isize;
_25.2.fld1 = _30.0.0.fld1;
(*_10).0.fld0 = core::ptr::addr_of!(_25.3.0);
_22 = core::ptr::addr_of_mut!((*_22));
_4 = _25.3.0 as u32;
_32 = _5;
Call(_30.1.fld0 = core::intrinsics::arith_offset(_30.0.0.fld0, (-9223372036854775808_isize)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_30.0.0.fld3 = -_30.1.fld3;
_17 = (*_10).0.fld2;
_30.0.0.fld2 = _17;
_30.0.0.fld0 = core::ptr::addr_of!(_25.3.0);
_30.1.fld1 = _25.2.fld1;
(*_10).0 = Adt38 { fld0: Move(_25.0),fld1: _25.2.fld1,fld2: _17,fld3: _30.1.fld3 };
_21 = Adt58::Variant0 { fld0: Move((*_10).0) };
Goto(bb11)
}
bb11 = {
_30.1.fld1 = _28;
(*_10).0.fld2 = Field::<Adt38>(Variant(_21, 0), 0).fld2 - _12;
Goto(bb12)
}
bb12 = {
(*_10).0.fld1 = _25.2.fld1;
_39 = !_23;
(*_10).0 = Move(Field::<Adt38>(Variant(_21, 0), 0));
_34 = _25.3.3 as f32;
_25.2.fld3 = (_32, _13);
(*_10).0.fld3 = _30.1.fld3 ^ _30.1.fld3;
place!(Field::<Adt38>(Variant(_21, 0), 0)).fld3 = (*_10).0.fld3;
_28 = _30.1.fld1;
_29 = Adt58::Variant3 { fld0: 8508632947871731016_i64 };
_10 = core::ptr::addr_of!((*_10));
_25.0 = core::ptr::addr_of!(_25.3.0);
RET = _25.2.fld1;
Goto(bb13)
}
bb13 = {
_17 = _30.0.0.fld2;
_30.3 = _30.0.0.fld2 ^ (*_10).0.fld2;
_30.0.0.fld1 = _25.2.fld1;
_30.1 = Adt38 { fld0: Move((*_24)),fld1: _30.0.0.fld1,fld2: _17,fld3: (*_10).0.fld3 };
_30.0.0.fld3 = _25.2.fld4 as i32;
(*_10).0.fld3 = -_30.1.fld3;
_30.0.0.fld0 = core::ptr::addr_of!(_36);
place!(Field::<Adt38>(Variant(_21, 0), 0)) = Adt38 { fld0: Move(_30.1.fld0),fld1: _25.2.fld1,fld2: (*_10).0.fld2,fld3: (*_10).0.fld3 };
_37 = [Field::<Adt38>(Variant(_21, 0), 0).fld3,_30.0.0.fld3,_30.1.fld3,(*_10).0.fld3];
_19 = _32;
_25.2.fld3.0 = _19 << _15;
_23 = _39;
_25.2.fld3 = (_32, _13);
_41.0.3 = [_25.3.0,_25.3.0,_25.3.0,_25.3.0,_25.3.0,_25.3.0,_25.3.0,_25.3.0];
SetDiscriminant(_21, 2);
(*_10).0.fld3 = _30.1.fld3 << _25.2.fld3.0;
_31 = core::ptr::addr_of!((*_10));
RET = (*_10).0.fld1;
_2 = !_32;
_30.0.0.fld1 = RET;
_30.0.0.fld3 = 289167444846520022848316593999706945290_u128 as i32;
Call((*_10).0.fld2 = core::intrinsics::bswap(_30.1.fld2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_44 = core::ptr::addr_of!(_25.2.fld3.1);
_30.2 = &_25.3.0;
(*_31).0.fld2 = -_30.3;
_41.0.0 = [_30.1.fld3,(*_10).0.fld3,_30.1.fld3,_30.0.0.fld3];
(*_10).0.fld3 = _25.3.3 as i32;
_30.1 = Adt38 { fld0: Move(_25.0),fld1: _30.0.0.fld1,fld2: (*_10).0.fld2,fld3: (*_10).0.fld3 };
_36 = -_25.3.0;
(*_31).0.fld2 = _30.1.fld2 >> _5;
(*_22) = !_25.2.fld4;
_36 = _25.3.0 + _25.3.0;
_2 = _25.2.fld3.0 * _32;
(*_22) = 8042303924641016026_i64 as u8;
_7 = _23 as u32;
_41.0.1 = &_9;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(9_usize, 23_usize, Move(_23), 36_usize, Move(_36), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(9_usize, 16_usize, Move(_16), 28_usize, Move(_28), 19_usize, Move(_19), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(9_usize, 3_usize, Move(_3), 1_usize, Move(_1), 51_usize, _51, 51_usize, _51), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u32) -> char {
mir! {
type RET = char;
let _2: u8;
let _3: [bool; 3];
let _4: *const (Adt38,);
let _5: char;
let _6: bool;
let _7: f32;
let _8: f32;
let _9: Adt21;
let _10: Adt21;
let _11: (([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]),);
let _12: f64;
let _13: i16;
let _14: *mut u128;
let _15: Adt47;
let _16: [i8; 3];
let _17: [u16; 6];
let _18: f32;
let _19: Adt47;
let _20: *const i16;
let _21: &'static &'static (u32, f32);
let _22: isize;
let _23: f32;
let _24: [u32; 5];
let _25: ();
let _26: ();
{
RET = '\u{1e832}';
RET = '\u{f8d42}';
RET = '\u{ca2e0}';
_1 = 907736881_u32 * 360221965_u32;
RET = '\u{cbf2c}';
RET = '\u{67b8e}';
RET = '\u{25ca9}';
RET = '\u{3fc3d}';
_1 = 1309113034_u32 >> 32568_i16;
Call(_1 = fn11(RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !1266359345_u32;
_1 = !1157254392_u32;
RET = '\u{6de18}';
_2 = !97_u8;
_1 = 1260231780_u32;
_2 = !151_u8;
RET = '\u{28feb}';
RET = '\u{7becd}';
_2 = 141_u8;
_2 = !103_u8;
RET = '\u{a6813}';
_2 = !16_u8;
_1 = !3140936988_u32;
_6 = RET >= RET;
_1 = 4088901537_u32 << _2;
RET = '\u{88624}';
RET = '\u{23b49}';
_5 = RET;
_6 = false;
_3 = [_6,_6,_6];
_2 = (-2049_i16) as u8;
_6 = _2 != _2;
_5 = RET;
Goto(bb2)
}
bb2 = {
_9.fld3.0 = _1 + _1;
_9.fld4 = _2 ^ _2;
RET = _5;
_9.fld0 = 15642_u16 as f64;
_9.fld3.1 = 21_i8 as f32;
_7 = -_9.fld3.1;
_3 = [_6,_6,_6];
_10.fld4 = !_9.fld4;
_1 = !_9.fld3.0;
_1 = _9.fld3.0 ^ _9.fld3.0;
_10.fld0 = 261680057899726665172411689590961730702_u128 as f64;
_8 = -_7;
_7 = _1 as f32;
_13 = -(-11706_i16);
_11.0.3 = [_13,_13,_13,_13,_13,_13,_13,_13];
_1 = _9.fld3.0;
_12 = _10.fld0 - _10.fld0;
_9.fld4 = !_10.fld4;
_9.fld2 = !1287_u16;
_10.fld2 = _9.fld2;
_5 = RET;
Goto(bb3)
}
bb3 = {
RET = _5;
_11.0.2 = _11.0.3;
_9.fld4 = _10.fld4 & _10.fld4;
_7 = -_8;
_13 = _1 as i16;
Call(_10.fld2 = core::intrinsics::bswap(_9.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = 196904387858677692191651105776247961404_u128 as u32;
_9.fld1 = RET;
_16 = [86_i8,9_i8,102_i8];
_9.fld0 = -_10.fld0;
_6 = !true;
_13 = _6 as i16;
Goto(bb5)
}
bb5 = {
_11.0.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_2 = _10.fld4;
_18 = _7 * _8;
_17 = [_9.fld2,_9.fld2,_10.fld2,_9.fld2,_10.fld2,_9.fld2];
_12 = _9.fld0 + _10.fld0;
_10.fld1 = _5;
_10 = Adt21 { fld0: _12,fld1: _9.fld1,fld2: _9.fld2,fld3: _9.fld3,fld4: _9.fld4 };
_13 = _12 as i16;
Goto(bb6)
}
bb6 = {
_2 = !_10.fld4;
_10.fld0 = -_12;
_5 = _9.fld1;
_2 = !_10.fld4;
_10.fld2 = !_9.fld2;
_15 = Adt47::Variant1 { fld0: _9.fld3.0 };
_18 = _13 as f32;
SetDiscriminant(_15, 3);
place!(Field::<bool>(Variant(_15, 3), 0)) = !_6;
place!(Field::<f32>(Variant(_15, 3), 3)) = _7 * _10.fld3.1;
Goto(bb7)
}
bb7 = {
_10.fld4 = _9.fld4 >> _10.fld2;
_9 = Adt21 { fld0: _10.fld0,fld1: _5,fld2: _10.fld2,fld3: _10.fld3,fld4: _10.fld4 };
place!(Field::<*const f32>(Variant(_15, 3), 2)) = core::ptr::addr_of!(_9.fld3.1);
Goto(bb8)
}
bb8 = {
_16 = [(-112_i8),50_i8,12_i8];
_9.fld3.0 = !_1;
_9.fld1 = _5;
RET = _5;
_10.fld4 = !_2;
_18 = _10.fld3.1 * _10.fld3.1;
place!(Field::<*const [i32; 4]>(Variant(_15, 3), 1)) = core::ptr::addr_of!(_11.0.0);
_5 = _10.fld1;
_11.0.0 = [(-1630014017_i32),501984510_i32,(-505539850_i32),1572055873_i32];
_9.fld3.0 = !_1;
_10.fld3.1 = _18 - _9.fld3.1;
_15 = Adt47::Variant1 { fld0: _10.fld3.0 };
_19 = Move(_15);
Goto(bb9)
}
bb9 = {
_22 = 9223372036854775807_isize;
_20 = core::ptr::addr_of!(_13);
_9.fld2 = (-231983876_i32) as u16;
RET = _5;
_10.fld3.0 = Field::<u32>(Variant(_19, 1), 0);
SetDiscriminant(_19, 3);
match _22 {
0 => bb4,
1 => bb6,
2 => bb10,
3 => bb11,
4 => bb12,
9223372036854775807 => bb14,
_ => bb13
}
}
bb10 = {
_16 = [(-112_i8),50_i8,12_i8];
_9.fld3.0 = !_1;
_9.fld1 = _5;
RET = _5;
_10.fld4 = !_2;
_18 = _10.fld3.1 * _10.fld3.1;
place!(Field::<*const [i32; 4]>(Variant(_15, 3), 1)) = core::ptr::addr_of!(_11.0.0);
_5 = _10.fld1;
_11.0.0 = [(-1630014017_i32),501984510_i32,(-505539850_i32),1572055873_i32];
_9.fld3.0 = !_1;
_10.fld3.1 = _18 - _9.fld3.1;
_15 = Adt47::Variant1 { fld0: _10.fld3.0 };
_19 = Move(_15);
Goto(bb9)
}
bb11 = {
_9.fld3.0 = _1 + _1;
_9.fld4 = _2 ^ _2;
RET = _5;
_9.fld0 = 15642_u16 as f64;
_9.fld3.1 = 21_i8 as f32;
_7 = -_9.fld3.1;
_3 = [_6,_6,_6];
_10.fld4 = !_9.fld4;
_1 = !_9.fld3.0;
_1 = _9.fld3.0 ^ _9.fld3.0;
_10.fld0 = 261680057899726665172411689590961730702_u128 as f64;
_8 = -_7;
_7 = _1 as f32;
_13 = -(-11706_i16);
_11.0.3 = [_13,_13,_13,_13,_13,_13,_13,_13];
_1 = _9.fld3.0;
_12 = _10.fld0 - _10.fld0;
_9.fld4 = !_10.fld4;
_9.fld2 = !1287_u16;
_10.fld2 = _9.fld2;
_5 = RET;
Goto(bb3)
}
bb12 = {
_2 = !_10.fld4;
_10.fld0 = -_12;
_5 = _9.fld1;
_2 = !_10.fld4;
_10.fld2 = !_9.fld2;
_15 = Adt47::Variant1 { fld0: _9.fld3.0 };
_18 = _13 as f32;
SetDiscriminant(_15, 3);
place!(Field::<bool>(Variant(_15, 3), 0)) = !_6;
place!(Field::<f32>(Variant(_15, 3), 3)) = _7 * _10.fld3.1;
Goto(bb7)
}
bb13 = {
_1 = !1266359345_u32;
_1 = !1157254392_u32;
RET = '\u{6de18}';
_2 = !97_u8;
_1 = 1260231780_u32;
_2 = !151_u8;
RET = '\u{28feb}';
RET = '\u{7becd}';
_2 = 141_u8;
_2 = !103_u8;
RET = '\u{a6813}';
_2 = !16_u8;
_1 = !3140936988_u32;
_6 = RET >= RET;
_1 = 4088901537_u32 << _2;
RET = '\u{88624}';
RET = '\u{23b49}';
_5 = RET;
_6 = false;
_3 = [_6,_6,_6];
_2 = (-2049_i16) as u8;
_6 = _2 != _2;
_5 = RET;
Goto(bb2)
}
bb14 = {
(*_20) = 12_i16 ^ 9734_i16;
_9.fld1 = _5;
place!(Field::<*const [i32; 4]>(Variant(_19, 3), 1)) = core::ptr::addr_of!(_11.0.0);
_24 = [_10.fld3.0,_1,_9.fld3.0,_1,_1];
_20 = core::ptr::addr_of!((*_20));
_2 = _10.fld4;
_10.fld3.1 = _8;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(10_usize, 3_usize, Move(_3), 5_usize, Move(_5), 24_usize, Move(_24), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(10_usize, 1_usize, Move(_1), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: char,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: char) -> u32 {
mir! {
type RET = u32;
let _13: *const *const i16;
let _14: i128;
let _15: f32;
let _16: [u8; 4];
let _17: u128;
let _18: f32;
let _19: u128;
let _20: *const [i32; 4];
let _21: u128;
let _22: i8;
let _23: i64;
let _24: f32;
let _25: [i128; 8];
let _26: [i64; 3];
let _27: (*mut u8, [i32; 4]);
let _28: &'static &'static [u16; 5];
let _29: u16;
let _30: f64;
let _31: (u128,);
let _32: i32;
let _33: u8;
let _34: u32;
let _35: bool;
let _36: i8;
let _37: [i64; 3];
let _38: &'static i16;
let _39: [u16; 5];
let _40: char;
let _41: [i64; 3];
let _42: ();
let _43: ();
{
_7 = _5;
_12 = _5;
_11 = _8;
RET = !1788387906_u32;
_10 = _12;
_2 = _6;
Call(_14 = fn12(_5, _6, _2, _4, RET, _10, _12, _8, RET, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = false as u32;
_11 = _9;
RET = 1294432618_i32 as u32;
_2 = _3;
_7 = _4;
_6 = _1;
_2 = _12;
_4 = _5;
_14 = (-37984187738952237445275940819856639752_i128);
_11 = _8;
_3 = _6;
_10 = _7;
_9 = _10;
_10 = _6;
_1 = _3;
_15 = 251_u8 as f32;
_2 = _4;
_16 = [40_u8,107_u8,60_u8,215_u8];
RET = 1054563303_u32;
Goto(bb2)
}
bb2 = {
_7 = _8;
_10 = _8;
_9 = _10;
_11 = _10;
_7 = _1;
_18 = _15;
_14 = (-19040768532915060635928525363953283357_i128);
Call(_4 = fn13(_9, _3, _16, _18, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _5;
RET = !3708235437_u32;
_11 = _5;
RET = 2910478667_u32 | 999254819_u32;
_2 = _9;
_16 = [99_u8,133_u8,130_u8,38_u8];
_7 = _11;
_19 = 3827_u16 as u128;
_8 = _10;
_10 = _7;
_16 = [82_u8,62_u8,39_u8,56_u8];
_4 = _1;
_16 = [240_u8,106_u8,113_u8,22_u8];
_9 = _12;
RET = 2686980242_u32;
_10 = _8;
RET = _14 as u32;
_5 = _1;
_21 = _19 + _19;
_21 = _19 >> RET;
_3 = _1;
Goto(bb4)
}
bb4 = {
_5 = _12;
_5 = _7;
_4 = _7;
_4 = _7;
_9 = _4;
_2 = _5;
_4 = _10;
_2 = _1;
RET = 1797535306_u32;
RET = !732388435_u32;
_18 = (-35_i8) as f32;
_10 = _9;
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
321241598388023402827446082067814928099 => bb10,
_ => bb9
}
}
bb5 = {
_8 = _5;
RET = !3708235437_u32;
_11 = _5;
RET = 2910478667_u32 | 999254819_u32;
_2 = _9;
_16 = [99_u8,133_u8,130_u8,38_u8];
_7 = _11;
_19 = 3827_u16 as u128;
_8 = _10;
_10 = _7;
_16 = [82_u8,62_u8,39_u8,56_u8];
_4 = _1;
_16 = [240_u8,106_u8,113_u8,22_u8];
_9 = _12;
RET = 2686980242_u32;
_10 = _8;
RET = _14 as u32;
_5 = _1;
_21 = _19 + _19;
_21 = _19 >> RET;
_3 = _1;
Goto(bb4)
}
bb6 = {
_7 = _8;
_10 = _8;
_9 = _10;
_11 = _10;
_7 = _1;
_18 = _15;
_14 = (-19040768532915060635928525363953283357_i128);
Call(_4 = fn13(_9, _3, _16, _18, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = false as u32;
_11 = _9;
RET = 1294432618_i32 as u32;
_2 = _3;
_7 = _4;
_6 = _1;
_2 = _12;
_4 = _5;
_14 = (-37984187738952237445275940819856639752_i128);
_11 = _8;
_3 = _6;
_10 = _7;
_9 = _10;
_10 = _6;
_1 = _3;
_15 = 251_u8 as f32;
_2 = _4;
_16 = [40_u8,107_u8,60_u8,215_u8];
RET = 1054563303_u32;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_15 = _18 + _18;
_21 = _19 * _19;
_16 = [74_u8,253_u8,107_u8,78_u8];
_21 = RET as u128;
_11 = _1;
_2 = _10;
RET = !1554519037_u32;
_17 = _21;
_8 = _10;
_9 = _8;
_15 = -_18;
_6 = _5;
_8 = _4;
_3 = _12;
_26 = [(-3275152777943483135_i64),(-6864693105774632675_i64),4714562324349712945_i64];
_8 = _11;
_4 = _12;
_14 = (-63717190434573431045187835004603933555_i128) >> _19;
_17 = !_21;
_1 = _12;
Goto(bb11)
}
bb11 = {
_20 = core::ptr::addr_of!(_27.1);
_15 = _18;
RET = 42911746_u32;
_4 = _8;
RET = 3510656015_u32 ^ 1821361096_u32;
(*_20) = [(-1671186510_i32),(-524482700_i32),1117909552_i32,(-1110298828_i32)];
_24 = _18;
(*_20) = [(-30697304_i32),1961982943_i32,(-253297461_i32),(-1513102753_i32)];
(*_20) = [(-1275043853_i32),(-1939801129_i32),1593365790_i32,(-1097286412_i32)];
_30 = 0_usize as f64;
_5 = _7;
Goto(bb12)
}
bb12 = {
_32 = (-1626894256_i32);
_23 = 4140161493956870393_i64 + 8859798052593599515_i64;
_9 = _10;
_31.0 = _17;
_19 = !_31.0;
_7 = _8;
_33 = 126_u8 | 14_u8;
(*_20) = [_32,_32,_32,_32];
_15 = -_18;
_5 = _10;
_27.1 = [_32,_32,_32,_32];
_27.0 = core::ptr::addr_of_mut!(_33);
_24 = _30 as f32;
_26 = [_23,_23,_23];
_29 = 55242_u16;
_7 = _9;
_33 = 252_u8;
(*_20) = [_32,_32,_32,_32];
_27.1 = [_32,_32,_32,_32];
(*_20) = [_32,_32,_32,_32];
_12 = _8;
_11 = _2;
_7 = _1;
(*_20) = [_32,_32,_32,_32];
_22 = _29 as i8;
_25 = [_14,_14,_14,_14,_14,_14,_14,_14];
_12 = _11;
RET = 4150790841_u32;
Goto(bb13)
}
bb13 = {
_11 = _10;
_8 = _4;
_27.1 = [_32,_32,_32,_32];
_26 = [_23,_23,_23];
_9 = _10;
_1 = _3;
Call(_17 = core::intrinsics::bswap(_31.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_1 = _12;
_20 = core::ptr::addr_of!((*_20));
_27.1 = [_32,_32,_32,_32];
_10 = _8;
RET = 998435079_u32 * 3105539775_u32;
_25 = [_14,_14,_14,_14,_14,_14,_14,_14];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(11_usize, 14_usize, Move(_14), 3_usize, Move(_3), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(11_usize, 31_usize, Move(_31), 6_usize, Move(_6), 19_usize, Move(_19), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(11_usize, 11_usize, Move(_11), 26_usize, Move(_26), 23_usize, Move(_23), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: char,mut _2: char,mut _3: char,mut _4: char,mut _5: u32,mut _6: char,mut _7: char,mut _8: char,mut _9: u32,mut _10: char,mut _11: char) -> i128 {
mir! {
type RET = i128;
let _12: usize;
let _13: &'static i16;
let _14: &'static &'static (u32, f32);
let _15: bool;
let _16: isize;
let _17: (&'static Adt27, [i32; 4], Adt47);
let _18: *mut &'static Adt41;
let _19: u8;
let _20: Adt47;
let _21: i16;
let _22: &'static bool;
let _23: [u32; 7];
let _24: (u16,);
let _25: (u128,);
let _26: [u16; 5];
let _27: u128;
let _28: *const i16;
let _29: (&'static Adt27, [i32; 4], Adt47);
let _30: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _31: *mut Adt21;
let _32: ();
let _33: ();
{
_11 = _2;
_11 = _10;
_7 = _11;
_9 = !_5;
_7 = _4;
RET = -51103377294896496967894009336090375040_i128;
_5 = _9 & _9;
_8 = _3;
_3 = _6;
_3 = _8;
_7 = _4;
RET = (-107582564278751180770234062496501240111_i128) >> _9;
Goto(bb1)
}
bb1 = {
_8 = _4;
_12 = 5282388097474967594_usize;
_11 = _1;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
5282388097474967594 => bb10,
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
_10 = _3;
RET = 167721303565092338111131394787610563343_i128 - 119147903152189634145776546634548330451_i128;
_6 = _10;
_11 = _1;
_6 = _11;
_2 = _1;
_9 = _5 << _12;
_1 = _4;
_2 = _1;
_12 = 6_usize << RET;
_2 = _6;
_3 = _6;
RET = true as i128;
RET = (-109453484223881804732894517331397934572_i128) + 123190859546327990670767236464446606809_i128;
_11 = _2;
_8 = _6;
_2 = _6;
_8 = _10;
_11 = _4;
_8 = _11;
_4 = _3;
RET = 80816160492973069689988729164185175139_i128 + (-110825722773200173347883512437580520197_i128);
_10 = _8;
_4 = _3;
_15 = true;
Goto(bb11)
}
bb11 = {
_8 = _3;
_8 = _1;
_12 = (-688878769_i32) as usize;
Goto(bb12)
}
bb12 = {
_16 = _6 as isize;
_12 = !781184285984437451_usize;
_9 = _5;
_17.1 = [(-917198572_i32),1766786249_i32,(-1739839719_i32),(-1463490438_i32)];
_7 = _6;
_8 = _4;
_5 = _15 as u32;
_7 = _11;
_17.1 = [(-1756762302_i32),1844140730_i32,(-1964215972_i32),(-431115965_i32)];
_11 = _1;
RET = (-832335822_i32) as i128;
_2 = _8;
_9 = 316293559_i32 as u32;
_2 = _8;
_10 = _6;
_6 = _3;
_12 = (-5636920356351702015_i64) as usize;
_19 = 20683_u16 as u8;
_19 = 221_u8;
_15 = true;
_9 = _5 & _5;
_9 = _5;
_2 = _1;
RET = _12 as i128;
RET = 59551_u16 as i128;
_7 = _8;
Goto(bb13)
}
bb13 = {
_11 = _7;
_6 = _10;
_13 = &_21;
_5 = (-27312_i16) as u32;
_19 = !17_u8;
_4 = _1;
_19 = 235_u8;
_21 = _4 as i16;
_1 = _4;
_3 = _8;
_24.0 = !46410_u16;
_26 = [_24.0,_24.0,_24.0,_24.0,_24.0];
_12 = _15 as usize;
_1 = _7;
_22 = &_15;
_19 = _12 as u8;
_8 = _7;
_13 = &_21;
_26 = [_24.0,_24.0,_24.0,_24.0,_24.0];
_19 = !231_u8;
Call(_16 = core::intrinsics::bswap((-116_isize)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_1 = _2;
_9 = _12 as u32;
_21 = !9431_i16;
_3 = _2;
_17.2 = Adt47::Variant1 { fld0: _9 };
_21 = (-5750_i16) ^ 4229_i16;
_20 = Move(_17.2);
_29.2 = Move(_20);
_25.0 = 333046597836206208543911031672248759328_u128 >> _24.0;
_8 = _3;
SetDiscriminant(_29.2, 1);
_9 = _5;
_13 = &_21;
_30.0.3.2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_19 = _8 as u8;
_30.0.2.fld4 = !_19;
_24.0 = 16966_u16 >> _21;
_4 = _11;
_12 = 7_usize;
_28 = core::ptr::addr_of!(_30.0.3.0);
_29.1 = [1033997675_i32,2101178685_i32,(-456357206_i32),1719833597_i32];
_8 = _6;
_30.0.1 = &_13;
_20 = Adt47::Variant1 { fld0: _9 };
_30.2 = core::ptr::addr_of!(_17.1);
place!(Field::<u32>(Variant(_29.2, 1), 0)) = !_5;
SetDiscriminant(_29.2, 3);
_10 = _3;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(12_usize, 26_usize, Move(_26), 16_usize, Move(_16), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(12_usize, 2_usize, Move(_2), 10_usize, Move(_10), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(12_usize, 19_usize, Move(_19), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: char,mut _3: [u8; 4],mut _4: f32,mut _5: char) -> char {
mir! {
type RET = char;
let _6: [i128; 8];
let _7: [i32; 3];
let _8: &'static Adt27;
let _9: &'static i16;
let _10: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _11: u16;
let _12: [u32; 5];
let _13: [u16; 5];
let _14: usize;
let _15: *const [i32; 4];
let _16: (i16, [i8; 3], [i128; 8], usize);
let _17: u16;
let _18: Adt48;
let _19: isize;
let _20: isize;
let _21: ((Adt38,), (u32, f32), u128);
let _22: &'static [u8; 4];
let _23: i16;
let _24: *mut u128;
let _25: &'static [u8; 4];
let _26: char;
let _27: [u16; 6];
let _28: ();
let _29: ();
{
_5 = _2;
_1 = _2;
_1 = _5;
_4 = (-9223372036854775808_isize) as f32;
_2 = _5;
_6 = [59099044201452024918442653997867276344_i128,40131255524586561877278884074007445255_i128,(-143683759224208154745428341929967729636_i128),(-126495737738944439311948221637562722926_i128),103177696240083887931008360126680952695_i128,(-2715740577653905577326533720587783369_i128),88447926992975418631004297629926731137_i128,(-1747302688906696566252519328015319948_i128)];
_3 = [173_u8,13_u8,88_u8,192_u8];
_4 = (-105958646866408189168149962631391439389_i128) as f32;
RET = _2;
_6 = [(-90894796265917208746365334279361061280_i128),132195664671953625844084489811878488325_i128,115099659476087170859108436698665366749_i128,(-101907819141039088726220933102754358372_i128),(-46043630899467731047865450763271086849_i128),(-38894650915455199799461808459383552478_i128),164727733826905831682558685592976639195_i128,(-130961273610430461496247403175176589468_i128)];
_1 = _5;
_7 = [(-1989659876_i32),311491925_i32,28651645_i32];
_6 = [129274571557698574644548461611588042412_i128,(-2818996709560281221634413343479158476_i128),3760836474901156558500884878498967576_i128,89768741482082357293240389144780952846_i128,169598461731851538085042915363791250687_i128,(-33602204763135330169382414575624627481_i128),5356604596086036918785432487345446552_i128,130744691052471482356970486345563469504_i128];
_7 = [65837646_i32,(-1101309839_i32),(-288758057_i32)];
_7 = [845943111_i32,(-800519474_i32),(-1722418450_i32)];
_3 = [75_u8,201_u8,238_u8,169_u8];
_1 = _2;
RET = _1;
_6 = [(-34748568524244960238741009310701006789_i128),32393996205431070343581172522788457857_i128,(-80552342807746105649622087353079201593_i128),37821434769549012392882906028776244399_i128,54407773257513371285134016312439854266_i128,(-102251178886748639359963600828293549991_i128),(-57826645581540527099794806314225039152_i128),(-128941284831954251547082105286371732103_i128)];
_1 = RET;
_5 = _1;
RET = _2;
_7 = [(-891571833_i32),2025630870_i32,302786865_i32];
_6 = [36586294633541351510111405591093480671_i128,(-39772627902733418043324811358302588345_i128),19916222042183959682924065030782979864_i128,48694404431027447004268343334067546240_i128,(-124210725900016361233476248947497742910_i128),(-151116244221073827490281280985398096694_i128),59048432008384966307500962352868038447_i128,149774343089249702756733793814198743704_i128];
Goto(bb1)
}
bb1 = {
_5 = _1;
_1 = _2;
_1 = RET;
_4 = 18128772760830597752_u64 as f32;
_2 = RET;
RET = _5;
_6 = [(-55339382414377100028220276778413570096_i128),4113616731620233983635077118198841903_i128,(-78499750356280247265633207830497560864_i128),(-24586141135929703138387599587121313688_i128),88260504240282645239660305790887351553_i128,(-32036587306550260493315162910524953915_i128),(-65685154615090827059857073869460799133_i128),76714916963592616748903395408841891264_i128];
_7 = [1797483999_i32,(-300839195_i32),(-661395150_i32)];
_5 = _1;
RET = _2;
_3 = [58_u8,106_u8,250_u8,209_u8];
Goto(bb2)
}
bb2 = {
_6 = [51143092328840972433100160706978590276_i128,67970723008614368070980547387165940738_i128,(-62914151696551969227685254373542950777_i128),(-5862314036935460326884655717556567484_i128),52238410411663249703632918350690024531_i128,(-20324232348983123300557553028113152783_i128),(-53639982538714597553330791388094377913_i128),18051257231660968886579960888872656764_i128];
_10.0.3.1 = [(-26_i8),105_i8,15_i8];
_10.0.2.fld3.1 = _4;
_10.0.3.3 = 1630181752167066279_usize;
_10.0.2.fld1 = RET;
_10.0.3.1 = [(-99_i8),110_i8,97_i8];
_10.0.2.fld3.1 = _4 * _4;
_10.0.2.fld3.0 = 2259250429_u32;
_12 = [_10.0.2.fld3.0,_10.0.2.fld3.0,_10.0.2.fld3.0,_10.0.2.fld3.0,_10.0.2.fld3.0];
_10.0.2.fld0 = (-31823_i16) as f64;
RET = _5;
RET = _1;
_10.0.3.3 = 7_usize ^ 6_usize;
_10.0.3.1 = [(-13_i8),19_i8,(-114_i8)];
_10.0.3.2 = _6;
_10.0.2.fld4 = !224_u8;
_10.0.2.fld1 = _1;
_11 = 45336_u16 << _10.0.2.fld4;
match _10.0.2.fld3.0 {
0 => bb1,
1 => bb3,
2259250429 => bb5,
_ => bb4
}
}
bb3 = {
_5 = _1;
_1 = _2;
_1 = RET;
_4 = 18128772760830597752_u64 as f32;
_2 = RET;
RET = _5;
_6 = [(-55339382414377100028220276778413570096_i128),4113616731620233983635077118198841903_i128,(-78499750356280247265633207830497560864_i128),(-24586141135929703138387599587121313688_i128),88260504240282645239660305790887351553_i128,(-32036587306550260493315162910524953915_i128),(-65685154615090827059857073869460799133_i128),76714916963592616748903395408841891264_i128];
_7 = [1797483999_i32,(-300839195_i32),(-661395150_i32)];
_5 = _1;
RET = _2;
_3 = [58_u8,106_u8,250_u8,209_u8];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
RET = _2;
_10.0.3.3 = 6_usize ^ 10026982937102437490_usize;
_2 = _1;
_9 = &_10.0.3.0;
_10.0.2.fld3 = (859214259_u32, _4);
Goto(bb6)
}
bb6 = {
RET = _10.0.2.fld1;
_6 = [(-44835649706677851579858668487442669473_i128),(-119534615043175256574930118441204106816_i128),169564823717462189308987335392579293953_i128,37302034278092691355033684921475052873_i128,153782202000198455637848342150897475409_i128,(-128977130882255124283634719653895437224_i128),144019860005535737100657025158943065871_i128,(-45713180667815574924882286416216152703_i128)];
Goto(bb7)
}
bb7 = {
_10.0.0 = core::ptr::addr_of!(_10.0.3.0);
_13 = [_11,_11,_11,_11,_11];
_10.0.2.fld3.1 = 9223372036854775807_isize as f32;
_10.0.2.fld1 = RET;
_10.0.2.fld0 = (-9223372036854775808_isize) as f64;
_10.0.2.fld2 = _10.0.3.3 as u16;
_10.0.3.0 = (-5490_i16);
_10.0.3.1 = [70_i8,62_i8,(-79_i8)];
_7 = [988110256_i32,864216407_i32,(-1784100348_i32)];
_4 = _10.0.2.fld3.1;
_7 = [696797111_i32,(-1333041040_i32),(-981393867_i32)];
_11 = 1893219816_i32 as u16;
_10.0.2.fld3.0 = !2530401768_u32;
_10.0.2.fld4 = 110_u8 ^ 251_u8;
_13 = [_11,_11,_10.0.2.fld2,_11,_11];
_10.0.3.3 = 16046372882952826819_usize >> _10.0.2.fld2;
_11 = !_10.0.2.fld2;
_14 = !_10.0.3.3;
_10.0.2.fld3.1 = 195221901933565294889653286290681577273_u128 as f32;
_2 = RET;
_10.0.1 = &_9;
_2 = _5;
_10.0.1 = &_9;
_10.0.2.fld3.1 = -_4;
_10.0.2.fld3 = (3648687785_u32, _4);
Call(_16 = fn14(_10.0.2.fld3.1, _10.0.3.3, _10.0.3, _10.0.3.2, _10.0.3, _14, Move(_10.0.2), _6, _10.0.3, _3, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10.0.1 = &_9;
Goto(bb9)
}
bb9 = {
_10.0.2.fld0 = _4 as f64;
_10.0.2.fld3.1 = 9223372036854775807_isize as f32;
_10.0.2.fld3.0 = RET as u32;
RET = _1;
_9 = &_10.0.3.0;
_10.0.2.fld3.1 = 7830046210986339072046257063038338669_i128 as f32;
_10.0.2.fld2 = _11;
_9 = &_16.0;
_10.0.1 = &_9;
_9 = &(*_9);
_10.0.2.fld3 = (2807663347_u32, _4);
_10.0.0 = core::ptr::addr_of!((*_9));
_10.0.2.fld2 = (-9223372036854775808_isize) as u16;
_19 = _16.3 as isize;
_10.0.3.2 = [105873071152624774202283773320208744915_i128,(-14930881957942923866846746766784809830_i128),(-3424682930618545075737730468087013273_i128),146584370782036388661227408168002722568_i128,51618436275883177897773912356448618437_i128,68852323487052593485973691799046219745_i128,(-52854250772847108321998366651787633102_i128),74994455099517385024895785127979277719_i128];
_13 = [_11,_11,_11,_11,_11];
_10.0.2.fld3 = (2562074615_u32, _4);
_10.0.2.fld4 = (-4126949226200691448_i64) as u8;
_1 = RET;
_16.2 = [(-104673425766396480593939554728186377921_i128),(-14459119690690328336185829027608699367_i128),(-57751595783597198638330790325578722397_i128),49944912112330224179317728291825297996_i128,(-114886399994951591342496797924567679167_i128),(-161786066944054817990069960230033549340_i128),155714432494740083010179349339781679388_i128,(-157468608701328884263925150073923691829_i128)];
_21.1.1 = -_10.0.2.fld3.1;
_21.2 = !297197882017008730530145447108355944130_u128;
_7 = [(-1702019303_i32),(-1716255854_i32),663417603_i32];
_9 = &(*_9);
_10.0.2.fld1 = _1;
match _10.0.2.fld3.0 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb10,
2562074615 => bb12,
_ => bb11
}
}
bb10 = {
_10.0.1 = &_9;
Goto(bb9)
}
bb11 = {
_5 = _1;
_1 = _2;
_1 = RET;
_4 = 18128772760830597752_u64 as f32;
_2 = RET;
RET = _5;
_6 = [(-55339382414377100028220276778413570096_i128),4113616731620233983635077118198841903_i128,(-78499750356280247265633207830497560864_i128),(-24586141135929703138387599587121313688_i128),88260504240282645239660305790887351553_i128,(-32036587306550260493315162910524953915_i128),(-65685154615090827059857073869460799133_i128),76714916963592616748903395408841891264_i128];
_7 = [1797483999_i32,(-300839195_i32),(-661395150_i32)];
_5 = _1;
RET = _2;
_3 = [58_u8,106_u8,250_u8,209_u8];
Goto(bb2)
}
bb12 = {
_21.1.0 = !_10.0.2.fld3.0;
_10.0.2.fld2 = _11 | _11;
_23 = (*_9);
_10.0.2.fld0 = _21.2 as f64;
_10.0.2.fld3.1 = _21.1.1 + _21.1.1;
_21.0.0.fld3 = !(-1946176472_i32);
_21.0.0 = Adt38 { fld0: Move(_10.0.0),fld1: _10.0.2.fld1,fld2: _19,fld3: 1883321919_i32 };
_5 = _2;
_3 = [_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4];
_21.2 = 304863867950916587738219411284613321580_u128;
_6 = [(-17278110219425612145121392658492436179_i128),48989102835694880041577882191605013238_i128,(-80894758723634884560793429025954167456_i128),(-59761789610431789323544742243219785698_i128),(-82938961600485012583192430704664751513_i128),(-161127737030168357168341826523316837403_i128),92065409076028074893582712345928673621_i128,46586271973424194709041445523799946343_i128];
_5 = RET;
_9 = &_16.0;
_17 = _16.3 as u16;
_13 = [_17,_17,_10.0.2.fld2,_10.0.2.fld2,_11];
_10.0.2.fld3.0 = !_21.1.0;
_20 = _21.0.0.fld2;
_21.0.0.fld2 = _19;
Goto(bb13)
}
bb13 = {
_3 = [_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4];
_7 = [_21.0.0.fld3,_21.0.0.fld3,_21.0.0.fld3];
_13 = [_17,_17,_17,_10.0.2.fld2,_17];
_9 = &(*_9);
_21.0.0.fld2 = _20 >> _21.0.0.fld3;
_21.0.0.fld2 = -_19;
_16.1 = [54_i8,(-41_i8),(-74_i8)];
_21.0.0.fld0 = core::ptr::addr_of!(_10.0.3.0);
_25 = &_3;
_10.0.2.fld1 = _2;
_26 = _5;
_10.0.2.fld3.0 = _21.1.0 * _21.1.0;
_3 = [_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4,_10.0.2.fld4];
_10.0.1 = &_9;
_2 = _26;
_14 = !_16.3;
_10.0.2.fld0 = _17 as f64;
Goto(bb14)
}
bb14 = {
_10.0.2.fld3 = _21.1;
_10.0.2.fld0 = (-107_i8) as f64;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(13_usize, 16_usize, Move(_16), 23_usize, Move(_23), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(13_usize, 6_usize, Move(_6), 7_usize, Move(_7), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: f32,mut _2: usize,mut _3: (i16, [i8; 3], [i128; 8], usize),mut _4: [i128; 8],mut _5: (i16, [i8; 3], [i128; 8], usize),mut _6: usize,mut _7: Adt21,mut _8: [i128; 8],mut _9: (i16, [i8; 3], [i128; 8], usize),mut _10: [u8; 4],mut _11: [i128; 8]) -> (i16, [i8; 3], [i128; 8], usize) {
mir! {
type RET = (i16, [i8; 3], [i128; 8], usize);
let _12: isize;
let _13: &'static (u32, f32);
let _14: [i32; 8];
let _15: u64;
let _16: bool;
let _17: ((Adt38,), Adt38, &'static i16, isize);
let _18: *mut *const i16;
let _19: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _20: bool;
let _21: isize;
let _22: [i64; 3];
let _23: isize;
let _24: Adt58;
let _25: [i16; 8];
let _26: i8;
let _27: [isize; 5];
let _28: f32;
let _29: [u8; 8];
let _30: *mut Adt21;
let _31: i8;
let _32: [bool; 3];
let _33: bool;
let _34: *const &'static &'static (u32, f32);
let _35: u8;
let _36: u32;
let _37: isize;
let _38: &'static i16;
let _39: ((Adt38,), Adt38, &'static i16, isize);
let _40: bool;
let _41: u64;
let _42: *mut &'static Adt41;
let _43: [i32; 4];
let _44: i64;
let _45: f64;
let _46: (f64,);
let _47: ();
let _48: ();
{
_9.0 = _3.0 << _7.fld4;
_9.0 = !_5.0;
RET = (_3.0, _9.1, _4, _2);
RET.2 = _8;
_3.2 = [45634283687248647239682006322102940795_i128,(-85544804208048673792399844438355391700_i128),(-73909040675182614942408857094957149651_i128),50406695313292928733691150848673818267_i128,(-105973223362489658197502490377462268416_i128),40264102105560804518025697509046503739_i128,51221288143071564984433444489855410115_i128,(-131866000256572948784728448398239678666_i128)];
_8 = [(-169307865059200657482200895904785869471_i128),134539793936878539620269399684926374618_i128,(-90580588778071493401204387113719382853_i128),2219183877865672034793871148448922626_i128,120945397631192239330887239752094712683_i128,(-23913047364663460360136024159924125882_i128),(-162620524464294762109250106746068243801_i128),(-50876430099828775853520773688307684941_i128)];
RET.0 = _5.0;
_7.fld2 = _7.fld4 as u16;
RET.2 = [62732848983307080947637536719347130000_i128,24336424914392697498531847262763453972_i128,(-106183321850892962397814844028337195833_i128),(-31808209378810282472144629318247957355_i128),(-140965562125320548256153159822211246695_i128),(-117156801208939286752416071389459131013_i128),21957405810966648902739108696071332305_i128,(-53653673436272769714686581414431705483_i128)];
_10 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4];
_3.3 = _5.3 | _6;
_7.fld3.1 = _1;
RET.3 = false as usize;
_1 = -_7.fld3.1;
_7.fld4 = 21_u8;
_4 = [(-33049421455186064359259985622504520075_i128),71665878982767306538216406572268978365_i128,147845011328075278364664590488598700145_i128,(-116215865362816425191435411050745936375_i128),368669566014977097692538859046508816_i128,(-162861882246783250486366737677675085790_i128),121930640045244931540196097847486849826_i128,(-26043838555889961711312230227330711260_i128)];
_3 = (RET.0, _9.1, RET.2, _2);
_10 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4];
RET.3 = _3.3 >> _3.3;
_9.1 = [(-29_i8),41_i8,93_i8];
_4 = [140582148931035927316193450277795969687_i128,(-32450861510293066173391348083179927062_i128),(-155760501281566155810338468308142320850_i128),47676920891804793543253865099502655214_i128,103632827849678132967639813054366097076_i128,64871316836985431718350307626139906651_i128,22554457736505501496313742273331335498_i128,96481290342244994864410037607774322065_i128];
RET.0 = 13170280515504887253984766346710748114_u128 as i16;
Goto(bb1)
}
bb1 = {
_3.2 = RET.2;
_3.3 = !RET.3;
_7.fld1 = '\u{74210}';
Goto(bb2)
}
bb2 = {
_5.2 = [(-65389080221831300133282378463373738522_i128),90267083149777426355802383378986938558_i128,(-102858234429605570827295066465348729167_i128),118004933528384168385001330143943482884_i128,15227609612991658627950862707365269940_i128,(-66953404304180657483683195273912431733_i128),86843409084710048851645629093174070029_i128,(-4970845331237971066998942189617012649_i128)];
_6 = RET.3;
_5.0 = RET.0 ^ _9.0;
_5.2 = [444354701232555924581312884262224897_i128,21718611958757267926386434772006865296_i128,(-135740953843706036398914233476657623054_i128),99995287671510226199143498125156160139_i128,149253278416339316271425902140860476379_i128,154341262322289161838527229283657489738_i128,10137855146724191571112153854451306504_i128,(-106562619826980924546799160791208185658_i128)];
_4 = RET.2;
_5.1 = [(-32_i8),24_i8,(-30_i8)];
_7.fld2 = 1579501493_i32 as u16;
_5.2 = [(-28263034908962563951156082259331112117_i128),60224174143829612544634417293856688095_i128,(-99959217249218910804997181261964828500_i128),114817137698444904169534196432958638577_i128,(-132466364252868493798527423458671637807_i128),163737072415926632758180122382100374675_i128,61385428253170167140195073292718427857_i128,(-1467900930113040899592724009785002958_i128)];
_8 = [44886155619220873618462530376159532896_i128,43708886678911303011969378222105579303_i128,123330910569237307578826958225516147378_i128,68136509503480439018275139721392890094_i128,143337773419856141619980012321144828062_i128,(-149842561864878228538665092318386600316_i128),131575491521864903356802050844782299626_i128,54649574561816623307151229552133061785_i128];
_15 = 13451878557075744708_u64;
_8 = _4;
_5.1 = RET.1;
RET.3 = !_9.3;
RET.2 = [(-166953822357127793294840585565985931035_i128),68579845521834320495423015473542782761_i128,(-40332476826463761391351601434387489080_i128),26672202440446651454830425152601944767_i128,96965584953149088535696131499539968684_i128,(-115670569288368422537795443596642444421_i128),(-155001030094558496808872582338992348087_i128),71770505346073499484393306882145659266_i128];
_17.0.0.fld3 = 1417747008_i32 | 77892003_i32;
_9.1 = _5.1;
_12 = -(-9223372036854775808_isize);
_9.2 = RET.2;
_17.0.0.fld0 = core::ptr::addr_of!(RET.0);
Goto(bb3)
}
bb3 = {
_17.1.fld2 = _12 & _12;
RET.2 = [18982534837733428011553103524863241237_i128,(-88934514062645178742465011753485637052_i128),(-130898881424004326685290085285811548570_i128),49034460950069792862983280603764094805_i128,54946709436237417530781136623518222238_i128,105960023485445493670106161281863115087_i128,94469915372813236364278755439943040588_i128,161693987745231276472207205992277110434_i128];
_5.3 = (-156697569107833992617660337296378264911_i128) as usize;
_17.0.0.fld0 = core::ptr::addr_of!(_9.0);
_9.2 = [125990923985022913485517292757620004275_i128,59827348222669249622505447165492433342_i128,8937552797696486711331335915226845957_i128,(-43320874078417217049923155408787995567_i128),(-26257439911936974283129111529263868136_i128),112090993408757199054804743600501962787_i128,142648351632581635109796134111242212808_i128,(-75942541083184731757052410676805443463_i128)];
_16 = _1 < _7.fld3.1;
_8 = [169620201340417561618710406947534225914_i128,(-140235618640399431526125698775345383289_i128),(-163946330342847913679850277235960642519_i128),(-125103073294045769262929841685117685341_i128),162119245121031767166073128856589862488_i128,3091710836169353522131335215154536747_i128,111152153658431676930537280541907633276_i128,(-77086526923791428199452836609646230090_i128)];
_7.fld3 = (2500481302_u32, _1);
_17.1.fld3 = _17.0.0.fld3;
_19.0.2 = Adt21 { fld0: _7.fld0,fld1: _7.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
RET.0 = -_5.0;
_18 = core::ptr::addr_of_mut!(_17.0.0.fld0);
_19.0.2.fld1 = _7.fld1;
_5.2 = [(-34610113438852336191431922994425253414_i128),(-103662577263493214866926917269409373385_i128),(-167394688502059830627636668293597518567_i128),(-106461998702162169112848776672328747316_i128),97981408839083468416889656055895255893_i128,(-85949321865309109034532963097601868459_i128),(-67220788100938516014513238126516031698_i128),(-116660981617381468310712575532561688129_i128)];
_19.0.0 = core::ptr::addr_of!(_19.0.3.0);
Call(_5.3 = core::intrinsics::bswap(_3.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17.0.0.fld1 = _19.0.2.fld1;
_7.fld0 = _19.0.2.fld0;
RET.2 = [(-70691922377213572321725947565997821811_i128),9303910896713435565965787234482649778_i128,(-112013188434413243197846354437909280095_i128),50978311075803897563061375986379503732_i128,95213564361796361004454511724333079785_i128,(-108926154522260314916403933099085400835_i128),(-113426697072063484963109031509625892898_i128),(-19735849735879845084987522301497690821_i128)];
_9.1 = [3_i8,(-55_i8),(-53_i8)];
match _7.fld3.0 {
2500481302 => bb5,
_ => bb3
}
}
bb5 = {
_17.2 = &_5.0;
_15 = 4454308961590092906_u64 & 7595221743255021477_u64;
_12 = 1664738465309644706_i64 as isize;
_7.fld3.0 = _19.0.2.fld3.0;
_13 = &_19.0.2.fld3;
_9 = (_5.0, _3.1, RET.2, _3.3);
_8 = _9.2;
_17.2 = &_9.0;
_17.1.fld0 = core::ptr::addr_of!(_3.0);
_3.0 = _7.fld0 as i16;
_3.3 = _9.3;
_19.0.2 = Adt21 { fld0: _7.fld0,fld1: _17.0.0.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
_19.0.3.0 = -_5.0;
_5.1 = [(-103_i8),(-32_i8),122_i8];
_19.0.1 = &_17.2;
_9 = RET;
_19.0.2 = Adt21 { fld0: _7.fld0,fld1: _17.0.0.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
_23 = !_17.1.fld2;
_5 = (_19.0.3.0, RET.1, RET.2, _3.3);
_19.0.3 = (_3.0, RET.1, RET.2, RET.3);
_19.0.2.fld2 = _7.fld2 | _7.fld2;
RET.1 = _9.1;
_14 = [_17.1.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3,_17.1.fld3,_17.1.fld3,_17.1.fld3,_17.1.fld3];
_17.1.fld1 = _19.0.2.fld1;
match _19.0.2.fld3.0 {
0 => bb1,
2500481302 => bb6,
_ => bb4
}
}
bb6 = {
_19.0.2.fld3.0 = _7.fld3.0 / _7.fld3.0;
_23 = _12 >> _3.3;
_4 = [38477434976900409825996451503850690654_i128,(-18051864236468175922058576363015086023_i128),3362364518756101314043689952879422586_i128,(-151496749965086902704118437598722077650_i128),(-61471908588044831970740032750953324393_i128),163997268548850090643091132771230674655_i128,(-84271226333769255731190029354357018347_i128),(-36274529021174276045514645384864298895_i128)];
match _7.fld3.0 {
2500481302 => bb8,
_ => bb7
}
}
bb7 = {
_17.2 = &_5.0;
_15 = 4454308961590092906_u64 & 7595221743255021477_u64;
_12 = 1664738465309644706_i64 as isize;
_7.fld3.0 = _19.0.2.fld3.0;
_13 = &_19.0.2.fld3;
_9 = (_5.0, _3.1, RET.2, _3.3);
_8 = _9.2;
_17.2 = &_9.0;
_17.1.fld0 = core::ptr::addr_of!(_3.0);
_3.0 = _7.fld0 as i16;
_3.3 = _9.3;
_19.0.2 = Adt21 { fld0: _7.fld0,fld1: _17.0.0.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
_19.0.3.0 = -_5.0;
_5.1 = [(-103_i8),(-32_i8),122_i8];
_19.0.1 = &_17.2;
_9 = RET;
_19.0.2 = Adt21 { fld0: _7.fld0,fld1: _17.0.0.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
_23 = !_17.1.fld2;
_5 = (_19.0.3.0, RET.1, RET.2, _3.3);
_19.0.3 = (_3.0, RET.1, RET.2, RET.3);
_19.0.2.fld2 = _7.fld2 | _7.fld2;
RET.1 = _9.1;
_14 = [_17.1.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3,_17.1.fld3,_17.1.fld3,_17.1.fld3,_17.1.fld3];
_17.1.fld1 = _19.0.2.fld1;
match _19.0.2.fld3.0 {
0 => bb1,
2500481302 => bb6,
_ => bb4
}
}
bb8 = {
_26 = _16 as i8;
_25 = [_5.0,_3.0,_3.0,_19.0.3.0,RET.0,_9.0,RET.0,_5.0];
RET.3 = _5.3;
_7.fld1 = _17.0.0.fld1;
_14 = [_17.0.0.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3,_17.0.0.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3];
_8 = [25342056409669651163015567035546818395_i128,78440800777901134359200190821040641139_i128,113781295233662354128066728574784680639_i128,(-35799990916736698432464468844844955402_i128),(-85007914312025575147683171951544091968_i128),(-2982107947623656801075704489649520361_i128),62032733381815065043763652854795332890_i128,(-134418722042903553058536560019065587761_i128)];
_7.fld3.0 = !_19.0.2.fld3.0;
_17.1.fld0 = core::ptr::addr_of!(_5.0);
_7.fld0 = _19.0.2.fld0;
_5.1 = [_26,_26,_26];
_17.0 = (Move(_17.1),);
_17.1.fld0 = core::ptr::addr_of!(_19.0.3.0);
_24 = Adt58::Variant0 { fld0: Move(_17.0.0) };
RET.3 = _26 as usize;
_37 = _23 | Field::<Adt38>(Variant(_24, 0), 0).fld2;
_37 = _23;
_27 = [_23,_23,_23,_37,_37];
_17.0 = (Move(Field::<Adt38>(Variant(_24, 0), 0)),);
_7.fld4 = _19.0.2.fld2 as u8;
Call(_17.1.fld2 = fn15(Move(_19.0), _11, _37, RET, RET.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19.0.3.2 = [(-126282871377002419937793084222602684773_i128),1445417497508087591094392742056860969_i128,101127906072651853936043158072259512456_i128,(-113983575373654937005796406747887303858_i128),(-117796029114750495160170487126509960520_i128),(-21452683380245596343048666727448642636_i128),(-157172207823007197221256111108113240702_i128),(-18109694205745351297761986998200783177_i128)];
Call(_22 = fn17(_37, _5.1, RET, _27, _9, Move(_17.0), _8, RET.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5.2 = _8;
_17.2 = &_3.0;
place!(Field::<Adt38>(Variant(_24, 0), 0)) = Adt38 { fld0: Move(_17.1.fld0),fld1: _7.fld1,fld2: _37,fld3: 239237968_i32 };
_13 = &_19.0.2.fld3;
_17.0.0.fld0 = Move(Field::<Adt38>(Variant(_24, 0), 0).fld0);
_17.0.0.fld3 = Field::<Adt38>(Variant(_24, 0), 0).fld3 * Field::<Adt38>(Variant(_24, 0), 0).fld3;
_8 = _19.0.3.2;
_2 = _6 * _5.3;
_5.1 = [_26,_26,_26];
_17.3 = !Field::<Adt38>(Variant(_24, 0), 0).fld2;
_10 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4];
_19.0.3.3 = _7.fld0 as usize;
_20 = !_16;
Goto(bb11)
}
bb11 = {
_19.0.3.0 = !_9.0;
(*_18) = core::ptr::addr_of!(_19.0.3.0);
_17.2 = &_9.0;
_31 = _26;
_3 = (_9.0, RET.1, _4, _2);
_19.0.2.fld2 = _7.fld2 & _7.fld2;
_32 = [_16,_16,_16];
_17.0.0.fld2 = _19.0.2.fld2 as isize;
RET = _9;
_37 = -_23;
_25 = [_5.0,RET.0,RET.0,_3.0,_9.0,_5.0,_3.0,_5.0];
_29 = [_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
_7.fld3.1 = -_1;
_17.2 = &RET.0;
RET.0 = _9.0 & _3.0;
_5 = (_3.0, _3.1, _11, RET.3);
_39.0.0.fld0 = Move((*_18));
match Field::<Adt38>(Variant(_24, 0), 0).fld3 {
0 => bb12,
1 => bb13,
239237968 => bb15,
_ => bb14
}
}
bb12 = {
_17.0.0.fld1 = _19.0.2.fld1;
_7.fld0 = _19.0.2.fld0;
RET.2 = [(-70691922377213572321725947565997821811_i128),9303910896713435565965787234482649778_i128,(-112013188434413243197846354437909280095_i128),50978311075803897563061375986379503732_i128,95213564361796361004454511724333079785_i128,(-108926154522260314916403933099085400835_i128),(-113426697072063484963109031509625892898_i128),(-19735849735879845084987522301497690821_i128)];
_9.1 = [3_i8,(-55_i8),(-53_i8)];
match _7.fld3.0 {
2500481302 => bb5,
_ => bb3
}
}
bb13 = {
_3.2 = RET.2;
_3.3 = !RET.3;
_7.fld1 = '\u{74210}';
Goto(bb2)
}
bb14 = {
_26 = _16 as i8;
_25 = [_5.0,_3.0,_3.0,_19.0.3.0,RET.0,_9.0,RET.0,_5.0];
RET.3 = _5.3;
_7.fld1 = _17.0.0.fld1;
_14 = [_17.0.0.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3,_17.0.0.fld3,_17.0.0.fld3,_17.1.fld3,_17.0.0.fld3];
_8 = [25342056409669651163015567035546818395_i128,78440800777901134359200190821040641139_i128,113781295233662354128066728574784680639_i128,(-35799990916736698432464468844844955402_i128),(-85007914312025575147683171951544091968_i128),(-2982107947623656801075704489649520361_i128),62032733381815065043763652854795332890_i128,(-134418722042903553058536560019065587761_i128)];
_7.fld3.0 = !_19.0.2.fld3.0;
_17.1.fld0 = core::ptr::addr_of!(_5.0);
_7.fld0 = _19.0.2.fld0;
_5.1 = [_26,_26,_26];
_17.0 = (Move(_17.1),);
_17.1.fld0 = core::ptr::addr_of!(_19.0.3.0);
_24 = Adt58::Variant0 { fld0: Move(_17.0.0) };
RET.3 = _26 as usize;
_37 = _23 | Field::<Adt38>(Variant(_24, 0), 0).fld2;
_37 = _23;
_27 = [_23,_23,_23,_37,_37];
_17.0 = (Move(Field::<Adt38>(Variant(_24, 0), 0)),);
_7.fld4 = _19.0.2.fld2 as u8;
Call(_17.1.fld2 = fn15(Move(_19.0), _11, _37, RET, RET.2), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_22 = [(-5791356734657398510_i64),(-5545098760198116098_i64),(-5445074353277030032_i64)];
_8 = [158985173212855899150379902602563592765_i128,147901714901296071140910876965791113253_i128,151721305845561864683229903450187543993_i128,(-20156943760253019523285225223886657318_i128),166151077527770354343738943377172916313_i128,(-129406365409883273287813939649696410410_i128),168384094588281246758131469699954579770_i128,(-103401529813014243704702897557363644299_i128)];
_18 = core::ptr::addr_of_mut!(_17.1.fld0);
RET = (_5.0, _5.1, _5.2, _3.3);
_39.1.fld1 = Field::<Adt38>(Variant(_24, 0), 0).fld1;
place!(Field::<Adt38>(Variant(_24, 0), 0)).fld2 = -_17.0.0.fld2;
_17.0.0.fld1 = Field::<Adt38>(Variant(_24, 0), 0).fld1;
(*_18) = Move(_39.0.0.fld0);
_15 = 18260041072786299721_u64;
_39.0.0 = Adt38 { fld0: Move(_17.1.fld0),fld1: _17.0.0.fld1,fld2: _17.0.0.fld2,fld3: _17.0.0.fld3 };
_19.0.2.fld3.0 = _7.fld3.0 + _7.fld3.0;
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(14_usize, 32_usize, Move(_32), 25_usize, Move(_25), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(14_usize, 29_usize, Move(_29), 4_usize, Move(_4), 11_usize, Move(_11), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(14_usize, 2_usize, Move(_2), 23_usize, Move(_23), 8_usize, Move(_8), 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: (*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)),mut _2: [i128; 8],mut _3: isize,mut _4: (i16, [i8; 3], [i128; 8], usize),mut _5: [i128; 8]) -> isize {
mir! {
type RET = isize;
let _6: Adt47;
let _7: isize;
let _8: (*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize));
let _9: bool;
let _10: isize;
let _11: usize;
let _12: [i8; 3];
let _13: char;
let _14: (*mut u8, [i32; 4]);
let _15: isize;
let _16: i32;
let _17: usize;
let _18: [i32; 8];
let _19: isize;
let _20: (bool, isize, Adt47);
let _21: u16;
let _22: [i128; 1];
let _23: &'static Adt27;
let _24: i64;
let _25: [i8; 3];
let _26: ();
let _27: ();
{
_3 = (-9223372036854775808_isize);
_1.2.fld3.1 = _1.2.fld0 as f32;
RET = _1.2.fld0 as isize;
_1.2.fld3.0 = 1916340335_u32;
_1.2.fld3.0 = 931410306_u32;
_1.3.3 = _4.3 - _4.3;
_1.3.2 = [126553970899402808516288515363364150827_i128,166315540060143324879273805132354271030_i128,116575514497001373327008128698839810464_i128,115599344566773915402890628420915007878_i128,(-154283547145383771299265304297609237704_i128),(-88072688399199911788497770813944294365_i128),(-75244822697597166553854078930323129411_i128),(-120438133573302833613590540479644153881_i128)];
_2 = _5;
_5 = [(-43175808954369931585431017137754168619_i128),(-86743139335136451936725820042945010698_i128),55400435169367568293827072529776319898_i128,53949306235626546416184086105930685061_i128,(-31405730404571935649354465165560615177_i128),106435976626841066870887107214742971240_i128,(-155938155866098966454101403166635726497_i128),(-8016769428143221852457096461862949031_i128)];
RET = _1.2.fld3.0 as isize;
_1.2.fld3.1 = 17890241588218587882_u64 as f32;
_4.1 = _1.3.1;
_5 = _4.2;
_4.0 = _1.3.0 - _1.3.0;
_1.3.1 = [(-120_i8),(-128_i8),68_i8];
RET = _3;
_4.2 = [(-6135176859329201012805934462163054972_i128),(-44194040795763466128840058237448539534_i128),(-50458814703410213256250343816725583429_i128),135160347724501405603035022777377496254_i128,(-135420615330509808381606848672745255888_i128),(-47999128499131388115693738753945569284_i128),144472227889051164680355431230090965499_i128,(-38623271292730910216700648733449150570_i128)];
_1.0 = core::ptr::addr_of!(_4.0);
_1.3.3 = !_4.3;
_1.2.fld4 = _1.2.fld3.0 as u8;
_1.3.0 = !_4.0;
_4.2 = _2;
_4.1 = [48_i8,(-63_i8),(-49_i8)];
_1.3 = (_4.0, _4.1, _4.2, _4.3);
_1.2.fld1 = '\u{51f18}';
_1.0 = core::ptr::addr_of!(_4.0);
RET = _3 | _3;
_1.2.fld3.0 = _1.2.fld4 as u32;
_4 = _1.3;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463454151235394913435648 => bb8,
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
RET = !_3;
_5 = _4.2;
_2 = _4.2;
_1.2.fld2 = 1360_u16 & 57849_u16;
_1.3.1 = [93_i8,112_i8,(-15_i8)];
_1.2.fld1 = '\u{5e144}';
_1.2.fld3.1 = _1.2.fld0 as f32;
_3 = RET >> _1.2.fld2;
_7 = -_3;
_4.0 = _1.3.0 | _1.3.0;
RET = _4.3 as isize;
_1.0 = core::ptr::addr_of!(_4.0);
Call(_1.2.fld2 = core::intrinsics::bswap(38876_u16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = _1.2.fld1 as isize;
_8.2.fld1 = _1.2.fld1;
_8.2.fld3.0 = RET as u32;
_1.3.1 = _4.1;
_7 = _1.2.fld3.1 as isize;
RET = _7;
_8.3.2 = [141653160140994669268168025072417968936_i128,(-95546503163583029860533747457827828208_i128),61849545650338969862937226213999886820_i128,160950408752779843148052173110750384931_i128,30284736919620226821553203265041786871_i128,110515703410092426088013264410787895424_i128,(-71016490892569425662931015652425126269_i128),30981125248393352084362495169396760695_i128];
_8.3.0 = _1.2.fld4 as i16;
_8.3.3 = 283823873173032864780784734844342267149_u128 as usize;
_10 = RET * RET;
_8.3.3 = !_4.3;
_1.3.0 = 7255082446739813688_u64 as i16;
RET = _10;
_1.2.fld3.0 = _8.2.fld3.0;
_8.2.fld2 = _1.2.fld2;
_8.0 = Move(_1.0);
_8.3 = (_4.0, _4.1, _4.2, _1.3.3);
_8.2.fld0 = 6415282220624237444_i64 as f64;
_4 = (_8.3.0, _8.3.1, _2, _8.3.3);
_8.2.fld2 = _1.2.fld2 - _1.2.fld2;
_8.2.fld4 = (-336873893609622035_i64) as u8;
_8.2.fld3.1 = _4.0 as f32;
Goto(bb10)
}
bb10 = {
_8.2.fld3.1 = _1.2.fld3.1;
_4.0 = (-111_i8) as i16;
_1.2.fld0 = _8.2.fld0 * _8.2.fld0;
_1.3 = (_8.3.0, _8.3.1, _2, _8.3.3);
_1.2.fld0 = _8.2.fld0 * _8.2.fld0;
_4 = _1.3;
_8.3.1 = [53_i8,51_i8,(-81_i8)];
_8.2.fld2 = (-134428344469270831921995272373639026744_i128) as u16;
_8.2.fld4 = _1.2.fld4 >> _4.0;
_12 = [97_i8,56_i8,(-82_i8)];
_1.3 = (_8.3.0, _4.1, _4.2, _4.3);
_1.2.fld3.1 = 54381827519499592433585861345470246641_i128 as f32;
_1.3.3 = !_8.3.3;
_1.2 = Adt21 { fld0: _8.2.fld0,fld1: _8.2.fld1,fld2: _8.2.fld2,fld3: _8.2.fld3,fld4: _8.2.fld4 };
_3 = _10 >> _4.3;
_1.2.fld2 = !_8.2.fld2;
_13 = _1.2.fld1;
_1.2 = Adt21 { fld0: _8.2.fld0,fld1: _8.2.fld1,fld2: _8.2.fld2,fld3: _8.2.fld3,fld4: _8.2.fld4 };
_1.2.fld4 = _8.2.fld4;
_8.3.0 = _1.3.0 | _1.3.0;
_1.0 = Move(_8.0);
_1.2.fld3.1 = _8.2.fld3.1;
_8.2.fld0 = _1.2.fld0;
_17 = !_1.3.3;
Goto(bb11)
}
bb11 = {
_8.2.fld3.1 = (-56002803599061012179153406779715804190_i128) as f32;
Call(_8.3 = fn16(), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14.0 = core::ptr::addr_of_mut!(_1.2.fld4);
_1.2.fld3.1 = _8.2.fld3.1;
_11 = !_8.3.3;
_8.2 = Move(_1.2);
_16 = (-1501791597_i32) >> _17;
_1.2.fld2 = _8.2.fld2;
_7 = -_3;
_1.2.fld3.1 = _16 as f32;
_19 = _7 + _3;
RET = _19 - _19;
RET = _3 ^ _3;
_19 = _3;
_1.2.fld0 = -_8.2.fld0;
_22 = [(-142496092491490505476719245832721789601_i128)];
_14.1 = [_16,_16,_16,_16];
_20.1 = 2614102878057359476_i64 as isize;
_1.3 = (_4.0, _8.3.1, _5, _8.3.3);
_1.2.fld4 = _8.2.fld4 | _8.2.fld4;
_18 = [_16,_16,_16,_16,_16,_16,_16,_16];
_8.2.fld3 = (1942445683_u32, _1.2.fld3.1);
_20.1 = _10;
_4.1 = [(-8_i8),100_i8,5_i8];
_18 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb13)
}
bb13 = {
_8.3.2 = [17105659713112149551257446284499317467_i128,108417233069667668699921393721640914601_i128,103187449176608914536889759981504558884_i128,120924843060078294027592619282700040433_i128,(-113042181457824074428429194907980622634_i128),22807887469539084195719716698590103539_i128,163774087627480244722743072084106561603_i128,(-16196874986347163496949149427629148039_i128)];
_21 = !_1.2.fld2;
_4.3 = _17 * _17;
_1.2.fld2 = _16 as u16;
_8.3.3 = _4.3;
_14.1 = [_16,_16,_16,_16];
_1.2 = Adt21 { fld0: _8.2.fld0,fld1: _13,fld2: _21,fld3: _8.2.fld3,fld4: _8.2.fld4 };
RET = _7 | _7;
_1.2.fld3.0 = !_8.2.fld3.0;
_14.0 = core::ptr::addr_of_mut!(_1.2.fld4);
_8.2 = Move(_1.2);
Goto(bb14)
}
bb14 = {
_8.3.0 = 125860389671659356756054694641977603291_u128 as i16;
RET = _3;
_8.3.1 = _4.1;
_8.3.3 = 12624408225819989109_u64 as usize;
_8.2.fld1 = _13;
_25 = [7_i8,(-97_i8),(-36_i8)];
_24 = 666841862783066192_i64;
_20.0 = !true;
_13 = _8.2.fld1;
_8.2.fld0 = _16 as f64;
_1.2 = Adt21 { fld0: _8.2.fld0,fld1: _8.2.fld1,fld2: _8.2.fld2,fld3: _8.2.fld3,fld4: _8.2.fld4 };
_8.0 = Move(_1.0);
_9 = _20.0;
_17 = !_1.3.3;
_2 = [18038867694248470381325255449257687096_i128,103000911616939974861922255617401867311_i128,(-53888361559310899288957821168160705181_i128),(-167590116316957946756020773554707354208_i128),36971959166527849306854227555171705142_i128,(-33317750923147415812460467918027649121_i128),(-148217861369719586398082377689669019650_i128),(-128852457270940093099638651998773340598_i128)];
_8.3.2 = [(-14271103098468855330947854116833606632_i128),154928958069809003519972312615044952733_i128,167142899267656098657673626692657682458_i128,(-14872929635403363067384947203579603065_i128),(-4957740159820437238768553343642737019_i128),(-42030668753094624205803981192195534392_i128),(-114874553389508720843590996038890438778_i128),7640179693829091943871691025766473735_i128];
_1.2 = Adt21 { fld0: _8.2.fld0,fld1: _8.2.fld1,fld2: _21,fld3: _8.2.fld3,fld4: _8.2.fld4 };
_8.2.fld4 = !_1.2.fld4;
_15 = _16 as isize;
_19 = (-10479194502130295751783294153614621391_i128) as isize;
_9 = _20.0 ^ _20.0;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(15_usize, 17_usize, Move(_17), 4_usize, Move(_4), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(15_usize, 10_usize, Move(_10), 19_usize, Move(_19), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(15_usize, 13_usize, Move(_13), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16() -> (i16, [i8; 3], [i128; 8], usize) {
mir! {
type RET = (i16, [i8; 3], [i128; 8], usize);
let _1: Adt21;
let _2: &'static &'static [u16; 5];
let _3: (u32, f32);
let _4: isize;
let _5: char;
let _6: u16;
let _7: char;
let _8: f32;
let _9: Adt58;
let _10: u16;
let _11: f64;
let _12: [i32; 3];
let _13: i128;
let _14: usize;
let _15: *mut *mut Adt21;
let _16: (&'static Adt27,);
let _17: u128;
let _18: i16;
let _19: *const *const i16;
let _20: &'static (u32, f32);
let _21: ([i32; 4], &'static &'static i16, [i16; 8], [i16; 8]);
let _22: ();
let _23: ();
{
RET.1 = [92_i8,(-81_i8),(-49_i8)];
RET.0 = (-21618_i16) ^ (-9806_i16);
RET.3 = !6_usize;
RET.3 = 610099896_u32 as usize;
RET.2 = [(-21967527079684958199197606730157302362_i128),(-138497803043677861767663603891031324105_i128),(-86321876177197275142510955632221776338_i128),11848146358470551522524319997540970322_i128,(-167682039631563454171993876977815465907_i128),147653139506545575336381365326708885735_i128,(-56132869145924798807809053259541214180_i128),(-72020321368943677132344771086155571475_i128)];
RET.2 = [(-6316024232362509886033865362276841578_i128),83807144644211363135974598137614894563_i128,134409080765632346116489051231218664489_i128,(-43888118316219360035746455574992573658_i128),(-133237659473358282954441522647240434957_i128),95101987335543476826503350668264880422_i128,(-32242434854519522048210771590217901327_i128),85519521399850623834517256121047297594_i128];
_1.fld1 = '\u{a69d7}';
_1.fld3.0 = _1.fld1 as u32;
RET.0 = 30771_i16 & 9159_i16;
_1.fld2 = !8716_u16;
RET.2 = [(-16264196877439779408011542226460405464_i128),(-143535192878582156909574535292580162301_i128),713587227583619641038656742540244695_i128,(-132029781217525220775630893231042065368_i128),(-119405890223764720353350120593164350135_i128),(-100654650783176448545173160355766382765_i128),134625726987326995400901713346532880296_i128,(-44623863430409413105414103614508273508_i128)];
RET.3 = 1_usize;
_1.fld4 = true as u8;
RET.3 = _1.fld3.0 as usize;
_1.fld4 = !216_u8;
RET.0 = _1.fld3.0 as i16;
RET.3 = 5705509696082968664_usize + 4_usize;
_1.fld0 = _1.fld3.0 as f64;
_1.fld2 = 49454_u16 & 26206_u16;
Goto(bb1)
}
bb1 = {
_1.fld0 = 223925995524171180600922492639207769765_u128 as f64;
_1.fld2 = 5400_u16 & 37145_u16;
RET.2 = [151328283867317621792190755015274373719_i128,(-9174768522484527572304735357512196511_i128),(-105568876751316173736652824404556843196_i128),(-42922023078235371318311700281422346137_i128),(-139665301366814316739340022535515025015_i128),(-61795140063945030799571675330745916175_i128),(-84866782660972509370684759354229334286_i128),16174574889538435564899902333979160352_i128];
Goto(bb2)
}
bb2 = {
_3.0 = !_1.fld3.0;
_1.fld3.1 = 7714749584626096885_i64 as f32;
_1.fld1 = '\u{5ab8a}';
_1.fld4 = !215_u8;
Goto(bb3)
}
bb3 = {
_1.fld3.1 = RET.0 as f32;
_1.fld3.1 = (-4889132905434213103653877797444872037_i128) as f32;
_1.fld3.1 = _1.fld4 as f32;
_4 = RET.3 as isize;
_3.0 = _1.fld3.0;
_3.0 = _1.fld3.0 << _1.fld4;
_1.fld0 = _1.fld4 as f64;
RET.2 = [(-52422531302782203234640582305269147235_i128),42109854313448902531403927503120228527_i128,141476517827413279660696788853001330906_i128,29612350766015772087968780985581760654_i128,113893470576889662654255982331572362275_i128,116895890692957302182999646354356884519_i128,107640135360726775896459401951438301252_i128,(-122363308214177679161722251704639807621_i128)];
Goto(bb4)
}
bb4 = {
_1.fld0 = 64766322649689088988771930684700693171_i128 as f64;
RET.0 = (-11911_i16) ^ (-16641_i16);
_1.fld1 = '\u{1fc68}';
_3.1 = -_1.fld3.1;
_1.fld0 = 794280461_i32 as f64;
RET.2 = [(-124924626392374496122975541291216414701_i128),(-146366282227233387593678543561563486353_i128),(-16411198630130036853751294740119083938_i128),92612150276672895510724204796257135141_i128,160123973691991151899516053166992754697_i128,(-137587175653245781266016504937416738338_i128),91532377492633631136252317026434687805_i128,170000280984812000605777691885272786173_i128];
RET.1 = [(-26_i8),106_i8,40_i8];
RET.0 = (-12678_i16) - 21148_i16;
_3.0 = _1.fld3.0;
_3.0 = _1.fld3.0;
_1.fld0 = RET.0 as f64;
RET.3 = 6_usize;
_1.fld3.1 = _3.1;
_1.fld1 = '\u{21dfa}';
RET.0 = (-2504_i16);
RET.0 = (-24069_i16) ^ (-13434_i16);
_5 = _1.fld1;
_8 = _3.1;
_3.1 = _8 * _8;
RET.1 = [98_i8,107_i8,(-36_i8)];
_5 = _1.fld1;
_7 = _1.fld1;
_1.fld2 = 43839_u16 * 27040_u16;
match RET.3 {
0 => bb5,
6 => bb7,
_ => bb6
}
}
bb5 = {
_1.fld3.1 = RET.0 as f32;
_1.fld3.1 = (-4889132905434213103653877797444872037_i128) as f32;
_1.fld3.1 = _1.fld4 as f32;
_4 = RET.3 as isize;
_3.0 = _1.fld3.0;
_3.0 = _1.fld3.0 << _1.fld4;
_1.fld0 = _1.fld4 as f64;
RET.2 = [(-52422531302782203234640582305269147235_i128),42109854313448902531403927503120228527_i128,141476517827413279660696788853001330906_i128,29612350766015772087968780985581760654_i128,113893470576889662654255982331572362275_i128,116895890692957302182999646354356884519_i128,107640135360726775896459401951438301252_i128,(-122363308214177679161722251704639807621_i128)];
Goto(bb4)
}
bb6 = {
_3.0 = !_1.fld3.0;
_1.fld3.1 = 7714749584626096885_i64 as f32;
_1.fld1 = '\u{5ab8a}';
_1.fld4 = !215_u8;
Goto(bb3)
}
bb7 = {
RET.1 = [(-67_i8),46_i8,(-91_i8)];
_4 = 15761366703634616002_u64 as isize;
match RET.3 {
0 => bb8,
1 => bb9,
2 => bb10,
6 => bb12,
_ => bb11
}
}
bb8 = {
_3.0 = !_1.fld3.0;
_1.fld3.1 = 7714749584626096885_i64 as f32;
_1.fld1 = '\u{5ab8a}';
_1.fld4 = !215_u8;
Goto(bb3)
}
bb9 = {
_1.fld3.1 = RET.0 as f32;
_1.fld3.1 = (-4889132905434213103653877797444872037_i128) as f32;
_1.fld3.1 = _1.fld4 as f32;
_4 = RET.3 as isize;
_3.0 = _1.fld3.0;
_3.0 = _1.fld3.0 << _1.fld4;
_1.fld0 = _1.fld4 as f64;
RET.2 = [(-52422531302782203234640582305269147235_i128),42109854313448902531403927503120228527_i128,141476517827413279660696788853001330906_i128,29612350766015772087968780985581760654_i128,113893470576889662654255982331572362275_i128,116895890692957302182999646354356884519_i128,107640135360726775896459401951438301252_i128,(-122363308214177679161722251704639807621_i128)];
Goto(bb4)
}
bb10 = {
_1.fld0 = 64766322649689088988771930684700693171_i128 as f64;
RET.0 = (-11911_i16) ^ (-16641_i16);
_1.fld1 = '\u{1fc68}';
_3.1 = -_1.fld3.1;
_1.fld0 = 794280461_i32 as f64;
RET.2 = [(-124924626392374496122975541291216414701_i128),(-146366282227233387593678543561563486353_i128),(-16411198630130036853751294740119083938_i128),92612150276672895510724204796257135141_i128,160123973691991151899516053166992754697_i128,(-137587175653245781266016504937416738338_i128),91532377492633631136252317026434687805_i128,170000280984812000605777691885272786173_i128];
RET.1 = [(-26_i8),106_i8,40_i8];
RET.0 = (-12678_i16) - 21148_i16;
_3.0 = _1.fld3.0;
_3.0 = _1.fld3.0;
_1.fld0 = RET.0 as f64;
RET.3 = 6_usize;
_1.fld3.1 = _3.1;
_1.fld1 = '\u{21dfa}';
RET.0 = (-2504_i16);
RET.0 = (-24069_i16) ^ (-13434_i16);
_5 = _1.fld1;
_8 = _3.1;
_3.1 = _8 * _8;
RET.1 = [98_i8,107_i8,(-36_i8)];
_5 = _1.fld1;
_7 = _1.fld1;
_1.fld2 = 43839_u16 * 27040_u16;
match RET.3 {
0 => bb5,
6 => bb7,
_ => bb6
}
}
bb11 = {
_1.fld0 = 223925995524171180600922492639207769765_u128 as f64;
_1.fld2 = 5400_u16 & 37145_u16;
RET.2 = [151328283867317621792190755015274373719_i128,(-9174768522484527572304735357512196511_i128),(-105568876751316173736652824404556843196_i128),(-42922023078235371318311700281422346137_i128),(-139665301366814316739340022535515025015_i128),(-61795140063945030799571675330745916175_i128),(-84866782660972509370684759354229334286_i128),16174574889538435564899902333979160352_i128];
Goto(bb2)
}
bb12 = {
_3.1 = -_8;
RET.3 = 420956095831684105_usize;
RET.1 = [12_i8,(-70_i8),78_i8];
_12 = [1375444746_i32,1940993373_i32,871265930_i32];
_1.fld3.0 = _3.0;
_13 = -68679535937045799886651402771554151146_i128;
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_1.fld3.1 = _8 - _3.1;
_1.fld2 = (-1285935182_i32) as u16;
RET.0 = (-31086_i16);
_11 = _1.fld0;
RET.1 = [122_i8,113_i8,(-46_i8)];
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
RET.0 = 29251_i16;
_1.fld2 = 42918_u16 + 39180_u16;
_1.fld4 = 195_u8 * 171_u8;
RET.1 = [(-106_i8),113_i8,(-95_i8)];
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
RET.1 = [(-113_i8),28_i8,(-93_i8)];
_13 = (-130332027873301128146570877703326035171_i128) & 143805554205053214634118522528537729199_i128;
Goto(bb13)
}
bb13 = {
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_7 = _1.fld1;
_14 = !RET.3;
_3 = (_1.fld3.0, _1.fld3.1);
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = _7 as usize;
_5 = _7;
_10 = _1.fld2;
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = !RET.3;
_12 = [(-1449292693_i32),2050029645_i32,1355923591_i32];
_1 = Adt21 { fld0: _11,fld1: _7,fld2: _10,fld3: _3,fld4: 108_u8 };
_14 = RET.3;
Goto(bb14)
}
bb14 = {
_1.fld3.0 = _3.0 - _3.0;
RET.3 = !_14;
_11 = 17816045390894484269_u64 as f64;
_1.fld3.1 = -_8;
_14 = RET.3 << _1.fld2;
_6 = (-927399690_i32) as u16;
_1.fld3.1 = _3.1;
_3 = (_1.fld3.0, _1.fld3.1);
_1.fld2 = _10;
_17 = 83454764410878284130869303857391984202_u128;
_17 = 211978631889128621715483166427983450427_u128;
RET.2 = [_13,_13,_13,_13,_13,_13,_13,_13];
_18 = -RET.0;
_3 = (_1.fld3.0, _8);
_17 = 73378202017782176633467864299007237815_u128 + 69584165780966334724857908869558543911_u128;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(16_usize, 14_usize, Move(_14), 5_usize, Move(_5), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(16_usize, 12_usize, Move(_12), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: [i8; 3],mut _3: (i16, [i8; 3], [i128; 8], usize),mut _4: [isize; 5],mut _5: (i16, [i8; 3], [i128; 8], usize),mut _6: (Adt38,),mut _7: [i128; 8],mut _8: usize) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _9: Adt41;
let _10: &'static &'static (u32, f32);
let _11: f32;
let _12: ((*const i16, &'static &'static i16, Adt21, (i16, [i8; 3], [i128; 8], usize)), *const [i32; 4], *const [i32; 4]);
let _13: u32;
let _14: *mut Adt21;
let _15: (&'static Adt27, [i32; 4], Adt47);
let _16: u128;
let _17: i16;
let _18: i16;
let _19: &'static Adt48;
let _20: ();
let _21: ();
{
Goto(bb1)
}
bb1 = {
_7 = [(-72349464255045601729435006938261518830_i128),55031472683945493771818390932210492649_i128,(-112805369097921460287690539602610972660_i128),130498654947019091072055603347293791383_i128,(-31904278338813319010165257169065326771_i128),68239037753255064320176395606275088582_i128,152154477695543301793618042054129514051_i128,131101193629891729687556061391260530509_i128];
_1 = (-35_i8) as isize;
_3.3 = !_5.3;
_3.2 = _5.2;
_12.0.3.3 = !_5.3;
RET = [5438812105394378165_i64,5764007155104998617_i64,(-5710845726965250519_i64)];
_12.0.2.fld4 = !24_u8;
_12.0.2.fld2 = 31297_u16;
_12.0.2.fld1 = _6.0.fld1;
_12.0.3.2 = [27898443818108100034214992958582286130_i128,(-164503525669684726583110266071254977431_i128),(-44593308841465943870679396242884082430_i128),105696959163151595629098900043164500375_i128,(-101073928005799330142037429288467981193_i128),141116908198079877100174322902676577614_i128,(-125785218511075757593528938601329534177_i128),(-97830796330183667727353831423389364283_i128)];
_6.0.fld3 = (-1073350720_i32);
RET = [(-7497164970299556560_i64),3170170552706857246_i64,(-2628230460492091640_i64)];
_5.2 = [(-155476930291439031385774335580707629705_i128),150814536924313984134697613622827080591_i128,(-33949586124302497643929052721868534236_i128),48122512038062426154295603623148036204_i128,146890075599796957906366925879998241767_i128,(-66286895030130639972866392226775062569_i128),(-78518354030360079562914425830814737681_i128),(-100673548093851072191944135047449527003_i128)];
_8 = !_5.3;
_4 = [_1,_1,_6.0.fld2,_6.0.fld2,_6.0.fld2];
_12.0.0 = Move(_6.0.fld0);
_11 = _12.0.2.fld2 as f32;
_12.0.3.1 = [78_i8,48_i8,61_i8];
_6.0.fld0 = core::ptr::addr_of!(_3.0);
_12.0.0 = Move(_6.0.fld0);
_5.2 = _7;
match _6.0.fld3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607430694860736 => bb8,
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
_3 = (_5.0, _12.0.3.1, _12.0.3.2, _8);
_12.0.3.0 = _5.0 | _5.0;
_8 = !_5.3;
_12.0.2.fld0 = 2296301257861530967_u64 as f64;
_12.0.2.fld3 = (700964990_u32, _11);
_8 = _12.0.3.3;
_12.0.2.fld1 = _6.0.fld1;
_12.0.2.fld3.0 = !2850024097_u32;
_6.0 = Adt38 { fld0: Move(_12.0.0),fld1: _12.0.2.fld1,fld2: _1,fld3: (-2048229616_i32) };
_12.0.2.fld3 = (459979305_u32, _11);
_13 = _12.0.2.fld3.0 ^ _12.0.2.fld3.0;
_3.3 = _8 << _6.0.fld3;
_12.0.3.3 = _3.3;
_12.0.3.3 = !_3.3;
_12.0.3.2 = _5.2;
_6.0.fld2 = _1 >> _12.0.3.0;
Goto(bb9)
}
bb9 = {
_3 = _12.0.3;
_12.0.3.0 = _5.0 ^ _3.0;
_12.0.2.fld3.1 = _11 - _11;
_6.0.fld0 = core::ptr::addr_of!(_3.0);
_12.0.2.fld1 = _6.0.fld1;
_12.0.3.2 = [(-155111971959721349726451777370260365008_i128),(-53521482246547328339087487499743331528_i128),17657434369094721896406305778649398261_i128,115488016859632086537854528521410692527_i128,89653492772852180655924701495242967614_i128,(-130746381782491546292092318103293555250_i128),(-50392594091064536364582532275303457374_i128),(-127949227667177308396987585036922894389_i128)];
_3.3 = !_12.0.3.3;
_4 = [_6.0.fld2,_6.0.fld2,_6.0.fld2,_1,_6.0.fld2];
match _6.0.fld3 {
0 => bb1,
1 => bb10,
340282366920938463463374607429719981840 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_7 = [(-72349464255045601729435006938261518830_i128),55031472683945493771818390932210492649_i128,(-112805369097921460287690539602610972660_i128),130498654947019091072055603347293791383_i128,(-31904278338813319010165257169065326771_i128),68239037753255064320176395606275088582_i128,152154477695543301793618042054129514051_i128,131101193629891729687556061391260530509_i128];
_1 = (-35_i8) as isize;
_3.3 = !_5.3;
_3.2 = _5.2;
_12.0.3.3 = !_5.3;
RET = [5438812105394378165_i64,5764007155104998617_i64,(-5710845726965250519_i64)];
_12.0.2.fld4 = !24_u8;
_12.0.2.fld2 = 31297_u16;
_12.0.2.fld1 = _6.0.fld1;
_12.0.3.2 = [27898443818108100034214992958582286130_i128,(-164503525669684726583110266071254977431_i128),(-44593308841465943870679396242884082430_i128),105696959163151595629098900043164500375_i128,(-101073928005799330142037429288467981193_i128),141116908198079877100174322902676577614_i128,(-125785218511075757593528938601329534177_i128),(-97830796330183667727353831423389364283_i128)];
_6.0.fld3 = (-1073350720_i32);
RET = [(-7497164970299556560_i64),3170170552706857246_i64,(-2628230460492091640_i64)];
_5.2 = [(-155476930291439031385774335580707629705_i128),150814536924313984134697613622827080591_i128,(-33949586124302497643929052721868534236_i128),48122512038062426154295603623148036204_i128,146890075599796957906366925879998241767_i128,(-66286895030130639972866392226775062569_i128),(-78518354030360079562914425830814737681_i128),(-100673548093851072191944135047449527003_i128)];
_8 = !_5.3;
_4 = [_1,_1,_6.0.fld2,_6.0.fld2,_6.0.fld2];
_12.0.0 = Move(_6.0.fld0);
_11 = _12.0.2.fld2 as f32;
_12.0.3.1 = [78_i8,48_i8,61_i8];
_6.0.fld0 = core::ptr::addr_of!(_3.0);
_12.0.0 = Move(_6.0.fld0);
_5.2 = _7;
match _6.0.fld3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607430694860736 => bb8,
_ => bb7
}
}
bb12 = {
_12.0.2.fld2 = 57288_u16 ^ 63587_u16;
_1 = _6.0.fld2 << _6.0.fld2;
_11 = _12.0.2.fld3.1;
match _6.0.fld3 {
0 => bb13,
340282366920938463463374607429719981840 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3.0 = _12.0.2.fld1 as i16;
_6.0.fld2 = _1;
_16 = _12.0.2.fld4 as u128;
_8 = !_5.3;
_2 = [114_i8,(-108_i8),(-23_i8)];
_12.0.2.fld4 = !179_u8;
_3.3 = _8 >> _12.0.3.0;
_14 = core::ptr::addr_of_mut!(_12.0.2);
_5.0 = _12.0.3.0;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(17_usize, 2_usize, Move(_2), 8_usize, Move(_8), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u32,mut _2: u32,mut _3: u32) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _4: [u32; 5];
let _5: bool;
let _6: (u32, f32);
let _7: i128;
let _8: isize;
let _9: ((Adt38,), (u32, f32), u128);
let _10: isize;
let _11: [i32; 8];
let _12: f32;
let _13: [i128; 8];
let _14: bool;
let _15: bool;
let _16: [i16; 8];
let _17: i8;
let _18: [i64; 3];
let _19: (bool, isize, Adt47);
let _20: ((u32, i8), u32, [u8; 4], [bool; 3]);
let _21: [i16; 8];
let _22: bool;
let _23: ();
let _24: ();
{
RET = [2022197120799831373691749802954309151_i128,7652454642904384870335975018076083302_i128,20178988400438490259039808585939297924_i128,134321412898009906991535754611596484582_i128,121062048264890779416260557802237614025_i128,140823384951102130427159814398611589174_i128,117604772355578193252212248889907415186_i128,(-109615366976823294785298853688527446278_i128)];
_1 = 252_u8 as u32;
_3 = _1;
RET = [86679284358539105959848304283039615786_i128,41569965801788275717035396120772664044_i128,10977505832468305168158291723209377183_i128,(-102901648539252550455309614952890486267_i128),554253174522424426591086105395443684_i128,(-21094495870028886705322543056239156488_i128),48861056796517733159810417660426979431_i128,1563676850719626428679159400499319584_i128];
_3 = _2 - _1;
_4 = [_2,_3,_1,_3,_3];
_3 = !_1;
RET = [95262825536513786586104788838966301754_i128,82699849300034392004226603059648761496_i128,106762369352422308614353331312941208963_i128,(-108789380270070741355690527904858197633_i128),(-18429839804918569169190475796982056592_i128),139770122719930474423007894962814204459_i128,161992887071776041509817748256564994746_i128,48453929243628177488781533088418598277_i128];
_2 = 8150_u16 as u32;
_4 = [_2,_2,_2,_3,_3];
_3 = !_1;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
RET = [(-54795112278662751859307893359572610801_i128),(-94417558383487657348946763296399706578_i128),145217237764395838654159982564373573493_i128,70181028149154042007986722759861538236_i128,107923161151228513677878697570210271309_i128,111240939268690208286830236825556074410_i128,(-63542083234451539786513977039693653053_i128),(-148604758887382979149113520325428024470_i128)];
Goto(bb3)
}
bb3 = {
_4 = [_3,_3,_1,_3,_3];
RET = [156140664567390250731089113429764539872_i128,(-10587851163259816534756521048283355553_i128),(-144720346787913249793194687795825400680_i128),(-50817907801608425009320712305416832059_i128),105231987254903601039683886998313048057_i128,(-148588719906926767602022215490292890695_i128),(-124716117205934028240210360615932351253_i128),(-35322782742696770070842968308044693771_i128)];
RET = [(-4528384821074525585335540447773135621_i128),34257980702893550976370571154428339313_i128,110649157012567889347121702947109788806_i128,(-93922352241265352946513191453330771906_i128),124175667494809998329750304534093043856_i128,94888517409304618551306164108504876087_i128,86860049456695034259565940337029563014_i128,(-98723694758363341154792727528742213465_i128)];
_5 = _3 != _1;
_1 = (-160866520396275911963534092365395238216_i128) as u32;
_4 = [_1,_1,_3,_3,_1];
_3 = _2 - _1;
_1 = !_2;
_3 = !_2;
_3 = !_2;
RET = [(-94906417321564233359616241342556827222_i128),111806981728925711367041048596997796347_i128,(-9830409401791036704537121372210467749_i128),107291257768183561443802939697001494404_i128,(-72789131457649539348037084772226009664_i128),(-66033198565246941913208993350898331923_i128),56996366143869066761373090193033725849_i128,107230437273803864954832110022123773244_i128];
RET = [(-46882041360083455843394686989060083743_i128),(-35914876802495534000709219664009758334_i128),(-120755650209780028420093489873811125320_i128),91659455291472884663323983950066815891_i128,(-149800293535575730974835562002003349879_i128),103503252310633688971835614658124500560_i128,(-66981634435314763869293057295376318850_i128),(-100508634035485652853286867461099546443_i128)];
_3 = _1 - _1;
_3 = !_2;
_6.0 = _3;
RET = [84393310602362874824511843145810802240_i128,136788288220522911812047017643080137861_i128,(-1840574265088499295961219510468386175_i128),91991405903784853559620401959472058023_i128,(-11855818072320184629428413725018377800_i128),54700966963920568023849768123741680763_i128,(-120080484046499322319783876970843141642_i128),(-17954979268115405009967197764089641748_i128)];
_5 = !true;
_6.1 = 8910922231873771439_u64 as f32;
_5 = _6.0 >= _2;
_8 = (-9223372036854775808_isize);
RET = [(-41979654845493438851356568530909268218_i128),(-11739313060559397013801100827668244911_i128),73203274775034705828232149127080692965_i128,166543743862203775576646437385078590933_i128,(-151443417543444326137116770839024160270_i128),117238452574076008225501154849012681712_i128,(-55375760046437655040882585304347806946_i128),(-22657123965279508730337526728299869362_i128)];
_2 = _3;
_5 = true;
_7 = (-97503518010739263260170698823211760588_i128) - (-16695645856084960441094029947839904841_i128);
_2 = !_3;
_4 = [_1,_6.0,_2,_1,_2];
_4 = [_2,_6.0,_2,_1,_6.0];
_5 = false;
_1 = 36836064691778744793630393762763227108_u128 as u32;
Goto(bb4)
}
bb4 = {
_1 = _2 + _3;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb5 = {
_4 = [_3,_3,_1,_3,_3];
RET = [156140664567390250731089113429764539872_i128,(-10587851163259816534756521048283355553_i128),(-144720346787913249793194687795825400680_i128),(-50817907801608425009320712305416832059_i128),105231987254903601039683886998313048057_i128,(-148588719906926767602022215490292890695_i128),(-124716117205934028240210360615932351253_i128),(-35322782742696770070842968308044693771_i128)];
RET = [(-4528384821074525585335540447773135621_i128),34257980702893550976370571154428339313_i128,110649157012567889347121702947109788806_i128,(-93922352241265352946513191453330771906_i128),124175667494809998329750304534093043856_i128,94888517409304618551306164108504876087_i128,86860049456695034259565940337029563014_i128,(-98723694758363341154792727528742213465_i128)];
_5 = _3 != _1;
_1 = (-160866520396275911963534092365395238216_i128) as u32;
_4 = [_1,_1,_3,_3,_1];
_3 = _2 - _1;
_1 = !_2;
_3 = !_2;
_3 = !_2;
RET = [(-94906417321564233359616241342556827222_i128),111806981728925711367041048596997796347_i128,(-9830409401791036704537121372210467749_i128),107291257768183561443802939697001494404_i128,(-72789131457649539348037084772226009664_i128),(-66033198565246941913208993350898331923_i128),56996366143869066761373090193033725849_i128,107230437273803864954832110022123773244_i128];
RET = [(-46882041360083455843394686989060083743_i128),(-35914876802495534000709219664009758334_i128),(-120755650209780028420093489873811125320_i128),91659455291472884663323983950066815891_i128,(-149800293535575730974835562002003349879_i128),103503252310633688971835614658124500560_i128,(-66981634435314763869293057295376318850_i128),(-100508634035485652853286867461099546443_i128)];
_3 = _1 - _1;
_3 = !_2;
_6.0 = _3;
RET = [84393310602362874824511843145810802240_i128,136788288220522911812047017643080137861_i128,(-1840574265088499295961219510468386175_i128),91991405903784853559620401959472058023_i128,(-11855818072320184629428413725018377800_i128),54700966963920568023849768123741680763_i128,(-120080484046499322319783876970843141642_i128),(-17954979268115405009967197764089641748_i128)];
_5 = !true;
_6.1 = 8910922231873771439_u64 as f32;
_5 = _6.0 >= _2;
_8 = (-9223372036854775808_isize);
RET = [(-41979654845493438851356568530909268218_i128),(-11739313060559397013801100827668244911_i128),73203274775034705828232149127080692965_i128,166543743862203775576646437385078590933_i128,(-151443417543444326137116770839024160270_i128),117238452574076008225501154849012681712_i128,(-55375760046437655040882585304347806946_i128),(-22657123965279508730337526728299869362_i128)];
_2 = _3;
_5 = true;
_7 = (-97503518010739263260170698823211760588_i128) - (-16695645856084960441094029947839904841_i128);
_2 = !_3;
_4 = [_1,_6.0,_2,_1,_2];
_4 = [_2,_6.0,_2,_1,_6.0];
_5 = false;
_1 = 36836064691778744793630393762763227108_u128 as u32;
Goto(bb4)
}
bb6 = {
RET = [(-54795112278662751859307893359572610801_i128),(-94417558383487657348946763296399706578_i128),145217237764395838654159982564373573493_i128,70181028149154042007986722759861538236_i128,107923161151228513677878697570210271309_i128,111240939268690208286830236825556074410_i128,(-63542083234451539786513977039693653053_i128),(-148604758887382979149113520325428024470_i128)];
Goto(bb3)
}
bb7 = {
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_9.0.0.fld3 = 843115176_i32 & 1940261103_i32;
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_9.0.0.fld2 = '\u{7bc46}' as isize;
_9.1 = (_2, _6.1);
_8 = _9.0.0.fld2 - _9.0.0.fld2;
_9.0.0.fld1 = '\u{23aff}';
_5 = !false;
_2 = _1;
_6.0 = !_2;
_6.1 = 11005280018822182230_usize as f32;
_2 = _1 >> _6.0;
_8 = _1 as isize;
_11 = [_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3,_9.0.0.fld3];
_8 = 264005915225719012938974830128113555096_u128 as isize;
_2 = !_6.0;
_3 = _6.0 >> _2;
_4 = [_1,_3,_6.0,_2,_1];
_6.1 = _9.1.1;
Goto(bb11)
}
bb11 = {
_10 = 203673769063323681441756171395144911077_u128 as isize;
_6.1 = -_9.1.1;
_10 = _5 as isize;
_3 = _9.1.0;
_12 = _9.0.0.fld3 as f32;
_13 = [_7,_7,_7,_7,_7,_7,_7,_7];
_5 = _12 > _12;
_6.0 = !_3;
_5 = false;
RET = _13;
Goto(bb12)
}
bb12 = {
_14 = _2 > _2;
_9.2 = 5453620420083410189_u64 as u128;
_13 = RET;
_9.0.0.fld3 = -1573577229_i32;
_1 = _3;
_1 = !_2;
_6.0 = !_2;
_9.1.0 = _1;
_9.1.1 = -_12;
_9.1 = (_6.0, _12);
_6.0 = _2;
_9.1.1 = _12;
_10 = _8;
_18 = [2968138903338429107_i64,(-4273461551882560433_i64),(-1332074491696882539_i64)];
_9.0.0.fld2 = _10;
_19.1 = _9.0.0.fld2;
_6.0 = !_2;
_1 = _2;
_9.0.0.fld3 = 72357594_i32 + 1439326006_i32;
_6.0 = _9.1.0;
Goto(bb13)
}
bb13 = {
_10 = !_9.0.0.fld2;
_9.0.0.fld3 = 3380403224127639539_usize as i32;
_16 = [(-18308_i16),(-22490_i16),(-18222_i16),26565_i16,13753_i16,19113_i16,(-23301_i16),(-31425_i16)];
_20.2 = [9_u8,37_u8,232_u8,241_u8];
_1 = !_9.1.0;
_16 = [(-14495_i16),(-27072_i16),(-5130_i16),(-29508_i16),(-11850_i16),(-28639_i16),19184_i16,15486_i16];
Goto(bb14)
}
bb14 = {
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_6 = (_9.1.0, _9.1.1);
_17 = (-32_i8) + (-51_i8);
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(18_usize, 10_usize, Move(_10), 1_usize, Move(_1), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(18_usize, 7_usize, Move(_7), 18_usize, Move(_18), 4_usize, Move(_4), 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [i32; 8],mut _2: [i32; 8]) -> Adt48 {
mir! {
type RET = Adt48;
let _3: [bool; 3];
let _4: i8;
let _5: [u8; 4];
let _6: isize;
let _7: (bool, isize, Adt47);
let _8: (f64,);
let _9: [i128; 8];
let _10: (u32, i8);
let _11: isize;
let _12: [i64; 3];
let _13: u32;
let _14: &'static (u32, f32);
let _15: char;
let _16: bool;
let _17: char;
let _18: ();
let _19: ();
{
_1 = _2;
_1 = [1152036407_i32,1672467732_i32,2049110814_i32,1519686565_i32,(-38604936_i32),2108622256_i32,(-273318815_i32),(-500993543_i32)];
RET = Adt48::Variant3 { fld0: false,fld1: '\u{bf3f4}' };
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{e5ba0}';
place!(Field::<bool>(Variant(RET, 3), 0)) = false;
Goto(bb1)
}
bb1 = {
_1 = _2;
place!(Field::<bool>(Variant(RET, 3), 0)) = false;
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{18cac}';
SetDiscriminant(RET, 1);
_3 = [false,true,true];
place!(Field::<Adt47>(Variant(RET, 1), 2)) = Adt47::Variant1 { fld0: 966706875_u32 };
place!(Field::<u128>(Variant(RET, 1), 1)) = 1629808799_i32 as u128;
place!(Field::<u8>(Variant(RET, 1), 4)) = 103_u8;
place!(Field::<usize>(Variant(RET, 1), 0)) = 1_usize;
_6 = '\u{391bf}' as isize;
place!(Field::<u128>(Variant(RET, 1), 1)) = !216989973343657949716524639726044109728_u128;
_6 = (-74_isize);
place!(Field::<[i128; 8]>(Variant(RET, 1), 5)) = [33807775347047265809526684490560919465_i128,139890052296028692807359912259270567206_i128,(-43955053808632414775435373176247002320_i128),(-116661093954491563501529586727116716288_i128),66623171741090490913267068971236693650_i128,(-33340323628172775836808860721526844177_i128),(-170062233594384025160847427803959605519_i128),35346668129850409427815985202872585600_i128];
place!(Field::<u32>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 1), 0)) = 65288660_u32 - 3255540944_u32;
SetDiscriminant(Field::<Adt47>(Variant(RET, 1), 2), 0);
place!(Field::<usize>(Variant(RET, 1), 0)) = 14753390524153938804_usize;
place!(Field::<u128>(Variant(RET, 1), 1)) = 99943719606262715720573094390341177437_u128 + 198087320262192343886878510510494473051_u128;
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{3036c}';
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{f577d}';
_1 = [2111625708_i32,394788341_i32,(-795191978_i32),1355622250_i32,(-2118025703_i32),254857166_i32,854956458_i32,1800125401_i32];
_3 = [true,true,true];
place!(Field::<u8>(Variant(RET, 1), 4)) = 193_u8 * 195_u8;
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{2c2bb}';
place!(Field::<i64>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 2)) = !(-3026483421861729501_i64);
_7.2 = Adt47::Variant1 { fld0: 2910249305_u32 };
_7.1 = _6;
place!(Field::<usize>(Variant(RET, 1), 0)) = !0_usize;
Goto(bb2)
}
bb2 = {
place!(Field::<[u8; 4]>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 0)) = [Field::<u8>(Variant(RET, 1), 4),Field::<u8>(Variant(RET, 1), 4),Field::<u8>(Variant(RET, 1), 4),Field::<u8>(Variant(RET, 1), 4)];
place!(Field::<i64>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 2)) = true as i64;
_1 = [(-448434128_i32),(-1996300238_i32),(-187518460_i32),1792283523_i32,(-634107410_i32),(-572126238_i32),992225584_i32,(-428949415_i32)];
_5 = Field::<[u8; 4]>(Variant(Field::<Adt47>(Variant(RET, 1), 2), 0), 0);
_7.1 = 45_i8 as isize;
place!(Field::<u32>(Variant(_7.2, 1), 0)) = Field::<char>(Variant(Field::<Adt47>(Variant(RET, 1), 2), 0), 1) as u32;
_4 = -(-112_i8);
place!(Field::<Adt47>(Variant(RET, 1), 2)) = Move(_7.2);
_1 = [1645948548_i32,(-764656238_i32),219655030_i32,(-258244536_i32),(-1787568668_i32),(-1419790791_i32),(-1627902539_i32),(-58127898_i32)];
place!(Field::<Adt47>(Variant(RET, 1), 2)) = Adt47::Variant1 { fld0: 198526191_u32 };
place!(Field::<Adt47>(Variant(RET, 1), 2)) = Adt47::Variant1 { fld0: 2755220267_u32 };
_1 = [833914027_i32,(-106309804_i32),1576283330_i32,(-1991798097_i32),1486540632_i32,(-1432927645_i32),1416488007_i32,1513795247_i32];
place!(Field::<usize>(Variant(RET, 1), 0)) = (-42421477133506862985526437258862321116_i128) as usize;
RET = Adt48::Variant3 { fld0: false,fld1: '\u{5777}' };
_5 = [22_u8,203_u8,188_u8,109_u8];
_5 = [240_u8,25_u8,58_u8,215_u8];
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{92380}';
place!(Field::<bool>(Variant(RET, 3), 0)) = false;
_9 = [(-23466851640348368284196628290769733322_i128),(-38897513962943437507765963258961266979_i128),31955532053992535218613528291086169504_i128,122427552257554975432586462904994846465_i128,37306201352133869727044715412889596023_i128,(-82976552582698905249321726950210682316_i128),109972575224056332992866383248629242737_i128,(-119996438472500624187172372072782922986_i128)];
_10.0 = 3407323593_u32;
SetDiscriminant(RET, 1);
_7.2 = Adt47::Variant1 { fld0: _10.0 };
_3 = [false,true,true];
_8.0 = 3315368257563198631_u64 as f64;
place!(Field::<[i128; 8]>(Variant(RET, 1), 5)) = _9;
RET = Adt48::Variant3 { fld0: true,fld1: '\u{5d08f}' };
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768211382 => bb11,
_ => bb10
}
}
bb3 = {
_1 = _2;
place!(Field::<bool>(Variant(RET, 3), 0)) = false;
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{18cac}';
SetDiscriminant(RET, 1);
_3 = [false,true,true];
place!(Field::<Adt47>(Variant(RET, 1), 2)) = Adt47::Variant1 { fld0: 966706875_u32 };
place!(Field::<u128>(Variant(RET, 1), 1)) = 1629808799_i32 as u128;
place!(Field::<u8>(Variant(RET, 1), 4)) = 103_u8;
place!(Field::<usize>(Variant(RET, 1), 0)) = 1_usize;
_6 = '\u{391bf}' as isize;
place!(Field::<u128>(Variant(RET, 1), 1)) = !216989973343657949716524639726044109728_u128;
_6 = (-74_isize);
place!(Field::<[i128; 8]>(Variant(RET, 1), 5)) = [33807775347047265809526684490560919465_i128,139890052296028692807359912259270567206_i128,(-43955053808632414775435373176247002320_i128),(-116661093954491563501529586727116716288_i128),66623171741090490913267068971236693650_i128,(-33340323628172775836808860721526844177_i128),(-170062233594384025160847427803959605519_i128),35346668129850409427815985202872585600_i128];
place!(Field::<u32>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 1), 0)) = 65288660_u32 - 3255540944_u32;
SetDiscriminant(Field::<Adt47>(Variant(RET, 1), 2), 0);
place!(Field::<usize>(Variant(RET, 1), 0)) = 14753390524153938804_usize;
place!(Field::<u128>(Variant(RET, 1), 1)) = 99943719606262715720573094390341177437_u128 + 198087320262192343886878510510494473051_u128;
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{3036c}';
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{f577d}';
_1 = [2111625708_i32,394788341_i32,(-795191978_i32),1355622250_i32,(-2118025703_i32),254857166_i32,854956458_i32,1800125401_i32];
_3 = [true,true,true];
place!(Field::<u8>(Variant(RET, 1), 4)) = 193_u8 * 195_u8;
place!(Field::<char>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 1)) = '\u{2c2bb}';
place!(Field::<i64>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 2)), 0), 2)) = !(-3026483421861729501_i64);
_7.2 = Adt47::Variant1 { fld0: 2910249305_u32 };
_7.1 = _6;
place!(Field::<usize>(Variant(RET, 1), 0)) = !0_usize;
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
Return()
}
bb11 = {
_10.1 = _4 & _4;
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{6d977}';
_7.0 = false;
_6 = !_7.1;
_5 = [157_u8,1_u8,3_u8,141_u8];
place!(Field::<bool>(Variant(RET, 3), 0)) = _7.0;
_3 = [Field::<bool>(Variant(RET, 3), 0),_7.0,Field::<bool>(Variant(RET, 3), 0)];
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{eed5c}';
place!(Field::<u32>(Variant(_7.2, 1), 0)) = _10.0 & _10.0;
RET = Adt48::Variant3 { fld0: _7.0,fld1: '\u{75887}' };
Goto(bb12)
}
bb12 = {
_10.0 = !Field::<u32>(Variant(_7.2, 1), 0);
place!(Field::<bool>(Variant(RET, 3), 0)) = !_7.0;
RET = Adt48::Variant3 { fld0: _7.0,fld1: '\u{70108}' };
Goto(bb13)
}
bb13 = {
_11 = _6;
_8.0 = 49334846_i32 as f64;
place!(Field::<bool>(Variant(RET, 3), 0)) = _7.0;
RET = Adt48::Variant3 { fld0: _7.0,fld1: '\u{8f662}' };
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{37172}';
_13 = _8.0 as u32;
place!(Field::<bool>(Variant(RET, 3), 0)) = _7.0;
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{af5db}';
_6 = !_11;
_5 = [38_u8,252_u8,148_u8,32_u8];
_13 = !_10.0;
SetDiscriminant(_7.2, 2);
_16 = Field::<bool>(Variant(RET, 3), 0) < _7.0;
_14 = &place!(Field::<(u32, f32)>(Variant(_7.2, 2), 3));
place!(Field::<(u32, f32)>(Variant(_7.2, 2), 3)).1 = 17989726355902876200_u64 as f32;
_7.0 = _16 == Field::<bool>(Variant(RET, 3), 0);
Goto(bb14)
}
bb14 = {
_12 = [4736807815128323686_i64,(-6997856129463222526_i64),9118562288138793111_i64];
place!(Field::<u64>(Variant(_7.2, 2), 1)) = 12225547421233282214_u64 << _10.0;
place!(Field::<Adt41>(Variant(_7.2, 2), 0)) = Adt41::Variant2 { fld0: Field::<(u32, f32)>(Variant(_7.2, 2), 3).1,fld1: _10.1,fld2: _6 };
place!(Field::<char>(Variant(RET, 3), 1)) = '\u{dba72}';
place!(Field::<(u32, f32)>(Variant(_7.2, 2), 3)).1 = Field::<f32>(Variant(Field::<Adt41>(Variant(_7.2, 2), 0), 2), 0);
_6 = Field::<isize>(Variant(Field::<Adt41>(Variant(_7.2, 2), 0), 2), 2) - _7.1;
place!(Field::<(u32, f32)>(Variant(_7.2, 2), 3)).1 = -Field::<f32>(Variant(Field::<Adt41>(Variant(_7.2, 2), 0), 2), 0);
place!(Field::<(u32, f32)>(Variant(_7.2, 2), 3)).1 = Field::<f32>(Variant(Field::<Adt41>(Variant(_7.2, 2), 0), 2), 0) * Field::<f32>(Variant(Field::<Adt41>(Variant(_7.2, 2), 0), 2), 0);
_15 = Field::<char>(Variant(RET, 3), 1);
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(19_usize, 15_usize, Move(_15), 16_usize, Move(_16), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(19_usize, 10_usize, Move(_10), 9_usize, Move(_9), 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(3498059718_u32), std::hint::black_box('\u{7db9a}'));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt21{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt21 {
fld0: f64,
fld1: char,
fld2: u16,
fld3: (u32, f32),
fld4: u8,
}
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: (u32, f32),
fld1: u64,
fld2: isize,
fld3: i128,
fld4: [bool; 3],

},
Variant1{
fld0: [bool; 3],

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: *const i16,
fld1: char,
fld2: isize,
fld3: i32,
}
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
fld0: Adt21,
fld1: (i16, [i8; 3], [i128; 8], usize),
fld2: *mut u128,
fld3: [i8; 3],

},
Variant1{
fld0: [i32; 4],
fld1: usize,
fld2: isize,
fld3: [u8; 4],
fld4: f64,
fld5: i32,
fld6: Adt27,
fld7: Adt38,

},
Variant2{
fld0: f32,
fld1: i8,
fld2: isize,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u8; 4],
fld1: char,
fld2: i64,

},
Variant1{
fld0: u32,

},
Variant2{
fld0: Adt41,
fld1: u64,
fld2: *const f32,
fld3: (u32, f32),

},
Variant3{
fld0: bool,
fld1: *const [i32; 4],
fld2: *const f32,
fld3: f32,
fld4: [u16; 5],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (u16,),
fld1: [u16; 5],
fld2: *const f32,
fld3: u32,
fld4: i16,
fld5: *const [i32; 4],
fld6: u64,

},
Variant1{
fld0: usize,
fld1: u128,
fld2: Adt47,
fld3: i8,
fld4: u8,
fld5: [i128; 8],
fld6: *const [i32; 4],

},
Variant2{
fld0: Adt47,
fld1: f64,
fld2: [i128; 8],
fld3: u16,
fld4: *mut u128,
fld5: i32,
fld6: i64,
fld7: Adt41,

},
Variant3{
fld0: bool,
fld1: char,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt38,

},
Variant1{
fld0: [u8; 4],
fld1: u32,
fld2: ((Adt38,), (u32, f32), u128),

},
Variant2{
fld0: (i16, [i8; 3], [i128; 8], usize),
fld1: [u16; 2],

},
Variant3{
fld0: i64,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt65{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt65 {
fld0: i8,
fld1: Adt21,
fld2: Adt48,
}

