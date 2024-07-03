use crate::{
    assembly::MethodCompileCtx,
    call_info::CallInfo,
    operand::{handle_operand, operand_address},
    place::{place_address_raw, place_get},
    r#type::{pointer_to_is_fat, Type},
};
use cilly::{
    call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_usize,
    field_desc::FieldDescriptor, fn_sig::FnSig, ld_field, ldc_i32, ldc_u64, size_of,
};
use rustc_middle::{
    mir::{CastKind, NullOp, Operand, Place, Rvalue},
    ty::{adjustment::PointerCoercion, GenericArgs, Instance, InstanceKind, ParamEnv, Ty, TyKind},
};
macro_rules! cast {
    ($ctx:ident,$operand:ident,$target:ident,$cast_name:path) => {{
        let target = $ctx.monomorphize(*$target);
        let target = $ctx.type_from_cache(target);
        let src = $operand.ty(&$ctx.body().local_decls, $ctx.tcx());
        let src = $ctx.monomorphize(src);
        let src = $ctx.type_from_cache(src);
        $cast_name(src, &target, handle_operand($operand, $ctx))
    }};
}
pub fn handle_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    target_location: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    match rvalue {
        Rvalue::Use(operand) => handle_operand(operand, ctx),
        Rvalue::CopyForDeref(place) => crate::place::place_get(place, ctx),
        Rvalue::Ref(_region, _kind, place) => crate::place::place_adress(place, ctx),
        Rvalue::AddressOf(_mutability, place) => crate::place::place_adress(place, ctx),
        Rvalue::Cast(
            CastKind::PointerCoercion(PointerCoercion::UnsafeFnPointer),
            operand,
            _dst,
        ) => handle_operand(operand, ctx),
        Rvalue::Cast(
            CastKind::PointerCoercion(
                PointerCoercion::MutToConstPointer | PointerCoercion::ArrayToPointer,
            )
            | CastKind::PtrToPtr,
            operand,
            dst,
        ) => ptr_to_ptr(ctx, operand, *dst),
        Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize), operand, target) => {
            if *crate::config::NEW_UNSIZE {
                crate::unsize::unsize2(ctx, operand, *target)
            } else {
                crate::unsize::unsize(ctx, operand, *target)
            }
        }
        Rvalue::BinaryOp(binop, operands) => {
            crate::binop::binop(*binop, &operands.0, &operands.1, ctx)
        }
        Rvalue::UnaryOp(binop, operand) => crate::unop::unop(*binop, operand, ctx),
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            cast!(ctx, operand, target, crate::casts::int_to_int)
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            cast!(ctx, operand, target, crate::casts::float_to_int)
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            cast!(ctx, operand, target, crate::casts::int_to_float)
        }
        Rvalue::NullaryOp(op, ty) => match op {
            NullOp::SizeOf => {
                let ty = ctx.type_from_cache(ctx.monomorphize(*ty));
                conv_usize!(size_of!(ty))
            }
            NullOp::AlignOf => {
                conv_usize!(ldc_u64!(crate::utilis::align_of(
                    ctx.monomorphize(*ty),
                    ctx.tcx()
                )))
            }
            NullOp::OffsetOf(fields) => {
                assert_eq!(fields.len(), 1);
                todo!("Can't calc offset of yet!");
            }
            rustc_middle::mir::NullOp::UbChecks => {
                if ctx.tcx().sess.ub_checks() {
                    CILNode::LdTrue
                } else {
                    CILNode::LdFalse
                }
            }
        },
        Rvalue::Aggregate(aggregate_kind, field_index) => crate::aggregate::handle_aggregate(
            ctx,
            target_location,
            aggregate_kind.as_ref(),
            field_index,
        ),
        Rvalue::Cast(
            CastKind::PointerCoercion(PointerCoercion::ClosureFnPointer(_)),
            ref operand,
            _to_ty,
        ) => match ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx())).kind() {
            TyKind::Closure(def_id, args) => {
                let instance = Instance::resolve_closure(
                    ctx.tcx(),
                    *def_id,
                    args,
                    rustc_middle::ty::ClosureKind::FnOnce,
                )
                .polymorphize(ctx.tcx());
                let call_info = CallInfo::sig_from_instance_(instance, ctx.tcx(), ctx.type_cache());

                let function_name = crate::utilis::function_name(ctx.tcx().symbol_name(instance));
                let call_site = CallSite::new(None, function_name, call_info.sig().clone(), true);
                CILNode::LDFtn(Box::new(call_site))
            }
            _ => panic!(
                "{} cannot be cast to a fn ptr",
                operand.ty(ctx.body(), ctx.tcx())
            ),
        },
        Rvalue::Cast(CastKind::Transmute, operand, dst) => {
            let dst = ctx.monomorphize(*dst);
            let dst_ty = dst;
            let dst = ctx.type_from_cache(dst);
            let src = operand.ty(&ctx.body().local_decls, ctx.tcx());
            let src = ctx.monomorphize(src);
            let src = ctx.type_from_cache(src);
            match (&src, &dst) {
                (
                    Type::ISize | Type::USize | Type::Ptr(_),
                    Type::ISize | Type::USize | Type::Ptr(_),
                ) => CILNode::TransmutePtr {
                    val: Box::new(handle_operand(operand, ctx)),
                    new_ptr: Box::new(dst),
                },
                (Type::U16, Type::DotnetChar) => handle_operand(operand, ctx),
                (_, _) => CILNode::TemporaryLocal(Box::new((
                    src,
                    [CILRoot::SetTMPLocal {
                        value: handle_operand(operand, ctx),
                    }]
                    .into(),
                    crate::place::deref_op(
                        crate::place::PlaceTy::Ty(dst_ty),
                        ctx,
                        CILNode::TransmutePtr {
                            val: Box::new(CILNode::LoadAddresOfTMPLocal),
                            new_ptr: Box::new(Type::Ptr(Box::new(dst))),
                        },
                    ),
                ))),
            }
        }
        Rvalue::ShallowInitBox(operand, dst) => {
            let dst = ctx.monomorphize(*dst);
            let boxed_dst = Ty::new_box(ctx.tcx(), dst);
            //let dst = tycache.type_from_cache(dst, tcx, method_instance);
            let src = operand.ty(&ctx.body().local_decls, ctx.tcx());
            let boxed_dst_type = ctx.type_from_cache(boxed_dst);
            let src = ctx.monomorphize(src);
            assert!(
                !pointer_to_is_fat(dst, ctx.tcx(), ctx.instance()),
                "ERROR: shallow init box used to initialze a fat box!"
            );
            let src = ctx.type_from_cache(src);

            CILNode::TemporaryLocal(Box::new((
                src,
                [CILRoot::SetTMPLocal {
                    value: handle_operand(operand, ctx),
                }]
                .into(),
                crate::place::deref_op(
                    crate::place::PlaceTy::Ty(boxed_dst),
                    ctx,
                    CILNode::TransmutePtr {
                        val: Box::new(CILNode::LoadAddresOfTMPLocal),
                        new_ptr: Box::new(Type::Ptr(Box::new(boxed_dst_type))),
                    },
                ),
            )))
        }
        Rvalue::Cast(CastKind::PointerWithExposedProvenance, operand, target) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/ptr/fn.from_exposed_addr.html) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `usize as *const/mut T`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.
            let target = ctx.monomorphize(*target);
            let target = ctx.type_from_cache(target);
            // Cast from usize/isize to any *T is a NOP, so we just have to load the operand.
            CILNode::TransmutePtr {
                val: Box::new(handle_operand(operand, ctx)),
                new_ptr: Box::new(target),
            }
        }
        Rvalue::Cast(CastKind::PointerExposeProvenance, operand, target) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.expose_addrl) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `*const/mut T as usize`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.
            let target = ctx.monomorphize(*target);
            let target = ctx.type_from_cache(target);
            // Cast to usize/isize from any *T is a NOP, so we just have to load the operand.
            CILNode::TransmutePtr {
                val: Box::new(handle_operand(operand, ctx)),
                new_ptr: Box::new(target),
            }
        }
        Rvalue::Cast(CastKind::FloatToFloat, operand, target) => {
            let target = ctx.monomorphize(*target);
            let target = ctx.type_from_cache(target);
            let mut ops = handle_operand(operand, ctx);
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
            _target,
        ) => {
            let operand_ty = operand.ty(ctx.body(), ctx.tcx());
            operand
                .constant()
                .expect("function must be constant in order to take its adress!");
            let operand_ty = ctx.monomorphize(operand_ty);

            let (instance, _subst_ref) = if let TyKind::FnDef(def_id, subst_ref) = operand_ty.kind()
            {
                let subst = ctx.monomorphize(*subst_ref);
                let env = ParamEnv::reveal_all();
                let Some(instance) = Instance::resolve(ctx.tcx(), env, *def_id, subst)
                    .expect("Invalid function def")
                else {
                    panic!("ERROR: Could not get function instance. fn type:{operand_ty:?}")
                };
                (instance, subst_ref)
            } else {
                todo!("Trying to call a type which is not a function definition!");
            };
            let function_name = crate::utilis::function_name(ctx.tcx().symbol_name(instance));
            let function_sig =
                crate::function_sig::sig_from_instance_(instance, ctx.tcx(), ctx.type_cache())
                    .expect(
                        "Could not get function signature when trying to get a function pointer!",
                    );
            //FIXME: propely handle `#[track_caller]`
            let call_site = CallSite::new(None, function_name, function_sig, true);
            CILNode::LDFtn(call_site.into())
        }
        //Rvalue::Cast(kind, _operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        Rvalue::Discriminant(place) => {
            let addr = crate::place::place_adress(place, ctx);
            let owner_ty = ctx.monomorphize(place.ty(ctx.body(), ctx.tcx()).ty);
            let owner = ctx.type_from_cache(owner_ty);

            let layout = ctx.layout_of(owner_ty);
            let target = ctx.type_from_cache(owner_ty.discriminant_ty(ctx.tcx()));
            let (disrc_type, _) = crate::utilis::adt::enum_tag_info(layout.layout, ctx.tcx());
            let owner = if let crate::r#type::Type::DotnetType(dotnet_type) = owner {
                dotnet_type.as_ref().clone()
            } else {
                eprintln!("Can't get the discirminant of type {owner_ty:?}, because it is a zst. Size:{} Discr type:{:?}",layout.layout.size.bytes(), owner_ty.discriminant_ty(ctx.tcx()));
                return crate::casts::int_to_int(Type::I32, &target, ldc_i32!(0));
            };

            if disrc_type == Type::Void {
                // Just alwways return 0 if the discriminat type is `()` - this seems to work, and be what rustc expects. Wierd, but OK.
                crate::casts::int_to_int(Type::I32, &target, ldc_i32!(0))
            } else {
                crate::casts::int_to_int(
                    disrc_type.clone(),
                    &target,
                    crate::utilis::adt::get_discr(layout.layout, addr, owner, ctx.tcx(), owner_ty),
                )
            }
        }
        Rvalue::Len(operand) => {
            let ty = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
            match ty.ty.kind() {
                TyKind::Slice(inner) => {
                    let slice_tpe = ctx.slice_ty(*inner).as_dotnet().unwrap();
                    let descriptor =
                        FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                    let addr = crate::place::place_address_raw(operand, ctx);
                    assert!(
                        !matches!(addr, CILNode::LDLoc(_)),
                        "improper addr {addr:?}. operand:{operand:?}"
                    );
                    ld_field!(addr, descriptor)
                }
                TyKind::Array(_ty, length) => {
                    conv_usize!(ldc_u64!(crate::utilis::try_resolve_const_size(
                        ctx.monomorphize(*length)
                    )
                    .unwrap() as u64))
                }
                _ => todo!("Get length of type {ty:?}"),
            }
        }
        Rvalue::Repeat(operand, times) => repeat(rvalue, ctx, operand, *times),
        Rvalue::ThreadLocalRef(def_id) => {
            if !def_id.is_local() && ctx.tcx().needs_thread_local_shim(*def_id) {
                let _instance = Instance {
                    def: InstanceKind::ThreadLocalShim(*def_id),
                    args: GenericArgs::empty(),
                };
                // Call instance
                todo!("Thread locals with shims unsupported!")
            } else {
                let alloc_id = ctx.tcx().reserve_and_set_static_alloc(*def_id);
                let rvalue_ty = rvalue.ty(ctx.body(), ctx.tcx());
                let rvalue_type = ctx.type_from_cache(rvalue_ty);
                CILNode::LoadGlobalAllocPtr {
                    alloc_id: alloc_id.0.into(),
                }
                .cast_ptr(rvalue_type)
            }
        }
        Rvalue::Cast(rustc_middle::mir::CastKind::FnPtrToPtr, operand, _) => {
            handle_operand(operand, ctx)
        }
        Rvalue::Cast(rustc_middle::mir::CastKind::DynStar, _, _) => {
            todo!("Unusported cast kind:DynStar")
        }
        Rvalue::Cast(_, _, _) => todo!(),
    }
}
fn repeat<'tcx>(
    rvalue: &Rvalue<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    operand: &Operand<'tcx>,
    times: rustc_middle::ty::Const<'tcx>,
) -> CILNode {
    let times = ctx.monomorphize(times);
    let times = times
        .try_eval_target_usize(ctx.tcx(), ParamEnv::reveal_all())
        .expect("Could not evalute array size as usize.");
    let array = ctx.monomorphize(rvalue.ty(ctx.body(), ctx.tcx()));
    let array = ctx.type_from_cache(array);
    let array_dotnet = array.clone().as_dotnet().expect("Invalid array type.");

    let operand_type = ctx.type_from_cache(ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx())));
    let operand = handle_operand(operand, ctx);
    let mut branches = Vec::new();
    for idx in 0..times {
        branches.push(CILRoot::Call {
            site: Box::new(CallSite::new(
                Some(array_dotnet.clone()),
                "set_Item".into(),
                FnSig::new(
                    &[
                        Type::Ptr(array.clone().into()),
                        Type::USize,
                        operand_type.clone(),
                    ],
                    Type::Void,
                ),
                false,
            )),
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
fn ptr_to_ptr<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    operand: &Operand<'tcx>,
    dst: Ty<'tcx>,
) -> CILNode {
    let target = ctx.monomorphize(dst);
    let target_pointed_to = match target.kind() {
        TyKind::RawPtr(typ, _) => typ,
        TyKind::Ref(_, inner, _) => inner,
        _ => panic!("Type is not ptr {target:?}."),
    };
    let source = ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx()));
    let source_pointed_to = match source.kind() {
        TyKind::RawPtr(typ, _) => *typ,
        TyKind::Ref(_, inner, _) => *inner,
        _ => panic!("Type is not ptr {target:?}."),
    };
    let source_type = ctx.type_from_cache(source);
    let target_type = ctx.type_from_cache(target);

    let src_fat = pointer_to_is_fat(source_pointed_to, ctx.tcx(), ctx.instance());
    let target_fat = pointer_to_is_fat(*target_pointed_to, ctx.tcx(), ctx.instance());
    match (src_fat, target_fat) {
        (true, true) => {
            let parrent = handle_operand(operand, ctx);

            crate::place::deref_op(
                crate::place::PlaceTy::Ty(target),
                ctx,
                CILNode::TemporaryLocal(Box::new((
                    source_type,
                    [CILRoot::SetTMPLocal { value: parrent }].into(),
                    CILNode::TransmutePtr {
                        val: Box::new(CILNode::LoadAddresOfTMPLocal),
                        new_ptr: Box::new(Type::Ptr(Box::new(target_type))),
                    },
                ))),
            )
        }
        (true, false) => {
            if source_type.as_dotnet().is_none() {
                eprintln!("source:{source:?}");
            }
            CILNode::TemporaryLocal(Box::new((
                source_type.clone(),
                [CILRoot::SetTMPLocal {
                    value: handle_operand(operand, ctx),
                }]
                .into(),
                CILNode::TransmutePtr {
                    val: Box::new(ld_field!(
                        CILNode::LoadAddresOfTMPLocal,
                        FieldDescriptor::new(
                            source_type.as_dotnet().unwrap(),
                            Type::Ptr(Type::Void.into()),
                            "data_pointer".into(),
                        )
                    )),
                    new_ptr: Box::new(target_type),
                },
            )))
        }
        (false, true) => {
            panic!("ERROR: a non-unsizing cast turned a sized ptr into an unsized one")
        }
        _ => CILNode::TransmutePtr {
            val: Box::new(handle_operand(operand, ctx)),
            new_ptr: Box::new(target_type),
        },
    }
}
