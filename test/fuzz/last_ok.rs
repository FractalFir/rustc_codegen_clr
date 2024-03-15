#![feature(custom_mir, core_intrinsics, const_hash)]
use core::intrinsics::mir::*;
use std::ffi::{c_char, c_int};
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize) -> i32 {
mir! {
let _4: Adt32;
let _40: ();
{
    _4 = Adt32::Variant2 { fld0: 0 };
    _1 = (-110_isize) >> Field::<u8>(Variant(_4, 2), 0);
Call(_40 = dump_var(15_usize,1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}
}
}
extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug{
    unsafe fn printf_debug(&self);
}
impl<T:PrintFDebug> PrintFDebug for *const T{
    unsafe fn printf_debug(&self){
    }
}
impl PrintFDebug for isize{
    unsafe fn printf_debug(&self){
        printf("%li\0".as_ptr() as *const c_char,*self as isize);
    }
}
impl PrintFDebug for (){
    unsafe fn printf_debug(&self){
    }
} 
fn dump_var(
    f: usize,
    var2: usize, val2: impl PrintFDebug,
) {
    unsafe{
        printf("\n_%u = \0".as_ptr() as *const c_char,var2);
        val2.printf_debug();
    }
}
pub fn main() {
    fn15(std::hint::black_box(0));
        }
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld1: char,
fld5: [i64; 1],
},
Variant1{
},
Variant2{
fld0: u8,
}}



