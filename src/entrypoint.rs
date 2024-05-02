use std::num::NonZeroU8;

use crate::{
    basic_block::BasicBlock,
    call, call_virt,
    cil::CallSite,
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_usize,
    function_sig::FnSig,
    ldc_u32,
    method::{Method, MethodType},
    mul,
    r#type::{DotnetTypeRef, Type},
    size_of,
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
        let sig = FnSig::new(
            &[Type::ManagedArray {
                element: Box::new(DotnetTypeRef::string_type().into()),
                dims: NonZeroU8::new(1).unwrap(),
            }],
            &Type::Void,
        );

        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Public,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![(
                Some("argc".into()),
                Type::Ptr(Type::Ptr(Type::U8.into()).into()),
            )],
            vec![
                BasicBlock::new(
                    vec![
                        // Allocate argc(mamaged arguments + exec path)
                        CILRoot::STLoc {
                            local: 0,
                            tree: call!(
                                CallSite::alloc(),
                                [
                                    (call!(
                                        CallSite::new(
                                            Some(DotnetTypeRef::managed_array()),
                                            "get_Length".into(),
                                            FnSig::new(
                                                &[DotnetTypeRef::managed_array().into()],
                                                &Type::I32
                                            ),
                                            false
                                        ),
                                        [CILNode::LDArg(0)]
                                    ) + conv_usize!(ldc_u32!(1)))
                                        * conv_usize!(size_of!(Type::ISize)),
                                    conv_usize!(ldc_u32!(8))
                                ]
                            ),
                        }
                        .into(),
                        // Set the first arg to exec path
                        CILRoot::STIndISize(
                            CILNode::LDLoc(0),
                            call!(
                                CallSite::mstring_to_ptr(),
                                [call_virt!(
                                    CallSite::new(
                                        Some(DotnetTypeRef::assembly()),
                                        "get_Location".into(),
                                        FnSig::new(
                                            &[DotnetTypeRef::assembly().into()],
                                            &DotnetTypeRef::string_type().into()
                                        ),
                                        false
                                    ),
                                    [call!(
                                        CallSite::new(
                                            Some(DotnetTypeRef::assembly()),
                                            "GetEntryAssembly".into(),
                                            FnSig::new(&[], &DotnetTypeRef::assembly().into()),
                                            true,
                                        ),
                                        []
                                    )]
                                )]
                            ),
                        )
                        .into(),
                        // FIXME: This exec path is absolute!
                        CILRoot::GoTo {
                            target: 1,
                            sub_target: 0,
                        }
                        .into(),
                    ],
                    0,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        // TODOD: iter trough args!
                        CILRoot::GoTo {
                            target: 2,
                            sub_target: 0,
                        }
                        .into(),
                    ],
                    1,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::Pop {
                            tree: call!(
                                Box::new(entrypoint.clone()),
                                [conv_usize!(CILNode::LDLoc(0)), conv_usize!(ldc_u32!(1))]
                            ),
                        }
                        .into(),
                        CILRoot::VoidRet.into(),
                    ],
                    2,
                    None,
                ),
            ],
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
