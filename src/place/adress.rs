use crate::{assert_morphic,cil::{CILOp, FieldDescriptor},function_sig::FnSig,r#type::{Type, TyCache}};
use super::PlaceTy;
use rustc_middle::{ty::{Instance, TyCtxt, TyKind,Ty},mir::PlaceElem};
pub fn local_adress(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::LDLocA(0)
    } else if local > method.arg_count {
        CILOp::LDLocA((local - method.arg_count) as u32)
    } else {
        CILOp::LDArgA((local - 1) as u32)
    }
}
pub fn address_last_dereference<'ctx>(target_type:Ty<'ctx>,curr_type:PlaceTy<'ctx>,tycache:&mut TyCache,tyctx: TyCtxt<'ctx>,method:Instance<'ctx>)->Vec<CILOp>{
    let curr_type = match curr_type{
        PlaceTy::Ty(curr_type) => curr_type,
        // Enums don't require any special handling
        PlaceTy::EnumVariant(_,_)=>return vec![],
    };
    //eprintln!("target_type:{target_type:?} curr_type:{curr_type:?}");
    // Get the type curr_type points to!
    let curr_points_to = super::pointed_type(curr_type.into());
    let curr_type = tycache.type_from_cache(curr_type, tyctx, Some(method));
    
    match (curr_points_to.kind(), target_type.kind()) {
        (TyKind::Slice(_), TyKind::Slice(_)) => 
            vec![],
        (TyKind::Slice(_), _) => 
            vec![CILOp::LDField(
                FieldDescriptor::new(
                    curr_type.as_dotnet().unwrap(),
                    Type::Ptr(Type::Void.into()),
                    "data_address".into(),
                )
                .into(),
            )],
        _=>vec![],
    }
    //println!("casting {source:?} source_pointed_to:{source_pointed_to:?} to {target:?} target_pointed_to:{target_pointed_to:?}. ops:{ops:?}");
}
pub fn place_elem_adress<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
    _body: &rustc_middle::mir::Body,
    type_cache: &mut crate::r#type::TyCache,
    place_ty:Ty<'ctx>
) -> Vec<CILOp> {
    let curr_type = curr_type.monomorphize(&method_instance, tyctx);
    assert_morphic!(curr_type);
    match place_elem {
        PlaceElem::Deref => address_last_dereference(place_ty,curr_type,type_cache,tyctx,method_instance),
        PlaceElem::Field(index, _) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                //TODO: Why was this commented out?
                //let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                let field_desc = crate::utilis::field_descrptor(
                    curr_type,
                    (*index).into(),
                    tyctx,
                    method_instance,
                    type_cache,
                );
                vec![CILOp::LDFieldAdress(field_desc.into())]
            }
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let field_desc = crate::utilis::enum_field_descriptor(
                    owner,
                    index.as_u32(),
                    var_idx,
                    tyctx,
                    method_instance,
                    type_cache,
                );
                let ops = vec![CILOp::LDFieldAdress(field_desc.into())];
                ops
            }
        },
        PlaceElem::Downcast(symbol,_) => {
            let curr_type = curr_type
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let curr_dotnet_type =
                type_cache.type_from_cache(curr_type, tyctx, Some(method_instance));
            let curr_dotnet_type =
                if let crate::r#type::Type::DotnetType(dotnet_type) = curr_dotnet_type {
                    dotnet_type.as_ref().clone()
                } else {
                    panic!();
                };
            let variant_name = symbol.unwrap();
            let field_name = format!("v_{variant_name}").into();
            let _curr_type_name = (curr_dotnet_type).name_path();
            let mut field_type = curr_dotnet_type.clone();
            field_type.append_path(&format!("/{variant_name}"));
            field_type.set_generics_identity();
            let field_desc = FieldDescriptor::boxed(
                curr_dotnet_type.clone(),
                crate::r#type::Type::DotnetType(Box::new(field_type)),
                field_name,
            );
            vec![CILOp::LDFieldAdress(field_desc)]
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            );
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                    let inner_type =
                        type_cache.type_from_cache(inner, tyctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    );
                    let _deref_op = super::deref_op(
                        super::PlaceTy::Ty(inner),
                        tyctx,
                        &method_instance,
                        type_cache,
                    );
                    let ops = vec![
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::Mul,
                        CILOp::Add,
                    ];
                    ops
                }
                TyKind::Array(element, _length) => {
                    //let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let ops = vec![
                        index,
                        CILOp::Call(
                            crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[array_type, Type::USize],
                                    &Type::Ptr(Type::GenericArg(0).into()),
                                ),
                                false,
                            )
                            .into(),
                        ),
                    ];
                    ops
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => {
            rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't handle porojection {place_elem:?} in body")}
        }
    }
}
