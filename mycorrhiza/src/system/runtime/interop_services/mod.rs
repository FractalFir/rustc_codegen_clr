use crate::intrinsics::RustcCLRInteropManagedClass;

pub type Marshal = RustcCLRInteropManagedClass<
    "System.Runtime.InteropServices",
    "System.Runtime.InteropServices.Marshal",
>;
