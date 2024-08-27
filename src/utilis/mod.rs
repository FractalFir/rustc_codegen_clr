use crate::{assembly::MethodCompileCtx, IString};
use cilly::{field_desc::FieldDescriptor, utilis::escape_class_name};
use rustc_middle::{
    mir::interpret::AllocId,
    ty::{
        AdtDef, Const, ConstKind, EarlyBinder, GenericArg, Instance, List, ParamEnv, SymbolName,
        Ty, TyCtxt, TyKind, TypeFoldable,
    },
};
pub mod adt;
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub const MANAGED_LD_LEN: &str = "rustc_clr_interop_managed_ld_len";
pub const MANAGED_LD_NULL: &str = "rustc_clr_interop_managed_ld_null";
pub const MANAGED_CHECKED_CAST: &str = "rustc_clr_interop_managed_checked_cast";
pub const MANAGED_IS_INST: &str = "rustc_clr_interop_managed_is_inst";
pub const MANAGED_LD_ELEM_REF: &str = "rustc_clr_interop_managed_ld_elem_ref";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}
pub fn as_adt(ty: Ty) -> Option<(AdtDef, &List<GenericArg>)> {
    match ty.kind() {
        TyKind::Adt(adt, subst) => Some((*adt, subst)),
        _ => None,
    }
}
pub fn adt_name<'tcx>(
    adt: AdtDef<'tcx>,
    tcx: TyCtxt<'tcx>,
    gargs: &'tcx List<GenericArg<'tcx>>,
) -> crate::IString {
    //TODO: find a better way to get adt name!
    let _gdef_str = if gargs
        .iter()
        .any(|garg| garg.as_type().is_some() || garg.as_const().is_some())
    {
        rustc_middle::ty::print::with_no_trimmed_paths! {tcx.def_path_str_with_args(adt.did(),gargs)}
    } else {
        rustc_middle::ty::print::with_no_trimmed_paths! {tcx.def_path_str(adt.did())}
    };
    let krate = adt.did().krate;
    let adt_instance = Instance::try_resolve(tcx, ParamEnv::reveal_all(), adt.did(), gargs)
        .unwrap()
        .unwrap();
    // Get the mangled path: it is absolute, and not poluted by types being rexported
    let auto_mangled =
        rustc_symbol_mangling::symbol_name_for_instance_in_crate(tcx, adt_instance, krate);
    // Then, demangle the type name, converting it to a Rust-style one (eg. `core::option::Option::h8zc8s`)
    let demangled = rustc_demangle::demangle(&auto_mangled);
    // Using formating preserves the generic hash.
    let demangled = format!("{demangled}");
    // Replace Rust namespace(module) spearators with C# ones.
    let dotnet_class_name = demangled.replace("::", ".");
    escape_class_name(&dotnet_class_name).into()
}

/// Gets the name of a field with index `idx`
pub fn field_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let field_def = adt_def
                .all_fields()
                .nth(idx as usize)
                .expect("Field index out of range.");
            crate::r#type::escape_field_name(&field_def.name.to_string()).into()
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
pub fn function_name(name: SymbolName) -> IString {
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
pub fn monomorphize<'tcx, T: TypeFoldable<TyCtxt<'tcx>> + Clone>(
    instance: &Instance<'tcx>,
    ty: T,
    ctx: TyCtxt<'tcx>,
) -> T {
    instance.instantiate_mir_and_normalize_erasing_regions(
        ctx,
        ParamEnv::reveal_all(),
        EarlyBinder::bind(ty),
    )
}
pub fn enum_field_descriptor<'tcx>(
    owner_ty: Ty<'tcx>,
    field_idx: u32,
    variant_idx: u32,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
    let field_ty = field.ty(ctx.tcx(), subst);
    let field_ty = ctx.monomorphize(field_ty);
    let field_ty = ctx.type_from_cache(field_ty);
    let owner_ty = ctx
        .type_from_cache(owner_ty)
        .as_class_ref()
        .expect("Error: tried to set a field of a non-object type!");

    FieldDescriptor::new(owner_ty, field_ty, field_name)
}
pub fn field_descrptor<'tcx>(
    owner_ty: Ty<'tcx>,
    field_idx: u32,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> FieldDescriptor {
    if let TyKind::Tuple(elements) = owner_ty.kind() {
        let element = elements[field_idx as usize];
        let element = ctx.monomorphize(element);
        let element = ctx.type_from_cache(element);
        return FieldDescriptor::new(
            crate::r#type::simple_tuple(
                &elements
                    .iter()
                    .map(|tpe| {
                        let tpe = ctx.monomorphize(tpe);
                        ctx.type_from_cache(tpe)
                    })
                    .collect::<Vec<_>>(),
                ctx.asm_mut(),
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
        let field_type = ctx.monomorphize(field_type);
        let field_type = ctx.type_from_cache(field_type);
        let owner_ty = ctx.monomorphize(owner_ty);
        let owner_type = ctx.type_from_cache(owner_ty);
        let field_name = format!("f_{field_idx}").into();
        return FieldDescriptor::new(
            owner_type.as_class_ref().expect("Closure type invalid!"),
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
    let field_ty = field.ty(ctx.tcx(), subst);
    let field_ty = ctx.monomorphize(field_ty);
    let field_ty = ctx.type_from_cache(field_ty);
    let owner_ty = ctx
        .type_from_cache(owner_ty)
        .as_class_ref()
        .expect("Error: tried to set a field of a non-object type!");
    FieldDescriptor::new(owner_ty, field_ty, field_name.into())
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
pub fn garg_to_string<'tcx>(garg: GenericArg<'tcx>, ctx: TyCtxt<'tcx>) -> IString {
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
            String::from_utf8(raw_bytes.into())
                .expect("String constant invalid!")
                .into()
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
/// Converts a generic argument to a boolean, and panics if it could not.
pub fn garag_to_bool<'tcx>(garg: GenericArg<'tcx>, _ctx: TyCtxt<'tcx>) -> bool {
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
pub fn compiletime_sizeof<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> u64 {
    let layout = tcx
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
pub fn is_fn_intrinsic<'tcx>(fn_ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    match fn_ty.kind() {
        TyKind::FnDef(did, _) => tcx.is_intrinsic(*did, tcx.item_name(*did)),
        TyKind::Closure(_, _) => false,
        _ => todo!("Can't get signature of {fn_ty}"),
    }
}
pub fn align_of<'tcx>(ty: rustc_middle::ty::Ty<'tcx>, tcx: TyCtxt<'tcx>) -> u64 {
    let layout = tcx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;

    let align = layout.align.abi;
    align.bytes()
}
pub fn is_zst<'tcx>(ty: rustc_middle::ty::Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let layout = tcx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.is_zst()
}
pub fn requries_align_adjustement<'tcx>(
    ty: rustc_middle::ty::Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> Option<u64> {
    //TODO: some types requre aligement smaller than 16 bytes but larger than their size. Handle that. Requires reimplementing .NETs algiement clacualtions.
    let align = align_of(ty, tcx);
    if align > 8 {
        Some(align)
    } else {
        None
    }
}
