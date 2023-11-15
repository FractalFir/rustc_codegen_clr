use crate::{
    codegen_error::{MethodCodegenError, CodegenError}, r#type::Type, utilis::skip_binder_if_no_generic_types,
};
use rustc_target::abi::call::Conv;
use rustc_middle::ty::{Instance,List, PolyFnSig, TyCtxt,ParamEnvAnd,ParamEnv};
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
        method_instance: &Instance<'tcx>,
    ) -> Result<Self, MethodCodegenError> {
        println!("sig:{sig:?}");

        let inputs = skip_binder_if_no_generic_types(sig.inputs())?
            .iter()
            .map(|v| {
                //println!("arg:{v:?}");
                let tmp = Type::from_ty(*v, tcx, method_instance);
                //println!("endarg");
                tmp
            })
            .collect();
        let out = skip_binder_if_no_generic_types(sig.output())?;
        //println!("out:{out:?}");
        let output = Type::from_ty(out, tcx, method_instance);
        Ok(Self { inputs, output })
    }
    /// Tries to create a signature using `from_poly_sig`. Will panic if `ABORT_ON_ERROR` is set.
    pub fn try_from_poly_sig<'tcx>(
        sig: &PolyFnSig<'tcx>,
        tcx: TyCtxt<'tcx>,
        method_instance: &Instance<'tcx>,
    ) -> Result<Self, crate::codegen_error::CodegenError> {
        if crate::ABORT_ON_ERROR {
            Self::from_poly_sig(sig, tcx, method_instance).map_err(|err| err.into())
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                Self::from_poly_sig(sig, tcx, method_instance)
            })) {
                Ok(inner_res) => Ok(inner_res?),
                Err(payload) => {
                    if let Some(msg) = payload.downcast_ref::<&str>() {
                        Err(crate::codegen_error::CodegenError::from_panic_message(msg))
                    } else {
                        Err(crate::codegen_error::CodegenError::from_panic_message(
                            "try_from_poly_sig panicked with a non-string message!",
                        ))
                    }
                }
            }
        }
    }
    /// Returns the sigmnature of function behind `function`.
    pub fn sig_from_instance<'tcx>(function:Instance<'tcx>,tcx: TyCtxt<'tcx>)->Result<Self,CodegenError>{
        let fn_abi = tcx.fn_abi_of_instance(ParamEnvAnd{param_env:ParamEnv::reveal_all(),value:(function,List::empty())});
        let fn_abi = match fn_abi{
            Ok(abi)=>abi,
            Err(error)=>todo!(),
        };
        let conv = fn_abi.conv;
        match conv{
            Conv::Rust=>(),
            Conv::C=>(),
            _=>panic!("ERROR:calling using convention {conv:?} is not supported!"),
        }
        assert!(!fn_abi.c_variadic);
        let ret = Type::from_ty(fn_abi.ret.layout.ty,tcx,&function);
        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in fn_abi.args.iter(){
            args.push(Type::from_ty(arg.layout.ty,tcx,&function));
        }
        Ok(Self{
            inputs:args,
            output:ret,
        })
    }
    /// Returns the list of function inputs.
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }
    /// Returns the function output.
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
