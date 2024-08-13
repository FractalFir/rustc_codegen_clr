#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    decl_macro,
    raw_os_error_ty
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![allow(dead_code, unused_imports)]
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::io::ErrorKind;
use std::io::RawOsError;
use std::path::Path;
//include!("../common.rs");
fn main() {
    std::hint::black_box(NOT_FILE_ERROR);
}
#[inline(never)]
fn open_from() -> Result<(), Error> {
    return Err(NOT_FILE_ERROR);
}
#[derive(Debug)]
pub struct Error {
    repr: Repr,
}
impl Error {
    #[inline]
    pub const fn from_static_message(msg: &'static SimpleMessage) -> Error {
        Self {
            repr: Repr::new_simple_message(msg),
        }
    }
}
#[repr(transparent)]
#[derive(Debug)]
pub struct Repr(NonNull<()>, PhantomData<ErrorData<Box<Custom>>>);

impl Repr {
    #[inline]
    pub const fn new_simple_message(m: &'static SimpleMessage) -> Self {
        // Safety: References are never null.
        Self(
            unsafe { NonNull::new_unchecked(m as *const _ as *mut ()) },
            PhantomData,
        )
    }
}
#[derive(Debug)]
#[repr(align(4))]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}
enum ErrorData<C> {
    SimpleMessage(&'static SimpleMessage),
    Custom(C),
}
#[repr(align(4))]
#[derive(Debug)]
pub struct SimpleMessage {
    kind: ErrorKind,
    message: &'static str,
}

impl SimpleMessage {
    pub const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }
}
pub const NOT_FILE_ERROR: Error = const_io_error!(
    ErrorKind::InvalidInput,
    "the source path is neither a regular file nor a symlink to a regular file",
);
pub macro const_io_error($kind:expr, $message:expr $(,)?) {
    Error::from_static_message({
        const MESSAGE_DATA: SimpleMessage = SimpleMessage::new($kind, $message);
        &MESSAGE_DATA
    })
}
