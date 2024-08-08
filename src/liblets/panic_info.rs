#![allow(internal_features, incomplete_features)]
#![feature(thread_local, unsized_const_params, core_intrinsics, lang_items)]
///! This is a low-level utilility used for converting .NET exceptions to panic payloads.
///! It uses unsafe code, and it's correctness is crucial.
///! The utility works by converting an exception to a PanicInfo, panicking using that info, and catching that panic payload using compiler intrinsics.
///! This utinily then returns the payload it obtained.
extern crate core;
use core::sync::atomic::AtomicPtr;
// Used to free the payload
extern "C" {
    fn free(ptr: *mut u8);
}
/// Stores the last message thrown. Used to ensure the message is freed after the next panic is thown.
#[thread_local]
static LAST_MESSAGE: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
/// Used to store a pointer to the last message. Neccessary to build a panic info with a static lifetime.
#[thread_local]
static PAYLOAD: core::cell::Cell<&'static str> = core::cell::Cell::new(&"");
/// Marks this string as an exception message. It will get freed the next time an exception is thrown.
pub fn exception_str(payload: *mut u8) {
    let payload = LAST_MESSAGE.swap(payload, std::sync::atomic::Ordering::Relaxed);
    if !payload.is_null() {
        unsafe { free(payload) };
    }
}
/// Interop utility. Calls an instance method, is "magic" and processed by the codegen.
#[inline(never)]
#[allow(unused_variables)]
fn rustc_clr_interop_managed_call1_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    const IS_STATIC: bool,
    Ret,
    Arg1,
>(
    arg1: Arg1,
) -> Ret {
    // The body of this function is fake and will be replaced with a proper interop call.
    core::intrinsics::abort();
}
/// Interop utility. Calls a virtual method method, is "magic" and processed by the codegen.
#[inline(never)]
#[allow(unused_variables)]
fn rustc_clr_interop_managed_call_virt1_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    const IS_STATIC: bool,
    Ret,
    Arg1,
>(
    arg1: Arg1,
) -> Ret {
    // The body of this function is fake and will be replaced with a proper interop call.
    core::intrinsics::abort();
}
/// Interop utliit. Represents a managed class type. May only be stored on the stack, and no pointer to it shall be taken.
#[derive(Clone, Copy)]
pub struct RustcCLRInteropManagedClass<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str>
{
    #[allow(dead_code)]
    prevent_construction: usize,
}
// Type definitions
/// .NET object
pub type Object = RustcCLRInteropManagedClass<"System.Runtime", "System.Object">;
/// .NET exception
pub type Exception = RustcCLRInteropManagedClass<"System.Runtime", "System.Exception">;
/// .NET string
pub type MString = RustcCLRInteropManagedClass<"System.Runtime", "System.String">;
/// A fake version of `PanicInfo`, used to bypass its private constructor. WARNING: this may be broken by layout randomization.
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct PanicInfo<'a> {
    payload: &'a (dyn core::any::Any + Send),
    message: Option<&'a core::fmt::Arguments<'a>>,
    location: &'a core::panic::Location<'a>,
    can_unwind: bool,
    force_no_backtrace: bool,
}
/// Converts a managed exception object to a Rust exception payload.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn exception_to_native(exception: Object) -> *mut u8 {
    #[no_mangle]
    /// Gets the length of a string.
    unsafe fn strlen(ptr: *mut u8) -> usize {
        let mut offset = 0;
        while *ptr.offset(offset) != b'\0' {
            offset += 1;
        }
        offset as usize
    }
    //1. Convert the exception to a managed string
    let mstring: MString = rustc_clr_interop_managed_call_virt1_::<
        "System.Runtime",
        "System.Object",
        false,
        "ToString",
        false,
        MString,
        Object,
    >(exception);
    //2. Convert the exception managed string to a UTF-8 null terminated string.
    let ptr: *mut u8 = rustc_clr_interop_managed_call1_::<
        "System.Runtime.InteropServices",
        "System.Runtime.InteropServices.Marshal",
        false,
        "StringToCoTaskMemUTF8",
        true,
        isize,
        MString,
    >(mstring) as *mut u8;

    //4. create a static reference to the message string
    let payload: &'static str =
        unsafe { from_utf8_unchecked(core::slice::from_raw_parts(ptr, strlen(ptr))) };

    custom_payload(payload)
}
// Needed for some convertion. Joinked from core
pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
    // SAFETY: the caller must guarantee that the bytes `v` are valid UTF-8.
    // Also relies on `&str` and `&[u8]` having the same layout.
    unsafe { core::mem::transmute(v) }
}
/// Converts a messagge to an panic payload by panicking and ctaching that panic.
fn custom_payload(pi: &str) -> *mut u8 {
    // Converts a packed messagge to a panic and panics.
    fn panicker(ptr: *mut u8) {
        let (msg, _) = unsafe { &*(ptr as *mut (&str, &mut *mut u8)) };
        panic!("{msg}")
    }
    /// "steals" the panic payload and stores it in the packed data structure.
    fn panic_stealer(data_ptr: *mut u8, payload: *mut u8) {
        let (_, payload_ptr) = unsafe { &mut *(data_ptr as *mut (&str, &mut *mut u8)) };
        **payload_ptr = payload;
    }
    // The resulting panic payload
    let mut res = core::ptr::null_mut();
    // Packed messagge + &mut payload result. Used to create a panic, catch it, and return the payload.
    let mut packed: (&str, &mut *mut u8) = (pi, &mut res);
    // Converts a packed messagge to a panic payload by panicking, and stores the result in the packed payload.
    let res = unsafe {
        core::intrinsics::catch_unwind(panicker, &mut packed as *const _ as *mut u8, panic_stealer)
    };
    // Some sanity checks
    assert_ne!(res, 0);
    assert_ne!(*packed.1, core::ptr::null_mut());
    // Return the payload
    *packed.1
}
