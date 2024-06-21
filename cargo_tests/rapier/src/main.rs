use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn test() {
    extern "C" {
        fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    }
    unsafe { printf("The test is propely run\n\0".as_ptr() as *const i8) };
}
