use super::{place_get_length, pointed_type, PlaceTy};
use crate::assert_morphic;
use crate::cil_op::{CILOp, FieldDescriptor};
use crate::place::{body_ty_is_by_adress, deref_op};
use crate::r#type::Type;
use crate::utilis::field_name;
use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{FloatTy, Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy};
pub fn local_body<'tcx>(local: usize, method: &rustc_middle::mir::Body<'tcx>) -> (CILOp, Ty<'tcx>) {
    let ty = method.local_decls[local.into()].ty;
    if body_ty_is_by_adress(&ty) {
        (super::adress::local_adress(local, method), ty)
    } else {
        (super::get::local_get(local, method), ty)
    }
}
pub fn place_elem_body<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
    _body: &rustc_middle::mir::Body,
) -> (PlaceTy<'ctx>, Vec<CILOp>) {
    let curr_type = curr_type.monomorphize(&method_instance, tyctx);
    assert_morphic!(curr_type);
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_type);
            assert_morphic!(pointed);
            if body_ty_is_by_adress(&pointed) {
                (pointed.into(), vec![])
            } else {
                (
                    pointed.into(),
                    deref_op(pointed.into(), tyctx, &method_instance),
                )
            }
        }
        PlaceElem::Field(index, field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                let gen_field_type = crate::utilis::generic_field_ty(
                    curr_type,
                    index.as_u32(),
                    tyctx,
                    method_instance,
                );
                //TODO: Why was this commented out?
                let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
                let field_name = field_name(curr_type, index.as_u32());
                let curr_type = crate::r#type::Type::from_ty(curr_type, tyctx, &method_instance);
                let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = curr_type {
                    dotnet_type.as_ref().clone()
                } else {
                    panic!();
                };

                let field_desc = FieldDescriptor::boxed(curr_type, gen_field_type, field_name);
                if body_ty_is_by_adress(&field_type) {
                    ((field_type).into(), vec![CILOp::LDFieldAdress(field_desc)])
                } else {
                    ((field_type).into(), vec![CILOp::LDField(field_desc)])
                }
            }
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let variant_name = crate::utilis::variant_name(owner, var_idx);
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let gen_field_type =
                    crate::utilis::generic_field_ty(owner, index.as_u32(), tyctx, method_instance);
                let owner = crate::r#type::Type::from_ty(owner, tyctx, &method_instance);
                let owner = if let crate::r#type::Type::DotnetType(owner) = owner {
                    owner.as_ref().clone()
                } else {
                    panic!();
                };
                let field_name = field_name(enm, index.as_u32());
                let mut field_owner = owner;

                field_owner.append_path(&format!("/{variant_name}"));
                let field_desc = FieldDescriptor::boxed(field_owner, gen_field_type, field_name);
                let ops = vec![CILOp::LDFieldAdress(field_desc)];
                ((*field_type).into(), ops)
            }
        },
        PlaceElem::Downcast(symbol, variant) => {
            let curr_type = curr_type
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let curr_dotnet_type = crate::r#type::Type::from_ty(curr_type, tyctx, &method_instance);
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
            let variant_type = PlaceTy::EnumVariant(curr_type, variant.as_u32());
            (variant_type, vec![CILOp::LDFieldAdress(field_desc)])
        }
        PlaceElem::Index(index) => {
            let mut ops = vec![crate::place::local_adress(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            )];
            let curr_ty = curr_type.as_ty().expect("Can't index into enum!");
            let tpe = Type::from_ty(curr_ty, tyctx, &method_instance);
            let class = if let Type::DotnetType(dotnet) = &tpe {
                dotnet
            } else {
                panic!("Can't index into type {tpe:?}");
            };
            let index_ty = Type::USize;
            let element_ty = crate::r#type::element_type(curr_ty);
            if body_ty_is_by_adress(&element_ty) {
                let signature = crate::function_sig::FnSig::new(
                    &[tpe.clone(), index_ty],
                    &Type::Ptr(Box::new(Type::GenericArg(0))),
                );
                //ops.push(local_get(index.as_usize(), &body));
                ops.push(CILOp::Call(crate::cil_op::CallSite::boxed(
                    Some(class.as_ref().clone()),
                    "get_Address".into(),
                    signature,
                    false,
                )));
                (element_ty.into(), ops)
            } else {
                let signature =
                    crate::function_sig::FnSig::new(&[tpe.clone(), index_ty], &Type::GenericArg(0));
                ops.push(CILOp::LDArg(0));
                ops.push(CILOp::Call(crate::cil_op::CallSite::boxed(
                    Some(class.as_ref().clone()),
                    "get_Item".into(),
                    signature,
                    false,
                )));
                (element_ty.into(), ops)
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in body"),
    }
}
