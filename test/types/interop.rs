#![allow(internal_features,incomplete_features)]
#![no_std]
#![feature(start,lang_items,core_intrinsics,adt_const_params)]
include!("../common.rs");
#[allow(dead_code)]
struct RustcCLRInteropManagedClass<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>{
    prevent_construction:usize,
}
struct RustcCLRInteropManagedChar;
struct RustcCLRInteropManagedArray<T,const DIMENSIONS:usize>{
    pd:core::marker::PhantomData<T>,
    prevent_construction:usize,
}
impl From<char> for RustcCLRInteropManagedChar{
    fn from(c:char)->RustcCLRInteropManagedChar{
        extern "C"{
            fn managed_char_from_utf8(c:char)->RustcCLRInteropManagedChar;
        }
        unsafe{
            managed_char_from_utf8(c)
        }
    }
}
#[inline(never)]
fn rustc_clr_interop_managed_call0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const METHOD:&'static str,const IS_STATIC:bool,Ret>(){
    core::intrinsics::abort();
}
#[inline(never)]
fn rustc_clr_interop_managed_call1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1>(){
    core::intrinsics::abort();
}
#[inline(never)]
fn rustc_clr_interop_managed_call2_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1,Arg2>(){
    core::intrinsics::abort();
}
#[inline(never)]
fn rustc_clr_interop_managed_ctor0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>()->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
//struct RustcCLRInteropManagedBool;
fn main(){
    let chr:*mut RustcCLRInteropManagedChar = unsafe{core::ptr::null_mut()};
    black_box(chr);
    //let bl = black_box(RustcCLRInteropManagedBool);
    let object:*mut RustcCLRInteropManagedClass<"System.Runtime","System.Object"> = unsafe{core::ptr::null_mut()};
    black_box(object);
    let string:*mut RustcCLRInteropManagedClass<"System.Runtime","System.String"> = unsafe{core::ptr::null_mut()};
    black_box(string);
    let tpe:*mut RustcCLRInteropManagedClass<"System.Runtime","System.Type"> = unsafe{core::ptr::null_mut()};
    black_box(tpe);
    let arr:*mut RustcCLRInteropManagedArray<i32,1> = unsafe{core::ptr::null_mut()};
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,2> = unsafe{core::ptr::null_mut()};
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,3> = unsafe{core::ptr::null_mut()};
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,4> = unsafe{core::ptr::null_mut()};
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<RustcCLRInteropManagedClass<"System.Runtime","System.Object">,1> = unsafe{core::ptr::null_mut()};
    black_box(arr);
    test_ctors();
}
fn test_ctors(){
    let obj = black_box(rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.Object">());
    black_box(obj);
    let strig = rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.String">();
    black_box(strig);
}
