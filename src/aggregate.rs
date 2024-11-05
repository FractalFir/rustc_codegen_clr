use crate::{
    assembly::MethodCompileCtx,
    operand::handle_operand,
    place::{place_get, place_set},
    r#type::{get_type, pointer_to_is_fat},
    utilis::{adt::set_discr, field_name},
};
use cilly::{
    cil_node::CILNode,
    cil_root::CILRoot,
    v2::{cilnode::MethodKind, ClassRef, FieldDesc, FnSig, Int, MethodRef},
    Const, Type,
};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{AggregateKind, Operand, Place},
    ty::{AdtDef, AdtKind, GenericArg, Instance, List, ParamEnv, Ty, TyKind},
};
use rustc_target::abi::FieldIdx;
/// Returns the CIL ops to create the aggreagate value specifed by `aggregate_kind` at `target_location`. Uses indivlidual values specifed by `value_index`
pub fn handle_aggregate<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    target_location: &Place<'tcx>,
    aggregate_kind: &AggregateKind<'tcx>,
    value_index: &IndexVec<FieldIdx, Operand<'tcx>>,
) -> (Vec<CILRoot>, CILNode) {
    // Get CIL ops for each value
    let values: Vec<_> = value_index
        .iter()
        .enumerate()
        .map(|operand| {
            (
                u32::try_from(operand.0).unwrap(),
                crate::operand::handle_operand(operand.1, ctx),
            )
        })
        .collect();
    match aggregate_kind {
        AggregateKind::Adt(adt_def, variant_idx, subst, _utai, active_field) => {
            let penv = ParamEnv::reveal_all();
            let subst = ctx.monomorphize(*subst);
            //eprintln!("Preparing to resolve {adt_def:?} {subst:?}");
            let adt_type = Instance::try_resolve(ctx.tcx(), penv, *adt_def, subst);

            let adt_type = adt_type
                .expect("Could not resolve instance")
                .expect("Could not resolve instance")
                .ty(ctx.tcx(), penv);
            let adt_type = ctx.monomorphize(adt_type);
            let TyKind::Adt(adt_def, subst) = adt_type.kind() else {
                panic!("Type {adt_type:?} is not a Algebraic Data Type!");
            };
            aggregate_adt(
                ctx,
                target_location,
                *adt_def,
                adt_type,
                subst,
                variant_idx.as_u32(),
                values,
                *active_field,
            )
        }
        AggregateKind::Array(element) => {
            // Check if this array is made up from uninit values
            if crate::operand::is_uninit(&value_index[FieldIdx::from_usize(0)], ctx) {
                // This array is created from uninitalized data, so it itsefl is uninitialzed, so we can skip initializing it.
                return (vec![], super::place::place_get(target_location, ctx));
            }
            let element = ctx.monomorphize(*element);
            let element = ctx.type_from_cache(element);
            let array_type = ClassRef::fixed_array(element, value_index.len(), ctx);
            let array_getter = super::place::place_adress(target_location, ctx);
            let sig = FnSig::new(
                Box::new([ctx.nref(array_type), Type::Int(Int::USize), element]),
                Type::Void,
            );
            let site = MethodRef::new(
                array_type,
                ctx.alloc_string("set_Item"),
                ctx.alloc_sig(sig),
                MethodKind::Instance,
                vec![].into(),
            );
            let mut sub_trees = Vec::new();
            for value in values {
                sub_trees.push(CILRoot::Call {
                    site: ctx.alloc_methodref(site.clone()),
                    args: [
                        array_getter.clone(),
                        CILNode::V2(ctx.alloc_node(Const::USize(u64::from(value.0)))),
                        value.1,
                    ]
                    .into(),
                });
            }
            (sub_trees, (super::place::place_get(target_location, ctx)))
        }
        AggregateKind::Tuple => {
            let tuple_getter = super::place::place_adress(target_location, ctx);
            let types: Vec<_> = value_index
                .iter()
                .map(|operand| {
                    let operand_ty = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
                    get_type(operand_ty, ctx)
                })
                .collect();
            let dotnet_tpe = crate::r#type::simple_tuple(&types, ctx);
            let mut sub_trees = Vec::new();
            for field in &values {
                // Assigining to a Void field is a NOP and must be skipped(since it can have wierd side-effects).
                if types[field.0 as usize] == cilly::v2::Type::Void {
                    continue;
                }
                let name = format!("Item{}", field.0 + 1);

                let field_name = ctx.alloc_string(name);
                sub_trees.push(CILRoot::SetField {
                    addr: Box::new(tuple_getter.clone()),
                    value: Box::new(field.1.clone()),
                    desc: ctx.alloc_field(FieldDesc::new(
                        dotnet_tpe,
                        field_name,
                        types[field.0 as usize],
                    )),
                });
            }
            (sub_trees, (super::place::place_get(target_location, ctx)))
        }
        AggregateKind::Closure(_def_id, _args) => {
            let closure_ty = ctx
                .monomorphize(target_location.ty(ctx.body(), ctx.tcx()))
                .ty;
            let closure_type = get_type(closure_ty, ctx);
            let closure_dotnet = closure_type.as_class_ref().expect("Invalid closure type!");
            let closure_getter = super::place::place_adress(target_location, ctx);
            let mut sub_trees = vec![];
            for (index, value) in value_index.iter_enumerated() {
                let field_ty = ctx.monomorphize(value.ty(ctx.body(), ctx.tcx()));
                let field_type = get_type(field_ty, ctx);
                if field_type == cilly::v2::Type::Void {
                    continue;
                }
                let field_name = ctx.alloc_string(format!("f_{}", index.as_u32()));
                sub_trees.push(CILRoot::SetField {
                    addr: Box::new(closure_getter.clone()),
                    value: Box::new(handle_operand(value, ctx)),
                    desc: ctx.alloc_field(FieldDesc::new(closure_dotnet, field_name, field_type)),
                });
            }

            (sub_trees, (place_get(target_location, ctx)))
        }
        AggregateKind::Coroutine(_def_id, _args) => {
            let coroutine_ty = ctx
                .monomorphize(target_location.ty(ctx.body(), ctx.tcx()))
                .ty;
            let coroutine_type = get_type(coroutine_ty, ctx);
            let closure_dotnet = coroutine_type
                .as_class_ref()
                .expect("Invalid closure type!");
            let closure_getter = super::place::place_adress(target_location, ctx);
            let mut sub_trees = vec![];
            for (index, value) in value_index.iter_enumerated() {
                let field_ty = ctx.monomorphize(value.ty(ctx.body(), ctx.tcx()));
                let field_type = get_type(field_ty, ctx);
                if field_type == cilly::v2::Type::Void {
                    continue;
                }
                let field_name = ctx.alloc_string(format!("f_{}", index.as_u32()));
                sub_trees.push(CILRoot::SetField {
                    addr: Box::new(closure_getter.clone()),
                    value: Box::new(handle_operand(value, ctx)),
                    desc: ctx.alloc_field(FieldDesc::new(closure_dotnet, field_name, field_type)),
                });
            }
            let layout = ctx.layout_of(coroutine_ty);
            let (disrc_type, _) = crate::utilis::adt::enum_tag_info(layout.layout, ctx);
            if disrc_type != Type::Void {
                sub_trees.push(set_discr(
                    layout.layout,
                    rustc_target::abi::VariantIdx::from_u32(0), // TODO: this assumes all coroutines start with a tag of 0
                    closure_getter,
                    closure_dotnet,
                    layout.ty,
                    ctx,
                ));
            }
            (sub_trees, (place_get(target_location, ctx)))
        }
        AggregateKind::RawPtr(pointee, mutability) => {
            let pointee = ctx.monomorphize(*pointee);
            let [data, meta] = &*value_index.raw else {
                panic!("RawPtr fields: {value_index:?}");
            };
            let fat_ptr = Ty::new_ptr(ctx.tcx(), pointee, *mutability);
            // Get the addres of the initialized structure
            let init_addr = super::place::place_adress(target_location, ctx);
            let meta_ty = ctx.monomorphize(meta.ty(ctx.body(), ctx.tcx()));
            let data_ty = ctx.monomorphize(data.ty(ctx.body(), ctx.tcx()));
            let fat_ptr_type = ctx.type_from_cache(fat_ptr);
            if !pointer_to_is_fat(pointee, ctx.tcx(), ctx.instance()) {
                // Double-check the pointer is REALLY thin
                assert!(fat_ptr_type.as_class_ref().is_none());
                assert!(
                    !crate::utilis::is_zst(data_ty, ctx.tcx()),
                    "data_ty:{data_ty:?} is a zst. That is bizzare, cause it should be a pointer?"
                );
                let data_type = ctx.type_from_cache(data_ty);
                let ptr_tpe = ctx.type_from_cache(pointee);
                assert_ne!(data_type, Type::Void);
                // Pointer is thin, just directly assign
                return (
                    [place_set(
                        target_location,
                        handle_operand(data, ctx).cast_ptr(ctx.nptr(ptr_tpe)),
                        ctx,
                    )]
                    .into(),
                    (place_get(target_location, ctx)),
                );
            }
            assert!(pointer_to_is_fat(pointee,ctx.tcx(), ctx.instance()), "A pointer to {pointee:?} is not fat, but its metadata is {meta_ty:?}, and not a zst:{is_meta_zst}",is_meta_zst = crate::utilis::is_zst(meta_ty,  ctx.tcx()));
            let fat_ptr_type = get_type(fat_ptr, ctx);
            // Assign the components
            let data_ptr_name = ctx.alloc_string(crate::DATA_PTR);
            let void_ptr = ctx.nptr(cilly::v2::Type::Void);
            let assign_ptr = CILRoot::SetField {
                addr: Box::new(init_addr.clone()),
                value: Box::new(values[0].1.clone().cast_ptr(ctx.nptr(Type::Void))),
                desc: ctx.alloc_field(FieldDesc::new(
                    fat_ptr_type.as_class_ref().unwrap(),
                    data_ptr_name,
                    void_ptr,
                )),
            };
            let name = ctx.alloc_string(crate::METADATA);
            let assign_metadata = CILRoot::SetField {
                addr: Box::new(init_addr),
                value: Box::new(handle_operand(meta, ctx)),
                desc: ctx.alloc_field(FieldDesc::new(
                    fat_ptr_type.as_class_ref().unwrap(),
                    name,
                    cilly::v2::Type::Int(Int::USize),
                )),
            };

            (
                [assign_ptr, assign_metadata].into(),
                (place_get(target_location, ctx)),
            )
        }
        _ => todo!("Unsuported aggregate kind {aggregate_kind:?}"),
    }
}
/// Builds an Algebraic Data Type (struct,enum,union) at location `target_location`, with fields set using ops in `fields`.
fn aggregate_adt<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    target_location: &Place<'tcx>,
    adt: AdtDef<'tcx>,
    adt_type: Ty<'tcx>,
    subst: &'tcx List<GenericArg<'tcx>>,
    variant_idx: u32,
    fields: Vec<(u32, CILNode)>,
    active_field: Option<FieldIdx>,
) -> (Vec<CILRoot>, CILNode) {
    let adt_type = ctx.monomorphize(adt_type);
    let adt_type_ref = get_type(adt_type, ctx).as_class_ref().unwrap();
    match adt.adt_kind() {
        AdtKind::Struct => {
            let obj_getter = crate::place::place_adress(target_location, ctx);

            let mut sub_trees = Vec::new();
            for field in fields {
                let field_def = adt
                    .all_fields()
                    .nth(field.0 as usize)
                    .expect("Could not find field!");
                let field_type = field_def.ty(ctx.tcx(), subst);
                let field_type = ctx.monomorphize(field_type);
                let field_type = ctx.type_from_cache(field_type);
                // Seting a void field is a no-op.
                if field_type == Type::Void {
                    continue;
                }
                let field_desc = crate::utilis::field_descrptor(adt_type, field.0, ctx);

                sub_trees.push(CILRoot::SetField {
                    addr: Box::new(obj_getter.clone()),
                    value: Box::new(field.1),
                    desc: (field_desc),
                });
            }
            (sub_trees, (crate::place::place_get(target_location, ctx)))
        }
        AdtKind::Enum => {
            let adt_adress_ops = crate::place::place_adress(target_location, ctx);

            let variant_name = crate::utilis::variant_name(adt_type, variant_idx);

            let variant_address = adt_adress_ops.clone();
            let mut sub_trees = Vec::new();
            let enum_variant = adt
                .variants()
                .iter()
                .nth(variant_idx as usize)
                .expect("Can't get variant index");
            for (field, field_value) in enum_variant.fields.iter().zip(fields.iter()) {
                let field_name = ctx.alloc_string(format!(
                    "{variant_name}_{fname}",
                    fname = crate::r#type::escape_field_name(&field.name.to_string())
                ));
                let field_type = get_type(field.ty(ctx.tcx(), subst), ctx);
                // Seting a void field is a no-op.
                if field_type == cilly::v2::Type::Void {
                    continue;
                }

                sub_trees.push(CILRoot::SetField {
                    addr: Box::new(variant_address.clone()),
                    value: Box::new(field_value.1.clone()),
                    desc: ctx.alloc_field(FieldDesc::new(adt_type_ref, field_name, field_type)),
                });
            }

            let layout = ctx.layout_of(adt_type);
            let (disrc_type, _) = crate::utilis::adt::enum_tag_info(layout.layout, ctx);
            if disrc_type != Type::Void {
                sub_trees.push(set_discr(
                    layout.layout,
                    variant_idx.into(),
                    adt_adress_ops,
                    adt_type_ref,
                    layout.ty,
                    ctx,
                ));
            }

            (sub_trees, (crate::place::place_get(target_location, ctx)))
        }
        AdtKind::Union => {
            let obj_getter = crate::place::place_adress(target_location, ctx);

            let mut sub_trees = Vec::new();
            let active_field = active_field.unwrap();
            assert_eq!(fields.len(), 1);
            let field_def = adt
                .all_fields()
                .nth(active_field.as_u32() as usize)
                .expect("Could not find field!");

            let field_ty = ctx.monomorphize(field_def.ty(ctx.tcx(), subst));
            let field_type = get_type(field_ty, ctx);
            // Seting a void field is a no-op.
            if field_type == cilly::v2::Type::Void {
                return (vec![], crate::place::place_get(target_location, ctx));
            }

            let field_name = field_name(adt_type, active_field.as_u32());

            let desc = FieldDesc::new(adt_type_ref, ctx.alloc_string(field_name), field_type);
            sub_trees.push(CILRoot::SetField {
                addr: Box::new(obj_getter.clone()),
                value: Box::new(fields[0].1.clone()),
                desc: ctx.alloc_field(desc),
            });
            (sub_trees, (crate::place::place_get(target_location, ctx)))
        }
    }
}
