/// A representation of a primitve type or a reference.
pub mod r#type;

use std::num::NonZeroU32;

use crate::{
    fn_ctx::MethodCompileCtx,
    utilis::{adt::FieldOffsetIterator, garg_to_string},
};
use cilly::v2::{Access, ClassDef, ClassRef, ClassRefIdx, Float, Int, StringIdx, Type};
pub use r#type::*;
use rustc_middle::ty::{AdtDef, AdtKind, FloatTy, IntTy, List, ParamEnv, Ty, TyKind, UintTy};
use rustc_span::def_id::DefId;
use rustc_target::abi::Layout;
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
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> ClassRefIdx {
    let cref = ClassRef::new(name, None, true, [].into());
    if ctx.asm_mut().contains_ref(&cref) {
        ctx.asm_mut().alloc_class_ref(cref)
    } else {
        let cref = ctx.asm_mut().alloc_class_ref(cref);
        let def = match def.adt_kind() {
            AdtKind::Struct => struct_(name, def, adt_ty, subst, ctx),
            AdtKind::Enum => enum_(name, def, adt_ty, subst, ctx),
            AdtKind::Union => union_(name, def, adt_ty, subst, ctx),
        };
        ctx.asm_mut().class_def(def);
        cref
    }
}
/// Converts a Rust MIR type to an optimized .NET type representation.
pub fn get_type<'tcx>(ty: Ty<'tcx>, ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>) -> Type {
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
            let sig = ctx
                .tcx()
                .normalize_erasing_late_bound_regions(ParamEnv::reveal_all(), sig);
            let inputs: Box<_> = sig.inputs().iter().map(|ty| get_type(*ty, ctx)).collect();
            let output = get_type(sig.output(), ctx);
            let sig = ctx.asm_mut().sig(inputs, output);
            // Extract the closure fields
            let fields: Box<[_]> = closure
                .upvar_tys()
                .iter()
                .map(|ty| get_type(ty, ctx))
                .collect();
            // Get a closure name.
            let name = closure_name(*def, &fields, sig, ctx);
            let name = ctx.asm_mut().alloc_string(name);
            // Get the layout of the closure
            let layout = ctx.layout_of(ty);
            // Allocate a class reference to the closure
            let cref = ctx
                .asm_mut()
                .alloc_class_ref(ClassRef::new(name, None, true, [].into()));
            // If there is no defition of this closure present, create the closure.
            if ctx.asm().class_ref_to_def(cref).is_none() {
                let type_def = closure_typedef(&fields, layout.layout, ctx, name);
                ctx.asm_mut().class_def(type_def);
            }
            Type::ClassRef(cref)
        }
        TyKind::Dynamic(_list, _, _) => {
            let name = ctx.asm_mut().alloc_string("Dyn");
            let cref = ctx
                .asm_mut()
                .alloc_class_ref(ClassRef::new(name, None, true, [].into()));
            if ctx.asm().class_ref_to_def(cref).is_none() {
                ctx.asm_mut().class_def(ClassDef::new(
                    name,
                    true,
                    0,
                    None,
                    vec![],
                    vec![],
                    vec![],
                    cilly::v2::Access::Public,
                    None,
                ));
            }
            Type::ClassRef(cref)
        }
        TyKind::Float(float) => match float {
            FloatTy::F16 => Type::Float(Float::F16),
            FloatTy::F32 => Type::Float(Float::F32),
            FloatTy::F64 => Type::Float(Float::F64),
            FloatTy::F128 => Type::Float(Float::F128),
        },
        TyKind::Foreign(_foregin) => Type::Void,
        TyKind::FnDef(_did, _subst) => Type::Void,
        TyKind::FnPtr(sig, _) => {
            let sig = ctx
                .tcx()
                .normalize_erasing_late_bound_regions(ParamEnv::reveal_all(), *sig);
            //let sig = crate::function_sig::from_poly_sig(method, tcx, self, sig);
            let output = get_type(ctx.monomorphize(sig.output()), ctx);
            let inputs: Box<[Type]> = sig
                .inputs()
                .iter()
                .map(|input| get_type(ctx.monomorphize(*input), ctx))
                .collect();
            let sig = ctx.asm_mut().sig(inputs, output);
            Type::FnPtr(sig)
        }
        TyKind::Never => Type::Void,
        TyKind::RawPtr(inner, _) | TyKind::Ref(_, inner, _) => {
            if pointer_to_is_fat(*inner, ctx.tcx(), ctx.instance()) {
                let inner = match inner.kind() {
                    TyKind::Slice(inner) => ctx.monomorphize(*inner),
                    TyKind::Str => Ty::new(ctx.tcx(), TyKind::Uint(UintTy::U8)),
                    _ => ctx.monomorphize(*inner),
                };
                Type::ClassRef(fat_ptr_to(inner, ctx))
            } else {
                let inner = get_type(*inner, ctx);
                ctx.asm_mut().nptr(inner)
            }
        }
        // Slice type is almost never refered to directly, and should pop up here ONLY in the case of
        // a DST.
        TyKind::Str => Type::Int(Int::U8),
        TyKind::Slice(inner) => {
            let inner = ctx.monomorphize(*inner);
            get_type(inner, ctx)
        }
        TyKind::Adt(def, subst) => {
            let name = crate::utilis::adt_name(*def, ctx.tcx(), subst);
            if is_name_magic(name.as_ref()) {
                if name.contains(INTEROP_CLASS_TPE_NAME) {
                    assert!(
                        subst.len() == 2,
                        "Managed object reference must have exactly 2 generic arguments!"
                    );
                    let assembly = garg_to_string(subst[0], ctx.tcx());
                    let assembly = Some(assembly)
                        .filter(|assembly| !assembly.is_empty())
                        .map(|asm| ctx.asm_mut().alloc_string(asm));
                    let name = garg_to_string(subst[1], ctx.tcx());
                    let name = ctx.asm_mut().alloc_string(name);
                    Type::ClassRef(ctx.asm_mut().alloc_class_ref(ClassRef::new(
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
                        .map(|asm| ctx.asm_mut().alloc_string(asm));
                    let name = garg_to_string(subst[1], ctx.tcx());
                    let name = ctx.asm_mut().alloc_string(name);
                    Type::ClassRef(ctx.asm_mut().alloc_class_ref(ClassRef::new(
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
                        elem: ctx.asm_mut().alloc_type(element),
                        dims: std::num::NonZeroU8::new(dimensions.try_into().unwrap()).unwrap(),
                    }
                } else if name.contains(INTEROP_CHR_TPE_NAME) {
                    Type::PlatformChar
                } else {
                    todo!("Interop type {name:?} is not yet supported!")
                }
            } else {
                let name = ctx.asm_mut().alloc_string(name);
                Type::ClassRef(get_adt(ty, *def, subst, name, ctx))
            }
        }
        TyKind::Array(element, length) => {
            // Get the lenght of thid array
            let length = ctx.monomorphize(*length);
            let length: usize = crate::utilis::try_resolve_const_size(length).unwrap();
            // Get the element of the arrau
            let element = ctx.monomorphize(*element);
            let element = get_type(element, ctx);
            // Get the layout and size of this array
            let layout = ctx.layout_of(ty);
            let arr_size = layout.layout.size();
            let element_name = element.mangle(ctx.asm());
            let arr_name = format!("A{length}_{element_name}");
            let arr_name = ctx.asm_mut().alloc_string(arr_name);
            // Get the reference to the array class
            let cref =
                ctx.asm_mut()
                    .alloc_class_ref(ClassRef::new(arr_name, None, true, [].into()));
            // If the array definition not already present, add it.
            if ctx.asm().class_ref_to_def(cref).is_none() {
                let fields = vec![(element, ctx.asm_mut().alloc_string("f0"), Some(0))];
                ctx.asm_mut().class_def(ClassDef::new(
                    arr_name,
                    true,
                    0,
                    None,
                    fields,
                    vec![],
                    vec![],
                    Access::Public,
                    Some(
                        NonZeroU32::new(
                            arr_size
                                .bytes()
                                .try_into()
                                .expect("Array size >= 2^32. Unsuported."),
                        )
                        .unwrap(),
                    ),
                ));
            }
            Type::ClassRef(cref)
        }
        TyKind::Alias(_, _) => panic!("Attempted to get the .NET type of an unmorphized type"),
        _ => todo!("Can't yet get type {ty:?} from type cache."),
    }
}

/// Returns a fat pointer to an inner type.
pub fn fat_ptr_to<'tcx>(
    mut inner: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> ClassRefIdx {
    inner = ctx.monomorphize(inner);
    let inner_tpe = get_type(inner, ctx);
    let name = format!("FatPtr{elem}", elem = inner_tpe.mangle(ctx.asm()));
    let name = ctx.asm_mut().alloc_string(name);
    let cref = ctx
        .asm_mut()
        .alloc_class_ref(ClassRef::new(name, None, true, [].into()));
    if ctx.asm().class_ref_to_def(cref).is_none() {
        let def = ClassDef::new(
            name,
            true,
            0,
            None,
            vec![
                (
                    ctx.asm_mut().nptr(Type::Void),
                    ctx.asm_mut().alloc_string(crate::DATA_PTR),
                    Some(0),
                ),
                (
                    Type::Int(Int::USize),
                    ctx.asm_mut().alloc_string(crate::METADATA),
                    Some(8),
                ),
            ],
            vec![],
            vec![],
            Access::Public,
            Some(NonZeroU32::new(16).unwrap()),
        );
        ctx.asm_mut().class_def(def);
    }
    cref
}
/// Returns the name of a clousre with a given id, fields, and signature.
pub fn closure_name(
    _def_id: DefId,
    fields: &[Type],
    _sig: cilly::v2::SigIdx,
    ctx: &mut MethodCompileCtx<'_, '_, '_, '_>,
) -> String {
    let mangled_fields: String = fields.iter().map(|tpe| tpe.mangle(ctx.asm())).collect();
    format!(
        "Closure{field_count}{mangled_fields}",
        field_count = fields.len()
    )
}
/// Creates a [`ClassDef`] representing a closure with certain layout and fields.
#[must_use]
pub fn closure_typedef(
    fields: &[Type],
    layout: Layout,
    ctx: &mut MethodCompileCtx<'_, '_, '_, '_>,
    closure_name: StringIdx,
) -> ClassDef {
    // Collects all field types, offsets, and names
    let field_iter = fields
        .iter()
        .enumerate()
        .map(|(idx, ty)| (format!("f_{idx}"), *ty));
    let offset_iter = FieldOffsetIterator::fields((*layout.0).clone());
    let mut fields = Vec::new();
    for ((name, field), offset) in (field_iter).zip(offset_iter) {
        if field == Type::Void {
            continue;
        }
        fields.push((field, ctx.asm_mut().alloc_string(name), Some(offset)));
    }
    // Create a class definition representing this closure.
    ClassDef::new(
        closure_name,
        true,
        0,
        None,
        fields,
        vec![],
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
    )
}
/// Turns an adt struct defintion into a [`ClassDef`]
fn struct_<'tcx>(
    name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> ClassDef {
    // Double-check is not a ZST.

    // Get the layout of this struct
    let layout = ctx.layout_of(adt_ty);

    // Go trough fields, collectiing them and their offsets
    let mut fields = Vec::new();
    let explicit_offset_iter =
        crate::utilis::adt::FieldOffsetIterator::fields((*layout.layout.0).clone());

    for (field, offset) in adt
        .variant(rustc_target::abi::VariantIdx::from_u32(0))
        .fields
        .iter()
        .zip(explicit_offset_iter)
    {
        let name = escape_field_name(&field.name.to_string());
        let field_type = get_type(ctx.monomorphize(field.ty(ctx.tcx(), subst)), ctx);

        if field_type == Type::Void {
            continue;
        }
        fields.push((field_type, ctx.asm_mut().alloc_string(name), Some(offset)));
    }
    ClassDef::new(
        name,
        true,
        0,
        None,
        fields,
        vec![],
        vec![],
        Access::Public,
        Some(
            NonZeroU32::new(layout.layout.size().bytes().try_into().unwrap())
                .expect("Type size can't be 0!"),
        ),
    )
}
/// Turns an adt enum defintion into a [`ClassDef`]
fn enum_<'tcx>(
    enum_name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> ClassDef {
    let layout = ctx.layout_of(adt_ty);
    let mut fields: Vec<(Type, StringIdx, Option<u32>)> = vec![];
    // Handle the enum tag.
    match &layout.variants {
        rustc_target::abi::Variants::Single { index: _ } => {
            let (tag_type, offset) =
                crate::utilis::adt::enum_tag_info(layout.layout, ctx.asm_mut());

            if tag_type != Type::Void {
                fields.push((
                    tag_type,
                    ctx.asm_mut().alloc_string(crate::ENUM_TAG),
                    Some(offset),
                ));
            }
        }
        rustc_target::abi::Variants::Multiple {
            tag: _,
            tag_encoding,
            tag_field: _,
            variants: _,
        } => {
            let layout = ctx.layout_of(adt_ty);

            match tag_encoding {
                rustc_target::abi::TagEncoding::Direct => {
                    let (tag_type, offset) =
                        crate::utilis::adt::enum_tag_info(layout.layout, ctx.asm_mut());

                    if tag_type != Type::Void {
                        fields.push((
                            tag_type,
                            ctx.asm_mut().alloc_string(crate::ENUM_TAG),
                            Some(offset),
                        ));
                    }
                }
                rustc_target::abi::TagEncoding::Niche {
                    untagged_variant: _,
                    niche_variants: _,
                    ..
                } => {
                    let (tag_type, offset) =
                        crate::utilis::adt::enum_tag_info(layout.layout, ctx.asm_mut());
                    let offsets = FieldOffsetIterator::fields((*layout.layout.0).clone());

                    assert!(offsets.count() > 0, "layout.fields:{:?}", layout.fields);
                    if tag_type != Type::Void {
                        fields.push((
                            tag_type,
                            ctx.asm_mut().alloc_string(crate::ENUM_TAG),
                            Some(offset),
                        ));
                    }
                }
            }
        }
    };
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

            variant_fields.push((field_ty, ctx.asm_mut().alloc_string(name), Some(offset)));
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
        vec![],
        Access::Public,
        Some(NonZeroU32::new(layout.layout.size().bytes().try_into().unwrap()).unwrap()),
    )
}
/// Turns an adt union defintion into a [`ClassDef`]
fn union_<'tcx>(
    name: StringIdx,
    adt: AdtDef<'tcx>,
    adt_ty: Ty<'tcx>,
    subst: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
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
        fields.push((
            field_type,
            ctx.asm_mut().alloc_string(field_name),
            Some(offset),
        ));
    }
    // Create a union ClassDef
    ClassDef::new(
        name,
        true,
        0,
        None,
        fields,
        vec![],
        vec![],
        Access::Public,
        Some(NonZeroU32::new(layout.layout.size().bytes().try_into().unwrap()).unwrap()),
    )
}
/*
fn array_methods(element_count: usize, arr_class: ClassDefIdx, element: Type, asm: &mut Assembly) {
    if element_count > 0 {
        let mimpl = cilly::v2::MethodImpl::MethodBody {
            blocks: vec![cilly::v2::BasicBlock::new(
                vec![
                    CILRoot::STObj {
                        tpe: element.clone().into(),
                        addr_calc: Box::new(
                            (conv_usize!(ld_field_address!(
                                CILNode::LDArg(0),
                                FieldDescriptor::boxed(
                                    (&def).into(),
                                    element.clone(),
                                    "f0".to_string().into(),
                                )
                            )) + CILNode::LDArg(1) * conv_usize!(size_of!(element.clone())))
                            .cast_ptr(Type::Ptr(Box::new(element.clone()))),
                        ),
                        value_calc: Box::new(CILNode::LDArg(2)),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
            locals: [].into(),
        };
        let set_usize = MethodDef::new(
            Access::Public,
            arr_class,
            asm.alloc_string("set_Item"),
            asm.sig(
                [
                    asm.nptr(Type::ClassRef(*arr_class)),
                    Type::Int(Int::USize),
                    element,
                ],
                cilly::v2::Type::Void,
            ),
            MethodKind::Instance,
            mimpl,
            vec![
                Some(asm.alloc_string("this")),
                Some(asm.alloc_string("idx")),
                Some(asm.alloc_string("val")),
            ],
        );

        def.add_method(set_usize);

        // get_Address(usize offset)
        let get_adress_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            cilly::fn_sig::FnSig::new(
                &[
                    Type::Ptr(Box::new(def.clone().into())),
                    Type::Int(Int::USize),
                ],
                Type::Ptr(element.clone().into()),
            ),
            "get_Address",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: (conv_usize!(ld_field_address!(
                        CILNode::LDArg(0),
                        FieldDescriptor::boxed(
                            (&def).into(),
                            element.clone(),
                            "f0".to_string().into(),
                        )
                    )) + CILNode::LDArg(1) * conv_usize!(size_of!(element.clone())))
                    .cast_ptr(Type::Ptr(Box::new(element.clone()))),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("this".into()), Some("idx".into())],
        );
        get_adress_usize.validate().unwrap();
        def.add_method(get_adress_usize);

        // get_Item
        let get_item_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            cilly::fn_sig::FnSig::new(
                &[
                    Type::Ptr(Box::new(def.clone().into())),
                    Type::Int(Int::USize),
                ],
                element.clone(),
            ),
            "get_Item",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: CILNode::LdObj {
                        ptr: Box::new(
                            (conv_usize!(ld_field_address!(
                                CILNode::LDArg(0),
                                FieldDescriptor::boxed(
                                    (&def).into(),
                                    element.clone(),
                                    "f0".to_string().into(),
                                )
                            )) + CILNode::LDArg(1) * conv_usize!(size_of!(element.clone())))
                            .cast_ptr(Type::Ptr(Box::new(element.clone()))),
                        ),
                        obj: Box::new(element),
                    },
                }
                .into()],
                0,
                None,
            )],
            vec![Some("this".into()), Some("idx".into())],
        );
        get_item_usize.validate().unwrap();
        def.add_method(get_item_usize);

        //to_string.set_ops(ops);
        //def.add_method(to_string);
    }
    def
}
*/
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
