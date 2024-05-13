use std::io::Write;
fn main(){
    println!("Hello from Rust to .NET!");
    std::fs::File::create("/tmp/rust_on_dotnet.txt").unwrap().write_all(b"Hi from Rust, .NET").unwrap();
    eprintln!("We are writing to stderr!");
    for arg in std::env::args(){
        std::io::stderr().write_all(arg.as_bytes());
        println!();
    }


    eprintln!("We are writing to stderr! eight:{eight}",eight = std::hint::black_box(8));
}
