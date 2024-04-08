#![feature(core_intrinsics, adt_const_params)]
#![feature(slice_ptr_get)]
#![feature(allocator_api)]
#![feature(once_cell_try)]
use std::ffi::{c_char, c_int, CString};
use std::hint::black_box;
use std::io::Write;

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn __rust_realloc(old:*mut u8, old_size:usize,new_size:usize,align:usize)->*mut u8;
}

macro_rules! test {
    ($name:ident) => {
        unsafe {
            printf(concat!("Running test ", stringify!($name), ".\n\0").as_ptr() as *const i8)
        };
        $name();
        unsafe {
            printf(concat!("Test ", stringify!($name), " succeded.\n\0").as_ptr() as *const i8)
        };
    };
}
fn print_string(string: &str) {
    unsafe {
        let mut string = string.to_owned();
        
        string.push('\0');
        if printf("%s\0".as_ptr() as *const i8, string.as_ptr() as *const i8) != 0 {
            printf("Could not print!\n\0".as_ptr() as *const i8);
            panic!();
        }
        printf("Could print. First char: %c!\n\0".as_ptr() as *const i8,string.chars().nth(0));
    }
}
fn main() {
   
    print_string("String literals work!\n");
}
