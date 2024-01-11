use std::ffi::OsStr;

pub extern "C" fn os_str(){
    let os_str = OsStr::new("foo");
    std::hint::black_box(os_str);
    let st = os_str.to_str().unwrap();
    std::hint::black_box(st);
}
fn main(){
    os_str();
}
