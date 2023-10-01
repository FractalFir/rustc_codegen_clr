use rustc_middle::ty::{
    AdtDef, Binder, BoundVariableKind, Const, EarlyBinder, Instance, ParamEnv, SymbolName, Ty,
    TyCtxt, TyKind, TypeFoldable,
};

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
pub fn generic_field_ty<'ctx>(owner_ty: Ty<'ctx>, field_idx: u32, ctx: TyCtxt<'ctx>) -> Ty<'ctx> {
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
            ty
        }
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
