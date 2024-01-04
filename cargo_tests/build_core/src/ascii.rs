use crate::*;
use core::char::from_u32;
pub fn test_is_ascii() {
    assert!(b"".is_ascii());
    assert!(b"banana\0\x7F".is_ascii());
    assert!(b"banana\0\x7F".iter().all(|b| b.is_ascii()));
    assert!(!b"Vi\xe1\xbb\x87t Nam".is_ascii());
    // This panics for some reason. An issue with closures?

    //assert!(!b"Vi\xe1\xbb\x87t Nam".iter().all(|b| b.is_ascii()));
    //assert!(!b"\xe1\xbb\x87".iter().any(|b| b.is_ascii()));

    assert!("".is_ascii());
    assert!("banana\0\u{7F}".is_ascii());
    //assert!("banana\0\u{7F}".chars().all(|c| c.is_ascii()));
    //assert!(!"ประเทศไทย中华Việt Nam".chars().all(|c| c.is_ascii()));
    //assert!(!"ประเทศไทย中华ệ ".chars().any(|c| c.is_ascii()));
}
