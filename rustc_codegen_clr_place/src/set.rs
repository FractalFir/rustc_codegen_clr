use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::{
    GetTypeExt,
    adt::{enum_field_descriptor, field_descrptor},
    r#type::fat_ptr_to,
    utilis::pointer_to_is_fat,
};

use crate::{PlaceTy, pointed_type};
use cilly::{
    BinOp, IntoAsmIndex, Type, call,
    cil_node::V1Node,
    cil_root::CILRoot,
    conv_usize, ld_field,
    {ClassRef, FieldDesc, Int, MethodRef, cilnode::MethodKind},
};
use rustc_middle::{
    mir::PlaceElem,
    ty::{FloatTy, IntTy, Ty, TyKind, UintTy},
};
pub fn local_set(local: usize, method: &rustc_middle::mir::Body, tree: V1Node) -> CILRoot {
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
    ctx: &mut MethodCompileCtx<'a, '_>,
    addr_calc: V1Node,
    value_calc: V1Node,
) -> CILRoot {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);

            ptr_set_op(pointed_type.into(), ctx, addr_calc, value_calc)
        }
        PlaceElem::Field(field_index, _field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                let curr_type = ctx.monomorphize(curr_type);
                let field_desc = field_descrptor(curr_type, (*field_index).into(), ctx);
                CILRoot::set_field(addr_calc, value_calc, field_desc)
            }
            super::PlaceTy::EnumVariant(enm, var_idx) => {
                let enm = ctx.monomorphize(enm);
                let field_desc = enum_field_descriptor(enm, field_index.as_u32(), var_idx, ctx);

                CILRoot::SetField {
                    addr: Box::new(addr_calc),
                    value: Box::new(value_calc),
                    desc: (field_desc),
                }
            }
        },
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::get::local_get(index.as_usize(), ctx.body(), ctx);

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let inner_ptr = ctx.nptr(inner_type);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let desc = FieldDesc::new(
                        slice,
                        ctx.alloc_string(cilly::DATA_PTR),
                        ctx.nptr(Type::Void),
                    );
                    let field_val = ld_field!(addr_calc, ctx.alloc_field(desc));
                    let size = ctx.size_of(inner_type).into_idx(ctx);
                    let size = ctx.alloc_node(cilly::CILNode::IntCast {
                        input: size,
                        target: Int::USize,
                        extend: cilly::cilnode::ExtendKind::ZeroExtend,
                    });
                    let offset = ctx.biop(index, size, BinOp::Mul);
                    let addr_calc = field_val.cast_ptr(inner_ptr) + V1Node::V2(offset);
                    ptr_set_op(super::PlaceTy::Ty(inner), ctx, addr_calc, value_calc)
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let element_type = ctx.type_from_cache(element);

                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    let arr_ref = ctx.nref(array_type);
                    let mref = MethodRef::new(
                        array_dotnet,
                        ctx.alloc_string("set_Item"),
                        ctx.sig([arr_ref, Type::Int(Int::USize), element_type], Type::Void),
                        MethodKind::Instance,
                        vec![].into(),
                    );
                    CILRoot::Call {
                        site: ctx.alloc_methodref(mref),
                        args: [addr_calc, V1Node::V2(index), value_calc].into(),
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
            let index = V1Node::V2(ctx.alloc_node(*offset));
            assert!(!from_end, "Indexing slice form end");

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);

                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let desc = FieldDesc::new(
                        slice,
                        ctx.alloc_string(cilly::DATA_PTR),
                        ctx.nptr(Type::Void),
                    );
                    let metadata = FieldDesc::new(
                        slice,
                        ctx.alloc_string(cilly::METADATA),
                        Type::Int(Int::USize),
                    );
                    let mref = MethodRef::new(
                        *ctx.main_module(),
                        ctx.alloc_string("bounds_check"),
                        ctx.sig(
                            [Type::Int(Int::USize), Type::Int(Int::USize)],
                            Type::Int(Int::USize),
                        ),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    let addr = ld_field!(addr_calc.clone(), ctx.alloc_field(desc))
                        .cast_ptr(ctx.nptr(inner_type))
                        + call!(
                            ctx.alloc_methodref(mref),
                            [
                                conv_usize!(index),
                                ld_field!(addr_calc, ctx.alloc_field(metadata)),
                            ]
                        ) * conv_usize!(V1Node::V2(ctx.size_of(inner_type).into_idx(ctx)));
                    ptr_set_op(super::PlaceTy::Ty(inner), ctx, addr, value_calc)
                }
                TyKind::Array(element, _length) => {
                    //println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    let arr_ref = ctx.nref(array_type);
                    let mref = MethodRef::new(
                        array_dotnet,
                        ctx.alloc_string("set_Item"),
                        ctx.sig([arr_ref, Type::Int(Int::USize), element], Type::Void),
                        MethodKind::Instance,
                        vec![].into(),
                    );
                    CILRoot::Call {
                        site: ctx.alloc_methodref(mref),
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
pub fn ptr_set_op<'tcx>(
    pointed_type: PlaceTy<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    addr_calc: V1Node,
    value_calc: V1Node,
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
                    tpe: Box::new(ClassRef::int_128(ctx).into()),
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
                    tpe: Box::new(ClassRef::uint_128(ctx).into()),
                    addr_calc: Box::new(addr_calc),
                    value_calc: Box::new(value_calc),
                },
            },
            TyKind::Float(float_ty) => match float_ty {
                FloatTy::F32 => CILRoot::STIndF32(addr_calc, value_calc),
                FloatTy::F64 => CILRoot::STIndF64(addr_calc, value_calc),
                FloatTy::F128 => CILRoot::STObj {
                    tpe: Type::Float(cilly::Float::F128).into(),
                    addr_calc: addr_calc.into(),
                    value_calc: Box::new(value_calc),
                },
                FloatTy::F16 => CILRoot::STObj {
                    tpe: Type::Float(cilly::Float::F16).into(),
                    addr_calc: addr_calc.into(),
                    value_calc: Box::new(value_calc),
                },
            },
            TyKind::Bool => CILRoot::STIndI8(addr_calc, value_calc), // Both Rust bool and a managed bool are 1 byte wide. .NET bools are 4 byte wide only in the context of Marshaling/PInvoke,
            // due to historic reasons(BOOL was an alias for int in early Windows, and it stayed this way.) - FractalFir
            TyKind::Char => CILRoot::STIndI32(addr_calc, value_calc), // always 4 bytes wide: https://doc.rust-lang.org/std/primitive.char.html#representation
            TyKind::Adt(_, _)
            | TyKind::Tuple(_)
            | TyKind::Array(_, _)
            | TyKind::Closure(_, _)
            | TyKind::Coroutine(_, _) => {
                let pointed_type = ctx.type_from_cache(pointed_type);
                CILRoot::STObj {
                    tpe: pointed_type.into(),
                    addr_calc: Box::new(addr_calc),
                    value_calc: Box::new(value_calc),
                }
            }
            TyKind::Ref(_, inner, _) => {
                if pointer_to_is_fat(*inner, ctx.tcx(), ctx.instance()) {
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
                if pointer_to_is_fat(*ty, ctx.tcx(), ctx.instance()) {
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
