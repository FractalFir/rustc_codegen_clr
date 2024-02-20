use super::{pointed_type, PlaceTy};
use crate::cil::{CILOp, FieldDescriptor};
use crate::function_sig::FnSig;
use crate::r#type::{pointer_to_is_fat, DotnetTypeRef, Type};

use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{FloatTy, Instance, IntTy, TyCtxt, TyKind, UintTy};

pub fn local_set(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::STLoc(0)
    } else if local > method.arg_count {
        CILOp::STLoc((local - method.arg_count) as u32)
    } else {
        CILOp::STArg((local - 1) as u32)
    }
}
pub fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: PlaceTy<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
    value_calc: Vec<CILOp>,
    addr_calc: Vec<CILOp>,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);
            let mut ops = Vec::new();
            ops.extend(addr_calc);
            ops.extend(value_calc);
            ops.extend(ptr_set_op(
                pointed_type.into(),
                ctx,
                &method_instance,
                type_cache,
            ));
            ops
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
                let mut ops = Vec::new();
                ops.extend(addr_calc);
                ops.extend(value_calc);
                ops.push(CILOp::STField(field_desc.into()));
                ops
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
                let mut ops = Vec::new();
                ops.extend(addr_calc);
                ops.extend(value_calc);
                ops.push(CILOp::STField(field_desc.into()));
                ops
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                ctx.optimized_mir(method_instance.def_id()),
            )
            .flatten();
            assert_eq!(index.len(), 1);
            let index = index[0].clone();
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
                    let ptr_set_op =
                        ptr_set_op(super::PlaceTy::Ty(inner), ctx, &method_instance, type_cache);
                    let mut ops = Vec::new();
                    ops.extend(addr_calc);
                    ops.extend([
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::ConvUSize(false),
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::ConvUSize(false),
                        CILOp::Mul,
                        CILOp::Add,
                    ]);
                    ops.extend(value_calc);
                    ops.extend(ptr_set_op);
                    ops
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, ctx);
                    let array_type =
                        type_cache.type_from_cache(curr_ty, ctx, Some(method_instance));
                    let element_type =
                        type_cache.type_from_cache(element, ctx, Some(method_instance));

                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let mut ops = Vec::new();
                    ops.extend(addr_calc);
                    ops.push(index);
                    ops.extend(value_calc);
                    ops.extend([CILOp::Call(
                        crate::cil::CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element_type],
                                &Type::Void,
                            ),
                            false,
                        )
                        .into(),
                    )]);
                    ops
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
            let index = CILOp::LdcI64(*offset as i64);
            assert!(!from_end, "Indexing slice form end");
            println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
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
                    let ptr_set_op =
                        ptr_set_op(super::PlaceTy::Ty(inner), ctx, &method_instance, type_cache);
                    let mut ops = Vec::new();
                    ops.extend(addr_calc);
                    ops.extend([
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::ConvUSize(false),
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::ConvUSize(false),
                        CILOp::Mul,
                        CILOp::Add,
                    ]);
                    ops.extend(value_calc);
                    ops.extend(ptr_set_op);
                    ops
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, ctx);
                    let element = type_cache.type_from_cache(element, ctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, ctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let mut ops = Vec::new();
                    ops.extend(addr_calc);
                    ops.push(index);
                    ops.push(CILOp::ConvUSize(false));
                    ops.extend(value_calc);
                    ops.extend([CILOp::Call(
                        crate::cil::CallSite::new(
                            Some(array_dotnet),
                            "set_Item".into(),
                            FnSig::new(
                                &[Type::Ptr(array_type.into()), Type::USize, element],
                                &Type::Void,
                            ),
                            false,
                        )
                        .into(),
                    )]);
                    ops
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
                vec![CILOp::LdcI64(*offset as i64)]
            } else {
                let mut get_len = place_get_length(curr_type, ctx, method_instance, type_cache);
                get_len.extend(vec![CILOp::LdcI64(*offset as i64), CILOp::Sub]);
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
) -> Vec<CILOp> {
    if let PlaceTy::Ty(pointed_type) = pointed_type {
        match pointed_type.kind() {
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I8 => vec![CILOp::STIndI8],
                IntTy::I16 => vec![CILOp::STIndI16],
                IntTy::I32 => vec![CILOp::STIndI32],
                IntTy::I64 => vec![CILOp::STIndI64],
                IntTy::Isize => vec![CILOp::STIndISize],
                IntTy::I128 => vec![CILOp::STObj(Box::new(DotnetTypeRef::int_128().into()))],
            },
            TyKind::Uint(int_ty) => match int_ty {
                UintTy::U8 => vec![CILOp::STIndI8],
                UintTy::U16 => vec![CILOp::STIndI16],
                UintTy::U32 => vec![CILOp::STIndI32],
                UintTy::U64 => vec![CILOp::STIndI64],
                UintTy::Usize => vec![CILOp::STIndISize],
                UintTy::U128 => vec![CILOp::STObj(Box::new(DotnetTypeRef::uint_128().into()))],
            },
            TyKind::Float(float_ty) => match float_ty {
                FloatTy::F32 => vec![CILOp::STIndF32],
                FloatTy::F64 => vec![CILOp::STIndF64],
            },
            TyKind::Bool => vec![CILOp::STIndI8], // Both Rust bool and a managed bool are 1 byte wide. .NET bools are 4 byte wide only in the context of Marshaling/PInvoke,
            // due to historic reasons(BOOL was an alias for int in early Windows, and it stayed this way.) - FractalFir
            TyKind::Char => vec![CILOp::STIndI32], // always 4 bytes wide: https://doc.rust-lang.org/std/primitive.char.html#representation
            TyKind::Adt(_, _) | TyKind::Tuple(_) | TyKind::Array(_, _) => {
                let pointed_type =
                    type_cache.type_from_cache(pointed_type, tyctx, Some(*method_instance));
                vec![CILOp::STObj(pointed_type.into())]
            }
            TyKind::Ref(_, inner, _) => {
                if pointer_to_is_fat(*inner, tyctx, Some(*method_instance)) {
                    vec![CILOp::STObj(
                        type_cache
                            .type_from_cache(pointed_type, tyctx, Some(*method_instance))
                            .into(),
                    )]
                } else {
                    vec![CILOp::STIndISize]
                }
            }
            TyKind::RawPtr(ty_and_mut) => {
                if pointer_to_is_fat(ty_and_mut.ty, tyctx, Some(*method_instance)) {
                    vec![CILOp::STObj(
                        type_cache
                            .type_from_cache(pointed_type, tyctx, Some(*method_instance))
                            .into(),
                    )]
                } else {
                    vec![CILOp::STIndISize]
                }
            }
            _ => todo!(" can't deref type {pointed_type:?} yet"),
        }
    } else {
        todo!("Can't set the value behind a poitner to an enum variant!");
    }
}
