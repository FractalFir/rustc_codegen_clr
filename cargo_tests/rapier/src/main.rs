#![feature(fmt_internals)]
use std::io::Write;
use std::fmt::Debug;
fn main(){
    use std::fmt::Write;

    println!("Hello from Rust to .NET!");
    std::fs::File::create("/tmp/rust_on_dotnet.txt").unwrap().write_all(b"Hi from Rust, .NET").unwrap();
    eprintln!("We are writing to stderr!");
    for arg in std::env::args(){
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
    {
        use std::fmt::Debug;
        formatter.write_str("Hello ").unwrap();
        Debug::fmt(&arg, &mut formatter).unwrap();
        formatter.write_str("\n").unwrap();
        Debug::fmt(&arg, &mut formatter).unwrap();
        formatter.write_str("!").unwrap();
    }
    test_fomratter(&formatter);
    formatter.write_fmt(format_args!("arg:{arg:?}"));
    test_fomratter(&formatter);

   
}
struct TestFmtEmpty;
impl Debug for TestFmtEmpty{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        test_fomratter(f);
        f.write_str("TestFmtEmpty")
    }
}
pub struct Formatter2<'a> {
    flags: u32,
    fill: char,
    align: Alignment,
    width: Option<usize>,
    precision: Option<usize>,

    buf: &'a mut (dyn Write + 'a),
}
fn test_fomratter<'a>(fmt:&std::fmt::Formatter<'a>){
    let fmt = unsafe{std::mem::transmute::<_,&Formatter2>(fmt)};
    let raw_width:(usize,usize) = unsafe{std::mem::transmute(fmt.width)};
    eprintln!("raw_width is:");
    std::io::stderr().write_all(raw_width.0.to_string().as_bytes());
    eprintln!(",");
    std::io::stderr().write_all(raw_width.1.to_string().as_bytes());
    eprintln!("width is:");
    
    match fmt.width{
        Some(width)=>{
            eprintln!("Some, ");
            std::io::stderr().write_all(width.to_string().as_bytes());
        }
        None=> eprintln!("None"),
    }
}
fn test_writr_str<'a>(fmt:&mut std::fmt::Formatter<'a>){
    eprintln!("Testing write_str");
    fmt.write_str("Bob.");
    test_fomratter(&*fmt);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Indication that contents should be left-aligned.
    Left,
    /// Indication that contents should be right-aligned.
    Right,
    /// Indication that contents should be center-aligned.
    Center,
}