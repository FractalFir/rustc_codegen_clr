use std::num::NonZeroU8;

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_isize, conv_usize, ldc_u32,
    method::{Attribute, Method, MethodType},
    v2::{Assembly, FnSig, Int},
    Type,
};

/// Creates a wrapper method around entypoint represented by `CallSite`
pub fn wrapper(entrypoint: &CallSite, asm: &mut Assembly) -> Method {
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);
    if entrypoint.signature().inputs() == [Type::Int(Int::ISize), uint8_ptr_ptr]
        && entrypoint.signature().output() == &Type::Int(Int::ISize)
    {
        let sig = FnSig::new(
            Box::new([Type::PlatformArray {
                elem: asm.alloc_type(Type::PlatformString),
                dims: NonZeroU8::new(1).unwrap(),
            }]),
            Type::Void,
        );

        let mut method = Method::new(
            AccessModifer::Extern,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![(Some("argc".into()), uint8_ptr_ptr)],
            vec![BasicBlock::new(
                vec![
                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            None,
                            ".tcctor".into(),
                            FnSig::new(Box::new([]), Type::Void),
                            true,
                        )),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            None,
                            "static_init".into(),
                            FnSig::new(Box::new([]), Type::Void),
                            true,
                        )),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Pop {
                        tree: call!(
                            Box::new(entrypoint.clone()),
                            [
                                conv_isize!(ldc_u32!(0)),
                                conv_usize!(ldc_u32!(0)).cast_ptr(uint8_ptr_ptr)
                            ]
                        ),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                2,
                None,
            )],
            vec![Some("args".into())],
        );
        //method.set_ops(ops);
        method.add_attribute(Attribute::EntryPoint);
        method
    } else if entrypoint.signature().inputs().is_empty()
        && entrypoint.signature().output() == &Type::Void
    {
        let sig = FnSig::new(Box::new([]), Type::Void);
        let mut method = Method::new(
            AccessModifer::Extern,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            None,
                            ".tcctor".into(),
                            FnSig::new(Box::new([]), Type::Void),
                            true,
                        )),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Call {
                        site: Box::new(CallSite::new(
                            None,
                            "static_init".into(),
                            FnSig::new(Box::new([]), Type::Void),
                            true,
                        )),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Call {
                        site: Box::new(entrypoint.clone()),
                        args: [].into(),
                    }
                    .into(),
                    //CILRoot::debug(&format!("Preparing to execute the main program.")).into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
            vec![],
        );

        method.add_attribute(Attribute::EntryPoint);
        method
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
