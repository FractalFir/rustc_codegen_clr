use crate::codegen_error::CodegenError;
use cilly::{FnSig, Type};
use rustc_abi::ExternAbi as TargetAbi;
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::r#type::get_type;
use rustc_middle::ty::{Instance, List, Ty, TyCtxt, TyKind};
use rustc_target::callconv::Conv;

/// Creates a `FnSig` from ` `. May not match the result of `sig_from_instance_`!
/// Use ONLY for function pointers!
pub fn from_poly_sig<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    sig: rustc_middle::ty::FnSigTys<TyCtxt<'tcx>>,
) -> FnSig {
    let output = get_type(ctx.monomorphize(sig.output()), ctx);
    let inputs: Box<[Type]> = sig
        .inputs()
        .iter()
        .map(|input| get_type(ctx.monomorphize(*input), ctx))
        .collect();
    FnSig::new(inputs, output)
}
/// Returns the signature of function behind `function`.
pub fn sig_from_instance_<'tcx>(
    function: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Result<FnSig, CodegenError> {
    let fn_abi = ctx
        .tcx()
        .fn_abi_of_instance(rustc_middle::ty::PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: (function, List::empty()),
        });
    let fn_abi = match fn_abi {
        Ok(abi) => abi,
        Err(_error) => todo!(),
    };
    let conv = fn_abi.conv;
    match conv {
        Conv::Rust | Conv::C => (),
        _ => panic!("ERROR:calling using convention {conv:?} is not supported!"),
    }
    //assert!(!fn_abi.c_variadic);
    let ret = ctx.monomorphize(fn_abi.ret.layout.ty);
    let ret = get_type(ret, ctx);
    let mut args = Vec::with_capacity(fn_abi.args.len());
    for arg in &fn_abi.args {
        let arg = ctx.monomorphize(arg.layout.ty);
        args.push(get_type(arg, ctx));
    }
    // There are 2 ABI enums for some reasons(they differ in what memebers they have)
    let fn_ty = function.ty(
        ctx.tcx(),
        rustc_middle::ty::TypingEnv::fully_monomorphized(),
    );
    let internal_abi = match fn_ty.kind() {
        TyKind::FnDef(_, _) => fn_ty.fn_sig(ctx.tcx()),
        TyKind::Closure(_, args) => args.as_closure().sig(),
        _ => todo!("Can't get signature of {fn_ty}"),
    }
    .abi();
    // Only those ABIs are supported
    match internal_abi {
        TargetAbi::C { unwind: _ }
        | TargetAbi::Cdecl { unwind: _ }
        | TargetAbi::RustIntrinsic
        | TargetAbi::Rust
        | TargetAbi::RustCold
        | TargetAbi::RustCall => (), /*Err(CodegenError::FunctionABIUnsuported(
        "\"rust_call\" ABI, used for things like clsoures, is not supported yet!",
        ))?,*/
        _ => todo!("Unsuported ABI:{internal_abi:?}"),
    }
    Ok(FnSig::new(args, ret))
}

/// Checks if this function is variadic.
#[must_use]
pub fn is_fn_variadic<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    ty.fn_sig(tcx).skip_binder().c_variadic
}
