use crate::{
    cil_op::{CILOp, CallSite},
    function_sig::FnSig,
    method::Method,
    r#type::Type,
};
/// Creates a wrapper method around entypoint represented by `CallSite`
pub fn wrapper(entrypoint: &CallSite) -> Method {
    if entrypoint.signature().inputs()
        == &[
            Type::ISize,
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::U8)))),
        ]
        && entrypoint.signature().output() == &Type::ISize
    {
        let sig = FnSig::new(&[], &Type::Void);
        let ops = vec![
            CILOp::LdcI32(0),
            CILOp::LdcI32(0),
            CILOp::Call(Box::new(entrypoint.clone())),
            CILOp::Pop,
            CILOp::Ret,
        ];
        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Public,
            true,
            sig,
            "entrypoint",
            vec![],
        );
        method.set_ops(ops);
        method.add_attribute(crate::method::Attribute::EntryPoint);
        method
    } 
    else if entrypoint.signature().inputs().is_empty()
        && entrypoint.signature().output() == &Type::Void
    {
        let sig = FnSig::new(&[], &Type::Void);
        let ops = vec![
            CILOp::Call(Box::new(entrypoint.clone())),
            CILOp::Ret,
        ];
        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Public,
            true,
            sig,
            "entrypoint",
            vec![],
        );
        method.set_ops(ops);
        method.add_attribute(crate::method::Attribute::EntryPoint);
        method
    } 
    else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
