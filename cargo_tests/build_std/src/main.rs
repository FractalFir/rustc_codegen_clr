#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_panic() {
    panic!();
}
fn main() {
    println!("Hi!");
    for arg in std::env::args() {
        println!("arg:{arg:?}");
    }
}
