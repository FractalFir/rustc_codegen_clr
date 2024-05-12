use std::io::Write;
fn main(){
    println!("Hello from Rust to .NET!");
    std::fs::File::create("/tmp/rust_on_dotnet.txt").unwrap().write_all(b"Hi from Rust, .NET").unwrap();
    eprintln!("We are writing to stderr!");
}