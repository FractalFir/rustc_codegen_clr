#![no_std]
#![feature(start,core_intrinsics,lang_items)]
use fastrand::*;
use mycorrhiza::{start,panic_handler};
use core::hint::black_box;
panic_handler!{}
start!{}
#[lang = "eh_personality"]
fn rust_eh_personality() {}
fn main() {
    let tuple = (2,black_box(8));
    black_box(tuple);
    let mut rng = black_box(fastrand::Rng::with_seed(0xDEAD_BEEF));
//let mut bytes: Vec<u8> = repeat_with(|| ).take(10_000).collect();


    mycorrhiza::system::console::Console::writeln_u64(black_box(rng.u64(..)));
}
