use crate::assembly::MethodCompileCtx;
use crate::operand::handle_operand;
use crate::r#type::pointer_to_is_fat;

use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::{conv_usize, ld_field, ld_field_address, ldc_u64, ptr};

use cilly::field_desc::FieldDescriptor;
use cilly::{DotnetTypeRef, Type};
use rustc_middle::{
    mir::Operand,
    ty::{Ty, TyKind},
};
struct UnsizeInfo<'tyctx> {
    /// Type the source pointer points to
    source_points_to: Ty<'tyctx>,
    /// The address of the rarget pointer. This pointer should always be a thin pointer, pointing to a fat pointer.
    target_ptr: CILNode,
    /// The source pointer. If the source pointer is fat, this will be a fat pointer!
    source_ptr: CILNode,
    target_dotnet: DotnetTypeRef,
    target_type: Type,
}
impl<'tyctx> UnsizeInfo<'tyctx> {
    pub fn for_unsize(
        ctx: &mut MethodCompileCtx<'tyctx, '_, '_>,
        operand: &Operand<'tyctx>,
        target: Ty<'tyctx>,
    ) -> Self {
        // Get the monomorphized source and target type
        let target = ctx.monomorphize(target);
        let source = ctx.monomorphize(operand.ty(ctx.body(), ctx.tyctx()));
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
                crate::utilis::field_descrptor(unique_ty.ty(ctx.tyctx(), unique_adt.1), 0, ctx);
            let source_ptr = ld_field!(source_ptr, non_null_ptr_desc.clone());
            // 3. Get Source* from NonNull<Source>
            let non_null_adt =
                crate::utilis::as_adt(unique_ty.ty(ctx.tyctx(), unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let source_ptr_desc =
                crate::utilis::field_descrptor(non_null_ty.ty(ctx.tyctx(), unique_adt.1), 0, ctx);

            let source_ptr = ld_field!(source_ptr, source_ptr_desc.clone());
            // 4. Get Unique<Target> from Box<Target>
            let unique_desc = crate::utilis::field_descrptor(target, 0, ctx);
            let target_ptr = ld_field_address!(CILNode::LoadAddresOfTMPLocal, unique_desc);
            // 5. Get NonNull<Target>  from Unique<Target>
            let unique_adt = crate::utilis::as_adt(target).unwrap();
            let unique_ty = unique_adt.0.all_fields().nth(0).unwrap();
            let target_ptr_desc =
                crate::utilis::field_descrptor(unique_ty.ty(ctx.tyctx(), unique_adt.1), 0, ctx);
            let target_ptr = ld_field_address!(target_ptr, target_ptr_desc);
            // 6. Get Target* from NonNull<Target>
            let non_null_adt =
                crate::utilis::as_adt(unique_ty.ty(ctx.tyctx(), unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let non_null_ptr_desc =
                crate::utilis::field_descrptor(non_null_ty.ty(ctx.tyctx(), non_null_adt.1), 0, ctx);
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
pub fn unsize<'tyctx>(
    ctx: &mut MethodCompileCtx<'tyctx, '_, '_>,
    operand: &Operand<'tyctx>,
    target: Ty<'tyctx>,
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
                .tyctx()
                .supertrait_vtable_slot((operand.ty(ctx.body(), ctx.tyctx()), target));
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
            let alloc_id = ctx
                .tyctx()
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
