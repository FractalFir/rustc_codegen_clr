use crate::assembly::MethodCompileCtx;
use crate::operand::{handle_operand, operand_address};
use crate::r#type::{fat_ptr_to, pointer_to_is_fat};
use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::field_desc::FieldDescriptor;
use cilly::v2::{ClassRefIdx, Int};
use cilly::Type;
use cilly::{conv_u32, conv_usize, ld_field, ld_field_address, ldc_i64, ldc_u32, size_of};
use rustc_middle::{
    mir::Operand,
    ty::{layout::TyAndLayout, ParamEnv, PolyExistentialTraitRef, Ty, TyKind, UintTy},
};
use rustc_target::abi::FIRST_VARIANT;
struct UnsizeInfo<'tcx> {
    /// Type the source pointer points to
    source_points_to: Ty<'tcx>,
    /// The address of the rarget pointer. This pointer should always be a thin pointer, pointing to a fat pointer.
    target_ptr: CILNode,

    target_dotnet: ClassRefIdx,
    target_type: Type,
    source_ptr: CILNode,
}
impl<'tcx> UnsizeInfo<'tcx> {
    fn new(
        source_points_to: Ty<'tcx>,
        target_ptr: CILNode,
        target_dotnet: ClassRefIdx,
        target_type: Type,
        source_ptr: CILNode,
    ) -> Self {
        Self {
            source_points_to,
            target_ptr,
            target_dotnet,
            target_type,
            source_ptr,
        }
    }

    pub fn for_unsize(
        ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
        operand: &Operand<'tcx>,
        target: Ty<'tcx>,
    ) -> Self {
        // Get the monomorphized source and target type
        let target = ctx.monomorphize(target);
        let source = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
        // Get the source and target types as .NET types

        let target_type = ctx.type_from_cache(target);
        // Get the target type as a fat pointer.
        let target_dotnet = target_type.as_class_ref().unwrap();

        // Unsizing a box
        if target.is_box() && source.is_box() {
            let sized_ptr = operand_address(operand, ctx);
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

            Self::new(
                derefed_source,
                target_ptr,
                non_null_ptr_desc.tpe().as_class_ref().unwrap(),
                target_type,
                source_ptr,
            )
        } else {
            let derefed_source = source.builtin_deref(true).unwrap();
            let sized_ptr = if pointer_to_is_fat(derefed_source, ctx.tcx(), ctx.instance()) {
                operand_address(operand, ctx)
            } else {
                handle_operand(operand, ctx)
            };

            Self::new(
                derefed_source,
                CILNode::LoadAddresOfTMPLocal,
                target_dotnet,
                target_type,
                sized_ptr,
            )
        }
    }
}

/// Preforms an unsizing cast on operand `operand`, converting it to the `target` type.
pub fn unsize2<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    operand: &Operand<'tcx>,
    target: Ty<'tcx>,
) -> CILNode {
    // Get the monomorphized source and target type
    let target = ctx.monomorphize(target);
    let source = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
    // Get the source and target types as .NET types

    let target_type = ctx.type_from_cache(target);
    // Get the target type as a fat pointer.

    let src_cil = operand_address(operand, ctx);

    let metadata = unsize_metadata(
        ctx,
        src_cil,
        ctx.layout_of(operand.ty(ctx.body(), ctx.tcx())),
        ctx.layout_of(target),
    );
    let fat_ptr_type = fat_ptr_to(Ty::new(ctx.tcx(), TyKind::Uint(UintTy::U8)), ctx);

    let metadata_field = FieldDescriptor::new(
        fat_ptr_type.clone(),
        cilly::v2::Type::Int(Int::USize),
        crate::METADATA.into(),
    );
    let ptr_field = FieldDescriptor::new(
        fat_ptr_type.clone(),
        ctx.asm_mut().nptr(cilly::v2::Type::Void),
        crate::DATA_PTR.into(),
    );

    let target_ptr = CILNode::LoadAddresOfTMPLocal;

    let init_metadata = CILRoot::set_field(
        target_ptr
            .clone()
            .cast_ptr(ctx.asm_mut().nptr(fat_ptr_type.clone().into())),
        metadata.cast_ptr(Type::Int(Int::USize)),
        metadata_field,
        ctx.validator(),
        Some(&target_type),
    );
    init_metadata
        .validate(ctx.validator(), Some(&target_type))
        .expect("init_metadata invalid!");

    let init_ptr = if crate::r#type::is_fat_ptr(source, ctx.tcx(), ctx.instance()) {
        let void_ptr = ctx.asm_mut().nptr(Type::Void);
        CILRoot::set_field(
            target_ptr.cast_ptr(ctx.asm_mut().nptr(fat_ptr_type.clone().into())),
            CILNode::LDIndPtr {
                ptr: Box::new(operand_address(operand, ctx).cast_ptr(ctx.asm_mut().nptr(void_ptr))),
                loaded_ptr: Box::new(ctx.asm_mut().nptr(Type::Void)),
            },
            ptr_field,
            ctx.validator(),
            Some(&target_type),
        )
    } else {
        let operand = if !source.is_any_ptr() {
            let source_type = ctx.type_from_cache(source);
            // If this type is a box<thin>, then its layout *should* be equivalent to a pointer, so this *should* be OK.
            CILNode::LDIndUSize {
                ptr: Box::new(
                    CILNode::TemporaryLocal(Box::new((
                        source_type,
                        Box::new([CILRoot::SetTMPLocal {
                            value: handle_operand(operand, ctx),
                        }]),
                        CILNode::LoadAddresOfTMPLocal,
                    )))
                    .cast_ptr(ctx.asm_mut().nptr(Type::Int(Int::USize))),
                ),
            }
        } else {
            handle_operand(operand, ctx)
        };
        // `source` is not a fat pointer, so operand should be a pointer.

        CILRoot::set_field(
            target_ptr.cast_ptr(ctx.asm_mut().nptr(fat_ptr_type.clone().into())),
            operand.cast_ptr(ctx.asm_mut().nptr(Type::Void)),
            ptr_field,
            ctx.validator(),
            Some(&target_type),
        )
    };

    init_ptr
        .validate(ctx.validator(), Some(&target_type))
        .expect("init_ptr invalid!");

    let res = CILNode::LdObj {
        ptr: Box::new(
            CILNode::TemporaryLocal(Box::new((
                Type::ClassRef(fat_ptr_type).clone(),
                [init_metadata, init_ptr].into(),
                CILNode::LoadAddresOfTMPLocal,
            )))
            .cast_ptr(ctx.asm_mut().nptr(target_type.clone())),
        ),
        obj: Box::new(target_type.clone()),
    };

    res
}
/// Adopted from https://github.com/rust-lang/rustc_codegen_cranelift/blob/45600348c009303847e8cddcfa8483f1f3d56625/src/unsize.rs#L64
pub(crate) fn unsized_info<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    source: Ty<'tcx>,
    target: Ty<'tcx>,
    old_info: Option<CILNode>,
) -> CILNode {
    let (source, target) =
        ctx.tcx()
            .struct_lockstep_tails_for_codegen(source, target, ParamEnv::reveal_all());
    match (&source.kind(), &target.kind()) {
        (&TyKind::Array(_, len), &TyKind::Slice(_)) => conv_usize!(ldc_i64!(len
            .eval_target_usize(ctx.tcx(), ParamEnv::reveal_all())
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
            let vptr_entry_idx = ctx.tcx().supertrait_vtable_slot((source, target));

            if let Some(entry_idx) = vptr_entry_idx {
                let entry_idx = u32::try_from(entry_idx).unwrap();
                let entry_offset =
                    ldc_u32!(entry_idx) * conv_u32!(size_of!(ctx.asm_mut().nptr(Type::Void)));
                CILNode::LDIndUSize {
                    ptr: Box::new(
                        (old_info + conv_usize!(entry_offset))
                            .cast_ptr(ctx.asm_mut().nptr(Type::Int(Int::USize))),
                    ),
                }
            } else {
                old_info
            }
        }
        (_, TyKind::Dynamic(data, ..)) => get_vtable(ctx, source, data.principal()),
        _ => panic!("unsized_info: invalid unsizing {source:?} -> {target:?}"),
    }
}

fn load_scalar_pair(
    addr: CILNode,
    ctx: &mut MethodCompileCtx<'_, '_, '_, '_>,
) -> (CILNode, CILNode) {
    (
        CILNode::LDIndUSize {
            ptr: Box::new(
                Box::new(addr.clone()).cast_ptr(ctx.asm_mut().nptr(Type::Int(Int::USize))),
            ),
        },
        CILNode::LDIndUSize {
            ptr: Box::new(
                Box::new(addr + conv_usize!(size_of!(Type::Int(Int::USize))))
                    .cast_ptr(ctx.asm_mut().nptr(Type::Int(Int::USize))),
            ),
        },
    )
}

pub(crate) fn get_vtable<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
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
    fx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    src_cil: CILNode,
    src_ty: TyAndLayout<'tcx>,
    dst_ty: TyAndLayout<'tcx>,
) -> CILNode {
    let mut coerce_ptr = || {
        let info = if fx
            .layout_of(src_ty.ty.builtin_deref(true).unwrap())
            .is_unsized()
        {
            let (_, old_info) = load_scalar_pair(src_cil.clone(), fx);
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
    fx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,

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
