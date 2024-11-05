#![feature(custom_mir, core_intrinsics)]
use std::intrinsics::mir::*;
use std::ffi::{c_char, c_int};
extern "C" {
	fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug{
	unsafe fn printf_debug(&self);
}
impl PrintFDebug for i8{
	unsafe fn printf_debug(&self){
		printf(c"%i".as_ptr(),*self as i8 as c_int);
	}
}
impl PrintFDebug for u8{
	unsafe fn printf_debug(&self){
	}
} 
impl PrintFDebug for i16{
	unsafe fn printf_debug(&self){
	}
}
impl PrintFDebug for u16{
	unsafe fn printf_debug(&self){
		printf(c"%u".as_ptr(),*self as u16 as c_int);
	}
} 
impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
	unsafe fn printf_debug(&self){
		for b in self{
		}
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
	}
}
impl PrintFDebug for u64{
	unsafe fn printf_debug(&self){
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
	}
} 
impl PrintFDebug for bool{
	unsafe fn printf_debug(&self){
	}
} 
impl PrintFDebug for (){
	unsafe fn printf_debug(&self){
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
	}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> f32 {
mir! {
let _16: (bool, i16, i8);
let _18: Adt46;
{
Call(_18 = fn1(_1, _16, _12, _1, _3, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: (bool, i16, i8),mut _3: u32,mut _4: bool,mut _5: isize,mut _6: i8) -> Adt46 {
mir! {
let _8: (u64, f32);
let _11: Adt43;
{
Call(RET = fn2(Field::<isize>(Variant(_11, 1), 0), _2.1, _6, _2.1, _8.0, _2.1, Field::<isize>(Variant(_11, 1), 0), _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: i16,mut _3: i8,mut _4: i16,mut _5: u64,mut _6: i16,mut _7: isize,mut _8: (bool, i16, i8)) -> Adt46 {
mir! {
let _9: ((u128, (i128, i8, i8, u16)),);
let _10: (bool, i16, i8);
let _12: isize;
let _14: (bool, i16, i8);
let _15: ((u128, (i128, i8, i8, u16)),);
{
Goto(bb1)
}
bb1 = {
Call(RET = fn3(_12, _9.0.1.3, _10, _8, _14.0, _15.0.1.3, _10.0, _9.0.1.3, _10, _10, _14), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: u16,mut _3: (bool, i16, i8),mut _4: (bool, i16, i8),mut _5: bool,mut _6: u16,mut _7: bool,mut _8: u16,mut _9: (bool, i16, i8),mut _10: (bool, i16, i8),mut _11: (bool, i16, i8)) -> Adt46 {
mir! {
let _16: (i8, i16);
{
Call(_10.1 = fn4(_11.0, _16.1, _11.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: i16,mut _3: bool) -> i16 {
mir! {
let _5: f32;
let _7: Adt49;
let _8: ((u64, f32), u8, f32, [bool; 2]);
{
Call(_7 = fn6(_1, _8, _3, _3, _5, _8.3, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: ((u64, f32), u8, f32, [bool; 2]),mut _3: bool,mut _4: bool,mut _5: f32,mut _6: [bool; 2],mut _7: ((u64, f32), u8, f32, [bool; 2])) -> Adt49 {
mir! {
let _9: (u64, f32);
let _10: bool;
let _27: Adt49;
{
_9.0 = 869855573_u32 as u64;
Call(_27.fld0 = fn13(_2, _3, _9, _1, _10, _9.1, _2.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: ((u64, f32), u8, f32, [bool; 2]),mut _2: bool,mut _3: (u64, f32),mut _4: bool,mut _5: bool,mut _6: f32,mut _7: [bool; 2]) -> u64 {
mir! {
let _8: (bool, i16, i8);
let _9: u32;
let _10: ((isize,), u32);
let _11: (i8, i16);
let _12: i128;
let _13: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _15: isize;
let _16: ();
let _17: ();
{
_8.2 = (-12_i8) - 17_i8;
_5 = _4;
Call(_8.0 = fn14(_2, _1.2, _1, _5, _5, _5, _5, _4, _3, _5, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = (_3, 116_u8, _3.1, _7);
_10.0 = (9223372036854775807_isize,);
_11 = (_8.2, (-4523_i16));
_12 = 151139259821040071730677522447599823526_i128;
_13.0.1 = (_12, _11.0, _11.0, 57107_u16);
_13.3 = !_12;
_13.1 = (_10.0.0,);
_3.0 = !_1.0.0;
_13.0.1.1 = _3.0 as i8;
_13.2 = '\u{4c697}';
_13.0.1.2 = !_11.0;
_13.0.0 = 113893313057273688726654081396925279186_u128;
_13.0.1.3 = 19991_u16 & 22753_u16;
_9 = _13.0.0 as u32;
_10 = (_13.1, _9);
_15 = _13.1.0 | _10.0.0;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(13_usize, 15_usize, Move(_15), 10_usize, Move(_10), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_16 = dump_var(13_usize, 8_usize, Move(_8), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: f32,mut _3: ((u64, f32), u8, f32, [bool; 2]),mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: (u64, f32),mut _10: bool,mut _11: ((u64, f32), u8, f32, [bool; 2])) -> bool {
mir! {
let _13: ();
let _14: ();
{
_6 = _8 != _7;
RET = !_6;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(14_usize, 10_usize, Move(_10), 5_usize, Move(_5), 1_usize, Move(_1), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ((u64, f32), u8, f32, [bool; 2]),mut _2: Adt40,mut _3: f32,mut _4: ((u64, f32), u8, f32, [bool; 2])) -> Adt40 {
mir! {
{
Goto(bb1)
}
bb1 = {
Return()
}
}
}
pub fn main() {
			fn0(std::hint::black_box(false), std::hint::black_box('\u{589ea}'), std::hint::black_box((-15_isize)), std::hint::black_box((-88_i8)), std::hint::black_box(7435_i16), std::hint::black_box((-472870630_i32)), std::hint::black_box(7675076535321132781_i64), std::hint::black_box((-25529925778249855087585504095293304169_i128)), std::hint::black_box(9773957077793597678_usize), std::hint::black_box(96_u8), std::hint::black_box(32550_u16), std::hint::black_box(328278619_u32), std::hint::black_box(9100384109035215234_u64), std::hint::black_box(204224232034591802215192571262808178036_u128));
		}
impl PrintFDebug for Adt40{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant2{fld0,fld1,fld2,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (i128, i8, i8, u16),
},
Variant1{
fld0: i16,
fld1: *mut [u32; 5],
fld2: (u64, f32),
fld3: [i64; 7],
},
Variant2{
fld0: i16,
fld1: (i32, char, i8),
fld2: (isize,),
}}
impl PrintFDebug for Adt41{
unsafe fn printf_debug(&self){
unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
}
impl PrintFDebug for Adt42{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: u16,
fld2: u128,
fld3: (i8, i16),
},
Variant1{
fld0: i32,
fld1: *mut [u32; 5],
fld2: u32,
fld3: u16,
fld4: [bool; 2],
}}
impl PrintFDebug for Adt43{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,}=>{
},
Self::Variant1{fld0,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [bool; 8],
fld1: char,
fld2: i16,
},
Variant1{
fld0: isize,
}}
impl PrintFDebug for Adt44{
unsafe fn printf_debug(&self){
unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
}
impl PrintFDebug for Adt45{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant3{fld0,fld1,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i64,
fld1: (u64, f32),
fld2: [i16; 7],
fld3: u16,
},
Variant1{
fld0: Adt40,
fld1: i32,
fld2: [u32; 5],
fld3: [bool; 2],
fld4: u32,
},
Variant2{
fld0: (u128, (i128, i8, i8, u16)),
fld1: *mut u32,
fld2: isize,
fld3: Adt41,
},
Variant3{
fld0: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld1: i8,
}}
impl PrintFDebug for Adt46{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant3{fld0,fld1,fld2,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: *mut [u32; 5],
},
Variant1{
fld0: bool,
fld1: Adt40,
fld2: Adt41,
fld3: usize,
},
Variant2{
fld0: Adt41,
fld1: usize,
fld2: i16,
fld3: (u128, (i128, i8, i8, u16)),
},
Variant3{
fld0: i16,
fld1: [bool; 2],
fld2: f32,
}}
impl PrintFDebug for Adt47{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
},
Self::Variant3{fld0,fld1,fld2,fld3,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: ((isize,), u32),
fld2: i128,
fld3: usize,
fld4: f32,
fld5: u32,
},
Variant1{
fld0: ((isize,), u32),
fld1: Adt44,
fld2: [bool; 8],
fld3: (bool, i16, i8),
},
Variant2{
fld0: (isize,),
fld1: [bool; 8],
fld2: Adt40,
fld3: f64,
fld4: [i16; 7],
fld5: Adt42,
fld6: u64,
},
Variant3{
fld0: (i128, i8, i8, u16),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (u64, f32),
fld3: *mut [u32; 5],
}}
impl PrintFDebug for Adt48{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant1{fld0,}=>{
},
Self::Variant2{fld0,fld1,fld2,}=>{
},
Self::Variant3{fld0,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const u64,
fld1: (i128, i8, i8, u16),
fld2: u64,
fld3: i32,
},
Variant1{
fld0: (i8, i16),
},
Variant2{
fld0: [i16; 7],
fld1: *const u64,
fld2: u8,
},
Variant3{
fld0: (i8, i16),
}}
impl PrintFDebug for Adt49{
unsafe fn printf_debug(&self){
unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: u64,
}
impl PrintFDebug for Adt50{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
},
Self::Variant1{fld0,}=>{
},
Self::Variant2{fld0,}=>{
},
Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut [i64; 7],
fld1: f32,
fld2: ((isize,), u32),
fld3: i8,
fld4: u8,
fld5: Adt42,
fld6: i64,
fld7: [i64; 7],
},
Variant1{
fld0: bool,
},
Variant2{
fld0: [bool; 8],
},
Variant3{
fld0: Adt43,
fld1: ((u128, (i128, i8, i8, u16)),),
fld2: u64,
fld3: *const u64,
fld4: f32,
fld5: [i64; 7],
fld6: *mut [u32; 5],
}}
impl PrintFDebug for Adt51{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
},
Self::Variant1{fld0,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: (i8, i16),
fld2: (isize,),
fld3: Adt44,
fld4: ([bool; 8], i128, (isize,)),
fld5: (i128, i8, i8, u16),
},
Variant1{
fld0: Adt41,
},
Variant2{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (isize,),
fld3: i8,
fld4: *const u64,
fld5: (u128, (i128, i8, i8, u16)),
fld6: ([bool; 8], i128, (isize,)),
}}
impl PrintFDebug for Adt52{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (u128, (i128, i8, i8, u16)),
fld1: [i16; 7],
fld2: (isize,),
fld3: i8,
fld4: (i32, char, i8),
},
Variant1{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: Adt47,
fld2: (bool, i16, i8),
fld3: usize,
}}
impl PrintFDebug for Adt53{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
},
Self::Variant2{fld0,fld1,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u128,
fld1: i64,
fld2: isize,
fld3: Adt45,
},
Variant1{
fld0: usize,
fld1: *const u64,
fld2: u16,
fld3: ((u64, f32), u8, f32, [bool; 2]),
fld4: i16,
fld5: Adt46,
fld6: (isize,),
},
Variant2{
fld0: *mut u32,
fld1: ([bool; 8], i128, (isize,)),
}}
impl PrintFDebug for Adt54{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,}=>{
},
Self::Variant1{fld0,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt53,
},
Variant1{
fld0: char,
},
Variant2{
fld0: Adt53,
fld1: Adt43,
fld2: u16,
fld3: i8,
},
Variant3{
fld0: u128,
fld1: Adt48,
fld2: (u128, (i128, i8, i8, u16)),
fld3: Adt42,
fld4: Adt44,
fld5: ((isize,), u32),
}}
impl PrintFDebug for Adt55{
unsafe fn printf_debug(&self){
unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
}
impl PrintFDebug for Adt56{
unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
Self::Variant0{fld0,fld1,fld2,fld3,}=>{
},
Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
},
Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
},
Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
},
	}
}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: i128,
fld1: Adt40,
fld2: Adt49,
fld3: Adt51,
},
Variant1{
fld0: Adt44,
fld1: u16,
fld2: (u128, (i128, i8, i8, u16)),
fld3: i128,
fld4: Adt41,
},
Variant2{
fld0: ((isize,), u32),
fld1: char,
fld2: u128,
fld3: i8,
fld4: Adt47,
fld5: Adt44,
fld6: i64,
fld7: Adt42,
},
Variant3{
fld0: Adt49,
fld1: u16,
fld2: *const u64,
fld3: i8,
fld4: f32,
fld5: *mut [u32; 5],

}}

