//! Mycorrhiza is a Rust .NET interop framework. It is part of the `rustc_codegen_clr` project, and allows you to interact with the .NET runtime directly.
//! One of aims of the `rustc_codegen_clr` is to reuse existing Rust features and syntax to allow semless integration between Rust and the .NET runtime
//! Mycorrhiza must "look" like a normal crate from the outside, even tough it deeply interacts with `rustc_codegen_clr`. It also should be possible to
//! implement an equivalent APIs in standard Rust.

#![allow(internal_features, incomplete_features)]
#![feature(core_intrinsics, unsized_const_params)]
pub mod bindings;pub use bindings::*;
/// Very low-level interop stuff. Don't use unless you need to.
pub mod intrinsics;

/// Wrappers around types from the `System` namespace
pub mod system;
/// C# `char` type
pub type DotNetChar = crate::intrinsics::RustcCLRInteropManagedChar;

#[macro_export]
macro_rules! panic_handler {
    () => {
        #[panic_handler]
        fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
            core::intrinsics::abort();
        }
    };
}
#[macro_export]
macro_rules! start {
    () => {
        #[start]
        fn start(_argc: isize, _argv: *const *const u8) -> isize {
            main();
            0
        }
    };
    ($entry_fn:ident) => {
        #[start]
        fn start(_argc: isize, _argv: *const *const u8) -> isize {
            $entry_fn();
            0
        }
    };
}
