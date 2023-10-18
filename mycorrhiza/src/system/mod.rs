pub mod console;
pub mod diagnostics;
pub mod runtime;
pub mod text;
pub type MString =
    crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime", "System.String">;
