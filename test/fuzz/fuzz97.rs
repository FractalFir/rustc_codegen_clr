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
pub fn fn0(mut _1: i64,mut _2: u32) -> isize {
mir! {
type RET = isize;
let _3: Adt36;
let _4: bool;
let _5: [u16; 3];
let _6: bool;
let _7: ((u16, i128), isize, [i32; 2], u128, i16);
let _8: Adt50;
let _9: [i16; 6];
let _10: bool;
let _11: [i8; 2];
let _12: [u64; 6];
let _13: f32;
let _14: ();
let _15: ();
{
RET = (-9223372036854775808_isize);
RET = (-50_isize) & (-32_isize);
_2 = 2289236340_u32 & 3320651849_u32;
RET = 143_u8 as isize;
_1 = 3835189212606216961_i64;
_2 = 666361597_u32 >> RET;
RET = 55118033983853640454482571100165729690_u128 as isize;
Call(RET = core::intrinsics::bswap(93_isize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 8893324052797972294_i64;
RET = -(-64_isize);
RET = 25906423106527888329075429494797936452_u128 as isize;
RET = -(-114_isize);
_1 = (-2053148136957827713_i64);
_2 = !701109470_u32;
_4 = false;
_1 = -5378806608539880769_i64;
_1 = 3684327430449018334_i64;
_4 = _2 < _2;
Goto(bb2)
}
bb2 = {
_5 = [34444_u16,45804_u16,21448_u16];
RET = 7835_u16 as isize;
Call(_7.1 = fn1(_5, _4, _5, _5, _4, RET, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = [23969_u16,47290_u16,8410_u16];
_7.2 = [(-84922265_i32),1712352187_i32];
_7.3 = !144657695747856165850079129051580351835_u128;
_7.4 = 9896_i16 ^ (-3317_i16);
_7.0.0 = 23254_u16 * 61844_u16;
_1 = -942582952446899600_i64;
_7.2 = [1894465221_i32,(-2059746696_i32)];
_7.0 = (43892_u16, (-14020458203503766176115309015489631937_i128));
_5 = [_7.0.0,_7.0.0,_7.0.0];
_7.0.1 = 134801415026515058454825828848498061635_i128 | (-152435391017268262203086133430293209668_i128);
RET = !_7.1;
_7.2 = [(-936203773_i32),(-1991863538_i32)];
_7.4 = -(-21822_i16);
_7.0.0 = !49500_u16;
_6 = _7.0.1 == _7.0.1;
RET = -_7.1;
_7.0 = (35987_u16, 121338936918384219670649817616631184716_i128);
_7.3 = 62442624520823859193191170573089358322_u128;
_7.4 = 28278_i16;
_5 = [_7.0.0,_7.0.0,_7.0.0];
_7.2 = [(-751698884_i32),1880681025_i32];
_4 = _6;
_2 = 1560086084_u32 & 2452701928_u32;
_6 = !_4;
_5 = [_7.0.0,_7.0.0,_7.0.0];
match _7.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
35987 => bb9,
_ => bb8
}
}
bb4 = {
_5 = [34444_u16,45804_u16,21448_u16];
RET = 7835_u16 as isize;
Call(_7.1 = fn1(_5, _4, _5, _5, _4, RET, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_1 = 8893324052797972294_i64;
RET = -(-64_isize);
RET = 25906423106527888329075429494797936452_u128 as isize;
RET = -(-114_isize);
_1 = (-2053148136957827713_i64);
_2 = !701109470_u32;
_4 = false;
_1 = -5378806608539880769_i64;
_1 = 3684327430449018334_i64;
_4 = _2 < _2;
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
RET = _7.1 * _7.1;
_7.1 = RET << _7.3;
_4 = _7.1 < _7.1;
match _7.0.1 {
0 => bb5,
1 => bb2,
2 => bb3,
121338936918384219670649817616631184716 => bb10,
_ => bb6
}
}
bb10 = {
RET = _7.1 - _7.1;
_7.4 = 29970_i16;
_7.1 = RET * RET;
_7.0.1 = (-86211671095817767524031697232054444901_i128);
_7.0.0 = 17630_u16 | 33173_u16;
_7.1 = RET;
RET = -_7.1;
match _7.3 {
0 => bb7,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
62442624520823859193191170573089358322 => bb15,
_ => bb14
}
}
bb11 = {
_1 = 8893324052797972294_i64;
RET = -(-64_isize);
RET = 25906423106527888329075429494797936452_u128 as isize;
RET = -(-114_isize);
_1 = (-2053148136957827713_i64);
_2 = !701109470_u32;
_4 = false;
_1 = -5378806608539880769_i64;
_1 = 3684327430449018334_i64;
_4 = _2 < _2;
Goto(bb2)
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
_9 = [_7.4,_7.4,_7.4,_7.4,_7.4,_7.4];
_6 = _4;
RET = _7.1;
_7.0.1 = 86175780962218290484178329093495247546_i128;
_7.3 = 281048958533028521268389649545985789014_u128 - 166451030742088391484049593459489117619_u128;
_1 = (-8247512374296486868_i64);
_7.4 = !(-30955_i16);
_2 = 3963274155_u32 & 2032672265_u32;
RET = 1688535100_i32 as isize;
_6 = _4;
_7.2 = [(-267907803_i32),(-1262543417_i32)];
RET = -_7.1;
RET = !_7.1;
_6 = _7.0.0 >= _7.0.0;
_7.0.1 = 61428490909595038127386282798765821246_i128;
_7.3 = 135616926051178613890542774659693046075_u128;
_7.0 = (53725_u16, (-11269689511567361883538549604200512551_i128));
_9 = [_7.4,_7.4,_7.4,_7.4,_7.4,_7.4];
_7.1 = !RET;
_4 = _2 <= _2;
Goto(bb16)
}
bb16 = {
Call(_14 = dump_var(0_usize, 9_usize, Move(_9), 4_usize, Move(_4), 1_usize, Move(_1), 15_usize, _15), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: [u16; 3],mut _2: bool,mut _3: [u16; 3],mut _4: [u16; 3],mut _5: bool,mut _6: isize,mut _7: i64,mut _8: bool) -> isize {
mir! {
type RET = isize;
let _9: [i32; 8];
let _10: [u128; 1];
let _11: u32;
let _12: [u32; 3];
let _13: f32;
let _14: i64;
let _15: ([i16; 6],);
let _16: char;
let _17: (i16, ([i16; 6],));
let _18: [i32; 8];
let _19: f64;
let _20: u16;
let _21: Adt39;
let _22: u64;
let _23: Adt42;
let _24: [u128; 1];
let _25: [u64; 6];
let _26: Adt36;
let _27: [i32; 8];
let _28: f64;
let _29: [u64; 6];
let _30: char;
let _31: [u64; 6];
let _32: [u128; 1];
let _33: *const [u64; 6];
let _34: ();
let _35: ();
{
_3 = [41998_u16,25567_u16,63983_u16];
_9 = [1685117214_i32,(-741153209_i32),1805060433_i32,1833915009_i32,(-1147982544_i32),(-1056045800_i32),1457149354_i32,1658491188_i32];
_10 = [52029954436559397423190545538713755280_u128];
_1 = _3;
_9 = [629038576_i32,(-1736966528_i32),1845919538_i32,1824853967_i32,971785448_i32,(-186754109_i32),(-379707093_i32),127203874_i32];
_10 = [191800018731651700819949063848178504418_u128];
RET = 18394370824378843595_u64 as isize;
_8 = _5;
_8 = _5;
_3 = [63126_u16,35065_u16,5544_u16];
_8 = RET >= _6;
RET = 20415_i16 as isize;
_5 = _2;
_7 = (-3768487459580030349_i64);
_8 = _2;
_9 = [(-959982997_i32),1950795707_i32,(-914293905_i32),(-981760267_i32),(-664183367_i32),(-128604110_i32),(-2036985338_i32),1443622960_i32];
_8 = _2;
_9 = [(-1778089151_i32),(-825053557_i32),1968584150_i32,(-246069124_i32),(-462527547_i32),(-552506169_i32),(-1067424673_i32),1443440017_i32];
_1 = [9564_u16,62556_u16,4121_u16];
_9 = [(-2123257813_i32),(-617221390_i32),999140789_i32,1267758851_i32,(-110110834_i32),1392237071_i32,(-2001616376_i32),(-1419589006_i32)];
match _7 {
340282366920938463459606119972188181107 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = 596068967_i32 as isize;
_9 = [1457874557_i32,(-296221114_i32),250100198_i32,(-1323307700_i32),1618049134_i32,(-523092831_i32),(-1688374813_i32),876920394_i32];
_5 = _8;
_6 = !RET;
RET = _6 >> _7;
_5 = _2;
_12 = [2757798697_u32,2947374847_u32,250845315_u32];
_3 = _1;
_7 = (-3909227477770421097_i64) & 9208169838045620348_i64;
Call(_7 = fn2(_4, _2, _9, _1, _6, _6, RET, _8, _1, _1, RET, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _2;
RET = _6 | _6;
_11 = 869465439_u32;
_2 = _8;
_13 = 10364338380393462234_u64 as f32;
_6 = RET + RET;
_2 = _8;
Goto(bb4)
}
bb4 = {
_11 = 2568187829_u32;
_6 = !RET;
_9 = [315187980_i32,(-876972143_i32),5798093_i32,(-213097726_i32),(-543298824_i32),1283898768_i32,464483806_i32,(-263222587_i32)];
RET = !_6;
_8 = RET < RET;
_9 = [626670258_i32,(-1519632545_i32),580037331_i32,1892808305_i32,(-1134785349_i32),1388651841_i32,547677858_i32,(-995604047_i32)];
_7 = -274999848173883891_i64;
_1 = [1067_u16,64844_u16,29632_u16];
_7 = (-1526419449115369979_i64);
_8 = _2;
_2 = _6 == RET;
_1 = [5277_u16,61330_u16,36293_u16];
_14 = _7;
_7 = -_14;
_13 = 49880_u16 as f32;
_13 = 53598_u16 as f32;
_4 = _1;
_4 = [46457_u16,28564_u16,12495_u16];
_5 = !_8;
_8 = _2 ^ _2;
_3 = [538_u16,37464_u16,5224_u16];
_5 = !_2;
_2 = !_5;
_10 = [36925068868619603100993207756571890963_u128];
_6 = RET;
_15.0 = [6606_i16,(-32548_i16),(-13133_i16),9250_i16,12177_i16,(-22692_i16)];
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2568187829 => bb10,
_ => bb9
}
}
bb5 = {
_8 = _2;
RET = _6 | _6;
_11 = 869465439_u32;
_2 = _8;
_13 = 10364338380393462234_u64 as f32;
_6 = RET + RET;
_2 = _8;
Goto(bb4)
}
bb6 = {
RET = 596068967_i32 as isize;
_9 = [1457874557_i32,(-296221114_i32),250100198_i32,(-1323307700_i32),1618049134_i32,(-523092831_i32),(-1688374813_i32),876920394_i32];
_5 = _8;
_6 = !RET;
RET = _6 >> _7;
_5 = _2;
_12 = [2757798697_u32,2947374847_u32,250845315_u32];
_3 = _1;
_7 = (-3909227477770421097_i64) & 9208169838045620348_i64;
Call(_7 = fn2(_4, _2, _9, _1, _6, _6, RET, _8, _1, _1, RET, _12), ReturnTo(bb3), UnwindUnreachable())
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
_15.0 = [(-3028_i16),23381_i16,28709_i16,(-22704_i16),14585_i16,(-15121_i16)];
_15.0 = [19476_i16,12042_i16,6535_i16,16757_i16,28860_i16,(-15389_i16)];
_16 = '\u{4f3e2}';
_17.1 = (_15.0,);
_14 = _7;
_15 = (_17.1.0,);
_19 = _6 as f64;
_10 = [312758295251278183256599380194863043636_u128];
_17.0 = -31457_i16;
_2 = _8;
RET = _6;
_9 = [1021490355_i32,29986285_i32,(-1417173014_i32),(-635724600_i32),77939411_i32,400570517_i32,(-1946298907_i32),(-216954621_i32)];
_20 = !10539_u16;
_16 = '\u{5169c}';
_17.1 = (_15.0,);
_8 = _5;
_18 = _9;
_20 = !20579_u16;
_15.0 = _17.1.0;
_11 = !1206831643_u32;
_6 = RET & RET;
_15 = (_17.1.0,);
_11 = _5 as u32;
_14 = _2 as i64;
_15 = (_17.1.0,);
Goto(bb11)
}
bb11 = {
_16 = '\u{6e152}';
_6 = 81_i8 as isize;
_17 = (8806_i16, _15);
_17.1.0 = _15.0;
_2 = _5;
_17.0 = !29264_i16;
_25 = [9937268246952303720_u64,6402657291037855491_u64,4066564007493641549_u64,13311306937120717470_u64,9500476349727044085_u64,16451420684835423295_u64];
_24 = _10;
_12 = [_11,_11,_11];
_17.1 = (_15.0,);
_22 = 12605610094428008953244870488591732438_u128 as u64;
_11 = 2365975615_u32 - 3961211710_u32;
Call(_18 = fn19(_5, _15, _17.1.0, _8, _17.1.0, _16, _17.1, _15, _25, _2, _24, _25, _9, _14, _17, _14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15 = (_17.1.0,);
_27 = [(-440918743_i32),(-573782078_i32),(-1340184574_i32),(-1539324157_i32),150417664_i32,2937591_i32,1083998191_i32,139416309_i32];
_24 = _10;
_28 = -_19;
_17.0 = _19 as i16;
_9 = [(-2139573980_i32),(-1744280632_i32),(-1352048314_i32),1336958302_i32,244074878_i32,306620293_i32,1641222422_i32,(-1242387642_i32)];
_25 = [_22,_22,_22,_22,_22,_22];
_26 = Adt36::Variant0 { fld0: _28 };
_27 = [(-1230070096_i32),1390652990_i32,1049570131_i32,767480547_i32,(-1350315829_i32),1325675655_i32,(-1560280450_i32),(-416871145_i32)];
_7 = -_14;
_16 = '\u{863e2}';
RET = -_6;
_1 = _4;
_14 = -_7;
_14 = -_7;
_4 = _3;
_22 = 16459639731325537576_u64 + 13337294802499386358_u64;
SetDiscriminant(_26, 3);
_29 = [_22,_22,_22,_22,_22,_22];
RET = _6;
_14 = _7 ^ _7;
_1 = [_20,_20,_20];
Goto(bb13)
}
bb13 = {
_17 = ((-13080_i16), _15);
_17.1.0 = _15.0;
_5 = !_8;
_31 = [_22,_22,_22,_22,_22,_22];
_9 = [948902561_i32,1431018221_i32,204153112_i32,(-1835315812_i32),(-1684150773_i32),530578580_i32,(-325965174_i32),(-2042865777_i32)];
_17.1 = (_15.0,);
_22 = 3357115528987368934_u64;
place!(Field::<*mut char>(Variant(_26, 3), 1)) = core::ptr::addr_of_mut!(_30);
match _17.0 {
0 => bb12,
1 => bb5,
2 => bb3,
3 => bb14,
4 => bb15,
340282366920938463463374607431768198376 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
RET = 596068967_i32 as isize;
_9 = [1457874557_i32,(-296221114_i32),250100198_i32,(-1323307700_i32),1618049134_i32,(-523092831_i32),(-1688374813_i32),876920394_i32];
_5 = _8;
_6 = !RET;
RET = _6 >> _7;
_5 = _2;
_12 = [2757798697_u32,2947374847_u32,250845315_u32];
_3 = _1;
_7 = (-3909227477770421097_i64) & 9208169838045620348_i64;
Call(_7 = fn2(_4, _2, _9, _1, _6, _6, RET, _8, _1, _1, RET, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_15.0 = [(-3028_i16),23381_i16,28709_i16,(-22704_i16),14585_i16,(-15121_i16)];
_15.0 = [19476_i16,12042_i16,6535_i16,16757_i16,28860_i16,(-15389_i16)];
_16 = '\u{4f3e2}';
_17.1 = (_15.0,);
_14 = _7;
_15 = (_17.1.0,);
_19 = _6 as f64;
_10 = [312758295251278183256599380194863043636_u128];
_17.0 = -31457_i16;
_2 = _8;
RET = _6;
_9 = [1021490355_i32,29986285_i32,(-1417173014_i32),(-635724600_i32),77939411_i32,400570517_i32,(-1946298907_i32),(-216954621_i32)];
_20 = !10539_u16;
_16 = '\u{5169c}';
_17.1 = (_15.0,);
_8 = _5;
_18 = _9;
_20 = !20579_u16;
_15.0 = _17.1.0;
_11 = !1206831643_u32;
_6 = RET & RET;
_15 = (_17.1.0,);
_11 = _5 as u32;
_14 = _2 as i64;
_15 = (_17.1.0,);
Goto(bb11)
}
bb17 = {
_18 = [(-1203103603_i32),864990112_i32,1867648923_i32,1346462899_i32,185387791_i32,(-2010977026_i32),474691084_i32,257359768_i32];
place!(Field::<i32>(Variant(_26, 3), 0)) = 2120458303_i32;
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(1_usize, 18_usize, Move(_18), 5_usize, Move(_5), 16_usize, Move(_16), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(1_usize, 12_usize, Move(_12), 11_usize, Move(_11), 25_usize, Move(_25), 22_usize, Move(_22)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(1_usize, 6_usize, Move(_6), 31_usize, Move(_31), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u16; 3],mut _2: bool,mut _3: [i32; 8],mut _4: [u16; 3],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: bool,mut _9: [u16; 3],mut _10: [u16; 3],mut _11: isize,mut _12: [u32; 3]) -> i64 {
mir! {
type RET = i64;
let _13: u64;
let _14: isize;
let _15: isize;
let _16: u8;
let _17: u8;
let _18: [u16; 3];
let _19: Adt36;
let _20: u128;
let _21: [i8; 2];
let _22: f32;
let _23: [u128; 1];
let _24: [bool; 2];
let _25: f32;
let _26: u64;
let _27: isize;
let _28: Adt39;
let _29: [i64; 8];
let _30: i64;
let _31: [u128; 1];
let _32: isize;
let _33: char;
let _34: [i64; 8];
let _35: ();
let _36: ();
{
RET = -8799003583862187039_i64;
_6 = _5 + _7;
_6 = _11;
_6 = _11;
_5 = _7;
RET = 7095812962687630801_i64 * (-1157846877619278052_i64);
_7 = -_5;
Goto(bb1)
}
bb1 = {
_7 = 20357705599611397847783388464117843790_i128 as isize;
_11 = _5 ^ _6;
_13 = '\u{e67ea}' as u64;
Call(_14 = fn3(_6, _2, _13, _1, _9, _11, _11, _5, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _4;
_3 = [1243613159_i32,(-602036802_i32),1137001381_i32,1969609433_i32,1593444583_i32,293114669_i32,(-935750404_i32),1512842184_i32];
_9 = _1;
RET = 1438592181054060714_i64 & (-138217884111378822_i64);
_12 = [927391692_u32,694228856_u32,2938311413_u32];
_2 = _8 | _8;
_13 = !12802284187523254631_u64;
_13 = 15578173898305200460_u64;
_11 = 7132438102104018599_usize as isize;
_10 = [53718_u16,23695_u16,59756_u16];
_14 = !_7;
_10 = _1;
_13 = _2 as u64;
_12 = [2090900015_u32,347677027_u32,580802324_u32];
_5 = -_6;
_4 = [8159_u16,25989_u16,61029_u16];
_13 = 14335500882968494294_u64;
_10 = [44718_u16,40144_u16,10552_u16];
Call(_8 = fn4(RET, _9, _5, _3, _10, _3, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = 3243658095590313968_u64;
_4 = _9;
RET = (-6669709529275102644_i64);
_13 = '\u{727a1}' as u64;
_16 = _2 as u8;
_1 = _10;
_15 = 42574_u16 as isize;
_11 = -_6;
_4 = _9;
_3 = [1800855900_i32,(-1898839585_i32),(-1971864358_i32),(-1372252962_i32),(-1928456843_i32),(-1088589002_i32),558505776_i32,(-243702990_i32)];
_15 = -_7;
_12 = [1555336801_u32,1265290344_u32,3621960453_u32];
_14 = _15;
_8 = _6 == _15;
_6 = _15;
_13 = 62803_u16 as u64;
_15 = _11 << RET;
_4 = _1;
_6 = -_7;
_15 = _11;
Call(_13 = core::intrinsics::bswap(4046277489646083672_u64), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = 3799061305645336865_u64;
_15 = _7 & _11;
_15 = _16 as isize;
_10 = _9;
_16 = 177_u8;
_3 = [(-2016794576_i32),2141113604_i32,(-885263708_i32),904945879_i32,(-867240901_i32),(-972826696_i32),(-1225438478_i32),899811502_i32];
Goto(bb5)
}
bb5 = {
_5 = 156090200516888286547855079278790191117_i128 as isize;
_12 = [2507990922_u32,1297377817_u32,482240630_u32];
_6 = (-52496235662664225035337282950910794088_i128) as isize;
_17 = 127108170841404146885956468142222474167_u128 as u8;
_1 = _9;
_18 = [42323_u16,37048_u16,36003_u16];
_10 = _9;
_1 = _10;
_8 = _2;
_18 = [48844_u16,57249_u16,5008_u16];
_6 = -_11;
_3 = [(-1152944975_i32),(-183893261_i32),(-652550601_i32),1569995749_i32,82889147_i32,(-489909343_i32),256910200_i32,(-371291486_i32)];
_4 = _18;
_3 = [529717361_i32,888215273_i32,(-1470648169_i32),248365016_i32,(-1336587298_i32),1517983662_i32,753866103_i32,(-1560717684_i32)];
_2 = !_8;
_14 = '\u{4c1d}' as isize;
_10 = [46644_u16,19153_u16,40096_u16];
_20 = !164461188304397889916152484675978062756_u128;
_20 = 323922960538347894552590270088925862621_u128;
_13 = !3635195358890515654_u64;
_20 = 216977119627587514806131022913265046064_u128 - 40901104115051256189691679016718322786_u128;
_14 = '\u{3fcc2}' as isize;
_17 = _2 as u8;
_18 = [54433_u16,55297_u16,65479_u16];
_15 = 2312529130_u32 as isize;
_12 = [686630385_u32,781483262_u32,1796944839_u32];
Call(_17 = core::intrinsics::bswap(_16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_17 = !_16;
_14 = _6;
_24 = [_8,_2];
_2 = _16 > _16;
_13 = !373212831010727543_u64;
_14 = _15 ^ _5;
_14 = 36256_u16 as isize;
_25 = 1_i8 as f32;
_23 = [_20];
_18 = [8030_u16,56861_u16,16300_u16];
_17 = _8 as u8;
_21 = [(-127_i8),(-6_i8)];
_4 = [2666_u16,17159_u16,17061_u16];
_11 = _15 << _15;
_5 = -_7;
_22 = _25;
_16 = _13 as u8;
_3 = [(-247105410_i32),(-1656334501_i32),(-1093068093_i32),1879767080_i32,1402430652_i32,(-86960539_i32),1039302453_i32,(-1474680213_i32)];
_20 = RET as u128;
_12 = [3249795316_u32,2929975204_u32,2997985159_u32];
_13 = 3915143871768484423_u64;
_2 = _8;
_10 = [381_u16,22021_u16,37524_u16];
_17 = _25 as u8;
match _13 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
3915143871768484423 => bb12,
_ => bb11
}
}
bb7 = {
_5 = 156090200516888286547855079278790191117_i128 as isize;
_12 = [2507990922_u32,1297377817_u32,482240630_u32];
_6 = (-52496235662664225035337282950910794088_i128) as isize;
_17 = 127108170841404146885956468142222474167_u128 as u8;
_1 = _9;
_18 = [42323_u16,37048_u16,36003_u16];
_10 = _9;
_1 = _10;
_8 = _2;
_18 = [48844_u16,57249_u16,5008_u16];
_6 = -_11;
_3 = [(-1152944975_i32),(-183893261_i32),(-652550601_i32),1569995749_i32,82889147_i32,(-489909343_i32),256910200_i32,(-371291486_i32)];
_4 = _18;
_3 = [529717361_i32,888215273_i32,(-1470648169_i32),248365016_i32,(-1336587298_i32),1517983662_i32,753866103_i32,(-1560717684_i32)];
_2 = !_8;
_14 = '\u{4c1d}' as isize;
_10 = [46644_u16,19153_u16,40096_u16];
_20 = !164461188304397889916152484675978062756_u128;
_20 = 323922960538347894552590270088925862621_u128;
_13 = !3635195358890515654_u64;
_20 = 216977119627587514806131022913265046064_u128 - 40901104115051256189691679016718322786_u128;
_14 = '\u{3fcc2}' as isize;
_17 = _2 as u8;
_18 = [54433_u16,55297_u16,65479_u16];
_15 = 2312529130_u32 as isize;
_12 = [686630385_u32,781483262_u32,1796944839_u32];
Call(_17 = core::intrinsics::bswap(_16), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_13 = 3799061305645336865_u64;
_15 = _7 & _11;
_15 = _16 as isize;
_10 = _9;
_16 = 177_u8;
_3 = [(-2016794576_i32),2141113604_i32,(-885263708_i32),904945879_i32,(-867240901_i32),(-972826696_i32),(-1225438478_i32),899811502_i32];
Goto(bb5)
}
bb9 = {
_13 = 3243658095590313968_u64;
_4 = _9;
RET = (-6669709529275102644_i64);
_13 = '\u{727a1}' as u64;
_16 = _2 as u8;
_1 = _10;
_15 = 42574_u16 as isize;
_11 = -_6;
_4 = _9;
_3 = [1800855900_i32,(-1898839585_i32),(-1971864358_i32),(-1372252962_i32),(-1928456843_i32),(-1088589002_i32),558505776_i32,(-243702990_i32)];
_15 = -_7;
_12 = [1555336801_u32,1265290344_u32,3621960453_u32];
_14 = _15;
_8 = _6 == _15;
_6 = _15;
_13 = 62803_u16 as u64;
_15 = _11 << RET;
_4 = _1;
_6 = -_7;
_15 = _11;
Call(_13 = core::intrinsics::bswap(4046277489646083672_u64), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_9 = _4;
_3 = [1243613159_i32,(-602036802_i32),1137001381_i32,1969609433_i32,1593444583_i32,293114669_i32,(-935750404_i32),1512842184_i32];
_9 = _1;
RET = 1438592181054060714_i64 & (-138217884111378822_i64);
_12 = [927391692_u32,694228856_u32,2938311413_u32];
_2 = _8 | _8;
_13 = !12802284187523254631_u64;
_13 = 15578173898305200460_u64;
_11 = 7132438102104018599_usize as isize;
_10 = [53718_u16,23695_u16,59756_u16];
_14 = !_7;
_10 = _1;
_13 = _2 as u64;
_12 = [2090900015_u32,347677027_u32,580802324_u32];
_5 = -_6;
_4 = [8159_u16,25989_u16,61029_u16];
_13 = 14335500882968494294_u64;
_10 = [44718_u16,40144_u16,10552_u16];
Call(_8 = fn4(RET, _9, _5, _3, _10, _3, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_7 = 20357705599611397847783388464117843790_i128 as isize;
_11 = _5 ^ _6;
_13 = '\u{e67ea}' as u64;
Call(_14 = fn3(_6, _2, _13, _1, _9, _11, _11, _5, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
RET = 5817285147687165252_i64 | (-3707789874772333594_i64);
_24 = [_8,_8];
_2 = _8 & _8;
_9 = [32441_u16,35423_u16,3823_u16];
_26 = _13;
_15 = '\u{c18d9}' as isize;
_25 = _22 + _22;
_21 = [(-35_i8),(-2_i8)];
_10 = [7964_u16,49286_u16,60057_u16];
_20 = 30057333453426615874132764020385982714_u128 - 41091525009633952133491204815405080851_u128;
_9 = [39123_u16,47909_u16,11033_u16];
_8 = !_2;
_27 = 3126169539_u32 as isize;
_21 = [25_i8,(-24_i8)];
_4 = [45282_u16,18811_u16,59899_u16];
_4 = [38718_u16,39297_u16,55368_u16];
_28 = Adt39::Variant1 { fld0: _20 };
Call(_26 = core::intrinsics::bswap(_13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10 = [44467_u16,9715_u16,10309_u16];
_21 = [110_i8,(-86_i8)];
place!(Field::<u128>(Variant(_28, 1), 0)) = !_20;
_8 = _2 == _2;
_21 = [(-70_i8),(-63_i8)];
_3 = [(-1654626648_i32),100500344_i32,(-1383456430_i32),265741464_i32,1065890974_i32,724171798_i32,184369999_i32,(-223881977_i32)];
_1 = [23849_u16,50851_u16,13754_u16];
_11 = -_27;
_10 = [4130_u16,18666_u16,10939_u16];
_26 = _2 as u64;
_15 = _11;
SetDiscriminant(_28, 3);
_24 = [_2,_2];
_9 = [61747_u16,52300_u16,19436_u16];
place!(Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2)).0 = (-10053_i16) & (-81_i16);
RET = (-5107482385638552326_i64);
_18 = [35421_u16,20203_u16,35415_u16];
_8 = !_2;
place!(Field::<usize>(Variant(_28, 3), 3)) = !3_usize;
_23 = [_20];
_27 = _25 as isize;
_1 = [51294_u16,15382_u16,48803_u16];
place!(Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_28, 3), 2).0];
_33 = '\u{5795c}';
match _13 {
0 => bb2,
1 => bb14,
2 => bb15,
3 => bb16,
3915143871768484423 => bb18,
_ => bb17
}
}
bb14 = {
_5 = 156090200516888286547855079278790191117_i128 as isize;
_12 = [2507990922_u32,1297377817_u32,482240630_u32];
_6 = (-52496235662664225035337282950910794088_i128) as isize;
_17 = 127108170841404146885956468142222474167_u128 as u8;
_1 = _9;
_18 = [42323_u16,37048_u16,36003_u16];
_10 = _9;
_1 = _10;
_8 = _2;
_18 = [48844_u16,57249_u16,5008_u16];
_6 = -_11;
_3 = [(-1152944975_i32),(-183893261_i32),(-652550601_i32),1569995749_i32,82889147_i32,(-489909343_i32),256910200_i32,(-371291486_i32)];
_4 = _18;
_3 = [529717361_i32,888215273_i32,(-1470648169_i32),248365016_i32,(-1336587298_i32),1517983662_i32,753866103_i32,(-1560717684_i32)];
_2 = !_8;
_14 = '\u{4c1d}' as isize;
_10 = [46644_u16,19153_u16,40096_u16];
_20 = !164461188304397889916152484675978062756_u128;
_20 = 323922960538347894552590270088925862621_u128;
_13 = !3635195358890515654_u64;
_20 = 216977119627587514806131022913265046064_u128 - 40901104115051256189691679016718322786_u128;
_14 = '\u{3fcc2}' as isize;
_17 = _2 as u8;
_18 = [54433_u16,55297_u16,65479_u16];
_15 = 2312529130_u32 as isize;
_12 = [686630385_u32,781483262_u32,1796944839_u32];
Call(_17 = core::intrinsics::bswap(_16), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_7 = 20357705599611397847783388464117843790_i128 as isize;
_11 = _5 ^ _6;
_13 = '\u{e67ea}' as u64;
Call(_14 = fn3(_6, _2, _13, _1, _9, _11, _11, _5, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_9 = _4;
_3 = [1243613159_i32,(-602036802_i32),1137001381_i32,1969609433_i32,1593444583_i32,293114669_i32,(-935750404_i32),1512842184_i32];
_9 = _1;
RET = 1438592181054060714_i64 & (-138217884111378822_i64);
_12 = [927391692_u32,694228856_u32,2938311413_u32];
_2 = _8 | _8;
_13 = !12802284187523254631_u64;
_13 = 15578173898305200460_u64;
_11 = 7132438102104018599_usize as isize;
_10 = [53718_u16,23695_u16,59756_u16];
_14 = !_7;
_10 = _1;
_13 = _2 as u64;
_12 = [2090900015_u32,347677027_u32,580802324_u32];
_5 = -_6;
_4 = [8159_u16,25989_u16,61029_u16];
_13 = 14335500882968494294_u64;
_10 = [44718_u16,40144_u16,10552_u16];
Call(_8 = fn4(RET, _9, _5, _3, _10, _3, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_13 = 3243658095590313968_u64;
_4 = _9;
RET = (-6669709529275102644_i64);
_13 = '\u{727a1}' as u64;
_16 = _2 as u8;
_1 = _10;
_15 = 42574_u16 as isize;
_11 = -_6;
_4 = _9;
_3 = [1800855900_i32,(-1898839585_i32),(-1971864358_i32),(-1372252962_i32),(-1928456843_i32),(-1088589002_i32),558505776_i32,(-243702990_i32)];
_15 = -_7;
_12 = [1555336801_u32,1265290344_u32,3621960453_u32];
_14 = _15;
_8 = _6 == _15;
_6 = _15;
_13 = 62803_u16 as u64;
_15 = _11 << RET;
_4 = _1;
_6 = -_7;
_15 = _11;
Call(_13 = core::intrinsics::bswap(4046277489646083672_u64), ReturnTo(bb4), UnwindUnreachable())
}
bb18 = {
_2 = !_8;
_14 = Field::<usize>(Variant(_28, 3), 3) as isize;
Goto(bb19)
}
bb19 = {
Call(_35 = dump_var(2_usize, 11_usize, Move(_11), 9_usize, Move(_9), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(2_usize, 4_usize, Move(_4), 21_usize, Move(_21), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_35 = dump_var(2_usize, 13_usize, Move(_13), 20_usize, Move(_20), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: bool,mut _3: u64,mut _4: [u16; 3],mut _5: [u16; 3],mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: [u16; 3]) -> isize {
mir! {
type RET = isize;
let _11: usize;
let _12: u128;
let _13: [i16; 6];
let _14: Adt44;
let _15: f32;
let _16: i64;
let _17: ();
let _18: ();
{
_1 = _6 & _8;
RET = _7;
_9 = 9360_i16 as isize;
_5 = [2115_u16,29_u16,4756_u16];
_3 = 15697_i16 as u64;
_4 = [61093_u16,30273_u16,24945_u16];
_11 = 0_usize >> _1;
_9 = _6 >> _11;
RET = _9;
_4 = [26500_u16,55650_u16,42852_u16];
_3 = 1910253358556217382_u64;
_4 = _5;
RET = _6 - _9;
_4 = _10;
_13 = [9153_i16,(-22128_i16),(-18624_i16),18399_i16,10887_i16,(-9979_i16)];
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(3_usize, 4_usize, Move(_4), 1_usize, Move(_1), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(3_usize, 6_usize, Move(_6), 3_usize, Move(_3), 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i64,mut _2: [u16; 3],mut _3: isize,mut _4: [i32; 8],mut _5: [u16; 3],mut _6: [i32; 8],mut _7: isize) -> bool {
mir! {
type RET = bool;
let _8: *const [u64; 6];
let _9: Adt36;
let _10: [i32; 8];
let _11: i32;
let _12: u8;
let _13: char;
let _14: [i64; 8];
let _15: [i64; 8];
let _16: *mut u32;
let _17: isize;
let _18: u8;
let _19: ((u16, i128), isize, [i32; 2], u128, i16);
let _20: [u128; 1];
let _21: isize;
let _22: i64;
let _23: isize;
let _24: i8;
let _25: i16;
let _26: char;
let _27: f32;
let _28: ([i16; 6],);
let _29: (i16, ([i16; 6],));
let _30: [i64; 8];
let _31: ();
let _32: ();
{
_1 = (-89_i8) as i64;
RET = !true;
_4 = [1253176513_i32,2106744343_i32,(-2118421465_i32),(-409877689_i32),(-1082307525_i32),(-1221042182_i32),(-1591466925_i32),153525720_i32];
_6 = _4;
_1 = !4851622547419269001_i64;
_7 = _3;
_1 = 7490371223263964136_i64;
_6 = [(-1600629499_i32),(-126909165_i32),(-1400567252_i32),(-608569990_i32),720773501_i32,(-119482687_i32),(-1392310856_i32),279343902_i32];
RET = !true;
Call(_5 = fn5(_7, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = true;
_5 = [40233_u16,34508_u16,19925_u16];
_2 = [50413_u16,27469_u16,9350_u16];
RET = !false;
_10 = [(-664089929_i32),648062881_i32,(-885141470_i32),584920101_i32,1222556716_i32,2060326439_i32,(-1364539499_i32),(-1956258668_i32)];
_4 = _6;
_3 = _7;
_5 = [52163_u16,47856_u16,40792_u16];
_6 = [(-1868954345_i32),(-1363046608_i32),1287183346_i32,(-399762532_i32),(-300777399_i32),1849228456_i32,293355797_i32,465950104_i32];
_11 = (-1298295523_i32) - (-1392021819_i32);
_1 = _11 as i64;
_1 = 3435463404917744993_u64 as i64;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_1 = !(-619916420498905701_i64);
Goto(bb2)
}
bb2 = {
_11 = !(-321167185_i32);
_5 = [56866_u16,26313_u16,7850_u16];
RET = !true;
_2 = [62655_u16,50713_u16,5813_u16];
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_2 = [60171_u16,15406_u16,566_u16];
_12 = !226_u8;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = -1898044161_i32;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_1 = 8885965975964093585_i64 | 5268640722447501849_i64;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_12 = 122_u8 & 43_u8;
_3 = 65887426144912220989475935948118638254_i128 as isize;
_2 = [39580_u16,63794_u16,40717_u16];
_5 = _2;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb3)
}
bb3 = {
_6 = _10;
_6 = _4;
_7 = 338953350775204457166993854038662214038_u128 as isize;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = '\u{27a58}';
_12 = _1 as u8;
_11 = (-394721681_i32) | 2018159250_i32;
_1 = 1452200628804942359_i64 * (-7982355858525141403_i64);
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_7 = !_3;
_7 = _12 as isize;
Goto(bb4)
}
bb4 = {
RET = false;
_2 = [3088_u16,55130_u16,28413_u16];
_19.0.0 = !42447_u16;
_19.2 = [_11,_11];
_2 = _5;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_19.1 = _3 | _7;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = _4;
_19.4 = !(-7377_i16);
_19.2 = [_11,_11];
_13 = '\u{dba3b}';
Goto(bb5)
}
bb5 = {
_19.0 = (23552_u16, 9384995860115668815129526377806150369_i128);
_2 = [_19.0.0,_19.0.0,_19.0.0];
RET = true;
_19.0.0 = 41277_u16;
_7 = _19.4 as isize;
_3 = -_7;
_19.3 = _13 as u128;
_19.2 = [_11,_11];
_19.0.1 = (-136556876035065279994905318114097675024_i128) ^ (-90301420541259566866678406488497684818_i128);
_18 = _19.3 as u8;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 343295784_i32 | (-729126628_i32);
_17 = _19.1;
Goto(bb6)
}
bb6 = {
_19.0.1 = 30_i8 as i128;
_17 = _19.1 - _3;
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_13 = '\u{4c19e}';
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !false;
_7 = _19.1;
_7 = -_17;
_19.0.0 = 3891_u16;
_19.0.1 = -69065661011195281659805723791725611443_i128;
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_19.2 = [_11,_11];
_19.0.1 = (-140281179606755333317900540612634005075_i128) >> _7;
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_18 = _12;
_19.4 = (-11292_i16);
_12 = _18 ^ _18;
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_20 = [_19.3];
_23 = -_17;
match _19.4 {
0 => bb5,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607431768200164 => bb12,
_ => bb11
}
}
bb7 = {
_19.0 = (23552_u16, 9384995860115668815129526377806150369_i128);
_2 = [_19.0.0,_19.0.0,_19.0.0];
RET = true;
_19.0.0 = 41277_u16;
_7 = _19.4 as isize;
_3 = -_7;
_19.3 = _13 as u128;
_19.2 = [_11,_11];
_19.0.1 = (-136556876035065279994905318114097675024_i128) ^ (-90301420541259566866678406488497684818_i128);
_18 = _19.3 as u8;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 343295784_i32 | (-729126628_i32);
_17 = _19.1;
Goto(bb6)
}
bb8 = {
RET = false;
_2 = [3088_u16,55130_u16,28413_u16];
_19.0.0 = !42447_u16;
_19.2 = [_11,_11];
_2 = _5;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_19.1 = _3 | _7;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = _4;
_19.4 = !(-7377_i16);
_19.2 = [_11,_11];
_13 = '\u{dba3b}';
Goto(bb5)
}
bb9 = {
_6 = _10;
_6 = _4;
_7 = 338953350775204457166993854038662214038_u128 as isize;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = '\u{27a58}';
_12 = _1 as u8;
_11 = (-394721681_i32) | 2018159250_i32;
_1 = 1452200628804942359_i64 * (-7982355858525141403_i64);
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_7 = !_3;
_7 = _12 as isize;
Goto(bb4)
}
bb10 = {
_11 = !(-321167185_i32);
_5 = [56866_u16,26313_u16,7850_u16];
RET = !true;
_2 = [62655_u16,50713_u16,5813_u16];
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_2 = [60171_u16,15406_u16,566_u16];
_12 = !226_u8;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = -1898044161_i32;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_1 = 8885965975964093585_i64 | 5268640722447501849_i64;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = !true;
_12 = 122_u8 & 43_u8;
_3 = 65887426144912220989475935948118638254_i128 as isize;
_2 = [39580_u16,63794_u16,40717_u16];
_5 = _2;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb3)
}
bb11 = {
RET = true;
_5 = [40233_u16,34508_u16,19925_u16];
_2 = [50413_u16,27469_u16,9350_u16];
RET = !false;
_10 = [(-664089929_i32),648062881_i32,(-885141470_i32),584920101_i32,1222556716_i32,2060326439_i32,(-1364539499_i32),(-1956258668_i32)];
_4 = _6;
_3 = _7;
_5 = [52163_u16,47856_u16,40792_u16];
_6 = [(-1868954345_i32),(-1363046608_i32),1287183346_i32,(-399762532_i32),(-300777399_i32),1849228456_i32,293355797_i32,465950104_i32];
_11 = (-1298295523_i32) - (-1392021819_i32);
_1 = _11 as i64;
_1 = 3435463404917744993_u64 as i64;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_1 = !(-619916420498905701_i64);
Goto(bb2)
}
bb12 = {
_26 = _13;
_19.2 = [_11,_11];
_23 = -_3;
_19.0.1 = (-9625282806072138524158583891876634458_i128);
_19.4 = (-3086_i16) ^ (-20615_i16);
_19.0.0 = !26914_u16;
RET = false;
_24 = -91_i8;
_25 = _19.4;
match _19.0.1 {
0 => bb13,
330657084114866324939216023539891576998 => bb15,
_ => bb14
}
}
bb13 = {
RET = false;
_2 = [3088_u16,55130_u16,28413_u16];
_19.0.0 = !42447_u16;
_19.2 = [_11,_11];
_2 = _5;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_19.1 = _3 | _7;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = _4;
_19.4 = !(-7377_i16);
_19.2 = [_11,_11];
_13 = '\u{dba3b}';
Goto(bb5)
}
bb14 = {
_19.0 = (23552_u16, 9384995860115668815129526377806150369_i128);
_2 = [_19.0.0,_19.0.0,_19.0.0];
RET = true;
_19.0.0 = 41277_u16;
_7 = _19.4 as isize;
_3 = -_7;
_19.3 = _13 as u128;
_19.2 = [_11,_11];
_19.0.1 = (-136556876035065279994905318114097675024_i128) ^ (-90301420541259566866678406488497684818_i128);
_18 = _19.3 as u8;
_6 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 343295784_i32 | (-729126628_i32);
_17 = _19.1;
Goto(bb6)
}
bb15 = {
_21 = -_3;
_25 = 3274184830_u32 as i16;
_25 = _19.4;
_3 = _12 as isize;
_19.0.0 = !60616_u16;
_7 = _19.0.0 as isize;
_18 = _17 as u8;
_19.3 = _18 as u128;
_10 = [_11,_11,_11,_11,_11,_11,_11,_11];
_20 = [_19.3];
_24 = (-49_i8) >> _17;
_23 = !_19.1;
_7 = _19.0.0 as isize;
_19.2 = [_11,_11];
_27 = _24 as f32;
_5 = _2;
RET = true;
_17 = _3;
_13 = _26;
_28.0 = [_19.4,_19.4,_19.4,_25,_25,_19.4];
_12 = _18;
_19.0.1 = (-141278878038712030865785669387399763875_i128) ^ 161696766152536693097175953582066064447_i128;
_6 = _10;
_30 = _15;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(4_usize, 24_usize, Move(_24), 25_usize, Move(_25), 15_usize, Move(_15), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(4_usize, 30_usize, Move(_30), 26_usize, Move(_26), 28_usize, Move(_28), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(4_usize, 5_usize, Move(_5), 11_usize, Move(_11), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: [i32; 8],mut _3: [i32; 8]) -> [u16; 3] {
mir! {
type RET = [u16; 3];
let _4: bool;
let _5: i8;
let _6: [i32; 6];
let _7: bool;
let _8: Adt48;
let _9: *mut char;
let _10: isize;
let _11: f64;
let _12: isize;
let _13: u64;
let _14: *mut u32;
let _15: [i64; 8];
let _16: [i32; 8];
let _17: Adt51;
let _18: u128;
let _19: u32;
let _20: Adt48;
let _21: Adt47;
let _22: Adt47;
let _23: i128;
let _24: f32;
let _25: [bool; 2];
let _26: Adt44;
let _27: u64;
let _28: &'static u32;
let _29: f32;
let _30: [u16; 3];
let _31: [u128; 1];
let _32: [i8; 2];
let _33: [u32; 3];
let _34: Adt46;
let _35: char;
let _36: bool;
let _37: [i32; 6];
let _38: char;
let _39: isize;
let _40: [u64; 6];
let _41: i64;
let _42: bool;
let _43: [u16; 3];
let _44: [bool; 2];
let _45: u8;
let _46: i64;
let _47: Adt39;
let _48: [i32; 2];
let _49: [i8; 2];
let _50: i128;
let _51: ();
let _52: ();
{
_2 = [(-719322541_i32),817889061_i32,1762579901_i32,326753497_i32,991652614_i32,(-1829915085_i32),677670480_i32,275162894_i32];
_3 = _2;
RET = [17083_u16,34017_u16,26599_u16];
_5 = !(-69_i8);
_2 = [(-1302335517_i32),221648593_i32,392401299_i32,714159012_i32,1762798600_i32,489414963_i32,(-551756463_i32),(-831937320_i32)];
_4 = true & false;
RET = [9921_u16,46808_u16,42359_u16];
_2 = [580936266_i32,865893942_i32,(-715418875_i32),1719682158_i32,492654798_i32,(-459291350_i32),1044710895_i32,(-994472668_i32)];
RET = [9149_u16,23986_u16,46397_u16];
_7 = _5 != _5;
_4 = _7;
_5 = 13_i8 << _1;
_6 = [(-661039734_i32),(-419885123_i32),(-486004032_i32),625733809_i32,786460002_i32,(-1060840794_i32)];
_1 = 9223372036854775807_isize - (-9223372036854775808_isize);
_7 = _4 ^ _4;
Goto(bb1)
}
bb1 = {
_1 = 9223372036854775807_isize * 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_5 = 63_i8 ^ 55_i8;
_7 = !_4;
_2 = _3;
_4 = _7;
_6 = [(-920510753_i32),1473114218_i32,1152142849_i32,1250291925_i32,1984086222_i32,(-1268492488_i32)];
_6 = [(-957456691_i32),1889774121_i32,(-1537774925_i32),(-452433305_i32),1339717816_i32,(-1224670217_i32)];
_8 = Adt48::Variant2 { fld0: _6 };
place!(Field::<[i32; 6]>(Variant(_8, 2), 0)) = _6;
SetDiscriminant(_8, 3);
place!(Field::<bool>(Variant(_8, 3), 0)) = _4 > _4;
place!(Field::<[u64; 6]>(Variant(_8, 3), 4)) = [5994505194484205098_u64,4652748143252787925_u64,2674518813425678660_u64,17380305009222076070_u64,1082391985884424083_u64,1684292507517655654_u64];
place!(Field::<i32>(Variant(_8, 3), 5)) = 631538226_i32;
_2 = _3;
Goto(bb2)
}
bb2 = {
place!(Field::<(i16, ([i16; 6],))>(Variant(_8, 3), 1)).1.0 = [30791_i16,(-12977_i16),20579_i16,1929_i16,15775_i16,(-4473_i16)];
place!(Field::<i8>(Variant(_8, 3), 3)) = _5;
_8 = Adt48::Variant2 { fld0: _6 };
_3 = _2;
_10 = _1 >> _5;
RET = [9289_u16,23521_u16,4947_u16];
SetDiscriminant(_8, 0);
_6 = [(-1926915709_i32),2012437839_i32,670959461_i32,(-936853403_i32),(-1525969649_i32),(-989373306_i32)];
Call(place!(Field::<u8>(Variant(_8, 0), 1)) = core::intrinsics::bswap(180_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [59629_u16,36038_u16,14902_u16];
_3 = _2;
_8 = Adt48::Variant2 { fld0: _6 };
_5 = 3940224425016287457_u64 as i8;
_1 = 63361_u16 as isize;
_4 = _7;
_2 = [(-995439997_i32),(-1169114307_i32),(-1884368339_i32),1675074070_i32,1933107868_i32,(-440322321_i32),(-873506445_i32),469183755_i32];
_2 = _3;
_2 = [(-2098131105_i32),753711140_i32,60733105_i32,(-1749670283_i32),(-506985021_i32),1599653755_i32,(-1248071955_i32),(-724080512_i32)];
place!(Field::<[i32; 6]>(Variant(_8, 2), 0)) = _6;
_1 = 397_u16 as isize;
_5 = 31_i8;
_5 = 97_i8 - (-44_i8);
Call(_12 = fn6(Move(_8), _10, _2, _6, _4, _3, _6, _3, _5, RET, _10, _2, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = [10353_u16,25532_u16,52019_u16];
_11 = 59941_u16 as f64;
_7 = _12 <= _12;
_11 = 57290599793814605593467044865573911610_i128 as f64;
_6 = [(-374756173_i32),(-1390029333_i32),(-1225828455_i32),(-1861079317_i32),(-786202889_i32),(-1488939103_i32)];
_1 = !_12;
_13 = 2609_u16 as u64;
_5 = 70_i8;
RET = [44222_u16,51721_u16,34649_u16];
_5 = (-1_i8) << _1;
_2 = [(-302080125_i32),126074439_i32,274628156_i32,1190862452_i32,(-1799773491_i32),(-341870107_i32),(-12681988_i32),(-566874434_i32)];
_3 = _2;
_1 = _12 - _12;
_15 = [2873228024363127087_i64,903373004064950381_i64,(-2218674217127479140_i64),(-6608977237837928061_i64),(-6038221862826407754_i64),(-5917882667944107973_i64),33404891685340518_i64,(-7945745347628655369_i64)];
_6 = [1441630477_i32,880072760_i32,417716384_i32,(-1311019442_i32),606963903_i32,2141129225_i32];
_2 = [818744855_i32,(-378289472_i32),1609109967_i32,1438431395_i32,1064996194_i32,(-1050277912_i32),(-859085537_i32),(-2070371891_i32)];
_10 = -_1;
RET = [9893_u16,29371_u16,4536_u16];
_7 = _1 == _1;
_1 = _12 - _12;
_6 = [92517313_i32,(-1703254692_i32),(-886342308_i32),(-1232325222_i32),(-232594484_i32),1188985553_i32];
_3 = [(-1021176253_i32),(-289325851_i32),(-1508976754_i32),1944336659_i32,(-1392983779_i32),651242050_i32,(-1096752968_i32),1457707231_i32];
_11 = 36492_u16 as f64;
_11 = _1 as f64;
_5 = _7 as i8;
_16 = _2;
Call(_18 = fn9(_10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = _3;
_2 = [(-1286865775_i32),949198417_i32,313825651_i32,(-1313427786_i32),1559445098_i32,1991309500_i32,930529486_i32,(-739066125_i32)];
_14 = core::ptr::addr_of_mut!(_19);
_19 = 3614441105_u32 ^ 2016660171_u32;
_6 = [302171890_i32,650600376_i32,1663304121_i32,1925065947_i32,(-933296821_i32),(-302196499_i32)];
_15 = [6571557036496792613_i64,(-8994536335021630253_i64),(-3604179580069952892_i64),(-5715527241291639442_i64),2329480952074939021_i64,8953605304115871159_i64,(-4655326357421821898_i64),977269963317023395_i64];
_6 = [(-1654887897_i32),(-2036238780_i32),1368083641_i32,230416920_i32,66685868_i32,838604789_i32];
RET = [29780_u16,39151_u16,12657_u16];
RET = [59064_u16,35857_u16,39145_u16];
_3 = [1704905302_i32,1973563150_i32,(-429764163_i32),(-1477462696_i32),1162059116_i32,(-299949940_i32),267048004_i32,807270808_i32];
_6 = [1459431426_i32,132078535_i32,1608001377_i32,562287404_i32,1855965946_i32,(-106478562_i32)];
_10 = _1 - _1;
_18 = !193553183967732975563216629204620120550_u128;
Call((*_14) = core::intrinsics::bswap(3169127883_u32), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = _10;
_16 = _2;
_11 = 4680_u16 as f64;
_10 = _1 | _1;
Goto(bb7)
}
bb7 = {
_3 = [1777382955_i32,(-624703838_i32),(-930886708_i32),1107955451_i32,1967237740_i32,(-1802769790_i32),1914570806_i32,1610104606_i32];
_1 = -_10;
_2 = _16;
_6 = [1852798684_i32,(-1907355646_i32),673747867_i32,196805171_i32,544562053_i32,1742602370_i32];
_20 = Adt48::Variant2 { fld0: _6 };
SetDiscriminant(_20, 2);
_13 = 220059321050731128_u64;
_7 = _4 ^ _4;
RET = [6888_u16,59936_u16,21456_u16];
place!(Field::<[i32; 6]>(Variant(_20, 2), 0)) = [(-938157379_i32),(-1871447254_i32),1739196947_i32,(-1435678044_i32),(-1800357001_i32),(-393496583_i32)];
_18 = !82805235726957028024621618345525614328_u128;
_25 = [_7,_7];
_12 = _10 | _1;
_16 = _2;
_19 = 3038020959_u32 & 4235094609_u32;
_16 = _3;
_11 = (-20358_i16) as f64;
_23 = -1213452266910502484042676529793452161_i128;
SetDiscriminant(_20, 1);
place!(Field::<u8>(Variant(_20, 1), 6)) = _18 as u8;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).3 = 33781_u16 as u128;
_28 = &_19;
_26 = Adt44::Variant0 { fld0: _4,fld1: _23 };
Goto(bb8)
}
bb8 = {
place!(Field::<u16>(Variant(_20, 1), 5)) = 63829_u16 - 55629_u16;
SetDiscriminant(_26, 2);
_29 = 2260566495389869043_i64 as f32;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).3 = (-4056140643464252453_i64) as u128;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).4 = (-22214_i16);
place!(Field::<u32>(Variant(_26, 2), 6)) = (*_14) - (*_28);
_10 = _23 as isize;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = [1682236563_i32,(-1466612663_i32),(-246265866_i32),(-1506176271_i32),(-904030497_i32),2136038871_i32,707660890_i32,610819712_i32];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0 = (Field::<u16>(Variant(_20, 1), 5), _23);
place!(Field::<[i32; 2]>(Variant(_26, 2), 2)) = [815988591_i32,114681908_i32];
place!(Field::<i128>(Variant(_26, 2), 5)) = _23;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).2 = Field::<[i32; 2]>(Variant(_26, 2), 2);
_19 = !Field::<u32>(Variant(_26, 2), 6);
_11 = 2046128630_i32 as f64;
_23 = -Field::<i128>(Variant(_26, 2), 5);
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).0 = (Field::<u16>(Variant(_20, 1), 5), Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.1);
_14 = core::ptr::addr_of_mut!((*_14));
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0.1 = !_23;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)) = (Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0, _12, Field::<[i32; 2]>(Variant(_26, 2), 2), Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).3, (-15824_i16));
match Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
340282366920938463463374607431768195632 => bb9,
_ => bb5
}
}
bb9 = {
(*_14) = !Field::<u32>(Variant(_26, 2), 6);
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).0.1 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4 as i128;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = Field::<u8>(Variant(_20, 1), 6) as i16;
_1 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
_32 = [_5,_5];
_12 = !_1;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = [1514228681_i32,(-1382588633_i32),825909005_i32,(-953491152_i32),2067077524_i32,(-699291463_i32),(-425632345_i32),171364074_i32];
_19 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.1 as u32;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).3 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0 as u128;
_7 = _4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0.0 = !Field::<u16>(Variant(_20, 1), 5);
RET = [Field::<u16>(Variant(_20, 1), 5),Field::<u16>(Variant(_20, 1), 5),Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0];
_5 = _18 as i8;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = [(-637812144_i32),(-1985156210_i32),1723130829_i32,(-1318167365_i32),(-261425403_i32),(-1772087509_i32),(-1805261787_i32),(-1088772736_i32)];
_13 = 2820100671286525071_u64;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0.0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0;
_14 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_26, 2), 6)));
_33 = [(*_14),_19,_19];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0;
_36 = _1 <= _1;
place!(Field::<[i32; 2]>(Variant(_26, 2), 2)) = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).2;
place!(Field::<i128>(Variant(_26, 2), 5)) = !_23;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).2 = [(-1774646148_i32),(-1619288705_i32)];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4;
Goto(bb10)
}
bb10 = {
_38 = '\u{e86ae}';
_16 = _2;
_2 = [1227928151_i32,1402228927_i32,1857631984_i32,(-1582271034_i32),1178858108_i32,466922563_i32,1829082859_i32,33166057_i32];
_38 = '\u{fb563}';
_30 = RET;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).4 = _29 as i16;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).1 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).4 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4 + Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)) = (Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0, _1, Field::<[i32; 2]>(Variant(_26, 2), 2), Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).3, Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4);
place!(Field::<i128>(Variant(_26, 2), 5)) = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).1 as i128;
_3 = [1044435684_i32,(-72633130_i32),805079252_i32,1076447450_i32,1256556105_i32,(-884036872_i32),(-1719108328_i32),(-497441449_i32)];
RET = [Field::<u16>(Variant(_20, 1), 5),Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0,Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0];
_28 = &place!(Field::<u32>(Variant(_26, 2), 6));
_3 = [(-1738928738_i32),(-134706514_i32),723882716_i32,1641872441_i32,1171485674_i32,1229564571_i32,35473208_i32,(-840645897_i32)];
_18 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).3 + Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).3;
place!(Field::<Adt45>(Variant(_20, 1), 1)) = Adt45::Variant0 { fld0: _11,fld1: _32,fld2: Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4,fld3: 16763474645813562635_usize };
_36 = _7 | _4;
_4 = !_7;
_35 = _38;
_31 = [_18];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).1 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).3 = _29 as u128;
_39 = Field::<u16>(Variant(_20, 1), 5) as isize;
match Field::<i16>(Variant(Field::<Adt45>(Variant(_20, 1), 1), 0), 2) {
0 => bb7,
1 => bb8,
2 => bb11,
340282366920938463463374607431768195632 => bb13,
_ => bb12
}
}
bb11 = {
(*_14) = !Field::<u32>(Variant(_26, 2), 6);
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).0.1 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4 as i128;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = Field::<u8>(Variant(_20, 1), 6) as i16;
_1 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
_32 = [_5,_5];
_12 = !_1;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = [1514228681_i32,(-1382588633_i32),825909005_i32,(-953491152_i32),2067077524_i32,(-699291463_i32),(-425632345_i32),171364074_i32];
_19 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.1 as u32;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).3 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0 as u128;
_7 = _4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0.0 = !Field::<u16>(Variant(_20, 1), 5);
RET = [Field::<u16>(Variant(_20, 1), 5),Field::<u16>(Variant(_20, 1), 5),Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0];
_5 = _18 as i8;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = [(-637812144_i32),(-1985156210_i32),1723130829_i32,(-1318167365_i32),(-261425403_i32),(-1772087509_i32),(-1805261787_i32),(-1088772736_i32)];
_13 = 2820100671286525071_u64;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0.0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0;
_14 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_26, 2), 6)));
_33 = [(*_14),_19,_19];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).0 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0;
_36 = _1 <= _1;
place!(Field::<[i32; 2]>(Variant(_26, 2), 2)) = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).2;
place!(Field::<i128>(Variant(_26, 2), 5)) = !_23;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).2 = [(-1774646148_i32),(-1619288705_i32)];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).4 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4;
Goto(bb10)
}
bb12 = {
_2 = _3;
_2 = [(-1286865775_i32),949198417_i32,313825651_i32,(-1313427786_i32),1559445098_i32,1991309500_i32,930529486_i32,(-739066125_i32)];
_14 = core::ptr::addr_of_mut!(_19);
_19 = 3614441105_u32 ^ 2016660171_u32;
_6 = [302171890_i32,650600376_i32,1663304121_i32,1925065947_i32,(-933296821_i32),(-302196499_i32)];
_15 = [6571557036496792613_i64,(-8994536335021630253_i64),(-3604179580069952892_i64),(-5715527241291639442_i64),2329480952074939021_i64,8953605304115871159_i64,(-4655326357421821898_i64),977269963317023395_i64];
_6 = [(-1654887897_i32),(-2036238780_i32),1368083641_i32,230416920_i32,66685868_i32,838604789_i32];
RET = [29780_u16,39151_u16,12657_u16];
RET = [59064_u16,35857_u16,39145_u16];
_3 = [1704905302_i32,1973563150_i32,(-429764163_i32),(-1477462696_i32),1162059116_i32,(-299949940_i32),267048004_i32,807270808_i32];
_6 = [1459431426_i32,132078535_i32,1608001377_i32,562287404_i32,1855965946_i32,(-106478562_i32)];
_10 = _1 - _1;
_18 = !193553183967732975563216629204620120550_u128;
Call((*_14) = core::intrinsics::bswap(3169127883_u32), ReturnTo(bb6), UnwindUnreachable())
}
bb13 = {
_27 = Field::<f64>(Variant(Field::<Adt45>(Variant(_20, 1), 1), 0), 0) as u64;
_42 = _36 | _7;
RET = [Field::<u16>(Variant(_20, 1), 5),Field::<u16>(Variant(_20, 1), 5),Field::<u16>(Variant(_20, 1), 5)];
_19 = (*_14) << Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).1 = _29 as isize;
place!(Field::<f64>(Variant(place!(Field::<Adt45>(Variant(_20, 1), 1)), 0), 0)) = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).3 as f64;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).1 = !_1;
_24 = _29;
place!(Field::<[u16; 3]>(Variant(_20, 1), 0)) = [Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0,Field::<u16>(Variant(_20, 1), 5),Field::<u16>(Variant(_20, 1), 5)];
match Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4 {
0 => bb1,
1 => bb5,
2 => bb9,
3 => bb14,
4 => bb15,
340282366920938463463374607431768195632 => bb17,
_ => bb16
}
}
bb14 = {
_1 = 9223372036854775807_isize * 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_5 = 63_i8 ^ 55_i8;
_7 = !_4;
_2 = _3;
_4 = _7;
_6 = [(-920510753_i32),1473114218_i32,1152142849_i32,1250291925_i32,1984086222_i32,(-1268492488_i32)];
_6 = [(-957456691_i32),1889774121_i32,(-1537774925_i32),(-452433305_i32),1339717816_i32,(-1224670217_i32)];
_8 = Adt48::Variant2 { fld0: _6 };
place!(Field::<[i32; 6]>(Variant(_8, 2), 0)) = _6;
SetDiscriminant(_8, 3);
place!(Field::<bool>(Variant(_8, 3), 0)) = _4 > _4;
place!(Field::<[u64; 6]>(Variant(_8, 3), 4)) = [5994505194484205098_u64,4652748143252787925_u64,2674518813425678660_u64,17380305009222076070_u64,1082391985884424083_u64,1684292507517655654_u64];
place!(Field::<i32>(Variant(_8, 3), 5)) = 631538226_i32;
_2 = _3;
Goto(bb2)
}
bb15 = {
RET = [10353_u16,25532_u16,52019_u16];
_11 = 59941_u16 as f64;
_7 = _12 <= _12;
_11 = 57290599793814605593467044865573911610_i128 as f64;
_6 = [(-374756173_i32),(-1390029333_i32),(-1225828455_i32),(-1861079317_i32),(-786202889_i32),(-1488939103_i32)];
_1 = !_12;
_13 = 2609_u16 as u64;
_5 = 70_i8;
RET = [44222_u16,51721_u16,34649_u16];
_5 = (-1_i8) << _1;
_2 = [(-302080125_i32),126074439_i32,274628156_i32,1190862452_i32,(-1799773491_i32),(-341870107_i32),(-12681988_i32),(-566874434_i32)];
_3 = _2;
_1 = _12 - _12;
_15 = [2873228024363127087_i64,903373004064950381_i64,(-2218674217127479140_i64),(-6608977237837928061_i64),(-6038221862826407754_i64),(-5917882667944107973_i64),33404891685340518_i64,(-7945745347628655369_i64)];
_6 = [1441630477_i32,880072760_i32,417716384_i32,(-1311019442_i32),606963903_i32,2141129225_i32];
_2 = [818744855_i32,(-378289472_i32),1609109967_i32,1438431395_i32,1064996194_i32,(-1050277912_i32),(-859085537_i32),(-2070371891_i32)];
_10 = -_1;
RET = [9893_u16,29371_u16,4536_u16];
_7 = _1 == _1;
_1 = _12 - _12;
_6 = [92517313_i32,(-1703254692_i32),(-886342308_i32),(-1232325222_i32),(-232594484_i32),1188985553_i32];
_3 = [(-1021176253_i32),(-289325851_i32),(-1508976754_i32),1944336659_i32,(-1392983779_i32),651242050_i32,(-1096752968_i32),1457707231_i32];
_11 = 36492_u16 as f64;
_11 = _1 as f64;
_5 = _7 as i8;
_16 = _2;
Call(_18 = fn9(_10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_38 = '\u{e86ae}';
_16 = _2;
_2 = [1227928151_i32,1402228927_i32,1857631984_i32,(-1582271034_i32),1178858108_i32,466922563_i32,1829082859_i32,33166057_i32];
_38 = '\u{fb563}';
_30 = RET;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).4 = _29 as i16;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).1 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).4 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4 + Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)) = (Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0, _1, Field::<[i32; 2]>(Variant(_26, 2), 2), Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).3, Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4);
place!(Field::<i128>(Variant(_26, 2), 5)) = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).1 as i128;
_3 = [1044435684_i32,(-72633130_i32),805079252_i32,1076447450_i32,1256556105_i32,(-884036872_i32),(-1719108328_i32),(-497441449_i32)];
RET = [Field::<u16>(Variant(_20, 1), 5),Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0,Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0];
_28 = &place!(Field::<u32>(Variant(_26, 2), 6));
_3 = [(-1738928738_i32),(-134706514_i32),723882716_i32,1641872441_i32,1171485674_i32,1229564571_i32,35473208_i32,(-840645897_i32)];
_18 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).3 + Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).3;
place!(Field::<Adt45>(Variant(_20, 1), 1)) = Adt45::Variant0 { fld0: _11,fld1: _32,fld2: Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).4,fld3: 16763474645813562635_usize };
_36 = _7 | _4;
_4 = !_7;
_35 = _38;
_31 = [_18];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).1 = -Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).1;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3)).3 = _29 as u128;
_39 = Field::<u16>(Variant(_20, 1), 5) as isize;
match Field::<i16>(Variant(Field::<Adt45>(Variant(_20, 1), 1), 0), 2) {
0 => bb7,
1 => bb8,
2 => bb11,
340282366920938463463374607431768195632 => bb13,
_ => bb12
}
}
bb17 = {
_43 = RET;
place!(Field::<[u16; 3]>(Variant(_20, 1), 0)) = [Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0).0.0,Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_20, 1), 3).0.0,Field::<u16>(Variant(_20, 1), 5)];
_42 = _36;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_26, 2), 0)).3 = _18 * _18;
place!(Field::<usize>(Variant(place!(Field::<Adt45>(Variant(_20, 1), 1)), 0), 3)) = 4_usize;
_19 = (*_28);
_35 = _38;
_32 = [_5,_5];
place!(Field::<u32>(Variant(_26, 2), 6)) = _19 - _19;
place!(Field::<u8>(Variant(_20, 1), 6)) = 87_u8 * 95_u8;
place!(Field::<[i32; 8]>(Variant(_26, 2), 3)) = _16;
Goto(bb18)
}
bb18 = {
Call(_51 = dump_var(5_usize, 1_usize, Move(_1), 12_usize, Move(_12), 30_usize, Move(_30), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(5_usize, 38_usize, Move(_38), 7_usize, Move(_7), 25_usize, Move(_25), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_51 = dump_var(5_usize, 43_usize, Move(_43), 5_usize, Move(_5), 16_usize, Move(_16), 42_usize, Move(_42)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_51 = dump_var(5_usize, 15_usize, Move(_15), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: Adt48,mut _2: isize,mut _3: [i32; 8],mut _4: [i32; 6],mut _5: bool,mut _6: [i32; 8],mut _7: [i32; 6],mut _8: [i32; 8],mut _9: i8,mut _10: [u16; 3],mut _11: isize,mut _12: [i32; 8],mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: u32;
let _15: f32;
let _16: isize;
let _17: u64;
let _18: [bool; 2];
let _19: Adt44;
let _20: i32;
let _21: [i16; 6];
let _22: Adt48;
let _23: Adt47;
let _24: ();
let _25: ();
{
_3 = [(-1667811354_i32),(-1633419461_i32),721100289_i32,1063704403_i32,(-42945302_i32),(-765949091_i32),1270003154_i32,970088907_i32];
_6 = [49052384_i32,1114335086_i32,1858810384_i32,1407302664_i32,1187318311_i32,741588628_i32,(-1969905344_i32),(-1545737374_i32)];
_14 = !3360503806_u32;
_6 = [(-993402569_i32),(-1461093275_i32),248445552_i32,(-238050497_i32),1743156970_i32,(-1931143217_i32),(-173671682_i32),2144827282_i32];
place!(Field::<[i32; 6]>(Variant(_1, 2), 0)) = [1089915306_i32,1014851294_i32,1324500018_i32,(-619337648_i32),(-2095981424_i32),(-1781571701_i32)];
_4 = [(-1077217741_i32),1829381884_i32,1732818579_i32,2045878145_i32,(-1032640675_i32),1965218852_i32];
_6 = [343151727_i32,(-384871323_i32),559732639_i32,(-925562455_i32),(-271574897_i32),(-1799718279_i32),(-303415044_i32),(-398130140_i32)];
_11 = _2 - _13;
_4 = Field::<[i32; 6]>(Variant(_1, 2), 0);
place!(Field::<[i32; 6]>(Variant(_1, 2), 0)) = _7;
_14 = 828370786_u32 << _11;
_6 = [347429464_i32,(-906383660_i32),1259423373_i32,(-1533087993_i32),1374047169_i32,(-906458282_i32),(-932356793_i32),(-1425000337_i32)];
RET = _2 | _11;
_5 = false;
Call(_6 = fn7(_12, Move(_1), _2, _11, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = RET;
_5 = !false;
_8 = [(-2030401426_i32),1997788228_i32,(-1617569186_i32),(-479572847_i32),948264377_i32,(-1062037526_i32),(-1574173201_i32),(-701502503_i32)];
_3 = _6;
_16 = RET + _2;
_10 = [22911_u16,25883_u16,10077_u16];
_8 = [(-1680310217_i32),(-862231613_i32),(-207580022_i32),28019710_i32,(-1967196849_i32),(-875413444_i32),1472678766_i32,1136828872_i32];
_16 = !RET;
_12 = [579728944_i32,1930086912_i32,(-1910677079_i32),(-708942289_i32),(-1233890041_i32),(-1025865599_i32),(-2113423048_i32),836409189_i32];
_6 = [(-297021802_i32),(-158574509_i32),(-1710716051_i32),1991643836_i32,2079316199_i32,622883696_i32,2060127139_i32,(-642978470_i32)];
_6 = [(-24321002_i32),1263132137_i32,1229764430_i32,282319703_i32,549557336_i32,(-245245981_i32),(-2125447885_i32),(-1721276558_i32)];
_17 = 3_usize as u64;
_12 = [(-1635309554_i32),1074411091_i32,(-906496473_i32),(-725013844_i32),(-1310706455_i32),1090863374_i32,(-1325589817_i32),(-859985481_i32)];
_3 = [(-1952837746_i32),(-592465946_i32),1563056704_i32,1496313499_i32,1606990344_i32,(-1450224357_i32),(-601056445_i32),(-238993677_i32)];
_10 = [49169_u16,22500_u16,14599_u16];
_10 = [34254_u16,17061_u16,34508_u16];
_2 = 4894784379199949300_i64 as isize;
_11 = _14 as isize;
_15 = 597560843_i32 as f32;
_12 = [(-354480430_i32),250899818_i32,727505916_i32,1502324361_i32,1056209234_i32,(-331446610_i32),1860179485_i32,(-826431171_i32)];
_6 = _8;
_16 = _14 as isize;
_17 = 14819683051108651666_u64;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
14819683051108651666 => bb10,
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
_1 = Adt48::Variant2 { fld0: _4 };
_4 = _7;
SetDiscriminant(_1, 2);
_8 = _12;
_18 = [_5,_5];
_1 = Adt48::Variant2 { fld0: _4 };
_7 = [(-1225048836_i32),796274517_i32,(-1546180851_i32),(-927978297_i32),522662140_i32,1727616263_i32];
_9 = (-75_i8);
_13 = _11;
_5 = _16 < _13;
_14 = _5 as u32;
_8 = [(-489477037_i32),1041681774_i32,1607063834_i32,722486701_i32,1530403541_i32,(-1536857681_i32),(-2028924370_i32),(-874864599_i32)];
_17 = 9516519590045853647_u64;
SetDiscriminant(_1, 2);
_10 = [15329_u16,3074_u16,53606_u16];
_17 = 2903189928258627679_u64;
_6 = [(-816877869_i32),719862706_i32,1233954216_i32,(-1123806964_i32),433387600_i32,(-1456102094_i32),(-1640689927_i32),(-946093341_i32)];
_2 = !_13;
_10 = [14385_u16,44325_u16,24068_u16];
_13 = -RET;
_13 = _9 as isize;
_11 = -_2;
place!(Field::<[i32; 6]>(Variant(_1, 2), 0)) = [(-712128162_i32),(-1690599398_i32),(-1472309634_i32),822766100_i32,294869942_i32,(-1456132903_i32)];
_3 = _6;
_12 = [(-198811991_i32),237752256_i32,136722925_i32,212399758_i32,1554967299_i32,(-856393135_i32),(-1894795003_i32),959131862_i32];
_6 = _3;
match _9 {
0 => bb1,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607431768211381 => bb15,
_ => bb14
}
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
_11 = RET;
_5 = !false;
_8 = [(-2030401426_i32),1997788228_i32,(-1617569186_i32),(-479572847_i32),948264377_i32,(-1062037526_i32),(-1574173201_i32),(-701502503_i32)];
_3 = _6;
_16 = RET + _2;
_10 = [22911_u16,25883_u16,10077_u16];
_8 = [(-1680310217_i32),(-862231613_i32),(-207580022_i32),28019710_i32,(-1967196849_i32),(-875413444_i32),1472678766_i32,1136828872_i32];
_16 = !RET;
_12 = [579728944_i32,1930086912_i32,(-1910677079_i32),(-708942289_i32),(-1233890041_i32),(-1025865599_i32),(-2113423048_i32),836409189_i32];
_6 = [(-297021802_i32),(-158574509_i32),(-1710716051_i32),1991643836_i32,2079316199_i32,622883696_i32,2060127139_i32,(-642978470_i32)];
_6 = [(-24321002_i32),1263132137_i32,1229764430_i32,282319703_i32,549557336_i32,(-245245981_i32),(-2125447885_i32),(-1721276558_i32)];
_17 = 3_usize as u64;
_12 = [(-1635309554_i32),1074411091_i32,(-906496473_i32),(-725013844_i32),(-1310706455_i32),1090863374_i32,(-1325589817_i32),(-859985481_i32)];
_3 = [(-1952837746_i32),(-592465946_i32),1563056704_i32,1496313499_i32,1606990344_i32,(-1450224357_i32),(-601056445_i32),(-238993677_i32)];
_10 = [49169_u16,22500_u16,14599_u16];
_10 = [34254_u16,17061_u16,34508_u16];
_2 = 4894784379199949300_i64 as isize;
_11 = _14 as isize;
_15 = 597560843_i32 as f32;
_12 = [(-354480430_i32),250899818_i32,727505916_i32,1502324361_i32,1056209234_i32,(-331446610_i32),1860179485_i32,(-826431171_i32)];
_6 = _8;
_16 = _14 as isize;
_17 = 14819683051108651666_u64;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
14819683051108651666 => bb10,
_ => bb9
}
}
bb15 = {
_4 = [1827506824_i32,(-1000069435_i32),848292907_i32,123480921_i32,(-779162392_i32),1195243716_i32];
SetDiscriminant(_1, 3);
_6 = [1700320555_i32,(-710053892_i32),1392390625_i32,(-220087616_i32),(-1362837881_i32),1930109550_i32,(-675007136_i32),1097623232_i32];
place!(Field::<[u64; 6]>(Variant(_1, 3), 4)) = [_17,_17,_17,_17,_17,_17];
_17 = (-4368715509683933916_i64) as u64;
_18 = [_5,_5];
place!(Field::<Adt38>(Variant(_1, 3), 2)).fld2 = [_9,_9];
place!(Field::<i32>(Variant(_1, 3), 5)) = !1858596123_i32;
_14 = _5 as u32;
_6 = _8;
_21 = [(-14144_i16),(-9670_i16),(-3568_i16),4181_i16,(-29114_i16),(-23191_i16)];
_20 = Field::<i32>(Variant(_1, 3), 5) + Field::<i32>(Variant(_1, 3), 5);
place!(Field::<bool>(Variant(_1, 3), 0)) = _5 | _5;
place!(Field::<(i16, ([i16; 6],))>(Variant(_1, 3), 1)).1 = (_21,);
place!(Field::<Adt38>(Variant(_1, 3), 2)).fld0 = core::ptr::addr_of!(place!(Field::<[u64; 6]>(Variant(_1, 3), 4)));
place!(Field::<i32>(Variant(_1, 3), 5)) = _20;
place!(Field::<Adt38>(Variant(_1, 3), 2)).fld0 = core::ptr::addr_of!(place!(Field::<[u64; 6]>(Variant(_1, 3), 4)));
place!(Field::<Adt38>(Variant(_1, 3), 2)).fld2 = [_9,_9];
_5 = !Field::<bool>(Variant(_1, 3), 0);
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(6_usize, 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(6_usize, 8_usize, Move(_8), 18_usize, Move(_18), 17_usize, Move(_17), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(6_usize, 11_usize, Move(_11), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i32; 8],mut _2: Adt48,mut _3: isize,mut _4: isize,mut _5: isize) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _6: [bool; 2];
let _7: ((u16, i128), isize, [i32; 2], u128, i16);
let _8: isize;
let _9: f32;
let _10: (i16, ([i16; 6],));
let _11: isize;
let _12: ((u16, i128), isize, [i32; 2], u128, i16);
let _13: char;
let _14: [i32; 8];
let _15: u32;
let _16: f64;
let _17: [i16; 6];
let _18: Adt36;
let _19: Adt41;
let _20: isize;
let _21: isize;
let _22: Adt48;
let _23: Adt38;
let _24: ();
let _25: ();
{
RET = [521761788_i32,(-1697603531_i32),(-407611768_i32),(-902988356_i32),(-1238626434_i32),(-1164188896_i32),(-1194281223_i32),433548685_i32];
SetDiscriminant(_2, 0);
_3 = _5 | _4;
_4 = _3;
_1 = RET;
_6 = [false,false];
_7.4 = (-3977741418028219940_i64) as i16;
_6 = [false,false];
_7.2 = [443820049_i32,(-4221737_i32)];
_4 = _3;
_8 = _4;
_7.0.0 = 46904_u16;
_7.4 = 28074_i16;
_7.0.0 = 61029_u16 << _4;
_7.1 = '\u{3680b}' as isize;
match _7.4 {
0 => bb1,
1 => bb2,
2 => bb3,
28074 => bb5,
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
_7.0.1 = (-3362521032526595948157650943258354879_i128);
_9 = (-1240964559_i32) as f32;
place!(Field::<u8>(Variant(_2, 0), 1)) = 65225359405028367890920922558080309970_u128 as u8;
_7.0.1 = !(-129443084659528900860959472557171040587_i128);
RET = [(-780860155_i32),(-1015108844_i32),575739823_i32,(-920457763_i32),(-1783064035_i32),1160183556_i32,(-1129210322_i32),123920880_i32];
_10.1.0 = [_7.4,_7.4,_7.4,_7.4,_7.4,_7.4];
_7.2 = [(-845862392_i32),(-2039928211_i32)];
_7.0.0 = 1_usize as u16;
_11 = _3 + _4;
_12 = (_7.0, _8, _7.2, 147477472720167419593407525881133283903_u128, _7.4);
_6 = [true,true];
_7 = (_12.0, _5, _12.2, _12.3, _12.4);
_7.0.1 = _12.3 as i128;
Goto(bb6)
}
bb6 = {
_5 = _3;
RET = [500820585_i32,(-215235183_i32),(-490148727_i32),(-1968337902_i32),911824805_i32,(-206076798_i32),(-544873383_i32),(-1409552947_i32)];
_7.4 = _12.4;
_12.2 = [(-1729532088_i32),(-234707498_i32)];
_10.1.0 = [_7.4,_7.4,_7.4,_7.4,_12.4,_7.4];
_10.0 = _7.4;
_8 = _5;
_10.1.0 = [_10.0,_12.4,_12.4,_12.4,_12.4,_7.4];
_7.4 = 4_usize as i16;
_12.4 = !_7.4;
_7.4 = _12.4;
_12.0 = (_7.0.0, _7.0.1);
_7.1 = _7.3 as isize;
Call(_7.3 = fn8(_4, _7.1, _12.3, _7.0.1, _12.3, _7.1, _12, _11, _7.1, _12, _11, _12.4, _11, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7.0 = (_12.0.0, _12.0.1);
place!(Field::<u8>(Variant(_2, 0), 1)) = !62_u8;
_7.2 = [(-24101702_i32),(-872479774_i32)];
_12.4 = !_10.0;
_10.0 = _12.4;
_4 = '\u{269f8}' as isize;
_12.0.1 = _7.0.1 & _7.0.1;
_7 = (_12.0, _11, _12.2, _12.3, _12.4);
_15 = (-5_i8) as u32;
_7.2 = _12.2;
_12 = (_7.0, _7.1, _7.2, _7.3, _7.4);
_7.4 = _10.0;
_7 = (_12.0, _8, _12.2, _12.3, _10.0);
_7.4 = Field::<u8>(Variant(_2, 0), 1) as i16;
RET = [497762066_i32,(-1209006510_i32),1695271723_i32,(-1226412019_i32),(-454094353_i32),709009282_i32,1202140446_i32,(-1998778235_i32)];
_12.4 = 7264593222263917481_i64 as i16;
RET = [1234476717_i32,(-110609443_i32),1647811627_i32,1412815033_i32,(-448381010_i32),1240172064_i32,1412096979_i32,(-39197334_i32)];
_12.0.0 = _7.0.0;
_7.2 = [1445208570_i32,(-901237899_i32)];
_7.4 = _10.0 * _10.0;
_7.0.1 = _12.0.1 >> _7.4;
_16 = Field::<u8>(Variant(_2, 0), 1) as f64;
_1 = [(-1834343589_i32),551497492_i32,(-991339466_i32),(-553936683_i32),(-992181683_i32),(-2079428610_i32),(-1784852347_i32),(-482197067_i32)];
RET = _1;
place!(Field::<u8>(Variant(_2, 0), 1)) = !187_u8;
Goto(bb8)
}
bb8 = {
_12.0.1 = _7.0.1 * _7.0.1;
RET = [(-259818373_i32),473549917_i32,1801826969_i32,1380329105_i32,(-1781847217_i32),1257274895_i32,(-1496348088_i32),(-508466810_i32)];
_11 = _7.1;
_9 = _12.0.0 as f32;
_7.1 = 6760282220267748691_usize as isize;
_19.fld0 = [8452825669588998126_i64,1546623068274966666_i64,182362952326730214_i64,7698922242117537860_i64,(-3110862079420522253_i64),(-1382386553192020764_i64),(-8211655741870191567_i64),1778782314460482587_i64];
_19.fld1 = !3429699960917991379_i64;
_18 = Adt36::Variant0 { fld0: _16 };
_12 = (_7.0, _5, _7.2, _7.3, _10.0);
_12.0.1 = _7.0.1 & _7.0.1;
_19.fld0 = [_19.fld1,_19.fld1,_19.fld1,_19.fld1,_19.fld1,_19.fld1,_19.fld1,_19.fld1];
match _12.3 {
0 => bb2,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
147477472720167419593407525881133283903 => bb16,
_ => bb15
}
}
bb9 = {
_7.0 = (_12.0.0, _12.0.1);
place!(Field::<u8>(Variant(_2, 0), 1)) = !62_u8;
_7.2 = [(-24101702_i32),(-872479774_i32)];
_12.4 = !_10.0;
_10.0 = _12.4;
_4 = '\u{269f8}' as isize;
_12.0.1 = _7.0.1 & _7.0.1;
_7 = (_12.0, _11, _12.2, _12.3, _12.4);
_15 = (-5_i8) as u32;
_7.2 = _12.2;
_12 = (_7.0, _7.1, _7.2, _7.3, _7.4);
_7.4 = _10.0;
_7 = (_12.0, _8, _12.2, _12.3, _10.0);
_7.4 = Field::<u8>(Variant(_2, 0), 1) as i16;
RET = [497762066_i32,(-1209006510_i32),1695271723_i32,(-1226412019_i32),(-454094353_i32),709009282_i32,1202140446_i32,(-1998778235_i32)];
_12.4 = 7264593222263917481_i64 as i16;
RET = [1234476717_i32,(-110609443_i32),1647811627_i32,1412815033_i32,(-448381010_i32),1240172064_i32,1412096979_i32,(-39197334_i32)];
_12.0.0 = _7.0.0;
_7.2 = [1445208570_i32,(-901237899_i32)];
_7.4 = _10.0 * _10.0;
_7.0.1 = _12.0.1 >> _7.4;
_16 = Field::<u8>(Variant(_2, 0), 1) as f64;
_1 = [(-1834343589_i32),551497492_i32,(-991339466_i32),(-553936683_i32),(-992181683_i32),(-2079428610_i32),(-1784852347_i32),(-482197067_i32)];
RET = _1;
place!(Field::<u8>(Variant(_2, 0), 1)) = !187_u8;
Goto(bb8)
}
bb10 = {
_5 = _3;
RET = [500820585_i32,(-215235183_i32),(-490148727_i32),(-1968337902_i32),911824805_i32,(-206076798_i32),(-544873383_i32),(-1409552947_i32)];
_7.4 = _12.4;
_12.2 = [(-1729532088_i32),(-234707498_i32)];
_10.1.0 = [_7.4,_7.4,_7.4,_7.4,_12.4,_7.4];
_10.0 = _7.4;
_8 = _5;
_10.1.0 = [_10.0,_12.4,_12.4,_12.4,_12.4,_7.4];
_7.4 = 4_usize as i16;
_12.4 = !_7.4;
_7.4 = _12.4;
_12.0 = (_7.0.0, _7.0.1);
_7.1 = _7.3 as isize;
Call(_7.3 = fn8(_4, _7.1, _12.3, _7.0.1, _12.3, _7.1, _12, _11, _7.1, _12, _11, _12.4, _11, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_7.0.1 = (-3362521032526595948157650943258354879_i128);
_9 = (-1240964559_i32) as f32;
place!(Field::<u8>(Variant(_2, 0), 1)) = 65225359405028367890920922558080309970_u128 as u8;
_7.0.1 = !(-129443084659528900860959472557171040587_i128);
RET = [(-780860155_i32),(-1015108844_i32),575739823_i32,(-920457763_i32),(-1783064035_i32),1160183556_i32,(-1129210322_i32),123920880_i32];
_10.1.0 = [_7.4,_7.4,_7.4,_7.4,_7.4,_7.4];
_7.2 = [(-845862392_i32),(-2039928211_i32)];
_7.0.0 = 1_usize as u16;
_11 = _3 + _4;
_12 = (_7.0, _8, _7.2, 147477472720167419593407525881133283903_u128, _7.4);
_6 = [true,true];
_7 = (_12.0, _5, _12.2, _12.3, _12.4);
_7.0.1 = _12.3 as i128;
Goto(bb6)
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
_7.3 = _15 as u128;
place!(Field::<u8>(Variant(_2, 0), 1)) = (-1940020612_i32) as u8;
_3 = _8 << _12.3;
_19.fld3 = 94_i8 & (-61_i8);
place!(Field::<u8>(Variant(_2, 0), 1)) = !189_u8;
_12.4 = !_10.0;
_9 = _3 as f32;
_21 = _3;
_17 = [_10.0,_7.4,_7.4,_7.4,_7.4,_7.4];
_23.fld1 = [(-272122478_i32),(-1658773226_i32)];
_7.0.0 = _12.0.0 + _12.0.0;
_10.1.0 = [_10.0,_10.0,_10.0,_10.0,_7.4,_7.4];
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(7_usize, 3_usize, Move(_3), 11_usize, Move(_11), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(7_usize, 10_usize, Move(_10), 7_usize, Move(_7), 25_usize, _25, 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: u128,mut _4: i128,mut _5: u128,mut _6: isize,mut _7: ((u16, i128), isize, [i32; 2], u128, i16),mut _8: isize,mut _9: isize,mut _10: ((u16, i128), isize, [i32; 2], u128, i16),mut _11: isize,mut _12: i16,mut _13: isize,mut _14: isize) -> u128 {
mir! {
type RET = u128;
let _15: isize;
let _16: Adt42;
let _17: ();
let _18: ();
{
_7.1 = (-6976104980844822562_i64) as isize;
_11 = _8;
_8 = _11;
_2 = -_13;
_4 = -_10.0.1;
_9 = _8 << _13;
_3 = 3265181296_u32 as u128;
RET = 160_u8 as u128;
_12 = _7.4;
_13 = _9;
_2 = 1226268502149011613_i64 as isize;
_6 = _9;
_7.2 = [(-1637401711_i32),337058656_i32];
_10.3 = !_5;
_7 = _10;
_14 = -_13;
RET = _10.3 * _7.3;
_7.3 = !RET;
_7.3 = RET;
_10.2 = [(-1959092272_i32),(-717744321_i32)];
_5 = _7.3 * RET;
_7.4 = 31_i8 as i16;
_5 = _7.3 >> _6;
_15 = -_6;
_3 = _7.3;
_4 = 109_u8 as i128;
_10.0.1 = 969555915_u32 as i128;
_10.3 = _5 >> _8;
_12 = _7.4;
_1 = -_14;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(8_usize, 2_usize, Move(_2), 5_usize, Move(_5), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(8_usize, 9_usize, Move(_9), 6_usize, Move(_6), 8_usize, Move(_8), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: bool) -> u128 {
mir! {
type RET = u128;
let _3: char;
let _4: isize;
let _5: u64;
let _6: ((u16, i128), isize, [i32; 2], u128, i16);
let _7: Adt47;
let _8: char;
let _9: (u16, i128);
let _10: Adt39;
let _11: *const [u64; 6];
let _12: Adt48;
let _13: isize;
let _14: [bool; 2];
let _15: [i16; 6];
let _16: isize;
let _17: [u16; 3];
let _18: [u16; 3];
let _19: ((u16, i128), isize, [i32; 2], u128, i16);
let _20: [i8; 2];
let _21: Adt48;
let _22: f64;
let _23: [i32; 2];
let _24: usize;
let _25: isize;
let _26: [i8; 2];
let _27: [i64; 8];
let _28: [bool; 2];
let _29: f64;
let _30: u8;
let _31: u8;
let _32: Adt38;
let _33: i128;
let _34: ();
let _35: ();
{
_2 = true | true;
_2 = !false;
_2 = true | true;
RET = 20605616881150878290622608958020559556_u128 | 91206220500040171697375384235803853665_u128;
_4 = '\u{a970f}' as isize;
_1 = 3781045921_u32 as isize;
RET = 198224115452924826541964807122370688927_u128 & 271698326526163743317324617134134257636_u128;
RET = !275232476880417161193284658436189507281_u128;
_4 = _1 & _1;
_1 = -_4;
_4 = (-9822_i16) as isize;
_3 = '\u{94e95}';
_3 = '\u{b3064}';
_3 = '\u{1f15b}';
_2 = false ^ false;
_5 = _1 as u64;
RET = 294660193016326034518819088966356161440_u128;
RET = 123726598952953585660594226380717376623_u128 | 70536626469674777711582305616082986239_u128;
_2 = !true;
_4 = RET as isize;
RET = !324856465560058073155096707910716263683_u128;
_3 = '\u{b7d8d}';
Goto(bb1)
}
bb1 = {
RET = 182323180165279064522946923273241223153_u128 - 294747663210525755728916632761978944360_u128;
Call(RET = fn10(_4, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = !16207101882912933075_u64;
_1 = _4;
_4 = !_1;
RET = !2164096237212957575667465671692725748_u128;
RET = 129850142093296274812632899607200424349_u128 - 157510968049174287618831633428089457751_u128;
_4 = _1 ^ _1;
_1 = (-34_i8) as isize;
_6.1 = _5 as isize;
_1 = _4 >> _4;
_6.4 = _2 as i16;
_3 = '\u{2c31b}';
_6.2 = [1321630852_i32,(-505450068_i32)];
_6.0.1 = _4 as i128;
_6.0.0 = 58761_u16;
_1 = (-20200024_i32) as isize;
_6.2 = [(-1440702161_i32),(-2019086531_i32)];
_1 = !_6.1;
_6.3 = RET + RET;
_6.2 = [827365288_i32,(-1346346081_i32)];
_2 = _4 <= _4;
RET = _6.3 & _6.3;
_4 = _6.1 >> _6.0.0;
_6.3 = !RET;
Call(_6.0.0 = core::intrinsics::transmute(_6.4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.0 = (41819_u16, (-166849943180048312842696350379665281773_i128));
_3 = '\u{ef7fc}';
_6.0 = (28670_u16, 70344445315788012361739824398180319334_i128);
_6.3 = _3 as u128;
_2 = false;
RET = _6.3;
_1 = _6.4 as isize;
_6.1 = -_4;
RET = !_6.3;
_6.4 = -32120_i16;
_6.3 = _3 as u128;
_6.3 = _3 as u128;
_6.0.0 = 23655_u16 + 33102_u16;
_6.0.1 = 89611180016560960997370143727639298561_i128 * 152502181216584866337702531047995968360_i128;
_4 = -_6.1;
RET = (-91_i8) as u128;
_2 = false | true;
_6.3 = _2 as u128;
_6.0 = (45995_u16, 75579708580155769772956424508278124454_i128);
_6.1 = _4 + _4;
_5 = 7655257314178351890_u64;
RET = _6.3 & _6.3;
_6.3 = RET;
_6.0.1 = 164126051859680980909433006774217266903_i128 * (-139153097847488202101022148435212189085_i128);
_6.2 = [(-2122269245_i32),129700148_i32];
_1 = !_6.1;
_8 = _3;
_3 = _8;
Goto(bb4)
}
bb4 = {
_6.4 = 21928_i16 | (-23338_i16);
_6.3 = !RET;
_6.1 = _1;
_9.1 = 752469550_u32 as i128;
_6.3 = _5 as u128;
_6.3 = 180_u8 as u128;
_9 = (_6.0.0, _6.0.1);
_3 = _8;
_5 = 7066554664742830555_u64;
_9.1 = !_6.0.1;
_8 = _3;
_6.4 = 3264_i16;
_3 = _8;
RET = _6.3;
_8 = _3;
_9 = _6.0;
_5 = _8 as u64;
Goto(bb5)
}
bb5 = {
RET = 2534378321_u32 as u128;
_8 = _3;
_6.0 = (_9.0, _9.1);
_9 = (_6.0.0, _6.0.1);
_4 = !_6.1;
_6.0 = _9;
_9 = (_6.0.0, _6.0.1);
_4 = _1 & _6.1;
_9 = (_6.0.0, _6.0.1);
_1 = (-40_i8) as isize;
_8 = _3;
_13 = _6.1;
_8 = _3;
_9.0 = !_6.0.0;
_6.3 = _2 as u128;
_13 = _4 ^ _6.1;
_14 = [_2,_2];
_9 = (_6.0.0, _6.0.1);
_6.4 = _2 as i16;
_15 = [_6.4,_6.4,_6.4,_6.4,_6.4,_6.4];
Goto(bb6)
}
bb6 = {
_15 = [_6.4,_6.4,_6.4,_6.4,_6.4,_6.4];
_6.0.0 = _5 as u16;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
45995 => bb9,
_ => bb8
}
}
bb7 = {
RET = 182323180165279064522946923273241223153_u128 - 294747663210525755728916632761978944360_u128;
Call(RET = fn10(_4, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_5 = !16207101882912933075_u64;
_1 = _4;
_4 = !_1;
RET = !2164096237212957575667465671692725748_u128;
RET = 129850142093296274812632899607200424349_u128 - 157510968049174287618831633428089457751_u128;
_4 = _1 ^ _1;
_1 = (-34_i8) as isize;
_6.1 = _5 as isize;
_1 = _4 >> _4;
_6.4 = _2 as i16;
_3 = '\u{2c31b}';
_6.2 = [1321630852_i32,(-505450068_i32)];
_6.0.1 = _4 as i128;
_6.0.0 = 58761_u16;
_1 = (-20200024_i32) as isize;
_6.2 = [(-1440702161_i32),(-2019086531_i32)];
_1 = !_6.1;
_6.3 = RET + RET;
_6.2 = [827365288_i32,(-1346346081_i32)];
_2 = _4 <= _4;
RET = _6.3 & _6.3;
_4 = _6.1 >> _6.0.0;
_6.3 = !RET;
Call(_6.0.0 = core::intrinsics::transmute(_6.4), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_8 = _3;
_9.0 = (-1788355643_i32) as u16;
_6.0.0 = RET as u16;
_15 = [_6.4,_6.4,_6.4,_6.4,_6.4,_6.4];
_5 = _6.3 as u64;
_4 = -_6.1;
_4 = _13;
_14 = [_2,_2];
_10 = Adt39::Variant1 { fld0: _6.3 };
_6.3 = Field::<u128>(Variant(_10, 1), 0);
_6.0.0 = _9.0;
SetDiscriminant(_10, 0);
_2 = false;
_1 = !_13;
_1 = _4 << _6.1;
place!(Field::<Adt38>(Variant(_10, 0), 5)).fld2 = [(-16_i8),(-6_i8)];
place!(Field::<char>(Variant(_10, 0), 1)) = _3;
place!(Field::<Adt38>(Variant(_10, 0), 5)).fld1 = [37293862_i32,1253947590_i32];
_10 = Adt39::Variant2 { fld0: _6 };
SetDiscriminant(_10, 1);
_1 = _6.1 | _4;
_3 = _8;
_14 = [_2,_2];
_17 = [_9.0,_6.0.0,_6.0.0];
RET = _6.3;
Goto(bb10)
}
bb10 = {
_3 = _8;
_6.1 = _6.0.1 as isize;
_6.1 = _13 >> _5;
_6.2 = [(-1128509035_i32),331905176_i32];
place!(Field::<u128>(Variant(_10, 1), 0)) = _2 as u128;
_20 = [(-112_i8),116_i8];
_19.0 = _9;
Goto(bb11)
}
bb11 = {
_19.0 = (_9.0, _9.1);
RET = _6.3;
_9.1 = _19.0.1 | _6.0.1;
Goto(bb12)
}
bb12 = {
_16 = 143_u8 as isize;
RET = Field::<u128>(Variant(_10, 1), 0);
_19.0.1 = _9.1;
_9.0 = _19.0.0 | _6.0.0;
_19.2 = _6.2;
SetDiscriminant(_10, 0);
place!(Field::<u128>(Variant(_10, 0), 4)) = !_6.3;
_23 = _19.2;
_18 = _17;
Goto(bb13)
}
bb13 = {
_14 = [_2,_2];
_19.4 = _6.4 << _4;
_19.1 = _1 * _13;
place!(Field::<u128>(Variant(_10, 0), 4)) = RET * RET;
place!(Field::<u8>(Variant(_10, 0), 0)) = 70_u8 + 42_u8;
_23 = _6.2;
_22 = 1795418673_i32 as f64;
_19.4 = !_6.4;
_6.1 = 15218616210847459468_usize as isize;
_6 = (_19.0, _1, _19.2, Field::<u128>(Variant(_10, 0), 4), _19.4);
_3 = _8;
place!(Field::<u128>(Variant(_10, 0), 4)) = RET;
_5 = !472901424819235063_u64;
_6.1 = _19.1;
_19.2 = [324936418_i32,(-2027779694_i32)];
_7 = Adt47::Variant2 { fld0: 3_usize,fld1: _22 };
place!(Field::<usize>(Variant(_7, 2), 0)) = 4_usize << _19.1;
_16 = _6.1 - _19.1;
_6.3 = RET;
_3 = _8;
_8 = _3;
RET = Field::<u128>(Variant(_10, 0), 4) & Field::<u128>(Variant(_10, 0), 4);
place!(Field::<u128>(Variant(_10, 0), 4)) = RET << Field::<usize>(Variant(_7, 2), 0);
Call(place!(Field::<Adt38>(Variant(_10, 0), 5)).fld0 = fn12(_1, Move(_7), _19.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_6.2 = _19.2;
_19.0.0 = _9.0;
_28 = _14;
_19.3 = !Field::<u128>(Variant(_10, 0), 4);
_24 = !7936138006722311727_usize;
_10 = Adt39::Variant2 { fld0: _19 };
_30 = !7_u8;
_6.2 = [(-32750561_i32),495594669_i32];
_5 = 715119689313715500_u64;
_6.2 = [77770232_i32,2054653294_i32];
_26 = [74_i8,(-79_i8)];
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_10, 2), 0)).2 = [1081438639_i32,(-2099607623_i32)];
_32.fld1 = [(-1742208085_i32),1820937590_i32];
_20 = _26;
_32.fld2 = [(-85_i8),125_i8];
_23 = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_10, 2), 0).2;
place!(Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_10, 2), 0)) = _6;
RET = Field::<((u16, i128), isize, [i32; 2], u128, i16)>(Variant(_10, 2), 0).4 as u128;
_16 = _19.4 as isize;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(9_usize, 4_usize, Move(_4), 9_usize, Move(_9), 3_usize, Move(_3), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(9_usize, 28_usize, Move(_28), 18_usize, Move(_18), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(9_usize, 6_usize, Move(_6), 30_usize, Move(_30), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: isize,mut _3: isize) -> u128 {
mir! {
type RET = u128;
let _4: [u128; 1];
let _5: [i32; 6];
let _6: u64;
let _7: ([i16; 6],);
let _8: ([i16; 6],);
let _9: [i32; 8];
let _10: i16;
let _11: [u32; 3];
let _12: u8;
let _13: *mut u32;
let _14: *const [u64; 6];
let _15: Adt47;
let _16: f64;
let _17: [i16; 6];
let _18: isize;
let _19: u64;
let _20: ();
let _21: ();
{
RET = 89013644130530674224400828815373029569_u128 * 68183646336666387427484663542035481421_u128;
_3 = RET as isize;
_2 = _3 >> _3;
RET = !463595136246209955747244822277226026_u128;
RET = 62155065963448478896440888849033721113_u128 + 144135138306642153742675326638117085373_u128;
_1 = _2;
_1 = -_3;
_1 = 17288432667779038389_usize as isize;
_1 = _2;
_3 = -_2;
_3 = _2;
RET = 122700980738939392366789615980719560257_u128 + 313832590570730566511785184868605494230_u128;
RET = 100228867155124276727010116439209008839_u128 + 21026422802041296920503417612105445109_u128;
_2 = (-12667_i16) as isize;
_2 = _1 | _1;
_5 = [(-1301859756_i32),(-22328188_i32),(-269152302_i32),(-710999983_i32),(-1120231740_i32),(-319551488_i32)];
_1 = _2;
RET = !30087368309726363656520125127858361562_u128;
_4 = [RET];
RET = 336062005474276272135326330804613274383_u128 >> _3;
RET = !169522025104222217258652373933153128189_u128;
RET = (-1159006664855595846_i64) as u128;
RET = 165989720056838937619691901868677808629_u128;
RET = 1375483272_i32 as u128;
_6 = 14164954026399191885_u64 - 16945485526497241661_u64;
Call(_3 = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [RET];
_7.0 = [(-25719_i16),16612_i16,(-17184_i16),(-9925_i16),32717_i16,24615_i16];
_2 = _3 & _3;
_6 = RET as u64;
RET = !276419391513762510383165685047158795538_u128;
_4 = [RET];
_4 = [RET];
_9 = [(-668745213_i32),(-217921589_i32),976420566_i32,(-1664483274_i32),(-527670315_i32),1943245781_i32,1537405826_i32,11625653_i32];
_7.0 = [12696_i16,24778_i16,(-28995_i16),(-615_i16),(-5454_i16),26945_i16];
_8.0 = [(-2611_i16),(-23701_i16),14683_i16,12470_i16,23113_i16,30086_i16];
_2 = _6 as isize;
Call(_3 = fn11(_8.0, _1, _5, _1, _1, _1, _1, _8, _8, _1, _7.0, _5, _7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 79243393290369638279462796236671243157_u128 & 38584625625505296322887988778660183363_u128;
_2 = RET as isize;
_2 = RET as isize;
_8.0 = _7.0;
Goto(bb3)
}
bb3 = {
_7.0 = [29582_i16,16674_i16,(-10189_i16),16294_i16,(-12418_i16),21743_i16];
_8 = (_7.0,);
_5 = [840374180_i32,(-732401600_i32),(-206882220_i32),(-1198390750_i32),(-1373108817_i32),492097113_i32];
_8.0 = [(-11702_i16),26218_i16,(-2763_i16),(-13804_i16),(-16179_i16),19051_i16];
Goto(bb4)
}
bb4 = {
_9 = [2023033226_i32,879249970_i32,(-1999100310_i32),1846098184_i32,(-330587854_i32),416845616_i32,(-1622438633_i32),1047646304_i32];
_8.0 = _7.0;
_3 = _2;
_3 = 4_usize as isize;
_12 = 121_u8;
match _12 {
0 => bb3,
1 => bb5,
2 => bb6,
121 => bb8,
_ => bb7
}
}
bb5 = {
_7.0 = [29582_i16,16674_i16,(-10189_i16),16294_i16,(-12418_i16),21743_i16];
_8 = (_7.0,);
_5 = [840374180_i32,(-732401600_i32),(-206882220_i32),(-1198390750_i32),(-1373108817_i32),492097113_i32];
_8.0 = [(-11702_i16),26218_i16,(-2763_i16),(-13804_i16),(-16179_i16),19051_i16];
Goto(bb4)
}
bb6 = {
RET = 79243393290369638279462796236671243157_u128 & 38584625625505296322887988778660183363_u128;
_2 = RET as isize;
_2 = RET as isize;
_8.0 = _7.0;
Goto(bb3)
}
bb7 = {
_4 = [RET];
_7.0 = [(-25719_i16),16612_i16,(-17184_i16),(-9925_i16),32717_i16,24615_i16];
_2 = _3 & _3;
_6 = RET as u64;
RET = !276419391513762510383165685047158795538_u128;
_4 = [RET];
_4 = [RET];
_9 = [(-668745213_i32),(-217921589_i32),976420566_i32,(-1664483274_i32),(-527670315_i32),1943245781_i32,1537405826_i32,11625653_i32];
_7.0 = [12696_i16,24778_i16,(-28995_i16),(-615_i16),(-5454_i16),26945_i16];
_8.0 = [(-2611_i16),(-23701_i16),14683_i16,12470_i16,23113_i16,30086_i16];
_2 = _6 as isize;
Call(_3 = fn11(_8.0, _1, _5, _1, _1, _1, _1, _8, _8, _1, _7.0, _5, _7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_2 = _1;
_7.0 = [3813_i16,(-29294_i16),(-18622_i16),22278_i16,30327_i16,(-29714_i16)];
_1 = _2 | _2;
_11 = [4001056391_u32,894963296_u32,457027618_u32];
match _12 {
0 => bb2,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
121 => bb14,
_ => bb13
}
}
bb9 = {
RET = 79243393290369638279462796236671243157_u128 & 38584625625505296322887988778660183363_u128;
_2 = RET as isize;
_2 = RET as isize;
_8.0 = _7.0;
Goto(bb3)
}
bb10 = {
RET = 79243393290369638279462796236671243157_u128 & 38584625625505296322887988778660183363_u128;
_2 = RET as isize;
_2 = RET as isize;
_8.0 = _7.0;
Goto(bb3)
}
bb11 = {
_7.0 = [29582_i16,16674_i16,(-10189_i16),16294_i16,(-12418_i16),21743_i16];
_8 = (_7.0,);
_5 = [840374180_i32,(-732401600_i32),(-206882220_i32),(-1198390750_i32),(-1373108817_i32),492097113_i32];
_8.0 = [(-11702_i16),26218_i16,(-2763_i16),(-13804_i16),(-16179_i16),19051_i16];
Goto(bb4)
}
bb12 = {
_9 = [2023033226_i32,879249970_i32,(-1999100310_i32),1846098184_i32,(-330587854_i32),416845616_i32,(-1622438633_i32),1047646304_i32];
_8.0 = _7.0;
_3 = _2;
_3 = 4_usize as isize;
_12 = 121_u8;
match _12 {
0 => bb3,
1 => bb5,
2 => bb6,
121 => bb8,
_ => bb7
}
}
bb13 = {
_7.0 = [29582_i16,16674_i16,(-10189_i16),16294_i16,(-12418_i16),21743_i16];
_8 = (_7.0,);
_5 = [840374180_i32,(-732401600_i32),(-206882220_i32),(-1198390750_i32),(-1373108817_i32),492097113_i32];
_8.0 = [(-11702_i16),26218_i16,(-2763_i16),(-13804_i16),(-16179_i16),19051_i16];
Goto(bb4)
}
bb14 = {
_7.0 = [9283_i16,(-28834_i16),(-19911_i16),(-28497_i16),(-2927_i16),2808_i16];
_4 = [RET];
_8.0 = _7.0;
_12 = 65_u8 * 70_u8;
_7.0 = [(-9742_i16),31026_i16,(-7269_i16),17565_i16,(-17698_i16),5336_i16];
_1 = _2;
RET = _2 as u128;
_1 = _2;
_16 = 3024651063560842818_i64 as f64;
_18 = !_2;
_8.0 = [(-20835_i16),7774_i16,(-6205_i16),(-21799_i16),(-3150_i16),4835_i16];
_3 = !_1;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(10_usize, 18_usize, Move(_18), 4_usize, Move(_4), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(10_usize, 6_usize, Move(_6), 12_usize, Move(_12), 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i16; 6],mut _2: isize,mut _3: [i32; 6],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: ([i16; 6],),mut _9: ([i16; 6],),mut _10: isize,mut _11: [i16; 6],mut _12: [i32; 6],mut _13: [i16; 6]) -> isize {
mir! {
type RET = isize;
let _14: (i16, ([i16; 6],));
let _15: f32;
let _16: Adt51;
let _17: ();
let _18: ();
{
RET = _7 & _7;
_8 = (_1,);
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(11_usize, 8_usize, Move(_8), 1_usize, Move(_1), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(11_usize, 4_usize, Move(_4), 3_usize, Move(_3), 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: Adt47,mut _3: isize) -> *const [u64; 6] {
mir! {
type RET = *const [u64; 6];
let _4: [u64; 6];
let _5: isize;
let _6: [u32; 3];
let _7: char;
let _8: u8;
let _9: Adt37;
let _10: [i64; 8];
let _11: [i32; 6];
let _12: [i32; 6];
let _13: u16;
let _14: i16;
let _15: u16;
let _16: (u16, i128);
let _17: Adt51;
let _18: isize;
let _19: *const [u64; 6];
let _20: ();
let _21: ();
{
_1 = 50144_u16 as isize;
_3 = Field::<usize>(Variant(_2, 2), 0) as isize;
Goto(bb1)
}
bb1 = {
place!(Field::<usize>(Variant(_2, 2), 0)) = !5_usize;
_1 = _3 & _3;
place!(Field::<usize>(Variant(_2, 2), 0)) = 12638892511046111585_usize >> _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 29_u8 as f64;
_3 = _1;
place!(Field::<usize>(Variant(_2, 2), 0)) = 3_usize;
place!(Field::<f64>(Variant(_2, 2), 1)) = Field::<usize>(Variant(_2, 2), 0) as f64;
_3 = _1 >> _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = 161932415829678908635623386548982060092_u128 as f64;
place!(Field::<f64>(Variant(_2, 2), 1)) = 121_i8 as f64;
_3 = Field::<f64>(Variant(_2, 2), 1) as isize;
_3 = _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = 149365280644998272221252461794244407498_i128 as f64;
_3 = _1 << _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = (-18821_i16) as f64;
_1 = true as isize;
Goto(bb2)
}
bb2 = {
place!(Field::<usize>(Variant(_2, 2), 0)) = !1_usize;
_3 = _1 | _1;
_4 = [4590132868912906636_u64,13861325848280028686_u64,16297850937417325439_u64,9222391123254133775_u64,6730263013186429820_u64,5735697454450074014_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = (-354451009_i32) as f64;
RET = core::ptr::addr_of!(_4);
SetDiscriminant(_2, 2);
RET = core::ptr::addr_of!((*RET));
Goto(bb3)
}
bb3 = {
(*RET) = [8146141846956222057_u64,16717510154955306537_u64,17046261857245419041_u64,2313973601136197904_u64,1743977946133685243_u64,14972740539155839595_u64];
(*RET) = [4305129847676281218_u64,16976245757579873706_u64,10393100316003021401_u64,16079699650483767942_u64,8384890916301726729_u64,229342403219208007_u64];
(*RET) = [12734815532785095070_u64,4942181660809592505_u64,714755602061285983_u64,14596452188723226544_u64,11286239517155289436_u64,5666496542801264753_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = 7334_i16 as f64;
(*RET) = [5691654398417631110_u64,5952161674275014231_u64,9507839920459209768_u64,4504712747051322655_u64,10114496797375738214_u64,6952203337679051666_u64];
RET = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!((*RET));
(*RET) = [4651013731896780797_u64,5382542673845604455_u64,11581019994104489836_u64,6076680023742965222_u64,14070433624378681663_u64,15937221150630880091_u64];
_1 = _3 - _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 278090484810580650446427803892714389507_u128 as f64;
(*RET) = [6837268618623518456_u64,17254437536698287067_u64,15208790306624877307_u64,6930382776069018914_u64,6589675813328870586_u64,4444589401747742762_u64];
(*RET) = [6638089966519605739_u64,17353997530130103264_u64,11197870207307044900_u64,1862237682606060330_u64,6468563876939298363_u64,12093506284821945902_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = 1493278274_i32 as f64;
place!(Field::<usize>(Variant(_2, 2), 0)) = 15230830316955082270_usize;
RET = core::ptr::addr_of!((*RET));
(*RET) = [3844426336302173007_u64,6587030610841690281_u64,7254028816878586949_u64,16764718092395248848_u64,15125333817080779043_u64,10982572800636251321_u64];
(*RET) = [9233893141684170806_u64,12662330027605368909_u64,14822809320602495840_u64,13402008529276242806_u64,17015383531801287396_u64,15607430031798669121_u64];
Goto(bb4)
}
bb4 = {
_4 = [3038334259162815548_u64,14809973375332239075_u64,17683891316144464236_u64,13407636479290102462_u64,16860353821995327327_u64,11705953520801828809_u64];
(*RET) = [3661470380669224338_u64,17892824246495765818_u64,11851976831786393718_u64,10551156297898564985_u64,6930283863339481125_u64,8906361519255653034_u64];
_3 = _1;
_3 = _1;
_7 = '\u{a26f2}';
_6 = [2084166067_u32,266748708_u32,4048840164_u32];
_6 = [1008924010_u32,1392644225_u32,1748370956_u32];
_4 = [17914314849736335111_u64,129070563749385649_u64,8994976243940090874_u64,9145039324243662763_u64,6599264851735803247_u64,15869907634701804111_u64];
_5 = _3 | _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 1242737352_u32 as f64;
(*RET) = [14649459650499090391_u64,9407929377910456616_u64,14809292113555148377_u64,1092991690099148783_u64,15728717037069511463_u64,14661669049629148524_u64];
_8 = 220_u8;
_3 = 12531_u16 as isize;
SetDiscriminant(_2, 3);
Goto(bb5)
}
bb5 = {
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).0 = !(-29701_i16);
place!(Field::<Adt44>(Variant(_2, 3), 1)) = Adt44::Variant0 { fld0: true,fld1: (-84382519993542675329531004249009669486_i128) };
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = -(-158871086702787147972470737085638547492_i128);
place!(Field::<*mut char>(Variant(_2, 3), 3)) = core::ptr::addr_of_mut!(_7);
_8 = 17_u8;
place!(Field::<*mut char>(Variant(_2, 3), 3)) = core::ptr::addr_of_mut!(_7);
Call(place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = fn13(_7, _5, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = (-37752022840266957293799749206478299934_i128) >> _5;
_4 = [4221614531876435856_u64,1723641790713702904_u64,5194636638286108024_u64,2111253171666321360_u64,11035152438574090978_u64,12180031493507860945_u64];
place!(Field::<[u16; 3]>(Variant(_2, 3), 4)) = [43168_u16,47220_u16,9701_u16];
RET = core::ptr::addr_of!(_4);
_10 = [(-727312265814305546_i64),2497012967180792084_i64,4581089448742915175_i64,(-5857609848935554442_i64),(-645115794174084217_i64),(-1577659494251502094_i64),7920921479341981433_i64,6505306866708914903_i64];
_11 = [1398613839_i32,(-1669318454_i32),(-1062734556_i32),(-1043805355_i32),(-1784047726_i32),1040005450_i32];
_8 = 39492_u16 as u8;
RET = core::ptr::addr_of!((*RET));
_4 = [4500590178919919283_u64,16066520113211901161_u64,17424927547206946709_u64,11753660236205160943_u64,10351114764897629699_u64,17468249626848846780_u64];
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0];
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = 108654700222351088702679485153236964900_i128 * 16737254972444161388681248920614276723_i128;
_10 = [(-3274810561798547956_i64),(-6247506485664323868_i64),2921408449363493155_i64,596183820316340352_i64,7119810151455870300_i64,(-1170601384083090116_i64),(-8187423545679027844_i64),(-2448958670417832832_i64)];
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0];
Call(place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)) = fn16(RET, _11, _5, _4, Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1), Field::<[u16; 3]>(Variant(_2, 3), 4), _5, _10, (*RET), _10, (*RET), _10, Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1), Field::<*mut char>(Variant(_2, 3), 3)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
place!(Field::<bool>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 0)) = Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0 == Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0;
place!(Field::<[u16; 3]>(Variant(_2, 3), 4)) = [29842_u16,21756_u16,43855_u16];
place!(Field::<bool>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 0)) = true;
(*RET) = [12205349779549933352_u64,8032258858525540469_u64,17402279348716161241_u64,13775575349228770138_u64,16626352334839001858_u64,16010497648890227456_u64];
_13 = _8 as u16;
_8 = !252_u8;
(*RET) = [4137788921717034729_u64,13830740434380057042_u64,9721209761856760890_u64,8401068493369858538_u64,10984882072252111063_u64,18276428697009882472_u64];
_1 = -_5;
_1 = _5;
(*RET) = [7027097512006834407_u64,9503810244507747628_u64,5055454223136262254_u64,9984045298964637599_u64,50897268602681744_u64,16118864422330157676_u64];
_7 = '\u{2c0d0}';
_16.0 = _13;
_16.1 = Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1) - Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1);
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0];
_6 = [2134111449_u32,2614577348_u32,2039702295_u32];
_11 = [(-1642216431_i32),1458311218_i32,(-1632266938_i32),(-169438243_i32),(-946563156_i32),2081070960_i32];
_14 = Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0;
_9 = Adt37::Variant1 { fld0: 7794707918135137137_usize,fld1: 1390083893658429269_u64 };
_12 = [(-177147505_i32),1513900375_i32,1212758990_i32,(-717160863_i32),1038189975_i32,(-1539687153_i32)];
_15 = !_16.0;
place!(Field::<*mut char>(Variant(_2, 3), 3)) = core::ptr::addr_of_mut!(_7);
SetDiscriminant(Field::<Adt44>(Variant(_2, 3), 1), 3);
place!(Field::<u64>(Variant(_9, 1), 1)) = 12850965598038337735_u64;
match Field::<u64>(Variant(_9, 1), 1) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
12850965598038337735 => bb15,
_ => bb14
}
}
bb8 = {
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = (-37752022840266957293799749206478299934_i128) >> _5;
_4 = [4221614531876435856_u64,1723641790713702904_u64,5194636638286108024_u64,2111253171666321360_u64,11035152438574090978_u64,12180031493507860945_u64];
place!(Field::<[u16; 3]>(Variant(_2, 3), 4)) = [43168_u16,47220_u16,9701_u16];
RET = core::ptr::addr_of!(_4);
_10 = [(-727312265814305546_i64),2497012967180792084_i64,4581089448742915175_i64,(-5857609848935554442_i64),(-645115794174084217_i64),(-1577659494251502094_i64),7920921479341981433_i64,6505306866708914903_i64];
_11 = [1398613839_i32,(-1669318454_i32),(-1062734556_i32),(-1043805355_i32),(-1784047726_i32),1040005450_i32];
_8 = 39492_u16 as u8;
RET = core::ptr::addr_of!((*RET));
_4 = [4500590178919919283_u64,16066520113211901161_u64,17424927547206946709_u64,11753660236205160943_u64,10351114764897629699_u64,17468249626848846780_u64];
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0];
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = 108654700222351088702679485153236964900_i128 * 16737254972444161388681248920614276723_i128;
_10 = [(-3274810561798547956_i64),(-6247506485664323868_i64),2921408449363493155_i64,596183820316340352_i64,7119810151455870300_i64,(-1170601384083090116_i64),(-8187423545679027844_i64),(-2448958670417832832_i64)];
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0,Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5).0];
Call(place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)) = fn16(RET, _11, _5, _4, Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1), Field::<[u16; 3]>(Variant(_2, 3), 4), _5, _10, (*RET), _10, (*RET), _10, Field::<i128>(Variant(Field::<Adt44>(Variant(_2, 3), 1), 0), 1), Field::<*mut char>(Variant(_2, 3), 3)), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
place!(Field::<(i16, ([i16; 6],))>(Variant(_2, 3), 5)).0 = !(-29701_i16);
place!(Field::<Adt44>(Variant(_2, 3), 1)) = Adt44::Variant0 { fld0: true,fld1: (-84382519993542675329531004249009669486_i128) };
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = -(-158871086702787147972470737085638547492_i128);
place!(Field::<*mut char>(Variant(_2, 3), 3)) = core::ptr::addr_of_mut!(_7);
_8 = 17_u8;
place!(Field::<*mut char>(Variant(_2, 3), 3)) = core::ptr::addr_of_mut!(_7);
Call(place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 0), 1)) = fn13(_7, _5, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_4 = [3038334259162815548_u64,14809973375332239075_u64,17683891316144464236_u64,13407636479290102462_u64,16860353821995327327_u64,11705953520801828809_u64];
(*RET) = [3661470380669224338_u64,17892824246495765818_u64,11851976831786393718_u64,10551156297898564985_u64,6930283863339481125_u64,8906361519255653034_u64];
_3 = _1;
_3 = _1;
_7 = '\u{a26f2}';
_6 = [2084166067_u32,266748708_u32,4048840164_u32];
_6 = [1008924010_u32,1392644225_u32,1748370956_u32];
_4 = [17914314849736335111_u64,129070563749385649_u64,8994976243940090874_u64,9145039324243662763_u64,6599264851735803247_u64,15869907634701804111_u64];
_5 = _3 | _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 1242737352_u32 as f64;
(*RET) = [14649459650499090391_u64,9407929377910456616_u64,14809292113555148377_u64,1092991690099148783_u64,15728717037069511463_u64,14661669049629148524_u64];
_8 = 220_u8;
_3 = 12531_u16 as isize;
SetDiscriminant(_2, 3);
Goto(bb5)
}
bb11 = {
(*RET) = [8146141846956222057_u64,16717510154955306537_u64,17046261857245419041_u64,2313973601136197904_u64,1743977946133685243_u64,14972740539155839595_u64];
(*RET) = [4305129847676281218_u64,16976245757579873706_u64,10393100316003021401_u64,16079699650483767942_u64,8384890916301726729_u64,229342403219208007_u64];
(*RET) = [12734815532785095070_u64,4942181660809592505_u64,714755602061285983_u64,14596452188723226544_u64,11286239517155289436_u64,5666496542801264753_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = 7334_i16 as f64;
(*RET) = [5691654398417631110_u64,5952161674275014231_u64,9507839920459209768_u64,4504712747051322655_u64,10114496797375738214_u64,6952203337679051666_u64];
RET = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!((*RET));
(*RET) = [4651013731896780797_u64,5382542673845604455_u64,11581019994104489836_u64,6076680023742965222_u64,14070433624378681663_u64,15937221150630880091_u64];
_1 = _3 - _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 278090484810580650446427803892714389507_u128 as f64;
(*RET) = [6837268618623518456_u64,17254437536698287067_u64,15208790306624877307_u64,6930382776069018914_u64,6589675813328870586_u64,4444589401747742762_u64];
(*RET) = [6638089966519605739_u64,17353997530130103264_u64,11197870207307044900_u64,1862237682606060330_u64,6468563876939298363_u64,12093506284821945902_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = 1493278274_i32 as f64;
place!(Field::<usize>(Variant(_2, 2), 0)) = 15230830316955082270_usize;
RET = core::ptr::addr_of!((*RET));
(*RET) = [3844426336302173007_u64,6587030610841690281_u64,7254028816878586949_u64,16764718092395248848_u64,15125333817080779043_u64,10982572800636251321_u64];
(*RET) = [9233893141684170806_u64,12662330027605368909_u64,14822809320602495840_u64,13402008529276242806_u64,17015383531801287396_u64,15607430031798669121_u64];
Goto(bb4)
}
bb12 = {
place!(Field::<usize>(Variant(_2, 2), 0)) = !1_usize;
_3 = _1 | _1;
_4 = [4590132868912906636_u64,13861325848280028686_u64,16297850937417325439_u64,9222391123254133775_u64,6730263013186429820_u64,5735697454450074014_u64];
place!(Field::<f64>(Variant(_2, 2), 1)) = (-354451009_i32) as f64;
RET = core::ptr::addr_of!(_4);
SetDiscriminant(_2, 2);
RET = core::ptr::addr_of!((*RET));
Goto(bb3)
}
bb13 = {
place!(Field::<usize>(Variant(_2, 2), 0)) = !5_usize;
_1 = _3 & _3;
place!(Field::<usize>(Variant(_2, 2), 0)) = 12638892511046111585_usize >> _3;
place!(Field::<f64>(Variant(_2, 2), 1)) = 29_u8 as f64;
_3 = _1;
place!(Field::<usize>(Variant(_2, 2), 0)) = 3_usize;
place!(Field::<f64>(Variant(_2, 2), 1)) = Field::<usize>(Variant(_2, 2), 0) as f64;
_3 = _1 >> _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = 161932415829678908635623386548982060092_u128 as f64;
place!(Field::<f64>(Variant(_2, 2), 1)) = 121_i8 as f64;
_3 = Field::<f64>(Variant(_2, 2), 1) as isize;
_3 = _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = 149365280644998272221252461794244407498_i128 as f64;
_3 = _1 << _1;
place!(Field::<f64>(Variant(_2, 2), 1)) = (-18821_i16) as f64;
_1 = true as isize;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<u64>(Variant(_9, 1), 1)) = _16.1 as u64;
_6 = [338739360_u32,1370881158_u32,3439633469_u32];
place!(Field::<i16>(Variant(place!(Field::<Adt44>(Variant(_2, 3), 1)), 3), 0)) = 52_i8 as i16;
_15 = !_13;
_18 = _8 as isize;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(12_usize, 1_usize, Move(_1), 14_usize, Move(_14), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(12_usize, 12_usize, Move(_12), 6_usize, Move(_6), 13_usize, Move(_13), 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: isize,mut _3: isize) -> i128 {
mir! {
type RET = i128;
let _4: bool;
let _5: i64;
let _6: ([i16; 6],);
let _7: [u16; 3];
let _8: usize;
let _9: i64;
let _10: (u16, i128);
let _11: char;
let _12: ([i16; 6],);
let _13: [i8; 2];
let _14: isize;
let _15: f64;
let _16: [i64; 8];
let _17: f64;
let _18: bool;
let _19: f64;
let _20: u8;
let _21: usize;
let _22: Adt45;
let _23: u32;
let _24: [i32; 8];
let _25: u16;
let _26: [u32; 3];
let _27: &'static u32;
let _28: bool;
let _29: bool;
let _30: ();
let _31: ();
{
_2 = !_3;
_2 = (-1750_i16) as isize;
RET = -93642983681438938697663644016446403018_i128;
_2 = 828089717_u32 as isize;
_2 = _3 & _3;
RET = 88328442254630643728089120824173135882_i128;
RET = 24165720617349158051647707342825114114_i128 - (-104987930149515989439411869287203221838_i128);
_3 = !_2;
_4 = !false;
Call(RET = core::intrinsics::bswap(132544011263114312589170224617039762834_i128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !_3;
_5 = 3461420317968803226_i64 << _2;
RET = 67911040724629115947215201628084671001_i128 - (-60516007438384119334083233382778355120_i128);
RET = 78928098539510382955518031461784382308_i128 << _5;
_1 = '\u{2824c}';
_5 = 1926279218_u32 as i64;
_2 = 70518503172157868_usize as isize;
RET = !(-136212752475603269679531154379652534678_i128);
_3 = _2 & _2;
Goto(bb2)
}
bb2 = {
_1 = '\u{c4849}';
RET = _5 as i128;
_6.0 = [19663_i16,17441_i16,15692_i16,(-66_i16),(-23739_i16),6553_i16];
_7 = [20319_u16,31886_u16,40108_u16];
_4 = false;
_3 = -_2;
_6.0 = [24817_i16,19964_i16,(-3622_i16),(-13885_i16),10000_i16,(-14743_i16)];
_3 = _2 * _2;
RET = 6807405737518798715204578509324840693_i128 + (-81221977847054319452384619165593848464_i128);
RET = 10777038010535951613948098386509817824_i128 | 66434516503809548383528735757733259934_i128;
_8 = !1_usize;
_7 = [37696_u16,36791_u16,24831_u16];
_7 = [19661_u16,45508_u16,25729_u16];
_3 = -_2;
_5 = 5284456871684630610_i64;
_8 = 1_usize + 12050226042915238277_usize;
_1 = '\u{4d6d4}';
RET = _3 as i128;
_9 = _5;
RET = 182149732418125474043374382131599143236_u128 as i128;
RET = (-120304018080988603408657129549036839977_i128);
_1 = '\u{142fe}';
RET = 115303022855264542343670820035917652967_i128;
_6.0 = [15110_i16,22363_i16,12453_i16,(-9171_i16),(-15058_i16),12702_i16];
Goto(bb3)
}
bb3 = {
_8 = _3 as usize;
_10.1 = !RET;
_9 = _4 as i64;
_10.0 = !20542_u16;
_10.1 = RET;
_8 = 5_usize;
_8 = 15142833599134609948_usize;
_8 = !2_usize;
_8 = 3_usize;
_6.0 = [14821_i16,(-24693_i16),(-22914_i16),3646_i16,(-13912_i16),(-508_i16)];
_10.0 = (-199421266_i32) as u16;
_11 = _1;
_10.1 = RET;
_8 = 9839917006018364631_usize + 8700840089247438580_usize;
_12.0 = _6.0;
Goto(bb4)
}
bb4 = {
_9 = 3895023407_u32 as i64;
RET = !_10.1;
_7 = [_10.0,_10.0,_10.0];
_3 = 8196802637961864507_u64 as isize;
_2 = _3;
_6 = (_12.0,);
_12.0 = [(-11722_i16),(-14081_i16),28858_i16,707_i16,1866_i16,1930_i16];
_2 = 4256849132_u32 as isize;
_5 = _9 ^ _9;
_6 = (_12.0,);
_4 = _8 < _8;
_6 = (_12.0,);
_8 = 5759145138045213075_usize;
RET = _2 as i128;
_12 = (_6.0,);
_3 = _2;
_4 = false;
_8 = 7236916610280608413_usize;
_6 = _12;
_13 = [126_i8,54_i8];
_12 = (_6.0,);
_6 = (_12.0,);
match _10.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
115303022855264542343670820035917652967 => bb9,
_ => bb8
}
}
bb5 = {
_8 = _3 as usize;
_10.1 = !RET;
_9 = _4 as i64;
_10.0 = !20542_u16;
_10.1 = RET;
_8 = 5_usize;
_8 = 15142833599134609948_usize;
_8 = !2_usize;
_8 = 3_usize;
_6.0 = [14821_i16,(-24693_i16),(-22914_i16),3646_i16,(-13912_i16),(-508_i16)];
_10.0 = (-199421266_i32) as u16;
_11 = _1;
_10.1 = RET;
_8 = 9839917006018364631_usize + 8700840089247438580_usize;
_12.0 = _6.0;
Goto(bb4)
}
bb6 = {
_1 = '\u{c4849}';
RET = _5 as i128;
_6.0 = [19663_i16,17441_i16,15692_i16,(-66_i16),(-23739_i16),6553_i16];
_7 = [20319_u16,31886_u16,40108_u16];
_4 = false;
_3 = -_2;
_6.0 = [24817_i16,19964_i16,(-3622_i16),(-13885_i16),10000_i16,(-14743_i16)];
_3 = _2 * _2;
RET = 6807405737518798715204578509324840693_i128 + (-81221977847054319452384619165593848464_i128);
RET = 10777038010535951613948098386509817824_i128 | 66434516503809548383528735757733259934_i128;
_8 = !1_usize;
_7 = [37696_u16,36791_u16,24831_u16];
_7 = [19661_u16,45508_u16,25729_u16];
_3 = -_2;
_5 = 5284456871684630610_i64;
_8 = 1_usize + 12050226042915238277_usize;
_1 = '\u{4d6d4}';
RET = _3 as i128;
_9 = _5;
RET = 182149732418125474043374382131599143236_u128 as i128;
RET = (-120304018080988603408657129549036839977_i128);
_1 = '\u{142fe}';
RET = 115303022855264542343670820035917652967_i128;
_6.0 = [15110_i16,22363_i16,12453_i16,(-9171_i16),(-15058_i16),12702_i16];
Goto(bb3)
}
bb7 = {
_2 = !_3;
_5 = 3461420317968803226_i64 << _2;
RET = 67911040724629115947215201628084671001_i128 - (-60516007438384119334083233382778355120_i128);
RET = 78928098539510382955518031461784382308_i128 << _5;
_1 = '\u{2824c}';
_5 = 1926279218_u32 as i64;
_2 = 70518503172157868_usize as isize;
RET = !(-136212752475603269679531154379652534678_i128);
_3 = _2 & _2;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_1 = _11;
_10.0 = 64675_u16;
_1 = _11;
_10.0 = 15940_u16 * 56898_u16;
_14 = _2;
_5 = _9 * _9;
_8 = !0_usize;
_12.0 = [12571_i16,6870_i16,(-24338_i16),6521_i16,23566_i16,(-7615_i16)];
_6 = (_12.0,);
_5 = _9;
_10.1 = RET;
_16 = [_9,_9,_5,_5,_9,_5,_5,_5];
_6.0 = [13888_i16,25381_i16,(-20548_i16),2225_i16,(-17397_i16),(-6177_i16)];
_14 = _2 ^ _2;
_10.1 = 979920093_i32 as i128;
_5 = _9;
_8 = 1_usize;
_15 = 1684979138074761145_u64 as f64;
_19 = _15;
_16 = [_9,_9,_9,_5,_9,_9,_5,_5];
_1 = _11;
_12.0[_8] = -_6.0[_8];
_20 = !154_u8;
_6.0 = _12.0;
_1 = _11;
_5 = _16[_8];
Goto(bb10)
}
bb10 = {
_8 = 3101552687362052559_usize;
_13 = [38_i8,38_i8];
_6 = _12;
_14 = _8 as isize;
_13 = [(-36_i8),(-62_i8)];
_6 = (_12.0,);
_10.0 = !45725_u16;
_20 = !237_u8;
_3 = _2;
_1 = _11;
_4 = !false;
_3 = _14 << _10.0;
_16 = [_9,_5,_5,_9,_5,_9,_5,_5];
_12 = (_6.0,);
_20 = _10.0 as u8;
_17 = 1437396021_u32 as f64;
_6.0 = _12.0;
Call(_11 = fn14(_7, _12.0, _12, _12, _6.0, _12.0, _10, _16, _3, _3, _6.0, _10.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = _19 as isize;
_22 = Adt45::Variant1 { fld0: _4 };
_10 = (2957_u16, RET);
Goto(bb12)
}
bb12 = {
RET = _10.1 + _10.1;
_17 = _15 * _19;
_6 = _12;
_6 = (_12.0,);
_20 = 198_u8;
_10.1 = 13312016948066499915_u64 as i128;
_25 = _10.0;
_16 = [_9,_9,_5,_5,_5,_9,_9,_9];
_10.0 = _25;
_14 = _3;
Goto(bb13)
}
bb13 = {
_2 = _14 + _14;
_12 = (_6.0,);
_18 = !Field::<bool>(Variant(_22, 1), 0);
_8 = 5_usize & 2985725899508354018_usize;
_3 = _2 >> _2;
_12.0 = [25440_i16,(-14350_i16),(-17379_i16),(-17382_i16),(-1484_i16),(-4787_i16)];
_17 = _19 * _15;
_2 = !_3;
SetDiscriminant(_22, 0);
_12.0 = [23441_i16,28079_i16,3497_i16,(-16002_i16),(-10943_i16),9087_i16];
_4 = _18;
_2 = !_3;
_6.0 = _12.0;
_5 = !_9;
place!(Field::<f64>(Variant(_22, 0), 0)) = -_17;
place!(Field::<i16>(Variant(_22, 0), 2)) = (-5041_i16) ^ 20791_i16;
match _25 {
0 => bb7,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
2957 => bb20,
_ => bb19
}
}
bb14 = {
_2 = !_3;
_5 = 3461420317968803226_i64 << _2;
RET = 67911040724629115947215201628084671001_i128 - (-60516007438384119334083233382778355120_i128);
RET = 78928098539510382955518031461784382308_i128 << _5;
_1 = '\u{2824c}';
_5 = 1926279218_u32 as i64;
_2 = 70518503172157868_usize as isize;
RET = !(-136212752475603269679531154379652534678_i128);
_3 = _2 & _2;
Goto(bb2)
}
bb15 = {
_14 = _19 as isize;
_22 = Adt45::Variant1 { fld0: _4 };
_10 = (2957_u16, RET);
Goto(bb12)
}
bb16 = {
_1 = '\u{c4849}';
RET = _5 as i128;
_6.0 = [19663_i16,17441_i16,15692_i16,(-66_i16),(-23739_i16),6553_i16];
_7 = [20319_u16,31886_u16,40108_u16];
_4 = false;
_3 = -_2;
_6.0 = [24817_i16,19964_i16,(-3622_i16),(-13885_i16),10000_i16,(-14743_i16)];
_3 = _2 * _2;
RET = 6807405737518798715204578509324840693_i128 + (-81221977847054319452384619165593848464_i128);
RET = 10777038010535951613948098386509817824_i128 | 66434516503809548383528735757733259934_i128;
_8 = !1_usize;
_7 = [37696_u16,36791_u16,24831_u16];
_7 = [19661_u16,45508_u16,25729_u16];
_3 = -_2;
_5 = 5284456871684630610_i64;
_8 = 1_usize + 12050226042915238277_usize;
_1 = '\u{4d6d4}';
RET = _3 as i128;
_9 = _5;
RET = 182149732418125474043374382131599143236_u128 as i128;
RET = (-120304018080988603408657129549036839977_i128);
_1 = '\u{142fe}';
RET = 115303022855264542343670820035917652967_i128;
_6.0 = [15110_i16,22363_i16,12453_i16,(-9171_i16),(-15058_i16),12702_i16];
Goto(bb3)
}
bb17 = {
_1 = _11;
_10.0 = 64675_u16;
_1 = _11;
_10.0 = 15940_u16 * 56898_u16;
_14 = _2;
_5 = _9 * _9;
_8 = !0_usize;
_12.0 = [12571_i16,6870_i16,(-24338_i16),6521_i16,23566_i16,(-7615_i16)];
_6 = (_12.0,);
_5 = _9;
_10.1 = RET;
_16 = [_9,_9,_5,_5,_9,_5,_5,_5];
_6.0 = [13888_i16,25381_i16,(-20548_i16),2225_i16,(-17397_i16),(-6177_i16)];
_14 = _2 ^ _2;
_10.1 = 979920093_i32 as i128;
_5 = _9;
_8 = 1_usize;
_15 = 1684979138074761145_u64 as f64;
_19 = _15;
_16 = [_9,_9,_9,_5,_9,_9,_5,_5];
_1 = _11;
_12.0[_8] = -_6.0[_8];
_20 = !154_u8;
_6.0 = _12.0;
_1 = _11;
_5 = _16[_8];
Goto(bb10)
}
bb18 = {
_1 = '\u{c4849}';
RET = _5 as i128;
_6.0 = [19663_i16,17441_i16,15692_i16,(-66_i16),(-23739_i16),6553_i16];
_7 = [20319_u16,31886_u16,40108_u16];
_4 = false;
_3 = -_2;
_6.0 = [24817_i16,19964_i16,(-3622_i16),(-13885_i16),10000_i16,(-14743_i16)];
_3 = _2 * _2;
RET = 6807405737518798715204578509324840693_i128 + (-81221977847054319452384619165593848464_i128);
RET = 10777038010535951613948098386509817824_i128 | 66434516503809548383528735757733259934_i128;
_8 = !1_usize;
_7 = [37696_u16,36791_u16,24831_u16];
_7 = [19661_u16,45508_u16,25729_u16];
_3 = -_2;
_5 = 5284456871684630610_i64;
_8 = 1_usize + 12050226042915238277_usize;
_1 = '\u{4d6d4}';
RET = _3 as i128;
_9 = _5;
RET = 182149732418125474043374382131599143236_u128 as i128;
RET = (-120304018080988603408657129549036839977_i128);
_1 = '\u{142fe}';
RET = 115303022855264542343670820035917652967_i128;
_6.0 = [15110_i16,22363_i16,12453_i16,(-9171_i16),(-15058_i16),12702_i16];
Goto(bb3)
}
bb19 = {
_8 = _3 as usize;
_10.1 = !RET;
_9 = _4 as i64;
_10.0 = !20542_u16;
_10.1 = RET;
_8 = 5_usize;
_8 = 15142833599134609948_usize;
_8 = !2_usize;
_8 = 3_usize;
_6.0 = [14821_i16,(-24693_i16),(-22914_i16),3646_i16,(-13912_i16),(-508_i16)];
_10.0 = (-199421266_i32) as u16;
_11 = _1;
_10.1 = RET;
_8 = 9839917006018364631_usize + 8700840089247438580_usize;
_12.0 = _6.0;
Goto(bb4)
}
bb20 = {
_10 = (_25, RET);
_27 = &_23;
RET = -_10.1;
_12 = (_6.0,);
_25 = _10.0 << _3;
_26 = [3794262085_u32,1527474887_u32,899166445_u32];
_19 = -_17;
Goto(bb21)
}
bb21 = {
Call(_30 = dump_var(13_usize, 11_usize, Move(_11), 16_usize, Move(_16), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_30 = dump_var(13_usize, 14_usize, Move(_14), 8_usize, Move(_8), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_30 = dump_var(13_usize, 25_usize, Move(_25), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [u16; 3],mut _2: [i16; 6],mut _3: ([i16; 6],),mut _4: ([i16; 6],),mut _5: [i16; 6],mut _6: [i16; 6],mut _7: (u16, i128),mut _8: [i64; 8],mut _9: isize,mut _10: isize,mut _11: [i16; 6],mut _12: u16) -> char {
mir! {
type RET = char;
let _13: (u16, i128);
let _14: Adt38;
let _15: Adt51;
let _16: Adt51;
let _17: isize;
let _18: Adt51;
let _19: [i32; 2];
let _20: [i32; 2];
let _21: [u16; 3];
let _22: isize;
let _23: isize;
let _24: bool;
let _25: Adt39;
let _26: (i16, ([i16; 6],));
let _27: f64;
let _28: [bool; 2];
let _29: u64;
let _30: f64;
let _31: [i8; 2];
let _32: *const [u64; 6];
let _33: isize;
let _34: [i8; 2];
let _35: *mut u32;
let _36: i8;
let _37: bool;
let _38: [i64; 8];
let _39: isize;
let _40: [i64; 8];
let _41: Adt40;
let _42: i128;
let _43: char;
let _44: [i64; 8];
let _45: u32;
let _46: [u16; 3];
let _47: Adt46;
let _48: bool;
let _49: [i8; 2];
let _50: i32;
let _51: u16;
let _52: (i16, ([i16; 6],));
let _53: Adt49;
let _54: ();
let _55: ();
{
_7.0 = _12 - _12;
_10 = _9 - _9;
_2 = _4.0;
_2 = _11;
_13.0 = 6651546571142104598_usize as u16;
_10 = (-86_i8) as isize;
_4 = _3;
RET = '\u{d149a}';
_1 = [_7.0,_12,_12];
_3.0 = [(-9118_i16),30338_i16,3402_i16,1862_i16,(-6455_i16),(-3462_i16)];
_13 = (_12, _7.1);
_7.0 = !_13.0;
RET = '\u{940fa}';
_7 = _13;
_10 = _9;
_14.fld1 = [(-1752394331_i32),(-1908763812_i32)];
_2 = _11;
_13.0 = _7.0 ^ _12;
_9 = 13804259923342582623_u64 as isize;
Goto(bb1)
}
bb1 = {
_4 = (_2,);
_7.0 = _12;
_13.0 = 2054884715_u32 as u16;
Goto(bb2)
}
bb2 = {
_5 = [15046_i16,(-2941_i16),(-9541_i16),22346_i16,(-11504_i16),26295_i16];
Call(_7 = fn15(_3.0, _2, _2, _4, _3.0, _6, _3.0, _6, _4, _3.0, _11, _3.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14.fld1 = [381273592_i32,2099500461_i32];
_12 = !_7.0;
_14.fld2 = [111_i8,45_i8];
_17 = RET as isize;
_6 = _11;
_1 = [_7.0,_13.0,_7.0];
_13.0 = _7.0 | _12;
_8 = [2239700923948703009_i64,608565749334393706_i64,602604460693826076_i64,(-7999426406543871797_i64),5243279642405487121_i64,8883801569915267305_i64,461343193536174153_i64,(-696296694006470391_i64)];
_23 = (-724763917_i32) as isize;
_13 = (_12, _7.1);
_7 = (_13.0, _13.1);
_24 = !false;
_14.fld2 = [90_i8,98_i8];
_12 = 7_usize as u16;
_26.1 = _4;
_4 = _26.1;
Goto(bb4)
}
bb4 = {
_12 = !_13.0;
_12 = 5_usize as u16;
_1 = [_13.0,_12,_7.0];
_26 = (15240_i16, _4);
_30 = 1631856762_u32 as f64;
_26.0 = -28290_i16;
_3.0 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_28 = [_24,_24];
_27 = -_30;
RET = '\u{c345e}';
_7 = (_13.0, _13.1);
_24 = false;
RET = '\u{6e1cc}';
_14.fld1 = [(-1310030937_i32),(-1217834411_i32)];
_26 = ((-25937_i16), _4);
_7.0 = _13.0;
_14.fld1 = [(-568455413_i32),(-1381446272_i32)];
_19 = _14.fld1;
match _26.0 {
0 => bb1,
340282366920938463463374607431768185519 => bb5,
_ => bb3
}
}
bb5 = {
_17 = _10;
_23 = !_9;
_17 = (-1176544982_i32) as isize;
_20 = [(-364762599_i32),1423679133_i32];
_4 = _26.1;
_33 = !_9;
_12 = _13.0;
_9 = 218_u8 as isize;
_28 = [_24,_24];
_29 = !2235621505498560701_u64;
_3.0 = _2;
_36 = (-678559589_i32) as i8;
match _26.0 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
340282366920938463463374607431768185519 => bb10,
_ => bb9
}
}
bb6 = {
_12 = !_13.0;
_12 = 5_usize as u16;
_1 = [_13.0,_12,_7.0];
_26 = (15240_i16, _4);
_30 = 1631856762_u32 as f64;
_26.0 = -28290_i16;
_3.0 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_28 = [_24,_24];
_27 = -_30;
RET = '\u{c345e}';
_7 = (_13.0, _13.1);
_24 = false;
RET = '\u{6e1cc}';
_14.fld1 = [(-1310030937_i32),(-1217834411_i32)];
_26 = ((-25937_i16), _4);
_7.0 = _13.0;
_14.fld1 = [(-568455413_i32),(-1381446272_i32)];
_19 = _14.fld1;
match _26.0 {
0 => bb1,
340282366920938463463374607431768185519 => bb5,
_ => bb3
}
}
bb7 = {
_14.fld1 = [381273592_i32,2099500461_i32];
_12 = !_7.0;
_14.fld2 = [111_i8,45_i8];
_17 = RET as isize;
_6 = _11;
_1 = [_7.0,_13.0,_7.0];
_13.0 = _7.0 | _12;
_8 = [2239700923948703009_i64,608565749334393706_i64,602604460693826076_i64,(-7999426406543871797_i64),5243279642405487121_i64,8883801569915267305_i64,461343193536174153_i64,(-696296694006470391_i64)];
_23 = (-724763917_i32) as isize;
_13 = (_12, _7.1);
_7 = (_13.0, _13.1);
_24 = !false;
_14.fld2 = [90_i8,98_i8];
_12 = 7_usize as u16;
_26.1 = _4;
_4 = _26.1;
Goto(bb4)
}
bb8 = {
_5 = [15046_i16,(-2941_i16),(-9541_i16),22346_i16,(-11504_i16),26295_i16];
Call(_7 = fn15(_3.0, _2, _2, _4, _3.0, _6, _3.0, _6, _4, _3.0, _11, _3.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_4 = (_2,);
_7.0 = _12;
_13.0 = 2054884715_u32 as u16;
Goto(bb2)
}
bb10 = {
_22 = !_17;
_7 = (_13.0, _13.1);
_22 = -_10;
_14.fld2 = [_36,_36];
_1 = [_12,_12,_13.0];
_25 = Adt39::Variant1 { fld0: 202833119987692104113427153364657409072_u128 };
_3.0 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_11 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_6 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_38 = _8;
Goto(bb11)
}
bb11 = {
_1 = [_13.0,_13.0,_7.0];
_13.0 = !_7.0;
RET = '\u{50d8d}';
place!(Field::<u128>(Variant(_25, 1), 0)) = 230320772200923385909906464093688458044_u128;
_2 = _11;
_39 = _22 >> _13.1;
_31 = _14.fld2;
_13.0 = _7.0 & _7.0;
_19 = _20;
_30 = _27 + _27;
_30 = 509664046_u32 as f64;
_23 = _22 * _39;
RET = '\u{1c93a}';
RET = '\u{c8746}';
_21 = [_13.0,_13.0,_13.0];
_2 = _4.0;
_19 = [(-372651238_i32),(-401532608_i32)];
_29 = !6355557094045843153_u64;
_30 = _27;
_5 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
SetDiscriminant(_25, 0);
Goto(bb12)
}
bb12 = {
_27 = -_30;
_24 = true;
_40 = _8;
_14.fld1 = _20;
place!(Field::<u128>(Variant(_25, 0), 4)) = 229242049343893926460224023289378555847_u128;
_12 = _13.0 - _13.0;
_26.1 = _3;
place!(Field::<char>(Variant(_25, 0), 1)) = RET;
_22 = _10;
_29 = 11015650383049166384_u64 * 8228903789398083419_u64;
_5 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_3.0 = _26.1.0;
_23 = _22;
_35 = core::ptr::addr_of_mut!(_45);
_17 = _39;
_13.0 = RET as u16;
_9 = !_17;
_39 = _22 - _17;
_13 = (_12, _7.1);
(*_35) = 1222681553_u32;
_2 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
place!(Field::<u128>(Variant(_25, 0), 4)) = Field::<char>(Variant(_25, 0), 1) as u128;
_4 = (_11,);
Goto(bb13)
}
bb13 = {
_7 = (_13.0, _13.1);
_34 = _14.fld2;
_7.0 = _26.0 as u16;
_23 = _39;
_7.0 = !_12;
_31 = _34;
_40 = [(-1428154700934185895_i64),(-5038379598503629917_i64),896125760843549340_i64,8677416184769076567_i64,(-2398039279469037683_i64),(-7015307988768890728_i64),(-3917100581055444544_i64),(-7134347397313694775_i64)];
_46 = [_12,_13.0,_12];
_24 = false | true;
_8 = [(-2513819203303039147_i64),(-5202630084875085276_i64),1093455404043064327_i64,(-679424273646087329_i64),3120601968762914324_i64,8558529729698431323_i64,8046639866294406298_i64,8105598807663630968_i64];
_31 = [_36,_36];
_26.1.0 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_39 = Field::<char>(Variant(_25, 0), 1) as isize;
_21 = [_13.0,_7.0,_12];
place!(Field::<*mut u32>(Variant(_25, 0), 2)) = _35;
_13.0 = !_7.0;
_44 = [(-2709077942872454300_i64),(-4362522969021394898_i64),(-4887268149499775143_i64),(-384299588699414398_i64),6648458751020482000_i64,(-2848408070731222079_i64),(-334301857335334093_i64),548845509715332076_i64];
_13 = (_7.0, _7.1);
Goto(bb14)
}
bb14 = {
_14.fld2 = [_36,_36];
_8 = _44;
_35 = core::ptr::addr_of_mut!((*_35));
_49 = _14.fld2;
_42 = _26.0 as i128;
_45 = _30 as u32;
_50 = 846600170_i32;
_13.0 = _12;
_5 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_22 = _23;
_8 = _44;
_24 = !false;
_8 = [6777540164922777025_i64,(-8588227698071891458_i64),2667826489790281847_i64,7134984943245061076_i64,7362154574732283047_i64,(-1879949090931710253_i64),(-2326229428912144252_i64),(-4596073294368846546_i64)];
place!(Field::<Adt38>(Variant(_25, 0), 5)).fld2 = [_36,_36];
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(14_usize, 5_usize, Move(_5), 12_usize, Move(_12), 21_usize, Move(_21), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(14_usize, 1_usize, Move(_1), 45_usize, Move(_45), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(14_usize, 26_usize, Move(_26), 39_usize, Move(_39), 46_usize, Move(_46), 38_usize, Move(_38)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(14_usize, 40_usize, Move(_40), 13_usize, Move(_13), 34_usize, Move(_34), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_54 = dump_var(14_usize, 24_usize, Move(_24), 36_usize, Move(_36), 55_usize, _55, 55_usize, _55), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [i16; 6],mut _2: [i16; 6],mut _3: [i16; 6],mut _4: ([i16; 6],),mut _5: [i16; 6],mut _6: [i16; 6],mut _7: [i16; 6],mut _8: [i16; 6],mut _9: ([i16; 6],),mut _10: [i16; 6],mut _11: [i16; 6],mut _12: [i16; 6]) -> (u16, i128) {
mir! {
type RET = (u16, i128);
let _13: [i32; 6];
let _14: f32;
let _15: &'static u32;
let _16: [u32; 3];
let _17: u128;
let _18: usize;
let _19: Adt45;
let _20: [u128; 1];
let _21: Adt42;
let _22: f32;
let _23: char;
let _24: usize;
let _25: [i8; 2];
let _26: f64;
let _27: f64;
let _28: f64;
let _29: *const [u64; 6];
let _30: Adt41;
let _31: ();
let _32: ();
{
_8 = _5;
_1 = [17021_i16,2313_i16,(-13686_i16),9432_i16,26916_i16,(-19806_i16)];
_5 = [(-18695_i16),28136_i16,(-904_i16),(-30488_i16),(-15678_i16),(-32497_i16)];
RET.0 = !48901_u16;
RET = (7812_u16, (-148497467141057670894659786407024574395_i128));
_2 = _6;
_5 = [21558_i16,(-17456_i16),(-12712_i16),(-13092_i16),28928_i16,(-23351_i16)];
RET = (12488_u16, 11130445450412621843153216639244248668_i128);
_5 = _10;
_8 = [(-11827_i16),10704_i16,29654_i16,22530_i16,25691_i16,8317_i16];
RET.0 = 26253_u16;
_5 = _3;
_1 = [5442_i16,26716_i16,9429_i16,(-16087_i16),12948_i16,(-573_i16)];
_12 = [18662_i16,29466_i16,(-8836_i16),(-19041_i16),(-4254_i16),(-24795_i16)];
Goto(bb1)
}
bb1 = {
_14 = (-91_i8) as f32;
_5 = [24207_i16,(-11460_i16),10870_i16,14695_i16,(-10881_i16),28689_i16];
RET.0 = 0_usize as u16;
_7 = [(-14384_i16),(-13367_i16),(-5204_i16),(-17535_i16),(-31910_i16),24226_i16];
_9.0 = [(-32227_i16),6156_i16,(-18816_i16),(-8598_i16),12950_i16,(-5674_i16)];
_14 = 3075248167_u32 as f32;
_9 = (_12,);
RET.0 = !61146_u16;
_16 = [2818688606_u32,3618595974_u32,534147910_u32];
RET = (36983_u16, 41333875290116924223955252665400455250_i128);
_13 = [1636014144_i32,1893324235_i32,(-904357899_i32),203577332_i32,(-332001959_i32),(-2011938123_i32)];
_13 = [49950440_i32,(-58871681_i32),(-1571317271_i32),(-740148284_i32),1371208987_i32,647584051_i32];
_2 = [26279_i16,(-13145_i16),(-9718_i16),18368_i16,(-10607_i16),27840_i16];
RET.1 = (-145924542445903479578401999390333907629_i128) << RET.0;
match RET.0 {
36983 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_3 = _11;
_8 = _9.0;
_1 = [10249_i16,15573_i16,(-29957_i16),(-5171_i16),1242_i16,(-21084_i16)];
_6 = [11316_i16,4359_i16,26534_i16,14488_i16,23483_i16,(-15142_i16)];
match RET.0 {
36983 => bb5,
_ => bb4
}
}
bb4 = {
_14 = (-91_i8) as f32;
_5 = [24207_i16,(-11460_i16),10870_i16,14695_i16,(-10881_i16),28689_i16];
RET.0 = 0_usize as u16;
_7 = [(-14384_i16),(-13367_i16),(-5204_i16),(-17535_i16),(-31910_i16),24226_i16];
_9.0 = [(-32227_i16),6156_i16,(-18816_i16),(-8598_i16),12950_i16,(-5674_i16)];
_14 = 3075248167_u32 as f32;
_9 = (_12,);
RET.0 = !61146_u16;
_16 = [2818688606_u32,3618595974_u32,534147910_u32];
RET = (36983_u16, 41333875290116924223955252665400455250_i128);
_13 = [1636014144_i32,1893324235_i32,(-904357899_i32),203577332_i32,(-332001959_i32),(-2011938123_i32)];
_13 = [49950440_i32,(-58871681_i32),(-1571317271_i32),(-740148284_i32),1371208987_i32,647584051_i32];
_2 = [26279_i16,(-13145_i16),(-9718_i16),18368_i16,(-10607_i16),27840_i16];
RET.1 = (-145924542445903479578401999390333907629_i128) << RET.0;
match RET.0 {
36983 => bb3,
_ => bb2
}
}
bb5 = {
_18 = '\u{a8d06}' as usize;
_4.0 = _8;
_8 = _1;
_18 = 17865734782342926510_usize ^ 13858962410086930324_usize;
_10 = _5;
RET = (52110_u16, 81314545083338167062592703013462179492_i128);
_9 = (_8,);
_11 = _7;
_18 = !3977066735641086208_usize;
_7 = [(-24195_i16),22634_i16,11931_i16,(-25795_i16),(-12653_i16),17032_i16];
RET = (64599_u16, (-56210153648339899877379816858075544835_i128));
RET.0 = 5269_u16 + 60834_u16;
RET = (4995_u16, 87056076600080103232777420258095520736_i128);
Goto(bb6)
}
bb6 = {
RET = (56023_u16, (-24148435570245038483261815091188303252_i128));
_6 = [(-2781_i16),23373_i16,(-11285_i16),(-5024_i16),(-29521_i16),15738_i16];
_7 = [11438_i16,31299_i16,(-9066_i16),(-13156_i16),(-7225_i16),30025_i16];
RET.0 = !20205_u16;
_8 = [(-5256_i16),(-19441_i16),(-30636_i16),15473_i16,17398_i16,5462_i16];
RET = (51246_u16, 57181484533485412978205982530069725611_i128);
RET = (31668_u16, (-102355842186780722269920760704243529734_i128));
_11 = [14748_i16,(-29043_i16),18848_i16,4887_i16,22576_i16,21471_i16];
_22 = -_14;
_6 = [(-27078_i16),(-16987_i16),10768_i16,15663_i16,17873_i16,(-708_i16)];
_17 = 270187464678821429282011034206471190902_u128 >> RET.1;
RET.0 = !50416_u16;
RET.0 = !16766_u16;
_3 = [13904_i16,24159_i16,4965_i16,(-5393_i16),2667_i16,24199_i16];
Goto(bb7)
}
bb7 = {
_12 = _9.0;
_6 = _3;
RET = (16545_u16, 98626389673981485652083726926132839904_i128);
_23 = '\u{5553c}';
_4 = (_12,);
_9 = (_7,);
_7 = _12;
RET.0 = 52956_u16;
_3 = [(-8478_i16),16746_i16,(-12722_i16),(-28589_i16),(-23849_i16),27081_i16];
RET.0 = _18 as u16;
RET.0 = !27063_u16;
_24 = !_18;
_25 = [(-103_i8),(-66_i8)];
_14 = -_22;
_24 = _18;
_26 = 155_u8 as f64;
_17 = 181845948626852412233921648938153016122_u128;
_12 = _8;
Goto(bb8)
}
bb8 = {
_9.0 = [22344_i16,(-2960_i16),(-4400_i16),(-21673_i16),18378_i16,22742_i16];
_12 = [13531_i16,(-26987_i16),(-24996_i16),(-30538_i16),11977_i16,(-13477_i16)];
_12 = [(-21826_i16),19275_i16,4481_i16,(-28652_i16),12336_i16,(-28582_i16)];
_4.0 = _12;
_11 = [23746_i16,(-7460_i16),(-5726_i16),17393_i16,613_i16,(-32609_i16)];
_5 = [31371_i16,12470_i16,12168_i16,(-19393_i16),13251_i16,(-5199_i16)];
_17 = 130379465381012151551653012986057147037_u128;
RET.1 = 62449273360148460502702822994165830328_i128;
_13 = [1972345538_i32,608277892_i32,(-1700453927_i32),319359611_i32,120475282_i32,1838591639_i32];
_6 = _4.0;
_17 = 186842786058155604087052843344669049103_u128;
RET.0 = 57394_u16 | 29876_u16;
_28 = _22 as f64;
_11 = [23797_i16,23336_i16,17247_i16,(-14225_i16),(-124_i16),4057_i16];
_1 = _5;
_6 = [(-19241_i16),7698_i16,(-8617_i16),(-13640_i16),1308_i16,(-3712_i16)];
_5 = _9.0;
RET.0 = !45194_u16;
_1 = [(-12965_i16),6943_i16,(-27518_i16),17141_i16,28553_i16,12573_i16];
_30.fld3 = 1518565270_i32 as i8;
match RET.1 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
62449273360148460502702822994165830328 => bb14,
_ => bb13
}
}
bb9 = {
_12 = _9.0;
_6 = _3;
RET = (16545_u16, 98626389673981485652083726926132839904_i128);
_23 = '\u{5553c}';
_4 = (_12,);
_9 = (_7,);
_7 = _12;
RET.0 = 52956_u16;
_3 = [(-8478_i16),16746_i16,(-12722_i16),(-28589_i16),(-23849_i16),27081_i16];
RET.0 = _18 as u16;
RET.0 = !27063_u16;
_24 = !_18;
_25 = [(-103_i8),(-66_i8)];
_14 = -_22;
_24 = _18;
_26 = 155_u8 as f64;
_17 = 181845948626852412233921648938153016122_u128;
_12 = _8;
Goto(bb8)
}
bb10 = {
RET = (56023_u16, (-24148435570245038483261815091188303252_i128));
_6 = [(-2781_i16),23373_i16,(-11285_i16),(-5024_i16),(-29521_i16),15738_i16];
_7 = [11438_i16,31299_i16,(-9066_i16),(-13156_i16),(-7225_i16),30025_i16];
RET.0 = !20205_u16;
_8 = [(-5256_i16),(-19441_i16),(-30636_i16),15473_i16,17398_i16,5462_i16];
RET = (51246_u16, 57181484533485412978205982530069725611_i128);
RET = (31668_u16, (-102355842186780722269920760704243529734_i128));
_11 = [14748_i16,(-29043_i16),18848_i16,4887_i16,22576_i16,21471_i16];
_22 = -_14;
_6 = [(-27078_i16),(-16987_i16),10768_i16,15663_i16,17873_i16,(-708_i16)];
_17 = 270187464678821429282011034206471190902_u128 >> RET.1;
RET.0 = !50416_u16;
RET.0 = !16766_u16;
_3 = [13904_i16,24159_i16,4965_i16,(-5393_i16),2667_i16,24199_i16];
Goto(bb7)
}
bb11 = {
_18 = '\u{a8d06}' as usize;
_4.0 = _8;
_8 = _1;
_18 = 17865734782342926510_usize ^ 13858962410086930324_usize;
_10 = _5;
RET = (52110_u16, 81314545083338167062592703013462179492_i128);
_9 = (_8,);
_11 = _7;
_18 = !3977066735641086208_usize;
_7 = [(-24195_i16),22634_i16,11931_i16,(-25795_i16),(-12653_i16),17032_i16];
RET = (64599_u16, (-56210153648339899877379816858075544835_i128));
RET.0 = 5269_u16 + 60834_u16;
RET = (4995_u16, 87056076600080103232777420258095520736_i128);
Goto(bb6)
}
bb12 = {
_14 = (-91_i8) as f32;
_5 = [24207_i16,(-11460_i16),10870_i16,14695_i16,(-10881_i16),28689_i16];
RET.0 = 0_usize as u16;
_7 = [(-14384_i16),(-13367_i16),(-5204_i16),(-17535_i16),(-31910_i16),24226_i16];
_9.0 = [(-32227_i16),6156_i16,(-18816_i16),(-8598_i16),12950_i16,(-5674_i16)];
_14 = 3075248167_u32 as f32;
_9 = (_12,);
RET.0 = !61146_u16;
_16 = [2818688606_u32,3618595974_u32,534147910_u32];
RET = (36983_u16, 41333875290116924223955252665400455250_i128);
_13 = [1636014144_i32,1893324235_i32,(-904357899_i32),203577332_i32,(-332001959_i32),(-2011938123_i32)];
_13 = [49950440_i32,(-58871681_i32),(-1571317271_i32),(-740148284_i32),1371208987_i32,647584051_i32];
_2 = [26279_i16,(-13145_i16),(-9718_i16),18368_i16,(-10607_i16),27840_i16];
RET.1 = (-145924542445903479578401999390333907629_i128) << RET.0;
match RET.0 {
36983 => bb3,
_ => bb2
}
}
bb13 = {
Return()
}
bb14 = {
_10 = [12558_i16,(-31781_i16),(-13095_i16),4367_i16,(-24378_i16),26140_i16];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(15_usize, 12_usize, Move(_12), 18_usize, Move(_18), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(15_usize, 7_usize, Move(_7), 3_usize, Move(_3), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(15_usize, 6_usize, Move(_6), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *const [u64; 6],mut _2: [i32; 6],mut _3: isize,mut _4: [u64; 6],mut _5: i128,mut _6: [u16; 3],mut _7: isize,mut _8: [i64; 8],mut _9: [u64; 6],mut _10: [i64; 8],mut _11: [u64; 6],mut _12: [i64; 8],mut _13: i128,mut _14: *mut char) -> (i16, ([i16; 6],)) {
mir! {
type RET = (i16, ([i16; 6],));
let _15: f32;
let _16: char;
let _17: isize;
let _18: char;
let _19: u64;
let _20: [i32; 2];
let _21: [u32; 3];
let _22: Adt43;
let _23: u64;
let _24: isize;
let _25: f64;
let _26: isize;
let _27: i16;
let _28: u8;
let _29: ();
let _30: ();
{
_5 = _13;
(*_14) = '\u{26b36}';
(*_14) = '\u{2e7ee}';
_7 = _3 * _3;
_11 = (*_1);
_2 = [(-769262308_i32),192267685_i32,(-1593220295_i32),679313673_i32,1348057980_i32,(-1777109026_i32)];
(*_1) = [12627379767506274705_u64,8057947494987908108_u64,7729607866600442377_u64,7071925340839839588_u64,10115804097915675268_u64,12165183213076594600_u64];
RET.0 = (-30923_i16);
(*_1) = [6757902892083366176_u64,5528209865710804338_u64,10055776505172205386_u64,6865989935563666257_u64,13193123321137667851_u64,9307845690735296622_u64];
RET.1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_16 = (*_14);
_5 = -_13;
_15 = (-1932897086_i32) as f32;
_7 = !_3;
_8 = _12;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768180533 => bb6,
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
_3 = _7;
_13 = (-2_i8) as i128;
_16 = (*_14);
(*_14) = _16;
_5 = _3 as i128;
_1 = core::ptr::addr_of!((*_1));
_5 = !_13;
_5 = _13;
RET.0 = (-29831_i16);
(*_14) = _16;
RET.1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
(*_1) = [6595428472698354017_u64,3871112142290898163_u64,14266900278479354755_u64,16889706868784247404_u64,13523162176069277606_u64,14812468268347797274_u64];
_17 = _3 >> _7;
_19 = !13497293373025742683_u64;
_11 = [_19,_19,_19,_19,_19,_19];
_17 = true as isize;
_21 = [2631095429_u32,2437920004_u32,341455840_u32];
_1 = core::ptr::addr_of!(_4);
_23 = _19 | _19;
RET.0 = 7437_i16 & 24776_i16;
_21 = [1249035141_u32,4258186377_u32,2493746217_u32];
_20 = [1055281562_i32,877496859_i32];
_20 = [2144042918_i32,502074483_i32];
RET.0 = !6771_i16;
_24 = !_3;
(*_1) = _9;
Call(_7 = core::intrinsics::transmute(_24), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = [_23,_23,_23,_23,_19,_19];
(*_1) = [_19,_19,_19,_19,_19,_19];
_18 = (*_14);
_20 = [(-1115901000_i32),(-1673247556_i32)];
_5 = 9555680132539905154_usize as i128;
_11 = [_23,_23,_19,_23,_19,_23];
_24 = _7 << _7;
(*_14) = _16;
RET.0 = -(-29607_i16);
_12 = [7264548777727505506_i64,(-4620557447493707560_i64),1585276536587392456_i64,(-1459176683075147421_i64),(-2658839960879196607_i64),5149335802136474549_i64,3964220612498052184_i64,8294282465103696900_i64];
_21 = [3343018471_u32,1204107919_u32,1080210873_u32];
_24 = _17 * _3;
_18 = (*_14);
(*_14) = _16;
_8 = [1321007328122616124_i64,438688911551072058_i64,(-3201191778515867019_i64),(-8490784186145091384_i64),7629549154534485254_i64,(-7851408858832728192_i64),(-2338425911025256417_i64),2422034022635810217_i64];
_20 = [1566118758_i32,1606875206_i32];
(*_14) = _16;
_2 = [(-738726729_i32),129707986_i32,(-323795605_i32),169035977_i32,(-1430205920_i32),(-2060803478_i32)];
_3 = _24;
_6 = [32936_u16,22494_u16,62962_u16];
RET.1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_15 = 8265449422891583075_i64 as f32;
_4 = [_19,_23,_23,_23,_19,_23];
_12 = [6048731444284529876_i64,9105300016694344552_i64,(-6982820320400697784_i64),3147417704206289780_i64,(-1548024892456243130_i64),3240622552495377864_i64,(-8926309102012530110_i64),7553453724465344294_i64];
(*_1) = [_19,_23,_23,_19,_23,_19];
_3 = _7 & _7;
_2 = [1722069645_i32,484089009_i32,1360556108_i32,(-1206985086_i32),367487198_i32,(-444273603_i32)];
_9 = [_23,_23,_19,_19,_19,_19];
_12 = [1988502385085241138_i64,1685239795984825017_i64,(-3570515078145109743_i64),(-3740491358205271082_i64),1766299864681449574_i64,7907520173989664294_i64,(-9029752508923833625_i64),8593574467106232422_i64];
Goto(bb8)
}
bb8 = {
_12 = [(-194961647757107863_i64),(-3760822247463470650_i64),1537248499122591187_i64,789037010491993301_i64,(-2681027970605491485_i64),472339441658176071_i64,(-802894509978540230_i64),7822311639861753387_i64];
RET.1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_6 = [31894_u16,29545_u16,16702_u16];
_9 = [_23,_19,_19,_19,_23,_23];
_26 = !_3;
_12 = [(-7488381076883101176_i64),9070887848721270035_i64,(-8508062368482806445_i64),5661289699233106839_i64,(-6126398889786659911_i64),7974464145484537106_i64,(-6076857472573031222_i64),2689608079080969576_i64];
_7 = _13 as isize;
_8 = [8969648571430645740_i64,7966208264044457891_i64,(-7667399501873361169_i64),4970984270864058109_i64,(-1014617030232629339_i64),1543288011710530031_i64,(-9204446482534997796_i64),7277216889245686006_i64];
RET.0 = (-30325_i16);
_10 = [5413332678518887066_i64,(-1443670871201315023_i64),(-9167102293906984164_i64),(-1546937644870380827_i64),6305352512931376666_i64,(-2481279943477012757_i64),1005308333779280737_i64,(-3250724964712850199_i64)];
_16 = _18;
_11 = (*_1);
_27 = -RET.0;
_4 = _11;
_3 = _17;
RET.0 = _27;
_3 = _26 + _17;
_21 = [3252497753_u32,2708438704_u32,1894581762_u32];
_11 = [_23,_23,_23,_23,_23,_19];
_13 = 126_i8 as i128;
_16 = (*_14);
_7 = 133_u8 as isize;
_18 = _16;
_13 = _5 - _5;
_26 = _3;
RET.0 = -_27;
Call(RET.1 = fn17(_26, (*_1), _3, (*_14), _6, _12, _24, _12, _24, _3, _26), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = (*_14);
_2 = [(-875750703_i32),970963949_i32,898583724_i32,(-695085655_i32),438295693_i32,(-2140466367_i32)];
_25 = 823882285_u32 as f64;
(*_1) = [_23,_23,_23,_23,_23,_23];
_27 = RET.0 * RET.0;
RET.1.0 = [_27,_27,_27,RET.0,RET.0,RET.0];
_20 = [843049017_i32,1066410126_i32];
_18 = _16;
_24 = _26 - _26;
RET.0 = _27 >> _3;
_14 = core::ptr::addr_of_mut!((*_14));
_20 = [580981622_i32,41987175_i32];
_12 = _8;
_6 = [14663_u16,60761_u16,55644_u16];
_10 = _8;
Goto(bb10)
}
bb10 = {
Call(_29 = dump_var(16_usize, 6_usize, Move(_6), 9_usize, Move(_9), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_29 = dump_var(16_usize, 16_usize, Move(_16), 27_usize, Move(_27), 26_usize, Move(_26), 4_usize, Move(_4)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_29 = dump_var(16_usize, 23_usize, Move(_23), 18_usize, Move(_18), 7_usize, Move(_7), 30_usize, _30), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: [u64; 6],mut _3: isize,mut _4: char,mut _5: [u16; 3],mut _6: [i64; 8],mut _7: isize,mut _8: [i64; 8],mut _9: isize,mut _10: isize,mut _11: isize) -> ([i16; 6],) {
mir! {
type RET = ([i16; 6],);
let _12: bool;
let _13: i64;
let _14: *mut char;
let _15: i32;
let _16: [i32; 2];
let _17: [bool; 2];
let _18: [i16; 6];
let _19: u128;
let _20: usize;
let _21: [u16; 3];
let _22: f32;
let _23: Adt37;
let _24: isize;
let _25: i8;
let _26: [u128; 1];
let _27: ();
let _28: ();
{
_3 = _7;
_9 = !_11;
RET.0 = [20193_i16,16727_i16,(-19678_i16),5559_i16,(-3915_i16),4097_i16];
_11 = _10;
RET.0 = [(-11370_i16),(-15182_i16),31134_i16,3925_i16,(-12756_i16),5060_i16];
_6 = [(-393450565825552199_i64),(-6033488360198551605_i64),(-8430715974900264431_i64),8889235537910657602_i64,7566483789680305758_i64,1483048173305561317_i64,8208887032775641905_i64,5252801698545229850_i64];
_10 = -_9;
_6 = [(-4038427284324074794_i64),3615479629725360343_i64,7975520071389801260_i64,6509239225718179517_i64,5927808896545315898_i64,4907469099311589994_i64,5227758843164777247_i64,6020570931394249130_i64];
_1 = !_9;
_12 = !true;
_3 = !_1;
_6 = _8;
_2 = [2014072659286709848_u64,475667216230263482_u64,8627835562450903589_u64,8444605086396924932_u64,3164571463249359980_u64,6227432961238430714_u64];
_7 = !_9;
_6 = _8;
RET.0 = [10062_i16,28787_i16,(-13471_i16),21080_i16,8581_i16,10091_i16];
_13 = !749824570655085939_i64;
_14 = core::ptr::addr_of_mut!(_4);
Call(_1 = fn18(_11, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = [(-31216_i16),8566_i16,914_i16,5725_i16,32008_i16,32225_i16];
_16 = [(-1673134960_i32),(-253913616_i32)];
_9 = _10;
_17 = [_12,_12];
_6 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = core::ptr::addr_of_mut!((*_14));
_3 = _11;
_9 = -_7;
_7 = (*_14) as isize;
Goto(bb2)
}
bb2 = {
_15 = 1155424846_i32 * (-603708920_i32);
_15 = 44_u8 as i32;
_16 = [_15,_15];
_2 = [17364889936685065294_u64,3866121154544250712_u64,16043382362738571094_u64,16536214499288083722_u64,6967669802560525767_u64,6489434464423678673_u64];
_18 = RET.0;
_18 = RET.0;
RET.0 = _18;
_7 = _1;
_15 = 1538765615_i32 ^ 1396712620_i32;
_2 = [15594596197689359337_u64,4413273739578086095_u64,16467575117512657238_u64,16209069840795632087_u64,15808075906673404377_u64,16141423898410611841_u64];
_19 = 285558203357478749909098242975397418462_u128;
_12 = true;
_15 = 1173149423_i32 - 1637133416_i32;
(*_14) = '\u{25ac5}';
_4 = '\u{107c47}';
match _19 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
285558203357478749909098242975397418462 => bb9,
_ => bb8
}
}
bb3 = {
RET.0 = [(-31216_i16),8566_i16,914_i16,5725_i16,32008_i16,32225_i16];
_16 = [(-1673134960_i32),(-253913616_i32)];
_9 = _10;
_17 = [_12,_12];
_6 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = core::ptr::addr_of_mut!((*_14));
_3 = _11;
_9 = -_7;
_7 = (*_14) as isize;
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
(*_14) = '\u{75c24}';
_5 = [13245_u16,38688_u16,60097_u16];
_13 = (-1701947727844802508_i64);
_17 = [_12,_12];
_2 = [11318290148423736300_u64,16828828809646090441_u64,15963528949409789015_u64,10351066754784747180_u64,17078021156458182335_u64,3022823717772855890_u64];
_10 = !_1;
RET.0 = [15063_i16,(-24001_i16),(-9007_i16),(-1965_i16),1524_i16,2892_i16];
_15 = (-1355495585_i32) & (-2043524578_i32);
_14 = core::ptr::addr_of_mut!(_4);
RET.0 = [(-13770_i16),29506_i16,(-20950_i16),(-13407_i16),(-17827_i16),(-1830_i16)];
_14 = core::ptr::addr_of_mut!((*_14));
_19 = 2768767760474919650564536834210514650_u128 * 334504746520739817960950449163782691340_u128;
_7 = _9 >> _1;
_18 = [12803_i16,(-17691_i16),(-19615_i16),29935_i16,(-1931_i16),10964_i16];
Goto(bb10)
}
bb10 = {
_19 = 301035448006529345769330282257582272940_u128;
_7 = 10096075491256774445_usize as isize;
_21 = [38495_u16,47678_u16,60475_u16];
_11 = _9 | _10;
_25 = 105_i8 & (-13_i8);
_23 = Adt37::Variant1 { fld0: 10300263379529357536_usize,fld1: 16819241827252650219_u64 };
_22 = 4_usize as f32;
_2 = [6403690865995687474_u64,875703632071312478_u64,18021225612477843922_u64,5454343805904362253_u64,15375739463028321717_u64,2632214367806178953_u64];
Goto(bb11)
}
bb11 = {
_25 = !67_i8;
RET = (_18,);
_20 = 9611842199067932640_usize;
_5 = [49671_u16,14065_u16,26877_u16];
_3 = _9;
_21 = [23799_u16,22816_u16,25694_u16];
_20 = 3_usize - 18063961473449485474_usize;
_26 = [_19];
_17 = [_12,_12];
_20 = 0_usize ^ 6_usize;
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
match _19 {
0 => bb4,
1 => bb3,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
301035448006529345769330282257582272940 => bb17,
_ => bb16
}
}
bb12 = {
_15 = 1155424846_i32 * (-603708920_i32);
_15 = 44_u8 as i32;
_16 = [_15,_15];
_2 = [17364889936685065294_u64,3866121154544250712_u64,16043382362738571094_u64,16536214499288083722_u64,6967669802560525767_u64,6489434464423678673_u64];
_18 = RET.0;
_18 = RET.0;
RET.0 = _18;
_7 = _1;
_15 = 1538765615_i32 ^ 1396712620_i32;
_2 = [15594596197689359337_u64,4413273739578086095_u64,16467575117512657238_u64,16209069840795632087_u64,15808075906673404377_u64,16141423898410611841_u64];
_19 = 285558203357478749909098242975397418462_u128;
_12 = true;
_15 = 1173149423_i32 - 1637133416_i32;
(*_14) = '\u{25ac5}';
_4 = '\u{107c47}';
match _19 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
285558203357478749909098242975397418462 => bb9,
_ => bb8
}
}
bb13 = {
RET.0 = [(-31216_i16),8566_i16,914_i16,5725_i16,32008_i16,32225_i16];
_16 = [(-1673134960_i32),(-253913616_i32)];
_9 = _10;
_17 = [_12,_12];
_6 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = core::ptr::addr_of_mut!((*_14));
_3 = _11;
_9 = -_7;
_7 = (*_14) as isize;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
RET.0 = [(-31216_i16),8566_i16,914_i16,5725_i16,32008_i16,32225_i16];
_16 = [(-1673134960_i32),(-253913616_i32)];
_9 = _10;
_17 = [_12,_12];
_6 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
_14 = core::ptr::addr_of_mut!((*_14));
_3 = _11;
_9 = -_7;
_7 = (*_14) as isize;
Goto(bb2)
}
bb17 = {
place!(Field::<usize>(Variant(_23, 1), 0)) = _20 | _20;
_2 = [4225533158079797381_u64,15064602135518749906_u64,6459848082276337235_u64,8716767916107917136_u64,12707145778765985405_u64,957925915870505596_u64];
_3 = 14852_u16 as isize;
_21 = _5;
RET.0 = [167_i16,(-29640_i16),29489_i16,32622_i16,(-27908_i16),(-945_i16)];
RET.0 = [(-30869_i16),6069_i16,(-16152_i16),5221_i16,(-15074_i16),(-24158_i16)];
_19 = 22077572569724415148119013589758465562_u128 - 281746895487663635436749003372216190160_u128;
_21 = [25231_u16,58273_u16,29211_u16];
_12 = !false;
RET.0 = _18;
_23 = Adt37::Variant1 { fld0: _20,fld1: 15997038520787030920_u64 };
_8 = [_13,_13,_13,_13,_13,_13,_13,_13];
_16 = [_15,_15];
_10 = _7 << _11;
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(17_usize, 5_usize, Move(_5), 25_usize, Move(_25), 8_usize, Move(_8), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(17_usize, 2_usize, Move(_2), 6_usize, Move(_6), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_27 = dump_var(17_usize, 12_usize, Move(_12), 21_usize, Move(_21), 10_usize, Move(_10), 28_usize, _28), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: Adt52;
let _4: i32;
let _5: i64;
let _6: ();
let _7: ();
{
_1 = (-6567_i16) as isize;
_5 = -(-5931704024275859659_i64);
Goto(bb1)
}
bb1 = {
RET = _2;
RET = _2;
RET = _2;
_3 = Adt52::Variant1 { fld0: 14858917829034250026_usize,fld1: 183_u8 };
_4 = true as i32;
Goto(bb2)
}
bb2 = {
Call(_6 = dump_var(18_usize, 1_usize, Move(_1), 5_usize, Move(_5), 7_usize, _7, 7_usize, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: bool,mut _2: ([i16; 6],),mut _3: [i16; 6],mut _4: bool,mut _5: [i16; 6],mut _6: char,mut _7: ([i16; 6],),mut _8: ([i16; 6],),mut _9: [u64; 6],mut _10: bool,mut _11: [u128; 1],mut _12: [u64; 6],mut _13: [i32; 8],mut _14: i64,mut _15: (i16, ([i16; 6],)),mut _16: i64) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _17: f64;
let _18: (i16, ([i16; 6],));
let _19: [i32; 6];
let _20: (i16, ([i16; 6],));
let _21: f32;
let _22: &'static u32;
let _23: i16;
let _24: *mut u32;
let _25: [i32; 6];
let _26: usize;
let _27: i8;
let _28: [bool; 2];
let _29: [i32; 6];
let _30: [u64; 6];
let _31: *mut u32;
let _32: (i16, ([i16; 6],));
let _33: ([i16; 6],);
let _34: Adt43;
let _35: char;
let _36: u128;
let _37: i8;
let _38: isize;
let _39: [i16; 6];
let _40: bool;
let _41: [u16; 3];
let _42: f32;
let _43: char;
let _44: i8;
let _45: (u16, i128);
let _46: Adt42;
let _47: [u64; 6];
let _48: Adt50;
let _49: (u16, i128);
let _50: Adt48;
let _51: ([i16; 6],);
let _52: char;
let _53: (u16, i128);
let _54: [i32; 2];
let _55: isize;
let _56: [u32; 3];
let _57: f64;
let _58: [u64; 6];
let _59: char;
let _60: i128;
let _61: [i16; 6];
let _62: ();
let _63: ();
{
_7 = (_3,);
_7 = (_8.0,);
_3 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
Goto(bb1)
}
bb1 = {
RET = _13;
_17 = 5_usize as f64;
_7 = (_8.0,);
_15.1 = _8;
_10 = !_1;
_8 = (_2.0,);
_2.0 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_5 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_6 = '\u{193c}';
_4 = _16 == _14;
RET = [2004412276_i32,(-2107659581_i32),1167166291_i32,(-58342817_i32),987811828_i32,567140125_i32,216333728_i32,(-2018776894_i32)];
_10 = _1 >= _4;
Goto(bb2)
}
bb2 = {
_18.1 = (_15.1.0,);
RET = [502275764_i32,(-691986363_i32),(-1236739004_i32),1156763064_i32,(-281329356_i32),1661237136_i32,(-330372402_i32),(-361075395_i32)];
_18 = (_15.0, _8);
_18.1 = (_7.0,);
_11 = [49804946059284463084359979025479303607_u128];
_19 = [(-1042876360_i32),(-431750730_i32),289375289_i32,1243200264_i32,1243224084_i32,134573083_i32];
_3 = [_15.0,_18.0,_18.0,_18.0,_15.0,_15.0];
RET = [902709478_i32,58672753_i32,(-1726270004_i32),1654491938_i32,(-2124748105_i32),1278368700_i32,908398250_i32,(-377871343_i32)];
_5 = _8.0;
_4 = _10;
_15.1.0 = [_18.0,_15.0,_15.0,_18.0,_18.0,_18.0];
_11 = [180455721714902388588734379177883110966_u128];
_18.0 = _15.0;
_5 = [_18.0,_18.0,_15.0,_18.0,_18.0,_15.0];
_1 = _10 < _10;
_5 = _7.0;
_7 = _8;
Goto(bb3)
}
bb3 = {
_6 = '\u{ab70a}';
_17 = 7_usize as f64;
_7 = (_8.0,);
_18 = (_15.0, _7);
_2.0 = [_15.0,_18.0,_18.0,_15.0,_15.0,_15.0];
_15.1.0 = [_15.0,_15.0,_15.0,_18.0,_15.0,_18.0];
_15.1 = (_7.0,);
_18.1.0 = [_15.0,_18.0,_18.0,_15.0,_15.0,_15.0];
_21 = 5474260947338826792_usize as f32;
_20 = (_15.0, _8);
Goto(bb4)
}
bb4 = {
_7 = (_20.1.0,);
_12 = [16530942409483927131_u64,5122493384232700946_u64,8447053248369035385_u64,16256379954392207075_u64,11369923621447151178_u64,3305878893862587379_u64];
_23 = _20.0 ^ _18.0;
_21 = (-99_isize) as f32;
_21 = _14 as f32;
_25 = _19;
_16 = _23 as i64;
_19 = [1186841723_i32,(-214776239_i32),1287253149_i32,(-617659480_i32),1714823203_i32,1222181120_i32];
_18.1.0 = [_15.0,_18.0,_18.0,_20.0,_15.0,_20.0];
Goto(bb5)
}
bb5 = {
_18.1.0 = [_18.0,_15.0,_20.0,_18.0,_20.0,_18.0];
RET = _13;
_18.1.0 = [_18.0,_15.0,_23,_18.0,_20.0,_15.0];
RET = [1992672656_i32,(-2024608628_i32),1408250369_i32,648279232_i32,(-399904504_i32),1407877815_i32,939085639_i32,2044276023_i32];
_4 = !_1;
_30 = [6070572707635137314_u64,10542452301347827286_u64,4894542806263362191_u64,5689474577466886714_u64,6276645759319935736_u64,6863330525496220682_u64];
_11 = [179299347574587136866157196537092977182_u128];
_26 = 6443_u16 as usize;
_1 = !_4;
_17 = _21 as f64;
_7 = (_15.1.0,);
_5 = _20.1.0;
_15.0 = !_20.0;
_20.0 = _23 + _23;
_14 = _10 as i64;
_2 = (_7.0,);
_7 = _18.1;
Goto(bb6)
}
bb6 = {
_2.0 = _8.0;
_18 = (_15.0, _2);
_32.0 = 10_u8 as i16;
_5 = [_23,_15.0,_20.0,_20.0,_18.0,_23];
_1 = _4 != _4;
_11 = [15732809913192068729025571020448948629_u128];
_32 = (_18.0, _20.1);
_1 = _4 > _4;
_4 = _1;
_17 = (-9223372036854775808_isize) as f64;
_3 = _18.1.0;
_2 = _15.1;
_15.0 = _20.0 + _20.0;
_18.0 = _20.0 >> _14;
_30 = [2534772110008293382_u64,2274392473063058608_u64,11984690712489723820_u64,11868084053532533972_u64,4475758028339358011_u64,1611801814844555414_u64];
_10 = !_1;
_8 = (_3,);
_14 = _16 & _16;
_8.0 = [_18.0,_18.0,_18.0,_18.0,_18.0,_15.0];
_30 = [10607013196638068564_u64,17650002650147562502_u64,3483281487029940174_u64,11958119073634330981_u64,11858847756891639153_u64,13822662595191735178_u64];
_4 = !_10;
_19 = _25;
_32 = (_15.0, _7);
Goto(bb7)
}
bb7 = {
_17 = 9223372036854775807_isize as f64;
_32 = _18;
_32.1 = (_2.0,);
_32.1.0 = [_18.0,_32.0,_15.0,_32.0,_15.0,_15.0];
_9 = [8852611180767079608_u64,17766578488861330091_u64,2392778724132284177_u64,14440078588347343560_u64,12005722572735113717_u64,6163550682551762701_u64];
_11 = [51388987113510669339248211515409272431_u128];
_32.1.0 = _8.0;
_18.1 = _15.1;
_32.0 = _18.0;
_7.0 = _8.0;
_26 = _17 as usize;
Goto(bb8)
}
bb8 = {
_21 = 133_u8 as f32;
_23 = _32.0;
_12 = [8787113406741750971_u64,15925204483388611094_u64,2513681644212724567_u64,8247180608694597985_u64,2494718312932412395_u64,16093129559920311880_u64];
_33.0 = [_18.0,_15.0,_18.0,_32.0,_18.0,_20.0];
_2.0 = _33.0;
_3 = [_18.0,_23,_23,_23,_18.0,_32.0];
Goto(bb9)
}
bb9 = {
_35 = _6;
_26 = 3370224613460256668_u64 as usize;
_1 = _4 >= _10;
_18.0 = _32.0 ^ _32.0;
_2.0 = _20.1.0;
_7.0 = [_18.0,_18.0,_32.0,_32.0,_20.0,_18.0];
_36 = 9223372036854775807_isize as u128;
_15.1.0 = [_18.0,_32.0,_18.0,_18.0,_15.0,_23];
_20.0 = _15.0 * _18.0;
_30 = _12;
_39 = _8.0;
_28 = [_4,_1];
_38 = 1092_u16 as isize;
_7.0 = _15.1.0;
_33.0 = [_32.0,_18.0,_23,_18.0,_18.0,_18.0];
_2.0 = _32.1.0;
_39 = [_18.0,_23,_18.0,_23,_18.0,_20.0];
_33.0 = _39;
_35 = _6;
_15.1 = _7;
_25 = [(-2143985585_i32),(-1273631374_i32),(-105955927_i32),339664319_i32,592807070_i32,(-955393907_i32)];
_15 = (_23, _8);
Goto(bb10)
}
bb10 = {
_15.1 = (_18.1.0,);
_27 = _26 as i8;
_19 = _25;
_41 = [5534_u16,19583_u16,64879_u16];
_15.1.0 = [_18.0,_20.0,_18.0,_20.0,_15.0,_18.0];
_29 = [(-668975443_i32),1322158747_i32,1392975111_i32,(-850757472_i32),(-109435239_i32),(-1306374122_i32)];
_44 = _27 + _27;
_28 = [_1,_10];
_3 = _15.1.0;
_8.0 = _2.0;
_14 = _16;
_11 = [_36];
_45.0 = 37238_u16 * 46905_u16;
_1 = !_4;
_26 = !1_usize;
_45.0 = !38563_u16;
Goto(bb11)
}
bb11 = {
_6 = _35;
_2 = _8;
_4 = _10 != _10;
RET = [(-520606851_i32),1806184404_i32,(-1218397680_i32),(-1897485399_i32),(-414163153_i32),(-1114742043_i32),(-73868248_i32),(-1556076392_i32)];
_10 = !_1;
_49.1 = !89737328945825078445292949147266836645_i128;
_9 = [10812142214940063456_u64,16913764781146536615_u64,12865514183580355925_u64,2495608846731462358_u64,15514854326408498192_u64,132794307200551973_u64];
Goto(bb12)
}
bb12 = {
_42 = _21;
_44 = _27 - _27;
_9 = [1589982199107187124_u64,9683225847906188606_u64,17477811497865905438_u64,4070939177248954230_u64,1263727951810469529_u64,10766375670586228746_u64];
_5 = _8.0;
_6 = _35;
_4 = _1;
_18.1.0 = [_18.0,_15.0,_20.0,_18.0,_32.0,_20.0];
_45.1 = 215040258_i32 as i128;
_44 = 3150670215_u32 as i8;
_9 = [1393104222594949692_u64,16803708362399233171_u64,8742315962964040991_u64,7703615556627052823_u64,7341279394693968151_u64,6616805293829865757_u64];
_53.0 = _18.0 as u16;
Goto(bb13)
}
bb13 = {
_34 = Adt43::Variant2 { fld0: _17,fld1: 4375037441005580999_u64,fld2: _18,fld3: RET };
place!(Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2)).0 = _32.0 & _18.0;
_49 = (_53.0, _45.1);
place!(Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2)) = (_32.0, _15.1);
_39 = [Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_18.0,_18.0,_23,_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0];
_19 = _25;
_47 = [252846179645260414_u64,9310556543100314690_u64,8328434066717995517_u64,3035071912779251352_u64,17596080337037552147_u64,15618157715368394996_u64];
_47 = _9;
_10 = !_1;
_45 = (_49.0, _49.1);
place!(Field::<u64>(Variant(_34, 2), 1)) = 13196243931009157032_u64 ^ 9895084186293331656_u64;
_19 = _25;
Goto(bb14)
}
bb14 = {
_45.1 = _49.1;
_47 = [Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1)];
_56 = [2826906689_u32,2808664306_u32,3696147173_u32];
_32.0 = _49.1 as i16;
place!(Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2)).1.0 = [Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_18.0,_20.0,_23];
_36 = !90695369882692884643241172172936088041_u128;
place!(Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2)) = _20;
_18.1.0 = [_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_18.0];
_30 = [Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1),Field::<u64>(Variant(_34, 2), 1)];
_60 = _49.1 << _45.0;
_44 = _27;
_5 = _20.1.0;
_15.1 = (_7.0,);
_27 = -_44;
_45.0 = _4 as u16;
_39 = [_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_20.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_18.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0];
_36 = 191_u8 as u128;
_18.1.0 = [_18.0,_15.0,_23,_18.0,Field::<(i16, ([i16; 6],))>(Variant(_34, 2), 2).0,_18.0];
SetDiscriminant(_34, 0);
_51.0 = _15.1.0;
_20.0 = 225_u8 as i16;
_43 = _6;
_26 = 15451447708423394024_usize;
_14 = -_16;
Goto(bb15)
}
bb15 = {
Call(_62 = dump_var(19_usize, 13_usize, Move(_13), 60_usize, Move(_60), 36_usize, Move(_36), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_62 = dump_var(19_usize, 16_usize, Move(_16), 23_usize, Move(_23), 19_usize, Move(_19), 41_usize, Move(_41)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_62 = dump_var(19_usize, 30_usize, Move(_30), 43_usize, Move(_43), 25_usize, Move(_25), 44_usize, Move(_44)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_62 = dump_var(19_usize, 26_usize, Move(_26), 7_usize, Move(_7), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_62 = dump_var(19_usize, 20_usize, Move(_20), 49_usize, Move(_49), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-4648773387971708532_i64)), std::hint::black_box(3311341160_u32));
                
            }
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt36::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: f64,

},
Variant1{
fld0: [i32; 2],
fld1: u128,
fld2: u8,
fld3: u64,
fld4: [i64; 8],
fld5: ((u16, i128), isize, [i32; 2], u128, i16),
fld6: *const [u64; 6],
fld7: [i32; 6],

},
Variant2{
fld0: *const [u64; 6],
fld1: char,
fld2: isize,
fld3: (i16, ([i16; 6],)),

},
Variant3{
fld0: i32,
fld1: *mut char,
fld2: f64,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt37::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: ((u16, i128), isize, [i32; 2], u128, i16),
fld1: [i16; 6],
fld2: u16,
fld3: [i64; 8],
fld4: usize,
fld5: i32,

},
Variant1{
fld0: usize,
fld1: u64,

},
Variant2{
fld0: bool,
fld1: [bool; 2],
fld2: u32,
fld3: f32,
fld4: *mut u32,
fld5: Adt36,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: *const [u64; 6],
fld1: [i32; 2],
fld2: [i8; 2],
}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: u8,
fld1: char,
fld2: *mut u32,
fld3: i128,
fld4: u128,
fld5: Adt38,

},
Variant1{
fld0: u128,

},
Variant2{
fld0: ((u16, i128), isize, [i32; 2], u128, i16),

},
Variant3{
fld0: f64,
fld1: u8,
fld2: (i16, ([i16; 6],)),
fld3: usize,
fld4: Adt37,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: f32,
fld1: u16,
fld2: [u16; 3],
fld3: i64,

},
Variant1{
fld0: i32,
fld1: [i32; 2],

},
Variant2{
fld0: [u64; 6],
fld1: Adt39,
fld2: f64,
fld3: u8,
fld4: [bool; 2],
fld5: i32,
fld6: i64,
fld7: i128,

},
Variant3{
fld0: *const [u64; 6],
fld1: [u16; 3],
fld2: [u32; 3],
fld3: [bool; 2],
fld4: i16,
fld5: (i16, ([i16; 6],)),
fld6: f64,
fld7: i128,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: [i64; 8],
fld1: i64,
fld2: *mut u32,
fld3: i8,
fld4: *const [u64; 6],
fld5: [i16; 6],
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [u128; 1],
fld1: char,
fld2: u16,
fld3: [i64; 8],
fld4: f64,
fld5: Adt37,
fld6: Adt38,
fld7: *mut u32,

},
Variant1{
fld0: usize,
fld1: [u16; 3],
fld2: *mut char,
fld3: i8,

},
Variant2{
fld0: ((u16, i128), isize, [i32; 2], u128, i16),
fld1: f32,
fld2: f64,
fld3: (u16, i128),
fld4: Adt39,
fld5: ([i16; 6],),

},
Variant3{
fld0: f64,
fld1: Adt40,
fld2: u16,
fld3: [i64; 8],
fld4: i16,
fld5: Adt41,
fld6: [u16; 3],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: f64,
fld1: [i16; 6],
fld2: [i32; 6],
fld3: ((u16, i128), isize, [i32; 2], u128, i16),
fld4: Adt40,
fld5: i32,
fld6: [u64; 6],
fld7: ([i16; 6],),

},
Variant1{
fld0: *const [u64; 6],
fld1: u32,
fld2: isize,
fld3: i8,
fld4: [bool; 2],
fld5: f64,
fld6: i64,
fld7: [u64; 6],

},
Variant2{
fld0: f64,
fld1: u64,
fld2: (i16, ([i16; 6],)),
fld3: [i32; 8],

},
Variant3{
fld0: *mut u32,
fld1: i64,
fld2: Adt42,
fld3: [u32; 3],
fld4: f32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: i128,

},
Variant1{
fld0: bool,
fld1: [i32; 6],
fld2: [i32; 2],
fld3: u8,
fld4: Adt36,
fld5: i32,
fld6: i64,

},
Variant2{
fld0: ((u16, i128), isize, [i32; 2], u128, i16),
fld1: *const [u64; 6],
fld2: [i32; 2],
fld3: [i32; 8],
fld4: i16,
fld5: i128,
fld6: u32,

},
Variant3{
fld0: i16,
fld1: [i32; 6],
fld2: Adt43,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: f64,
fld1: [i8; 2],
fld2: i16,
fld3: usize,

},
Variant1{
fld0: bool,

},
Variant2{
fld0: [u32; 3],
fld1: Adt38,
fld2: Adt43,
fld3: i8,
fld4: *mut u32,
fld5: [i32; 2],
fld6: ([i16; 6],),
fld7: [u16; 3],

},
Variant3{
fld0: Adt38,
fld1: Adt44,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [bool; 2],
fld1: Adt44,
fld2: ((u16, i128), isize, [i32; 2], u128, i16),
fld3: [i8; 2],
fld4: i16,
fld5: [i32; 6],

},
Variant1{
fld0: Adt41,
fld1: Adt39,
fld2: isize,
fld3: *mut char,
fld4: Adt40,

},
Variant2{
fld0: (u16, i128),
fld1: char,
fld2: *mut u32,
fld3: u64,
fld4: Adt43,
fld5: [i32; 2],
fld6: Adt45,

},
Variant3{
fld0: Adt45,
fld1: [i32; 6],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: ([i16; 6],),
fld1: [u32; 3],
fld2: Adt41,
fld3: [u16; 3],
fld4: usize,
fld5: i64,

},
Variant1{
fld0: bool,
fld1: [i32; 6],
fld2: Adt42,
fld3: u64,
fld4: Adt40,
fld5: Adt38,
fld6: u128,

},
Variant2{
fld0: usize,
fld1: f64,

},
Variant3{
fld0: u128,
fld1: Adt44,
fld2: Adt39,
fld3: *mut char,
fld4: [u16; 3],
fld5: (i16, ([i16; 6],)),
fld6: Adt36,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const [u64; 6],
fld1: u8,

},
Variant1{
fld0: [u16; 3],
fld1: Adt45,
fld2: Adt42,
fld3: ((u16, i128), isize, [i32; 2], u128, i16),
fld4: Adt39,
fld5: u16,
fld6: u8,

},
Variant2{
fld0: [i32; 6],

},
Variant3{
fld0: bool,
fld1: (i16, ([i16; 6],)),
fld2: Adt38,
fld3: i8,
fld4: [u64; 6],
fld5: i32,
fld6: Adt40,
fld7: i128,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: usize,
fld1: *const [u64; 6],
fld2: Adt38,
fld3: i8,
fld4: f64,

},
Variant1{
fld0: Adt41,

},
Variant2{
fld0: [i32; 8],
fld1: i8,

},
Variant3{
fld0: bool,
fld1: f32,
fld2: f64,
fld3: Adt44,
fld4: Adt48,
fld5: [u16; 3],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: usize,

},
Variant1{
fld0: (u16, i128),
fld1: Adt48,
fld2: Adt43,
fld3: Adt40,
fld4: Adt38,

},
Variant2{
fld0: *mut u32,
fld1: f32,
fld2: i32,

},
Variant3{
fld0: Adt40,
fld1: [i64; 8],
fld2: isize,
fld3: Adt48,
fld4: Adt46,
fld5: Adt41,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt37,
fld1: Adt50,
fld2: [i32; 6],
fld3: Adt42,

},
Variant1{
fld0: [i8; 2],
fld1: [u64; 6],
fld2: isize,

},
Variant2{
fld0: usize,
fld1: Adt48,
fld2: (i16, ([i16; 6],)),
fld3: i32,
fld4: i16,

},
Variant3{
fld0: [i16; 6],
fld1: Adt40,
fld2: Adt37,
fld3: (u16, i128),
fld4: i16,
fld5: u64,
fld6: Adt44,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt51,
fld1: char,
fld2: u32,
fld3: Adt48,
fld4: Adt44,
fld5: ([i16; 6],),

},
Variant1{
fld0: usize,
fld1: u8,

}}

