pub mod text;
pub mod runtime;
pub mod console;
pub type MString = crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime","System.String">;