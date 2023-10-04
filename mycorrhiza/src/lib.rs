
//! Mycorrhiza is a Rust .NET interop framework. It is part of the `rustc_codegen_clr` project, and allows you to interact with the .NET runtime directly.
//! One of aims of the `rustc_codegen_clr` is to reuse existing Rust features and syntax to allow semless integration between Rust and the .NET runtime
//! Mycorrhiza must "look" like a normal crate from the outside, even tough it deeply interacts with `rustc_codegen_clr`. It also should be possible to
//! implement an equivalent APIs in standard Rust.
#![no_std]
#![feature(core_intrinsics,adt_const_params)]
/// Very low-level interop stuff. Don't use unless you need to.
pub mod intrinsics;
pub mod system;
pub type ManagedChar = crate::intrinsics::RustcCLRInteropManagedChar;