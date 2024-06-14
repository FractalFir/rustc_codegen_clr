use cilly::field_desc::FieldDescriptor;
use rustc_middle::{
    mir::interpret::AllocId,
    ty::{
        AdtDef, Const, ConstKind, EarlyBinder, GenericArg, Instance, List, ParamEnv, SymbolName,
        Ty, TyCtxt, TyKind, TypeFoldable,
    },
};
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}

use crate::{r#type::TyCache, IString};
pub mod adt;
pub fn as_adt(ty: Ty) -> Option<(AdtDef, &List<GenericArg>)> {
    match ty.kind() {
        TyKind::Adt(adt, subst) => Some((*adt, subst)),
        _ => None,
    }
}
pub fn adt_name<'tyctx>(
    adt: AdtDef<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    gargs: &'tyctx List<GenericArg<'tyctx>>,
) -> crate::IString {
    //TODO: find a better way to get adt name!
    let _gdef_str = if gargs
        .iter()
        .any(|garg| garg.as_type().is_some() || garg.as_const().is_some())
    {
        rustc_middle::ty::print::with_no_trimmed_paths! {tyctx.def_path_str_with_args(adt.did(),gargs)}
    } else {
        rustc_middle::ty::print::with_no_trimmed_paths! {tyctx.def_path_str(adt.did())}
    };
    let krate = adt.did().krate;
    let adt_instance = Instance::resolve(tyctx, ParamEnv::reveal_all(), adt.did(), gargs)
        .unwrap()
        .unwrap();
    // Get the mangled path: it is absolute, and not poluted by types being rexported
    let auto_mangled =
        rustc_symbol_mangling::symbol_name_for_instance_in_crate(tyctx, adt_instance, krate);
    // Then, demangle the type name, converting it to a Rust-style one (eg. `core::option::Option::h8zc8s`)
    let demangled = rustc_demangle::demangle(&auto_mangled);
    // Using formating preserves the generic hash.
    let demangled = format!("{demangled}");
    // Replace Rust namespace(module) spearators with C# ones.
    let dotnet_class_name = demangled.replace("::", ".");
    escape_class_name(&dotnet_class_name)
}
pub fn escape_class_name(name: &str) -> IString {
    name.replace('\'', "_ap_").into()
}
/// Gets the name of a field with index `idx`
pub fn field_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let field_def = adt_def
                .all_fields()
                .nth(idx as usize)
                .expect("Field index out of range.");
            crate::r#type::escape_field_name(&field_def.name.to_string())
        }
        TyKind::Tuple(_) => format!("Item{}", idx + 1).into(),
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
/// Gets the name of a enum variant with index `idx`
pub fn variant_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let variant_def = &adt_def.variants()[idx.into()];
            variant_def.name.to_string().into()
        }
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
/// Escapes the name of a function
pub fn function_name(name: SymbolName) -> crate::IString {
    let mut name: String = name.to_string();
    // Name TOO long
    if *crate::config::ESCAPE_NAMES {
        name = name.replace('.', "_dot_").replace('$', "_ds_");
    }
    if name.len() > 1000 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        //TODO: make hashes consitant!
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        format!("{}_{}", &name[..1000], calculate_hash(&name)).into()
    } else {
        name.into()
    }
}
/// Monomorphizes type `ty`
pub fn monomorphize<'tyctx, T: TypeFoldable<TyCtxt<'tyctx>> + Clone>(
    instance: &Instance<'tyctx>,
    ty: T,
    ctx: TyCtxt<'tyctx>,
) -> T {
    instance.instantiate_mir_and_normalize_erasing_regions(
        ctx,
        ParamEnv::reveal_all(),
        EarlyBinder::bind(ty),
    )
}
pub fn enum_field_descriptor<'ctx>(
    owner_ty: Ty<'ctx>,
    field_idx: u32,
    variant_idx: u32,
    ctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
    type_cache: &mut TyCache,
) -> FieldDescriptor {
    let (adt, subst) = as_adt(owner_ty).expect("Tried to get a field of a non ADT or tuple type!");
    let variant = adt
        .variants()
        .iter()
        .nth(variant_idx as usize)
        .expect("No enum variant with such index!");
    let field = variant
        .fields
        .iter()
        .nth(field_idx as usize)
        .expect("No enum field with provided index!");
    let variant_name: IString = variant.name.to_string().into();
    let field_name = format!(
        "{variant_name}_{fname}",
        fname = crate::r#type::escape_field_name(&field.name.to_string())
    )
    .into();
    let field_ty = field.ty(ctx, subst);
    let field_ty = crate::utilis::monomorphize(&method_instance, field_ty, ctx);
    let field_ty = type_cache.type_from_cache(field_ty, ctx, method_instance);
    let owner_ty = type_cache
        .type_from_cache(owner_ty, ctx, method_instance)
        .as_dotnet()
        .expect("Error: tried to set a field of a non-object type!");

    FieldDescriptor::new(owner_ty, field_ty, field_name)
}
pub fn field_descrptor<'tyctx>(
    owner_ty: Ty<'tyctx>,
    field_idx: u32,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> FieldDescriptor {
    if let TyKind::Tuple(elements) = owner_ty.kind() {
        let element = elements[field_idx as usize];
        let element = monomorphize(&method_instance, element, tyctx);
        let element = type_cache.type_from_cache(element, tyctx, method_instance);
        return FieldDescriptor::new(
            crate::r#type::simple_tuple(
                &elements
                    .iter()
                    .map(|tpe| {
                        let tpe = crate::utilis::monomorphize(&method_instance, tpe, tyctx);
                        type_cache.type_from_cache(tpe, tyctx, method_instance)
                    })
                    .collect::<Vec<_>>(),
            ),
            element,
            format!("Item{}", field_idx + 1).into(),
        );
    } else if let TyKind::Closure(_, args) = owner_ty.kind() {
        let closure = args.as_closure();
        let field_type = closure
            .upvar_tys()
            .iter()
            .nth(field_idx as usize)
            .expect("Could not find closure fields!");
        let field_type = crate::utilis::monomorphize(&method_instance, field_type, tyctx);
        let field_type = type_cache.type_from_cache(field_type, tyctx, method_instance);
        let owner_ty = crate::utilis::monomorphize(&method_instance, owner_ty, tyctx);
        let owner_type = type_cache.type_from_cache(owner_ty, tyctx, method_instance);
        let field_name = format!("f_{field_idx}").into();
        return FieldDescriptor::new(
            owner_type.as_dotnet().expect("Closure type invalid!"),
            field_type,
            field_name,
        );
    }
    let (adt, subst) = as_adt(owner_ty).expect("Tried to get a field of a non ADT or tuple type!");
    let field = adt
        .all_fields()
        .nth(field_idx as usize)
        .expect("No field with provided index!");
    let field_name = crate::r#type::escape_field_name(&field.name.to_string());
    let field_ty = field.ty(tyctx, subst);
    let field_ty = crate::utilis::monomorphize(&method_instance, field_ty, tyctx);
    let field_ty = type_cache.type_from_cache(field_ty, tyctx, method_instance);
    let owner_ty = type_cache
        .type_from_cache(owner_ty, tyctx, method_instance)
        .as_dotnet()
        .expect("Error: tried to set a field of a non-object type!");
    FieldDescriptor::new(owner_ty, field_ty, field_name)
}

/// Tires to get the value of Const `size` as usize.
pub fn try_resolve_const_size(size: Const) -> Result<usize, &'static str> {
    let scalar = match size.try_to_scalar() {
        Some(value) => Ok(value),
        None => Err("Can't resolve scalar array size!"),
    }?;
    let value = scalar.to_u64().expect("Could not convert scalar to u64!");
    Ok(usize::try_from(value).expect("Const size value too big."))
}

/// Converts a generic argument to a string, and panics if it could not.
pub fn garg_to_string<'tyctx>(garg: GenericArg<'tyctx>, ctx: TyCtxt<'tyctx>) -> String {
    let str_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let kind = str_const.kind();
    match kind {
        ConstKind::Value(ty, val_tree) => {
            let raw_bytes = val_tree
                .try_to_raw_bytes(ctx, ty)
                .expect("String const did not contain valid string!");
            let tpe = ty
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
/// Converts a generic argument to a boolean, and panics if it could not.
pub fn garag_to_bool<'tyctx>(garg: GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> bool {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");

    let kind = usize_const.kind();
    match kind {
        ConstKind::Value(ty, val_tree) => {
            let scalar = val_tree
                .try_to_scalar_int()
                .expect("String const did not contain valid scalar!");
            let tpe = ty;
            assert!(
                tpe.is_bool(),
                "Generic argument was not a bool type! ty:{tpe:?}"
            );
            scalar.to_uint(scalar.size()) != 0
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
/// This function returns the size of a type at the compile time. This should be used ONLY for handling constants. It currently assumes a 64 bit env
pub fn compiletime_sizeof<'tyctx>(ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> u64 {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.size.bytes()
}
/// Ensures that a type is morphic.
#[macro_export]
macro_rules! assert_morphic {
    ($ty:ident) => {
        let ty_kind = $ty.kind();
        debug_assert!(
            !matches!(ty_kind, TyKind::Alias(_, _)),
            "ERROR: NON MORPHIC TYPE(ALIAS TYPE) {ty:?} WHERE MORPHIC TYPE EXPECTED!",
            ty = $ty
        );
        debug_assert!(
            !matches!(ty_kind, TyKind::Param(_)),
            "ERROR: NON MORPHIC TYPE(GENERIC PARAM TYPE) {ty:?} WHERE MORPHIC TYPE EXPECTED!",
            ty = $ty
        );
    };
}

pub(crate) fn alloc_id_to_u64(alloc_id: AllocId) -> u64 {
    alloc_id.0.into()
}
pub fn is_fn_intrinsic<'tyctx>(fn_ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> bool {
    match fn_ty.kind() {
        TyKind::FnDef(did, _) => tyctx.is_intrinsic(*did, tyctx.item_name(*did)),
        TyKind::Closure(_, _) => false,
        _ => todo!("Can't get signature of {fn_ty}"),
    }
}
pub fn align_of<'tyctx>(ty: rustc_middle::ty::Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> u64 {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;

    let align = layout.align.abi;
    align.bytes()
}
pub fn is_zst<'tyctx>(ty: rustc_middle::ty::Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> bool {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.is_zst()
}
pub fn requries_align_adjustement<'tyctx>(
    ty: rustc_middle::ty::Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
) -> Option<u64> {
    //TODO: some types requre aligement smaller than 16 bytes but larger than their size. Handle that. Requires reimplemting .NETs algiement clacualtions.
    let align = align_of(ty, tyctx);
    if align > 16 {
        Some(align)
    } else {
        None
    }
}
pub fn is_unsized<'tyctx>(ty: rustc_middle::ty::Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> bool {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.is_unsized()
}
