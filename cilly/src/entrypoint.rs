use std::num::NonZeroU8;

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_isize, conv_usize, ldc_u32,
    method::{Attribute, Method, MethodType},
    v2::{cilnode::MethodKind, Assembly, FnSig, Int, MethodRef},
    Type,
};

/// Creates a wrapper method around entypoint represented by `MethodRefIdx`
pub fn wrapper(entrypoint: MethodRef, asm: &mut Assembly) -> Method {
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);
    let entry_sig = &asm[entrypoint.sig()];
    if entry_sig.inputs() == [Type::Int(Int::ISize), uint8_ptr_ptr]
        && entry_sig.output() == &Type::Int(Int::ISize)
    {
        let sig = FnSig::new(
            Box::new([Type::PlatformArray {
                elem: asm.alloc_type(Type::PlatformString),
                dims: NonZeroU8::new(1).unwrap(),
            }]),
            Type::Void,
        );
        let mref = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string(".tcctor"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let static_mref = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string("static_init"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
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
                        site: asm.alloc_methodref(mref),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Call {
                        site: asm.alloc_methodref(static_mref),
                        args: [].into(),
                    }
                    .into(),
                    CILRoot::Pop {
                        tree: call!(
                            (asm.alloc_methodref(entrypoint)),
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
    } else if entry_sig.inputs().is_empty() && entry_sig.output() == &Type::Void {
        let sig = FnSig::new(Box::new([]), Type::Void);
        let tcctor = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string(".tcctor"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let static_init = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string("static_init"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let blocks = vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: asm.alloc_methodref(tcctor),
                    args: [].into(),
                }
                .into(),
                CILRoot::Call {
                    site: asm.alloc_methodref(static_init),
                    args: [].into(),
                }
                .into(),
                CILRoot::Call {
                    site: asm.alloc_methodref(entrypoint.clone()),
                    args: [].into(),
                }
                .into(),
                //CILRoot::debug(&format!("Preparing to execute the main program.")).into(),
                CILRoot::VoidRet.into(),
            ],
            0,
            None,
        )];
        let mut method = Method::new(
            AccessModifer::Extern,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![],
            blocks,
            vec![],
        );

        method.add_attribute(Attribute::EntryPoint);
        method
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
