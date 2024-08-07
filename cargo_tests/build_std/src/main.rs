#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_panic() {
    panic!();
}
#[test]
#[should_panic]
fn div_by_zero() {
    let zero = std::hint::black_box(0);
    let res = 64 / zero;
    std::hint::black_box(res);
}
fn main() {
    println!("Hi!");
    for arg in std::env::args() {
        println!("arg:{arg:?}");
    }
}
