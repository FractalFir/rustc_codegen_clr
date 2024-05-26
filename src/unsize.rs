use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;

use crate::operand::handle_operand;

use crate::{conv_usize, ld_field, ld_field_address, ldc_u64};

use crate::r#type::TyCache;
use cilly::field_desc::FieldDescriptor;
use cilly::{DotnetTypeRef, Type};
use rustc_middle::{
    mir::Operand,
    ty::{Instance, Ty, TyCtxt, TyKind},
};
struct UnsizeInfo<'tyctx> {
    /// Type the source pointer points to
    source_points_to: Ty<'tyctx>,
    target_ptr: CILNode,
    source_ptr: CILNode,
    target_dotnet: DotnetTypeRef,
    target_type: Type,
}
impl<'tyctx> UnsizeInfo<'tyctx> {
    pub fn for_unsize(
        tyctx: TyCtxt<'tyctx>,
        method: &rustc_middle::mir::Body<'tyctx>,
        method_instance: Instance<'tyctx>,
        tycache: &mut TyCache,
        operand: &Operand<'tyctx>,
        target: Ty<'tyctx>,
    ) -> Self {
        // Get the monomorphized source and target type
        let target = crate::utilis::monomorphize(&method_instance, target, tyctx);
        let source =
            crate::utilis::monomorphize(&method_instance, operand.ty(method, tyctx), tyctx);
        // Get the source and target types as .NET types
        let source_type = tycache.type_from_cache(source, tyctx, Some(method_instance));
        let target_type = tycache.type_from_cache(target, tyctx, Some(method_instance));
        // Get the target type as a fat pointer.
        let target_dotnet = target_type.as_dotnet().unwrap();
        let mut sized_ptr = handle_operand(operand, tyctx, method, method_instance, tycache);
        // Unsizing a box
        if target.is_box() && source.is_box() {
            // 1. Get Unqiue<Source> from Box<Source>
            let unique_desc =
                crate::utilis::field_descrptor(source, 0, tyctx, method_instance, tycache);
            let source_ptr = ld_field!(sized_ptr, unique_desc);
            // 2. Get NonNull<Source> from Unuqie<Source>
            let unique_adt = crate::utilis::as_adt(source).unwrap();
            let unique_ty = unique_adt.0.all_fields().nth(0).unwrap();
            let non_null_ptr_desc = crate::utilis::field_descrptor(
                unique_ty.ty(tyctx, unique_adt.1),
                0,
                tyctx,
                method_instance,
                tycache,
            );
            let source_ptr = ld_field!(source_ptr, non_null_ptr_desc.clone());
            // 3. Get Source* from NonNull<Source>
            let non_null_adt = crate::utilis::as_adt(unique_ty.ty(tyctx, unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let source_ptr_desc = crate::utilis::field_descrptor(
                non_null_ty.ty(tyctx, unique_adt.1),
                0,
                tyctx,
                method_instance,
                tycache,
            );
            let source_ptr = ld_field!(source_ptr, source_ptr_desc.clone());
            // 4. Get Unique<Target> from Box<Target>
            let unique_desc =
                crate::utilis::field_descrptor(target, 0, tyctx, method_instance, tycache);
            let target_ptr = ld_field_address!(CILNode::LoadAddresOfTMPLocal, unique_desc);
            // 5. Get NonNull<Target>  from Unique<Target>
            let unique_adt = crate::utilis::as_adt(target).unwrap();
            let unique_ty = unique_adt.0.all_fields().nth(0).unwrap();
            let target_ptr_desc = crate::utilis::field_descrptor(
                unique_ty.ty(tyctx, unique_adt.1),
                0,
                tyctx,
                method_instance,
                tycache,
            );
            let target_ptr = ld_field_address!(target_ptr, target_ptr_desc);
            // 6. Get Target* from NonNull<Target>
            let non_null_adt = crate::utilis::as_adt(unique_ty.ty(tyctx, unique_adt.1)).unwrap();
            let non_null_ty = non_null_adt.0.all_fields().nth(0).unwrap();
            let non_null_ptr_desc = crate::utilis::field_descrptor(
                non_null_ty.ty(tyctx, non_null_adt.1),
                0,
                tyctx,
                method_instance,
                tycache,
            );
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
                        let field_descriptor = crate::utilis::field_descrptor(
                            source,
                            0,
                            tyctx,
                            method_instance,
                            tycache,
                        );
                        sized_ptr = CILNode::TemporaryLocal(Box::new((
                            source_type,
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
    tyctx: TyCtxt<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
    operand: &Operand<'tyctx>,
    target: Ty<'tyctx>,
) -> CILNode {
    let info = UnsizeInfo::for_unsize(tyctx, method, method_instance, tycache, operand, target);
    match (
        info.source_points_to.kind(),
        target.builtin_deref(true).unwrap().kind(),
    ) {
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
                addr: info.target_ptr.clone(),
                value: conv_usize!(ldc_u64!(length as u64)),
                desc: metadata_field,
            };
            let init_ptr = CILRoot::SetField {
                addr: info.target_ptr,
                value: CILNode::TransmutePtr {
                    val: Box::new(info.source_ptr),
                    new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                },
                desc: ptr_field,
            };
            CILNode::TemporaryLocal(Box::new((
                info.target_type,
                [init_len, init_ptr].into(),
                CILNode::LoadTMPLocal,
            )))
        }
        (
            TyKind::Dynamic(_data_a, _, _src_dyn_kind),
            TyKind::Dynamic(_data_b, _, _target_dyn_kind),
        ) => {
            todo!("dyn to dyn cats not yet supported!")
        }
        (_, TyKind::Dynamic(data, _, _dyn_kind)) => {
            let alloc_id = tyctx.vtable_allocation((info.source_points_to, data.principal()));
            let metadata_field =
                FieldDescriptor::new(info.target_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field = FieldDescriptor::new(
                info.target_dotnet,
                Type::Ptr(Type::Void.into()),
                "data_pointer".into(),
            );
            let init_vtable_ptr = CILRoot::SetField {
                addr: info.target_ptr.clone(),
                value: CILNode::TransmutePtr {
                    val: Box::new(CILNode::LoadGlobalAllocPtr {
                        alloc_id: alloc_id.0.into(),
                    }),
                    new_ptr: Box::new(Type::USize),
                },
                desc: metadata_field,
            };
            let init_obj_ptr = CILRoot::SetField {
                addr: info.target_ptr,
                value: CILNode::TransmutePtr {
                    val: Box::new(info.source_ptr),
                    new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                },
                desc: ptr_field,
            };
            CILNode::TemporaryLocal(Box::new((
                info.target_type,
                [init_vtable_ptr, init_obj_ptr].into(),
                CILNode::LoadTMPLocal,
            )))
        }
        (_, _) => todo!(
            "Unhandled unsizing cast:{source:?} -> {target:?}",
            source = info.source_points_to
        ),
    }
}
