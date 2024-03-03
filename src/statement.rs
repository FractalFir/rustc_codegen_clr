use crate::{
    cil_tree::{cil_node::CILNode, cil_root::CILRoot, CILTree},
    ldc_i32, mul,
    r#type::{TyCache, Type},
    size_of,
};
use rustc_middle::{
    mir::{Body, CopyNonOverlapping, NonDivergingIntrinsic, Statement, StatementKind},
    ty::{Instance, ParamEnv, TyCtxt},
};
pub fn handle_statement<'tcx>(
    statement: &Statement<'tcx>,
    tyctx: TyCtxt<'tcx>,
    method: &Body<'tcx>,
    method_instance: Instance<'tcx>,
    type_cache: &mut TyCache,
) -> Option<CILTree> {
    let kind = &statement.kind;
    match kind {
        StatementKind::StorageLive(_local) => None,
        StatementKind::StorageDead(_local) => None,
        StatementKind::SetDiscriminant {
            place,
            variant_index,
        } => {
            let owner_ty = place.ty(method, tyctx).ty;
            let owner_ty = crate::utilis::monomorphize(&method_instance, owner_ty, tyctx);
            let owner = type_cache.type_from_cache(owner_ty, tyctx, Some(method_instance));

            let layout = tyctx
                .layout_of(rustc_middle::ty::ParamEnvAnd {
                    param_env: ParamEnv::reveal_all(),
                    value: owner_ty,
                })
                .expect("Could not get type layout!");
            let (disrc_type, _) = crate::utilis::adt::enum_tag_info(&layout.layout, tyctx);
            let owner = if let crate::r#type::Type::DotnetType(dotnet_type) = owner {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            //ops.push();
            Some(
                CILRoot::SetField {
                    desc: crate::cil::FieldDescriptor::new(
                        owner,
                        disrc_type.clone(),
                        "value__".into(),
                    ),
                    addr: crate::place::place_adress(
                        place,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    ),
                    value: crate::casts::int_to_int(
                        Type::I32,
                        disrc_type,
                        ldc_i32!(variant_index.as_u32() as i32),
                    ),
                }
                .into(),
            )
        }
        StatementKind::Assign(palce_rvalue) => {
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            // Skip void assigments. Assigining to or from void type is a NOP.
            if type_cache.type_from_cache(
                crate::utilis::monomorphize(&method_instance, place.ty(method, tyctx).ty, tyctx),
                tyctx,
                Some(method_instance),
            ) == crate::r#type::Type::Void
            {
                return None;
            }

            Some(
                crate::place::place_set(
                    &place,
                    tyctx,
                    crate::rvalue::handle_rvalue(
                        rvalue,
                        tyctx,
                        &place,
                        method,
                        method_instance,
                        type_cache,
                    ),
                    method,
                    method_instance,
                    type_cache,
                )
                .into(),
            )
        }
        StatementKind::Intrinsic(non_diverging_intirinsic) => {
            match non_diverging_intirinsic.as_ref() {
                NonDivergingIntrinsic::Assume(_) => None,
                NonDivergingIntrinsic::CopyNonOverlapping(CopyNonOverlapping {
                    src,
                    dst,
                    count,
                }) => {
                    let dst_op = crate::operand::handle_operand(
                        dst,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let src_op = crate::operand::handle_operand(
                        src,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let count_op = crate::operand::handle_operand(
                        count,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let src_ty = src.ty(method, tyctx);
                    let src_ty = crate::utilis::monomorphize(&method_instance, src_ty, tyctx);
                    let ptr_type = type_cache.type_from_cache(src_ty, tyctx, Some(method_instance));
                    let crate::r#type::Type::Ptr(pointed) = ptr_type else {
                        rustc_middle::ty::print::with_no_trimmed_paths! { panic!("Copy nonoverlaping called with non-pointer type {src_ty:?}")};
                    };

                    Some(
                        CILRoot::CpBlk {
                            src: src_op,
                            dst: dst_op,
                            len: mul!(count_op, size_of!(pointed)),
                        }
                        .into(),
                    )
                }
            }
        }
        _ => {
            rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Unsuported statement kind {kind:?}")}
        }
    }
}
