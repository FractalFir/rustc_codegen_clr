#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_fail() {
    panic!();
}
pub fn main() {
    let mut vec = vec![];
    assert_eq!(vec.len(),0);
    vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8_u8];
    let mut s = String::new();
    s.push_str("Hi. Message");
    s.push_str("Hi. Message message messafe message mesasage!");
    s.push_str("Hi. Message");
    println!("Hello world! vec:{vec:?} s:{s:?}");
}
