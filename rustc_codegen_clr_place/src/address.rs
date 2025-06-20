use super::PlaceTy;
use crate::pointer_to_is_fat;
use cilly::{
    Assembly, Const, FieldDesc, Int, Interned, IntoAsmIndex, MethodRef, Type, call,
    cil_node::V1Node, cilnode::MethodKind, conv_usize, ld_field,
};
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::{
    GetTypeExt,
    adt::{FieldOffsetIterator, enum_field_descriptor, field_descrptor},
    r#type::{fat_ptr_to, get_type},
};
use rustc_middle::{
    mir::PlaceElem,
    ty::{Ty, TyKind},
};
pub fn local_address(
    local: usize,
    method: &rustc_middle::mir::Body,
    asm: &mut Assembly,
) -> Interned<cilly::v2::CILNode> {
    let local = if let Some(spread_arg) = method.spread_arg
        && local == spread_arg.as_usize()
    {
        cilly::CILNode::LdLocA(
            (method.local_decls.len() - method.arg_count)
                .try_into()
                .unwrap(),
        )
    } else if local == 0 {
        cilly::CILNode::LdLocA(0)
    } else if local > method.arg_count {
        cilly::CILNode::LdLocA(u32::try_from(local - method.arg_count).unwrap())
    } else {
        cilly::CILNode::LdArgA(u32::try_from(local - 1).unwrap())
    };
    let local = asm.alloc_node(local);
    asm.alloc_node(cilly::CILNode::RefToPtr(local))
}
pub fn address_last_dereference<'tcx>(
    target_ty: Ty<'tcx>,
    curr_type: PlaceTy<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    addr_calc: V1Node,
) -> V1Node {
    let curr_type = match curr_type {
        PlaceTy::Ty(curr_type) => curr_type,
        // Enums don't require any special handling
        PlaceTy::EnumVariant(_, _) => return addr_calc,
    };
    // Get the type curr_type points to!
    let curr_points_to = super::pointed_type(curr_type.into());
    let curr_type = ctx.type_from_cache(curr_type);
    let target_type = ctx.type_from_cache(target_ty);

    match (
        pointer_to_is_fat(curr_points_to, ctx.tcx(), ctx.instance()),
        pointer_to_is_fat(target_ty, ctx.tcx(), ctx.instance()),
    ) {
        (true, false) => {
            let data_ptr_name = ctx.alloc_string(cilly::DATA_PTR);
            let void_ptr = ctx.nptr(Type::Void);
            V1Node::LDIndPtr {
                ptr: Box::new(V1Node::LDField {
                    field: ctx.alloc_field(FieldDesc::new(
                        curr_type.as_class_ref().unwrap(),
                        data_ptr_name,
                        void_ptr,
                    )),
                    addr: addr_calc.into(),
                }),
                loaded_ptr: Box::new(ctx.nptr(target_type)),
            }
        }
        (false, true) => panic!("Invalid last dereference in address!"),
        (false, false) => addr_calc,
        (true, true) => V1Node::LdObj {
            ptr: Box::new(addr_calc),
            obj: Box::new(curr_type),
        },
    }
}
fn field_address<'a>(
    curr_type: super::PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_>,
    addr_calc: V1Node,
    field_index: u32,
    field_type: Ty<'a>,
) -> V1Node {
    match curr_type {
        super::PlaceTy::Ty(curr_type) => {
            let curr_type = ctx.monomorphize(curr_type);
            let field_ty = ctx.monomorphize(field_type);
            match (
                pointer_to_is_fat(curr_type, ctx.tcx(), ctx.instance()),
                pointer_to_is_fat(field_ty, ctx.tcx(), ctx.instance()),
            ) {
                (false, false) => {
                    let field_desc = field_descrptor(curr_type, field_index, ctx);
                    V1Node::LDFieldAdress {
                        addr: addr_calc.into(),
                        field: (field_desc),
                    }
                }
                (false, true) => panic!(
                    "Sized type {curr_type:?} contains an unsized field of type {field_ty}. This is a bug."
                ),
                (true, false) => {
                    let mut explicit_offset_iter =
                        FieldOffsetIterator::fields(ctx.layout_of(curr_type).layout.0.0.clone());
                    let offset = explicit_offset_iter
                        .nth(field_index as usize)
                        .expect("Field index not in field offset iterator");
                    let curr_type_fat_ptr = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        curr_type,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let data_ptr_name = ctx.alloc_string(cilly::DATA_PTR);
                    let void_ptr = ctx.nptr(Type::Void);
                    let addr_descr = ctx.alloc_field(FieldDesc::new(
                        curr_type_fat_ptr.as_class_ref().unwrap(),
                        data_ptr_name,
                        void_ptr,
                    ));
                    // Get the address of the unsized object.
                    let obj_addr = ld_field!(addr_calc, addr_descr);
                    let obj = ctx.type_from_cache(field_type);
                    // Add the offset to the object.
                    (obj_addr + V1Node::V2(ctx.alloc_node(Const::USize(u64::from(offset)))))
                        .cast_ptr(ctx.nptr(obj))
                }
                (true, true) => {
                    let mut explicit_offset_iter =
                        FieldOffsetIterator::fields(ctx.layout_of(curr_type).layout.0.0.clone());
                    let offset = explicit_offset_iter
                        .nth(field_index as usize)
                        .expect("Field index not in field offset iterator");
                    let curr_type_fat_ptr = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        curr_type,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let data_ptr_name = ctx.alloc_string(cilly::DATA_PTR);
                    let metadata_name = ctx.alloc_string(cilly::METADATA);
                    let void_ptr = ctx.nptr(Type::Void);

                    let addr_descr = ctx.alloc_field(FieldDesc::new(
                        curr_type_fat_ptr.as_class_ref().unwrap(),
                        data_ptr_name,
                        void_ptr,
                    ));
                    // Get the address of the unsized object.
                    let obj_addr = ld_field!(addr_calc.clone(), addr_descr);
                    let metadata_descr = ctx.alloc_field(FieldDesc::new(
                        curr_type_fat_ptr.as_class_ref().unwrap(),
                        metadata_name,
                        Type::Int(Int::USize),
                    ));
                    let metadata = ld_field!(addr_calc, metadata_descr);
                    let ptr =
                        obj_addr + V1Node::V2(ctx.alloc_node(Const::USize(u64::from(offset))));
                    let field_fat_ptr = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        field_ty,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    V1Node::create_slice(field_fat_ptr.as_class_ref().unwrap(), ctx, metadata, ptr)
                }
            }
        }
        super::PlaceTy::EnumVariant(enm, var_idx) => {
            let owner = ctx.monomorphize(enm);
            let field_desc = enum_field_descriptor(owner, field_index, var_idx, ctx);
            V1Node::LDFieldAdress {
                addr: addr_calc.into(),
                field: field_desc,
            }
        }
    }
}
pub fn place_elem_address<'tcx>(
    place_elem: &PlaceElem<'tcx>,
    curr_type: PlaceTy<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    place_ty: Ty<'tcx>,
    addr_calc: V1Node,
) -> V1Node {
    let curr_type = curr_type.monomorphize(ctx);

    match place_elem {
        PlaceElem::Deref => address_last_dereference(place_ty, curr_type, ctx, addr_calc),
        PlaceElem::Field(field_index, field_ty) => {
            field_address(curr_type, ctx, addr_calc, field_index.as_u32(), *field_ty)
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::local_get(index.as_usize(), ctx.body(), ctx);
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(inner, ctx);

                    let data_ptr_name = ctx.alloc_string(cilly::DATA_PTR);
                    let void_ptr = ctx.nptr(Type::Void);
                    let desc = ctx.alloc_field(FieldDesc::new(slice, data_ptr_name, void_ptr));
                    // This is a false positive
                    //    #[allow(unused_parens)]
                    let size = ctx.size_of(inner_type);
                    let size = size.into_idx(ctx);
                    let size = ctx.alloc_node(cilly::CILNode::IntCast {
                        input: size,
                        target: Int::USize,
                        extend: cilly::cilnode::ExtendKind::ZeroExtend,
                    });
                    let offset = ctx.biop(index, size, cilly::BinOp::Mul);
                    (ld_field!(addr_calc.clone(), desc)).cast_ptr(ctx.nptr(inner_type))
                        + V1Node::V2(offset)
                }
                TyKind::Array(element, _) => {
                    let mref = array_get_address(ctx, *element, curr_ty);
                    call!(ctx.alloc_methodref(mref), [addr_calc, V1Node::V2(index)])
                }
                _ => {
                    todo!("Can't index into {curr_ty}!")
                }
            }
        }
        PlaceElem::Subslice { from, to, from_end } => {
            let elem_type = curr_type
                .as_ty()
                .expect("Can't index into an enum!")
                .sequence_element_type(ctx.tcx());
            let elem_type = get_type(elem_type, ctx);
            let curr_type = fat_ptr_to(curr_type.as_ty().expect("Can't index into an enum!"), ctx);

            if *from_end {
                //assert!(from >= to, "from_end:{from_end} from:{from} to:{to}");
                let metadata_name = ctx.alloc_string(cilly::METADATA);
                let metadata_field = ctx.alloc_field(FieldDesc::new(
                    curr_type,
                    metadata_name,
                    Type::Int(Int::USize),
                ));
                let data_ptr_name = ctx.alloc_string(cilly::DATA_PTR);
                let void_ptr = ctx.nptr(Type::Void);
                let ptr_field = ctx.alloc_field(FieldDesc::new(curr_type, data_ptr_name, void_ptr));
                // len = end - start
                // [from..slice.len() - to] -> (slice.len() - to) - from -> (slice.len() - (to + from)
                let metadata = V1Node::Sub(
                    Box::new(ld_field!(addr_calc.clone(), metadata_field)),
                    Box::new(V1Node::V2(ctx.alloc_node(Const::USize(*to + from)))),
                );

                let data_ptr = if elem_type != Type::Void {
                    ld_field!(addr_calc, ptr_field)
                        + V1Node::V2(ctx.alloc_node(Const::USize(*from)))
                            * conv_usize!(V1Node::V2(ctx.size_of(elem_type).into_idx(ctx)))
                } else {
                    ld_field!(addr_calc, ptr_field)
                };
                V1Node::create_slice(curr_type, ctx, metadata, data_ptr)
            } else {
                let void_ptr = ctx.nptr(Type::Void);
                let data_ptr = ctx.alloc_string(cilly::DATA_PTR);

                let ptr_field = ctx.alloc_field(FieldDesc::new(curr_type, data_ptr, void_ptr));
                let metadata = V1Node::V2(ctx.alloc_node(Const::USize(to - from)));
                let data_ptr = ld_field!(addr_calc, ptr_field)
                    + V1Node::V2(ctx.alloc_node(Const::USize(*from)))
                        * conv_usize!(V1Node::V2(ctx.size_of(elem_type).into_idx(ctx)));

                V1Node::create_slice(curr_type, ctx, metadata, data_ptr)
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
            let _ = min_length;
            //assert!(!from_end, "Indexing slice form end");
            //println!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);

                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let void_ptr = ctx.nptr(Type::Void);
                    let data_ptr = ctx.alloc_string(cilly::DATA_PTR);
                    let desc = ctx.alloc_field(FieldDesc::new(slice, data_ptr, void_ptr));
                    let metadata = ctx.alloc_string(cilly::METADATA);
                    let len =
                        ctx.alloc_field(FieldDesc::new(slice, metadata, Type::Int(Int::USize)));
                    let index = if *from_end {
                        //eprintln!("Slice index from end is:{offset}");
                        V1Node::Sub(
                            Box::new(ld_field!(addr_calc.clone(), len)),
                            Box::new(V1Node::V2(ctx.alloc_node(Const::USize(*offset)))),
                        )
                    } else {
                        V1Node::V2(ctx.alloc_node(Const::USize(*offset)))
                        //ops.extend(derf_op);
                    };

                    ld_field!(addr_calc.clone(), desc).cast_ptr(ctx.nptr(inner_type))
                        + (index * conv_usize!(V1Node::V2(ctx.size_of(inner_type).into_idx(ctx))))
                }
                TyKind::Array(element, _) => {
                    let mref = array_get_address(ctx, *element, curr_ty);
                    if *from_end {
                        todo!("Can't index array from end!");
                    } else {
                        call!(
                            ctx.alloc_methodref(mref),
                            [addr_calc, V1Node::V2(ctx.alloc_node(Const::USize(*offset)))]
                        )
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => {
            rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't handle porojection {place_elem:?} in address")}
        }
    }
}
pub fn array_get_address<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    element: Ty<'tcx>,
    curr_ty: Ty<'tcx>,
) -> MethodRef {
    let element = ctx.monomorphize(element);
    let element = ctx.type_from_cache(element);
    let array_type = ctx.type_from_cache(curr_ty);
    let array_dotnet = array_type.as_class_ref().expect("Non array type");
    let arr_ref = ctx.nref(array_type);
    let element_ptr = ctx.nptr(element);
    MethodRef::new(
        array_dotnet,
        ctx.alloc_string("get_Address"),
        ctx.sig([arr_ref, Type::Int(Int::USize)], element_ptr),
        MethodKind::Instance,
        vec![].into(),
    )
}
