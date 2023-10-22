use rustc_middle::ty::{
    AdtDef, Binder, BoundVariableKind, Const, ConstKind, EarlyBinder, GenericArg, Instance,
    ParamEnv, SymbolName, Ty, TyCtxt, TyKind, TypeFoldable,
};
pub const BEGIN_TRY: &str = "rustc_clr_interop_begin_try";
pub const END_TRY: &str = "rustc_clr_interop_end_try";
pub const BEGIN_CATCH: &str = "rustc_clr_interop_begin_catch";
pub const END_CATCH: &str = "rustc_clr_interop_end_catch";
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}
use crate::{codegen_error::MethodCodegenError, r#type::DotnetTypeRef};
pub fn skip_binder_if_no_generic_types<T>(binder: Binder<T>) -> Result<T, MethodCodegenError> {
    /*
    if binder
        .bound_vars()
        .iter()
        .any(|bound_var_kind| matches!(bound_var_kind, BoundVariableKind::Ty(_)))
    {
        crate::codegen_error!("Could not resolve generic!");
    } else {
        Ok(binder.skip_binder())
    }*/
    Ok(binder.skip_binder())
}
pub fn adt_name(adt: &AdtDef) -> crate::IString {
    //TODO: find a better way to get adt name!
    format!("{adt:?}").replace("::", ".").into()
}
/// Gets the name of a field with index `idx`
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
    name.to_string()
        .replace('$', "_ds_")
        .replace("..", "_dd_")
        .into()
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
/// Gets the type of field with index `field_idx`, returning a GenericArg if the types field is generic
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
            println!("Generic field type {ty:?}");
            crate::r#type::Type::from_ty(ty, ctx)
        }
        TyKind::Tuple(_) => crate::r#type::Type::GenericArg(field_idx),
        _ => todo!("Can't get field {field_idx} belonging to type {owner_ty:?}"),
    }
}
/// Returns the size of a tag of an enum with `variants` variants.
pub fn enum_tag_size(variants: u64) -> u32 {
    (((u64::from(u64::BITS) - u64::from((variants).leading_zeros())) + 8 - 1) / 8) as u32
}
/// Gets the type of the tag of enum with `variants` varinats.
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
/// Tires to get the value of Const `size` as usize.
pub fn try_resolve_const_size(size: &Const) -> Result<usize, &'static str> {
    let scalar = match size.try_to_scalar() {
        Some(value) => Ok(value),
        None => Err("Can't resolve scalar array size!"),
    }?;
    let value = scalar.to_u64().expect("Could not convert scalar to u64!");
    Ok(value as usize)
}
/// Converts a generic argument to a string, and panics if it could not.
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
/// Converts a generic argument to a boolean, and panics if it could not.
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
/// Checks if `ty` is a TyKind::Alias
pub fn is_ty_alias(ty: Ty) -> bool {
    matches!(ty.kind(), TyKind::Alias(_, _))
}
/// This function returns the size of a type at the compile time. This should be used ONLY for handling constants. It currently assumes a 64 bit env
pub fn compiletime_sizeof(ty: Ty) -> usize {
    use rustc_middle::ty::{IntTy, UintTy};
    match ty.kind() {
        TyKind::Int(int) => match int {
            IntTy::I32 => std::mem::size_of::<i32>(),
            _ => todo!("Can't compute compiletime sizeof {int:?}"),
        },
        TyKind::Uint(int) => match int {
            UintTy::U32 => std::mem::size_of::<u32>(),
            _ => todo!("Can't compute compiletime sizeof {int:?}"),
        },
        _ => todo!("Can't compute compiletime sizeof {ty:?}"),
    }
}
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
pub fn string_class()->DotnetTypeRef{
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.String");;
    string.set_valuetype(false);
    string
}
pub fn usize_class()->DotnetTypeRef{
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.UIntPtr");
    //TODO: Inwestigate this. The valuetype prefix seems to be missing from UIntPtr in compiled C# code
    string.set_valuetype(false);
    string
}
/// Translated MIR statements should have the total stack diff of 0.
pub fn check_statement(ops:&[crate::cil_op::CILOp],statement:&rustc_middle::mir::Statement){
    let mut stack = 0;
    for op in ops{
        stack += op.stack_diff();
    }
    if stack != 0{
        eprintln!("Propable miscompilation: statement {statement:?} resulted in ops {ops:?} and did not pass the stack check.");
        let mut stack = 0;
        for op in ops{
            let diff = op.stack_diff();
            eprintln!("\t{op:?} changed stack by {diff}, to {stack}");
            stack += diff;
        }
        panic!();
    }
}