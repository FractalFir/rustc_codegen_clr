use crate::assembly::MethodCompileCtx;
use crate::place::place_get;
use cilly::{cil_node::CILNode, cil_root::CILRoot, cil_tree::CILTree, size_of};
use cilly::{conv_usize, Type};

use rustc_middle::mir::{CopyNonOverlapping, NonDivergingIntrinsic, Statement, StatementKind};
#[allow(clippy::match_same_arms)]
pub fn handle_statement<'tcx>(
    statement: &Statement<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> Option<CILTree> {
    let kind = &statement.kind;
    match kind {
        StatementKind::StorageLive(_local) => None,
        StatementKind::StorageDead(_local) => None,
        StatementKind::SetDiscriminant {
            place,
            variant_index,
        } => {
            let owner_ty = place.ty(ctx.body(), ctx.tyctx()).ty;
            let owner_ty = ctx.monomorphize(owner_ty);
            let owner = ctx.type_from_cache(owner_ty);

            let layout = ctx.layout_of(owner_ty);
            //let (disrc_type, _) = crate::utilis::adt::enum_tag_info(&layout.layout, tyctx);
            let owner = if let crate::r#type::Type::DotnetType(dotnet_type) = owner {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            //ops.push();

            Some(
                crate::utilis::adt::set_discr(
                    layout.layout,
                    *variant_index,
                    crate::place::place_adress(place, ctx),
                    &owner,
                    ctx.tyctx(),
                    owner_ty,
                )
                .into(),
            )
        }
        StatementKind::Assign(palce_rvalue) => {
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            let ty = ctx.monomorphize(place.ty(ctx.body(), ctx.tyctx()).ty);
            // Skip void assigments. Assigining to or from void type is a NOP.
            if crate::utilis::is_zst(ctx.monomorphize(ty), ctx.tyctx()) {
                return None;
            }
            let value_calc = crate::rvalue::handle_rvalue(rvalue, &place, ctx);
            let method = ctx.instance();
            let tyctx = ctx.tyctx();
            let value_calc = crate::r#type::tycache::validity_check(
                value_calc,
                ty,
                ctx.type_cache(),
                method,
                tyctx,
            );
            Some(crate::place::place_set(&place, value_calc, ctx).into())
        }
        StatementKind::Intrinsic(non_diverging_intirinsic) => {
            match non_diverging_intirinsic.as_ref() {
                NonDivergingIntrinsic::Assume(_) => None,
                NonDivergingIntrinsic::CopyNonOverlapping(CopyNonOverlapping {
                    src,
                    dst,
                    count,
                }) => {
                    let dst_op = crate::operand::handle_operand(dst, ctx);
                    let src_op = crate::operand::handle_operand(src, ctx);
                    let count_op = crate::operand::handle_operand(count, ctx);
                    let src_ty = src.ty(ctx.body(), ctx.tyctx());
                    let src_ty = ctx.monomorphize(src_ty);
                    let ptr_type = ctx.type_from_cache(src_ty);
                    let crate::r#type::Type::Ptr(pointed) = ptr_type else {
                        rustc_middle::ty::print::with_no_trimmed_paths! { panic!("Copy nonoverlaping called with non-pointer type {src_ty:?}")};
                    };

                    Some(
                        CILRoot::CpBlk {
                            src: Box::new(src_op),
                            dst: Box::new(dst_op),
                            len: Box::new(count_op * conv_usize!(size_of!(pointed))),
                        }
                        .into(),
                    )
                }
            }
        }
        StatementKind::FakeRead(_) => {
            panic!("Fake reads should not be passed from the backend to the forntend!")
        }
        StatementKind::PlaceMention(place) => Some(
            CILRoot::Pop {
                tree: place_get(place, ctx),
            }
            .into(),
        ),
        //Since deinitialization writes "uninint" bytes to the place, it is safe to write nothing here. "uninit" bytes can be anything, so they can be what was there previously too.
        StatementKind::Deinit(_) => None,
        //TODO: consider adding some .NET specific coverage info(Is that even possible?).
        StatementKind::Coverage(_) => None,
        // A no-op in non-const scenarions, so safe to do nothing.
        StatementKind::ConstEvalCounter => None,
        // A no-op does nothing, so safe to do... nothing.
        StatementKind::Nop => None,
        // This is related to stacked borrow. TODO:consider emmiting info that would prevent wrong optimizations here.
        StatementKind::Retag(_, _) => None,
        // A no-op at runtime.
        StatementKind::AscribeUserType(_, _) => None,
    }
}
