/// A representation of a primitve type or a reference.
pub mod r#type;

use std::{
    collections::HashSet,
    num::{NonZero, NonZeroU32},
};

use crate::{
    fn_ctx::MethodCompileCtx,
    utilis::{adt::FieldOffsetIterator, garg_to_string},
};
use cilly::{
    add, ld_arg, ptr_cast,
    tpe::simd::SIMDVector,
    v2::{
        cilnode::MethodKind, Access, BasicBlock, CILNode, CILRoot, ClassDef, ClassDefIdx, ClassRef,
        ClassRefIdx, Float, Int, MethodDef, MethodImpl, StringIdx, Type,
    },
    Assembly, IntoAsmIndex,
};
pub use r#type::*;
use rustc_abi::Layout;
use rustc_middle::ty::{AdtDef, AdtKind, FloatTy, IntTy, List, Ty, TyKind, UintTy};
use rustc_span::def_id::DefId;
//pub use tycache::*;
//pub use type_def::*;

#[must_use]
pub fn from_int(int_tpe: &IntTy) -> cilly::Type {
    use cilly::Type;
    match int_tpe {
        IntTy::I8 => Type::Int(Int::I8),
        IntTy::I16 => Type::Int(Int::I16),
        IntTy::I32 => Type::Int(Int::I32),
        IntTy::I64 => Type::Int(Int::I64),
        IntTy::I128 => Type::Int(Int::I128),
        IntTy::Isize => Type::Int(Int::ISize),
    }
}

#[must_use]
pub fn from_uint(uint_tpe: &UintTy) -> cilly::Type {
    use cilly::Type;
    match uint_tpe {
        UintTy::U8 => Type::Int(Int::U8),
        UintTy::U16 => Type::Int(Int::U16),
        UintTy::U32 => Type::Int(Int::U32),
        UintTy::U64 => Type::Int(Int::U64),
        UintTy::U128 => Type::Int(Int::U128),
        UintTy::Usize => Type::Int(Int::USize),
    }
}

#[must_use]
pub fn from_float(float: &FloatTy) -> cilly::Type {
    use cilly::Type;
    match float {
        FloatTy::F16 => Type::Float(Float::F16),
        FloatTy::F32 => Type::Float(Float::F32),
        FloatTy::F64 => Type::Float(Float::F64),
        FloatTy::F128 => Type::Float(Float::F128),
    }
}
fn get_adt<'tcx>(
    adt_ty: Ty<'tcx>,
    def: AdtDef<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    name: StringIdx,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> ClassRefIdx {
    let cref = ClassRef::new(name, None, true, [].into());
    if ctx.contains_ref(&cref) {
        ctx.alloc_class_ref(cref)
    } else {
        let cref = ctx.alloc_class_ref(cref);
        let def = match def.adt_kind() {
            AdtKind::Struct => struct_(name, def, adt_ty, subst, ctx),
            AdtKind::Enum => enum_(name, def, adt_ty, subst, ctx),
            AdtKind::Union => union_(name, def, adt_ty, subst, ctx),
        };
        ctx.class_def(def);
        cref
    }
}
/// Converts a Rust MIR type to an optimized .NET type representation.
pub fn get_type<'tcx>(ty: Ty<'tcx>, ctx: &mut MethodCompileCtx<'tcx, '_>) -> Type {
    let ty = ctx.monomorphize(ty);
    // If this is a ZST, return a void type.
    if crate::utilis::is_zst(ty, ctx.tcx()) {
        return Type::Void;
    }

    match ty.kind() {
        TyKind::Bound(_, _inner) => Type::Void,
        TyKind::Bool => Type::Bool,
        TyKind::Char => Type::Int(Int::U32),
        TyKind::Closure(def, args) => {
            // Get the info about this closure: its sig + fields
            let closure = args.as_closure();
            // Extract the sig
            let mut sig = closure.sig();
            sig = ctx.monomorphize(sig);
            let sig = ctx.tcx().normalize_erasing_late_bound_regions(
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
                sig,
            );
            let inputs: Box<_> = sig.inputs().iter().map(|ty| get_type(*ty, ctx)).collect();
            let output = get_type(sig.output(), ctx);
            let sig = ctx.sig(inputs, output);
            // Extract the closure fields
            let fields: Box<[_]> = closure
                .upvar_tys()
                .iter()
                .map(|ty| get_type(ty, ctx))
                .collect();
            // Get a closure name.
            let name = closure_name(*def, &fields, sig, ctx);
            let name = ctx.alloc_string(name);
            // Get the layout of the closure
            let layout = ctx.layout_of(ty);
            // Allocate a class reference to the closure
            let cref = ctx.alloc_class_ref(ClassRef::new(name, None, true, [].into()));
            // If there is no defition of this closure present, create the closure.
            if ctx.class_ref_to_def(cref).is_none() {
                let type_def = closure_typedef(&fields, layout.layout, ctx, name);
                ctx.class_def(type_def);
            }
            Type::ClassRef(cref)
        }
        TyKind::Dynamic(_list, _, _) => {
            let name = ctx.alloc_string("Dyn");
            let cref = ctx.alloc_class_ref(ClassRef::new(name, None, true, [].into()));
            if ctx.class_ref_to_def(cref).is_none() {
                ctx.class_def(ClassDef::new(
                    name,
                    true,
                    0,
                    None,
                    vec![],
                    vec![],
                    cilly::v2::Access::Public,
                    None,
                    None,
                    false, // Two separate pointers.
                ));
            }
            Type::ClassRef(cref)
        }
        TyKind::Float(float) => from_float(float),
        TyKind::Foreign(_foregin) => Type::Void,
        TyKind::FnDef(_did, _subst) => Type::Void,
        TyKind::FnPtr(sig, _) => {
            let sig = ctx.tcx().normalize_erasing_late_bound_regions(
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
                *sig,
            );
            //let sig = crate::function_sig::from_poly_sig(method, tcx, self, sig);
            let output = get_type(ctx.monomorphize(sig.output()), ctx);
            let inputs: Box<[Type]> = sig
                .inputs()
                .iter()
                .map(|input| get_type(ctx.monomorphize(*input), ctx))
                .collect();
            let sig = ctx.sig(inputs, output);
            Type::FnPtr(sig)
        }
        TyKind::Int(int) => from_int(int),
        TyKind::Uint(int) => from_uint(int),
        TyKind::Never => Type::Void,
        TyKind::RawPtr(inner, _) | TyKind::Ref(_, inner, _) => {
            if pointer_to_is_fat(*inner, ctx.tcx(), ctx.instance()) {
                let inner = match inner.kind() {
                    TyKind::Slice(inner) => ctx.monomorphize(*inner),
                    TyKind::Str => Ty::new_uint(ctx.tcx(), UintTy::U8),
                    _ => ctx.monomorphize(*inner),
                };
                Type::ClassRef(fat_ptr_to(inner, ctx))
            } else {
                let inner = get_type(*inner, ctx);
                ctx.nptr(inner)
            }
        }
        // Slice type is almost never refered to directly, and should pop up here ONLY in the case of
        // a DST.
        TyKind::Str => Type::Int(Int::U8),
        TyKind::Slice(inner) => {
            let inner = ctx.monomorphize(*inner);
            get_type(inner, ctx)
        }
        TyKind::Tuple(types) => {
            let types: Vec<_> = types.iter().map(|ty| get_type(ty, ctx)).collect();
            if types.is_empty() {
                Type::Void
            } else {
                let name = tuple_name(&types, ctx);
                let name = ctx.alloc_string(name);
                let cref = ClassRef::new(name, None, true, [].into());
                // This only checks if a refernce to this class has already been allocated. In theory, allocating a class reference beforhand could break this, and make it not add the type definition
                if !ctx.contains_ref(&cref) {
                    let layout = ctx.layout_of(ty);
                    let _ = tuple_typedef(&types, layout.layout, ctx, name);
                }
                Type::ClassRef(ctx.alloc_class_ref(cref))
            }
        }
        TyKind::Adt(def, subst) => {
            let name = crate::utilis::adt_name(*def, ctx.tcx(), subst);
            if def.repr().simd() {
                let (count, elem) = ty.simd_size_and_type(ctx.tcx());
                let elem = ctx.type_from_cache(elem);
                // if count == 1, then this is just a single type.
                if count == 1 {
                    return elem;
                }
                return Type::SIMDVector(SIMDVector::new(
                    elem.try_into().unwrap(),
                    count.try_into().unwrap(),
                ));
            }
            if is_name_magic(name.as_ref()) {
                if name.contains(INTEROP_CLASS_TPE_NAME) {
                    assert!(
                        subst.len() == 2,
                        "Managed object reference must have exactly 2 generic arguments!"
                    );
                    let assembly = garg_to_string(subst[0], ctx.tcx());
                    let assembly = Some(assembly)
                        .filter(|assembly| !assembly.is_empty())
                        .map(|asm| ctx.alloc_string(asm));
                    let name = garg_to_string(subst[1], ctx.tcx());
                    let name = ctx.alloc_string(name);
                    Type::ClassRef(ctx.alloc_class_ref(ClassRef::new(
                        name,
                        assembly,
                        false,
                        [].into(),
                    )))
                } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
                    assert!(
                        subst.len() == 2,
                        "Managed struct reference must have exactly 2 generic arguments!"
                    );
                    let assembly = garg_to_string(subst[0], ctx.tcx());
                    let assembly = Some(assembly)
                        .filter(|assembly| !assembly.is_empty())
                        .map(|asm| ctx.alloc_string(asm));
                    let name = garg_to_string(subst[1], ctx.tcx());
                    let name = ctx.alloc_string(name);
                    Type::ClassRef(ctx.alloc_class_ref(ClassRef::new(
                        name,
                        assembly,
                        true,
                        [].into(),
                    )))
                } else if name.contains(INTEROP_ARR_TPE_NAME) {
                    assert!(subst.len() == 2, "Managed array reference must have exactly 2 generic arguments: type and dimension count!");
                    let element = &subst[0].as_type().expect("Array type must be specified!");
                    let element = get_type(ctx.monomorphize(*element), ctx);
                    let dimensions = garag_to_usize(subst[1], ctx.tcx());
                    Type::PlatformArray {
                        elem: ctx.alloc_type(element),
                        dims: std::num::NonZeroU8::new(dimensions.try_into().unwrap()).unwrap(),
                    }
                } else if name.contains(INTEROP_CHR_TPE_NAME) {
                    Type::PlatformChar
                } else {
                    todo!("Interop type {name:?} is not yet supported!")
                }
            } else {
                let name = ctx.alloc_string(name);
                Type::ClassRef(get_adt(ty, *def, subst, name, ctx))
            }
        }
        TyKind::Array(element, length) => {
            // Get the lenght of thid array
            let length = ctx.monomorphize(*length);
            let length: usize = crate::utilis::try_resolve_const_size(length).unwrap();
            // Get the element of the array
            let element = ctx.monomorphize(*element);
            let element = get_type(element, ctx);
            // Get the layout and size of this array
            let layout = ctx.layout_of(ty);
            let arr_size = layout.layout.size().bytes();
            let arr_align = layout.layout.align().pref.bytes();
            // An array of this size can't be represented on the .NET side
            if std::convert::TryInto::<u32>::try_into(arr_size).is_err() {
                eprintln!("WARNING: Array {ty:?} of size {arr_size:?} can't be represented on the .NET side. Repleacing it with an unsided void.");
                return Type::Void;
            }
            let cref = fixed_array(ctx, element, length as u64, arr_size, arr_align);
            Type::ClassRef(cref)
        }
        TyKind::Alias(_, _) => panic!("Attempted to get the .NET type of an unmorphized type"),
        TyKind::Coroutine(defid, coroutine_args) => {
            let coroutine_args = coroutine_args.as_coroutine();

            // Extract the closure fields
            let fields: Box<[_]> = coroutine_args
                .upvar_tys()
                .iter()
                .map(|ty| get_type(ty, ctx))
                .collect();
            // Get a coroutine name.
            let name = coroutine_name(*defid, &fields, ctx);
            let name = ctx.alloc_string(name);
            // Get the layout of the coroutine
            let layout = ctx.layout_of(ty);
            // Allocate a class reference to the coroutine
            let cref = ctx.alloc_class_ref(ClassRef::new(name, None, true, [].into()));
            // If there is no defition of this coroutine present, create the coroutine.
            if ctx.class_ref_to_def(cref).is_none() {
                let mut type_def = closure_typedef(&fields, layout.layout, ctx, name);
                handle_tag(&layout.layout, ctx, ty, type_def.fields_mut());
                ctx.class_def(type_def);
            }

            Type::ClassRef(cref)
        }
        _ => todo!("Can't yet get type {ty:?} from type cache."),
    }
}
//
fn fixed_array(
    asm: &mut Assembly,
    element: Type,
    length: u64,
    arr_size: u64,
    align: u64,
) -> ClassRefIdx {
    // Get the reference to the array class
    let cref = ClassRef::fixed_array(element, length, asm);

    // If the array definition not already present, add it.
    if asm.class_ref_to_def(cref).is_none() {
        let fields = vec![(element, asm.alloc_string("f0"), Some(0))];
        let class_ref = asm.class_ref(cref).clone();
        let Ok(size) = std::convert::TryInto::<u32>::try_into(arr_size) else {
            panic!(
                "Array of {element:?} with size {arr_size} >= 2^32. Unsuported.",
                element = element.mangle(asm)
            )
        };
        let arr = asm.class_def(ClassDef::new(
            class_ref.name(),
            true,
            0,
            None,
            fields,
            vec![],
            Access::Public,
            Some(NonZeroU32::new(size).unwrap()),
            NonZeroU32::new(align.try_into().unwrap()),
            true,
        ));
        // Common nodes
        let ldarg_2 = ld_arg!(2).into_idx(asm);
        let elem_tpe_idx = asm.alloc_type(element);
        let elem_addr = add!(
            ptr_cast!(ld_arg!(0), *elem_tpe_idx),
            cilly::mul!(ld_arg!(1), cilly::size_of!(elem_tpe_idx))
        )
        .into_idx(asm);
        // Defintion of the set_Item method.
        let set_item = asm.alloc_string("set_Item");
        let this_ref = asm.nref(Type::ClassRef(cref));
        let set_sig = asm.sig([this_ref, Type::Int(Int::USize), element], Type::Void);
        let arg_names = vec![
            Some(asm.alloc_string("this")),
            Some(asm.alloc_string("idx")),
            Some(asm.alloc_string("elem")),
        ];
        let set_root = asm.alloc_root(CILRoot::StInd(Box::new((
            elem_addr, ldarg_2, element, false,
        ))));
        let void_ret = asm.alloc_root(CILRoot::VoidRet);
        asm.new_method(MethodDef::new(
            Access::Public,
            arr,
            set_item,
            set_sig,
            MethodKind::Instance,
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![set_root, void_ret], 0, None)],
                locals: vec![],
            },
            arg_names,
        ));
        // Implementation of the get_Item method
        let get_item = asm.alloc_string("get_Item");
        let get_sig = asm.sig([this_ref, Type::Int(Int::USize)], element);
        let arg_names = vec![
            Some(asm.alloc_string("this")),
            Some(asm.alloc_string("idx")),
        ];
        let elem_val = asm.alloc_node(CILNode::LdInd {
            addr: elem_addr,
            tpe: elem_tpe_idx,
            volatile: false,
        });
        let elem_ret = asm.alloc_root(CILRoot::Ret(elem_val));
        asm.new_method(MethodDef::new(
            Access::Public,
            arr,
            get_item,
            get_sig,
            MethodKind::Instance,
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![elem_ret], 0, None)],
                locals: vec![],
            },
            arg_names,
        ));
        // Implementation of the get_Address method
        let get_address = asm.alloc_string("get_Address");
        let elem_ref_tpe = asm.nptr(element);
        let addr_sig = asm.sig([this_ref, Type::Int(Int::USize)], elem_ref_tpe);
        let arg_names = vec![
            Some(asm.alloc_string("this")),
            Some(asm.alloc_string("idx")),
        ];

        let elem_ret = asm.alloc_root(CILRoot::Ret(elem_addr));
        asm.new_method(MethodDef::new(
            Access::Public,
            arr,
            get_address,
            addr_sig,
            MethodKind::Instance,
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![elem_ret], 0, None)],
                locals: vec![],
            },
            arg_names,
        ));
    }
    cref
}

/// Returns a fat pointer to an inner type.
pub fn fat_ptr_to<'tcx>(mut inner: Ty<'tcx>, ctx: &mut MethodCompileCtx<'tcx, '_>) -> ClassRefIdx {
    inner = ctx.monomorphize(inner);
    let inner_tpe = get_type(inner, ctx);
    let name = format!("FatPtr{elem}", elem = inner_tpe.mangle(ctx));
    let name = ctx.alloc_string(name);
    let cref = ctx.alloc_class_ref(ClassRef::new(name, None, true, [].into()));
    if ctx.class_ref_to_def(cref).is_none() {
        let def = ClassDef::new(
            name,
            true,
            0,
            None,
            vec![
                (
                    ctx.nptr(Type::Void),
                    ctx.alloc_string(crate::DATA_PTR),
                    Some(0),
                ),
                (
                    Type::Int(Int::USize),
                    ctx.alloc_string(crate::METADATA),
                    Some(8),
                ),
            ],
            vec![],
            Access::Public,
            Some(NonZeroU32::new(16).unwrap()),
            Some(NonZeroU32::new(8).unwrap()),
            true,
        );
        ctx.class_def(def);
    }
    cref
}
/// Returns the name of a clousre with a given id, fields, and signature.
pub fn closure_name(
    _def_id: DefId,
    fields: &[Type],
    _sig: cilly::v2::SigIdx,
    ctx: &mut MethodCompileCtx<'_, '_>,
) -> String {
    let mangled_fields: String = fields.iter().map(|tpe| tpe.mangle(ctx)).collect();
    format!(
        "Closure{field_count}{mangled_fields}",
        field_count = fields.len()
    )
}
/// Returns the name of a coroutine with a given id, fields, and signature.
pub fn coroutine_name(
    def_id: DefId,
    fields: &[Type],
    ctx: &mut MethodCompileCtx<'_, '_>,
) -> String {
    let mangled_fields: String = fields.iter().map(|tpe| tpe.mangle(ctx)).collect();
    format!(
        "Coroutine{def_id:?}{field_count}{mangled_fields}",
        field_count = fields.len()
    )
}
/// Creates a [`ClassDef`] representing a closure with certain layout and fields.
#[must_use]
pub fn closure_typedef(
    fields: &[Type],
    layout: Layout,
    ctx: &mut MethodCompileCtx<'_, '_>,
    closure_name: StringIdx,
) -> ClassDef {
    // Collects all field types, offsets, and names
    let field_iter = fields
        .iter()
        .enumerate()
        .map(|(idx, ty)| (format!("f_{idx}"), *ty));
    let offset_iter = FieldOffsetIterator::fields((*layout.0).clone());
    let mut fields = Vec::new();
    let mut unique_checks = HashSet::new();
    for ((name, field), offset) in (field_iter).zip(offset_iter) {
        if field == Type::Void {
            continue;
        }
        fields.push((field, ctx.alloc_string(name), Some(offset)));
        unique_checks.insert(offset);
    }
    let has_nonverlaping_layout = unique_checks.len() == fields.len();
    // Create a class definition representing this closure.
    ClassDef::new(
        closure_name,
        true,
        0,
        None,
        fields,
        vec![],
        Access::Public,
        Some(
            NonZeroU32::new(
                layout
                    .size()
                    .bytes()
                    .try_into()
                    .expect("Closure size exceeds 2^32"),
            )
            .unwrap(),
        ),
        Some(
            NonZeroU32::new(
                layout
                    .align()
                    .pref
                    .bytes()
                    .try_into()
                    .expect("Closure alignement exceeds 2^32"),
            )
            .unwrap(),
        ),
        has_nonverlaping_layout,
    )
}
/// Turns an adt struct defintion into a [`ClassDef`]
fn struct_<'tcx>(
    name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> ClassDef {
    // Double-check is not a ZST.

    // Get the layout of this struct
    let layout = ctx.layout_of(adt_ty);

    // Go trough fields, collectiing them and their offsets
    let mut fields = Vec::new();
    let explicit_offset_iter =
        crate::utilis::adt::FieldOffsetIterator::fields((*layout.layout.0).clone());
    let mut unique_checks = HashSet::new();
    for (field, offset) in adt
        .variant(rustc_abi::VariantIdx::from_u32(0))
        .fields
        .iter()
        .zip(explicit_offset_iter)
    {
        let name = escape_field_name(&field.name.to_string());
        let field_type = get_type(ctx.monomorphize(field.ty(ctx.tcx(), subst)), ctx);
        if field_type == Type::Void {
            continue;
        }
        unique_checks.insert(offset);
        fields.push((field_type, ctx.alloc_string(name), Some(offset)));
    }
    let size = layout.layout.size().bytes();
    let size = if let Ok(size) = std::convert::TryInto::<u32>::try_into(size) {
        size
    } else if *crate::config::ABORT_ON_ERROR {
        panic!("Struct {adt_ty:?} size {size} >= 2^32. Unsuported.")
    } else {
        eprintln!("WARNING: Struct {adt_ty:?} excceeds max size of 2^32. Clamping the size, this can cause UB.");
        u32::MAX
    };
    let has_nonverlaping_layout = unique_checks.len() == fields.len();
    ClassDef::new(
        name,
        true,
        0,
        None,
        fields,
        vec![],
        Access::Public,
        NonZeroU32::new(size),
        Some(
            NonZeroU32::new(
                layout
                    .layout
                    .align()
                    .pref
                    .bytes()
                    .try_into()
                    .expect("Struct alignement exceeds 2^32"),
            )
            .unwrap(),
        ),
        has_nonverlaping_layout,
    )
}

fn handle_tag<'tcx>(
    layout: &Layout,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    adt_ty: Ty<'tcx>,
    fields: &mut Vec<(Type, StringIdx, Option<u32>)>,
) {
    match &layout.variants {
        rustc_abi::Variants::Single { index: _ } => {
            let (tag_type, offset) = crate::utilis::adt::enum_tag_info(*layout, ctx);

            if tag_type != Type::Void {
                fields.push((tag_type, ctx.alloc_string(crate::ENUM_TAG), Some(offset)));
            }
        }
        rustc_abi::Variants::Empty => (),
        rustc_abi::Variants::Multiple {
            tag: _,
            tag_encoding,
            tag_field: _,
            variants: _,
        } => {
            let layout = ctx.layout_of(adt_ty);

            match tag_encoding {
                rustc_abi::TagEncoding::Direct => {
                    let (tag_type, offset) = crate::utilis::adt::enum_tag_info(layout.layout, ctx);

                    if tag_type != Type::Void {
                        fields.push((tag_type, ctx.alloc_string(crate::ENUM_TAG), Some(offset)));
                    }
                }
                rustc_abi::TagEncoding::Niche {
                    untagged_variant: _,
                    niche_variants: _,
                    ..
                } => {
                    let (tag_type, offset) = crate::utilis::adt::enum_tag_info(layout.layout, ctx);
                    let offsets = FieldOffsetIterator::fields((*layout.layout.0).clone());

                    assert!(offsets.count() > 0, "layout.fields:{:?}", layout.fields);
                    if tag_type != Type::Void {
                        fields.push((tag_type, ctx.alloc_string(crate::ENUM_TAG), Some(offset)));
                    }
                }
            }
        }
    }
}
/// Turns an adt enum defintion into a [`ClassDef`]
fn enum_<'tcx>(
    enum_name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> ClassDef {
    let layout = ctx.layout_of(adt_ty);
    let mut fields: Vec<(Type, StringIdx, Option<u32>)> = vec![];
    // Handle the enum tag.
    handle_tag(&layout.layout, ctx, adt_ty, &mut fields);
    // Handle enum variants
    for (vidx, variant) in adt.variants().iter_enumerated() {
        let variant_name = variant.name.to_string();
        let mut variant_fields = vec![];
        let field_offset_iter = crate::utilis::adt::enum_variant_offsets(adt, layout.layout, vidx);

        for (field, offset) in variant.fields.iter().zip(field_offset_iter) {
            let name = format!(
                "{variant_name}_{fname}",
                fname = escape_field_name(&field.name.to_string())
            );
            let field_ty = get_type(field.ty(ctx.tcx(), subst), ctx);
            if field_ty == Type::Void {
                continue;
            }

            variant_fields.push((field_ty, ctx.alloc_string(name), Some(offset)));
        }

        fields.extend(variant_fields);
    }
    // Check no field is void.
    fields
        .iter()
        .for_each(|(tpe, _, _)| assert_ne!(*tpe, Type::Void));
    ClassDef::new(
        enum_name,
        true,
        0,
        None,
        fields,
        vec![],
        Access::Public,
        Some(NonZeroU32::new(layout.layout.size().bytes().try_into().unwrap()).unwrap()),
        Some(
            NonZeroU32::new(
                layout
                    .layout
                    .align()
                    .pref
                    .bytes()
                    .try_into()
                    .expect("Enum alignement exceeds 2^32"),
            )
            .unwrap(),
        ),
        false,
    )
}
/// Turns an adt union defintion into a [`ClassDef`]
fn union_<'tcx>(
    name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> ClassDef {
    // Get union layout
    let layout = ctx.layout_of(adt_ty);
    let mut fields = Vec::new();
    // Get union fields
    for (field, offset) in adt
        .all_fields()
        .zip(crate::utilis::adt::FieldOffsetIterator::fields(
            (*layout.layout.0).clone(),
        ))
    {
        let field_name = escape_field_name(&field.name.to_string());
        let field_ty = ctx.monomorphize(field.ty(ctx.tcx(), subst));
        let field_type = get_type(field_ty, ctx);
        if field_type == Type::Void {
            continue;
        }
        fields.push((field_type, ctx.alloc_string(field_name), Some(offset)));
    }
    // Create a union ClassDef
    ClassDef::new(
        name,
        true,
        0,
        None,
        fields,
        vec![],
        Access::Public,
        Some(NonZeroU32::new(layout.layout.size().bytes().try_into().unwrap()).unwrap()),
        Some(
            NonZeroU32::new(
                layout
                    .layout
                    .align()
                    .pref
                    .bytes()
                    .try_into()
                    .expect("Union alignement exceeds 2^32"),
            )
            .unwrap(),
        ),
        false,
    )
}
#[must_use]
pub fn escape_field_name(name: &str) -> String {
    match name.chars().next() {
        None => "fld".into(),
        Some(first) => {
            if !(first.is_alphabetic() || first == '_')
        || name == "value"
        || name == "flags"
        || name == "alignment"
        || name == "init"
        || name == "string"
        || name == "nint"
        || name == "nuint"
        || name == "out"
        || name == "rem"
        || name == "add"
        || name == "div"
        || name == "error"
        || name == "opt"
        || name == "private"
        || name == "public"
        || name == "object"
        || name == "class"
        //FIXME: this is a sign of a bug. ALL fields not starting with a letter should have been caught by the statement above.
        || name == "0"
            {
                format!("m_{name}")
            } else {
                name.into()
            }
        }
    }
}
#[must_use]
pub fn tuple_typedef(
    elements: &[Type],
    layout: Layout,
    ctx: &mut MethodCompileCtx<'_, '_>,
    name: StringIdx,
) -> ClassDefIdx {
    let field_iter = elements
        .iter()
        .enumerate()
        .map(|(idx, ele)| (format!("Item{}", idx + 1), *ele));
    let explicit_offset_iter = FieldOffsetIterator::fields((*layout.0).clone());

    let mut fields = Vec::new();
    for ((name, field), offset) in (field_iter).zip(explicit_offset_iter) {
        if field == Type::Void {
            continue;
        }
        fields.push((field, ctx.alloc_string(name), Some(offset)));
    }
    ctx.class_def(ClassDef::new(
        name,
        true,
        0,
        None,
        fields,
        vec![],
        Access::Public,
        Some(
            NonZero::new(
                layout
                    .size()
                    .bytes()
                    .try_into()
                    .expect("Tuple size >= 2^32. Unsuported"),
            )
            .expect("Zero-sized tuple!"),
        ),
        Some(
            NonZeroU32::new(
                layout
                    .align()
                    .pref
                    .bytes()
                    .try_into()
                    .expect("Tuple alignement exceeds 2^32"),
            )
            .unwrap(),
        ),
        true,
    ))
}
