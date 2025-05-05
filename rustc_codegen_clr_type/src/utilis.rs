use crate::r#type::get_type;
use cilly::{
    Assembly, ClassRef, Int, MethodRef, Type, bimap::Interned, call, cil_node::V1Node,
    cilnode::MethodKind, utilis::escape_class_name,
};
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_middle::ty::Const;
use rustc_middle::ty::List;
use rustc_middle::ty::{
    AdtDef, ConstKind, EarlyBinder, GenericArg, Instance, PseudoCanonicalInput, Ty, TyCtxt, TyKind,
    TypeFoldable,
};
use rustc_span::def_id::DefId;

/// This struct represetnts either a primitive .NET type (F32,F64), or stores information on how to lookup a more complex type (struct,class,array)
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub struct DotnetArray {
    pub element: Type,
    pub dimensions: u64,
}

#[must_use]
pub fn max_value(tpe: &Type, asm: &mut Assembly) -> V1Node {
    match tpe {
        Type::Int(Int::USize) => {
            let mref = MethodRef::new(
                ClassRef::usize_type(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        Type::Int(Int::U64) => V1Node::V2(asm.alloc_node(u64::MAX)),
        Type::Int(Int::U32) => V1Node::V2(asm.alloc_node(u32::MAX)),
        Type::Int(Int::U16) => V1Node::V2(asm.alloc_node(u16::MAX)),
        Type::Int(Int::U8) => V1Node::V2(asm.alloc_node(u8::MAX)),
        Type::Int(Int::I64) => V1Node::V2(asm.alloc_node(i64::MAX)),
        Type::Int(Int::I32) => V1Node::V2(asm.alloc_node(i32::MAX)),
        Type::Int(Int::I16) => V1Node::V2(asm.alloc_node(i16::MAX)),
        Type::Int(Int::I8) => V1Node::V2(asm.alloc_node(i8::MAX)),
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
pub const INTEROP_CLASS_TPE_NAME: &str = "RustcCLRInteropManagedClass";
pub const INTEROP_STRUCT_TPE_NAME: &str = "RustcCLRInteropManagedStruct";
pub const INTEROP_CHR_TPE_NAME: &str = "RustcCLRInteropManagedChar";
pub const INTEROP_ARR_TPE_NAME: &str = "RustcCLRInteropManagedArray";
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed object reference must have exactly 2 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx.tcx());
        let assembly = Some(assembly)
            .filter(|assembly| !assembly.is_empty())
            .map(|a| ctx.alloc_string(a));
        let name = garg_to_string(subst[1], ctx.tcx());
        let name = ctx.alloc_string(name);
        let dotnet_tpe = ctx.alloc_class_ref(ClassRef::new(name, assembly, false, [].into()));
        Type::ClassRef(dotnet_tpe)
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        assert!(
            subst.len() == 3,
            "Managed struct reference must have exactly 3 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx.tcx());
        let assembly = Some(assembly)
            .filter(|assembly| !assembly.is_empty())
            .map(|a| ctx.alloc_string(a));
        let name = garg_to_string(subst[1], ctx.tcx());
        let name = ctx.alloc_string(name);
        let dotnet_tpe = ctx.alloc_class_ref(ClassRef::new(name, assembly, true, [].into()));
        Type::ClassRef(dotnet_tpe)
    } else if name.contains(INTEROP_ARR_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed array reference must have exactly 2 generic arguments: type and dimension count!"
        );
        let element = &subst[0].as_type().expect("Array type must be specified!");
        let element = ctx.monomorphize(*element);
        let dimensions = garag_to_usize(subst[1], ctx.tcx());
        let element = get_type(element, ctx);

        Type::PlatformArray {
            elem: ctx.alloc_type(element),
            dims: std::num::NonZeroU8::new(dimensions.try_into().unwrap()).unwrap(),
        }
    } else if name.contains(INTEROP_CHR_TPE_NAME) {
        Type::PlatformChar
    } else {
        todo!("Interop type {name:?} is not yet supported!")
    }
}
#[must_use]
pub fn garag_to_usize<'tcx>(garg: GenericArg<'tcx>, _ctx: TyCtxt<'tcx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let kind = usize_const.kind();
    match kind {
        ConstKind::Value(val) => {
            let scalar = val
                .valtree
                .try_to_scalar_int()
                .expect("String const did not contain valid scalar!");
            let ty = val.ty;
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
#[must_use]
pub fn tuple_name(elements: &[Type], asm: &Assembly) -> String {
    let generics: String = elements.iter().map(|t| t.mangle(asm)).collect();
    format!(
        "Tuple{generic_count}{generics}",
        generic_count = generics.len()
    )
}
/// Creates a tuple with no more than 8 elements.
#[must_use]
pub fn simple_tuple(elements: &[cilly::Type], asm: &mut Assembly) -> Interned<ClassRef> {
    let name = tuple_name(elements, asm);
    let name = asm.alloc_string(name);
    asm.alloc_class_ref(ClassRef::new(name, None, true, [].into()))
}

#[must_use]
pub fn is_fat_ptr<'tcx>(
    ptr_type: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: rustc_middle::ty::Instance<'tcx>,
) -> bool {
    use rustc_abi::BackendRepr;
    let ptr_type = monomorphize(&method, ptr_type, tcx);
    let layout = tcx
        .layout_of(PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: ptr_type,
        })
        .expect("Can't get layout of a type.")
        .layout;
    let abi = layout.0.0.backend_repr;
    match abi {
        BackendRepr::Scalar(_) => false,
        BackendRepr::ScalarPair(_, _) => true,
        _ => panic!("Unexpected abi of pointer to {ptr_type:?}. The ABI was:{abi:?}"),
    }
}
/// Monomorphizes type `ty`
pub fn monomorphize<'tcx, T: TypeFoldable<TyCtxt<'tcx>> + Clone>(
    instance: &Instance<'tcx>,
    ty: T,
    ctx: TyCtxt<'tcx>,
) -> T {
    instance.instantiate_mir_and_normalize_erasing_regions(
        ctx,
        rustc_middle::ty::TypingEnv::fully_monomorphized(),
        EarlyBinder::bind(ty),
    )
}
/// Converts a generic argument to a string, and panics if it could not.
pub fn garg_to_string<'tcx>(garg: GenericArg<'tcx>, ctx: TyCtxt<'tcx>) -> String {
    let str_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let kind = str_const.kind();
    match kind {
        ConstKind::Value(val) => {
            let raw_bytes = val
                .try_to_raw_bytes(ctx)
                .expect("String const did not contain valid string!");
            let tpe = val
                .ty
                .builtin_deref(true)
                .expect("Type of generic argument was not a reference, can't resolve as string!");
            assert!(
                tpe.is_str(),
                "Generic argument was not a string, but {str_const:?}!"
            );
            String::from_utf8(raw_bytes.into()).expect("String constant invalid!")
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
#[must_use]
pub fn pointer_to_is_fat<'tcx>(
    pointed_type: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: rustc_middle::ty::Instance<'tcx>,
) -> bool {
    is_fat_ptr(
        Ty::new_ptr(tcx, pointed_type, rustc_hir::Mutability::Mut),
        tcx,
        method,
    )
}

pub fn is_zst<'tcx>(ty: rustc_middle::ty::Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let layout = tcx
        .layout_of(PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.is_zst()
}
pub fn adt_name<'tcx>(
    adt: AdtDef<'tcx>,
    tcx: TyCtxt<'tcx>,
    gargs: &'tcx List<GenericArg<'tcx>>,
) -> String {
    //TODO: find a better way to get adt instances!
    let krate = adt.did().krate;
    let adt_instance = instance_try_resolve(adt.did(), tcx, gargs);
    // Get the mangled path: it is absolute, and not poluted by types being rexported
    let auto_mangled =
        rustc_symbol_mangling::symbol_name_for_instance_in_crate(tcx, adt_instance, krate);
    // Then, demangle the type name, converting it to a Rust-style one (eg. `core::option::Option::h8zc8s`)
    let demangled = rustc_demangle::demangle(&auto_mangled);
    // Using formating preserves the generic hash.
    let demangled = format!("{demangled}");
    // Replace Rust namespace(module) spearators with C# ones.
    let dotnet_class_name = demangled.replace("::", ".");
    escape_class_name(&dotnet_class_name)
}
// WARNING: this is *wrong*: For some reason, `Instance::try_resolve` should not operate on structs(why?), and this just silences the newly introduced warning.
pub fn instance_try_resolve<'tcx>(
    adt: DefId,
    tcx: TyCtxt<'tcx>,
    gargs: &'tcx List<GenericArg<'tcx>>,
) -> Instance<'tcx> {
    tcx.resolve_instance_raw(PseudoCanonicalInput {
        typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
        value: (adt, gargs),
    })
    .unwrap()
    .unwrap()
}
/// Tries to get the value of Const `size` as usize.
pub fn try_resolve_const_size(size: Const) -> Result<usize, &'static str> {
    let value = match size.try_to_value() {
        Some(value) => Ok(value),
        None => Err("Can't resolve scalar array size!"),
    }?;
    let value = value
        .valtree
        .try_to_scalar()
        .unwrap()
        .to_u64()
        .expect("Could not convert scalar to u64!");
    Ok(usize::try_from(value).expect("Const size value too big."))
}
