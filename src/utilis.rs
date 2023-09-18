use rustc_middle::ty::{Binder, BoundVariableKind,AdtDef};

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

