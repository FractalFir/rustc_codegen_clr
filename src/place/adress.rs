use super::PlaceTy;
use crate::{
    assembly::MethodCompileCtx,
    assert_morphic,
    r#type::{pointer_to_is_fat, Type},
};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_usize,
    field_desc::FieldDescriptor, fn_sig::FnSig, ld_field, ldc_u64, size_of,
};
use rustc_middle::{
    mir::PlaceElem,
    ty::{Ty, TyKind},
};
pub fn local_adress(local: usize, method: &rustc_middle::mir::Body) -> CILNode {
    if let Some(spread_arg) = method.spread_arg
        && local == spread_arg.as_usize()
    {
        return CILNode::MRefToRawPtr(Box::new(CILNode::LDLocA(
            (method.local_decls.len() - method.arg_count)
                .try_into()
                .unwrap(),
        )));
    }
    if local == 0 {
        CILNode::MRefToRawPtr(CILNode::LDLocA(0).into())
    } else if local > method.arg_count {
        CILNode::MRefToRawPtr(
            CILNode::LDLocA(u32::try_from(local - method.arg_count).unwrap()).into(),
        )
    } else {
        CILNode::MRefToRawPtr(CILNode::LDArgA(u32::try_from(local - 1).unwrap()).into())
    }
}
pub fn address_last_dereference<'ctx>(
    target_ty: Ty<'ctx>,
    curr_type: PlaceTy<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
    addr_calc: CILNode,
) -> CILNode {
    let curr_type = match curr_type {
        PlaceTy::Ty(curr_type) => curr_type,
        // Enums don't require any special handling
        PlaceTy::EnumVariant(_, _) => return addr_calc,
    };
    //eprintln!("target_type:{target_type:?} curr_type:{curr_type:?}");
    // Get the type curr_type points to!
    let curr_points_to = super::pointed_type(curr_type.into());
    let curr_type = ctx.type_from_cache(curr_type);
    let target_type = ctx.type_from_cache(target_ty);
    match (
        pointer_to_is_fat(curr_points_to, ctx.tyctx(), ctx.method_instance()),
        pointer_to_is_fat(target_ty, ctx.tyctx(), ctx.method_instance()),
    ) {
        (true, false) => CILNode::LDIndPtr {
            ptr: Box::new(CILNode::LDField {
                field: FieldDescriptor::new(
                    curr_type.as_dotnet().unwrap(),
                    Type::Ptr(Type::Void.into()),
                    "data_pointer".into(),
                )
                .into(),
                addr: addr_calc.into(),
            }),
            loaded_ptr: Box::new(Type::Ptr(Box::new(target_type))),
        },
        (false, true) => panic!("Invalid last dereference in address!"),
        (false, false) | (true, true) => addr_calc,
    }
    /*match (curr_points_to.kind(), target_type.kind()) {
        (TyKind::Slice(_), TyKind::Slice(_)) => addr_calc,
        (TyKind::Slice(_), _) => CILNode::LDField {
            field: FieldDescriptor::new(
                curr_type.as_dotnet().unwrap(),
                Type::Ptr(Type::Void.into()),
                "data_pointer".into(),
            )
            .into(),
            addr: addr_calc.into(),
        },
        _ => addr_calc,
    }*/
    //println!("casting {source:?} source_pointed_to:{source_pointed_to:?} to {target:?} target_pointed_to:{target_pointed_to:?}. ops:{ops:?}");
}
pub fn place_elem_adress<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
    place_ty: Ty<'ctx>,
    addr_calc: CILNode,
) -> CILNode {
    let curr_type = curr_type.monomorphize(ctx);
    assert_morphic!(curr_type);
    match place_elem {
        PlaceElem::Deref => address_last_dereference(place_ty, curr_type, ctx, addr_calc),
        PlaceElem::Field(index, field_ty) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                //TODO: Why was this commented out?

                let curr_ty = ctx.monomorphize(curr_type);
                if crate::r#type::pointer_to_is_fat(curr_ty, ctx.tyctx(), ctx.method_instance()) {
                    assert_eq!(
                        index.as_u32(),
                        0,
                        "Can't handle DST with more than 1 field."
                    );
                    let field_ty = ctx.monomorphize(*field_ty);
                    let curr_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tyctx(),
                        curr_ty,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let field_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tyctx(),
                        field_ty,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    return CILNode::TemporaryLocal(Box::new((
                        curr_type,
                        [CILRoot::SetTMPLocal { value: addr_calc }].into(),
                        CILNode::LdObj {
                            ptr: Box::new(CILNode::TransmutePtr {
                                val: CILNode::LoadAddresOfTMPLocal.into(),
                                new_ptr: Box::new(Type::Ptr(Box::new(field_type.clone()))),
                            }),
                            obj: field_type.into(),
                        },
                    )));
                    //todo!("Handle DST fields. DST:")
                }
                let field_desc = crate::utilis::field_descrptor(curr_ty, (*index).into(), ctx);
                CILNode::LDFieldAdress {
                    addr: addr_calc.into(),
                    field: field_desc.into(),
                }
            }
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = ctx.monomorphize(enm);
                let field_desc =
                    crate::utilis::enum_field_descriptor(owner, index.as_u32(), var_idx, ctx);
                CILNode::LDFieldAdress {
                    addr: addr_calc.into(),
                    field: field_desc.into(),
                }
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(index.as_usize(), ctx.method());
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    // This is a false positive
                    #[allow(unused_parens)]
                    (CILNode::TransmutePtr {
                        val: Box::new(ld_field!(addr_calc.clone(), desc)),
                        new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                    } + conv_usize!(size_of!(inner_type)) * conv_usize!(index))
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element_type = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");

                    call!(
                        CallSite::new(
                            Some(array_dotnet),
                            "get_Address".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize],
                                Type::Ptr(element_type.into()),
                            ),
                            false,
                        ),
                        [addr_calc, index]
                    )
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::Subslice { from, to, from_end } => {
            let curr_type = ctx.slice_ref_to(curr_type.as_ty().expect("Can't index into an enum!"));
            let curr_dotnet = curr_type.as_dotnet().unwrap();
            if *from_end {
                let metadata_field =
                    FieldDescriptor::new(curr_dotnet.clone(), Type::USize, "metadata".into());
                let ptr_field = FieldDescriptor::new(
                    curr_dotnet.clone(),
                    Type::Ptr(Type::Void.into()),
                    "data_pointer".into(),
                );
                CILNode::TemporaryLocal(Box::new((
                    curr_type,
                    [
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                            value: Box::new(CILNode::Sub(
                                Box::new(ld_field!(addr_calc.clone(), metadata_field.clone())),
                                Box::new(conv_usize!(ldc_u64!(*to + 1))),
                            )),
                            desc: Box::new(metadata_field),
                        },
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                            value: Box::new(
                                ld_field!(addr_calc, ptr_field.clone())
                                    + conv_usize!(ldc_u64!(*from)),
                            ),
                            desc: Box::new(ptr_field.clone()),
                        },
                    ]
                    .into(),
                    CILNode::LoadTMPLocal,
                )))
            } else {
                let metadata_field =
                    FieldDescriptor::new(curr_dotnet.clone(), Type::USize, "metadata".into());
                let ptr_field = FieldDescriptor::new(
                    curr_dotnet.clone(),
                    Type::Ptr(Type::Void.into()),
                    "data_pointer".into(),
                );
                CILNode::TemporaryLocal(Box::new((
                    curr_type,
                    [
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                            value: Box::new(conv_usize!(ldc_u64!(to - from))),
                            desc: Box::new(metadata_field),
                        },
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                            value: Box::new(
                                ld_field!(addr_calc, ptr_field.clone())
                                    + conv_usize!(ldc_u64!(*from)),
                            ),
                            desc: Box::new(ptr_field.clone()),
                        },
                    ]
                    .into(),
                    CILNode::LoadTMPLocal,
                )))
            }
        }
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let _ = min_length;
            //assert!(!from_end, "Indexing slice form end");
            //println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);

                    let inner_type = ctx.type_from_cache(inner);
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();
                    let desc = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let len = FieldDescriptor::new(slice.clone(), Type::USize, "metadata".into());
                    let index = if *from_end {
                        //eprintln!("Slice index from end is:{offset}");
                        CILNode::Sub(
                            Box::new(ld_field!(addr_calc.clone(), len.clone())),
                            Box::new(conv_usize!(ldc_u64!(*offset))),
                        )
                    } else {
                        conv_usize!(ldc_u64!(*offset))
                        //ops.extend(derf_op);
                    };
                    CILNode::TransmutePtr {
                        val: Box::new(ld_field!(addr_calc.clone(), desc)),
                        new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                    } + (call!(
                        CallSite::new(
                            None,
                            "bounds_check".into(),
                            FnSig::new(&[Type::USize, Type::USize], Type::USize),
                            true
                        ),
                        [conv_usize!(index), ld_field!(addr_calc, len)]
                    ) * conv_usize!(CILNode::SizeOf(inner_type.into())))
                }
                TyKind::Array(element, _) => {
                    let element_ty = ctx.monomorphize(*element);

                    let element = ctx.type_from_cache(element_ty);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if *from_end {
                        todo!("Can't index array from end!");
                    } else {
                        call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    Type::Ptr(element.into()),
                                ),
                                false,
                            ),
                            [
                                addr_calc,
                                CILNode::ZeroExtendToUSize(ldc_u64!(*offset).into()),
                            ]
                        )
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => {
            rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't handle porojection {place_elem:?} in adress")}
        }
    }
}
