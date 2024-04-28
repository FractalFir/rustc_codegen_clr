#![recursion_limit = "1024"]

extern crate core;
use core::ffi::c_char;
use core::ffi::c_int;

fn fn4() -> () {
    let mut _5 = 6.6;
    _5 = 16692680020138226209_u64 as f64;
    unsafe {
        printf(
            "%f\n\0".as_ptr() as *const c_char,
            _5 as core::ffi::c_double,
        )
    };
}
pub fn fn0() -> () {
    let _5 = 2279710617_u32 as i64;
    unsafe {
        printf("%li\n\0".as_ptr() as *const c_char, _5);
    }
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
fn main() {
    std::hint::black_box(fn4());
    std::hint::black_box(fn0());
}
