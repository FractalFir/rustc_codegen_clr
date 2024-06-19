use crate::{assembly::MethodCompileCtx, r#type::Type};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, conv_usize, field_desc::FieldDescriptor,
    fn_sig::FnSig, ld_field,
};
use rustc_middle::{
    mir::{Place, PlaceElem},
    ty::TyKind,
};

pub(super) fn local_get(local: usize, method: &rustc_middle::mir::Body) -> CILNode {
    if let Some(spread_arg) = method.spread_arg
        && local == spread_arg.as_usize()
    {
        return CILNode::LDLoc(
            (method.local_decls.len() - method.arg_count)
                .try_into()
                .unwrap(),
        );
    }
    if local == 0 {
        CILNode::LDLoc(0)
    } else if local > method.arg_count {
        CILNode::LDLoc(
            u32::try_from(local - method.arg_count)
                .expect("Method has more than 2^32 local varaibles"),
        )
    } else {
        CILNode::LDArg(u32::try_from(local - 1).expect("Method has more than 2^32 local variables"))
    }
}
/// Returns the ops for getting the value of place.
pub fn place_get<'tyctx>(
    place: &Place<'tyctx>,
    ctx: &mut MethodCompileCtx<'tyctx, '_, '_>,
) -> CILNode {
    if place.projection.is_empty() {
        local_get(place.local.as_usize(), ctx.body())
    } else {
        let (mut op, mut ty) = super::local_body(place.local.as_usize(), ctx);
        ty = ctx.monomorphize(ty);
        let mut ty = ty.into();

        let (head, body) = super::slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) = super::place_elem_body(elem, ty, ctx, op);
            ty = curr_ty.monomorphize(ctx);
            op = curr_ops;
        }
        place_elem_get(head, ty, ctx, op)
    }
}

fn place_elem_get<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: super::PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_, '_>,
    addr_calc: CILNode,
) -> CILNode {
    match place_elem {
        PlaceElem::Deref => super::deref_op(super::pointed_type(curr_type).into(), ctx, addr_calc),
        PlaceElem::Field(field_index, _field_type) => match curr_type {
            super::PlaceTy::Ty(curr_type) => {
                let curr_type = ctx.monomorphize(curr_type);
                let _field_type = ctx.monomorphize(curr_type);

                let field_desc =
                    crate::utilis::field_descrptor(curr_type, (*field_index).into(), ctx);
                CILNode::LDField {
                    addr: addr_calc.into(),
                    field: field_desc.into(),
                }
            }
            super::PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = ctx.monomorphize(enm);
                let field_desc =
                    crate::utilis::enum_field_descriptor(owner, field_index.as_u32(), var_idx, ctx);
                CILNode::LDField {
                    addr: addr_calc.into(),
                    field: field_desc.into(),
                }
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");

            let index_type = ctx.monomorphize(ctx.body().local_decls[*index].ty);
            let index = crate::place::local_get(index.as_usize(), ctx.body());

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();

                    let index_type = ctx.type_from_cache(index_type);
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let size = crate::casts::int_to_int(
                        Type::I32,
                        &index_type,
                        CILNode::SizeOf(inner_type.clone().into()),
                    );
                    let addr = CILNode::Add(
                        Box::new(CILNode::TransmutePtr {
                            val: CILNode::LDField {
                                addr: addr_calc.into(),
                                field: desc.into(),
                            }
                            .into(),
                            new_ptr: Box::new(Type::Ptr(Box::new(inner_type))),
                        }),
                        CILNode::Mul(index.into(), size.into()).into(),
                    );
                    super::deref_op(super::PlaceTy::Ty(inner), ctx, addr)
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    call!(
                        CallSite::new(
                            Some(array_dotnet),
                            "get_Item".into(),
                            FnSig::new(&[Type::Ptr(array_type.into()), Type::USize], element),
                            false,
                        ),
                        [addr_calc, CILNode::ZeroExtendToUSize(index.into())]
                    )
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => {
            let _ = min_length;
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = CILNode::LdcU64(*offset);
            assert!(!from_end, "Indexing slice form end");
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();
                    let data_pointer = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let metadata = FieldDescriptor::new(slice, Type::USize, "metadata".into());

                    let addr = CILNode::TransmutePtr {
                        val: Box::new(ld_field!(addr_calc.clone(), data_pointer)),
                        new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                    } + call!(
                        CallSite::new(
                            None,
                            "bounds_check".into(),
                            FnSig::new(&[Type::USize, Type::USize], Type::USize),
                            true
                        ),
                        [conv_usize!(index), ld_field!(addr_calc, metadata),]
                    ) * CILNode::ZeroExtendToUSize(
                        CILNode::SizeOf(inner_type.into()).into(),
                    );
                    super::deref_op(super::PlaceTy::Ty(inner), ctx, addr)
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    //eprintln!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
                    call!(
                        CallSite::new(
                            Some(array_dotnet),
                            "get_Item".into(),
                            FnSig::new(&[Type::Ptr(array_type.into()), Type::USize], element),
                            false,
                        ),
                        [addr_calc, CILNode::ZeroExtendToUSize(index.into())]
                    )
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::Subtype(_tpe) => addr_calc,
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
