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
    //formatter.pad_integral(true,"p","100");
    (std::hint::black_box(8_i32)).fmt(&mut formatter);
    std::hint::black_box(&formatter);
    std::io::stderr().write_all(buff.as_bytes());
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
  
    std::io::stderr().write_all(buffer.as_bytes());
    eprintln!("We are writing to stderr! eight:{eight}",eight = std::hint::black_box("eight"));
}
