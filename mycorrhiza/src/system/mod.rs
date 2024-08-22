use runtime::interop_services::Marshal;

pub mod console;
pub mod diagnostics;
pub mod runtime;
pub mod text;
pub type MString =
    crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime", "System.String">;

impl From<&str> for MString {
    fn from(val: &str) -> Self {
        Marshal::static2::<"PtrToStringUTF8", isize, i32, MString>(
            val.as_ptr() as isize,
            val.len() as i32,
        )
    }
}
