use crate::cil::FieldDescriptor;
use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;

use crate::operand::handle_operand;

use crate::{conv_usize, ld_field, ldc_u64};

use crate::r#type::{TyCache, Type};
use rustc_middle::{
    mir::{Operand, Place, Rvalue},
    ty::{Instance, Ty, TyCtxt, TyKind},
};

pub fn unsize<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
    operand: &Operand<'tyctx>,
    target: Ty<'tyctx>,
) -> CILNode {
    // Get the monomorphized source and target type
    let target = crate::utilis::monomorphize(&method_instance, target, tyctx);
    let source = crate::utilis::monomorphize(&method_instance, operand.ty(method, tyctx), tyctx);
    // Get the source and target types as .NET types
    let source_type = tycache.type_from_cache(source, tyctx, Some(method_instance));
    let target_type = tycache.type_from_cache(target, tyctx, Some(method_instance));
    // Get the target type as a fat pointer.
    let target_dotnet = target_type.as_dotnet().unwrap();
    if source.is_box() {
        panic!("Can't unsize boxes yet");
    }
    let mut sized_ptr = handle_operand(&operand, tyctx, method, method_instance, tycache);
    let derefed_source = match source.kind() {
        TyKind::RawPtr(tpe, _) => *tpe,
        TyKind::Ref(_, inner, _) => *inner,
        TyKind::Adt(_, _) => {
            if source.is_box() {
                let inner = source.boxed_ty();
                let field_descriptor =
                    crate::utilis::field_descrptor(source, 0, tyctx, method_instance, tycache);
                sized_ptr = CILNode::TemporaryLocal(Box::new((
                    source_type,
                    [CILRoot::SetTMPLocal { value: sized_ptr }].into(),
                    ld_field!(CILNode::LoadAddresOfTMPLocal, field_descriptor),
                )));
                inner
            } else {
                panic!("Non ptr type:{source:?}")
            }
        }
        _ => panic!("Non ptr type:{source:?}"),
    };
    let length = if let TyKind::Array(_, length) = derefed_source.kind() {
        crate::utilis::try_resolve_const_size(*length).unwrap()
    } else {
        panic!("Non array type. source:{source:?} target:{target:?}")
    };
    let metadata_field =
        FieldDescriptor::new(target_dotnet.clone(), Type::USize, "metadata".into());
    let ptr_field = FieldDescriptor::new(
        target_dotnet,
        Type::Ptr(Type::Void.into()),
        "data_pointer".into(),
    );
    CILNode::TemporaryLocal(Box::new((
        target_type,
        [
            CILRoot::SetField {
                addr: CILNode::LoadAddresOfTMPLocal,
                value: conv_usize!(ldc_u64!(length as u64)),
                desc: metadata_field,
            },
            CILRoot::SetField {
                addr: CILNode::LoadAddresOfTMPLocal,
                value: sized_ptr,
                desc: ptr_field,
            },
        ]
        .into(),
        CILNode::LoadTMPLocal,
    )))
}
