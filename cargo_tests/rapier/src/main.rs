#![feature(fmt_internals,sync_unsafe_cell,numfmt,core_intrinsics,flt2dec,no_sanitize,extern_types,specialization,maybe_uninit_uninit_array,maybe_uninit_slice,never_type,exposed_provenance)]
use std::fmt::Debug;
use std::io::Write;
//mod fmt;
fn main() {
    /*use std::fmt::Write;

    println!("Hello from Rust to .NET!");
    std::fs::File::create("/tmp/rust_on_dotnet.txt")
        .unwrap()
        .write_all(b"Hi from Rust, .NET")
        .unwrap();
    eprintln!("We are writing to stderr!");
    for arg in std::env::args() {
        std::io::stderr().write_all(arg.as_bytes());
        println!();
    }
 
    test_fomratter(&formatter);
    formatter.write_str("OK");
    test_fomratter(&formatter);
    formatter.write_char('O');
    test_fomratter(&formatter);
   
    let arg2 = std::hint::black_box(TestFmtEmpty(1));
    {
        use std::fmt::Debug;
        formatter.write_str("Hello ").unwrap();
        Debug::fmt(&arg, &mut formatter).unwrap();
        formatter.write_str("\n").unwrap();
        Debug::fmt(&arg2, &mut formatter).unwrap();
        formatter.write_str("!").unwrap();
    }
    test_fomratter(&formatter);
    call_dyn(&arg, &mut formatter);
    /*std::thread::spawn(|| {
        for i in 1..10 {
            eprintln!("hi number from the spawned thread!");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    std::thread::sleep();
    */
    //unsafe{fmt::write(std::mem::transmute::<&mut std::fmt::Formatter,&mut fmt::Formatter>(&mut formatter),)};
    //test_fomratter(&formatter);
    */
    /*let arg = std::hint::black_box(TestFmtEmpty(0));
    let mut buf = String::with_capacity(0x100);
    let mut formatter = std::fmt::Formatter::new(&mut buf);
    let args = unsafe{std::mem::transmute::<std::fmt::Arguments<'_>,fmt::Arguments<'_>>(format_args!("arg:{arg:?}\n"))};
    unsafe{args.args[0].fmt(std::mem::transmute::<&mut std::fmt::Formatter,&mut fmt::Formatter>(&mut formatter))};*/

    eprintln!("Formatting in .NET! Test int: {int} Test float:{float}
    dur:{dur:?}",int = std::hint::black_box(64),float = std::hint::black_box(3.14159),dur = std::hint::black_box(std::time::Duration::from_millis(1000)));
}
struct TestFmtEmpty(u32);
impl Debug for TestFmtEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        test_fomratter(f);
        unsafe{printf("addr of self is %p\n\0".as_ptr() as *const i8,self as *const _ as usize);}
        f.write_str("TestFmtEmpty")
    }
}

fn test_fomratter<'a>(fmt: &std::fmt::Formatter<'a>) {
    use std::io::Write;
    
    let raw_width: (usize, usize) = unsafe { std::mem::transmute(fmt.width()) };
    unsafe{printf("raw_width is %p,tag is %p.\n\0".as_ptr() as *const i8,raw_width.0,raw_width.1);
    printf("addr is %p\n\0".as_ptr() as *const i8,fmt as *const _ as usize);}
    match fmt.align() {
        Some(_) => eprintln!("align is Some"),
        None => eprintln!("align is None"),
    }
    match fmt.width() {
        Some(width) => {
            eprintln!("Some, ");
            std::io::stderr().write_all(width.to_string().as_bytes());
        }
        None => eprintln!("None"),
    }
}
fn test_writr_str<'a>(fmt: &mut std::fmt::Formatter<'a>) {
    eprintln!("Testing write_str");
    fmt.write_str("Bob.");
    test_fomratter(&*fmt);
}
extern "C" {
    fn printf(_: *const core::ffi::c_char, _: ...) -> core::ffi::c_int;
}