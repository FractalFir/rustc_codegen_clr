#![no_std]
#![feature(start,core_intrinsics,lang_items)]
use mycorrhiza::{start,panic_handler};
panic_handler!{}
start!{}
#[lang = "eh_personality"]
fn rust_eh_personality() {}
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
    sb.append_char('\n');
    sb.append_char('\r');
    sb.append_char('ó');
    sb.append_char('ą');
    sb.append_char('ź');
    sb.append_char('ż');
    sb.append_char('漢');
    sb.append_char('字');
    sb.append_char('\n');
    sb.append_char('\r');
    let mstr = sb.to_mstring();
    mycorrhiza::system::console::Console::writeln_string(mstr);
    //println!("Hello, world!");
}
