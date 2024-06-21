use crate::{utilis::garg_to_string, IString};
use rustc_middle::ty::{GenericArg, TyCtxt};
pub struct AssemblyRef {
    name: IString,
}
impl AssemblyRef {
    pub fn decode_assembly_ref<'tcx>(arg: GenericArg<'tcx>, tcx: TyCtxt<'tcx>) -> Self {
        let name = garg_to_string(arg, tcx).into();
        Self { name }
    }
    /// Name of the assembly reference if it is extern(not empty).
    pub fn name(&self) -> Option<&str> {
        if self.name.is_empty() {
            None
        } else {
            Some(self.name.as_ref())
        }
    }
}
