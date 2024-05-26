use crate::cil::CallSite;
use crate::cil_tree::cil_node::CILNode;
use cilly::fn_sig::FnSig;
use crate::call;
use cilly::{DotnetTypeRef, Type};

use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::{AdtDef, ConstKind, GenericArg, Ty, TyCtxt, TyKind};
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
pub fn c_void(tyctx: TyCtxt) -> Type {
    let lang_items = tyctx.lang_items();
    let c_void = lang_items.c_void().expect("c_void not defined.");
    let name = rustc_codegen_ssa::back::symbol_export::symbol_name_for_instance_in_crate(
        tyctx,
        ExportedSymbol::NonGeneric(c_void),
        c_void.krate,
    );
    let demangled = rustc_demangle::demangle(&name);
    // Using formating preserves the generic hash.
    let name = format!("{demangled}");
    let name = crate::utilis::escape_class_name(&name);
    DotnetTypeRef::new::<&str,_>(None, name).into()
}
pub fn max_value(tpe:&Type) -> CILNode {
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
pub fn magic_type<'tyctx>(
    name: &str,
    _adt: &AdtDef<'tyctx>,
    subst: &[GenericArg<'tyctx>],
    ctx: TyCtxt<'tyctx>,
    //method: &Instance<'tyctx>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed object reference must have exactly 2 generic arguments!"
        );
        let assembly: Box<str> = garg_to_string(subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx);
        let dotnet_tpe = DotnetTypeRef::new(assembly,name).with_valuetype(false);
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed struct reference must have exactly 2 generic arguments!"
        );
        let assembly: Box<str> = garg_to_string(subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx);
        let dotnet_tpe = DotnetTypeRef::new(assembly,name);
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
fn garag_to_usize<'tyctx>(garg: GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    if usize_const.ty().is_integral() {
        let kind = usize_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let scalar = value
                    .try_to_scalar_int()
                    .expect("String const did not contain valid scalar!");
                u64::try_from(scalar.try_to_uint(scalar.size()).unwrap())
                    .expect("Scalar of type usize has value over 2^64")
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    } else {
        panic!(
            "Generic argument was not a unit type! ty:{:?}",
            usize_const.ty()
        );
    }
}
/// Creates a tuple with no more than 8 elements.
#[must_use]
pub fn simple_tuple(elements: &[Type]) -> DotnetTypeRef {
    //assert!(elements.len() <= 8,"Tuple ({elements:?}) contains more than 8 elements, so it can't be stored inside a simple tuple.");
    let name = tuple_name(elements);

    DotnetTypeRef::new::<&str,_>(None, name)
}
use crate::utilis::garg_to_string;

use super::tuple_name;
#[must_use]
pub fn pointer_to_is_fat<'tyctx>(
    mut pointed_type: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method: Option<rustc_middle::ty::Instance<'tyctx>>,
) -> bool {
    use rustc_middle::ty::ParamEnv;
    method.inspect(|method| {
        pointed_type = crate::utilis::monomorphize(method, pointed_type, tyctx);
    });

    let is_trivialy_sized = pointed_type.is_trivially_sized(tyctx);
    if is_trivialy_sized {
        // Sized types don't need fat pointers
        false
    } else {
        // TODO: PROPELY check if type is sized
        !pointed_type.is_sized(tyctx, ParamEnv::reveal_all())
        //true
    }
}
