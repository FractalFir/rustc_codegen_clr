pub type Console = crate::intrinsics::RustcCLRInteropManagedClass<"System.Console","System.Console">;
use crate::system::MString;
impl Console{
    pub fn writeln_string(string:MString){
        Self::static1_::<"WriteLine",MString,()>( string)
    }
}