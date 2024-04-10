use rustc_middle::mir::interpret::AllocId;
use rustc_middle::ty::{
    AdtDef, Const, ConstKind, EarlyBinder, GenericArg, Instance, List, ParamEnv, SymbolName, Ty,
    TyCtxt, TyKind, TypeFoldable,
};
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}

use crate::{cil::FieldDescriptor, r#type::TyCache, IString};
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
    let dotnet_class_name = demangled.replace("::",".");
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
    let field_ty = type_cache.type_from_cache(field_ty, ctx, Some(method_instance));
    let owner_ty = type_cache
        .type_from_cache(owner_ty, ctx, Some(method_instance))
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
        let element = type_cache.type_from_cache(element, tyctx, Some(method_instance));
        return FieldDescriptor::new(
            crate::r#type::simple_tuple(
                &elements
                    .iter()
                    .map(|tpe| {
                        let tpe = crate::utilis::monomorphize(&method_instance, tpe, tyctx);
                        type_cache.type_from_cache(tpe, tyctx, Some(method_instance))
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
        let field_type = type_cache.type_from_cache(field_type, tyctx, Some(method_instance));
        let owner_ty = crate::utilis::monomorphize(&method_instance, owner_ty, tyctx);
        let owner_type = type_cache.type_from_cache(owner_ty, tyctx, Some(method_instance));
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
    let field_ty = type_cache.type_from_cache(field_ty, tyctx, Some(method_instance));
    let owner_ty = type_cache
        .type_from_cache(owner_ty, tyctx, Some(method_instance))
        .as_dotnet()
        .expect("Error: tried to set a field of a non-object type!");
    FieldDescriptor::new(owner_ty, field_ty, field_name)
    /*
    let def = type_cache.type_def_from_cache(owner_ty, ctx, Some(method_instance)); //TypeDef::from_ty(owner_ty, ctx, &method_instance);
    let def = def.clone();
    let type_ref = type_cache
        .type_from_cache(owner_ty, ctx, Some(method_instance)) //
        .as_dotnet()
        .expect("Field owner not a dotnet type!");
    def.field_desc_from_rust_field_idx(type_ref, field_idx)*/
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
fn dummy_span_propably_unsafe() -> rustc_span::Span {
    let res = rustc_span::Span::with_root_ctxt(rustc_span::BytePos(0), rustc_span::BytePos(0));
    assert!(res.is_dummy());
    res
}
/// Converts a generic argument to a string, and panics if it could not.
pub fn garg_to_string<'tyctx>(garg: GenericArg<'tyctx>, ctx: TyCtxt<'tyctx>) -> String {
    let str_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");

    let val_tree = str_const
        .eval(ctx, ParamEnv::reveal_all(), dummy_span_propably_unsafe())
        .expect("Could not eval const!");
    let tpe = str_const
        .ty()
        .builtin_deref(true)
        .expect("Type of generic argument was not a reference, can't resolve as string!");
    assert!(
        tpe.ty.is_str(),
        "Generic argument was not a string, but {str_const:?}!"
    );

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
/// Converts a generic argument to a boolean, and panics if it could not.
pub fn garag_to_bool<'tyctx>(garg: GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> bool {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let tpe = usize_const.ty();
    assert!(
        tpe.is_bool(),
        "Generic argument was not a bool type! ty:{tpe:?}"
    );
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
/// This function returns the size of a type at the compile time. This should be used ONLY for handling constants. It currently assumes a 64 bit env
pub fn compiletime_sizeof<'tyctx>(ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> usize {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.size.bytes() as usize
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

/// Translated MIR statements should have the total stack diff of 0.
pub fn check_debugable(
    ops: &[crate::cil::CILOp],
    debugable: impl std::fmt::Debug,
    does_return_void: bool,
) {
    let mut stack = 0;
    for op in ops {
        if !(does_return_void && *op == crate::cil::CILOp::Ret) {
            stack += op.stack_diff();
        }
    }
    if stack != 0 {
        rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!("Propable miscompilation: {debugable:?} resulted in ops {ops:?} and did not pass the stack check.")};
        let mut stack = 0;
        for (index, op) in ops.iter().enumerate() {
            if does_return_void && *op == crate::cil::CILOp::Ret {
                eprintln!("{index}:\t{op:?} changed stack by 0, to {stack}");
            } else {
                let diff = op.stack_diff();
                stack += diff;
                eprintln!("{index}:\t{op:?} changed stack by {diff}, to {stack}");
            }
        }
        assert!(
            *crate::config::ALLOW_MISCOMPILATIONS,
            "Miscompiled  {debugable:?}."
        );
    }
}

pub fn max_stack(ops: &[crate::cil::CILOp], does_return_void: bool) -> usize {
    let mut stack = 0;
    let mut max_stack = 0;
    for op in ops {
        if !(does_return_void && *op == crate::cil::CILOp::Ret) {
            stack += op.stack_diff();
            max_stack = max_stack.max(stack);
        }
    }
    max_stack as usize
}
pub(crate) fn alloc_id_to_u64(alloc_id: AllocId) -> u64 {
    unsafe { std::mem::transmute(alloc_id) }
}
pub fn is_fn_intrinsic<'tyctx>(fn_ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> bool {
    match fn_ty.kind() {
        TyKind::FnDef(did, _) => tyctx.is_intrinsic(*did, tyctx.item_name(*did)),
        TyKind::Closure(_, _) => false,
        _ => todo!("Can't get signature of {fn_ty}"),
    }
}
pub fn align_of<'tcx>(ty: rustc_middle::ty::Ty<'tcx>, tyctx: TyCtxt<'tcx>) -> u64 {
    let layout = tyctx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;

    let align = layout.align.abi;
    // FIXME: this field is likely private for a reason. I should not do this get its value. Find a better way to get aligement.
    let pow2 = u64::from(unsafe { std::mem::transmute::<_, u8>(align) });
    1 << pow2
}
