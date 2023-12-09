use crate::{
    cil::{CILOp, CallSite, FieldDescriptor, StaticFieldDescriptor},
    r#type::{DotnetTypeRef, TyCache, Type},
};
use rustc_abi::Size;
use rustc_middle::mir::{
    interpret::{AllocId, AllocRange, GlobalAlloc, Scalar},
    ConstOperand, ConstValue,
};
use rustc_middle::ty::{
    AdtDef, AdtKind, FloatTy, Instance, IntTy, List, ParamEnv, Ty, TyCtxt, TyKind, UintTy,
};
pub fn handle_constant<'ctx>(
    constant_op: &ConstOperand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
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
/// Returns the ops neceasry to create constant ADT of type represented by `adt_def` and `subst` with byte values matching the ones in the slice bytes
fn create_const_adt_from_bytes<'ctx>(
    ty: Ty<'ctx>,
    adt_def: AdtDef<'ctx>,
    subst: &'ctx List<rustc_middle::ty::GenericArg<'ctx>>,
    tyctx: TyCtxt<'ctx>,
    bytes: &[u8],
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    match adt_def.adt_kind() {
        AdtKind::Struct => {
            let mut curr_offset = 0;
            let cil_ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
            let cil_ty = tycache.type_from_cache(cil_ty, tyctx, Some(method_instance));
            let dotnet_ty = cil_ty.as_dotnet().expect("ADT must be a value type!");
            let mut creator_ops = vec![CILOp::NewTMPLocal(cil_ty.clone().into())];
            for (_field_idx, field) in adt_def.all_fields().enumerate() {
                let ftype = field.ty(tyctx, subst);
                let sizeof = crate::utilis::compiletime_sizeof(ftype, tyctx);
                let field_bytes = &bytes[curr_offset..(curr_offset + sizeof)];
                let field_ops =
                    create_const_from_slice(ftype, tyctx, field_bytes, method_instance, tycache);
                creator_ops.push(CILOp::LoadAddresOfTMPLocal);
                creator_ops.extend(field_ops);
                let cil_ftype =
                    tycache.type_from_cache(field.ty(tyctx, subst), tyctx, Some(method_instance));
                let name = field.name.to_string();
                let name = crate::r#type::escape_field_name(&name);
                creator_ops.push(CILOp::STField(crate::cil::FieldDescriptor::boxed(
                    dotnet_ty.clone(),
                    cil_ftype,
                    name,
                )));
                rustc_middle::ty::print::with_no_trimmed_paths! {println!(
                    "Const field {name} of type {ftype} with bytes {field_bytes:?}",
                    name = field.name
                )};
                // Increment the offset.
                curr_offset += sizeof;
            }
            creator_ops.push(CILOp::LoadTMPLocal);
            creator_ops.push(CILOp::FreeTMPLocal);
            creator_ops
        }
        AdtKind::Enum => {
            let variant_size = crate::utilis::enum_tag_size(adt_def.variants().len() as u64);
            let mut curr_offset = 0;
            let enum_ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
            let enum_ty = tycache.type_from_cache(enum_ty, tyctx, Some(method_instance));
            let enum_dotnet: DotnetTypeRef = if let Type::DotnetType(ty_ref) = &enum_ty {
                ty_ref.as_ref().clone()
            } else {
                panic!("Invalid enum type {enum_ty:?}");
            };
            let mut ops = vec![CILOp::NewTMPLocal(enum_ty.into())];
            let curr_variant = match variant_size {
                0 => todo!("Can't yet handle constant enums with 0-sized tags."),
                1 => {
                    curr_offset = 1;
                    let variant = bytes[0] as u32;
                    ops.extend([
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LdcI32(variant as i32),
                        CILOp::STField(FieldDescriptor::boxed(
                            enum_dotnet.clone(),
                            Type::U8,
                            "_tag".into(),
                        )),
                    ]);
                    variant
                }
                _ => todo!("Can't yet support enums with {variant_size} wide tags."),
            };
            assert!(curr_variant < adt_def.variants().len() as u32);
            let active_variant = &adt_def.variants()[curr_variant.into()];
            for _field in &active_variant.fields {
                todo!("Can't yet create const enum fields.");
            }
            ops.push(CILOp::FreeTMPLocal);
            //todo!("Can't load const enum from bytes {bytes:?}!");
            ops
        }
        AdtKind::Union => todo!("Can't load const union from bytes {bytes:?}!"),
    }
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the slice bytes
fn create_const_from_slice<'ctx>(
    ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    bytes: &[u8],
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
    // TODO: Read up on the order of bytes inside a const allocation and ensure it is correct. All .NET target will be Little Enidian, but if we want to support
    // big enidian targets in the future, this will need to be revised.
    match ty.kind() {
        TyKind::Adt(adt_def, subst) => {
            create_const_adt_from_bytes(ty, *adt_def, subst, tyctx, bytes, method_instance, tycache)
        }
        TyKind::Int(int) => match int {
            IntTy::I8 => vec![
                CILOp::LdcI32(i8::from_le_bytes(
                    bytes[..std::mem::size_of::<i8>()].try_into().unwrap(),
                ) as i32),
                CILOp::ConvI8(false),
            ],
            IntTy::I16 => vec![
                CILOp::LdcI32(i16::from_le_bytes(
                    bytes[..std::mem::size_of::<i16>()].try_into().unwrap(),
                ) as i32),
                CILOp::ConvI16(false),
            ],
            IntTy::I32 => vec![CILOp::LdcI32(i32::from_le_bytes(
                bytes[..std::mem::size_of::<i32>()].try_into().unwrap(),
            ))],
            IntTy::I64 => vec![CILOp::LdcI64(i64::from_le_bytes(
                bytes[..std::mem::size_of::<i64>()].try_into().unwrap(),
            ))],
            IntTy::Isize => vec![
                CILOp::LdcI64(i64::from_le_bytes(
                    bytes[..crate::utilis::compiletime_sizeof(ty, tyctx)]
                        .try_into()
                        .unwrap(),
                )),
                CILOp::ConvUSize(false),
            ],
            _ => todo!("Can't yet load const value of type {int:?} with bytes:{bytes:?}"),
        },
        TyKind::Uint(int) => match int {
            UintTy::U8 => vec![
                CILOp::LdcI32(i8::from_le_bytes(
                    bytes[..std::mem::size_of::<i8>()].try_into().unwrap(),
                ) as i32),
                CILOp::ConvU8(false),
            ],
            UintTy::U16 => vec![
                CILOp::LdcI32(i16::from_le_bytes(
                    bytes[..std::mem::size_of::<i16>()].try_into().unwrap(),
                ) as i32),
                CILOp::ConvU16(false),
            ],
            UintTy::U32 => vec![CILOp::LdcI32(i32::from_le_bytes(
                bytes[..std::mem::size_of::<i32>()].try_into().unwrap(),
            ))],
            UintTy::U64 => vec![CILOp::LdcI64(i64::from_le_bytes(
                bytes[..std::mem::size_of::<i64>()].try_into().unwrap(),
            ))],
            UintTy::Usize => vec![
                CILOp::LdcI64(i64::from_le_bytes(
                    bytes[..crate::utilis::compiletime_sizeof(ty, tyctx)]
                        .try_into()
                        .unwrap(),
                )),
                CILOp::ConvUSize(false),
            ],
            _ => todo!("Can't yet load const value of type {int:?} with bytes:{bytes:?}"),
        },
        TyKind::RawPtr(type_and_mut) => match type_and_mut.ty.kind() {
            TyKind::Slice(_) => {
                todo!("Can't load const slices.")
            }
            TyKind::Str => {
                todo!("Can't load const string slices.")
            }
            _ => {
                eprintln!("WARNING: assuming sizeof<*T>() == 8!");
                vec![CILOp::LdcI64(i64::from_le_bytes(
                    bytes[..std::mem::size_of::<i64>()].try_into().unwrap(),
                ))]
            }
        },
        TyKind::Bool => vec![CILOp::LdcI32(bytes[0] as i32)],
        TyKind::Tuple(elements) => {
            assert!(
                elements.len() < 8,
                "Can't create a const tuple with more than 8 elements yet!"
            );
            let element_types: Vec<_> = elements
                .iter()
                .map(|ele| {
                    let ele = crate::utilis::monomorphize(&method_instance, ele, tyctx);
                    tycache.type_from_cache(ele, tyctx, Some(method_instance))
                })
                .collect();
            let tuple_dotnet = crate::r#type::tuple_type(&element_types);
            let tuple_type: Type = tuple_dotnet.clone().into();
            let mut ops = vec![CILOp::NewTMPLocal(tuple_type.clone().into())];
            let mut curr_offset = 0;
            for (idx, (element_type, element_ty)) in
                element_types.iter().zip(elements.iter()).enumerate()
            {
                let sizeof = crate::utilis::compiletime_sizeof(element_ty, tyctx);
                let field_bytes = &bytes[curr_offset..(curr_offset + sizeof)];
                let field_ops = create_const_from_slice(
                    element_ty,
                    tyctx,
                    field_bytes,
                    method_instance,
                    tycache,
                );
                ops.push(CILOp::LoadAddresOfTMPLocal);
                ops.extend(field_ops);
                ops.push(CILOp::STField(FieldDescriptor::boxed(
                    tuple_dotnet.clone(),
                    element_type.clone(),
                    format!("Item{num}", num = idx + 1).into(),
                )));
                curr_offset += sizeof;
            }
            ops.push(CILOp::LoadTMPLocal);
            ops.push(CILOp::FreeTMPLocal);
            ops
        }
        TyKind::Array(element_ty, length) => {
            let array_type = tycache.type_from_cache(ty, tyctx, Some(method_instance));
            let dotnet_array_type = array_type.clone().as_dotnet().expect("Array not array!");
            let length = crate::utilis::monomorphize(&method_instance, *length, tyctx);
            let element_ty = crate::utilis::monomorphize(&method_instance, *element_ty, tyctx);

            let element_sizeof = crate::utilis::compiletime_sizeof(element_ty, tyctx);
            let length = crate::utilis::try_resolve_const_size(&length).unwrap();
            let mut curr_offset = 0;
            let mut res = vec![CILOp::NewTMPLocal(array_type.clone().into())];
            for index in 0..length {
                res.push(CILOp::LoadAddresOfTMPLocal);
                res.push(CILOp::LdcI64(index as u64 as i64));
                res.extend(create_const_from_slice(
                    element_ty,
                    tyctx,
                    &bytes[curr_offset..],
                    method_instance,
                    tycache,
                ));
                res.push(CILOp::Call(
                    CallSite::new(
                        Some(dotnet_array_type.clone()),
                        "set_Item".into(),
                        crate::function_sig::FnSig::new(
                            &[array_type.clone(), Type::ISize, Type::GenericArg(0)],
                            &Type::Void,
                        ),
                        false,
                    )
                    .into(),
                ));
                curr_offset += element_sizeof;
            }
            res.extend([CILOp::LoadTMPLocal, CILOp::FreeTMPLocal]);
            res
        }
        _ => todo!("Can't yet load const value of type {ty:?} with bytes:{bytes:?}"),
    }
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the allocation
fn create_const_from_data<'ctx>(
    ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    alloc_id: AllocId,
    offset_bytes: u64,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    let alloc = tyctx.global_alloc(alloc_id);
    // Constant should be memory:
    let memory = alloc.unwrap_memory();
    let len = memory.0.len();
    let range = AllocRange {
        start: Size::from_bytes(offset_bytes),
        size: Size::from_bytes((len as u64) - offset_bytes),
    };
    let bytes = memory.0.get_bytes_unchecked(range);
    create_const_from_slice(ty, tyctx, bytes, method_instance, tycache)
}

fn load_const_value<'ctx>(
    const_val: ConstValue<'ctx>,
    const_ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    match const_val {
        ConstValue::Scalar(scalar) => {
            load_const_scalar(scalar, const_ty, tyctx, method, method_instance, tycache)
        }
        ConstValue::ZeroSized => {
            let tpe = crate::utilis::monomorphize(&method_instance, const_ty, tyctx);
            let tpe = tycache.type_from_cache(tpe, tyctx, Some(method_instance));
            vec![
                CILOp::NewTMPLocal(tpe.into()),
                CILOp::LoadTMPLocal,
                CILOp::FreeTMPLocal,
            ]
        }
        ConstValue::Slice { data, meta } => {
            let slice_type = tycache.type_from_cache(const_ty, tyctx, Some(method_instance));
            let slice_dotnet = slice_type.as_dotnet().expect("Slice type invalid!");
            let metadata_field =
                FieldDescriptor::new(slice_dotnet.clone(), Type::USize, "metadata".into());
            let ptr_field = FieldDescriptor::new(
                slice_dotnet,
                Type::Ptr(Type::Void.into()),
                "data_address".into(),
            );
            // TODO: find a better way to get an alloc_id. This is likely to be incoreect.
            let alloc_id = tyctx.reserve_and_set_memory_alloc(data);
            let alloc_id: u64 = crate::utilis::alloc_id_to_u64(alloc_id);

            vec![
                CILOp::NewTMPLocal(slice_type.into()),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LdcI64(meta as i64),
                CILOp::STField(metadata_field.into()),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LoadGlobalAllocPtr { alloc_id },
                CILOp::STField(ptr_field.into()),
                CILOp::LoadTMPLocal,
                CILOp::FreeTMPLocal,
            ]
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
) -> Vec<CILOp> {
    let scalar_type = crate::utilis::monomorphize(&method_instance, scalar_type, tyctx);
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(ptr, _size) => {
            let (alloc_id, offset) = ptr.into_parts();
            let global_alloc = tyctx.global_alloc(alloc_id);
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
                        return vec![
                            CILOp::LDStaticField(
                                StaticFieldDescriptor::new(None, Type::U8, name.clone().into())
                                    .into(),
                            ),
                            CILOp::NewTMPLocal(Type::U8.into()),
                            CILOp::SetTMPLocal,
                            CILOp::LoadAddresOfTMPLocal,
                            CILOp::FreeTMPLocal,
                        ];
                    }
                    if name == "environ" {
                        return vec![
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
                            CILOp::FreeTMPLocal,
                        ];
                    }
                    let attrs = tyctx.codegen_fn_attrs(def_id);
                    rustc_middle::ty::print::with_no_trimmed_paths! {
                        eprintln!("Codegening static {name}.")
                    };
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
                    let alloc_id = crate::utilis::alloc_id_to_u64(alloc_id);
                    return vec![CILOp::LoadGlobalAllocPtr { alloc_id }];
                }
                GlobalAlloc::Memory(_const_allocation) => {
                    return vec![
                        CILOp::LoadGlobalAllocPtr {
                            alloc_id: alloc_id.0.into(),
                        },
                        CILOp::LdcI64(offset.bytes() as i64),
                        CILOp::ConvISize(false),
                        CILOp::Add,
                    ];
                }
                _ => todo!("Unhandled global alloc {global_alloc:?}"),
            }
            //panic!("alloc_id:{alloc_id:?}")
        }
    };
    let tpe = crate::utilis::monomorphize(&method_instance, scalar_type, tyctx);
    let tpe = tycache.type_from_cache(tpe, tyctx, Some(method_instance));
    match scalar_type.kind() {
        TyKind::Int(int_type) => load_const_int(scalar_u128, int_type),
        TyKind::Uint(uint_type) => load_const_uint(scalar_u128, uint_type),
        TyKind::Float(ftype) => load_const_float(scalar_u128, ftype, tyctx),
        TyKind::Bool => vec![CILOp::LdcI32(scalar_u128 as i32)],
        TyKind::RawPtr(_) => {
            let value = i64::from_ne_bytes((scalar_u128 as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value)]
        }
        TyKind::Adt(adt_def, _subst) => match adt_def.adt_kind() {
            AdtKind::Enum => {
                let field_type = Type::U8;
                let enum_dotnet = tpe.as_dotnet().expect("Enum scalar not an ADT!");
                vec![
                    CILOp::NewTMPLocal(tpe.into()),
                    CILOp::LoadAddresOfTMPLocal,
                    CILOp::LdcI64(scalar_u128 as i64),
                    CILOp::STField(Box::new(crate::cil::FieldDescriptor::new(
                        enum_dotnet.clone(),
                        field_type,
                        "_tag".into(),
                    ))),
                    CILOp::LoadTMPLocal,
                    CILOp::FreeTMPLocal,
                ]
            }
            _ => todo!("Can't load const ADT scalars of type {scalar_type:?}"),
        },
        TyKind::Char => {
            let value = i64::from_ne_bytes((scalar_u128 as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvU64(false)]
        }
        _ => todo!("Can't load scalar constants of type {scalar_type:?}!"),
    }
}
fn load_const_float(value: u128, int_type: &FloatTy, _tyctx: TyCtxt) -> Vec<CILOp> {
    match int_type {
        FloatTy::F32 => {
            let value = f32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcF32(value)]
        }
        FloatTy::F64 => {
            let value = f64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcF64(value)]
        }
    }
}
pub fn load_const_int(value: u128, int_type: &IntTy) -> Vec<CILOp> {
    match int_type {
        IntTy::I8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvI8(false)]
        }
        IntTy::I16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvI16(false)]
        }
        IntTy::I32 => {
            let value = i32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcI32(value)]
        }
        IntTy::I64 => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvI64(false)]
        }
        IntTy::Isize => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvISize(false)]
        }
        IntTy::I128 => {
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
            let i128_class = DotnetTypeRef::new(Some("System.Runtime"), "System.Int128");
            let ctor_sig = crate::function_sig::FnSig::new(&[Type::U64, Type::U64], &Type::I128);
            vec![
                CILOp::LdcI64(high),
                CILOp::LdcI64(low),
                CILOp::Call(CallSite::boxed(
                    Some(i128_class),
                    ".ctor".into(),
                    ctor_sig,
                    true,
                )),
            ]
        }
    }
}
pub fn load_const_uint(value: u128, int_type: &UintTy) -> Vec<CILOp> {
    match int_type {
        UintTy::U8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvU8(false)]
        }
        UintTy::U16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvU16(false)]
        }
        UintTy::U32 => {
            let value = i32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcI32(value)]
        }
        UintTy::U64 => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvU64(false)]
        }
        UintTy::Usize => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvUSize(false)]
        }
        UintTy::U128 => {
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
            let i128_class = DotnetTypeRef::new(Some("System.Runtime"), "System.UInt128");
            let ctor_sig = crate::function_sig::FnSig::new(&[Type::U64, Type::U64], &Type::U128);
            vec![
                CILOp::LdcI64(high),
                CILOp::LdcI64(low),
                CILOp::Call(CallSite::boxed(
                    Some(i128_class),
                    ".ctor".into(),
                    ctor_sig,
                    true,
                )),
            ]
        }
    }
}
