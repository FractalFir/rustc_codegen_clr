#![allow(internal_features,incomplete_features,improper_ctypes,dead_code)]
#![no_std]
#![feature(start,lang_items,core_intrinsics,adt_const_params)]
include!("../common.rs");
#[allow(dead_code)]
#[derive(Clone,Copy)]
struct RustcCLRInteropManagedClass<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>{
    prevent_construction:usize,
}
#[derive(Clone,Copy)]
struct RustcCLRInteropManagedChar{
    size:u16,
}
#[derive(Clone,Copy)]
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

#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool>()->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1>(arg1:Arg1)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor2_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1,Arg2>(arg1:Arg1,arg2:Arg2)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor3_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1,Arg2,Arg3>(arg1:Arg1,arg2:Arg2,arg3:Arg3)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
//struct RustcCLRInteropManagedBool;
fn main(){
    let chr:*mut RustcCLRInteropManagedChar = unsafe{core::ptr::null_mut()};
    black_box(chr);
    //let bl = black_box(RustcCLRInteropManagedBool);
    let object:*mut RustcCLRInteropManagedClass<"System.Runtime","System.Object"> = core::ptr::null_mut();
    black_box(object);
    let string:*mut RustcCLRInteropManagedClass<"System.Runtime","System.String"> = core::ptr::null_mut();
    black_box(string);
    let tpe:*mut RustcCLRInteropManagedClass<"System.Runtime","System.Type"> = core::ptr::null_mut();
    black_box(tpe);
    let arr:*mut RustcCLRInteropManagedArray<i32,1> = core::ptr::null_mut();
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,2> = core::ptr::null_mut();
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,3> = core::ptr::null_mut();
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<i32,4> = core::ptr::null_mut();
    black_box(arr);
    let arr:*mut RustcCLRInteropManagedArray<RustcCLRInteropManagedClass<"System.Runtime","System.Object">,1> = core::ptr::null_mut();
    black_box(arr);
    //test_ctors();
    console_writeline();
    new_helloworld();
}
fn test_ctors(){
    let obj = black_box(rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.Object",false>());
    black_box(obj);
    //let strig = rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.String">();
    //black_box(strig);
    let sb = rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.Text.StringBuilder",false>();
    black_box(sb);

    let vec3 = rustc_clr_interop_managed_ctor3_::<"System.Numerics.Vectors","System.Numerics.Vector3",true,f32,f32,f32>(0.0,1.5,4.6);
    black_box(vec3);
}
type StringBuilder = RustcCLRInteropManagedClass<"System.Runtime","System.Text.StringBuilder">;
fn console_writeline(){
    rustc_clr_interop_managed_call0_::<"System.Console","System.Console",false,"WriteLine",()>();
    rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),i32>(64);
}
fn new_helloworld(){
    let sb = rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.Text.StringBuilder",false>();
    black_box(sb);
    let m_char = unsafe{core::mem::transmute::<u16,RustcCLRInteropManagedChar>(black_box(64))}; 
    rustc_clr_interop_managed_call2_::<"System.Runtime","System.Text.StringBuilder",false,"Append",false,StringBuilder,StringBuilder,RustcCLRInteropManagedChar>(sb,m_char);
}
