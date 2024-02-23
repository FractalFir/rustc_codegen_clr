use crate::{codegen_error::CodegenError, function_sig::FnSig, r#type::TyCache};
use rustc_middle::ty::{Instance, List, ParamEnv, ParamEnvAnd, TyCtxt, TyKind};
use rustc_target::abi::call::Conv;
use rustc_target::spec::abi::Abi as TargetAbi;
pub struct CallInfo {
    sig: FnSig,
    has_track_caller: bool,
    split_last_tuple: bool,
}
impl CallInfo {
    /// Returns the signature of function behind `function`.
    pub fn sig_from_instance_<'tyctx>(
        function: Instance<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
        tycache: &mut TyCache,
    ) -> Result<Self, CodegenError> {
        let fn_abi = tyctx.fn_abi_of_instance(ParamEnvAnd {
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
        //assert!(!fn_abi.c_variadic);
        let ret = tycache.type_from_cache(fn_abi.ret.layout.ty, tyctx, Some(function));
        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in fn_abi.args.iter() {
            args.push(tycache.type_from_cache(arg.layout.ty, tyctx, Some(function)));
        }
        // There are 2 ABI enums for some reasons(they differ in what memebers they have)
        let fn_ty = function.ty(tyctx, ParamEnv::reveal_all());
        let internal_abi = match fn_ty.kind() {
            TyKind::FnDef(_, _) => fn_ty.fn_sig(tyctx),
            TyKind::Closure(_, args) => args.as_closure().sig(),
            _ => todo!("Can't get signature of {fn_ty}"),
        }
        .abi();
        // Only those ABIs are supported
        let split_last_tuple = match internal_abi {
            TargetAbi::C { unwind: _ } => false,
            TargetAbi::Cdecl { unwind: _ } => false,
            TargetAbi::RustIntrinsic => false,
            TargetAbi::Rust => false,
            TargetAbi::RustCold => false,
            TargetAbi::RustCall => true, /*Err(CodegenError::FunctionABIUnsuported(
            "\"rust_call\" ABI, used for things like clsoures, is not supported yet!",
            ))?,*/
            _ => todo!("Unsuported ABI:{internal_abi:?}"),
        };
        let mut sig = FnSig::new(&args, &ret);
        if fn_abi.c_variadic {
            let remaining = fn_abi.args[(fn_abi.fixed_count as usize)..]
                .iter()
                .map(|ty| {
                    tycache.type_from_cache(
                        crate::utilis::monomorphize(&function, ty.layout.ty, tyctx),
                        tyctx,
                        Some(function),
                    )
                });
            let mut inputs = sig.inputs().to_vec();
            inputs.extend(remaining);
            sig.set_inputs(inputs);
        }
        let has_track_caller = false;
        Ok(Self {
            sig,
            has_track_caller,
            split_last_tuple,
        })
    }

    pub fn sig(&self) -> &FnSig {
        &self.sig
    }

    fn has_track_caller(&self) -> bool {
        self.has_track_caller
    }

    pub fn split_last_tuple(&self) -> bool {
        self.split_last_tuple
    }
}
