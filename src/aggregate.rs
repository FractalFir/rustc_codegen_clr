use crate::{
    cil::{CallSite, FieldDescriptor},
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_usize, ldc_u64,
    operand::handle_operand,
    place::place_get,
    r#type::{DotnetTypeRef, TyCache, Type},
    utilis::{adt::set_discr, field_name, monomorphize},
};
use rustc_index::IndexVec;
use rustc_middle::mir::{AggregateKind, Operand, Place};
use rustc_middle::ty::{AdtDef, AdtKind, GenericArg, Instance, List, ParamEnv, Ty, TyCtxt, TyKind};
use rustc_target::abi::FieldIdx;
/// Returns the CIL ops to create the aggreagate value specifed by `aggregate_kind` at `target_location`. Uses indivlidual values specifed by `value_index`
pub fn handle_aggregate<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    target_location: &Place<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    aggregate_kind: &AggregateKind<'tyctx>,
    value_index: &IndexVec<FieldIdx, Operand<'tyctx>>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    // Get CIL ops for each value
    let values: Vec<_> = value_index
        .iter()
        .enumerate()
        .map(|operand| {
            (
                operand.0 as u32,
                crate::operand::handle_operand(operand.1, tyctx, method, method_instance, tycache),
            )
        })
        .collect();
    match aggregate_kind {
        AggregateKind::Adt(adt_def, variant_idx, subst, _utai, active_field) => {
            let penv = ParamEnv::reveal_all();
            let subst = crate::utilis::monomorphize(&method_instance, *subst, tyctx);
            //eprintln!("Preparing to resolve {adt_def:?} {subst:?}");
            let adt_type = Instance::resolve(tyctx, penv, *adt_def, subst);

            let adt_type = adt_type
                .expect("Could not resolve instance")
                .expect("Could not resolve instance")
                .ty(tyctx, penv);
            let adt_type = monomorphize(&method_instance, adt_type, tyctx);
            let (adt_def, subst) = if let TyKind::Adt(def_id, subst) = adt_type.kind() {
                (def_id, subst)
            } else {
                panic!("Type {adt_type:?} is not a Algebraic Data Type!");
            };
            aggregate_adt(
                tyctx,
                target_location,
                method,
                adt_def,
                adt_type,
                subst,
                variant_idx.as_u32(),
                values,
                method_instance,
                active_field,
                tycache,
            )
        }
        AggregateKind::Array(element) => {
            let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
            let element = tycache.type_from_cache(element, tyctx, Some(method_instance));
            let array_type = DotnetTypeRef::array(element.clone(), value_index.len());
            let array_getter = super::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                tycache,
            );
            let sig = crate::function_sig::FnSig::new(
                &[
                    Type::Ptr(Into::<Type>::into(array_type.clone()).into()),
                    Type::USize,
                    element,
                ],
                &Type::Void,
            );
            let site = CallSite::new(Some(array_type), "set_Item".into(), sig, false);
            let mut sub_trees = Vec::new();
            for value in values {
                sub_trees.push(CILRoot::Call {
                    site: site.clone(),
                    args: [
                        array_getter.clone(),
                        conv_usize!(ldc_u64!(value.0 as u64)),
                        value.1,
                    ]
                    .into(),
                });
            }
            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(super::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    tycache,
                )),
            )
        }
        AggregateKind::Tuple => {
            let tuple_getter = super::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                tycache,
            );
            let types: Vec<_> = value_index
                .iter()
                .map(|operand| {
                    let operand_ty = crate::utilis::monomorphize(
                        &method_instance,
                        operand.ty(method, tyctx),
                        tyctx,
                    );
                    tycache.type_from_cache(operand_ty, tyctx, Some(method_instance))
                })
                .collect();
            let dotnet_tpe = crate::r#type::simple_tuple(&types);
            let mut sub_trees = Vec::new();
            for field in values.iter() {
                let name = format!("Item{}", field.0 + 1);
                sub_trees.push(CILRoot::SetField {
                    addr: tuple_getter.clone(),
                    value: field.1.clone(),
                    desc: FieldDescriptor::new(
                        dotnet_tpe.clone(),
                        types[field.0 as usize].clone(),
                        name.into(),
                    ),
                });
            }
            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(super::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    tycache,
                )),
            )
        }
        AggregateKind::Closure(_def_id, _args) => {
            let closure_ty = crate::utilis::monomorphize(
                &method_instance,
                target_location.ty(method, tyctx),
                tyctx,
            )
            .ty;
            let closure_type = tycache.type_from_cache(closure_ty, tyctx, Some(method_instance));
            let closure_dotnet = closure_type.as_dotnet().expect("Invalid closure type!");
            let closure_getter = super::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                tycache,
            );
            let mut sub_trees = vec![];
            for (index, value) in value_index.iter_enumerated() {
                let field_ty =
                    crate::utilis::monomorphize(&method_instance, value.ty(method, tyctx), tyctx);
                let field_ty = tycache.type_from_cache(field_ty, tyctx, Some(method_instance));

                sub_trees.push(CILRoot::SetField {
                    addr: closure_getter.clone(),
                    value: handle_operand(value, tyctx, method, method_instance, tycache),
                    desc: FieldDescriptor::new(
                        closure_dotnet.clone(),
                        field_ty,
                        format!("f_{}", index.as_u32()).into(),
                    ),
                })
            }

            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    tycache,
                )),
            )
        }
        _ => todo!("Unsuported aggregate kind {aggregate_kind:?}"),
    }
}
/// Builds an Algebraic Data Type (struct,enum,union) at location `target_location`, with fields set using ops in `fields`.
fn aggregate_adt<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    target_location: &Place<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    adt: &AdtDef<'tyctx>,
    adt_type: Ty<'tyctx>,
    subst: &'tyctx List<GenericArg<'tyctx>>,
    variant_idx: u32,
    fields: Vec<(u32, CILNode)>,
    method_instance: Instance<'tyctx>,
    _active_field: &Option<FieldIdx>,
    type_cache: &mut crate::r#type::TyCache,
) -> CILNode {
    let adt_type = crate::utilis::monomorphize(&method_instance, adt_type, tyctx);
    let adt_type_ref = type_cache.type_from_cache(adt_type, tyctx, Some(method_instance));
    let adt_type_ref = if let Type::DotnetType(type_ref) = adt_type_ref {
        type_ref.as_ref().clone()
    } else {
        panic!("Can't get fields of type {adt_type:?}");
    };
    match adt.adt_kind() {
        AdtKind::Struct => {
            let obj_getter = crate::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            );

            let mut sub_trees = Vec::new();
            for field in fields {
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let field_type = field_def.ty(tyctx, subst);
                let field_type = crate::utilis::monomorphize(&method_instance, field_type, tyctx);
                let field_type =
                    type_cache.type_from_cache(field_type, tyctx, Some(method_instance));
                // Seting a void field is a no-op.
                if field_type == Type::Void {
                    continue;
                }
                let field_desc = crate::utilis::field_descrptor(
                    adt_type,
                    field.0,
                    tyctx,
                    method_instance,
                    type_cache,
                );

                sub_trees.push(CILRoot::SetField {
                    addr: obj_getter.clone(),
                    value: field.1,
                    desc: field_desc,
                });
            }
            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(crate::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    type_cache,
                )),
            )
        }
        AdtKind::Enum => {
            let adt_adress_ops = crate::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            );

            let variant_name = crate::utilis::variant_name(adt_type, variant_idx);

            let variant_address = adt_adress_ops.clone();
            let mut sub_trees = Vec::new();
            let enum_variant = adt
                .variants()
                .iter()
                .nth(variant_idx as usize)
                .expect("Can't get variant index");
            for (field, field_value) in enum_variant.fields.iter().zip(fields.iter()) {
                let field_name = format!(
                    "{variant_name}_{fname}",
                    fname = crate::r#type::escape_field_name(&field.name.to_string())
                )
                .into();
                let field_type = type_cache.type_from_cache(
                    field.ty(tyctx, subst),
                    tyctx,
                    Some(method_instance),
                );
                // Seting a void field is a no-op.
                if field_type == Type::Void {
                    continue;
                }

                sub_trees.push(CILRoot::SetField {
                    addr: variant_address.clone(),
                    value: field_value.1.clone(),
                    desc: FieldDescriptor::new(adt_type_ref.clone(), field_type, field_name),
                });
            }
            // Set tag
            {
                let layout = tyctx
                    .layout_of(rustc_middle::ty::ParamEnvAnd {
                        param_env: ParamEnv::reveal_all(),
                        value: adt_type,
                    })
                    .expect("Could not get type layout!");
                let (disrc_type, _) = crate::utilis::adt::enum_tag_info(&layout.layout, tyctx);
                if disrc_type != Type::Void {
                    sub_trees.push(set_discr(
                        &layout.layout,
                        variant_idx.into(),
                        adt_adress_ops,
                        adt_type_ref,
                        tyctx,
                        layout.ty,
                    ))
                }
            }
            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(crate::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    type_cache,
                )),
            )
        }
        AdtKind::Union => {
            let obj_getter = crate::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            );

            let mut sub_trees = Vec::new();
            for field in fields {
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let field_type = field_def.ty(tyctx, subst);
                let field_type = crate::utilis::monomorphize(&method_instance, field_type, tyctx);
                let field_type =
                    type_cache.type_from_cache(field_type, tyctx, Some(method_instance));
                // Seting a void field is a no-op.
                if field_type == Type::Void {
                    continue;
                }

                let field_name = field_name(adt_type, field.0);

                let desc = FieldDescriptor::new(adt_type_ref.clone(), field_type, field_name);
                sub_trees.push(CILRoot::SetField {
                    addr: obj_getter.clone(),
                    value: field.1,
                    desc,
                });
            }

            CILNode::SubTrees(
                sub_trees.into(),
                Box::new(crate::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    type_cache,
                )),
            )
        }
    }
}
