#![allow(internal_features,incomplete_features)]
#![no_std]
#![feature(start,lang_items,core_intrinsics,adt_const_params)]
include!("../common.rs");
#[allow(dead_code)]
struct RustcCLRInteropManagedClass<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>;
struct RustcCLRInteropManagedChar;
struct RustcCLRInteropManagedArray<T,const DIMENSIONS:usize>{
    pd:core::marker::PhantomData<T>,
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
}
