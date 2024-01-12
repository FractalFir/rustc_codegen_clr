use super::PlaceTy;
use crate::{
    assert_morphic,
    cil::{CILOp, FieldDescriptor},
    function_sig::FnSig,
    r#type::{TyCache, Type},
};
use rustc_middle::{
    mir::PlaceElem,
    ty::{Instance, Ty, TyCtxt, TyKind},
};
pub fn local_adress(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::LDLocA(0)
    } else if local > method.arg_count {
        CILOp::LDLocA((local - method.arg_count) as u32)
    } else {
        CILOp::LDArgA((local - 1) as u32)
    }
}
pub fn address_last_dereference<'ctx>(
    target_type: Ty<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tycache: &mut TyCache,
    tyctx: TyCtxt<'ctx>,
    method: Instance<'ctx>,
) -> Vec<CILOp> {
    let curr_type = match curr_type {
        PlaceTy::Ty(curr_type) => curr_type,
        // Enums don't require any special handling
        PlaceTy::EnumVariant(_, _) => return vec![],
    };
    //eprintln!("target_type:{target_type:?} curr_type:{curr_type:?}");
    // Get the type curr_type points to!
    let curr_points_to = super::pointed_type(curr_type.into());
    let curr_type = tycache.type_from_cache(curr_type, tyctx, Some(method));

    match (curr_points_to.kind(), target_type.kind()) {
        (TyKind::Slice(_), TyKind::Slice(_)) => vec![],
        (TyKind::Slice(_), _) => vec![CILOp::LDField(
            FieldDescriptor::new(
                curr_type.as_dotnet().unwrap(),
                Type::Ptr(Type::Void.into()),
                "data_address".into(),
            )
            .into(),
        )],
        _ => vec![],
    }
    //println!("casting {source:?} source_pointed_to:{source_pointed_to:?} to {target:?} target_pointed_to:{target_pointed_to:?}. ops:{ops:?}");
}
pub fn place_elem_adress<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
    _body: &rustc_middle::mir::Body,
    type_cache: &mut crate::r#type::TyCache,
    place_ty: Ty<'ctx>,
) -> Vec<CILOp> {
    let curr_type = curr_type.monomorphize(&method_instance, tyctx);
    assert_morphic!(curr_type);
    match place_elem {
        PlaceElem::Deref => {
            address_last_dereference(place_ty, curr_type, type_cache, tyctx, method_instance)
        }
        PlaceElem::Field(index, field_ty) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                //TODO: Why was this commented out?
                //let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
                let curr_ty = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                if crate::r#type::pointer_to_is_fat(curr_ty, tyctx, Some(method_instance)) {
                    use rustc_middle::ty::TypeAndMut;
                    assert_eq!(
                        index.as_u32(),
                        0,
                        "Can't handle DST with more than 1 field."
                    );
                    let field_ty = crate::utilis::monomorphize(&method_instance, *field_ty, tyctx);
                    let curr_type = type_cache.type_from_cache(
                        Ty::new_ptr(
                            tyctx,
                            TypeAndMut {
                                ty: curr_ty,
                                mutbl: rustc_middle::ty::Mutability::Mut,
                            },
                        ),
                        tyctx,
                        Some(method_instance),
                    );
                    let field_type = type_cache.type_from_cache(
                        Ty::new_ptr(
                            tyctx,
                            TypeAndMut {
                                ty: field_ty,
                                mutbl: rustc_middle::ty::Mutability::Mut,
                            },
                        ),
                        tyctx,
                        Some(method_instance),
                    );
                    return vec![
                        CILOp::NewTMPLocal(curr_type.into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LdObj(field_type.clone().into()),
                        CILOp::FreeTMPLocal,
                    ];
                    //todo!("Handle DST fields. DST:")
                }
                let field_desc = crate::utilis::field_descrptor(
                    curr_ty,
                    (*index).into(),
                    tyctx,
                    method_instance,
                    type_cache,
                );
                vec![CILOp::LDFieldAdress(field_desc.into())]
            }
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let field_desc = crate::utilis::enum_field_descriptor(
                    owner,
                    index.as_u32(),
                    var_idx,
                    tyctx,
                    method_instance,
                    type_cache,
                );
                let ops = vec![CILOp::LDFieldAdress(field_desc.into())];
                ops
            }
        },
        PlaceElem::Downcast(symbol, _) => {
            let curr_type = curr_type
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let curr_dotnet_type =
                type_cache.type_from_cache(curr_type, tyctx, Some(method_instance));
            let curr_dotnet_type =
                if let crate::r#type::Type::DotnetType(dotnet_type) = curr_dotnet_type {
                    dotnet_type.as_ref().clone()
                } else {
                    panic!();
                };
            let variant_name = symbol.unwrap();
            let field_name = format!("v_{variant_name}").into();
            let _curr_type_name = (curr_dotnet_type).name_path();
            let mut field_type = curr_dotnet_type.clone();
            field_type.append_path(&format!("/{variant_name}"));
            let field_desc = FieldDescriptor::boxed(
                curr_dotnet_type.clone(),
                crate::r#type::Type::DotnetType(Box::new(field_type)),
                field_name,
            );
            vec![CILOp::LDFieldAdress(field_desc)]
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            );
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                    let inner_type =
                        type_cache.type_from_cache(inner, tyctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    );

                    let ops = vec![
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::Mul,
                        CILOp::Add,
                    ];
                    ops
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let element_type =
                        type_cache.type_from_cache(element, tyctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let ops = vec![
                        index,
                        CILOp::Call(
                            crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[array_type, Type::USize],
                                    &Type::Ptr(element_type.into()),
                                ),
                                false,
                            )
                            .into(),
                        ),
                    ];
                    ops
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::Subslice { from, to, from_end } => {
            let curr_type = crate::r#type::tycache::slice_ref_to(
                tyctx,
                type_cache,
                curr_type.as_ty().expect("Can't index into an enum!"),
                Some(method_instance),
            );
            let curr_dotnet = curr_type.as_dotnet().unwrap();
            if *from_end {
                vec![
                    CILOp::NewTMPLocal(Box::new(curr_type.clone())),
                    CILOp::SetTMPLocal,
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LDField(FieldDescriptor::boxed(
                        curr_dotnet.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    )),
                    CILOp::LdcI64(*from as i64),
                    CILOp::ConvUSize(false),
                    CILOp::Add,
                    CILOp::STField(FieldDescriptor::boxed(
                        curr_dotnet.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    )),
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LDField(FieldDescriptor::boxed(
                        curr_dotnet.clone(),
                        Type::USize,
                        "metadata".into(),
                    )),
                    CILOp::LdcI64(*to as i64),
                    CILOp::ConvUSize(false),
                    CILOp::Sub,
                    CILOp::STField(FieldDescriptor::boxed(
                        curr_dotnet,
                        Type::USize,
                        "metadata".into(),
                    )),
                    CILOp::LoadTMPLocal,
                    CILOp::FreeTMPLocal,
                ]
            } else {
                vec![
                    CILOp::NewTMPLocal(Box::new(curr_type.clone())),
                    CILOp::SetTMPLocal,
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LDField(FieldDescriptor::boxed(
                        curr_dotnet.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    )),
                    CILOp::LdcI64(*from as i64),
                    CILOp::ConvUSize(false),
                    CILOp::Add,
                    CILOp::STField(FieldDescriptor::boxed(
                        curr_dotnet.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    )),
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LdcI64((to - from) as i64),
                    CILOp::STField(FieldDescriptor::boxed(
                        curr_dotnet,
                        Type::USize,
                        "metadata".into(),
                    )),
                    CILOp::LoadTMPLocal,
                    CILOp::FreeTMPLocal,
                ]
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
            let index = CILOp::LdcI64(*offset as i64);
            //assert!(!from_end, "Indexing slice form end");
            eprintln!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);

                    let inner_type =
                        type_cache.type_from_cache(inner, tyctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    );
                    let len = FieldDescriptor::new(slice, Type::USize, "metadata".into());
                    let derf_op = super::deref_op(
                        super::PlaceTy::Ty(inner),
                        tyctx,
                        &method_instance,
                        type_cache,
                    );
                    if *from_end {
                        let mut ops = vec![
                            CILOp::Dup,
                            CILOp::LDField(len.into()),
                            CILOp::NewTMPLocal(Type::USize.into()),
                            CILOp::SetTMPLocal,
                            CILOp::LDField(desc.into()),
                            CILOp::LoadTMPLocal,
                            index,
                            CILOp::Sub,
                            CILOp::SizeOf(inner_type.into()),
                            CILOp::Mul,
                            CILOp::Add,
                        ];
                        // ops.extend(derf_op);
                        ops
                        //todo!("Can't index slice from end!");
                    } else {
                        let mut ops = vec![
                            CILOp::LDField(desc.into()),
                            index,
                            CILOp::SizeOf(inner_type.into()),
                            CILOp::Mul,
                            CILOp::Add,
                        ];

                        //ops.extend(derf_op);
                        ops
                    }
                }
                TyKind::Array(element, length) => {
                    let element_ty = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let length = crate::utilis::monomorphize(&method_instance, *length, tyctx);
                    let length = crate::utilis::try_resolve_const_size(length).unwrap();
                    let element =
                        type_cache.type_from_cache(element_ty, tyctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if *from_end {
                        vec![
                            CILOp::LdcI64(length as u64 as i64),
                            CILOp::ConvUSize(false),
                            index,
                            CILOp::Sub,
                            CILOp::Call(
                                crate::cil::CallSite::new(
                                    Some(array_dotnet),
                                    "get_Address".into(),
                                    FnSig::new(
                                        &[array_type, Type::USize],
                                        &Type::Ptr(element.into()),
                                    ),
                                    false,
                                )
                                .into(),
                            ),
                        ]
                        //todo!("Can't index array from end!");
                    } else {
                        vec![
                            index,
                            CILOp::Call(
                                crate::cil::CallSite::new(
                                    Some(array_dotnet),
                                    "get_Address".into(),
                                    FnSig::new(
                                        &[array_type, Type::USize],
                                        &Type::Ptr(element.into()),
                                    ),
                                    false,
                                )
                                .into(),
                            ),
                        ]
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
