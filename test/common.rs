#[allow(unused_imports)]
use core::intrinsics::sqrtf32;
#[allow(dead_code)]
extern "C" {
    fn puts(msg: *const u8);
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
    fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
use core::intrinsics::black_box;
#[allow(unused_macros)]
#[macro_export]
macro_rules! test {
    ($condition:expr) => {
        if !core::intrinsics::black_box($condition) {
            crate::rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u32,
            >(line!());
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::breakpoint() };
            core::intrinsics::abort();
        }
    };
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! test_eq {
    ($a:expr,$b:expr) => {{
        let a = $a;
        let b = $b;
        if core::intrinsics::black_box(&a) != core::intrinsics::black_box(&b) {
            //Put::putnl(a);
            //Put::putnl(b);
            crate::rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u32,
            >(line!());
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::breakpoint() };
            core::intrinsics::abort();
        }
    }};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! test_ne {
    ($a:expr,$b:expr) => {
        if core::intrinsics::black_box($a) == core::intrinsics::black_box($b) {
            //Put::putnl($a);
            //Put::putnl($b);
            crate::rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u32,
            >(line!());
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::breakpoint() };
            core::intrinsics::abort();
        }
    };
}
//Interop thingies
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_call0_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    Ret,
>() -> Ret {
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
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
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_call2_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    const IS_STATIC: bool,
    Ret,
    Arg1,
    Arg2,
>(
    arg1: Arg1,
    arg2: Arg2,
) -> Ret {
    core::intrinsics::abort();
}
trait Put: Sized {
    fn putnl(val: Self) {
        rustc_clr_interop_managed_call1_::<
            "System.Console",
            "System.Console",
            false,
            "WriteLine",
            true,
            (),
            Self,
        >(val);
    }
    fn put(val: Self) {
        rustc_clr_interop_managed_call1_::<
            "System.Console",
            "System.Console",
            false,
            "Write",
            true,
            (),
            Self,
        >(val);
    }
}
impl Put for i8 {
    fn putnl(val: Self) {
        <i32 as Put>::putnl(val as i32);
    }
    fn put(val: Self) {
        <i32 as Put>::put(val as i32);
    }
}
impl Put for i16 {
    fn putnl(val: Self) {
        <i32 as Put>::putnl(val as i32);
    }
    fn put(val: Self) {
        <i32 as Put>::put(val as i32);
    }
}
impl Put for i32 {}
impl Put for i64 {}
impl Put for u8 {
    fn putnl(val: Self) {
        <u32 as Put>::putnl(val as u32);
    }
    fn put(val: Self) {
        <u32 as Put>::put(val as u32);
    }
}
impl Put for u16 {
    fn putnl(val: Self) {
        <u32 as Put>::putnl(val as u32);
    }
    fn put(val: Self) {
        <u32 as Put>::put(val as u32);
    }
}
impl Put for usize {
    fn putnl(val: Self) {
        <u64 as Put>::putnl(val as u64);
    }
    fn put(val: Self) {
        <u64 as Put>::put(val as u64);
    }
}
impl<T> Put for *mut T {
    fn putnl(val: Self) {
        <usize as Put>::putnl(val as usize);
    }
    fn put(val: Self) {
        <usize as Put>::put(val as usize);
    }
}
impl<T> Put for *const T {
    fn putnl(val: Self) {
        <usize as Put>::putnl(val as usize);
    }
    fn put(val: Self) {
        <usize as Put>::put(val as usize);
    }
}
impl Put for u128 {
    fn putnl(val: Self) {
        <u64 as Put>::put(val as u64);
        <u64 as Put>::putnl((val / (u64::MAX as u128)) as u64);
    }
    fn put(val: Self) {
        <u64 as Put>::put(val as u64);
        <u64 as Put>::put((val / (u64::MAX as u128)) as u64);
    }
}
impl Put for i128 {
    fn putnl(val: Self) {
        <u64 as Put>::put(val as u64);
        <u64 as Put>::putnl((val as u128 / (u64::MAX as u128)) as u64);
    }
    fn put(val: Self) {
        <u64 as Put>::put(val as u64);
        <u64 as Put>::put((val as u128 / (u64::MAX as u128)) as u64);
    }
}
impl Put for u32 {}
impl Put for u64 {}
impl Put for f32 {}
impl Put for f64 {}
fn println(msg: &str) {
    unsafe {
        let tmp = malloc(msg.len() + 1) as *mut u8;
        let tmp_slice: &mut [u8] = core::slice::from_raw_parts_mut(tmp, msg.len() + 1);
        tmp_slice[..msg.len()].clone_from_slice(msg.as_bytes());
        tmp_slice[msg.len()] = b'\0';
        printf(c"%s\n".as_ptr(), tmp_slice.as_ptr() as *const i8);
        free(tmp as *mut core::ffi::c_void);
    }
}
struct Vecy<T:Copy + 'static>{
    backer:&'static mut [T],
}
impl<T:Copy + 'static> Vecy<T>{
    fn new(t:T, size:usize)->Self{
        let backer = unsafe{std::slice::from_raw_parts_mut(malloc(size * std::mem::size_of_val(&t)) as *mut T, size)};
        for val in backer.iter_mut(){
            *val = t;
        }
        Self{backer}
    }
    fn as_slice_mut<'a>(&'a mut self)->&'a mut [T]{
        self.backer
    }
    fn as_slice<'a>(&'a  self)->&'a  [T]{
        &self.backer[..]
    }
}
impl<T:Copy + 'static> Drop for Vecy<T>{
    fn drop(&mut self){
        unsafe{free(self.backer.as_ptr() as *mut _)};
    }
}