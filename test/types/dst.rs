#![no_std]
#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    error_generic_member_access,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
#[repr(transparent)]
pub struct Slice {
    pub inner: [u8],
}
impl AsRef<Slice> for str {
    fn as_ref(&self) -> &Slice {
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<OsStr> for Slice {
    fn as_ref(&self) -> &OsStr {
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<OsStr> for str {
    fn as_ref(&self) -> &OsStr {
        let s: &Slice = self.as_ref();
        s.as_ref()
    }
}
impl Slice {
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        unsafe { Ok(core::mem::transmute(&self.inner)) }
    }
}
#[repr(transparent)]
pub struct OsStr {
    inner: Slice,
}
pub extern "C" fn os_str() {
    let os_str = OsStr::new("foo");
    black_box(os_str);
    let st = os_str.to_str().unwrap();
    black_box(st);
}
impl OsStr {
    #[inline]
    pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &OsStr {
        s.as_ref()
    }
    #[inline]
    pub fn to_str(&self) -> Option<&str> {
        unsafe { Some(core::mem::transmute(&self.inner.inner)) }
    }
}
fn main() {
    os_str();
    // A reference to child should be thin.
    test_eq!(core::mem::size_of::<&Child>(), core::mem::size_of::<&()>());
    //test10fn();
}
#[derive(Debug)]
struct Parent(Option<u8>);
#[derive(Debug)]
struct Child {
    parent: Parent,
}

fn test10fn() -> impl Sized {
    use core::error::request_value;
    use core::error::Request;

    impl core::fmt::Display for Parent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "a parent failed")
        }
    }
    impl core::error::Error for Parent {
        fn provide<'a>(&'a self, request: &mut Request<'a>) {
            println("Provide inner");
            if let Some(v) = self.0 {
                println("Calling provide_value");
                request.provide_value::<u8>(v);
                println("Called provide_value");
            }
            println("Provide inner end");
        }
    }

    impl Child {
        // Pretend that this takes a lot of resources to evaluate.
        fn an_expensive_computation(&self) -> Option<u8> {
            Some(99)
        }
    }
    impl core::fmt::Display for Child {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "child failed: \n  because of parent: {}", self.parent)
        }
    }

    impl core::error::Error for Child {
        fn provide<'a>(&'a self, request: &mut Request<'a>) {
            // In general, we don't know if this call will provide
            // an `u8` value or not...
            println("Pareparing to call provide");
            self.parent.provide(request);
            println("Called provide");
            // ...so we check to see if the `u8` is needed before
            // we run our expensive computation.
            println("Pareparing to call would_be_satisfied_by_value_of");
            if request.would_be_satisfied_by_value_of::<u8>() {
                if let Some(v) = self.an_expensive_computation() {
                    request.provide_value::<u8>(v);
                }
            }
            println("Called would_be_satisfied_by_value_of");
            // The request will be satisfied now, regardless of if
            // the parent provided the value or we did.
            assert!(!request.would_be_satisfied_by_value_of::<u8>());
        }
    }
    let parent = Parent(Some(42));
    let child = Child { parent };
    assert_eq!(Some(42), request_value::<u8>(&child));
    let parent = Parent(None);
    let child = Child { parent };
    assert_eq!(Some(99), request_value::<u8>(&child));
}
