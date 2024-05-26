
#![feature(start, core_intrinsics, lang_items)]
use core::intrinsics::black_box;
use mycorrhiza::{panic_handler, start};

fn main() {
    let time = black_box(Fibonachi::benchmark());
    mycorrhiza::system::console::Console::writeln_f64(time);
    //let time = black_box(BigAlloc::benchmark());
    //mycorrhiza::system::console::Console::writeln_f64(time);
}
trait BenchmarkableFn {
    const RUNS:usize;
    fn run();
    #[cfg(not(test))]
    fn benchmark() -> f64 {
        use mycorrhiza::system::diagnostics::Stopwatch;
        // Let the JIT warm up.
        for _ in 0..Self::RUNS {
            Self::run();
        }
        let stopwatch = Stopwatch::new();
        stopwatch.start();
        for _ in 0..Self::RUNS {
            Self::run();
        }
        stopwatch.stop();
        let ms = stopwatch.elapsed_milliseconds();
        let ns = (ms * 1_000_000) as f64;
        let ns_per_iter = ns / (Self::RUNS as f64);
        ns_per_iter
    }
    #[cfg(test)]
    fn benchmark() -> f64 {
        // Here just to elimnate any wierd codegen flukes
        for _ in 0..Self::RUNS {
            Self::run();
        }
        let stopwatch = std::time::Instant::now();
        for _ in 0..Self::RUNS {
            Self::run();
        }
        let ms = stopwatch.elapsed().as_millis();
        let ns = (ms * 1_000_000) as f64;
        let ns_per_iter = ns / (Self::RUNS as f64);
        ns_per_iter
    }
}
struct Fibonachi;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
impl BenchmarkableFn for Fibonachi {
    const RUNS:usize = 100_000_000;
    fn run() {
        black_box(fibonacci(black_box(10)));
    }
}
/* 
struct BigAlloc;
impl BenchmarkableFn for BigAlloc {
    const RUNS:usize = 100_000;
    fn run() {
        let mut vec = vec![0;85_000];
        black_box(&mut vec);
        for val in &mut vec{
            *val = 0;
        }
        black_box(vec);
    }
}*/


#[cfg(test)]
#[test]
fn native_bench() {
    let time1 = black_box(Fibonachi::benchmark());
   // let time2 = black_box(BigAlloc::benchmark());
    panic!("{time1}");
}
