#![no_std]
#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
include!("../common.rs");
#[repr(transparent)]
pub struct Slice {
    pub inner: [u8],
}
impl AsRef<Slice> for str{
    fn as_ref(&self) -> &Slice {
        unsafe{core::mem::transmute(self)}
    }
}
impl AsRef<OsStr> for Slice{
    fn as_ref(&self) -> &OsStr {
        unsafe{core::mem::transmute(self)}
    }
}
impl AsRef<OsStr> for str{
    fn as_ref(&self) -> &OsStr {
        let s: &Slice = self.as_ref();
        s.as_ref()
    }
}
impl Slice{
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        unsafe{Ok(core::mem::transmute(&self.inner))}
    }
}
#[repr(transparent)]
pub struct OsStr {
    inner: Slice,
}
pub extern "C" fn os_str(){
    let os_str = OsStr::new("foo");
    black_box(os_str);
    let st = os_str.to_str().unwrap();
    black_box(st);
}
impl OsStr{
    #[inline]
    pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &OsStr {
        s.as_ref()
    }
    #[inline]
    pub fn to_str(&self) -> Option<&str> {
        unsafe{Some(core::mem::transmute(&self.inner.inner))}
    }
}
fn main(){
    os_str();
}
