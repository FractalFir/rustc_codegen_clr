pub type Console = crate::intrinsics::RustcCLRInteropManagedClass<"System.Console","System.Console">;
use crate::system::MString;
impl Console{
    #[inline(always)]
    pub fn writeln_string(string:MString){
        Self::static1_::<"WriteLine",MString,()>( string)
    }
    #[inline(always)]
    pub fn writeln_u64(ulong:u64){
        Self::static1_::<"WriteLine",u64,()>( ulong)
    }
    #[inline(always)]
    pub fn writeln_f64(double:f64){
        Self::static1_::<"WriteLine",f64,()>( double)
    }
}