use std::ffi::c_char;
use std::ffi::c_int;
use std::hint::black_box;
extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
fn bench_for_each_chain_fold<const BIG: u32>() -> u32 {
    let mut acc = 0;
    unsafe { printf(c"Calling `fold` %d times\n".as_ptr(), BIG) };
    let iter = (0..BIG).chain(0..BIG).map(black_box);
    for_each_fold(iter, |x| acc += x);
    unsafe { printf(c"DONE!\n".as_ptr()) };
    acc
}
fn for_each_fold<I, F>(iter: I, mut f: F)
where
    I: Iterator,
    F: FnMut(I::Item),
{
    iter.fold((), move |(), item| f(item));
}
fn main() {
    black_box(bench_for_each_chain_fold::<1>());
    black_box(bench_for_each_chain_fold::<10>());
    black_box(bench_for_each_chain_fold::<100>());
    black_box(bench_for_each_chain_fold::<1000>());
    black_box(bench_for_each_chain_fold::<10_000>());
}
