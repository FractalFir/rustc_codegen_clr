#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    improper_ctypes_definitions,
    improper_ctypes
)]
#![no_std]
#![feature(unboxed_closures)]
include!("../common.rs");
type Object = RustcCLRInteropManagedClass<"System.Runtime", "System.Object">;
type MString = RustcCLRInteropManagedClass<"System.Runtime", "System.String">;
//struct RustcCLRInteropVirtualMethodDef<const name:&'static str,const access:u8>{pd:core::marker::PhantomData<Func>}
struct RustcCLRInteropFieldDef<T, const ACCESS: u8, const IS_STATIC: bool> {
    pd: core::marker::PhantomData<T>,
}
struct RustcCLRInteropClassAccess<const ACCESS: u8> {}
//struct RustcCLRInteropCtorDef<Func,const access:u8>{pd:core::marker::PhantomData<Func>}
//struct RustcCLRInteropMethodDef<const name:&'static str,Func:Fn,const access:u8>{pd:core::marker::PhantomData<Func>}
const PRIVATE: u8 = 0;
const PROTECTED: u8 = 1;
const PUBLIC: u8 = 2;
trait Func {}
unsafe impl Sync for RustcCLRInteropManagedCustomTypeDef_CustomTypedef {}
#[allow(non_camel_case_types)]
struct RustcCLRInteropManagedCustomTypeDef_CustomTypedef {
    // Mandatory
    rustc_clr_interop_extends: Object,
    rustc_clr_interop_access: RustcCLRInteropClassAccess<PUBLIC>,
    //implements0:
    //rustc_clr_interop_virtual_def:RustcCLRInteropVirtualMethodDef<"ToString",fn()->MString,custom_to_string,PUBLIC>,
    //rustc_clr_interop_method_def:RustcCLRInteropMethodDef<"IsHappy",is_happy,PUBLIC>,
    // Needed, but not mandatory
    //rustc_clr_interop_ctor_def:RustcCLRInteropCtorDef<funny_ctor,PUBLIC>,
    // Normal fields
    happy: RustcCLRInteropFieldDef<bool, PUBLIC, true>,
    // Static fields
    total_happy: RustcCLRInteropFieldDef<usize, PUBLIC, true>,
}
/*
#[no_mangle]
pub extern fn tydef_custom_typedef(_:&RustcCLRInteropManagedCustomTypeDef_CustomTypedef){}*/
/*
fn is_happy()->MString{

}
fn funny_ctor()->MString{

}
fn custom_to_string()->MString{

}*/
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct RustcCLRInteropManagedClass<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> {
    prevent_construction: usize,
}
#[derive(Clone, Copy)]
struct RustcCLRInteropManagedChar {
    size: u16,
}
#[derive(Clone, Copy)]
struct RustcCLRInteropManagedArray<T, const DIMENSIONS: usize> {
    pd: core::marker::PhantomData<T>,
    prevent_construction: usize,
}
impl From<char> for RustcCLRInteropManagedChar {
    fn from(c: char) -> RustcCLRInteropManagedChar {
        extern "C" {
            fn managed_char_from_utf8(c: char) -> RustcCLRInteropManagedChar;
        }
        unsafe { managed_char_from_utf8(c) }
    }
}

#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor0_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
>() -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor1_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    Arg1,
>(
    arg1: Arg1,
) -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor2_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    Arg1,
    Arg2,
>(
    arg1: Arg1,
    arg2: Arg2,
) -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_ctor3_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    Arg1,
    Arg2,
    Arg3,
>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
) -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
    core::intrinsics::abort();
}
//struct RustcCLRInteropManagedBool;
fn main() {
    let chr: *mut RustcCLRInteropManagedChar = core::ptr::null_mut();
    black_box(chr);
    //let bl = black_box(RustcCLRInteropManagedBool);
    let object: *mut RustcCLRInteropManagedClass<"System.Runtime", "System.Object"> =
        core::ptr::null_mut();
    black_box(object);
    let string: *mut RustcCLRInteropManagedClass<"System.Runtime", "System.String"> =
        core::ptr::null_mut();
    black_box(string);
    let tpe: *mut RustcCLRInteropManagedClass<"System.Runtime", "System.Type"> =
        core::ptr::null_mut();
    black_box(tpe);
    //let arr:*mut RustcCLRInteropManagedArray<i32,1> = core::ptr::null_mut();
    //black_box(arr);
    //let arr:*mut RustcCLRInteropManagedArray<i32,2> = core::ptr::null_mut();
    //black_box(arr);
    //let arr:*mut RustcCLRInteropManagedArray<i32,3> = core::ptr::null_mut();
    //black_box(arr);
    //let arr:*mut RustcCLRInteropManagedArray<i32,4> = core::ptr::null_mut();
    //black_box(arr);
    //let arr:*mut RustcCLRInteropManagedArray<RustcCLRInteropManagedClass<"System.Runtime","System.Object">,1> = core::ptr::null_mut();
    //black_box(arr);
    //test_ctors();
    console_writeline();
    new_helloworld();
}
fn test_ctors() {
    let obj = black_box(rustc_clr_interop_managed_ctor0_::<
        "System.Runtime",
        "System.Object",
        false,
    >());
    black_box(obj);
    //let strig = rustc_clr_interop_managed_ctor0_::<"System.Runtime","System.String">();
    //black_box(strig);
    let sb =
        rustc_clr_interop_managed_ctor0_::<"System.Runtime", "System.Text.StringBuilder", false>();
    black_box(sb);

    let vec3 = rustc_clr_interop_managed_ctor3_::<
        "System.Numerics.Vectors",
        "System.Numerics.Vector3",
        true,
        f32,
        f32,
        f32,
    >(0.0, 1.5, 4.6);
    black_box(vec3);
}
type StringBuilder = RustcCLRInteropManagedClass<"System.Runtime", "System.Text.StringBuilder">;
fn console_writeline() {
    rustc_clr_interop_managed_call0_::<"System.Console", "System.Console", false, "WriteLine", ()>(
    );
    rustc_clr_interop_managed_call1_::<
        "System.Console",
        "System.Console",
        false,
        "WriteLine",
        true,
        (),
        i32,
    >(64);
}
fn new_helloworld() {
    let sb =
        rustc_clr_interop_managed_ctor0_::<"System.Runtime", "System.Text.StringBuilder", false>();
    black_box(sb);
    let m_char = unsafe { core::mem::transmute::<u16, RustcCLRInteropManagedChar>(black_box(64)) };
    rustc_clr_interop_managed_call2_::<
        "System.Runtime",
        "System.Text.StringBuilder",
        false,
        "Append",
        false,
        StringBuilder,
        StringBuilder,
        RustcCLRInteropManagedChar,
    >(sb, m_char);
}
