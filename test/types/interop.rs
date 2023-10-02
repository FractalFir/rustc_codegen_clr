#![allow(internal_features,incomplete_features)]
#![no_std]
#![feature(start,lang_items,core_intrinsics,adt_const_params)]
include!("../common.rs");
#[allow(dead_code)]
struct RustcCLRInteropManagedClass<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>;
struct RustcCLRInteropManagedChar;
impl From<char> for RustcCLRInteropManagedChar{
    
}
//struct RustcCLRInteropManagedBool;
fn main(){
    let chr:*mut RustcCLRInteropManagedChar = unsafe{core::ptr::null_mut()};
    black_box(chr);
    //let bl = black_box(RustcCLRInteropManagedBool);
    //let y:RustcCLRInteropManagedClass<"System.Runtime","System.Object"> = RustcCLRInteropManagedClass{};
    //black_box(y);
}
