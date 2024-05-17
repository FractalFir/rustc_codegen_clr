#![feature(fmt_internals)]
use std::io::Write;
use std::fmt::Debug;
fn main(){
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
    //formatter.pad_integral(true,"p","100");
    (std::hint::black_box(8_i32)).fmt(&mut formatter);
    test_fomratter(&formatter);
    std::hint::black_box(&formatter);
    //std::io::stderr().write_all(buff.as_bytes());
    //let mut buffer = String::new();
    //std::io::stdin().read_line(&mut buffer).unwrap();
  
    //std::io::stderr().write_all(buffer.as_bytes());
    eprintln!("TestFmtEmpty:{:?}",std::hint::black_box(TestFmtEmpty));
    eprintln!("We are writing to stderr! eight:{eight}",eight = std::hint::black_box("eight"));
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Indication that contents should be left-aligned.
    Left,
    /// Indication that contents should be right-aligned.
    Right,
    /// Indication that contents should be center-aligned.
    Center,
}