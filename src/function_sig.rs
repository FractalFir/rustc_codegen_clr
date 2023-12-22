use crate::{
    codegen_error::CodegenError,
    r#type::{TyCache, Type},
};
use rustc_middle::ty::{Instance, List, ParamEnv, ParamEnvAnd, TyCtxt};
use rustc_target::abi::call::Conv;
use serde::{Deserialize, Serialize};
use rustc_target::spec::abi::Abi as TargetAbi;
/// Function signature.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct FnSig {
    inputs: Vec<Type>,
    output: Type,
}
impl FnSig {
    /// Returns the signature of function behind `function`.
    pub fn sig_from_instance_<'tcx>(
        function: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
        tycache: &mut TyCache,
    ) -> Result<Self, CodegenError> {
        let fn_abi = tcx.fn_abi_of_instance(ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: (function, List::empty()),
        });
        let fn_abi = match fn_abi {
            Ok(abi) => abi,
            Err(_error) => todo!(),
        };
        let conv = fn_abi.conv;
        match conv {
            Conv::Rust => (),
            Conv::C => (),
            _ => panic!("ERROR:calling using convention {conv:?} is not supported!"),
        }
        assert!(!fn_abi.c_variadic);
        let ret = tycache.type_from_cache(fn_abi.ret.layout.ty, tcx, Some(function));
        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in fn_abi.args.iter() {
            args.push(tycache.type_from_cache(arg.layout.ty, tcx, Some(function)));
        }
        // There are 2 ABI enums for some reasons(they differ in what memebers they have)
        let internal_abi = function.ty(tcx,ParamEnv::reveal_all()).fn_sig(tcx).abi();
        // Only those ABIs are supported
        match internal_abi{
            TargetAbi::C { unwind }=>(),
            TargetAbi::Cdecl { unwind }=>(),
            TargetAbi::RustIntrinsic=>(),
            TargetAbi::Rust =>(),
            TargetAbi::RustCold =>(),
            TargetAbi::RustCall => Err(CodegenError::FunctionABIUnsuported("\"rust_call\" ABI, used for things like clsoures, is not supported yet!"))?,
            _=>todo!("Unsuported ABI:{internal_abi:?}")
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
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct FunctionCallInfo {
    inputs: Vec<Type>,
    output: Type,
}