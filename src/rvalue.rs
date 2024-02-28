use crate::cil::{CILOp, CallSite, FieldDescriptor};
use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;
use crate::function_sig::FnSig;
use crate::operand::handle_operand;
use crate::place::deref_op;
use crate::{conv_usize, ld_field, ldc_u32, ldc_u64, size_of};

use crate::r#type::{pointer_to_is_fat, TyCache, Type};
use rustc_middle::{
    mir::{CastKind, NullOp, Place, Rvalue},
    ty::{adjustment::PointerCoercion, Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
pub fn handle_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    tyctx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
    tycache: &mut TyCache,
) -> CILNode {
    match rvalue {
        Rvalue::Use(operand) => handle_operand(operand, tyctx, method, method_instance, tycache),
        Rvalue::CopyForDeref(place) => {
            crate::place::place_get(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::Ref(_region, _kind, place) => {
            crate::place::place_adress(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::AddressOf(_mutability, place) => {
            crate::place::place_adress(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(
            CastKind::PointerCoercion(PointerCoercion::MutToConstPointer) | CastKind::PtrToPtr | CastKind::PointerCoercion(PointerCoercion::ArrayToPointer),
            operand,
            dst,
        ) => {
            let target = crate::utilis::monomorphize(&method_instance, *dst, tyctx);
            let target_pointed_to = match target.kind() {
                TyKind::RawPtr(type_and_mut) => type_and_mut.ty,
                TyKind::Ref(_, inner, _) => *inner,
                _ => panic!("Type is not ptr {target:?}."),
            };
            let source =
                crate::utilis::monomorphize(&method_instance, operand.ty(method, tyctx), tyctx);
            let source_pointed_to = match source.kind() {
                TyKind::RawPtr(type_and_mut) => type_and_mut.ty,
                TyKind::Ref(_, inner, _) => *inner,
                _ => panic!("Type is not ptr {target:?}."),
            };
            let source_type = tycache.type_from_cache(source, tyctx, Some(method_instance));
            //let target_type = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src_fat = pointer_to_is_fat(source_pointed_to, tyctx, Some(method_instance));
            let target_fat = pointer_to_is_fat(target_pointed_to, tyctx, Some(method_instance));
            match (src_fat, target_fat) {
                (true, true) => {
                    let mut res = Vec::new();
                    res.push(CILOp::NewTMPLocal(source_type.into()));
                    res.push(CILOp::SetTMPLocal);
                    res.push(CILOp::LoadAddresOfTMPLocal);
                    res.push(CILOp::FreeTMPLocal);
                    let parrent =
                        handle_operand(operand, tyctx, method, method_instance, tycache).into();
                    crate::place::deref_op(
                        crate::place::PlaceTy::Ty(target),
                        tyctx,
                        &method_instance,
                        tycache,
                        CILNode::RawOps {
                            parrent,
                            ops: res.into(),
                        },
                    )
                }
                (true, false) => {
                    if source_type.as_dotnet().is_none() {
                        eprintln!("source:{source:?}");
                    }
                    ld_field!(
                        handle_operand(operand, tyctx, method, method_instance, tycache),
                        FieldDescriptor::new(
                            source_type.as_dotnet().unwrap(),
                            Type::Ptr(Type::Void.into()),
                            "data_pointer".into(),
                        )
                    )
                }
                _ => handle_operand(operand, tyctx, method, method_instance, tycache),
            }
        }
        Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize), operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let source =
                crate::utilis::monomorphize(&method_instance, operand.ty(method, tyctx), tyctx);
            let source_type = tycache.type_from_cache(source, tyctx, Some(method_instance));
            let target_type = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let target_dotnet = target_type.as_dotnet().unwrap();
            let mut parrent = handle_operand(operand, tyctx, method, method_instance, tycache);
            let derefed_source = match source.kind() {
                TyKind::RawPtr(tpe) => tpe.ty,
                TyKind::Ref(_, inner, _) => *inner,
                TyKind::Adt(_, _) => {
                    if source.is_box() {
                        let inner = source.boxed_ty();
                        let field_descriptor = crate::utilis::field_descrptor(
                            source,
                            0,
                            tyctx,
                            method_instance,
                            tycache,
                        );
                        parrent = ld_field!(parrent, field_descriptor);
                        inner
                    } else {
                        panic!("Non ptr type:{source:?}")
                    }
                }
                _ => panic!("Non ptr type:{source:?}"),
            };
            let length = if let TyKind::Array(_, length) = derefed_source.kind() {
                crate::utilis::try_resolve_const_size(*length).unwrap()
            } else {
                panic!("Non array type. source:{source:?} target:{target:?}")
            };

            CILNode::RawOps {
                parrent: parrent.into(),
                ops: [
                    CILOp::NewTMPLocal(source_type.clone().into()),
                    CILOp::SetTMPLocal,
                    // a:*[T;n] = stack_top;
                    CILOp::NewTMPLocal(target_type.into()),
                    // b:Slice = unint();
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LoadUnderTMPLocal(1),
                    CILOp::STField(
                        FieldDescriptor::new(
                            target_dotnet.clone(),
                            Type::Ptr(Type::Void.into()),
                            "data_pointer".into(),
                        )
                        .into(),
                    ),
                    // b.data_pointer = (a as *mut RustVoid);
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LdcI64(length as u64 as i64),
                    CILOp::ConvUSize(false),
                    CILOp::STField(
                        FieldDescriptor::new(target_dotnet, Type::USize, "metadata".into()).into(),
                    ),
                    // b.metadata = (length_i64 as usize);
                    CILOp::LoadTMPLocal,
                    // stack_top = b;
                    CILOp::FreeTMPLocal,
                    CILOp::FreeTMPLocal,
                ]
                .into(),
            }
            //todo!("Array to slice {res:?}!")
            //
        }
        Rvalue::BinaryOp(binop, operands) => crate::binop::binop_unchecked(
            *binop,
            &operands.0,
            &operands.1,
            tyctx,
            method,
            method_instance,
            tycache,
        ),
        Rvalue::CheckedBinaryOp(binop, operands) => CILNode::RawOpsParrentless {
            ops: crate::checked_binop::binop_checked(
                *binop,
                &operands.0,
                &operands.1,
                tyctx,
                method,
                method_instance,
                tycache,
            )
            .into(),
        },
        Rvalue::UnaryOp(binop, operand) => {
            crate::unop::unop(*binop, operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            crate::casts::int_to_int(
                src,
                target,
                handle_operand(operand, tyctx, method, method_instance, tycache),
            )
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));

            crate::casts::float_to_int(
                src,
                target,
                handle_operand(operand, tyctx, method, method_instance, tycache),
            )
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            crate::casts::int_to_float(
                src,
                target,
                handle_operand(operand, tyctx, method, method_instance, tycache),
            )
        }
        Rvalue::NullaryOp(op, ty) => match op {
            NullOp::SizeOf => {
                let ty = crate::utilis::monomorphize(&method_instance, *ty, tyctx);
                let ty = tycache.type_from_cache(ty, tyctx, Some(method_instance));
                conv_usize!(size_of!(ty))
            }
            NullOp::AlignOf => {
                let ty = crate::utilis::monomorphize(&method_instance, *ty, tyctx);
                conv_usize!(ldc_u64!(crate::utilis::align_of(ty, tyctx) as u64))
            }
            NullOp::OffsetOf(fields) => {
                assert_eq!(fields.len(), 1);
                //let (variant, field) = fields[0];
                todo!("Can't calc offset of yet!");
            }
            // TODO: propely set this to 0 or 1 depending if debug assertions are enabled.
            NullOp::DebugAssertions => ldc_u32!(0), //todo!("Unsuported nullary {op:?}!"),
        },
        Rvalue::Aggregate(aggregate_kind, field_index) => crate::aggregate::handle_aggregate(
            tyctx,
            target_location,
            method,
            aggregate_kind.as_ref(),
            field_index,
            method_instance,
            tycache,
        ),
        Rvalue::Cast(CastKind::Transmute, operand, dst) => {
            let dst = crate::utilis::monomorphize(&method_instance, *dst, tyctx);
            let dst_ty = dst;
            let dst = tycache.type_from_cache(dst, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            match (&src, &dst) {
                (
                    Type::ISize | Type::USize | Type::Ptr(_),
                    Type::ISize | Type::USize | Type::Ptr(_),
                ) => handle_operand(operand, tyctx, method, method_instance, tycache),
                (Type::U16, Type::DotnetChar) => {
                    handle_operand(operand, tyctx, method, method_instance, tycache)
                }

                (_, Type::U64) => CILNode::RawOps {
                    parrent: handle_operand(operand, tyctx, method, method_instance, tycache)
                        .into(),
                    ops: [
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndI64,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                },
                (_, Type::U32) => CILNode::RawOps {
                    parrent: handle_operand(operand, tyctx, method, method_instance, tycache)
                        .into(),
                    ops: [
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndI32,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                },
                (_, Type::F32) => CILNode::RawOps {
                    parrent: handle_operand(operand, tyctx, method, method_instance, tycache)
                        .into(),
                    ops: [
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndF32,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                },
                (_, Type::F64) => CILNode::RawOps {
                    parrent: handle_operand(operand, tyctx, method, method_instance, tycache)
                        .into(),
                    ops: [
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndF64,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                },
                (_, _) => {
                    let mut res = Vec::new();
                    let operand = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.push(CILOp::NewTMPLocal(src.into()));
                    res.push(CILOp::SetTMPLocal);
                    res.push(CILOp::LoadAddresOfTMPLocal);
                    res.push(CILOp::FreeTMPLocal);
                    crate::place::deref_op(
                        crate::place::PlaceTy::Ty(dst_ty),
                        tyctx,
                        &method_instance,
                        tycache,
                        CILNode::RawOps {
                            parrent: operand.into(),
                            ops: res.into(),
                        },
                    )
                }
            }
        }
        Rvalue::ShallowInitBox(operand, dst) => {
            let dst = crate::utilis::monomorphize(&method_instance, *dst, tyctx);
            let boxed_dst = Ty::new_box(tyctx, dst);
            //let dst = tycache.type_from_cache(dst, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));

            let parrent = handle_operand(operand, tyctx, method, method_instance, tycache);
            deref_op(
                boxed_dst.into(),
                tyctx,
                &method_instance,
                tycache,
                CILNode::RawOps {
                    parrent: parrent.into(),
                    ops: [
                        CILOp::NewTMPLocal(crate::r#type::Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                },
            )
        }
        Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/ptr/fn.from_exposed_addr.html) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `usize as *const/mut T`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.

            // Cast from usize/isize to any *T is a NOP, so we just have to load the operand.
            handle_operand(operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::PointerExposeAddress, operand, _) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.expose_addrl) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `*const/mut T as usize`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.

            // Cast to usize/isize from any *T is a NOP, so we just have to load the operand.
            handle_operand(operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::FloatToFloat, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let mut ops = handle_operand(operand, tyctx, method, method_instance, tycache);
            match target {
                Type::F32 => ops = CILNode::ConvF32(ops.into()),
                Type::F64 => ops = CILNode::ConvF64(ops.into()),
                _ => panic!("Can't preform a FloatToFloat cast to type {target:?}"),
            }
            ops
        }
        Rvalue::Cast(
            CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer),
            operand,
            target,
        ) => {
            let operand_ty = operand.ty(method, tyctx);
            operand
                .constant()
                .expect("function must be constant in order to take its adress!");
            let operand_ty = crate::utilis::monomorphize(&method_instance, operand_ty, tyctx);
            let _target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let (instance, _subst_ref) = if let TyKind::FnDef(def_id, subst_ref) = operand_ty.kind()
            {
                let subst = crate::utilis::monomorphize(&method_instance, *subst_ref, tyctx);
                let env = ParamEnv::reveal_all();
                let Some(instance) =
                    Instance::resolve(tyctx, env, *def_id, subst).expect("Invalid function def")
                else {
                    panic!("ERROR: Could not get function instance. fn type:{operand_ty:?}")
                };

                (instance, subst_ref)
            } else {
                todo!("Trying to call a type which is not a function definition!");
            };
            let function_name = crate::utilis::function_name(tyctx.symbol_name(instance));
            let function_sig = FnSig::sig_from_instance_(instance, tyctx, tycache)
                .expect("Could not get function signature when trying to get a function pointer!");
            //FIXME: propely handle `#[track_caller]`
            let call_site = CallSite::new(None, function_name, function_sig, true);
            CILNode::LDFtn(call_site.into())
        }
        //Rvalue::Cast(kind, _operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        Rvalue::Discriminant(place) => {
            let addr = crate::place::place_adress(place, tyctx, method, method_instance, tycache);
            let owner_ty = place.ty(method, tyctx).ty;
            let owner_ty = crate::utilis::monomorphize(&method_instance, owner_ty, tyctx);
            let owner = tycache.type_from_cache(owner_ty, tyctx, Some(method_instance));
            //TODO: chose proper tag type based on variant count of `owner`
            //let discr_ty = owner_ty.discriminant_ty(tyctx);
            //let discr_type = tycache.type_from_cache(discr_ty, tyctx, Some(method_instance));
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

            let target = tycache.type_from_cache(
                owner_ty.discriminant_ty(tyctx),
                tyctx,
                Some(method_instance),
            );
            crate::casts::int_to_int(
                disrc_type.clone(),
                target,
                CILNode::LDField {
                    field: crate::cil::FieldDescriptor::new(owner, disrc_type, "_tag".into())
                        .into(),
                    addr: addr.into(),
                },
            )
        }
        Rvalue::Len(operand) => {
            let ty = operand.ty(method, tyctx);
            let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
            // let tpe = tycache.type_from_cache(ty.ty, tyctx, Some(method_instance));
            match ty.ty.kind() {
                TyKind::Slice(inner) => {
                    let slice_tpe = tycache
                        .slice_ty(*inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let descriptor =
                        FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());

                    ld_field!(
                        crate::place::place_adress(
                            operand,
                            tyctx,
                            method,
                            method_instance,
                            tycache
                        ),
                        descriptor
                    )
                }
                _ => todo!("Get length of type {ty:?}"),
            }
        }
        Rvalue::Repeat(operand, times) => {
            let times = crate::utilis::monomorphize(&method_instance, *times, tyctx);
            let times = times
                .try_eval_target_usize(tyctx, ParamEnv::reveal_all())
                .expect("Could not evalute array size as usize.");
            let array =
                crate::utilis::monomorphize(&method_instance, rvalue.ty(method, tyctx), tyctx);
            let array = tycache.type_from_cache(array, tyctx, Some(method_instance));
            let array_dotnet = array.clone().as_dotnet().expect("Invalid array type.");

            let operand_type = tycache.type_from_cache(
                crate::utilis::monomorphize(&method_instance, operand.ty(method, tyctx), tyctx),
                tyctx,
                Some(method_instance),
            );
            let operand = handle_operand(operand, tyctx, method, method_instance, tycache);
            let mut branches = Vec::new();
            for idx in 0..times {
                branches.push(CILRoot::Call {
                    site: CallSite::new(
                        Some(array_dotnet.clone()),
                        "set_Item".into(),
                        FnSig::new(
                            &[
                                Type::Ptr(array.clone().into()),
                                Type::USize,
                                operand_type.clone(),
                            ],
                            &Type::Void,
                        ),
                        false,
                    ),
                    args: [
                        CILNode::LoadAddresOfTMPLocal,
                        conv_usize!(ldc_u64!(idx)),
                        operand.clone(),
                    ]
                    .into(),
                });
            }
            let branches: Box<_> = branches.into();
            CILNode::TemporaryLocal(Box::new((array.clone(), branches, CILNode::LoadTMPLocal)))
        }
        _ => rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Unhandled RValue {rvalue:?}")},
    }
}
