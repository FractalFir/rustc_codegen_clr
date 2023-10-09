pub mod text;
pub mod runtime;
pub mod console;
pub mod diagnostics;
pub type MString = crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime","System.String">;