#![feature(core_intrinsics, adt_const_params)]
#![feature(slice_ptr_get)]
#![feature(allocator_api)]
#![feature(once_cell_try)]
use std::ffi::{c_char, c_int};
use std::hint::black_box;
use std::io::Write;
mod cstr;
mod exchange_malloc;
use crate::exchange_malloc::exchange_malloc_test;
extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
mod map_copy {
    pub struct Map<I, F> {
        // Used for `SplitWhitespace` and `SplitAsciiWhitespace` `as_str` methods
        iter: I,
        f: F,
    }
    impl<I, F> Map<I, F> {
        pub fn new(iter: I, f: F) -> Map<I, F> {
            Map { iter, f }
        }
    }
    impl<B, I: Iterator, F> Iterator for Map<I, F>
    where
        F: FnMut(I::Item) -> B,
    {
        type Item = B;

        #[inline]
        fn next(&mut self) -> Option<B> {
            self.iter.next().map(&mut self.f)
        }
    }
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
    unsafe {
        printf("Called interop managed call when compiled native code.\n\0".as_ptr() as *const i8)
    };
    core::intrinsics::abort();
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
fn test_ptr_offset_from_unsigned() {
    // Define two pointers
    let ptr1: *const u8 = 0x1000 as *const u8;
    let ptr2: *const u8 = 0x1004 as *const u8;

    // Calculate the offset between ptr2 and ptr1
    let offset = unsafe { std::intrinsics::ptr_offset_from_unsigned(ptr2, ptr1) };

    // Expected result: 4 bytes (because ptr2 is 4 bytes ahead of ptr1)
    assert_eq!(offset, 4);

    // Test with another example
    let ptr3: *const u8 = 0x2000 as *const u8;
    let ptr4: *const u8 = 0x1000 as *const u8;

    let offset2 = unsafe { std::intrinsics::ptr_offset_from_unsigned(ptr3, ptr4) };

    // Expected result: 4096 bytes (because ptr4 is 4096 bytes ahead of ptr3)
    assert_eq!(offset2, 4096);

    // Additional test case: Pointers are equal
    let ptr5: *const u8 = 0x3000 as *const u8;

    let offset3 = unsafe { std::intrinsics::ptr_offset_from_unsigned(ptr5, ptr5) };

    // Expected result: 0 (since both pointers are the same)
    assert_eq!(offset3, 0);
}
fn collect_test() {
    let numbers: Vec<_> = std::hint::black_box(0..100).collect();
    std::hint::black_box(&numbers);
    for (number, idx) in numbers.iter().enumerate() {
        if std::hint::black_box(number) != *idx {
            unsafe { printf("collect_test failed: items not equal.\n\0".as_ptr() as *const i8) };
            unsafe { core::intrinsics::abort() };
        }
    }
}
fn map_option_test() {
    let option = Some(std::hint::black_box(2_u64));
    let option = option.map(|v| v * v);
    let number = option.unwrap();
    if std::hint::black_box(number) != 4 {
        rustc_clr_interop_managed_call1_::<
            "System.Console",
            "System.Console",
            false,
            "WriteLine",
            true,
            (),
            u64,
        >(number);
        unsafe { printf("map_option_test failed: items not equal.\n\0".as_ptr() as *const i8) };
        unsafe { core::intrinsics::abort() };
    }
}
fn test_vec() {
    unsafe {
        printf("Testing the vec type...\n\0".as_ptr() as *const i8);
        let mut chars = Vec::with_capacity(1);
        printf("Allocated the vec type!\n\0".as_ptr() as *const i8);
        chars.push('H');
        printf("Pushed a value into the vec type!\n\0".as_ptr() as *const i8);
        let val = chars.pop().unwrap();
        printf("Poped a value from the vec type!\n\0".as_ptr() as *const i8);

        printf("The a value is %c!\n\0".as_ptr() as *const i8, val);
    }
}
fn test_string() {
    unsafe {
        printf("Testing the String type...\n\0".as_ptr() as *const i8);
        let mut string = String::with_capacity(1);
        printf("Allocated the String type!\n\0".as_ptr() as *const i8);
        string.push('H');
        printf("Pushed a value into the String type!\n\0".as_ptr() as *const i8);
        let val = string.pop().unwrap();
        printf("Poped a value from the String type!\n\0".as_ptr() as *const i8);

        printf("The a value is %c!\n\0".as_ptr() as *const i8, val);
    }
}
fn map_test() {
    for (idx, number) in std::hint::black_box(0..100).map(|i| i * i).enumerate() {
        if std::hint::black_box(number) != idx * idx {
            rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u64,
            >(number as u64);
            rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u64,
            >(idx as u64);
            unsafe { printf("map_test1b failed: items not equal.\n\0".as_ptr() as *const i8) };
            unsafe { core::intrinsics::abort() };
        }
    }
}

fn print_args(){
    for (idx,mut arg) in std::env::args().enumerate(){
        arg.push('\0');
        unsafe{printf("Arg %d:%s\n\0".as_ptr() as *const i8,idx as u32, arg.as_ptr())};
        drop(arg);
    }
}
fn test_file(){
    use std::io::Write;
    let mut file = std::fs::File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
fn main() {
    print_args();
    let idx = black_box(64);
    let s = format!("Hello!\n\0");
    test_file();
    unsafe{printf(s.as_ptr() as *const i8)};
    let _ = s;
    return;
    test_vec();
    test_string();
    let int = std::hint::black_box(8);
    let boxed_int = std::hint::black_box(Box::new(8));
    //let mut file = std::fs::File::create("foo.txt").unwrap();
    //test!(exchange_malloc_test);
    //test!(map_option_test);
    let mut string = String::with_capacity(100);
    string.push('H');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');
    string.push('.');
    string.push('\n');
    string.push('T');
    string.push('h');
    string.push('i');
    string.push('s');
    string.push(' ');
    string.push('m');
    string.push('e');
    string.push('s');
    string.push('s');
    string.push('a');
    string.push('g');
    string.push('e');
    string.push(' ');
    string.push('w');
    string.push('a');
    string.push('s');
    string.push(' ');
    string.push('c');
    string.push('r');
    string.push('e');
    string.push('a');
    string.push('t');
    string.push('e');
    string.push('d');
    string.push(' ');
    string.push('u');
    string.push('s');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push(' ');
    string.push('R');
    string.push('u');
    string.push('s');
    string.push('t');
    string.push('s');
    string.push(' ');
    string.push('`');
    string.push('s');
    string.push('t');
    string.push('d');
    string.push(':');
    string.push(':');
    string.push('s');
    string.push('t');
    string.push('r');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push(':');
    string.push(':');
    string.push('S');
    string.push('t');
    string.push('r');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push('`');
    string.push(' ');
    string.push('t');
    string.push('y');
    string.push('p');
    string.push('e');
    string.push(' ');
    string.push('i');
    string.push('n');
    string.push('s');
    string.push('i');
    string.push('d');
    string.push('e');
    string.push(' ');
    string.push('t');
    string.push('h');
    string.push('e');
    string.push(' ');
    string.push('.');
    string.push('N');
    string.push('E');
    string.push('T');
    string.push(' ');
    string.push('r');
    string.push('u');
    string.push('n');
    string.push('t');
    string.push('i');
    string.push('m');
    string.push('e');
    string.push('!');
    string.push('\n');
    string.push('\0');
    unsafe { printf("%s\n\0".as_ptr() as *const i8, string.as_ptr()) };
    //test!(collect_test);
    //test!(map_test);
    use cstr::test_cstr;
    test!(test_cstr);
    //std::hint::black_box(&string);
    if unsafe { printf("Testing some cool shit\n\0".as_ptr() as *const i8) } < 0 {
        std::intrinsics::abort();
    }
    //let mut f = std::fs::File::create("foo.txt").unwrap();

    //std::hint::black_box(f);
    //std::io::stdout().write_all(b"hello world\n").unwrap();
    let owned = Box::new(black_box(&[0, 1, 2, 3, 4, 5, 6][..]));
    if owned.len() != 7 {
        unsafe { printf("Boxed slice size mismacth!\n\0".as_ptr() as *const i8) };
        unsafe { core::intrinsics::abort() };
    }

    for (idx, val) in owned.iter().enumerate() {
        if idx != *val {
            //unsafe { printf("Slice impropely copied %d %d!\n\0".as_ptr() as *const i8,idx,val) };
            rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u64,
            >(idx as u64);
            rustc_clr_interop_managed_call1_::<
                "System.Console",
                "System.Console",
                false,
                "WriteLine",
                true,
                (),
                u64,
            >(*val as u64);
            //unsafe { core::intrinsics::abort() };
        }
    }
    let owned = black_box("Test\n\0").to_owned();
    if owned.len() != 6 {
        unsafe { printf(owned.as_ptr() as *const i8) };
        unsafe { core::intrinsics::abort() };
    } else {
        unsafe { printf(owned.as_ptr() as *const i8) };
    };
    test_thread_local();
    //lock_test();
    // test_stdout();
    test_stderr();

    let s = format!("Hello!\n\0");
    unsafe { printf(s.as_ptr() as *const i8) };
    let s = format!("Hello??? WTF is going on???{}\n\0", black_box(65));
    unsafe { printf(s.as_ptr() as *const i8) };

    let val = std::hint::black_box(*boxed_int);
    //let val = std::hint::black_box(string);
}
fn test_thread_local() {
    use std::cell::Cell;
    unsafe{printf("Testing a thread local.\n\0".as_ptr() as *const i8)};
    thread_local! {
        static X: Cell<i32> = panic!("!");
    }

    // Calling X.get() here would result in a panic.
    unsafe{printf("Preparing to set a thread local.\n\0".as_ptr() as *const i8)};
    X.set(123); // But X.set() is fine, as it skips the initializer above.
    unsafe{printf("Seting a thread local succeded.\n\0".as_ptr() as *const i8)};
    assert_eq!(X.get(), 123);
    unsafe{printf("Acquiring a thread local succeded.\n\0".as_ptr() as *const i8)};
}
fn test_stderr() {
    use std::io::{self, Write};
    let stdout = io::stderr();
    let mut handle = stdout;

    handle.write_all(b"hello error").unwrap();
}
fn test_stdout() {
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"hello world").unwrap();
}
fn lock_test() {
    use std::sync::OnceLock;

    static COMPUTATION: OnceLock<u8> = OnceLock::new();
    if let Some(_) = COMPUTATION.get() {
        core::intrinsics::abort();
    }
    if *COMPUTATION.get_or_try_init(|| Ok::<u8, ()>(77)).unwrap() != 77 {
        core::intrinsics::abort();
    }
}

#[test]
fn fun() {}
