#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_fail() {
    panic!();
}
pub fn main() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8_u8];
    println!("A");
    let mut s = String::new();
    println!("B");
    s.push_str("Hi. Message");
    println!("C");
    s.push_str("Hi. Message message messafe message mesasage!");
    println!("D");
    s.push_str("Hi. Message");
    println!("E");
    println!("Hi! UWU:{vec:?} {s:?}");
    println!("F");
}
