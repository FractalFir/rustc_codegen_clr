use crate::clr_method::LocalPlacement;
use crate::sign_cast;
use crate::statement::CodegenCtx;
use crate::{types::Type, BaseIR};
use rustc_middle::mir::{Place, PlaceElem};
enum Projection<'a, T> {
    OnlyHead(&'a T),
    BodyAndHead(&'a [T], &'a T),
}
fn adress_unless_copy(innertype: &Type) -> bool {
    if let Type::Struct { .. } = innertype {
        return true;
    } else if let Type::Slice(_) = innertype {
        return true;
    }
    false
}
/// This function handles any projection elements but the last one. This is needed because some projection elements need to have different behavior when being getting/setting their value.
/// E.g. Structs fields need to be copied, instead of their adress being taken.
fn projection_element<'ctx>(
    element: &PlaceElem<'ctx>,
    var_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> (Type, Vec<BaseIR>) {
    match element {
        PlaceElem::Deref => {
            /*
            if let Type::Ref(refd) = var_type {
                if adress_unless_copy(refd.as_ref()) {
                    return (refd.as_ref().clone(), vec![BaseIR::Nop]);
                }
            }
            let derefed_type = var_type
                .pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.deref_op();
            (derefed_type.clone(), vec![deref_op])
            */
            todo!("Can't dereference pointers yet!")
        }
        PlaceElem::Field(idx, ty) => {
            /*
            let field_type = Type::from_ty(ty, codegen_ctx.tyctx());
            let var_name = if let Type::Struct(struct_name) = var_type {
                struct_name
            } else {
                panic!("Tried to get a type of a non-struct type {ty:?}.");
            };
            //TODO: figure out how to get the field index PROPELY
            let idx =
                unsafe { std::mem::transmute::<rustc_target::abi::FieldIdx, u32>(*idx) } as usize;
            let getter = codegen_ctx
                .asm()
                .get_field_getter(idx, &var_name)
                .expect("Can't get field getter!");
            assert_eq!(getter.len(), 1);
            let mut getter = getter[0].clone();
            if let BaseIR::LDField {
                field_parent,
                field_name,
                field_type,
            } = getter
            {
                getter = BaseIR::LDFieldAdress {
                    field_parent,
                    field_name,
                    field_type,
                };
            }

            (field_type, vec![getter])
            */
            todo!("Can't get adresses of fields yet!")
        }
        PlaceElem::Index(index_operand) => {
            /*
            let arr_type = var_type;
            let element_type = arr_type
                .element_type()
                .expect("Tried to index into a type that was neither a slice nor an array!");
            if adress_unless_copy(&element_type) {
                let mut indexer = vec![match codegen_ctx
                    .meth()
                    .local_id_placement(index_operand.index() as u32)
                {
                    LocalPlacement::Arg(arg_num) => BaseIR::LDArg(arg_num),
                    LocalPlacement::Var(var) => BaseIR::LDLoc(var),
                }];
                indexer.push(BaseIR::Call {
                    sig: crate::FunctionSignature::new(
                        &[Type::ISize],
                        &Type::Ref(Box::new(element_type.clone())),
                    ),
                    function_name: format!(
                        "{arr_name}::item_Adress",
                        arr_name = arr_type.il_name()
                    )
                    .into(),
                });
                (element_type, indexer)
            } else {
                panic!("Can't indexing of primitve types is broken!")
            }
            */
            todo!("Can't index yet!")
        }
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => {
            /*
            assert!(!from_end, "Can't handle constant indexing from back!");
            let arr_type = var_type;
            let element_type = arr_type
                .element_type()
                .expect("Tried to index into a type that was neither a slice nor an array!");
            if adress_unless_copy(&element_type) {
                let offset = *offset;
                let mut indexer = vec![BaseIR::LDConstI64(sign_cast!(offset, u64, i64))];
                indexer.push(BaseIR::Call {
                    sig: crate::FunctionSignature::new(
                        &[Type::ISize],
                        &Type::Ref(Box::new(element_type.clone())),
                    ),
                    function_name: format!(
                        "{arr_name}::item_Adress",
                        arr_name = arr_type.il_name()
                    )
                    .into(),
                });
                (element_type, indexer)
            } else {
                panic!("Can't indexing of primitve types is broken!")
            }*/
            todo!("Can't do constant indexing yet!")
        }
    }
}
/// This function gets the value of the projected element and should be used on the last element in the projection chain.
/// It behaves slightly differently than `projection_element` because it, for example copies struct fields instead of just getting their address.
pub(crate) fn projection_element_get<'ctx>(
    element: &PlaceElem<'ctx>,
    var_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    match element {
        PlaceElem::Deref => {
            /*
            let derefed_type = var_type
                .pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            vec![derefed_type.deref_op()]
            */
            todo!("Can't dereference pointers yet!")
        }
        PlaceElem::Field(idx, ty) => {
            /*
            let _field_type = Type::from_ty(ty, codegen_ctx.tyctx());
            let var_name = if let Type::Struct(struct_name) = var_type {
                struct_name
            } else {
                panic!("Tried to get a type of a non-struct type {ty:?}.");
            };
            //TODO: figure out how to get the field index PROPELY
            let idx =
                unsafe { std::mem::transmute::<rustc_target::abi::FieldIdx, u32>(*idx) } as usize;
            let getter = codegen_ctx
                .asm()
                .get_field_getter(idx, &var_name)
                .expect("Can't get field getter!");
            assert_eq!(getter.len(), 1);
            getter
            */
            todo!("Can't get fields yet!")
        }
        PlaceElem::Index(index_operand) => {
            /*
            let arr_type = var_type;
            let element_type = arr_type
                .element_type()
                .expect("Tried to index into a type that was neither a slice nor an array!");
            let mut indexer = vec![match codegen_ctx
                .meth()
                .local_id_placement(index_operand.index() as u32)
            {
                LocalPlacement::Arg(arg_num) => BaseIR::LDArg(arg_num),
                LocalPlacement::Var(var) => BaseIR::LDLoc(var),
            }];
            indexer.push(BaseIR::Call {
                sig: crate::FunctionSignature::new(&[Type::ISize], &element_type),
                function_name: format!("{arr_name}::get_Item", arr_name = arr_type.il_name())
                    .into(),
            });
            indexer
            */
            todo!("Can't index yet!")
        }
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => todo!("can't handle constant indexing in get"),
    }
}
/// This function sest the value of the projected element and should be used on the last element in the projection chain.
pub(crate) fn projection_element_set<'ctx>(
    element: &PlaceElem<'ctx>,
    var_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    match element {
        PlaceElem::Deref => {
            /*
            let derefed_type = var_type
                .pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.set_pointed_op();
            vec![deref_op]
            */
            todo!("Can't set pointers yet!")
        }
        PlaceElem::Field(idx, ty) => {
            /*
            let _field_type = Type::from_ty(ty, codegen_ctx.tyctx());
            let var_name = if let Type::Struct{struct_name) = var_type {
                struct_name
            } else {
                panic!("Tried to get a type of a non-struct type {ty:?}.");
            };
            //TODO: figure out how to get the field index PROPELY
            let idx =
                unsafe { std::mem::transmute::<rustc_target::abi::FieldIdx, u32>(*idx) } as usize;
            let setter = codegen_ctx
                .asm()
                .get_field_setter(idx, &var_name)
                .expect("Can't get field getter!");
            //assert_eq!(setter.len(), 1);
            setter
            */
            todo!("Can't set fields yet")
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => todo!("can't handle constant indexing in set"),
    }
}
fn split_head<'a>(projection: &'a [PlaceElem<'a>]) -> Projection<'a, PlaceElem<'a>> {
    match projection.len() {
        0 => panic!("ERROR: Can't get the last element of a zero-sized projection chain!"),
        1 => Projection::OnlyHead(&projection[0]),
        _ => Projection::BodyAndHead(
            &projection[..(projection.len() - 1)],
            &projection[projection.len() - 1],
        ),
    }
}
pub(crate) fn projection_adress<'ctx>(
    place: &Place<'ctx>,
    local_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    let projection = place.projection;
    let mut ops = Vec::new();
    let mut var_type = local_type.clone();
    ops.push(
        match codegen_ctx.meth().local_id_placement(place.local.into()) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        },
    );
    for projection in projection {
        let (vtype, op) = projection_element(&projection, &var_type, codegen_ctx);
        ops.extend(op);
        var_type = vtype;
    }
    ops
}
pub(crate) fn project<
    'a,
    'ctx,
    F: Fn(&PlaceElem<'ctx>, &Type, &CodegenCtx<'ctx, '_>) -> Vec<BaseIR>,
    L: Fn(LocalPlacement) -> BaseIR,
>(
    place: &Place<'ctx>,
    local_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
    head_handler: F,
    local_handler: L,
) -> (Vec<BaseIR>, Vec<BaseIR>) {
    let projection = place.projection;
    let mut adress_calc = Vec::new();
    if projection.is_empty() {
        return (
            Vec::new(),
            vec![local_handler(
                codegen_ctx.meth().local_id_placement(place.local.into()),
            )],
        );
    } else {
        let ld_loc_op = match codegen_ctx.meth().local_id_placement(place.local.into()) {
            LocalPlacement::Arg(arg_num) => BaseIR::LDArg(arg_num),
            LocalPlacement::Var(var) => BaseIR::LDLoc(var),
        };
        adress_calc.push(ld_loc_op);
    }
    println!("adress_calc is {adress_calc:?}");
    assert!(
        !projection.is_empty(),
        "Can't generate ops for empty projection chain!"
    );
    match split_head(projection) {
        Projection::OnlyHead(head) => (Vec::new(), {
            adress_calc.extend(head_handler(head, local_type, codegen_ctx));
            adress_calc
        }),
        Projection::BodyAndHead(body, head) => {
            let mut last_type = local_type.clone();
            for projection in body {
                let (var_type, op) = projection_element(projection, &last_type, codegen_ctx);
                adress_calc.extend(op);
                last_type = var_type;
            }
            let last_op = head_handler(head, &last_type, codegen_ctx);
            (adress_calc, last_op)
        }
    }
}
pub(crate) fn projection_get<'a, 'ctx>(
    place: &Place<'ctx>,
    local_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    /*
    let (mut addr_calc, getter) = project(
        place,
        local_type,
        codegen_ctx,
        projection_element_get,
        |local| match local {
            LocalPlacement::Arg(arg_num) => BaseIR::LDArg(arg_num),
            LocalPlacement::Var(var) => BaseIR::LDLoc(var),
        },
    );
    println!("addr_calc:{addr_calc:?} getter:{getter:?}");
    addr_calc.extend(getter);
    addr_calc*/
    crate::codegen::place::place_getter_ops(place, codegen_ctx)
}
/// Used to handle the "body" of a projection chain. Use [`projection_element_get`] or [`projection_element_set`] to handle the head(last element)
pub(crate) fn projection_set<'a, 'ctx>(
    //projection: &'a [PlaceElem<'a>],
    place: &Place<'ctx>,
    local_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> (Vec<BaseIR>, Vec<BaseIR>) {
    println!("place:{place:?}");
    project(
        place,
        local_type,
        codegen_ctx,
        projection_element_set,
        |local| match local {
            LocalPlacement::Arg(arg_num) => BaseIR::STArg(arg_num),
            LocalPlacement::Var(var) => BaseIR::STLoc(var),
        },
    )
}
