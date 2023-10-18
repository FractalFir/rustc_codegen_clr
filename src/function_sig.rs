use crate::{
    codegen_error::MethodCodegenError, r#type::Type, utilis::skip_binder_if_no_generic_types,
};
use rustc_middle::ty::{PolyFnSig, TyCtxt};
use serde::{Deserialize, Serialize};
/// Function signature.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct FnSig {
    inputs: Vec<Type>,
    output: Type,
}
impl FnSig {
    /// Creates a function signature from a fn sig. Does not morphize!
    pub fn from_poly_sig<'tcx>(
        sig: &PolyFnSig<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Result<Self, MethodCodegenError> {
        println!("sig:{sig:?}");

        let inputs = skip_binder_if_no_generic_types(sig.inputs())?
            .iter()
            .map(|v| {
                println!("arg:{v:?}");
                let tmp = Type::from_ty(*v, tcx);
                println!("endarg");
                tmp
            })
            .collect();
        let out = skip_binder_if_no_generic_types(sig.output())?;
        println!("out:{out:?}");
        let output = Type::from_ty(out, tcx);
        Ok(Self { inputs, output })
    }
    /// Creates a function signature from a fn sig, using the parrent method to morphize the call
    pub fn from_poly_sig_mono<'tcx>(
        sig: &PolyFnSig<'tcx>,
        tcx: TyCtxt<'tcx>,
        method: &rustc_middle::ty::Instance<'tcx>,
    ) -> Result<Self, MethodCodegenError> {
        let inputs = skip_binder_if_no_generic_types(sig.inputs())?
            .iter()
            .map(|v| Type::from_ty(crate::utilis::monomorphize(method, *v, tcx), tcx))
            .collect();
        //
        let output = Type::from_ty(
            crate::utilis::monomorphize(
                method,
                skip_binder_if_no_generic_types(sig.output())?,
                tcx,
            ),
            tcx,
        );
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
