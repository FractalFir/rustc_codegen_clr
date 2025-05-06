use crate::assembly::MethodCompileCtx;
use crate::rvalue::is_rvalue_unint;
use crate::utilis::adt::set_discr;
use rustc_codegen_clr_place::{place_adress, place_get, place_set};
use rustc_codegen_clr_type::utilis::is_zst;
use rustc_codegen_clr_type::GetTypeExt;

use cilly::{cil_node::V1Node, cil_root::V1Root, cil_tree::CILTree, size_of};
use cilly::{zero_extend, CILRoot};

use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::mir::{CopyNonOverlapping, NonDivergingIntrinsic, Statement, StatementKind};
#[allow(clippy::match_same_arms)]
pub fn handle_statement<'tcx>(
    statement: &Statement<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Vec<CILTree> {
    let kind = &statement.kind;
    match kind {
        StatementKind::StorageLive(_local) => vec![],
        StatementKind::StorageDead(_local) => vec![],
        StatementKind::SetDiscriminant {
            place,
            variant_index,
        } => {
            let owner_ty = place.ty(ctx.body(), ctx.tcx()).ty;
            let owner_ty = ctx.monomorphize(owner_ty);
            let owner = ctx.type_from_cache(owner_ty);

            let layout = ctx.layout_of(owner_ty);
            //let (disrc_type, _) = adt::enum_tag_info(&layout.layout, tcx);
            let cilly::Type::ClassRef(owner) = owner else {
                panic!("Nonsense operation: attempted to set the discriminant of type {owner_ty:?}, which is not valid.");
            };
            //ops.push();

            vec![set_discr(
                layout.layout,
                *variant_index,
                place_adress(place, ctx),
                owner,
                owner_ty,
                ctx,
            )
            .into()]
        }
        StatementKind::Assign(palce_rvalue) => {
            if is_rvalue_unint(&palce_rvalue.as_ref().1, ctx) {
                return vec![];
            }
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            let ty = ctx.monomorphize(place.ty(ctx.body(), ctx.tcx()).ty);
            // Skip void assigments. Assigining to or from void type is a NOP.
            if is_zst(ctx.monomorphize(ty), ctx.tcx()) {
                return vec![];
            }
            let tpe = ctx.type_from_cache(ty);
            let tpe = ctx.alloc_type(tpe);
            if crate::rvalue::is_rvalue_const_0(rvalue, ctx) {
                return vec![V1Root::InitObj(place_adress(&place, ctx), tpe).into()];
            }
            let (mut trees, value_calc) = crate::rvalue::handle_rvalue(rvalue, &place, ctx);
            trees.push(place_set(&place, value_calc, ctx));
            trees
                .into_iter()
                .map(|v1| {
                    let root = CILRoot::from_v1(&v1, ctx);
                    V1Root::V2(ctx.alloc_root(root)).into()
                })
                .collect()
        }
        StatementKind::Intrinsic(non_diverging_intirinsic) => {
            match non_diverging_intirinsic.as_ref() {
                NonDivergingIntrinsic::Assume(_) => vec![],
                NonDivergingIntrinsic::CopyNonOverlapping(CopyNonOverlapping {
                    src,
                    dst,
                    count,
                }) => {
                    let dst_op = handle_operand(dst, ctx);
                    let src_op = handle_operand(src, ctx);
                    let count_op = handle_operand(count, ctx);
                    let src_ty = src.ty(ctx.body(), ctx.tcx());
                    let src_ty = ctx.monomorphize(src_ty);
                    let ptr_type = ctx.type_from_cache(src_ty);
                    let cilly::Type::Ptr(pointed) = ptr_type else {
                        rustc_middle::ty::print::with_no_trimmed_paths! { panic!("Copy nonoverlaping called with non-pointer type {src_ty:?}")};
                    };

                    vec![V1Root::CpBlk {
                        src: Box::new(src_op),
                        dst: Box::new(dst_op),
                        len: Box::new(
                            count_op * V1Node::V2(zero_extend!(size_of!(pointed), usize)(ctx)),
                        ),
                    }
                    .into()]
                }
            }
        }
        StatementKind::FakeRead(_) => {
            panic!("Fake reads should not be passed from the backend to the forntend!")
        }
        rustc_middle::mir::StatementKind::BackwardIncompatibleDropHint { .. } => todo!(),
        StatementKind::PlaceMention(place) => vec![V1Root::Pop {
            tree: place_get(place, ctx),
        }
        .into()],
        //Since deinitialization writes "uninint" bytes to the place, it is safe to write nothing here. "uninit" bytes can be anything, so they can be what was there previously too.
        StatementKind::Deinit(_) => vec![],
        //TODO: consider adding some .NET specific coverage info(Is that even possible?).
        StatementKind::Coverage(_) => vec![],
        // A no-op in non-const scenarions, so safe to do nothing.
        StatementKind::ConstEvalCounter => vec![],
        // A no-op does nothing, so safe to do... nothing.
        StatementKind::Nop => vec![],
        // This is related to stacked borrow. TODO:consider emmiting info that would prevent wrong optimizations here.
        StatementKind::Retag(_, _) => vec![],
        // A no-op at runtime.
        StatementKind::AscribeUserType(_, _) => vec![],
    }
}
