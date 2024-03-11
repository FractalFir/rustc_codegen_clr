use super::{pointed_type, PlaceTy};
use crate::cil::{CallSite, FieldDescriptor};
use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;
use crate::function_sig::FnSig;
use crate::r#type::{pointer_to_is_fat, DotnetTypeRef, Type};
use crate::{add, call, conv_usize, ld_field, ldc_u64, mul, size_of};

use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{FloatTy, Instance, IntTy, TyCtxt, TyKind, UintTy};

pub fn local_set(local: usize, method: &rustc_middle::mir::Body, tree: CILNode) -> CILRoot {
    if local == 0 {
        CILRoot::STLoc { local: 0, tree }
    } else if local > method.arg_count {
        CILRoot::STLoc {
            local: (local - method.arg_count) as u32,
            tree,
        }
    } else {
        CILRoot::STArg {
            arg: (local - 1) as u32,
            tree,
        }
    }
}
pub fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: PlaceTy<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
    addr_calc: CILNode,
    value_calc: CILNode,
) -> CILRoot {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);

            ptr_set_op(
                pointed_type.into(),
                ctx,
                &method_instance,
                type_cache,
                addr_calc,
                value_calc,
            )
        }
        PlaceElem::Field(index, _field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
                let field_desc = crate::utilis::field_descrptor(
                    curr_type,
                    (*index).into(),
                    ctx,
                    method_instance,
                    type_cache,
                );
                CILRoot::SetField {
                    addr: addr_calc,
                    value: value_calc,
                    desc: field_desc,
                }
            }
            super::PlaceTy::EnumVariant(enm, var_idx) => {
                let enm = crate::utilis::monomorphize(&method_instance, enm, ctx);
                let field_desc = crate::utilis::enum_field_descriptor(
                    enm,
                    index.as_u32(),
                    var_idx,
                    ctx,
                    method_instance,
                    type_cache,
                );

                CILRoot::SetField {
                    addr: addr_calc,
                    value: value_calc,
                    desc: field_desc,
                }
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                ctx.optimized_mir(method_instance.def_id()),
            );
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, ctx);
                    let inner_type = type_cache.type_from_cache(inner, ctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, ctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );

                    ptr_set_op(
                        super::PlaceTy::Ty(inner),
                        ctx,
                        &method_instance,
                        type_cache,
                        add!(
                            ld_field!(addr_calc, desc),
                            mul!(index, conv_usize!(size_of!(inner_type)))
                        ),
                        value_calc,
                    )
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, ctx);
                    let array_type =
                        type_cache.type_from_cache(curr_ty, ctx, Some(method_instance));
                    let element_type =
                        type_cache.type_from_cache(element, ctx, Some(method_instance));

                    let array_dotnet = array_type.as_dotnet().expect("Non array type");

                    CILRoot::Call {
                        site: crate::cil::CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element_type],
                                &Type::Void,
                            ),
                            false,
                        ),
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
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = ldc_u64!(*offset);
            assert!(!from_end, "Indexing slice form end");
            
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, ctx);

                    let inner_type = type_cache.type_from_cache(inner, ctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, ctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let metadata = FieldDescriptor::new(slice, Type::USize, "metadata".into());
                    let addr = add!(
                        ld_field!(
                            addr_calc.clone(),
                            desc
                        ),
                        mul!(
                            call!(
                                CallSite::new(
                                    None,
                                    "bounds_check".into(),
                                    FnSig::new(&[Type::USize, Type::USize], &Type::USize),
                                    true
                                ),
                                [
                                    ld_field!(addr_calc, metadata),
                                    conv_usize!(ldc_u64!(*min_length))
                                ]
                            ),
                            conv_usize!(CILNode::SizeOf(inner_type.into()))
                        )
                    );
                    ptr_set_op(
                        super::PlaceTy::Ty(inner),
                        ctx,
                        &method_instance,
                        type_cache,
                        addr,
                        value_calc,
                    )
                }
                TyKind::Array(element, _length) => {
                    //println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
                    let element = crate::utilis::monomorphize(&method_instance, *element, ctx);
                    let element = type_cache.type_from_cache(element, ctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, ctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    CILRoot::Call {
                        site: crate::cil::CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element],
                                &Type::Void,
                            ),
                            false,
                        ),
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
fn ptr_set_op<'ctx>(
    pointed_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: &Instance<'ctx>,
    type_cache: &mut crate::r#type::TyCache,
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
                    addr_calc,
                    value_calc,
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
                    addr_calc,
                    value_calc,
                },
            },
            TyKind::Float(float_ty) => match float_ty {
                FloatTy::F32 => CILRoot::STIndF32(addr_calc, value_calc),
                FloatTy::F64 => CILRoot::STIndF64(addr_calc, value_calc),
            },
            TyKind::Bool => CILRoot::STIndI8(addr_calc, value_calc), // Both Rust bool and a managed bool are 1 byte wide. .NET bools are 4 byte wide only in the context of Marshaling/PInvoke,
            // due to historic reasons(BOOL was an alias for int in early Windows, and it stayed this way.) - FractalFir
            TyKind::Char => CILRoot::STIndI32(addr_calc, value_calc), // always 4 bytes wide: https://doc.rust-lang.org/std/primitive.char.html#representation
            TyKind::Adt(_, _) | TyKind::Tuple(_) | TyKind::Array(_, _) => {
                let pointed_type =
                    type_cache.type_from_cache(pointed_type, tyctx, Some(*method_instance));
                CILRoot::STObj {
                    tpe: pointed_type.into(),
                    addr_calc,
                    value_calc,
                }
            }
            TyKind::Ref(_, inner, _) => {
                if pointer_to_is_fat(*inner, tyctx, Some(*method_instance)) {
                    CILRoot::STObj {
                        tpe: type_cache
                            .type_from_cache(pointed_type, tyctx, Some(*method_instance))
                            .into(),
                        addr_calc,
                        value_calc,
                    }
                } else {
                    CILRoot::STIndISize(addr_calc, value_calc)
                }
            }
            TyKind::RawPtr(ty_and_mut) => {
                if pointer_to_is_fat(ty_and_mut.ty, tyctx, Some(*method_instance)) {
                    CILRoot::STObj {
                        tpe: type_cache
                            .type_from_cache(pointed_type, tyctx, Some(*method_instance))
                            .into(),
                        addr_calc,
                        value_calc,
                    }
                } else {
                    CILRoot::STIndISize(addr_calc, value_calc)
                }
            }
            _ => todo!(" can't deref type {pointed_type:?} yet"),
        }
    } else {
        todo!("Can't set the value behind a poitner to an enum variant!");
    }
}
