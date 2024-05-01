use crate::{
    cil::{CallSite, FieldDescriptor, StaticFieldDescriptor},
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_u64, conv_usize, ldc_u64,
    r#type::{DotnetTypeRef, TyCache, Type},
};
use rustc_abi::Size;
use rustc_middle::mir::{
    interpret::{AllocId, AllocRange, GlobalAlloc, Scalar},
    ConstOperand, ConstValue,
};
use rustc_middle::ty::{AdtKind, FloatTy, Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy};
pub fn handle_constant<'ctx>(
    constant_op: &ConstOperand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let constant = constant_op.const_;
    let constant = crate::utilis::monomorphize(&method_instance, constant, tyctx);
    let evaluated = constant
        .eval(tyctx, ParamEnv::reveal_all(), constant_op.span)
        .expect("Could not evaluate constant!");
    load_const_value(
        evaluated,
        constant.ty(),
        tyctx,
        method,
        method_instance,
        tycache,
    )
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the allocation
fn create_const_from_data<'ctx>(
    ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    alloc_id: AllocId,
    offset_bytes: u64,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let alloc = tyctx.global_alloc(alloc_id);
    // Constant should be memory:
    let memory = alloc.unwrap_memory();
    let len = memory.0.len();
    let range = AllocRange {
        start: Size::from_bytes(offset_bytes),
        size: Size::from_bytes((len as u64) - offset_bytes),
    };

    //TODO: fix layout issues!
    if memory.0.provenance().ptrs().is_empty() && true {
        let _bytes = memory.0.get_bytes_unchecked(range);
        //eprintln!("Creating const {ty:?} from data of length {len}.");
        //create_const_from_slice(ty, tyctx, bytes, method_instance, tycache)
    } else {

        //panic!("Constant requires rellocation support!");
    }
    let ptr = CILNode::LoadGlobalAllocPtr {
        alloc_id: alloc_id.0.into(),
    };
    let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
    crate::place::deref_op(ty.into(), tyctx, &method_instance, tycache, ptr)
}

pub(crate) fn load_const_value<'ctx>(
    const_val: ConstValue<'ctx>,
    const_ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    match const_val {
        ConstValue::Scalar(scalar) => {
            load_const_scalar(scalar, const_ty, tyctx, method, method_instance, tycache)
        }
        ConstValue::ZeroSized => {
            let tpe = crate::utilis::monomorphize(&method_instance, const_ty, tyctx);
            let tpe = tycache.type_from_cache(tpe, tyctx, Some(method_instance));
            CILNode::TemporaryLocal(Box::new((tpe, [].into(), CILNode::LoadTMPLocal)))
        }
        ConstValue::Slice { data, meta } => {
            let slice_type = tycache.type_from_cache(const_ty, tyctx, Some(method_instance));
            let slice_dotnet = slice_type.as_dotnet().expect("Slice type invalid!");
            let metadata_field =
                FieldDescriptor::new(slice_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field = FieldDescriptor::new(
                slice_dotnet,
                Type::Ptr(Type::Void.into()),
                "data_pointer".into(),
            );
            // TODO: find a better way to get an alloc_id. This is likely to be incoreect.
            let alloc_id = tyctx.reserve_and_set_memory_alloc(data);
            let alloc_id: u64 = crate::utilis::alloc_id_to_u64(alloc_id);

            CILNode::TemporaryLocal(Box::new((
                slice_type,
                [
                    CILRoot::SetField {
                        addr: CILNode::LoadAddresOfTMPLocal,
                        value: conv_usize!(ldc_u64!(meta)),
                        desc: metadata_field,
                    },
                    CILRoot::SetField {
                        addr: CILNode::LoadAddresOfTMPLocal,
                        value: CILNode::LoadGlobalAllocPtr { alloc_id },
                        desc: ptr_field,
                    },
                ]
                .into(),
                CILNode::LoadTMPLocal,
            )))
        }
        ConstValue::Indirect { alloc_id, offset } => {
            create_const_from_data(
                const_ty,
                tyctx,
                alloc_id,
                offset.bytes(),
                method_instance,
                tycache,
            )
            //todo!("Can't handle by-ref allocation {alloc_id:?} {offset:?}")
        } //_ => todo!("Unhandled const value {const_val:?} of type {const_ty:?}"),
    }
}
fn load_const_scalar<'ctx>(
    scalar: Scalar,
    scalar_type: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    _method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let scalar_type = crate::utilis::monomorphize(&method_instance, scalar_type, tyctx);
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(ptr, _size) => {
            let (alloc_id, offset) = ptr.into_parts();
            let global_alloc = tyctx.global_alloc(alloc_id.alloc_id());
            match global_alloc {
                GlobalAlloc::Static(def_id) => {
                    assert!(tyctx.is_static(def_id));

                    let name = tyctx
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
                                    StaticFieldDescriptor::new(None, Type::U8, name.clone().into())
                                        .into(),
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
                                    StaticFieldDescriptor::new(None, Type::U8, name.clone().into())
                                        .into(),
                                ),
                            }]
                            .into(),
                            CILNode::LoadAddresOfTMPLocal,
                        )));
                    }
                    let attrs = tyctx.codegen_fn_attrs(def_id);

                    if let Some(import_linkage) = attrs.import_linkage {
                        rustc_middle::ty::print::with_no_trimmed_paths! {
                            panic!("Static {def_id:?} requires special linkage {import_linkage:?} handling.")
                        };
                    }

                    // TODO: figure out why
                    // internal compiler error: compiler/rustc_const_eval/src/const_eval/machine.rs:395:21: trying to call extern function
                    // happens.
                    let alloc = tyctx
                        .eval_static_initializer(def_id)
                        .expect("No initializer??");
                    //def_id.ty();
                    let _tyctx = tyctx.reserve_and_set_memory_alloc(alloc);
                    let alloc_id = crate::utilis::alloc_id_to_u64(alloc_id.alloc_id());
                    return CILNode::LoadGlobalAllocPtr { alloc_id };
                }
                GlobalAlloc::Memory(_const_allocation) => {
                    return CILNode::Add(
                        CILNode::LoadGlobalAllocPtr {
                            alloc_id: alloc_id.alloc_id().0.into(),
                        }
                        .into(),
                        CILNode::ConvUSize(CILNode::LdcU64(offset.bytes()).into()).into(),
                    );
                }
                GlobalAlloc::Function(finstance) => {
                    // If it is a function, patch its pointer up.
                    let call_info =
                        crate::call_info::CallInfo::sig_from_instance_(finstance, tyctx, tycache)
                            .unwrap();
                    let function_name = crate::utilis::function_name(tyctx.symbol_name(finstance));
                    return CILNode::LDFtn(
                        CallSite::new(None, function_name, call_info.sig().clone(), true).into(),
                    );
                }
                _ => todo!("Unhandled global alloc {global_alloc:?}"),
            }
            //panic!("alloc_id:{alloc_id:?}")
        }
    };
    let tpe = crate::utilis::monomorphize(&method_instance, scalar_type, tyctx);
    let tpe = tycache.type_from_cache(tpe, tyctx, Some(method_instance));
    //TODO: This assumes a LE target
    match scalar_type.kind() {
        TyKind::Int(int_type) => load_const_int(scalar_u128, int_type),
        TyKind::Uint(uint_type) => load_const_uint(scalar_u128, uint_type),
        TyKind::Float(ftype) => load_const_float(scalar_u128, ftype, tyctx),
        TyKind::Bool => CILNode::LdcI32(scalar_u128 as i32),
        TyKind::RawPtr(_, _) => CILNode::ConvUSize(CILNode::LdcU64(scalar_u128 as u64).into()),
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
                CILNode::TemporaryLocal(Box::new((
                    Type::Ptr(tpe.clone().into()),
                    [].into(),
                    CILNode::LdObj {
                        ptr: CILNode::LoadTMPLocal.into(),
                        obj: Type::Void.into(),
                    },
                )))
            } else {
                CILNode::LdObj {
                    ptr: Box::new(CILNode::PointerToConstValue(scalar_u128)),
                    obj: tpe.into(),
                }
            }
        }
        TyKind::Adt(_, _subst) => CILNode::LdObj {
            ptr: Box::new(CILNode::PointerToConstValue(scalar_u128)),
            obj: tpe.into(),
        },
        TyKind::Char => CILNode::LdcU32(scalar_u128 as u32),
        _ => todo!("Can't load scalar constants of type {scalar_type:?}!"),
    }
}
fn load_const_float(value: u128, float_type: &FloatTy, _tyctx: TyCtxt) -> CILNode {
    match float_type {
        FloatTy::F16 => todo!("Can't hanlde 16 bit floats yet!"),
        FloatTy::F32 => {
            let value = f32::from_ne_bytes((value as u32).to_ne_bytes());
            CILNode::LdcF32(value)
        }
        FloatTy::F64 => {
            let value = f64::from_ne_bytes((value as u64).to_ne_bytes());
            CILNode::LdcF64(value)
        }
        FloatTy::F128 => todo!("Can't hanlde 128 bit floats yet!"),
    }
}
pub fn load_const_int(value: u128, int_type: &IntTy) -> CILNode {
    match int_type {
        IntTy::I8 => {
            let value = i8::from_ne_bytes([value as u8]);
            CILNode::ConvI8(CILNode::LdcI32(value as i32).into())
        }
        IntTy::I16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            CILNode::ConvI16(CILNode::LdcI32(value as i32).into())
        }
        IntTy::I32 => CILNode::LdcI32(i32::from_ne_bytes((value as u32).to_ne_bytes())),
        IntTy::I64 => CILNode::LdcI64(i64::from_ne_bytes((value as u64).to_ne_bytes())),
        IntTy::Isize => CILNode::ConvISize(
            CILNode::LdcI64(i64::from_ne_bytes((value as u64).to_ne_bytes())).into(),
        ),
        IntTy::I128 => {
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value >> 64) as u64;
            let ctor_sig = crate::function_sig::FnSig::new(
                &[
                    Type::ManagedReference(Type::U128.into()),
                    Type::U64,
                    Type::U64,
                ],
                &Type::Void,
            );
            CILNode::NewObj {
                site: CallSite::boxed(
                    Some(DotnetTypeRef::int_128()),
                    ".ctor".into(),
                    ctor_sig,
                    false,
                ),
                args: [conv_u64!(ldc_u64!(high)), conv_u64!(ldc_u64!(low))].into(),
            }
        }
    }
}
pub fn load_const_uint(value: u128, int_type: &UintTy) -> CILNode {
    match int_type {
        UintTy::U8 => {
            let value = value as u8;
            CILNode::ConvU8(CILNode::LdcU32(value as u32).into())
        }
        UintTy::U16 => {
            let value = value as u16;
            CILNode::ConvU16(CILNode::LdcU32(value as u32).into())
        }
        UintTy::U32 => CILNode::ConvU32(CILNode::LdcU32(value as u32).into()),
        UintTy::U64 => CILNode::ConvU64(CILNode::LdcU64(value as u64).into()),
        UintTy::Usize => CILNode::ConvUSize(CILNode::LdcU64(value as u64).into()),
        UintTy::U128 => {
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value >> 64) as u64;
            let ctor_sig = crate::function_sig::FnSig::new(
                &[
                    Type::ManagedReference(Type::U128.into()),
                    Type::U64,
                    Type::U64,
                ],
                &Type::Void,
            );
            CILNode::NewObj {
                site: CallSite::boxed(
                    Some(DotnetTypeRef::uint_128()),
                    ".ctor".into(),
                    ctor_sig,
                    false,
                ),
                args: [conv_u64!(ldc_u64!(high)), conv_u64!(ldc_u64!(low))].into(),
            }
        }
    }
}
