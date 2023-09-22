use rustc_middle::ty::{TyCtxt,Ty,TyKind,AdtDef, Binder, BoundVariableKind,Instance,ParamEnv,EarlyBinder,TypeFoldable,SymbolName};

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
        TyKind::Adt(adt_def, subst) => {
            let field_def = adt_def
                .all_fields()
                .nth(idx as usize)
                .expect("Field index out of range.");
            field_def.name.to_string().into()
        }
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
pub fn function_name(name:SymbolName)->crate::IString{
    name.to_string().replace("$","_ds_").replace("..","_dd_").into()
}
pub fn monomorphize<'tcx,T: TypeFoldable<TyCtxt<'tcx>> + Clone,>(instance:&Instance<'tcx>,ty:T,ctx:TyCtxt<'tcx>) -> T
{
        instance.subst_mir_and_normalize_erasing_regions(
            ctx,
            ParamEnv::reveal_all(),
            EarlyBinder::bind(ty),
        )
}
