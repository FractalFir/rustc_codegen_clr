#![no_std]
#![feature(start,core_intrinsics,lang_items)]
use mycorrhiza::{start,panic_handler};
use core::intrinsics::black_box;
panic_handler!{}
start!{}
#[lang = "eh_personality"]
fn rust_eh_personality() {}
fn main() {
    let time = black_box(Fibonachi::benchmark());
}
trait BenchmarkableFn{
    fn run();
    fn benchmark()->f64{
        use mycorrhiza::system::diagnostics::Stopwatch;
        // Let the JIT warm up.
        for _ in 0..black_box(100_000/100){
            Self::run();
        }
        let stopwatch = Stopwatch::new();
        stopwatch.start();
        for _ in 0..black_box(100_000){
            Self::run();
        }
        stopwatch.stop();
        let ms = stopwatch.elapsed_milliseconds();
        let ns = (ms*1_000) as f64;
        let ns_per_iter = ns/(black_box(100_000) as f64);
        ns_per_iter
    }
}
struct Fibonachi;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
impl BenchmarkableFn for Fibonachi{
    fn run(){
        black_box(fibonacci(black_box(5)));
    }
}
