fn main() {
    let result = std::panic::catch_unwind(|| {
        panic!("oh no!");
    });
    println!("Catched a panic!");
    assert!(result.is_err());
    println!("Rsult is correctly an error.");
}
/*
#[test]
fn test() {}
#[test]
fn test2() {}
#[test]
fn test3() {}
#[test]
#[ignore]
fn test_ignored() {}
*/
#[test]
fn test_panic() {
    panic!("oh no!");
}
