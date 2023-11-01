#![no_std]
#![feature(start,core_intrinsics,lang_items)]
//use glam::*;
use mycorrhiza::{start,panic_handler};
use core::hint::black_box;
panic_handler!{}
start!{}
#[lang = "eh_personality"]
fn rust_eh_personality() {}
fn main() {
    let tuple = (2,black_box(8));
    black_box(tuple);
    mycorrhiza::system::runtime::Console;
}
