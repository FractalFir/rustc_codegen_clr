#[lang = "eh_personality"]
fn rust_eh_personality() {}
use core::panic::PanicInfo;
#[allow(dead_code)]
extern "C"{
    fn puts(msg:*const u8);
    fn malloc(size:usize)->*mut ();
    fn realloc(ptr:*mut (),size:usize)->*mut ();
} 
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    core::intrinsics::abort();
}
#[start]
fn start(_argc:isize,_argv: *const *const u8) -> isize{
    main();
    // 'All OK!' message
    let msg = 0x00_21_4B_4F_20_6C_6C_41_i64;
    unsafe{puts(core::ptr::addr_of!(msg).cast())}
    0
}
use core::intrinsics::black_box;
#[allow(unused_macros)]
macro_rules! test{
    ($condition:expr)=>{
        if !black_box($condition){
            core::intrinsics::abort();
        }
    }
}
#[allow(unused_macros)]
macro_rules! test_eq{
    ($a:expr,$b:expr)=>{
        if black_box($a) != black_box($b){
            core::intrinsics::abort();
        }
    }
}
#[allow(unused_macros)]
macro_rules! test_ne{
    ($a:expr,$b:expr)=>{
        if black_box($a) == black_box($b){
            core::intrinsics::abort();
        }
    }
}

