#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_fail() {
    panic!();
}
pub fn main() {
    println!("Hi! UWU");
}
