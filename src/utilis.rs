use std::env::Args;

use rustc_middle::mir::interpret::AllocId;
use rustc_middle::ty::{
    AdtDef, AdtKind, Const, ConstKind, EarlyBinder, FloatTy, GenericArg, Instance, List, ParamEnv,
    SymbolName, Ty, TyCtxt, TyKind, TypeFoldable,
};
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}
use crate::method;
use crate::r#type::pointer_to_is_fat;
use crate::{
    cil::{CILOp, FieldDescriptor},
    r#type::TyCache,
    r#type::{DotnetTypeRef, Type},
    IString,
};
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
    let auto_mangled =
        rustc_symbol_mangling::symbol_name_for_instance_in_crate(tyctx, adt_instance, krate);
    escape_class_name(&auto_mangled)
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
    let name: String = name.to_string();
    // Name TOO long
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
    let field_name = crate::r#type::escape_field_name(&field.name.to_string());
    let field_ty = field.ty(ctx, subst);
    let field_ty = crate::utilis::monomorphize(&method_instance, field_ty, ctx);
    let field_ty = type_cache.type_from_cache(field_ty, ctx, Some(method_instance));
    let owner_ty = type_cache
        .type_from_cache(owner_ty, ctx, Some(method_instance))
        .as_dotnet()
        .expect("Error: tried to set a field of a non-object type!");
    assert!(owner_ty.asm().is_none(), "External enum!");
    let variant_name = variant.name.to_string();
    let enum_variant_dotnet = DotnetTypeRef::new(
        None,
        &format!(
            "{owner_name}/{variant_name}",
            owner_name = owner_ty.name_path()
        ),
    );
    FieldDescriptor::new(enum_variant_dotnet, field_ty, field_name)
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
/// Returns the size of a tag of an enum with `variants` variants.
pub fn enum_tag_size(_variants: u64) -> u32 {
    /*u32::try_from(((u64::from(u64::BITS) - u64::from((variants).leading_zeros())) + 8 - 1) / 8)
        .expect("Enum variant over 2^4294967296")*/
    // Seems to be always 4
    1
}
/// Gets the type of the tag of enum with `variants` varinats.
pub fn tag_from_enum_variants(variants: u64) -> crate::r#type::Type {
    let var_size = enum_tag_size(variants);
    // println!("variants:{variants}tag_size:{var_size}");
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

    let val_tree = str_const
        .eval(ctx, ParamEnv::reveal_all(), None)
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
pub fn compiletime_sizeof<'tyctx>(
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
) -> usize {
    use rustc_middle::ty::{IntTy, UintTy};
    match ty.kind() {
        TyKind::Int(int) => match int {
            IntTy::I8 => std::mem::size_of::<i8>(),
            IntTy::I16 => std::mem::size_of::<i16>(),
            IntTy::I32 => std::mem::size_of::<i32>(),
            IntTy::I64 => std::mem::size_of::<i64>(),
            IntTy::I128 => std::mem::size_of::<i128>(),
            IntTy::Isize => {
                eprintln!("WARNING: Assuming sizeof::<isize>() == 8!");
                8
            }
        },
        TyKind::Uint(int) => match int {
            UintTy::U8 => std::mem::size_of::<u8>(),
            UintTy::U16 => std::mem::size_of::<u16>(),
            UintTy::U32 => std::mem::size_of::<u32>(),
            UintTy::U64 => std::mem::size_of::<u64>(),
            UintTy::U128 => std::mem::size_of::<u128>(),
            UintTy::Usize => {
                eprintln!("WARNING: Assuming sizeof::<usize>() == 8!");
                8
            }
        },
        TyKind::Float(float_ty) => match float_ty {
            FloatTy::F32 => std::mem::size_of::<f32>(),
            FloatTy::F64 => std::mem::size_of::<f64>(),
        },
        TyKind::Bool => std::mem::size_of::<u8>(),
        TyKind::Adt(def, subst) => match def.adt_kind() {
            AdtKind::Struct => def
                .all_fields()
                .map(|field| compiletime_sizeof(field.ty(tyctx, subst), tyctx, method_instance))
                .sum::<usize>(),
            AdtKind::Union => def
                .all_fields()
                .map(|field| compiletime_sizeof(field.ty(tyctx, subst), tyctx, method_instance))
                .max()
                .unwrap_or(0),
            AdtKind::Enum => {
                let tag = match def.variants().len() {
                    0 => 0,
                    1..=256 => 1,
                    257..=65_535 => 2,
                    65_536..=4_294_967_295 => 4,
                    _ => 8,
                };
                let mut max_size = 0;
                for variant in def.variants() {
                    let variant_size = variant
                        .fields
                        .iter()
                        .map(|field| {
                            let field = field.ty(tyctx, subst);
                            let field = monomorphize(&method_instance, field, tyctx);
                            compiletime_sizeof(field, tyctx, method_instance)
                        })
                        .sum::<usize>();
                    max_size = max_size.max(variant_size);
                }
                // Tag + largest variant
                tag + max_size
            }
        },
        TyKind::Tuple(elements) => elements
            .iter()
            .map(|element| compiletime_sizeof(element, tyctx, method_instance))
            .sum::<usize>(),
        TyKind::RawPtr(type_and_mut) => {
            if pointer_to_is_fat(type_and_mut.ty, tyctx, Some(method_instance)) {
                eprintln!("WARNING: Assuming sizeof::<*T>() == sizeof::<isize>() == 8!");
                8 * 2
            } else {
                eprintln!("WARNING: Assuming sizeof::<*T>() == sizeof::<isize>() == 8!");
                8
            }
        }
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
/// Returns a [`DotnetTypeRef`] describing the string class.
pub fn string_class() -> DotnetTypeRef {
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.String");
    string.set_valuetype(false);
    string
}
/// Returns a [`DotnetTypeRef`] describing the usize class.
pub fn usize_class() -> DotnetTypeRef {
    let mut string = DotnetTypeRef::new(Some("System.Runtime"), "System.UIntPtr");
    //TODO: Inwestigate this. The valuetype prefix seems to be missing from UIntPtr in compiled C# code
    string.set_valuetype(false);
    string
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
pub(crate) fn alloc_id_to_u64(alloc_id: AllocId) -> u64 {
    unsafe { std::mem::transmute(alloc_id) }
}
/// Part of micompilatio detection.
pub(crate) fn verify_locals_within_range(ops: &[CILOp], argc: u32, locc: u32) -> bool {
    ops.iter().all(|op| match op {
        CILOp::LDLoc(local) | CILOp::LDLocA(local) | CILOp::STLoc(local) => *local < locc,
        CILOp::LDArg(arg) | CILOp::LDArgA(arg) | CILOp::STArg(arg) => *arg < argc,
        _ => true,
    })
}
pub fn is_fn_intrinsic<'tyctx>(fn_ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> bool {
    use rustc_target::spec::abi::Abi;
    let internal_abi = match fn_ty.kind() {
        TyKind::FnDef(_, _) => fn_ty.fn_sig(tyctx),
        TyKind::Closure(_, args) => args.as_closure().sig(),
        _ => todo!("Can't get signature of {fn_ty}"),
    }
    .abi();
    // Only those ABIs are supported
    match internal_abi {
        Abi::RustIntrinsic => true,
        _ => false,
    }
}
