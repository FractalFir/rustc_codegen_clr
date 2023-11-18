use crate::{
    codegen_error::{CodegenError, MethodCodegenError},
    r#type::{Type, TyCache},
    utilis::skip_binder_if_no_generic_types,
};
use rustc_middle::ty::{Instance, List, ParamEnv, ParamEnvAnd, PolyFnSig, TyCtxt};
use rustc_target::abi::call::Conv;
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
    /// Returns the signature of function behind `function`.
    pub fn sig_from_instance<'tcx>(
        function: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Result<Self, CodegenError> {
        let fn_abi = tcx.fn_abi_of_instance(ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: (function, List::empty()),
        });
        let fn_abi = match fn_abi {
            Ok(abi) => abi,
            Err(error) => todo!(),
        };
        let conv = fn_abi.conv;
        match conv {
            Conv::Rust => (),
            Conv::C => (),
            _ => panic!("ERROR:calling using convention {conv:?} is not supported!"),
        }
        assert!(!fn_abi.c_variadic);
        let ret = Type::from_ty(fn_abi.ret.layout.ty, tcx, &function);
        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in fn_abi.args.iter() {
            args.push(Type::from_ty(arg.layout.ty, tcx, &function));
        }
        Ok(Self {
            inputs: args,
            output: ret,
        })
    }
    /// Returns the signature of function behind `function`.
    pub fn sig_from_instance_<'tcx>(
        function: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
        tycache:&mut TyCache,
    ) -> Result<Self, CodegenError> {
        let fn_abi = tcx.fn_abi_of_instance(ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: (function, List::empty()),
        });
        let fn_abi = match fn_abi {
            Ok(abi) => abi,
            Err(error) => todo!(),
        };
        let conv = fn_abi.conv;
        match conv {
            Conv::Rust => (),
            Conv::C => (),
            _ => panic!("ERROR:calling using convention {conv:?} is not supported!"),
        }
        assert!(!fn_abi.c_variadic);
        let ret = tycache.type_from_cache(fn_abi.ret.layout.ty, tcx);
        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in fn_abi.args.iter() {
            args.push(tycache.type_from_cache(arg.layout.ty, tcx));
        }
        Ok(Self {
            inputs: args,
            output: ret,
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
