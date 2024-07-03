use crate::assembly::MethodCompileCtx;
use crate::operand::{handle_operand, operand_address};
use crate::r#type::pointer_to_is_fat;
use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::field_desc::FieldDescriptor;
use cilly::{
    conv_u32, conv_usize, ld_field, ld_field_address, ldc_i64, ldc_u32, ldc_u64, ptr, size_of,
};
use cilly::{DotnetTypeRef, Type};
use rustc_middle::{
    mir::Operand,
    ty::{layout::TyAndLayout, ParamEnv, PolyExistentialTraitRef, Ty, TyKind},
};
use rustc_target::abi::FIRST_VARIANT;
struct UnsizeInfo<'tcx> {
    /// Type the source pointer points to
    source_points_to: Ty<'tcx>,
    /// The address of the rarget pointer. This pointer should always be a thin pointer, pointing to a fat pointer.
    target_ptr: CILNode,
    /// The source pointer. If the source pointer is fat, this will be a fat pointer!
    source_ptr: CILNode,
    target_dotnet: DotnetTypeRef,
    target_type: Type,
}
impl<'tcx> UnsizeInfo<'tcx> {
    pub fn for_unsize(
        ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
        operand: &Operand<'tcx>,
        target: Ty<'tcx>,
    ) -> Self {
        // Get the monomorphized source and target type
        let target = ctx.monomorphize(target);
        let source = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
        // Get the source and target types as .NET types
        let source_type = ctx.type_from_cache(source);
        let target_type = ctx.type_from_cache(target);
        // Get the target type as a fat pointer.
        let target_dotnet = target_type.as_dotnet().unwrap();
        let mut sized_ptr = handle_operand(operand, ctx);
        // Unsizing a box
        if target.is_box() && source.is_box() {
            // 1. Get Unqiue<Source> from Box<Source>
            let unique_desc = crate::utilis::field_descrptor(source, 0, ctx);
            let source_ptr = ld_field!(sized_ptr, unique_desc);
            // 2. Get NonNull<Source> from Unuqie<Source>
            let unique_adt = crate::utilis::as_adt(source).unwrap();
            let unique_ty = unique_adt.0.all_fields().nth(0).unwrap();
            let non_null_ptr_desc =
                crate::utilis::field_descrptor(unique_ty.ty(ctx.tcx(), unique_adt.1), 0, ctx);
            let source_ptr = ld_field!(source_ptr, non_null_ptr_desc.clone());
            // 3. Get Source* from NonNull<Source>
            let non_null_adt =
                crate::utilis::as_adt(unique_ty.ty(ctx.tcx(), unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let source_ptr_desc =
                crate::utilis::field_descrptor(non_null_ty.ty(ctx.tcx(), unique_adt.1), 0, ctx);

            let source_ptr = ld_field!(source_ptr, source_ptr_desc.clone());
            // 4. Get Unique<Target> from Box<Target>
            let unique_desc = crate::utilis::field_descrptor(target, 0, ctx);
            let target_ptr = ld_field_address!(CILNode::LoadAddresOfTMPLocal, unique_desc);
            // 5. Get NonNull<Target>  from Unique<Target>
            let unique_adt = crate::utilis::as_adt(target).unwrap();
            let unique_ty = unique_adt.0.all_fields().nth(0).unwrap();
            let target_ptr_desc =
                crate::utilis::field_descrptor(unique_ty.ty(ctx.tcx(), unique_adt.1), 0, ctx);
            let target_ptr = ld_field_address!(target_ptr, target_ptr_desc);
            // 6. Get Target* from NonNull<Target>
            let non_null_adt =
                crate::utilis::as_adt(unique_ty.ty(ctx.tcx(), unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let non_null_ptr_desc =
                crate::utilis::field_descrptor(non_null_ty.ty(ctx.tcx(), non_null_adt.1), 0, ctx);
            let target_ptr = ld_field_address!(target_ptr, non_null_ptr_desc.clone());
            // 7. Set the target->metatdata = len and target->ptr = source->ptr
            let derefed_source = source.boxed_ty();

            Self {
                source_points_to: derefed_source,
                target_ptr,
                source_ptr,
                target_dotnet: non_null_ptr_desc.tpe().as_dotnet().unwrap(),
                target_type,
            }
        } else {
            let derefed_source = match source.kind() {
                TyKind::RawPtr(tpe, _) => *tpe,
                TyKind::Ref(_, inner, _) => *inner,
                TyKind::Adt(_, _) => {
                    if source.is_box() {
                        let inner = source.boxed_ty();
                        let field_descriptor = crate::utilis::field_descrptor(source, 0, ctx);
                        sized_ptr = CILNode::TemporaryLocal(Box::new((
                            source_type.clone(),
                            [CILRoot::SetTMPLocal { value: sized_ptr }].into(),
                            ld_field!(CILNode::LoadAddresOfTMPLocal, field_descriptor),
                        )));
                        inner
                    } else {
                        panic!("Non ptr type:{source:?}")
                    }
                }
                _ => panic!("Non ptr type:{source:?}"),
            };

            Self {
                source_points_to: derefed_source,
                target_ptr: CILNode::LoadAddresOfTMPLocal,
                source_ptr: sized_ptr,
                target_dotnet,
                target_type,
            }
        }
    }
}

/// Preforms an unsizing cast on operand `operand`, converting it to the `target` type.
pub fn unsize2<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    operand: &Operand<'tcx>,
    target: Ty<'tcx>,
) -> CILNode {
    let info = UnsizeInfo::for_unsize(ctx, operand, target);
    let src_cil = operand_address(operand, ctx);
    let metadata = unsize_metadata(
        ctx,
        src_cil,
        ctx.layout_of(operand.ty(ctx.body(), ctx.tcx())),
        ctx.layout_of(target),
    );
    let metadata_field =
        FieldDescriptor::new(info.target_dotnet.clone(), Type::USize, "metadata".into());
    let ptr_field = FieldDescriptor::new(
        info.target_dotnet,
        Type::Ptr(Type::Void.into()),
        "data_pointer".into(),
    );
    let init_metadata = CILRoot::SetField {
        addr: Box::new(info.target_ptr.clone()),
        value: Box::new(metadata.cast_ptr(Type::USize)),
        desc: Box::new(metadata_field),
    };
    let init_ptr = if pointer_to_is_fat(info.source_points_to, ctx.tcx(), ctx.instance()) {
        CILRoot::SetField {
            addr: Box::new(info.target_ptr),
            value: Box::new(CILNode::LDIndPtr {
                ptr: Box::new(operand_address(operand, ctx).cast_ptr(ptr!(Type::Void))),
                loaded_ptr: Box::new(ptr!(Type::Void)),
            }),
            desc: Box::new(ptr_field),
        }
    } else {
        CILRoot::SetField {
            addr: Box::new(info.target_ptr),
            value: Box::new(handle_operand(operand, ctx).cast_ptr(ptr!(Type::Void))),
            desc: Box::new(ptr_field),
        }
    };

    CILNode::TemporaryLocal(Box::new((
        info.target_type,
        [init_metadata, init_ptr].into(),
        CILNode::LoadTMPLocal,
    )))
}
/// Preforms an unsizing cast on operand `operand`, converting it to the `target` type.
pub fn unsize<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    operand: &Operand<'tcx>,
    target: Ty<'tcx>,
) -> CILNode {
    let info = UnsizeInfo::for_unsize(ctx, operand, target);
    let target_points_to = target.builtin_deref(true).unwrap();
    match (info.source_points_to.kind(), target_points_to.kind()) {
        (TyKind::Array(_, length), _) => {
            let length = crate::utilis::try_resolve_const_size(*length).unwrap();
            let metadata_field =
                FieldDescriptor::new(info.target_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field = FieldDescriptor::new(
                info.target_dotnet,
                Type::Ptr(Type::Void.into()),
                "data_pointer".into(),
            );
            let init_len = CILRoot::SetField {
                addr: Box::new(info.target_ptr.clone()),
                value: Box::new(conv_usize!(ldc_u64!(length as u64))),
                desc: Box::new(metadata_field),
            };
            let init_ptr = CILRoot::SetField {
                addr: Box::new(info.target_ptr),
                value: Box::new(info.source_ptr.cast_ptr(ptr!(Type::Void))),
                desc: Box::new(ptr_field),
            };
            CILNode::TemporaryLocal(Box::new((
                info.target_type,
                [init_len, init_ptr].into(),
                CILNode::LoadTMPLocal,
            )))
        }
        (
            TyKind::Dynamic(data_a, _, _src_dyn_kind),
            TyKind::Dynamic(data_b, _, _target_dyn_kind),
        ) => {
            // source_ptr should be fat
            ctx.assert_fat_pointer_type(&info.source_ptr, &());
            // info.target_ptr should be a thin pointer to a fat pointer.
            //ctx.assert_raw_pointer_type(&info.target_ptr, &());
            if data_a.principal_def_id() == data_b.principal_def_id() {
                return CILNode::TemporaryLocal(Box::new((
                    Type::DotnetType(Box::new(DotnetTypeRef::new::<&str, _>(None, "FatPtrDyn"))),
                    [CILRoot::SetTMPLocal {
                        value: *Box::new(info.source_ptr),
                    }]
                    .into(),
                    CILNode::LdObj {
                        ptr: Box::new(
                            CILNode::LoadAddresOfTMPLocal.cast_ptr(ptr!(info.target_type.clone())),
                        ),
                        obj: Box::new(info.target_type),
                    },
                )));
            }
            let vptr_entry_idx = ctx
                .tcx()
                .supertrait_vtable_slot((operand.ty(ctx.body(), ctx.tcx()), target));
            if let Some(entry_idx) = vptr_entry_idx {
                todo!("dyn to dyn cats not yet supported. src_points_to:{src_points_to:?} target_points_to:{target_points_to:?} entry_idx:{entry_idx:?}",src_points_to = info.source_points_to)
            } else {
                CILNode::TemporaryLocal(Box::new((
                    info.target_type.clone(),
                    [CILRoot::SetTMPLocal {
                        value: CILNode::LdObj {
                            ptr: Box::new(
                                ld_field!(
                                    info.source_ptr,
                                    FieldDescriptor::new(
                                        Type::DotnetType(Box::new(DotnetTypeRef::new::<&str, _>(
                                            None,
                                            "FatPtrDyn"
                                        )))
                                        .as_dotnet()
                                        .unwrap(),
                                        ptr!(Type::Void),
                                        "data_pointer".into(),
                                    )
                                )
                                .cast_ptr(ptr!(info.target_type.clone())),
                            ),
                            obj: Box::new(info.target_type),
                        },
                    }]
                    .into(),
                    CILNode::LoadTMPLocal,
                )))
            }
        }
        (_, TyKind::Dynamic(data, _, _dyn_kind)) => {
            println!("info.source_points_to:{:?}", info.source_points_to);
            let alloc_id = ctx
                .tcx()
                .vtable_allocation((info.source_points_to, data.principal()));
            let metadata_field =
                FieldDescriptor::new(info.target_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field =
                FieldDescriptor::new(info.target_dotnet, ptr!(Type::Void), "data_pointer".into());
            let init_vtable_ptr = CILRoot::SetField {
                addr: Box::new(info.target_ptr.clone()),
                value: Box::new(
                    CILNode::LoadGlobalAllocPtr {
                        alloc_id: alloc_id.0.into(),
                    }
                    .cast_ptr(Type::USize),
                ),
                desc: Box::new(metadata_field),
            };

            let init_obj_ptr = CILRoot::SetField {
                addr: Box::new(info.target_ptr),
                value: Box::new(info.source_ptr.cast_ptr(ptr!(Type::Void))),
                desc: Box::new(ptr_field),
            };
            CILNode::TemporaryLocal(Box::new((
                info.target_type,
                [init_vtable_ptr, init_obj_ptr].into(),
                CILNode::LoadTMPLocal,
            )))
        }
        (_, _) => todo!(
            "Unhandled unsizing cast:{:?} -> {target:?}",
            info.source_points_to
        ),
    }
}
/// Adopted from https://github.com/rust-lang/rustc_codegen_cranelift/blob/45600348c009303847e8cddcfa8483f1f3d56625/src/unsize.rs#L64
pub(crate) fn unsized_info<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_, '_>,
    source: Ty<'tcx>,
    target: Ty<'tcx>,
    old_info: Option<CILNode>,
) -> CILNode {
    let (source, target) =
        fx.tcx()
            .struct_lockstep_tails_erasing_lifetimes(source, target, ParamEnv::reveal_all());
    match (&source.kind(), &target.kind()) {
        (&TyKind::Array(_, len), &TyKind::Slice(_)) => conv_usize!(ldc_i64!(len
            .eval_target_usize(fx.tcx(), ParamEnv::reveal_all())
            as i64)),
        (
            &TyKind::Dynamic(data_a, _, src_dyn_kind),
            &TyKind::Dynamic(data_b, _, target_dyn_kind),
        ) if src_dyn_kind == target_dyn_kind => {
            let old_info =
                old_info.expect("unsized_info: missing old info for trait upcasting coercion");
            if data_a.principal_def_id() == data_b.principal_def_id() {
                // A NOP cast that doesn't actually change anything, should be allowed even with invalid vtables.
                return old_info;
            }

            // trait upcasting coercion
            let vptr_entry_idx = fx.tcx().supertrait_vtable_slot((source, target));

            if let Some(entry_idx) = vptr_entry_idx {
                let entry_idx = u32::try_from(entry_idx).unwrap();
                let entry_offset = ldc_u32!(entry_idx) * conv_u32!(size_of!(ptr!(Type::Void)));
                let vptr_ptr = CILNode::LDIndUSize {
                    ptr: Box::new(
                        (old_info + conv_usize!(entry_offset)).cast_ptr(ptr!(Type::USize)),
                    ),
                };
                vptr_ptr
            } else {
                old_info
            }
        }
        (_, TyKind::Dynamic(data, ..)) => get_vtable(fx, source, data.principal()),
        _ => panic!("unsized_info: invalid unsizing {source:?} -> {target:?}"),
    }
}

fn load_scalar_pair(addr: CILNode) -> (CILNode, CILNode) {
    (
        CILNode::LDIndUSize {
            ptr: Box::new(addr.clone()),
        },
        CILNode::LDIndUSize {
            ptr: Box::new(addr + conv_usize!(size_of!(Type::USize))),
        },
    )
}

pub(crate) fn get_vtable<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_, '_>,
    ty: Ty<'tcx>,
    trait_ref: Option<PolyExistentialTraitRef<'tcx>>,
) -> CILNode {
    let ty = fx.monomorphize(ty);
    let alloc_id = fx.tcx().vtable_allocation((ty, trait_ref));
    CILNode::LoadGlobalAllocPtr {
        alloc_id: alloc_id.0.get(),
    }
}
/// Coerce `src`, which is a reference to a value of type `src_ty`,
/// to a value of type `dst_ty` and store the result in `dst`
pub(crate) fn unsize_metadata<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_, '_>,
    src_cil: CILNode,
    src_ty: TyAndLayout<'tcx>,
    dst_ty: TyAndLayout<'tcx>,
) -> CILNode {
    let mut coerce_ptr = || {
        let info = if fx
            .layout_of(src_ty.ty.builtin_deref(true).unwrap())
            .is_unsized()
        {
            let (_, old_info) = load_scalar_pair(src_cil.clone());
            unsize_ptr_metadata(fx, src_ty, dst_ty, Some(old_info))
        } else {
            unsize_ptr_metadata(fx, src_ty, dst_ty, None)
        };
        info
    };

    match (&src_ty.ty.kind(), &dst_ty.ty.kind()) {
        (&TyKind::Ref(..), &TyKind::Ref(..) | &TyKind::RawPtr(..))
        | (&TyKind::RawPtr(..), &TyKind::RawPtr(..)) => coerce_ptr(),
        (&TyKind::Adt(def_a, subst_a), &TyKind::Adt(def_b, subst_b)) => {
            assert_eq!(def_a, def_b);

            for i in 0..def_a.variant(FIRST_VARIANT).fields.len() {
                let src_f = &def_a.variant(FIRST_VARIANT).fields[i.into()];
                let dst_f = &def_b.variant(FIRST_VARIANT).fields[i.into()];
                let src_f_ty = fx.layout_of(src_f.ty(fx.tcx(), subst_a));
                let dst_f_ty = fx.layout_of(dst_f.ty(fx.tcx(), subst_b));
                if src_f_ty.layout.is_zst() {
                    // No data here, nothing to copy/coerce.
                    continue;
                }
                if src_f_ty.ty != dst_f_ty.ty {
                    return unsize_metadata(fx, src_cil, src_f_ty, dst_f_ty);
                }
            }
            todo!()
        }
        _ => panic!("unsize_metadata: invalid coercion {src_ty:?} -> {dst_ty:?}",),
    }
}
/// Coerce `src` to `dst_ty`.
fn unsize_ptr_metadata<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_, '_>,

    src_layout: TyAndLayout<'tcx>,
    dst_layout: TyAndLayout<'tcx>,
    old_info: Option<CILNode>,
) -> CILNode {
    match (&src_layout.ty.kind(), &dst_layout.ty.kind()) {
        (&TyKind::Ref(_, a, _), &TyKind::Ref(_, b, _) | &TyKind::RawPtr(b, _))
        | (&TyKind::RawPtr(a, _), &TyKind::RawPtr(b, _)) => unsized_info(fx, *a, *b, old_info),
        (&TyKind::Adt(def_a, _), &TyKind::Adt(def_b, _)) => {
            assert_eq!(def_a, def_b);

            if src_layout == dst_layout {
                return old_info.unwrap();
            }

            let mut result = None;
            for i in 0..src_layout.fields.count() {
                let src_f = src_layout.field(fx, i);

                assert_eq!(
                    src_layout.fields.offset(i).bytes(),
                    0,
                    "{:?}",
                    src_layout.ty
                );
                assert_eq!(dst_layout.fields.offset(i).bytes(), 0);
                if src_f.is_1zst() {
                    // We are looking for the one non-1-ZST field; this is not it.
                    continue;
                }
                assert_eq!(src_layout.size, src_f.size);

                let dst_f = dst_layout.field(fx, i);
                assert_ne!(src_f.ty, dst_f.ty);
                assert_eq!(result, None);
                result = Some(unsize_ptr_metadata(fx, src_f, dst_f, old_info.clone()));
            }
            result.unwrap()
        }
        _ => panic!("unsize_ptr_metadata: called on bad types"),
    }
}
