use crate::cil_op::{CILOp, FieldDescriptor};
use crate::r#type::Type;
use crate::utilis::field_name;
use rustc_index::IndexVec;
use rustc_middle::mir::{AggregateKind, Operand, Place};
use rustc_middle::ty::{AdtDef, AdtKind, GenericArg, Instance, List, ParamEnv, Ty, TyCtxt, TyKind};
use rustc_target::abi::FieldIdx;
pub fn handle_aggregate<'tcx>(
    tcx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    aggregate_kind: &AggregateKind<'tcx>,
    field_index: &IndexVec<FieldIdx, Operand<'tcx>>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let fields: Vec<_> = field_index
        .iter()
        .enumerate()
        .map(|operand| {
            (
                operand.0 as u32,
                crate::operand::handle_operand(operand.1, tcx, method, method_instance),
            )
        })
        .collect();
    match aggregate_kind {
        AggregateKind::Adt(adt_def, variant_idx, subst, _utai, active_field) => {
            let penv = ParamEnv::empty();
            let adt_type = Instance::resolve(tcx, penv, *adt_def, subst)
                .expect("Could not resolve instance")
                .expect("Could not resolve instance")
                .ty(tcx, penv);
            let (adt_def, subst) = if let TyKind::Adt(def_id, subst) = adt_type.kind() {
                (def_id, subst)
            } else {
                panic!("Type {adt_type:?} is not a Algebraic Data Type!");
            };
            aggregate_from_adt(
                tcx,
                target_location,
                method,
                adt_def,
                adt_type,
                subst,
                variant_idx.as_u32(),
                fields,
                method_instance,
                active_field,
            )
        }
        _ => todo!("Unsuported aggregate kind {aggregate_kind:?}"),
    }
}
fn aggregate_from_adt<'tcx>(
    tcx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    adt: &AdtDef<'tcx>,
    adt_type: Ty<'tcx>,
    subst: &'tcx List<GenericArg<'tcx>>,
    variant_idx: u32,
    fields: Vec<(u32, Vec<CILOp>)>,
    method_instance: Instance<'tcx>,
    active_field: &Option<FieldIdx>,
) -> Vec<CILOp> {
    let adt_type = crate::utilis::monomorphize(&method_instance, adt_type, tcx);
    let adt_type_ref = Type::from_ty(adt_type, tcx);
    let adt_type_ref = if let Type::DotnetType(type_ref) = adt_type_ref {
        type_ref.as_ref().clone()
    } else {
        panic!("Can't get fields of type {adt_type:?}");
    };
    match adt.adt_kind() {
        AdtKind::Struct => {
            let obj_getter =
                crate::place::place_adress(target_location, tcx, method, method_instance);
            let mut ops: Vec<CILOp> = Vec::with_capacity(fields.len() * 2);
            for field in fields {
                ops.extend(obj_getter.iter().cloned());
                ops.extend(field.1);
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let _field_type = field_def.ty(tcx, subst);

                let field_type = crate::utilis::generic_field_ty(adt_type, field.0, tcx);
                let field_name = field_name(adt_type, field.0);
                let field_desc = crate::cil_op::FieldDescriptor::boxed(
                    adt_type_ref.clone(),
                    crate::r#type::Type::from_ty(field_type, tcx),
                    field_name,
                );
                ops.push(CILOp::STField(field_desc));
            }
            ops.extend(crate::place::place_get(
                target_location,
                tcx,
                method,
                method_instance,
            ));
            ops
        }
        AdtKind::Enum => {
            let adt_adress_ops =
                crate::place::place_adress(target_location, tcx, method, method_instance);

            let mut variant_type = adt_type_ref.clone(); //adt_type.variant_type(variant).expect("Can't get variant index");
            let variant_name = crate::utilis::variant_name(adt_type, variant_idx);
            variant_type.append_path(&format!("/{variant_name}"));
            let mut variant_identity = variant_type.clone();

            variant_identity.set_generics_identity();
            // Get variant adress
            let variant_field_desc = FieldDescriptor::new(
                adt_type_ref.clone(),
                Type::DotnetType(Box::new(variant_identity)),
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
            for (field_idx, (field, field_value)) in
                enum_variant.fields.iter().zip(fields.iter()).enumerate()
            {
                ops.extend(variant_address.clone());
                ops.extend(field_value.1.clone());
                let field_name = field.name.to_string();
                let field_name = crate::type_def::escape_field_name(&field_name);
                let field = Type::from_ty(
                    crate::utilis::generic_field_ty(adt_type, field_idx as u32, tcx),
                    tcx,
                );
                ops.push(CILOp::STField(Box::new(FieldDescriptor::new(
                    variant_type.clone(),
                    field,
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
                tcx,
                method,
                method_instance,
            ));
            ops
        }
        AdtKind::Union => {
            let obj_getter =
                crate::place::place_adress(target_location, tcx, method, method_instance);
            let mut ops: Vec<CILOp> = Vec::with_capacity(fields.len() * 2);
            for field in fields {
                ops.extend(obj_getter.iter().cloned());
                ops.extend(field.1);
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let _field_type = field_def.ty(tcx, subst);

                let field_type = crate::utilis::generic_field_ty(adt_type, field.0, tcx);
                let field_name = field_name(adt_type, field.0);
                let field_desc = crate::cil_op::FieldDescriptor::boxed(
                    adt_type_ref.clone(),
                    crate::r#type::Type::from_ty(field_type, tcx),
                    field_name,
                );
                ops.push(CILOp::STField(field_desc));
            }
            ops.extend(crate::place::place_get(
                target_location,
                tcx,
                method,
                method_instance,
            ));
            ops
        }
    }
}
