#![feature(lang_items)]
#![feature(start)]
#![allow(internal_features)]
#![no_std]
#![feature(core_intrinsics)]
use core::panic::PanicInfo;
#[lang = "eh_personality"]
fn rust_eh_personality() {}
extern "C" {
    fn puts(string:*const u8);
    fn putc(c:u8);
}
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
#[start]
fn main(_argc: isize, _argv: *const *const u8)->isize{
    //Broken array
    let message:[u8;6] = [0x48,0x64,0x6C,0x6F,0x20,0x0];
    unsafe{puts(core::ptr::addr_of!(message) as *const u8)};
    /* 
    unsafe{putc(0x48)};
    unsafe{putc(0x65)};
    unsafe{putc(0x6C)};
    unsafe{putc(0x6C)};
    unsafe{putc(0x6F)};
    unsafe{putc(0x20)};
    unsafe{putc(0x43)};
    unsafe{putc(0x4C)};
    unsafe{putc(0x52)};
    unsafe{putc(0x20)};
    unsafe{putc(0x66)};
    unsafe{putc(0x72)};
    unsafe{putc(0x6F)};
    unsafe{putc(0x6D)};
    unsafe{putc(0x20)};
    unsafe{putc(0x52)};
    unsafe{putc(0x75)};
    unsafe{putc(0x73)};
    unsafe{putc(0x74)};
    unsafe{putc(0x21)};
    unsafe{putc(0x0A)};
    */
    0
}