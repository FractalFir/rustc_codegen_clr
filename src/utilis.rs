use rustc_middle::ty::{
    AdtDef, Binder, Const, ConstKind, EarlyBinder, GenericArg, Instance, ParamEnv, SymbolName, Ty,
    TyCtxt, TyKind, TypeFoldable,
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
use crate::{codegen_error::MethodCodegenError, r#type::{DotnetTypeRef, Type}, type_def::TypeDef, cil_op::FieldDescriptor};
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
pub fn field_descrptor<'ctx>(
    owner_ty: Ty<'ctx>,
    field_idx: u32,
    ctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
) -> FieldDescriptor {
    if let TyKind::Tuple(elements) = owner_ty.kind(){
        assert!(elements.len() < 8,"Tuples with more than 8 elements are not supported!");
        return FieldDescriptor::new(
            crate::r#type::tuple_type(&elements.iter().map(|tpe|Type::from_ty(tpe,ctx,&method_instance)).collect::<Vec<_>>()),
            Type::GenericArg(field_idx),
            format!("Item{}",field_idx + 1).into(),
        );
    }
    let defs = TypeDef::from_ty(owner_ty, ctx, &method_instance);
    let type_ref = Type::from_ty(owner_ty, ctx, &method_instance).as_dotnet().expect("Field owner not a dotnet type!");
    let def = defs.iter().last().expect("`crate::utilis::field_descrptor` called on type without a definition.");
    def.field_desc_from_rust_field_idx(type_ref,field_idx)
}
/// Gets the type of field with index `field_idx`, returning a GenericArg if the types field is generic
pub fn generic_field_ty<'ctx>(
    owner_ty: Ty<'ctx>,
    field_idx: u32,
    ctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
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
            crate::r#type::Type::from_ty(ty, ctx, &method_instance)
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
        0 => Type::Void,
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

    let val_tree = str_const
        .eval(ctx, ParamEnv::reveal_all(), None)
        .expect("Could not eval const!");
    let tpe = str_const
        .ty()
        .builtin_deref(true)
        .expect("Type of generic argument was not a reference, can't resolve as string!");
    if !tpe.ty.is_str() {
        panic!("Generic argument was not a string, but {str_const:?}!");
    } else {
        let kind = str_const.kind();
        match kind {
            ConstKind::Value(_) => {
                let raw_bytes = val_tree
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
            IntTy::I8 => std::mem::size_of::<i8>(),
            IntTy::I16 => std::mem::size_of::<i16>(),
            IntTy::I32 => std::mem::size_of::<i32>(),
            IntTy::I64 => std::mem::size_of::<i64>(),
            IntTy::I128 => std::mem::size_of::<i128>(),
            _ => todo!("Can't compute compiletime sizeof {int:?}"),
        },
        TyKind::Uint(int) => match int {
            UintTy::U8 => std::mem::size_of::<u8>(),
            UintTy::U16 => std::mem::size_of::<u16>(),
            UintTy::U32 => std::mem::size_of::<u32>(),
            UintTy::U128 => std::mem::size_of::<u128>(),
            UintTy::Usize => {
                eprintln!("WARNING: Assuming sizeof::<usize>() == 8!");
                8
            }
            _ => todo!("Can't compute compiletime sizeof {int:?}"),
        },
        TyKind::Bool => std::mem::size_of::<u8>(),
        _ => todo!("Can't compute compiletime sizeof {ty:?}"),
    }
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
pub fn string_class() -> DotnetTypeRef {
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.String");
    string.set_valuetype(false);
    string
}
pub fn usize_class() -> DotnetTypeRef {
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.UIntPtr");
    //TODO: Inwestigate this. The valuetype prefix seems to be missing from UIntPtr in compiled C# code
    string.set_valuetype(false);
    string
}
/// Translated MIR statements should have the total stack diff of 0.
pub fn check_debugable(
    ops: &[crate::cil_op::CILOp],
    debugable: impl std::fmt::Debug,
    does_return_void: bool,
) {
    use colored::Colorize;

    let mut stack = 0;
    for op in ops {
        if !(does_return_void && *op == crate::cil_op::CILOp::Ret) {
            stack += op.stack_diff();
        }
    }
    if stack != 0 {
        rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!("Propable miscompilation: {debugable:?} resulted in ops {ops:?} and did not pass the stack check.")};
        let mut stack = 0;
        for (index, op) in ops.iter().enumerate() {
            if !(does_return_void && *op == crate::cil_op::CILOp::Ret) {
                let diff = op.stack_diff();
                stack += diff;
                if stack < 0 {
                    eprintln!(
                        "{}",
                        format!("{index}:\t{op:?} changed stack by {diff}, to {stack}").red()
                    );
                } else {
                    eprintln!("{index}:\t{op:?} changed stack by {diff}, to {stack}");
                }
            } else {
                if stack < 0 {
                    eprintln!(
                        "{}",
                        format!("{index}:\t{op:?} changed stack by 0, to {stack}").red()
                    );
                } else {
                    eprintln!("{index}:\t{op:?} changed stack by 0, to {stack}");
                }
            }
        }
        //panic!();
    }
}
