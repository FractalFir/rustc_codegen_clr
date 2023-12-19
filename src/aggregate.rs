use crate::{
    cil::CallSite,
    cil::{CILOp, FieldDescriptor},
    r#type::{DotnetTypeRef, TyCache, Type},
    utilis::{field_name, monomorphize},
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
) -> Vec<CILOp> {
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
            let mut ops: Vec<CILOp> = Vec::with_capacity(values.len() * 2);
            let array_getter = super::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                tycache,
            );
            let sig = crate::function_sig::FnSig::new(
                &[array_type.clone().into(), Type::USize, Type::GenericArg(0)],
                &Type::Void,
            );
            let call_site = CallSite::boxed(Some(array_type), "set_Item".into(), sig, false);
            for value in values {
                ops.extend(array_getter.iter().cloned());
                ops.push(CILOp::LdcI64(value.0 as u64 as i64));
                ops.push(CILOp::ConvUSize(false));
                ops.extend(value.1);
                ops.push(CILOp::Call(call_site.clone()));
            }
            ops.extend(super::place::place_get(
                target_location,
                tyctx,
                method,
                method_instance,
                tycache,
            ));
            ops
        }
        AggregateKind::Tuple => {
            if values.len() > 8 {
                todo!("Tuples with more than 8 fields are not supported yet.");
            } else {
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
                let mut ops: Vec<CILOp> = Vec::with_capacity(values.len() * 2);
                for field in values.iter() {
                    let name = format!("Item{}", field.0 + 1);
                    ops.extend(tuple_getter.iter().cloned());
                    ops.extend(field.1.iter().cloned());
                    ops.push(CILOp::STField(FieldDescriptor::boxed(
                        dotnet_tpe.clone(),
                        Type::GenericArg(field.0),
                        name.into(),
                    )));
                }
                ops.extend(super::place::place_get(
                    target_location,
                    tyctx,
                    method,
                    method_instance,
                    tycache,
                ));
                ops
            }
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
    fields: Vec<(u32, Vec<CILOp>)>,
    method_instance: Instance<'tyctx>,
    _active_field: &Option<FieldIdx>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
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
            let mut ops: Vec<CILOp> = Vec::with_capacity(fields.len() * 2);
            for field in fields {
                ops.extend(obj_getter.iter().cloned());
                ops.extend(field.1);
                let field_desc = crate::utilis::field_descrptor(
                    adt_type,
                    field.0,
                    tyctx,
                    method_instance,
                    type_cache,
                );
                ops.push(CILOp::STField(field_desc.into()));
            }
            ops.extend(crate::place::place_get(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops
        }
        AdtKind::Enum => {
            let adt_adress_ops = crate::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            );

            let mut variant_type = adt_type_ref.clone(); //adt_type.variant_type(variant).expect("Can't get variant index");
            let variant_name = crate::utilis::variant_name(adt_type, variant_idx);
            variant_type.append_path(&format!("/{variant_name}"));
            // Get variant adress
            let variant_field_desc = FieldDescriptor::new(
                adt_type_ref.clone(),
                Type::DotnetType(Box::new(variant_type.clone())),
                format!("v_{variant_name}").into(),
            );
            let mut variant_address = adt_adress_ops.clone();
            variant_address.push(CILOp::LDFieldAdress(Box::new(variant_field_desc)));
            let mut ops = Vec::new();
            let enum_variant = adt
                .variants()
                .iter()
                .nth(variant_idx as usize)
                .expect("Can't get variant index");
            for (field, field_value) in enum_variant.fields.iter().zip(fields.iter()) {
                ops.extend(variant_address.clone());
                ops.extend(field_value.1.clone());
                let field_name = field.name.to_string();
                let field_name = crate::r#type::escape_field_name(&field_name);
                let field_type = type_cache.type_from_cache(
                    field.ty(tyctx, subst),
                    tyctx,
                    Some(method_instance),
                );
                ops.push(CILOp::STField(Box::new(FieldDescriptor::new(
                    variant_type.clone(),
                    field_type,
                    field_name,
                ))));
            }
            // Set tag
            {
                ops.extend(adt_adress_ops);
                ops.push(CILOp::LdcI32(variant_idx as i32));
                let field_name = "_tag".into();
                let field = Type::U8;
                ops.push(CILOp::STField(Box::new(FieldDescriptor::new(
                    adt_type_ref,
                    field,
                    field_name,
                ))));
            }
            ops.extend(crate::place::place_get(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops
        }
        AdtKind::Union => {
            let obj_getter = crate::place::place_adress(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            );
            let mut ops: Vec<CILOp> = Vec::with_capacity(fields.len() * 2);
            for field in fields {
                ops.extend(obj_getter.iter().cloned());
                ops.extend(field.1);
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let field_type = field_def.ty(tyctx, subst);
                let field_type = crate::utilis::monomorphize(&method_instance, field_type, tyctx);
                let field_type =
                    type_cache.type_from_cache(field_type, tyctx, Some(method_instance));
                let field_name = field_name(adt_type, field.0);
                //let field_name = crate::utilis::field_name(ty, idx)
                let field_desc =
                    FieldDescriptor::boxed(adt_type_ref.clone(), field_type, field_name);

                ops.push(CILOp::STField(field_desc));
            }
            ops.extend(crate::place::place_get(
                target_location,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops
        }
    }
}
