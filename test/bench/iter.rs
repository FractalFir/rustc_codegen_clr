const BIG: i64 = 10_000_000;
fn bench_for_each_chain_fold() -> i64 {
    let mut acc = 0;
    let iter = (0i64..BIG).chain(0..BIG).map(std::hint::black_box);
    for_each_fold(iter, |x| acc += x);
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
    std::hint::black_box(bench_for_each_chain_fold());
}
