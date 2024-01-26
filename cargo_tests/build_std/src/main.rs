#![feature(core_intrinsics)]
use std::io::Write;
use std::hint::black_box;
extern "C" {
    fn puts(msg: *const u8);
}
macro_rules! test{
    ($name:ident)=>{
        unsafe{puts(concat!("Running test ",stringify!($name),".\n\0").as_ptr())};
        $name();
        unsafe{puts(concat!("Test ",stringify!($name)," succeded.\n\0").as_ptr())};
    }
}
fn collect_test(){
    let numbers:Vec<_> = std::hint::black_box(0..100).collect();
    std::hint::black_box(&numbers);
    for (number,idx) in numbers.iter().enumerate(){
        if std::hint::black_box(number) != *idx{
            unsafe{puts("collect_test failed: items not equal.\n\0".as_ptr())};
            unsafe{core::intrinsics::abort()};
        }
    } 
}
fn main() {
    let int = std::hint::black_box(8);
    let boxed_int = std::hint::black_box(Box::new(int));
   
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
    test!(collect_test);
    std::hint::black_box(&string);
    unsafe{puts(string.as_ptr())};
    unsafe{puts("Testing some cool shit\n\0".as_ptr())};
    //let mut f = std::fs::File::create("foo.txt").unwrap();

    //std::hint::black_box(f);
    //std::io::stdout().write_all(b"hello world\n").unwrap();
    let owned = black_box("UWU\n\0").to_owned();
    if owned.len() != 5{
        unsafe{puts(owned.as_ptr())};
        unsafe{core::intrinsics::abort()};
    }
    else{
        unsafe{puts(owned.as_ptr())};
    }
    
    //let s = format!("Hello??? WTF is going on???{}\0",black_box(65));
    //unsafe{puts(s.as_ptr())};
    
    let val = std::hint::black_box(*boxed_int);
    let val = std::hint::black_box(string);
}
