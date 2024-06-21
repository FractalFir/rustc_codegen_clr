use crate::assembly::MethodCompileCtx;

use cilly::{
    call_site::CallSite,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    conv_u64, conv_usize,
    field_desc::FieldDescriptor,
    ldc_u64,
    static_field_desc::StaticFieldDescriptor,
    DotnetTypeRef, FnSig, Type,
};
use rustc_middle::{
    mir::{
        interpret::{AllocId, GlobalAlloc, Scalar},
        ConstOperand, ConstValue,
    },
    ty::{FloatTy, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy},
};
pub fn handle_constant<'tcx>(
    constant_op: &ConstOperand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let constant = constant_op.const_;
    let constant = ctx.monomorphize(constant);
    let evaluated = constant
        .eval(ctx.tcx(), ParamEnv::reveal_all(), constant_op.span)
        .expect("Could not evaluate constant!");
    load_const_value(evaluated, constant.ty(), ctx)
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the allocation
fn create_const_from_data<'tcx>(
    ty: Ty<'tcx>,
    alloc_id: AllocId,
    offset_bytes: u64,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let _ = offset_bytes;
    let ptr = CILNode::LoadGlobalAllocPtr {
        alloc_id: alloc_id.0.into(),
    };
    let ty = ctx.monomorphize(ty);
    let tpe = ctx.type_from_cache(ty);
    crate::place::deref_op(
        ty.into(),
        ctx,
        CILNode::TransmutePtr {
            val: Box::new(ptr),
            new_ptr: Box::new(Type::Ptr(Box::new(tpe))),
        },
    )
}

pub(crate) fn load_const_value<'tcx>(
    const_val: ConstValue<'tcx>,
    const_ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    match const_val {
        ConstValue::Scalar(scalar) => load_const_scalar(scalar, const_ty, ctx),
        ConstValue::ZeroSized => {
            let tpe = ctx.monomorphize(const_ty);
            assert!(
                crate::utilis::is_zst(tpe, ctx.tcx()),
                "Zero sized const with a non-zero size. It is {tpe:?}"
            );
            let tpe = ctx.type_from_cache(tpe);
            CILNode::TemporaryLocal(Box::new((tpe, [].into(), CILNode::LoadTMPLocal)))
        }
        ConstValue::Slice { data, meta } => {
            let slice_type = ctx.type_from_cache(const_ty);
            let slice_dotnet = slice_type.as_dotnet().expect("Slice type invalid!");
            let metadata_field =
                FieldDescriptor::new(slice_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field = FieldDescriptor::new(
                slice_dotnet,
                Type::Ptr(Type::Void.into()),
                "data_pointer".into(),
            );
            // TODO: find a better way to get an alloc_id. This is likely to be incoreect.
            let alloc_id = ctx.tcx().reserve_and_set_memory_alloc(data);
            let alloc_id: u64 = crate::utilis::alloc_id_to_u64(alloc_id);

            CILNode::TemporaryLocal(Box::new((
                slice_type,
                [
                    CILRoot::SetField {
                        addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                        value: Box::new(conv_usize!(ldc_u64!(meta))),
                        desc: Box::new(metadata_field),
                    },
                    CILRoot::SetField {
                        addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                        value: Box::new(CILNode::TransmutePtr {
                            val: Box::new(CILNode::LoadGlobalAllocPtr { alloc_id }),
                            new_ptr: Box::new(Type::Ptr(Type::Void.into())),
                        }),
                        desc: Box::new(ptr_field),
                    },
                ]
                .into(),
                CILNode::LoadTMPLocal,
            )))
        }
        ConstValue::Indirect { alloc_id, offset } => {
            create_const_from_data(const_ty, alloc_id, offset.bytes(), ctx)
            //todo!("Can't handle by-ref allocation {alloc_id:?} {offset:?}")
        } //_ => todo!("Unhandled const value {const_val:?} of type {const_ty:?}"),
    }
}
fn load_scalar_ptr(
    ctx: &mut MethodCompileCtx<'_, '_, '_>,
    ptr: rustc_middle::mir::interpret::Pointer,
) -> CILNode {
    let (alloc_id, offset) = ptr.into_parts();
    let global_alloc = ctx.tcx().global_alloc(alloc_id.alloc_id());
    match global_alloc {
        GlobalAlloc::Static(def_id) => {
            assert!(ctx.tcx().is_static(def_id));
            assert_eq!(offset.bytes(), 0);
            let name = ctx
                .tcx()
                .opt_item_name(def_id)
                .expect("Static without name")
                .to_string();
            /* */
            if name == "__rust_alloc_error_handler_should_panic"
                || name == "__rust_no_alloc_shim_is_unstable"
            {
                return CILNode::TemporaryLocal(Box::new((
                    Type::U8,
                    [CILRoot::SetTMPLocal {
                        value: CILNode::LDStaticField(
                            StaticFieldDescriptor::new(None, Type::U8, name.clone().into()).into(),
                        ),
                    }]
                    .into(),
                    CILNode::LoadAddresOfTMPLocal,
                )));
            }
            if name == "environ" {
                return CILNode::TemporaryLocal(Box::new((
                    Type::U8,
                    [CILRoot::SetTMPLocal {
                        value: CILNode::LDStaticField(
                            StaticFieldDescriptor::new(None, Type::U8, name.clone().into()).into(),
                        ),
                    }]
                    .into(),
                    CILNode::LoadAddresOfTMPLocal,
                )));
            }
            let attrs = ctx.tcx().codegen_fn_attrs(def_id);

            if let Some(import_linkage) = attrs.import_linkage {
                // TODO: this could cause issues if the pointer to the static is not imediatly dereferenced.
                if name == "statx" {
                    return CILNode::TemporaryLocal(Box::new((
                        Type::DelegatePtr(Box::new(FnSig::new(
                            [
                                Type::I32,
                                Type::Ptr(Box::new(Type::U8)),
                                Type::I32,
                                Type::U32,
                                Type::Ptr(Box::new(Type::Void)),
                            ],
                            Type::I32,
                        ))),
                        [CILRoot::SetTMPLocal {
                            value: CILNode::LDFtn(Box::new(CallSite::builtin(
                                "statx".into(),
                                FnSig::new(
                                    &[
                                        Type::I32,
                                        Type::Ptr(Type::U8.into()),
                                        Type::I32,
                                        Type::U32,
                                        Type::Ptr(Type::Void.into()),
                                    ],
                                    Type::I32,
                                ),
                                true,
                            ))),
                        }]
                        .into(),
                        CILNode::LoadAddresOfTMPLocal,
                    )));
                }
                if name == "getrandom" {
                    return CILNode::TemporaryLocal(Box::new((
                        Type::DelegatePtr(Box::new(FnSig::new(
                            &[Type::Ptr(Type::U8.into()), Type::USize, Type::U32],
                            Type::USize,
                        ))),
                        [CILRoot::SetTMPLocal {
                            value: CILNode::LDFtn(Box::new(CallSite::builtin(
                                "getrandom".into(),
                                FnSig::new(
                                    &[Type::Ptr(Type::U8.into()), Type::USize, Type::U32],
                                    Type::USize,
                                ),
                                true,
                            ))),
                        }]
                        .into(),
                        CILNode::LoadAddresOfTMPLocal,
                    )));
                }
                if name == "__dso_handle" {
                    return CILNode::TemporaryLocal(Box::new((
                        Type::DelegatePtr(Box::new(FnSig::new(&[], Type::Void))),
                        [CILRoot::SetTMPLocal {
                            value: CILNode::LDFtn(Box::new(CallSite::builtin(
                                "__dso_handle".into(),
                                FnSig::new(&[], Type::Void),
                                true,
                            ))),
                        }]
                        .into(),
                        CILNode::LoadAddresOfTMPLocal,
                    )));
                }
                if name == "__cxa_thread_atexit_impl" {
                    return CILNode::TemporaryLocal(Box::new((
                        Type::DelegatePtr(Box::new(FnSig::new(
                            &[
                                Type::DelegatePtr(Box::new(FnSig::new(
                                    [Type::Ptr(Box::new(Type::Void))],
                                    Type::Void,
                                ))),
                                Type::Ptr(Box::new(Type::Void)),
                                Type::Ptr(Box::new(Type::Void)),
                            ],
                            Type::Void,
                        ))),
                        [CILRoot::SetTMPLocal {
                            value: CILNode::LDFtn(Box::new(CallSite::builtin(
                                "__cxa_thread_atexit_impl".into(),
                                FnSig::new(
                                    &[
                                        Type::DelegatePtr(Box::new(FnSig::new(
                                            [Type::Ptr(Box::new(Type::Void))],
                                            Type::Void,
                                        ))),
                                        Type::Ptr(Box::new(Type::Void)),
                                        Type::Ptr(Box::new(Type::Void)),
                                    ],
                                    Type::Void,
                                ),
                                true,
                            ))),
                        }]
                        .into(),
                        CILNode::LoadAddresOfTMPLocal,
                    )));
                }
                rustc_middle::ty::print::with_no_trimmed_paths! {
                    panic!("Static {def_id:?} requires special linkage {import_linkage:?} handling. Its name is:{name:?}")
                };
            }

            let alloc = ctx
                .tcx()
                .eval_static_initializer(def_id)
                .expect("No initializer??");
            //def_id.ty();
            let _memory = ctx.tcx().reserve_and_set_memory_alloc(alloc);
            let alloc_id = crate::utilis::alloc_id_to_u64(alloc_id.alloc_id());
            CILNode::LoadGlobalAllocPtr { alloc_id }
        }
        GlobalAlloc::Memory(_const_allocation) => {
            if offset.bytes() != 0 {
                CILNode::Add(
                    CILNode::LoadGlobalAllocPtr {
                        alloc_id: alloc_id.alloc_id().0.into(),
                    }
                    .into(),
                    CILNode::ZeroExtendToUSize(CILNode::LdcU64(offset.bytes()).into()).into(),
                )
            } else {
                CILNode::LoadGlobalAllocPtr {
                    alloc_id: alloc_id.alloc_id().0.into(),
                }
            }
        }
        GlobalAlloc::Function(finstance) => {
            assert_eq!(offset.bytes(), 0);
            // If it is a function, patch its pointer up.
            let call_info = crate::call_info::CallInfo::sig_from_instance_(
                finstance,
                ctx.tcx(),
                ctx.type_cache(),
            );
            let function_name = crate::utilis::function_name(ctx.tcx().symbol_name(finstance));
            return CILNode::LDFtn(
                CallSite::new(None, function_name, call_info.sig().clone(), true).into(),
            );
        }
        GlobalAlloc::VTable(..) => todo!("Unhandled global alloc {global_alloc:?}"),
    }
    //panic!("alloc_id:{alloc_id:?}")
}
fn load_const_scalar<'tcx>(
    scalar: Scalar,
    scalar_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let scalar_ty = ctx.monomorphize(scalar_type);

    let scalar_type = ctx.type_from_cache(scalar_ty);
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int.to_uint(scalar.size()),
        Scalar::Ptr(ptr, _size) => {
            assert!(
                matches!(scalar_type, Type::Ptr(_) | Type::DelegatePtr(_)),
                "Invalid const ptr: {scalar_type:?}"
            );
            return CILNode::TransmutePtr {
                val: Box::new(load_scalar_ptr(ctx, ptr)),
                new_ptr: Box::new(scalar_type),
            };
        }
    };

    match scalar_ty.kind() {
        TyKind::Int(int_type) => load_const_int(scalar_u128, *int_type),
        TyKind::Uint(uint_type) => load_const_uint(scalar_u128, *uint_type),
        TyKind::Float(ftype) => load_const_float(scalar_u128, *ftype, ctx.tcx()),
        TyKind::Bool => {
            if scalar_u128 == 0 {
                CILNode::LdFalse
            } else {
                CILNode::LdTrue
            }
        }
        TyKind::RawPtr(_, _) => CILNode::TransmutePtr {
            val: Box::new(CILNode::ZeroExtendToUSize(
                CILNode::LdcU64(
                    u64::try_from(scalar_u128).expect("pointers must be smaller than 2^64"),
                )
                .into(),
            )),
            new_ptr: Box::new(scalar_type),
        },
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
                CILNode::TemporaryLocal(Box::new((
                    Type::Ptr(scalar_type.clone().into()),
                    [].into(),
                    CILNode::LdObj {
                        ptr: CILNode::LoadTMPLocal.into(),
                        obj: Type::Void.into(),
                    },
                )))
            } else {
                CILNode::LdObj {
                    ptr: Box::new(CILNode::TransmutePtr {
                        val: Box::new(CILNode::PointerToConstValue(Box::new(scalar_u128))),
                        new_ptr: Box::new(Type::Ptr(Box::new(scalar_type.clone()))),
                    }),
                    obj: scalar_type.into(),
                }
            }
        }
        TyKind::Adt(_, _subst) => CILNode::LdObj {
            ptr: Box::new(CILNode::TransmutePtr {
                val: Box::new(CILNode::PointerToConstValue(Box::new(scalar_u128))),
                new_ptr: Box::new(Type::Ptr(Box::new(scalar_type.clone()))),
            }),
            obj: scalar_type.into(),
        },
        TyKind::Char => CILNode::LdcU32(u32::try_from(scalar_u128).unwrap()),
        _ => todo!("Can't load scalar constants of type {scalar_ty:?}!"),
    }
}
fn load_const_float(value: u128, float_type: FloatTy, _tcx: TyCtxt) -> CILNode {
    match float_type {
        FloatTy::F16 => todo!("Can't hanlde 16 bit floats yet!"),
        FloatTy::F32 => {
            let value = f32::from_ne_bytes((u32::try_from(value).unwrap()).to_ne_bytes());
            CILNode::LdcF32(value)
        }
        FloatTy::F64 => {
            let value = f64::from_ne_bytes((u64::try_from(value).unwrap()).to_ne_bytes());
            CILNode::LdcF64(value)
        }
        FloatTy::F128 => todo!("Can't hanlde 128 bit floats yet!"),
    }
}
pub fn load_const_int(value: u128, int_type: IntTy) -> CILNode {
    match int_type {
        IntTy::I8 => {
            let value = i8::from_ne_bytes([u8::try_from(value).unwrap()]);
            CILNode::LdcI8(value)
        }
        IntTy::I16 => {
            let value = i16::from_ne_bytes((u16::try_from(value).unwrap()).to_ne_bytes());
            CILNode::LdcI16(value)
        }
        IntTy::I32 => CILNode::LdcI32(i32::from_ne_bytes(
            (u32::try_from(value).unwrap()).to_ne_bytes(),
        )),
        IntTy::I64 => CILNode::SignExtendToI64(
            CILNode::LdcI64(i64::from_ne_bytes(
                (u64::try_from(value).unwrap()).to_ne_bytes(),
            ))
            .into(),
        ),
        IntTy::Isize => CILNode::SignExtendToISize(
            CILNode::LdcI64(i64::from_ne_bytes(
                (u64::try_from(value).unwrap()).to_ne_bytes(),
            ))
            .into(),
        ),
        IntTy::I128 => {
            let low = u128_low_u64(value);
            let high = (value >> 64) as u64;
            let ctor_sig = FnSig::new(
                &[
                    Type::ManagedReference(Type::U128.into()),
                    Type::U64,
                    Type::U64,
                ],
                Type::Void,
            );
            CILNode::NewObj(Box::new(CallOpArgs {
                site: CallSite::boxed(
                    Some(DotnetTypeRef::int_128()),
                    ".ctor".into(),
                    ctor_sig,
                    false,
                ),
                args: [conv_u64!(ldc_u64!(high)), conv_u64!(ldc_u64!(low))].into(),
            }))
        }
    }
}
pub fn load_const_uint(value: u128, int_type: UintTy) -> CILNode {
    match int_type {
        UintTy::U8 => {
            let value = u8::try_from(value).unwrap();
            CILNode::ConvU8(CILNode::LdcU32(u32::from(value)).into())
        }
        UintTy::U16 => {
            let value = u16::try_from(value).unwrap();
            CILNode::ConvU16(CILNode::LdcU32(u32::from(value)).into())
        }
        UintTy::U32 => CILNode::ConvU32(CILNode::LdcU32(u32::try_from(value).unwrap()).into()),
        UintTy::U64 => {
            CILNode::ZeroExtendToU64(CILNode::LdcU64(u64::try_from(value).unwrap()).into())
        }
        UintTy::Usize => {
            CILNode::ZeroExtendToUSize(CILNode::LdcU64(u64::try_from(value).unwrap()).into())
        }
        UintTy::U128 => {
            let low = u128_low_u64(value);
            let high = (value >> 64) as u64;
            let ctor_sig = FnSig::new(
                &[
                    Type::ManagedReference(Type::U128.into()),
                    Type::U64,
                    Type::U64,
                ],
                Type::Void,
            );
            CILNode::NewObj(Box::new(CallOpArgs {
                site: CallSite::boxed(
                    Some(DotnetTypeRef::uint_128()),
                    ".ctor".into(),
                    ctor_sig,
                    false,
                ),
                args: [conv_u64!(ldc_u64!(high)), conv_u64!(ldc_u64!(low))].into(),
            }))
        }
    }
}
fn u128_low_u64(value: u128) -> u64 {
    u64::try_from(value & u128::from(u64::MAX)).expect("trucating cast error")
}
