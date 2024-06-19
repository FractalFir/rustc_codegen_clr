use crate::{
    assembly::MethodCompileCtx,
    place::{pointed_type, PlaceTy},
    r#type::pointer_to_is_fat,
};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_usize,
    field_desc::FieldDescriptor, fn_sig::FnSig, ld_field, ldc_u64, size_of, DotnetTypeRef, Type,
};
use rustc_middle::{
    mir::PlaceElem,
    ty::{FloatTy, IntTy, TyKind, UintTy},
};
pub fn local_set(local: usize, method: &rustc_middle::mir::Body, tree: CILNode) -> CILRoot {
    if let Some(spread_arg) = method.spread_arg
        && local == spread_arg.as_usize()
    {
        return CILRoot::STLoc {
            local: (method.local_decls.len() - method.arg_count)
                .try_into()
                .unwrap(),
            tree,
        };
    }
    if local == 0 {
        CILRoot::STLoc { local: 0, tree }
    } else if local > method.arg_count {
        CILRoot::STLoc {
            local: u32::try_from(local - method.arg_count).unwrap(),
            tree,
        }
    } else {
        CILRoot::STArg {
            arg: u32::try_from(local - 1).unwrap(),
            tree,
        }
    }
}
pub fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_, '_>,
    addr_calc: CILNode,
    value_calc: CILNode,
) -> CILRoot {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);

            ptr_set_op(pointed_type.into(), ctx, addr_calc, value_calc)
        }
        PlaceElem::Field(field_index, _field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                let curr_type = ctx.monomorphize(curr_type);
                let field_desc =
                    crate::utilis::field_descrptor(curr_type, (*field_index).into(), ctx);
                CILRoot::SetField {
                    addr: Box::new(addr_calc),
                    value: Box::new(value_calc),
                    desc: Box::new(field_desc),
                }
            }
            super::PlaceTy::EnumVariant(enm, var_idx) => {
                let enm = ctx.monomorphize(enm);
                let field_desc =
                    crate::utilis::enum_field_descriptor(enm, field_index.as_u32(), var_idx, ctx);

                CILRoot::SetField {
                    addr: Box::new(addr_calc),
                    value: Box::new(value_calc),
                    desc: Box::new(field_desc),
                }
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(index.as_usize(), ctx.body());
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
                    ptr_set_op(
                        super::PlaceTy::Ty(inner),
                        ctx,
                        CILNode::TransmutePtr {
                            val: Box::new(ld_field!(addr_calc, desc)),
                            new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                        } + index * conv_usize!(size_of!(inner_type)),
                        value_calc,
                    )
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let element_type = ctx.type_from_cache(element);

                    let array_dotnet = array_type.as_dotnet().expect("Non array type");

                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element_type],
                                Type::Void,
                            ),
                            false,
                        )),
                        args: [addr_calc, index, value_calc].into(),
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
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
            let index = ldc_u64!(*offset);
            assert!(!from_end, "Indexing slice form end");

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
                    let metadata = FieldDescriptor::new(slice, Type::USize, "metadata".into());
                    let addr = CILNode::TransmutePtr {
                        val: Box::new(ld_field!(addr_calc.clone(), desc)),
                        new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                    } + call!(
                        CallSite::new(
                            None,
                            "bounds_check".into(),
                            FnSig::new(&[Type::USize, Type::USize], Type::USize),
                            true
                        ),
                        [conv_usize!(index), ld_field!(addr_calc, metadata),]
                    ) * conv_usize!(CILNode::SizeOf(inner_type.into()));
                    ptr_set_op(super::PlaceTy::Ty(inner), ctx, addr, value_calc)
                }
                TyKind::Array(element, _length) => {
                    //println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element],
                                Type::Void,
                            ),
                            false,
                        )),
                        args: [addr_calc, conv_usize!(index), value_calc].into(),
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        /*
        PlaceElem::ConstantIndex {
            offset,
            min_length: _,
            from_end,
        } => {
            let mut ops = if !from_end {
                CILRoot::LdcI64(*offset as i64)]
            } else {
                let mut get_len = place_get_length(curr_type, ctx, method_instance, type_cache);
                get_len.extend(CILRoot::LdcI64(*offset as i64), CILOp::Sub]);
                get_len
            };
            ops.extend(place_elem_set_at(
                curr_type,
                ctx,
                &method_instance,
                type_cache,
            ));
            ops
        }*/
        _ => todo!("Can't handle porojection {place_elem:?} in set"),
    }
}
/// Returns a set of instructons to set a pointer to a `pointed_type` to a value from the stack.
pub fn ptr_set_op<'ctx>(
    pointed_type: PlaceTy<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
    addr_calc: CILNode,
    value_calc: CILNode,
) -> CILRoot {
    if let PlaceTy::Ty(pointed_type) = pointed_type {
        match pointed_type.kind() {
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I8 => CILRoot::STIndI8(addr_calc, value_calc),
                IntTy::I16 => CILRoot::STIndI16(addr_calc, value_calc),
                IntTy::I32 => CILRoot::STIndI32(addr_calc, value_calc),
                IntTy::I64 => CILRoot::STIndI64(addr_calc, value_calc),
                IntTy::Isize => CILRoot::STIndISize(addr_calc, value_calc),
                IntTy::I128 => CILRoot::STObj {
                    tpe: Box::new(DotnetTypeRef::int_128().into()),
                    addr_calc: Box::new(addr_calc),
                    value_calc: Box::new(value_calc),
                },
            },
            TyKind::Uint(int_ty) => match int_ty {
                UintTy::U8 => CILRoot::STIndI8(addr_calc, value_calc),
                UintTy::U16 => CILRoot::STIndI16(addr_calc, value_calc),
                UintTy::U32 => CILRoot::STIndI32(addr_calc, value_calc),
                UintTy::U64 => CILRoot::STIndI64(addr_calc, value_calc),
                UintTy::Usize => CILRoot::STIndISize(addr_calc, value_calc),
                UintTy::U128 => CILRoot::STObj {
                    tpe: Box::new(DotnetTypeRef::uint_128().into()),
                    addr_calc: Box::new(addr_calc),
                    value_calc: Box::new(value_calc),
                },
            },
            TyKind::Float(float_ty) => match float_ty {
                FloatTy::F32 => CILRoot::STIndF32(addr_calc, value_calc),
                FloatTy::F64 => CILRoot::STIndF64(addr_calc, value_calc),
                _ => todo!("Can't yet set {float_ty:?} behnind a pointer."),
            },
            TyKind::Bool => CILRoot::STIndI8(addr_calc, value_calc), // Both Rust bool and a managed bool are 1 byte wide. .NET bools are 4 byte wide only in the context of Marshaling/PInvoke,
            // due to historic reasons(BOOL was an alias for int in early Windows, and it stayed this way.) - FractalFir
            TyKind::Char => CILRoot::STIndI32(addr_calc, value_calc), // always 4 bytes wide: https://doc.rust-lang.org/std/primitive.char.html#representation
            TyKind::Adt(_, _) | TyKind::Tuple(_) | TyKind::Array(_, _) | TyKind::Closure(_, _) => {
                let pointed_type = ctx.type_from_cache(pointed_type);
                CILRoot::STObj {
                    tpe: pointed_type.into(),
                    addr_calc: Box::new(addr_calc),
                    value_calc: Box::new(value_calc),
                }
            }
            TyKind::Ref(_, inner, _) => {
                if pointer_to_is_fat(*inner, ctx.tyctx(), ctx.instance()) {
                    CILRoot::STObj {
                        tpe: ctx.type_from_cache(pointed_type).into(),
                        addr_calc: Box::new(addr_calc),
                        value_calc: Box::new(value_calc),
                    }
                } else {
                    let inner = ctx.type_from_cache(*inner);
                    CILRoot::STIndPtr(addr_calc, value_calc, Box::new(inner))
                }
            }
            TyKind::RawPtr(ty, _) => {
                if pointer_to_is_fat(*ty, ctx.tyctx(), ctx.instance()) {
                    CILRoot::STObj {
                        tpe: ctx.type_from_cache(pointed_type).into(),
                        addr_calc: Box::new(addr_calc),
                        value_calc: Box::new(value_calc),
                    }
                } else {
                    let inner = ctx.type_from_cache(*ty);
                    CILRoot::STIndPtr(addr_calc, value_calc, Box::new(inner))
                }
            }
            _ => todo!(" can't deref type {pointed_type:?} yet"),
        }
    } else {
        todo!("Can't set the value behind a poitner to an enum variant!");
    }
}
