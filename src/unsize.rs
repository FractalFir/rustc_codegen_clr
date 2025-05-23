use crate::assembly::MethodCompileCtx;

use cilly::cil_node::V1Node;
use cilly::cil_root::V1Root;

use cilly::{conv_u32, conv_usize, IntoAsmIndex};
use cilly::{Const, Type};
use cilly::{FieldDesc, Int};
use rustc_abi::FieldIdx;
use rustc_abi::FIRST_VARIANT;
use rustc_codegen_clr_place::place_address_raw;
use rustc_codegen_clr_type::r#type::fat_ptr_to;
use rustc_codegen_clr_type::utilis::is_fat_ptr;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codgen_clr_operand::constant::get_vtable;
use rustc_codgen_clr_operand::{handle_operand, operand_address};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{layout::TyAndLayout, Ty, TyKind, UintTy},
};
/// Preforms an unsizing cast on operand `operand`, converting it to the `target` type.
pub fn unsize2<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    operand: &Operand<'tcx>,
    target: Ty<'tcx>,
    destination: Place<'tcx>,
) -> (Vec<V1Root>, V1Node) {
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
    let fat_ptr_type = fat_ptr_to(Ty::new_uint(ctx.tcx(), UintTy::U8), ctx);

    let metadata_field = FieldDesc::new(
        fat_ptr_type,
        ctx.alloc_string(crate::METADATA),
        cilly::Type::Int(Int::USize),
    );
    let ptr_field = FieldDesc::new(
        fat_ptr_type,
        ctx.alloc_string(crate::DATA_PTR),
        ctx.nptr(cilly::Type::Void),
    );
    let dst = place_address_raw(&destination, ctx);
    let target_ptr = dst.clone();

    let init_metadata = V1Root::set_field(
        target_ptr.clone().cast_ptr(ctx.nptr(fat_ptr_type)),
        metadata.cast_ptr(Type::Int(Int::USize)),
        ctx.alloc_field(metadata_field),
    );

    let init_ptr = if is_fat_ptr(source, ctx.tcx(), ctx.instance()) {
        let void_ptr = ctx.nptr(Type::Void);
        V1Root::set_field(
            target_ptr.cast_ptr(ctx.nptr(fat_ptr_type)),
            V1Node::LDIndPtr {
                ptr: Box::new(operand_address(operand, ctx).cast_ptr(ctx.nptr(void_ptr))),
                loaded_ptr: Box::new(ctx.nptr(Type::Void)),
            },
            ctx.alloc_field(ptr_field),
        )
    } else {
        let operand = if source.is_any_ptr() {
            handle_operand(operand, ctx)
        } else {
            let source_type = ctx.type_from_cache(source);
            // If this type is a box<thin>, then its layout *should* be equivalent to a pointer, so this *should* be OK.
            V1Node::transmute_on_stack(
                handle_operand(operand, ctx),
                source_type,
                Type::Int(Int::USize),
                ctx,
            )
        };
        // `source` is not a fat pointer, so operand should be a pointer.

        V1Root::set_field(
            target_ptr.cast_ptr(ctx.nptr(fat_ptr_type)),
            operand.cast_ptr(ctx.nptr(Type::Void)),
            ctx.alloc_field(ptr_field),
        )
    };
    let source_size = ctx.layout_of(source).size.bytes();
    let target_size = ctx.layout_of(source).size.bytes();
    // Assumes a 64 bit pointer!
    let copy_val = if source_size > 8 && !source.is_any_ptr() && target_size != source_size {
        let addr = operand_address(operand, ctx);

        let addr = V1Node::Add(
            Box::new(addr),
            Box::new(V1Node::V2(ctx.alloc_node(8_isize))),
        );
        let dst_addr = V1Node::MRefToRawPtr(Box::new(dst.clone()));
        let const_16 = V1Node::V2(ctx.alloc_node(16_isize));
        let dst_addr = V1Node::Add(Box::new(dst_addr), Box::new(const_16));
        eprintln!("WARNING:Can't propely unsize types with sized fields yet. unsize assumes that layout of Wrapper<&T> ==   layout of Wrapper<FatPtr<T>>!");
        V1Root::CpBlk {
            dst: Box::new(dst_addr),
            src: Box::new(addr),
            len: Box::new(V1Node::V2(ctx.alloc_node(Const::USize(source_size - 8)))),
        }
    } else {
        V1Root::Nop
    };
    (
        [copy_val, init_metadata, init_ptr].into(),
        V1Node::LdObj {
            ptr: Box::new(dst.cast_ptr(ctx.nptr(target_type))),
            obj: Box::new(target_type),
        },
    )
}
/// Adopted from <https://github.com/rust-lang/rustc_codegen_cranelift/blob/45600348c009303847e8cddcfa8483f1f3d56625/src/unsize.rs#L64>
fn unsized_info<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    source: Ty<'tcx>,
    target: Ty<'tcx>,
    old_info: Option<V1Node>,
) -> V1Node {
    let (source, target) = ctx.tcx().struct_lockstep_tails_for_codegen(
        source,
        target,
        rustc_middle::ty::TypingEnv::fully_monomorphized(),
    );
    match (&source.kind(), &target.kind()) {
        (&TyKind::Array(_, len), &TyKind::Slice(_)) => {
            let len = len
                .try_to_target_usize(ctx.tcx())
                .expect("Could not eval array length.");
            V1Node::V2(ctx.alloc_node(Const::USize(len)))
        }
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
                let entry_offset = V1Node::V2(ctx.alloc_node(entry_idx))
                    * conv_u32!(V1Node::V2(ctx.size_of(Int::USize).into_idx(ctx)));
                V1Node::LDIndUSize {
                    ptr: Box::new(
                        (old_info + conv_usize!(entry_offset)).cast_ptr(ctx.nptr(Int::USize)),
                    ),
                }
            } else {
                old_info
            }
        }
        (_, TyKind::Dynamic(data, ..)) => get_vtable(
            ctx,
            source,
            data.principal()
                .map(|principal| ctx.tcx().instantiate_bound_regions_with_erased(principal)),
        )
        .into(),
        _ => panic!("unsized_info: invalid unsizing {source:?} -> {target:?}"),
    }
}

fn load_scalar_pair(addr: V1Node, ctx: &mut MethodCompileCtx<'_, '_>) -> (V1Node, V1Node) {
    (
        V1Node::LDIndUSize {
            ptr: Box::new(Box::new(addr.clone()).cast_ptr(ctx.nptr(Type::Int(Int::USize)))),
        },
        V1Node::LDIndUSize {
            ptr: Box::new(
                Box::new(addr + conv_usize!(V1Node::V2(ctx.size_of(Int::ISize).into_idx(ctx))))
                    .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
            ),
        },
    )
}
/// Coerce `src`, which is a reference to a value of type `src_ty`,
/// to a value of type `dst_ty` and store the result in `dst`
fn unsize_metadata<'tcx>(
    fx: &mut MethodCompileCtx<'tcx, '_>,
    src_cil: V1Node,
    src_ty: TyAndLayout<'tcx>,
    dst_ty: TyAndLayout<'tcx>,
) -> V1Node {
    let mut coerce_ptr = || {
        if fx
            .layout_of(src_ty.ty.builtin_deref(true).unwrap())
            .is_unsized()
        {
            let (_, old_info) = load_scalar_pair(src_cil.clone(), fx);
            unsize_ptr_metadata(fx, src_ty, dst_ty, Some(old_info))
        } else {
            unsize_ptr_metadata(fx, src_ty, dst_ty, None)
        }
    };

    match (&src_ty.ty.kind(), &dst_ty.ty.kind()) {
        (&TyKind::Ref(..), &TyKind::Ref(..) | &TyKind::RawPtr(..))
        | (&TyKind::RawPtr(..), &TyKind::RawPtr(..)) => coerce_ptr(),
        (&TyKind::Adt(def_a, subst_a), &TyKind::Adt(def_b, subst_b)) => {
            assert_eq!(def_a, def_b);

            for i in 0..def_a.variant(FIRST_VARIANT).fields.len() {
                let src_f = &def_a.variant(FIRST_VARIANT).fields[FieldIdx::from_usize(i)];
                let dst_f = &def_b.variant(FIRST_VARIANT).fields[FieldIdx::from_usize(i)];
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
    fx: &mut MethodCompileCtx<'tcx, '_>,

    src_layout: TyAndLayout<'tcx>,
    dst_layout: TyAndLayout<'tcx>,
    old_info: Option<V1Node>,
) -> V1Node {
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
// New unsizing semantics should use new local allocator
