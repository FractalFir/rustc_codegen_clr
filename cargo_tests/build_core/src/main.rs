#![allow(internal_features,unused_imports,incomplete_features,unused_variables,dead_code,improper_ctypes_definitions)]
#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![no_std]
mod ascii;
mod array;
use mycorrhiza::{start,panic_handler};
panic_handler!{}
start!{}
extern "C"{
    fn puts(msg: *const u8);
}
use crate::ascii::*;
use crate::array::*;
#[lang = "eh_personality"]
fn rust_eh_personality() {}
macro_rules! test{
    ($name:ident)=>{
       unsafe{puts(concat!("Running test ",stringify!($name),".\n\r\0").as_ptr())};
       $name();
       unsafe{puts(concat!("Test ",stringify!($name)," passed.\n\r\0").as_ptr())};
    }
}
fn main() {
    let sb = mycorrhiza::system::text::StringBuilder::empty();
    sb.append_char('H');
    sb.append_char('e');
    sb.append_char('l');
    sb.append_char('l');
    sb.append_char('o');
    sb.append_char(' ');
    sb.append_char('W');
    sb.append_char('o');
    sb.append_char('r');
    sb.append_char('l');
    sb.append_char('d');
    sb.append_char('!');
    let mstr = sb.to_mstring();
    mycorrhiza::system::console::Console::writeln_string(mstr);
    let sb = mycorrhiza::system::text::StringBuilder::empty();
    //println!("Hello, world!");
    sb.append_char('ó');
    //sb.append_char('ą');
    //sb.append_char('ź');
    //sb.append_char('ż');
    //sb.append_char('漢');
    //sb.append_char('字');
    sb.append_char('\n');
    sb.append_char('\r');
    let mstr = sb.to_mstring();
    mycorrhiza::system::console::Console::writeln_string(mstr);
    //use mycorrhiza::std::vec::Vec;
    test!(test_is_ascii);
    test!(array_from_ref);
}
