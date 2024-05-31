#![feature(fmt_internals)]
use std::fmt::Debug;
use std::io::Write;
fn main() {
    use std::fmt::Write;

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
    let mut buff = String::with_capacity(0x100);
    let mut formatter = std::fmt::Formatter::new(&mut buff);
    test_fomratter(&formatter);
    formatter.write_str("OK");
    test_fomratter(&formatter);
    formatter.write_char('O');
    test_fomratter(&formatter);
    let arg = std::hint::black_box(TestFmtEmpty);
    let arg2 = std::hint::black_box(TestFmtEmpty);
    {
        use std::fmt::Debug;
        formatter.write_str("Hello ").unwrap();
        Debug::fmt(&arg, &mut formatter).unwrap();
        formatter.write_str("\n").unwrap();
        Debug::fmt(&arg2, &mut formatter).unwrap();
        formatter.write_str("!").unwrap();
    }
    test_fomratter(&formatter);
    std::thread::spawn(|| {
        for i in 1..10 {
            eprintln!("hi number from the spawned thread!");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    std::thread::sleep(std::time::Duration::from_millis(100));
    /*formatter.write_fmt(format_args!("arg:{arg:?}\n"));
    test_fomratter(&formatter);*/
}
struct TestFmtEmpty;
impl Debug for TestFmtEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        test_fomratter(f);
        f.write_str("TestFmtEmpty")
    }
}

fn test_fomratter<'a>(fmt: &std::fmt::Formatter<'a>) {
    match fmt.align() {
        Some(_) => eprintln!("align is Some"),
        None => eprintln!("align is None"),
    }
    let raw_width: (usize, usize) = unsafe { std::mem::transmute(fmt.width()) };
    eprintln!("raw_width is:");
    std::io::stderr().write_all(raw_width.0.to_string().as_bytes());
    eprintln!(",");
    std::io::stderr().write_all(raw_width.1.to_string().as_bytes());
    eprintln!("width is:");

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
