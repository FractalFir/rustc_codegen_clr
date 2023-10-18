pub type Console =
    crate::intrinsics::RustcCLRInteropManagedClass<"System.Console", "System.Console">;
use crate::system::MString;
impl Console {
    #[inline(always)]
    pub fn writeln_string(string: MString) {
        Self::static1::<"WriteLine", MString, ()>(string)
    }
    #[inline(always)]
    pub fn writeln_u64(ulong: u64) {
        Self::static1::<"WriteLine", u64, ()>(ulong)
    }
    #[inline(always)]
    pub fn writeln_f64(double: f64) {
        Self::static1::<"WriteLine", f64, ()>(double)
    }
}
