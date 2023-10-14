use rustc_middle::ty::{
    AdtDef, Binder, BoundVariableKind, Const, ConstKind, EarlyBinder, GenericArg, Instance,
    ParamEnv, SymbolName, Ty, TyCtxt, TyKind, TypeFoldable,
};
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}
use crate::codegen_error::CodegenError;
pub fn skip_binder_if_no_generic_types<T>(binder: Binder<T>) -> Result<T, CodegenError> {
    if binder
        .bound_vars()
        .iter()
        .any(|bound_var_kind| matches!(bound_var_kind, BoundVariableKind::Ty(_)))
    {
        Err(CodegenError::UnersolvedGeneric)
    } else {
        Ok(binder.skip_binder())
    }
}
pub fn adt_name(adt: &AdtDef) -> crate::IString {
    //TODO: find a better way to get adt name!
    format!("{adt:?}").replace("::", ".").into()
}
pub fn field_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let field_def = adt_def
                .all_fields()
                .nth(idx as usize)
                .expect("Field index out of range.");
            crate::type_def::escape_field_name(&field_def.name.to_string())
        }
        TyKind::Tuple(_) => format!("Item{}", idx + 1).into(),
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
pub fn variant_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let variant_def = &adt_def.variants()[idx.into()];
            variant_def.name.to_string().into()
        }
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
pub fn function_name(name: SymbolName) -> crate::IString {
    name.to_string()
        .replace('$', "_ds_")
        .replace("..", "_dd_")
        .into()
}
pub fn monomorphize<'tcx, T: TypeFoldable<TyCtxt<'tcx>> + Clone>(
    instance: &Instance<'tcx>,
    ty: T,
    ctx: TyCtxt<'tcx>,
) -> T {
    instance.subst_mir_and_normalize_erasing_regions(
        ctx,
        ParamEnv::reveal_all(),
        EarlyBinder::bind(ty),
    )
}
pub fn generic_field_ty<'ctx>(
    owner_ty: Ty<'ctx>,
    field_idx: u32,
    ctx: TyCtxt<'ctx>,
) -> crate::r#type::Type {
    match owner_ty.kind() {
        TyKind::Adt(adt_def, _) => {
            let ty = ctx
                .type_of(
                    adt_def
                        .all_fields()
                        .nth(field_idx as usize)
                        .expect("ERROR: invalid field idx")
                        .did,
                )
                .instantiate_identity();
            crate::r#type::Type::from_ty(ty, ctx)
        }
        TyKind::Tuple(_) => crate::r#type::Type::GenericArg(field_idx),
        _ => todo!("Can't get field {field_idx} belonging to type {owner_ty:?}"),
    }
}
// /pub fn polimorphize(ty: Ty<'ctx>)
pub fn enum_tag_size(variants: u64) -> u32 {
    (((u64::from(u64::BITS) - u64::from((variants).leading_zeros())) + 8 - 1) / 8) as u32
}

pub fn tag_from_enum_variants(variants: u64) -> crate::r#type::Type {
    use crate::r#type::Type;
    let var_size = enum_tag_size(variants);
    println!("variants:{variants}tag_size:{var_size}");
    match var_size {
        1 => Type::U8,
        2 => Type::U16,
        4 => Type::U32,
        8 => Type::U64,
        _ => todo!("Can't yet have {var_size} byte wide enum tag!"),
    }
}
pub fn try_resolve_const_size(size: &Const) -> Result<usize, &'static str> {
    let scalar = match size.try_to_scalar() {
        Some(value) => Ok(value),
        None => Err("Can't resolve scalar array size!"),
    }?;
    let value = scalar.to_u64().expect("Could not convert scalar to u64!");
    Ok(value as usize)
}
pub fn garg_to_string<'tyctx>(garg: &GenericArg<'tyctx>, ctx: TyCtxt<'tyctx>) -> String {
    let str_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let tpe = str_const
        .ty()
        .builtin_deref(true)
        .expect("Type of generic argument was not a reference, can't resolve as string!");
    if !tpe.ty.is_str() {
        panic!("Generic argument was not a string, but {str_const:?}!");
    } else {
        let kind = str_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let raw_bytes = value
                    .try_to_raw_bytes(ctx, str_const.ty())
                    .expect("String const did not contain valid string!");
                String::from_utf8(raw_bytes.into()).expect("String constant invalid!")
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    }
}
pub fn garag_to_bool<'tyctx>(garg: &GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> bool {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let tpe = usize_const.ty();
    if !tpe.is_bool() {
        panic!("Generic argument was not a bool type! ty:{:?}", tpe);
    } else {
        let kind = usize_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let scalar = value
                    .try_to_scalar_int()
                    .expect("String const did not contain valid scalar!");
                scalar.try_to_uint(scalar.size()).unwrap() != 0
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    }
}
