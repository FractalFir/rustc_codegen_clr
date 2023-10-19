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
    //let a = black_box(DVec2::new(3.6754,black_box(7.7657)));
    //let b = DVec3::new(3.1265,5.9878,black_box(-7.9847));
    //black_box(find_midpoint(a,b));
}
