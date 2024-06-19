use cilly::{
    cil_node::CILNode, cil_root::CILRoot, eq, field_desc::FieldDescriptor, gt_un, ldc_u64, sub,
    DotnetTypeRef, Type,
};
use rustc_middle::ty::{AdtDef, Ty, TyCtxt};
use rustc_target::abi::{
    FieldIdx, FieldsShape, Layout, LayoutS, TagEncoding, VariantIdx, Variants,
};
pub fn enum_variant_offsets(_: AdtDef, layout: Layout, vidix: VariantIdx) -> FieldOffsetIterator {
    FieldOffsetIterator::fields(get_variant_at_index(vidix, (*layout.0).clone()))
}

#[derive(Clone, Debug)]
pub(crate) enum FieldOffsetIterator {
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
    pub fn from_fields_shape(fields: &rustc_target::abi::FieldsShape<FieldIdx>) -> Self {
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
            FieldsShape::Array { .. } => todo!("Unhandled fields shape: {fields:?}"),
        }
    }
    pub fn fields(parent: LayoutS<FieldIdx, rustc_target::abi::VariantIdx>) -> FieldOffsetIterator {
        //eprintln!("ADT fields:{:?}",parent.fields);
        Self::from_fields_shape(&parent.fields)
    }
}
/// Takes layout of an enum as input, and returns the type of its tag(Void if no tag) and the size of the tag(0 if no tag).
pub fn enum_tag_info<'tyctx>(r#enum: Layout<'tyctx>, _: TyCtxt<'tyctx>) -> (Type, u32) {
    match r#enum.variants() {
        Variants::Single { .. } => (
            Type::Void,
            FieldOffsetIterator::from_fields_shape(r#enum.fields())
                .next()
                .unwrap_or(0),
        ),
        Variants::Multiple { tag, tag_field, .. } => (
            scalr_to_type(*tag),
            FieldOffsetIterator::from_fields_shape(r#enum.fields())
                .nth(*tag_field)
                .unwrap_or(0),
        ),
    }
}
fn scalr_to_type(scalar: rustc_target::abi::Scalar) -> Type {
    let primitive = match scalar {
        rustc_target::abi::Scalar::Union { value }
        | rustc_target::abi::Scalar::Initialized { value, .. } => value,
    };
    primitive_to_type(primitive)
}
fn primitive_to_type(primitive: rustc_target::abi::Primitive) -> Type {
    use rustc_target::abi::Integer;
    use rustc_target::abi::Primitive;
    match primitive {
        Primitive::Int(int, sign) => match (int, sign) {
            (Integer::I8, true) => Type::I8,
            (Integer::I16, true) => Type::I16,
            (Integer::I32, true) => Type::I32,
            (Integer::I64, true) => Type::I64,
            (Integer::I128, true) => Type::I128,
            (Integer::I8, false) => Type::U8,
            (Integer::I16, false) => Type::U16,
            (Integer::I32, false) => Type::U32,
            (Integer::I64, false) => Type::U64,
            (Integer::I128, false) => Type::U128,
        },
        Primitive::Float(rustc_abi::Float::F16) => Type::F16,
        Primitive::Float(rustc_abi::Float::F32) => Type::F32,
        Primitive::Float(rustc_abi::Float::F64) => Type::F64,
        Primitive::Float(rustc_abi::Float::F128) => todo!("No support for 128 bit floats yet!"),
        Primitive::Pointer(_) => Type::Ptr(Type::Void.into()),
    }
}
pub fn get_variant_at_index(
    variant_index: VariantIdx,
    layout: LayoutS<FieldIdx, rustc_target::abi::VariantIdx>,
) -> LayoutS<FieldIdx, rustc_target::abi::VariantIdx> {
    match layout.variants {
        Variants::Single { .. } => layout,
        Variants::Multiple { variants, .. } => variants[variant_index].clone(),
    }
}
pub fn set_discr<'tyctx>(
    layout: Layout<'tyctx>,
    variant_index: VariantIdx,
    enum_addr: CILNode,
    enum_tpe: &DotnetTypeRef,
    tyctx: TyCtxt<'tyctx>,
    ty: Ty<'tyctx>,
) -> CILRoot {
    if get_variant_at_index(variant_index, (*layout.0).clone())
        .abi
        .is_uninhabited()
    {
        // Could be skipped, but keeping a throw here can with CIL correctnes. Each block *must* terminate with a jump, return or a throw.
        // By inserting a throw, we are able to remove all code
        // after it safely.
        return CILRoot::throw(
            "UB: SetDiscirminant used, but the specified enum variant is not inhabited.",
        );
    }
    match layout.variants {
        Variants::Single { index } => {
            assert_eq!(index, variant_index);
            CILRoot::Nop
        }
        Variants::Multiple {
            tag_encoding: TagEncoding::Direct,
            ..
        } => {
            let (tag_tpe, _) = enum_tag_info(layout, tyctx);
            let tag_val = ldc_u64!(ty
                .discriminant_for_variant(tyctx, variant_index)
                .unwrap()
                .val
                .try_into()
                .expect("Enum varaint id can't fit in u64."));
            let tag_val = crate::casts::int_to_int(Type::U64, &tag_tpe, tag_val);
            CILRoot::SetField {
                addr: Box::new(enum_addr),
                value: Box::new(tag_val),
                desc: Box::new(FieldDescriptor::new(
                    enum_tpe.clone(),
                    tag_tpe,
                    "value__".into(),
                )),
            }
        }
        Variants::Multiple {
            tag_encoding:
                TagEncoding::Niche {
                    untagged_variant,
                    ref niche_variants,
                    niche_start,
                },
            ..
        } => {
            if variant_index == untagged_variant {
                CILRoot::Nop
            } else {
                let (tag_tpe, _) = enum_tag_info(layout, tyctx);
                //let niche = self.project_field(bx, tag_field);
                //let niche_llty = bx.cx().immediate_backend_type(niche.layout);
                let niche_value = variant_index.as_u32() - niche_variants.start().as_u32();
                let niche_value = (niche_value as u128).wrapping_add(niche_start);
                let tag_val = ldc_u64!(niche_value
                    .try_into()
                    .expect("Enum varaint id can't fit in u64."));
                let tag_val = crate::casts::int_to_int(Type::U64, &tag_tpe, tag_val);
                CILRoot::SetField {
                    addr: Box::new(enum_addr),
                    value: Box::new(tag_val),
                    desc: Box::new(FieldDescriptor::new(
                        enum_tpe.clone(),
                        tag_tpe,
                        "value__".into(),
                    )),
                }
            }
        }
    }
}

pub fn get_discr<'tyctx>(
    layout: Layout<'tyctx>,
    enum_addr: CILNode,
    enum_tpe: DotnetTypeRef,
    tyctx: TyCtxt<'tyctx>,
    ty: Ty<'tyctx>,
) -> CILNode {
    //return CILNode::
    assert!(
        !layout.abi.is_uninhabited(),
        "UB: enum layout is unanhibited!"
    );
    let (tag_tpe, _) = crate::utilis::adt::enum_tag_info(layout, tyctx);
    let tag_encoding = match layout.variants {
        Variants::Single { index } => {
            let discr_val = ty
                .discriminant_for_variant(tyctx, index)
                .map_or(index.as_u32() as u128, |discr| discr.val);
            let tag_val = ldc_u64!(discr_val.try_into().expect("Tag does not fit within a u64"));
            return crate::casts::int_to_int(Type::U64, &tag_tpe, tag_val);
        }
        Variants::Multiple {
            ref tag_encoding, ..
        } => tag_encoding,
    };

    // Decode the discriminant (specifically if it's niche-encoded).
    let discr = match *tag_encoding {
        TagEncoding::Direct => {
            if tag_tpe == Type::Void {
                //CILNode::LDOb
                todo!();
            } else {
                CILNode::LDField {
                    field: FieldDescriptor::new(enum_tpe, tag_tpe.clone(), "value__".into()).into(),
                    addr: enum_addr.into(),
                }
            }
        }
        TagEncoding::Niche {
            untagged_variant,
            ref niche_variants,
            niche_start,
        } => {
            let (disrc_type, _) = crate::utilis::adt::enum_tag_info(layout, tyctx);
            let relative_max = niche_variants.end().as_u32() - niche_variants.start().as_u32();
            let tag = CILNode::LDField {
                field: FieldDescriptor::new(enum_tpe, disrc_type.clone(), "value__".into()).into(),
                addr: enum_addr.into(),
            };
            // We have a subrange `niche_start..=niche_end` inside `range`.
            // If the value of the tag is inside this subrange, it's a
            // "niche value", an increment of the discriminant. Otherwise it
            // indicates the untagged variant.
            // A general algorithm to extract the discriminant from the tag
            // is:
            // relative_tag = tag - niche_start
            // is_niche = relative_tag <= (ule) relative_max
            // discr = if is_niche {
            //     cast(relative_tag) + niche_variants.start()
            // } else {
            //     untagged_variant
            // }
            // However, we will likely be able to emit simpler code.
            let (is_niche, tagged_discr, delta) = if relative_max == 0 {
                // Best case scenario: only one tagged variant. This will
                // likely become just a comparison and a jump.
                // The algorithm is:
                // is_niche = tag == niche_start
                // discr = if is_niche {
                //     niche_start
                // } else {
                //     untagged_variant
                // }

                let is_niche = eq!(
                    tag,
                    crate::casts::int_to_int(
                        Type::U64,
                        &disrc_type,
                        ldc_u64!(niche_start
                            .try_into()
                            .expect("tag is too big to fit within u64"))
                    )
                ); //bx.icmp(IntPredicate::IntEQ, tag, niche_start);

                let tagged_discr = crate::casts::int_to_int(
                    Type::U64,
                    &disrc_type,
                    ldc_u64!(u64::from(niche_variants.start().as_u32())),
                );
                (is_niche, tagged_discr, 0)
            } else {
                eprintln!("General alg used for {ty:?}");
                // The special cases don't apply, so we'll have to go with
                // the general algorithm.
                //let tag = crate::casts::int_to_int(disrc_type.clone(), &Type::U64, tag);
                let relative_discr = sub!(
                    tag,
                    crate::casts::int_to_int(
                        Type::U64,
                        &disrc_type,
                        ldc_u64!(niche_start
                            .try_into()
                            .expect("tag is too big to fit within u64"))
                    )
                );

                let is_niche = eq!(
                    gt_un!(
                        relative_discr.clone(),
                        crate::casts::int_to_int(
                            Type::U64,
                            &disrc_type,
                            ldc_u64!(u64::from(relative_max))
                        )
                    ),
                    CILNode::LdFalse
                );
                (
                    is_niche,
                    relative_discr,
                    niche_variants.start().as_u32() as u128,
                )
            };

            let tagged_discr = if delta == 0 {
                tagged_discr
            } else {
                let delta = crate::casts::int_to_int(
                    Type::U64,
                    &disrc_type,
                    ldc_u64!(delta.try_into().expect("Tag does not fit within u64")),
                );
                assert!(matches!(
                    disrc_type.clone(),
                    Type::U8
                        | Type::I8
                        | Type::U16
                        | Type::I16
                        | Type::U32
                        | Type::I32
                        | Type::U64
                        | Type::I64
                        | Type::Ptr(_)
                        | Type::USize
                        | Type::ISize
                ));
                tagged_discr + delta
            };

            // In principle we could insert assumes on the possible range of `discr`, but
            // currently in LLVM this seems to be a pessimization.

            CILNode::select(
                disrc_type.clone(),
                tagged_discr,
                crate::casts::int_to_int(
                    Type::U64,
                    &disrc_type,
                    ldc_u64!(u64::from(untagged_variant.as_u32())),
                ),
                is_niche,
            )
        }
    };
    discr

    //discr
}
