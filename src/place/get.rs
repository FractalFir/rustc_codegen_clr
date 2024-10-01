use crate::{assembly::MethodCompileCtx, r#type::fat_ptr_to};
use cilly::{
    call,
    cil_node::CILNode,
    conv_usize, ld_field, ldc_u32, ldc_u64,
    v2::{cilnode::MethodKind, FieldDesc, FnSig, Int, MethodRef},
    Type,
};
use rustc_middle::{
    mir::{Place, PlaceElem},
    ty::Ty,
    ty::TyKind,
};

use super::body_ty_is_by_adress;

pub(super) fn local_get(local: usize, method: &rustc_middle::mir::Body) -> CILNode {
    if let Some(spread_arg) = method.spread_arg
        && local == spread_arg.as_usize()
    {
        return CILNode::LDLoc(
            (method.local_decls.len() - method.arg_count)
                .try_into()
                .unwrap(),
        );
    }
    if local == 0 {
        CILNode::LDLoc(0)
    } else if local > method.arg_count {
        CILNode::LDLoc(
            u32::try_from(local - method.arg_count)
                .expect("Method has more than 2^32 local varaibles"),
        )
    } else {
        CILNode::LDArg(u32::try_from(local - 1).expect("Method has more than 2^32 local variables"))
    }
}
/// Returns the ops for getting the value of place.
pub fn place_get<'tcx>(place: &Place<'tcx>, ctx: &mut MethodCompileCtx<'tcx, '_>) -> CILNode {
    if place.projection.is_empty() {
        local_get(place.local.as_usize(), ctx.body())
    } else {
        let (mut op, mut ty) = super::local_body(place.local.as_usize(), ctx);
        ty = ctx.monomorphize(ty);
        let mut ty = ty.into();

        let (head, body) = super::slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) = super::place_elem_body(elem, ty, ctx, op);
            ty = curr_ty.monomorphize(ctx);
            op = curr_ops;
        }

        place_elem_get(head, ty, ctx, op)
    }
}
fn get_field<'a>(
    curr_type: super::PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_>,
    addr_calc: CILNode,
    field_index: u32,
    field_type: Ty<'a>,
) -> CILNode {
    match curr_type {
        super::PlaceTy::Ty(curr_type) => {
            let curr_type = ctx.monomorphize(curr_type);
            let field_type = ctx.monomorphize(field_type);
            match (
                crate::r#type::pointer_to_is_fat(curr_type, ctx.tcx(), ctx.instance()),
                crate::r#type::pointer_to_is_fat(field_type, ctx.tcx(), ctx.instance()),
            ) {
                (false, false) => {
                    let field_desc = crate::utilis::field_descrptor(curr_type, field_index, ctx);
                    CILNode::LDField {
                        addr: addr_calc.into(),
                        field: field_desc,
                    }
                }
                (false, true) => panic!("Sized type {curr_type:?} contains an unsized field of type {field_type}. This is a bug."),
                (true,false)=>{
                    let mut explicit_offset_iter = crate::utilis::adt::FieldOffsetIterator::fields(
                        ctx.layout_of(curr_type).layout.0 .0.clone(),
                    );
                    let offset = explicit_offset_iter
                        .nth(field_index as usize)
                        .expect("Field index not in field offset iterator");
                    let curr_type_fat_ptr = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        curr_type,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let addr_descr = FieldDesc::new(curr_type_fat_ptr.as_class_ref().unwrap(),ctx.alloc_string(crate::DATA_PTR),ctx.nptr(Type::Void),);
                    // Get the address of the unsized object.
                    let obj_addr = ld_field!(addr_calc, ctx.alloc_field(addr_descr));
                    let obj = ctx.type_from_cache(field_type);
                    // Add the offset to the object.
                    CILNode::LdObj{ ptr: Box::new(obj_addr + conv_usize!(ldc_u32!(offset))), obj: Box::new(obj) }
                },
                (true,true)=>panic!("Nonsensical operation: attempted to get value of the unsized type {field_type}. Unsized types can only be accessed by address."),
            }
        }
        super::PlaceTy::EnumVariant(enm, var_idx) => {
            let owner = ctx.monomorphize(enm);
            let field_desc = crate::utilis::enum_field_descriptor(owner, field_index, var_idx, ctx);
            CILNode::LDField {
                addr: addr_calc.into(),
                field: field_desc,
            }
        }
    }
}
fn place_elem_get<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: super::PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_>,
    addr_calc: CILNode,
) -> CILNode {
    match place_elem {
        PlaceElem::Deref => super::deref_op(super::pointed_type(curr_type).into(), ctx, addr_calc),
        PlaceElem::Field(field_index, field_type) => {
            get_field(curr_type, ctx, addr_calc, field_index.as_u32(), *field_type)
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");

            let index_type = ctx.monomorphize(ctx.body().local_decls[*index].ty);
            let index = crate::place::local_get(index.as_usize(), ctx.body());

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);

                    let index_type = ctx.type_from_cache(index_type);
                    let desc = FieldDesc::new(
                        slice,
                        ctx.alloc_string(crate::DATA_PTR),
                        ctx.nptr(Type::Void),
                    );
                    let size = crate::casts::int_to_int(
                        Type::Int(Int::I32),
                        index_type,
                        CILNode::SizeOf(inner_type.into()),
                        ctx,
                    );
                    let addr = (ld_field!(addr_calc, ctx.alloc_field(desc))
                        .cast_ptr(ctx.nptr(inner_type)))
                        + (index * size);
                    super::deref_op(super::PlaceTy::Ty(inner), ctx, addr)
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    let arr_ref = ctx.nref(array_type);
                    let mref = MethodRef::new(
                        (array_dotnet),
                        ctx.alloc_string("get_Item"),
                        ctx.sig([arr_ref, Type::Int(Int::USize)], element),
                        MethodKind::Instance,
                        vec![].into(),
                    );
                    call!(
                        ctx.alloc_methodref(mref),
                        [addr_calc, CILNode::ZeroExtendToUSize(index.into())]
                    )
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
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

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let data_pointer = FieldDesc::new(
                        slice,
                        ctx.alloc_string(crate::DATA_PTR),
                        ctx.nptr(Type::Void),
                    );
                    let metadata = FieldDesc::new(
                        slice,
                        ctx.alloc_string(crate::METADATA),
                        Type::Int(Int::USize),
                    );
                    let index = if *from_end {
                        //eprintln!("Slice index from end is:{offset}");
                        CILNode::Sub(
                            Box::new(ld_field!(addr_calc.clone(), ctx.alloc_field(metadata))),
                            Box::new(conv_usize!(ldc_u64!(*offset))),
                        )
                    } else {
                        conv_usize!(ldc_u64!(*offset))
                        //ops.extend(derf_op);
                    };
                    let addr = ld_field!(addr_calc.clone(), ctx.alloc_field(data_pointer))
                        .cast_ptr(ctx.nptr(inner_type))
                        + (conv_usize!(index))
                            * CILNode::ZeroExtendToUSize(CILNode::SizeOf(inner_type.into()).into());
                    super::deref_op(super::PlaceTy::Ty(inner), ctx, addr)
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    //eprintln!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
                    let arr_ref = ctx.nref(array_type);
                    if *from_end {
                        todo!("Can't index array from end!");
                    } else {
                        let index = CILNode::LdcU64(*offset);
                        let mref = MethodRef::new(
                            (array_dotnet),
                            ctx.alloc_string("get_Item"),
                            ctx.sig([arr_ref, Type::Int(Int::USize)], element),
                            MethodKind::Instance,
                            vec![].into(),
                        );
                        call!(
                            ctx.alloc_methodref(mref),
                            [addr_calc, CILNode::ZeroExtendToUSize(index.into())]
                        )
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::Subtype(tpe) => {
            if body_ty_is_by_adress(curr_type.as_ty().unwrap(), ctx) {
                super::deref_op((*tpe).into(), ctx, addr_calc)
            } else {
                addr_calc
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
