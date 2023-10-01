use crate::{codegen_error::CodegenError, r#type::Type, utilis::skip_binder_if_no_generic_types};
use rustc_middle::ty::{PolyFnSig, TyCtxt};
use serde::{Deserialize, Serialize};
/// Function signature.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct FnSig {
    inputs: Vec<Type>,
    output: Type,
}
impl FnSig {
    pub fn from_poly_sig<'tcx>(
        sig: &PolyFnSig<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Result<Self, CodegenError> {
        let inputs = skip_binder_if_no_generic_types(sig.inputs())?
            .iter()
            .map(|v| Type::from_ty(*v, tcx))
            .collect();
        let output = Type::from_ty(skip_binder_if_no_generic_types(sig.output())?, tcx);
        Ok(Self { inputs, output })
    }
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }
    pub fn output(&self) -> &Type {
        &self.output
    }
    /// Creates a new function signature. For non-static functions, this must include the hidden first `this` argument!
    pub fn new(inputs: &[Type], output: &Type) -> Self {
        Self {
            inputs: inputs.into(),
            output: output.clone(),
        }
    }
}
