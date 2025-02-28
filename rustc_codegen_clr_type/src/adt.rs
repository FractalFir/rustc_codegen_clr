use cilly::{
    v2::{Assembly, Float, Int}, FieldDesc, Type
};
use rustc_abi::{FieldIdx, FieldsShape, Layout, LayoutData, VariantIdx, Variants};
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_middle::ty::{Ty,TyKind,GenericArg,AdtDef};
use rustc_middle::ty::List;
use crate::{utilis::simple_tuple, GetTypeExt};

pub fn enum_variant_offsets(_: AdtDef, layout: Layout, vidix: VariantIdx) -> FieldOffsetIterator {
    FieldOffsetIterator::fields(get_variant_at_index(vidix, (*layout.0).clone()))
}

#[derive(Clone, Debug)]
pub enum FieldOffsetIterator {
    Explicit { offsets: Box<[u32]>, index: usize },
    NoOffset { count: u64 },
    Empty,
}
impl Iterator for FieldOffsetIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        match self {
            Self::Explicit { offsets, index } => {
                let next = offsets.get(*index);
                *index += 1;
                next.copied()
            }
            Self::NoOffset { count } => {
                if *count > 0 {
                    *count -= 1;
                    Some(0)
                } else {
                    None
                }
            }
            Self::Empty => None,
        }
    }
}
impl FieldOffsetIterator {
    pub fn from_fields_shape(fields: &rustc_abi::FieldsShape<FieldIdx>) -> Self {
        match fields {
            FieldsShape::Arbitrary {
                offsets,
                memory_index,
            } => {
                let offsets: Box<[_]> = memory_index
                    .iter()
                    .enumerate()
                    .map(|(index, _mem_idx)| {
                        u32::try_from(
                            offsets[FieldIdx::from_u32(u32::try_from(index).unwrap())].bytes(),
                        )
                        .unwrap()
                    })
                    //TODO: ask what does field offset of 4294967295 means.
                    .map(|offset| {
                        if offset > u32::from(u16::MAX) {
                            0
                        } else {
                            offset
                        }
                    })
                    .collect();
                FieldOffsetIterator::Explicit { offsets, index: 0 }
            }
            FieldsShape::Union(count) => FieldOffsetIterator::NoOffset {
                count: Into::<usize>::into(*count) as u64,
            },
            FieldsShape::Primitive => Self::Empty,
            FieldsShape::Array { stride, count } => {
                let mut curr: u32 = 0;
                let mut offsets = Vec::new();
                for _ in 0..*count {
                    offsets.push(curr);
                    curr += std::convert::TryInto::<u32>::try_into(stride.bytes())
                        .expect("Array stride too large");
                }
                FieldOffsetIterator::Explicit {
                    offsets: offsets.into(),
                    index: 0,
                }
            }
        }
    }
    pub fn fields(parent: LayoutData<FieldIdx, rustc_abi::VariantIdx>) -> FieldOffsetIterator {
        //eprintln!("ADT fields:{:?}",parent.fields);
        Self::from_fields_shape(&parent.fields)
    }
}
/// Takes layout of an enum as input, and returns the type of its tag(Void if no tag) and the size of the tag(0 if no tag).
pub fn enum_tag_info(r#enum: Layout<'_>, asm: &mut Assembly) -> (Type, u32) {
    match r#enum.variants() {
        Variants::Single { .. } => (
            Type::Void,
            FieldOffsetIterator::from_fields_shape(r#enum.fields())
                .next()
                .unwrap_or(0),
        ),
        Variants::Multiple { tag, tag_field, .. } => (
            scalr_to_type(*tag, asm),
            FieldOffsetIterator::from_fields_shape(r#enum.fields())
                .nth(*tag_field)
                .unwrap_or(0),
        ),
        Variants::Empty => (Type::Void, 0),
    }
}
fn scalr_to_type(scalar: rustc_abi::Scalar, asm: &mut Assembly) -> Type {
    let primitive = match scalar {
        rustc_abi::Scalar::Union { value } | rustc_abi::Scalar::Initialized { value, .. } => value,
    };
    primitive_to_type(primitive, asm)
}
fn primitive_to_type(primitive: rustc_abi::Primitive, asm: &mut Assembly) -> Type {
    use rustc_abi::Integer;
    use rustc_abi::Primitive;
    match primitive {
        Primitive::Int(int, sign) => match (int, sign) {
            (Integer::I8, true) => Type::Int(Int::I8),
            (Integer::I16, true) => Type::Int(Int::I16),
            (Integer::I32, true) => Type::Int(Int::I32),
            (Integer::I64, true) => Type::Int(Int::I64),
            (Integer::I128, true) => Type::Int(Int::I128),
            (Integer::I8, false) => Type::Int(Int::U8),
            (Integer::I16, false) => Type::Int(Int::U16),
            (Integer::I32, false) => Type::Int(Int::U32),
            (Integer::I64, false) => Type::Int(Int::U64),
            (Integer::I128, false) => Type::Int(Int::U128),
        },
        Primitive::Float(rustc_abi::Float::F16) => Type::Float(Float::F16),
        Primitive::Float(rustc_abi::Float::F32) => Type::Float(Float::F32),
        Primitive::Float(rustc_abi::Float::F64) => Type::Float(Float::F64),
        Primitive::Float(rustc_abi::Float::F128) => todo!("No support for 128 bit floats yet!"),
        Primitive::Pointer(_) => asm.nptr(Type::Void),
    }
}
pub fn get_variant_at_index(
    variant_index: VariantIdx,
    layout: LayoutData<FieldIdx, rustc_abi::VariantIdx>,
) -> LayoutData<FieldIdx, rustc_abi::VariantIdx> {
    match layout.variants {
        Variants::Single { .. } => layout,
        Variants::Multiple { variants, .. } => variants[variant_index].clone(),
        Variants::Empty => todo!("Empty variants have no variants."),
    }
}
pub fn enum_field_descriptor<'tcx>(
    owner_ty: Ty<'tcx>,
    field_idx: u32,
    variant_idx: u32,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> cilly::FieldIdx {
    let (adt, subst) = as_adt(owner_ty).expect("Tried to get a field of a non ADT type!");
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
    let variant_name = variant.name.to_string();
    let field_name = ctx.alloc_string(format!(
        "{variant_name}_{fname}",
        fname = crate::r#type::escape_field_name(&field.name.to_string())
    ));
    let field_ty = field.ty(ctx.tcx(), subst);
    let field_ty = ctx.monomorphize(field_ty);
    let field_ty = ctx.type_from_cache(field_ty);
    let owner_ty = ctx
        .type_from_cache(owner_ty)
        .as_class_ref()
        .expect("Error: tried to set a field of a non-object type!");

    ctx.alloc_field(FieldDesc::new(owner_ty, field_name, field_ty))
}
pub fn field_descrptor<'tcx>(
    owner_ty: Ty<'tcx>,
    field_idx: u32,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> cilly::FieldIdx {
    if let TyKind::Tuple(elements) = owner_ty.kind() {
        let element = elements[field_idx as usize];
        let element = ctx.monomorphize(element);
        let element = ctx.type_from_cache(element);
        let elements = elements
            .iter()
            .map(|tpe| {
                let tpe = ctx.monomorphize(tpe);
                ctx.type_from_cache(tpe)
            })
            .collect::<Vec<_>>();
        let field_name = ctx.alloc_string(format!("Item{}", field_idx + 1));
        let tuple_type = simple_tuple(&elements, ctx);
        return ctx.alloc_field(FieldDesc::new(tuple_type, field_name, element));
    } else if let TyKind::Closure(_, args) = owner_ty.kind() {
        let closure = args.as_closure();
        let field_type = closure
            .upvar_tys()
            .iter()
            .nth(field_idx as usize)
            .expect("Could not find closure fields!");
        let field_type = ctx.monomorphize(field_type);
        let field_type = ctx.type_from_cache(field_type);
        let owner_ty = ctx.monomorphize(owner_ty);
        let owner_type = ctx.type_from_cache(owner_ty);
        let field_name = ctx.alloc_string(format!("f_{field_idx}"));
        return ctx.alloc_field(FieldDesc::new(
            owner_type.as_class_ref().expect("Closure type invalid!"),
            field_name,
            field_type,
        ));
    } else if let TyKind::Coroutine(_, args) = owner_ty.kind() {
        let coroutine = args.as_coroutine();
        let field_type = coroutine
            .upvar_tys()
            .iter()
            .nth(field_idx as usize)
            .expect("Could not find coroutine fields!");
        let field_type = ctx.monomorphize(field_type);
        let field_type = ctx.type_from_cache(field_type);
        let owner_ty = ctx.monomorphize(owner_ty);
        let owner_type = ctx.type_from_cache(owner_ty);
        let field_name = ctx.alloc_string(format!("f_{field_idx}"));
        return ctx.alloc_field(FieldDesc::new(
            owner_type.as_class_ref().expect("Coroutine type invalid!"),
            field_name,
            field_type,
        ));
    }
    let (adt, subst) = as_adt(owner_ty).expect("Tried to get a field of a non ADT or tuple type!");
    let field = adt
        .all_fields()
        .nth(field_idx as usize)
        .expect("No field with provided index!");
    let field_name = crate::r#type::escape_field_name(&field.name.to_string());
    let field_ty = field.ty(ctx.tcx(), subst);
    let field_ty = ctx.monomorphize(field_ty);
    let field_ty = ctx.type_from_cache(field_ty);
    let owner_ty = ctx
        .type_from_cache(owner_ty)
        .as_class_ref()
        .expect("Error: tried to set a field of a non-object type!");
    let field_name = ctx.alloc_string(field_name);
    ctx.alloc_field(FieldDesc::new(owner_ty, field_name, field_ty))
}
pub fn as_adt(ty: Ty) -> Option<(AdtDef, &List<GenericArg>)> {
    match ty.kind() {
        TyKind::Adt(adt, subst) => Some((*adt, subst)),
        _ => None,
    }
}