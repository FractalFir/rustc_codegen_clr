#![allow(
    internal_features,
    unused_imports,
    incomplete_features,
    unused_variables,
    dead_code,
    improper_ctypes_definitions
)]
#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
//#![no_std]
use std::time::Instant;
//use mycorrhiza::{start,panic_handler};
//panic_handler!{}
//start!{}
//#[lang = "eh_personality"]
//fn rust_eh_personality() {}
extern "C" {
    fn puts(msg: *const u8);
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
    std::hint::black_box(&string);
    unsafe{puts(string.as_ptr())};
   //let mut f = File::create("foo.txt")?;
    //black_box(f);
    let val = std::hint::black_box(*boxed_int);
}
