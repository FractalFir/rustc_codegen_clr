use cilly::{
    call, call_site::CallSite, cil_node::CILNode, fn_sig::FnSig, ldc_u32, ldc_u64,
    utilis::escape_class_name, AsmStringContainer, DotnetTypeRef, Type,
};

use rustc_middle::{
    middle::exported_symbols::ExportedSymbol,
    ty::{AdtDef, ConstKind, GenericArg, ParamEnv, Ty, TyCtxt, TyKind},
};
/// This struct represetnts either a primitive .NET type (F32,F64), or stores information on how to lookup a more complex type (struct,class,array)
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub struct DotnetArray {
    pub element: Type,
    pub dimensions: u64,
}
#[must_use]
/// Finds the `c_void` type.
/// # Panics
/// Will panic if `c_void` is not defined.
pub fn c_void(tcx: TyCtxt) -> Type {
    let lang_items = tcx.lang_items();
    let c_void = lang_items.c_void().expect("c_void not defined.");
    let name = rustc_codegen_ssa::back::symbol_export::symbol_name_for_instance_in_crate(
        tcx,
        ExportedSymbol::NonGeneric(c_void),
        c_void.krate,
    );
    let demangled = rustc_demangle::demangle(&name);
    // Using formating preserves the generic hash.
    let name = format!("{demangled}");
    let name = escape_class_name(&name);
    DotnetTypeRef::new::<&str, _>(None, name).into()
}
#[must_use]
pub fn max_value(tpe: &Type) -> CILNode {
    match tpe {
        Type::USize => call!(
            CallSite::new_extern(
                DotnetTypeRef::usize_type(),
                "get_MaxValue".into(),
                FnSig::new(&[], Type::USize),
                true
            ),
            []
        ),
        Type::U64 => ldc_u64!(u64::MAX),
        Type::U32 => ldc_u32!(u32::MAX),
        _ => todo!("Can't get the max value of {tpe:?}"),
    }
}

/// Gets the element type of a slice OR array.
/// # Panics
/// Panics if type is not a slice or an array.
#[must_use]
pub fn element_type(src: Ty<'_>) -> Ty<'_> {
    match src.kind() {
        TyKind::Slice(element) | TyKind::Array(element, _) => *element,
        _ => panic!("Can't get element type of {src:?}"),
    }
}
const INTEROP_CLASS_TPE_NAME: &str = "RustcCLRInteropManagedClass";
const INTEROP_STRUCT_TPE_NAME: &str = "RustcCLRInteropManagedStruct";
const INTEROP_CHR_TPE_NAME: &str = "RustcCLRInteropManagedChar";
const INTEROP_ARR_TPE_NAME: &str = "RustcCLRInteropManagedArray";
#[must_use]
/// Checks if a type is a magic interop type.
pub fn is_name_magic(name: &str) -> bool {
    name.contains("RustcCLRInteropManaged")
}
/// Handling of `magic` interop types.
/// # Panics
/// Will panic if interop type is invalid.
#[must_use]
pub fn magic_type<'tcx>(
    name: &str,
    _adt: &AdtDef<'tcx>,
    subst: &[GenericArg<'tcx>],
    ctx: TyCtxt<'tcx>,
    //method: &Instance<'tcx>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed object reference must have exactly 2 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx);
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx);
        let dotnet_tpe = DotnetTypeRef::new(assembly, name).with_valuetype(false);
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed struct reference must have exactly 2 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx);
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx);
        let dotnet_tpe = DotnetTypeRef::new(assembly, name);
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_ARR_TPE_NAME) {
        assert!(subst.len() == 2, "Managed array reference must have exactly 2 generic arguments: type and dimension count!");
        let element = &subst[0].as_type().expect("Array type must be specified!");
        let dimensions = garag_to_usize(subst[1], ctx);
        let _ = (element, dimensions);

        todo!()
    } else if name.contains(INTEROP_CHR_TPE_NAME) {
        Type::DotnetChar
    } else {
        todo!("Interop type {name:?} is not yet supported!")
    }
}
fn garag_to_usize<'tcx>(garg: GenericArg<'tcx>, _ctx: TyCtxt<'tcx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let kind = usize_const.kind();
    match kind {
        ConstKind::Value(ty, value) => {
            let scalar = value
                .try_to_scalar_int()
                .expect("String const did not contain valid scalar!");
            assert!(
                ty.is_integral(),
                "Generic argument was not a unit type! ty:{ty:?}",
            );
            u64::try_from(scalar.to_uint(scalar.size()))
                .expect("Scalar of type usize has value over 2^64")
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
/// Creates a tuple with no more than 8 elements.
#[must_use]
pub fn simple_tuple(elements: &[Type]) -> DotnetTypeRef {
    // Since no intering can happen at this stage, we can pass an empty AsmStringContainer safely.
    let name = tuple_name(elements);

    DotnetTypeRef::new::<&str, _>(None, name)
}
use crate::utilis::{garg_to_string, monomorphize};

use super::tuple_name;
#[must_use]
pub fn pointer_to_is_fat<'tcx>(
    pointed_type: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: rustc_middle::ty::Instance<'tcx>,
) -> bool {
    use rustc_target::abi::Abi;
    let pointed_type = monomorphize(&method, pointed_type, tcx);
    let layout = tcx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: Ty::new_ptr(tcx, pointed_type, rustc_hir::Mutability::Mut),
        })
        .expect("Can't get layout of a type.")
        .layout;
    let abi = layout.abi();
    match abi {
        Abi::Scalar(_) => false,
        Abi::ScalarPair(_, _) => true,
        _ => panic!("Unexpected abi of pointer to {pointed_type:?}. The ABI was:{abi:?}"),
    }
}
