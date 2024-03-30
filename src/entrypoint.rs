use crate::{
    basic_block::BasicBlock,
    call,
    cil::CallSite,
    cil_tree::cil_node::CILNode,
    cil_tree::cil_root::CILRoot,
    conv_usize,
    function_sig::FnSig,
    ldc_u32,
    method::{Method, MethodType},
    r#type::Type,
};
/// Creates a wrapper method around entypoint represented by `CallSite`
pub fn wrapper(entrypoint: &CallSite) -> Method {
    if entrypoint.signature().inputs()
        == [
            Type::ISize,
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::U8)))),
        ]
        && entrypoint.signature().output() == &Type::ISize
    {
        let sig = FnSig::new(&[], &Type::Void);

        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Public,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::Pop {
                        tree: call!(
                            Box::new(entrypoint.clone()),
                            [conv_usize!(ldc_u32!(0)), conv_usize!(ldc_u32!(0))]
                        ),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
        );
        //method.set_ops(ops);
        method.add_attribute(crate::method::Attribute::EntryPoint);
        method
    } else if entrypoint.signature().inputs().is_empty()
        && entrypoint.signature().output() == &Type::Void
    {
        let sig = FnSig::new(&[], &Type::Void);
        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Public,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::Call {
                        site: entrypoint.clone(),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
        );

        method.add_attribute(crate::method::Attribute::EntryPoint);
        method
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
