

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub extern "C" fn setup(){
    let test_box = Box::new(128);
    std::hint::black_box(&test_box);

}
