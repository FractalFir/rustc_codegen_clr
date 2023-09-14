use super::place::Place;
use crate::{base_ir::BaseIR, base_ir::FiledDescriptor, statement::CodegenCtx, types::Type};
use rustc_index::IndexVec;
use rustc_middle::mir::{AggregateKind as AKind, Operand, Place as RustPlace};
use rustc_middle::ty::Instance;
use rustc_middle::ty::ParamEnv;
use rustc_target::abi::FieldIdx;
pub(crate) fn handle_agregate<'tyctx>(
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &RustPlace<'tyctx>,
    aggregate: &AKind<'tyctx>,
    fields: &IndexVec<FieldIdx, Operand<'tyctx>>,
) -> Vec<BaseIR> {
    let aggregate = AggregateKind::from_ag(aggregate, codegen_ctx);
    let target_location: Place = super::place::Place::from(target_location, codegen_ctx);
    let fields = fields
        .iter()
        .enumerate()
        .map(|operand| {
            (
                operand.0 as u32,
                crate::statement::handle_operand(operand.1, codegen_ctx),
            )
        })
        .collect();
    create_aggregate(target_location, aggregate, fields)
}
enum AggregateKind {
    Tuple,
    Array {
        element_type: Type,
    },
    Adt {
        adt_type: Type,
        variant: u32,
        active_field: Option<u32>,
    },
}
impl AggregateKind {
    fn from_ag<'tyctx>(ag: &AKind<'tyctx>, ctx: &CodegenCtx<'tyctx, '_>) -> Self {
        match ag {
            AKind::Tuple => Self::Tuple,
            AKind::Array(tpe) => Self::Array {
                element_type: Type::from_ty(tpe, ctx.tyctx()),
            },
            AKind::Generator(_, _, _) => todo!("Generators are not supported!"),
            AKind::Closure(_, _) => todo!("Closures are not supported!"),
            AKind::Adt(def_id, varaint, subst_ref, _utai, active_field) => {
                let penv = ParamEnv::empty();
                let adt_type =
                    Instance::resolve(*ctx.tyctx(), ParamEnv::empty(), *def_id, subst_ref)
                        .expect("Could not resolve instance")
                        .expect("Could not resolve instance");
                let adt_type = Type::from_ty(&adt_type.ty(*ctx.tyctx(), penv), ctx.tyctx());
                Self::Adt {
                    adt_type,
                    variant: varaint.as_u32(),
                    active_field: active_field.map(|v| v.as_u32()),
                }
            }
        }
    }
}
fn create_adt(adt_type:Type,fields: Vec<(u32, Vec<BaseIR>)>, target_location: Place,variant:u32)->Vec<BaseIR>{
    match &adt_type{
        Type::Struct { name, .. }=>{
            let obj_getter = super::place::place_adress_ops(&target_location);
            let mut ops: Vec<BaseIR> = Vec::with_capacity(fields.len() * 2);
            for field in fields {
                ops.extend(obj_getter.iter().cloned());
                ops.extend(field.1);
                ops.push(BaseIR::STField(Box::new(FiledDescriptor {
                    owner: adt_type.clone(),
                    variant: variant,
                    field_index: field.0,
                })));
            }
            ops.extend(super::place::place_get_ops(&target_location));
            ops
        },
        Type::Enum { name, variants }=>{
            let mut getter_ops =  super::place::place_adress_ops(&target_location);
            
            let variant_type = adt_type.variant_type(variant).expect("Can't get variant index");
            // Get variant adress
            getter_ops.push(BaseIR::LDFieldAdress(Box::new(FiledDescriptor { owner: adt_type.clone(), variant: variant, field_index: u32::MAX })));
            let mut ops = Vec::new();
            let enum_variant = adt_type.enum_variant(variant).expect("Can't get variant index");
            for (field,field_value) in enum_variant.fields.iter().zip(fields.iter()){
                ops.extend(getter_ops.clone());
                ops.extend(field_value.1.clone());
                ops.push(BaseIR::STField(Box::new(FiledDescriptor {
                    owner: variant_type.clone(),
                    variant: variant,
                    field_index: field_value.0,
                })));
            }
            ops.extend(super::place::place_get_ops(&target_location));
            ops
        }
        _=>todo!("Can't create ADT of type {adt_type:?}"),
    }
}
fn create_aggregate(
    target_location: Place,
    aggreagate: AggregateKind,
    fields: Vec<(u32, Vec<BaseIR>)>,
) -> Vec<BaseIR> {
    match aggreagate {
        AggregateKind::Array { element_type } => {
            let array_type = Type::Array {
                element: Box::new(element_type.clone()),
                length: fields.len() as u64,
            };
            let mut ops: Vec<BaseIR> = Vec::with_capacity(fields.len() * 2);
            let array_getter = super::place::place_adress_ops(&target_location);
            for field in fields {
                ops.extend(array_getter.iter().cloned());
                ops.extend(field.1);
                ops.extend(super::array::array_set(
                    array_type.clone(),
                    vec![BaseIR::LDConstI64(field.0 as u64 as i64)],
                ));
            }
            ops.extend(super::place::place_get_ops(&target_location));
            ops
        }
        AggregateKind::Adt {
            adt_type,
            variant,
            active_field,
        } => create_adt(adt_type, fields, target_location, variant),
        AggregateKind::Tuple => todo!("Can't construct tuple aggreagtes yet!"),
    }
}
