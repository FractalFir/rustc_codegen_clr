use crate::{
    cil::{CILOp, CallSite, FieldDescriptor, StaticFieldDescriptor}, cil_tree::{cil_node::CILNode, cil_root::CILRoot}, conv_usize, ldc_u64, r#type::{DotnetTypeRef, TyCache, Type}
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
        .eval(tyctx, ParamEnv::reveal_all(), Some(constant_op.span))
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

fn load_const_value<'ctx>(
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
            CILNode::TemporaryLocal(Box::new((tpe,[].into(),CILNode::LoadTMPLocal)))
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

            CILNode::TemporaryLocal(Box::new((slice_type.into(),[
                CILRoot::SetField { addr: CILNode::LoadAddresOfTMPLocal, value: conv_usize!(ldc_u64!(meta as u64)), desc: metadata_field.into() },
                CILRoot::SetField { addr: CILNode::LoadAddresOfTMPLocal, value: CILNode::LoadGlobalAllocPtr { alloc_id }, desc: ptr_field.into() }
            ].into(),CILNode::LoadTMPLocal)))
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
                    if name == "__rust_alloc_error_handler_should_panic"
                        || name == "__rust_no_alloc_shim_is_unstable"
                    {
                        /* 
                        return CILNode::RawOpsParrentless {
                            ops: [
                                CILOp::LDStaticField(
                                    StaticFieldDescriptor::new(None, Type::U8, name.clone().into())
                                        .into(),
                                ),
                                CILOp::NewTMPLocal(Type::U8.into()),
                                CILOp::SetTMPLocal,
                                CILOp::LoadAddresOfTMPLocal,
                                CILOp::ConvUSize(false),
                                CILOp::FreeTMPLocal,
                            ]
                            .into(),};*/
                            todo!();
                        
                    }
                    if name == "environ" {/* 
                        return CILNode::RawOpsParrentless {
                            ops: [
                                CILOp::LDStaticField(
                                    StaticFieldDescriptor::new(
                                        None,
                                        Type::Ptr(Type::Ptr(Type::U8.into()).into()),
                                        name.clone().into(),
                                    )
                                    .into(),
                                ),
                                CILOp::NewTMPLocal(Type::U8.into()),
                                CILOp::SetTMPLocal,
                                CILOp::LoadAddresOfTMPLocal,
                                CILOp::ConvUSize(false),
                                CILOp::FreeTMPLocal,
                            ]
                            .into(),
                        };*/
                        todo!();
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
        TyKind::RawPtr(_) => CILNode::ConvUSize(CILNode::LdcU64(scalar_u128 as u64).into()),
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
                CILNode::RawOpsParrentless {
                    ops: [
                        CILOp::NewTMPLocal(Type::Void.into()),
                        CILOp::LoadTMPLocal,
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                }
            } else {
                let low = (scalar_u128 & u128::from(u64::MAX)) as u64;
                let high = (scalar_u128 << 64) as u64;
                let low = i64::from_ne_bytes(low.to_ne_bytes());
                let high = i64::from_ne_bytes(high.to_ne_bytes());
                let ctor_sig = crate::function_sig::FnSig::new(
                    &[
                        Type::ManagedReference(Type::U128.into()),
                        Type::U64,
                        Type::U64,
                    ],
                    &Type::Void,
                );
                CILNode::RawOpsParrentless {
                    ops: [
                        CILOp::NewTMPLocal(Type::U128.into()),
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LdcI64(high),
                        CILOp::ConvU64(false),
                        CILOp::LdcI64(low),
                        CILOp::ConvU64(false),
                        CILOp::Call(CallSite::boxed(
                            Some(DotnetTypeRef::uint_128()),
                            ".ctor".into(),
                            ctor_sig,
                            false,
                        )),
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::ConvUSize(false),
                        CILOp::LdObj(tpe.into()),
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                }
            }
        }
        TyKind::Adt(adt_def, _subst) => match adt_def.adt_kind() {
            AdtKind::Enum => {
                let layout = tyctx
                    .layout_of(rustc_middle::ty::ParamEnvAnd {
                        param_env: ParamEnv::reveal_all(),
                        value: scalar_type,
                    })
                    .expect("Could not get type layout!");
                let (disrc_type, _) = crate::utilis::adt::enum_tag_info(&layout.layout, tyctx);
                let enum_dotnet = tpe.as_dotnet().expect("Enum scalar not an ADT!");
                let mut ops = vec![
                    CILOp::NewTMPLocal(tpe.into()),
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::ConvUSize(false),
                ];

                ops.extend(
                    crate::casts::int_to_int(
                        Type::I64,
                        disrc_type.clone(),
                        CILNode::LdcI64(scalar_u128 as u64 as i64),
                    )
                    .flatten(),
                );

                ops.extend([
                    CILOp::STField(Box::new(crate::cil::FieldDescriptor::new(
                        enum_dotnet.clone(),
                        disrc_type,
                        "value__".into(),
                    ))),
                    CILOp::LoadTMPLocal,
                    CILOp::FreeTMPLocal,
                ]);
                CILNode::RawOpsParrentless { ops: ops.into() }
            }
            AdtKind::Struct => {
                //assert!(adt_def.size() < 16);
                let low = (scalar_u128 & u128::from(u64::MAX)) as u64;
                let high = (scalar_u128 << 64) as u64;
                let low = i64::from_ne_bytes(low.to_ne_bytes());
                let high = i64::from_ne_bytes(high.to_ne_bytes());
                let ctor_sig = crate::function_sig::FnSig::new(
                    &[
                        Type::ManagedReference(Type::U128.into()),
                        Type::U64,
                        Type::U64,
                    ],
                    &Type::Void,
                );
                CILNode::RawOpsParrentless {
                    ops: [
                        CILOp::NewTMPLocal(Type::U128.into()),
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LdcI64(high),
                        CILOp::ConvU64(false),
                        CILOp::LdcI64(low),
                        CILOp::ConvU64(false),
                        CILOp::Call(CallSite::boxed(
                            Some(DotnetTypeRef::uint_128()),
                            ".ctor".into(),
                            ctor_sig,
                            false,
                        )),
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::ConvUSize(false),
                        CILOp::LdObj(tpe.into()),
                        CILOp::FreeTMPLocal,
                    ]
                    .into(),
                }
            }
            _ => todo!("Can't load const ADT scalars of type {scalar_type:?}"),
        },
        TyKind::Char => CILNode::LdcU32(scalar_u128 as u32),
        _ => todo!("Can't load scalar constants of type {scalar_type:?}!"),
    }
}
fn load_const_float(value: u128, int_type: &FloatTy, _tyctx: TyCtxt) -> CILNode {
    match int_type {
        FloatTy::F32 => {
            let value = f32::from_ne_bytes((value as u32).to_ne_bytes());
            CILNode::LdcF32(value)
        }
        FloatTy::F64 => {
            let value = f64::from_ne_bytes((value as u64).to_ne_bytes());
            CILNode::LdcF64(value)
        }
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
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
            //eprintln!("value:{value:x} high:{high} low:{low}");
            let ctor_sig = crate::function_sig::FnSig::new(
                &[
                    Type::ManagedReference(Type::I128.into()),
                    Type::U64,
                    Type::U64,
                ],
                &Type::Void,
            );
            CILNode::RawOpsParrentless {
                ops: [
                    //CILOp::NewTMPLocal(Type::I128.into()),
                    //CILOp::LoadAddresOfTMPLocal,
                    CILOp::LdcI64(high),
                    CILOp::ConvI64(false),
                    CILOp::ConvU64(false),
                    CILOp::LdcI64(low),
                    CILOp::ConvI64(false),
                    CILOp::ConvU64(false),
                    CILOp::NewObj(CallSite::boxed(
                        Some(DotnetTypeRef::int_128()),
                        ".ctor".into(),
                        ctor_sig,
                        false,
                    )),
                    //CILOp::LoadTMPLocal,
                    //CILOp::FreeTMPLocal,
                ]
                .into(),
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
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
            let ctor_sig = crate::function_sig::FnSig::new(
                &[
                    Type::ManagedReference(Type::U128.into()),
                    Type::U64,
                    Type::U64,
                ],
                &Type::Void,
            );
            CILNode::RawOpsParrentless {
                ops: vec![
                    CILOp::LdcI64(high),
                    CILOp::ConvI64(false),
                    CILOp::ConvU64(false),
                    CILOp::LdcI64(low),
                    CILOp::ConvI64(false),
                    CILOp::ConvU64(false),
                    CILOp::NewObj(CallSite::boxed(
                        Some(DotnetTypeRef::uint_128()),
                        ".ctor".into(),
                        ctor_sig,
                        false,
                    )),
                ]
                .into(),
            }
        }
    }
}
